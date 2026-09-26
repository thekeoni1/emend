# 0018: Milestone 2 part 3, rows and columns

Status: APPROVED by the maintainer 2026-09-26 (planning draft the
same day): decisions 1 to 6 taken as planning recommended, in the
maintainer's reply "agree with all recs". Sent to the implementing
session the same day.
Context: `CLAUDE.md` ("What emend is": like openpyxl, and does NOT
fix up references on row or column insertion, by design; "The
fidelity harness is the acceptance instrument"; "32-bit and
sizes"), `ROADMAP.md` ("Milestone 2", the outline approved
2026-09-23, part 3; "Standing items", the Milestone 2 ceiling),
`specs/0011-editing-cells.md` ("Module `package`: the worksheet
rewrite", the order rule, the limits that leave `<dimension>`,
`spans`, `<cols>` and merged cells raw, all of them touched here),
`specs/0013-formulas.md` (the formula-change rule and the chain),
`specs/0016-new-workbook-and-sheets.md` (the template worksheet;
the workbook rewrite's remapping helpers), `specs/0017-fuller-
styles.md` (the close-out rulings carried here; the Milestone 2
ceiling), `specs/0001-bootstrap.md` ("Rules for every phase").
ECMA-376 Part 1, 18.3.1.99 (`worksheet`, the order of the root's
children), 18.3.1.73 (`row`), 18.3.1.13 (`col`), 18.3.1.55
(`mergeCell`), 18.3.1.66 (`pane`) and 18.3.1.87 (`sheetView`) are
cited where the design leans on them; no text is copied from them.

## Goal

A caller can give a column a width or hide it, give a row a height
or hide it, freeze panes at a cell, merge and unmerge a range, and
insert or delete rows or columns, on any worksheet of an opened
workbook or of `Package::new()`'s, with everything the edit does
not touch passed through byte for byte. This is the last part of
the Milestone 2 outline that goes in the core; the comparison
point is openpyxl's `column_dimensions[..].width` and `.hidden`,
`row_dimensions[..].height` and `.hidden`, `freeze_panes`,
`merge_cells`, `unmerge_cells`, `insert_rows`, `delete_rows`,
`insert_cols` and `delete_cols`, with openpyxl's documented limit
kept as `CLAUDE.md` states it: no formula anywhere is rewritten
when rows or columns move. What does move with them is stated
exactly ("Insert and delete" below), and what stays where it was
is stated just as exactly, so a caller knows which formulas,
names, drawings and comments now point elsewhere.

Nothing new is read back as a typed value: a column's width, a
row's height, a pane and a merge stay unmodelled on the read side,
as `<cols>` and `<mergeCells>` are today. The harness is the
acceptance: over the corpus each sample leaves every other entry
byte-identical, the worksheet identical outside the elements the
edit names, and those elements as the rule gives them, as an
independent parser in the run script shows; every output opens in
LibreOffice; and, for an insertion or deletion, emend's own reader
reads every cell back at its moved reference.

## The size question (decision 1, before any code)

The budget figure after part 2 is 399176 section bytes against the
Milestone 2 ceiling of 458752 (`records/size-m2p2-2026-09-26-
1fc8327.md`): 59576 of headroom, and the ceiling is judged at the
close of this part on the last core probe (`specs/0017`, "The size
ceiling", A). The approved outline estimated this part at about
25000 and said "where the headroom runs out; becomes a feature if
measurements say so". Planning's estimate for the design below is
30000 to 40000 section bytes: the shift rule on one axis with its
interval arithmetic (small), the worksheet rewrite gaining a second
pass shape (rows and cells retagged, elements skipped, containers
created in schema position), the `<cols>` interval splitting, the
pane and merge composers, the table and pivot reading for the
refusal, and thirteen messages. Part 1 measured 36876 against a
guess of 25000; part 2 measured 23192 against 15000 to 20000. The
guess is a guess.

- A (recommended): build in the core, in four phases with the rows
  before the columns, and measure at P2 (rows) before P3
  (columns): P2's logs rebuild `probe-emend-fullstyles-write-xml`
  and `probe-zip-xml` at P2's commit and print the whole growth
  since 1fc8327; above 40000 the maintainer re-rules before P3's
  code. The judgment of the ceiling is at P4's size record, as
  ruled. Should the record land above 458752 the maintainer
  chooses then between a feature and a higher ceiling, with the
  measurement in hand, as decision 6 of `specs/0016` did.
- B: the whole part behind a Cargo feature `rowcol` off by
  default, outside the budget figure as `dates` is, measured beside
  it on its own probe; the core is what part 2 left, and the ceiling
  is judged on `probe-emend-fullstyles-write-xml` alone, which
  passes it today. A consumer that wants a row inserted turns the
  feature on. Cheapest in risk, and the outline's own fallback.
- C: the formats, panes and merges (P1) in the core and the
  insertions and deletions (P2 and P3) behind `rowcol`. Two
  measurements and two module headers for one part; not
  recommended unless A's P2 checkpoint trips, where it becomes the
  natural re-ruling.

Taken: A, 2026-09-26. Under B or C, "Crate" below would name the
feature and the items behind it; the probe, tests and samples would
run with the feature on.

## Facts planning checked (2026-09-26)

Over the 27 corpus workbooks (26 `.xlsx`, 1 `.xltx`), by element
shapes and counts with Python's `zipfile` and `ElementTree` over
the worksheet, workbook, table and pivot parts, never a cell's
text, a formula or a name:

- 92 worksheet parts, none with a prefixed root. Every one has
  `dimension` (always a range, never one cell), `sheetViews`,
  `sheetFormatPr`, `sheetData`, `pageMargins` and `pageSetup`;
  `sheetPr` 90, `cols` 88, `mergeCells` 46, `dataValidations` 43,
  `tableParts` 37, `drawing` 37, `hyperlinks` 31,
  `conditionalFormatting` 29 (as one or several root children),
  `extLst` 12, `legacyDrawing` 9, `sheetProtection` 7,
  `ignoredErrors` 7, `phoneticPr` 4, `autoFilter` 2, one
  `dataConsolidate`, two `mc:AlternateContent`. In all 92 the root's
  children are in the schema's order; 39 names in that order are
  the table "Creating an element" uses.
- `sheetData`: 261920 `row` elements, every sheet's rows in
  increasing order, no row past 130001; every row has `r`; `spans`
  on 1104 rows only (one LibreOffice-written sheet carries 260784
  rows each with `collapsed`, `customFormat`, `customHeight`,
  `hidden`, `ht`, `outlineLevel` and `r`, so those attributes
  without `spans` are the commonest shape by count and the rarest
  by sheet). Excel's rows carry `spans` and `x14ac:dyDescent`, `ht`
  with `customHeight="1"` (about 1000), `s` with
  `customFormat="1"` (about 150), `thickBot` and `thickTop` (about
  90), `hidden` (1). A `customHeight` without `ht` does not occur.
- `cols`: 447 `col` elements, always `min`, `max` and `width`,
  `customWidth` 419, `style` 376, `hidden` 167 (160 of them the
  LibreOffice sheet's, with `collapsed` and `outlineLevel`),
  `bestFit` 9; `max` reaches 16384 (a trailing element to the last
  column) in some sheets. In every sheet the elements are in
  increasing `min` order and no two overlap; 2 sheets have a gap.
- `sheetFormatPr`: `defaultRowHeight` in all 92, `defaultColWidth`
  72, `customHeight` 80, `zeroHeight` and the outline levels 42.
- `sheetViews`: one `sheetView` per sheet; 34 with no children, 49
  with one `selection`, 9 with a `pane` (every one
  `state="frozen"` with `topLeftCell`, `ySplit` 1 to 5 and
  `activePane` `bottomLeft` or `bottomRight`; `xSplit` present in 6,
  two of them `0`) followed by 1 to 4 `selection` elements each
  naming its `pane`. A selection has `activeCell`, `activeCellId`,
  `pane` and `sqref` in 56 of 74, `sqref` alone in 12.
- `mergeCells`: 109 `mergeCell` elements over the 46 sheets, `count`
  equal to the children in every one, no single-cell merge, 80 at
  or after row 2, none straddling row 2.
- Formulas: 262538 normal, 336 shared, 5 array, no data table, in
  65 sheets; in one sheet a shared or array `ref` straddles row 2.
  9 workbooks have a `calcChain` part.
- Tables: 51 table parts over 37 sheets, 43 with their own
  `autoFilter`, 2 with the header at row 1; 8 pivot table parts over
  5 sheets, each with a `location` child carrying `ref`. Of the 27
  first worksheets, 6 have a table or a pivot table, and each of
  those 6 has one at or after row 2 and at or after column 2.
- `conditionalFormatting`: 32 `sqref` ranges; `dataValidation`: 323
  elements, 334 ranges, every container's `count` equal to its
  children; `hyperlink`: 265 `ref` values, one of them a range;
  `definedNames` in 16 workbooks, 160 names; 9 comments parts
  (every one beside a `vmlDrawing` relationship); 38 drawing parts.
- The template worksheet (`specs/0016`) is a root with `sheetData`
  alone: no `dimension`, `sheetViews`, `sheetFormatPr` or `cols`.

## Design

### Crate

Everything below is in the core, `zip` and `quick-xml` only (under
decision 1 A). No new dependency. The public surface grows by
`cells::Area` and eleven `Package` calls; nothing existing changes
signature.

### Module `cells`: `Area`

`Area { first: CellRef, last: CellRef }`, the rectangle both
corners included that `package` has used privately for an array
formula's `ref` since part 6, made public with its two corners
normalised (`first` the smaller row and column). `Area::parse(&[u8])
-> Option<Area>` reads `A1:B2` or `A1` (one cell); `Area::a1() ->
String` writes `A1:B2`, or `A1` when the corners coincide.
`Area::cells() -> u64` is the cell count, so a caller can tell one
cell from many. `Area` derives `Clone`, `Copy`, `PartialEq`, `Eq`,
`Debug`.

### Module `package`: the calls

Every call names a worksheet part as `set_cell` does; a part that
is not a worksheet, or that the write replaces or removes, is
refused at the write with the existing messages. An index is
1-based; the sheet's limits are 1048576 rows and 16384 columns
(the `xlsx` grid), and an index outside them is refused at the
call. Recording never reads the part: every check against the
file's content is the write's, in the one pass.

Formats, recorded per part and index, the last for an index
winning per field:

- `set_column_width(&mut self, part, column: u32, width:
  Option<f64>) -> Result<(), Error>`: `Some(w)` with `w` finite in
  `0..=255` writes `width="w" customWidth="1"` on the `<col>` that
  covers `column`, `None` removes `width` and `customWidth` from
  it. Out of range refused at the call.
- `set_column_hidden(&mut self, part, column: u32, hidden: bool)
  -> Result<(), Error>`: `hidden="1"` set on, or removed from, the
  covering `<col>`.
- `set_row_height(&mut self, part, row: u32, height: Option<f64>)
  -> Result<(), Error>`: `Some(h)` with `h` finite in `0..=409`
  writes `ht="h" customHeight="1"` on the `<row>`, `None` removes
  `ht` and `customHeight`.
- `set_row_hidden(&mut self, part, row: u32, hidden: bool) ->
  Result<(), Error>`: `hidden="1"` set on, or removed from, the
  `<row>`.

A number is written by `cells::number_text` (part 5's form). A
`<col>` "covers" a column when `min <= column <= max`; the write
splits it so that exactly the edited column carries the change
and every other column keeps every attribute it had (a `<col
min="1" max="5" width="9" customWidth="1" style="3"/>` edited at
column 3 becomes three elements, `1..2`, `3` and `4..5`, the outer
two byte-for-byte the original's attributes, the middle one with
the change applied). A column no `<col>` covers gets a new
element of its own, placed by `min` order, with `min`, `max` and
the edited attributes alone; a `None` width on such a column
records nothing to write (no element is created to say nothing).
An element left with `min` and `max` alone is dropped; a `<cols>`
left with no children is dropped; a `<cols>` that is absent is
created ("Creating an element"). A `<cols>` whose elements are not
in increasing `min` order or overlap is refused; a second `<cols>`
is refused. A `<row>` that is absent is created by the existing
row-creation path (`Pending::write_rows`, which now writes the
row's format attributes too) in row order, empty when no cell edit
falls in it; a `<row/>` that is present is retagged, every other
attribute and every child kept.

Panes:

- `freeze_panes(&mut self, part, at: Option<CellRef>) ->
  Result<(), Error>`: `Some(c)` freezes the rows above and the
  columns left of `c`, `None` unfreezes. `A1` is refused at the
  call ("a freeze at A1 freezes nothing"). The write replaces the
  first `<sheetView>`'s `<pane>` and every `<selection>` child (the
  pane's `activePane` and the selections' `pane` attributes go
  together, so both go) with, for `Some(c)` at row r and column k:
  `<pane xSplit="k-1" ySplit="r-1" topLeftCell="c"
  activePane="P" state="frozen"/>` followed by `<selection
  pane="P"/>`, where `xSplit` is written only when k > 1, `ySplit`
  only when r > 1, and P is `bottomRight` when both, `bottomLeft`
  when rows alone, `topRight` when columns alone (18.3.1.66; the
  corpus's frozen views take the first two forms). The rest of the
  view's children (`pivotSelection`, `extLst`) and every other
  `sheetView` are copied raw. For `None` the pane and selections
  are removed and nothing is written. A `<sheetViews>` that is
  absent is created with `<sheetView workbookViewId="0">` around
  the pane and selection (the template worksheet has none); a
  `<sheetViews>` with no `<sheetView>` child is refused; a second
  `<sheetViews>` is refused.

Merges:

- `merge_cells(&mut self, part, area: Area) -> Result<(), Error>`:
  one cell refused at the call ("a merge needs more than one
  cell"). The write appends `<mergeCell ref="A1:B2"/>` to
  `<mergeCells>` (created after `sheetData` in schema position when
  absent) and sets `count`; an area that overlaps a merge the part
  has, or one recorded earlier in the same write, is refused. The
  covered cells other than the top-left lose their value and
  formula and keep their `s`, exactly as `clear_cell` leaves a
  cell (decision 6); a covered cell inside a shared, array or data
  table range is refused with part 4's message; a value or style
  edit recorded for a covered cell other than the top-left is
  refused ("a cell edit inside a merge this write records"); a
  formula cleared this way is a formula change under `specs/0013`'s
  rule.
- `unmerge_cells(&mut self, part, area: Area)`: the write removes
  the `<mergeCell>` whose `ref` parses to exactly `area` and sets
  `count`; none refused ("no merge of that range"); a `<mergeCells>`
  left empty is dropped. Nothing is written into the cells.

Insert and delete:

- `insert_rows(&mut self, part, at: u32, count: u32)`,
  `delete_rows(&mut self, part, at: u32, count: u32)`,
  `insert_columns(&mut self, part, at: u32, count: u32)`,
  `delete_columns(&mut self, part, at: u32, count: u32)`, each
  `-> Result<(), Error>`: `count` is 1 or more and `at + count - 1`
  is within the grid, else refused at the call ("a row or column
  index or count is out of range").

One worksheet takes one insertion or deletion per write and no
other edit of any kind in that write (decision 5): a second shift
for the same part, or any cell, style, format, pane or merge edit
for it, is refused at the write with one message. A caller that
wants to insert and then fill writes twice (`write_to` into a
`Cursor<Vec<u8>>`, `open` again); the module header says so. Other
worksheets in the same write take their own edits as usual.

### The shift rule

One rule on one axis (rows or columns), applied to an index and
to an interval `[a, b]` on that axis. Insertion of `n` at `k`: an
index `i < k` is unchanged and `i >= k` becomes `i + n`; an
interval with `b < k` is unchanged, with `a >= k` moves whole, with
`a < k <= b` grows (`b + n`). Deletion of `n` at `k` (the indices
`k ..= k + n - 1`): `i < k` unchanged, `i` inside gone, `i >= k +
n` becomes `i - n`; an interval with `b < k` unchanged, with `a >=
k + n` moves whole, wholly inside gone, else cut to what survives
(`a` stays or becomes `k`; `b` becomes `b - n` or `k - 1`). A cell
reference is an index on each axis; a range is an interval on
each axis; a range gone on either axis is gone.

What the rule is applied to, in the one worksheet pass:

- `<row r>` and every `<c r>` in `sheetData`: retagged
  (`with_attribute`, the part 1 helper) with every other attribute
  and every child byte-identical; a deleted row's element and a
  deleted cell's element are skipped whole; a row left without
  cells keeps its element. A row or cell that the rule would move
  past the grid is refused ("a row or column would be moved off
  the worksheet"); an interval (`dimension`, a merge, a `<col>`,
  `spans`, a list range) that would extend past the grid is
  clipped to it, since a `<col max="16384">` means "to the end".
  The order rule of part 4 holds on the input.
- A shared, array or data table formula's `ref`: moved whole when
  the rule moves it whole; refused when the rule would grow, cut or
  remove it ("an insertion or deletion cuts a shared, array or data
  table formula"). Formula text, `si`, `r1` and `r2` are never
  touched.
- `<dimension ref>`: as an interval on both axes; an emptied
  dimension becomes `A1`.
- `<mergeCell ref>`: as an interval on both axes; one that the rule
  removes, or reduces to one cell, is dropped and `count` set; a
  `<mergeCells>` left empty is dropped.
- Columns only: `<col min max>` as an interval, an element the rule
  removes dropped, a `<cols>` left empty dropped; `<row spans>` as
  an interval, the attribute dropped when the rule removes it.
- Under decision 2 A: `conditionalFormatting sqref`,
  `dataValidation sqref`, `hyperlink ref` and `autoFilter ref`,
  each a list of ranges (a list for the first two, one range for the
  others): each range shifted or dropped; an element whose list
  empties is dropped whole (with its content), `<dataValidations
  count>` set and a `<dataValidations>` or `<hyperlinks>` left empty
  dropped.

What is refused before anything is written:

- A table or pivot table the rule would move or cut: the write
  reads each `table` and `pivotTable` relationship of the worksheet
  (their type names in the `package` catalogue beside `THEME` and
  `CALC_CHAIN`), the table part's root `ref` and the pivot part's
  `<location ref>`, and refuses when that range is not wholly before
  `k` on the edit's axis ("an insertion or deletion would move a
  table or a pivot table"). A table's `ref` binds its header row to
  its columns and Excel repairs a table whose header cells no
  longer match; moving the parts is part 4 of the outline. A table
  or pivot wholly before the point is untouched and stays right.

What is copied raw, stated in the module header as the caller's
picture (decision 4): every formula's text, in cells, defined
names, conditional formats, data validations and tables; the
`sheetViews` (a pane's `topLeftCell` and splits, a selection); rows
and columns' `outlineLevel` and `collapsed` (they travel with their
row or column, which is Excel's behaviour); comments and their VML
anchors; drawings and charts (anchored by row and column in their
own parts, they stay where they are, as openpyxl leaves them);
`extLst` content (x14 conditional formats, data validations and
sparklines carry their own ranges); `ignoredErrors`,
`protectedRanges`, `rowBreaks`, `colBreaks`, `sortState`; the
`sheetFormatPr` defaults; `spans` on a row insertion or deletion;
every defined name, including print areas and filter databases;
pivot caches. A caller inserting above a comment, a chart or a
formula's input gets openpyxl's result: the cells move, the rest
points where it did.

The formula-change rule: an insertion or deletion that moves or
removes at least one `<f>` is a formula change under `specs/0013`
(the chain dropped once when the workbook has one, `fullCalcOnLoad`
set), because the chain names cells by reference and every cached
value downstream of a moved cell may be stale. One that meets no
`<f>` at or after the point leaves both alone.

### Creating an element

`sheetViews`, `cols` and `mergeCells` can be absent (the template
worksheet has none; 4 corpus sheets lack `cols`, 46 lack
`mergeCells`). An absent element the write needs is created in
schema position: written immediately before the first root child
of the main namespace whose name comes later in the sequence of
18.3.1.99, or before the root's end tag when none does. The
sequence is one static table of the 39 names (`sheetPr`,
`dimension`, `sheetViews`, `sheetFormatPr`, `cols`, `sheetData`,
`sheetCalcPr`, `sheetProtection`, `protectedRanges`, `scenarios`,
`autoFilter`, `sortState`, `dataConsolidate`, `customSheetViews`,
`mergeCells`, `phoneticPr`, `conditionalFormatting`,
`dataValidations`, `hyperlinks`, `printOptions`, `pageMargins`,
`pageSetup`, `headerFooter`, `rowBreaks`, `colBreaks`,
`customProperties`, `cellWatches`, `ignoredErrors`, `smartTags`,
`drawing`, `legacyDrawing`, `legacyDrawingHF`, `drawingHF`,
`picture`, `oleObjects`, `controls`, `webPublishItems`,
`tableParts`, `extLst`); a root child not in it, or not of the
main namespace, is passed over. A created element takes the root's
prefix, as part 5's `numFmts` does. The rewrite of part 5 created
`numFmts` by a table of what may follow it; this generalises that
to one table and one rule, and `numFmts` is not changed.

### Errors

Thirteen new `Error::Package` messages, in the catalogue as the
earlier parts' are, one constant each:

- "a row or column index or count is out of range" (at the call)
- "a column width or a row height is out of range" (at the call)
- "a freeze at A1 freezes nothing" (at the call)
- "a merge needs more than one cell" (at the call)
- "a merge overlaps a merge the worksheet has"
- "no merge of that range"
- "a cell edit inside a merge this write records"
- "a worksheet with a row or column insertion or deletion takes no
  other edit in the same write"
- "a row or column would be moved off the worksheet"
- "an insertion or deletion cuts a shared, array or data table
  formula"
- "an insertion or deletion would move a table or a pivot table"
- "a worksheet whose cols are out of order or overlap cannot be
  edited"
- "a worksheet has two cols, sheetViews or mergeCells elements, or
  a sheetViews without a sheetView"

Reused: "a worksheet without sheetData cannot be edited" (a row
format on a part with no `sheetData`), the order and explicit-
reference refusals, "a cell in a shared, array or data table
formula cannot be edited" (a merge's clear), "a cell edit to a part
this write replaces" (every edit here on a replaced or removed
part), and the unbound-prefix refusal of part 2 wherever an element
this part retags carries one.

### 32-bit and streaming

Every index is `u32` and every count `u64`; the rule's arithmetic
is checked (`checked_add`, `checked_sub`), and the grid check makes
an overflow unreachable in practice. The worksheet pass streams as
today: one token at a time, the output through the temporary file,
no row held beyond its own tag except a deleted element's tokens
skipped as they arrive. The table and pivot refusal reads two
small parts before the pass. No row of a worksheet is held in
memory whole (P1 verification ruling 4 names the counted
containers that are).

### Size

`probe-emend-rowcol-write-xml`: `probe-emend-fullstyles-write-
xml`'s work, unchanged, then on the same package before its write
`set_column_width`, `set_column_hidden`, `set_row_height`,
`set_row_hidden`, `freeze_panes` and `merge_cells` on the sheet,
with the column, row, cell and area from the command line; after
that write, the output opened and written a fourth time with
`insert_rows` at a row from the command line, then opened and
written a fifth time with `delete_columns` at a column from the
command line, and after each the cell count read back and the
number cell's reference read back (`moved_row`, `moved_col`, as
printed indices), so both axes and both directions of the rule
are linked and nothing is dead. The size log measures it as part
2's did, classifies every probe against `records/size-m2p2-2026-
09-26-1fc8327.md`, prints the part's own cost (the new probe minus
`probe-emend-fullstyles-write-xml` rebuilt at the same commit), the
whole growth since 1fc8327, and the budget figure (the new probe
minus `probe-zip-xml`) against 458752. The P2 interim measurement
is "The size question", A.

### Harness runs

`tools/emend-copy` gains five operations, given after the two paths
as the others are; none takes a reference or a number from its
command line, and every constant below is the tool's own. W is the
first worksheet part in `<sheets>` order (as `--sheet-sample`), M
its largest row number and N its largest column number, as the
tool's cell read gives them; T is the largest last row, and U the
largest last column, over the `ref` of every table part and the
`location ref` of every pivot table part W's relationships name,
read by the tool with `quick-xml` (0 when there are none). A
refusal exits 2 with no output, as every sample does.

- `--format-sample <in> <out>` (P1): `set_column_width(W, 2,
  Some(20.5))`, `set_column_hidden(W, 3, true)`,
  `set_row_height(W, 2, Some(30.0))`, `set_row_hidden(W, 3, true)`,
  `freeze_panes(W, Some(B2))`, `merge_cells(W, (M + 1, 1)..(M + 1,
  2))` (a merge below the data, so no cell is cleared and the check
  is structural); writes; reopens; prints `format` with
  `cells_before`, `cells_after` (equal, asserted as
  `--style-sample` asserts, the output removed on a difference),
  `rows_before`, `rows_after` (row elements as the reader counts
  them; `rows_before + (rows 2 and 3 that were absent)`, asserted),
  `merges_before`, `merges_after` (`mergeCell` elements, counted by
  the tool with `quick-xml`; `+ 1`, asserted).
- `--insert-rows-sample <in> <out>` (P2): k = max(2, T + 1);
  `insert_rows(W, k, 2)`; prints `insert_rows` with `at` k,
  `count` 2, `cells_before`, `cells_after` (equal, asserted),
  `from_k_before` (cells at row k or after in the input),
  `from_k2_after` (cells at row k + 2 or after in the output;
  equal, asserted), `formulas_moved` (input cells with a formula
  at row k or after), `chain_before`, `chain_after` (1 or 0, the
  workbook's `calc_chain`), `full_calc` (the output's flag).
- `--delete-rows-sample <in> <out>` (P2): the same k;
  `delete_rows(W, k, 1)`; prints `delete_rows` with `at`, `count`
  1, `cells_before`, `cells_after` (`cells_before` minus the input's
  cells at row k, asserted), `from_k1_before` (cells at row k + 1
  or after), `from_k_after` (equal, asserted), `formulas_moved`
  (input formulas at row k or after), `chain_before`,
  `chain_after`, `full_calc`.
- `--insert-columns-sample <in> <out>` (P3): c = max(2, U + 1);
  `insert_columns(W, c, 1)`; prints `insert_columns` with `at` c,
  `count` 1 and the counts as the row samples, over columns.
- `--delete-columns-sample <in> <out>` (P3): the same c;
  `delete_columns(W, c, 1)`; as `--delete-rows-sample`, over
  columns.

k and c step past every table and pivot so the sample applies on
the 6 first sheets that have one; the refusal itself is the
negative log's on a fixture. A shared or array `ref` straddling k
or c makes the sample refuse that workbook: the run log computes,
from the input, which workbooks the rule refuses and expects
exactly those to exit 2 with no output (the runner's existing
handling of a tool failure, the outcome then `refused` in the
record rather than a failure), and the corpus fact above says at
most one sheet is such at row 2.

The runner and the record script are unchanged; each run names its
operation through `--tool-arg` and one `--label` per run
directory, two run directories for the two-sample phases (`specs/
0017` P3 ruling 1). Expected results, `lo` and `lo_in` `ok` in
every case; in every workbook W `changed`, every other entry
`identical` unflagged except: for the two row samples and the two
column samples, when `formulas_moved` is above 0 and the input has
a chain, the chain entry `removed` and the workbook part, its rels
part and `[Content_Types].xml` `changed` as `--formula-sample`
expects them; `descriptor` alone allowed on a changed entry;
`order_preserved` `yes`; `files_identical` `no`. The run log's own
check over the extracted input and output parts, with
`ElementTree` and byte spans and a control that reports a planted
difference per check, printing no reference from the corpus, no
text and no formula:

- P1: every root child of W other than `sheetViews`, `cols`,
  `sheetData` and `mergeCells` byte-identical; the first
  `sheetView`'s start tag identical, its children exactly the pane
  and selection elements the design gives for B2 (the log builds
  the expected bytes itself) followed by its input children other
  than `pane` and `selection` byte-identical, every other
  `sheetView` identical; `cols` modelled as intervals: for each
  column 1 to the largest `max` seen, the effective attributes
  equal the input's except column 2 (`width` 20.5, `customWidth`
  1) and column 3 (`hidden` 1), the elements in increasing `min`
  order and disjoint, and every element's attribute order as the
  design gives it; `sheetData`: every row other than 2 and 3
  byte-identical, rows 2 and 3 with the input's attributes and
  children and the design's additions, created when absent;
  `mergeCells`: the input's children a prefix, one more with the
  expected `ref` and `count` grown by one, created in schema
  position when absent (the log checks the position against the
  table).
- P2 and P3: every root child other than the elements the shift
  rule names byte-identical; every row before the point
  byte-identical; every row at or after the point with its number
  as the rule gives, every other attribute identical, every cell's
  `r` as the rule gives and every other byte of the cell identical
  (the check retags with its own `ElementTree` model and compares
  bytes), a deleted row or cell absent; `dimension`, every
  `mergeCell` and (decision 2 A) every `sqref` and `ref` as the
  rule gives, dropped when the rule drops them, counts right;
  `spans` as the rule gives on the column samples; the chain and
  the flag exactly when `formulas_moved` is above 0 and the input
  has a chain; the expected-refusal set computed and matched.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. From part 8's decision
3, every phase's run log also runs the edit open check on the host
over that phase's keep.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m2p3-<phase>/` (`gate.log`,
`test.log` on both targets, `negative.log`, `run.log`, `scan.log`,
`size.log` in P2 and P4); every rule the earlier parts added
(`specs/0017` "Phases" lists them, and its close-out ruling 4
carries four more: the unbound-prefix case in the first negative
log; a size log rerun after the run log prints, as a check, that
its hash lines equal those of the attempt the run log cites; a
moved keep listed by its pair count and the hash of its listing; the
size log installs its record and prints the installed file's
sha256; and a comment in a script never spells a placeholder-shaped
path it does not declare); the scan log last; at most two
independent checks; report; STOP. No push during the part; the
close-out push follows P4 and carries the record commit d0f8f72 of
part 2's close-out with this part's.

Part 3's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0017-fuller-styles.md` as planning left them after part 2's
close-out (the close-out rulings and the two ROADMAP edits), and
this spec.

### P1: `Area`, the formats, panes and merges, creating an element

The doc commit; then `Area` public, the schema-position table and
creation rule, the four format calls with the `<cols>` splitting
and the row retagging and creation, `freeze_panes`, `merge_cells`
and `unmerge_cells` with the merge's clear, the messages of this
phase, and `--format-sample`. Tests on byte-built parts: a `<col>`
split at its first, middle and last column and at a one-column
element; a width `None` on a column no element covers (nothing
written) and on a covered one (the element split, the piece
dropped when bare); a column past every element (a new element
placed by `min`); `<cols>` created before `sheetData` in the
template part, dropped when its last child goes, refused when out
of order, overlapping or doubled; a row retagged with `ht` and
`customHeight`, then `None` removing both, `hidden` set and
removed; a row created empty in order among the file's rows, and
one created that a cell set also fills (one element); the pane for
B2, B1, A2 (the three `activePane` forms and the omitted split),
`None` removing pane and selections, a view with an `extLst` child
kept after the pane, `sheetViews` created in the template part,
refused with no `sheetView`, refused when doubled; a merge
appended with `count`, created in schema position before each of
`phoneticPr`, `conditionalFormatting` and `pageMargins` and at the
root's end, the covered cells cleared with `s` kept and the
top-left untouched, a formula in a covered cell removed and the
chain rule triggered, a covered cell in an array range refused, a
set inside a pending merge refused, an overlap with the file's
merge and with a pending merge refused, one cell refused at the
call; unmerge of a present range, of an absent one (refused), the
last merge dropping the container; the number forms of a width and
a height; the range refusals at the call; the part 4, 5 and 6
worksheet tests unedited and passing; the tool's line. The
negative log through the binary: the unbound-prefix case carried
from part 2 (`--font-sample` on a fixture from the template with
an `xf` child whose prefix no declaration binds, exit 2, the
message, no output), `--format-sample` on a fixture whose `<cols>`
overlap (refused) and on one with two `<mergeCells>` (refused),
each with a control. The run log: `--format-sample` on both
targets, the checks as "Harness runs", the keep and the edit open
check, recorded as `records/m2p3-p1-<utc-date>-<shortsha>.md` and
`.txt`.

### P2: rows inserted and deleted, the interim measurement

The doc commit (the P1 rulings and verification); then the shift
rule as one function over an axis, its application to rows and
cells, `<f ref>`, `dimension`, merges and (decision 2 A) the four
list attributes, the table and pivot reading and refusal, the
one-shift rule, the formula-change rule, the messages of this
phase, and the two row samples. Tests: the rule's arithmetic on
indices and intervals for every case named in "The shift rule",
pinned as a table the log prints; a byte-built sheet with rows
before, at and after the point, each retagged or skipped as the
rule says, attributes and children byte-identical; a `<row/>` and
a `<c/>` (empty elements) shifted; a deleted row with cells gone
whole; a shared `ref` before the point (untouched), after (moved),
straddling (refused), inside a deletion (refused), the master
deleted with its whole range (allowed); an array likewise; a
dimension moved, grown, cut and emptied to `A1`; a merge moved,
grown, cut, dropped when removed and when reduced to one cell,
`count` following, the container dropped when empty; each list
attribute moved, cut, a range dropped from the list, the element
dropped when the list empties, `dataValidations count` following;
a table wholly before (allowed), at (refused), after (refused), a
pivot likewise, on parts built with the relationship and the part;
a chain dropped and the flag set when an `<f>` moves, neither when
none does, neither when the only `<f>` is before the point; a row
moved off the grid refused; a second insertion for the part and a
cell set for the part refused with the one message; the part 4 and
P1 tests unedited; the tool's two lines. The negative log through
the binary: `--insert-rows-sample` on a fixture from the template
with a table part whose `ref` covers row 2 (refused), on one with
a shared formula straddling row 2 (refused), on one with a cell at
row 1048576 (refused), each with a control. The run log: both row
samples on both targets in two run directories, the checks as
"Harness runs", recorded as `records/m2p3-p2-insert-<utc-date>-
<shortsha>.md`, `records/m2p3-p2-delete-...` and `.txt`. The size
log: `probe-emend-fullstyles-write-xml` and `probe-zip-xml` rebuilt
at the phase commit, the whole growth since 1fc8327 printed against
the 40000 checkpoint of decision 1 A, no record file; the report
states the figure.

### P3: columns inserted and deleted

The doc commit (the P2 rulings and verification, and the
maintainer's checkpoint ruling if the figure tripped it); then the
rule applied on the column axis to cells, `spans`, `<col>`
intervals, `dimension`, merges and the list attributes, and the two
column samples. Tests: cells retagged across every row, a deleted
column's cells gone and its rows kept, `spans` moved, grown, cut
and dropped, `<col>` moved, grown, cut, dropped and the container
dropped, a `<col max="16384">` on an insertion (clipped, not
refused) beside a cell at column 16384 (refused: moved off the
grid), the table and pivot refusal on the column axis, the
one-shift rule between a row and a column shift; the P2 tests
unedited; the tool's two lines. The negative log: the column forms
of P2's three fixtures. The run log: both column samples in two run
directories, recorded as `records/m2p3-p3-insert-...` and
`records/m2p3-p3-delete-...`.

### P4: the catalogue and limits, the probe, the size record

The doc commit (the P3 rulings and verification); then the
catalogue and the limits in the `package` and `cells` module
headers (the caller's picture: what moves, what stays, the
two-writes pattern, what is not read back, the Excel check
pending), `probe-emend-rowcol-write-xml` measured as "Size" says,
recorded as `records/size-m2p3-<utc-date>-<shortsha>.md` with
`size.log` printing every figure the record carries, installing
the record and printing its sha256, and the fifteen probes
classified against part 2's record; the judgment of the Milestone
2 ceiling stated in the record as a fact (under or over 458752,
by how much); the negative log through the binary with every
refusal of this part reachable through the binary run once (the
six fixture cases of P1 to P3, the doubled-element and no-
`sheetView` cases; the at-call refusals are not reachable, since
the tool's constants are fixed, and the log says so); the corpus
run of all five samples again on both targets with the release
binaries the size log hashed, every output byte-identical to the
phase run directories' outputs by sha256, in five run directories,
recorded as `records/m2p3-p4-<sample>-<utc-date>-<shortsha>.md`
and `.txt`. The report states the part's own cost, the whole
growth, the budget figure against the ceiling, and the Milestone 2
result.

### Close-out push (after P4)

As part 2's, with the self-scan rule on the push and ci-run scripts
(`specs/0001`), one record commit after the push, the scan rerun
last, the `ANTHROPIC_API_KEY` check. The push carries d0f8f72 and
this part's commits, on planning's word and the maintainer's
authorisation typed in the implementing session. The Excel check
for Milestone 2 (every corpus row of every sample of parts 1 to 3,
on the laptop) is the maintainer's, after the push.

## Decisions for the maintainer

Decided by the maintainer 2026-09-26: 1 as A, 2 as A, 3 to 6 as
recommended.

1. The size question, as "The size question" states: A, the core in
   four phases with the interim measurement after P2 at a 40000
   checkpoint and the ceiling judged at P4 (recommended); B, the
   whole part behind `rowcol`; C, P1 in the core and the shifts
   behind `rowcol`.
2. What the shift rule moves besides the grid: A (recommended),
   `dimension`, merges, `cols`, `spans`, shared and array `ref`s,
   and the four list attributes (`conditionalFormatting sqref`,
   `dataValidation sqref`, `hyperlink ref`, `autoFilter ref`), so a
   validation, a conditional format, a link and a filter stay on
   the cells they were on, at a cost planning guesses at 3000 to
   5000 section bytes for the list handling; B, the first five
   only, the four attributes copied raw and stated in the limits,
   which is what openpyxl does and what its users report as a
   trap. Either way `sheetViews` and `extLst` stay raw.
3. Tables and pivot tables: refused when the edit would move or cut
   one, allowed when wholly before the point (recommended), the
   table parts' own rewrite left to part 4; the alternative copies
   them raw, and a table whose `ref` no longer covers its header is
   a file Excel repairs, which the harness cannot see until the
   laptop check.
4. Comments, drawings, views, `extLst`, defined names and every
   formula copied raw and stated in the limits (recommended; the
   openpyxl comparison, and the `CLAUDE.md` definition for the
   formulas); the alternative refuses an insertion or deletion on a
   worksheet with a comments relationship whose comment would move,
   which reads one more part for a case openpyxl does not refuse.
5. One insertion or deletion per worksheet per write, and no other
   edit to that worksheet in the same write (recommended): every
   edit is then in the input file's coordinates and nothing needs
   translating; the alternative applies later edits in the shifted
   grid, sequentially as openpyxl does in memory, which needs every
   recorded reference of the part translated through each shift
   and an order rule in the header.
6. A merge clears the covered cells other than the top-left, `s`
   kept (recommended; Excel and openpyxl both do); the alternative
   leaves them, and the hidden values come back on unmerge.

## Planning rulings during P1 (2026-09-26)

Ruled on the implementing session's hold before its first code
commit, after planning verified the doc commit fd6bbf8 (parent
d0f8f72, the three blobs at the hashes sent):

1. `rows_before` and `rows_after` (Q1): the reader yields cells and
   counts no rows, so the tool counts `<row>` elements in the first
   main-namespace `sheetData` with `quick-xml`, as it counts
   `mergeCell`, and records whether rows 2 and 3 were present for
   the assertion. "As the reader counts them" in "Harness runs" was
   planning's slip; this ruling replaces it.
2. Creation against an input out of schema order (Q2): the pass
   creates `cols`, `sheetViews` or `mergeCells` before the first
   later-sequence child, and an element of that name met afterwards
   in the input is refused with the doubled-elements message ("a
   worksheet has two cols, sheetViews or mergeCells elements, or a
   sheetViews without a sheetView"). No corpus sheet is out of
   order; the refusal is the defence.
3. A malformed `<col>` (Q3): one lacking `min` or `max`, a `min` or
   `max` that is not a decimal `u32`, a `min` of 0 or a `min` above
   `max`, when the write edits `cols`, is refused with "a worksheet
   whose cols are out of order or overlap cannot be edited". No new
   message; the module header lists the forms.
4. A `<mergeCell>` whose `ref` does not parse as an `Area` (Q4),
   when the write merges or unmerges on that part, is refused, and
   the message is widened so it says why: "a merge overlaps a merge
   the worksheet has, or a mergeCell ref does not parse". "Errors"
   reads with this text.
5. Merge and unmerge in one write (Q5): an unmerge matches the
   file's merges only, and unmerging a pending merge is "no merge
   of that range"; a merge is checked for overlap against the
   file's merges less those this write unmerges, and against the
   merges recorded before it; two unmerges of one area remove it
   once. As proposed.
6. `unmerge_cells` returns `Result<(), Error>` (Q6), refusing an
   area outside the grid at the call; `merge_cells` and
   `freeze_panes` refuse an out-of-grid area or cell at the call,
   all three with "a row or column index or count is out of range";
   an `Area` given with unnormalised corners is normalised at the
   call. "Module `package`: the calls" reads with this.
7. Removed against replaced parts (Q7): the new edits join
   `cell_parts` and take the two existing messages as cell edits
   do, "an edit to a removed part" for a removed part and "a cell
   edit to a part this write replaces" for a replaced one. The
   sentence in "Errors" that put both under the second message was
   planning's, wrong; this ruling corrects it.
8. The merge's clear (Q8): confirmed. It meets the cells present in
   the file, writes nothing for an absent one, and refuses a
   covered cell carrying an attribute other than `r`, `s` and `t`
   with "a cell carries an attribute this layer does not model",
   exactly as `clear_cell` does.
9. A `<sheetViews>` with no `<sheetView>` (Q9): refused for `None`
   as for `Some`, the check being structural. Confirmed.

## Planning verification and rulings on P1 (2026-09-26)

Verified by planning from primaries, never from the report alone:

1. Commits, from git: fd6bbf8 (docs, the three files at planning's
   hashes), d03ed7b (the crate), b8595ce (the tool, the phase
   commit), 2e54f33 (the record, ten files), a chain from d0f8f72;
   every message ASCII, ending with the Co-Authored-By line, no
   session link; each commit planning docs alone, code alone or
   records alone; nothing pushed. The six committed logs and the
   four record files at the sha256 the report gives. Planning's own
   `scripts/privacy.sh --range d0f8f72..HEAD`: 4 commits, 641
   blobs, 0 findings; no log names the home directory (the two
   `/home/` strings in the scan and run scripts are a control
   fixture's text and a grep pattern); `ANTHROPIC_API_KEY` unset.
2. The logs at their verdict lines, every one attempt 1 (no rerun
   header): gate "checks all as expected: yes"; test "yes", 665
   passed on each target, 0 failed, 2 ignored, 31 added; negative
   "yes", N1 the carried unbound-prefix case, N3 and N4 the two
   fixture refusals, exit 2 and no output each, the controls
   turning; run "yes", 27 of 27 on both targets, its comparison's
   flags all yes with 12 of 12 plants turning their own flag, the
   host and i686 outputs equal, the keep 266 pairs with part 2 P3's
   220 byte for byte and 46 new all from this phase's tests, the
   open record 0 findings; scan final stage PASS, 0 hits. Each log
   counts its own stderr at 0 bytes with the one-byte control. The
   spec pinned at 09c5c539 in gate, test, run and scan.
3. Planning's own measurements: the workspace tests on planning's
   build at b8595ce, 665 passed, 0 failed, 2 ignored on the host and
   the crate's tests passing on i686; clippy with `-D warnings` and
   `cargo doc` clean; planning's own comparison reader over the run
   directory (a byte-span model of the root's children, the pane
   and selection bytes, the `cols` intervals' effective attributes,
   rows 2 and 3, the cells, the merge appended with its `count` and
   its schema position), 27 of 27 on both labels with no problem,
   a planted change to the pane caught, host and i686 outputs equal
   in 27 of 27; its created counts (4 rows, 16 `mergeCells`, no
   `sheetViews` or `cols`) equal to the run log's per label.
4. Check 1's F3: the rewrite holds `<mergeCells>` whole because its
   `count` sits in its start tag, and `<cols>` whole by choice. The
   sentence in "32-bit and streaming" that nothing of a worksheet is
   held whole was planning's, wrong for a counted container; ruled:
   a root child the write counts or reorders (`cols`, `mergeCells`
   and, in P2, `dataValidations`) is held whole, bounded by that
   element's own size and never by the sheet's rows; the sentence
   reads "no row of a worksheet is held in memory whole", and the
   module header says which elements are.
5. Check 1's F4: `with_attribute` replaces a present attribute where
   it stands, so a row with `customHeight` and no `ht` comes out
   with `ht` after it; the design's order applies to attributes the
   write adds. Accepted as `with_attribute`'s rule (every other
   byte kept); no corpus row has the shape.
6. Check 1's F6 and F7: `Area::cells` multiplies two `u64` that a
   hand-built area can make 2^32 each; ruled `saturating_mul` with
   a test, and the write assertion the comment in the indices test
   claims, both in P2's first code commit. F5: a test that a sheet
   removal drops the part's format, pane and merge edits, in P2.
   F8 (the tool counts through `emend::xml::Stream`, quick-xml
   underneath, as `--sheet-sample` does) and F9 (one control for
   N3 and N4) are remarks, accepted.
7. Check 1's F14 to F18 are report wording, answered in the
   implementing session's message; from P2 a report states each
   log's attempt number outright rather than by the absence of a
   rerun line.
8. The implementing session reports one unlogged dry run of a
   script that killed itself through `pkill -f`, repeated once and
   recorded in its gotchas; nothing was lost and no log cites it.
   Noted; a dry run of a script that kills processes is run once,
   by pattern-checked pid, and never again by pattern.
9. P1 is DONE. P2 follows "P2" with rulings 4 to 6 in its first
   code commit.

## Out of scope

- Reading widths, heights, panes or merges back as typed values.
- Styles on rows and columns (`<row s>`, `<col style>`): kept when
  they exist, never set. A later part if a caller needs it.
- Outline grouping (`outlineLevel`, `collapsed`), row and column
  breaks, `sheetFormatPr` defaults, split (unfrozen) panes,
  selections.
- Moving tables, pivot tables, comments, drawings, defined names,
  conditional formats' and validations' formulas, or any formula
  text, on an insertion or deletion: not done by `CLAUDE.md`'s
  definition; the table and pivot parts' own `ref` is part 4's.
- Two shifts on one worksheet in one write, or a shift with other
  edits: write twice.
- Formula evaluation, rendering, autofit and reference fixup, by
  `CLAUDE.md`'s definition.
