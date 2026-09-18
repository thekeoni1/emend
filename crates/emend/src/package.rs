//! The Open Packaging Conventions layer: content types, relationships, sheets.
//!
//! `specs/0009-package-layer.md`, "Module `package`". A [`Package`] wraps an
//! [`Archive`] and parses nothing until asked: [`Package::content_types`]
//! reads `[Content_Types].xml`, [`Package::relationships`] reads one part's
//! `.rels`, and [`Package::workbook`] follows the package's `officeDocument`
//! relationship to the workbook part and maps its sheets to theirs. Each
//! answer is parsed once and kept. Nothing here reads a cell, and nothing
//! here writes: the edits and `write_to` are P3.
//!
//! Every name is bytes, compared byte for byte, because a zip entry's name is
//! bytes and the archive layer indexes it that way. No percent-decoding and
//! no case folding, except that a content type `Default`'s extension is
//! compared ASCII case-insensitively, as the convention says. The corpus
//! needs neither of the two the layer refuses: of its 644 internal
//! relationship targets every one resolves to an entry by exact bytes, and
//! none carries `%` or `&` (Design, "Limits"; the survey measures both
//! rules rather than assuming them, through its `unresolved` and `case_only`
//! keys).
//!
//! # The namespaces
//!
//! The two package namespaces have one form each. Everything from the
//! spreadsheet vocabulary has two, the transitional and the strict, and a
//! lookup tries them in that order: [`MAIN`], [`RELATIONSHIPS_ATTRIBUTE`] and
//! the eight relationship types below are pairs for that reason.
//!
//! # Errors
//!
//! [`Error::Xml`] carries the entry's index and comes from [`Stream`] or from
//! an attribute the layer interprets (a namespace URI, a sheet name), never
//! from a byte of the workbook's content.
//!
//! [`Error::Package`] carries one of these fixed strings and nothing else:
//!
//! - "no `[Content_Types].xml` part" (`open_reads_no_xml`,
//!   `content_types_part_absent`).
//! - "a content types part whose root is not Types" (`root_wrong_in_types`).
//! - "a relationships part whose root is not Relationships"
//!   (`root_wrong_in_rels`).
//! - "two relationships with the same id" (`two_relationships_one_id`).
//! - "a relationship target outside the package" (`target_above_root`).
//! - "an external relationship has no part" (`external_target`).
//! - "no officeDocument relationship" (`no_office_document`).
//! - "more than one officeDocument relationship" (`two_office_documents`).
//! - "the officeDocument relationship names no entry"
//!   (`office_document_target_absent`).
//! - "a workbook part whose root is not workbook" (`root_wrong_in_workbook`).
//! - "a sheet element lacks name, sheetId or r:id" (`sheet_lacks_rel_id`).
//! - "a sheetId that is not a number" (`sheet_id_not_a_number`).
//! - "a sheet's relationship is missing" (`sheet_relationship_missing`).
//! - "an element or attribute uses a prefix its document does not bind",
//!   raised by [`Scope`] and reached from here through an `r:id` whose prefix
//!   nothing binds (`rel_id_prefix_unbound`).

#[cfg(test)]
mod tests;

use std::io::{Read, Seek};

use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};

use crate::archive::{Archive, Error};
use crate::xml::{Scope, Stream};

/// The part every package must have, by exact bytes.
pub const CONTENT_TYPES_PART: &[u8] = b"[Content_Types].xml";

/// The namespace of `[Content_Types].xml` (OPC, part 2).
pub const CONTENT_TYPES_NS: &[u8] = b"http://schemas.openxmlformats.org/package/2006/content-types";

/// The namespace of a `.rels` part (OPC, part 2).
pub const RELATIONSHIPS_NS: &[u8] = b"http://schemas.openxmlformats.org/package/2006/relationships";

/// The main spreadsheet namespace, transitional then strict.
pub const MAIN: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/spreadsheetml/2006/main",
    b"http://purl.oclc.org/ooxml/spreadsheetml/main",
];

/// The namespace of the `r:id` attribute, transitional then strict.
pub const RELATIONSHIPS_ATTRIBUTE: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships",
];

/// The `officeDocument` relationship type, transitional then strict.
pub const OFFICE_DOCUMENT: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument",
];

/// The `worksheet` relationship type, transitional then strict.
pub const WORKSHEET: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet",
];

/// The `chartsheet` relationship type, transitional then strict.
pub const CHARTSHEET: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/chartsheet",
];

/// The `dialogsheet` relationship type, transitional then strict.
pub const DIALOGSHEET: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/dialogsheet",
];

/// The `sharedStrings` relationship type, transitional then strict.
pub const SHARED_STRINGS: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/sharedStrings",
];

/// The `styles` relationship type, transitional then strict.
pub const STYLES: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/styles",
];

/// The `calcChain` relationship type, transitional then strict.
pub const CALC_CHAIN: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/calcChain",
];

/// The `theme` relationship type, transitional then strict.
pub const THEME: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/theme",
];

/// One `Default` element of `[Content_Types].xml`, bytes as written.
///
/// The name shadows the prelude's `Default` trait inside this module, which
/// is why nothing here names that trait. `specs/0009-package-layer.md` fixes
/// the type's name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Default {
    /// The `Extension` attribute, without a dot.
    pub extension: Vec<u8>,
    /// The `ContentType` attribute.
    pub content_type: Vec<u8>,
}

/// One `Override` element of `[Content_Types].xml`, bytes as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Override {
    /// The `PartName` attribute, its leading slash kept.
    pub part_name: Vec<u8>,
    /// The `ContentType` attribute.
    pub content_type: Vec<u8>,
}

/// A package's content types, in document order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentTypes {
    defaults: Vec<Default>,
    overrides: Vec<Override>,
}

impl ContentTypes {
    /// Every `Default`, in document order.
    pub fn defaults(&self) -> &[Default] {
        &self.defaults
    }

    /// Every `Override`, in document order.
    pub fn overrides(&self) -> &[Override] {
        &self.overrides
    }

    /// The content type of the entry `name`, or `None`.
    ///
    /// The `Override` whose part name is `/` followed by `name` byte for
    /// byte, else the `Default` whose extension equals the bytes after the
    /// name's last `.` compared ASCII case-insensitively (the convention
    /// makes an extension case-insensitive; the corpus writes them in lower
    /// case), else nothing.
    pub fn of(&self, name: &[u8]) -> Option<&[u8]> {
        let mut part_name = Vec::with_capacity(name.len() + 1);
        part_name.push(b'/');
        part_name.extend_from_slice(name);
        if let Some(over) = self
            .overrides
            .iter()
            .find(|over| over.part_name == part_name)
        {
            return Some(&over.content_type);
        }
        let dot = name.iter().rposition(|byte| *byte == b'.')?;
        let extension = &name[dot + 1..];
        self.defaults
            .iter()
            .find(|default| default.extension.eq_ignore_ascii_case(extension))
            .map(|default| default.content_type.as_slice())
    }
}

/// One `Relationship` element of a `.rels` part, bytes as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relationship {
    /// The `Id` attribute.
    pub id: Vec<u8>,
    /// The `Type` attribute, the full URI.
    pub kind: Vec<u8>,
    /// The `Target` attribute, as written: absolute, relative or external.
    pub target: Vec<u8>,
    /// Whether `TargetMode` is `External`, by exact bytes.
    pub external: bool,
}

/// The relationships of one part, or of the package itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relationships {
    source: Vec<u8>,
    list: Vec<Relationship>,
}

impl Relationships {
    /// The part these belong to, empty for the package's own.
    pub fn source(&self) -> &[u8] {
        &self.source
    }

    /// Every relationship, in document order.
    pub fn all(&self) -> &[Relationship] {
        &self.list
    }

    /// The relationship with that id, or `None`.
    pub fn by_id(&self, id: &[u8]) -> Option<&Relationship> {
        self.list.iter().find(|rel| rel.id == id)
    }

    /// Every relationship whose `Type` is exactly `kind`, in document order.
    ///
    /// The full URI, not a name: a caller that means one of the eight types
    /// this module knows passes one form of that constant, and tries the
    /// other form after it, as [`Relationships::first_of`] does.
    pub fn of_kind<'s>(&'s self, kind: &[u8]) -> impl Iterator<Item = &'s Relationship> {
        self.list.iter().filter(move |rel| rel.kind == kind)
    }

    /// The first relationship of either form of `kind`, transitional first.
    ///
    /// "The first in document order when there are several"
    /// (`specs/0009-package-layer.md`), and the transitional form before the
    /// strict one, as the namespace rule says.
    pub fn first_of(&self, kind: [&[u8]; 2]) -> Option<&Relationship> {
        kind.into_iter().find_map(|form| self.of_kind(form).next())
    }

    /// The entry name an internal relationship's target resolves to.
    ///
    /// The target without its leading slash when it has one, else the target
    /// joined to the source part's directory; `.` and `..` segments are
    /// resolved either way. The spec's letter resolves them for the relative
    /// case alone, which on this corpus is the same function: no absolute
    /// target carries a dot segment. Resolving both is the weaker assumption,
    /// since a target with `..` in it is not an entry name whichever form it
    /// takes.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "an external relationship has no part", or "a
    /// relationship target outside the package" when `..` climbs above the
    /// root or nothing is left.
    pub fn target_part(&self, rel: &Relationship) -> Result<Vec<u8>, Error> {
        if rel.external {
            return Err(Error::Package("an external relationship has no part"));
        }
        let outside = Error::Package("a relationship target outside the package");
        let mut segments: Vec<&[u8]> = Vec::new();
        let target = match rel.target.strip_prefix(b"/") {
            Some(absolute) => absolute,
            None => {
                // The source's directory, which is nothing when the source is
                // the package itself or an entry at the root.
                if let Some(slash) = self.source.iter().rposition(|byte| *byte == b'/') {
                    for segment in self.source[..slash].split(|byte| *byte == b'/') {
                        if !segment.is_empty() {
                            segments.push(segment);
                        }
                    }
                }
                &rel.target[..]
            }
        };
        for segment in target.split(|byte| *byte == b'/') {
            match segment {
                // An empty segment is a doubled or trailing slash, and `.` is
                // the directory itself: neither moves.
                b"" | b"." => {}
                b".." => {
                    if segments.pop().is_none() {
                        return Err(outside);
                    }
                }
                _ => segments.push(segment),
            }
        }
        if segments.is_empty() {
            return Err(outside);
        }
        let mut name = Vec::new();
        for (at, segment) in segments.iter().enumerate() {
            if at > 0 {
                name.push(b'/');
            }
            name.extend_from_slice(segment);
        }
        Ok(name)
    }
}

/// What a sheet's relationship says it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SheetKind {
    /// A `worksheet` relationship.
    Worksheet,
    /// A `chartsheet` relationship.
    Chartsheet,
    /// A `dialogsheet` relationship.
    Dialogsheet,
    /// Anything else, recorded rather than refused.
    Other,
}

/// One `<sheet>` of the workbook's `<sheets>`, in document order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sheet {
    /// The `name` attribute, unescaped and normalized.
    pub name: String,
    /// The `sheetId` attribute, a decimal `u32`.
    pub sheet_id: u32,
    /// The `r:id` attribute, bytes as written.
    pub rel_id: Vec<u8>,
    /// The `state` attribute, bytes as written, absent when not written.
    pub state: Option<Vec<u8>>,
    /// The entry name the sheet's relationship resolves to.
    pub part: Vec<u8>,
    /// The kind that relationship's `Type` says.
    pub kind: SheetKind,
}

/// The workbook part and what it points at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workbook {
    /// The workbook part's entry name.
    pub part: Vec<u8>,
    /// Its sheets, in `<sheets>` order.
    pub sheets: Vec<Sheet>,
    /// The resolved part of its first `sharedStrings` relationship.
    pub shared_strings: Option<Vec<u8>>,
    /// The resolved part of its first `styles` relationship.
    pub styles: Option<Vec<u8>>,
    /// The resolved part of its first `calcChain` relationship.
    pub calc_chain: Option<Vec<u8>>,
    /// The resolved part of its first `theme` relationship.
    pub theme: Option<Vec<u8>>,
}

impl Workbook {
    /// The sheet with that unescaped name, matched exactly.
    pub fn sheet(&self, name: &str) -> Option<&Sheet> {
        self.sheets.iter().find(|sheet| sheet.name == name)
    }
}

/// A spreadsheet package: the archive, and the parts it names.
///
/// See the module header. Opening parses nothing; each accessor parses on its
/// first call and keeps the answer, so a package whose content types part is
/// malformed still opens.
pub struct Package<R: Read + Seek> {
    archive: Archive<R>,
    content_types: Option<ContentTypes>,
    /// Parsed relationships by source, the package's own under an empty name.
    relationships: Vec<Relationships>,
    workbook: Option<Workbook>,
}

impl<R: Read + Seek> Package<R> {
    /// Opens the archive and parses no XML.
    ///
    /// # Errors
    ///
    /// Whatever [`Archive::open`] refuses.
    pub fn open(reader: R) -> Result<Self, Error> {
        Ok(Package {
            archive: Archive::open(reader)?,
            content_types: None,
            relationships: Vec::new(),
            workbook: None,
        })
    }

    /// The archive underneath.
    pub fn archive(&self) -> &Archive<R> {
        &self.archive
    }

    /// The index of the entry named `name`, by exact bytes.
    pub fn part(&self, name: &[u8]) -> Option<usize> {
        self.archive.index_of(name)
    }

    /// One entry's decompressed bytes.
    ///
    /// # Errors
    ///
    /// Whatever [`Archive::read`] refuses.
    pub fn read_part(&mut self, index: usize) -> Result<impl Read + '_, Error> {
        self.archive.read(index)
    }

    /// The package's content types, parsed on the first call and kept.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no `[Content_Types].xml` part" or "a content types
    /// part whose root is not Types"; [`Error::Xml`] for bytes that are not
    /// well-formed XML.
    pub fn content_types(&mut self) -> Result<&ContentTypes, Error> {
        if self.content_types.is_none() {
            let index = self
                .archive
                .index_of(CONTENT_TYPES_PART)
                .ok_or(Error::Package("no [Content_Types].xml part"))?;
            let parsed = parse_content_types(&mut self.archive, index)?;
            return Ok(self.content_types.insert(parsed));
        }
        // The branch above returns whenever the field was empty.
        Ok(self.content_types.as_ref().unwrap())
    }

    /// The relationships of the part `source`, or of the package when
    /// `source` is empty, parsed on the first call for that source and kept.
    ///
    /// An absent rels part gives an empty set: most parts have none, so that
    /// is an answer, not an error.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a relationships part whose root is not
    /// Relationships" or "two relationships with the same id";
    /// [`Error::Xml`] for bytes that are not well-formed XML.
    pub fn relationships(&mut self, source: &[u8]) -> Result<&Relationships, Error> {
        if let Some(at) = self
            .relationships
            .iter()
            .position(|rels| rels.source == source)
        {
            return Ok(&self.relationships[at]);
        }
        let part = rels_part_name(source);
        let parsed = match self.archive.index_of(&part) {
            Some(index) => parse_relationships(&mut self.archive, index, source.to_vec())?,
            None => Relationships {
                source: source.to_vec(),
                list: Vec::new(),
            },
        };
        self.relationships.push(parsed);
        // The push above leaves at least one.
        Ok(self.relationships.last().unwrap())
    }

    /// The workbook part and its sheets, parsed on the first call and kept.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] for any of the officeDocument, workbook root and
    /// sheet messages the module header lists; [`Error::Xml`] for bytes that
    /// are not well-formed XML.
    pub fn workbook(&mut self) -> Result<&Workbook, Error> {
        if self.workbook.is_none() {
            let parsed = self.parse_workbook()?;
            return Ok(self.workbook.insert(parsed));
        }
        // The branch above returns whenever the field was empty.
        Ok(self.workbook.as_ref().unwrap())
    }

    /// [`Package::workbook`]'s work, once, with nothing cached yet.
    fn parse_workbook(&mut self) -> Result<Workbook, Error> {
        let part = {
            let rels = self.relationships(b"")?;
            let found: Vec<&Relationship> = OFFICE_DOCUMENT
                .into_iter()
                .flat_map(|form| rels.of_kind(form))
                .collect();
            match found.as_slice() {
                [] => return Err(Error::Package("no officeDocument relationship")),
                [rel] => rels.target_part(rel)?,
                // Both forms are counted together: a package that carries a
                // transitional and a strict officeDocument relationship does
                // not say which document is the workbook.
                _ => return Err(Error::Package("more than one officeDocument relationship")),
            }
        };
        let index = self.archive.index_of(&part).ok_or(Error::Package(
            "the officeDocument relationship names no entry",
        ))?;
        let raw = parse_workbook_sheets(&mut self.archive, index)?;
        let rels = self.relationships(&part)?;
        let mut sheets = Vec::with_capacity(raw.len());
        for sheet in raw {
            let rel = rels
                .by_id(&sheet.rel_id)
                .ok_or(Error::Package("a sheet's relationship is missing"))?;
            sheets.push(Sheet {
                name: sheet.name,
                sheet_id: sheet.sheet_id,
                rel_id: sheet.rel_id,
                state: sheet.state,
                part: rels.target_part(rel)?,
                kind: sheet_kind(&rel.kind),
            });
        }
        Ok(Workbook {
            part,
            sheets,
            shared_strings: resolved_first(rels, SHARED_STRINGS)?,
            styles: resolved_first(rels, STYLES)?,
            calc_chain: resolved_first(rels, CALC_CHAIN)?,
            theme: resolved_first(rels, THEME)?,
        })
    }
}

/// The rels part of `source`, or the package's own when `source` is empty.
///
/// `<dir>/<base>` keeps its `<dir>/_rels/<base>.rels`; the package's is
/// `_rels/.rels`, which is the same rule with an empty base.
pub fn rels_part_name(source: &[u8]) -> Vec<u8> {
    let mut name = Vec::with_capacity(source.len() + 13);
    match source.iter().rposition(|byte| *byte == b'/') {
        Some(slash) => {
            name.extend_from_slice(&source[..=slash]);
            name.extend_from_slice(b"_rels/");
            name.extend_from_slice(&source[slash + 1..]);
        }
        None => {
            name.extend_from_slice(b"_rels/");
            name.extend_from_slice(source);
        }
    }
    name.extend_from_slice(b".rels");
    name
}

/// The part name a rels part's own name says its relationships belong to.
///
/// The inverse of [`rels_part_name`], or `None` for an entry that is not
/// named as a rels part. The survey uses it to find the source of every
/// `.rels` entry it sees.
pub fn rels_part_source(name: &[u8]) -> Option<Vec<u8>> {
    let base = name.strip_suffix(b".rels")?;
    let (directory, file) = match base.iter().rposition(|byte| *byte == b'/') {
        Some(slash) => (&base[..=slash], &base[slash + 1..]),
        None => (&base[..0], base),
    };
    let directory = directory.strip_suffix(b"_rels/")?;
    let mut source = Vec::with_capacity(directory.len() + file.len());
    source.extend_from_slice(directory);
    source.extend_from_slice(file);
    Some(source)
}

/// Whether `namespace` is either form of the main spreadsheet namespace.
fn is_main(namespace: Option<&[u8]>) -> bool {
    matches!(namespace, Some(ns) if MAIN.contains(&ns))
}

/// Whether `namespace` is either form of the `r:id` namespace.
fn is_relationships_attribute(namespace: Option<&[u8]>) -> bool {
    matches!(namespace, Some(ns) if RELATIONSHIPS_ATTRIBUTE.contains(&ns))
}

/// The sheet kind a relationship type names.
fn sheet_kind(kind: &[u8]) -> SheetKind {
    if WORKSHEET.contains(&kind) {
        SheetKind::Worksheet
    } else if CHARTSHEET.contains(&kind) {
        SheetKind::Chartsheet
    } else if DIALOGSHEET.contains(&kind) {
        SheetKind::Dialogsheet
    } else {
        SheetKind::Other
    }
}

/// The resolved part of the first relationship of either form of `kind`.
fn resolved_first(rels: &Relationships, kind: [&[u8]; 2]) -> Result<Option<Vec<u8>>, Error> {
    match rels.first_of(kind) {
        Some(rel) => Ok(Some(rels.target_part(rel)?)),
        None => Ok(None),
    }
}

/// The value of the unprefixed attribute `name`, bytes as written.
///
/// Namespace declarations are skipped: `xmlns` and `xmlns:<p>` are not
/// attributes of the element in the sense any caller here means.
fn written_attribute(
    start: &BytesStart<'_>,
    name: &[u8],
    index: usize,
) -> Result<Option<Vec<u8>>, Error> {
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (local, prefix) = attribute.key.decompose();
        if prefix.is_none() && local.into_inner().as_bytes() == name {
            return Ok(Some(attribute.value.as_bytes().to_vec()));
        }
    }
    Ok(None)
}

/// Reads `[Content_Types].xml`.
///
/// A `Default` or `Override` element that does not carry both its attributes
/// declares nothing, so it is recorded as nothing: no message in the spec's
/// catalogue covers one, and the layer reports what the part wrote.
fn parse_content_types<R: Read + Seek>(
    archive: &mut Archive<R>,
    index: usize,
) -> Result<ContentTypes, Error> {
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut scope = Scope::new(index);
    let mut root = false;
    let mut defaults = Vec::new();
    let mut overrides = Vec::new();
    while let Some(token) = stream.next()? {
        let (start, empty) = match token.event {
            Event::Start(ref start) => (start, false),
            Event::Empty(ref start) => (start, true),
            Event::End(_) => {
                scope.end();
                continue;
            }
            _ => continue,
        };
        // A start tag's own declarations bind for that tag, so the scope
        // takes them before its name is resolved; an empty element's are
        // dropped again below.
        scope.start(start)?;
        let (namespace, local) = scope.resolve_element(start.name())?;
        let ours = namespace == Some(CONTENT_TYPES_NS);
        if !root {
            if !ours || local != b"Types" {
                return Err(Error::Package(
                    "a content types part whose root is not Types",
                ));
            }
            root = true;
        } else if ours
            && local == b"Default"
            && let (Some(extension), Some(content_type)) = (
                written_attribute(start, b"Extension", index)?,
                written_attribute(start, b"ContentType", index)?,
            )
        {
            defaults.push(Default {
                extension,
                content_type,
            });
        } else if ours
            && local == b"Override"
            && let (Some(part_name), Some(content_type)) = (
                written_attribute(start, b"PartName", index)?,
                written_attribute(start, b"ContentType", index)?,
            )
        {
            overrides.push(Override {
                part_name,
                content_type,
            });
        }
        if empty {
            scope.end();
        }
    }
    Ok(ContentTypes {
        defaults,
        overrides,
    })
}

/// Reads one `.rels` part, whose relationships belong to `source`.
///
/// As with a content type's elements, a `Relationship` that does not carry
/// all three of `Id`, `Type` and `Target` relates nothing and is recorded as
/// nothing.
fn parse_relationships<R: Read + Seek>(
    archive: &mut Archive<R>,
    index: usize,
    source: Vec<u8>,
) -> Result<Relationships, Error> {
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut scope = Scope::new(index);
    let mut root = false;
    let mut list: Vec<Relationship> = Vec::new();
    while let Some(token) = stream.next()? {
        let (start, empty) = match token.event {
            Event::Start(ref start) => (start, false),
            Event::Empty(ref start) => (start, true),
            Event::End(_) => {
                scope.end();
                continue;
            }
            _ => continue,
        };
        scope.start(start)?;
        let (namespace, local) = scope.resolve_element(start.name())?;
        let ours = namespace == Some(RELATIONSHIPS_NS);
        if !root {
            if !ours || local != b"Relationships" {
                return Err(Error::Package(
                    "a relationships part whose root is not Relationships",
                ));
            }
            root = true;
        } else if ours
            && local == b"Relationship"
            && let (Some(id), Some(kind), Some(target)) = (
                written_attribute(start, b"Id", index)?,
                written_attribute(start, b"Type", index)?,
                written_attribute(start, b"Target", index)?,
            )
        {
            if list.iter().any(|rel| rel.id == id) {
                return Err(Error::Package("two relationships with the same id"));
            }
            let mode = written_attribute(start, b"TargetMode", index)?;
            list.push(Relationship {
                id,
                kind,
                target,
                external: mode.as_deref() == Some(b"External"),
            });
        }
        if empty {
            scope.end();
        }
    }
    Ok(Relationships { source, list })
}

/// One `<sheet>` as the workbook part writes it, before resolution.
struct RawSheet {
    name: String,
    sheet_id: u32,
    rel_id: Vec<u8>,
    state: Option<Vec<u8>>,
}

/// Reads the workbook part's `<sheets>`, skipping everything else.
fn parse_workbook_sheets<R: Read + Seek>(
    archive: &mut Archive<R>,
    index: usize,
) -> Result<Vec<RawSheet>, Error> {
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut scope = Scope::new(index);
    let mut root = false;
    /// The root element's depth, so `<sheets>` is 2 and a `<sheet>` 3.
    const ROOT_DEPTH: usize = 1;
    let mut depth = 0usize;
    let mut sheets_at: Option<usize> = None;
    let mut out = Vec::new();
    while let Some(token) = stream.next()? {
        let (start, empty) = match token.event {
            Event::Start(ref start) => (start, false),
            Event::Empty(ref start) => (start, true),
            Event::End(_) => {
                scope.end();
                if sheets_at == Some(depth) {
                    sheets_at = None;
                }
                depth = depth.saturating_sub(1);
                continue;
            }
            _ => continue,
        };
        depth += 1;
        scope.start(start)?;
        let (namespace, local) = scope.resolve_element(start.name())?;
        let ours = is_main(namespace);
        if !root {
            if !ours || local != b"workbook" {
                return Err(Error::Package("a workbook part whose root is not workbook"));
            }
            root = true;
        } else if ours && local == b"sheets" && depth == ROOT_DEPTH + 1 {
            sheets_at = Some(depth);
        } else if ours && local == b"sheet" && sheets_at == Some(depth - 1) {
            out.push(read_sheet(start, &scope, index)?);
        }
        if empty {
            scope.end();
            if sheets_at == Some(depth) {
                sheets_at = None;
            }
            depth = depth.saturating_sub(1);
        }
    }
    Ok(out)
}

/// Reads one `<sheet>` element's attributes.
fn read_sheet(start: &BytesStart<'_>, scope: &Scope, index: usize) -> Result<RawSheet, Error> {
    let mut name: Option<String> = None;
    let mut sheet_id: Option<Vec<u8>> = None;
    let mut rel_id: Option<Vec<u8>> = None;
    let mut state: Option<Vec<u8>> = None;
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        match local {
            // A sheet name is a value the layer interprets, so it is
            // unescaped and normalized (P2 ruling 2); the rest are bytes as
            // written, and `sheetId` is checked below.
            b"name" if namespace.is_none() => {
                name = Some(
                    attribute
                        .normalized_value(XmlVersion::Implicit1_0)
                        .map_err(|_| Error::Xml(index))?
                        .into_owned(),
                );
            }
            b"sheetId" if namespace.is_none() => {
                sheet_id = Some(attribute.value.as_bytes().to_vec());
            }
            b"state" if namespace.is_none() => {
                state = Some(attribute.value.as_bytes().to_vec());
            }
            b"id" if is_relationships_attribute(namespace) => {
                rel_id = Some(attribute.value.as_bytes().to_vec());
            }
            _ => {}
        }
    }
    let (Some(name), Some(sheet_id), Some(rel_id)) = (name, sheet_id, rel_id) else {
        return Err(Error::Package(
            "a sheet element lacks name, sheetId or r:id",
        ));
    };
    let not_a_number = Error::Package("a sheetId that is not a number");
    if sheet_id.is_empty() || !sheet_id.iter().all(|byte| byte.is_ascii_digit()) {
        return Err(not_a_number);
    }
    let sheet_id = std::str::from_utf8(&sheet_id)
        .ok()
        .and_then(|digits| digits.parse::<u32>().ok())
        .ok_or(not_a_number)?;
    Ok(RawSheet {
        name,
        sheet_id,
        rel_id,
        state,
    })
}
