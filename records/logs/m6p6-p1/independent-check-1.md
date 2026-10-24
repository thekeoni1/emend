Check 1, M6 part 6 P1 (specs/0043 "P1: the scripts", rulings A to F)

1. Does the diff do what the spec says (P1, as amended by rulings A to F) and nothing else?
Verdict: yes.

Evidence:
- `git diff --numstat f15a11b 99b671d` lists six files. `git show --stat 99b671d` shows that the code commit touches only the three scripts. ROADMAP.md, specs/0042 and specs/0043 come from the two doc commits, 9c62247 and 55e6363.
- scripts/compare-run.sh: two lines change. OPS_LIST gains chartsheet, chart-format, chart-analytic, chart-text and named-style, and the refusal now reads "--op must name one of the sixteen ops". This is "nothing else in it changes" as ruling B Q1 corrects it.
- scripts/compare-record.py:
  - OPS gains the same five ops. SCALE is unchanged.
  - The docstring cites specs/0043 beside the old citation, and the default stem is compare-m6p6- in both the docstring and the code.
  - The title reads "Milestone 6 part 6 P2" and the opening paragraph names specs/0043 beside specs/0037. These are ruling B Q2's three places.
- scripts/compare-record-test.py:
  - The changes are OPS, the stem (three places), the docstring's "sixteen" and the counts with their check text: ops 16, scale rows 102, 48 corpus rows, 16 detail rows, 102 scale rows.
  - The arithmetic is right: 16 x 3 = 48, and (16 + 16 + 1 + 1) x 3 = 102.
  - No check() call is added. The ninth added line is the OPS tuple's wrap.
- No "eleven" or "m5p6" is left in any of the three scripts at 99b671d (grep).
- scripts/openpyxl-ops.py is not in the range, as ruling A requires.
- Ruling C: commit 99b671d is "the three scripts alone".
- The doc commits' blob hashes match their commit messages (2085cb2a..., ceed814e...). The working spec hashes to 38bd78f4..., as given.
- Doubt (not a defect): "nothing else" for the whole range depends on counting ROADMAP.md and specs/0042 as planning's doc content. Their text is planning's, and I did not judge it.

2. Do the logs contain every line the spec requires?
Verdict: yes.

Evidence:
- **Test log:**
  - It runs `TMPDIR=<scratch>/w/tmp bash <repo>/scripts/openpyxl-ops-test.sh ...` (block O, "109 ok lines, no FAIL, the last line PASS ... : yes").
  - It runs `python3 -I -B <repo>/scripts/compare-record-test.py` (T1, 34 ok lines, "compare-record-test: PASS", "34 ok lines, no FAIL, the last line PASS ... : yes").
  - It prints both scripts whole (`git show <phase commit>:... | sed`).
  - It does not run check.sh or the gate; the only mentions are in comments and a status-pattern control. It states "this log runs no cargo".
- **Negative log**, all four cases:
  - N1: the runner exits 2 with "error: --op must name one of the sixteen ops; ...". N1b: a run.openpyxl.tsv naming no-such-op is refused with "names another op" (ruling B Q3).
  - N2: a row naming named-style in the chart-text run is refused.
  - N3: "error: the named-style runs do not cover the same workbooks".
  - N4: the planted path is "in the run subdirectory's stderr file", with 0 lines of it in the runner's output and 0 in the tables.
  - The controls are C0 and R0, and there is no binary case.
- **Gate, item 5 and ruling BI:**
  - The gate prints "the charts probe's sections: 1151328; part 5's size record's: 1151328; the difference ...: 0".
  - The budget line reads 451448 against the record's 451448.
  - 1055368 and 1075632 appear only in the printed script's comments (change Q85 and history), never in an output line.
- **Ruling C:**
  - du -sb shows 9583979959 for m6p5-p3 and 2143316010 for m6p5-close, each followed by `rm -r` (no -f) exit 0, with the kept-names checks as-expected. The script uses the `"${PREV_DIR:?}/${n:?}"` form.
  - The README records the stray rm line.
  - Observation: the gate also visits m6p5-p1 and m6p5-p2 ("0 of 1"), which is more than ruling C's two roots. Nothing was removed there.
- **Ruling E:**
  - The logged gate shows "lines: 0", and change X85 is in the printed source.
  - <phase>/dry/d1/gate.log has "lines: 3" and UNEXPECTED at that check, and ends "no".
  - The README names d1 and d2.
- **Ruling F:**
  - The negative log has "rerun:" naming F2, RERUN_FILES naming the superseded log, outer stderr and .sh in <superseded>/negative-1 (hashed), and the RERUN_FILES .sh check "yes".
  - It has the two root lines for <corpus-runs> and <scale-runs> (change NH58). Their wording equals records/logs/m5p6-p2/test.log lines 52 and 53, with " (change NH58)" appended.
  - PLANNING_SPEC is 38bd78f4....
  - The README names d3 and the rerun.
- **Endings:** each of the three logs ends "... checks all as expected: yes".

3. Does every number and quoted string in the draft report appear in a primary?
Verdict: yes with exceptions.

Checked and found in a primary:
- All four commit hashes and their subjects, and origin/main ee78ffa (git rev-parse).
- The numstat table, which equals git exactly.
- Both PLANNING_SPEC hashes and which logs used each: gate and test 17adfe77..., negative and scan 38bd78f4....
- The four log hashes, which equal sha256sum of the files.
- 9583979959, 2143316010, the nine PASS lines, 538/711/575/753/756/818/1199, 155, 169, 1151328, the difference 0 and 451448.
- "109 ok lines, no FAIL, the last line PASS" and "34 ok lines, no FAIL, the last line PASS".
- "ops 16; scale rows 102", 48, 16 and 102.
- "lines: 3", "lines: 0", X85 and NH58.
- The F2 sentence, which is verbatim at line 6014 of <phase>/scan-pre/scan.log and also in the README.
- "--op must name one of the sixteen ops".
- The scan pre-check's hash cf1b10d3... (it equals check1/logs/scan-pre.log), "total hits: 0", "structural checks failed: 0" and "RESULT: PASS".
- The first pre-check's "RESULT: FAIL".

Exceptions and doubts:
- (a) The path `<phase>/scan-pre2/scan.log` is not named in any check1 primary; the README names only `<phase>/scan-pre/scan.log`. I confirmed it from the file system, read only: that file hashes to cf1b10d3....
- (b) The report gives the stray line as `rm -rf /dev/null`. The spec and the README give it in full as `rm -rf /dev/null 2>/dev/null; true`. It is a shortened quote, not a wrong one.
- (c) "Both ruled by planning before any fix" is not marked as the session's statement, and no primary shows the timing. Rulings E and F prescribe the fixes, which supports it but does not prove it. "Neither changed the phase's code" is supported: 99b671d is unchanged, and the fixes are generator changes X85 and NH58.
- (d) These are the session's statements, carried only by the README: d1, d2 and d3 reading "yes", the forks, the models, the generator and script hashes, and the stray rm line failing and removing nothing. I verified d1's gate.log independently. I did not verify d2, d3 or the generator hashes.
- (e) "Nothing pushed (session's statement)": git shows origin/main at ee78ffa. That is the local remote-tracking ref, and no network check was made.
- (f) "2026-10-23 (UTC 2026-10-24)": the logs' end times are 2026-10-24T02:41Z to 02:47Z. The local date 2026-10-23 is the session's.
- (g) "(no log prints it in this form)" is correct as worded. The gate does print per-file "added lines" 20, 9, 9, 2 and 78 in another form.

No figure differs from its primary.
