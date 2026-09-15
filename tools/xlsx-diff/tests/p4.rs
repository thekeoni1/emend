//! T1 P4 tests (`specs/0004-xlsx-diff.md`): the `local` flag and the `attrs`
//! extension, a `files_identical` difference inside a prefix, the no-leak
//! sentinel test, and the zip64 entry (ignored by default). Every archive is
//! generated from fictional content in a temporary directory; nothing is
//! committed.

mod common;

use std::fs;
use std::io::Write;
use std::path::Path;

use common::*;
use zip::unstable::write::FileOptionsExt;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, DateTime, ZipWriter};

/// A stored entry of fictional text used as the base for the flag cases.
fn base(name: &str) -> RawEntry {
    raw_stored(name, &text(12))
}

#[test]
fn local_header_fields_each_alone() {
    let cases: Vec<(&str, RawEntry, &str)> = vec![
        (
            "local version needed",
            RawEntry {
                local_version_needed: Some(10),
                ..base("f.txt")
            },
            "local",
        ),
        (
            "local flags",
            RawEntry {
                local_flags: Some(0x0800),
                ..base("f.txt")
            },
            "local",
        ),
        (
            "local time",
            RawEntry {
                local_time: Some(0x6000),
                ..base("f.txt")
            },
            "local",
        ),
        (
            "local date",
            RawEntry {
                local_date: Some(0x5a2f),
                ..base("f.txt")
            },
            "local",
        ),
        (
            "local name",
            RawEntry {
                local_name: Some(b"g.txt".to_vec()),
                ..base("f.txt")
            },
            "local",
        ),
        (
            "central version needed",
            RawEntry {
                version_needed: 10,
                local_version_needed: Some(20),
                ..base("f.txt")
            },
            "attrs",
        ),
    ];
    let dir = TempDir::new("local");
    let input = dir.path("in.zip");
    fs::write(&input, raw_zip(&[], &[base("e.txt"), base("f.txt")], b"")).expect("write");
    for (k, (label, changed, flag)) in cases.into_iter().enumerate() {
        let output = dir.path(&format!("case-{k}.zip"));
        fs::write(&output, raw_zip(&[], &[base("e.txt"), changed], b"")).expect("write");
        assert_eq!(meta(&input), meta(&output), "precondition for {label}");

        let result = run(&input, &output);
        assert_eq!(result.status, 1, "{label}");
        assert_eq!(result.stderr, "", "{label}");
        let entry_rows = rows(&result.stdout, "entry");
        assert_eq!(entry_rows.len(), 2, "{label}");
        assert_eq!(
            (entry_rows[0][2], entry_rows[0][16]),
            ("identical", ""),
            "{label}"
        );
        assert_eq!(
            (entry_rows[1][2], entry_rows[1][16]),
            ("identical", flag),
            "{label}"
        );
        assert_eq!(archive_value(&result.stdout, "flagged"), "1", "{label}");
        assert_eq!(
            status_counts(&result.stdout),
            expected_archive([2, 2, 2, 0, 0, 0, 0]),
            "{label}"
        );
    }
}

#[test]
fn files_identical_no_from_one_byte_inside_a_prefix() {
    let dir = TempDir::new("prefix-byte");
    let entries = [base("a.txt"), base("b.txt")];
    let prefix = pattern(200_000);
    let other = flipped(&prefix, &[100_000]);
    let (bytes_in, bytes_out) = (
        raw_zip(&prefix, &entries, b""),
        raw_zip(&other, &entries, b""),
    );
    assert_eq!(
        bytes_in.len(),
        bytes_out.len(),
        "precondition: equal lengths"
    );
    let differing: Vec<usize> = bytes_in
        .iter()
        .zip(&bytes_out)
        .enumerate()
        .filter(|(_, (x, y))| x != y)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(
        differing,
        [100_000],
        "precondition: one differing byte, inside the prefix, in the second 64 KiB buffer"
    );
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    fs::write(&input, &bytes_in).expect("write");
    fs::write(&output, &bytes_out).expect("write");

    let result = run(&input, &output);
    assert_eq!(result.status, 0);
    assert_eq!(result.stderr, "");
    let entry_rows = rows(&result.stdout, "entry");
    assert_eq!(entry_rows.len(), 2);
    for row in &entry_rows {
        assert_eq!((row[2], row[16]), ("identical", ""));
    }
    assert_eq!(archive_value(&result.stdout, "flagged"), "0");
    assert_eq!(archive_value(&result.stdout, "prefix_in"), "200000");
    assert_eq!(archive_value(&result.stdout, "prefix_out"), "200000");
    assert_eq!(archive_value(&result.stdout, "files_identical"), "no");
}

/// The content sentinel string, built from pieces so that the whole string
/// does not appear in the source.
fn sentinel_text() -> String {
    ["zq", "-sent", "inel-", "t1p4"].concat()
}

/// Sentinel bytes placed in every sentinel entry's content.
const SENTINEL_BYTES: [u8; 6] = [0xfe, 0x01, 0x80, 0xff, 0x00, 0x7f];

/// The sentinel entry name: valid UTF-8, every character before `.bin` one
/// that the escaping prints as `\xNN`.
const SENTINEL_NAME: &str = "\u{1}\u{fe}\u{7f}\u{80}\u{2}.bin";

/// The escaped form of `SENTINEL_NAME`, written out independently of the
/// tool's escaping.
const SENTINEL_ESCAPED: &str = "\\x01\\xc3\\xbe\\x7f\\xc2\\x80\\x02.bin";

/// A name of the same byte length as `SENTINEL_NAME`, patched into it for
/// the duplicate-name case.
const SAME_LENGTH_NAME: &str = "other-a.bin";

/// Fictional lines, each carrying the sentinel string and the sentinel bytes.
fn sentinel_content(lines: usize) -> Vec<u8> {
    let mut data = Vec::new();
    for i in 0..lines {
        data.extend_from_slice(format!("fictional row {i} {} ", sentinel_text()).as_bytes());
        data.extend_from_slice(&SENTINEL_BYTES);
        data.push(b'\n');
    }
    data
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    occurrences(haystack, needle) > 0
}

fn occurrences(haystack: &[u8], needle: &[u8]) -> usize {
    haystack
        .windows(needle.len())
        .filter(|w| *w == needle)
        .count()
}

/// Runs the comparison with byte buffers, so that a leaked non-UTF-8 byte is
/// seen as a byte rather than failing a UTF-8 conversion.
fn capture(input: &Path, output: &Path) -> (u8, Vec<u8>, Vec<u8>) {
    let (mut stdout, mut stderr) = (Vec::new(), Vec::new());
    let status = xlsx_diff::run(input, output, &mut stdout, &mut stderr);
    (status, stdout, stderr)
}

/// Checks one captured run. The sentinel string, the sentinel bytes and the
/// raw sentinel name never appear in stdout or stderr. The escaped sentinel
/// name appears in stdout only as the whole of column 2 of an `entry` row,
/// and in stderr at most once, in a single error line. Returns the number of
/// `entry` rows that name the sentinel entry. The messages never print the
/// captured output.
fn assert_no_leak(label: &str, stdout: &[u8], stderr: &[u8]) -> usize {
    let text = sentinel_text();
    for (stream, bytes) in [("stdout", stdout), ("stderr", stderr)] {
        assert!(
            !contains(bytes, text.as_bytes()),
            "{label}: the sentinel string appears in {stream}"
        );
        assert!(
            !contains(bytes, &SENTINEL_BYTES),
            "{label}: the sentinel bytes appear in {stream}"
        );
        assert!(
            !contains(bytes, SENTINEL_NAME.as_bytes()),
            "{label}: the raw sentinel name appears in {stream}"
        );
    }
    let mut named_rows = 0;
    for line in stdout.split(|&b| b == b'\n') {
        let fields: Vec<&[u8]> = line.split(|&b| b == b'\t').collect();
        for (k, field) in fields.iter().enumerate() {
            if contains(field, SENTINEL_ESCAPED.as_bytes()) {
                assert!(
                    k == 1 && fields[0] == b"entry" && *field == SENTINEL_ESCAPED.as_bytes(),
                    "{label}: the escaped sentinel name appears outside column 2 of an entry row"
                );
                named_rows += 1;
            }
        }
    }
    let error_lines = stderr
        .split(|&b| b == b'\n')
        .filter(|l| !l.is_empty())
        .count();
    assert!(error_lines <= 1, "{label}: more than one stderr line");
    assert!(
        occurrences(stderr, SENTINEL_ESCAPED.as_bytes()) <= 1,
        "{label}: the escaped sentinel name appears more than once in stderr"
    );
    named_rows
}

/// The status column of every `entry` row, in order.
fn statuses(stdout: &[u8]) -> Vec<String> {
    stdout
        .split(|&b| b == b'\n')
        .filter(|l| l.starts_with(b"entry\t"))
        .map(|l| {
            let fields: Vec<&[u8]> = l.split(|&b| b == b'\t').collect();
            String::from_utf8_lossy(fields[2]).into_owned()
        })
        .collect()
}

/// An extra field with a private header id holding the sentinel string.
fn sentinel_extra() -> Vec<u8> {
    let body = sentinel_text().into_bytes();
    let mut field = 0x6666u16.to_le_bytes().to_vec();
    field.extend_from_slice(&u16::try_from(body.len()).expect("short").to_le_bytes());
    field.extend_from_slice(&body);
    field
}

/// One comparison that ends with exit 0 or 1: a label, the input and output
/// archive bytes, the exit status, the statuses in row order, and the number
/// of `entry` rows naming the sentinel entry.
type Comparison<'a> = (&'a str, Vec<u8>, Vec<u8>, u8, Vec<&'a str>, usize);

#[test]
fn no_sentinel_in_any_output() {
    let dir = TempDir::new("sentinel");
    let data = sentinel_content(40);
    let changed_data = flipped(&data, &[3]);
    let level = |name: &str, level: i64| Spec {
        level: Some(level),
        ..deflated(name, &data)
    };

    let mut compared: Vec<Comparison> = Vec::new();
    let write = |entries: &[Spec]| -> Vec<u8> {
        let path = dir.path("scratch.zip");
        write_zip(&path, entries);
        fs::read(&path).expect("read")
    };
    compared.push((
        "every status",
        write(&[
            deflated("keep.txt", &data),
            level("recompressed.txt", 1),
            deflated("changed.txt", &data),
            deflated(SENTINEL_NAME, &data),
            deflated("dropped.txt", &data),
        ]),
        write(&[
            deflated("keep.txt", &data),
            level("recompressed.txt", 9),
            deflated("changed.txt", &changed_data),
            deflated(SENTINEL_NAME, &data),
            deflated("added.txt", &data),
        ]),
        1,
        vec![
            "identical",
            "recompressed",
            "changed",
            "identical",
            "dropped",
            "added",
        ],
        1,
    ));
    compared.push((
        "sentinel entry identical",
        write(&[deflated(SENTINEL_NAME, &data)]),
        write(&[deflated(SENTINEL_NAME, &data)]),
        0,
        vec!["identical"],
        1,
    ));
    compared.push((
        "sentinel entry recompressed",
        write(&[level(SENTINEL_NAME, 1)]),
        write(&[level(SENTINEL_NAME, 9)]),
        1,
        vec!["recompressed"],
        1,
    ));
    compared.push((
        "sentinel entry changed",
        write(&[deflated(SENTINEL_NAME, &data)]),
        write(&[deflated(SENTINEL_NAME, &changed_data)]),
        1,
        vec!["changed"],
        1,
    ));
    compared.push((
        "sentinel entry changed length",
        write(&[deflated(SENTINEL_NAME, &data)]),
        write(&[deflated(SENTINEL_NAME, &data[..data.len() / 2])]),
        1,
        vec!["changed"],
        1,
    ));
    compared.push((
        "sentinel entry dropped",
        write(&[deflated(SENTINEL_NAME, &data), deflated("keep.txt", &data)]),
        write(&[deflated("keep.txt", &data)]),
        1,
        vec!["dropped", "identical"],
        1,
    ));
    compared.push((
        "sentinel entry added",
        write(&[deflated("keep.txt", &data)]),
        write(&[deflated("keep.txt", &data), deflated(SENTINEL_NAME, &data)]),
        1,
        vec!["identical", "added"],
        1,
    ));
    let flagged = |comment: &str| RawEntry {
        comment: format!("{} {comment}", sentinel_text()).into_bytes(),
        local_extra: sentinel_extra(),
        central_extra: sentinel_extra(),
        ..raw_stored(SENTINEL_NAME, &data)
    };
    let mut other_extra = sentinel_extra();
    other_extra.push(0);
    let len = other_extra.len() - 4;
    other_extra[2..4].copy_from_slice(&u16::try_from(len).expect("short").to_le_bytes());
    compared.push((
        "sentinel comments and extra fields",
        raw_zip(
            &[],
            &[flagged("one")],
            format!("{} archive one", sentinel_text()).as_bytes(),
        ),
        raw_zip(
            &[],
            &[RawEntry {
                central_extra: other_extra,
                ..flagged("two")
            }],
            format!("{} archive two", sentinel_text()).as_bytes(),
        ),
        1,
        vec!["identical"],
        1,
    ));

    for (label, input_bytes, output_bytes, exit, expected, named) in compared {
        let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
        fs::write(&input, &input_bytes).expect("write");
        fs::write(&output, &output_bytes).expect("write");
        let (status, stdout, stderr) = capture(&input, &output);
        assert_eq!(status, exit, "{label}: exit status");
        assert!(stderr.is_empty(), "{label}: stderr is empty");
        assert_eq!(statuses(&stdout), expected, "{label}: statuses");
        assert_eq!(
            assert_no_leak(label, &stdout, &stderr),
            named,
            "{label}: entry rows naming the sentinel entry"
        );
    }

    // Error paths: (label, input, output, the expected error line).
    let plain = write(&[deflated(SENTINEL_NAME, &data)]);
    let mut errors: Vec<(&str, Vec<u8>, Vec<u8>, String)> = Vec::new();

    let encrypted = {
        let mut writer = ZipWriter::new(std::io::Cursor::new(Vec::new()));
        let options = SimpleFileOptions::default()
            .last_modified_time(DateTime::default())
            .with_deprecated_encryption(b"fictional-password")
            .expect("ZipCrypto options");
        writer
            .start_file(SENTINEL_NAME, options)
            .expect("start an entry");
        writer.write_all(&data).expect("write an entry");
        writer.finish().expect("finish the archive").into_inner()
    };
    errors.push((
        "encrypted sentinel entry",
        plain.clone(),
        encrypted,
        format!("error: entry {SENTINEL_ESCAPED} in the output archive is encrypted\n"),
    ));

    let mut bad_crc = plain.clone();
    let crc = crc_of(&bad_crc, SENTINEL_NAME.as_bytes());
    set_crc(&mut bad_crc, SENTINEL_NAME.as_bytes(), crc ^ 0x0000_00ff);
    errors.push((
        "corrupt sentinel entry",
        plain.clone(),
        bad_crc,
        format!(
            "error: entry {SENTINEL_ESCAPED} in the output archive failed to decompress or its CRC-32 does not match\n"
        ),
    ));

    let stored_plain = write(&[stored(SENTINEL_NAME, &data)]);
    errors.push((
        "short sentinel entry",
        stored_plain.clone(),
        cut_entry_tail(&stored_plain, 100),
        format!(
            "error: entry {SENTINEL_ESCAPED} in the output archive ends before its declared compressed size\n"
        ),
    ));

    let mut method = stored_plain.clone();
    set_method(&mut method, SENTINEL_NAME.as_bytes(), 12);
    errors.push((
        "unsupported method on the sentinel entry",
        stored_plain.clone(),
        method,
        format!(
            "error: entry {SENTINEL_ESCAPED} in the output archive uses unsupported compression method 12\n"
        ),
    ));

    let mut duplicate = write(&[
        deflated(SENTINEL_NAME, &data),
        deflated(SAME_LENGTH_NAME, &data),
    ]);
    assert_eq!(
        patch(
            &mut duplicate,
            SAME_LENGTH_NAME.as_bytes(),
            SENTINEL_NAME.as_bytes()
        ),
        2,
        "precondition: local and central name patched"
    );
    errors.push((
        "duplicate sentinel name",
        plain.clone(),
        duplicate,
        format!("error: duplicate entry name in the output archive: {SENTINEL_ESCAPED}\n"),
    ));

    errors.push((
        "output file of sentinel content",
        plain.clone(),
        data.clone(),
        "error: the output file is not a readable zip archive\n".to_string(),
    ));

    for (label, input_bytes, output_bytes, line) in errors {
        let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
        fs::write(&input, &input_bytes).expect("write");
        fs::write(&output, &output_bytes).expect("write");
        let (status, stdout, stderr) = capture(&input, &output);
        assert_eq!(status, 2, "{label}: exit status");
        assert!(stderr == line.as_bytes(), "{label}: the error line");
        assert_no_leak(label, &stdout, &stderr);
    }

    // Paths holding the sentinel string are never printed.
    let good = dir.path("good.zip");
    fs::write(&good, &plain).expect("write");
    let missing = dir.path(&format!("missing-{}.zip", sentinel_text()));
    let (status, stdout, stderr) = capture(&missing, &good);
    assert_eq!(status, 2, "missing file: exit status");
    assert!(
        stderr.starts_with(b"error: cannot read the input file: "),
        "missing file: the error line"
    );
    assert_no_leak("missing file named with the sentinel", &stdout, &stderr);

    let not_zip = dir.path(&format!("not-zip-{}.zip", sentinel_text()));
    fs::write(&not_zip, &data).expect("write");
    let (status, stdout, stderr) = capture(&not_zip, &good);
    assert_eq!(status, 2, "not a zip: exit status");
    assert!(
        stderr == b"error: the input file is not a readable zip archive\n",
        "not a zip: the error line"
    );
    assert_no_leak("non-zip file named with the sentinel", &stdout, &stderr);
}

/// Size of the zip64 entry: 4 GiB and one byte.
const ZIP64_SIZE: u64 = 4 * 1024 * 1024 * 1024 + 1;

/// Writes one deflated entry of `ZIP64_SIZE` zero bytes in 1 MiB chunks,
/// with the byte at `one_at` set to 1 when given. The entry is never held
/// in memory whole.
fn write_zeros_entry(path: &Path, one_at: Option<u64>) {
    const WRITE_CHUNK: usize = 1024 * 1024;
    let mut writer = ZipWriter::new(fs::File::create(path).expect("create the archive"));
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .large_file(true)
        .last_modified_time(DateTime::default());
    writer
        .start_file("zeros.bin", options)
        .expect("start an entry");
    let mut chunk = vec![0u8; WRITE_CHUNK];
    let mut written = 0u64;
    while written < ZIP64_SIZE {
        let len = (ZIP64_SIZE - written).min(WRITE_CHUNK as u64) as usize;
        chunk[..len].fill(0);
        if let Some(at) = one_at
            && at >= written
            && at < written + len as u64
        {
            chunk[(at - written) as usize] = 1;
        }
        writer.write_all(&chunk[..len]).expect("write a chunk");
        written += len as u64;
    }
    writer.finish().expect("finish the archive");
}

#[test]
#[ignore = "writes and compares a 4 GiB + 1 byte entry; run with --ignored zip64"]
fn zip64_entry_over_4_gib() {
    let dir = TempDir::new("zip64");
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zeros_entry(&input, None);
    write_zeros_entry(&output, Some(4 * 1024 * 1024 * 1024));

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let entry_rows = rows(&result.stdout, "entry");
    assert_eq!(entry_rows.len(), 1);
    let row = &entry_rows[0];
    assert_eq!(row[2], "changed");
    assert_eq!((row[7], row[8]), ("4294967297", "4294967297"), "sizes");
    assert_eq!(
        (row[13], row[14], row[15]),
        ("4294967296", "1", "4294967296"),
        "columns 14 to 16"
    );
}
