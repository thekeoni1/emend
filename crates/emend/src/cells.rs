//! Reading what a worksheet says, without changing anything.
//!
//! `specs/0010-reading-cells.md`, "Module `cells`". P1 is the shared strings
//! table; the cell reader is P2 and the typed views are P3.
//!
//! Nothing here evaluates, converts or writes. A shared string item is the
//! text a producer put there, unescaped and not trimmed: the format says
//! `xml:space="preserve"` on 110010 of the corpus's 110666 `<t>` elements and
//! this reader would keep the spaces either way, because trimming would be a
//! change no caller asked for.
//!
//! # Text and references
//!
//! `quick-xml` 0.42 splits a text run at every general reference: the text
//! either side arrives as [`Event::Text`] and the reference itself as
//! [`Event::GeneralRef`], carrying the name between `&` and `;`. So text is
//! assembled here rather than unescaped in one call. A character reference
//! (`&#65;`, `&#x41;`) is resolved by `quick-xml`; the five references XML
//! predefines are expanded by name; anything else is a reference this layer
//! cannot expand, and it is refused rather than dropped or passed through,
//! since a value with a hole in it is not the value.
//!
//! # Errors
//!
//! This module raises [`Error::Package`] with the fixed strings catalogued in
//! [`crate::package`]'s module header, each beside the test that pins it, and
//! keeps no list of its own: one list for the three modules, checked as a set
//! against all three sources by
//! `package::tests::the_catalogue_and_the_source_hold_the_same_messages`.
//! [`Error::Xml`] carries the entry's index, as in every part.
//!
//! # Limits
//!
//! A shared strings table is held whole: one `String` per item and the `Vec`.
//! The corpus's largest shared strings PART is 2748976 bytes of XML, and its
//! 27 parts hold 110582 items between them (`specs/0010-reading-cells.md`,
//! "Facts planning checked"): a part's byte size and the corpus's item
//! count are two different measurements and neither bounds the other. A
//! consumer on a 32-bit target that meets a table larger than its address
//! space cannot read that workbook; the cells themselves stream, so the
//! table is the only whole-part cost in this module.

use quick_xml::events::{BytesRef, Event};

use crate::archive::Error;
use crate::package::MAIN;
use crate::xml::{Scope, Stream};

use std::io::Read;

/// The shared strings of one workbook, in the order the part writes them.
///
/// A cell of kind `SharedString` carries an index into this table as its
/// value. `count` and `uniqueCount` are ignored: the items are what is there,
/// and the corpus has parts whose attributes disagree with their contents.
///
/// A workbook with no `sharedStrings` relationship gives an empty table, so a
/// cell that names an index in such a workbook is out of range rather than a
/// special case for the caller to remember.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SharedStrings {
    items: Vec<String>,
}

impl SharedStrings {
    /// How many items the table holds.
    ///
    /// `u32` because the format indexes strings with one, and because no
    /// count in this crate's API is `usize` (`specs/0010`, "32-bit and
    /// streaming"). A table longer than `u32::MAX` cannot be indexed by the
    /// format, so the length is saturated rather than wrapped.
    pub fn len(&self) -> u32 {
        u32::try_from(self.items.len()).unwrap_or(u32::MAX)
    }

    /// Whether the table holds no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The item at `index`, or `None` past the end.
    pub fn get(&self, index: u32) -> Option<&str> {
        self.items
            .get(usize::try_from(index).ok()?)
            .map(String::as_str)
    }

    /// Reads a `sharedStrings` part.
    ///
    /// The root must be `sst` in the main namespace, either spelling. Each
    /// `<si>` becomes the concatenation of every `<t>` text under it, the
    /// `<r>` runs' included, with `<rPh>` subtrees skipped: the phonetic
    /// reading is a separate string a producer shows beside the text, not
    /// part of it.
    ///
    /// The corpus carries no `<rPh>` and no `<phoneticPr>` at all
    /// (`specs/0010-reading-cells.md`, "Facts planning checked"), so the
    /// skip is proved by this module's tests and by nothing in the corpus
    /// run. It is here because the format allows them, not because a
    /// measurement asked for it.
    pub(crate) fn read<R: Read>(stream: &mut Stream<R>, index: usize) -> Result<Self, Error> {
        let mut scope = Scope::new(index);
        let mut items: Vec<String> = Vec::new();
        let mut root = false;
        // Depth of the `<rPh>` subtree being skipped, 0 when not in one. A
        // depth rather than a flag: `<rPh>` holds elements of its own and the
        // skip has to end at the matching end tag, not the first one.
        let mut phonetic = 0usize;
        // `Some` while inside an `<si>`: the item being assembled.
        let mut item: Option<String> = None;
        // True while inside a `<t>` whose text belongs to the current item.
        let mut text = false;

        while let Some(token) = stream.next()? {
            match token.event {
                Event::Start(ref start) | Event::Empty(ref start) => {
                    let empty = matches!(token.event, Event::Empty(_));
                    scope.start(start)?;
                    let (namespace, local) = scope.resolve_element(start.name())?;
                    let ours = MAIN.contains(&namespace.unwrap_or_default());
                    if !root {
                        if !ours || local != b"sst" {
                            return Err(Error::Package(
                                "the shared strings part's root is not sst",
                            ));
                        }
                        root = true;
                    } else if phonetic > 0 {
                        // Inside a skipped subtree nothing is read; the depth
                        // still tracks nesting so the skip ends in the right
                        // place.
                        if !empty {
                            phonetic += 1;
                        }
                    } else if ours && local == b"si" {
                        item = Some(String::new());
                        // An empty `<si/>` is an empty string, not a missing
                        // one: the item is pushed here and nothing adds to it.
                        if empty {
                            items.push(item.take().unwrap_or_default());
                        }
                    } else if ours && local == b"rPh" {
                        if !empty {
                            phonetic = 1;
                        }
                    } else if ours && local == b"t" && item.is_some() {
                        // An empty `<t/>` contributes nothing, which is the
                        // empty string the spec asks for.
                        if !empty {
                            text = true;
                        }
                    }
                    if empty {
                        scope.end();
                    }
                }
                Event::End(_) => {
                    let (namespace, local) = scope.resolve_element(
                        // The scope holds the open element's name; the end
                        // tag's own bytes are what `quick-xml` gives, and
                        // `quick-xml` has already matched them to the start
                        // tag, so resolving the end tag's name is enough.
                        match token.event {
                            Event::End(ref end) => end.name(),
                            _ => unreachable!("matched End above"),
                        },
                    )?;
                    let ours = MAIN.contains(&namespace.unwrap_or_default());
                    if phonetic > 0 {
                        phonetic -= 1;
                    } else if ours && local == b"t" {
                        text = false;
                    } else if ours
                        && local == b"si"
                        && let Some(done) = item.take()
                    {
                        items.push(done);
                    }
                    scope.end();
                }
                Event::Text(ref chunk) => {
                    if text
                        && phonetic == 0
                        && let Some(into) = item.as_mut()
                    {
                        into.push_str(chunk);
                    }
                }
                // The guard is what decides whether the reference is this
                // item's: one outside a `<t>` this layer reads is not
                // expanded, and so is not refused either.
                Event::GeneralRef(ref reference) if text && phonetic == 0 => {
                    let expanded = expand(reference, index)?;
                    if let Some(into) = item.as_mut() {
                        into.push(expanded);
                    }
                }
                _ => {}
            }
        }
        if !root {
            return Err(Error::Package("the shared strings part's root is not sst"));
        }
        Ok(Self { items })
    }
}

/// The character a general reference stands for.
///
/// A character reference is resolved by `quick-xml`; the five XML predefines
/// are expanded by name. Anything else is refused: a document type could
/// declare it, this layer does not read document types, and a value missing
/// one of its characters is not the value.
fn expand(reference: &BytesRef<'_>, index: usize) -> Result<char, Error> {
    if let Some(resolved) = reference
        .resolve_char_ref()
        .map_err(|_| Error::Xml(index))?
    {
        return Ok(resolved);
    }
    match &**reference {
        "amp" => Ok('&'),
        "lt" => Ok('<'),
        "gt" => Ok('>'),
        "quot" => Ok('"'),
        "apos" => Ok('\''),
        _ => Err(Error::Package(
            "a cell holds a reference this layer cannot expand",
        )),
    }
}

#[cfg(test)]
mod tests;
