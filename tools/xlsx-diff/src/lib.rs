//! xlsx-diff: the entry-level fidelity harness (`specs/0004-xlsx-diff.md`).
//!
//! Compares two zip archives entry by entry and prints tab-separated rows on
//! stdout. It never prints entry content: rows carry names, indices,
//! compression methods, sizes, CRC-32 values, offsets and flag names, and an
//! error line names the side and at most an escaped entry name.
//!
//! T1 P3 fills every column of the `entry` rows, including the flags, writes
//! every `archive` row, and sets the exit status as the spec's "Exit status"
//! section says. T1 P4 adds the seventh flag, `local`, and version needed to
//! extract to `attrs`. T1 P5 narrows `local` to local-only differences, adds
//! internal attributes and disk number start to `attrs`, and adds the bytes
//! after each entry's data to `descriptor`.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

use zip::result::ZipError;
use zip::{HasZipMetadata, ZipArchive};

/// The usage line printed on a usage error.
pub const USAGE: &str = "usage: xlsx-diff <input.xlsx> <output.xlsx>";

/// Exit status when every entry is identical with no flags, the order is
/// preserved and the archive comments are equal.
pub const EXIT_SAME: u8 = 0;

/// Exit status when the archives were compared and a difference exists.
pub const EXIT_DIFFERENT: u8 = 1;

/// Exit status for usage errors and every other failure.
pub const EXIT_ERROR: u8 = 2;

/// Size of each of the two buffers used to compare bytes side by side.
const CHUNK: usize = 64 * 1024;

/// Signature of a central directory file header.
const CENTRAL_HEADER_SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x01, 0x02];

/// Length of the fixed part of a central directory file header.
const CENTRAL_HEADER_LEN: usize = 46;

/// Signature of a local file header.
const LOCAL_HEADER_SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x03, 0x04];

/// Length of the fixed part of a local file header.
const LOCAL_HEADER_LEN: usize = 30;

/// General purpose bit 3: sizes and CRC-32 follow the data in a descriptor.
const DATA_DESCRIPTOR_FLAG: u16 = 0x0008;

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
    Short(Side, Vec<u8>),
    Encrypted(Side, Vec<u8>),
    Method(Side, Vec<u8>, u16),
    Data(Side, Vec<u8>),
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
            Error::Short(side, name) => write!(
                f,
                "entry {} in the {side} archive ends before its declared compressed size",
                escape_name(name)
            ),
            Error::Encrypted(side, name) => write!(
                f,
                "entry {} in the {side} archive is encrypted",
                escape_name(name)
            ),
            Error::Method(side, name, method) => write!(
                f,
                "entry {} in the {side} archive uses unsupported compression method {method}",
                escape_name(name)
            ),
            Error::Data(side, name) => write!(
                f,
                "entry {} in the {side} archive failed to decompress or its CRC-32 does not match",
                escape_name(name)
            ),
            Error::Write(kind) => write!(f, "cannot write the rows: {kind}"),
        }
    }
}

/// Maps an error from the zip crate while opening an archive to a message
/// that carries no crate text.
fn zip_error(side: Side, err: ZipError) -> Error {
    match err {
        ZipError::Io(e) => Error::Open(side, e.kind()),
        ZipError::UnsupportedArchive(_) | ZipError::CompressionMethodNotSupported(_) => {
            Error::Unsupported(side)
        }
        _ => Error::NotZip(side),
    }
}

/// Maps an error from the zip crate while opening an entry for decompression.
fn entry_error(side: Side, name: Vec<u8>, err: ZipError) -> Error {
    match err {
        ZipError::UnsupportedArchive(message) if message == ZipError::PASSWORD_REQUIRED => {
            Error::Encrypted(side, name)
        }
        ZipError::CompressionMethodNotSupported(method) => Error::Method(side, name, method),
        _ => Error::Entry(side, name),
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

/// The fields of one central directory file header that the flags compare,
/// taken from the record's own bytes: the zip crate decodes the comment,
/// turns an invalid DOS date into no date, and does not keep the central
/// extra field apart.
struct Record {
    name: Vec<u8>,
    version_made_by: u16,
    version_needed: u16,
    flags: u16,
    time: u16,
    date: u16,
    external_attributes: u32,
    internal_attributes: u16,
    disk_start: u16,
    /// Bytes 6 to 27: version needed, general purpose flags, method, DOS
    /// time, DOS date, CRC-32, compressed size and uncompressed size, in the
    /// same layout as the local header's bytes 4 to 25.
    central_fixed: [u8; 22],
    extra: Vec<u8>,
    comment: Vec<u8>,
}

/// The parts of an entry's local file header that the flags compare.
struct Local {
    /// Bytes 4 to 25: version needed, general purpose flags, method, DOS
    /// time, DOS date, CRC-32, compressed size and uncompressed size.
    fixed: [u8; 22],
    name: Vec<u8>,
    extra: Vec<u8>,
    /// Absolute file offset of the entry's compressed data.
    data_start: u64,
}

/// Metadata of one entry: the central directory record, the zip crate's
/// method, sizes and CRC-32, and the local file header.
struct Meta {
    record: Record,
    method: u16,
    size: u64,
    compressed: u64,
    crc: u32,
    local: Local,
    /// Absolute file offset of the local file header.
    header_start: u64,
    /// The bytes between the end of the entry's compressed data and the next
    /// local header, or the central directory for the last entry, as a
    /// half-open range of absolute file offsets.
    gap: (u64, u64),
}

impl Meta {
    fn name(&self) -> &[u8] {
        &self.record.name
    }
}

/// An opened archive, its archive-level values, and the metadata of its
/// entries, by index.
struct Archive {
    side: Side,
    zip: ZipArchive<BufReader<File>>,
    entries: Vec<Meta>,
    comment: Vec<u8>,
    prefix: u64,
    central_start: u64,
    bytes: u64,
}

/// Opens one archive, reads its central directory records, rejects a
/// duplicate raw name and an encrypted entry, and collects every entry's
/// metadata and local file header.
fn open(path: &Path, side: Side) -> Result<Archive, Error> {
    let file = File::open(path).map_err(|e| Error::Open(side, e.kind()))?;
    let bytes = file
        .metadata()
        .map_err(|e| Error::Open(side, e.kind()))?
        .len();
    let mut zip = ZipArchive::new(BufReader::new(file)).map_err(|e| zip_error(side, e))?;

    // The zip crate keys its entries by name, so a repeated name collapses
    // into one entry. The central directory is read directly to find it.
    let mut raw = BufReader::new(File::open(path).map_err(|e| Error::Open(side, e.kind()))?);
    let central_start = zip.central_directory_start();
    let records = central_records(&mut raw, central_start, zip.len(), side)?;
    let mut seen = HashSet::with_capacity(records.len());
    for record in &records {
        if !seen.insert(record.name.as_slice()) {
            return Err(Error::Duplicate(side, record.name.clone()));
        }
    }
    if records.len() != zip.len() {
        return Err(Error::CentralDirectory(side));
    }

    let mut entries = Vec::with_capacity(records.len());
    for (index, record) in records.into_iter().enumerate() {
        let entry = zip
            .by_index_raw(index)
            .map_err(|_| Error::Entry(side, record.name.clone()))?;
        let metadata = entry.get_metadata();
        // The record read here and the crate's entry at the same index must
        // be the same central directory header.
        if entry.name_raw() != record.name.as_slice()
            || metadata.flags != record.flags
            || metadata.external_attributes != record.external_attributes
        {
            return Err(Error::CentralDirectory(side));
        }
        if metadata.encrypted {
            return Err(Error::Encrypted(side, record.name));
        }
        // zip 8.6.0 gives the method number only through this deprecated call.
        #[allow(deprecated)]
        let method = entry.compression().to_u16();
        let (size, compressed, crc) = (entry.size(), entry.compressed_size(), entry.crc32());
        let header_start = entry.header_start();
        let local = local_header(&mut raw, header_start, side, &record.name)?;
        entries.push(Meta {
            record,
            method,
            size,
            compressed,
            crc,
            local,
            header_start,
            gap: (0, 0),
        });
    }

    // Each entry's data ends at its data start plus its compressed size. The
    // gap runs from there to the first local header at or after that point,
    // or to the central directory if none is. Data that would run past the
    // central directory leaves an empty gap: reading the entry's data then
    // reports the specific error (a short raw stream or a failed
    // decompression). Data that runs over a later local header is itself
    // compared, and its gap starts where it ends.
    let mut starts: Vec<u64> = entries.iter().map(|m| m.header_start).collect();
    starts.sort_unstable();
    for meta in &mut entries {
        let Some(end) = meta.local.data_start.checked_add(meta.compressed) else {
            return Err(Error::Entry(side, meta.record.name.clone()));
        };
        let at = starts.partition_point(|&s| s < end);
        let next = starts
            .get(at)
            .copied()
            .unwrap_or(central_start)
            .min(central_start);
        meta.gap = (end.min(next), next);
    }
    Ok(Archive {
        side,
        comment: zip.comment().to_vec(),
        prefix: zip.offset(),
        central_start,
        bytes,
        zip,
        entries,
    })
}

/// Reads the central directory file headers, in order, from `start` (an
/// absolute file offset, as `ZipArchive::central_directory_start` gives it,
/// prefix included) until the first record that is not a central file
/// header. `expected` is the entry count the zip crate reports.
fn central_records(
    reader: &mut BufReader<File>,
    start: u64,
    expected: usize,
    side: Side,
) -> Result<Vec<Record>, Error> {
    let bad = |_| Error::CentralDirectory(side);
    let le16 = |b: &[u8], at: usize| u16::from_le_bytes([b[at], b[at + 1]]);
    reader.seek(SeekFrom::Start(start)).map_err(bad)?;
    let mut records = Vec::with_capacity(expected);
    loop {
        let mut header = [0u8; CENTRAL_HEADER_LEN];
        let got = read_full(reader, &mut header).map_err(bad)?;
        if got < 4 || header[..4] != CENTRAL_HEADER_SIGNATURE {
            break;
        }
        if got < CENTRAL_HEADER_LEN {
            return Err(Error::CentralDirectory(side));
        }
        let mut field = |len: u16| -> Result<Vec<u8>, Error> {
            let mut bytes = vec![0u8; usize::from(len)];
            if read_full(reader, &mut bytes).map_err(bad)? != bytes.len() {
                return Err(Error::CentralDirectory(side));
            }
            Ok(bytes)
        };
        let name = field(le16(&header, 28))?;
        let extra = field(le16(&header, 30))?;
        let comment = field(le16(&header, 32))?;
        let mut central_fixed = [0u8; 22];
        central_fixed.copy_from_slice(&header[6..28]);
        records.push(Record {
            name,
            version_made_by: le16(&header, 4),
            version_needed: le16(&header, 6),
            flags: le16(&header, 8),
            time: le16(&header, 12),
            date: le16(&header, 14),
            external_attributes: u32::from_le_bytes([
                header[38], header[39], header[40], header[41],
            ]),
            internal_attributes: le16(&header, 36),
            disk_start: le16(&header, 34),
            central_fixed,
            extra,
            comment,
        });
    }
    Ok(records)
}

/// Reads the local file header at `start` (an absolute file offset, as
/// `ZipFile::header_start` gives it): its fixed fields after the signature,
/// its name and its extra field.
fn local_header(
    reader: &mut BufReader<File>,
    start: u64,
    side: Side,
    name: &[u8],
) -> Result<Local, Error> {
    let bad = || Error::Entry(side, name.to_vec());
    reader.seek(SeekFrom::Start(start)).map_err(|_| bad())?;
    let mut header = [0u8; LOCAL_HEADER_LEN];
    let got = read_full(reader, &mut header).map_err(|_| bad())?;
    if got != LOCAL_HEADER_LEN || header[..4] != LOCAL_HEADER_SIGNATURE {
        return Err(bad());
    }
    let mut fixed = [0u8; 22];
    fixed.copy_from_slice(&header[4..26]);
    let mut field = |len: u16| -> Result<Vec<u8>, Error> {
        let mut bytes = vec![0u8; usize::from(len)];
        if read_full(reader, &mut bytes).map_err(|_| bad())? != bytes.len() {
            return Err(bad());
        }
        Ok(bytes)
    };
    let local_name = field(u16::from_le_bytes([header[26], header[27]]))?;
    let extra = field(u16::from_le_bytes([header[28], header[29]]))?;
    let data_start = start
        .checked_add(LOCAL_HEADER_LEN as u64 + local_name.len() as u64 + extra.len() as u64)
        .ok_or_else(bad)?;
    Ok(Local {
        fixed,
        name: local_name,
        extra,
        data_start,
    })
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
    let (name_in, declared_in) = (
        input.entries[i].name().to_vec(),
        input.entries[i].compressed,
    );
    let (name_out, declared_out) = (
        output.entries[j].name().to_vec(),
        output.entries[j].compressed,
    );
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
            return Err(Error::Short(side_in, name_in));
        }
        if got_b < CHUNK && total_b != declared_out {
            return Err(Error::Short(side_out, name_out));
        }
        if got_a != got_b || buf_a[..got_a] != buf_b[..got_b] {
            return Ok(false);
        }
        if got_a < CHUNK {
            return Ok(true);
        }
    }
}

/// How the uncompressed bytes of two entries compare.
enum Content {
    /// The same bytes.
    Equal,
    /// Equal lengths; the first differing offset, the count of differing
    /// bytes and the last differing offset.
    Differs { first: u64, count: u64, last: u64 },
    /// Different lengths; the first differing offset, which is the shorter
    /// length when the bytes agree up to there.
    Length(u64),
}

/// Decompresses both entries through two 64 KiB buffers, side by side, in
/// one pass. The longer stream is read to its end, so every CRC-32 is
/// checked. A decompression or CRC-32 error, or a stream whose length is not
/// its declared uncompressed size, is an error.
fn content(
    input: &mut Archive,
    i: usize,
    output: &mut Archive,
    j: usize,
) -> Result<Content, Error> {
    let (side_in, side_out) = (input.side, output.side);
    let (name_in, size_in) = (input.entries[i].name().to_vec(), input.entries[i].size);
    let (name_out, size_out) = (output.entries[j].name().to_vec(), output.entries[j].size);
    let mut a = input
        .zip
        .by_index(i)
        .map_err(|e| entry_error(side_in, name_in.clone(), e))?;
    let mut b = output
        .zip
        .by_index(j)
        .map_err(|e| entry_error(side_out, name_out.clone(), e))?;
    let mut buf_a = vec![0u8; CHUNK];
    let mut buf_b = vec![0u8; CHUNK];
    let (mut total_a, mut total_b) = (0u64, 0u64);
    let (mut done_a, mut done_b) = (false, false);
    let mut first = None;
    let (mut count, mut last) = (0u64, 0u64);
    while !(done_a && done_b) {
        let got_a = if done_a {
            0
        } else {
            read_full(&mut a, &mut buf_a).map_err(|_| Error::Data(side_in, name_in.clone()))?
        };
        let got_b = if done_b {
            0
        } else {
            read_full(&mut b, &mut buf_b).map_err(|_| Error::Data(side_out, name_out.clone()))?
        };
        // Until one stream has ended, both have delivered the same number of
        // bytes, so `total_a` is the offset of this chunk on both sides.
        let common = got_a.min(got_b);
        if buf_a[..common] != buf_b[..common] {
            for k in 0..common {
                if buf_a[k] != buf_b[k] {
                    let at = total_a + k as u64;
                    first.get_or_insert(at);
                    count += 1;
                    last = at;
                }
            }
        }
        total_a += got_a as u64;
        total_b += got_b as u64;
        done_a |= got_a < CHUNK;
        done_b |= got_b < CHUNK;
    }
    if total_a != size_in {
        return Err(Error::Data(side_in, name_in));
    }
    if total_b != size_out {
        return Err(Error::Data(side_out, name_out));
    }
    if total_a != total_b {
        return Ok(Content::Length(first.unwrap_or(total_a.min(total_b))));
    }
    Ok(match first {
        None => Content::Equal,
        Some(first) => Content::Differs { first, count, last },
    })
}

/// Byte ranges of the fields that the local header (from its byte 4) and the
/// central record (from its byte 6) share: version needed, general purpose
/// flags, method, DOS time, DOS date, CRC-32, compressed size, uncompressed
/// size.
const SHARED_FIELDS: [(usize, usize); 8] = [
    (0, 2),
    (2, 4),
    (4, 6),
    (6, 8),
    (8, 10),
    (10, 14),
    (14, 18),
    (18, 22),
];

/// Whether a shared field differs between the two local headers while the
/// same field of the two central records does not.
fn local_only(a: &Meta, b: &Meta) -> bool {
    SHARED_FIELDS.iter().any(|&(from, to)| {
        a.local.fixed[from..to] != b.local.fixed[from..to]
            && a.record.central_fixed[from..to] == b.record.central_fixed[from..to]
    })
}

/// Whether two byte ranges, one in each file, hold the same bytes: unequal
/// lengths are different, and equal lengths are compared through two
/// 64 KiB buffers.
fn gaps_equal(
    input: &mut File,
    a: (u64, u64),
    output: &mut File,
    b: (u64, u64),
) -> Result<bool, Error> {
    let (len_a, len_b) = (a.1 - a.0, b.1 - b.0);
    if len_a != len_b {
        return Ok(false);
    }
    if len_a == 0 {
        return Ok(true);
    }
    input
        .seek(SeekFrom::Start(a.0))
        .map_err(|e| Error::Open(Side::Input, e.kind()))?;
    output
        .seek(SeekFrom::Start(b.0))
        .map_err(|e| Error::Open(Side::Output, e.kind()))?;
    let mut ra = input.take(len_a);
    let mut rb = output.take(len_b);
    let mut buf_a = vec![0u8; CHUNK];
    let mut buf_b = vec![0u8; CHUNK];
    loop {
        let got_a =
            read_full(&mut ra, &mut buf_a).map_err(|e| Error::Open(Side::Input, e.kind()))?;
        let got_b =
            read_full(&mut rb, &mut buf_b).map_err(|e| Error::Open(Side::Output, e.kind()))?;
        if got_a != got_b || buf_a[..got_a] != buf_b[..got_b] {
            return Ok(false);
        }
        if got_a < CHUNK {
            return Ok(true);
        }
    }
}

/// Column 17 for an entry present on both sides: the names of the flags that
/// are set, in the spec's order, comma-separated.
fn flags(a: &Meta, b: &Meta, same_gap: bool) -> String {
    let (ra, rb) = (&a.record, &b.record);
    let mut set = Vec::new();
    if (ra.time, ra.date) != (rb.time, rb.date) {
        set.push("mtime");
    }
    if a.local.extra != b.local.extra || ra.extra != rb.extra {
        set.push("extra");
    }
    if ra.flags != rb.flags {
        set.push("gpflags");
    }
    if (ra.flags & DATA_DESCRIPTOR_FLAG) != (rb.flags & DATA_DESCRIPTOR_FLAG) || !same_gap {
        set.push("descriptor");
    }
    if ra.external_attributes != rb.external_attributes
        || ra.internal_attributes != rb.internal_attributes
        || ra.disk_start != rb.disk_start
        || ra.version_made_by != rb.version_made_by
        || ra.version_needed != rb.version_needed
    {
        set.push("attrs");
    }
    if ra.comment != rb.comment {
        set.push("comment");
    }
    if local_only(a, b) || a.local.name != b.local.name {
        set.push("local");
    }
    set.join(",")
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

/// Columns 14 to 16 of an `entry` row for a content comparison result.
fn offset_columns(content: Option<&Content>) -> [String; 3] {
    match content {
        Some(Content::Differs { first, count, last }) => {
            [first.to_string(), count.to_string(), last.to_string()]
        }
        Some(Content::Length(first)) => [first.to_string(), String::new(), String::new()],
        Some(Content::Equal) | None => Default::default(),
    }
}

/// Writes one `entry` row.
fn write_entry(
    out: &mut dyn Write,
    name: &[u8],
    status: &str,
    input: Option<(usize, &Meta)>,
    output: Option<(usize, &Meta)>,
    content: Option<&Content>,
    flags: &str,
) -> Result<(), Error> {
    let [ia, ma, ua, ca, ka] = side_columns(input);
    let [ib, mb, ub, cb, kb] = side_columns(output);
    let [first, count, last] = offset_columns(content);
    writeln!(
        out,
        "entry\t{}\t{status}\t{ia}\t{ib}\t{ma}\t{mb}\t{ua}\t{ub}\t{ca}\t{cb}\t{ka}\t{kb}\t{first}\t{count}\t{last}\t{flags}",
        escape_name(name)
    )
    .map_err(|e| Error::Write(e.kind()))
}

/// Writes one `archive` row.
fn write_archive(out: &mut dyn Write, key: &str, value: impl fmt::Display) -> Result<(), Error> {
    writeln!(out, "archive\t{key}\t{value}").map_err(|e| Error::Write(e.kind()))
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

/// Row counts by status, and the count of entries with a flag set.
#[derive(Default)]
struct Counts {
    identical: u64,
    recompressed: u64,
    changed: u64,
    dropped: u64,
    added: u64,
    flagged: u64,
}

/// The sums of the uncompressed and compressed sizes of an archive's entries.
fn size_sums(archive: &Archive) -> Result<(u64, u64), Error> {
    let overflow = || Error::CentralDirectory(archive.side);
    let mut sums = (0u64, 0u64);
    for meta in &archive.entries {
        sums.0 = sums.0.checked_add(meta.size).ok_or_else(overflow)?;
        sums.1 = sums.1.checked_add(meta.compressed).ok_or_else(overflow)?;
    }
    Ok(sums)
}

/// Whether the two files are byte-equal, compared through two 64 KiB
/// buffers once their lengths are known equal.
fn files_identical(input: &Path, output: &Path, len_in: u64, len_out: u64) -> Result<bool, Error> {
    if len_in != len_out {
        return Ok(false);
    }
    let mut a = File::open(input).map_err(|e| Error::Open(Side::Input, e.kind()))?;
    let mut b = File::open(output).map_err(|e| Error::Open(Side::Output, e.kind()))?;
    let mut buf_a = vec![0u8; CHUNK];
    let mut buf_b = vec![0u8; CHUNK];
    loop {
        let got_a =
            read_full(&mut a, &mut buf_a).map_err(|e| Error::Open(Side::Input, e.kind()))?;
        let got_b =
            read_full(&mut b, &mut buf_b).map_err(|e| Error::Open(Side::Output, e.kind()))?;
        if got_a != got_b || buf_a[..got_a] != buf_b[..got_b] {
            return Ok(false);
        }
        if got_a < CHUNK {
            return Ok(true);
        }
    }
}

/// Compares the two archives and writes every row. Returns whether the exit
/// status is 0: every entry `identical` with no flags, the order preserved
/// and the archive comments equal.
fn compare(input: &Path, output: &Path, out: &mut dyn Write) -> Result<bool, Error> {
    let mut a = open(input, Side::Input)?;
    let mut b = open(output, Side::Output)?;
    let mut gap_in = File::open(input).map_err(|e| Error::Open(Side::Input, e.kind()))?;
    let mut gap_out = File::open(output).map_err(|e| Error::Open(Side::Output, e.kind()))?;
    let out_index: HashMap<Vec<u8>, usize> = b
        .entries
        .iter()
        .enumerate()
        .map(|(j, meta)| (meta.name().to_vec(), j))
        .collect();

    let mut counts = Counts::default();
    let mut matched = vec![false; b.entries.len()];
    let mut order_preserved = true;
    let mut previous: Option<usize> = None;
    for i in 0..a.entries.len() {
        match out_index.get(a.entries[i].name()).copied() {
            Some(j) => {
                matched[j] = true;
                if previous.is_some_and(|p| j < p) {
                    order_preserved = false;
                }
                previous = Some(j);
                let (ea, eb) = (&a.entries[i], &b.entries[j]);
                let same_meta = ea.method == eb.method
                    && ea.crc == eb.crc
                    && ea.size == eb.size
                    && ea.compressed == eb.compressed;
                let identical = same_meta && raw_equal(&mut a, i, &mut b, j)?;
                let (status, found) = if identical {
                    counts.identical += 1;
                    ("identical", None)
                } else {
                    let found = content(&mut a, i, &mut b, j)?;
                    if matches!(found, Content::Equal) {
                        counts.recompressed += 1;
                        ("recompressed", None)
                    } else {
                        counts.changed += 1;
                        ("changed", Some(found))
                    }
                };
                let (ea, eb) = (&a.entries[i], &b.entries[j]);
                let same_gap = gaps_equal(&mut gap_in, ea.gap, &mut gap_out, eb.gap)?;
                let set = flags(ea, eb, same_gap);
                if !set.is_empty() {
                    counts.flagged += 1;
                }
                write_entry(
                    out,
                    ea.name(),
                    status,
                    Some((i, ea)),
                    Some((j, eb)),
                    found.as_ref(),
                    &set,
                )?;
            }
            None => {
                counts.dropped += 1;
                let ea = &a.entries[i];
                write_entry(out, ea.name(), "dropped", Some((i, ea)), None, None, "")?;
            }
        }
    }
    for (j, eb) in b.entries.iter().enumerate() {
        if !matched[j] {
            counts.added += 1;
            write_entry(out, eb.name(), "added", None, Some((j, eb)), None, "")?;
        }
    }

    let comment_equal = a.comment == b.comment;
    let (uncompressed_in, compressed_in) = size_sums(&a)?;
    let (uncompressed_out, compressed_out) = size_sums(&b)?;
    let same_files = files_identical(input, output, a.bytes, b.bytes)?;

    write_archive(out, "entries_in", a.entries.len() as u64)?;
    write_archive(out, "entries_out", b.entries.len() as u64)?;
    write_archive(out, "identical", counts.identical)?;
    write_archive(out, "recompressed", counts.recompressed)?;
    write_archive(out, "changed", counts.changed)?;
    write_archive(out, "dropped", counts.dropped)?;
    write_archive(out, "added", counts.added)?;
    write_archive(out, "flagged", counts.flagged)?;
    write_archive(out, "order_preserved", yes_no(order_preserved))?;
    write_archive(out, "comment_equal", yes_no(comment_equal))?;
    write_archive(out, "prefix_in", a.prefix)?;
    write_archive(out, "prefix_out", b.prefix)?;
    write_archive(out, "central_start_in", a.central_start)?;
    write_archive(out, "central_start_out", b.central_start)?;
    write_archive(out, "uncompressed_in", uncompressed_in)?;
    write_archive(out, "uncompressed_out", uncompressed_out)?;
    write_archive(out, "compressed_in", compressed_in)?;
    write_archive(out, "compressed_out", compressed_out)?;
    write_archive(out, "bytes_in", a.bytes)?;
    write_archive(out, "bytes_out", b.bytes)?;
    write_archive(out, "files_identical", yes_no(same_files))?;

    let every_entry_identical = counts.recompressed == 0
        && counts.changed == 0
        && counts.dropped == 0
        && counts.added == 0
        && counts.flagged == 0;
    Ok(every_entry_identical && order_preserved && comment_equal)
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
