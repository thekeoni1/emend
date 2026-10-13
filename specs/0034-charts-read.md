# specs/0034: charts read: the feature `charts`

Status: APPROVED 2026-10-13 (Milestone 5 part 3; decisions 1 to 3
decided A by the maintainer 2026-10-13, typed in the planning
session).

Milestone 5, "charts and read breadth", was approved by the
maintainer 2026-10-11 (`ROADMAP.md`, "Milestone 5"): reading first,
then charts; charts a Cargo feature `charts` of this crate, off by
default, measured in the size record like every other feature; the
Milestone 2 ceiling kept for the core; the Milestone 2 Excel check
gates no push in this milestone. This part is the third of six and
the first of the three chart parts: every chart a worksheet's
drawing holds, read typed. Reads only; no writer changes. Parts 4
and 5 (charts written) build on this part's model: what this part
reads is what those parts write, so the model's names are chosen
for both.

## Goal

A consumer that opens a workbook it did not write learns what each
sheet's charts show: for every chart, its anchor on the sheet, its
kind (one or more plots, each bar, line, pie, doughnut, scatter,
area, radar, bubble, stock or surface, flat or 3D), its title, its
series with the cell ranges they draw and the name each carries,
its axes with their titles, number formats and bounds, its legend,
and whether it draws a pivot table. Every string the reader returns
is counted or digested by the sample on the corpus, never printed;
every read part is passed through byte for byte.

## Size and packaging

The budget figure after part 2 is 436992 section bytes against the
ceiling of 458752 (`records/size-m5p2-2026-10-13-f35de0d.md`):
21760 of headroom, held for fixes; the core's growth in part 2 was
0. The new feature `charts` is measured by a new probe,
`probe-emend-charts-write-xml` (the format probe with the feature,
then `charts` of its first sheet, the chart count and the series
count printed), the twenty-ninth, classified as the others are.
Planning's estimate, priced from part 2's readers (the pivot and
external link readers with their types and the survey cost 42556
in all) and from the `images` feature's drawing walk (the `images`
feature costs 67832, most of it the writer): 20000 to 35000 for the
anchor walk shared with `images`, the chart part reader (a dozen
elements, five enumerations, the series and axis models) and the
types. Planning is told the figure in the report at any value; a
feature cost above 45000, or core growth above 2000, is a finding
the report names. The core's growth is what the feature gates add,
expected 0. The `images` feature's own cost must not move by more
than the gate-widening of the shared walk (expected 0, accepted up
to 500): the drawing walk it already carries gains a second gate,
no code.

## Facts planning checked (2026-10-13)

- The corpus, counted by planning with Python's `zipfile`,
  `ElementTree` and regular expressions over every regular file of
  `/home/dev/emend-testing/corpus` (26 `.xlsx` and 1 `.xltx`), the
  script under planning's job directory, never committed: 26 chart
  parts (`xl/charts/chart<n>.xml`, content type
  `drawingml.chart+xml`) in 11 workbooks, at most 4 in one; the
  harness's `chart` family of 56 entries is these 26 plus 15 chart
  style and 15 chart colour-style parts (`chartStyle` and
  `chartColorStyle` relationships of 15 charts in 7 workbooks); 1
  `chartUserShapes` relationship in 1; `chartex` 0; chartsheets 0.
  Every one of the 26 is reached from a worksheet's drawing: the
  kinds sample of part 2 (`records/m5p2-p3-kinds-2026-10-13-f35de0d`)
  sums `sheet_charts` to 26 and `charts` to 26 over the 27; of the
  38 drawing parts 37 are reached from a sheet, the 38th the user
  shapes drawing of a chart.
- Plot kinds (the children of `plotArea` named `*Chart`): `barChart`
  17 in 10 (`barDir` `bar` 11 in 5, `col` 6 in 6; `grouping`
  `clustered` 14, `stacked` 3, `standard` 4 over the bar and line
  plots), `lineChart` 4 in 4, `pieChart` 3 in 3, `scatterChart` 3
  in 3 (`scatterStyle` on each), `doughnutChart` 1 in 1 (`holeSize`,
  `firstSliceAng`); no 3D, area, radar, bubble, stock, surface or
  `ofPie` plot. 24 charts hold one plot and 2 hold two (combined:
  a bar and a line in each). Series per chart: 1 in 19 charts, 2 in
  4, 3 in 2, 5 in 1; 38 series in all (ruling D).
- Series: `tx` from `strRef` 26 in 8, a literal `v` 9 in 3, none 3
  in 2; `cat` `strRef` 28 in 10 and `strLit` 1 in 1; `val` `numRef`
  33 in 10 and `numLit` 1 in 1; the scatter plots' `yVal` `numRef` 4
  in 3, their categories in `cat`, no `xVal` anywhere (ruling D);
  every `numRef`, `strRef` and
  `multiLvlStrRef` carries an `f` and a cache (65 in 10; none
  without). Formatting: `spPr` with `solidFill` 30 in 10 (`srgbClr`
  12 in 4, `schemeClr` 18 in 6) and `ln` 35 in 10; `dLbls` 15 in 5;
  `dPt` 11 in 6; `marker` 8 in 4; `smooth` 8 in 4; `explosion` 2 in
  2; `invertIfNegative` 26 in 10; `extLst` 26 in 7; no `trendline`
  and no `errBars`.
- Titles: a chart `title` 11 in 5, each `rich` (none `strRef`, none
  without `tx`); `autoTitleDeleted` `0` 14 in 7 and `1` 12 in 6;
  an axis `title` 2 in 1.
- Axes: `catAx` 20, `valAx` 24, no `dateAx` and no `serAx`; 12 of
  the 136 `axId` and `crossAx` values are negative, in 1 workbook
  (ruling C);
  `numFmt` 43 in 10; `majorGridlines` 17 in 9; `scaling` `min` or
  `max` 6 in 3; `delete` `1` 5 in 3.
- The chart: `legend` 15 in 8; `plotVisOnly` 26, `dispBlanksAs` 25,
  `roundedCorners` 26, `lang` 26, `spPr` 26 in 11; `txPr` 16 in 7;
  `date1904` and `printSettings` 16 in 7; `pivotSource` 5 in 2 (a
  pivot chart); `externalData` 0; `mc:AlternateContent` somewhere in
  the chart part 16 in 7; `extLst` on `chartSpace` 6 in 3, none on
  `chart` or `plotArea`.
- The drawings: every chart sits in a `graphicFrame` of a
  `twoCellAnchor` (26; `editAs` `oneCell` 21, absent 5; none
  `oneCellAnchor` or `absoluteAnchor`), its `graphicData` `uri` the
  chart namespace and its `c:chart` carrying `r:id`; every such
  frame has `xfrm` and `cNvPr`. Beside them the drawings hold `pic`
  21, `sp` 45 in 7, `grpSp` 7 in 1, `cxnSp` 2 in 1, a root-level
  `mc:AlternateContent` 3 in 2, and 4 `twoCellAnchor` holding
  something else; the user shapes drawing holds a `relSizeAnchor`.
  The drawings' relationships: `chart` 26, `image` 34, `hyperlink`
  36.
- The crate: `Package::images` (feature `images`) already walks a
  worksheet's drawing part by `read_drawing_roots` and `read_images`
  in `package.rs`, naming the anchors (`Anchor`, `EditAs`) and
  taking the first `Choice` of a root `AlternateContent`
  (`specs/0028` P3 ruling 1); `sheet_parts` (feature `read`) counts
  the drawings' `chart` relationships; the constant `CHART` exists
  under `read`. The chart part itself is read by nothing.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. A new Cargo feature
`charts`, off by default, with no dependency of its own, measured
beside the core as `read` is: one new call on `Package`, its types,
and the message, all gated. The core builds, tests and links
without it; the tool turns it on. `read_drawing_roots`, the anchor
model it fills and `Anchor` and `EditAs` gain the gate
`any(feature = "images", feature = "charts")`; nothing else gated
under `images` widens, and nothing in the chart reader needs
`images` or `read` (the constant `CHART` gains `charts` to its
gate as `PIVOT_TABLE` gained `read`). Nothing existing changes
signature. The module `package/chart.rs`, as `pivot.rs` and
`extlink.rs` are.

### The call

`Package::charts(&mut self, part: &[u8])` returning
`Result<Vec<ChartEntry>, Error>`, for a worksheet part: each root
child of the drawing part the sheet's `<drawing>` names, in document
order, that is an anchor
(`oneCellAnchor`, `twoCellAnchor`, `absoluteAnchor`, or the first
such in a root `AlternateContent`'s first `Choice`, as `images`
takes them) holding a `graphicFrame` whose `graphicData` holds a
`c:chart`, the chart part its `r:id` names through the drawing's
rels read and modelled. An anchor holding a picture or a shape
gives none; a `graphicFrame` holding something other than a chart
(a slicer, a timeline, a diagram) gives none. A snapshot of the
file, not of the pending edits; empty for a part with no
`<drawing>`.

```
pub struct ChartEntry {
    pub part: Vec<u8>,            // the chart part's entry name
    pub anchor: ChartAnchor,
    pub chart: Chart,
}
pub struct ChartAnchor {
    pub kind: Anchor,            // shared with `images`
    pub from: (u32, u32),        // col, row, zero-based; (0,0) absolute
    pub to: Option<(u32, u32)>,  // a two-cell anchor's `to`
    pub edit_as: Option<EditAs>, // a two-cell anchor's `editAs`
}
pub struct Chart {
    pub title: Option<String>,    // "Titles" below
    pub auto_title_deleted: bool, // `autoTitleDeleted` (default false)
    pub plots: Vec<Plot>,         // the `plotArea`'s `*Chart` children
    pub axes: Vec<ChartAxis>,     // the `plotArea`'s axes in order
    pub legend: Option<LegendPosition>,
    pub pivot_source: Option<String>, // `pivotSource/name`
    pub style: Option<u32>,       // `c:style` `val` (the `mc` form too)
}
pub struct Plot {
    pub kind: ChartKind,
    pub bar_direction: Option<BarDirection>, // bar plots
    pub grouping: Option<Grouping>,          // bar, line, area
    pub scatter_style: Option<ScatterStyle>, // scatter plots
    pub vary_colors: bool,                   // `varyColors`
    pub series: Vec<Series>,
    pub axis_ids: Vec<i64>,                  // `axId`s (ruling C)
}
pub enum ChartKind {
    Bar, Bar3D, Line, Line3D, Pie, Pie3D, Doughnut, OfPie, Scatter,
    Area, Area3D, Radar, Bubble, Stock, Surface, Surface3D,
}
pub enum BarDirection { Bar, Column }
pub enum Grouping { Standard, Clustered, Stacked, PercentStacked }
pub enum ScatterStyle {
    None, Line, LineMarker, Marker, Smooth, SmoothMarker,
}
pub struct Series {
    pub index: u32,               // `idx`
    pub order: u32,               // `order`
    pub name: Option<SeriesName>,
    pub categories: Option<Data>, // `cat`, or a scatter's `xVal`
    pub values: Option<Data>,     // `val`, or a scatter's `yVal`
    pub sizes: Option<Data>,      // a bubble's `bubbleSize`
    pub fill: Option<u32>,        // `spPr/solidFill/srgbClr`, 0xRRGGBB
    pub smooth: bool,             // `smooth` `val` (default false)
    pub marker: bool,             // a `marker` element present
    pub labels: bool,             // a `dLbls` element present
}
pub enum SeriesName { Reference(String), Text(String) }
pub enum Data {
    // `*Ref`: its `f`, and its cache's `ptCount` when a cache is there
    Reference { formula: String, cached: Option<u64> },
    // `numLit` or `strLit`: its `ptCount`
    Literal { count: u64 },
}
pub struct ChartAxis {
    pub id: i64,                  // `axId` (ruling C)
    pub kind: AxisKind,           // Category, Value, Date, Series
    pub position: AxisPosition,   // `axPos`: Bottom, Left, Right, Top
    pub crosses: i64,             // `crossAx` (ruling C)
    pub title: Option<String>,
    pub number_format: Option<String>, // `numFmt` `formatCode`
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub deleted: bool,            // `delete` `val`
    pub major_gridlines: bool,
}
pub enum LegendPosition { Bottom, Left, Right, Top, TopRight }
```

`fill` is the six hex digits of `srgbClr`'s `val` as a number, no
alpha (a `schemeClr` or any other fill gives `None`; the `read`
feature's `ResolvedColor` is not used, since a chart colour needs
no palette). Every string is the file's text as written: a formula
without its leading `=`, as `f` holds it; a sheet name inside a
formula is content and the sample digests the whole formula.

### Titles

A chart's or an axis's `title` reads as the concatenation of the
`a:t` text of every run of its `tx/rich` paragraphs, paragraphs
joined by a newline, or the text of `tx/strRef/strCache`'s first
point, or `None` for a `title` with no `tx` (an automatic title) or
no `title` at all; `auto_title_deleted` says the automatic title
is off. Formatting inside a run is not read.

### Series data

`cat`, `val`, `xVal`, `yVal` and `bubbleSize` each hold one of
`numRef`, `strRef`, `multiLvlStrRef`, `numLit` or `strLit`. A
reference form reads its `f` (required) and its cache's `ptCount`
`val` if a cache is present; a literal form reads its `ptCount`
(0 when absent). The cached values are never read (decision 3 A).
`cat` and `xVal` fill `categories`; `val` and `yVal` fill `values`;
a series with both `cat` and `xVal` is refused.

### Unmodelled content

Everything else is walked past by `past`: `dPt`, `dLbls`'s
content, `trendline`, `errBars`, `extLst`, `txPr`, `spPr` other
than the solid fill, `view3D`, `floor`, `sideWall`, `backWall`,
`dTable`, `printSettings`, `userShapes`, `externalData`,
`clrMapOvr`, the `upDownBars`, `hiLowLines` and `dropLines`, and
every `mc:AlternateContent` below the root (its `Choice` is not
entered; the `Fallback` is not entered). The chart style, colour
style and user shapes parts are never read.

### Errors

One new `Error::Package` message, in the catalogue marked by
feature:

`charts`:
- "a chart cannot be read": a `graphicFrame`'s `c:chart` without
  `r:id` or naming no `chart` relationship, or one whose part is
  missing (the relationship is internal; `TargetMode` `External`
  refused); a root other than `chartSpace`; no `chart` or no
  `plotArea`; a `*Chart` child with no `ser` list is allowed but a
  `ser` with no `idx` or `order` is refused; a reference form with
  no `f`; a series with both `cat` and `xVal`; an axis with no
  `axId`, `axPos` or `crossAx`; an `axPos`, `grouping`, `barDir`,
  `scatterStyle`, `legendPos` or `crosses` value outside its
  enumeration; a `ptCount` or `val` that is not a number where one
  is required; an `axId` naming no axis of the chart; an `idx` or
  `order` that is not a number; an `srgbClr` `val` that is not six
  hex digits; a boolean `val` outside `0`, `1`, `true` and `false`
  (P1 ruling F); a `crosses` with no `val`, a `numFmt` with no
  `formatCode` and a `pivotSource` with no `name` (P2 ruling N).

Shared: "no such part"; "a worksheet has two drawing elements" and
"a drawing part or its relationship cannot be read" as `images`
raises them, now under either feature; part 2's unbound-prefix
refusal; `Error::Xml` for bytes that are not UTF-8 or not well
formed. Nothing is checked twice.

### 32-bit and streaming

The chart part is streamed by `Stream` and the model held, bounded
by the chart's series, axes and strings (the corpus's largest
chart part is read for its size in P1's report; the caches are
walked past, never held). The drawing part is read as `images`
reads it. Every count is `u32` or `u64`.

### Harness runs

`tools/emend-copy` gains one operation, given after the two paths
as the others are; it takes no reference, name or number from its
command line. No reference, no name, no path and no text from the
corpus is printed: the sample prints counts, flags and digests
(64-bit FNV-1a over each value and a newline, as part 2's samples
digest). A chart title, a series name, a formula, a number format
and a pivot source are corpus content and are never printed.

- `--charts-sample <in> <out>`: writes the input back unedited;
  `charts` of every worksheet of `Workbook::sheets`; prints
  `charts` with `sheets`, `charts`, `plots`, `kind_bar`,
  `kind_line`, `kind_pie` (each with its 3D kind, P1 ruling H),
  `kind_doughnut`, `kind_scatter`,
  `kind_other`, `combined` (charts with two or more plots),
  `bar_dir_bar`, `bar_dir_col`, `grouping_clustered`,
  `grouping_stacked`, `grouping_standard`, `grouping_percent`,
  `series`, `name_ref`, `name_text`, `unnamed`, `cat_ref`,
  `cat_lit`, `val_ref`, `val_lit`, `sizes`, `cached` (references
  with `cached` `Some`), `fills`, `smooth`, `markers`, `labels`,
  `titles`, `auto_title_deleted`, `axes`, `axis_cat`, `axis_val`,
  `axis_other`, `axis_titles`, `axis_numfmt`, `axis_bounds` (`min`
  or `max` `Some`), `axis_deleted`, `gridlines`, `legends`,
  `pivot_charts`, `styles`, `anchor_two`, `anchor_one`,
  `anchor_abs`, `edit_as_onecell`, `edit_as_twocell`,
  `edit_as_abs`, `digest_titles`, `digest_names`,
  `digest_formulas`, `digest_numfmts` and `digest_sources`. Expected
  results over the corpus: every entry `identical`,
  `files_identical` `yes`; `charts` summed 26 over 11 workbooks and
  equal in every workbook to the kinds sample's `sheet_charts`
  (`records/m5p2-p3-kinds-2026-10-13-f35de0d.txt`'s run directory,
  the per-workbook value read from part 2's record and compared in
  the run log); `plots` 28; `combined` 2; `series` 38 (ruling D);
  `kind_bar`
  17, `kind_line` 4, `kind_pie` 3, `kind_scatter` 3,
  `kind_doughnut` 1, `kind_other` 0; `name_ref` 26, `name_text` 9,
  `unnamed` 3 (the 38); `titles` 11; `axes` 44 (`axis_cat` 20,
  `axis_val` 24, `axis_other` 0); `legends` 15; `pivot_charts` 5;
  `anchor_two` 26, `edit_as_onecell` 21. The keep and the edit
  open check as every run log.

## Phases

As `specs/0001` and part 2's: a doc commit first, code commits, one
record commit, the scan last, at most two independent checks per
phase, a report, STOP. One target root per phase,
`<home>/.cache/emend-build/m5p3-<phase>/target`. P1's gate prunes
part 2's four roots (`<home>/.cache/emend-build/m5p2-p1/target`,
`m5p2-p2/target`, `m5p2-p3/target` and `m5p2-close/target`,
`specs/0033` close-out ruling 5), each listed with `du -sb` and
removed with `rm -r`; the close-out's `gen.py` and logs stay. Every
rule of part 2's "Phases" paragraph carries: the three logged
scripts in the phase directory with `scripts/counting.sh` copied by
`git show`; a dry run before each logged run; a refused or failed
attempt hashed in the step's own log through `RERUN_NOTE` and
`RERUN_FILES` before a rerun (`specs/0032` P3 verification ruling 1,
`specs/0033` close-out ruling 1); `input: TMPDIR=<scratch>/tmp` and
the `mktemp` check with controls; `python3 -I -B` everywhere and
the gate's `-B` check; the gate's line on `scripts/counting.sh`'s
own `mktemp` (`specs/0033` P2 ruling 7). Logs in
`records/logs/m5p3-<phase>/` (`gate.log`, `test.log` on both targets
with the feature sets off, `charts`, `charts,images`, `charts,read`
and every feature, `negative.log`, `run.log`, `scan.log`,
`size.log` in P3); the scan's denylist form whole-word. No push
during the part; the close-out push follows P3 and carries the
record commit 017e480 of part 2's close-out with this part's.

Part 3's first doc commit carries: this spec; `ROADMAP.md` with
part 2's Close row DONE (record 017e480) and this part's rows;
`specs/0033-read-breadth-survey.md` gaining "Planning rulings on
the close-out (2026-10-13)", the text of planning's close-out
rulings 1 to 7 copied verbatim from its cache.

### P1: the feature, the anchors, the plots and series

The doc commit; the prunes; then the feature, the gate widening,
`ChartEntry`, `ChartAnchor`, `Chart` with its `title`,
`auto_title_deleted` and `plots` fields (the `axes`, `legend`,
`pivot_source` and `style` fields are added to the struct in P2,
so that each phase's API listing names what that phase fills; no
field is present and unfilled), `Plot`, `Series`, `SeriesName`,
`Data` and the enumerations, `charts`, the message, and
`--charts-sample` with the fields P1 fills (the P2 fields added to
its line in P2).
Tests on built packages: a sheet whose drawing holds a chart in a
two-cell anchor with `editAs`, one in a one-cell anchor, one in an
absolute anchor, a picture and a shape between them, and a
`graphicFrame` holding no chart; a chart of each `ChartKind`
(sixteen parts, each with one series, the kind read); a combined
chart; a bar plot with each `barDir` and each `grouping`; a scatter
plot with each `scatterStyle`, `xVal` and `yVal`; a bubble plot
with `bubbleSize`; a series with each name form and none; each
data form with and without a cache; a rich title of two paragraphs,
a `strRef` title, a title with no `tx`, `autoTitleDeleted`; a
solid `srgbClr` fill, a `schemeClr` fill (`None`), `smooth`,
`marker`, `dLbls`; unmodelled children walked past (`dPt`,
`trendline`, `errBars`, `extLst`, an `mc:AlternateContent` inside
`spPr`); the refusals of "Errors" reachable in P1 (the `r:id`
cases, the root, no `chart`, no `plotArea`, `ser` without `idx`,
a reference without `f`, both `cat` and `xVal`, an enumeration
value outside its set, a `ptCount` that is not a number); the
strict namespace of the drawing relationship; a sheet with no
`<drawing>`; the tool's line. The gate as part 2 P1's (the crate
built and tested with the feature sets above on both targets, the
`pub fn` lines listed off and on) plus the prunes. The negative
log through the binary: the sample on a fixture whose chart's
`c:chart` names no relationship (refused, with a control). The run
log: `--charts-sample` on both targets over the corpus, the checks
as "Harness runs" for the P1 fields, the keep and the edit open
check, recorded as `records/m5p3-p1-<utc-date>-<shortsha>.md` and
`.txt`; the largest chart part's size in the report.

### P2: axes, legend, pivot source, style; the catalogue

The doc commit (the P1 verification rulings); the carry (P1
rulings E and G: the negative `axId` joined to a negative
`crossAx`, and `multiLvlStrRef` without a cache);
then `ChartAxis`, `AxisKind`, `AxisPosition`, `LegendPosition`, the
`axes`, `legend`, `pivot_source` and `style` fields, the axis
refusals, the sample's P2 fields; then the `package` header's
catalogue and limits for this part (the message by feature; the
title text rules; the cached values never read; the style, colour
style and user shapes parts never read; the `Fallback` never
entered). Tests on built packages: each `AxisKind` and each
`AxisPosition`; an axis with a title, a `numFmt`, `min` and `max`,
`delete` `1`, `majorGridlines`; a `legendPos` of each value and a
legend with none (`Right`, the schema default); `pivotSource`;
`c:style` plain and inside `mc:AlternateContent` at the root (the
`Choice` form read, the `Fallback` not); the refusals (no `axId`,
no `axPos`, no `crossAx`, an `axPos` of `middle`, a `legendPos` of
`centre`, an `axId` naming no axis); the tool's line. The gate as
P1's. The negative log through the binary: the sample on a fixture
whose axis lacks `axPos` (refused, with a control). The run log:
`--charts-sample` on both targets over the corpus with every field,
the checks as "Harness runs", recorded as
`records/m5p3-p2-<utc-date>-<shortsha>.md` and `.txt`.

### P3: the probe, the size record, the API listing, the CHANGELOG

The doc commit (the P2 verification rulings); the carry (P2
rulings T, U and X: `c:style` inside the `Choice`, the header's
three test names, the `min` and `max` spellings);
then `probe-emend-charts-write-xml` as "Size and packaging" says
(its manifest, `required-features`, its classification); the size
log, recorded as `records/size-m5p3-<utc-date>-<shortsha>.md`, with
the `charts` cost, the `images` cost's movement (expected 0,
accepted to 500) and the core's growth (expected 0, accepted to
2000) each printed and judged; the API listing rerun as part 2 P3
did, recorded as `records/api-review-m5p3-<utc-date>-<shortsha>.md`
(the new items listed, the count stated, the gate-widened items
listed as widened); `CHANGELOG.md`'s "Added" gains one bullet
citing this spec, under `specs/0031`'s citation check. The
negative log through the binary for every message of the part
reachable there, the unreachable ones stated with the test that
covers each. The run log: `--charts-sample` on both targets over
the corpus with the size log's release binaries, byte-identical to
P2's run by sha256 (P2 runs the release binaries, as P1 did; P1
ruling K), and `--kinds-sample` once more with the
release binaries (byte-identical to part 2's record by sha256),
recorded as `records/m5p3-p3-<sample>-<utc-date>-<shortsha>.md`
and `.txt`. The open record of the part's keep.

### Close-out push (after P3)

As part 2's (`specs/0033` "Close-out push (after P3)" and its
close-out rulings): the generator transforms part 2's scripts
(`gen.py` b3ca0a8c and the three it printed, carried cmp-equal),
its numbered changes continuing from 37; the doc commit holds this
spec and `ROADMAP.md`; the base is part 2's close-out doc commit
742051b, named by its subject ("M5 part 2 close-out: commit
planning docs (the P3 verification and rulings)");
`PREV_RECORD_PATTERN` is `^M5 part 2 close-out record` (017e480),
in the range exactly once; the close-out doc commit's subject is
"M5 part 3 close-out: commit planning docs (the P3 verification and
rulings)"; `CARGO_TARGET_DIR` printed and checked as
`<home>/.cache/emend-build/m5p3-close/target`; no Excel-record
check, the stray-record count 0 kept; the scan's denylist form
whole-word; the record commit "M5 part 3 close-out record: the
push, the CI run and the scan" holding the three logs only; pre
mode on a clean tree; one independent check before the report. The
push carries 017e480 and this part's commits, on planning's word
and the maintainer's authorisation typed in the implementing
session.

## Planning rulings during P1 (2026-10-13)

A. The shared walk ("Crate"): planning read `read_drawing_roots`,
   `DrawingRoot` and `read_images` in `package.rs` after the doc
   commit (37cf313, re-made as 266c410 with the same tree).
   `read_drawing_roots` records each root's byte
   span, whether it is an anchor, and its `r:` attribute values, for
   `remove_image`; it reads no `from`, `to` or `editAs` and no
   `graphicFrame`; the anchor reading `images` uses lives in
   `read_images`, which is picture-specific. Widening either to
   `charts` would gate dead code under a charts-only build. Ruled:
   `read_drawing_roots`, `DrawingRoot` and `read_images` stay under
   `images`; `package/chart.rs` streams the drawing itself, taking
   the anchors exactly as "The call" says (the root anchors and the
   first anchor of a root `AlternateContent`'s first `Choice`, as
   `images` takes them; `from`, `to`, `editAs`; a `graphicFrame`'s
   `graphicData` and its `c:chart` `r:id`). Under
   `any(feature = "images", feature = "charts")` go only the items
   both features call, each where its current gate lacks `charts`:
   `Anchor`, `EditAs`, `drawing_of`, `element_rel_id`,
   `DRAWING_ELEMENT`, `TWO_DRAWINGS`, `DRAWING_UNREADABLE`,
   `SPREADSHEET_DRAWING`, `MARKUP_COMPATIBILITY`, `digits_value` and
   `DRAWING`. No line `images` compiles changes, so the `images`
   cost's expected movement at P3 is 0. The spec's "Crate" sentence
   naming `read_drawing_roots` is read as this ruling.
B. Decision 2 A, the chartsheet refusal: the crate has no "not a
   worksheet" message; a chartsheet part handed to `charts` meets
   `element_rel_id`'s existing "the worksheet part's root is not
   worksheet", as `images` does today. Ruled: that message, named
   in P1's report and covered by one test; the sample calls
   `charts` on worksheets only.

C. Negative `axId` values (P1 finding, before the logged runs): the
   implementing session's unlogged dev sample refused one corpus
   workbook with "a chart cannot be read"; planning recounted with
   ElementTree (counts only): of the corpus's 136 `axId` and
   `crossAx` values, 12 are negative, all in 2 chart parts of 1
   workbook, each a sign and 10 digits inside the i32 range, and
   every plot's `axId` joins an axis of its chart (48 of 48). The
   schema types `axId` as `unsignedInt`; Excel writes the signed
   32-bit value it holds. An axis id is compared by equality and
   never computed with. Ruled, option (b): `Plot::axis_ids` is
   `Vec<i64>`, and in P2 `ChartAxis::id` and `ChartAxis::crosses`
   are `i64`, read from an optionally signed decimal integer the
   `i64` holds (anything else refused as "a chart cannot be read");
   the value is kept as written so parts 4 and 5 write back the
   form they read. Not (a): a bit pattern in a `u32` prints 10
   digits for a value the file wrote with a sign, and the writers
   would have to choose a form. Not (c): the 3 charts are real.
   The "Harness runs" figures stand (26 charts); the facts bullet
   on axes gains "12 of the 136 `axId` and `crossAx` values are
   negative, in 1 workbook". Tests: a negative `axId` joined to a
   negative `crossAx`, a `val` of `+1` refused or accepted as the
   implementing session's integer form decides (named in the
   report), a `val` of `1.0` refused.

D. The series figure (P1 finding, before the logged run log): the
   spec's "39 series in all" and "`series` 39" were planning's
   arithmetic slip over its own histogram (1 in 19 charts, 2 in 4,
   3 in 2, 5 in 1 is 38), and the name forms 26, 9 and 3 sum to 38;
   planning recounted `ser` over the 26 chart parts: 38. Ruled: the
   run log expects `series` 38; the spec's two figures and the
   facts bullet read 38. The same recount finds no `xVal` in the
   corpus (the 4 scatter series carry `cat` and `yVal`), so the
   facts bullet on series data is corrected; no "Harness runs"
   figure depends on it. Ruling C's integer form: Rust's
   `i64::from_str`, a leading `+` accepted, named in the report;
   accepted.

## Planning verification of P1 (2026-10-13)

Verified from primaries: the five logs at the record commit
cf8f145 (gate 0921ea16, test 15f70960, negative b2c054fa, run
25c1c24a, scan bd2cf684 PASS, 0 hits, 0 structural failures; each
of the four "all as expected: yes"); the diff 017e480..54bf510 (11
files, 3210 insertions, 27 deletions; the `cfg` widenings read
line by line); the record commit's four record files and
independent-check-1.md (8811f7d2); every commit message ASCII
with the Co-Authored-By line only; the working tree at the record
commit holding only this spec; the privacy scan of
017e480..cf8f145 (4 commits, 1671 blobs, 0 findings, PASS);
planning's own build at 54bf510 in a detached worktree with its
own target (`<home>/.cache/emend-planning/m5p3-p1-build.txt`): the
five feature sets on both targets and the workspace at the log's
figures (515, 542, 578, 645, 1017; 1225), clippy and doc clean,
release emend-copy on both targets, `--charts-sample` over the
corpus byte-identical to the run directory's outputs and lines on
both targets, and planning's host totals equal to the record's.
The figures stand: charts 26 in 11, plots 28, combined 2, series
38, the kind, name, title and anchor figures as "Harness runs";
the largest chart part 28215 bytes; the four part 2 target roots
pruned (du -sb then rm -r, no -f).

The disclosures. Attempt 1's scan pre-check refused two undeclared
placeholders (`<api>` carried in test.log's change notes,
`<sheets>` in chartscmp.py's docstring); the fixes changed a root
line and a docstring, the reruns carry RERUN_NOTE/RERUN_FILES, and
the attempt is kept under the phase directory: the rerun form was
followed. The report's 19743143592 sum and the two new script
hashes are stated as the implementer's own, not a log's: accepted.
The superseded commits (0edc2df, cbc5664, ca48df1, fbb11a8,
37cf313) were never logged; 54bf510's tree equals fbb11a8's, and
266c410's message carries the three sha256 lines.

Rulings:

E. Ruling C's test of a negative `axId` joined to a negative
   `crossAx` needs `ChartAxis`, so it is carried to P2 (its test
   list gains it; "the carry" in "P2").
F. Three refusals P1 documents and tests beyond "Errors": an
   `srgbClr` `val` that is not six hex digits, a boolean `val`
   outside `0`, `1`, `true` and `false`, and a non-numeric `idx`
   or `order`. Each is a malformed value the schema forbids, each
   gives the part's one message, and "Nothing is checked twice"
   holds. Accepted; "Errors" names them. The anchor refusals (no
   `col` or `row`, `editAs` outside its set) are `images`'s own,
   under "the drawing part is read as `images` reads it".
G. "Each data form with and without a cache": `multiLvlStrRef` is
   tested with a cache only. Carried to P2 as one test (the form
   without a cache, `cached` `None`).
H. The sample counts `Bar3D`, `Line3D` and `Pie3D` into `kind_bar`,
   `kind_line` and `kind_pie`, and asserts the six kind counts sum
   to `plots`. Accepted: the corpus holds no 3D plot, so no
   "Harness runs" figure moves; "Harness runs" says so.
I. The `DRAWINGML` widening to `any(images, read, charts)`, one
   more than ruling A's list: chart.rs needs the DrawingML main
   namespace; an `images` build already compiles it, so the
   `images` cost's expected movement stays 0. Accepted; ruling A's
   list is read with `DRAWINGML` added.
J. ROADMAP.md's part 3 paragraph said "39 series" at 266c410
   (planning's text, ruling D's slip); it reads 38 from P2's doc
   commit. Ruling A's text cited the doc commit as 37cf313; it now
   names the re-made 266c410 as well.
K. P1's run used the release binaries, as part 2's P2 and P3 did.
   P2 runs the release binaries too, and "P3" reads "byte-identical
   to P2's run" (not "debug run").
L. The integer form: `idx`, `order` and `ptCount` share `axId`'s
   `from_str`, so a leading `+` is accepted there too. Accepted as
   ruling D's form; no spec text changes.

P1 is DONE. P2's doc commit carries this spec (rulings A to L and
this section) and ROADMAP.md (P1 DONE, the series figure).

## Planning rulings during P2 (2026-10-13)

Before the logged runs, on the implementing session's dev report
(commits d148ad1, f659089, b30ebbd; dev figures axes 44, legends
15, pivot charts 5, styles 16 in 7, every P1 figure unchanged, the
P1 fields of each line equal to P1's run lines 27 of 27).

M. The catalogue test gains `package/chart.rs` as its tenth source,
   and `text_of`'s error match is rewritten so the extractor sees
   no `Error::Package(_)` arm (`Xml` and `Io` pass through,
   anything else is the refusal); `text_of` raises only those and
   its two `Package` messages, so the behaviour is the same. This
   P1 code change in the P2 crate commit is accepted, named in the
   message and the report.
N. Three forms the schema requires and the implementing session
   proposed to accept: a `crosses` with no `val`, a `numFmt` with
   no `formatCode`, a `pivotSource` with no `name`. Ruled: each is
   refused with "a chart cannot be read", as "a reference form with
   no `f`" and "a `ser` with no `idx`" are: a required value absent
   is refused, a value outside its set is refused, and nothing else
   is checked. "Errors" names the three; one test each (the
   `number_format` doc comment reads "`None` for no `numFmt`").
O. A `legendPos` with no `val` is `Right`, the schema default, as
   "P2" says. Accepted.
P. `min` and `max` are parsed by Rust's `f64::from_str`, which
   takes the `xsd:double` spellings `INF`, `-INF` and `NaN` and
   some more; the value is held, never computed with. Accepted as
   ruling L's form; named in the report.
Q. `digest_titles` stays the chart titles; axis titles are counted
   (`axis_titles`) and not digested, as "Harness runs" lists.
   Accepted.
R. `style` is `c:style` at the root, or `c:style` or `c14:style`
   inside the first `Choice` of a root `mc:AlternateContent`,
   whatever the `Choice`'s `Requires`; the `Fallback` is never
   entered. Accepted (the corpus's 16 styles all sit in a `Choice`
   holding `c14:style`).
S. P1's fixtures named axis ids with no axes; P2's join check
   refuses that, so the fixtures gain a `catAx` and a `valAx`, the
   signed-id test three axes, and the tool test's first chart its
   two axes, with the P1 assertions unchanged. Accepted. The keep
   against P1's open record: 0 new pairs, exactly 1 changed (the
   charts-sample tool test's, its input bytes changed), 0 gone;
   the open check covers the changed pair. The negative fixture is
   the tool test's form with the `valAx` lacking `axPos`, its
   control the same with `axPos` `l`.

## Planning verification of P2 (2026-10-13)

Verified from primaries: the five logs at the record commit
1e0b03c (gate 0ac53788, test ba1f1d56, negative 8335ee02, run
124a4939, scan a9ae035e PASS, 0 hits, 0 structural failures; each
of the four "all as expected: yes"); the diff d148ad1..27569d2
and the ruling N diff b30ebbd..27569d2 (3 files, 53 insertions,
11 deletions; the tool's content unchanged); d148ad1 holding
planning's two files unedited by hash; the record files and
independent-check-1.md (467967c4); every commit message ASCII with
the Co-Authored-By line only; the privacy scan of cf8f145..1e0b03c
(4 commits, 1681 blobs, 0 findings, PASS); planning's own build at
27569d2 in its detached worktree
(`<home>/.cache/emend-planning/m5p3-p2-build.txt`): the five
feature sets and the workspace on both targets at the log's
figures (515, 553, 589, 656, 1028; 1237), clippy and doc clean,
release emend-copy on both targets (the i686 binary's sha256
equal to run.log's), `--charts-sample` over the corpus
byte-identical to the run directory's outputs and lines on both
targets, 27 of 27, and planning's host totals at the figures:
axes 44 (cat 20, val 24, other 0), legends 15, pivot charts 5,
styles 16, axis number formats 43, bounds 6, deleted 5, gridlines
17, axis titles 2, every P1 figure unchanged. The keep: 0 new, 1
changed (the charts-sample tool test's), 0 gone, as ruling S.

The disclosures. The final scan's attempt 1 found one `f_home` hit
in the check's answer (the phase directory written absolute); the
path is now `<phase>`, the attempt is kept under the phase
directory and named by RERUN_NOTE/RERUN_FILES: the rerun form was
followed. The superseded f659089 and b30ebbd were never logged.
The run's dry run 1 expected 21 figures by the implementer's
miscount (22 is right); unlogged, disclosed. The report's
statements with no primary (the variable name, the dry-run
hashes, the test breakdown 2 + 6 + 3) are stated as the
implementer's own: accepted.

Rulings:

T. The check's note (a): ruling R's `c:style` inside the `Choice`
   has no test (only `c14:style` there). Carried to P3 as one test.
   Note (b): the `Fallback` is shown unread both when the `Choice`
   holds a style and when it holds none; nothing more is needed.
U. Note (c): the package header's Errors list does not name the
   three ruling N tests. Carried to P3 (a header edit, no code).
V. Note (d): a repeated `legend` or `pivotSource` keeps the last
   read. The schema allows one; nothing is checked twice.
   Accepted, no change.
W. Note (e), ruling M's "same behaviour": planning read `text_of`
   in `cells.rs`; it raises one `Package` literal, `Error::Xml`,
   and what the stream and `expand` raise (`Xml`, `Io`, `Package`),
   so the two mappings agree on every error it can return.
   Confirmed.
X. Ruling P's spellings are shown by no test. Carried to P3 as one
   test (`INF`, `-INF`, `NaN` and an exponent form read as `min`
   or `max`).
Y. The check brief: the checker writes every path under the
   placeholders the scan declares (`<phase>` among them); P3's
   brief says so before the check runs.

P2 is DONE. P3's doc commit carries this spec (rulings A to Y and
the two verification sections) and ROADMAP.md (P2 DONE).

## Decisions for the maintainer

Decided by the maintainer 2026-10-13: 1, 2 and 3 as recommended (A
in each), typed in the planning session.

1. The model's depth: A (recommended), the plots with their series
   data and formatting flags, the axes with titles, formats and
   bounds, the legend, the pivot source and the style, as "The
   call" says, so parts 4 and 5 write what this part reads; B, the
   kind, the title, the series references and the anchor alone
   (`ChartAxis`, `LegendPosition`, `fill`, `smooth`, `marker`,
   `labels`, `pivot_source`, `style` dropped; P2 shrinks to the
   catalogue), a smaller reader the write parts grow.
2. Chartsheets (`chartsheet+xml`, corpus 0): A (recommended), out
   of this part; `charts` takes a worksheet part and a chartsheet
   part is refused with "no such part"'s sibling "not a worksheet"
   if the crate has one, else reads as empty (the implementing
   session names which in P1's report); B, a chartsheet's drawing
   read too, proven on a built package as part 2 proved external
   links.
3. The series' cached values: A (recommended), the cache's point
   count alone (`cached`), no value read, so no cell value type
   enters the chart model; B, the cached points read as numbers
   and strings (`Vec<Value>`-like), for a consumer that wants the
   chart's data without reading the sheet.

## Out of scope

- Writing, editing or removing a chart (parts 4 and 5); a chart's
  formatting beyond the solid fill flag and the four booleans
  (fonts, line styles, gradients, data labels' content, 3D view,
  walls and floors); the chart style, colour style and user shapes
  parts; `chartex` charts (corpus 0); charts in chartsheets
  (decision 2); the cached values (decision 3); `extLst` content
  (data label ranges, filtered series); trendlines and error bars
  beyond being walked past; a drawing's shapes, connectors, groups
  and diagrams.
