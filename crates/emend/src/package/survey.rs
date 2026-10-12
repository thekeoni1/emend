//! What a package holds, counted by part kind (`specs/0033-read-breadth-
//! survey.md`, "The survey"). The Cargo feature `read`.
//!
//! [`Package::part_kinds`] reads the content types and every entry's name and
//! nothing else: each entry is counted by the content type its override or
//! its extension's default gives it, compared whole by exact bytes, and
//! nothing is inferred from a name. A strict package writes the same content
//! types as a transitional one, so one spelling counts both.
//! [`Package::sheet_parts`] reads one sheet's relationships, its drawings'
//! relationships, and the sheet part once, streamed, for its sparkline
//! groups. A relationship is counted as the package states it: a target
//! naming no entry is counted, not resolved.

use std::io::{Read, Seek};

use quick_xml::events::Event;

use super::{
    CHART, COMMENTS, CONTENT_TYPES_PART, DRAWING, HYPERLINK, IMAGE, NO_SUCH_PART, PIVOT_TABLE,
    PRINTER_SETTINGS, Package, SLICER, TABLE, TIMELINE, VML_DRAWING,
};
use crate::archive::Error;
use crate::xml::{Scope, Stream};

/// The namespace of `x14:sparklineGroup` (Microsoft's 2009 extension).
const X14: &[u8] = b"http://schemas.microsoft.com/office/spreadsheetml/2009/9/main";

/// The prefix every content type below shares but the image types.
const VND: &[u8] = b"application/vnd.";

/// How many entries of each kind a package holds, by content type. The Cargo
/// feature `read`.
///
/// Every entry but `[Content_Types].xml` is counted once, so the fields sum to
/// the entry count less one.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PartKinds {
    /// `...spreadsheetml.worksheet+xml`.
    pub worksheets: u32,
    /// `...spreadsheetml.chartsheet+xml`.
    pub chartsheets: u32,
    /// `...spreadsheetml.dialogsheet+xml`.
    pub dialogsheets: u32,
    /// `ms-excel.macrosheet+xml` and `ms-excel.intlmacrosheet+xml`.
    pub macrosheets: u32,
    /// `...drawingml.chart+xml`.
    pub charts: u32,
    /// `ms-office.chartex+xml`.
    pub chartex: u32,
    /// `...drawing+xml`.
    pub drawings: u32,
    /// Any type beginning `image/`.
    pub images: u32,
    /// `...spreadsheetml.table+xml`.
    pub tables: u32,
    /// `...spreadsheetml.pivotTable+xml`.
    pub pivot_tables: u32,
    /// `...spreadsheetml.pivotCacheDefinition+xml`.
    pub pivot_caches: u32,
    /// `...spreadsheetml.pivotCacheRecords+xml`.
    pub pivot_cache_records: u32,
    /// `...spreadsheetml.externalLink+xml`.
    pub external_links: u32,
    /// `ms-office.vbaProject`.
    pub vba_projects: u32,
    /// `ms-excel.slicer+xml`.
    pub slicers: u32,
    /// `ms-excel.slicerCache+xml`.
    pub slicer_caches: u32,
    /// `ms-excel.timeline+xml`.
    pub timelines: u32,
    /// `ms-excel.timelineCache+xml`.
    pub timeline_caches: u32,
    /// `...spreadsheetml.comments+xml`.
    pub comments: u32,
    /// `ms-excel.threadedcomments+xml`.
    pub threaded_comments: u32,
    /// `ms-excel.person+xml`.
    pub persons: u32,
    /// `...customXmlProperties+xml`, one per custom XML item.
    pub custom_xml: u32,
    /// `...spreadsheetml.connections+xml`.
    pub connections: u32,
    /// `...spreadsheetml.queryTable+xml`.
    pub query_tables: u32,
    /// `...custom-properties+xml`.
    pub custom_properties: u32,
    /// `...spreadsheetml.printerSettings`.
    pub printer_settings: u32,
    /// `...spreadsheetml.calcChain+xml`.
    pub calc_chains: u32,
    /// An entry with a type none of the fields above names.
    pub other_typed: u32,
    /// An entry with no type.
    pub untyped: u32,
}

/// What one sheet's relationships, and its drawings' relationships, say it
/// holds, with its sparkline groups. The Cargo feature `read`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SheetParts {
    /// `drawing` relationships.
    pub drawings: u32,
    /// `chart` relationships of the drawings.
    pub charts: u32,
    /// `image` relationships of the drawings.
    pub images: u32,
    /// `comments` relationships.
    pub comments: u32,
    /// `vmlDrawing` relationships.
    pub vml_drawings: u32,
    /// `table` relationships.
    pub tables: u32,
    /// `pivotTable` relationships.
    pub pivot_tables: u32,
    /// `slicer` relationships.
    pub slicers: u32,
    /// `timeline` relationships.
    pub timelines: u32,
    /// `printerSettings` relationships.
    pub printer_settings: u32,
    /// `hyperlink` relationships whose `TargetMode` is `External`.
    pub external_hyperlinks: u32,
    /// `x14:sparklineGroup` elements anywhere in the sheet part.
    pub sparkline_groups: u32,
}

impl PartKinds {
    /// The field a content type counts in.
    fn field(&mut self, content_type: Option<&[u8]>) -> &mut u32 {
        let Some(content_type) = content_type else {
            return &mut self.untyped;
        };
        if content_type.starts_with(b"image/") {
            return &mut self.images;
        }
        let Some(kind) = content_type.strip_prefix(VND) else {
            return &mut self.other_typed;
        };
        let sml = kind.strip_prefix(b"openxmlformats-officedocument.spreadsheetml.");
        match sml {
            Some(b"worksheet+xml") => &mut self.worksheets,
            Some(b"chartsheet+xml") => &mut self.chartsheets,
            Some(b"dialogsheet+xml") => &mut self.dialogsheets,
            Some(b"table+xml") => &mut self.tables,
            Some(b"pivotTable+xml") => &mut self.pivot_tables,
            Some(b"pivotCacheDefinition+xml") => &mut self.pivot_caches,
            Some(b"pivotCacheRecords+xml") => &mut self.pivot_cache_records,
            Some(b"externalLink+xml") => &mut self.external_links,
            Some(b"comments+xml") => &mut self.comments,
            Some(b"connections+xml") => &mut self.connections,
            Some(b"queryTable+xml") => &mut self.query_tables,
            Some(b"printerSettings") => &mut self.printer_settings,
            Some(b"calcChain+xml") => &mut self.calc_chains,
            Some(_) => &mut self.other_typed,
            None => match kind {
                b"ms-excel.macrosheet+xml" | b"ms-excel.intlmacrosheet+xml" => {
                    &mut self.macrosheets
                }
                b"openxmlformats-officedocument.drawingml.chart+xml" => &mut self.charts,
                b"ms-office.chartex+xml" => &mut self.chartex,
                b"openxmlformats-officedocument.drawing+xml" => &mut self.drawings,
                b"ms-office.vbaProject" => &mut self.vba_projects,
                b"ms-excel.slicer+xml" => &mut self.slicers,
                b"ms-excel.slicerCache+xml" => &mut self.slicer_caches,
                b"ms-excel.timeline+xml" => &mut self.timelines,
                b"ms-excel.timelineCache+xml" => &mut self.timeline_caches,
                b"ms-excel.threadedcomments+xml" => &mut self.threaded_comments,
                b"ms-excel.person+xml" => &mut self.persons,
                b"openxmlformats-officedocument.customXmlProperties+xml" => &mut self.custom_xml,
                b"openxmlformats-officedocument.custom-properties+xml" => {
                    &mut self.custom_properties
                }
                _ => &mut self.other_typed,
            },
        }
    }
}

/// Whether `kind` is either form of `of`; a type with one form lists it
/// twice and is still counted once.
fn is(kind: &[u8], of: [&[u8]; 2]) -> bool {
    of.contains(&kind)
}

impl<R: Read + Seek> Package<R> {
    /// How many entries of each kind the package holds, by content type
    /// (`specs/0033-read-breadth-survey.md`, "The survey"). Reads the content
    /// types, parsed once and kept, and the entry names; no other part. The
    /// Cargo feature `read`.
    ///
    /// # Errors
    ///
    /// Whatever [`Package::content_types`] refuses.
    pub fn part_kinds(&mut self) -> Result<PartKinds, Error> {
        self.content_types()?;
        // Filled by the call above.
        let types = self.content_types.as_ref().unwrap();
        let mut kinds = PartKinds::default();
        for entry in self.archive.entries() {
            let name = entry.name();
            if name == CONTENT_TYPES_PART {
                continue;
            }
            let field = kinds.field(types.of(name));
            *field = field.saturating_add(1);
        }
        Ok(kinds)
    }

    /// What the sheet part `part` holds by its relationships, its drawings'
    /// relationships and its sparkline groups (`specs/0033-read-breadth-
    /// survey.md`, "The survey"). A part with no rels part counts 0 of each
    /// relationship; a drawing relationship that is external or whose target
    /// does not resolve is counted and not followed; a drawing with no rels
    /// part adds no chart or image. The sheet part is streamed once and only
    /// a count is held. The Cargo feature `read`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part" when the archive holds no entry
    /// `part`; whatever [`Package::relationships`] refuses for the sheet's or
    /// a drawing's rels part; "an element or attribute uses a prefix its
    /// document does not bind"; [`Error::Xml`] for bytes that are not
    /// well-formed XML or not UTF-8.
    pub fn sheet_parts(&mut self, part: &[u8]) -> Result<SheetParts, Error> {
        let index = self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        let mut counts = SheetParts::default();
        let mut drawings = Vec::new();
        {
            let rels = self.relationships(part)?;
            for rel in rels.all() {
                let kind = rel.kind.as_slice();
                let field = if is(kind, DRAWING) {
                    // An external target has no part, so it is not followed.
                    if let Ok(target) = rels.target_part(rel) {
                        drawings.push(target);
                    }
                    &mut counts.drawings
                } else if is(kind, COMMENTS) {
                    &mut counts.comments
                } else if is(kind, VML_DRAWING) {
                    &mut counts.vml_drawings
                } else if is(kind, TABLE) {
                    &mut counts.tables
                } else if is(kind, PIVOT_TABLE) {
                    &mut counts.pivot_tables
                } else if is(kind, SLICER) {
                    &mut counts.slicers
                } else if is(kind, TIMELINE) {
                    &mut counts.timelines
                } else if is(kind, PRINTER_SETTINGS) {
                    &mut counts.printer_settings
                } else if is(kind, HYPERLINK) && rel.external {
                    &mut counts.external_hyperlinks
                } else {
                    continue;
                };
                *field = field.saturating_add(1);
            }
        }
        for drawing in drawings {
            for rel in self.relationships(&drawing)?.all() {
                let kind = rel.kind.as_slice();
                let field = if is(kind, CHART) {
                    &mut counts.charts
                } else if is(kind, IMAGE) {
                    &mut counts.images
                } else {
                    continue;
                };
                *field = field.saturating_add(1);
            }
        }
        let mut stream = Stream::new(self.archive.read(index)?, index)?;
        let mut scope = Scope::new(index);
        while let Some(token) = stream.next()? {
            match token.event {
                Event::Start(ref start) | Event::Empty(ref start) => {
                    scope.start(start)?;
                    let (namespace, local) = scope.resolve_element(start.name())?;
                    if namespace == Some(X14) && local == b"sparklineGroup" {
                        counts.sparkline_groups = counts.sparkline_groups.saturating_add(1);
                    }
                    if matches!(token.event, Event::Empty(_)) {
                        scope.end();
                    }
                }
                Event::End(_) => scope.end(),
                _ => {}
            }
        }
        Ok(counts)
    }
}
