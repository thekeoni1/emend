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
  default.
- Licence: MIT OR Apache-2.0, copyright holder "emend contributors".
- GitHub: `thekeoni1/emend`, private. Every push passes the privacy gate
  (T0 P6) and needs planning's word plus the maintainer's authorisation
  in the implementing session. Commit messages carry no session links.

## Milestone 1: greenfield core (parts 1 to 3 DONE and PUSHED; part 4 APPROVED 2026-09-22; parts 5 to 8 PROPOSED)

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
| P1 | `drop_calc_chain` in the crate with `--drop-calc-chain` calling it; `Value`, `set_cell`, `clear_cell`, the worksheet rewrite in `write_to`, the seven messages; the corpus run `--drop-calc-chain` against part 2 P3's expectations | PROPOSED |
| P2 | `--edit-sample`; the corpus run with the run log's `expat` cell comparison | PROPOSED |
| P3 | `--edit-formula-sample`; the catalogue and Limits; `probe-emend-cells-write-xml` and the size record against the ceiling; the negative log through the binary | PROPOSED |
| Close | close-out push after P3, as part 3's, carrying b58e894 | PROPOSED |

## Unscoped

Listed so they are not lost. Scoping any of them needs the maintainer's
approval.

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
- MSRV policy.
- README, where every fidelity, speed or size sentence cites a record.
