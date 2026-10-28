# M7 part 2 P1, independent check 1 (its answer verbatim, but the home directory's path written <home>, twice, and the harness session directory under it written <session>, once; the answer as given is sha256 6fc7aedbe074d6bcfad96c046d67766b9933abb5acdaf7f0aafd12a0bd6c24d7 in <phase>/logged/check1-answer.md)

The draft report holds up on almost every number, but I found five disagreements. The most important: the negative log shows the refusals for fixes 3, 4 and 5 only as passing tests, and the report says every K section has a control when K3 has none.

## 1. Does the diff do what the spec says P1 does, and nothing else?

**Verdict: yes.**

**Doc commit bb7c35f.** Its parent is 658dbc0. It holds the three files at the hashes its message gives, the same hashes the logs README prints at lines 3 to 9. In ROADMAP.md, part 1's Close row reads DONE and its heading reads DONE and PUSHED. Part 2's P1 row reads PLANNED where the spec (line 334) said IN PROGRESS; ruling F (spec 426 to 429) accepts that slip.

**Fix 1, 4a2582b.** `split: None` now writes no `splitType` (chart_write.rs around line 2165); `Some` still writes `splitType` and `splitPos`. The two of-pie tests and the chart-surface form test in null_rewrite.rs are updated to match.

**Fix 2, f79e447.** In `rewrite_workbook_part`, the held bytes are now written before the created elements at both sites (package.rs about 14250 and 14404). The doc comments at 14080 and 14787 are amended as ruling E says. The new test `the_workbook_element_is_created_before_the_next_main_child_after_a_planted_one` plants `xr:revisionPtr`, then `mc:AlternateContent`, then both, and checks the created element comes after them and before `bookViews`. The calcPr and names tests are re-expected to match the new rule.

**Fix 3, 64f9062.** There is a new `SCATTER_STYLE_NONE` constant. It is checked straight after `plot_applies` for the first plot and after the combined plot's kind rules, which is what ruling D asks. The `add_chart` Errors list (two places), one catalogue entry and the style test (its `None` row dropped) are updated, and a new test covers both calls.

**Fix 4, 35e3d1d.** There is a new `CHARTSHEET_KEEPS_CHART` constant and an `is_chartsheet` helper. The change is limited to chartsheets in `record_removals`: the filter is under the `charts` feature and the refusal applies only when the part is a chartsheet. A removal plus an addition in one write is handed through a `LeftDrawings` value. The two tests are renamed to the names in ruling A.

**Fix 5, 5d14c89.** There is a new `MOVING_AVERAGE_PERIOD` check for periods outside 2 to 255; `Polynomial` keeps the generic message. The field doc now reads "2 to 255", and the three tests are adjusted.

**Docs, 43471ed.** README "Out of scope by design" gains the two new limits and the styles precondition sentence, which was absent at 658dbc0 (grep found nothing). CHANGELOG "Not supported by design" gains the two new limits.

**Nothing else.**
- The 13 files in `git diff --stat` are all accounted for above.
- There are no non-ASCII bytes in the diff or the commit messages.
- No commit adds `dbg!`, `println` or TODO lines.
- Each message has the count sentence and a single `Co-Authored-By` trailer.

**Minor wording point.** The README says "and the `cellXfs` entries"; the spec says "and `cellXfs` entries". Ruling G accepted the sentence as written.

## 2. Do the logs contain every line the spec requires of P1?

**Verdict: yes with exceptions.**

**What is present:**
- **Gate.**
  - Mount propagation shared and one keepalive (gate.log 17 to 31).
  - The prune: both targets listed with sizes and removed by `rm -r`, with its control (205 to 321).
  - The commit roles in order (370 to 460) and nine PASS lines (10162 to 10173).
  - The spec pin plus PLANNING_SPEC and PLANNING_ROADMAP checks (335 to 368).
- **Negative.**
  - K1: no `splitType` in pairs 1 and 10, and the `pos` form in pair 2 (negative.log 5600 area).
  - K2: the created element after a planted `xr:revisionPtr` and before `bookViews`, with a control using the old order.
  - K4: replacement in one write accepted, with a control.
  - K5: periods 2 and 255 written.
- **Run.**
  - Host and i686 test runs: 1452 passed, 0 failed, 2 ignored each (2827 to 2876).
  - All six reruns, each compared with part 1's record; formula and protect counts unchanged, as ruling G(a) requires.
  - The keep and the open record, compared by totals and by pair name: gone 7 and new 17, each listed by name, with the ruling A, H(1) and I(1) keys.
  - The trendline write test's .9 changed output, with 0 new keys (ruling H(3)/I).
  - Ruling C's 8 re-kept pairs and the 24 allowed pairs named.
  - 14 record files installed, their hashes matching what I computed.
- **Size.**
  - Budget 451512, 7240 under the ceiling, core movement -16, and the 8 FINDING lines (11180 to 11187).
  - Charts and metadata costs attributed by file (13392 to 13402).
- **Logs README.** Discloses the keepalive restart and the root mount, the fork's cargo runs, and d1 to d4 with changes NI64, TI, TJ and SF.
- **Scan pre-check.** RESULT: PASS (scan-pre-check.log 6713 to 6717), 15 records scanned.
- **FINDING lines.** Every emitted one (size 8; gate, negative and run 0) is listed in the report. In run.log the only other matches for the word are script text and the two "...: 0" summary lines.

**Exceptions:**
- **a. Refusals for fixes 3 to 5 are shown only by passing tests.** The spec (180 to 186) asks the negative log to show, among other things:
  - "`None` refused and `Marker` accepted";
  - "the chartsheet removal refused alone";
  - "the period 1, 256 and 65535 refused".

  No log line prints any of these:
  - **Fix 3.** K3 says only that refused calls kept pairs with no chart part. It says "the Marker cases, taken, write nothing".
  - **Fix 4.** The refused removal alone appears only as narrative ("its refused removal writes nothing"), with no check line.
  - **Fix 5.** No line names periods 1, 256 or 65535; it says only "the field-rules test kept 48 pair(s)".

  The evidence is the seven tests passing by exact name and the messages present in the source (M1 to M3).
- **b. K3 has no control line.** K1, K2, K4 and K5 each have one.
- **c. The size record does not carry ruling J's attribution.** Ruling J (spec 534 to 535) says the record must name "this attribution", meaning the shrink is put down to fix 2 by planning's reading. records/size-m7p2-2026-10-28-43471ed.md does not mention ruling J or fix 2. Around its line 337 it instead says: "5d14c89, 35e3d1d, 64f9062, f79e447, the part's five fix commits". That lists four commits while calling them five; size.log keeps the two lists separate. This may follow from ruling K's "scripts regenerate for this spec's hash and no other change", but it is still a gap against J. The report and the logs README do state the attribution.
- **d. Still pending, as expected.** The independent check and the final scan are not done yet; the README says scan.log is written last.

## 3. Does every number and quoted string in the draft report appear in a primary?

**Verdict: yes with exceptions.**

**Checked and matching:**
- The commits and subjects, and the PLANNING_SPEC and PLANNING_ROADMAP hashes (I computed both; the spec is 606 lines).
- The step end times, the prune byte counts and the emend-copy hash and size.
- 1452 / 0 / 2, "reruns done 6 of 6; record pairs written 6; FINDING lines so far: 0", and the keep's 2251 and 2251.
- The open record's 2251 / 2239 / 2240 / 0, and 2241 against 2251 with the quoted gone/new line.
- 6 of 6 re-kept and 24 allowed, and the 14 record files.
- 451512, 7240, 458752, 451528, -16, and the core movement of sections -16, file 0, .text -16.
- The 8 FINDING lines at size.log 11180 to 11187, repeated in the record at 13602 to 13609, the quoted core FINDING string, and the scan's four result lines.
- The five script hashes, which match `sha256sum` of `<phase>/scripts/*.sh` and README lines 123 to 127.
- The dry-run causes, which match the README.

**Exceptions:**
- **i. "each with its control"** is wrong for K3 (see 2b).
- **ii. Gate start time.** "2026-10-27T22:02:20Z" comes from `logged.out` and the README. The gate log's own line 442 says "run at 2026-10-27T22:02:22Z". This is a two-second difference between a driver time and a log time.
- **iii. "part 1's (M6 part 6's record) 451528".** Part 1 has no size record. The primaries call this figure M6 part 6's record (size.log 11036 to 11040), so the "part 1's" label is not from any primary.
- **iv. "d5 was clean except the scan pre-check's four expected dry-layout lines".** The README (175 to 179) and ruling K say d5's pre-check read "RESULT: FAIL" on four structural checks. "Clean except" makes that sound milder than it was.
- **v. Not in my listed primaries.** "Outer standard error is 0 bytes for each step" and the 22:02:20Z start come from `logged.out` and the empty `logged/*.outer.stderr` files. The README repeats both (lines 181 to 183), and the files agree.

## Anything done beyond reading

- I created, modified and deleted no files.
- I ran only read-only commands: git show, diff, log, sha256sum, grep, sed, wc, cmp and ls.
- The harness saved one oversized command output (the spec listing) on its own, to <session>/tool-results/bbkdi47wi.txt.
- I read nothing under <home>/emend-testing.
