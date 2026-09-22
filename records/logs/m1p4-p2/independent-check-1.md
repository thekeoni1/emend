## Independent check, M1 part 4 P2 (check 1)

I read only the files in the check directory. Paths below are relative to it. I ran no git command. Line numbers are 1-based. For run.log, the numbers are the file's own lines.

### Question 1: does the diff do what the spec says and nothing else?

**How the diffs relate.** I extracted each code file's hunks from `diff-combined-57d2334..595cf8b.diff` and compared them with the per-commit diffs:
- lines 14-285 of the combined diff are identical to `diff-4b7f55c.diff:32-303`;
- lines 599-1052 of the combined diff are identical to `diff-595cf8b.diff:28-481`.

The only other files in the combined diff are ROADMAP.md, `specs/0001` and `specs/0011` (combined lines 1-13 and 286-598). That matches `doc-commit-4cf3671.stat` (3 files) and gate.log:805-812 (7 files).

**4b7f55c (checked against `diff-4b7f55c.diff`).**
- **Ruling 1: done.**
  - write_to's doc now names the three messages (diff-4b7f55c.diff:74-78).
  - There is one write-path test each: `a_cell_reference_not_a1_refuses_the_edit` (265-274), `a_row_number_not_a_number_refuses_the_edit` (276-282) and `an_unknown_formula_type_refuses_an_edit_to_its_cell` (284-301).
  - The module catalogue names each test (39-64). The spec did not ask for the catalogue entries, but they document the ruling and are harmless.
  - One cosmetic reflow: in the "a row number is not a number" entry, the unchanged sentence "a row is numbered from 1..." is re-wrapped (48-54). No wording changed.
- **Ruling 4: done, through a refactor the spec did not ask for.**
  - The chain rule's condition moves unchanged out of write_to into a new private `Package::drop_calc_chain_once` (86-95, 102-115). The old comment becomes its doc.
  - The refactor is justified by ruling 4: it is what lets a test read what was recorded before the package is consumed.
  - The already-dropped test counts (1,1,1) after the caller's drop and (1,1,1) after the rule (221-243).
  - A new control counts (0,0,0) before the rule and (1,1,1) after it (245-263).
  - Caveat: both tests call `drop_calc_chain_once` directly. The count is therefore of the extracted function, not of write_to's own call. That write_to calls it is shown only indirectly: under R4 the control fails inside write_to with `Unsupported("two changes for one entry")` (test.log:2339-2340).
- **Ruling 8: done.**
  - `refused` and `refused_edit` no longer use `expect_err`. A write that is not refused panics with what it wrote (125-185).
  - The M2 test is rewritten the same way (188-203).
  - The spec says "printing the error they refused on" (spec/0011-editing-cells.md:431-432). When the write was not refused there is no error to print, so printing the output is the only faithful reading. I accept it.
  - test.log:2515 shows 20 direct `write_to(...).expect_err("refused")` sites remain in the package tests (21 at the parent). Ruling 8 named only four, so this is in scope, but those 20 would still print only "refused: ()" if they failed.
- Nothing else is in this diff.

**595cf8b (checked against `diff-595cf8b.diff`).**
- **`--edit-sample` matches "Harness runs" (spec/0011:295-316):**
  - A and B are picked as Number or SharedString cells with a value and no formula (152-159), and M is the largest row (151);
  - the edits are 42.5, a clear, and Text/Boolean at (M+1, 1) and (M+1, 2) (194-201);
  - the output is written, reopened and read, and read_back is computed as specified (217-234);
  - the tool asserts `cells_after == cells_before + 2 - dropped` and exits 2 with a message otherwise (236-245);
  - the line is `edit` then tab-separated key=value, counts only (246-264).
- **Not specified by the spec, but justified:**
  - `Error::NoWorksheet` (87, 182): the spec does not say what happens when no sheet is a Worksheet.
  - `Error::NoNewRow` (88, 184): guards `M + 1` against u32 overflow.
  - Both are defensive and both are exercised: NoWorksheet in `the_edit_sample_refuses_a_workbook_with_no_worksheet` (456-480), both in negative.log E8 and E9.
- **Small issues in the tool code:**
  - Reopening the OUTPUT maps a failure to `Error::ReadInput` (217), so the message would name the wrong file.
  - The `CellsAfter` refusal fires after the output has been renamed into place (210-215, 240-245), so a refused run leaves an output file. The tool's other refusals leave no output. The spec only requires "exit 2 with a message".
- **Tests:**
  - Four integration tests on `cells_specs`, with the sheet replaced by a stored one (292-308, 378-480). Three of them compare the rewritten sheet's bytes with hand-built bytes.
  - The fourth test (no worksheet) compares no bytes.
  - The usage test gains three shapes (278-281).
- Nothing else is in this diff.

### Question 2: do the logs contain every line the spec requires?

**The P2 acceptance (spec/0011:329-352 as corrected by 614-630):**

| Requirement | Evidence | Verdict |
|---|---|---|
| First worksheet part changed; every other entry identical and unflagged | run.log:1413, 1416-1417 (host); 1529, 1532-1533 (i686). Check logic: run.log:691-711 | present |
| order_preserved yes, files_identical no | run.log:1343, 1459 | present |
| lo and lo_in ok | run.log:1342, 1458 | present |
| Prefix and suffix bytes identical | cmp.py run.log:1208-1214; accept_one run.log:567-568; every per-workbook line reads yes, run.log:1360-1412 and 1476-1528 | present |
| Counts asserted against the tool line: changed = set - 2 + cleared - dropped; removed = dropped; added 2; sum = cells_before | run.log:569-572, with cells_in and cells_out tied at 573-574 | present |
| Reference-level assertions from the comparison's own pick | run.log:1231-1243 (refs_as_edited, a_value, b_kept_s_no_value, new_cells), asserted at run.log:575-580 | present (see note below) |
| read_back = set | run.log:564 | present |
| Totals | run.log:1414, 1418-1419; 1530, 1534-1535 | present |
| Control on a planted difference | Generated pairs run.log:1270-1289 (five planted faults each reported, unstyled B counted as removed, accept_one controls); real pair run.log:1420-1423, 1536-1539 (added=3, refused) | present |
| Both targets | run.log:1309, 1425; agreement 1541-1546 (same entry classes, same edit lines, 27 byte-identical outputs) | present |

Note on the reference-level row: in all 27 workbooks, on both targets, `b_styled=1` and `dropped=0`. The unstyled-B branch of the acceptance is therefore exercised only by the generated control (run.log:1273-1275), the integration test and negative.log E1, never by the corpus.

The script accepts a `descriptor` flag on the changed worksheet entry (run.log:708). P2's acceptance text is silent on flags for the changed entry; only P3 cites `specs/0008` for `descriptor` (spec/0011:357-358). The record shows `descriptor` 17 on worksheet entries (m1p4-p2-...md:25, 54).

**The specs/0001 log rules:**
- **Phase commit given to the script and compared with HEAD, with a control:** present in every log.
  - gate.log:1-8 (null and short-sha controls);
  - test.log:9-11 (null control only);
  - negative.log:8-10 (null control only);
  - run.log:1-9;
  - scan.log:1-7.
- **Every sha256 printed by a command:**
  - the planning file: gate.log:11-12 and 1024; test.log:7-8; negative.log:4-5 and 2886; run.log:11-12 and 1578; scan.log:1167;
  - the binaries: gate.log:1016-1019, re-checked at run.log:1296-1304;
  - the record pair: run.log:1562-1563;
  - the logs: scan.log sibling block, under "command: wc -c and sha256sum". All of these equal the sha256 values I computed over the check-directory copies.
- **No absolute paths:** my grep found none. The only home-shaped string is a fixture, `/home/someone/emend-testing/...` in run.log:819, written literally. scan.log's f_home treats another account's home as a non-match by design (control listed in scan.log). Note only.
- **Counts a commit message states, printed by a log:**
  - 595cf8b's "three shapes": test.log:2521-2522.
  - 4b7f55c's "three" messages: test.log:2200-2204.
  - 4b7f55c's message also states "(1, 1, 1) both times" and "(0, 0, 0) before the rule". No log prints (0, 0, 0). (1, 1, 1) appears only as the right-hand side of R4's failed assertion (test.log:2338). The test.log header says it prints "the one count a message states" (test.log, script item 6). That undercounts.
- **Tests a message names exist at that commit:** test.log:2503-2520, all yes, with a control.
- **Bite claims measured:** ten mutations, each printing the planted change, the run and the assertion's output (test.log:2275-2485), with a clean-worktree control (2487-2496).
  - Exception: 4b7f55c's message says the unknown-formula test "also edits the cell beside it, to show only the edited cell's formula is read" (diff-4b7f55c.diff:13-14), and the report repeats it (report-draft.md:13-14).
  - No mutation makes the rewrite read every cell's formula. That implicit bite is not measured.

### Question 3: does every number and quoted string in report-draft.md appear in a primary?

Everything I checked is found in a primary, except the following.

- **report-draft.md:37, "the 26 .xlsx files": NOT FOUND.** No primary says 26. The corpus is 27 workbooks (run.log:1306-1307; record .md files 27). The number may be right if one workbook is not .xlsx, but nothing I was given shows that.
- **report-draft.md:4, "unedited at the hashes planning gave": NOT VERIFIABLE.** gate.log:805-811 prints the committed hashes, but no primary holds the hashes planning gave. `spec/0001-bootstrap.md` in the check directory does hash to the committed 84c149d3....
- **report-draft.md:36, "Every log prints its hash and accounts for its one status line": OVERSTATED.**
  - gate, test, negative and run logs do both (gate.log:1023; test.log status block; negative.log:2884; run.log:1576).
  - scan.log prints the planning file's hash (1167) but has no working-tree status check at all.
- **report-draft.md:59, "the tool's two reachable messages": a characterization, not in a log.**
  - The tool has three new messages. `CellsAfter` is reached only under mutation T1 (test.log:2359, 2364).
  - negative.log never says it is unreachable.
- **report-draft.md:26, "with the sheet stored and its bytes compared by hand": imprecise.** True for three of the four tests; the no-worksheet test compares no bytes (diff-595cf8b.diff:456-480).
- **report-draft.md:70, "All 17 flags are `descriptor`": supported by the record, not by run.log.** run.log only prints "flagged 17" and allows `''` or `descriptor`. The record prints descriptor 17 on worksheet entries (m1p4-p2-...md:25, 54).

Numbers and strings I confirmed:
- **Commit list:** report lines 3-6 match gate.log:774-786.
- **Gate:** report lines 41-44 (3 commits, 7 files, nine PASS lines, no gate file) match gate.log:787, 812, 815-816, 990-1004.
- **Tests:** 390/382/8 (4 package, 4 tool) match test.log:1396, 1832, 1863, 1867-1875.
- **Catalogue at 4b7f55c:** eight entries match test.log:2195.
- **Mutations:** the ten named mutations match test.log:2275-2485; "refused: ()" 0 against 4 matches test.log:2483 and the per-mutation lines; R4's (2, 2, 2) against (1, 1, 1) matches test.log:2337-2338.
- **Negative log:** N35-N41, E1-E10, 0 added messages and the three unreached messages match the negative.log sections read above.
- **Run log:** summary 27/27/27/27, entry totals 843/27/0/17, totals 81/27/0/27/2084915/2084969, the controls and cross-target agreement all match run.log:1342-1344, 1414 and 1541-1546.
- **Record pair:** 14969 and 198677 bytes and both sha256 values match run.log:1562-1563 and my own hashes.
- **Log hashes:** every log sha256 in the report matches scan.log and my own hashes.
- **Planning file:** 740db7aa... matches every log and my own hash of spec/0011.
- **P2-F1 wording:** matches spec/0011:612-630.

### Findings

1. **Defect.** report-draft.md:37, "26 .xlsx files", appears in no primary; the corpus is 27 workbooks (run.log:1306-1307).
2. **Defect.** report-draft.md:36, "Every log ... accounts for its one status line", is false for scan.log, which has no status check.
3. **Risk.** 4b7f55c's message states the counts (1, 1, 1) and (0, 0, 0). No log prints (0, 0, 0), and (1, 1, 1) appears only as R4's expected value (test.log:2338). The message does not say that no log prints them, which is what specs/0001 requires.
4. **Risk.** The claim that the unknown-formula test shows "only the edited cell's formula is read" (commit message and report-draft.md:13-14) is an implicit bite claim that no mutation measures.
5. **Risk.** The ruling-4 count reads `drop_calc_chain_once` called directly, not write_to's call. write_to's use of it is evidenced only indirectly, by R4 breaking the control (test.log:2339-2340).
6. **Risk.** 20 direct `expect_err("refused")` sites remain in the package tests (test.log:2515). Ruling 8's four are fixed; the rest would still print "refused: ()" on failure. The report does not mention them.
7. **Risk.** No corpus workbook has an unstyled B (b_styled=1 and dropped=0 in all 27 on both targets). The `dropped`/`removed` arm of the corrected clause is proven only on generated input.
8. **Risk.** `--edit-sample` leaves the output file in place when it refuses on `CellsAfter` (diff-595cf8b.diff:210-245), unlike the tool's other refusals.
9. **Note.** `NoWorksheet` and `NoNewRow` are not in the spec. They are justified as defensive and are tested (E8, E9, and the integration test).
10. **Note.** The output reopen maps a failure to `Error::ReadInput` (diff-595cf8b.diff:217), so the message would name the wrong file.
11. **Note.** The run script accepts `descriptor` on the changed worksheet entry (run.log:708). P2's acceptance text is silent on it, and the report does not cite `specs/0008` for it.
12. **Note.** "the tool's two reachable messages" (report-draft.md:59) and "bytes compared by hand" for all four tests (report-draft.md:26) are characterizations that go beyond the primaries.
13. **Note.** "the hashes planning gave" (report-draft.md:4) cannot be verified from the check directory.
14. **Note.** There is a cosmetic reflow in the catalogue's "a row number is not a number" entry (diff-4b7f55c.diff:48-54), and a literal home-shaped fixture path in run.log:819.
