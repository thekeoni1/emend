//! A kept chart edited in place (`specs/0041-chart-text-and-edits.md`,
//! "The chart part", rulings E and F). The Cargo feature `charts`.
//!
//! [`Package::set_chart_title`] and [`Package::set_chart_series`] check the
//! texts and references at the call and record the edit; the write resolves
//! the chart [`Package::charts`] lists at the index through its frame, as
//! `Package::remove_chart` resolves one, reads the chart part whole and
//! rewrites it: every byte copied but the elements the edits replace,
//! remove or insert, the part's prefixes and declaration kept as found.
//! Its one message, "no series at that position", is a chart writer's
//! constant, catalogued with the rest in the `package` module's header.

use std::io::{Cursor, Read, Seek};

use quick_xml::events::{BytesStart, Event};
use quick_xml::name::PrefixDeclaration;

#[cfg(feature = "rowcol")]
use super::SHIFT_ALONE;
use super::chart::{self, CHART_NAMESPACE};
use super::chart_write::{
    CHART_PART_UNREADABLE, NO_CHART_AT, NO_SERIES_AT, NewSeriesName, reference, text,
};
use super::{
    CELL_EDIT_REPLACED, CHART, DRAWING_UNREADABLE, DRAWINGML, NO_SUCH_PART, Package, Parts,
    REMOVED_PART_EDITED, Removal,
};
use crate::archive::{Error, replacement_from_reader};
use crate::xml::{Scope, Stream};

/// A kept series' references to replace (`specs/0041-chart-text-and-
/// edits.md`, "Types"): each `Some` replaces that reference of the series,
/// written with its `f` alone and no cache; `None` leaves it. The Cargo
/// feature `charts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ChartSeriesEdit<'a> {
    /// The series' name, `tx`.
    pub name: Option<NewSeriesName<'a>>,
    /// The categories, `cat`, or a scatter's or a bubble's x values, `xVal`.
    pub categories: Option<&'a str>,
    /// The values, `val`, or a scatter's or a bubble's y values, `yVal`.
    pub values: Option<&'a str>,
}

/// An edit as the call checked it: the title's escaped text, or none to
/// remove it; or a series' position and its escaped references.
#[derive(Clone)]
pub(super) enum ChartEdit {
    Title(Option<String>),
    Series(usize, SeriesEdit),
}

/// A series edit as the call checked it.
#[derive(Clone)]
pub(super) struct SeriesEdit {
    name: Option<Name>,
    categories: Option<String>,
    values: Option<String>,
}

/// A series' new name, escaped.
#[derive(Clone)]
enum Name {
    Reference(String),
    Text(String),
}

impl ChartEdit {
    /// Whether `self` and `other` edit the same thing of one chart: its
    /// title, or the same series (ruling E, Q2).
    fn same_target(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Title(_), Self::Title(_)) => true,
            (Self::Series(one, _), Self::Series(two, _)) => one == two,
            _ => false,
        }
    }
}

impl<R: Read + Seek> Package<R> {
    /// Records the title of the chart [`Package::charts`] lists at `index`
    /// for the worksheet part `part` set to `title`, or removed for `None`
    /// (`specs/0041-chart-text-and-edits.md`, "Types", "The chart part").
    /// The Cargo feature `charts`.
    ///
    /// The write rewrites the chart part in place. A kept rich title keeps
    /// its `bodyPr`, its `lstStyle`, its first paragraph's `pPr` and its
    /// first run's `rPr`, the run's `t` alone replaced, further runs and
    /// paragraphs dropped; a title whose `tx` holds a reference, or rich
    /// text with no run, takes the writer's rich text; the title's other
    /// children are kept byte for byte, and a title with no `tx` takes one
    /// as its first child (ruling E). A chart with no title takes the
    /// writer's as `c:chart`'s first child, an `autoTitleDeleted` of `1`
    /// becoming `0`. `None` removes the title and sets `autoTitleDeleted`
    /// to `1`, inserting it where it is absent.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: "a chart text is
    /// empty or holds a control character" for a title that is empty,
    /// holds a character below U+0020, U+FFFE or U+FFFF, or is longer than
    /// 32767 bytes. The write refuses, part by part in record order: an
    /// index edited twice, removed in the same write, or on a part this
    /// write adds an image or a chart to, "no chart at that position" (the
    /// caller writes twice); what `Package::remove_chart` refuses of the
    /// part, a chartsheet with "the worksheet part's root is not
    /// worksheet" (ruling E, Q4); an index past the list, "no chart at
    /// that position"; a frame whose relationship names no chart part, a
    /// part whose root is not `chartSpace`, or one the walk cannot read,
    /// "a chart part cannot be read".
    pub fn set_chart_title(
        &mut self,
        part: &[u8],
        index: usize,
        title: Option<&str>,
    ) -> Result<(), Error> {
        self.parts().set_chart_title(part, index, title)
    }

    /// Records the series at `series` of the chart [`Package::charts`]
    /// lists at `index` for the worksheet part `part` edited by `edit`
    /// (`specs/0041-chart-text-and-edits.md`, "Types", "The chart part").
    /// The Cargo feature `charts`.
    ///
    /// `series` counts the `ser` elements of the plot area in document
    /// order across its plots, from 0. The write rewrites the chart part in
    /// place: the series' `tx` replaced by the writer's form for the name,
    /// or inserted after `order`; its `cat` or `xVal` and its `val` or
    /// `yVal` (by the plot: a scatter's and a bubble's take `xVal` and
    /// `yVal`) replaced by a `numRef` or `strRef` holding the `f` alone, no
    /// cache, the form found kept and the writer's taken where the element
    /// holds another or is absent; an absent element inserted where the
    /// schema places it. Every other byte is kept.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: "a chart text is
    /// empty or holds a control character" for a name, then the categories,
    /// then the values, that is empty, holds a character below U+0020,
    /// U+FFFE or U+FFFF, or is longer than its limit (a reference, 8192
    /// bytes; a text name, 32767). The write refuses what
    /// [`Package::set_chart_title`] says it refuses, an index's same series
    /// edited twice among them, then a `series` past the plot area's count,
    /// "no series at that position"; a series under an element other than
    /// the plots the writer knows, "a chart part cannot be read".
    pub fn set_chart_series(
        &mut self,
        part: &[u8],
        index: usize,
        series: usize,
        edit: &ChartSeriesEdit<'_>,
    ) -> Result<(), Error> {
        self.parts().set_chart_series(part, index, series, edit)
    }
}

impl Parts<'_> {
    /// The body of [`Package::set_chart_title`].
    fn set_chart_title(
        &mut self,
        part: &[u8],
        index: usize,
        title: Option<&str>,
    ) -> Result<(), Error> {
        let title = title.map(text).transpose()?;
        self.edits
            .chart_edits
            .push((part.to_vec(), index, ChartEdit::Title(title)));
        Ok(())
    }

    /// The body of [`Package::set_chart_series`]: the name, the categories
    /// and the values checked in field order.
    fn set_chart_series(
        &mut self,
        part: &[u8],
        index: usize,
        series: usize,
        edit: &ChartSeriesEdit<'_>,
    ) -> Result<(), Error> {
        let name = match edit.name {
            None => None,
            Some(NewSeriesName::Reference(formula)) => Some(Name::Reference(reference(formula)?)),
            Some(NewSeriesName::Text(name)) => Some(Name::Text(text(name)?)),
        };
        let work = SeriesEdit {
            name,
            categories: edit.categories.map(reference).transpose()?,
            values: edit.values.map(reference).transpose()?,
        };
        self.edits
            .chart_edits
            .push((part.to_vec(), index, ChartEdit::Series(series, work)));
        Ok(())
    }

    /// Records each edited chart part's rewrite (ruling E, Q1): part by part
    /// in record order, an edit of a chart edited twice, removed or on a
    /// part this write adds to; then the part's refusals as a removal's;
    /// then each index past the list; then each frame's chart part; then,
    /// chart by chart, the walk (a series past the count, then what it
    /// cannot read).
    pub(super) fn record_chart_edits(&mut self) -> Result<(), Error> {
        if self.edits.chart_edits.is_empty() {
            return Ok(());
        }
        type PartEdits = Vec<(usize, ChartEdit)>;
        let mut parts: Vec<(Vec<u8>, PartEdits)> = Vec::new();
        for (part, index, edit) in self.edits.chart_edits.clone() {
            let at = match parts.iter().position(|(have, _)| *have == part) {
                Some(at) => at,
                None => {
                    parts.push((part, Vec::new()));
                    parts.len() - 1
                }
            };
            let (part, edits) = &mut parts[at];
            let twice = edits
                .iter()
                .any(|(have, other)| *have == index && other.same_target(&edit));
            let removed = self
                .edits
                .drawing_removals
                .iter()
                .any(|(at, removal)| at == part && *removal == Removal::Chart(index));
            let added = self.edits.drawing_edits.iter().any(|(at, _, _)| at == part);
            if twice || removed || added {
                return Err(NO_CHART_AT);
            }
            edits.push((index, edit));
        }
        let mut rewritten: Vec<Vec<u8>> = Vec::new();
        for (part, edits) in parts {
            if self.edits.part_removals.contains(&part) {
                return Err(REMOVED_PART_EDITED);
            }
            if self
                .edits
                .part_replacements
                .iter()
                .any(|(name, _)| *name == part)
            {
                return Err(CELL_EDIT_REPLACED);
            }
            let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
            #[cfg(feature = "rowcol")]
            if self.edits.shifts.iter().any(|(at, _)| *at == part) {
                return Err(SHIFT_ALONE);
            }
            // A chartsheet is refused as the cell calls refuse it: its chart
            // is not edited in place (ruling E, Q4).
            let (drawing, _) = self.sheet_drawing_of(&part, index, false)?;
            self.edited_part_untouched(drawing.as_deref())?;
            let Some(target) = drawing else {
                return Err(NO_CHART_AT);
            };
            let at = self.archive.index_of(&target).ok_or(DRAWING_UNREADABLE)?;
            let frames = chart::chart_frames(self.archive.read(at)?, at)?;
            // The charts in the order the edits first name them.
            let mut charts: Vec<(usize, Vec<ChartEdit>)> = Vec::new();
            for (index, edit) in edits {
                match charts.iter_mut().find(|(have, _)| *have == index) {
                    Some((_, list)) => list.push(edit),
                    None => charts.push((index, vec![edit])),
                }
            }
            if charts.iter().any(|(index, _)| *index >= frames.len()) {
                return Err(NO_CHART_AT);
            }
            let mut resolved: Vec<(Vec<u8>, usize, Vec<ChartEdit>)> = Vec::new();
            for (index, list) in charts {
                let (_, id, _) = &frames[index];
                let chart_part = {
                    let rels = self.relationships(&target)?;
                    let rel = rels
                        .by_id(id.as_bytes())
                        .filter(|rel| !rel.external && CHART.contains(&rel.kind.as_slice()))
                        .ok_or(CHART_PART_UNREADABLE)?;
                    rels.target_part(rel).map_err(|_| CHART_PART_UNREADABLE)?
                };
                // Two frames naming one part, both edited: the caller writes
                // twice.
                if rewritten.contains(&chart_part)
                    || resolved.iter().any(|(have, _, _)| *have == chart_part)
                {
                    return Err(NO_CHART_AT);
                }
                self.edited_part_untouched(Some(&chart_part))?;
                let entry = self
                    .archive
                    .index_of(&chart_part)
                    .ok_or(CHART_PART_UNREADABLE)?;
                if !chart::is_chart_space(self.archive.read(entry)?, entry)? {
                    return Err(CHART_PART_UNREADABLE);
                }
                resolved.push((chart_part, entry, list));
            }
            for (chart_part, entry, list) in resolved {
                let mut bytes = Vec::new();
                self.archive.read(entry)?.read_to_end(&mut bytes)?;
                let out = edit_chart(&bytes, entry, &list)?;
                self.replace_part(&chart_part, replacement_from_reader(Cursor::new(out)));
                rewritten.push(chart_part);
            }
        }
        Ok(())
    }
}

/// The namespaces the walk tells apart.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
    Chart,
    Drawing,
    Other,
}

/// An element of the chart part: its namespace, local name and prefix (with
/// its colon, or empty), its `val` attribute, and its byte ranges: the
/// start tag `start..open`, the end tag `close..end` (an empty-element
/// tag's `open`, `close` and `end` the same). Children are kept only on the
/// paths an edit walks.
struct Element {
    space: Space,
    local: Vec<u8>,
    prefix: Vec<u8>,
    val: Option<Vec<u8>>,
    start: usize,
    open: usize,
    close: usize,
    end: usize,
    empty: bool,
    children: Vec<Element>,
}

impl Element {
    fn is(&self, space: Space, local: &[u8]) -> bool {
        self.space == space && self.local == local
    }

    fn chart(&self, local: &[u8]) -> bool {
        self.is(Space::Chart, local)
    }

    /// The first chart-namespace child named `local`.
    fn child(&self, local: &[u8]) -> Option<&Element> {
        self.children.iter().find(|child| child.chart(local))
    }

    /// Whether the walk keeps this element's children, given its parent's
    /// namespace and name.
    fn walked(&self, parent: Option<(Space, &[u8])>) -> bool {
        match parent {
            None => true,
            Some((Space::Chart, b"chartSpace")) => self.chart(b"chart"),
            Some((Space::Chart, b"chart")) => {
                self.chart(b"title") || self.chart(b"autoTitleDeleted") || self.chart(b"plotArea")
            }
            Some((Space::Chart, b"title")) => self.chart(b"tx"),
            Some((Space::Chart, b"tx")) => self.chart(b"rich"),
            Some((Space::Chart, b"rich")) => self.is(Space::Drawing, b"p"),
            // Every child of a paragraph, those a title edit drops among
            // them, and a run's text.
            Some((Space::Drawing, b"p")) => true,
            Some((Space::Drawing, b"r")) => self.is(Space::Drawing, b"t"),
            // The plots, their series, the series' children, and a
            // reference's form.
            Some((Space::Chart, b"plotArea")) => self.space == Space::Chart,
            Some((Space::Chart, b"ser")) => true,
            Some((Space::Chart, b"cat" | b"val" | b"xVal" | b"yVal")) => {
                self.chart(b"numRef") || self.chart(b"strRef")
            }
            Some((Space::Chart, _)) => self.chart(b"ser"),
            _ => false,
        }
    }
}

/// The plots the writer knows, whose `ser` elements an edit counts
/// (ruling E, Q5); a scatter's and a bubble's take `xVal` and `yVal`.
const PLOTS: [&[u8]; 16] = [
    b"barChart",
    b"bar3DChart",
    b"lineChart",
    b"line3DChart",
    b"pieChart",
    b"pie3DChart",
    b"doughnutChart",
    b"ofPieChart",
    b"areaChart",
    b"area3DChart",
    b"scatterChart",
    b"bubbleChart",
    b"radarChart",
    b"stockChart",
    b"surfaceChart",
    b"surface3DChart",
];

/// A series' children after its values in the schema, any kind's: where an
/// absent reference goes before.
const TRAILING: [&[u8]; 5] = [b"shape", b"smooth", b"bubbleSize", b"bubble3D", b"extLst"];

/// The chart part's elements as the walk keeps them, the root returned,
/// and the prefix the root binds to DrawingML (with its colon) with
/// whether the chart namespace is the strict one. A part that is not
/// well-formed cannot be read.
fn read_tree(bytes: &[u8], index: usize) -> Result<(Element, Option<Vec<u8>>, bool), Error> {
    let unreadable = |error: Error| match error {
        Error::Xml(_) => CHART_PART_UNREADABLE,
        other => other,
    };
    let mut stream = Stream::new(Cursor::new(bytes), index).map_err(unreadable)?;
    let mut scope = Scope::new(index);
    let mut at: usize = if stream.bom() { 3 } else { 0 };
    // Each open element with whether its children are kept.
    let mut stack: Vec<(Element, bool)> = Vec::new();
    let mut root: Option<Element> = None;
    let mut drawing: Option<Vec<u8>> = None;
    let mut strict = false;
    while let Some(token) = stream.next().map_err(unreadable)? {
        let from = at;
        at = at
            .checked_add(token.raw.len())
            .ok_or(CHART_PART_UNREADABLE)?;
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start).map_err(unreadable)?;
                let element =
                    element_of(start, &scope, from, at, empty, index).map_err(unreadable)?;
                if stack.is_empty() && root.is_none() {
                    (drawing, strict) = root_bindings(start, index).map_err(unreadable)?;
                }
                let keep = match stack.last() {
                    None => true,
                    Some((parent, kept)) => {
                        *kept && element.walked(Some((parent.space, &parent.local)))
                    }
                };
                if empty {
                    scope.end();
                    finish(&mut stack, &mut root, element);
                } else {
                    stack.push((element, keep));
                }
            }
            Event::End(_) => {
                scope.end();
                let (mut element, _) = stack.pop().ok_or(CHART_PART_UNREADABLE)?;
                element.close = from;
                element.end = at;
                finish(&mut stack, &mut root, element);
            }
            _ => {}
        }
    }
    let root = root.ok_or(CHART_PART_UNREADABLE)?;
    if !stack.is_empty() || !root.chart(b"chartSpace") {
        return Err(CHART_PART_UNREADABLE);
    }
    Ok((root, drawing, strict))
}

/// Hands a finished element to its parent where the parent keeps its
/// children, or makes it the root.
fn finish(stack: &mut [(Element, bool)], root: &mut Option<Element>, element: Element) {
    match stack.last_mut() {
        Some((parent, kept)) => {
            if *kept && element.walked(Some((parent.space, &parent.local))) {
                parent.children.push(element);
            }
        }
        None => *root = Some(element),
    }
}

/// An element met at `from..at`.
fn element_of(
    start: &BytesStart<'_>,
    scope: &Scope,
    from: usize,
    at: usize,
    empty: bool,
    index: usize,
) -> Result<Element, Error> {
    let (namespace, local) = scope.resolve_element(start.name())?;
    let space = match namespace {
        Some(namespace) if CHART_NAMESPACE.contains(&namespace) => Space::Chart,
        Some(namespace) if DRAWINGML.contains(&namespace) => Space::Drawing,
        _ => Space::Other,
    };
    let local = local.to_vec();
    let prefix = match start.name().prefix() {
        Some(prefix) => [prefix.into_inner().as_bytes(), b":"].concat(),
        None => Vec::new(),
    };
    let mut val = None;
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_ref() == "val" {
            val = Some(attribute.value.as_bytes().to_vec());
        }
    }
    Ok(Element {
        space,
        local,
        prefix,
        val,
        start: from,
        open: at,
        close: at,
        end: at,
        empty,
        children: Vec::new(),
    })
}

/// The prefix the root's own declarations bind to DrawingML, with its
/// colon (empty for the default namespace), and whether the chart
/// namespace it declares is the strict one.
fn root_bindings(start: &BytesStart<'_>, index: usize) -> Result<(Option<Vec<u8>>, bool), Error> {
    let mut drawing = None;
    let mut strict = false;
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        let value = attribute.value.as_bytes();
        if value == CHART_NAMESPACE[1] {
            strict = true;
        }
        if !DRAWINGML.contains(&value) || drawing.is_some() {
            continue;
        }
        match attribute.key.as_namespace_binding() {
            Some(PrefixDeclaration::Named(prefix)) => {
                drawing = Some([prefix.as_bytes(), b":"].concat());
            }
            Some(PrefixDeclaration::Default) => drawing = Some(Vec::new()),
            None => {}
        }
    }
    Ok((drawing, strict))
}

/// One change to the part's bytes: `from..to` replaced by `bytes` (an
/// insertion where the two are equal).
struct Splice {
    from: usize,
    to: usize,
    bytes: Vec<u8>,
}

/// The chart part `bytes` (entry `index`) with `edits` applied.
fn edit_chart(bytes: &[u8], index: usize, edits: &[ChartEdit]) -> Result<Vec<u8>, Error> {
    let (root, drawing, strict) = read_tree(bytes, index)?;
    let chart = root.child(b"chart").ok_or(CHART_PART_UNREADABLE)?;
    if chart.empty {
        return Err(CHART_PART_UNREADABLE);
    }
    let forms = Forms {
        drawing,
        strict: usize::from(strict),
    };
    // The series in document order across the plots, with their plots'
    // names (ruling E, Q5).
    let mut series: Vec<(&[u8], &Element)> = Vec::new();
    if let Some(area) = chart.child(b"plotArea") {
        for plot in &area.children {
            for one in plot.children.iter().filter(|child| child.chart(b"ser")) {
                series.push((&plot.local, one));
            }
        }
    }
    let mut splices: Vec<Splice> = Vec::new();
    for edit in edits {
        if let ChartEdit::Series(at, _) = edit
            && *at >= series.len()
        {
            return Err(NO_SERIES_AT);
        }
    }
    for edit in edits {
        match edit {
            ChartEdit::Title(title) => {
                title_splices(chart, title.as_deref(), &forms, &mut splices)?
            }
            ChartEdit::Series(at, work) => {
                let (plot, one) = series[*at];
                if !PLOTS.contains(&plot) || one.empty {
                    return Err(CHART_PART_UNREADABLE);
                }
                series_splices(plot, one, work, &mut splices);
            }
        }
    }
    // Stable: two insertions at one place keep the order they were made in.
    splices.sort_by_key(|splice| (splice.from, splice.to));
    let mut out = Vec::with_capacity(bytes.len());
    let mut from = 0usize;
    for splice in splices {
        if splice.from < from {
            return Err(CHART_PART_UNREADABLE);
        }
        out.extend_from_slice(&bytes[from..splice.from]);
        out.extend_from_slice(&splice.bytes);
        from = splice.to;
    }
    out.extend_from_slice(&bytes[from..]);
    Ok(out)
}

/// What a written element needs beyond the part: the DrawingML prefix the
/// root binds, if any, and the namespace form (1 strict).
struct Forms {
    drawing: Option<Vec<u8>>,
    strict: usize,
}

impl Forms {
    /// The writer's rich text of `title` in a `tx`, chart elements with
    /// prefix `c`: the DrawingML elements with the root's prefix, or with
    /// `a` declared on the `rich` element where the root binds none
    /// (ruling E, Q9).
    fn tx(&self, c: &[u8], title: &str) -> Vec<u8> {
        let (a, declaration): (&[u8], Vec<u8>) = match &self.drawing {
            Some(prefix) => (prefix, Vec::new()),
            None => (
                b"a:",
                [b" xmlns:a=\"", DRAWINGML[self.strict], b"\""].concat(),
            ),
        };
        let mut out = Vec::new();
        for piece in [
            b"<".as_slice(),
            c,
            b"tx><",
            c,
            b"rich",
            &declaration,
            b"><",
            a,
            b"bodyPr/><",
            a,
            b"lstStyle/><",
            a,
            b"p><",
            a,
            b"r><",
            a,
            b"t>",
            title.as_bytes(),
            b"</",
            a,
            b"t></",
            a,
            b"r></",
            a,
            b"p></",
            c,
            b"rich></",
            c,
            b"tx>",
        ] {
            out.extend_from_slice(piece);
        }
        out
    }
}

/// A chart-namespace element `<cX v="VALUE"/>`'s bytes with prefix `c`.
fn value_element(c: &[u8], name: &[u8], value: &[u8]) -> Vec<u8> {
    [b"<".as_slice(), c, name, b" val=\"", value, b"\"/>"].concat()
}

/// A reference element `<cNAME><cFORM><cf>F</cf></cFORM></cNAME>`.
fn reference_element(c: &[u8], name: &[u8], form: &[u8], formula: &str) -> Vec<u8> {
    [
        b"<".as_slice(),
        c,
        name,
        b"><",
        c,
        form,
        b"><",
        c,
        b"f>",
        formula.as_bytes(),
        b"</",
        c,
        b"f></",
        c,
        form,
        b"></",
        c,
        name,
        b">",
    ]
    .concat()
}

/// Whether a boolean element's `val` is true: absent, `1` or `true`.
fn flag(element: &Element) -> bool {
    matches!(element.val.as_deref(), None | Some(b"1" | b"true"))
}

/// The splices of a title edit (ruling E, Q7 and Q8).
fn title_splices(
    chart: &Element,
    title: Option<&str>,
    forms: &Forms,
    splices: &mut Vec<Splice>,
) -> Result<(), Error> {
    let c = &chart.prefix;
    let kept = chart.child(b"title");
    let deleted = chart.child(b"autoTitleDeleted");
    let Some(title) = title else {
        // Removed, `autoTitleDeleted` set to 1 or inserted where the title
        // was, or first.
        let place = match kept {
            Some(kept) => {
                splices.push(Splice {
                    from: kept.start,
                    to: kept.end,
                    bytes: Vec::new(),
                });
                kept.end
            }
            None => chart.open,
        };
        let one = value_element(c, b"autoTitleDeleted", b"1");
        match deleted {
            Some(deleted) if flag(deleted) => {}
            Some(deleted) => splices.push(Splice {
                from: deleted.start,
                to: deleted.end,
                bytes: value_element(&deleted.prefix, b"autoTitleDeleted", b"1"),
            }),
            None => splices.push(Splice {
                from: place,
                to: place,
                bytes: one,
            }),
        }
        return Ok(());
    };
    let Some(kept) = kept else {
        // Inserted first, the writer's form; an `autoTitleDeleted` of 1
        // becomes 0.
        let mut bytes = [b"<".as_slice(), c, b"title>"].concat();
        bytes.extend_from_slice(&forms.tx(c, title));
        bytes.extend_from_slice(&value_element(c, b"overlay", b"0"));
        bytes.extend_from_slice(&[b"</".as_slice(), c, b"title>"].concat());
        splices.push(Splice {
            from: chart.open,
            to: chart.open,
            bytes,
        });
        if let Some(deleted) = deleted
            && flag(deleted)
        {
            splices.push(Splice {
                from: deleted.start,
                to: deleted.end,
                bytes: value_element(&deleted.prefix, b"autoTitleDeleted", b"0"),
            });
        }
        return Ok(());
    };
    let t = &kept.prefix;
    if kept.empty {
        let bytes = [
            [b"<".as_slice(), t, b"title>"].concat(),
            forms.tx(t, title),
            [b"</".as_slice(), t, b"title>"].concat(),
        ]
        .concat();
        splices.push(Splice {
            from: kept.start,
            to: kept.end,
            bytes,
        });
        return Ok(());
    }
    let Some(tx) = kept.child(b"tx") else {
        // No `tx`: one inserted as the title's first child.
        splices.push(Splice {
            from: kept.open,
            to: kept.open,
            bytes: forms.tx(t, title),
        });
        return Ok(());
    };
    // A kept rich text's first run: its `t` alone replaced, further runs
    // and paragraphs dropped; otherwise the writer's rich text.
    let run = tx.child(b"rich").and_then(|rich| {
        let paragraph = rich
            .children
            .iter()
            .find(|child| child.is(Space::Drawing, b"p"))?;
        let run = paragraph
            .children
            .iter()
            .find(|child| child.is(Space::Drawing, b"r"))?;
        let t = run
            .children
            .iter()
            .find(|child| child.is(Space::Drawing, b"t"))?;
        Some((rich, paragraph, run, t))
    });
    let Some((rich, paragraph, run, text)) = run else {
        splices.push(Splice {
            from: tx.start,
            to: tx.end,
            bytes: forms.tx(&tx.prefix, title),
        });
        return Ok(());
    };
    if text.empty {
        let a = &text.prefix;
        splices.push(Splice {
            from: text.start,
            to: text.end,
            bytes: [b"<".as_slice(), a, b"t>", title.as_bytes(), b"</", a, b"t>"].concat(),
        });
    } else {
        splices.push(Splice {
            from: text.open,
            to: text.close,
            bytes: title.as_bytes().to_vec(),
        });
    }
    // The first paragraph keeps its `pPr`, the run and its `endParaRPr`.
    for child in &paragraph.children {
        let kept = child.is(Space::Drawing, b"pPr")
            || child.is(Space::Drawing, b"endParaRPr")
            || std::ptr::eq(child, run);
        if !kept {
            splices.push(Splice {
                from: child.start,
                to: child.end,
                bytes: Vec::new(),
            });
        }
    }
    for child in &rich.children {
        if child.is(Space::Drawing, b"p") && !std::ptr::eq(child, paragraph) {
            splices.push(Splice {
                from: child.start,
                to: child.end,
                bytes: Vec::new(),
            });
        }
    }
    Ok(())
}

/// The splices of a series edit on `one`, a series of the plot `plot`.
fn series_splices(plot: &[u8], one: &Element, work: &SeriesEdit, splices: &mut Vec<Splice>) {
    let c = &one.prefix;
    let scatter = plot == b"scatterChart" || plot == b"bubbleChart";
    let (x, x_form, y): (&[u8], &[u8], &[u8]) = if scatter {
        (b"xVal", b"numRef", b"yVal")
    } else {
        (b"cat", b"strRef", b"val")
    };
    let find = |local: &[u8]| one.children.iter().find(|child| child.chart(local));
    if let Some(name) = &work.name {
        let bytes = match name {
            Name::Reference(formula) => reference_element(c, b"tx", b"strRef", formula),
            Name::Text(name) => [
                b"<".as_slice(),
                c,
                b"tx><",
                c,
                b"v>",
                name.as_bytes(),
                b"</",
                c,
                b"v></",
                c,
                b"tx>",
            ]
            .concat(),
        };
        match find(b"tx") {
            Some(tx) => splices.push(Splice {
                from: tx.start,
                to: tx.end,
                bytes,
            }),
            None => {
                let place = find(b"order")
                    .or_else(|| find(b"idx"))
                    .map_or(one.open, |after| after.end);
                splices.push(Splice {
                    from: place,
                    to: place,
                    bytes,
                });
            }
        }
    }
    let trailing = one
        .children
        .iter()
        .find(|child| child.space == Space::Chart && TRAILING.contains(&child.local.as_slice()))
        .map_or(one.close, |child| child.start);
    let kept_y = find(y);
    // The form found, `numRef` or `strRef`; the writer's otherwise.
    let form = |element: &Element, writer: &'static [u8]| -> Vec<u8> {
        element
            .children
            .iter()
            .find(|child| child.chart(b"numRef") || child.chart(b"strRef"))
            .map_or(writer.to_vec(), |child| child.local.clone())
    };
    if let Some(categories) = &work.categories {
        match find(x) {
            Some(kept) => splices.push(Splice {
                from: kept.start,
                to: kept.end,
                bytes: reference_element(c, x, &form(kept, x_form), categories),
            }),
            None => {
                let place = kept_y.map_or(trailing, |kept| kept.start);
                splices.push(Splice {
                    from: place,
                    to: place,
                    bytes: reference_element(c, x, x_form, categories),
                });
            }
        }
    }
    if let Some(values) = &work.values {
        match kept_y {
            Some(kept) => splices.push(Splice {
                from: kept.start,
                to: kept.end,
                bytes: reference_element(c, y, &form(kept, b"numRef"), values),
            }),
            None => splices.push(Splice {
                from: trailing,
                to: trailing,
                bytes: reference_element(c, y, b"numRef", values),
            }),
        }
    }
}
