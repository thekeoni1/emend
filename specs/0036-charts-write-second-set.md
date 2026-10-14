# specs/0036: charts write, the second set

Status: APPROVED 2026-10-14 (Milestone 5 part 5; decisions 1 to 3
decided A by the maintainer 2026-10-14, typed in the planning
session).

Milestone 5, "charts and read breadth", was approved by the
maintainer 2026-10-11 (`ROADMAP.md`, "Milestone 5"): charts a Cargo
feature `charts` of this crate, off by default, measured in the size
record like every other feature; the Milestone 2 ceiling kept for
the core; the Milestone 2 Excel check gates no push in this
milestone. This part is the fifth of six and the last of the three
chart parts: the kinds part 4 left out, combined charts, data
labels, series colours and number formats, and the removal of a
chart. Part 4 (`specs/0035`) wrote bar, column, line, pie, scatter
and area charts through `Package::add_chart`; this part extends the
same call and its types, and part 3's reader (`specs/0034`) stays
the writer's second acceptance instrument: every chart this part
writes is read back by `Package::charts` and compared field by
field, as far as the reader models it.

## Goal

A consumer adds a doughnut, radar, bubble, stock or 3D chart, or a
chart that combines two plots on one plot area, with data labels,
a solid colour per series and a number format on an axis or on
the labels; and removes a chart from a worksheet it did not write,
the chart part and what only it reached going with it, every other
entry byte for byte. The output opens in LibreOffice, and
`Package::charts` reads each written chart back as specified in
every field it models. Nothing else about a chart is written in
this milestone (`Out of scope`).

## Size and packaging

The budget figure after part 4 is 436992 section bytes against the
ceiling of 458752 (`records/size-m5p4-2026-10-14-574a77d.md`):
21760 of headroom, held for fixes; the core's growth in part 4 was
0 and the `charts` feature costs 85160, of which the writer 40732
(part 4 ruling O: a finding, above the 40000 line and above
planning's 21000 to 34000 estimate). The feature's cost lies
outside the budget, as every feature's does; the finding is about
the estimate and about what a consumer that links `charts` pays.
Everything in this part goes under `charts`, measured by
`probe-emend-charts-write-xml`, whose work gains the new forms and
the removal ("The probe").

The compaction (decision 1). Part 4's emission writes each fixed
element form as its own byte-string literal and
`extend_from_slice` call: `chart_write.rs` holds about seventy
such forms, and its measured rate is about 500 section bytes per
form written, call site and literal together. Planning's reading
of the measured code names two savings that keep every output
byte identical: a `val` element helper, `value_element(out,
name, value)` writing `<c:NAME val="VALUE"/>` from two short
slices, taking the place of some forty literals that differ only
in the name and the value (the `axId`, `grouping`, `varyColors`,
`delete`, `axPos`, `crossAx`, `crosses`, `crossBetween`,
`majorTickMark`, `minorTickMark`, `tickLblPos`, `overlay`,
`legendPos`, `autoTitleDeleted` and plot-option elements); and one
axis tail written by one function for `catAx`, `valAx` and the
`serAx` this part adds, the forms differing by the children each
takes, as the `valAx` function already does for two uses. Under
decision 1 A, P1 compacts first, in its first code commit, with
every part 4 test passing unchanged (the outputs byte-identical,
which the read-back tests and the P4 samples' byte comparison
prove); its saving is printed by P1's gate as the `charts` probe's
size-profile figure on i686 against part 4's 1055368, told to
planning at any value and judged nowhere before P4. Planning
expects 3000 to 8000 back.

Planning's estimate for the part, priced at the measured rate per
form and per arm, each item's forms counted from "The chart part"
below: the kinds (doughnut 3 forms, radar 3 and one arm of three,
bubble 6, stock 5, the four 3D plots 8 with `view3D` 2 and `serAx`
6: 33 forms, five enumeration arms) 15000 to 20000; combined
charts (the second plot's series, the secondary pair of axes with
its `delete` and `crosses` forms, 8 forms) 4000 to 6000; data
labels (8 forms, a position arm of nine), series colours (4 forms)
and number formats (3 forms) 7000 to 9000; `remove_chart` (the
anchor removal shared with `images` taken by a predicate, the
chart part, its rels part and the parts those rels alone reach
removed with their `Override`s, the drawing's emptying shared)
5000 to 8000; the call's new checks 2000 to 3000. In all 33000 to
46000 before the compaction, 25000 to 43000 after it. Planning is
told the figure in the report at any value; a writer cost (the
probe's movement over part 4's 1055368, the compaction included)
above 48000, or core growth above 2000, is a finding the report
names. The `images` feature's own cost may move by the predicate
the removal shares (expected 0, accepted up to 2000): the report
prints and judges the movement, as part 4's did.

## Facts planning checked (2026-10-14)

- Part 4's writer: `crates/emend/src/package/chart_write.rs` at
  574a77d, 669 lines; `NewChart` holds `plot`, `title`, `series`,
  `category_axis_title`, `value_axis_title`, `legend`, `size`;
  `NewSeries` holds `name`, `categories`, `values`; `NewPlot` has
  five variants; the emission is literal forms with
  `extend_from_slice`; `value_axis_bytes` already serves the
  scatter's two axes and every other plot's value axis.
- Part 3's reader models, per plot, `kind` (sixteen `ChartKind`
  variants: `Bar`, `Bar3D`, `Line`, `Line3D`, `Pie`, `Pie3D`,
  `Doughnut`, `OfPie`, `Scatter`, `Area`, `Area3D`, `Radar`,
  `Bubble`, `Stock`, `Surface`, `Surface3D`), `bar_direction`,
  `grouping`, `scatter_style`, `vary_colors`, `series`,
  `axis_ids`; per series `index`, `order`, `name`, `categories`,
  `values`, `sizes` (`bubbleSize`), `fill` (`spPr/solidFill/
  srgbClr` as `0xRRGGBB`), `smooth`, `marker`, `labels` (a `dLbls`
  present); per axis `kind` (`Category`, `Value`, `Date`,
  `Series`), `position`, `crosses`, `title`, `number_format`
  (`numFmt`'s `formatCode`), `min`, `max`, `deleted`,
  `major_gridlines`. It walks past `dLbls`'s content, `view3D`,
  `hiLowLines`, `upDownBars`, `dropLines` and `spPr` other than
  the solid fill. Radar style, hole size, bubble scale and 3D
  shape are not modelled: the read-back compares what is.
- The image removal (`Package::remove_image`, `specs/0028`):
  `image_removals` holds `(part, index)`; the write finds the
  index-th picture anchor, removes it, removes the relationships
  no kept element names, and the media each removed relationship
  targets when no relationship of the package reaches it; a
  drawing left with no anchor goes with its rels part, its
  `Override`, the sheet's relationship and its `<drawing>`.
  Refusals: an index past the list, an index named twice, a part
  this write also adds an image to.
- The corpus's charts (the per-workbook `charts` lines and the
  totals in `records/logs/m5p3-p3/run.log`, counts only; part 4
  P3's rerun was byte-identical): 26 charts over 27 workbooks,
  2 combined (two plots in one plot area), 1 doughnut, 15 series
  with labels, 12 with a solid fill, 43 axis number formats, 16 charts
  with a `style` value, 5 pivot charts; the chart style and
  colours parts (`xl/charts/style<N>.xml`, `colors<N>.xml`) are
  reached from a chart part's rels, which is what the removal
  follows. No radar, bubble, stock or 3D chart in the
  corpus: those kinds are proven on built packages and by the
  LibreOffice open check of the samples' outputs.
- The schema (ECMA-376 part 1, 21.2.2, planning's reading; a
  LibreOffice refusal of a written form in P1's or P2's tests is
  a finding against this reading): `doughnutChart` holds
  `varyColors`, `ser`*, `dLbls`?, `firstSliceAng`, `holeSize`
  (1 to 90); `radarChart` holds `radarStyle` (`standard`,
  `marker`, `filled`), `varyColors`, `ser`*, `dLbls`?, `axId`
  x2; `bubbleChart` holds `varyColors`, `ser`* (each with
  `xVal`, `yVal`, `bubbleSize`), `dLbls`?, `bubble3D`?,
  `bubbleScale`, `showNegBubbles`, `axId` x2; `stockChart` holds
  `ser` x3 or x4, `dLbls`?, `dropLines`?, `hiLowLines`?,
  `upDownBars`?, `axId` x2; `bar3DChart` holds `barDir`,
  `grouping`, `varyColors`, `ser`*, `dLbls`?, `gapWidth`,
  `gapDepth`?, `shape`?, `axId` x2 or x3; `line3DChart` holds
  `grouping`, `varyColors`, `ser`*, `dLbls`?, `dropLines`?,
  `gapDepth`?, `axId` x3; `pie3DChart` holds `varyColors`,
  `ser`*, `dLbls`?; `area3DChart` holds `grouping`,
  `varyColors`, `ser`*, `dLbls`?, `dropLines`?, `gapDepth`?,
  `axId` x2 or x3; `view3D` (`rotX`, `hPercent`?, `rotY`,
  `depthPercent`?, `rAngAx`, `perspective`?) sits in `chart`
  after `autoTitleDeleted` and before `plotArea`; `serAx` holds
  `axId`, `scaling`, `delete`, `axPos`, `title`?, `numFmt`?,
  `majorTickMark`, `minorTickMark`, `tickLblPos`, `crossAx`,
  `crosses`, `tickLblSkip`?, `tickMarkSkip`?; `dLbls` after a
  series' `dPt`s and before `cat` holds `numFmt`?, `spPr`?,
  `txPr`?, `dLblPos`?, `showLegendKey`, `showVal`, `showCatName`,
  `showSerName`, `showPercent`, `showBubbleSize`; a series'
  `spPr` sits after `tx` and before `invertIfNegative`, `marker`
  or `dPt`; `numFmt` carries `formatCode` and `sourceLinked`.
  Element order inside `ser` by plot: bar `idx`, `order`, `tx`,
  `spPr`, `invertIfNegative`?, `dLbls`?, `cat`, `val`, `shape`?;
  line, radar and stock `idx`, `order`, `tx`, `spPr`, `marker`?,
  `dLbls`?, `cat`, `val`, `smooth`?; area `idx`, `order`, `tx`,
  `spPr`, `dLbls`?, `cat`, `val`; pie and doughnut `idx`,
  `order`, `tx`, `spPr`, `explosion`?, `dLbls`?, `cat`, `val`;
  scatter `idx`, `order`, `tx`, `spPr`, `marker`?, `dLbls`?,
  `xVal`, `yVal`, `smooth`?; bubble `idx`, `order`, `tx`, `spPr`,
  `invertIfNegative`?, `dLbls`?, `xVal`, `yVal`, `bubbleSize`,
  `bubble3D`?.
- `tools/emend-copy` at 574a77d: `--remove-image-sample` adds an
  image, writes into memory, reopens, removes the added image
  and, when the input had one, the first kept one, and prints
  `images_before`, `images_after`, `kept_removed`, `media_before`,
  `media_after`, `media_dropped`, `drawing_dropped`,
  `anchors_before`, `anchors_after`; the removal samples of this
  part follow that shape.

## Design

### Crate

Everything under the feature `charts`, in `package/chart_write.rs`
(the call, the forms) and `package.rs` (the removal beside the
image removal, the drawing walk shared). No new dependency. The
feature gates `images` and `charts` share for the drawing writer
and now the anchor removal stay `any(feature = "images", feature
= "charts")` (part 4 ruling A).

### The call

`Package::add_chart(&mut self, part: &[u8], anchor: ImageAnchor,
chart: &NewChart<'_>)` is unchanged in signature. The types grow
(the crate is unpublished, Milestone 4 decision 1 HOLD; the API
listing marks each changed row `changed`, with the verdict `keep`):

```
pub struct NewChart<'a> {
    pub plot: NewPlot,
    pub title: Option<&'a str>,
    pub series: &'a [NewSeries<'a>],
    pub category_axis_title: Option<&'a str>,
    pub value_axis_title: Option<&'a str>,
    pub category_axis_format: Option<&'a str>,   // new
    pub value_axis_format: Option<&'a str>,      // new
    pub legend: Option<LegendPosition>,
    pub size: Option<(u32, u32)>,
    pub combined: Option<NewCombined<'a>>,       // new
}

pub enum NewPlot {
    Bar { direction: BarDirection, grouping: Grouping },
    Line { grouping: Grouping },
    Pie,
    Scatter { style: ScatterStyle },
    Area { grouping: Grouping },
    Doughnut { hole_size: u8 },                  // new, 1 to 90
    Radar { style: RadarStyle },                 // new
    Bubble,                                      // new
    Stock,                                       // new
    Bar3D { direction: BarDirection, grouping: Grouping },  // new
    Line3D { grouping: Grouping },               // new
    Pie3D,                                       // new
    Area3D { grouping: Grouping },               // new
}

pub enum RadarStyle { Standard, Marker, Filled } // new

pub struct NewSeries<'a> {
    pub name: Option<NewSeriesName<'a>>,
    pub categories: Option<&'a str>,
    pub values: &'a str,
    pub sizes: Option<&'a str>,                  // new, bubbleSize
    pub color: Option<u32>,                      // new, 0xRRGGBB
    pub labels: Option<NewLabels<'a>>,           // new
}

pub struct NewLabels<'a> {                       // new
    pub value: bool,
    pub category: bool,
    pub series_name: bool,
    pub percent: bool,
    pub position: Option<LabelPosition>,
    pub number_format: Option<&'a str>,
}

pub enum LabelPosition {                         // new
    BestFit, Center, InsideEnd, InsideBase, OutsideEnd,
    Left, Right, Above, Below,
}

pub struct NewCombined<'a> {                     // new
    pub plot: NewPlot,
    pub series: &'a [NewSeries<'a>],
    pub secondary_axis: bool,
    pub secondary_axis_title: Option<&'a str>,
    pub secondary_axis_format: Option<&'a str>,
}
```

`Package::remove_chart(&mut self, part: &[u8], index: usize) ->
Result<(), Error>` (new): records the removal of the `index`-th
chart of `part` in the order `Package::charts` lists them, as
`remove_image` records an image's.

### Texts and references

As part 4 ("Texts and references", ruling F): a title, an axis
title, a text name, a format code and a label format are texts
(not empty, no character below U+0020, no U+FFFE or U+FFFF, at
most 32767 bytes, escaped); `sizes` is a reference (at most 8192
bytes, `&`, `<`, `>` escaped, neither parsed nor quoted). A format
code is written as given into `formatCode`: the crate checks it
as a text and nothing more (openpyxl stores the same string
unchecked; Excel and LibreOffice read a code they do not
understand as `General`). A colour is a `u32` at most `0xFFFFFF`,
written as six upper-case hex digits.

### Applicability

The part 4 rules stand. The new ones, each refused with "a chart
option does not apply to its kind" unless named otherwise:

- `Doughnut`: `hole_size` 1 to 90; any number of series; no axis
  title and no axis format (as a pie). `Pie3D`: as `Pie`.
- `Radar`: `Standard`, `Marker` or `Filled`; two axes as a line.
- `Bubble`: every series carries `sizes`; two value axes as a
  scatter. Any other plot's series with `sizes` is refused.
- `Stock`: three or four series (high, low, close; or open, high,
  low, close), in that order, the caller's responsibility; no
  colour on a series (the lines are the high-low lines); the
  category axis and the value axis as a line.
- `Bar3D`: `Clustered`, `Stacked`, `PercentStacked` or `Standard`;
  `Standard` adds the series axis. `Line3D` and `Area3D`:
  `Standard`, `Stacked` or `PercentStacked`; `Standard` adds the
  series axis. `Clustered` on `Line3D` or `Area3D` is refused.
- `labels.percent` only on `Pie`, `Pie3D` and `Doughnut`;
  `labels.position` not on `Area`, `Area3D`, `Stock`, `Bubble`,
  the 3D plots, a `Radar` with `Filled`, or a `Line3D`; on the
  others every position the plot takes: `Bar` and `Bar3D` take
  `Center`, `InsideEnd`, `InsideBase`, `OutsideEnd` (`OutsideEnd`
  not on a stacked or percent-stacked bar); `Line`, `Scatter` and
  `Radar` take `Center`, `Left`, `Right`, `Above`, `Below`; `Pie`
  and `Pie3D` take `BestFit`, `Center`, `InsideEnd`, `OutsideEnd`;
  `Doughnut` takes none (a label on a doughnut sits at its
  default). A `position` the plot does not take is refused.
- `category_axis_format` and `value_axis_format`: not on `Pie`,
  `Pie3D` or `Doughnut`; `category_axis_format` not on a
  `Scatter` or `Bubble` (their x axis is a value axis, which
  takes `value_axis_format` for the y axis only in this
  milestone).
- `combined`: the first plot `Bar`, `Line`, `Area`, `Scatter`,
  `Bar3D`, `Line3D` or `Area3D` and the second `Bar`, `Line`,
  `Area` or `Scatter` of a different variant than the first's
  (a bar and a line, a line and an area, a bar and a scatter);
  the second's series one or more, each checked as the first's;
  the second plot's `grouping` and `style` rules as its own;
  the first plot not `Pie`, `Pie3D`, `Doughnut`, `Radar`,
  `Bubble` or `Stock`, and the second not a 3D plot. `Scatter`
  as either plot requires `secondary_axis` (a scatter's x axis is
  a value axis and cannot share a category axis), and a `Scatter`
  first plot is refused with a non-scatter second. The
  `secondary_axis_title` and `secondary_axis_format` require
  `secondary_axis`.
- A `color` on a series of a `Pie`, `Pie3D` or `Doughnut` colours
  the whole series (every slice), which is what the file says;
  accepted.

The check order (part 4 ruling C) extends: the grid; the anchor
range; the size applicability; the size range; no series; then
the first plot's kind rules (grouping, series count, hole size,
`sizes`, the axis titles and formats, the labels); then the
combined plot's rules; then the texts in field order (the title,
the category axis title, the value axis title, the category axis
format, the value axis format, the secondary axis title, the
secondary axis format, then series by series the name, the
categories, the values, the sizes, the label format, the first
plot's series then the second's).

### Anchors

As part 4 ("Anchors"): unchanged.

### The parts

As part 4 ("The parts") for an addition. For a removal: the chart
part the removed frame's relationship targets is removed with its
`Override`; its rels part (`xl/charts/_rels/chart<N>.xml.rels`),
when present, is removed, and each part a relationship of it
targets (the chart style and colours parts, a user-shapes drawing
with its own rels part, an embedded package) is removed with its
`Override` when no relationship of the package reaches it once
the removed relationships are left out, as the image remover
drops media; a relationship of the removed rels part with
`TargetMode="External"` is dropped with the part and reaches
nothing. The drawing's `CHART` relationship goes as a picture's
`r:embed` does, and a drawing left with no anchor goes as
`remove_image` says. A chart part reached by two frames (two
sheets, or one sheet twice; a form Excel does not write but the
reader accepts) is kept while any frame names it. Nothing else in
the package is touched; `calcChain` and the cached values of the
sheet are not the chart's.

### The chart part

Part 4's "The chart part" stands for the five plots and the axes,
with these additions. In `chart`, after `autoTitleDeleted` and
before `plotArea`, for a 3D plot:

```
<c:view3D><c:rotX val="X"/><c:rotY val="Y"/><c:rAngAx val="R"/>
</c:view3D>
```

`X`, `Y`, `R` are `15`, `20`, `1` for `Bar3D`, `Line3D` and
`Area3D`, and `30`, `0`, `0` for `Pie3D`.

The new plots (SERIES as part 4's with the series additions
below; `G`, `D`, `S` as part 4's; `H` the hole size; `RS`
`standard`, `marker` or `filled`):

```
<c:doughnutChart><c:varyColors val="1"/>SERIES
<c:firstSliceAng val="0"/><c:holeSize val="H"/></c:doughnutChart>
<c:radarChart><c:radarStyle val="RS"/><c:varyColors val="0"/>
SERIES<c:axId val="10"/><c:axId val="20"/></c:radarChart>
<c:bubbleChart><c:varyColors val="0"/>SERIES
<c:bubbleScale val="100"/><c:showNegBubbles val="0"/>
<c:axId val="10"/><c:axId val="20"/></c:bubbleChart>
<c:stockChart>SERIES<c:hiLowLines/>
<c:axId val="10"/><c:axId val="20"/></c:stockChart>
<c:bar3DChart><c:barDir val="D"/><c:grouping val="G"/>
<c:varyColors val="0"/>SERIES<c:gapWidth val="150"/>
<c:shape val="box"/><c:axId val="10"/><c:axId val="20"/>
[<c:axId val="30"/>]</c:bar3DChart>
<c:line3DChart><c:grouping val="G"/><c:varyColors val="0"/>
SERIES<c:axId val="10"/><c:axId val="20"/>[<c:axId val="30"/>]
</c:line3DChart>
<c:pie3DChart><c:varyColors val="1"/>SERIES</c:pie3DChart>
<c:area3DChart><c:grouping val="G"/><c:varyColors val="0"/>
SERIES<c:axId val="10"/><c:axId val="20"/>[<c:axId val="30"/>]
</c:area3DChart>
```

The third `axId` is written when the series axis is (grouping
`Standard` on a 3D plot). A stock plot's series write
`<c:marker><c:symbol val="none"/></c:marker>` after `spPr`'s
position and no `smooth`; a bubble plot's series write `xVal`,
`yVal` and `<c:bubbleSize><c:numRef><c:f>REF</c:f></c:numRef>
</c:bubbleSize>`.

The series additions, in schema order inside `ser` ("Facts"):
after `tx`, with a `color`,

```
<c:spPr><a:solidFill><a:srgbClr val="RRGGBB"/></a:solidFill>
[<a:ln><a:solidFill><a:srgbClr val="RRGGBB"/></a:solidFill></a:ln>]
</c:spPr>
```

the `ln` form on `Line`, `Line3D`, `Scatter`, `Radar` and a
stock's series (the line is what shows); then, with `labels`,
before `cat` or `xVal`,

```
<c:dLbls>[<c:numFmt formatCode="F" sourceLinked="0"/>]
[<c:dLblPos val="P"/>]<c:showLegendKey val="0"/>
<c:showVal val="V"/><c:showCatName val="C"/>
<c:showSerName val="N"/><c:showPercent val="E"/>
<c:showBubbleSize val="0"/></c:dLbls>
```

`V`, `C`, `N`, `E` `1` or `0` from the flags; `P` `bestFit`,
`ctr`, `inEnd`, `inBase`, `outEnd`, `l`, `r`, `t`, `b`. A `labels`
with every flag false is accepted and writes the form (Excel
shows nothing; the reader reads `labels` true).

The axes: part 4's forms, with `numFmt formatCode="F"
sourceLinked="0"` in place of `General` `1` when a format is
given; a radar's axes are a line's; a bubble's are a scatter's.
The series axis, after the value axis:

```
<c:serAx><c:axId val="30"/><c:scaling><c:orientation val="minMax"/>
</c:scaling><c:delete val="0"/><c:axPos val="b"/>
<c:majorTickMark val="out"/><c:minorTickMark val="none"/>
<c:tickLblPos val="nextTo"/><c:crossAx val="20"/>
<c:crosses val="autoZero"/></c:serAx>
```

A combined chart writes the first plot, then the second plot
element with its series (`idx` and `order` continuing the count),
in `plotArea` after the first; without `secondary_axis` the
second names `axId` `10` and `20`; with it, `30` and `40`, and
after the primary axes a second pair follows: a `catAx` `30`
(`delete` `1`, `axPos` `b`, `crossAx` `40`) and a `valAx` `40`
(`axPos` `r`, `crossAx` `30`, `crosses` `max`, no
`majorGridlines`, the secondary title and format), or for a
scatter second plot two `valAx` (`30` at `b` deleted, `40` at
`r`, `crosses` `max`, `crossBetween` `midCat`). The reader reads
the plots in order and the axes in order; the read-back compares
the plot kinds, the series counts and the axis ids.

### Errors

Two new `Error::Package` messages under `charts`: "no chart at
that position" (a removal whose index is past the list, named
twice, or on a part this write also adds a chart to; as "no image
at that position") and "a chart part cannot be read" (a removal
whose frame names a chart relationship with no part, or a chart
part that is not a `chartSpace`; raised at the write). The part 4
messages cover the new rules: "a chart option does not apply to
its kind" for every applicability rule above, "a chart text is
empty or holds a control character" for the new texts and the
`sizes` reference, "a chart anchor is out of range" unchanged. A
colour above `0xFFFFFF` is refused with "a chart option does not
apply to its kind". At the write, the drawing's messages as part
4 says. Nothing is checked twice.

### 32-bit and streaming

As part 4 ("32-bit and streaming"): the chart part is built in
memory; a removal reads the chart part's rels part and the
`[Content_Types].xml` once, as the image removal does; every
count `u64`.

### Harness runs

`tools/emend-copy` gains four operations, given after the two
paths as the others are; no reference, name or number from the
command line; every constant below is the tool's own. W, M and
the quoting are part 4's. Nothing from the corpus is printed.

- `--chart-more-sample <in> <out>` (P1): four `add_chart` calls
  on W, each `OneCell { from: (M + 2 + 16 k, 1), offset: (0, 0) }`
  for k 0 to 3 with `size: Some((480, 288))`, the plots in order
  `Doughnut { hole_size: 50 }`, `Radar { Marker }`, `Bubble`,
  `Stock`, each `title: Some("emend more k")`, `legend:
  Some(Right)`; the doughnut and the radar one series `NewSeries
  { name: Some(Reference(B1 of W)), categories: Some(A2:A(M'+1)),
  values: B2:B(M'+1) }` (M' = max(M, 1), part 4 ruling J); the
  bubble the same with `sizes: Some(C2:C(M'+1))`; the stock three
  series over B, C and D with names `Text("high")`, `Text("low")`,
  `Text("close")` and categories A; axis titles `Some("emend x")`,
  `Some("emend y")` for the radar, bubble and stock, none for
  the doughnut. Prints `chartmore` with `charts_before`,
  `charts_after` (`+ 4`, asserted), `kinds_read_back` (the four
  last entries of `charts(W)` whose plot reads as specified: kind,
  series count, `sizes` present on the bubble's series, axes
  count; 4, asserted), `chart_parts_after` (`+ 4`, asserted).
  Expected results: as part 4's kinds sample with four parts
  `added`; `lo` and `lo_in` `ok` for every corpus workbook.
- `--chart-3d-sample <in> <out>` (P2): five `add_chart` calls on
  W as above for k 0 to 4: `Bar3D { Column, Clustered }`, `Line3D
  { Standard }` (the series axis), `Pie3D`, `Area3D { Stacked }`,
  and a combined `Bar { Column, Clustered }` with `combined:
  Some(NewCombined { plot: Line { Standard }, series: [one series
  over C], secondary_axis: true, secondary_axis_title: Some("emend
  y2"), secondary_axis_format: Some("0.0") })`; each one series
  as the kinds sample's, `title: Some("emend 3d k")`, `legend:
  Some(Bottom)`. Prints `chart3d` with `charts_before`,
  `charts_after` (`+ 5`), `kinds_read_back` (5: kind, series
  count, axes count 3 for the `Line3D`, the combined's two plots
  and four axes with ids 10, 20, 30, 40 and the secondary axis's
  `number_format` `Some("0.0")`), `chart_parts_after` (`+ 5`),
  each asserted. Expected results: as the more sample's with five
  parts `added`.
- `--chart-labels-sample <in> <out>` (P3): two `add_chart` calls
  on W: a `Bar { Column, Clustered }` with two series over B and
  C, `color: Some(0x1F77B4)` and `Some(0xFF7F0E)`, `labels:
  Some(NewLabels { value: true, category: false, series_name:
  false, percent: false, position: Some(OutsideEnd),
  number_format: Some("#,##0") })` on both, `category_axis_format:
  Some("@")`, `value_axis_format: Some("#,##0.00")`; and a `Pie`
  with one series, `color: Some(0x2CA02C)`, `labels: Some(NewLabels
  { value: false, category: true, series_name: false, percent:
  true, position: Some(BestFit), number_format: None })`. Prints
  `chartlabels` with `charts_before`, `charts_after` (`+ 2`),
  `read_back` (2: every series' `fill` and `labels` true, the bar's
  axes' `number_format` `Some("@")` and `Some("#,##0.00")`),
  `chart_parts_after` (`+ 2`), each asserted.
- `--remove-chart-sample <in> <out>` (P3): adds one chart as the
  add-chart sample's (two-cell anchor) on W, writes into memory,
  reopens, removes the added chart (the last of `charts(W)`) and,
  when the input's W had a chart, its first one (`kept_removed`
  1); writes; reopens the output and prints `removechart` with
  `charts_before`, `charts_after` (`charts_before - kept_removed`,
  asserted), `kept_removed`, `chart_parts_before`,
  `chart_parts_after` (the `xl/charts/` entries other than rels,
  style and colours parts), `parts_dropped` (every entry the
  output lacks that the input had: the chart part, its rels, the
  style and colours parts it alone reached, counted), `drawing_
  dropped` (1 when W's drawing went with its last anchor),
  `anchors_before`, `anchors_after`, `charts_other_sheets` (the
  charts of every other sheet, before and after equal, asserted).
  Expected results: when `kept_removed` is 1, the chart part and
  its rels `removed`, each part it alone reached `removed`, W's
  drawing and its rels `changed` (or `removed` with W's part
  `changed` when `drawing_dropped` is 1), `[Content_Types].xml`
  `changed`, every other entry `identical`; when `kept_removed` is
  0, every entry `identical` (the addition and its removal cancel
  byte for byte: a strong fidelity check of the removal), the
  output a byte copy of the input (`identical` for every entry,
  asserted in the tool). `lo` and `lo_in` `ok` for every corpus
  workbook.
- The two part 4 samples (`--add-chart-sample`,
  `--chart-kinds-sample`) and the three image samples rerun in
  P1 on both targets after the compaction: their outputs and
  lines byte-identical to their latest records
  (`records/m5p4-p3-addchart-2026-10-14-574a77d.txt`,
  `records/m5p4-p3-chartkinds-2026-10-14-574a77d.txt`,
  `records/m3p1-p3-image-2026-10-02-6578925.txt`,
  `m4p1-p3-anchor-2026-10-10-669fa91.txt`,
  `m4p1-p3-removeimage-2026-10-10-669fa91.txt`, each by sha256 in
  the run log), proving the compaction and the shared predicate
  change no output byte; a difference is a finding.
- `--charts-sample` rerun in P4 with the release binaries,
  byte-identical to part 3's record as part 4 P3 read it (part 4
  ruling S), proving the reader untouched.

The keep and the edit open check as every run log.

### The probe

`probe-emend-charts-write-xml` keeps part 4's work and gains, on
the same second package: `remove_chart(first, 0)` of the chart it
added, then a second `add_chart` whose plot kind comes from the
probe's command line as an index 0 to 12 over the thirteen
`NewPlot` variants (the probe's constants give each variant's
options; the index in the size log's run is 9, `Bar3D { Column,
Clustered }`), combined with a `Line { Standard }` second plot
on a secondary axis with a title and a format, two series with a
colour, labels (`value` true, `OutsideEnd`, a format) and the
axis formats, written into memory and reopened as part 4's is,
then `charts`: `charts 1` and `chart_series 3`. The kind index
from the command line keeps every plot arm live; the removal, the
combined pair, the labels, the colours and the formats are
linked by the call. The second output stays the format probe's,
byte for byte.

## Phases

As `specs/0001` and part 4's: a doc commit first, code commits,
one record commit, the scan last, at most two independent checks
per phase, a report, STOP. One target root per phase,
`<home>/.cache/emend-build/m5p5-<phase>/target`. P1's gate prunes
part 4's four roots (`<home>/.cache/emend-build/m5p4-p1/target`,
`m5p4-p2/target`, `m5p4-p3/target` and `m5p4-close/target`), each
listed with `du -sb` and removed with `rm -r`; the close-out's
`gen.py` and logs stay. Every rule of part 4's "Phases" paragraph
carries. Logs in `records/logs/m5p5-<phase>/` (`gate.log`,
`test.log` on both targets with the six feature sets, `negative
.log`, `run.log`, `scan.log`, `size.log` in P4); the scan's
denylist form whole-word. The gate takes the test figures from
the previous phase's committed `test.log` (part 4 ruling T). No
push during the part; the close-out push follows P4 and carries
the record commit d2c14d8 of part 4's close-out with this part's.

Part 5's first doc commit carries: this spec; `ROADMAP.md` with
part 4's Close row DONE (record d2c14d8), the M5 heading and this
part's rows; `specs/0035-charts-write-first-set.md` with
"Planning rulings on the close-out (2026-10-14)" (rulings U and
V), as the working tree holds them.

### P1: the compaction, doughnut, radar, bubble and stock charts

The doc commit; the prunes; the carries from part 4: the probe's
doc comment corrected (ruling R, in P1's first code commit, the
compaction's); then the compaction (decision 1 A: the `val`
element helper and the shared axis tail, every part 4 test
passing unchanged, the four part 4 samples and the three image
samples byte-identical to their records in the run log, the
`charts` probe's size-profile figure printed by the gate against
1055368); then `Doughnut`, `Radar`, `RadarStyle`, `Bubble`,
`Stock`, `NewSeries::sizes`, their forms and rules, and
`--chart-more-sample`. Tests on built packages, each written
chart read back and compared: a doughnut with two series and hole
size 50 (`varyColors` read true, no axis), hole size 0 and 91
refused; a radar with each style; a bubble with `sizes` on every
series (`sizes` read back), one series without `sizes` refused, a
bar series with `sizes` refused; a stock with three and with four
series, two and five refused, a colour on a stock series refused;
the tool's line. The gate as part 4 P1's plus the prunes and the
probe figure. The negative log through the binary: the add-chart
sample on a fixture whose worksheet holds two `drawing` elements
(refused with the two-drawing message, with a control; part 4
ruling Q), and the part 1 messages for a sheet added in the same
write, a replaced and a removed part stated reachable or
unreachable, each with its covering test. The run log: the more
sample on both targets over the corpus, `lo` and `lo_in` `ok` for
every workbook, the five reruns byte-identical, the keep and the
edit open check, recorded as `records/m5p5-p1-<utc-date>-
<shortsha>.md` and `.txt`.

### P2: the 3D kinds and combined charts

The doc commit (the P1 verification rulings); the carry; then
`Bar3D`, `Line3D`, `Pie3D`, `Area3D`, `view3D`, the series axis,
`NewCombined`, `NewChart::combined`, the secondary axis pair,
their rules, and `--chart-3d-sample`. Tests on built packages,
each read back and compared: each 3D plot with each grouping it
takes (`Standard` reading three axis ids and a `Series` axis;
`Clustered` on `Line3D` refused), `Pie3D` as a pie; a bar with a
combined line on the primary axes (two plots, two axes) and on a
secondary axis (four axes, ids 10 to 40, the `valAx` `40` at
`Right`, `crosses` `30`, the title and format read back), a line
with a combined area, a bar with a combined scatter on a
secondary axis, a scatter first plot with a bar second refused, a
pie with a combined plot refused, a combined 3D second plot
refused, a secondary title without `secondary_axis` refused; the
tool's line. The gate as P1's. The negative log through the
binary: the 3D sample on a fixture whose drawing root is not
`wsDr` (refused, with a control; part 4 ruling K's case). The run
log: the more and 3D samples on both targets over the corpus,
recorded as `records/m5p5-p2-<utc-date>-<shortsha>.md` and
`.txt`.

### P3: labels, colours, number formats; `remove_chart`; the catalogue

The doc commit (the P2 verification rulings); the carry; then
`NewLabels`, `LabelPosition`, `NewSeries::color` and `labels`,
`NewChart::category_axis_format` and `value_axis_format`, their
rules and forms; `remove_chart` with the two messages, the shared
anchor predicate, the part removals; `--chart-labels-sample` and
`--remove-chart-sample`; then the `package` header's catalogue
and limits for this part. Tests on built packages: labels on a
bar series with each position it takes and one it does not
(refused), `percent` on a pie and on a bar (refused), a label
format read back through the file's bytes (the reader does not
model it: the test asserts the `dLbls` bytes), a colour on a bar
series (`fill` read back) and on a line series (the `ln` form in
the bytes), a colour above `0xFFFFFF` refused, each axis format
read back, a format on a pie refused; a removal of the only chart
of a sheet (the frame, the relationship, the chart part, its
rels, its style and colours parts, the `Override`s and the drawing
gone), of the first of two (the second kept byte for byte, its
part kept), of a chart whose style part another chart's rels also
reaches (the part kept), an index past the list and an index
named twice refused, a removal and an addition on one part
refused, a frame naming a chart relationship with no part refused
at the write; an image and a chart removed from one sheet in one
write; the tool's lines. The gate as P1's. The negative log
through the binary: the remove-chart sample on a fixture whose
chart frame names a relationship with no part (refused with the
cannot-be-read message, with a control), and the two new messages
otherwise stated with their covering tests. The run log: the
labels and remove-chart samples on both targets over the corpus,
with the keep and the edit open check, recorded as
`records/m5p5-p3-<utc-date>-<shortsha>.md` and `.txt` for each.

### P4: the probe, the size record, the API listing, the CHANGELOG

The doc commit (the P3 verification rulings); the carry; then the
probe's additions as "The probe" says; the size log, recorded as
`records/size-m5p5-<utc-date>-<shortsha>.md`, with the `charts`
cost, its movement over part 4's 85160 (the writer's cost, the
compaction included, judged against 48000), the `images` cost's
movement (expected 0, accepted to 2000) and the core's growth
(expected 0, accepted to 2000) each printed and judged; the API
listing rerun as part 4 P3 did, recorded as
`records/api-review-m5p5-<utc-date>-<shortsha>.md`, the changed
rows named; `CHANGELOG.md`'s "Added" gains one bullet citing this
spec. The negative log through the binary for every message of
the part reachable there, the unreachable ones stated with the
test that covers each. The run log: the four samples on both
targets over the corpus with the size log's release binaries,
byte-identical to their phase runs by sha256, and
`--charts-sample` once more (part 4 ruling S's reading), recorded
as `records/m5p5-p4-<sample>-<utc-date>-<shortsha>.md` and `.txt`.
The open record of the part's keep.

### Close-out push (after P4)

As part 4's (`specs/0035` "Close-out push (after P3)" and its
close-out rulings U and V): the generator transforms part 4's
scripts (`gen.py` bc2ea873 and the three it printed, carried
cmp-equal), its numbered changes continuing from 41; the doc
commit holds this spec and `ROADMAP.md`; the base is part 4's
close-out doc commit 934d2e1, named by its subject ("M5 part 4
close-out: commit planning docs (the P3 verification and
rulings)"); `PREV_RECORD_PATTERN` is `^M5 part 4 close-out record`
(d2c14d8), in the range exactly once; the close-out doc commit's
subject is "M5 part 5 close-out: commit planning docs (the P4
verification and rulings)"; `CARGO_TARGET_DIR` printed and checked
as `<home>/.cache/emend-build/m5p5-close/target`; no Excel-record
check, the stray-record count 0 kept; the scan's denylist form
whole-word; the record commit "M5 part 5 close-out record: the
push, the CI run and the scan" holding the three logs only; pre
mode on a clean tree; one independent check before the report.
The push carries d2c14d8 and this part's commits, on planning's
word and the maintainer's authorisation typed in the implementing
session.

## Decisions for the maintainer

Decided by the maintainer 2026-10-14: 1, 2 and 3 as recommended (A
in each), typed in the planning session.

1. The compaction ("Size and packaging"): A (recommended), compact
   part 4's emission in P1's first code commit (the `val` element
   helper and the shared axis tail), outputs byte-identical,
   proven by the tests and the seven sample reruns, the saving
   printed early and measured in P4; B, no compaction, the new
   forms written as part 4's are and the writer's cost judged
   against 48000 from 1055368 as is.
2. Combined charts ("The call", "The chart part"): A
   (recommended), two plots on one plot area with an optional
   secondary value axis (`NewCombined::secondary_axis`, the axis
   pair 30/40, its title and format): the common bar-and-line on
   two scales, which openpyxl writes the same way; B, two plots
   sharing the primary axes only, the secondary axis out of scope
   (about 2000 section bytes less).
3. The removal's reach ("The parts"): A (recommended), a removed
   chart part takes with it its rels part and every part those
   relationships alone reach (the chart style and colours parts
   Excel writes beside every chart, a user-shapes drawing, an
   embedded package), as the image remover drops media no
   relationship reaches, leaving no orphan entry; B, the chart
   part and its rels part only, the style and colours parts left
   as orphans in the package (no reader minds; about 1500 section
   bytes less; the harness lists them as `identical`).

## Out of scope

Surface and of-pie charts; chartsheets; trendlines, error bars,
drop lines and up-down bars; per-point formatting (`dPt`); line
widths, dash styles, marker symbols and sizes beyond the stock's
`none`; gradient and pattern fills; axis scaling (`min`, `max`,
`logBase`), tick skips and date axes; a secondary category axis
with its own categories; a chart's `style` element and the chart
style and colours parts for a written chart; the text properties
(`txPr`) of titles and labels; editing a kept chart's series or
title in place (a consumer removes and adds); caches; formula
checking of references and of format codes (decision 2 of part
4). Any of them is a later milestone's part.
