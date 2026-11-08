//! `EMEND_COPY_KEEP_OUTPUT` (`specs/0047-harness-findings.md`, "The
//! diagnosis record (P1)"): a failed read-back leaves its output in place
//! when the variable is `1`, and the error line ends `(output kept)`.
//!
//! The fixture is a fictional workbook built here, byte by byte, with one
//! sheet and no `<bookViews>`: the sheet sample adds a sheet, moves it first
//! and asserts the first view's active tab moved by one, which a workbook
//! with no view cannot show, so its read-back fails on "active tab". The
//! zip builder follows `tests/null_rewrite.rs`, copied rather than shared,
//! as that file's own is. The variable is set on the child's command only,
//! never on this process.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

const LOCAL_SIG: u32 = 0x0403_4b50;
const CENTRAL_SIG: u32 = 0x0201_4b50;
const EOCD_SIG: u32 = 0x0605_4b50;
const VARIABLE: &str = "EMEND_COPY_KEEP_OUTPUT";
const ACTIVE_TAB: &str = "error: the output's active tab is not what the edits imply";

/// A temporary directory, removed when dropped.
struct TempDir(PathBuf);

impl TempDir {
    fn new(label: &str) -> TempDir {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "emend-copy-keep-{}-{label}-{n}",
            std::process::id()
        ));
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

/// What the tool did.
struct Ran {
    status: i32,
    stdout: String,
    stderr: String,
}

/// Runs the sheet sample with the variable set to `value`, or removed from
/// the child's environment when `None`.
fn sheet_sample(input: &Path, output: &Path, value: Option<&str>) -> Ran {
    let mut command = Command::new(env!("CARGO_BIN_EXE_emend-copy"));
    command.arg(input).arg(output).arg("--sheet-sample");
    match value {
        Some(value) => command.env(VARIABLE, value),
        None => command.env_remove(VARIABLE),
    };
    let out = command.output().expect("run emend-copy");
    Ran {
        status: out.status.code().expect("an exit code"),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

fn crc32(data: &[u8]) -> u32 {
    let mut table = [0u32; 256];
    for (n, slot) in table.iter_mut().enumerate() {
        let mut c = n as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 {
                0xEDB8_8320 ^ (c >> 1)
            } else {
                c >> 1
            };
        }
        *slot = c;
    }
    let mut crc = 0xFFFF_FFFFu32;
    for byte in data {
        crc = table[((crc ^ u32::from(*byte)) & 0xFF) as usize] ^ (crc >> 8);
    }
    crc ^ 0xFFFF_FFFF
}

fn u16_of(n: usize) -> [u8; 2] {
    (n as u16).to_le_bytes()
}

fn u32_of(n: usize) -> [u8; 4] {
    (n as u32).to_le_bytes()
}

/// A stored archive of the entries, in order, with no descriptors, no extra
/// fields and no comment.
fn build(entries: &[(&str, String)]) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut offsets = Vec::new();
    for (name, data) in entries {
        offsets.push(bytes.len());
        bytes.extend_from_slice(&LOCAL_SIG.to_le_bytes());
        bytes.extend_from_slice(&u16_of(20)); // version needed
        bytes.extend_from_slice(&u16_of(0)); // flags
        bytes.extend_from_slice(&u16_of(0)); // stored
        bytes.extend_from_slice(&u16_of(0)); // time
        bytes.extend_from_slice(&u16_of(0x21)); // date
        bytes.extend_from_slice(&crc32(data.as_bytes()).to_le_bytes());
        bytes.extend_from_slice(&u32_of(data.len()));
        bytes.extend_from_slice(&u32_of(data.len()));
        bytes.extend_from_slice(&u16_of(name.len()));
        bytes.extend_from_slice(&u16_of(0)); // extra length
        bytes.extend_from_slice(name.as_bytes());
        bytes.extend_from_slice(data.as_bytes());
    }
    let central_start = bytes.len();
    for ((name, data), offset) in entries.iter().zip(&offsets) {
        bytes.extend_from_slice(&CENTRAL_SIG.to_le_bytes());
        bytes.extend_from_slice(&u16_of(20)); // version made by
        bytes.extend_from_slice(&u16_of(20)); // version needed
        bytes.extend_from_slice(&u16_of(0)); // flags
        bytes.extend_from_slice(&u16_of(0)); // stored
        bytes.extend_from_slice(&u16_of(0));
        bytes.extend_from_slice(&u16_of(0x21));
        bytes.extend_from_slice(&crc32(data.as_bytes()).to_le_bytes());
        bytes.extend_from_slice(&u32_of(data.len()));
        bytes.extend_from_slice(&u32_of(data.len()));
        bytes.extend_from_slice(&u16_of(name.len()));
        bytes.extend_from_slice(&u16_of(0)); // extra
        bytes.extend_from_slice(&u16_of(0)); // comment
        bytes.extend_from_slice(&u16_of(0)); // disk
        bytes.extend_from_slice(&u16_of(0)); // internal attributes
        bytes.extend_from_slice(&u32_of(0)); // external attributes
        bytes.extend_from_slice(&u32_of(*offset));
        bytes.extend_from_slice(name.as_bytes());
    }
    let central_size = bytes.len() - central_start;
    bytes.extend_from_slice(&EOCD_SIG.to_le_bytes());
    bytes.extend_from_slice(&u16_of(0)); // this disk
    bytes.extend_from_slice(&u16_of(0)); // central directory disk
    bytes.extend_from_slice(&u16_of(entries.len()));
    bytes.extend_from_slice(&u16_of(entries.len()));
    bytes.extend_from_slice(&u32_of(central_size));
    bytes.extend_from_slice(&u32_of(central_start));
    bytes.extend_from_slice(&u16_of(0)); // comment length
    bytes
}

const TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const MAIN_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
const ATTR_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const RELS_TYPE: &str = "application/vnd.openxmlformats-package.relationships+xml";
const WORKBOOK_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
const SHEET_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";

/// One sheet with one cell and no `<bookViews>` in the workbook part.
fn no_views_workbook() -> Vec<u8> {
    build(&[
        (
            "[Content_Types].xml",
            format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
                 <Types xmlns=\"{TYPES_NS}\">\
                 <Default Extension=\"rels\" ContentType=\"{RELS_TYPE}\"/>\
                 <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
                 <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
                 <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{SHEET_TYPE}\"/>\
                 </Types>"
            ),
        ),
        (
            "_rels/.rels",
            format!(
                "<Relationships xmlns=\"{RELS_NS}\">\
                 <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/officeDocument\" \
                 Target=\"xl/workbook.xml\"/></Relationships>"
            ),
        ),
        (
            "xl/workbook.xml",
            format!(
                "<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
                 <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets>\
                 </workbook>"
            ),
        ),
        (
            "xl/_rels/workbook.xml.rels",
            format!(
                "<Relationships xmlns=\"{RELS_NS}\">\
                 <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/worksheet\" \
                 Target=\"worksheets/sheet1.xml\"/></Relationships>"
            ),
        ),
        (
            "xl/worksheets/sheet1.xml",
            format!(
                "<worksheet xmlns=\"{MAIN_NS}\"><sheetData>\
                 <row r=\"1\"><c r=\"A1\"><v>1</v></c></row></sheetData></worksheet>"
            ),
        ),
    ])
}

/// The fixture written to `in.xlsx` in a new directory.
fn fixture(label: &str) -> (TempDir, PathBuf, PathBuf) {
    let dir = TempDir::new(label);
    let input = dir.path("in.xlsx");
    fs::write(&input, no_views_workbook()).expect("write the fixture");
    let output = dir.path("out.xlsx");
    (dir, input, output)
}

#[test]
fn without_the_variable_a_failed_read_back_removes_the_output() {
    let (_dir, input, output) = fixture("unset");
    let ran = sheet_sample(&input, &output, None);
    assert_eq!(ran.status, 2, "stderr: {}", ran.stderr);
    assert_eq!(ran.stderr, format!("{ACTIVE_TAB}\n"));
    assert_eq!(ran.stdout, "");
    assert!(!output.exists(), "the output is removed");
}

#[test]
fn with_the_variable_one_a_failed_read_back_keeps_the_output() {
    let (_dir, input, output) = fixture("one");
    let ran = sheet_sample(&input, &output, Some("1"));
    assert_eq!(ran.status, 2, "stderr: {}", ran.stderr);
    assert_eq!(ran.stderr, format!("{ACTIVE_TAB} (output kept)\n"));
    assert_eq!(ran.stdout, "");
    let kept = fs::read(&output).expect("the output is kept");
    assert_eq!(
        &kept[..4],
        &LOCAL_SIG.to_le_bytes(),
        "the kept output is a zip"
    );
    // The kept output is the one written: it holds the added sheet's part.
    let text = String::from_utf8_lossy(&kept);
    assert!(text.contains("xl/worksheets/sheet2.xml"), "the added part");
}

#[test]
fn any_other_value_removes_the_output_as_before() {
    for value in ["0", "", "yes", "1 "] {
        let (_dir, input, output) = fixture("other");
        let ran = sheet_sample(&input, &output, Some(value));
        assert_eq!(ran.status, 2, "value {value:?}: {}", ran.stderr);
        assert_eq!(ran.stderr, format!("{ACTIVE_TAB}\n"), "value {value:?}");
        assert!(!output.exists(), "value {value:?}: the output is removed");
    }
}

#[test]
fn a_refusal_before_the_output_is_written_says_nothing_of_a_kept_output() {
    let dir = TempDir::new("refused");
    let input = dir.path("in.xlsx");
    fs::write(&input, b"not a zip").expect("write the input");
    let output = dir.path("out.xlsx");
    let ran = sheet_sample(&input, &output, Some("1"));
    assert_eq!(ran.status, 2);
    assert!(ran.stderr.starts_with("error: "), "stderr: {}", ran.stderr);
    assert!(
        !ran.stderr.contains("(output kept)"),
        "stderr: {}",
        ran.stderr
    );
    assert!(!output.exists(), "no output is written");
}
