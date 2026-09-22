//! The null rewrite applied to a byte-built archive
//! (`specs/0009-package-layer.md`, "P1").
//!
//! The archive is built here, byte by byte, and the output is read back by a
//! reader written here: neither shares code with the layer the tool uses, so a
//! mistake in the layer cannot be the same mistake in the check. The byte
//! level builder follows the patterns of the core's own tests, copied into
//! this crate rather than shared across crates, as `tools/passthrough`'s tests
//! already do.
//!
//! Deflated entries are made without a compressor: a deflate stream of stored
//! (uncompressed) blocks is valid deflate, which is how the core's tests build
//! theirs. The layer re-compresses a replacement with the `zip` crate, so a
//! rewritten entry's COMPRESSED bytes may differ from the input's while its
//! decompressed bytes must not; that is why the checks below compare CRC-32
//! and uncompressed size for rewritten entries, and raw bytes for untouched
//! ones.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

const METHOD_STORE: u16 = 0;
const METHOD_DEFLATE: u16 = 8;
const LOCAL_SIG: u32 = 0x0403_4b50;
const CENTRAL_SIG: u32 = 0x0201_4b50;
const EOCD_SIG: u32 = 0x0605_4b50;
const CENTRAL_FIXED: usize = 46;
const BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

// ------------------------------------------------------------ scaffolding

/// A temporary directory, removed when dropped.
struct TempDir(PathBuf);

impl TempDir {
    fn new(label: &str) -> TempDir {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "emend-copy-test-{}-{label}-{n}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create the temporary directory");
        TempDir(dir)
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }

    /// The names left in the directory, sorted, so a leftover temporary is
    /// visible to a test.
    fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(&self.0)
            .expect("list the temporary directory")
            .map(|e| {
                e.expect("a directory entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        names.sort();
        names
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

fn emend_copy(args: &[&Path]) -> Ran {
    let out = Command::new(env!("CARGO_BIN_EXE_emend-copy"))
        .args(args)
        .output()
        .expect("run emend-copy");
    Ran {
        status: out.status.code().expect("an exit code"),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// The tool with the operation given after the two paths.
fn emend_copy_with(input: &Path, output: &Path, operation: &[&str]) -> Ran {
    let mut args: Vec<&Path> = vec![input, output];
    for arg in operation {
        args.push(Path::new(arg));
    }
    emend_copy(&args)
}

fn crc32(data: &[u8]) -> u32 {
    // The CRC-32 of the zip format, computed directly so the check shares
    // nothing with the layer or the `zip` crate.
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

/// A deflate stream of stored blocks: valid deflate with no compressor.
fn deflate_stored(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let chunks: Vec<&[u8]> = if data.is_empty() {
        vec![&[][..]]
    } else {
        data.chunks(0xFFFF).collect()
    };
    for (index, chunk) in chunks.iter().enumerate() {
        out.push(u8::from(index + 1 == chunks.len()));
        let len = chunk.len() as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(chunk);
    }
    out
}

// ------------------------------------------------------------- the builder

struct Spec {
    name: Vec<u8>,
    data: Vec<u8>,
    method: u16,
}

impl Spec {
    fn stored(name: &[u8], data: &[u8]) -> Spec {
        Spec {
            name: name.to_vec(),
            data: data.to_vec(),
            method: METHOD_STORE,
        }
    }

    fn deflated(name: &[u8], data: &[u8]) -> Spec {
        Spec {
            name: name.to_vec(),
            data: data.to_vec(),
            method: METHOD_DEFLATE,
        }
    }

    fn payload(&self) -> Vec<u8> {
        match self.method {
            METHOD_DEFLATE => deflate_stored(&self.data),
            _ => self.data.clone(),
        }
    }
}

fn u16_of(n: usize) -> [u8; 2] {
    (n as u16).to_le_bytes()
}

fn u32_of(n: usize) -> [u8; 4] {
    (n as u32).to_le_bytes()
}

/// Builds an archive from the specs, in order, with no descriptors, no extra
/// fields, no comment and no prefix.
fn build(specs: &[Spec]) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut offsets = Vec::new();
    for spec in specs {
        let payload = spec.payload();
        offsets.push(bytes.len());
        bytes.extend_from_slice(&LOCAL_SIG.to_le_bytes());
        bytes.extend_from_slice(&u16_of(20)); // version needed
        bytes.extend_from_slice(&u16_of(0)); // flags
        bytes.extend_from_slice(&spec.method.to_le_bytes());
        bytes.extend_from_slice(&u16_of(0)); // time
        bytes.extend_from_slice(&u16_of(0x21)); // date
        bytes.extend_from_slice(&crc32(&spec.data).to_le_bytes());
        bytes.extend_from_slice(&u32_of(payload.len()));
        bytes.extend_from_slice(&u32_of(spec.data.len()));
        bytes.extend_from_slice(&u16_of(spec.name.len()));
        bytes.extend_from_slice(&u16_of(0)); // extra length
        bytes.extend_from_slice(&spec.name);
        bytes.extend_from_slice(&payload);
    }

    let central_start = bytes.len();
    for (spec, offset) in specs.iter().zip(&offsets) {
        let payload = spec.payload();
        bytes.extend_from_slice(&CENTRAL_SIG.to_le_bytes());
        bytes.extend_from_slice(&u16_of(20)); // version made by
        bytes.extend_from_slice(&u16_of(20)); // version needed
        bytes.extend_from_slice(&u16_of(0)); // flags
        bytes.extend_from_slice(&spec.method.to_le_bytes());
        bytes.extend_from_slice(&u16_of(0));
        bytes.extend_from_slice(&u16_of(0x21));
        bytes.extend_from_slice(&crc32(&spec.data).to_le_bytes());
        bytes.extend_from_slice(&u32_of(payload.len()));
        bytes.extend_from_slice(&u32_of(spec.data.len()));
        bytes.extend_from_slice(&u16_of(spec.name.len()));
        bytes.extend_from_slice(&u16_of(0)); // extra
        bytes.extend_from_slice(&u16_of(0)); // comment
        bytes.extend_from_slice(&u16_of(0)); // disk
        bytes.extend_from_slice(&u16_of(0)); // internal attributes
        bytes.extend_from_slice(&u32_of(0)); // external attributes
        bytes.extend_from_slice(&u32_of(*offset));
        bytes.extend_from_slice(&spec.name);
    }
    let central_size = bytes.len() - central_start;

    bytes.extend_from_slice(&EOCD_SIG.to_le_bytes());
    bytes.extend_from_slice(&u16_of(0)); // this disk
    bytes.extend_from_slice(&u16_of(0)); // central directory disk
    bytes.extend_from_slice(&u16_of(specs.len()));
    bytes.extend_from_slice(&u16_of(specs.len()));
    bytes.extend_from_slice(&u32_of(central_size));
    bytes.extend_from_slice(&u32_of(central_start));
    bytes.extend_from_slice(&u16_of(0)); // comment length
    bytes
}

// -------------------------------------------------------- the independent reader

#[derive(Debug, Clone, PartialEq, Eq)]
struct OutEntry {
    name: Vec<u8>,
    method: u16,
    crc32: u32,
    uncompressed_size: u64,
    /// The entry's compressed bytes, as they sit in the file.
    compressed: Vec<u8>,
}

/// Reads an archive without the layer and without the `zip` crate.
///
/// Like the core's own independent parser, this SEARCHES for the central
/// directory position at which exactly the recorded number of records parse
/// and consume exactly the recorded size, rather than trusting the recorded
/// offset, so a mistake about the offset base cannot be shared.
fn parse_independently(bytes: &[u8]) -> Vec<OutEntry> {
    let u16at = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]);
    let u32at = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());

    let eocd = (0..=bytes.len().saturating_sub(22))
        .rev()
        .find(|&at| {
            bytes[at..at + 4] == EOCD_SIG.to_le_bytes()
                && at + 22 + usize::from(u16at(at + 20)) == bytes.len()
        })
        .expect("an end-of-central-directory record");
    let total = usize::from(u16at(eocd + 10));
    let central_size = u32at(eocd + 12) as usize;

    let central = (0..bytes.len())
        .find(|&start| {
            let mut at = start;
            for _ in 0..total {
                if at + CENTRAL_FIXED > bytes.len()
                    || bytes[at..at + 4] != CENTRAL_SIG.to_le_bytes()
                {
                    return false;
                }
                at += CENTRAL_FIXED
                    + usize::from(u16at(at + 28))
                    + usize::from(u16at(at + 30))
                    + usize::from(u16at(at + 32));
            }
            at == start + central_size
        })
        .expect("a central directory of the recorded size");

    // The offset base falls out of where the directory actually is.
    let base = central - u32at(eocd + 16) as usize;

    let mut entries = Vec::new();
    let mut at = central;
    for _ in 0..total {
        let name_len = usize::from(u16at(at + 28));
        let extra_len = usize::from(u16at(at + 30));
        let comment_len = usize::from(u16at(at + 32));
        let name = bytes[at + CENTRAL_FIXED..at + CENTRAL_FIXED + name_len].to_vec();
        let method = u16at(at + 10);
        let crc = u32at(at + 16);
        let compressed_size = u32at(at + 20) as usize;
        let uncompressed_size = u64::from(u32at(at + 24));
        let local = base + u32at(at + 42) as usize;

        assert_eq!(
            bytes[local..local + 4],
            LOCAL_SIG.to_le_bytes(),
            "no local header for {:?}",
            String::from_utf8_lossy(&name)
        );
        let local_name_len = usize::from(u16at(local + 26));
        let local_extra_len = usize::from(u16at(local + 28));
        let data = local + 30 + local_name_len + local_extra_len;
        entries.push(OutEntry {
            name,
            method,
            crc32: crc,
            uncompressed_size,
            compressed: bytes[data..data + compressed_size].to_vec(),
        });
        at += CENTRAL_FIXED + name_len + extra_len + comment_len;
    }
    entries
}

// ------------------------------------------------------------------ fixtures

/// A part holding the forms a corpus part holds, without a mark.
fn plain_part() -> Vec<u8> {
    let mut part = Vec::new();
    part.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n");
    part.extend_from_slice(b"<Types xmlns=\"http://example.invalid/content-types\">");
    part.extend_from_slice(b"<Default Extension=\"rels\" ContentType=\"application/rels\" />");
    part.extend_from_slice(b"<Default Extension=\"xml\" ContentType=\"application/xml\"/>");
    part.extend_from_slice(b"<Override PartName=\"/doc.xml\" ContentType=\"application/doc\"/>");
    part.extend_from_slice(b"<!--a comment--><?instruction data?>");
    part.extend_from_slice("<Text>&amp;&#65; \u{e9}\u{4e2d}</Text>".as_bytes());
    part.extend_from_slice(b"</Types>\n");
    part
}

/// The same with a byte order mark, as 158 of the corpus's parts have.
fn marked_part() -> Vec<u8> {
    let mut part = Vec::from(&BOM[..]);
    part.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    part.extend_from_slice(b"<Relationships xmlns=\"http://example.invalid/rels\">");
    part.extend_from_slice(b"<Relationship Id=\"rId1\" Type=\"http://example.invalid/t\"");
    part.extend_from_slice(b" Target=\"doc.xml\"/></Relationships>");
    part
}

/// Bytes that are not valid UTF-8, as a real binary part is not.
fn binary_part() -> Vec<u8> {
    let mut data = Vec::from(&b"\x89PNG\r\n\x1a\n"[..]);
    for n in 0..300u32 {
        data.extend_from_slice(&n.to_le_bytes());
        data.push(0xFF);
        data.push(0x80);
    }
    data
}

/// The archive P1 names: three XML parts and one binary part.
fn specs() -> Vec<Spec> {
    vec![
        Spec::deflated(b"[Content_Types].xml", &plain_part()),
        Spec::stored(b"_rels/.rels", &marked_part()),
        Spec::deflated(b"xl/media/image1.bin", &binary_part()),
        Spec::deflated(b"docProps/app.XML", &plain_part()),
    ]
}

// --------------------------------------------------------------------- tests

#[test]
fn the_null_rewrite_keeps_every_xml_part_and_leaves_the_binary_alone() {
    let dir = TempDir::new("rewrite");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let built = build(&specs());
    fs::write(&input, &built).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--xml-null-rewrite"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(ran.stderr, "", "the tool wrote to stderr");

    let before = parse_independently(&built);
    let after = parse_independently(&fs::read(&output).expect("read the output"));

    // Order and names are preserved.
    assert_eq!(
        after.iter().map(|e| e.name.clone()).collect::<Vec<_>>(),
        before.iter().map(|e| e.name.clone()).collect::<Vec<_>>()
    );

    for (was, now) in before.iter().zip(&after) {
        let name = String::from_utf8_lossy(&was.name).into_owned();
        let xml = name.to_ascii_lowercase().ends_with(".xml")
            || name.to_ascii_lowercase().ends_with(".rels");
        // A rewritten entry's DECOMPRESSED bytes are unchanged, which the
        // CRC-32 and the uncompressed size together pin; its compressed bytes
        // may differ, because the layer re-compresses a replacement.
        assert_eq!(now.crc32, was.crc32, "{name}: the CRC-32 changed");
        assert_eq!(
            now.uncompressed_size, was.uncompressed_size,
            "{name}: the uncompressed size changed"
        );
        assert_eq!(now.method, was.method, "{name}: the method changed");
        if !xml {
            // The binary entry was not touched at all, so even its stored
            // bytes are the input's.
            assert_eq!(
                now.compressed, was.compressed,
                "{name}: an untouched entry's bytes changed"
            );
        }
    }

    // The .XML name was rewritten too: the match is case-insensitive.
    let upper = after
        .iter()
        .find(|e| e.name == b"docProps/app.XML")
        .expect("the upper-case name");
    assert_eq!(upper.crc32, crc32(&plain_part()));

    // No temporary of the tool's own survived.
    assert_eq!(
        dir.names(),
        vec!["in.xlsx".to_string(), "out.xlsx".to_string()],
        "a temporary was left behind"
    );
}

#[test]
fn a_marked_part_keeps_its_mark_through_the_rewrite() {
    // The mark is in no token, so it is written back from `bom()` alone. If
    // that were dropped the CRC-32 would change; this pins the bytes.
    let dir = TempDir::new("mark");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    fs::write(&input, build(&specs())).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--xml-null-rewrite"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);

    let after = parse_independently(&fs::read(&output).expect("read the output"));
    let rels = after
        .iter()
        .find(|e| e.name == b"_rels/.rels")
        .expect("the rels part");
    let expected = marked_part();
    assert_eq!(rels.crc32, crc32(&expected));
    assert_eq!(rels.uncompressed_size, expected.len() as u64);
    // Stored in the input, so stored in the output, and its bytes ARE the
    // part: the mark is visible in them.
    assert_eq!(rels.compressed, expected);
    assert_eq!(&rels.compressed[..3], &BOM[..]);
}

#[test]
fn an_archive_with_no_xml_parts_comes_back_byte_identical() {
    let dir = TempDir::new("noxml");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let built = build(&[
        Spec::deflated(b"xl/media/image1.bin", &binary_part()),
        Spec::stored(b"mimetype", b"application/octet-stream"),
    ]);
    fs::write(&input, &built).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--xml-null-rewrite"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    // No entry was named for rewriting, so there were no edits at all.
    assert_eq!(fs::read(&output).expect("read the output"), built);
}

#[test]
fn a_part_that_is_not_xml_under_an_xml_name_is_refused() {
    // The rewrite parses what it rewrites, so a binary part misnamed .xml is
    // an error, not a silent copy. Exit 2, the message on stderr, and no
    // output left where the harness would read one.
    let dir = TempDir::new("badxml");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    fs::write(
        &input,
        build(&[Spec::deflated(b"xl/worksheets/sheet1.xml", &binary_part())]),
    )
    .expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--xml-null-rewrite"]);
    assert_eq!(ran.status, 2, "stderr: {}", ran.stderr);
    assert!(
        ran.stderr
            .starts_with("error: not well-formed XML or not UTF-8 (entry 0)"),
        "unexpected message: {}",
        ran.stderr
    );
    assert_eq!(
        dir.names(),
        vec!["in.xlsx".to_string()],
        "the refusal left a file behind"
    );
}

#[test]
fn the_operations_are_refused_unless_exactly_one_follows_the_paths() {
    // `specs/0009-package-layer.md`, "Harness runs": at most one operation per
    // invocation, given AFTER the two paths. Every other shape is the usage
    // error. negative.log runs these against the built binary as well.
    let dir = TempDir::new("usage");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    fs::write(&input, build(&specs())).expect("write the input");

    let cases: Vec<Vec<&str>> = vec![
        // Two operations at once, in both orders.
        vec!["--xml-null-rewrite", "--replace", "[Content_Types].xml"],
        vec!["--replace", "[Content_Types].xml", "--xml-null-rewrite"],
        // The same operation twice.
        vec!["--xml-null-rewrite", "--xml-null-rewrite"],
        // Two operations at once again, with the one P2 adds.
        vec!["--survey", "--xml-null-rewrite"],
        vec!["--xml-null-rewrite", "--survey"],
        // Two operations at once again, with the one P3 adds.
        vec!["--drop-calc-chain", "--survey"],
        vec!["--survey", "--drop-calc-chain"],
        // Two operations at once again, with part 3 P1's and P2's. P1 added
        // no case of its own for `--shared-strings`; both are here now, so
        // the list covers every operation the tool has.
        vec!["--shared-strings", "--survey"],
        vec!["--cells", "--shared-strings"],
        // And with part 4 P2's.
        vec!["--edit-sample", "--cells"],
        vec!["--cells", "--edit-sample"],
        vec!["--edit-sample", "[Content_Types].xml"],
        // And with part 4 P3's.
        vec!["--edit-formula-sample", "--edit-sample"],
        vec!["--edit-sample", "--edit-formula-sample"],
        vec!["--edit-formula-sample", "[Content_Types].xml"],
        // And with part 5 P1's.
        vec!["--styles", "--cells"],
        vec!["--cells", "--styles"],
        vec!["--styles", "[Content_Types].xml"],
        // A flag that takes no argument, given one.
        vec!["--drop-calc-chain", "[Content_Types].xml"],
        vec!["--cells", "[Content_Types].xml"],
        // An unknown operation.
        vec!["--xml-null"],
        vec!["--replace-all", "[Content_Types].xml"],
        // A flag that takes no argument, given one.
        vec!["--xml-null-rewrite", "[Content_Types].xml"],
        vec!["--survey", "[Content_Types].xml"],
        // An operation that needs a name, without one.
        vec!["--replace"],
    ];
    for case in cases {
        let ran = emend_copy_with(&input, &output, &case);
        assert_eq!(ran.status, 2, "{case:?} was not refused");
        assert!(
            ran.stderr.contains("usage: emend-copy"),
            "{case:?} was refused without the usage line: {}",
            ran.stderr
        );
        assert!(!output.exists(), "{case:?} produced an output");
    }

    // An operation BEFORE the paths is refused too.
    let ran = emend_copy(&[
        Path::new("--xml-null-rewrite"),
        input.as_path(),
        output.as_path(),
    ]);
    assert_eq!(ran.status, 2);
    assert!(ran.stderr.contains("usage: emend-copy"), "{}", ran.stderr);
    assert!(
        !output.exists(),
        "an operation before the paths produced an output"
    );

    // The control: exactly one operation after the paths is accepted.
    let ran = emend_copy_with(&input, &output, &["--xml-null-rewrite"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert!(output.exists(), "the accepted form produced no output");
}

#[test]
fn no_operation_is_still_a_verbatim_round_trip() {
    // Part 1's behaviour is unchanged by the new argument shape.
    let dir = TempDir::new("verbatim");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let built = build(&specs());
    fs::write(&input, &built).expect("write the input");

    let ran = emend_copy_with(&input, &output, &[]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(fs::read(&output).expect("read the output"), built);
}

#[test]
fn replace_is_unchanged() {
    // The other operation still replaces exactly one entry.
    let dir = TempDir::new("replace");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let built = build(&specs());
    fs::write(&input, &built).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--replace", "_rels/.rels"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);

    let before = parse_independently(&built);
    let after = parse_independently(&fs::read(&output).expect("read the output"));
    for (was, now) in before.iter().zip(&after) {
        assert_eq!(now.name, was.name);
        assert_eq!(now.crc32, was.crc32);
        assert_eq!(now.uncompressed_size, was.uncompressed_size);
    }
}

// --------------------------------------------------------------- the survey

/// The namespaces a real package writes, which the survey's fixture must use:
/// the layer knows these URIs and nothing else.
const TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const MAIN_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
const ATTR_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// The smallest package the survey can count, plus one untyped entry and one
/// external relationship so that every key it prints has something to count.
fn package_specs() -> Vec<Spec> {
    package_specs_with("")
}

/// The same package with `extra` more `Override` elements, so a test can make
/// an override name a part the archive does not hold.
fn package_specs_with(extra: &str) -> Vec<Spec> {
    let content_types = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
         <Types xmlns=\"{TYPES_NS}\">\
         <Default Extension=\"rels\" ContentType=\"application/rels\"/>\
         <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
         <Override PartName=\"/xl/workbook.xml\" ContentType=\"application/workbook\"/>\
         <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"application/sheet\"/>\
         {extra}</Types>"
    );
    let package_rels = format!(
        "<Relationships xmlns=\"{RELS_NS}\">\
         <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/officeDocument\" Target=\"xl/workbook.xml\"/>\
         </Relationships>"
    );
    let workbook = format!(
        "<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
         <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
    );
    let workbook_rels = format!(
        "<Relationships xmlns=\"{RELS_NS}\">\
         <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/worksheet\" Target=\"worksheets/sheet1.xml\"/>\
         <Relationship Id=\"rId2\" Type=\"{ATTR_NS}/styles\" Target=\"styles.xml\"/>\
         <Relationship Id=\"rId3\" Type=\"{ATTR_NS}/hyperlink\" \
         Target=\"https://example.invalid/x\" TargetMode=\"External\"/>\
         </Relationships>"
    );
    vec![
        Spec::deflated(b"[Content_Types].xml", content_types.as_bytes()),
        Spec::stored(b"_rels/.rels", package_rels.as_bytes()),
        Spec::deflated(b"xl/workbook.xml", workbook.as_bytes()),
        Spec::stored(b"xl/_rels/workbook.xml.rels", workbook_rels.as_bytes()),
        Spec::deflated(b"xl/worksheets/sheet1.xml", b"<sheetData/>"),
        Spec::deflated(b"xl/styles.xml", b"<styleSheet/>"),
        // Neither typed nor parsed: the survey reads only the parts it must.
        Spec::deflated(b"xl/media/image1.bin", &binary_part()),
    ]
}

#[test]
fn the_survey_counts_the_map_and_edits_nothing() {
    // `specs/0009-package-layer.md`, "Harness runs": one line, the word
    // `survey` then tab-separated key=value pairs, counts only, and the
    // output byte-identical because the operation makes no edit.
    let dir = TempDir::new("survey");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let bytes = build(&package_specs());
    fs::write(&input, &bytes).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--survey"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(
        ran.stdout,
        "survey\tentries=7\ttyped=5\tuntyped=1\tdefaults=2\toverrides=2\
         \toverrides_unresolved=0\trels_parts=2\
         \trelationships=4\texternal=1\tresolved=3\tunresolved=0\tcase_only=0\tsheets=1\
         \tsheets_resolved=1\tshared_strings=0\tstyles=1\tcalc_chain=0\ttheme=0\n",
        "the survey line"
    );
    assert_eq!(
        fs::read(&output).expect("read the output"),
        bytes,
        "the survey edits nothing"
    );
    assert_eq!(
        dir.names(),
        vec!["in.xlsx".to_string(), "out.xlsx".to_string()],
        "a temporary was left behind"
    );
}

#[test]
fn the_survey_counts_an_override_that_names_no_entry() {
    // `specs/0009-package-layer.md`, P2 ruling 11: the spec's fact that every
    // one of the corpus's 614 Override part names resolves exactly was
    // witnessed by no key, because `case_only` counts only the case-folded
    // near miss and an Override naming no entry at all was counted nowhere.
    // Two extra Overrides here: one part the archive does not hold in any
    // spelling, and one it holds only after ASCII folding. Both are
    // unresolved by exact bytes, so `overrides_unresolved` is 2, and only the
    // second is a fold match, so `case_only` stays 1 and keeps its meaning.
    let dir = TempDir::new("survey-overrides");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let bytes = build(&package_specs_with(
        "<Override PartName=\"/xl/nowhere.xml\" ContentType=\"application/absent\"/>\
         <Override PartName=\"/xl/STYLES.xml\" ContentType=\"application/folded\"/>",
    ));
    fs::write(&input, &bytes).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--survey"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(
        ran.stdout,
        "survey\tentries=7\ttyped=5\tuntyped=1\tdefaults=2\toverrides=4\
         \toverrides_unresolved=2\trels_parts=2\
         \trelationships=4\texternal=1\tresolved=3\tunresolved=0\tcase_only=1\tsheets=1\
         \tsheets_resolved=1\tshared_strings=0\tstyles=1\tcalc_chain=0\ttheme=0\n",
        "the survey line"
    );
    assert_eq!(
        fs::read(&output).expect("read the output"),
        bytes,
        "the survey still edits nothing"
    );
}

#[test]
fn the_survey_refuses_a_package_it_cannot_read() {
    // An error from any parse is exit 2 with the message, so the runner
    // records the run as an error outcome. P1's fixture is an archive of XML
    // parts but not a package: its content types root is in another
    // namespace.
    let dir = TempDir::new("survey-error");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    fs::write(&input, build(&specs())).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--survey"]);
    assert_eq!(ran.status, 2, "stderr: {}", ran.stderr);
    assert!(
        ran.stderr
            .contains("a content types part whose root is not Types"),
        "the message: {}",
        ran.stderr
    );
    assert!(ran.stdout.is_empty(), "no survey line: {}", ran.stdout);
    assert_eq!(
        dir.names(),
        vec!["in.xlsx".to_string()],
        "the refusal left a file behind"
    );
}

/// The same package with a calculation chain: the entry, its `Override` and
/// the workbook relationship that names it, which is what the recipe drops.
fn package_specs_with_a_chain() -> Vec<Spec> {
    let content_types = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
         <Types xmlns=\"{TYPES_NS}\">\
         <Default Extension=\"rels\" ContentType=\"application/rels\"/>\
         <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
         <Override PartName=\"/xl/workbook.xml\" ContentType=\"application/workbook\"/>\
         <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"application/sheet\"/>\
         <Override PartName=\"/xl/calcChain.xml\" ContentType=\"application/chain\"/>\
         </Types>"
    );
    let workbook_rels = format!(
        "<Relationships xmlns=\"{RELS_NS}\">\
         <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/worksheet\" Target=\"worksheets/sheet1.xml\"/>\
         <Relationship Id=\"rId2\" Type=\"{ATTR_NS}/styles\" Target=\"styles.xml\"/>\
         <Relationship Id=\"rId3\" Type=\"{ATTR_NS}/hyperlink\" \
         Target=\"https://example.invalid/x\" TargetMode=\"External\"/>\
         <Relationship Id=\"rId4\" Type=\"{ATTR_NS}/calcChain\" Target=\"calcChain.xml\"/>\
         </Relationships>"
    );
    let mut specs = package_specs();
    for spec in &mut specs {
        if spec.name == b"[Content_Types].xml" {
            spec.data = content_types.clone().into_bytes();
        }
        if spec.name == b"xl/_rels/workbook.xml.rels" {
            spec.data = workbook_rels.clone().into_bytes();
        }
    }
    specs.push(Spec::deflated(
        b"xl/calcChain.xml",
        b"<calcChain><c r=\"A1\" i=\"1\"/></calcChain>",
    ));
    specs
}

/// The entry of that name in a parsed output.
#[track_caller]
fn entry_named<'a>(entries: &'a [OutEntry], name: &[u8]) -> &'a OutEntry {
    entries
        .iter()
        .find(|entry| entry.name == name)
        .unwrap_or_else(|| panic!("no entry named {:?}", String::from_utf8_lossy(name)))
}

#[test]
fn the_recipe_drops_the_chain_and_leaves_every_other_entry_alone() {
    // `specs/0009-package-layer.md`, "Module `package`": three calls, the
    // relationship, the override and the entry. The two rewritten parts must
    // hold exactly the bytes this test builds by hand, and every other entry
    // must come back with the bytes it went in with.
    let dir = TempDir::new("chain");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let specs = package_specs_with_a_chain();
    fs::write(&input, build(&specs)).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--drop-calc-chain"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    let out_bytes = fs::read(&output).expect("read the output");
    let entries = parse_independently(&out_bytes);

    let got: Vec<String> = entries
        .iter()
        .map(|entry| String::from_utf8_lossy(&entry.name).into_owned())
        .collect();
    assert_eq!(
        got,
        vec![
            "[Content_Types].xml",
            "_rels/.rels",
            "xl/workbook.xml",
            "xl/_rels/workbook.xml.rels",
            "xl/worksheets/sheet1.xml",
            "xl/styles.xml",
            "xl/media/image1.bin",
        ],
        "the chain's entry is gone and nothing else is"
    );

    // The rewritten parts, by CRC-32 and size against bytes built here: the
    // element's own bytes gone, everything around them kept.
    let types_expected = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
         <Types xmlns=\"{TYPES_NS}\">\
         <Default Extension=\"rels\" ContentType=\"application/rels\"/>\
         <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
         <Override PartName=\"/xl/workbook.xml\" ContentType=\"application/workbook\"/>\
         <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"application/sheet\"/>\
         </Types>"
    );
    let rels_expected = format!(
        "<Relationships xmlns=\"{RELS_NS}\">\
         <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/worksheet\" Target=\"worksheets/sheet1.xml\"/>\
         <Relationship Id=\"rId2\" Type=\"{ATTR_NS}/styles\" Target=\"styles.xml\"/>\
         <Relationship Id=\"rId3\" Type=\"{ATTR_NS}/hyperlink\" \
         Target=\"https://example.invalid/x\" TargetMode=\"External\"/>\
         </Relationships>"
    );
    for (name, expected) in [
        (b"[Content_Types].xml".as_slice(), types_expected),
        (b"xl/_rels/workbook.xml.rels".as_slice(), rels_expected),
    ] {
        let entry = entry_named(&entries, name);
        assert_eq!(
            entry.uncompressed_size,
            expected.len() as u64,
            "the size of {:?}",
            String::from_utf8_lossy(name)
        );
        assert_eq!(
            entry.crc32,
            crc32(expected.as_bytes()),
            "the bytes of {:?}",
            String::from_utf8_lossy(name)
        );
    }

    // Every entry the recipe did not name: the input's own bytes, not merely
    // the same CRC-32.
    for spec in &specs {
        if spec.name == b"[Content_Types].xml"
            || spec.name == b"xl/_rels/workbook.xml.rels"
            || spec.name == b"xl/calcChain.xml"
        {
            continue;
        }
        let entry = entry_named(&entries, &spec.name);
        assert_eq!(
            entry.compressed,
            spec.payload(),
            "the entry {:?} was rewritten",
            String::from_utf8_lossy(&spec.name)
        );
    }

    assert_eq!(
        dir.names(),
        vec!["in.xlsx".to_string(), "out.xlsx".to_string()],
        "a temporary was left behind"
    );
}

#[test]
fn the_recipe_leaves_a_package_with_no_chain_byte_for_byte() {
    // The control for the operation: with no calcChain relationship the
    // recipe records no edit, and no edits is the verbatim round trip.
    let dir = TempDir::new("chain-absent");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let bytes = build(&package_specs());
    fs::write(&input, &bytes).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--drop-calc-chain"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(
        fs::read(&output).expect("read the output"),
        bytes,
        "a package with no chain came back changed"
    );
}

/// The same package with a `sharedStrings` part of two items and the
/// relationship and `Override` that make it reachable.
///
/// `specs/0010-reading-cells.md`, P1 ruling 8. The second item is written
/// with a CR LF pair, so the `bytes` figure below is what proves the line-end
/// normalisation of ruling 1 reaches the built binary: without it the pair
/// survives and `bytes` reads 9 rather than 8.
fn shared_strings_specs() -> Vec<Spec> {
    let content_types = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
         <Types xmlns=\"{TYPES_NS}\">\
         <Default Extension=\"rels\" ContentType=\"application/rels\"/>\
         <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
         <Override PartName=\"/xl/workbook.xml\" ContentType=\"application/workbook\"/>\
         <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"application/sheet\"/>\
         <Override PartName=\"/xl/sharedStrings.xml\" \
         ContentType=\"application/sharedStrings\"/>\
         </Types>"
    );
    let package_rels = format!(
        "<Relationships xmlns=\"{RELS_NS}\">\
         <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/officeDocument\" Target=\"xl/workbook.xml\"/>\
         </Relationships>"
    );
    let workbook = format!(
        "<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
         <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
    );
    let workbook_rels = format!(
        "<Relationships xmlns=\"{RELS_NS}\">\
         <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/worksheet\" Target=\"worksheets/sheet1.xml\"/>\
         <Relationship Id=\"rId2\" Type=\"{ATTR_NS}/sharedStrings\" Target=\"sharedStrings.xml\"/>\
         </Relationships>"
    );
    // "alpha" is five bytes; "a\r\nb" is four as written and three once the
    // pair is one LF. Two items, eight bytes.
    let strings = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
         <sst xmlns=\"{MAIN_NS}\" count=\"2\" uniqueCount=\"2\">\
         <si><t>alpha</t></si><si><t>a\r\nb</t></si></sst>"
    );
    vec![
        Spec::deflated(b"[Content_Types].xml", content_types.as_bytes()),
        Spec::stored(b"_rels/.rels", package_rels.as_bytes()),
        Spec::deflated(b"xl/workbook.xml", workbook.as_bytes()),
        Spec::stored(b"xl/_rels/workbook.xml.rels", workbook_rels.as_bytes()),
        Spec::deflated(b"xl/worksheets/sheet1.xml", b"<sheetData/>"),
        Spec::deflated(b"xl/sharedStrings.xml", strings.as_bytes()),
    ]
}

/// The same package with a worksheet the cell reader can read.
///
/// `specs/0010-reading-cells.md`, "Harness runs": one sheet, six cells, and
/// between them a value of every shape the counts distinguish, so the line
/// the test asserts is a measurement of this fixture and not a restatement
/// of the keys. `C2` is a `<c/>` with neither value nor formula, `A2` is a
/// shared master and `B2` its follower.
fn cells_specs() -> Vec<Spec> {
    let sheet = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
         <worksheet xmlns=\"{MAIN_NS}\"><sheetData>\
         <row r=\"1\"><c r=\"A1\" s=\"1\"><v>1.5</v></c>\
         <c r=\"B1\" t=\"s\"><v>1</v></c>\
         <c r=\"C1\" t=\"b\"><v>1</v></c></row>\
         <row r=\"2\"><c r=\"A2\"><f t=\"shared\" ref=\"A2:A3\" si=\"0\">A1*2</f><v>3</v></c>\
         <c r=\"B2\"><f t=\"shared\" si=\"0\"/><v>3</v></c>\
         <c r=\"C2\"/></row>\
         </sheetData></worksheet>"
    );
    let mut specs = shared_strings_specs();
    for spec in &mut specs {
        if spec.name == b"xl/worksheets/sheet1.xml" {
            *spec = Spec::deflated(b"xl/worksheets/sheet1.xml", sheet.as_bytes());
        }
    }
    specs
}

#[test]
fn cells_counts_every_cell_and_edits_nothing() {
    // `specs/0010-reading-cells.md`, "Harness runs": one line, the word
    // `cells` then tab-separated key=value pairs in the spec's own key order,
    // counts only, and the output byte-identical because the operation makes
    // no edit. A cell's value IS content, so no value and no shared string
    // appears in the line; `resolved=1` is how the table is shown to have
    // been read at all.
    let dir = TempDir::new("cells");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let bytes = build(&cells_specs());
    fs::write(&input, &bytes).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--cells"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(
        ran.stdout,
        "cells\tsheets=1\tcells=6\tnumber=4\tshared=1\tinline=0\tformula_string=0\
         \tboolean=1\terror=0\tdate=0\tvalues=5\tempty=1\tformulas=2\tshared_master=1\
         \tshared_follower=1\tarray=0\tdata_table=0\tnumbers_parsed=3\tbooleans_parsed=1\
         \tresolved=1\tunresolved=0\tmax_row=2\tmax_col=3\tstyled=1\n",
        "the cells line"
    );
    assert_eq!(
        fs::read(&output).expect("read the output"),
        bytes,
        "--cells edits nothing"
    );
    assert_eq!(
        dir.names(),
        vec!["in.xlsx".to_string(), "out.xlsx".to_string()],
        "a temporary was left behind"
    );
}

#[test]
fn shared_strings_counts_the_items_and_edits_nothing() {
    // `specs/0010-reading-cells.md`, "Harness runs" and P1 ruling 8: one
    // line, the word `strings` then tab-separated key=value pairs, counts
    // only, and the output byte-identical because the operation makes no
    // edit. Two counts and nothing else, because a shared string IS content:
    // no item, no length of a single one and no fragment of one is printed.
    let dir = TempDir::new("shared-strings");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let bytes = build(&shared_strings_specs());
    fs::write(&input, &bytes).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--shared-strings"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(
        ran.stdout, "strings\titems=2\tbytes=8\n",
        "the strings line"
    );
    assert_eq!(
        fs::read(&output).expect("read the output"),
        bytes,
        "--shared-strings edits nothing"
    );
    assert_eq!(
        dir.names(),
        vec!["in.xlsx".to_string(), "out.xlsx".to_string()],
        "a temporary was left behind"
    );
}

// ------------------------------------------------------- the sample edit

/// `cells_specs` with its worksheet replaced by `sheet_data` inside a
/// worksheet root, STORED, so the test can compare the rewritten part's
/// bytes (the layer keeps an entry's method) without an inflater.
fn edit_specs(sheet_data: &str) -> Vec<Spec> {
    let sheet = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
         <worksheet xmlns=\"{MAIN_NS}\"><dimension ref=\"A1\"/>{sheet_data}\
         <pageMargins left=\"0.7\"/></worksheet>"
    );
    let mut specs = cells_specs();
    for spec in &mut specs {
        if spec.name == b"xl/worksheets/sheet1.xml" {
            *spec = Spec::stored(b"xl/worksheets/sheet1.xml", sheet.as_bytes());
        }
    }
    specs
}

/// Runs `--edit-sample` on `specs` and returns the line and the output's
/// sheet part, asserting every other entry came back as it went in.
#[track_caller]
fn edit_sample(label: &str, specs: &[Spec]) -> (String, String) {
    let dir = TempDir::new(label);
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let bytes = build(specs);
    fs::write(&input, &bytes).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--edit-sample"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(
        dir.names(),
        vec!["in.xlsx".to_string(), "out.xlsx".to_string()],
        "a temporary was left behind"
    );
    let before = parse_independently(&bytes);
    let after = parse_independently(&fs::read(&output).expect("read the output"));
    assert_eq!(
        after.iter().map(|e| e.name.clone()).collect::<Vec<_>>(),
        before.iter().map(|e| e.name.clone()).collect::<Vec<_>>(),
        "the entry names and their order"
    );
    let mut sheet = None;
    for (was, now) in before.iter().zip(&after) {
        if now.name == b"xl/worksheets/sheet1.xml" {
            assert_eq!(now.method, 0, "the sheet stays stored");
            sheet = Some(String::from_utf8(now.compressed.clone()).expect("UTF-8"));
        } else {
            assert_eq!(
                (
                    now.method,
                    now.crc32,
                    now.uncompressed_size,
                    &now.compressed
                ),
                (
                    was.method,
                    was.crc32,
                    was.uncompressed_size,
                    &was.compressed
                ),
                "{} changed",
                String::from_utf8_lossy(&now.name)
            );
        }
    }
    (ran.stdout, sheet.expect("the sheet is in the output"))
}

/// The worksheet `edit_specs` builds, with `sheet_data` in it.
fn edited(sheet_data: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
         <worksheet xmlns=\"{MAIN_NS}\"><dimension ref=\"A1\"/>{sheet_data}\
         <pageMargins left=\"0.7\"/></worksheet>"
    )
}

/// The row the sample edit adds, numbered `row`.
fn added_row(row: u32) -> String {
    format!(
        "<row r=\"{row}\"><c r=\"A{row}\" t=\"inlineStr\"><is><t xml:space=\"preserve\">emend</t>\
         </is></c><c r=\"B{row}\" t=\"b\"><v>1</v></c></row>"
    )
}

#[test]
fn the_edit_sample_sets_clears_and_adds_a_row() {
    // `specs/0011-editing-cells.md`, "Harness runs": A is A1 (a number with
    // a value, styled), B is B1 (a shared string, unstyled, so its clear
    // drops it), C1 a boolean and A2, B2 formulas are not candidates, and M
    // is 2. Set 3, cleared 1, dropped 1, 6 cells before and 6 + 2 - 1 after,
    // and all three set values read back.
    let rows = "<row r=\"1\"><c r=\"A1\" s=\"1\"><v>1.5</v></c>\
                <c r=\"B1\" t=\"s\"><v>1</v></c>\
                <c r=\"C1\" t=\"b\"><v>1</v></c></row>\
                <row r=\"2\"><c r=\"A2\"><f t=\"shared\" ref=\"A2:A3\" si=\"0\">A1*2</f><v>3</v></c>\
                <c r=\"B2\"><f t=\"shared\" si=\"0\"/><v>3</v></c>\
                <c r=\"C2\"/></row>";
    let (line, sheet) = edit_sample(
        "edit-sample",
        &edit_specs(&format!("<sheetData>{rows}</sheetData>")),
    );
    assert_eq!(
        line,
        "edit\tset=3\tcleared=1\tdropped=1\trows_added=1\tcells_before=6\tcells_after=7\
         \tread_back=3\n",
        "the edit line"
    );
    let want = rows
        .replace("<v>1.5</v>", "<v>42.5</v>")
        .replace("<c r=\"B1\" t=\"s\"><v>1</v></c>", "");
    assert_eq!(
        sheet,
        edited(&format!("<sheetData>{want}{}</sheetData>", added_row(3))),
        "the sheet"
    );
}

#[test]
fn the_edit_sample_keeps_a_styled_second_cell() {
    // B styled: the clear keeps `<c r s/>`, so nothing is dropped and the
    // count grows by the two new cells alone.
    let rows =
        "<row r=\"4\"><c r=\"A4\" t=\"s\"><v>0</v></c><c r=\"B4\" s=\"2\"><v>7</v></c></row>";
    let (line, sheet) = edit_sample(
        "edit-sample-styled",
        &edit_specs(&format!("<sheetData>{rows}</sheetData>")),
    );
    assert_eq!(
        line,
        "edit\tset=3\tcleared=1\tdropped=0\trows_added=1\tcells_before=2\tcells_after=4\
         \tread_back=3\n",
        "the edit line"
    );
    assert_eq!(
        sheet,
        edited(&format!(
            "<sheetData><row r=\"4\"><c r=\"A4\"><v>42.5</v></c><c r=\"B4\" s=\"2\"/></row>{}\
             </sheetData>",
            added_row(5)
        )),
        "the sheet"
    );
}

#[test]
fn the_edit_sample_on_an_empty_sheet_adds_row_one() {
    // No cells: A and B absent, M 0, and the two new cells in row 1 of a
    // `<sheetData/>` that is opened up.
    let (line, sheet) = edit_sample("edit-sample-empty", &edit_specs("<sheetData/>"));
    assert_eq!(
        line,
        "edit\tset=2\tcleared=0\tdropped=0\trows_added=1\tcells_before=0\tcells_after=2\
         \tread_back=2\n",
        "the edit line"
    );
    assert_eq!(
        sheet,
        edited(&format!("<sheetData>{}</sheetData>", added_row(1))),
        "the sheet"
    );
}

#[test]
fn the_edit_sample_refuses_a_workbook_with_no_worksheet() {
    // The only sheet's relationship is a chartsheet: there is nothing to
    // edit, so the run is exit 2 and no output is left.
    let mut specs = edit_specs("<sheetData/>");
    for spec in &mut specs {
        if spec.name == b"xl/_rels/workbook.xml.rels" {
            let rels = String::from_utf8(spec.data.clone())
                .expect("UTF-8")
                .replace("/worksheet\"", "/chartsheet\"");
            *spec = Spec::stored(b"xl/_rels/workbook.xml.rels", rels.as_bytes());
        }
    }
    let dir = TempDir::new("edit-sample-none");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    fs::write(&input, build(&specs)).expect("write the input");
    let ran = emend_copy_with(&input, &output, &["--edit-sample"]);
    assert_eq!(ran.status, 2, "stdout: {}", ran.stdout);
    assert_eq!(
        ran.stderr, "error: the workbook has no sheet of kind Worksheet\n",
        "the message"
    );
    assert_eq!(ran.stdout, "", "a line was printed");
    assert_eq!(dir.names(), vec!["in.xlsx".to_string()], "a file was left");
}

// ----------------------------------------------- the formula sample edit

/// The worksheet the formula sample tests edit: a shared master and its
/// follower, which are not candidates, then two normal formulas, of which
/// the first, C2, is the one the sample sets.
const FORMULA_ROWS: &str = "<row r=\"1\"><c r=\"A1\" s=\"1\"><v>2</v></c>\
     <c r=\"B1\"><f t=\"shared\" ref=\"B1:B2\" si=\"0\">A1*2</f><v>4</v></c></row>\
     <row r=\"2\"><c r=\"B2\"><f t=\"shared\" si=\"0\"/><v>4</v></c>\
     <c r=\"C2\" s=\"3\"><f>A1+1</f><v>3</v></c><c r=\"D2\"><f>A1+2</f><v>4</v></c></row>";

/// `FORMULA_ROWS` as the sample writes them back: C2's formula gone, its
/// value 1 and its `s` kept, and nothing else changed.
fn formula_rows_edited() -> String {
    FORMULA_ROWS.replace(
        "<c r=\"C2\" s=\"3\"><f>A1+1</f><v>3</v></c>",
        "<c r=\"C2\" s=\"3\"><v>1</v></c>",
    )
}

/// Runs `--edit-formula-sample` on `specs` and returns the line and the
/// output's bytes, asserting exit 0 and that no temporary was left.
#[track_caller]
fn formula_sample(label: &str, specs: &[Spec]) -> (String, Vec<u8>) {
    let dir = TempDir::new(label);
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    fs::write(&input, build(specs)).expect("write the input");
    let ran = emend_copy_with(&input, &output, &["--edit-formula-sample"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(
        dir.names(),
        vec!["in.xlsx".to_string(), "out.xlsx".to_string()],
        "a temporary was left behind"
    );
    (ran.stdout, fs::read(&output).expect("read the output"))
}

/// Asserts that every entry of `specs` not in `changed` came back with the
/// bytes it went in with.
#[track_caller]
fn untouched_but(entries: &[OutEntry], specs: &[Spec], changed: &[&[u8]]) {
    for spec in specs {
        if changed.contains(&spec.name.as_slice()) {
            continue;
        }
        let entry = entry_named(entries, &spec.name);
        assert_eq!(
            (entry.method, &entry.compressed),
            (spec.method, &spec.payload()),
            "the entry {:?} was rewritten",
            String::from_utf8_lossy(&spec.name)
        );
    }
}

#[test]
fn the_formula_sample_replaces_the_first_normal_formula_and_drops_the_chain() {
    // `specs/0011-editing-cells.md`, "Harness runs": the shared master and
    // follower are skipped, C2 is set to 1, the chain goes in the same
    // write, and the sheet's five cells read back with C2 as a number and no
    // formula. The two parts the chain's removal rewrites must hold what
    // `--drop-calc-chain` writes for the same input.
    let sheet = edited(&format!("<sheetData>{FORMULA_ROWS}</sheetData>"));
    let mut specs = package_specs_with_a_chain();
    for spec in &mut specs {
        if spec.name == b"xl/worksheets/sheet1.xml" {
            *spec = Spec::stored(b"xl/worksheets/sheet1.xml", sheet.as_bytes());
        }
    }
    let (line, output) = formula_sample("formula-sample", &specs);
    assert_eq!(
        line, "edit\tformula_replaced=1\tchain_dropped=1\tcells_after=5\tread_back=1\n",
        "the edit line"
    );
    let entries = parse_independently(&output);
    let names: Vec<String> = entries
        .iter()
        .map(|entry| String::from_utf8_lossy(&entry.name).into_owned())
        .collect();
    assert_eq!(
        names,
        vec![
            "[Content_Types].xml",
            "_rels/.rels",
            "xl/workbook.xml",
            "xl/_rels/workbook.xml.rels",
            "xl/worksheets/sheet1.xml",
            "xl/styles.xml",
            "xl/media/image1.bin",
        ],
        "the chain's entry is gone and nothing else is"
    );
    assert_eq!(
        entry_named(&entries, b"xl/worksheets/sheet1.xml").compressed,
        edited(&format!("<sheetData>{}</sheetData>", formula_rows_edited())).into_bytes(),
        "the sheet"
    );

    let dir = TempDir::new("formula-sample-recipe");
    let input = dir.path("in.xlsx");
    let recipe = dir.path("out.xlsx");
    fs::write(&input, build(&specs)).expect("write the input");
    let ran = emend_copy_with(&input, &recipe, &["--drop-calc-chain"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    let recipe = parse_independently(&fs::read(&recipe).expect("read the recipe's output"));
    for name in [
        b"[Content_Types].xml".as_slice(),
        b"xl/_rels/workbook.xml.rels".as_slice(),
    ] {
        let (got, want) = (entry_named(&entries, name), entry_named(&recipe, name));
        assert_eq!(
            (got.method, got.crc32, &got.compressed),
            (want.method, want.crc32, &want.compressed),
            "{} differs from the recipe's",
            String::from_utf8_lossy(name)
        );
    }
    untouched_but(
        &entries,
        &specs,
        &[
            b"[Content_Types].xml",
            b"xl/_rels/workbook.xml.rels",
            b"xl/worksheets/sheet1.xml",
            b"xl/calcChain.xml",
        ],
    );
}

#[test]
fn the_formula_sample_reads_on_to_a_later_worksheet_and_keeps_an_absent_chain() {
    // The first worksheet has only a shared formula, so the second is
    // edited; there is no chain, so it alone changes and chain_dropped is 0.
    // The first sheet's count is not the one the line carries: the second
    // sheet's is.
    let second = edited(&format!("<sheetData>{FORMULA_ROWS}</sheetData>"));
    let mut specs = edit_specs(
        "<sheetData><row r=\"1\"><c r=\"A1\"><f t=\"shared\" ref=\"A1:A2\" si=\"0\">1+1</f>\
         <v>2</v></c></row><row r=\"2\"><c r=\"A2\"><f t=\"shared\" si=\"0\"/><v>2</v></c>\
         </row></sheetData>",
    );
    for spec in &mut specs {
        let text = String::from_utf8(spec.data.clone()).unwrap_or_default();
        if spec.name == b"xl/workbook.xml" {
            let text = text.replace(
                "</sheets>",
                "<sheet name=\"Sheet2\" sheetId=\"2\" r:id=\"rId9\"/></sheets>",
            );
            *spec = Spec::deflated(b"xl/workbook.xml", text.as_bytes());
        }
        if spec.name == b"xl/_rels/workbook.xml.rels" {
            let text = text.replace(
                "</Relationships>",
                &format!(
                    "<Relationship Id=\"rId9\" Type=\"{ATTR_NS}/worksheet\" \
                     Target=\"worksheets/sheet2.xml\"/></Relationships>"
                ),
            );
            *spec = Spec::stored(b"xl/_rels/workbook.xml.rels", text.as_bytes());
        }
    }
    specs.push(Spec::stored(b"xl/worksheets/sheet2.xml", second.as_bytes()));
    let (line, output) = formula_sample("formula-sample-second", &specs);
    assert_eq!(
        line, "edit\tformula_replaced=1\tchain_dropped=0\tcells_after=5\tread_back=1\n",
        "the edit line"
    );
    let entries = parse_independently(&output);
    assert_eq!(entries.len(), specs.len(), "an entry was added or dropped");
    assert_eq!(
        entry_named(&entries, b"xl/worksheets/sheet2.xml").compressed,
        edited(&format!("<sheetData>{}</sheetData>", formula_rows_edited())).into_bytes(),
        "the second sheet"
    );
    untouched_but(&entries, &specs, &[b"xl/worksheets/sheet2.xml"]);
}

#[test]
fn the_formula_sample_with_no_normal_formula_is_byte_identical() {
    // `cells_specs` holds a shared master and its follower and no normal
    // formula: no edit, the output the input's own bytes, and every key 0,
    // where a cells_after of 0 means no sheet was read (P3 ruling 4).
    let specs = cells_specs();
    let (line, output) = formula_sample("formula-sample-none", &specs);
    assert_eq!(
        line, "edit\tformula_replaced=0\tchain_dropped=0\tcells_after=0\tread_back=0\n",
        "the edit line"
    );
    assert_eq!(output, build(&specs), "the output is not the input");
}

#[test]
fn the_formula_sample_refuses_what_the_write_refuses_and_leaves_nothing() {
    // The first normal formula's cell carries `cm`, which the write refuses:
    // exit 2, the layer's message, no line, and no output or temporary left.
    let rows = FORMULA_ROWS.replace("<c r=\"C2\" s=\"3\">", "<c r=\"C2\" s=\"3\" cm=\"1\">");
    let specs = edit_specs(&format!("<sheetData>{rows}</sheetData>"));
    let dir = TempDir::new("formula-sample-refused");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    fs::write(&input, build(&specs)).expect("write the input");
    let ran = emend_copy_with(&input, &output, &["--edit-formula-sample"]);
    assert_eq!(ran.status, 2, "stdout: {}", ran.stdout);
    assert_eq!(
        ran.stderr, "error: a cell carries an attribute this layer does not model\n",
        "the message"
    );
    assert_eq!(ran.stdout, "", "a line was printed");
    assert_eq!(dir.names(), vec!["in.xlsx".to_string()], "a file was left");
}

// ------------------------------------------------------------- --styles

/// The survey's package with a workbook that counts from 1904 and a styles
/// part the reader can read (`specs/0012-styles.md`, "Harness runs").
///
/// Ten cell formats, one per case the `dated` count distinguishes: a
/// built-in General (0), a built-in date not spelled out (14), a custom
/// number (164), a custom date (165), a date letter inside a quoted run
/// (166), inside a bracketed run (167) and after a backslash (168), a date
/// letter after a quote that never closes (169), a producer's spelling of a
/// built-in percentage (9), and no `numFmtId` at all. The codes are
/// fictional. Hand count: `xfs` 10, `num_fmts` 7, `custom` 6, `dated` 3
/// (14, 165 and 169), `date1904` 1.
fn styles_specs() -> Vec<Spec> {
    let workbook = format!(
        "<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><workbookPr date1904=\"1\"/>\
         <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
    );
    let styles = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
         <styleSheet xmlns=\"{MAIN_NS}\"><numFmts count=\"7\">\
         <numFmt numFmtId=\"9\" formatCode=\"0%\"/>\
         <numFmt numFmtId=\"164\" formatCode=\"0.00\"/>\
         <numFmt numFmtId=\"165\" formatCode=\"yyyy-mm-dd\"/>\
         <numFmt numFmtId=\"166\" formatCode=\"&quot;day &quot;0\"/>\
         <numFmt numFmtId=\"167\" formatCode=\"[Red]0\"/>\
         <numFmt numFmtId=\"168\" formatCode=\"\\d0\"/>\
         <numFmt numFmtId=\"169\" formatCode=\"0&quot;y\"/></numFmts>\
         <fonts count=\"1\"><font><name val=\"Fictional\"/></font></fonts>\
         <cellXfs count=\"10\"><xf numFmtId=\"0\"/><xf numFmtId=\"14\"/>\
         <xf numFmtId=\"164\"/><xf numFmtId=\"165\"/><xf numFmtId=\"166\"/>\
         <xf numFmtId=\"167\"/><xf numFmtId=\"168\"/><xf numFmtId=\"169\"/>\
         <xf numFmtId=\"9\"/><xf fontId=\"0\"/></cellXfs></styleSheet>"
    );
    let mut specs = package_specs();
    for spec in &mut specs {
        if spec.name == b"xl/workbook.xml" {
            *spec = Spec::deflated(b"xl/workbook.xml", workbook.as_bytes());
        } else if spec.name == b"xl/styles.xml" {
            *spec = Spec::deflated(b"xl/styles.xml", styles.as_bytes());
        }
    }
    specs
}

#[test]
fn styles_counts_the_styles_part_and_edits_nothing() {
    // `specs/0012-styles.md`, "Harness runs": one line, the word `styles`
    // then tab-separated key=value pairs in the spec's key order, counts
    // only, and the output byte-identical because the operation makes no
    // edit.
    let dir = TempDir::new("styles");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    let bytes = build(&styles_specs());
    fs::write(&input, &bytes).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--styles"]);
    assert_eq!(ran.status, 0, "stderr: {}", ran.stderr);
    assert_eq!(
        ran.stdout, "styles\txfs=10\tnum_fmts=7\tcustom=6\tdated=3\tdate1904=1\n",
        "the styles line"
    );
    assert_eq!(
        fs::read(&output).expect("read the output"),
        bytes,
        "--styles edits nothing"
    );
    assert_eq!(
        dir.names(),
        vec!["in.xlsx".to_string(), "out.xlsx".to_string()],
        "a temporary was left behind"
    );
}

#[test]
fn styles_refuses_a_styles_part_it_cannot_read_and_leaves_nothing() {
    // The survey's own package carries `<styleSheet/>` in no namespace, which
    // the reader refuses by its root: exit 2, the message, no line, no file.
    let dir = TempDir::new("styles-refused");
    let input = dir.path("in.xlsx");
    let output = dir.path("out.xlsx");
    fs::write(&input, build(&package_specs())).expect("write the input");

    let ran = emend_copy_with(&input, &output, &["--styles"]);
    assert_eq!(ran.status, 2, "stdout: {}", ran.stdout);
    assert_eq!(
        ran.stderr, "error: the styles part's root is not styleSheet\n",
        "the message"
    );
    assert_eq!(ran.stdout, "", "a line was printed");
    assert_eq!(dir.names(), vec!["in.xlsx".to_string()], "a file was left");
}
