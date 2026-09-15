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

### T0b: continuous integration (`specs/0003-ci.md`), DONE 2026-09-15

Approved by the maintainer on 2026-09-15. Ran after T2 P0 and before T1.

| Phase | Part | Status |
|-------|------|--------|
| P1 | `check.sh --ci` (steps 1 to 7, privacy skipped); workflow with a read-only token, a SHA-pinned checkout and the pinned toolchain; a green run recorded for a pushed commit | DONE (71c44b8 to ec7bf8c; pushed 401a2f7; run 34977885012 success) |
| P2 | Prove CI fails: a throwaway branch with a formatting error, the red run recorded, the branch deleted | DONE (41a99a1 to c053374; run 34991440664 failure on the deleted branch; main unchanged at 401a2f7) |
| Close | Push the local T0b P1 and P2 record commits and the P2 rulings, using the T0 close-out steps; record the CI run on the pushed commit | DONE (pushed 199b468; run 34993434019 success; record 62c4e80 local) |

### T1: xlsx-diff, the fidelity harness (`specs/0004-xlsx-diff.md`)

| Phase | Part | Status |
|-------|------|--------|
| P1 | `zip` 8.6.0 pinned with the pure-Rust deflate backend; lib and bin split; `entry` rows with indexes, methods, sizes and CRCs; statuses `identical` (raw bytes equal), `dropped`, `added` and the provisional `differs`; name escaping; tests on generated zips | DONE (8356f9a to 6ccae09) |
| P2 | Streaming content comparison through fixed buffers: `recompressed` vs `changed`, first and last differing offset, differing-byte count, all `u64`; memory bound shown; the short-stream error tested | DONE (a3166a5 to 24f4ef0) |
| P3 | All `archive` rows (counts, order, comment, offsets, totals, `files_identical`), the six flags, exit status 0/1/2 as specified; column 14 as the first differing offset for different lengths | DONE (f0adb45 to 827766a) |
| P4 | No-leak sentinel test on stdout and stderr; the `local` flag; a prefix-only `files_identical` test; zip64 entry over 4 GiB uncompressed compared by streaming on i686 (ignored by default, run explicitly and logged) | DONE (564af3b to b5180be) |
| Close | Push T1 P1 to P4 with their records, after P4 (maintainer decision); record the CI run | DONE (pushed 83c9852; run 35017518231 success; record 343e91f local) |
| P5 | Flag completeness: `local` narrowed to local-only differences, `attrs` with internal attributes and disk number, `descriptor` with the bytes after the data; exit 0 then covers every entry byte from local header to next header; records go with the next push | READY |

### T2: corpus (runs alongside T1; `specs/0005-corpus.md`)

Private, in `/home/dev/emend-testing`. Nothing from it is committed. A
separate CORPUS session generates and downloads files (P2 to P4,
maintainer decision 2026-09-15); the maintainer supplies P5 and P6; the
implementing session does P1 and P7.

| Phase | Part | Status |
|-------|------|--------|
| P0 | Privacy gate corpus-era additions: Zone.Identifier files, OLE2 and other archive signatures, both forms of patterns 10 and 11 in the self-test, checked pipeline exits (`specs/0002-gate-corpus-additions.md`); runs before T1 and before any corpus file | DONE (9324e64 to 0870c36) |
| P1 | Manifest format (in the spec) and two scripts in the repo, `corpus-add.sh` and `corpus-coverage.sh`, that print only ids, producers, sources, tags and counts; after the corpus session's first report | READY |
| P2 | Corpus session: LibreOffice container pinned by image digest, version recorded, headless conversion smoke test on a generated fictional file | ACTIVE |
| P3 | Corpus session: LibreOffice exports with fictional content, generated as flat ODS by a script and converted headless, covering the tags LibreOffice can produce | ACTIVE |
| P4 | Corpus session: Microsoft's public Excel templates added, with sources noted in the private manifest | ACTIVE |
| P5 | Maintainer: about 5 Google Sheets exports covering the tag list | PLANNED |
| P6 | Maintainer: desktop Excel files for pivots, macros (.xlsm), threaded comments and slicers, plus any own workbooks | PLANNED |
| P7 | Coverage check: every tag covered, or gaps listed for a planning ruling on starting T3 anyway | PLANNED |

### T3: passthrough baseline

| Phase | Part | Status |
|-------|------|--------|
| P1 | Subprocess isolation (timeout, panic, abort, memory) and the zip raw-copy passthrough run over the corpus on host | PLANNED |
| P2 | i686 musl runs; stripped size of minimal `zip` and `zip` plus `quick-xml` consumers (size baseline); dependency and licence tree of the `zip` features used | PLANNED |
| P3 | LibreOffice open check: definition pinned, implemented, negative test on a deliberately corrupted output | PLANNED |
| P4 | First full recorded run: `records/step0-<date>-<sha>.md` and `.tsv`, aggregated by part family | PLANNED |

### T4: moved into Milestone 1

The names-only XML comparison is now Milestone 1 part 7 (maintainer
ruling 12, 2026-09-14).

### T5: architecture decision

| Phase | Part | Status |
|-------|------|--------|
| P1 | `records/decision-architecture.md`: the maintainer chose greenfield on 2026-09-14, before any measurement | DONE (committed in ced7f91, byte-identical to planning's draft) |

## Standing items (maintainer rulings 2026-09-14)

- Architecture: greenfield hybrid. umya-spreadsheet and rust_xlsxwriter
  are reference material only.
- Excel: no Excel checks during step zero. From Milestone 1 on, once per
  milestone, the maintainer opens outputs listed by sha256 in desktop
  Excel, and pass or fail per hash is recorded. Excel for the web is not
  a substitute.
- Size budget: provisional 250 KiB, stripped, i686 musl, measured as
  emend's increase over a consumer that already links `zip` and
  `quick-xml`. Re-ruled after T3 records the baseline.
- Licence: MIT OR Apache-2.0, copyright holder "emend contributors".
- GitHub: `thekeoni1/emend`, private. Every push passes the privacy gate
  (T0 P6) and needs planning's word plus the maintainer's authorisation
  in the implementing session. Commit messages carry no session links.

## Milestone 1: greenfield core (PROPOSED)

Outline approved as a proposal by the maintainer on 2026-09-14. Each
part is scoped into phases, with its own spec, before work on it starts.
The core depends on `zip` and `quick-xml` only.

1. Zip layer: read the central directory with `u64` sizes and offsets,
   copy untouched entries as their original compressed bytes in the
   original order, and rewrite touched entries by streaming.
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

## Unscoped

Listed so they are not lost. Scoping any of them needs the maintainer's
approval.

- Size-measurement consumer: a small i686 musl binary that links the
  core, with the byte delta recorded against the commit measured.
- A machine with desktop Excel for the per-milestone check. Milestone 1
  cannot be accepted without it.
- Gate self-test script: a committed script that reruns every phase's
  negative tests. The maintainer approved scoping it after T0
  (2026-09-15).
- Privacy gate backlog: findings raised after T2 P0 that do not concern a
  data flow this project has.
  - Unchecked `sed`, `tr` and `cut` exit statuses in two helper
    pipelines of `scripts/privacy.sh` (T2 P0 finding 1).
  - Content findings in archive or compound-file blobs at ordinary paths
    print the path (T2 P0 finding 2).
  - Both fit the gate self-test script task.
- MSRV policy.
- README, where every fidelity, speed or size sentence cites a record.
