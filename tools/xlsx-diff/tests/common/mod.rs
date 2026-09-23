//! Helpers shared by the xlsx-diff tests (`specs/0004-xlsx-diff.md`). Every
//! archive is generated here from fictional content, in a temporary
//! directory that is removed when the test ends. Nothing is committed.

#![allow(dead_code)]

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, DateTime, ZipArchive, ZipWriter};

/// A temporary directory, removed when dropped.
pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new(label: &str) -> TempDir {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("xlsx-diff-test-{}-{label}-{n}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create the temporary directory");
        TempDir(dir)
    }

    pub fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// One entry to write: its name, bytes, method and deflate level.
#[derive(Clone)]
pub struct Spec {
    pub name: String,
    pub data: Vec<u8>,
    pub method: CompressionMethod,
    pub level: Option<i64>,
}

pub fn deflated(name: &str, data: &[u8]) -> Spec {
    Spec {
        name: name.to_string(),
        data: data.to_vec(),
        method: CompressionMethod::Deflated,
        level: None,
    }
}

pub fn stored(name: &str, data: &[u8]) -> Spec {
    Spec {
        method: CompressionMethod::Stored,
        ..deflated(name, data)
    }
}

/// Fictional text of a given number of lines.
pub fn text(lines: usize) -> Vec<u8> {
    (0..lines)
        .map(|i| format!("fictional row {i}: alpha beta gamma {}\n", i * 7 % 13))
        .collect::<String>()
        .into_bytes()
}

/// Fictional bytes of a given length.
pub fn pattern(len: usize) -> Vec<u8> {
    const WORD: &[u8] = b"fictional-test-bytes ";
    (0..len).map(|i| WORD[i % WORD.len()]).collect()
}

/// A copy of `data` with the byte at each offset xored with 0x01.
pub fn flipped(data: &[u8], offsets: &[usize]) -> Vec<u8> {
    let mut out = data.to_vec();
    for &at in offsets {
        out[at] ^= 0x01;
    }
    out
}

pub fn write_zip(path: &Path, entries: &[Spec]) {
    let file = fs::File::create(path).expect("create the archive");
    let mut writer = ZipWriter::new(file);
    for entry in entries {
        let options = SimpleFileOptions::default()
            .compression_method(entry.method)
            .compression_level(entry.level)
            .last_modified_time(DateTime::default());
        writer
            .start_file(entry.name.as_str(), options)
            .expect("start an entry");
        writer.write_all(&entry.data).expect("write an entry");
    }
    writer.finish().expect("finish the archive");
}

/// What `run` returned and printed.
pub struct Outcome {
    pub status: u8,
    pub stdout: String,
    pub stderr: String,
}

pub fn run(input: &Path, output: &Path) -> Outcome {
    let (mut stdout, mut stderr) = (Vec::new(), Vec::new());
    let status = xlsx_diff::run(input, output, &mut stdout, &mut stderr);
    Outcome {
        status,
        stdout: String::from_utf8(stdout).expect("stdout is UTF-8"),
        stderr: String::from_utf8(stderr).expect("stderr is UTF-8"),
    }
}

/// The rows of one kind, split into columns. Every line must be an `entry`
/// row of 17 columns, an `archive` row of 3, or an `xml` row of the width
/// its kind word gives (`specs/0014-xml-comparison.md`, "Output").
pub fn rows<'a>(stdout: &'a str, kind: &str) -> Vec<Vec<&'a str>> {
    let mut selected = Vec::new();
    for line in stdout.lines() {
        let columns: Vec<&str> = line.split('\t').collect();
        match columns[0] {
            "entry" => assert_eq!(columns.len(), 17, "entry row width: {line}"),
            "archive" => assert_eq!(columns.len(), 3, "archive row width: {line}"),
            "xml" => {
                let width = match columns.get(2).copied() {
                    Some("parse") => 8,
                    Some("element") | Some("attribute") => 6,
                    Some("values") => 7,
                    other => panic!("unexpected xml row kind {other:?}"),
                };
                assert_eq!(columns.len(), width, "xml row width: {line}");
            }
            other => panic!("unexpected row kind {other}"),
        }
        if columns[0] == kind {
            selected.push(columns);
        }
    }
    selected
}

/// Every `archive` key, in the spec's order.
pub const ARCHIVE_KEYS: [&str; 25] = [
    "entries_in",
    "entries_out",
    "identical",
    "recompressed",
    "changed",
    "dropped",
    "added",
    "flagged",
    "order_preserved",
    "comment_equal",
    "prefix_in",
    "prefix_out",
    "central_start_in",
    "central_start_out",
    "uncompressed_in",
    "uncompressed_out",
    "compressed_in",
    "compressed_out",
    "bytes_in",
    "bytes_out",
    "files_identical",
    "xml_compared",
    "xml_parse_errors",
    "xml_names_equal",
    "xml_values_equal",
];

/// The `archive` rows as `key=value` strings, in order.
pub fn archive_rows(stdout: &str) -> Vec<String> {
    rows(stdout, "archive")
        .into_iter()
        .map(|c| format!("{}={}", c[1], c[2]))
        .collect()
}

/// The first seven `archive` rows (the entry counts and the status counts)
/// as `key=value` strings, after checking that every key is present in order.
pub fn status_counts(stdout: &str) -> Vec<String> {
    let keys: Vec<String> = rows(stdout, "archive")
        .iter()
        .map(|c| c[1].to_string())
        .collect();
    assert_eq!(keys, ARCHIVE_KEYS, "archive keys in order");
    archive_rows(stdout).into_iter().take(7).collect()
}

/// The value of the one `archive` row with `key`.
pub fn archive_value(stdout: &str, key: &str) -> String {
    let found: Vec<String> = rows(stdout, "archive")
        .iter()
        .filter(|c| c[1] == key)
        .map(|c| c[2].to_string())
        .collect();
    assert_eq!(found.len(), 1, "one archive row {key}");
    found[0].clone()
}

/// The entry counts and status counts expected, as `status_counts` gives
/// them.
pub fn expected_archive(counts: [u64; 7]) -> Vec<String> {
    let keys = [
        "entries_in",
        "entries_out",
        "identical",
        "recompressed",
        "changed",
        "dropped",
        "added",
    ];
    keys.iter()
        .zip(counts)
        .map(|(k, v)| format!("{k}={v}"))
        .collect()
}

/// The (name, status) pairs of the `entry` rows.
pub fn summary(stdout: &str) -> Vec<(String, String)> {
    rows(stdout, "entry")
        .iter()
        .map(|r| (r[1].to_string(), r[2].to_string()))
        .collect()
}

/// Method, uncompressed size, compressed size and CRC-32 of every entry, read
/// independently of the tool.
pub fn meta(path: &Path) -> Vec<[String; 4]> {
    let mut archive = ZipArchive::new(fs::File::open(path).expect("open")).expect("read");
    (0..archive.len())
        .map(|i| {
            let entry = archive.by_index_raw(i).expect("entry");
            #[allow(deprecated)]
            let method = entry.compression().to_u16();
            [
                method.to_string(),
                entry.size().to_string(),
                entry.compressed_size().to_string(),
                format!("{:08x}", entry.crc32()),
            ]
        })
        .collect()
}

/// Asserts columns 4 to 13 of an entry row against the independent metadata.
pub fn assert_columns(
    row: &[&str],
    input: Option<(usize, &[String; 4])>,
    output: Option<(usize, &[String; 4])>,
) {
    let side = |s: Option<(usize, &[String; 4])>| match s {
        Some((index, m)) => [
            index.to_string(),
            m[0].clone(),
            m[1].clone(),
            m[2].clone(),
            m[3].clone(),
        ],
        None => Default::default(),
    };
    let (a, b) = (side(input), side(output));
    assert_eq!(row[3], a[0], "column 4");
    assert_eq!(row[4], b[0], "column 5");
    for k in 0..4 {
        assert_eq!(row[5 + 2 * k], a[k + 1], "column {}", 6 + 2 * k);
        assert_eq!(row[6 + 2 * k], b[k + 1], "column {}", 7 + 2 * k);
    }
}

/// Asserts columns 14 to 17 of an entry row.
pub fn assert_tail(row: &[&str], expected: [&str; 4]) {
    assert_eq!(&row[13..], expected, "columns 14 to 17 of {}", row[1]);
}

/// Replaces every occurrence of `from` with `to` (same length) and returns
/// the count.
pub fn patch(bytes: &mut [u8], from: &[u8], to: &[u8]) -> usize {
    assert_eq!(from.len(), to.len());
    let (mut count, mut i) = (0, 0);
    while i + from.len() <= bytes.len() {
        if &bytes[i..i + from.len()] == from {
            bytes[i..i + from.len()].copy_from_slice(to);
            count += 1;
            i += from.len();
        } else {
            i += 1;
        }
    }
    count
}

fn le16(bytes: &[u8], at: usize) -> usize {
    usize::from(u16::from_le_bytes([bytes[at], bytes[at + 1]]))
}

fn le32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

/// Offsets of the local header and the central directory record of the
/// entry named `name`.
pub fn header_offsets(bytes: &[u8], name: &[u8]) -> (usize, usize) {
    let find = |signature: [u8; 4], name_len_at: usize, name_at: usize| {
        (0..bytes.len().saturating_sub(name_at + name.len()))
            .find(|&i| {
                bytes[i..i + 4] == signature
                    && le16(bytes, i + name_len_at) == name.len()
                    && &bytes[i + name_at..i + name_at + name.len()] == name
            })
            .expect("the header is in the archive")
    };
    (
        find([0x50, 0x4b, 0x03, 0x04], 26, 30),
        find([0x50, 0x4b, 0x01, 0x02], 28, 46),
    )
}

/// Writes a new CRC-32 into both headers of the entry named `name`.
pub fn set_crc(bytes: &mut [u8], name: &[u8], crc: u32) {
    let (local, central) = header_offsets(bytes, name);
    bytes[local + 14..local + 18].copy_from_slice(&crc.to_le_bytes());
    bytes[central + 16..central + 20].copy_from_slice(&crc.to_le_bytes());
}

/// The CRC-32 recorded in the central directory for the entry named `name`.
pub fn crc_of(bytes: &[u8], name: &[u8]) -> u32 {
    let (_, central) = header_offsets(bytes, name);
    le32(bytes, central + 16)
}

/// Writes a compression method number into both headers of the entry named
/// `name`.
pub fn set_method(bytes: &mut [u8], name: &[u8], method: u16) {
    let (local, central) = header_offsets(bytes, name);
    bytes[local + 8..local + 10].copy_from_slice(&method.to_le_bytes());
    bytes[central + 10..central + 12].copy_from_slice(&method.to_le_bytes());
}

/// Removes the last `cut` bytes of the entry data that directly precede the
/// central directory, and moves the end of central directory record's
/// offset down to match. The central record keeps its declared sizes, so the
/// entry's data now runs short.
pub fn cut_entry_tail(bytes: &[u8], cut: usize) -> Vec<u8> {
    let eocd = (0..bytes.len() - 21)
        .rev()
        .find(|&i| bytes[i..i + 4] == [0x50, 0x4b, 0x05, 0x06])
        .expect("the end of central directory record");
    let central = le32(bytes, eocd + 16) as usize;
    let mut out = Vec::with_capacity(bytes.len() - cut);
    out.extend_from_slice(&bytes[..central - cut]);
    out.extend_from_slice(&bytes[central..]);
    let new_eocd = eocd - cut;
    let new_central = u32::try_from(central - cut).expect("small archive");
    out[new_eocd + 16..new_eocd + 20].copy_from_slice(&new_central.to_le_bytes());
    out
}

/// CRC-32 (IEEE) of `data`, computed bit by bit, independent of the zip crate.
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = if crc & 1 == 1 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

/// A raw deflate stream of stored blocks holding `data`, split at each
/// offset in `splits` (an offset equal to the length gives an empty final
/// block). Every block but the last has BFINAL clear.
pub fn stored_blocks(data: &[u8], splits: &[usize]) -> Vec<u8> {
    let mut bounds = vec![0];
    bounds.extend_from_slice(splits);
    bounds.push(data.len());
    let mut out = Vec::new();
    for (k, pair) in bounds.windows(2).enumerate() {
        let piece = &data[pair[0]..pair[1]];
        let len = u16::try_from(piece.len()).expect("a stored block holds at most 65535 bytes");
        out.push(u8::from(k + 2 == bounds.len()));
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(piece);
    }
    out
}

/// A zip archive with one deflated entry whose raw stream is `raw`, written
/// byte by byte by `raw_zip`, with the CRC-32 and size of `data`.
pub fn single_deflate_entry_zip(name: &str, data: &[u8], raw: &[u8]) -> Vec<u8> {
    raw_zip(&[], &[raw_deflated(name, data, raw)], &[])
}

/// One entry of an archive written byte by byte by `raw_zip`. Every header
/// field the flags compare is set here directly; the local header repeats
/// the central record's values unless a `local_*` field overrides one.
#[derive(Clone)]
pub struct RawEntry {
    pub name: Vec<u8>,
    /// The uncompressed bytes, for the CRC-32 and the uncompressed size.
    pub data: Vec<u8>,
    /// 0 (stored, `raw` equals `data`) or 8 (`raw` is the deflate stream).
    pub method: u16,
    /// The bytes stored for the entry.
    pub raw: Vec<u8>,
    pub version_made_by: u16,
    /// General purpose flags; bit 3 writes a data descriptor after the data
    /// and zeros in the local header's CRC-32 and size fields.
    pub flags: u16,
    pub time: u16,
    pub date: u16,
    pub local_extra: Vec<u8>,
    pub central_extra: Vec<u8>,
    pub comment: Vec<u8>,
    pub external_attributes: u32,
    /// Version needed to extract, in both headers unless overridden below.
    pub version_needed: u16,
    /// Local header values that differ from the central record's, when set.
    pub local_version_needed: Option<u16>,
    pub local_flags: Option<u16>,
    pub local_time: Option<u16>,
    pub local_date: Option<u16>,
    pub local_name: Option<Vec<u8>>,
    /// Internal attributes and disk number start in the central record.
    pub internal_attributes: u16,
    pub disk_start: u16,
    /// With flag bit 3: whether the data descriptor starts with its optional
    /// signature.
    pub descriptor_signature: bool,
    /// Bytes written after the entry's data and descriptor, before the next
    /// local header or the central directory.
    pub padding_after: Vec<u8>,
    /// Writes 0xffffffff in the central record's two size fields, as a zip64
    /// placeholder; the sizes then come from a zip64 extra field that the
    /// caller puts in `central_extra`.
    pub central_size_placeholder: bool,
    /// As `central_size_placeholder`, for the compressed size field alone.
    pub central_compressed_placeholder: bool,
    /// As `central_size_placeholder`, for the uncompressed size field alone.
    pub central_uncompressed_placeholder: bool,
    /// A compressed size to declare in both headers instead of the length of
    /// `raw` (it may run over the next local header).
    pub declared_compressed: Option<usize>,
}

/// A stored entry: version 2.0 made by MS-DOS, no flags, DOS time 00:00:00
/// on 1980-01-01, no extra fields, no comment and no attributes.
pub fn raw_stored(name: &str, data: &[u8]) -> RawEntry {
    RawEntry {
        name: name.as_bytes().to_vec(),
        data: data.to_vec(),
        method: 0,
        raw: data.to_vec(),
        version_made_by: 20,
        flags: 0,
        time: 0,
        date: 0x0021,
        local_extra: Vec::new(),
        central_extra: Vec::new(),
        comment: Vec::new(),
        external_attributes: 0,
        version_needed: 20,
        local_version_needed: None,
        local_flags: None,
        local_time: None,
        local_date: None,
        local_name: None,
        internal_attributes: 0,
        disk_start: 0,
        descriptor_signature: true,
        padding_after: Vec::new(),
        central_size_placeholder: false,
        central_compressed_placeholder: false,
        central_uncompressed_placeholder: false,
        declared_compressed: None,
    }
}

/// A deflated entry whose deflate stream is `raw`, otherwise as `raw_stored`.
pub fn raw_deflated(name: &str, data: &[u8], raw: &[u8]) -> RawEntry {
    RawEntry {
        method: 8,
        raw: raw.to_vec(),
        ..raw_stored(name, data)
    }
}

/// An archive written byte by byte: `prefix` bytes, then each entry's local
/// header, extra field, data and (for flag bit 3) data descriptor, then the
/// central directory and the end of central directory record with
/// `comment`. Offsets in the headers count from the end of `prefix`.
pub fn raw_zip(prefix: &[u8], entries: &[RawEntry], comment: &[u8]) -> Vec<u8> {
    let order: Vec<usize> = (0..entries.len()).collect();
    raw_zip_with_local_order(prefix, entries, &order, comment)
}

/// As `raw_zip`, but the local entries are written in `local_order` (a
/// permutation of the entry indices) while the central directory keeps the
/// order of `entries`.
pub fn raw_zip_with_local_order(
    prefix: &[u8],
    entries: &[RawEntry],
    local_order: &[usize],
    comment: &[u8],
) -> Vec<u8> {
    fn u16_of(n: usize) -> [u8; 2] {
        u16::try_from(n).expect("a 16-bit field").to_le_bytes()
    }
    fn u32_of(n: usize) -> [u8; 4] {
        u32::try_from(n).expect("a 32-bit field").to_le_bytes()
    }
    let mut out = prefix.to_vec();
    let mut local_offsets = vec![0usize; entries.len()];
    let mut sorted_order = local_order.to_vec();
    sorted_order.sort_unstable();
    assert_eq!(
        sorted_order,
        (0..entries.len()).collect::<Vec<_>>(),
        "local_order is a permutation"
    );
    for &k in local_order {
        let e = &entries[k];
        local_offsets[k] = out.len() - prefix.len();
        let compressed = e.declared_compressed.unwrap_or(e.raw.len());
        let descriptor = e.flags & 0x0008 != 0;
        let crc = crc32(&e.data).to_le_bytes();
        let local_name = e.local_name.as_ref().unwrap_or(&e.name);
        let local_version = e.local_version_needed.unwrap_or(e.version_needed);
        out.extend_from_slice(&[0x50, 0x4b, 0x03, 0x04]);
        out.extend_from_slice(&local_version.to_le_bytes());
        out.extend_from_slice(&e.local_flags.unwrap_or(e.flags).to_le_bytes());
        out.extend_from_slice(&e.method.to_le_bytes());
        out.extend_from_slice(&e.local_time.unwrap_or(e.time).to_le_bytes());
        out.extend_from_slice(&e.local_date.unwrap_or(e.date).to_le_bytes());
        if descriptor {
            out.extend_from_slice(&[0; 12]);
        } else {
            out.extend_from_slice(&crc);
            out.extend_from_slice(&u32_of(compressed));
            out.extend_from_slice(&u32_of(e.data.len()));
        }
        out.extend_from_slice(&u16_of(local_name.len()));
        out.extend_from_slice(&u16_of(e.local_extra.len()));
        out.extend_from_slice(local_name);
        out.extend_from_slice(&e.local_extra);
        out.extend_from_slice(&e.raw);
        if descriptor {
            if e.descriptor_signature {
                out.extend_from_slice(&[0x50, 0x4b, 0x07, 0x08]);
            }
            out.extend_from_slice(&crc);
            out.extend_from_slice(&u32_of(compressed));
            out.extend_from_slice(&u32_of(e.data.len()));
        }
        out.extend_from_slice(&e.padding_after);
    }
    let central = out.len() - prefix.len();
    for (e, &offset) in entries.iter().zip(&local_offsets) {
        out.extend_from_slice(&[0x50, 0x4b, 0x01, 0x02]);
        out.extend_from_slice(&e.version_made_by.to_le_bytes());
        out.extend_from_slice(&e.version_needed.to_le_bytes());
        out.extend_from_slice(&e.flags.to_le_bytes());
        out.extend_from_slice(&e.method.to_le_bytes());
        out.extend_from_slice(&e.time.to_le_bytes());
        out.extend_from_slice(&e.date.to_le_bytes());
        out.extend_from_slice(&crc32(&e.data).to_le_bytes());
        if e.central_size_placeholder || e.central_compressed_placeholder {
            out.extend_from_slice(&[0xff; 4]);
        } else {
            out.extend_from_slice(&u32_of(e.declared_compressed.unwrap_or(e.raw.len())));
        }
        if e.central_size_placeholder || e.central_uncompressed_placeholder {
            out.extend_from_slice(&[0xff; 4]);
        } else {
            out.extend_from_slice(&u32_of(e.data.len()));
        }
        out.extend_from_slice(&u16_of(e.name.len()));
        out.extend_from_slice(&u16_of(e.central_extra.len()));
        out.extend_from_slice(&u16_of(e.comment.len()));
        out.extend_from_slice(&e.disk_start.to_le_bytes());
        out.extend_from_slice(&e.internal_attributes.to_le_bytes());
        out.extend_from_slice(&e.external_attributes.to_le_bytes());
        out.extend_from_slice(&u32_of(offset));
        out.extend_from_slice(&e.name);
        out.extend_from_slice(&e.central_extra);
        out.extend_from_slice(&e.comment);
    }
    let central_len = out.len() - prefix.len() - central;
    out.extend_from_slice(&[0x50, 0x4b, 0x05, 0x06, 0, 0, 0, 0]);
    out.extend_from_slice(&u16_of(entries.len()));
    out.extend_from_slice(&u16_of(entries.len()));
    out.extend_from_slice(&u32_of(central_len));
    out.extend_from_slice(&u32_of(central));
    out.extend_from_slice(&u16_of(comment.len()));
    out.extend_from_slice(comment);
    out
}

/// The central directory offset recorded in the end of central directory
/// record (the last one in `bytes`), read independently of the tool.
pub fn recorded_central_offset(bytes: &[u8]) -> u64 {
    let eocd = (0..bytes.len() - 21)
        .rev()
        .find(|&i| bytes[i..i + 4] == [0x50, 0x4b, 0x05, 0x06])
        .expect("the end of central directory record");
    u64::from(le32(bytes, eocd + 16))
}

/// The version-made-by field of the central directory record of the entry
/// named `name`, read independently of the tool.
pub fn central_version_made_by(bytes: &[u8], name: &[u8]) -> u16 {
    let (_, central) = header_offsets(bytes, name);
    u16::from_le_bytes([bytes[central + 4], bytes[central + 5]])
}

pub fn binary(args: &[&Path]) -> (Option<i32>, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_xlsx-diff"))
        .args(args)
        .output()
        .expect("run the binary");
    (
        out.status.code(),
        String::from_utf8(out.stdout).expect("stdout is UTF-8"),
        String::from_utf8(out.stderr).expect("stderr is UTF-8"),
    )
}
