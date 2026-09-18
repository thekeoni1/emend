# 0008: Milestone 1 part 1, the zip layer

Status: P1 DONE 2026-09-17 (doc acc36fb, 791d0d7, c302ec3; code
e14dbee, cb2b4da; record ff92a11); P2 DONE 2026-09-17 (doc 608e6a4,
9a1cd86, e1c3b07; code ac1bd12; record 9b7ef68); P3 DONE 2026-09-17
(doc 6f3d5c5, 98cac7d, 3d1f8ee, 6cec416; code adb0cc8, 5a8b491,
740648e; records b28d839, 17ac5d4); close-out push DONE 2026-09-17
(doc 7e9846a pushed; record c414445 local). PART 1 DONE. Planning
draft 2026-09-16; approved by the maintainer the same day; entered the
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
  (1980-01-01, so the output is deterministic), no comment; version
  made by equal to version needed, with a zero host byte; internal
  attributes, external attributes and disk start 0; no extra field
  unless a size or the offset exceeds 32 bits, in which case the three
  fixed fields carry the marker and a zip64 extra field holds both
  sizes in the local header and both sizes and the offset in the
  record (the four fields and the clause fixed at P2's verification
  from what P2 wrote); appended after the last entry in file order and
  in the central directory.
- Harness expectation for a replaced entry: status `changed` or
  `recompressed`, and no flag except `descriptor` on an entry that
  used a data descriptor and whose compressed size changed: the
  harness compares the bytes after the data raw (`specs/0004`, P5),
  and the descriptor this design keeps records the new size, so that
  flag is what the design requires, not a finding (measured by P2
  before its corpus run; ruling 7 below). Any other flag on a replaced
  entry, `descriptor` on a replaced entry that used none, and any
  flag on any other entry are findings. Every other entry `identical`
  and unflagged.

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

`tools/size-probe` gains three probes, built in the `size` profile on
i686 musl as the baseline was. `probe-emend-copy` opens a file with
the layer and writes it back with no edits, as first specified;
`probe-emend-edit` takes its edits from the command line (a replace of
a named entry with its own bytes, a removal and an addition), so the
compiler cannot prove any edit path dead and drop it, which the
no-edit probe lets it do (P3 ruling 3); `probe-emend-edit-xml`, from
P3's repair round, is `probe-emend-edit` followed by the streaming
`quick-xml` count `probe-zip-xml` makes over the replaced entry's
bytes, built with both features, so that it is the consumer the
budget describes: one that already carries `zip` and `quick-xml` and
reads entries. For each probe the record gives the file size, the
section total and `.text` from `size -A`, and the deltas of file size
and section total over `probe-zip` (473636) and over `probe-zip-xml`
(539172). The budget is judged on `probe-emend-edit-xml` over
`probe-zip-xml`, with the other two probes beside it and the
differences stated. File sizes on this target move in 4096-byte
steps (the probes are not themselves whole pages: all leave the same
remainder modulo 4096, so every DIFFERENCE is whole pages), so a
change under a page is invisible there and a change of a few bytes
can cost a page; section totals resolve to the byte, which is why
both are recorded.

Maintainer ruling (2026-09-17), on `records/size-m1p1-2026-09-17-
6cec416.md`: the budget for the whole core at the close of Milestone 1
is 256000 section bytes over `probe-zip-xml`, judged on
`probe-emend-edit-xml`, with the file delta reported beside it. Part 1
spent 132732, leaving 123268 for parts 2 to 8. Checkpoint: after part
2's size record, if part 2 alone costs more than about 40 KB of
sections, the maintainer chooses between a higher ceiling (384 KiB
was named) and putting styles, dates and the like behind Cargo
features off by default. Part 2's size log adds two measurements: a
baseline probe that also writes through the `zip` crate's writer, so
the compressor's cost is separated from the layer's own code; and a
note on a stored-replacement mode that would let a consumer accepting
larger files skip the compressor.

Maintainer decision (2026-09-18, given to planning), on part 2's size
record: part 2 alone cost 51136 section bytes, over the checkpoint's
40960, and the maintainer chose the higher ceiling. The budget for the
whole core at the close of Milestone 1 is 393216 section bytes (384
KiB) over `probe-zip-xml`, judged as before on the edit-and-read probe
with the file delta beside it. Parts 1 and 2 together spent 183980,
leaving 209236 for parts 3 to 8. No Cargo feature is added on this
ground; the dependency rule's requirement that anything heavier than
the two crates sits behind a feature off by default stands unchanged,
and the core's own cell reading and writing is never behind a feature.

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
expected `recompressed` (or `identical`), with `descriptor` alone
where the entry used a data descriptor and its compressed size
changed and no flag otherwise (ruling 7 below), every other entry
`identical` unflagged, `lo` and
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
7. The `descriptor` flag on a replaced entry, found by the
   implementing session before the corpus run and measured on one
   workbook with a reader that used neither the layer nor the
   harness: the replaced `[Content_Types].xml` came back
   `recompressed` with `descriptor` as its only flag, every other
   entry `identical` and unflagged, and the 16 bytes after its data
   differed from the input's in the compressed-size field alone (397
   became 399; signature, CRC-32 and uncompressed size equal). That
   is inherent: the harness's `descriptor` covers the raw bytes after
   the data (`specs/0004-xlsx-diff.md`, P5), the design keeps the
   descriptor in the original's form, and a re-deflated stream has a
   new length. The alternatives were refused: clearing bit 3 and
   moving the sizes into the local header changes the general purpose
   flags of an entry whose flags the design says are the original's;
   contriving a stream of the original length is not a real option.
   The corpus holds 27 `[Content_Types].xml` entries, 17 with a data
   descriptor, and 292 descriptor entries in all (the number T3 P4's
   step-zero record flagged when the crate's writer dropped them).
   Ruling: the expectation in "Touched entries" and in P2 is amended
   as above, before the run and with this finding as its reason; the
   evidence that only the size fields change is (a) the layer's
   descriptor-form test on byte-built fixtures, asserting the output
   descriptor equals the input's with only the CRC-32 and size fields
   rewritten, for both signature forms and both size widths, and (b)
   a structure-only inspection of one corpus pair per descriptor form
   present, in `negative.log` or `run.log`, printing the bytes after
   the data of input and output as hex (a signature, a CRC-32 and two
   sizes; no content). The count is the record's `content-types`
   family row (entries, statuses, `flagged`, `descriptor`), which the
   report cites per label. A refinement of the harness's flag to
   compare the descriptor's form and its consistency with the entry's
   own values goes to the xlsx-diff backlog on `ROADMAP.md`.

### Planning rulings on P2 (2026-09-17, verification)

Verified from primaries at 9b7ef68: the five commits and the files
each holds, by path; the messages ASCII with the Co-Authored-By line
last; the phase diff's hash; the nine record files' sizes and sha256s;
`scan.log` written last (13:29Z), hashing every committed sibling at
its committed bytes (all seven compared by planning), listing both
answers with their sha256s, PASS; the three rerun headers naming the
superseded log by sha256 and the reason; the time order the report
gives; the record recount (870 entry rows per label, the replaced
entry `recompressed` in all 27 with `descriptor` alone on 16 and no
flag on 11, the 843 other rows `identical` and unflagged, every
workbook `ok`, diff exit 1, `files_identical` `no`, `lo` and `lo_in`
`ok`, `leftover` `no`, on both labels); the `content-types` family
rows and the `Tool arguments:` line; `gate.log`'s eight PASS and four
binary hashes; `test.log`'s 55 passed and 1 ignored per target, 125
and 2 for the workspace, and the 4 GiB run on both targets (19.61 s
and 4788 kB; 27.48 s and 2324 kB; ext4; files removed);
`negative.log`'s R1 to R5, X1, X2 and E1 to E9c, all as expected;
`run.log`'s ruling 7(b) block; both check answers in the
`specs/0001-bootstrap.md` layout, each body at the size and hash its
header states.

Planning's own runs: `scripts/privacy.sh --range ff92a11..HEAD`, PASS
over five commits; the phase's 9290 added lines scanned for non-ASCII,
U+2014, denylist words, absolute paths and the three bare shapes, all
zero with the controls firing (the four path-like hits are scan
controls inside printed scripts); the full gate at HEAD, eight PASS,
its binaries at the hashes `gate.log` lists; `cargo test -p emend`
with 55 passed and 1 ignored on both targets; and six fixtures of
planning's own through `emend-copy --replace`: a level-0 deflated entry
replaced with itself so that the entry after it moves, in the relative
and the absolute convention, with and without zip64 end records, and a
descriptor entry with and without the signature. Every output kept
its input's offset base and prefix, moved the later entry and kept its
bytes, was read by an independent reader and by python's `zipfile`,
and drew from the harness `recompressed` and `identical` unflagged
for the four, and `recompressed` with `descriptor` alone for the two,
whose trailing bytes differ in the compressed-size field alone.

Rulings on the report's eleven gaps:

1. The synthesised zip64 end records are untested. Carried to P3 as
   its first item, with the fixture below: P2's delivered behaviour
   needs no change, the path is out of reach of the corpus and of
   `emend-copy`, and a repair round would rerun five logs for one
   test. P3 precedes the push.
2. An added entry's version made by, internal and external attributes
   and disk start: written into "Touched entries" above.
3. "A replacement error leaves no output file": satisfied by the
   tool's ordering, which is now the design: `emend-copy` reads every
   replacement source before it creates the output temporary, so
   every replacement failure it can reach precedes the output file
   (E2, E5, E7 and E9 each assert none was left); the layer writes to
   a `Write` and owns only its temporary, whose removal on error is
   tested. No fault hook, as the P1 ruling said.
4. An added entry's zip64 extra field: written into "Touched entries".
5. Both offset conventions reach replace and remove-plus-add only:
   accepted as meeting the sentence's intent, each edit kind on a
   parameterised fixture with the movement asserted; the other moving
   tests pin other properties.
6. `parse_independently` and `read_tail` share the layer's constants:
   accepted; the harness binary read the 4 GiB output and the 27
   corpus outputs, and planning's reader shares nothing.
7. Untested error branches: P3's catalogue enumerates and tests each
   (listed under P3).
8. The unwinding path of the temporary's removal and the `cfg(unix)`
   mode: accepted for this project's targets; P3 adds the panicking
   Replacement test.
9. The ten gate-file names only inside the printed script: from P3,
   `gate.log` prints them in its checks section.
10. Three doc commits against ruling 2's one: accepted; the rulings
    arrived one after another, and a ruling that arrives after the
    phase's doc commit goes in as its own doc commit before the code
    commit, as P1 did.
11. The first scan pass hashed the superseded `negative.log`, found by
    check 2: the final pass covers the committed one (planning compared
    the hash) and the rule is now in `specs/0001-bootstrap.md`.

Residues: the report is a message, not a primary; both checks and
planning traced its figures to the logs. The unsigned-descriptor
ambiguity (a descriptor without a signature whose CRC-32 equals the
signature value is read as signed) is a documented limit for P3's
catalogue.

P2 is DONE.

### P3: size, the catalogue and the budget

First, the tests P2 left, on a fixture of 65535 entries with no zip64
end records: (a) one entry added, so the count crosses the 16-bit
threshold and the writer synthesises the zip64 end record and locator,
asserted byte by byte against the layout the zip specification gives
(built in the test field by field, not by the layer's functions), with
the plain end record's markers; (b) one entry removed, so the count
becomes 65534 and the end record must say so, which pins the
`rewrite_eocd` repair below; (c) a no-edit round trip, byte-identical,
the control. Each output is read back by the independent parser.

The two P2 repairs, in one code commit of their own before P3's
feature commit (P3 rulings 1 and 2): `rewrite_eocd` keeps a marker
only when the input carried zip64 end records, since only then is
0xFFFF or 0xFFFFFFFF a marker rather than a value, and otherwise
writes every field clamped, which is the marker exactly when the value
does not fit; and `refuse_duplicate_names` is no longer quadratic, the
duplicate reported being the first record in directory order whose
name appeared earlier, as before.

Then `probe-emend-copy` and `probe-emend-edit`, measured as "Size"
says, recorded as `records/size-m1p1-<utc-date>-<shortsha>.md`, with
`size.log` printing every figure the record carries, the two baseline
probes re-measured, and the section cost of each duplicate-name
candidate the implementing session built; the error catalogue in the
crate's documentation, with a test for each branch P2 left untested:
`plan`'s method refusal, `compress_replacement`'s method refusal and
its "something other than one entry" (moved into a helper that takes
an opened archive so a test can reach it), `put_value`'s two messages,
`rewrite_local_header`'s "a replacement too large for a local header
with no zip64 extra field", and a Replacement that panics, after which
no layer temporary remains; and the documented limits: the 64 KiB
zip64 scan window (`MAX_ZIP64_SCAN`), a value that would need a zip64
field the input does not carry, and the unsigned-descriptor ambiguity.
`gate.log` prints the ten gate-file names in its checks section. The
report asks the maintainer to re-rule the size budget with the
measured numbers in hand.

### Planning rulings during P3 (2026-09-17)

Given on the implementing session's report before P3's code commit;
doc commit 6f3d5c5 was in, nothing else.

1. A defect in P2's writer, found by test (b) above and measured: on a
   65535-entry archive with no zip64 end records the count field
   holds 0xFFFF as a VALUE, `rewrite_eocd` kept it as if it were a
   marker, and removing one entry produced an end record claiming
   65535 entries over 65534 records, with no zip64 record to resolve
   it, at exit 0 (the test's assertion: entries on this disk, left
   65535, right 65534). Planning confirmed the kept-marker branch in
   the committed source. The P2 verification's ruling 1, "P2's
   delivered behaviour needs no change", was wrong in this corner;
   the test P3 was told to write first is what found it, before the
   push. Ruling: the repair as described under P3, in the repairs
   commit, with tests (b) and (c) written into P3's list so a test
   cites the spec. No tool-level negative case exists, because
   `emend-copy` cannot remove or add; the tests are the pins, and the
   report quotes the failing assertion from the pre-repair run.
2. A quadratic in P1's open: `refuse_duplicate_names` compared each
   name with every earlier one (about 2 x 10^9 comparisons at 65535
   entries; the three tests took 90 s on the host in debug). Ruling:
   the check is made linear-logarithmic, not the tests ignored; the
   implementing session measured `sort_unstable_by` at +5056 section
   bytes (+4304 `.text`), crossing a page, and built one more
   candidate, the stable `sort_by`, in the same scratch way from a
   tree differing only in the sort: 584164 against 584348 section
   bytes at the same file size, so the unstable sort is kept, and
   `size.log` prints both figures at the final commit; a `HashSet`
   candidate is optional. (Planning first wrote the two sorts the
   other way round; corrected before the doc commit on the
   implementing session's reading.) The layer is a zip layer, and an
   open whose time grows with the square of the entry count is a
   defect at any size.
3. Reading the size: file sizes on i686 musl are page-aligned, and
   the no-edit probe lets link-time optimisation drop part of the
   replacement path (the implementing session measured a scratch
   probe with edits from argv at the same file size and +2500 section
   bytes, +2016 `.text`). Ruling: as "Size" now says, two probes, both
   figures, the budget judged on the editing probe. The measured
   figures before the repairs, i686 musl in the `size` profile:
   `probe-zip` 473636 (sections 464320), `probe-zip-xml` 539172
   (sections 533216), both equal to the committed baseline, and the
   no-edit probe at P2's code 588324 (sections 578892), that is
   +49152 in file size over `probe-zip-xml` against the provisional
   250 KiB.
4. The helper for "something other than one entry" is approved: a
   Replacement receives a `&mut dyn Write` and cannot start a second
   file, so the check is testable only on an opened archive.

### Planning rulings on P3 (2026-09-17, first verification)

Verified from primaries at b28d839: the five commits and the files
each holds, by path (the two repairs alone in adb0cc8); the messages;
the phase diff's hash and size; the seven record files at the
reported hashes; `scan.log` last (14:23Z), hashing all six committed
siblings at their committed bytes (each compared by planning),
asserting one answer and one record, PASS; the two rerun headers; the
time order; `gate.log`'s eight PASS on a cold cache and the ten gate
files listed in its checks section; `test.log`'s 71 passed and 1
ignored per target, 141 and 2 for the workspace, and the 4 GiB case
on both targets; `negative.log`'s D1 with the pre-repair binary
failing on the count and the gate's binary passing, X1, T1 and T2,
P1 to P7; the size log's builds, deltas, split and candidates; the
size record equal to the copy the log prints; the check answer in the
prescribed layout. Planning's own runs: `privacy.sh --range
9b7ef68..HEAD` PASS over five commits; the 5671 added lines at zero
non-ASCII, U+2014, denylist words and bare shapes, controls firing;
the full gate at HEAD, eight PASS; `cargo test -p emend` 71 and 1 on
both targets; and the four probes rebuilt by planning to the SAME
bytes (file sizes, section totals, `.text` and sha256s equal to
`size.log`'s), so the size measurement is reproducible.

The diff does what P3 asks. The repairs are as ruled. The probes,
the catalogue, the tests per branch and the limits are delivered. One
check ran, and the implementing session stopped for a ruling rather
than run the second on findings that would change the code; that was
right.

Corrections to the rulings above, from the phase's own logs: ruling
3's "file sizes on i686 musl are page-aligned" was false; the four
probes leave 2596 bytes modulo 4096 and only their differences are
whole pages, which "Size" now says. Ruling 3's +2500 and +2016 for the
edit paths were measured at P2's code; at 5a8b491 they are +2600 and
+1936, with reading an entry through the layer at +62540 section
bytes, measured by a third build that edits but reads nothing.

On the reading cost: `probe-zip-xml` reads an entry through `zip`
too, so the crate's entry-reading path is in the baseline the budget
is measured against, while neither emend probe links `quick-xml`; the
delta "over `probe-zip-xml`" therefore adds emend and subtracts
`quick-xml` and is not the budget's own definition, a consumer that
already carries both crates. Hence the fifth probe below.

Ruling: ONE repair round, then the second check, the final scan and a
second record commit. The code commit (message "M1 part 1 P3 repairs:
...") holds:

1. A test that the replacement temporary is created with mode 0600
   (Unix): a Replacement that finds this process's `emend-replace-`
   file under the temporary directory and reads its mode.
2. A test for "an entry name longer than a header can record".
3. A test that a non-zip file with a stray end-record signature in
   its tail is refused with "bytes after the end-of-central-directory
   record", and the catalogue line for that message reworded to what
   the code checks: a signature was found whose record does not end
   the file, and no record does.
4. The Limits preamble "each is pinned by a test" replaced by the
   truth: which messages no test reaches and why ("a structure larger
   than this target's address space" needs a structure past 32 bits on
   a 32-bit target; "a zip feature the zip crate refused" and "a
   replacement the zip crate could not compress" need the crate to
   fail where no fixture makes it), the rest pinned.
5. The two stale comments in `tools/size-probe/Cargo.toml`.
6. `probe-emend-edit-xml` as "Size" describes, features `emend,xml`.
7. Log scripts (none a gate file): a positive control for every
   `find`-based count (a planted file counted, then removed); the
   gate-file membership read through `counting.sh` forms so a read
   error is never absence; `test.log` printing the sha256 of each test
   executable it runs, and `negative.log` comparing its copies to
   them; the size script's header without the retracted link-time
   sentence; the final scan asserting two answers and the record
   count.

`size.log` is rerun at the repair commit: it asserts the four earlier
probes' sha256s equal those `size.log` printed at 5a8b491 (the
repairs touch no library code, so the bytes are expected identical;
a difference is a finding), measures the fifth, and writes a new
record `records/size-m1p1-<utc-date>-<shortsha>.md` with five rows
and the judged figure; the 5a8b491 record stands as history. The
second check reads the whole phase diff from 9b7ef68, the rerun logs,
check 1's answer and the corrected report.

Accepted without repair: test (c) asserting byte identity alone (an
output identical to a builder-made input needs no second reading);
`compress_replacement` refusing a method before its temporary exists
(declared, and a test depends on it); a rerun header quoting the
superseded run's figures, marked as the superseded run's; the log
header form of every phase so far, the command on line 1 and the
commit on line 2, now written into `specs/0001-bootstrap.md`.

### Planning rulings during the P3 repair round (2026-09-17)

Given on the implementing session's findings after the round's doc
commit 3d1f8ee and code commit 740648e, before its size, negative and
scan logs ran.

1. The byte-identity premise was planning's and it was wrong. A
   doc-comment change moves every code line below it, and the panic
   locations compiled into the binary carry line numbers, in
   `.data.rel.ro`, even with `panic = "abort"` and stripping; the
   build id follows from the content. The implementing session
   measured, at 740648e against 5a8b491: file size, section total and
   `.text` unchanged for both emend probes; `probe-emend-copy` with
   134 differing bytes, 20 of them the `.note.gnu.build-id` payload
   and 110 four-byte slots in `.data.rel.ro` each exactly 16 higher;
   `probe-emend-edit` the same shape with 117 slots; `archive.rs` 16
   net lines longer above its first code line; and a rebuild at
   5a8b491 in a scratch clone reproducing the first `size.log`'s
   hashes. Ruling: `size.log` asserts (a) the four earlier probes'
   file size, section total and `.text` equal the first log's; (b)
   the rebuild at 5a8b491 reproduces the first log's sha256s for the
   two emend probes; (c) every byte that differs between the 5a8b491
   and 740648e binaries lies in `.note.gnu.build-id` or in a four-byte
   slot inside `.data.rel.ro` whose value rose by exactly the line
   shift, computed from git as the net lines added above the first
   code line; (d) the counts printed; (e) `probe-zip` and
   `probe-zip-xml` keep plain sha256 equality. The fifth probe
   measured 670244 file bytes, 665948 of sections and 510883 of
   `.text`, so the judged figure over `probe-zip-xml` is 131072 file
   bytes and 132732 section bytes, which the log prints and the new
   record carries in its five rows.
2. The shape `$(yn [ A ] && [ B ])`, forbidden by
   `specs/0001-bootstrap.md` since T0b P2 because it evaluates A only,
   was found by the implementing session in the first round's
   `size.log` (eleven checks) and `negative.log` (two). Planning
   grepped every committed log: 31 instances across T0b P2, the T0b
   close-out, T6 P1 and Milestone 1 part 1 P1, P2 and P3, caught by
   neither planning nor six independent checks. (Planning's first
   grep found 27; it required a bracket test with no substitution
   inside, and the implementing session's unanchored form found the
   other four. An operator inside an awk program or outside the
   substitution is a false hit of that form; the round's scan uses
   it over each sibling's printed-script section, and a false hit
   still goes into a named function.) Planning audited each second
   condition: 30 are evidenced beside the check, by a printed value
   (the repository's `PRIVATE` visibility, the hook's `0 finding(s)`
   line, the CI gate's three lines, the dependency trees' line
   counts, N7's one FAIL line and N8's one line, the P1 defect's
   `differs` and `identical` outcomes, R3's refusal count, the usage
   text, the descriptor forms, every size figure, the resolved
   commits and the timings) or by the checked thing's later
   use in the same log (binaries and stand-ins that then ran, record
   files that were committed, regexes then measured); one, the first
   `size.log`'s "probe-emend-copy exits 0 and its copy is
   byte-identical", by nothing in its log. Planning ran that probe at
   740648e on a generated archive and the copy is byte-identical, and
   the P1 and P2 records show the no-edit copy byte-identical on 27
   workbooks. No committed verdict changes. Ruling: the round's
   scripts evaluate each compound whole (done); from this round the
   phase's `scan.log`, which reads every sibling, greps each sibling's
   printed script for the shape and reports a hit as a structural
   failure, and `specs/0001-bootstrap.md` says so.
3. This ruling enters as a doc commit after 740648e, since a log may
   cite only a spec. `gate.log` and `test.log` as run at 740648e
   stand: a doc commit changes no code, and the scan covers the range.
   `size.log`, `negative.log`, the second check, the final scan and
   the record commit follow it.

### Planning rulings on P3 (2026-09-17, second verification)

Verified from primaries at 17ac5d4: the four commits of the round and
the files each holds, by path; the messages; the phase diff
9b7ef68..6cec416 at the reported hash and size; the nine record files
at the reported hashes; `scan.log` last (15:10Z), hashing all eight
committed siblings at their committed bytes (each compared by
planning), asserting two answers, two records and four doc commits,
reading the bare-shape files at the commit, finding no compound shape
in any sibling's printed script (planning's own grep agrees, and the
scan's own script no longer matches its regex), PASS; the rerun
headers naming every superseded run; `gate.log` at 740648e with eight
PASS; `test.log` at 740648e with 78 passed and 1 ignored per target,
148 and 2 for the workspace, every test executable hashed; `size.log`
at 6cec416 with rulings (a) to (e) all yes, the controls printing
their counts, the five probes' hashes; `negative.log` with every
copied binary equal to the hash its own log prints (the test binary
to `test.log`'s), D1 at 30.37 s against 0.75 s, P8, P9 and P9c; both
check answers in the prescribed layout. Planning's own runs:
`privacy.sh --range origin/main..HEAD`, PASS over all 26 unpushed
commits; the round's 2199 and the range's 31887 added lines at zero
non-ASCII, U+2014, denylist words, e-mail forms and bare shapes, the
controls firing; the full gate at HEAD, eight PASS; `cargo test -p
emend` 78 and 1 on both targets; the five probes rebuilt to the
record's figures AND to `size.log`'s five sha256s. Check 2's caveat
that the mode test could be flaky was examined: every test that
replaces either holds `REPLACEMENT_LOCK`, goes through `apply`, which
holds it, or is refused in `plan` before a temporary exists, so no
second temporary can appear while the mode test runs.

Rulings on the report's open items and check 2's residues:

1. The Limits preamble kept, now true, with the untested-messages
   paragraph under "Errors": accepted; the placement reads better than
   the one item 4 asked for.
2. `probe-emend-copy.rs`'s module doc still says the budget is measured
   against its delta: a stale comment, fixed in Milestone 1 part 2's
   first code commit, not by a further round. The same for the
   candidates comment in the size script.
3. `ROADMAP.md`'s "four tests" was planning's count; corrected to
   seven new and four tightened.
4. The `find` patterns: planning read the name forms in P2's diff,
   `emend-replace-<pid>-<n>.tmp` for the layer's temporary and
   `emend-4gib-in-<pid>.zip` and `-out-` for the 4 GiB fixtures, both
   in the temporary directory itself; the patterns match them.
5. The +16 slots being panic-location line numbers is an inference;
   the measurement is that every differing byte outside the build id
   is a four-byte `.data.rel.ro` slot risen by exactly the line shift,
   with a wrong shift leaving bytes unexplained. Accepted as stated.
6. The size script's candidate-form line printed but not asserted, and
   the compound checks as named functions rather than one function
   each: accepted; the candidates' figures differ from one another and
   from the committed build's, which is the guard.
7. Two messages asserted by calling `rewrite_central_record` directly
   rather than through an archive: accepted, the producing code runs.

Residues carried: items 2 and 6 above; the size budget itself, which
the maintainer re-rules on the record at 6cec416 (probe-emend-edit-xml
over probe-zip-xml: 131072 file bytes, 132732 section bytes, against
the provisional 256000; reading an entry through the layer is 62540
section bytes of the editing probe's cost, most of it the crate's
entry-reading path, which the baseline also links).

P3 is DONE. Part 1's close-out push follows, as the section below
says, once the maintainer types the authorisation in the implementing
session.

### Close-out push (after P3)

As `specs/0005-corpus.md`, "Close-out push (after P7)".

Planning ruling (2026-09-17): the close-out push is verified and DONE.
- The maintainer typed the authorisation in the implementing session
  after the pre-push checks had passed and the session had stopped;
  `push.log` quotes it in a line marked as written by the caller.
- Doc commit 7e9846a holds exactly planning's two doc files, unedited
  (72 insertions, 4 deletions). Record commit c414445 touches only
  `records/logs/m1p1-close/`; it is local and goes with the next push.
- `push.log` recorded the base before the push (d1868a7, read from git
  and from `ls-remote`, 27 commits), the eight PASS lines, the hook
  equal to the installer's, `core.hooksPath` empty, the denylist
  variable unset, and the hook's scan of d1868a7..7e9846a: 27 commits,
  339 blobs, 0 findings. Planning ran `scripts/privacy.sh` over the
  same range itself: 27 commits, 339 blobs, 0 findings.
- From GitHub itself, by planning: `thekeoni1/emend` is PRIVATE, one
  branch `main` at 7e9846a equal to local main, no tags, every author
  and committer of the 27 pushed commits the noreply address, the
  Actions settings unchanged (selected actions, GitHub-owned only, no
  patterns). Run 35240792493 on 7e9846a concluded `success` on
  `ubuntu-24.04`, one job, every step success or skipped.
- The three logs hold one `no`, the API key check, as intended;
  `scan.log` was written last, covers both siblings and the 60 files
  and 27 messages of the range, and finds no compound shape in either
  printed script; planning's own scans of the three logs are clean.
- Noted: the close-out scripts are T2's, with the failed-command reads
  hardened and an `ls-remote` check on the base added.

Milestone 1 part 1 is DONE. Residues for part 2's first code commit:
the stale doc sentence in `probe-emend-copy.rs` and the candidates
comment in the size script. The size budget awaits the maintainer's
ruling on `records/size-m1p1-2026-09-17-6cec416.md`.

## Out of scope

- Parsing any XML; parts 2 to 8 of Milestone 1.
- Methods other than stored and deflate for touched entries;
  encryption; multi-disk archives; repairing a broken archive.
- Changing an untouched entry's bytes for any reason.
