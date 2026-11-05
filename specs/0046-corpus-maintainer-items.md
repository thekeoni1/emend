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

## Planning rulings during P1 (2026-10-29)

A. The doc commit 1af3f79 (parent 39e0c1b; this spec 2fdeb4b3,
   `ROADMAP.md` 5f8fbfea, 27da228's form) and the code commit
   f8f4ee1 (`scripts/corpus-tags.py` 9905d7c8, `scripts/corpus-
   tags-test.py` ccc1e689; both ASCII) read from git; the test run
   by planning with `python3 -I -B`: 87 ok, 0 FAIL, "corpus-tags-
   test: PASS"; the proposer has no write, rename, remove or child
   process; its stdout is the one line. The four points refine
   "The intake and the additions (P1)" and are accepted as its
   reading: (a) the producer from `Application` exactly "Microsoft
   Excel" to `excel-win` and exactly "Microsoft Macintosh Excel" to
   `excel-mac`, anything else or no `docProps/app.xml` to `unknown`
   (the spec's "platform hint" was planning's guess; Excel for
   Windows writes no platform word), the value never printed; (b)
   the worksheet elements matched by local name in any namespace
   under `xl/worksheets/`, the formula `f` in the transitional or
   strict main namespace only, `pane` with `state="frozen"`, and in
   `xl/workbook.xml` `definedName`, `workbookProtection` and
   `workbookPr` `date1904` "1" or "true"; a table part's own
   `autoFilter` not counted; (c) `chartex` the word on an entry
   `xl/charts/chartEx*.xml`, `comments` on an entry starting
   `xl/comments` or under `xl/threadedComments/`, `threaded` the
   word on the latter; (d) an extension not one of the four printed
   as `other`; a non-zip, an unreadable zip or a malformed scanned
   part refused with one "refused: " line and exit 1; any other
   error "error: <class>" and exit 2. `<corpus>/work/incoming/`
   exists, mode 700, empty, its path given to the maintainer in the
   implementing session. The generator and its dry runs are next;
   the logged run waits on the maintainer's files, the intake
   listing and planning's tag ruling.

B. Planning's word on P1's logged stage 1 (2026-10-29). The dry runs
   under `<phase>/dry` read from the phase directory: d2 pos, the
   four modes exit 0 and "checks all as expected: yes", 3 fictional
   workbooks added to a dry corpus (mode 700, under the dry
   directory, never `<corpus>`), the stand-in emptied, the manifest
   2 to 5 rows, coverage 31 lines with its uncovered and
   single-producer lines; d2 neg, list exit 1 with 3 FINDING lines
   and add exit 1 with 6 (the refused proposer, a directory, an id
   in the corpus already, a corpus-add.py refusal, a ruled row
   naming no entry or one twice, a sha256 not the ruling's), 4
   entries left, the manifest unchanged; every stderr 0 bytes; no
   home path in any d2 log; d1's helper defect (`test` for `[`)
   fixed and d2 rerun, d1 kept under `superseded`. The gate g3 exit
   0, "gate log checks all as expected: yes", no FINDING line; the
   dry scan FAIL on P3's two dry-layout checks alone, hits 0,
   errors 0. The generator intake.py b13020e6 with intake.tmpl
   a163c0f1 (script cc740945, P3's run script 144ecba7 the frame,
   changes PI1 to PI7), gate.py 58444181 (script d9d584c5, series
   96), scan.py 2220adbd (script d362aaee, changes MC to MF). The
   real corpus is untouched: the incoming directory empty, the
   manifest 27 rows. Part 2 P4's dry build roots (1.4 MB each)
   stay, as `specs/0044` ruling R keeps dry roots; accepted. The
   word is given for logged-1.sh (a1cca474) once, after the
   maintainer types in the implementing session that the files are
   in: the gate with PRUNE_PREVIOUS=yes removing part 2's seven
   build roots by `rm -r "${PREV_DIR:?}/${n:?}"` with the control
   (`specs/0044` ruling AD), then the listing. The listing comes to
   planning for the tag ruling; `<phase>/ruling.tsv` (mode 600,
   never committed, its notes never printed) is written from that
   ruling and the maintainer's words; logged-2.sh waits for a
   second word.

C. The intake in two rounds (2026-11-02). The maintainer uploaded
   an archive of five Google Sheets exports to the host (273060
   bytes, sha256 c17258325fc38c4a...) and asked planning, in the
   planning session, to start on those first; the desktop Excel
   files (P6) follow. Planning unpacked the archive by `python3 -I
   -B` with the zip module into `<corpus>/work/incoming/` (five
   `.xlsx` members, one folder level, no traversal, each written
   mode 600 with O_EXCL, each a zip; sizes 6783, 12205, 225468,
   14161 and 35370 bytes; the archive itself is not in the corpus),
   the one deviation from "The intake and the additions (P1)",
   where the maintainer copies; the maintainer's typed request in
   the planning session is the authority for it, and the words
   "those are the google sheets" are the producer and source for
   all five: `google-sheets`, `google-sheets-export`, no note,
   unless the maintainer types otherwise in either session before
   the ruling.tsv is written. The producer is the maintainer's
   word, not the proposer's `unknown` from a Sheets export's
   `docProps/app.xml`. Round 1 is these five: logged-1.sh (ruling
   B's word stands; the gate with the prune, then the listing),
   planning's tag ruling, `<phase>/ruling.tsv`, then a driver for
   INTAKE_MODE=add alone (logged-2.sh is split: its add mode runs
   per round, its coverage and negative modes run once after the
   last round as logged-3.sh). Round 2 is the P6 files when they
   are in: the listing again (the sequence numbers of a listing are
   that listing's; the add log names the round), planning's tag
   ruling, the add driver again, then logged-3.sh, the record, the
   commit and the scan as briefed. A second word is given per
   driver. The maintainer's confirmation that round 1's files are
   in is this ruling; the implementing session need not wait for
   it to be typed there.

D. Stage 1's first logged run, a FINDING (2026-11-02). The maintainer
   typed the prune line in the implementing session; logged-1.sh
   (a1cca474) ran once, 15:04:03Z to 15:16:32Z at HEAD f8f4ee1,
   PLANNING_SPEC 5255cacd. The gate exited 1, "gate log checks all
   as expected: no", one UNEXPECTED line and no other, read by
   planning from `<phase>/logged/gate.log` (9eea6b27, 861875 bytes)
   line 459: the change C96 control "the same check refuses M7 part
   2 P1's directory, whose target root is there before a yes run".
   The control reads a live directory that the yes run's own prune
   had already emptied; the dry run g3 ran with PRUNE_PREVIOUS=list,
   where that root was still there, so the control never met the
   yes order. The defect is in the control; the prune's own checks
   are as-expected and the prune stands: six target roots removed
   by `rm -r` exit 0, each confirmed gone (part 2 P1's two
   8757835318 bytes, P2's one 6227782648, P3's two 6697145547, the
   close-out's one 2146536745; 23829300258 bytes in all), the
   control R90 as-expected, part 2 P4's directory holding no target
   root at its top and nothing removed there (ruling B's "seven"
   counted P4; six is the number). Planning confirmed the six paths
   gone and the kept names there. No FINDING line in gate.log; the
   listing did not run; the incoming directory is untouched, five
   files. A second defect in the driver: logged-1.sh copies the
   gate's stderr from under `<phase>/scripts/out/` while the gate
   writes `<phase>/out/gate.stderr`, so `driver.out` (f5c280f6)
   holds two cp error lines and an empty stderr size; the gate's
   own stderr is 0 bytes. The fix, named: (a) the C96 control
   plants a target-shaped directory under the dry build root, as
   R90 does, instead of reading part 2 P1's directory, as the next
   numbered change in gen/gate.py, with dry runs in both list and
   yes mode against fixture directories under the dry run's own
   directory (`specs/0043` ruling Q); (b) the driver's cp path is
   `<phase>/out/gate.stderr`; (c) the yes run's gate.log,
   driver.out and gate.outer.stderr move to
   `<phase>/superseded/gate-yes-1/` (by `mv`, kept, named in the
   record's README with their sha256s as the run that made the
   prune and failed on the control), `<phase>/logged` is empty
   again, and logged-1.sh reruns once with PRUNE_PREVIOUS=list,
   which deletes nothing (no maintainer line is needed for it), so
   the gate lists the six roots gone and the listing runs. The
   word for that rerun follows planning's reading of the dry runs.
   Also ruled here: the split drivers logged-2-add.sh 206f4763
   (round N reads list-rN.log and ruling-rN.tsv, round 1 list.log
   and ruling.tsv; the add mode binds each ruling row to a file by
   its sha256), logged-3.sh 85f8a61e (coverage then negative, once,
   after the last round) and logged-2-list.sh ca4982b5 (the list
   step alone for rounds 2 to 9) are verified from the phase
   directory and accepted; the superseded logged-2.sh b75e63f6 is
   kept. The implementing session's disclosed slip (fragments of
   four file names in its own tool output from a `find -printf`
   listing; nothing in a log, record, file or message; redone with
   `python3 -I -B` printing sequence, mode, size and extension) is
   accepted as disclosed; the pattern is in gotchas.md.

E. Ruling D's fix verified; the word for stage 1's rerun
   (2026-11-02). Read from the phase directory: gen/gate.py
   e4d7c050 regenerates scripts/m7p3-p1-gate.sh 90ff9be3 (8429
   lines, series 97); change C97 is (a): the control reads a
   target-shaped directory the gate makes under its own build root
   in list and yes mode alike. Two changes beyond (a) to (c),
   disclosed and accepted: within C97 the generator no longer exits
   on a prune root that is not a directory, it prints the root as
   removed and skips it (the kept-name lists identical, diffed by
   the implementing session), so the generator runs at all now
   that the six roots are gone; change A97 reads part 2's
   directories under GATE_PREV_ROOT, set by a dry driver alone to
   fixtures (the input line prints whether it is set; logged-1.sh
   unsets it), without which a yes-mode dry run could touch only
   the real directories. logged-1.sh 12abf602 (the old a1cca474
   kept under superseded): the cp path (b), PRUNE_PREVIOUS=list
   (c), GATE_PREV_ROOT in the unset line, and the refusal now on a
   non-empty `<phase>/logged`. The yes run's three files are under
   `<phase>/superseded/gate-yes-1/` (gate.log 9eea6b27, driver.out
   f5c280f6, gate.outer.stderr empty) and `<phase>/logged` is empty
   (0 entries). An intermediate generator (gate-gen3, gate.py
   b73cf68b) had a nested parameter expansion that bash -n accepts
   and bash refuses at run time: dry g4 (list) and g5 (yes) each
   exited 1 on the stderr check alone (262 bytes), their prune and
   C97 lines as-expected; fixed as an if/else, g4 and g5 kept, the
   pattern in gotchas.md. The dry driver dry-gate3.sh ec536c57
   builds fixtures beside the dry directory (`dry/g<n>-prev`: part
   2's five directories by their top-level names, a target-shaped
   root at each of the six prune places), under spec afd38b76 and
   the intake dry directory d2/pos. g6 (list, gate.log b8ff65f6)
   and g7 (yes, gate.log aa7b4fea): gate exit 0, "gate log checks
   all as expected: yes", 0 UNEXPECTED, no FINDING line, every
   stderr 0 bytes, the C97 control as-expected in both; g6 "with
   list, every name of the prune set is still there" for the four
   directories, g7 every `rm -r` exit 0 and every prune set gone;
   the dry scan FAIL on P3's two dry-layout checks alone, as g3.
   The real rerun's path, list mode with the six roots gone (0 of
   N there, the present_all note skipped, kept_all read on the real
   directories as the yes run already did after its prune), is not
   dry-run further: the one skipped note asserts nothing, and the
   rest g6 took. The word is given for logged-1.sh (12abf602) once,
   PRUNE_PREVIOUS=list, deleting nothing, no maintainer line
   needed, with PLANNING_SPEC the hash of this spec as it stands
   with this ruling. The listing then comes to planning for the
   tag ruling (ruling F).

F. The tag ruling for round 1 and the word for its additions
   (2026-11-02). Stage 1's rerun read from `<phase>/logged`:
   driver.out bc007e65; gate.log 655ccce4 (861186 bytes), "gate
   exit 0; gate log checks all as expected: yes", 0 UNEXPECTED, the
   four prune sets "0 of N there" and part 2 P4's directory
   as-expected; list.log 393be9f3 (59553 bytes), "list exit 0; list
   log checks all as expected: yes", "FINDING lines printed above:
   0"; the four stderr files 0 bytes; the tree the spec alone. The
   listing: five entries, each a file, none in the corpus, each
   proposer line of its form with exit 0 and 0 bytes of stderr; the
   sha256s are the five planning wrote at ruling C. Planning's own
   read of each file's parts and elements (entry names and element
   names only, by `python3 -I -B` with the zip module; nothing
   printed from any file) agrees with the proposer on every tag;
   every file has no `docProps/app.xml` (the manifest's `app` will
   read `none`), consistent with a Sheets export, and the producer
   is the maintainer's word (ruling C). The tags are ruled as
   proposed, the word `threaded` on entry 4 being the proposer's
   note on its `comments` tag (the part under
   `xl/threadedComments/`), not a tag:
   1. 91d2bdffa46a, 6783 bytes: plain,strings,formulas,freeze
   2. 90a5b01f9541, 12205 bytes: plain,strings,formulas,hyperlinks
   3. 453a166dcdfe, 225468 bytes: plain,strings,formulas,freeze,
      images
   4. 15205770ae5c, 14161 bytes: plain,strings,formulas,freeze,
      comments
   5. 6e0be3d0f2b1, 35370 bytes: plain,strings,formulas,hyperlinks,
      freeze
   Each row: producer `google-sheets`, producer version `unknown`
   (the manifest's free text or `unknown`; a Sheets export carries
   no version), source `google-sheets-export`, no note. The ruling
   file `<phase>/ruling.tsv` (mode 600) is those five rows in the
   intake script's column order (sequence, sha256, producer,
   producer version, source, tags), tab-separated, newline-ended,
   no note column: sha256 1b61e173450d0600
   9854674aadd906718ed7f76e44aebf37b57a736c5451b0ff. None of
   `specs/0005` P5's `cf`, `validation`, `merge`, `charts` or
   `names` is in these five; that stays a gap for the coverage
   record unless round 2 brings a Sheets export with them. The word
   is given for logged-2-add.sh (206f4763) round 1 once, with
   PLANNING_SPEC the hash of this spec as it stands with this
   ruling, after the ruling file is written at that sha256. The
   five files are then removed from the incoming directory by the
   add mode (`rm` of each one path); logged-3.sh waits for round
   2's additions.

G. Round 1's additions verified (2026-11-02). Read from
   `<phase>/logged` and the corpus: ruling.tsv mode 600, 731
   bytes, sha256 1b61e173 as ruling F gives; add-r1.log 02665c31
   (63695 bytes), "add-r1 exit 0; add log checks all as expected:
   yes", 0 UNEXPECTED, the five `added` lines as corpus-add.py
   printed them (lines 849 to 877: each id, `xlsx`, the bytes,
   `google-sheets` and the ruled tags), each entry "removed from the
   incoming directory: yes", "ruled rows: 5; added: 5; refused: 0;
   manifest rows before 27, after 32", "entries left in the
   incoming directory: 0", "FINDING lines printed above: 0";
   driver-2-r1.out c6bbfc6c; both stderr files 0 bytes; the tree
   the spec alone. The manifest holds 32 rows (17 libreoffice, 10
   excel-win, 5 google-sheets); the corpus directory 32 files; each
   of the five copies has the sha256 and byte count of its entry;
   the incoming directory is empty. Round 1 is complete. Round 2
   waits on the maintainer's desktop Excel files (and, if the
   maintainer chooses, one more Sheets export carrying `cf`,
   `validation`, `merge`, `charts` and `names`); then
   logged-2-list.sh round 2, the tag ruling, the round-2 add, and
   logged-3.sh, each on planning's word.

H. Round 2 received (2026-11-05). The maintainer's second zip
   (65078 bytes, sha256 93d3036dfe3ded17 76bd272c8061115058bff804
   ab89ef05ca6fad7dbc584fab) read by planning with python zipfile:
   8 entries, one directory and seven files, no path traversal.
   Planning unpacked the seven flat into the incoming directory
   (each copy mode 600; the directory was empty before and holds 7
   after). Each file, in name order, with its bytes and sha256:
   1. xlsm, 13504: 4bd861696651fc80 6469412eca99aa6ff6a72844ed94da
      323e3b9f4973847116
   2. xlsx, 10297: 7055450ada76c821 ad84529dd48286afb7f46391c995a0
      56758952422230fc84
   3. xlsx, 17800: 495c7b79b183135e 426293e9a6e8159f365330e8a5ed8a
      3dbc39ec2f7616b262
   4. xlsx, 9245: 0d7a34c16e9d1905 5839be41e5561e848072157385db3a
      28716b4e37b6ec4989
   5. xlsx, 13632: 9de2ee20fc6aea65 5677b4fdf446f50721fa6d10652e5c
      bb09781de2fbfd50da
   6. xlsx, 10176: 9956034c43267017 df1352d421058f7b2c96b1f999c635
      957d64596a197b5888
   7. xlsx, 8549: 2adafe76df11eab9 21dffe46207b860d0d3f8167fd0052
      b112bb21ec5984bd79
   Every one carries `docProps/app.xml` with `Application` exactly
   "Microsoft Excel" and `AppVersion` 16.0300, so the producer is
   `excel-win` by ruling A(a) for all seven; the maintainer sent no
   Excel for Mac file, no workbook of their own and no further
   Sheets export, so the P5 gap ruling F names stays. The word is
   given for logged-2-list.sh (ca4982b5) round 2 once, with
   PLANNING_SPEC the hash of this spec as it stands with this
   ruling. The tag ruling for round 2 follows the listing as
   ruling I.

I. Round 2 tags (2026-11-05). The list run read from `<phase>/logged`:
   list-r2.log a9a56665 (60132 bytes), "list-r2 exit 0; list log
   checks all as expected: yes", "FINDING lines printed above: 0",
   both stderr files 0 bytes, driver-list-r2.out c35267a3, the tree
   the spec alone. The seven entries (log lines 823 to 848) carry
   the sha256 and byte count ruling H gives, none is in the corpus
   already, and the proposer's line on each agrees with planning's
   own zip-module read (entry names and element names only). The
   tags are ruled as proposed, the word `chartex` on entry 5 being
   the proposer's note on its `charts` tag (the part
   `xl/charts/chartEx1.xml`), not a tag; entry 2's `comments` is a
   legacy note (`xl/comments1.xml` with a VML drawing, no
   `xl/threadedComments/` part), which is what Excel wrote and
   covers the tag:
   1. 4bd861696651, 13504 bytes, xlsm: plain,strings,macros
   2. 7055450ada76, 10297 bytes: plain,comments
   3. 495c7b79b183, 17800 bytes: plain,strings,names,pivots,
      slicers,timelines
   4. 0d7a34c16e9d, 9245 bytes: plain,strings,sparklines
   5. 9de2ee20fc6a, 13632 bytes: plain,strings,names,charts
   6. 9956034c4326, 10176 bytes: plain,strings,formulas,external
   7. 2adafe76df11, 8549 bytes: plain
   Each row: producer `excel-win` (ruling H), producer version
   "16.0300 (AppVersion)" as the manifest's ten ms-template rows
   write it, source `maintainer`, no note (the maintainer gave
   none). The ruling file `<phase>/ruling-r2.tsv` (mode 600) is
   those seven rows in the intake script's column order (sequence,
   sha256, producer, producer version, source, tags), tab-separated,
   newline-ended, no note column, 934 bytes: sha256
   100f73f814330cb6 f55d2484d7b9ff5fe88bcae7dd3886f9d21531f9c4a0
   e11a. After this round `macros` is covered (one file), `charts`
   has its first chartEx part, and `comments` its first Excel-
   written note; pivots, slicers, timelines, external and
   sparklines stay single-producer (excel-win), date1904 and large
   single-producer (libreoffice). The word is given for
   logged-2-add.sh (206f4763) round 2 once, with PLANNING_SPEC the
   hash of this spec as it stands with this ruling, after the
   ruling file is written at that sha256. The seven files are then
   removed from the incoming directory by the add mode; logged-3.sh
   follows on a further word.

J. Round 2's additions verified (2026-11-05). The implementing
   session's first ruling file wrote the producer version as
   `16.0300` alone (843 bytes, sha256 56dd3766); its hash check
   against ruling I caught it before any run, the file was moved
   to `<phase>/superseded/ruling-r2.tsv.56dd3766` (mode 600, read
   by planning at that size and hash), and the file as run was
   written afresh: `<phase>/ruling-r2.tsv` mode 600, 934 bytes,
   sha256 100f73f8 as ruling I gives. Read from `<phase>/logged`
   and the corpus: add-r2.log f18a2fb6 (65420 bytes), "add-r2 exit
   0; add log checks all as expected: yes", 0 UNEXPECTED, the seven
   `added` lines as corpus-add.py printed them (lines 857 to 899:
   each id, its extension, the bytes, `excel-win` and the ruled
   tags), each entry "removed from the incoming directory: yes",
   "ruled rows: 7; added: 7; refused: 0; manifest rows before 32,
   after 39", "entries left in the incoming directory: 0", "FINDING
   lines printed above: 0"; driver-2-r2.out 652468cf; both stderr
   files 0 bytes; the tree the spec alone. The manifest holds 39
   rows (17 libreoffice, 17 excel-win, 5 google-sheets); the corpus
   directory 39 files; each of the seven copies has the sha256 and
   byte count of its row, the row's producer version "16.0300
   (AppVersion)", `app` "Microsoft Excel 16.0300", source
   `maintainer` and an empty note; the incoming directory is
   empty. Round 2 is complete and the intake is closed at 39. The
   word is given for logged-3.sh (85f8a61e) once, coverage then
   negative, with PLANNING_SPEC the hash of this spec as it stands
   with this ruling; the coverage record is then written as
   `records/corpus-coverage-2026-11-05.md` (or the UTC date of the
   run) in the form of `records/corpus-coverage-2026-09-16.md`.

K. The coverage and negative runs and the record verified
   (2026-11-05); the record commit ruled. Read from
   `<phase>/logged`: coverage.log 7261f0c1 (63646 bytes), "coverage
   log checks all as expected: yes", "rows at entry: 39", the
   coverage script ff02af14 at f8f4ee1 with `--verify`, exit 0, no
   stderr, stdout 65 lines sha256 31fa6d7e (planning hashed the 65
   lines from `files 39` to the single-producer line straight from
   the log: equal), uncovered 0, single-producer 8 (pivots,
   slicers, timelines, macros, external, sparklines, date1904,
   large); negative.log 86f3420c (59054 bytes), "negative log
   checks all as expected: yes", the three refusals in `specs/0005`
   P1's words in a scratch corpus under EMEND_CORPUS_DIR, the two
   acceptance controls; FINDING 0 in each; the four stderr files 0
   bytes; driver-3.out 50e89dd5. The record
   `records/corpus-coverage-2026-11-05.md` (6284 bytes, sha256
   2e1d4ad6, 0 non-ASCII bytes) is in the 2026-09-16 form; its
   fenced body hashes to 31fa6d7e over 65 lines, so it is that
   stdout byte for byte; its "Gaps" line is the two lines as
   printed; its forward reference to
   `records/logs/m7p3-p1/coverage.log` is accepted, that path
   being in the same commit. The intake's manifest and corpus
   counts after the negative run (39 and 39) are the implementing
   session's own count plus the coverage log's `rows at entry`;
   accepted, since the negative mode writes only under its scratch
   corpus. The record commit, on f8f4ee1, holds: the record;
   `records/logs/m7p3-p1/` with gate.log (655ccce4, the list-mode
   rerun that passed), gate-prune.log (the superseded yes run
   9eea6b27, 861875 bytes, whose prune removed the six part 2 build
   roots before it failed on the C96 control; ruling D), list.log
   (393be9f3), add-r1.log (02665c31), list-r2.log (a9a56665),
   add-r2.log (f18a2fb6), coverage.log (7261f0c1), negative.log
   (86f3420c), README.md, independent-check-1.md and scan.log; no
   ruling file, no driver output. The README (ASCII) names each
   log with its sha256 and byte count and what it is, the two
   rounds, the two slips of the phase (the yes run's C96 failure
   and fix, ruling D; the first round-2 ruling file 56dd3766
   superseded before any run, ruling J), the superseded generator
   and driver versions by hash, and the dry runs g4/g5's failure
   (ruling E). The scan runs with EXPECT_RECORDS=1 on the tree as
   it will be committed (the spec's working copy excluded as the
   uncommitted planning document it is), after the check; one
   independent check for this phase (`CLAUDE.md`'s three
   questions, given the spec, the diff of the record commit, the
   draft report, the gate and scan logs). The commit message, in
   85dd323's form: "M7 part 3 P1 record: the coverage record, the
   logs, the check, the scan". The report under `<phase>`.

L. The record commit verified (2026-11-05) and the three disclosed
   deviations ruled. Read from git: 1113530 (full
   11135308a4c292bbabd609d227e0017dd0cc5dd0, parent f8f4ee1; 7
   local commits ahead of origin/main), the message in 85dd323's
   form, ASCII, ending with the Co-Authored-By line alone; 12 new
   files, 34985 insertions, each at the sha256 and byte count the
   implementing session reported (the record 2e1d4ad6, README
   24b66453 8462, add-r1 02665c31, add-r2 f18a2fb6, coverage
   7261f0c1, gate-prune 9eea6b27, gate 655ccce4, check 1 7af28ed0
   10300, list-r2 a9a56665, list 393be9f3, negative 86f3420c, scan
   e3c3e7b9 467578); the tree the working spec alone. scan.log:
   "check answers present: 1", "total hits: 0", "structural checks
   failed: 0", "RESULT: PASS". (1) Scan change MG (gen/scan.py
   2220adbd to 3071674d, script d362aaee to 87ef2284, 5619 lines;
   old copies in `<phase>/superseded/scan-gen1`): the sibling list
   grows from MD's five to ruling K's eight; read in the committed
   scan log's printed script (SIBLINGS names the eight) and
   accepted. (2) The committed check answer is verbatim but for
   one edit its heading names: the ruled producer version, quoted
   six times, replaced by a bracketed word. That value is Excel's
   `AppVersion` string as the manifest's ten ms-template rows
   already carry it and as ruling I writes it; it is not private
   and the edit was not needed; it is accepted as committed on
   85dd323's precedent (the heading discloses it; the unedited
   answer, sha256 600316f9, is in `<phase>/report-final.md`,
   c62f931d, mode 600). Future check answers quoting it are
   committed unedited. (3) The final scan's first try failed one
   structural check (the status check admits only untracked
   records and the files were already staged); it is kept as
   `<phase>/superseded/scan-final-1-staged.log` ("RESULT: FAIL",
   structural 1), the fix was `git restore --staged records/`
   (index only), and the second try is the committed scan.log.
   That is a failing run recorded, fixed with the fix named, and
   run again; the README omits it and the report has it; this
   ruling puts it in the tree. Check 1's three answers are each
   "yes-with-exceptions"; its exceptions were disposed in the
   README before the final scan (the stderr sentence corrected, the
   placeholder filled, the round named by the file names and
   rulings G and J, the scan hashes with their primary in the
   printed script); read and accepted.

## Planning verification of P1 (2026-11-05)

Verified from primaries, never from the implementing session's
summaries alone: the proposer and its tests (ruling A); the stage-1
yes run, its prune of six part 2 build roots and its C96 failure,
the fix and the dry runs g6 and g7 (rulings D and E); the list-mode
rerun and round 1's listing, planning's own zip-module read of each
file, the tags, the add (rulings E to G); the maintainer's second
zip, unpacked by planning, round 2's listing, tags and add (rulings
H to J); the coverage and negative runs and the record (ruling K);
the record commit and the scan (ruling L). The corpus holds 39
files (17 libreoffice, 17 excel-win, 5 google-sheets); no tag is
uncovered; eight stay single-producer (pivots, slicers, timelines,
macros, external, sparklines, date1904, large), excel-win for the
first six and libreoffice for the last two. The P5 gap stays:
no Sheets export carries `cf`, `validation`, `merge`, `charts` or
`names`; no file is from Excel for Mac; `excel-mac` is unused.
Each slip of the phase is disclosed in the logs' README or in
rulings D, E, J and L. P1 is DONE at 1113530. P2 begins with its
doc commit (this spec with rulings A to L and this verification;
the ROADMAP's P1 row DONE, its P2 row IN PROGRESS).

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
