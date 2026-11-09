# M8 part 1 P2: independent check 1

The answer of a fresh subagent given only the spec (as committed in
4296d03 and as working, with rulings A to G), specs/0001, the per-commit
and combined diffs (8b138cd..8816db0), gate-prune.log, gate.log, run.log,
moves.out, the two P1 driver outputs, the third driver's output, the
superseded run1 run.log and gate2 gate.log, the logs README as it stood
before the check, the scan pre-check log, two of the records and the
draft report, under <scratch>/check1. Extracted from the subagent's
transcript verbatim (sha256 f5a04b5185e09acdb6e92e6a6547ddcfa099017a00fccfb928652c23783badea); nothing is edited.
The README's section "After independent check 1" answers each exception.

root: <scratch> stands for the implementing session's scratch directory

**Independent check 1: M8 part 1 P2 (materials under `<scratch>/check1`)**

I hashed and counted every material with sha256sum and wc. Each of these matches the figure the README and draft report give:

| File | sha256 | Lines / bytes |
|---|---|---|
| logs/gate-prune.log | 2aad3fff | 906338 bytes |
| logs/gate.log | 8066a4c5 | 11934 lines, 920327 bytes |
| logs/run.log | d275146e | 7256 lines, 491135 bytes |
| logs/moves.out | cb0a6b4d | 14663 bytes |
| logs/rerun-driver.out | 36eeb629 | 2468 bytes |
| logs/logged-driver.out | ba23c399 | 1298 bytes |
| logs/logged3-driver.out | 33c4d489 | |
| logs/scan-pre-check.log | cd2e282c | |
| superseded/run1-run.log | b0ae5da8 | 7200 lines |
| superseded/gate2-gate.log | e872e83d | 11933 lines |
| spec-0047-committed.md | 03144cde | |
| spec-0047-working.md | 5c56f5ad | |

The two sample records hash to 5a1e3476 and fac8fbc0, which is what run.log lines 6981 and 7041 say was installed. A grep for non-ASCII bytes over the diffs, the draft report and the README found none.

## 1. Does the diff do what the spec says (with its rulings) and nothing else?

**yes, with exceptions**

- **Files touched.** The combined diff 8b138cd..8816db0 touches 11 files. Each belongs to one of the five commits, and no other file is touched.
- **4296d03 (doc commit).** It holds ROADMAP.md and specs/0047 only, which is what ruling D asks. Its message carries 03144cde and 50daf043, and gate.log lines 255-266 confirm both. ROADMAP marks P1 DONE and P2 IN PROGRESS.
- **182482a (count rule).** This is "The fixes (P2)" group 1 and ruling D.
  - `lo_count_check` is split out with the arguments reference, ods and reason, and keeps the 0/1/2 exit codes.
  - It parses both archives with zipfile and ElementTree, not a regex (diffs/182482a.diff around lines 225-280).
  - A cache table is told by the three marks; the reason uses the four-count wording.
  - The `count` class in edit-open-check.sh matches the new wording (lines 36-48).
  - lo-check-test.sh builds fixtures with `python3 -I -B` in a mktemp directory. It covers ruling D's four cases plus control (v).
- **bbcb69a (read-back).** This is decision 3 A.
  - `workbook_counts` returns a fourth value, `views`.
  - `sheet_read_back` branches on `views_before`. Without a view the output must read 0 and hold none.
  - The sheet line gains `views_before` and `views_after`.
  - The keep_output.rs fixture gains an empty bookViews, and null_rewrite.rs's expected lines gain the two fields.
  - The negative case is a unit test in main.rs, a deviation ruling E accepted.
  - The usage text is unchanged.
- **67310df (documents).** The README sentence reads 451512 / 7240 / 458752 and cites records/size-m7p2-2026-10-28-43471ed.md. The CHANGELOG "Unreleased"/"Changed" bullet is word for word the spec's text, and the 0.1.0 entry is untouched (diffs/67310df.diff lines 25-48).
- **8816db0 (the fix, ruling F).**
  - `-ne` became `-gt` (line 95), and the comment says why (lines 79-86).
  - Case (vi) is `(1, 1, ["displayed"])`, expecting 0 (lines 59 and 67). Case (iii) still expects 1.
  - edit-open-check.sh is unchanged.
  - The message names run.log b0ae5da8 and quotes line 3092 word for word; I compared it with superseded/run1-run.log line 3092.
  - The message ends with the diff-alone sentence and the Co-Authored-By line only.
  - The gate shows it working: gate.log line 9991 reads "ok (vi) lo_count_check exits 0 (got 0)" and lines 9993-9994 read "lo-check-test: PASS".

Exceptions:
- a. The 8816db0 message says "(specs/0047 ruling F, option A)" (diffs/8816db0.diff line 10). Ruling F in spec-0047-working.md (around lines 493-534) names no options. The code is right; the attribution in the message is not quite accurate.
- b. "The fixes (P2)" still says the test gains "a negative case where the cache count and the external sheet names differ" (spec-0047-working.md around lines 185-187). After ruling F only "caches exceed names" fails; case (iii) covers that and case (vi) passes. This follows ruling F, but the spec paragraph was not updated to match.
- c. Accepted deviations, listed for completeness: the negative read-back test is a unit test rather than an integration test (ruling D asked for integration; ruling E accepted the change), and the moves were not made by the logged driver (ruling G accepted this).

## 2. Do the logs contain every line the spec and its rulings require?

**yes, with exceptions**

- **The prune, yes mode.** gate-prune.log covers it:
  - lines 189 and 191: PRUNE_PREVIOUS yes; 28 kept names;
  - lines 231-232: 6251746515 bytes;
  - line 233: `rm -r "${PREV_DIR:?}/${n:?}"`;
  - line 234: rm -r exit 0;
  - lines 235-236: gone, and kept names still there;
  - control lines 183-188 (R90, ruling AD);
  - last line: "gate log checks all as expected: yes".
- **The prune, list mode.** gate.log covers it:
  - line 11: PRUNE_PREVIOUS list;
  - lines 189 and 229: "names of the prune set there ... 0 of 1; their bytes in all: 0";
  - line 231: "nothing of the prune set is there: nothing to remove (change M100)";
  - line 233: the kept names are still there;
  - last line: yes.
- **The rerun notes.**
  - gate.log: lines 4-5 (RERUN_NOTE set; RERUN_FILES are the gate2 gate.log and logged2.sh); line 366 names e872e83d, line 371, W71 and the driver-export cause; lines 369-370 hash both files; line 372 reads as-expected.
  - run.log: lines 4-5 and 74 name b0ae5da8, line 3092, id 9956034c4326 and 8816db0; lines 76-81 hash both files and check they are there.
  - This matches ruling G's split of each step's inputs.
- **The 59 runs and the record pairs.** run.log line 2951 onward. Driver lines 72-73 put "runs done 59 of 59 ... record pairs written 59" at run.log line 6860. run.log lines 6926-7051 install each file with its sha256 and bytes, ending "every record file was installed, byte for byte: yes". logged3-driver.out line 79 reads "records written: 59 .md, 59 .txt".
- **The expected-change table.** run.log lines 7053-7120:
  - one row per run;
  - group 1 met in 59 of 59, lo_in fail to ok in 59, lo fail to ok in 58;
  - google-sheets met 5;
  - TX unchanged 7;
  - three yes totals and a control.
  - The ids and the seven pairs are at lines 2922-2948.
- **FINDING lines.** run.log line 7123 reads "FINDING lines printed above: 0". The driver's grep (logged3-driver.out lines 9-75) matches only rerun notes, summaries reading "so far 0", and the 7122/7123 section. UNEXPECTED counts are 0 and 0 (driver lines 76-78).
- **The moves with hashes.** moves.out:
  - lines 1-4: HEAD, the counts 118 and 59, and the hashes b0ae5da8 and 2aad3fff;
  - from line 5: each record's sha256, then run.log's;
  - the rundirs list: 32309 files, its sha256 a016c0a7;
  - the `mv` lines;
  - the "after" section: files match yes, run directories match yes, gate-prune.log 2aad3fff, 0 left, superseded-runs names 2 then 61.
- **The README's headings.**
  - "The superseded runs" holds run1 (b0ae5da8, line 3092, cause, fix) and gate2 (e872e83d, line 371 quoted, cause, fix) at README lines 111-153.
  - "The moves" is at line 155, "The rule's edge" at line 180, and "P1's driver outputs" at line 200 with both hashes and byte counts (lines 208-209).
- **Ruling F's record list.** The record commit's logs (draft-report line 16) agree with ruling F's list as ruling G amends it.

Exceptions:
- a. Ruling F says "every moved file's sha256 printed". For the 32309 run-directory files, moves.out prints only the sha256 of `<phase>/logged2/rundirs.sha256`, a list file that is not committed. The `<phase>/gate`, `<phase>/out` and `<phase>/scripts/out` moves carry no hashes in moves.out. Ruling G accepted moves.out as the verified primary, so this is noted, not a failure.
- b. Ruling G asks for gate2's moves to be hashed. Per the README (line 150), that output is `<phase>/superseded/gate2/moves-gate2.out`, kept and not committed, and it is not among these materials. No committed log carries the gate2 move hashes.

## 3. Does every number and quoted string in the draft report and the logs README appear in a primary?

**no**

Most do; I checked them by hash, wc and line lookup. These do not appear in any primary among the materials.

**Draft report:**
- a. Lines 24-25: "`scripts/privacy.sh --range 8b138cd..HEAD` after it: 5 commits, 0 findings, PASS". gate.log line 9469 says "commits in the range: 5". No log here holds a privacy.sh range run or its result; I searched gate.log, scan-pre-check.log and logged3-driver.out.
- b. Line 29: logged3.sh "2bf6d599...". It appears only in the README and the draft report.
- c. Lines 103-104: "only the MW lines differ from the previous scan script". The previous scan script is not among the materials, so I could not check this.

**Logs README:**
- d. Line 54: gate script hash "6cfbfcc2". Line 64: run script hash "4de98728". Neither appears in gate.log, run.log, scan-pre-check.log or any other material. The gen/gate.py ca974255 and gen/run.py 670316b3 hashes do appear (gate.log lines 169 and 174).
- e. Line 75: logged3.sh sha256 2bf6d599, as in (b).
- f. Line 158: moves.sh sha256 "b90e64ba". moves.out does not print it.
- g. Line 197: "`<phase>/dry-d3.out` 7929c82b". Not in any material. "Dry d3 read as d2 did" is planning's statement in ruling G, not a log line.
- h. Line 141: gate2 "exit 1". superseded/gate2-gate.log ends "gate log checks all as expected: no" and has one UNEXPECTED line (371), but no exit status is recorded in the materials. The second driver's output is not provided.
- i. Lines 124-125: the run1 reason "the ods holds 1 table elements, 0 of them external-link caches, and the workbook 1 sheet elements and 1 external sheet names". It appears only in spec ruling F (spec-0047-working.md around lines 497-499), which is planning's text. It is not in superseded/run1-run.log; the first run's record files are not among the materials.
- j. Line 148: "no run directory or record was made" (gate2). This is only in ruling G; no log here shows it.
- k. Line 150: `moves-gate2.out`. Not among the materials (see Q2 b).
- l. Lines 213-222, the leftovers. None of these names or byte counts appears in any material:
  - tmp.xL4AeFD8GJ 29552724
  - tmp.XQ2hAd9Chs 29552736
  - G 4646960, R 1273383, S 4595934
  - dc2 20173, lo2 9453 (9453 occurs in run.log and scan-pre-check.log only inside unrelated hashes)
  - session-f 29862
  - tmp.uDLoJr2BMA 23
- m. Lines 195-196: the claim that the dry corpus "carries a second external sheet name that no formula uses". The generator's hash 2c5de985 does appear (gate.log line 170). The claim about what it carries is only planning's ruling F/G text; no dry log is among the materials.

**Small inaccuracies (the strings exist, but what is cited does not quite match):**
- n. README line 49 cites gate-prune.log "lines 189 to 235" for the 28 kept names. The "every kept name is still there" line is 236.
- o. README line 58 cites gate.log "lines 189 to 232". The kept-name line is 233.
- p. README line 41 fills the "Last line" column for moves.out with "the moves' output (below)". That is not a line of moves.out; its last line is the git status entry "?? records/logs/m8p1-p2/".
- q. README line 66 says the runs went "from 01:48:20Z to 06:29Z by the driver's line". 01:48:20Z comes from run.log line 2956, not from the driver. The driver reads 01:48:18Z (run start) and 06:29:29Z (logged3-driver.out lines 6-7).

Everything else I checked appears in a primary: every other hash, size, line number, count and quoted log string in both documents. That includes the commit subjects and files, the expected-change table text, the seven pairs and their triples, the per-sample summary counts (changed 1 / changed 6), the scan's "structural checks failed: 0", "RESULT: PASS", EXPECT_RECORDS 59 of 59, the MW line, stderr 0 bytes (scan-pre-check.log line 8653), and the README scanned at 0dc61b03 (scan-pre-check.log line 6891), which equals the README as given.
