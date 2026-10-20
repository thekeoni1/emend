//! A chart written into a worksheet's drawing from cell ranges
//! (`specs/0035-charts-write-first-set.md`). The Cargo feature `charts`.
//!
//! The second set (`specs/0036-charts-write-second-set.md`) adds the
//! doughnut, radar, bubble and stock plots, the 3D plots, a second plot
//! combined with the first, data labels, a solid colour per series and the
//! axes' number formats, and [`Package::remove_chart`].
//!
//! `specs/0039-chart-kinds-and-formatting.md` adds the surface plots, the
//! contour form and the 3D one, and the of-pie plot with its split; and a
//! series' marker, its line's width and dash, and its formatted points.
//!
//! [`Package::add_chart`] checks the call and records the chart as an image
//! is recorded: the write adds the chart part, created whole from the
//! caller's strings, and an anchor holding a `graphicFrame` that names it,
//! through the drawing writer the feature `images` shares (P1 ruling A).
//! References are written as the caller gives them, escaped, never parsed
//! or quoted (decision 2), and no cache is written (decision 3).

use std::io::{Read, Seek};

use quick_xml::escape::{escape, partial_escape};

use super::chart::CHART_NAMESPACE;
use super::{
    BarDirection, DRAWINGML, DrawingForm, EMU_PER_PIXEL, Grouping, INDEX_OUT_OF_RANGE, ImageAnchor,
    LegendPosition, NewDrawing, Package, Parts, Removal, ScatterStyle, drawing_a, drawing_r,
    in_grid,
};
use crate::archive::Error;

/// What a chart text that is empty, holds a character below U+0020 or a
/// noncharacter U+FFFE or U+FFFF, or is longer than its limit is refused
/// with (P1 ruling F); the `package` module's catalogue lists it with the
/// other three (P1 verification ruling G).
pub(crate) const CHART_TEXT_INVALID: Error =
    Error::Package("a chart text is empty or holds a control character");
/// What a chart option its plot's kind does not take is refused with.
pub(crate) const CHART_OPTION_KIND: Error =
    Error::Package("a chart option does not apply to its kind");
/// What a chart size its anchor does not take, or a missing one it needs,
/// is refused with.
pub(crate) const CHART_SIZE_ANCHOR: Error =
    Error::Package("a chart size does not apply to its anchor");
/// What a chart anchor or size out of range is refused with.
pub(crate) const CHART_ANCHOR_OUT_OF_RANGE: Error =
    Error::Package("a chart anchor is out of range");
/// What a chart removal whose index is past the list, named twice, or on a
/// part this write also adds to is refused with
/// (`specs/0036-charts-write-second-set.md`, "Errors").
pub(crate) const NO_CHART_AT: Error = Error::Package("no chart at that position");
/// What a chart removal whose frame names no chart part, or a part that is
/// not a `chartSpace`, is refused with at the write.
pub(crate) const CHART_PART_UNREADABLE: Error = Error::Package("a chart part cannot be read");
/// What the chart calls refuse a part whose root is neither a worksheet
/// nor a chartsheet with (`specs/0038-chartsheets.md`, "Errors").
pub(crate) const NOT_SHEET_ROOT: Error =
    Error::Package("the sheet part's root is not worksheet or chartsheet");
/// What a chart on a chartsheet in another than an absolute anchor is
/// refused with at the write (decision 1 A).
pub(crate) const CHARTSHEET_ANCHOR: Error =
    Error::Package("a chartsheet's chart takes an absolute anchor");
/// What a second chart on a chartsheet is refused with at the write
/// (decision 2 A, P1 ruling A).
pub(crate) const ONE_CHART: Error = Error::Package("a chartsheet holds one chart");
/// What a chart number outside its range is refused with
/// (`specs/0039-chart-kinds-and-formatting.md`, "Errors"): an of-pie's
/// position below 1, a split value that is not finite or is negative, a
/// percent above 100.
pub(crate) const CHART_VALUE_RANGE: Error = Error::Package("a chart value is out of its range");

/// A chart part's content type (`specs/0035-charts-write-first-set.md`,
/// "The parts").
pub(crate) const CHART_CONTENT_TYPE: &[u8] =
    b"application/vnd.openxmlformats-officedocument.drawingml.chart+xml";

/// The longest title, axis title or text name, in bytes.
const TEXT_LIMIT: usize = 32_767;
/// The longest reference, in bytes.
const REFERENCE_LIMIT: usize = 8_192;
/// The widest line, in EMU (`specs/0039-chart-kinds-and-formatting.md`,
/// "Applicability").
const LINE_WIDTH_LIMIT: u32 = 20_116_800;

/// A chart as a caller gives it to [`Package::add_chart`]
/// (`specs/0035-charts-write-first-set.md`, "The call"). The Cargo feature
/// `charts`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NewChart<'a> {
    /// The plot: its kind and the options the kind takes.
    pub plot: NewPlot,
    /// The chart's title; `None` writes `autoTitleDeleted` 1.
    pub title: Option<&'a str>,
    /// The series, one or more, in order.
    pub series: &'a [NewSeries<'a>],
    /// The category axis's title, or a scatter's x axis's; a pie takes
    /// none.
    pub category_axis_title: Option<&'a str>,
    /// The value axis's title, or a scatter's y axis's; a pie takes none.
    pub value_axis_title: Option<&'a str>,
    /// The category axis's number format code; a pie, a 3D pie, a
    /// doughnut, a scatter and a bubble take none.
    pub category_axis_format: Option<&'a str>,
    /// The value axis's number format code, or a scatter's or a bubble's y
    /// axis's; a pie, a 3D pie and a doughnut take none.
    pub value_axis_format: Option<&'a str>,
    /// The legend's position; `None` writes no legend.
    pub legend: Option<LegendPosition>,
    /// The chart's pixel width and height: required for a one-cell or
    /// absolute anchor, refused for a two-cell anchor.
    pub size: Option<(u32, u32)>,
    /// A second plot on the same plot area, after the first.
    pub combined: Option<NewCombined<'a>>,
}

/// A second plot combined with a chart's first (`specs/0036-charts-write-
/// second-set.md`, "The call"): a bar, line, area or scatter plot of a
/// variant other than the first's, on the primary axes or on a secondary
/// pair. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NewCombined<'a> {
    /// The second plot: its kind and the options the kind takes.
    pub plot: NewPlot,
    /// Its series, one or more, in order, after the first plot's.
    pub series: &'a [NewSeries<'a>],
    /// Whether it takes the secondary axes, ids 30 and 40.
    pub secondary_axis: bool,
    /// The secondary value axis's title; requires `secondary_axis`.
    pub secondary_axis_title: Option<&'a str>,
    /// The secondary value axis's number format code; requires
    /// `secondary_axis`.
    pub secondary_axis_format: Option<&'a str>,
}

/// A new chart's plot: its kind and the options the kind takes. The Cargo
/// feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NewPlot {
    /// `barChart`: its direction and its grouping, `Clustered`, `Stacked`
    /// or `PercentStacked`.
    Bar {
        /// Bars or columns.
        direction: BarDirection,
        /// How the series are grouped.
        grouping: Grouping,
    },
    /// `lineChart`: its grouping, `Standard`, `Stacked` or
    /// `PercentStacked`.
    Line {
        /// How the series are grouped.
        grouping: Grouping,
    },
    /// `pieChart`: one series, no axes, `varyColors` 1.
    Pie,
    /// `scatterChart`: its style; two value axes.
    Scatter {
        /// The `scatterStyle`, any of the six.
        style: ScatterStyle,
    },
    /// `areaChart`: its grouping, `Standard`, `Stacked` or
    /// `PercentStacked`.
    Area {
        /// How the series are grouped.
        grouping: Grouping,
    },
    /// `doughnutChart`: its hole size, 1 to 90 percent; no axes,
    /// `varyColors` 1.
    Doughnut {
        /// The `holeSize`, in percent of the chart's radius.
        hole_size: u8,
    },
    /// `radarChart`: its style; the axes a line's.
    Radar {
        /// The `radarStyle`.
        style: RadarStyle,
    },
    /// `bubbleChart`: every series with its sizes; two value axes as a
    /// scatter's.
    Bubble,
    /// `stockChart`: three or four series, high, low and close or open,
    /// high, low and close, in that order; the high-low lines; the axes a
    /// line's.
    Stock,
    /// `bar3DChart`: its direction and any grouping; `Standard` adds the
    /// series axis.
    Bar3D {
        /// Bars or columns.
        direction: BarDirection,
        /// How the series are grouped.
        grouping: Grouping,
    },
    /// `line3DChart`: its grouping, `Standard`, `Stacked` or
    /// `PercentStacked`; the series axis whatever the grouping (P2 ruling
    /// F).
    Line3D {
        /// How the series are grouped.
        grouping: Grouping,
    },
    /// `pie3DChart`: as `Pie`.
    Pie3D,
    /// `area3DChart`: its grouping, `Standard`, `Stacked` or
    /// `PercentStacked`; `Standard` adds the series axis.
    Area3D {
        /// How the series are grouped.
        grouping: Grouping,
    },
    /// `surfaceChart`, the contour form seen from above: one or more
    /// series, which take no colour and no labels; the axes a line's and
    /// the series axis (`specs/0039-chart-kinds-and-formatting.md`).
    Surface {
        /// Whether the surface is drawn as a wireframe, `wireframe`.
        wireframe: bool,
    },
    /// `surface3DChart`: as `Surface`, drawn in 3D.
    Surface3D {
        /// Whether the surface is drawn as a wireframe, `wireframe`.
        wireframe: bool,
    },
    /// `ofPieChart`: one series, no axes, `varyColors` 1, as `Pie`; its
    /// second plot a pie or a bar.
    OfPie {
        /// The second plot's form, `ofPieType`.
        kind: OfPieKind,
        /// Which points go to the second plot; `None` writes `splitType`
        /// `auto`.
        split: Option<OfPieSplit>,
    },
}

/// An of-pie plot's second plot, `ofPieType`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OfPieKind {
    /// `pie`: pie of pie.
    Pie,
    /// `bar`: bar of pie.
    Bar,
}

/// An of-pie plot's split, `splitType` with `splitPos`
/// (`specs/0039-chart-kinds-and-formatting.md`, "The call"; the custom
/// split is out of scope, decision 2 A). The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OfPieSplit {
    /// `pos`: the last N points go to the second plot; N at least 1.
    Position(u32),
    /// `val`: the points below the value; finite and not negative.
    Value(f64),
    /// `percent`: the points below the percentage; finite, 0 to 100.
    Percent(f64),
}

/// A new radar plot's `radarStyle`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadarStyle {
    /// `standard`.
    Standard,
    /// `marker`.
    Marker,
    /// `filled`.
    Filled,
}

/// A new chart's series: its name and its references, each written as the
/// file's `f` text, as given (decision 2). The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NewSeries<'a> {
    /// The series' name, `tx`.
    pub name: Option<NewSeriesName<'a>>,
    /// The categories, `cat`, or a scatter's or bubble's x values, `xVal`.
    pub categories: Option<&'a str>,
    /// The values, `val`, or a scatter's or bubble's y values, `yVal`.
    pub values: &'a str,
    /// A bubble's sizes, `bubbleSize`: required on a bubble plot's series,
    /// refused on any other plot's.
    pub sizes: Option<&'a str>,
    /// The series' solid colour as `0xRRGGBB`, at most `0xFFFFFF`; a stock
    /// plot's series take none.
    pub color: Option<u32>,
    /// The series' data labels.
    pub labels: Option<NewLabels<'a>>,
    /// The series' marker: a line's, a scatter's, or a radar's of the
    /// standard or marker style (`specs/0039-chart-kinds-and-formatting.md`,
    /// "Applicability").
    pub marker: Option<NewMarker>,
    /// The series' line, its width and dash: a line's, a 3D line's, a
    /// scatter's, a radar's or a stock plot's.
    pub line: Option<NewLine>,
    /// The series' formatted points, `dPt`, in order; a surface's series
    /// take none.
    pub points: &'a [NewPoint],
}

/// A series' or a point's marker, `marker`: its symbol, its size and its
/// solid fill (`specs/0039-chart-kinds-and-formatting.md`, "The call"). The
/// Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NewMarker {
    /// The marker's `symbol`.
    pub symbol: MarkerSymbol,
    /// The marker's `size`, 2 to 72; `MarkerSymbol::None` takes none.
    pub size: Option<u8>,
    /// The marker's solid colour as `0xRRGGBB`, at most `0xFFFFFF`;
    /// `MarkerSymbol::None` takes none.
    pub color: Option<u32>,
}

/// A marker's `symbol`; `picture` is not one (it needs a `blip`). The Cargo
/// feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerSymbol {
    /// `none`: no marker drawn.
    None,
    /// `auto`.
    Auto,
    /// `circle`.
    Circle,
    /// `dash`.
    Dash,
    /// `diamond`.
    Diamond,
    /// `dot`.
    Dot,
    /// `plus`.
    Plus,
    /// `square`.
    Square,
    /// `star`.
    Star,
    /// `triangle`.
    Triangle,
    /// `x`.
    X,
}

/// A series' line, the series `spPr`'s `a:ln`: its width and its dash, one
/// of them at least (`specs/0039-chart-kinds-and-formatting.md`, "The
/// call"). The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NewLine {
    /// The line's width `w` in EMU (12700 per point; decision 1 A), at most
    /// 20116800.
    pub width: Option<u32>,
    /// The line's dash, `prstDash`.
    pub dash: Option<DashStyle>,
}

/// A line's preset dash, `prstDash`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DashStyle {
    /// `solid`.
    Solid,
    /// `dot`.
    Dot,
    /// `dash`.
    Dash,
    /// `lgDash`.
    LargeDash,
    /// `dashDot`.
    DashDot,
    /// `lgDashDot`.
    LargeDashDot,
    /// `lgDashDotDot`.
    LargeDashDotDot,
    /// `sysDash`.
    SystemDash,
    /// `sysDot`.
    SystemDot,
    /// `sysDashDot`.
    SystemDashDot,
    /// `sysDashDotDot`.
    SystemDashDotDot,
}

/// One formatted point of a series, `dPt`: its index and one or more of a
/// colour, an explosion and a marker (`specs/0039-chart-kinds-and-
/// formatting.md`, "The call", decision 4 A). The index is not checked
/// against the values. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NewPoint {
    /// The point's index, `idx`, counted from 0; two points of one series
    /// take two indexes.
    pub index: u32,
    /// The point's solid colour as `0xRRGGBB`, at most `0xFFFFFF`.
    pub color: Option<u32>,
    /// The point's `explosion` in percent: a pie's, a 3D pie's, a
    /// doughnut's or an of-pie's alone.
    pub explosion: Option<u32>,
    /// The point's marker, where the series takes one.
    pub marker: Option<NewMarker>,
}

/// A new series' data labels, `dLbls`: what each label shows, its position
/// and its number format code. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NewLabels<'a> {
    /// Whether a label shows the value.
    pub value: bool,
    /// Whether a label shows the category.
    pub category: bool,
    /// Whether a label shows the series' name.
    pub series_name: bool,
    /// Whether a label shows the percentage; a pie, a 3D pie and a
    /// doughnut alone take it.
    pub percent: bool,
    /// Where the labels sit; `None` leaves them at the plot's default.
    pub position: Option<LabelPosition>,
    /// The labels' number format code.
    pub number_format: Option<&'a str>,
}

/// Where a series' data labels sit, `dLblPos`; each plot takes some
/// (`specs/0036-charts-write-second-set.md`, "Applicability"). The Cargo
/// feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelPosition {
    /// `bestFit`.
    BestFit,
    /// `ctr`.
    Center,
    /// `inEnd`.
    InsideEnd,
    /// `inBase`.
    InsideBase,
    /// `outEnd`.
    OutsideEnd,
    /// `l`.
    Left,
    /// `r`.
    Right,
    /// `t`.
    Above,
    /// `b`.
    Below,
}

/// A new series' name: a reference written as `tx/strRef/f`, or a text
/// written as `tx/v`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NewSeriesName<'a> {
    /// A reference, the file's `f` text.
    Reference(&'a str),
    /// A text.
    Text(&'a str),
}

/// A series as the call checked it.
#[derive(Clone)]
struct SeriesWork {
    name: Option<NameWork>,
    categories: Option<String>,
    values: String,
    sizes: Option<String>,
    color: Option<u32>,
    labels: Option<LabelsWork>,
    marker: Option<NewMarker>,
    line: Option<NewLine>,
    points: Vec<NewPoint>,
}

/// A series' labels as the call checked them.
#[derive(Clone)]
struct LabelsWork {
    flags: [bool; 4],
    position: Option<LabelPosition>,
    format: Option<String>,
}

/// A series' name as the call checked it.
#[derive(Clone)]
enum NameWork {
    Reference(String),
    Text(String),
}

/// A chart as the call checked it: what the write builds its part and its
/// frame from, and its extent in EMU (both 0 for a two-cell anchor).
#[derive(Clone)]
pub(super) struct ChartWork {
    plot: NewPlot,
    title: Option<String>,
    series: Vec<SeriesWork>,
    category_axis_title: Option<String>,
    value_axis_title: Option<String>,
    category_axis_format: Option<String>,
    value_axis_format: Option<String>,
    legend: Option<LegendPosition>,
    combined: Option<CombinedWork>,
    pub(super) cx: u64,
    pub(super) cy: u64,
}

/// A combined second plot as the call checked it.
#[derive(Clone)]
struct CombinedWork {
    plot: NewPlot,
    series: Vec<SeriesWork>,
    secondary_axis: bool,
    title: Option<String>,
    format: Option<String>,
}

impl<R: Read + Seek> Package<R> {
    /// Records `chart` on the worksheet part `part`, placed by `anchor`
    /// (`specs/0035-charts-write-first-set.md`, "The call"). The Cargo
    /// feature `charts`.
    ///
    /// The write adds the chart part `xl/charts/chart<N>.xml`, N the
    /// smallest positive integer whose name no entry of the archive or of
    /// this write's additions takes, ASCII case folded, an entry this write
    /// removes still counting, with its `Override`; and appends an anchor
    /// holding a `graphicFrame` that names it through a relationship of type
    /// [`CHART`](super::CHART) on the drawing's rels part, as
    /// [`Package::add_image_at`](super::Package) appends a picture, creating
    /// the sheet's drawing part as it does. An image and a chart recorded on
    /// one sheet take the drawing in record order. Titles, axis titles and
    /// text names are written as one run's text, escaped; a reference is
    /// written as the `f` text, `&`, `<` and `>` escaped, neither parsed nor
    /// quoted; no cache is written. [`Package::charts`] reads the chart
    /// back as given.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded, the first of these
    /// in this order (P1 ruling C): "a row or column index or count is out
    /// of range" for a cell of the anchor off the grid; "a chart anchor is
    /// out of range" for an offset or position past `i32::MAX` EMU, or a
    /// two-cell anchor whose `to` is before its `from` in row or column; "a
    /// chart size does not apply to its anchor" for a size on a two-cell
    /// anchor or none on another; "a chart anchor is out of range" for a
    /// width or height of 0 or past `i32::MAX` EMU; "a chart option does not
    /// apply to its kind" for no series, a bar's `Standard` grouping, a
    /// line's, an area's, a 3D line's or a 3D area's `Clustered`, a pie or
    /// a 3D pie with more than one series or an axis title or format, a
    /// doughnut whose hole size is outside 1 to 90 or with an axis title or
    /// format, a stock plot with other than three or four series, a bubble
    /// plot's series without sizes or another plot's with them, a category
    /// axis format on a scatter or a bubble, a colour above `0xFFFFFF` or on
    /// a stock plot's series, a label's percentage on a plot other than a
    /// pie, a 3D pie, a doughnut or an of-pie, a label position the plot
    /// does not take, a colour or labels on a surface's series, an of-pie
    /// with more than one series or an axis title or format; "a chart value
    /// is out of its range" for an of-pie's split position below 1, a split
    /// value that is not finite or is negative, or a percent above 100
    /// (`specs/0039-chart-kinds-and-formatting.md`, "Applicability"); then
    /// each series' marker, line and points in field order, "a chart option
    /// does not apply to its kind" for a marker on a plot other than a
    /// line, a scatter or a radar of the standard or marker style, a
    /// `MarkerSymbol::None` with a size or a colour, a line on a plot other
    /// than a line, a 3D line, a scatter, a radar or a stock plot or with
    /// neither field, a point on a surface's series, with no field but its
    /// index or of an index another point of the series takes, a point's
    /// explosion on a plot other than a pie, a 3D pie, a doughnut or an
    /// of-pie, a point's marker where the series takes none, and a marker's
    /// or a point's colour above `0xFFFFFF`, and "a chart value is out of
    /// its range" for a marker size outside 2 to 72 or a line width above
    /// 20116800; then "a chart option does not apply to its kind" for a
    /// combined plot a first plot other than a bar, line or area, a second
    /// other than a bar, line, area or scatter or of the first's
    /// variant, a second with no series or that breaks its own kind's rules,
    /// a scatter second or a secondary title or format without
    /// `secondary_axis` (`specs/0036-charts-write-second-set.md`,
    /// "Applicability", P2 rulings G, H and J, P3 ruling L), then its
    /// series' marker, line and points by its kind as the first plot's; "a
    /// chart text is empty or holds a control character" for a title, axis
    /// title, axis format, secondary axis title, secondary axis format,
    /// name, reference (values, then sizes) or label format, in that order,
    /// series by series, the first plot's then the second's, that is empty,
    /// holds a character below U+0020, U+FFFE or U+FFFF, or is longer than
    /// 32767 bytes (a reference, 8192). The write refuses what
    /// [`Package::add_image`](super::Package) says it refuses of the part, a
    /// part this write also removes an image from, "no image at that
    /// position" (P1 ruling B), and one it removes a chart from, "no chart at
    /// that position".
    ///
    /// `part` may be a chartsheet's (`specs/0038-chartsheets.md`, "`add_chart`
    /// and `remove_chart` on a chartsheet"): the write then refuses an anchor
    /// other than [`ImageAnchor::Absolute`], "a chartsheet's chart takes an
    /// absolute anchor"; a chart beside a chart frame its drawing holds, or
    /// a second added in the same write, "a chartsheet holds one chart"
    /// (replacing a chart takes two writes, `remove_chart` then this call:
    /// P1 ruling A); a chartsheet without a `drawing` element, "a drawing
    /// part or its relationship cannot be read"; and a part whose root is
    /// neither a worksheet nor a chartsheet, "the sheet part's root is not
    /// worksheet or chartsheet".
    pub fn add_chart(
        &mut self,
        part: &[u8],
        anchor: ImageAnchor,
        chart: &NewChart<'_>,
    ) -> Result<(), Error> {
        self.parts().add_chart(part, anchor, chart)
    }

    /// Records the removal of the chart [`Package::charts`] lists at `index`
    /// for the worksheet part `part` (`specs/0036-charts-write-second-
    /// set.md`, "The call", "The parts"). The Cargo feature `charts`.
    ///
    /// The write skips the root child of the drawing part that holds the
    /// chart's frame whole, as `Package::remove_image` skips a picture's,
    /// every other byte kept, and removes the frame's relationship when no
    /// kept element names it. The chart part it targets is removed with its
    /// `Override` when no relationship of the package reaches it once the
    /// removed relationships are left out, with its rels part and, the same
    /// way, each part that rels part's relationships alone reach (the chart
    /// style and colours parts, a user shapes drawing with its own rels
    /// part, an embedded package; decision 3 A). A drawing left with no
    /// anchor goes as an image removal's does.
    ///
    /// # Errors
    ///
    /// None at the call. The write refuses an index past the list, an index
    /// named twice, and a part this write also adds an image or a chart to,
    /// "no chart at that position" (the caller writes twice); a frame whose
    /// relationship names no chart part, or a part whose root is not
    /// `chartSpace`, "a chart part cannot be read"; and what
    /// [`Package::add_image`](super::Package) says it refuses of the part,
    /// as an image removal does. On a chartsheet the drawing stays, empty,
    /// its `drawing` element required (`specs/0038-chartsheets.md`); a part
    /// whose root is neither a worksheet nor a chartsheet is refused with
    /// "the sheet part's root is not worksheet or chartsheet".
    pub fn remove_chart(&mut self, part: &[u8], index: usize) -> Result<(), Error> {
        self.parts().remove_chart(part, index)
    }

    /// Records a new chartsheet named `name` at the end of the list, holding
    /// `chart`, and returns its entry name (`specs/0038-chartsheets.md`,
    /// "`add_chartsheet`"). The Cargo feature `charts`.
    ///
    /// Recorded as [`Package::add_sheet`](super::Package) records a
    /// worksheet, the name checked by the same rules and the `<sheet>`
    /// taking `sheetId` one above the largest: the entry
    /// `<dir>/chartsheets/sheet<n>.xml`, `n` the smallest positive integer
    /// whose name no entry of the archive or of this write's additions
    /// takes, ASCII case folded, holding `sheetViews` (one `sheetView` with
    /// `workbookViewId` 0 and `zoomToFit` 1) and `drawing`; its `Override`
    /// of the chartsheet content type unless a `Default` for `xml` names
    /// it; a chartsheet relationship on the workbook part in the form of
    /// the workbook's worksheet relationships; and the sheet's rels part,
    /// a drawing part with its `Override` and rels part, and the chart part
    /// with its `Override`, the chart's frame in an `absoluteAnchor` at 0,0
    /// of the chart's `size`, as [`Package::add_chart`] writes one. The
    /// projected list shows the sheet as
    /// [`SheetKind::Chartsheet`](super::SheetKind). The chart calls do not
    /// take the new part until it is written ("no such part"), as for a
    /// worksheet added in the same write.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: the name rules'
    /// messages and what [`Package::add_sheet`](super::Package) refuses;
    /// then what [`Package::add_chart`] refuses of the chart, "a chart
    /// size does not apply to its anchor" for no `size` (P1 ruling B)
    /// among them; and "no Default covers rels" for a package whose
    /// content types have no `Default` for the `rels` extension.
    pub fn add_chartsheet(&mut self, name: &str, chart: &NewChart<'_>) -> Result<Vec<u8>, Error> {
        self.parts().add_chartsheet(name, chart)
    }
}

impl Parts<'_> {
    /// The body of [`Package::add_chart`]: the call's checks in P1 ruling
    /// C's order, then the chart recorded.
    fn add_chart(
        &mut self,
        part: &[u8],
        anchor: ImageAnchor,
        chart: &NewChart<'_>,
    ) -> Result<(), Error> {
        let work = chart_work(anchor, chart)?;
        self.edits
            .drawing_edits
            .push((part.to_vec(), anchor, NewDrawing::Chart(Box::new(work))));
        Ok(())
    }

    /// The body of [`Package::remove_chart`].
    fn remove_chart(&mut self, part: &[u8], index: usize) -> Result<(), Error> {
        self.edits
            .drawing_removals
            .push((part.to_vec(), Removal::Chart(index)));
        Ok(())
    }
}

/// [`Package::add_chart`]'s checks of `anchor` and `chart`, in P1 ruling
/// C's order, and the chart as the write takes it; [`Package::add_chartsheet`]
/// checks its chart the same way (`specs/0038-chartsheets.md`).
pub(super) fn chart_work(anchor: ImageAnchor, chart: &NewChart<'_>) -> Result<ChartWork, Error> {
    {
        let most = u64::from(i32::MAX.unsigned_abs());
        let fits = |(across, down): (u32, u32)| {
            u64::from(across) * EMU_PER_PIXEL <= most && u64::from(down) * EMU_PER_PIXEL <= most
        };
        let cells: &[_] = match &anchor {
            ImageAnchor::OneCell { from, .. } => &[*from],
            ImageAnchor::TwoCell { from, to, .. } => &[*from, *to],
            ImageAnchor::Absolute { .. } => &[],
        };
        if !cells.iter().all(|cell| in_grid(*cell)) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        let in_range = match anchor {
            ImageAnchor::OneCell { offset, .. } => fits(offset),
            ImageAnchor::TwoCell {
                from,
                to,
                from_offset,
                to_offset,
                ..
            } => to.row >= from.row && to.col >= from.col && fits(from_offset) && fits(to_offset),
            ImageAnchor::Absolute { position } => fits(position),
        };
        if !in_range {
            return Err(CHART_ANCHOR_OUT_OF_RANGE);
        }
        let two_cell = matches!(anchor, ImageAnchor::TwoCell { .. });
        let (cx, cy) = match (two_cell, chart.size) {
            // The cells and offsets set the box; no extent is written.
            (true, None) => (0, 0),
            (false, Some((width, height))) => {
                let extent = |pixels: u32| {
                    let emu = u64::from(pixels) * EMU_PER_PIXEL;
                    (pixels > 0 && emu <= most).then_some(emu)
                };
                let (Some(cx), Some(cy)) = (extent(width), extent(height)) else {
                    return Err(CHART_ANCHOR_OUT_OF_RANGE);
                };
                (cx, cy)
            }
            _ => return Err(CHART_SIZE_ANCHOR),
        };
        if chart.series.is_empty() {
            return Err(CHART_OPTION_KIND);
        }
        let axes = AxisOptions {
            titles: chart.category_axis_title.is_some() || chart.value_axis_title.is_some(),
            category_format: chart.category_axis_format.is_some(),
            value_format: chart.value_axis_format.is_some(),
        };
        if !plot_applies(chart.plot, chart.series, axes) {
            return Err(CHART_OPTION_KIND);
        }
        if let NewPlot::OfPie {
            split: Some(split), ..
        } = chart.plot
        {
            let in_range = match split {
                OfPieSplit::Position(points) => points >= 1,
                OfPieSplit::Value(value) => value.is_finite() && value >= 0.0,
                OfPieSplit::Percent(percent) => {
                    percent.is_finite() && (0.0..=100.0).contains(&percent)
                }
            };
            if !in_range {
                return Err(CHART_VALUE_RANGE);
            }
        }
        series_options(chart.plot, chart.series)?;
        if let Some(combined) = &chart.combined {
            // The first plot a bar, line or area (no 3D plot combines: P2
            // ruling G; a scatter's x axis cannot share a category axis: P2
            // ruling J); the second a bar, line, area or scatter of another
            // variant, its series by its own kind's rules (P3 ruling L).
            let first = matches!(
                chart.plot,
                NewPlot::Bar { .. } | NewPlot::Line { .. } | NewPlot::Area { .. }
            );
            let second = matches!(
                combined.plot,
                NewPlot::Bar { .. }
                    | NewPlot::Line { .. }
                    | NewPlot::Area { .. }
                    | NewPlot::Scatter { .. }
            );
            let scatter = matches!(combined.plot, NewPlot::Scatter { .. });
            let secondary_only =
                combined.secondary_axis_title.is_some() || combined.secondary_axis_format.is_some();
            let applies = first
                && second
                && std::mem::discriminant(&chart.plot) != std::mem::discriminant(&combined.plot)
                && !combined.series.is_empty()
                && plot_applies(combined.plot, combined.series, AxisOptions::NONE)
                && (combined.secondary_axis || !scatter && !secondary_only);
            if !applies {
                return Err(CHART_OPTION_KIND);
            }
            series_options(combined.plot, combined.series)?;
        }
        let title = chart.title.map(text).transpose()?;
        let category_axis_title = chart.category_axis_title.map(text).transpose()?;
        let value_axis_title = chart.value_axis_title.map(text).transpose()?;
        let category_axis_format = chart.category_axis_format.map(text).transpose()?;
        let value_axis_format = chart.value_axis_format.map(text).transpose()?;
        let (secondary_title, secondary_format) = match &chart.combined {
            Some(combined) => (
                combined.secondary_axis_title.map(text).transpose()?,
                combined.secondary_axis_format.map(text).transpose()?,
            ),
            None => (None, None),
        };
        let series = series_work(chart.series)?;
        let combined = match &chart.combined {
            Some(combined) => Some(CombinedWork {
                plot: combined.plot,
                series: series_work(combined.series)?,
                secondary_axis: combined.secondary_axis,
                title: secondary_title,
                format: secondary_format,
            }),
            None => None,
        };
        Ok(ChartWork {
            plot: chart.plot,
            title,
            series,
            category_axis_title,
            value_axis_title,
            category_axis_format,
            value_axis_format,
            legend: chart.legend,
            combined,
            cx,
            cy,
        })
    }
}

/// Which axis options a chart gives its first plot: an axis title, the
/// category axis's format, the value axis's format.
#[derive(Clone, Copy)]
struct AxisOptions {
    titles: bool,
    category_format: bool,
    value_format: bool,
}

impl AxisOptions {
    /// None given, as for a combined second plot.
    const NONE: Self = Self {
        titles: false,
        category_format: false,
        value_format: false,
    };
}

/// Whether `plot` takes `series` and the axis options given (`specs/0036-
/// charts-write-second-set.md`, "Applicability"): its grouping, its series
/// count, a doughnut's hole, sizes on a bubble's series alone, the axis
/// titles and formats, each series' colour and its labels; a surface's
/// series take no colour and no labels, an of-pie takes a pie's rules
/// (`specs/0039-chart-kinds-and-formatting.md`, "Applicability").
fn plot_applies(plot: NewPlot, series: &[NewSeries<'_>], axes: AxisOptions) -> bool {
    let count = series.len();
    let bubble = plot == NewPlot::Bubble;
    let no_axis = !axes.titles && !axes.category_format && !axes.value_format;
    let kind = match plot {
        NewPlot::Bar { grouping, .. } => grouping != Grouping::Standard,
        NewPlot::Line { grouping }
        | NewPlot::Area { grouping }
        | NewPlot::Line3D { grouping }
        | NewPlot::Area3D { grouping } => grouping != Grouping::Clustered,
        NewPlot::Pie | NewPlot::Pie3D | NewPlot::OfPie { .. } => count == 1 && no_axis,
        NewPlot::Doughnut { hole_size } => (1..=90).contains(&hole_size) && no_axis,
        NewPlot::Stock => (3..=4).contains(&count),
        NewPlot::Scatter { .. } | NewPlot::Bubble => !axes.category_format,
        _ => true,
    };
    let round = matches!(
        plot,
        NewPlot::Pie | NewPlot::Pie3D | NewPlot::Doughnut { .. } | NewPlot::OfPie { .. }
    );
    let surface = matches!(plot, NewPlot::Surface { .. } | NewPlot::Surface3D { .. });
    kind && series.iter().all(|one| {
        one.sizes.is_some() == bubble
            && one
                .color
                .is_none_or(|color| color <= 0xFF_FFFF && plot != NewPlot::Stock && !surface)
            && one.labels.is_none_or(|labels| {
                !surface
                    && (round || !labels.percent)
                    && labels
                        .position
                        .is_none_or(|position| takes_position(plot, position))
            })
    })
}

/// Whether `plot`'s labels take `position` ("Applicability", P3 ruling L):
/// a bar's the four, `OutsideEnd` not when stacked; a line's, a scatter's
/// and a radar's other than filled the five; a pie's, a 3D pie's and an
/// of-pie's the four; no other plot's any.
fn takes_position(plot: NewPlot, position: LabelPosition) -> bool {
    use LabelPosition::{
        Above, Below, BestFit, Center, InsideBase, InsideEnd, Left, OutsideEnd, Right,
    };
    match plot {
        NewPlot::Bar { grouping, .. } => match position {
            Center | InsideEnd | InsideBase => true,
            OutsideEnd => grouping == Grouping::Clustered,
            _ => false,
        },
        NewPlot::Line { .. }
        | NewPlot::Scatter { .. }
        | NewPlot::Radar {
            style: RadarStyle::Standard | RadarStyle::Marker,
        } => matches!(position, Center | Left | Right | Above | Below),
        NewPlot::Pie | NewPlot::Pie3D | NewPlot::OfPie { .. } => {
            matches!(position, BestFit | Center | InsideEnd | OutsideEnd)
        }
        _ => false,
    }
}

/// Each series' marker, line and points, by `plot`'s kind, series by series
/// in field order (`specs/0039-chart-kinds-and-formatting.md`,
/// "Applicability"): the marker's symbol, size and colour; the line's
/// fields; then point by point the index, the colour, the explosion and the
/// marker. A marker on a line, a scatter, or a radar of the standard or
/// marker style; a line on a line, a 3D line, a scatter, a radar or a stock
/// plot; points on every kind but the surfaces, an explosion on the round
/// kinds alone.
fn series_options(plot: NewPlot, series: &[NewSeries<'_>]) -> Result<(), Error> {
    let takes_marker = matches!(
        plot,
        NewPlot::Line { .. }
            | NewPlot::Scatter { .. }
            | NewPlot::Radar {
                style: RadarStyle::Standard | RadarStyle::Marker,
            }
    );
    let takes_line = matches!(
        plot,
        NewPlot::Line { .. }
            | NewPlot::Line3D { .. }
            | NewPlot::Scatter { .. }
            | NewPlot::Radar { .. }
            | NewPlot::Stock
    );
    let round = matches!(
        plot,
        NewPlot::Pie | NewPlot::Pie3D | NewPlot::Doughnut { .. } | NewPlot::OfPie { .. }
    );
    let surface = matches!(plot, NewPlot::Surface { .. } | NewPlot::Surface3D { .. });
    let marker_options = |marker: NewMarker| {
        if !takes_marker
            || marker.symbol == MarkerSymbol::None
                && (marker.size.is_some() || marker.color.is_some())
        {
            return Err(CHART_OPTION_KIND);
        }
        if marker.size.is_some_and(|size| !(2..=72).contains(&size)) {
            return Err(CHART_VALUE_RANGE);
        }
        if marker.color.is_some_and(|color| color > 0xFF_FFFF) {
            return Err(CHART_OPTION_KIND);
        }
        Ok(())
    };
    for one in series {
        if let Some(marker) = one.marker {
            marker_options(marker)?;
        }
        if let Some(line) = one.line {
            if !takes_line || line.width.is_none() && line.dash.is_none() {
                return Err(CHART_OPTION_KIND);
            }
            if line.width.is_some_and(|width| width > LINE_WIDTH_LIMIT) {
                return Err(CHART_VALUE_RANGE);
            }
        }
        for (k, point) in one.points.iter().enumerate() {
            let bare = point.color.is_none() && point.explosion.is_none() && point.marker.is_none();
            if surface || bare || one.points[..k].iter().any(|seen| seen.index == point.index) {
                return Err(CHART_OPTION_KIND);
            }
            if point.color.is_some_and(|color| color > 0xFF_FFFF)
                || point.explosion.is_some() && !round
            {
                return Err(CHART_OPTION_KIND);
            }
            if let Some(marker) = point.marker {
                marker_options(marker)?;
            }
        }
    }
    Ok(())
}

/// The series' texts and references checked in field order, series by
/// series: the name, the categories, the values, the sizes, the label
/// format.
fn series_work(series: &[NewSeries<'_>]) -> Result<Vec<SeriesWork>, Error> {
    let mut work = Vec::with_capacity(series.len());
    for one in series {
        let name = match one.name {
            None => None,
            Some(NewSeriesName::Reference(formula)) => {
                Some(NameWork::Reference(reference(formula)?))
            }
            Some(NewSeriesName::Text(name)) => Some(NameWork::Text(text(name)?)),
        };
        work.push(SeriesWork {
            name,
            categories: one.categories.map(reference).transpose()?,
            values: reference(one.values)?,
            sizes: one.sizes.map(reference).transpose()?,
            color: one.color,
            labels: match one.labels {
                None => None,
                Some(labels) => Some(LabelsWork {
                    flags: [
                        labels.value,
                        labels.category,
                        labels.series_name,
                        labels.percent,
                    ],
                    position: labels.position,
                    format: labels.number_format.map(text).transpose()?,
                }),
            },
            marker: one.marker,
            line: one.line,
            points: one.points.to_vec(),
        });
    }
    Ok(work)
}

/// Whether `plot` writes the series axis, id 30: a `Standard` bar or area
/// 3D plot, a 3D line plot whatever its grouping (P2 ruling F), and both
/// surfaces (`specs/0039-chart-kinds-and-formatting.md`).
fn series_axis(plot: NewPlot) -> bool {
    matches!(
        plot,
        NewPlot::Bar3D {
            grouping: Grouping::Standard,
            ..
        } | NewPlot::Area3D {
            grouping: Grouping::Standard
        } | NewPlot::Line3D { .. }
            | NewPlot::Surface { .. }
            | NewPlot::Surface3D { .. }
    )
}

/// Whether `value` is a text a chart may hold: not empty, no character
/// below U+0020, U+FFFE or U+FFFF, and at most `limit` bytes.
fn valid(value: &str, limit: usize) -> bool {
    !value.is_empty()
        && value.len() <= limit
        && !value
            .chars()
            .any(|c| c < '\u{20}' || c == '\u{fffe}' || c == '\u{ffff}')
}

/// A title, axis title or text name, checked and escaped as a comment's
/// text is (P1 ruling F).
fn text(value: &str) -> Result<String, Error> {
    if !valid(value, TEXT_LIMIT) {
        return Err(CHART_TEXT_INVALID);
    }
    Ok(escape(value).into_owned())
}

/// A reference, checked and escaped: `&`, `<` and `>` as entities, nothing
/// parsed or quoted (P1 ruling F).
fn reference(value: &str) -> Result<String, Error> {
    if !valid(value, REFERENCE_LIMIT) {
        return Err(CHART_TEXT_INVALID);
    }
    Ok(partial_escape(value).into_owned())
}

impl ChartWork {
    /// The chart part (`specs/0035-charts-write-first-set.md`, "The chart
    /// part"): the declaration on its own line, then the `chartSpace` on
    /// one line, its namespaces the strict forms when `strict` (P1 ruling
    /// E).
    pub(super) fn part_bytes(&self, strict: bool) -> Vec<u8> {
        let s = usize::from(strict);
        let mut out = Vec::new();
        out.extend_from_slice(
            b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<c:chartSpace xmlns:c=\"",
        );
        out.extend_from_slice(CHART_NAMESPACE[s]);
        out.extend_from_slice(b"\" xmlns:a=\"");
        out.extend_from_slice(DRAWINGML[s]);
        out.extend_from_slice(b"\"><c:roundedCorners val=\"0\"/><c:chart>");
        if let Some(title) = &self.title {
            title_bytes(&mut out, title);
        }
        value_element(
            &mut out,
            b"autoTitleDeleted",
            if self.title.is_some() { b"0" } else { b"1" },
        );
        // A 3D plot's view (`specs/0036-charts-write-second-set.md`, "The
        // chart part"); the contour's from above, with no perspective
        // (`specs/0039-chart-kinds-and-formatting.md`).
        let view: Option<[&[u8]; 3]> = match self.plot {
            NewPlot::Bar3D { .. }
            | NewPlot::Line3D { .. }
            | NewPlot::Area3D { .. }
            | NewPlot::Surface3D { .. } => Some([b"15", b"20", b"1"]),
            NewPlot::Pie3D => Some([b"30", b"0", b"0"]),
            NewPlot::Surface { .. } => Some([b"90", b"0", b"0"]),
            _ => None,
        };
        if let Some([x, y, r]) = view {
            out.extend_from_slice(b"<c:view3D>");
            value_element(&mut out, b"rotX", x);
            value_element(&mut out, b"rotY", y);
            value_element(&mut out, b"rAngAx", r);
            if matches!(self.plot, NewPlot::Surface { .. }) {
                value_element(&mut out, b"perspective", b"0");
            }
            out.extend_from_slice(b"</c:view3D>");
        }
        out.extend_from_slice(b"<c:plotArea><c:layout/>");
        plot_bytes(&mut out, self.plot, &self.series, 0, [b"10", b"20"]);
        if let Some(combined) = &self.combined {
            let ids: [&[u8]; 2] = if combined.secondary_axis {
                [b"30", b"40"]
            } else {
                [b"10", b"20"]
            };
            plot_bytes(
                &mut out,
                combined.plot,
                &combined.series,
                self.series.len(),
                ids,
            );
        }
        self.axes_bytes(&mut out);
        out.extend_from_slice(b"</c:plotArea>");
        if let Some(legend) = self.legend {
            out.extend_from_slice(b"<c:legend>");
            value_element(
                &mut out,
                b"legendPos",
                match legend {
                    LegendPosition::Bottom => b"b",
                    LegendPosition::Left => b"l",
                    LegendPosition::Right => b"r",
                    LegendPosition::Top => b"t",
                    LegendPosition::TopRight => b"tr",
                },
            );
            out.extend_from_slice(b"<c:overlay val=\"0\"/></c:legend>");
        }
        out.extend_from_slice(
            b"<c:plotVisOnly val=\"1\"/><c:dispBlanksAs val=\"gap\"/></c:chart></c:chartSpace>",
        );
        out
    }

    /// The first plot's axes: the category axis then the value axis; a bar
    /// plot with direction `Bar` has its category axis at `l` and its value
    /// axis at `b`; a scatter's or a bubble's x axis is a value axis with no
    /// gridlines; a pie, a doughnut and an of-pie have none; then the series
    /// axis when the plot writes it, and a combined plot's secondary pair.
    fn axes_bytes(&self, out: &mut Vec<u8>) {
        let x = &self.category_axis_title;
        let y = &self.value_axis_title;
        let x_format = self.category_axis_format.as_deref();
        let y_format = self.value_axis_format.as_deref();
        let primary = Axis {
            title: &None,
            format: None,
            deleted: false,
            crosses: b"autoZero",
        };
        match self.plot {
            NewPlot::Pie | NewPlot::Pie3D | NewPlot::Doughnut { .. } | NewPlot::OfPie { .. } => {}
            NewPlot::Scatter { .. } | NewPlot::Bubble => {
                let x_axis = Axis {
                    title: x,
                    ..primary
                };
                let y_axis = Axis {
                    title: y,
                    format: y_format,
                    ..primary
                };
                axis_bytes(
                    out,
                    b"10",
                    b"b",
                    false,
                    &x_axis,
                    b"20",
                    Form::Value(b"midCat"),
                );
                axis_bytes(
                    out,
                    b"20",
                    b"l",
                    true,
                    &y_axis,
                    b"10",
                    Form::Value(b"midCat"),
                );
            }
            _ => {
                let bars = matches!(
                    self.plot,
                    NewPlot::Bar {
                        direction: BarDirection::Bar,
                        ..
                    } | NewPlot::Bar3D {
                        direction: BarDirection::Bar,
                        ..
                    }
                );
                let (category, value): (&[u8], &[u8]) =
                    if bars { (b"l", b"b") } else { (b"b", b"l") };
                let x_axis = Axis {
                    title: x,
                    format: x_format,
                    ..primary
                };
                let y_axis = Axis {
                    title: y,
                    format: y_format,
                    ..primary
                };
                axis_bytes(out, b"10", category, false, &x_axis, b"20", Form::Category);
                axis_bytes(
                    out,
                    b"20",
                    value,
                    true,
                    &y_axis,
                    b"10",
                    Form::Value(b"between"),
                );
            }
        }
        if series_axis(self.plot) {
            axis_bytes(out, b"30", b"b", false, &primary, b"20", Form::Series);
        }
        if let Some(combined) = self.combined.as_ref().filter(|one| one.secondary_axis) {
            // The secondary pair: the x axis 30 at the bottom, deleted; the
            // value axis 40 at the right, crossing at the maximum, with the
            // secondary title and format and no gridlines.
            let hidden = Axis {
                deleted: true,
                ..primary
            };
            let shown = Axis {
                title: &combined.title,
                format: combined.format.as_deref(),
                deleted: false,
                crosses: b"max",
            };
            let (x_form, between): (Form<'_>, &[u8]) =
                if matches!(combined.plot, NewPlot::Scatter { .. }) {
                    (Form::Value(b"midCat"), b"midCat")
                } else {
                    (Form::Category, b"between")
                };
            axis_bytes(out, b"30", b"b", false, &hidden, b"40", x_form);
            axis_bytes(out, b"40", b"r", false, &shown, b"30", Form::Value(between));
        }
    }

    /// The anchor's body: a `graphicFrame` whose `c:chart` names the
    /// relationship `rel`, its `cNvPr` `id` and name from `id`, the chart
    /// namespace bound on `c:chart` and the relationships namespace with it
    /// when the drawing's root does not bind it (`specs/0035-charts-write-
    /// first-set.md`, "Anchors").
    pub(super) fn frame_bytes(form: &DrawingForm, id: u64, rel: &[u8]) -> Vec<u8> {
        let s = usize::from(form.strict);
        let x = form.x.as_slice();
        let a = drawing_a(form);
        let a = a.as_slice();
        let (r, bind_r) = drawing_r(form);
        let id = id.to_string();
        [
            b"<".as_slice(),
            x,
            b"graphicFrame macro=\"\"><",
            x,
            b"nvGraphicFramePr><",
            x,
            b"cNvPr id=\"",
            id.as_bytes(),
            b"\" name=\"Chart ",
            id.as_bytes(),
            b"\"/><",
            x,
            b"cNvGraphicFramePr/></",
            x,
            b"nvGraphicFramePr><",
            x,
            b"xfrm><",
            a,
            b"off x=\"0\" y=\"0\"/><",
            a,
            b"ext cx=\"0\" cy=\"0\"/></",
            x,
            b"xfrm><",
            a,
            b"graphic><",
            a,
            b"graphicData uri=\"",
            CHART_NAMESPACE[s],
            b"\"><c:chart xmlns:c=\"",
            CHART_NAMESPACE[s],
            b"\"",
            &bind_r,
            b" ",
            &r,
            b":id=\"",
            rel,
            b"\"/></",
            a,
            b"graphicData></",
            a,
            b"graphic></",
            x,
            b"graphicFrame>",
        ]
        .concat()
    }
}

/// A plot element with its series: the options before the series,
/// `varyColors`, the series (`idx` and `order` from `first`), the options
/// after them, the axis ids `ids` and the series axis's when the plot writes
/// it.
fn plot_bytes(
    out: &mut Vec<u8>,
    plot: NewPlot,
    series: &[SeriesWork],
    first: usize,
    ids: [&[u8]; 2],
) {
    let element: &[u8] = match plot {
        NewPlot::Bar { .. } => b"barChart",
        NewPlot::Line { .. } => b"lineChart",
        NewPlot::Pie => b"pieChart",
        NewPlot::Scatter { .. } => b"scatterChart",
        NewPlot::Area { .. } => b"areaChart",
        NewPlot::Doughnut { .. } => b"doughnutChart",
        NewPlot::Radar { .. } => b"radarChart",
        NewPlot::Bubble => b"bubbleChart",
        NewPlot::Stock => b"stockChart",
        NewPlot::Bar3D { .. } => b"bar3DChart",
        NewPlot::Line3D { .. } => b"line3DChart",
        NewPlot::Pie3D => b"pie3DChart",
        NewPlot::Area3D { .. } => b"area3DChart",
        NewPlot::Surface { .. } => b"surfaceChart",
        NewPlot::Surface3D { .. } => b"surface3DChart",
        NewPlot::OfPie { .. } => b"ofPieChart",
    };
    tag(out, b"<c:", element);
    match plot {
        NewPlot::Bar { direction, .. } | NewPlot::Bar3D { direction, .. } => value_element(
            out,
            b"barDir",
            match direction {
                BarDirection::Bar => b"bar",
                BarDirection::Column => b"col",
            },
        ),
        NewPlot::Scatter { style } => value_element(
            out,
            b"scatterStyle",
            match style {
                ScatterStyle::None => b"none".as_slice(),
                ScatterStyle::Line => b"line",
                ScatterStyle::LineMarker => b"lineMarker",
                ScatterStyle::Marker => b"marker",
                ScatterStyle::Smooth => b"smooth",
                ScatterStyle::SmoothMarker => b"smoothMarker",
            },
        ),
        NewPlot::Radar { style } => value_element(
            out,
            b"radarStyle",
            match style {
                RadarStyle::Standard => b"standard".as_slice(),
                RadarStyle::Marker => b"marker",
                RadarStyle::Filled => b"filled",
            },
        ),
        NewPlot::Surface { wireframe } | NewPlot::Surface3D { wireframe } => {
            value_element(out, b"wireframe", if wireframe { b"1" } else { b"0" });
        }
        // The schema's order puts `ofPieType` first.
        NewPlot::OfPie { kind, .. } => value_element(
            out,
            b"ofPieType",
            match kind {
                OfPieKind::Pie => b"pie",
                OfPieKind::Bar => b"bar",
            },
        ),
        _ => {}
    }
    if let NewPlot::Bar { grouping, .. }
    | NewPlot::Line { grouping }
    | NewPlot::Area { grouping }
    | NewPlot::Bar3D { grouping, .. }
    | NewPlot::Line3D { grouping }
    | NewPlot::Area3D { grouping } = plot
    {
        value_element(out, b"grouping", grouping_value(grouping));
    }
    // A pie's, a doughnut's and an of-pie's slices vary; a stock plot and
    // the surfaces have no `varyColors`.
    let round = matches!(
        plot,
        NewPlot::Pie | NewPlot::Pie3D | NewPlot::Doughnut { .. } | NewPlot::OfPie { .. }
    );
    let surface = matches!(plot, NewPlot::Surface { .. } | NewPlot::Surface3D { .. });
    if plot != NewPlot::Stock && !surface {
        value_element(out, b"varyColors", if round { b"1" } else { b"0" });
    }
    series_bytes(out, plot, series, first);
    match plot {
        NewPlot::Bar { grouping, .. } => {
            value_element(out, b"gapWidth", b"150");
            // Excel draws stacked bars side by side without it.
            if matches!(grouping, Grouping::Stacked | Grouping::PercentStacked) {
                value_element(out, b"overlap", b"100");
            }
        }
        NewPlot::Bar3D { .. } => {
            value_element(out, b"gapWidth", b"150");
            value_element(out, b"shape", b"box");
        }
        NewPlot::Line { .. } => value_element(out, b"marker", b"1"),
        NewPlot::Pie => value_element(out, b"firstSliceAng", b"0"),
        NewPlot::Doughnut { hole_size } => {
            value_element(out, b"firstSliceAng", b"0");
            value_element(out, b"holeSize", hole_size.to_string().as_bytes());
        }
        NewPlot::Bubble => {
            value_element(out, b"bubbleScale", b"100");
            value_element(out, b"showNegBubbles", b"0");
        }
        NewPlot::Stock => out.extend_from_slice(b"<c:hiLowLines/>"),
        NewPlot::OfPie { split, .. } => {
            value_element(out, b"gapWidth", b"150");
            let (split_type, position): (&[u8], Option<String>) = match split {
                None => (b"auto", None),
                Some(OfPieSplit::Position(points)) => (b"pos", Some(points.to_string())),
                Some(OfPieSplit::Value(value)) => (b"val", Some(value.to_string())),
                Some(OfPieSplit::Percent(percent)) => (b"percent", Some(percent.to_string())),
            };
            value_element(out, b"splitType", split_type);
            if let Some(position) = position {
                value_element(out, b"splitPos", position.as_bytes());
            }
            value_element(out, b"secondPieSize", b"75");
            out.extend_from_slice(b"<c:serLines/>");
        }
        _ => {}
    }
    if !round {
        value_element(out, b"axId", ids[0]);
        value_element(out, b"axId", ids[1]);
        if series_axis(plot) {
            value_element(out, b"axId", b"30");
        }
    }
    tag(out, b"</c:", element);
}

/// Each series in order, `idx` and `order` its position counted from
/// `first`; its colour after `tx`, with the line's colour, width and dash
/// on a line, 3D line, scatter, radar or stock plot; a stock series' marker
/// `none`, or the series' marker; its points; its labels before the
/// references; a scatter's and a bubble's references as
/// `xVal` and `yVal`, both `numRef`, a bubble's sizes after them.
fn series_bytes(out: &mut Vec<u8>, plot: NewPlot, series: &[SeriesWork], first: usize) {
    let scatter = matches!(plot, NewPlot::Scatter { .. } | NewPlot::Bubble);
    let (x, x_form, y): (&[u8], &[u8], &[u8]) = if scatter {
        (b"xVal", b"numRef", b"yVal")
    } else {
        (b"cat", b"strRef", b"val")
    };
    for (k, series) in series.iter().enumerate() {
        let k = (first + k).to_string();
        out.extend_from_slice(b"<c:ser>");
        value_element(out, b"idx", k.as_bytes());
        value_element(out, b"order", k.as_bytes());
        match &series.name {
            None => {}
            Some(NameWork::Reference(formula)) => {
                element_bytes(out, b"tx", b"strRef", formula);
            }
            Some(NameWork::Text(name)) => {
                out.extend_from_slice(b"<c:tx><c:v>");
                out.extend_from_slice(name.as_bytes());
                out.extend_from_slice(b"</c:v></c:tx>");
            }
        }
        if series.color.is_some() || series.line.is_some() {
            out.extend_from_slice(b"<c:spPr>");
            if let Some(color) = series.color {
                solid_fill(out, color);
            }
            if matches!(
                plot,
                NewPlot::Line { .. }
                    | NewPlot::Line3D { .. }
                    | NewPlot::Scatter { .. }
                    | NewPlot::Radar { .. }
                    | NewPlot::Stock
            ) {
                // The line's width and dash with the series' colour
                // (`specs/0039-chart-kinds-and-formatting.md`, "The chart
                // part").
                out.extend_from_slice(b"<a:ln");
                if let Some(width) = series.line.and_then(|line| line.width) {
                    out.extend_from_slice(format!(" w=\"{width}\"").as_bytes());
                }
                out.extend_from_slice(b">");
                if let Some(color) = series.color {
                    solid_fill(out, color);
                }
                if let Some(dash) = series.line.and_then(|line| line.dash) {
                    dash_bytes(out, dash);
                }
                out.extend_from_slice(b"</a:ln>");
            }
            out.extend_from_slice(b"</c:spPr>");
        }
        if plot == NewPlot::Stock {
            out.extend_from_slice(b"<c:marker><c:symbol val=\"none\"/></c:marker>");
        } else if let Some(marker) = series.marker {
            marker_bytes(out, marker);
        }
        for point in &series.points {
            out.extend_from_slice(b"<c:dPt>");
            value_element(out, b"idx", point.index.to_string().as_bytes());
            if let Some(marker) = point.marker {
                marker_bytes(out, marker);
            }
            if let Some(explosion) = point.explosion {
                value_element(out, b"explosion", explosion.to_string().as_bytes());
            }
            if let Some(color) = point.color {
                out.extend_from_slice(b"<c:spPr>");
                solid_fill(out, color);
                out.extend_from_slice(b"</c:spPr>");
            }
            out.extend_from_slice(b"</c:dPt>");
        }
        if let Some(labels) = &series.labels {
            labels_bytes(out, labels);
        }
        if let Some(categories) = &series.categories {
            element_bytes(out, x, x_form, categories);
        }
        element_bytes(out, y, b"numRef", &series.values);
        if let Some(sizes) = &series.sizes {
            element_bytes(out, b"bubbleSize", b"numRef", sizes);
        }
        out.extend_from_slice(b"</c:ser>");
    }
}

/// An `a:solidFill` of the colour `color`, six upper-case hex digits.
fn solid_fill(out: &mut Vec<u8>, color: u32) {
    out.extend_from_slice(
        format!("<a:solidFill><a:srgbClr val=\"{color:06X}\"/></a:solidFill>").as_bytes(),
    );
}

/// A line's `a:prstDash`.
fn dash_bytes(out: &mut Vec<u8>, dash: DashStyle) {
    let value: &[u8] = match dash {
        DashStyle::Solid => b"solid",
        DashStyle::Dot => b"dot",
        DashStyle::Dash => b"dash",
        DashStyle::LargeDash => b"lgDash",
        DashStyle::DashDot => b"dashDot",
        DashStyle::LargeDashDot => b"lgDashDot",
        DashStyle::LargeDashDotDot => b"lgDashDotDot",
        DashStyle::SystemDash => b"sysDash",
        DashStyle::SystemDot => b"sysDot",
        DashStyle::SystemDashDot => b"sysDashDot",
        DashStyle::SystemDashDotDot => b"sysDashDotDot",
    };
    for piece in [b"<a:prstDash val=\"".as_slice(), value, b"\"/>"] {
        out.extend_from_slice(piece);
    }
}

/// A series' or a point's `marker`: the symbol, the size, then the fill.
fn marker_bytes(out: &mut Vec<u8>, marker: NewMarker) {
    out.extend_from_slice(b"<c:marker>");
    value_element(
        out,
        b"symbol",
        match marker.symbol {
            MarkerSymbol::None => b"none",
            MarkerSymbol::Auto => b"auto",
            MarkerSymbol::Circle => b"circle",
            MarkerSymbol::Dash => b"dash",
            MarkerSymbol::Diamond => b"diamond",
            MarkerSymbol::Dot => b"dot",
            MarkerSymbol::Plus => b"plus",
            MarkerSymbol::Square => b"square",
            MarkerSymbol::Star => b"star",
            MarkerSymbol::Triangle => b"triangle",
            MarkerSymbol::X => b"x",
        },
    );
    if let Some(size) = marker.size {
        value_element(out, b"size", size.to_string().as_bytes());
    }
    if let Some(color) = marker.color {
        out.extend_from_slice(b"<c:spPr>");
        solid_fill(out, color);
        out.extend_from_slice(b"</c:spPr>");
    }
    out.extend_from_slice(b"</c:marker>");
}

/// A series' `dLbls`: the format, the position, then what a label shows.
fn labels_bytes(out: &mut Vec<u8>, labels: &LabelsWork) {
    out.extend_from_slice(b"<c:dLbls>");
    if let Some(format) = &labels.format {
        number_format(out, format);
    }
    if let Some(position) = labels.position {
        value_element(
            out,
            b"dLblPos",
            match position {
                LabelPosition::BestFit => b"bestFit".as_slice(),
                LabelPosition::Center => b"ctr",
                LabelPosition::InsideEnd => b"inEnd",
                LabelPosition::InsideBase => b"inBase",
                LabelPosition::OutsideEnd => b"outEnd",
                LabelPosition::Left => b"l",
                LabelPosition::Right => b"r",
                LabelPosition::Above => b"t",
                LabelPosition::Below => b"b",
            },
        );
    }
    value_element(out, b"showLegendKey", b"0");
    let names: [&[u8]; 4] = [b"showVal", b"showCatName", b"showSerName", b"showPercent"];
    for (name, flag) in names.into_iter().zip(labels.flags) {
        value_element(out, name, if flag { b"1" } else { b"0" });
    }
    out.extend_from_slice(b"<c:showBubbleSize val=\"0\"/></c:dLbls>");
}

/// A `numFmt` of the format code `format`, not source linked.
fn number_format(out: &mut Vec<u8>, format: &str) {
    out.extend_from_slice(b"<c:numFmt formatCode=\"");
    out.extend_from_slice(format.as_bytes());
    out.extend_from_slice(b"\" sourceLinked=\"0\"/>");
}

/// A `grouping` value.
fn grouping_value(grouping: Grouping) -> &'static [u8] {
    match grouping {
        Grouping::Standard => b"standard",
        Grouping::Clustered => b"clustered",
        Grouping::Stacked => b"stacked",
        Grouping::PercentStacked => b"percentStacked",
    }
}

/// A chart or axis title, its escaped text one run of one paragraph.
fn title_bytes(out: &mut Vec<u8>, text: &str) {
    out.extend_from_slice(b"<c:title><c:tx><c:rich><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>");
    out.extend_from_slice(text.as_bytes());
    out.extend_from_slice(b"</a:t></a:r></a:p></c:rich></c:tx><c:overlay val=\"0\"/></c:title>");
}

/// An element `<c:NAME val="VALUE"/>`.
fn value_element(out: &mut Vec<u8>, name: &[u8], value: &[u8]) {
    for piece in [b"<c:".as_slice(), name, b" val=\"", value, b"\"/>"] {
        out.extend_from_slice(piece);
    }
}

/// `open` (`<c:` or `</c:`), the element's name and `>`.
fn tag(out: &mut Vec<u8>, open: &[u8], name: &[u8]) {
    for piece in [open, name, b">"] {
        out.extend_from_slice(piece);
    }
}

/// An axis's title, number format, deletion and crossing: an axis with no
/// format writes `General`, source linked; the primary axes are shown and
/// cross at `autoZero`.
#[derive(Clone, Copy)]
struct Axis<'a> {
    title: &'a Option<String>,
    format: Option<&'a str>,
    deleted: bool,
    crosses: &'a [u8],
}

/// An axis's form: a `catAx`, a `valAx` with its `crossBetween`, or a
/// `serAx`, which takes no number format and no tail after `crosses`.
#[derive(Clone, Copy)]
enum Form<'a> {
    Category,
    Value(&'a [u8]),
    Series,
}

/// An axis of `form`: its id, position, gridlines, title, format,
/// deletion, crossing axis and crossing.
fn axis_bytes(
    out: &mut Vec<u8>,
    id: &[u8],
    position: &[u8],
    gridlines: bool,
    axis: &Axis<'_>,
    cross: &[u8],
    form: Form<'_>,
) {
    let element: &[u8] = match form {
        Form::Category => b"catAx",
        Form::Value(_) => b"valAx",
        Form::Series => b"serAx",
    };
    tag(out, b"<c:", element);
    value_element(out, b"axId", id);
    out.extend_from_slice(b"<c:scaling><c:orientation val=\"minMax\"/></c:scaling>");
    value_element(out, b"delete", if axis.deleted { b"1" } else { b"0" });
    value_element(out, b"axPos", position);
    if gridlines {
        out.extend_from_slice(b"<c:majorGridlines/>");
    }
    if let Some(title) = axis.title {
        title_bytes(out, title);
    }
    match (form, axis.format) {
        (Form::Series, _) => {}
        (_, Some(format)) => number_format(out, format),
        (_, None) => {
            out.extend_from_slice(b"<c:numFmt formatCode=\"General\" sourceLinked=\"1\"/>")
        }
    }
    out.extend_from_slice(
        b"<c:majorTickMark val=\"out\"/><c:minorTickMark val=\"none\"/><c:tickLblPos val=\"nextTo\"/>",
    );
    value_element(out, b"crossAx", cross);
    value_element(out, b"crosses", axis.crosses);
    match form {
        Form::Value(between) => value_element(out, b"crossBetween", between),
        Form::Category => out.extend_from_slice(
            b"<c:auto val=\"1\"/><c:lblAlgn val=\"ctr\"/><c:lblOffset val=\"100\"/><c:noMultiLvlLbl val=\"0\"/>",
        ),
        Form::Series => {}
    }
    tag(out, b"</c:", element);
}

/// A series' `tx`, `cat`, `val`, `xVal` or `yVal` element holding a
/// reference of `form`.
fn element_bytes(out: &mut Vec<u8>, element: &[u8], form: &[u8], formula: &str) {
    tag(out, b"<c:", element);
    tag(out, b"<c:", form);
    out.extend_from_slice(b"<c:f>");
    out.extend_from_slice(formula.as_bytes());
    out.extend_from_slice(b"</c:f>");
    tag(out, b"</c:", form);
    tag(out, b"</c:", element);
}
