Independent check 1

Inputs read: everything under <phase>/check1/. I modified nothing and ran no cargo, git or build commands.

## 1. Does the diff do what the spec says for P1 (and rulings A to C), and nothing else?

**Verdict: YES WITH NOTES**

- **Files.** diff.patch touches 9 files: the three doc files and six code files, with no Cargo.lock and no manifest. gate.log:6155 reads "Cargo.lock in the phase diff: 0; manifests in the phase diff: 0".
- **Compaction is first and touches no test file.** gate.log:6179-6180 reads "test files touched: by the compaction commit 0" and "the compaction commit touches no test file ...: as-expected". Commit 5d8ce24's message describes value_element, tag, the single plot walk, axis_bytes and element_bytes. diff.patch:507-610 shows these helpers. The old catAx, valAx, pie, bar, line, scatter and area literals rebuild to the same byte sequence; I traced each one.
- **Probe doc comment (ruling R).** diff.patch:2441-2452 states that the chart's references and texts are the probe's own constants, citing ruling R.
- **New kinds.** `Doughnut { hole_size: u8 }`, `Radar { style: RadarStyle }`, `Bubble` and `Stock` are at diff.patch:103-125, and `pub enum RadarStyle { Standard, Marker, Filled }` at 126-135. `NewSeries::sizes` is at 146-149.
- **Emitted forms match spec-working.md "The chart part".** I checked each against diff.patch:424-564:
  - doughnut: varyColors 1, then firstSliceAng, then holeSize, with no axId;
  - radar: radarStyle comes before varyColors 0;
  - bubble: xVal, yVal and bubbleSize as numRef, then bubbleScale 100 and showNegBubbles 0;
  - stock: no varyColors, a `<c:marker><c:symbol val="none"/></c:marker>` on each series, and hiLowLines;
  - axes: radar and stock use a line's axes, bubble uses a scatter's.
- **Rules.** diff.patch:295-313 refuses with CHART_OPTION_KIND: a hole outside 1..=90 or with an axis title, stock with other than 3..=4 series, and sizes present exactly when the plot is a bubble. Sizes pass through `reference` after values (diff.patch:330), so they meet CHART_TEXT_INVALID.
- **Ruling B (b).** The re-export changes one line, at diff.patch:60-61 and gate.log:6175.
- **Ruling B (a).** The `sizes: None,` edits are at diff.patch:634-670. gate.log:6684-6695 names one base test body edit, texts_are_escaped_and_references_written_as_given.
- **`--chart-more-sample`.** diff.patch:2036-2190 matches "Harness runs": four plots, anchors M+2+16k, 480x288, "emend more k", legend Right, no axis titles on the doughnut, bubble sizes over C, stock high/low/close over B/C/D, and the four asserted counts.
- **Tests.** The tests match P1's list, minus "a colour on a stock series", which ruling B moved to P3.
- **Ruling C.** gate.log:6005-6008 and scan-pre-check.log:125-127 show it applied.

Notes:
- (a) spec-working.md P1 says "the four part 4 samples and the three image samples", while "Harness runs" names two part 4 samples (five records). The run follows "Harness runs". This inconsistency is in planning's spec, not in the work.
- (b) The compaction keeps literal forms for delete, crosses, the tick marks and others, which the spec listed for value_element. Ruling A accepts the compaction as it stands.

## 2. Do the logs contain every line the spec requires for P1?

**Verdict: YES**

- **Prunes.** Each of the four roots shows `du -sb`, then `rm -r` with "(no -f)", then "every name of the prune set is gone: as-expected" (gate.log:110-115, 173-178, 229-234, 277-282). The byte counts are 11176508696, 5620278808, 7100254922 and 1964236421.
- **Probe.** gate.log:7244 reads "file 1059364 bytes; sections 1055684". gate.log:7246 reads "... part 4's: 1055368; the difference ...: 316; told to planning, judged nowhere".
- **Ruling T.** gate.log:6943-6956 reads the figures from part 4 P3's committed test.log (515/578/551/616/681/1055). Host and i686 then report 515, 585, 551, 623, 688 and 1062 (gate.log:6963-7199).
- **Six feature sets on both targets.** In test.log, see 3193 onward for E, and 3024, 3046 and 3258-3260 for W: 1279 = 1269 + 10. Plants P1 to P14 each read "yes", with an unplanted control (3474).
- **Negative.** negative.log:4223 reads "N3 ...: exit 2, no stdout, the error line exactly the TWO_DRAWINGS message: yes". Line 4224 reads "N3: no output file: yes", and 4218 gives control B1. Lines 3941-3946 state part 1's three messages unreachable, covered by a_chart_on_a_removed_replaced_or_added_part_is_refused.
- **Run.**
  - Ok rows show "lo ok 27; lo_in ok 27" on both labels (run.log:4417, 4496).
  - Host and i686 outputs are identical for 27 of 27 (4556).
  - The five reruns, on both targets, each read "byte-identical ... 27 of 27 ...: yes" (4676-5334). Their records are named at 66-100.
  - The keep shows 23/0/0 (5391, 5415, 5419), and the open check reads "no verdict and no class ... moved: yes" (6911).
  - Both record pairs were written and installed (4563, 4597, 6969-6970).
- **Endings.** Each of the four logs ends "<name> log checks all as expected: yes".

## 3. Does every number and quoted string in report-draft.md appear in a primary?

**Verdict: YES WITH NOTES**

- **Hashes.** I recomputed every sha256, and all match the report:
  - gate.log d423bd45...
  - test.log 2ab6fe6d...
  - negative.log e13d04db...
  - run.log 00e490b3...
  - the four records faef6e01..., 832aa53f..., 218b7050... and a34efa65...
  - spec-committed a3d55ede... and spec-working fa4a4c90...
  - The doc-commit hashes match commit-messages.txt.
- **ASCII.** report-draft.md is ASCII, with no U+2014.
- **Numbers.** These are all verified in primaries:
  - the by-file diffstat (recomputed from diff.patch: 979/179 code, 871/6 doc);
  - the 204 pub lines (gate.log:6598) and the counts lines (gate.log:5936-5942);
  - drawing created 17 and appended to 10 (run.log:4451);
  - the families (run.log:4566) and the open-check totals (run.log:6902);
  - scan pre-check "total hits: 0", "structural checks failed: 0" and RESULT: PASS;
  - RERUN_NOTE unset in all four logs.
- **Quotes.** Each quoted string matches verbatim, sometimes as a substring of a longer log line.

Statements I could NOT find in any primary:
1. The per-commit diffstats: compaction "137/151", kinds "5 files, 494/31", tool "2 files, 353/2". diff.patch covers only the combined range, and no log prints per-commit line counts. The report does not mark these as lacking a primary.
2. "image, anchor and removeimage against the records part 4 P1 used". The run log names the records (m3p1-p3-image, m4p1-p3-anchor, m4p1-p3-removeimage) but nowhere says part 4 P1 used them.

Statements the report itself declares as having no primary:
- the compaction's 348-byte dev saving (spec-working.md ruling A repeats it, also as "no primary");
- gate dry run 1's failure and the aborted second dry run (finding 2);
- the unlogged 26-workbook dev run (finding 5).

Disagreements: none of substance. The two unsourced items above should either be sourced or marked "no primary" in the report.
