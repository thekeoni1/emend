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
