//! A worksheet's column and row formats and its settings, read as the types
//! the writers take (`specs/0027-read-back.md`, "Columns and rows" and "The
//! sheet in one pass"). The Cargo feature `read`.
//!
//! Each call reads the worksheet part forwards once and holds no more of it
//! than one element at a time: [`Package::columns`] the `<col>` elements of
//! its `<cols>`, [`Package::rows`] one `<row>`'s start tag, and
//! [`Package::sheet_settings`] each small element to its end tag, with the
//! merges and hyperlinks the part lists. Every value read is one the
//! matching writer accepts: an attribute value that does not parse or lies
//! outside the writer's range is refused with "a sheet setting cannot be
//! read", as is a `ref` that is not an area and a hyperlink `r:id` naming no
//! relationship. Attributes and elements the types have no field for are
//! read past; a part with two of an element the writers refuse twice is
//! refused with the writer's own message; schema order is not checked
//! ("Planning rulings during P2", ruling 2).

use std::io::{Read, Seek};
use std::marker::PhantomData;

use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};

use super::Orientation;
use super::{
    HEADER_FOOTER_CHILDREN, HeaderFooter, MAIN, NO_SUCH_PART, Package, PageMargins, PageOrder,
    PageSetup, PrintOptions, RELATIONSHIPS_ATTRIBUTE, SheetProtection, SheetView, TWO_ELEMENTS,
    TWO_SETTINGS, TWO_SHEET_ELEMENTS, ViewKind,
};
use crate::archive::Error;
use crate::cells::{
    Area, CellRef, ColorAttributes, color_from, decimal, forbidden_in_xml, row_number,
    skip_subtree, text_of,
};
use crate::styles::Color;
use crate::xml::{Scope, Stream};

/// What a worksheet's settings, columns or rows this layer cannot read into
/// the writers' types are refused with; catalogued in [`crate::package`]'s
/// module header.
pub(crate) const SETTING_UNREADABLE: Error = Error::Package("a sheet setting cannot be read");

fn refusal() -> Error {
    SETTING_UNREADABLE
}

/// The root a worksheet part must have, as [`crate::cells::Cells`] refuses
/// another.
const NOT_WORKSHEET: Error = Error::Package("the worksheet part's root is not worksheet");

/// One `<col>` of a worksheet's `<cols>`: the columns `first` to `last`,
/// 1-based, and their format. The Cargo feature `read`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColumnFormat {
    /// `min`.
    pub first: u32,
    /// `max`.
    pub last: u32,
    /// `width`, in `0..=255`.
    pub width: Option<f64>,
    /// `customWidth`.
    pub custom_width: bool,
    /// `hidden`.
    pub hidden: bool,
    /// `style`, a cell format index.
    pub style: Option<u32>,
    /// `outlineLevel`, `0..=7`.
    pub outline_level: u8,
    /// `collapsed`.
    pub collapsed: bool,
}

/// One `<row>` of a worksheet's `<sheetData>` and its format, its cells
/// read past. The Cargo feature `read`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowFormat {
    /// `r`, or the previous row plus one when absent, 1 for the first, as
    /// [`crate::cells::Cells`] numbers it ("Planning rulings during P2",
    /// ruling 1).
    pub row: u32,
    /// `ht`, in points, `0..=409`.
    pub height: Option<f64>,
    /// `customHeight`.
    pub custom_height: bool,
    /// `hidden`.
    pub hidden: bool,
    /// `s`, a cell format index.
    pub style: Option<u32>,
    /// `customFormat`.
    pub custom_format: bool,
    /// `outlineLevel`, `0..=7`.
    pub outline_level: u8,
    /// `collapsed`.
    pub collapsed: bool,
}

/// A hyperlink's target as the part gives it. The Cargo feature `read`.
/// This type was formerly `Target`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HyperlinkTarget {
    /// The relationship's target, for a hyperlink with an `r:id`.
    Url(String),
    /// The `location` attribute, for a hyperlink without an `r:id`.
    Location(String),
}

/// One `<hyperlink>` of a worksheet. The Cargo feature `read`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HyperlinkEntry {
    /// The cell, the top-left one of a `ref` that is a range.
    pub cell: CellRef,
    /// Where it goes.
    pub target: HyperlinkTarget,
    /// `display`.
    pub display: Option<String>,
    /// `tooltip`.
    pub tooltip: Option<String>,
}

/// A worksheet's settings, read in one pass (`specs/0027-read-back.md`,
/// "The sheet in one pass"). A setting the part does not carry is `None`,
/// empty or its type's default. The Cargo feature `read`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SheetSettings {
    /// `<sheetPr>`'s `<tabColor>`.
    pub tab_color: Option<Color>,
    /// `<outlinePr>`'s `summaryBelow` and `summaryRight`.
    pub outline_summary: (Option<bool>, Option<bool>),
    /// `<pageSetUpPr>`'s `fitToPage`.
    pub fit_to_page: Option<bool>,
    /// The first `<sheetView>`'s attributes.
    pub view: SheetView,
    /// The first `<sheetView>`'s `<pane>`'s `topLeftCell` when its `state`
    /// is `frozen`; `None` for no pane or a split one.
    pub freeze: Option<CellRef>,
    /// `<printOptions>`.
    pub print_options: PrintOptions,
    /// `<pageMargins>`, `None` without one.
    pub margins: Option<PageMargins>,
    /// `<pageSetup>`, its `fit_to_page` from `<sheetPr>`.
    pub page_setup: PageSetup,
    /// `<headerFooter>`'s six texts.
    pub header_footer: HeaderFooter,
    /// `<autoFilter ref>`.
    pub auto_filter: Option<Area>,
    /// Every `<mergeCell ref>`, in part order.
    pub merges: Vec<Area>,
    /// Every `<hyperlink>`, in part order.
    pub hyperlinks: Vec<HyperlinkEntry>,
    /// `<sheetProtection>`'s sixteen flags, each the schema's default when
    /// absent, the value the writer omits (`sheet` false among them), from
    /// the table the writer compares against ("Planning verification and
    /// rulings on P2", ruling 3); the password hash never read.
    pub protection: Option<SheetProtection>,
    /// `<sheetFormatPr>`'s `outlineLevelRow` and `outlineLevelCol`.
    pub outline_max: (u8, u8),
}

/// A streaming read of a worksheet's `<row>` formats, built by
/// [`Package::rows`]. It reads forwards once, holds one row's start tag at a
/// time, never seeks and borrows the package as [`crate::cells::Cells`]
/// does; after an error it is spent. The Cargo feature `read`.
pub struct Rows<'p, R: Read + Seek> {
    walk: RowWalk<'p>,
    package: PhantomData<&'p mut Package<R>>,
}

impl<R: Read + Seek> Iterator for Rows<'_, R> {
    type Item = Result<RowFormat, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.walk.done {
            return None;
        }
        match self.walk.advance() {
            Ok(Some(row)) => Some(Ok(row)),
            Ok(None) => {
                self.walk.done = true;
                None
            }
            Err(error) => {
                self.walk.done = true;
                Some(Err(error))
            }
        }
    }
}

/// The state of a [`Rows`], naming no reader type.
struct RowWalk<'p> {
    stream: Stream<Box<dyn Read + 'p>>,
    scope: Scope,
    index: usize,
    root: bool,
    /// Whether a `<sheetData>` is open.
    data: bool,
    /// The last row number given, 0 before the first.
    last_row: u32,
    done: bool,
}

impl RowWalk<'_> {
    fn advance(&mut self) -> Result<Option<RowFormat>, Error> {
        while let Some(token) = self.stream.next()? {
            let mut found: Option<(RowFormat, bool)> = None;
            let mut skip = false;
            match token.event {
                Event::Start(ref start) | Event::Empty(ref start) => {
                    let empty = matches!(token.event, Event::Empty(_));
                    self.scope.start(start)?;
                    let element = Element::of(start, &self.scope, self.index, empty)?;
                    if !self.root {
                        if !element.main || element.local != b"worksheet" {
                            return Err(NOT_WORKSHEET);
                        }
                        self.root = true;
                    } else if element.main && element.local == b"sheetData" && !self.data {
                        self.data = !empty;
                    } else if element.main && element.local == b"row" && self.data {
                        let row = match row_number(start, &self.scope, self.index)? {
                            Some(row) => row,
                            None => self.last_row.saturating_add(1),
                        };
                        self.last_row = row;
                        found = Some((row_format(&element, row)?, empty));
                    } else if !empty {
                        skip = true;
                    }
                    if empty {
                        self.scope.end();
                    }
                }
                Event::End(ref end) => {
                    let (namespace, local) = self.scope.resolve_element(end.name())?;
                    let closes_data =
                        MAIN.contains(&namespace.unwrap_or_default()) && local == b"sheetData";
                    self.scope.end();
                    if closes_data {
                        self.data = false;
                    }
                }
                _ => {}
            }
            if let Some((row, empty)) = found {
                if !empty {
                    skip_subtree(&mut self.stream, &mut self.scope, self.index)?;
                }
                return Ok(Some(row));
            }
            if skip {
                skip_subtree(&mut self.stream, &mut self.scope, self.index)?;
            }
        }
        if !self.root {
            return Err(NOT_WORKSHEET);
        }
        Ok(None)
    }
}

impl<R: Read + Seek> Package<R> {
    /// The `<col>` elements of the worksheet part `part`'s `<cols>`, in
    /// part order (`specs/0027-read-back.md`, "Columns and rows"). The
    /// Cargo feature `read`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part", "the worksheet part's root is not
    /// worksheet", "a worksheet has two cols, sheetViews or mergeCells
    /// elements, or a sheetViews without a sheetView" for two `<cols>`, and
    /// "a sheet setting cannot be read" for a `<col>` without `min` or
    /// `max`, or an attribute that does not parse or lies outside the
    /// writer's range; [`Error::Xml`] for bytes that are not well-formed XML
    /// or not UTF-8.
    pub fn columns(&mut self, part: &[u8]) -> Result<Vec<ColumnFormat>, Error> {
        let index = self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        let mut stream = Stream::new(self.archive.read(index)?, index)?;
        let mut scope = Scope::new(index);
        let mut depth = 0usize;
        let mut cols: Option<bool> = None;
        let mut out = Vec::new();
        while let Some(token) = stream.next()? {
            let mut skip = false;
            match token.event {
                Event::Start(ref start) | Event::Empty(ref start) => {
                    let empty = matches!(token.event, Event::Empty(_));
                    depth += 1;
                    scope.start(start)?;
                    let element = Element::of(start, &scope, index, empty)?;
                    if depth == 1 {
                        if !element.main || element.local != b"worksheet" {
                            return Err(NOT_WORKSHEET);
                        }
                    } else if depth == 2 && element.main && element.local == b"cols" {
                        if cols.replace(!empty).is_some() {
                            return Err(TWO_ELEMENTS);
                        }
                    } else if depth == 3
                        && cols == Some(true)
                        && element.main
                        && element.local == b"col"
                    {
                        out.push(column_format(&element)?);
                        skip = !empty;
                    } else if !empty {
                        skip = true;
                    }
                    if empty {
                        scope.end();
                        depth -= 1;
                    }
                }
                Event::End(_) => {
                    if depth == 2 && cols == Some(true) {
                        cols = Some(false);
                    }
                    scope.end();
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
            if skip {
                skip_subtree(&mut stream, &mut scope, index)?;
                depth -= 1;
            }
        }
        Ok(out)
    }

    /// A streaming read of the `<row>` formats of the worksheet part
    /// `part`'s `<sheetData>` (`specs/0027-read-back.md`, "Columns and
    /// rows", decision 4). The Cargo feature `read`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part", and whatever the first bytes of
    /// the part refuse; each item may be [`Error::Package`] "the worksheet
    /// part's root is not worksheet", "a row number is not a number", or
    /// "a sheet setting cannot be read" for an attribute that does not
    /// parse or lies outside the writer's range, or [`Error::Xml`].
    pub fn rows(&mut self, part: &[u8]) -> Result<Rows<'_, R>, Error> {
        let index = self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        let reader: Box<dyn Read + '_> = Box::new(self.archive.read(index)?);
        Ok(Rows {
            walk: RowWalk {
                stream: Stream::new(reader, index)?,
                scope: Scope::new(index),
                index,
                root: false,
                data: false,
                last_row: 0,
                done: false,
            },
            package: PhantomData,
        })
    }

    /// The settings of the worksheet part `part`, read in one pass
    /// (`specs/0027-read-back.md`, "The sheet in one pass", decision 5). The
    /// Cargo feature `read`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part", "the worksheet part's root is not
    /// worksheet"; for two of an element the writers refuse twice, the
    /// writer's message: "a worksheet has two cols, sheetViews or mergeCells
    /// elements, or a sheetViews without a sheetView", "a worksheet has two
    /// hyperlinks or sheetProtection elements", "a worksheet has two
    /// sheetPr, autoFilter, printOptions, pageMargins, pageSetup or
    /// headerFooter elements" (`<sheetFormatPr>` with them); "a sheet
    /// setting cannot be read" for an attribute value that does not parse
    /// or lies outside the writer's range, a `ref` that is not an area, a
    /// hyperlink with neither an `r:id` nor a `location` or with an `r:id`
    /// naming no relationship, and a header or footer text the writer
    /// refuses; what [`crate::cells::Cells`] refuses in a header or footer
    /// text; [`Error::Xml`] for bytes that are not well-formed XML or not
    /// UTF-8; whatever [`Package::relationships`] refuses for the part's
    /// rels part, when a hyperlink names a relationship.
    pub fn sheet_settings(&mut self, part: &[u8]) -> Result<SheetSettings, Error> {
        let index = self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        let mut stream = Stream::new(self.archive.read(index)?, index)?;
        let mut pass = Pass {
            index,
            scope: Scope::new(index),
            settings: SheetSettings::default(),
            links: Vec::new(),
            seen: Vec::new(),
        };
        pass.read(&mut stream)?;
        // The part is read; the relationships come after it.
        drop(stream);
        let Pass {
            mut settings,
            links,
            ..
        } = pass;
        settings.page_setup.fit_to_page = settings.fit_to_page;
        for link in links {
            let target = match (link.id, link.location) {
                (Some(id), _) => {
                    let relationships = self.relationships(part)?;
                    let relationship = relationships.by_id(&id).ok_or(SETTING_UNREADABLE)?;
                    // The relationship keeps its `Target` as written,
                    // escaped; the URL is the attribute's value.
                    let written = std::str::from_utf8(&relationship.target)
                        .map_err(|_| SETTING_UNREADABLE)?;
                    let url =
                        quick_xml::escape::unescape(written).map_err(|_| SETTING_UNREADABLE)?;
                    HyperlinkTarget::Url(url.into_owned())
                }
                (None, Some(location)) if !location.is_empty() => {
                    HyperlinkTarget::Location(location)
                }
                _ => return Err(SETTING_UNREADABLE),
            };
            settings.hyperlinks.push(HyperlinkEntry {
                cell: link.cell,
                target,
                display: link.display,
                tooltip: link.tooltip,
            });
        }
        Ok(settings)
    }
}

/// A hyperlink as the pass reads it, its relationship resolved after.
struct Link {
    cell: CellRef,
    id: Option<Vec<u8>>,
    location: Option<String>,
    display: Option<String>,
    tooltip: Option<String>,
}

/// The one pass of [`Package::sheet_settings`].
struct Pass {
    index: usize,
    scope: Scope,
    settings: SheetSettings,
    links: Vec<Link>,
    /// The root children read so far, by local name, for the duplicates.
    seen: Vec<&'static [u8]>,
}

/// The root children [`Pass`] reads, each with the message a second one is
/// refused with.
const READ_CHILDREN: [(&[u8], u8); 11] = [
    (b"sheetPr", 2),
    (b"sheetViews", 0),
    (b"sheetFormatPr", 2),
    (b"sheetProtection", 1),
    (b"autoFilter", 2),
    (b"mergeCells", 0),
    (b"hyperlinks", 1),
    (b"printOptions", 2),
    (b"pageMargins", 2),
    (b"pageSetup", 2),
    (b"headerFooter", 2),
];

impl Pass {
    fn read<R: Read>(&mut self, stream: &mut Stream<R>) -> Result<(), Error> {
        let mut root = false;
        while let Some(token) = stream.next()? {
            let mut child: Option<Element> = None;
            match token.event {
                Event::Start(ref start) | Event::Empty(ref start) => {
                    let empty = matches!(token.event, Event::Empty(_));
                    self.scope.start(start)?;
                    let element = Element::of(start, &self.scope, self.index, empty)?;
                    if !root {
                        if !element.main || element.local != b"worksheet" {
                            return Err(NOT_WORKSHEET);
                        }
                        root = true;
                        if empty {
                            self.scope.end();
                        }
                        continue;
                    }
                    if empty {
                        self.scope.end();
                    }
                    child = Some(element);
                }
                Event::End(_) => {
                    // Only the root is entered without being consumed.
                    self.scope.end();
                }
                _ => {}
            }
            if let Some(element) = child {
                self.root_child(stream, element)?;
            }
        }
        if !root {
            return Err(NOT_WORKSHEET);
        }
        Ok(())
    }

    /// One child of the root, its start tag given to the scope, read or
    /// passed over through its end tag.
    fn root_child<R: Read>(
        &mut self,
        stream: &mut Stream<R>,
        element: Element,
    ) -> Result<(), Error> {
        let known = READ_CHILDREN
            .iter()
            .find(|(name, _)| element.main && *name == element.local.as_slice());
        let Some(&(name, message)) = known else {
            if !element.empty {
                skip_subtree(stream, &mut self.scope, self.index)?;
            }
            return Ok(());
        };
        if self.seen.contains(&name) {
            return Err(match message {
                0 => TWO_ELEMENTS,
                1 => TWO_SHEET_ELEMENTS,
                _ => TWO_SETTINGS,
            });
        }
        self.seen.push(name);
        let settings = &mut self.settings;
        match name {
            b"sheetFormatPr" => {
                settings.outline_max = (
                    element.level(b"outlineLevelRow")?,
                    element.level(b"outlineLevelCol")?,
                );
            }
            b"sheetProtection" => {
                // Each flag its attribute, else the schema's default: the
                // writer omits an attribute equal to it, so its own form
                // reads back as written.
                let mut protection = SheetProtection::default();
                for (attribute, _, schema) in SheetProtection::default().options() {
                    let value = element.flag(attribute)?.unwrap_or(schema);
                    *protection.field_mut(attribute) = value;
                }
                settings.protection = Some(protection);
            }
            b"autoFilter" => {
                settings.auto_filter = Some(element.area(b"ref")?.ok_or(SETTING_UNREADABLE)?);
            }
            b"printOptions" => {
                settings.print_options = PrintOptions {
                    horizontal_centered: element.flag(b"horizontalCentered")?,
                    vertical_centered: element.flag(b"verticalCentered")?,
                    headings: element.flag(b"headings")?,
                    grid_lines: element.flag(b"gridLines")?,
                };
            }
            b"pageMargins" => {
                let margins = PageMargins {
                    left: element.number(b"left")?,
                    right: element.number(b"right")?,
                    top: element.number(b"top")?,
                    bottom: element.number(b"bottom")?,
                    header: element.number(b"header")?,
                    footer: element.number(b"footer")?,
                };
                if margins
                    .fields()
                    .iter()
                    .any(|(_, value, _)| value.is_some_and(|inches| inches < 0.0))
                {
                    return Err(SETTING_UNREADABLE);
                }
                settings.margins = Some(margins);
            }
            b"pageSetup" => {
                let use_first = element.flag(b"useFirstPageNumber")?.unwrap_or(false);
                let setup = PageSetup {
                    orientation: match element.value(b"orientation") {
                        None | Some("default") => None,
                        Some(text) => Some(
                            [Orientation::Portrait, Orientation::Landscape]
                                .into_iter()
                                .find(|o| o.keyword() == text.as_bytes())
                                .ok_or(SETTING_UNREADABLE)?,
                        ),
                    },
                    paper_size: element.whole(b"paperSize")?,
                    scale: element.whole(b"scale")?,
                    fit_to_width: element.whole(b"fitToWidth")?,
                    fit_to_height: element.whole(b"fitToHeight")?,
                    fit_to_page: None,
                    first_page_number: match use_first {
                        true => element.whole(b"firstPageNumber")?,
                        false => None,
                    },
                    black_and_white: element.flag(b"blackAndWhite")?,
                    draft: element.flag(b"draft")?,
                    copies: element.whole(b"copies")?,
                    page_order: match element.value(b"pageOrder") {
                        None => None,
                        Some(text) => Some(
                            [PageOrder::DownThenOver, PageOrder::OverThenDown]
                                .into_iter()
                                .find(|o| o.keyword() == text.as_bytes())
                                .ok_or(SETTING_UNREADABLE)?,
                        ),
                    },
                };
                if !setup.in_range() {
                    return Err(SETTING_UNREADABLE);
                }
                settings.page_setup = setup;
            }
            _ => {}
        }
        if element.empty {
            return Ok(());
        }
        // The elements with children read them; the rest are passed over.
        match name {
            b"sheetPr" => self.sheet_pr(stream),
            b"sheetViews" => self.sheet_views(stream),
            b"mergeCells" => self.merges(stream),
            b"hyperlinks" => self.hyperlinks(stream),
            b"headerFooter" => self.header_footer(stream),
            _ => skip_subtree(stream, &mut self.scope, self.index),
        }
    }

    fn sheet_pr<R: Read>(&mut self, stream: &mut Stream<R>) -> Result<(), Error> {
        while let Some(child) = step(stream, &mut self.scope, self.index)? {
            let settings = &mut self.settings;
            match child.local.as_slice() {
                b"tabColor" if child.main => {
                    let color = child.color()?;
                    if let Color::Theme {
                        tint: Some(tint), ..
                    } = color
                        && !(-1.0..=1.0).contains(&tint)
                    {
                        return Err(SETTING_UNREADABLE);
                    }
                    settings.tab_color = Some(color);
                }
                b"outlinePr" if child.main => {
                    settings.outline_summary =
                        (child.flag(b"summaryBelow")?, child.flag(b"summaryRight")?);
                }
                b"pageSetUpPr" if child.main => {
                    settings.fit_to_page = child.flag(b"fitToPage")?;
                }
                _ => {}
            }
            if !child.empty {
                skip_subtree(stream, &mut self.scope, self.index)?;
            }
        }
        Ok(())
    }

    fn sheet_views<R: Read>(&mut self, stream: &mut Stream<R>) -> Result<(), Error> {
        let mut first = true;
        while let Some(child) = step(stream, &mut self.scope, self.index)? {
            if first && child.main && child.local == b"sheetView" {
                first = false;
                let view = SheetView {
                    zoom: child.whole(b"zoomScale")?,
                    show_grid_lines: child.flag(b"showGridLines")?,
                    show_headings: child.flag(b"showRowColHeaders")?,
                    show_zeros: child.flag(b"showZeros")?,
                    show_formulas: child.flag(b"showFormulas")?,
                    right_to_left: child.flag(b"rightToLeft")?,
                    view: match child.value(b"view") {
                        None => None,
                        Some(text) => Some(
                            [
                                ViewKind::Normal,
                                ViewKind::PageBreakPreview,
                                ViewKind::PageLayout,
                            ]
                            .into_iter()
                            .find(|kind| kind.keyword() == text.as_bytes())
                            .ok_or(SETTING_UNREADABLE)?,
                        ),
                    },
                };
                if view.zoom.is_some_and(|zoom| !(10..=400).contains(&zoom)) {
                    return Err(SETTING_UNREADABLE);
                }
                self.settings.view = view;
                if !child.empty {
                    self.pane(stream)?;
                }
            } else if !child.empty {
                skip_subtree(stream, &mut self.scope, self.index)?;
            }
        }
        Ok(())
    }

    /// The children of the first `<sheetView>`: its first `<pane>`.
    fn pane<R: Read>(&mut self, stream: &mut Stream<R>) -> Result<(), Error> {
        let mut first = true;
        while let Some(child) = step(stream, &mut self.scope, self.index)? {
            if first && child.main && child.local == b"pane" {
                first = false;
                if child.value(b"state") == Some("frozen") {
                    self.settings.freeze = match child.value(b"topLeftCell") {
                        None => None,
                        Some(text) => {
                            Some(CellRef::parse(text.as_bytes()).ok_or(SETTING_UNREADABLE)?)
                        }
                    };
                }
            }
            if !child.empty {
                skip_subtree(stream, &mut self.scope, self.index)?;
            }
        }
        Ok(())
    }

    fn merges<R: Read>(&mut self, stream: &mut Stream<R>) -> Result<(), Error> {
        while let Some(child) = step(stream, &mut self.scope, self.index)? {
            if child.main && child.local == b"mergeCell" {
                let area = child.area(b"ref")?.ok_or(SETTING_UNREADABLE)?;
                self.settings.merges.push(area);
            }
            if !child.empty {
                skip_subtree(stream, &mut self.scope, self.index)?;
            }
        }
        Ok(())
    }

    fn hyperlinks<R: Read>(&mut self, stream: &mut Stream<R>) -> Result<(), Error> {
        while let Some(child) = step(stream, &mut self.scope, self.index)? {
            if child.main && child.local == b"hyperlink" {
                let area = child.area(b"ref")?.ok_or(SETTING_UNREADABLE)?;
                self.links.push(Link {
                    cell: area.first,
                    id: child.relationship.clone(),
                    location: child.value(b"location").map(str::to_owned),
                    display: child.value(b"display").map(str::to_owned),
                    tooltip: child.value(b"tooltip").map(str::to_owned),
                });
            }
            if !child.empty {
                skip_subtree(stream, &mut self.scope, self.index)?;
            }
        }
        Ok(())
    }

    fn header_footer<R: Read>(&mut self, stream: &mut Stream<R>) -> Result<(), Error> {
        while let Some(child) = step(stream, &mut self.scope, self.index)? {
            let at = HEADER_FOOTER_CHILDREN
                .iter()
                .position(|name| child.main && *name == child.local.as_slice());
            match at {
                Some(at) => {
                    let text = match child.empty {
                        true => String::new(),
                        false => text_of(stream, &mut self.scope, self.index)?,
                    };
                    if text.chars().count() > 255 || text.chars().any(forbidden_in_xml) {
                        return Err(SETTING_UNREADABLE);
                    }
                    let texts = &mut self.settings.header_footer;
                    let slot = [
                        &mut texts.odd_header,
                        &mut texts.odd_footer,
                        &mut texts.even_header,
                        &mut texts.even_footer,
                        &mut texts.first_header,
                        &mut texts.first_footer,
                    ]
                    .into_iter()
                    .nth(at)
                    .ok_or(SETTING_UNREADABLE)?;
                    *slot = Some(text);
                }
                None if !child.empty => skip_subtree(stream, &mut self.scope, self.index)?,
                None => {}
            }
        }
        Ok(())
    }
}

impl SheetProtection {
    /// The field an attribute of [`SheetProtection::options`] names.
    fn field_mut(&mut self, attribute: &[u8]) -> &mut bool {
        match attribute {
            b"sheet" => &mut self.sheet,
            b"objects" => &mut self.objects,
            b"scenarios" => &mut self.scenarios,
            b"formatCells" => &mut self.format_cells,
            b"formatColumns" => &mut self.format_columns,
            b"formatRows" => &mut self.format_rows,
            b"insertColumns" => &mut self.insert_columns,
            b"insertRows" => &mut self.insert_rows,
            b"insertHyperlinks" => &mut self.insert_hyperlinks,
            b"deleteColumns" => &mut self.delete_columns,
            b"deleteRows" => &mut self.delete_rows,
            b"selectLockedCells" => &mut self.select_locked_cells,
            b"sort" => &mut self.sort,
            b"autoFilter" => &mut self.auto_filter,
            b"pivotTables" => &mut self.pivot_tables,
            _ => &mut self.select_unlocked_cells,
        }
    }
}

/// A `<col>`'s format.
fn column_format(element: &Element) -> Result<ColumnFormat, Error> {
    let width = element.number(b"width")?;
    if width.is_some_and(|width| width > 255.0) {
        return Err(SETTING_UNREADABLE);
    }
    Ok(ColumnFormat {
        first: element.whole(b"min")?.ok_or(SETTING_UNREADABLE)?,
        last: element.whole(b"max")?.ok_or(SETTING_UNREADABLE)?,
        width,
        custom_width: element.flag(b"customWidth")?.unwrap_or(false),
        hidden: element.flag(b"hidden")?.unwrap_or(false),
        style: element.whole(b"style")?,
        outline_level: element.level(b"outlineLevel")?,
        collapsed: element.flag(b"collapsed")?.unwrap_or(false),
    })
}

/// A `<row>`'s format, numbered `row`.
fn row_format(element: &Element, row: u32) -> Result<RowFormat, Error> {
    let height = element.number(b"ht")?;
    if height.is_some_and(|height| height > 409.0) {
        return Err(SETTING_UNREADABLE);
    }
    Ok(RowFormat {
        row,
        height,
        custom_height: element.flag(b"customHeight")?.unwrap_or(false),
        hidden: element.flag(b"hidden")?.unwrap_or(false),
        style: element.whole(b"s")?,
        custom_format: element.flag(b"customFormat")?.unwrap_or(false),
        outline_level: element.level(b"outlineLevel")?,
        collapsed: element.flag(b"collapsed")?.unwrap_or(false),
    })
}

/// An element met: whether it is in the main namespace, its local name, its
/// unprefixed attributes unescaped and normalised, its `r:id` when it has
/// one, and whether it was an empty-element tag.
struct Element {
    main: bool,
    local: Vec<u8>,
    attributes: Vec<(Vec<u8>, String)>,
    relationship: Option<Vec<u8>>,
    empty: bool,
}

impl Element {
    fn of(start: &BytesStart<'_>, scope: &Scope, index: usize, empty: bool) -> Result<Self, Error> {
        let (namespace, local) = scope.resolve_element(start.name())?;
        let main = MAIN.contains(&namespace.unwrap_or_default());
        let local = local.to_vec();
        let mut attributes = Vec::new();
        let mut relationship = None;
        for attribute in start.attributes() {
            let attribute = attribute.map_err(|_| Error::Xml(index))?;
            if attribute.key.as_namespace_binding().is_some() {
                continue;
            }
            let (namespace, name) = scope.resolve_attribute(attribute.key)?;
            match namespace {
                None => {
                    let value = attribute
                        .normalized_value(XmlVersion::Implicit1_0)
                        .map_err(|_| Error::Xml(index))?
                        .into_owned();
                    attributes.push((name.to_vec(), value));
                }
                Some(namespace)
                    if RELATIONSHIPS_ATTRIBUTE.contains(&namespace) && name == b"id" =>
                {
                    relationship = Some(attribute.value.as_bytes().to_vec());
                }
                Some(_) => {}
            }
        }
        Ok(Self {
            main,
            local,
            attributes,
            relationship,
            empty,
        })
    }

    fn value(&self, name: &[u8]) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }

    /// A flag: `1` or `true`, `0` or `false`.
    fn flag(&self, name: &[u8]) -> Result<Option<bool>, Error> {
        match self.value(name) {
            None => Ok(None),
            Some("1") | Some("true") => Ok(Some(true)),
            Some("0") | Some("false") => Ok(Some(false)),
            Some(_) => Err(SETTING_UNREADABLE),
        }
    }

    /// A decimal `u32`.
    fn whole(&self, name: &[u8]) -> Result<Option<u32>, Error> {
        self.value(name)
            .map(|text| decimal(text).ok_or(SETTING_UNREADABLE))
            .transpose()
    }

    /// An outline level, `0..=7`, 0 when absent.
    fn level(&self, name: &[u8]) -> Result<u8, Error> {
        match self.whole(name)? {
            None => Ok(0),
            Some(level) if level <= 7 => Ok(level as u8),
            Some(_) => Err(SETTING_UNREADABLE),
        }
    }

    /// A finite number at least zero.
    fn number(&self, name: &[u8]) -> Result<Option<f64>, Error> {
        self.value(name)
            .map(|text| {
                text.parse::<f64>()
                    .ok()
                    .filter(|number| number.is_finite() && *number >= 0.0)
                    .ok_or(SETTING_UNREADABLE)
            })
            .transpose()
    }

    /// A `ref` that is an area, a single cell included.
    fn area(&self, name: &[u8]) -> Result<Option<Area>, Error> {
        self.value(name)
            .map(|text| Area::parse(text.as_bytes()).ok_or(SETTING_UNREADABLE))
            .transpose()
    }

    /// A colour element's colour, its form as a font's colour is read.
    fn color(&self) -> Result<Color, Error> {
        let owned = |name: &[u8]| self.value(name).map(str::to_owned);
        color_from(
            ColorAttributes {
                rgb: owned(b"rgb"),
                theme: owned(b"theme"),
                tint: owned(b"tint"),
                indexed: owned(b"indexed"),
                auto: owned(b"auto"),
            },
            refusal,
        )
    }
}

/// The next child of the element being read, its start tag given to the
/// scope (and given back for an empty one), or `None` at the element's own
/// end tag. Text and comments between the children are passed over.
fn step<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
) -> Result<Option<Element>, Error> {
    loop {
        let Some(token) = stream.next()? else {
            return Err(Error::Xml(index));
        };
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start)?;
                let element = Element::of(start, scope, index, empty)?;
                if empty {
                    scope.end();
                }
                return Ok(Some(element));
            }
            Event::End(_) => {
                scope.end();
                return Ok(None);
            }
            _ => {}
        }
    }
}
