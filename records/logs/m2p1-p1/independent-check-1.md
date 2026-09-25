# Independent check 1, M2 part 1 P1

Checked commit: ade3c49be9071cfab25d42ed94289d0050217c61
HEAD at the check: ade3c49be9071cfab25d42ed94289d0050217c61
Checker: a fresh general-purpose subagent (not a fork), given only the files below in one scratch directory.
Inputs, each with its sha256 (as given to the checker):
- 0016-new-workbook-and-sheets.md: e4d03eccde8282c305d8b715233044d4f61d000e698a85d063dd37c419fefa91
- 0001-bootstrap.md: 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
- code-commit-ade3c49.diff: 8875af4960ae7da5f6ae3c1cd7d16dbbb3a22db7c4f1409aacbc4f2499a7a66c
- doc-commit-1450dbd.diff: 76af02a0cf5561fb350f33b3a96088cc14f15af085277eb847186c36b1b0c7b5
- combined.diff: f03b9ea17b08f3f066196646040749c2c9ba5167178f17c17271b5f9cef46c1f
- draft-report.md: 21b56c3d8c1862a5b7627fa5545cd6b1442863c945758e20121f6a16593b6a24
- gate.log: 07b528c9771568030b7926a045a473f2fe17f7412dcb911b83bd519989a6d6dc
- test.log: 1db1cac369e6da4e577d6d9886459b1ad0c94f2ec1bb79a0eaeed978497cc846
- negative.log: 43544b262d53fdce0ccdd8da756f920775e3619d55a17a0151c3ac0366db438f
- run.log: 33b6b1dcf39ed4329dbaaaaf95c9d3a8898509c8598e0be3eba4517a44cd47e0
- scan-pre-check.log: f870ede87ea2db0a6be9ac0f5f281c9a28af1d61ac0d81c8794138fea075d9de
- m2p1-open-2026-09-25-ade3c49.md: a7767765a14cdbd0d34529dadb15abc159425f383e8ad0ac1508409f06bcfb02
- m2p1-open-2026-09-25-ade3c49.txt: 70684a4ed62ca32dbcb3ad23222fcf31b685c96e631812e2dd302ed5bec88f06
- m2p1-p1-2026-09-25-ade3c49.md: 917260f4e3bb3f496a9404a1441318a0fbd88a9da0ef919711b970eebf5abce8
- m2p1-p1-2026-09-25-ade3c49.txt: c98ec386ecf4e90968f18f12b2a4b1e91792f5070d2b62a325ccaff498300167
The negative.log given was attempt 2, sha256 43544b262d53fdce0ccdd8da756f920775e3619d55a17a0151c3ac0366db438f; the committed negative.log is attempt 3, rerun for this check's F4.
The answer is ASCII and holds no absolute machine path; it is reproduced verbatim below (sha256 of the answer text 5c5bc6281db3a8d5321c11bb5ae0ab2d29276a4b678614ba9c3f5657217d5a54, 10685 bytes).

---

I ran this check from the files in the check directory only. The harness put CLAUDE.md into my context, and I have not used it as evidence.

Before comparing anything, I hashed the inputs:
- `combined.diff` hashes to f03b9ea1, the value gate.log line 1064 prints for the phase's combined diff.
- The file sections of the two per-commit diffs hash to the values gate.log prints: 33e5560d for ade3c49 and 521cc3eb for 1450dbd (gate.log lines 1059 to 1062).
- The four sibling logs hash to the values scan-pre-check.log prints (lines 1262 to 1292).
- The four record files hash to the values run.log prints on install (around lines 2235 to 2246).

**Question 1: does the diff do what the spec says for P1, and nothing else?**

Yes, apart from the nits listed as findings below.

- **Doc commit (I read doc-commit-1450dbd.diff).** It changes exactly three files: ROADMAP.md, specs/0012-styles.md and the new specs/0016 (diff lines 14, 78, 98). Gate.log lines 1017 to 1056 check each file against planning's hashes (d45f4932, afb9fee6, a7cf94e1) at the doc commit and at HEAD, and confirm there is no fourth file. I also rebuilt the committed specs/0016 from the diff and compared it with the working-tree spec: the only difference is the "Planning rulings during P1" section, which matches gate.log lines 1305 to 1308 (working tree e4d03ecc, uncommitted). Nothing else in the doc commit: confirmed.
- **Code commit (I read code-commit-ade3c49.diff).** It touches four files, all scripts or records.
  - `scripts/edit-open-check.sh`: the heading reads exactly as the spec words it (diff line 151).
  - `scripts/excel-check-list.py`:
    - umask 077 is set before any write (line 285), and IndexError is among the caught exceptions (line 314).
    - `--run-plain` can be repeated (line 280). Its rows take the directory's basename as label and have empty references (lines 256, 301), and it is refused under another corpus root (lines 302 to 303).
    - The size check against `out_bytes` applies to `--run` and `--run-plain` alike, with ruling 1's error words (lines 251 to 252).
    - Each accepted fixture row is followed by its input row, with the record's `in_bytes` and `in_sha256` (lines 272 to 273). The docstring covers all of it (lines 162 to 216).
  - `scripts/excel-check-open.ps1` (new):
    - Builds paths only from `$PSScriptRoot` (lines 366 to 368 and 405).
    - Opens with `Open(path, 0, $true)`, Excel visible, alerts on (lines 399, 400 and 411), and quits at the end (line 435).
    - Reads values from `Worksheets.Item(1)` with the `Value2` rules (lines 384 to 396 and 422). A read that throws records `read-error` and keeps `pass` (lines 423 to 426).
    - Writes the eight columns as ASCII with tabs, CR and LF cleaned out (lines 380 to 382, 401 and 436).
  - `records/excel-check-template.md`:
    - The automated procedure and the input and seen columns (lines 63 to 95).
    - The by-hand fallback (lines 87 to 92).
    - A label column in both the corpus and fixture tables (lines 120 and 127).
    - The allowed-input and output-finding sentence (lines 136 to 141).

  The commit message ends with the Co-Authored-By line only (diff line 27; gate.log line 1010). Nothing else in the code commit: confirmed, apart from small additions noted in F5 and F6.
- **Combined diff (I read combined.diff).** It holds seven files (lines 1, 65, 177, 190, 363, 473, 493). Its code sections and its doc sections are byte-identical to the two per-commit diffs. It has no crate, tool, Cargo.lock or manifest path, and gate.log lines 1137 to 1141 agree. No gate file is in it (gate.log lines 1113 to 1126). Nothing else in the phase range: confirmed.

**Question 2: do the logs contain every line the spec requires?**

Yes, with the gap in F4.

- **Style-sample record against part 5 P2's expectations (rulings 8 to 10):**
  - run.log 1497 to 1512: the part 5 P2 script is extracted, its sha256 e5380f13 checked against the value that log printed, and cmp.py read from its heredoc with a control.
  - run.log 1514 to 1630: 27 ok; lo and lo_in ok on 27; entry totals identical 816, changed 54, flagged 34; each workbook's changed entries are the two parts and the rest identical; read_back equals styled plus 2 on 27.
  - The run.log line 1624 totals match ruling 9 (992, 1046, 27, 27), asserted at lines 1627 to 1630.
  - Planted-flag controls, run.log 1632 to 1655: 17 plants turned their flag, and the reorder plant turned dated_xf_ok alone.
  - The record was run with `--allow-single` (run.log 1663 to 1666), and its `.md` line 8 reads "Targets: host". The ten-check record comparison with its fixed control is at run.log 1793 to 1796.
- **Keep on both targets compared:** run.log 1904 to 1911 (298 files each, byte-identical, 149 whole pairs). Row-for-row equality with part 8 P2's record: run.log 1913 to 1917.
- **Edit open check over the host keep:**
  - run.log 1919 onward: totals 149; lo_in 138/11; lo 139/10.
  - The heading as ruled is checked with a control against the old `.md` (around line 2195).
  - Acceptance at run.log 2209 to 2219: crate and tool each have 0 outputs refused on an accepted input. The open record's `.md` lines 42 to 48 read "None."
- **The list (run.log 2249 to 2344):**
  - Uses `--run` from part 6 P3's run directory plus `--run-plain`.
  - Counts line: corpus 54 (with A 19, with F 15; 27 from `--run-plain`), fixture 138, input 138, 11 left out.
  - Every row checked against its file (330 of 330) with a control.
  - `--run-plain` sizes checked against `out_bytes`.
  - References checked against part 6 P2's derivation (27 of 27) with a control.
- **Staged zip sha256 printed:** run.log 2356 to 2361 (99b3376d, 332 entries, testzip ok, mode 600).
- **List script negatives (negative.log 928 to 1101):**
  - L1 to L5 (the earlier cases).
  - L6: `--run-plain` with no summary.
  - L7 and L7r: one byte past `out_bytes`, on `--run-plain` and on `--run`.
  - L8: another corpus root.
  - L9: a short manifest row, with the base script's IndexError traceback as the control.
  - L0 and L10: mode 600 against the base script's 644 under the same umask.
  - Keep pair with a refused input: fixture.b has no row (L0, around line 994).
- **PowerShell ASCII and absolute-path grep with controls:** negative.log 1103 to 1121. One control per form: non-ASCII, drive, UNC, root and home paths, plus the relative shapes passing.
- **Tree-status checks printing the lines:** gate.log 1298 to 1310; test.log 1935 to 1941; negative.log 1127 to 1134; run.log 63 to 64 (entry) and 2368 to 2379 (end); scan-pre-check.log 1354 to 1366.
- **Rerun headers:**
  - run.log 28 to 46 names attempts 1 and 2 and prints the hashes of their logs, scripts, records, lists and zips.
  - negative.log 30 to 33 names attempt 1 with its hash.
  - Gate and test are single attempts.
- **Forbidden-shape rules:**
  - Every log greps its own script first, with fixture controls (each log's lines 5 to 15).
  - scan-pre-check.log 1401 to 1428 reports 0 instances over nine files, with cross-controls.
  - scan-pre-check.log 1378 to 1399 reports 0 hits from the pre-P6d bare-shape control.
  - The scan's result is PASS (lines 1430 and 1434).

**Question 3: does every number, hash and quoted string in draft-report.md appear in a primary?**

Nearly all do. I found these in primaries:
- 1450dbd, its parent d4ab367, and the d45f4932, afb9fee6, a7cf94e1 and e4d03ecc prefixes (gate.log).
- 523 on both targets (test.log lines 1273 and 1844).
- Everything in the style run: 27/27, 54, 34, 992, 1046, 27, 27, "17 of 17" and "ten checks".
- 149 pairs; 138/11 and 139/10; findings 0.
- 54, 19, 15, 27, 138, 138, 11 and 330.
- bcda16d9... in full, and 99b3376d... in full with 33010705 bytes, 332 entries and 332 files.
- The four record hashes (917260f4, c98ec386, a7767765, 70684a4e), which match the run.log install lines and my own hashing.
- The attempt hashes 3e4c5b5e and c94a8152 (run.log lines 30 and 38), and "2 bytes" (negative.log line 31).

The exceptions are F1 to F3 below.

**Findings**

- **F1 (nit), report line 21.** It says cmp.py "was extracted at the printed sha256 e5380f13...". In run.log lines 1499 to 1503, e5380f13 is the hash of the extracted part 5 P2 run script. cmp.py's own hash is ea176c10 (run.log line 1504). The hash is attached to the wrong file.
- **F2 (nit), report line 44.** "a syntax parse (0 errors)": the count "0 errors" appears in no primary. Ruling 11 allows the check to stay unlogged, but the report should say the count is not logged.
- **F3 (nit), report line 38.** It gives attempt 1's fix only as dropping the family table's total row. run.log line 28 says the attempt 2 script also removes a superseded run directory named by SUPERSEDED_RUN, and the fixed control also cuts the `.txt` by one summary row (run.log line 1796). The report leaves out both changes.
- **F4 (gap), negative.log line 36 and lines 822 to 826.** The negative tests run `scripts/excel-check-list.py` from the main working tree, with all fixtures and outputs in `<scratch>/w`. specs/0001 "How this task runs" says every negative test runs in a scratch copy and never in the main working tree, and that a departure needs planning's word first. The log cites no such ruling. From these inputs I cannot confirm whether an earlier ruling covers this pattern. Status lines 1127 to 1134 show the tree unchanged afterwards.
- **F5 (nit), code diff line 279.** The spec says `--run` is "given once", but the script does not enforce it: if `--run` is repeated, argparse keeps the last value silently. The extra refusal "a --run-plain is not a directory" (lines 289 to 290) goes slightly beyond the spec but is harmless.
- **F6 (nit), code diff lines 369 to 372, 390, 430 and 435.** The .ps1 adds three things the spec does not state: it refuses to start when `open-results.txt` already exists, prints progress lines, and applies an absolute-value bound of 1e15 to the whole-number case. Also, `$wb.Close($false)` is outside any try block. If Close throws, the script ends before `Quit` and before writing any results.
- **F7 (nit), run.log lines 1481 to 1487.** The "tool unchanged since part 6 P3" check lists changed files only under tools/emend-copy, crates/emend and Cargo.toml. It leaves out Cargo.lock and the xlsx-diff tool, and it does not compare the release binary hashes with part 6 P3's run row. Report line 16 states only what the log shows, so this is a nit.
- **F8 (nit).** The report's log list leaves out scan-pre-check.log, which ended "RESULT: PASS" with 0 total hits (lines 1430 and 1434).

Verdict: the diff matches the P1 spec and rulings, and the logs hold every required line, with one gap to confirm (F4) and nits F1 to F3 and F5 to F8.
