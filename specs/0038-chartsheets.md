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
