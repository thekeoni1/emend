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
        }
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
            let crc = crc32(&spec.data);
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
                out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
                out.extend_from_slice(&(spec.data.len() as u32).to_le_bytes());
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
            let crc = crc32(&spec.data);

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
    assert!(matches!(err, Error::Unsupported(_)), "got {err:?}");
}

#[test]
fn reading_an_encrypted_entry_is_refused() {
    let mut spec = Spec::new(b"secret.txt", b"not really encrypted");
    spec.gp_extra = GP_ENCRYPTED;
    let build = Build::new(vec![spec]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    assert!(archive.entries()[0].is_encrypted());
    let err = archive.read(0).err().expect("must refuse");
    assert!(matches!(err, Error::Unsupported(_)), "got {err:?}");
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

#[test]
fn a_non_empty_edits_is_refused_in_p1() {
    let build = Build::new(vec![Spec::new(b"one.txt", b"payload")]);
    let mut archive = Archive::open(Cursor::new(build.bytes())).expect("open");
    let mut edits = Edits::new();
    edits.remove(0);
    assert!(!edits.is_empty());
    let mut out = Vec::new();
    let err = archive.write_to(&mut out, edits).expect_err("must refuse");
    assert!(matches!(err, Error::Unsupported(_)), "got {err:?}");
}
