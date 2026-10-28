# M7 part 2 P1, independent check 2 (its answer verbatim, but the home directory's path written <home>, 2 times, and the harness session directory under it written <session>, once; the answer as given is sha256 574f8977a73f9c8137a75cce94a80df75cb86855d8e03d4100f480148b939914 in <phase>/logged/check2-answer.md)

## Second independent check of the M7 part 2 P1 rerun (ruling L)

**Overall:** the rerun closes check 1's three gaps (a), (b) and (c). Every check line in both regenerated logs reads yes, and I found no wrong number or misquote in the report. What remains is a set of weaknesses in how strict the new readings and controls are, plus a few report statements that none of the three primaries supports. Both are listed below.

### 1. Negative log: **yes, with exceptions**

**What ruling L (1) asks for is all there:**
- **Message texts.** All three are in negative.log 5702-5704. R3, R4 and R5 repeat them at 5862, 5877 and 5895.
- **R3, fix 3.** It reads chart_write.rs at 43471ed and prints line 1435 (the message), 1443/1444 (None refused), 1454 (the chartsheet's expect_err), and 1459/1460/1468 (Marker taken). Then come the check (yes), the pass line, and the control: a copy with None and Marker swapped is refused (5862-5874).
- **R4, fix 4.** For the removal alone, the log prints line 883, the message at 885, the `refused_after` line before the removal, and "lines naming add_chart in the body: 0". For the removal with an addition, it prints lines 830, 832 and 834. Both tests' pass lines follow, then the control: an addition planted after the removal is refused (5877-5892).
- **R5, fix 5.** It prints the `PERIOD` constant (count 1) and lines 5722, 5725, 5726, 5729 and 5735: `[0, 1, 256, 1000, u16::MAX]` refused with PERIOD, and `[2, 255]` taken. The pass line follows, then the control: a copy with 256 cut out is refused (5895-5908).
- **K3 now has a control (5812-5822).** The scatter style test runs alone with its own keep (exit 0, "1 passed"). Pair 3 has `scatterStyle val="marker"` 1 in xl/charts/chart1.xml, and the check reads yes.
- **Every check is yes.** Outside the printed script and library there are 109 lines ending ": yes" and no check ending ": no". The only ": no" line (132, "ANTHROPIC_API_KEY set in this shell: no") is a status line. The last line is "negative log checks all as expected: yes" (5945).
- **The printed script is the one that ran.** It matches `<phase>/scripts/m7p2-p1-negative.sh` apart from one trailing blank line. Its sha256 is b3e84f4c..., as the report says.

**Exceptions** (I checked each against the script text at log lines 5247-5442):
- **E1. K3's control does not use K3's own reading.** K3 counts chart parts by matching `^xl/charts/chart[0-9]+[.]xml$` over `zx names` (5253). The control instead uses `zx count ... 'scatterStyle val="marker"'` (5268). It does show that an accepted scatter pair holds a chart part, which is what the ruling's words ask. It does not show that K3's own regex-and-pipe can find a chart part, so if that reading always returned 0 the control would not catch it. A same-reading control was not explicitly required.
- **E2. "Refused by add_chart and by add_chartsheet" (R3, 5872) is not shown by the lines read.** No line read names `add_chartsheet` (the call is at source 1452). Line 1454 is a bare `.expect_err("refused"),`, and the log never reads the `call` helper. I confirmed from source that `call` is `add_chart` (chart_write.rs 297-299), so the claim is true, but the log does not prove it.
- **E3. R5's refusing call itself is not read.** The `refused(` line (5723) is missing; only its arguments are read (5725 `trended(...)`, 5726 `PERIOD,`). The check only tests that the lines come in ascending order, not that they belong to one call. So "the refusing call read from the test file" is met only through its arguments.
- **E4. R4's accepted half has no control of its own.** The one R4 control covers only the refused body. That meets "a control each" if read per fix. The control also exercises only the `add_chart`-count part of the three-part check. The `refused_after` line check has no control.
- **E5. The "exact test's pass line" is a count.** It is printed as `test <path> ... ok: 1`, a count of the exact line in tests.out. The raw cargo line is never printed verbatim, here or in the earlier run section.

### 2. Size record and size log: **yes**

- **Attribution.** The record has it at line 142, directly after the core-movement paragraph (137-141). It matches ruling L (2)'s text word for word, preceded by "This movement is". The log prints its own form at size.log 11069 and the record copy at 13561. It checks it at 13828 ("the record names ruling J's attribution ... once, after the core movement (change SG): yes"), with a control at 13829.
- **The two commit lists are kept apart.** Record 338-340 reads: "5d14c89, 35e3d1d, 64f9062, f79e447; the part's five fix commits are 4a2582b f79e447 64f9062 35e3d1d 5d14c89." This matches size.log 11026 and 11070, and is checked at 13827.
  - The record's other "five" statements list all five commits: line 213, line 246, and line 144 (which names no list).
- **Checks.** The log ends "checks not as expected: 0" / "size log checks all as expected: yes" (13893-13894). No check line after the script ends ": no". The "inside its band: no" lines are indented `judged:` lines that go with the FINDINGs, and the "same section layout: no" lines are informational.
- **Installed record.** Its sha256 is f63dc6ac..., which matches my own sha256sum, and the log checks it byte for byte (13858-13859).
- **Spec.** PLANNING_SPEC is 866c16ce... in both logs, and my sha256sum of specs/0045 gives the same. The spec has 645 lines.
- **The FINDING lines it emits** (size.log 11195-11202):
  1. `FINDING: probe-emend-cells-write-xml: its growth over M6 part 6's record, -16 section bytes, shrank 16, a FINDING (expected 0 to 0, accepted up to 2000)`
  2. the same for probe-emend-styles-write-xml, -16
  3. probe-emend-formulas-write-xml, -16
  4. probe-emend-sheets-write-xml, -16
  5. probe-emend-rowcol-write-xml, -32 (shrank 32)
  6. probe-emend-richtext-write-xml, -16
  7. probe-emend-richcomment-write-xml, -16
  8. `FINDING: this part's core movement, the core's growth, -16 section bytes, shrank 16, a FINDING (expected 0 to 300, accepted up to 1500)`

  The printed record repeats all eight at 13618-13625, and the installed record at lines 199-206. These are ruling J's eight; there are no others.
- **Side note, outside ruling L.** The record's opening (lines 5-8) says the fixes change code under the `metadata` feature. Its attribution table (line 229) says the metadata cost's own files touched are "none", and that cost moved +16 only because the core shrank 16. This is template wording, not a number error.

### 3. Report numbers and quotes: **yes, with exceptions**

**These match the primaries:**
- The spec hash 866c... and its 645 lines.
- Both last lines, the negative end time 01:37:52Z, and the size start and end times 01:37:52Z and 01:43:25Z.
- "K1 to K5 ... each with its control", the description of K3's control, the R3-R5 contents, and "All yes".
- 451512, 7240, 458752, 451528 and -16, and sections -16 / file 0 / .text -16 (size.log 11067).
- The attribution quote, which matches record line 142 exactly, and its log line 11069.
- The cause-sentence quote, which matches record line 340 exactly.
- The 8 FINDING lines at 11195-11202, the repeat at 13618-13625, and the quoted core FINDING string.
- Negative FINDING lines 0.
- Changes NJ64 and SG.
- The report's check 1 text matches records/logs/m7p2-p1/independent-check-1.md, apart from that file's header line and its two `<home>` substitutions.

**Exceptions:**
- **i. Negative start time.** The report gives the driver's 01:37:28Z. The log's own start is 01:37:29Z (negative.log 105). Line 28 marks it as the driver's time, so this is not unmarked. But the report shows the log's own start beside the driver's for the gate and not for the negative step.
- **ii. Not in the three primaries and not marked as the session's:**
  - the script sha256s (line 70);
  - attempt 1's hashes (line 74);
  - "d6 ... read yes" (line 66).

  I checked the hashes myself: `<phase>/scripts/*.sh` match line 70, and `<phase>/logged/superseded/` negative.log, size.log and the record match 6727ff5b..., 627f5141... and 56c7ae20.... The logs README also carries them. I did not check d6.
- **iii. Line 33 overstates slightly.** "The refusing and accepting calls by their exact lines" does not hold for R3's add_chartsheet call or R5's `refused(` call (E2, E3 above).

No other disagreements.

### What I did beyond reading the five primaries
- I ran `git show 43471ed:` on chart_write.rs and chartsheet.rs to see the test bodies and the `call` helper.
- I ran sha256sum on the logs, the record, the spec and ROADMAP, on `<phase>/scripts/*.sh`, and on `<phase>/logged/superseded/*`.
- I diffed each log's printed script against its script file (they match apart from one trailing blank line) and the report's check 1 text against independent-check-1.md.
- I grepped the logs README for the hashes and the changes.
- I created, modified and deleted nothing, ran no cargo or script, and read nothing under <home>/emend-testing. The harness saved one oversized output (the record and report together) on its own, to <session>/tool-results/b779xdvdy.txt.
