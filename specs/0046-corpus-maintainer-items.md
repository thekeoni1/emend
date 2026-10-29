# 0046: Milestone 7 part 3, the corpus's maintainer items

Status: APPROVED 2026-10-29 (Milestone 7 part 3; decisions 1 to 4
A, the recommended option in each, typed in the planning session
2026-10-29; drafted the same day). Written by the planning session
for the implementing session.
The P1 brief follows part 2, DONE and PUSHED and PUBLISHED
2026-10-29 (`specs/0045-publish.md`, "Planning verification of P4",
ruling AO), and the maintainer's decisions here.

Milestone 7, "release", was approved 2026-10-25 (`ROADMAP.md`,
"Milestone 7"): its decision 3 takes the corpus's maintainer items,
T2 P5 and P6 of `specs/0005-corpus.md`, as a part after the publish:
"T2 P5 and P6's files added by `scripts/corpus-add.py`, the coverage
record rerun, the passthrough and every sample run over the grown
corpus with every output opened, findings recorded and carried to
the next milestone, not fixed in this one." emend 0.1.0 is on the
registry (part 2 P4). This part grows the corpus with what only the
maintainer can supply, Google Sheets exports (P5) and desktop Excel
files (P6), and measures emend over the grown corpus at one commit.
No code the probes link changes; nothing found is fixed here.

Context: `CLAUDE.md` ("Corpus rules": real workbooks are PRIVATE,
quoted in records as entry names, byte counts, hashes and pass/fail
only, their cell contents never in code, tests, fixtures, comments,
commit messages or reports; "The fidelity harness is the acceptance
instrument"; "Secrets and boundaries"), `specs/0001-bootstrap.md`
("Rules for every phase"), `specs/0005-corpus.md` (the manifest
format, the tag list, "P5: Google Sheets exports (maintainer)", "P6:
desktop Excel files (maintainer)", "P7: coverage check", the P1
rulings on `corpus-add.py`), `specs/0006-passthrough.md` ("The
runner", the open check), `specs/0015-open-checks.md` ("The
open-check library"), `specs/0043-measurement-and-close.md` (the
close-out form; rulings Q and R), `specs/0044-excel-check.md` ("The
reruns (P1)", the form of a sample rerun; rulings AD and Q),
`specs/0045-publish.md` ("Close-out push (after P3)"). No external
text is cited.

## Goal

At the close-out, HEAD holds:

- `scripts/corpus-tags.py`, a read-only tag proposer for a staged
  workbook, with its tests on fictional fixtures;
- the intake log of the maintainer's files: for each, its sequence
  number, extension, byte count, the producer read from
  `docProps/app.xml`, the tags proposed and the tags ruled, and the
  `added` line `corpus-add.py` printed; never an original name, a
  note or a byte of content;
- `records/corpus-coverage-<utc-date>.md`, the coverage record rerun
  over the grown corpus in the 2026-09-16 record's form, every gap
  that remains listed;
- the passthrough record and the 58 sample records over the grown
  corpus at one commit, each compared with its latest committed
  record on the 27 earlier workbooks, the new workbooks' outcomes
  listed; every output opened in the pinned LibreOffice image by the
  runner's open check;
- the findings, each a FINDING line in a run log and a row in the
  P2 record's README, carried to the next milestone's first spec as
  scoping input, none fixed;
- `ROADMAP.md`'s part 3 rows DONE.

The close-out push carries part 2's four local commits (8f742d0,
27da228, 472d361, 39e0c1b) with this part's.

## Size

No code the probes link changes: no size record. The Milestone 2
ceiling stays for the next core milestone's first decision
(Milestone 7 decision 2).

## Facts planning checked (2026-10-29)

- The corpus: `<corpus>` (`/home/dev/emend-testing` on this host,
  mode 700, not inside a git work tree), its manifest of 27 rows
  (17 `libreoffice`, 10 `excel-win`; 26 `xlsx`, 1 `xltx`; no
  `google-sheets` row and no `macros` tag), `<corpus>/runs` of 99
  directories, `<corpus>/work` holding the Excel-check stages. The
  disk has 716 GB free.
- `scripts/corpus-add.py`: `<file> <producer> <producer_version>
  <source> <tags> [note]`; producers `excel-win`, `excel-mac`,
  `google-sheets`, `libreoffice`, `unknown`; sources
  `libreoffice-export`, `ms-template`, `google-sheets-export`,
  `maintainer`; 23 tags; extensions `xlsx`, `xlsm`, `xltx`, `xltm`.
  It prints one line, `added <id> <ext> <bytes> <producer> <tags>`,
  never the original name, the note, the corpus path or content;
  refusals on stderr with exit 1. The `app` column is read from
  `docProps/app.xml` by the script itself.
- `scripts/corpus-coverage.py --verify` wrote
  `records/corpus-coverage-2026-09-16.md` (commit 8f43882, 27
  files); its stdout holds ids, extensions, byte counts, producers,
  sources, tags and counts only.
- `tools/emend-copy`'s usage text names 59 flags ending in
  `-sample`; less `--new-sample`, 58 were run in part 1 P1 at
  82c07fb (`records/m7p1-<name>-2026-10-25-82c07fb.md`) and six of
  them again in part 2 P1 at 43471ed (`records/m7p2-<name>-
  2026-10-27-43471ed.md`); the runner `scripts/passthrough-run.sh`
  with `--label host`, `--tool-arg --<name>-sample`, the pinned
  image `localhost/emend-lo:25.2.3-2-deb13u6`, `--lo-timeout 600`,
  `--timeout 300`, recorded by `scripts/passthrough-record.py
  --run <dir> --stem <stem>`.
- The passthrough tool `tools/passthrough` is the runner's default
  tool; its records are `step0-<run>` by default and `--stem`
  renames them.
- The privacy scan's patterns name no document property; a
  workbook's `docProps/core.xml` is never printed by the runner,
  the records or the tag proposer. Committed fixtures are fictional
  (`CLAUDE.md`); nothing from the maintainer's files becomes one.

## The intake and the additions (P1)

The maintainer puts the files in `<corpus>/work/incoming/`, a
directory the implementing session makes first (mode 700, empty)
and names to the maintainer in its session as that path. The
maintainer copies the files there from their own machines, under
any names; a file name is private like a note and is never printed.
The maintainer then types in the implementing session that the
files are in, and for each file, in the directory's sorted order or
by a sequence number the intake listing prints, the producer
(`google-sheets`, `excel-win` or `excel-mac`) and the source
(`google-sheets-export` or `maintainer`) if the proposer's reading
of `docProps/app.xml` is to be overridden, and any note.

`scripts/corpus-tags.py <file>` reads one workbook, never writes,
and prints one line: the extension, the byte count, the producer
proposed from `docProps/app.xml`'s `Application` (`Microsoft Excel`
with a `Windows` or `Macintosh` platform hint to `excel-win` or
`excel-mac`, a missing or other value to `unknown`; Google Sheets
exports carry no `app.xml` or a generic one, so the proposer says
`unknown` and the maintainer's word names `google-sheets`), and the
tags proposed from the parts and elements present: `plain` always;
`strings` on `xl/sharedStrings.xml`; `formulas` on an `<f` element
in any worksheet; `names` on `<definedName`; `merge` on
`<mergeCell `; `validation` on `<dataValidation `; `hyperlinks` on
`<hyperlink `; `autofilter` on `<autoFilter`; `freeze` on
`state="frozen"`; `cf` on `<conditionalFormatting`; `tables` on
`xl/tables/`; `charts` on `xl/charts/` (a chartex part under it
counts as `charts`; the proposer prints `chartex` as a word beside
the tags, not as a tag); `pivots` on `xl/pivotTables/`; `slicers`
on `xl/slicers/`; `timelines` on `xl/timelines/`; `macros` on
`xl/vbaProject.bin`; `comments` on `xl/comments` or
`xl/threadedComments/` (threaded printed as a word beside the
tags); `images` on `xl/media/`; `external` on `xl/externalLinks/`;
`sparklines` on `sparklineGroup`; `protection` on
`<sheetProtection` or `<workbookProtection`; `date1904` on
`date1904="1"` or `"true"` in `xl/workbook.xml`; `large` when the
entries' uncompressed sizes sum above 60000000. The proposer reads
entry names and matches elements by name; it prints no attribute
value, text or name from the file. Its tests are fictional
fixtures built in the test, one per tag, and a refusal test for a
file that is not a zip. A tag the maintainer names beyond the
proposal is taken; a proposed tag the maintainer strikes is struck;
planning rules on the final tags per file in the intake ruling.

Then, per file, `scripts/corpus-add.py <file> <producer>
<producer_version> <source> <tags> [note]` once, the `added` line
copied into the intake log, and the file removed from the incoming
directory by `rm` of that one path (never `rm -r`). A refusal is a
FINDING: recorded, the file left, planning rules. After the last
addition `scripts/corpus-coverage.py --verify` runs and its stdout
becomes `records/corpus-coverage-<utc-date>.md` in the 2026-09-16
record's form (the commit, the script's sha256, the command, the
body); the intake log prints the tags still covered by one
producer and the tags uncovered, for the record's "gaps" list.

The intake log, the coverage log and the negative log (the
proposer's refusal on a non-zip file; `corpus-add.py`'s refusals on
an unknown tag and an unknown source, as `specs/0005` P1 showed
them) are this phase's logs. The run is by the generated logged
script, a dry run first with fictional fixtures in a directory of
its own standing for the incoming directory (`specs/0043` ruling
Q), the logged run once over the maintainer's files.

## The runs over the grown corpus (P2)

At P2's phase commit, on the host, as part 1 P1's "The reruns":
`scripts/passthrough-run.sh` with the passthrough tool (stem
`m7p3-passthrough-<utc-date>-<shortsha>`), then every flag ending
in `-sample` that `emend-copy`'s usage text names less
`--new-sample`, each with `--label host`, `--tool-arg
--<name>-sample`, the pinned image, `--lo-timeout 600`, `--timeout
300`, `TMPDIR` under the run's working directory on the disk, into
`<runs>/<utc-date>-<shortsha>-<name>`, recorded as
`records/m7p3-<name>-<utc-date>-<shortsha>.md` and `.txt`. The run
log prints the flag list as the usage text gives it, the exclusion,
the directory each run wrote and its outcome counts from
`summary.host.tsv`, and asserts every `run.host.tsv` names the phase
commit and its flag. The runner's open check opens every output in
the pinned image (`specs/0006` P3); no Excel run (decision 3).

Each record is compared with the sample's latest committed record
(the part 2 P1 record for the six reruns at 43471ed, the part 1
P1 record for the rest; the passthrough with
`records/step0-2026-09-16-39defb7.md`, the latest passthrough
record) on the 27 earlier ids by outcome per id: a changed outcome
is a FINDING, recorded, not fixed. The new ids' outcomes are listed
per sample; an error, a refusal or an open-check failure on a new
id is a FINDING, recorded, not fixed (Milestone 7 decision 3). The
README of the record commit tables the findings by sample and id
with the outcome words the summary gives, for the next milestone's
first spec to read; it names no cell content and no original name.

The i686 label is not run (part 1's reason). The scale tiers and the
openpyxl comparison are not run.

## Close-out push (after P2)

As part 2's (`specs/0045`, "Close-out push (after P3)") with these
differences: the base is part 2's close-out doc commit cf727f7 by
its subject ("M7 part 2 close-out: commit planning docs (ruling AJ
and decision 7)"); `PREV_RECORD_PATTERN` is `^M7 part 2 close-out
record` (8f742d0), in the range exactly once; the close-out doc
commit's subject is "M7 part 3 close-out: commit planning docs (the
P2 verification and rulings)"; `CARGO_TARGET_DIR` `<home>/.cache/
emend-build/m7p3-close/target`; the two Excel-record checks in pre
mode read exactly one `records/excel-check-m7-*.md` and exactly one
`records/excel-check-m7p2-*.md`, unchanged; no tag is created and
`git push origin main` pushes the branch alone, after the
maintainer's typed authorisation in the implementing session; the
post scan runs over the range; no whole-history scan. The record
commit "M7 part 3 close-out record: the push, the CI run and the
scan" holds the three logs only. One independent check before the
report. Part 2's P4 record and P4-close doc commit go with this
push; the ROADMAP's milestone heading then reads DONE and PUSHED.

## Harness runs

P1: none (the coverage script). P2: the passthrough and the 58
samples over the grown corpus, every output opened.

## Phases

Each phase as `specs/0001` "Rules for every phase": a doc commit
first, the code commits if any, one record commit, the scan last,
at most two independent checks, the report under `<phase>`, then
stop. Target root `<home>/.cache/emend-build/m7p3-pN/target`. The
gate generator is part 2's with the part's names, as a numbered
change; its prune lists and removes part 2's P1 to P4 and close-out
build targets at P1's gate by `rm -r` with the control
(`specs/0044` ruling AD). A dry driver sets every build root under
the dry run's own directory (`specs/0043` ruling Q); every dry and
logged run report lists every FINDING line found by grepping the
word (ruling R; `specs/0044` ruling Q).

### P1: the intake and the additions

The doc commit holds this spec and `ROADMAP.md` (part 3's block,
its P1 row IN PROGRESS), on 39e0c1b. One code commit for
`scripts/corpus-tags.py` and its tests; the incoming directory
made and named to the maintainer; the maintainer's files and word;
the dry run; the logged run (the intake, the additions, the
coverage record); the record commit (the coverage record, the
logs, the check); the scan.

### P2: the runs over the grown corpus

The doc commit (P1's verification and rulings; the P2 row IN
PROGRESS). No code commit. The run log; the records; the record
commit (the records, the logs, the README of findings, the check);
the scan.

## Decisions for the maintainer

Decided by the maintainer 2026-10-29: 1 to 4 as recommended (A in
each), typed in the planning session.

1. The intake: A (recommended), the maintainer copies the files
   into `<corpus>/work/incoming/` and names the producer and source
   per file in the implementing session where the proposer's
   reading is to be overridden; `scripts/corpus-tags.py` proposes
   the tags, planning rules them. B, the maintainer writes producer,
   source and tags by hand per file and no proposer is written.
2. What goes in: A (recommended), whatever P5 and P6 files the
   maintainer can supply now, the part running over them, and the
   tags still uncovered or single-producer listed in the coverage
   record as gaps; B, the part waits until every item of `specs/0005`
   P5 and P6 is in hand.
3. The open check: A (recommended), the runner's LibreOffice open
   check over every output, no Excel run in this part, the new
   workbooks joining the next Excel check; B, an Excel check over
   the new workbooks' outputs staged as part 1's and run by the
   maintainer, gating nothing.
4. The maintainer's own workbooks' document properties: A
   (recommended), added as they are; `docProps/core.xml` is never
   printed by any script or record, the original name and the note
   stay in the private manifest, and no file of the maintainer's
   becomes a committed fixture. B, the maintainer clears the
   creator and last-modified-by fields before staging, which alters
   the inputs under test.

## Out of scope

- Any fix for what the runs find (Milestone 7 decision 3): findings
  are carried to the next milestone's first spec.
- A new tag in `corpus-add.py`'s list (`chartex` and threaded
  comments are words in the intake log under `charts` and
  `comments`); a change to `corpus-coverage.py`.
- The i686 label; the scale tiers; the openpyxl comparison; the
  Milestone 2 ceiling.
- Any fixture, test or comment holding a byte of a maintainer's
  workbook; any committed file naming an original file name or a
  note.
- The xlsx-diff, passthrough and privacy-gate backlogs and the MSRV
  policy under "Unscoped".
