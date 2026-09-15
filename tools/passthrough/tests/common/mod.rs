//! Helpers shared by the passthrough tests (`specs/0006-passthrough.md`).
//! Every archive is generated here from fictional content, in a temporary
//! directory that is removed when the test ends. Nothing is committed. The
//! byte-level builder follows the patterns of `xlsx-diff`'s tests, copied
//! into this crate as the spec allows (not shared across crates).

#![allow(dead_code)]

use std::fs;
use std::io::Write;
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, DateTime, ZipWriter};

/// A temporary directory, removed when dropped.
pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new(label: &str) -> TempDir {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "passthrough-test-{}-{label}-{n}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create the temporary directory");
        TempDir(dir)
    }

    pub fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }

    pub fn root(&self) -> &Path {
        &self.0
    }

    /// The names in the directory, sorted.
    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(&self.0)
            .expect("list the temporary directory")
            .map(|e| {
                e.expect("a directory entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        names.sort();
        names
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Fictional text of a given number of lines.
pub fn text(lines: usize) -> Vec<u8> {
    (0..lines)
        .map(|i| format!("fictional row {i}: alpha beta gamma {}\n", i * 7 % 13))
        .collect::<String>()
        .into_bytes()
}

/// Options for an entry written with the zip crate: the method, and the
/// DOS time 1980-01-01 00:00:00.
pub fn options(method: CompressionMethod) -> SimpleFileOptions {
    SimpleFileOptions::default()
        .compression_method(method)
        .last_modified_time(DateTime::default())
}

/// Writes an archive with the zip crate; `build` adds the entries.
pub fn zip_with(path: &Path, build: impl FnOnce(&mut ZipWriter<fs::File>)) {
    let file = fs::File::create(path).expect("create the archive");
    let mut writer = ZipWriter::new(file);
    build(&mut writer);
    writer.finish().expect("finish the archive");
}

/// Adds one file entry with the zip crate.
pub fn add_file(
    writer: &mut ZipWriter<fs::File>,
    name: &str,
    data: &[u8],
    options: SimpleFileOptions,
) {
    writer.start_file(name, options).expect("start an entry");
    writer.write_all(data).expect("write an entry");
}

/// What the passthrough binary did.
pub struct Copied {
    pub code: Option<i32>,
    pub signal: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

/// Runs the binary on `args` with stdin closed, the fault variables removed
/// and then `env` applied.
pub fn passthrough(args: &[&Path], env: &[(&str, &str)]) -> Copied {
    let mut command = Command::new(env!("CARGO_BIN_EXE_passthrough"));
    command
        .args(args)
        .stdin(Stdio::null())
        .env_remove("PASSTHROUGH_FAULT")
        .env_remove("PASSTHROUGH_FAULT_ALLOWED");
    for (key, value) in env {
        command.env(key, value);
    }
    let out = command.output().expect("run the binary");
    Copied {
        code: out.status.code(),
        signal: out.status.signal(),
        stdout: String::from_utf8(out.stdout).expect("stdout is UTF-8"),
        stderr: String::from_utf8(out.stderr).expect("stderr is UTF-8"),
    }
}

/// Copies `input` to `output` and asserts a clean success: exit 0, nothing
/// on stdout or stderr.
pub fn copy_ok(input: &Path, output: &Path) {
    let copied = passthrough(&[input, output], &[]);
    assert_eq!(
        copied.code,
        Some(0),
        "passthrough exit; stderr: {}",
        copied.stderr
    );
    assert_eq!(copied.stdout, "", "passthrough stdout");
    assert_eq!(copied.stderr, "", "passthrough stderr");
}

/// What `xlsx_diff::run` returned and printed.
pub struct Outcome {
    pub status: u8,
    pub stdout: String,
    pub stderr: String,
}

pub fn compare(input: &Path, output: &Path) -> Outcome {
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

/// A raw deflate stream of one final stored block holding `data`.
pub fn stored_block(data: &[u8]) -> Vec<u8> {
    let len = u16::try_from(data.len()).expect("a stored block holds at most 65535 bytes");
    let mut out = vec![1];
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(&(!len).to_le_bytes());
    out.extend_from_slice(data);
    out
}

/// One entry of an archive written byte by byte by `raw_zip`. The local
/// header repeats the central record's values.
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
    pub version_needed: u16,
    /// General purpose flags; bit 3 writes a data descriptor, with its
    /// signature, after the data and zeros in the local header's CRC-32 and
    /// size fields.
    pub flags: u16,
    pub time: u16,
    pub date: u16,
    pub local_extra: Vec<u8>,
    pub central_extra: Vec<u8>,
    pub comment: Vec<u8>,
    pub external_attributes: u32,
    /// Writes 0xffffffff in the central record's two size fields, as a zip64
    /// placeholder; the sizes then come from a zip64 extra field that the
    /// caller puts in `central_extra`.
    pub central_size_placeholder: bool,
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
        version_needed: 20,
        flags: 0,
        time: 0,
        date: 0x0021,
        local_extra: Vec::new(),
        central_extra: Vec::new(),
        comment: Vec::new(),
        external_attributes: 0,
        central_size_placeholder: false,
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
    fn u16_of(n: usize) -> [u8; 2] {
        u16::try_from(n).expect("a 16-bit field").to_le_bytes()
    }
    fn u32_of(n: usize) -> [u8; 4] {
        u32::try_from(n).expect("a 32-bit field").to_le_bytes()
    }
    let mut out = prefix.to_vec();
    let mut local_offsets = Vec::with_capacity(entries.len());
    for e in entries {
        local_offsets.push(out.len() - prefix.len());
        let descriptor = e.flags & 0x0008 != 0;
        let crc = crc32(&e.data).to_le_bytes();
        out.extend_from_slice(&[0x50, 0x4b, 0x03, 0x04]);
        out.extend_from_slice(&e.version_needed.to_le_bytes());
        out.extend_from_slice(&e.flags.to_le_bytes());
        out.extend_from_slice(&e.method.to_le_bytes());
        out.extend_from_slice(&e.time.to_le_bytes());
        out.extend_from_slice(&e.date.to_le_bytes());
        if descriptor {
            out.extend_from_slice(&[0; 12]);
        } else {
            out.extend_from_slice(&crc);
            out.extend_from_slice(&u32_of(e.raw.len()));
            out.extend_from_slice(&u32_of(e.data.len()));
        }
        out.extend_from_slice(&u16_of(e.name.len()));
        out.extend_from_slice(&u16_of(e.local_extra.len()));
        out.extend_from_slice(&e.name);
        out.extend_from_slice(&e.local_extra);
        out.extend_from_slice(&e.raw);
        if descriptor {
            out.extend_from_slice(&[0x50, 0x4b, 0x07, 0x08]);
            out.extend_from_slice(&crc);
            out.extend_from_slice(&u32_of(e.raw.len()));
            out.extend_from_slice(&u32_of(e.data.len()));
        }
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
        if e.central_size_placeholder {
            out.extend_from_slice(&[0xff; 8]);
        } else {
            out.extend_from_slice(&u32_of(e.raw.len()));
            out.extend_from_slice(&u32_of(e.data.len()));
        }
        out.extend_from_slice(&u16_of(e.name.len()));
        out.extend_from_slice(&u16_of(e.central_extra.len()));
        out.extend_from_slice(&u16_of(e.comment.len()));
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
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
