//! Byte-level archives and an independent reader, for every layer's tests.
//!
//! `specs/0009-package-layer.md`, "Crate": the builder and the independent
//! parser of `archive/tests.rs` live here so that the package tests build
//! their fixtures and read their outputs through code that shares nothing
//! with either layer. A bug in a builder therefore cannot be the same bug as
//! one in the code under test, and the crate still has no dev-dependencies
//! (`specs/0008-zip-layer.md`, "Crate").
//!
//! Nothing here calls the layer. That is why the zip format's own numbers are
//! declared below from the specification rather than imported from
//! `archive.rs`: before the move the independent parser read the layer's
//! `CENTRAL_FIXED`, so a wrong record size would have been the same mistake
//! on both sides of the comparison, which is the one thing that parser exists
//! to rule out (P2 ruling 1). The two method constants were already declared
//! this way; these ten follow them.
//!
//! Deflate is produced as a run of stored blocks, which is valid deflate and
//! needs no encoder; CRC-32 is computed bitwise below.

// --------------------------------------------- the zip format's own numbers
//
// PKWARE APPNOTE 6.3.x, section 4.3 (the record layouts), 4.4 (the field
// values) and 4.5 (the extra fields). Read from the specification, never from
// the layer under test.

/// Local file header signature (APPNOTE 4.3.7).
pub(crate) const LOCAL_SIG: u32 = 0x0403_4b50;
/// Central directory record signature (APPNOTE 4.3.12).
pub(crate) const CENTRAL_SIG: u32 = 0x0201_4b50;
/// End of central directory record signature (APPNOTE 4.3.16).
pub(crate) const EOCD_SIG: u32 = 0x0605_4b50;
/// Zip64 end of central directory record signature (APPNOTE 4.3.14).
pub(crate) const ZIP64_EOCD_SIG: u32 = 0x0606_4b50;
/// Zip64 end of central directory locator signature (APPNOTE 4.3.15).
pub(crate) const ZIP64_LOCATOR_SIG: u32 = 0x0706_4b50;
/// Fixed part of a central directory record, before the name, the extra
/// field and the comment (APPNOTE 4.3.12).
pub(crate) const CENTRAL_FIXED: usize = 46;
/// Fixed part of a zip64 end of central directory record (APPNOTE 4.3.14).
pub(crate) const ZIP64_EOCD_FIXED: u64 = 56;
/// Header id of the zip64 extended information extra field (APPNOTE 4.5.3).
pub(crate) const ZIP64_EXTRA_ID: u16 = 0x0001;
/// The recorded value that says the real one is in the zip64 extra field
/// (APPNOTE 4.4.1.4 and 4.5.3).
pub(crate) const U32_MAX_MARKER: u32 = 0xFFFF_FFFF;
/// General purpose bit 3: the sizes follow the data in a descriptor
/// (APPNOTE 4.4.4).
pub(crate) const GP_DESCRIPTOR: u16 = 1 << 3;

// ---------------------------------------------------------------- helpers

/// CRC-32, bitwise, so the tests carry their own and do not borrow the
/// layer's or a crate's.
pub(crate) fn crc32(data: &[u8]) -> u32 {
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
pub(crate) fn deflate_stored(data: &[u8]) -> Vec<u8> {
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

pub(crate) const METHOD_STORE: u16 = 0;
pub(crate) const METHOD_DEFLATE: u16 = 8;

/// How one entry is laid out in a fixture.
#[derive(Clone)]
pub(crate) struct Spec {
    pub(crate) name: Vec<u8>,
    pub(crate) data: Vec<u8>,
    pub(crate) method: u16,
    /// `Some(true)` a descriptor with its signature, `Some(false)` without.
    pub(crate) descriptor: Option<bool>,
    pub(crate) local_extra: Vec<u8>,
    pub(crate) central_extra: Vec<u8>,
    pub(crate) comment: Vec<u8>,
    /// Bytes of padding written after this entry's span.
    pub(crate) padding: usize,
    /// Record the sizes and the offset as zip64 markers plus a zip64 extra
    /// field, without building a large archive.
    pub(crate) zip64: bool,
    /// Extra general purpose bits, for the encrypted case.
    pub(crate) gp_extra: u16,
    /// A CRC-32 to record in place of the data's own. The layer never checks
    /// a CRC on the paths these tests reach, so this is how a fixture puts a
    /// chosen value where a reader will look at it.
    pub(crate) recorded_crc: Option<u32>,
}

impl Spec {
    pub(crate) fn new(name: &[u8], data: &[u8]) -> Self {
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
    pub(crate) fn crc(&self) -> u32 {
        self.recorded_crc.unwrap_or_else(|| crc32(&self.data))
    }

    pub(crate) fn deflated(mut self) -> Self {
        self.method = METHOD_DEFLATE;
        self
    }

    pub(crate) fn with_descriptor(mut self, signature: bool) -> Self {
        self.descriptor = Some(signature);
        self
    }

    pub(crate) fn payload(&self) -> Vec<u8> {
        match self.method {
            METHOD_DEFLATE => deflate_stored(&self.data),
            _ => self.data.clone(),
        }
    }

    pub(crate) fn gp_flags(&self) -> u16 {
        let mut flags = self.gp_extra;
        if self.descriptor.is_some() {
            flags |= GP_DESCRIPTOR;
        }
        flags
    }
}

/// Builds a complete archive: a prefix, every entry's span in the given file
/// order, the central directory in `central_order`, and the end records.
pub(crate) struct Build {
    pub(crate) prefix: Vec<u8>,
    pub(crate) specs: Vec<Spec>,
    /// Indices into `specs`, in central directory order. Defaults to file
    /// order; T1 P6 showed the two can differ.
    pub(crate) central_order: Option<Vec<usize>>,
    pub(crate) comment: Vec<u8>,
    pub(crate) zip64_end: bool,
    /// Written into the end-of-central-directory record's disk fields.
    pub(crate) disk: u16,
    /// Whether the recorded offsets COUNT THE PREFIX (file-absolute) or not
    /// (relative to the archive proper). Both conventions are in use, and a
    /// fixture must state which it builds rather than inherit whatever the
    /// implementation happens to do: an earlier version of this builder wrote
    /// the entry offsets relative and the zip64 locator absolute, so it
    /// encoded the layer's own mixture and no test could fail on the
    /// difference. That is how the silent prefix loss survived
    /// (`specs/0008-zip-layer.md`, the P1 rulings of 2026-09-17, item 3).
    pub(crate) absolute_offsets: bool,
}

impl Build {
    pub(crate) fn new(specs: Vec<Spec>) -> Self {
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

    pub(crate) fn bytes(&self) -> Vec<u8> {
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

// ------------------------------------------------- an independent reader

/// One entry as the independent reader sees it.
#[derive(Debug, Clone)]
pub(crate) struct OutEntry {
    pub(crate) name: Vec<u8>,
    pub(crate) method: u16,
    pub(crate) crc32: u32,
    pub(crate) compressed_size: u64,
    pub(crate) uncompressed_size: u64,
    pub(crate) local_offset: u64,
    pub(crate) gp_flags: u16,
}

/// What the independent reader makes of an archive.
#[derive(Debug)]
pub(crate) struct Parsed {
    /// What a recorded offset is added to in order to get a file position.
    pub(crate) base: u64,
    /// Where the central directory actually starts.
    pub(crate) central_position: u64,
    pub(crate) entries: Vec<OutEntry>,
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
pub(crate) fn parse_independently(bytes: &[u8]) -> Parsed {
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

/// Serializes every test that makes a temporary of the layers' own form.
///
/// `cargo test` runs the tests in threads of ONE process, so they share both
/// the temporary directory and the process id in the temporary's name.
/// Without this, a snapshot like `a_replacement_error_leaves_no_temporary_
/// behind` catches another test's file in flight and fails on a race rather
/// than on the layer. Both layers make temporaries of that form: the zip
/// layer compresses a replacement into one, and from M1 part 2 P3 the package
/// layer writes each rewritten part into one, so the two layers' tests hold
/// the SAME lock and it lives here rather than in either of them.
pub(crate) static TEMPORARY_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// The layers' temporaries belonging to this process, by name.
pub(crate) fn replacement_temporaries() -> Vec<std::ffi::OsString> {
    let prefix = format!("emend-replace-{}-", std::process::id());
    let mut found: Vec<std::ffi::OsString> = std::fs::read_dir(std::env::temp_dir())
        .expect("read the temporary directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name())
        .filter(|name| name.to_string_lossy().starts_with(&prefix))
        .collect();
    found.sort();
    found
}

/// The entry names an archive holds, in central directory order.
pub(crate) fn names(parsed: &Parsed) -> Vec<Vec<u8>> {
    parsed.entries.iter().map(|e| e.name.clone()).collect()
}

/// The bytes of the stored entry named `name`, found with this reader's own
/// offsets.
///
/// `specs/0009-package-layer.md`, "P3": an edit test compares the edited
/// part's bytes with bytes the test builds by hand, so it needs the output's
/// bytes back without asking the layer that wrote them. Stored only: every
/// package fixture is stored and [`crate::archive::Archive::write_to`] keeps
/// an entry's method, so a rewritten part comes back stored, and a caller
/// that reaches a compressed entry is told rather than given plausible
/// rubbish.
#[track_caller]
pub(crate) fn stored_data(bytes: &[u8], name: &[u8]) -> Vec<u8> {
    /// The local header's fixed part: signature, version, flags, method,
    /// time, date, CRC-32, both sizes and both lengths, as `Build` writes it
    /// above.
    const FIXED: usize = 30;
    /// Where the name and extra lengths sit inside that fixed part.
    const NAME_LENGTH_AT: usize = 26;
    const EXTRA_LENGTH_AT: usize = 28;

    let parsed = parse_independently(bytes);
    let entry = parsed
        .entries
        .iter()
        .find(|entry| entry.name == name)
        .unwrap_or_else(|| panic!("no entry named {:?}", String::from_utf8_lossy(name)));
    assert_eq!(
        entry.method,
        METHOD_STORE,
        "stored_data on a compressed entry: {:?}",
        String::from_utf8_lossy(name)
    );
    let at = (parsed.base + entry.local_offset) as usize;
    assert_eq!(
        bytes[at..at + 4],
        LOCAL_SIG.to_le_bytes(),
        "no local header at the recorded offset for {:?}",
        String::from_utf8_lossy(name)
    );
    let name_length =
        u16::from_le_bytes([bytes[at + NAME_LENGTH_AT], bytes[at + NAME_LENGTH_AT + 1]]) as usize;
    let extra_length =
        u16::from_le_bytes([bytes[at + EXTRA_LENGTH_AT], bytes[at + EXTRA_LENGTH_AT + 1]]) as usize;
    let start = at + FIXED + name_length + extra_length;
    bytes[start..start + entry.uncompressed_size as usize].to_vec()
}

// ------------------------------------------------------------------ the keep
//
// `specs/0015-open-checks.md`, "The keep" and the P1 rulings 1 to 3: with a
// keep directory named, every successful edit test's input and output are
// written there, so a script can open each pair in LibreOffice afterwards.
// With none named nothing is written and no test behaves differently.

/// The environment variable that names the keep directory.
pub(crate) const KEEP_VARIABLE: &str = "EMEND_KEEP_DIR";

thread_local! {
    /// The keep tests' override of the variable, for this thread alone:
    /// `Some(Some(dir))` keeps into `dir`, `Some(None)` keeps nothing, and
    /// `None` reads the variable. The variable is process-wide, and setting
    /// it is unsafe in this edition, which the crate forbids (P1 ruling 3).
    static KEEP_OVERRIDE: std::cell::RefCell<Option<Option<std::path::PathBuf>>> =
        const { std::cell::RefCell::new(None) };
    /// How many pairs this thread has kept: a test that writes more than
    /// once keeps each write under the next number (P1 ruling 1).
    static KEEP_COUNT: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    /// The bytes of the fixture this thread last opened, for the keep.
    static FIXTURE: std::cell::RefCell<Option<Vec<u8>>> = const { std::cell::RefCell::new(None) };
}

/// Sets this thread's override: `Some(dir)` keeps into `dir`, `None` keeps
/// nothing whatever the variable says.
pub(crate) fn override_keep(dir: Option<std::path::PathBuf>) {
    KEEP_OVERRIDE.with(|slot| *slot.borrow_mut() = Some(dir));
}

/// The directory this thread keeps into, if any. A named directory that is
/// not there is a mistake in the run, so it fails the test rather than
/// keeping nothing quietly.
#[track_caller]
pub(crate) fn keep_dir() -> Option<std::path::PathBuf> {
    let dir = KEEP_OVERRIDE
        .with(|slot| slot.borrow().clone())
        .unwrap_or_else(|| std::env::var_os(KEEP_VARIABLE).map(std::path::PathBuf::from))?;
    assert!(
        dir.is_dir(),
        "the keep directory {} is not a directory",
        dir.display()
    );
    Some(dir)
}

/// Records `bytes` as the fixture this thread is about to open.
pub(crate) fn record_fixture(bytes: &[u8]) {
    FIXTURE.with(|slot| *slot.borrow_mut() = Some(bytes.to_vec()));
}

/// The recorded fixture, taken: a second write needs a second open.
pub(crate) fn take_fixture() -> Option<Vec<u8>> {
    FIXTURE.with(|slot| slot.borrow_mut().take())
}

/// Keeps `input` and `output` as `<test>.<n>.in.xlsx` and
/// `<test>.<n>.out.xlsx` in the keep directory, `<test>` being this thread's
/// name with `::` made `.` and `n` counting this thread's pairs from 1.
/// Nothing when no directory is named. A name already there is refused, so a
/// directory holds one run, and it is refused before either file is written.
#[track_caller]
pub(crate) fn keep(input: &[u8], output: &[u8]) {
    let Some(dir) = keep_dir() else {
        return;
    };
    let n = KEEP_COUNT.with(|count| {
        count.set(count.get() + 1);
        count.get()
    });
    let thread = std::thread::current();
    let test = thread
        .name()
        .expect("the keep names a pair after the test's thread")
        .replace("::", ".");
    let pair = [
        (dir.join(format!("{test}.{n}.in.xlsx")), input),
        (dir.join(format!("{test}.{n}.out.xlsx")), output),
    ];
    for (path, _) in &pair {
        assert!(
            !path.exists(),
            "the keep refuses {}: the name is already there",
            path.display()
        );
    }
    for (path, bytes) in &pair {
        std::fs::write(path, bytes).expect("write a kept file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    /// A fresh directory named after the test, removed first if a failed
    /// run left one.
    fn fresh(label: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("emend-keep-test-{}-{label}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create the directory");
        dir
    }

    fn listed(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(dir)
            .expect("list the directory")
            .map(|entry| {
                entry
                    .expect("an entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        names.sort();
        names
    }

    #[test]
    fn the_keep_names_each_pair_after_the_test_and_numbers_them() {
        let dir = fresh("names");
        override_keep(Some(dir.clone()));
        keep(b"first in", b"first out");
        keep(b"second in", b"second out");
        let test = "testing.tests.the_keep_names_each_pair_after_the_test_and_numbers_them";
        assert_eq!(
            listed(&dir),
            [
                format!("{test}.1.in.xlsx"),
                format!("{test}.1.out.xlsx"),
                format!("{test}.2.in.xlsx"),
                format!("{test}.2.out.xlsx"),
            ]
        );
        let read = |name: String| std::fs::read(dir.join(name)).expect("read a kept file");
        assert_eq!(read(format!("{test}.1.in.xlsx")), b"first in");
        assert_eq!(read(format!("{test}.1.out.xlsx")), b"first out");
        assert_eq!(read(format!("{test}.2.in.xlsx")), b"second in");
        assert_eq!(read(format!("{test}.2.out.xlsx")), b"second out");
        std::fs::remove_dir_all(&dir).expect("remove the directory");
    }

    #[test]
    fn the_keep_refuses_a_name_already_there_and_writes_neither_file() {
        let dir = fresh("refuses");
        let test = "testing.tests.the_keep_refuses_a_name_already_there_and_writes_neither_file";
        // A previous run's output under the name the first pair takes.
        std::fs::write(dir.join(format!("{test}.1.out.xlsx")), b"earlier").expect("plant");
        override_keep(Some(dir.clone()));
        let refused = std::panic::catch_unwind(|| keep(b"in", b"out"));
        let message = *refused
            .expect_err("the keep refused")
            .downcast::<String>()
            .expect("a formatted message");
        assert!(
            message.contains("the name is already there"),
            "the message: {message}"
        );
        assert_eq!(listed(&dir), [format!("{test}.1.out.xlsx")]);
        assert_eq!(
            std::fs::read(dir.join(format!("{test}.1.out.xlsx"))).expect("read"),
            b"earlier"
        );
        std::fs::remove_dir_all(&dir).expect("remove the directory");
    }

    #[test]
    fn with_no_keep_directory_nothing_is_written() {
        // The override stands for an unset variable, whatever the run's
        // environment holds, so a directory the test watches stays empty.
        let dir = fresh("none");
        override_keep(None);
        assert_eq!(keep_dir(), None);
        keep(b"in", b"out");
        assert_eq!(listed(&dir), Vec::<String>::new());
        std::fs::remove_dir_all(&dir).expect("remove the directory");
    }

    #[test]
    fn a_keep_directory_that_is_not_there_fails_the_test() {
        let dir = fresh("missing").join("not-there");
        override_keep(Some(dir));
        let refused = std::panic::catch_unwind(|| keep(b"in", b"out"));
        let message = *refused
            .expect_err("the keep refused")
            .downcast::<String>()
            .expect("a formatted message");
        assert!(
            message.contains("is not a directory"),
            "the message: {message}"
        );
        std::fs::remove_dir_all(fresh("missing")).expect("remove the directory");
    }
}
