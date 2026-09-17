//! Byte-built archives for the zip layer.
//!
//! `specs/0008-zip-layer.md`, "P1: open, index, verbatim write": every test
//! asserts the OUTPUT BYTES, not this crate's reading of them. A round trip
//! that only re-opened would pass even if the layer rewrote every header, so
//! each round-trip test compares the written archive with the input byte for
//! byte.
//!
//! Nothing here uses a library to build a fixture. The crate has no
//! dev-dependencies (`specs/0008`, "Crate"), so a bug in a builder cannot be
//! the same bug as one in the layer. Deflate is produced as a run of STORED
//! blocks, which is valid deflate and needs no encoder; CRC-32 is computed
//! bitwise below.

use std::io::{Cursor, Read};

use super::*;

// ---------------------------------------------------------------- helpers

/// CRC-32, bitwise, so the tests carry their own and do not borrow the
/// layer's or a crate's.
fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

/// A valid deflate stream made of stored blocks. RFC 1951 section 3.2.4: a
/// block header of one byte (bit 0 final, bits 1-2 zero for stored), then
/// LEN and its one's complement, then the raw bytes.
fn deflate_stored(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let chunks: Vec<&[u8]> = if data.is_empty() {
        vec![&[][..]]
    } else {
        data.chunks(0xFFFF).collect()
    };
    for (index, chunk) in chunks.iter().enumerate() {
        let last = index + 1 == chunks.len();
        out.push(u8::from(last));
        let len = chunk.len() as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(chunk);
    }
    out
}

const METHOD_STORE: u16 = 0;
const METHOD_DEFLATE: u16 = 8;

/// How one entry is laid out in a fixture.
#[derive(Clone)]
struct Spec {
    name: Vec<u8>,
    data: Vec<u8>,
    method: u16,
    /// `Some(true)` a descriptor with its signature, `Some(false)` without.
    descriptor: Option<bool>,
    local_extra: Vec<u8>,
    central_extra: Vec<u8>,
    comment: Vec<u8>,
    /// Bytes of padding written after this entry's span.
    padding: usize,
    /// Record the sizes and the offset as zip64 markers plus a zip64 extra
    /// field, without building a large archive.
    zip64: bool,
    /// Extra general purpose bits, for the encrypted case.
    gp_extra: u16,
    /// A CRC-32 to record in place of the data's own. The layer never checks
    /// a CRC on the paths these tests reach, so this is how a fixture puts a
    /// chosen value where a reader will look at it.
    recorded_crc: Option<u32>,
}

impl Spec {
    fn new(name: &[u8], data: &[u8]) -> Self {
        Spec {
            name: name.to_vec(),
            data: data.to_vec(),
            method: METHOD_STORE,
            descriptor: None,
            local_extra: Vec::new(),
            central_extra: Vec::new(),
            comment: Vec::new(),
            padding: 0,
            zip64: false,
            gp_extra: 0,
            recorded_crc: None,
        }
    }

    /// The CRC-32 the fixture records: the override, or the data's own.
    fn crc(&self) -> u32 {
        self.recorded_crc.unwrap_or_else(|| crc32(&self.data))
    }

    fn deflated(mut self) -> Self {
        self.method = METHOD_DEFLATE;
        self
    }

    fn with_descriptor(mut self, signature: bool) -> Self {
        self.descriptor = Some(signature);
        self
    }

    fn payload(&self) -> Vec<u8> {
        match self.method {
            METHOD_DEFLATE => deflate_stored(&self.data),
            _ => self.data.clone(),
        }
    }

    fn gp_flags(&self) -> u16 {
        let mut flags = self.gp_extra;
        if self.descriptor.is_some() {
            flags |= GP_DESCRIPTOR;
        }
        flags
    }
}

/// Builds a complete archive: a prefix, every entry's span in the given file
/// order, the central directory in `central_order`, and the end records.
struct Build {
    prefix: Vec<u8>,
    specs: Vec<Spec>,
    /// Indices into `specs`, in central directory order. Defaults to file
    /// order; T1 P6 showed the two can differ.
    central_order: Option<Vec<usize>>,
    comment: Vec<u8>,
    zip64_end: bool,
    /// Written into the end-of-central-directory record's disk fields.
    disk: u16,
    /// Whether the recorded offsets COUNT THE PREFIX (file-absolute) or not
    /// (relative to the archive proper). Both conventions are in use, and a
    /// fixture must state which it builds rather than inherit whatever the
    /// implementation happens to do: an earlier version of this builder wrote
    /// the entry offsets relative and the zip64 locator absolute, so it
    /// encoded the layer's own mixture and no test could fail on the
    /// difference. That is how the silent prefix loss survived
    /// (`specs/0008-zip-layer.md`, the P1 rulings of 2026-09-17, item 3).
    absolute_offsets: bool,
}

impl Build {
    fn new(specs: Vec<Spec>) -> Self {
        Build {
            prefix: Vec::new(),
            specs,
            central_order: None,
            comment: Vec::new(),
            zip64_end: false,
            disk: 0,
            absolute_offsets: false,
        }
    }

    fn bytes(&self) -> Vec<u8> {
        let mut out = self.prefix.clone();
        // The one value every recorded offset is measured from. With
        // `absolute_offsets` the producer counts the prefix, so the base is
        // 0; otherwise offsets are relative to the archive proper and the
        // base is the prefix length. Every recorded quantity below goes
        // through it: the local header offsets, the zip64 extra field's
        // offset, the central directory's offset in both end records, and
        // the locator's record offset.
        let base = if self.absolute_offsets {
            0
        } else {
            self.prefix.len() as u64
        };
        let mut offsets = vec![0u64; self.specs.len()];

        for (index, spec) in self.specs.iter().enumerate() {
            offsets[index] = out.len() as u64 - base;
            let payload = spec.payload();
            let crc = spec.crc();
            let sizes_in_header = spec.descriptor.is_none();

            out.extend_from_slice(&LOCAL_SIG.to_le_bytes());
            out.extend_from_slice(&20u16.to_le_bytes()); // version needed
            out.extend_from_slice(&spec.gp_flags().to_le_bytes());
            out.extend_from_slice(&spec.method.to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes()); // time
            out.extend_from_slice(&0u16.to_le_bytes()); // date
            out.extend_from_slice(&if sizes_in_header { crc } else { 0 }.to_le_bytes());
            out.extend_from_slice(
                &if sizes_in_header {
                    payload.len() as u32
                } else {
                    0
                }
                .to_le_bytes(),
            );
            out.extend_from_slice(
                &if sizes_in_header {
                    spec.data.len() as u32
                } else {
                    0
                }
                .to_le_bytes(),
            );
            out.extend_from_slice(&(spec.name.len() as u16).to_le_bytes());
            out.extend_from_slice(&(spec.local_extra.len() as u16).to_le_bytes());
            out.extend_from_slice(&spec.name);
            out.extend_from_slice(&spec.local_extra);
            out.extend_from_slice(&payload);

            if let Some(with_signature) = spec.descriptor {
                if with_signature {
                    out.extend_from_slice(&0x0807_4b50u32.to_le_bytes());
                }
                out.extend_from_slice(&crc.to_le_bytes());
                // A zip64 entry's descriptor carries 8-byte sizes. Writing 4
                // regardless would build a fixture no reader agrees with, and
                // the layer reads the form from the central record's markers,
                // so the two would disagree on where the descriptor ends.
                if spec.zip64 {
                    out.extend_from_slice(&(payload.len() as u64).to_le_bytes());
                    out.extend_from_slice(&(spec.data.len() as u64).to_le_bytes());
                } else {
                    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
                    out.extend_from_slice(&(spec.data.len() as u32).to_le_bytes());
                }
            }
            out.extend(std::iter::repeat_n(0xAAu8, spec.padding));
        }

        let central_start = out.len() as u64;
        let order: Vec<usize> = self
            .central_order
            .clone()
            .unwrap_or_else(|| (0..self.specs.len()).collect());

        for &index in &order {
            let spec = &self.specs[index];
            let payload = spec.payload();
            let crc = spec.crc();

            let mut extra = spec.central_extra.clone();
            let (csize, usize_field, offset_field) = if spec.zip64 {
                let mut field = Vec::new();
                field.extend_from_slice(&ZIP64_EXTRA_ID.to_le_bytes());
                field.extend_from_slice(&24u16.to_le_bytes());
                field.extend_from_slice(&(spec.data.len() as u64).to_le_bytes());
                field.extend_from_slice(&(payload.len() as u64).to_le_bytes());
                field.extend_from_slice(&offsets[index].to_le_bytes());
                extra.extend_from_slice(&field);
                (U32_MAX_MARKER, U32_MAX_MARKER, U32_MAX_MARKER)
            } else {
                (
                    payload.len() as u32,
                    spec.data.len() as u32,
                    offsets[index] as u32,
                )
            };

            out.extend_from_slice(&CENTRAL_SIG.to_le_bytes());
            out.extend_from_slice(&0x031Eu16.to_le_bytes()); // version made by
            out.extend_from_slice(&20u16.to_le_bytes()); // version needed
            out.extend_from_slice(&spec.gp_flags().to_le_bytes());
            out.extend_from_slice(&spec.method.to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes()); // time
            out.extend_from_slice(&0u16.to_le_bytes()); // date
            out.extend_from_slice(&crc.to_le_bytes());
            out.extend_from_slice(&csize.to_le_bytes());
            out.extend_from_slice(&usize_field.to_le_bytes());
            out.extend_from_slice(&(spec.name.len() as u16).to_le_bytes());
            out.extend_from_slice(&(extra.len() as u16).to_le_bytes());
            out.extend_from_slice(&(spec.comment.len() as u16).to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes()); // disk start
            out.extend_from_slice(&0u16.to_le_bytes()); // internal attributes
            out.extend_from_slice(&0u32.to_le_bytes()); // external attributes
            out.extend_from_slice(&offset_field.to_le_bytes());
            out.extend_from_slice(&spec.name);
            out.extend_from_slice(&extra);
            out.extend_from_slice(&spec.comment);
        }
        let central_size = out.len() as u64 - central_start;
        let central_offset = central_start - base;
        let total = self.specs.len() as u64;

        if self.zip64_end {
            let record_position = out.len() as u64;
            out.extend_from_slice(&ZIP64_EOCD_SIG.to_le_bytes());
            out.extend_from_slice(&(ZIP64_EOCD_FIXED - 12).to_le_bytes());
            out.extend_from_slice(&0x031Eu16.to_le_bytes());
            out.extend_from_slice(&45u16.to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes()); // disk
            out.extend_from_slice(&0u32.to_le_bytes()); // central start disk
            out.extend_from_slice(&total.to_le_bytes());
            out.extend_from_slice(&total.to_le_bytes());
            out.extend_from_slice(&central_size.to_le_bytes());
            out.extend_from_slice(&central_offset.to_le_bytes());

            out.extend_from_slice(&ZIP64_LOCATOR_SIG.to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes());
            // Through the base, like every other recorded offset. Writing the
            // absolute position here regardless was what made the locator
            // fixture agree with the implementation whatever it did.
            out.extend_from_slice(&(record_position - base).to_le_bytes());
            out.extend_from_slice(&1u32.to_le_bytes());
        }

        out.extend_from_slice(&EOCD_SIG.to_le_bytes());
        out.extend_from_slice(&self.disk.to_le_bytes());
        out.extend_from_slice(&self.disk.to_le_bytes());
        out.extend_from_slice(&(total as u16).to_le_bytes());
        out.extend_from_slice(&(total as u16).to_le_bytes());
        out.extend_from_slice(&(central_size as u32).to_le_bytes());
        out.extend_from_slice(&(central_offset as u32).to_le_bytes());
        out.extend_from_slice(&(self.comment.len() as u16).to_le_bytes());
        out.extend_from_slice(&self.comment);
        out
    }
}

/// Opens the bytes, writes them back with no edits, and returns the output.
fn round_trip(input: &[u8]) -> Vec<u8> {
    let mut archive = Archive::open(Cursor::new(input.to_vec())).expect("open");
    let mut out = Vec::new();
    archive.write_to(&mut out, Edits::new()).expect("write_to");
    out
}

/// The assertion every round-trip test makes: the output bytes ARE the input
/// bytes. Reports the first difference rather than dumping either archive.
fn assert_identical(input: &[u8], output: &[u8]) {
    if input == output {
        return;
    }
    let first = input
        .iter()
        .zip(output.iter())
        .position(|(a, b)| a != b)
        .unwrap_or(input.len().min(output.len()));
    panic!(
        "output differs: input {} bytes, output {} bytes, first difference at {}",
        input.len(),
        output.len(),
        first
    );
}

fn check_round_trip(build: &Build) {
    let input = build.bytes();
    let output = round_trip(&input);
    assert_identical(&input, &output);
}

/// The error from an archive that must be refused.
///
/// `expect_err` would need `Archive` to implement `Debug`, and the compiler
/// suggests deriving it. That would shape a public type around a test's
/// convenience and pull a `R: Debug` bound and the `zip` crate's internals
/// into the API, so the test carries this instead.
fn open_err(bytes: Vec<u8>) -> Error {
    match Archive::open(Cursor::new(bytes)) {
        Ok(_) => panic!("the archive was accepted; it must be refused"),
        Err(e) => e,
    }
}

// ------------------------------------------------------------ round trips

#[test]
fn stored_entries_round_trip() {
    let build = Build::new(vec![
        Spec::new(b"one.txt", b"first entry"),
        Spec::new(b"two.txt", b"second entry"),
    ]);
    check_round_trip(&build);
}

#[test]
fn deflated_entries_round_trip() {
    let build = Build::new(vec![
        Spec::new(b"one.txt", b"first entry").deflated(),
        Spec::new(b"two.txt", b"second entry").deflated(),
    ]);
    check_round_trip(&build);
}

#[test]
fn descriptor_with_signature_round_trips() {
    let build = Build::new(vec![
        Spec::new(b"d.txt", b"descriptor payload").with_descriptor(true),
        Spec::new(b"plain.txt", b"no descriptor"),
    ]);
    check_round_trip(&build);
}

#[test]
fn descriptor_without_signature_round_trips() {
    let build = Build::new(vec![
        Spec::new(b"d.txt", b"descriptor payload").with_descriptor(false),
        Spec::new(b"plain.txt", b"no descriptor"),
    ]);
    check_round_trip(&build);
}

#[test]
fn zip64_markers_round_trip() {
    let mut spec = Spec::new(b"big.txt", b"not actually big");
    spec.zip64 = true;
    let mut build = Build::new(vec![spec, Spec::new(b"small.txt", b"plain")]);
    build.zip64_end = true;
    check_round_trip(&build);
}

#[test]
fn a_zip64_archive_has_no_prefix() {
    // The value that was actually wrong when this layer first met a zip64
    // archive. `open` took the central directory to end where the
    // end-of-central-directory record begins, but the zip64 record and its
    // locator sit between the two, so prefix_len came out 76 rather than 0
    // and every span_start was shifted by it. The round trip above would
    // pass again once the offsets happened to line up; this pins the cause.
    let mut spec = Spec::new(b"big.txt", b"not actually big");
    spec.zip64 = true;
    let mut build = Build::new(vec![spec, Spec::new(b"small.txt", b"plain")]);
    build.zip64_end = true;
    let archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    assert_eq!(archive.prefix_len(), 0);
    assert_eq!(archive.entries().len(), 2);
    assert_eq!(archive.entries()[0].uncompressed_size(), 16);
}

#[test]
fn a_zip64_archive_with_a_prefix_round_trips() {
    // A zip64 archive behind a real prefix, in the RELATIVE convention only.
    // An earlier comment here claimed this case stopped the zip64 and prefix
    // corrections cancelling; it does not, because it builds one convention.
    // The four cases below cover both, which is what pins them.
    let mut spec = Spec::new(b"big.txt", b"behind a stub");
    spec.zip64 = true;
    let mut build = Build::new(vec![spec, Spec::new(b"small.txt", b"plain")]);
    build.zip64_end = true;
    build.prefix = b"MZ stub bytes before the archive".to_vec();
    let input = build.bytes();
    let archive = Archive::open(Cursor::new(input.clone())).expect("open");
    assert_eq!(archive.prefix_len(), build.prefix.len() as u64);
    check_round_trip(&build);
}

#[test]
fn a_prefix_round_trips() {
    let mut build = Build::new(vec![Spec::new(b"one.txt", b"after a stub")]);
    build.prefix = b"MZ this is a self-extracting stub".to_vec();
    check_round_trip(&build);
}

#[test]
fn comments_round_trip() {
    let mut spec = Spec::new(b"one.txt", b"commented");
    spec.comment = b"an entry comment".to_vec();
    let mut build = Build::new(vec![spec, Spec::new(b"two.txt", b"plain")]);
    build.comment = b"an archive comment".to_vec();
    check_round_trip(&build);
}

#[test]
fn unknown_extra_fields_round_trip() {
    // Header id 0x5566 is not one the layer knows; both headers carry one,
    // and they differ so a swap would show.
    let mut spec = Spec::new(b"one.txt", b"extras");
    spec.local_extra = vec![0x66, 0x55, 0x04, 0x00, 1, 2, 3, 4];
    spec.central_extra = vec![0x66, 0x55, 0x02, 0x00, 9, 9];
    check_round_trip(&Build::new(vec![spec, Spec::new(b"two.txt", b"plain")]));
}

#[test]
fn file_order_differing_from_central_order_round_trips() {
    let mut build = Build::new(vec![
        Spec::new(b"first-in-file.txt", b"a"),
        Spec::new(b"second-in-file.txt", b"b"),
        Spec::new(b"third-in-file.txt", b"c"),
    ]);
    build.central_order = Some(vec![2, 0, 1]);
    check_round_trip(&build);
}

#[test]
fn padding_between_entries_round_trips() {
    let mut first = Spec::new(b"one.txt", b"padded after");
    first.padding = 7;
    check_round_trip(&Build::new(vec![first, Spec::new(b"two.txt", b"plain")]));
}

#[test]
fn a_non_ascii_name_round_trips() {
    check_round_trip(&Build::new(vec![Spec::new(
        "caf\u{e9}-\u{2014}.txt".as_bytes(),
        b"named in UTF-8",
    )]));
}

#[test]
fn a_directory_entry_round_trips() {
    check_round_trip(&Build::new(vec![
        Spec::new(b"dir/", b""),
        Spec::new(b"dir/one.txt", b"inside"),
    ]));
}

#[test]
fn an_empty_archive_round_trips() {
    check_round_trip(&Build::new(Vec::new()));
}

#[test]
fn an_archive_comment_on_an_empty_archive_round_trips() {
    let mut build = Build::new(Vec::new());
    build.comment = b"comment with no entries".to_vec();
    check_round_trip(&build);
}

// ------------------------------------------- both offset conventions

/// A stub, then two entries, in the convention the caller names.
///
/// `specs/0008-zip-layer.md`, the P1 rulings of 2026-09-17, item 3: fixtures
/// are built in BOTH conventions from an explicit parameter, never from what
/// the implementation happens to do. The defect these pin dropped the prefix
/// silently at exit 0, because `prefix_len` was derived as
/// `central_start - central_offset`, which is 0 when a producer records
/// file-absolute offsets.
fn check_behind_a_stub(zip64: bool, absolute: bool) {
    let mut first = Spec::new(b"one.txt", b"behind a stub");
    first.zip64 = zip64;
    let mut build = Build::new(vec![first, Spec::new(b"two.txt", b"plain")]);
    build.zip64_end = zip64;
    build.prefix = b"MZ stub bytes before the archive".to_vec();
    build.absolute_offsets = absolute;

    let input = build.bytes();
    let archive = Archive::open(Cursor::new(input.clone())).expect("open");
    // The value the defect got wrong. It is the bytes before the first local
    // header, in either convention.
    assert_eq!(archive.prefix_len(), build.prefix.len() as u64);
    let output = round_trip(&input);
    assert_identical(&input, &output);
}

#[test]
fn a_plain_archive_behind_a_stub_with_relative_offsets_round_trips() {
    check_behind_a_stub(false, false);
}

#[test]
fn a_plain_archive_behind_a_stub_with_absolute_offsets_round_trips() {
    check_behind_a_stub(false, true);
}

#[test]
fn a_zip64_archive_behind_a_stub_with_relative_offsets_round_trips() {
    check_behind_a_stub(true, false);
}

#[test]
fn a_zip64_archive_behind_a_stub_with_absolute_offsets_round_trips() {
    check_behind_a_stub(true, true);
}

#[test]
fn an_empty_archive_behind_a_stub_round_trips_in_both_conventions() {
    for absolute in [false, true] {
        let mut build = Build::new(Vec::new());
        build.prefix = b"MZ stub".to_vec();
        build.absolute_offsets = absolute;
        let input = build.bytes();
        let archive = Archive::open(Cursor::new(input.clone())).expect("open");
        // With no entries there is no local header, so everything up to the
        // central directory is prefix.
        assert_eq!(
            archive.prefix_len(),
            build.prefix.len() as u64,
            "absolute_offsets = {absolute}"
        );
        let output = round_trip(&input);
        assert_identical(&input, &output);
    }
}

// --------------------------------------------------------------- the index

#[test]
fn the_index_reports_what_the_records_say() {
    let build = Build::new(vec![
        Spec::new(b"one.txt", b"first entry").deflated(),
        Spec::new(b"dir/", b""),
        Spec::new(b"three.txt", b"third").with_descriptor(true),
    ]);
    let input = build.bytes();
    let archive = Archive::open(Cursor::new(input)).expect("open");

    assert_eq!(archive.entries().len(), 3);
    assert_eq!(archive.index_of(b"three.txt"), Some(2));
    assert_eq!(archive.index_of(b"absent.txt"), None);
    assert_eq!(archive.prefix_len(), 0);

    let first = &archive.entries()[0];
    assert_eq!(first.name(), b"one.txt");
    assert_eq!(first.method(), METHOD_DEFLATE);
    assert_eq!(first.uncompressed_size(), b"first entry".len() as u64);
    assert_eq!(first.crc32(), crc32(b"first entry"));
    assert!(!first.is_dir());
    assert!(!first.has_descriptor());
    assert!(!first.is_encrypted());

    assert!(archive.entries()[1].is_dir());
    assert!(archive.entries()[2].has_descriptor());
}

#[test]
fn reading_an_entry_gives_its_bytes_back() {
    let build = Build::new(vec![
        Spec::new(b"stored.txt", b"stored bytes"),
        Spec::new(b"deflated.txt", b"deflated bytes").deflated(),
    ]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");

    let mut got = Vec::new();
    // The read's own result is asserted, not discarded: with `.ok()` a read
    // error inside the test was invisible and only the value assertion stood
    // (independent check 2 of e14dbee; the P1 rulings of 2026-09-17, item 4).
    let n = archive
        .read(0)
        .expect("read 0")
        .read_to_end(&mut got)
        .expect("read 0 to the end");
    assert_eq!(n, b"stored bytes".len());
    assert_eq!(got, b"stored bytes");

    let mut got = Vec::new();
    let n = archive
        .read(1)
        .expect("read 1")
        .read_to_end(&mut got)
        .expect("read 1 to the end");
    assert_eq!(n, b"deflated bytes".len());
    assert_eq!(got, b"deflated bytes");
}

#[test]
fn a_write_after_a_read_still_round_trips() {
    // write_to takes the reader out of the ZipArchive and puts it back; a
    // read afterwards must still work, and a second write must match the
    // first. This is the one place the Option could strand the reader.
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload")]);
    let input = build.bytes();
    let mut archive = Archive::open(Cursor::new(input.clone())).expect("open");

    let mut first = Vec::new();
    archive.write_to(&mut first, Edits::new()).expect("first");
    let mut bytes = Vec::new();
    archive
        .read(0)
        .expect("read")
        .read_to_end(&mut bytes)
        .expect("read to the end");
    assert_eq!(bytes, b"payload");
    let mut second = Vec::new();
    archive.write_to(&mut second, Edits::new()).expect("second");

    assert_identical(&input, &first);
    assert_identical(&input, &second);
}

// ---------------------------------------------------------------- refusals

#[test]
fn not_a_zip_is_refused() {
    let err = open_err(vec![0u8; 512]);
    assert!(matches!(err, Error::NotZip), "got {err:?}");
}

#[test]
fn a_file_too_short_for_a_record_is_refused() {
    let err = open_err(vec![0u8; 4]);
    assert!(matches!(err, Error::NotZip), "got {err:?}");
}

#[test]
fn a_local_offset_beyond_the_file_is_refused() {
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload")]);
    let mut input = build.bytes();
    // The central record's local header offset field, patched to a position
    // past the end of the file.
    let central = input
        .windows(4)
        .position(|w| w == CENTRAL_SIG.to_le_bytes())
        .expect("a central record");
    input[central + 42..central + 46].copy_from_slice(&0x0000_7FFFu32.to_le_bytes());
    let err = open_err(input);
    assert!(matches!(err, Error::Truncated), "got {err:?}");
}

#[test]
fn duplicate_names_are_refused() {
    let build = Build::new(vec![
        Spec::new(b"same.txt", b"first"),
        Spec::new(b"same.txt", b"second"),
    ]);
    let err = open_err(build.bytes());
    assert!(matches!(err, Error::DuplicateName(1)), "got {err:?}");
}

#[test]
fn a_multi_disk_record_is_refused() {
    let mut build = Build::new(vec![Spec::new(b"one.txt", b"payload")]);
    build.disk = 1;
    let err = open_err(build.bytes());
    assert_eq!(unsupported_message(err), "a multi-disk archive");
}

#[test]
fn reading_an_encrypted_entry_is_refused() {
    let mut spec = Spec::new(b"secret.txt", b"not really encrypted");
    spec.gp_extra = GP_ENCRYPTED;
    let build = Build::new(vec![spec]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    assert!(archive.entries()[0].is_encrypted());
    let err = archive.read(0).err().expect("must refuse");
    assert_eq!(unsupported_message(err), "reading an encrypted entry");
}

#[test]
fn an_absent_index_is_refused() {
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload")]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let err = archive.read(9).err().expect("must refuse");
    assert!(matches!(err, Error::NoSuchEntry(9)), "got {err:?}");
}

/// A writer that takes `limit` bytes and then fails, so `write_to` stops
/// part-way through the spans.
struct FailingWriter {
    written: usize,
    limit: usize,
}

impl std::io::Write for FailingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.written >= self.limit {
            return Err(std::io::Error::other("the writer stopped"));
        }
        let take = buf.len().min(self.limit - self.written);
        self.written += take;
        Ok(take)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn a_failed_write_leaves_a_usable_archive() {
    // `specs/0008-zip-layer.md`, the P1 rulings of 2026-09-17, item 6: after
    // a write that fails part-way, every later call returns Err or works,
    // never panics, and this test says which. Both of them WORK: write_to
    // puts the reader back on the failure path as well as the success path,
    // so neither Option can strand it.
    //
    // An earlier version of this comment cited "Planning's pin", which was a
    // cross-session message and not a primary. Both independent checks of
    // e14dbee found it. A planning message is an instruction, never a source.
    let build = Build::new(vec![
        Spec::new(b"one.txt", b"payload one"),
        Spec::new(b"two.txt", b"payload two"),
    ]);
    let input = build.bytes();
    let mut archive = Archive::open(Cursor::new(input.clone())).expect("open");

    let mut failing = FailingWriter {
        written: 0,
        limit: 20,
    };
    let err = archive
        .write_to(&mut failing, Edits::new())
        .expect_err("the write must fail");
    assert!(matches!(err, Error::Io(_)), "got {err:?}");

    // Works, not Err: the entry still reads.
    let mut bytes = Vec::new();
    archive
        .read(0)
        .expect("read works after a failed write")
        .read_to_end(&mut bytes)
        .expect("read to the end after a failed write");
    assert_eq!(bytes, b"payload one");

    // Works, not Err: a second write still produces the input byte for byte.
    let mut out = Vec::new();
    archive
        .write_to(&mut out, Edits::new())
        .expect("write works after a failed write");
    assert_identical(&input, &out);
}

// ------------------------------------------------- an independent reader

/// One entry as the independent reader sees it.
#[derive(Debug, Clone)]
struct OutEntry {
    name: Vec<u8>,
    method: u16,
    crc32: u32,
    compressed_size: u64,
    uncompressed_size: u64,
    local_offset: u64,
    gp_flags: u16,
}

/// What the independent reader makes of an archive.
#[derive(Debug)]
struct Parsed {
    /// What a recorded offset is added to in order to get a file position.
    base: u64,
    /// Where the central directory actually starts.
    central_position: u64,
    entries: Vec<OutEntry>,
}

/// Parses an archive without using the layer.
///
/// `specs/0008-zip-layer.md`, "P2: replace, remove, add": an edit test's
/// output is read back by "an independent reader ... that shares no code with
/// the layer". This shares no code AND no method: rather than deriving the
/// central directory's position from the end records, as `archive.rs` does, it
/// SEARCHES for a position at which exactly `total` records parse and consume
/// exactly `central_size` bytes. A mistake in the layer's end-record handling
/// therefore cannot be the same mistake here, and the offset base falls out of
/// the found position rather than being assumed.
fn parse_independently(bytes: &[u8]) -> Parsed {
    let eocd = (0..=bytes.len().saturating_sub(22))
        .rev()
        .find(|&at| {
            bytes[at..at + 4] == EOCD_SIG.to_le_bytes()
                && at + 22 + u16::from_le_bytes([bytes[at + 20], bytes[at + 21]]) as usize
                    == bytes.len()
        })
        .expect("an end-of-central-directory record");
    let u16at = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]);
    let u32at = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
    let u64at = |at: usize| u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap());

    // Zip64 values win where the 32-bit field carries the marker. The zip64
    // record is found by searching too, not by the locator's offset.
    let mut total = u64::from(u16at(eocd + 10));
    let mut central_size = u64::from(u32at(eocd + 12));
    let mut recorded_offset = u64::from(u32at(eocd + 16));
    if let Some(at) = (0..eocd)
        .rev()
        .find(|&at| bytes[at..at + 4] == ZIP64_EOCD_SIG.to_le_bytes())
    {
        total = u64at(at + 32);
        central_size = u64at(at + 40);
        recorded_offset = u64at(at + 48);
    }

    // The position at which exactly `total` records parse and end after
    // exactly `central_size` bytes.
    let central_position = (0..bytes.len())
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
            at == start + central_size as usize
        })
        .expect("a central directory of the recorded size");

    let mut entries = Vec::new();
    let mut at = central_position;
    for _ in 0..total {
        let name_len = usize::from(u16at(at + 28));
        let extra_len = usize::from(u16at(at + 30));
        let comment_len = usize::from(u16at(at + 32));
        let name = bytes[at + CENTRAL_FIXED..at + CENTRAL_FIXED + name_len].to_vec();
        let (raw_compressed, raw_uncompressed, raw_offset) =
            (u32at(at + 20), u32at(at + 24), u32at(at + 42));

        // The zip64 extra field supplies whichever fields carry the marker, in
        // this order and only those.
        let extra_start = at + CENTRAL_FIXED + name_len;
        let mut zip64 = Vec::new();
        let mut scan = extra_start;
        while scan + 4 <= extra_start + extra_len {
            let id = u16at(scan);
            let size = usize::from(u16at(scan + 2));
            if id == ZIP64_EXTRA_ID {
                let mut cursor = scan + 4;
                while cursor + 8 <= scan + 4 + size {
                    zip64.push(u64at(cursor));
                    cursor += 8;
                }
                break;
            }
            scan += 4 + size;
        }
        let mut next = zip64.into_iter();
        let uncompressed_size = if raw_uncompressed == U32_MAX_MARKER {
            next.next().expect("a zip64 uncompressed size")
        } else {
            u64::from(raw_uncompressed)
        };
        let compressed_size = if raw_compressed == U32_MAX_MARKER {
            next.next().expect("a zip64 compressed size")
        } else {
            u64::from(raw_compressed)
        };
        let local_offset = if raw_offset == U32_MAX_MARKER {
            next.next().expect("a zip64 local header offset")
        } else {
            u64::from(raw_offset)
        };

        entries.push(OutEntry {
            name,
            method: u16at(at + 10),
            crc32: u32at(at + 16),
            compressed_size,
            uncompressed_size,
            local_offset,
            gp_flags: u16at(at + 8),
        });
        at += CENTRAL_FIXED + name_len + extra_len + comment_len;
    }

    let base = central_position as u64 - recorded_offset;
    // Every recorded local offset must land on a local header once the base is
    // applied: that is what proves the output kept the input's convention.
    for entry in &entries {
        let at = (base + entry.local_offset) as usize;
        assert_eq!(
            bytes[at..at + 4],
            LOCAL_SIG.to_le_bytes(),
            "no local header at the recorded offset for {:?}",
            String::from_utf8_lossy(&entry.name)
        );
    }
    Parsed {
        base,
        central_position: central_position as u64,
        entries,
    }
}

/// The entry names an archive holds, in central directory order.
fn names(parsed: &Parsed) -> Vec<Vec<u8>> {
    parsed.entries.iter().map(|e| e.name.clone()).collect()
}

/// A replacement that writes these bytes.
fn bytes_replacement(data: &[u8]) -> Replacement {
    let data = data.to_vec();
    Box::new(move |out: &mut dyn Write| out.write_all(&data))
}

/// Serializes every test that makes a replacement temporary.
///
/// `cargo test` runs these in threads of ONE process, so they share both the
/// temporary directory and the process id in the temporary's name. Without
/// this, the snapshot in `a_replacement_error_leaves_no_temporary_behind`
/// catches another test's file in flight and the test fails on a race rather
/// than on the layer. Every replacement in these tests goes through `apply`
/// or holds this lock itself; the refusal tests make no temporary, because
/// `plan` rejects them before any compression starts.
static REPLACEMENT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Opens the input, applies the edits, and returns the output bytes.
fn apply(input: &[u8], edits: Edits) -> Vec<u8> {
    let _guard = REPLACEMENT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut archive = Archive::open(Cursor::new(input.to_vec())).expect("open");
    let mut out = Vec::new();
    archive.write_to(&mut out, edits).expect("write_to");
    out
}

// ------------------------------------------------------ replace, remove, add

#[test]
fn a_deflated_entry_is_replaced_and_the_others_are_untouched() {
    let build = Build::new(vec![
        Spec::new(b"one.txt", b"first entry").deflated(),
        Spec::new(b"two.txt", b"second entry").deflated(),
    ]);
    let input = build.bytes();
    let before = parse_independently(&input);

    let mut edits = Edits::new();
    edits.replace(
        0,
        bytes_replacement(b"a much longer first entry than before"),
    );
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    assert_eq!(names(&after), names(&before));
    // The replaced entry: new CRC-32 and sizes, the original's method.
    assert_eq!(after.entries[0].method, METHOD_DEFLATE);
    assert_eq!(
        after.entries[0].crc32,
        crc32(b"a much longer first entry than before")
    );
    assert_eq!(
        after.entries[0].uncompressed_size,
        b"a much longer first entry than before".len() as u64
    );
    // The untouched entry: every recorded value the same, and its bytes moved
    // only by the change in the first entry's length.
    assert_eq!(after.entries[1].crc32, before.entries[1].crc32);
    assert_eq!(
        after.entries[1].compressed_size,
        before.entries[1].compressed_size
    );
    assert_eq!(after.base, before.base, "the offset convention changed");
}

#[test]
fn a_stored_entry_is_replaced_stored() {
    let build = Build::new(vec![
        Spec::new(b"stored.txt", b"stored bytes"),
        Spec::new(b"after.txt", b"unchanged"),
    ]);
    let input = build.bytes();

    let mut edits = Edits::new();
    edits.replace(0, bytes_replacement(b"different stored bytes"));
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    assert_eq!(after.entries[0].method, METHOD_STORE);
    assert_eq!(after.entries[0].crc32, crc32(b"different stored bytes"));
    // Stored: the compressed size IS the uncompressed size.
    assert_eq!(
        after.entries[0].compressed_size,
        b"different stored bytes".len() as u64
    );
    assert_eq!(
        after.entries[0].uncompressed_size,
        b"different stored bytes".len() as u64
    );
}

#[test]
fn a_replacement_may_come_from_a_reader() {
    // The `io::copy` wrapper of `specs/0008-zip-layer.md`, "Module `archive`".
    let build = Build::new(vec![Spec::new(b"one.txt", b"first").deflated()]);
    let input = build.bytes();

    let mut edits = Edits::new();
    edits.replace(
        0,
        replacement_from_reader(Cursor::new(b"from a reader".to_vec())),
    );
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    assert_eq!(after.entries[0].crc32, crc32(b"from a reader"));
    assert_eq!(
        after.entries[0].uncompressed_size,
        b"from a reader".len() as u64
    );
}

#[test]
fn a_removed_entry_leaves_the_others_intact() {
    let build = Build::new(vec![
        Spec::new(b"one.txt", b"first"),
        Spec::new(b"two.txt", b"second"),
        Spec::new(b"three.txt", b"third"),
    ]);
    let input = build.bytes();

    let mut edits = Edits::new();
    edits.remove(1);
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    assert_eq!(
        names(&after),
        vec![b"one.txt".to_vec(), b"three.txt".to_vec()]
    );
    assert_eq!(after.entries[1].crc32, crc32(b"third"));
    // The survivor's span is shorter by exactly the removed entry's span.
    assert!(output.len() < input.len());
}

#[test]
fn an_added_entry_is_appended() {
    let build = Build::new(vec![Spec::new(b"one.txt", b"first")]);
    let input = build.bytes();

    let mut edits = Edits::new();
    edits.add(b"added.txt".to_vec(), bytes_replacement(b"brand new bytes"));
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    assert_eq!(
        names(&after),
        vec![b"one.txt".to_vec(), b"added.txt".to_vec()]
    );
    let added = &after.entries[1];
    assert_eq!(added.method, METHOD_DEFLATE);
    assert_eq!(added.crc32, crc32(b"brand new bytes"));
    assert_eq!(added.uncompressed_size, b"brand new bytes".len() as u64);
    // Flags 0: no data descriptor, so the sizes stand in the header.
    assert_eq!(added.gp_flags, 0);
    // Appended after the last entry in file order.
    assert!(added.local_offset > after.entries[0].local_offset);
}

#[test]
fn order_is_preserved_around_a_removed_and_an_added_entry() {
    let build = Build::new(vec![
        Spec::new(b"one.txt", b"first"),
        Spec::new(b"two.txt", b"second"),
        Spec::new(b"three.txt", b"third"),
    ]);
    let input = build.bytes();

    let mut edits = Edits::new();
    edits.remove(1);
    edits.add(b"added.txt".to_vec(), bytes_replacement(b"new"));
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    assert_eq!(
        names(&after),
        vec![
            b"one.txt".to_vec(),
            b"three.txt".to_vec(),
            b"added.txt".to_vec()
        ]
    );
    // Central directory order and file order agree, which is what
    // `order_preserved` reports.
    let offsets: Vec<u64> = after.entries.iter().map(|e| e.local_offset).collect();
    let mut sorted = offsets.clone();
    sorted.sort_unstable();
    assert_eq!(offsets, sorted);
}

// ---------------------------------------------------------- edit refusals

#[test]
fn two_changes_for_one_entry_are_refused() {
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload")]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut edits = Edits::new();
    edits.replace(0, bytes_replacement(b"new"));
    edits.remove(0);
    let mut out = Vec::new();
    let err = archive.write_to(&mut out, edits).expect_err("must refuse");
    assert_eq!(unsupported_message(err), "two changes for one entry");
    // Refused BEFORE anything was written.
    assert!(out.is_empty(), "the output was written to anyway");
}

#[test]
fn an_edit_to_an_absent_index_is_refused() {
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload")]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut edits = Edits::new();
    edits.remove(7);
    let mut out = Vec::new();
    let err = archive.write_to(&mut out, edits).expect_err("must refuse");
    assert!(matches!(err, Error::NoSuchEntry(7)), "got {err:?}");
    assert!(out.is_empty());
}

#[test]
fn adding_a_name_the_archive_already_has_is_refused() {
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload")]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut edits = Edits::new();
    edits.add(b"one.txt".to_vec(), bytes_replacement(b"new"));
    let mut out = Vec::new();
    let err = archive.write_to(&mut out, edits).expect_err("must refuse");
    assert!(matches!(err, Error::DuplicateName(_)), "got {err:?}");
}

#[test]
fn replacing_an_encrypted_entry_is_refused() {
    let mut spec = Spec::new(b"secret.txt", b"not really encrypted");
    spec.gp_extra = GP_ENCRYPTED;
    let build = Build::new(vec![spec]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut edits = Edits::new();
    edits.replace(0, bytes_replacement(b"new"));
    let mut out = Vec::new();
    let err = archive.write_to(&mut out, edits).expect_err("must refuse");
    assert_eq!(unsupported_message(err), "replacing an encrypted entry");
}

#[test]
fn a_replacement_error_leaves_no_temporary_behind() {
    // `specs/0008-zip-layer.md`, "P2": "a replacement error leaves no output
    // file"; the P2 rulings of 2026-09-17, item 4: the test "also asserts that
    // no temporary of that name form remains". The layer writes to a `Write`,
    // not to a path, so what it owns is the temporary; `emend-copy` owns the
    // output file and negative.log covers that.
    let _guard = REPLACEMENT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload").deflated()]);
    let before = replacement_temporaries();

    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut edits = Edits::new();
    edits.replace(
        0,
        Box::new(|_: &mut dyn Write| Err(io::Error::other("the replacement failed"))),
    );
    let mut out = Vec::new();
    let err = archive
        .write_to(&mut out, edits)
        .expect_err("the write must fail");
    assert!(matches!(err, Error::Io(_)), "got {err:?}");

    assert_eq!(
        replacement_temporaries(),
        before,
        "a replacement temporary was left in the temporary directory"
    );
}

/// The layer's replacement temporaries belonging to this process, by name.
fn replacement_temporaries() -> Vec<std::ffi::OsString> {
    let prefix = format!("emend-replace-{}-", std::process::id());
    let mut found: Vec<std::ffi::OsString> = std::fs::read_dir(env::temp_dir())
        .expect("read the temporary directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name())
        .filter(|name| name.to_string_lossy().starts_with(&prefix))
        .collect();
    found.sort();
    found
}

// ------------------------------------------------ the replaced entry's form

fn at_u16(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(bytes[at..at + 2].try_into().unwrap())
}

fn at_u32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap())
}

/// Where an entry's compressed data starts in the output.
fn data_start_of(bytes: &[u8], parsed: &Parsed, index: usize) -> usize {
    let at = (parsed.base + parsed.entries[index].local_offset) as usize;
    at + usize_of(LOCAL_FIXED).unwrap()
        + usize::from(at_u16(bytes, at + 26))
        + usize::from(at_u16(bytes, at + 28))
}

/// Replacing an entry that uses a data descriptor keeps the descriptor in the
/// form the original used, with ONLY the CRC-32 and the two size fields
/// rewritten.
///
/// `specs/0008-zip-layer.md`, the P2 rulings of 2026-09-17, item 7(a): this is
/// the evidence that only those fields change, "for both signature forms and
/// both size widths". The width is 8 bytes when the entry's central record
/// states its sizes as zip64 markers, and 4 otherwise. Item 7 is why the
/// harness flags such an entry `descriptor` and why that one flag is the
/// design's requirement rather than a finding.
fn check_descriptor_kept(signature: bool, zip64: bool) {
    let old_data: &[u8] = b"descriptor payload";
    let new_data: &[u8] = b"a different descriptor payload, longer than before";
    let mut first = Spec::new(b"d.txt", old_data).with_descriptor(signature);
    first.zip64 = zip64;
    let mut build = Build::new(vec![first, Spec::new(b"plain.txt", b"no descriptor")]);
    build.zip64_end = zip64;

    let input = build.bytes();
    let before = parse_independently(&input);

    let mut edits = Edits::new();
    edits.replace(0, bytes_replacement(new_data));
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    // The flag is the original's, so the descriptor is still declared.
    assert_eq!(after.entries[0].gp_flags & GP_DESCRIPTOR, GP_DESCRIPTOR);
    assert_eq!(after.entries[0].crc32, crc32(new_data));
    assert_eq!(after.entries[0].uncompressed_size, new_data.len() as u64);

    // The local header still carries zeros for the CRC-32 and both sizes:
    // with a descriptor those fields hold no value to update.
    let at = (after.base + after.entries[0].local_offset) as usize;
    assert_eq!(
        &output[at + 14..at + 26],
        &[0u8; 12],
        "local header not zeroed"
    );

    // Both descriptors as bytes, at the width the original used.
    let width = if zip64 { 8 } else { 4 };
    let fields = if signature { 4 } else { 0 };
    let length = fields + 4 + 2 * width;
    let old_at = data_start_of(&input, &before, 0) + before.entries[0].compressed_size as usize;
    let new_at = data_start_of(&output, &after, 0) + after.entries[0].compressed_size as usize;
    let old = &input[old_at..old_at + length];
    let new = &output[new_at..new_at + length];

    // The form is the original's: the signature is kept when it was there and
    // is not invented when it was not.
    if signature {
        assert_eq!(at_u32(new, 0), DESCRIPTOR_SIG, "the signature was not kept");
    } else {
        assert_ne!(
            at_u32(new, 0),
            DESCRIPTOR_SIG,
            "a signature was added that the original did not have"
        );
    }
    // Everything outside the CRC-32 and the two sizes is byte-identical, which
    // for a descriptor is the signature when there is one.
    assert_eq!(
        &old[..fields],
        &new[..fields],
        "a byte outside the rewritten fields changed"
    );
    assert_eq!(old.len(), new.len(), "the descriptor changed width");

    let value = |bytes: &[u8], at: usize| -> u64 {
        if zip64 {
            u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap())
        } else {
            u64::from(at_u32(bytes, at))
        }
    };

    // The output's fields carry the NEW values.
    assert_eq!(at_u32(new, fields), crc32(new_data), "descriptor CRC-32");
    assert_eq!(
        value(new, fields + 4),
        after.entries[0].compressed_size,
        "descriptor compressed size"
    );
    assert_eq!(
        value(new, fields + 4 + width),
        new_data.len() as u64,
        "descriptor uncompressed size"
    );
    // And the input's held the OLD ones in the same places, so what is shown
    // above is a rewrite of those fields and not a descriptor of another shape.
    assert_eq!(at_u32(old, fields), crc32(old_data), "the input's CRC-32");
    assert_eq!(
        value(old, fields + 4),
        before.entries[0].compressed_size,
        "the input's compressed size"
    );
    assert_eq!(
        value(old, fields + 4 + width),
        old_data.len() as u64,
        "the input's uncompressed size"
    );
}

#[test]
fn a_replaced_entry_keeps_its_descriptor_with_a_signature() {
    check_descriptor_kept(true, false);
}

#[test]
fn a_replaced_entry_keeps_its_descriptor_without_a_signature() {
    check_descriptor_kept(false, false);
}

#[test]
fn a_replaced_entry_keeps_its_zip64_descriptor_with_a_signature() {
    check_descriptor_kept(true, true);
}

#[test]
fn a_replaced_entry_keeps_its_zip64_descriptor_without_a_signature() {
    check_descriptor_kept(false, true);
}

#[test]
fn a_replaced_zip64_entry_keeps_its_markers() {
    let mut spec = Spec::new(b"big.txt", b"not actually big");
    spec.zip64 = true;
    let mut build = Build::new(vec![spec, Spec::new(b"small.txt", b"plain")]);
    build.zip64_end = true;
    let input = build.bytes();
    let new_data: &[u8] = b"a replacement for the zip64 entry";

    let mut edits = Edits::new();
    edits.replace(0, bytes_replacement(new_data));
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    // The record's SHAPE is the input's: the fixed fields keep the marker even
    // though the new sizes would fit in 32 bits, so the harness's `attrs`
    // flag, which compares the raw bytes, stays clear. The real values are
    // read from the zip64 extra field by the independent reader above.
    let record = after.central_position as usize;
    assert_eq!(at_u32(&output, record + 20), U32_MAX_MARKER, "compressed");
    assert_eq!(at_u32(&output, record + 24), U32_MAX_MARKER, "uncompressed");
    assert_eq!(after.entries[0].crc32, crc32(new_data));
    assert_eq!(after.entries[0].uncompressed_size, new_data.len() as u64);
}

#[test]
fn padding_after_a_replaced_entry_is_kept() {
    // The span runs to the next local header, so the padding is part of it.
    // Dropping it would set the harness's `descriptor` flag on a replaced
    // entry, which P2's acceptance forbids.
    let mut first = Spec::new(b"one.txt", b"padded after").deflated();
    first.padding = 7;
    let build = Build::new(vec![first, Spec::new(b"two.txt", b"plain")]);
    let input = build.bytes();

    let mut edits = Edits::new();
    edits.replace(0, bytes_replacement(b"a longer payload than before"));
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    let end = data_start_of(&output, &after, 0) + after.entries[0].compressed_size as usize;
    assert_eq!(
        &output[end..end + 7],
        &[0xAAu8; 7],
        "the padding was dropped"
    );
    // And the next entry starts right after it.
    assert_eq!(
        (after.base + after.entries[1].local_offset) as usize,
        end + 7
    );
}

// ------------------------------------- edits in both offset conventions

/// An edit that MOVES an entry, on a fixture built in the named convention.
///
/// `specs/0008-zip-layer.md`, "P2": every edit test that moves an entry runs
/// on a fixture built in BOTH conventions from an explicit parameter, the
/// output's offsets checked to keep the input's convention, and the output
/// read back by an independent reader. P1's defect was exactly a convention
/// the corpus never exercises.
fn check_edit_behind_a_stub(zip64: bool, absolute: bool) {
    let mut first = Spec::new(b"one.txt", b"behind a stub").deflated();
    first.zip64 = zip64;
    let mut build = Build::new(vec![first, Spec::new(b"two.txt", b"plain")]);
    build.zip64_end = zip64;
    build.prefix = b"MZ stub bytes before the archive".to_vec();
    build.absolute_offsets = absolute;

    let input = build.bytes();
    let before = parse_independently(&input);
    assert_eq!(
        before.base,
        if absolute {
            0
        } else {
            build.prefix.len() as u64
        },
        "the fixture was not built in the convention asked for"
    );

    let mut edits = Edits::new();
    edits.replace(
        0,
        bytes_replacement(b"a replacement long enough to move the second entry along"),
    );
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    // The output keeps the INPUT's convention, because the prefix is verbatim
    // and every recorded offset is a position minus the same base.
    assert_eq!(after.base, before.base, "the offset convention changed");
    assert_eq!(
        &output[..build.prefix.len()],
        &build.prefix[..],
        "the prefix"
    );
    assert_eq!(names(&after), names(&before));
    // The edit really did move the second entry, so the offsets were rewritten
    // rather than happening to match.
    assert_ne!(
        after.entries[1].local_offset, before.entries[1].local_offset,
        "the second entry did not move, so this proves nothing"
    );
    assert_eq!(after.entries[1].crc32, before.entries[1].crc32);
}

#[test]
fn a_replace_behind_a_stub_with_relative_offsets() {
    check_edit_behind_a_stub(false, false);
}

#[test]
fn a_replace_behind_a_stub_with_absolute_offsets() {
    check_edit_behind_a_stub(false, true);
}

#[test]
fn a_zip64_replace_behind_a_stub_with_relative_offsets() {
    check_edit_behind_a_stub(true, false);
}

#[test]
fn a_zip64_replace_behind_a_stub_with_absolute_offsets() {
    check_edit_behind_a_stub(true, true);
}

/// A removal and an addition, which move every later entry, in one convention.
fn check_remove_and_add_behind_a_stub(absolute: bool) {
    let mut build = Build::new(vec![
        Spec::new(b"one.txt", b"first"),
        Spec::new(b"two.txt", b"second"),
        Spec::new(b"three.txt", b"third"),
    ]);
    build.prefix = b"MZ stub bytes before the archive".to_vec();
    build.absolute_offsets = absolute;

    let input = build.bytes();
    let before = parse_independently(&input);

    let mut edits = Edits::new();
    edits.remove(0);
    edits.add(b"added.txt".to_vec(), bytes_replacement(b"appended bytes"));
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    assert_eq!(after.base, before.base, "the offset convention changed");
    assert_eq!(
        &output[..build.prefix.len()],
        &build.prefix[..],
        "the prefix"
    );
    assert_eq!(
        names(&after),
        vec![
            b"two.txt".to_vec(),
            b"three.txt".to_vec(),
            b"added.txt".to_vec()
        ]
    );
    // Removing the first entry moved the survivors towards the prefix.
    assert!(after.entries[0].local_offset < before.entries[1].local_offset);
    assert_eq!(after.entries[0].crc32, crc32(b"second"));
    assert_eq!(after.entries[2].crc32, crc32(b"appended bytes"));
}

#[test]
fn a_remove_and_add_behind_a_stub_with_relative_offsets() {
    check_remove_and_add_behind_a_stub(false);
}

#[test]
fn a_remove_and_add_behind_a_stub_with_absolute_offsets() {
    check_remove_and_add_behind_a_stub(true);
}

// --------------------------------------------- the ignored 4 GiB case

/// The large entry: 4 GiB and one byte, STORED, so the archive is over 4 GiB
/// on disk and the second entry's offset and the central directory's need the
/// zip64 forms at real size (`specs/0008-zip-layer.md`, the P2 rulings of
/// 2026-09-17, item 3).
const LARGE_SIZE: u64 = 4 * 1024 * 1024 * 1024 + 1;
const LARGE_NAME: &str = "big.bin";
const SMALL_NAME: &str = "small.xml";
const SMALL_DATA: &[u8] = b"<small>the entry the test replaces</small>";
/// DIFFERENT bytes, so the harness's verdict on the small entry is `changed`
/// and not `recompressed`.
const SMALL_REPLACEMENT: &[u8] = b"<small>different bytes, so this entry is changed</small>";

/// One entry as the central directory records it.
struct FileEntry {
    name: Vec<u8>,
    crc32: u32,
    compressed_size: u64,
    uncompressed_size: u64,
    local_offset: u64,
}

/// What an archive's end records and central directory say.
struct Tail {
    zip64: bool,
    entries: Vec<FileEntry>,
}

/// Reads an archive's end records and central directory BY SEEKING.
///
/// `parse_independently` above takes the whole archive as a slice, which would
/// pull 4 GiB into memory and break this test's own RSS ceiling. This touches
/// only the tail and the central directory, both small.
fn read_tail(path: &std::path::Path) -> Tail {
    let mut file = File::open(path).expect("open the archive");
    let len = file.seek(SeekFrom::End(0)).expect("seek to the end");
    let window = len.min(66_560);
    let mut tail = vec![0u8; window as usize];
    file.seek(SeekFrom::Start(len - window)).expect("seek");
    file.read_exact(&mut tail).expect("read the tail");

    let u16at = |b: &[u8], at: usize| u16::from_le_bytes(b[at..at + 2].try_into().unwrap());
    let u32at = |b: &[u8], at: usize| u32::from_le_bytes(b[at..at + 4].try_into().unwrap());
    let u64at = |b: &[u8], at: usize| u64::from_le_bytes(b[at..at + 8].try_into().unwrap());

    let eocd = (0..=tail.len() - 22)
        .rev()
        .find(|&at| {
            tail[at..at + 4] == EOCD_SIG.to_le_bytes()
                && at + 22 + usize::from(u16at(&tail, at + 20)) == tail.len()
        })
        .expect("an end-of-central-directory record");
    let eocd_position = len - window + eocd as u64;

    // The zip64 record, when the input carried one, ends where the locator
    // begins; its position is what the central directory ends at.
    let zip64 = eocd >= usize_of(ZIP64_LOCATOR_LEN).unwrap()
        && tail[eocd - usize_of(ZIP64_LOCATOR_LEN).unwrap()..][..4]
            == ZIP64_LOCATOR_SIG.to_le_bytes();
    let (total, central_size, recorded_offset, central_end) = if zip64 {
        let at = (0..eocd)
            .rev()
            .find(|&at| tail[at..at + 4] == ZIP64_EOCD_SIG.to_le_bytes())
            .expect("a zip64 end-of-central-directory record");
        (
            u64at(&tail, at + 32),
            u64at(&tail, at + 40),
            u64at(&tail, at + 48),
            len - window + at as u64,
        )
    } else {
        (
            u64::from(u16at(&tail, eocd + 10)),
            u64::from(u32at(&tail, eocd + 12)),
            u64::from(u32at(&tail, eocd + 16)),
            eocd_position,
        )
    };

    let central_start = central_end - central_size;
    // These fixtures have no prefix, so a recorded offset IS a file position.
    assert_eq!(
        central_start - recorded_offset,
        0,
        "the fixture grew a prefix, which this reader does not expect"
    );
    let mut central = vec![0u8; central_size as usize];
    file.seek(SeekFrom::Start(central_start)).expect("seek");
    file.read_exact(&mut central).expect("read the directory");

    let mut entries = Vec::new();
    let mut at = 0usize;
    for _ in 0..total {
        let name_len = usize::from(u16at(&central, at + 28));
        let extra_len = usize::from(u16at(&central, at + 30));
        let comment_len = usize::from(u16at(&central, at + 32));
        let (raw_compressed, raw_uncompressed, raw_offset) = (
            u32at(&central, at + 20),
            u32at(&central, at + 24),
            u32at(&central, at + 42),
        );
        // The zip64 extra field carries whichever fields hold the marker, in
        // this order and only those.
        let mut values = Vec::new();
        let extra_start = at + CENTRAL_FIXED + name_len;
        let mut scan = extra_start;
        while scan + 4 <= extra_start + extra_len {
            let size = usize::from(u16at(&central, scan + 2));
            if u16at(&central, scan) == ZIP64_EXTRA_ID {
                let mut cursor = scan + 4;
                while cursor + 8 <= scan + 4 + size {
                    values.push(u64at(&central, cursor));
                    cursor += 8;
                }
                break;
            }
            scan += 4 + size;
        }
        let mut next = values.into_iter();
        let uncompressed_size = if raw_uncompressed == U32_MAX_MARKER {
            next.next().expect("a zip64 uncompressed size")
        } else {
            u64::from(raw_uncompressed)
        };
        let compressed_size = if raw_compressed == U32_MAX_MARKER {
            next.next().expect("a zip64 compressed size")
        } else {
            u64::from(raw_compressed)
        };
        let local_offset = if raw_offset == U32_MAX_MARKER {
            next.next().expect("a zip64 local header offset")
        } else {
            u64::from(raw_offset)
        };
        entries.push(FileEntry {
            name: central[at + CENTRAL_FIXED..at + CENTRAL_FIXED + name_len].to_vec(),
            crc32: u32at(&central, at + 16),
            compressed_size,
            uncompressed_size,
            local_offset,
        });
        at += CENTRAL_FIXED + name_len + extra_len + comment_len;
    }
    Tail { zip64, entries }
}

/// Writes the input archive: the large stored entry, then the small deflated
/// one. Neither entry is ever held in memory whole.
fn write_large_archive(path: &std::path::Path) {
    let file = File::create(path).expect("create the archive");
    let mut writer = zip::ZipWriter::new(file);
    let large = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .large_file(true)
        .last_modified_time(zip::DateTime::default());
    writer
        .start_file(LARGE_NAME, large)
        .expect("start the large entry");
    let chunk = vec![0u8; 1024 * 1024];
    let mut written = 0u64;
    while written < LARGE_SIZE {
        // The minimum is taken in u64 BEFORE the cast: on a 32-bit target the
        // remaining count does not fit in a usize until near the end.
        let take = (LARGE_SIZE - written).min(chunk.len() as u64) as usize;
        writer.write_all(&chunk[..take]).expect("write a chunk");
        written += take as u64;
    }
    // NO large_file here, deliberately, and it is not an oversight.
    //
    // `large_file(true)` makes the crate prepend a zip64 extended information
    // extra field to the entry's LOCAL header, and that field records the
    // entry's two sizes. Replacing the entry changes its compressed size, so
    // those bytes change, and the harness's `extra` flag compares the local
    // extra field raw (`specs/0004-xlsx-diff.md`): the replaced entry came
    // back flagged `extra`, which item 3 forbids. It is the same shape as the
    // `descriptor` finding of ruling 7, reached through a different field.
    //
    // The flag is also unnecessary here. Item 3 asks that "the second entry's
    // offset and the central directory's need the zip64 forms at real size",
    // which is about this entry's OFFSET, not its sizes: the offset lives in
    // the CENTRAL record, the crate writes it as a zip64 field because it is
    // over 4 GiB, and it is the same in input and output because the large
    // entry before it is copied verbatim. So the central extra field does not
    // change, and without a local zip64 extra field the sizes go in the fixed
    // fields, where a change is reported by the status and not by a flag.
    let small = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .last_modified_time(zip::DateTime::default());
    writer
        .start_file(SMALL_NAME, small)
        .expect("start the small entry");
    writer.write_all(SMALL_DATA).expect("write the small entry");
    writer.finish().expect("finish the archive");
}

/// The two archives, removed when the test ends however it ends.
struct Scratch(std::path::PathBuf, std::path::PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
        let _ = fs::remove_file(&self.1);
    }
}

/// The harness binary named by `EMEND_XLSX_DIFF`.
///
/// Read FIRST, before anything is built, and a panic when it is unset or names
/// nothing executable: the crate has no dev-dependencies and the harness links
/// no layer code, so the test cannot call the harness as a library, and it must
/// never skip the read-back silently (the P2 rulings of 2026-09-17, item 3).
/// No path is printed, so a failure cannot put one in a log.
fn harness_binary() -> std::path::PathBuf {
    let raw = env::var_os("EMEND_XLSX_DIFF").unwrap_or_else(|| {
        panic!("EMEND_XLSX_DIFF is unset; it must name the xlsx-diff binary of this target")
    });
    let path = std::path::PathBuf::from(raw);
    let meta = fs::metadata(&path).expect("EMEND_XLSX_DIFF names nothing that can be read");
    assert!(meta.is_file(), "EMEND_XLSX_DIFF does not name a file");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert!(
            meta.permissions().mode() & 0o111 != 0,
            "EMEND_XLSX_DIFF names nothing executable"
        );
    }
    path
}

#[test]
#[ignore = "writes and rewrites an archive over 4 GiB; run with --ignored zip64"]
fn zip64_replace_in_an_archive_over_4_gib() {
    // First, before anything is built.
    let harness = harness_binary();

    let directory = env::temp_dir();
    let input = directory.join(format!("emend-4gib-in-{}.zip", std::process::id()));
    let output = directory.join(format!("emend-4gib-out-{}.zip", std::process::id()));
    let _scratch = Scratch(input.clone(), output.clone());

    write_large_archive(&input);
    let input_len = fs::metadata(&input).expect("stat the input").len();
    assert!(
        input_len > u64::from(U32_MAX_MARKER),
        "the fixture is not over 4 GiB, so it proves nothing about zip64"
    );
    let before = read_tail(&input);
    assert!(before.zip64, "the fixture carries no zip64 end records");

    // Replace the SMALL entry; the large one is copied verbatim.
    let mut archive = Archive::open(File::open(&input).expect("open")).expect("open the archive");
    let index = archive
        .index_of(SMALL_NAME.as_bytes())
        .expect("the small entry");
    let mut edits = Edits::new();
    edits.replace(index, bytes_replacement(SMALL_REPLACEMENT));
    let handle = File::create(&output).expect("create the output");
    archive
        .write_to(std::io::BufWriter::new(handle), edits)
        .expect("write_to");

    // The output's own zip64 fields and offsets.
    let after = read_tail(&output);
    assert!(after.zip64, "the output carries no zip64 end records");
    assert_eq!(after.entries.len(), 2);
    assert_eq!(after.entries[0].name, LARGE_NAME.as_bytes());
    assert_eq!(after.entries[1].name, SMALL_NAME.as_bytes());

    // The large entry is untouched, and still 4 GiB + 1 stored.
    assert_eq!(after.entries[0].crc32, before.entries[0].crc32);
    assert_eq!(after.entries[0].uncompressed_size, LARGE_SIZE);
    assert_eq!(after.entries[0].compressed_size, LARGE_SIZE);

    // The second entry's offset is over 4 GiB, which is the whole point: it
    // cannot be recorded in the 32-bit field.
    assert!(
        after.entries[1].local_offset > u64::from(U32_MAX_MARKER),
        "the second entry's offset is not over 4 GiB"
    );
    assert_eq!(after.entries[1].crc32, crc32(SMALL_REPLACEMENT));
    assert_eq!(
        after.entries[1].uncompressed_size,
        SMALL_REPLACEMENT.len() as u64
    );

    // The harness's verdict, from the binary of this target.
    let run = std::process::Command::new(&harness)
        .arg(&input)
        .arg(&output)
        .output()
        .expect("run the harness");
    assert_eq!(
        String::from_utf8_lossy(&run.stderr),
        "",
        "the harness wrote to stderr"
    );
    assert_eq!(run.status.code(), Some(1), "the harness's exit status");

    let stdout = String::from_utf8(run.stdout).expect("the harness prints UTF-8");
    let mut seen = Vec::new();
    let mut flagged = None;
    for line in stdout.lines() {
        let cells: Vec<&str> = line.split('\t').collect();
        match cells.first() {
            Some(&"entry") => {
                // Column 17 is the flags, empty when there are none.
                seen.push((
                    cells[1].to_string(),
                    cells[2].to_string(),
                    cells[16].to_string(),
                ));
            }
            Some(&"archive") if cells[1] == "flagged" => flagged = Some(cells[2].to_string()),
            _ => {}
        }
    }
    assert_eq!(
        seen,
        vec![
            (
                LARGE_NAME.to_string(),
                "identical".to_string(),
                String::new()
            ),
            (SMALL_NAME.to_string(), "changed".to_string(), String::new()),
        ],
        "the harness's entry rows"
    );
    assert_eq!(flagged.as_deref(), Some("0"), "the harness's flagged count");
}

// ---------------------------------------- the zip64 end-record synthesis

/// The largest count the end-of-central-directory record's 16-bit fields
/// hold. It is also the marker value, which is why the fixture below is
/// interesting.
const MAX_16: usize = 0xFFFF;

/// An archive of `count` empty stored entries, carrying no zip64 end records.
fn many_entries(count: usize) -> Vec<u8> {
    let specs = (0..count)
        .map(|i| Spec::new(format!("e{i:05}").as_bytes(), b""))
        .collect();
    Build::new(specs).bytes()
}

/// Whether the four bytes just before an end record without a comment are a
/// zip64 locator's signature, read by position rather than by the layer.
/// The signature is written as a literal, not the layer's constant.
fn locator_precedes_end(bytes: &[u8]) -> bool {
    let end = bytes.len() - 22;
    bytes[end - 20..end - 16] == 0x0706_4b50u32.to_le_bytes()
}

#[test]
fn adding_past_65535_entries_synthesises_the_zip64_end_records() {
    // `specs/0008-zip-layer.md`, "P3", first item: P2 wrote this path and no
    // test reached it (the P2 verification rulings, item 1).
    let input = many_entries(MAX_16);
    assert!(
        !locator_precedes_end(&input),
        "the fixture already carries zip64 end records"
    );
    let input_end = input.len() - 22;
    assert_eq!(
        at_u16(&input, input_end + 10),
        0xFFFF,
        "the fixture's count sits at the 16-bit limit"
    );

    let mut edits = Edits::new();
    edits.add(b"added.xml".to_vec(), bytes_replacement(b"<added/>"));
    let output = apply(&input, edits);

    // From the end: the end record (22 bytes, no comment), the locator (20),
    // the zip64 end record (56, no extensible data sector).
    let end = output.len() - 22;
    let locator = end - 20;
    let record = locator - 56;

    // The central directory as the independent reader finds it.
    let parsed = parse_independently(&output);
    assert_eq!(parsed.entries.len(), MAX_16 + 1);
    assert_eq!(parsed.entries.last().expect("an entry").name, b"added.xml");
    let central_start = parsed.central_position;
    let central_size = record as u64 - central_start;
    let central_offset = central_start - parsed.base;
    let total = (MAX_16 + 1) as u64;

    // The zip64 end-of-central-directory record, field by field as the zip
    // specification lays it out (APPNOTE 4.3.14), with literal values rather
    // than the layer's constants or functions.
    let mut want_record = Vec::new();
    want_record.extend_from_slice(&0x0606_4b50u32.to_le_bytes()); // signature
    want_record.extend_from_slice(&44u64.to_le_bytes()); // size of the rest
    want_record.extend_from_slice(&45u16.to_le_bytes()); // version made by
    want_record.extend_from_slice(&45u16.to_le_bytes()); // version needed
    want_record.extend_from_slice(&0u32.to_le_bytes()); // this disk
    want_record.extend_from_slice(&0u32.to_le_bytes()); // the directory's disk
    want_record.extend_from_slice(&total.to_le_bytes()); // entries on this disk
    want_record.extend_from_slice(&total.to_le_bytes()); // entries in total
    want_record.extend_from_slice(&central_size.to_le_bytes());
    want_record.extend_from_slice(&central_offset.to_le_bytes());
    assert_eq!(want_record.len(), 56);
    assert_eq!(
        &output[record..locator],
        &want_record[..],
        "the zip64 end record"
    );

    // The zip64 end-of-central-directory locator (APPNOTE 4.3.15).
    let mut want_locator = Vec::new();
    want_locator.extend_from_slice(&0x0706_4b50u32.to_le_bytes()); // signature
    want_locator.extend_from_slice(&0u32.to_le_bytes()); // the record's disk
    want_locator.extend_from_slice(&(record as u64 - parsed.base).to_le_bytes());
    want_locator.extend_from_slice(&1u32.to_le_bytes()); // total disks
    assert_eq!(want_locator.len(), 20);
    assert_eq!(
        &output[locator..end],
        &want_locator[..],
        "the zip64 locator"
    );

    // The plain end record: both counts carry the marker because 65536 does
    // not fit in 16 bits; the size and the offset fit and stay plain.
    assert_eq!(
        at_u32(&output, end),
        0x0605_4b50,
        "the end record's signature"
    );
    assert_eq!(at_u16(&output, end + 8), 0xFFFF, "entries on this disk");
    assert_eq!(at_u16(&output, end + 10), 0xFFFF, "entries in total");
    assert_eq!(
        u64::from(at_u32(&output, end + 12)),
        central_size,
        "the central directory's size"
    );
    assert_eq!(
        u64::from(at_u32(&output, end + 16)),
        central_offset,
        "the central directory's offset"
    );
}

#[test]
fn removing_from_65535_entries_writes_a_plain_count() {
    // The companion the spec's fixture invites. With no zip64 end records, a
    // count of exactly 65535 sits in the 16-bit field as 0xFFFF, which is
    // also the marker value. A writer that keeps "a field already carrying
    // the marker" would keep it here too, and a removal would then leave
    // 0xFFFF over a count of 65534 with no zip64 record to resolve it.
    let input = many_entries(MAX_16);
    let mut edits = Edits::new();
    edits.remove(0);
    let output = apply(&input, edits);

    let end = output.len() - 22;
    assert!(
        !locator_precedes_end(&output),
        "a zip64 locator was written for a count that fits in 16 bits"
    );
    assert_eq!(
        usize::from(at_u16(&output, end + 8)),
        MAX_16 - 1,
        "entries on this disk"
    );
    assert_eq!(
        usize::from(at_u16(&output, end + 10)),
        MAX_16 - 1,
        "entries in total"
    );
    assert_eq!(parse_independently(&output).entries.len(), MAX_16 - 1);
}

#[test]
fn a_65535_entry_archive_round_trips_byte_for_byte() {
    // The control for the two tests above: with no edit the count is
    // unchanged, and the output must be the input exactly, 0xFFFF included.
    let input = many_entries(MAX_16);
    let output = apply(&input, Edits::new());
    assert_identical(&input, &output);
}

// ------------------------------------------------------- the error catalogue

/// The refusal an `Unsupported` carries, or a panic naming what came instead.
fn unsupported_message(err: Error) -> &'static str {
    match err {
        Error::Unsupported(message) => message,
        other => panic!("expected Unsupported, got {other:?}"),
    }
}

#[test]
fn the_catalogue_names_every_refusal() {
    // `specs/0008-zip-layer.md`, "P3": the error catalogue lives in the
    // crate's documentation. This reads the module's own source, collects
    // every message an `Unsupported` is built with, and fails if the module
    // header does not list it, so the catalogue cannot silently fall behind.
    let source = include_str!("../archive.rs");
    let doc = source
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("//!"))
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ");

    let opener = "Error::Unsupported(";
    let mut messages = Vec::new();
    let mut rest = source;
    while let Some(at) = rest.find(opener) {
        rest = &rest[at + opener.len()..];
        if let Some(body) = rest.trim_start().strip_prefix('"') {
            let end = body.find('"').expect("a closing quote");
            messages.push(&body[..end]);
        }
    }
    messages.sort_unstable();
    messages.dedup();
    // The scan's own control: a broken scan would find nothing and pass.
    assert!(
        messages.len() >= 15,
        "the scan found only {} distinct messages",
        messages.len()
    );
    let missing: Vec<&str> = messages
        .iter()
        .copied()
        .filter(|message| !doc.contains(message))
        .collect();
    assert!(
        missing.is_empty(),
        "refusals missing from the catalogue: {missing:?}"
    );
}

#[test]
fn a_replacement_for_an_entry_of_another_method_is_refused_when_planned() {
    let mut spec = Spec::new(b"bzip.txt", b"not really bzip2");
    spec.method = 12;
    let build = Build::new(vec![spec]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut edits = Edits::new();
    edits.replace(0, bytes_replacement(b"new"));
    let mut out = Vec::new();
    let err = archive.write_to(&mut out, edits).expect_err("must refuse");
    assert_eq!(
        unsupported_message(err),
        "replacing an entry whose method is neither stored nor deflate"
    );
    assert!(out.is_empty(), "refused after writing");
}

#[test]
fn compressing_with_another_method_is_refused_before_a_file_exists() {
    let _guard = REPLACEMENT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let before = replacement_temporaries();
    let err = match compress_replacement(bytes_replacement(b"new"), 12) {
        Ok(_) => panic!("method 12 was compressed"),
        Err(err) => err,
    };
    assert_eq!(
        unsupported_message(err),
        "writing an entry whose method is neither stored nor deflate"
    );
    assert_eq!(replacement_temporaries(), before, "a temporary was created");
}

#[test]
fn a_temporary_archive_of_two_entries_is_refused() {
    // `compress_replacement` cannot reach this: a Replacement gets a
    // `&mut dyn Write` and cannot start a second entry. So the check is
    // exercised on the helper it lives in.
    let build = Build::new(vec![Spec::new(b"a", b"1"), Spec::new(b"b", b"2")]);
    let archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let err = sole_entry_fields(&archive).expect_err("two entries must be refused");
    assert_eq!(
        unsupported_message(err),
        "the zip crate wrote something other than one entry"
    );
    // And the control: one entry is accepted.
    let one = Build::new(vec![Spec::new(b"a", b"1")]);
    let one = Archive::open(Cursor::new(one.bytes())).expect("open");
    assert!(sole_entry_fields(&one).is_ok());
}

#[test]
fn a_value_past_32_bits_in_a_plain_field_is_refused() {
    let build = Build::new(vec![Spec::new(b"a", b"1")]);
    let archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let record = archive.entries()[0].central.clone();
    let err = rewrite_central_record(&record, u64::from(u32::MAX) + 1, None)
        .expect_err("an offset past 32 bits in a plain field");
    assert_eq!(
        unsupported_message(err),
        "a value that would need a zip64 field the input does not carry"
    );
    // The control: the largest value the field holds plainly is written.
    assert!(rewrite_central_record(&record, u64::from(u32::MAX) - 1, None).is_ok());
}

#[test]
fn a_marker_with_no_zip64_extra_field_is_refused_on_rewrite() {
    // `open` refuses such a record first, so the rewrite is exercised on the
    // function itself, with a record whose offset field alone carries the
    // marker and which has no extra field at all.
    let build = Build::new(vec![Spec::new(b"a", b"1")]);
    let archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut record = archive.entries()[0].central.clone();
    record[42..46].copy_from_slice(&U32_MAX_MARKER.to_le_bytes());
    let err = rewrite_central_record(&record, 5, None).expect_err("no extra field");
    assert_eq!(
        unsupported_message(err),
        "a zip64 marker with no zip64 extra field"
    );
}

#[test]
fn a_replacement_too_large_for_a_plain_local_header_is_refused() {
    let _guard = REPLACEMENT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let build = Build::new(vec![Spec::new(b"a", b"1")]);
    let archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let entry = archive.entries()[0].clone();
    assert!(!entry.has_descriptor(), "precondition: sizes in the header");

    // A replacement whose compressed size needs 33 bits, standing in for the
    // real thing: the refusal depends on the sizes alone.
    let temp = TempFile::new().expect("a temporary name");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&temp.0)
        .expect("a temporary file");
    let made = Made {
        file,
        data_start: 0,
        crc32: 0,
        compressed_size: u64::from(u32::MAX) + 1,
        uncompressed_size: 1,
        _temp: temp,
    };
    let err = rewrite_local_header(&entry.local_header, &entry, &made)
        .expect_err("a 33-bit size with no local zip64 field");
    assert_eq!(
        unsupported_message(err),
        "a replacement too large for a local header with no zip64 extra field"
    );
}

#[test]
fn a_panicking_replacement_leaves_no_temporary_and_a_refusing_archive() {
    // `specs/0008-zip-layer.md`, "P3", and the P2 verification rulings, item
    // 8: the temporary is removed on unwinding too, and after the panic every
    // call on the archive returns an error rather than panicking again.
    let _guard = REPLACEMENT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload").deflated()]);
    let before = replacement_temporaries();
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");

    let mut edits = Edits::new();
    edits.replace(
        0,
        Box::new(|out: &mut dyn Write| -> io::Result<()> {
            out.write_all(b"half written")?;
            panic!("a replacement panicked, deliberately, in a test");
        }),
    );
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut out = Vec::new();
        let _ = archive.write_to(&mut out, edits);
    }));
    assert!(outcome.is_err(), "the panic did not propagate");
    assert_eq!(
        replacement_temporaries(),
        before,
        "the replacement's temporary survived the unwinding"
    );

    let read = archive.read(0).err().expect("read must refuse");
    assert_eq!(
        unsupported_message(read),
        "the archive's reader is checked out"
    );
    let write = archive
        .write_to(Vec::new(), Edits::new())
        .expect_err("write must refuse");
    assert_eq!(
        unsupported_message(write),
        "the archive's reader is checked out"
    );
}

#[test]
fn the_crate_path_reports_the_index_asked_for() {
    // It rendered usize::MAX until P3, a different number on each target.
    let err = zip_read_error(zip::result::ZipError::FileNotFound, 7);
    assert!(matches!(err, Error::NoSuchEntry(7)), "got {err:?}");
    assert_eq!(err.to_string(), "no entry with index 7");
}

#[test]
fn bytes_after_the_end_record_are_refused_with_their_own_message() {
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload")]);
    let mut bytes = build.bytes();
    bytes.extend_from_slice(b"trailing");
    assert_eq!(
        unsupported_message(open_err(bytes)),
        "bytes after the end-of-central-directory record"
    );
    // The control: a file with no end record at all is still not a zip.
    assert!(matches!(open_err(vec![7u8; 64]), Error::NotZip));
}

/// A zip64 archive whose end record carries an extensible data sector of
/// `sector` bytes, inserted after the record's fixed part.
fn zip64_with_sector(sector: usize) -> Vec<u8> {
    let mut first = Spec::new(b"one.txt", b"zip64 fixture");
    first.zip64 = true;
    let mut build = Build::new(vec![first]);
    build.zip64_end = true;
    let mut bytes = build.bytes();
    let record = bytes
        .windows(4)
        .rposition(|w| w == 0x0606_4b50u32.to_le_bytes())
        .expect("a zip64 end record");
    // Everything the record's size field counts grows by the sector; the
    // locator records where the record STARTS, which does not move.
    let size = u64::from_le_bytes(bytes[record + 4..record + 12].try_into().unwrap());
    bytes[record + 4..record + 12].copy_from_slice(&(size + sector as u64).to_le_bytes());
    let at = record + 56;
    bytes.splice(at..at, std::iter::repeat_n(0u8, sector));
    bytes
}

#[test]
fn a_zip64_sector_past_the_scan_window_is_refused_with_its_own_message() {
    let over = zip64_with_sector(0x1_0000);
    assert_eq!(
        unsupported_message(open_err(over)),
        "a zip64 end record not found within 65535 bytes of extensible data before its locator"
    );
    // The control, at the window's edge: 65535 bytes of sector is found, and
    // the sector is carried through a round trip.
    let edge = zip64_with_sector(0xFFFF);
    let output = round_trip(&edge);
    assert_identical(&edge, &output);
}

/// A stored entry with an UNSIGNED descriptor recording the signature value
/// as its CRC-32, then `padding` bytes, then a second entry.
fn crc_is_the_signature(padding: usize) -> Vec<u8> {
    let mut first = Spec::new(b"u.txt", b"payload").with_descriptor(false);
    first.recorded_crc = Some(0x0807_4b50);
    first.padding = padding;
    Build::new(vec![first, Spec::new(b"next.txt", b"after")]).bytes()
}

#[test]
fn an_unsigned_descriptor_with_the_signature_as_crc_is_refused_when_nothing_follows() {
    // The documented limit, first form: read as signed, the descriptor looks
    // four bytes longer than the span holds.
    let _guard = REPLACEMENT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut archive = Archive::open(Cursor::new(crc_is_the_signature(0))).expect("open");
    let mut edits = Edits::new();
    edits.replace(0, bytes_replacement(b"new payload"));
    let err = archive
        .write_to(Vec::new(), edits)
        .expect_err("the misread descriptor overruns the span");
    assert!(matches!(err, Error::Truncated), "got {err:?}");
}

#[test]
fn an_unsigned_descriptor_with_the_signature_as_crc_gains_a_signature_over_padding() {
    // The documented limit, second form, and the silent one: with padding
    // after it, the descriptor is rewritten WITH a signature, and the span
    // keeps its length by carrying four bytes less padding.
    let input = crc_is_the_signature(7);
    let mut edits = Edits::new();
    edits.replace(0, bytes_replacement(b"new payload"));
    let output = apply(&input, edits);
    let after = parse_independently(&output);

    let data_end = data_start_of(&output, &after, 0) + after.entries[0].compressed_size as usize;
    let next = (after.base + after.entries[1].local_offset) as usize;
    assert_eq!(
        at_u32(&output, data_end),
        DESCRIPTOR_SIG,
        "the descriptor was written with a signature"
    );
    // Unsigned descriptor 12 + padding 7 = 19 in the input; signed 16 + 3 now.
    assert_eq!(next - data_end, 19, "the span changed length");
    assert_eq!(
        &output[data_end + 16..next],
        &[0xAAu8; 3],
        "four bytes less padding"
    );
}

// ------------------------------------------ the repair round's catalogue tests

#[cfg(unix)]
#[test]
fn the_replacement_temporary_is_created_with_mode_0600() {
    // `specs/0008-zip-layer.md`, the P3 first-verification rulings, item 1:
    // the documented limit "mode 0600 on Unix" is pinned here. The
    // replacement runs while its temporary exists, so it looks the file up
    // by this process's name form and reads its mode.
    use std::os::unix::fs::PermissionsExt;
    use std::sync::{Arc, Mutex};

    let _guard = REPLACEMENT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let before = replacement_temporaries();
    let seen: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
    let seen_inside = Arc::clone(&seen);
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload").deflated()]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut edits = Edits::new();
    edits.replace(
        0,
        Box::new(move |out: &mut dyn Write| {
            let mut modes = seen_inside.lock().unwrap();
            for name in replacement_temporaries() {
                if before.contains(&name) {
                    continue;
                }
                let meta = std::fs::metadata(env::temp_dir().join(&name))?;
                modes.push(meta.permissions().mode() & 0o777);
            }
            out.write_all(b"new payload")
        }),
    );
    let mut out = Vec::new();
    archive.write_to(&mut out, edits).expect("write_to");
    assert_eq!(
        *seen.lock().unwrap(),
        vec![0o600],
        "exactly one temporary, with mode 0600, while the replacement ran"
    );
}

#[test]
fn an_added_name_longer_than_a_header_can_record_is_refused() {
    let _guard = REPLACEMENT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload")]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut edits = Edits::new();
    edits.add(vec![b'n'; 0x1_0000], bytes_replacement(b"new"));
    let mut out = Vec::new();
    let err = archive.write_to(&mut out, edits).expect_err("must refuse");
    assert_eq!(
        unsupported_message(err),
        "an entry name longer than a header can record"
    );

    // The control: a name of 65535 bytes, the most a header can record, is
    // written and read back.
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut edits = Edits::new();
    edits.add(vec![b'n'; 0xFFFF], bytes_replacement(b"new"));
    let mut out = Vec::new();
    archive.write_to(&mut out, edits).expect("write_to");
    let parsed = parse_independently(&out);
    assert_eq!(parsed.entries.len(), 2);
    assert_eq!(parsed.entries[1].name.len(), 0xFFFF);
}

#[test]
fn a_non_zip_with_a_stray_end_record_signature_is_refused_as_trailing_bytes() {
    // The first verification rulings, item 3: the refusal fires on what the
    // code checks, a signature whose record would end before the file does,
    // with no record ending at the end, whether or not the file is a zip.
    let mut bytes = vec![7u8; 512];
    bytes[100..104].copy_from_slice(&0x0605_4b50u32.to_le_bytes());
    bytes[120..122].copy_from_slice(&0u16.to_le_bytes());
    assert_eq!(
        unsupported_message(open_err(bytes.clone())),
        "bytes after the end-of-central-directory record"
    );
    // The control: the same file without the signature is not a zip.
    bytes[100..104].copy_from_slice(&[7u8; 4]);
    assert!(matches!(open_err(bytes), Error::NotZip));
}

/// A zip64 archive of two entries, for the locator refusals.
fn zip64_fixture() -> Vec<u8> {
    let mut spec = Spec::new(b"big.txt", b"not actually big");
    spec.zip64 = true;
    let mut build = Build::new(vec![spec, Spec::new(b"small.txt", b"plain")]);
    build.zip64_end = true;
    build.bytes()
}

#[test]
fn a_zip64_locator_without_its_record_is_refused() {
    let mut bytes = zip64_fixture();
    // The control: the fixture opens as it is.
    Archive::open(Cursor::new(bytes.clone())).expect("the fixture opens");
    let record = bytes
        .windows(4)
        .rposition(|w| w == 0x0606_4b50u32.to_le_bytes())
        .expect("a zip64 end record");
    bytes[record] ^= 0xFF;
    assert_eq!(
        unsupported_message(open_err(bytes)),
        "a zip64 locator without its record"
    );
}

#[test]
fn a_locator_and_a_directory_disagreeing_on_the_base_are_refused() {
    let mut bytes = zip64_fixture();
    let locator = bytes
        .windows(4)
        .rposition(|w| w == 0x0706_4b50u32.to_le_bytes())
        .expect("a zip64 locator");
    // The locator's record offset, one byte short: its base becomes 1 while
    // the central directory's stays 0.
    let at = locator + 8;
    let offset = u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap());
    assert!(offset > 0);
    bytes[at..at + 8].copy_from_slice(&(offset - 1).to_le_bytes());
    assert_eq!(
        unsupported_message(open_err(bytes)),
        "the zip64 locator and the central directory disagree on the offset base"
    );
}

#[test]
fn reading_an_entry_of_a_method_the_crate_cannot_decode_is_refused() {
    // The layer accepts any method at open; the `zip` crate refuses method 12
    // when the entry is read, and the refusal reaches the caller as the
    // catalogued message (P2's negative.log E9 reached it through
    // emend-copy).
    let mut spec = Spec::new(b"bzip.txt", b"not really bzip2");
    spec.method = 12;
    let build = Build::new(vec![spec, Spec::new(b"plain.txt", b"plain")]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let err = archive.read(0).err().expect("must refuse");
    assert_eq!(
        unsupported_message(err),
        "an entry the zip crate could not decompress"
    );
    // The control: a stored entry in the same archive reads.
    let mut plain = Vec::new();
    archive
        .read(1)
        .expect("read")
        .read_to_end(&mut plain)
        .expect("read_to_end");
    assert_eq!(plain, b"plain");
}

#[test]
fn the_crate_refusals_carry_their_catalogued_messages() {
    // No fixture makes the `zip` crate refuse at open, or while compressing,
    // where this layer has already accepted the input, so those two messages
    // are pinned here, at the mapping functions (the catalogue says so). The
    // read mapping is asserted here too; an archive reaches it above.
    let open = zip_open_error(zip::result::ZipError::UnsupportedArchive("x"));
    assert_eq!(
        unsupported_message(open),
        "a zip feature the zip crate refused"
    );
    let read = zip_read_error(zip::result::ZipError::InvalidArchive("x".into()), 0);
    assert_eq!(
        unsupported_message(read),
        "an entry the zip crate could not decompress"
    );
    let write = zip_write_error(zip::result::ZipError::InvalidArchive("x".into()));
    assert_eq!(
        unsupported_message(write),
        "a replacement the zip crate could not compress"
    );
    // The controls: the crate's I/O errors stay I/O errors.
    let io = zip_write_error(zip::result::ZipError::Io(io::Error::other("x")));
    assert!(matches!(io, Error::Io(_)), "got {io:?}");
}
