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
- `UNSCOPED`: an idea. Scoping it needs the maintainer's approval.

Planning updates this file when it verifies a phase. New milestones or
ideas are added only after they are scoped and the maintainer approves.

## Milestone 0: step zero (architecture decision)

Goal: decide between extending umya-spreadsheet and a greenfield hybrid,
from measurements. Plan and decision criteria:
`specs/0000-step-zero-plan.md`.

### T0: repo bootstrap (`specs/0001-bootstrap.md`)

| Phase | Part | Status |
|-------|------|--------|
| P1 | Commit CLAUDE.md, ROADMAP.md and specs as they stand; ASCII and U+2014 scan | READY |
| P2 | Cargo workspace, toolchain pin, licence files, xlsx-diff skeleton, .gitignore; build and test on host and i686 by hand | READY |
| P3 | `scripts/check.sh` steps host, i686, fmt, clippy; negative tests | READY |
| P4 | `check.sh` steps native (no C, `-sys` allowlist) and licences; negative tests | READY |
| P5 | `check.sh` step hygiene (U+2014, office files, corpus paths, API key); toolchain record; negative tests | READY |

### T1: xlsx-diff, the fidelity harness

| Phase | Part | Status |
|-------|------|--------|
| P1 | Entry inventory across both archives: `dropped`, `added`, raw-identical, index columns; tests on synthetic zips | PLANNED |
| P2 | Streaming content comparison: `recompressed` vs `changed`, first and last differing offset, differing-byte count, all `u64` | PLANNED |
| P3 | Archive-level lines (entry order, comment, totals), timestamp and extra-field flags, exit codes 0/1/2 | PLANNED |
| P4 | No-leak sentinel test on stdout and stderr; zip64 entry over 4 GiB uncompressed compared by streaming on i686 (ignored by default, run explicitly and logged) | PLANNED |

### T2: corpus (maintainer-led, runs alongside T1)

Private, in `/home/dev/emend-testing`. Nothing from it is committed.

| Phase | Part | Status |
|-------|------|--------|
| P1 | Manifest format and a coverage script in the repo that prints only ids, producers, tags and counts | PLANNED |
| P2 | LibreOffice container pinned by image digest, version recorded, headless conversion smoke test on a generated fictional file | PLANNED |
| P3 | LibreOffice exports with fictional content, generated as flat ODS by a script and converted headless, covering the tags LibreOffice can produce | PLANNED |
| P4 | Microsoft's downloadable Excel templates added, with sources noted in the private manifest | PLANNED |
| P5 | Maintainer: about 5 Google Sheets exports covering the tag list | PLANNED |
| P6 | Maintainer: desktop Excel files for pivots, macros (.xlsm), threaded comments and slicers, plus any own workbooks | PLANNED |
| P7 | Coverage check: every tag covered, or gaps listed for a planning ruling on starting T3 anyway | PLANNED |

### T3: step-zero runner and first recorded run

| Phase | Part | Status |
|-------|------|--------|
| P1 | Subprocess isolation (timeout, panic, abort, memory) and arm A0 zip-raw on host; run over the corpus | PLANNED |
| P2 | Arms A1 umya `read` and A2 umya `lazy_read` | PLANNED |
| P3 | i686 musl builds and runs of all arms; stripped size of minimal A0 and A1 binaries (size baseline); umya dependency and licence tree | PLANNED |
| P4 | LibreOffice open check: definition pinned, implemented, negative test on a deliberately corrupted output | PLANNED |
| P5 | First full recorded run: `records/step0-<date>-<sha>.md` and `.tsv`, aggregated by part family | PLANNED |

### T4: names-only XML comparison

| Phase | Part | Status |
|-------|------|--------|
| P1 | Namespace-resolved element and attribute name multisets; tests showing prefix, attribute-order and whitespace noise count as no difference | PLANNED |
| P2 | Canonical value-stream hashing: match yes/no and differing count, never values; leak sentinel test | PLANNED |
| P3 | Run over T3 outputs and record | PLANNED |

### T5: architecture decision

| Phase | Part | Status |
|-------|------|--------|
| P1 | Planning drafts `records/decision-architecture.md` from primaries against E1 to E5 | PLANNED |
| P2 | Independent check of the draft by a fresh subagent given only the draft, the criteria and the records | PLANNED |
| P3 | Maintainer rules; this file gains the Milestone 1 scope | PLANNED |

## Standing items (maintainer rulings 2026-09-14)

- Excel: no Excel checks during step zero. From Milestone 1 on, once per
  milestone, the maintainer opens outputs listed by sha256 in desktop
  Excel, and pass or fail per hash is recorded. Excel for the web is not
  a substitute.
- Size budget: provisional 250 KiB, stripped, i686 musl, measured as
  emend's increase over a consumer that already links `zip` and
  `quick-xml`. Re-ruled after T3 P3 records the baseline.
- Licence: MIT OR Apache-2.0, copyright holder "emend contributors".

## After step zero (UNSCOPED)

Listed so they are not lost. None is scoped until T5 is ruled and the
maintainer approves.

- Milestone 1: the core (open, read, edit, write back) on the
  architecture T5 picks, depending on `zip` and `quick-xml` only.
- Size-measurement consumer: a small i686 musl binary that links the
  core, with the byte delta recorded against the commit measured.
- Where the per-milestone Excel check runs (which machine).
- MSRV policy.
- README, where every fidelity, speed or size sentence cites a record.
