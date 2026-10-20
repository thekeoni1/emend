## Question 1

**Verdict:** Yes. The diff does what P1 asks and nothing outside it. I found no missing item and no break from the spec's forms or rules, only the small points listed under "Small differences".

**Coverage, checked against phase.diff:**
- The diff hashes to f38a085d..., which is the combined-diff hash in gate.log.
- The specs/0040 file it adds is byte-identical to spec-0040.md (sha256 f1919878...).

**Types ("The call"):**
- `NewSeries` gains `trendline: Option<NewTrendline<'a>>` and `error_bars: &'a [NewErrorBars<'a>]`.
- `NewTrendline`, `TrendlineKind` (Linear, Exponential, Logarithmic, Power, Polynomial(u8), MovingAverage(u16)), `NewErrorBars`, `ErrorDirection`, `ErrorBarKind` and `ErrorValue` (Fixed, Percentage, StandardDeviation, StandardError, Custom{plus, minus}) have the spec's fields.
- The derives are as the spec gives: `NewTrendline`, `NewErrorBars` and `ErrorValue` have `PartialEq` without `Eq`; the three enumerations have `Eq`; `NewSeries` drops `Eq`.
- All are re-exported from `package`. `NewChart`, `NewAxis` and the plot lines are not touched, which is right because they belong to P2.
- No reader change: chart.rs is not in the diff.
- No new message: package.rs changes only its re-export list, and the catalogue is unchanged.

**Applicability:**
- `takes_analysis` accepts Bar, Line, Scatter, Area and Bubble and refuses every other kind with CHART_OPTION_KIND.
- A moving average refuses a forecast, an intercept, the equation and R-squared. Logarithmic and Power refuse an intercept.
- The ranges are enforced: polynomial order 2 to 6, period 2 or more, forecasts finite and not negative, intercept finite, Fixed/Percentage/StandardDeviation finite and positive.
- A direction is required on a scatter or bubble and refused elsewhere. Two sets are allowed only on a scatter or bubble, and only with different directions; three or more are refused.
- The trendline name goes through `text`, and the custom references go through `reference`.
- The checks run after the marker, line and points and before the combined plot's rules, as the tests show.
- A combined second plot's series are judged by the second plot's kind, as the test shows.

**Forms ("The chart part"):**
- The trendline is written after `dLbls`, then each `errBars`, before `cat`/`xVal`.
- Trendline order: `name` element, `trendlineType`, `order`/`period`, `forward`, `backward`, `intercept`, `dispRSqr`, `dispEq`. A moving average writes both flags as 0.
- errBars order: `errDir` (only when given), `errBarType`, `errValType`, `noEndCap` (the negation of `end_cap`), then `plus` before `minus` as `numRef`, or `val`. Standard error writes neither.
- Numbers use `f64::to_string`.

**Tests:** 7 crate tests and 3 tool tests, covering every item in the P1 list.

**Sample:** the line, scatter and column charts match "Harness runs" exactly:
- Line: Linear showing the equation and R-squared with Fixed 1.5 Both and end caps; then Polynomial(3) named "emend fit", forward 2, with Percentage 5 Minus.
- Scatter, LineMarker: Exponential, with Y StdDev 1 and X Custom over C and D.
- Column, clustered: MovingAverage(2) with StandardError and no end caps.

References are bounded to 1000 rows.

**Small differences (none blocks):**
1. Extra field: the tool's line also prints `chart_parts_after`. The spec lists only the counts before and after, the series, the rows and the read-back. This follows the format sample's form and is asserted.
2. Check order: within error bars, the code checks the number of sets before each set's direction and value. The spec says only "in field order", and the set count is not a field. The test `count_first` pins this order and the doc comment states it.
3. Conflicting wording: the main.rs doc comment says the charts are "placed as the more kinds sample's". The a22f6d7 commit message says "placed as the format sample's", and the spec says "as `--chart-format-sample` does".
4. Cosmetic: one rewrapped doc-comment line in add_chart's doc is longer than its neighbours: "apply to its kind" for a combined plot a first plot other than a bar, line or area, a second. The fmt check still passes.

## Question 2

**Verdict:** Yes, with one weak spot. Every P1 log requirement has a line; one fixture item is shown only indirectly.

**Requirements with lines:**
- **Target root:** gate.log line 514 gives `<target>` as `<phase>/target`, and line 511 gives `<phase>` as `<home>/.cache/emend-build/m6p3-p1`.
- **Prune:** gate.log lists each of m6p2-p1, m6p2-p2, m6p2-p3 and m6p2-close with du -sb, removes it with `rm -r` exit 0, and shows the kept names still there.
- **Doc commit by hash:** gate.log checks the three blobs against the message's hashes and the brief's prefixes, and shows the doc commit adds 0040 and modifies 0039 and ROADMAP.md, nothing else.
- **test.log:** both targets, with the feature sets off, charts, charts,images, charts,comments, charts,read and every feature, plus the workspace. The 7 new crate tests and the tool's 3 are found by name. Clippy and rustdoc run with -D warnings.
- **negative.log:** N1 to N12 through the binary, with N12 and its control B7. The P1 rules are listed as unreachable through the binary, with their covering tests named.
- **Corpus run, both targets:** 27/27 ok, lo and lo_in ok 27, and the chart family shows 81 added (three per workbook).
- **Rows:** bounded at 1000, with the two large workbooks at the bound.
- **Fixture generator:** written with zipfile, its text printed in the log: twelve cells; trendline with poly, order 2 and the name element; errBars with both and fixedVal 1; dropLines; upDownBars; dateAx with days; valAx with logBase, max, min and majorUnit.
- **Fixture checks:** its own forms (seven checks). `--chart-trend-sample` count 4. The ElementTree check of the series' child order, trendline then errBars before the references, and the plot lines before the axIds (twelve checks). lo and lo_in ok on each fixture output.
- **Keep and open check:** the keep is regenerated and the open check is run.
- **Scan:** the scan pre-check uses the whole-word denylist form.
- **Size:** the probe figure is printed in gate.log.

**Weak spot:** the spec says `--charts-sample` on the fixture reads "the date axis as `Date`, the value axis's `min` and `max`". The only evidence is the charts line's `axis_other=1` and `axis_bounds=1`. `axis_other` does not tell a Date axis from a Series axis, and `axis_bounds` counts axes that have a bound, not min and max separately. The log's verdict line "the date axis as axis_other 1 ... axis_bounds 1, the value axis's min and max: yes" rests on that indirect reading.

**Minor:** no run.log line asserts that its own checks print no corpus name, reference, path or text. Only the scan pre-check's 0 hits over run.log covers this. The scan's final run is not here yet.

## Question 3

**Verdict:** Every number and hash in the report matches a primary. Some statements rest only on the session's word or the README, and one claim is stronger than its primary.

**Checked and matching:**
- **Commit hashes and subjects:** 07cc871..., 12f1314..., a22f6d7..., and the base 2933e3a with its subject.
- **Numstat table:** every row matches what I recomputed from phase.diff (25/7, 3/2, 392/4, 819/5, 8/0, 55/0, 537/0, 293/1, 190/0, 6/0).
- **Hashes:** all four log hashes, all four record hashes and the scan-pre-check hash 575aeefd... match my sha256 of the files.
- **End times:** 18:59:04Z, 19:00:43Z, 19:00:52Z and 21:08:17Z.
- **Messages:** 154/154.
- **Prune bytes:** 5891042093, 5834798973, 7902484767 and 2047152568.
- **gate.log:**
  - Test counts: 247 (83, 20, 144); the 2 edited tests by name; 10 new tests (7 and 3).
  - Probe: 1103064, 1097236 and 5828.
  - check.sh: nine PASS lines.
- **test.log:** 1376 against 1366 (+10), 515 (+0), and 659/701/704/762/1141, each +7.
- **negative.log:** N1 to N12, DRAWING_UNREADABLE, B7.
- **Corpus run:**
  - Files and opens: 27/27; lo, lo_in and leftover 27.
  - Entries: 798/72/124/0/0.
  - Rows: 6 to 1000, with 9132ce5303d8 and ab0e6a529465 at 1000.
- **Fixture:** id 04de5598008c, 3702 bytes, the seven and twelve checks, and the line `charts_before=1 charts_after=4 series=4 rows=6 read_back=3 chart_parts_after=4`.
- **Keep:** 1896 pairs, 3792 files, 1748, 148 new, 0 gone, 0 differ; open record 1896.
- **Size estimate:** 16500 to 26000, as in the spec.
- **Branch names:** as in specs/0039 ruling U.

**Disagreements:**
1. **Overclaim on the brief's hash.** The report says the specs/0040 blob is "the hash the brief gave (gate.log checks the three files against the brief's hashes and the message's)". gate.log checks only the brief's 8-digit prefixes ("its first 8 digits the prefix planning's brief gave", "(the prefix planning's brief gave: f1919878)"). It checks the full hashes against the doc commit's message, not against the brief.
2. **No primary for the dry run.** "Each logged log is the first of its kind; one dry run before them ended yes for all four" is not marked as the session's statement. Its only source is the README, and no log, record, diff or commit message shows it.
3. **Findings 2 to 5 have no log primary.** The two superseded negative dry runs and their reasons, the `rm -rf` of the scratch directory, "the facts file had named debug binaries", and the keep's first failed run appear only in the README's session statements. That run.log builds release binaries is in run.log.
4. **Finding 7 is not printed by any log.** The deletion of the six branches is supported only by the README ("git printed each name deleted"). The names themselves are in the diff (specs/0039 ruling U).
5. **Internal wording conflict.** Finding 8 says "No failed dry or logged run of the phase's four scripts", while Finding 2 reports two refused development dry runs of the negative generator. The README settles it (those were earlier versions, not the final script), but Finding 8's wording does not say so.