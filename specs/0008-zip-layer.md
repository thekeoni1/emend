# 0008: Milestone 1 part 1, the zip layer

Status: P1 READY (planning draft 2026-09-16; approved by the maintainer
the same day; entered the repository with T6 P1's verification).
Context: `CLAUDE.md` (architecture, dependencies, 32-bit and sizes),
`records/decision-architecture.md`, `ROADMAP.md` ("Milestone 1"),
`specs/0004-xlsx-diff.md` (the harness's rows, flags and exit status),
`specs/0006-passthrough.md` (the T3 P1 finding table, the runner, the
record script, the size baseline), `specs/0001-bootstrap.md` ("Rules
for every phase").

## Goal

The core's first module: open an `.xlsx` as a zip archive, index every
entry with `u64` offsets, and write the archive back with every
untouched entry byte for byte in its original place and every touched
entry rewritten by streaming. The harness is the acceptance: a no-edit
round trip is byte-identical to the input, and an edit leaves every
other entry `identical` with an empty flags column.

## Facts planning checked (2026-09-16)

- `zip` 8.6.0 re-exports no deflate encoder or decoder and does not
  re-export `flate2` (its `lib.rs` `pub use` list). Within the
  dependency rule, the crate's own `ZipArchive` (decompressing reads)
  and `ZipWriter` (compressing writes) are the only compression paths.
- Per entry, `ZipArchive` exposes `header_start`, `data_start`,
  `central_header_start`, `compressed_size`, `size`, `crc32`,
  `compression`, `extra_data`, `name_raw`, `encrypted`,
  `version_made_by` and `last_modified`; per archive, the prefix
  offset and the comment. It does not expose the raw central record,
  the raw local header, the general purpose flags or the data
  descriptor. The layer parses those itself.
- T3 P1: the crate's writer is not byte-preserving. T3 P4: with the
  crate's raw copy every one of the 870 corpus entries stayed
  `identical` but every one was flagged (`attrs` 870, `extra` 578,
  `gpflags`, `descriptor` and `local` 292), and no file was
  byte-identical.
- `tools/xlsx-diff` has its own header parser. The core shares no code
  with it: the harness stays an independent instrument, so a parsing
  bug shared by both could not hide itself.
- Size baseline (T3 P2): `probe-zip` 473636 bytes, `probe-zip-xml`
  539172 bytes, i686 musl, the `size` profile. Budget: provisional
  250 KiB above the second, to be re-ruled by the maintainer when
  Milestone 1 starts.
- On the 32-bit target `usize` is 32 bits. The two `large` corpus
  files hold a 63.7 MB entry each; the harness's ignored zip64 test
  builds an archive over 4 GiB.

## Design

### Crate

`crates/emend`, a library, `publish = false` until the maintainer says
otherwise, edition 2024, `license.workspace = true`, added to the
workspace `members` (an ordinary code-commit edit, not a departure). Dependencies in
part 1: `zip` only, pinned and featured as the tools are (`=8.6.0`,
`default-features = false`, `deflate-flate2-zlib-rs`). `quick-xml`
joins in part 2, so part 1's size measurement is the zip layer alone.
No dev-dependencies: tests build archives byte by byte, as T1 did.

### Module `archive`

- `Archive<R: Read + Seek>`: `open(reader) -> Result<Self, Error>`
  reads the end-of-central-directory record, the zip64 records when
  present, the central directory and every local header, and refuses
  what it cannot represent (below). `entries() -> &[Entry]` in central
  directory order; `index_of(name: &[u8]) -> Option<usize>`;
  `read(index) -> Result<impl Read + '_, Error>` gives the
  decompressed bytes through the `zip` crate; `comment() -> &[u8]`;
  `prefix_len() -> u64`.
- `Entry`: `name() -> &[u8]`, `method() -> u16`, `crc32() -> u32`,
  `compressed_size() -> u64`, `uncompressed_size() -> u64`,
  `local_offset() -> u64`, `has_descriptor() -> bool`,
  `is_dir() -> bool`, `is_encrypted() -> bool`.
- `Edits`: a set of changes keyed by entry index, plus additions:
  `replace(index, Replacement)`, `remove(index)`,
  `add(name: Vec<u8>, Replacement)`. `Replacement` is
  `Box<dyn FnOnce(&mut dyn Write) -> io::Result<()>>`, so a later part
  can stream a rewritten worksheet straight into it; a reader is
  wrapped with `io::copy`.
- `Archive::write_to(&mut self, out: impl Write, edits: Edits) ->
  Result<(), Error>`. The output is a plain `Write`, no `Seek`:
  offsets are computed as bytes go out, so the output can be a pipe.

### What the writer preserves

- The prefix (bytes before the first local header) is copied first,
  verbatim.
- Entries are written in file order (ascending local offset), which
  T1 P6 showed can differ from central directory order. An untouched
  entry's span, from its local header to the next local header in
  file order (or to the central directory for the last), is copied
  verbatim: header, data, data descriptor, padding and any orphaned
  bytes inside the span included.
- The central directory is written in the original order. An
  untouched entry's record is copied verbatim except the local header
  offset field, or the zip64 extra field's offset when the record
  carries the 0xFFFFFFFF marker. Extra fields, comments, attributes and
  versions are untouched.
- The end: a zip64 end-of-central-directory record and locator when
  the input had them or the output needs them (a count, size or offset
  at a threshold), then the end-of-central-directory record with the
  original comment; counts, sizes and offsets updated, nothing else.
- Consequence, and P1's acceptance: a no-edit round trip is
  byte-identical to the input, prefix and comment included.

### Touched entries

- Replace: the new local header is the original's bytes with the
  CRC-32 and the two sizes rewritten (zip64 extra field included when
  the original carried one), and, when the original used a data
  descriptor, zeros in the header and a descriptor after the data in
  the original's form (with or without signature, 4- or 8-byte
  sizes). Version needed, flags, method, DOS time and date, name and
  extra field are the original's. The method is the original's: stored
  or deflate; a replacement for an entry of any other method is an
  error. The data streams through the `zip` crate's `ZipWriter` into a
  temporary file, never into memory, and the compressed bytes, CRC-32
  and sizes are taken from there; a stored entry takes the same path
  with the stored method, so the CRC-32 is the crate's. The central
  record is the original's bytes with the CRC-32, the sizes and the
  offset rewritten.
- Remove: the span and the record are omitted.
- Add: a new local header and record, version needed 20 (45 with
  zip64), flags 0 (no descriptor), deflate, DOS time and date zero
  (1980-01-01, so the output is deterministic), no extra field, no
  comment; appended after the last entry in file order and in the
  central directory.
- Harness expectation for a replaced entry: status `changed` or
  `recompressed` and an empty flags column, because every header field
  the change does not require is the original's. A flag on a replaced
  entry is a finding. Every other entry `identical` and unflagged.

### Errors

`Error`: `Io`, `NotZip` (no end-of-central-directory record),
`Truncated` (an offset or size beyond the file), `Unsupported`
(encrypted entry touched or read; a touched entry with a method other
than stored or deflate; a multi-disk archive; a zip64 record the layer
cannot follow), `DuplicateName` (two central records with the same raw
name; refused at open, since an edit by name would be ambiguous, and
the harness refuses them too), `NoSuchEntry`. Messages never carry
entry content; they may carry a name escaped as the harness escapes
names.

### 32-bit and streaming

Every offset, size and count is `u64`. The index holds, per entry, the
name bytes, the central record bytes and the local header bytes (30
fixed bytes plus name and extra field), and nothing of the data. Copies
go through a 64 KiB buffer. A replacement streams to a temporary file,
then to the output. Tests run on both targets; the ignored zip64 test
(a replaced entry in an archive over 4 GiB) runs on both when the
layer's size or offset code changes, as T1 P5 ruled for the harness.

### Size

`tools/size-probe` gains `probe-emend-copy`: opens a file with the
layer and writes it back with no edits. Built in the `size` profile on
i686 musl, its size minus `probe-zip` (473636) is the zip layer's cost;
its size minus `probe-zip-xml` (539172) is the number the budget is
measured against once `quick-xml` is linked. Both go in the record.

### Harness runs

`scripts/passthrough-run.sh` takes any copy tool through
`--passthrough`, so the corpus runs use it unchanged with
`tools/emend-copy` (`emend-copy <in> <out> [--replace <name>]`,
exit 0 or 2, errors on stderr as `error: ...`). The record script
writes the records with `--stem`. Raw outputs stay under the corpus
directory's `runs/`.

## Phases

Each phase ends as in `specs/0001-bootstrap.md`: the doc commit, the
code commit(s), one record commit with the logs and the check answers;
logs in `records/logs/m1p1-<phase>/` (`gate.log`, `test.log` on both
targets, `negative.log`, `scan.log`, and `run.log` where a corpus run
is made); at most two independent checks in plain ASCII; report; STOP.
No push during the part; the close-out push comes after P3.

### P1: open, index, verbatim write

`crates/emend` with `archive::{Archive, Entry, Edits, Error}`;
`write_to` with an empty `Edits` only. Unit tests on byte-built
archives: stored and deflated entries; a data descriptor with and
without signature; zip64 sizes and offsets (an archive built with the
markers, not a large one); a prefix; an archive comment and entry
comments; unknown extra fields in both headers; file order different
from central order; padding between entries; a name with non-ASCII
bytes; a directory entry; an empty archive; and the refusals:
truncated, not a zip, duplicate names, an encrypted entry read, a
multi-disk record. Every test asserts the output bytes, not the
crate's reading of them. `tools/emend-copy` without `--replace`. The
corpus run on both targets with the LibreOffice check, recorded as
`records/m1p1-p1-<utc-date>-<shortsha>.md` and `.txt`. Acceptance:
every entry `identical` with an empty flags column, `files_identical`
`yes`, diff exit 0, `lo` and `lo_in` `ok`, on every corpus file on both
targets; the gate eight PASS; `negative.log` covers the tool's exit
codes and refusals on generated files.

### P2: replace, remove, add

`Edits` in full; `--replace <name>` in `emend-copy` replaces the named
entry with its own decompressed bytes. Tests: the descriptor form kept;
zip64 sizes on a replaced entry; a stored original; a deflated
original; remove; add; order preserved around a removed and an added
entry; a replacement error leaves no output file; the ignored 4 GiB
case on both targets. The corpus run replaces `[Content_Types].xml` in
every file: expected `recompressed` (or `identical`) with an empty
flags column for that entry, every other entry `identical` unflagged,
`lo` and `lo_in` `ok`, recorded as `records/m1p1-p2-<utc-date>-
<shortsha>.md` and `.txt`.

### P3: size and the budget

`probe-emend-copy`, built and measured as the baseline was, recorded
as `records/size-m1p1-<utc-date>-<shortsha>.md` with both deltas; the
error catalogue in the crate's documentation; the report asks the
maintainer to re-rule the size budget with the measured number in
hand.

### Close-out push (after P3)

As `specs/0005-corpus.md`, "Close-out push (after P7)".

## Out of scope

- Parsing any XML; parts 2 to 8 of Milestone 1.
- Methods other than stored and deflate for touched entries;
  encryption; multi-disk archives; repairing a broken archive.
- Changing an untouched entry's bytes for any reason.
