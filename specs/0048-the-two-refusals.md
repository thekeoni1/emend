# 0048: Milestone 8 part 2, the two refusals behind their features

Status: APPROVED 2026-11-09 (decisions 1 to 3 A, typed in the
planning session: "great, i agree" to planning's recommendation of
A in each); the milestone APPROVED 2026-11-07 (Milestone 8,
decisions 1 to 4 A, `ROADMAP.md` "Milestone 8"); part 1 DONE and
PUSHED 2026-11-09 (`specs/0047`). P1 DONE 2026-11-09 at cbef379
(rulings A and B); P2 DONE 2026-11-09 at 572e71f (rulings C and D);
P3 DONE 2026-11-10 at 053d27c under Amendment 1, decision 4 A
(rulings E to M); P4 DONE 2026-11-10 at 5def62f (ruling N); the
close-out DONE and PUSHED 2026-11-10 at 9f862ca (ruling O; the
record f44debd local, it goes with part 3's push). PART DONE.

Milestone 8 part 2 implements the milestone's decisions 2 and 3 and
part 1's decision 2: the two documented refusals Milestone 7 part
3's P2 met on the grown corpus (`records/logs/m7p3-p2/README.md`,
"The findings", groups 3 and 4) become edits behind their features,
and `remove_sheet` refuses a sheet a pivot cache sources (group 5,
`specs/0047` decision 2 A). Each change is proven by tests on built
fixtures, by the samples over the 39 corpus workbooks compared per
id with part 1 P2's records, and by the size record against the
ceiling. Part 3 (`specs/0049`) runs the Excel check over this
part's changed samples and releases 0.1.1.

## Goal

1. The Cargo feature `rowcol`: an insertion or deletion that would
   grow or cut a shared formula group's `ref`, or that deletes the
   master's row or column while a member stays, unshares the group
   before the shift (Milestone 8 decision 2 A): the master and each
   member get their own `<f>` holding the master's text translated
   to the cell, with no `t`, `ref` or `si`; then the rows or columns
   shift as today, formula text untouched by the shift as the
   crate's definition says. A group the rule moves whole, or leaves
   where it is, stays shared. Array and data table formulas stay
   refused with the message as it is.
2. The Cargo feature `comments`: a worksheet with a threaded
   comments relationship takes `add_comment`, `add_rich_comment`,
   `set_comment_box` and `remove_comment` on a cell no threaded
   comment names, writing the legacy comments part beside the
   threaded one as on any sheet (Milestone 8 decision 3 A); a call
   on a cell a threaded comment names is refused with a new message.
   `copy_sheet` keeps the sheet-level refusal (decision 2).
3. The core: `remove_sheet` refuses a sheet a pivot cache sources
   (`specs/0047` decision 2 A), judged by the size record against
   the ceiling.
4. The size record, the README's feature figures and the
   CHANGELOG's "Unreleased" entries for the three changes.
5. The 59 runs rerun over the 39 workbooks with every output
   opened, compared per id with part 1 P2's records
   (`records/m8p1-<name>-2026-11-09-8816db0`), the expected changes
   enumerated below and nothing else changed; a finding recorded
   and fixed in the part, or carried with a reason.

## Size

Code under `rowcol` and `comments` changes, and code the core links
(`remove_sheet`). The part's size record is P3's, in Milestone 7
part 2's form (`records/size-m8p2-<utc-date>-<shortsha>.md`), over
`records/size-m7p2-2026-10-28-43471ed.md` as the base figures, each
probe rebuilt at 43471ed by the same log as that record's. The
budget (`probe-emend-format-write-xml` over `probe-zip-xml`,
451512 at 43471ed) is expected to grow 300 to 2500 section bytes
for the pivot cache source check and is accepted up to 4000; it
must stay under the ceiling 458752 (headroom 7240). A budget at or
above the ceiling stops P3 before the record commit: the part
stops, planning writes an amendment whose first decision reopens
Milestone 8 decision 1 as that decision says, with the refusal
narrowed to the `sheet` attribute alone as the recommended option.
The `rowcol` feature's cost (58244 at 43471ed) is expected to grow
3000 to 12000 for the translator and the unshare and is accepted up
to 16000; the `comments` cost (63440) 500 to 3000, accepted up to
5000. Every other probe that links emend is expected to grow by 0
to the core's growth and accepted up to it plus 2000; the three
that link no emend 0 and up to 2000. A growth outside its band, or
a shrink, is a finding, recorded and not fixed; each movement is
attributed by file as part 7's method does. P1 and P2 run the
`size` profile build of their probes as a check only (no record):
the rowcol probe after P1 and the comments probe after P2, each
figure in the phase's run log and judged against the band here.

## Facts planning checked (2026-11-09)

- The shift (`crates/emend/src/package.rs`, `#[cfg(feature =
  "rowcol")]`): `shift_worksheet_from` streams the worksheet once,
  reads every cell's `<f>` start tag whether the row is kept or
  deleted ("its formulas are still read") and passes it to
  `shifted_formula`, which returns the bytes unchanged for a tag
  whose `t` is not `shared`, `array` or `dataTable` or that has no
  `ref`, moves a `ref` the rule moves whole, and returns
  `CUTS_FORMULA` ("an insertion or deletion cuts a shared, array or
  data table formula") when the rule changes the interval, when the
  master lies in a deleted row with its `ref` kept whole, and when
  the moved end passes the grid with the start inside it. A member
  (`<f t="shared" si="N"/>`, no `ref`) passes through unchanged.
  `formula_tag` reads `t` and `ref`; `cells.rs`'s `read_formula`
  reads `si` and refuses a shared formula without one ("a shared
  formula has no index"). The cell-edit path refuses an edit on a
  cell in a shared or array formula (`IN_SHARED_OR_ARRAY`), out of
  this part's scope. `specs/0018` "The shift rule" says "Formula
  text, `si`, `r1` and `r2` are never touched"; decision 2 A
  changes that for the groups the unshare takes.
- The threaded comments refusal: `THREADED_COMMENTS` (one URL,
  `.../office/2017/10/relationships/threadedComment`, written
  twice) is tested at the write in `record_comments`, once per
  edited part, raising `THREADED_COMMENTS_HELD` ("a worksheet has
  threaded comments"), and in `record_copies` for a copy's source.
  No code reads a threaded comments part; `Package::comments`
  reads the legacy fallback Excel writes beside it. Tests:
  `comments::a_sheet_with_threaded_comments_is_refused` and the
  copy test in `copy.rs` that refuses a copy of such a sheet.
- `remove_sheet` (the core): the projected list, `LAST_SHEET`,
  `LAST_VISIBLE_SHEET`, every read before the first record (P4
  ruling 5 of `specs/0016`), the reachability walk, the calc chain.
  `PIVOT_CACHE_DEFINITION` names the relationship type; the
  workbook's `<pivotCaches>` is in the root-children order list.
  The pivot cache reader (`package/pivot.rs`, `PivotSource`
  reading `worksheetSource`'s `sheet`, `ref` and `name`) is the
  Cargo feature `read`'s; the core has none.
- The four workbooks, read by a planning script over the corpus
  printing counts and booleans only. 90a5b01f9541's first
  worksheet: 1 shared master, 6 members, no array or data table
  formula; the row sample's point (row 2) cuts the master's `ref`,
  the column sample's (column 2) does not. 91d2bdffa46a's: 2
  masters, 3 members; the row point 2 meets a `ref` lying wholly in
  the deleted row (allowed today, part 1's delete-rows record ok),
  the column point 2 cuts one master. In both, every member follows
  its master in document order, every master carries text and no
  member does. 15205770ae5c: one threaded comments part under
  `xl/threadedComments/`, root `ThreadedComments` in the 2018
  `threadedcomments` namespace (below), 1 `threadedComment` with a
  `ref`; a persons part; the worksheet
  holds threadedComment, comments and vmlDrawing relationships; its
  legacy part holds 1 comment and 1 author starting `tc=`; its
  last row is 46, so the comment samples' cell is row 47, column 1.
  495c7b79b183: 2 sheets; 1 `<pivotCache>` in the workbook with 1
  pivotCacheDefinition relationship; its source `type="worksheet"`
  with a `sheet` attribute naming the last sheet, no `name`, no
  `r:id`; 2 defined names; the last sheet has no table, pivot
  table or slicer part.
- Part 1 P2's records for the four ids (outcome, exit, lo, lo_in):
  delete-rows 90a5b01f9541 error 2 - ok; delete-columns
  91d2bdffa46a error 2 - ok; comment, richcomment, commentbox and
  copy 15205770ae5c each error 2 - ok; remove 495c7b79b183 ok 0
  fail ok; every other row of the four ids in the nine samples
  insert-rows, delete-rows, insert-columns, delete-columns, remove,
  comment, richcomment, commentbox and copy is ok 0 ok ok.
- The samples (`tools/emend-copy/src/main.rs`): `shift_sample`
  takes the first worksheet and the point `max(2, T + 1)`, T the
  last row or column of its table and pivot table ranges; a
  deletion removes one row or column there. `remove_sample`
  removes the last sheet and treats "the last sheet cannot be
  removed" and "the last visible sheet cannot be removed" as a
  removal of nothing, the output written and the record's removed
  count 0. `comment_sample`, `commentbox_sample` and
  `richcomment_sample` take the first worksheet's cell at its last
  row plus one, column 1; `copy_sample` copies the first worksheet.
- Milestone 7 part 2's size record: the budget 451512; the
  `rowcol` cost 58244; the `comments` cost 63440; the README's
  feature list (lines 24 and 28) cites Milestone 6 part 6's 58260
  and 63424 for them.

## Design

### The unshare (`rowcol`)

A shared formula group is the master `<f t="shared" ref="R"
si="N">text</f>` and every `<f t="shared" si="N"/>` of the same
worksheet with the same `N`. The shift pass decides at the master,
from `shifted_formula`'s cases:

- The rule moves `ref` whole or leaves it: the group stays shared
  and `ref` moves as today.
- The rule changes the interval (grows or cuts it), or the master's
  row or column is deleted while `ref` is not wholly inside the
  deletion: the group is unshared. The master's `<f>` is written
  with `t`, `ref` and `si` removed and every other attribute kept;
  its text is kept. The master's unescaped text, cell and `si` are
  held for the pass. Each later member of that `si` is written as
  `<f>` with `t` and `si` removed, every other attribute kept, and
  the text the translator gives for the member's cell; a member
  that carries text of its own keeps it. The master's `<v>` and
  every member's `<v>` stay as written.
- `ref` wholly inside the deletion: the group goes with its cells,
  as today (P2 ruling 1 of `specs/0018`).
- A moved end past the grid: refused as today.

A member whose master the pass has not met, in a group the pass
would unshare, cannot be written: the pass records every member's
`si` met before a master and refuses the shift with "a shared
formula member precedes its master" when such a master is then
unshared. A member whose master is never met is written as it is
(the file's fault, not the shift's; nothing is changed). Array and
data table formulas take `CUTS_FORMULA` as today. The crate's
definition stands: the shift never fixes up references; the
translation is the shared formula's own meaning made explicit, as
Excel displays it, and the text so written is then moved with its
cell like any formula.

### The translator

`translated_formula(text, dr, dc) -> Result<String, Error>`, a
`rowcol` function, shifts each relative A1 reference in a formula's
text by `dr` rows and `dc` columns (either may be negative) and
leaves everything else as written:

- A reference is an optional `$`, one to three letters, an optional
  `$` and one or more digits (a cell), a cell `:` a cell (an area),
  one to three letters `:` one to three letters with their `$`
  (whole columns), or digits `:` digits with their `$` (whole
  rows). Letters match ignoring case and are written back as
  given. A `$` component stays; a relative component moves. A
  column past `XFD` or a row past 1048576 in the input is not a
  reference.
- Not a reference: a token immediately preceded by a letter, digit,
  `_`, `.` or `$` beyond its own (a name's tail, a sheet's tail), a
  token immediately followed by `(` (a function, as `LOG10(`), by a
  letter, digit, `_` or `.` (a name), or by `!` (a sheet name).
- Skipped whole: a string literal from `"` to the next `"` with
  `""` an escaped quote; a quoted sheet name from `'` to the next
  `'` with `''` escaped, and the `!` after it; a structured
  reference's bracket text from `[` to its matching `]`, nested
  brackets counted; an error literal `#` to the next `!` or `?`
  (`#REF!`, `#N/A` ends at `A`: `#` to the end of the letters and
  `/`). The reference after a sheet name (`Sheet!A1`, `'x'!A1:B2`,
  `S1:S3!A1`) is translated like any other.
- A moved component below 1, or a column past `XFD` or a row past
  1048576, refuses with "an unshared formula would leave the
  grid". The output is the input's bytes with each moved reference
  rewritten in place, so a formula with no relative reference is
  returned unchanged.

The translator is tested alone (`shift.rs` or a new `translate.rs`
under `package/tests/`) on: a cell, an area, mixed `$`, whole
columns and rows, a function whose name reads as a reference
(`LOG10(`, `ATAN2(`), a name with a reference-like tail
(`Sales_A1`), a string holding a reference, a quoted sheet with
`''` and `!`, an unquoted sheet, a 3-D sheet span, a structured
reference with nested brackets, `#REF!` and `#N/A`, lowercase
letters, negative offsets, and the two grid refusals.

### Threaded comments (`comments`)

The sheet-level check in `record_comments` is replaced by a
per-cell one. For an edited part whose relationships hold a
`THREADED_COMMENTS` relationship, the pass reads each such target
(a part this write adds has none): the root `ThreadedComments` in
the namespace `http://schemas.microsoft.com/office/spreadsheetml/`
followed by `2018/threadedcomments` (one string, split here for
width), each child `threadedComment`'s `ref` parsed as a cell (an area
`ref` is read as its first cell; a `ref` that does not parse
refuses). A root of another name or namespace, a child without
`ref`, or a part that cannot be read refuses with "a threaded
comments part cannot be read". An addition, rich addition, box or
removal on a cell the set holds is refused with "a cell has a
threaded comment"; every other cell is edited as on any sheet, the
legacy comments part and the VML rewritten as today with the
fallback comments and their `tc=` authors kept as written. The
threaded parts, the persons part and their relationships are never
written. `record_copies` keeps `THREADED_COMMENTS_HELD` for a
source with the relationship (decision 2 A). `Package::comments`
is unchanged.

Tests (`comments.rs`): an addition, a rich addition (with
`richtext`), a box and a removal on a cell no threaded comment
names, on a built fixture holding a threaded part with one comment
at another cell, a legacy part with its fallback and a VML part:
the legacy part gains or loses the comment, the fallback and its
author stay byte for byte, the threaded part, the persons part and
the three relationships are untouched; the same four calls on the
threaded cell refused with the new message and nothing recorded; a
root of another name, a child without `ref`, an unparsed `ref` and
a missing target each refused with the read message; a sheet with
the relationship and no legacy part (the addition creates one, as
on a sheet with none); the copy of such a sheet still refused.

### The pivot cache source (the core)

`remove_sheet`, after `LAST_VISIBLE_SHEET` and before the first
record, reads each `PIVOT_CACHE_DEFINITION` relationship of the
workbook part, the target's root `pivotCacheDefinition`, and its
`<cacheSource type="worksheet">`'s `<worksheetSource>`. The sheet
is refused with "a pivot cache sources the sheet" when:

- `sheet` equals the removed sheet's name (unescaped, byte
  equality); or
- (decision 1 A) `name` equals the `name` or `displayName` of a
  table part the walk's R reaches from the sheet; or `name` equals
  a workbook `<definedName name>` whose text, after an optional
  `=`, starts with the sheet's name quoted (`'...'!`, `''` an
  escaped quote) or bare (`name!`) followed by `!`. Under 1 B,
  `sheet` alone.

A cache of another `type`, one whose `worksheetSource` has an
`r:id` (an external workbook) and neither `sheet` nor a matching
`name`, a target that is not there, or a `<pivotCaches>` the
workbook lacks contribute nothing; a cache part that cannot be
read, or whose root is not `pivotCacheDefinition` in the main
namespace, refuses with "a pivot cache cannot be read". Each read
is a streaming pass of the kind `part_ref` makes; the `read`
feature's reader is not linked into the core. A removed sheet that
holds the pivot table while another sheet holds the source stays
allowed, as today. A sheet this write adds has no part a cache
could name and is not checked.

Tests (`tests.rs`'s remove section or a new `pivot_remove.rs`): a
fixture whose cache names the sheet by `sheet` (refused, nothing
recorded, the next call on the package works), by a table's
`displayName` (refused), by a defined name qualified with the
sheet quoted and bare (refused), by a defined name qualified with
another sheet (allowed), with `r:id` and no `sheet` (allowed), of
`type="external"` (allowed), a workbook with no `<pivotCaches>`
(allowed), the pivot table's own sheet removed with the source
elsewhere (allowed, the cache kept), and an unreadable cache part
(the read message).

### Errors

New messages, each added to the crate docs' list with its tests:
"a shared formula member precedes its master" and "an unshared
formula would leave the grid" (`rowcol`); "a cell has a threaded
comment" and "a threaded comments part cannot be read"
(`comments`); "a pivot cache sources the sheet" and "a pivot cache
cannot be read" (the core). Changed: "an insertion or deletion
cuts a shared, array or data table formula" is raised for array
and data table formulas and for a shared group's moved end past
the grid; its docs entry and `specs/0018`'s rule text are not
rewritten (this spec records the change). "a worksheet has
threaded comments" is raised by `copy_sheet`'s write alone; its
docs entry says so.

### The harness (`emend-copy`)

`remove_sample` treats "a pivot cache sources the sheet" as it
treats the two "last" refusals (decision 3 A): nothing removed,
the output written, the removed count 0, the summary line naming
the refusal class (`removed_reason` = `last`, `last_visible`,
`pivot_source` or `-`). Under 3 B the refusal is the sample's
outcome, exit 2, as today for any other refusal. The shift,
comment, rich comment, box and copy samples are unchanged; their
read-backs already check what the fixes change (cell counts, the
comment counts, the legacy part). `scripts/lo-check.sh` is
unchanged.

### The probes

`probe-emend-rowcol-write-xml` and `probe-emend-comments-write-xml`
keep their sources; the new code is linked through the calls they
already make. No new probe. The record's method is Milestone 7
part 2's.

### The documents

README: the `rowcol` and `comments` lines of the feature list cite
this part's record's figures, in P4's first commit, after the
record exists (ruling H). CHANGELOG, under "## Unreleased":
"### Added", the unshare and the per-cell threaded comments rule
(each as the crate's definition words it); "### Changed", the
`remove_sheet` refusal; the "### Changed" bullet of part 1 stands.
The 0.1.0 entry is untouched.

## Harness runs

P1: the four row and column samples over the 39 workbooks, every
output opened, compared per id with part 1 P2's four records:
delete-rows 90a5b01f9541 and delete-columns 91d2bdffa46a each
error 2 - ok to ok 0 ok ok, nothing else changed. P2: the comment,
richcomment, commentbox and copy samples over the 39: the first
three on 15205770ae5c error 2 - ok to ok 0 ok ok, copy unchanged
(decision 2 A), nothing else changed. P3: the remove sample over
the 39: 495c7b79b183 ok 0 fail ok to ok 0 ok ok with the removed
count 0 and `removed_reason` `pivot_source` (decision 3 A; under 3
B error 2 - ok), nothing else changed; the size record. P4: the
passthrough and the 58 samples over the 39, every output opened,
compared per id with part 1 P2's 59 records: the seven changes
above and nothing else. Each run's expected changes are listed in
its generator as the comparison's FACTS, a deviation a FINDING.

## Phases

Each phase as `specs/0001` "Rules for every phase": a doc commit
first, the code commits, one record commit, the scan last, at most
two independent checks, the report under `<phase>`, then stop.
Target root `<home>/.cache/emend-build/m8p2-pN/target`. The gate,
run and scan generators are part 1 P2's with the part's names, as
numbered changes; P1's gate prunes part 1's P1, P2 and close-out
build targets by `rm -r` with the control (`specs/0044` ruling AD).
A dry driver sets every build root and TMPDIR under the dry run's
own directory (`specs/0043` ruling Q, `specs/0047` ruling A); every
dry and logged run report lists every FINDING line (`specs/0046`
ruling R); dry runs' build roots are kept for the maintainer's
deletion list. The dry corpus is part 1 P2's generator's fictional
files, extended with a shared formula group the point cuts (rows
and columns), a threaded comments part with its fallback, and a
pivot cache sourcing the last sheet. Each phase's code commits
carry "the diff alone carries what it changes"; the doc commit
names each file as "<path> sha256 <hash>" with no count sentence.

### P1: the unshare

The doc commit (this spec, the ROADMAP's part 2 block with P1 IN
PROGRESS, `specs/0047`'s close-out verification and the ROADMAP
rows part 1 left uncommitted). The translator with its tests; the
shift pass's unshare with tests in `shift.rs` (a cut group
unshared: the master and the members with their texts, `<v>`
kept, other attributes kept; a grown group; a deleted master with
members kept; a group moved whole stays shared; a group inside the
deletion gone; array and data table still refused; a member before
its master refused; a translation off the grid refused; the column
axis for each); the crate docs' error list. The four samples over
the 39 (above); the rowcol probe's `size` figure in the run log.
One independent check. Stop after the doc commit and the code
commits with the dry run, for planning's word on the diffs; then
the logged run.

### P2: threaded comments

The threaded part reader and the per-cell rule with the tests
above; `record_copies` unchanged; the docs. The four samples over
the 39; the comments probe's figure. One independent check. Stop
as P1.

### P3: the pivot cache source, the size record, the documents

The refusal with its tests; `remove_sample`'s `removed_reason`
with its test (`tools/emend-copy/tests/`); the remove sample over
the 39; the size record over the twenty-nine probes with the
movements attributed; the README figures and the CHANGELOG
entries. A budget at or above the ceiling stops P3 as "Size" says.
One independent check. Stop as P1. Amendment 1 (decision 4 A): the
stop at 6b13c52 is followed by one fix commit (ruling F), then the
documents commit, the generators and the dry run as before.

### P4: the reruns

The passthrough and the 58 samples over the 39 with every output
opened, the records `records/m8p2-<name>-<utc-date>-<shortsha>`,
compared per id with part 1 P2's; the logs README. Two independent
checks (the records against the run, and the report). No code
unless a finding is fixed, with the fix named and the affected
samples rerun as part 1 P2 did. The first commit after the doc
commit is the README's two feature figures citing P3's size record
(ruling H), with the CHANGELOG clause of ruling K (4) and the crate
docs' sentence of ruling M (1).

### Close-out push (after P4)

As part 1's (`specs/0047`, "Close-out push (after P2)") with these
differences: the base is part 1's close-out doc commit e1fc6e4 by
its subject ("M8 part 1 close-out: commit planning docs (the P2
verification and rulings)"); `PREV_RECORD_PATTERN` is `^M8 part 1
close-out record` (454dea3), in the range exactly once; the
close-out doc commit's subject is "M8 part 2 close-out: commit
planning docs (the P4 verification and rulings)";
`CARGO_TARGET_DIR` `<home>/.cache/emend-build/m8p2-close/target`;
the CHANGELOG check reads the "Unreleased" heading with its
"Added" and "Changed" sections above the dated 0.1.0 heading and
the 0.1.0 entry unchanged in the range; the two Excel-record
checks as part 1's; the size record `records/size-m8p2-*.md` read
exactly once in the range; no tag; `git push origin main` alone
after the maintainer's typed authorisation in the implementing
session; the post scan over the range. The record commit "M8 part
2 close-out record: the push, the CI run and the scan" holds the
three logs only. One independent check. Part 1's record 454dea3
goes with this push.

## Planning rulings during P1 (2026-11-09)

A. The stop report verified from git, the diffs, the tests and the
   dry logs, and the word for the logged gate and run. The doc
   commit 02ba09d names the three files at the hashes planning
   printed; the code commit 2740ce7 (diff 6564082e) does what "The
   unshare" and "The translator" say: `shifted_formula` returns
   `Unshare` for a shared group the rule grows or cuts or whose
   master's row is deleted with the `ref` not wholly inside the
   deletion; the pass holds each unshared master's input cell and
   unescaped text and writes each later member's `<f>` with the
   translated text (its own text kept when it has one), `t`, `ref`
   and `si` removed and every other attribute kept; a member met
   before its master in a group then unshared is refused; array and
   data table formulas take the cuts message; the translator's
   fifteen cases cover the spec's list. Rulings on the report's
   points: (1) the size FINDING (the `rowcol` cost 76888 against
   58244, a growth of 18644 over the accepted 16000) stands as
   recorded, not fixed, as "Size" says; the band was planning's
   estimate and is not moved; the logged run will print the same
   FINDING and that is not a rerun; P3's size record attributes the
   growth by file. (2) One code commit for the translator, the
   unshare and the docs list is accepted for the reasons given (the
   translator alone is dead code; the catalogue test wants each
   message listed where it is raised). (3) The three renamed tests
   are accepted: the straddling cases moved to the new unshare
   tests, and emend-copy's two refusal tests use array formulas,
   which stay refused. (4) A byte past ASCII read as a name
   character is accepted; the spec's "letter" includes it. (5) The
   cuts entry's text kept, its test list changed: as "Errors" says.
   (6) The forks' calls (labels W*, no passthrough run, no rerun
   note, a growth below 3000 a FINDING too) are accepted; the gate
   and scan admit specs/0048 and `ROADMAP.md` as planning's
   uncommitted files and planning edits neither `specs/0047` nor
   anything else before the record commit. (7) The logged gate's
   prune of part 1's P2 and close-out build targets is this spec's
   "Phases" rule (`specs/0044` ruling AD). Noted, not a change: a
   mixed-case column (`aB1`) is written back in upper case; a
   lowercase one in lower case; the spec's "as given" reads so.

B. The record commit cbef379 verified (2026-11-09) from git, the
   logs and the README: parent 2740ce7, the subject as the brief
   gave it, the diff-alone sentence, the Co-Authored-By line only,
   ASCII; five files under `records/logs/m8p2-p1/` (gate.log
   ba3e318f, run.log 194f6dd4, scan.log 26261cca, README.md
   492430dc, independent-check-1.md 9e1e37a9) and nothing else.
   gate.log ends yes with the seven sets at their counts on both
   targets, shift 44 and translate 15 by name, and the prune of
   part 1's P2 and close-out targets by the `rm -r` form with its
   control. run.log ends yes: the four samples over the 39, every
   output opened, each compared per id with part 1 P2's record;
   delete-rows 90a5b01f9541 and delete-columns 91d2bdffa46a from
   error/-/ok to ok/ok/ok, every other row the same, new 0 and
   missing 0 in each sample; the one FINDING line, the `rowcol`
   growth 18644 (1061616 against the format probe's 984728, the
   cost 76888 against 58244), as ruling A (1) ruled. The scan PASS,
   "structural checks failed: 0", run before the record commit as
   part 1 P2's was. No log holds a byte outside ASCII, an address
   or a home path (the two "/home/" strings in run.log and the one
   in scan.log are the scripts' own patterns). Check 1 answers yes
   with exceptions, yes with one precision point, and names README
   figures without a committed primary; each is disposed in the
   README's "After independent check 1" and planning rules them
   so: (1) the three test gaps (a data table formula cut on the
   column axis; a column-axis member with an attribute or text of
   its own; a shared group's end pushed past the grid by an
   insertion at or before its master, refused with the cuts
   message) are tests P2 adds in a test-only commit of its own
   before its threaded-comments commits, the gate's `all` count
   moved by them; no code changes for them. (2) A row written with
   a leading zero is not a reference: accepted, Excel writes none;
   a member in a deleted row before its master counted as early:
   accepted, a layout Excel does not write. (3) The per-id
   comparison reads outcome, lo and lo_in and not the exit code,
   which the record rows print: accepted for P1 to P4; "error 2 -
   ok" in this spec names the four columns for the reader. (4) No
   scan after the record commit, as part 1 P2. The README's
   figures from files under `<phase>` (the mktemp name, the
   driver's start time, the outer stderr sizes) are named with
   their sources in its after-check section, as ruling C of
   `specs/0047` allows; its line-range and wording slips are
   corrected there and the README stands as committed. P1 is DONE
   at cbef379.

## Planning verification of P1 (2026-11-09)

Verified from primaries, never from the implementing session's
summaries alone: the doc and code commits' diffs against "The
unshare", "The translator" and "Errors"; the translator's module
and its fifteen cases read by planning; the unshare tests' fixtures
and expected outputs read by planning (the member's text is the
master's read from the member's input cell, then moved with its
cell, as the crate's definition says); dry run d1 (ruling A); the
logged gate and run, the scan, check 1 and the record commit
(ruling B). The two corpus refusals of group 3 are gone: both
samples now write an output that opens, and no other row of the
four samples over the 39 moved. The `rowcol` cost grew 18644 over
the band's accepted 16000, recorded and not fixed; P3's size record
attributes it by file. P1 is DONE at cbef379; the working spec and
the ROADMAP stay uncommitted for P2's doc commit.

## Planning rulings during P2 (2026-11-09)

C. The stop report verified from git, the diffs, the tests and the
   dry logs, and the word for the logged gate and run. The doc
   commit 94fc5a4 names the two files at the hashes planning
   printed; the test-only commit 8586040 adds the three tests of
   ruling B (1) and no code; the code commit c609bf3 does what
   "Threaded comments" says: `threaded_cells` reads each
   threadedComment relationship's target by a streaming pass
   (`threaded_refs`: the root `ThreadedComments` in the 2018
   namespace, each child's `ref` an area read as its first cell),
   `record_comments` refuses an addition, removal or replacement
   on a named cell with "a cell has a threaded comment" and edits
   every other cell as before, the threaded parts never written;
   "a threaded comments part cannot be read" covers another root,
   a child without `ref`, an unparsed `ref`, an external or missing
   target and an XML error; `record_copies` keeps the sheet-level
   refusal. The five tests cover the spec's list on a fixture
   holding a threaded part, a persons part, a legacy part with the
   `tc=` fallback and a plain comment, and a VML part, asserting
   the fallback, the threaded part and the persons part byte for
   byte. Dry d1: the gate and run yes, the three pairs met and the
   copy pair kept, the scan FAIL on the four dry-layout lines
   alone, no FINDING; the `comments` cost 66576 against 63440, a
   growth of 3136, above the expected 3000 and within the accepted
   5000, no FINDING. Rulings on the report's points: (1) the growth
   is accepted; P3's record attributes it. (2) The sheet-level test
   removed, its case replaced by the five: accepted. (3) The
   by-name comments run of 60 with `comments,richtext,rowcol`:
   accepted; the filter takes the submodules. (4)
   `THREADED_COMMENTS_HELD` gated on `comments` and `copy`
   together: accepted, the copy is its only raiser. (5) The copy
   pair as an explicit "kept" class with a control: accepted. (6)
   The test-only commit classed as a code commit: accepted; no
   further change. (7) Planning edits specs/0048 alone before the
   record commit. (8) The prune of P1's target is the spec's rule.
   Noted, not a change: one doc-comment line of `add_comment` runs
   long after the edit; P3 may rewrap it in a docs commit if it
   touches that comment, otherwise it stands.

D. The record commit 572e71f verified (2026-11-09) from git, the
   logs and the README: parent c609bf3, the subject as the brief
   gave it, the diff-alone sentence, the Co-Authored-By line only,
   ASCII; five files under `records/logs/m8p2-p2/` (gate.log
   d0027553, run.log 0646a0c7, scan.log 1779a159, README.md
   4cc1be00, independent-check-1.md d9bbb6e6) and nothing else.
   gate.log ends yes with the seven sets at their counts on both
   targets (cc 762, all 1236), shift 47 and comments 60 by name,
   and the prune of P1's target by the `rm -r` form with its
   control. run.log ends yes: the four samples over the 39, every
   output opened, compared per id with part 1 P2's records; on
   15205770ae5c comment, richcomment and commentbox from error/-/ok
   to ok/ok/ok and copy kept error/-/ok, every other row the same,
   new 0 and missing 0; no FINDING; the `comments` cost 66576
   against 63440, the growth 3136 accepted. The scan PASS before
   the record commit. No log holds a byte outside ASCII, an address
   or a home path (the scripts' own patterns apart). Check 1
   answers yes to the diff and the logs with coverage notes and
   finds the README's figures in primaries or named with their
   files; each note is disposed in the README's after-check section
   and planning rules them so: (1) the threaded part's external
   target refusal and the XML-error mapping get tests in P3's
   test-only commit; the read message covers an XML error alone,
   and a read failure of the archive (`Error::Io`) passes through
   as raised, which is right: it is not a malformed part. (2) A box
   alone on a threaded cell meets "no comment addition on that
   cell" first, as any lone box does; the spec's box refusal is met
   through the addition: accepted as is. P2 is DONE at 572e71f.

## Planning verification of P2 (2026-11-09)

Verified from primaries, never from the implementing session's
summaries alone: the three commits' diffs against "Threaded
comments", "Errors" and ruling B (1); the reader and the per-cell
check read by planning; the five tests' fixture and assertions
(the fallback, the threaded part and the persons part byte for
byte); dry run d1 (ruling C); the logged gate and run, the scan,
check 1 and the record commit (ruling D). Group 4's three corpus
refusals are gone: the comment, rich comment and box samples write
outputs that open on the threaded-comments workbook, and the copy
stays refused as decision 2 A rules. No other row of the four
samples over the 39 moved. The `comments` cost grew 3136, within
the accepted band. P2 is DONE at 572e71f; the working spec and the
ROADMAP stay uncommitted for P3's doc commit.

## Planning rulings during P3 (2026-11-09)

E. The ceiling stop verified (2026-11-09) from the report, the four
   commits and the diff of 5ab0e96, and the measurement ruled. The
   doc commit a1a0ae0 at the two hashes; ea6dc9f the two tests of
   ruling D (1), no code; 5ab0e96 the refusal as "The pivot cache
   source" says under decision 1 A (one workbook pass for the
   `<pivotCache>` ids and the defined names, one pass per cache
   part, one per table root, `normalized_value` for each name,
   two `format!` prefixes; `TABLE` and `PIVOT_CACHE_DEFINITION`
   made unconditional), with `pivot_remove.rs`; 6b13c52 the
   sample's reason class, emend-copy alone. The format probe at
   6b13c52 is 994200 section bytes against 984728 at c609bf3 and
   at the record, so the whole growth of 9472 is 5ab0e96's; the
   budget 460984 is 2232 above the ceiling 458752 and "Size" stops
   the phase before the record commit, as it did. Planning's word
   was to measure, not to change: in a local clone at 6b13c52
   under `<phase>/measure`, nothing committed, four builds of the
   format and zip probes (zip 533216 in each): A as committed
   460984; B (the `sheet` attribute alone, patch 5b5b783d) 456200,
   2552 under the ceiling, growth 4688; A-slim (bytes for strings,
   no `format!`, patch 44879689) 460876; B-slim (patch 393713aa)
   456320. The crate has no unescape helper of its own, so
   `normalized_value` stays in every variant. The cost of decision
   1 A over B, 4784, is the defined-name pass with `text_of` and
   the table root read. The four commits stand as history; the
   generators and the dry run wait on the amendment's decision.
   Planning's band for the core growth (expected 300 to 2500,
   accepted 4000) was an estimate from line counts, not from a
   measured pass of like shape; the lesson is in planning's
   memory and in "Amendment 1" below.

F. The fix under Amendment 1, decision 4 A (ruled 2026-11-09, after
   the maintainer's "i agree with recs"). One code commit on top of
   6b13c52, its message naming this amendment and the stop ("the
   budget above the ceiling at 6b13c52, ruling E") and ending with
   the diff-alone sentence. The change: `pivot_sourced` keeps the
   `sheet` attribute case in the core; the defined-name case (the
   `<definedNames>` read of `workbook_pivot_caches`, the two
   prefixes, the match) is a block under `cfg(feature =
   "metadata")`, and the table case (the sheet's `table`
   relationships, `root_names`) a block under `cfg(feature =
   "tables")`; a cache source's `name` is read only under either
   feature; `TABLE`'s `cfg` returns as it stood at c609bf3 (it
   already names `tables`); `PIVOT_CACHE_DEFINITION` stays
   unconditional, the core reads it. The measurement's `sheet`-only
   patch (ruling E, 5b5b783d) is the reference for the core's shape,
   written again in the repository's tree, nothing applied from the
   clone. The tests of the two name cases in `pivot_remove.rs` run
   under their features in the form the crate's feature tests use;
   the `sheet` tests and the two error tests stay unconditional; the
   gate's test run covers both features as it covers the existing
   feature tests. The crate docs' entry for "a pivot cache sources
   the sheet" names the three cases and the two features; the
   documents commit's README removal sentence and CHANGELOG entry
   say the same. The core probe measured 456200 in the clone for
   this shape; the figure that counts is the size record's, its
   growth over 451512 a FINDING against the unmoved band (4688 if
   the record repeats the clone), attributed by file; each of the
   `tables` and `metadata` probes is judged against the 500 to 3000
   band, accepted 5000, as a feature cost (ruling G). Then the
   documents commit, the generators (P2's as numbered changes, the
   target `<phase>/target`, the prune of P2's target), the dry run
   (the dry corpus extended with pivot-sourcing fixtures of the
   three kinds) and the stop report, as "P3" says. `ROADMAP.md` is
   not edited before the logged run (the run admits this spec
   alone); its P3 row is written at P4's doc commit.

G. The fix commit verified and the feature bands read
   (2026-11-09). c076900 (amended per ruling I to 37e800f) is
   ruling F's change: the `sheet` case in
   the core, the `wanted` names and the two name cases in blocks
   under `any(tables, metadata)`, `metadata` and `tables`, the two
   pivot structs with `cfg` fields, `TABLE`'s `cfg` as at c609bf3,
   `PIVOT_CACHE_DEFINITION` unconditional, the table test and its
   helper under `tables` and the defined-name test under
   `metadata`, the crate docs' entry naming the three cases; two
   files, no other change. The dev size check at the fix: format
   989480 over zip 533216, a budget of 456264, 2488 under the
   ceiling, growth 4752 over 451512 (the clone's 456200 plus the
   kept parameters). "Size" names a feature's cost as its probe
   over the format probe (`rowcol` 58244, `comments` 63440 are such
   figures), so Amendment 1's band for `tables` and `metadata` is
   read the same way: the cost at Milestone 7 part 2's record is
   46212 for `tables` and 49764 for `metadata`; at the fix 48592
   and 52032, growths of 2380 and 2268, inside 500 to 3000. The
   raw probe growths (7132 and 7020) are the core's growth plus
   those costs; "Size"'s "every other probe" band does not apply
   to these two, the amendment's does. The size record judges each
   feature-linking probe this way and prints both figures.
H. The README's feature figures and the record's name
   (2026-11-09). The size record is named for the phase head the
   probes are built at, as `size-m7p2-2026-10-28-43471ed.md` was
   named for part 2 P1's documents commit; its UTC date is the
   logged run's. A documents commit therefore cannot cite its own
   record. As M6 part 6 P4 (37062b7) and M8 part 1 P2 (67310df)
   cited a record from a later phase, the README's `rowcol` and
   `comments` figures are P4's first commit after P4's doc commit,
   on the code path, docs only, no CHANGELOG line (the figures cite
   a record as the budget sentence does). P3's documents commit
   carries the CHANGELOG entries and the README's removal sentence
   alone; the record commit is unchanged in shape. "The documents"
   and "P4" say so.
I. The fix commit's doc defect (2026-11-09). At c076900 the doc of
   `remove_sheet` still links [`TABLE`], which the commit put back
   under its `cfg`, so the crate's doc build with `-D warnings`
   fails with no features and with `metadata` alone; the same
   paragraph still reads all three cases as unconditional. The
   implementing session's doc check ran under `--all-features`
   alone, which hides it (its gotcha). The repair is the doc
   paragraph alone: the `sheet` case, then each name case under
   its Cargo feature, no link to `TABLE`, the paragraph reflowed to
   the file's doc width. c076900 is HEAD, unpushed, and cited by no
   log and no committed file (ruling G alone names it, uncommitted),
   so it is amended, as P2's doc commit 122696b was amended to
   4296d03 before anything referenced it (its stop report, point
   5), and ruling F's one fix commit holds. The amended message adds
   one sentence naming the doc paragraph; the new hash replaces
   c076900 in ruling G when the stop report prints it; c076900 is
   local and unreferenced. The stop report lists the doc build's
   feature sets with their exit codes.
J. The size record's build (2026-11-09). As Milestone 7 part 2
   P1's `size.log` was, the record is written by its own logged
   script: a fourth generator from M7 part 2's printed size script
   with this part's names as numbered changes, writing
   `records/logs/m8p2-p3/size.log` and the record (the probes at
   the phase head, each rebuilt at 43471ed by the base record's
   log, the attribution by file, ruling G's two figures for each
   feature-linking probe, the record written then installed); the
   run log stays the remove sample and cites `size.log` by hash;
   the scan reads four logs; the record commit carries the size
   log with the three others. The size record is named for the
   phase head the size script builds at (ruling H). The run log and
   the size log each read `PLANNING_SPEC` at run time.
K. The stop verified and the word given (2026-11-09). From the
   commits: 37e800f is c076900's code diff with the doc paragraph
   repaired (one file, 16 lines between them, no code), its message
   with ruling I's sentence; 8065795 the CHANGELOG's three bullets
   and the README's removal sentence as "The documents" says under
   ruling H, no office literal, no U+2014. From dry d2 (gate
   59a04a10, size 733c8028, run e64292a8, scan f1536045): the
   three logs end "checks all as expected: yes"; the scan FAILs on
   the four dry-layout structural checks alone, the a1a0ae0
   diff-alone line information; the remove pair 495c7b79b183
   ok/fail/ok to ok/ok/ok met with removed 0 and `pivot_source`,
   the two dry-only fixtures reading `pivot_source` printed and not
   judged; the clippy and doc sets include the crate with no
   feature and with `tables` and `metadata` alone; the kept names
   26, P2's target listed. The size figures as the report says:
   budget 456264, 2488 under; core 4752; costs rowcol 18628,
   comments 2948, tables 2380, metadata 2268; 11 FINDING lines, all
   in `size.log`. The report's points:
   (1) richcomment, printnames and copycomments stay FINDINGs:
   "Size" puts them in the "every other probe" band and a band is
   not moved after the figure is read; the logs README's findings
   list states, for each, the core's 4752 and the judged cost
   growths it carries, from the size log's 5a to 5f figures, so the
   reader sees the cause; no generator change. (2) The six shrinks
   stay FINDINGs, as "Size" says and M7 part 2's record did. (3)
   The attribution at file granularity naming the five commits is
   part 7's method; the record's prose for the core names 5ab0e96
   and 37e800f as the cause, which suffices. (4)
   `PIVOT_CACHE_DEFINITION` is `pub` in `pub mod package`, so it is
   a public item without `read` since 5ab0e96; it stays so (the
   relationship-type constants are `pub` by the crate's convention
   and the core reads it); the CHANGELOG's "Changed" bullet gains
   one clause saying so in P4's docs-only commit (ruling H), no
   change to P3. (5) The gate counting no emend-copy tests per set
   follows P2; check.sh's workspace run covers them. (6) The scan
   not checking the record commit's subject follows P2; planning
   reads the subject. (7) The doc-set slip is in both sessions'
   gotchas. The word: the logged driver in the order gate, size,
   run, scan with PRUNE_PREVIOUS=yes over P2's target alone and its
   control; the record named for the phase head 8065795 with the
   run's UTC date; the four logs under `records/logs/m8p2-p3/`, the
   README listing every FINDING line of the four logs, one
   independent check given the spec, the range diff, the draft
   report and the four logs; the record commit "M8 part 2 P3
   record: the size record, the logs, the README, the check"; the
   final scan; the report; STOP. Planning does not edit this spec
   while the driver runs.

L. The interrupted logged run (given by message 2026-11-10, written
   here after the record commit because the gate log pins this
   spec at 37a8987d). The host's disk filled on 2026-11-09 and the
   machine stopped during the size step: the gate log was complete
   ("yes", 3528ea39, P2's target pruned), the size log cut
   mid-line at 10710 lines (6339d013), the run and the scan not
   started. An environment failure, not a failed check: the gate
   log stands and is not rerun, its six binaries checked against
   its hashes first; attempt 1's files are moved, not deleted, to
   `<phase>/superseded/size-attempt-1`; the size step runs again
   from its start as attempt 2 with the cause in its rerun note,
   then the run and the scan; the README has a section on the
   interruption with both hashes; a build failing on an artifact
   the stop corrupted is a stop and a report. Then as ruling K.
M. The points of P3's final report (2026-11-10). (1) The crate
   docs' sentence that `TABLE` is public with no feature is false
   since 37e800f; it, the CHANGELOG clause of ruling K (4) and the
   README figures of ruling H are P4's first commit, documents and
   that one doc comment, no compiled code. (2) The scan run after
   the README, as P2's, is the phases' form; rulings K and L named
   the order of the logs, not a driver step. (3) The README's
   after-check section corrects its lead sentence on the three
   raw-growth probes; it stands as committed. (4) The size log's
   "P1" at line 95 is a generator carry, the same in dry d2; not
   regenerated. (5) and (6) are answered in the README's
   after-check section; nothing follows.

## Planning verification of P3 (2026-11-10)

From the record commit 053d27c (seven files, nothing else) and its
logs: gate.log 3528ea39, size.log f65e0cef (attempt 2), run.log
3b72e757, each ending "checks all as expected: yes" with
`PLANNING_SPEC` 37a8987d; scan.log 23543045, structural failures
0, RESULT PASS; the size record
`records/size-m8p2-2026-11-10-8065795.md` (b4a386c3). The budget
is 456264, under the ceiling 458752 by 2488. Eleven FINDING lines,
all in the size log, recorded and not fixed: the core's growth
4752 over the accepted 4000; the `rowcol` cost's growth 18628 over
the accepted 16000; six probes shrinking 16 or 32; richcomment
7556, printnames 7656 and copycomments 7700 over 6752. The
`tables` and `metadata` costs grew 2380 and 2268, inside their
band. The remove sample over the 39: same 38, changed 1, the pair
495c7b79b183 ok/fail/ok to ok/ok/ok met with its line, no other id
reading `pivot_source`, no FINDING. Check 1's answer is in the
record verbatim with its dispositions in the README. P3 is DONE;
nothing is pushed.

## Planning rulings during P4 (2026-11-10)

N. The stop verified and the word given (2026-11-10). The doc
   commit 3bbac34 at the two hashes; f625b99 the README's two
   figures (76872 and 66388, the record's), the CHANGELOG clause
   and the crate docs' sentence, doc comment lines alone in the
   crate. Dry d1: the gate and run logs end "yes", 59 of 59 runs,
   the scan FAILs on the four dry-layout checks alone. Its 8
   FINDING lines are one cause: the dry workbooks' worksheet roots
   declare no `xmlns:r`; adding a chart declares it and removing
   the chart leaves the declaration, so the sheet part does not
   return to its input bytes and the remove-chart sample reads
   error. (1) "Seven changes" is the six measured changes and the
   copy pair kept; the table holds that. (2) The dry FINDINGs are
   accepted as a dry cut, no second dry run: the 39 real workbooks
   declare the prefix (part 1 P2's remove-chart records read ok on
   all 39) and the logged run judges them. The residue itself, a
   namespace declaration an addition wrote left after its removal
   on a root that had none, is a finding of this part: recorded in
   the logs README and at the close-out in `ROADMAP.md` under
   "Unscoped", not fixed here (a core change). (3) The run
   summaries' two counters carried past the remove sample are left
   as generated, the totals being right; the README says so. (4)
   and (5) stand. (6) The README's budget sentence cites the figure
   at the 0.1.0 tag by its own words; it is updated with the
   release, part 3, not here. (7) An `rm -rf` typed on a path that
   did not exist removed nothing; the rule stands, the form is
   `rm -r "${var:?}"` in a logged script alone. The word: the
   logged gate with PRUNE_PREVIOUS=yes over P3's target alone, the
   run, the records, the README with every FINDING line, two
   independent checks, the record commit "M8 part 2 P4 record: the
   59 records, the logs, the README, the checks", the final scan,
   the report, STOP. Planning does not edit this spec while the
   driver runs.

## Planning verification of P4 (2026-11-10)

From the record commit 5def62f (124 files, all under `records/`)
and its logs: gate.log a27d8ee7 and run.log 4f148de2, each ending
"checks all as expected: yes" with `PLANNING_SPEC` 9a6e0db5;
scan.log 199c614f, RESULT PASS. The gate pruned P3's target and
ran the seven test sets at P3's figures on the host and on i686,
0 failed. The run: 59 of 59, 0 FINDING lines. Planning compared
the 59 record pairs itself, per id and per column but the wall
time and the peak memory, with part 1 P2's 59 of the same names:
39 ids in each, six rows changed and no other, the six of the
table (delete-rows 90a5b01f9541, delete-columns 91d2bdffa46a,
comment, richcomment and commentbox 15205770ae5c, each
error/-/ok to ok/ok/ok; remove 495c7b79b183 ok/fail/ok to
ok/ok/ok, its output now a passthrough), and copy 15205770ae5c
kept error/-/ok. The two checks' answers are in the record
verbatim; the two README sentences both found wrong (the dry
ids, which are nine real ids labelling generated workbooks, and
when ruling N was written, which was before the run) are
answered in the README's last section and stand corrected there.
The namespace residue is reproduced in the README on a generated
workbook and shows on none of the 39. A first final-scan call
without its stage wrote an empty log into the tree and exited 1;
the file was moved out, not deleted, and the scan rerun by its
usage; a slip of invocation, not a rerun of a failing check. P4
is DONE; nothing is pushed.

## Planning rulings during the close-out (2026-11-10)

O. No prune in the close-out (given by message, written after the
   push). Planning's brief asked the close-out's gate to prune
   P4's target; the close-out push script has no prune block
   (`specs/0032` P3 verification ruling 9, `specs/0031` P2
   verification ruling 6) and "Close-out push (after P4)" names
   none. The clause was a planning slip and is withdrawn. Part 3
   P1's gate prunes P4's target and the close-out's.

## Planning verification of the close-out (2026-11-10)

From the record commit f44debd (three logs, nothing else) and the
remote: the doc commit 9f862ca holds the two planning files at
the hashes of the brief; push.log b9d17939 ends "push checks all
as expected: yes", pre mode before it "yes", the maintainer's
authorisation typed in the implementing session and recorded
with its date; `git ls-remote` reads 9f862ca for the branch, no
tag on it; ci-run.log 26aa929a, run 42935203506, no step with a
conclusion other than success or skipped; scan.log 9712087f over
e1fc6e4..9f862ca, RESULT PASS. No FINDING line in the three
logs. The pushed range is 19 commits and carries part 1's record
454dea3. The independent check answers yes to its three
questions; its point on the size block's label ("the parent of"
over the adding commit, the computation right) is fixed in the
next carried script. The part is DONE and PUSHED; the record
f44debd is local.

## Amendment 1 (2026-11-09): the budget above the ceiling

The facts: the budget at 6b13c52 is 460984, 2232 above 458752;
the `sheet`-only rule measures 456200, 2552 under, a growth of
4688 over the record's 451512 and above the accepted 4000 of
"Size"; the committed rule's two extra cases (a table's name and
a defined name) cost 4784 of core bytes (ruling E). Milestone 8
decision 1 said the ceiling is reopened "only when a change to
code the core links needs it, as that spec's first decision";
this is that decision, number 4 below. Under 4 A, "Size" reads:
the core rule is the `sheet` attribute alone, its growth 4688 a
FINDING against the band (the band is not moved, as P1's was
not), recorded and attributed in the size record; the table-name
case is the Cargo feature `tables`'s and the defined-name case
the Cargo feature `metadata`'s, each a block of `pivot_sourced`
under its `cfg`, each feature's cost expected to grow 500 to 3000
and accepted up to 5000 over Milestone 7 part 2's record; the
`tables` and `metadata` tests of `pivot_remove.rs` run under their
features; the crate docs' entry for "a pivot cache sources the
sheet" names the three cases and their features; `TABLE` stays
unconditional only if the core still reads it, else its `cfg`
returns with `tables` added. The fix is one code commit after
5ab0e96 naming this amendment and the stop ("the budget above the
ceiling at 6b13c52, ruling E"), then the documents commit, the
generators and the dry run as "P3" says; the size record judges
the result. Under 4 B, "Size" reads: the ceiling is 466944 (456
KiB) from this part on, the budget 460984 under it by 5960, the
growth 9472 a FINDING against the unmoved band; the README's size
sentence and the ROADMAP's Milestone 8 decision 1 line say so; no
code changes; the generators and the dry run follow. Under 4 C,
the core rule is the `sheet` attribute alone and the two name
cases are a documented limit: the README's removal sentence says
a pivot cache built from a table or a named range on the removed
sheet is left sourcing nothing (the finding of `specs/0047`
decision 2 B for those cases); the `tables` and `metadata` tests
of the two cases are removed with the code; otherwise as 4 A.

## Decisions for the maintainer

To be typed in the planning session; the recommended option is A
in each. P1 and P2 need none of them; P3 needs 1 and 3; the copy
sample's expectation in P2 and P4 needs 2.

Decided by the maintainer 2026-11-09, typed in the planning session
("great, i agree" to planning's recommendation): 1 A, 2 A, 3 A. The
A branches above apply.

Decided by the maintainer 2026-11-09, typed in the planning session
("i agree with recs" to planning's recommendation of A): 4 A.
"Size" reads as Amendment 1's 4 A paragraph says; ruling F has the
fix.

4. The budget above the ceiling (Amendment 1; Milestone 8 decision
   1 reopened): A (recommended), the ceiling 458752 stands; the
   core refuses by the `sheet` attribute alone (456200, 2552
   under; the growth 4688 a recorded FINDING); the table-name case
   goes behind `tables` and the defined-name case behind
   `metadata`, the features that already read those parts, so a
   consumer linking them keeps the whole rule at no core cost; B,
   the ceiling is raised to 466944 (456 KiB) and the committed rule
   stands whole in the core (460984, 5960 under the new ceiling);
   C, the `sheet` attribute alone in the core and the two name
   cases a documented limit, no feature code.

1. The pivot source by `name`: A (recommended), the refusal covers
   a `sheet` attribute naming the sheet, a `name` naming a table
   part reached from the sheet, and a `name` naming a defined name
   qualified with the sheet, so a cache built from a table or a
   named range is caught; B, the `sheet` attribute alone, the
   smaller core change, a cache built from a table on the removed
   sheet left sourcing nothing as today.
2. A copy of a sheet with threaded comments: A (recommended), still
   refused with "a worksheet has threaded comments"; carrying the
   threaded part and the persons part with fresh ids is a feature
   of its own, and the copy sample on 15205770ae5c stays a recorded
   error; B, the copy carries the legacy part alone and drops the
   threaded relationship, which Excel then shows as plain notes.
3. The remove sample on a sourced sheet: A (recommended), the
   sample treats the new refusal as it treats the two "last"
   refusals, removing nothing and recording the reason, so the
   output is a passthrough and opens; B, the refusal is the
   sample's outcome, exit 2, a documented refusal in every record
   as groups 3 and 4 were.

## Out of scope

- Fixing up references on insertion or deletion; any change to
  array or data table formulas; an edit on a cell in a shared
  formula (`IN_SHARED_OR_ARRAY`).
- Writing a threaded comment, a persons part, or a reply; reading
  a threaded comment beyond the legacy fallback.
- Removing or rewriting a pivot cache, or pointing it elsewhere.
- The Excel check and the release of 0.1.1 (part 3,
  `specs/0049`).
- Any new corpus file; any change to `corpus-add.py`, the tags or
  `scripts/lo-check.sh`.
- Any fixture, test or record holding a byte of a maintainer's
  workbook, a sheet or table name of one, an original name or a
  note.
- The xlsx-diff, passthrough and privacy-gate backlogs under
  "Unscoped".
