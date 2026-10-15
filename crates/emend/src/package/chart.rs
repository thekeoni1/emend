//! A worksheet's charts, read as written (`specs/0034-charts-read.md`). The
//! Cargo feature `charts`.
//!
//! [`Package::charts`] finds the drawing part a worksheet's `<drawing>`
//! names, as [`Package::images`](super::Package) finds it, and streams it
//! once for its anchors: each root anchor, or the first anchor in a root
//! `AlternateContent`'s first `Choice` (P1 ruling A), whose `graphicFrame`
//! holds a `c:chart`, gives one chart. Each chart part the anchors name is
//! then streamed once: the title, the plots and their series, the axes, the
//! legend's position, the pivot source and the style are kept and every
//! other child is walked past with its subtree, so what is held is bounded
//! by the series, the axes and their strings, not by the caches, whose
//! points are never read. An `AlternateContent` at the chart part's root
//! is entered at its first `Choice` for the style alone; none below the
//! drawing's root anchors or the chart part's root children is entered,
//! and a `Fallback` never. The chart style, colour style and user shapes
//! parts are never opened.

use std::io::{Read, Seek};

use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};

use super::{
    Anchor, CHART, DRAWING_UNREADABLE, DRAWINGML, EditAs, MARKUP_COMPATIBILITY, NO_SUCH_PART,
    Package, Parts, SPREADSHEET_DRAWING, digits_value, is_relationships_attribute,
};
use crate::archive::Error;
use crate::cells::{skip_subtree, text_of};
use crate::xml::{Scope, Stream};

/// What a chart this layer cannot read into the types is refused with;
/// [`crate::package`]'s catalogue lists it.
pub(crate) const CHART_UNREADABLE: Error = Error::Package("a chart cannot be read");

/// The chart namespace, transitional then strict (ECMA-376 Part 1, 21.2).
pub(super) const CHART_NAMESPACE: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/drawingml/2006/chart",
    b"http://purl.oclc.org/ooxml/drawingml/chart",
];

/// The Office 2007 chart extension namespace, whose `style` an `mc:Choice`
/// at the chart part's root carries (`c14:style`).
const CHART_2007_NAMESPACE: &[u8] = b"http://schemas.microsoft.com/office/drawing/2007/8/2/chart";

/// One chart of a worksheet's drawing: its part, its anchor and what the
/// part says. The Cargo feature `charts`.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartEntry {
    /// The chart part's entry name, the `c:chart` `r:id` resolved through
    /// the drawing's rels part.
    pub part: Vec<u8>,
    /// The anchor the chart's frame sits in.
    pub anchor: ChartAnchor,
    /// The chart part, read.
    pub chart: Chart,
}

/// The anchor a chart's frame sits in. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartAnchor {
    /// The anchor's kind, shared with `images`.
    pub kind: Anchor,
    /// The anchor's `from` column and row, zero-based; both 0 for an
    /// absolute anchor.
    pub from: (u32, u32),
    /// A two-cell anchor's `to` column and row, zero-based; `None` for the
    /// other kinds.
    pub to: Option<(u32, u32)>,
    /// A two-cell anchor's `editAs`; `None` when absent and for the other
    /// kinds.
    pub edit_as: Option<EditAs>,
}

/// A chart part's `c:chart`. The Cargo feature `charts`.
#[derive(Clone, Debug, PartialEq)]
pub struct Chart {
    /// The title's text: the `a:t` text of every run of its `tx/rich`
    /// paragraphs, the paragraphs joined by a newline, or the text of its
    /// `tx/strRef/strCache`'s first point; `None` for no `title`, a title
    /// with no `tx` (an automatic title), or a `strRef` with no point.
    pub title: Option<String>,
    /// `autoTitleDeleted`; false when absent.
    pub auto_title_deleted: bool,
    /// The `plotArea`'s plot children, in order.
    pub plots: Vec<Plot>,
    /// The `plotArea`'s axes (`catAx`, `valAx`, `dateAx`, `serAx`), in
    /// order.
    pub axes: Vec<ChartAxis>,
    /// The `legend`'s `legendPos`, `Right` (the schema's default) for a
    /// legend without one; `None` for no `legend`.
    pub legend: Option<LegendPosition>,
    /// `pivotSource`'s `name`, as written: the pivot table a pivot chart
    /// shows; `None` for no `pivotSource`.
    pub pivot_source: Option<String>,
    /// The chart part's `style` `val`: a root `c:style`, or the `style` of
    /// the first `Choice` of a root `mc:AlternateContent` (`c14:style`), its
    /// `Fallback` not read; `None` for neither.
    pub style: Option<u32>,
}

/// One plot of a chart's `plotArea`. The Cargo feature `charts`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plot {
    /// The plot element read.
    pub kind: ChartKind,
    /// `barDir`, for the bar plots; `Column` when the element has no `val`.
    pub bar_direction: Option<BarDirection>,
    /// `grouping`, for the bar, line and area plots; the schema's default
    /// when the element has no `val`.
    pub grouping: Option<Grouping>,
    /// `scatterStyle`, for a scatter plot; `Marker` when the element has
    /// no `val`.
    pub scatter_style: Option<ScatterStyle>,
    /// `varyColors`; false when absent.
    pub vary_colors: bool,
    /// The `ser` children, in order.
    pub series: Vec<Series>,
    /// The `axId` children's values, in order, as written: an optionally
    /// signed decimal integer, since Excel writes the signed 32-bit value it
    /// holds where the schema says unsigned (P1 ruling C).
    pub axis_ids: Vec<i64>,
}

/// One axis of a chart's `plotArea`. The Cargo feature `charts`.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartAxis {
    /// `axId`, as written: an optionally signed decimal integer (P1 ruling
    /// C).
    pub id: i64,
    /// The axis element read.
    pub kind: AxisKind,
    /// `axPos`.
    pub position: AxisPosition,
    /// `crossAx`: the `id` of the axis this one crosses, as written (P1
    /// ruling C).
    pub crosses: i64,
    /// The axis title's text, as [`Chart::title`] reads a title.
    pub title: Option<String>,
    /// `numFmt`'s `formatCode`, as written; `None` for no `numFmt`.
    pub number_format: Option<String>,
    /// `scaling/min`'s `val`.
    pub min: Option<f64>,
    /// `scaling/max`'s `val`.
    pub max: Option<f64>,
    /// `delete`; false when absent.
    pub deleted: bool,
    /// Whether a `majorGridlines` element is present.
    pub major_gridlines: bool,
}

/// The axis elements of a `plotArea`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisKind {
    /// `catAx`.
    Category,
    /// `valAx`.
    Value,
    /// `dateAx`.
    Date,
    /// `serAx`.
    Series,
}

/// An axis's `axPos`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisPosition {
    /// `b`.
    Bottom,
    /// `l`.
    Left,
    /// `r`.
    Right,
    /// `t`.
    Top,
}

/// A legend's `legendPos`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendPosition {
    /// `b`.
    Bottom,
    /// `l`.
    Left,
    /// `r`, the schema's default.
    Right,
    /// `t`.
    Top,
    /// `tr`.
    TopRight,
}

/// The plot elements of a `plotArea`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartKind {
    /// `barChart`.
    Bar,
    /// `bar3DChart`.
    Bar3D,
    /// `lineChart`.
    Line,
    /// `line3DChart`.
    Line3D,
    /// `pieChart`.
    Pie,
    /// `pie3DChart`.
    Pie3D,
    /// `doughnutChart`.
    Doughnut,
    /// `ofPieChart`.
    OfPie,
    /// `scatterChart`.
    Scatter,
    /// `areaChart`.
    Area,
    /// `area3DChart`.
    Area3D,
    /// `radarChart`.
    Radar,
    /// `bubbleChart`.
    Bubble,
    /// `stockChart`.
    Stock,
    /// `surfaceChart`.
    Surface,
    /// `surface3DChart`.
    Surface3D,
}

/// A bar plot's `barDir`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarDirection {
    /// `bar`: horizontal bars.
    Bar,
    /// `col`: vertical columns.
    Column,
}

/// A plot's `grouping`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grouping {
    /// `standard`.
    Standard,
    /// `clustered`.
    Clustered,
    /// `stacked`.
    Stacked,
    /// `percentStacked`.
    PercentStacked,
}

/// A scatter plot's `scatterStyle`. The Cargo feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScatterStyle {
    /// `none`.
    None,
    /// `line`.
    Line,
    /// `lineMarker`.
    LineMarker,
    /// `marker`.
    Marker,
    /// `smooth`.
    Smooth,
    /// `smoothMarker`.
    SmoothMarker,
}

/// One `ser` of a plot. The Cargo feature `charts`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Series {
    /// `idx`.
    pub index: u32,
    /// `order`.
    pub order: u32,
    /// `tx`: a `strRef`'s formula or a literal `v`; `None` without `tx`.
    pub name: Option<SeriesName>,
    /// `cat`, or a scatter's `xVal`.
    pub categories: Option<Data>,
    /// `val`, or a scatter's `yVal`.
    pub values: Option<Data>,
    /// A bubble plot's `bubbleSize`.
    pub sizes: Option<Data>,
    /// `spPr/solidFill/srgbClr`'s `val` as `0xRRGGBB`; `None` for any other
    /// fill or none.
    pub fill: Option<u32>,
    /// `smooth`; false when absent.
    pub smooth: bool,
    /// Whether a `marker` element is present.
    pub marker: bool,
    /// Whether a `dLbls` element is present.
    pub labels: bool,
}

/// A series' `tx`. The Cargo feature `charts`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SeriesName {
    /// `strRef`: its `f`, as written.
    Reference(String),
    /// `v`: the text.
    Text(String),
}

/// A series' data: one of `cat`, `val`, `xVal`, `yVal` or `bubbleSize`.
/// The Cargo feature `charts`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Data {
    /// `numRef`, `strRef` or `multiLvlStrRef`.
    Reference {
        /// `f`, as written (no leading `=`).
        formula: String,
        /// The cache's `ptCount`, 0 when the cache has none; `None` without
        /// a cache. The cached points are never read.
        cached: Option<u64>,
    },
    /// `numLit` or `strLit`.
    Literal {
        /// `ptCount`, 0 when absent.
        count: u64,
    },
}

impl<R: Read + Seek> Package<R> {
    /// The charts of the worksheet part `part` (`specs/0034-charts-read.md`,
    /// "The call"): each root child of the drawing part its `<drawing>`
    /// element's `r:id` names, in document order, that is a
    /// `oneCellAnchor`, `twoCellAnchor` or `absoluteAnchor`, or the first
    /// such anchor in a root `AlternateContent`'s first `Choice`, holding a
    /// `graphicFrame` whose `graphic/graphicData` holds a `c:chart`, with
    /// the chart part its `r:id` names, read. An anchor holding a picture
    /// or a shape, or a frame holding something other than a chart, gives
    /// none. A snapshot of the file, not of the pending edits; empty for a
    /// part with no `<drawing>`. The Cargo feature `charts`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a chart cannot be read" for a `c:chart` without
    /// `r:id` or naming no internal `chart` relationship, or one whose part
    /// is missing; a chart part whose root is not `chartSpace`, with no
    /// `chart` or no `plotArea`; a `ser` with no `idx` or `order`; a
    /// reference form with no `f`; a series with both `cat` and `xVal`; an
    /// axis with no `axId`, `axPos` or `crossAx`; a `crosses` with no `val`,
    /// a `numFmt` with no `formatCode` and a `pivotSource` with no `name`
    /// (P2 ruling N); a `grouping`, `barDir`,
    /// `scatterStyle`, `axPos`, `legendPos` or `crosses` value outside its
    /// set; an `idx`, `order`, `ptCount`, `style`, `min`, `max` or `srgbClr`
    /// value that is not a number, an `axId` or `crossAx` that is not a
    /// decimal integer an `i64` holds (P1 ruling C), and a boolean `val`
    /// that is not one; a plot's `axId` naming no axis of the chart. "no such part" for a part the
    /// archive lacks; "the worksheet part's root is not worksheet"
    /// for any other part, a chartsheet included (P1 ruling B); "a
    /// worksheet has two drawing elements" and "a drawing part or its
    /// relationship cannot be read" as [`Package::images`](super::Package)
    /// raises them, and the latter for a chart's anchor whose `from` or
    /// `to` lacks a `col` or `row` number or whose `editAs` is outside its
    /// set; "an element or attribute uses a prefix its document does not
    /// bind"; [`Error::Xml`] for a chart part's bytes that are not
    /// well-formed XML or not UTF-8.
    pub fn charts(&mut self, part: &[u8]) -> Result<Vec<ChartEntry>, Error> {
        self.parts().charts(part)
    }
}

impl Parts<'_> {
    /// The body of [`Package::charts`].
    fn charts(&mut self, part: &[u8]) -> Result<Vec<ChartEntry>, Error> {
        let sheet = self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        let Some(target) = self.drawing_of(part, sheet)? else {
            return Ok(Vec::new());
        };
        let index = self.archive.index_of(&target).ok_or(DRAWING_UNREADABLE)?;
        let found = chart_frames(self.archive.read(index)?, index)?;
        let mut out = Vec::with_capacity(found.len());
        for (anchor, id, _) in found {
            let chart_part = {
                let rels = self.relationships(&target)?;
                let rel = rels
                    .by_id(id.as_bytes())
                    .filter(|rel| !rel.external && CHART.contains(&rel.kind.as_slice()))
                    .ok_or(CHART_UNREADABLE)?;
                rels.target_part(rel).map_err(|_| CHART_UNREADABLE)?
            };
            let at = self.archive.index_of(&chart_part).ok_or(CHART_UNREADABLE)?;
            let chart = read_chart(self.archive.read(at)?, at)?;
            out.push(ChartEntry {
                part: chart_part,
                anchor,
                chart,
            });
        }
        Ok(out)
    }
}

/// The namespaces the readers tell apart.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
    Chart,
    Chart2007,
    Main,
    Drawing,
    Compat,
    Other,
}

/// An element met: its namespace and local name, its unprefixed attributes
/// unescaped and normalised, its `r:id` (either form of the relationships
/// namespace), and whether it was an empty-element tag.
struct Node {
    space: Space,
    local: Vec<u8>,
    attributes: Vec<(Vec<u8>, String)>,
    relationship_id: Option<String>,
    empty: bool,
}

impl Node {
    fn of(start: &BytesStart<'_>, scope: &Scope, index: usize, empty: bool) -> Result<Self, Error> {
        let (namespace, local) = scope.resolve_element(start.name())?;
        let space = match namespace {
            Some(namespace) if CHART_NAMESPACE.contains(&namespace) => Space::Chart,
            Some(CHART_2007_NAMESPACE) => Space::Chart2007,
            Some(namespace) if DRAWINGML.contains(&namespace) => Space::Main,
            Some(namespace) if SPREADSHEET_DRAWING.contains(&namespace) => Space::Drawing,
            Some(namespace) if namespace == MARKUP_COMPATIBILITY => Space::Compat,
            _ => Space::Other,
        };
        let local = local.to_vec();
        let mut attributes = Vec::new();
        let mut relationship_id = None;
        for attribute in start.attributes() {
            let attribute = attribute.map_err(|_| Error::Xml(index))?;
            if attribute.key.as_namespace_binding().is_some() {
                continue;
            }
            let (namespace, name) = scope.resolve_attribute(attribute.key)?;
            let relationship = is_relationships_attribute(namespace) && name == b"id";
            if namespace.is_none() || relationship {
                let value = attribute
                    .normalized_value(XmlVersion::Implicit1_0)
                    .map_err(|_| Error::Xml(index))?
                    .into_owned();
                if relationship {
                    relationship_id = Some(value);
                } else {
                    attributes.push((name.to_vec(), value));
                }
            }
        }
        Ok(Self {
            space,
            local,
            attributes,
            relationship_id,
            empty,
        })
    }

    /// Whether this is the element `local` of `space`.
    fn is(&self, space: Space, local: &[u8]) -> bool {
        self.space == space && self.local == local
    }

    /// Whether this is the chart-namespace element `local`.
    fn chart(&self, local: &[u8]) -> bool {
        self.is(Space::Chart, local)
    }

    fn value(&self, name: &[u8]) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }

    /// `val` parsed as `T`; refused when absent or when it does not parse.
    fn val<T: std::str::FromStr>(&self) -> Result<T, Error> {
        self.value(b"val")
            .ok_or(CHART_UNREADABLE)?
            .parse()
            .map_err(|_| CHART_UNREADABLE)
    }

    /// A boolean element's `val`: true when absent, as the schema's default.
    fn flag(&self) -> Result<bool, Error> {
        match self.value(b"val") {
            None | Some("1" | "true") => Ok(true),
            Some("0" | "false") => Ok(false),
            Some(_) => Err(CHART_UNREADABLE),
        }
    }
}

/// The root element of a part, its start given to `scope`.
fn root<R: Read>(stream: &mut Stream<R>, scope: &mut Scope, index: usize) -> Result<Node, Error> {
    while let Some(token) = stream.next()? {
        if let Event::Start(ref start) | Event::Empty(ref start) = token.event {
            let empty = matches!(token.event, Event::Empty(_));
            scope.start(start)?;
            return Node::of(start, scope, index, empty);
        }
    }
    Err(Error::Xml(index))
}

/// Whether `node` has children to ask [`child`] for: an empty-element tag
/// has none, and is given back to `scope` here.
fn open(scope: &mut Scope, node: &Node) -> bool {
    if node.empty {
        scope.end();
    }
    !node.empty
}

/// The next child of the element whose start was given to `scope` last and
/// not yet ended, its start given to `scope` in turn; `None` at the parent's
/// end tag, which is consumed and given back to `scope`.
fn child<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
) -> Result<Option<Node>, Error> {
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start)?;
                return Node::of(start, scope, index, empty).map(Some);
            }
            Event::End(_) => {
                scope.end();
                return Ok(None);
            }
            _ => {}
        }
    }
    Err(Error::Xml(index))
}

/// Walks past `node`'s subtree, through its end tag.
fn past<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<(), Error> {
    if open(scope, node) {
        skip_subtree(stream, scope, index)
    } else {
        Ok(())
    }
}

/// Whether to read `node`'s children: when `wanted` is false its subtree
/// is walked past, and an empty-element tag has none to read; either way
/// `scope` is left as [`child`] expects.
fn enter<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
    wanted: bool,
) -> Result<bool, Error> {
    if wanted {
        Ok(open(scope, node))
    } else {
        past(stream, scope, index, node).map(|()| false)
    }
}

/// The text of the simple-content element `node`, through its end tag; an
/// element inside it is refused with `refusal`.
fn text<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
    refusal: Error,
) -> Result<String, Error> {
    if !open(scope, node) {
        return Ok(String::new());
    }
    // `text_of`'s own refusals (an element inside, a reference it cannot
    // expand) become `refusal`; a stream's error passes through.
    text_of(stream, scope, index).map_err(|error| match error {
        Error::Xml(_) | Error::Io(_) => error,
        _ => refusal,
    })
}

/// The chart anchors of a drawing part as [`read_anchors`] finds them, a
/// part that is not well-formed refused as one whose root is not `wsDr`:
/// what [`Package::charts`] lists and `Package::remove_chart` removes by
/// index, the latter by the root child's ordinal.
pub(super) fn chart_frames(
    input: impl Read,
    index: usize,
) -> Result<Vec<(ChartAnchor, String, usize)>, Error> {
    read_anchors(input, index).map_err(|error| match error {
        Error::Xml(_) => DRAWING_UNREADABLE,
        other => other,
    })
}

/// Whether a chart part's root is `chartSpace`, in either namespace; a part
/// that is not well-formed is not.
pub(super) fn is_chart_space(input: impl Read, index: usize) -> Result<bool, Error> {
    let mut stream = match Stream::new(input, index) {
        Ok(stream) => stream,
        Err(Error::Xml(_)) => return Ok(false),
        Err(other) => return Err(other),
    };
    let mut scope = Scope::new(index);
    match root(&mut stream, &mut scope, index) {
        Ok(node) => Ok(node.chart(b"chartSpace")),
        Err(Error::Xml(_)) => Ok(false),
        Err(other) => Err(other),
    }
}

/// The chart anchors of a drawing part, streamed once: each root anchor,
/// or the first anchor of a root `AlternateContent`'s first `Choice`,
/// whose `graphicFrame` holds a `c:chart`, with that element's `r:id` and
/// the ordinal of the root child it sits in. A root other than `wsDr`
/// cannot be read.
fn read_anchors(
    input: impl Read,
    index: usize,
) -> Result<Vec<(ChartAnchor, String, usize)>, Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let root = root(&mut stream, &mut scope, index)?;
    if !root.is(Space::Drawing, b"wsDr") {
        return Err(DRAWING_UNREADABLE);
    }
    let mut found = Vec::new();
    if !open(&mut scope, &root) {
        return Ok(found);
    }
    // The root children met, each chart taking its root's ordinal.
    let mut roots = 0usize;
    while let Some(node) = child(&mut stream, &mut scope, index)? {
        roots += 1;
        let ordinal = roots - 1;
        let with =
            |chart: Option<(ChartAnchor, String)>| chart.map(|(anchor, id)| (anchor, id, ordinal));
        if let Some(kind) = anchor_kind(&node) {
            found.extend(with(read_anchor(
                &mut stream,
                &mut scope,
                index,
                &node,
                kind,
            )?));
            continue;
        }
        let alternate = node.is(Space::Compat, b"AlternateContent");
        if !enter(&mut stream, &mut scope, index, &node, alternate)? {
            continue;
        }
        let mut choices = 0u32;
        while let Some(branch) = child(&mut stream, &mut scope, index)? {
            let choice = branch.is(Space::Compat, b"Choice");
            choices += u32::from(choice);
            if !enter(
                &mut stream,
                &mut scope,
                index,
                &branch,
                choice && choices == 1,
            )? {
                continue;
            }
            let mut taken = false;
            while let Some(inner) = child(&mut stream, &mut scope, index)? {
                match anchor_kind(&inner) {
                    Some(kind) if !taken => {
                        taken = true;
                        found.extend(with(read_anchor(
                            &mut stream,
                            &mut scope,
                            index,
                            &inner,
                            kind,
                        )?));
                    }
                    _ => past(&mut stream, &mut scope, index, &inner)?,
                }
            }
        }
    }
    Ok(found)
}

/// The anchor kind of a spreadsheet-drawing anchor element.
fn anchor_kind(node: &Node) -> Option<Anchor> {
    match (node.space, node.local.as_slice()) {
        (Space::Drawing, b"oneCellAnchor") => Some(Anchor::OneCell),
        (Space::Drawing, b"twoCellAnchor") => Some(Anchor::TwoCell),
        (Space::Drawing, b"absoluteAnchor") => Some(Anchor::Absolute),
        _ => None,
    }
}

/// Reads an anchor, its start given to `scope`, through its end tag: the
/// chart its `graphicFrame` holds with the anchor, or `None`.
fn read_anchor<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
    kind: Anchor,
) -> Result<Option<(ChartAnchor, String)>, Error> {
    let edit_as = node.value(b"editAs").map(str::to_owned);
    let mut from = None;
    let mut to = None;
    let mut id = None;
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if inner.is(Space::Drawing, b"from") {
                from = Some(read_marker(stream, scope, index, &inner)?);
            } else if inner.is(Space::Drawing, b"to") {
                to = Some(read_marker(stream, scope, index, &inner)?);
            } else if id.is_none() && inner.is(Space::Drawing, b"graphicFrame") {
                id = read_frame(stream, scope, index, &inner)?;
            } else {
                past(stream, scope, index, &inner)?;
            }
        }
    }
    let Some(id) = id else {
        return Ok(None);
    };
    let cell = |marker: Option<(Option<u32>, Option<u32>)>| match marker {
        Some((Some(column), Some(row))) => Ok((column, row)),
        _ => Err(DRAWING_UNREADABLE),
    };
    let (from, to, edit_as) = match kind {
        Anchor::Absolute => ((0, 0), None, None),
        Anchor::OneCell => (cell(from)?, None, None),
        Anchor::TwoCell => {
            let edit_as = match edit_as.as_deref() {
                None => None,
                Some("twoCell") => Some(EditAs::TwoCell),
                Some("oneCell") => Some(EditAs::OneCell),
                Some("absolute") => Some(EditAs::Absolute),
                Some(_) => return Err(DRAWING_UNREADABLE),
            };
            (cell(from)?, Some(cell(to)?), edit_as)
        }
    };
    Ok(Some((
        ChartAnchor {
            kind,
            from,
            to,
            edit_as,
        },
        id,
    )))
}

/// Reads a `from` or `to`, its start given to `scope`, through its end
/// tag: its `col` and `row` numbers, `None` for one absent or not a number.
fn read_marker<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<(Option<u32>, Option<u32>), Error> {
    let (mut column, mut row) = (None, None);
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            let field = if inner.is(Space::Drawing, b"col") {
                &mut column
            } else if inner.is(Space::Drawing, b"row") {
                &mut row
            } else {
                past(stream, scope, index, &inner)?;
                continue;
            };
            let value = text(stream, scope, index, &inner, DRAWING_UNREADABLE)?;
            *field =
                digits_value(value.trim().as_bytes()).and_then(|value| u32::try_from(value).ok());
        }
    }
    Ok((column, row))
}

/// Reads a `graphicFrame`, its start given to `scope`, through its end
/// tag: the `r:id` of the `c:chart` its `graphic/graphicData` holds, or
/// `None` for a frame holding no chart; a `c:chart` without `r:id` cannot
/// be read.
fn read_frame<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<Option<String>, Error> {
    let mut id = None;
    if !open(scope, node) {
        return Ok(id);
    }
    while let Some(graphic) = child(stream, scope, index)? {
        if !enter(
            stream,
            scope,
            index,
            &graphic,
            graphic.is(Space::Main, b"graphic"),
        )? {
            continue;
        }
        while let Some(data) = child(stream, scope, index)? {
            if !enter(
                stream,
                scope,
                index,
                &data,
                data.is(Space::Main, b"graphicData"),
            )? {
                continue;
            }
            while let Some(inner) = child(stream, scope, index)? {
                if id.is_none() && inner.chart(b"chart") {
                    id = Some(inner.relationship_id.clone().ok_or(CHART_UNREADABLE)?);
                }
                past(stream, scope, index, &inner)?;
            }
        }
    }
    Ok(id)
}

/// Reads a chart part.
fn read_chart(input: impl Read, index: usize) -> Result<Chart, Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let root = root(&mut stream, &mut scope, index)?;
    if !root.chart(b"chartSpace") {
        return Err(CHART_UNREADABLE);
    }
    let mut chart = None;
    let (mut pivot_source, mut style) = (None, None);
    if open(&mut scope, &root) {
        while let Some(node) = child(&mut stream, &mut scope, index)? {
            if chart.is_none() && node.chart(b"chart") {
                chart = Some(read_chart_element(&mut stream, &mut scope, index, &node)?);
            } else if node.chart(b"pivotSource") {
                pivot_source = Some(read_pivot_source(&mut stream, &mut scope, index, &node)?);
            } else if style.is_none() && node.chart(b"style") {
                style = Some(node.val()?);
                past(&mut stream, &mut scope, index, &node)?;
            } else if style.is_none() && node.is(Space::Compat, b"AlternateContent") {
                style = read_style_choice(&mut stream, &mut scope, index, &node)?;
            } else {
                past(&mut stream, &mut scope, index, &node)?;
            }
        }
    }
    let mut chart = chart.ok_or(CHART_UNREADABLE)?;
    chart.pivot_source = pivot_source;
    chart.style = style;
    Ok(chart)
}

/// Reads a `pivotSource`, its start given to `scope`, through its end tag:
/// its `name`'s text, which it must have (P2 ruling N).
fn read_pivot_source<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<String, Error> {
    let mut name = None;
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if inner.chart(b"name") {
                name = Some(text(stream, scope, index, &inner, CHART_UNREADABLE)?);
            } else {
                past(stream, scope, index, &inner)?;
            }
        }
    }
    name.ok_or(CHART_UNREADABLE)
}

/// Reads a root `mc:AlternateContent`, its start given to `scope`, through
/// its end tag: the `val` of the first `style` (either chart namespace or
/// the 2007 extension's) in its first `Choice`; the other branches and the
/// `Fallback` are walked past.
fn read_style_choice<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<Option<u32>, Error> {
    let mut style = None;
    let mut choices = 0u32;
    if !open(scope, node) {
        return Ok(style);
    }
    while let Some(branch) = child(stream, scope, index)? {
        let choice = branch.is(Space::Compat, b"Choice");
        choices += u32::from(choice);
        if !enter(stream, scope, index, &branch, choice && choices == 1)? {
            continue;
        }
        while let Some(inner) = child(stream, scope, index)? {
            if style.is_none() && (inner.chart(b"style") || inner.is(Space::Chart2007, b"style")) {
                style = Some(inner.val()?);
            }
            past(stream, scope, index, &inner)?;
        }
    }
    Ok(style)
}

/// Reads a `c:chart`, its start given to `scope`, through its end tag.
fn read_chart_element<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<Chart, Error> {
    let mut title = None;
    let mut auto_title_deleted = false;
    let mut plots = None;
    let mut legend = None;
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if inner.chart(b"title") {
                title = read_title(stream, scope, index, &inner)?;
            } else if inner.chart(b"autoTitleDeleted") {
                auto_title_deleted = inner.flag()?;
                past(stream, scope, index, &inner)?;
            } else if plots.is_none() && inner.chart(b"plotArea") {
                plots = Some(read_plot_area(stream, scope, index, &inner)?);
            } else if inner.chart(b"legend") {
                legend = Some(read_legend(stream, scope, index, &inner)?);
            } else {
                past(stream, scope, index, &inner)?;
            }
        }
    }
    let (plots, axes) = plots.ok_or(CHART_UNREADABLE)?;
    Ok(Chart {
        title,
        auto_title_deleted,
        plots,
        axes,
        legend,
        pivot_source: None,
        style: None,
    })
}

/// Reads a `legend`, its start given to `scope`, through its end tag: its
/// `legendPos`, `Right` without one.
fn read_legend<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<LegendPosition, Error> {
    let mut position = LegendPosition::Right;
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if inner.chart(b"legendPos") {
                position = match inner.value(b"val") {
                    Some("b") => LegendPosition::Bottom,
                    Some("l") => LegendPosition::Left,
                    None | Some("r") => LegendPosition::Right,
                    Some("t") => LegendPosition::Top,
                    Some("tr") => LegendPosition::TopRight,
                    Some(_) => return Err(CHART_UNREADABLE),
                };
            }
            past(stream, scope, index, &inner)?;
        }
    }
    Ok(position)
}

/// Reads a `title`, its start given to `scope`, through its end tag: its
/// text as [`Chart::title`] says.
fn read_title<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<Option<String>, Error> {
    let mut title = None;
    if !open(scope, node) {
        return Ok(title);
    }
    while let Some(inner) = child(stream, scope, index)? {
        if !enter(stream, scope, index, &inner, inner.chart(b"tx"))? {
            continue;
        }
        while let Some(form) = child(stream, scope, index)? {
            if form.chart(b"rich") {
                title = Some(read_rich(stream, scope, index, &form)?);
            } else if form.chart(b"strRef") {
                title = read_first_point(stream, scope, index, &form)?;
            } else {
                past(stream, scope, index, &form)?;
            }
        }
    }
    Ok(title)
}

/// Reads a `rich`, its start given to `scope`, through its end tag: the
/// `a:t` text of each paragraph's runs, the paragraphs joined by a newline.
fn read_rich<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<String, Error> {
    let mut paragraphs: Vec<String> = Vec::new();
    if !open(scope, node) {
        return Ok(String::new());
    }
    while let Some(paragraph) = child(stream, scope, index)? {
        if !paragraph.is(Space::Main, b"p") {
            past(stream, scope, index, &paragraph)?;
            continue;
        }
        let mut line = String::new();
        if open(scope, &paragraph) {
            while let Some(run) = child(stream, scope, index)? {
                if !enter(stream, scope, index, &run, run.is(Space::Main, b"r"))? {
                    continue;
                }
                while let Some(inner) = child(stream, scope, index)? {
                    if inner.is(Space::Main, b"t") {
                        line.push_str(&text(stream, scope, index, &inner, CHART_UNREADABLE)?);
                    } else {
                        past(stream, scope, index, &inner)?;
                    }
                }
            }
        }
        paragraphs.push(line);
    }
    Ok(paragraphs.join("\n"))
}

/// Reads a title's `strRef`, its start given to `scope`, through its end
/// tag: the text of its `strCache`'s first `pt`, or `None`.
fn read_first_point<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<Option<String>, Error> {
    let mut point = None;
    if !open(scope, node) {
        return Ok(point);
    }
    while let Some(cache) = child(stream, scope, index)? {
        if !enter(stream, scope, index, &cache, cache.chart(b"strCache"))? {
            continue;
        }
        while let Some(pt) = child(stream, scope, index)? {
            if !enter(
                stream,
                scope,
                index,
                &pt,
                point.is_none() && pt.chart(b"pt"),
            )? {
                continue;
            }
            let mut value = String::new();
            while let Some(inner) = child(stream, scope, index)? {
                if inner.chart(b"v") {
                    value = text(stream, scope, index, &inner, CHART_UNREADABLE)?;
                } else {
                    past(stream, scope, index, &inner)?;
                }
            }
            point = Some(value);
        }
    }
    Ok(point)
}

/// The plot kind of a `plotArea` child.
fn plot_kind(node: &Node) -> Option<ChartKind> {
    if node.space != Space::Chart {
        return None;
    }
    Some(match node.local.as_slice() {
        b"barChart" => ChartKind::Bar,
        b"bar3DChart" => ChartKind::Bar3D,
        b"lineChart" => ChartKind::Line,
        b"line3DChart" => ChartKind::Line3D,
        b"pieChart" => ChartKind::Pie,
        b"pie3DChart" => ChartKind::Pie3D,
        b"doughnutChart" => ChartKind::Doughnut,
        b"ofPieChart" => ChartKind::OfPie,
        b"scatterChart" => ChartKind::Scatter,
        b"areaChart" => ChartKind::Area,
        b"area3DChart" => ChartKind::Area3D,
        b"radarChart" => ChartKind::Radar,
        b"bubbleChart" => ChartKind::Bubble,
        b"stockChart" => ChartKind::Stock,
        b"surfaceChart" => ChartKind::Surface,
        b"surface3DChart" => ChartKind::Surface3D,
        _ => return None,
    })
}

/// The axis kind of a `plotArea` child.
fn axis_kind(node: &Node) -> Option<AxisKind> {
    if node.space != Space::Chart {
        return None;
    }
    Some(match node.local.as_slice() {
        b"catAx" => AxisKind::Category,
        b"valAx" => AxisKind::Value,
        b"dateAx" => AxisKind::Date,
        b"serAx" => AxisKind::Series,
        _ => return None,
    })
}

/// Reads a `plotArea`, its start given to `scope`, through its end tag: its
/// plots and its axes, every other child walked past; a plot's `axId`
/// naming no axis cannot be read.
fn read_plot_area<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<(Vec<Plot>, Vec<ChartAxis>), Error> {
    let mut plots = Vec::new();
    let mut axes = Vec::new();
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if let Some(kind) = plot_kind(&inner) {
                plots.push(read_plot(stream, scope, index, &inner, kind)?);
            } else if let Some(kind) = axis_kind(&inner) {
                axes.push(read_axis(stream, scope, index, &inner, kind)?);
            } else {
                past(stream, scope, index, &inner)?;
            }
        }
    }
    let joined = plots
        .iter()
        .flat_map(|plot: &Plot| plot.axis_ids.iter())
        .all(|id| axes.iter().any(|axis: &ChartAxis| axis.id == *id));
    if !joined {
        return Err(CHART_UNREADABLE);
    }
    Ok((plots, axes))
}

/// Reads an axis, its start given to `scope`, through its end tag; one with
/// no `axId`, `axPos` or `crossAx` cannot be read.
fn read_axis<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
    kind: AxisKind,
) -> Result<ChartAxis, Error> {
    let (mut id, mut position, mut crosses) = (None, None, None);
    let mut axis = ChartAxis {
        id: 0,
        kind,
        position: AxisPosition::Bottom,
        crosses: 0,
        title: None,
        number_format: None,
        min: None,
        max: None,
        deleted: false,
        major_gridlines: false,
    };
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if inner.chart(b"axId") {
                id = Some(inner.val()?);
            } else if inner.chart(b"crossAx") {
                crosses = Some(inner.val()?);
            } else if inner.chart(b"axPos") {
                position = Some(match inner.value(b"val") {
                    Some("b") => AxisPosition::Bottom,
                    Some("l") => AxisPosition::Left,
                    Some("r") => AxisPosition::Right,
                    Some("t") => AxisPosition::Top,
                    _ => return Err(CHART_UNREADABLE),
                });
            } else if inner.chart(b"crosses") {
                // A `crosses` with no `val` is refused (P2 ruling N).
                if !matches!(inner.value(b"val"), Some("autoZero" | "max" | "min")) {
                    return Err(CHART_UNREADABLE);
                }
            } else if inner.chart(b"delete") {
                axis.deleted = inner.flag()?;
            } else if inner.chart(b"majorGridlines") {
                axis.major_gridlines = true;
            } else if inner.chart(b"numFmt") {
                // A `numFmt` with no `formatCode` is refused (P2 ruling N).
                let format = inner.value(b"formatCode").ok_or(CHART_UNREADABLE)?;
                axis.number_format = Some(format.to_owned());
            } else if inner.chart(b"title") {
                axis.title = read_title(stream, scope, index, &inner)?;
                continue;
            } else if inner.chart(b"scaling") {
                (axis.min, axis.max) = read_scaling(stream, scope, index, &inner)?;
                continue;
            }
            past(stream, scope, index, &inner)?;
        }
    }
    axis.id = id.ok_or(CHART_UNREADABLE)?;
    axis.position = position.ok_or(CHART_UNREADABLE)?;
    axis.crosses = crosses.ok_or(CHART_UNREADABLE)?;
    Ok(axis)
}

/// Reads an axis's `scaling`, its start given to `scope`, through its end
/// tag: its `min` and `max` values.
fn read_scaling<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<(Option<f64>, Option<f64>), Error> {
    let (mut min, mut max) = (None, None);
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if inner.chart(b"min") {
                min = Some(inner.val()?);
            } else if inner.chart(b"max") {
                max = Some(inner.val()?);
            }
            past(stream, scope, index, &inner)?;
        }
    }
    Ok((min, max))
}

/// Reads a plot, its start given to `scope`, through its end tag.
fn read_plot<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
    kind: ChartKind,
) -> Result<Plot, Error> {
    let bar = matches!(kind, ChartKind::Bar | ChartKind::Bar3D);
    let grouped = bar
        || matches!(
            kind,
            ChartKind::Line | ChartKind::Line3D | ChartKind::Area | ChartKind::Area3D
        );
    let mut plot = Plot {
        kind,
        bar_direction: None,
        grouping: None,
        scatter_style: None,
        vary_colors: false,
        series: Vec::new(),
        axis_ids: Vec::new(),
    };
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if bar && inner.chart(b"barDir") {
                plot.bar_direction = Some(match inner.value(b"val") {
                    Some("bar") => BarDirection::Bar,
                    None | Some("col") => BarDirection::Column,
                    Some(_) => return Err(CHART_UNREADABLE),
                });
            } else if grouped && inner.chart(b"grouping") {
                plot.grouping = Some(match inner.value(b"val") {
                    None if bar => Grouping::Clustered,
                    None | Some("standard") => Grouping::Standard,
                    Some("clustered") => Grouping::Clustered,
                    Some("stacked") => Grouping::Stacked,
                    Some("percentStacked") => Grouping::PercentStacked,
                    Some(_) => return Err(CHART_UNREADABLE),
                });
            } else if kind == ChartKind::Scatter && inner.chart(b"scatterStyle") {
                plot.scatter_style = Some(match inner.value(b"val") {
                    Some("none") => ScatterStyle::None,
                    Some("line") => ScatterStyle::Line,
                    Some("lineMarker") => ScatterStyle::LineMarker,
                    None | Some("marker") => ScatterStyle::Marker,
                    Some("smooth") => ScatterStyle::Smooth,
                    Some("smoothMarker") => ScatterStyle::SmoothMarker,
                    Some(_) => return Err(CHART_UNREADABLE),
                });
            } else if inner.chart(b"varyColors") {
                plot.vary_colors = inner.flag()?;
            } else if inner.chart(b"axId") {
                plot.axis_ids.push(inner.val()?);
            } else if inner.chart(b"ser") {
                plot.series.push(read_series(stream, scope, index, &inner)?);
                continue;
            }
            past(stream, scope, index, &inner)?;
        }
    }
    Ok(plot)
}

/// Reads a `ser`, its start given to `scope`, through its end tag.
fn read_series<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<Series, Error> {
    let (mut index_value, mut order) = (None, None);
    let mut series = Series {
        index: 0,
        order: 0,
        name: None,
        categories: None,
        values: None,
        sizes: None,
        fill: None,
        smooth: false,
        marker: false,
        labels: false,
    };
    let (mut cat, mut x_val) = (false, false);
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if inner.chart(b"idx") {
                index_value = Some(inner.val()?);
            } else if inner.chart(b"order") {
                order = Some(inner.val()?);
            } else if inner.chart(b"tx") {
                series.name = read_series_name(stream, scope, index, &inner)?;
                continue;
            } else if inner.chart(b"spPr") {
                series.fill = read_fill(stream, scope, index, &inner)?;
                continue;
            } else if inner.chart(b"cat") || inner.chart(b"xVal") {
                cat |= inner.chart(b"cat");
                x_val |= inner.chart(b"xVal");
                series.categories = read_data(stream, scope, index, &inner)?;
                continue;
            } else if inner.chart(b"val") || inner.chart(b"yVal") {
                series.values = read_data(stream, scope, index, &inner)?;
                continue;
            } else if inner.chart(b"bubbleSize") {
                series.sizes = read_data(stream, scope, index, &inner)?;
                continue;
            } else if inner.chart(b"smooth") {
                series.smooth = inner.flag()?;
            } else if inner.chart(b"marker") {
                series.marker = true;
            } else if inner.chart(b"dLbls") {
                series.labels = true;
            }
            past(stream, scope, index, &inner)?;
        }
    }
    if cat && x_val {
        return Err(CHART_UNREADABLE);
    }
    series.index = index_value.ok_or(CHART_UNREADABLE)?;
    series.order = order.ok_or(CHART_UNREADABLE)?;
    Ok(series)
}

/// Reads a series' `tx`, its start given to `scope`, through its end tag.
fn read_series_name<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<Option<SeriesName>, Error> {
    let mut name = None;
    if !open(scope, node) {
        return Ok(name);
    }
    while let Some(inner) = child(stream, scope, index)? {
        if inner.chart(b"strRef") {
            let (formula, _) = read_reference(stream, scope, index, &inner)?;
            name = Some(SeriesName::Reference(formula));
        } else if inner.chart(b"v") {
            name = Some(SeriesName::Text(text(
                stream,
                scope,
                index,
                &inner,
                CHART_UNREADABLE,
            )?));
        } else {
            past(stream, scope, index, &inner)?;
        }
    }
    Ok(name)
}

/// Reads a series' `spPr`, its start given to `scope`, through its end
/// tag: its `solidFill`'s `srgbClr` `val` as a number, or `None`.
fn read_fill<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<Option<u32>, Error> {
    let mut fill = None;
    if !open(scope, node) {
        return Ok(fill);
    }
    while let Some(inner) = child(stream, scope, index)? {
        if !enter(
            stream,
            scope,
            index,
            &inner,
            inner.is(Space::Main, b"solidFill"),
        )? {
            continue;
        }
        while let Some(colour) = child(stream, scope, index)? {
            if colour.is(Space::Main, b"srgbClr") {
                let value = colour.value(b"val").ok_or(CHART_UNREADABLE)?;
                if value.len() != 6 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                    return Err(CHART_UNREADABLE);
                }
                fill = u32::from_str_radix(value, 16).ok();
            }
            past(stream, scope, index, &colour)?;
        }
    }
    Ok(fill)
}

/// Reads a series' `cat`, `val`, `xVal`, `yVal` or `bubbleSize`, its start
/// given to `scope`, through its end tag.
fn read_data<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<Option<Data>, Error> {
    let mut data = None;
    if !open(scope, node) {
        return Ok(data);
    }
    while let Some(inner) = child(stream, scope, index)? {
        if inner.chart(b"numRef") || inner.chart(b"strRef") || inner.chart(b"multiLvlStrRef") {
            let (formula, cached) = read_reference(stream, scope, index, &inner)?;
            data = Some(Data::Reference { formula, cached });
        } else if inner.chart(b"numLit") || inner.chart(b"strLit") {
            data = Some(Data::Literal {
                count: read_point_count(stream, scope, index, &inner)?,
            });
        } else {
            past(stream, scope, index, &inner)?;
        }
    }
    Ok(data)
}

/// Reads a reference form, its start given to `scope`, through its end
/// tag: its `f`, which it must have, and its cache's point count.
fn read_reference<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<(String, Option<u64>), Error> {
    let mut formula = None;
    let mut cached = None;
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if inner.chart(b"f") {
                formula = Some(text(stream, scope, index, &inner, CHART_UNREADABLE)?);
            } else if inner.chart(b"numCache")
                || inner.chart(b"strCache")
                || inner.chart(b"multiLvlStrCache")
            {
                cached = Some(read_point_count(stream, scope, index, &inner)?);
            } else {
                past(stream, scope, index, &inner)?;
            }
        }
    }
    Ok((formula.ok_or(CHART_UNREADABLE)?, cached))
}

/// Reads a cache or a literal, its start given to `scope`, through its end
/// tag: its `ptCount` `val`, 0 when absent; the points are walked past.
fn read_point_count<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    node: &Node,
) -> Result<u64, Error> {
    let mut count = 0;
    if open(scope, node) {
        while let Some(inner) = child(stream, scope, index)? {
            if inner.chart(b"ptCount") {
                count = inner.val()?;
            }
            past(stream, scope, index, &inner)?;
        }
    }
    Ok(count)
}
