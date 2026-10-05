# 0027: Milestone 3 part 5, reading back as typed values

Status: APPROVED by the maintainer 2026-10-05 ("A on all
(Recommended)", chosen in the planning session's question:
decisions 1 to 6 as planning recommended, 7 recorded). Sent to the
implementing session the same day.

Context: `CLAUDE.md` ("What emend is": like openpyxl, which reads
every one of these back; "Dependencies and licences": anything
beyond the core behind a Cargo feature off by default; "32-bit and
sizes": stream where the format allows it), `ROADMAP.md`
("Milestone 3", the outline approved 2026-10-01, item 5: fonts,
fills, borders, alignment, number formats, widths, heights,
panes, merges, hyperlinks, protection, the read side Milestone 2
left out part by part; feature `read`; 30000 to 45000),
`specs/0010-reading-cells.md` (`Cells`, `Cell::style`, the shared
strings table held whole), `specs/0012-styles.md` and
`specs/0017-fuller-styles.md` (`Styles`, `Xf`, `NumberFormat`;
`Font`, `Fill`, `Border`, `Alignment`, `Protection` and their
writers), `specs/0018-rows-and-columns.md` (the row and column
formats, panes, merges), `specs/0019-names-links-properties-
protection.md` (`LinkTarget`, `SheetProtection`), `specs/0024-
rich-text.md` ("The read": the `<rPr>` reader given the name
child's local name, "part 5 may give it `name`"), `specs/0025-
sheet-and-print-settings.md` and `specs/0026-grouping-and-line-
styles.md` (the settings and the line attributes written, their
typed read left to this part), `specs/0023-scale.md` (the decision
record's finding 2, the shared strings table's hold, carried
here; the close-out rulings of `specs/0026`, carried here),
`specs/0001-bootstrap.md` ("Rules for every phase"). ECMA-376
Part 1, 18.8 (styles: `font`, `fill`, `border`, `xf`, `alignment`,
`protection`, `numFmt`), 18.3.1.13 (`col`), 18.3.1.73 (`row`),
18.3.1.66 (`pane`), 18.3.1.55 (`mergeCell`), 18.3.1.47
(`hyperlink`), 18.3.1.85 (`sheetProtection`), 18.4.9 (`sst`) are
cited where the design leans on them; no text is copied from
them. openpyxl's `cell.font`, `fill`, `border`, `alignment`,
`number_format`, `protection`, `column_dimensions`,
`row_dimensions`, `freeze_panes`, `merged_cells`, `cell.hyperlink`,
`ws.protection`, `sheet_view`, `page_setup`, `page_margins`,
`print_options`, `HeaderFooter`, `sheet_properties` are the
behavioural reference, read only.

## Goal

What a cell looks like, as the types the writers take: a cell's
format resolved through `cellXfs` into a font, a fill, a border,
an alignment, a protection and a number format; a worksheet's
column and row formats, including part 4's levels, flags and
styles; its frozen pane, merges, hyperlinks, protection and every
setting part 3 writes, read in one pass; and the shared strings
table's hold decided (finding 2 of the scale record). Every typed
read gives a value the matching writer accepts, and writing it
back reproduces the element, which the samples prove on the
corpus. Everything behind the Cargo feature `read`, off by
default. Reads only; no writer changes.

## Size and packaging (decision 1, before any code)

The budget figure after part 4 is 436928 section bytes against
the ceiling of 458752 (`records/size-m3p4-2026-10-05-ef392d4.md`):
21824 of headroom, held for fixes. The outline's estimate for
this part is 30000 to 45000. Planning's, priced from parts 2 to 4
(`richtext`'s `<rPr>` reader inside 12640; `sheetprops`'s seven
merged elements 28268; `rowcol`'s growth 22888 for four fields
and a coalescing), is 35000 to 50000 for `read`: six element
readers over the styles part, a one-pass sheet reader of twelve
elements, two line-format readers and an item iterator; a reader
costs about what a writer does. The core's growth is what the
feature gates add, expected near 0; above 2000 section bytes, or
a feature cost above 55000, planning is told in the report.

- A (recommended): one Cargo feature `read`, off by default,
  declared beside `sheetprops` with no dependency of its own, the
  typed reads of parts 3 and 4's settings under it too (reading
  is the capability, whichever part wrote the element); one
  probe, `probe-emend-read-write-xml`, over the format probe. 26
  probes.
- B: each feature grows its own read calls (`sheetprops` reads
  its elements, `rowcol` its attributes, `metadata` its links and
  protection) and `read` keeps the styles alone: five features
  grow, five figures move, and a consumer reading a tab colour
  carries the writer.

## Facts planning checked (2026-10-05)

- The corpus, counted by planning with Python's `zipfile` and
  regular expressions over every regular file of the directory
  (27 workbooks, 95 worksheet parts), names and counts only. The
  styles parts hold 1391 `font` (473 in one workbook), 666 `fill`
  (172), 471 `border` (123), 1451 `xf` under `cellXfs` (153), 256
  `numFmt` (47), 1391 `dxf` and 221 `cellStyle`. Font children:
  `name` 1144, `family` 1065, `sz` 1000, `scheme` 964, `color`
  950, `strike` 638, `vertAlign`, `outline` and `shadow` 633 each,
  `u` 567, `b` 412, `i` 271, `condense` and `extend` 174 each,
  `charset` 39. Fills: `patternFill` 660 (`bgColor` 600, `fgColor`
  326), `gradientFill` 6 (with `stop` and `color`). Border sides:
  786 with `style`; 26 workbooks carry a `diagonalUp` or
  `diagonalDown`. `xf` attributes: the four ids on all 1451,
  `xfId` 992, `applyFont` 1214, `applyAlignment` 1177, `applyBorder`
  922, `applyProtection` 663, `applyFill` 621, `applyNumberFormat`
  271, `pivotButton` 1. Alignment: `vertical` 1304, `horizontal`
  1250, `wrapText` 939, `indent` 928, `shrinkToFit` and
  `textRotation` 626 each, `readingOrder` 443, `justifyLastLine`
  429, `relativeIndent` 11. Protection: `locked` 259, `hidden`
  203. Sheets: 8 workbooks carry a `pane`, all frozen, none split;
  18 carry merges; 9 carry hyperlinks (`ref` 265, `display` 264,
  `r:id` 198, `location` 67, `tooltip` 61, `xr:uid` 65); 5 carry
  `sheetProtection`.
- `Styles` (`styles.rs`) reads `<numFmts>` into `NumFmt`s and
  `<cellXfs>` into `Xf`s (the four ids, `xfId`, the element's raw
  bytes with the spans of its `alignment` and `protection`), and
  counts `<fonts>`, `<fills>` and `<borders>`; it is parsed once
  and lent (`Package::styles`). `Cell::style` is the cell's `s`.
- The writers' types (`styles/elements.rs`): `Font` (ten fields,
  not `charset`, `outline`, `shadow`, `condense`, `extend`),
  `Fill` (a pattern with two colours, no gradient), `Border`
  (five `Side`s and the two diagonal flags), `Alignment` (six
  fields, not `justifyLastLine`, `readingOrder`, `relativeIndent`),
  `Protection` (two), `NumberFormat` (built-in id or code), each
  with a `compose` writer; `Color` with its four forms. Part 2's
  `read_properties` (`cells.rs`) reads font children into a `Font`
  given the name child's local name, `charset` and the other four
  read past.
- `set_hyperlink` takes `LinkTarget` (a URL through a relationship
  or a location in the workbook) with an optional tooltip;
  `protect_sheet` takes `SheetProtection` (fourteen flags and an
  optional password); `freeze_panes` writes a `<pane>` with
  `state="frozen"` and a `topLeftCell`.
- The shared strings table: `Package::shared_strings` parses the
  part on every call and holds every item (46.2 bytes per item on
  i686 over tier d's 4800000 items, 216572 kB; the scale record's
  finding 2); `--cells` resolves against the held table.
- The feature costs after part 4 (section bytes over the core
  probe): `dates` 4840, `richtext` 12640, `sheetprops` 28268,
  `images` 40880, `tables` 46536, `rules` 48452, `metadata` 50036,
  `comments` 57224, `rowcol` 58136.
- openpyxl, planning's reading of 3.1, read only: every cell
  carries `font`, `fill`, `border`, `alignment`, `protection` and
  `number_format` resolved from the styles part on load; a
  gradient fill reads as `GradientFill`; `column_dimensions` and
  `row_dimensions` carry width or height, hidden, outline level,
  collapsed and a style; `freeze_panes` is the pane's
  `topLeftCell`; `merged_cells.ranges`; `cell.hyperlink` with
  `target`, `location`, `display`, `tooltip`; `ws.protection`; the
  sheet view, page setup, margins, print options, header and
  footer and sheet properties as objects; the shared strings are
  read whole into a list.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. Under decision 1 A,
everything below sits behind the Cargo feature `read`, off by
default: new calls on `Package`, new types for what the writers
have none, and the typed fields `Styles` gains, all gated; the
`<rPr>` reader of part 2 becomes the `<font>` reader with the name
child `name`, one definition under `any(richtext, read)`. The core
builds, tests and links without it; the tool turns it on. Nothing
existing changes signature.

### Cell formats

`Styles` gains, with the feature, `fonts: Vec<Font>`, `fills:
Vec<Option<Fill>>`, `borders: Vec<Border>` and, per `Xf`, its
`alignment: Option<Alignment>` and `protection: Option<Protection>`
read typed, parsed in the same one read of the part; `Package::
cell_style(index: u32) -> Result<CellStyle, Error>` resolves a
cell's `s` through `cell_xfs[index]` into `CellStyle { number_
format: NumberFormat, font: Font, fill: Option<Fill>, border:
Border, alignment: Option<Alignment>, protection: Option<
Protection> }`, derived `Clone`, `Debug`, `PartialEq`:

- `number_format`: `Code` when `numFmtId` names a `numFmt` of the
  part, else `BuiltIn(id)`; `numFmtId` absent reads as
  `BuiltIn(0)`.
- `font`: the `<font>` at `fontId` through the one reader; the
  five children `Font` has no field for read past, as part 2
  reads them. `fontId` absent reads as the part's font 0.
- `fill`: the `<fill>` at `fillId` when it is a `patternFill`
  (`patternType` into `Pattern`, `fgColor` and `bgColor` into
  `Color`; a `patternFill` with no `patternType` reads as
  `Pattern::None`), `None` for a `gradientFill`, which the type
  cannot hold (decision 3); the catalogue says so.
- `border`: the five sides each `Some(Side)` when the element
  carries a `style` other than `none` (P1 verification ruling 2),
  its `color` into `Color`; `diagonalUp` and `diagonalDown` as
  flags.
- `alignment` and `protection`: the `xf`'s children when present,
  each field from its attribute, the three alignment attributes
  the type has no field for read past.
- Every `Color` through the four forms as part 2 reads one; a
  value that does not parse, an index past a list, or a child
  outside the main namespace: refused, "a style cannot be read".
  Every value the reader gives is one `add_style` accepts, which
  the round-trip sample proves.

`Cell` gains nothing: a caller resolves `cell.style` by
`cell_style` when it wants the picture, as openpyxl's lazy
readers do, so a cell pass costs what it costs today.

### Columns and rows

- `Package::columns(part) -> Result<Vec<ColumnFormat>, Error>`:
  one `ColumnFormat { first: u32, last: u32, width: Option<f64>,
  custom_width: bool, hidden: bool, style: Option<u32>,
  outline_level: u8, collapsed: bool }` per `<col>`, in part
  order, bounded by `<cols>` as the writers' hold is.
- `Package::rows(part) -> Result<Rows<'_, R>, Error>`: a streaming
  iterator yielding `RowFormat { row: u32, height: Option<f64>,
  custom_height: bool, hidden: bool, style: Option<u32>,
  custom_format: bool, outline_level: u8, collapsed: bool }` per
  `<row>` of `sheetData`, one at a time, never holding the sheet
  (decision 4); it borrows the package as `Cells` does. A `<row>`
  without `r` is refused with part 4's message, as `Cells`
  refuses it.

### The sheet in one pass

`Package::sheet_settings(part) -> Result<SheetSettings, Error>`
reads the worksheet part once, streaming, holding each small
element it reads to its end tag (decision 5), into `SheetSettings
{ tab_color: Option<Color>, outline_summary: (Option<bool>,
Option<bool>), fit_to_page: Option<bool>, view: SheetView, freeze:
Option<CellRef>, print_options: PrintOptions, margins: Option<
PageMargins>, page_setup: PageSetup, header_footer: HeaderFooter,
auto_filter: Option<Area>, merges: Vec<Area>, hyperlinks:
Vec<HyperlinkEntry>, protection: Option<SheetProtection>,
outline_max: (u8, u8) }`, derived `Clone`, `Debug`, `PartialEq`:

- `tab_color`, `outline_summary`, `fit_to_page` from `<sheetPr>`'s
  children; `view` from the first `<sheetView>`'s attributes into
  part 3's `SheetView` (each `Some` when the attribute is
  present); `freeze` the first `<sheetView>`'s `<pane>`'s
  `topLeftCell` when `state` is `frozen`, `None` for no pane or a
  split one (the writer writes no split); `print_options`,
  `margins` (all six, the element's presence), `page_setup`
  (the eleven fields of part 3's type, `fit_to_page` from
  `<sheetPr>`), `header_footer` (the six texts) into part 3's
  types; `auto_filter` the `<autoFilter ref>`; `merges` every
  `<mergeCell ref>` as an `Area`; `hyperlinks` each `<hyperlink>`
  as `HyperlinkEntry { cell: CellRef, target: Target, display:
  Option<String>, tooltip: Option<String> }` with `Target::Url(
  String)` resolved through the part's rels (an `r:id` naming no
  relationship refused) or `Target::Location(String)`, a
  hyperlink whose `ref` is a range taking its top-left cell;
  `protection` the fourteen flags of `SheetProtection` (the
  password hash kept as `None`, never read back);
  `outline_max` from `<sheetFormatPr>`.
- The types are part 3's and part 4's where they exist
  (`SheetView`, `PrintOptions`, `PageMargins`, `PageSetup`,
  `HeaderFooter` under `sheetprops`; `SheetProtection` under
  `metadata`): under decision 1 A they are lifted to `any(their
  feature, read)`, one definition each, so `read` alone carries
  the types and no writer.
- A part with two of an element the writers refuse twice is
  refused here with the same message; an attribute value that
  does not parse, "a sheet setting cannot be read".

### The shared strings table (decision 6)

The hold stays and is stated: `Package::shared_strings` holds the
table, 46 bytes per item on i686, a ceiling near 46 million unique
items under a 2048 MiB address space; `Package::shared_string_
items(&mut self) -> Result<Items<'_, R>, Error>` is added, a
streaming iterator yielding each `<si>`'s flattened text (and
with `richtext` its runs) one at a time, for a consumer that walks
the table without holding it; a consumer that resolves cells at
random holds the table or builds its own index. Measured on tier
d: the iterator's peak RSS under 16384 kB on both labels, against
the held table's 216572 and 266288.

### Errors

Two new `Error::Package` messages, in the catalogue as the earlier
parts' are, marked by feature:

`read`:
- "a style cannot be read" (at the read: a child outside the main
  namespace or unknown to the type, a value that does not parse,
  an id past its list, a colour with no form `Color` has)
- "a sheet setting cannot be read" (at the read: an attribute
  value that does not parse, a `ref` that is not an area, a
  hyperlink `r:id` naming no relationship)

Shared (constants under the feature, one definition): part 4's
row and cell messages where `rows` meets them; the writers'
two-elements messages for a part with two `<sheetViews>`,
`<cols>`, `<mergeCells>`, `<hyperlinks>`, `<sheetProtection>` or
a settings element; `specs/0010`'s "a cell holds a reference this
layer cannot expand"; part 2's unbound-prefix refusal; "no such
part"; `Error::Xml` for bytes that are not UTF-8. Nothing is
checked twice.

### 32-bit and streaming

Every count is `u32` or `u64`; the styles part is held as today
(its typed vectors beside the counts, bounded by the part: the
corpus's largest holds 473 fonts); `columns` is bounded by
`<cols>`; `rows` and `shared_string_items` hold one element at a
time; `sheet_settings` holds each small element to its end tag
and a `Vec` of merges and hyperlinks bounded by those elements.
No new whole-part hold; the one whole-part hold the crate has is
stated under decision 6.

### Size

One probe under decision 1 A: `probe-emend-read-write-xml`
(feature `read` of the probe crate, turning on `emend/read`): the
format probe's work, then on its output `cell_style` of every
cell of the named sheet (the count of distinct fonts printed),
`columns`, `rows` (the row count printed), `sheet_settings` (the
merge count printed) and `shared_string_items` (the item count
printed). The size log measures as part 4's did, classifies the
twenty-five probes of part 4's record against it, the new one
listed as new, prints the feature's cost (the read probe minus the
format probe rebuilt at the same commit), the core's growth (the
format probe at the commit minus the same probe rebuilt at
ef392d4, both printed; expected near 0), the budget against
458752, and a cause per probe; the cause control and the manifest
cause listing stay.

### Harness runs

`tools/emend-copy` gains two operations, given after the two paths
as the others are; neither takes a reference, name or number from
its command line. W is the first worksheet part in `<sheets>`
order. No reference and no text from the corpus is printed: the
samples print counts and flags.

- `--read-sample <in> <out>` (P2): writes the input back unedited
  (as `--cells` does) and reads the input: `cell_style` of every
  cell of every worksheet (through `cells` and `Cell::style`),
  `columns` and `rows` of W, `sheet_settings` of W, and
  `shared_string_items`; prints `read` with `cells` (cells read),
  `styled` (cells with an `s`), `resolved` (cell styles resolved,
  equal to `styled`, asserted), `fonts`, `fills`, `borders`,
  `gradients` (fills read as `None`), `codes` (number formats
  read as `Code`), `columns` (`<col>` of W), `rows` (rows of W),
  `merges`, `links`, `frozen` (0 or 1), `protected` (0 or 1),
  `items` (table items iterated, equal to `shared_strings().len()`,
  asserted), `sst_items_equal` (every item's text equal to the
  held table's at the same index, asserted). Expected results:
  every entry `identical`, `files_identical` `yes`.
- `--roundtrip-sample <in> <out>` (P1): for each distinct `s` of
  W's cells, `cell_style(s)` then `add_style(None, &Style { number_
  format, font, fill, border, alignment, protection })` with the
  value read (a `fill` of `None` given as no fill), the package
  written; reopens the output and for each added index
  `cell_style` again, asserting equality with the value read
  from the input; prints `roundtrip` with `styles_in` (distinct
  `s` on W), `styles_added` (equal, asserted), `xfs_before`,
  `xfs_after` (`+ styles_in`, asserted), `fonts_before`,
  `fonts_after`, `fills_before`, `fills_after`, `borders_before`,
  `borders_after` (each grown by the distinct values added, as
  `add_style` appends), `equal` (the read-back equalities, equal
  to `styles_in`, asserted). Expected results: `xl/styles.xml`
  `changed` and every other entry `identical`; `lo` and `lo_in`
  `ok`.

The run log's own check over the extracted input and output
parts, with `ElementTree` and byte spans and a control that
reports a planted difference per check, printing no name, no
reference and no text from the corpus:

- roundtrip, `xl/styles.xml`: the input's `fonts`, `fills`,
  `borders`, `numFmts` and `cellXfs` children each a
  byte-identical prefix of the output's; for each added `xf`, the
  appended `font`, `fill` and `border` elements compared to the
  input's element the read resolved: equal child by child as the
  type models them (the five unmodelled font children, the three
  unmodelled alignment attributes and a gradient absent on the
  written side, by the catalogue's rule), which is the written
  proof that the reader and the writer agree.
- read: every entry byte-identical; the line's counts against
  the survey's independent counts of W's `<col>`, `<row>`,
  `<mergeCell>`, `<hyperlink>`, the pane and the protection, and
  the table's `<si>` count.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. Every phase's run
log runs the edit open check on the host over that phase's keep,
names the phase directories it compares against by their record
commit, prints the devices of `TMPDIR` and of the corpus
directory, and counts the calc-chain workbooks with the rule
covering them. P2's run log also runs tier d's
`shared_string_items` through the tool (`--items`, a read
operation beside `--shared-strings`) on both labels under the cap,
the output byte-identical to the input and the peak RSS under
16384 kB (decision 6). The Excel check (decision 7) shows nothing
new for a read; the roundtrip sample joins its list.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m3p5-<phase>/` (`gate.log`,
`test.log` on both targets with the feature off and on,
`negative.log`, `run.log`, `scan.log`, `size.log` in P3); every
rule the earlier parts added (`specs/0026` "Phases" lists them);
the scan log last; at most two independent checks; report; STOP.
No push during the part; the close-out push follows P3 and carries
the record commit d64b4e8 of part 4's close-out with this part's.

Part 5's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0026-grouping-and-line-styles.md` as planning left them
after part 4's close-out (the close-out rulings 1 to 4, the Close
row and the part 4 heading), and this spec. Part 5's first code
commit carries, before any other change, `specs/0026` close-out
ruling 3's two items: the catalogue's "a column group" (`//!`) and
the rowcol probe's doc comment "a fourth time" (`//`).

### P1: cell formats and the round trip

The doc commit; the first code commit as above; then the feature
`read` declared in the crate and turned on by the tool, the
`<font>` reader as the one definition, the fill, border, alignment
and protection readers, `Styles`'s typed fields, `CellStyle`,
`cell_style`, the first message, and `--roundtrip-sample`. Tests
on byte-built parts: each font field read (name, size, bold,
italic, strike, each underline, each vertical alignment, each
colour form, family, each scheme; `b val="0"`; `charset` read
past); each pattern and both colours; a gradient as `None`; each
side and both diagonals; each alignment field with
`justifyLastLine` read past; both protection fields; a built-in
and a coded number format; an `xf` with no `fontId`; a round trip
of each through `add_style` on a byte-built part, the composed
element equal to the read one child by child; the refusals (an
id past its list, a colour with no form, a `val` that does not
parse, a child outside the namespace); a part with 473 fonts read
(the corpus's largest, byte-built); the tool's line. The gate as
part 4's P1 (the crate built and tested with the feature off and
on, on both targets, the `pub fn` lines listed off and on). The
negative log through the binary: the sample on a fixture whose
styles part names a font past `<fonts>` (refused, with a
control). The run log: `--roundtrip-sample` on both targets over
the corpus, the checks as "Harness runs", the keep and the edit
open check, recorded as `records/m3p5-p1-<utc-date>-<shortsha>.md`
and `.txt`.

### P2: the sheet, the lines and the table

The doc commit (the P1 verification rulings); the carry, if any;
then `ColumnFormat`, `columns`, `RowFormat`, `Rows`, `rows`,
`SheetSettings`, `HyperlinkEntry`, `Target`, `sheet_settings`
with the types lifted under `any`, `shared_string_items` with
`Items`, the second message, `--read-sample` and `--items`. Tests
on byte-built parts: every column and row field read, a `<col>`
range, a row without `r` refused; every settings field read from
a part holding each element (part 3's and part 4's written forms
among them), from a part holding none (the defaults: `None`,
empty, `SheetView::default()`), a split pane as `None`, a
hyperlink of each target with a range `ref`, an `r:id` naming no
relationship refused, two `<mergeCells>` refused; the table
iterator over a part with three items and with runs; the tool's
lines. The negative log through the binary: the sample on a
fixture with two `<sheetViews>` (refused, with a control). The
run log: `--read-sample` on both targets over the corpus, tier
d's `--items` on both labels under the cap with its peak checked,
recorded as `records/m3p5-p2-<utc-date>-<shortsha>.md` and `.txt`
and `records/m3p5-scale-d-items-<utc-date>-<shortsha>.md`.

### P3: the catalogue and limits, the probe, the size record

The doc commit (the P2 verification rulings); the carry, if any;
then the `package` header's catalogue and limits for this part
(the messages by feature; what the typed view drops, element by
element; the gradient and the split pane as `None`; the password
never read; the table's hold stated with its figures and the
iterator beside it); the probe with the size-probe manifest's
feature; the size log as "Size" says, recorded as
`records/size-m3p5-<utc-date>-<shortsha>.md`; the negative log
through the binary for every message of the part reachable there,
the unreachable ones stated with the test that covers each; both
samples again on both targets with the size log's release
binaries, byte-identical to P1's and P2's runs by sha256, recorded
as `records/m3p5-p3-<sample>-<utc-date>-<shortsha>.md` and
`.txt`. The open record of the part's keep.

### Close-out push (after P3)

As part 4's: the self-scan rule on the push and ci-run scripts,
`ci-run.log`, then the post-stage `scan.log` over the pushed range
with `push.log` and `ci-run.log` as its siblings, then one record
commit holding the three logs, the `ANTHROPIC_API_KEY` check; the
push script's previous-record pattern naming part 4's record
commit d64b4e8 (`^M3 part 4 close-out record`), its base part 4's
close-out doc commit 6e03f98 resolved by its subject ("M3 part 4
close-out: commit planning docs (the P3 verification and rulings,
decision 7 deferred again)"), its tip this part's close-out doc
commit; pre mode on a clean tree; one independent check before
the report. The push carries d64b4e8 and this part's commits, on
planning's word and the maintainer's authorisation typed in the
implementing session. Before the push, the Excel check for
Milestone 2 (every corpus row of every Milestone 2 sample, with
Milestone 3's samples on its list), deferred by the maintainer
nine times, last on 2026-10-05, is run (Milestone 2's acceptance
record then follows Milestone 1's form) or deferred again by a
decision recorded here (decision 7).

## Planning rulings during P1 (2026-10-05)

Asked by the implementing session after the doc commit 6d49977,
before any code.

1. The carried probe fix. `specs/0026` P3 verification ruling 2
   called the probe's "a fourth time" a `//` comment with a
   catalogue phrase to copy; it is a `//!` line (the probe's
   crate doc, lines 15 to 17) and no catalogue text gives the
   corrected phrase (the manifest's description carries the same
   miscount). Ruling: the proposal is taken. Those lines name the
   outputs by their arguments ("written to the fourth output with
   `insert_rows` ..., then that opened and written to the fifth
   output with `delete_columns` ..."), `//!` lines alone, in the
   first code commit beside the catalogue's "a column group"; the
   manifest's description is corrected in P3 with the size
   record, where the probe table is written. `specs/0026`'s
   ruling reads as corrected by this one.

## Planning verification and rulings on P1 (2026-10-05)

Verified by planning from primaries: the doc commit 6d49977 holds
this file at a655b5fd, `specs/0026-grouping-and-line-styles.md`
at aecc629a and `ROADMAP.md` at 53d2a173, the hashes planning
sent; the carry commit fbb171e changes `//!` lines alone (0 other
lines) in the catalogue and the rowcol probe; the crate commit
c019afa (the font reader lifted under `any`, `color_from`, the
styles read module, `CellStyle`, `cell_style`, 18 tests), the
tool commit 8d4d468 and the phase commit e727b25 (the message
catalogued, the module checked) read in full by planning; the
record commit 4264cd0 adds the six logs and four records; 6 of 6
messages ASCII, ending with the Co-Authored-By line only; the
logs at the reported hashes, each ending all as expected, the
scan RESULT: PASS, every log pinning a655b5fd and printing
5209aecb as planning's working copy; the check answer ASCII; the
compound-shape grep 0; every `rm -rf` in the logs the guarded
form on a directory the script made or `counting.sh`'s own;
planning's own build at e727b25 in a detached worktree: 513 crate
tests with the features off, 530 with `read`, 533 with
`richtext`, 550 with both and 855 with every feature, on both
targets, 1039 workspace tests on both, clippy and doc 0 lines;
the i686 release `emend-copy` at the run log's hash ed428826 (the
host's differs by build path); planning's own run of
`--roundtrip-sample` with its own binaries over all 27 corpus
workbooks on both labels, 54 outputs and 54 tool lines
byte-identical to the run directory's; planning's own privacy
scan d64b4e8..4264cd0, 0 findings; `ANTHROPIC_API_KEY` unset.

1. Item 1, the asymmetry: a `<fill>` holding no `patternFill` (a
   gradient, or an empty fill) reads as `fill: None`, and
   `add_style` given no fill writes `fillId="0"`, so such a cell
   format reads back as fill 0's value, not as `None`; the
   corpus's 362 used cell formats reach none (the survey), so the
   sample's equality held on all 27. Ruling: decision 3's
   consequence, stated in P3's catalogue; the sample's `equal`
   compares every field but a `None` fill, which it compares with
   fill 0's read value, so a workbook reaching one is not a
   failure; the change, with a test, goes in P2's first code
   commit. Item 2 (with `read` on, `Package::styles` refuses a
   part the typed read cannot take, for every call reading the
   part): the richtext precedent for comments; accepted and
   stated in the catalogue; no corpus part meets it.
2. R2: a side with `style="none"` reads as `None`, since
   `BorderStyle` has no `none`; "Cell formats" reads as corrected
   ("when it carries a `style` other than `none`"). R1: an empty
   `<fill>` reads as `None` with the gradient; accepted. R3:
   attributes the types do not name are read past on every
   element the module reads, as the three named alignment ones
   are, and a child inside `<alignment>` or `<protection>` is not
   examined, since both are read from their start tags; accepted
   as coded and stated in the catalogue; a test that an unknown
   attribute is read past goes in P2's first code commit.
3. R4: "a round trip of each" meant each field kind, which the
   120 combinations cover, and the read-side tests cover every
   variant; P2's first code commit extends the round-trip test
   over every enum variant (each underline, vertical alignment,
   pattern, border style, horizontal and vertical alignment),
   since the loop is cheap and the writer's keywords are the
   thing proved. R5, R6 (the gradient line and the Excel check
   list): P3's catalogue and the close-out, as "Phases" says. R7
   (the run reader's colour through `color_from`, equal by
   construction and by the richtext tests): accepted. The
   standing `CARGO_BUILD_JOBS` departure: accepted as before.
4. The disclosure that the crate commit raised a message the
   catalogue test did not compare until the phase commit: from
   now on a message is catalogued in the commit that raises it
   and the catalogue test reads the module in that commit, part
   4's form; recorded, no change to P1.
5. P1 is DONE. P2 is released: its doc commit carries this file
   with "Planning rulings during P1" and this section, and
   `ROADMAP.md`'s P1 row; its first code commit is rulings 1 to
   3's items (the sample's `None`-fill comparison with a test,
   the unknown-attribute test, the round-trip test over every
   variant); then "P2: the sheet, the lines and the table".

## Decisions for the maintainer

Decided by the maintainer 2026-10-05: 1 to 6 as recommended (A in
each), in the reply "A on all (Recommended)" to planning's
question, typed in the planning session; 7 recorded.

1. Packaging: A (recommended), one feature `read` with one probe,
   the settings and line read-backs under it with the writers'
   types lifted under `any`; B, each writer's feature grows its
   own reads.
2. The cell format: A (recommended), `cell_style(index)` resolves
   a cell's `s` on request, the styles part parsed typed once, and
   `Cell` is unchanged, so a cell pass costs today's bytes and
   time; B, every `Cell` carries its resolved `CellStyle`, which
   openpyxl does and which multiplies a 125-million-cell pass.
3. What the types cannot hold: A (recommended), a gradient fill
   reads as `fill: None`, a split pane as `freeze: None`, and the
   five font children and three alignment attributes the writers
   never write are read past, every case named in the catalogue,
   so every value read is one the writer accepts; B, new types
   for gradients, split panes and the eight attributes, and
   writers for them.
4. Rows: A (recommended), a streaming iterator, one `<row>` at a
   time, as `Cells` streams; B, a `Vec` of every row's format,
   which holds a million rows on tier c.
5. The sheet read: A (recommended), one call reading the part once
   into `SheetSettings`; B, a call per element, each a pass over
   the part.
6. The shared strings table: A (recommended), the hold stays and
   is stated with its measured figures, and a streaming item
   iterator is added for walks, measured on tier d; B, a
   byte-offset index per item with on-demand re-reads, about 8
   bytes per item plus a seek per resolve, a second design for
   one path.
7. The Excel check for Milestone 2: deferred nine times, last on
   2026-10-05 (`specs/0026` decision 7), to run before this part's
   close-out push at the latest or be deferred again by a decision
   recorded here. Recorded, not asked now; asked at the close-out.

## Out of scope

- Writing anything new; a change to any writer.
- `dxfs` (differential styles) read back, `cellStyles` and
  `cellStyleXfs` (named styles), the theme's colours resolved to
  RGB, the indexed palette, the default font: a `Color::Theme` or
  `Indexed` is given as the index it is.
- Gradient fills, split panes, `justifyLastLine`, `readingOrder`,
  `relativeIndent`, `charset`, `outline`, `shadow`, `condense`,
  `extend` as typed values (decision 3); the password hash.
- Reading defined names, validations, conditional formats,
  tables, comments and images: their parts' reads exist.
- A random-access shared strings resolver that does not hold the
  table (decision 6).
