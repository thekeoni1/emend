# 0005: T2 corpus (maintainer-led)

Status: P1 READY (planning, 2026-09-15). Runs alongside T1. The maintainer
may start collecting files as soon as the manifest format below exists;
the scripts in P1 make that easier but are not required to start.
Context: `CLAUDE.md` ("Corpus rules"), `specs/0000-step-zero-plan.md`
(the T2 outline, the producers, the tag list, ruling 6),
`specs/0002-gate-corpus-additions.md` (T2 P0, done).

## Goal

A private corpus of real workbooks, 20 to 40 files, covering every
feature tag at least once and most tags from more than one producer,
described by a private manifest, so that T3 and Milestone 1 can cite a
workbook by id, producer and tags and nothing else.

## Privacy rules for the corpus (restating CLAUDE.md, with specifics)

- Everything under the corpus directory is private: the files, the
  manifest, the run outputs. Nothing under it is ever copied into the
  repository, quoted in a commit, a log, a record or a report.
- The repository cites a workbook only by its id (the first 12 hex
  digits of its sha256), its producer and its tags, plus byte counts,
  entry names and pass or fail. Never its original file name, never a
  cell value, never a defined name, sheet name or author.
- The corpus directory is `EMEND_CORPUS_DIR`, defaulting to
  `$HOME/emend-testing`. Scripts in the repository never print that
  path, and the repository's scripts never contain a literal home path.
- No corpus file goes to CI, ever.

## Layout of the corpus directory (private)

- `corpus/<id>.<ext>`: the workbooks, renamed by id; `<ext>` is one of
  `xlsx`, `xlsm`, `xltx`, `xltm`, lowercase. Mode 600.
- `manifest.tsv`: one header row, then one row per workbook.
- `runs/<utc-date>-<shortsha>/`: raw outputs of T3 and later runs.
- Anything else the maintainer keeps there (download notes, source
  URLs) is private too.

## Manifest format

Tab-separated, UTF-8, one header row with exactly these columns in this
order:

| Column | Content |
|--------|---------|
| `id` | first 12 hex digits of the sha256 of the file |
| `sha256` | the full sha256, 64 lowercase hex digits |
| `bytes` | file size |
| `ext` | `xlsx`, `xlsm`, `xltx` or `xltm` |
| `producer` | `excel-win`, `excel-mac`, `google-sheets`, `libreoffice` or `unknown` |
| `producer_version` | free text, or `unknown` |
| `app` | the `Application` and `AppVersion` values from `docProps/app.xml`, joined by a space, or `none` when the part or the values are absent |
| `source` | `libreoffice-export`, `ms-template`, `google-sheets-export` or `maintainer` |
| `tags` | comma-separated tags from the fixed list below, no spaces |
| `added` | UTC date, `YYYY-MM-DD` |
| `original_name` | the file name before renaming; PRIVATE, never printed by any script |
| `note` | free text; PRIVATE, never printed by any script |

The fixed tag list, from `specs/0000-step-zero-plan.md`: `plain`,
`strings`, `formulas`, `names`, `merge`, `validation`, `hyperlinks`,
`autofilter`, `freeze`, `cf`, `tables`, `charts`, `pivots`, `slicers`,
`timelines`, `macros`, `comments`, `images`, `external`, `sparklines`,
`protection`, `date1904`, `large`.

A row's `id` is unique. The file `corpus/<id>.<ext>` exists and hashes
to `sha256`.

## Phases

T2 phases run between T1 phases when the implementing session is free.
Each implementing phase follows `specs/0001-bootstrap.md` "How this
task runs", with logs in `records/logs/t2-<phase>/`. Maintainer-led
phases (P4 to P6) have no implementing work and no logs; the coverage
report of P7 records their result.

### P1: manifest scripts

Two scripts in `scripts/`, bash, `#!/usr/bin/env bash`, `set -euo
pipefail`, `/usr/bin/grep` explicitly, no dependency beyond coreutils,
`sha256sum`, `python3` (for reading `docProps/app.xml` with the
standard `zipfile` module; no third-party module).

`scripts/corpus-add.sh <file> <producer> <producer_version> <source> <tags> [note]`:
- Resolves the corpus directory from `EMEND_CORPUS_DIR` or the default,
  requires it to exist with mode 700, and requires it to be outside any
  git work tree (`git -C <dir> rev-parse` fails).
- Requires the file's extension, lowercased, to be one of the four, and
  its first four bytes to be `50 4B 03 04`. An OLE2 file (an old `.xls`
  or an encrypted workbook) is refused with the message `refused: not a
  zip-based workbook`. Nothing else about the file is printed.
- Validates `producer`, `source` and every tag against the fixed lists.
- Computes sha256 and id, refuses a duplicate id, copies the file to
  `corpus/<id>.<ext>` with mode 600, reads `app` from
  `docProps/app.xml` (values only, XML-unescaped, whitespace collapsed;
  `none` when absent), and appends the manifest row, creating the
  manifest with its header when missing.
- Prints exactly one line: `added <id> <ext> <bytes> <producer> <tags>`.
  It never prints the original name, the note, the corpus path or any
  content of the file.

`scripts/corpus-coverage.sh [--verify]`:
- Reads the manifest. Validates the header, the column count of every
  row, `producer`, `source`, the tags, the uniqueness of ids, and that
  every `corpus/<id>.<ext>` exists. With `--verify`, also recomputes
  every sha256. Any failure prints `FAIL <reason>` (reasons never
  include private columns) and exits 1.
- Prints, in this order:
  - `files <n>`;
  - one line per workbook: `<id> <ext> <bytes> <producer> <source>
    <tags>`;
  - one line per tag in the fixed order: `tag <name> <count>
    <producers>` where `<producers>` is the comma-separated list of
    producers that cover it (or `-`);
  - `uncovered <k>: <tag,...>` (or `uncovered 0`);
  - `single-producer <k>: <tag,...>` for tags covered by exactly one
    producer.
- Exits 0. Its output is safe to paste into a record, and T2 P7 does.

Tests, in `negative.log` and `positive.log` under a scratch corpus
directory that the test creates in a temp dir and deletes afterwards,
using generated workbooks only (a minimal zip with `[Content_Types].xml`
and a `docProps/app.xml` is enough; the scripts do not read
worksheets):
- add a generated `xlsx`: the `added` line has the expected id; the
  manifest gains one row; the copied file has mode 600 and the right
  hash;
- add the same file again: refused as a duplicate, manifest unchanged;
- add a file whose first bytes are OLE2: refused, nothing added;
- add with an unknown tag, an unknown producer, an upper-case
  extension `.XLSX` (accepted, stored as `xlsx`), and a `.csv`
  (refused);
- an original name containing a sentinel string never appears in any
  output of either script;
- coverage on the scratch manifest lists every tag with the right
  counts, and `--verify` fails after one file is modified in place.

Logs: `gate.log`, `positive.log`, `negative.log`, `scan.log`. No push.

### P2: LibreOffice, pinned

A Containerfile in the repository that builds a headless LibreOffice
image from a base image pinned by digest, with the package version
pinned; `soffice --version` recorded; a smoke test that converts a
generated flat ODS to xlsx headless and back, on a fictional file. The
digest, the package version and the conversion command go in
`records/libreoffice-<date>.md`. Scoped in its own spec section when
T1 P2 verifies.

### P3: LibreOffice exports with fictional content

A script generates flat ODS files, one per tag LibreOffice can produce
(at least `plain`, `strings`, `formulas`, `names`, `merge`,
`validation`, `hyperlinks`, `autofilter`, `freeze`, `cf`, `charts`,
`comments`, `images`, `protection`, `date1904`, `large`), converts each
with the P2 image, and adds the results with `corpus-add.sh` under
producer `libreoffice`, source `libreoffice-export`. Fictional content
only, so these files could be public, but they are kept in the corpus
like everything else.

### P4: Microsoft templates (maintainer)

The maintainer downloads Excel templates from Microsoft's template site,
adds them with `corpus-add.sh` (producer `excel-win` unless the file
says otherwise, source `ms-template`), and keeps the source URLs in a
private note. Target tags: `tables`, `charts`, `cf`, `names`, `strings`,
`formulas`, `images`, `sparklines`.

### P5: Google Sheets exports (maintainer)

About five exports covering `formulas`, `cf`, `validation`,
`hyperlinks`, `merge`, `freeze`, `comments`, `charts`, `names`, added
with producer `google-sheets`, source `google-sheets-export`.

### P6: desktop Excel files (maintainer)

Files for what templates rarely contain: `pivots`, `slicers`,
`timelines`, `macros` (an `.xlsm`), `comments` (threaded), `external`,
`sparklines`, plus any own workbooks the maintainer is comfortable using.
Producer `excel-win` or `excel-mac`, source `maintainer`. Each is added
with `corpus-add.sh`; the private note may say where it came from.

### P7: coverage check

`scripts/corpus-coverage.sh --verify` is run and its output committed
as `records/corpus-coverage-<date>.md`. Every tag covered, or the gaps
listed for a planning ruling on starting T3 anyway. The `large` tag
needs one workbook whose largest worksheet part exceeds 50 MB
uncompressed; if none exists, P3 generates one.

## Out of scope

- Encrypted workbooks, `.xlsb`, `.xls` (refused by `corpus-add.sh`).
- Any automatic tag detection. Tags are set by the person or script that
  adds the file; T3 and Milestone 1 may later add a checker that reads
  entry names to confirm tags such as `charts` or `pivots`.

## Done

T2 is done when P7's coverage record is committed and planning has
ruled on any gaps.
