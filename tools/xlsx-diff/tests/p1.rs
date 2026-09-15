//! T1 P1 tests (`specs/0004-xlsx-diff.md`), with the P2 statuses and archive
//! rows. Every archive is generated from fictional content in a temporary
//! directory; nothing is committed. The P1 recompression case (then
//! `differs`) is now `recompressed_entry` in `p2.rs`, and the P1 flipped
//! stored byte is part of `wrong_crc_is_an_error` there.

mod common;

use std::fs;

use common::*;

#[test]
fn equal_archives_are_identical() {
    let dir = TempDir::new("equal");
    let entries = [
        deflated("xl/workbook.xml", &text(200)),
        stored("docProps/app.xml", &text(3)),
    ];
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &entries);
    write_zip(&output, &entries);

    for other in [output.clone(), dir.path("copy.zip")] {
        if other.ends_with("copy.zip") {
            fs::copy(&input, &other).expect("copy the archive");
        }
        let result = run(&input, &other);
        assert_eq!(result.status, 0);
        assert_eq!(result.stderr, "");
        let (mi, mo) = (meta(&input), meta(&other));
        let entry_rows = rows(&result.stdout, "entry");
        assert_eq!(entry_rows.len(), 2);
        for (i, row) in entry_rows.iter().enumerate() {
            assert_eq!(row[1], entries[i].name);
            assert_eq!(row[2], "identical");
            assert_columns(row, Some((i, &mi[i])), Some((i, &mo[i])));
            assert_tail(row, ["", "", "", ""]);
        }
        assert_eq!(
            archive_rows(&result.stdout),
            expected_archive([2, 2, 2, 0, 0, 0, 0])
        );
    }
}

#[test]
fn stored_entry_beside_deflated_entry() {
    let dir = TempDir::new("stored");
    let entries = [stored("a.txt", &text(20)), deflated("b.txt", &text(20))];
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &entries);
    write_zip(&output, &entries);
    let result = run(&input, &output);
    assert_eq!(result.status, 0);
    let entry_rows = rows(&result.stdout, "entry");
    assert_eq!(
        (entry_rows[0][5], entry_rows[0][6], entry_rows[0][2]),
        ("0", "0", "identical")
    );
    assert_eq!(
        (entry_rows[1][5], entry_rows[1][6], entry_rows[1][2]),
        ("8", "8", "identical")
    );
    assert_eq!(
        entry_rows[0][7], entry_rows[0][9],
        "a stored entry's sizes are equal"
    );
}

#[test]
fn stored_entry_across_several_buffers_is_identical() {
    let dir = TempDir::new("buffers");
    let entries = [
        stored("big.bin", &pattern(200_000)),
        deflated("small.xml", &text(30)),
    ];
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &entries);
    write_zip(&output, &entries);
    let result = run(&input, &output);
    assert_eq!(result.status, 0);
    assert_eq!(result.stderr, "");
    assert_eq!(
        summary(&result.stdout),
        [
            ("big.bin".to_string(), "identical".to_string()),
            ("small.xml".to_string(), "identical".to_string())
        ]
    );
}

#[test]
fn dropped_entry() {
    let dir = TempDir::new("dropped");
    let (a, b, c) = (
        deflated("a.xml", &text(10)),
        deflated("b.xml", &text(11)),
        stored("c.xml", &text(12)),
    );
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &[a.clone(), b, c.clone()]);
    write_zip(&output, &[a, c]);
    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let (mi, mo) = (meta(&input), meta(&output));
    let entry_rows = rows(&result.stdout, "entry");
    let summary: Vec<(&str, &str)> = entry_rows.iter().map(|r| (r[1], r[2])).collect();
    assert_eq!(
        summary,
        [
            ("a.xml", "identical"),
            ("b.xml", "dropped"),
            ("c.xml", "identical")
        ]
    );
    assert_columns(&entry_rows[0], Some((0, &mi[0])), Some((0, &mo[0])));
    assert_columns(&entry_rows[1], Some((1, &mi[1])), None);
    assert_columns(&entry_rows[2], Some((2, &mi[2])), Some((1, &mo[1])));
    for row in &entry_rows {
        assert_tail(row, ["", "", "", ""]);
    }
    assert_eq!(
        archive_rows(&result.stdout),
        expected_archive([3, 2, 2, 0, 0, 1, 0])
    );
}

#[test]
fn added_entry() {
    let dir = TempDir::new("added");
    let (a, d) = (deflated("a.xml", &text(10)), deflated("d.xml", &text(9)));
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, std::slice::from_ref(&a));
    write_zip(&output, &[a, d]);
    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    let (mi, mo) = (meta(&input), meta(&output));
    let entry_rows = rows(&result.stdout, "entry");
    let summary: Vec<(&str, &str)> = entry_rows.iter().map(|r| (r[1], r[2])).collect();
    assert_eq!(summary, [("a.xml", "identical"), ("d.xml", "added")]);
    assert_columns(&entry_rows[0], Some((0, &mi[0])), Some((0, &mo[0])));
    assert_columns(&entry_rows[1], None, Some((1, &mo[1])));
    for row in &entry_rows {
        assert_tail(row, ["", "", "", ""]);
    }
    assert_eq!(
        archive_rows(&result.stdout),
        expected_archive([1, 2, 1, 0, 0, 0, 1])
    );
}

#[test]
fn names_are_escaped() {
    assert_eq!(
        xlsx_diff::escape_name(&[0x00, 0x1f, b' ', b'A', b'\\', b'~', 0x7f, 0x80, 0xc3, 0xff]),
        "\\x00\\x1f A\\\\~\\x7f\\x80\\xc3\\xff"
    );

    let dir = TempDir::new("escape");
    let names = [
        "a\u{0}b.txt",
        "c\u{7f}d.txt",
        "\u{e9}.txt",
        "back\\slash.txt",
    ];
    let entries: Vec<Spec> = names.iter().map(|n| deflated(n, &text(2))).collect();
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &entries);
    write_zip(&output, &entries);
    let result = run(&input, &output);
    assert_eq!(result.status, 0);
    let printed: Vec<&str> = rows(&result.stdout, "entry").iter().map(|r| r[1]).collect();
    assert_eq!(
        printed,
        [
            "a\\x00b.txt",
            "c\\x7fd.txt",
            "\\xc3\\xa9.txt",
            "back\\\\slash.txt"
        ]
    );
}

#[test]
fn duplicate_name_is_an_error() {
    let dir = TempDir::new("duplicate");
    let (good, dup) = (dir.path("good.zip"), dir.path("dup.zip"));
    write_zip(
        &good,
        &[deflated("one.txt", &text(4)), deflated("two.txt", &text(5))],
    );
    let mut bytes = fs::read(&good).expect("read");
    assert_eq!(
        patch(&mut bytes, b"two.txt", b"one.txt"),
        2,
        "precondition: local and central name patched"
    );
    fs::write(&dup, bytes).expect("write");

    let result = run(&dup, &good);
    assert_eq!(result.status, 2);
    assert_eq!(result.stdout, "");
    assert_eq!(
        result.stderr,
        "error: duplicate entry name in the input archive: one.txt\n"
    );

    let result = run(&good, &dup);
    assert_eq!(result.status, 2);
    assert_eq!(result.stdout, "");
    assert_eq!(
        result.stderr,
        "error: duplicate entry name in the output archive: one.txt\n"
    );
}

#[test]
fn non_zip_file_is_an_error() {
    let dir = TempDir::new("nonzip");
    let (good, plain) = (dir.path("good.zip"), dir.path("plain.zip"));
    write_zip(&good, &[deflated("a.txt", &text(3))]);
    fs::write(&plain, "fictional plain text, not an archive\n").expect("write");
    let result = run(&good, &plain);
    assert_eq!(result.status, 2);
    assert_eq!(result.stdout, "");
    assert_eq!(
        result.stderr,
        "error: the output file is not a readable zip archive\n"
    );
}

#[test]
fn missing_file_is_an_error() {
    let dir = TempDir::new("missing");
    let good = dir.path("good.zip");
    write_zip(&good, &[deflated("a.txt", &text(3))]);
    let result = run(&dir.path("absent.zip"), &good);
    assert_eq!(result.status, 2);
    assert_eq!(result.stdout, "");
    assert!(
        result
            .stderr
            .starts_with("error: cannot read the input file: "),
        "{}",
        result.stderr
    );
    assert_eq!(result.stderr.lines().count(), 1);
    assert!(
        !result.stderr.contains("absent"),
        "no path in the error line"
    );
}

#[test]
fn binary_usage_error() {
    let dir = TempDir::new("usage");
    let a = dir.path("a.zip");
    for args in [
        vec![],
        vec![a.as_path()],
        vec![a.as_path(), a.as_path(), a.as_path()],
    ] {
        let (code, stdout, stderr) = binary(&args);
        assert_eq!(code, Some(2));
        assert_eq!(stdout, "");
        assert_eq!(stderr, format!("{}\n", xlsx_diff::USAGE));
    }
}

#[test]
fn binary_exit_status_follows_the_comparison() {
    let dir = TempDir::new("binary");
    let (input, same, fewer) = (
        dir.path("in.zip"),
        dir.path("same.zip"),
        dir.path("fewer.zip"),
    );
    write_zip(
        &input,
        &[deflated("a.xml", &text(5)), deflated("b.xml", &text(6))],
    );
    write_zip(
        &same,
        &[deflated("a.xml", &text(5)), deflated("b.xml", &text(6))],
    );
    write_zip(&fewer, &[deflated("a.xml", &text(5))]);

    let (code, stdout, stderr) = binary(&[&input, &same]);
    assert_eq!((code, stderr.as_str()), (Some(0), ""));
    assert_eq!(rows(&stdout, "entry").len(), 2);

    let (code, stdout, _) = binary(&[&input, &fewer]);
    assert_eq!(code, Some(1));
    assert!(archive_rows(&stdout).contains(&"dropped=1".to_string()));
}
