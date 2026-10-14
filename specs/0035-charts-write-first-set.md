# specs/0035: charts write, the first set: `add_chart`

Status: APPROVED 2026-10-13 (Milestone 5 part 4; decisions 1 to 3
decided A by the maintainer 2026-10-13, typed in the planning
session).

Milestone 5, "charts and read breadth", was approved by the
maintainer 2026-10-11 (`ROADMAP.md`, "Milestone 5"): charts a Cargo
feature `charts` of this crate, off by default, measured in the size
record like every other feature; the Milestone 2 ceiling kept for
the core; the Milestone 2 Excel check gates no push in this
milestone. This part is the fourth of six and the second of the
three chart parts: a chart written into a worksheet's drawing from
cell ranges. Part 3 (`specs/0034`) chose its model's names for the
writers; this part writes what part 3 reads, and part 3's reader is
the writer's second acceptance instrument beside the harness: every
chart this part writes is read back by `Package::charts` and
compared field by field.

## Goal

A consumer adds a chart to a worksheet of a workbook it did not
write: a bar, column, line, pie, scatter or area chart drawing one
or more series from cell ranges the consumer names, with a title,
axis titles and a legend, anchored on the sheet as an image is
(one-cell, two-cell or absolute). The output keeps every other
entry byte for byte, LibreOffice opens every corpus output, and
`Package::charts` reads the written chart back as the consumer
specified it. Nothing else about a chart is written in this part
(part 5: doughnut, radar, bubble, stock and 3D kinds, combined
charts, data labels, series colours and number formats,
`remove_chart`).

## Size and packaging

The budget figure after part 3 is 436992 section bytes against the
ceiling of 458752 (`records/size-m5p3-2026-10-13-d74ea74.md`):
21760 of headroom, held for fixes; the core's growth in part 3 was
0 and the `charts` feature cost 44428 (the reader). The writer goes
under the same feature `charts` (decision 1), measured by the same
probe, `probe-emend-charts-write-xml`, which gains the write: the
format probe with the feature, then `add_chart` of one column chart
with one series on its first sheet from the probe's own constants,
then `charts` of that sheet, the chart count and the series count
printed (1 and 1; part 3's P3 ruling AA printed 0 and 0 with
nothing to write). The probe's figure then moves by the writer's
cost, and the size record states the feature's whole cost, part
3's figure and the delta.

Planning's estimate, priced as part 3's ruling AC asks, the
enumeration sets and the parsers separately: the enumeration sets
written (five plot elements, two bar directions, four groupings,
six scatter styles, five legend positions, four axis positions,
three anchor forms: the strings and their match arms) 4000 to
6000; the chart part's emission (about sixty fixed element forms
of `chartSpace`, `chart`, `plotArea`, the plot, `ser`, `catAx`,
`valAx`, `title`, `legend`, with the escaping of six caller
strings) 10000 to 16000; the drawing additions (the `graphicFrame`
form in each anchor, the drawing's creation or append shared with
`images`, the `chart` relationship, the part allocation and the
content type `Override`, each shared with the image writer and
regated) 4000 to 7000; the call's checks (the texts, the
references, the kind and grouping applicability, the size and
anchor range) 3000 to 5000; number parsers 0 (the writer parses
nothing; it formats `u32` and `u64`, which the crate does today).
In all 21000 to 34000 for the writer, so a `charts` feature cost of
65000 to 79000. Planning is told the figure in the report at any
value; a writer cost (the probe's movement over part 3's 44428)
above 40000, or core growth above 2000, is a finding the report
names. The core's growth is what the feature gates add, expected
0. The `images` feature's own cost may move by the regating and
the split of the anchor writer it shares (expected 0, accepted up
to 1500): the report prints and judges the movement.

## Facts planning checked (2026-10-13)

- The corpus's 26 chart parts, counted by planning with Python's
  `zipfile` and `ElementTree` (counts only, the script under
  planning's job directory, never committed; part 3's facts stand):
  `chartSpace` children in order `date1904`, `lang`,
  `roundedCorners`, (`AlternateContent`), (`pivotSource`), `chart`,
  `spPr`, `txPr`, `printSettings`, (`extLst`), (`userShapes`) in
  16, and `lang`, `roundedCorners`, `chart`, `spPr` in 10;
  `roundedCorners` `0` in all 26; `chart` children `title` (11),
  `autoTitleDeleted` (26), (`pivotFmts`), `plotArea` (26),
  `legend` (15), `plotVisOnly` (26), `dispBlanksAs` (25),
  (`showDLblsOverMax`); `plotVisOnly` `1` 16 and `0` 10;
  `dispBlanksAs` `gap` 22, `span` 2, `zero` 1.
- `plotArea` children: (`layout`), the plots, the axes, `spPr`;
  a bar or line plot with `catAx` then `valAx` (16) or `valAx`
  then `catAx` (3); a scatter plot with two `valAx` (2); a pie or
  doughnut plot with no axis (4).
- `barChart` children: `barDir`, `grouping`, `varyColors`, the
  `ser`s, (`dLbls`), `gapWidth`, (`overlap`), `axId`, `axId`;
  `lineChart`: `grouping`, `varyColors`, the `ser`s, (`dLbls`),
  (`hiLowLines`), `marker`, (`smooth`), `axId`, `axId`;
  `scatterChart`: `scatterStyle`, `varyColors`, the `ser`s,
  (`dLbls`), `axId`, `axId`; `pieChart`: `varyColors`, the `ser`,
  (`dLbls`), `firstSliceAng`.
- `ser` children in order: `idx`, `order`, (`tx`), (`spPr`),
  (`invertIfNegative`), (`marker`), (`dPt`s), (`dLbls`), (`cat`),
  `val` or `yVal`, (`smooth`), (`extLst`); the reference forms
  each carry `f` and a cache (part 3: 65 in 10, none without).
- `catAx` children in order: `axId`, `scaling`, `delete`, `axPos`,
  (`majorGridlines`), (`title`), (`numFmt`), `majorTickMark`,
  `minorTickMark`, `tickLblPos`, (`spPr`), (`txPr`), `crossAx`,
  `crosses`, `auto`, `lblAlgn`, `lblOffset`, `noMultiLvlLbl`,
  (`extLst`); `valAx`: `axId`, `scaling`, `delete`, `axPos`,
  (`majorGridlines`), (`title`), `numFmt`, `majorTickMark`,
  `minorTickMark`, `tickLblPos`, (`spPr`), (`txPr`), `crossAx`,
  `crosses`, `crossBetween`, (`majorUnit`), (`dispUnits`),
  (`extLst`); `crosses` `autoZero` in all 44; `crossBetween`
  `between` 18 and `midCat` 6; `orientation` `minMax` 43, `maxMin`
  1; `title` children `tx`, (`layout`), `overlay`, (`spPr`),
  (`txPr`); `legend` children `legendPos`, (`layout`), `overlay`,
  `spPr`, `txPr`, `overlay` `0` in all 28.
- The drawings' 30 `graphicFrame`s (26 charts, 3 slicers, 1
  timeline): attribute `macro` on 20 (empty) and none on 10;
  children `nvGraphicFramePr` (`cNvPr`, `cNvGraphicFramePr`),
  `xfrm`, `graphic` in all 30; `xfrm` with `off` 0,0 and `ext` 0,0
  in 20 and non-zero in 10; the `c:chart` child of `graphicData`
  carries `r:id` in all 26.
- The crate: `add_image` and `add_image_at` (feature `images`)
  create or append to a worksheet's drawing part, allocate the
  media and drawing entries and their `Override`s, create the
  `drawing` element in schema position and the `DRAWING` and
  `IMAGE` relationships, and write the three anchor forms with
  `cNvPr` ids one above the largest in the part (`specs/0022`
  "Images: the parts", `specs/0028` "Anchors"); `ImageAnchor` and
  `EditAs` are the anchor model; the constant `CHART` (the
  relationship kind, pair form) exists under `read` and `charts`;
  `Package::charts` reads every anchor form's `graphicFrame`
  (`specs/0034`). No chart part is written by anything.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. The feature
`charts` gains the writer (decision 1): one new call on `Package`,
its types and its messages, all gated. The anchor model
(`ImageAnchor`, `EditAs`, the EMU conversion and its range check)
and the drawing writer (the part's creation or append, the
`drawing` element, the drawing's rels part, the `cNvPr` id) gain
the gate `any(feature = "images", feature = "charts")`; the anchor
writer takes the anchor's body (the `pic` or the `graphicFrame`)
from its caller so that both features share one form. Nothing
existing changes signature or output: the image writer's bytes are
byte-identical to part 3's (the image samples' run outputs prove
it, "Harness runs"). The module `package/chart_write.rs`, beside
`chart.rs`.

### The call

`Package::add_chart(&mut self, part: &[u8], anchor: ImageAnchor,
chart: &NewChart<'_>)` returning `Result<(), Error>`, for a
worksheet part: records one chart on the sheet, written at the
write as an image is (a snapshot of the pending edits in call
order; an image and a chart on one sheet in one write take the
drawing in call order).

```
pub struct NewChart<'a> {
    pub plot: NewPlot,
    pub title: Option<&'a str>,          // None: `autoTitleDeleted` 1
    pub series: &'a [NewSeries<'a>],     // one or more
    pub category_axis_title: Option<&'a str>, // the category or x axis
    pub value_axis_title: Option<&'a str>,    // the value or y axis
    pub legend: Option<LegendPosition>,  // None: no legend
    pub size: Option<(u32, u32)>,        // pixels; see "Anchors"
}
pub enum NewPlot {
    Bar { direction: BarDirection, grouping: Grouping },
    Line { grouping: Grouping },
    Pie,
    Scatter { style: ScatterStyle },
    Area { grouping: Grouping },
}
pub struct NewSeries<'a> {
    pub name: Option<NewSeriesName<'a>>,
    pub categories: Option<&'a str>,     // `cat`, or a scatter's `xVal`
    pub values: &'a str,                 // `val`, or a scatter's `yVal`
}
pub enum NewSeriesName<'a> { Reference(&'a str), Text(&'a str) }
```

`NewChart`, `NewSeries` and `NewSeriesName` derive `Clone`, `Copy`,
`Debug`, `PartialEq`, `Eq`; `NewPlot` likewise. `BarDirection`,
`Grouping`, `ScatterStyle` and `LegendPosition` are part 3's
enumerations, shared. A chart is read back by `charts` as one
`ChartEntry` whose `chart.plots` holds one `Plot` of the kind the
variant names (`Bar`, `Line`, `Pie`, `Scatter`, `Area`), with the
direction, grouping and style given, `vary_colors` true for a pie
and false otherwise, the series in order with `index` and `order`
`0`, `1`, ..., the names and references as given (`cached` `None`,
decision 3), `fill` `None`, the flags false; `title` as given,
`auto_title_deleted` true when `title` is `None`; two axes for a
bar, line, area or scatter plot (ids 10 and 20, "The chart part")
and none for a pie; `legend` as given; `pivot_source` and `style`
`None`.

### Texts and references

A title, an axis title and a `Text` name are written as the text
of one run in one paragraph of a `rich` body, XML-escaped. A
`Reference` name, a `categories` and a `values` string are written
as the `f` text of a `strRef` or `numRef` verbatim (decision 2):
the caller writes the formula as the file holds it, without a
leading `=`, with the sheet name quoted as a formula quotes it
(`'My Sheet'!$A$2:$A$9`); the crate neither parses nor quotes it,
and `charts` reads it back unchanged. Every string is refused when
empty or when it holds a character below U+0020 ("a chart text is
empty or holds a control character"); a title or name longer than
32767 bytes, or a reference longer than 8192 bytes, is refused
with the same message (the limits of the kept-record wording, so
that nothing unbounded enters the part). A `Text` name writes
`tx/v`; a `Reference` name writes `tx/strRef/f`. `categories`
writes `cat/strRef/f` for a bar, line, pie or area series and
`xVal/numRef/f` for a scatter series; `values` writes `val/numRef/f`
and, for a scatter series, `yVal/numRef/f`. No cache is written
(decision 3): `charts` reads `cached` `None` for each.

### Applicability

A `Bar` plot's grouping is `Clustered`, `Stacked` or
`PercentStacked` (`Standard` is the 3D form's and is refused); a
`Line` or `Area` plot's grouping is `Standard`, `Stacked` or
`PercentStacked` (`Clustered` refused); a `Pie` plot takes exactly
one series (two or more refused) and no axis title (either `Some`
refused); a `Scatter` plot's `style` is any of the six. A chart
with no series is refused. The message for each: "a chart option
does not apply to its kind".

### Anchors

`anchor` is `ImageAnchor`, written as `add_image_at` writes it
(`specs/0028` "Anchors"), the `pic` replaced by the `graphicFrame`
below. `size` is the chart's pixel width and height: required for
a one-cell or absolute anchor (it fills `ext`; `None` refused) and
refused for a two-cell anchor (`Some` refused; the cells set the
box), with one message "a chart size does not apply to its
anchor". A size whose EMU form exceeds `i32::MAX`, a width or
height of 0, an offset or position out of range, a `to` cell
before `from`, or a cell off the grid, is refused as the image
writer refuses them, under one chart message "a chart anchor is
out of range" (the grid message of part 3 of Milestone 1 for the
cell, as `add_image_at` raises it).

The `graphicFrame`, inside the anchor after `from`, `to`, `pos` or
`ext` as the form has them and before `clientData` (one line each,
broken to fit; `I` one above the largest `cNvPr` `id` in the part,
`N` the relationship's number):

```
<xdr:graphicFrame macro="">
<xdr:nvGraphicFramePr><xdr:cNvPr id="I" name="Chart I"/>
<xdr:cNvGraphicFramePr/></xdr:nvGraphicFramePr>
<xdr:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/></xdr:xfrm>
<a:graphic><a:graphicData
 uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
<c:chart xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
 xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
 r:id="rIdN"/></a:graphicData></a:graphic>
</xdr:graphicFrame>
```

The prefixes follow `specs/0022`'s rule: the root's prefix for the
spreadsheet-drawing namespace, `a` and `r` bound on the added
elements when the root does not bind them, `c` always bound on the
`c:chart` element (no corpus drawing root binds the chart
namespace). The `xfrm` is the zero form the corpus's 20 of 30
frames carry.

### The parts

The chart part `xl/charts/chart<N>.xml` takes the smallest
positive N such that no entry of the archive and no addition of
this write has the name, case folded, removed entries counted (the
media rule); its content type is an `Override`
`application/vnd.openxmlformats-officedocument.drawingml.chart+xml`
through the override rule. The drawing's rels part gets a
relationship of kind `CHART` (pair form, the sheet's relationship
form) with the target `../charts/chart<N>.xml`, created when the
drawing is created or when a kept drawing has no rels part. The
chart part has no rels part of its own. A sheet with no drawing
part gets one as `add_image` creates it (the drawing entry, its
`Override`, the `DRAWING` relationship, the `drawing` element in
schema position); a kept drawing is appended to as `specs/0022`
says.

### The chart part

Created whole (the declaration on its own line, then one line,
broken here to fit). The `c` prefix and the chart namespace, `a`
bound on the root; `xmlns:r` bound only when a `Reference` name is
written (it is not: `strRef` carries no `r:` attribute, so the
root binds `c` and `a` only).

```
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
 xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
<c:roundedCorners val="0"/>
<c:chart>
[<c:title><c:tx><c:rich><a:bodyPr/><a:lstStyle/><a:p><a:r>
<a:t>TITLE</a:t></a:r></a:p></c:rich></c:tx><c:overlay val="0"/>
</c:title>]
<c:autoTitleDeleted val="A"/>
<c:plotArea><c:layout/>
PLOT
AXES
</c:plotArea>
[<c:legend><c:legendPos val="P"/><c:overlay val="0"/></c:legend>]
<c:plotVisOnly val="1"/><c:dispBlanksAs val="gap"/>
</c:chart>
</c:chartSpace>
```

`A` is `0` with a title and `1` without. `P` is `b`, `l`, `r`,
`t` or `tr` from `LegendPosition`. PLOT is one of:

```
<c:barChart><c:barDir val="D"/><c:grouping val="G"/>
<c:varyColors val="0"/>SERIES<c:gapWidth val="150"/>
[<c:overlap val="100"/>]<c:axId val="10"/><c:axId val="20"/>
</c:barChart>
<c:lineChart><c:grouping val="G"/><c:varyColors val="0"/>
SERIES<c:marker val="1"/><c:axId val="10"/><c:axId val="20"/>
</c:lineChart>
<c:pieChart><c:varyColors val="1"/>SERIES
<c:firstSliceAng val="0"/></c:pieChart>
<c:scatterChart><c:scatterStyle val="S"/><c:varyColors val="0"/>
SERIES<c:axId val="10"/><c:axId val="20"/></c:scatterChart>
<c:areaChart><c:grouping val="G"/><c:varyColors val="0"/>
SERIES<c:axId val="10"/><c:axId val="20"/></c:areaChart>
```

`D` is `bar` or `col`; `G` `standard`, `clustered`, `stacked` or
`percentStacked`; `S` the six `scatterStyle` values; `overlap` is
written for a stacked or percent-stacked bar (Excel draws stacked
bars side by side without it). SERIES is, for each series in
order, `K` its zero-based position and `REF` an `f` form:

```
<c:ser><c:idx val="K"/><c:order val="K"/>
[<c:tx><c:strRef><c:f>REF</c:f></c:strRef></c:tx>
 | <c:tx><c:v>TEXT</c:v></c:tx>]
[<c:cat><c:strRef><c:f>REF</c:f></c:strRef></c:cat>
 | <c:xVal><c:numRef><c:f>REF</c:f></c:numRef></c:xVal>]
<c:val><c:numRef><c:f>REF</c:f></c:numRef></c:val>
 | <c:yVal><c:numRef><c:f>REF</c:f></c:numRef></c:yVal>
</c:ser>
```

AXES is empty for a pie, and otherwise a category axis then a
value axis; for a scatter plot both are `valAx` (the x axis the
first). The category axis of a `Bar` with direction `Bar` sits at
`l` and its value axis at `b`; every other plot's category axis
sits at `b` and value axis at `l`:

```
<c:catAx><c:axId val="10"/><c:scaling><c:orientation val="minMax"/>
</c:scaling><c:delete val="0"/><c:axPos val="X"/>[TITLE]
<c:numFmt formatCode="General" sourceLinked="1"/>
<c:majorTickMark val="out"/><c:minorTickMark val="none"/>
<c:tickLblPos val="nextTo"/><c:crossAx val="20"/>
<c:crosses val="autoZero"/><c:auto val="1"/><c:lblAlgn val="ctr"/>
<c:lblOffset val="100"/><c:noMultiLvlLbl val="0"/></c:catAx>
<c:valAx><c:axId val="20"/><c:scaling><c:orientation val="minMax"/>
</c:scaling><c:delete val="0"/><c:axPos val="Y"/><c:majorGridlines/>
[TITLE]<c:numFmt formatCode="General" sourceLinked="1"/>
<c:majorTickMark val="out"/><c:minorTickMark val="none"/>
<c:tickLblPos val="nextTo"/><c:crossAx val="10"/>
<c:crosses val="autoZero"/><c:crossBetween val="B"/></c:valAx>
```

A scatter's x axis is the `valAx` form with `axId` `10`, `crossAx`
`20`, no `majorGridlines` and `crossBetween` `midCat`; its y axis
the `valAx` form above with `crossBetween` `midCat`; `B` is
`between` otherwise. TITLE is the chart's `title` form with the
axis title's text. These are the corpus's common axis children
less the formatting (`spPr`, `txPr`), openpyxl's defaults, which
Excel and LibreOffice open; `charts` reads back `number_format`
`Some("General")`, `major_gridlines` true on the value axis,
`crosses` the other axis's id, `min` and `max` `None`, `deleted`
false, the titles as given.

### Errors

Four new `Error::Package` messages under `charts`, in the
catalogue marked by feature: "a chart text is empty or holds a
control character", "a chart option does not apply to its kind",
"a chart size does not apply to its anchor" and "a chart anchor is
out of range", each raised at the call. At the write: "a drawing
part or its relationship cannot be read" and "a worksheet has two
drawing elements" as `images` raises them, now under either
feature; a chart on a sheet added in the same write, a replaced or
a removed part: part 1's messages, as an image. Nothing is checked
twice.

### 32-bit and streaming

The chart part is built in memory, bounded by the caller's strings
(each limited, "Texts and references"); the drawing is read and
appended as `images` does it. Every count is `u32` or `u64`.

### Harness runs

`tools/emend-copy` gains two operations, given after the two paths
as the others are; neither takes a reference, name or number from
its command line; every constant below is the tool's own. W is
the first worksheet part in `<sheets>` order and M its largest row
number as the tool's cell read gives it (0 for an empty sheet); the
tool builds its references from W's name, quoted by doubling each
apostrophe and wrapping in apostrophes always, and prints nothing
of them. No reference, no name, no path and no text from the
corpus is printed: the samples print counts and flags.

- `--add-chart-sample <in> <out>` (P1): `add_chart(W, TwoCell {
  from: (M + 2, 1), to: (M + 18, 9), offsets zero, edit_as:
  OneCell }, NewChart { plot: Bar { direction: Column, grouping:
  Clustered }, title: Some("emend chart"), series: [NewSeries {
  name: Some(Text("emend series")), categories: Some(A1:A(M+1) of
  W), values: B1:B(M+1) of W }, NewSeries { name: Some(Reference(
  C1 of W)), categories: None, values: C1:C(M+1) of W }],
  category_axis_title: Some("emend categories"),
  value_axis_title: Some("emend values"), legend: Some(Bottom),
  size: None })`; writes; reopens the output and prints `addchart`
  with `charts_before`, `charts_after` (`+ 1`, asserted),
  `read_back_equal` (1 when the last entry of `charts(W)` reads as
  "The call" says for this chart, every field compared, asserted),
  `anchors_before`, `anchors_after` (`+ 1`, asserted),
  `chart_parts_before`, `chart_parts_after` (`xl/charts/` entries,
  `+ 1`, asserted), `drawing_created` (1 when the input's W had no
  drawing, else 0), `two_after` (entries of `charts(W)` with a
  two-cell anchor, `+ 1`, asserted). Expected results: the chart
  part `added`; W's drawing part and its rels part `added` (when
  `drawing_created` is 1, with W's part `changed` and its rels
  part `added` or `changed`) or `changed`; `[Content_Types].xml`
  `changed`; every other entry `identical`; `lo` and `lo_in` `ok`
  for every corpus workbook.
- `--chart-kinds-sample <in> <out>` (P2): six `add_chart` calls on
  W, each `OneCell { from: (M + 2 + 16 k, 1), offset: (0, 0) }` for
  k 0 to 5 with `size: Some((480, 288))`, the plots in order `Bar
  { Bar, Clustered }`, `Bar { Column, Stacked }`, `Line { Standard
  }`, `Pie`, `Scatter { LineMarker }`, `Area { Standard }`, each
  with `title: Some("emend kind k")`, one series `NewSeries {
  name: Some(Reference(B1 of W)), categories: Some(A2:A(M+1) of W),
  values: B2:B(M+1) of W }` (M at least 1 for the sample; M 0
  takes row 1), `legend: Some(Right)`, no axis title for the pie
  and `Some("emend x")`, `Some("emend y")` for the others; writes;
  reopens and prints `chartkinds` with `charts_before`,
  `charts_after` (`+ 6`, asserted), `kinds_read_back` (the count
  of the six last entries of `charts(W)` whose plot reads as
  specified, kind, direction, grouping, style, series, axes count,
  titles and legend compared; 6, asserted), `one_after` (`+ 6`,
  asserted), `chart_parts_after` (`+ 6`, asserted). Expected
  results: as the add-chart sample's with six parts `added`; `lo`
  and `lo_in` `ok` for every corpus workbook.
- The image samples (`--image-sample`, `--anchor-sample`,
  `--remove-image-sample`) rerun in P1 on both targets: their
  outputs and lines byte-identical to the latest record of each
  (`records/m3p1-p3-image-2026-10-02-6578925.txt`,
  `m4p1-p3-anchor-2026-10-10-669fa91.txt` and
  `m4p1-p3-removeimage-2026-10-10-669fa91.txt`, each compared by
  sha256 in the run log), proving the regated drawing writer
  unchanged; a difference is a finding.
- `--charts-sample` rerun in P3 on both targets with the release
  binaries: byte-identical to part 3's record
  (`records/m5p3-p3-charts-2026-10-13-d74ea74.txt`), proving the
  reader untouched.

The keep and the edit open check as every run log.

## Phases

As `specs/0001` and part 3's: a doc commit first, code commits, one
record commit, the scan last, at most two independent checks per
phase, a report, STOP. One target root per phase,
`<home>/.cache/emend-build/m5p4-<phase>/target`. P1's gate prunes
part 3's four roots (`<home>/.cache/emend-build/m5p3-p1/target`,
`m5p3-p2/target`, `m5p3-p3/target` and `m5p3-close/target`), each
listed with `du -sb` and removed with `rm -r`; the close-out's
`gen.py` and logs stay. Every rule of part 3's "Phases" paragraph
carries: the three logged scripts in the phase directory with
`scripts/counting.sh` copied by `git show`; a dry run before each
logged run; a refused or failed attempt hashed in the step's own
log through `RERUN_NOTE` and `RERUN_FILES` before a rerun;
`input: TMPDIR=<scratch>/tmp` and the `mktemp` check with
controls; `python3 -I -B` everywhere and the gate's `-B` check;
the gate's line on `scripts/counting.sh`'s own `mktemp`. Logs in
`records/logs/m5p4-<phase>/` (`gate.log`, `test.log` on both
targets with the feature sets off, `charts`, `charts,images`,
`charts,read` and every feature, `negative.log`, `run.log`,
`scan.log`, `size.log` in P3); the scan's denylist form whole-word.
No push during the part; the close-out push follows P3 and carries
the record commit 7a3fec6 of part 3's close-out with this part's.

Part 4's first doc commit carries: this spec; `ROADMAP.md` with
part 3's Close row DONE (record 7a3fec6) and this part's rows;
`specs/0034-charts-read.md` gaining "Planning rulings on the
close-out (2026-10-13)" (rulings AD and AE), the text copied
verbatim from planning's cache.

### P1: the writer, bar and line charts

The doc commit; the prunes; then the regating and the split of the
anchor writer, `NewChart`, `NewPlot` with its `Bar` and `Line`
variants (`Pie`, `Scatter` and `Area` are added in P2, so that
each phase's API listing names what that phase writes), `NewSeries`,
`NewSeriesName`, `add_chart` with the texts, references,
applicability, anchor and size rules, the chart part, the
`graphicFrame`, the parts and relationships, the four messages,
and `--add-chart-sample`. Tests on built packages, each written
chart read back by `charts` and compared: a column chart with two
series (a text name, a reference name, with and without
categories), a title, both axis titles and a legend on a sheet
with no drawing (the part, the `Override`, the `DRAWING` and
`CHART` relationships, the `drawing` element in position); the
same on a sheet with a kept drawing holding a picture (the picture
kept, the frame appended, the `cNvPr` id one above); a bar chart
with direction `Bar` (the axes swapped); each `Bar` grouping with
and without `overlap`; a line chart with each grouping; no title
(`autoTitleDeleted` `1`), no legend, no axis titles; each anchor
form with `size` as "Anchors" says; the refusals (each text rule,
each applicability rule reachable in P1, each size and anchor
rule, the drawing refusals as `images` tests them); an image and a
chart on one sheet in one write, in call order; the tool's line;
the image tests under `images` alone, `charts` alone and both. The
gate as part 3 P1's (the crate built and tested with the feature
sets on both targets, the `pub fn` lines listed off and on) plus
the prunes. The negative log through the binary: the add-chart
sample on a fixture whose worksheet names a `drawing` relationship
with no part (refused with the cannot-be-read message, with a
control). The run log: `--add-chart-sample` on both targets over
the corpus, the checks as "Harness runs", `lo` and `lo_in` `ok` for
every workbook, the three image samples byte-identical to their
records, the keep and the edit open check, recorded as
`records/m5p4-p1-<utc-date>-<shortsha>.md` and `.txt`.

### P2: pie, scatter and area charts; the catalogue

The doc commit (the P1 verification rulings); the carry; then the
`Pie`, `Scatter` and `Area` variants, their forms, their
applicability rules, `--chart-kinds-sample`; then the `package`
header's catalogue and limits for this part (the messages by
feature; the text and reference limits; no cache written; the
anchor writer shared). Tests on built packages, each read back and
compared: a pie with one series (no axes, `varyColors` `1`,
`vary_colors` read true), a pie with two series refused, a pie with
an axis title refused; a scatter with each style, `xVal` and
`yVal`, and one with no categories (no `xVal`); an area chart with
each grouping, `Clustered` refused; the tool's line. The gate as
P1's. The negative log through the binary: the kinds sample on a
fixture whose drawing root is not `wsDr` (refused, with a
control). The run log: both samples on both targets over the
corpus, the checks as "Harness runs", `lo` and `lo_in` `ok` for
every workbook, recorded as `records/m5p4-p2-<utc-date>-<shortsha>.md`
and `.txt`.

### P3: the probe, the size record, the API listing, the CHANGELOG

The doc commit (the P2 verification rulings); the carry; then the
probe's write as "Size and packaging" says; the size log, recorded
as `records/size-m5p4-<utc-date>-<shortsha>.md`, with the `charts`
cost, its movement over part 3's 44428 (the writer's cost), the
`images` cost's movement (expected 0, accepted to 1500) and the
core's growth (expected 0, accepted to 2000) each printed and
judged; the API listing rerun as part 3 P3 did, recorded as
`records/api-review-m5p4-<utc-date>-<shortsha>.md`; `CHANGELOG.md`'s
"Added" gains one bullet citing this spec, under `specs/0031`'s
citation check. The negative log through the binary for every
message of the part reachable there, the unreachable ones stated
with the test that covers each. The run log: both samples on both
targets over the corpus with the size log's release binaries,
byte-identical to P2's run by sha256, and `--charts-sample` once
more with the release binaries (byte-identical to part 3's record
by sha256), recorded as
`records/m5p4-p3-<sample>-<utc-date>-<shortsha>.md` and `.txt`. The
open record of the part's keep.

### Close-out push (after P3)

As part 3's (`specs/0034` "Close-out push (after P3)" and its
close-out rulings): the generator transforms part 3's scripts
(`gen.py` 18f278e0 and the three it printed, carried cmp-equal),
its numbered changes continuing from 39; the doc commit holds this
spec and `ROADMAP.md`; the base is part 3's close-out doc commit
93a14ef, named by its subject ("M5 part 3 close-out: commit
planning docs (the P3 verification and rulings)");
`PREV_RECORD_PATTERN` is `^M5 part 3 close-out record` (7a3fec6),
in the range exactly once; the close-out doc commit's subject is
"M5 part 4 close-out: commit planning docs (the P3 verification and
rulings)"; `CARGO_TARGET_DIR` printed and checked as
`<home>/.cache/emend-build/m5p4-close/target`; no Excel-record
check, the stray-record count 0 kept; the scan's denylist form
whole-word; the record commit "M5 part 4 close-out record: the
push, the CI run and the scan" holding the three logs only; pre
mode on a clean tree; one independent check before the report. The
push carries 7a3fec6 and this part's commits, on planning's word
and the maintainer's authorisation typed in the implementing
session.

## Decisions for the maintainer

Decided by the maintainer 2026-10-13: 1, 2 and 3 as recommended (A
in each), typed in the planning session.

1. Packaging ("Size and packaging"): A (recommended), the writer
   under the feature `charts` with the reader, as `images` holds
   its reader and writer, measured by the same probe, which gains
   the write; B, a second feature `charts-write` depending on
   `charts`, measured alone by a thirtieth probe, for a consumer
   that reads charts and never writes one.
2. References ("Texts and references"): A (recommended), a
   reference is a string the caller writes as the file's `f` text
   (openpyxl's model stores the same string; the crate neither
   parses nor quotes it, and part 3's reader returns it unchanged);
   B, a typed reference (a sheet index and a cell range) that the
   crate formats and quotes, a small parser-free formatter that
   part 5 could then reuse for its number formats.
3. Caches ("Texts and references"): A (recommended), no cache is
   written: Excel and LibreOffice compute the series from the
   cells on open (openpyxl's form; Excel's Protected View shows
   such a chart empty until editing is enabled, as it does for
   openpyxl's charts); B, the caches filled from the sheet's cells
   at the write (a cell read of every referenced range in the
   writer, with the `read` feature's value types; a larger writer
   and a second reader of the sheet).

## Out of scope

- Doughnut, radar, bubble, stock and 3D kinds, combined charts
  (two plots in one chart), data labels, series colours and fills,
  number formats, markers and smoothing, axis bounds, gridlines on
  the category axis, `remove_chart` (part 5); editing a kept chart;
  chartsheets; the caches (decision 3); the chart style, colour
  style and user shapes parts; `chartex`; a chart's formatting
  (`spPr`, `txPr`, fonts, layouts); rendering (LibreOffice's open
  check loads the file; what it draws is not checked here; part 6
  compares with openpyxl).
