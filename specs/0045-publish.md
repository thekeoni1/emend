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

## Planning rulings during P1 (2026-10-27)

Asked by the implementing session after the doc commit bb7c35f
(parent 658dbc0, the three files at their hashes, ASCII, the one
trailer; planning read it from git), before any code or dry run.

A. R1, fix 4. Today a chartsheet's chart is replaced in two writes
   (`specs/0038` P1 ruling A: the removal alone, then the addition;
   `remove_chart` refuses a part the same write adds a chart to).
   Ruling: on a chartsheet alone, a removal and an addition in one
   write replace the chart: the one-chart count leaves the removed
   frame out, and "no chart at that position" does not apply to a
   chartsheet; a removal alone on a chartsheet is refused at the
   write with "a chartsheet keeps its chart". Worksheets are
   unchanged. `specs/0038` P1 ruling A and its empty-drawing
   sentence are amended by this ruling, not rewritten. The two
   fixture tests are renamed to what they now show, `a_chart_is_
   replaced_by_a_removal_and_an_addition_in_one_write` and
   `a_removal_alone_on_a_chartsheet_is_refused` (the second keeps
   no output); the open record's comparison with part 1's prints
   the pairs gone and the pairs new by name, the old two names'
   instances gone and the one-write test's new, which is expected
   and no FINDING; any other difference is a FINDING.
B. R2, fix 5. The lower bounds are already refused with "a chart
   value is out of its range" (`chart_write.rs` 1547 and 1548;
   the variant at 540). Ruling: a new constant "a moving-average
   period is 2 to 255" for a period below 2 or above 255 (periods
   0 and 1 change message, their tests updated); `Polynomial`
   keeps the generic message; the field doc reads "2 to 255". The
   spec's "written unchecked" reads "the upper bound unchecked".
C. R3, the re-kept fixtures. Ruling: "re-kept" means the 8 pairs
   the fixes reach (the scatter style .1, the of-pie type-and-split
   .1 and .10, the of-pie colour .1, the trendline kinds .9, the
   points .22, and the two chartsheet tests of ruling A). The other
   24 of part 1's pass/refused pairs (the three style tests, 16
   conditional, 4 linestyle, `several_edits` .1) stay refused under
   the documented precondition; P1's run log names them as allowed.
   P2's stage takes the whole keep, as the list script writes it
   (it has no filter): about 162 corpus outputs and the keep's
   pairs; P2's fixture acceptance is the 8 pairs `pass` on the
   output and the record's totals otherwise part 1's, the
   difference printed by the assembly's totals beside part 1's.
D. R4, fix 3. Refused at the write, in the plot's own checks, the
   `Errors` doc list updated in place. Amended (R4b, the same day):
   the plot's own checks run at the call, in `Package::add_chart`
   and `Package::add_chartsheet` (the of-pie split range at
   `chart_write.rs` 1078 to 1090; the catalogue, `package.rs` 939
   to 950, says "at the call"); only the chartsheet's anchor and
   one-chart refusals are raised at the write. Ruling:
   `ScatterStyle::None` is refused at the call, in the plot's own
   checks straight after `plot_applies` beside the of-pie split
   check, with "a new scatter plot takes a scatter style other
   than none", for a first plot and for a combined chart's scatter
   second plot alike, through `add_chart` and `add_chartsheet`;
   one catalogue entry; the `add_chart` Errors list updated in
   place. Fix 5's message is raised at the call too, where
   `trendline_options` runs (ruling B).
E. R5, fix 2. `specs/0019`'s "Creating an element in the workbook"
   and the doc comments at `package.rs` 14080 and 14785 are
   amended by this ruling: a created element goes immediately
   before the first root child of the main namespace that follows
   it in `WORKBOOK_CHILDREN`, or at the root's end. The held-bytes
   mechanism stays; held tokens are written before the created
   element.
F. The committed `ROADMAP.md` reads PLANNED on part 2's P1 row
   where the spec said IN PROGRESS: planning's slip, accepted as
   committed; the working copy now reads IN PROGRESS and goes in
   P2's doc commit.

G. The six P1 commits (4a2582b, f79e447, 64f9062, 35e3d1d, 5d14c89,
   43471ed) read by planning from git: ASCII, the one trailer, the
   count sentence in each; the limit sentences in the README and
   the CHANGELOG as "The fixes (P1)" words them, the styles
   precondition sentence added since it was absent. Two readings
   for the dry run: (a) fix 2 moves held whitespace, comments and
   foreign children ahead of a created element, so the formula
   sample's outputs (`calcPr`) and the protect sample's
   (`workbookProtection`) change bytes; their host counts are
   expected unchanged, a count that moves a FINDING; (b) the open
   record's comparison with part 1's: pairs new from the tests
   this part adds (fix 2's protection test, fix 3's, any other
   named in the report) are expected and no FINDING, as ruling A's
   renamings are; a pair gone other than ruling A's two, or a pair
   whose open result changes from ok, is a FINDING.

H. Dry run d1 stopped at the gate's environment line (`specs/0029`
   item 1): the root's mount propagation read `private` after the
   WSL restart of 2026-10-27 (part 1's gates read `shared`) and no
   keepalive process ran. The implementing session restarted the
   host's keepalive task (`Start-ScheduledTask`, no elevation;
   `pgrep` counts 1 now) and disclosed it; `mount --make-rshared /`
   needs root, which neither session has, and the maintainer runs
   it from the host (`wsl -d Ubuntu -u root mount --make-rshared
   /`). The check stands as written; d1 stays under
   `<phase>/dry/d1`; d2 runs from the start once `findmnt` reads
   `shared`. Readings for d2: (1) the scatter-style test's `None`
   row is gone with fix 3, so part 1's pair `a_scatter_chart_takes_
   each_style_with_x_and_y_values.6` is gone: expected, named, no
   FINDING. (2) `a_removal_alone_on_a_chartsheet_is_refused` keeps
   two pairs, the fixture write (.1) and the `remove_sheet` control
   (.2); ruling A's "keeps no output" reads "the refused removal
   keeps nothing". (3) `each_trendline_kind_writes_its_type_its_
   order_or_period_and_its_fields` gains three refused periods, so
   its pairs after them renumber: its new keys and its changed
   same-key outputs are expected and no FINDING when every open
   result is ok; the report names the shifted keys. (4) The new
   gate input `PLANNING_ROADMAP` (change J91) pins the working
   `ROADMAP.md` in the gate, negative, run and scan scripts; the
   size script admits " M ROADMAP.md" by pattern as its template
   does: accepted. (5) Size: the `metadata` cost has no file of its
   own, so fix 2's `package.rs` change moves the core budget, which
   "Size" expects (0 to 300, accepted to 1500, the ceiling a stop);
   Milestone 7 decision 2's "adds no core code" is superseded for
   fix 2 by the maintainer's choices of 2026-10-26 and decision 1 A,
   recorded here; a shrink is a FINDING as the template says;
   `SIZE_TARGET` and `SIZE_TMP` (change SE) accepted for ruling Q.
   (6) The carried classifier counts the docs commit 43471ed as
   code, so the range reads one doc and six code commits: accepted
   as carried. (7) The gate fork ran `cargo test -p emend` per
   feature set into dev-target to confirm its expected counts,
   against FACTS' "forks do not run cargo": disclosed, accepted,
   named in the README's disclosures; no log was affected.

I. Dry run d3 (2026-10-27; gate yes, negative yes, run no): planning
   read the three logs, the three emitted FINDING lines and the
   records. (1) Ruling H (3) named the wrong test. Fix 5 (5d14c89)
   put its three added refused periods (256, 1000, 65535) and its
   two taken ones (2, 255) in `the_trendline_and_error_bar_field_
   rules_are_refused`, whose pairs go from 45 to 48; the write test
   `each_trendline_kind_writes_its_type_its_order_or_period_and_
   its_fields` gains no key and changes one output (.9, the period
   65535 written as 255). Ruling: the refused test's new keys .46,
   .47 and .48 are expected by key, as the gone keys of ruling A
   are, when each one's open result is ok; any other new key of
   that test stays a FINDING; the write test's .9 is expected as H
   (3) reads, its new-key count 0 accepted. d3's three FINDING lines
   are these three keys, each ok ok in d3's record: read, closed by
   this ruling, not fixed. (2) The two NO lines ("each of the six
   samples is named once by the binary's usage text", "the 8
   re-kept pairs of ruling C are each one of finding 5's names")
   came from `count_matches -xF -- <key> <file>`: the committed
   `scripts/counting.sh` takes (mode, pattern, file...), so the
   `--` was the pattern. Change TI (drop the `--`, two positive
   controls) accepted; the data read right by planning from the
   log's own lists (6 flags of 59, 8 names of 32). (3) d3 stays
   under `<phase>/dry/d3`; d4 runs from the start under
   `<phase>/dry/d4` on the regenerated scripts and reaches the size
   budget and the scan pre-check. Nothing logged before d4 reads
   yes in all and planning gives the word.

J. Dry run d4 (2026-10-27; gate, negative and run yes; size no):
   planning read the four logs, every emitted FINDING line (gate 0,
   negative 0, run 0, size 8) and the size budget lines. (1) The
   run: ruling I's three keys read "new, expected by key, open ok
   ok"; gone 7, new 17, none otherwise; FINDING lines 0. (2) The
   size NO line ("TMPDIR is exported as <scratch>/tmp") is change
   SE's miss: `tmpdir_set` compared with the default
   `$PHASE_DIR/tmp`, not `SIZE_TMP`. Change SF (the comparison
   reads `${SIZE_TMP:-$PHASE_DIR/tmp}`, nothing else) accepted.
   (3) The budget figure is 451512, 16 section bytes under M6 part
   6's 451528, 7240 under the ceiling; the core moved -16 section
   bytes, .text -16, file 0; every core probe moved -16 (rowcol
   -32). The eight FINDING lines (seven probes, the core movement)
   are the shrink "Size" did not expect (0 to 300). Planning's
   reading of the five fix commits' `package.rs` changes: fix 2
   (f79e447) is the one change to code the core links, and it
   merges the two sites that wrote the held tokens into one ahead
   of the branch, so less code; fixes 3 and 5 change doc comments
   alone there; fix 4's code is in the drawing removal path behind
   the `images` and `charts` features, which the core probes do not
   link. Ruling: the shrink is attributed to fix 2 by this reading,
   not by a measurement; the eight lines are recorded, not fixed,
   and closed by this ruling; the record and the report name the
   figure, the movement and this attribution; "Size" above stands
   as written, its expectation read as a growth that did not come.
   (4) The feature costs moved within their bands (charts +1784,
   images +884, comments +16, read +16): not judged, as "Size"
   says. (5) d4 stays under `<phase>/dry/d4`; d5 runs from the
   start under `<phase>/dry/d5` with SF and reaches the scan
   pre-check. Nothing logged before d5 reads yes in all and
   planning gives the word.

K. Dry run d5 (2026-10-27): planning read the five logs. Gate,
   negative, run and size read yes; the run's pairs gone 7 and new
   17 all as expected, its FINDING lines 0; the size figures are
   d4's (451512, -16, 7240 under the ceiling) and its eight FINDING
   lines are ruling J's, no others in any log. The scan pre-check
   reads FAIL on four structural checks, each from the dry layout
   (not the default log and record directories; 2 of 6 run
   directories, the dry run's two reruns; 7 records of 15; 52
   output lines holding the home directory, every one under
   `<phase>/dry/d5`, none elsewhere); total hits 0, read or
   counting errors 0: the pre-check passes as a dry run passes
   (`specs/0043` ruling R). Ruling: planning's word for the logged
   run is given. It runs from `logged.sh` modelled on part 1 P2's,
   gate, negative, run (the six samples, the whole keep) and size,
   `PRUNE_PREVIOUS=yes` (`specs/0044` ruling AD: `rm -r
   "${PREV_DIR:?}/${n:?}"` in the generated gate with a control);
   the logs README discloses the keepalive restart and the root
   mount (ruling H), the forks' cargo runs (H (7)), and d1 to d4
   with their changes (NI64, TI, TJ, SF); then the scan pre-check,
   the one independent check, the final scan, ONE record commit,
   the report under `<phase>`. The eight size FINDING lines are
   expected in the logged run as ruling J closes them; any other
   FINDING line is reported before the record commit. The scripts
   regenerate for this spec's hash and no other change; the
   report names each script's sha256. No push.

L. The logged run (2026-10-27/28; gate, negative, run and size yes,
   scan pre-check PASS, hits 0) and the first independent check:
   planning read the four logs, the size record and the check's
   answer. The check's three gaps hold against the primaries, read
   by planning: (a) the negative log shows fixes 3 to 5's refusals
   as the seven tests passing by name and the kept outputs (K3 to
   K5) alone; "P1's logs" above asks for each refusal with its
   control; (b) K3 has no control line; (c) the size record does
   not carry ruling J's attribution, and one sentence of it names
   four commits as the five fix commits. A failing run is a finding
   recorded, then fixed, then run again with the fix named (the
   working rules). Ruling: (1) the negative generator gains, for
   each of fixes 3, 4 and 5, a section reading the refusal at the
   phase commit: the message text, the test's refusing and
   accepting calls read from the test file at that commit (`None`
   refused and `Marker` accepted; the removal alone refused and the
   removal with an addition accepted; the periods 0, 1, 256, 1000
   and 65535 refused and 2 and 255 taken), the exact test's pass
   line, and a control each; K3 gains a control (an accepted
   scatter pair holds its chart part). (2) The size template gains,
   after the core movement, "attributed by planning's reading to
   fix 2 (f79e447), the one change to code the core links
   (`specs/0045` ruling J), not by a measurement", and the sentence
   that calls four commits five keeps the two lists apart as the
   size log does. (3) The negative and the size run again, logged,
   under this spec's hash; the gate and run logs stand under the
   hash they pinned. The first negative log, the first size log and
   the first size record go to `<phase>/logged/superseded/` as
   attempt 1, named in the README with their sha256 and the cause
   (this ruling), with the changes' names. (4) The report carries
   the check's answer verbatim and its minor points as the
   implementing session listed them (both gate times; "M6 part 6's
   record's 451528"; d5's "RESULT: FAIL on its four dry-layout
   checks alone"; the driver's figures marked as the session's
   statements). (5) The phase's second and last independent check
   reads the regenerated negative and size logs, the size record
   and the revised report alone; then the final scan, ONE record
   commit, the report under `<phase>`. No push.

## Planning verification of P1 (2026-10-28)

Read from the primaries, not the report: the record commit 58c4f51
(parent 43471ed, 23 files added: the eight files under
`records/logs/m7p2-p1/`, the six samples' record pairs and the open
pair named `-2026-10-27-43471ed.`, the size record
`size-m7p2-2026-10-28-43471ed.md`; the message ASCII with the
Co-Authored-By line only, the count sentence carried, its form the
part 1 P2 record's); nine commits ahead of origin/main aed8087, no
push; the tree after it " M ROADMAP.md" and " M specs/0045" alone.
The gate (sha256 f5d7ee38) and run (30be6ead) logs stand under the
spec hash cf085c08 as ruling K read them; the negative (e8054745)
and size (872d5eda) logs under 866c16ce, the working spec's hash at
645 lines, each ending "checks all as expected: yes"; the lines
ending ": no" outside the printed scripts are the API-key line in
each, and in the size log the "same section layout" and override
lines, informational as in d5. FINDING lines emitted: gate 0, run 0
(the run's, read under ruling K), negative 0, size 8 at 11195 to
11202, ruling J's seven probes and the core movement, repeated in
the record at 13618 to 13625, no others; the word appears besides
in the "judged:" lines that restate the same eight. The budget
451512 at size.log 11056, 7240 under the ceiling 458752, -16 against
M6 part 6's 451528. The size record (f63dc6ac, 398 lines) carries
ruling L (2)'s sentence at line 142 after the core movement, checked
at 13828 with a control, and the two commit lists apart at line 340.
The negative log's K3 control (5812 to 5822: the scatter style test
alone with its own keep, the marker pair's `xl/charts/chart1.xml`
present) and the R3, R4 and R5 sections (5860 to 5908: each
message; `None` refused and `Marker` taken read at lines 1435 to
1468 of the test file at 43471ed; the removal alone inside
`refused_after` with no `add_chart` and the removal with an addition
at 830 to 834; `PERIOD` once, periods 0, 1, 256, 1000 and `u16::MAX`
refused and 2 and 255 taken at 5722 to 5735; each exact test's pass
line counted 1; a control each, refusing a planted copy). Attempt
1's negative log (6727ff5b), size log (627f5141) and size record
(56c7ae20) under `<phase>/logged/superseded/` with 0-byte outer
stderr files, named in the README with the cause and changes NJ64
and SG; the README (291 lines) discloses the keepalive restart and
root mount, the gate fork's cargo runs, d1 to d6 with changes NI64,
TI, TJ, SF, and the final scan's first run. Check 2's committed copy
differs from the answer as given (574f8977) by its heading line and
the `<home>` and `<session>` substitutions the heading names, the
same edit named in check 1's heading; check 2 reads yes with
exceptions, yes, yes with exceptions. The final scan's first run
(scan-final-1.log, 4f0bd14e) read FAIL on 2 hits of form f_p6, the
harness session path quoted in both check answers; the committed
scan.log (f09b3ae1) is the second run after the `<session>` edit:
hits 0, errors 0, structural checks failed 0, RESULT: PASS. No
committed file of the commit holds a home path, a session id or
U+2014 outside pattern text in the printed scripts. Six run
directories `-43471ed-` under `<corpus>/runs`; the host keep 4502
files under `<phase>/keep-host`.

Rulings. M. Check 2's exceptions E1 to E5 (K3's control by another
   reading than K3's own; R3's `add_chartsheet` refusal read through
   its `.expect_err` line; R5's `refused(` line unread; R4's accepted
   half without a control of its own; the pass line printed as a
   count) are recorded, not fixed: each is a strictness point on a
   reading whose claim check 2 confirmed from the source, ruling L
   (5) makes check 2 the phase's last, and a change would need a
   third logged negative run and a third check. The size template's
   opening wording that names the `metadata` feature (check 2's side
   note) is template text, recorded here. Check 2's report points i
   to iii are carried in the report as the session listed them.
N. The `<session>` edit to both committed check files is accepted:
   the harness session directory is neither a primary nor the
   maintainer's, and each heading names the edit and the answer's
   sha256 as given. The final scan's first run is a finding recorded,
   the fix named (the edit), the run again named (scan.log), as
   CLAUDE.md asks; no further check.
O. P1 is DONE. Its dry runs d1 to d6 under `<phase>/dry`, the
   superseded generators and attempt 1 under `<phase>/superseded`
   and `<phase>/logged/superseded`, and `<phase>/target` are the
   maintainer's to reclaim by a method they name in the implementing
   session, after P2's zip is copied, with the part 1 items ruling R
   of `specs/0044` listed.
P. P2 as "The second Excel check (P2)" and "P2: the second Excel
   check": the doc commit holds this spec and `ROADMAP.md` (the P1
   row DONE, the P2 row IN PROGRESS), each file named by path and
   blob sha256 in the message; the stage
   `<corpus>/work/excel-check-m7p2-<utc-date>/` by
   `scripts/excel-check-list.py` over P1's six run directories
   (`--run` the formula sample's, `--run-plain` the five others, in
   the usage text's order), `--keep <phase>/keep-host`, `--record`
   the open record `.txt` of 58c4f51; the zip beside it by
   `zipfile`, stored; the logs and the independent check as part 1's
   P1 ran them, the stage's counts asserted against the list; the
   Milestone 2 and part 1 stages and zips not read, moved or
   removed. The maintainer carries the zip by the laptop route,
   runs the opener, brings `open-results.txt` back under the stage
   and gives the machine section in the implementing session.
   Planning assembles the record by part 1's script
   (`assemble-excel-record.py`, sha256 54f344a7, unchanged unless a
   ruling names a change) as `records/excel-check-m7p2-<utc-date>.md`;
   the by-hand file is empty when nothing is refused. Acceptance as
   decision 3 A: every corpus row of the five samples and every
   fixture pair `pass`; a refusal returns the part to P1 for that
   fix. Nothing of P2 runs until the P2 doc commit is in.

## Planning rulings during P2 (2026-10-28)

Q. Dry d1 read from `<phase>/dry/d1`: gate and negative yes (the
   prune set empty under ruling O, P1's directory's names listed
   and its keep hashed at entry and end; cases FC, F1, F2, SC, S1,
   S2 and L1 to L3 with LC as part 1), run no: two counts passed
   `--` as count_matches's pattern (P1 d3's form), the six flags
   unmatched, `dir_ok` defined inside the loop and undefined for its
   control (stderr 98 bytes), seven lines no, no list, no stage, no
   zip, the keep's listing hash equal at entry and end; FINDING
   lines emitted 0 in each log. Change TN (the `--` dropped at both
   sites, `dir_ok` defined before the loop; run script f9ebe043)
   accepted; no other script holds the form. d2 from the start under
   `<phase>/dry/d2`. The eight questions: (1) yes: P2's gate prunes
   nothing in either mode and checks P1's directory and keep
   unchanged; the machinery stays, `PRUNE_PREVIOUS=list` in the
   dry and logged runs alike. (2) The eight doc-only sections
   carried from part 1 P2's printed gate (2979d334, e538e0a) as a
   second pinned source, A92 to S92: accepted. (3) The flag order
   from the USAGE constant at the phase commit, read by `git show`
   and not from a binary, TK to TN: accepted, the constant's lines
   printed. (4) The stage holds the whole keep as the list script
   lists it (4502 files, 2251 pairs) with the six samples' outputs;
   the spec's "about 162 corpus outputs and the re-kept pairs" is
   the corpus rows' size, not a narrowing; the acceptance stays
   decision 3 A's. (5) Yes: the record's copy and the assembly rerun
   (part 1's ruling W) are a second logged run after the maintainer's
   results come back; P2 has ONE record commit, after that second
   run, holding the stage's logs, the record's logs and the record;
   the stage's logs stand under `<phase>/logged` meanwhile; the
   phase's two independent checks are one on the stage run (the run
   log's directory list against the tool's flag list, as part 1) and
   one on the record run. (6) The work directory `<phase>/tmp` and
   the gate's target root `<phase>/target` in dry runs as P1's:
   accepted; STAGE_ROOT and the build roots move. (7) The scan
   takes an input EXPECT_RECORDS and asserts the count of
   `records/excel-check-m7p2-*.md` at the tip equals it: 0 for the
   stage run's scans, 1 for the record run's. (8) The negative
   fixtures' `run.host.tsv` name 43471ed, P1's phase commit, as the
   real directories do: a change named in the generator after d2
   ends, then d3 from the start; cosmetic, but a committed log's
   fixture reads as the real file's form.
R. Dry d2 read from `<phase>/dry/d2`: gate, negative and run yes,
   the scan pre-check FAIL on its two dry-layout lines alone (AK's
   defaults; KL's 24 home-directory lines, all under the dry
   directory), hits 0, errors 0; FINDING lines emitted 0 in each log
   (the run's summary line aside). The run: `--run` the formula
   sample's directory and five `--run-plain` in the usage order,
   each `-43471ed-`; the list 4640 rows (corpus 162, fixture 2239,
   input 2239; 12 pairs left out, input not accepted) at sha256
   dd796c22, 1241251 bytes, mode 600, no row printed; the stage
   mode 700, files 600, 4642 files, files/ counts equal to the
   rows by source, the opener 0f2bed0b equal to the phase commit's,
   staged files differing from their rows 0; the zip 124132653
   bytes, mode 600, sha256 77d12042, 4642 entries stored, testzip
   ok; P1's keep listing 41afdbbf at entry and end. Changes LV
   (scan: EXPECT_RECORDS required and printed, the count of
   `records/excel-check-m7p2-*.md` equal to it, a control) and NH65
   (negative: the fixtures name P1's phase commit by its subject,
   checked as the base's parent) accepted; scan 663febe1, negative
   085e8876. d3 from the start under `<phase>/dry/d3` with
   PLANNING_SPEC this ruling's hash. Planning's word for the stage's
   logged run follows a d3 that reads yes in all with the scan
   pre-check failing on its two dry-layout lines alone.

S. Dry d3 read from `<phase>/dry/d3` under PLANNING_SPEC d823f16e:
   gate, negative and run yes, the scan pre-check FAIL on AK's and
   KL's dry-layout lines alone (24 home-directory lines, all under
   the dry directory), hits 0, errors 0, EXPECT_RECORDS 0 printed
   with its control, NH65's fixture lines yes; FINDING lines
   emitted 0 in each log; the list dd796c22 as d2's; the zip
   dd5c209d, 124132653 bytes as d2's, every entry's CRC equal to
   d2's and every entry's time different (planning compared the two
   zips by `zipfile`): the zip's sha256 follows the copy time and
   the list's sha256 is the stage's stable identity; the logged
   zip's hash is the one the record and the README name, no dry
   hash is expected to match it. Planning's WORD for the stage's
   logged run: `<phase>/logged.sh` on P1's pattern, gate with
   `PRUNE_PREVIOUS=list`, negative, run with STAGE_ROOT unset so the
   stage is `<corpus>/work/excel-check-m7p2-<utc-date>/` and the
   zip beside it, PLANNING_SPEC this ruling's hash; the logs to
   `<phase>/logged/`, not to `records/logs/m7p2-p2/`, until the
   record run (ruling Q (5)); then the README draft (the
   disclosures: d1's TN, d2's LV and NH65, the zip-hash reading
   above), the scan pre-check with EXPECT_RECORDS=0, check 1 on the
   stage run (the run log's directory list against the tool's flag
   list; the stage's counts against the list), the report under
   `<phase>`; no record commit yet, no push. The stage's logs are
   then frozen: no script of the record run rewrites them. The
   maintainer carries the zip by the laptop route and runs the
   opener as "The maintainer's check (P2)" of `specs/0044`; the
   implementing session does nothing on the stage after check 1
   until the results file is back.

T. The stage's logged run read from `<phase>/logged` under
   PLANNING_SPEC 2dd9a9b4: gate (6ce0b431), negative (d760f1c7)
   and run (7c7ab5a2) each ending yes, FINDING lines emitted 0, the
   outer stderr files 0 bytes; the scan pre-check FAIL on AK's and
   KL's lines alone with hits 0 and EXPECT_RECORDS 0 checked. The
   stage `<corpus>/work/excel-check-m7p2-2026-10-28/` read by
   planning: mode 700, 4642 files all 600, every directory 700,
   files/ 162, 2239 and 2239; the list dd796c22 (the dry runs'),
   4640 rows, 13 columns, every row's bytes and sha256 equal to its
   staged file's (planning's own pass over the rows, 0 mismatches,
   no row printed); the opener byte-equal to the tree's; the zip
   df824832, 124132653 bytes, mode 600, 4642 entries all stored,
   testzip clean by planning's own `zipfile` read. The Milestone 2
   and part 1 stages and zips present with their 2026-10-09,
   2026-10-25 and 2026-10-26 times. Check 1 (6ef88673) read yes with
   an exception, yes with exceptions, yes with exceptions; its
   points: (a) the doc commit's subject names rulings M to P while
   its spec diff also carries A to L, never committed before: the
   subject understates, the message's file hashes are exact, ruling
   P asked for "this spec"; recorded, the commit stands, not
   rebuilt. (b) "not read, moved or removed" is proved by no log:
   planning's listing above records it for the stage run; the
   record run's run log prints the two older stages' and zips' names,
   sizes and modification times at its start and end, equal.
   (c) The zip's entry count rested on the run log for the check:
   planning's `zipfile` read above is the second reading. (d) The
   wording points went into the report and README as the session
   said. The stage's logs are frozen. The maintainer carries the zip
   by the laptop route of part 1, runs the opener, copies
   `open-results.txt` back under the stage and gives the machine
   section in the implementing session; nothing else runs on the
   stage until then.

U. The record run, answering the drafted generators (record-run
   d27f3ceb, record-negative 3922d29a, scan-record cc55182a as
   drafts; changes TO to TR, NI65 to NL65, LW). (1) The record run
   has its own gate log, `record-gate.log`, on part 1 P2's gate:
   the environment lines, HEAD, PLANNING_SPEC, the tree's allowance
   (" M specs/0045-publish.md" alone at its start, the record
   written in the run step only, as part 1's ruling W),
   `PRUNE_PREVIOUS=list` and nothing pruned, P1's keep and the two
   older stages checked unchanged. (2) The names `record-gate.log`,
   `record-negative.log` and `record-run.log` beside the stage run's
   `gate.log`, `negative.log` and `run.log`; the record run's final
   scan is the committed `scan.log`; the stage run's pre-check log
   stays under `<phase>/logged` and the README names both runs.
   (3) The negative's assembly stops run on scratch copies of the
   real results file as part 1's ruling W (a planted row, exit 2
   "names no list row"; a removed row, exit 2 "list row with no
   result"), the log printing the exit code and the `STOP:` line
   only, never a row or a value; the fictional fixtures' cases stay
   as further controls. (4) The record carries one table row per
   corpus row (162) and per fixture pair (2239, the input's result
   beside each), 2401 table rows; the run log's pair counts are
   2401 of 2401 corpus and fixture rows found in the list by
   `bytes` and `sha256`, and 2239 of 2239 input pairs, part 1's W
   form. (5) With nothing refused the by-hand file is its header
   line alone, its sha256 and row count 0 printed; a refusal fills
   it as part 1's rulings T and U. (6) Check 2's brief after the
   record run: the three record logs, the scan log, the record, the
   README and the draft report as its only primaries, CLAUDE.md's
   three questions, the stage run's logs named as frozen under the
   README's hashes. The inputs the run script takes come from
   planning's record brief after the results are back: the record's
   sha256, lines and bytes; the by-hand file's sha256 and rows; the
   results file's sha256; the three machine arguments; the ruling's
   letter.

V. The record run's drafts (record-gate c2f8e459 with change A93,
   record-negative 8f9ece7e with NM65, record-run b1ed92d8,
   scan-record e97d9159) are accepted as drafts; the real scripts
   are generated from the record brief. (1) The results file comes
   back under the stage as `open-results.txt`, the opener's own
   name; the maintainer copies it under that name, and a different
   name is one more replacement named in the brief. (2) The stage
   run's three logs move from `<phase>/logged` into
   `records/logs/m7p2-p2/` after the record run's three logs are
   written and before the final scan, by a driver step that copies
   each and prints its sha256 equal to the frozen hash the README
   names (gate 6ce0b431, negative d760f1c7, run 7c7ab5a2); the
   record gate's allowance stays " M specs/0045-publish.md" alone.
   The final scan then reads all six logs and the record in place.

W. The results, read by planning from the stage (2026-10-28): the
   file `open-results.txt` sha256 84bff35d, 4641 lines, 527093
   bytes, 4640 rows in the list's order, one result per list row,
   no row repeated, no path-like text, one generic message on
   every `fail` (722) and none elsewhere. Corpus: 159 `pass`, 3
   `fail`; the `host` row, chart-surface, protect and chartsheet
   27 of 27 each; 53 `seen` values compared with `expect`, 0
   differing. Fixtures: the six re-kept pairs and the one-write
   test's three pairs `pass` on both sides; pairs pass/pass 1857,
   refused/refused 337, refused/pass 21, pass/refused 24, the 24
   exactly P1's run log's allowed list. The 3 refused corpus rows
   are the edit sample on `054df34b1472` and `f8459281d739` and
   the remove sample on `18d1e8ba72b5`: the same three workbooks
   as part 1's findings 1 and 2, their outputs byte-identical to
   part 1's (bytes and sha256 equal to the part 1 record's rows,
   which read `repair (by hand)`), the two samples no fix touched
   because the maintainer decided on 2026-10-26 to document their
   causes as limits (the table header edited, the removed sheet's
   references). Decision 3 A's words "every row pass" did not
   carve those rows out: a planning error in this spec, recorded
   here, not rewritten. Decision 6 below puts the acceptance to
   the maintainer. The results route: at the maintainer's request
   in the implementing session the zip went to the Windows host
   and back by the maintainer's scp, the results file copied into
   the stage under the opener's name (sha256 equal at every step;
   the stage otherwise untouched: list dd796c22, opener 0f2bed0b,
   4642 files mode 600 besides it); accepted; the host's copies go
   at the maintainer's word; no host path enters a log, a record
   or a message. The Excel line is part 1's, the maintainer having
   said it is the same build; the OS and date lines verbatim.

X. The assembly script for this record is part 1's with these
   changes, as `assemble-excel-record-p2.py` (sha256 7f65c2f7)
   under `<phase>/record-from-planning/`: the title and the opening
   paragraph name this spec, the stage, P1's phase commit 43471ed,
   its six sample records and its open record; the by-hand bullet
   is computed from the by-hand file (rows, `pass`, repair offered,
   repair failed); the totals table is followed by part 1's totals
   and pair counts, the difference the fixes made (ruling C); the
   findings' opening names the two limits, each corpus finding
   reads "refused by the script ... the by-hand outcome in the
   row", the pass/refused pairs are named as the 24 allowed
   (ruling C); the acceptance paragraph reads decision 6 A. The
   tables, the machine section, the stops and the counting are
   unchanged (diff: 30 lines against 54f344a7). A trial run on a
   scratch by-hand file of three `repair (by hand)` rows printed
   2461 lines and 2401 table rows; the trial and its by-hand file
   were removed; the real assembly runs once the maintainer's three
   outcomes are in, and the record run's inputs name this script's
   hash in place of 54f344a7 (ruling U (6), the brief).

Y. The record, assembled by planning on 2026-10-29. The maintainer's
   three by-hand outcomes, typed in the implementing session on
   2026-10-29 and relayed verbatim: `repair` on each of the three
   files (the edit sample's `054df34b1472` and `f8459281d739`, the
   remove sample's `18d1e8ba72b5`); no `repair failed`. The by-hand
   file `byhand-2026-10-28.tsv` (named by the record's date, the
   run's) under `<phase>/record-from-planning/`: sha256 73b8e06d,
   3 rows, each cell `repair (by hand)`, the label column the
   list's own labels. A first by-hand file with the bare sample
   names `edit` and `remove` as labels stopped the script with
   exit 2 ("by-hand row is not a refused corpus row"): the stop
   working as ruling U (4) says; the file was rewritten with the
   list's labels and the assembly run once more with that fix
   named here. The by-hand opening was a day after the run, so the
   script's by-hand bullet no longer reads "the same day": it reads
   "on 2026-10-29 (the run was 2026-10-28)", the one line changed
   against ruling X's 7f65c2f7 (the reversed change hashes back to
   7f65c2f7); the script is now sha256 35f29a5f, and every pin of
   the assembly script in the record run names 35f29a5f and the
   path `<phase>/record-from-planning/assemble-excel-record-p2.py`.
   The record `excel-check-m7p2-2026-10-28.md` under
   `<phase>/record-from-planning/records/`: sha256 6ca1f6e8, 2461
   lines, 637291 bytes, 2411 table rows, non-ASCII 0, no path, no
   name, no cell content. Planning checked it row by row against
   the stage: the 162 corpus rows equal the list's bytes and
   sha256, the results' cells and `seen` values, in the list's
   order, the three by-hand cells in place; the 2239 pair rows
   (output side, then input side) equal the list and the results,
   no pair repeated or missing. Totals 4640; corpus 159 `pass`,
   3 `repair`, 0 `fail`; fixture 1878 / 0 / 361; input 1881 / 0 /
   358; pairs as ruling W; 53 `seen`, 0 differing; part 1's totals
   beside them. Findings 1 and 2 are the two limits' instances,
   finding 3 the 24 allowed pairs. The acceptance reads decision 6
   A: every row `pass` save the limits' three rows; accepted. The
   twelve inputs of the record run: RECORD_DATE 2026-10-28,
   REC_SHA 6ca1f6e8, REC_LINES 2461, REC_BYTES 637291, BYHAND_SHA
   73b8e06d, BYHAND_ROWS 3, RESULTS_SHA 84bff35d, RESULTS_REL
   `open-results.txt` under the stage (ruling V (1), no
   replacement), EXCEL_ARG, OS_ARG and DATE_ARG the lines ruling W
   names, RULING Y. The record run's checks that count the corpus
   read 159 `pass`, 3 `repair`, 0 `fail` (decision 6 A), not
   "every corpus row pass". The record's totals are what P3's
   README sentence cites.

Z. The record run's dry runs and the changes they named (2026-10-29,
   read by planning from `<phase>/dry/record-d1` to `record-d5`, the
   generators and the superseded copies). The dry driver
   `<phase>/dry-record.sh` (sha256 67379f74) runs the three record
   scripts serially with every build root, the records destination
   and the work directory under the dry run's own directory
   (ruling Q's form), the tree never written. d1: the gate read no,
   its one UNEXPECTED its own standard error: change A93's start
   check called `note_fn` before the script defines it, a bug in
   the accepted draft c2f8e459. d2: the negative read no on NF42's
   placeholder rule, `<stage>` printed and declared by no root
   line. d3: the run stopped at entry, reading the record under
   `<phase>/record-from-planning/` where ruling Y put it under
   `records/`. d4: all yes, but NM65 printed a STOP line whole,
   one corpus row's key after the message (ruling U (3): never a
   row); the d4 log stays under the dry directory, uncommitted.
   d5: all yes; gate 8c6197b6, negative be24a5ff, run 7857205d; 0
   FINDING lines printed (the word occurs 17 times in the run log,
   every one in the script's carried text or its summary line);
   the record copy and the assembly's output both sha256 6ca1f6e8
   and cmp-equal; the totals recomputed equal the record's; the
   corpus counts 159 / 3 / 0 with the results-alone control
   159 / 0 / 3 refused; 3 of 3 by-hand cells `repair (by hand)`,
   3 of 3 naming a `fail` row; 2401 of 2401 and 2239 of 2239; the
   stage and the older stages unchanged; the gate's two `no` lines
   are answers over its planted control script, its control line
   as-expected. The changes, each accepted: A94 (the gate,
   generator 353b206b, script faceefd0): A93's check calls `note`
   with `is`. NN65 and NO65 (the negative, generator 8a5a2f39,
   script 5b2cb9df): a root line declares `<stage>`; the STOP
   lines are cut after the message's colon, part 1's form. TP's
   path and TS (the run, generator d3f554de, script 590c78b0):
   the record read under `records/`; the twelve inputs of ruling
   Y filled; the assembly script pinned at 35f29a5f under its
   part 2 name in the run and the negative, part 1's 54f344a7 kept
   only to refill the template; TS reads decision 6 A (the corpus
   counts 159 / 3 / 0, the by-hand cells, the by-hand rows naming
   refused rows, each with a control); the Excel line left off the
   stale list, being part 1's verbatim (ruling W). These three
   scripts replace ruling V's drafts c2f8e459, 8f9ece7e and
   b1ed92d8; the driver `record-logged.sh` stays 14fd7f3e. R1:
   `<phase>/negative`, the stage negative's build root (one empty
   `tmp/`, 2026-10-27), would be refused by NG62 at the logged
   run's step 2. Ruled (a): the implementing session moves it to
   `<phase>/superseded/stage-negative-build` by `mv` before the
   logged run, the driver unchanged; the logged README names the
   move. The logged run reads PLANNING_SPEC as the working spec
   stands with this ruling in it.

AA. Change LX, ruled after the fact (2026-10-29). The record scan's
   LW check kept P1's form ("every record named by the short sha is
   a phase record"), while this phase's one record carries the date
   alone, so the scan's first pre-check read FAIL on three structural
   lines, hits 0: two from check 1's answer, installed before the
   pre-check and held aside for it, and LW's. Change LX gives the
   check part 1 P2's LN form ("no record is named by this phase's
   short sha"; generator e3bb9c6a, script 7a58ada4); the second
   pre-check read PASS (83cd6d33) and the final scan PASS, hits 0,
   structural 0, check answers 2. The change was made and run after
   ruling Z and before any ruling named it: a failing run recorded,
   fixed and run again with the fix named, as `CLAUDE.md` asks, but
   the ruling should have come first; accepted here, not rewritten.
   The generator's chain (b8a430de at the record gate, 7d03b6cf
   with the date filled, e3bb9c6a with LX) stands in the README.

## Planning verification of P2 (2026-10-29)

Read from the primaries, not the report: the record commit 16c083f
(parent 5ae8367, 11 files added: `records/excel-check-m7p2-2026-10-
28.md` and the ten files under `records/logs/m7p2-p2/`; the message
ASCII with the Co-Authored-By line only, the count sentence carried,
its form the P1 record's save the trailer's model name, which that
session's harness wrote without "(1M context)"; noted, not
rewritten); eleven commits ahead of origin/main aed8087, no push;
the tree after it " M specs/0045-publish.md" alone. The record is
sha256 6ca1f6e8, byte-equal to planning's (ruling Y). The stage
logs are the frozen ones (gate 6ce0b431, negative d760f1c7, run
7c7ab5a2); the record logs (record-gate c3d6e324, record-negative
77febe49, record-run b6688a63) each read PLANNING_SPEC 00243056,
the working spec with ruling Z, and each ends "checks all as
expected: yes" (13:22:25Z to 13:22:28Z). The lines ending ": no"
outside the printed scripts are the API-key line in each and, in
the record gate, two answers over its planted control script, its
control line as-expected. FINDING lines emitted: 0 in each; the
word occurs in the record-run log alone, 17 times, in the printed
script and its heading and summary. The record run: the copy and
the assembly's output both 6ca1f6e8 and cmp-equal; 2401 of 2401
rows found in the list and 2239 of 2239 input pairs; the totals
recomputed equal the record's; the corpus counts 159 / 3 / 0 with
the results-alone control 159 / 0 / 3 refused; 3 of 3 by-hand
cells `repair (by hand)`, 3 naming a `fail` row, 0 a `pass` row;
the stage's listing and the older stages unchanged. The record
negative: AC assembles to 6ca1f6e8; A1 and A2 stop with exit 2 and
their messages, the row key cut. R1 (a) done: the README names the
`mv` of the stage negative's build root to
`<superseded>/stage-negative-build`; the driver 14fd7f3e unchanged.
The scan (1156dc20): RESULT: PASS, total hits 0, structural 0,
check answers 2, after change LX (ruling AA). Planning's own scan
of the eleven files: no name, no address, no home or host path, no
session link, nothing outside ASCII; the only `Users/` matches are
the split control fixtures the gate and the scan print. Check 2
(4759222a, read whole): 1 yes; 2 yes with E1 (the copy step's
hash line is the driver's output, not a log) and E2 (the pre-check
read an earlier README); 3 yes with R1 to R4; W1 and W2 carried
template wording; the strictness note on `pairs.awk`; no
disagreement with the record's figures; the README's "After check
2" dispositions read and agreed. P2 is DONE 2026-10-29 under
decision 6 A: every corpus row `pass` save the two limits' three
rows, opened by hand with a repair offered; every fixture pair as
P1 allowed. P3 next; nothing of P3 runs until the P3 doc commit is
in.

## Planning rulings during P3 (2026-10-29)

AB. The CHANGELOG heading. It reads "## 0.1.0 (unreleased)" today
   and stays so through P3: the date is the close-out doc commit's,
   written there ("The release texts and the manifest (P3)"). P3's
   run log checks the heading's form alone, "## 0.1.0 (" followed
   by one parenthesised word or date, with a control refusing a
   heading of another version, and prints the heading as it stands.

AC. What P1 wrote counts. The two limits stand in the README's "Out
   of scope by design" and the CHANGELOG's "Not supported by design"
   since 43471ed and are not written again; P3's run log reads them
   there. P3's docs commit writes: the CHANGELOG's "Fixed", five
   entries in the order of "The fixes (P1)", each naming the fix as
   the spec names it, this spec and part 1's record
   (`records/excel-check-m7-2026-10-26.md`); the CHANGELOG's
   "Measured" and the README's "What has been measured", each
   gaining the two Excel sentences, one per record, in the form the
   section's sentences have (the figures, then the record's path in
   parentheses). Part 1's figures are the spec's ("The release texts
   and the manifest (P3)"); this part's, from the record's totals
   line (6ca1f6e8, ruling Y): 4640 rows, corpus 159 pass and 3
   repair (the two documented limits' three outputs, opened by hand
   with a repair offered), fixture 1878 pass and 361 refused, input
   1881 pass and 358 refused (`records/excel-check-m7p2-2026-10-28.
   md`). The run log prints every figure of both sentences by a
   command over the records beside the sentence, with a control
   refusing a planted figure (the negative log). The crate-level doc
   comment is read against the README's first paragraph and printed
   unchanged unless a sentence contradicts a fix, in which case the
   log names the sentence and the docs commit changes it.

AD. The P3 doc commit: the form of 5ae8367 (each file "<path>
   sha256 <sha256>", the count sentence, the Co-Authored-By line
   only), subject "M7 part 2 P3: commit planning docs (P2's
   verification and rulings Q to AD)", holding this spec and
   `ROADMAP.md` (the P2 row DONE, the P3 row IN PROGRESS). The code
   commit holds `crates/emend/Cargo.toml` alone; the docs commit
   `CHANGELOG.md` and `README.md` (and the crate-level doc comment
   if AC changes it); the record commit the logs, the README and the
   one check under `records/logs/m7p2-p3/`.

AE. P3's phase directory is `<home>/.cache/emend-build/m7p2-p3`,
   its generators P2's (`<home>/.cache/emend-build/m7p2-p2/gen`)
   with the part's names as numbered changes, the gate's prune list
   naming no build target of P2's (the stage and record runs built
   nothing of the crate). The packaging dry run of "The release
   texts and the manifest (P3)" is the run log's; the archive is
   extracted under the phase directory, outside any git checkout;
   the log prints `CARGO_REGISTRY_TOKEN` and `CARGO_REGISTRIES_
   CRATES_IO_TOKEN` absent, the credentials file not read (the dry
   run passes `--allow-dirty` only if the tree is dirty at that
   point, and the log says which). A dry driver as ruling Q; every
   FINDING line listed by grepping the word (ruling R). The scan
   reads the four commits, EXPECT_RECORDS=0.

AF. The negative log's packaging case, read by planning from the
   implementing session's trial (2026-10-29): the spec's words for
   it ("with `template/_rels/.rels` taken out of the `include` list,
   the verify build failing as the finding did") do not fail, cargo's
   include globs matching dotfiles, so `template/**` carries the
   file: a planning error in this spec, recorded, not rewritten. The
   negative log runs three cases over the archive extracted outside
   any checkout: (a) the refusal, the manifest of the code commit's
   parent 612986a (no `include` list) packaged, `cargo package
   --offline` exit 101 naming `_rels/.rels`, `specs/0031`'s finding
   reproduced, its list without the file; (b) information, the code
   commit's list less its `template/_rels/.rels` line, the file
   still packaged and the count printed, so the by-name line is
   belt and braces and the list as a whole is the fix; (c) the
   control, the code commit's manifest, exit 0, the file listed.
   The by-name line stays as the spec says. The trial's counts (the
   checkout 73 entries, the archive 72, the pre-P3 manifest 71) are
   the trial's; the logged run prints its own.

AG. The texts. The three commits read from their diffs: 612986a
   (this spec 8a515384 and `ROADMAP.md` 884c6e33, the form of
   5ae8367), 790c488 (the manifest alone: `publish = false` gone,
   the seven-entry `include` list with the dotfile by name, a
   comment citing this spec and `specs/0031`), b016818 (the
   CHANGELOG's five "Fixed" entries in P1's order, each citing this
   spec by fix number and part 1's record; the two Excel sentences
   in "Measured" and in the README, their figures equal to the two
   records' totals lines, part 1's 6018 / 1528 / 32 / 1864 / 365 /
   1874 / 355 and this part's 4640 / 159 / 3 / 1878 / 361 / 1881 /
   358; the heading unchanged; nothing outside ASCII). The wording
   "fixed five of the causes and documented the rest as limits" and
   "every sample's outputs on the private test corpus" for part 1,
   and "six samples' outputs over the 27 workbooks" with the three
   repairs named for this part, are confirmed. One change, one more
   docs commit before any log, the form of b016818: "Microsoft Excel
   opened the N files" reads "Microsoft Excel was asked to open the
   N files" in all four places (the sentence goes on to count the
   refused). The same commit adds `charts` to the crate-level doc
   comment's feature list, the line it lacks for the feature the
   manifest has since Milestone 5: "- `charts`: charts read, added,
   formatted and removed, and chartsheets.", after the `copy` line;
   the run log prints the list and the manifest's feature names
   side by side, each of the manifest's named in the list, with a
   control refusing a list lacking one. The doc comment's first two
   paragraphs equal the README's word for word, as read.

AH. P3's dry runs, read by planning from `<phase>/dry/d1` to `d3`
   (2026-10-29). The docs commit f7cabba, the phase commit, read
   from its diff: "was asked to open" in the four places, the
   CHANGELOG items rewrapped, the one `charts` line as ruling AG
   gives it, the three chart modules gated on the feature. d1: the
   gate read no, its message check pinning part 1's trailer with
   "(1M context)"; change T95 takes the harness's line and makes
   the older form a refusal control (the scan's LY the same). d2:
   all but the scan yes; the run printed a flag twice in one
   sentence, a shell expansion slip, fixed in TU's echo with the
   reason in the line; the superseded run script was removed by a
   plain `rm -f` in the implementing session's shell after its
   copy to `<superseded>/run-d2`: recorded; a superseded script is
   moved, not removed. d3: gate de1b2023, negative 801833ab, run
   cb47c395 yes, 0 FINDING lines (the word in the run log 13 times,
   in its printed script, its heading and its count); the scan
   pre-check 512c02b1 FAIL on its two dry-layout lines alone, 0
   hits, as P2's dry runs read. Read in d3: the archive of 2639
   files outside any checkout, `--allow-dirty` not passed there,
   its list 72 entries equal to the checkout's 73 less
   `.cargo_vcs_info.json`, `template/_rels/.rels` once in each;
   `cargo publish --dry-run` exit 0, "Packaged 72 files", the one
   "aborting upload" warning, no other; the two token variables
   absent with controls; the four sentences each holding their
   record's seven figures, a planted figure refused; the heading's
   form with three controls; the limits read in both sections; the
   five Fixed items in order; 12 of 12 features named, a copy
   lacking `charts` refused; the negative's (a) exit 101 naming
   `_rels/.rels`, 71 entries, (b) 72 with the file, (c) exit 0.
   The readings: G1, the tree allowance in the carried status
   section's form (the spec at PLANNING_SPEC, `ROADMAP.md` at
   PLANNING_ROADMAP, `records/logs/m7p2-p3/`), accepted; G2, the
   prune empty, P1's directory checked unchanged, P2's not named,
   the crate's diff Cargo.toml and lib.rs alone, accepted; R3, the
   script's statement is enough (it names, opens and reads no
   credentials file, runs no `cargo login`, gives the dry run no
   token) and CARGO_HOME is not moved; the run log adds one
   information line, whether `<home>/.cargo/credentials.toml`
   exists, yes or no and never a byte of it, a FINDING for the
   maintainer if yes (the token belongs on the laptop alone); S1
   and S2, the scan over the five commits with the record commit
   last, EXPECT_RECORDS=0 counting records the base lacks, with
   the planted control at 1, accepted. N4, a change: the figure
   check also matches the seven figures in the totals line's
   order, as whole words with any text between, in each sentence,
   so a swap of two figures is refused; the negative's control
   swaps two figures of a copy and the check refuses it. The
   change and R3's line go through a dry run d4; if d4 reads yes on
   the gate, the negative and the run, the pre-check failing on its
   two dry-layout lines alone with 0 hits, the logged run follows
   without a further hold, d4 reported with it; otherwise the
   report comes first. PLANNING_SPEC for the logged run is this
   spec with this ruling in it.

## Planning verification of P3 (2026-10-29)

Read from the primaries, not the report: the record commit 85dd323
(parent f7cabba, 6 files added under `records/logs/m7p2-p3/`; the
message ASCII with the Co-Authored-By line only, the count sentence
carried, its form the P2 record's); sixteen commits ahead of
origin/main aed8087, no push; the tree after it " M specs/0045-
publish.md" alone. The five commits of the phase, each read from
its diff earlier (rulings AG and AH): 612986a, 790c488, b016818,
f7cabba, 85dd323. The three logs (gate 9e76fec0, negative 8588b183,
run 906e641b) each read PLANNING_SPEC 64044f95, the working spec
with ruling AH, and each ends "checks all as expected: yes"
(14:28:08Z to 14:35:32Z); the lines ending ": no" outside the
printed scripts are the API-key line in each, the gate's two
answers over its planted control script, and the swapped copies'
"in order: no" in the run and the negative, each the refusal a
control expects. FINDING lines emitted: 0 in each; the word stands
in the run log alone, on 16 lines and 18 times (ruling AH's "13
times" for d3 was a line count; check 1's D1). The run: the archive
outside any checkout, `--allow-dirty` passed in the checkout (one
status line, the spec) and not there; the lists 73 and 72 equal
less `.cargo_vcs_info.json`, `template/_rels/.rels` once in each;
`cargo publish --dry-run` exit 0, "Packaged 72 files", the one
"aborting upload" warning and no other; both token variables
absent with controls; `<home>/.cargo/credentials.toml` absent
(existence alone, change TY), so no R3 finding; the four sentences
each holding their record's seven figures and in the totals line's
order, the swapped copies refused (change TY); the heading's form
with its three controls; the limits read in both sections; the
five Fixed items in order citing part 1's record; 12 of 12
features named. The negative: (a) exit 101 naming `_rels/.rels`
with 71 entries, (b) 72 with the file, (c) exit 0; the planted
figure and the swapped copy refused (changes NS65, NU65); the copy
lacking `charts` refused (NT65). The scan (33484899): RESULT: PASS,
total hits 0, structural 0, check answers 1, record files expected
0 (change LZ). Planning's own scan of the six files: no name, no
address, no home or host path, no session link, nothing outside
ASCII. Check 1 (700798b8, read whole): 1 yes; 2 and 3 yes with
exceptions E1 to E3 and D1 to D5, none a disagreement on the diff,
the figures or a verdict; it read fixes 2, 3 and 5's text against
the code; the README's "After check 1" dispositions read and
agreed. Its observation on the `include` entry `README.md`, which
names no file under the crate while cargo packages the readme
through the `readme` key: the entry stays, naming the file as the
package carries it (the spec's "the README path cargo resolves"),
cargo warning about neither form; no code commit. P3 is DONE
2026-10-29. The close-out next; nothing of it runs until the
close-out doc commit is in.

## Planning rulings during the close-out (2026-10-29)

AI. The close-out doc commit holds this spec, `ROADMAP.md` (the P3
   row DONE, the Close row IN PROGRESS) and `CHANGELOG.md`, whose
   heading becomes "## 0.1.0 (<date>)" with the UTC date of the day
   the commit is made, nothing else of the file changed; its
   subject "M7 part 2 close-out: commit planning docs (the P3
   verification and rulings)", its body the three "<path> sha256
   <sha256>" lines and the Co-Authored-By line, the form of aed8087
   (no count sentence, the message naming no count).
   The pre-mode run checks the heading's date equal to the doc
   commit's committer date in UTC (`git log -1 --format=%cd
   --date=format-local:%Y-%m-%d` under TZ=UTC), with a control
   refusing another date. The rest as "Close-out push (after P3)":
   the generated scripts as part 1's close-out with the named
   differences, pre mode on a clean tree, the two Excel-record
   checks, the tag `v0.1.0` created on the tip after the pre checks
   pass with the one-line ASCII message "emend 0.1.0", the pre
   report to planning, planning's word, the maintainer's typed
   authorisation in the implementing session, one `git push
   --follow-tags`, the CI run, the post scan, the whole-history scan
   at the tag beside the 2026-10-22 counts, the record commit with
   the three logs, one check, the report. The tag is not pushed, and
   nothing is pushed, before both the word and the authorisation.

AJ. Finding C1, read by planning from `scripts/install-hooks.sh`
   (2026-10-29, after the close-out doc commit ebe6bc7): the
   pre-push hook blocks any pushed ref whose object is not a commit
   ("privacy.sh checks commits. An annotated tag's own message is
   not checked, so pushing anything but a commit is blocked"), so
   `git push --follow-tags` with an annotated `v0.1.0` would be
   refused whole, branch included, and the hook is not bypassed. The
   spec's "annotated tag" (the Goal, "Close-out push (after P3)",
   ruling AI) is superseded, not rewritten: the tag `v0.1.0` is a
   lightweight tag on the tip, created after the pre checks pass,
   its object the tip commit, so the hook scans it as a commit; the
   push is one `git push origin main v0.1.0`, both refs named, no
   `--follow-tags` (which carries annotated tags alone). The hook,
   finding the tag ref absent on the remote, scans the whole history
   of the tip in the push; its counts print in the push log beside
   the generator's own whole-history scan at the tag and the
   2026-10-22 counts (825 commits, 3484 blobs, 0 findings). `git
   clone --branch v0.1.0` and `.cargo_vcs_info.json` (P4) are the
   same for a lightweight tag. Decision 7 puts the tag's form to the
   maintainer; this ruling and the decision go into a second
   close-out doc commit, "M7 part 2 close-out: commit planning docs
   (ruling AJ and decision 7)", aed8087's form, so the tree is clean
   for the pre mode. The hook's tag rule itself is left as it is; a
   change to it is a phase of its own, not this close-out's.

## Decisions for the maintainer

Decided by the maintainer 2026-10-27: 1 to 5 as recommended (A in
each), typed in the planning session. Decision 6 decided 2026-10-28:
A, typed in the planning session.

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
6. The three refused corpus rows (ruling W): A (recommended), the
   maintainer opens the three files by hand on the laptop as part
   1's rulings T and U (`repair` or `repair failed` per file), the
   by-hand file carries the three rows, the record names them as
   the two documented limits' instances, and the acceptance reads
   every row `pass` save those three; P2 DONE, P3 next. B, the
   same without a re-opening: part 1's by-hand cells carried onto
   the identical bytes, the record saying so. C, back to P1 to
   fix the two limits' causes, overturning the 2026-10-26 choices.

7. The tag's form (ruling AJ, finding C1): A (recommended), a
   lightweight tag `v0.1.0` on the tip, pushed with the branch in
   one `git push origin main v0.1.0`, the hook scanning the tip's
   whole history as a commit; the maintainer's typed push
   authorisation in the implementing session, naming the branch and
   the lightweight tag, is this decision's A. B, the branch pushed
   alone now and the maintainer creates the annotated tag on the
   laptop from the clean clone at publication, P4 reading it back;
   nothing tagged here. C, a hook change first (an annotated tag's
   message checked as a commit message, its target scanned), a
   numbered code commit with its own gate and logs before the push:
   back to a phase.

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
