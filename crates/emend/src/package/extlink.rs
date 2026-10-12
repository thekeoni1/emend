//! A workbook's external links, read as written
//! (`specs/0033-read-breadth-survey.md`, "External links"). The Cargo feature
//! `read`.
//!
//! [`Package::external_links`] reads the workbook part's first
//! `<externalReferences>` in order, each `r:id` resolved through the
//! workbook's `externalLink` relationships, and streams each link part once:
//! the one child that says what the link is (`externalBook`, `ddeLink` or
//! `oleLink`) is kept with the sheet names and the defined-name count, and
//! the cached values (`sheetDataSet`, `ddeItems`, `oleItems`) are walked past
//! with their subtrees. A target is the link part's relationship `Target` as
//! written: nothing is decoded and no file is opened.

use std::io::{Read, Seek};

use super::pivot::{Element, child, items, past, root};
use super::{EXTERNAL_LINK, NO_SUCH_PART, Package, Relationships};
use crate::archive::Error;
use crate::xml::{Scope, Stream};

/// What an external link part, or a workbook reference to one, this layer
/// cannot read into the types is refused with; [`crate::package`]'s
/// catalogue lists it.
pub(crate) const EXTERNAL_LINK_UNREADABLE: Error =
    Error::Package("an external link cannot be read");

/// One external link: its part and what it links to. The Cargo feature
/// `read`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalLink {
    /// The external link part's entry name.
    pub part: Vec<u8>,
    /// What the part links to.
    pub kind: ExternalLinkKind,
}

/// What an external link part links to, by the one child it holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExternalLinkKind {
    /// An `externalBook`: another workbook.
    Book {
        /// The `Target` of the link part's relationship the `r:id` names, as
        /// written.
        target: String,
        /// The `sheetName` `val`s, in order.
        sheet_names: Vec<String>,
        /// The count of `definedName` children.
        defined_names: u32,
    },
    /// A `ddeLink`.
    Dde {
        /// Its `ddeService`.
        service: String,
        /// Its `ddeTopic`.
        topic: String,
    },
    /// An `oleLink`.
    Ole {
        /// Its `progId`.
        prog_id: String,
        /// The `Target` of the link part's relationship the `r:id` names, as
        /// written.
        target: String,
    },
}

impl<R: Read + Seek> Package<R> {
    /// Every external link of the workbook, read once and lent
    /// (`specs/0033-read-breadth-survey.md`, "External links"): the
    /// workbook part's first `<externalReferences>` in order, each
    /// `externalReference`'s `r:id` resolved through the workbook's
    /// relationships; a workbook with none gives an empty slice. The Cargo
    /// feature `read`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "an external link cannot be read" for an
    /// `externalReference` with no `r:id`, or one naming no workbook
    /// relationship or one of another type; a link part whose root is not
    /// `externalLink` in the main namespace, or which holds none or more
    /// than one of `externalBook`, `ddeLink` and `oleLink`; an
    /// `externalBook` or `oleLink` with no `r:id`, or one naming no
    /// relationship of the link part or one whose `TargetMode` is not
    /// `External`; a `sheetName` with no `val`; a `ddeLink` lacking
    /// `ddeService` or `ddeTopic`; an `oleLink` with no `progId`. "no such
    /// part" for a relationship naming no entry; whatever
    /// [`Package::workbook`] and [`Package::relationships`] refuse, "an
    /// external relationship has no part" among them for an `externalLink`
    /// relationship whose `TargetMode` is `External`; "an element or
    /// attribute uses a prefix its document does not bind"; [`Error::Xml`]
    /// for bytes that are not well-formed XML or not UTF-8.
    pub fn external_links(&mut self) -> Result<&[ExternalLink], Error> {
        if self.external_links.is_none() {
            let links = self.read_external_links()?;
            self.external_links = Some(links);
        }
        // Filled above.
        Ok(self.external_links.as_deref().unwrap())
    }

    /// The body of [`Package::external_links`], before the list is kept.
    fn read_external_links(&mut self) -> Result<Vec<ExternalLink>, Error> {
        let workbook = self.workbook()?.part.clone();
        let ids = {
            let index = self.archive.index_of(&workbook).ok_or(NO_SUCH_PART)?;
            let mut stream = Stream::new(self.archive.read(index)?, index)?;
            read_references(&mut stream, index)?
        };
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let mut parts = Vec::with_capacity(ids.len());
        {
            let rels = self.relationships(&workbook)?;
            for id in &ids {
                let rel = rels
                    .by_id(id.as_bytes())
                    .filter(|rel| EXTERNAL_LINK.contains(&rel.kind.as_slice()))
                    .ok_or(EXTERNAL_LINK_UNREADABLE)?;
                parts.push(rels.target_part(rel)?);
            }
        }
        let mut links = Vec::with_capacity(parts.len());
        for part in parts {
            let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
            let rels = self.relationships(&part)?.clone();
            let mut stream = Stream::new(self.archive.read(index)?, index)?;
            let kind = read_link(&mut stream, index, &rels)?;
            links.push(ExternalLink { part, kind });
        }
        Ok(links)
    }
}

/// The `r:id`s of the workbook part's first `<externalReferences>`, in
/// order; the rest of the part is not read.
fn read_references<R: Read>(stream: &mut Stream<R>, index: usize) -> Result<Vec<String>, Error> {
    let mut scope = Scope::new(index);
    let workbook = root(stream, &mut scope, index)?;
    let mut ids = Vec::new();
    if workbook.empty {
        return Ok(ids);
    }
    while let Some(element) = child(stream, &mut scope, index)? {
        if element.is(b"externalReferences") {
            items(
                stream,
                &mut scope,
                index,
                &element,
                b"externalReference",
                |reference| {
                    let id = reference
                        .relationship_id()
                        .ok_or(EXTERNAL_LINK_UNREADABLE)?;
                    ids.push(id.to_owned());
                    Ok(())
                },
            )?;
            break;
        }
        past(stream, &mut scope, index, &element)?;
    }
    Ok(ids)
}

/// The `Target` of the link part's external relationship `element`'s `r:id`
/// names.
fn external_target(element: &Element, rels: &Relationships) -> Result<String, Error> {
    let rel = element
        .relationship_id()
        .and_then(|id| rels.by_id(id.as_bytes()))
        .filter(|rel| rel.external)
        .ok_or(EXTERNAL_LINK_UNREADABLE)?;
    String::from_utf8(rel.target.clone()).map_err(|_| EXTERNAL_LINK_UNREADABLE)
}

/// Reads an `externalLink` part.
fn read_link<R: Read>(
    stream: &mut Stream<R>,
    index: usize,
    rels: &Relationships,
) -> Result<ExternalLinkKind, Error> {
    let mut scope = Scope::new(index);
    let root = root(stream, &mut scope, index)?;
    if !root.is(b"externalLink") {
        return Err(EXTERNAL_LINK_UNREADABLE);
    }
    let mut kind = None;
    if !root.empty {
        while let Some(element) = child(stream, &mut scope, index)? {
            let read = if element.is(b"externalBook") {
                read_book(stream, &mut scope, index, &element, rels)?
            } else if element.is(b"ddeLink") {
                let service = element.value(b"ddeService");
                let topic = element.value(b"ddeTopic");
                let (Some(service), Some(topic)) = (service, topic) else {
                    return Err(EXTERNAL_LINK_UNREADABLE);
                };
                let dde = ExternalLinkKind::Dde {
                    service: service.to_owned(),
                    topic: topic.to_owned(),
                };
                past(stream, &mut scope, index, &element)?;
                dde
            } else if element.is(b"oleLink") {
                let prog_id = element.value(b"progId").ok_or(EXTERNAL_LINK_UNREADABLE)?;
                let ole = ExternalLinkKind::Ole {
                    prog_id: prog_id.to_owned(),
                    target: external_target(&element, rels)?,
                };
                past(stream, &mut scope, index, &element)?;
                ole
            } else {
                past(stream, &mut scope, index, &element)?;
                continue;
            };
            if kind.is_some() {
                return Err(EXTERNAL_LINK_UNREADABLE);
            }
            kind = Some(read);
        }
    }
    kind.ok_or(EXTERNAL_LINK_UNREADABLE)
}

/// Reads an `externalBook`, through its end tag.
fn read_book<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    book: &Element,
    rels: &Relationships,
) -> Result<ExternalLinkKind, Error> {
    let target = external_target(book, rels)?;
    let mut sheet_names = Vec::new();
    let mut defined_names = 0u32;
    if !book.empty {
        while let Some(element) = child(stream, scope, index)? {
            if element.is(b"sheetNames") {
                items(stream, scope, index, &element, b"sheetName", |name| {
                    let val = name.value(b"val").ok_or(EXTERNAL_LINK_UNREADABLE)?;
                    sheet_names.push(val.to_owned());
                    Ok(())
                })?;
            } else if element.is(b"definedNames") {
                items(stream, scope, index, &element, b"definedName", |_| {
                    defined_names = defined_names.saturating_add(1);
                    Ok(())
                })?;
            } else {
                past(stream, scope, index, &element)?;
            }
        }
    }
    Ok(ExternalLinkKind::Book {
        target,
        sheet_names,
        defined_names,
    })
}
