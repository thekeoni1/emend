# 0008: Milestone 1 part 1, the zip layer

Status: P1 DONE 2026-09-17 (doc acc36fb, 791d0d7, c302ec3; code
e14dbee, cb2b4da; record ff92a11); P2 READY. Planning draft
2026-09-16; approved by the maintainer the same day; entered the
repository with T6 P1's verification.
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
`--passthrough` for the output path; its leftover accounting matched
only the passthrough tool's temporary name until the P1 ruling in
`specs/0006-passthrough.md` widened the glob to any tool's. With that
one change the corpus runs use it with `tools/emend-copy`
(`emend-copy <in> <out> [--replace <name>]`, exit 0 or 2, errors on
stderr as `error: ...`; its temporary is
`.<name>.emend-copy-<pid>.tmp`). `--replace <name>` reaches the tool
through the runner's `--tool-arg <arg>` (repeatable, appended after
the output path; the Milestone 1 part 1 P2 ruling in
`specs/0006-passthrough.md`), and the run row's `tool_args` column and
the record's `Tool arguments:` line say which arguments a run was made
with. The record script writes the records with `--stem`. Raw outputs
stay under the corpus directory's `runs/`.

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

### Planning rulings during P1 (2026-09-17)

Given after the two independent checks of the code commit e14dbee and
before the repair round. Planning reproduced the defect with its own
fixtures against its own build of e14dbee: a 129-byte archive with a
16-byte stub and file-absolute offsets came back 113 bytes, exit 0,
empty stderr; the archive-relative control round-tripped byte for
byte; the zip64 forms behaved the same way (217 to 201 bytes,
silently; the relative form refused as "a zip64 locator without its
record"). Python's zipfile reads all four fixtures.

1. The defect. `Archive::open` derives one value, `prefix_len`, as
   `central_start - central_offset` and uses it both as the offset
   base (what a recorded offset is added to) and as the length of the
   prefix to copy. Those coincide only when a producer records offsets
   relative to the archive proper. With file-absolute offsets the base
   is 0, so the bytes before the first local header are never copied,
   with exit 0 and no flag. The repair separates them: `offset_base =
   central_start - central_offset` for every offset conversion (the
   local offsets read and rewritten, the zip64 record's central
   offset, the locator's record offset); the prefix copy length is the
   smallest local header position in file order, or `central_start`
   for an empty archive. The output keeps the input's convention
   because the prefix is verbatim, so the base is unchanged.
   `prefix_len()` reports the bytes before the first local header,
   which is what the name says.
2. The zip64 locator, same cause, P1 not P2: the design says a zip64
   archive behind a prefix is kept and the P1 list names zip64 offsets
   and a prefix, so refusing the relative form is a spec violation,
   not a limitation. The zip64 end record is found by its signature
   scanning back from the locator and checked to end exactly where the
   locator begins (its size field plus 12); the base is the found
   position minus the locator's recorded offset, and it must agree
   with the base the central directory gives, else `Unsupported`
   naming the disagreement. The rewritten locator offset is the new
   record position minus the base.
3. Tests build their fixtures in BOTH conventions from an explicit
   parameter, never from what the implementation happens to do: the
   plain and the zip64 archive, each with a stub, each absolute and
   relative, each round-tripped byte for byte; the empty archive with
   and without a prefix. `negative.log` reproduces planning's four
   fixtures with the committed `emend-copy` before and after the
   repair (129 to 113 and 217 to 201 bytes before, the relative zip64
   form refused; all four identical after), so the defect and the fix
   are both measured. The comment that stated the assumption as fact
   goes.
4. Check 2's smaller findings: the 64 KiB copy buffer moves to the
   heap (P1; the 32-bit rule); `zip::ZipArchive` is constructed on
   the first `read()`, not in `open()`, so what opens is decided by
   the layer's parser alone, as the module header claims (P1); the
   five tests that discard `read_to_end` with `.ok()` assert its
   result (P1); the index rendered by `NoSuchEntry` and the refusal of
   trailing bytes after the end record go to P3's error catalogue,
   where every error's wording and class is fixed (the layer keeps
   refusing trailing bytes; the message must say so, not `NotZip`).
5. Citations. `gate.log` quoted a self-test rule as this spec's, and a
   test cited "Planning's pin"; both came from planning's messages,
   which are not primaries. The gate-file rule is now in
   `specs/0001-bootstrap.md` ("Rules for every phase") and the two
   tests of item 6 are in this section, so the reruns cite those. A
   planning message is an instruction, never a source: what a log or a
   test cites must be in a spec or a log before the run, and planning
   puts a new rule into a spec in the same doc commit that starts the
   phase.
6. The design note stands: `Option<zip::ZipArchive>`, taken by
   `write_to` and put back, no signature change. Two tests pin it:
   after `write_to`, `read()` still works and a second `write_to`
   gives identical bytes; a `write_to` that fails part-way (a writer
   that errors after N bytes) leaves an `Archive` on which every later
   call returns `Err` or works, never panics, and the test says which.
7. Commit structure. e14dbee stays; the repairs are one new code
   commit, so the defect and its fix are both in history and the diff
   the check answers pin stays reproducible. This ruling and the
   `specs/0001` rule are one doc commit on top of e14dbee, before the
   repair commit. Then one record commit: the five logs rerun at the
   repair commit, each header naming the log it replaces by sha256,
   and both check answers with a header line naming the repair commit
   and saying they checked e14dbee. Six commits in the phase, each for
   a reason named here.
8. The reruns are unchecked by a subagent (two checks were run) and
   verified by planning from primaries. `run.log`'s header is
   corrected: the corpus directory's NAME appears in the printed
   script as the default from `CLAUDE.md`, and its path is written
   `<corpus>`. The environment finding keeps its substance and either
   prints its figures or drops them. The draft report's false
   sentences the checks named are corrected, and the ROADMAP backlog
   item is not claimed closed in full: the runner's `|| die` half
   stays open.

### Planning rulings on P1 (2026-09-17, verification)

Verified from primaries at ff92a11: the six commits and their messages
(ASCII, Co-Authored-By last); all nine record files at the hashes the
report gave; the repair diff cb2b4da against rulings 1, 2 and 4 (the
offset base separated from the prefix length, the zip64 end record
found by its signature and its base required to agree, the heap
buffer, the crate's archive built on the first `read`, the reader
returned plain by `write_to`); the four rerun headers naming the log
each replaces by sha256 and the state it ran at; `scan.log` last,
hashing the four sibling logs, both answers and the record pair at
their committed bytes, 22 files through the bare-shape control; the
runs serialized (gate 01:23Z, test 01:24Z, negative 01:25Z, run
01:25 to 01:30Z, scan 01:32Z). `negative.log` P1 to P4 measure the
defect and the fix with the binary from e14dbee and the repaired one
(129 to 113 and 205 to 189 bytes before, the relative zip64 form
refused; all four identical after). Both check answers' bodies hash
to the answers as first written (89ed8f91..., 8990b660...).

Planning's own runs at ff92a11: the full gate (eight PASS, exit 0,
empty stderr); `scripts/privacy.sh --range 6f4764d..HEAD` (6 commits,
264 blobs, 0 findings); the crate's 33 tests on the host; the four
fixtures planning built for the defect ruling, through a fresh build
of the tool (sha256 c03d6058..., the hash `gate.log` records): all
four byte-identical, 129 and 217 bytes; a recount of the record's
`.txt` by section and label: 27 files per label, every outcome `ok`,
`files_identical` `yes`, `leftover` `no`, `lo` and `lo_in` `ok`, diff
exit 0; 870 entry rows per label, every status `identical`, no
non-empty flags column. Against the T3 P4 baseline (870 flagged per
label, no file identical) this is the result the verbatim-span design
exists for. P1's acceptance holds on both targets.

Residues, accepted as recorded:

- The check answers' committed layout departs from
  `specs/0001-bootstrap.md` item 3: a NOTE block written at commit
  time, no `---` separator, and the input table with its sha256s
  written by the checker inside the answer rather than by the
  implementing session in the header. The bodies are verbatim and
  hash to the originals. From P2 the prescribed layout is used: the
  implementing session's header (checked commit, HEAD, each input
  with its sha256), a `---` line, then the answer.
- The zip64 end record is searched for within 64 KiB before the
  locator (`MAX_ZIP64_SCAN`), so an extensible data sector larger than
  that is refused as "a zip64 locator without its record". A
  documented limit, into P3's error catalogue with its own wording.
- The design line "a reader is wrapped with `io::copy`" (Replacement)
  is not delivered in P1, which consumes no Replacement; it is P2 work
  and P2's list below names it (check 1 gap 12).
- The runner's `|| die` guards still have no negative case; the
  stale-temporary half of that backlog item is closed by `negative.log`
  L4, and `ROADMAP.md` says so.

P1 is DONE.

### P2: replace, remove, add

`Edits` in full, with the `io::copy` wrapper that turns a reader into
a `Replacement`; `--replace <name>` in `emend-copy` replaces the named
entry with its own decompressed bytes. Tests: the descriptor form kept;
zip64 sizes on a replaced entry; a stored original; a deflated
original; remove; add; order preserved around a removed and an added
entry; a replacement error leaves no output file; the ignored 4 GiB
case on both targets; and, because P1's defect was a convention the
corpus never exercises, every edit test that moves an entry runs on a
fixture built in BOTH offset conventions from an explicit parameter,
with the output's offsets checked to keep the input's convention and
the output read back by an independent reader (the harness, or a
generated-fixture parser in the test that shares no code with the
layer). The corpus run replaces `[Content_Types].xml` in every file:
expected `recompressed` (or `identical`) with an empty flags column
for that entry, every other entry `identical` unflagged, `lo` and
`lo_in` `ok`, recorded as `records/m1p1-p2-<utc-date>-<shortsha>.md`
and `.txt`. The check answers use the layout `specs/0001-bootstrap.md`
prescribes.

### Planning rulings before P2 (2026-09-17)

Given on the implementing session's questions before any P2 run.

1. The runner cannot pass `--replace` to the tool. The departure,
   `--tool-arg <arg>` with a `tool_args` run-row column and a
   `Tool arguments:` record line, is ruled in
   `specs/0006-passthrough.md` ("Planning ruling for Milestone 1
   part 1 P2"), with its negative cases R1 to R5. P2's corpus run is
   the runner with `--tool-arg --replace --tool-arg
   '[Content_Types].xml'`; the runner and record-script hunks go into
   P2's code commit and the report names them.
2. The doc commit is one commit of three files, `ROADMAP.md`,
   `specs/0006-passthrough.md` and `specs/0008-zip-layer.md`, made
   before the code commit. (P1 had two because its second ruling came
   after the first was committed.)
3. The ignored 4 GiB case is the layer's own test in `crates/emend`,
   not a rerun of the harness's, which belongs to `tools/xlsx-diff`
   and reruns on the harness's size code. Its shape: an archive over
   4 GiB on disk holding a stored entry of 4 GiB + 1 zero bytes and
   then a small deflated entry, so that the second entry's offset and
   the central directory's need the zip64 forms at real size; built
   as the harness's was (the crate's writer with `large_file(true)`)
   or byte by byte. The small entry is replaced with DIFFERENT bytes
   and the large one is copied verbatim. The test asserts the
   output's zip64 fields, the second entry's offset over 4 GiB and
   the replaced entry's CRC-32 and sizes, then has the output read
   back by the harness BINARY of the same target: the test runs it
   with `std::process::Command` at the path in `EMEND_XLSX_DIFF`,
   read first, before anything is built, and panics when the variable
   is unset or names nothing executable, never skipping silently. The
   crate stays without dev-dependencies and the harness links no
   layer code, which is why the test cannot call the harness as a
   library (ruled 2026-09-17 on the implementing session's question;
   the ordinary tests never read the variable). Expected from the
   harness: exit 1, the large entry `identical`, the small one
   `changed`, `flagged` 0. `test.log` sets the variable to the binary
   the gate built and prints that binary's sha256, the one `gate.log`
   lists; `negative.log` shows the test with the variable unset
   failing before it writes a file, with the run in `test.log` as its
   control. Run explicitly on
   both targets with the test binary under `/usr/bin/time -v`, as
   T1 P4 ran the harness's, with the output, wall time and peak RSS
   in `test.log`, peak RSS under 64 MiB. The files go where
   `std::env::temp_dir()` points, and the run sets `TMPDIR` to a
   directory on a disk-backed file system, named in the log only as
   `<scratch>` with its type from `df --output=fstype`: two archives
   over 4 GiB would take 8 GiB of the 11 GiB RAM-backed tmpfs at
   `/tmp`. The test removes its files.
4. The layer's replacement temporary is a file under
   `std::env::temp_dir()`, so `TMPDIR` is honoured: opened with
   `create_new` and mode 0600 under a name holding the process id and
   a counter, removed on every exit path, error and unwinding
   included. The "a replacement error leaves no output file" test
   also asserts that no temporary of that name form remains. No crate
   is added for this. `emend-copy`'s own temporary beside the output
   is a different file and stays as P1 made it.
5. `[Content_Types].xml` coming back `identical` from the corpus run
   is not a failure and not a finding: a re-deflation can reproduce
   the original bytes, which is why the P2 text says "(or
   identical)". The report counts `recompressed` and `identical` for
   that entry per label. All 27 `identical` on a label WOULD be a
   finding, since it would say the replacement path did not run. The
   expected diff exit for a file with a `recompressed` entry is 1 and
   its `files_identical` is `no` (`specs/0004-xlsx-diff.md`, "Exit
   status"); `identical` throughout gives 0 and `yes`.
6. The record pair is `records/m1p1-p2-<utc-date>-<shortsha>.md` and
   `.txt` through `--stem`, the logs under `records/logs/m1p1-p2/`,
   the check answers in the `specs/0001-bootstrap.md` item 3 layout,
   as the P2 text says.

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
