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
