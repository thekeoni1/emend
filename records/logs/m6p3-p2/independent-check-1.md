# Independent check: M6 part 3 P2 (specs/0040), draft report and logs

I read only the files in the check1 directory: spec-0040-working.md, diff.patch, commit-messages.txt, report-draft.md, logs/ and records/. I ran no build, cargo, git or container command. Line numbers refer to those files.

## Hashes checked first

- diff.patch has sha256 9e2be037f75490df1dd7d1f8182e46c047f7352c6d7e4bf6b618f19cc53fdfa4. That is the phase's combined-diff hash printed in gate.log, so diff.patch is the gated diff.
- spec-0040-working.md has sha256 b36f6165...e51e8, which is PLANNING_SPEC.
- All four logs, scan-pre.log and the six records hash to the values given in report-draft.md lines 85-99 and logs/README.md lines 31-34.
- Per-file added and deleted counts, computed from diff.patch, equal the report's numstat table (report lines 30-40) for all 11 files.

## Question 1: Does the diff do what the spec says for P2, as amended by rulings J to P, and nothing else?

**Answer: PARTLY.** The code, tests, tool, op and doc commit match the spec and rulings, with one stale doc paragraph and one rule the spec does not name.

### What matches (with evidence)

- **NewChart and the new types.**
  - NewChart gains drop_lines, high_low_lines, up_down_bars, category_axis and value_axis (diff 119-134).
  - NewAxis, NewDateAxis and TimeUnit have the fields and derives "The call" gives (diff 137-187).
  - They are re-exported from package (diff 65-70).
  - NewCombined is untouched.
- **Plot-line rules (plot_lines, diff 262-279).**
  - Drop lines are allowed on Line, Area and Stock.
  - High-low lines and up-down bars are allowed on Line and Stock.
  - A stock plot with high_low_lines false is refused.
- **Axis rules (axis_options, diff 293-372).** Follows "Applicability" with rulings J and K:
  - The round kinds are refused first.
  - The axis form comes next: date only on the category axis of Bar, Line, Area or Stock.
  - Then each field in declaration order, its option-kind check before its range check.
  - Min below max is judged at max.
  - Units are refused on a date axis (ruling J).
  - The date counts are checked last.
  - Check order: after series_options and before the combined rules (diff 228-232).
- **Forms.**
  - The plot lines come after the series and before a line plot's marker and the axIds. Stock's old fixed hiLowLines arm is removed, so its bytes are unchanged when high_low_lines is true (diff 492-504, 512).
  - Scaling order is logBase, orientation, max, min (diff 548-567).
  - majorUnit and minorUnit come after crossBetween; tickLblSkip and tickMarkSkip come after lblOffset and before noMultiLvlLbl (diff 571-605).
  - dateAx is followed by auto, lblOffset, baseTimeUnit, then the unit and time-unit pairs (diff 606-620, 735-741).
  - Numbers are written with f64 to_string (diff 545).
- **Ruling L.** Every NewChart literal gains the five fields, with `plot == NewPlot::Stock` where the plot is a variable. gate.log 7870-7880 shows the 50 main.rs lines; diff 2408-2435 shows the probe's 10 lines. P2 makes no probe option change, which is correct because that is P3's work.
- **Rulings C and D.** Ruling C's main.rs doc line is fixed (diff 1716-1717). Ruling D's Errors doc is rewrapped (diff 196-218).
- **Package header limits paragraph.** Done (diff 25-60). It adds no catalogue entry: negative.log 6997 shows list items 166 and 166.
- **Tool and ruling M.** `--chart-axis-sample` builds the three charts the spec describes. Its line keys and order follow ruling M, and it asserts bounded_axes 2 (diff 1925-2160).
- **Ruling P.** In 3736eea the Bar's series takes `categories: None` (diff 2030-2033), and the tool test asserts no cat or xVal in chart3 (diff 2372-2378).
- **Op and ruling O.**
  - The op is as "Harness runs" describes (diff 1500-1568).
  - The date column is max_column + 1, starting at ANALYTIC_DATE_FIRST 45292 and covering rows 2 to M+1 (diff 1550-1553).
  - The first LineChart keeps A2:A(M+1).
  - The op test asserts $Y$2:$Y$101 and 15 checks (diff 1440-1452, 1457-1458).
- **Doc commit (ruling I).** It carries specs/0040 and ROADMAP.md at the hashes in its message (gate.log 7616-7665). The ROADMAP change is planning's single P1 row.
- **Scope.** There are 7 new crate tests and 3 tool tests, and the tests cover each P2 item. No manifest or lock file changes and no new message (gate.log 7824-7900).

### Disagreements

1. **Stale module doc (main.rs, diff 1726-1735).** The `--chart-axis-sample` module doc still says all three charts are "each of one series over B named by B1 against A2:A(L)", which includes the column chart. Since 3736eea and ruling P, the column's series takes no categories. The function doc was updated ("a column chart without categories", diff 1922), and so was the tool test comment (diff 2201-2202), but the module doc was not. 3736eea's message (commit-messages.txt 10-15) does not mention the module doc. This is wording only (no code changes), but the doc disagrees with ruling P's amended "Harness runs" text (spec 752-756).
2. **A rule beyond the spec (diff 316 and 324).** `min` and `max` are refused with CHART_VALUE_RANGE when not finite. The spec (Applicability, spec 257-260) names "min below max when both are given" but no finiteness rule. The report (line 52 and item 7) and the README (item 8) disclose it as the session's choice. It is still an addition planning has not ruled on.
3. **Observation, not a defect.** Ruling L says "the gate names the base tests edited by their field lines". gate.log 8467-8468 prints "edit lines: 0; base tests edited or gone: 0". That is consistent with the diff: the field lines landed in the helper functions column_chart, bare and titled (diff 655-680), not in #[test] bodies. So the gate names none, and it does not name the edited helpers either.

## Question 2: Do the logs contain every line the spec and rulings require?

**Answer: PARTLY.** Everything required at this stage is present. The final scan.log does not exist yet, and one README statement is weaker than ruling P's wording.

### Present (with evidence)

- **Logs.**
  - gate.log, test.log, negative.log and run.log each end "<name> log checks all as expected: yes".
  - scan-pre.log ends with "structural checks failed: 0" and "RESULT: PASS".
  - ANTHROPIC_API_KEY is unset in every log: gate 6 and 347, test 3, negative 9 and 130, run 20 and 186, scan-pre 14 and 104.
- **Ruling I.**
  - The gate prunes nothing (gate.log 11).
  - The probe figure is printed and judged nowhere: 1107076, which is 4012 over P1's 1103064 and 9840 over 1097236 (gate.log section 509ff).
- **test.log.**
  - Both targets, the workspace and the six feature sets, host equal to i686 (test.log 2157-2278).
  - The 7 and 3 new tests pass by name (2362ff).
  - The ops test in the pinned image passes, including the chart-analytic op's six lines, the $Y$2:$Y$101 check and "made its 15 checks" (2557-2660).
- **negative.log.**
  - N1 to N13 are refused through the binary. This phase's N13 has DRAWING_UNREADABLE, with control B8 printing the tool test's line (8836-8848, 8879-8904, 8935-8937).
  - The plot-line and axis rules are named as unreachable through the binary, with their covering tests (8090-8096, 7763-7785).
- **run.log, corpus runs.**
  - The axis sample on 27 of 27 workbooks per target, lo and lo_in ok 27 each.
  - 81 chart parts added (records chartaxis .md, chart row).
  - Every line as ruling M gives it; the same output on both targets (6420-6604).
  - P1's trend run rerun byte-identical by sha256, 27 of 27 per target (6868-6970).
- **run.log, openpyxl fixture.**
  - Written by the pinned image, id 894b239f...
  - The `--charts-sample` line has axes=4, axis_other=1 and axis_bounds=1 (7100).
  - The axis line shows charts_after=5 (7121-7122).
  - lo and lo_in are ok on all six fixture runs.
- **Ruling N.** All twelve fields are printed with both values and both preceding siblings. The equality-required constants are checked. The three uncompared forms are named. "fields compared 12; the same form 12" is followed by sixteen planted controls (run.log 7154ff; the fieldcmp output is at relative lines 267-302).
- **Keep and open record.**
  - The keep is 2121 pairs, of which 225 are new and named, 0 gone and 0 changed (7459-7530).
  - The open record has 2121 pairs, no verdict or class moved, and no output refused on an accepted input (10201-10442).
- **README.**
  - It names F1 and F2 as LibreOffice facts, with the bisection spans and the source of the earlier figure (rulings O and P).
  - It states the rebuild from 9a6a220 to d367326 (ruling N).
  - It states the forks' rm uses (ruling G).

### Gaps

1. **No final scan.log.** The spec's "Phases" (spec 423-427) lists scan.log among the logs, and specs/0001 has it written last. Only the pre-check is in the bundle; README 35 marks the scan as "written last". The final scan cannot be verified until it exists.
2. **Ruling P's README statement.** Ruling P (spec 760-762) asks the README to state that LibreOffice opens the sample's output on the fixture, "lo-check rc 0". README 79-86 reports "outcome ok, lo ok, lo_in ok" through passthrough-run.sh, whose open check is lo-check.sh's. That is equivalent in effect, but no rc 0 is stated. No log prints this hand measurement; 3736eea's message states only that LibreOffice opens the output.
3. **Cosmetic log label.** test.log 2672 prints "of them this phase's logs: 1", but the one status line is the modified specs/0040, which the pattern admits under PLANNING_SPEC (test.log 1157). The check is right; the label is wrong.

## Question 3: Does every number and quoted string in report-draft.md and logs/README.md appear in a primary?

**Answer: PARTLY.** Almost every number matches a primary. One number is wrong, one claim is inaccurate, and several statements appear in no primary and are not marked as the session's.

### Matching a primary

- Every commit hash and subject in both tables (gate.log 272-310, 7537-7546).
- Spec hashes b36f... and f9d63..., and ROADMAP 33c2b578...
- All log, record and scan-pre hashes, and the four "run ended at" times.
- 154 messages; 11 files, none added; 50 and 0; ten probe lines; 257 base tests (90, 20, 147); 1379 and 1389; 10 new tests (7 and 3); nine PASS lines.
- 1107076, 1103064, 4012, 1097236, 9840.
- Test counts 1386, 1376, 515, 666, 708, 711, 769, 1148; 97 ok lines and six op lines.
- N1 to N13, DRAWING_UNREADABLE, B8.
- Corpus figures: 27 of 27; lo and lo_in 27; identical 798, changed 72, added 124, dropped 0, recompressed 0; the two ids at 1000 rows.
- Fixture figures: 30072 bytes, 15 entries, eight checks; the axis and trend lines verbatim; axes 4, axis_other 1, axis_bounds 1; 12 of 12.
- Keep and open record: 2121, 4242, 1896, 225, 1979, 142.
- F1 figures from ruling O: 99 cells, 116233 to 42886443, 42770210 days (the subtraction checks), 13M, 1M and 100k spans, past 9999-12-31, exit 134, std::bad_alloc, ANALYTIC_DATE_FIRST 45292.
- F2's $A$2:$A$102 (ruling P).
- README labels: A77 is in gate.log, WO in test.log, NA50 in negative.log, NYO in run.log, JY in scan-pre.log. K75, ND48, NYN and f9d63076 are also in the logs.
- The README's "58 planted copies" equals 5+8+11+15+19 in negative.log (ND43 to ND48).
- The report's numstat table matches diff.patch.

### Disagreements and unsupported items

1. **Wrong number.** Report line 125 says "(each +7 over P1's)" after a list that includes "features off 515". test.log 2296 shows features off at 515 against P1's 515, a difference of 0. Only the five charts sets are +7; the workspace is +10.
2. **Inaccurate claim.** Report line 9 says "Every committed log ends ...". At the primaries' time the logs are not committed: there is no record commit in commit-messages.txt, and scan-pre.log 5470 shows "?? records/logs/m6p3-p2/". They should be called the logs, not the committed logs.
3. **Hash in no primary.** Report item 4 (line 182) gives "op 4454629". That hash appears in no primary: not in the spec's ruling N, not in the README, not in any log. Item 4 is marked as the session's statement, so this is not counted as a failure, but nothing supports the hash.
4. **README statements no log prints, not marked as the session's.** They sit outside the README's "session's statements" section:
   - "Each log is the first logged run of its kind; no logged attempt was refused or stopped" (README 42-43).
   - "the phase's commits are of 2026-10-20" (README 50). No primary prints a commit date.
   - Parts of the F1 bisection: "the DateAxis chart alone aborts it, with or without its units and its logBase; the first chart ... and logBase alone open" (README 61-64), and "the first three and the last two of A1:A100" (README 71). The spec gives the spans only.
   - The F2 hand measurement's details (README 79-86, and report lines 77-79).

   None of these contradicts a primary. The F2 line it quotes is the logged fixture axis line. They are still the session's own statements.
5. **Statements correctly marked as the session's, no contradiction found:**
   - Report line 3 (nothing pushed).
   - Report lines 11-13 (the dry runs).
   - Report lines 96-98 (outer standard error). The logs' start and end times confirm the serial order: gate 04:07:07 to 04:13:24, test 04:13:24 to 04:15:21, negative 04:15:22 to 04:15:33, run 04:15:33 to 06:46:08.
   - Report items 1 to 9 and README items 1 to 9.
   - The rebuild 9a6a220 to d367326 agrees with spec ruling N.
   - Item 8's rows 101, and 100 at d1192d2, are consistent with ruling P and with the tool's rows = max_row behaviour shown by the tool tests.
6. **Wording only.** README 72 says "Ruling O (option a)". Ruling O itself only refuses option (b) and describes the fix. "Option (a)" is ruling P's wording (spec 752).