# ROADMAP

Every task is broken into phases. Each phase is committed, gated, checked
by an independent subagent, reported, and verified by planning from
primaries before the next phase starts (CLAUDE.md). Design detail lives
in `specs/`; this file tracks the parts and their status.

Status values:

- `DONE`: verified by planning from primaries, with the record commit named.
- `ACTIVE`: the implementing session is on it.
- `READY`: phase spec written.
- `PLANNED`: outlined here and in `specs/0000-step-zero-plan.md`; the
  phase spec is written when the previous task verifies.
- `PROPOSED`: an outline the maintainer approved as a proposal; each part
  is scoped into phases before work on it starts.
- `UNSCOPED`: an idea. Scoping it needs the maintainer's approval.

Planning updates this file when it verifies a phase. New milestones or
ideas are added only after they are scoped and the maintainer approves.

## Milestone 0: step zero (foundations)

Architecture decided 2026-09-14: greenfield hybrid
(`records/decision-architecture.md`). Goal: the fidelity harness, the
private corpus and a measured passthrough baseline. Plan:
`specs/0000-step-zero-plan.md`.

### T0: repo bootstrap (`specs/0001-bootstrap.md`), DONE 2026-09-15

| Phase | Part | Status |
|-------|------|--------|
| P1 | Commit CLAUDE.md, ROADMAP.md and specs as they stand; ASCII and U+2014 scan | DONE (rewritten in P1R; now af5a2c6 to d3754dc) |
| P1R | Rewrite the P1 commit messages without the Claude-Session line, trees unchanged; commit the maintainer and planning doc updates | DONE (doc commit d495f0a; records through 88d5644) |
| P2 | Cargo workspace, toolchain pin, licence files, xlsx-diff skeleton, .gitignore; build and test on host and i686 by hand | DONE (1a985a1 to 9daa4c6) |
| P3 | `scripts/check.sh` steps host, i686, fmt, clippy; case-insensitive `.gitignore`; negative tests | DONE (ced7f91 to 53b99d8) |
| P4 | `check.sh` steps native (no C, `-sys` allowlist) and licences; colour off; negative tests | DONE (977635b to a00d6fd) |
| P5 | `check.sh` step hygiene (U+2014, office files in any case, corpus paths, API key); `--all-features` and `_sys` fixes to native and licences; toolchain record; negative tests | DONE (d8c1ee1 to 9b7aef1) |
| P5b | Hygiene hardening: read the index, fail control bytes in paths, case-insensitive corpus-path match, API key check before step 1, in-script controls; negative tests | DONE (873851f to 65cdeae) |
| P6 | Privacy gate: repo-local identity, `scripts/privacy.sh` with shape-based patterns as `check.sh` step privacy, pre-push hook, private denylist outside the repo; scans ignore git attributes and fail on read errors; corpus paths never printed; negative tests | DONE (10a42d6 to 28a44aa) |
| P6b | Privacy gate hardening: corpus and office paths checked across history, denylist override refused, per-pattern self-test, email and path patterns, count-only office line; negative tests | DONE (690c0f5 to cf5240b) |
| P7 | First push to the private GitHub repo, after the maintainer's GitHub email settings and authorisation in the implementing session | DONE (pushed af4b355; records 9c63579 to 302b1b2 local) |
| P6c | Privacy gate remaining gaps: office names with suffixes, more spreadsheet types, archive signatures, full commit headers, emails in paths, more Windows path forms; no push after P7 until done | DONE (a371cff to 8e7e665) |
| Close | Push the local P7 and P6c record commits, using P7's pre-push checks | DONE (pushed e0a5175; records 00c64bb and 41e2919 local) |
| P6d | Narrow the three path patterns so a bare shape named in prose passes the gate and a real path still fails (the scan rule in `specs/0001` still expects no bare shape in a committed file); self-test and negative cases (`specs/0002-gate-corpus-additions.md`, added 2026-09-15) | DONE (4641ae3 to 8f45639) |

### T0b: continuous integration (`specs/0003-ci.md`), DONE 2026-09-15

Approved by the maintainer on 2026-09-15. Ran after T2 P0 and before T1.

| Phase | Part | Status |
|-------|------|--------|
| P1 | `check.sh --ci` (steps 1 to 7, privacy skipped); workflow with a read-only token, a SHA-pinned checkout and the pinned toolchain; a green run recorded for a pushed commit | DONE (71c44b8 to ec7bf8c; pushed 401a2f7; run 34977885012 success) |
| P2 | Prove CI fails: a throwaway branch with a formatting error, the red run recorded, the branch deleted | DONE (41a99a1 to c053374; run 34991440664 failure on the deleted branch; main unchanged at 401a2f7) |
| Close | Push the local T0b P1 and P2 record commits and the P2 rulings, using the T0 close-out steps; record the CI run on the pushed commit | DONE (pushed 199b468; run 34993434019 success; record 62c4e80 local) |

### T1: xlsx-diff, the fidelity harness (`specs/0004-xlsx-diff.md`), DONE 2026-09-15

| Phase | Part | Status |
|-------|------|--------|
| P1 | `zip` 8.6.0 pinned with the pure-Rust deflate backend; lib and bin split; `entry` rows with indexes, methods, sizes and CRCs; statuses `identical` (raw bytes equal), `dropped`, `added` and the provisional `differs`; name escaping; tests on generated zips | DONE (8356f9a to 6ccae09) |
| P2 | Streaming content comparison through fixed buffers: `recompressed` vs `changed`, first and last differing offset, differing-byte count, all `u64`; memory bound shown; the short-stream error tested | DONE (a3166a5 to 24f4ef0) |
| P3 | All `archive` rows (counts, order, comment, offsets, totals, `files_identical`), the six flags, exit status 0/1/2 as specified; column 14 as the first differing offset for different lengths | DONE (f0adb45 to 827766a) |
| P4 | No-leak sentinel test on stdout and stderr; the `local` flag; a prefix-only `files_identical` test; zip64 entry over 4 GiB uncompressed compared by streaming on i686 (ignored by default, run explicitly and logged) | DONE (564af3b to b5180be) |
| Close | Push T1 P1 to P4 with their records, after P4 (maintainer decision); record the CI run | DONE (pushed 83c9852; run 35017518231 success; record 343e91f local) |
| P5 | Flag completeness: `local` narrowed to local-only differences, `attrs` with internal attributes and disk number, `descriptor` with the bytes after the data; records go with the next push | DONE (b58c086 to 4f6f347) |
| P6 | Closing the exit-0 guarantee: `attrs` covers raw central size, CRC and method bytes that decode equal (zip64 placeholders); `order_preserved` covers the file order; the data-overrun case tested; the last T1 phase | DONE (8a898b3 to 8c979bc; records local until the next push) |

### T2: corpus (`specs/0005-corpus.md`), P7 DONE 2026-09-16; P5 and P6 open (maintainer)

Private, in `/home/dev/emend-testing`. Nothing from it is committed. A
separate CORPUS session generates and downloads files (P2 to P4,
maintainer decision 2026-09-15); the maintainer supplies P5 and P6; the
implementing session does P1 and P7.

| Phase | Part | Status |
|-------|------|--------|
| P0 | Privacy gate corpus-era additions: Zone.Identifier files, OLE2 and other archive signatures, both forms of patterns 10 and 11 in the self-test, checked pipeline exits (`specs/0002-gate-corpus-additions.md`); runs before T1 and before any corpus file | DONE (9324e64 to 0870c36) |
| P1 | Manifest format (in the spec) and two python3 scripts in the repo, `corpus-add.py` (the corpus session's helper, imported through a recorded step) and `corpus-coverage.py`, that print only ids, producers, sources, tags and counts, with logged negative cases | DONE (45dd9e3 to 869796b) |
| P2 | Corpus session: LibreOffice container pinned by image digest, version recorded, headless conversion smoke test on a generated fictional file | DONE (verified 2026-09-15; image `localhost/emend-lo:25.2.3-2-deb13u6`) |
| P3 | Corpus session: LibreOffice exports with fictional content, generated as flat ODS by a script and converted headless, covering the tags LibreOffice can produce | DONE (17 files, verified 2026-09-15) |
| P4 | Corpus session: Microsoft's public Excel templates added, with sources noted in the private manifest | DONE (10 files, verified 2026-09-15) |
| P5 | Maintainer: a few Google Sheets exports, so the single-producer tags gain a second producer | PLANNED |
| P6 | Maintainer: one .xlsm with a macro (`macros`, the only uncovered tag), a desktop Excel file with threaded comments and one with a chartex chart, plus any own workbooks | PLANNED |
| P7 | P1 follow-ups (docstring, temporary-file order in `corpus-add.py`, UTF-8 cases), then the coverage record over the real corpus and the LibreOffice pins record | DONE (58063fb to 7b9350a; `records/corpus-coverage-2026-09-16.md`: 27 files, 22 of 23 tags, 7 single-producer) |
| Close | Push everything since 29b0bad (T3 P3, T0 P6d, T3 P4, T2 P1, T2 P7 and their records) with the pending doc updates; record the CI run (`specs/0005-corpus.md`, "Close-out push (after P7)") | DONE (pushed d1868a7, 48 commits; run 35106637754 success; record 731d6ff local) |

### T3: passthrough baseline (`specs/0006-passthrough.md`), DONE 2026-09-16

| Phase | Part | Status |
|-------|------|--------|
| P1 | The `passthrough` copy tool (raw copy of every entry in order, archive comment, fault injection behind a guard), the runner script with subprocess isolation (timeout, panic, abort, memory) and the record script, all on the host with generated archives; the first finding table on what the zip writer preserves | DONE (d63ad31 to 33c0198; finding: the zip crate's raw copy is not byte-preserving) |
| P2 | i686 musl runs; `tools/size-probe` and the `size` profile; stripped sizes of the `zip` and `zip` plus `quick-xml` consumers recorded (size baseline); the `zip` feature and licence tree cited; runner fixes (leftover temporary files, guarded commands) | DONE (8bbda4d to b1e7982; baseline 473636 and 539172 bytes, `records/size-baseline-2026-09-15-6a7533e.md`) |
| Close | Push the T1 P5 and P6, T3 P1 and T3 P2 records with the pending doc updates, after P2 (maintainer decision); record the CI run | DONE (pushed 29b0bad; run 35032688348 success; record 24d4a16 local) |
| P3 | LibreOffice open check with the corpus session's image: definition pinned, `lo`, `lo_in` and the `error` value, the LibreOffice record, negative tests on corrupted files, two measured findings | DONE (fd5dcf9 to 2638d21) |
| P4 | Pre-run work (P3 carry-overs, `--lo-timeout`, tolerant tag matching, a chart-sheet measurement), then the first full recorded run over the corpus on both targets: `records/step0-<date>-<sha>.md` and `.txt`, aggregated by part family | DONE (39defb7 to 2222c66; every entry identical on both targets with the writer's flags catalogued, every workbook opening in LibreOffice; `records/step0-2026-09-16-39defb7.md` and `.txt`) |

### T4: moved into Milestone 1

The names-only XML comparison is now Milestone 1 part 7 (maintainer
ruling 12, 2026-09-14).

### T5: architecture decision

| Phase | Part | Status |
|-------|------|--------|
| P1 | `records/decision-architecture.md`: the maintainer chose greenfield on 2026-09-14, before any measurement | DONE (committed in ced7f91, byte-identical to planning's draft) |

### T6: gate self-test (`specs/0007-gate-self-test.md`)

Approved for scoping by the maintainer 2026-09-15; scoped 2026-09-16.
One case per detection class from the T0, T2 P0 and T0 P6d negatives,
run against scratch clones; run whenever a gate file changes.

| Phase | Part | Status |
|-------|------|--------|
| P1 | `check.sh --only <step>`; `scripts/counting.sh` (the T2 P7 counting library, committed); `scripts/gate-self-test.sh` with the hygiene, privacy, hook and fmt cases and their controls; the first run committed as `self-test.log` | DONE 2026-09-16 (code 0a337fb and f7d9dc7; records 4d2ece2 and 6f4764d) |
| P2 | `--full`: the build-level cases (host, i686, licences, native), timed; the rule that a phase changing a gate file runs the self-test and commits its output; carried from P1 check 2: gap 8 (H3 and K3 sourcing), finding 17 (V12 arithmetic), a coverage regression check, a parallel mode with the wall time re-measured, the `--range <base>..HEAD` note | PLANNED (after Milestone 1 part 1) |

## Standing items (maintainer rulings 2026-09-14)

- Architecture: greenfield hybrid. umya-spreadsheet and rust_xlsxwriter
  are reference material only.
- Excel: no Excel checks during step zero. From Milestone 1 on, once per
  milestone, the maintainer opens outputs listed by sha256 in desktop
  Excel, and pass or fail per hash is recorded. Excel for the web is not
  a substitute.
- Size budget: RULED by the maintainer 2026-09-17 at 256000 section
  bytes over `probe-zip-xml` for the whole core at the close of
  Milestone 1, judged on `probe-emend-edit-xml`, i686 musl, the `size`
  profile (`specs/0008-zip-layer.md`, "Size"). Part 1 spent 132732;
  123268 remain. Checkpoint after part 2's size record: more than about
  40 KB for part 2 alone means a higher ceiling or features off by
  default. Milestone 2: RULED by the maintainer 2026-09-25 at 458752
  section bytes over `probe-zip-xml` (448 KiB), judged at the close
  of part 3 on the last core probe, with a checkpoint after part 2's
  size record: above 25000 for part 2 alone means re-ruling before
  part 3's spec (`specs/0017`, "The size ceiling"). The Milestone 1
  ceiling of 393216 stands as Milestone 1's record.
- Licence: MIT OR Apache-2.0, copyright holder "emend contributors".
- GitHub: `thekeoni1/emend`, private. Every push passes the privacy gate
  (T0 P6) and needs planning's word plus the maintainer's authorisation
  in the implementing session. Commit messages carry no session links.

## Milestone 1: greenfield core (parts 1 to 8 DONE and PUSHED; PROVISIONALLY ACCEPTED 2026-09-24 on Excel check 1, confirmed by Excel check 2)

Outline approved as a proposal by the maintainer on 2026-09-14. Each
part is scoped into phases, with its own spec, before work on it starts.
The core depends on `zip` and `quick-xml` only. The step-zero baseline
is `records/step0-2026-09-16-39defb7.md` and `.txt` (T3 P4): a run of
the new zip layer over the corpus must show every untouched entry
`identical` with no flag, where the zip crate's raw copy flagged all
870 entries.

1. Zip layer: read the central directory with `u64` sizes and offsets,
   copy untouched entries as their original compressed bytes in the
   original order, and rewrite touched entries by streaming. T3 P1
   showed that the zip crate's raw copy is not byte-preserving (it
   drops extra fields, zip64 fields and prefixes, rewrites data
   descriptors and attributes, and collapses duplicate names), so this
   layer writes local headers and central records itself from the
   input's header bytes, and uses the crate for reading at most.
2. Relationships and content types, parsed only when an edit needs them.
3. Reading cells: values, cached formula results, formulas and shared
   strings.
4. Editing cells: a streaming rewrite of only the affected worksheet.
5. Styles: add new ones without renumbering existing ones.
6. Calculation chain: remove it when formulas change, so Excel rebuilds
   it.
7. Names-only XML comparison (formerly T4), added to the harness.
8. LibreOffice open checks for every edit test; the per-milestone Excel
   check.

### Part 1: the zip layer (`specs/0008-zip-layer.md`)

Approved by the maintainer 2026-09-16. `crates/emend` with the
`archive` module: open and index with `u64` offsets, verbatim spans
for untouched entries, streamed replacements through the `zip` crate,
the corpus run as acceptance, the size delta measured. No push during
the part; the close-out push follows P3.

| Phase | Part | Status |
|-------|------|--------|
| P1 | `Archive`, `Entry`, `Edits`, `Error`; open, index, verbatim write with empty edits; byte-built unit tests on both targets; `tools/emend-copy`; the corpus run on both targets recorded as `records/m1p1-p1-<date>-<sha>.md` and `.txt`, every entry `identical` and unflagged, `files_identical` `yes` | DONE 2026-09-17 (doc acc36fb, 791d0d7, c302ec3; code e14dbee, cb2b4da; record ff92a11) |
| P2 | replace, remove, add; the `io::copy` Replacement wrapper; `--replace <name>`; the runner's `--tool-arg` and the record's `Tool arguments:` line; edit tests in both offset conventions; the ignored 4 GiB case; the corpus run replacing `[Content_Types].xml` | DONE 2026-09-17 (doc 608e6a4, 9a1cd86, e1c3b07; code ac1bd12; record 9b7ef68) |
| P3 | the three 65535-entry tests and the two P2 repairs they forced (the end record's count kept as a marker; the quadratic duplicate-name check); `probe-emend-copy` and `probe-emend-edit` with the size record giving file and section deltas; the error catalogue with a test per untested branch and the documented limits; the maintainer re-rules the budget | DONE 2026-09-17 (doc 6f3d5c5, 98cac7d, 3d1f8ee, 6cec416; code adb0cc8, 5a8b491, 740648e; records b28d839, 17ac5d4; the repair round added `probe-emend-edit-xml`, seven tests and four tightened, the catalogue's truth about untested messages, and log-script controls) |
| Close | close-out push after P3, as T2's | DONE 2026-09-17 (doc 7e9846a pushed, origin/main at 7e9846a; record c414445 with the next push) |

### Part 2: the package layer (`specs/0009-package-layer.md`)

Approved by the maintainer 2026-09-17. `quick-xml` joins the core; the
`xml` module gives every token with the input bytes it covers, so a
rewrite copies what it does not change; the `package` module reads
content types, relationships and the workbook's sheet map and edits
content types and relationships. Each phase has a corpus run; the
size record after P3 is the maintainer's checkpoint (more than about
40 KB of sections for part 2 alone means a higher ceiling or features
off by default). Part 6 will apply P3's removal recipe under the
formula-change rule. No push during the part; the close-out push
follows P3 and carries the local record c414445.

| Phase | Part | Status |
|-------|------|--------|
| P1 | `quick-xml` 0.42.0 pinned; `xml::Stream` with raw tokens and the byte order mark kept; `emend-copy --xml-null-rewrite`; the two part 1 residues; the corpus run: every `.xml` and `.rels` entry `recompressed` or `identical`, never `changed` | DONE 2026-09-18 (doc 0e6c60d, 9b87536, b97d01c, 248fdd8; code a9f2cad, 4725795; record d6fa91d; 781 XML entries recompressed or identical on both targets, 89 others identical) |
| P2 | the test builder moved to `testing.rs`; `package`: content types, relationships with target resolution, the workbook's sheets by relationship, the prefix stack; `emend-copy --survey`; the corpus run with the survey aggregated: nothing untyped, unresolved or case-only | DONE 2026-09-18 (doc bbde39e to c8b8fec; code 82d4e77, 5fc1b3d, 1334830; record eacffa2) |
| P3 | the edits (relationships, overrides, parts) applied through a temporary-file rewrite; `--drop-calc-chain`; `probe-zip-xml-write` and `probe-emend-package-xml`; the size record with the checkpoint figure and the stored-replacement note; the catalogue and limits; the corpus run: nine workbooks change in exactly three entries, eighteen are byte-identical | DONE 2026-09-18 (doc 0a884ee to d6fcc5f; code 11de26e, 5fdf65a, 5dba448, e04db06, 14c2e14; record 711fc8b; part 2 cost 51136 section bytes, above the 40960 checkpoint: maintainer decision open) |
| Close | close-out push after P3, as part 1's, with record c414445 | DONE 2026-09-18 (pushed f4f48e8, 30 commits, 376 blobs, 0 findings; CI run 35341839926 success; doc 9bd9102 and record 7702f6e local, the scan a recorded FAIL on a bare shape in two comment lines of a committed P3 log, ruled in `specs/0009` "Planning rulings on the close-out") |

### Part 3: reading cells (`specs/0010-reading-cells.md`)

Approved by the maintainer 2026-09-18 and sent to implementation. The
`cells` module streams a worksheet's cells (reference, style, kind,
value as written, formula) over `xml::Stream`; shared strings are
parsed into a table that resolves a cell's index; typed views parse a
number, a boolean and an error on request; nothing is written, and
every harness output is byte-identical. Acceptance: the reader's
counts over the corpus equal planning's independent counts (95 sheets,
2094601 cells, 262879 formulas, 110582 shared strings, every index
resolved). The size record after P3 is judged against the ceiling the
part 2 checkpoint ruling set (2026-09-18: 393216 section bytes over
`probe-zip-xml`, `specs/0008` "Size"). No push during the part; the
close-out push follows P3 and carries the local commits 9bd9102 and
7702f6e.

| Phase | Part | Status |
|-------|------|--------|
| P1 | the items part 2 carried (the `xml.rs` count and the set-equality test, the `Default`-spelling test, `--drop-calc-chain` reading content types first, the rewrapped comments, the run and self-test script forms); `SharedStrings`, `Package::shared_strings`, `--shared-strings`; the corpus run: 110582 items, per file equal to an independent count | DONE 2026-09-18 (d1ebf1c, local; verified by planning from primaries; planning's finding P1-F1, carriage returns kept in text, ruled for P2's first commit with CDATA, nested elements and a missing target; `specs/0010` "Planning verification and rulings on P1") |
| P2 | first commit: the P1 rulings (line ends, CDATA, nested elements, a missing target, the `--shared-strings` test, the rename); then `cells::{Cell, CellRef, Cells, Formula, FormulaKind, Kind}`, `Package::cells`, `resolve`, `--cells`; the corpus run with every count asserted by name, `bytes` 1497091 | DONE 2026-09-18 (9339dca, local; verified by planning from primaries and its own expat count; finding P2-F1 resolved against planning's survey, `values` 2089191 and `empty` 5410; `specs/0010` "Planning verification and rulings on P2") |
| P3 | first commit: the P2 rulings (decimal `u32` digits only, row 0 refused; the strict namespace named by bytes); then `number`, `boolean`, `error`; the catalogue and limits; `probe-emend-cells-xml` with the float-parser variant; the size record against the checkpoint ruling; the negative log through the binary | DONE 2026-09-18 (13fe4a4, records 8e9d468, local; verified by planning from primaries, its own gate, tests and corpus run; budget 247348 section bytes, 145868 under the 393216 ceiling; findings P3-F1 closed and P3-F2 ruled; `specs/0010` "Planning verification and rulings on P3") |
| Close | close-out push after P3, as part 2's, carrying 9bd9102 and 7702f6e | DONE 2026-09-22 (pushed c824d87, 23 commits, 406 blobs, 0 findings; CI run 35675112635 success; doc 8e365a8 and CLAUDE.md 95b93f4 pushed; record split by the maintainer's choice, c824d87 pushed and b58e894 local; ruled in `specs/0010` "Planning verification and rulings on the close-out") |

### Part 4: editing cells (`specs/0011-editing-cells.md`)

Approved by the maintainer 2026-09-22 and sent to implementation,
the three decisions taken as planning recommended (inline strings;
formula cells overwritable with the chain dropped; `<dimension>` left
alone). `Value` (number, text,
boolean); `Package::set_cell`, `clear_cell` and `drop_calc_chain`
(the part 2 recipe moved into the crate); `write_to` rewrites an
edited worksheet by streaming, copying every untouched token raw,
replacing edited cells and inserting new cells and rows in order;
edits to shared and array formula cells refused; a removed formula
drops the calculation chain. Acceptance: over the corpus a four-cell
sample edit leaves every other entry byte-identical and the edited
part identical outside the four cells, by an independent parser in
the run script; every output opens in LibreOffice; the values read
back. No push during the part; the close-out push follows P3 and
carries the local commit b58e894.

| Phase | Part | Status |
|-------|------|--------|
| P1 | `drop_calc_chain` in the crate with `--drop-calc-chain` calling it; `Value`, `set_cell`, `clear_cell`, the worksheet rewrite in `write_to`, the seven messages; the corpus run `--drop-calc-chain` against part 2 P3's expectations | DONE 2026-09-22 (d7a7a7f, records 57d2334, local; three code commits by ruling 1 on finding P1-F1, the eighth message and data tables refused; verified by planning from primaries, its own gate, tests and record comparison with part 2 P3; `specs/0011` "Planning verification and rulings on P1") |
| P2 | `--edit-sample`; the corpus run with the run log's `expat` cell comparison | DONE 2026-09-22 (595cf8b, records ed89a57, local; finding P2-F1, planning's clause, ruled during P2; verified by planning from primaries, its own gate, tests and its own `expat` comparison agreeing on all 27; `specs/0011` "Planning verification and rulings on P2") |
| P3 | `--edit-formula-sample`; the catalogue and Limits; `probe-emend-cells-write-xml` and the size record against the ceiling; the negative log through the binary | DONE 2026-09-22 (4b31e2a, records df83dab, local; verified by planning from primaries, its own gate, tests, size rebuild and `expat` comparison; budget 278064 section bytes, 115152 under the 393216 ceiling, own cost 4100 and whole growth 30716; `specs/0011` "Planning verification and rulings on P3") |
| Close | close-out push after P3, as part 3's, carrying b58e894 | DONE 2026-09-22 (pushed 35e38a5, 17 commits, 423 blobs, 0 findings; CI run 35753087397 success; one record commit 7cc4f13 local, with the next push; `specs/0011` "Planning verification and rulings on the close-out") |

### Part 5: styles (`specs/0012-styles.md`)

Approved by the maintainer 2026-09-22 and sent to implementation,
the three decisions taken as planning recommended (number formats
and derived cell formats only, with fonts, fills and borders later;
dates as the Cargo feature `dates`, off by default, no dependency;
style indices validated against the styles part at write).
`Package::styles` reads the number formats and cell formats;
`add_cell_style` appends an `xf` copied from an existing one with
its number format changed, and a custom format with an id no
existing one uses, so nothing is renumbered; `set_cell_style`
composes with part 4's value edits; `Workbook::date1904`;
`dates::{DateTime, Epoch}` to and from serial numbers in both
epochs. Acceptance: over the corpus a sample that
adds two cell formats and one custom format, styles one cell and
writes one dated cell leaves every other entry byte-identical, the
styles part identical outside the two containers it grows, and the
worksheet identical outside the two cells, by an independent parser
in the run script; every output opens in LibreOffice; the styles
and values read back. No push during the part; the close-out push
follows P3 and carries the local commit 7cc4f13.

| Phase | Part | Status |
|-------|------|--------|
| P1 | `Workbook::date1904`; the `styles` module read (`Styles`, `NumFmt`, `Xf`, `NumberFormat`); `--styles`; `add_cell_style`, `set_cell_style`, the styles rewrite and the worksheet start-tag rewrite in `write_to`, the ten messages; the corpus run `--styles` against planning's survey | DONE 2026-09-22 (abf2618, records c3d1adb, local; five P1 rulings before code; verified by planning from primaries, its own gate, tests, privacy scan and per-workbook `ElementTree` count agreeing on all 27 on both targets; `specs/0012` "Planning verification and rulings on P1") |
| P2 | `--style-sample`; the corpus run with the run log's independent XML reader and byte-span check | DONE 2026-09-22 (7c96575, records 747a96e, local; eight P2 rulings after check 1, run and negative logs rerun; verified by planning from primaries, its own gate, tests, privacy scan and its own comparison over the run directory agreeing on all 27 on both targets; `specs/0012` "Planning verification and rulings on P2") |
| P3 | the `dates` feature; `--date-sample`; the catalogue and Limits; `probe-emend-styles-write-xml` and `probe-emend-dates-write-xml` with the size record against the ceiling; the negative log through the binary | DONE 2026-09-22 (f565868, records 39baf23, local; seven P3 rulings before code; verified by planning from primaries, its own gate, tests with the feature off and on, privacy scan, its own rebuild of four probes reproducing the size log's hashes, and its own comparison over the run directory agreeing on all 27 on both targets; budget figure 304412 of 393216; `specs/0012` "Planning verification and rulings on P3") |
| Close | close-out push after P3, as part 4's, carrying 7cc4f13 | DONE 2026-09-23 (pushed 35e38a5..77d2b50, CI run 35884307712 success; record b0c21ef local, goes with the next push) |

### Part 6: formulas and the calculation chain (`specs/0013-formulas.md`)

Approved by the maintainer 2026-09-23 and sent to implementation,
the four decisions taken as planning recommended (no cached value
beside a written formula; `fullCalcOnLoad="1"` set by the
formula-change rule and offered as `recalculate_on_open`; a formula
text starting with `=` refused; acceptance on the LibreOffice
computation check, the Excel check pending the desktop machine).
`Value::Formula` writes `<f>` with no `<v>`; a write that adds,
replaces or removes a normal formula drops the calculation chain
(part 4's recipe) and asks the workbook for a full calculation on
load, one attribute rewritten on `calcPr` with every other byte of
the workbook part copied raw; `Workbook::full_calc_on_load` reads
the flag back. Acceptance: over the corpus a sample that writes two
formulas and replaces one leaves every other entry byte-identical,
the worksheet identical outside the three cells and the workbook
part outside its `calcPr` tag, the chain gone on the 9 workbooks
that have one, by an independent parser in the run script; every
output opens in LibreOffice, which computes the written formulas to
the expected values in a second conversion; the formulas and the
flag read back. No push during the part; the close-out push follows
P3 and carries the local commit b0c21ef.

| Phase | Part | Status |
|-------|------|--------|
| P1 | `Value::Formula` and its refusals; the formula-change rule for a written formula; `Package::recalculate_on_open`, the `calcPr` rewrite, `Workbook::full_calc_on_load`; `--calc`; the corpus run `--calc` against planning's survey (`chain` 9, `full_calc` 0) | DONE 2026-09-23 (da8794b, records 6d85828, local; six P1 rulings before code; verified by planning from primaries, its own gate, tests, privacy scan and its own `--calc` run agreeing on all 27; `specs/0013` "Planning verification and rulings on P1") |
| P2 | `--formula-sample`; the corpus run with the run log's independent XML comparison and the LibreOffice computation check | DONE 2026-09-23 (71e643b, records a29c3f9, local; the spec's expected `chain_after` corrected mid-phase to the chained workbooks without an A, `specs/0013` P2 ruling 1; the record pair's expectation column is the runner's generic one; verified by planning from primaries, its own gate, tests, privacy scan, its own comparison over the run directory agreeing on all 27 on both targets and its own LibreOffice second conversion computing all 53 written cells; `specs/0013` "Planning verification and rulings on P2") |
| P3 | the catalogue and Limits; `probe-emend-formulas-write-xml` and the size record against the ceiling; the negative log through the binary; the corpus run again with the release binaries | DONE 2026-09-23 (e0a7b95, records 7e623de, local; verified by planning from primaries, its own gate, tests, privacy scan, its own rebuild of five probes reproducing the size log's hashes, and its own comparison over the run directory agreeing on all 27 on both targets; budget figure 312812 of 393216, part 6's cost to the budget 8400 (968 over the styles probe plus 7432 of that probe's drift), the `dates` feature 4980 outside it; `specs/0013` "Planning verification and rulings on P3") |
| Close | close-out push after P3, as part 5's, carrying b0c21ef | DONE 2026-09-23 (pushed 77d2b50..6d2dc62, CI run 35924910390 success; record 03669ca local, goes with the next push) |

### Part 7: the names-only XML comparison (`specs/0014-xml-comparison.md`)

Approved by the maintainer 2026-09-23 and sent to implementation,
the four decisions taken as planning recommended (values as
multisets of hashes with two one-sided counts; names printed for
every family; two phases and no size record; `quick-xml` in
`xlsx-diff`, which never links `emend`). For every changed `.xml`,
`.rels` or `.vml` entry, `xlsx-diff` prints `xml` rows: a `parse`
row per side, an `element` or `attribute` row for each resolved
name whose counts differ, and a `values` row with the two one-sided
counts of value hashes that have no match; four `archive` rows
summarise them. Names and counts only; no value, hash or sample of
one is ever printed. The record script gains a table by family.
Acceptance: the no-leak test, and part 6's `--formula-sample` rerun
over the corpus with the run log asserting the tool's rows against
deltas it derives from the input. No push during the part; the
close-out push follows P2 and carries the local commit 03669ca.

| Phase | Part | Status |
|-------|------|--------|
| P1 | `quick-xml` in `xlsx-diff`; the comparison, the `xml` rows and the four `archive` rows; the Limits; the fixture tests and the no-leak test; the negative log | DONE 2026-09-23 (43731f2, records 5fa6c8f, local; nine P1 rulings before code; verified by planning from primaries, its own gate, tests, privacy scan, its own no-leak run and its own run of the built tool over the 27 corpus pairs of part 6 P3 with derived expectations agreeing on all 27; `specs/0014` "Planning verification and rulings on P1") |
| P2 | the record script's table and `.txt` rows with its test; the corpus run `--formula-sample` with the new tool, the run log asserting the rows against its own derivation; the entry statuses compared with part 6 P3's record | DONE 2026-09-23 (ea37ad7, records a39c3cd, local; three code commits, the third a defect fix the negative log measured; verified by planning from primaries, its own gate, tests on both targets, privacy scan, its own comparison over the run directory agreeing with the tool on all 27 workbooks on both labels and with part 6 P3's outputs byte for byte, and its own tally of the record pair reproducing every family row; record pair `records/m1p7-p2-2026-09-24-ea37ad7.md` and `.txt`, named by the run's UTC date; `specs/0014` "Planning verification and rulings on P2") |
| Close | close-out push after P2, as part 6's, carrying 03669ca and part 7's ten commits | DONE 2026-09-23 (pushed 6d2dc62..4bc741e, 12 commits; CI run 35941782215 success; record 9232e61 local, goes with the next push; verified by planning from the logs, git and `gh`; `specs/0014` "Planning verification and rulings on the close-out") |

### Part 8: open checks for every edit test and the Excel check (`specs/0015-open-checks.md`)

Approved by the maintainer 2026-09-23 and sent to implementation,
the four decisions taken as planning recommended (every crate test
that writes a workbook and every tool test that exits 0 with an
output is kept, the archive-level tests excluded; kept files named
by the test thread's name; from Milestone 2 on every phase's run
log opens the kept pairs on the host; the Excel list holds part 6
P3's host outputs and the kept fixture outputs). An opt-in keep of
each edit test's input and output, switched by an environment
variable the gate never sets; the runner's open check moved into a
shared library; a script that opens every kept pair in the pinned
image and records hashes and results by test name; the Excel check
as a list by sha256 the maintainer carries to the desktop machine
and a record template that takes pass, repair or fail per hash.
Milestone 1 is accepted when that record shows pass on every row.
No core code; no size record. No push during the part; the
close-out push follows P2 and carries the local commit 9232e61.

Carried from P2 (`specs/0015` P2 verification rulings 1, 4 and 6),
scripts-only, fixed in one code commit of the first phase that next
edits `scripts/edit-open-check.sh` or `scripts/excel-check-list.py`
and before the list is regenerated for the Excel check: the
record's "refused by LibreOffice" heading names a fixture
LibreOffice cannot load (zip-level, or no relationships part, or
an empty content-types root); the list script sets umask 077 and
catches IndexError. From Excel check 1 (`specs/0015`, "Planning
rulings on the Excel check", 2026-09-24): the list carries an
input row per fixture pair and the outputs of a style-sample
corpus run; the template carries the automated fixture procedure
(a committed PowerShell script driving Excel through COM, alerts
on, read-only) and an input column; `specs/0012`'s limits state
the styles-part precondition (font, fill, border and cell-style
collections present). Excel check 2 runs on that list and its
filled record is retained. Scoped 2026-09-24 into Milestone 2 part
1 P1 (`specs/0016`, "The carried items"), scripts only, before any
crate change, so the check runs on Milestone 1's code; the
`specs/0012` sentence is in part 1's first doc commit.

Excel check 1 (2026-09-24, `records/excel-check-m1-2026-09-24.md`,
PROVISIONAL): 27 of 27 corpus rows pass with their values (7
measured, 20 reported by the maintainer, filled record not
retained); fixtures 80 pass and 58 refused outright, the inputs
opened too: 76 pass/pass, 54 refused/refused (allowed), 4
refused/pass (explained), 4 pass/refused (ruled the fixtures'
styles-part precondition, not a defect). Excel refuses skeletal
fixtures LibreOffice tolerates; the fixture rule is now
LibreOffice's.

| Phase | Part | Status |
|-------|------|--------|
| P1 | the keep in `testing.rs`, `written`, the cells test and the tool's helper; `scripts/lo-check.sh` sourced by the runner; `scripts/edit-open-check.sh`; the keep tests; the negative log with the runner's T3 P3 cases rerun; the run log's keeps on both targets compared and the first record `records/m1p8-open-<date>-<sha>.md` and `.txt` | DONE 2026-09-23 (e2b8a81, records 50d2c4a, local; eight P1 rulings, two of them mid-phase on the tool fixtures; verified by planning from primaries, its own gate, tests on both targets, privacy scan, its own keep run reproducing every kept pair's hashes and its own LibreOffice pass agreeing on all 149 pairs; record `records/m1p8-open-2026-09-24-e2b8a81.md` and `.txt`: 149 pairs, 13 inputs refused, none on an accepted input; finding F3: five crate fixtures failed only the sheet count, fixed in P2; `specs/0015` "Planning verification and rulings on P1") |
| P2 | first code commit: the prefix-tolerant sheet count and the record's reason classes (P1 verification ruling 3), the script rerun on P1's keep; then `scripts/excel-check-list.py`, the record template `records/excel-check-template.md`, the dry run of the list on part 6 P3's run directory with every hash checked; the negative log | DONE 2026-09-24 (da080f1, records 686ba6d, local; eleven P2 rulings, eight of them mid-phase on the script; verified by planning from primaries, its own gate, tests on both targets, clippy, its own keep comparison on all 298 files and LibreOffice verdicts agreeing on all 149 pairs, and its own derivation of the list's 27 corpus rows agreeing cell for cell without printing a reference; record `records/m1p8-open-2026-09-24-da080f1.md` and `.txt`: 149 pairs, 8 refused by LibreOffice, 3 the sheet count did not apply to, 0 findings; the list at sha256 0e7e19f4..., 165 rows, uncommitted; findings F1 to F10 ruled, F2, F6 and F7 carried; `specs/0015` "Planning verification and rulings on P2") |
| Close | close-out push after P2, as part 7's, carrying 9232e61 | DONE 2026-09-24 (pushed 4bc741e..b06e0a7, 17 commits, 528 blobs, 0 findings; CI run 36006157721 success; verified by planning from git, `gh` and its own blob scan; the post-push scan a recorded FAIL on two compound-shape checks in the push script's printed source, the pushed content unaffected, ruled in `specs/0015` "Planning verification and rulings on the close-out": push.log committed unedited, a correction log reruns the two checks through named functions, the scan rerun ends FAIL as recorded; doc commit f6c2dba and record commit 065f3e4 local, verified by planning, go with the next push) |

## Milestone 2: openpyxl's editing surface (parts 1 to 4 DONE and PUSHED 2026-10-01; ACCEPTANCE PENDING the Excel check, deferred a thirteenth time on 2026-10-09 at Milestone 3 part 7's close-out and a fourteenth time on 2026-10-09 at Milestone 4's approval by the maintainer's decisions, `specs/0029` "Planning rulings before the close-out" and `specs/0030` "Out of scope"; it gates no push in Milestone 4)

Outline approved as a proposal by the maintainer on 2026-09-23,
after planning's comparison with openpyxl. Every part is DONE and
pushed (origin/main fe584b3, 2026-10-01). Acceptance waits on the
per-milestone Excel check, every corpus row of every Milestone 2
sample on the laptop, deferred by the maintainer on 2026-09-27,
2026-09-29, twice on 2026-09-30 and on 2026-10-01 (`specs/0022`
decision 7), due before Milestone 3 part 1's close-out push at the
latest; the acceptance record follows Milestone 1's form. Scoping
started after Milestone 1 was accepted (part 8 and the
per-milestone Excel check);
each part is scoped into phases, with its own spec, before work on
it starts. The core stays on `zip` and `quick-xml`; anything that
does not fit the size ceiling goes behind a Cargo feature (outside
the budget figure, as `dates` is) or into a second crate. Size
figures are planning's estimates in section bytes; the headroom
after part 6 is 80404, and parts 7 and 8 add no core code.

1. A new workbook from a minimal fictional template embedded in
   the crate (generated by a recorded script and pinned by hash; no
   writer, since every later edit goes through the existing
   rewrite); add, remove, rename and reorder sheets. About 25 KB.
2. Fuller cell styles: fonts, fills, borders, alignment, through
   the part 5 styles rewrite. About 20 KB.
3. Row and column operations: insert and delete rows and columns
   (without reference fixup, as openpyxl), widths, heights, hidden
   rows and columns, freeze panes, merged cells. About 25 KB, where
   the headroom runs out; a feature if the measurement says so.
4. Behind features or in a second crate: defined names, hyperlinks,
   comments, data validation, conditional formatting, tables,
   protection, document properties, images.

Stays out by CLAUDE.md's definition: formula evaluation, rendering,
autofit, and reference fixup on row or column insertion. Corpus P5
and P6 (Google Sheets exports; a macro file, threaded comments,
chartex) were wanted before part 1, since removing a sheet can
orphan the parts they cover; the maintainer chose on 2026-09-24
not to supply them yet, so part 1 P4 runs with the gap recorded
(`specs/0016` decision 4).

### Part 1: a new workbook and the sheet list (`specs/0016-new-workbook-and-sheets.md`)

Approved by the maintainer 2026-09-24 and sent to implementation,
decisions 1 to 4 taken as planning recommended in the maintainer's
reply to the plan and decision 5 (the PowerShell script opens the
corpus rows too and reads the sample values through COM) with the
spec's approval. A new
workbook is six fictional template parts held as XML text in the
crate, recorded as additions on an empty archive and written
through the existing write, then opened back: no writer. Sheet
edits (`add_sheet`, `rename_sheet`, `move_sheet`, `remove_sheet`,
and `sheets` for the projected list) are recorded and applied by
`write_to` as one rewrite of the workbook part's `<sheets>`,
`<definedNames>` and `<bookViews>` regions with the package
layer's relationship, override and entry edits; removal takes the
parts only the removed sheet reaches and drops the calculation
chain. Names are checked by Excel's rules. Not fixed, as openpyxl:
formula and defined-name text naming a renamed or removed sheet,
pivot sources, the document-properties title list. Estimate 25 KB
of the 80404 headroom. P1 is scripts only and carries part 8's
items and the Excel check 2 list; the close-out push carries the
four local commits f6c2dba, 065f3e4, a49f2d3 and d4ab367.

| Phase | Part | Status |
|-------|------|--------|
| P1 | the carried part 8 items in `scripts/edit-open-check.sh`, `scripts/excel-check-list.py` (`--run-plain`, input rows, umask, IndexError), `scripts/excel-check-open.ps1` and the record template; the style-sample corpus run on the host at HEAD; the keep on both targets and the edit open check; the Excel check 2 list from part 6 P3's run, the style-sample run and the keep, staged as one zip | DONE 2026-09-25 (ade3c49, records 56e7c34, local; eleven P1 rulings before code, two of them corrections of the spec; verified by planning from primaries, its own privacy scan, its own reading of the `.ps1`, its own checks of the list's 330 rows and the zip's 332 entries by hash, the keep against the open record and the 27 style outputs byte-identical to part 5 P2's; record `records/m2p1-p1-2026-09-25-ade3c49.md` and `.txt`, open record `records/m2p1-open-2026-09-25-ade3c49.md` and `.txt`; the list at sha256 bcda16d9..., uncommitted; G1 to G3 ruled, the `.ps1` Close fix and the rebuilt zip in P2; `specs/0016` "Planning verification and rulings on P1") |
| P2 | the six template files and the empty-archive constant; `Package::new`; the `styles` header's precondition sentence; the `.ps1` Close fix and the rebuilt Excel check 2 zip (P1 verification ruling 3); `--new-sample` with the pair diffed, compared across targets and opened | DONE 2026-09-25 (e7c7061, records 3a1f0ec, local; six P2 rulings before code; verified by planning from primaries, its own privacy scan, tests on its own build, its own `--new-sample` run reproducing both files byte for byte, its own diff and LibreOffice opens of the pair, and its own checks of the rebuilt zip's 332 entries by hash; template files pinned in run.log; open record `records/m2p1-open-2026-09-25-e7c7061.md` and `.txt`, 155 pairs, 0 findings; the Excel check 2 zip at sha256 b25fdf16..., uncommitted; `specs/0016` "Planning verification and rulings on P2") |
| P3 | `Package::sheets`, the name rules, `add_sheet`, `rename_sheet`, `move_sheet`; the three-region workbook rewrite; `--sheet-sample` over the corpus with the run log's comparison | DONE 2026-09-25 (09b42e4, records 2797fe9, local; ten P3 rulings, two of them corrections of the spec, ruling 9 a harness gap: the open check counted an output against the input's sheets, fixed in a7f7060; verified by planning from primaries, its own privacy scan, tests on both targets and clippy on its own build, its own `--sheet-sample` corpus run byte-identical to the phase's and its own comparison of the three regions on 27 of 27; records `records/m2p1-p3-2026-09-25-09b42e4.md` and `.txt`, 95 sheets to 122; open record `records/m2p1-open-2026-09-25-09b42e4.md` and `.txt`, 175 pairs, 0 findings; `specs/0016` "Planning verification and rulings on P3") |
| P4 | the reachability walk, `remove_sheet`, the chain rule; the catalogue and limits; `probe-emend-sheets-write-xml` and the size record; `--remove-sample` over the corpus with the independent walk in the run log | DONE 2026-09-25 (3bd4257, records c7d7870, local; eleven P4 rulings; verified by planning from primaries, its own privacy scan, tests on its own build, its own rebuild of three probes reproducing the figures, its own `--remove-sample` run with all 27 outputs byte-identical, and its own walk on both labels; `specs/0016` "Planning verification and rulings on P4"; size finding: the budget figure 478992 section bytes against the ceiling 393216, 85776 over; the logged attribution puts 109012 of it on `Package::new`'s second copy of the read and write paths and 25508 on the sheet edits; the maintainer's decision 6 records P4 as measured with the excess a finding and adds P5) |
| P5 | one copy of the read and write paths under the generic shell (`Package<R>`, `Archive<R>` and `write_to` unchanged in signature; the reader and the writer erased once underneath); the thirteen probes measured again against P4's record; `--remove-sample` over the corpus with every output byte-identical to P4's | DONE 2026-09-25 (975c974, records 558d659, local; seven P5 rulings; the one public change is `R: 'static` on the two `open` constructors; the budget figure 375984 section bytes, 17232 under the ceiling 393216, the sheets probe down 103008, no earlier probe grew; every corpus output byte-identical to P4's on both labels; verified by planning from primaries, its own privacy scan, tests on its own build, its own rebuild of six probes reproducing the figures and hashes, its own run and its own walk; `specs/0016` "Planning verification and rulings on P5") |
| Close | close-out push after P5 with the push and ci-run self-scan rule (`specs/0001`, planning's doc edit) and the scan header naming the m1p8-close finding | DONE 2026-09-25 (pushed b06e0a7..c115d45, 28 commits, 0 findings; CI run 36158180907 success; the post-push scan PASS with the m1p8-close finding run again and passed; one record commit e81bf5b local, goes with the next push; verified by planning from git, `gh`, the three logs and its own blob scan; `specs/0016` "Planning verification and rulings on the close-out") |
| Close | close-out push after P4 with the push and ci-run self-scan rule (`specs/0001`, planning's doc edit) and the scan header naming the m1p8-close finding | SUPERSEDED 2026-09-25 (decision 6: the push followed P5, the row above) |

### Part 2: fuller cell styles (`specs/0017-fuller-styles.md`), DONE and PUSHED 2026-09-26

Approved by the maintainer 2026-09-25 and sent to implementation,
decision 1 taken as A (a Milestone 2 ceiling of 458752 section
bytes judged at the close of part 3, with a checkpoint after this
part's size record; part 1 had left 17232 under 393216 against an
estimate of about 20000 for this part) and decisions 2 to 4 as
planning recommended. The design: `Package::add_style(base,
&Style)` records a cell format derived as part 5's, naming a new
font, fill and border appended to their collections and carrying
a new `alignment` and `protection`; nothing new is read back; the
styles rewrite grows five containers instead of two. Decisions 2
to 4 (the field set, deduplication among additions only, a
missing collection refused) are design choices with planning's
recommendation, taken. Planning's estimate 15000 to 20000 section
bytes.

| Phase | Part | Status |
|-------|------|--------|
| P1 | the counts and accessors; the types and composers; `add_style` with `add_cell_style` over it; the rewrite over five containers; the three messages; `--font-sample` over the corpus | DONE 2026-09-25 (cc81810, records 6080f2a, local; five P1 rulings, two of them corrections of the spec, ruling 3 wiring borders in P1; verified by planning from primaries, its own privacy scan, tests and clippy on its own build, and its own comparison over the run directory agreeing on 27 of 27 on both labels; records `records/m2p2-p1-2026-09-25-cc81810.md` and `.txt`; open record `records/m2p2-open-2026-09-25-cc81810.md` and `.txt`, 210 pairs, 0 findings; check 1's F1, F2, F7, F8, F9 and F12 ruled, four of them into P2's first code commit; `specs/0017` "Planning verification and rulings on P1") |
| P2 | the `xf` content replacement (`alignment`, `protection`); the P1 verification items (the font name's references, the count wording and test, the stderr and `removed_in` script rules); `--border-sample` over the corpus | DONE 2026-09-25 (09c941f, records ebfe711, local; every log attempt 1 with its own stderr counted at 0; verified by planning from primaries, its own privacy scan, tests and clippy on its own build, and its own comparison over the run directory agreeing on 27 of 27 on both labels; records `records/m2p2-p2-2026-09-25-09c941f.md` and `.txt`; open record `records/m2p2-open-2026-09-25-09c941f.md` and `.txt`, 220 pairs, 0 findings; check 1's F1, F3, F7, F9 and F10 ruled, F3's unbound-prefix refusal kept with a P3 test; `specs/0017` "Planning verification and rulings on P2") |
| P3 | the P2 verification items (the unbound-prefix test and sentence, the gate and run script fixes); the catalogue and limits; `probe-emend-fullstyles-write-xml` and the size record against the Milestone 2 ceiling 458752 with the checkpoint at 25000 for the part; the negative log through the binary; both samples again with the release binaries | DONE 2026-09-26 (1fc8327, records 76f4461, local; two P3 rulings, the samples in two run directories and the checkpoint judged on the whole growth; verified by planning from primaries, its own privacy scan, tests, clippy and doc on its own build, its own rebuild of the two budget probes reproducing the hash and the figures, and its own comparisons over both run directories agreeing on 27 of 27 on both labels; the budget figure 399176 section bytes, 59576 under the Milestone 2 ceiling 458752; the part's whole growth 23192, under the 25000 checkpoint; records `records/size-m2p2-2026-09-26-1fc8327.md`, `records/m2p2-p3-font-2026-09-26-1fc8327.md`, `records/m2p2-p3-border-2026-09-26-1fc8327.md` and `.txt`, open record `records/m2p2-open-2026-09-26-1fc8327.md` and `.txt`, 220 pairs, 0 findings; check 1's F7, F10, F9, F11 and F14 ruled, three rules carried to part 3; `specs/0017` "Planning verification and rulings on P3") |
| Close | close-out push after P3, carrying part 1's record commit e81bf5b and this part's fifteen commits, with the push and ci-run self-scan rule (`specs/0001`) | DONE 2026-09-26 (pushed c115d45..42588cc, 17 commits, 654 blobs, 0 findings; CI run 36209039533 success; the post-push scan PASS; one record commit d0f8f72 local, goes with the next push; verified by planning from git, `gh`, the three logs and its own blob scan; `specs/0017` "Planning verification and rulings on the close-out") |

### Part 3: rows and columns (`specs/0018-rows-and-columns.md`)

Approved by the maintainer 2026-09-26 and sent to implementation,
decisions 1 to 6 as planning recommended: built in the core (A)
with an interim measurement after P2 against a 40000 checkpoint
and the Milestone 2 ceiling of 458752 judged at P4's size record
(the checkpoint tripped at 57868; decision 7 A, taken 2026-09-26,
puts the insertions and deletions behind the Cargo feature `rowcol`
from P3's first code commit and judges the ceiling on the core
probe with the feature off);
the shift rule moves the grid, `dimension`, merges, `cols`,
`spans`, shared and array `ref`s and the four range lists (A);
tables and pivot tables refused when an edit would move or cut
one; comments, drawings, views, defined names and every formula
copied raw and stated in the limits; one insertion or deletion per
worksheet per write and no other edit to that worksheet in the
same write; a merge clears the covered cells except the top-left.
The design: `cells::Area` public; `set_column_width`,
`set_column_hidden`, `set_row_height`, `set_row_hidden`,
`freeze_panes`, `merge_cells`, `unmerge_cells`, `insert_rows`,
`delete_rows`, `insert_columns`, `delete_columns` on `Package`;
absent `sheetViews`, `cols` and `mergeCells` created in schema
position by one table of the root's child order. Planning's
estimate 30000 to 40000 section bytes against 59576 of headroom.

| Phase | Part | Status |
|-------|------|--------|
| P1 | `Area` public; the schema-position table and creation rule; the four format calls with `<cols>` splitting and row retagging; `freeze_panes`; `merge_cells` and `unmerge_cells`; `--format-sample` over the corpus; the negative log with the unbound-prefix case carried from part 2 | DONE 2026-09-26 (b8595ce, records 2e54f33, local; nine P1 rulings before code, two of them corrections of the spec; every log attempt 1; verified by planning from primaries, its own privacy scan, tests on both targets, clippy and doc on its own build, and its own comparison over the run directory agreeing on 27 of 27 on both labels; records `records/m2p3-p1-2026-09-26-b8595ce.md` and `.txt`; open record `records/m2p3-open-2026-09-26-b8595ce.md` and `.txt`, 266 pairs, 0 findings; check 1's F3 to F9 ruled, F3 a correction of the spec's streaming sentence, F5 to F7 into P2's first code commit; `specs/0018` "Planning verification and rulings on P1") |
| P2 | the shift rule on the row axis: rows, cells, `<f ref>`, `dimension`, merges, the four range lists; the table and pivot refusal; the one-shift rule; the formula-change rule; `--insert-rows-sample` and `--delete-rows-sample`; the interim size measurement against the 40000 checkpoint | DONE 2026-09-26 (a4b55f0, records dd7062b, local; fourteen P2 rulings, run log at attempt 3 after ruling 13 corrected an unconditional expectation and ruling 14 accepted a wrong derived count beside a right listing hash; the 40000 checkpoint TRIPPED at 57868 whole growth (P1 31196, P2 26672), 1708 under the ceiling, decision 7 with the maintainer before P3's code; verified by planning from primaries, its own privacy scan, tests and clippy on its own build, its own probe rebuild reproducing every size figure, and its own byte-level model of the shift agreeing on 27 of 27 for both samples on both labels; records `records/m2p3-p2-insert-2026-09-26-a4b55f0.md`, `records/m2p3-p2-delete-2026-09-26-a4b55f0.md` and `.txt`; open record `records/m2p3-open-2026-09-26-a4b55f0.md` and `.txt`, 297 pairs, 0 regressions; check 1's F4 to F10 and F12 ruled, five tests and two doc corrections carried to P3; `specs/0018` "Planning verification and rulings on P2") |
| P3 | decision 7 A first: the shifts (`insert_rows`, `delete_rows`, `TABLE`, `PIVOT_TABLE`, the pass and its messages) behind the Cargo feature `rowcol`, off by default, the P1 calls in core, the carried P2 tests and script rules; then the rule on the column axis: cells, `spans`, `<col>` intervals, `dimension`, merges, the lists; `--insert-columns-sample` and `--delete-columns-sample`; a size log showing the core figure with the feature off | DONE 2026-09-26 (f52684c, records 3cd9696, local; the shifts behind `rowcol` from b675f3d; the core figure with the feature off 430372, equal to P1's, 28380 under the ceiling; five P3 rulings before the column code; verified by planning from primaries, its own privacy scan, tests off and on, clippy and doc off and on, its own probe rebuild reproducing the core figure, and its own byte-level model of the column shift agreeing on 27 of 27 for both samples on both labels; records `records/m2p3-p3-insert-2026-09-26-f52684c.md`, `records/m2p3-p3-delete-2026-09-26-f52684c.md` and `.txt`; open record `records/m2p3-open-2026-09-26-f52684c.md` and `.txt`, 332 pairs, 0 regressions; two checks, no code defect; two tests, one header sentence and six script rules carried to P4; `specs/0018` "Planning verification and rulings on P3") |
| P4 | the catalogue and limits; `probe-emend-format-write-xml` (core) and `probe-emend-rowcol-write-xml` (the feature) and the size record with the Milestone 2 ceiling judged; the negative log through the binary; all five samples again with the release binaries | DONE 2026-09-27 (b728fdf, records d0073cd, local; seven P4 rulings, two of them corrections of planning's counts, ruling 5 a correction of the spec's "unreachable" claim, rulings 6 and 7 after the implementing session's disk-space restart; gate and test at attempt 2; verified by planning from primaries, its own privacy scan, tests off and on, clippy and doc off and on on its own build, its own rebuild of all sixteen probes reproducing every section figure, and its own byte comparison of the five rerun directories against the phase runs, 54 of 54 identical each; the Milestone 2 result: the budget figure 434668 section bytes on the core probe, 24084 under the ceiling 458752; the part's own cost 4296, the `rowcol` feature 34704 outside the budget; records `records/size-m2p3-2026-09-27-b728fdf.md`, `records/m2p3-p4-<sample>-2026-09-27-b728fdf.md` and `.txt` for the five samples, open record `records/m2p3-open-2026-09-27-b728fdf.md` and `.txt`, 336 pairs, 0 findings; two checks, one code defect (the probes' usage strings) and one header gap, both carried to part 4's first code commit with two remarks; `specs/0018` "Planning verification and rulings on P4") |
| Close | close-out push after P4, carrying part 2's record commit d0f8f72 and this part's commits, with the push and ci-run self-scan rule (`specs/0001`) | DONE 2026-09-27 (pushed 6dbb4a0, 21 commits; run 36327723486 success; record 5e57db1 local; five close-out rulings in `specs/0018`; part 4 headroom 24084 section bytes) |

### Part 4: the openpyxl features, in sub-parts (planning split 2026-09-27)

The outline's item 4 lists nine capabilities. Planning split it on
2026-09-27 into sub-parts with their own specs, each behind its own
Cargo feature off by default, measured beside the core as `dates`
and `rowcol` are, with the Milestone 2 ceiling untouched by design:
4a names, links, properties and protection (`specs/0019`); 4b data
validation and conditional formatting; 4c tables; 4d comments and
images. The order is the maintainer's to change; 4b is scoped
(`specs/0020`, 2026-09-29), 4c (`specs/0021`, 2026-09-30) and 4d
(`specs/0022`, 2026-09-30), the last of part 4. The Excel check for parts 1 to 3 was
deferred by the maintainer on 2026-09-27 to before part 4a's
close-out push at the latest (`specs/0019` decision 6).

### Part 4a: names, links, properties, protection (`specs/0019-names-links-properties-protection.md`)

Approved by the maintainer 2026-09-27 and sent to implementation,
decisions 1 to 5 as planning recommended: one Cargo feature
`metadata` holding all four capabilities, one probe; a defined name
added or removed is a formula change; a worksheet root that binds
no relationships prefix is retagged for a hyperlink; the legacy
sixteen-bit password hash, printable ASCII passwords; the core
properties part created for a workbook without one. `DefinedName`
and `Workbook::defined_names`, `add_defined_name`,
`remove_defined_name`; `LinkTarget`, `set_hyperlink`,
`remove_hyperlink`; `Property`, `Properties`, `properties`,
`set_property`; `SheetProtection`, `legacy_password_hash`,
`protect_sheet`, `unprotect_sheet`, `protect_workbook`,
`unprotect_workbook`; the workbook root's schema-position table
(the one core change). Acceptance: over the corpus each sample
leaves every other entry byte-identical and the edited part
identical outside the elements the edit names, by an independent
parser in the run script; every output opens in LibreOffice; the
names and properties read back. No push during the part; the
close-out push follows P4 and carries 5e57db1.

| Phase | Part | Status |
|-------|------|--------|
| P1 | the feature; `DefinedName`, the read, the two name calls, the workbook table with `calcPr` moved onto it, the formula-change rule; `--names-sample`; the first code commit carries part 3's close-out items (three, ruling 1: the fourth already in the tree) | DONE 2026-09-28 (e981810, records 4c3485a, local; two P1 rulings during the phase, the second a message-only rewrite of three unpushed commits with trees checked equal; verified by planning from primaries, its own privacy scan, tests off and on on both targets, clippy and doc on its own build, and its own tool run over the corpus with every output byte-identical to the phase run's; corpus run 27 of 27, 816 identical, 45 changed, 9 dropped; keep 367 pairs; two checks, ten verification rulings, three code items and six script rules carried to P2; `specs/0019` "Planning verification and rulings on P1") |
| P2 | `LinkTarget`, `HYPERLINK`, the two link calls, the relationship removal, the root retag; `--link-sample` | DONE 2026-09-28 (cced544, records a9e5cb6, local; no rerun; verified by planning from primaries, its own privacy scan, tests off and on on both targets, clippy and doc on its own build, and its own tool run over the corpus with every output byte-identical to the phase run's; corpus run 27 of 27, 822 identical, 48 changed, 6 added; keep 381 pairs; two checks, ten verification rulings, six code items and three script rules carried to P3; `specs/0019` "Planning verification and rulings on P2") |
| P3 | `Property`, `Properties`, the read and `set_property` with the part's creation; `SheetProtection`, the hash, the four protection calls; `--props-sample`, `--protect-sample` | DONE 2026-09-28 (372bc21, records d3bb207, local; the P2 carries in bc46388; ten mid-phase rulings on check 1's F1, a defect of 51acd32 (a property edit on a package whose core relationship names a missing part is lost at the write), recorded as measured and carried to P4's first code commit; verified by planning from primaries, its own privacy scan, tests off and on on both targets, clippy and doc on its own build, and its own release tool run of both samples byte-identical to the phase's 27 of 27 each; props 843 identical / 27 changed, protect 816 / 54, none dropped or added; records `records/m2p4a-p3-props-2026-09-28-372bc21.md`, `records/m2p4a-p3-protect-2026-09-28-372bc21.md` and `.txt`, open record `records/m2p4a-open-2026-09-28-372bc21.md` and `.txt`; `specs/0019` "Planning rulings during P3" and "Planning verification and rulings on P3") |
| P4 | the catalogue and limits; `probe-emend-metadata-write-xml` and the size record (the feature's cost, the core's growth); the negative log through the binary; all four samples again with the release binaries | DONE 2026-09-28 (3e67745, record b18d967, local; the P3 carries in 4ece0f0, the catalogue in fb34ab0; verified by planning from primaries, its own privacy scan, tests off and on on both targets, clippy and doc on its own build, its own probes reproducing every figure to the byte, and its own run of all four samples byte-identical to the phase's 27 of 27 each on both targets; the feature `metadata` costs 50404 section bytes outside the budget, the core grew 428 for the part and 556 since b728fdf, the budget figure 435224 against 458752, 23528 of headroom; one defect of form (four two-case tests for the six the rulings named) accepted as measured; fourteen rulings in `specs/0019` "Planning verification and rulings on P4"; records `records/size-m2p4a-2026-09-29-3e67745.md`, `records/m2p4a-p4-<sample>-2026-09-29-3e67745.md` and `.txt` for names, link, props and protect, open record `records/m2p4a-open-2026-09-29-3e67745.md` and `.txt`) |
| Close | close-out push after P4, carrying 5e57db1 and this part's commits; the Excel check for parts 1 to 3 run or deferred again before it | DONE 2026-09-29 (pushed 6dbb4a0..b9bf28c, 22 commits, 791 blobs, 0 findings; CI run 36562738031 success; doc b9bf28c pushed, record 335da53 local, goes with the next push; the Excel check deferred again by the maintainer, decision 6, now due before part 4b's close-out with this part's four samples on its list; verified by planning from git, `gh` and its own blob scan; `specs/0019` "Planning verification and rulings on the close-out"; part 4b headroom 23528 section bytes) |

### Part 4b: data validation and conditional formatting (`specs/0020-validation-and-conditional-formatting.md`)

Approved by the maintainer 2026-09-29 and sent to implementation,
decisions 1 to 5 as planning recommended: one feature `rules` and
one probe; the visual rules in the 2007 form; both lists read
back; two validations on one cell refused; a differential solid
fill written as given. Behind
a feature, outside the budget; estimate 40000 to 60000 section
bytes with part 4a's 2.5x ratio in view; the core's growth
expected near 0. Carries from part 4a: the `add_part` test in P1's
first code commit, the scan control, the check-prompt rule, the
Excel check (parts 1 to 4a, deferred again 2026-09-29) before the
close-out push with this part's three samples.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the feature `rules`; `Validation`, `add_validation`, `remove_validation`, `validations`; the container pass with the overlap check; `--validate-sample` | DONE 2026-09-29 (doc 715177a, carry 8b82262, crate 2ed9070, tool e79739a, record 157c11c; 27/27 both targets, 0 refused; core growth 0; planning verified from primaries, `specs/0020` P1 verification rulings 1 to 9) |
| P2 | `add_differential_style` through the styles rewrite; `Rule`'s six dxf kinds, `add_conditional_format`, `remove_conditional_format`, `conditional_formats`, the priority rule; `--cf-sample` | DONE 2026-09-29 (doc 609eb7f, carry 5ad72b5, crate fa3ad04 + 08729d3, tool 20d7d93, record 0999a35; 27/27 both targets, 0 refused; core growth 0; planning verified from primaries, `specs/0020` P2 verification rulings 1 to 9) |
| P3 | the visual rules: colour scales, data bars, icon sets; `--scale-sample` | DONE 2026-09-30 (doc b88f686, carry 03bbe82, crate 1265000 + 631c719, tool c2fe42b, record dd1a415; 27/27 both targets, 0 refused; core growth 0; P3 rulings 1 to 4 mid-phase, ruling 2 a new phase commit with every log run again; planning verified from primaries, `specs/0020` P3 verification rulings 1 to 6) |
| P4 | the catalogue and limits; `probe-emend-rules-write-xml` and the size record (the feature's cost, the core's growth); the negative log through the binary; all three samples again with the release binaries | DONE 2026-09-30 (doc 2679b59, carry 51c32da, catalogue 3e48f99, probe 5854e34, record 2f7933b; feature `rules` 48116 section bytes, core growth 0, budget 435224 against 458752, 23528 under, unchanged from 4a; negative log 5 of 15 reached, 10 at-call stated unreachable; all three samples 27/27 both targets byte-identical to the phase runs; planning verified from primaries, its own probe builds reproducing every figure and its own three runs; `specs/0020` P4 verification rulings 1 to 9; `records/size-m2p4b-2026-09-30-5854e34.md`) |
| Close | close-out push after P4, carrying 335da53 and this part's commits; the Excel check for parts 1 to 4a and this part's three samples run or deferred again before it | DONE 2026-09-30 (pushed b9bf28c..116def3, 25 commits, 852 blobs, 0 findings; CI run 36711711649 success; doc commits 4a2aadb and 116def3 pushed, the second holding decision 6's record after the push script's pre mode rightly refused a modified tree, `specs/0020` P4 verification rulings 10 and 11; record c143473 local, goes with the next push; the Excel check deferred again by the maintainer, decision 6, now due before the next part's close-out with this part's three samples on its list; verified by planning from git, `gh` and its own blob scan; `specs/0020` "Planning verification and rulings on the close-out"; next part's headroom 23528 section bytes) |

### Part 4c: tables (`specs/0021-tables.md`)

Approved by the maintainer 2026-09-30 and sent to implementation,
decisions 1 to 5 as planning recommended: 4c before 4d; one feature
`tables` and one probe; the add call writes the column names on the
header row and refuses a header cell holding a formula; an added
table refused where it overlaps a table, a pivot table, a merge or
the autofilter; no totals row or calculated column, built-in styles
only. `Table`, `TableStyle`, `TableStyleInfo`, `TableEntry`;
`add_table`, `remove_table`, `tables`; the table part created with
its override and relationship, the `<tableParts>` pass, the chain
dropped on removal. Behind a feature, outside the budget; estimate
30000 to 45000 section bytes with parts 4a's and 4b's ratios in
view; the core's growth expected near 0. Carries from part 4b: the
three catalogue phrases in P1's first code commit, the size log's
cause control, the run log's record-commit naming, the Excel check
(parts 1 to 4b, deferred again 2026-09-30) before the close-out
push with this part's sample.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the feature `tables`; the types, `tables`, `add_table` with the name, column and style rules, the header cells, the part writer and the allocation, the override and relationship, the `<tableParts>` pass with the overlap check; `--table-sample` | DONE 2026-09-30 (doc 1fefcbf, carry 0985ac4, crate 2e84516, tool 40ae039, record 7176a34; 27/27 both targets, 0 refused, 25 header rows appended and 2 filled per target; seven P1 rulings mid-phase, two of them corrections of the spec's sentences on an added sheet and the strict form, one a grid-bounded name rule that found the same defect in part 4a's defined-name rule; verified by planning from primaries, its own build and tests on both targets, clippy and doc, its own sample run byte-identical to the phase's on 27 of 27 on both targets and its own check of every added part's bytes; records `records/m2p4c-p1-2026-09-30-40ae039.md` and `.txt`, open record `records/m2p4c-open-2026-09-30-40ae039.md` and `.txt`; check 1's items carried to P2's first code commit with the 4a fix; `specs/0021` "Planning verification and rulings on P1") |
| P2 | `remove_table` with its four removals; `--table-sample`'s in-memory removal | DONE 2026-09-30 (doc 30241a1, carry 2b1cd7d, crate 4a68144, tool 6cfd504, record af43c51; 27/27 both targets, 0 refused, the removal's counts asserted on every workbook; the carry fixed part 4a's defined-name rule (grid-bounded, R1C1 forms refused) beside the P1 items; verified by planning from primaries, its own build and tests on both targets, clippy and doc, its own sample run byte-identical to the phase's on 27 of 27 on both targets; records `records/m2p4c-p2-2026-09-30-6cfd504.md` and `.txt`, open record `records/m2p4c-open-2026-09-30-6cfd504.md` and `.txt`; three doc and test items carried to P3's first code commit; `specs/0021` "Planning verification and rulings on P2") |
| P3 | the catalogue and limits; `probe-emend-tables-write-xml` and the size record (the feature's cost, the core's growth); the negative log through the binary; the sample again with the release binaries | DONE 2026-09-30 (doc 1b8fe5a, carry f70495e, catalogue b7e341c, probe 3c5f0b7, record e1f3736; the feature `tables` costs 46860 section bytes over the core probe, above the estimate's 30000 to 45000 by 1860 and outside the budget; the core's growth 0; the budget 435224, 23528 under 458752, unchanged from part 4b; the metadata probe +32 from the P2 carry's defined-name fix; 4 of the part's 10 messages reached through the binary, 6 stated unreachable with their tests; 27/27 both targets byte-identical to P2's run; verified by planning from primaries, its own build and tests on both targets, clippy and doc, its own probe builds reproducing the size log's hashes, its own sample run byte-identical to the phase's on 27 of 27 on both targets; `records/size-m2p4c-2026-09-30-3c5f0b7.md`, `records/m2p4c-p3-2026-09-30-3c5f0b7.md` and `.txt`, open record `records/m2p4c-open-2026-09-30-3c5f0b7.md` and `.txt`; a test and two `//!` phrases carried to the next part's first code commit; `specs/0021` "Planning verification and rulings on P3") |
| Close | close-out push after P3, carrying c143473 and this part's commits; the Excel check for parts 1 to 4b and this part's sample run or deferred again before it | DONE 2026-09-30 (pushed 116def3..c8bfaf9, 17 commits, 873 blobs, 0 findings; CI run 36795385286 success; the Excel check deferred again by decision 6, `specs/0021` P3 ruling 7; doc c8bfaf9 pushed, record 5da68da local, goes with the next push; `specs/0021` "Planning verification and rulings on the close-out") |

### Part 4d: comments and images (`specs/0022-comments-and-images.md`)

Approved by the maintainer 2026-09-30 and sent to implementation,
decisions 1 to 6 as planning recommended: two features `comments`
and `images` with a probe each; a comment written as a comments part
and a VML legacy-drawing shape, openpyxl's shape; a sheet with
threaded comments refused for comment edits; PNG and JPEG only, the
pixel size read from the file's header by the part's own code with
a caller override; an image appended as a one-cell anchor to an
existing drawing part; a comment on a cell that has one replaces
it, images never overlap-checked. `Comment`, `CommentEntry`;
`add_comment`, `remove_comment`, `comments`; `Image`, `ImageEntry`,
`Anchor`; `add_image`, `images`; image removal out of scope. Behind
features, outside the budget; estimate 40000 to 55000 section bytes
each; the core's growth expected near 0. Carries from part 4c: a
`remove_table` test and two catalogue phrases in P1's first code
commit, the push script's previous-record pattern and one
independent check at the close-out, the Excel check (parts 1 to
4c, deferred again 2026-09-30) before the close-out push with this
part's two samples.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the feature `comments`; `Comment`, `CommentEntry`, `comments`, `add_comment`; the comments and VML parts created or appended, the allocation, the overrides and relationships, `legacyDrawing` in schema position, the threaded-comments refusal; `--comment-sample` | DONE 2026-10-01 (doc 77ed347, carry 39d7ab7, crate 96dde75, tool f7d6a7d, record 2d675da; 27/27 both targets, 0 refused, 22 workbooks with both parts created and 5 appended to; five P1 rulings mid-phase, the VML shape-id rule made per part on the implementing session's corpus survey, two of them corrections of planning's own text; verified by planning from primaries, its own build and tests on both targets off and on, clippy and doc, its own sample run byte-identical to the phase's on 27 of 27 on both targets and its own check of every output's parts; records `records/m2p4d-p1-2026-10-01-f7d6a7d.md` and `.txt`, open record `records/m2p4d-open-2026-10-01-f7d6a7d.md` and `.txt`; one refusal and its tests carried to P2's first code commit; `specs/0022` "Planning verification and rulings on P1") |
| P2 | `remove_comment` with its skips and drops; `--comment-sample`'s in-memory removal | DONE 2026-10-01 (doc 9523065, carry 89045f2, removal 193aa84, tool fcd254b, the removal case f255959, record 0eadfec; 27/27 both targets, 0 refused, the removal's counts asserted on every workbook; the carry refuses a comment edit on a sheet whose comments or VML part the write touches; one P2 ruling mid-phase, a test for the carry's only load-bearing path, found by the implementing session's plant check; verified by planning from primaries, its own build and tests on both targets off and on, clippy and doc, its own sample run byte-identical to the phase's on 27 of 27 on both targets and the outputs byte-identical to P1's; records `records/m2p4d-p2-2026-10-01-f255959.md` and `.txt`, open record `records/m2p4d-open-2026-10-01-f255959.md` and `.txt`; two comment corrections and a test carried to P3's first code commit; `specs/0022` "Planning verification and rulings on P2") |
| P3 | the feature `images`; `Image`, `ImageEntry`, `Anchor`, the PNG and JPEG header reads, `images`, `add_image`; the media entry, the drawing part created or appended, its rels part, `drawing` in schema position; `--image-sample` | DONE 2026-10-01 (doc d5d1124, carry 16913cf, crate b65ea6d, tool b5244d6, record 2b85b22; 27/27 both targets, 0 refused, 17 drawings created and 10 appended to, the comment sample once more byte-identical to P2's; two P3 rulings mid-phase, a root-level AlternateContent read and the tool's PNG constant corrected to its true size, both found by the implementing session; verified by planning from primaries, its own build and tests on both targets off and on, clippy and doc, its own runs of both samples byte-identical to the phase's on 27 of 27 on both targets and its own check of every output's drawing, media and rels; records `records/m2p4d-p3-2026-10-01-b5244d6.md` and `.txt`, open record `records/m2p4d-open-2026-10-01-b5244d6.md` and `.txt`; four items carried to P4's first code commit; `specs/0022` "Planning verification and rulings on P3") |
| P4 | the catalogue and limits; `probe-emend-comments-write-xml` and `probe-emend-images-write-xml` and the size record; the negative log through the binary; both samples again with the release binaries | DONE 2026-10-01 (doc 20b8c61, carry 5906712, catalogue f5819ce, probes c1f6602, record de946fb; the feature `comments` costs 57032 section bytes over the core probe, above the estimate's 40000 to 55000 by 2032, `images` 40960, within it, both outside the budget; the core's growth 0; the budget 435224, 23528 under 458752, unchanged from part 4c; the tables probe +60 from the shared code; 5 of the part's 10 messages reached through the binary, 5 stated unreachable with their tests; both samples 27/27 both targets byte-identical to P3's runs; verified by planning from primaries, its own build and tests on both targets off and on, clippy and doc, its own probe builds reproducing the size log's hashes and figures, its own runs of both samples byte-identical to the phase's on 27 of 27 on both targets and its own check of every output's parts; `records/size-m2p4d-2026-10-01-c1f6602.md`, `records/m2p4d-p4-image-2026-10-01-c1f6602.md`, `records/m2p4d-p4-comment-2026-10-01-c1f6602.md` and their `.txt`, open record `records/m2p4d-open-2026-10-01-c1f6602.md` and `.txt`; no carry; `specs/0022` "Planning verification and rulings on P4") |
| Close | close-out push after P4, carrying 5da68da and this part's commits, the previous-record pattern naming 5da68da, one independent check; the Excel check for parts 1 to 4c and this part's samples run or deferred again before it | DONE 2026-10-01 (pushed c8bfaf9..fe584b3, 23 commits, 938 blobs, 0 findings; CI run 36891705251 success; the Excel check deferred again by decision 7, `specs/0022` P4 ruling 6; one independent check; the scan script's missing `ANTHROPIC_API_KEY` line found and carried to the next close-out; doc fe584b3 pushed, record ab5b3aa local, goes with the next push; verified by planning from git and `gh`, its own scan of the range and its own count of the maintainer's address over every blob, 0; `specs/0022` "Planning verification and rulings on the close-out") |

Excel check 2 (after P1, the maintainer's, retained record):
confirms Milestone 1 when every corpus row passes with its values
and no fixture output is refused on an accepted input.

## Milestone 3: the rest of openpyxl's surface, and proof at scale (APPROVED 2026-10-01; parts 1 to 7 DONE and PUSHED 2026-10-09; the Milestone 2 Excel check deferred a thirteenth time, then a fourteenth at Milestone 4's approval; it gates no push in Milestone 4)

Outline drafted by planning on 2026-10-01 and approved by the
maintainer the same day ("agree with recs, lets get started"), with
planning's four recommendations taken: the Milestone 2 ceiling
kept for the core, so every capability here goes behind a Cargo
feature off by default and is measured beside the core as part 4
was; charts out (under "Unscoped", a second crate for a later
milestone); corpus P5 and P6 (Google Sheets exports, a macro file,
threaded comments, chartex) supplied by the maintainer before part
1; the scale proof first, since it alone can change the design of
the others. Each part is scoped into phases with its own spec
before work on it starts, with a corpus sample per capability, the
LibreOffice open checks, a size probe and the close-out push.
Estimates are planning's, in section bytes, scaled from the
measured costs of `tables` (46920) and `comments` (57032); the
core's headroom after Milestone 2 is 23528, held for fixes.

1. Proof at scale on i686: a generated large workbook (a recorded
   script, pinned by hash, never committed) run through every
   sample on the musl target under a memory cap, peak memory and
   time recorded beside the corpus figures. No new capability; the
   streaming claim in `CLAUDE.md` measured. Decides whether cell
   reading needs a true streaming mode before parts 2 and 5 build
   on the read path.
2. Rich text: runs with their own font in a cell (shared and
   inline strings) and in a comment, written and read back, as
   openpyxl's `CellRichText`. Feature `richtext`. 40000 to 55000.
3. Sheet and print settings: tab colour, the sheet view (zoom,
   gridlines, headings), page setup and margins, print area and
   print titles as the built-in names, headers and footers, a
   sheet-level autofilter; worksheet elements in schema position
   through part 3's pass. Feature `sheetprops`. 35000 to 50000.
4. Outline grouping and row and column styles: outline levels and
   collapsed state, a style on a whole row or column, kept through
   part 3's shifts. The feature `rowcol` grows. 20000 to 30000.
5. Reading back as typed values: fonts, fills, borders, alignment,
   number formats, widths, heights, panes, merges, hyperlinks,
   protection, the read side Milestone 2 left out part by part.
   Feature `read`. 30000 to 45000.
6. Images and comments, the second half: image removal with the
   media drop, two-cell and absolute anchors, a comment's box size
   and visibility, worksheet copy within a workbook (openpyxl's
   `copy_worksheet`). The features `images` and `comments` grow.
   25000 to 35000.
7. Engineering close: MSRV policy; the harness and privacy-gate
   backlogs under "Unscoped"; a README whose every fidelity, size
   and speed sentence cites a record; an API review before the
   crate is published anywhere.

Stays out: charts, pivot tables and VBA beyond passthrough, and by
`CLAUDE.md`'s definition formula evaluation, rendering, autofit and
reference fixup.

### Part 1: proof at scale (`specs/0023-scale.md`, DONE and PUSHED 2026-10-02)

Approved by the maintainer 2026-10-01 ("a on all"): four generated
tiers (a 50000 rows, b 800000, c 1048576 rows by 120 columns with
one entry over 4 GiB, d 800000 with every string unique), every
sample run over each on both targets under the 2048 MiB cap, the
memory class of each path predicted from the code and measured, a
decision record for parts 2, 5 and 6. The LibreOffice check on tier
a only; the host diff on tier c; the cap on both labels. No new
capability; a defect at scale is fixed here, a design limit is
carried. Tier c was corrected to 120 columns during P1 (planning's
draft sized it at 48 from a guessed bytes-per-cell).

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | `scripts/scale-gen.py` and its tier t test; the four tiers generated with their manifest; the survey of every sample's memory class | DONE 2026-10-01 (doc 48c6e0d, scripts a509ad3, record e2ff972; tiers a, b, c, d generated under the cap in 2.0, 32.7, 198.5 and 35.9 s with peak RSS under 45 MB, tier c's sheet 5025024826 bytes with the zip64 field; the survey predicts 20 samples streaming, 5 holding the output in the tool's read-back, and tier c refusing 8 at the grid; four findings ruled: 24 samples since `--new-sample` writes its input, `TMPDIR` on the disk, a cell past the grid a defect fixed in P2, tier c's fewer write paths recorded; verified by planning from primaries, the generator read in full, its own hashes and central-directory reads of the four tiers, the survey's cited lines read, its own scan; `records/m3p1-gen-2026-10-01-a509ad3.md`; `specs/0023` "Planning verification and rulings on P1") |
| P2 | the grid refusal at the write (P1 ruling 5); the 192 runs, tier a with LibreOffice, then b, d, c, both labels, `TMPDIR` on the disk; the run records per tier | DONE 2026-10-02 (doc 38ec8c2, fix 8d05845, record 079db61; 192 runs, 96 record pairs, one directory per tier per sample; memory class 48 compared, 0 disagreements: 19 samples stream at most 8004 kB on tiers a and b, 5 hold the output in the tool's read-back; tier c 16 ok, 30 refused at the grid or the shift, names killed on both labels in the tool's in-memory second write; five samples rewrote the 5 GB zip64 entry under 8 MiB on i686; LibreOffice ok on tier a's 48 rows; four findings ruled, the survey's link omission recorded, the shared strings figure added to P3; verified by planning from primaries with its own build, its own reruns of tier a and four tier c samples byte-identical, its own recount and scan; `specs/0023` "Planning verification and rulings on P2") |
| P3 | the decision record; the catalogue block "What streams and what is held"; the size record; part 4d's two samples rerun; the open record | DONE 2026-10-02 (doc c9a59be, catalogue 6578925, record fe3c19e; the core grew 128 section bytes, all the grid fix's, the catalogue 0, budget 435352, 23400 under; the decision record `records/scale-m3p1-2026-10-02-6578925.md`: every sample's class on every tier, tier c's outcomes, the shared strings table held at 46.2 bytes per unique item on i686 (carried to part 5), the VML and drawing whole reads (part 6), the runner and record-script carries (part 7); tier c `--cells` streams 125829220 cells under 8 MB on both targets; part 4d's two samples rerun byte-identical to 4d's; no open verdict moved; verified by planning from primaries with its own build, its own size probes at three commits reproducing every figure, its own read runs and sample reruns, its own scan; `specs/0023` "Planning verification and rulings on P3") |
| Close | close-out push after P3, carrying ab5b3aa and this part's commits, the scan script's key check and the ci-run newline, the previous-record pattern naming ab5b3aa, one independent check; Milestone 2's Excel check run or deferred again before it | DONE 2026-10-02 (doc 21b89bb pushed, origin/main at 21b89bb, 11 commits, CI 37005285780 success, scan PASS with the key line, record 3cbd6fd local; decision 6 deferred a sixth time; verified by planning from git, gh and its own scan; `specs/0023` "Planning verification and rulings on the close-out") |

### Part 2: rich text (`specs/0024-rich-text.md`, DONE and PUSHED 2026-10-03)

Approved by the maintainer 2026-10-02 ("A on all"): runs with
their own font in a cell, written as an inline string (never the
shared strings table) and read back from inline strings and from
the table, and in a comment, as openpyxl's `CellRichText`; the
run's font is `styles::Font` with `name` as `rFont`, the five
`rPr`-only children not modelled; `Cell::runs`,
`SharedStrings::runs_of` and `CommentEntry::runs` under the
feature, with the contract, measured on tier d, that an item
without runs costs nothing more; `add_rich_comment` beside
`add_comment`. Feature `richtext`, off by default, two probes
(the feature alone, and with `comments`); planning's estimate
30000 to 45000 section bytes for the feature and 5000 to 15000
for the pair. The corpus has rich runs in 8 workbooks' tables and
5 workbooks' comments, no inline string; the samples write the
corpus's first case of each. The Milestone 2 Excel check is due
before this part's close-out push (decision 7).

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the feature, `Run`, `RichText`, `Value::Rich`, the inline write, `Cell::runs`, `SharedStrings::runs_of`, the `rPr` reader, `--richtext-sample` on the corpus | DONE 2026-10-02 (doc e11d0f5, carry e51ec15, crate 1174bf1, tool 7d2f40d, record c53bbb3; 27 of 27 ok on both labels, LibreOffice ok, every entry identical but the edited worksheet; 18 crate and 2 tool tests; planning's own rerun byte-identical; planning's corpus fact corrected, P1 verification ruling 1) |
| P2 | `add_rich_comment`, `CommentEntry::runs`, `--richcomment-sample` on the corpus | DONE 2026-10-03 (doc fc24238, empty runs 8a268f2, crate 17d1e6b, tool dc33002, record ea4a090; 27 of 27 ok on both labels, LibreOffice ok; 12 tests; planning's own rerun byte-identical; planning's comment-run fact corrected, P2 verification ruling 1) |
| P3 | the catalogue and limits; the two probes and the size record; part 1's three read runs repeated with the no-growth contract; both samples rerun byte-identical | DONE 2026-10-03 (doc 7e14eab, carry 7c1e11e, catalogue d07793a, probes 0c3e4fa, record b018e52; `richtext` costs 12672 section bytes, the pair with `comments` 14820, the core grew 368, budget 435720, 23032 under; tier d peaks within 104 kB of part 1's on both labels, tier c within 200 kB; both samples byte-identical to P1's and P2's runs; planning's own probes and runs reproduce every figure) |
| Close | close-out push after P3, carrying 3cbd6fd and this part's commits, the previous-record pattern naming 3cbd6fd, one independent check; the Milestone 2 Excel check run or deferred again before it | DONE 2026-10-03 (doc 50d9e70 pushed, origin/main 21b89bb..50d9e70, 17 commits, CI 37118021913 success, record 21f1434 local; the Excel check deferred a seventh time, decision 7) |

### Part 3: sheet and print settings (`specs/0025-sheet-and-print-settings.md`, DONE and PUSHED 2026-10-04)

Approved by the maintainer 2026-10-03 ("A on all"): the tab colour
and fit-to-page under `sheetPr`; zoom, gridlines, headings and the
view kind on the first `sheetView`; print options, margins and
page setup; the six header and footer texts as the format's code
strings with a `section` builder; the print area and print titles
as the sheet-scoped built-in names in Excel's form; a sheet-level
autofilter with its hidden `_FilterDatabase` name; each element
replaced with only its modelled attributes changed and the rest
kept (a `pageSetup`'s printer-settings `r:id` survives), or created
in schema position. Writes only; the typed read-back is part 5's.
Feature `sheetprops`, off by default, two probes (the feature
alone; with `metadata` for the three name-writing calls);
planning's estimate 18000 to 30000 section bytes for the feature
and 3000 to 8000 for the pair, priced from part 2 since no part is
created. The corpus carries every element on most of its 95
sheets, 41 root-child orders all in the schema table, and the
three built-in names. The Milestone 2 Excel check is due before
this part's close-out push (decision 7).

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the feature, the settings pass with its attribute merge, `set_tab_color`, `set_sheet_view`, `--sheetview-sample` on the corpus | DONE 2026-10-03 (doc e5f998a, carry bf5e877, crate 4f53d49, tool 9db5f4f, record 138fe80; 27 of 27 ok on both labels, LibreOffice ok, every entry identical but the edited worksheet, the kept attributes byte-identical; 17 crate and 2 tool tests; planning's own rerun byte-identical) |
| P2 | `set_print_options`, `set_page_margins`, `set_page_setup`, `set_header_footer` with `section`, `--pagesetup-sample` on the corpus | DONE 2026-10-03 (doc cf70ada, carry 688d059, crate 791a794, tool a6cf46f, record 25f9bba; 27 of 27 ok on both labels, LibreOffice ok, the printer-settings `r:id` kept on every sheet that had one; 27 crate and 2 tool tests; planning's own rerun byte-identical) |
| P3 | `set_print_area`, `set_print_titles`, `set_auto_filter` with the built-in names through the `metadata` writer, `--printarea-sample` on the corpus | DONE 2026-10-03 (doc 4359bdb, carry d4f9456, crate 8faf378, tool 5c1a6b3, record 38d4008; 21 of 27 ok on both labels and 6 refused by the table overlap the survey predicted, LibreOffice ok on the 21, the calc chain kept; planning's own rerun byte-identical with the same six refusals) |
| P4 | the catalogue and limits; the two probes and the size record with a cause per probe; the three samples rerun byte-identical | DONE 2026-10-03 (doc 2b19bb6, carry f59c78c, catalogue b0c6bd8, probes 01b14c0, record 6d57689; `sheetprops` costs 28148 section bytes, the pair with `metadata` 34468, the core grew 736, budget 436456, 22296 under; the three samples byte-identical to P1's, P2's and P3's runs; planning's own probes and reruns reproduce every figure) |
| Close | close-out push after P4, carrying 21f1434 and this part's commits, the previous-record pattern naming 21f1434, one independent check; the Milestone 2 Excel check run or deferred again before it | DONE 2026-10-04 (doc a8dd1d3 pushed, origin/main 50d9e70..a8dd1d3, 22 commits, CI 37223570403 success, record 0caa390 local; the Excel check deferred an eighth time, decision 7) |

### Part 4: outline grouping and row and column styles (`specs/0026-grouping-and-line-styles.md`, DONE and PUSHED 2026-10-05)

Approved by the maintainer 2026-10-04 ("A on all"): outline levels
1 to 7 on rows and on columns with the hidden flag of the group
call, the collapsed flag by a call on the row or column the caller
names (openpyxl's shape, nothing automatic), the sheet's outline
maximums raised and never lowered, the summary position under
`sheetPr`'s `outlinePr`; a style on a whole row (`s` with
`customFormat`) or a whole column (`style`), the index checked at
the write as a cell's is; column ranges through part 3's split
rule then coalesced, so a group is one `col` element as Excel
writes it; every attribute kept through a shift as it already
travels. Writes only; the typed read-back is part 5's. The feature
`rowcol` grows and its probe grows; planning's estimate 10000 to
20000 section bytes of growth, priced from parts 2 and 3. The
corpus holds no grouping at all (every level 0), so the samples
write the first witnesses; row styles sit in 6 workbooks and
column styles in 26. The Milestone 2 Excel check is due before
this part's close-out push (decision 7).

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the settings pass gated under either feature, `group_rows`, `group_columns` with the coalescing, `set_row_collapsed`, `set_column_collapsed`, `set_outline_summary`, the `sheetFormatPr` maximums, `--group-sample` on the corpus | DONE 2026-10-04 (doc 56ebb3f, crate 7b3d9b2, tool 7693267 and 93a92d5, record d26c470; 27 of 27 ok on both labels, LibreOffice ok, five rows edited in place and the rest created; 22 crate and 3 tool tests; planning's own rerun byte-identical) |
| P2 | `set_row_style`, `set_column_style` with the index check at the write, `--linestyle-sample` on the corpus | DONE 2026-10-04 (doc c7c9e39, carry d1278c3, crate 9d266f0, tool 8416a1e, record 4a23c4b; 27 of 27 ok on both labels, LibreOffice ok, W and the styles part changed and every other entry identical; 13 crate and 2 tool tests; planning's own rerun byte-identical) |
| P3 | the catalogue and limits; the rowcol probe grown and the size record; both samples rerun byte-identical | DONE 2026-10-05 (doc dcdb50d, carry 34ac951, catalogue de5c09f, probe ef392d4, record 23f9589; `rowcol` grew 22888 section bytes to 58136, the core grew 472, budget 436928, 21824 under; both samples byte-identical to P1's and P2's runs; planning's own probes and reruns reproduce every figure) |
| Close | close-out push after P3, carrying 0caa390 and this part's commits, the previous-record pattern naming 0caa390, one independent check; the Milestone 2 Excel check run or deferred again before it | DONE 2026-10-05 (doc 6e03f98 pushed, origin/main a8dd1d3..6e03f98, 17 commits, CI 37307047253 success, record d64b4e8 local; the Excel check deferred a ninth time, decision 7) |

### Part 5: reading back as typed values (`specs/0027-read-back.md`, DONE and PUSHED 2026-10-05)

Approved by the maintainer 2026-10-05 ("A on all"): a cell's
format resolved on request through `cellXfs` into the writers'
own types (font, fill, border, alignment, protection, number
format), the styles part parsed typed once and `Cell` unchanged;
column formats as a list and row formats as a streaming iterator,
part 4's levels, flags and styles among them; a worksheet's
frozen pane, merges, hyperlinks, protection and every setting part
3 writes read in one pass; what the writer types cannot hold (a
gradient fill, a split pane, eight minor attributes) read as None
or read past, each case named in the catalogue, so every value
read is one the writer accepts, which a round-trip sample proves
on the corpus; the shared strings table's hold stated with the
scale record's figures and a streaming item iterator added beside
it, measured on tier d. Reads only. Feature `read`, off by default,
one probe, the writers' types lifted under either feature;
planning's estimate 35000 to 50000 section bytes, priced from
parts 2 to 4. The Milestone 2 Excel check is due before this
part's close-out push (decision 7).

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the feature, the font reader shared with part 2, the fill, border, alignment and protection readers, `Styles`'s typed fields, `CellStyle` and `cell_style`, `--roundtrip-sample` on the corpus | DONE 2026-10-05 (doc 6d49977, carry fbb171e, crate c019afa, tool 8d4d468, phase e727b25, record 4264cd0; 27 of 27 ok on both labels, every one of W's 362 used cell formats read and written back equal, LibreOffice ok; 18 crate and 2 tool tests; planning's own rerun byte-identical) |
| P2 | `columns`, the `rows` iterator, `sheet_settings` in one pass with `HyperlinkEntry`, `shared_string_items`, `--read-sample` on the corpus and tier d's `--items` under the cap | DONE 2026-10-05 (doc 35d1bab, carry 712d53d, crate bd02ce4, tool 6ec0055, record 9d89487; 27 of 27 read on both labels with every entry identical, every count equal to the survey's; tier d's 4800000 items walked at 7720 and 7536 kB peak against the held table's 266288 and 216572; 15 crate and 2 tool tests; planning's own reruns byte-identical) |
| P3 | the catalogue and limits; the probe and the size record; both samples rerun byte-identical | DONE 2026-10-05 (doc 71d338e, carry afc6a91, catalogue 6b531f6, probe 6b6125b, record bec28eb; `read` costs 51972 section bytes, the core grew 0, budget 436928, 21824 under; both samples byte-identical to P1's and P2's runs; planning's own probes and reruns reproduce every figure) |
| Close | close-out push after P3, carrying d64b4e8 and this part's commits, the previous-record pattern naming d64b4e8, one independent check; the Milestone 2 Excel check run or deferred again before it | DONE 2026-10-05 (doc 9d62635 pushed, origin/main 6e03f98..9d62635, 18 commits, CI 37385478857 success, record 6aba171 local; the Excel check deferred a tenth time, decision 7; `specs/0027` "Planning verification and rulings on the close-out") |

### Part 6: images and comments, the second half (`specs/0028-images-and-comments-second-half.md`, DONE and PUSHED 2026-10-08)

Approved by the maintainer 2026-10-05 ("A on all"): an image
removed by its index in `images` with its anchor skipped, its
relationships dropped and its media entry dropped when the removal
walk finds nothing else reaching it, an emptied drawing dropped
whole; an image added on a two-cell or an absolute anchor through
`add_image_at`, `add_image` unchanged; a comment's box (pixels, as
openpyxl) and visibility set on an addition and read back on
`CommentEntry`; `copy_sheet` within a workbook by one streaming
pass over the source (cells, formats, merges, settings, hyperlinks
and, with `comments`, the comments carried; images, charts, tables,
the extension list and local defined names not), behind a new
feature `copy`. The features `images` and `comments` grow in place;
the VML and drawing whole-part holds stated as the scale record's
carry. Planning's estimate 30000 to 45000 section bytes over the
three features, priced from parts 2 to 5. The Milestone 2 Excel
check was deferred an eleventh time at this scoping (decision 9),
due before part 7's close-out push at the latest.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | `CommentBox`, `set_comment_box`, the shape writer's size and visibility, the `shape` read, `--commentbox-sample` on the corpus | DONE 2026-10-06 (2adf953, records 9cec1de, local; the `ClientData` content model confirmed a choice, so `x:Visible` before `x:Row` stands; the read's VML part choice, the `px` rounding and the dangling-relationship refusal accepted as rulings 2, 5 and 6; verified by planning from primaries, its own build, tests on both targets, privacy scan and its own `--commentbox-sample` run agreeing on all 27 on both labels; `specs/0028` "Planning verification and rulings on P1") |
| P2 | `ImageAnchor`, `EditAs`, `add_image_at`, `remove_image` with the relationship and media drops through the walk, `--anchor-sample` and `--remove-image-sample` on the corpus | DONE 2026-10-07 (d3c4a9c, records d69e39c, local; twelve mid-phase rulings: the removal sample's recompressed entries, the empty rels part and the input's empty drawing accepted as the rule's consequences, the run log rerun twice with the host disk full named and its header corrected, the drawing count 38 by name and 37 `wsDr`, the build-pruning rule from P3; verified by planning from primaries, its own build, tests on both targets, privacy scan and its own runs of all three samples agreeing on all 27 on both labels; `specs/0028` "Planning verification and rulings on P2") |
| P3 | the feature `copy`, `copy_sheet` with the pass, the hyperlink relationships, the comment carry and the formula-change rule, `--copy-sample` on the corpus | DONE 2026-10-07 (6a90674, records f73a91b, local; eleven mid-phase rulings: the copy rels part added where the source has a hyperlink or comments relationship, the AlternateContent drop, the replace_part refusal, the idmap count, the bad hyperlink id refusal, the chartsheet-only negative fixture, the constants under copy, one target root with a subdirectory per source tree; verification rulings 1 to 7: the sheetPr and sheetView check gap and the pageSetup wording to part 7, the pre-ruling-10 root-level artifacts pruned at P4; 1109 tests on both targets; 216 of 216 outputs byte-identical to planning's own runs) |
| P4 | the catalogue and limits; the two grown and two new probes and the size record; the four samples rerun byte-identical | DONE 2026-10-08 (doc f54041f, the xml prefix 42b5de8, phase 9cfd92e, record c616900, local; ten mid-phase rulings: the `grpSp` brief withdrawn, the constants, the probes copying in a second package, the `xml` prefix finding fixed in the core (+64, cause not isolated among six commits), the negative shape, the logged order, the podman warning traced to the root mount and fixed by the maintainer, two generator placeholders, the check answer's one path; `images` +27016 above 25000 stated with the log's cause, `comments` +6196, `copy` 13756, the pair 18864, budget 436992, 21760 under; four samples byte-identical to P3's on both labels, the keep 1093 with the one new pair; verified by planning from primaries, its own build, tests on both targets, privacy scan and its own runs of all four samples agreeing on all 27 on both labels; `specs/0028` "Planning verification and rulings on P4") |
| Close | close-out push after P4, carrying 6aba171 and this part's commits, the previous-record pattern naming 6aba171, one independent check; the Milestone 2 Excel check asked again before it | DONE 2026-10-08 (doc 046cfb8 pushed, origin/main 9d62635..046cfb8, 19 commits, CI 37853950682 success, record bafd780 local; the Excel check deferred a twelfth time, decision 9, a hard requirement of part 7's close-out push; `specs/0028` "Planning verification and rulings on the close-out") |

### Part 7: the engineering close (`specs/0029-engineering-close.md`, DONE and PUSHED 2026-10-09)

Scope chosen by the maintainer 2026-10-08 ("Core close only"):
the items carried from parts 1 to 6 (the `specs/0001` rules for
the gate's environment line, one target root per phase, the
`rm -rf` form and the sessions' environment; the harness scripts'
header hashes and `leftover` column; the copy's `sheetPr` and
`sheetView` route; two sentences of `specs/0028` amended; the size
log's cause attribution per probe's own sources), the MSRV policy,
the API review record, the README, and the Milestone 2 Excel check
as a hard requirement of the close-out push. Decisions 1 to 4
taken as A: `rust-version` equal to the pin and checked by the
gate; the emptied relationships part kept; the review a record
only, its changes carried to the next milestone; a short README
whose measured sentences cite records. The xlsx-diff, passthrough
and privacy-gate backlogs stay under "Unscoped". No feature is
added; the core is expected not to move.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the four `specs/0001` rules; `counting.sh`'s `rm -rf` form (self-test); the record script's header hashes and the runner's `leftover` column; part 6 P4's targets pruned; the environment line first in the gate log | DONE (phase bcef9c3, record bdaa890, 2026-10-08) |
| P2 | `sheetPr` and `sheetView` through `check_copy_relationships` with a test; the two `specs/0028` amendments | DONE (phase 8d54741, record d3faae6, 2026-10-09) |
| P3 | `rust-version` and the hygiene agreement check (self-test); the size log with the per-probe cause attribution and the size record; the API review record; part 6's four samples rerun byte-identical; one independent check | DONE (phase 09d2e0f, record ed0a9ef, 2026-10-09; `records/size-m3p7-2026-10-09-09d2e0f.md`, `records/api-review-m3p7-2026-10-09-09d2e0f.md`) |
| P4 | `README.md` with the citation check; the push script's Excel-record check; the close-out scripts generated; the gate's doc-directory rule | DONE (phase ad7cfec, record a199fe4, 2026-10-09; two README sentences corrected in P5) |
| P5 | the Excel list: the formula sample and every gone sample rerun on the host at the closing commit with records, the host keep reopened into an open record, the list staged (`specs/0029` P4 ruling 2) | DONE (phase f39bc7b, record daab8c7, 2026-10-09; 23 reruns recorded as `records/m3p7-p5-*`, `records/m3p7-open-2026-10-09-f39bc7b.md`; the list 3134 rows staged, printarea out) |
| Close | close-out push after P5, carrying bafd780 and this part's commits, the previous-record pattern naming bafd780, base 046cfb8 by its subject, the tip checked by its subject (change 21), one independent check; the Milestone 2 Excel check skipped by the maintainer's decision of 2026-10-09 (deferred a thirteenth time, the push script's Excel-record check removed; `specs/0029` "Planning rulings before the close-out"), the stage kept on disk for the next milestone's first part | DONE 2026-10-09 (doc ebec51d pushed, origin/main 046cfb8..ebec51d, 20 commits, CI 38002814655 success, scan PASS, record 582dd2c local; the Excel check deferred a thirteenth time by the maintainer; `specs/0029` "Planning verification and rulings on the close-out") |

## Milestone 4: release readiness (APPROVED 2026-10-09; part 1 DONE and PUSHED 2026-10-10; part 2 DONE and PUSHED 2026-10-11; decision 1 HOLD, nothing published; the milestone DONE and PUSHED 2026-10-11)

Theme chosen by the maintainer 2026-10-09 ("Release readiness",
the recommended option of four: charts as a second crate, the
backlogs, and embedding into a real consumer were the others).
Two parts, each scoped into phases with its own spec before it
starts, the Milestone 2 ceiling kept for the core, no feature
added. Nothing is published to any registry in this milestone
without a separate maintainer decision at part 2's close-out.
The Milestone 2 Excel check was skipped again by the maintainer
2026-10-09 (the fourteenth deferral): it gates no push in this
milestone, stands as the pending acceptance item of Milestones 2
and 3, and its stage stays on disk; the push script's
Excel-record check (`records/logs/m3p7-p4/negative.log`) is
restored by the part whose push the maintainer makes it gate.

### Part 1: the carries and the API changes (`specs/0030-carries-and-api-changes.md`, APPROVED 2026-10-09, decisions 1 and 2 A; DONE and PUSHED 2026-10-10)

- The Excel list script accepts a run directory with error rows,
  leaves them out and counts them; the printarea sample's six are
  `specs/0025`'s recorded overlap refusals, not a finding.
- The `rm -rf` line-205 question (decision 1 A: the runner refuses
  a reused run directory) and the size log's attribution note
  (decision 2 A: per-file attribution kept, the finer method under
  "Unscoped").
- The API review's six changes applied (five renames, one hide),
  no alias kept, the listing rerun, every sample rerun
  byte-identical, the size record with 0 growth.
- The record commit 582dd2c carried, the push script naming it by
  `^M3 part 7 close-out record`, base ebec51d by its subject.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the list script's error rows; decision 1's sentence in `specs/0001`; "Unscoped" per decision 2 | DONE (phase 37ab66b, record 828b7dc, 2026-10-09; the runner refuses a reused run directory, 267e689) |
| P2 | the six API changes, the workspace following, the negative log's old-name build | DONE (phase 8902f87, record 84c40da, 2026-10-10; five renames c68f350..cce4285; `prefix_len` under `cfg(test)`, P2 ruling 3) |
| P3 | the size log and record (0 growth), the listing rerun, the samples rerun byte-identical, one independent check | DONE (phase 669fa91, record ed4082d, 2026-10-10; `records/size-m4p1-2026-10-10-669fa91.md` 0 of 28 moved; `records/api-review-m4p1-2026-10-10-669fa91.md` 544 keep; 27 sample records byte-identical to Milestone 3's runs) |
| Close | close-out push after P3, carrying 582dd2c, base ebec51d by its subject, the tip by its subject, no Excel-record check | DONE (doc e4ab25b, pushed ebec51d..e4ab25b 17 commits 2026-10-10, CI run 38048068003 success, record 20285f1 local, goes with part 2's push; `specs/0030` "Planning verification and rulings on the close-out") |

### Part 2: the release metadata (`specs/0031-release-metadata.md`, APPROVED 2026-10-10, decisions 2 to 4 A; decision 1 HOLD 2026-10-10; DONE and PUSHED 2026-10-11)

- `CHANGELOG.md` from the specs and the records, under the README's
  citation check extended to `specs/` paths; the crate-level doc
  comment rewritten from the README (decision 3 A); the README and
  the two licence texts in the package (decision 4 A).
- The crate's package metadata (description, readme, repository
  (decision 2 A), keywords, categories, docs.rs all-features) with
  `publish = false` kept; `cargo package --list`, `cargo package`
  and a `cargo publish --dry-run` on a scratch copy recorded; the
  size record with 0 growth; the API listing unchanged at 544.
- Close-out decision 1 for the maintainer, asked by planning after
  P2's verification: publish 0.1.0 or hold. Publishing itself, if
  decided, is a later step with its own spec, which carries two
  items from P2: an `include` list in the manifest so a package
  built outside a git checkout holds `template/_rels/.rels`
  (tested by a `git archive` dry run), and the registry's own
  validation of the keywords and categories, which a dry run does
  not perform.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | `CHANGELOG.md`, the doc comment, the licence copies; the citation check over both files; the prune of part 1's P3 set and the close-out target | DONE (phase d41c390, record 5640bfa, 2026-10-10; README 18 of 18 and CHANGELOG 29 of 29 digit sentences cited; scan denylist form made whole-word, P1 ruling 6) |
| P2 | the manifest metadata, the three packaging runs, the size record, the listing rerun, one independent check | DONE (phase 2c31aad, record 24b927c, 2026-10-10; `records/size-m4p2-2026-10-10-2c31aad.md` 0 of 28 moved; the package 56 files, 522253 bytes, verify build and publish dry run exit 0; finding: a package built outside a git checkout drops `template/_rels/.rels`, carried to the publish step, `specs/0031` P2 rulings 1 and 7) |
| Close | close-out push after P2, carrying 20285f1, base e4ab25b by its subject, decision 1 recorded, no Excel-record check | DONE (doc ac5dc0d, pushed e4ab25b..ac5dc0d 10 commits 2026-10-11, 1568 blobs, 0 findings; CI run 38161194914 success; scan PASS with change 27's whole-word denylist form; pre attempt 1 superseded on the scan's own placeholder check, kept outside the repository; record b0f0b1f local, goes with the next push; decision 1 HOLD, `publish = false` stays; verified by planning from git, `gh`, the three logs and its own scan; `specs/0031` "Planning verification and rulings on the close-out") |

## Milestone 5: charts and read breadth (APPROVED 2026-10-11; part 1 DONE 2026-10-12; part 2 DONE 2026-10-13; part 3 DONE 2026-10-13; part 4 DONE 2026-10-14; part 5 DONE and PUSHED 2026-10-16; part 6 DONE and PUSHED 2026-10-18; milestone DONE and PUSHED 2026-10-18)

Theme chosen by the maintainer 2026-10-11 after Milestone 4's
close-out, from planning's reading of emend against openpyxl: the
editing surface is close to parity, the reading surface and charts
are behind. Decisions typed in the planning session 2026-10-11:
charts are a Cargo feature `charts` of this crate, off by default,
measured in the size record like every other feature (not a second
crate); reading first, then charts; the openpyxl comparison
measurement is part 6; the Milestone 2 Excel check gates no push in
this milestone (it stays the pending acceptance item of Milestones
2 and 3, its stage on disk). The Milestone 2 ceiling is kept for
the core; every addition is behind a feature; each part has its own
spec before it starts and is scoped into phases as before. The
corpus holds 11 workbooks with charts and 4 with pivot tables
(`records/corpus-coverage-2026-09-16.md`).

### Part 1: read breadth A (`specs/0032-read-breadth-styles.md`, APPROVED 2026-10-11, decisions 1 to 3 A; DONE 2026-10-12)

The items `specs/0027` left out of scope: named styles (`cellStyles`
and `cellStyleXfs`), differential styles (`dxfs`) as typed values,
theme and indexed colours resolved to RGB, the default font. The
`read` feature grows; no writer changes; no new feature (decision
1 A); a `dxf` fill with no `patternType` reads as solid (decision 2
A); a colour the file defines no RGB for resolves to `None`
(decision 3 A).

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | `NamedStyle`, `named_style`, `cell_style_xfs` and `cell_styles` on `Styles`, `dxfs` and `differential_styles`, `default_font`; `--namedstyles-sample` with the `dxf` round trip; the prune of Milestone 4 part 2's three target roots | DONE (phase c8b9391, record 06f5cba, 2026-10-11; 27 of 27 ok on both targets, dxf_written and dxf_equal 831, totals the spec's 14 figures; tests 514, 562, 600, 934 both targets; 16 crate tests and 3 tool tests added; verified by planning from the diffs, the five logs, its own build and its own corpus run byte-identical; `specs/0032` "Planning verification and rulings on P1", rulings 1 to 8: the off-feature test and two comment wraps carried to P2, catalogue wording to P3) |
| P2 | the palette and `indexed_colors`, `theme_colors`, `ResolvedColor`, `resolve_color`, `tinted`; `--colors-sample`; the carry from P1 (one off-feature test, two comment wraps) | DONE (phase 70b3e00, record e39d028, 2026-10-12; 27 of 27 ok on both targets, 870 entries identical and nothing changed, the comparison and the swap's proof 27 of 27, tints on other forms 1, totals the spec's 12 figures; tests 515, 584, 622, 956 both targets; 22 crate tests, 1 off-feature carry test and 3 tool tests added; verified by planning from the diffs, the five logs, its own build, its own corpus run byte-identical and its own resolution check 27 of 27; `specs/0032` "Planning verification and rulings on P2", rulings 1 to 8: the long-override test and doc clause and the catalogue reflow carried to P3) |
| P3 | the catalogue, the probe grown, the size record, the API listing rerun, the CHANGELOG bullet, both samples with the release binaries; the carry from P2 (the long-override test and doc clause, the catalogue reflow) | DONE (phase 6f51899, record 2dfca6c, 2026-10-12; `records/size-m5p1-2026-10-12-6f51899.md`: core growth 0, budget 436992 with 21760 under, `read` costs 70044 section bytes; `records/api-review-m5p1-2026-10-12-6f51899.md` 567 rows, 24 new; both samples byte-identical to P1's and P2's runs on both targets; tests 515, 585, 623, 957, workspace 1157; verified by planning from the diffs, the six logs, its own build, its own sample runs and its own three size probes reproducing the record's figures to the byte; `specs/0032` "Planning verification and rulings on P3", rulings 1 to 10) |
| Close | close-out push after P3, carrying b0f0b1f, base ac5dc0d by its subject, `PREV` `^M4 part 2 close-out record`, no Excel-record check | DONE 2026-10-12 (doc c02942e pushed, origin/main ac5dc0d..c02942e, 15 commits, 1630 blobs, 0 findings; CI run 38245207259 success; the post-push scan PASS; record 3f2082e local, goes with part 2's push; verified by planning from git, `gh`, the three logs and its own blob scan; `specs/0032` "Planning rulings on the close-out", rulings 1 to 9) |

### Part 2: read breadth B (`specs/0033-read-breadth-survey.md`, APPROVED 2026-10-12, decisions 1 to 3 A; DONE 2026-10-13)

A workbook survey call naming which part kinds the package holds,
with counts, from the content types and the relationships alone
(`part_kinds`, `sheet_parts`, with the sparkline groups counted by
one streamed pass over a sheet); pivot table definitions read typed
(name, location, source, the fields by axis, the data fields, the
cache's field names); external link targets read typed (the book's
target, sheet names and defined-name count, or the DDE or OLE
identity). The corpus holds 9 pivot tables in 4 workbooks and no
external link part (the coverage record's `external` tag names
external data), so the external link reader is proven on built
packages and on a generated fixture through the binary. Reads
only; the `read` feature grows (decision 1 A); no writer changes.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | `PartKinds`, `part_kinds`, `SheetParts`, `sheet_parts`, the relationship constants; `--kinds-sample`; the prune of part 1's four target roots; the `TMPDIR` line and the `python3 -B` gate check | DONE (phase 3865f30, record 0264870, 2026-10-12; 27 of 27 ok on both targets, 870 entries identical, the tool and the check agree 27, planning's figures met 24 of 24 after ruling 1 (custom_xml 30 in 10 was the right count); tests 515, 594, 632, 966 both targets, workspace 1168; 9 crate tests and 2 tool tests added; the committed scripts call `python3 -I -B`, the gate self-test 50 cases PASS; verified by planning from the diffs, the six logs, its own build, its own corpus run byte-identical and its own privacy scan of 1624 blobs; `specs/0033` "Planning rulings on P1", rulings 1 to 12: the tool test's comment fix carried to P2, the mktemp file check from P2) |
| P2 | the pivot types, `pivot_tables`, "a pivot table cannot be read"; `--pivot-sample`; the carry from P1 (the tool test's comment) | DONE (phase 61d23fc, record c4d67b6, 2026-10-12; 27 of 27 ok on both targets, outputs byte-identical across targets, the tool and the check agree 27, planning's pivot figures met 18 of 18 after ruling 1 (`data_field_flags` 10 in 3 was the right count), kinds 24 of 24; tests 515, 608, 646, 980 both targets, workspace 1184; 14 crate tests and 2 tool tests added; the keep one new pair, 0 changed, open check 0 findings; verified by planning from the diffs, the six logs, its own build, its own corpus run byte-identical and its own privacy scan of 1633 blobs; `specs/0033` "Planning rulings on P2", rulings 1 to 10: the catalogue entry and the counting library's mktemp line to P3) |
| P3 | the external link types, `external_links`, "an external link cannot be read"; `--extlink-sample` with the generated fixture; the catalogue, the probe grown, the size record, the API listing rerun, the CHANGELOG bullet, the three samples with the release binaries | DONE (phase f35de0d, record 38b01c7, 2026-10-13; the phase ran twice: the first independent check found two refusals with no failing test, fixed in the tests-only phase commit and every log rerun; extlinks 0 in 27 workbooks, 9 of 9 figures, the generated fixture read on both targets; kinds 24 of 24, pivots 18 of 18; the three samples with the release binaries byte-identical to P1's, P2's and this phase's runs on both targets; tests 515, 618, 656, 990 both targets, workspace 1196; 10 crate tests and 2 tool tests added; API listing 660 (7 cfg-widened, 93 new); size: core growth 0, budget 436992 (21760 under), the feature `read` 112600, a FINDING above the 110000 line (planning's estimate missed; ruling 1); verified by planning from the diffs, the eight logs, its own build, its own corpus runs byte-identical and its own privacy scan of 1661 blobs; `specs/0033` "Planning rulings during P3" A to C and "Planning verification and rulings on P3" 1 to 6) |
| Close | close-out push after P3, carrying 3f2082e, base c02942e by its subject, `PREV` `^M5 part 1 close-out record`, no Excel-record check | DONE 2026-10-13 (doc 742051b pushed, origin/main c02942e..742051b, 19 commits, 1680 blobs, 0 findings; CI run 38392560343 success; the post-push scan PASS; record 017e480 local, goes with part 3's push; verified by planning from git, `gh`, the three logs and its own blob scan; `specs/0033` "Planning rulings on the close-out") |

### Part 3: charts read (`specs/0034-charts-read.md`, APPROVED 2026-10-13, decisions 1 to 3 A; DONE 2026-10-13)

The feature `charts`, off by default, measured by a new probe: per
worksheet, each chart's anchor, its plots (kind, bar direction,
grouping, scatter style, series with their references, names,
cached point counts, solid fill and flags), its title, axes (kind,
position, title, number format, bounds), legend, pivot source and
style, read from the drawing and chart parts (`Package::charts`).
The corpus holds 26 charts in 11 workbooks (28 plots, 38 series,
44 axes, 5 pivot charts), every one reached from a worksheet's
drawing; chartsheets (corpus 0) and the cached values stay out
(decisions 2 and 3 A). Proven by reading every chart in the corpus
and writing each workbook back byte-identical.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the feature, the drawing walk shared with `images`, `ChartEntry`, `ChartAnchor`, `Chart`, `Plot`, `Series`, `Data` and the enumerations, `charts`, "a chart cannot be read"; `--charts-sample`; the prune of part 2's four target roots | DONE 2026-10-13 (doc 266c410, crate c566a01, tool 54bf510, record cf8f145; 27 chart tests, 11 of 11 plants caught; 26 charts in 11 workbooks, series 38, 27 of 27 equal to the kinds sample; scan PASS; rulings E to L) |
| P2 | `ChartAxis`, `LegendPosition`, the `axes`, `legend`, `pivot_source` and `style` fields; the sample's remaining fields; the catalogue | DONE 2026-10-13 (doc d148ad1, crate 60ad190, tool 27569d2, record 1e0b03c; 38 chart tests, 14 of 14 plants caught; axes 44, legends 15, pivot charts 5, 22 of 22 figures met; scan PASS; rulings M to Y) |
| P3 | `probe-emend-charts-write-xml`, the size record, the API listing rerun, the CHANGELOG bullet, the samples with the release binaries | DONE 2026-10-13 (doc 91c1cee, carry 7d2640e, probe 7711ea2, CHANGELOG d74ea74, record 4b410c6; charts cost 44428, core growth 0, images movement 0; API 718 items, 58 new; 40 chart tests, 16 of 16 plants caught; both samples byte-identical to their records; scan PASS; rulings Z to AC) |
| Close | close-out push after P3, carrying 017e480, base 742051b by its subject, `PREV` `^M5 part 2 close-out record`, no Excel-record check | DONE 2026-10-13 (doc 93a14ef pushed, origin/main 742051b..93a14ef, 15 commits, 1714 blobs, 0 findings; CI run 38493970300 success; the post-push scan PASS; record 7a3fec6 local, goes with part 4's push; verified by planning from git, `gh`, the three logs and its own blob scan; `specs/0034` "Planning rulings on the close-out", AD and AE) |

### Part 4: charts write, the first set (`specs/0035-charts-write-first-set.md`, APPROVED 2026-10-13, decisions 1 to 3 A; DONE 2026-10-14)

Bar, column, line, pie, scatter and area charts from cell ranges,
with title, axis titles and legend, anchored like images through
the drawing writer of `specs/0022` and `specs/0028`
(`Package::add_chart`, the feature `charts`, decision 1 A); the
references are formula strings written verbatim (decision 2 A) and
no cache is written (decision 3 A); every written chart is read
back by part 3's `Package::charts` and compared; LibreOffice opens
every corpus output.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the regated drawing writer shared with `images`, `NewChart`, `NewPlot` (`Bar`, `Line`), `NewSeries`, `NewSeriesName`, `add_chart` with the texts, references, applicability, anchor and size rules, the chart part, the `graphicFrame`, the four messages; `--add-chart-sample`; the image samples byte-identical to their records | DONE 2026-10-14 (doc 5cfb51e, crate 4c36f05, tool 981c00c, record d8d4e66; six feature sets on both targets 515/573/551/611/676/1050, workspace 1261; `--add-chart-sample` 27/27 both targets, a drawing created in 17 workbooks and appended to in 10, every chart read back equal; the image, anchor and remove-image samples byte-identical to their records; six P1 rulings A to F (one drawing-additions list under either feature, image removal with a chart addition refused, the check order, six feature sets, the strict sheet's drawing form, the escaping rules) and three after verification G to I; verified by planning from primaries, its own privacy scan, its own build and tests on both targets, clippy and doc, its own sample runs byte-identical to the phase's on 27 of 27 on both targets; records `records/m5p4-p1-2026-10-14-981c00c.md` and `.txt`, open record `records/m5p4-open-2026-10-14-981c00c.md` and `.txt`; `specs/0035` "Planning verification of P1") |
| P2 | the `Pie`, `Scatter` and `Area` variants; `--chart-kinds-sample`; the catalogue | DONE 2026-10-14 (doc 7eca1a0, crate e7827bb, tool e86da06, record 6d736e5; six feature sets 515/578/551/616/681/1055, workspace 1269; `--chart-kinds-sample` 27/27 both targets, 162 of 162 charts read back with their kinds; the add-chart rerun byte-identical to P1's; rulings J and K mid-phase, L to N after verification (the final scan's first attempt accepted under `specs/0034` ruling Y); verified by planning from primaries, its own privacy scan, its own build and tests on both targets, clippy and doc, its own sample runs byte-identical to the phase's on 27 of 27 on both targets; records `records/m5p4-p2-2026-10-14-e86da06.md` and `.txt`, open record `records/m5p4-open-2026-10-14-e86da06.md` and `.txt`; `specs/0035` "Planning verification of P2") |
| P3 | the probe's write, the size record (the writer's cost as the probe's movement over 44428), the API listing rerun, the CHANGELOG bullet, the samples with the release binaries | DONE 2026-10-14 (doc e87e1ba, probe 0cdcb08, CHANGELOG 574a77d, record 27a9a80; FINDING: the writer costs 40732 section bytes, above the 40000 line and planning's 21000 to 34000 estimate (ruling O; the `charts` feature 85160 over the core, its estimate 65000 to 79000), recorded and not fixed, part 5 priced from it and deciding whether the emission is compacted; the `images` feature moved by 384 (accepted to 1500), the core by 0, every other probe by 0; the budget 436992, 21760 under the ceiling, unchanged from part 3; the API listing 734 rows, 1 gone and 17 new, every verdict keep; the CHANGELOG's 33 digit sentences all cited; the three samples with the release binaries byte-identical to P2's and part 3's runs, 27/27 both targets; open record 1201 pairs, 0 findings; rulings O and P mid-phase, Q to T after verification (the negative logs' gap on the shared drawing messages carried to part 5 P1, the probe's doc comment carried to part 5's first code commit); verified by planning from primaries, its own privacy scan, its own build and tests on both targets, clippy and doc, its own probe builds reproducing the size log's four figures and hashes, its own sample runs byte-identical to the phase's on 27 of 27 on both targets; `records/size-m5p4-2026-10-14-574a77d.md`, `records/api-review-m5p4-2026-10-14-574a77d.md`, `records/m5p4-p3-<sample>-2026-10-14-574a77d.md` and `.txt` for the three samples, open record `records/m5p4-open-2026-10-14-574a77d.md` and `.txt`; `specs/0035` "Planning verification of P3") |
| Close | close-out push after P3, carrying 7a3fec6, base 93a14ef by its subject, `PREV` `^M5 part 3 close-out record`, no Excel-record check | DONE 2026-10-14 (doc 934d2e1 pushed, origin/main 93a14ef..934d2e1, 14 commits, 1751 blobs, 0 findings; CI run 38600594786 success; the post-push scan PASS; record d2c14d8 local, goes with part 5's push; verified by planning from git, `gh`, the three logs and its own blob scan; `specs/0035` "Planning rulings on the close-out", U and V) |

### Part 5: charts write, the second set (`specs/0036-charts-write-second-set.md`, APPROVED 2026-10-14, decisions 1 to 3 A; DONE and PUSHED 2026-10-16)

Doughnut, radar, bubble, stock and 3D variants, combined charts
with an optional secondary value axis (decision 2 A), data labels,
series colours and number formats, through the same `add_chart`
and its grown types; `remove_chart`, taking with the chart part
every part its relationships alone reach (decision 3 A); part 4's
emission compacted first, outputs byte-identical (decision 1 A),
the writer's cost judged against 48000 over part 4's 1055368.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the compaction (the `val` element helper, the shared axis tail; the five sample reruns byte-identical; the probe figure printed by the gate); `Doughnut`, `Radar`, `Bubble`, `Stock`, `NewSeries::sizes`; `--chart-more-sample`; the part 4 carries (rulings Q, R, T) | DONE 2026-10-14 (doc b60d4f5, compaction 5d8ce24, crate 01a5eba, tool ad48df3, record e86561a; the compaction saved 348 section bytes in development, not planning's 3000 to 8000 (ruling A; planning's attribution by symbol: the emission inlined into `write_to` about half of part 4's 40732, the regated drawing writer a quarter); the probe at 1055684, 316 over part 4's, told; six feature sets 515/585/551/623/688/1062, workspace 1279; `--chart-more-sample` 27/27 both targets, 4 kinds read back in each workbook; the five reruns byte-identical to their records; N3 shows the two-drawing message through the binary (part 4 ruling Q); rulings A to D mid-phase, E after verification; verified by planning from primaries, its own privacy scan, its own build and tests on both targets, clippy and doc, its own sample runs byte-identical to the phase's on 27 of 27 on both targets for the six samples; records `records/m5p5-p1-2026-10-14-ad48df3.md` and `.txt`, open record `records/m5p5-open-2026-10-14-ad48df3.md` and `.txt`; `specs/0036` "Planning verification of P1") |
| P2 | `Bar3D`, `Line3D`, `Pie3D`, `Area3D`, `view3D`, the series axis; `NewCombined` with the secondary axis pair; `--chart-3d-sample` | DONE 2026-10-15 (doc 4ef3a8c, crate a5823b7, tool e576023, record 1bb8e02; the crate commit a rewrite of a3deead after independent check 1 found the series axis written inline rather than through the shared axis tail (ruling I; the logs rerun at the new phase commit); the probe at 1058244, 2876 over part 4's, told; six feature sets 515/599/551/637/702/1076, workspace 1296; `--chart-3d-sample` 27/27 both targets, 5 kinds read back in each workbook, 135 added chart parts read by planning against the spec's `view3D`, `serAx` and secondary-pair forms; the chartmore rerun byte-identical to P1's; N4 shows the cannot-be-read message through the binary; rulings F to J mid-phase, K after verification; verified by planning from primaries, its own privacy scan, its own build and tests on both targets, clippy and doc, its own sample runs byte-identical to the phase's on 27 of 27 on both targets for both samples; records `records/m5p5-p2-2026-10-15-e576023.md` and `.txt`, open record `records/m5p5-open-2026-10-15-e576023.md` and `.txt`; `specs/0036` "Planning verification of P2") |
| P3 | `NewLabels`, `LabelPosition`, series colours, axis and label number formats; `remove_chart` with its two messages; `--chart-labels-sample`, `--remove-chart-sample`; the catalogue | DONE 2026-10-15 (doc 67aaa4f, crate 784e3c0, tool 4836d14, record 17cb3c6; the crate and tool commits a lint rewrite of 250caaa and cfb83cc before any logged run (ruling P); the probe at 1071980, 16612 over part 4's and 13736 over P2's, told, judged in P4; six feature sets 515/617/551/656/720/1095, workspace 1319; both samples 27/27 on both targets, host and i686 outputs byte-identical; the removal's 4 kept charts removed with their rels, style and colours parts, 2 empty drawings dropped, the rest identical or recompressed (ruling M); N5 shows the reader's cannot-be-read message through the binary with a control, the two new messages unreachable (ruling N); the keep 1435 = 1278 + 157, 0 changed; open record 1435 pairs, 0 findings; rulings L to N mid-phase, O to Q after verification (three documentation gaps and one test carried into P4's first code commit, the self-closing drawing root noted for part 6); verified by planning from primaries, its own privacy scan, its own build and tests on both targets, clippy and doc, its own reading of the 54 added chart parts and the 27 removal rows in the run directories, its own sample runs byte-identical to the phase's on 27 of 27 on both targets; records `records/m5p5-p3-chartlabels-2026-10-15-4836d14.md` and `.txt`, `records/m5p5-p3-removechart-2026-10-15-4836d14.md` and `.txt`, open record `records/m5p5-open-2026-10-15-4836d14.md` and `.txt`; `specs/0036` "Planning verification of P3") |
| P4 | the probe's removal and command-line kind, the size record (the writer's cost over part 4's 1055368, judged against 48000), the API listing rerun with the changed rows, the CHANGELOG bullet, the samples with the release binaries | DONE 2026-10-15 (doc 0bd800d, probe 17eaa26, CHANGELOG a8bfe71, record 6cee4f8; the writer's cost 20264 section bytes over part 4's 85160, under the 48000 line and under planning's 25000 to 43000 estimate after the compaction; the `charts` feature 105424 over the core; the core 0; the `images` feature 11152 SMALLER than part 4's, P3's shared removal, accepted under ruling S and measured across 784e3c0; the budget 436992, 21760 under the ceiling, unchanged; the probe takes a kind index 0 to 12 after the removal, each kind with the options it takes (ruling R), index 0 printing charts 1 and chart_series 3; the API listing 756 rows, 1 gone and 23 new, every verdict keep; the CHANGELOG bullet cited; the five samples with the release binaries byte-identical to their phase runs, 27/27 both targets; open record 1435 pairs, 0 findings; rulings R and S mid-phase, T after verification; verified by planning from primaries, its own privacy scan, its own build and tests on both targets, clippy and doc, its own probe builds reproducing the size log's five figures and hashes, its own probe runs at every kind index, its own sample runs byte-identical to the phase's on 27 of 27 on both targets for the five samples; `records/size-m5p5-2026-10-15-a8bfe71.md`, `records/api-review-m5p5-2026-10-15-a8bfe71.md`, `records/m5p5-p4-<sample>-2026-10-15-a8bfe71.md` and `.txt` for the five samples, open record `records/m5p5-open-2026-10-15-a8bfe71.md` and `.txt`; `specs/0036` "Planning verification of P4") |
| Close | close-out push after P4, carrying d2c14d8, base 934d2e1 by its subject, `PREV` `^M5 part 4 close-out record`, no Excel-record check | DONE 2026-10-16 (doc c4a01cf pushed, origin/main 934d2e1..c4a01cf, 19 commits, 1828 blobs, 0 findings; CI run 38917010492 success; the post-push scan PASS; record 543c418 local, goes with part 6's push; the maintainer's authorisation typed 2026-10-16 in the implementing session after planning's word of 2026-10-15; one independent check at the pre stage and one at the push stage, blocking 0; verified by planning from git, `gh`, the three logs and its own blob scan; `specs/0036` "Planning rulings on the close-out", U to W) |

### Part 6: measurement and close (`specs/0037-measurement-and-close.md`, APPROVED 2026-10-16, decisions 1 to 3 A; DONE and PUSHED 2026-10-18)

The openpyxl comparison: eleven requests, each an existing
`emend-copy` sample and the openpyxl call that makes the same
request, run over the corpus and the scale tiers, openpyxl in a
second pinned image built from the LibreOffice image's base digest
(decision 1 A; neither the host nor that image has openpyxl), each
tool's output compared with its input by the harness and opened in
LibreOffice, wall time and peak RSS recorded beside each other;
tiers a and b for every request, `roundtrip` on c and d, under a
16384 MiB cap (decision 2 A); the size record as Milestone 5's
closing figure with the attributions parts 4 and 5 promised; the
self-closing drawing root accepted as an empty drawing (decision 3
A, `specs/0036` ruling Q); the API review rerun; the README's
feature list and measured sentences brought to Milestone 5's
records, with a comparison paragraph citing the comparison record.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the openpyxl image (Containerfile, rootless build, the image record); `scripts/openpyxl-ops.py` with its test; `scripts/compare-run.sh`; the record script's `openpyxl` label; the negative log | DONE 2026-10-16 (doc 2e36426, ops 310093d, runner 1e29c4e, record 1cdd3ed; the ops and runner commits a rewrite of 05e4b25 and 71aaaee after the scan's pre-check found two chained shapes in the test script, ruling G; the image `localhost/emend-openpyxl:3.1.5-dfsg-2-deb13` id 894b239f, 211283030 bytes, python3-openpyxl 3.1.5+dfsg-2, `records/openpyxl-image-2026-10-16.md`; FINDING: a killed podman client leaves its container running, the runner names and removes it (ruling A), the open check carried to P3; rulings A to G mid-phase, H to K after verification; verified by planning from primaries, its own privacy scan, its own image inspect, its own run of the ops test (76 ok) and its own roundtrip run over the 27 corpus workbooks (27 ok); `specs/0037` "Planning verification of P1") |
| P2 | the 22 corpus runs and the scale runs of decision 2; the record pairs; `scripts/compare-record.py` and `records/compare-m5p6-<date>-<sha>.md`; the open record | DONE 2026-10-18 (doc 09d82ac, trap 8593ba0, start 89ef6eb, compare 5e52758, record d5e73cb; FINDING: the logged run (exit 1, `records/logs/m5p6-p2/run.log` kept as written) failed three of its own checks, each a defect in the run script's checks and not in the measurement (an undefined keep helper, its stderr line, the xml-family check expecting an XML row for openpyxl's re-encoded media); rulings Q and R: the checks fixed and run again over the same runs in `check.log`, 143 of 143 records and 70 of 70 record pairs, no rerun; 27 of 27 outputs per op and tool, every one opened; roundtrip emend 870 identical and 0 dropped against openpyxl 28 and 533; medians 0.10 s and 0.81 s, ratio 8.10 on all eleven ops; openpyxl's container start 0.46 to 0.56 s inside its times; scale every run ok but openpyxl tier c roundtrip cap (137, 320.13 s, 16576384 kB); open record 1435 pairs; scan PASS; rulings L to P mid-phase, Q and R on the failed run, S to V after verification; verified by planning from primaries, its own privacy scan, its own re-derivation of all 105 comparison rows from the run directories; `records/compare-m5p6-2026-10-16-5e52758.md`, the 70 record pairs `records/m5p6-*-2026-10-16-5e52758.md` and `.txt`, open record `records/m5p6-open-2026-10-16-5e52758.md` and `.txt`; `specs/0037` "Planning verification of P2") |
| P3 | the self-closing root accepted (decision 3 A) with its tests and the catalogue; the size record; the API record; parts 4 and 5's samples rerun byte-identical; the CHANGELOG bullet | DONE 2026-10-18 (doc c4b2598, fix ab00735, open-check 940a1ef, ops ef9da37, record c5a529b; the fix and open-check commits a rewrite of e1c4f87 and 6a2c1bb for the count sentence, ruling X; FINDING: the ops script's MemoryError handler could raise a second MemoryError and exit 1 (1 of 6 dry runs), fixed in ef9da37 (ruling Y); FINDING: the first logged negative run refused at entry, its status pattern not taking the test log's API record (change PZ8), fixed and rerun with the fix named (ruling Z); size record `records/size-m5p6-2026-10-18-ef9da37.md`: the core grew 0 section bytes, the images probe 556 and the charts probe 492, budget 436992, 29 probes reproduced at a8bfe71, no finding; API record `records/api-review-m5p6-2026-10-18-ef9da37.md`: 756 items keep, 0 rows differ from part 5 P4's; the seven samples (`records/m5p6-p3-*-2026-10-18-ef9da37.md` and `.txt`) rerun on both targets byte-identical to their records, 27 of 27, charts against both of its records; open record `records/m5p6-open-2026-10-18-ef9da37.md` and `.txt`, 1437 pairs, 0 findings; scan PASS; rulings W to Y mid-phase, Z to AE after verification; verified by planning from primaries, its own privacy scan, its own comparison of the eight record pairs with their predecessors, its own run of the defined-helper check over the regenerated negative script; `specs/0037` "Planning verification of P3") |
| P4 | the README (the `charts` and `read` lines, the measured sentences, the comparison paragraph), the run log's README check; planning's ROADMAP edits in the doc commit | DONE 2026-10-18 (doc aad0b35, README 7355d98 and 9e171da, record b821213; the doc and README commits reworded for the count rule before any log (ruling AG), FINDING: three comparison sentences cited no record, amended (ruling AH); the first four logs superseded by ruling AI (every feature line cites `records/size-m5p6-2026-10-18-ef9da37.md`, images 57620; the samples named as part 5's); FINDING, change RRO: the second run log's undeclared placeholder, caught by the scan's pre-check, the run log run a third time (ruling AJ); two independent checks, the second's gaps ruled AK; verified by planning from primaries, its own privacy scan and its own per-sentence README citation check; `specs/0037` "Planning verification of P4") |
| Close | close-out push after P4, carrying 543c418, base c4a01cf by its subject, `PREV` `^M5 part 5 close-out record`, no Excel-record check; the Milestone 5 heading DONE and PUSHED | DONE 2026-10-18 (doc c970940 pushed, origin/main c4a01cf..c970940, 20 commits, 2014 blobs, 0 findings; CI run 39175897896 success; the post-push scan PASS; record 67c9144 local, goes with Milestone 6 part 1's push; the maintainer's authorisation typed 2026-10-18 in the implementing session after planning's word the same day; one independent check at the pre stage, blocking 0; the first pre-stage scan a recorded finding on a fixture archive read as text, ruling AO; verified by planning from git, `gh`, the three logs and its own blob scan; `specs/0037` "Planning verification of the close-out", AN to AQ) |

## Milestone 6: chart parity and named styles (APPROVED 2026-10-16)

Theme chosen by the maintainer 2026-10-16 during Milestone 5 part 6's
P2 runs, from planning's list of what openpyxl can do that emend
cannot (`specs/0036`, "Out of scope"; `specs/0034` decision 2,
chartsheets out; the styles reader's gradient pass-over). Decisions
typed in the planning session 2026-10-16: every item of that list
is a part of this milestone, taken one part at a time as before
(decision 1); gradient fills written and named-style creation go in
the core within the Milestone 2 ceiling, each phase's size record
proving the fit (decision 2 A); the order is chartsheets, the chart
formatting parts, styles, then measurement and close (decision 3 A);
the milestone ends with a measurement part that reruns the openpyxl
comparison of Milestone 5 part 6 with the new ops (decision 4 A). The
chart parts stay under the `charts` feature, off by default, measured
by the charts probe; the Milestone 2 Excel check gates no push in
this milestone either. Each part has its own spec before it starts
and is scoped into phases as before. The milestone starts after
Milestone 5 part 6's close-out push.

### Part 1: chartsheets (`specs/0038-chartsheets.md`, APPROVED 2026-10-18, decisions 1 to 3 A; DONE and PUSHED 2026-10-19)

`charts` reads the chart parts a chartsheet's drawing holds (refused
today, `specs/0034` P1 ruling B); `add_chartsheet` creates one holding
a written chart; `add_chart` and `remove_chart` take a chartsheet
part; `rename_sheet`, `move_sheet` and `remove_sheet` on a chartsheet
proven; a chartsheet copy out (openpyxl copies worksheets only); the
survey already counts them. openpyxl's `create_chartsheet` and
`Chartsheet.add_chart`. Decisions 1 to 3 A: an `absoluteAnchor` at
0,0 of a required `size`; a second chart on a chartsheet refused;
no `pageMargins` written (openpyxl's form, confirmed on the image).

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the root rule and `CHARTSHEET_CHILDREN`; `charts` on a chartsheet; the chartsheet form; `add_chartsheet`, `add_chart` and `remove_chart` on a chartsheet; the three messages; `--chartsheet-sample`; the tests on built packages; the negative log; the corpus run and the open record | DONE 2026-10-19 (doc 715aa75, core 56b2937, tool f917618, record f948fd9, local; rulings A to C during the phase; verified by planning from primaries, its own privacy scan, the core diff read whole; corpus 27 of 27 on both targets, 35 new keep pairs, open record 1472 pairs, no finding; charts probe 10316 over part 6's record, inside the estimate; one finding, change NWZ, ruling D; rulings D to I, `specs/0038` "Planning verification of P1") |
| P2 | the `chartsheet` op of `scripts/openpyxl-ops.py` with its test; the `package` header's catalogue and limits; the fixture written by the image; the samples on it; P1's corpus run rerun byte-identical | DONE 2026-10-19 (doc 50def41, carry 1117c8e, op aae54ea, limits 2a706dc, record ef645e2, local; ruling J during the phase, the openpyxl fixture refusing a sheet add by `specs/0016`'s no-prefix rule, no crate change; verified by planning from primaries, its own privacy scan, the three diffs read whole; corpus rerun 27 of 27 byte-identical to P1's on both targets, fixture charts read on both, open record 1474 pairs, no finding; rulings K to M, `specs/0038` "Planning verification of P2") |
| P3 | the probe's `add_chartsheet`; the size record (`charts` against part 5's 105424, `images`, `comments` and the core each judged); the API record; the CHANGELOG bullet; the negative log through the binary; P2's runs rerun on the release binaries; the open record | DONE 2026-10-19 (doc 8910350, probe 3a5ea11, changelog 4cc4f2e, record 20d00be, local; verified by planning from primaries, its own privacy scan, the two diffs read whole; `charts` costs 119344, this part's cost 15396 over part 6's record, above the 15000 line, a finding recorded and not fixed, ruling O; `images` and `comments` costs moved -116 and -64, inside 1500, the record's two FINDING lines a generator fault, ruling N; core 1968, accepted; budget 438960, 19792 under; API one row new, `Package::add_chartsheet`; corpus rerun 27 of 27 byte-identical on both targets; open record 1474 pairs unchanged; rulings N to Q, `specs/0038` "Planning verification of P3") |
| Close | close-out push after P3, carrying 67c9144, base c970940 by its subject, `PREV` `^M5 part 6 close-out record`, no Excel-record check | DONE 2026-10-19 (doc 5b8b25a pushed, origin/main c970940..5b8b25a, 15 commits, 2051 blobs, 0 findings; CI run 39285297969 success; the post-push scan PASS; record 6cecae7 local, goes with Milestone 6 part 2's push; the maintainer's authorisation typed 2026-10-19 in the implementing session after planning's word the same day; one independent check at the pre stage, blocking 0; change 48, the scan's no-archive check, ruling R; verified by planning from git, `gh`, the three logs and its own blob scans; `specs/0038` "Planning verification of the close-out", R to T) |

### Part 2: chart kinds and visual formatting (`specs/0039-chart-kinds-and-formatting.md`, APPROVED 2026-10-19, decisions 1 to 4 A; DONE and PUSHED 2026-10-20)

Surface and of-pie written kinds (`NewPlot::Surface`, `Surface3D`,
`OfPie` with its type and split); a series' marker (symbol, size,
colour), line width and dash style; per-point formatting (`dPt`:
a colour, an explosion, a marker). The visual half of
`specs/0036`'s out-of-scope list. openpyxl's `SurfaceChart`,
`ProjectedPieChart`, `Marker`, `LineProperties` and `DataPoint`.
The reader is unchanged (it models `marker` as a flag). Decisions
1 to 4 A: a line width in EMU; the custom split out; a marker on a
3D line refused; a point carries index, colour, explosion, marker.
Estimate 15500 to 25500 under `charts`, a finding above 30000.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | the surface and of-pie kinds, their rules and forms, the surface axes, the new range message; `--chart-surface-sample`; the tests on built packages; the negative log; the corpus run, the Excel-form fixture and the open record | DONE 2026-10-19 (doc 90ad3a1, core 3affd4a, tool 0a55f0b, fix 01a3059, record d638334, local; dry run 2 a finding, LibreOffice timing out at 600 s on the two large workbooks' 130001-point surfaces and of-pies, fixed by ruling A bounding the sample's references to 1000 rows, the timeout kept by ruling B; verified by planning from primaries, its own privacy scan, the forms read in the source; corpus 27 of 27 with lo and lo_in ok on both targets; the Excel-form fixture's plots in schema order, ofPieType first; open record 1565 pairs, no finding; the check's items carried by ruling E, the fixture's kinds ruling F; rulings A to H, `specs/0039` "Planning rulings during P1" and "Planning verification of P1") |
| P2 | markers, lines and points on `NewSeries`; `--chart-format-sample`; the `chart-format` op of `scripts/openpyxl-ops.py` with its test; the catalogue and limits; the openpyxl fixture; P1's corpus run rerun byte-identical | DONE 2026-10-20 (doc c7b9421, core 1637495, tool 121e7f2, op 32e4b70, record bf16a46, local; two generator faults in the dry runs, ruling I; verified by planning from primaries, its own privacy scan, the rules and forms read in the source; corpus 27 of 27 with lo and lo_in ok on both targets, the two large workbooks at 1000 rows by ruling C; P1's run rerun byte-identical 27 of 27; the openpyxl fixture's nine fields the same; open record 1748 pairs, no finding; probe sections 1096868, 5348 over part 1's record, judged in P3; rulings I to M, `specs/0039` "Planning verification of P2") |
| P3 | the probe's three kinds and series options; the size record (`charts` against part 1's 119344, `images`, `comments` and the core each judged by ruling N's form); the API record; the CHANGELOG bullet; the negative log through the binary; P2's runs rerun on the release binaries; the open record | DONE 2026-10-20 (doc 09e5f94, test 905085e, probe 8b2320a, changelog 50a3989, record 959785e, local; the test commit rebuilt by a fixup before any dry run, ruling N; verified by planning from primaries, its own privacy scan, the test and the probe read in the source; charts probe 1097236, +5716 over part 1's record, the cost 125060, below the estimate's floor and not a finding, ruling P; core 0, images and comments 0, budget 438960, 19792 under the ceiling; API 776 rows, 1 gone, 20 new, three rows marked Eq dropped; P2's runs rerun byte-identical 27 of 27 on both targets, the fixtures 4 of 4; open record 1748 pairs, no finding; the size record's stale index clause carried to `specs/0040`, ruling Q; rulings N to S, `specs/0039` "Planning verification of P3") |
| Close | close-out push after P3, carrying 6cecae7, base 5b8b25a by its subject, `PREV` `^M6 part 1 close-out record`, no Excel-record check | DONE 2026-10-20 (doc bcc53a5 pushed, origin/main 5b8b25a..bcc53a5, 17 commits, 2104 blobs, 0 findings; CI run 39449541516 success; the post-push scan PASS; record 2933e3a local, goes with Milestone 6 part 3's push; the maintainer's authorisation typed 2026-10-20 in the implementing session after planning's word the same day; one independent check at the pre stage, blocking 0; changes 49 to 51, the two leftovers of `specs/0038` ruling R corrected; the six older local branches of ruling J deleted by ruling U; verified by planning from git, `gh`, the three logs and its own blob scans; `specs/0039` "Planning verification of the close-out", T and U) |

### Part 3: analytic chart elements (`specs/0040-analytic-chart-elements.md`, APPROVED 2026-10-20, decisions 1 to 4 A; DONE and PUSHED 2026-10-21)

Trendlines (`NewTrendline`: six kinds, forecast, intercept, equation
and R-squared, a name) and error bars (`NewErrorBars`: direction,
kind, fixed, percentage, standard deviation, standard error or custom
references) on a written series; drop lines, high-low lines and
up-down bars on a line, area or stock plot; axis options (`NewAxis`:
`min`, `max`, a logarithmic base, a reversed orientation, major and
minor units, tick label and tick mark skips) and a date category axis
(`NewDateAxis`). The analytic half of `specs/0036`'s list. openpyxl's
`Trendline`, `ErrorBars`, `ChartLines`, `UpDownBars`, `Scaling` and
`DateAxis`. The reader is unchanged. Decisions 1 to 4 A: the five 2D
kinds take trendlines and error bars; the plot lines are booleans on
`NewChart`; the date axis is in; the reader is unchanged. No new
message. Estimate 16500 to 26000 under `charts`, a finding above
31000.

| Phase | Scope | Status |
| --- | --- | --- |
| P1 | `NewTrendline` and `NewErrorBars` on `NewSeries`, their rules and forms; `--chart-trend-sample`; the tests on built packages; the negative log; the corpus run, the Excel-form fixture and the open record | DONE 2026-10-20 (phase commit a22f6d7, record 74b63de, local; 27 of 27 both targets, tests 1376, probe 1103064 told; the Excel-form fixture read; open record 1896 pairs, 148 new, no finding; rulings A to I) |
| P2 | the plot lines and `NewAxis` with `NewDateAxis` on `NewChart`; `--chart-axis-sample`; the `chart-analytic` op of `scripts/openpyxl-ops.py` with its test; the limits; the openpyxl fixture; P1's corpus run rerun byte-identical | DONE 2026-10-21 (phase commit 3736eea, record 65703ee, local; 27 of 27 both targets, P1's run rerun byte-identical, tests 1386, probe 1107076 told; the openpyxl fixture read and compared twelve of twelve; open record 2121 pairs, 225 new, no finding; findings F1 and F2 ruled O and P; rulings J to S) |
| P3 | the probe's options and its one-piece description; the size record (`charts` against part 2's 125060, `images`, `comments` and the core each judged by ruling N's form); the API record; the CHANGELOG bullet; the negative log through the binary; P2's runs rerun on the release binaries; the open record | DONE 2026-10-21 (phase commit 0707236, record 2c2a376, local; charts 135784, a move of 10724 below the estimate, told; API 815 items, NewSeries loses Eq; P2's runs rerun byte-identical 27 of 27 on four labels, the fixture 6 of 6; open record 2121 pairs, none moved; rulings T to W) |
| Close | close-out push after P3, carrying 2933e3a, base bcc53a5 by its subject, `PREV` `^M6 part 2 close-out record`, no Excel-record check | DONE 2026-10-21 (doc 69f4c89 pushed, origin/main bcc53a5..69f4c89, 19 commits; CI run 39613840305 success; record 5d1c302 local, goes with the next push; finding C1 ruled X) |

### Part 4: chart text and in-place edits (`specs/0041-chart-text-and-edits.md`, APPROVED 2026-10-21, decisions 1 to 4 A; IN PROGRESS from 2026-10-21)

Text properties (`NewText`: size, bold, italic, colour, typeface) on
a written chart's title, axis titles, axis labels, data labels and
legend; the chart's `style` value (no style or colour parts,
decision 2 A); a kept chart's title and one series' references
changed in place (`set_chart_title`, `set_chart_series`), every other
byte kept; the reader unchanged. Estimate 12000 to 20500 under
`charts`, a finding above 25000.

| Phase | Scope | Status |
|---|---|---|
| P1 | `NewText` on `NewChart` and `NewLabels`, the rules and forms; `--chart-text-sample`; the tests on built packages; the negative log; the corpus run, the Excel-form fixture and the open record | DONE 2026-10-21 (phase commit f838d65, record 1156042, local; 27 of 27 both targets, tests 1396, probe 1112212 told; the Excel-form fixture read, its style parts kept byte for byte; open record 2173 pairs, 52 new, no finding; findings S1 and S2 ruled C and D; rulings A to D) |
| P2 | `style`; `ChartSeriesEdit`, `set_chart_title`, `set_chart_series` and the in-place walk; `--chart-edit-sample`; the `chart-text` op of `scripts/openpyxl-ops.py` with its test; the catalogue and limits; the openpyxl fixture; P1's corpus run rerun byte-identical | DONE 2026-10-22 (phase commit f229553, record 6203ddc, local; 27 of 27 both targets, kept_edited 4, P1's run rerun byte-identical, tests 1414, probe 1136996 told (29036 over part 3, above the 25000 line, P3 judges); the openpyxl fixture read and compared seven of seven; open record 2215 pairs, 42 new, no finding; findings S1 and S2 ruled H and I; rulings E to I) |
| P3 | the probe's text, style and edit; the size record (`charts` against part 3's 135784, `images`, `comments` and the core each judged); the API record with the added-by fix (`specs/0040` ruling V); the CHANGELOG bullet; the negative log through the binary; the P1 and P2 runs rerun on the release binaries; the open record | PLANNED |
| Close | close-out push after P3, carrying 5d1c302, base 69f4c89 by its subject, `PREV` `^M6 part 3 close-out record`, the pre driver passing HEAD (`specs/0040` ruling X), no Excel-record check | PLANNED |

### Part 5: gradient fills and named styles (PLANNED)

Gradient fills written (`Fill` gains the gradient form the reader
passes over today); `add_named_style` writing `cellStyles`,
`cellStyleXfs` and a cell style's `xfId`; core, within the Milestone
2 ceiling (decision 2 A). openpyxl's `NamedStyle` and
`Workbook.add_named_style`.

### Part 6: measurement and close (PLANNED)

The openpyxl comparison of Milestone 5 part 6 rerun with a chartsheet
op and a chart-format op added; the size and API records; the README
updated to cite the new record; the Unscoped list pruned of what this
milestone delivered.

## Unscoped

Listed so they are not lost. Scoping any of them needs the maintainer's
approval.

- A sheet add on a workbook whose root binds no prefix for the
  relationships namespace: openpyxl declares it on each `<sheet>`
  element, so `add_sheet` and `add_chartsheet` refuse every
  openpyxl-written workbook (`specs/0016`'s rule; found 2026-10-19 on
  Milestone 6 part 1 P2's fixture, `specs/0038` ruling J). The fix
  is a `<sheet>` carrying its own declaration, as openpyxl writes it.

- Size-measurement consumer: scoped into Milestone 1 part 1 P3 as
  `probe-emend-copy` (2026-09-16).
- A machine with desktop Excel for the per-milestone check. Milestone 1
  cannot be accepted without it.
- xlsx-diff backlog: harness findings raised after T1 P6 that do not
  concern a real data flow (a corpus file in T3, or an emend output).
  - The `descriptor` flag compares the bytes after the data raw, so a
    re-deflated entry whose kept descriptor records its new length
    always carries it; comparing the descriptor's form and its
    consistency with the entry's own values would let the flag mean a
    form change alone (Milestone 1 part 1 P2, specs/0008 ruling 7).
  - The zip64 test is rerun only when the gap or size code changes, not
    per phase (T1 P5 gap 3).
  - No test tells the two size branches of `raw_only` apart, since every
    fixture is stored with equal sizes; the method and CRC-32 branches
    are unreachable with zip 8.6.0 (T1 P6 gap 1).
  - The file-order tie handling has no test; the byte builder writes one
    local header per entry (T1 P6 gap 2).
  - The data-overrun test shows no error and exit 0 but not where the
    gap starts (T1 P6 gap 3).
  - The single-field zip64 placeholder fixtures use a field layout the
    zip specification does not describe (T1 P6 gap 4).
  - A tautological precondition in `tests/p6.rs` (T1 P6 gap 5).
- Passthrough backlog (T3 P2 gaps): the runner's `|| die` guards have
  no negative case (the stale-temporary-file removal's was added by
  Milestone 1 part 1 P1, `negative.log` L4); case 12 of the P2
  negative log counts no fixture strings; some guards sit outside the
  reviewed diff hunks; a fixed 1 s wait in `faults.rs`; the size
  record's feature text is literal in the size script.
  - Part 7 P2: the record script checks no row order within an
    entry's `xml` rows; a diff file not of the tool's making stops
    it with a Python exception, not a refusal (specs/0014 P2
    verification ruling 3).
  - T3 P4: the status 1 to 123 `fail` branch of the open check has no
    positive control on the pinned image (every conversion exits 0);
    the record's four-condition section lists every diff-exit-1 row
    under a heading that says failing, to be split per condition;
    `record.log`'s corpus-name count and two minor gate forms keep a
    bare `grep -c` (check 2); the per-run record holds every entry row
    (1797 lines), so Milestone 1 decides whether later records keep
    entry rows only for entries that are not identical and unflagged.
- Privacy gate backlog: findings raised after T2 P0 that do not concern a
  data flow this project has.
  - Committed logs before T0 P6d hold absolute paths under the generic
    account name (86 occurrences in 55 files); accepted as recorded, no
    rewrite (T0 P6d ruling).
  - The narrowed path patterns leave a segment starting with a byte
    outside the path-character class unmatched, and match a bare shape
    followed by a full stop (T0 P6d residue).
  - Unchecked `sed`, `tr` and `cut` exit statuses in two helper
    pipelines of `scripts/privacy.sh` (T2 P0 finding 1).
  - Content findings in archive or compound-file blobs at ordinary paths
    print the path (T2 P0 finding 2).
  - Both fit the gate self-test script task.
- MSRV policy (Milestone 3 part 7).
- The publish step (Milestone 4 part 2, decision 1 held 2026-10-10):
  an `include` list in the crate manifest so a package built outside
  a git checkout holds `template/_rels/.rels`, tested by a `git
  archive` dry run; `publish = false` lifted; the tag; `cargo
  publish` typed by the maintainer in the implementing session; the
  registry's page and its validation of the keywords and categories
  read back (`specs/0031` P2 rulings 1, 4 and 7).
- Charts: Milestone 5 delivered the `charts` feature, a DrawingML
  chart reader and writer (`specs/0034` to `specs/0036`), measured
  against openpyxl in part 6. What openpyxl writes that emend does
  not (chartsheets; surface and of-pie kinds, markers, lines and
  per-point formatting; trendlines, error bars and axis scaling;
  text properties, chart styles and in-place edits) is Milestone
  6's scope, approved 2026-10-16. Nothing of charts stays unscoped
  beyond that milestone; a second crate is no longer considered.
- README, where every fidelity, speed or size sentence cites a record.
