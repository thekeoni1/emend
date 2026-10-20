Independent check, M6 part 2 P3 (specs/0039, diff bf16a46..50a3989)

Materials: spec-0039.md (sha256 681e4799..., the same as `git show 50a3989:specs/0039-...`), phase.diff, the five logs, scan-pre-check.log, README.md, the records and report-draft.md. Every copy in check1/ is byte-identical (cmp) to its file in the repository. I also ran read-only git: log, show, diff, numstat, cat-file, patch-id and reflog.

**1. Does the diff do what the spec and the brief say for P3, and nothing else? YES, with three notes.**

- **Doc commit 09e5f94.** Its subject is the brief's. It touches only specs/0039 (+69) and ROADMAP.md (1/1). At 50a3989 the blobs hash to 681e479969fc...fd9d1 and dc44a8680e6d...e9ff, the hashes in the commit message and the README. The ROADMAP change is the P2 row only (phase.diff, ROADMAP hunk); the P3 row stays PLANNED.
- **Test commit 905085e (ruling K).**
  - The bubble case now asserts `{point_written}<c:xVal>` once and `{point_written}<c:yVal>` once (chart_write.rs at 50a3989, around lines 4738-4777). The scatter and stock cases keep one pattern each. The ruling is named in the subject and the body.
  - BUBBLES (chart_write.rs 1672-1695): the second series has `categories: None`, so it has no xVal. Asserting "the dPt before xVal on both series" literally is impossible. The session's reading (dPt before each series' first reference) is the only workable one. Each pattern can match only its own series, so the case does assert both. I agree with the reading; planning is asked to confirm it (draft finding 2).
- **Probe commit 8b2320a.** PLOTS is now `[NewPlot; 16]`, with `Surface { wireframe: true }`, `Surface3D { wireframe: false }` and `OfPie { Bar, Some(Position(1)) }` after Area3D. That is the enum's declared order: chart_write.rs NewPlot, Area3D, then Surface, Surface3D, OfPie. The refusal now reads "0 to 15".
  - Surface series: no colour, no labels, no marker, line or points.
  - Of-pie: one series, round (no axis options).
  - Marker (Circle, 7, the series' colour) on Line, Scatter and Radar. PLOTS' radar is `RadarStyle::Marker`, and Line3D is not marked (decision 3 A).
  - Line (28575, Dash) on Line, Line3D, Scatter, Radar and Stock.
  - One point (index 0, D62728) on every non-surface kind: exploded 10 on Pie, Pie3D, Doughnut and OfPie, and given a marker on the marked kinds.
  - The chartsheet code is untouched.
  - This matches "The probe" and the P2 verification's kind table.
- **Changelog commit 50a3989.** One 4-line bullet under "### Added" (CHANGELOG.md heading at line 5; the hunk sits before "### Changed"). It cites specs/0039.
- **Rebuilt chain (draft finding 1).** In the reflog, 85971db, e3408a5 (fixup), 50de67f and cd9816c exist and were rebased. The probe and changelog commits have the same `git patch-id` as their earlier forms (7ef91f2d..., 8c9b7b06...), so they have no change of their own. Branch p2-before-reorder is gone (ruling J).
- **Nothing else** is in the diff: 5 files, numstat 4/0, 1/1, 21/10, 69/0, 103/16.

Notes, none a breach of the spec's text:

- **(a) The probe's option coverage.** The probe uses only `MarkerSymbol::Circle`, `DashStyle::Dash`, `OfPieKind::Bar` and `OfPieSplit::Position`, all from constants. The spec prescribes exactly these. But its "so every arm is live", and the probe header's line 39 claim that nothing can be proven dead, cannot hold for the other symbol, dash, kind and split arms: the optimiser may drop them. The estimate (15500 to 25500) priced eleven arms each for symbol and dash, three for the split and two for the type. The measured part cost is 5716, and making the options live in P3 added only 368 over P2's gate (gate.log 9230). The low figure may partly be an artefact of this; I have not measured that, and it is for planning to read.
- **(b) The combined line's series** (`COMBINED_SERIES`) takes no marker, line or point, against "each series gains". It goes through the same `series_options` / `series_work` (chart_write.rs 758-805), and the Line arm is live through kind 1, so no arm is lost.
- **(c) Stale wording.** The probe header (line 18) keeps "index 0 to 12 over the thirteen `NewPlot` variants" as the second set's paragraph, with the third set's paragraph added after it.

**2. Do the logs contain every line the spec requires for P3? YES**

- **gate.log**
  - 11: no prune (C72, C73).
  - 485-489: sixteen kinds and "0 to 15" at 50a3989; the base thirteen and "0 to 12", with controls.
  - 499-500: 154 messages, none added or gone.
  - 723-735: 83 base tests, one edited (ruling K), the two patterns.
  - 809: nine PASS lines.
  - 9224-9230: sections 1097236; 5716, 3336 and 368, printed.
- **test.log**
  - 154-196: both targets; ws 1366, off 515, charts 652, charts,images 694, charts,comments 697, charts,read 755, all 1134.
  - 215-241: difference 0 against P2.
  - 3661 and 3919: the formatter's line "rows 776; ... 1 gone, 20 new; ... Eq dropped 3".
- **size.log**
  - 9913-9915: core growth 0, "expected 0, accepted up to 2000: 0, as expected; a FINDING: no".
  - 9919-9922: budget 438960, 19792 under 458752, a move of 0.
  - 9923: 5a, 1097236 - 972176 = 125060 against 119344, a move of 5716.
  - 9924: 5a', against the estimate and the 30000 line, "a FINDING: no".
  - 9925-9926: images 57504 and comments 63292, each moved 0, the probes' growth printed. The bands judge them with "accepted up to 1500 either way" (raw size.log near the judged lines). The images and comments probes are "printed, not judged" (ruling N a).
  - 12002-12005: index 16 refused.
  - 12032-12037: indexes 13-15 exit 0 with 31 lines.
  - QF lines 9858-9868, with a control.
- **negative.log:** N1 to N11 exit 2, no stdout, the exact message, no output file (686-707). CHART_VALUE_RANGE and CHART_OPTION_KIND are shown unreachable for P1's and P2's rules (ND44, ND45), with the planted copies and the covering tests named. ND46 states that P3 adds no message, case or path, with a diff check and a control.
- **run.log**
  - 181-204: the size log's four release binaries copied and hash-checked (4 of 4).
  - Corpus rows 27, ok 27, lo and lo_in 27, on both samples and both targets.
  - 478-505: byte-identical to P2, 27 of 27 each, 4 of 4.
  - 519: the image id.
  - Fresh fixture: lines 4 of 4 and fields nine of nine.
  - P2's kept fixture: 4 of 4 by sha256 (ruling P's form).
  - The keep: 1748 pairs, 3496 files, 0 new, 0 gone, 0 changed. The open check: crate 1614, tool 134, no output refused.
- **Records:** the API record marks NewChart, NewCombined and NewPlot "keep; Eq dropped (ruling E (b))" with both derive lines. The size record has the cost, part 1's figure and the delta, and its copy paragraph cites its band (ruling N b).
- **Scan:** scan.log is not yet present (it is written last). The pre-check ends "structural checks failed: 0" and "RESULT: PASS".

One defect in a record, not in a log line: records/size-m6p2-2026-10-20-50a3989.md line 85 (size.log 12179) describes the charts probe as "index 0 to 12 over the thirteen `NewPlot` variants ... two series each with a colour and value labels", then adds "the kind index runs 0 to 15". The row contradicts itself. Cosmetic, but it is in a primary.

**3. Does every number and quoted string in the draft appear in a primary? YES, with one wording point.**

- **Hashes:**
  - Full commit hashes: from git.
  - Doc-commit blob hashes, all log and record sha256s and the scan pre-check's 0cefb600...: recomputed, all match.
  - Release-binary hashes: run.log 193-202.
  - 894b239f...: run.log 519.
  - 86978c7a...: run.log, P2's fixture section.
- **Numstat:** matches `git diff --numstat`.
- **Times:** the five "run ended at" times match each log's tail.
- **Counts:**
  - 83 and 154: gate.log 723 and 499.
  - 1097236, 1091520, 5716, 1093900, 3336, 1096868 and 368: gate.log 9205-9230.
  - The test counts: test.log 154-241.
  - The quoted formatter line: test.log 3919.
  - 776, 1 gone, 20 new, 15 names: the API record lines 14-50, and the use row lists 15 names.
- **Dates and negative cases:** 784e3c0 and 4c36f05 are in the API record lines 26-28. N1-N11, ND44, ND45 and ND46 are in negative.log.
- **Size figures and quotes:** 125060, 972176, 119344, 57504, 63292, 0, 438960, 19792 and 458752, and the quotes "below the estimate of 15500 to 25500; a FINDING: no" and "expected 0, accepted up to 2000: 0, as expected; a FINDING: no", are in size.log 9913-9924. "31 lines, the first 29 index 0's" is at size.log 12033-12037.
- **Run-log figures:** 27, 4 of 4, nine of nine, 1748, 3496, 1614 and 134 are in run.log. `tmp.9cLZvfOBZq` is at run.log 6787.
- **Old-chain hashes:** 85971db, e3408a5, 50de67f and cd9816c are in git (reflog and cat-file).
- **Wording point:** for the three heuristically dated API rows, the draft says "dated by the listing's heuristic ... the record says so". The record (line 24) gives two possible causes and "does not say which". The README's statement 5 says that correctly. The heuristic is in fact the only possible cause, since 784e3c0 and 4c36f05 predate NewMarker and NewPoint (1637495). Even so, the draft's sentence should say the record leaves the cause open. Also not mentioned in the draft: those rows' `spec` column reads specs/0035 or specs/0036 for items that are specs/0039's (API record table rows 555, 556 and 564).
- **Session statements:** the dry run d1, stderr 0 bytes and "no log takes PLANNING_SPEC" are the session's statements; the README labels them so, and no log prints them.
