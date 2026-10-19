# specs/0039: chart kinds and visual formatting (Milestone 6 part 2)

Status: APPROVED 2026-10-19 (Milestone 6 part 2; decisions 1 to 4 A,
typed in the planning session 2026-10-19: a line width in EMU; the
custom split out of scope; a marker on a 3D line refused; a point
carries its index, a colour, an explosion and a marker). Planning
session. Builds on `specs/0035` (the first set), `specs/0036` (the
second set) and `specs/0038` (chartsheets); the writer's forms, checks
and messages named there stand. The part is proven on built packages, on
the corpus run of two new samples, on two generated fixtures and by the
LibreOffice open check, as part 1 was; the Milestone 2 Excel check gates
no push.

## Goal

A consumer writes the two kinds the writer lacks, a surface chart
(`surfaceChart`, the contour form, and `surface3DChart`) and an of-pie
chart (`ofPieChart`, pie of pie or bar of pie, with its split), through
`Package::add_chart` and `Package::add_chartsheet` as it writes the
thirteen kinds of parts 4 and 5; and gives a series of any line-drawing
kind a marker (its symbol, size and colour), a line width and a dash
style, and gives any series a formatted point (`dPt`: a colour, a
slice's explosion, a marker). `charts` reads every written chart back as
far as the reader models it, the output keeps every other entry byte for
byte, LibreOffice opens every corpus output and both fixtures. This is
the visual half of `specs/0036`'s out-of-scope list; the analytic half
(trendlines, error bars, axis scaling) is part 3, the text properties
and the chart style part 4.

## Size and packaging

The budget figure after part 1 is 438960 section bytes against the
ceiling of 458752 (`records/size-m6p1-2026-10-19-4cc4f2e.md`): 19792 of
headroom, held for fixes and part 5. Everything in this part goes under
the feature `charts`, measured by `probe-emend-charts-write-xml`, whose
work gains the three new `NewPlot` variants in its kind index and, on
every kind that takes them, a marker, a line and one formatted point per
series ("The probe"). The `charts` feature costs 119344 section bytes
after part 1 (the probe 1091520); the probe's movement over that is this
part's cost, and the size record states the feature's whole cost, part
1's figure and the delta.

Planning's estimate, priced as `specs/0036` prices (about 500 section
bytes per fixed form, call site and literal together, and per
enumeration arm): the surface plots (two elements, `wireframe`, the
three `axId`s, the series axis the 3D plots already write, the contour's
`view3D` with `perspective`: 6 forms) 3000 to 5000; the of-pie plot
(`ofPieType`, `gapWidth`, `splitType` with `splitPos`, `secondPieSize`,
`serLines`: 6 forms, two arms for the type, three for the split) 3000 to
5000; markers (`marker`, `symbol` with eleven arms, `size`, the marker's
`spPr`: 4 forms) 2500 to 4000; the line (`ln` with `w`, `prstDash` with
eleven arms: 3 forms) 2000 to 3500; points (`dPt`, `idx`, `explosion`,
the point's `spPr` and marker reused: 5 forms and a loop) 3000 to 5000;
the checks and the one new message 2000 to 3000. In all 15500 to 25500
under `charts`. Planning is told the figure in the report at any value;
a cost above 30000 is a finding the report names. The core's growth is
expected 0 and accepted up to 2000. The `images` and `comments` costs
may move (expected 0, accepted up to 1500 each): the size log judges the
costs' movements under their own names, with the probes' growth printed
beside them unjudged (`specs/0038` ruling N), and the copy paragraph
cites the band it is judged by.

## Facts planning checked (2026-10-19)

- The corpus holds no surface and no of-pie chart: the charts survey's
  corpus total `kind_other 0` over 26 charts
  (`records/logs/m5p3-p1/run.log`, planning's expected line; part 1
  reran the survey byte-identical). So the two kinds are proven on built
  packages, on the corpus run of the samples (every workbook gains the
  charts) and by the LibreOffice open check, as the radar, bubble, stock
  and 3D kinds were in `specs/0036`.
- The reader (`crates/emend/src/package/chart.rs` at 5b8b25a) models
  `ChartKind::OfPie`, `Surface` and `Surface3D`, and per series `fill`,
  `smooth`, `marker` (a `marker` element present) and `labels`; it does
  not model a marker's symbol or size, a line's width or dash, a point
  (`dPt`) or an of-pie's split. The read-back of this part compares what
  is modelled (the kind, the series count, the fill, `marker` true); the
  written `ser` bytes are compared whole in the crate tests.
- The writer (`chart_write.rs` at 5b8b25a, 1359 lines): a series' `spPr`
  is written after `tx` with a `solidFill` and, on `Line`, `Line3D`,
  `Scatter`, `Radar` and `Stock`, an `a:ln` holding the same fill; a
  stock series writes `<c:marker><c:symbol val="none"/></c:marker>`
  after it; `dLbls` follows; `view3D` is written for the four 3D kinds
  with `rotX`, `rotY`, `rAngAx`; the series axis (id 30, `serAx`) is
  written by `series_axis` for a `Standard` 3D bar or area and every 3D
  line; `plot_applies` and `takes_position` hold the kind rules;
  `value_element` writes any `val` element.
- openpyxl 3.1.5 on the pinned image (planning's probe, counts and forms
  only): `SurfaceChart` and `SurfaceChart3D` write `surfaceChart` and
  `surface3DChart` holding `wireframe` (when set), the series (each
  `idx`, `order`, `spPr`, `cat`, `val`), and three `axId`s, with
  `catAx`, `valAx` and `serAx` after the plot; the contour's `view3D` is
  `rotX` 90, `rotY` 0, `rAngAx` 0, `perspective` 0, the 3D's `rotX` 15,
  `rotY` 20, `rAngAx` 1; both write empty `floor`, `sideWall` and
  `backWall`. `ProjectedPieChart` writes `ofPieChart` with `varyColors`
  1, the series, `ofPieType` (`pie` or `bar`), `splitType` (`auto`,
  `val`, `percent`, `pos`, `cust`), `splitPos` when given,
  `secondPieSize` 75 and an empty `serLines`, with no axes; openpyxl
  writes `ofPieType` after `dLbls`, out of the schema's order (below).
  `Marker` holds `symbol` (`auto`, `circle`, `dash`, `diamond`, `dot`,
  `none`, `picture`, `plus`, `square`, `star`, `triangle`, `x`), `size`
  2 to 72 and `spPr`; `LineProperties` holds `w` 0 to 20116800 EMU and
  `prstDash` (`solid`, `dot`, `dash`, `lgDash`, `dashDot`, `lgDashDot`,
  `lgDashDotDot`, `sysDash`, `sysDot`, `sysDashDot`, `sysDashDotDot`);
  `DataPoint` holds `idx`, `invertIfNegative`, `marker`, `bubble3D`,
  `explosion`, `spPr`, `pictureOptions`. A line series with a marker, a
  line and a point writes `spPr` (`ln` with `w`, its `solidFill`, then
  `prstDash`), `marker` (`symbol`, `size`, `spPr`), `dPt` (`idx`,
  `spPr`), `val`, `smooth`; a pie series' `dPt` writes `idx`,
  `explosion`, `spPr`.
- The schema (ECMA-376 part 1, 21.2.2; planning's reading through
  openpyxl's per-kind element lists and a third-party binding of
  `CT_OfPieChart`; a LibreOffice refusal of a written form in P1's or
  P2's tests is a finding against this reading): `ofPieChart` holds
  `ofPieType`, `varyColors`, `ser`*, `dLbls`?, `gapWidth`?,
  `splitType`?, `splitPos`?, `custSplit`?, `secondPieSize`?,
  `serLines`*; `surfaceChart` and `surface3DChart` hold `wireframe`?,
  `ser`*, `bandFmts`?, `axId` x3; a surface series holds `idx`, `order`,
  `tx`, `spPr`, `cat`, `val` and no marker, point or labels; `dPt` sits
  after `spPr` (and after `marker` on a line, scatter or radar series,
  after `explosion` on a pie's, after `invertIfNegative` on a bar's or
  bubble's) and before `dLbls`; `dPt` holds `idx`, `invertIfNegative`?,
  `marker`?, `bubble3D`?, `explosion`?, `spPr`?; `marker` holds
  `symbol`?, `size`? (2 to 72), `spPr`?; `a:ln` carries `w` (0 to
  20116800) and holds a fill then `prstDash`; `view3D` takes
  `perspective` after `rAngAx`; `floor`, `sideWall` and `backWall` are
  optional in `chart`.
- `tools/emend-copy` at 5b8b25a: six chart samples
  (`--add-chart-sample`, `--chart-kinds-sample`, `--chart-more-sample`,
  `--chart-3d-sample`, `--chart-labels-sample`, `--remove-chart-sample`)
  and `--chartsheet-sample`, each on the part `first_worksheet` gives
  with its cells read first; `chart_kinds_sample` writes one chart of
  each of six kinds past the last row with one series from columns A and
  B and reads them back. `scripts/openpyxl-ops.py` holds twelve ops,
  `chartsheet` the last.

## Design

### Crate

Under `charts`, in the writer alone: no reader change (the read-back
compares what part 3's reader models). `images` and `comments` gain
nothing.

### The call

`add_chart` and `add_chartsheet` are unchanged in signature. The types
grow (the crate is unpublished, Milestone 4 decision 1 HOLD; the API
listing marks each changed row `changed`, verdict `keep`):

```
pub enum NewPlot {
    ... the thirteen variants of parts 4 and 5 ...
    Surface { wireframe: bool },                 // new
    Surface3D { wireframe: bool },               // new
    OfPie { kind: OfPieKind, split: Option<OfPieSplit> },  // new
}

pub enum OfPieKind { Pie, Bar }                  // new, ofPieType

pub enum OfPieSplit {                            // new, splitType
    Position(u32),      // pos: the last N points go to the second plot
    Value(f64),         // val: points below the value
    Percent(f64),       // percent: points below the percentage
}

pub struct NewSeries<'a> {
    pub name: Option<NewSeriesName<'a>>,
    pub categories: Option<&'a str>,
    pub values: &'a str,
    pub sizes: Option<&'a str>,
    pub color: Option<u32>,
    pub labels: Option<NewLabels<'a>>,
    pub marker: Option<NewMarker>,               // new
    pub line: Option<NewLine>,                   // new
    pub points: &'a [NewPoint],                  // new, dPt
}

pub struct NewMarker {                           // new
    pub symbol: MarkerSymbol,
    pub size: Option<u8>,                        // 2 to 72
    pub color: Option<u32>,                      // 0xRRGGBB
}

pub enum MarkerSymbol {                          // new
    None, Auto, Circle, Dash, Diamond, Dot, Plus, Square, Star,
    Triangle, X,
}

pub struct NewLine {                             // new
    pub width: Option<u32>,                      // EMU (decision 1 A)
    pub dash: Option<DashStyle>,
}

pub enum DashStyle {                             // new, prstDash
    Solid, Dot, Dash, LargeDash, DashDot, LargeDashDot,
    LargeDashDotDot, SystemDash, SystemDot, SystemDashDot,
    SystemDashDotDot,
}

pub struct NewPoint {                            // new, one dPt
    pub index: u32,                              // idx
    pub color: Option<u32>,                      // 0xRRGGBB
    pub explosion: Option<u32>,                  // percent, round plots
    pub marker: Option<NewMarker>,               // line-drawing plots
}
```

`picture` is not a symbol (it needs a `blip`). `MarkerSymbol::None`
writes `symbol` `none` and takes no `size` and no `color`. A `NewLine`
with neither field is refused. A `NewPoint` with no field but its index
is refused; two points of one series with the same index are refused;
the index is not checked against the values (references are not
evaluated, `specs/0035` decision 2; Excel ignores a point past the
data).

### Applicability

The parts 4, 5 and 1 rules stand. The new ones, each refused with "a
chart option does not apply to its kind" unless named otherwise:

- `Surface` and `Surface3D`: one or more series (Excel needs two to draw
  a surface; the file is valid with one, as openpyxl writes it, and the
  rule is the caller's); `wireframe` either way; the axis titles and
  formats as a line's; the series take no `color`, no `labels`, no
  `marker`, no `line` and no `points` (a surface series holds none of
  them); not a combined plot's first or second plot. The plot writes the
  category axis, the value axis and the series axis (id 30).
- `OfPie`: one series, no axis title or format, `varyColors` 1 (as
  `Pie`); `labels.percent` and the pie's four positions; a `color`
  colours every slice; `points` with `explosion` and `color`, no
  `marker`; `split` `None` writes `splitType` `auto`; `Position(n)`
  requires `n` at least 1; `Value(v)` and `Percent(p)` require a finite
  non-negative number, `p` at most 100; `custSplit` is not written
  (decision 2 A). Not a combined plot's first or second plot.
- `marker`: on `Line`, `Scatter`, and `Radar` with `Standard` or
  `Marker` style (a filled radar draws none); refused on `Line3D`
  (decision 3 A), on `Stock` (its series keep the fixed `none` form),
  `Bar`, `Area`, `Pie`, `Doughnut`, `Bubble`, the other 3D kinds and the
  two surfaces. `size` 2 to 72, and not with `MarkerSymbol::None`.
- `line`: on `Line`, `Line3D`, `Scatter`, `Radar` and `Stock`, the kinds
  whose `spPr` already carries the line's colour; refused on every other
  kind. `width` at most 20116800 EMU.
- `points`: on every kind but the two surfaces. A point's `explosion`
  only on `Pie`, `Pie3D`, `Doughnut` and `OfPie`; a point's `marker`
  only where the series takes one, by the series' rules; a point's
  `color` on any kind that takes `points` (on a stock series too, where
  the series' own colour is refused, since a point's fill is what the
  file says; accepted).
- A combined second plot's series take these options by the second
  plot's kind, as they take labels and colours.
- A number out of its range (a marker `size`, a line `width`, an of-pie
  split) is refused with one new message, "a chart value is out of its
  range"; a colour above `0xFFFFFF` keeps the existing option-kind
  refusal.

The check order (`specs/0036` "Applicability") extends: after the first
plot's kind rules and before the combined plot's, each series' marker,
line and points in field order (the marker's symbol, size and colour;
the line's fields; then point by point the index, the colour, the
explosion, the marker); the texts after, unchanged.

### The chart part

Parts 4 and 5's "The chart part" stands. For a `Surface`, in `chart`
after `autoTitleDeleted`:

```
<c:view3D><c:rotX val="90"/><c:rotY val="0"/><c:rAngAx val="0"/>
<c:perspective val="0"/></c:view3D>
```

for a `Surface3D` the 3D form of `specs/0036` (`15`, `20`, `1`). No
`floor`, `sideWall` or `backWall` (optional; the other 3D kinds write
none). The new plots (SERIES as below; `WF` `1` or `0`; `T` `pie` or
`bar`; `ST` `auto`, `pos`, `val` or `percent`; `SP` the position or the
number, written as the caller gave it, an integer for `Position`):

```
<c:surfaceChart><c:wireframe val="WF"/>SERIES
<c:axId val="10"/><c:axId val="20"/><c:axId val="30"/></c:surfaceChart>
<c:surface3DChart><c:wireframe val="WF"/>SERIES
<c:axId val="10"/><c:axId val="20"/><c:axId val="30"/>
</c:surface3DChart>
<c:ofPieChart><c:ofPieType val="T"/><c:varyColors val="1"/>SERIES
<c:gapWidth val="150"/><c:splitType val="ST"/>[<c:splitPos val="SP"/>]
<c:secondPieSize val="75"/><c:serLines/></c:ofPieChart>
```

A surface series writes `idx`, `order`, `tx`, `cat`, `val` and nothing
else. The axes after a surface plot: the category axis and the value
axis as a line's, then the series axis of `specs/0036` (`serAx` 30,
`axPos` `b`, `crossAx` 20). An of-pie writes no axes.

The series additions, in schema order inside `ser`: after `tx`, the
`spPr` grows when a `line` is given (the `ln` takes `w` and, after the
fill, `prstDash`; written with or without a `color`):

```
<c:spPr>[<a:solidFill><a:srgbClr val="RRGGBB"/></a:solidFill>]
<a:ln[ w="W"]>[<a:solidFill><a:srgbClr val="RRGGBB"/></a:solidFill>]
[<a:prstDash val="D"/>]</a:ln></c:spPr>
```

`D` `solid`, `dot`, `dash`, `lgDash`, `dashDot`, `lgDashDot`,
`lgDashDotDot`, `sysDash`, `sysDot`, `sysDashDot`, `sysDashDotDot`.
Then, with a `marker`, in the marker's place (after `spPr` on a line,
scatter or radar series; a stock series keeps its fixed form):

```
<c:marker><c:symbol val="S"/>[<c:size val="N"/>]
[<c:spPr><a:solidFill><a:srgbClr val="RRGGBB"/></a:solidFill></c:spPr>]
</c:marker>
```

`S` `none`, `auto`, `circle`, `dash`, `diamond`, `dot`, `plus`,
`square`, `star`, `triangle`, `x`. Then each point, in the order given,
after the marker's place and before `dLbls`:

```
<c:dPt><c:idx val="I"/>[MARKER][<c:explosion val="E"/>]
[<c:spPr><a:solidFill><a:srgbClr val="RRGGBB"/></a:solidFill></c:spPr>]
</c:dPt>
```

MARKER the marker form above. The schema's `dPt` order is `idx`,
`invertIfNegative`, `marker`, `bubble3D`, `explosion`, `spPr`: the
marker before the explosion (they never meet: no kind takes both). The
reader walks past `dPt` as it walks past any unmodelled child; `marker`
is read true for the series when the series' own `marker` element is
present, not for a point's.

### Errors

One new message, "a chart value is out of its range" (a marker size
outside 2 to 72, a line width above 20116800, an of-pie position below
1, a non-finite or negative split value, a percent above 100). Every
other refusal is an existing message: the option-kind message for every
applicability rule above, the texts, the references, the anchors. The
catalogue in the `package` header lists it under `charts`.

### 32-bit and streaming

Nothing new: the forms are literals and short formatted values; `f64`
values are written with Rust's shortest round-trip form; no part is read
whole.

### Harness runs

`--chart-surface-sample` in `emend-copy` (under `charts`, P1): on each
corpus workbook, as `--chart-kinds-sample` does, four charts past the
last row of the first worksheet, each read back: a `Surface`
(`wireframe` false) and a `Surface3D` (`wireframe` true) of three series
from columns B to D against A, both axis titles; an `OfPie` `Pie` with
no split and an `OfPie` `Bar` with `Position(2)`, one series from column
B, a legend; the tool's line prints the chart count before and after,
the plots counted by kind (surface, surface 3D, of-pie) and the series
count. Expected on the corpus: `--chart-kinds-sample`'s entry pattern
(the sheet, its rels, the drawing, the content types `changed` or
`added`, four chart parts `added`); `lo` and `lo_in` `ok` for every
workbook; the open check of the keep.

`--chart-format-sample` (P2): on each corpus workbook, three charts past
the last row, each read back: a `Line` of two series from columns B and
C, the first with a circle marker of size 7 in a colour, a line of 28575
EMU dashed `Dash` and one point (index 1, a colour, a square marker),
the second with `MarkerSymbol::None` and a `SystemDot` line; a `Pie` of
one series with two points (index 0 exploded 10, index 1 coloured); a
`Scatter` `LineMarker` of one series with a diamond marker of size 5 and
a `LargeDash` line of 12700 EMU. The tool's line prints the chart count
before and after, the series count and the series read back with
`marker` true (3). Expected on the corpus: the kinds sample's entry
pattern with three chart parts `added`; `lo` and `lo_in` `ok`; the open
check of the keep.

Then the two generated fixtures, each under the phase directory, never
committed, their values printable:

- Excel's form (P1), written by the run log's own generator with
  `zipfile`: one worksheet of twelve cells, one drawing holding a
  `surface3DChart` of two series and an `ofPieChart` (`bar`, split `pos`
  2) in schema order (the generator's text printed in the log).
  `--charts-sample` on it on both targets reads both charts (count 2,
  series 3, the kinds `Surface3D` and `OfPie`); `--chart-surface-sample`
  adds four beside them (count 6); the run log's own check with
  `ElementTree` confirms each written plot's children in schema order,
  `ofPieType` first, the surface's three `axId`s and `serAx`, the
  contour's `view3D`.
- openpyxl's form (P2), written by the pinned image through
  `scripts/openpyxl-ops.py chart-format <in> <out>` (the op grows the
  script and its test; a `LineChart` over the input's first sheet with a
  marker, a line width and dash and a `DataPoint`, and a `PieChart` with
  an exploded `DataPoint`): `--charts-sample` on it reads both charts
  (count 2, the line's series `marker` true); `--chart-format-sample`
  adds three beside them (count 5); the run log's own check confirms the
  written `ser` forms against the openpyxl ones field by field where
  both write the field (`w`, `prstDash`, `symbol`, `size`, `idx`,
  `explosion`, the fills), each printed.

`lo` and `lo_in` `ok` on every fixture output. The run log's own checks
print no corpus name, reference, path or text.

### The probe

`PLOTS` gains `Surface { wireframe: true }`, `Surface3D { wireframe:
false }` and `OfPie { kind: Bar, split: Some(Position(1)) }` after
`Area3D`, sixteen kinds, the index the command line names; the second
set's chart takes the kind's series rules (no colour, labels, marker,
line or points on a surface's series; one series on an of-pie, as a
pie); on every kind that takes them, each series gains a marker
(`Circle`, size 7, a colour), a line (28575 EMU, `Dash`) and one point
(index 0, a colour; an explosion of 10 on the round kinds; a marker
where taken), so every arm is live; the chartsheet addition of part 1 is
unchanged. The probe's line count is unchanged (`chartsheet_charts`
last).

## Phases

Each phase as `specs/0001` "Rules for every phase" and part 1's "Phases"
paragraph: a doc commit first, the code commits, one record commit, the
scan last, at most two independent checks, the report under `<phase>`,
then stop. Target root `<home>/.cache/emend-build/m6p2-pN/target`; P1's
gate prunes Milestone 6 part 1's roots (`m6p1-p1`, `m6p1-p2`, `m6p1-p3`,
`m6p1-close`, listed with `du -sb`, removed with `rm -r`). Logs in
`records/logs/m6p2-<phase>/` (`gate.log`, `test.log` on both targets
with the feature sets off, `charts`, `charts,images`, `charts,comments`,
`charts,read` and every feature, `negative.log`, `run.log`, `scan.log`,
`size.log` in P3); the scan's denylist form whole-word. No push during
the part; the close-out push follows P3.

Part 2's first doc commit carries: this spec; `specs/0038` with its
close-out verification and rulings R to T; `ROADMAP.md` with part 1's
heading DONE and PUSHED and this part's rows; all three as planning left
them, by hash.

Generator rules carried from `specs/0038` and its rulings into this part
and every later one: those `specs/0038` "Phases" carries from
`specs/0037` (AK a, AK b, AC d, Z, AA, AD); a generator's own
placeholder check prints its control in its log (ruling D); RERUN_FILES
also names the superseded script that wrote the attempt (ruling F); the
size log judges the `images` and `comments` costs' movements under their
own names, the probes' growth printed beside them unjudged, and the copy
paragraph cites the band it is judged by (ruling N); the close-out
generator corrects the two cosmetic leftovers of part 1's close-out
scripts (ruling R).

### P1: the surface and of-pie kinds

The doc commit; the prunes; then `NewPlot::Surface`, `Surface3D` and
`OfPie` with `OfPieKind` and `OfPieSplit`, their rules, their forms, the
surface axes and `view3D`, the new message; `--chart-surface-sample`.
Tests on built packages, each written chart read back by `charts` and
compared and its `ser` and plot bytes compared whole: a contour and a 3D
surface of three series, each `wireframe` value, the axes (three ids,
`serAx` last); an of-pie of each type and each split form, `splitPos` as
given; a surface series with a colour, labels, a marker, a line or a
point refused; an of-pie with two series, an axis title, a position of
0, a negative value, a percent above 100 or a combined plot refused,
each with the message named; a surface as either combined plot refused;
`add_chartsheet` of a surface. The gate as part 1 P1's plus the prunes.
The negative log through the binary: every message of the part reachable
there, else why not, with the covering tests named (the samples build
their calls from constants; part 1's pattern). The run log:
`--chart-surface-sample` on both targets over the corpus and
`--charts-sample` and `--chart-surface-sample` over the Excel-form
fixture, the checks as "Harness runs", recorded as
`records/m6p2-p1-<utc-date>-<shortsha>.md` and `.txt`; the open record.

### P2: markers, lines and points; the openpyxl fixture; the catalogue

The doc commit (the P1 verification rulings); the carry; then
`NewMarker`, `MarkerSymbol`, `NewLine`, `DashStyle` and `NewPoint` on
`NewSeries`, their rules and forms, the check order;
`--chart-format-sample`; the `chart-format` op of
`scripts/openpyxl-ops.py` with its test line in
`scripts/openpyxl-ops-test.sh`; the `package` header's catalogue and
limits for this part. Tests on built packages: a line series with each
marker symbol, with and without size and colour, the bytes compared
whole; `None` with a size refused; a size of 1 and of 73 refused; a line
with width, with dash, with both, with neither refused, a width above
the limit refused, with and without the series' colour; a marker on each
kind that takes one and refused on each that does not (the stock's fixed
form kept); a line on each kind that takes one and refused on the
others; points on a bar, a line (with a marker), a pie (with an
explosion), a scatter, a stock (a colour) and a bubble, in the order
given; a point with no field, two points of one index, an explosion on a
bar, a marker on a bar's point, a point on a surface, each refused; the
combined second plot's series with each option by its kind; the check
order test extended (`the_new_texts_follow_the_check_order`'s pattern).
The run log: the fixture written by the image (the plan printed first,
rootless, the image id checked against
`records/openpyxl-image-2026-10-16.md`), `--charts-sample` and
`--chart-format-sample` on it on both targets, the checks as "Harness
runs"; `--chart-format-sample` over the corpus on both targets; the
corpus run of P1 rerun byte-identical by sha256; recorded as
`records/m6p2-p2-<sample>-<utc-date>-<shortsha>.md` and `.txt`; the open
record.

### P3: the probe, the size record, the API listing, the CHANGELOG

The doc commit (the P2 verification rulings); the carry; the probe's
three kinds and the series options as "The probe" says; the size log,
recorded as `records/size-m6p2-<utc-date>-<shortsha>.md`, with the
`charts` cost, its movement over part 1's 119344 judged against the
estimate and the 30000 line, the `images` and `comments` costs'
movements and the core's growth each printed and judged (ruling N's
form); the API listing rerun, recorded as
`records/api-review-m6p2-<utc-date>-<shortsha>.md`; `CHANGELOG.md`'s
"Added" gains one bullet citing this spec. The negative log through the
binary for every message of the part reachable there. The run log: the
P2 runs rerun with the size log's release binaries, byte-identical by
sha256 (the openpyxl fixture by part 1's P3 reading, `specs/0038` ruling
P), recorded as `records/m6p2-p3-<sample>-<utc-date>-<shortsha>.md` and
`.txt`; the open record of the part's keep.

### Close-out push (after P3)

As part 1's (`specs/0038` "Close-out push (after P3)" and its rulings R
to T): the generator transforms part 1's close-out scripts, its numbered
changes continuing from 49, correcting the two cosmetic leftovers ruling
R names; the doc commit holds this spec and `ROADMAP.md`; the base is
part 1's close-out doc commit, named by its subject "M6 part 1
close-out: commit planning docs (the P3 verification and rulings)";
`PREV_RECORD_PATTERN` is `^M6 part 1 close-out record`, in the range
exactly once; the close-out doc commit's subject is "M6 part 2
close-out: commit planning docs (the P3 verification and rulings)";
`CARGO_TARGET_DIR` printed and checked as
`<home>/.cache/emend-build/m6p2-close/target`; no Excel-record check;
the scan's denylist form whole-word; the range holds no archive (change
48's check); the record commit "M6 part 2 close-out record: the push,
the CI run and the scan" holding the three logs only; pre mode on a
clean tree; one independent check before the report. The push carries
part 1's close-out record commit and this part's commits, on planning's
word and the maintainer's authorisation typed in the implementing
session. The hashes are filled in at the briefing.

## Decisions for the maintainer

1. The unit of a line's `width`: A (recommended), EMU as the file's `w`
   attribute and openpyxl's `LineProperties.w` take it (12700 per point;
   28575 is Excel's 2.25 pt default), checked at most 20116800; B,
   hundredths of a point (a `u16`, 225 for 2.25 pt), converted to EMU at
   the write.
2. An of-pie's custom split (`splitType` `cust` with `custSplit` listing
   the second plot's points): A (recommended), out of scope
   (`OfPieSplit` has the three forms Excel's dialog offers by rule;
   `cust` is a later part's); B, a fourth form `Custom(&[u32])` writing
   `custSplit` with one `secondPiePt` per index.
3. A marker on a 3D line's series: A (recommended), refused (Excel draws
   no marker on a `line3DChart`, though the schema takes the element);
   B, accepted and written as a line's.
4. What a formatted point carries: A (recommended), its index, a colour,
   an explosion on the round kinds and a marker where the series takes
   one, as "The call" says; B, its index and a colour only (the
   explosion and the point marker a later part's).

## Out of scope

`custSplit` (decision 2); the `picture` marker symbol; a marker's line
(its `spPr` holds a fill alone); a line's cap, compound and custom dash;
`bandFmts` on a surface; `floor`, `sideWall` and `backWall`; a point's
`invertIfNegative`, `bubble3D` and `pictureOptions`; the series-level
`invertIfNegative`, `shape` and `pictureOptions`; gradient and pattern
fills on a chart; reading a marker's symbol or size, a line's width or
dash, a point or a split (the reader models `marker` as a flag);
trendlines, error bars, drop lines, up-down bars and axis scaling (part
3); `txPr` and the chart `style` (part 4); editing a kept chart in place
(part 4).
