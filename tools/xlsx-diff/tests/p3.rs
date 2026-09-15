//! T1 P3 tests (`specs/0004-xlsx-diff.md`): archive rows, flags and exit
//! status. Every archive is generated from fictional content in a temporary
//! directory; nothing is committed.

mod common;

use std::fs;

use common::*;

/// A stored entry of fictional text used as the base for the flag cases.
fn base(name: &str) -> RawEntry {
    raw_stored(name, &text(12))
}

#[test]
fn every_archive_row_in_order() {
    let dir = TempDir::new("rows");
    let entries = [
        deflated("xl/workbook.xml", &text(300)),
        stored("docProps/core.xml", &text(7)),
    ];
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &entries);
    write_zip(&output, &entries);
    let (bytes_in, bytes_out) = (
        fs::read(&input).expect("read"),
        fs::read(&output).expect("read"),
    );
    assert_eq!(
        bytes_in, bytes_out,
        "precondition: the writer is deterministic"
    );

    let result = run(&input, &output);
    assert_eq!(result.status, 0);
    assert_eq!(result.stderr, "");
    let m = meta(&input);
    let sum = |k: usize| -> u64 { m.iter().map(|e| e[k].parse::<u64>().expect("number")).sum() };
    let central = recorded_central_offset(&bytes_in).to_string();
    let len = bytes_in.len().to_string();
    let expected: Vec<String> = [
        ("entries_in", "2".to_string()),
        ("entries_out", "2".to_string()),
        ("identical", "2".to_string()),
        ("recompressed", "0".to_string()),
        ("changed", "0".to_string()),
        ("dropped", "0".to_string()),
        ("added", "0".to_string()),
        ("flagged", "0".to_string()),
        ("order_preserved", "yes".to_string()),
        ("comment_equal", "yes".to_string()),
        ("prefix_in", "0".to_string()),
        ("prefix_out", "0".to_string()),
        ("central_start_in", central.clone()),
        ("central_start_out", central),
        ("uncompressed_in", sum(1).to_string()),
        ("uncompressed_out", sum(1).to_string()),
        ("compressed_in", sum(2).to_string()),
        ("compressed_out", sum(2).to_string()),
        ("bytes_in", len.clone()),
        ("bytes_out", len),
        ("files_identical", "yes".to_string()),
    ]
    .iter()
    .map(|(k, v)| format!("{k}={v}"))
    .collect();
    assert_eq!(archive_rows(&result.stdout), expected);
    assert_ne!(sum(1), sum(2), "precondition: the sums differ");
}

#[test]
fn reordered_entries() {
    let dir = TempDir::new("order");
    let (a, b, c) = (
        deflated("a.xml", &text(10)),
        deflated("b.xml", &text(11)),
        deflated("c.xml", &text(12)),
    );
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &[a.clone(), b.clone(), c.clone()]);
    write_zip(&output, &[c, a, b]);

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let entry_rows = rows(&result.stdout, "entry");
    let summary: Vec<(&str, &str, &str, &str)> = entry_rows
        .iter()
        .map(|r| (r[1], r[2], r[4], r[16]))
        .collect();
    assert_eq!(
        summary,
        [
            ("a.xml", "identical", "1", ""),
            ("b.xml", "identical", "2", ""),
            ("c.xml", "identical", "0", "")
        ]
    );
    assert_eq!(
        status_counts(&result.stdout),
        expected_archive([3, 3, 3, 0, 0, 0, 0])
    );
    assert_eq!(archive_value(&result.stdout, "order_preserved"), "no");
    assert_eq!(archive_value(&result.stdout, "flagged"), "0");
    assert_eq!(archive_value(&result.stdout, "comment_equal"), "yes");

    // A dropped entry between kept ones does not break the order.
    let kept = dir.path("kept.zip");
    write_zip(
        &kept,
        &[deflated("a.xml", &text(10)), deflated("c.xml", &text(12))],
    );
    let result = run(&input, &kept);
    assert_eq!(result.status, 1);
    assert_eq!(archive_value(&result.stdout, "order_preserved"), "yes");
}

#[test]
fn changed_archive_comment() {
    let dir = TempDir::new("comment");
    let entries = [base("a.txt"), base("b.txt")];
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    fs::write(&input, raw_zip(&[], &entries, b"fictional comment one")).expect("write");
    fs::write(&output, raw_zip(&[], &entries, b"fictional comment two")).expect("write");

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    for row in rows(&result.stdout, "entry") {
        assert_eq!((row[2], row[16]), ("identical", ""));
    }
    assert_eq!(archive_value(&result.stdout, "comment_equal"), "no");
    assert_eq!(archive_value(&result.stdout, "order_preserved"), "yes");
    assert_eq!(archive_value(&result.stdout, "flagged"), "0");
    assert_eq!(archive_value(&result.stdout, "files_identical"), "no");

    let same = dir.path("same.zip");
    fs::write(&same, raw_zip(&[], &entries, b"fictional comment one")).expect("write");
    let result = run(&input, &same);
    assert_eq!(result.status, 0);
    assert_eq!(archive_value(&result.stdout, "comment_equal"), "yes");
}

#[test]
fn each_flag_alone() {
    let extra = |id: u16, body: &[u8]| -> Vec<u8> {
        let mut field = id.to_le_bytes().to_vec();
        field.extend_from_slice(&u16::try_from(body.len()).expect("short").to_le_bytes());
        field.extend_from_slice(body);
        field
    };
    let cases: Vec<(&str, RawEntry, &str)> = vec![
        (
            "time",
            RawEntry {
                time: 0x6000,
                ..base("f.txt")
            },
            "mtime",
        ),
        (
            "date",
            RawEntry {
                date: 0x5a2f,
                ..base("f.txt")
            },
            "mtime",
        ),
        (
            "local-extra",
            RawEntry {
                local_extra: extra(0x6666, b"zq01"),
                ..base("f.txt")
            },
            "extra",
        ),
        (
            "central-extra",
            RawEntry {
                central_extra: extra(0x6666, b"zq02"),
                ..base("f.txt")
            },
            "extra",
        ),
        (
            "gpflags",
            RawEntry {
                flags: 0x0800,
                ..base("f.txt")
            },
            "gpflags",
        ),
        (
            "attributes",
            RawEntry {
                external_attributes: 0x0020,
                ..base("f.txt")
            },
            "attrs",
        ),
        (
            "version-made-by",
            RawEntry {
                version_made_by: 0x031e,
                ..base("f.txt")
            },
            "attrs",
        ),
        (
            "comment",
            RawEntry {
                comment: b"fictional entry comment".to_vec(),
                ..base("f.txt")
            },
            "comment",
        ),
    ];
    let dir = TempDir::new("flags");
    let input = dir.path("in.zip");
    fs::write(&input, raw_zip(&[], &[base("e.txt"), base("f.txt")], b"")).expect("write");
    for (label, changed, flag) in cases {
        let output = dir.path(&format!("{label}.zip"));
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
fn data_descriptor_on_one_side() {
    let dir = TempDir::new("descriptor");
    let with = RawEntry {
        flags: 0x0008,
        ..base("d.txt")
    };
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    fs::write(&input, raw_zip(&[], &[base("d.txt")], b"")).expect("write");
    fs::write(&output, raw_zip(&[], &[with], b"")).expect("write");
    assert_eq!(meta(&input), meta(&output), "precondition");

    for (a, b) in [(&input, &output), (&output, &input)] {
        let result = run(a, b);
        assert_eq!(result.status, 1);
        assert_eq!(result.stderr, "");
        let entry_rows = rows(&result.stdout, "entry");
        assert_eq!(entry_rows.len(), 1);
        assert_eq!(
            (entry_rows[0][2], entry_rows[0][16]),
            ("identical", "gpflags,descriptor,local")
        );
        assert_eq!(archive_value(&result.stdout, "flagged"), "1");
    }
}

#[test]
fn flags_on_a_changed_entry_and_none_on_dropped_or_added() {
    let dir = TempDir::new("flag-status");
    let changed = RawEntry {
        time: 0x6000,
        ..raw_stored("c.txt", &flipped(&text(12), &[3]))
    };
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    fs::write(
        &input,
        raw_zip(&[], &[base("c.txt"), base("gone.txt")], b""),
    )
    .expect("write");
    fs::write(&output, raw_zip(&[], &[changed, base("new.txt")], b"")).expect("write");

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    let summary: Vec<(&str, &str, &str, &str)> = rows(&result.stdout, "entry")
        .iter()
        .map(|r| (r[1], r[2], r[13], r[16]))
        .collect();
    assert_eq!(
        summary,
        [
            ("c.txt", "changed", "3", "mtime"),
            ("gone.txt", "dropped", "", ""),
            ("new.txt", "added", "", "")
        ]
    );
    assert_eq!(archive_value(&result.stdout, "flagged"), "1");
}

#[test]
fn files_identical_yes_on_a_byte_copy_and_no_otherwise() {
    let dir = TempDir::new("files");
    // Over 64 KiB, so the file comparison takes more than one buffer.
    let entries = [raw_stored("big.bin", &pattern(200_000))];
    let input = dir.path("in.zip");
    fs::write(&input, raw_zip(&[], &entries, b"fictional-x")).expect("write");

    let copy = dir.path("copy.zip");
    fs::copy(&input, &copy).expect("copy");
    let result = run(&input, &copy);
    assert_eq!(result.status, 0);
    assert_eq!(archive_value(&result.stdout, "files_identical"), "yes");

    // The same length, one byte different at the very end.
    let last = dir.path("last-byte.zip");
    fs::write(&last, raw_zip(&[], &entries, b"fictional-y")).expect("write");
    let result = run(&input, &last);
    assert_eq!(result.status, 1);
    assert_eq!(
        archive_value(&result.stdout, "bytes_in"),
        archive_value(&result.stdout, "bytes_out")
    );
    assert_eq!(archive_value(&result.stdout, "files_identical"), "no");

    // A different length.
    let longer = dir.path("longer.zip");
    fs::write(&longer, raw_zip(&[], &entries, b"fictional-xx")).expect("write");
    let result = run(&input, &longer);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    assert_ne!(
        archive_value(&result.stdout, "bytes_in"),
        archive_value(&result.stdout, "bytes_out")
    );
    assert_eq!(archive_value(&result.stdout, "files_identical"), "no");

    // The same length, one byte different in a middle buffer of the file.
    let middle = dir.path("middle.zip");
    let flipped_entries = [raw_stored(
        "big.bin",
        &flipped(&pattern(200_000), &[100_000]),
    )];
    fs::write(&middle, raw_zip(&[], &flipped_entries, b"fictional-x")).expect("write");
    let result = run(&input, &middle);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    assert_eq!(rows(&result.stdout, "entry")[0][2], "changed");
    assert_eq!(
        archive_value(&result.stdout, "bytes_in"),
        archive_value(&result.stdout, "bytes_out")
    );
    assert_eq!(archive_value(&result.stdout, "files_identical"), "no");
}

#[test]
fn leading_junk_bytes() {
    let dir = TempDir::new("prefix");
    let entries = [base("a.txt"), base("b.txt")];
    let junk = pattern(1000);
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    let (with, without) = (raw_zip(&junk, &entries, b""), raw_zip(&[], &entries, b""));
    fs::write(&input, &with).expect("write");
    fs::write(&output, &without).expect("write");
    let central = recorded_central_offset(&without);

    let result = run(&input, &output);
    assert_eq!(result.status, 0);
    assert_eq!(result.stderr, "");
    for row in rows(&result.stdout, "entry") {
        assert_eq!((row[2], row[16]), ("identical", ""));
    }
    assert_eq!(archive_value(&result.stdout, "prefix_in"), "1000");
    assert_eq!(archive_value(&result.stdout, "prefix_out"), "0");
    assert_eq!(
        archive_value(&result.stdout, "central_start_in"),
        (central + 1000).to_string()
    );
    assert_eq!(
        archive_value(&result.stdout, "central_start_out"),
        central.to_string()
    );
    assert_eq!(
        archive_value(&result.stdout, "bytes_in"),
        with.len().to_string()
    );
    assert_eq!(archive_value(&result.stdout, "files_identical"), "no");
}

#[test]
fn output_side_rows_take_the_output_values() {
    // Every in/out pair differs, so a row printing the other side's value
    // fails.
    let dir = TempDir::new("output-side");
    let b_data = text(30);
    let input_bytes = raw_zip(&[], &[base("a.txt")], b"");
    let output_bytes = raw_zip(
        &pattern(500),
        &[
            base("a.txt"),
            raw_deflated("b.txt", &b_data, &stored_blocks(&b_data, &[100])),
        ],
        b"",
    );
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    fs::write(&input, &input_bytes).expect("write");
    fs::write(&output, &output_bytes).expect("write");
    let (mi, mo) = (meta(&input), meta(&output));
    let sum = |m: &[[String; 4]], k: usize| -> u64 {
        m.iter().map(|e| e[k].parse::<u64>().expect("number")).sum()
    };

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let expected = [
        ("entries_in", "1".to_string()),
        ("entries_out", "2".to_string()),
        ("prefix_in", "0".to_string()),
        ("prefix_out", "500".to_string()),
        (
            "central_start_in",
            recorded_central_offset(&input_bytes).to_string(),
        ),
        (
            "central_start_out",
            (recorded_central_offset(&output_bytes) + 500).to_string(),
        ),
        ("uncompressed_in", sum(&mi, 1).to_string()),
        ("uncompressed_out", sum(&mo, 1).to_string()),
        ("compressed_in", sum(&mi, 2).to_string()),
        ("compressed_out", sum(&mo, 2).to_string()),
        ("bytes_in", input_bytes.len().to_string()),
        ("bytes_out", output_bytes.len().to_string()),
    ];
    for (key, value) in &expected {
        assert_eq!(&archive_value(&result.stdout, key), value, "{key}");
    }
    for pair in expected.chunks(2) {
        assert_ne!(pair[0].1, pair[1].1, "precondition: {} differs", pair[0].0);
    }
    assert_ne!(
        sum(&mo, 1),
        sum(&mo, 2),
        "precondition: the output's sums differ from each other"
    );
}

#[test]
fn different_lengths_across_several_buffers() {
    // Column 14 is the first differing offset past the first 64 KiB buffer,
    // or the shorter length when the bytes agree up to there.
    let dir = TempDir::new("lengths-buffers");
    let data = pattern(200_000);
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(
        &input,
        &[
            deflated("agree.bin", &data),
            deflated("differ.bin", &data),
            deflated("longer-out.bin", &data[..70_000]),
        ],
    );
    write_zip(
        &output,
        &[
            deflated("agree.bin", &data[..150_000]),
            deflated("differ.bin", &flipped(&data[..150_000], &[140_000])),
            deflated("longer-out.bin", &flipped(&data, &[69_999])),
        ],
    );

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let entry_rows = rows(&result.stdout, "entry");
    let expected = [
        ["150000", "", "", ""],
        ["140000", "", "", ""],
        ["69999", "", "", ""],
    ];
    assert_eq!(entry_rows.len(), 3);
    for (i, row) in entry_rows.iter().enumerate() {
        assert_eq!(row[2], "changed");
        assert_tail(row, expected[i]);
    }
}
