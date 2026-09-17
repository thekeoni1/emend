//! The zip layer: open an archive, index it, write it back.
//!
//! `specs/0008-zip-layer.md`, "Module `archive`". The layer parses the zip
//! structures it needs itself, because `zip` 8.6.0 exposes neither the raw
//! central record nor the raw local header, the general purpose flags or the
//! data descriptor (checked against the crate's source, 2026-09-16). It uses
//! the crate for one thing only: decompressing an entry's bytes in [`read`].
//!
//! [`read`]: Archive::read
//!
//! # Why the reader moves between two owners
//!
//! `zip::ZipArchive` takes the reader by value and gives it back only through
//! `into_inner`; there is no `get_mut`. A decompressing read therefore needs
//! the archive to own the reader, while a verbatim span copy needs raw access
//! to the same bytes. [`Archive`] holds the `ZipArchive` in an `Option`:
//! [`Archive::read`] borrows it, and [`Archive::write_to`] takes it out,
//! recovers the reader, copies the spans, and puts a fresh one back.
//!
//! # Why spans, and not entry-by-entry reconstruction
//!
//! An untouched entry is copied from its local header to the next local
//! header in file order, data descriptor, padding and any orphaned bytes
//! included. Rebuilding an entry from its parts instead would silently drop
//! exactly those bytes. T3 P4 measured what that costs: with the `zip`
//! crate's own raw copy all 870 corpus entries stayed `identical` and every
//! one of them was flagged, and no file was byte-identical.
//!
//! Every offset, size and count here is `u64`. `usize` is 32 bits on
//! `i686-unknown-linux-musl`.

use std::fmt;
use std::io::{self, Read, Seek, SeekFrom, Write};

#[cfg(test)]
mod tests;

/// Local file header signature.
const LOCAL_SIG: u32 = 0x0403_4b50;
/// Central directory record signature.
const CENTRAL_SIG: u32 = 0x0201_4b50;
/// End of central directory record signature.
const EOCD_SIG: u32 = 0x0605_4b50;
/// Zip64 end of central directory record signature.
const ZIP64_EOCD_SIG: u32 = 0x0606_4b50;
/// Zip64 end of central directory locator signature.
const ZIP64_LOCATOR_SIG: u32 = 0x0706_4b50;

/// Fixed part of a local file header, before the name and extra field.
const LOCAL_FIXED: u64 = 30;
/// Fixed part of a central directory record, before name, extra and comment.
const CENTRAL_FIXED: usize = 46;
/// Fixed part of an end-of-central-directory record, before the comment.
const EOCD_FIXED: u64 = 22;
/// A zip64 end-of-central-directory locator is exactly this long.
const ZIP64_LOCATOR_LEN: u64 = 20;
/// Fixed part of a zip64 end-of-central-directory record, before the
/// extensible data sector.
const ZIP64_EOCD_FIXED: u64 = 56;

/// The largest archive comment a zip file can carry, so the furthest back an
/// end-of-central-directory record can start from the end of the file.
const MAX_COMMENT: u64 = 0xFFFF;

/// The value a 32-bit field carries when the real value lives in a zip64
/// extra field or record.
const U32_MAX_MARKER: u32 = 0xFFFF_FFFF;
/// The same, for the 16-bit disk fields.
const U16_MAX_MARKER: u16 = 0xFFFF;

/// Header id of the zip64 extended information extra field.
const ZIP64_EXTRA_ID: u16 = 0x0001;

/// General purpose bit 0: the entry is encrypted.
const GP_ENCRYPTED: u16 = 1 << 0;
/// General purpose bit 3: sizes and CRC-32 follow the data in a descriptor.
const GP_DESCRIPTOR: u16 = 1 << 3;

/// Copies go through a buffer of this size, so a 63.7 MB entry never lands in
/// memory whole.
const COPY_BUFFER: usize = 64 * 1024;

/// Everything that stops the layer.
///
/// A message never carries an entry's content. It may carry an entry index,
/// which is not content.
#[derive(Debug)]
pub enum Error {
    /// The underlying reader or writer failed.
    Io(io::Error),
    /// No end-of-central-directory record was found.
    NotZip,
    /// A recorded offset or size lies beyond the file.
    Truncated,
    /// A zip feature this layer does not represent.
    Unsupported(&'static str),
    /// Two central records carry the same raw name, so an edit by name would
    /// be ambiguous. Refused at open, as the harness refuses them.
    ///
    /// The index is the second of the two records, in central directory
    /// order. The name is deliberately not carried: `specs/0008` permits an
    /// escaped name but does not require one, and the escaping rule belongs
    /// to the harness.
    DuplicateName(usize),
    /// No entry has that index.
    NoSuchEntry(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "input or output failed: {}", e.kind()),
            Error::NotZip => f.write_str("no end-of-central-directory record: not a zip archive"),
            Error::Truncated => f.write_str("a recorded offset or size lies beyond the file"),
            Error::Unsupported(what) => write!(f, "unsupported zip feature: {what}"),
            Error::DuplicateName(index) => {
                write!(f, "two central records share a raw name (entry {index})")
            }
            Error::NoSuchEntry(index) => write!(f, "no entry with index {index}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

/// What the layer knows about one entry, and nothing of its data.
#[derive(Debug, Clone)]
pub struct Entry {
    name: Vec<u8>,
    method: u16,
    crc32: u32,
    compressed_size: u64,
    uncompressed_size: u64,
    /// The local header offset as the central record records it: relative to
    /// the start of the archive proper, so the prefix is not included.
    local_offset: u64,
    gp_flags: u16,
    /// The raw central directory record, exactly as it stands in the input.
    central: Vec<u8>,
    /// The raw local file header, fixed part, name and extra field.
    local_header: Vec<u8>,
    /// Absolute file offset of the local header.
    span_start: u64,
    /// Absolute file offset one past the entry's span: the next local header
    /// in file order, or the start of the central directory for the last.
    span_end: u64,
}

impl Entry {
    /// The entry's name, as raw bytes. Never decoded: a name is not
    /// necessarily UTF-8, and re-encoding one would change the archive.
    pub fn name(&self) -> &[u8] {
        &self.name
    }

    /// The compression method id: 0 stored, 8 deflate.
    pub fn method(&self) -> u16 {
        self.method
    }

    /// The CRC-32 of the uncompressed bytes, as the central record records it.
    pub fn crc32(&self) -> u32 {
        self.crc32
    }

    /// The compressed size, from the zip64 extra field where present.
    pub fn compressed_size(&self) -> u64 {
        self.compressed_size
    }

    /// The uncompressed size, from the zip64 extra field where present.
    pub fn uncompressed_size(&self) -> u64 {
        self.uncompressed_size
    }

    /// The local header offset the central record records.
    pub fn local_offset(&self) -> u64 {
        self.local_offset
    }

    /// Whether the entry's sizes and CRC-32 follow its data in a descriptor.
    pub fn has_descriptor(&self) -> bool {
        self.gp_flags & GP_DESCRIPTOR != 0
    }

    /// Whether the entry is a directory, by its name.
    pub fn is_dir(&self) -> bool {
        self.name.last() == Some(&b'/')
    }

    /// Whether the entry is encrypted.
    pub fn is_encrypted(&self) -> bool {
        self.gp_flags & GP_ENCRYPTED != 0
    }

    /// The bytes this entry occupies in the file: its local header, its data,
    /// its data descriptor if it has one, and any padding or orphaned bytes
    /// before the next header.
    fn span_len(&self) -> u64 {
        self.span_end - self.span_start
    }
}

/// How an entry's new bytes are produced. A part 2 concern: a rewritten
/// worksheet can stream straight into the writer rather than being built in
/// memory first.
pub type Replacement = Box<dyn FnOnce(&mut dyn Write) -> io::Result<()>>;

/// A set of changes to apply while writing an archive back.
///
/// Part 1 implements the empty case only; [`Archive::write_to`] refuses a
/// non-empty `Edits`.
#[derive(Default)]
pub struct Edits {
    replacements: Vec<(usize, Replacement)>,
    removals: Vec<usize>,
    additions: Vec<(Vec<u8>, Replacement)>,
}

impl fmt::Debug for Edits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Edits")
            .field("replacements", &self.replacements.len())
            .field("removals", &self.removals.len())
            .field("additions", &self.additions.len())
            .finish()
    }
}

impl Edits {
    /// An empty set of changes: a write is then a verbatim round trip.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the entry at `index` with bytes the replacement writes.
    pub fn replace(&mut self, index: usize, replacement: Replacement) -> &mut Self {
        self.replacements.push((index, replacement));
        self
    }

    /// Removes the entry at `index`.
    pub fn remove(&mut self, index: usize) -> &mut Self {
        self.removals.push(index);
        self
    }

    /// Appends a new entry with the given raw name.
    pub fn add(&mut self, name: Vec<u8>, replacement: Replacement) -> &mut Self {
        self.additions.push((name, replacement));
        self
    }

    /// Whether this set changes nothing.
    pub fn is_empty(&self) -> bool {
        self.replacements.is_empty() && self.removals.is_empty() && self.additions.is_empty()
    }
}

/// An opened zip archive, indexed and ready to be written back.
pub struct Archive<R: Read + Seek> {
    /// The reader, held plain until a [`Archive::read`] needs the `zip` crate.
    /// Exactly one of `reader` and `inner` is `Some` at any time.
    ///
    /// `open` no longer constructs a `ZipArchive`, so what this layer accepts
    /// is decided by this layer's parser alone, as the module header says
    /// (`specs/0008-zip-layer.md`, the P1 rulings of 2026-09-17, item 4).
    reader: Option<R>,
    /// Built on the first `read`, from `reader`.
    inner: Option<zip::ZipArchive<R>>,
    /// Entries in central directory order.
    entries: Vec<Entry>,
    /// Indices into `entries`, ordered by `span_start`. T1 P6 showed file
    /// order can differ from central directory order.
    file_order: Vec<usize>,
    /// What a recorded offset is added to in order to get a file position:
    /// `central_start - central_offset`.
    ///
    /// This is NOT the length of the prefix, and conflating the two was the
    /// defect the independent checks of e14dbee found. They coincide only
    /// when a producer records offsets relative to the archive proper; a
    /// producer that records them file-absolute gives a base of 0 while the
    /// prefix is any length at all. See the P1 rulings of 2026-09-17, item 1.
    offset_base: u64,
    /// Bytes before the first local header in file order, copied verbatim;
    /// `central_start` when the archive has no entries.
    prefix_len: u64,
    /// The archive comment, from the end-of-central-directory record.
    comment: Vec<u8>,
    /// The raw end records, captured so the layer re-emits whatever the input
    /// carried, including a zip64 extensible data sector, with only the
    /// numeric fields rewritten.
    eocd_raw: Vec<u8>,
    zip64_eocd_raw: Option<Vec<u8>>,
    zip64_locator_raw: Option<Vec<u8>>,
}

impl<R: Read + Seek> Archive<R> {
    /// Reads the end records, the central directory and every local header,
    /// and refuses what the layer cannot represent.
    ///
    /// The index holds, per entry, the name bytes, the central record bytes
    /// and the local header bytes, and nothing of the data.
    pub fn open(mut reader: R) -> Result<Self, Error> {
        let file_len = reader.seek(SeekFrom::End(0))?;

        let (eocd_pos, eocd_raw) = find_eocd(&mut reader, file_len)?;
        let eocd = parse_eocd(&eocd_raw)?;

        let zip64 = read_zip64(&mut reader, eocd_pos, file_len)?;

        // Zip64 values win where the 32-bit fields carry the marker.
        let total_entries = match &zip64 {
            Some(z) => z.fields.total_entries,
            None => u64::from(eocd.total_entries),
        };
        let central_size = match &zip64 {
            Some(z) => z.fields.central_size,
            None => u64::from(eocd.central_size),
        };
        let central_offset = match &zip64 {
            Some(z) => z.fields.central_offset,
            None => u64::from(eocd.central_offset),
        };

        // One predicate, one error: the disk fields of the two records mean
        // the same thing, so nesting the zip64 case only duplicated the
        // message and drew clippy's collapsible_if.
        let multi_disk = eocd.disk != 0
            || eocd.central_start_disk != 0
            || zip64
                .as_ref()
                .is_some_and(|z| z.fields.disk != 0 || z.fields.central_start_disk != 0);
        if multi_disk {
            return Err(Error::Unsupported("a multi-disk archive"));
        }

        // The central directory ends where the next record begins, and with
        // zip64 that is the zip64 record, NOT the end-of-central-directory
        // record: the zip64 record and its 20-byte locator sit between the
        // two. Taking eocd_pos here put central_start 76 bytes late, so
        // prefix_len came out 76 rather than 0, every span_start was shifted
        // by it, and the first local header read found no signature. The
        // writer always computed the record's position as following the
        // central directory, so only this reading disagreed.
        let central_end = match &zip64 {
            Some(z) => z.record_position,
            None => eocd_pos,
        };
        let central_start = central_end
            .checked_sub(central_size)
            .ok_or(Error::Truncated)?;

        // THE OFFSET BASE, which is what a recorded offset is added to in
        // order to get a file position. It is NOT the length of the prefix.
        // The two coincide only when a producer records offsets relative to
        // the archive proper; a producer that records them file-absolute
        // gives a base of 0 behind a prefix of any length. Using one value
        // for both dropped the prefix silently, at exit 0 (the P1 rulings of
        // 2026-09-17, item 1).
        let offset_base = central_start
            .checked_sub(central_offset)
            .ok_or(Error::Truncated)?;

        // The locator states the base too. If the two disagree the archive is
        // not one this layer can rewrite without guessing which to believe.
        if let Some(z) = &zip64 {
            let locator_base = z
                .record_position
                .checked_sub(z.locator_offset)
                .ok_or(Error::Truncated)?;
            if locator_base != offset_base {
                return Err(Error::Unsupported(
                    "the zip64 locator and the central directory disagree on the offset base",
                ));
            }
        }

        let mut central = vec![0u8; usize_of(central_size)?];
        reader.seek(SeekFrom::Start(central_start))?;
        reader.read_exact(&mut central)?;

        let mut entries = parse_central(&central, total_entries)?;
        refuse_duplicate_names(&entries)?;

        // Each entry's local header, read where the central record says it
        // is. Data is never read here.
        for entry in entries.iter_mut() {
            let start = offset_base
                .checked_add(entry.local_offset)
                .ok_or(Error::Truncated)?;
            if start.checked_add(LOCAL_FIXED).ok_or(Error::Truncated)? > file_len {
                return Err(Error::Truncated);
            }
            entry.span_start = start;
            entry.local_header = read_local_header(&mut reader, start)?;
        }

        // File order, and from it each span's end.
        let mut file_order: Vec<usize> = (0..entries.len()).collect();
        file_order.sort_by_key(|&i| entries[i].span_start);
        for position in 0..file_order.len() {
            let this = file_order[position];
            let end = match file_order.get(position + 1) {
                Some(&next) => entries[next].span_start,
                None => central_start,
            };
            if end < entries[this].span_start {
                return Err(Error::Truncated);
            }
            entries[this].span_end = end;
        }

        // THE PREFIX: the bytes before the first local header in file order,
        // whatever the recorded offsets say. With no entries there is no
        // local header, so everything up to the central directory is prefix.
        let prefix_len = match file_order.first() {
            Some(&first) => entries[first].span_start,
            None => central_start,
        };

        let comment = eocd.comment.clone();
        // The end records are kept as raw bytes so the writer re-emits
        // whatever the input carried, extensible data sector included, with
        // only the numeric fields rewritten.
        let (zip64_locator_raw, zip64_eocd_raw) = match zip64 {
            Some(z) => (Some(z.locator_raw), Some(z.record_raw)),
            None => (None, None),
        };

        Ok(Archive {
            reader: Some(reader),
            inner: None,
            entries,
            file_order,
            offset_base,
            prefix_len,
            comment,
            eocd_raw,
            zip64_eocd_raw,
            zip64_locator_raw,
        })
    }

    /// Every entry, in central directory order.
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// The index of the entry with this raw name, if there is one. Names are
    /// unique: [`Archive::open`] refuses an archive whose central directory
    /// repeats one.
    pub fn index_of(&self, name: &[u8]) -> Option<usize> {
        self.entries.iter().position(|e| e.name == name)
    }

    /// The archive comment.
    pub fn comment(&self) -> &[u8] {
        &self.comment
    }

    /// The number of bytes before the first local header in file order, or
    /// the bytes before the central directory when the archive has no
    /// entries.
    ///
    /// This is a measured position, not `central_start - central_offset`:
    /// the two differ whenever a producer records file-absolute offsets.
    pub fn prefix_len(&self) -> u64 {
        self.prefix_len
    }

    /// The entry's decompressed bytes, through the `zip` crate.
    pub fn read(&mut self, index: usize) -> Result<impl Read + '_, Error> {
        if index >= self.entries.len() {
            return Err(Error::NoSuchEntry(index));
        }
        if self.entries[index].is_encrypted() {
            return Err(Error::Unsupported("reading an encrypted entry"));
        }
        // The `zip` archive is built here, on the first read, and not in
        // `open`: what this layer ACCEPTS is then decided by this layer's
        // parser alone, as the module header claims, rather than gated on the
        // crate's agreement (the P1 rulings of 2026-09-17, item 4). An
        // archive this layer can parse can therefore always be byte-copied,
        // even if the crate would refuse to decompress an entry of it.
        if self.inner.is_none() {
            let reader = self.take_reader()?;
            self.inner = Some(zip::ZipArchive::new(reader).map_err(zip_open_error)?);
        }
        let inner = self
            .inner
            .as_mut()
            .ok_or(Error::Unsupported("the archive's reader is checked out"))?;
        inner.by_index(index).map_err(zip_read_error)
    }

    /// Writes the archive to `out`.
    ///
    /// The output is a plain [`Write`]: offsets are computed as bytes go out,
    /// so it can be a pipe.
    ///
    /// Part 1 implements the empty-`edits` case, which is a verbatim round
    /// trip: the output is byte-identical to the input, prefix and comment
    /// included. A non-empty `edits` is refused until part 2.
    pub fn write_to(&mut self, mut out: impl Write, edits: Edits) -> Result<(), Error> {
        if !edits.is_empty() {
            return Err(Error::Unsupported(
                "replace, remove and add arrive in Milestone 1 part 1 P2",
            ));
        }

        let mut reader = self.take_reader()?;
        let result = self.write_all_parts(&mut reader, &mut out);
        // The reader goes back plain, whatever happened, so a failed write
        // leaves a usable Archive rather than one that has lost it. Nothing
        // is rebuilt here, so no recovery failure can mask the write's own
        // error; a later read builds the `zip` archive again from this
        // reader.
        self.reader = Some(reader);
        result
    }

    /// The reader, from wherever it currently lives.
    ///
    /// Exactly one of `reader` and `inner` holds it. Both being `None` means
    /// a previous call panicked between taking and returning it, which no
    /// path here does.
    fn take_reader(&mut self) -> Result<R, Error> {
        if let Some(reader) = self.reader.take() {
            return Ok(reader);
        }
        if let Some(archive) = self.inner.take() {
            return Ok(archive.into_inner());
        }
        Err(Error::Unsupported("the archive's reader is checked out"))
    }

    /// The write itself, with the reader in hand.
    fn write_all_parts(&self, reader: &mut R, out: &mut impl Write) -> Result<(), Error> {
        // 1. The prefix, verbatim.
        reader.seek(SeekFrom::Start(0))?;
        copy_exact(reader, out, self.prefix_len)?;

        // 2. Every entry's span, in file order, verbatim. With no edits the
        //    new offsets equal the old ones, which is why the round trip is
        //    byte-identical; they are still computed rather than assumed, so
        //    part 2 inherits a writer that already tracks them.
        //    Every recorded offset is a file position MINUS THE OFFSET BASE,
        //    never minus the prefix length: that is what keeps the output in
        //    the input's own convention, and conflating the two is what
        //    dropped the prefix (the P1 rulings of 2026-09-17, item 1).
        let mut written = self.prefix_len;
        let mut new_offsets = vec![0u64; self.entries.len()];
        for &index in &self.file_order {
            let entry = &self.entries[index];
            new_offsets[index] = written
                .checked_sub(self.offset_base)
                .ok_or(Error::Truncated)?;
            reader.seek(SeekFrom::Start(entry.span_start))?;
            copy_exact(reader, out, entry.span_len())?;
            written += entry.span_len();
        }

        // 3. The central directory, in the original order, each record
        //    verbatim but for its local header offset.
        let central_offset = written
            .checked_sub(self.offset_base)
            .ok_or(Error::Truncated)?;
        let mut central_size = 0u64;
        for (index, entry) in self.entries.iter().enumerate() {
            let record = rewrite_central_offset(&entry.central, new_offsets[index])?;
            out.write_all(&record)?;
            central_size += record.len() as u64;
            written += record.len() as u64;
        }

        // 4. The end records, as the input carried them, with the counts,
        //    sizes and offsets rewritten and nothing else. `written` is now a
        //    real position, the zip64 record's own, rather than a sum
        //    recomputed from the prefix.
        let total = self.entries.len() as u64;
        if let (Some(z64), Some(locator)) = (&self.zip64_eocd_raw, &self.zip64_locator_raw) {
            let z64_position = written;
            let record = rewrite_zip64_eocd(z64, total, central_size, central_offset)?;
            out.write_all(&record)?;
            let recorded = z64_position
                .checked_sub(self.offset_base)
                .ok_or(Error::Truncated)?;
            let locator = rewrite_zip64_locator(locator, recorded)?;
            out.write_all(&locator)?;
        }
        let eocd = rewrite_eocd(&self.eocd_raw, total, central_size, central_offset)?;
        out.write_all(&eocd)?;
        out.flush()?;
        Ok(())
    }
}

/// Scans backwards for the end-of-central-directory signature and returns its
/// position and its bytes.
fn find_eocd<R: Read + Seek>(reader: &mut R, file_len: u64) -> Result<(u64, Vec<u8>), Error> {
    if file_len < EOCD_FIXED {
        return Err(Error::NotZip);
    }
    let window = (EOCD_FIXED + MAX_COMMENT).min(file_len);
    let start = file_len - window;
    let mut buffer = vec![0u8; usize_of(window)?];
    reader.seek(SeekFrom::Start(start))?;
    reader.read_exact(&mut buffer)?;

    let fixed = usize_of(EOCD_FIXED)?;
    // The last candidate wins: a record whose own comment contains the
    // signature would otherwise shadow the real one.
    let mut found = None;
    for offset in (0..=buffer.len().saturating_sub(fixed)).rev() {
        if read_u32(&buffer[offset..])? != EOCD_SIG {
            continue;
        }
        let comment_len = usize::from(read_u16(&buffer[offset + 20..])?);
        if offset + fixed + comment_len == buffer.len() {
            found = Some(offset);
            break;
        }
    }
    let offset = found.ok_or(Error::NotZip)?;
    Ok((start + offset as u64, buffer[offset..].to_vec()))
}

/// The fields of an end-of-central-directory record.
struct Eocd {
    disk: u16,
    central_start_disk: u16,
    total_entries: u16,
    central_size: u32,
    central_offset: u32,
    comment: Vec<u8>,
}

fn parse_eocd(raw: &[u8]) -> Result<Eocd, Error> {
    if raw.len() < usize_of(EOCD_FIXED)? {
        return Err(Error::NotZip);
    }
    let comment_len = usize::from(read_u16(&raw[20..])?);
    let fixed = usize_of(EOCD_FIXED)?;
    if raw.len() != fixed + comment_len {
        return Err(Error::Truncated);
    }
    Ok(Eocd {
        disk: read_u16(&raw[4..])?,
        central_start_disk: read_u16(&raw[6..])?,
        total_entries: read_u16(&raw[10..])?,
        central_size: read_u32(&raw[12..])?,
        central_offset: read_u32(&raw[16..])?,
        comment: raw[fixed..].to_vec(),
    })
}

/// The fields of a zip64 end-of-central-directory record.
struct Zip64Eocd {
    disk: u32,
    central_start_disk: u32,
    total_entries: u64,
    central_size: u64,
    central_offset: u64,
}

/// What the end of a zip64 archive carries.
///
/// `record_position` is where the record ACTUALLY is, found by its signature;
/// `locator_offset` is what the locator RECORDS. Their difference is the
/// offset base as the locator states it, which `open` requires to agree with
/// the base the central directory gives.
struct Zip64End {
    locator_raw: Vec<u8>,
    record_raw: Vec<u8>,
    fields: Zip64Eocd,
    record_position: u64,
    locator_offset: u64,
}

/// The furthest back a zip64 end record's extensible data sector is followed
/// when scanning for its signature.
const MAX_ZIP64_SCAN: u64 = 0xFFFF;

/// Reads the zip64 locator and record when the locator sits immediately
/// before the end-of-central-directory record.
///
/// The record is found BY ITS SIGNATURE, scanning back from the locator, and
/// not by seeking to the offset the locator records. An earlier version
/// seeked to that offset as though it were a file position, which is only one
/// of the two conventions in use: an archive behind a prefix whose locator
/// records an archive-relative offset was refused as "a zip64 locator without
/// its record" (`specs/0008-zip-layer.md`, the P1 rulings of 2026-09-17,
/// item 2). The record must end exactly where the locator begins, which is
/// what makes a positional search safe.
fn read_zip64<R: Read + Seek>(
    reader: &mut R,
    eocd_pos: u64,
    _file_len: u64,
) -> Result<Option<Zip64End>, Error> {
    if eocd_pos < ZIP64_LOCATOR_LEN {
        return Ok(None);
    }
    let locator_pos = eocd_pos - ZIP64_LOCATOR_LEN;
    let mut locator = vec![0u8; usize_of(ZIP64_LOCATOR_LEN)?];
    reader.seek(SeekFrom::Start(locator_pos))?;
    reader.read_exact(&mut locator)?;
    if read_u32(&locator)? != ZIP64_LOCATOR_SIG {
        return Ok(None);
    }
    let locator_offset = read_u64(&locator[8..])?;

    // The window that could hold the record: its fixed part plus whatever
    // extensible data sector it carries, ending where the locator begins.
    if locator_pos < ZIP64_EOCD_FIXED {
        return Err(Error::Unsupported("a zip64 locator without its record"));
    }
    let window = (ZIP64_EOCD_FIXED + MAX_ZIP64_SCAN).min(locator_pos);
    let window_start = locator_pos - window;
    let mut buffer = vec![0u8; usize_of(window)?];
    reader.seek(SeekFrom::Start(window_start))?;
    reader.read_exact(&mut buffer)?;

    // The record's own size field covers everything after its first 12 bytes,
    // so a candidate is the record only if it ends exactly at the locator.
    let fixed = usize_of(ZIP64_EOCD_FIXED)?;
    let mut found = None;
    for at in (0..=buffer.len().saturating_sub(fixed)).rev() {
        if read_u32(&buffer[at..])? != ZIP64_EOCD_SIG {
            continue;
        }
        let recorded = read_u64(&buffer[at + 4..])?;
        let total_len = match recorded.checked_add(12) {
            Some(n) => n,
            None => continue,
        };
        if total_len < ZIP64_EOCD_FIXED {
            continue;
        }
        if (at as u64).checked_add(total_len) == Some(window) {
            found = Some((at, usize_of(total_len)?));
            break;
        }
    }
    let (at, total) = found.ok_or(Error::Unsupported("a zip64 locator without its record"))?;
    let raw = buffer[at..at + total].to_vec();
    let record_position = window_start + at as u64;

    let parsed = Zip64Eocd {
        disk: read_u32(&raw[16..])?,
        central_start_disk: read_u32(&raw[20..])?,
        total_entries: read_u64(&raw[32..])?,
        central_size: read_u64(&raw[40..])?,
        central_offset: read_u64(&raw[48..])?,
    };
    Ok(Some(Zip64End {
        locator_raw: locator,
        record_raw: raw,
        fields: parsed,
        record_position,
        locator_offset,
    }))
}

/// Parses every central directory record into an [`Entry`].
fn parse_central(central: &[u8], total_entries: u64) -> Result<Vec<Entry>, Error> {
    let mut entries = Vec::new();
    let mut at = 0usize;
    while at + CENTRAL_FIXED <= central.len() {
        if read_u32(&central[at..])? != CENTRAL_SIG {
            break;
        }
        let gp_flags = read_u16(&central[at + 8..])?;
        let method = read_u16(&central[at + 10..])?;
        let crc32 = read_u32(&central[at + 16..])?;
        let compressed = read_u32(&central[at + 20..])?;
        let uncompressed = read_u32(&central[at + 24..])?;
        let name_len = usize::from(read_u16(&central[at + 28..])?);
        let extra_len = usize::from(read_u16(&central[at + 30..])?);
        let comment_len = usize::from(read_u16(&central[at + 32..])?);
        let disk_start = read_u16(&central[at + 34..])?;
        let local_offset = read_u32(&central[at + 42..])?;

        let record_len = CENTRAL_FIXED + name_len + extra_len + comment_len;
        if at + record_len > central.len() {
            return Err(Error::Truncated);
        }
        let name = central[at + CENTRAL_FIXED..at + CENTRAL_FIXED + name_len].to_vec();
        let extra =
            &central[at + CENTRAL_FIXED + name_len..at + CENTRAL_FIXED + name_len + extra_len];

        // The zip64 extra field supplies whichever fields carry the marker,
        // in a fixed order and only those.
        let z = zip64_extra(extra, uncompressed, compressed, local_offset, disk_start)?;
        if z.disk_start != 0 {
            return Err(Error::Unsupported("a multi-disk archive"));
        }

        entries.push(Entry {
            name,
            method,
            crc32,
            compressed_size: z.compressed,
            uncompressed_size: z.uncompressed,
            local_offset: z.local_offset,
            gp_flags,
            central: central[at..at + record_len].to_vec(),
            local_header: Vec::new(),
            span_start: 0,
            span_end: 0,
        });
        at += record_len;
    }

    if entries.len() as u64 != total_entries {
        return Err(Error::Truncated);
    }
    Ok(entries)
}

/// The values a central record carries once its zip64 extra field is applied.
struct Zip64Fields {
    uncompressed: u64,
    compressed: u64,
    local_offset: u64,
    disk_start: u32,
}

fn zip64_extra(
    extra: &[u8],
    uncompressed: u32,
    compressed: u32,
    local_offset: u32,
    disk_start: u16,
) -> Result<Zip64Fields, Error> {
    let mut fields = Zip64Fields {
        uncompressed: u64::from(uncompressed),
        compressed: u64::from(compressed),
        local_offset: u64::from(local_offset),
        disk_start: u32::from(disk_start),
    };
    let mut at = 0usize;
    while at + 4 <= extra.len() {
        let id = read_u16(&extra[at..])?;
        let size = usize::from(read_u16(&extra[at + 2..])?);
        if at + 4 + size > extra.len() {
            return Err(Error::Truncated);
        }
        if id == ZIP64_EXTRA_ID {
            let body = &extra[at + 4..at + 4 + size];
            let mut cursor = 0usize;
            if uncompressed == U32_MAX_MARKER {
                fields.uncompressed = take_u64(body, &mut cursor)?;
            }
            if compressed == U32_MAX_MARKER {
                fields.compressed = take_u64(body, &mut cursor)?;
            }
            if local_offset == U32_MAX_MARKER {
                fields.local_offset = take_u64(body, &mut cursor)?;
            }
            if disk_start == U16_MAX_MARKER {
                if cursor + 4 > body.len() {
                    return Err(Error::Truncated);
                }
                fields.disk_start = read_u32(&body[cursor..])?;
            }
            return Ok(fields);
        }
        at += 4 + size;
    }
    if uncompressed == U32_MAX_MARKER
        || compressed == U32_MAX_MARKER
        || local_offset == U32_MAX_MARKER
        || disk_start == U16_MAX_MARKER
    {
        return Err(Error::Unsupported(
            "a zip64 marker with no zip64 extra field",
        ));
    }
    Ok(fields)
}

fn take_u64(body: &[u8], cursor: &mut usize) -> Result<u64, Error> {
    if *cursor + 8 > body.len() {
        return Err(Error::Truncated);
    }
    let value = read_u64(&body[*cursor..])?;
    *cursor += 8;
    Ok(value)
}

/// Refuses two central records with the same raw name, since an edit by name
/// would be ambiguous.
fn refuse_duplicate_names(entries: &[Entry]) -> Result<(), Error> {
    let mut seen: Vec<&[u8]> = Vec::with_capacity(entries.len());
    for (index, entry) in entries.iter().enumerate() {
        if seen.contains(&entry.name.as_slice()) {
            return Err(Error::DuplicateName(index));
        }
        seen.push(&entry.name);
    }
    Ok(())
}

/// Reads one local file header: the fixed part, the name and the extra field.
fn read_local_header<R: Read + Seek>(reader: &mut R, start: u64) -> Result<Vec<u8>, Error> {
    let mut fixed = vec![0u8; usize_of(LOCAL_FIXED)?];
    reader.seek(SeekFrom::Start(start))?;
    reader.read_exact(&mut fixed)?;
    if read_u32(&fixed)? != LOCAL_SIG {
        return Err(Error::Truncated);
    }
    let name_len = usize::from(read_u16(&fixed[26..])?);
    let extra_len = usize::from(read_u16(&fixed[28..])?);
    let mut rest = vec![0u8; name_len + extra_len];
    reader.read_exact(&mut rest)?;
    fixed.extend_from_slice(&rest);
    Ok(fixed)
}

/// Copies one central record with its local header offset rewritten, in the
/// fixed field or in the zip64 extra field when the record carries the marker.
fn rewrite_central_offset(record: &[u8], new_offset: u64) -> Result<Vec<u8>, Error> {
    let mut out = record.to_vec();
    let recorded = read_u32(&out[42..])?;
    if recorded != U32_MAX_MARKER {
        if new_offset > u64::from(U32_MAX_MARKER) {
            return Err(Error::Unsupported(
                "an offset that would need a zip64 field the input does not carry",
            ));
        }
        let bytes = (new_offset as u32).to_le_bytes();
        out[42..46].copy_from_slice(&bytes);
        return Ok(out);
    }

    // The marker is set, so the real offset lives in the zip64 extra field,
    // after whichever of the two sizes also carry the marker.
    let uncompressed = read_u32(&out[24..])?;
    let compressed = read_u32(&out[20..])?;
    let name_len = usize::from(read_u16(&out[28..])?);
    let extra_len = usize::from(read_u16(&out[30..])?);
    let extra_start = CENTRAL_FIXED + name_len;
    let extra_end = extra_start + extra_len;
    if extra_end > out.len() {
        return Err(Error::Truncated);
    }

    let mut at = extra_start;
    while at + 4 <= extra_end {
        let id = read_u16(&out[at..])?;
        let size = usize::from(read_u16(&out[at + 2..])?);
        if at + 4 + size > extra_end {
            return Err(Error::Truncated);
        }
        if id == ZIP64_EXTRA_ID {
            let body = at + 4;
            let mut cursor = 0usize;
            if uncompressed == U32_MAX_MARKER {
                cursor += 8;
            }
            if compressed == U32_MAX_MARKER {
                cursor += 8;
            }
            if cursor + 8 > size {
                return Err(Error::Truncated);
            }
            out[body + cursor..body + cursor + 8].copy_from_slice(&new_offset.to_le_bytes());
            return Ok(out);
        }
        at += 4 + size;
    }
    Err(Error::Unsupported(
        "a zip64 offset marker with no zip64 extra field",
    ))
}

/// Copies the end-of-central-directory record with its counts, size and
/// offset rewritten, and nothing else. A field already carrying the zip64
/// marker keeps it.
fn rewrite_eocd(raw: &[u8], total: u64, size: u64, offset: u64) -> Result<Vec<u8>, Error> {
    let mut out = raw.to_vec();
    if read_u16(&out[8..])? != U16_MAX_MARKER {
        out[8..10].copy_from_slice(&clamp_u16(total).to_le_bytes());
    }
    if read_u16(&out[10..])? != U16_MAX_MARKER {
        out[10..12].copy_from_slice(&clamp_u16(total).to_le_bytes());
    }
    if read_u32(&out[12..])? != U32_MAX_MARKER {
        out[12..16].copy_from_slice(&clamp_u32(size).to_le_bytes());
    }
    if read_u32(&out[16..])? != U32_MAX_MARKER {
        out[16..20].copy_from_slice(&clamp_u32(offset).to_le_bytes());
    }
    Ok(out)
}

/// Copies the zip64 end-of-central-directory record with its counts, size and
/// offset rewritten. The extensible data sector is carried across untouched.
fn rewrite_zip64_eocd(raw: &[u8], total: u64, size: u64, offset: u64) -> Result<Vec<u8>, Error> {
    let mut out = raw.to_vec();
    if out.len() < usize_of(ZIP64_EOCD_FIXED)? {
        return Err(Error::Truncated);
    }
    out[24..32].copy_from_slice(&total.to_le_bytes());
    out[32..40].copy_from_slice(&total.to_le_bytes());
    out[40..48].copy_from_slice(&size.to_le_bytes());
    out[48..56].copy_from_slice(&offset.to_le_bytes());
    Ok(out)
}

/// Copies the zip64 locator with the record's new position.
fn rewrite_zip64_locator(raw: &[u8], record_position: u64) -> Result<Vec<u8>, Error> {
    let mut out = raw.to_vec();
    if out.len() < usize_of(ZIP64_LOCATOR_LEN)? {
        return Err(Error::Truncated);
    }
    out[8..16].copy_from_slice(&record_position.to_le_bytes());
    Ok(out)
}

fn clamp_u16(value: u64) -> u16 {
    if value > u64::from(U16_MAX_MARKER) {
        U16_MAX_MARKER
    } else {
        value as u16
    }
}

fn clamp_u32(value: u64) -> u32 {
    if value > u64::from(U32_MAX_MARKER) {
        U32_MAX_MARKER
    } else {
        value as u32
    }
}

/// Copies exactly `len` bytes through a fixed buffer, so a large entry never
/// lands in memory whole.
fn copy_exact<R: Read, W: Write>(reader: &mut R, out: &mut W, len: u64) -> Result<(), Error> {
    // On the heap, not the stack: `usize` is 32 bits on
    // i686-unknown-linux-musl and a musl thread's default stack is far
    // smaller than the main thread's, so a 64 KiB frame leaves a consumer
    // calling from a spawned thread little margin (independent check 2 of
    // e14dbee; the P1 rulings of 2026-09-17, item 4).
    let mut buffer = vec![0u8; COPY_BUFFER];
    let mut left = len;
    while left > 0 {
        let want = left.min(COPY_BUFFER as u64) as usize;
        reader.read_exact(&mut buffer[..want])?;
        out.write_all(&buffer[..want])?;
        left -= want as u64;
    }
    Ok(())
}

/// A `u64` as a `usize`, refused rather than truncated where `usize` is 32
/// bits. Every caller here sizes a buffer, so a value this large is a
/// structure the layer cannot hold, not a stream it could copy.
fn usize_of(value: u64) -> Result<usize, Error> {
    usize::try_from(value)
        .map_err(|_| Error::Unsupported("a structure larger than this target's address space"))
}

fn read_u16(bytes: &[u8]) -> Result<u16, Error> {
    let slice: [u8; 2] = bytes.get(..2).ok_or(Error::Truncated)?.try_into().unwrap();
    Ok(u16::from_le_bytes(slice))
}

fn read_u32(bytes: &[u8]) -> Result<u32, Error> {
    let slice: [u8; 4] = bytes.get(..4).ok_or(Error::Truncated)?.try_into().unwrap();
    Ok(u32::from_le_bytes(slice))
}

fn read_u64(bytes: &[u8]) -> Result<u64, Error> {
    let slice: [u8; 8] = bytes.get(..8).ok_or(Error::Truncated)?.try_into().unwrap();
    Ok(u64::from_le_bytes(slice))
}

/// Maps a `zip` crate error from opening an archive, carrying no crate text.
fn zip_open_error(err: zip::result::ZipError) -> Error {
    match err {
        zip::result::ZipError::Io(e) => Error::Io(e),
        zip::result::ZipError::UnsupportedArchive(_)
        | zip::result::ZipError::CompressionMethodNotSupported(_) => {
            Error::Unsupported("a zip feature the zip crate refused")
        }
        _ => Error::NotZip,
    }
}

/// Maps a `zip` crate error from reading an entry, carrying no crate text.
fn zip_read_error(err: zip::result::ZipError) -> Error {
    match err {
        zip::result::ZipError::Io(e) => Error::Io(e),
        zip::result::ZipError::FileNotFound => Error::NoSuchEntry(usize::MAX),
        _ => Error::Unsupported("an entry the zip crate could not decompress"),
    }
}
