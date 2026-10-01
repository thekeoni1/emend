//! emend: read-modify-write of Office Open XML spreadsheets.
//!
//! Milestone 1 part 1 is the zip layer alone (`specs/0008-zip-layer.md`):
//! open an archive, index every entry with `u64` offsets, and write the
//! archive back with every untouched entry byte for byte in its original
//! place.
//!
//! Part 2 P1 adds the second layer's first module (`specs/0009-package-
//! layer.md`): [`xml`] streams a part's tokens, each with the exact input
//! bytes it covers, so a rewrite copies what it does not change. Still
//! nothing there reads a cell.
//!
//! Part 3 adds [`cells`] (`specs/0010-reading-cells.md`): the shared strings
//! table, and a streaming read of one worksheet's cells. The crate reads a
//! cell from here on; it still does not evaluate a formula, render, or
//! convert a date.
//!
//! Part 5 adds [`styles`] (`specs/0012-styles.md`): the styles part's number
//! formats and cell formats, read. Its Cargo feature `dates`, off by default,
//! adds the module `dates`: a civil date and time to and from the serial
//! number a cell holds, in the workbook's own epoch.
//!
//! The acceptance instrument is the fidelity harness, not this crate's own
//! reading of its output: a no-edit round trip is byte-identical to the
//! input, and `tools/xlsx-diff` reports every entry `identical` with an
//! empty flags column. The layer therefore shares no code with the harness,
//! so a parsing bug common to both could not hide itself.
//!
//! Every zip offset, entry size, byte count and file position is `u64`.
//! `usize` is 32 bits on `i686-unknown-linux-musl`, which is the target that
//! matters for the size budget, and a workbook may hold an entry larger than
//! that type would like.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod archive;
pub mod cells;
#[cfg(feature = "dates")]
pub mod dates;
pub mod package;
pub mod styles;
#[cfg(test)]
mod testing;
pub mod xml;

pub use archive::{Archive, Edits, Entry, Error, Replacement, replacement_from_reader};
pub use cells::{Area, Cell, CellRef, Cells, Formula, FormulaKind, Kind, SharedStrings, Value};
#[cfg(feature = "dates")]
pub use dates::{DateTime, Epoch};
#[cfg(feature = "images")]
pub use package::{Anchor, Image, ImageEntry};
#[cfg(feature = "rules")]
pub use package::{
    Cfvo, CfvoKind, ErrorStyle, IconSetKind, Operator, Rule, RuleEntry, Validation,
    ValidationEntry, ValidationKind,
};
#[cfg(feature = "comments")]
pub use package::{Comment, CommentEntry};
#[cfg(feature = "metadata")]
pub use package::{
    DefinedName, LinkTarget, Properties, Property, SheetProtection, legacy_password_hash,
};
#[cfg(feature = "tables")]
pub use package::{Table, TableEntry, TableStyle, TableStyleInfo};
pub use styles::{
    Alignment, Border, BorderStyle, Color, Fill, Font, Horizontal, NumFmt, NumberFormat, Pattern,
    Protection, Scheme, Side, Style, Styles, Underline, Vertical, VerticalAlign, Xf,
};
pub use xml::{BYTE_ORDER_MARK, Scope, Stream, Token};
