# 0025: Milestone 3 part 3, sheet and print settings

Status: APPROVED by the maintainer 2026-10-03 ("A on all
(Recommended)", chosen in the planning session's question:
decisions 1 to 6 as planning recommended, 7 recorded). Sent to the
implementing session the same day.

Context: `CLAUDE.md` ("What emend is": like openpyxl; "Dependencies
and licences": anything beyond the core behind a Cargo feature off
by default; "The fidelity harness is the acceptance instrument"),
`ROADMAP.md` ("Milestone 3", the outline approved 2026-10-01, item
3: tab colour, the sheet view, page setup and margins, print area
and print titles as the built-in names, headers and footers, a
sheet-level autofilter, worksheet elements in schema position;
feature `sheetprops`; 35000 to 50000), `specs/0018-rows-and-
columns.md` ("Creating an element": the schema-position rule and
its table of 39 names; `freeze_panes` on the first `<sheetView>`),
`specs/0019-names-links-properties-protection.md` (defined names
and the `_xlnm.` refusal, its "Out of scope" leaving the built-in
names to "a later part with page setup"; `protect_sheet`'s
replace-or-create of one element), `specs/0021-tables.md` (a
table's own `<autoFilter>`), `specs/0024-rich-text.md` (the
feature pattern with a second probe for a pair of features; the
close-out rulings 2 to 4, carried here), `specs/0001-bootstrap.md`
("Rules for every phase"). ECMA-376 Part 1, 18.3.1.82 (`sheetPr`),
18.3.1.90 (`tabColor`), 18.3.1.65 (`pageSetUpPr`), 18.3.1.87
(`sheetView`), 18.3.1.70 (`printOptions`), 18.3.1.62
(`pageMargins`), 18.3.1.63 (`pageSetup`), 18.3.1.46
(`headerFooter`), 18.3.1.2 (`autoFilter`), 18.2.5 (`definedName`,
the built-in names), 18.3.1.99 (`worksheet`, the child order) are
cited where the design leans on them; no text is copied from
them. openpyxl's `Worksheet.sheet_properties` (`tabColor`,
`pageSetUpPr`), `sheet_view`, `page_setup`, `page_margins`,
`print_options`, `HeaderFooter` and `HeaderFooterItem`,
`print_area`, `print_title_rows` and `print_title_cols`, and
`auto_filter` are the behavioural reference, read only.

## Goal

The settings a caller sets on a worksheet without touching its
cells, each written into the element the format keeps it in, the
element replaced or created in schema position: the tab colour
and fit-to-page under `<sheetPr>`; zoom, gridlines, headings and
the view kind on the first `<sheetView>`; the print options,
margins and page setup; the six header and footer texts; the
print area and print titles as the sheet-scoped built-in defined
names; a sheet-level autofilter with its hidden built-in name. A
corpus sample per phase with the open checks; the size probe; the
catalogue; the close-out push. Everything behind the Cargo
feature `sheetprops`, off by default; the two capabilities that
write defined names need `metadata` too. Writes only: the typed
read-back of these settings is part 5's.

## Size and packaging (decision 1, before any code)

The budget figure after part 2 is 435720 section bytes against
the ceiling of 458752 (`records/size-m3p2-2026-10-03-0c3e4fa.md`):
23032 of headroom, held for fixes. The outline's estimate for this
part is 35000 to 50000, scaled from features that create parts.
Part 2 showed a feature that creates no part costs far less
(`richtext` 12672 for a write branch, a reader and a sparse
list; `specs/0024` P3 verification ruling 2). This part creates
no part either: it rewrites or creates nine worksheet elements by
one attribute-merge routine and two kinds of defined name through
`metadata`'s writer. Planning's estimate is 18000 to 30000 section
bytes for `sheetprops` alone and 3000 to 8000 for the pair with
`metadata`. The core's growth for the part is what the feature
gates add, printed by the size log and expected near 0; above
2000 section bytes, or a feature cost above 35000, planning is
told in the report.

- A (recommended): one Cargo feature `sheetprops`, off by
  default, declared beside `richtext` with no dependency of its
  own; the print area, print titles and autofilter calls under
  `sheetprops` and `metadata` together, as `add_rich_comment` sits
  under two; two probes, `probe-emend-sheetprops-write-xml` (the
  feature alone, over the core probe) and
  `probe-emend-printnames-write-xml` (`metadata` and `sheetprops`,
  over the `metadata` probe). 25 probes.
- B: `sheetprops` depends on `metadata` (`sheetprops = ["metadata"]`),
  one probe, 24 probes; a consumer that wants a tab colour carries
  the names, links, properties and protection code (50436).

## Facts planning checked (2026-10-03)

- The corpus, counted by planning with Python's `zipfile` and
  regular expressions over every regular file of the directory
  (27 workbooks, 95 worksheet parts), names and counts only:
  `sheetPr` in 26 workbooks (93 parts; attributes `filterMode` 45,
  `codeName` 31), `tabColor` in 25 (76; `rgb` 29, `theme` 47,
  `tint` 16), `pageSetUpPr` in 26 (83; `fitToPage` 81,
  `autoPageBreaks` 33); `sheetView` in all 27 (95, one per part;
  16 distinct attributes, `workbookViewId` on all 95,
  `showGridLines` 90, `zoomScaleNormal` 73, `tabSelected` 55,
  `zoomScale` 46, `view` 45); `pane` in 8 (9), `selection` in 21
  (77); `printOptions` in 25 (84; `horizontalCentered` 84,
  the other four 45 each); `pageMargins` in 27 (95, the six
  attributes on all 95); `pageSetup` in 27 (95; `orientation` 95,
  `fitToHeight` 79, `scale` 68, `r:id` 50, `horizontalDpi` and
  `verticalDpi` 48, `paperSize` 47, `fitToWidth` 46,
  `firstPageNumber` and `useFirstPageNumber` 19, nine others 45
  each); `headerFooter` in 24 (78; `differentFirst` 78,
  `differentOddEven` 45, `alignWithMargins` 2), `oddHeader` 45,
  `oddFooter` 77, no `even` or `first` header or footer; the
  header and footer codes used: `&P` 34, `&N` 32, `&C` 2, `&R` 2;
  `autoFilter` in 2 (2, `ref` only); `sheetProtection` in 5 (7);
  the built-in defined names `_xlnm.Print_Titles` 25,
  `_xlnm.Print_Area` 5, `_xlnm._FilterDatabase` 3; 41 distinct
  root-child orders, every one a subsequence of the 39-name table
  of `specs/0018`.
- `WORKSHEET_CHILDREN` (`package.rs`) is that table; `CREATED`
  holds the three elements part 3 creates (`sheetViews`, `cols`,
  `mergeCells`), each created in schema position by
  `created_element`; `protect_sheet` replaces or creates
  `<sheetProtection>` through its own pass (`ProtectPass`,
  `create_before`), refusing a part with two of them; `freeze_panes`
  rewrites the first `<sheetView>`'s children and keeps its start
  tag's attributes as bytes; a `<sheetViews>` the write creates
  holds `<sheetView workbookViewId="0">`.
- `add_defined_name` refuses an `_xlnm.`-prefixed name at the
  call (`check_defined_name`); `DefinedName` reads `hidden`; the
  writer writes `name`, `localSheetId` and the text, and `specs/
  0019` leaves the built-in names to this part.
- A `<pageSetup>` may carry `r:id` naming a `printerSettings`
  relationship (50 of the corpus's 95); an attribute this layer
  does not model must survive a rewrite of the element, else the
  relationship is orphaned.
- The feature costs after part 2 (section bytes over the core
  probe): `dates` 4824, `richtext` 12672, `rowcol` 35472,
  `images` 40960, `tables` 46904, `rules` 48180, `metadata` 50436,
  `comments` 57224; the pair `comments` and `richtext` 14820 over
  the `comments` probe.
- openpyxl, planning's reading of 3.1, read only:
  `ws.sheet_properties.tabColor` takes a colour; `pageSetUpPr.
  fitToPage`; `ws.sheet_view.zoomScale`, `showGridLines`,
  `showRowColHeaders`, `view` (`normal`, `pageBreakPreview`,
  `pageLayout`); `ws.page_setup` with `orientation`, `paperSize`,
  `scale`, `fitToWidth`, `fitToHeight`, `firstPageNumber`,
  `useFirstPageNumber`, `blackAndWhite`, `draft`, `copies`,
  `pageOrder`; `ws.page_margins` with the six; `ws.print_options`
  with `horizontalCentered`, `verticalCentered`, `headings`,
  `gridLines`; `ws.oddHeader.left.text` and the like, six items by
  three parts each, joined as `&L`, `&C`, `&R` codes, a literal
  `&` written `&&`; `ws.print_area` and `ws.print_title_rows`
  written as `_xlnm.Print_Area` and `_xlnm.Print_Titles` with the
  sheet's `localSheetId`; `ws.auto_filter.ref` written as
  `<autoFilter ref>` with a hidden `_xlnm._FilterDatabase` name.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. Under decision 1 A,
everything below sits behind the Cargo feature `sheetprops`, off
by default, with the types and calls of its own and its messages
in the catalogue marked as the feature's; the print area, print
titles and autofilter calls sit under `sheetprops` and `metadata`
together, because they write defined names through `specs/0019`'s
writer. The core builds, tests and links without it; the tool
`emend-copy` turns it on. Nothing existing changes signature.

### Types

In `package`, feature `sheetprops`, each derived `Clone`, `Debug`,
`Default`, `PartialEq` (floats where the format has them):

- The tab colour is `styles::Color` itself (`rgb`, `theme` with
  `tint`, `indexed`, `auto`), no alias, written as `<tabColor>`'s
  attributes in the form `Font`'s colour is.
- `SheetView { zoom: Option<u32>, show_grid_lines: Option<bool>,
  show_headings: Option<bool>, show_zeros: Option<bool>,
  show_formulas: Option<bool>, right_to_left: Option<bool>, view:
  Option<ViewKind> }`; `ViewKind { Normal, PageBreakPreview,
  PageLayout }`. `zoom` is `zoomScale`, 10 to 400.
- `PrintOptions { horizontal_centered, vertical_centered,
  headings, grid_lines: Option<bool> }` (the four openpyxl
  exposes; `gridLinesSet` is not modelled).
- `PageMargins { left, right, top, bottom, header, footer:
  Option<f64> }`, inches, finite and at least zero.
- `PageSetup { orientation: Option<Orientation>, paper_size:
  Option<u32>, scale: Option<u32>, fit_to_width: Option<u32>,
  fit_to_height: Option<u32>, fit_to_page: Option<bool>,
  first_page_number: Option<u32>, black_and_white: Option<bool>,
  draft: Option<bool>, copies: Option<u32>, page_order:
  Option<PageOrder> }`; `Orientation { Portrait, Landscape }`,
  `PageOrder { DownThenOver, OverThenDown }`. `scale` 10 to 400;
  `copies` at least 1; `fit_to_page` is `<pageSetUpPr fitToPage>`
  under `<sheetPr>`, which is where the format keeps the switch
  the two `fitTo` counts depend on, so one call sets all three;
  `first_page_number` writes `useFirstPageNumber="1"` with it.
- `HeaderFooter { odd_header, odd_footer, even_header,
  even_footer, first_header, first_footer: Option<String> }`:
  each text is the format's code string as `HeaderFooterItem`
  joins it, stored and escaped, never parsed; `HeaderFooter::
  section(left, center, right) -> String` builds one from three
  parts, writing `&L`, `&C`, `&R` before each non-empty part and
  a literal `&` as `&&`. `differentOddEven` is written when an
  even text is given, `differentFirst` when a first text is.

### The calls

Each records an edit on the worksheet part `part`, as
`freeze_panes` and `protect_sheet` do (P1 ruling 1): refused at
the call for a value out of range or a text holding a character
XML cannot carry, nothing recorded; the part never looked up at
the call, so a sheet `add_sheet` adds in the same write is
accepted and the edit lands on its blank worksheet; a part
neither in the archive nor added refused at the write with "no
such part" by the write's existing check over the edited parts;
a part this write removes or replaces, or a part with a row or
column shift in the same write, refused at the write with part
1's and part 3's messages. The last call for one element on one
part wins, field by field: two calls each set the fields they
give.

- `set_tab_color(part, color: Option<Color>)`: `Some` writes
  `<tabColor>` under `<sheetPr>`, created when absent (`<sheetPr>`
  created in schema position, `<tabColor>` placed as 18.3.1.82
  orders the children: `tabColor`, `outlinePr`, `pageSetUpPr`);
  `None` drops it.
- `set_sheet_view(part, view: &SheetView)`: each `Some` field
  written as its attribute on the first `<sheetView>`, every
  other attribute kept; a `<sheetViews>` created as `freeze_panes`
  creates one. `tabSelected` and `workbookViewId` are not
  modelled: which sheet is active is the workbook's.
- `set_print_options(part, options: &PrintOptions)`,
  `set_page_margins(part, margins: &PageMargins)`,
  `set_page_setup(part, setup: &PageSetup)`: each `Some` field
  written as its attribute on the element, every other attribute
  kept (`r:id` among them), the element created in schema position
  when absent; `fit_to_page` goes to `<pageSetUpPr>` under
  `<sheetPr>` by the tab colour's rule. A `<pageMargins>` created
  with fields missing takes Excel's defaults for the rest (0.7,
  0.7, 0.75, 0.75, 0.3, 0.3), since the schema requires all six.
- `set_header_footer(part, hf: &HeaderFooter)`: each `Some` text
  written as its child element of `<headerFooter>`, in schema
  order, the other children kept; `None` leaves a kept child as it
  is; `Some("")` drops it; `differentOddEven` and `differentFirst`
  set when an even or first text is written, kept otherwise.
- `set_print_area(part, areas: &[Area])` (with `metadata`): writes
  the sheet-scoped `_xlnm.Print_Area` whose text is the areas
  joined by `,`, each as `'<sheet name>'!$A$1:$B$2` with the sheet
  name quoted as a formula quotes it (an apostrophe doubled);
  replaces one the file has for that sheet; an empty slice drops
  it. `set_print_titles(part, rows: Option<(u32, u32)>, cols:
  Option<(u32, u32)>)`: `_xlnm.Print_Titles` as `'<name>'!$1:$3`,
  `'<name>'!$A:$B`, or both joined by `,`; both `None` drops it.
  Each goes through `specs/0019`'s name edit with the built-in
  check lifted for these two names and the scope fixed to the
  part's sheet, so the workbook rewrite, the `localSheetId`
  remapping and the formula-change rule are the ones already
  measured. The names are never parsed on read.
- `set_auto_filter(part, area: Option<Area>)` (with `metadata`):
  `Some` writes `<autoFilter ref>` in schema position (replacing
  one the part has, its children dropped, since a filter's
  columns and sort state belong to the old range) and the hidden
  sheet-scoped `_xlnm._FilterDatabase` with the area's text as
  the print area's; `None` drops both. A part whose area overlaps
  a table's `ref` is refused at the write, "a table overlaps a
  table, a merge or an autofilter" (`specs/0021`'s message, since
  Excel keeps the two apart); the table's own `<autoFilter>` is
  inside its part and untouched.

### The pass

One routine, "the settings pass", inside the worksheet rewrite of
`specs/0011`: for each of the elements `sheetPr`, `sheetViews`
(its first `sheetView`), `autoFilter`, `printOptions`,
`pageMargins`, `pageSetup` and `headerFooter` with a recorded
edit, the element is held whole to its end tag (bounded by its
own size, never by the sheet's rows), its attributes read, the
modelled ones replaced, the rest kept in their order, the modelled
ones written first in the schema's attribute order and the kept
ones after; its children kept, except the ones a call replaces
(`tabColor` and `pageSetUpPr` under `sheetPr`, the six texts under
`headerFooter`, every child of a replaced `autoFilter`). An absent
element is created in schema position by `specs/0018`'s rule and
table, the created element taking the root's prefix. A part with
two of any such element, or one out of schema order, is refused,
"a worksheet has two sheetPr, autoFilter, printOptions,
pageMargins, pageSetup or headerFooter elements" (the existing
message covers `sheetViews`). A pass edits a `<sheetView>` that
`freeze_panes` edits in the same write by the same element pass,
the pane after the attributes.

### Errors

Four new `Error::Package` messages, in the catalogue as the
earlier parts' are, one constant each, marked by feature:

`sheetprops`:
- "a sheet setting is out of range" (at the call: zoom, scale,
  copies, paper size, a page number, a fit count, a margin not
  finite or below zero, a theme tab colour's tint not finite or
  outside `-1.0..=1.0`; P1 verification ruling 1)
- "a header or footer text is not valid" (at the call: a text
  over 255 characters, Excel's limit, or holding a forbidden
  character)
- "a worksheet has two sheetPr, autoFilter, printOptions,
  pageMargins, pageSetup or headerFooter elements"

`sheetprops` and `metadata`:
- "a print area or title is not valid" (at the call: an empty
  area list past the drop case, a title row or column pair out of
  order or off the grid)

Shared (constants under the feature, one definition): part 3's
index message for an area off the grid; part 1's "no such part",
"an edit to a removed part" and "a cell edit to a part this write
replaces"; part 3's one-shift message; part 4a's defined-name
messages where the name edit raises them; part 4c's overlap
message; part 2's unbound-prefix refusal; `Error::Xml` for bytes
that are not UTF-8. Nothing is checked twice.

### 32-bit and streaming

Every count is `u32` or `u64`; the worksheet pass streams as
today, with each settings element held whole to its end tag, the
largest in the corpus a `<sheetView>` with its `<selection>`
children and a `<headerFooter>` with two texts; the workbook
rewrite for a built-in name is `specs/0019`'s, measured. No new
whole-part hold.

### Size

Two probes under decision 1 A. `probe-emend-sheetprops-write-xml`
(feature `sheetprops` of the probe crate, turning on
`emend/sheetprops`): `probe-emend-format-write-xml`'s work,
unchanged, then on the same package before its write
`set_tab_color`, `set_sheet_view` (a zoom from its command line
and gridlines off), `set_print_options`, `set_page_margins`,
`set_page_setup` (landscape, fit to one page wide) and
`set_header_footer` (an odd header built by `section`) on the
named sheet; after the write the output opened and the count of
`sheetPr`, `printOptions`, `pageMargins`, `pageSetup` and
`headerFooter` elements of that part read by the probe's own
`quick-xml` count and printed. `probe-emend-printnames-write-xml`
(features `metadata` and `sheetprops`): `probe-emend-metadata-
write-xml`'s work, then `set_print_area`, `set_print_titles` and
`set_auto_filter`, the defined-name count read back through
`workbook` and printed. The size log measures them as part 2's
did, classifies the twenty-three probes of part 2's record against
it, the two new ones listed as new, prints the feature's cost (the
sheetprops probe minus the format probe rebuilt at the same
commit), the pair's cost (the printnames probe minus the metadata
probe), the core's growth for the part (the format probe at the
commit minus the same probe rebuilt at 0c3e4fa, both printed;
expected near 0), the budget figure against 458752, and a cause
per probe read off the diff's paths (`specs/0024` P3 verification
ruling 5, done here rather than carried); the cause control and
the manifest cause listing stay.

### Harness runs

`tools/emend-copy` gains three operations, given after the two
paths as the others are; none takes a reference, name or number
from its command line; every constant below is the tool's own. W
is the first worksheet part in `<sheets>` order and M its largest
row number as the tool's cell read gives it. No reference and no
text from the corpus is printed. Each sample reopens its output and
reads the elements it wrote with the tool's own `quick-xml` walk
(the typed read is part 5's), asserting the attributes it set,
and prints counts.

- `--sheetview-sample <in> <out>` (P1): `set_tab_color(W,
  Some(Color::Rgb(0xFF00AA55)))`, `set_sheet_view(W, zoom 85,
  gridlines off, headings on)`; prints `sheetview` with
  `sheetpr_before`, `sheetpr_after` (`<sheetPr>` of W, 0 or 1; 1
  after, asserted), `tabcolor_before`, `tabcolor_after` (1 after,
  asserted), `views_before`, `views_after` (`<sheetView>` of W;
  equal, or 1 after a created `<sheetViews>`, asserted), `zoom_ok`,
  `grid_ok`, `headings_ok` (each 1 when the first `<sheetView>`
  carries the set value, asserted), `kept_attrs` (the first
  `<sheetView>`'s attributes other than the three, before and
  after, equal in number and bytes, asserted), `r_bound_before`.
- `--pagesetup-sample <in> <out>` (P2): `set_print_options`
  (centred horizontally), `set_page_margins` (left 0.5),
  `set_page_setup` (landscape, fit to 1 wide by 0 tall,
  `fit_to_page` true) and `set_header_footer` (odd header
  `section("emend", "", "&P of &N")`, odd footer `section("",
  "emend", "")`); prints `pagesetup` with, for each of
  `printOptions`, `pageMargins`, `pageSetup`, `headerFooter`,
  `<name>_before` and `<name>_after` (0 or 1; 1 after, asserted),
  `rid_kept` (1 when W's `<pageSetup>` had `r:id` and keeps it
  byte for byte, or had none, asserted), `fit_ok`, `margin_ok`,
  `orient_ok`, `header_ok`, `footer_ok` (1 each, asserted),
  `hf_kept` (the `headerFooter` children other than the two,
  equal before and after, asserted), `r_bound_before`.
- `--printarea-sample <in> <out>` (P3): `set_print_area(W, [A1:
  (M + 1, 8)])`, `set_print_titles(W, rows (1, 1), None)`,
  `set_auto_filter(W, Some(A1:(M + 1, 8)))`; a W whose table
  overlaps the area refuses the sample whole with the overlap
  message, and the run log reports it as a finding with the
  workbook id; prints `printarea` with `names_before`,
  `names_after` (defined names of the workbook; `+ 3` less the
  ones W already had of the three, asserted), `area_ok`,
  `titles_ok`, `filterdb_ok` (1 each when the sheet-scoped name
  exists with the expected text, asserted), `autofilter_before`,
  `autofilter_after` (`<autoFilter>` of W; 1 after, asserted),
  `hidden_ok` (1 when `_FilterDatabase` is hidden, asserted),
  `r_bound_before`.

The runner and the record script are unchanged; the run names its
operation through `--tool-arg` and one `--label`. Expected
results, `lo` and `lo_in` `ok` in every case; `descriptor` alone
allowed on a changed entry; `order_preserved` `yes`;
`files_identical` `no`; for the first two samples W `changed` and
every other entry `identical`; for the third, W `changed` (the
autofilter) and `xl/workbook.xml` `changed` (the names), every
other entry `identical`; the calc chain is never dropped (no
formula changes).

The run log's own check over the extracted input and output
parts, with `ElementTree` and byte spans and a control that
reports a planted difference per check, printing no name, no
reference and no text from the corpus:

- W: every root child byte-identical except the elements the
  sample sets, each in schema position (checked against the
  table) whether kept or created; on a kept element, the
  attributes the sample did not set byte-identical and in their
  input order, the children the sample did not replace
  byte-identical; the set attributes carrying the tool's
  constants; `<sheetPr>`'s children in 18.3.1.82's order.
- `xl/workbook.xml`, printarea: the input's defined names a
  byte-identical prefix of the output's less the three the sample
  replaces, the three present with the expected `name`,
  `localSheetId`, `hidden` and text; every other root child
  byte-identical.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. Every phase's run
log also runs the edit open check on the host over that phase's
keep, names the phase directories it compares against by their
record commit, and prints the device of `TMPDIR` and of the
corpus directory (`specs/0024` P3 verification ruling 5).
LibreOffice's rendering of a setting is not checked, only that
the file opens; the Excel check (decision 7) shows them as Excel
shows them, and this part's three samples join its list.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m3p3-<phase>/` (`gate.log`,
`test.log` on both targets with the features off and on,
`negative.log`, `run.log`, `scan.log`, `size.log` in P4); every
rule the earlier parts added (`specs/0024` "Phases" lists them,
with its P2 ruling 1 on `rm -rf` and its P1 and P2 verification
rulings 3 and 6 on the scripts' checks); the scan log last; at
most two independent checks; report; STOP. No push during the
part; the close-out push follows P4 and carries the record commit
21f1434 of part 2's close-out with this part's.

Part 3's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0024-rich-text.md` as planning left them after part 2's
close-out (the close-out rulings 1 to 4, the Close row and the
part 2 heading), and this spec. Part 3's first code commit
carries, before any other change, `specs/0024` close-out ruling
3's items: the three `//!` edits to the catalogue's rich text
block (P3 verification ruling 3 there) and the test pinning that
an unbound prefix on a kept `<rPr>` child is refused by part 2's
rule, through the table and a comment (ruling 4 there).

### P1: the sheet properties and the view

The doc commit; the first code commit as above; then the feature
`sheetprops` declared in the crate and turned on by the tool, the
settings pass with its attribute-merge routine, `TabColor`'s
use of `Color`, `SheetView`, `ViewKind`, `set_tab_color`,
`set_sheet_view`, the two messages reachable here, and
`--sheetview-sample`. Tests on byte-built parts: a tab colour in
each colour form written into a part with no `<sheetPr>` (created
in schema position before `<dimension>`, the bytes checked against
literal bytes), with a `<sheetPr>` holding `codeName` and
`pageSetUpPr` (the attribute and child kept, `tabColor` first),
with a kept `tabColor` (replaced) and `None` on each (dropped,
and a `<sheetPr>` left with no attribute and no child dropped
with it); a sheet view with each field
on a part whose first `<sheetView>` carries the corpus's sixteen
attributes (the set ones replaced in place of their schema
position, the rest kept in order), on a part with no
`<sheetViews>` (created with `workbookViewId="0"`), with a
`freeze_panes` in the same write (both applied, the pane after
the attributes); two calls on one part (the second's fields win,
the first's others stand); the refusals (zoom 9 and 401, a part
with two `<sheetPr>`, a `<sheetPr>` after `<dimension>`); a
setting with a shift refused and on a replaced part refused; the
tool's line. The gate as part 2's P1: the crate built and tested
with the feature off and on, on both targets, the `pub fn` and
`pub const` first lines listed off and on. The negative log
through the binary: the sample on a fixture with two `<sheetPr>`
(refused, with a control). The run log: `--sheetview-sample` on
both targets over the corpus, the checks as "Harness runs", the
keep and the edit open check, recorded as
`records/m3p3-p1-<utc-date>-<shortsha>.md` and `.txt`.

### P2: print options, margins, page setup, headers and footers

The doc commit (the P1 verification rulings); the carry, if any;
then `PrintOptions`, `PageMargins`, `PageSetup`, `Orientation`,
`PageOrder`, `HeaderFooter` with `section`, the four calls, the
header message, and `--pagesetup-sample`. Tests on byte-built
parts: each element created in schema position on a part lacking
it (each against literal bytes; `<pageMargins>` with Excel's
defaults for the fields not given), each replaced on a part
holding it with every corpus attribute (the set ones replaced,
`r:id` and the rest kept in order); `fit_to_page` creating
`<pageSetUpPr>` under an existing `<sheetPr>` and under a created
one; each header and footer child written and replaced, `Some("")`
dropping one, `None` keeping one, `differentFirst` and
`differentOddEven` set and kept; `section` with every part empty
("" written as no text), with an `&` in a part (`&&`), with codes
in a part (passed through); the refusals (scale 9, copies 0, a
negative margin, a NaN margin, a 256-character header, a
forbidden character); the four elements all created in one write
in schema order; the tool's line. The negative log through the
binary: the sample on a fixture with two `<pageSetup>` (refused,
with a control). The run log: `--pagesetup-sample` on both
targets, recorded as `records/m3p3-p2-<utc-date>-<shortsha>.md`
and `.txt`.

### P3: the print area, the print titles, the autofilter

The doc commit (the P2 verification rulings); the carry, if any;
then under `sheetprops` and `metadata` the three calls, the
built-in check lifted for the three names inside the name edit
(every other `_xlnm.` name still refused through
`add_defined_name`), the hidden flag on `_FilterDatabase`, the
sheet-name quoting, the overlap check against the part's tables,
the one message, and `--printarea-sample`. Tests on byte-built
parts and workbooks: a print area of one and of two areas written
into a workbook with no `<definedNames>` (created, as `specs/0019`
creates it) and with one holding a `_xlnm.Print_Area` for another
sheet (kept) and for this sheet (replaced); print titles of rows,
of columns and of both; a sheet name with a space and with an
apostrophe quoted; an autofilter on a part with none (created in
schema position after `<sheetData>` and before `<mergeCells>`),
replacing one with children (dropped), overlapping a table
(refused at the write), and `None` dropping both the element and
the name; the three names' `localSheetId` after a sheet removal in
the same write remapped by `specs/0016`'s rule; the refusals (an
empty title pair, a title pair out of order, an area off the
grid); the tool's line. The negative log through the binary: the
sample on a fixture whose first sheet's table covers A1:H2 (refused
with the overlap message, with a control). The run log:
`--printarea-sample` on both targets, recorded as
`records/m3p3-p3-<utc-date>-<shortsha>.md` and `.txt`; a corpus
sheet whose table overlaps the area is a finding recorded with
its id, not a failure, and the record shows the refusal.

### P4: the catalogue and limits, the probes, the size record

The doc commit (the P3 verification rulings); the carry, if any;
then the `package` header's catalogue and limits for this part
(the messages marked by feature; the attribute-merge rule and
what each call never writes: `tabSelected`, `workbookViewId`,
`gridLinesSet`, `autoPageBreaks`, `outlinePr`, `cellComments`,
`errors`, `useFirstPageNumber` without a number, the printer
settings part, a filter's columns or sort state, an even or first
text without the flag; what the read of part 5 will give); the
two probes with the size-probe manifest's features; the size log
as "Size" says, recorded as `records/size-m3p3-<utc-date>-
<shortsha>.md`; the negative log through the binary for every
message of the part reachable there, the unreachable ones stated
with the test that covers each; the three samples again on both
targets with the size log's release binaries, byte-identical to
P1's, P2's and P3's runs by sha256, recorded as `records/m3p3-p4-
<sample>-<utc-date>-<shortsha>.md` and `.txt`. The open record of
the part's keep.

### Close-out push (after P4)

As part 2's: the self-scan rule on the push and ci-run scripts,
`ci-run.log`, then the post-stage `scan.log` over the pushed range
with `push.log` and `ci-run.log` as its siblings, then one record
commit holding the three logs, the `ANTHROPIC_API_KEY` check; the
push script's previous-record pattern naming part 2's record
commit 21f1434 (`^M3 part 2 close-out record`), its base part 2's
close-out doc commit 50d9e70 resolved by its subject ("M3 part 2
close-out: commit planning docs (the P3 verification and rulings,
decision 7 deferred again)"), its tip this part's close-out doc
commit; pre mode on a clean tree; one independent check before
the report. The push carries 21f1434 and this part's commits, on
planning's word and the maintainer's authorisation typed in the
implementing session. Before the push, the Excel check for
Milestone 2 (every corpus row of every Milestone 2 sample, with
Milestone 3's samples on its list), deferred by the maintainer
seven times, last on 2026-10-03, is run (Milestone 2's acceptance
record then follows Milestone 1's form) or deferred again by a
decision recorded here (decision 7).

## Planning rulings during P1 (2026-10-03)

Asked by the implementing session after the doc commit e5f998a
and the carry commit bf5e877, before any log.

1. "The calls" said a part the archive lacks is refused at the
   call, "as the settings calls of `specs/0018` and `specs/0019`
   do". They do not: `freeze_panes`, `protect_sheet`,
   `set_row_height` and `merge_cells` record without looking the
   part up, and the write's check over the edited parts raises
   "no such part" for a part neither in the archive nor added by
   `add_sheet` in the same write, which is how a setting on a
   sheet added in the same write is accepted. Ruling: this part's
   calls do the same; their parts join the write's list of edited
   parts, and the removed-part, replaced-part and one-shift
   refusals come from the same existing checks. "The calls" reads
   as corrected. A test pins a tab colour on a sheet added in the
   same write.

## Planning verification and rulings on P1 (2026-10-03)

Verified by planning from primaries: the doc commit e5f998a holds
this file at c15d68d2, `specs/0024-rich-text.md` at ae1ab201 and
`ROADMAP.md` at fb5d2ac4, the hashes planning sent; the carry
commit bf5e877 changes `//!` lines alone in `package.rs` (0 other
lines) and adds the two unbound-prefix tests; the crate commit
4f53d49 and the tool commit 9db5f4f read in full by planning; the
record commit 138fe80 adds the six logs and four records; 5 of 5
messages ASCII, ending with the Co-Authored-By line only; the logs
at the reported hashes, each ending all as expected, the scan
RESULT: PASS, every log pinning c15d68d2 and printing a555f5f1 as
planning's working copy; the compound-shape grep 0; every
`rm -rf` in the logs either the `"${var:?}"` form on a directory
the script made or `counting.sh`'s own; planning's own build at
9db5f4f in a detached worktree: 513 crate tests with the feature
off, 529 with it on and 774 with every feature, on both targets,
947 workspace tests on both, clippy and doc 0 lines; the i686
release `emend-copy` at the run log's hash a81c2a5c (the host's
differs by build path); planning's own run of `--sheetview-sample`
with its own binaries over all 27 corpus workbooks on both
labels, 54 outputs and 54 tool lines byte-identical to the run
directory's; planning's own privacy scan 21f1434..138fe80, 6
commits, 0 findings; `ANTHROPIC_API_KEY` unset.

1. R1 accepted and written into "Errors": a theme tab colour whose
   tint is not finite or outside `-1.0..=1.0` is refused at the
   call with "a sheet setting is out of range", the range the
   style writer enforces, so every colour the call records is one
   the write accepts.
2. R2, the merge routine's doc comment ("its unprefixed name")
   against the code (the raw qualified name compared, so a
   prefixed attribute of the same local name is kept): the
   behaviour is right, since a prefixed attribute is another
   attribute; the wording is wrong. Carried to P2's first code
   commit as the `//` fix. R3, separators between kept attributes
   written as one space and whitespace before the tag's end
   dropped, each attribute's own bytes kept: accepted; "The pass"
   reads "in its order" as the attributes' order, not the
   separators'; P2's catalogue sentence says so. R4, the two-
   `<sheetPr>` refusal only when the write edits `<sheetPr>`: as
   "The pass" scopes it; a test pins a view-only edit on such a
   part passing, in P2's first code commit. R5, the first view's
   pane skip only on a pane edit, with the test pinning a view
   alone keeping the pane and selections: accepted as a
   behaviour-preserving change. R6 (`Copy` and `Eq` on `SheetView`
   and `ViewKind`, `Color` used without an alias) and R7 (the
   sixteenth attribute from the implementing session's unlogged
   survey of all 95 parts): accepted; "Types" reads `Color` for the
   tab colour.
3. The log reservations: L1, the run log's stale change sentence
   kept as history with a correcting change, accepted (the
   correction is what P2 verification ruling 6 of `specs/0024`
   asked); L2, the open check's own `TMPDIR` device not printed:
   P2's run log prints it beside the runs'; L3, the plants on the
   host label only: accepted (the comparison is label-blind); L4,
   no corpus W lacks `<sheetViews>`, so the created-view path is
   proven by the crate's and the tool's tests: accepted and stated
   in the catalogue at P4.
4. The disclosure: a fork of the implementing session ran `rm -rf`
   by hand on a pre-existing bytecode cache directory under the
   phase's build directory, which the maintainer's rule forbids
   (`specs/0024` P2 ruling 1, the absolute case); nothing of value
   was lost and no logged script removes what it did not create.
   Recorded; disclosed to the maintainer in planning's report; the
   implementing session's fork prompts now forbid any hand `rm
   -rf`, and planning's own say the same.
5. The check's items with no primary (the pre-check hash, the "Q1"
   label, the disclosure, "nothing pushed"): working state,
   labelled so. Accepted.
6. P1 is DONE. P2 is released: its doc commit carries this file
   with "Planning rulings during P1" and this section, and
   `ROADMAP.md`'s P1 row; its first code commit is ruling 2's
   items (the `//` wording fix and the view-only test); then
   "P2: print options, margins, page setup, headers and footers".

## Decisions for the maintainer

Decided by the maintainer 2026-10-03: 1 to 6 as recommended (A in
each), in the reply "A on all (Recommended)" to planning's
question, typed in the planning session; 7 recorded.

1. Packaging: A (recommended), one feature `sheetprops` with two
   probes, the three name-writing calls under `sheetprops` and
   `metadata` together; B, `sheetprops` depends on `metadata`,
   one probe.
2. Attribute merge: A (recommended), a settings call replaces the
   attributes it models and keeps every other attribute and child
   byte for byte, so a `<pageSetup>`'s `r:id` and a `<sheetView>`'s
   `tabSelected` survive; B, the element rewritten from the typed
   value alone, as `protect_sheet` writes `<sheetProtection>`,
   which would orphan the printer settings relationship on 50 of
   the corpus's 95 sheets.
3. Headers and footers: A (recommended), the text is the format's
   code string, stored and escaped, with `section` building one
   from three parts and doubling a literal `&`, as openpyxl's
   `HeaderFooterItem` does; no parsing, no font codes typed; B, a
   typed model of the codes (font, size, page, date, file), a
   parser and a writer, priced near `richtext`.
4. The autofilter's name: A (recommended), `<autoFilter>` and the
   hidden sheet-scoped `_xlnm._FilterDatabase` written together,
   Excel's and openpyxl's form, and dropped together; B, the
   element alone, which Excel repairs on open.
5. The print area's and titles' form: A (recommended), the texts
   written as Excel writes them (`'<sheet>'!$A$1:$B$2`, titles as
   whole rows or columns, areas joined by `,`), the sheet name
   quoted as a formula quotes it, never parsed back in this part;
   B, the caller gives the text, as `add_defined_name` takes one,
   and the built-in check is merely lifted.
6. Reads: A (recommended), writes only in this part, each sample
   asserting through the tool's own XML walk; the typed read-back
   (`sheet_view`, `page_setup`, the names) is part 5's, as the
   outline puts it; B, a read call per element here, part 5
   shrinking.
7. The Excel check for Milestone 2: deferred seven times, last on
   2026-10-03 (`specs/0024` decision 7), to run before this part's
   close-out push at the latest or be deferred again by a decision
   recorded here. Recorded, not asked now; asked at the close-out.

## Out of scope

- Outline levels, `outlinePr` and row or column styles (part 4);
  the typed read-back (part 5); the comment box (part 6).
- Page breaks (`rowBreaks`, `colBreaks`), `customSheetViews`, a
  second `<sheetView>`, `tabSelected` and the workbook's active
  sheet, `printerSettings` parts, `cellComments` and `errors` on
  `<pageSetup>`, `gridLinesSet`, `autoPageBreaks`, `alignWithMargins`
  and `scaleWithDoc` on `<headerFooter>`: kept as found, never
  written.
- A filter's columns, criteria and sort state: the element is
  the range alone, as openpyxl writes one without criteria.
- Parsing a header or footer code string, or a built-in name's
  text, in either direction.
- Any other built-in name (`_xlnm.Criteria`, `_xlnm.Extract`,
  `_xlnm.Database`, `_xlnm.Sheet_Title`): still refused by
  `add_defined_name`.
