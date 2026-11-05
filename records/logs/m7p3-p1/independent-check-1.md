# Independent check 1 (M7 part 3 P1 record)

The answer of the fresh subagent, verbatim but for one edit: the ruled
producer version value it quoted (from the working spec's ruling I, a
value the intake never prints) is replaced by <the ruled producer
version>, <the version number> or <the producer-version word>. The
unedited answer, sha256 600316f92437296f31168fe3805f62420c0aed032e1fdca5c86d51c71b43ef3f, is in the implementing
session's report outside the tree. <scratch> in the answer stands for
the implementing session's scratch directory.

---

# Independent check 1: the M7 part 3 P1 record commit

## Question 1: does the diff do what ruling K says, and nothing else?

I rebuilt every file from <scratch>/check1/staged.diff and hashed it. All ten are new files (mode 100644) with 0 non-ASCII bytes. The working spec appears in git status as unstaged (" M"), so it is not in the diff, as ruling K requires.

| File in diff | Named by ruling K | Bytes | sha256 (rebuilt) |
|---|---|---|---|
| records/corpus-coverage-2026-11-05.md | yes ("the record") | 6284 | 2e1d4ad6... |
| records/logs/m7p3-p1/README.md | yes | 6925 | 4410c9d3... |
| records/logs/m7p3-p1/gate.log | yes (655ccce4) | 861186 | 655ccce4... |
| records/logs/m7p3-p1/gate-prune.log | yes (9eea6b27, 861875) | 861875 | 9eea6b27... |
| records/logs/m7p3-p1/list.log | yes (393be9f3) | 59553 | 393be9f3... |
| records/logs/m7p3-p1/add-r1.log | yes (02665c31) | 63695 | 02665c31... |
| records/logs/m7p3-p1/list-r2.log | yes (a9a56665) | 60132 | a9a56665... |
| records/logs/m7p3-p1/add-r2.log | yes (f18a2fb6) | 65420 | f18a2fb6... |
| records/logs/m7p3-p1/coverage.log | yes (7261f0c1) | 63646 | 7261f0c1... |
| records/logs/m7p3-p1/negative.log | yes (86f3420c) | 59054 | 86f3420c... |

- **Extra:** nothing. There is no ruling file, no driver output and no spec in the diff.
- **Missing:** independent-check-1.md and scan.log. The brief says they come later, so I do not count them.
- **The record:**
  - Its fenced body is 65 lines with sha256 31fa6d7e855f77b1..., the same as coverage.log lines 822 to 886 with the indent removed.
  - It is in the 2026-09-16 form: commit, date, script sha256, command, body.
  - Its "Gaps" line matches coverage.log lines 890 and 891.
- **README content against ruling K:** each log is listed with sha256, bytes and what it is. It covers the two rounds, the two slips (C96 with ruling D, and 56dd3766 with ruling J), the g4/g5 dry-run failure (ruling E), and the superseded generator and driver hashes. Three points in it:
  1. **Inaccurate sentence.** The README says "Every log's own standard error and the driver's capture of it were 0 bytes." For gate-prune.log this is not true. Ruling D says that run's driver.out "holds two cp error lines and an empty stderr size", so the driver never captured the stderr. Only the gate's own stderr was 0 bytes, and gate-prune.log's tail shows that ("<phase>/out/gate.stderr: 0 bytes").
  2. **Self-contradiction.** The README says "No file name, note, producer version or cell content ... is in these logs or this record". Line 208 of the diff (the README's slip bullet) then describes the producer version's form: 'without its <the producer-version word> word'. The version number itself does not appear, but the sentence and the bullet disagree.
  3. **Placeholder.** The heading "Independent check 1 ... dispositions below" has nothing below it yet.

## Question 2: do the logs contain every line the spec requires?

**Intake listing, round 1 (list.log):**
- "entries: 5".
- Entries 1 to 5 each have the form "a file, N bytes, sha256 ..., id ...; in the corpus already: no", followed by "corpus-tags.py exit 0: ..." and "(standard error: 0 bytes)".
- Entry 4 carries "comments threaded".
- The sha256s and sizes match ruling F.
- "FINDING lines printed above: 0" and "list log checks all as expected: yes".

**Intake listing, round 2 (list-r2.log):**
- Lines 823 to 850 list entries 1 to 7. The sha256s and sizes match ruling H, and entry 5 carries "charts chartex".
- "FINDING lines printed above: 0" and "list log checks all as expected: yes".

**Added lines:**
- add-r1.log lines 849 to 877 hold five "corpus-add.py exit 0: added ..." lines. Each entry has "removed from the incoming directory: yes".
- add-r1.log also has "ruled rows: 5; added: 5; refused: 0; manifest rows before 27, after 32", "entries left in the incoming directory: 0" and FINDING 0.
- add-r2.log lines 857 to 899 hold seven added lines. It has "ruled rows: 7; added: 7; refused: 0; manifest rows before 32, after 39", "entries left ...: 0" and FINDING 0.
- **Small gap:** ruling C says "the add log names the round". Neither add log's text contains the word "round" anywhere; only the file names (add-r1, add-r2) and the README name it. Rulings G and J accepted both logs as they are.

**Coverage (coverage.log):**
- "rows at entry: 39" (line 816).
- Line 820: "exit 0; standard error 0 bytes; stdout 65 lines, sha256 31fa6d7e...".
- The stdout runs from line 822 ("files 39") to line 886, including "uncovered 0" and "single-producer 8: pivots,slicers,timelines,macros,external,sparklines,date1904,large".
- The gaps block at lines 889 to 891 repeats both lines.
- FINDING 0 (line 895) and "coverage log checks all as expected: yes".

**Negative (negative.log):**
- Line 815: "refused: not a zip-based workbook"; line 822: "refused: unknown, empty or repeated tag"; line 826: "refused: unknown source".
- Controls: line 819 (the proposer accepts a fictional workbook) and lines 830 to 831 (corpus-add.py accepts with a known tag and source).
- FINDING 0 (line 835) and "negative log checks all as expected: yes".

**Prune lines (gate-prune.log):**
- Lines 242 to 246: P1 "2 of 2; their bytes in all: 8757835318", then "rm -r exit 0" for target and negative/target.
- Lines 312 to 316: P2, 6227782648 bytes.
- Lines 379 to 383: P3, 6697145547 bytes.
- Lines 436 to 440: close-out, 2146536745 bytes.
- Lines 179 to 184: the R90 control.
- Line 458: P4's directory, as-expected.
- Line 459: the C96 control, UNEXPECTED. This is the only UNEXPECTED result line.
- Last line: "gate log checks all as expected: no".
- The four sizes add up to 23829300258. That total is printed in ruling D and the README, not in the log itself.

**Rerun (gate.log):**
- Lines 236, 301, 355 and 402: "0 of 2", "0 of 1", "0 of 2", "0 of 1".
- Line 423: the C97 control, as-expected.
- Line 11: PRUNE_PREVIOUS list. Line 179: GATE_PREV_ROOT unset.
- 0 UNEXPECTED. Last line: "gate log checks all as expected: yes".
- Neither gate log contains the word FINDING.

**Stderr sizes:** every intake log has "wc -c <scripts>/out/intake.stderr: 0 bytes" near its end, and both gate logs have "<phase>/out/gate.stderr: 0 bytes". The driver-level stderr files are not in the diff, so I could not check them here.

## Question 3: does every number and quoted string in report-draft.md appear in a primary?

**Found:**
- The spec's sha256 87680af5... (sha256sum of spec.md gives it; scan-pre.log line 8) and its 713 lines.
- f8f4ee11...
- The record's 2e1d4ad6... and 6284 bytes.
- ff02af14..., 31fa6d7e..., 65 lines, and "files 39", "uncovered 0" and the single-producer line.
- All eight log hash prefixes and byte counts, and both gate last lines.
- 23829300258 and "six" (ruling D).
- Both "ruled rows" quotes.
- The tag counts: macros 1, comments 8, charts 12, pivots 5, slicers 3, timelines 2, external 3, sparklines 5 (record body).
- The three refusal quotes.
- 12abf602 (ruling E and README) and 56dd3766 with 843 bytes (ruling J).
- <the ruled producer version> (ruling I).
- "structural checks failed: 0" and "RESULT: PASS" (scan-pre.log tail).
- The MD "five" and MG "eight" sibling lists, and SIBLINGS/SCRIPT_LOGS naming the eight (scan-pre.log lines 4073, 4097, 4176 and 4178).

**Not found, or only partly found:**
1. "gen/scan.py (now 3071674d, script 87ef2284)": these hashes appear only in the implementer's own README. scan-pre.log never prints them. "Nothing else in the scan changed" is not shown by any primary I have.
2. "exit 0" for the pre-check scan: scan-pre.log prints no exit status for itself, only "RESULT: PASS".
3. "The resume check of 2026-11-02": the words "resume check" are not in any primary. Ruling D describes this slip as "fragments of four file names in its own tool output from a `find -printf` listing". The report leaves out "four" and where the fragments came from.
4. "each with a passing control" (Negative): the log has two acceptance controls for three refusals. The unknown-tag and unknown-source refusals share the one corpus-add.py control at line 830. Ruling K itself says "the two acceptance controls". The wording overstates what the log shows.
5. The Privacy section says no "producer version text beyond the ruled '<the version number> <the producer-version word>' ... appears" in any staged file. I disagree in part: the README carries the word <the producer-version word> as part of describing the producer version (diff line 208). The number <the version number> does not appear.
6. Also worth naming, though it is not a missing number: the report presents change MG as going "beyond ruling K's word". Nothing in the spec rules on MG. It needs planning's word, and the report has disclosed it.

## Privacy

- I found no original file name, note or cell content in the staged files.
- Every .xls name in the diff is an id-named corpus copy (`<corpus>/corpus/<id>.<ext>`) or one of the fictional `<work>/neg/notzip.xlsx` and `fictional.xlsx`.
- There is no "Microsoft" string, no home path, no session id and no email address.
- The one borderline item is the README's <the producer-version word> word above.

## Verdicts

1. yes-with-exceptions: the files are exactly ruling K's (the check and scan come later), but the README wrongly says the driver's stderr capture was 0 bytes for gate-prune.log, contradicts itself on producer-version text, and has an empty dispositions placeholder.
2. yes-with-exceptions: the add logs never name the round in their text (ruling C).
3. yes-with-exceptions: the scan hashes 3071674d and 87ef2284, the pre-check's "exit 0" and the "resume check" wording have no primary; "each with a passing control" overstates the log; the privacy claim is contradicted by the README's <the producer-version word> word.
