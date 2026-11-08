# 0047: Milestone 8 part 1, the harness-side findings and the docs

Status: APPROVED 2026-11-08 (decisions 1 to 4 A, typed in the
planning session: "i agree with recommendations, proceed"); the
milestone APPROVED 2026-11-07 (Milestone 8, decisions 1 to 4 A,
`ROADMAP.md` "Milestone 8"). P1 DONE 2026-11-07 at 8b138cd; P2 IN
PROGRESS 2026-11-08 (ruling D).

Milestone 8, "corpus findings and the core ceiling", fixes the five
finding groups Milestone 7 part 3's P2 recorded over the grown
corpus and did not fix (`records/logs/m7p3-p2/README.md`, "The
findings"; `specs/0046-corpus-maintainer-items.md`, "Planning
verification of P2"). This part takes the three groups that are the
harness's, not the core's, and the two documentation sentences
whose figure is stale; no code the probes link changes. Part 2
(`specs/0048`) takes the two refusals behind their Cargo features
and whatever this part's diagnoses hand it (decisions 2 and 3
below); part 3 the Excel check and the release of 0.1.1.

Context: `CLAUDE.md` ("Corpus rules": real workbooks are PRIVATE,
quoted in records as entry names, byte counts, hashes and pass/fail
only; "The fidelity harness is the acceptance instrument": a failing
run is a finding to record, then fix, then run again with the fix
named), `specs/0001-bootstrap.md` ("Rules for every phase"),
`specs/0006-passthrough.md` ("The runner", the open check),
`specs/0015-open-checks.md` ("The open-check library", the
sheet-count rule), `specs/0016-new-workbook-and-sheets.md` (the
sheet sample and its read-back), `specs/0044-excel-check.md` ("The
reruns (P1)", the form of a sample rerun), `specs/0046` ("The runs
over the grown corpus (P2)", the comparison per id; rulings M to S;
"Close-out push (after P2)"). No external text is cited.

## Goal

At the close-out, HEAD holds:

- `records/diagnosis-m8p1-<utc-date>-<shortsha>.md`, one record of
  the three harness-side groups read from part 3 P2's run
  directories and their LibreOffice conversions by a read-only
  script, every statement a count, a boolean or an attribute value
  that is not a name; never a sheet name, a table name, an original
  name or a byte of cell content;
- the open check's sheet-count rule (`scripts/lo-check.sh`) telling
  LibreOffice's external-link cache tables from the workbook's
  sheets, as decision 1 rules, with its tests on fictional fixtures;
- `emend-copy`'s sheet-sample read-back reading the input's
  `<bookViews>`, as decision 3 rules, with its test;
- the README's size sentence citing the latest size record's figure
  and the CHANGELOG carrying the correction as decision 4 rules;
- the passthrough and the 58 sample records over the 39 workbooks at
  one commit, each compared per id with part 3 P2's record, the
  changes exactly the ones the fixes imply, every other FINDING
  recorded;
- `ROADMAP.md`'s part 1 rows DONE.

The close-out push carries Milestone 7 part 3's two local commits
(ed918a1, 06b96e5) with this part's.

## Size

No code the probes link changes: `scripts/lo-check.sh` is a script,
`tools/emend-copy` is the harness tool and no probe links it, and
the README and CHANGELOG are text. No size record. The ceiling is
458752 (Milestone 8 decision 1); the budget figure the README is to
cite is 451512 section bytes, Milestone 7 part 2's record
(`records/size-m7p2-2026-10-28-43471ed.md`, the format probe 984728
over `probe-zip-xml` 533216).

## Facts planning checked (2026-11-07)

Read from part 3 P2's run directories under `<corpus>/runs/
2026-11-05-d7a8196-<name>/<id>/` by counts and booleans, never by a
name; the ids and tags are the manifest's, already in committed
records.

Group 1, 9956034c4326 (excel-win, maintainer; tags plain, strings,
formulas, external; 10176 bytes): the workbook part holds 1
`<sheet>`, 1 `<externalReference>` and a `<bookViews>`; the one
externalLink part is an `externalBook` with 1 `<sheetName>` and 1
`<sheetData>`. LibreOffice's conversion of the input and of the
passthrough output each hold 2 `<table:table>` elements: the first
with a 6-character name and a style whose `table:display` is true;
the second with a 30-character name holding `://` and `#`,
`table:print="false"`, and a style whose `table:display` is false
(ruling A). The sheet sample's output holds 2 sheets with
`activeTab="1"`; its
conversion holds 3 tables, the third the same cache table. The rule
(`scripts/lo-check.sh`, `LO_SHEET_PATTERN` against
`<table:table(?=[\s/>])`) counts the cache table as a sheet.

Group 5, 495c7b79b183 (excel-win, maintainer; tags plain, strings,
names, pivots, slicers, timelines; 17800 bytes): the input holds 2
sheets, 1 `<pivotCache>` in the workbook, 2 defined names, and the
first sheet's relationships name the pivot table, the slicer, the
timeline and a drawing; its conversion holds 2 tables, both
displayed. The remove sample removed the last sheet: the output
holds 1 sheet, the pivot cache, the pivot table, the slicer and
timeline parts and the first sheet's relationships unchanged, and
`xl/pivotCache/pivotCacheDefinition1.xml`'s `<worksheetSource>`
names the removed sheet, not the kept one; neither defined name
names the removed sheet. Its conversion holds 2 tables: the first
displayed; the second with a 14-character name that is not the
removed sheet's, `table:display` false, LibreOffice's own. The
finding is on emend's output, which the README's removal sentence
("A sheet removed leaves every reference to it ... as they were")
covers for formulas and defined names and does not mention for a
pivot cache's source; decision 2.

Group 2, the five google-sheets ids (15205770ae5c, 453a166dcdfe,
6e0be3d0f2b1, 90a5b01f9541, 91d2bdffa46a): no workbook part holds a
`<bookViews>` element, and no worksheet holds `tabSelected`. The
sheet sample adds a sheet, moves it first and asserts the first
view's `activeTab` moved by one (`tools/emend-copy/src/main.rs`,
`sheet_read_back`, "active tab"); `workbook_counts` reads an absent
view as 0. The core remaps `activeTab` only on a present
`<workbookView>` ("absent reads 0, and stays absent when it maps to
0", `workbook_view_tag`) and creates no `<bookViews>`. So the input
read 0, the output read 0 and the assertion wanted 1; the tool
removed the output and exited 2. The input opened; no output was
opened.

The size sentences: the README (line 70 at 06b96e5) cites 451528
and `records/size-m6p6-2026-10-24-7ac8b74.md`; the CHANGELOG's
0.1.0 entry cites 436992 and
`records/size-m3p7-2026-10-09-09d2e0f.md`. The figure at the tag
v0.1.0 (cf727f7) was 451512, measured at 43471ed before the tag
(`records/size-m7p2-2026-10-28-43471ed.md`, "This part's core
movement": 984744 to 984728). No record was made at the tag, and
part 2's P1 record is the latest.

Part 3 P2's 59 runs took from 15:52:04Z to 20:08:42Z on 2026-11-05
(run.log's first and last timestamps); `scripts/passthrough-run.sh`
takes `--ids`.

## The diagnosis record (P1)

A read-only script `scripts/lo-tables.py` reads a LibreOffice
conversion (an ods archive) and a workbook, and prints, for the
workbook: the `<sheet>` count, each sheet's `state` (visible when
absent), the `<externalReference>` count, the `<sheetName>` count
over the externalLink parts, whether a `<bookViews>` is present and
its first view's `activeTab` (absent printed as `absent`), the
`<pivotCache>` count and, for each pivotCacheDefinition part,
whether its `<worksheetSource>` names a sheet of the workbook
(`kept`, `missing` or `none`); for the conversion: the
`<table:table>` count and, per table, the name's length, whether it
holds `://` or starts with `file:`, whether it holds `#`, the
`table:print` value or `absent`, and the style's `table:display`
value or `absent`. It prints no name and no cell content; its
`--self-test` builds its own fixtures in a temporary directory and
checks every column, including a table name holding `://` and `#`
and one that does not. The script refuses to print a line that
holds a byte outside the printable ASCII range (the control: a
fixture name with such a byte makes it exit 2 with "refused").

The diagnosis record, `records/diagnosis-m8p1-<utc-date>-<shortsha>.md`,
holds the script's output over: group 1's passthrough input and
output conversions and its sheet-sample output; group 5's
passthrough and remove conversions and outputs; group 2's five
inputs (the passthrough outputs, byte-identical to the inputs by
part 3's records) and, since the sheet sample removed its outputs,
the five sheet-sample outputs rebuilt at the phase commit with
`emend-copy --sheet-sample` under an environment variable
`EMEND_COPY_KEEP_OUTPUT=1` that P1's one code change adds to the
tool: with it set, a failed read-back leaves the output in place
and the error line reads "... (output kept)". The record states, per
group, the classification the numbers support, in the words of
"Facts planning checked" or against them. Nothing is fixed in P1.

The run directories are read in place; nothing under `<corpus>` is
written. Every path printed is `<corpus>`-relative.

## The fixes (P2)

Group 1, as decision 1 rules. Under A: `scripts/lo-check.sh`'s
count compares the sheet count with the count of tables that are
not external-link cache tables, a cache table being one whose
`table:name` holds `://` or starts with `file:` and also holds `#`
and whose `table:print` is `false`; and it requires the cache-table
count to equal the `<sheetName>` count over the workbook's
externalLink parts, printing "the ods holds N table elements, M of
them external-link caches, and the workbook S sheet elements and X
external sheet names" as the reason when either comparison fails.
`scripts/lo-check-test.sh` (or the existing test of the library)
gains a fictional fixture pair with a cache table and one without,
and a negative case where the cache count and the external sheet
names differ. `scripts/edit-open-check.sh`'s reason class `count`
matches the new wording.

Group 2, as decision 3 rules. Under A: `sheet_read_back` takes
whether the input's workbook holds a `<bookViews>` (a fourth value
from `workbook_counts`); with one, `active_after` must be
`active_before + 1` as now; without one, the output must hold none
and read 0, and the edit line gains `views_before` and
`views_after` (0 or 1). The tool's test on a fictional fixture
without `<bookViews>` passes and one with it still passes; the
negative test (a doctored output with a view added) fails with
"active tab" as now.

Group 5: no harness change in this part under decision 2 A; the
finding stays a FINDING in P2's rerun, expected, and part 2 takes
the refusal. Under B the README's removal sentence names pivot
cache sources and the finding stays a FINDING in every rerun.

The size sentences, as decision 4 rules. Under A: the README's
sentence reads 451512, 7240 under the ceiling of 458752, citing
`records/size-m7p2-2026-10-28-43471ed.md`; the CHANGELOG gains an
"Unreleased" section above 0.1.0 with one "Changed" bullet: "The
README's size sentence cites the figure at the tag, 451512 section
bytes (`records/size-m7p2-2026-10-28-43471ed.md`); the 0.1.0 entry
below cited Milestone 3 part 7's 436992 and stands as released."
Nothing in the 0.1.0 entry changes.

## The reruns (P2)

At P2's phase commit, on the host, as `specs/0046` "The runs over
the grown corpus (P2)": the passthrough (stem
`m8p1-passthrough-<utc-date>-<shortsha>`) and every flag ending in
`-sample` that `emend-copy`'s usage text names less `--new-sample`,
over the 39 workbooks, `--label host`, the pinned image,
`--lo-timeout 600`, `--timeout 300`, into
`<runs>/<utc-date>-<shortsha>-<name>`, recorded as
`records/m8p1-<name>-<utc-date>-<shortsha>.md` and `.txt`.

Each record is compared per id on (outcome, lo, lo_in) with part 3
P2's record of the same sample
(`records/m7p3-<name>-2026-11-05-d7a8196.md`, the latest committed).
The expected changes are exactly: on 9956034c4326, `lo_in` fail to
ok in all 59 runs and `lo` fail to ok in the 58 where it failed
(decision 1 A); on the five google-sheets ids, the sheet sample's
outcome error to ok with `lo` ok (decision 3 A). Under a B the
corresponding changes are not expected. Every other difference is a
FINDING, recorded, not fixed. Group 5's remove line and groups 3
and 4's refusals are unchanged and are not findings of this part;
the run log says so by id. The README of the record commit tables
the expected changes and any FINDING by sample and id.

The i686 label, the scale tiers and the openpyxl comparison are not
run (`specs/0046`, the same reasons).

## Harness runs

P1: `scripts/lo-tables.py --self-test`; the five sheet-sample
rebuilds with the output kept. P2: the passthrough and the 58
samples over the 39 workbooks, every output opened.

## Phases

Each phase as `specs/0001` "Rules for every phase": a doc commit
first, the code commits if any, one record commit, the scan last,
at most two independent checks, the report under `<phase>`, then
stop. Target root `<home>/.cache/emend-build/m8p1-pN/target`. The
gate generator is part 3's with the part's names, as a numbered
change; its prune lists and removes Milestone 7 part 3's P1, P2 and
close-out build targets at P1's gate by `rm -r` with the control
(`specs/0044` ruling AD). A dry driver sets every build root under
the dry run's own directory (`specs/0043` ruling Q); every dry and
logged run report lists every FINDING line found by grepping the
word (ruling R; `specs/0044` ruling Q).

### P1: the diagnosis

The doc commit holds this spec and `ROADMAP.md` (Milestone 8's
block, part 1's block with its P1 row IN PROGRESS), on 06b96e5. Two
code commits: `scripts/lo-tables.py` with its self-test, and
`emend-copy`'s `EMEND_COPY_KEEP_OUTPUT` with its test. The dry run;
the logged run (the self-test, the five rebuilds, the script over
the directories, the record assembled by the run); the record commit
(the record, the logs, the check); the scan.

### P2: the fixes and the reruns

The doc commit (P1's verification and rulings; the P2 row IN
PROGRESS). The code commits, one per fix: the count rule and its
tests; the read-back and its test; the two documents. The run log;
the 59 records; the record commit (the records, the logs, the
README, the check); the scan.

## Close-out push (after P2)

As part 3's (`specs/0046`, "Close-out push (after P2)") with these
differences: the base is part 3's close-out doc commit 4316630 by
its subject ("M7 part 3 close-out: commit planning docs (the P2
verification and rulings)"); `PREV_RECORD_PATTERN` is `^M7 part 3
close-out record` (ed918a1), in the range exactly once; the
close-out doc commit's subject is "M8 part 1 close-out: commit
planning docs (the P2 verification and rulings)";
`CARGO_TARGET_DIR` `<home>/.cache/emend-build/m8p1-close/target`;
the two Excel-record checks read exactly one
`records/excel-check-m7-*.md` and one `records/excel-check-m7p2-*.md`,
unchanged; the CHANGELOG check reads the "Unreleased" heading above
the dated 0.1.0 heading and requires the 0.1.0 entry unchanged in
the range (decision 4 A); no tag; `git push origin main` alone after
the maintainer's typed authorisation in the implementing session;
the post scan over the range; no whole-history scan. The record
commit "M8 part 1 close-out record: the push, the CI run and the
scan" holds the three logs only. One independent check before the
report. Milestone 7 part 3's record ed918a1 and close doc commit
06b96e5 go with this push.

## Planning rulings during P1 (2026-11-07)

A. Dry run d1 read and the facts corrected. Planning ran the
   committed `scripts/lo-tables.py` (12a57fc) over the real
   conversions: the cache table's name is 30 characters and its
   style's `table:display` is false; the drafted facts read 40 and
   "no `table:display`" because planning's first probe counted the
   escaped XML text (two apostrophes as `&apos;`) and matched only
   style names of one form. The facts above are corrected in place
   before the logged run so that the record's "as the facts" lines
   read against the truth; group 5's hidden table (14 characters,
   display false, print absent) and every other fact stand. The
   fictional dry corpus (gen/make-dry-corpus.py) and the record's
   classifier (run-parts/classify.py) are built to the drafted
   facts: both are brought to the corrected ones (a 30-character
   cache name; display false), the scripts regenerated, and a dry
   run d2 made. d1 itself is accepted: gate.log d5bfe37b and run.log
   c6db7afc "checks all as expected: yes", 0 UNEXPECTED, every
   stderr 0 bytes, 0 FINDING emitted; its scan FAILs on three
   structural checks, each the dry layout (the phase directory,
   LOGDIR and RECDIR, the home directory in the dry paths), as part
   3 P2's dry scans did (`specs/0046` ruling M). The deviations are
   accepted: the gate's template is part 3 P1's printed gate, which
   has the code-commit forms P2's lacked; the five rebuilds go
   through `scripts/passthrough-run.sh --ids` with
   `EMEND_COPY_KEEP_OUTPUT=1` and no conversion, into
   `<runs>/<utc-date>-<shortsha>-sheet-kept`, each id reading
   error, exit 2, the line ending "(output kept)", the output kept;
   the passthrough output workbooks of groups 1 and 5 are read too;
   the keep-output variable acts for every sample, as the spec's
   sentence reads. The dry run's run script left a temporary
   directory under `<phase>/tmp`, outside dry/d1: the dry driver
   sets TMPDIR under dry/dN as well as every build root (`specs/
   0043` ruling Q extended), and the leftovers are left in place,
   listed by name and bytes in the report for the maintainer's
   deletion list. The word: after d2 reads as d1 did (gate and run
   yes, 0 UNEXPECTED, 0 FINDING on the fictional corpus, the scan's
   only failures the three dry-layout lines), the logged run follows
   without a further stop: the gate with PRUNE_PREVIOUS=yes, the
   run, the record commit (the record, the logs under
   records/logs/m8p1-p1/, the check), the scan last, one
   independent check, the report. A departure from the corrected
   facts on the real files is a FINDING for planning, not a failure,
   and the run stops nowhere for it.

B. The logged run's one "no" ruled (2026-11-07). gate.log d8503ba1
   (878025 bytes) reads "gate log checks all as expected: yes", 0
   UNEXPECTED, the prune done with PRUNE_PREVIOUS=yes: the three
   Milestone 7 part 3 target roots (6227787948, 6227466908 and
   2146536849 bytes by du -sb) removed by rm -r "${PREV_DIR:?}/
   ${n:?}", each "gone" line as expected, the kept names still
   there; the gate stands. run.log 1b9c7230 (240530 bytes) ends
   "run log checks all as expected: no" on one check, "they are the
   five ids specs/0047 names, in that order (change TK)": the
   manifest lists the google-sheets ids as 91d2bdffa46a,
   90a5b01f9541, 453a166dcdfe, 15205770ae5c, 6e0be3d0f2b1 and the
   spec lists the same five sorted; the implementing session's
   script compared order as well as membership, and its dry corpus
   wrote the sorted order, so d1 and d2 could not see it. Every
   other check reads yes, 0 UNEXPECTED, every stderr 0 bytes; the
   five rebuilds read error, exit 2, the composed line, the output
   kept; the record 7801689e (24333 bytes) reads 0 statements
   against the corrected facts (70 "as the facts", the three
   classifications supported, 0 FINDING emitted), and its group 1
   lines read name_length=30 and display=false as ruling A has
   them. This is a failing run: recorded, fixed, run again with the
   fix named (`CLAUDE.md`). Change TR: the check compares the
   manifest's google-sheets ids with the spec's five as a set, with
   a control that refuses one id replaced; the dry corpus writes its
   manifest in the manifest's real order. The first run directory
   `<runs>/2026-11-08-01b0896-sheet-kept` is moved, not removed, to
   `<corpus>/superseded-runs/2026-11-08-01b0896-sheet-kept-run1`
   (the directory the corpus already keeps for stopped runs), the
   first record and run.log 1b9c7230 to `<phase>/superseded/run1/`
   with their sha256 printed, all three moves by the rerun's logged
   driver with `mv` and printed before and after; this is the one
   write under `<corpus>` beyond the new run directory that P1
   allows. Then dry d3 of the run step alone (the gate and scan
   scripts unchanged at 43de576f and 11b1791e), and, if d3 reads as
   d2 did, the run step rerun with RERUN_NOTE naming run.log
   1b9c7230 and change TR, without a further stop: the record
   commit (the record, gate.log d8503ba1 as it stands, the rerun's
   run.log, the check, scan.log), the scan last, one independent
   check, the report under `<phase>`; the README of the logs names
   the superseded run and the moves. The d1 leftovers under
   `<phase>/tmp` (tmp.dLnN1l8eiD 5466780 bytes, g 2609885, r
   1196996, s 1962816, d2check 49957) and the logged runs' <work>
   directories stay, for the maintainer's deletion list.

C. The record commit 8b138cd verified (2026-11-07) from git: parent
   01b0896, the subject as ruled, the diff-alone sentence, the
   Co-Authored-By line only, ASCII; six files, the record
   records/diagnosis-m8p1-2026-11-08-01b0896.md b4418552 (24333
   bytes; 0 statements against the facts, 70 "as the facts", the
   three classifications supported, 0 FINDING) and
   records/logs/m8p1-p1 with README.md f7d79dc5, gate.log d8503ba1,
   run.log 4770abad (the rerun; its last line yes, the one ": no"
   the API-key line, RERUN_NOTE naming run.log 1b9c7230 and change
   TR, the set check and its control yes), independent-check-1.md
   96c5dd3f and scan.log 224df0ed ("structural checks failed: 0",
   RESULT: PASS, EXPECT_RECORDS 1 of 1, PLANNING_SPEC 8bc1c702). The
   first run directory is at `<corpus>/superseded-runs/
   2026-11-08-01b0896-sheet-kept-run1` and the rerun's at
   `<runs>/2026-11-08-01b0896-sheet-kept`; no committed file holds
   a home path, an office-file literal or a byte outside ASCII.
   Check 1 answers yes with one exception, yes with exceptions, and
   no; every exception is a figure whose primary is a file under
   `<phase>` that the commit does not carry (the dry logs, the
   generators after change TR, the two driver outputs with the
   moves' before and after lines), each named with its source in
   the README's "After independent check 1"; planning read the two
   driver outputs (rerun-driver.out 36eeb629, 2468 bytes;
   logged-driver.out ba23c399, 1298 bytes; no home path, no
   office-file literal, ASCII) and rules that both go into P2's
   record commit under records/logs/m8p1-p1/ as the moves' primary,
   named in P2's README. gate.log's gen/run.py hash is the
   generator before change TR, and the gate stands because it does
   not read that generator's output (ruling B). The exception on Q1
   is ruling A's acceptance. P1 is DONE at 8b138cd. P2 waits on the
   maintainer's letters for decisions 1 to 4.

D. Decisions 1 to 4 taken as A (2026-11-08) and P2 briefed. Four
   points the spec left to the phase are fixed here. The count rule:
   `scripts/lo-check.sh`'s comparison moves into a function the
   script also exposes alone (the workbook package, the ods and the
   reason file as its arguments, the same exit codes), so that the
   rule is tested without a conversion; a new `scripts/lo-check-test.sh`
   builds fictional xlsx and ods archives with `python3 -I -B` in a
   temporary directory and runs the function over them: a pair with
   one sheet, one external sheet name and one cache table (ok), a
   pair with no cache table and no external reference (ok), a pair
   whose cache count and external sheet names differ (fail, the new
   reason), and a pair whose non-cache tables and sheets differ
   (fail, the new reason); a cache table is told by the three marks
   of decision 1 A read with a parser, never by a regex over the raw
   text (ruling A's lesson). The read-back: its test is an
   integration test under `tools/emend-copy/tests/` on fictional
   fixtures the test writes, as `keep_output.rs` does. The gate: its
   prune lists and removes P1's build target
   `<home>/.cache/emend-build/m8p1-p1/target` alone, by `rm -r` with
   the control (`specs/0044` ruling AD); P1's dry roots, its `tmp`
   leftovers and its two driver outputs stay. The record commit
   carries, besides `records/logs/m8p1-p2/`, the two P1 driver
   outputs under `records/logs/m8p1-p1/` (ruling C), named in P2's
   README with their hashes. The comparison per id is part 3 P2's
   run script's, against `records/m7p3-<name>-2026-11-05-d7a8196.md`;
   the run script checks the expected-change table of "The reruns
   (P2)" and prints each departure as a FINDING line. P2 stops once:
   after the doc commit, the three code commits and the dry run, for
   planning's word on the diffs and the dry logs; the logged run (the
   gate with the prune, the 59 runs, the record commit, the scan, one
   independent check, the report) then follows without a further
   stop. The doc commit is on 8b138cd with the two planning files as
   they stand after this ruling.

## Planning verification of P1 (2026-11-07)

Verified from primaries, never from the implementing session's
summaries alone: the two code commits' diffs and tests; dry runs d1
and d2 and the committed script run by planning over the real
conversions (ruling A, which corrected two of planning's own
facts); the logged run's gate and the prune; the first run's one
failing check, its cause and the rerun under change TR (ruling B);
the record commit, the record, the check and the scan (ruling C).
The three groups are classified as the spec drafted them, now on
measured ground: group 1 is the open check's rule counting
LibreOffice's external-link cache table, whose name holds `://` and
`#` and whose style hides it and marks it not for printing; group 5
is emend's output leaving a pivot cache sourcing a removed sheet,
for which LibreOffice adds a hidden table of its own; group 2 is
the sample tool's assertion on inputs with no `<bookViews>`, which
the core neither needs nor creates. P1 is DONE at 8b138cd; the
planning files stay uncommitted for P2's doc commit, which waits on
decisions 1 to 4.

## Decisions for the maintainer

To be typed in the planning session; the recommended option is A
in each. P1 needs none of them.

1. Group 1, the external-link cache tables: A (recommended), the
   count rule excludes LibreOffice's cache tables by their form
   (`://` or `file:` with `#` in the name, `table:print="false"`)
   and requires their count to equal the externalLink parts' sheet
   names; B, the count rule is skipped, with "count skipped:
   external references" as the reason class, for a workbook holding
   an `<externalReference>`.
2. Group 5, a removed sheet a pivot cache sources: A (recommended),
   emend's finding: part 2's `remove_sheet` refuses a sheet named by
   a pivot cache's `<worksheetSource>` ("a pivot cache sources the
   sheet"), judged by the size record against the ceiling as every
   core change is, Milestone 8 decision 1 reopened only if the
   budget would cross; B, a documented limit: the README's removal
   sentence names pivot cache sources, and the finding stays a
   FINDING in every rerun.
3. Group 2, no `<bookViews>` in the input: A (recommended), the
   harness's assertion reads the input's `<bookViews>` and expects
   none in the output when the input has none, the tool's line
   saying which; B, part 2's core creates `<bookViews>` with a
   `<workbookView activeTab="N">` when absent and a sheet moves or
   is removed, so the active sheet stays the same (an element the
   input lacks is added; the budget moves).
4. The CHANGELOG: A (recommended), the 0.1.0 entry stands as
   released and an "Unreleased" section carries the correction, to
   become 0.1.1's in part 3; B, the 0.1.0 entry's sentence is edited
   in place.

Decided by the maintainer 2026-11-08, typed in the planning session
("i agree with recommendations, proceed"): 1 A, 2 A, 3 A, 4 A. P2
reads the A branches above; decision 2 A's refusal is part 2's
(`specs/0048`).

## Out of scope

- The two refusals (groups 3 and 4) and any change to code the
  probes link (part 2, `specs/0048`).
- The Excel check and the release of 0.1.1 (part 3).
- Any new corpus file; any change to `corpus-add.py` or the tags.
- Any fixture, test or record holding a byte of a maintainer's
  workbook, a sheet or table name of one, an original name or a
  note.
- The i686 label; the scale tiers; the openpyxl comparison.
- The xlsx-diff, passthrough and privacy-gate backlogs under
  "Unscoped".
