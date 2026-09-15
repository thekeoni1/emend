//! T1 P6 tests (`specs/0004-xlsx-diff.md`): closing the exit-0 guarantee.
//! `attrs` covers raw central method, CRC-32 and size bytes that decode to
//! equal values, and `order_preserved` covers the order of the local entries
//! in the file. Every archive is written byte by byte from fictional content
//! in a temporary directory; nothing is committed.

mod common;

use std::fs;

use common::*;

/// A stored entry of fictional text.
fn base(name: &str) -> RawEntry {
    raw_stored(name, &text(12))
}

/// A zip64 extended information extra field (header id 0x0001) holding the
/// uncompressed and then the compressed size.
fn zip64_extra(uncompressed: u64, compressed: u64) -> Vec<u8> {
    let mut field = 0x0001u16.to_le_bytes().to_vec();
    field.extend_from_slice(&16u16.to_le_bytes());
    field.extend_from_slice(&uncompressed.to_le_bytes());
    field.extend_from_slice(&compressed.to_le_bytes());
    field
}

/// A stored entry whose central record carries a zip64 extra field with its
/// sizes, and the plain sizes or 0xffffffff placeholders in the size fields.
fn zip64_entry(name: &str, placeholder: bool) -> RawEntry {
    let entry = base(name);
    let len = entry.data.len() as u64;
    RawEntry {
        central_extra: zip64_extra(len, len),
        central_size_placeholder: placeholder,
        ..entry
    }
}

/// The (status, flags) pair of every `entry` row.
fn status_and_flags(stdout: &str) -> Vec<(String, String)> {
    rows(stdout, "entry")
        .iter()
        .map(|r| (r[2].to_string(), r[16].to_string()))
        .collect()
}

fn pairs(expected: &[(&str, &str)]) -> Vec<(String, String)> {
    expected
        .iter()
        .map(|(s, f)| (s.to_string(), f.to_string()))
        .collect()
}

#[test]
fn zip64_placeholder_against_plain_sizes() {
    let dir = TempDir::new("p6-zip64");
    let (plain, placeholder, placeholder_copy) = (
        dir.path("plain.zip"),
        dir.path("placeholder.zip"),
        dir.path("placeholder-copy.zip"),
    );
    fs::write(
        &plain,
        raw_zip(&[], &[base("a.txt"), zip64_entry("z.txt", false)], b""),
    )
    .expect("write");
    fs::write(
        &placeholder,
        raw_zip(&[], &[base("a.txt"), zip64_entry("z.txt", true)], b""),
    )
    .expect("write");
    assert_eq!(
        meta(&plain),
        meta(&placeholder),
        "precondition: the zip crate decodes equal methods, sizes and CRC-32 values"
    );
    assert_ne!(
        fs::read(&plain).expect("read"),
        fs::read(&placeholder).expect("read"),
        "precondition: the archives differ in bytes"
    );

    let result = run(&plain, &placeholder);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    assert_eq!(
        status_and_flags(&result.stdout),
        pairs(&[("identical", ""), ("identical", "attrs")])
    );
    assert_eq!(archive_value(&result.stdout, "flagged"), "1");
    assert_eq!(archive_value(&result.stdout, "order_preserved"), "yes");

    // A placeholder in one size field alone also gives `attrs` alone: for a
    // stored entry both sizes are equal, so the decoded value is the same
    // whichever field the zip64 extra field fills.
    for (label, compressed_only) in [("compressed size", true), ("uncompressed size", false)] {
        let single = dir.path(&format!("single-{compressed_only}.zip"));
        let entry = zip64_entry("z.txt", false);
        let entry = RawEntry {
            central_compressed_placeholder: compressed_only,
            central_uncompressed_placeholder: !compressed_only,
            ..entry
        };
        fs::write(&single, raw_zip(&[], &[base("a.txt"), entry], b"")).expect("write");
        assert_eq!(meta(&plain), meta(&single), "precondition for {label}");
        let result = run(&plain, &single);
        assert_eq!(result.status, 1, "{label}");
        assert_eq!(result.stderr, "", "{label}");
        assert_eq!(
            status_and_flags(&result.stdout),
            pairs(&[("identical", ""), ("identical", "attrs")]),
            "{label}"
        );
    }

    // The same placeholders on both sides give exit 0.
    fs::write(
        &placeholder_copy,
        raw_zip(&[], &[base("a.txt"), zip64_entry("z.txt", true)], b""),
    )
    .expect("write");
    let result = run(&placeholder, &placeholder_copy);
    assert_eq!(result.status, 0);
    assert_eq!(result.stderr, "");
    assert_eq!(
        status_and_flags(&result.stdout),
        pairs(&[("identical", ""), ("identical", "")])
    );
}

#[test]
fn local_entries_permuted_with_the_central_order_kept() {
    let dir = TempDir::new("p6-permuted");
    let entries = [
        base("a.txt"),
        raw_stored("b.txt", &text(20)),
        raw_stored("c.txt", &text(30)),
    ];
    assert_eq!(
        raw_zip_with_local_order(&[], &entries, &[0, 1, 2], b""),
        raw_zip(&[], &entries, b""),
        "precondition: the natural local order writes the same bytes as raw_zip"
    );
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    fs::write(&input, raw_zip(&[], &entries, b"")).expect("write");
    fs::write(
        &output,
        raw_zip_with_local_order(&[], &entries, &[2, 0, 1], b""),
    )
    .expect("write");
    assert_eq!(meta(&input), meta(&output), "precondition");

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let summary: Vec<(&str, &str, &str, &str)> = rows(&result.stdout, "entry")
        .iter()
        .map(|r| (r[1], r[2], r[4], r[16]))
        .collect();
    assert_eq!(
        summary,
        [
            ("a.txt", "identical", "0", ""),
            ("b.txt", "identical", "1", ""),
            ("c.txt", "identical", "2", "")
        ],
        "the central order is kept"
    );
    assert_eq!(archive_value(&result.stdout, "flagged"), "0");
    assert_eq!(archive_value(&result.stdout, "order_preserved"), "no");
    assert_eq!(archive_value(&result.stdout, "files_identical"), "no");
    assert_eq!(
        status_counts(&result.stdout),
        expected_archive([3, 3, 3, 0, 0, 0, 0])
    );

    // The same permutation on both sides preserves the order.
    let same = dir.path("same.zip");
    fs::write(
        &same,
        raw_zip_with_local_order(&[], &entries, &[2, 0, 1], b""),
    )
    .expect("write");
    let result = run(&output, &same);
    assert_eq!(result.status, 0);
    assert_eq!(archive_value(&result.stdout, "order_preserved"), "yes");
}

#[test]
fn declared_size_running_over_the_next_header_alike_on_both_sides() {
    let dir = TempDir::new("p6-overrun");
    let first = base("a.txt");
    // The declared size takes in the 40 bytes after the entry's data: the next
    // entry's whole local header (30 fixed bytes and the 5-byte name, with no
    // extra field) and the first 5 bytes of its data.
    let overrun = RawEntry {
        declared_compressed: Some(first.raw.len() + 40),
        ..first
    };
    let entries = [overrun, base("b.txt"), base("c.txt")];
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    fs::write(&input, raw_zip(&[], &entries, b"")).expect("write");
    fs::write(&output, raw_zip(&[], &entries, b"")).expect("write");

    let result = run(&input, &output);
    assert_eq!(result.status, 0);
    assert_eq!(result.stderr, "");
    assert_eq!(
        status_and_flags(&result.stdout),
        pairs(&[("identical", ""), ("identical", ""), ("identical", "")])
    );
    let expected_size = (text(12).len() + 40).to_string();
    let entry_rows = rows(&result.stdout, "entry");
    assert_eq!(
        (entry_rows[0][9], entry_rows[0][10]),
        (expected_size.as_str(), expected_size.as_str()),
        "the declared compressed size is the one compared"
    );
    assert_eq!(archive_value(&result.stdout, "order_preserved"), "yes");
}
