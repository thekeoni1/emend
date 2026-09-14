# 0000: Step zero plan

Status: rulings recorded 2026-09-14 (see the end of this file). Phase
breakdown and status: `ROADMAP.md`.

## Question step zero answers

Extend umya-spreadsheet, or build a greenfield hybrid that parses only
what it edits and passes every other zip entry through byte for byte.
The answer comes from measurements over a private corpus. No workbook
model is written until the decision record (T5) is ruled on.

## Facts at planning time

Checked 2026-09-14 on the development machine (WSL2, kernel
6.18.33.2-microsoft-standard-WSL2):

- rustc 1.98.1, cargo 1.98.1. Installed targets include
  `i686-unknown-linux-musl`. A scratch hello-world built for that target
  is a static 32-bit ELF, and its `cargo test` runs natively on this
  kernel (unstripped release binary: 561880 bytes). T0 re-proves this
  inside the repo.
- umya-spreadsheet 3.1.0: MIT, `rust-version = 1.88.0`, 21 normal
  dependencies (aes, base64, byteorder, cbc, cfb, chrono, encoding_rs,
  fancy-regex, hmac, html5gum, imagesize, jiff, md-5, num-traits, phf,
  quick-xml ^0.41, rand, rgb, sha2, thousands, zip ^8.6). It offers
  `reader::xlsx::read` and `reader::xlsx::lazy_read`.
- Current releases: zip 8.6.0 (MIT), quick-xml 0.42.0 (MIT). umya pins
  quick-xml ^0.41, so a consumer on 0.42 would link two quick-xml
  copies. That bears on the core dependency rule regardless of fidelity.
- No Excel on this machine or its Windows host. LibreOffice is not
  installed. podman and docker are available.
- Private corpus directory: `/home/dev/emend-testing` (mode 700, outside
  the repo), with `corpus/` for inputs and `runs/` for raw outputs.
  Currently empty.

## Task sequence

One task at a time. Every task is split into small phases. Each phase is
committed, gated, checked by an independent subagent (CLAUDE.md), and
reported; planning then verifies it from primaries before the next phase
starts. Each phase spec names the logs it must produce and at least one
negative test showing its checks can fail. T2 is maintainer-led and runs
alongside T1.

| Task | What | Spec |
|------|------|------|
| T0 | Repo bootstrap: workspace, gate script, 32-bit test proof | `0001-bootstrap.md` |
| T1 | `xlsx-diff`: the entry-level fidelity harness | written after T0 verifies |
| T2 | Corpus assembly and manifest (maintainer-led) | written alongside T1 |
| T3 | Step-zero runner and first recorded run | written after T1 verifies |
| T4 | Names-only XML comparison for changed entries | written after T3 reports |
| T5 | Decision record: extend or greenfield | planning writes, maintainer rules |

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

Sources (ruling 2026-09-14):

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

### T3: runner (outline)

`tools/step0`, a non-default workspace member, so umya never enters the
build of anything else. For every corpus file, three arms, each in a
subprocess with a timeout (default 300 s) to isolate panics, aborts and
memory exhaustion:

- `A0 zip-raw` (control): copy every entry with the `zip` crate's raw
  copy, in the original order. It shows what a pure passthrough
  container achieves, which is the greenfield ceiling. It also separates
  container noise from model loss.
- `A1 umya-read`: `reader::xlsx::read` then `writer::xlsx::write`, no
  edits.
- `A2 umya-lazy`: `reader::xlsx::lazy_read` then write, no edits.

Per file and arm, record: outcome (ok, error, panic, timeout, crash),
wall time, peak RSS, output size, then the `xlsx-diff` rows.

The runner runs both host and `i686-unknown-linux-musl` builds. It also
records the stripped i686 musl release size of a minimal A0 binary and a
minimal A1 binary, plus the full dependency and licence tree of umya as
built. The A0 binary is the baseline for the size budget.

Raw outputs go to `/home/dev/emend-testing/runs/<utc-date>-<shortsha>/`.
The committed record is
`records/step0-<utc-date>-<shortsha>.md` plus a `.tsv` of entry rows
keyed by corpus id. The record aggregates rows by part family:
content-types, rels, workbook, worksheet, sharedStrings, styles, theme,
calcChain, drawing, chart, pivotTable, pivotCache, table, queryTable,
connections, externalLink, comments, threadedComments, persons, vml,
media, embeddings, vbaProject, customXml, docProps, printerSettings,
slicer, timeline, richData, metadata, other.

LibreOffice open check: the pinned container from T2 P2 opens each
output headless. What counts as "opens without repair" for LibreOffice
has to be pinned down, because LibreOffice has no repair prompt.
Candidate: headless conversion exits 0, stderr is empty of load errors,
and the sheet count matches the input. T3 P4 settles this.

No Excel checks during step zero (ruling 2026-09-14).

### T4: names-only XML comparison (outline)

For `changed` XML entries, parse both sides with quick-xml in streaming
mode and compare multisets of element names (namespace URI plus local
name) and of (element, attribute) names. Ignore attribute order, prefix
choice, whitespace-only text and the XML declaration. Report names and
count deltas only. Values are compared by hashing a canonical value
stream per side; the report says only whether they match and how many
differ, never what they are. This distinguishes serialisation noise from
information loss, which byte diffs alone cannot.

## Decision criteria (accepted 2026-09-14)

Fixed before data arrives, so the result is not argued after the fact.

Hard criteria. If either fails, the answer is greenfield:

- E4 Reachable byte identity (the deciding criterion): a credible route
  exists for umya to emit the original bytes of every entry an edit
  does not touch, without replacing its writer wholesale. A0 shows the
  container side is achievable; the question is umya's writer.
- E5 Rules compliance, pass or fail: umya's dependencies other than zip
  and quick-xml can move behind off-by-default features, the quick-xml
  version skew can be resolved, and the core's i686 musl size delta
  fits the budget below.

Supporting criteria. If E4 and E5 pass, these decide whether the
extension is localisable; a failure that is not localisable means
greenfield:

- E1 Robustness: A1 or A2 reads and writes every corpus file without
  error, panic, timeout or crash, on host and i686. Failures that do
  occur trace to a small number of local reader defects.
- E2 No structural loss: no `dropped` entries, or dropped entries belong
  to part families that could be carried as raw passthrough with a
  change local to one module.
- E3 Localised semantic loss: T4 name-level differences are confined to
  a short, enumerable list of names, each traceable to one umya module.

Either way, `xlsx-diff` and the T4 comparison remain the project's
fidelity harness.

## Maintainer rulings (2026-09-14)

1. Licence: MIT OR Apache-2.0.
2. Decision criteria: accepted as above, with E4 deciding and E5 a hard
   pass or fail.
3. Excel: no Excel checks during step zero. From Milestone 1, once per
   milestone, the maintainer opens outputs listed by sha256 in desktop
   Excel; pass or fail per hash is recorded. Excel for the web is not a
   substitute.
4. Size budget: provisional 250 KiB, stripped, i686 musl, measured as
   emend's increase over a consumer that already links `zip` and
   `quick-xml`. Re-ruled after T3 P3 records the baseline.
5. LibreOffice: a container pinned by image digest, version recorded.
6. Corpus: the three sources listed under T2.
7. Copyright holder in the licence files: emend contributors.

Still open: MSRV policy (after step zero).
