# Independent check 2, M2 part 3 P3

Written by the implementing session: the checked phase commit is
f52684c0fadc4f77485b6113f8fee03c357defcd, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
<scratch>/p3/check2, holding these files, by sha256:

    b5dc3ae3a735174fd0d6047a7c9a7ca2627b8ecca9a7deabde56da9c2abadbad  diff-b675f3d.patch
    8562f3c4a5b95048399bbe7605b301c8ab5aba0254a6a5c300c7d22c9279367c  diff-c3b5bbd.patch
    a55e7d49e0e53bb8e16f800ae0f7b7582a9b3569a45dac511c5f2752e2f06c72  diff-commands.txt
    23e7738ce0a3126174a0361be8739c454c4773bddfa13c3aa3bfa2cbbe124a26  diff-doc-ec68ccd.patch
    ff9d878e2fd3f6e29aaa333ffd2d3191754a3673d216ffc5a1ecd31b2f70aeb3  diff-f52684c.patch
    5ba58fc1c28f0f48ec752409c32f6a07584acd711b5b289d787799c694385794  diff-phase.patch
    2b9644fcd467ae5a11c39d6563178f6cff707306ff2ea9ecb2306de88f4a53f3  independent-check-1.md
    b0daa22083a1760b7425ba6ab70af81c492b17b49c2b30f56cb1f3d62a6e598f  log-gate.log
    14aefcd7173b9a0a0ac2a00ae33a79064fa5c3eefc06ffba147316594a657952  log-negative.log
    bb2547aa65246344f6b2ec2d210c1e05f9da65ad290582184f31d33fdc997328  log-run.log
    e2584443ab4f3b0523c088391ea5077cfcf4299855bec40c779577b430e4bcbb  log-scan-final-for-check2.log
    6b0d6ea68002990f3b10de7d0a2ad5578c04de78517de5ced43f59b3c43d57f6  log-size.log
    d797a135fe21a9c5013ce6427a3a2089bb431bde2f0e3b79133fb11ee68dfa82  log-test.log
    85d32022f216baf7a47c6f063519ddec81b36a62dd5f60fe432c1dd3957a33aa  m2p3-open-2026-09-26-f52684c.md
    c069b012421321e491a5534319594cb633854afa725d435cc1c925e919c625bb  m2p3-open-2026-09-26-f52684c.txt
    ba385c6df546083e5275b0bcc69a3e0c881775acb41e7dc1101f16b3166e0b52  m2p3-p3-delete-2026-09-26-f52684c.md
    82e43e31d88f7609c184699a6dd9c777f364ed9a1fc06af6aa5e38a7d2d66d51  m2p3-p3-delete-2026-09-26-f52684c.txt
    942fe8622010c9e787b8dda72df41d3d9468dacf54de27571672551d9ccc6d2a  m2p3-p3-insert-2026-09-26-f52684c.md
    b68dba95b3c3592691c9b82ca6665431321984fb1da47a993558a5d826442603  m2p3-p3-insert-2026-09-26-f52684c.txt
    3bf1c6d69fd91d55dcb40bdededb1e1864fe55d6fb444d9fb6457cf34d1fd300  report-draft-2.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    59c50efe872074dac10f239a19a9e45740393a467fa8a6e2c0615de01d3ff914  specs-0018-rows-and-columns.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-gate.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-negative.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-run.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-final.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-size.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-test.txt
    5e5a6b83fbad52a2d3b1e7b5ae11f61a6d5531d09fd54c12b88d967fee6f04da  superseded-m2p3-p3-test-attempt-1.sh
    034519d3f0b3f22a88d4c948fe64c27e1a3d83a9ef26ea91a89e0909a5a15a8d  superseded-test-attempt-1.log

This is the phase's second and last check. The answer below the rule is
the checker's final message, verbatim, taken from its transcript
(sha256 68f0591ca99fd155934650a4d5ec80769e1b2ae895c1f15075c32eab051451e1, 11027 bytes).

---

The harness put a CLAUDE.md project instructions file into my context; I did not use it as evidence.

I read only the files in `<scratch>/p3/check2`. All inputs except `report-draft-2.md` and `log-scan-final-for-check2.log` have the same sha256 that check 1's header lists. Line numbers below are those of the named file.

**Short answer:**
- **Q1:** Yes. The diff does what P3 says. The exceptions are the catalogue extensions ahead of P4 and one test gap (G6).
- **Q2:** Yes, with two gaps. The carried moved-keep and moved-run-directory checks exist only in branches that never ran (G7). The doc commit's message is not checked for counts (G8). One more gap: two plants print no assertion output (G9).
- **Q3:** One quoted string has no primary (G10). Three unlogged claims are not backed by any input (G11 to G13).

## Question 1: does the diff do what the spec says for P3, and nothing else?

**G1 (remark), `diff-phase.patch` against the four per-commit diffs.** The blob indices chain: `c276d87..859d90f`, `110b834..af1aa60..569350d`, `d38b811..0f5aecc`, `653db5c..6bd55f6..6b22af8`, `f5bfd4b..a740c25`, `0c3bd44..bc69c9a`, `64c5be0..7f3f687`, `308c2bc..cdf2dcb` and `e716496..4050c0c`. That is 9 files, and `log-gate.log` 1430-1440 lists the same 9. No other change is in the range.

**G2 (remark), `diff-doc-ec68ccd.patch`.** "Nothing else" confirmed.
- It touches only `ROADMAP.md` and `specs/0018`.
- `log-gate.log` 1389-1413 matches both blobs to planning's hashes (`cd524d29...`, `2efb06e1...`), with controls.
- The hunks are decision 7 A: "Crate", the clip sentence, "32-bit", "Size", P3, P4, decision 7, and the P2 rulings and verification.

**G3 (remark), `diff-b675f3d.patch`.** "Nothing else" confirmed against spec 700-710, with one addition (G4).
- `rowcol = []` (lines 9-13) and the tool's `features = ["dates", "rowcol"]` (line 558).
- The cfg attribute on `TABLE`, `PIVOT_TABLE`, `insert_rows` and `delete_rows` (lines 86, 95, 154, 162).
- The five message constants (301-314), and the shift pass and its helpers (321-418).
- `mod shift` (429).
- `shift_alone` (263-285) is the removed block at 230-249, textually the same.
- The ruling 13 sentence (73-76).
- Six appended tests (443-545), which match P2 verification rulings 1 (F4-F8) and 3 (F10).
- The `no_shifts` helper and the cfg'd `named` chain and `retain` (103-138, 178) are the gating's own plumbing.

**G4 (remark), `diff-b675f3d.patch` 21-56.** This commit also adds test names to the package header's error catalogue. Check 1's F4 flagged catalogue work ahead of P4 only in c3b5bbd, and report disclosure 13 (lines 135-137) answers only for c3b5bbd.
- The sentence marking these as the feature's messages (lines 23-25) is required by "Crate" (spec 175-177).
- The test-name additions are, like F4, ahead of P4's "catalogue and limits".

**G5 (remark), `diff-c3b5bbd.patch`.** "Nothing else" confirmed apart from F4 and G4.
- `AxisShift` (228-307) follows P3 ruling 5.
- `spans_tag` (314-346) follows ruling 2.
- `col_tag` (354-361) follows ruling 1: it checks `min`/`max` only.
- The cell path (496-519), the `<cols>` stream through `LinksBuffer` (531-566), `shifted_formula` (591-612) and `tables_before` (217) apply the rule on the column axis.
- The dimension, merge and list call sites are not in any hunk. Their type changed from `Shift` to `AxisShift` (99, 199, 208, 371, 380, 412, 421), so they now run through `AxisShift::area_clipped`. Plant S7 shows this at `log-test.log` 2775-2779.
- The spec's P3 test list is covered by the ten added tests (640-901), except G6.

**G6 (gap), `diff-c3b5bbd.patch` 697-749 against `specs-0018-rows-and-columns.md` 716.** Check 1's F6 stands.
- No `<col>` is dropped while other `<col>` elements survive.
- `delete_cols(4, 3)` (718-723) only cuts.
- No column-axis test covers `autoFilter ref`.
- The report acknowledges this in disclosure 9 and defers it to P4.

**G7a (remark), `diff-f52684c.patch`.** "Nothing else" confirmed.
- The two flags (48-57) and the usage string (29).
- `shift_sample`, `axis_counts`, `shift_read_back` and `table_last` (124-335).
- The column lines carry exactly P3 ruling 3's fields (158-190).
- The row lines keep `from_k2_after` and count 2 (142, 200).
- Three appended tests (361-415).
- Four lines were added inside the base's usage test (347-350). `log-gate.log` 1885-1890 records them as the allowed four.

## Question 2: do the logs contain every line the spec requires?

**Present:**
- **gate:**
  - Feature off and on, both targets: 512, 543, 512, 543 (`log-gate.log` 1940-1967).
  - Ruling 4(a) listing with the attribute lines; exactly six items carry the cfg (1734-1880).
  - Ruling 4(b) consumer crate: six error lines, each item named once, and a clean build with the feature on (1969-2017).
  - Gate files checked against the diff, no self-test needed (1466-1508).
  - The commit-message sentence in the three code commits (1383-1387).
  - Phase commit compared with HEAD, with controls (41-46).
- **test:**
  - Both targets with the feature on, shown by 31 shift tests including 8 column tests (`log-test.log` 2418-2423).
  - The shift table (2564-2591).
  - Eight plants (2682-2794).
  - The rerun header hashes attempt 1 (44-47), and those hashes match the superseded files byte for byte. The only script change is change Q (a two-line edit, plus the Q comment).
- **negative:** column forms of P2's three fixtures, each with a control, run in a worktree with the gate's binary (`log-negative.log` 657-820).
- **size:** probes at f52684c with the feature off, the `cargo tree` evidence, 430372 beside P1's 430372, and no record file (`log-size.log` 551-622).
- **run:** both samples in two run directories, the refusal set computed before the run, plants 10 of 10, labels byte-equal, both record pairs installed with sha256, the keep, and the open check (`log-run.log` 3261-4790).
- **scan final:** PASS, 0 hits, 0 structural failures (`log-scan-final-for-check2.log` 1670-1674).
- **stderr:** every `stderr-*.txt` is 0 bytes.

**G7 (gap), `log-run.log` 28 and script 270-300, against spec 734-739.** Check 1's F16 and F17 stand.
- The "pairs times two equals files" check and the moved run directory listing sit only in branches that did not run, because nothing was moved.
- Only the pair-count pattern has a control (line 28).
- The listing has no control of its own.
- The report states this correctly (disclosure 3).

**G8 (gap), `log-gate.log` 1383-1385.** The commit-message count rule is checked only for the three code commits. The doc commit ec68ccd's message (9 lines, 1363) is not checked, and no log prints it. This is check 1's F18 limit; the report does not answer it.

**G9 (gap), `log-test.log` 2755 and 2765.** Plants S5 and S6 print only the "panicked at" line, with no assertion or message line. specs/0001 asks that test.log print "the failing assertion's output". The other six plants print it.

**G9a (remark), `specs-0018-rows-and-columns.md` 1219 against `log-gate.log` 1964.** Ruling 4's unlogged 533 predates c3b5bbd. The gate's 543 equals 533 plus the ten column tests, so the figures are consistent.

**G9b (remark), `log-size.log` 591 against 603-606.** The core probe at f52684c is 963588, exactly P1's b8595ce figure (read from P2's log). This confirms the gating removed the shift's whole cost from the core.

## Question 3: does every number and quoted string in report-draft-2.md appear in a primary?

Most do:

| Report content | Where it appears |
|---|---|
| Spec sha256 | gate 2023-2025 |
| Doc blob hashes | gate 1389-1413 |
| 512, 543, the six error lines | gate |
| 704, 0, 2; 20, 18, 2; eight plants | test 1661, 2405, 2588, 2685-2793 |
| Test attempt 1 failing two checks | superseded log 2561-2562 |
| The negative cases | negative 761-802 |
| 963588, 533216, 430372, -26672, 0, 31196, 28380, 458752 | size 591-620 |
| 27 ok; 829, 0, 40, 1; 10 of 10; 11 and 1; 27 byte-identical | run 3300-3302, 3361, 3493-3497, 3702-3897 |
| 664, 332, 297, 35 | run 4167-4180 |
| 280 and 52 | run 4760-4766 |
| Record stems | run 3501, 3901, 4781 |
| "change S" | run 102 |
| "outside the tests" | gate 1735 |
| "the diff alone carries what it changes" | gate 459 |
| 557 | test 1778 |
| "(draft for check 1)" in the title | stale, see G14 |

These do not:

**G10 (defect), `report-draft-2.md` 119-123 (disclosure 10).** The quoted string "when it inserts or deletes rows" appears in no primary. No diff hunk holds that header sentence. The claim of "a wording slip in c3b5bbd" is therefore unsourced here: c3b5bbd does not touch that sentence, so at most it is an omission.

**G11 (gap), `report-draft-2.md` 132-133 (disclosure 12).** "I ran both with the feature off unlogged before b675f3d and c3b5bbd" rests on no primary and states no result. Under specs/0001 it cannot count as evidence; it should be dropped or clearly marked as not evidence.

**G12 (remark), `report-draft-2.md` 13-14 and 104-107.** Two statements cannot be checked from the inputs:
- "I disclosed both to planning by message".
- Disclosure 6's "This differs from P2's gate script". The report does say that script is not among the inputs.

**G13 (remark), `report-draft-2.md` 40-41.** The scan pre-check result (0 hits, 0 structural failures) is backed only by check 1's F25 table; the pre-check log is not in my inputs. The final run made for this check also passed with 0 and 0 (scan 1670-1674).

**G14 (remark), `report-draft-2.md` 1.** The title still reads "draft for check 1". The draft does not yet give check 1's committed path and sha256; the scan log prints `2b9644fc...`.

## Are the report's answers to check 1's findings right?

- **F21 (disclosure 1):** Right. It now claims only what spec 1180 says, and the invented commit id and quote are gone.
- **F22 (disclosure 5):** Right. The count is no longer quoted, and the three tests are backed by `diff-f52684c.patch` 361, 379 and 404.
- **F23, F16, F17 (disclosure 3):** Right. Only the pair count has a control, at line 28.
- **F24 (lines 25-28 and disclosure 6):** Right for the renames, which the diff now carries. Disclosure 6's comparison with P2 still cannot be checked here (G12).
- **F6 (disclosure 9):** Right, and deferred to P4 (G6).
- **F8 (disclosures 8 and 10):** Partly right. Disclosure 8 matches `diff-c3b5bbd.patch` 85-88, where `cols` is streamed and never held under a shift. Disclosure 10 adds an unsourced quote (G10), and it does not address F8's actual point about `cols` in the P1 ruling 4 text.
- **F12 (disclosure 11):** Right and suitably hedged. `log-size.log` 569-576 also shows emend-copy turning on both `dates` and `rowcol`.
- **F20 (disclosure 12):** The proposal is right. The unlogged-run sentence is not (G11). The claim that the clippy step runs on the workspace is supported only by the gate script's comment (`log-gate.log` 1001).
- **F4 (disclosure 13):** Right, but incomplete (G4).
- **Not answered:** F5 (the unreachable row-axis arms), F18's doc-commit limit (G8) and F19 (the repeated `set -uo pipefail`). F5 and F19 are harmless.
