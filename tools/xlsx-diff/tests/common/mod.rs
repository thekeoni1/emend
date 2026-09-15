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
/// row of 17 columns or an `archive` row of 3.
pub fn rows<'a>(stdout: &'a str, kind: &str) -> Vec<Vec<&'a str>> {
    let mut selected = Vec::new();
    for line in stdout.lines() {
        let columns: Vec<&str> = line.split('\t').collect();
        match columns[0] {
            "entry" => assert_eq!(columns.len(), 17, "entry row width: {line}"),
            "archive" => assert_eq!(columns.len(), 3, "archive row width: {line}"),
            other => panic!("unexpected row kind {other}"),
        }
        if columns[0] == kind {
            selected.push(columns);
        }
    }
    selected
}

/// The `archive` rows as `key=value` strings, in order.
pub fn archive_rows(stdout: &str) -> Vec<String> {
    rows(stdout, "archive")
        .into_iter()
        .map(|c| format!("{}={}", c[1], c[2]))
        .collect()
}

/// The `archive` rows expected for the given counts, in the P2 order.
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

/// Sets general purpose bit 0 (encrypted) in both headers of the entry
/// named `name`, without encrypting anything.
pub fn set_encrypted_flag(bytes: &mut [u8], name: &[u8]) {
    let (local, central) = header_offsets(bytes, name);
    bytes[local + 6] |= 0x01;
    bytes[central + 8] |= 0x01;
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
