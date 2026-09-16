# 0005: T2 corpus (maintainer-led)

Status: P1 DONE, verified by planning 2026-09-15 (45dd9e3 to 869796b;
import commit e8087f6, phase commits e3dd52e and 4318820). P2, P3 and
P4 DONE by the CORPUS session, verified by planning 2026-09-15 from the
corpus directory (rulings below). P5 and P6 are the maintainer's, open.
P7 DONE, verified by planning 2026-09-16 (58063fb to 7b9350a;
`records/corpus-coverage-2026-09-16.md`). T2 is DONE apart from P5 and
P6; the close-out push follows (section below).

The CORPUS session is a third session, separate from planning and
implementing. It works only under the corpus directory, never changes
the repository, and follows the brief the maintainer keeps privately in
that directory. It generates the LibreOffice files (P2, P3) and
downloads Microsoft's public templates (P4) with its own private
helper that follows the manifest format below. Planning verifies its
report from the directory itself. The repository scripts of P1 come
afterwards, so they can replace that helper.
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

P1 and P7 are implementing phases and run between T1 phases when the
implementing session is free, following `specs/0001-bootstrap.md` "How
this task runs", with logs in `records/logs/t2-<phase>/`. P2, P3 and P4
belong to the CORPUS session; P5 and P6 to the maintainer. These have
no repository commits and no logs; the coverage record of P7 reports
their result. If the Containerfile and generators of P2 and P3 are to
enter the repository, that is a deliberate, recorded import step that
planning rules on (CLAUDE.md, "Secrets and boundaries").

### P1: manifest scripts

Two scripts in `scripts/`. Planning ruled on 2026-09-15 that they may
be python3 (standard library only, `#!/usr/bin/env python3`), like
`scripts/passthrough-record.py`, instead of bash; the corpus session's
private helper `tools/corpus-add.py` is written that way and follows
these rules. Importing it into the repository as `scripts/corpus-add.py`
is a deliberate, recorded step (CLAUDE.md, "Secrets and boundaries"):
the phase report names the private file's sha256 at import and every
change made to it, and the phase adds the logged negative cases below,
which the private helper never had. No other file from the corpus
directory enters the repository.

`scripts/corpus-add.py <file> <producer> <producer_version> <source> <tags> [note]`:
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

`scripts/corpus-coverage.py [--verify]`:
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

### Planning rulings on the corpus session's delivery (2026-09-15)

Planning verified P2, P3 and P4 from the corpus directory itself, not
from the session's report:
- 27 files (17 `libreoffice`, `libreoffice-export`; 10 `excel-win`,
  `ms-template`; 26 xlsx and 1 xltx), 17693628 bytes, all mode 600
  under a mode 700 directory. The manifest has the 12 columns in order;
  every id is unique and equals the first 12 hex digits of the file's
  sha256; every byte count matches; every file starts with the zip
  signature; every producer, source and tag is from the fixed lists;
  every `ms-template` note holds a source URL.
- Tags: planning checked every claimed tag against the file's own parts
  with its own rules (charts, pivot, slicer, timeline, table, comment,
  media, external and vbaProject entries; sparkline, conditional
  formatting, data validation, hyperlink, autofilter, frozen pane,
  merged cell, formula and protection elements; defined names, the
  1904 flag, and the largest worksheet part). Every claimed tag is
  present. 22 of 23 tags are covered; `macros` is not. Seven tags come
  from one producer only: pivots, slicers, timelines, external,
  sparklines (excel-win) and date1904, large (libreoffice). Threaded
  comments and chartex charts appear in no file. LibreOffice writes an
  empty `<workbookProtection/>` element in every file; the session
  rightly does not count it as protection.
- Privacy: `dc:creator`, `cp:lastModifiedBy`, `Company` and `Manager`
  are empty or absent in all 27 files. No part of any file holds the
  user name, the host name, a home path, an email address outside the
  example domains, or a private-denylist entry. One Microsoft template
  carries its own author's Windows path in an `x15ac:absPath` element:
  Microsoft's public metadata, not this project's, and workbook content
  never reaches the repository.
- The image: `localhost/emend-lo:25.2.3-2-deb13u6`, id
  `3f9baefc6e23...`, present in local podman storage with the size the
  record gives; LibreOffice 25.2.3.2 from Debian `4:25.2.3-2+deb13u6`
  on `debian@sha256:abc9cb88...`. Conversions run through a Basic macro
  in a warmed profile because headless `--convert-to` drops frozen
  panes and left cached formula values at 0 on the 130000-row sheet;
  the session recorded both findings.
- The repository was not changed by the corpus session.

Rulings:
- P2, P3 and P4 are DONE. The corpus is usable for T3 P3 and P4 now.
- P5 and P6 (maintainer): what only the maintainer can supply is one
  `.xlsm` with a macro (`macros`), a desktop Excel file with threaded
  comments, one with a chartex chart such as a waterfall, and a few
  Google Sheets exports so that the single-producer tags gain a second
  producer. Added with `scripts/corpus-add.py` once P1 lands, or with
  the private helper before then.
- P1 (implementing session): the two scripts in python3, as the "P1"
  section now says, with the private helper imported through the
  recorded step. P7 follows P5 and P6, or runs with the gaps listed if
  the maintainer prefers to start T3 P4 first (T3 P4 may run before P7
  in any case; the record cites the corpus as it stands).

### Planning rulings on P1 (2026-09-15)

- P1 is verified from primaries by planning's own checks:
  - Doc commit 45dd9e3 holds exactly planning's three doc files (143
    insertions, 30 deletions), unchanged since. Import commit e8087f6
    adds only `scripts/corpus-add.py`, whose blob hashes to the sha256
    its message records, equal to the private helper's hash as planning
    computed it. The two phase commits touch only the two scripts; the
    four record commits only `records/logs/t2-p1/`. All eight messages
    end with the Co-Authored-By trailer only, under the noreply
    identity. Both scripts are mode 100755 with no literal home path.
  - Planning ran the full gate: eight PASS.
  - Planning read both scripts against this spec: the corpus directory
    from `EMEND_CORPUS_DIR`, the mode 700 and work-tree checks, the
    signature and extension refusals, the manifest rules, mode 600 on
    the copy and the manifest, one output line, and no path, name or
    note in any message; the coverage script's validation, output and
    `FAIL` lines.
  - Planning ran `corpus-coverage.py --verify` over the real corpus,
    read-only: 27 files, every hash verified, the same coverage as
    planning's own count (22 tags, `macros` uncovered, 7 single-producer
    tags). On a scratch corpus, planning added a generated workbook
    with a sentinel in its name and note (one `added` line, mode 600,
    `app` from `docProps/app.xml`), was refused for the duplicate and
    for an OLE2 file named `.xlsx`, and saw no sentinel in any script
    output. A first attempt with a malformed `docProps/app.xml` was
    refused, which is right.
  - The six record files, the two scripts and the eight messages have
    zero hits on every scan; the superseded logs at f692dbf hold no
    non-ASCII byte and only the gate script's own path-control lines.
    `scripts/privacy.sh` over the 151 local commits: PASS.
- Check 1's bug (a non-UTF-8 text field reached the manifest write
  after the copy) was real and is fixed in 4318820. Check 2's gaps 1 to
  3 are P7 work: the coverage docstring's line-number claim narrowed to
  the row failures; the add script writing the row before the copy is
  final (copy to a temporary name in `corpus/`, append the row, rename
  into place, remove the temporary file on any failure); cases for the
  file name and `producer_version` on the UTF-8 path. Gaps 4 to 7
  accepted.
- The spec's remaining `.sh` names were planning's oversight; they now
  read `.py`.

### P2: LibreOffice, pinned (CORPUS session)

A Containerfile, kept in the corpus directory's `tools/`, that builds a
headless LibreOffice image from a base image pinned by digest, with the
package version pinned; `soffice --version` recorded; a smoke test that
converts a generated flat ODS to xlsx headless and back, on a fictional
file. The digest, the package version and the conversion command are
recorded privately first, and copied into `records/libreoffice-<date>.md`
by the implementing session at P7.

### P3: LibreOffice exports with fictional content (CORPUS session)

A script generates flat ODS files, one per tag LibreOffice can produce
(at least `plain`, `strings`, `formulas`, `names`, `merge`,
`validation`, `hyperlinks`, `autofilter`, `freeze`, `cf`, `charts`,
`comments`, `images`, `protection`, `date1904`, `large`), converts each
with the P2 image, and adds the results under producer `libreoffice`,
source `libreoffice-export`, with the corpus session's helper (later
`corpus-add.py`). Fictional content
only, so these files could be public, but they are kept in the corpus
like everything else.

### P4: Microsoft templates (CORPUS session)

The corpus session downloads Excel templates from Microsoft's public
template site, adds them (producer from `docProps/app.xml`, `excel-win`
unless the file says otherwise; source `ms-template`), and keeps the
source URLs in the private note column. Target tags: `tables`, `charts`, `cf`, `names`, `strings`,
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
with `corpus-add.py`; the private note may say where it came from.

### P7: coverage check

First the P1 follow-ups (planning rulings on P1): the coverage
docstring, the temporary-file-then-rename order in `corpus-add.py`
with a logged case for a failed append, and the UTF-8 cases for the
file name and `producer_version`. Then `scripts/corpus-coverage.py
--verify` is run over the real corpus and its output committed as
`records/corpus-coverage-<date>.md`, with the LibreOffice pins copied
from the corpus directory's `tools/libreoffice.md` into
`records/libreoffice-<date>.md` if T3 P3 has not done so. Every tag
covered, or the gaps listed for a planning ruling. The `large` tag is
covered (two files over 60 MB uncompressed).

### Planning rulings on P7 (2026-09-16)

- P7 is verified from primaries by planning's own checks and is DONE
  (58063fb to 7b9350a; phase commits 58063fb, 0c3c6a3, 743b6f1, c2493d8
  and 4f4b683; the record 8f5553c and the record-log commits):
  - Doc commit 854dbaa holds exactly planning's three doc files (114
    insertions, 13 deletions). The phase commits touch only the two
    scripts and the coverage record; the record commits only
    `records/logs/t2-p7/`. All twelve messages end with the
    Co-Authored-By trailer only, under the noreply identity.
  - Planning ran the full gate (eight PASS) and `scripts/privacy.sh`
    over the 47 local commits since the last push (294 blobs, PASS),
    and scanned the nine changed files itself: no non-ASCII or control
    byte, no U+2014, no email beyond the scan's `example.invalid`
    control, no pattern hit under the live or the pre-P6d forms, no
    denylist hit, no hostname. Planning's stem scan of the manifest's
    original names against the nine files matched two six-letter
    English words, the names of fictional generated exports, used as
    ordinary prose (one appears in `CLAUDE.md` from before the corpus
    existed) and as a scratch file name; not a finding.
  - Planning ran `corpus-coverage.py --verify` over the real corpus
    itself: exit 0, and its stdout is byte for byte the record's body.
    The record's script hash is the script at 0c3c6a3 and since.
  - Planning reproduced the staged order in a scratch corpus: the
    happy path leaves one file, one row and no temporary; with the
    manifest read-only the add exits 2 naming only `PermissionError`
    and leaves the file count, the row count and the temporaries as
    they were.
  - The four logs hold 107 results, none unexpected; both check
    answers are committed in ASCII with the specs/0001 header.
- Gaps ruled:
  - `macros` uncovered: the maintainer's P6 item, open.
  - Seven single-producer tags (`pivots`, `slicers`, `timelines`,
    `external`, `sparklines` from `excel-win`; `date1904`, `large` from
    `libreoffice`): the maintainer's P5 and P6 items, open. Milestone 1
    starts on the corpus as it is.
  - The `large` claim above ("two files over 60 MB uncompressed") was
    not measured in this phase; planning measured it from the T3 P4
    run's entries: the two `large` files total 66508521 and 66509534
    bytes uncompressed over ten and eleven entries, the largest entry
    63745319 and 63745347 bytes. The claim stands as measured.
  - The LibreOffice-pins clause is satisfied by
    `records/libreoffice-2026-09-15.md` (T3 P3); no new record.
  - The negative tests ran from the main working tree with every write
    under a temporary directory and the tree asserted clean before and
    after; accepted as a departure this once, as in T3 P3. From the
    next phase the scratch-clone form of T3 P4 is the rule, and a
    departure from it needs planning's word before the run.
  - The rename-failure branch of the staged order is present and
    unexercised (planning's ruling: no fault hook; the direction is
    the benign one and the coverage check catches it).
- Findings the implementing session reported against itself stand as
  reported: a class of defect declared closed four times while an
  instance stood (remedied by the shared counting library with its
  controls, run before any form is used); two commit messages (0c3c6a3,
  8f43882) stating measurements no log printed, later named rather than
  superseded; check 2's answer first committed with a false
  "unrecoverable" hash, corrected at 7b9350a. The record's Commit line
  names the run commit 8f43882, the script commit 0c3c6a3 and the
  record commit, by design.
- T2 is DONE apart from the maintainer's P5 and P6 items. The next
  step is the close-out push below.

### Close-out push (after P7)

Everything since 29b0bad (T3 P3, T0 P6d, T3 P4, T2 P1 and T2 P7 with
their records, the T3 close record 24d4a16 and the pending doc updates)
goes to GitHub in one push, on planning's word and the maintainer's
authorisation typed in the implementing session. The push follows the
T0b close-out steps (`specs/0003-ci.md`, "Close-out push (after P2)"):
planning's pending doc updates committed unedited first; the hook,
`core.hooksPath` and `EMEND_PRIVATE_DENYLIST` checks and the full gate;
the base recorded before the push; `git push origin main`; `ls-remote`
equal to local main and the repository still PRIVATE; the CI run on the
pushed commit recorded. Logs in `records/logs/t2-close/`: `push.log`,
`ci-run.log`, `scan.log`, every count in the counting form with a
control. No independent check. Report and STOP. The record commit goes
with the next push.

## Out of scope

- Encrypted workbooks, `.xlsb`, `.xls` (refused by `corpus-add.py`).
- Any automatic tag detection. Tags are set by the person or script that
  adds the file; T3 and Milestone 1 may later add a checker that reads
  entry names to confirm tags such as `charts` or `pivots`.

## Done

T2 is done when P7's coverage record is committed and planning has
ruled on any gaps.
