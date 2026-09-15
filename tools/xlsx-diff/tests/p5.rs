//! T1 P5 tests (`specs/0004-xlsx-diff.md`): flag completeness. `local` is
//! narrowed to local-only differences, `attrs` covers internal attributes
//! and disk number start, and `descriptor` covers the bytes after each
//! entry's data. Every archive is written byte by byte from fictional
//! content in a temporary directory; nothing is committed.

mod common;

use std::fs;

use common::*;

/// A stored entry of fictional text used as the base for the flag cases.
fn base(name: &str) -> RawEntry {
    raw_stored(name, &text(12))
}

/// Compares archives written from `input` and `output` and returns the
/// outcome, after checking that the zip crate sees the same method, sizes
/// and CRC-32 values in both.
fn compare_raw(dir: &TempDir, label: &str, input: &[RawEntry], output: &[RawEntry]) -> Outcome {
    let (a, b) = (
        dir.path(&format!("{label}-in.zip")),
        dir.path(&format!("{label}-out.zip")),
    );
    fs::write(&a, raw_zip(&[], input, b"")).expect("write");
    fs::write(&b, raw_zip(&[], output, b"")).expect("write");
    assert_eq!(meta(&a), meta(&b), "precondition for {label}");
    run(&a, &b)
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
fn narrowed_and_extended_flags_each_alone() {
    let cases: Vec<(&str, RawEntry, &str)> = vec![
        (
            "local-only time",
            RawEntry {
                local_time: Some(0x6000),
                ..base("f.txt")
            },
            "local",
        ),
        (
            "local-only flags",
            RawEntry {
                local_flags: Some(0x0800),
                ..base("f.txt")
            },
            "local",
        ),
        (
            "internal attributes",
            RawEntry {
                internal_attributes: 1,
                ..base("f.txt")
            },
            "attrs",
        ),
        (
            "disk number start",
            RawEntry {
                disk_start: 1,
                ..base("f.txt")
            },
            "attrs",
        ),
        (
            "time in both headers",
            RawEntry {
                time: 0x6000,
                ..base("f.txt")
            },
            "mtime",
        ),
        (
            "flags in both headers",
            RawEntry {
                flags: 0x0800,
                ..base("f.txt")
            },
            "gpflags",
        ),
    ];
    let dir = TempDir::new("p5-flags");
    for (label, changed, flag) in cases {
        let result = compare_raw(
            &dir,
            &label.replace(' ', "-"),
            &[base("e.txt"), base("f.txt")],
            &[base("e.txt"), changed],
        );
        assert_eq!(result.status, 1, "{label}");
        assert_eq!(result.stderr, "", "{label}");
        assert_eq!(
            status_and_flags(&result.stdout),
            pairs(&[("identical", ""), ("identical", flag)]),
            "{label}"
        );
        assert_eq!(archive_value(&result.stdout, "flagged"), "1", "{label}");
    }
}

#[test]
fn descriptors_that_differ_only_in_the_signature() {
    let with = |name: &str| RawEntry {
        flags: 0x0008,
        ..base(name)
    };
    let without = |name: &str| RawEntry {
        flags: 0x0008,
        descriptor_signature: false,
        ..base(name)
    };
    let dir = TempDir::new("p5-signature");
    let result = compare_raw(
        &dir,
        "signature",
        &[with("d.txt"), with("e.txt")],
        &[without("d.txt"), without("e.txt")],
    );
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    assert_eq!(
        status_and_flags(&result.stdout),
        pairs(&[("identical", "descriptor"), ("identical", "descriptor")])
    );
    assert_eq!(archive_value(&result.stdout, "flagged"), "2");

    // The same descriptors on both sides carry no flag.
    let result = compare_raw(
        &dir,
        "signature-same",
        &[with("d.txt"), without("e.txt")],
        &[with("d.txt"), without("e.txt")],
    );
    assert_eq!(result.status, 0);
    assert_eq!(
        status_and_flags(&result.stdout),
        pairs(&[("identical", ""), ("identical", "")])
    );
}

#[test]
fn padding_after_an_entry() {
    let padded = |name: &str| RawEntry {
        padding_after: pattern(64),
        ..base(name)
    };
    let dir = TempDir::new("p5-padding");

    // Padding between two entries on one side: `descriptor` on the entry
    // before the padding.
    let result = compare_raw(
        &dir,
        "between",
        &[padded("a.txt"), base("b.txt")],
        &[base("a.txt"), base("b.txt")],
    );
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    assert_eq!(
        status_and_flags(&result.stdout),
        pairs(&[("identical", "descriptor"), ("identical", "")])
    );
    assert_eq!(archive_value(&result.stdout, "flagged"), "1");
    assert_eq!(
        status_counts(&result.stdout),
        expected_archive([2, 2, 2, 0, 0, 0, 0])
    );

    // The same case with the padding removed: exit 0.
    let result = compare_raw(
        &dir,
        "removed",
        &[base("a.txt"), base("b.txt")],
        &[base("a.txt"), base("b.txt")],
    );
    assert_eq!(result.status, 0);
    assert_eq!(result.stderr, "");
    assert_eq!(
        status_and_flags(&result.stdout),
        pairs(&[("identical", ""), ("identical", "")])
    );

    // Equal padding on both sides: no flag, exit 0.
    let result = compare_raw(
        &dir,
        "equal",
        &[padded("a.txt"), base("b.txt")],
        &[padded("a.txt"), base("b.txt")],
    );
    assert_eq!(result.status, 0);
    assert_eq!(archive_value(&result.stdout, "flagged"), "0");

    // Padding after the last entry, before the central directory.
    let result = compare_raw(
        &dir,
        "last",
        &[base("a.txt"), padded("b.txt")],
        &[base("a.txt"), base("b.txt")],
    );
    assert_eq!(result.status, 1);
    assert_eq!(
        status_and_flags(&result.stdout),
        pairs(&[("identical", ""), ("identical", "descriptor")])
    );

    // Padding of the same length but different bytes, larger than one
    // 64 KiB buffer, differing only in its last byte.
    let long = pattern(200_000);
    let long_other = flipped(&long, &[199_999]);
    let result = compare_raw(
        &dir,
        "long",
        &[
            RawEntry {
                padding_after: long,
                ..base("a.txt")
            },
            base("b.txt"),
        ],
        &[
            RawEntry {
                padding_after: long_other,
                ..base("a.txt")
            },
            base("b.txt"),
        ],
    );
    assert_eq!(result.status, 1);
    assert_eq!(
        status_and_flags(&result.stdout),
        pairs(&[("identical", "descriptor"), ("identical", "")])
    );
}
