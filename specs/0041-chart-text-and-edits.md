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

## Planning rulings during P1 (2026-10-21)

A. Four readings of the implementing session, ruled. (a) Check order: in
add_chart, after the axis options (specs/0040 ruling K) and before the
combined plot's rules, the chart's four text fields in declaration order
(title_text, axis_title_text, axis_label_text, legend_text), each its
applicability first (title_text with no title, axis text on a round kind
or with no axis title, legend_text with no legend: the option-kind
message), then its NewText fields in declaration order; a series'
NewLabels::text in the series options after its error bars, so a
combined plot's series take theirs with that plot's. (b) A colour above
0xFFFFFF is refused with "a chart option does not apply to its kind", as
a series', a marker's and a point's colour is; "a chart value is out of
its range" covers size, font and style alone, as "Errors" says. (c) The
font is checked by valid() with the limit 31: 1 to 31 BYTES, no
character below U+0020, U+FFFE or U+FFFF (the writer's rule for every
title and name; "Types"' "characters" is read as that unit), refused
with the range message, escaped as an attribute. (d) A title or axis
title whose NewText is given always writes both forms: a:pPr with the
defRPr and the run's a:rPr carrying the same attributes and children, no
lang; NewText::default() writes an empty defRPr and an empty rPr;
without a NewText the title keeps today's form, no pPr and no rPr.
axis_label_text goes on the primary category and value axes, a scatter's
and a bubble's two value axes included, never on a series axis or the
combined plot's secondary pair ("the secondary axis takes no text" is
the pair).

B. "Harness runs" is corrected on the fixtures' style reading. The
reader takes a root mc:AlternateContent's style from its first Choice,
c14:style, never the Fallback (specs/0034 ruling R), so Chart::style on
the Excel-form fixture is Some(102); and --charts-sample prints no style
value, only styles, the count of charts whose style is Some. So:
--charts-sample on the Excel-form fixture reads count 1, series 1,
styles 1 (not "style 2"); the run log's own ElementTree check of the
kept part, before and after the text sample, confirms the
mc:AlternateContent holds c14:style 102 with the fallback c:style 2, and
the kept part and both style parts byte for byte; no claim of the value
goes through the reader. The fixture keeps the corpus's form (c14:style
102, the fallback 2). Likewise in P2, --charts-sample on the openpyxl
fixture reads count 1, styles 1, the run log's own check reading the
plain c:style val 26. The dev figures told (keep 2173 pairs, 52 new from
ten new tests, none changed; chart_write.rs tests 97 to 104,
null_rewrite.rs 150 to 153, two base tests edited by the one line text:
None) are judged at the report.

C. Finding S1 (the final scan, try 1, final.log 41b50740: total hits 1,
form f_p6, RESULT: FAIL, structural 0): check 1's answer quotes the
absolute path of a file it wrote in the implementing session's
scratchpad, a path holding the session id, which P6 catches; nothing
else in it hits. Ruled: the committed independent-check-1.md holds the
answer with that ONE path replaced by "<scratch>/spec41.md" (the logs'
placeholder rule), a named scrub that alters no judgment; the original
stays outside the tree, its sha256 f067043e stated with the scrubbed
file's in the README and the report; the final scan reruns as a named
rerun, try 1 kept under superseded/scan-final-try1. No check 2. Carried
as a rule: the check's brief says to name no absolute path (the
placeholders <repo>, <scratch>, <home>, <phase>), and a check answer is
scanned by the P6 form as the logs are. The check's 1a (title_text's doc
cites a section "The call" that does not exist; it is "Types") and 1b
(add_chart's Errors prose places the labels' text as if in order) are
comment slips carried to P2's first code commit, as specs/0040 ruling Q
was carried; 1c is answered (plot_applies refuses an axis title on a
round kind before any text check).

D. Finding S2 (the final scan, try 2, final.log 5a7c9251: total hits 0,
structural checks failed 1, RESULT: FAIL): the invocation gave
RERUN_FILES the live script beside the superseded try-1 log, and change
AP requires every named file under <phase>/superseded. Ruling C's scrub
is confirmed by try 2 (hits 0; independent-check-1.md dd38d6ad,
differing from the original f067043e on its line 77 alone). Fix accepted
as named (S2-a): RERUN_FILES names the two superseded final.logs (try 1,
try 2) alone, RERUN_NOTE names both attempts and their causes, the
scripts unchanged; try 2 kept under superseded/scan-final-try2; the
third run is the scan. Ruling F's check is met by the logs alone (each
prints the script byte for byte); the live script is never a rerun file.

## Planning verification of P1 (2026-10-21)

Verified from primaries: the chain 5d1c302, 245831d (doc, its three
blobs at the hashes planning left), 4d2cdc3 (core), f838d65 (tool, the
phase commit), 1156042 (record, 11 files), all local, the tree clean but
for this working spec, origin/main still 69f4c89; the eleven files at
the report's hashes (gate be22c684, test 6ddf72ed, negative 0e0f0940,
run 48522aa8, scan 05ad73ed, README d13570b3, the check dd38d6ad; the
charttext records 490575fe and b0ea3721, the open records 5a7de6d8 and
9536cbcd; the report 68546236); the four logs ending "checks all as
expected: yes"; the scan (try 3) "RESULT: PASS", total hits 0,
structural 0, its RERUN_FILES the two superseded final.logs alone and
its RERUN_NOTE naming both attempts (rulings C and D); the run log
taking this working spec at 8a0dc59a (rulings A and B), the scan at
bc81842f (A to D). The run log: 27 of 27 ok on both targets, lo ok 27
and lo_in ok 27, the host and i686 outputs byte for byte the same, 27;
the Excel-form fixture: --charts-sample reads charts 1, series 1, styles
1 (ruling B), --chart-text-sample charts_before 1, charts_after 4,
series 4, rows 6, texts 6, styles 0, read_back 3, the outputs the same
on both targets; the kept chart part, its rels and both style parts byte
for byte the fixture's, the first Choice c14:style 102 and the Fallback
c:style 2, each txPr in its place by the schema's sequence; the keep
2173 pairs, 52 new, exactly this phase's tests' and all ok and ok, none
of the 2121 moved. The test log: 1396 on both targets, the feature sets
515, 673, 715, 718, 776 and 1155. The gate: the charts probe 1112212
section bytes, +4252 over part 3's 1107960, told and judged nowhere
here; part 3's four target roots pruned after du -sb. The negative log:
14 cases refused, N1 to N14. The independent check's 77 lines verbatim
in the report (its line 77 the scrubbed one, ruling C), yes on all three
questions; 1a and 1b carried to P2's first code commit (the "The call"
citation is at chart_write.rs line 138 at f838d65); 1c answered. The
README and the report state the scrub, both hashes, S1, S2 and the gate
fork's scratchpad doc builds (a departure from its brief, touching
neither the phase target nor the repository). Privacy: 4 commits, 2139
blobs, 0 findings, PASS. The phase's two independent checks: check 1 and
planning's privacy scan. P1 is DONE.

## Planning rulings during P2 (2026-10-21)

E. Nine readings of the edit calls, ruled. (Q1) Agreed, and "The chart
part"'s "Order of checks at the call" is amended: at the call nothing is
recorded on an error; set_chart_title checks the title by the text rule,
set_chart_series the edit's references in field order (name, categories,
values) by the reference rule; the part and the index are checked at the
write as remove_chart's are, per part in record order: the twice,
removal and addition refusals; remove_chart's part refusals; the index
past the frame list ("no chart at that position"); the frame's chart
part ("a chart part cannot be read": no part, or not chartSpace); the
series index ("no series at that position"); the walk's own failures on
a malformed part, "a chart part cannot be read". (Q2) Agreed:
set_chart_title twice on one index, or set_chart_series twice on one
(index, series), is "no chart at that position"; a title edit with
series edits on one chart, and edits of distinct series, go in one walk.
(Q3) Agreed: a part this write adds an image or a chart to refuses the
edit part-wide; an index this write removes refuses it; a removal of
another index on the same part is allowed, the indexes naming the
input's list. (Q4) Agreed: on a chartsheet part the edit calls are
refused with the worksheet-root message the cell calls use
(NOT_WORKSHEET), not remove_chart's. (Q5) Amended: no kind takes no
categories, so "Applicability"'s refusal of categories is struck; the
walk names cat and val, or xVal and yVal under scatterChart and
bubbleChart, by the ser's parent; a parent outside the plot elements the
writer knows is "a chart part cannot be read". (Q6) Agreed: the
reference form found (numRef or strRef) is kept with its f replaced and
its cache dropped; the writer's form (cat strRef; xVal, val, yVal
numRef) is used where the element is absent or is a literal or a
multiLvlStrRef. (Q7) Amended toward keeping bytes: in a kept rich title
the bodyPr, lstStyle, the first paragraph's pPr and its first run's rPr
are kept, that run's t alone replaced, further runs and paragraphs
dropped; a tx holding a strRef, or a rich with no run, is replaced by
the writer's rich form; the title's other children (layout, overlay,
spPr, txPr, extLst) are kept byte for byte; a title with no tx gets one
inserted as its first child; no overlay is added. The Excel-form
fixture's check reads "the title run's t alone replaced" accordingly.
(Q8) Agreed: None with no title writes autoTitleDeleted 1 as c:chart's
first child where absent; Some with no title inserts the title as
c:chart's first child, an autoTitleDeleted 1 becoming 0 and an absent
one staying absent. (Q9) Agreed: the chart elements written take the
prefix the root binds to the chart namespace ("c:", none or another);
the rich text takes the DrawingML prefix in scope at the root, and where
none is bound the inserted rich carries its own xmlns:a. The edit
sample's "FIRST kept chart" is index 0 of the first worksheet's
Package::charts list read before the first write, where non-empty; its
new title the constant "emend kept" (ruling H); the read-back through
the reader (the title, the name's reference, the two ranges).

F. (Q10) style, NewChart's last field, is checked in declaration order:
its range (1 to 48, else "a chart value is out of its range") right
after the four texts' checks (ruling A a) and before the combined plot's
rules; one test pins it (a bad legend_text with a bad style takes the
legend's message).

G. (Q11) No P2 sample wrote a style, so the openpyxl comparison had no
emend style val to set beside openpyxl's 26. Ruled (c): the edit
sample's added chart takes style 26 (a one-line tool change, a new tool
commit after 1adc74d, the phase commit moving to it; the generators take
it); the comparison takes emend's c:style from the edit sample's added
chart part and openpyxl's from the fixture, the same constant, equality
required, each printed with its preceding sibling as found:
roundedCorners for emend, none for openpyxl, whose style is the root's
first child (the implementing session's dev run of the op; planning's
"roundedCorners on both" was wrong), the difference named and not
judged, as the catAx's tickLblPos is; P1's text sample is unchanged and
its rerun stays byte-identical; the edit sample's line is unchanged (the
reader reads the style, read_back stays 3). The openpyxl fixture's
charts line reading labels 0 is a reading fact named in the log:
openpyxl puts dLbls on the plot, which the reader's per-series flag does
not count. The run fork's hand figures (the Excel-form fixture through
the edit sample: charts 2, series 1, rows 6, read_back 3, kept_edited 1,
the title run's t alone replaced; the openpyxl fixture: styles 1,
charts_after 4, kept_edited 1) are judged at the report.

H. (Check 1 of P2, point 1a) The edit sample titles the kept chart
"emend kept", its added chart "emend edit" at the first write and "emend
edited" at the second, three distinct constants the tool and every log
agree on; ruling E had named "emend edit" for the kept chart. Ruled (a):
ruling E is amended in place to "emend kept"; no code or log changes;
the README names the departure. Points 1b (the twice, removal and
addition refusals across all parts first, then each part's remaining
checks, no test pinning the order across two parts), 1c (an inserted
whole title carries overlay 0, the writer's form; ruling E's "no overlay
is added" concerns a kept title gaining a tx) and 1d (the enclosing
element's prefix, the root's unless rebound) are accepted readings; 2a
(the entry identity enforced by the tool's exit 2, not printed) is
accepted, the run log naming it; 3a and 3b are wording fixes in the
report's "after the check" section. 3c: the charts probe at f229553 is
1136996 section bytes, 29036 over part 3's 1107960, above the 25000
finding line before P3 adds anything; stated in the report, judged by
P3's size log, where a movement above 25000 is a finding recorded and
not fixed ("Size and packaging"). The check answer holds two U+2026 and
no U+2014 or path form: committed verbatim, the scan the arbiter; if the
scan refuses non-ASCII, the two are replaced by "..." as a named scrub
with both hashes stated, as ruling C's was.

I. The final scan of P2. Try 1 (final.log 3d47f32e): total hits 6, form
f_nonascii, the check answer's two U+2026 (lines 43 and 55), RESULT:
FAIL, structural 0; ruling H's scrub applied as change S1-a, each U+2026
replaced by "...", the original 3aaf0a7d outside the tree and the
committed file 26f35b4c differing on those two lines alone, finding S1
in the README. Try 2 (final.log 29ad1ae6): RESULT: PASS, hits 0,
structural 0, but its printed RERUN_NOTE holds the home directory on one
line, an expanded absolute path given where P1 gave the literal <phase>;
the scan does not scan its own output; not committed, kept as
superseded/scan-final-try2 (finding S2). Ruled: a third run, the scripts
unchanged (change S2-a); RERUN_NOTE uses the literal placeholders and
names both attempts and their causes; RERUN_FILES names the two
superseded final.logs, the script printing them scrubbed; before
installing, the log is grepped for the home directory (count 0). Carried
to P3's scan generator: a self-check that no line of the scan's own
output holds the home directory, the RERUN_NOTE line included.

## Planning verification of P2 (2026-10-22)

Verified from primaries: the chain 1156042, 8faf930 (doc, blobs fd01e97e
and 897ae22b), 075abca (core, 1a and 1b carried), 5548bcc (tool),
1adc74d (op), f229553 (the edit sample's style 26, ruling G, the phase
commit), 6203ddc (record, 13 files), all local, 11 commits over
origin/main 69f4c89, the tree clean but for this working spec; the
thirteen files and the report at the hashes reported (gate 4ecd52bc,
test bef63747, negative 647e3945, run be00488e, scan 66e01fc6, README
0e111f83, the check 26f35b4c; the chartedit records 7b10b177 and
83c9ccde, charttext 095f5578 and c16fdea1, open 5637fa0c and 920f5fdc;
the report b6c34f52); the four logs ending "checks all as expected:
yes"; the scan (try 3) "RESULT: PASS", hits 0, structural 0, RERUN_FILES
the two superseded final.logs, RERUN_NOTE with the placeholders, no line
of the scan, the README or the check holding the home directory (rulings
H and I); the run log taking this working spec at d59ed185 (A to G), the
scan at bcfe65d2 (A to I). The run log: the edit sample 27 of 27 ok on
both targets, lo ok 27 and lo_in ok 27, the outputs byte for byte the
same across targets, read_back 81 of 81, kept_edited 4, every added
chart part holding one c:style 26 after roundedCorners (27) and each
kept part keeping the input's style (4); the text sample rerun
byte-identical to P1's run by sha256, 27 of 27 per target; the
Excel-form fixture through the edit sample charts 2, series 1, rows 6,
read_back 3, kept_edited 1, the fixture's title run's t alone replaced
byte for byte, its four txPr, its c14:style, its rels and both style
parts kept; the openpyxl fixture through the three samples (styles 1 and
labels 0, a reading fact named; charts_after 4; kept_edited 1, rows 100)
and the field comparison seven of seven the same form, style 26 equal
with the preceding siblings named (none; roundedCorners), sz, b and
latin compared, the two txPr placed by the schema, openpyxl's empty
runs, bare endParaRPr and bare title run named and not compared; the
keep 2215 pairs, 42 new, exactly this phase's tests' and all ok and ok,
none moved. The test log: 1414 on both targets, the sets 515, 688, 730,
733, 791 and 1170; the ops test PASS. The gate: the charts probe 1136996
section bytes, 29036 over part 3's 1107960, above the 25000 line before
P3, stated in the report and judged by P3's size log (ruling H). The
negative log: 15 cases refused, N1 to N15. The independent check's 95
lines verbatim in the report in the scrubbed form (ruling H), yes on all
three questions with one disagreement (1a, ruling H) and the doubts
accepted as readings; the README names rulings E to I, S1 with both
hashes, S2, the title constants, the forks' departures (block O restored
from 65703ee's print; the scan fork's one rm against its move-only
brief) and the prune. The records are dated 2026-10-22 (the logged run
crossed midnight UTC) at f229553; accepted. Privacy: 6 commits, 2158
blobs, 0 findings, PASS. The phase's two independent checks: check 1 and
planning's privacy scan. P2 is DONE.

## Planning rulings during P3 (2026-10-22)

J. (Q12) "The probe" said the edit falls on the first set's chart, but
the probe removes that chart before the second add (add, reopen,
remove_chart, reopen, add the second set, reopen), so after the second
add the first worksheet holds the second set's chart alone, at index 0,
and the first set survives only as the chartsheet's chart, which ruling
E (Q4) refuses to edit. Ruled (a): after the second add's reopen the
probe calls set_chart_title(first, 0, Some("probe edited")) and
set_chart_series(first, 0, 0, ...) with a Text name and the two constant
references, one call each on the worksheet's one chart, the second
set's; then writes, reopens and prints as today; the line count and
every printed figure unchanged; the walk meets all sixteen kinds' parts
across the kind index. The header's description reads "then the second
set's chart, the sheet's one chart, edited in place". Option (b),
re-adding the first set's chart, is refused (it moves the printed charts
figure). The second set's options as read: title_text 1400 bold;
axis_title_text and axis_label_text on every kind but the four round
ones, the surfaces and the scatter, bubble, radar and stock kinds
included since they write axes (a kind the writer refuses is a question
back to planning); legend_text on every kind; NewLabels::text on every
series that takes labels, not the surfaces'; style Some(26) on every
kind; one NewText constant per option: the title 1400 bold, the axis
titles 1000 italic, the axis labels 900, the labels 800 with colour
1F77B4, the legend 900 Arial. The doc commit 11e1715 (parent 6203ddc,
blobs edaeea43 and 9b72d75f) is verified; the carry is skipped, nothing
carried from P2's check and no doc slip found.

K. The charts probe at 0217d1f is 1138352 section bytes on i686 (the
implementing session's dev figure), 30392 over part 3's 1107960 and 1356
over P2's gate figure 1136996; P2's own movement, 24784 over P1's
1112212, is the in-place walk linked in through write_to before the
probe called it. A movement above 25000 is a finding by "Size and
packaging": the size log records it, recorded and not fixed, the report
names it, no code changes in this part. Its cause is planning's
estimate: the walk was priced at 5000 to 9000 and measures about 24800;
the text properties and the style priced 5000 to 8000 measure about 5600
(P1's 4252 and the probe's 1356). Carried: the next spec that adds an
XML walk prices it at this part's measured cost, not by forms. The probe
commit 0217d1f (the probe's source alone, ruling J's edit, the one
description) and the CHANGELOG commit 1cc0de4 (one bullet of five lines
citing this spec, the phase commit) are verified; the writer refused no
text option on any kind, so ruling J's reading stands.

L. (Q13) Ruling V's added-by fix (api-listing.py bb1fba22, M3 part 7's
59f4043c with the one labelled change AV1: a member row's added-by is
the oldest commit of the file's history whose copy, read by the
program's own parser, holds a member of that kind and name in that
container; root items keep the -S rule) re-dates 42 rows at part 3's
phase commit 0707236, not the four ruling V named, the other 38 the same
misdating (planning checked Area to d03ed7b and Scope to 5fc1b3d by git
log -S; the equal lines at 6c27167 and a9f2cad are CellRef's and
Stream's). Ruled, the recommended form: the P3 record compares rows on
the key without added (module, kind, name, container, cfg) and names
separately, under "added-by re-dated by ruling V (change AV1)", every
row whose added-by differs from the old program's at the same phase
commit; the test log runs both programs at the phase commit and requires
the two listings to differ in added and added_subject alone and the four
named rows to read 12f1314, 12f1314, 3c1f1f3 and 3c1f1f3; it shows part
3's 42 by the same run at 0707236. The alternative, the old key with the
moves as gone and new pairs, is refused as noise. The new program is the
listing's from this part on; the record names its hash and AV1.

M. (Q14) The size log's cause check is widened (change QW): a crate
commit's code lines in the core's sources may be comment lines,
re-export-list lines, or lines gated under #[cfg(feature = "charts")],
the gating read by a reader printed in the log (gated.py 34eb101a) with
fixture controls refusing an ungated line. P2's core commit 075abca has
19 such lines in package.rs (the chart_edit module, its re-export, the
recorded edits and their arms, the write's call), all gated (planning
counted 19 code lines and the gates in the diff); P1's 4d2cdc3 adds
re-export lines alone. Ruled (a): QW is the check, consistent with the
measured core growth of 0; option (b), a finding against part 3's
narrower form, is refused. The finding under ruling K is built from the
committed gate figures, P1 4252, P2 24784 and P3 1356 summing to 30392,
with the check that the steps sum to the cost. The scan's self-check
(KL) reads its own standard output, so the scan writes to a file, never
a pipe; the negative log's NC54 control refuses the base's set less
NO_SERIES_AT.

N. (Check 1 of P3, independent-check-1.md eb21e98d: yes to all three
questions, fourteen points.) Ruled, with no code, log or record change
and no rerun. Points 1, 4 and 6 are stale wording: the gate, negative
and run logs call c3a19a41 "rulings J to L" where the file holds J to
M (gate 8 lines, negative 9, run 2, counted by grep); the gate heading
at line 9336 says "two added style lines alone (Q80)" above the 81/19
check (Q81); size.log repeats the heading at lines 10648 and 10650.
The checks under them hold; the README and the report name them.
Point 2: api-listing.py bb1fba22 and gated.py 34eb101a live under
<phase>/gen as M3 part 7's program did, pinned by the committed
test.log (AV1 printed as a diff -u against part 7's program) and
size.log (gated.py printed whole); ruling L's "from this part on"
means that pinned program. Committing it is a scripts/ change outside
this part. Point 3: the diff and 0217d1f's message carry which series
take label text and that round is the four round kinds; no log prints
them, and the sixteen index runs exiting 0 show the writer refused
nothing. Point 5: the spec's "P1 and P2 runs rerun" is met through
P2's two run directories, P2's text run being P1's byte-identical
rerun (README, departures). Point 7: the open text equals P2's
(920f5fdc) because 0 pairs changed. Point 8: scan.log comes last by
design. Points 9 to 14 are the report's wording: "no code changes" is
said in ruling K's sense, no change made to fix the finding; the
numstat note says the gate prints 81/19, 5/0 and the doc commit's 0
and 1 removed, the spec's +161 and ROADMAP's +1 from the diff alone;
"line count and figures unchanged" cites size.log 12603 and 12650;
"stay local" is the session's statement; the two trimmed quotes are
noted. These corrections go in a section after the verbatim answer.
The checker's one command that redirected to /dev/stdin blocked, was
killed by the session, wrote nothing; the README names it. The
"/dev/stdin" in the answer is no path form the scan reads.

## Planning verification of P3 (2026-10-22)

Verified from primaries after the record commit 5d5eee0 landed on
1cc0de4: the six logs, the README, the check answer, the two records
and the three record pairs, each read in the tree at the record commit;
every hash below is sha256sum's own print.

- The phase: 11e1715 (doc: this spec edaeea43, ROADMAP 9b72d75f),
  0217d1f (the probe), 1cc0de4 (the CHANGELOG bullet, five lines under
  "### Added", citing this spec). No code commit, no carry, no prune.
  The record 5d5eee0 holds 16 files, parent 1cc0de4, ASCII message,
  the Co-Authored-By line alone.
- The logs (records/logs/m6p4-p3): gate f358db59, test 5f95b843,
  negative 53ffae05, size 72233d1f, run d5ecdcde; each ends "<log>
  checks all as expected: yes"; each names PLANNING_SPEC c3a19a41
  (this spec's working copy with rulings J to M). The scan 9d36f95a:
  RESULT: PASS, total hits 0, structural 0, PLANNING_SPEC 029cb50c
  (rulings J to N), self-check KL yes. No line of any committed file
  holds the home directory (the four "/home/" lines in the run, scan
  and size logs are the scripts' own pattern text and a planted
  control); no non-ASCII byte. The README 952c90d0.
- The size record (records/size-m6p4-2026-10-22-1cc0de4.md, 8f45ada5):
  charts 166176 over the core probe (1138352 over 972176), a move of
  30392 over part 3's 135784 (probe 1107960), above the finding line of
  25000: a FINDING, recorded and not fixed, its cause planning's
  estimate (ruling K). The steps as the record reads them from the
  committed gate logs: P1 4252 (1107960 to 1112212), P2 24784 (1112212
  to 1136996), the probe commit 1356 (1136996 to 1138352). Core,
  images and comments moved 0. The budget 438960, headroom 19792,
  unchanged.
- The API record (records/api-review-m6p4-2026-10-22-1cc0de4.md,
  8e9b3f6e): 834 rows, compared by key without added-by (ruling L);
  1 gone (the chart_write re-export whose text grew) and 20 new (the
  re-export, NewText and its five fields, NewChart's five fields,
  NewLabels.text, ChartSeriesEdit and its three fields, the two Package
  methods); 44 rows re-dated by the AV1 program at 1cc0de4 and part 3's
  42 at 0707236, ruling V's four among them (NewChart.category_axis
  and value_axis 3c1f1f3, NewTrendline.name and NewErrorBars.value
  12f1314); Eq dropped 0.
- The gate log: 1417 test names at the base and the phase commit, 155
  distinct messages, nine PASS lines; the test log: 1414 passed on both
  targets, the six feature runs P2's figures.
- The negative log: 15 cases refused (N1 to N15); NO_SERIES_AT's
  message once in the catalogue at the phase commit and at the base,
  quoted in this spec and in no earlier chart spec, covered by two
  tests named in the log, and unreachable through the binary as P2's
  log stated.
- The run log: with the size log's four release binaries (4 of 4);
  charttext and chartedit rerun on both labels byte-identical to P2's
  runs (4 of 4, 27 of 27 each); the Excel-form fixture's three samples
  byte-identical to P2's (6 of 6); the openpyxl fixture written fresh by
  the pinned image, its six lines P2's (6 of 6; specs/0038 ruling P)
  and P2's own fixture rerun byte-identical (6 of 6); the keep 2215
  pairs, 0 new, 0 gone, 0 changed; the open record's text byte-identical
  to P2's (920f5fdc), lo_in 2203/12/0, lo 2204/11/0, the twelve and
  eleven P2's.
- The record pairs at 1cc0de4: charttext 6d965562/9f512550, chartedit
  c6aebb64/d35a9b4c, open e4bbd4e6/920f5fdc; 27 of 27 on each label.
- The check (eb21e98d): yes to all three questions, fourteen points,
  ruled by ruling N; in the record and the report verbatim, no path
  form, no non-ASCII byte. The report (b0c90573, outside the tree)
  carries the corrections of points 9 to 14 after the answer.
- Privacy scan 6203ddc..5d5eee0: 4 commits, 2162 blobs, 0 findings,
  PASS.

P3 is DONE. Part 4's work is complete; the close-out follows.

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
