# Independent check, M1 part 4 P2 (check 2)

I read only the files in the check directory. Paths below are relative to it, and line numbers start at 1. I ran no git command. For run.log I give the file's own line numbers.

## Question 1: does the diff do what the spec says and nothing else?

**How the diffs relate.** I compared the combined diff with the per-commit diffs mechanically:
- `diff-combined-57d2334..595cf8b.diff:14-285` is byte-identical to `diff-4b7f55c.diff:32-303`.
- `diff-combined-57d2334..595cf8b.diff:599-1052` is byte-identical to `diff-595cf8b.diff:28-481`.
- The rest of the combined diff is ROADMAP.md (1-13), specs/0001 (286-303) and specs/0011 (304-598). That matches `doc-commit-4cf3671.stat` (3 files).
- The committed specs/0011 hunk contains neither "Planning rulings during P2" nor "P2-F1" (a grep of the combined diff finds no match). The doc commit therefore did not pick up planning's later, uncommitted edit.

**4b7f55c, judged on `diff-4b7f55c.diff` alone, against spec/0011-editing-cells.md:427-431 and rulings 1, 4 and 8 (spec/0011:560-599):**
- **Ruling 1: done.**
  - write_to's doc names the three `cells` messages (diff-4b7f55c.diff:74-78).
  - There is one write-path package test each: lines 265-274, 276-282 and 284-301.
  - The module catalogue names each test (39-64). The spec did not ask for that, but it is harmless.
  - One reflow in the "a row number is not a number" entry (48-54) changes no wording.
- **Ruling 4: done, by an extraction.**
  - The condition moves unchanged into `Package::drop_calc_chain_once` (86-95, 102-115). `removed_formula && !pending && drop` becomes `if removed_formula { !pending && drop }`, which short-circuits the same way.
  - The already-dropped test counts (1,1,1) then (1,1,1) (221-243). The new control counts (0,0,0) then (1,1,1) (245-263).
  - Both tests call `drop_calc_chain_once` directly, so the count is of the function, not of write_to's own call. That write_to calls it is shown only indirectly: under R4, the control panics inside `written(package)` with `Unsupported("two changes for one entry")` (test.log:2405). The panic at tests.rs:2995:24 maps to diff-4b7f55c.diff:262, the `written(package)` call.
- **Ruling 8: done.**
  - `refused` and `refused_edit` go through `refusal`, and a write that is not refused panics with what it wrote (125-185). The M2 test is rewritten the same way (188-203).
  - The spec says "printing the error they refused on" (spec/0011:430-431). When nothing was refused there is no error to print, so printing the output is the only faithful reading.
- **Nothing else** is in diff-4b7f55c.diff.
- **One wording problem.** The commit message (diff-4b7f55c.diff:13-14) says the unknown-formula test shows "only the edited cell's formula is read". The code and R1d show something different:
  - R1d's site, `if let Ok(FormulaTag::Array(area)) = tag {` (test.log applier; failure at test.log:2384), shows that every cell's formula tag IS read, for the array-extent check. Only the refusal is limited to the edited cell.
  - The source doc says it correctly: "on the write path, for an edited cell only" (diff-4b7f55c.diff:63).
  - So the message's phrase overstates what the code does. The report repeats it (report-draft.md:14-15).

**595cf8b, judged on `diff-595cf8b.diff` alone, against spec/0011:295-316 and 432-435:**
- **A, B and M.** A and B are the first and second `Number`/`SharedString` cells with a value and no formula, and B records whether it has `s` (152-159). M is the maximum row over the cells, 0 when there are none (145, 151).
- **The edits.** 42.5, a clear, `Text("emend")` at (M+1,1) and `Boolean(true)` at (M+1,2) (194-201).
- **The write-back.** The package is written, reopened and read through `Package::cells`. `read_back` counts `number()` 42.5, an InlineString "emend" and `boolean()` true (217-234).
- **The counts.** `set` is 2 or 3, `cleared` 0 or 1, and `dropped` 1 only for an unstyled B. `cells_after` is asserted equal to `cells_before + 2 - dropped`, with exit 2 otherwise (236-245).
- **The line.** `edit` then tab-separated key=value, counts only (246-264).
- **`rows_added` is the constant 1** (250). The spec writes "`rows_added` 1", so this matches the letter, but the tool measures nothing here (see Question 2).
- **Additions the spec does not name.** `NoWorksheet` and `NoNewRow` (87-88, 182, 184) are defensive and tested (456-480; negative.log E8/E9).
- **Small defects already carried as F-4 and F-5:**
  - The `CellsAfter` refusal comes after the rename (210-215, 240-245), so a refused run leaves an output file.
  - A failure to reopen the output maps to `Error::ReadInput` (217).
- **Tests.**
  - The four integration tests are on `cells_specs` with the sheet stored (292-308, 378-480). Three compare the sheet's bytes with hand-built bytes; the no-worksheet test compares none.
  - The usage test gains three shapes (278-281).
- **Nothing else** is in diff-595cf8b.diff.

## Question 2: do the logs contain every line the spec requires?

**P2 acceptance** (spec/0011:329-352 as corrected by 614-630):

| Requirement | Evidence | Verdict |
|---|---|---|
| First worksheet part `changed`, the only changed entry; every other entry identical and unflagged | run.log:708 logic; 1413, 1416-1417; 1529, 1532-1533 | present |
| order_preserved yes, files_identical no; lo and lo_in ok | run.log:1342-1343, 1458-1459 | present |
| expat prefix and suffix bytes identical | cmp.py run.log:1206-1214; accept_one 567-568; per workbook 1360-1412, 1476-1528 | present |
| changed = set - 2 + cleared - dropped; removed = dropped; added 2; sum = cells_before | run.log:569-572 (plus cells_in/cells_out tied at 573-574, and a/b tied to set/cleared at 575-576) | present |
| Reference-level checks from the comparison's own pick of A, B, M | cmp.py run.log:1231-1243; asserted at 577-580 | present, with the caveats below |
| read_back = set | run.log:564 | present |
| Totals: rows_added 27, cells_after = sum of cells_before + 2 - dropped | run.log:1414, 1418-1419; 1530, 1534-1535 | present |
| Planted-difference control | generated pairs run.log:1270-1289; real pair 1420-1423, 1536-1539 | present |
| Both targets | host 1309-1423, i686 1425-1539, agreement 1541-1546 | present |

Caveats on the reference-level row:
- **"Keeps its `s` bytes" is checked as the parsed attribute value, not bytes.** The check is `cout[b][1] == cin[b][1]` (run.log, cmp.py `b_ok`).
- **`new_cells` has no negative control.** The five planted faults on generated pairs flip `refs_as_edited`, `prefix_same`, `suffix_same`, `b_kept_s_no_value` and `a_value` (run.log:1276-1285). None flips `new_cells`, and the real-pair plant leaves `new_cells=yes` (run.log:1421).
- **The controls' printed labels repeat one field.** Three of them name the same field twice, for example "(prefix_same=no, prefix_same=no)" (run.log:1279, 1281, 1285; the loop at run.log:530-532).
- **The rows_added total is not a measurement.** It sums a constant the tool prints (diff-595cf8b.diff:250). The comparison ignores `<row>` elements, so no line shows that a new row element was written, only that the new cells are at (M+1,1) and (M+1,2).
- **The unstyled-B arm is never exercised by the corpus.** All 54 comparison lines read `b_styled=1` with dropped 0. That arm is shown only by run.log:1273-1275, the integration test and negative.log E1.

**The specs/0001 rules** (spec/0001-bootstrap.md:95-245):
- **Phase commit given to the script and compared with HEAD, with a control:**
  - gate.log:1-8 (null and short-sha controls);
  - test.log:1-2 and 11-13 (null control only);
  - negative.log:1-2 and 8-10 (null control only);
  - run.log:1-9;
  - scan.log:1-7.
- **Every sha256 printed by a command.**
  - Planning's file: gate.log:11-12, 1024; test.log:9-10, 2602; negative.log:4-5, 2886; run.log:11-12, 1578; scan.log:1176.
  - The binaries: gate.log:1016-1019, re-checked at run.log:1296-1304.
  - The record pair: run.log:1562-1563.
  - The logs: scan.log sibling block, under "command: wc -c and sha256sum".
  - Attempt 1: test.log:5-6.
  - I recomputed every log, record and spec hash in the directory, and all match.
- **No absolute paths.** My grep found only fixture strings built by concatenation or written as controls (run.log:819; scan.log:326-327, 358; gate.log:444). independent-check-1.md:85 quotes `/home/someone/...`, which is a fixture, not a machine path.
- **Counts a commit message states, printed by a log:**
  - 4b7f55c's "three" messages: test.log:2242-2249.
  - 4b7f55c's "(1, 1, 1) both times" and "(0, 0, 0) before the rule": test.log:2579-2588, read from the test body at 4b7f55c, with both tests passing on both targets and a no-tuple control.
  - 595cf8b's "three shapes": test.log:2596-2597.
- **Tests a message names exist at that commit:** test.log:2567-2595, with a control at 595cf8b only.
- **Bite claims measured.** There are eleven mutation blocks (test.log:2323-2547), each with the planted change, the run and the assertion's output, and a clean-tree control (2550-2560). R1d measures the unknown-formula test's bite, failing at tests.rs:3028:15 (the `edited_sheet` B1 call, diff-4b7f55c.diff:296) with the message at test.log:2384.
- **Superseded attempts hashed.** test.log:4-6 names attempt 1 and hashes it by command: `45d74fa1...`, 164132 bytes. The log prints only the basename, so nothing in it shows where attempt 1 is kept.

**test.log attempt 2 against check-1 findings 3 and 4.**
- Finding 3 (message counts no log printed) is addressed by test.log:2579-2588. The tuples are read from the source, not printed at run time. Together with the pass lines that is adequate. tests.rs is untouched by 595cf8b, so the source at 4b7f55c is the source that ran.
- Finding 4 (no mutation for the unedited-cell claim) is addressed by R1d (test.log:2375-2390).
- The header hashes attempt 1 (test.log:4-6), and the script's header change 10 explains the rerun (test.log:~50-58).
- Consequence: check 1's citations into test.log (for example `test.log:2515`, `2339-2340`) point at attempt 1, which is not committed. In attempt 2 those lines are 2590 and 2405.

**scan.log** has no working-tree status check, which the report acknowledges. It includes independent-check-1.md but not this answer, so it must be rerun. The current scan.log (68918 bytes, sha256 59408b22bef297348de03d287a9b5f10f48f419be319753c9499414cbf875761 by my computation, which is not a primary) then becomes a superseded attempt to hash.

## Question 3: does every number and quoted string in report-draft.md appear in a primary?

**Confirmed:**
- **Commits, lines 3-7:** gate.log:776-786.
- **Ruling 1, lines 10-16:** diff-4b7f55c.diff:39-78, 265-301.
- **Chain tuples, lines 18-21:** diff 221-263 and test.log:2579-2588.
- **Integration and usage tests, lines 27-30:** diff-595cf8b.diff:278-281, 378-480.
- **P2-F1, lines 33-38:** spec/0011:610-630; the hash is in every log.
- **Status accounting, lines 39-40:** gate.log:1022, test.log:2600, negative.log:2884, run.log:1576. scan.log has no status line.
- **Gate, lines 46-49:** hash in scan.log; gate.log:787-789, 812, 815-816, 990-1004.
- **Tests, line 50 onward:**
  - hash, 45d74fa1..., 164132: test.log:4-6;
  - 390/382/8 (4, 4): test.log:1443, 1879, 1911-1921, 2563;
  - eight entries: test.log:2241;
  - eleven mutations and R1d's message: test.log:2323-2560, 2384;
  - the tuples: test.log:2579-2588;
  - "refused: ()" 0 against 4: per-block lines and 2547;
  - R4's (2, 2, 2)/(1, 1, 1): test.log:2398-2401.
- **Negative, lines 66-77:** hash in scan.log; N35-N41 at 2265-2307; E1/E2 lines equal the integration tests' asserted lines (negative.log E1/E2; diff-595cf8b.diff:397-398, 445-446); E3-E10 at 2327-2387; 0 messages added at 1979; the three unreached messages at 2800-2802.
- **Run, lines 78-98:** hash in scan.log; run dir at run.log:1294; 27/27/27/27 at 1342; 843/27/0/17 at 1344; descriptor 17 at m1p4-p2-...md:25, 54; clauses at 1413; totals 81/27/0/27/2084915/2084969 at 1414; controls at 1270-1289 and 1420-1423; agreement at 1541-1546.
- **Check-1 carry-overs, lines 110-123:** F-1 at test.log:2405; F-2 20/21 at test.log:2590; F-3 in the run.log per-workbook lines; F-4/F-5 at diff-595cf8b.diff:210-245, 217; F-6 at diff-4b7f55c.diff:48-54.
- **Record pair, lines 126-127:** 14969/198677 and both hashes at run.log:1562-1563 and scan.log.

**Not found, or disagreeing:**
1. **report-draft.md:14-15** and **107-108**, "so only the edited cell's formula is read". R1d (test.log applier; failure at 2384) shows every cell's formula tag is read. Only the refusal is limited to the edited cell. This disagrees with the primary; the code doc's wording (diff-4b7f55c.diff:63) is the accurate one.
2. **report-draft.md:50-51**, "kept outside the repository": not in any primary. test.log:6 prints the basename only.
3. **report-draft.md:82-83**, "as specs/0008 allows for a replaced entry, and as P1's run allowed": neither specs/0008 nor P1's run.log is in the directory. spec/0011:357-358 cites specs/0008 for `descriptor` only under P3, and spec/0011:568-571 (ruling 3) cites it for `identical`/`recompressed`, not `descriptor`.
4. **report-draft.md:41-42**, "the corpus's .xlsx files": spec/0011:628-629 calls it "the uncommitted smoke run over the debug binary". No primary says which files or which extension.
5. **report-draft.md:70-71**, "needs a library that writes the wrong cell count; it is reached only under mutation T1": T1 mutates the TOOL's `dropped` computation (test.log T1 applier: `Some((_, false))` becomes `Some((_, true))`), not the library. The second half is supported (test.log:2413-2436); the first half is a characterization that T1 does not show.
6. **report-draft.md:4**, "unedited": the report itself says the comparison is not in a log. Accepted as stated.

## Findings

1. **Defect.** The commit message of 4b7f55c (diff-4b7f55c.diff:13-14) and report-draft.md:14-15 and 107-108 say "only the edited cell's formula is read". R1d's own mutation site (test.log, applier; 2384) shows every cell's formula tag is read, and only the edited cell's unknown type refuses. The report should use the code doc's wording (diff-4b7f55c.diff:63). The message is committed and stays as it is, so the report should state the correction.
2. **Risk.** The comparison's `new_cells` flag has no planted fault that turns it to `no` (run.log:1270-1289, 1421). The five controls cover the other flags only.
3. **Risk.** `rows_added` is a constant in the tool (diff-595cf8b.diff:250), and the comparison ignores `<row>` elements. The "rows_added 27" total (run.log:1414, 1530) therefore restates the constant rather than measuring that a row was added.
4. **Risk.** The removed/dropped arm of the corrected clause is never exercised by the corpus (b_styled=1 and dropped=0 in all 54 lines). It is shown only on generated input. This is F-3; I agree with it.
5. **Risk.** The ruling-4 counts read `drop_calc_chain_once` called directly. write_to's use of it is shown only by R4 (test.log:2405). This is F-1; I agree with it.
6. **Risk.** scan.log does not yet include this answer and must be rerun. The present scan.log must then be hashed and kept as a superseded attempt.
7. **Note.** "kept outside the repository" (report-draft.md:50-51), "as specs/0008 allows ... and as P1's run allowed" (82-83), ".xlsx files" (41-42) and "a library that writes the wrong cell count" (70-71) are not supported by the primaries in the directory. The last one misdescribes T1, which mutates the tool.
8. **Note.** `b_kept_s_no_value` compares the parsed `s` value, not its bytes, where the spec says "keeps its `s` bytes" (spec/0011:345-346).
9. **Note.** Three planted-fault labels print one field twice, for example "(prefix_same=no, prefix_same=no)" (run.log:1279, 1281, 1285). This is cosmetic.
10. **Note.** Check 1's committed answer cites line numbers in test.log attempt 1, which is not committed. In attempt 2, 2515 is 2590 and 2339-2340 is 2404-2405.
11. **Note.** test.log and negative.log control the phase-commit comparison with the null commit only. gate, run and scan logs also refuse a short sha.
12. **Note.** Both code commits do what the spec assigns them and nothing else: 4b7f55c judged on diff-4b7f55c.diff, 595cf8b on diff-595cf8b.diff. F-4 and F-5 (the output left on a `CellsAfter` refusal, and `ReadInput` for the output reopen) stand as carried.
