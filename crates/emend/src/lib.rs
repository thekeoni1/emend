//! emend: read-modify-write of Office Open XML spreadsheets.
//!
//! emend is a pure-Rust library for read-modify-write of Office Open XML
//! spreadsheets: open an existing .xlsx, change what you were asked to
//! change, hand the file back with everything else intact. It parses only
//! what an edit touches and passes every other zip entry through byte for
//! byte.
//!
//! The comparison point is openpyxl, and the definition is exact: like
//! openpyxl, emend does not evaluate formulas (it stores formula strings and
//! reads cached values), does not render, does not autofit, and does not fix
//! up references on row or column insertion. Those limits are by design and
//! are not gaps.
//!
//! The core (open, read, edit, write back) depends on `zip` and `quick-xml`
//! and nothing else. Everything heavier is a Cargo feature, off by default,
//! with no dependency of its own:
//!
//! - `dates`: a civil date and time to and from a cell's serial number.
//! - `rowcol`: row and column insertions, deletions, grouping and styles.
//! - `metadata`: defined names, hyperlinks, document properties, protection.
//! - `rules`: data validations and conditional formats.
//! - `tables`: tables added, listed and removed.
//! - `comments`: cell comments, their box size and their visibility.
//! - `images`: images, their anchors and their removal.
//! - `richtext`: rich text runs in a cell and, with `comments`, in a comment.
//! - `sheetprops`: the tab colour, the sheet view and the print settings.
//! - `read`: a cell's format and a sheet's formats and settings, read back.
//! - `copy`: a worksheet copied within its workbook.
//! - `charts`: charts read, added, formatted and removed, and chartsheets.
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
pub use cells::{Area, Cell, CellKind, CellRef, Cells, Formula, FormulaKind, SharedStrings, Value};
#[cfg(feature = "richtext")]
pub use cells::{RichText, Run};
#[cfg(feature = "dates")]
pub use dates::{DateTime, Epoch};
#[cfg(feature = "images")]
pub use package::{Anchor, EditAs, Image, ImageAnchor, ImageEntry};
#[cfg(feature = "rules")]
pub use package::{
    Cfvo, CfvoKind, ErrorStyle, IconSetKind, Operator, Rule, RuleEntry, Validation,
    ValidationEntry, ValidationKind,
};
#[cfg(feature = "comments")]
pub use package::{Comment, CommentBox, CommentEntry};
#[cfg(feature = "metadata")]
pub use package::{
    DefinedName, LinkTarget, Properties, Property, SheetProtection, legacy_password_hash,
};
#[cfg(feature = "sheetprops")]
pub use package::{
    HeaderFooter, Orientation, PageMargins, PageOrder, PageSetup, PrintOptions, SheetView, ViewKind,
};
#[cfg(feature = "tables")]
pub use package::{Table, TableEntry, TableStyle, TableStyleInfo};
pub use styles::{
    Alignment, Border, BorderStyle, Color, Fill, Font, Gradient, GradientKind, Horizontal, NumFmt,
    NumberFormat, Pattern, Protection, Scheme, Side, Stop, Style, Styles, Underline, Vertical,
    VerticalAlign, Xf,
};
pub use xml::{BYTE_ORDER_MARK, Scope, Stream, Token};
