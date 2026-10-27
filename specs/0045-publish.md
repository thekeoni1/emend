# 0045: Milestone 7 part 2, the fixes and the publish of 0.1.0

Status: APPROVED 2026-10-27 (Milestone 7 part 2; decisions 1 to 5 A,
the recommended option in each, typed in the planning session
2026-10-27; drafted the same day). Written by the planning session
for the implementing session.
The P1 brief follows part 1's close-out, DONE and PUSHED 2026-10-26
(`specs/0044-excel-check.md`, "Planning verification of the
close-out", ruling AG).

Milestone 7, "release", was approved 2026-10-25 (`ROADMAP.md`,
"Milestone 7"): the Excel check first (decision 1), the ceiling
decided at the next core milestone (decision 2), the corpus items
after the publish (decision 3), the version 0.1.0 (decision 4). Part
1's record, `records/excel-check-m7-2026-10-26.md`, found five
things: the chart-surface sample refused on all 27 workbooks, the
protect sample on 2, the edit sample on 2, the remove sample on 1,
and 32 fixture pairs refused on the output alone. Planning read
their causes from the files (`specs/0044` ruling AG) and the
maintainer decided per finding on 2026-10-26, typed in the planning
session: fix before the publish the of-pie split, the workbook
protection's place, the scatter style `none`, a chartsheet left
without its chart, and the trendline period's range; document as
limits a table header edited, a removed sheet's references, and the
skeletal-fixture precondition. This part makes those fixes, has the
maintainer open the fixed outputs in Excel once more, prepares the
manifest, the CHANGELOG and the README, and pushes the tag. The
maintainer publishes from a clean clone of the tag on a machine of
their own; the sessions verify the publication against the tag.

Context: `CLAUDE.md` ("Secrets and boundaries": no API key or
registry token in any session's environment or file; "No code is
copied ... out, except through a deliberate, recorded step the
planning session has ruled on": the publication is that step; "The
fidelity harness is the acceptance instrument"), `specs/0001-
bootstrap.md` ("Rules for every phase"), `specs/0031-release-
readiness.md` (P2 rulings 1 and 4, the P2 verification ruling 5:
the `include` list tested by the `git archive` dry run, the
registry's validation of keywords and categories; decision 1, the
hold), `specs/0038-chartsheets.md` ("`add_chart` and `remove_chart`
on a chartsheet"), `specs/0039-chart-kinds-and-formatting.md` ("The
call", the of-pie split), `specs/0019-names-links-properties-
protection.md` ("Creating an element in the workbook"),
`specs/0015-open-checks.md` (the Excel-check rulings; ruling 4's
precondition), `specs/0043-measurement-and-close.md` (the close-out
form; rulings Q and R), `specs/0044-excel-check.md` (rulings T to
AG; "The restored Excel-record check"). No external text is cited.

## Goal

At the close-out, HEAD holds:

- the five fixes, each with a test, and the kept fixtures Excel
  refused on the output alone re-kept and opened;
- the three limits written where the README and the CHANGELOG list
  what emend does not do;
- a second Excel record, `records/excel-check-m7p2-<utc-date>.md`,
  over the fixed outputs, every row `pass` (decision 3);
- the crate manifest with an `include` list proven by the `git
  archive` dry run, `publish = false` lifted, the version 0.1.0;
- the CHANGELOG's 0.1.0 heading dated and its "Fixed" section
  holding the five fixes; the README's measured sentences citing
  part 1's record and this part's;
- the size record of the milestone;
- the annotated tag `v0.1.0` on the close-out doc commit, pushed
  with it;
- `ROADMAP.md`'s part 2 rows DONE.

After the push the maintainer publishes; after the publication the
sessions verify it (P4) and record it, the record committed and
carried by part 3's push.

## Size

P1 changes code under the `charts` and `metadata` features and, by
decision 1, the workbook rewrite's insertion rule, which the core
links. The milestone's size record is P1's, in part 7's form
(`records/size-m7p2-<utc-date>-<shortsha>.md`): the budget expected
equal to `records/size-m6p6-2026-10-24-7ac8b74.md`'s 451528 under
decision 1 B, and under 1 A moved by the rule's change, expected 0
to 300 section bytes and accepted up to 1500 within the ceiling
458752 (headroom 7224); the `charts` and `metadata` costs expected
to move by the fixes, each movement attributed by file. A budget
above the ceiling stops the part: the fix is narrowed (1 B) before
anything else runs. P3 changes the manifest and the texts only, no
probe source: no size record.

## Facts planning checked (2026-10-27)

- `crates/emend/Cargo.toml`: `version = "0.1.0"`, `publish =
  false`, no `include` list; `readme`, `repository`, `keywords` (5)
  and `categories` (2) set; the licence texts beside it; the
  template under `crates/emend/template/`, its `_rels/.rels` the
  dotfile the packaging finding named (`specs/0031` P2 ruling 1).
  The tools' manifests are not published.
- `CHANGELOG.md`: "## 0.1.0 (unreleased)" with "Added", "Changed",
  "Fixed" (empty), "Measured" and "Not supported by design". The
  README's "What has been measured" cites a record per sentence and
  names no Excel record; "Out of scope by design" lists the design
  limits.
- No tag exists (`git tag` prints none).
- The of-pie writer (`crates/emend/src/package/chart_write.rs`
  2163 to 2177) writes `splitType` `auto` for `split: None`; the
  record shows every `auto` refused (4 fixtures, 27 corpus outputs)
  and `pos`, `val` and `percent` accepted. The scatter writer (2073
  to 2084) writes `none` for `ScatterStyle::None`; refused (1
  fixture), the five others accepted. `TrendlineKind::MovingAverage
  (u16)` (539) is written unchecked; 65535 refused, 2 accepted;
  `Polynomial(u8)` is documented 2 to 6.
- `WORKBOOK_CHILDREN` (`crates/emend/src/package.rs` 14790) and its
  rule: a created element goes immediately after the last root child
  of the main namespace that precedes it, so `workbookProtection`
  lands right after `workbookPr`, before an `xr:revisionPtr` or
  `mc:AlternateContent` Excel wrote there; both refused workbooks
  had one, the eight accepted additions had `bookViews` next. The
  worksheet rewrite already creates before the next child in schema
  order (the conditional tests say so).
- `specs/0038`, "`add_chart` and `remove_chart` on a chartsheet":
  the drawing part stays, empty, "which `specs/0037` ruling Q
  accepts (an empty `wsDr` opens in LibreOffice and Excel)". Excel
  refused both such fixtures; an empty drawing on a worksheet
  (the remove-chart sample) it accepted. Ruling Q stands for
  worksheets.
- `scripts/check.sh` runs nine steps (host, i686, fmt, clippy, doc,
  native, licences, hygiene, privacy); `cargo publish --dry-run` is
  not among them (Milestone 4 part 2 ran it in its own log).
- The restored push-script check matches exactly one file
  `records/excel-check-m7-*.md`; this part's second record is named
  `excel-check-m7p2-` so that count stays one.

## The fixes (P1)

Each fix is a numbered item; each has a unit test on a built
package (the catalogue's form) and, where a kept fixture Excel
refused, the fixture's test re-keeps its output.

1. The of-pie split. `split: None` writes no `splitType` element
   (the schema's default is `auto`; the element Excel refuses is
   not written); `Some` writes `splitType` and `splitPos` as now.
   The chart-surface sample is unchanged; its outputs change.
2. The workbook protection's place. Decision 1: A, the workbook
   rewrite creates an element immediately before the first root
   child of the main namespace that follows it in
   `WORKBOOK_CHILDREN`, or at the root's end when none follows,
   so an Excel element between `workbookPr` and `bookViews` stays
   before the new one (the worksheet rule); B, that reading for
   `workbookProtection` alone. A test builds a workbook with an
   `xr:revisionPtr` after `workbookPr` and reads the element back
   before `bookViews`.
3. `ScatterStyle::None` on a new plot. Decision 2: A, refused by
   `add_chart` with "a new scatter plot takes a scatter style
   other than none"; B, written as `marker`. The read side keeps
   `None` (a file may hold it).
4. A chartsheet's only chart. A write that would leave a
   chartsheet's drawing with no chart frame is refused, "a
   chartsheet keeps its chart"; a removal and an addition in the
   same write still replace it (`specs/0038` decision 2's form).
   `remove_sheet` is the way to drop the chart with its sheet.
   `specs/0038`'s sentence on the empty drawing is amended by
   ruling in this spec, not rewritten.
5. The trendline period. `MovingAverage(period)` with `period`
   outside 2 to 255 is refused, "a moving-average period is 2 to
   255"; `Polynomial(order)` outside 2 to 6 the same way if it is
   not already.

The three limits, as text:

- A cell edit on a table's header row changes the cell and not the
  table's column name; the two must be kept in step by the caller,
  or Excel repairs the file (README "Out of scope by design",
  CHANGELOG "Not supported by design").
- A sheet removed leaves every reference to it, and to its tables,
  in other sheets' formulas and defined names as they were, as
  insertions and deletions leave references (the same sections).
- A style edit assumes the styles part carries the font, fill,
  border and cell-style collections and `cellXfs` entries that name
  them (`specs/0015` Excel-check ruling 4; `specs/0012`'s limits):
  the README sentence is checked present, added if absent.

P1's logs: the gate; the negative log (each fix's refusal with its
control: the of-pie output holding no `splitType` and the `pos`
form still written; the created element after a planted
`xr:revisionPtr`, before `bookViews`; `None` refused and `Marker`
accepted; the chartsheet removal refused alone and accepted with an
addition; the period 1, 256 and 65535 refused and 2 and 255
accepted); the run log (the host and i686 test runs; the keep
regenerated and opened by `scripts/edit-open-check.sh` into
`records/m7p2-open-<utc-date>-<shortsha>`, compared with part 1's
open record by pairs and totals; the formula sample (the list
script's `--run`) and the five samples whose outputs part 1's
record names, chart-surface, protect, edit, remove and chartsheet,
rerun on the host over the 27 workbooks by
`scripts/passthrough-run.sh` and recorded as `records/m7p2-<name>-
<utc-date>-<shortsha>`); the size log; the README; one independent
check; the scan.

## The second Excel check (P2)

Decision 3: A, a stage of the fixed outputs only, by
`scripts/excel-check-list.py` as part 1 ran it, over P1's six
rerun directories (`--run` the formula sample's, whose rows carry
the references; `--run-plain` the five others), P1's keep and open
record; the stage
`<corpus>/work/excel-check-m7p2-<utc-date>/`, its zip beside it;
the maintainer carries, runs the opener and brings back
`open-results.txt` and the machine section as part 1's rulings T
and U; planning assembles `records/excel-check-m7p2-<utc-date>.md`
by part 1's script (its by-hand file empty when nothing is refused),
the implementing session commits it with the logs. Acceptance: every
corpus row of the five samples `pass`, and every fixture pair of the
re-kept fixtures `pass` on the output; a refusal returns the part to
P1 for that fix. B, no second check; the fixes rest on their tests
and LibreOffice until the next milestone's check.

The stage is small (about 162 corpus outputs and the re-kept pairs)
and the run is minutes; the zip goes by the route part 1 used.

## The release texts and the manifest (P3)

- `crates/emend/Cargo.toml`: an `include` list naming `src/**`,
  `template/**` (its dotfile by name, `template/_rels/.rels`),
  `Cargo.toml`, the two licence texts and the README path cargo
  resolves; `publish = false` removed. The run log proves the list
  by the `git archive` dry run of `specs/0031` P2 ruling 1's form:
  the archive extracted outside any git checkout, `cargo package
  --list` and `cargo publish --dry-run --allow-dirty` run there,
  the file count equal to the checkout's and `template/_rels/.rels`
  among them, the dry run's exit 0 with cargo's one "aborting
  upload" warning counted once (ruling 2 of the same spec). The
  dry run reaches the registry index and nothing else; no token is
  set and the log prints the environment's token variables absent.
  The registry's validation of keywords and categories is the
  publication's to show (P4).
- `CHANGELOG.md`: "## 0.1.0 (<utc-date>)", the date of the
  close-out doc commit; "Fixed" lists the five fixes, each citing
  this spec and part 1's record; "Not supported by design" gains
  the two new limits; "Measured" gains the Excel sentence below.
- `README.md`: "What has been measured" gains one sentence per
  Excel record: part 1's (6018 rows; corpus 1528 pass and 32
  repair, fixture 1864 pass and 365 refused, input 1874 pass and
  355 refused; the five findings fixed in this part) and this
  part's (its totals); every figure is the record's, printed by a
  command in the run log beside the sentence. "Out of scope by
  design" gains the two new limits.
- The crate-level doc comment is read against the README's first
  paragraph (`specs/0031` decision 3) and left as it is unless a
  sentence contradicts a fix.

P3's logs: the gate (the hygiene step, if it reads `publish`, is
told by a numbered change); the negative log (the archive dry run
with `template/_rels/.rels` taken out of the `include` list, the
verify build failing as the finding did; the README figures check
refusing a planted figure); the run log (the dry run; the texts'
figures; the CHANGELOG date equal to the doc commit's date read
from git at the close-out, so the date is written at the close-out
doc commit and P3's log checks the heading's form only); the
README; one independent check; the scan.

## Close-out push (after P3)

As part 1's (`specs/0044`, "Close-out push (after P2)") with these
differences: the base is part 1's close-out doc commit aed8087 by
its subject ("M7 part 1 close-out: commit planning docs (the P2
verification and rulings)"); `PREV_RECORD_PATTERN` is `^M7 part 1
close-out record` (658dbc0), in the range exactly once; the
close-out doc commit's subject is "M7 part 2 close-out: commit
planning docs (the P3 verification and rulings)", and it also
carries the CHANGELOG's date; `CARGO_TARGET_DIR` `<home>/.cache/
emend-build/m7p2-close/target`; the Excel-record check in pre mode
reads exactly one `records/excel-check-m7-*.md` as restored and,
by a numbered change, exactly one `records/excel-check-m7p2-*.md`
with a result cell (decision 3 A), each with its control; a
numbered change creates the annotated tag `v0.1.0` on the tip
after the pre checks pass, its message one ASCII line ("emend
0.1.0"), and pushes the branch and the tag in one `git push` with
`--follow-tags` after the maintainer's typed authorisation; the
post scan runs over the range; decision 5 A adds the whole-history
private-content scan at the tag, its counts printed beside the
2026-10-22 run's (825 commits, 3484 blobs, 0 findings). The record
commit "M7 part 2 close-out record: the push, the CI run and the
scan" holds the three logs only. One independent check before the
report.

## The publication and its verification (P4)

The maintainer publishes from a clean clone of the tag on a machine
of their own (decision 4), with their own registry token, which no
session ever sees: `git clone --branch v0.1.0 --depth 1
https://github.com/thekeoni1/emend`, then `cargo publish -p emend`
in the clone, after `cargo login` on that machine. The maintainer
tells the implementing session when the registry lists 0.1.0.

Then P4, no code: the run log downloads
`https://static.crates.io/crates/emend/emend-0.1.0.crate` and the
index entry for `emend`, prints the crate file's sha256 equal to
the index's `cksum`, extracts the crate file into a scratch
directory and compares every file with the tag's tree by sha256
(`git archive v0.1.0` extracted beside it): every file of the
package equal to the tag's, the two files cargo adds
(`Cargo.toml.orig` and `.cargo_vcs_info.json`, the latter naming
the tag's commit) and the normalised `Cargo.toml` listed and read;
the registry's page fetched and its keywords and categories read
back as the manifest's (`specs/0031` ruling 4). No token is set;
the log prints the token variables absent. A difference is a
FINDING and a stop: the maintainer yanks nothing on a session's
word; planning rules. The record commit "M7 part 2 P4 record: the
publication verified" holds the log and the README; it goes with
part 3's push. `ROADMAP.md`'s part 2 heading then reads DONE and
PUSHED and PUBLISHED.

## Harness runs

P1: the keep and its open record; the six samples' host reruns.
P2: none (the maintainer's Excel run). P3: the packaging dry run.
P4: none. The scale tiers and the openpyxl comparison are not run.

## Phases

Each phase as `specs/0001` "Rules for every phase": a doc commit
first, the code commits if any, one record commit, the scan last,
at most two independent checks, the report under `<phase>`, then
stop. Target root `<home>/.cache/emend-build/m7p2-pN/target`. The
gate generator is part 1 P2's with the part's names, as a numbered
change; its prune lists and removes part 1's P2 and close-out build
targets at P1's gate by `rm -r` with the control (`specs/0044`
ruling AD). A dry driver sets every build root under the dry run's
own directory (`specs/0043` ruling Q); every dry and logged run
report lists every FINDING line found by grepping the word (ruling
R; `specs/0044` ruling Q).

### P1: the fixes

The doc commit holds this spec, `ROADMAP.md` (part 1's close-out
row DONE and its heading DONE and PUSHED, part 2's P1 row IN
PROGRESS) and the working `specs/0044` (its close-out verification
and ruling AG), on 658dbc0. Code commits per fix, each with its
tests; the docs commit for the three limits; the record commit
(the open record, the five sample records, the size record, the
logs, the check); the scan.

### P2: the second Excel check

The doc commit (P1's verification and rulings; the P2 row IN
PROGRESS). The stage and the zip as "The second Excel check (P2)";
the logs: the gate; the negative log (the list script's refusals as
part 1 showed them; the stage assertion); the run log (the list,
the stage, the zip; the record copied and the assembly rerun as
part 1's ruling W); the README; one independent check; the scan.
Then the maintainer's run and the record.

### P3: the release texts and the manifest

The doc commit (P2's verification; the P3 row IN PROGRESS). One
code commit for the manifest, one docs commit for the texts; the
record commit (the logs, the check); the scan.

### P4: the publication verified

After the maintainer's publication. The doc commit (the close-out's
verification; the P4 row IN PROGRESS). The record commit; the scan.

## Decisions for the maintainer

Decided by the maintainer 2026-10-27: 1 to 5 as recommended (A in
each), typed in the planning session.

1. The workbook rewrite's insertion rule: A (recommended), before
   the next main-namespace child in schema order, for every created
   workbook child, the worksheet rule; B, for `workbookProtection`
   alone.
2. `ScatterStyle::None` on a new plot: A (recommended), refused
   with an error; B, written as `marker`.
3. The second Excel check: A (recommended), run over the fixed
   outputs and gating the publish, every row `pass`; B, none, the
   fixes resting on their tests and LibreOffice.
4. Where the maintainer publishes: A (recommended), the laptop,
   with rustup installed there and a clean clone of the tag, the
   token entered there alone; B, the desktop's Windows side, where
   the credentials file would be readable from the sessions'
   mounts, not recommended.
5. The whole-history private-content scan at the tag: A
   (recommended), rerun in the close-out beside the range scan, its
   counts recorded; B, the range scan only.

## Out of scope

- Any fix beyond the five; the table-header edit, the sheet
  removal's references and the fixture precondition, documented
  and not fixed (part 1 decision 2 and the maintainer's 2026-10-26
  choices); the custom of-pie split (`specs/0039` decision 2 A).
- The Milestone 2 ceiling (Milestone 7 decision 2); the corpus
  additions (part 3); the i686 label's sample reruns; the scale
  tiers; the openpyxl comparison.
- Any registry token, credentials file or `cargo login` in a
  session's environment or files; a session running `cargo
  publish` other than `--dry-run`; yanking.
- The xlsx-diff, passthrough and privacy-gate backlogs and the MSRV
  policy under "Unscoped".
