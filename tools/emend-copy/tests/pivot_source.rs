//! The removal sample on a workbook whose pivot cache sources its last
//! sheet (`specs/0048-the-two-refusals.md`, "The harness", decision 3 A):
//! the layer refuses the removal with "a pivot cache sources the sheet", the
//! sample treats it as it treats the two "last" refusals, nothing is
//! removed, the output is the input's bytes and the line names the class,
//! `removed_reason=pivot_source`.
//!
//! The fixture is a fictional two-sheet workbook built here, byte by byte,
//! whose one pivot cache's worksheet source names the last sheet. The zip
//! builder follows `tests/book_views.rs`, copied rather than shared, as
//! that file's own is.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

const LOCAL_SIG: u32 = 0x0403_4b50;
const CENTRAL_SIG: u32 = 0x0201_4b50;
const EOCD_SIG: u32 = 0x0605_4b50;

/// A temporary directory, removed when dropped.
struct TempDir(PathBuf);

impl TempDir {
    fn new(label: &str) -> TempDir {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "emend-copy-pivot-{}-{label}-{n}",
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

/// Runs the removal sample, with `EMEND_COPY_KEEP_OUTPUT` removed from the
/// child's environment so a failure removes the output as in a corpus run.
fn remove_sample(input: &Path, output: &Path) -> Ran {
    let out = Command::new(env!("CARGO_BIN_EXE_emend-copy"))
        .arg(input)
        .arg(output)
        .arg("--remove-sample")
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

/// Two sheets, the second the source of the workbook's one pivot cache.
fn workbook() -> Vec<u8> {
    let cache_type =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml";
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
                 <Override PartName=\"/xl/worksheets/sheet2.xml\" ContentType=\"{SHEET_TYPE}\"/>\
                 <Override PartName=\"/xl/pivotCache/pivotCacheDefinition1.xml\" ContentType=\"{cache_type}\"/>\
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
                 <sheets><sheet name=\"Report\" sheetId=\"1\" r:id=\"rId1\"/>\
                 <sheet name=\"Data\" sheetId=\"2\" r:id=\"rId2\"/></sheets>\
                 <pivotCaches><pivotCache cacheId=\"1\" r:id=\"rId3\"/></pivotCaches>\
                 </workbook>"
            ),
        ),
        (
            "xl/_rels/workbook.xml.rels",
            format!(
                "<Relationships xmlns=\"{RELS_NS}\">\
                 <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/worksheet\" \
                 Target=\"worksheets/sheet1.xml\"/>\
                 <Relationship Id=\"rId2\" Type=\"{ATTR_NS}/worksheet\" \
                 Target=\"worksheets/sheet2.xml\"/>\
                 <Relationship Id=\"rId3\" Type=\"{ATTR_NS}/pivotCacheDefinition\" \
                 Target=\"pivotCache/pivotCacheDefinition1.xml\"/></Relationships>"
            ),
        ),
        (
            "xl/worksheets/sheet1.xml",
            format!(
                "<worksheet xmlns=\"{MAIN_NS}\"><sheetData>\
                 <row r=\"1\"><c r=\"A1\"><v>1</v></c></row></sheetData></worksheet>"
            ),
        ),
        (
            "xl/worksheets/sheet2.xml",
            format!(
                "<worksheet xmlns=\"{MAIN_NS}\"><sheetData>\
                 <row r=\"1\"><c r=\"A1\"><v>2</v></c></row></sheetData></worksheet>"
            ),
        ),
        (
            "xl/pivotCache/pivotCacheDefinition1.xml",
            format!(
                "<pivotCacheDefinition xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
                 <cacheSource type=\"worksheet\"><worksheetSource ref=\"A1:A1\" sheet=\"Data\"/>\
                 </cacheSource><cacheFields count=\"0\"/></pivotCacheDefinition>"
            ),
        ),
    ])
}

#[test]
fn a_last_sheet_a_pivot_cache_sources_is_not_removed_and_the_line_names_the_class() {
    let dir = TempDir::new("source");
    let input = dir.path("in.xlsx");
    fs::write(&input, workbook()).expect("write the fixture");
    let output = dir.path("out.xlsx");
    let ran = remove_sample(&input, &output);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(ran.stderr, "");
    assert_eq!(
        ran.stdout,
        "remove\tsheets_before=2\tsheets_after=2\tremoved=0\tparts=0\tchain_before=0\
         \tchain_after=0\tnames_before=0\tnames_after=0\tlocal_removed=0\tactive_before=0\
         \tactive_after=0\tremoved_reason=pivot_source\n",
        "the line"
    );
    assert_eq!(
        fs::read(&output).expect("read the output"),
        fs::read(&input).expect("read the input"),
        "the output is the input's bytes"
    );
}
