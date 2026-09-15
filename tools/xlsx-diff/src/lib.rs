//! xlsx-diff: the entry-level fidelity harness (`specs/0004-xlsx-diff.md`).
//!
//! Compares two zip archives entry by entry and prints tab-separated rows on
//! stdout. It never prints entry content: rows carry names, indices,
//! compression methods, sizes and CRC-32 values, and an error line names the
//! side and at most an escaped entry name.
//!
//! T1 P1 fills columns 1 to 13 of the `entry` rows. For an entry present on
//! both sides the status is `identical` or the provisional `differs`, which
//! P2 replaces with `recompressed` and `changed`.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

use zip::ZipArchive;
use zip::result::ZipError;

/// The usage line printed on a usage error.
pub const USAGE: &str = "usage: xlsx-diff <input.xlsx> <output.xlsx>";

/// Exit status when every entry is identical.
pub const EXIT_SAME: u8 = 0;

/// Exit status when the archives were compared and a difference exists.
pub const EXIT_DIFFERENT: u8 = 1;

/// Exit status for usage errors and every other failure.
pub const EXIT_ERROR: u8 = 2;

/// Size of each of the two buffers used to compare entry bytes side by side.
const CHUNK: usize = 64 * 1024;

/// Signature of a central directory file header.
const CENTRAL_HEADER_SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x01, 0x02];

/// Length of the fixed part of a central directory file header.
const CENTRAL_HEADER_LEN: usize = 46;

/// Which of the two archives a message or a value belongs to.
#[derive(Clone, Copy)]
enum Side {
    Input,
    Output,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Side::Input => "input",
            Side::Output => "output",
        })
    }
}

/// Everything that stops a comparison. The messages name the side and at
/// most an escaped entry name. They never carry entry content, file paths or
/// text from the zip crate.
enum Error {
    Open(Side, io::ErrorKind),
    NotZip(Side),
    Unsupported(Side),
    CentralDirectory(Side),
    Duplicate(Side, Vec<u8>),
    Entry(Side, Vec<u8>),
    Write(io::ErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Open(side, kind) => write!(f, "cannot read the {side} file: {kind}"),
            Error::NotZip(side) => write!(f, "the {side} file is not a readable zip archive"),
            Error::Unsupported(side) => {
                write!(f, "the {side} archive uses an unsupported zip feature")
            }
            Error::CentralDirectory(side) => {
                write!(f, "cannot read the central directory of the {side} archive")
            }
            Error::Duplicate(side, name) => write!(
                f,
                "duplicate entry name in the {side} archive: {}",
                escape_name(name)
            ),
            Error::Entry(side, name) => write!(
                f,
                "cannot read entry {} in the {side} archive",
                escape_name(name)
            ),
            Error::Write(kind) => write!(f, "cannot write the rows: {kind}"),
        }
    }
}

/// Maps an error from the zip crate to a message that carries no crate text.
fn zip_error(side: Side, err: ZipError) -> Error {
    match err {
        ZipError::Io(e) => Error::Open(side, e.kind()),
        ZipError::UnsupportedArchive(_) | ZipError::CompressionMethodNotSupported(_) => {
            Error::Unsupported(side)
        }
        _ => Error::NotZip(side),
    }
}

/// Escapes a raw entry name for printing: bytes 0x20 to 0x7E other than `\`
/// as themselves, `\` as `\\`, and every other byte as `\xNN` with two
/// lowercase hex digits.
pub fn escape_name(raw: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(raw.len());
    for &byte in raw {
        match byte {
            b'\\' => out.push_str("\\\\"),
            0x20..=0x7e => out.push(char::from(byte)),
            _ => {
                out.push_str("\\x");
                out.push(char::from(HEX[usize::from(byte >> 4)]));
                out.push(char::from(HEX[usize::from(byte & 0x0f)]));
            }
        }
    }
    out
}

/// Metadata of one entry, as the central directory and the zip crate give it.
struct Meta {
    name: Vec<u8>,
    method: u16,
    size: u64,
    compressed: u64,
    crc: u32,
}

/// An opened archive and the metadata of its entries, by index.
struct Archive {
    side: Side,
    zip: ZipArchive<BufReader<File>>,
    entries: Vec<Meta>,
}

/// Opens one archive, reads its central directory names, rejects a duplicate
/// raw name, and collects every entry's metadata.
fn open(path: &Path, side: Side) -> Result<Archive, Error> {
    let file = File::open(path).map_err(|e| Error::Open(side, e.kind()))?;
    let mut zip = ZipArchive::new(BufReader::new(file)).map_err(|e| zip_error(side, e))?;

    // The zip crate keys its entries by name, so a repeated name collapses
    // into one entry. The central directory is read directly to find it.
    let mut raw = BufReader::new(File::open(path).map_err(|e| Error::Open(side, e.kind()))?);
    let records = central_names(&mut raw, zip.central_directory_start(), zip.len(), side)?;
    let mut seen = HashSet::with_capacity(records.len());
    for name in &records {
        if !seen.insert(name.as_slice()) {
            return Err(Error::Duplicate(side, name.clone()));
        }
    }
    if records.len() != zip.len() {
        return Err(Error::CentralDirectory(side));
    }

    let mut entries = Vec::with_capacity(records.len());
    for (index, record) in records.into_iter().enumerate() {
        let entry = zip
            .by_index_raw(index)
            .map_err(|_| Error::Entry(side, record.clone()))?;
        if entry.name_raw() != record.as_slice() {
            return Err(Error::CentralDirectory(side));
        }
        // zip 8.6.0 gives the method number only through this deprecated call.
        #[allow(deprecated)]
        let method = entry.compression().to_u16();
        entries.push(Meta {
            method,
            size: entry.size(),
            compressed: entry.compressed_size(),
            crc: entry.crc32(),
            name: record,
        });
    }
    Ok(Archive { side, zip, entries })
}

/// Reads the raw names of the central directory file headers, in order, from
/// `start` (an absolute file offset, as `ZipArchive::central_directory_start`
/// gives it, prefix included) until the first record that is not a central
/// file header. `expected` is the entry count the zip crate reports.
fn central_names(
    reader: &mut BufReader<File>,
    start: u64,
    expected: usize,
    side: Side,
) -> Result<Vec<Vec<u8>>, Error> {
    let bad = |_| Error::CentralDirectory(side);
    let mut position = start;
    let mut names = Vec::with_capacity(expected);
    loop {
        reader.seek(SeekFrom::Start(position)).map_err(bad)?;
        let mut header = [0u8; CENTRAL_HEADER_LEN];
        let got = read_full(reader, &mut header).map_err(bad)?;
        if got < 4 || header[..4] != CENTRAL_HEADER_SIGNATURE {
            break;
        }
        if got < CENTRAL_HEADER_LEN {
            return Err(Error::CentralDirectory(side));
        }
        let name_len = u16::from_le_bytes([header[28], header[29]]);
        let extra_len = u16::from_le_bytes([header[30], header[31]]);
        let comment_len = u16::from_le_bytes([header[32], header[33]]);
        let mut name = vec![0u8; usize::from(name_len)];
        if read_full(reader, &mut name).map_err(bad)? != name.len() {
            return Err(Error::CentralDirectory(side));
        }
        names.push(name);
        position = position
            .checked_add(46 + u64::from(name_len) + u64::from(extra_len) + u64::from(comment_len))
            .ok_or(Error::CentralDirectory(side))?;
    }
    Ok(names)
}

/// Reads until `buf` is full or the reader ends, and returns the count read.
fn read_full<R: Read + ?Sized>(reader: &mut R, buf: &mut [u8]) -> io::Result<usize> {
    let mut filled = 0;
    while filled < buf.len() {
        match reader.read(&mut buf[filled..]) {
            Ok(0) => break,
            Ok(n) => filled += n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(filled)
}

/// Compares the raw, still compressed, bytes of two entries through two
/// 64 KiB buffers, side by side. A stream that ends before its declared
/// compressed size is a read error, never a difference.
fn raw_equal(input: &mut Archive, i: usize, output: &mut Archive, j: usize) -> Result<bool, Error> {
    let (side_in, side_out) = (input.side, output.side);
    let (name_in, declared_in) = (input.entries[i].name.clone(), input.entries[i].compressed);
    let (name_out, declared_out) = (output.entries[j].name.clone(), output.entries[j].compressed);
    let mut a = input
        .zip
        .by_index_raw(i)
        .map_err(|_| Error::Entry(side_in, name_in.clone()))?;
    let mut b = output
        .zip
        .by_index_raw(j)
        .map_err(|_| Error::Entry(side_out, name_out.clone()))?;
    let mut buf_a = vec![0u8; CHUNK];
    let mut buf_b = vec![0u8; CHUNK];
    let (mut total_a, mut total_b) = (0u64, 0u64);
    loop {
        let got_a =
            read_full(&mut a, &mut buf_a).map_err(|_| Error::Entry(side_in, name_in.clone()))?;
        let got_b =
            read_full(&mut b, &mut buf_b).map_err(|_| Error::Entry(side_out, name_out.clone()))?;
        total_a += got_a as u64;
        total_b += got_b as u64;
        if got_a < CHUNK && total_a != declared_in {
            return Err(Error::Entry(side_in, name_in));
        }
        if got_b < CHUNK && total_b != declared_out {
            return Err(Error::Entry(side_out, name_out));
        }
        if got_a != got_b || buf_a[..got_a] != buf_b[..got_b] {
            return Ok(false);
        }
        if got_a < CHUNK {
            return Ok(true);
        }
    }
}

/// The columns of one side of an `entry` row: index, method, uncompressed
/// size, compressed size and CRC-32, or five empty columns.
fn side_columns(side: Option<(usize, &Meta)>) -> [String; 5] {
    match side {
        Some((index, meta)) => [
            index.to_string(),
            meta.method.to_string(),
            meta.size.to_string(),
            meta.compressed.to_string(),
            format!("{:08x}", meta.crc),
        ],
        None => Default::default(),
    }
}

/// Writes one `entry` row: columns 1 to 13 filled, 14 to 17 empty.
fn write_entry(
    out: &mut dyn Write,
    name: &[u8],
    status: &str,
    input: Option<(usize, &Meta)>,
    output: Option<(usize, &Meta)>,
) -> Result<(), Error> {
    let [ia, ma, ua, ca, ka] = side_columns(input);
    let [ib, mb, ub, cb, kb] = side_columns(output);
    writeln!(
        out,
        "entry\t{}\t{status}\t{ia}\t{ib}\t{ma}\t{mb}\t{ua}\t{ub}\t{ca}\t{cb}\t{ka}\t{kb}\t\t\t\t",
        escape_name(name)
    )
    .map_err(|e| Error::Write(e.kind()))
}

/// Writes one `archive` row.
fn write_archive(out: &mut dyn Write, key: &str, value: u64) -> Result<(), Error> {
    writeln!(out, "archive\t{key}\t{value}").map_err(|e| Error::Write(e.kind()))
}

/// Compares the two archives and writes every row. Returns whether every
/// entry is identical.
fn compare(input: &Path, output: &Path, out: &mut dyn Write) -> Result<bool, Error> {
    let mut a = open(input, Side::Input)?;
    let mut b = open(output, Side::Output)?;
    let out_index: HashMap<Vec<u8>, usize> = b
        .entries
        .iter()
        .enumerate()
        .map(|(j, meta)| (meta.name.clone(), j))
        .collect();

    let (mut identical, mut differs, mut dropped, mut added) = (0u64, 0u64, 0u64, 0u64);
    let mut matched = vec![false; b.entries.len()];
    for i in 0..a.entries.len() {
        match out_index.get(&a.entries[i].name).copied() {
            Some(j) => {
                matched[j] = true;
                let (ea, eb) = (&a.entries[i], &b.entries[j]);
                let same_meta = ea.method == eb.method
                    && ea.crc == eb.crc
                    && ea.size == eb.size
                    && ea.compressed == eb.compressed;
                let same = same_meta && raw_equal(&mut a, i, &mut b, j)?;
                let status = if same {
                    identical += 1;
                    "identical"
                } else {
                    differs += 1;
                    "differs"
                };
                let (ea, eb) = (&a.entries[i], &b.entries[j]);
                write_entry(out, &ea.name, status, Some((i, ea)), Some((j, eb)))?;
            }
            None => {
                dropped += 1;
                let ea = &a.entries[i];
                write_entry(out, &ea.name, "dropped", Some((i, ea)), None)?;
            }
        }
    }
    for (j, eb) in b.entries.iter().enumerate() {
        if !matched[j] {
            added += 1;
            write_entry(out, &eb.name, "added", None, Some((j, eb)))?;
        }
    }

    write_archive(out, "entries_in", a.entries.len() as u64)?;
    write_archive(out, "entries_out", b.entries.len() as u64)?;
    write_archive(out, "identical", identical)?;
    write_archive(out, "differs", differs)?;
    write_archive(out, "dropped", dropped)?;
    write_archive(out, "added", added)?;
    Ok(differs == 0 && dropped == 0 && added == 0)
}

/// Compares `input` with `output`, writing the rows to `stdout` and at most
/// one `error: ` line to `stderr`. Returns the exit status: [`EXIT_SAME`],
/// [`EXIT_DIFFERENT`] or [`EXIT_ERROR`]. Rows written before an error stay
/// written; a consumer trusts them only for exit status 0 or 1.
pub fn run(input: &Path, output: &Path, stdout: &mut dyn Write, stderr: &mut dyn Write) -> u8 {
    match compare(input, output, stdout) {
        Ok(true) => EXIT_SAME,
        Ok(false) => EXIT_DIFFERENT,
        Err(err) => {
            let _ = writeln!(stderr, "error: {err}");
            EXIT_ERROR
        }
    }
}
