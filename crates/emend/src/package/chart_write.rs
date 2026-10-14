//! A chart written into a worksheet's drawing from cell ranges
//! (`specs/0035-charts-write-first-set.md`). The Cargo feature `charts`.
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
    LegendPosition, NewDrawing, Package, Parts, ScatterStyle, drawing_a, drawing_r, in_grid,
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

/// A chart part's content type (`specs/0035-charts-write-first-set.md`,
/// "The parts").
pub(crate) const CHART_CONTENT_TYPE: &[u8] =
    b"application/vnd.openxmlformats-officedocument.drawingml.chart+xml";

/// The longest title, axis title or text name, in bytes.
const TEXT_LIMIT: usize = 32_767;
/// The longest reference, in bytes.
const REFERENCE_LIMIT: usize = 8_192;

/// A chart as a caller gives it to [`Package::add_chart`]
/// (`specs/0035-charts-write-first-set.md`, "The call"). The Cargo feature
/// `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    /// The legend's position; `None` writes no legend.
    pub legend: Option<LegendPosition>,
    /// The chart's pixel width and height: required for a one-cell or
    /// absolute anchor, refused for a two-cell anchor.
    pub size: Option<(u32, u32)>,
}

/// A new chart's plot: its kind and the options the kind takes. The Cargo
/// feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}

/// A new chart's series: its name and its references, each written as the
/// file's `f` text, as given (decision 2). The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NewSeries<'a> {
    /// The series' name, `tx`.
    pub name: Option<NewSeriesName<'a>>,
    /// The categories, `cat`, or a scatter's x values, `xVal`.
    pub categories: Option<&'a str>,
    /// The values, `val`, or a scatter's y values, `yVal`.
    pub values: &'a str,
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
    legend: Option<LegendPosition>,
    pub(super) cx: u64,
    pub(super) cy: u64,
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
    /// line's or an area's `Clustered`, or a pie with more than one series
    /// or an axis title; "a chart text is empty or holds a control
    /// character" for a title, axis title, name or reference, in that
    /// order, series by series, that is empty, holds a character below
    /// U+0020, U+FFFE or U+FFFF, or is longer than 32767 bytes (a reference,
    /// 8192). The write refuses what [`Package::add_image`](super::Package)
    /// says it refuses of the part, and a part this write also removes an
    /// image from, "no image at that position" (P1 ruling B).
    pub fn add_chart(
        &mut self,
        part: &[u8],
        anchor: ImageAnchor,
        chart: &NewChart<'_>,
    ) -> Result<(), Error> {
        self.parts().add_chart(part, anchor, chart)
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
        let applies = match chart.plot {
            NewPlot::Bar { grouping, .. } => grouping != Grouping::Standard,
            NewPlot::Line { grouping } | NewPlot::Area { grouping } => {
                grouping != Grouping::Clustered
            }
            NewPlot::Pie => {
                chart.series.len() == 1
                    && chart.category_axis_title.is_none()
                    && chart.value_axis_title.is_none()
            }
            NewPlot::Scatter { .. } => true,
        };
        if !applies {
            return Err(CHART_OPTION_KIND);
        }
        let title = chart.title.map(text).transpose()?;
        let category_axis_title = chart.category_axis_title.map(text).transpose()?;
        let value_axis_title = chart.value_axis_title.map(text).transpose()?;
        let mut series = Vec::with_capacity(chart.series.len());
        for one in chart.series {
            let name = match one.name {
                None => None,
                Some(NewSeriesName::Reference(formula)) => {
                    Some(NameWork::Reference(reference(formula)?))
                }
                Some(NewSeriesName::Text(name)) => Some(NameWork::Text(text(name)?)),
            };
            series.push(SeriesWork {
                name,
                categories: one.categories.map(reference).transpose()?,
                values: reference(one.values)?,
            });
        }
        self.edits.drawing_edits.push((
            part.to_vec(),
            anchor,
            NewDrawing::Chart(Box::new(ChartWork {
                plot: chart.plot,
                title,
                series,
                category_axis_title,
                value_axis_title,
                legend: chart.legend,
                cx,
                cy,
            })),
        ));
        Ok(())
    }
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
        out.extend_from_slice(b"<c:plotArea><c:layout/>");
        self.plot_bytes(&mut out);
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

    /// The plot element with its series: the options before the series,
    /// `varyColors`, the series, the options after them, the axis ids.
    fn plot_bytes(&self, out: &mut Vec<u8>) {
        let element: &[u8] = match self.plot {
            NewPlot::Bar { .. } => b"barChart",
            NewPlot::Line { .. } => b"lineChart",
            NewPlot::Pie => b"pieChart",
            NewPlot::Scatter { .. } => b"scatterChart",
            NewPlot::Area { .. } => b"areaChart",
        };
        tag(out, b"<c:", element);
        match self.plot {
            NewPlot::Bar { direction, .. } => value_element(
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
            _ => {}
        }
        if let NewPlot::Bar { grouping, .. }
        | NewPlot::Line { grouping }
        | NewPlot::Area { grouping } = self.plot
        {
            value_element(out, b"grouping", grouping_value(grouping));
        }
        let pie = self.plot == NewPlot::Pie;
        value_element(out, b"varyColors", if pie { b"1" } else { b"0" });
        self.series_bytes(out);
        match self.plot {
            NewPlot::Bar { grouping, .. } => {
                value_element(out, b"gapWidth", b"150");
                // Excel draws stacked bars side by side without it.
                if matches!(grouping, Grouping::Stacked | Grouping::PercentStacked) {
                    value_element(out, b"overlap", b"100");
                }
            }
            NewPlot::Line { .. } => value_element(out, b"marker", b"1"),
            NewPlot::Pie => value_element(out, b"firstSliceAng", b"0"),
            _ => {}
        }
        if !pie {
            value_element(out, b"axId", b"10");
            value_element(out, b"axId", b"20");
        }
        tag(out, b"</c:", element);
    }

    /// Each series in order, `idx` and `order` its zero-based position; a
    /// scatter's references as `xVal` and `yVal`, both `numRef`.
    fn series_bytes(&self, out: &mut Vec<u8>) {
        let scatter = matches!(self.plot, NewPlot::Scatter { .. });
        let (x, x_form, y): (&[u8], &[u8], &[u8]) = if scatter {
            (b"xVal", b"numRef", b"yVal")
        } else {
            (b"cat", b"strRef", b"val")
        };
        for (k, series) in self.series.iter().enumerate() {
            let k = k.to_string();
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
            if let Some(categories) = &series.categories {
                element_bytes(out, x, x_form, categories);
            }
            element_bytes(out, y, b"numRef", &series.values);
            out.extend_from_slice(b"</c:ser>");
        }
    }

    /// The category axis then the value axis; a bar plot with direction
    /// `Bar` has its category axis at `l` and its value axis at `b`; a
    /// scatter plot's x axis is a value axis with no gridlines; a pie has
    /// none.
    fn axes_bytes(&self, out: &mut Vec<u8>) {
        let x = &self.category_axis_title;
        let y = &self.value_axis_title;
        match self.plot {
            NewPlot::Pie => {}
            NewPlot::Scatter { .. } => {
                axis_bytes(out, b"10", b"b", false, x, b"20", Some(b"midCat"));
                axis_bytes(out, b"20", b"l", true, y, b"10", Some(b"midCat"));
            }
            _ => {
                let bars = matches!(
                    self.plot,
                    NewPlot::Bar {
                        direction: BarDirection::Bar,
                        ..
                    }
                );
                let (category, value): (&[u8], &[u8]) =
                    if bars { (b"l", b"b") } else { (b"b", b"l") };
                axis_bytes(out, b"10", category, false, x, b"20", None);
                axis_bytes(out, b"20", value, true, y, b"10", Some(b"between"));
            }
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

/// An axis: a `catAx` when `between` is `None`, else a `valAx` with that
/// `crossBetween`; its id, position, gridlines, title and crossing axis.
fn axis_bytes(
    out: &mut Vec<u8>,
    id: &[u8],
    position: &[u8],
    gridlines: bool,
    title: &Option<String>,
    cross: &[u8],
    between: Option<&[u8]>,
) {
    let element: &[u8] = if between.is_some() {
        b"valAx"
    } else {
        b"catAx"
    };
    tag(out, b"<c:", element);
    value_element(out, b"axId", id);
    out.extend_from_slice(
        b"<c:scaling><c:orientation val=\"minMax\"/></c:scaling><c:delete val=\"0\"/>",
    );
    value_element(out, b"axPos", position);
    if gridlines {
        out.extend_from_slice(b"<c:majorGridlines/>");
    }
    if let Some(title) = title {
        title_bytes(out, title);
    }
    out.extend_from_slice(
        b"<c:numFmt formatCode=\"General\" sourceLinked=\"1\"/><c:majorTickMark val=\"out\"/><c:minorTickMark val=\"none\"/><c:tickLblPos val=\"nextTo\"/>",
    );
    value_element(out, b"crossAx", cross);
    out.extend_from_slice(b"<c:crosses val=\"autoZero\"/>");
    match between {
        Some(between) => value_element(out, b"crossBetween", between),
        None => out.extend_from_slice(
            b"<c:auto val=\"1\"/><c:lblAlgn val=\"ctr\"/><c:lblOffset val=\"100\"/><c:noMultiLvlLbl val=\"0\"/>",
        ),
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
