Independent check 1 of M1 part 4 P1

Scope and limits. I worked only from the files in the check directory. phase.diff is one combined diff of b58e894..d7a7a7f, not one diff per commit. So I can check what the range does as a whole, but I cannot tell from a primary which of the three code commits holds which change. Where the spec or a ruling gives a rule for a single commit, I say below that I could not confirm it. I compared the four sibling logs and the record .md with the byte counts and sha256 values scan.log and run.log print for them, and they match (gate.log 64316 / 6f1782a6..., test.log 148361 / 8b062453..., negative.log 160764 / c75857c5..., run.log 110667 / 8fa2278c..., record .md 9257 / b35adafe...).

======================================================================
1. Does the diff do what the spec (as amended) says for P1, and nothing else?
======================================================================

Mostly yes. Details:

Doc commit 8d865e9. The diff carries CLAUDE.md (only line 1 changed: "-c/0c0c# CLAUDE.md..." became "+# CLAUDE.md..."), ROADMAP.md, specs/0010 (+60 lines) and a new specs/0011 (469 lines). That matches "Phases" ("Part 4's first doc commit carries, unedited: CLAUDE.md ..., ROADMAP.md, specs/0010 ... and this spec"). I cannot check "unedited" from the primaries.

First code commit (drop_calc_chain):
- Package::drop_calc_chain (phase.diff, from the "+    pub fn drop_calc_chain(&mut self) -> Result<bool, Error>" hunk) reads the workbook, finds the calcChain relationship and records the relationship, the Override when there is one, and the part when there is one. It returns true, or false having recorded nothing. This matches "Module `package`".
- tools/emend-copy now calls package.workbook()?, package.content_types()? and then package.drop_calc_chain()?, and the tool's own copy of the recipe is deleted. This matches.
- Test: the_method_records_what_the_tools_recipe_recorded covers a chain, a chain with no Override, a chain with no entry, and no chain. the_method_records_nothing_when_the_workbook_is_refused covers the refused read. Both go beyond the spec's minimum; fine.
- "nothing else" in this commit: I cannot confirm it (combined diff).

Second and third code commits (Value, set_cell, clear_cell, the rewrite, eight messages):
- Value (cells.rs): the Number, Text and Boolean forms match the spec text. The Display/{:E} boundary is written as "(1e-7..1e21).contains(&magnitude)" or zero. A carriage return is written as &#13;. The forbidden-character set matches.
- lib.rs re-exports Value.
- Validation in write_to: "no such part", "an edit to a removed part", "a cell edit to a part this write replaces", and value.check(). This matches.
- rewrite_worksheet:
  - Refuses a missing r ("NO_EXPLICIT_REFERENCES").
  - Refuses rows or cells out of order, including a cell whose reference names another row ("if reference.row != open || reference.col <= last_col"), as ruling 4 says.
  - Refuses shared, array and dataTable formulas: dataTable is in FormulaTag::Array (ruling 3), a ref is tracked and an unparseable ref is refused.
  - Refuses an unmodelled attribute on an edited cell only.
  - Refuses a set when there is no sheetData, as Error::Package NO_SHEET_DATA, while a clear alone passes (ruling 2).
  - New elements take the prefix of the element they go into.
  - An empty <row/> is opened up.
  - A new row goes before the next row's start tag, after whitespace (ruling 4).
  - The chain is dropped when a normal formula is removed, unless calc_chain_pending() says it is already recorded.
- The catalogue in package.rs's header lists the eight messages with the new wording (ruling 3).

Ruling 5 ("the third code commit holds rulings 2 and 3 with their tests ... and nothing else"): I cannot confirm this from the combined diff. test.log's catalogue block (7 entries at aa87cbc, 8 at d7a7a7f) fits it but does not prove it.

Things that are extra, differ from the spec, or are worth noting:

a) The rewrite raises three existing cells-module messages the spec does not list for the rewrite:
   - "a cell reference is not of the form A1" (phase.diff line 734)
   - "a row number is not a number" (explicit_row)
   - "a formula has a type this layer does not know" (formula_tag, reached only for an EDITED cell; for an unedited cell the error is ignored by "if let Ok(...)")

   They are reasonable reuses and add nothing to the catalogue. But:
   - write_to's doc Errors list names only "the worksheet part's root is not worksheet" of the cells messages, not these three.
   - No package test pins any of them on the write path. negative.log's C family counts them only through the cells tests and the --cells cases.

b) Spec "32-bit and streaming" says a worksheet's edits are held "as a map from (u32, u32) to Option<Value>". Recorded::cell_edits is a Vec that gains an entry on every set_cell or clear_cell, duplicates included. The map (a BTreeMap<CellRef, ...>) is built only per part at write time. So memory grows with every recorded edit, not with distinct cells. This is a small departure from the design text.

c) Spec P1 "clear of an absent cell (byte-identical)". clearing_a_cell_that_is_not_there_changes_nothing compares the worksheet part's stored text with the input. rewrite_worksheet still produces a replacement for any part that has edits, clear-only ones included. So the zip entry itself is rewritten, and the test does not show that it is byte-identical at the entry level. This depends on how "byte-identical" is read; I note it as an uncertainty.

d) Spec P1 "a normal formula replaced ... (the three removals applied, the output surveyed)". replacing_a_normal_formula_drops_the_chain_in_the_same_write reopens the output and checks workbook().calc_chain is None and the Override count is 2. That is a partial survey, not the tool's --survey.

e) Spec P1 "a formula replaced when the caller already called drop_calc_chain (applied once)". a_chain_the_caller_already_dropped_is_dropped_once asserts only the final state (assert_chain_gone). It does not count the removals recorded, so "once" rests on a double removal changing the output or being refused. I cannot confirm that from the diff.

f) Spec text: "--drop-calc-chain calls it and its output does not change (P1's corpus run shows that against part 2 P3's expectations)". run.log checks the class expectations (nine dropped, eighteen identical, two changed per chain). It does not compare output bytes or hashes with part 2 P3's outputs. That meets the acceptance line under "Harness runs" but not the stronger phrase "its output does not change".

g) The spec asks that test outputs be "read by the independent parser". The tests compare stored bytes through stored_data and assert_part; the comment says "the independent zip reader". No independent XML parser is used, but byte comparison is stricter, so I do not count this as a defect.

Nothing I found contradicts a ruling.

======================================================================
2. Do the logs contain every line the spec requires?
======================================================================

"Phases" list:
- gate.log, test.log (both targets), negative.log, run.log and scan.log are present. There is no size.log, which is correct for P1.
- scan.log has the self-scan (lines 16 to 27) and the narrowed f_home with controls (lines 979 to 985).
- Commit message counts:
  - aa87cbc's "seven new messages" is printed by test.log line 2090 ("the catalogue holds seven cell-edit messages at the commit whose message says seven: yes").
  - FINDING: d7a7a7f's message says "the message becomes ... in the catalogue, the write_to doc and the two tests that pinned the old wording". No log prints that count of two, and the message does not say that no log prints it. spec-0001 (lines 127 to 131) requires one or the other. Under ruling 1's widened rule, the message also claims that tests pin something without naming them.
  - 2e3bb2a's message describes its tests ("the recipe as the tool wrote it, kept in the test module, and the method record the same removals ...") without naming them. No log shows that those tests exist AT 2e3bb2a; test.log runs only at d7a7a7f. Not verifiable from the primaries.

P1 test list compared with test.log's added names (test.log lines 1746 to 1779). Every named case has a test:
- replace number, s kept, t="n" gone: a_number_cell_replaced_keeps_its_style_and_loses_its_type
- text: a_text_is_written_inline_and_escaped (& < > escaped, spaces, tab, LF, CR as &#13;)
- boolean, both values: a_boolean_is_written_as_one_or_zero
- number forms: a_numbers_text_is_display_between_the_bounds_and_exponent_outside (2.5, 1, -0, 0.1+0.2, 1e21, 1.5e-8, 1e-7, f64::MAX)
- NaN and both infinities: a_number_that_is_not_finite_is_refused
- insert before, between, at end: a_new_cell_goes_where_its_column_puts_it
- new row between, before first, after last: a_new_row_goes_where_its_number_puts_it
- into an empty sheetData: an_empty_sheet_data_is_opened_up_for_new_rows
- several edits, one row and several rows: several_edits_in_one_row_and_in_several_rows_go_in_one_write
- set twice, and set then clear: the_last_edit_recorded_for_a_cell_wins
- clear absent: clearing_a_cell_that_is_not_there_changes_nothing (see 1c)
- clear styled and unstyled: a_cleared_cell_keeps_its_style_or_goes_with_its_own_bytes
- cm refused: a_cell_with_an_attribute_this_layer_does_not_model_is_refused
- shared master, follower, array master, inside the ref; outside accepted: a_cell_in_a_shared_or_array_formula_is_refused
- formula with chain, and without: replacing_a_normal_formula_drops_the_chain_in_the_same_write, replacing_a_formula_without_a_chain_changes_the_sheet_alone
- caller already dropped the chain: a_chain_the_caller_already_dropped_is_dropped_once (see 1e)
- row without r, cell without r: a_worksheet_without_explicit_references_is_refused
- descending row, descending cell, repeated reference: a_worksheet_out_of_order_is_refused
- two sheets in one write: edits_to_two_sheets_go_in_one_write
- not a worksheet, refused by root: a_cell_edit_to_a_part_that_is_not_a_worksheet_is_refused_by_root
- missing, removed, replaced part: a_cell_edit_to_a_missing_removed_or_replaced_part_is_refused
- bytes outside sheetData: every_byte_outside_sheet_data_is_kept
- strict namespace: the_strict_namespace_is_edited_as_the_transitional_one
- read back as InlineString, number(), boolean(): the_values_written_read_back_through_the_cell_reader
- refused write leaves nothing: a_refused_cell_edit_leaves_no_temporary_and_no_output
- first commit's test: the_method_records_what_the_tools_recipe_recorded
- the rulings' tests: a_set_on_a_worksheet_without_sheet_data_is_refused, a_data_table_is_refused_as_an_array_is

Both targets ran 382 with the same names (test.log lines 2272 to 2274).

Negative log: P1 adds no operation, so the log shows instead that none of the eight messages is reachable. negative.log lines 2567 to 2576 print each message with "raised by a case here 0, named in the package tests N", and N16 to N22 and D1 to D5 run --drop-calc-chain. That is an acceptable reading of "where an operation reaches them".

Corpus acceptance (P1 under "Harness runs"): for both targets run.log prints:
- the nine chains dropped: "nine entries dropped in all, one per chain"
- the two parts changed: "one of the two is [Content_Types].xml ...", "the other is a rels part ..."
- eighteen files_identical yes
- lo ok 27 and lo_in ok 27

The record pair's .md matches (lines 17 to 18, 47).

Ruling 6:
- gate.log, test.log, negative.log and run.log each print both planning files' sha256 in the header and account for exactly two status lines at the end (gate.log line 1080, test.log line 2277, negative.log line 2658, run.log line 1620).
- gate.log is rerun at d7a7a7f, and its header hashes gate-at-aa87cbc.log by command (gate.log lines 10 to 12).
- scan.log has no status check and prints the two hashes in its body (around line 1250). That is acceptable, since the ruling covers only logs whose status check would count the files.

spec-0001 rules:
- Phase commit given and compared with HEAD, with a control:
  - gate.log lines 3 to 8 (null commit and short sha)
  - test.log lines 2, 10 to 12 (null only)
  - negative.log lines 2, 9 to 11 (null only)
  - run.log lines 3 to 8
  - scan.log lines 3 to 7

  All print both values.
- sha256 values printed by command: I found none that was typed. The planning-file hashes, the superseded attempt and the record hashes are all printed by commands.
- No absolute machine paths: none. run.log uses "<corpus>", which is not one of the three placeholders spec-0001 names (<repo>, <scratch>, <home>), but run.log lines 34 to 37 declare it. The only literal home-shaped string is a fictional control ("/home/someone/emend-testing/...", run.log line 917).
- Claims that a test bites must be measured. The mutation block (test.log lines 2102 to 2266) prints each planted change, each run and each failing test. FINDING: its header line "the failures, with the values each assertion saw" holds only for M1, M6 and part of M7. For M2, M3, M4, M5 and M7's first failure, only the "panicked at crates/emend/src/package/tests.rs:NNNN" line is printed, with no assertion message or values. spec-0001 lines 101 to 107 ask for "the failing assertion's output".

Stale or inaccurate prose inside the printed scripts and log headers (the "audited against specs/0001" claims did not catch these):
- gate.log lines 97 to 104: "(M1 part 3 P1 ruling 11, added by THIS phase's own doc commit)" and "The first two attempts of this log read the commit from HEAD ...; both independent checks of this phase found it". Both are false for this phase: its doc commit does not touch specs/0001, and no check existed when the log ran.
- gate.log line 10 (the rerun header): "aa87cbc's message named two tests that did not exist". aa87cbc's message named no tests. It said "each pinned by a test", and the spec's rulings say two of the seven had none.
- negative.log line 14: the script's first line reads "# Milestone 1 part 3 P3 negative.log (specs/0010 ...)", and lines 44 to 46 say "U1 to U13, --cells, THIS PHASE'S OPERATION". Both are carried over from part 3.
- run.log lines 22 to 24: "within a label the edit pass then the survey pass". The log shows the survey pass first: host survey 12:51:28 to 12:53:57, then edit 12:53:57 to 12:56:26, and the same order for i686.
- run.log line 20 ("longer than P2's single pass") and lines 28 to 29 ("the figure P1 and P2 measured") refer to earlier phases without saying which part.

======================================================================
3. Numbers and quoted strings in report-draft.md compared with the primaries
======================================================================

Found, and matching:
- The range b58e894..d7a7a7f, with the base as the doc commit's parent (gate.log lines 809 to 810).
- The four full shas (gate.log lines 814 to 817).
- The doc commit's four files (phase.diff).
- "seven messages" (aa87cbc's message).
- Rulings and finding named for d7a7a7f (its message: "rulings 2, 3 and 5 (finding P1-F1)", "by ruling 1").
- The two test names (test.log, d7a7a7f's message).
- The planning-file sha256 values c4a1be26... and 209b9b6c... (every log).
- gate-at-aa87cbc.log 745e8967..., 61586 bytes (gate.log line 12).
- "4 commits", "1 doc, 3 code", "9 files", "nine PASS lines", no self-test (gate.log).
- 382 and 382 with the same names; 34 added, all package, none removed; cells and tool counts unchanged; catalogue counts 7 and 8; seven mutations and the clean control (test.log).
- N16 to N22, D1 to D5; 8 added, exactly the catalogue's eight (negative.log).
- 9 with calc_chain=1; 27 rows; ok, lo and lo_in 27; files_identical 18/9; totals 843/18/9/0/0; Override 45 -> 44; Relationship 21 -> 20; same classes and same survey lines (run.log).
- Record .md 9257 / b35adafe..., .txt 197958 / 95aafd69... (run.log lines 1605 to 1606; scan.log).
- "the check answers went through all three greps as prose" as the only failed structural check (scan.log line 1313, "structural checks failed: 1").
- Choices 1 to 7, "was Error::Xml at aa87cbc" and "was removable at aa87cbc" (aa87cbc's message); mutations M1 to M7 (test.log).

Not found, or found different:
- Report line 26, "(the counts the two messages state)": d7a7a7f's message does not state 8 or "eight". It says "The new message joins the catalogue with its test." Only aa87cbc's message states a count (seven).
- Report lines 11 to 12, "its claim ... was false for two of the seven choices at aa87cbc": no log or diff in the primaries measures the test set at aa87cbc. The claim rests on the spec's P1 rulings (planning's confirmation), which is not one of the listed primaries.
- Report line 18, "Every log prints both hashes and accounts for exactly those two status lines": this is true of gate, test, negative and run. The scan.log I was given (the first run) prints both hashes but has no status check, so "every log" overstates it for scan.log. I cannot see the final scan.log.
- Report lines 40 to 41 describe scan.log as "the final run after the check answers". I have only the first run, so the final run's content is unverified.
- Report line 58, "Independent checks: CHECKS", is a placeholder still to be filled.

Summary of the substantive findings:
1. d7a7a7f's message states a count ("the two tests") that no log prints, and it names no tests.
2. For five of the mutations, the bite evidence has no assertion output.
3. gate.log carries false "this phase" claims from part 3 P1, and its rerun header misdescribes aa87cbc's message.
4. run.log's TERMS contradicts the order it actually ran in.
5. The rewrite raises three undocumented, unpinned reused messages.
6. The edits are held as a growing Vec, not the map the spec describes.
7. Two report statements are slightly inaccurate or unsupported by a primary (the "two messages state" parenthetical, and "Every log" for scan.log), and the CHECKS placeholder remains.