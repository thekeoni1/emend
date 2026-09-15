//! T3 P1 tests (`specs/0006-passthrough.md`): what the zip crate's raw copy
//! preserves. Each test writes an archive from fictional content, copies it
//! with the passthrough binary and compares the pair with `xlsx_diff::run`.
//! The assertions state what the copy gives, not that it is identical: every
//! non-`identical` status or flag here is a finding in the P1 report. Header
//! fields are also read directly from the bytes, independently of both tools,
//! to show which field the flag comes from.

mod common;

use std::fs;
use std::path::{Path, PathBuf};

use common::*;
use zip::CompressionMethod;

const NAMES: [&str; 3] = [
    "[Content_Types].xml",
    "xl/workbook.xml",
    "xl/worksheets/sheet1.xml",
];

/// The (name, status, flags) triple of every `entry` row.
fn entries(stdout: &str) -> Vec<(String, String, String)> {
    rows(stdout, "entry")
        .iter()
        .map(|r| (r[1].to_string(), r[2].to_string(), r[16].to_string()))
        .collect()
}

fn expect(expected: &[(&str, &str, &str)]) -> Vec<(String, String, String)> {
    expected
        .iter()
        .map(|(n, s, f)| (n.to_string(), s.to_string(), f.to_string()))
        .collect()
}

/// Copies `input` to `out.zip` beside it, checks that no temporary file is
/// left, and compares the pair.
fn copy_and_compare(dir: &TempDir, input: &Path) -> (Outcome, PathBuf) {
    let output = dir.path("out.zip");
    copy_ok(input, &output);
    assert_eq!(
        dir.names(),
        ["in.zip", "out.zip"],
        "no temporary file is left"
    );
    let outcome = compare(input, &output);
    assert_eq!(outcome.stderr, "", "xlsx-diff stderr");
    (outcome, output)
}

fn le16(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn le32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

/// Offsets of every occurrence of `signature`. The fixtures hold fictional
/// text, which never contains a zip signature.
fn find_all(bytes: &[u8], signature: [u8; 4]) -> Vec<usize> {
    (0..bytes.len().saturating_sub(3))
        .filter(|&i| bytes[i..i + 4] == signature)
        .collect()
}

/// Fields of a central directory record, apart from the name, the extra
/// field bytes, the comment bytes and the local header offset.
#[derive(Debug, PartialEq)]
struct Central {
    made_by: u16,
    needed: u16,
    flags: u16,
    crc: u32,
    compressed: u32,
    uncompressed: u32,
    extra_len: u16,
    comment_len: u16,
    external: u32,
}

fn centrals(path: &Path) -> Vec<Central> {
    let bytes = fs::read(path).expect("read the archive");
    find_all(&bytes, [0x50, 0x4b, 0x01, 0x02])
        .into_iter()
        .map(|at| Central {
            made_by: le16(&bytes, at + 4),
            needed: le16(&bytes, at + 6),
            flags: le16(&bytes, at + 8),
            crc: le32(&bytes, at + 16),
            compressed: le32(&bytes, at + 20),
            uncompressed: le32(&bytes, at + 24),
            extra_len: le16(&bytes, at + 30),
            comment_len: le16(&bytes, at + 32),
            external: le32(&bytes, at + 38),
        })
        .collect()
}

/// Version needed, flags, CRC-32, compressed size, uncompressed size and
/// extra field length of every local header.
fn locals(path: &Path) -> Vec<[u32; 6]> {
    let bytes = fs::read(path).expect("read the archive");
    find_all(&bytes, [0x50, 0x4b, 0x03, 0x04])
        .into_iter()
        .map(|at| {
            [
                u32::from(le16(&bytes, at + 4)),
                u32::from(le16(&bytes, at + 6)),
                le32(&bytes, at + 14),
                le32(&bytes, at + 18),
                le32(&bytes, at + 22),
                u32::from(le16(&bytes, at + 28)),
            ]
        })
        .collect()
}

fn three_entries(path: &Path, method: CompressionMethod) {
    zip_with(path, |w| {
        for (k, name) in NAMES.iter().enumerate() {
            add_file(w, name, &text(10 * (k + 1)), options(method));
        }
    });
}

/// A deflated entry as the zip crate writes one on Unix: version made by
/// 3.20 on Unix, version needed 2.0, mode 0644. On this baseline a raw
/// fixture's flags come only from the feature it adds.
fn unix_deflated(name: &str, data: &[u8]) -> RawEntry {
    RawEntry {
        version_made_by: 0x0314,
        external_attributes: 0o100644 << 16,
        ..raw_deflated(name, data, &stored_block(data))
    }
}

fn zip64_extra(uncompressed: u64, compressed: u64) -> Vec<u8> {
    let mut field = 0x0001u16.to_le_bytes().to_vec();
    field.extend_from_slice(&16u16.to_le_bytes());
    field.extend_from_slice(&uncompressed.to_le_bytes());
    field.extend_from_slice(&compressed.to_le_bytes());
    field
}

#[test]
fn stored_entries_are_copied_byte_identical() {
    let dir = TempDir::new("stored");
    let input = dir.path("in.zip");
    three_entries(&input, CompressionMethod::Stored);
    let (outcome, _) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 0);
    assert_eq!(
        entries(&outcome.stdout),
        expect(&[
            (NAMES[0], "identical", ""),
            (NAMES[1], "identical", ""),
            (NAMES[2], "identical", "")
        ])
    );
    assert_eq!(archive_value(&outcome.stdout, "files_identical"), "yes");
}

#[test]
fn deflated_entries_are_copied_byte_identical() {
    let dir = TempDir::new("deflated");
    let input = dir.path("in.zip");
    three_entries(&input, CompressionMethod::Deflated);
    let (outcome, _) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 0);
    assert_eq!(
        entries(&outcome.stdout),
        expect(&[
            (NAMES[0], "identical", ""),
            (NAMES[1], "identical", ""),
            (NAMES[2], "identical", "")
        ])
    );
    assert_eq!(archive_value(&outcome.stdout, "files_identical"), "yes");
}

#[test]
fn directory_entry_loses_its_directory_mode() {
    let dir = TempDir::new("directory");
    let input = dir.path("in.zip");
    zip_with(&input, |w| {
        w.add_directory("xl/media/", options(CompressionMethod::Stored))
            .expect("add a directory");
        add_file(
            w,
            "xl/workbook.xml",
            &text(5),
            options(CompressionMethod::Deflated),
        );
    });
    let (outcome, output) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 1);
    assert_eq!(
        entries(&outcome.stdout),
        expect(&[
            ("xl/media/", "identical", "attrs"),
            ("xl/workbook.xml", "identical", "")
        ])
    );
    assert_eq!(archive_value(&outcome.stdout, "flagged"), "1");
    assert_eq!(archive_value(&outcome.stdout, "files_identical"), "no");
    let (a, b) = (centrals(&input), centrals(&output));
    assert_eq!(
        (a[0].made_by, a[0].needed, a[0].external),
        (0x0314, 20, 0o40755 << 16),
        "input: a directory, version needed 2.0"
    );
    assert_eq!(
        (b[0].made_by, b[0].needed, b[0].external),
        (0x030a, 10, 0o100755 << 16),
        "output: a regular file mode, version needed 1.0"
    );
    assert_eq!(a[1], b[1], "the file entry's central record");
}

#[test]
fn unknown_extra_field_is_dropped_and_entry_comment_kept() {
    let dir = TempDir::new("extra");
    let input = dir.path("in.zip");
    let extra = vec![0xfe, 0xca, 4, 0, 1, 2, 3, 4];
    let entry = RawEntry {
        local_extra: extra.clone(),
        central_extra: extra,
        comment: b"fictional entry comment".to_vec(),
        ..unix_deflated(NAMES[1], &text(5))
    };
    let first = unix_deflated(NAMES[0], &text(3));
    fs::write(&input, raw_zip(&[], &[first, entry], b"")).expect("write");
    let (outcome, output) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 1);
    assert_eq!(
        entries(&outcome.stdout),
        expect(&[
            (NAMES[0], "identical", ""),
            (NAMES[1], "identical", "extra")
        ])
    );
    let (a, b) = (centrals(&input), centrals(&output));
    assert_eq!((a[1].extra_len, a[1].comment_len), (8, 23), "input");
    assert_eq!((b[1].extra_len, b[1].comment_len), (0, 23), "output");
    assert_eq!((locals(&input)[1][5], locals(&output)[1][5]), (8, 0));
    let bytes = |key| {
        archive_value(&outcome.stdout, key)
            .parse::<u64>()
            .expect("a count")
    };
    assert_eq!(bytes("bytes_in") - bytes("bytes_out"), 16);
}

#[test]
fn archive_comment_is_kept() {
    let dir = TempDir::new("comment");
    let input = dir.path("in.zip");
    zip_with(&input, |w| {
        add_file(
            w,
            "xl/workbook.xml",
            &text(5),
            options(CompressionMethod::Deflated),
        );
        w.set_comment("fictional archive comment")
            .expect("set the comment");
    });
    let (outcome, _) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 0);
    assert_eq!(archive_value(&outcome.stdout, "comment_equal"), "yes");
    assert_eq!(archive_value(&outcome.stdout, "files_identical"), "yes");
}

#[test]
fn leading_junk_bytes_are_dropped() {
    let dir = TempDir::new("junk");
    let input = dir.path("in.zip");
    let prefix = b"fictional leading junk bytes\n";
    let entries_in = [
        unix_deflated(NAMES[0], &text(3)),
        unix_deflated(NAMES[1], &text(5)),
    ];
    fs::write(&input, raw_zip(prefix, &entries_in, b"")).expect("write");
    let (outcome, _) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 0);
    assert_eq!(
        entries(&outcome.stdout),
        expect(&[(NAMES[0], "identical", ""), (NAMES[1], "identical", "")])
    );
    assert_eq!(
        archive_value(&outcome.stdout, "prefix_in"),
        prefix.len().to_string()
    );
    assert_eq!(archive_value(&outcome.stdout, "prefix_out"), "0");
    assert_eq!(archive_value(&outcome.stdout, "files_identical"), "no");
}

#[test]
fn data_descriptor_is_folded_into_the_local_header() {
    let dir = TempDir::new("descriptor");
    let input = dir.path("in.zip");
    let data = text(5);
    let entry = RawEntry {
        flags: 0x0008,
        ..unix_deflated(NAMES[1], &data)
    };
    let first = unix_deflated(NAMES[0], &text(3));
    fs::write(&input, raw_zip(&[], &[first, entry], b"")).expect("write");
    let (outcome, output) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 1);
    assert_eq!(
        entries(&outcome.stdout),
        expect(&[
            (NAMES[0], "identical", ""),
            (NAMES[1], "identical", "gpflags,descriptor,local")
        ])
    );
    let (a, b) = (locals(&input), locals(&output));
    assert_eq!(
        (a[1][1], a[1][2], a[1][3]),
        (0x0008, 0, 0),
        "input: bit 3 and zeros"
    );
    assert_eq!(
        (b[1][1], b[1][2], b[1][3]),
        (0, crc32(&data), stored_block(&data).len() as u32),
        "output: no bit 3, the CRC-32 and size in the local header"
    );
    assert_eq!(centrals(&output)[1].flags, 0);
    let descriptor = [0x50, 0x4b, 0x07, 0x08];
    assert_eq!(
        find_all(&fs::read(&input).expect("read"), descriptor).len(),
        1
    );
    assert_eq!(
        find_all(&fs::read(&output).expect("read"), descriptor).len(),
        0
    );
}

#[test]
fn zip64_extra_fields_of_a_small_entry_are_dropped() {
    let dir = TempDir::new("zip64-option");
    let input = dir.path("in.zip");
    zip_with(&input, |w| {
        add_file(
            w,
            NAMES[1],
            &text(5),
            options(CompressionMethod::Deflated).large_file(true),
        );
    });
    let (outcome, output) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 1);
    assert_eq!(
        entries(&outcome.stdout),
        expect(&[(NAMES[1], "identical", "extra,attrs")])
    );
    let (a, b) = (centrals(&input), centrals(&output));
    assert_eq!(
        (a[0].made_by, a[0].needed, a[0].compressed, a[0].extra_len),
        (0x032d, 45, 0xffff_ffff, 20),
        "input: zip64 placeholders and extra field"
    );
    assert_eq!(
        (b[0].made_by, b[0].needed, b[0].extra_len),
        (0x0314, 20, 0),
        "output: plain sizes, no extra field"
    );
    assert_eq!(
        (b[0].compressed.to_string(), b[0].uncompressed.to_string()),
        (
            archive_value(&outcome.stdout, "compressed_out"),
            archive_value(&outcome.stdout, "uncompressed_out")
        ),
        "output: the plain sizes in the central record"
    );
    assert_eq!(locals(&input)[0][5], 20, "input: a local zip64 extra field");
    assert_eq!(locals(&output)[0][5], 0, "output: no local extra field");
}

#[test]
fn zip64_placeholders_written_byte_by_byte_are_dropped() {
    let dir = TempDir::new("zip64-raw");
    let input = dir.path("in.zip");
    let data = text(5);
    let base = unix_deflated(NAMES[1], &data);
    let entry = RawEntry {
        central_extra: zip64_extra(data.len() as u64, base.raw.len() as u64),
        central_size_placeholder: true,
        ..base
    };
    let first = unix_deflated(NAMES[0], &text(3));
    fs::write(&input, raw_zip(&[], &[first, entry], b"")).expect("write");
    let (outcome, output) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 1);
    assert_eq!(
        entries(&outcome.stdout),
        expect(&[
            (NAMES[0], "identical", ""),
            (NAMES[1], "identical", "extra,attrs")
        ])
    );
    let b = centrals(&output);
    assert_eq!(
        (b[1].compressed, b[1].uncompressed, b[1].extra_len),
        (stored_block(&data).len() as u32, data.len() as u32, 0)
    );
}

#[test]
fn ms_dos_made_by_and_version_needed_are_rewritten() {
    let dir = TempDir::new("ms-dos");
    let input = dir.path("in.zip");
    fs::write(&input, raw_zip(&[], &[raw_stored(NAMES[1], &text(5))], b"")).expect("write");
    let (outcome, output) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 1);
    assert_eq!(
        entries(&outcome.stdout),
        expect(&[(NAMES[1], "identical", "attrs")])
    );
    let (a, b) = (centrals(&input), centrals(&output));
    assert_eq!(
        (a[0].made_by, a[0].needed, a[0].external),
        (0x0014, 20, 0),
        "input: MS-DOS, version needed 2.0, no attributes"
    );
    assert_eq!(
        (b[0].made_by, b[0].needed, b[0].external),
        (0x030a, 10, 0o100644 << 16),
        "output: Unix, version needed 1.0, mode 0644"
    );
}

#[test]
fn two_hundred_entries_are_copied_byte_identical() {
    let dir = TempDir::new("many");
    let input = dir.path("in.zip");
    zip_with(&input, |w| {
        for i in 0..200 {
            add_file(
                w,
                &format!("xl/worksheets/sheet{i}.xml"),
                &text(2),
                options(CompressionMethod::Deflated),
            );
        }
    });
    let (outcome, _) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 0);
    assert_eq!(archive_value(&outcome.stdout, "entries_out"), "200");
    assert_eq!(archive_value(&outcome.stdout, "identical"), "200");
    assert_eq!(archive_value(&outcome.stdout, "files_identical"), "yes");
}

#[test]
fn empty_archive_is_copied_byte_identical() {
    let dir = TempDir::new("empty");
    let input = dir.path("in.zip");
    zip_with(&input, |_| {});
    let (outcome, _) = copy_and_compare(&dir, &input);
    assert_eq!(outcome.status, 0);
    assert_eq!(archive_value(&outcome.stdout, "entries_in"), "0");
    assert_eq!(archive_value(&outcome.stdout, "entries_out"), "0");
    assert_eq!(archive_value(&outcome.stdout, "files_identical"), "yes");
}

#[test]
fn duplicate_names_keep_only_the_last_entry() {
    let dir = TempDir::new("duplicate");
    let input = dir.path("in.zip");
    let (first, second) = (text(3), text(4));
    let archive = raw_zip(
        &[],
        &[
            unix_deflated("a.xml", &first),
            unix_deflated("a.xml", &second),
        ],
        b"",
    );
    fs::write(&input, archive).expect("write");
    let output = dir.path("out.zip");
    // The copy succeeds: the zip crate's reader lists one entry per name.
    copy_ok(&input, &output);
    assert_eq!(
        dir.names(),
        ["in.zip", "out.zip"],
        "no temporary file is left"
    );
    assert_eq!(
        centrals(&input).len(),
        2,
        "two central records in the input"
    );
    let b = centrals(&output);
    assert_eq!(b.len(), 1, "one central record in the output");
    assert_eq!(
        b[0].crc,
        crc32(&second),
        "the last entry of that name is kept"
    );
    // The harness refuses the input for its duplicate name.
    let outcome = compare(&input, &output);
    assert_eq!(outcome.status, 2);
    assert_eq!(outcome.stdout, "");
    assert_eq!(
        outcome.stderr,
        "error: duplicate entry name in the input archive: a.xml\n"
    );
}
