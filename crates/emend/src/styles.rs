//! Reading what a workbook's styles part defines: its number formats and its
//! cell formats.
//!
//! `specs/0012-styles.md`, "Module `styles`: what is read". [`Styles`] holds
//! the `numFmt` children of `<numFmts>` and the `xf` children of `<cellXfs>`,
//! both in part order, so `cell_xfs[i]` is what a cell's `s="i"` names. Every
//! other element of the part is walked past with its subtree: fonts, fills,
//! borders, `cellStyleXfs`, `cellStyles`, `dxfs`, table styles, colours and
//! extensions are not modelled here, and nothing about them is read.
//!
//! [`crate::package::Package::styles`] reads the part once and keeps the
//! answer, as [`crate::package::Package::workbook`] does.
//!
//! # Number formats
//!
//! A cell format names its number format by id. An id of 164 or more is a
//! custom format the part must define in `<numFmts>`; an id below 164 is one
//! of the built-in formats of ECMA-376 Part 1, 18.8.30, which a producer may
//! also spell out in `<numFmts>`, and when it does the spelled-out code is
//! the answer [`Styles::number_format`] gives. The built-in ids that table
//! defines:
//!
//! - 0 `General`;
//! - 1 to 4, integer and two-decimal forms with and without a thousands
//!   separator;
//! - 9 and 10, percentages; 11 and 48, scientific; 12 and 13, fractions;
//! - 14 to 17, dates; 18 to 21, times; 22, a date and a time;
//! - 37 to 40, accounting-like integer forms; 45 to 47, durations;
//! - 49, `@` (text);
//! - 5 to 8, 23 to 26 and 41 to 44, and the ranges 27 to 36 and 50 to 58,
//!   locale-defined.
//!
//! This layer does not check that an id below 164 is one the table defines,
//! and what a consumer shows for one it does not is not this layer's to say.
//!
//! A `formatCode` is read as a sheet name is, through
//! `normalized_value(XmlVersion::Implicit1_0)` (`specs/0009-package-layer.md`,
//! P2 ruling 2; `specs/0012-styles.md`, P1 ruling 3): a reference such as
//! `&#9;` becomes the character it stands for, and a literal tab, line feed
//! or carriage return in the attribute becomes a space, as XML 1.0 section
//! 3.3.3 says a reader normalises an attribute value.
//!
//! # Errors
//!
//! This module raises [`Error::Package`] with the fixed strings catalogued in
//! [`crate::package`]'s module header, each beside the test that pins it, and
//! keeps no list of its own. [`Error::Xml`] carries the entry's index, as in
//! every part.
//!
//! # Limits
//!
//! - The part is held as [`Styles`]: every `numFmt`'s id and code, and every
//!   `xf`'s five ids and its own bytes. That is bounded by the part's size,
//!   which is 160215 bytes for the corpus's largest; a styles part larger than
//!   the address space cannot be read.
//! - The apply flags of an `xf`, its `alignment` and its `protection` are not
//!   modelled. Its bytes are kept, privately, so that a new cell format can
//!   be made as a copy of one.
//! - A format code is text to this layer: which numbers it shows as dates is
//!   the caller's to decide.

use std::io::Read;

use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};

use crate::archive::Error;
use crate::cells::decimal;
use crate::package::MAIN;
use crate::xml::{Scope, Stream};

#[cfg(test)]
mod tests;

/// What a workbook's styles part defines, as far as this layer reads it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Styles {
    /// The `numFmt` children of `<numFmts>`, in part order.
    pub num_fmts: Vec<NumFmt>,
    /// The `xf` children of `<cellXfs>`, in part order: `cell_xfs[i]` is what
    /// a cell's `s="i"` names.
    pub cell_xfs: Vec<Xf>,
    /// Whether the part has a `<numFmts>` among the root's children, empty
    /// or not: a rewrite that adds a code creates one only when it has none.
    has_num_fmts: bool,
}

/// One `<numFmt>`: a number format the part defines.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumFmt {
    /// `numFmtId`, a decimal `u32`.
    pub id: u32,
    /// `formatCode`, unescaped and normalised as the module header says.
    pub code: String,
}

/// One `<xf>` of `<cellXfs>`: a cell format.
///
/// Each id is its attribute when present and `None` when absent, as the
/// schema leaves them optional. The element's own bytes are kept privately,
/// so a new cell format can be made as a copy of this one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Xf {
    /// `numFmtId`: the number format, built-in or one of `num_fmts`.
    pub num_fmt_id: Option<u32>,
    /// `fontId`: an index into the part's `<fonts>`.
    pub font_id: Option<u32>,
    /// `fillId`: an index into the part's `<fills>`.
    pub fill_id: Option<u32>,
    /// `borderId`: an index into the part's `<borders>`.
    pub border_id: Option<u32>,
    /// `xfId`: an index into the part's `<cellStyleXfs>`.
    pub xf_id: Option<u32>,
    /// The element as the part writes it: start tag through end tag, or its
    /// one empty-element tag.
    raw: Vec<u8>,
}

impl Xf {
    /// The element's own bytes, as the part writes them.
    #[cfg(test)]
    pub(crate) fn raw(&self) -> &[u8] {
        &self.raw
    }
}

/// A cell format recorded by [`crate::package::Package::add_cell_style`],
/// composed at that call and written by the styles rewrite.
///
/// Held without its element name: the new `xf` takes the qualified-name
/// prefix of the `<cellXfs>` it goes into, which the rewrite reads from the
/// part, so the name is the one thing written at the rewrite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NewXf {
    /// Everything after the name in the start tag, up to and not including
    /// its closing `>` or `/>`: the attributes, each with its leading space.
    attributes: Vec<u8>,
    /// The base's bytes between its start tag and its end tag, `None` when
    /// the new element is written empty.
    content: Option<Vec<u8>>,
}

impl NewXf {
    /// The element, with `prefix` (with its colon, or empty) on its name.
    pub(crate) fn element(&self, prefix: &[u8]) -> Vec<u8> {
        let mut out = [b"<".as_slice(), prefix, b"xf"].concat();
        out.extend_from_slice(&self.attributes);
        match &self.content {
            None => out.extend_from_slice(b"/>"),
            Some(content) => {
                out.push(b'>');
                out.extend_from_slice(content);
                out.extend_from_slice(&[b"</".as_slice(), prefix, b"xf>"].concat());
            }
        }
        out
    }
}

/// A cell format's number format.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumberFormat {
    /// A built-in format by id, one the part does not spell out (see the
    /// module header for the ids ECMA-376 defines).
    BuiltIn(u32),
    /// A format code the part defines in `<numFmts>`, custom or a producer's
    /// own spelling of a built-in id.
    Code(String),
}

impl Styles {
    /// Whether the part has a `<numFmts>`, empty or not.
    pub(crate) fn has_num_fmts(&self) -> bool {
        self.has_num_fmts
    }

    /// The new cell format [`crate::package::Package::add_cell_style`]
    /// records: a copy of the cell format `base` with its number format
    /// changed to `num_fmt_id`, or the form `specs/0012-styles.md` gives for
    /// no base.
    ///
    /// With a base, the start tag is written anew from the base's own bytes:
    /// its attributes in its order, each as a space, the name, `=`, the quote
    /// character the base used for it, the value's bytes as the base wrote
    /// them and that quote again (P1 ruling 5), except that `numFmtId`'s
    /// value is the new id (added first when the base had none) and
    /// `applyNumberFormat`'s is `1` (appended last when the base had none).
    /// The base's content between its start and end tags is kept raw, and a
    /// base with none gives an empty element. `None` when `base` is out of
    /// range, or when the base's start tag does not scan, which a part the
    /// reader accepted cannot give.
    pub(crate) fn derived_xf(&self, base: Option<u32>, num_fmt_id: u32) -> Option<NewXf> {
        let Some(base) = base else {
            return Some(NewXf {
                attributes: format!(
                    " numFmtId=\"{num_fmt_id}\" fontId=\"0\" fillId=\"0\" borderId=\"0\" \
                     xfId=\"0\" applyNumberFormat=\"1\""
                )
                .into_bytes(),
                content: None,
            });
        };
        let raw = &self.cell_xfs.get(usize::try_from(base).ok()?)?.raw;
        let tag = scan_start_tag(raw)?;
        let id = num_fmt_id.to_string();
        let mut attributes = Vec::new();
        let named = |name: &[u8]| tag.attributes.iter().any(|a| &raw[a.name.clone()] == name);
        if !named(b"numFmtId") {
            attributes.extend_from_slice(format!(" numFmtId=\"{id}\"").as_bytes());
        }
        for attribute in &tag.attributes {
            let name = &raw[attribute.name.clone()];
            let value: &[u8] = match name {
                b"numFmtId" => id.as_bytes(),
                b"applyNumberFormat" => b"1",
                _ => &raw[attribute.value.clone()],
            };
            attributes.push(b' ');
            attributes.extend_from_slice(name);
            attributes.push(b'=');
            attributes.push(attribute.quote);
            attributes.extend_from_slice(value);
            attributes.push(attribute.quote);
        }
        if !named(b"applyNumberFormat") {
            attributes.extend_from_slice(b" applyNumberFormat=\"1\"");
        }
        let content = if tag.empty {
            None
        } else {
            // The end tag is the element's last `<`: the reader kept the
            // element's tokens and nothing after them.
            let end = raw.iter().rposition(|byte| *byte == b'<')?;
            Some(raw.get(tag.end..end)?.to_vec()).filter(|content| !content.is_empty())
        };
        Some(NewXf {
            attributes,
            content,
        })
    }

    /// The number format of the cell format `xf`.
    ///
    /// `None` when `xf` is out of range. Else the code of the first `numFmt`
    /// whose id is the cell format's `numFmtId`, an id below 164 that a
    /// producer spelled out included; else [`NumberFormat::BuiltIn`] with the
    /// cell format's id, or `BuiltIn(0)` (`General`) when it has none.
    pub fn number_format(&self, xf: u32) -> Option<NumberFormat> {
        let xf = self.cell_xfs.get(usize::try_from(xf).ok()?)?;
        let id = xf.num_fmt_id.unwrap_or(0);
        Some(match self.num_fmts.iter().find(|format| format.id == id) {
            Some(format) => NumberFormat::Code(format.code.clone()),
            None => NumberFormat::BuiltIn(id),
        })
    }

    /// Reads a styles part.
    ///
    /// The root must be `styleSheet` in the main namespace, either spelling
    /// and any prefix. Under it, the `numFmt` children of `<numFmts>` and the
    /// `xf` children of `<cellXfs>` are read; every other element, at any
    /// depth, is walked past with its subtree. A second `<numFmts>` or
    /// `<cellXfs>` among the root's children is refused: the schema allows
    /// one of each, and a second `<cellXfs>` would make every `s` ambiguous
    /// (`specs/0012-styles.md`, P1 ruling 4).
    pub(crate) fn read<R: Read>(stream: &mut Stream<R>, index: usize) -> Result<Self, Error> {
        let mut scope = Scope::new(index);
        let mut styles = Styles::default();
        let mut root = false;
        let mut depth = 0usize;
        // Which container among the root's children is open, if either.
        let mut open: Option<Container> = None;
        let mut seen_num_fmts = false;
        let mut seen_cell_xfs = false;

        while let Some(token) = stream.next()? {
            // What the token asks for, decided inside the match and acted on
            // after it, because `token` borrows the stream the actions read.
            let mut skip = false;
            let mut xf: Option<(Xf, bool)> = None;
            match token.event {
                Event::Start(ref start) | Event::Empty(ref start) => {
                    let empty = matches!(token.event, Event::Empty(_));
                    depth += 1;
                    scope.start(start)?;
                    let (namespace, local) = scope.resolve_element(start.name())?;
                    let ours = MAIN.contains(&namespace.unwrap_or_default());
                    if !root {
                        if !ours || local != b"styleSheet" {
                            return Err(NOT_STYLE_SHEET);
                        }
                        root = true;
                    } else if depth == 2 && ours && local == b"numFmts" {
                        if seen_num_fmts {
                            return Err(TWO_CONTAINERS);
                        }
                        seen_num_fmts = true;
                        styles.has_num_fmts = true;
                        if !empty {
                            open = Some(Container::NumFmts);
                        }
                    } else if depth == 2 && ours && local == b"cellXfs" {
                        if seen_cell_xfs {
                            return Err(TWO_CONTAINERS);
                        }
                        seen_cell_xfs = true;
                        if !empty {
                            open = Some(Container::CellXfs);
                        }
                    } else if depth == 3
                        && ours
                        && local == b"numFmt"
                        && open == Some(Container::NumFmts)
                    {
                        styles.num_fmts.push(read_num_fmt(start, &scope, index)?);
                        skip = !empty;
                    } else if depth == 3
                        && ours
                        && local == b"xf"
                        && open == Some(Container::CellXfs)
                    {
                        xf = Some((read_xf(start, &scope, index, token.raw)?, empty));
                    } else if !empty {
                        skip = true;
                    }
                    if empty {
                        scope.end();
                        depth -= 1;
                    }
                }
                Event::End(_) => {
                    // Only the root and the two containers are entered
                    // without being consumed whole, so an end tag at depth 2
                    // closes a container.
                    if depth == 2 {
                        open = None;
                    }
                    scope.end();
                    depth = depth.saturating_sub(1);
                }
                // Text, comments and references outside a `numFmt` or an
                // `xf` belong to nothing this layer reads.
                _ => {}
            }
            if let Some((mut xf, empty)) = xf {
                if !empty {
                    consume_subtree(stream, &mut scope, index, Some(&mut xf.raw))?;
                    depth -= 1;
                }
                styles.cell_xfs.push(xf);
            } else if skip {
                consume_subtree(stream, &mut scope, index, None)?;
                depth -= 1;
            }
        }
        if !root {
            return Err(NOT_STYLE_SHEET);
        }
        Ok(styles)
    }
}

/// The two containers this layer reads the children of.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Container {
    NumFmts,
    CellXfs,
}

/// What a styles part is refused with, one message each, catalogued in
/// [`crate::package`]'s module header.
const NOT_STYLE_SHEET: Error = Error::Package("the styles part's root is not styleSheet");
const INCOMPLETE_NUM_FMT: Error = Error::Package("a numFmt lacks numFmtId or formatCode");
const NOT_A_NUMBER: Error = Error::Package("a style attribute is not a number");
const TWO_CONTAINERS: Error =
    Error::Package("the styles part has more than one numFmts or cellXfs");

/// A `<numFmt>` element as the styles rewrite writes one: `prefix` (with its
/// colon, or empty) on the name, and the code escaped with the crate's
/// `escape`, then a tab, a line feed and a carriage return as character
/// references, so the code reads back equal to itself through the
/// attribute-value normalisation the reader applies (P1 ruling 3).
pub(crate) fn num_fmt_element(prefix: &[u8], id: u32, code: &str) -> Vec<u8> {
    let escaped = quick_xml::escape::escape(code)
        .replace('\t', "&#9;")
        .replace('\n', "&#10;")
        .replace('\r', "&#13;");
    let mut out = [b"<".as_slice(), prefix, b"numFmt numFmtId=\""].concat();
    out.extend_from_slice(id.to_string().as_bytes());
    out.extend_from_slice(b"\" formatCode=\"");
    out.extend_from_slice(escaped.as_bytes());
    out.extend_from_slice(b"\"/>");
    out
}

/// A start tag's bytes with its unprefixed `count` attribute's value replaced
/// by `count`, every other byte kept; the bytes unchanged when there is no
/// `count`.
///
/// # Errors
///
/// [`Error::Package`] "a style attribute is not a number" for a `count` that
/// is not a decimal `u32` as `decimal` reads one; [`Error::Xml`] for a tag
/// that does not scan, which a part the parser accepted cannot give.
pub(crate) fn counted(raw: &[u8], count: u64, index: usize) -> Result<Vec<u8>, Error> {
    let tag = scan_start_tag(raw).ok_or(Error::Xml(index))?;
    for attribute in &tag.attributes {
        if &raw[attribute.name.clone()] != b"count" {
            continue;
        }
        let value =
            std::str::from_utf8(&raw[attribute.value.clone()]).map_err(|_| Error::Xml(index))?;
        decimal(value).ok_or(NOT_A_NUMBER)?;
        let mut out = raw[..attribute.value.start].to_vec();
        out.extend_from_slice(count.to_string().as_bytes());
        out.extend_from_slice(&raw[attribute.value.end..]);
        return Ok(out);
    }
    Ok(raw.to_vec())
}

/// One attribute of a scanned start tag, as positions in its bytes.
struct ScannedAttribute {
    /// The qualified name.
    name: std::ops::Range<usize>,
    /// `"` or `'`.
    quote: u8,
    /// The value between the quotes, still escaped.
    value: std::ops::Range<usize>,
}

/// A scanned start tag or empty-element tag.
struct ScannedTag {
    attributes: Vec<ScannedAttribute>,
    /// The position after the tag's closing `>`.
    end: usize,
    /// Whether it closed with `/>`.
    empty: bool,
}

/// Scans the start tag at the front of `raw`: its attributes with their
/// quote characters and value positions, and where it ends.
///
/// The tag has already been parsed by `quick-xml`, so this only has to find
/// positions in well-formed bytes, which the parser's own attribute iterator
/// does not report; `None` for bytes that are not a start tag.
fn scan_start_tag(raw: &[u8]) -> Option<ScannedTag> {
    let space = |byte: u8| matches!(byte, b' ' | b'\t' | b'\n' | b'\r');
    if raw.first() != Some(&b'<') {
        return None;
    }
    let mut at = 1;
    while at < raw.len() && !space(raw[at]) && raw[at] != b'/' && raw[at] != b'>' {
        at += 1;
    }
    let mut attributes = Vec::new();
    loop {
        while at < raw.len() && space(raw[at]) {
            at += 1;
        }
        match *raw.get(at)? {
            b'>' => {
                return Some(ScannedTag {
                    attributes,
                    end: at + 1,
                    empty: false,
                });
            }
            b'/' => {
                return (raw.get(at + 1) == Some(&b'>')).then_some(ScannedTag {
                    attributes,
                    end: at + 2,
                    empty: true,
                });
            }
            _ => {}
        }
        let name_start = at;
        while at < raw.len() && raw[at] != b'=' && !space(raw[at]) {
            at += 1;
        }
        let name = name_start..at;
        while at < raw.len() && space(raw[at]) {
            at += 1;
        }
        if raw.get(at) != Some(&b'=') {
            return None;
        }
        at += 1;
        while at < raw.len() && space(raw[at]) {
            at += 1;
        }
        let quote = *raw.get(at)?;
        if quote != b'"' && quote != b'\'' {
            return None;
        }
        at += 1;
        let length = raw.get(at..)?.iter().position(|byte| *byte == quote)?;
        attributes.push(ScannedAttribute {
            name,
            quote,
            value: at..at + length,
        });
        at += length + 1;
    }
}

/// Reads a `<numFmt>` start tag: its id, and its code unescaped and
/// normalised.
fn read_num_fmt(start: &BytesStart<'_>, scope: &Scope, index: usize) -> Result<NumFmt, Error> {
    let mut id: Option<String> = None;
    let mut code: Option<String> = None;
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        if namespace.is_some() {
            continue;
        }
        match local {
            // As written: an id is a token the format spells one way, and
            // `decimal` decides below whether it is one.
            b"numFmtId" => id = Some(attribute.value.as_ref().to_owned()),
            b"formatCode" => {
                code = Some(
                    attribute
                        .normalized_value(XmlVersion::Implicit1_0)
                        .map_err(|_| Error::Xml(index))?
                        .into_owned(),
                );
            }
            _ => {}
        }
    }
    let (Some(id), Some(code)) = (id, code) else {
        return Err(INCOMPLETE_NUM_FMT);
    };
    Ok(NumFmt {
        id: decimal(&id).ok_or(NOT_A_NUMBER)?,
        code,
    })
}

/// Reads an `<xf>` start tag's five ids; `raw` is the tag's own bytes, which
/// start the element's.
fn read_xf(start: &BytesStart<'_>, scope: &Scope, index: usize, raw: &[u8]) -> Result<Xf, Error> {
    let mut xf = Xf {
        num_fmt_id: None,
        font_id: None,
        fill_id: None,
        border_id: None,
        xf_id: None,
        raw: raw.to_vec(),
    };
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        if namespace.is_some() {
            continue;
        }
        let slot = match local {
            b"numFmtId" => &mut xf.num_fmt_id,
            b"fontId" => &mut xf.font_id,
            b"fillId" => &mut xf.fill_id,
            b"borderId" => &mut xf.border_id,
            b"xfId" => &mut xf.xf_id,
            _ => continue,
        };
        *slot = Some(decimal(&attribute.value).ok_or(NOT_A_NUMBER)?);
    }
    Ok(xf)
}

/// Consumes an element's subtree, to and including its matching end tag,
/// appending every token's bytes to `into` when there is one.
///
/// Called with the start tag already given to `scope`, and gives it back.
/// Every start tag inside is given to `scope` too, so a namespace declared
/// inside the subtree is dropped where it should be.
///
/// # Errors
///
/// [`Error::Xml`] when the part ends before the end tag, and whatever
/// [`Scope::start`] refuses.
fn consume_subtree<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    mut into: Option<&mut Vec<u8>>,
) -> Result<(), Error> {
    let mut depth = 0usize;
    while let Some(token) = stream.next()? {
        if let Some(into) = into.as_deref_mut() {
            into.extend_from_slice(token.raw);
        }
        match token.event {
            Event::Start(ref start) => {
                scope.start(start)?;
                depth += 1;
            }
            Event::Empty(ref start) => {
                scope.start(start)?;
                scope.end();
            }
            Event::End(_) => {
                scope.end();
                if depth == 0 {
                    return Ok(());
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    Err(Error::Xml(index))
}
