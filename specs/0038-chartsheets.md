# specs/0038: chartsheets

Status: APPROVED 2026-10-18 (Milestone 6 part 1; decisions 1 to 3 A,
typed in the planning session 2026-10-18; drafted 2026-10-16; the
openpyxl facts confirmed on the pinned image 2026-10-18; the one fact
still marked "to confirm" needs a chartsheet written by desktop Excel,
which the corpus lacks). The P1 brief follows Milestone 5 part 6's
close-out push, DONE 2026-10-18.

Milestone 6, "chart parity and named styles", was approved by the
maintainer 2026-10-16 (`ROADMAP.md`, "Milestone 6"): every item of
`specs/0036`'s out-of-scope list a part, one part at a time; the
chart parts under the feature `charts`, off by default, measured
by the charts probe; the Milestone 2 ceiling kept for the core; the
Milestone 2 Excel check gates no push. This part is the first of
six: the chartsheet, the one sheet kind that holds a chart and no
cells, which the crate today lists (`SheetKind::Chartsheet`),
counts (`survey`), removes (`remove_sheet`), renames and moves, and
otherwise refuses. After this part a consumer reads the chart of a
chartsheet, creates a chartsheet around a chart it writes, and
replaces that chart; openpyxl's `create_chartsheet` and
`Chartsheet.add_chart`.

## Goal

A consumer opens a workbook with a chartsheet and reads its chart
through `Package::charts` as it reads a worksheet's; creates a
chartsheet with `Package::add_chartsheet`, named like a worksheet,
holding one chart written by part 4 and 5's writer from cell ranges
of any worksheet; removes that chart with `remove_chart` and adds
another with `add_chart`; renames, moves and removes a chartsheet
with the calls that take a worksheet. The output keeps every other
entry byte for byte, LibreOffice opens every corpus output and both
generated fixtures, and `charts` reads every written chartsheet
back as the consumer specified it. Nothing else about a chartsheet
is written in this part (its view, page setup, header and footer,
protection; a copy; a chartsheet with no chart; see "Out of
scope").

## Size and packaging

The budget figure after Milestone 5 part 5 is 436992 section bytes
against the ceiling of 458752
(`records/size-m5p5-2026-10-15-a8bfe71.md`): 21760 of headroom,
held for fixes; part 6 adds no code to the core.
Everything in this part goes under the feature `charts`, measured
by `probe-emend-charts-write-xml`, whose work gains an
`add_chartsheet` of the probe's column chart on its second output
and a `charts` read of the new part (the chart count printed, 1).
The `charts` feature costs 105424 section bytes after part 5; the
probe's movement over that is this part's cost, and the size record
states the feature's whole cost, part 5's figure and the delta.

Planning's estimate, priced as `specs/0034` ruling AC asks: the
blank chartsheet form and the sheet addition of kind chartsheet
(the part name, the relationship of the chartsheet kind, the
`Override`, the `<sheet>` element through `SheetEdit::Add` with a
kind) 2500 to 4000; the drawing created on the new part with its
one absolute anchor through the anchor writer `images` and
`charts` share, and the `drawing` relationship on a chartsheet rels
part 1500 to 3000; the root rule (`worksheet` or `chartsheet` for
the three chart calls, the chartsheet's child order for the
`drawing` element's position) 1000 to 2000; the checks and the
three messages 1000 to 2000; number parsers 0. In all 6000 to
11000 under `charts`. Planning is told the figure in the report at
any value; a cost above 15000 is a finding the report names. The
core's growth is expected 0 and accepted up to 2000; the `images`
and `comments` costs may move by the root rule's regating of the
helper they share (expected 0, accepted up to 1500 each): the
report prints and judges each movement.

## Facts planning checked (2026-10-16)

- The corpus holds no chartsheet and no dialogsheet: 0
  `chartsheet+xml` and 0 `dialogsheet+xml` content types over the
  27 workbooks, counted by planning with Python's `zipfile` (counts
  only; `specs/0034` "Facts" counted 0 and `specs/0033` lists the
  kinds). So this part is proven on built packages and on two
  generated fixtures, as `specs/0033` P3 proved external links: one
  in Excel's form written by the run log's own generator, one in
  openpyxl's form written by the pinned openpyxl image of
  `specs/0037` (its op list grows by `chartsheet`).
- The schema (ECMA-376 part 1, `CT_Chartsheet`): children in order
  `sheetPr`, `sheetViews`, `sheetProtection`, `customSheetViews`,
  `pageMargins`, `pageSetup`, `headerFooter`, `drawing`,
  `legacyDrawing`, `legacyDrawingHF`, `drawingHF`, `picture`,
  `webPublishItems`, `extLst`; `sheetViews` and `drawing` are
  required, every other child optional; `sheetViews` holds
  `CT_ChartsheetView` elements (`tabSelected`, `zoomScale`,
  `workbookViewId`, `zoomToFit`), not the worksheet's
  `CT_SheetView`. A chartsheet's drawing is a `wsDr` as a
  worksheet's, with the same three anchor forms; Excel writes a
  chartsheet's chart in an `absoluteAnchor` at `pos` 0,0 with an
  `ext` of the page's size in EMU (to confirm on a chartsheet
  written by desktop Excel when the maintainer supplies one; the
  corpus has none).
- openpyxl 3.1.5 (`openpyxl/chartsheet/chartsheet.py`, confirmed on the
  pinned image 2026-10-18 by planning, a workbook with one chartsheet
  and a bar chart saved and its entries read): `Chartsheet` writes
  `sheetViews` (one `sheetView` with `workbookViewId` 0 and `zoomToFit`
  1) and `drawing` (`rId1`) and nothing else; `sheetPr`, `pageMargins`,
  `pageSetup` and `headerFooter` are `None` unless set and are not
  written; `add_chart` sets the chart's anchor to `AbsoluteAnchor()`, so
  its drawing holds an `absoluteAnchor` with `pos` 0,0 and `ext` 0 by 0
  (the chart's `width` and `height`, 15 by 7.5 cm by default, are not
  used; `zoomToFit` lets Excel fill the page); the part is
  `xl/chartsheets/sheet<N>.xml`, the content type
  `...spreadsheetml.chartsheet+xml` (the full string is in `survey.rs`),
  the workbook relationship of type `chartsheet`; the chartsheet's
  drawing relationship targets the drawing part by its absolute name
  (`/xl/drawings/drawing<N>.xml`); `copy_worksheet` takes a worksheet
  only (`WorksheetCopy`) and copies no image or chart.
- The crate: `Sheet.kind` reads the workbook relationship's type
  (`WORKSHEET`, `CHARTSHEET`, `DIALOGSHEET` in both namespace
  forms); `survey` counts `chartsheets`; `remove_sheet` removes a
  chartsheet with the parts only it reaches
  (`a_chartsheet_is_removed_with_what_only_it_reaches`);
  `rename_sheet` and `move_sheet` take any kind and have no
  chartsheet test; `copy_sheet` refuses a chartsheet ("a sheet copy
  needs a worksheet", `specs/0028`); `add_sheet` writes the blank
  worksheet template (`template/xl/worksheets/sheet1.xml`, 157
  bytes), allocates `sheetId` one above the largest, the first free
  `worksheets/sheet<N>.xml` beside the workbook part, an `Override`
  unless a `Default` for `xml` names the worksheet content type,
  and the workbook relationship in the form the workbook's own
  worksheet relationships use.
- `charts`, `add_chart` and `remove_chart` resolve the sheet's
  drawing through `drawing_of` and `element_rel_id`, which refuses
  any root but `worksheet` with "the worksheet part's root is not
  worksheet" (`specs/0034` P1 ruling B, the test
  `a_chartsheet_part_is_refused_as_not_a_worksheet`), places a
  created `drawing` element by `WORKSHEET_CHILDREN`, and is shared
  with `images` (`drawing`) and `comments` (`legacyDrawing`). The
  anchor writer writes `absoluteAnchor` from `ImageAnchor::Absolute
  { position }` with `NewChart::size`, required for that form.
- `add_chart` takes one chart per call and any number of calls on
  one worksheet; the `cNvPr` id is one above the largest in the
  drawing; `remove_chart` skips the frame's root child and removes
  the chart part with what only it reaches (`specs/0036`).

## Design

### Crate

Under `charts`, with the reader and writer. The `images` and
`comments` features do not gain chartsheets: an image on a
chartsheet and a comment on one stay refused as today (the root
rule below is theirs only where the feature is `charts`).

### Reading

`Package::charts(part)` takes a worksheet part or a chartsheet part.
The root rule: `element_rel_id` learns the roots a caller accepts;
the three chart calls accept `worksheet` and `chartsheet`, and the
message for any other root becomes, for them, "the sheet part's
root is not worksheet or chartsheet" (one new message); `images`
and `comments` keep their message and their one root. The
`drawing` element's schema place is looked up in the list of the
root met (`WORKSHEET_CHILDREN` or a new `CHARTSHEET_CHILDREN` of the
fourteen names above). A chartsheet's drawing is read as a
worksheet's: every anchor form, the frames in document order, each
chart read by part 3's reader; a chartsheet with no `drawing`
element (invalid by the schema) reads as empty, as a worksheet
without one does. Nothing else of the chartsheet is read in this
part: no `ChartsheetView`, no page setup.

### `add_chartsheet`

```
Package::add_chartsheet(&mut self, name: &str, chart: &NewChart<'_>)
    -> Result<Vec<u8>, Error>
```

Creates a chartsheet named `name` at the end of the sheet list,
holding `chart`, and returns the new part's name. The name rules
are `add_sheet`'s (`check_sheet_name`). `chart` is checked as
`add_chart` checks it, with `size` required (decision 1 A: the
chart fills an absolute anchor at 0,0 of `size`; "a chart's size is
required by its anchor" is the existing message for a missing size
on an absolute anchor, reused). The call records:

- the chartsheet part, the first free `chartsheets/sheet<N>.xml`
  beside the workbook part, holding the blank chartsheet form:
  `sheetViews` with one `sheetView` carrying `workbookViewId` 0 and
  `zoomToFit` 1, then (decision 3 B only) `pageMargins` with Excel's
  worksheet defaults (`left` and `right` 0.7, `top` and `bottom`
  0.75, `header` and `footer` 0.3), then `drawing` with the drawing
  relationship's id; no `sheetPr` (optional, openpyxl omits it);
  the namespaces as the worksheet template declares them plus `r`;
  a constant in the crate, not a template
  file (the `template` directory is `Package::new`'s six parts);
- its `Override` of the chartsheet content type unless a `Default`
  for `xml` names it (the worksheet rule);
- the workbook relationship of the chartsheet kind, in the
  namespace form the workbook's worksheet relationships use;
- the `<sheet>` element through `SheetEdit::Add` grown by a kind,
  with `sheetId` one above the largest, so the projected list shows
  the new sheet as `SheetKind::Chartsheet` and a later `charts`,
  `remove_chart` or `add_chart` in the same write meets the kind;
- the drawing part and its rels part, created as `add_chart`
  creates them on a worksheet without one, holding the chart's
  frame in an `absoluteAnchor` at 0,0 of `size`, the chart part
  and its `Override`, all through the writers of parts 4 and 5.

A chartsheet added in this write cannot be removed, renamed or
moved in the same write (`add_sheet`'s rule for a worksheet, "no
such part" or the sheet-edit rule the crate applies, named in P1's
report).

### `add_chart` and `remove_chart` on a chartsheet

`add_chart(part, anchor, chart)` on a chartsheet part: `anchor` must
be `ImageAnchor::Absolute` (decision 1 A; a one-cell or two-cell
anchor refused with "a chartsheet's chart takes an absolute
anchor"), and the chartsheet's drawing must hold no frame (decision
2 A; refused with "a chartsheet holds one chart"; a removal in the
same write counts, so remove-then-add replaces a chart in one
write, the removal's skip and the addition's append as on a
worksheet). `remove_chart(part, index)` on a chartsheet: as on a
worksheet; the drawing part stays, empty, which `specs/0037` ruling
Q accepts (an empty `wsDr` opens in LibreOffice and Excel).

### The other sheet calls

`rename_sheet`, `move_sheet` and `remove_sheet` already take a
chartsheet; this part adds the tests that say so on a built package
and names them in the catalogue. `copy_sheet` keeps its refusal
(openpyxl copies worksheets only; a chartsheet copy would copy a
drawing and a chart, which no copy does; "Out of scope").

### Errors

Three new messages: "the sheet part's root is not worksheet or
chartsheet" (the chart calls on a dialogsheet or any other root),
"a chartsheet's chart takes an absolute anchor", "a chartsheet
holds one chart". Every other refusal is an existing message:
the name rules, "no such part", the chart's own checks, the size
rule, the drawing refusals. The catalogue in the `package` header
lists the three under `charts`.

### 32-bit and streaming

Nothing new: the chartsheet form is a constant, the drawing and
chart writers stream as in parts 4 and 5, and no chartsheet is read
whole beyond `element_rel_id`'s token walk.

### Harness runs

`--chartsheet-sample` in `emend-copy` (under `charts`): on each
corpus workbook, `add_chartsheet` of the name `emend chartsheet`
with `add_chart_sample`'s column chart (two series from the first
worksheet's columns A to C below its last row, the title, both
axis titles, the legend at the bottom) and `size` 9144000 by
6096000 EMU (Excel's default chart page at 100 per cent, to
confirm), then `charts` of the new part read back and compared
field by field, then the write; the tool's line prints the part
name's length, the chart count (1), the series count (2) and the
sheet count after. Expected on the corpus: every entry `identical`
but `xl/workbook.xml`, its rels part and `[Content_Types].xml`
(`changed`); five entries `added` (the chartsheet, its rels, the
drawing, its rels, the chart); `lo` and `lo_in` `ok` for every
workbook; the open check of the keep.

Then the two generated fixtures, each under the phase directory,
never committed, their values printable:

- Excel's form, written by the run log's own generator with
  `zipfile`: one worksheet of six cells, one chartsheet after it
  holding a column chart of those cells in an `absoluteAnchor` with
  the chartsheet children in schema order (the generator's text
  printed in the log). `--charts-sample` on it on both targets
  reads the chart (count 1, series 1); `--chartsheet-sample` adds a
  second chartsheet (sheet count 3, the first chartsheet
  byte-identical); the run log's own check with `ElementTree`
  confirms the new chartsheet's root children in schema order, its
  `sheetView` attributes, the `absoluteAnchor` with `pos` 0,0 and
  the `ext` of `size`, the `<sheet>` element last with the
  chartsheet relationship type, the `Override`.
- openpyxl's form, written by the pinned image through
  `scripts/openpyxl-ops.py chartsheet <in> <out>` (the op grows the
  script and its test; `create_chartsheet`, `add_chart` of a bar
  chart over the input's first sheet): `--charts-sample` on it
  reads the chart through its `absoluteAnchor` with `ext` 0 by 0
  (count 1), and the chartsheet sample adds a second chartsheet
  beside it.

`lo` and `lo_in` `ok` on every fixture output. The run log's own
checks print no corpus name, reference, path or text.

## Phases

Each phase as `specs/0001` "Rules for every phase" and part 6's
"Phases" paragraph: a doc commit first, the code commits, one
record commit, the scan last, at most two independent checks, the
report under `<phase>`, then stop. Target root
`<home>/.cache/emend-build/m6p1-pN/target`; P1's gate prunes
Milestone 5 part 6's roots (listed with `du -sb`, removed with
`rm -r`). Logs in `records/logs/m6p1-<phase>/` (`gate.log`,
`test.log` on both targets with the feature sets off, `charts`,
`charts,images`, `charts,comments`, `charts,read` and every
feature, `negative.log`, `run.log`, `scan.log`, `size.log` in P3);
the scan's denylist form whole-word. No push during the part; the
close-out push follows P3.

Part 1's first doc commit carries: this spec; `specs/0037` with its
close-out verification and rulings AN to AQ; `ROADMAP.md` with the
Milestone 5 heading and part 6's Close row DONE and PUSHED and this
part's rows; all three as planning left them, by hash.

Generator rules carried from `specs/0037` into this part and every later
one: `gen/defined.py` reads `is` and `isnot` calls as well as `yn` and
`ynnot`, so the gate's count is no longer 0 (ruling AK a); every rerun
log asserts its RERUN_FILES hashing with a check line, the run log's
form (ruling AK b); the size generator's stale labels ("M5 part 4's
headroom", "with its ruling W") are corrected to this part's (ruling AC
d); each script self-checks its helpers in its own shape block (ruling
Z), every refused or superseded attempt is named by RERUN_NOTE and
hashed by RERUN_FILES (ruling AA), and `yn` and `ynnot` themselves are
checked as defined, with a control (ruling AD).

### P1: the reader and the writer

The doc commit; the prunes; then the root rule and
`CHARTSHEET_CHILDREN`, `charts` on a chartsheet, the chartsheet
form, `SheetEdit::Add`'s kind, `add_chartsheet`, `add_chart` and
`remove_chart` on a chartsheet, the three messages,
`--chartsheet-sample`. Tests on built packages, each written
chartsheet read back by `charts` and compared: `add_chartsheet` on
a workbook with one worksheet (the part, its rels, the drawing,
the chart, the `Override`, the `<sheet>` last with the chartsheet
type and `sheetId` one above, the projected list's kind); on a
workbook whose `Default` for `xml` names the chartsheet type (no
`Override`); on a strict-namespace workbook (the strict
relationship form); a name taken, a name invalid, a missing `size`
each refused; `charts` on a chartsheet with one chart in each
anchor form; `add_chart` with a two-cell anchor refused; a second
chart refused; remove-then-add in one write; `remove_chart` leaving
an empty drawing; the chart calls on a dialogsheet part refused
with the new root message; the `images` and `comments` refusals on
a chartsheet unchanged; `rename_sheet`, `move_sheet` and
`remove_sheet` on a chartsheet; the tool's line. The gate as part
5 P1's plus the prunes. The negative log through the binary: the
chartsheet sample on a fixture whose workbook rels part names a
chartsheet relationship with no part (refused, with a control).
The run log: `--chartsheet-sample` on both targets over the corpus
and `--charts-sample` and `--chartsheet-sample` over the Excel-form
fixture, the checks as "Harness runs", recorded as
`records/m6p1-p1-<utc-date>-<shortsha>.md` and `.txt`; the open
record.

### P2: the openpyxl fixture and the catalogue

The doc commit (the P1 verification rulings); the carry; then the
`chartsheet` op of `scripts/openpyxl-ops.py` with its test line in
`scripts/openpyxl-ops-test.sh`; the `package` header's catalogue
and limits for this part. The run log: the fixture written by the
image (the plan printed first, rootless, the image id checked
against `records/openpyxl-image-2026-10-16.md`), `--charts-sample`
and `--chartsheet-sample` on it on both targets, the checks as
"Harness runs", the corpus run of P1 rerun byte-identical by
sha256, recorded as `records/m6p1-p2-<utc-date>-<shortsha>.md` and
`.txt`.

### P3: the probe, the size record, the API listing, the CHANGELOG

The doc commit (the P2 verification rulings); the carry; the probe's
`add_chartsheet` as "Size and packaging" says; the size log,
recorded as `records/size-m6p1-<utc-date>-<shortsha>.md`, with the
`charts` cost, its movement over part 5's 105424, the `images` and
`comments` movements and the core's growth each printed and judged;
the API listing rerun, recorded as
`records/api-review-m6p1-<utc-date>-<shortsha>.md`; `CHANGELOG.md`'s
"Added" gains one bullet citing this spec. The negative log through
the binary for every message of the part reachable there. The run
log: the P2 runs rerun with the size log's release binaries,
byte-identical by sha256, recorded as
`records/m6p1-p3-<sample>-<utc-date>-<shortsha>.md` and `.txt`; the
open record of the part's keep.

### Close-out push (after P3)

As Milestone 5 part 6's (`specs/0037` "Close-out push (after P4)"
and its close-out rulings): the generator transforms part 6's
scripts, its numbered changes continuing from where part 6's
close-out left them; the doc commit holds this spec and
`ROADMAP.md`; the base is part 6's close-out doc commit, named by
its subject; `PREV_RECORD_PATTERN` is `^M5 part 6 close-out
record`, in the range exactly once; the close-out doc commit's
subject is "M6 part 1 close-out: commit planning docs (the P3
verification and rulings)"; `CARGO_TARGET_DIR` printed and checked
as `<home>/.cache/emend-build/m6p1-close/target`; no Excel-record
check; the scan's denylist form whole-word; the record commit "M6
part 1 close-out record: the push, the CI run and the scan" holding
the three logs only; pre mode on a clean tree; one independent
check before the report. The push carries part 6's close-out record
commit and this part's commits, on planning's word and the
maintainer's authorisation typed in the implementing session. The
hashes and the change number are filled in at the briefing.

## Planning rulings during P1 (2026-10-18)

A. Remove-then-add in one write. The spec's "`add_chart` and
   `remove_chart` on a chartsheet" paragraph said the removal's skip and
   the addition's append replace a chartsheet's chart in one write, "as
   on a worksheet"; on a worksheet that write is refused today
   (`record_removals`, "no chart at that position" for a removal on a
   part the same write adds to; `specs/0036` P2's test "a removal and an
   addition on one part refused"; `remove_chart`'s doc), so the spec
   contradicted the tested rule. Ruled as the implementer's option B:
   the refusal stands for every sheet kind; on a chartsheet a
   replacement takes two writes, `remove_chart` then `add_chart`; "a
   chartsheet holds one chart" counts the frames the drawing holds, so
   `add_chart` on a chartsheet whose drawing holds a frame is refused;
   the P1 test "remove-then-add in one write" reads remove, write, add,
   write, with a control that remove and add in one write is refused
   with the worksheet's message. Decision 2 A's "remove-then-add
   replaces it" means in two writes.

B. Message text. The spec's "a chart's size is required by its anchor"
   names no string in the crate; the existing `CHART_SIZE_ANCHOR`, "a
   chart size does not apply to its anchor", already covers a missing
   size on an absolute anchor. Reused as it stands; the spec's wording
   is corrected by this ruling, and the message count of the part is
   three as "Errors" says.

C. Noted, no ruling: `NewChart::size` is in pixels (9525 EMU each), so
   the sample's 9144000 by 6096000 EMU is `size` (960, 640); a sheet
   added in the same write has no archive entry, so `add_chartsheet`
   writes its drawing, chart and rels itself through the shared writers,
   and the chart calls on a chartsheet added in the same write are
   refused with "no such part", as on a worksheet added in the same
   write.

## Planning verification of P1 (2026-10-19)

Verified from primaries: the chain 715aa75 (the three planning files at
planning's hashes), 56b2937, f917618 and f948fd9 on 67c9144; the record
commit adds the seven files of `records/logs/m6p1-p1` and the two record
pairs and nothing else; the messages ASCII, each carrying the count
sentence and the Co-Authored-By line alone; no U+2014 and no home path
in the range's diff; the seven logs and four records at the hashes the
README and the report give, each log ending "checks all as expected:
yes", the scan RESULT: PASS; planning's own privacy scan over
67c9144..f948fd9 (4 commits, 2013 blobs, 0 findings). The core diff read
whole: the root rule through `sheet_element_rel_id` with the
chartsheet's child list, the chartsheet form (decision 3 A),
`SheetEdit::Add` with a kind and the carried parts, `add_chartsheet`
with every read and check before the first record, the anchor and one-
chart checks at the write (decisions 1 and 2 A, ruling A), the empty
drawing kept, the three messages; nothing outside P1's list. The created
drawing takes `strict: false` and the `xl/` names as `add_chart`'s own
created drawing does (parts 4 and 5), so check 1's item 4 is the
existing rule. The chartsheet sample's chart equals `add_chart_sample`'s
field for field but `size` (960, 640), so item 5 is answered. Test
totals on both targets off 515, charts 634, charts,images 676,
charts,comments 679, charts,read 737, every feature 1116; the corpus 27
of 27 on both targets, `lo` and `lo_in` ok on 27, the entry families as
"Harness runs" expects (the workbook part, its rels and the content
types changed; the chartsheet, two rels, the drawing and the chart
added); the keep's 35 new pairs; the open record 1472 pairs with the
same 12 and 11 LibreOffice failures as its predecessor's 1437, so no
finding. The charts probe reads 1086440, 10316 over part 6's record,
inside the estimate of 6000 to 11000; P3's size record judges it.
Planning hashed the superseded run script itself (335c74f5) and the two
dry logs (af1a3dbf, ca08ba01): as the report says.

D. FINDING, change NWZ: the first run log printed two placeholders
   before a slash with no root line declaring them, found by the scan's
   pre-check; the run generator's own check read a fixed token list. The
   rerun at the same commit under RERUN_NOTE and RERUN_FILES is accepted
   (rulings AA and AK b honoured). The widened check's control is a
   session's statement no log prints: from P2 on, a generator's own
   placeholder check prints its control in the log it writes, the scan's
   form.

E. The same-write rule for an added chartsheet. The spec's sentence
   ("cannot be removed, renamed or moved in the same write") was
   planning's error; the crate's rule is `specs/0016` P4 ruling 4, a
   pending sheet may be removed (its additions dropped, nothing
   written), renamed or moved like any sheet of the projected list. The
   chartsheet takes that rule, its removal dropping the four carried
   parts, as tested. The sentence is corrected by this ruling. P2's
   first code commit adds the test of a pending chartsheet renamed and
   moved in the write that added it, the output read back through the
   sheet list and `charts`.

F. Ruling AA's reach. AA names logged attempts. For P1: the first run
   log, its stderr and its pre-check hashed by RERUN_FILES is accepted;
   the first run's records derive from it and need no hash; the two dry
   logs hashed in the README alone is accepted, a dry run being no
   logged attempt. From P2 on, RERUN_FILES also names the superseded
   script that wrote the attempt, so the rerun's log hashes the
   generator state it replaced.

G. The three messages catalogued in the `package` header in P1, which
   the spec gave to P2, is accepted: the catalogue test requires it.
   P2's "catalogue and limits" reads the limits alone.

H. N7, "the sheet part's root is not worksheet or chartsheet" reachable
   through the binary on a chartsheet relationship to a dialogsheet
   part, noted as shown; the message count stays three. The sample
   line's `kept_charts` key and `--charts-sample` reading chartsheets
   are accepted as disclosed; the fixture run of P2 needs the latter.

I. The core commit amended after the first dry test log (an unused test
   constant under two feature sets, an intra-doc link with no features)
   and the tool commit replayed, messages unchanged, the old pair on
   `p1-before-fix`: local, unreported commits (`specs/0001`). The dry
   negative driver fault, rerun with the gate log, noted. P1 DONE.

## Planning rulings during P2 (2026-10-19)

J. The openpyxl fixture refuses `--chartsheet-sample`. openpyxl declares
   the relationships namespace on each `<sheet>` element, not on the
   workbook root, so `add_chartsheet`, through `add_sheet`'s path, is
   refused with "the workbook root binds no prefix for the relationships
   namespace" (`specs/0016`, the sheet-add rule, tested by
   `a_workbook_root_binding_no_relationships_prefix_refuses_an_add`);
   the corpus never met it because no corpus workbook is openpyxl's. The
   spec's "adds a second chartsheet beside it" was planning's error.
   Ruled as the implementer's option A: no crate change; the run log
   checks the refusal on the fixture (exit 2, that message, no output
   file) with `--charts-sample`'s read as the positive check (sheets 3,
   charts 1, series 1, the bar kind, the absolute anchor), on both
   targets. A sheet add on a workbook whose root binds no relationships
   prefix (every openpyxl-written workbook) is a parity item for a later
   part, listed in `ROADMAP.md` "Unscoped"; the maintainer decides when.

## Planning verification of P2 (2026-10-19)

Verified from primaries: the chain 50def41 (the two planning files at
planning's hashes), 1117c8e, aae54ea, 2a706dc and ef645e2 on f948fd9;
the record commit adds the seven files of `records/logs/m6p1-p2` and the
two record pairs and nothing else; the messages ASCII with the count
sentence and the Co-Authored-By line alone; no U+2014 and no home path
in the range's diff; the seven logs and four records at the hashes the
README and the report give, each log ending "checks all as expected:
yes", the scan RESULT: PASS naming its passing pre-check by RERUN_NOTE
and RERUN_FILES; planning's own privacy scan over f948fd9..ef645e2 (5
commits, 2021 blobs, 0 findings). The three code diffs read whole:
ruling E's test (added, renamed, moved first, read back through the
sheet list and `charts`); the `chartsheet` op and its test line (one
absolute anchor at 0,0 of ext 0 by 0, one bar chart over the fixture's
ranges, 13 checks); the header's chartsheet limits paragraph with two
doc hunks (ruling K). Test totals on both targets off 515, charts 635,
charts,images 677, charts,comments 680, charts,read 738, every feature
1117, one above P1's in each set that builds the test; the corpus rerun
27 of 27 on both targets byte-identical to P1's by sha256, `lo` and
`lo_in` ok on 27; the openpyxl fixture written by the pinned image
rootless as uid 1001 with the image id checked against the image record;
`--charts-sample` on it sheets 3, charts 1, series 1, a bar in an
absolute anchor, `lo` and `lo_in` ok; `--chartsheet-sample` refused on
both targets with the NO_RELATIONSHIPS_PREFIX literal, no output and no
temporary (ruling J); N8 the same refusal through the binary on a
fixture of openpyxl's form; the keep 1474 pairs, 2 new; the open
record's 12 and 11 LibreOffice failures as before, no finding; the
charts probe 1086440, 0 over P1. The report's corrections after check 1
(the negative log's end time 08:33:50Z, the pre-check's directory, two
statements marked the session's) read against the logs.

K. The limits commit's two doc hunks outside ruling G's "the limits
   alone": (a) the charts-reading sentence "A chartsheet is not read (P1
   ruling B)", false since P1, replaced by the reading through the
   chartsheet's one `drawing` element; (b) the worksheet content type's
   doc comment moved back to its constant, from which P1's chartsheet
   constants had parted it. Both ruled in: (a) is the catalogue's own
   correction, which "catalogue and limits" covers before ruling G
   narrowed it, and the header must not state a falsehood; (b) is P1's
   misplacement. The report disclosed (b) only and check 1 found (a);
   the commit message names both.

L. The dry gate's generator fault (change I69 cut the line setting
   PKG_TESTS; fixed, the failed log and its script kept under
   superseded) and the dry driver that ran nothing: noted; neither was a
   logged attempt (ruling F). `compare-run.sh` and `compare-record.py`
   unchanged, the chartsheet op being a fixture op and not a comparison
   op: accepted as the session's statement, and Milestone 6 part 6
   decides whether the comparison gains it.

M. P2 DONE. P3 as the spec's "P3: the probe, the size record, the API
   listing, the CHANGELOG", with these readings: the size record states
   the `charts` feature's whole cost, part 6's record's figure (105916,
   probe 1076124) and part 5's 105424 as the spec names, and the delta
   over part 6's record, judged against the estimate of 6000 to 11000
   and the finding line at 15000; the `images` and `comments` movements
   and the core's growth each printed and judged (0 expected, accepted
   to 1500 each and 2000); the size generator's stale labels corrected
   (`specs/0037` ruling AC d); the negative log through the binary for
   every message of the part reachable there (N6 to N8 as P1's and P2's,
   the anchor and one-chart messages if reachable, else the log says why
   not); P2's runs rerun with the size log's release binaries byte-
   identical by sha256.

## Planning verification of P3 (2026-10-19)

Verified from primaries: the chain 8910350 (the two planning files at
planning's hashes, f15be1a0 and e39db59f), 3a5ea11, 4cc4f2e and 20d00be
on ef645e2; the record commit adds the eight files of
`records/logs/m6p1-p3` and the six records and nothing else; no crate
file changed; the messages ASCII with the count sentence and the Co-
Authored-By line alone; the eight logs and six records at the hashes the
README and the report give, each log ending "checks all as expected:
yes", the scan RESULT: PASS naming its passing pre-check (29ee3bf7, in
`<phase>/superseded/pre-check-1`) by RERUN_NOTE and RERUN_FILES;
planning's own privacy scan over ef645e2..20d00be (4 commits, 2032
blobs, 0 findings). The two diffs read whole: the probe's
`add_chartsheet` of the first set's column chart at (960, 640), written
and reopened, `charts` of the new part printed as `chartsheet_charts` (1
at every kind index); the CHANGELOG's one bullet under "Added" citing
this spec. The API record: one row new, `Package::add_chartsheet`, none
gone, the three messages crate-private. The negative log: N6 to N8 as
P1's and P2's, N9 new (ruling P), CHARTSHEET_ANCHOR and ONE_CHART
unreachable through the binary with five planted controls and their
covering tests named (ruling M met). The run log: P2's corpus run rerun
on the size log's release binaries 27 of 27 byte-identical by sha256 on
both targets; the fixture two ways (ruling O); the keep 1474 pairs
unchanged, 0 new, the open .txt P2's byte for byte. The independent
check's answer in the report verbatim (its file carries a five-line
heading before it). The size figures are read off size.log and the
record, and judged by planning here, below.

N. The size record stands as committed, with two of its judgments
   corrected by this ruling and not by a rerun. (a) The `images` and
   `comments` bands of "Size and packaging" are on the features' costs'
   movements, which the record prints: images 57504 against 57620, a
   move of -116; comments 63292 against 63356, a move of -64; each
   inside 1500, no finding. The record's two FINDING lines for those
   probes judge the probes' growth (1852 and 1904), which is each move
   with the core's growth of 1968, and so are not findings of this part;
   the size log's findings for this part are one, the charts probe's.
   (b) The record's copy paragraph cites this spec for an expected
   growth of 0 it does not state; the bands table's judgment is the one
   that stands: `copy` 1792 and the pair 1932, each accepted up to 2000,
   no finding. Both are generator faults of the size log; a rerun of the
   size log and the run log for two labels is not asked. Generator rule
   from the next size log on (Milestone 6 part 2): the `images` and
   `comments` bands judge the costs' movements, printed and judged under
   their own names, with the probes' growth printed beside them
   unjudged; the copy paragraph cites the band it is judged by, or the
   spec section that states its expectation, never a section that does
   not.

O. Finding 2 of the report stands as a finding: this part's cost, the
   charts probe's growth over part 6's record, 15396 section bytes,
   above the finding line of 15000 and planning's estimate of 6000 to
   11000; the feature `charts` costs 119344, a move of 13428 over part
   6's 105916 and 13920 over part 5's 105424; the core grew 1968,
   expected 0, accepted up to 2000; the budget figure 438960, 19792
   under 458752. Recorded, not fixed: the cost lies with P1's chart
   writer and package code (56b2937, the one commit changing code lines
   among the probes' sources). Against the estimate's top of 11000 the
   `charts` cost moved 2428 more, and the core's 1968, expected 0, is
   the rest; planning's reading, not measured, is that
   `SheetEdit::Add`'s kind and carried children sit in the core's sheet
   path, not under `charts`. Milestone 6 part 6 (measurement and close)
   reads the `charts` cost whole and decides whether any of it is to be
   regated; no part of Milestone 6 takes this as a reason to change
   code.

P. The fixture's "byte-identical" reading is confirmed. openpyxl writes
   a new file each run, so a fresh fixture's outputs cannot match P2's
   by sha256; the run log reruns P2's own kept fixture (the one whose op
   output P2's run.log hashed, 834a7f6d) byte for byte on both targets,
   2 of 2, and compares a fresh fixture's charts line and refusal's
   standard error with P2's run.log, each target. That is what ruling
   M's "byte-identical by sha256" asks of the P2 runs. N9 stays: the
   add-chart sample on B4 with its worksheet part's root a chartsheet,
   refused by the cell read with NOT_WORKSHEET before
   `Package::add_chart`, with its refusal-text controls; it is the
   binary's half of why CHARTSHEET_ANCHOR is unreachable, and the
   accepted add-chart sample on B0 (B1) is its accepted control. An
   accepted add-chart sample on B4 itself is not asked. The carry: the
   brief named none, and the test log's comment says P2's was the
   part's; no carry commit is owed.

Q. P3 DONE. Part 1's close-out as the spec's "Close-out push (after
   P3)": the doc commit commits this verification and rulings N to Q and
   the ROADMAP's P3 row; pre mode on a clean tree; planning's word, then
   the maintainer's typed authorisation in the implementing session; the
   push; the CI run; the post scan; the record commit with the three
   logs. The pushed range carries 67c9144, Milestone 5 part 6's
   close-out record, as its first commit, `PREV` `^M5 part 6 close-out
   record` once, base c970940 by its subject.

## Decisions for the maintainer

1. The anchor of a chartsheet's chart: A (recommended), an
   `absoluteAnchor` at 0,0 with `ext` of the chart's `size`, required
   (`add_chart` on a chartsheet takes `ImageAnchor::Absolute` only, any
   position; the sample's 9144000 by 6096000 EMU is the figure to
   confirm against a chartsheet written by desktop Excel); B, openpyxl's
   form, an `absoluteAnchor` at 0,0 with `ext` 0 by 0 and no `size`
   argument (`zoomToFit` lets Excel fill the page; what LibreOffice
   draws for a zero extent is not known until the open check). Neither
   openpyxl nor Excel writes a cell anchor on a chartsheet, so none is
   offered. Either form is read; the choice is what `add_chartsheet`
   writes and what `add_chart` accepts on a chartsheet.
2. A second chart on a chartsheet: A (recommended), refused ("a
   chartsheet holds one chart"; Excel's model, one chart per
   chartsheet; remove-then-add replaces it); B, allowed (the schema
   permits any number of frames; Excel shows the first).
3. The chartsheet form's `pageMargins`: A (recommended), omitted
   (optional by the schema; openpyxl omits it and its chartsheets open
   in Excel; the smaller write); B, written with Excel's worksheet
   defaults (`left` and `right` 0.7, `top` and `bottom` 0.75, `header`
   and `footer` 0.3), on the reading that desktop Excel writes it, a
   reading no sample confirms.

## Out of scope

- A chartsheet's view (`ChartsheetView`), page setup, header and
  footer, protection and `customSheetViews` read or written (a
  later part if wanted; the `sheetprops` writers target the
  worksheet's children); a chartsheet with no chart
  (`add_chartsheet` takes the chart); a copy of a chartsheet
  (openpyxl copies worksheets only); an image or a comment on a
  chartsheet; a dialogsheet; `chartex`; surface and of-pie kinds,
  markers, lines and per-point formatting (part 2); trendlines,
  error bars and axis scaling (part 3); text properties, the chart
  style and in-place edits (part 4).
