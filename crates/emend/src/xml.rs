//! The XML layer: every token with the exact input bytes it covers.
//!
//! `specs/0009-package-layer.md`, "Module `xml`". [`Stream`] reads a part's
//! bytes with `quick-xml` and hands back each token together with the input's
//! OWN bytes for it, taken from a tee and never re-serialised. A rewrite can
//! therefore copy what it does not change, byte for byte, and write only what
//! it does: the acceptance is that the mark when present, followed by every
//! token's [`Token::raw`] in order, is the input exactly.
//!
//! Nothing here reads a cell. [`Scope`], added in part 2 P2, is the prefix
//! stack the package layer resolves names through; [`Stream`] itself never
//! resolves one, and the null rewrite of P1 never builds a scope.
//!
//! # Why the whole input is fed to the parser
//!
//! `quick-xml` strips a leading UTF-8 byte order mark in its `Init` state
//! only, once, by consuming from the `BufRead` directly, so those three bytes
//! never reach `buffer_position()`. The draft of `specs/0009` had this stream
//! strip the mark itself and feed the rest, which loses three bytes on a part
//! that starts with TWO marks: the crate strips the second one uncounted and
//! no token's `raw` holds it. Planning measured both mechanisms and ruled
//! (`specs/0009-package-layer.md`, "Planning rulings during P1", item 1) that
//! the stream feeds the WHOLE input, mark included, and maps the parser's
//! positions through [`Stream::origin`]: 3 when a mark was read, 0 otherwise.
//! A second mark then comes back inside a text token, like any other content.
//!
//! # Errors
//!
//! This module produces two variants of its own, catalogued here as
//! `archive`'s are, and [`Scope`] raises a third:
//!
//! - [`Error::Xml`], carrying the entry's index: the part's bytes are not
//!   well-formed XML as `quick-xml` reads it (see Limits for what that
//!   includes), or they are not UTF-8 anywhere, in markup or in text, since
//!   0.42 decodes text as `str`. Rendered as "not well-formed XML or not
//!   UTF-8 (entry N)". The message carries the index and NEVER a position or
//!   any bytes: the bytes either side of a parse error are content.
//! - [`Error::Io`] when the underlying reader fails, so a failing disk is
//!   never reported as a malformed part. The kind is carried across, not the
//!   error itself: `quick-xml` holds its I/O error in an `Arc` to stay
//!   `Clone`, so it cannot be moved out (see `parse_error`).
//! - [`Error::Package`] "an element or attribute uses a prefix its document
//!   does not bind", from [`Scope::resolve_element`] and
//!   [`Scope::resolve_attribute`]. It is catalogued with the fourteen other
//!   package messages, and with the test that pins it
//!   (`package::tests::rel_id_prefix_unbound`), in [`crate::package`]'s
//!   module header rather than here, since the only way to reach it is
//!   through a package part.
//!
//! The first two are asserted by a test in this module's `tests`; the third
//! is asserted from `package`, where the only caller that can reach it is.
//!
//! # Limits
//!
//! The parser runs with the crate's defaults: no trimming, end names checked,
//! empty elements not expanded, a dangling `&` refused. Three classes that
//! XML calls not well-formed are ACCEPTED by those defaults, measured and
//! ruled in `specs/0009-package-layer.md`, "Planning rulings during P1",
//! item 2:
//!
//! - an element left open at the end of the input (`<a>`, `<a>hi`);
//! - an unknown entity in text (`<a>&foo;</a>`, a general reference event);
//! - more than one root element (`<a/><b/>`), and text before or after one.
//!
//! A fourth joined them in P1 ruling 4, found by this module's own binary-part
//! test: a character XML forbids that is valid UTF-8 is accepted and
//! reproduced; the parser checks the encoding, not the character range, so
//! nothing in this layer refuses what a spreadsheet application would call
//! corrupt. NUL, the other C0 controls, DEL, `&#0;` and U+FFFE all pass, in
//! text, in an attribute value and inside a name. What makes a real binary
//! part refusable is that its bytes are not valid UTF-8, which is what the
//! `Xml` error above reports.
//!
//! A null rewrite of such a part still reproduces it byte for byte, which is
//! this layer's contract; whether a package part with an unclosed root is
//! refused is the package parsers' question in part 2 P2, not this module's.
//!
//! Two more limits of the mechanism:
//!
//! - A part is read forwards, once. There is no seeking back to an earlier
//!   token: [`Token::raw`] is valid only until the next [`Stream::next`].
//! - The tee holds the read-ahead plus the current token, never the part, so
//!   the bound is the largest single token. A token larger than this target's
//!   address space could not be held, but neither could the buffer a parser
//!   needs for it; the corpus's largest part is 63745347 bytes and its
//!   largest token is far smaller.
//!
//! Every position and count here is `u64`. `usize` is 32 bits on
//! `i686-unknown-linux-musl`.

use std::io::{self, BufReader, Chain, Cursor, Read};

use quick_xml::Reader;
use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::{PrefixDeclaration, QName};

use crate::archive::Error;

#[cfg(test)]
mod tests;

/// The UTF-8 byte order mark, the three bytes [`Stream::bom`] reports.
///
/// A rewrite that copies a part writes these before the first token's
/// [`Token::raw`] when [`Stream::bom`] is true, and nothing otherwise.
pub const BYTE_ORDER_MARK: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// The buffer the parser reads the part through.
const READ_AHEAD: usize = 64 * 1024;

/// How many released bytes the tee keeps at its front before compacting.
///
/// Compacting moves the retained bytes down, so doing it once per this many
/// released bytes makes the cost per byte of input constant, where draining on
/// every token would move the read-ahead once per token.
const COMPACT_AT: usize = 64 * 1024;

/// A reader that keeps a copy of every byte read through it.
///
/// The copy is what [`Token::raw`] borrows from, which is why a token's bytes
/// are the INPUT's and not a re-serialisation of the event.
struct Tee<R> {
    inner: R,
    /// Every byte read through this reader that has not been compacted away.
    buf: Vec<u8>,
    /// Index in `buf` of the first byte a caller may still ask for.
    front: usize,
    /// The input position of `buf[0]`: what compacting has dropped.
    dropped: u64,
}

impl<R: Read> Read for Tee<R> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(out)?;
        self.buf.extend_from_slice(&out[..n]);
        Ok(n)
    }
}

impl<R> Tee<R> {
    /// Marks every byte before `position` as one no caller can ask for again.
    fn release_to(&mut self, position: u64) {
        self.front = (position - self.dropped) as usize;
        if self.front >= COMPACT_AT {
            self.buf.drain(..self.front);
            self.dropped += self.front as u64;
            self.front = 0;
        }
    }

    /// The recorded bytes of the input's `from..to`.
    fn slice(&self, from: u64, to: u64) -> &[u8] {
        let start = (from - self.dropped) as usize;
        let end = (to - self.dropped) as usize;
        &self.buf[start..end]
    }
}

/// What the parser reads: the bytes `new` took off the front, put back before
/// the rest of the part, with every byte recorded on the way through.
type Source<R> = BufReader<Tee<Chain<Cursor<Vec<u8>>, R>>>;

/// One token of a part: the event, and the input bytes it covers.
///
/// `raw` runs from the end of the previous token to the parser's position
/// after this one, so consecutive tokens' `raw` are contiguous and cover the
/// whole part apart from a leading byte order mark. It borrows the stream
/// until the next [`Stream::next`] call.
pub struct Token<'a> {
    /// The event as `quick-xml` parsed it.
    pub event: Event<'a>,
    /// The input's own bytes for this token, never re-serialised.
    pub raw: &'a [u8],
}

/// A streaming reader over one part's bytes.
///
/// See the module header for the byte order mark, the errors and the limits.
pub struct Stream<R: Read> {
    reader: Reader<Source<R>>,
    /// The parser's scratch buffer, which every event borrows from.
    buf: Vec<u8>,
    /// The entry this part came from, carried into errors and nothing else.
    index: usize,
    bom: bool,
    /// The input position the parser's own position 0 corresponds to.
    origin: u64,
    /// The input position after the last token returned.
    seen: u64,
    done: bool,
}

impl<R: Read> Stream<R> {
    /// Opens a part for reading.
    ///
    /// Reads the first three bytes to report a byte order mark, then feeds the
    /// whole input, mark included, to the parser (see the module header).
    /// `index` is the entry's index, carried into errors.
    pub fn new(mut reader: R, index: usize) -> Result<Self, Error> {
        let mut head = Vec::with_capacity(BYTE_ORDER_MARK.len());
        let mut byte = [0u8; 1];
        while head.len() < BYTE_ORDER_MARK.len() {
            match reader.read(&mut byte) {
                Ok(0) => break,
                Ok(_) => head.push(byte[0]),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(Error::Io(e)),
            }
        }
        let bom = head == BYTE_ORDER_MARK;
        let origin = if bom { BYTE_ORDER_MARK.len() as u64 } else { 0 };
        let tee = Tee {
            inner: Cursor::new(head).chain(reader),
            buf: Vec::new(),
            front: 0,
            dropped: 0,
        };
        Ok(Stream {
            reader: Reader::from_reader(BufReader::with_capacity(READ_AHEAD, tee)),
            buf: Vec::new(),
            index,
            bom,
            origin,
            seen: origin,
            done: false,
        })
    }

    /// Whether the part began with a UTF-8 byte order mark.
    ///
    /// The mark is in no token's [`Token::raw`], so a rewrite writes
    /// [`BYTE_ORDER_MARK`] first when this is true.
    pub fn bom(&self) -> bool {
        self.bom
    }

    /// The next token, or `None` at the end of the input.
    ///
    /// Not [`Iterator::next`], and it cannot be: a [`Token`] borrows the
    /// stream until the following call, which the standard trait's item type
    /// cannot express. The name is the one `specs/0009-package-layer.md`,
    /// "Module `xml`", fixes.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<Option<Token<'_>>, Error> {
        if self.done {
            return Ok(None);
        }
        let from = self.seen;
        let index = self.index;
        self.buf.clear();
        let event = match self.reader.read_event_into(&mut self.buf) {
            Ok(event) => event,
            Err(err) => {
                // A part is read once: a parse error is final, and asking
                // again would restart the parser mid-document.
                self.done = true;
                return Err(parse_error(err, index));
            }
        };
        if matches!(event, Event::Eof) {
            self.done = true;
            return Ok(None);
        }
        let end = self.origin + self.reader.buffer_position();
        self.reader.get_mut().get_mut().release_to(from);
        let raw = self.reader.get_ref().get_ref().slice(from, end);
        self.seen = end;
        Ok(Some(Token { event, raw }))
    }

    /// The input position after the last token returned, the position the next
    /// token's [`Token::raw`] begins at.
    ///
    /// Before the first token this is the first counted byte: 3 with a mark, 0
    /// without.
    ///
    /// This exists for the tests, which check that the tokens tile the part by
    /// position, so it is compiled only for them rather than widening the
    /// layer's surface.
    #[cfg(test)]
    pub(crate) fn position(&self) -> u64 {
        self.seen
    }
}

/// Maps the parser's failure onto the layer's error.
///
/// An I/O failure stays an I/O failure: only the part's own bytes make an
/// `Xml`. The crate holds its I/O error in an `Arc` to stay `Clone`, so the
/// kind is carried across rather than the error itself, which is all the
/// layer's `Display` shows anyway.
fn parse_error(err: quick_xml::Error, index: usize) -> Error {
    match err {
        quick_xml::Error::Io(e) => Error::Io(io::Error::from(e.kind())),
        _ => Error::Xml(index),
    }
}

/// The prefix stack a package parser resolves names through.
///
/// `specs/0009-package-layer.md`, "Module `xml`": every `xmlns` and
/// `xmlns:<p>` declaration is recorded with the depth it was declared at and
/// dropped at the matching end tag. A start tag's own declarations bind for
/// that tag, so `<x:sheets xmlns:x="...">` resolves its own name; an empty
/// element's declarations apply to itself alone, so a caller calls
/// [`Scope::start`], resolves, and calls [`Scope::end`] before the next
/// token. There is no separate call for an empty element: one that recorded
/// and dropped the declarations together could not resolve the element's own
/// name, which is the only reason to record them at all.
///
/// The null rewrite never resolves a name, so nothing in P1's path builds one.
///
/// # Errors
///
/// - [`Error::Package`] "an element or attribute uses a prefix its document
///   does not bind", from either resolver.
/// - [`Error::Xml`], carrying the entry index given to [`Scope::new`], when a
///   namespace declaration's value holds a reference the parser cannot
///   resolve. A namespace URI is a value this layer interprets, so it is
///   read through `normalized_value` rather than compared as written
///   (`specs/0009-package-layer.md`, P2 ruling 2).
///
/// The version passed is `XmlVersion::Implicit1_0`, which the ruling names:
/// 0.42.0's variants are `Implicit1_0`, `Explicit1_0` and `Explicit1_1`, the
/// two 1.0 ones taking the same XML 1.0 path (1.1 would additionally fold
/// U+0085 and U+2028). `Implicit1_0` is also what the crate's own deprecated
/// `unescape_value` passes, so this call is the method the spec first named,
/// with the deprecation gone.
pub struct Scope {
    /// Each binding as (depth, prefix, namespace); the prefix is `None` for a
    /// default namespace. Later entries shadow earlier ones with the same
    /// prefix, so a lookup walks backwards.
    bindings: Vec<(usize, Option<Vec<u8>>, Vec<u8>)>,
    depth: usize,
    index: usize,
}

impl Scope {
    /// A scope with nothing bound, for the entry `index`.
    ///
    /// The index is carried only so that a malformed declaration can name the
    /// entry it came from, as every other error in this layer does.
    pub fn new(index: usize) -> Self {
        Scope {
            bindings: Vec::new(),
            depth: 0,
            index,
        }
    }

    /// Records a start tag's declarations. Call [`Scope::end`] at its end tag.
    pub fn start(&mut self, start: &BytesStart<'_>) -> Result<(), Error> {
        self.depth += 1;
        self.declare(start)
    }

    /// Drops the declarations of the start tag just ended.
    pub fn end(&mut self) {
        self.bindings.retain(|(depth, _, _)| *depth < self.depth);
        self.depth = self.depth.saturating_sub(1);
    }

    /// The namespace and local name of an element's name.
    ///
    /// An unprefixed element takes the default namespace when one is bound.
    pub fn resolve_element<'n>(&self, name: QName<'n>) -> Result<(Option<&[u8]>, &'n [u8]), Error> {
        let (local, prefix) = name.decompose();
        let namespace = match prefix {
            Some(prefix) => Some(self.bound(Some(prefix.into_inner().as_bytes()))?),
            None => self.lookup(None),
        };
        Ok((namespace, local.into_inner().as_bytes()))
    }

    /// The namespace and local name of an attribute's name.
    ///
    /// An unprefixed attribute has NO namespace, whatever default is bound,
    /// which is why this is not the same function as
    /// [`Scope::resolve_element`].
    pub fn resolve_attribute<'n>(
        &self,
        name: QName<'n>,
    ) -> Result<(Option<&[u8]>, &'n [u8]), Error> {
        let (local, prefix) = name.decompose();
        let namespace = match prefix {
            Some(prefix) => Some(self.bound(Some(prefix.into_inner().as_bytes()))?),
            None => None,
        };
        Ok((namespace, local.into_inner().as_bytes()))
    }

    /// Adds every namespace declaration this start tag carries.
    fn declare(&mut self, start: &BytesStart<'_>) -> Result<(), Error> {
        for attribute in start.attributes() {
            let attribute = attribute.map_err(|_| Error::Xml(self.index))?;
            let Some(declaration) = attribute.key.as_namespace_binding() else {
                continue;
            };
            let prefix = match declaration {
                PrefixDeclaration::Default => None,
                PrefixDeclaration::Named(prefix) => Some(prefix.as_bytes().to_vec()),
            };
            let value = attribute
                .normalized_value(XmlVersion::Implicit1_0)
                .map_err(|_| Error::Xml(self.index))?;
            self.bindings
                .push((self.depth, prefix, value.as_bytes().to_vec()));
        }
        Ok(())
    }

    /// The namespace bound to `prefix`, innermost first, or `None`.
    ///
    /// A declaration of the empty string undoes a binding, as XML says, so it
    /// reads as nothing bound rather than as a namespace whose name is empty.
    fn lookup(&self, prefix: Option<&[u8]>) -> Option<&[u8]> {
        self.bindings
            .iter()
            .rev()
            .find(|(_, bound, _)| bound.as_deref() == prefix)
            .map(|(_, _, namespace)| namespace.as_slice())
            .filter(|namespace| !namespace.is_empty())
    }

    /// [`Scope::lookup`], refusing a prefix no ancestor binds.
    fn bound(&self, prefix: Option<&[u8]>) -> Result<&[u8], Error> {
        self.lookup(prefix).ok_or(Error::Package(
            "an element or attribute uses a prefix its document does not bind",
        ))
    }
}
