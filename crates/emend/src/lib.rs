//! emend: read-modify-write of Office Open XML spreadsheets.
//!
//! emend edits existing Excel workbooks (.xlsx files) from Rust. Open a
//! workbook, change the cells, styles or sheets you need, and write it back.
//! Every zip entry the edit did not touch comes back byte for byte, so the
//! charts, pivot tables, macros and formatting you never looked at are still
//! there.
//!
//! emend has the scope of Python's openpyxl and the same limits: it does not
//! evaluate formulas, render, or autofit.
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
//!
//! # Example
//!
//! ```
//! use emend::{CellRef, Value, package::Package};
//! use std::io::{Read, Seek, Write};
//!
//! fn edit<R, W>(input: R, output: W) -> Result<(), Box<dyn std::error::Error>>
//! where
//!     R: Read + Seek + 'static,
//!     W: Write,
//! {
//!     let mut book = Package::open(input)?;
//!     let sheet = book.workbook()?.sheets.first().ok_or("no sheet")?.part.clone();
//!     let cell = CellRef::parse(b"B2").ok_or("not an A1 reference")?;
//!     book.set_cell(&sheet, cell, Value::Text("checked".into()));
//!     book.write_to(output)?;
//!     Ok(())
//! }
//! #
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #     let mut input = Vec::new();
//! #     Package::new()?.write_to(&mut input)?;
//! #     let mut output = Vec::new();
//! #     edit(std::io::Cursor::new(input), &mut output)?;
//! #     let mut book = Package::open(std::io::Cursor::new(output))?;
//! #     let sheet = book.workbook()?.sheets.first().ok_or("no sheet")?.part.clone();
//! #     let cell = book.cells(&sheet)?.next().ok_or("no cell")??;
//! #     assert_eq!(cell.reference, CellRef { row: 2, col: 2 });
//! #     assert_eq!(cell.value.as_deref(), Some("checked"));
//! #     Ok(())
//! # }
//! ```

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
