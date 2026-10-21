# specs/0041: chart text and in-place edits (Milestone 6 part 4)

Status: APPROVED 2026-10-21 (Milestone 6 part 4; decisions 1 to 4 A,
typed in the planning session 2026-10-21: the title's properties in
`defRPr` and `rPr`; `c:style` alone, no style or colour parts; the
title and one series edited in place; the reader unchanged). Planning
session. Builds on `specs/0035` (the first set), `specs/0036`
(the second set), `specs/0038` (chartsheets), `specs/0039` (kinds and
formatting) and `specs/0040` (analytic elements); the writer's forms,
checks and messages named there stand. The part is proven on built
packages, on the corpus run of two new samples, on two generated
fixtures and by the LibreOffice open check, as part 3 was; the
Milestone 2 Excel check gates no push.

## Goal

The text and editing items of `specs/0036`'s out-of-scope list,
Milestone 6's part 4 (`ROADMAP.md`, "Part 4: chart text and in-place
edits"): a written chart's title, axis titles, axis labels, data
labels and legend take text properties (size, bold, italic, colour,
typeface); a written chart takes a `style` value; a kept chart's title
and a kept series' references are changed in place, every other byte
of the chart part kept. openpyxl's `RichText` with
`CharacterProperties` on `title`, `txPr` and `dataLabels`, and
`ChartBase.style`. The reader is unchanged: it reads a chart's
`title`, `style`, and each series' `name`, `categories` and `values`
already (`specs/0034`), which is what the edit sample reads back.

## Size and packaging

The budget figure after part 3 is 438960 section bytes against the
ceiling of 458752 (`records/size-m6p3-2026-10-21-0707236.md`): 19792
of headroom, held for fixes and part 5. Everything in this part goes
under the feature `charts`, measured by `probe-emend-charts-write-xml`,
whose work gains text properties on every chart of the second set, the
style value, and an in-place edit of one kept chart ("The probe"). The
`charts` feature costs 135784 section bytes after part 3 (the probe
1107960); the probe's movement over that is this part's cost, and the
size record states the feature's whole cost, part 3's figure and the
delta.

Planning's estimate, priced as `specs/0036` prices (about 500 section
bytes per fixed form, call site and literal together) and, after
`specs/0039` ruling P, for the arms the probe reaches. The text
properties (`txPr` with `bodyPr`, `lstStyle`, `p`, `pPr`, `defRPr`
with `sz`, `b`, `i`, `solidFill` `srgbClr`, `latin`, `endParaRPr`, and
the title's `rPr`: 9 forms, 5 call sites) 4500 to 7000; the `style`
element 500 to 1000; the in-place edit (the chart part resolved from
the frame as the removal resolves it, the part's walk, the `title`
replaced or inserted, one `ser` found by index, its `tx`, `cat` or
`xVal`, `val` or `yVal` replaced or inserted without caches: about 12
forms and states) 5000 to 9000; the checks 2000 to 3500. In all 12000
to 20500 under `charts`. Planning is told the figure in the report at
any value; a cost above 25000 is a finding the report names. The
core's growth is expected 0 and accepted up to 2000. The `images` and
`comments` costs may move (expected 0, accepted up to 1500 each): the
size log judges the costs' movements under their own names, the
probes' growth printed beside them unjudged (`specs/0038` ruling N).
The chart style and colour parts are NOT written (decision 2 A): one
pair of Excel's literals would cost 8000 to 11500 by itself, more than
half the part's estimate, for parts Excel regenerates.

## Facts planning checked (2026-10-21)

- The corpus (`specs/0034`'s survey, re-counted by planning's script,
  never committed; counts only): 26 chart parts in 11 workbooks; 16
  with a `style` value; 15 with a chart style part and a chart colour
  style part (relationship types `chartStyle` and `chartColorStyle`
  from the chart part's rels to `style<N>.xml` and `colors<N>.xml`,
  content types `application/vnd.ms-office.chartstyle+xml` and
  `application/vnd.ms-office.chartcolorstyle+xml`, roots
  `cs:chartStyle` of 7646 to 10575 bytes and `cs:colorStyle` of 222 to
  878 bytes); 107 `txPr` elements, by parent `valAx` 31, `dLbl` 23,
  `legend` 20, `catAx` 17, `dLbls` 10, `title` 6; the 13 titles all
  `rich` with `a:pPr` and `a:rPr`, 3 with a `txPr` beside the `tx`;
  the run properties' attributes `sz`, `b`, `i`, `u`, `strike`,
  `kern`, `baseline`, their children `latin`, `ea`, `cs`,
  `solidFill` (`schemeClr` 60 with `lumMod` and `lumOff`, `srgbClr`
  28), `uFillTx`; `sz` from 900 to 1500, 1000 the commonest; `bodyPr`
  attributes `rot`, `wrap`, `anchor`, `vert` and the insets.
- openpyxl 3.1.5 on the pinned image (planning's probe 2026-10-21):
  `chart.style = 26` writes `<style val="26"/>` (plain `c:style`, no
  `mc:AlternateContent`) and NO style or colour part; a `title` string
  writes `tx`, `rich`, `bodyPr`, `p`, `pPr` with an EMPTY `defRPr`,
  and the run with `t`, no `rPr`; `x_axis.txPr` with
  `CharacterProperties(sz=1400, b=True, i=False, latin=Font("Arial"))`
  writes `txPr` after `minorTickMark` (before `crossAx`, openpyxl
  writing no `tickLblPos`) holding `bodyPr`, `p`, `pPr` with
  `defRPr sz="1400" b="1" i="0"` and a `latin typeface="Arial"` child,
  an empty run and an `endParaRPr` repeating the properties;
  `dataLabels.txPr` the same form inside `dLbls` before `showVal`.
- The writer at 69f4c89: `title_bytes` writes `title`, `tx`, `rich`,
  `bodyPr`, `lstStyle`, `p`, `r`, `t`; no `pPr`, no `rPr`, no `txPr`
  anywhere, no `style` element. `remove_chart` resolves the frame at
  an index to the chart part through the drawing's rels and removes
  the part with its rels and the parts those alone reach (the style
  and colour parts among them, `specs/0036` decision 3 A).
- The reader at 69f4c89: `Chart` holds `title`, `auto_title_deleted`,
  `style` (`c:style` plain or in `mc:AlternateContent`, `specs/0034`
  ruling R), `plots` with each `Series`' `name`, `categories`,
  `values` and `sizes`; it walks past `txPr` and `spPr`. Nothing of
  this part changes it (decision 4 A).
- The schema: `a:sz` is 100 to 400000 (hundredths of a point);
  `c:style` `val` 1 to 48; `CT_TextCharacterProperties` children in
  order `ln`, fills (`solidFill` among them), `effectLst`, `highlight`,
  `uLnTx`, `uFillTx`, `latin`, `ea`, `cs`, `sym`, `hlinkClick`;
  `CT_Title` children `tx`, `layout`, `overlay`, `spPr`, `txPr`;
  `CT_CatAx` and `CT_ValAx` take `txPr` after `spPr`, which follows
  `tickLblPos`; `CT_DLbls` takes `txPr` after `spPr`, which follows
  `numFmt`, before `dLblPos`; `CT_Legend` takes `txPr` after `spPr`,
  which follows `overlay`; `CT_ChartSpace` takes `style` after
  `roundedCorners`, before `clrMapOvr`, `pivotSource`, `protection`,
  `chart`; `CT_Chart` takes `title` first, then `autoTitleDeleted`;
  `CT_BarSer` and its kin take `tx` after `order`, then `spPr` and the
  kind's own children, then `cat` and `val` (`xVal`, `yVal` and
  `bubbleSize` on a scatter and a bubble); `numRef` and `strRef` take
  `f` then an optional cache.
- Excel's behaviour planning relies on: a chart without style and
  colour parts opens and is styled by `c:style` (every openpyxl chart
  of Milestone 5 part 6 and this milestone opened so); a series
  reference without a cache is recomputed on open (openpyxl writes
  none); a title's text properties in `pPr`'s `defRPr` govern the run
  where the run's `rPr` repeats nothing.

## Design

### Crate

Under `charts`, in the writer and in one new in-place walk: no reader
change (decision 4 A). `images` and `comments` gain nothing. No new
dependency.

### Types

- `NewText<'a> { size: Option<u16>, bold: bool, italic: bool, color:
  Option<u32>, font: Option<&'a str> }`, `Clone`, `Copy`, `Debug`,
  `PartialEq`, `Eq`. `size` in hundredths of a point (1400 is 14 pt),
  100 to 40000; `color` an RGB value as a series fill is given, written
  `solidFill` `srgbClr` with six upper-case hex digits; `font` the
  `latin` typeface, 1 to 31 characters, no control character, escaped
  as an attribute. `Default` is every field off: `NewText::default()`
  writes an empty `defRPr` (openpyxl's title form).
- `NewChart` gains `title_text: Option<NewText<'a>>`,
  `axis_title_text: Option<NewText<'a>>` (both axis titles),
  `axis_label_text: Option<NewText<'a>>` (both axes' tick labels),
  `legend_text: Option<NewText<'a>>` and `style: Option<u8>`;
  `NewLabels` gains `text: Option<NewText<'a>>`. The `Default`s are
  `None`. `NewChart` and `NewLabels` keep their derives.
- `ChartSeriesEdit<'a> { name: Option<NewSeriesName<'a>>, categories:
  Option<&'a str>, values: Option<&'a str> }`, `Clone`, `Copy`,
  `Debug`, `PartialEq`, `Eq`, `Default`: each `Some` replaces that
  reference of the kept series; `None` leaves it.
- `Package::set_chart_title(&mut self, part: &[u8], index: usize,
  title: Option<&str>) -> Result<(), Error>` and
  `Package::set_chart_series(&mut self, part: &[u8], index: usize,
  series: usize, edit: &ChartSeriesEdit<'_>) -> Result<(), Error>`,
  recorded at the call and applied at the write as `remove_chart` is.
  `package.rs` re-exports `NewText` and `ChartSeriesEdit`.

### Applicability

- `title_text` applies where `title` is `Some` (refused with the
  option-kind message where the chart has no title);
  `axis_title_text` where at least one axis title is given;
  `axis_label_text` and `legend_text` on every kind that writes the
  axes or the legend (`legend_text` with `legend` `Some`); a round
  kind refuses `axis_title_text` and `axis_label_text` as it refuses
  the axis titles. `NewLabels::text` applies wherever labels apply.
- `size` 100 to 40000, `font` by the rule above, else "a chart value
  is out of its range". `style` 1 to 48, else the range message.
- A combined chart's second plot takes `NewLabels::text` by the second
  plot's labels; the secondary axis takes no text (unchanged).
- `set_chart_title` and `set_chart_series` take a part and an index as
  `remove_chart` does and refuse what it refuses ("no chart at that
  position", "a chart part cannot be read", the sheet-root message);
  an index this write removes, adds to or edits twice is refused with
  "no chart at that position". `set_chart_series`'s `series` counts
  the `ser` elements of the plot area in document order across its
  plots, from 0; past the count, "no series at that position" (ONE new
  message). The edit's references are checked as a `NewSeries`'s are
  (the sheet-qualified range forms, `specs/0035`); `categories` is
  refused on a series whose plot takes no categories (a pie's are
  categories; a scatter's and a bubble's `categories` names `xVal`,
  as `NewSeries` does) and `values` names `val` or `yVal` by the plot.

### The chart part

- Text properties as `txPr`: `<c:txPr><a:bodyPr/><a:lstStyle/><a:p>
  <a:pPr><a:defRPr ATTRS>CHILDREN</a:defRPr></a:pPr><a:endParaRPr
  lang="en-US"/></a:p></c:txPr>`, ATTRS `sz`, `b` (`1`), `i` (`1`)
  where given, CHILDREN `solidFill` then `latin` where given; placed
  after `tickLblPos` on an axis (before `crossAx`), after `overlay` on
  the legend, after `numFmt` in `dLbls` (before `dLblPos` and the
  `show` flags); the `title`'s and an axis title's properties inside
  the rich text, `a:pPr` with the `defRPr` before the run and the run's
  `a:rPr` carrying the same attributes and children (Excel's form,
  decision 1 A), the `lstStyle` kept.
- `style` as `<c:style val="N"/>` after `roundedCorners` (plain, the
  schema's and openpyxl's form; no `mc:AlternateContent`, no style or
  colour part, decision 2 A).
- The in-place walk: the chart part is read whole and rewritten
  through the existing drawing walk's resolution of the frame; the
  rewrite copies every byte except: the `c:chart`'s `title` element
  replaced by `title_bytes`'s form for `Some` (an existing `txPr`
  beside the `tx` kept, the `tx` replaced; a `strRef` title replaced
  whole), or removed with `autoTitleDeleted` set `1` for `None`; where
  the chart has no `title`, one is inserted as `c:chart`'s first
  child and an `autoTitleDeleted` `1` becomes `0`. For a series, the
  `ser` at the index: `tx` replaced by the writer's form for `name`
  (inserted after `order` where absent); `cat` or `xVal` and `val` or
  `yVal` replaced by `numRef` or `strRef` with `f` alone, NO cache
  (inserted at the schema place where absent, before `val`, or before
  the plot kind's trailing children where the series has neither);
  `extLst` kept. A series the plot area's `ser` count does not reach
  is refused before the write touches the part. The part's prefix
  (`c:` or none) and declaration are kept as found; a part whose
  elements carry another prefix for the chart namespace is walked by
  namespace, as the reader does (`specs/0034`).
- Order of checks at the call: the part and index as `remove_chart`'s
  (ruling order carried), then the title's text or the edit's
  references in declaration order, then the series index at the
  write, then the part's readability.

### Errors

One new message, "no series at that position". "a chart value is out
of its range" covers `size`, `font` and `style`; "a chart option does
not apply to its kind" the applicability rules; the references' and
the chart position's refusals are the existing ones. The catalogue in
the `package` header gains the entry and the limits (the `size` and
`style` ranges, the `font` length).

### Harness runs

`--chart-text-sample` (P1): three charts past the last row of the
first worksheet, each read back, the references bounded by
`specs/0039` ruling C: a `Bar` with a title in 14 pt bold, axis titles
in 10 pt italic, axis labels in 9 pt, data labels on its one series in
8 pt with the colour `1F77B4`, the legend in 9 pt `Arial`; a `Line` of
two series with a title whose `NewText` is the default (openpyxl's
empty `defRPr`) and `style` 26; a `Pie` with data labels in 10 pt bold
`Arial` and the legend at the right in 9 pt. The tool's line prints
the chart count before and after, the series count, the rows, the
`txPr` elements and `style` elements its own reading of the three new
parts counts (`texts`, `styles`), and the read-back count (3; the
reader reads the titles and the style). Expected on the corpus: three
chart parts `added`; `lo` and `lo_in` `ok`; the open check of the
keep.

`--chart-edit-sample` (P2): two writes. The first adds one `Bar` of
one series past the last row (the format sample's shape); the second,
on the first write's output, calls `set_chart_title` with a new title
and `set_chart_series` on series 0 with a new `name` reference, new
`categories` and new `values` (the next column, bounded), and prints
the chart count (unchanged), the series count, the rows, the
edited chart's title and references read back equal to what was set
(`read_back` 3: the title, the name's reference, the two ranges), and
the entry statuses: the chart part `changed`, every other entry
`identical` to the first write's output. On a workbook holding kept
charts the sample also edits the FIRST kept chart's title alone, in
the second write, and reads it back (`kept_edited` 1 or 0), so the
walk meets Excel's and LibreOffice's parts on the corpus. Expected on
the corpus: `lo` and `lo_in` `ok` for every workbook; the open check
of the keep.

Then the two generated fixtures, each under the phase directory, never
committed, read on both targets, byte-identical across them:

- Excel's form (P1), written by the run log's own generator with
  `zipfile`: one worksheet of twelve cells, one drawing holding a
  `barChart` of one series with `dLbls` holding a `txPr`, a `title`
  whose rich text carries `pPr` `defRPr` and `rPr` (14 pt bold), a
  `catAx` and `valAx` each with a `txPr` after `tickLblPos`, a
  `legend` with a `txPr`, and `mc:AlternateContent` holding `c14:style`
  102 with the fallback `c:style` 2 (the corpus's form), with a chart
  style part and a colour style part reached from the chart's rels, in
  schema order (the generator's text printed in the log).
  `--charts-sample` on it on both targets reads the chart (count 1,
  series 1, `style` 2); `--chart-text-sample` adds three beside it
  (count 4); `--chart-edit-sample` edits it in place (`kept_edited`
  1) and the run log's own check with `ElementTree` confirms the
  edited part keeps its `txPr`s, its `style`, its rels and both style
  parts byte for byte, the `title`'s `tx` alone replaced.
- openpyxl's form (P2), written by the pinned image through
  `scripts/openpyxl-ops.py chart-text <in> <out>` (the op grows the
  script and its test; a `BarChart` over the input's first sheet with
  `style` 26, a `title`, `x_axis.txPr` of `sz` 1400 bold with `latin`
  `Arial`, `dataLabels` showing values with a `txPr` of `sz` 900):
  `--charts-sample` on it reads the chart (count 1, `style` 26);
  `--chart-text-sample` adds three beside it (count 4);
  `--chart-edit-sample` edits it (`kept_edited` 1); the run log's own
  check confirms the written forms against the openpyxl ones field by
  field where both write the field (`style` `val`, `sz`, `b`, `i`,
  `latin` `typeface`, each `txPr`'s parent and its place by the
  schema's sequence with the preceding sibling printed for both,
  `specs/0040` ruling N's form), each printed; openpyxl's empty run
  and `endParaRPr` and the title's missing `rPr` are named in the log
  and not compared.

`lo` and `lo_in` `ok` on every fixture output. The run log's own checks
print no corpus name, reference, path or text.

### The probe

`PLOTS` is unchanged, sixteen kinds. The second set's chart of every
kind takes `title_text` (14 pt bold), `axis_title_text` and
`axis_label_text` where the kind writes axes, `legend_text`, and
`NewLabels::text` on every series that takes labels; every kind takes
`style` 26. After the second add, the probe edits the first set's
chart in place: `set_chart_title` to a constant and `set_chart_series`
on series 0 with constant references (one arm each), so the walk is
priced. One value per option from the probe's constants ("Size and
packaging"). The chartsheet addition is unchanged; the probe's line
count is unchanged (`chartsheet_charts` last). The probe's header
comment and the size record's charts probe row describe the second
add and the edit as ONE description with one index range, 0 to 15
(`specs/0039` ruling Q).

## Phases

Each phase as `specs/0001` "Rules for every phase" and part 3's
"Phases" paragraph: a doc commit first, the code commits, one record
commit, the scan last, at most two independent checks, the report
under `<phase>`, then stop. Target root
`<home>/.cache/emend-build/m6p4-pN/target`; P1's gate prunes Milestone
6 part 3's roots (`m6p3-p1`, `m6p3-p2`, `m6p3-p3`, `m6p3-close`, listed
with `du -sb`, removed with `rm -r "${var:?}"`, `specs/0040` ruling
G). Logs in `records/logs/m6p4-<phase>/` (`gate.log`, `test.log` on
both targets with the feature sets off, `charts`, `charts,images`,
`charts,comments`, `charts,read` and every feature, `negative.log`,
`run.log`, `scan.log`, `size.log` in P3); the scan's denylist form
whole-word. The generator rules of `specs/0040` P1 to P3 and its
rulings (A to X) are carried. No push during the part; the close-out
push follows P3.

Part 4's first doc commit carries: this spec; `specs/0040` with its
close-out verification and ruling X; `ROADMAP.md` with part 3's
heading DONE and PUSHED and this part's rows; all three as planning
left them, by hash.

### P1: text properties

The doc commit; `NewText`, the five `NewChart` fields less `style`,
`NewLabels::text`, their rules and forms; `--chart-text-sample`; the
tests on built packages (each placement's form and place, the range
and applicability refusals, the escaping of `font`); the negative log
through the binary; the corpus run; the Excel-form fixture read
(`--charts-sample` and `--chart-text-sample` alone in P1, the edit
sample being P2's); the open record of the part's keep. P1 ships no
`set_chart_*` call and no `style`.

### P2: the style value and the in-place edits

The doc commit (the P1 verification rulings); `style`;
`ChartSeriesEdit`, `set_chart_title`, `set_chart_series`, the in-place
walk; `--chart-edit-sample`; the `chart-text` op of
`scripts/openpyxl-ops.py` with its test line; the catalogue entry and
limits; the openpyxl fixture through all three samples; the Excel-form
fixture through the edit sample; P1's corpus run rerun byte-identical;
the open record.

### P3: the probe, the size record, the API listing, the CHANGELOG

The doc commit (the P2 verification rulings); the carry; the probe's
options and edit as "The probe" says; the size log, recorded as
`records/size-m6p4-<utc-date>-<shortsha>.md`, with the `charts` cost,
its movement over part 3's 135784 judged against the estimate and the
25000 line, the `images` and `comments` costs' movements and the
core's growth each printed and judged (`specs/0038` ruling N's form);
the API listing rerun, recorded as
`records/api-review-m6p4-<utc-date>-<shortsha>.md`, its added-by
column taken from the first commit whose diff adds the member's
declaration line within its type (`specs/0040` ruling V; the script's
change named and tested on part 3's four misdated rows);
`CHANGELOG.md`'s "Added" gains one bullet citing this spec. The
negative log through the binary for every message reachable there.
The run log: the P1 and P2 runs rerun with the size log's release
binaries, byte-identical by sha256 (the openpyxl fixture by part 1's
P3 reading, `specs/0038` ruling P), recorded as
`records/m6p4-p3-<sample>-<utc-date>-<shortsha>.md` and `.txt`; the
open record of the part's keep.

### Close-out push (after P3)

As part 3's (`specs/0040` "Close-out push (after P3)" and ruling X):
the generator transforms part 3's close-out scripts, its numbered
changes continuing from where part 3's ended; the pre driver passes
HEAD (ruling X); the doc commit holds this spec and `ROADMAP.md`; the
base is part 3's close-out doc commit, named by its subject "M6 part 3
close-out: commit planning docs (the P3 verification and rulings)";
`PREV_RECORD_PATTERN` is `^M6 part 3 close-out record`, in the range
exactly once; the close-out doc commit's subject is "M6 part 4
close-out: commit planning docs (the P3 verification and rulings)";
`CARGO_TARGET_DIR` printed and checked as
`<home>/.cache/emend-build/m6p4-close/target`; no Excel-record check;
the scan's denylist form whole-word; the range holds no archive; the
record commit "M6 part 4 close-out record: the push, the CI run and
the scan" holding the three logs only; pre mode on a clean tree; one
independent check before the report. The push carries part 3's
close-out record commit and this part's commits, on planning's word
and the maintainer's authorisation typed in the implementing session.
The hashes are filled in at the briefing.

## Decisions for the maintainer

1. The title's text form. A (recommended): the properties in the rich
   text's `a:pPr` `defRPr` AND the run's `a:rPr` (Excel's form, 10 of
   the corpus's 13 titles; openpyxl reads either); B, the `defRPr`
   alone (openpyxl's own form, the run bare).
2. The chart style and colour parts. A (recommended): write `c:style`
   alone, as openpyxl does; no `style<N>.xml` or `colors<N>.xml`
   (Excel regenerates them; a literal pair would cost 8000 to 11500
   section bytes, the corpus's parts being 7.6 to 10.6 KB and 0.2 to
   0.9 KB); B, write a fixed pair of Excel's parts for every chart
   whose `style` is given, the cost added to the estimate.
3. The in-place edit's scope. A (recommended): the title and one
   series' name, categories and values by index, caches dropped; B,
   the title alone (`set_chart_series` deferred to a later part).
4. The reader. A (recommended): unchanged; the read-back relies on
   `title`, `style` and the series' references it reads today, the
   run log's own XML check reading the `txPr`s; B, the reader gains
   the text properties of the title (`Chart::title_text`) for the
   read-back, a reader change under `charts`.

## Out of scope

Text properties beyond size, bold, italic, one colour and one
typeface (underline, strike, kerning, spacing, scheme colours, East
Asian and complex-script faces, `bodyPr` rotation and anchoring,
per-run formatting, line breaks); the legend's per-entry text; a
`txPr` on a trendline label or a data table; the style and colour
parts (decision 2 A); editing a kept chart's kind, axes, labels,
formatting or anchor, or a kept series' options (markers, fills,
trendlines); adding or removing a series of a kept chart; caches
written or kept; a chartsheet's chart edited in place (the frame
resolution is the worksheet drawing's; a chartsheet's follows in a
later part); reading `txPr`. Any of them is a later milestone's part.
