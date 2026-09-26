//! Reading what a worksheet says, and the values a cell edit writes.
//!
//! `specs/0010-reading-cells.md`, "Module `cells`". P1 is the shared strings
//! table, P2 the cell reader, and P3 the typed views' tests, the Limits
//! section below and the size record.
//!
//! Nothing here evaluates or converts, and the readers write nothing:
//! [`Value`] is what [`crate::package::Package::set_cell`] writes, in the
//! forms `specs/0011-editing-cells.md`, "Module `cells`", gives. A shared
//! string item is the text a producer put there, unescaped and not trimmed:
//! the format says `xml:space="preserve"` on 110010 of the corpus's 110666
//! `<t>` elements and this reader would keep the spaces either way, because
//! trimming would be a change no caller asked for.
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
//! One function, `text_of`, assembles the text of every simple-content
//! element this crate reads: `<t>`, `<v>` and `<f>`
//! (`specs/0010-reading-cells.md`, P1 ruling 4). The three rules below
//! therefore hold once rather than once per element. One function above it,
//! `text_under`, reads a run container, and both containers the format has
//! take it: an `<si>` of the shared strings table and an `<is>` of an inline
//! string cell, so the `<rPh>` rule is written once as well.
//!
//! # Line ends
//!
//! XML 1.0 section 2.11 requires a processor to normalise a CR LF pair and a
//! lone CR to a single LF before parsing, so every conforming reader sees LF:
//! expat-based readers, and therefore openpyxl, do, and so do Excel and
//! LibreOffice. `quick-xml` 0.42 does NOT, and hands the bytes over as
//! written, so the normalisation happens here. A CR reached through a
//! character reference (`&#13;`) is KEPT, as 2.11 says: the reference stands
//! for a character and is not part of a line end.
//!
//! The corpus holds sixteen carriage returns inside `<t>` text, in eleven
//! items over ten workbooks, which is why `--shared-strings` reports 1497091
//! bytes and not the 1497107 a reader without this normalisation reports
//! (`specs/0010-reading-cells.md`, finding P1-F1).
//!
//! # CDATA
//!
//! A CDATA section inside a text element is that element's text, verbatim
//! once line ends are normalised: `<t><![CDATA[a]]></t>` reads "a" exactly as
//! `<t>a</t>` does, because CDATA is a way of writing characters and not a
//! different kind of content. The corpus carries no CDATA section at all, so
//! this module's tests are the only thing that proves it.
//!
//! # Elements inside text
//!
//! `<t>`, `<v>` and `<f>` are simple content in the schema: they hold
//! characters and nothing else. An element inside one is refused rather than
//! skipped or flattened, for the same reason an unknown reference is: a value
//! with a hole in it is not the value.
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
//! table is the only WHOLE-PART cost in this module. It is not the only
//! whole-element cost: one cell's value, formula text or inline string is
//! assembled into a `String` as it is read, so the bound this module offers
//! is one token, one cell and one such element at a time, which is the model
//! `specs/0010-reading-cells.md`, "32-bit and streaming", states. The
//! corpus's longest value is 192 bytes and its longest formula 511.
//!
//! [`Cells`] holds one token and one [`Cell`] at a time and never seeks, so a
//! worksheet part of any size streams: the corpus's largest worksheet is
//! read the same way as its smallest. What a caller then does with the cells
//! is the caller's own bound.
//!
//! What this module does NOT do, each a decision rather than an omission
//! (`specs/0010-reading-cells.md`, "Module `cells`"):
//!
//! - No SEEKING, in either reader. A worksheet is read forwards once, and a
//!   caller that wants one cell of a large sheet pays the walk to it; there
//!   is no index of a sheet's cells to seek with, because the format keeps
//!   none. The shared strings table is the one thing read whole, above.
//! - No EVALUATION. A formula is its text, and the value beside it is the
//!   cached result a producer wrote: this layer neither computes the one nor
//!   checks it against the other, and a stale cached value is reported as it
//!   was found. That is openpyxl's contract too, and `CLAUDE.md` makes it a
//!   limit by design rather than a gap.
//! - No DATE CONVERSION. [`Kind::Date`] carries its ISO 8601 text, and a
//!   number carrying a date format is a number here: the format's serial
//!   numbers need the workbook's 1900/1904 base and a calendar, which is
//!   part 5's work and the dependency rule's reason for keeping it behind a
//!   feature.
//! - No LOOKUP of a style. A cell's `s` is carried as the index it is; the
//!   styles part is not read in this part, so a caller asking what the cell
//!   looks like gets the number and not an answer.
//! - Attributes this layer does not model are not READ, and a part 4 rewrite
//!   copies them raw: `<f>`'s beyond `t`, `si` and `ref`, which is `ca`,
//!   `aca` and the rest (`ref` IS modelled, as [`Formula::reference`], and
//!   the three that are read are named here so the list is a boundary and
//!   not a gesture), `<row>`'s own attributes, `<c>`'s beyond `r`, `s` and
//!   `t`, [`FormulaKind::DataTable`]'s (which the corpus has no witness
//!   for), and `<sst>`'s `count` and `uniqueCount`, which are ignored in
//!   favour of the items that are there.
//! - No SHEET-SIZE limit. 1048576 by 16384 is Excel's limit and not the
//!   format's, so a reference past it is read rather than refused, and
//!   [`CellRef`]'s two `u32` fields are what a consumer checks.
//!
//! What [`Value`] writes, each a decision of `specs/0011-editing-cells.md`,
//! "Module `cells`":
//!
//! - Four kinds only: a number, a text, a boolean and, from part 6, a normal
//!   formula. No date (part 5: a number with a date style is written as the
//!   number it is) and no error value.
//! - A formula is written as `<f>` alone: no `t`, no `<v>`, no `ca`, no
//!   `ref` and no `si`, so no cached value and no shared, array or data
//!   table formula is ever written (`specs/0013-formulas.md`, "Module
//!   `cells`"). Its text is escaped for `&`, `<` and `>` with quotes left
//!   literal and a carriage return as a character reference (P1 ruling 2).
//!   An empty text, a text starting with `=` and a text holding a character
//!   XML 1.0 forbids are refused at write; nothing else about the text is
//!   checked, because emend does not parse formulas, and a text an
//!   application rejects is the caller's.
//! - A text is always an inline string, with `xml:space="preserve"` always
//!   written and a carriage return written as a character reference; never
//!   a shared string, and never rich text.
//! - A number is written as its `Display` when it is zero or its magnitude
//!   is at least `1e-7` and below `1e21`, else in its `{:E}` form. A number
//!   that is not finite, and a text holding a character XML 1.0 forbids, are
//!   refused at write.
//! - No style. A new cell carries no `s`; a replaced one keeps its own.

use quick_xml::events::{BytesRef, BytesStart, Event};

use crate::archive::Error;
use crate::package::{MAIN, Package};
use crate::xml::{Scope, Stream};

use std::io::{Read, Seek};
use std::marker::PhantomData;

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
    ///
    /// Each item is assembled by `text_under`, the same function the cell
    /// reader gives an `<is>` to, so line ends are normalised, a CDATA
    /// section counts as text, an element inside a `<t>` is refused, and the
    /// `<rPh>` rule is written once for both containers.
    pub(crate) fn read<R: Read>(stream: &mut Stream<R>, index: usize) -> Result<Self, Error> {
        let mut scope = Scope::new(index);
        let mut items: Vec<String> = Vec::new();
        let mut root = false;

        while let Some(token) = stream.next()? {
            // Set when the token opened an `<si>`, whose text is a new item.
            // The read happens after this match, because `token` borrows the
            // stream that `text_under` has to read from.
            let mut open_item = false;
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
                    } else if ours && local == b"si" {
                        // An empty `<si/>` is an empty string, not a missing
                        // one, and has no content for `text_under` to read.
                        if empty {
                            items.push(String::new());
                        } else {
                            open_item = true;
                        }
                    }
                    if empty {
                        scope.end();
                    }
                }
                Event::End(_) => scope.end(),
                // Text and references OUTSIDE an `<si>` are not any item's:
                // whitespace between elements, and a reference in a part of
                // the tree this layer does not read, which is therefore not
                // expanded and not refused either.
                _ => {}
            }
            if open_item {
                items.push(text_under(stream, &mut scope, index)?);
            }
        }
        if !root {
            return Err(Error::Package("the shared strings part's root is not sst"));
        }
        Ok(Self { items })
    }

    /// The text a cell of kind [`Kind::SharedString`] names, or the cell's own
    /// value for every other kind.
    ///
    /// A shared string cell carries an index into this table as its value, so
    /// reading its text is a table lookup and not a property of the cell. For
    /// every other kind there is nothing to look up and the cell's own value
    /// is the answer, which lets a caller run every cell through this one call
    /// (`specs/0010-reading-cells.md`, "Module `cells`").
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a shared string index is not a number" for a value
    /// that is not a decimal `u32`, and "a shared string index is out of
    /// range" for one past the end of the table. A workbook with no shared
    /// strings relationship gives an empty table, so a shared string cell in
    /// such a workbook is out of range rather than a case of its own. A
    /// shared string cell with NO value names no index at all, and resolves to
    /// `None` rather than being refused: there is nothing malformed about it
    /// to report.
    ///
    /// The one lifetime covers both arguments because the answer comes from
    /// one or the other: the table's own item for a shared string, and the
    /// cell's own value for every other kind.
    pub fn resolve<'a>(&'a self, cell: &'a Cell) -> Result<Option<&'a str>, Error> {
        if cell.kind != Kind::SharedString {
            return Ok(cell.value.as_deref());
        }
        let Some(value) = cell.value.as_deref() else {
            return Ok(None);
        };
        let index =
            decimal(value).ok_or(Error::Package("a shared string index is not a number"))?;
        match self.get(index) {
            Some(item) => Ok(Some(item)),
            None => Err(Error::Package("a shared string index is out of range")),
        }
    }
}

/// One cell's position, both 1-based.
///
/// `A1` is `{ row: 1, col: 1 }`. No sheet-size limit is enforced: 1048576
/// rows by 16384 columns is Excel's limit and not the format's, and a
/// consumer that needs it checks these two numbers
/// (`specs/0010-reading-cells.md`, "Module `cells`").
///
/// 1-based is a property of every reference the reader hands out, not a
/// promise the type enforces: `parse` refuses a leading zero, `row_number`
/// refuses `r="0"`, a row with no `r` counts from 1 and a cell with no `r`
/// takes the previous column plus 1. A value constructed by hand can hold a
/// zero, and `a1()` will write it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellRef {
    /// The row, 1-based.
    pub row: u32,
    /// The column, 1-based: `A` is 1, `Z` 26, `AA` 27.
    pub col: u32,
}

impl CellRef {
    /// Reads an A1 reference.
    ///
    /// Accepts one or more ASCII capital letters then one or more ASCII
    /// digits with no leading zero, and nothing else: no `$`, no lower case,
    /// no range, no sheet name, no trailing bytes. `None` for anything else,
    /// and for a column or row that does not fit `u32`.
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let letters = bytes
            .iter()
            .take_while(|byte| byte.is_ascii_uppercase())
            .count();
        let (letters, digits) = bytes.split_at(letters);
        if letters.is_empty() || digits.is_empty() {
            return None;
        }
        // A leading zero is refused rather than accepted and folded: `A01`
        // is not how the format writes row 1, and a reader that took it
        // would report a reference no producer wrote.
        if digits[0] == b'0' || !digits.iter().all(u8::is_ascii_digit) {
            return None;
        }
        // `u64` accumulators and a check per step: the whole point is to
        // return `None` for a value too large, so nothing may wrap on the
        // way to finding that out, on a 32-bit target either.
        let mut col: u64 = 0;
        for letter in letters {
            col = col * 26 + u64::from(letter - b'A') + 1;
            if col > u64::from(u32::MAX) {
                return None;
            }
        }
        let mut row: u64 = 0;
        for digit in digits {
            row = row * 10 + u64::from(digit - b'0');
            if row > u64::from(u32::MAX) {
                return None;
            }
        }
        Some(Self {
            row: u32::try_from(row).ok()?,
            col: u32::try_from(col).ok()?,
        })
    }

    /// Writes the reference back the way the format spells it.
    pub fn a1(&self) -> String {
        // Enough for `u32::MAX`, which is seven letters in this base-26
        // spelling, and for the ten digits of the row.
        let mut letters = [0u8; 7];
        let mut written = 0;
        let mut col = self.col;
        while col > 0 && written < letters.len() {
            letters[written] = b'A' + ((col - 1) % 26) as u8;
            written += 1;
            col = (col - 1) / 26;
        }
        let mut out = String::with_capacity(written + 10);
        for letter in letters[..written].iter().rev() {
            out.push(char::from(*letter));
        }
        out.push_str(&self.row.to_string());
        out
    }
}

/// A rectangle of cells, both corners included: a merge, or an array
/// formula's `ref` (`specs/0018-rows-and-columns.md`, "Module `cells`:
/// `Area`").
///
/// [`Area::parse`] gives the corners normalised, `first` the smaller row and
/// the smaller column; a value built by hand may not be, and the calls that
/// take one normalise it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Area {
    /// The top-left corner.
    pub first: CellRef,
    /// The bottom-right corner.
    pub last: CellRef,
}

impl Area {
    /// Reads `A1:B2` or `A1` (one cell), each half as [`CellRef::parse`]
    /// reads it; `None` for anything else.
    pub fn parse(value: &[u8]) -> Option<Self> {
        let mut halves = value.splitn(2, |byte| *byte == b':');
        let a = CellRef::parse(halves.next()?)?;
        let b = match halves.next() {
            Some(half) => CellRef::parse(half)?,
            None => a,
        };
        Some(Area { first: a, last: b }.normalised())
    }

    /// Writes `A1:B2`, or `A1` when the corners coincide.
    pub fn a1(&self) -> String {
        if self.first == self.last {
            self.first.a1()
        } else {
            format!("{}:{}", self.first.a1(), self.last.a1())
        }
    }

    /// The number of cells the area covers, `u64::MAX` for an area built by
    /// hand so large that `u64` cannot count it.
    pub fn cells(&self) -> u64 {
        let rows = u64::from(self.last.row.abs_diff(self.first.row)) + 1;
        let cols = u64::from(self.last.col.abs_diff(self.first.col)) + 1;
        rows.saturating_mul(cols)
    }

    /// The same area with `first` the smaller row and column.
    pub(crate) fn normalised(self) -> Self {
        Area {
            first: CellRef {
                row: self.first.row.min(self.last.row),
                col: self.first.col.min(self.last.col),
            },
            last: CellRef {
                row: self.first.row.max(self.last.row),
                col: self.first.col.max(self.last.col),
            },
        }
    }

    /// Whether `cell` lies inside the area.
    pub(crate) fn holds(&self, cell: CellRef) -> bool {
        (self.first.row..=self.last.row).contains(&cell.row)
            && (self.first.col..=self.last.col).contains(&cell.col)
    }

    /// Whether the two areas share a cell.
    pub(crate) fn overlaps(&self, other: &Area) -> bool {
        self.first.row <= other.last.row
            && other.first.row <= self.last.row
            && self.first.col <= other.last.col
            && other.first.col <= self.last.col
    }
}

/// What a cell's `t` attribute says its value is.
///
/// Any other `t` is refused with "a cell has a type this layer does not
/// know": a reader that guessed would report a wrong value as a right one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    /// `t` absent or `n`: the value is a number's text as written.
    Number,
    /// `t="s"`: the value is an index into the shared strings table.
    SharedString,
    /// `t="inlineStr"`: the value is the text under the cell's `<is>`.
    InlineString,
    /// `t="str"`: the value is a formula's cached string result.
    FormulaString,
    /// `t="b"`: the value is `1` or `0`.
    Boolean,
    /// `t="e"`: the value is an error's text, such as `#REF!`.
    Error,
    /// `t="d"`: the value is an ISO 8601 date, carried as its text.
    Date,
}

impl Kind {
    /// The kind a `t` attribute's bytes name.
    fn parse(value: &[u8]) -> Result<Self, Error> {
        match value {
            b"n" => Ok(Kind::Number),
            b"s" => Ok(Kind::SharedString),
            b"inlineStr" => Ok(Kind::InlineString),
            b"str" => Ok(Kind::FormulaString),
            b"b" => Ok(Kind::Boolean),
            b"e" => Ok(Kind::Error),
            b"d" => Ok(Kind::Date),
            _ => Err(Error::Package("a cell has a type this layer does not know")),
        }
    }
}

/// What a formula's `t` attribute says it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FormulaKind {
    /// `t` absent or `normal`.
    Normal,
    /// `t="shared"`: one of a group sharing one expression.
    ///
    /// `index` is `si`, and `master` is whether this `<f>` carried a `ref`,
    /// which is how the format marks the cell that holds the expression the
    /// others follow. A shared `<f>` with no `si` is refused.
    Shared {
        /// The `si` attribute, the group's number.
        index: u32,
        /// Whether this is the group's master, that is, whether `ref` was
        /// present.
        master: bool,
    },
    /// `t="array"`.
    Array,
    /// `t="dataTable"`, accepted by name only.
    ///
    /// The corpus holds no witness (`specs/0010-reading-cells.md`, "Facts
    /// planning checked"), so its own attributes are not modelled and part
    /// 4's rewrite copies them raw.
    DataTable,
}

/// One cell's formula, as written and never evaluated.
///
/// `ca`, `aca` and every other attribute of `<f>` are not modelled here;
/// part 4's rewrite copies them raw. emend does not evaluate formulas, by
/// design, as openpyxl does not (`CLAUDE.md`, "What emend is").
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Formula {
    /// The formula unescaped, without a leading `=`, and empty for a shared
    /// follower, which carries no expression of its own.
    pub text: String,
    /// What the `t` attribute said.
    pub kind: FormulaKind,
    /// The `ref` attribute as written, when present: a range for a shared
    /// master or an array formula.
    pub reference: Option<String>,
}

/// One cell of a worksheet, as the part writes it.
///
/// Nothing is trimmed, converted or evaluated. A `Number` cell's `value` is
/// the number's text as the producer wrote it, so `1.10` stays `1.10` and
/// [`Cell::number`] parses it on request; a `Date` cell carries its ISO text;
/// a `SharedString` cell carries an index, which
/// [`SharedStrings::resolve`] looks up.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    /// Where the cell is: its own `r` when it had one, else the column after
    /// the previous cell of its row.
    pub reference: CellRef,
    /// The `s` attribute, an index into the styles part, absent when the
    /// cell carried none. The styles themselves are not read here.
    pub style: Option<u32>,
    /// What the `t` attribute said.
    pub kind: Kind,
    /// The text of `<v>` unescaped, `Some("")` for `<v/>`, the text under
    /// `<is>` for an inline string, and `None` when the cell has neither.
    pub value: Option<String>,
    /// The cell's formula, when it has one.
    pub formula: Option<Formula>,
}

impl Cell {
    /// The value as an `f64`, for a `Number` cell whose value parses.
    ///
    /// `None` for every other kind, for a `Number` cell with no value, and
    /// for a value `str::parse::<f64>` refuses. Never an error: a producer's
    /// number text that the standard library will not parse is a fact about
    /// the workbook for the caller to see, not a malformed package.
    ///
    /// The parse is `str::parse::<f64>` and nothing else, so what it takes is
    /// wider than what a spreadsheet writes. A caller that needs the narrower
    /// thing reads [`Cell::value`] itself. Each case below is measured in
    /// `cells::tests` and not merely described here, by the test named
    /// against it: three tests do the work, not one.
    ///
    /// - `inf`, `INF`, `infinity` and `nan`, `NaN` parse, and `-inf` with
    ///   them, so a cell whose text is `nan` reads as `Some(NaN)` and not as
    ///   `None`. Those six spellings are the measured ones
    ///   (`the_parse_is_the_standard_librarys_and_wider_than_the_format`);
    ///   the standard library takes others, and this comment does not say
    ///   which, because nothing here measures them.
    /// - a magnitude past `f64` becomes an infinity rather than a refusal:
    ///   `1e309` is `Some(inf)`. So an infinity a caller sees may be the
    ///   producer's own word or its overflow, and this layer does not
    ///   distinguish them
    ///   (`the_parse_is_the_standard_librarys_and_wider_than_the_format`).
    /// - a leading `+` is taken, unlike in the indices `decimal` reads: this
    ///   is a producer's number, not one of the format's tokens (same test).
    /// - surrounding space is not taken: `" 1"` and `"1 "` are `None`, as is
    ///   an empty value
    ///   (`a_number_cell_whose_text_is_not_a_number_has_none`).
    /// - `1.10` is `1.1`, and `-0` keeps its sign as `Some(-0.0)`, which is
    ///   equal to `0.0` without being the same bits
    ///   (`a_numbers_text_is_read_as_an_f64`).
    pub fn number(&self) -> Option<f64> {
        if self.kind != Kind::Number {
            return None;
        }
        self.value.as_deref()?.parse().ok()
    }

    /// The value as a `bool`, for a `Boolean` cell.
    ///
    /// `Some(true)` for `1` and `Some(false)` for `0`, which is what the
    /// format writes; `None` for anything else, including `TRUE`, and for
    /// every other kind.
    pub fn boolean(&self) -> Option<bool> {
        if self.kind != Kind::Boolean {
            return None;
        }
        match self.value.as_deref()? {
            "1" => Some(true),
            "0" => Some(false),
            _ => None,
        }
    }

    /// The value, for an `Error` cell: the error's own text, such as `#REF!`.
    pub fn error(&self) -> Option<&str> {
        if self.kind != Kind::Error {
            return None;
        }
        self.value.as_deref()
    }
}

/// What a cell can be set to (`specs/0011-editing-cells.md`, "Module
/// `cells`: what can be written").
///
/// A date is part 5's (a number with a date style is written as the number it
/// is), a formula part 6's (`specs/0013-formulas.md`, "Module `cells`"), and
/// an error value has no use a caller has asked for. A value is checked when
/// the package is written, not when it is made: see [`Value::Number`],
/// [`Value::Text`] and [`Value::Formula`] for what is refused.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// A number, written as a cell with no `t`.
    ///
    /// The text is `f64`'s `Display` when the value is zero or its magnitude
    /// is at least `1e-7` and below `1e21`, else its `{:E}` form, so a value
    /// prints as `2.5`, `100`, `-0`, `0.30000000000000004`, `1E21` or
    /// `1.5E-8` and never as hundreds of digits. The boundary is
    /// ECMAScript's `Number::toString`, a published rule. A value that is
    /// not finite is refused: the format has no NaN and no infinity.
    Number(f64),
    /// A text, written as an inline string (`t="inlineStr"`).
    ///
    /// Escaped for XML, with a carriage return written as `&#13;` so a
    /// reader's line-end normalisation keeps it; a tab and a line feed are
    /// written as they are. A text holding a character XML 1.0 forbids
    /// (U+0000 to U+0008, U+000B, U+000C, U+000E to U+001F, U+FFFE, U+FFFF)
    /// is refused.
    Text(String),
    /// A boolean, written as `t="b"` with `1` or `0`.
    Boolean(bool),
    /// A normal formula, written as `<c r="..."><f>text</f></c>`: no `t`, no
    /// `<v>`, no `ca`, `ref` or `si`, so the cell has no cached value until
    /// an application computes it. emend never evaluates it.
    ///
    /// The text is what [`Formula::text`] reads back, without a leading `=`.
    /// It is escaped with `quick_xml`'s `partial_escape`: `&`, `<` and `>`,
    /// and a carriage return as `&#13;`, so quotes stay literal as openpyxl
    /// writes them (`specs/0013-formulas.md`, P1 ruling 2). A text that is
    /// empty, that starts with `=`, or that holds a character XML 1.0
    /// forbids (the set [`Value::Text`] lists) is refused. Nothing else is
    /// checked: emend does not parse formulas.
    Formula(String),
}

impl Value {
    /// Refuses a value the format cannot carry, with the message
    /// [`crate::package::Package::write_to`] reports for it.
    pub(crate) fn check(&self) -> Result<(), Error> {
        match self {
            Value::Number(number) if !number.is_finite() => Err(Error::Package(
                "a number that is not finite cannot be written",
            )),
            Value::Text(text) if text.chars().any(forbidden_in_xml) => {
                Err(Error::Package("a text holds a character XML cannot carry"))
            }
            Value::Formula(text) if text.is_empty() => {
                Err(Error::Package("a formula text is empty"))
            }
            Value::Formula(text) if text.starts_with('=') => {
                Err(Error::Package("a formula text starts with an equals sign"))
            }
            Value::Formula(text) if text.chars().any(forbidden_in_xml) => Err(Error::Package(
                "a formula text holds a character XML cannot carry",
            )),
            _ => Ok(()),
        }
    }

    /// The cell element for this value at `reference`, with `style` (an `s`
    /// attribute's bytes as the input wrote them) after `r` when there is
    /// one. One line, no whitespace added. The value has been checked.
    ///
    /// `prefix` is the qualified-name prefix the elements take, with its
    /// colon (`b"x:"`), or empty: the prefix of the element the cell goes
    /// into, so a part that binds the main namespace to a prefix gets its new
    /// elements in that namespace too.
    pub(crate) fn element(
        &self,
        reference: CellRef,
        style: Option<&[u8]>,
        prefix: &[u8],
    ) -> Vec<u8> {
        let p = prefix;
        let mut out = [b"<".as_slice(), p, b"c r=\""].concat();
        out.extend_from_slice(reference.a1().as_bytes());
        out.push(b'"');
        if let Some(style) = style {
            out.extend_from_slice(b" s=\"");
            out.extend_from_slice(style);
            out.push(b'"');
        }
        match self {
            Value::Number(number) => {
                out.extend_from_slice(&[b"><".as_slice(), p, b"v>"].concat());
                out.extend_from_slice(number_text(*number).as_bytes());
                out.extend_from_slice(&[b"</".as_slice(), p, b"v></", p, b"c>"].concat());
            }
            Value::Text(text) => {
                out.extend_from_slice(
                    &[
                        b" t=\"inlineStr\"><".as_slice(),
                        p,
                        b"is><",
                        p,
                        b"t xml:space=\"preserve\">",
                    ]
                    .concat(),
                );
                // A reader normalises a raw carriage return away (part 3 P1
                // finding P1-F1), so it goes out as a character reference.
                // quick-xml 0.42.0's `escape` already writes it as `&#13;`;
                // the `replace` is a no-op kept in case a later version
                // leaves it raw.
                let escaped = quick_xml::escape::escape(text.as_str());
                out.extend_from_slice(escaped.replace('\r', "&#13;").as_bytes());
                out.extend_from_slice(
                    &[b"</".as_slice(), p, b"t></", p, b"is></", p, b"c>"].concat(),
                );
            }
            Value::Boolean(value) => {
                let digit: &[u8] = if *value { b"1" } else { b"0" };
                out.extend_from_slice(
                    &[
                        b" t=\"b\"><".as_slice(),
                        p,
                        b"v>",
                        digit,
                        b"</",
                        p,
                        b"v></",
                        p,
                        b"c>",
                    ]
                    .concat(),
                );
            }
            Value::Formula(text) => {
                out.extend_from_slice(&[b"><".as_slice(), p, b"f>"].concat());
                let escaped = quick_xml::escape::partial_escape(text.as_str());
                out.extend_from_slice(escaped.as_bytes());
                out.extend_from_slice(&[b"</".as_slice(), p, b"f></", p, b"c>"].concat());
            }
        }
        out
    }
}

/// A number's text as [`Value::Number`] writes it. `number` is finite.
pub(crate) fn number_text(number: f64) -> String {
    let magnitude = number.abs();
    if number == 0.0 || (1e-7..1e21).contains(&magnitude) {
        format!("{number}")
    } else {
        format!("{number:E}")
    }
}

/// Whether XML 1.0 forbids `c` in a document, as [`Value::Text`] lists.
///
/// A number format code is refused on the same set
/// (`specs/0012-styles.md`, "Module `styles`: what can be added").
pub(crate) fn forbidden_in_xml(c: char) -> bool {
    matches!(
        c,
        '\u{0}'..='\u{8}' | '\u{B}' | '\u{C}' | '\u{E}'..='\u{1F}' | '\u{FFFE}' | '\u{FFFF}'
    )
}

/// A streaming read over one worksheet part.
///
/// Built by [`Package::cells`]. It reads forwards once, holds one cell at a
/// time and never seeks, so a worksheet part is never held whole, however
/// large it is (`specs/0010-reading-cells.md`, "32-bit and streaming"). One
/// ELEMENT is held whole while it is read: a `<v>`, an `<f>` or an `<is>`
/// becomes a `String`, so a single value larger than the address space is
/// beyond this reader even though the part around it is not.
///
/// Elements are resolved through [`Scope`] and matched only in the main
/// namespace, both spellings. An element in any other namespace, and any
/// element this layer does not know, is skipped with its subtree, so
/// `extLst`, conditional formatting, data validation and the rest cost
/// nothing but the tokens they take to walk past.
///
/// `<row>` without `r` is the previous row plus one, 1 for the first; `<c>`
/// without `r` is the previous cell's column plus one within its row, 1 for
/// the first. A `<c>` whose row disagrees with its enclosing `<row>` is NOT
/// refused: the cell's own reference wins, which is what every other reader
/// does, and the row's attributes are not modelled here.
///
/// # Errors
///
/// Each item may be [`Error::Package`] with one of "the worksheet part's
/// root is not worksheet", "a cell reference is not of the form A1", "a row
/// number is not a number", "a cell's style index is not a number", "a cell
/// has a type this layer does not know", "a formula has a type this layer
/// does not know", "a shared formula has no index", "a text element holds an
/// element" or "a cell holds a reference this layer cannot expand", or
/// [`Error::Xml`] for bytes that are not well-formed XML or not UTF-8. After
/// an error the iterator is spent and yields `None`: a reader that carried on
/// from an unknown position would be guessing.
pub struct Cells<'p, R: Read + Seek> {
    /// The walk itself, compiled once for every reader type.
    walk: Walk<'p>,
    /// `Cells` borrows the package it streams from, and the reader type is
    /// named so the type says whose borrow it holds.
    package: PhantomData<&'p mut Package<R>>,
}

impl<'p, R: Read + Seek> Cells<'p, R> {
    /// Opens a worksheet part's reader. `index` is the entry's index.
    pub(crate) fn new(reader: Box<dyn Read + 'p>, index: usize) -> Result<Self, Error> {
        Ok(Self {
            walk: Walk::new(reader, index)?,
            package: PhantomData,
        })
    }
}

impl<R: Read + Seek> Iterator for Cells<'_, R> {
    type Item = Result<Cell, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.walk.next()
    }
}

/// The state of a [`Cells`], with every body of its walk, which names no
/// reader type (`specs/0016-new-workbook-and-sheets.md`, "P5" and "Planning
/// rulings during P5", ruling 1).
struct Walk<'p> {
    /// The part's tokens.
    ///
    /// Boxed because [`crate::archive::Archive::read`] hands back an opaque
    /// reader whose type cannot be named in a struct field. One allocation
    /// per worksheet opened, beside a part that is megabytes, and it keeps
    /// the `zip` crate's own types out of this crate's API.
    stream: Stream<Box<dyn Read + 'p>>,
    scope: Scope,
    /// The entry this part came from, carried into errors and nothing else.
    index: usize,
    /// Whether the root has been seen and checked.
    root: bool,
    /// Whether a `<sheetData>` is open.
    data: bool,
    /// The open `<row>`'s number, `None` when none is open.
    row: Option<u32>,
    /// The last row number seen, 0 before the first, which is what a `<row>`
    /// without `r` counts from. Kept across the end tag, unlike `row`.
    last_row: u32,
    /// The previous cell's column in the open row, 0 before its first cell.
    col: u32,
    /// Set once the part is spent or has been refused.
    done: bool,
}

impl<'p> Walk<'p> {
    /// The body of [`Cells`]' constructor.
    fn new(reader: Box<dyn Read + 'p>, index: usize) -> Result<Self, Error> {
        Ok(Self {
            stream: Stream::new(reader, index)?,
            scope: Scope::new(index),
            index,
            root: false,
            data: false,
            row: None,
            last_row: 0,
            col: 0,
            done: false,
        })
    }

    /// Reads forwards to the next cell, or to the end of the part.
    fn advance(&mut self) -> Result<Option<Cell>, Error> {
        while let Some(token) = self.stream.next()? {
            // What the token asks for, decided inside the match and acted on
            // after it, because `token` borrows the stream those actions
            // have to read from.
            let mut opened: Option<Opening> = None;
            let mut skip = false;
            match token.event {
                Event::Start(ref start) | Event::Empty(ref start) => {
                    let empty = matches!(token.event, Event::Empty(_));
                    self.scope.start(start)?;
                    let (namespace, local) = self.scope.resolve_element(start.name())?;
                    let ours = MAIN.contains(&namespace.unwrap_or_default());
                    if !self.root {
                        if !ours || local != b"worksheet" {
                            return Err(Error::Package(
                                "the worksheet part's root is not worksheet",
                            ));
                        }
                        self.root = true;
                    } else if ours && local == b"sheetData" && !self.data {
                        // An empty `<sheetData/>` opens nothing and gives no
                        // cells.
                        self.data = !empty;
                    } else if ours && local == b"row" && self.data && self.row.is_none() {
                        let number = match row_number(start, &self.scope, self.index)? {
                            Some(number) => number,
                            // Saturating rather than wrapping: a row after
                            // `u32::MAX` cannot be numbered, and repeating
                            // the last number is the only answer left that
                            // is not a different row's.
                            None => self.last_row.saturating_add(1),
                        };
                        self.last_row = number;
                        self.col = 0;
                        if !empty {
                            self.row = Some(number);
                        }
                    } else if ours
                        && local == b"c"
                        && let Some(row) = self.row
                    {
                        opened = Some(read_opening(
                            start,
                            &self.scope,
                            row,
                            self.col,
                            self.index,
                            empty,
                        )?);
                    } else if !empty {
                        // Unknown here means unknown to this layer, in any
                        // namespace: skipping with the subtree is what makes
                        // an `extLst` cost nothing.
                        skip = true;
                    }
                    if empty {
                        self.scope.end();
                    }
                }
                Event::End(ref end) => {
                    let (namespace, local) = self.scope.resolve_element(end.name())?;
                    let ours = MAIN.contains(&namespace.unwrap_or_default());
                    // Decided before `scope.end()`, which drops what the
                    // namespace borrowed. Only the elements this reader
                    // ENTERED can reach here: a `<c>` and an unknown subtree
                    // are each consumed whole, end tag included.
                    let closes_row = ours && local == b"row";
                    let closes_data = ours && local == b"sheetData";
                    self.scope.end();
                    if closes_row {
                        self.row = None;
                    } else if closes_data {
                        self.data = false;
                    }
                }
                // Text between elements is nobody's value: a cell's value is
                // in its `<v>` or its `<is>`.
                _ => {}
            }
            if let Some(opening) = opened {
                self.col = opening.reference.col;
                if opening.empty {
                    // `<c/>`: no value and no formula, and its end tag was
                    // accounted for above.
                    return Ok(Some(Cell {
                        reference: opening.reference,
                        style: opening.style,
                        kind: opening.kind,
                        value: None,
                        formula: None,
                    }));
                }
                return Ok(Some(self.read_cell(opening)?));
            }
            if skip {
                skip_subtree(&mut self.stream, &mut self.scope, self.index)?;
            }
        }
        Ok(None)
    }

    /// Reads one open `<c>`'s children, to and including its end tag.
    fn read_cell(&mut self, opening: Opening) -> Result<Cell, Error> {
        let mut value: Option<String> = None;
        let mut formula: Option<Formula> = None;
        loop {
            let Some(token) = self.stream.next()? else {
                return Err(Error::Xml(self.index));
            };
            // Which of the cell's fields the element just opened reads into,
            // decided inside the match and acted on after it.
            let mut into: Option<Slot> = None;
            let mut skip = false;
            match token.event {
                Event::Start(ref start) | Event::Empty(ref start) => {
                    let empty = matches!(token.event, Event::Empty(_));
                    self.scope.start(start)?;
                    let (namespace, local) = self.scope.resolve_element(start.name())?;
                    let ours = MAIN.contains(&namespace.unwrap_or_default());
                    if ours && local == b"v" {
                        // `<v/>` is an empty value, not a missing one.
                        if empty {
                            value = Some(String::new());
                        } else {
                            into = Some(Slot::Value);
                        }
                    } else if ours && local == b"f" {
                        let (kind, reference) = read_formula(start, &self.scope, self.index)?;
                        if empty {
                            formula = Some(Formula {
                                text: String::new(),
                                kind,
                                reference,
                            });
                        } else {
                            into = Some(Slot::Formula(kind, reference));
                        }
                    } else if ours && local == b"is" {
                        if empty {
                            value = Some(String::new());
                        } else {
                            into = Some(Slot::Inline);
                        }
                    } else if !empty {
                        skip = true;
                    }
                    if empty {
                        self.scope.end();
                    }
                }
                // The first end tag reached is this cell's own: `<v>`, `<f>`
                // and `<is>` are each consumed whole below, and an unknown
                // child with its subtree, so no inner end tag can arrive
                // here. That is a property of how they are read, not an
                // assumption about where the end tag sits.
                Event::End(_) => {
                    self.scope.end();
                    return Ok(Cell {
                        reference: opening.reference,
                        style: opening.style,
                        kind: opening.kind,
                        value,
                        formula,
                    });
                }
                _ => {}
            }
            match into {
                Some(Slot::Value) => {
                    value = Some(text_of(&mut self.stream, &mut self.scope, self.index)?);
                }
                Some(Slot::Formula(kind, reference)) => {
                    let text = text_of(&mut self.stream, &mut self.scope, self.index)?;
                    formula = Some(Formula {
                        text,
                        kind,
                        reference,
                    });
                }
                // An inline string is a run container, exactly as an `<si>`
                // is, so it takes the same function and the same `<rPh>`
                // rule.
                Some(Slot::Inline) => {
                    value = Some(text_under(&mut self.stream, &mut self.scope, self.index)?);
                }
                None => {
                    if skip {
                        skip_subtree(&mut self.stream, &mut self.scope, self.index)?;
                    }
                }
            }
        }
    }

    /// The body of [`Cells`]' `Iterator::next`.
    fn next(&mut self) -> Option<Result<Cell, Error>> {
        if self.done {
            return None;
        }
        match self.advance() {
            Ok(Some(cell)) => Some(Ok(cell)),
            Ok(None) => {
                self.done = true;
                None
            }
            Err(error) => {
                self.done = true;
                Some(Err(error))
            }
        }
    }
}

/// What a `<c>` start tag said, before its children are read.
struct Opening {
    reference: CellRef,
    style: Option<u32>,
    kind: Kind,
    /// Whether the tag was `<c/>`, which has no children to read.
    empty: bool,
}

/// Which field of a cell the element just opened reads into.
enum Slot {
    Value,
    Formula(FormulaKind, Option<String>),
    Inline,
}

/// A decimal `u32` as this layer reads one: ASCII digits and nothing else.
///
/// Every number this layer reads out of an attribute comes through here: a
/// `<row>`'s `r`, a `<c>`'s `s`, an `<f>`'s `si` and the shared string index
/// in `SharedStrings::resolve` (P2 ruling 2). One helper, so one spelling is
/// a number everywhere or nowhere.
///
/// `u32::from_str` alone is wider than the format in two ways, both measured
/// by `what_the_standard_librarys_parse_takes_and_this_refuses`: it takes a
/// leading `+`, and it reads any number of leading zeros. This refuses both,
/// so `s="+1"`, `si="+0"` and `r="007"` are malformed rather than quietly
/// read as 1, 0 and 7. A leading `-` is not one of the two: an unsigned
/// `from_str` refuses `"-1"` itself, so `decimal` and the standard library
/// agree there and the refusal a caller sees for a signed spelling is not
/// this helper's doing.
///
/// What is left after the two checks is a canonical decimal, and `from_str`
/// is then asked only for the value and the range: out of `u32` is `None`,
/// the same answer as a letter.
///
/// `0` itself is a number: a style index and a shared string index count
/// from 0. Whether zero is a legal value for a particular attribute belongs
/// to the caller, and `row_number` is the one that refuses it.
pub(crate) fn decimal(value: &str) -> Option<u32> {
    let digits = value.as_bytes();
    if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) {
        return None;
    }
    if digits.len() > 1 && digits[0] == b'0' {
        return None;
    }
    value.parse().ok()
}

/// Reads a `<c>` start tag's attributes.
///
/// `row` and `previous` are the open row's number and the previous cell's
/// column, which together give the reference of a `<c>` with no `r`.
///
/// `r`, `s` and `t` are read as the bytes they were written as, not
/// unescaped: each is a token the format spells one way, and a producer that
/// wrote `&#65;1` for `A1` is refused by the shape check rather than
/// silently accepted.
fn read_opening(
    start: &BytesStart<'_>,
    scope: &Scope,
    row: u32,
    previous: u32,
    index: usize,
    empty: bool,
) -> Result<Opening, Error> {
    let mut reference: Option<CellRef> = None;
    let mut style: Option<u32> = None;
    // `t` absent is `Number`, as the format says.
    let mut kind = Kind::Number;
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
            b"r" => {
                reference = Some(
                    CellRef::parse(attribute.value.as_bytes())
                        .ok_or(Error::Package("a cell reference is not of the form A1"))?,
                );
            }
            b"s" => {
                style = Some(
                    decimal(&attribute.value)
                        .ok_or(Error::Package("a cell's style index is not a number"))?,
                );
            }
            b"t" => kind = Kind::parse(attribute.value.as_bytes())?,
            _ => {}
        }
    }
    Ok(Opening {
        reference: reference.unwrap_or(CellRef {
            row,
            col: previous.saturating_add(1),
        }),
        style,
        kind,
        empty,
    })
}

/// The `r` attribute of a `<row>`, when it has one.
///
/// A row is numbered from 1, so `r="0"` is refused with the same message as
/// `r="one"`: both are spellings this layer has no row number for, and the
/// catalogue holds one message for the case (P2 ruling 2). Refusing it is
/// what keeps every reference the reader emits inside the range
/// `CellRef::parse` accepts, since a row with no `r` counts from 1.
fn row_number(start: &BytesStart<'_>, scope: &Scope, index: usize) -> Result<Option<u32>, Error> {
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        if namespace.is_none() && local == b"r" {
            let number = decimal(&attribute.value)
                .filter(|number| *number > 0)
                .ok_or(Error::Package("a row number is not a number"))?;
            return Ok(Some(number));
        }
    }
    Ok(None)
}

/// Reads an `<f>` start tag's attributes: its kind and its `ref`.
///
/// The attributes are collected before the kind is decided, because `master`
/// depends on whether `ref` is present and a producer may write the two in
/// either order.
///
/// An `si` that is not a decimal `u32` is refused with the same message as
/// an `si` that is absent: either way this layer has no index to carry, and
/// the catalogue holds one message for the case. `decimal` decides what a
/// decimal `u32` is, so a signed, zero-padded or out-of-range `si` is the
/// same case as `si="x"` (P2 ruling 4).
fn read_formula(
    start: &BytesStart<'_>,
    scope: &Scope,
    index: usize,
) -> Result<(FormulaKind, Option<String>), Error> {
    let mut named: Option<String> = None;
    let mut shared: Option<u32> = None;
    let mut reference: Option<String> = None;
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
            b"t" => named = Some(attribute.value.as_ref().to_owned()),
            b"si" => shared = decimal(&attribute.value),
            // Carried as written: this layer does not read ranges, and part
            // 4's rewrite writes back what it was given.
            b"ref" => reference = Some(attribute.value.as_ref().to_owned()),
            _ => {}
        }
    }
    let kind = match named.as_deref() {
        None | Some("normal") => FormulaKind::Normal,
        Some("shared") => {
            let Some(index) = shared else {
                return Err(Error::Package("a shared formula has no index"));
            };
            FormulaKind::Shared {
                index,
                master: reference.is_some(),
            }
        }
        Some("array") => FormulaKind::Array,
        Some("dataTable") => FormulaKind::DataTable,
        Some(_) => {
            return Err(Error::Package(
                "a formula has a type this layer does not know",
            ));
        }
    };
    Ok((kind, reference))
}

/// The text of a run container: an `<si>` of the shared strings table or an
/// `<is>` of an inline string cell.
///
/// Called with the container's start tag already given to `scope`, and
/// consumes through its matching end tag, which it gives back to `scope`.
/// Every `<t>` under it contributes its text, the `<r>` runs' included, and
/// an `<rPh>` subtree contributes nothing: the phonetic reading is a
/// separate string a producer shows beside the text, not part of it.
///
/// The two containers hold the same thing and take the same function, so the
/// `<rPh>` rule and the line-end rule are each written once
/// (`specs/0010-reading-cells.md`, P1 ruling 4 and "Module `cells`").
///
/// # Errors
///
/// Whatever [`text_of`] refuses for one of the `<t>` elements, and
/// [`Error::Xml`] when the part ends before the container's end tag.
fn text_under<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
) -> Result<String, Error> {
    let mut out = String::new();
    // Depth of the `<rPh>` subtree being skipped, 0 when not in one. A depth
    // rather than a flag: `<rPh>` holds elements of its own and the skip has
    // to end at the matching end tag, not at the first one.
    let mut phonetic = 0usize;
    // Depth inside the container, 0 at its own children's level, so the end
    // tag reached at 0 is the container's own.
    let mut depth = 0usize;
    loop {
        let Some(token) = stream.next()? else {
            return Err(Error::Xml(index));
        };
        // Set when the token opened a `<t>` whose text belongs here. The read
        // happens after the match, because `token` borrows the stream
        // `text_of` has to read from.
        let mut open_text = false;
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = MAIN.contains(&namespace.unwrap_or_default());
                if phonetic > 0 {
                    // Inside a skipped subtree nothing is read; the depth
                    // still tracks nesting so the skip ends in the right
                    // place.
                    if !empty {
                        phonetic += 1;
                    }
                } else if ours && local == b"rPh" {
                    if !empty {
                        phonetic = 1;
                    }
                } else if ours && local == b"t" {
                    // An empty `<t/>` contributes the empty string, which is
                    // to say nothing.
                    if !empty {
                        open_text = true;
                    }
                } else if !empty {
                    depth += 1;
                }
                if empty {
                    scope.end();
                }
            }
            Event::End(_) => {
                scope.end();
                if phonetic > 0 {
                    phonetic -= 1;
                } else if depth > 0 {
                    depth -= 1;
                } else {
                    return Ok(out);
                }
            }
            // Text and references outside a `<t>` are not the container's:
            // whitespace between runs, and a reference in a part of the tree
            // this layer does not read, which is therefore not expanded and
            // not refused either.
            _ => {}
        }
        if open_text {
            let text = text_of(stream, scope, index)?;
            out.push_str(&text);
        }
    }
}

/// Consumes an element's subtree, to and including its matching end tag.
///
/// Called with the start tag already given to `scope`, and gives it back.
/// Every start tag inside is given to `scope` too, so the scope's own depth
/// stays right and a namespace declared inside a skipped subtree is dropped
/// where it should be.
///
/// # Errors
///
/// [`Error::Xml`] when the part ends before the end tag, and whatever
/// [`Scope::start`] refuses.
fn skip_subtree<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
) -> Result<(), Error> {
    let mut depth = 0usize;
    while let Some(token) = stream.next()? {
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

/// The text of one simple-content element, to its matching end tag.
///
/// Called with the element's start tag already given to `scope`, and consumes
/// through the end tag, which it gives back to `scope`. It serves `<t>`,
/// `<v>` and `<f>` alike (`specs/0010-reading-cells.md`, P1 ruling 4), so
/// line ends, CDATA and the refusal of a nested element are decided in one
/// place for all three.
///
/// The first [`Event::End`] reached is this element's, because a nested start
/// is refused above it: there is no inner element whose end tag could come
/// first. That is a property of the refusal, not an assumption about where
/// the end tag sits.
///
/// # Errors
///
/// [`Error::Package`] "a text element holds an element" for an element inside
/// the text, and "a cell holds a reference this layer cannot expand" for a
/// reference [`expand`] does not know. [`Error::Xml`] when the part ends
/// before the end tag.
fn text_of<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
) -> Result<String, Error> {
    let mut out = String::new();
    // Set when the last character read was a literal CR, which was appended
    // as LF. A literal LF straight after it is the second half of a CR LF
    // pair and adds nothing. The state is kept ACROSS chunks, because a pair
    // can be split between two `Event::Text` runs.
    let mut after_cr = false;
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(_) | Event::Empty(_) => {
                return Err(Error::Package("a text element holds an element"));
            }
            Event::End(_) => {
                scope.end();
                return Ok(out);
            }
            Event::Text(ref chunk) => push_text(&mut out, chunk, &mut after_cr),
            // A CDATA section is characters written another way, so it is
            // this element's text and takes the same normalisation.
            Event::CData(ref chunk) => push_text(&mut out, chunk, &mut after_cr),
            Event::GeneralRef(ref reference) => {
                out.push(expand(reference, index)?);
                // A CR a reference stands for is kept and is not half of a
                // line end, so a literal LF after it is a line end of its
                // own.
                after_cr = false;
            }
            _ => {}
        }
    }
    Err(Error::Xml(index))
}

/// Appends a chunk with XML 1.0 section 2.11 line ends.
///
/// CR LF and a lone CR both become one LF. `after_cr` carries the "the last
/// character was a CR" state out of the call, so a pair split across two
/// chunks is still one LF.
fn push_text(into: &mut String, chunk: &str, after_cr: &mut bool) {
    for character in chunk.chars() {
        match character {
            '\r' => {
                into.push('\n');
                *after_cr = true;
            }
            '\n' => {
                if !*after_cr {
                    into.push('\n');
                }
                *after_cr = false;
            }
            _ => {
                into.push(character);
                *after_cr = false;
            }
        }
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
