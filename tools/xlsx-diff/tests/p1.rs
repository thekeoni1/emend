//! T1 P1 tests (`specs/0004-xlsx-diff.md`). Every archive is generated here
//! from fictional content, in a temporary directory that is removed when the
//! test ends. Nothing is committed.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, DateTime, ZipArchive, ZipWriter};

/// A temporary directory, removed when dropped.
struct TempDir(PathBuf);

impl TempDir {
    fn new(label: &str) -> TempDir {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("xlsx-diff-test-{}-{label}-{n}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create the temporary directory");
        TempDir(dir)
    }

    fn path(&self, name: &str) -> PathBuf {
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
struct Spec {
    name: String,
    data: Vec<u8>,
    method: CompressionMethod,
    level: Option<i64>,
}

fn deflated(name: &str, data: &[u8]) -> Spec {
    Spec {
        name: name.to_string(),
        data: data.to_vec(),
        method: CompressionMethod::Deflated,
        level: None,
    }
}

fn stored(name: &str, data: &[u8]) -> Spec {
    Spec {
        method: CompressionMethod::Stored,
        ..deflated(name, data)
    }
}

/// Fictional text of a given number of lines.
fn text(lines: usize) -> Vec<u8> {
    (0..lines)
        .map(|i| format!("fictional row {i}: alpha beta gamma {}\n", i * 7 % 13))
        .collect::<String>()
        .into_bytes()
}

fn write_zip(path: &Path, entries: &[Spec]) {
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
struct Outcome {
    status: u8,
    stdout: String,
    stderr: String,
}

fn run(input: &Path, output: &Path) -> Outcome {
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
fn rows<'a>(stdout: &'a str, kind: &str) -> Vec<Vec<&'a str>> {
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
fn archive_rows(stdout: &str) -> Vec<String> {
    rows(stdout, "archive")
        .into_iter()
        .map(|c| format!("{}={}", c[1], c[2]))
        .collect()
}

/// Method, uncompressed size, compressed size and CRC-32 of every entry, read
/// independently of the tool.
fn meta(path: &Path) -> Vec<[String; 4]> {
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

/// Asserts columns 4 to 13 of an entry row against the independent metadata,
/// and columns 14 to 17 empty.
fn assert_columns(
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
    assert_eq!(&row[13..], ["", "", "", ""], "columns 14 to 17");
}

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
        }
        assert_eq!(
            archive_rows(&result.stdout),
            [
                "entries_in=2",
                "entries_out=2",
                "identical=2",
                "differs=0",
                "dropped=0",
                "added=0"
            ]
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
    assert_eq!(
        archive_rows(&result.stdout),
        [
            "entries_in=3",
            "entries_out=2",
            "identical=2",
            "differs=0",
            "dropped=1",
            "added=0"
        ]
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
    assert_eq!(
        archive_rows(&result.stdout),
        [
            "entries_in=1",
            "entries_out=2",
            "identical=1",
            "differs=0",
            "dropped=0",
            "added=1"
        ]
    );
}

#[test]
fn recompressed_content_differs() {
    let dir = TempDir::new("recompressed");
    let data = text(2000);
    let fast = Spec {
        level: Some(1),
        ..deflated("level.xml", &data)
    };
    let best = Spec {
        level: Some(9),
        ..deflated("level.xml", &data)
    };
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &[fast, deflated("method.xml", &data)]);
    write_zip(&output, &[best, stored("method.xml", &data)]);
    let (mi, mo) = (meta(&input), meta(&output));
    assert_eq!(
        mi[0][3], mo[0][3],
        "precondition: equal CRC-32 for the level entry"
    );
    assert_ne!(
        mi[0][2], mo[0][2],
        "precondition: different compressed sizes for the level entry"
    );

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    let entry_rows = rows(&result.stdout, "entry");
    let summary: Vec<(&str, &str)> = entry_rows.iter().map(|r| (r[1], r[2])).collect();
    assert_eq!(
        summary,
        [("level.xml", "differs"), ("method.xml", "differs")]
    );
    assert_columns(&entry_rows[0], Some((0, &mi[0])), Some((0, &mo[0])));
    assert_columns(&entry_rows[1], Some((1, &mi[1])), Some((1, &mo[1])));
    assert_eq!((entry_rows[1][5], entry_rows[1][6]), ("8", "0"));
    assert_eq!(
        archive_rows(&result.stdout),
        [
            "entries_in=2",
            "entries_out=2",
            "identical=0",
            "differs=2",
            "dropped=0",
            "added=0"
        ]
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

/// Replaces every occurrence of `from` with `to` (same length) and returns
/// the count.
fn patch(bytes: &mut [u8], from: &[u8], to: &[u8]) -> usize {
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

fn binary(args: &[&Path]) -> (Option<i32>, String, String) {
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
