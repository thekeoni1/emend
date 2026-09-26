# Independent check 1, M2 part 3 P2

Written by the implementing session: the checked phase commit is
a4b55f09e740fc701f825128587244cb725ba923, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
<scratch>/p2/check1 (<inputs> in the answer), holding these files, by sha256:

    843a6b595babf953a07f449482e6e5d3ae9c4ab8b4da78aedcbda62b34c46b9a  diff-7578ee0.patch
    09d1a9d0d2008bc4f03ab276e45ee28b89ec32ad9b2df0483699dcce8aee07f6  diff-94cf3b6.patch
    2a0313004b2acfb54c9c2d0dbd7364b0382d02a38b8f806b07bb106c0504135c  diff-a4b55f0.patch
    7b8ce14f65a80c4429dfb1595a1144cc56462045d91a26f23f3ec7449e7d69c5  diff-b2fa1e5.patch
    7b55c7bf7809f2c5b4342c36295f101aa2d6677c42207518c7b1d61fa33ff5d7  diff-commands.txt
    0ea751df3d4366cfbd0b473dc23e20220f3f9ee41cda9730adb5f01be5d6ee33  diff-phase.patch
    6f4ae04755aa892da2bd39fbe02cff40068227e8ea887f06619625c1fe2041e7  log-gate.log
    99a52a4392615a77476cce5c58c466277887be452c6fe1db23dceed875eaded5  log-negative.log
    e707f319d9675d90da2698118c4783332575bdc3ba6452db245cdd44c673cc70  log-run.log
    b0bb0e2c80b13eff442dcabbe941512bf9d30c83551cef94392406d6b3ea07f4  log-scan-pre-check.log
    e32d2cf0687b5fd62a108d8ef12ab40befce61be0a2e2922f565ef4ee0d43d8a  log-size.log
    d784e61969e37d8b8cca35c12fe47778b5b799d9390bd6b24fbbe5f90cf8ca07  log-test.log
    9e2b672cec71c3c58809bbd23bef378b3e4962c6f35f763dd441c16b5fe02de7  record-m2p3-open-2026-09-26-a4b55f0.md
    b088002282d7d147cc6d333ae16a8b96ed6092b08021c9dbb56b2ee9df19033d  record-m2p3-open-2026-09-26-a4b55f0.txt
    487b875359dd45dc0cd8878b3a70bcc217daaeeecfd07cb4a3b0378d1da3b41b  record-m2p3-p2-delete-2026-09-26-a4b55f0.md
    4addf0a74d93e1325d7835f16552657487e5db0dc860f811aa93589af341cefb  record-m2p3-p2-delete-2026-09-26-a4b55f0.txt
    05f0d16a5ed7d329bff36f34e7ae74d7d3bba16aca82eb75ded9e37e0627e4f5  record-m2p3-p2-insert-2026-09-26-a4b55f0.md
    badbf8c2675729d5a7763033ab6ac3187f788a835c28371a1adb5794ca5d06d0  record-m2p3-p2-insert-2026-09-26-a4b55f0.txt
    66989d40e3d0abac0467bbf3f679bc82d0746ca88c6000e4dda74fc901aef213  report-draft.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    7e600cc743f5849aa1c9616241034e19691273af0f36588aafd368fc15fa9864  specs-0018-rows-and-columns.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-gate.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-negative.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-run.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-pre-check.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-size.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-test.txt
    78dac57b6e74dcd0d6c64e33b0e0af7ebc5342210a4dac6417a3fbb870e40fe3  superseded-m2p3-p2-run-attempt-2.sh
    c4d3a7f0dbd0be9ebd333b486f4d68026e4a2723d2ecca3736b6746f63bff734  superseded-run-attempt-1.log
    059e165b6288564c829d664757e1dbc0a4f0c9cf809e09ddc204e8117ba2f021  superseded-run-attempt-1.stderr
    d3540afb2cb9803eb25070d01e25481b1200eafe16e837694f21e53b893c3d92  superseded-run-attempt-2.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded-run-attempt-2.stderr

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 5973f368c97d346c628370758515fa041094e8392b334548eaeca5ca441e8d79, 8478 bytes).

---

Disclosure: my harness gave me the project's CLAUDE.md, a memory index, git status and an attribution reminder. I did not use any of it as evidence. Every statement below comes from files in <inputs>.

**Q1. Does the diff do what P2 says, and nothing else? No, narrowly.** The code in diff-94cf3b6.patch, diff-7578ee0.patch and diff-a4b55f0.patch does what P2 and rulings 1 to 14 require, and I found nothing unruled added (diff-phase.patch holds the same nine files). The "no" is for four of P2's listed test cases that are missing or partial, and one spec sentence on arithmetic that the code does not follow.

**Q2. Do the logs contain every required line? Yes.** Every line I checked for is present, with the remarks in F11 and F12.

**Q3. Does every number and quoted string in report-draft.md appear in a primary? No.** Four spec-hash attributions are not in the logs, and one claim rests on a log that is not in <inputs> (F13 to F15).

**Findings**

F1 (Q1, confirmed). diff-94cf3b6.patch carries only P1 verification rulings 4 to 6:
- F6: `saturating_mul` in `Area::cells`, with a test.
- F7: the indices test now writes the package and asserts the output is the input.
- F5: a test that removing a pending sheet drops its format, pane and merge edits.
- Ruling 4: the header sentence on root children held whole.
- diff-7578ee0.patch lines 72-79 extend that sentence to `dataValidations`, as ruling 8 asks.

F2 (Q1, confirmed). The shift rule, in diff-7578ee0.patch lines 353-392, matches "The shift rule" case by case. The things it is applied to all match the spec and rulings:
- Rows and cells are retagged, and a deleted row is dropped whole.
- `f ref`: an interval wholly inside a deletion is kept raw (ruling 1). A grow or cut is refused. A move whole past the grid is off-grid; an end past the grid is a cut (ruling 5).
- `dimension` becomes A1 when emptied.
- `mergeCell`: dropped when removed or reduced to one cell, with `count` following.
- The four lists (ruling 4 form): `hyperlinks` is streamed and `mergeCells`/`dataValidations` are held (ruling 8). An unparseable range gets the fourteenth message (ruling 2).
- The table and pivot refusal reads relationships before the write (lines 266-298). An unparseable or missing ref gets the table message.
- The one-shift rule is at lines 236-262.
- The formula-change rule is ruling 7's form.
- The five messages are at lines 304-315, catalogued at lines 25-54.
- The hyperlink relationship sentence answers ruling 3.

F3 (Q1, confirmed). The tool part of diff-a4b55f0.patch matches "Harness runs": the k formula, the two lines' fields and the assertions. The refactor into `sample_run` is ruling 10's.

F4 (Q1, missing test). P2 says "an array likewise", meaning the five shared cases. `an_array_formula_likewise_and_one_whose_ref_does_not_parse` (diff-7578ee0.patch line 1182) covers only moved, straddling-insert refused and deleted-whole. It has no array ref before the point and no deletion inside an array ref.

F5 (Q1, partial test). P2 says "each list attribute moved, cut, a range dropped from the list, the element dropped when the list empties". The test at diff-7578ee0.patch line 1313 does not cover each attribute for each case:
- Only `autoFilter` is cut. No `conditionalFormatting` or `dataValidation` range is cut.
- `autoFilter` is never dropped when emptied.
- Hyperlink ref, autoFilter ref and dataValidation sqref are never tested unparsed. Only CF is tested unparsed, though the code path is shared.

F6 (Q1, departure from the test list). P2 asks for "a second insertion for the part ... refused with the one message". The test at diff-7578ee0.patch line 1419 records `insert_rows` then `delete_rows(SHEET, 9, 1)`. A second insertion is not tested; the code path (`shifts.len() == 1`) is the same.

F7 (Q1, partial test). The chain test at diff-7578ee0.patch lines 1233-1257 asserts the chain gone on the deletion (line 1243) but not the `fullCalcOnLoad` flag. The flag is asserted only for the two insertions.

F8 (Q1, header claim without a test). The catalogue at diff-7578ee0.patch lines 47-49 says the table message is also raised for a ref "that is absent" and for "a part that is not there". The cited test only covers an unparseable table ref. No test has an absent ref, a missing target part, or an unparseable pivot `location ref`.

F9 (Q1, departure). "32-bit and streaming" (specs-0018-rows-and-columns.md line 433) says the arithmetic "is checked (`checked_add`, `checked_sub`)". `Shift::index`, `Shift::interval` and `shift` (diff-7578ee0.patch lines 175, 353-392) widen to u64 and use plain `+` and `-`. Those operations cannot overflow at these bounds, but the code does not use the form the spec names.

F10 (Q1, unspecified behaviour, for planning). `area_clipped` (diff-7578ee0.patch line 400) returns None for a range moved wholly past the grid. As a result:
- Such a merge or list range is dropped silently.
- Such a `dimension` becomes A1 (line 680).
The spec rules only on clipping a range that extends past the grid, not on one that lands wholly past it. The same diff also adds two public consts, `TABLE` and `PIVOT_TABLE`. The shift rule asks for them, but the "Crate" sentence on the public surface does not list them, and the gate's `pub fn` listing does not see them (log-gate.log lines 1473-1487).

F11 (Q2, confirmed).
- **Each log:**
  - gate: nine PASS, the gate-file listing, the base-test edits, the spec hash (lines 896-900).
  - test: both targets at 685, the shift table at lines 2398-2426 (20 lines: 18 rows, 2 clip calls), 20 tests added.
  - negative: S0 to S5 as ruling 9 gives, plus two usage shapes (lines 682-756).
  - size: the three commits and every ruling 12 figure (lines 555-566).
  - run: both samples on both targets, 8 flags and 8 of 8 plants per sample, refusal set 0, the ruling 13 recompressed count, keep 297 = 266 + 31, open check.
- **Carried rules:**
  - Each log counts its stderr at 0 bytes with the one-byte control. All six stderr-*.txt files are 0 bytes.
  - The scan pre-check (lines 1580-1601) confirms placeholders are declared by root lines.
  - The shape greps run in every log and in the scan (lines 1548-1578).
  - The rerun header in log-run.log line 27 names attempts 1 and 2 and hashes 11 files. Every hash I recomputed on the superseded-* and record-* files in <inputs> matches.
  - The moved keep is listed by its hash (line 54).

F12 (Q2, remarks).
- (a) log-run.log line 54 prints "pairs 0". Ruling 14 accepts this and no check reads the count. The script line with the defect is log-run.log line 297 (not quoted here).
- (b) Attempt 2's two run directories are "hashed" only through their two summary files and a file count (log-run.log lines 40-51), not a whole-directory listing. Ruling 13 says "hashed" without a form.
- (c) diff-7578ee0.patch line 16, a commit message, says "Five messages". No log prints that count. specs-0001 requires a count in a commit message to be printed by a log, or the message must say none prints it. The count is visible in the diff.

F13 (Q3, not found). report-draft.md lines 5-6 say test and negative pin 4a7726f4. Neither log-test.log nor log-negative.log prints any hash of the spec: zero matches for 4a7726f4, and no sha256 line for specs/0018.

F14 (Q3, contradicted). report-draft.md line 6 says size pins 4a7726f4. log-size.log line 578 prints bf49e9c7. The size log ran at 16:35:00Z (line 28), after the spec had changed.

F15 (Q3, not in a log). report-draft.md line 7 says the scan pins 7e600cc7, and lines 25-26 and 54 describe the final scan stage as attempt 1. The final stage is not in <inputs>, and log-scan-pre-check.log lines 1505 and 1517 pin bf49e9c7. The spec copy in <inputs> does hash to 7e600cc7, but no log in <inputs> prints that hash.

F16 (Q3, confirmed). Every other number and quoted string in report-draft.md is in a primary:
- **Commits and hashes:** 2e54f33, 8cbe6a47, 5c28004f (log-gate.log lines 1177-1194); 694d553d (log-run.log line 54).
- **Gate:** nine PASS; two pub fn lines added.
- **Test:** 685, 0, 2, 20; 20, 18 and 2.
- **Negative:** S0 at 4.
- **Run:** 27 of 27; 8 flags; 8 of 8 plants; 4 recompressed; 12 of 27 and 2; 297, 266, 31; "pairs 0"; run.log line 297 and `${TAB}`.
- **Size:** 399176, 430372, 457044, 57868, 31196, 26672, 17868, 1708, 458752.
- **Statements:** "five messages" (diff); the record names; the attempt 1 and 2 accounts (superseded-run-attempt-1.stderr and superseded-run-attempt-2.log lines 2791-3363).
