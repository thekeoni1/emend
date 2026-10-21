# specs/0040: analytic chart elements (Milestone 6 part 3)

Status: APPROVED 2026-10-20 (Milestone 6 part 3; decisions 1 to 4 A,
typed in the planning session 2026-10-20: the five 2D kinds take
trendlines and error bars; the plot lines are booleans on `NewChart`;
the date axis is in; the reader is unchanged). Planning session.
Builds on `specs/0035` (the first set), `specs/0036` (the second set),
`specs/0038` (chartsheets) and `specs/0039` (kinds and formatting);
the writer's forms, checks and messages named there stand. The part
is proven on built packages, on the corpus run of two new samples, on
two generated fixtures and by the LibreOffice open check, as part 2
was; the Milestone 2 Excel check gates no push.

## Goal

The analytic half of `specs/0036`'s out-of-scope list, Milestone 6's
part 3 (`ROADMAP.md`, "Part 3: analytic chart elements"): a written
series takes a trendline and error bars; a written line, area or stock
plot takes drop lines, high-low lines and up-down bars; a written
chart's axes take a scale (`min`, `max`, a logarithmic base, a
reversed orientation), major and minor units, tick label and tick mark
skips, and a date category axis with its time units. openpyxl's
`Trendline`, `ErrorBars`, `ChartLines`, `UpDownBars`, `Scaling`,
`DateAxis` and the axis attributes `majorUnit`, `minorUnit`,
`tickLblSkip` and `tickMarkSkip`. The reader is unchanged: it already
reads an axis's `min`, `max` and date kind and walks past `trendline`
and `errBars` ("Facts").

## Size and packaging

The budget figure after part 2 is 438960 section bytes against the
ceiling of 458752 (`records/size-m6p2-2026-10-20-50a3989.md`): 19792
of headroom, held for fixes and part 5. Everything in this part goes
under the feature `charts`, measured by `probe-emend-charts-write-xml`,
whose work gains, on every kind that takes them, a trendline and error
bars per series, the plot lines, and axis options ("The probe"). The
`charts` feature costs 125060 section bytes after part 2 (the probe
1097236); the probe's movement over that is this part's cost, and the
size record states the feature's whole cost, part 2's figure and the
delta.

Planning's estimate, priced as `specs/0036` prices (about 500 section
bytes per fixed form, call site and literal together) and, after
`specs/0039` ruling P, for the arms the probe reaches: one arm of each
enumeration plus its dispatch, since the probe takes one value per
option from its constants and the optimiser may drop the arms it never
reaches. Trendlines (`trendline`, `name`, `trendlineType` with its
dispatch, `order`, `period`, `forward`, `backward`, `intercept`,
`dispRSqr`, `dispEq`: 10 forms) 4000 to 6000; error bars (`errBars`,
`errDir`, `errBarType`, `errValType` with their dispatches, `noEndCap`,
`plus` and `minus` references, `val`: 8 forms) 3500 to 5500; the plot
lines (`dropLines`, `hiLowLines`, `upDownBars` with `gapWidth`: 3
forms) 1000 to 2000; the axes (`logBase`, `orientation` `maxMin`,
`max`, `min`, `majorUnit`, `minorUnit`, `tickLblSkip`, `tickMarkSkip`,
`dateAx`, `baseTimeUnit`, `majorTimeUnit`, `minorTimeUnit` with the
time unit's dispatch: 13 forms) 5500 to 8500; the checks 2500 to 4000.
In all 16500 to 26000 under `charts`. Planning is told the figure in
the report at any value; a cost above 31000 is a finding the report
names. The core's growth is expected 0 and accepted up to 2000. The
`images` and `comments` costs may move (expected 0, accepted up to 1500
each): the size log judges the costs' movements under their own names,
the probes' growth printed beside them unjudged (`specs/0038` ruling
N), and the copy paragraph cites the band it is judged by. The size
record's charts probe row is rewritten as one description with one
index range (`specs/0039` ruling Q; "The probe").

## Facts planning checked (2026-10-20)

- The corpus holds no `trendline` and no `errBars` (`specs/0034`,
  "Facts": the survey over 26 charts), no `dateAx` and no `serAx`,
  `scaling` `min` or `max` on some axes; `hiLowLines`, `upDownBars`
  and `dropLines` are walked past by the reader. So the elements are
  proven on built packages, on the corpus run of the samples (every
  workbook gains the charts) and by the LibreOffice open check, as the
  kinds of `specs/0036` and `specs/0039` were.
- The reader (`crates/emend/src/package/chart.rs` at bcc53a5) models
  per axis `kind` (`Category`, `Value`, `Date`, `Series`), `position`,
  `crosses`, `title`, `number_format`, `min` and `max` (`scaling`'s
  `min` and `max` values, `read_scaling`), `deleted` and
  `major_gridlines`; per series `index`, `order`, `name`,
  `categories`, `values`, `sizes`, `fill`, `smooth`, `marker` and
  `labels`. `read_series` walks past every other child, `trendline`
  and `errBars` among them, with its subtree; `read_axis` reads a
  `crosses` of `autoZero`, `max` or `min` and refuses any other
  value; it does not read `logBase`, `orientation`, the units, the
  skips or a date axis's time units. So the read-back compares an
  axis's `min`, `max` and `Date` kind, and the test and the run log's
  own `ElementTree` check read the rest from the written text.
- The writer (`crates/emend/src/package/chart_write.rs` at bcc53a5):
  `axis_bytes` writes `<c:scaling><c:orientation val="minMax"/>
  </c:scaling>` for every axis, the fixed tick marks and label
  position, `crossAx`, `crosses`, then for a value axis
  `crossBetween`, for a category axis `auto`, `lblAlgn`, `lblOffset`
  and `noMultiLvlLbl`, for a series axis nothing; it writes no
  `dateAx`. `series_bytes` writes `idx`, `order`, `tx`, `spPr`, the
  marker, the points, `dLbls`, then the references; `plot_bytes`
  writes after the series a line plot's `<c:marker val="1"/>`, a stock
  plot's fixed `<c:hiLowLines/>` and the other kinds' tails, then the
  `axId`s. The reader's `Series` and `ChartAxis` are unchanged by this
  part.
- openpyxl 3.1.5 on the pinned image (planning's probe 2026-10-20, a
  built workbook, no corpus content): a series' `trendline` is
  written after `marker` and before `errBars`, which precedes `xVal`,
  `cat` or `val`; `trendline` holds `trendlineType` (`linear`, `log`,
  `poly`, `power`, `exp`, `movingAvg`), `order`, `period`, `forward`,
  `backward`, `intercept`, `dispRSqr`, `dispEq`, and openpyxl writes
  a trendline's `name` as an ATTRIBUTE of `trendline` (`<trendline
  name="fit">`), not the schema's `name` element; `errBars` holds
  `errDir` (`x` or `y`, written on every kind, a line's included),
  `errBarType` (`both`, `plus`, `minus`), `errValType` (`cust`,
  `fixedVal`, `percentage`, `stdDev`, `stdErr`), `noEndCap`, then for
  `cust` `minus` BEFORE `plus` (each a `numRef`), else `val` (none for
  `stdErr`); a line chart's `dropLines`, `hiLowLines` and
  `upDownBars` (holding `gapWidth` 150) follow the last series and
  precede the `axId`s, a stock chart's `hiLowLines` and `upDownBars`
  the same; `scaling` holds `logBase`, `orientation`, `max`, `min` in
  that order; a value axis's `majorUnit` and `minorUnit` follow
  `crossAx`; a category axis's `tickLblSkip` and `tickMarkSkip` follow
  `lblOffset`; `DateAxis` writes `dateAx` with `numFmt`, then after
  `crossAx` `baseTimeUnit`, `majorUnit`, `majorTimeUnit` (the units
  `days`, `months`, `years`). openpyxl's `LineChart` carries
  `dropLines`, `hiLowLines`, `upDownBars`; `StockChart` the same;
  `AreaChart` `dropLines` alone; `BarChart`, `ScatterChart`,
  `RadarChart`, `PieChart`, `BubbleChart` and `SurfaceChart` none.
  openpyxl writes no `crosses` and no `crossBetween` by default, and
  its `lblOffset` is bounded 0 to 1000.
- The schema (ECMA-376 part 1, 21.2.2, planning's reading; a
  LibreOffice refusal of a written form in P1's or P2's tests is a
  finding against this reading): `trendline` holds `name`?, `spPr`?,
  `trendlineType`, `order`? (2 to 6, a polynomial's), `period`? (2 or
  more, a moving average's), `forward`?, `backward`?, `intercept`?,
  `dispRSqr`?, `dispEq`?, `trendlineLbl`?; `errBars` holds `errDir`?,
  `errBarType`, `errValType`, `noEndCap`?, `plus`?, `minus`?, `val`?,
  `spPr`?; a bar, line, area or bubble series holds `trendline`* and
  `errBars`? after `dLbls` and before `cat` or `xVal`, a scatter
  series `trendline`* and `errBars` x0 to 2, a pie, doughnut, of-pie,
  radar, stock or surface series neither; `lineChart` holds after its
  series `dLbls`?, `dropLines`?, `hiLowLines`?, `upDownBars`?,
  `marker`?, `smooth`?, `axId` x2; `areaChart` `dLbls`?, `dropLines`?,
  `axId` x2; `stockChart` `dLbls`?, `dropLines`?, `hiLowLines`?,
  `upDownBars`?, `axId` x2; `line3DChart` and `area3DChart` take
  `dropLines`? (decision 1); `upDownBars` holds `gapWidth`?, `upBars`?,
  `downBars`?; `scaling` holds `logBase`? (2 to 1000), `orientation`?
  (`maxMin`, `minMax`), `max`?, `min`?; `valAx` ends `crossAx`,
  `crosses`?, `crossBetween`?, `majorUnit`?, `minorUnit`?, `dispUnits`?;
  `catAx` ends `crossAx`, `crosses`?, `auto`?, `lblAlgn`?, `lblOffset`?,
  `tickLblSkip`? (1 or more), `tickMarkSkip`? (1 or more),
  `noMultiLvlLbl`?; `dateAx` holds `axId`, `scaling`, `delete`,
  `axPos`, `majorGridlines`?, `minorGridlines`?, `title`?, `numFmt`?,
  `majorTickMark`?, `minorTickMark`?, `tickLblPos`?, `spPr`?, `txPr`?,
  `crossAx`, `crosses`?, `auto`?, `lblOffset`?, `baseTimeUnit`?,
  `majorUnit`? (a positive number), `majorTimeUnit`?, `minorUnit`?,
  `minorTimeUnit`? (the units `days`, `months`, `years`); `serAx` ends
  `crosses`, `tickLblSkip`?, `tickMarkSkip`?.
- Excel's behaviour planning relies on, from the schema and the Excel
  forms `specs/0034` surveyed, not re-measured here: a moving average
  takes no forecast, intercept, equation or R-squared; an intercept
  applies to a linear, exponential or polynomial trendline; a 3D chart
  draws no trendline and no error bars (decision 1); a surface's and a
  series axis take no option of this part.
- `tools/emend-copy` at bcc53a5: `--chart-format-sample` adds three
  charts past the last row of the first worksheet, each read back, and
  prints one line of counts; the samples of this part follow that
  shape. `scripts/openpyxl-ops.py`'s `OPS` end `"chartsheet",
  "chart-format"`; the new op follows `chart-format`'s shape, its
  constants named in the header.

## Design

### Crate

Under `charts`, in the writer alone: no reader change (decision 4 A;
the read-back compares what part 3's reader models, an axis's `min`,
`max` and `Date` kind among them). `images` and `comments` gain
nothing. No new dependency.

### The call

`add_chart` and `add_chartsheet` are unchanged in signature. The types
grow (the crate is unpublished, Milestone 4 decision 1 HOLD; the API
listing marks each changed row `changed`, verdict `keep`):

- `NewSeries` gains `trendline: Option<NewTrendline<'a>>` and
  `error_bars: &'a [NewErrorBars<'a>]` (none, one, or on a scatter or
  bubble series two, one per direction).
- `NewTrendline<'a> { kind: TrendlineKind, forward: Option<f64>,
  backward: Option<f64>, intercept: Option<f64>, show_equation: bool,
  show_r_squared: bool, name: Option<&'a str> }`; `TrendlineKind {
  Linear, Exponential, Logarithmic, Power, Polynomial(u8),
  MovingAverage(u16) }`, the polynomial's order 2 to 6, the moving
  average's period 2 or more.
- `NewErrorBars<'a> { direction: Option<ErrorDirection>, kind:
  ErrorBarKind, value: ErrorValue<'a>, end_cap: bool }`;
  `ErrorDirection { X, Y }`; `ErrorBarKind { Both, Plus, Minus }`;
  `ErrorValue<'a> { Fixed(f64), Percentage(f64), StandardDeviation(f64),
  StandardError, Custom { plus: &'a str, minus: &'a str } }`, the two
  references in the formula form the series' references take.
- `NewChart` gains `drop_lines: bool`, `high_low_lines: bool`,
  `up_down_bars: bool` (decision 2 A), `category_axis: Option<NewAxis>`
  and `value_axis: Option<NewAxis>`. `NewCombined` is unchanged: the
  secondary axis takes no option of this part ("Out of scope").
- `NewAxis { min: Option<f64>, max: Option<f64>, log_base: Option<f64>,
  reversed: bool, major_unit: Option<f64>, minor_unit: Option<f64>,
  tick_label_skip: Option<u32>, tick_mark_skip: Option<u32>, date:
  Option<NewDateAxis> }`; `NewDateAxis { base_unit: TimeUnit, major:
  Option<(u32, TimeUnit)>, minor: Option<(u32, TimeUnit)> }`;
  `TimeUnit { Days, Months, Years }` (decision 3 A).
- `NewTrendline`, `NewErrorBars`, `ErrorValue` and `NewAxis` derive
  `Clone`, `Copy`, `Debug` and `PartialEq` without `Eq` (their `f64`
  fields, as `OfPieSplit` forced on `NewPlot`, `specs/0039` ruling E
  (b)); `NewSeries` derives `Eq` at bcc53a5 and loses it with the
  trendline field, the API listing marking its row; `NewChart` already
  lacks `Eq`. `TrendlineKind`,
  `ErrorDirection`, `ErrorBarKind`, `NewDateAxis` and `TimeUnit`
  derive `Eq` as well. The new fields are none and false where a test
  or sample builds a series or chart with `..` from an existing one.

### Applicability

The parts 4, 5, 1 and 2 rules stand. The new ones, each refused with
"a chart option does not apply to its kind" unless named otherwise:

- `trendline` and `error_bars`: on a `Bar`, `Line`, `Scatter`, `Area`
  or `Bubble` series; refused on `Pie`, `Pie3D`, `Doughnut`, `OfPie`,
  `Radar`, `Stock`, the surfaces and every 3D kind (decision 1 A). A
  combined second plot's series take them by the second plot's kind.
- A trendline's `forward`, `backward`, `intercept`, `show_equation`
  and `show_r_squared` are refused on `MovingAverage`; `intercept` is
  refused on `Logarithmic`, `Power` and `MovingAverage`. `forward` and
  `backward` must be finite and non-negative, `intercept` finite, the
  polynomial's order 2 to 6, the period 2 or more; otherwise "a chart
  value is out of its range". `name` is written as the `name` element
  (the schema's form, not openpyxl's attribute), escaped as a title is.
- Error bars' `direction`: required on a `Scatter` or `Bubble` series
  (`Some`), refused on every other kind (`None` there, and no `errDir`
  is written); two sets on one series only on `Scatter` or `Bubble`
  and with different directions, else the option-kind message; more
  than two refused. `Fixed`, `Percentage` and `StandardDeviation` take
  a finite positive number (else the range message); `Custom`'s two
  references are checked as the series' references are and written as
  `plus` then `minus` (the schema's order; openpyxl's is the reverse).
- `drop_lines`: on `Line`, `Area` and `Stock`; `high_low_lines` and
  `up_down_bars`: on `Line` and `Stock`. A stock plot writes
  `hiLowLines` today and keeps doing so: `high_low_lines` false on a
  stock is refused (the plot cannot drop them), true accepted. Every
  other kind refuses all three.
- `category_axis` and `value_axis`: on every kind that writes the axis
  (the round kinds refuse both); `category_axis` names a scatter's or
  bubble's x axis, as `category_axis_title` does. The value-axis
  fields `min`, `max`, `log_base`, `major_unit` and `minor_unit` apply
  to a value axis (and to a scatter's or bubble's x axis); `min`,
  `max`, `major_unit` and `minor_unit` also to a date axis; they are
  refused on a category axis. `tick_label_skip` and `tick_mark_skip`
  apply to a category axis (a `catAx`), refused on a value or date
  axis. `date` applies to the category axis of a `Bar`, `Line`, `Area`
  or `Stock` plot; refused elsewhere and on `value_axis`. `reversed`
  applies to any axis. A surface's series axis takes no option. `min`
  below `max` when both are given, `log_base` 2 to 1000, the units
  finite and positive, the skips 1 or more, a date unit's count 1 or
  more; otherwise the range message. A combined chart's secondary axis
  takes none of these (unchanged).

The check order (`specs/0039` "Applicability") extends: after each
series' marker, line and points and before the combined plot's rules,
each series' trendline then its error bars in field order; then the
plot lines; then the category axis's fields and the value axis's in
declaration order; the texts after, unchanged.

### The chart part

The forms, in schema order, each written only where the option is
given:

- A series' `trendline`, after `dLbls` and before `cat` or `xVal`:
  `<c:trendline>[<c:name>NAME</c:name>]<c:trendlineType val="TYPE"/>
  [<c:order val="N"/>][<c:period val="N"/>][<c:forward val="F"/>]
  [<c:backward val="B"/>][<c:intercept val="I"/>]<c:dispRSqr
  val="0|1"/><c:dispEq val="0|1"/></c:trendline>`, TYPE one of
  `linear`, `exp`, `log`, `power`, `poly`, `movingAvg`; `order` on
  `poly`, `period` on `movingAvg`; on `movingAvg` `dispRSqr` and
  `dispEq` are written `0`.
- A series' `errBars`, after the trendline: `<c:errBars>[<c:errDir
  val="x|y"/>]<c:errBarType val="both|plus|minus"/><c:errValType
  val="fixedVal|percentage|stdDev|stdErr|cust"/><c:noEndCap
  val="0|1"/>[<c:plus><c:numRef><c:f>REF</c:f></c:numRef></c:plus>
  <c:minus>...</c:minus>][<c:val val="V"/>]</c:errBars>`; `noEndCap`
  is `0` for `end_cap` true; a scatter's or bubble's two sets in the
  order given.
- The plot lines, after the last series and before a line plot's
  `marker`, an area plot's or a stock plot's `axId`s: `<c:dropLines/>`,
  `<c:hiLowLines/>`, `<c:upDownBars><c:gapWidth val="150"/>
  </c:upDownBars>`.
- An axis's `scaling`: `<c:scaling>[<c:logBase val="L"/>]
  <c:orientation val="minMax|maxMin"/>[<c:max val="X"/>][<c:min
  val="N"/>]</c:scaling>`.
- A value axis's tail: after `crossBetween`, `[<c:majorUnit val="U"/>]
  [<c:minorUnit val="U"/>]`. A category axis's tail: after `lblOffset`
  and before `noMultiLvlLbl`, `[<c:tickLblSkip val="N"/>]
  [<c:tickMarkSkip val="N"/>]`.
- A date axis in a category axis's place: `<c:dateAx>` with the same
  `axId`, `scaling`, `delete`, `axPos`, `majorGridlines` where the
  category axis has them, the title, the number format (`General`
  source-linked when none is given, as today), the fixed tick marks and
  label position, `crossAx`, `crosses`, then `<c:auto val="1"/>
  <c:lblOffset val="100"/><c:baseTimeUnit val="days|months|years"/>
  [<c:majorUnit val="N"/><c:majorTimeUnit val="UNIT"/>][<c:minorUnit
  val="N"/><c:minorTimeUnit val="UNIT"/>]</c:dateAx>`. The plot's
  `axId`s are unchanged.

Numbers are written with Rust's shortest round-trip form, as the of-pie
split is.

### Errors

No new message. "a chart value is out of its range" covers every number
rule above; "a chart option does not apply to its kind" every
applicability rule; the references' refusals are the existing ones.
The catalogue in the `package` header gains no entry; its `charts`
limits paragraph names the ranges.

### 32-bit and streaming

Nothing new: the forms are literals and short formatted values; no
part is read whole.

### Harness runs

`--chart-trend-sample` in `emend-copy` (under `charts`, P1): on each
corpus workbook, as `--chart-format-sample` does, three charts past the
last row of the first worksheet, each read back, the references bounded
by `specs/0039` ruling C: a `Line` of two series from columns B and C
against A, the first with a `Linear` trendline showing the equation and
R-squared and `Fixed` error bars of 1.5 both ways with end caps, the
second with a `Polynomial(3)` trendline named and forecast forward 2
and `Percentage` error bars of 5 minus only; a `Scatter` `LineMarker`
of one series (B against A) with an `Exponential` trendline and two
error-bar sets, `Y` `StandardDeviation` 1 and `X` `Custom` over columns
C and D; a `Bar` of one series with a `MovingAverage(2)` trendline and
`StandardError` error bars without end caps. The tool's line prints
the chart count before and after, the series count, the rows and the
read-back count (3). Expected on the corpus: the format sample's entry
pattern (three chart parts `added`); `lo` and `lo_in` `ok` for every
workbook; the open check of the keep.

`--chart-axis-sample` (P2): three charts past the last row, each read
back: a `Line` of one series with drop lines, high-low lines and
up-down bars, its value axis `min` 0, `max` 100, `major_unit` 25,
`minor_unit` 5, its category axis `tick_label_skip` 2 and
`tick_mark_skip` 2; a `Scatter` `Marker` of one series with a
`log_base` 10 value axis and a `reversed` x axis with `min` 1; a `Bar`
of one series with a date category axis (`Days`, major 1 `Months`) and
a `reversed` value axis. The tool's line prints the chart count before
and after, the series count, the axes read back as `Date` (1), the
axes read back with a `min` or `max` (3) and the read-back count (3).
Expected on the corpus: three chart parts `added`; `lo` and `lo_in`
`ok`; the open check of the keep.

Then the two generated fixtures, each under the phase directory, never
committed, their values printable:

- Excel's form (P1), written by the run log's own generator with
  `zipfile`: one worksheet of twelve cells, one drawing holding a
  `lineChart` of one series with a `trendline` (`poly`, order 2, the
  `name` element) and `errBars` (`both`, `fixedVal` 1), `dropLines`
  and `upDownBars`, a `dateAx` with `baseTimeUnit` `days` and a
  `valAx` with `scaling` `logBase` 10, `max`, `min`, `majorUnit`, in
  schema order (the generator's text printed in the log).
  `--charts-sample` on it on both targets reads the chart (count 1,
  series 1, the date axis as `Date`, the value axis's `min` and `max`);
  `--chart-trend-sample` adds three beside it (count 4); the run log's
  own check with `ElementTree` confirms each written series' children
  in schema order, `trendline` then `errBars` before the references,
  the plot lines before the `axId`s.
- openpyxl's form (P2), written by the pinned image through
  `scripts/openpyxl-ops.py chart-analytic <in> <out>` (the op grows the
  script and its test; a `LineChart` over the input's first sheet with
  a linear `Trendline` showing the equation, `ErrorBars` `fixedVal`
  1.5, `dropLines`, the y axis `min` 0, `max` 50, `majorUnit` 10, the
  x axis `tickLblSkip` 2; and a second `LineChart` with a `DateAxis`
  of `baseTimeUnit` `days` and `majorUnit` 2 `days`, its y axis
  `logBase` 10): `--charts-sample` on it reads both charts (count 2,
  one `Date` axis, the value axis `min` and `max`); `--chart-axis-
  sample` adds three beside them (count 5); the run log's own check
  confirms the written forms against the openpyxl ones field by field
  where both write the field (`trendlineType`, `dispEq`, `errBarType`,
  `errValType`, `val`, `logBase`, `max`, `min`, `majorUnit`,
  `tickLblSkip`, `baseTimeUnit`, `majorTimeUnit`), each printed; the
  `name` attribute, `errDir` on a line and the `minus`-`plus` order
  are openpyxl's forms, named in the log and not compared.

`lo` and `lo_in` `ok` on every fixture output. The run log's own checks
print no corpus name, reference, path or text.

### The probe

`PLOTS` is unchanged, sixteen kinds. On every kind that takes them,
each series of the second set's chart gains a trendline (`Linear`,
showing R-squared, forecast forward 1) and error bars (`Fixed` 1,
`Both`, end caps; on a scatter `Y`); the line plot (kind 1) takes drop
lines, high-low lines and up-down bars, the area plot drop lines, the
stock plot drop lines and up-down bars; the value axis of every kind
that writes one takes `min` 0, `max` 100, `major_unit` 25,
`minor_unit` 5 and is `reversed`; the category axis of a bar, line or
area kind takes `tick_label_skip` 2 and `tick_mark_skip` 2, and on
kind 1 the date form (`Days`, major 1 `Days`); a scatter's x axis
takes `log_base` 10. One value per option from the probe's constants,
so the estimate prices the arms reached ("Size and packaging"). The
chartsheet addition of part 1 is unchanged; the probe's line count is
unchanged (`chartsheet_charts` last). The probe's header comment and
the size record's charts probe row describe the second add as ONE
description with one index range, 0 to 15, replacing the layered
clauses (`specs/0039` ruling Q): the size log's row text is rewritten
accordingly.

## Phases

Each phase as `specs/0001` "Rules for every phase" and part 2's
"Phases" paragraph: a doc commit first, the code commits, one record
commit, the scan last, at most two independent checks, the report
under `<phase>`, then stop. Target root
`<home>/.cache/emend-build/m6p3-pN/target`; P1's gate prunes Milestone
6 part 2's roots (`m6p2-p1`, `m6p2-p2`, `m6p2-p3`, `m6p2-close`, listed
with `du -sb`, removed with `rm -r`). Logs in `records/logs/m6p3-
<phase>/` (`gate.log`, `test.log` on both targets with the feature
sets off, `charts`, `charts,images`, `charts,comments`, `charts,read`
and every feature, `negative.log`, `run.log`, `scan.log`, `size.log`
in P3); the scan's denylist form whole-word. No push during the part;
the close-out push follows P3.

Part 3's first doc commit carries: this spec; `specs/0039` with its
close-out verification and rulings T and U; `ROADMAP.md` with part 2's
heading DONE and PUSHED and this part's rows; all three as planning
left them, by hash.

Generator rules carried from `specs/0039` and its rulings into this
part and every later one: those `specs/0039` "Phases" carries (AK a,
AK b, AC d, Z, AA, AD, D, F, N, R); the sample bounds its references
to 1000 rows and prints `rows` (ruling A, C); the close-out generator's
changes continue from 52; a probe that means to keep every arm live
takes its values from its command line, and an estimate prices the
arms the probe reaches (ruling P).

### P1: trendlines and error bars

The doc commit; `NewTrendline`, `NewErrorBars` and their enumerations
on `NewSeries`; the rules and forms; `--chart-trend-sample`; the tests
on built packages (every kind that takes and refuses them, each
trendline kind, each error value, the two-set scatter, the field
rules, the forms by text, the read-back); the negative log; the corpus
run, the Excel-form fixture and the open record.

### P2: plot lines, axis options, the openpyxl fixture, the catalogue

The carry; `drop_lines`, `high_low_lines`, `up_down_bars`, `NewAxis`
and `NewDateAxis` on `NewChart`; the rules and forms; `--chart-axis-
sample`; the `chart-analytic` op of `scripts/openpyxl-ops.py` with its
test; the `package` header's limits; the tests on built packages (the
axis options on each kind, the date axis form, the scaling order, the
skips before `noMultiLvlLbl`, the plot lines before `marker` and the
`axId`s, the read-back of `min`, `max` and `Date`); the negative log;
the corpus run of the new sample, P1's run rerun byte-identical, the
openpyxl fixture and the open record.

### P3: the probe, the size record, the API listing, the CHANGELOG

The doc commit (the P2 verification rulings); the carry; the probe's
options as "The probe" says and the one-piece description; the size
log, recorded as `records/size-m6p3-<utc-date>-<shortsha>.md`, with
the `charts` cost, its movement over part 2's 125060 judged against
the estimate and the 31000 line, the `images` and `comments` costs'
movements and the core's growth each printed and judged (ruling N's
form); the API listing rerun, recorded as
`records/api-review-m6p3-<utc-date>-<shortsha>.md`, the rows that lose
`Eq` marked; `CHANGELOG.md`'s "Added" gains one bullet citing this
spec. The negative log through the binary for every message reachable
there. The run log: the P2 runs rerun with the size log's release
binaries, byte-identical by sha256 (the openpyxl fixture by part 1's
P3 reading, `specs/0038` ruling P), recorded as
`records/m6p3-p3-<sample>-<utc-date>-<shortsha>.md` and `.txt`; the
open record of the part's keep.

### Close-out push (after P3)

As part 2's (`specs/0039` "Close-out push (after P3)" and its rulings
T and U): the generator transforms part 2's close-out scripts, its
numbered changes continuing from 52; the doc commit holds this spec
and `ROADMAP.md`; the base is part 2's close-out doc commit, named by
its subject "M6 part 2 close-out: commit planning docs (the P3
verification and rulings)"; `PREV_RECORD_PATTERN` is `^M6 part 2
close-out record`, in the range exactly once; the close-out doc
commit's subject is "M6 part 3 close-out: commit planning docs (the P3
verification and rulings)"; `CARGO_TARGET_DIR` printed and checked as
`<home>/.cache/emend-build/m6p3-close/target`; no Excel-record check;
the scan's denylist form whole-word; the range holds no archive; the
record commit "M6 part 3 close-out record: the push, the CI run and
the scan" holding the three logs only; pre mode on a clean tree; one
independent check before the report. The push carries part 2's
close-out record commit and this part's commits, on planning's word
and the maintainer's authorisation typed in the implementing session.
The hashes are filled in at the briefing.

## Planning verification of P1 (2026-10-20)

Verified from primaries: the chain 2933e3a, 07cc871 (doc), 12f1314
(core), a22f6d7 (tool, the phase commit), 74b63de (record), all local,
the tree clean, origin/main still bcc53a5; the doc commit's three
blobs at f1919878, 472e81b3 and b1a7df3c, the hashes planning left;
the seven files of records/logs/m6p3-p1 at the hashes the report
gives, the four logs ending "checks all as expected: yes" and the scan
"RESULT: PASS" with "structural checks failed: 0"; every commit
message ASCII with the Co-Authored-By line alone, the three counting
messages carrying the diff-alone sentence; planning's own privacy scan
over 2933e3a..74b63de, 4 commits, 2096 blobs, 0 findings, PASS; the
record commit's files holding no home path, name or address (the two
"/home/" hits are the run log's and the scan's own path patterns);
ANTHROPIC_API_KEY unset in planning's shell and "unset" in every log.
The code: NewTrendline, TrendlineKind, NewErrorBars, ErrorDirection,
ErrorBarKind and ErrorValue as "The call" gives them, re-exported;
NewSeries gaining the two fields and losing Eq; takes_analysis the
five 2D kinds; trendline_options and error_bars_options the
"Applicability" rules with the two messages; series_bytes writing the
trendline after dLbls then each errBars before cat or xVal;
trendline_bytes the name element, trendlineType, order or period,
forward, backward, intercept, dispRSqr, dispEq; error_bars_bytes
errDir where given, errBarType, errValType, noEndCap, plus then minus
as numRef, else val, none for stdErr; the reader untouched. The tool:
chart_trend_sample's three charts as "Harness runs" gives them, the
references through bounded_last_row, the read-back every field. The
logs: the prune of m6p2-p1, p2, p3 and close with du -sb then rm -r;
base tests 247, none gone, the two edited by their field lines; ten
new tests, seven and three, by name; messages 154 at both ends;
check.sh nine PASS; the probe 1103064 (5828 over 1097236, told, not
judged); tests 1376 on both targets, the six sets 515, 659, 701, 704,
762, 1141; N1 to N12 refused through the binary, N12 the trend sample
on a non-wsDr drawing, the rule messages named unreachable with their
covering tests; the corpus run 27 of 27 ok on each target, lo and
lo_in ok 27, every output the same on both targets, rows 6 to 1000
with two workbooks at the bound, entries identical 798, changed 72,
added 124 (81 chart parts), dropped 0; the Excel-form fixture
04de5598008c, 3702 bytes, its seven own-form checks, --charts-sample
reading it on both targets with the input written back unedited and
--chart-trend-sample adding three (count 4, rows 6, read_back 3), the
twelve form checks, the same output on both targets; the keep 1896
pairs, 148 new and exactly this phase's by name, 0 gone, 0 changed;
the open record 1896 pairs, every new pair ok on both sides, no
verdict or class of the 1748 moved; the independent check's answer
verbatim in the record and the report, the report's edits after it
wording alone.

### Ruling A: P1 is DONE

The ten new tests, the forms, the rules, the sample and the fixture
are as the spec gives them; nothing reopens P1. The probe's 5828 is
the part's running figure, judged by P3's size log against the
estimate of 16500 to 26000 and the 31000 line ("Size and packaging").

### Ruling B: the fixture's Date axis and bounds (check item (a))

Accepted as read. The fixture holds one dateAx and no serAx, so the
charts line's axis_other 1 can only be the date axis read as Date,
and axis_bounds 1 the one value axis holding a bound; the run log's
two controls (series 2, axis_other 0) refuse the line otherwise, and
the reader's own tests of specs/0034 read min, max and Date apart. No
tool change: the charts line's fields are part 1's. P2's
--chart-axis-sample prints the Date and bound counts the spec names,
and the openpyxl fixture, holding one DateAxis, is read by the same
rule.

### Ruling C: the doc comment's "more kinds sample's" (check item (b))

A wording error in tools/emend-copy/src/main.rs: the trend sample's
charts are placed as the format sample's, as the spec and the tool
commit say. P2's tool commit corrects that one line and names this
ruling; no code moves.

### Ruling D: the overlong doc line (check item (c))

The rewrapped line in add_chart's Errors doc is rewrapped to its
neighbours' width in P2's core commit, which extends that doc for the
plot lines and the axes; the commit message names this ruling.

### Ruling E: the set count before each set's fields (check item (d))

Accepted as the order: "in field order" is each set's fields; the
number of sets and whether the kind takes any are checked first, as
the doc comment states and the check-order test pins. P2's rules
follow the same shape: the kind, then the count where one applies,
then each field.

### Ruling F: run.log's no-text property (check item (e))

As every phase since specs/0028: the scan is the instrument that
covers the logs' text, and its final run is committed with the logs.
No log asserts it of itself.

### Ruling G: the session's statements (report items 1 to 5, 7, 8)

Accepted, with one reminder. Generators written by forked subagents
and reviewed by the session are the session's; the logs are the
primaries either way. The negative script's two superseded
development dry runs and the keep's first failed run are development,
not logged runs; none of the logged runs was refused or repeated. The
run log's release binaries are its template's form. The branches'
deletion rests on ruling U and on git branch listing main alone
(planning checked). The reminder: the negative fork's rm -rf on its
own scratch directory was outside a logged script and touched nothing
else, so it stands; from P2 every fork follows the scripts' rule, rm
-r without -f on a directory the same fork made, written with
"${var:?}", and the README says so if one does otherwise.

### Ruling H: the report's edits after the check (report item 10)

Accepted: the check's five Question 3 disagreements were wording (the
prefix check, the dry run's and the statements' sourcing, item 8's
scope) and the edits mark them as the session's statements; no number
moved, and planning's figures above are the logs', not the report's.

### Ruling I: P2's doc commit and carry

P2's doc commit carries this spec with this section and ROADMAP.md
with P1's row DONE, on 74b63de; its subject "M6 part 3 P2: commit
planning docs (the P1 verification)". The generators carry P1's with
the numbered changes continuing; the gate prunes nothing (the m6p3-p1
root stays for P3's rerun); the size figure is told at any value
again. NewSeries losing Eq is P3's API listing's, as item 6 says.

## Planning rulings during P2 (2026-10-20)

Asked by the implementing session after the P2 doc commit d8479f5,
before the code.

### Ruling J: major_unit and minor_unit on a date axis

"Applicability" and "The chart part" conflict: the first applies
NewAxis's major_unit and minor_unit to a date axis, the second takes
a dateAx's majorUnit and minorUnit from NewDateAxis's major and minor,
each a count with its time unit, the only form the schema gives a
dateAx. The second stands. On a date axis NewAxis's major_unit and
minor_unit are refused with the option-kind message; min, max and
reversed apply and are written in scaling; log_base, the skips and
the units' count come from NewDateAxis alone. "Applicability" reads
"min and max also to a date axis" from here.

### Ruling K: the check order of the axes

Confirmed as the implementing session reads it: the first plot's
series options as P1 checks them, then the plot lines, then the
category axis's fields, then the value axis's, all before the
combined plot's rules, then the combined plot's series options, then
the texts. For each axis, ruling E's shape: first whether the chart
writes that axis (the round kinds refuse either), then its form,
dateAx where date is given on the category axis of a Bar, Line, Area
or Stock plot and refused where date is given elsewhere, catAx or
valAx otherwise; then each field in NewAxis's declaration order, its
option-kind check by that form before its range check (a log_base on
a catAx is option-kind whatever its value; min below max is judged
when max is reached with min given); date's own counts last, each 1
or more.

### Ruling L: the probe's kind 1 and the stock literals

"The probe" gives the line plot's category axis (kind 1) both the
skips and the date form, which ruling J's forms refuse together: on
kind 1 the category axis takes the date form alone (Days, major 1
Days), the skips going on the bar and area kinds' category axes and
on no line kind's; P3 writes it so. Every NewChart literal with
NewPlot::Stock in the tests, the tool and the probe gains
high_low_lines: true, the written bytes unchanged, as the session
reads "Applicability"; the gate names the base tests edited by their
field lines.

### Ruling M: the axis sample's bounded-axis count

"Harness runs" describes three charts holding two axes with a min or
max (the Line's value axis, min 0 and max 100; the Scatter's x axis,
min 1) and then says the line prints that count as 3: planning's
arithmetic error. The description governs: the charts stay as
written, and the tool's line prints and asserts the bounded axes as
2, the Date axes as 1, series 3, read_back 3, the chart and chart
part counts three more than before. The line's keys, in order:
chartaxis, charts_before, charts_after, series, rows, date_axes,
bounded_axes, read_back, chart_parts_after; rows as specs/0039
rulings A and C. "Harness runs" reads "(2)" from here.

### Ruling N: the openpyxl fixture's field-by-field comparison

"Field by field where both write the field" compares forms, not
constants: the two samples' values differ by design. For each of the
twelve fields the run log reads openpyxl's element from the op's
output and emend's from emend's output on the same fixture, the axis
fields from --chart-axis-sample's charts and the five trendline and
error-bar fields from --chart-trend-sample run on the same fixture;
it compares the element's local name, its parent and its place among
its siblings by the schema's sequence, every sibling before it one
the schema puts before it and every sibling after it one the schema
puts after it, each element's preceding sibling printed for both
outputs (logBase with no sibling before it; majorUnit after
crossBetween where that is written, openpyxl 3.1.5 writing none,
otherwise after crossAx or crosses; tickLblSkip after lblOffset;
majorTimeUnit right after majorUnit), and val's lexical kind, an
enumeration value of the same set or a decimal number; it prints both
values; and where the two write the same constant (trendlineType
linear, dispEq 1, errBarType both, errValType fixedVal, logBase 10,
baseTimeUnit days) it requires them equal as well. The name
attribute, errDir on a line and the minus-plus order are named and
not compared, openpyxl writing none of them here. The tool commit's
rebuild (9a6a220 to d367326, a type alias for clippy, the op commit
recommitted unchanged) is accepted as a local rewrite before the
report (specs/0001); the README states it.

O. Finding F1 (P2 dry run 1, run log checks "fixture charts", "fixture
chart-axis" and "fixture chart-trend" on both targets: no). The pinned
LibreOffice (emend-lo 25.2.3-2-deb13u6) aborts with std::bad_alloc, exit
134, on the chart-analytic op's OUTPUT before emend touches it, so lo_in
and lo read error on the three fixture runs; the chart-axis corpus run
is 27 of 27 ok, lo and lo_in ok, on both targets. The cause, the
implementing session's bisection through scripts/lo-check.sh: the
DateAxis chart's categories are the fixture's column A, scale-gen's
numbers (planning read the fixture: 99 numeric cells in A2 to A100,
116233 to 42886443, a span of 42770210 days; the implementing session's
message said about 25.7M to 38.2M, about 13M days, a figure its README
states the source of); a synthetic span of 13M days aborts LibreOffice,
spans of 1M and 100k days open, a 100-day span past 9999-12-31 opens.
Not an emend defect: a LibreOffice limit on the input. The requirement
"lo and lo_in ok on every fixture output" stands; weakening the check
(option b) is refused. The op changes: chart-analytic writes a date
column of its own, serial numbers from ANALYTIC_DATE_FIRST 45292
(2024-01-01) on, one per row, rows 2 to M+1, as plain numbers with no
number format, into the column after the first sheet's last (max_column
+ 1; Y on the fixture, whose first sheet spans A to X), and the DateAxis
chart's categories reference that column; the first LineChart keeps
A2:A(M+1). The header's op text names the constant and the column; the
test line asserts the dated chart's category reference $Y$2:$Y$101
beside what it asserts now. This is a new op commit after d1192d2
(subject "M6 part 3 P2: the chart-analytic op's date column (finding
F1)"), not a rewrite, so the finding and its fix are in the history; its
test line reruns; the five generators' op-dependent lines follow it; the
run log's checks are unchanged. Dry run 1 goes to superseded/ naming F1
and the dry run reruns from the start at the new op commit. The phase
README names F1 with the bisection's spans as a LibreOffice fact; the
negative log is unchanged. Ruling N's comparison is unaffected (the
categories are not among its fields).

P. Finding F2 (after ruling O, measured by hand with the d1 release
binaries on the ruling O fixture): --chart-axis-sample's output aborts
the pinned LibreOffice the same way (lo_in ok, lo error, exit 134, both
targets), because the sample's Bar with the date category axis takes its
categories from column A ($A$2:$A$102) like the other samples, the span
of ruling O on emend's own chart; d1 hid it because the input already
failed. The charts and trend samples are lo and lo_in ok and
byte-identical across targets; on the corpus the axis sample is 27 of 27
lo ok. Ruled: option (a). The axis sample's Bar takes no categories
(categories: None; the writer writes no cat element, which the schema
allows), so its date axis runs over the points' indices; "Harness runs"
reads "a Bar of one series without categories with a date category
axis". Option (b), the sheet's last column, is refused: on a corpus
workbook that column is arbitrary, and so is column A, so (a) alone
removes the hazard from every input. The read-back compares counts, not
references, so ruling M's line is unchanged (series 3, date_axes 1,
bounded_axes 2, read_back 3). Before the commit the implementing session
measures by hand that LibreOffice opens the sample's output on the
fixture (lo-check rc 0) and the README states it. A new tool commit
after 87db736 (subject "M6 part 3 P2: the axis sample's date-axis bar
takes no categories (finding F2)"), not a rewrite; the phase commit
moves to it and the generators carry a seventh role; the run log's
checks are unchanged. Accepted side effects of ruling O: the op's date
column makes the fixture's first sheet 101 rows, so the axis and trend
samples print rows=101 on the fixture (the run's expectation follows,
change NYN); the charts line's digest_formulas changes and the run does
not pin it; fieldcmp 12 of 12. The README names F2 beside F1 as a
LibreOffice fact; the negative log is unchanged. The P3 probe is
unaffected (it runs no LibreOffice).

## Planning verification of P2 (2026-10-21)

Verified from primaries: the chain 74b63de, d8479f5 (doc), 3c1f1f3
(core), d367326 (tool), d1192d2 (op), 87db736 (the op's date column,
finding F1), 3736eea (the tool's date-axis bar without categories,
finding F2, the phase commit), 65703ee (record, 13 files), all local,
the tree clean but for this working spec, origin/main still bcc53a5; the
seven files of records/logs/m6p3-p2 at the report's hashes (gate
dae75dd9, test 195f2b02, negative d0c3a57b, run 51253a45, scan 8ccafd7f,
README 6ccbafe3, the check 096ccbea; the report a21f365e), the four logs
ending "checks all as expected: yes", the scan "RESULT: PASS" with total
hits 0 and structural checks failed 0; every PLANNING_SPEC input this
working spec at b36f6165. The run log: the chartaxis corpus run 27 of 27
ok on both targets, lo ok 27 and lo_in ok 27, the host and i686 outputs
byte for byte the same, 27; the charttrend run rerun byte-identical to
P1's by sha256, 27 of 27 per target; the six openpyxl fixture checks
yes, the axis line as ruling M (charts_before 2, charts_after 5, series
3, rows 101, date_axes 1, bounded_axes 2, read_back 3, chart_parts_after
5) and the trend line rows 101; ruling N's comparison twelve of twelve,
each field printed with both values and both preceding siblings (emend's
majorUnit after crossBetween, openpyxl's after crossAx; majorTimeUnit
after majorUnit on both); the keep 2121 pairs, the 225 new all ok and
ok, none moved. The test log: 1386 on both targets, the feature sets
515, 666, 708, 711, 769 and 1148 (the charts sets +7, off +0). The gate:
the probe 1107076, +4012 over P1's 1103064, told and judged nowhere
here. Ruling C: the trend sample's module doc line reads "the format
sample's". Ruling D: no doc line of chart_write.rs is over 100 columns.
3736eea sets categories: None on the date-axis column chart alone. The
independent check's 146 lines are verbatim in the report, PARTLY on all
three questions; its gaps and disagreements are fixed in the report and
README (the final scan now committed, the "+7" claim corrected to the
charts sets, the session's statements marked, ruling O's wording,
lo-check rc 0 stated) or ruled below. The hash 4454629 the check found
in no primary is a commit object in the repository, the first op commit
rebuilt as d1192d2, a local rewrite before the report (specs/0001), as
9a6a220 was. The README's F1 account states the fixture's span planning
measured and the source of the session's first figure. Privacy: 7
commits, 2115 blobs, 0 findings, PASS. The phase's two independent
checks: check 1 and planning's privacy scan. P2 is DONE. The records and
logs are dated 2026-10-21 (the logged runs began after midnight UTC)
while the phase's commits are of 2026-10-20; accepted.

Q. The axis sample's module doc entry (tools/emend-copy/src/main.rs,
`--chart-axis-sample`) still says each chart is "of one series over B
named by B1 against A2:A(L)"; since 3736eea the column chart's series
has no categories. Carried to P3: its carry commit corrects the entry
(the column chart's series without categories, ruling P), no P2 log
rerun.

R. The finite min and max rule (CHART_VALUE_RANGE on a non-finite min or
max) is kept. "min below max when both are given" is read as a
comparison of finite numbers, and a max or min alone finite as the units
are: a NaN or an infinity written as a val attribute is no number Excel
takes. The spec's sentence is read so; no new message, no new test.

S. test.log's label "of them this phase's logs: 1" counts the working
spec's status line, admitted under PLANNING_SPEC; the check is right,
the label wrong. P3's test generator prints the phase's own paths'
count; the P2 log stands.

## Decisions for the maintainer

1. Which kinds take a trendline and error bars: A (recommended), the
   five 2D kinds the schema gives them to, `Bar`, `Line`, `Scatter`,
   `Area` and `Bubble`, the 3D kinds refused (Excel draws neither on a
   3D chart, and the 3D line's marker was refused the same way in
   part 2); B, the 3D bar, line and area kinds as well, the schema
   allowing them, the drop lines on the 3D line and area too.
2. Where the plot lines live: A (recommended), three booleans on
   `NewChart` (`drop_lines`, `high_low_lines`, `up_down_bars`), refused
   by kind as every option is, the `NewPlot` variants and the probe's
   kind index unchanged; B, fields on the `Line`, `Area` and `Stock`
   variants of `NewPlot`, which changes three variants' shapes, every
   caller's match and the reader-side `ChartKind` comparison nowhere.
3. The date axis: A (recommended), in this part, on the category axis
   of a bar, line, area or stock plot, with a base unit and optional
   major and minor units (openpyxl's `DateAxis`), the chart's
   categories left as the caller references them; B, out of this part
   (the `Unscoped` list), the axis options otherwise as above.
4. The reader: A (recommended), unchanged, as parts 1 and 2 left it,
   the read-back comparing an axis's `min`, `max` and `Date` kind and
   the tests and run logs reading the rest from the written text; B,
   `Series` gains `trendline: bool` and `error_bars: bool` (a presence
   flag each, as `marker` and `labels` are), read under `charts` and
   compared by the read-back, about 300 section bytes more under the
   feature.

## Out of scope

Trendline labels (`trendlineLbl`) and formatting (`spPr`); error bar
formatting; up and down bar formatting (`upBars`, `downBars`); custom
error values given inline (`numLit`); display units (`dispUnits`);
`crossesAt`; minor gridlines; the secondary axis's options on a
combined chart; a date axis on a scatter or bubble plot; reading any
of these back beyond what the reader models today; everything of parts
4 to 6 (`ROADMAP.md`, Milestone 6).
