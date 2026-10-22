//! Reading what a workbook's styles part defines: its number formats and its
//! cell formats.
//!
//! `specs/0012-styles.md`, "Module `styles`: what is read". [`Styles`] holds
//! the `numFmt` children of `<numFmts>` and the `xf` children of `<cellXfs>`,
//! both in part order, so `cell_xfs[i]` is what a cell's `s="i"` names, and
//! counts the children of `<fonts>`, `<fills>` and `<borders>`
//! (`specs/0017-fuller-styles.md`, "Module `styles`: the counts"). Without
//! the Cargo feature `read`, every other element of the part is walked past
//! with its subtree: what a font, a fill or a border holds, `cellStyleXfs`,
//! `cellStyles`, `dxfs`, table styles, colours and extensions are not
//! modelled, and nothing about them is read. With it, table styles,
//! `<mruColors>`, extensions and elements of other namespaces are still
//! walked past.
//!
//! [`crate::package::Package::styles`] reads the part once and keeps the
//! answer, as [`crate::package::Package::workbook`] does.
//!
//! With the Cargo feature `read`, the same one read also reads each font,
//! fill and border, and each cell format's first `alignment` and
//! `protection`, into the types [`Style`] carries, and
//! `Package::cell_style` resolves a cell format into them as a `CellStyle`
//! (`specs/0027-read-back.md`, "Cell formats"); what they
//! cannot hold is read past or refused as that module says. The same read
//! also reads `cellStyleXfs`, `cellStyles`, the main namespace's `dxfs`
//! and `<colors>`'s `indexedColors`, and `Package::resolve_color` resolves
//! a colour through them, the default palette and the theme
//! (`specs/0032-read-breadth-styles.md`).
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
//! - Each direct child of an `xf` has its name resolved, to find the first
//!   `alignment` and `protection` in the main namespace, so a child whose
//!   prefix no declaration binds refuses the part ("an element or attribute
//!   uses a prefix its document does not bind"), as a name the crate
//!   resolves anywhere does; such a part is not namespace-well-formed
//!   (`specs/0017-fuller-styles.md`, P2 verification ruling 5).
//! - A format code is text to this layer: which numbers it shows as dates is
//!   the caller's to decide.
//! - What [`crate::package::Package::add_style`] adds is a cell format
//!   derived from an existing one, or the plain form over no base, with its
//!   number format, font, fill, border, alignment and protection as the
//!   [`Style`] gives them, a custom number format when its code is new, and
//!   a font, a fill and a border appended to their collections
//!   (`specs/0017-fuller-styles.md`); `dxfs` and `cellStyles` are never
//!   touched.
//! - A style edit assumes the styles part carries the font, fill, border and
//!   cell-style collections and `cellXfs` entries that name them, as every
//!   workbook Excel or LibreOffice writes does. A new `xf` names font 0,
//!   fill 0, border 0 and cell style 0, and Excel refuses a cell whose `s`
//!   names an `xf` in a part without those collections (`specs/0012-styles.md`,
//!   the limits, from Excel check 1).
//! - Two additions with equal codes share one `numFmt` id, and two with equal
//!   fonts, fills or borders one element and one id; nothing is compared with
//!   the part's own fonts, fills and borders, which are counted and not read;
//!   two additions with an equal base and style are two cell formats, since
//!   nothing is deduplicated at the `xf` level.
//! - What cannot be written: a font's `charset`, `outline`, `shadow`,
//!   `condense` or `extend`, an inside-of-range border side,
//!   and an alignment's `readingOrder`, `justifyLastLine` or
//!   `relativeIndent` (`specs/0017-fuller-styles.md`, decision 2).
//! - An addition is refused, with the messages catalogued in
//!   [`crate::package`]'s module header, for a base that names no existing
//!   `xf`, a built-in id of 164 or more, a code that is empty or holds a
//!   character XML cannot carry, a font, fill or border for a part without
//!   its collection, a font name that is empty or holds such a character, a
//!   font size, a tint or a text rotation out of range, and an index or id
//!   `u32` cannot hold.

use std::io::Read;

use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};

use crate::archive::Error;
use crate::cells::decimal;
use crate::package::{MAIN, STYLE_OUT_OF_RANGE};
use crate::xml::{Scope, Stream};

#[cfg(feature = "read")]
mod colors;
mod elements;
#[cfg(feature = "read")]
mod read;
#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "read"))]
pub(crate) use colors::PALETTE;
#[cfg(feature = "read")]
pub use colors::ResolvedColor;
#[cfg(feature = "read")]
pub(crate) use colors::{read_theme, resolve};
pub(crate) use elements::Composed;
#[cfg(any(feature = "rules", feature = "sheetprops"))]
pub(crate) use elements::color_element;
pub use elements::{
    Alignment, Border, BorderStyle, Color, Fill, Font, Gradient, GradientKind, Horizontal, Pattern,
    Protection, Scheme, Side, Stop, Style, Underline, Vertical, VerticalAlign,
};
#[cfg(feature = "read")]
pub use read::{CellStyle, NamedStyle};

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
    /// The number of `font` children of `<fonts>`, `None` without one.
    fonts: Option<u32>,
    /// The number of `fill` children of `<fills>`, `None` without one.
    fills: Option<u32>,
    /// The number of `border` children of `<borders>`, `None` without one.
    borders: Option<u32>,
    /// The fonts, fills and borders read typed (`specs/0027-read-back.md`,
    /// "Cell formats"), and the differential styles
    /// (`specs/0032-read-breadth-styles.md`). The Cargo feature `read`.
    #[cfg(feature = "read")]
    typed: read::Typed,
    /// The `xf` children of `<cellStyleXfs>`, in part order: the formats the
    /// named styles name (`specs/0032-read-breadth-styles.md`, "Named
    /// styles"). The Cargo feature `read`.
    #[cfg(feature = "read")]
    pub cell_style_xfs: Vec<Xf>,
    /// The `cellStyle` children of `<cellStyles>`, in part order. The Cargo
    /// feature `read`.
    #[cfg(feature = "read")]
    pub cell_styles: Vec<NamedStyle>,
    /// The `rgb` of each `rgbColor` of `<colors>`'s `<indexedColors>`, in
    /// order, as [`Color::Rgb`] holds one; `None` without the element
    /// (`specs/0032-read-breadth-styles.md`, "Colours"). An index within it
    /// resolves here, one past it through the default palette. The Cargo
    /// feature `read`.
    #[cfg(feature = "read")]
    pub indexed_colors: Option<Vec<u32>>,
}

/// One `<numFmt>`: a number format the part defines.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumFmt {
    /// `numFmtId`, a decimal `u32`.
    pub id: u32,
    /// `formatCode`, unescaped and normalised as the module header says.
    pub code: String,
}

/// One `<xf>` of `<cellXfs>`: a cell format; or, with the Cargo feature
/// `read`, of `<cellStyleXfs>`: a named style's format.
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
    /// Where its first `alignment` child lies in `raw`, start tag to end
    /// tag, when it has one (`specs/0017-fuller-styles.md`, "The content").
    alignment: Option<std::ops::Range<usize>>,
    /// Where its first `protection` child lies in `raw`, likewise.
    protection: Option<std::ops::Range<usize>>,
    /// Its first `alignment` child read typed. The Cargo feature `read`.
    #[cfg(feature = "read")]
    alignment_value: Option<Alignment>,
    /// Its first `protection` child read typed. The Cargo feature `read`.
    #[cfg(feature = "read")]
    protection_value: Option<Protection>,
}

impl Xf {
    /// The element's own bytes, as the part writes them.
    #[cfg(test)]
    pub(crate) fn raw(&self) -> &[u8] {
        &self.raw
    }
}

/// A cell format recorded by [`crate::package::Package::add_style`],
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
    /// What goes between its start tag and its end tag: the base's bytes and
    /// any new `alignment` and `protection`, which take the prefix at the
    /// rewrite as the name does. `None` when the element is written empty.
    content: Option<Composed>,
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
                out.extend_from_slice(&content.element(prefix));
                out.extend_from_slice(&[b"</".as_slice(), prefix, b"xf>"].concat());
            }
        }
        out
    }
}

/// What a new cell format changes of its base: each id given, `None` for
/// what the base keeps, and the `alignment` and `protection` elements given.
#[derive(Clone, Debug, Default)]
pub(crate) struct Changes {
    /// `numFmtId`.
    pub(crate) num_fmt: Option<u32>,
    /// `fontId`.
    pub(crate) font: Option<u32>,
    /// `fillId`.
    pub(crate) fill: Option<u32>,
    /// `borderId`.
    pub(crate) border: Option<u32>,
    /// The new `alignment` child.
    pub(crate) alignment: Option<Composed>,
    /// The new `protection` child.
    pub(crate) protection: Option<Composed>,
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

    /// The number of fonts the part's `<fonts>` holds, `None` when it has
    /// none. Nothing inside a font is read without the Cargo feature `read`.
    pub fn fonts(&self) -> Option<u32> {
        self.fonts
    }

    /// The number of fills the part's `<fills>` holds, `None` when it has
    /// none. Nothing inside a fill is read without the Cargo feature `read`.
    pub fn fills(&self) -> Option<u32> {
        self.fills
    }

    /// The number of borders the part's `<borders>` holds, `None` when it
    /// has none. Nothing inside a border is read without the Cargo feature
    /// `read`.
    pub fn borders(&self) -> Option<u32> {
        self.borders
    }

    /// The new cell format [`crate::package::Package::add_style`] records: a
    /// copy of the cell format `base` with what `changes` gives replaced, or
    /// the form `specs/0017-fuller-styles.md` gives for no base.
    ///
    /// With a base, the start tag is written anew from the base's own bytes:
    /// its attributes in its order, each as a space, the name, `=`, the quote
    /// character the base used for it, the value's bytes as the base wrote
    /// them and that quote again (`specs/0012-styles.md`, P1 ruling 5),
    /// except that each id `changes` gives has the new value and each apply
    /// flag of what it gives the value `1`. An id the base lacks is added:
    /// `numFmtId` first, the others after the base's attributes in the order
    /// `fontId fillId borderId`, then each apply flag the base lacks in the
    /// order `applyNumberFormat applyFont applyFill applyBorder
    /// applyAlignment applyProtection`.
    ///
    /// The content is the base's bytes between its start and end tags, with
    /// its first `alignment` child replaced by the one given and its first
    /// `protection` likewise; an `alignment` the base lacks goes first, a
    /// `protection` it lacks after the base's `alignment` when it has one and
    /// first otherwise, after any new `alignment`. Every other byte is the
    /// base's. No content gives an empty element. `None` when `base` is out
    /// of range, or when the base's start tag does not scan, which a part the
    /// reader accepted cannot give.
    ///
    /// With no base: `numFmtId`, `fontId`, `fillId` and `borderId` with the
    /// given ids or 0, `xfId="0"`, the apply flag of each thing given, in the
    /// order above, and the new `alignment` and `protection` as its content.
    pub(crate) fn derived_xf(&self, base: Option<u32>, changes: &Changes) -> Option<NewXf> {
        // `(id attribute, new value)` in the order they are added.
        let ids: [(&[u8], Option<u32>); 4] = [
            (b"numFmtId", changes.num_fmt),
            (b"fontId", changes.font),
            (b"fillId", changes.fill),
            (b"borderId", changes.border),
        ];
        // `(apply flag, whether its thing is given)`, in the order added.
        let flags: [(&[u8], bool); 6] = [
            (b"applyNumberFormat", changes.num_fmt.is_some()),
            (b"applyFont", changes.font.is_some()),
            (b"applyFill", changes.fill.is_some()),
            (b"applyBorder", changes.border.is_some()),
            (b"applyAlignment", changes.alignment.is_some()),
            (b"applyProtection", changes.protection.is_some()),
        ];
        let mut attributes = Vec::new();
        let mut push = |name: &[u8], quote: u8, value: &[u8]| {
            attributes.push(b' ');
            attributes.extend_from_slice(name);
            attributes.push(b'=');
            attributes.push(quote);
            attributes.extend_from_slice(value);
            attributes.push(quote);
        };
        let Some(base) = base else {
            for (name, id) in ids {
                push(name, b'"', id.unwrap_or(0).to_string().as_bytes());
            }
            push(b"xfId", b'"', b"0");
            for (flag, given) in flags {
                if given {
                    push(flag, b'"', b"1");
                }
            }
            let mut content = Composed::new();
            for child in [&changes.alignment, &changes.protection]
                .into_iter()
                .flatten()
            {
                content.append(child);
            }
            return Some(NewXf {
                attributes,
                content: (!content.is_empty()).then_some(content),
            });
        };
        let xf = self.cell_xfs.get(usize::try_from(base).ok()?)?;
        let raw = &xf.raw;
        let tag = scan_start_tag(raw)?;
        let named = |name: &[u8]| tag.attributes.iter().any(|a| &raw[a.name.clone()] == name);
        if let Some(id) = changes.num_fmt
            && !named(b"numFmtId")
        {
            push(b"numFmtId", b'"', id.to_string().as_bytes());
        }
        for attribute in &tag.attributes {
            let name = &raw[attribute.name.clone()];
            let new_id = ids
                .iter()
                .find(|(id_name, _)| name == *id_name)
                .and_then(|(_, id)| id.map(|id| id.to_string().into_bytes()));
            let new_flag = flags
                .iter()
                .any(|(flag, given)| *given && name == *flag)
                .then(|| b"1".to_vec());
            let value = new_id
                .or(new_flag)
                .unwrap_or_else(|| raw[attribute.value.clone()].to_vec());
            push(name, attribute.quote, &value);
        }
        for (name, id) in &ids[1..] {
            if let Some(id) = id
                && !named(name)
            {
                push(name, b'"', id.to_string().as_bytes());
            }
        }
        for (flag, given) in flags {
            if given && !named(flag) {
                push(flag, b'"', b"1");
            }
        }
        // The base's content, from after its start tag to its end tag, which
        // is its last `<`: the reader kept the element's tokens and nothing
        // after them.
        let (start, end) = if tag.empty {
            (tag.end, tag.end)
        } else {
            (tag.end, raw.iter().rposition(|byte| *byte == b'<')?)
        };
        // Each edit: `(position in raw, bytes of raw it replaces, element)`,
        // in the order they apply; at one position an `alignment` comes
        // before a `protection`.
        let mut edits: Vec<(usize, usize, &Composed)> = Vec::new();
        if let Some(new) = &changes.alignment {
            edits.push(match &xf.alignment {
                Some(span) => (span.start, span.end, new),
                None => (start, start, new),
            });
        }
        if let Some(new) = &changes.protection {
            edits.push(match (&xf.protection, &xf.alignment) {
                (Some(span), _) => (span.start, span.end, new),
                (None, Some(alignment)) => (alignment.end, alignment.end, new),
                (None, None) => (start, start, new),
            });
        }
        edits.sort_by_key(|(at, _, _)| *at);
        let mut content = Composed::new();
        let mut at = start;
        for (from, to, new) in edits {
            content.raw(raw.get(at..from)?);
            content.append(new);
            at = to.max(at);
        }
        content.raw(raw.get(at..end)?);
        Some(NewXf {
            attributes,
            content: (!content.is_empty()).then_some(content),
        })
    }

    /// The number format of the cell format `xf`.
    ///
    /// `None` when `xf` is out of range. Else the code of the first `numFmt`
    /// whose id is the cell format's `numFmtId`, an id below 164 that a
    /// producer spelled out included; else [`NumberFormat::BuiltIn`] with the
    /// cell format's id, or `BuiltIn(0)` (`General`) when it has none.
    pub fn number_format(&self, xf: u32) -> Option<NumberFormat> {
        Some(self.format_of(self.cell_xfs.get(usize::try_from(xf).ok()?)?))
    }

    /// The number format of the format `xf`, as [`Styles::number_format`]
    /// says.
    fn format_of(&self, xf: &Xf) -> NumberFormat {
        let id = xf.num_fmt_id.unwrap_or(0);
        match self.num_fmts.iter().find(|format| format.id == id) {
            Some(format) => NumberFormat::Code(format.code.clone()),
            None => NumberFormat::BuiltIn(id),
        }
    }

    /// Reads a styles part.
    ///
    /// The root must be `styleSheet` in the main namespace, either spelling
    /// and any prefix. Under it, the `numFmt` children of `<numFmts>` and the
    /// `xf` children of `<cellXfs>` are read, and the `font`, `fill` and
    /// `border` children of `<fonts>`, `<fills>` and `<borders>` counted;
    /// every other element, at any depth, is walked past with its subtree. A
    /// second of any of those five containers among the root's children is
    /// refused: the schema allows one of each, and a second `<cellXfs>` would
    /// make every `s` ambiguous (`specs/0012-styles.md`, P1 ruling 4;
    /// `specs/0017-fuller-styles.md`, "Module `styles`: the counts").
    pub(crate) fn read<R: Read>(stream: &mut Stream<R>, index: usize) -> Result<Self, Error> {
        let mut scope = Scope::new(index);
        let mut styles = Styles::default();
        let mut root = false;
        let mut depth = 0usize;
        // Which container among the root's children is open, if any, and
        // which of them have been seen.
        let mut open: Option<Container> = None;
        let mut seen = [false; 5];
        // The same for the containers only the typed read enters
        // (`specs/0032-read-breadth-styles.md`, P1 ruling 3).
        #[cfg(feature = "read")]
        let mut open_typed: Option<TypedContainer> = None;
        #[cfg(feature = "read")]
        let mut seen_typed = [false; 4];

        while let Some(token) = stream.next()? {
            // What the token asks for, decided inside the match and acted on
            // after it, because `token` borrows the stream the actions read.
            let mut skip = false;
            // An `xf` read, whether it was empty, and whether it is one of
            // `<cellStyleXfs>`.
            let mut xf: Option<(Xf, bool, bool)> = None;
            // A font, a fill or a border to read typed, with its start tag.
            #[cfg(feature = "read")]
            let mut typed: Option<(Container, read::Child)> = None;
            // A `dxf` to read, and whether it was empty.
            #[cfg(feature = "read")]
            let mut dxf: Option<bool> = None;
            // An `indexedColors` to read, and whether it was empty.
            #[cfg(feature = "read")]
            let mut indexed_colors: Option<bool> = None;
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
                    } else if depth == 2
                        && ours
                        && let Some(container) = Container::named(local)
                    {
                        if std::mem::replace(&mut seen[container as usize], true) {
                            return Err(TWO_CONTAINERS);
                        }
                        match styles.count_of(container) {
                            Some(count) => *count = Some(0),
                            None => styles.has_num_fmts |= container == Container::NumFmts,
                        }
                        if !empty {
                            open = Some(container);
                        }
                    } else if cfg!(feature = "read")
                        && depth == 2
                        && ours
                        && TypedContainer::named(local).is_some()
                    {
                        #[cfg(feature = "read")]
                        if let Some(container) = TypedContainer::named(local) {
                            if std::mem::replace(&mut seen_typed[container as usize], true) {
                                return Err(read::STYLE_UNREADABLE);
                            }
                            if !empty {
                                open_typed = Some(container);
                            }
                        }
                    } else if depth == 3
                        && ours
                        && let Some(container) = open
                        && container.counted_child() == Some(local)
                        && let Some(count) = styles.count_of(container)
                    {
                        let counted = count.unwrap_or(0).checked_add(1);
                        *count = Some(counted.ok_or(STYLE_OUT_OF_RANGE)?);
                        #[cfg(not(feature = "read"))]
                        {
                            skip = !empty;
                        }
                        #[cfg(feature = "read")]
                        {
                            typed =
                                Some((container, read::Child::of(start, &scope, index, empty)?));
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
                        xf = Some((read_xf(start, &scope, index, token.raw)?, empty, false));
                    } else if cfg!(feature = "read") && depth == 3 && ours {
                        #[cfg(feature = "read")]
                        match (open_typed, local) {
                            (Some(TypedContainer::CellStyleXfs), b"xf") => {
                                xf = Some((read_xf(start, &scope, index, token.raw)?, empty, true));
                            }
                            (Some(TypedContainer::CellStyles), b"cellStyle") => {
                                styles
                                    .cell_styles
                                    .push(read::read_named_style(start, &scope, index)?);
                                skip = !empty;
                            }
                            (Some(TypedContainer::Dxfs), b"dxf") => dxf = Some(empty),
                            (Some(TypedContainer::Colors), b"indexedColors") => {
                                if styles.indexed_colors.is_some() {
                                    return Err(read::STYLE_UNREADABLE);
                                }
                                indexed_colors = Some(empty);
                            }
                            _ => skip = !empty,
                        }
                    } else if !empty {
                        skip = true;
                    }
                    if empty {
                        scope.end();
                        depth -= 1;
                    }
                }
                Event::End(_) => {
                    // Only the root and the five containers are entered
                    // without being consumed whole, so an end tag at depth 2
                    // closes a container.
                    if depth == 2 {
                        open = None;
                        #[cfg(feature = "read")]
                        {
                            open_typed = None;
                        }
                    }
                    scope.end();
                    depth = depth.saturating_sub(1);
                }
                // Text, comments and references outside a `numFmt` or an
                // `xf` belong to nothing this layer reads.
                _ => {}
            }
            #[cfg(feature = "read")]
            if let Some((container, element)) = typed {
                let empty = element.empty();
                let into = &mut styles.typed;
                match container {
                    Container::Fonts => into
                        .fonts
                        .push(read::read_font(stream, &mut scope, index, empty)?),
                    Container::Fills => into.fills.push(read::read_fill(
                        stream,
                        &mut scope,
                        index,
                        empty,
                        Pattern::None,
                    )?),
                    Container::Borders => into.borders.push(read::read_border(
                        stream, &mut scope, index, &element, false,
                    )?),
                    // Only the three counted containers record a child here.
                    Container::NumFmts | Container::CellXfs => {}
                }
                if !empty {
                    depth -= 1;
                }
            }
            #[cfg(feature = "read")]
            if let Some(empty) = dxf {
                styles
                    .typed
                    .dxfs
                    .push(read::read_dxf(stream, &mut scope, index, empty)?);
                if !empty {
                    depth -= 1;
                }
            }
            #[cfg(feature = "read")]
            if let Some(empty) = indexed_colors {
                styles.indexed_colors =
                    Some(read::read_indexed_colors(stream, &mut scope, index, empty)?);
                if !empty {
                    depth -= 1;
                }
            }
            if let Some((mut xf, empty, named)) = xf {
                if !empty {
                    let spans = consume_subtree(stream, &mut scope, index, Some(&mut xf.raw))?;
                    xf.alignment = spans.alignment;
                    xf.protection = spans.protection;
                    #[cfg(feature = "read")]
                    {
                        xf.alignment_value = spans.alignment_value;
                        xf.protection_value = spans.protection_value;
                    }
                    depth -= 1;
                }
                #[cfg(feature = "read")]
                if named {
                    styles.cell_style_xfs.push(xf);
                    continue;
                }
                let _ = named;
                styles.cell_xfs.push(xf);
            } else if skip {
                consume_subtree(stream, &mut scope, index, None)?;
                depth -= 1;
            }
        }
        if !root {
            return Err(NOT_STYLE_SHEET);
        }
        // A named style naming a format past `<cellStyleXfs>`.
        #[cfg(feature = "read")]
        if styles.cell_styles.iter().any(|named| {
            usize::try_from(named.xf_id).map_or(true, |at| at >= styles.cell_style_xfs.len())
        }) {
            return Err(read::STYLE_UNREADABLE);
        }
        Ok(styles)
    }
}

/// The containers among the root's children this layer reads or counts the
/// children of, in the schema's order.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Container {
    NumFmts,
    Fonts,
    Fills,
    Borders,
    CellXfs,
}

impl Container {
    /// The container a root child's local name is, if any.
    fn named(local: &[u8]) -> Option<Self> {
        Some(match local {
            b"numFmts" => Self::NumFmts,
            b"fonts" => Self::Fonts,
            b"fills" => Self::Fills,
            b"borders" => Self::Borders,
            b"cellXfs" => Self::CellXfs,
            _ => return None,
        })
    }

    /// The child a counted container's count counts.
    fn counted_child(self) -> Option<&'static [u8]> {
        match self {
            Self::Fonts => Some(b"font"),
            Self::Fills => Some(b"fill"),
            Self::Borders => Some(b"border"),
            Self::NumFmts | Self::CellXfs => None,
        }
    }
}

/// The containers among the root's children only the typed read enters
/// (`specs/0032-read-breadth-styles.md`). The Cargo feature `read`.
#[cfg(feature = "read")]
#[derive(Clone, Copy, PartialEq, Eq)]
enum TypedContainer {
    CellStyleXfs,
    CellStyles,
    Dxfs,
    Colors,
}

#[cfg(feature = "read")]
impl TypedContainer {
    /// The container a root child's local name is, if any.
    fn named(local: &[u8]) -> Option<Self> {
        Some(match local {
            b"cellStyleXfs" => Self::CellStyleXfs,
            b"cellStyles" => Self::CellStyles,
            b"dxfs" => Self::Dxfs,
            b"colors" => Self::Colors,
            _ => return None,
        })
    }
}

#[cfg(not(feature = "read"))]
struct TypedContainer;

#[cfg(not(feature = "read"))]
impl TypedContainer {
    /// No container without the feature.
    fn named(_: &[u8]) -> Option<Self> {
        None
    }
}

impl Styles {
    /// The count a counted container keeps.
    fn count_of(&mut self, container: Container) -> Option<&mut Option<u32>> {
        match container {
            Container::Fonts => Some(&mut self.fonts),
            Container::Fills => Some(&mut self.fills),
            Container::Borders => Some(&mut self.borders),
            Container::NumFmts | Container::CellXfs => None,
        }
    }
}

/// What a styles part is refused with, one message each, catalogued in
/// [`crate::package`]'s module header.
const NOT_STYLE_SHEET: Error = Error::Package("the styles part's root is not styleSheet");
const INCOMPLETE_NUM_FMT: Error = Error::Package("a numFmt lacks numFmtId or formatCode");
const NOT_A_NUMBER: Error = Error::Package("a style attribute is not a number");
const TWO_CONTAINERS: Error =
    Error::Package("the styles part has more than one numFmts, fonts, fills, borders or cellXfs");

/// `text` as an attribute value the styles rewrite writes: escaped with the
/// crate's `escape`, then a tab, a line feed and a carriage return as
/// character references, so the value reads back equal to itself through the
/// attribute-value normalisation the reader applies (`specs/0012-styles.md`,
/// P1 ruling 3; a font's name the same way, `specs/0017-fuller-styles.md`,
/// P1 verification ruling 4).
pub(crate) fn attribute_text(text: &str) -> String {
    quick_xml::escape::escape(text)
        .replace('\t', "&#9;")
        .replace('\n', "&#10;")
        .replace('\r', "&#13;")
}

/// A `<numFmt>` element as the styles rewrite writes one: `prefix` (with its
/// colon, or empty) on the name, and the code as [`attribute_text`] writes
/// it.
pub(crate) fn num_fmt_element(prefix: &[u8], id: u32, code: &str) -> Vec<u8> {
    let escaped = attribute_text(code);
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
pub(crate) struct ScannedAttribute {
    /// The qualified name.
    pub(crate) name: std::ops::Range<usize>,
    /// `"` or `'`.
    quote: u8,
    /// The value between the quotes, still escaped.
    pub(crate) value: std::ops::Range<usize>,
}

/// A scanned start tag or empty-element tag.
pub(crate) struct ScannedTag {
    /// The position after the element's name, where its attributes begin.
    pub(crate) name_end: usize,
    pub(crate) attributes: Vec<ScannedAttribute>,
    /// The position after the tag's closing `>`.
    pub(crate) end: usize,
    /// Whether it closed with `/>`.
    empty: bool,
}

/// Scans the start tag at the front of `raw`: its attributes with their
/// quote characters and value positions, and where it ends.
///
/// The tag has already been parsed by `quick-xml`, so this only has to find
/// positions in well-formed bytes, which the parser's own attribute iterator
/// does not report; `None` for bytes that are not a start tag. The workbook
/// rewrite of `package` uses it for `calcPr` (`specs/0013-formulas.md`).
pub(crate) fn scan_start_tag(raw: &[u8]) -> Option<ScannedTag> {
    let space = |byte: u8| matches!(byte, b' ' | b'\t' | b'\n' | b'\r');
    if raw.first() != Some(&b'<') {
        return None;
    }
    let mut at = 1;
    while at < raw.len() && !space(raw[at]) && raw[at] != b'/' && raw[at] != b'>' {
        at += 1;
    }
    let name_end = at;
    let mut attributes = Vec::new();
    loop {
        while at < raw.len() && space(raw[at]) {
            at += 1;
        }
        match *raw.get(at)? {
            b'>' => {
                return Some(ScannedTag {
                    name_end,
                    attributes,
                    end: at + 1,
                    empty: false,
                });
            }
            b'/' => {
                return (raw.get(at + 1) == Some(&b'>')).then_some(ScannedTag {
                    name_end,
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
        alignment: None,
        protection: None,
        #[cfg(feature = "read")]
        alignment_value: None,
        #[cfg(feature = "read")]
        protection_value: None,
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

/// Where the first `alignment` and `protection` children of a consumed
/// element lie in the bytes it was appended to, start tag to end tag.
#[derive(Default)]
struct ChildSpans {
    alignment: Option<std::ops::Range<usize>>,
    protection: Option<std::ops::Range<usize>>,
    /// The same two children read typed. The Cargo feature `read`.
    #[cfg(feature = "read")]
    alignment_value: Option<Alignment>,
    #[cfg(feature = "read")]
    protection_value: Option<Protection>,
}

/// Consumes an element's subtree, to and including its matching end tag,
/// appending every token's bytes to `into` when there is one, and returns
/// where its first `alignment` and `protection` children in the main
/// namespace lie in `into` (none without `into`).
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
) -> Result<ChildSpans, Error> {
    let mut depth = 0usize;
    let mut spans = ChildSpans::default();
    // The direct child being read, which slot it fills and where it began.
    let mut open: Option<(bool, usize)> = None;
    while let Some(token) = stream.next()? {
        let at = into.as_deref().map_or(0, Vec::len);
        if let Some(into) = into.as_deref_mut() {
            into.extend_from_slice(token.raw);
        }
        let after = into.as_deref().map_or(0, Vec::len);
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start)?;
                if depth == 0 && into.is_some() {
                    let (namespace, local) = scope.resolve_element(start.name())?;
                    let which = match local {
                        b"alignment" => Some(true),
                        b"protection" => Some(false),
                        _ => None,
                    };
                    if let Some(which) = which
                        && MAIN.contains(&namespace.unwrap_or_default())
                    {
                        let slot = if which {
                            &mut spans.alignment
                        } else {
                            &mut spans.protection
                        };
                        if slot.is_none() {
                            #[cfg(feature = "read")]
                            if which {
                                spans.alignment_value =
                                    Some(read::read_alignment(start, scope, index)?);
                            } else {
                                spans.protection_value =
                                    Some(read::read_protection(start, scope, index)?);
                            }
                            if empty {
                                *slot = Some(at..after);
                            } else {
                                open = Some((which, at));
                            }
                        }
                    }
                }
                if empty {
                    scope.end();
                } else {
                    depth += 1;
                }
            }
            Event::End(_) => {
                scope.end();
                if depth == 0 {
                    return Ok(spans);
                }
                depth -= 1;
                if depth == 0
                    && let Some((which, from)) = open.take()
                {
                    let slot = if which {
                        &mut spans.alignment
                    } else {
                        &mut spans.protection
                    };
                    *slot = Some(from..after);
                }
            }
            _ => {}
        }
    }
    Err(Error::Xml(index))
}
