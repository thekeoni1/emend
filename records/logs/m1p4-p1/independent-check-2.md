Independent check 2 of M1 part 4 P1

Scope. I worked only from the files in the check directory and did not run git or cargo. I did run sha256sum and wc -c over the files I was given. The results match what scan.log prints for gate.log (64477 / 0574502a...), test.log (157069 / 723bd4db...), negative.log (161899 / 556752ff...), run.log (116015 / 3c487c11...), independent-check-1.md (18080 / 522137b0...) and the record .md (9263 / 658fada1...). spec-0011-editing-cells.md hashes to c4a1be26..., which is the uncommitted hash every log prints. phase.diff is one combined diff of b58e894..d7a7a7f, so I cannot split it by commit.

======================================================================
1. Does the diff do what the spec (as amended) says for P1, and nothing else?
======================================================================

Mostly yes. The departures are the ones the report already lists (F-a to F-d), plus two things I cannot confirm.

Matches the spec:
- Value (cells.rs, "pub enum Value"):
  - The Number, Text and Boolean forms are as "Module `cells`" says.
  - The boundary is written as "number == 0.0 || (1e-7..1e21).contains(&magnitude)", then Display, else {:E}.
  - A carriage return is written as &#13;.
  - The forbidden-character set is as the spec lists it.
  - check() refuses with the two messages the spec gives.
  - lib.rs re-exports Value.
- set_cell, clear_cell and drop_calc_chain have the signatures the spec gives.
- drop_calc_chain returns Ok(false) before reading content types when there is no chain, and otherwise records the three conditional removals.
- tools/emend-copy reads workbook() and content_types(), then calls package.drop_calc_chain()?. The tool's own copy of the recipe is deleted.
- validate(): "no such part", "an edit to a removed part", CELL_EDIT_REPLACED, then value.check() over every set.
- rewrite_worksheet:
  - root check; missing r refused (explicit_row, and cell_tag's reference.ok_or(NO_EXPLICIT_REFERENCES));
  - order check "if reference.row != open || reference.col <= last_col", which covers ruling 4's other-row case;
  - shared, array and dataTable refused, with dataTable in FormulaTag::Array (ruling 3), and an unparseable ref refused through "area.ok_or(IN_SHARED_OR_ARRAY)";
  - the unmodelled-attribute check sits only inside "if pending.peek() == Some(reference)" (ruling 4);
  - new elements take prefix_of the enclosing element; an empty row/ or sheetData/ is opened up;
  - new rows are written before the row start tag, after earlier whitespace tokens;
  - the final "if pending.sets(None) { return Err(NO_SHEET_DATA) }" means a clear alone is a no-op (ruling 2).
- write_to drops the chain through "removed_formula && !self.calc_chain_pending()? && self.drop_calc_chain()?" and then validates again. That is the spec's rule, with "pending" meaning the relationship is already recorded.
- The catalogue lists the eight messages with the ruling 3 wording.

Departures and extras (F-a, F-b, F-c and F-d are already in the report):
- F-a: the rewrite raises three existing messages the spec does not list: "a cell reference is not of the form A1", "a row number is not a number" and "a formula has a type this layer does not know". write_to's doc does not name them.
- F-b: edits are held in a growing Vec. The BTreeMap is built per part at write time.
- F-c: a clear of an absent cell still produces a replacement entry.
- F-d: a_chain_the_caller_already_dropped_is_dropped_once checks only the final state.
- decimal becomes pub(crate) in cells.rs. It supports explicit_row and is harmless.
- Not in the report. Spec P1 asks for "the output surveyed". replacing_a_normal_formula_drops_the_chain_in_the_same_write checks only workbook().calc_chain.is_none() and overrides().len() == 2, not a survey (check 1 item d).
- Not in the report. Spec P1 asks for output "read by the independent parser". The tests compare stored bytes through stored_data and assert_part, and no XML parser is used (check 1 item g). Byte equality is stricter, so I do not count this as a defect. But the report says nothing about either d or g.

Cannot confirm from the primaries:
- That 2e3bb2a holds "nothing else" and that d7a7a7f holds "rulings 2 and 3 with their tests ... and nothing else" (ruling 5).
  - Partial evidence: gate.log line 829 says the last commit that modified cells.rs is aa87cbc, so d7a7a7f did not touch cells.rs.
  - test.log lines 2162 to 2164 show 7 catalogue entries at aa87cbc and 8 at d7a7a7f.
- That the doc commit's four files are "unedited" (report line 4).
- That the set-equality test "reads the module that raises it" ("Errors"). It is not in the diff. The only statement about it is inherited prose in negative.log's script.

======================================================================
2. Do the logs contain every line the spec requires?
======================================================================

P1 test list. test.log lines 1820 to 1853 name 34 added tests. Every case in the P1 list maps to one:
- number replaced: a_number_cell_replaced_keeps_its_style_and_loses_its_type
- text escaping: a_text_is_written_inline_and_escaped (spaces, tab, LF, &#13;)
- boolean, both values: a_boolean_is_written_as_one_or_zero
- the eight number forms: a_numbers_text_is_display_between_the_bounds_and_exponent_outside
- NaN and both infinities: a_number_that_is_not_finite_is_refused
- insert before, between, at end: a_new_cell_goes_where_its_column_puts_it
- new rows, and an empty sheetData: a_new_row_goes_where_its_number_puts_it, an_empty_sheet_data_is_opened_up_for_new_rows
- several edits: several_edits_in_one_row_and_in_several_rows_go_in_one_write
- set twice, and set then clear: the_last_edit_recorded_for_a_cell_wins
- clear of an absent cell: clearing_a_cell_that_is_not_there_changes_nothing
- clear styled and unstyled: a_cleared_cell_keeps_its_style_or_goes_with_its_own_bytes
- cm refused: a_cell_with_an_attribute_this_layer_does_not_model_is_refused
- shared master, follower, array master, inside the ref, outside accepted: a_cell_in_a_shared_or_array_formula_is_refused
- normal formula with and without a chain: replacing_a_normal_formula_drops_the_chain_in_the_same_write, replacing_a_formula_without_a_chain_changes_the_sheet_alone
- caller already dropped the chain: a_chain_the_caller_already_dropped_is_dropped_once
- row or cell without r: a_worksheet_without_explicit_references_is_refused
- descending row, descending cell, repeated reference: a_worksheet_out_of_order_is_refused (it also covers a repeated row and a cell in another row)
- two sheets: edits_to_two_sheets_go_in_one_write
- refused by root: a_cell_edit_to_a_part_that_is_not_a_worksheet_is_refused_by_root
- missing, removed, replaced part: a_cell_edit_to_a_missing_removed_or_replaced_part_is_refused
- bytes outside sheetData: every_byte_outside_sheet_data_is_kept
- strict namespace: the_strict_namespace_is_edited_as_the_transitional_one
- read back: the_values_written_read_back_through_the_cell_reader
- no temporary, no output: a_refused_cell_edit_leaves_no_temporary_and_no_output
- first commit's test: the_method_records_what_the_tools_recipe_recorded
- ruling 2 and 3 tests: a_set_on_a_worksheet_without_sheet_data_is_refused, a_data_table_is_refused_as_an_array_is

Host and i686 each ran 382 with the same names (test.log lines 2376 and 2377). The caveats in 1 on "surveyed" and "independent parser" apply here too.

Negative log requirement. P1 adds no operation that edits a cell, and negative.log says and measures that:
- Lines 1879 to 1897: 8 messages added, exactly the catalogue's eight.
- Lines 2577 to 2586: each is "raised by a case here 0, named in the package tests N".
- --drop-calc-chain runs through the built binary in N16 to N22 (lines 2034 to 2089) and D1 to D5 (lines 2294 to 2336).
- The binary hashes at lines 1771 to 1774 equal gate.log lines 1073 to 1076. negative.log does not assert that equality; it only states it (line 1776).

Corpus acceptance for P1 ("Harness runs"). run.log shows each item for both targets:
- nine dropped: "nine entries dropped in all, one per chain" (lines 1370 and 1571)
- two parts changed: "one of the two is [Content_Types].xml ..." and "the other is a rels part ..."
- eighteen files_identical yes: "files_identical yes: 18; no: 9"
- lo and lo_in: "lo ok: 27; lo_in ok: 27"

The record .md lines 17, 18 and 47 agree. The spec's stronger phrase "its output does not change" is now measured at run.log lines 1649 to 1655: "byte-identical to part 2 P3's 54; different 0; missing 0", with a control.

Ruling 6:
- gate.log (lines 14 to 16, 1080 to 1083), test.log (lines 11 to 14, 2402 to 2405), negative.log (lines 4 to 6, 2668 to 2671) and run.log (lines 16 to 18, 1692 to 1695) each name both files with sha256 in the header and account for exactly two status lines.
- scan.log has no status check and scans both files (lines 1257 and 1268).
- gate.log is rerun at d7a7a7f and hashes gate-at-aa87cbc.log by command (lines 11 and 12: 745e8967..., 61586 bytes).

spec-0001 rules:
- Phase commit given, compared with HEAD, with a control, both values printed:
  - gate.log lines 2 to 8 (null and short sha)
  - test.log line 2 and lines 15 to 17 (null only)
  - negative.log line 2 and lines 13 to 15 (null only)
  - run.log lines 2 to 8
  - scan.log lines 2 to 7
  - All pass.
- sha256 printed by command: every hash in the logs comes from a sha256sum line in a printed script. That includes the superseded attempts in the gate, test, negative, run and scan rerun headers.
- No absolute machine paths: none in any log.
  - The path-shaped strings are assembled fixtures (gate.log line 475; scan.log lines 319, 372, 373 and 404) or the fictional control at run.log line 981.
  - run.log uses "<corpus>", which is not one of the three placeholders spec-0001 names, but declares it (lines 31 to 34).
  - Non-ASCII and control bytes are 0 in all five logs, by my own grep.
- Bite claims measured: test.log lines 2178 to 2373 print each planted change, each run and the failing output, with the clean-worktree control at line 2373. See the check 1 comparison below for how weak some of that output is.
- Message counts printed by a log:
  - aa87cbc "seven new messages": test.log line 2166.
  - d7a7a7f "the two tests that pinned the old wording": test.log lines 2396 to 2399.
- A test a message says pins something exists at that commit:
  - 2e3bb2a: test.log lines 2382 to 2384. The message does not name its tests. The log chose the two names, which fit the description.
  - aa87cbc: five of seven at lines 2386 to 2390; the two missing ones are shown as 0 at lines 2391 and 2393. That is finding P1-F1, left as history by ruling 1.
  - d7a7a7f: lines 2392 and 2394.

New issues I found in the logs:
- gate.log script header change 3 (line 45) and the comment near line 267 say "THIS PHASE ADDS NO FILE" and "This phase ADDS no file". The log's own output says "files added in this phase: 1" (line 931, specs/0011-editing-cells.md). The claim is meant to be about code files, but as written it is false, and check 1 did not catch it.
- gate.log script line 411 still says "so the two new lines of Cargo.toml can be read against the licences they pull in". This phase does not touch Cargo.toml (the 9 files are at gate.log lines 841 to 849). It is inherited prose making a false claim about this phase, the same class as the three comments attempt 3 fixed.
- test.log header change 9 is titled "SECOND ATTEMPT". The third attempt's filter change is described only in the rerun note (line 4), not in the numbered header changes. The list also runs 9 before 8. Minor.
- scan.log (run 2, 13:25:48Z) scans only independent-check-1.md (line 1229, "check answers scanned: 1"). The spec puts the scan log last, so it has to run again after this answer. The report's "scan.log: SCAN" placeholder must cite that later run, which I cannot check.

Check 1 findings the report says were fixed, and whether the reruns show it:
- gate.log false "this phase" comments (check 1 lines 134 and 135): fixed.
  - The "added by THIS phase's own doc commit" and "both independent checks of this phase" text is gone.
  - The rerun header (line 10) now reads "aa87cbc's message said its seven choices were each pinned by a test; two were not."
  - But see the two new stale comments above.
- test.log mutations without assertion output (check 1 line 131): fixed in letter.
  - M2, M3 and M5, and M7's first failure, now print one line beyond the location: "refused: ()" (lines 2266, 2282, 2315, 2355). That is expect_err's own message showing the Ok value.
  - M4 prints "write_to: Package(...)" (line 2298).
  - It meets "the failing assertion's output", but for four of the failures that output says only that the write returned Ok, not what was written.
  - The M7 run also fails an unrelated test, a_cell_in_a_shared_or_array_formula_is_refused. The report does not mention it.
- test.log, the two message claims with no printing log (check 1 lines 71 and 72): fixed (test.log lines 2380 to 2399).
- negative.log title and "--cells, THIS PHASE'S OPERATION" (check 1 line 136): partly fixed.
  - The title is fixed (line 17).
  - The text "U1 to U13, --cells, THIS PHASE'S OPERATION" is still there verbatim at line 48. It is now disclaimed rather than corrected: lines 18 to 20 say "The family list that follows is part 3 P3's". Whether a printed check label was corrected cannot be checked, because attempt 1 is not in the primaries.
- run.log TERMS order and the unqualified P1/P2 (check 1 lines 137 and 138): fixed.
  - Line 24 now reads "the survey pass then the edit pass", which matches the timestamps at lines 1245 and 1331.
  - Line 29 now reads "M1 part 2 P1 and P2".
- run.log "output does not change" shown only by class counts (check 1 item f): fixed (lines 1649 to 1655).
- Report parenthetical "(the counts the two messages state)" and "Every log": both fixed in the new draft (lines 19 and 20).

======================================================================
3. Numbers and quoted strings in report-draft.md compared with the primaries
======================================================================

Found and matching (line numbers are report-draft.md's):
- Line 3: range b58e894..d7a7a7f and base (gate.log lines 808 and 809).
- Lines 4 to 7: the four full shas (gate.log lines 814 to 817).
- Line 6: "seven messages" (aa87cbc's message).
- Lines 7 to 9: "rulings 1, 2, 3, 5; finding P1-F1" (d7a7a7f's message: "rulings 2, 3 and 5 (finding P1-F1)", "by ruling 1"). The quoted new message matches the diff.
- Lines 12 and 13: "each pinned by a test" (aa87cbc's message); five at aa87cbc and two at 0 (test.log lines 2386 to 2393).
- Lines 16 and 17: c4a1be26... and 209b9b6c... (every log).
- Lines 27 and 28: 745e8967..., 6f1782a6... (gate.log lines 12 and 13).
- Line 30: 4 commits, 1 doc and 3 code, 9 files, all scored, nine PASS lines (gate.log lines 818, 825, 850, 1040 and 1058).
- Lines 31 and 32: no gate file, no self-test; CLAUDE.md classifier (header change 2).
- Lines 36 and 38: 8b062453..., 8d807726... (test.log lines 6 and 7).
- Line 40: 382 each, same names; 34 added, all package; 0 removed; cells 86 and tool 14 unchanged (test.log lines 1356, 1784, 1820, 1854, 2097 and 2119).
- Line 42: 7 and 8 (test.log lines 2163 and 2164).
- Lines 44 to 48: test.log lines 2382 to 2399, including "2 at aa87cbc" and 0 at the phase commit.
- Lines 49 and 50: seven mutations, each failing its named test, clean control passes. Wording caveat in section 2.
- Line 53: c75857c5... (negative.log line 9).
- Lines 55 to 57: N16 to N22, D1 to D5; 8 added, exactly the catalogue's eight.
- Line 59: run directory <corpus>/runs/2026-09-22-d7a7a7f-r2 (run.log line 1227).
- Line 60: 8fa2278c..., b35adafe..., 95aafd69... (run.log lines 13 to 15).
- Lines 66 to 71, both targets: 9; 27 rows; ok, lo and lo_in 27; 18/9; totals 843/18/9/0/0; Override 45 -> 44; Relationship 21 -> 20.
- Lines 72 to 74: 54 byte-identical, with a control (run.log lines 1653 to 1655).
- Lines 77 and 78: 9263 / 658fada1..., 197958 / 1bfb6b4f... (run.log lines 1678 and 1679, scan.log lines 1230 and 1240, and my own sha256sum of the .md).
- Lines 82 to 91: the choices and M1 to M7 (aa87cbc's message; spec rulings 2 to 4; test.log lines 2230 to 2366).
- Lines 95 to 107: the three quoted messages and the F-b, F-c and F-d facts appear in the diff (cell_tag, explicit_row, formula_tag; Recorded::cell_edits; the rewrite always returns a replacement; assert_chain_gone).

Found only as authored prose, not measured in any primary I have:
- Line 38: "FAILED seven checks of its own" and the blank-line explanation. These appear only in test.log's rerun note (line 4). Attempt 2 is hashed but not in the primaries, so "seven" cannot be checked.
- Line 28: "three comments". This appears in gate.log's rerun note and header change 7. Check 1 quoted two sentences plus the rerun header, so "three" depends on how they are counted.
- Line 4: "unedited". No primary shows it.

Missing or different:
- Report lines 93 to 107 list the check 1 findings not fixed as F-a to F-d only. Check 1 items d ("output surveyed" only partly) and g (no independent XML parser) are neither fixed nor listed. Check 1's note that ruling 5's "nothing else" cannot be verified from a combined diff is also not addressed.
- Report line 22 says every superseded attempt is "hashed by command in the header of the log that replaced it". That is true. But the report does not say that scan.log must run a third time after this check, and until it does the "SCAN" line has no primary.
- Line 109, "Independent checks: CHECKS", is still a placeholder.

Summary of disagreements:
1. gate.log still carries false "this phase" prose: "ADDS NO FILE" against its own "files added in this phase: 1", and "the two new lines of Cargo.toml".
2. Four of the mutation failures print only "refused: ()". It meets the rule in letter and says little.
3. negative.log's inherited "THIS PHASE'S OPERATION" line is disclaimed, not corrected.
4. The report omits check 1 items d and g.
5. Ruling 5's and the first commit's "nothing else" cannot be confirmed from a combined diff.
6. scan.log has to run again after this check.