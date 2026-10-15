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
  this write also adds an image or a chart to (ruling O).
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
  `Bar3D`, `Line3D` or a `Radar` with `Filled` (ruling L); on the
  others every position the plot takes: `Bar` takes
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
- `combined`: the first plot `Bar`, `Line` or `Area` (a 3D first
  plot refused, ruling G; a `Scatter` first plot refused in every
  case, ruling J) and the second `Bar`, `Line`, `Area` or
  `Scatter` of a different variant than the first's
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
twice, or on a part this write also adds an image or a chart to,
as "no image at that position"; ruling O) and "a chart part
cannot be read" (a removal whose frame names a chart relationship
with no part, or a chart part that is not a `chartSpace`; raised
at the write). The part 4
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
  0, as the image removal sample's record (`specs/0028` ruling F1;
  ruling M): every entry of the input is in the output with the
  same uncompressed bytes (asserted in the tool), `identical` or
  `recompressed` in the harness (the second write deflates what
  the first wrote), except that a rels part the addition created
  (W's or its drawing's) may be `added` and left with no
  relationship, and W's drawing goes when it held no anchor
  before the write (`drawing_dropped` 1; its rels part and
  `Override` with it; W, its rels part and `[Content_Types].xml`
  `changed`). `lo` and `lo_in` `ok` for every corpus workbook.
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
added, written into memory and reopened (a removal and an
addition on one part in one write are refused, "Errors"), then a
second `add_chart` whose plot kind comes from the probe's command
line as an index 0 to 12 over the thirteen `NewPlot` variants in
declaration order (the probe's constants give each variant only
the options its kind takes, ruling R: a `Line { Standard }`
second plot on a secondary axis with a title and a format on a
bar or area first plot, the `OutsideEnd` position where the kind
takes it, a colour where the kind takes one, the axis formats
where the kind takes them, the series count and `sizes` by the
kind; labels `value` true with a format on every kind), written
into memory and reopened as part 4's is, then `charts`. The index
in the size log's run is 0, `Bar { Column, Clustered }`, on which
every option applies: `charts 1` and `chart_series 3`. The kind
index from the command line keeps every plot arm live; the
removal, the combined pair, the labels, the colours and the
formats are linked by the call. The second output stays the
format probe's, byte for byte.

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
passing unchanged, the two part 4 samples and the three image
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
chart frame names a relationship with no part (refused by the
sample's own count with the reader's "a chart cannot be read",
`specs/0034`, with a control; ruling N), and the two new messages
stated unreachable through the binary with their covering tests.
The run log: the
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

## Planning rulings during P1 (2026-10-14)

The doc commit b60d4f5 verified by planning: the three files at
their hashes (a3d55ede, bd15b44f, fb66162a), nothing else, the
message ASCII with the Co-Authored-By line only, the tree clean.

A. The compaction (5d8ce24) is accepted as decision 1 A asks:
   every output byte unchanged, every part 4 test and figure
   unchanged, the helpers as "Size and packaging" names them. Its
   measured saving in development is 348 section bytes (1055368
   to 1055020, the implementer's dev builds, no primary; the gate
   prints the figure), not the 3000 to 8000 planning expected:
   planning's reading of the emission as 500 bytes per literal
   form was wrong, and the writer's 40732 sits mostly outside the
   emission's literals. The miss is planning's and changes
   nothing in P1: the compaction stands (smaller source, one axis
   function for the series axis to join), the kinds follow, the
   estimate "33000 to 46000 before the compaction" is the part's
   estimate as it stands and the finding line of 48000 is kept.
   Before P4's brief, planning attributes part 4's writer cost by
   symbol from its own unstripped size-profile build in its
   worktree and writes the attribution into this spec, so that
   P4's size record and part 6's measurement price what is
   actually there.
B. "A colour on a stock series refused" in "P1" is a spec error:
   `NewSeries::color` arrives in P3. The test moves to P3 beside
   the colour rules (a colour on a stock series refused with "a
   chart option does not apply to its kind"); P1's list reads
   without it. Adding `NewSeries::sizes` gives every existing
   `NewSeries` literal `sizes: None`: part 4's tests, the tool and
   the probe are edited for it in P1's kinds commit, the
   compaction commit's tests staying unchanged; accepted.
   `RadarStyle` is a `pub` enum of `chart_write.rs` re-exported
   from `emend::package` beside `NewPlot`, as the other chart
   enumerations are.
C. The doc commit b60d4f5's message wraps each file's hash onto
   the line after its path (a bullet form), not the one-line
   `<path> sha256 <hash>` form part 4's scan check (change GN) and
   gate check expect. The hashes are right (planning verified the
   three blobs). The checks are changed, not the commits: each
   reads the message with its line breaks folded (as change J3's
   diff-alone check does) and finds, for each file, its path
   followed by `sha256` and the blob's hash, with a control that
   accepts the folded form and one that refuses a wrong hash. The
   one-line form stays the rule for later doc commits.
D. "The four part 4 samples" in "P1" is a spec error for "the two
   part 4 samples" (`--add-chart-sample`, `--chart-kinds-sample`),
   as "Harness runs" and the brief name them: five reruns in all
   with the three image samples. The run followed "Harness runs";
   nothing is rerun. The sentence is corrected in place.

## Planning verification of P1 (2026-10-14)

Verified from primaries: the five logs at the record commit
e86561a (gate d423bd45, test 2ab6fe6d, negative e13d04db, run
00e490b3, scan 2fea399f PASS, 0 hits, 0 structural failures; each
of the four "all as expected: yes"); the record commit's 10 files
and independent-check-1.md (60a72304); the four records at the
report's hashes; b60d4f5 holding the three documents at their
hashes and nothing else; every commit message ASCII with the
Co-Authored-By line only; the privacy scan of d2c14d8..e86561a
(5 commits, 1755 blobs, 0 findings, PASS); the kinds commit's
emission read against "The chart part" (the doughnut's
`firstSliceAng` then `holeSize`, the radar's `radarStyle` before
`varyColors`, the bubble's `bubbleSize` after `yVal` with
`bubbleScale` and `showNegBubbles`, the stock's `hiLowLines` and
the `none` marker, the axes a line's or a scatter's); the gate's
probe figure 1055684 (316 over part 4's, told, ruling A); the
negative log's N3 (the two-drawing message through the binary,
ruling Q) and the three part 1 messages stated with their test;
planning's own build at ad48df3 in its detached worktree
(`<home>/.cache/emend-planning/m5p5-p1-build.txt`): the six
feature sets and the workspace on both targets at the log's
figures (515, 585, 551, 623, 688, 1062; 1279), clippy and doc
clean, release emend-copy on both targets (the i686 binary
4f779c5a equal to the log's), `--chart-more-sample` and the five
reruns over the corpus byte-identical to the six run directories'
outputs and lines on both targets, 27 of 27 each; planning's
more totals `charts_before` 6, `charts_after` 114,
`kinds_read_back` 108, `chart_parts_after` 179.

The disclosures. The gate's dry run 1 (the chart write test count
30 for 32, two tests also gated on `images`; change I59) and the
aborted relaunch, both kept outside the repository: accepted. The
negative labels N3/B1 for the brief's N1/B0: accepted, the brief's
labels were loose. One base test body edited by `sizes: None,`
alone: accepted (ruling B). The dev run over 26: accepted.

The attribution of part 4's writer cost (ruling A's promise),
from planning's own unstripped size-profile builds of the
`charts` probe on i686 at d74ea74 and 574a77d
(`<home>/.cache/emend-planning/m5p5-attrib.txt`, `nm -S`, names
only): `.text` grew 34576 and `.rodata` 3072 of the 40732
section bytes. By symbol, `Parts::write_to` grew by 17751: the
chart part's and the frame's emission is inlined into the write
(`part_bytes`, `plot_bytes`, `axes_bytes`, `frame_bytes` have no
symbol of their own); the drawing writer that a `charts`-only
consumer links for the first time is the next 10000 (`drawing_of`
2026, `anchor_bytes` 1257 and its closure 655, `NewDrawing::body`
1150, two `rewrite_worksheet_from` instances 2025,
`relative_target` 1106, `ElementPass::root_tag` 984,
`rels_prefix` 775), which the `images` probe already paid, so the
`images` figure did not move; the call's checks and helpers about
2000 (`series_bytes` 543, `value_axis_bytes` 280, `partial_escape`
322, `element_bytes`, `reference_bytes`, `text`, `reference`,
`valid`, `title_bytes`); the rest in drop glue, iterator
instances and the probe's `main`. So the emission's literals are
about half the writer's cost and the regated drawing writer a
quarter: part 4's "4000 to 7000" for the drawing additions was
the miss, and a table-driven emission would reach at most the
inlined half. No compaction beyond P1's; the estimate and the
48000 line stand (ruling A); part 6's measurement cites this
attribution.

Rulings:

E. No carry from P1 to P2 beyond rulings A to D; the "P2" list
   stands.

P1 is DONE. P2's doc commit carries this spec (rulings A to E and
this section).

## Planning rulings during P2 (2026-10-14)

The doc commit 4ef3a8c verified by planning: specs/0036 (c98cf950)
and ROADMAP.md (3eefa7ec) unedited, nothing else, the one-line
hash form, the tree clean.

F. A `Line3D` always writes the series axis and three `axId`s,
   whatever its grouping ("Facts": the schema requires three);
   "The chart part"'s bracket on `line3DChart`'s third `axId` is
   read as unconditional. `Bar3D` and `Area3D` keep the bracket
   (the series axis with `Standard` only).
G. A 3D first plot in `combined` is refused with "a chart option
   does not apply to its kind": Excel combines no 3D plot with
   another, and the series axis would collide with the secondary
   pair's ids. "Applicability"'s list of first plots reads `Bar`,
   `Line`, `Area` or `Scatter`; the test "a combined 3D second
   plot refused" gains its first-plot twin. The 3D sample's
   combined chart is a 2D bar and is unaffected.
H. "Of a different variant than the first's" is read literally
   over the 2D plots that remain: a `Bar` second on a `Bar` first
   is refused, a `Line` on a `Bar` accepted, and so on. The
   `combined: None` edits to every existing `NewChart` literal go
   in the crate commit, the edited base test bodies named as P1's
   gate named them (ruling B (a)).
I. The crate commit a3deead writes `serAx` inline in `axes_bytes`
   (two `value_element` calls and one literal) instead of through
   the one axis function "Size and packaging" requires for
   `catAx`, `valAx` and `serAx`; independent check 1 found it,
   the implementing session disclosed it before the record
   commit. Ruled: fixed by a rewrite of the local, unreported
   crate commit (specs/0001, amend before the report): the axis
   function gains the `serAx` form (no gridlines, title, number
   format or `crossBetween`, the `axPos` `b`, `crossAx` `20`,
   `crosses` `autoZero`), `axes_bytes` calls it, the bytes of
   "The chart part"'s `serAx` unchanged (plants P3 and P4 keep
   catching them). The same rewrite tightens the last case of
   `the_combined_rules_and_texts_follow_the_check_order` to
   separate the first plot's texts from the second's. The tool
   commit is re-based on it; the dry runs, the four logged runs,
   the check and the scan are redone at the new phase commit;
   the four logs and the four untracked records written at
   6bb8b26 (cited by nothing committed) are deleted first. No
   push is involved. The probe figure moves; told in P4 as the
   phase says.
J. A `Scatter` first plot in `combined` is refused in every case:
   with a non-scatter second by "Applicability"'s own sentence
   and with a scatter second by ruling H (the first's variant).
   The first-plot list is corrected in place to `Bar`, `Line` or
   `Area`; the code's check, which refuses a first plot other
   than those three, stands. The crate commit's message (in the
   rewrite) attributes the 3D refusal to ruling G and the scatter
   refusal to this ruling, not both to G; the report says the
   same. The more sample's rerun is the phase's ("the more and 3D
   samples"), not an addition.

## Planning verification of P2 (2026-10-15)

Verified from primaries: the five logs at the record commit
1bb8e02 (gate 10b64b4c, test 5a73456d, negative 91860cdb, run
19f0b68c, scan b27e2648 PASS, PLANNING_SPEC e91c254d the working
copy; each of the four "all as expected: yes"); the record
commit's 10 files and independent-check-1.md (bfe7907a); the four
records at the report's hashes (p2 a76cb89c and 0b218464, open
2bd9b252 and ceaf5e6a); every commit message ASCII with the
Co-Authored-By line only; the privacy scan of e86561a..1bb8e02
(4 commits, 1763 blobs, 0 findings, PASS); the crate commit
a5823b7 (the rewrite of a3deead under ruling I): `axis_bytes`
takes a three-way form with the series form, `axes_bytes` calls
it, the message attributes the 3D refusal to G and the scatter
refusal to J; the 3D sample's 135 added chart parts in the run
directory read against "The chart part" (27 of each of the five
kinds; `view3D` after `autoTitleDeleted` at 15/20/1 and the pie's
30/0/0 in all 108; the `serAx` byte for byte after the `valAx`
and the plot's third `axId` in the 27 standard 3D lines, none on
the clustered 3D bar, the stacked 3D area or the pie; the
combined bar and line with the secondary pair 30/40, the second
plot naming 30 and 40, `idx` 0 and 1, in all 27); the gate's
probe figure 1058244 (2876 over part 4's, 2560 over P1's, told,
ruling A); the test log's fourteen plants each caught; the
negative log's N4 (the 3D sample on the non-`wsDr` root refused
with the cannot-be-read message, control B2); the run log's
chartmore rerun byte-identical to P1's on 27 of 27 on each
target, the keep 1278 = 1224 + 54 with 0 changed and 0 gone, the
open check 1266/12 and 1267/11 with 0 findings and no verdict
moved; planning's own build at e576023 in its detached worktree
(`<home>/.cache/emend-planning/m5p5-p2-build.txt`): the six
feature sets 515/599/551/637/702/1076 and the workspace 1296 on
both targets, clippy and doc 0 lines, the i686 release
`emend-copy` equal by sha256 to the record's (67411db7), and the
chartmore and chart3d samples over the corpus on both targets
byte-identical to the phase's run directories, outputs and lines,
27 of 27 each (chart3d totals charts_before 6, charts_after 141,
kinds_read_back 135, chart_parts_after 206).

Rulings:

K. The comments check 1 noted stand until P3: `add_chart`'s doc
   comment ("P2 rulings G and H"), the inline comment at the
   first-plot check (ending "P2 ruling G") and the test module's
   doc comment ("P2 rulings F to H") are corrected to name
   ruling J for the scatter refusal and "F to J" in P3's crate
   commit, which edits both files anyway; no further rewrite.
   The report's four statements about the superseded round,
   labelled as having no primary: accepted. Check 1's note (c),
   the `combined: None` edits landing in the three helpers and
   no base test body: consistent with ruling H, accepted. The
   probe at 1058244: told, judged in P4 (ruling A).

## Planning rulings during P3 (2026-10-15)

The doc commit 67aaa4f verified by planning: specs/0036 (4f9bc8ff)
and ROADMAP.md (deab2d2f) unedited, nothing else, the one-line
hash form, the tree clean.

L. "Applicability"'s labels bullet named "the 3D plots" among the
   plots refusing `labels.position` and then gave `Bar3D` and
   `Pie3D` positions; the implementing session asked. Ruled as it
   recommended: a position is refused on `Bar3D`, `Line3D` and
   `Area3D`, and `Pie3D` takes the pie's four (Excel offers no
   label position on a 3D column, line or area chart and the
   pie's on a 3D pie). The bullet is corrected in place. The
   labels sample (a 2D bar and a 2D pie) is unaffected. Two
   readings accepted with it: the stock series' `ln` form in "The
   chart part" is unreachable under ruling B and is written for
   the plots listed, the refusal kept; a combined second plot's
   series take labels and colours by the second plot's own kind
   rules ("each checked as the first's").

M. "Harness runs" expected the remove-chart sample's `kept_removed`
   0 output to be a byte copy of its input. It cannot be, for the
   reason `specs/0028` ruling F1 already recorded for the image
   removal sample: the second write deflates the entries the
   first touched (`recompressed`), a rels part the addition
   created stays, empty (`added`), and a drawing that held no
   anchor before the write goes with its last (`dropped`). The
   implementing session found it in a dev run and asked; the
   expectation was planning's error, repeated. Ruled as it
   recommended: the expectation is corrected in place to the
   image removal sample's pattern (`records/m4p1-p3-removeimage-
   2026-10-10-669fa91.md`); the tool asserts the uncompressed
   bytes, and the run log checks the harness statuses against
   the cases named. No change to the removal layer.

N. The P3 negative case cannot reach "a chart part cannot be
   read" through the binary: the remove-chart sample prints
   `charts_before` and `charts_other_sheets`, so `charts` reads
   every sheet's frames before any removal and refuses the bad
   frame with the reader's "a chart cannot be read" (`specs/0034`).
   The implementing session asked; ruled as it recommended: N5
   expects the reader's message, with a control (the same fixture
   with the part in place, exit 0); "no chart at that position"
   and "a chart part cannot be read" are stated unreachable
   through the binary with their covering tests
   (`an_index_past_the_list_named_twice_or_with_an_addition_is_refused`
   and `a_frame_naming_no_chart_part_is_refused_at_the_write`).
   The sample is not changed. The phase text is corrected in
   place.

## Planning verification of P3 (2026-10-15)

Verified from primaries: the five logs at the record commit
17cb3c6 (gate 1f07861f, test 39bc06b2, negative 2e11c1d2, run
eea06a2f, scan df985d4a PASS, PLANNING_SPEC 6ecaf055 the working
copy; each of the four "all as expected: yes"); the record
commit's 12 files and independent-check-1.md (c5c6e9e2); the six
records at the report's hashes (chartlabels b4d50241 and
90ccabda, removechart e5459272 and fc3e318a, open a0825862 and
2144b942); every commit message ASCII with the Co-Authored-By
line only; the privacy scan of 1bb8e02..17cb3c6 (4 commits, 1777
blobs, 0 findings, PASS); the crate commit 784e3c0 read against
"The chart part": the series `spPr` after `tx` with the `ln` copy
on a line, 3D line, scatter, radar or stock plot, the colour as
six upper-case hex digits, `dLbls` before the references in the
spec's child order with the nine `dLblPos` values, the axis
`numFmt` with `sourceLinked` 0 through one helper and none on the
series axis, every format code through the text check and
quick-xml's `escape` (a quoted format is written as `&quot;`,
tested); the removal worklist in `package.rs`: each entry no kept
relationship reaches goes with its `Override`, then its rels part
and what that part alone reaches, external targets left out
(decision 3 A); ruling K's three comments corrected; the labels
sample's 54 added chart parts in the run directory read against
"The chart part" (in all 27 workbooks one bar with the two
colours, the value labels outside the end in the format, both
axis formats and no `General`, and one pie with its colour, the
category and percentage labels at the best fit and no `numFmt`);
the remove-chart run directory's 27 host rows against ruling M
and the removal rule (21 with nothing removed and nothing dropped,
only identical, recompressed or a rels part added; 2 with W's
empty drawing dropped and W, its rels and the content types
changed; 4 with a kept chart removed, the chart part, its rels and
the style and colours parts it alone reached dropped, counted in
`parts_dropped`); the gate's probe figure 1071980 (16612 over part
4's, 13736 over P2's, told, ruling A); the test log's fourteen
plants each caught; the negative log's N5 (the remove-chart
sample on a frame naming a relationship with no part refused with
the reader's "a chart cannot be read", control B3 exit 0) and the
two new messages stated unreachable with their covering tests
(ruling N); the run log's keep 1435 = 1278 + 157 with 0 changed
and 0 gone, the open check 1423/12 and 1424/11 with 0 findings
and no verdict moved; planning's own build at 4836d14 in its
detached worktree (`<home>/.cache/emend-planning/m5p5-p3-build
.txt`): the six feature sets 515/617/551/656/720/1095 and the
workspace 1319 on both targets, clippy and doc 0 lines, the i686
release `emend-copy` equal by sha256 to the record's (43d5e04f),
and the chartlabels and removechart samples over the corpus on
both targets byte-identical to the phase's run directories,
outputs and lines, 27 of 27 each (chartlabels totals
charts_before 6, charts_after 60, read_back 54, chart_parts_after
125; removechart totals
charts_before 6, charts_after 2, kept_removed 4, parts_dropped 12,
drawing_dropped 2, anchors 26 to 22, charts_other_sheets 20).

Rulings:

O. Check 1's notes (a) to (c) are documentation gaps the code does
   not share; carried into P4's first code commit (the probe
   commit), as ruling K carried P2's: (a) the header's Limits
   bullet "Texts and references" names a format code and a label
   format among the texts at 32767 bytes, checked as a text and
   nothing more, and the colour's form (a `u32` at most
   `0xFFFFFF`, six upper-case hex digits); (c) the doc comments on
   `category_axis_format` and `value_axis_format` name `Pie3D`
   among the kinds that take none. (b) The code and the catalogue
   refuse "no chart at that position" for a part the same write
   adds an image or a chart to, the image path's rule; "The call"
   read "an image" and "Errors" read "a chart", each half: both
   are corrected in place to "an image or a chart". P4's first
   code commit adds one test under `charts,images`, a chart
   removal on a part the same write adds an image to, refused;
   P4's test figures count it. No rewrite of P3.
P. Accepted as disclosed: the lint rewrite of 250caaa and cfb83cc
   into 784e3c0 and 4836d14 before any logged run (`specs/0001`,
   unreported local commits); the catalogue's two message entries
   in the crate commit for the catalogue test, the rest in the
   tool commit; the one base test body gaining `color: None` and
   `labels: None` (ruling B's pattern); the dry-run failures and
   the dev runs, labelled; the sample records named
   `m5p5-p3-<sample>-<utc-date>-<shortsha>`, the only reading of
   "for each"; the check's four unlabelled statements, now
   labelled. The probe at 1071980: told, judged in P4 (ruling A;
   16612 of the part's writer cost so far, under the 33000 to
   46000 estimate before the probe's own additions link the rest).
Q. The self-closing `<xdr:wsDr/>` root refused on an append
   (disclosure 7) is part 4's drawing writer as it stands, no
   corpus producer writes it, and nothing in this part changes
   it; noted for part 6's engineering close as a candidate (accept
   the form or name it in the catalogue), not a finding of P3.

## Planning rulings during P4 (2026-10-15)

The doc commit 0bd800d verified by planning: specs/0036 (157f785d)
and ROADMAP.md (b582c49a) unedited, nothing else, the one-line
hash form, the tree clean.

R. "The probe" named index 9 (`Bar3D`) for the size log's run with
   a combined second plot and `OutsideEnd` labels, which rulings G
   and L refuse, and sent the same options to every index, which
   most kinds refuse by their own rules; and it named one write
   for the removal and the second addition on one part, which
   "Errors" refuses. Planning's errors; the implementing session
   asked. Ruled as it proposed (its A): the probe's per-variant
   constants give each kind only the options it takes (the
   combined `Line` second plot on bar and area first plots, the
   position where the kind takes `OutsideEnd`, colours, formats,
   series count and `sizes` by the kind's rules, a stock's three
   series without a colour, a pie's one), so every index writes
   and reads back; the removal is its own in-memory write before
   the second addition; the size log's run uses index 0, `Bar {
   Column, Clustered }`, where every option applies and `charts
   1`, `chart_series 3` hold. The linked code is the same at any
   index. The paragraph is corrected in place.

S. "Size and packaging" bounds the `images` feature's movement
   "by the predicate the removal shares (expected 0, accepted up
   to 2000)": a bound on growth, what a consumer linking `images`
   pays, as every size line of this project is. The implementing
   session found the `images` probe 11152 section bytes SMALLER
   than part 4's 1038424 at the phase (1027272, first seen in P4
   because P3's gate printed the `charts` probe alone) and asked;
   its own three-commit measurement places the whole shrink in
   P3's crate commit 784e3c0, where `record_image_removals` and
   `image_removals` became the shared `record_removals` and
   `drawing_removals`, with the images probe's lines and outputs
   byte-identical across it and the `images` tests unchanged.
   Ruled as it recommended (its reading 1): a shrink is not a
   finding; the size log prints the movement with its sign,
   judges it "shrank 11152, accepted" against the growth bound,
   and the size record attributes it to 784e3c0 with the
   measurement at 1bb8e02 and 4836d14 (figures and sha256s).
   Planning reproduces the three figures from its own probe
   builds before the row is marked DONE, as it did in part 4 P3.

## Planning verification of P4 (2026-10-15)

Verified from primaries: the six logs at the record commit 6cee4f8
(gate b6138cb8, test b8565e6f, negative bc488c73, size a4c72773,
run d3890e76, scan 556c1bb5 PASS, PLANNING_SPEC 27a100ab the
working copy; each of the five "all as expected: yes"); the
record commit's 21 files and independent-check-1.md (113abb75);
the fourteen records at the report's hashes (the size record
fa7b6452, the API record 5e0a6b60, the five sample pairs, the
open pair 4d4ee8bc and 2144b942, the latter byte-identical to
P3's); every commit message ASCII with the Co-Authored-By line
only; the privacy scan of 17cb3c6..6cee4f8 (4 commits, 1793
blobs, 0 findings, PASS); the probe commit 17eaa26 read against
"The probe" and ruling R (the four steps each written into memory
and reopened; the thirteen `NewPlot` variants in declaration
order; the per-kind options: the combined line on a bar or area
first plot, `OutsideEnd` on a clustered bar, a pie or a 3D pie, a
colour on every series but a stock's, one series on a pie or 3D
pie and three on a stock, `sizes` on a bubble, no axis text on a
pie, 3D pie or doughnut, no category format on a scatter or
bubble, value labels in a format on every kind) and ruling O's
carry (the Limits bullet, the two doc comments, the test under
`images` named in the catalogue); the CHANGELOG's one bullet
citing this spec; the API record's one gone row (the old
re-export line, replaced by the widened one) and 23 new rows,
every one an item of this part; the size log's figures (core
970208, growth 0; charts 1075632, the feature 105424, the
writer's cost 20264 over 85160, under 48000 and under the 25000
to 43000 estimate after the compaction; images 1027272, the
feature 57064, 11152 under part 4's, accepted under ruling S and
measured across P3's crate commit at 1bb8e02 and 4836d14; the
budget 436992, 21760 under the ceiling, unchanged); the test
log's fifteen plants each caught and the E figures P3's plus the
one test under `charts,images` and all; the negative log's 128
constants and the new test named for "no chart at that
position"; the run log's five samples byte-identical to their
phase runs on both targets, 27 of 27 each, the keep 1435 with 0
new and 0 changed, the open check 1423/12 and 1424/11 with 0
findings and no verdict moved; planning's own build at a8bfe71
in its detached worktree
(`<home>/.cache/emend-planning/m5p5-p4-build.txt`): the six
feature sets 515/617/551/657/720/1096 and the workspace 1320 on
both targets, clippy and doc 0 lines, the three size-profile
probes on i686 equal by sha256 to the size log's (format
9f02b70a, charts b3a617e2, images 1d39ed45) and the images probe
rebuilt at 1bb8e02 and 4836d14 equal by sha256 to the size log's
(29c9d2d1 at 1038424, 255fce29 at 1027272), and the five samples
over the corpus on both targets byte-identical to the phase's run
directories, outputs and lines, 27 of 27 each; and
planning's own run of its charts probe binary at every kind index
on the size log's generated fixtures: indexes 0 to 12 each exit 0
with thirty lines and `charts 1`, `chart_series` 3, 2, 1, 2, 3,
2, 2, 2, 3, 2, 2, 1, 2 in index order, index 13 refused with exit
2 and the one error line, which covers check 1's note that
indexes 1 to 12 rested on the implementer's dev runs.

Rulings:

T. Check 1's additional note: the probe commit's message says no
   log prints a count for it, while the size log prints the
   probe's `charts 1` and `chart_series 3`. Those lines are the
   probe's output, not a count of what the commit changes; the
   gate's and the scan's count rule accepted the message, and so
   does planning. The check's three disagreements were met in
   the report's wording before it was sent (the `PLOTS` array
   with per-kind derivation, the two statements labelled, the
   per-index figures labelled as dev runs); planning's own run
   above is the primary for every index. The dry-run failures,
   the rustfmt of the probe's additions and the dev measurements
   are accepted as disclosed. P4 is DONE; the part's code is
   complete. The close-out follows as "Close-out push (after P4)"
   says, on planning's word, given with the close-out brief, and
   the maintainer's authorisation typed in the implementing
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
