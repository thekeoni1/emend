# 0000: Step zero plan

Status: the architecture was decided on 2026-09-14. The maintainer chose
the greenfield hybrid before any step-zero measurement (maintainer
ruling 11; `records/decision-architecture.md`). Step zero is reduced to
what the greenfield build still needs. Phase breakdown and status:
`ROADMAP.md`.

## What step zero is now

Step zero was planned to decide, from measurements, between extending
umya-spreadsheet and building a greenfield hybrid. That question is
closed. Step zero now delivers three things the greenfield build needs:

- the fidelity harness `xlsx-diff` (T1);
- a private corpus of real workbooks (T2);
- one measured run proving plain zip passthrough on that corpus, on the
  host and on 32-bit, plus the size baseline for the core (T3).

The names-only XML comparison (formerly T4) moves into Milestone 1.

## Facts at planning time

Checked 2026-09-14 on the development machine (WSL2, kernel
6.18.33.2-microsoft-standard-WSL2):

- rustc 1.98.1, cargo 1.98.1. Installed targets include
  `i686-unknown-linux-musl`. A static 32-bit test binary builds and its
  tests run natively on this kernel (T0 P2 re-proved this in the repo).
- rustup 1.29.1 has auto-install on. A toolchain named `1.98.1` is
  separate from `stable`, even though both are rustc 1.98.1. emend
  commands run with `RUSTUP_AUTO_INSTALL=0`.
- umya-spreadsheet 3.1.0: MIT, 21 normal dependencies, quick-xml ^0.41.
  Its source was read for the architecture decision; see
  `records/decision-architecture.md`.
- Current releases: zip 8.6.0 (MIT), quick-xml 0.42.0 (MIT).
- No Excel on this machine or its Windows host. LibreOffice is not
  installed. podman and docker are available.
- Private corpus directory: `/home/dev/emend-testing` (mode 700, outside
  the repo), with `corpus/` for inputs and `runs/` for raw outputs.

## Task sequence

One task at a time. Every task is split into small phases. Each phase is
committed, gated, checked by an independent subagent (CLAUDE.md), and
reported; planning then verifies it from primaries before the next phase
starts. Each phase spec names the logs it must produce and at least one
negative test showing its checks can fail. T2 is maintainer-led and runs
alongside T1.

| Task | What | Spec |
|------|------|------|
| T0 | Repo bootstrap: workspace, gate script, privacy gate, first push | `0001-bootstrap.md` |
| T1 | `xlsx-diff`: the entry-level fidelity harness | `0004-xlsx-diff.md` |
| T2 | Corpus assembly and manifest (maintainer-led) | `0005-corpus.md` |
| T3 | Passthrough baseline: zip raw-copy run over the corpus, size baseline | written after T1 verifies |
| T4 | Moved into Milestone 1 (names-only XML comparison) | - |
| T5 | Architecture decision record | `records/decision-architecture.md` |

### T1: xlsx-diff (outline)

A binary crate in `tools/xlsx-diff`, depending on `zip` only (built with
`default-features = false` and only the codecs xlsx needs, with no C
backends; the gate proves this). Given an input and an output workbook:

- One row per entry name across both archives, with status:
  - `identical`: decompressed bytes equal and raw compressed stream,
    method and header fields equal.
  - `recompressed`: decompressed bytes equal, raw bytes differ.
  - `changed`: decompressed bytes differ.
  - `dropped`: in input, not in output.
  - `added`: in output, not in input.
- Columns: entry name, status, uncompressed and compressed size on each
  side, method on each side, CRC32 on each side, index on each side,
  first differing offset, and when sizes are equal, the count of
  differing bytes and the last differing offset. Flags for timestamp and
  extra-field differences.
- Archive-level lines: entry order preserved yes/no, archive comment
  equal yes/no, total sizes.
- All sizes, offsets and counts are `u64`. Entries are compared by
  streaming with fixed buffers; no entry is loaded whole into memory.
- It never prints entry content. A test plants a sentinel string in
  synthetic fixture entries and asserts it is absent from stdout and
  stderr.
- Exit codes: 0 when every entry is `identical`, 1 when any difference
  exists, 2 on error.
- Tests use small zips generated in test code by the `zip` crate. No
  real workbooks.

### T2: corpus (maintainer-led)

Files go in `/home/dev/emend-testing/corpus/`. A manifest
`/home/dev/emend-testing/manifest.tsv` (private, never committed) lists
for each file: sha256, producer, producer version, feature tags, and a
private note. Records in the repo cite a workbook only as the first 12
hex digits of its sha256 plus producer and tags. Original filenames
never appear in the repo, because filenames can leak contents.

Producers: `excel-win`, `excel-mac`, `google-sheets`, `libreoffice`.

Sources (ruling 6):

1. LibreOffice exports with fictional content, produced on this machine
   from a pinned LibreOffice container.
2. Microsoft's downloadable Excel templates, which are made in Excel and
   carry tables, charts, conditional formatting and styles.
3. From the maintainer: about 5 Google Sheets exports, a few desktop
   Excel files for what templates rarely contain (pivots, macros,
   threaded comments, slicers), and any own workbooks the maintainer is
   comfortable using.

Feature tags to cover at least once each, with more than one producer
where the producer can emit the feature:

- `plain`: values, number formats, styles, column widths
- `strings`: shared strings, inline strings, rich text runs
- `formulas`: cached values, shared formulas, array and dynamic arrays
- `names`: defined names, print areas and titles
- `merge`, `validation`, `hyperlinks`, `autofilter`, `freeze`
- `cf`: conditional formatting, including x14 extension rules
- `tables`: structured tables
- `charts`: classic charts and chartex (e.g. waterfall)
- `pivots`: pivot tables and caches; `slicers`, `timelines`
- `macros`: .xlsm with vbaProject.bin
- `comments`: legacy comments (VML) and threaded comments with persons
- `images`: pictures, shapes, drawings
- `external`: external links, data connections, query tables
- `sparklines`
- `protection`: sheet and workbook protection (not encryption)
- `date1904`
- `large`: at least one workbook whose largest worksheet part exceeds
  50 MB uncompressed, for the 32-bit address-space concern

Out of scope for step zero: encrypted workbooks (CFB container), .xlsb,
.xls.

Starting size: 20 to 40 files, enough that every tag above is covered.

### T3: passthrough baseline (outline)

A runner in `tools/`, named in the T3 spec. For every corpus file it
copies every entry with the `zip` crate's raw copy, in the original
order, in a subprocess with a timeout (default 300 s) that isolates
panics, aborts and memory exhaustion. Then it runs `xlsx-diff` on input
and output.

- The expectation is that every entry is `identical`. Any other status,
  error or crash is a finding about real archives (zip64, data
  descriptors, extra fields, comments, entry order, unusual producers)
  that the greenfield zip layer has to handle.
- Per file, record: outcome (ok, error, panic, timeout, crash), wall
  time, peak RSS, output size, and the `xlsx-diff` rows.
- Run on the host and on `i686-unknown-linux-musl`.
- Size baseline: the stripped i686 musl release size of a minimal
  consumer linking `zip` only, and of one linking `zip` and
  `quick-xml`. The core's budget (ruling 4) is measured against the
  second.
- Record the dependency and licence tree of the `zip` features used.
- LibreOffice open check: the pinned container from T2 P2 opens each
  output headless. LibreOffice has no repair prompt, so the T3 spec pins
  what "opens without repair" means. Candidate: headless conversion
  exits 0, stderr has no load errors, and the sheet count matches the
  input.
- No Excel checks during step zero (ruling 3).

Raw outputs go to `/home/dev/emend-testing/runs/<utc-date>-<shortsha>/`.
The committed record is `records/step0-<utc-date>-<shortsha>.md` plus a
`.tsv` of entry rows keyed by corpus id. The record aggregates rows by
part family: content-types, rels, workbook, worksheet, sharedStrings,
styles, theme, calcChain, drawing, chart, pivotTable, pivotCache, table,
queryTable, connections, externalLink, comments, threadedComments,
persons, vml, media, embeddings, vbaProject, customXml, docProps,
printerSettings, slicer, timeline, richData, metadata, other.

### Moved to Milestone 1: names-only XML comparison (outline)

For `changed` XML entries, parse both sides with quick-xml in streaming
mode and compare multisets of element names (namespace URI plus local
name) and of (element, attribute) names. Ignore attribute order, prefix
choice, whitespace-only text and the XML declaration. Report names and
count deltas only. Values are compared by hashing a canonical value
stream per side; the report says only whether they match and how many
differ, never what they are. In Milestone 1 this shows that edited
entries differ only where the edit requires.

## Decision criteria (retired 2026-09-14)

Criteria E1 to E5 (robustness, structural loss, localised semantic loss,
reachable byte identity, rules compliance) were fixed before any data
arrived. They were retired unevaluated when the maintainer chose the
greenfield hybrid (ruling 11).

## Maintainer rulings (2026-09-14)

1. Licence: MIT OR Apache-2.0.
2. Decision criteria E1 to E5 accepted, with E4 deciding and E5 a hard
   pass or fail. Retired by ruling 11.
3. Excel: no Excel checks during step zero. From Milestone 1, once per
   milestone, the maintainer opens outputs listed by sha256 in desktop
   Excel; pass or fail per hash is recorded. Excel for the web is not a
   substitute.
4. Size budget: provisional 250 KiB, stripped, i686 musl, measured as
   emend's increase over a consumer that already links `zip` and
   `quick-xml`. Re-ruled after T3 records the baseline.
5. LibreOffice: a container pinned by image digest, version recorded.
6. Corpus: the three sources listed under T2.
7. Copyright holder in the licence files: emend contributors.
8. `memory/` in CLAUDE.md means Claude Code's memory folder for this
   project, outside the repo.
9. Commit messages carry no Claude-Session line. The P1 commits are
   rewritten to remove it (T0 P1R).
10. GitHub: `thekeoni1/emend`, private. A privacy gate is T0 P6, and
    the first push is T0 P7, right after P6 verifies. Nothing personal
    (names, email addresses, host paths, session links) is ever pushed.
11. Architecture: greenfield hybrid, chosen before any step-zero
    measurement, based on planning's reading of umya-spreadsheet
    3.1.0's source. CLAUDE.md records the decision.
12. Restructure: T3 keeps only the zip raw-copy passthrough run and the
    size baseline; the umya-spreadsheet runs are dropped; T4 moves into
    Milestone 1; T5 is the decision record; the Milestone 1 outline goes
    on ROADMAP.md as a proposal.
13. The machine-wide rustup auto-install setting stays as it is. emend
    uses the per-command `RUSTUP_AUTO_INSTALL=0` guard.

Still open: MSRV policy (after step zero).
