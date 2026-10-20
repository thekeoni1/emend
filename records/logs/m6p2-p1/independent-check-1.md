# M6 part 2 P1: independent check 1

A fresh subagent given only the spec (planning's working copy), the phase
diff, the draft report, the four logs, the scan pre-check, the logs'
README and the records, in `<phase>/check1`. Its answer, verbatim
(extracted from its transcript, sha256
2b6de19a150e07f315c0829166d2eb56284093c0fa3e40896e88f2c9d4e719c7):

---

1. Does the diff do what the spec says for P1, including rulings A to C, and nothing else?

PARTLY. The core work and the ruling A fix match the spec. The gaps are a few spec P1 items that are missing or narrower than written, and one public API change the spec does not name.

What matches, checked in the diff and in git at 01a3059:
- The doc commit 90ad3a1 holds the three planning files at the hashes its message gives: specs/0039 f5cab2f6..., specs/0038 9822c6dc..., ROADMAP 1511c95a.... I checked them with `git show | sha256sum`.
- The phase's own commits are doc, crate, tool and fix, in that order on base 6cecae7. Every message is ASCII and ends with the Co-Authored-By line alone, with no Claude-Session line.
- The diff matches its file (phase.diff sha256 e9df5c97..., the same as the gate.log line "the phase's combined diff").
- chart_write.rs (phase.diff lines 78 to 416):
  - `NewPlot::Surface`, `Surface3D` and `OfPie`, with `OfPieKind` and `OfPieSplit`, which has three forms and no custom split (decision 2 A).
  - `plot_applies`: a surface's series take no colour and no labels; an of-pie takes a pie's rules.
  - `takes_position`: the of-pie takes the pie's four positions.
  - `series_axis` covers both surfaces.
  - The contour's `view3D` is 90/0/0 with `perspective` 0; the 3D surface's is 15/20/1.
  - Surfaces write no `varyColors`.
  - The of-pie writes `ofPieType` first, `varyColors` 1, the series, `gapWidth` 150, `splitType`, `splitPos` when a split is given, `secondPieSize` 75 and `serLines`, and no axIds.
  - `CHART_VALUE_RANGE` is checked after `plot_applies` and before the combined plot. This is the order "Applicability" and "The chart part" give (spec lines 222 to 235 and 263 to 293).
- Ruling A (spec lines 554 to 566) is commit 01a3059 with the exact subject the ruling gives. It changes only the surface sample: `SAMPLE_LAST_ROW` 1001, `last = max_row.max(1)+1` capped at 1001, the chart anchors unchanged, `rows` added to the line, and a third tool test (M = 1500, rows=1000, anchor row 1501). `git show 01a3059 --stat` touches only tools/emend-copy main.rs and null_rewrite.rs.
- Ruling B: every logged LibreOffice run uses `--lo-timeout 600` (run.log, the commands at 5405, 5484 and in the fixture section).
- Ruling C bears on P2 only, and nothing for it appears in the diff.

Disagreements and gaps:
a. Spec line 456 asks for "a surface series with a colour, labels, a marker, a line or a point refused". Only colour and labels are tested (`a_surface_series_with_a_colour_or_labels_is_refused`), because `NewMarker`, `NewLine` and `NewPoint` arrive in P2. Commit 3affd4a's message says so ("the marker, line and point fields arrive in P2"). The report does not.
b. The spec does not name this API change: `NewChart`, `NewCombined` and `NewPlot` lose `derive(Eq)` (phase.diff, the chart_write.rs hunks at old lines 76, 108 and 125). That is a public trait removal, forced by the `f64` in `OfPieSplit`. Commit 3affd4a's message discloses it; the report does not. Spec line 149 expects the API listing to mark changed rows. That listing is P3's, but the report should name this change now.
c. Spec line 452 says "each written chart read back by charts and compared". Three places fall short:
   - `an_of_pie_takes_a_colour_the_percent_and_a_pie_s_positions` checks the coloured of-pie's bytes with `contains` but does not read it back.
   - The one-series surface with axis formats in the contour test only gets `call(...).expect("taken")`; it is never written or read.
   - The chartsheet surface test compares the plot with `contains`, not the whole part.
d. The `package` header catalogue entry for the new message (package.rs, phase.diff lines 47 to 77) arrives in P1, while spec line 476 lists "the `package` header's catalogue and limits for this part" under P2. "Errors" (spec line 340) arguably allows it. It is a small step ahead of P2's scope.
e. Cosmetic: some doc-comment lines run past the file's 100-column width. One is in `add_chart`'s Errors paragraph ("chart option does not apply to its kind", then, for a combined plot a first plot ...). The others are two lines of the `--chart-surface-sample` header in main.rs edited by 01a3059. Also, the `add_chart` paragraph now names "a chart option does not apply to its kind" twice in a way that reads awkwardly.
f. The crate commit 3affd4a also edits tools/emend-copy/src/main.rs (+25, `chart_as_read`'s new arms). The match needs them to compile, and the message says so. I note it only because the report's role table implies the tool commit holds all the tool changes.

2. Do the logs contain every line the spec requires?

PARTLY. Gate, test and negative are complete. Two run-log and record items are short.

- gate.log: it prunes all four M6 part 1 roots (m6p1-p1, m6p1-p2, m6p1-p3, m6p1-close), each listed with `du -sb` and removed with `rm -r` without -f (lines 218 to 407). The probe is printed and not judged (8892 to 8896). The chart-write count is 74 with 4 gated to images (8287 to 8290). It ends "gate log checks all as expected: yes" (9131). That it matches "part 1 P1's gate" in full I could not check from these files alone.
- test.log: both targets, each of the six feature sets plus the workspace (1708 to 1804). The figures against the previous log are at 1824 to 1850, host equals i686 at 1880 to 1886, and the 8 crate tests plus 3 tool tests passed by name (1892 onward). Complete.
- negative.log:
  - the new message read from source and found in the catalogue (5579 to 5639);
  - the reason it cannot be reached through the binary, with its covering tests named (5991);
  - N10 and its control B5 (6632 to 6686);
  - "the 10 cases were refused, N1 to N10" (6698).
  Complete for spec line 461.
- run.log:
  - the corpus run on both targets, 27 of 27 with lo and lo_in ok (5405 to 5590);
  - ruling A's rows checks (5576 to 5581);
  - the fixture generator printed (from 5631), the ElementTree check (5916 to 5946), lo and lo_in ok on every fixture output;
  - the keep and the open record (5954 to 8040).
- Gap A: spec lines 383 and 384 say `--charts-sample` on the fixture "reads both charts (count 2, series 3, the kinds `Surface3D` and `OfPie`)". The log shows charts=2, series=3 and kind_other=2, and its check line reads "a 3D surface and an of-pie, both kind_other". The binary's line cannot tell Surface3D from OfPie, and no log line shows emend's reader naming those two kinds on the fixture. Only the Python ElementTree check identifies them, and it checks the XML, not the reader.
- Gap B, records:
  - Ruling A says the tool's `rows` field is there "so the record shows where the bound applied". records/m6p2-p1-2026-10-19-01a3059.md and .txt contain no `rows=` and no tool line (`grep -c 'rows='` gives 0 in both). Only run.log shows rows=1000 for the two large workbooks.
  - Ruling B wants "the P1 record's findings paragraph" to cite dry run 2 by sha256, the two rows, the cause and the fix commit. Neither record file has a findings paragraph. The citation exists only in README.md statement 2 and in report-draft.md finding 1. That counts only if "the P1 record" means the README in the record commit.
- Not yet checkable: ruling B's rule F says RERUN_FILES must name the superseded run script. The final scan has not run; scan-pre-check.log line 6 has RERUN_FILES unset. Its line 101 ("every file RERUN_FILES names lies in <phase>/superseded: yes") is met trivially. scan.log does not exist yet; README marks it "written last".

3. Does every number and quoted string in report-draft.md and README.md appear in a primary?

PARTLY. The great majority check out; the items listed below do not.

Verified against primaries:
- The sha256 values of all four logs, the four records and the scan pre-check, which match the files and records/ in the repo.
- The working spec at 7dff6b25...: the gate (12), test (8) and run (13) logs print it as PLANNING_SPEC. The negative log prints the same hash at line 142, as the working file's hash, not as an input.
- All four commit hashes and subjects, and the base subject, from git.
- The numstat table, which matches `git diff --numstat 6cecae7 01a3059` row for row.
- The four end times, from each log's "run ended at" line.
- The prune byte counts 12080832921, 5737364002, 7801598890 and 2018655097.
- 74 chart-write tests, 4 gated to images.
- The probe figures 1093900, 1091520/2380, 1075632/18268 and 1055368/38532.
- The test totals 1354/1343/11, 515, 643, 685, 688, 746 and 1125, each with its difference.
- N1 to N10, N10's exit 2, no stdout and no output file, and B5.
- Position(2).
- The corpus figures 27/27/27/27 and the entry totals 798/0/72/0/151/43.
- The two ids at rows=1000.
- The fixture line, rows=4.
- The keep: 1565 pairs, 3130 files, 91 new, 0 gone or changed.
- The open record: 1553 ok and 12 fail for lo_in, 1554 ok and 11 fail for lo, 0 errors, findings 0.
- RESULT: PASS with "structural checks failed: 0".
- The spec figures: rows `$2:$130002`, 130001 points or slices, 600 s, 25 of 27, and the dry-run-2/d2 path.

Not in any primary (session statements, or forward claims):
- sha256 325bc0a9... (the dry run 2 run log), 08a2bffa... (the gen1 run script), a98909dd... (the dry gate 1 log) and 89cf7b8f... (its script). No log, record, the spec or git contains them. I did not hash the files under `<phase>/superseded`, since those are outside the files given.
- `<phase>/superseded/gen1/scripts` and `<phase>/superseded/dry-gate-1`: no primary names either path. The spec names only `<phase>/superseded/dry-run-2/d2`.
- "lo error" as the class of the two rows in dry run 2. The spec says only "time out".
- That dry gate 1 and dry run 2 ran "at 0a55f0b"; that dry run 3 ran "at 01a3059" and "ended yes for all four logs"; and "no script was regenerated after it".
- "Planning accepted I71" (README 157) and "change I71, accepted" (report 110). The spec's rulings A to C do not mention I71. gate.log names I71 only as a change number in its own check lines.
- "sixty-seven" appears only in a script comment in gate.log (line 6477), not in an output line.
- "named by the final scan's RERUN_FILES (rule F)" (report 108 and 109) describes a scan that has not run.
- "the brief named" (report 116): the brief is not a primary.
- "outer standard error 0 bytes". The logs show only each script's own stderr file at 0 bytes. The report marks this as the session's statement, but README 139 and 140 state it as fact without that hedge.
- "Nothing pushed" can be checked against git only partly. The status shows the four phase commits on local main with the logs untracked.

Statements that contradict or overstate a primary:
- README 123 and 124: "which every log was run against (PLANNING_SPEC)". The negative log takes no PLANNING_SPEC input; it admits the modified spec by its path pattern (negative.log lines 1 to 13, 142 and 6748). The report's own caveat (lines 7 and 8) is correct, and the README's line is not.
- Report 10: "Every committed log ends ..." The logs are not committed yet (git status: `?? records/logs/m6p2-p1/`).
- README 175 and 176 and report 114: "the gen1 generators and scripts kept in `<phase>/superseded/gen1`". gate.log line 188 hashes `<phase>/gen/gate.py.gen1` (90935cdf..., 72122 bytes), a gen1 generator still in the live generator directory. That does not disprove the statement, but the report should say why a gen1 file sits in gen/.
- README 178: "the line's ten fields". The run log lists nine keys (line 5358); ten is right only if the word `chartsurface` counts as a field.

The report and README quote no workbook cell contents, and contain no non-ASCII bytes and no U+2014.
