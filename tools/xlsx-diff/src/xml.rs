//! The names-only XML comparison (`specs/0014-xml-comparison.md`).
//!
//! For an entry whose bytes changed and whose name ends in `.xml`, `.rels`
//! or `.vml`, each side is read with `quick-xml`'s `NsReader` into three
//! things: the multiset of resolved element names, the multiset of resolved
//! attribute names, and a vector of 64-bit hashes, one per value. The rows
//! print names and counts only; no value, no hash and no sample of one is
//! ever printed.
//!
//! # Limits
//!
//! - Names are resolved as XML Namespaces 1.0 says: an element is
//!   `{namespace}local`, `{}` when unbound; an attribute is
//!   `{namespace}local@{namespace}attr`, and an unprefixed attribute is in no
//!   namespace. Prefix choice therefore never shows. `xmlns` and `xmlns:p`
//!   declarations are not attributes here. An unknown prefix is a parse
//!   error for that side.
//! - A value is an attribute (hashed with its element's and its own resolved
//!   names) or a text node with content (hashed with its enclosing element's
//!   resolved name, or with no name outside every element). A text node is
//!   the run of text, CDATA sections and references between two tags;
//!   comments and processing instructions inside a run are transparent, so
//!   `x<!--c-->y` is one node. A node of XML whitespace only (space, tab,
//!   line feed, carriage return, after references are resolved) is not a
//!   value. The declaration, comments, processing instructions and a
//!   document type declaration are ignored.
//! - Line ends: `quick-xml` hands text over as written, so a CR LF pair and
//!   a lone CR in text or CDATA become one LF here (XML 1.0 section 2.11). A
//!   CR written as `&#13;` stays a CR, so it differs from a raw one. A CR and
//!   an LF with a comment, a processing instruction or a CDATA delimiter
//!   between them are not adjacent, so they are two line ends. Attribute
//!   values go through `normalized_value` for XML 1.0, which normalises
//!   their whitespace as XML 1.0 section 3.3.3 says.
//! - Text references: the five XML predefines and character references are
//!   resolved. A text node holding any other reference, or a character
//!   reference that does not resolve, keeps that reference as written and
//!   is hashed raw; an attribute value `normalized_value` refuses is hashed
//!   as written. Each such value counts once in the `parse` row.
//! - Values are compared as multisets: both vectors are sorted and merged
//!   once, and the row gives how many hashes on each side have no match on
//!   the other. It never says which value, nor where. Two different values
//!   with the same 64-bit hash would count as equal.
//! - The hash is `std`'s `DefaultHasher` with its fixed keys (SipHash 1-3),
//!   fed a tag byte and then each part's length as a `u64` and its bytes, so
//!   the value does not depend on the width of `usize`. It is never printed.
//! - Memory: the name maps are bounded by the schema's names, not the data.
//!   The value vectors hold 8 bytes per value per side, both alive at once
//!   while they are merged, and are freed before the next entry. An entry
//!   they cannot fit is an allocation failure of the process.
//! - Parse errors: an error the reader returns is at `error_position()`; an
//!   unknown prefix, an attribute error and the end of input with an element
//!   still open are at `buffer_position()` after the event. The message is
//!   never printed, since `quick-xml` quotes the bytes it did not expect. The
//!   tool does not check that a root element exists or that nothing stands
//!   outside it. A side that fails prints no name or value rows for either
//!   side; its raw-value count counts up to the failure. A text node the
//!   error interrupts was never hashed, so a raw reference in it is not
//!   counted.
//! - Encoding: the tool reads UTF-8 only, since `quick-xml` is built
//!   without its encoding feature. An entry in another encoding, such as
//!   one starting with a UTF-16 byte order mark, is a parse error for that
//!   side; the corpus holds none (`specs/0014-xml-comparison.md`, "Facts").
//! - An I/O error while reading an entry (a failed decompression or CRC-32)
//!   is not a parse error: the comparison stops with exit status 2.

use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hasher};
use std::io::BufRead;

use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::ResolveResult;
use quick_xml::reader::NsReader;

/// Whether an entry with this raw name is compared when its bytes changed:
/// its name, lower-cased, ends in `.xml`, `.rels` or `.vml`.
pub(crate) fn is_xml_name(raw: &[u8]) -> bool {
    let lower = raw.to_ascii_lowercase();
    [&b".xml"[..], b".rels", b".vml"]
        .iter()
        .any(|ext| lower.ends_with(ext))
}

/// What one side of an entry held.
pub(crate) struct Read {
    /// Resolved element names and their counts.
    pub(crate) elements: BTreeMap<Vec<u8>, u64>,
    /// Resolved `element@attribute` names and their counts.
    pub(crate) attributes: BTreeMap<Vec<u8>, u64>,
    /// One hash per value, in document order until sorted.
    pub(crate) values: Vec<u64>,
    /// Values hashed as written because they did not unescape.
    pub(crate) raw: u64,
    /// The offset of the parse error, if the side failed.
    pub(crate) error: Option<u64>,
}

/// Why reading a side stopped early without a parse error.
pub(crate) struct Io;

/// Tag bytes that keep an attribute's hash apart from a text node's.
const TAG_ATTRIBUTE: u8 = 1;
const TAG_TEXT: u8 = 2;

/// The hash of one value: a tag byte, then each part's length as a `u64`
/// and its bytes.
pub(crate) fn value_hash(tag: u8, parts: &[&[u8]]) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_u8(tag);
    for part in parts {
        hasher.write_u64(part.len() as u64);
        hasher.write(part);
    }
    hasher.finish()
}

/// A resolved name, as the namespace and the local name.
struct Name {
    namespace: Vec<u8>,
    local: Vec<u8>,
}

impl Name {
    /// `{namespace}local`.
    fn written(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.namespace.len() + self.local.len() + 2);
        out.push(b'{');
        out.extend_from_slice(&self.namespace);
        out.push(b'}');
        out.extend_from_slice(&self.local);
        out
    }
}

/// The resolved namespace, or `None` for an unknown prefix.
fn resolved_namespace(result: ResolveResult<'_>) -> Option<Vec<u8>> {
    match result {
        ResolveResult::Bound(ns) => Some(ns.as_ref().as_bytes().to_vec()),
        ResolveResult::Unbound => Some(Vec::new()),
        ResolveResult::Unknown(_) => None,
    }
}

/// The text node being assembled, with its line-end state.
#[derive(Default)]
struct Text {
    content: String,
    /// The last character appended was a literal CR, appended as LF.
    after_cr: bool,
    /// A reference in the node did not resolve and was kept as written.
    raw: bool,
}

impl Text {
    /// Appends literal characters with CR LF and a lone CR as one LF.
    fn push_literal(&mut self, chunk: &str) {
        for character in chunk.chars() {
            match character {
                '\r' => {
                    self.content.push('\n');
                    self.after_cr = true;
                }
                '\n' => {
                    if !self.after_cr {
                        self.content.push('\n');
                    }
                    self.after_cr = false;
                }
                _ => {
                    self.content.push(character);
                    self.after_cr = false;
                }
            }
        }
    }

    /// Records markup inside the node (a comment, a processing instruction
    /// or a CDATA delimiter): the characters either side of it are not
    /// adjacent in the document, so a CR before it and an LF after it are
    /// two line ends (XML 1.0 section 2.11 applies to adjacent characters).
    fn markup(&mut self) {
        self.after_cr = false;
    }

    /// Appends a character a reference stands for. It is not half of a
    /// line end, so a literal LF after it is a line end of its own.
    fn push_reference(&mut self, character: char) {
        self.content.push(character);
        self.after_cr = false;
    }

    /// Appends a reference that does not resolve, as written.
    fn push_unresolved(&mut self, name: &str) {
        self.content.push('&');
        self.content.push_str(name);
        self.content.push(';');
        self.after_cr = false;
        self.raw = true;
    }
}

/// The character a predefined or character reference stands for, or `None`.
fn expand(name: &str) -> Option<char> {
    if name.starts_with('#') {
        let reference = quick_xml::events::BytesRef::new(name);
        return reference.resolve_char_ref().ok().flatten();
    }
    match name {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        _ => None,
    }
}

/// The state of one side while it is read.
struct Reading {
    read: Read,
    /// The resolved names of the open elements, innermost last.
    open: Vec<Name>,
    text: Text,
}

impl Reading {
    /// Ends the current text node, hashing it unless it is empty or XML
    /// whitespace only.
    fn flush_text(&mut self) {
        let text = std::mem::take(&mut self.text);
        if text
            .content
            .bytes()
            .all(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r'))
        {
            return;
        }
        let (namespace, local): (&[u8], &[u8]) = match self.open.last() {
            Some(name) => (&name.namespace, &name.local),
            None => (b"", b""),
        };
        self.read.values.push(value_hash(
            TAG_TEXT,
            &[namespace, local, text.content.as_bytes()],
        ));
        if text.raw {
            self.read.raw += 1;
        }
    }

    /// Counts a start or empty tag's name and attributes and hashes the
    /// attribute values. Returns `false` on an unknown prefix or an
    /// attribute error.
    fn element<R>(&mut self, reader: &NsReader<R>, start: &BytesStart<'_>) -> bool {
        let resolver = reader.resolver();
        let (result, local) = resolver.resolve_element(start.name());
        let Some(namespace) = resolved_namespace(result) else {
            return false;
        };
        let name = Name {
            namespace,
            local: local.as_ref().as_bytes().to_vec(),
        };
        let written = name.written();
        *self.read.elements.entry(written.clone()).or_insert(0) += 1;
        for attribute in start.attributes() {
            let Ok(attribute) = attribute else {
                return false;
            };
            if attribute.key.as_namespace_binding().is_some() {
                continue;
            }
            let (result, attr_local) = resolver.resolve_attribute(attribute.key);
            let Some(attr_namespace) = resolved_namespace(result) else {
                return false;
            };
            let attr = Name {
                namespace: attr_namespace,
                local: attr_local.as_ref().as_bytes().to_vec(),
            };
            let mut key = written.clone();
            key.push(b'@');
            key.extend_from_slice(&attr.written());
            *self.read.attributes.entry(key).or_insert(0) += 1;
            let value = match attribute.normalized_value(XmlVersion::Implicit1_0) {
                Ok(value) => value.into_owned(),
                Err(_) => {
                    self.read.raw += 1;
                    attribute.value.into_owned()
                }
            };
            self.read.values.push(value_hash(
                TAG_ATTRIBUTE,
                &[
                    &name.namespace,
                    &name.local,
                    &attr.namespace,
                    &attr.local,
                    value.as_bytes(),
                ],
            ));
        }
        self.open.push(name);
        true
    }
}

/// Reads one side to its end, or to its first parse error.
pub(crate) fn read_side<R: BufRead>(source: R) -> Result<Read, Io> {
    let mut reader = NsReader::from_reader(source);
    let mut state = Reading {
        read: Read {
            elements: BTreeMap::new(),
            attributes: BTreeMap::new(),
            values: Vec::new(),
            raw: 0,
            error: None,
        },
        open: Vec::new(),
        text: Text::default(),
    };
    let mut buf = Vec::new();
    loop {
        buf.clear();
        let event = match reader.read_event_into(&mut buf) {
            Ok(event) => event,
            Err(quick_xml::Error::Io(_)) => return Err(Io),
            Err(_) => {
                state.read.error = Some(reader.error_position());
                return Ok(state.read);
            }
        };
        match event {
            Event::Start(ref start) => {
                state.flush_text();
                if !state.element(&reader, start) {
                    state.read.error = Some(reader.buffer_position());
                    return Ok(state.read);
                }
            }
            Event::Empty(ref start) => {
                state.flush_text();
                if !state.element(&reader, start) {
                    state.read.error = Some(reader.buffer_position());
                    return Ok(state.read);
                }
                state.open.pop();
            }
            Event::End(_) => {
                state.flush_text();
                state.open.pop();
            }
            Event::Text(ref chunk) => state.text.push_literal(chunk),
            // A CDATA section's delimiters are markup, so a CR before or
            // after one is not half of a pair across it.
            Event::CData(ref chunk) => {
                state.text.markup();
                state.text.push_literal(chunk);
                state.text.markup();
            }
            Event::GeneralRef(ref reference) => match expand(reference) {
                Some(character) => state.text.push_reference(character),
                None => state.text.push_unresolved(reference),
            },
            // Transparent to the text node, but markup: a CR before one and
            // an LF after it are two line ends.
            Event::Comment(_) | Event::PI(_) => state.text.markup(),
            Event::Decl(_) | Event::DocType(_) => {}
            Event::Eof => {
                state.flush_text();
                if !state.open.is_empty() {
                    state.read.error = Some(reader.buffer_position());
                }
                return Ok(state.read);
            }
        }
    }
}

/// The counts of hashes on each side without a match on the other, as
/// multisets. Sorts both vectors.
pub(crate) fn one_sided(a: &mut [u64], b: &mut [u64]) -> (u64, u64) {
    a.sort_unstable();
    b.sort_unstable();
    let (mut i, mut j) = (0, 0);
    let (mut in_only, mut out_only) = (0u64, 0u64);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Equal => {
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Less => {
                in_only += 1;
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                out_only += 1;
                j += 1;
            }
        }
    }
    in_only += (a.len() - i) as u64;
    out_only += (b.len() - j) as u64;
    (in_only, out_only)
}

/// The names whose counts differ between two maps, in byte order, with
/// both counts.
pub(crate) fn differing(
    a: &BTreeMap<Vec<u8>, u64>,
    b: &BTreeMap<Vec<u8>, u64>,
) -> Vec<(Vec<u8>, u64, u64)> {
    let mut names: Vec<&Vec<u8>> = a.keys().chain(b.keys()).collect();
    names.sort_unstable();
    names.dedup();
    names
        .into_iter()
        .filter_map(|name| {
            let (x, y) = (
                a.get(name).copied().unwrap_or(0),
                b.get(name).copied().unwrap_or(0),
            );
            (x != y).then(|| (name.clone(), x, y))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The hash is the same on every target the tool is built for
    /// (`specs/0014-xml-comparison.md`, P1 ruling 7). The constant is this
    /// test's; no record ever carries a hash.
    #[test]
    fn value_hash_is_pinned() {
        let hash = value_hash(TAG_TEXT, &[b"urn:example", b"t", b"fictional"]);
        assert_eq!(hash, PINNED);
    }

    const PINNED: u64 = 15_503_866_538_912_441_501;
}
