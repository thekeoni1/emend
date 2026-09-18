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
//! nothing here reads a cell.
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
pub mod package;
#[cfg(test)]
mod testing;
pub mod xml;

pub use archive::{Archive, Edits, Entry, Error, Replacement, replacement_from_reader};
pub use cells::SharedStrings;
pub use xml::{BYTE_ORDER_MARK, Scope, Stream, Token};
