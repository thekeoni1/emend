//! The sheet sample's read-back of the active tab against the input's
//! `<bookViews>` (`specs/0047-harness-findings.md`, "The fixes (P2)",
//! decision 3 A, and ruling D): with a `bookViews` in the input the first
//! view's active tab moves by one; without one the output holds none and
//! reads 0, since the core remaps a present view and creates none. The line
//! carries `views_before` and `views_after`.
//!
//! The fixtures are fictional workbooks built here, byte by byte, with one
//! sheet: one with no `<bookViews>`, one with a view whose active tab is 0,
//! and a negative one whose `<bookViews>` holds no view, so the tab cannot
//! move and the read-back fails on "active tab". An output doctored to hold
//! a view the input lacked cannot come out of the binary, whose core creates
//! none, so that case is the unit test in `src/main.rs`
//! (`a_view_in_the_output_of_an_input_without_one_fails_on_active_tab`). The
//! zip builder follows `tests/keep_output.rs`, copied rather than shared, as
//! that file's own is.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

const LOCAL_SIG: u32 = 0x0403_4b50;
const CENTRAL_SIG: u32 = 0x0201_4b50;
const EOCD_SIG: u32 = 0x0605_4b50;
const ACTIVE_TAB: &str = "error: the output's active tab is not what the edits imply";

/// A temporary directory, removed when dropped.
struct TempDir(PathBuf);

impl TempDir {
    fn new(label: &str) -> TempDir {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "emend-copy-views-{}-{label}-{n}",
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

/// Runs the sheet sample, with `EMEND_COPY_KEEP_OUTPUT` removed from the
/// child's environment so a failure removes the output as in a corpus run.
fn sheet_sample(input: &Path, output: &Path) -> Ran {
    let out = Command::new(env!("CARGO_BIN_EXE_emend-copy"))
        .arg(input)
        .arg(output)
        .arg("--sheet-sample")
        .env_remove("EMEND_COPY_KEEP_OUTPUT")
        .output()
        .expect("run emend-copy");
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

/// One sheet with one cell, the workbook part's `views` (empty for none)
/// before its `<sheets>`.
fn workbook(views: &str) -> Vec<u8> {
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
                "<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">{views}\
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

/// The workbook with `views` written to `in.xlsx` in a new directory, the
/// sheet sample run over it into `out.xlsx`.
fn run(label: &str, views: &str) -> (TempDir, PathBuf, Ran) {
    let dir = TempDir::new(label);
    let input = dir.path("in.xlsx");
    fs::write(&input, workbook(views)).expect("write the fixture");
    let output = dir.path("out.xlsx");
    let ran = sheet_sample(&input, &output);
    (dir, output, ran)
}

/// Whether the output's workbook part names a `bookViews` element.
fn holds_views(output: &Path) -> bool {
    let bytes = fs::read(output).expect("read the output");
    String::from_utf8_lossy(&bytes).contains("<bookViews")
}

#[test]
fn a_workbook_without_book_views_passes_and_the_output_holds_none() {
    let (_dir, output, ran) = run("none", "");
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(
        ran.stdout,
        "sheet\tsheets_before=1\tsheets_after=2\tfirst=1\tcells=1\tnames=0\
         \tactive_before=0\tactive_after=0\tviews_before=0\tviews_after=0\n"
    );
    assert_eq!(ran.stderr, "");
    assert!(output.exists(), "the output is written");
    assert!(!holds_views(&output), "the output holds no bookViews");
}

#[test]
fn a_workbook_with_book_views_still_passes_with_the_tab_moved() {
    let (_dir, output, ran) = run(
        "views",
        "<bookViews><workbookView activeTab=\"0\"/></bookViews>",
    );
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(
        ran.stdout,
        "sheet\tsheets_before=1\tsheets_after=2\tfirst=1\tcells=1\tnames=0\
         \tactive_before=0\tactive_after=1\tviews_before=1\tviews_after=1\n"
    );
    assert_eq!(ran.stderr, "");
    assert!(holds_views(&output), "the output keeps its bookViews");
}

#[test]
fn book_views_with_no_view_fails_on_active_tab() {
    let (_dir, output, ran) = run("empty", "<bookViews/>");
    assert_eq!(ran.status, 2, "stdout: {}", ran.stdout);
    assert_eq!(ran.stderr, format!("{ACTIVE_TAB}\n"));
    assert_eq!(ran.stdout, "");
    assert!(!output.exists(), "the output is removed");
}
