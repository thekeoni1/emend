//! A shared formula's text translated to another cell
//! (`specs/0048-the-two-refusals.md`, "The translator"). The Cargo feature
//! `rowcol`.
//!
//! [`translated_formula`] moves each relative A1 reference in a formula's
//! text by a row and a column offset and leaves every other byte as written,
//! which is what a shared formula's member means: the master's text read
//! from the member's cell, as Excel displays it. The scan knows the four
//! reference forms (a cell, an area, whole columns, whole rows) and the four
//! spans that hold none (a string literal, a quoted sheet name, a structured
//! reference's brackets, an error literal); it does not parse the formula,
//! and a token it cannot read as a reference is copied.

use super::UNSHARED_OFF_GRID;
use crate::archive::Error;

/// Excel's last column, `XFD`.
const LAST_COLUMN: i64 = 16384;
/// Excel's last row.
const LAST_ROW: i64 = 1_048_576;

/// `text` with each relative A1 reference moved `dr` rows and `dc` columns.
///
/// The output is the input with each moved reference rewritten in place, so
/// a text with no relative reference comes back unchanged. A `$` component
/// stays; letters match ignoring case, and a moved column is written in the
/// case the input wrote it.
///
/// # Errors
///
/// "an unshared formula would leave the grid" when a moved component falls
/// below 1, past column `XFD` or past row 1048576.
pub(crate) fn translated_formula(text: &str, dr: i64, dc: i64) -> Result<String, Error> {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    // The input up to `copied` is in `out`; every span this rewrites starts
    // and ends at an ASCII byte, so each slice is on a character boundary.
    let mut copied = 0;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => i = quoted_end(bytes, i, b'"'),
            // A quoted sheet name: the `!` after it is copied with it, and
            // the reference after that is read like any other.
            b'\'' => i = quoted_end(bytes, i, b'\''),
            b'[' => i = bracket_end(bytes, i),
            b'#' => i = error_end(bytes, i),
            _ => {
                if i > 0 && name_byte(bytes[i - 1], true) {
                    i += 1;
                    continue;
                }
                let Some(found) = reference_at(bytes, i) else {
                    i += 1;
                    continue;
                };
                if bytes.get(found.end).is_some_and(|&next| follows_name(next)) {
                    // A function, a name or a sheet name: copied whole, so
                    // neither half of `S1:S3!` is read on its own.
                    i = found.end;
                    continue;
                }
                let moved = found.moved(bytes, dr, dc)?;
                if moved.as_bytes() != &bytes[i..found.end] {
                    out.push_str(&text[copied..i]);
                    out.push_str(&moved);
                    copied = found.end;
                }
                i = found.end;
            }
        }
    }
    out.push_str(&text[copied..]);
    Ok(out)
}

/// Whether `byte` may continue a name, so a reference cannot start right
/// after it. A byte past ASCII belongs to a character a name may hold.
/// `dollar` counts `$` as such a byte, which it is before a token.
fn name_byte(byte: u8, dollar: bool) -> bool {
    byte.is_ascii_alphanumeric()
        || byte == b'_'
        || byte == b'.'
        || !byte.is_ascii()
        || dollar && byte == b'$'
}

/// Whether a token followed by `byte` is a function, a name or a sheet name
/// rather than a reference.
fn follows_name(byte: u8) -> bool {
    byte == b'(' || byte == b'!' || name_byte(byte, false)
}

/// The index past a span from the quote at `start` to its closing quote,
/// a doubled quote being one escaped; the end of the text when it has none.
fn quoted_end(bytes: &[u8], start: usize, quote: u8) -> usize {
    let mut i = start + 1;
    while i < bytes.len() {
        if bytes[i] == quote {
            if bytes.get(i + 1) == Some(&quote) {
                i += 2;
                continue;
            }
            return i + 1;
        }
        i += 1;
    }
    bytes.len()
}

/// The index past a structured reference's bracket text, nested brackets
/// counted; the end of the text when the brackets do not close.
fn bracket_end(bytes: &[u8], start: usize) -> usize {
    let mut depth = 0usize;
    for (i, &byte) in bytes.iter().enumerate().skip(start) {
        if byte == b'[' {
            depth += 1;
        } else if byte == b']' {
            depth -= 1;
            if depth == 0 {
                return i + 1;
            }
        }
    }
    bytes.len()
}

/// The index past an error literal: `#` and its letters, digits, `/` and
/// `_`, with the `!` or `?` that ends it (`#REF!`, `#DIV/0!`, `#NAME?`), or
/// without one (`#N/A`).
fn error_end(bytes: &[u8], start: usize) -> usize {
    let mut i = start + 1;
    while i < bytes.len()
        && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'/' || bytes[i] == b'_')
    {
        i += 1;
    }
    if i < bytes.len() && (bytes[i] == b'!' || bytes[i] == b'?') {
        i += 1;
    }
    i
}

/// One part of a reference: a column or a row, where it is written and
/// whether a `$` holds it.
#[derive(Clone, Copy)]
struct Component {
    /// The byte range of the letters or digits, the `$` excluded.
    from: usize,
    to: usize,
    value: i64,
    absolute: bool,
}

/// A reference the scan found: where it starts and ends, and its
/// components in order, each with whether it is a column.
struct Found {
    start: usize,
    parts: Vec<(Component, bool)>,
    end: usize,
}

impl Found {
    /// The reference's text with each relative component moved.
    fn moved(&self, bytes: &[u8], dr: i64, dc: i64) -> Result<String, Error> {
        let mut out = String::new();
        // Each gap between components (a `$`, the `:`) is copied as written.
        let mut at = self.start;
        for (part, column) in &self.parts {
            out.push_str(&String::from_utf8_lossy(&bytes[at..part.from]));
            let written = &bytes[part.from..part.to];
            if part.absolute || (*column && dc == 0) || (!*column && dr == 0) {
                out.push_str(&String::from_utf8_lossy(written));
            } else if *column {
                let to = part.value + dc;
                if !(1..=LAST_COLUMN).contains(&to) {
                    return Err(UNSHARED_OFF_GRID);
                }
                let letters = column_letters(to);
                if written.iter().all(u8::is_ascii_lowercase) {
                    out.push_str(&letters.to_ascii_lowercase());
                } else {
                    out.push_str(&letters);
                }
            } else {
                let to = part.value + dr;
                if !(1..=LAST_ROW).contains(&to) {
                    return Err(UNSHARED_OFF_GRID);
                }
                out.push_str(&to.to_string());
            }
            at = part.to;
        }
        out.push_str(&String::from_utf8_lossy(&bytes[at..self.end]));
        Ok(out)
    }
}

/// A column number's letters.
fn column_letters(mut column: i64) -> String {
    let mut letters = Vec::new();
    while column > 0 {
        letters.push(b'A' + ((column - 1) % 26) as u8);
        column = (column - 1) / 26;
    }
    letters.reverse();
    String::from_utf8_lossy(&letters).into_owned()
}

/// An optional `$` and one to three letters naming a column up to `XFD`, at
/// `i`: the component and the index past it.
fn column_at(bytes: &[u8], i: usize) -> Option<(Component, usize)> {
    let absolute = bytes.get(i) == Some(&b'$');
    let from = i + usize::from(absolute);
    let count = bytes[from.min(bytes.len())..]
        .iter()
        .take_while(|byte| byte.is_ascii_alphabetic())
        .count();
    if count == 0 || count > 3 {
        return None;
    }
    let to = from + count;
    let value = bytes[from..to].iter().fold(0i64, |sum, byte| {
        sum * 26 + i64::from(byte.to_ascii_uppercase() - b'A') + 1
    });
    (value <= LAST_COLUMN).then_some((
        Component {
            from,
            to,
            value,
            absolute,
        },
        to,
    ))
}

/// An optional `$` and a row number from 1 to 1048576 with no leading zero,
/// at `i`: the component and the index past it.
fn row_at(bytes: &[u8], i: usize) -> Option<(Component, usize)> {
    let absolute = bytes.get(i) == Some(&b'$');
    let from = i + usize::from(absolute);
    let count = bytes[from.min(bytes.len())..]
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    if count == 0 || count > 7 || bytes[from] == b'0' {
        return None;
    }
    let to = from + count;
    let value = bytes[from..to]
        .iter()
        .fold(0i64, |sum, byte| sum * 10 + i64::from(byte - b'0'));
    (value <= LAST_ROW).then_some((
        Component {
            from,
            to,
            value,
            absolute,
        },
        to,
    ))
}

/// A cell at `i`: its column, its row and the index past it.
fn cell_at(bytes: &[u8], i: usize) -> Option<(Component, Component, usize)> {
    let (column, after) = column_at(bytes, i)?;
    let (row, end) = row_at(bytes, after)?;
    Some((column, row, end))
}

/// The longest reference at `i`: an area, whole columns, whole rows or a
/// cell, in that order.
fn reference_at(bytes: &[u8], i: usize) -> Option<Found> {
    if let Some((c1, r1, end)) = cell_at(bytes, i) {
        if bytes.get(end) == Some(&b':')
            && let Some((c2, r2, end)) = cell_at(bytes, end + 1)
        {
            return Some(Found {
                start: i,
                parts: vec![(c1, true), (r1, false), (c2, true), (r2, false)],
                end,
            });
        }
        return Some(Found {
            start: i,
            parts: vec![(c1, true), (r1, false)],
            end,
        });
    }
    if let Some((c1, end)) = column_at(bytes, i)
        && bytes.get(end) == Some(&b':')
        && let Some((c2, end)) = column_at(bytes, end + 1)
        && !bytes.get(end).is_some_and(u8::is_ascii_digit)
    {
        return Some(Found {
            start: i,
            parts: vec![(c1, true), (c2, true)],
            end,
        });
    }
    if let Some((r1, end)) = row_at(bytes, i)
        && bytes.get(end) == Some(&b':')
        && let Some((r2, end)) = row_at(bytes, end + 1)
    {
        return Some(Found {
            start: i,
            parts: vec![(r1, false), (r2, false)],
            end,
        });
    }
    None
}
