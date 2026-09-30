# 0021: Milestone 2 part 4c, tables

Status: APPROVED by the maintainer 2026-09-30 (planning outline the
same day): decisions 1 to 5 taken as planning recommended, in the
maintainer's reply "great lets go", typed in the planning session;
decision 6 recorded. Sent to the implementing session the same day.
Context: `CLAUDE.md` ("What emend is": like openpyxl; "Dependencies
and licences": anything beyond the core behind a Cargo feature off
by default; "The fidelity harness is the acceptance instrument"),
`ROADMAP.md` ("Milestone 2", the outline approved 2026-09-23, item
4; "Part 4: the openpyxl features, in sub-parts"; "Standing items",
the Milestone 2 ceiling), `specs/0009-package-layer.md` (parts,
relationships, overrides: `add_part`, `add_relationship`,
`add_override` and their removals, the created rels part),
`specs/0011-editing-cells.md` (`set_cell`, the cell pass and its
formula refusals), `specs/0016-new-workbook-and-sheets.md` (a
sheet's chain of parts dropped with the sheet, P4 ruling 9),
`specs/0018-rows-and-columns.md` ("Creating an element", the
schema-position table; "The shift rule", where a table at or after
the point already refuses the shift; the merge overlap rule), `specs/
0019-names-links-properties-protection.md` (the feature pattern, the
`<hyperlinks>` container pass with its `r:id` and the relationships
prefix retag, decision 3 A), `specs/0020-validation-and-conditional-
formatting.md` (the worksheet container pass held to its end tag,
the `count` set rule, the P4 verification rulings 2, 4 and 6 and
the close-out rulings 8 and 10 carried here), `specs/0001-
bootstrap.md` ("Rules for every phase"). ECMA-376 Part 1, 18.5.1.2
(`table`), 18.5.1.1 (`autoFilter` in a table), 18.5.1.4
(`tableColumns`), 18.5.1.3 (`tableColumn`), 18.5.1.5
(`tableStyleInfo`), 18.3.1.94 and 18.3.1.95 (`tablePart`,
`tableParts`), 18.3.1.42 (`autoFilter` on a worksheet), 18.2.5
(`definedName`, the shared name space), 18.8.42 (`tableStyles`, the
custom styles this part reads but never writes), are cited where
the design leans on them; no text is copied from them.

## Why a part 4c

Part 4b (`specs/0020`, DONE and pushed 2026-09-30) took the two
capabilities that add rules to a worksheet. This part takes the one
that adds a part of its own: a table names a rectangle of cells
whose first row holds column names, gives it a name Excel formulas
can use, an autofilter and a banded style. A table is its own XML
part, linked from the worksheet by a relationship and a `<tablePart>`
element, so this is the first part that creates a worksheet-level
part and its chain, which `specs/0016` P4 already drops with a
sheet and `specs/0018` P2 already refuses to move. Comments and
images (4d) follow, with their own spec.

## Goal

A caller can add a table to an area of a worksheet (a name, one
column name per column, a built-in banded style or none), remove
one by name, and read the ones a worksheet has; on an opened
workbook or on `Package::new()`'s, with everything the edit does not
touch passed through byte for byte, the harness proving the change
on every corpus workbook, and the feature's cost measured beside the
core, which does not grow. What openpyxl's `Table`, `TableStyleInfo`
and `ws.add_table` do, with openpyxl's shape of the part. A totals
row and calculated columns are read back as flags and never written
(decision 5 A).

## Size and packaging (decision 2, before any code)

The budget figure after part 4b is 435224 section bytes against
the Milestone 2 ceiling of 458752 (`records/size-m2p4b-2026-09-30-
5854e34.md`): 23528 of headroom, held for fixes. The outline puts
part 4 behind features, so nothing here is in the core budget.
Planning's estimate, with part 4a's ratio in view (a guess of 15000
to 20000 against a measured 50404) and part 4b's (40000 to 60000
against 48116), is 30000 to 45000 section bytes whole: the part
writer and the allocation of a name, an id and a part name across
the workbook (8000 to 12000), the worksheet container pass with the
relationship, the override, the header cells and the overlap check
(10000 to 15000), the removal's chain (4000 to 6000), the read-back
(8000 to 12000). The core's growth for the part is what the feature
gates add, printed by the size log and expected near 0; above 2000
section bytes planning is told in the report.

- A (recommended): one Cargo feature `tables`, off by default,
  declared beside `dates`, `rowcol`, `metadata` and `rules` with no
  dependency of its own; one probe measuring its cost beside the
  core; the core probe unchanged. One feature, one figure, one
  measurement, as parts 4a and 4b chose.
- B: folded into `rules`. One feature fewer; a consumer that wants
  a drop-down list then carries the table writer, and part 4b's
  measured figure stops being a figure for anything.

## Facts planning checked (2026-09-30)

Over the 27 corpus workbooks, by element shapes and counts with
Python's `zipfile` and `ElementTree` over the table, worksheet,
rels and content-types parts, never a table's name, a column's
name, a reference or a cell's text:

- Table parts (`xl/tables/`) in 11 workbooks, 51 in all, 1 to 23
  in a workbook (the largest `id` in a workbook 1, 1, 2, 2, 2, 2,
  2, 9, 14, 16, 23); one `Override` and one `table` relationship
  each; 49 relationship targets in the absolute form
  (`/xl/tables/..`) and 2 relative (`../tables/..`); 42 roots
  prefixed, 9 in the default namespace; no byte order mark, every
  declaration `standalone="yes"`.
- Root attributes: `id`, `name`, `displayName` and `ref` on all 51,
  `name` equal to `displayName` on all 51, no `displayName` with a
  space; `totalsRowCount` 37, `totalsRowShown` 11, `headerRowCount`
  2 (never `0`), `headerRowDxfId` 31, `dataDxfId` 23,
  `headerRowCellStyle` 18, `totalsRowDxfId` 14, the `mc:Ignorable`
  and `xr:uid` pair on 49. Children: `tableColumns` 51, `tableStyleInfo`
  49, `autoFilter` 43, `extLst` 42, `sortState` 2. Columns 2 to 24
  per table, 451 in all, each with `id` and `name`; 133
  `calculatedColumnFormula` and 16 `totalsRowFormula` children,
  `totalsRowFunction` on 237 columns.
- `tableStyleInfo` names: 5 of 49 are built-in (`TableStyleLight`
  or `TableStyleMedium` with a number), 44 name a custom style held
  in the styles part's `<tableStyles>` (no `xl/tableStyles.xml`
  part anywhere). This part writes built-in names only and reads
  any name back as text.
- `<tableParts>` in 37 worksheets, `count` on 35 and equal to the
  children on every one, absent on 2; the preceding root child:
  `drawing` 15, `headerFooter` 14, `pageSetup` 3, `legacyDrawing` 2,
  `ignoredErrors` 2, `mc:AlternateContent` 1: schema order in every
  case, `tableParts` being the last main-namespace child before
  `extLst` in `WORKSHEET_CHILDREN` (part 3's table already lists
  both). 2 worksheets carry a sheet-level `<autoFilter>`, neither on
  a sheet with tables; 3 workbooks carry a `_xlnm._FilterDatabase`
  name, which a table does not need.
- Pivot table parts in 4 workbooks; the shift pass already reads a
  table part's root `ref` and a pivot's `<location ref>` through
  `part_ref` and refuses a shift at or before either
  (`TABLE_MOVED`), so every corpus table `ref` parses through
  `Area::parse`. `remove_sheet` already drops a sheet's table parts,
  their overrides and relationships with the sheet (`specs/0016` P4
  ruling 9). `Workbook::defined_names` is behind `metadata`; this
  part reads `<definedName name>` values itself for the name rule.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. Under decision 2 A
everything below sits behind a Cargo feature `tables`, off by
default: the types `Table`, `TableStyle`, `TableStyleInfo`,
`TableEntry`, three `Package` calls, with the messages of this part
in the catalogue marked as the feature's. The core builds, tests and
links without it; the tool `emend-copy` turns it on as it does
`rules`. Nothing existing changes signature. No core change: the
worksheet container pass, the part allocation and the chain removal
are the feature's own, sharing the readers the core has (`part_ref`,
the rels reader, the overrides).

### Names

A table's name is one text written as both `name` and
`displayName`, as Excel and openpyxl write a new table (every corpus
table has the two equal). The rule, Excel's for a table name, which
shares the defined names' space (18.2.5): 1 to 255 characters; the
first a letter, an underscore or a backslash; the rest letters,
digits, full stops or underscores; no space; not a cell reference in
`A1` form that lies on Excel's grid (`CellRef::parse` of the name
in upper case succeeds and the cell is within 16384 columns and
1048576 rows, part 3's `in_grid`; so `A1` and `abc1` are refused
while `Table1`, Excel's own default, and `xyz1` are names, since
`TABLE` and `XYZ` lie past column `XFD`; P1 ruling 5), not a
reference in `R1C1` form (`R`, optional digits, `C`, optional
digits, ignoring case; P1 verification ruling 4, from P2) or a
lone `C`, `c`, `R` or `r`; refused at the call otherwise ("a table
name is not valid"). At the write the name is compared, ASCII case
folded, with every table the workbook has (each worksheet's `table`
relationships' parts' `name` and `displayName`), every `definedName`
`name` in the workbook part, and every table added earlier in the
same write; a match is refused ("a table name is already in use").
The comparison is over the projected workbook: a table or sheet this
write removes is not counted, a table it adds is.

A column name is text: 1 to 255 characters, following part 4's
character rule, distinct from the table's other column names with
ASCII case folded; the list's length equals the area's width;
refused at the call otherwise ("a table column name is not valid").
Excel requires a column name to equal the text of its header cell,
so the header cells are written (decision 3 A, below).

### Types

`TableStyle`: `Light(u8)` (1 to 21), `Medium(u8)` (1 to 28),
`Dark(u8)` (1 to 11), Excel's built-in banded styles, written as
`TableStyleLight7`, `TableStyleMedium2`, `TableStyleDark1`; a number
outside its range is refused at the call ("a table style number is
out of range"). `TableStyleInfo { style: TableStyle, first_column:
bool, last_column: bool, row_stripes: bool, column_stripes: bool }`,
the four flags written as `showFirstColumn`, `showLastColumn`,
`showRowStripes` and `showColumnStripes`, `0` or `1`, always. Both
derive `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`.

`Table<'a> { name: &'a str, columns: &'a [&'a str], style:
Option<TableStyleInfo> }`; `style` `None` writes no
`<tableStyleInfo>`, which Excel shows as a table with no banding, as
openpyxl's `Table` without a `tableStyleInfo` does. Derived `Clone`,
`Copy`, `Debug`, `PartialEq`, `Eq`.

`TableEntry { name: String, display_name: String, area: Area,
columns: Vec<String>, style: Option<(String, bool, bool, bool,
bool)>, header_row: bool, totals_row: bool, calculated: bool, part:
Vec<u8> }`: `style` holds the `tableStyleInfo` name as text (built-
in or custom) with the four flags (absent flags `false`);
`header_row` is `headerRowCount` other than `0` (absent is `1`);
`totals_row` is `totalsRowCount` other than `0` (absent is `0`);
`calculated` is whether any column carries a
`calculatedColumnFormula`; `part` is the table part's entry name.
Derived `Clone`, `Debug`, `PartialEq`, `Eq`.

### The calls

- `add_table(&mut self, part, area: Area, table: Table) ->
  Result<(), Error>`: records one table on the worksheet part
  `part` (as `set_cell` names it) over `area`. At the call: the
  name, column and style rules above, and an area of fewer than two
  rows ("a table area is too small": a header row and at least one
  data row, which is the least Excel makes). The call also records
  `set_cell(part, cell, Value::Text(name))` for each column's header
  cell, the area's top row left to right, as a caller's own edit
  would (the last record for a cell in one write wins, so a later
  `set_cell` on a header cell wins, and the caller then owns the
  mismatch Excel repairs). At the write: an area that overlaps a
  cell of a table the part has (each `table` relationship's part's
  root `ref`, read as the shift pass reads it), of a table added
  earlier in the same write for the part, of a pivot table the part
  has (its `<location ref>`), of a `<mergeCell ref>` the part keeps
  or this write adds, or of the part's `<autoFilter ref>`, is
  refused ("a table overlaps a table, a pivot table, a merge or the
  autofilter"; decision 4 A, the rule Excel enforces and part 3's
  merge rule models; a kept `mergeCell` or `autoFilter` whose `ref`
  does not parse meets the same message, P1 ruling 7 (D)); a header
  cell of the area whose kept `<c>`
  holds an `<f>` is refused ("a table header cell holds a formula";
  decision 3 A: a formula silently turned into a heading is a
  surprise, and the caller clears it first if that is meant). A
  `ref` the overlap check cannot read (a relationship with no part,
  a root with no `ref`, one that does not parse) is refused ("a
  table part or its relationship cannot be read").
- `remove_table(&mut self, part, name: &str) -> Result<(), Error>`:
  records the removal of the table of that name (ASCII case folded,
  against `name` and `displayName`) from the worksheet part `part`.
  At the write: the `table` relationship whose part's root names it
  is found, and the four removals are recorded as `drop_calc_chain`
  records its three: the `<tablePart>` element whose `r:id` is that
  relationship's, the relationship, the `Override` for the part when
  the content types part has one, and the entry; none of that name
  at the write is refused ("no table of that name"), as is a name
  this write also adds for the part (the caller writes twice), a
  table named twice for removal in one write, and a removal on a
  sheet added in the same write, which has no file table (P2
  verification ruling 2). The
  cells keep their values and styles, as Excel's "Convert to Range"
  keeps them; a differential style the table's `dxfId` attributes
  named stays in `<dxfs>` (a `dxf` is never removed, part 4b). A
  `<tableParts>` left empty is dropped.
- `tables(&mut self, part) -> Result<Vec<TableEntry>, Error>`: reads
  the part's `<tableParts>` children in part order, each `r:id`
  resolved through the part's rels part, each table part's root
  and its `tableColumns` and `tableStyleInfo` read once. A
  `tablePart` with no `r:id`, an `r:id` with no relationship, a
  relationship with no part, a root with no `id`, `name`,
  `displayName` or `ref`, or a `ref` that does not parse, refuses
  the read ("a table part or its relationship cannot be read").
  Nothing is kept: the caller's snapshot, as `validations` is. A
  part with no `<tableParts>` gives an empty list.

### The part

Under `add_table` the write creates one entry per addition, in
record order, the way the package writes its created parts (part
4a's core properties part is the model): the entry name `xl/tables/
tableN.xml` with N the smallest positive integer such that no entry
of the input's archive and no addition of this write has that name,
compared ASCII case folded (an entry this write removes still
counts as taken, since the core refuses an addition under a removed
name with "an edit to a removed part"; P1 ruling 6); the `id` the
smallest positive integer no table part of the
projected workbook carries (every worksheet's `table` relationships'
parts' root `id`, and the additions before it); the `Override`
with `ContentType="application/vnd.openxmlformats-officedocument.
spreadsheetml.table+xml"` recorded through the override rule (a part
name that already has one is refused with part 1's message); the
relationship of type `TABLE` in the form of the worksheet's own
relationship in the workbook's rels part, transitional or strict,
the retag binding the same form, as part 4a writes a hyperlink's
(`specs/0019` P2 verification ruling 2; P1 ruling 3) recorded on
the part's rels part with the relative target `../tables/tableN.
xml` from the worksheet's directory, the form Excel and openpyxl
write (the corpus's absolute targets are read, never written), the
rels part created as part 4a creates one for a hyperlink when the
sheet has none. The bytes, the declaration on
its own line and the rest one line with no whitespace between
elements (broken here to fit):

```
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<table xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
 id="N" name=".." displayName=".." ref="A1:B3" totalsRowShown="0">
<autoFilter ref="A1:B3"/>
<tableColumns count="2">
<tableColumn id="1" name=".."/><tableColumn id="2" name=".."/>
</tableColumns>
<tableStyleInfo name="TableStyleMedium2" showFirstColumn="0"
 showLastColumn="0" showRowStripes="1" showColumnStripes="0"/>
</table>
```

The declaration and the root in the default namespace, no byte order
mark, no `xr` namespace and no `xr:uid`, `totalsRowShown="0"` always
(Excel's own for a table whose totals row was never shown),
`headerRowCount` omitted (the schema's default of one header row),
the `autoFilter` always with the table's own `ref`, one
`tableColumn` per name with ids 1 upward, the `tableStyleInfo`
omitted when `style` is `None`, every text escaped as part 4 escapes
a cell's. This is openpyxl's shape and what Excel writes for a new
table with no totals row, so Excel and LibreOffice open it without
repair; the harness's open check proves the opening, the Excel check
the look.

### The worksheet

In the worksheet pass, for a part with a table addition or removal:
the `<tableParts>` container is held until its end tag, as part 4b
holds `<dataValidations>` (bounded by one container: 23 elements at
most in the corpus); a kept `<tablePart>` whose `r:id` a removal
names is skipped whole; every addition is appended before the end
tag as `<tablePart r:id=".."/>`, taking the container's prefix for
the element and the root's relationships prefix for the attribute;
the `count` attribute set to the kept plus the added (added when
absent, part 4b's rule for a worksheet container; the corpus has 2
without). A container left empty is dropped; one that is absent is
created in schema position (part 3's table: immediately before
`extLst` when the part has one, else before the root's end tag); a
second is refused ("a worksheet has two tableParts elements"). A
root binding no prefix to the relationships namespace is retagged
with one, as part 4a does for a hyperlink (its decision 3 A and its
message). The header cells are ordinary cell edits of the same
write, so the cell pass writes them, and the row past the last is
appended as any new cell's is; a table edit and a row or column
shift on one part is "any other edit" of part 3's one-shift rule
and refused with that message (the shift pass's own table refusal
stands unchanged for a table the part keeps). A table edit to a
part this write replaces or removes meets part 1's message for a
cell edit, as a hyperlink does. A table on a sheet added in the
same write is refused with part 1's "no such part", as an external
link is (`specs/0019` P2 verification ruling 4; a rels part is
never created for a part the archive lacks, `specs/0009`); the
caller writes twice (P1 ruling 2).

### Creating an element

Part 3's rule and table, unchanged: `tableParts` goes immediately
before the first root child of the main namespace that comes later
in `WORKSHEET_CHILDREN`, which is `extLst` alone, or before the
root's end tag. A part whose `tableParts` comes after its `extLst`
is out of schema order and refused with the two-container message,
nothing written (part 4b's P2 verification ruling 2 applies as it
did to `dataValidations`).

### Errors

Ten new `Error::Package` messages, in the catalogue as the earlier
parts' are, one constant each:

- "a table name is not valid" (at the call)
- "a table column name is not valid" (at the call)
- "a table style number is out of range" (at the call)
- "a table area is too small" (at the call)
- "a table name is already in use"
- "a table overlaps a table, a pivot table, a merge or the
  autofilter"
- "a table header cell holds a formula"
- "no table of that name"
- "a worksheet has two tableParts elements"
- "a table part or its relationship cannot be read" (at the read
  and the write)

Reused: part 1's "an edit to a removed part" and "a cell edit to a
part this write replaces" for a table edit on a removed or a
replaced part, its "an override for that part already exists" and
"no such override" through the override rule, part 3's one-shift
message and its "a row or column index or count is out of range"
(an `Area` is on the grid by construction, so only a header cell
row past the grid meets it, which no area allows), part 4's "a text
holds a character XML cannot carry" at every name's call, part 4a's
relationships-prefix message where the retag cannot bind one, part
2's unbound-prefix refusal wherever an element this part reads
carries one, and `Error::Xml` for a value that is not UTF-8. Nothing
is checked twice.

### 32-bit and streaming

Every count is `u32` or `u64` as elsewhere; an element ordinal is
`usize`. The worksheet pass streams as today; the `<tableParts>`
container is held until its end tag. The overlap and name checks
read each table part's root start tag through `part_ref`'s form
(the first start tag alone, the part streamed no further), one
part at a time; `tables` streams each part to the end of
`tableStyleInfo` or the root. The created part is built in memory
(bounded by the column names). The read-back holds what it reads.

### Size

One probe, under decision 2 A. `probe-emend-tables-write-xml`
(feature `tables` of the probe crate, which turns on `emend/tables`
as `rules` does `emend/rules`, built alone with its own feature
set): `probe-emend-format-write-xml`'s work, unchanged, then on the
same package before its write `add_table` (one area of two columns
and three rows at a fixed place past the format probe's edits, two
column names from the command line, `Medium(2)` with row stripes),
then after the write the output opened and the table count read
back through `tables` and printed, so both sides are linked and
nothing is dead. The size log measures it as part 4b's did,
classifies the eighteen probes of part 4b's record against it, the
nineteenth, the new probe, listed as new, prints the feature's cost
(the tables probe minus `probe-emend-format-write-xml` rebuilt at
the same commit), the core's growth for the part (the format probe
at the commit minus the same probe rebuilt at 5854e34, both
printed; expected near 0), the whole growth since b728fdf (the
format probe at the commit minus 967884, probe bytes included), and
the budget figure (the format probe minus `probe-zip-xml`) against
458752, which this part does not move by design. Carried from part
4b's P4 verification ruling 2: the size log gains a control that a
code verdict with no cause fails, and lists the manifest cause
(`tools/size-probe/Cargo.toml`) apart from the crate causes.

### Harness runs

`tools/emend-copy` gains one operation, given after the two paths
as the others are; it takes no reference, name or number from its
command line, and every constant below is the tool's own. W is the
first worksheet part in `<sheets>` order and M its largest row
number as the tool's cell read gives it; a cell past the grid
refuses the sample whole with part 3's index message (no corpus
workbook has that shape). The area below is built by the tool from
M; no reference from the corpus is printed.

- `--table-sample <in> <out>` (P1, extended in P2): `add_table(W,
  (M + 1, 1):(M + 3, 2), Table { name: "emend_table", columns:
  ["emend_a", "emend_b"], style: Some(Medium(2), row stripes only)
  })`; writes; reopens the output and prints `table` with
  `tables_before`, `tables_after` (the entries `tables(W)` returns;
  `+ 1`, asserted, the output removed on a difference as
  `--style-sample` does), `parts_before`, `parts_after` (entries
  under `xl/tables/`, counted by the tool over the archive; `+ 1`,
  asserted), `rels_before`, `rels_after` (relationships of type
  `TABLE` in W's rels; `+ 1`, asserted), `container_before` (1 when
  W has a `<tableParts>`), `r_bound_before` (1 when W's root binds a
  relationships prefix), `id_after` (the added table's `id` as read
  back), `cells_before`, `cells_after` (`+ 2`, asserted); then, from
  P2, on the reopened output in memory, `remove_table(W,
  "emend_table")`, a write into a `Cursor`, and that reopened for
  `tables_removed` (equal to `tables_before`, asserted) and
  `parts_removed` (equal to `parts_before`, asserted), so the
  removal is exercised on every corpus workbook without a second
  harness output. A corpus table named `emend_table`, or a table,
  merge or autofilter on the cells the tool picks, would refuse the
  sample; M + 1 is below every cell, and the run log reports a
  refusal as a finding.

The runner and the record script are unchanged; the run names its
operation through `--tool-arg` and one `--label`. Expected results,
`lo` and `lo_in` `ok` in every case; `descriptor` alone allowed on a
changed entry; `order_preserved` `yes`; `files_identical` `no`; and
for the sample: W `changed`; W's rels part `changed`, or `added`
when the input has none; `[Content_Types].xml` `changed`; one entry
`added` (`xl/tables/tableN.xml`); every other entry `identical`.

The run log's own check over the extracted input and output parts,
with `ElementTree` and byte spans and a control that reports a
planted difference per check, printing no name, no reference and no
text from the corpus:

- W: every root child other than `sheetData` and `tableParts`
  byte-identical; `sheetData` differs by exactly one appended
  `<row>` holding two cells (the input's rows a byte-identical
  prefix), or, when the input holds an empty row M + 1, by that row
  gaining the two cells and nothing else (P1 ruling 7 (C); 2 corpus
  workbooks); the input's `tablePart` elements a byte-identical prefix
  of the output's, one more with the expected `r:id` (the log
  derives it as the rels rule does), the container's `count` equal
  to its children, created in schema position when absent (checked
  against the table).
- W's rels: the input's relationships a byte-identical prefix of
  the output's, one more of type `TABLE` with the expected relative
  target; created with the package's form when absent.
- `[Content_Types].xml`: one more `Override` with the table content
  type and the expected part name, the rest byte-identical.
- The added part: exactly the bytes "The part" gives, the log
  building them itself with N and `id` derived as the rules give
  them (the smallest unused, computed by the log over the input's
  table parts), so that every corpus workbook with tables exercises
  the allocation past 1.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. From part 8's decision
3, every phase's run log also runs the edit open check on the host
over that phase's keep. Carried from part 4b's P4 verification
ruling 4: the run log names the phase directories it compares
against by their record commit, not by a `git log --grep` of a
subject. LibreOffice's rendering of a table is not checked, only
that the file opens; the Excel check (decision 6) will show the
banding as Excel shows it, and this part's sample joins its list.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m2p4c-<phase>/` (`gate.log`,
`test.log` on both targets with the feature off and on,
`negative.log`, `run.log`, `scan.log`, `size.log` in P3); every
rule the earlier parts added (`specs/0020` "Phases" lists them and
its close-out ruling 8 carries these: every log pins the spec's
hash; a figure labelled another log's is read from that log or
labelled as measured here; an exemption counts tokens in the form,
never lines; a superseded attempt stays on disk under
`<home>/.cache/emend-build/<phase>/`; no regex stand-in is printed
in the placeholder form; planning runs no cargo while a logged
script or a pre-check runs; the push script's authorisation as
labelled inputs, change 8; the size log's cause control and the run
log's record-commit naming above); the scan log last; at most two
independent checks, each check's prompt giving the checker its
directory as `<phase>/check<n>` and saying the answer writes paths
so; report; STOP. No push during the part; the close-out push
follows P3 and carries the record commit c143473 of part 4b's
close-out with this part's.

Part 4c's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0020-validation-and-conditional-formatting.md` as planning
left them after part 4b's close-out (the close-out rulings and the
ROADMAP rows), and this spec. Part 4c's first code commit carries,
before any other change, the three items of `specs/0020`'s P4
verification ruling 6, `//!` edits to the `package` header's
catalogue: the differential style's number format, alignment and
protection "refused at the call", a style with none of the three
included; a value point's `val` holds the caller's number as the
crate prints numbers; the compose claim cites the validation and
the conditional tests.

### P1: the types, the part and the addition

The doc commit; the first code commit as above; then the feature
`tables` declared in the crate and turned on by the tool, `Table`,
`TableStyle`, `TableStyleInfo`, `TableEntry`, `tables`, `add_table`
with the name, column and style rules, the header cells, the part
writer and the allocation of the entry name and the id, the
override and the relationship, the `<tableParts>` pass with the
overlap check and the creation in schema position, the retag, the
messages of this phase (all but "no table of that name"), and
`--table-sample` without its removal. Tests on byte-built parts: the
tables read from a worksheet with two (one with a custom style name,
totals row and a calculated column, one with no style; an absolute
and a relative target) and from one with none; a `tablePart` whose
`r:id` binds nothing, a relationship whose part is missing and a
root without `ref` each refused; the added part's bytes checked
against literal bytes with and without a style; the entry name and
id skipping the ones in use (a workbook with `table1` and `table3`
gets `table2` with id 2 when ids 1 and 3 are taken); two additions
in one write taking successive names and ids; an addition to a part
with a container (appended, the kept children byte-identical,
`count` set), to the template part (created before the root's end,
the rels part created, the root retagged; the log prints the
output), to a part with `extLst` (created before it) and to a
prefixed root; the header cells written as text and the row
appended; an overlap with a kept table, a pivot, a merge, the
autofilter and an earlier addition each refused; a header cell with
an `<f>` refused; a name in use by a kept table, by a defined name
and by an earlier addition each refused, one in use by a removed
sheet's table accepted with the next free entry name and the freed
id (ruling 6); the call-time refusals one by one (an empty
name, a name with a space, `A1`, `R`, a 256-character name, an
empty column, two columns differing in case only, three names for
two columns, `Light(22)`, a one-row area; `Table1` and `xyz1`
accepted as names, ruling 5); a table edit with a shift
refused, and on a replaced part refused; the tool's line. The gate:
the crate built and tested with the feature off and on, on both
targets, and the `pub fn` and `pub const` first lines listed with it
off (the part's calls absent) and on (present), as part 4b's P1
did; every later log that runs the crate's tests runs them with the
feature on. The negative log through the binary: `--table-sample`
on a fixture from the template holding a table on the sample's
area (refused as an overlap) and on one with two `<tableParts>`
(refused), each with a control. The run log: `--table-sample` on
both targets, the checks as "Harness runs", the keep and the edit
open check, recorded as `records/m2p4c-p1-<utc-date>-<shortsha>.md`
and `.txt`.

### P2: the removal

The doc commit (the P1 verification rulings); the carry of P1's
verification, if any, as the first code commit; then `remove_table`
with its four removals and its message, the empty container
dropped, and `--table-sample`'s in-memory removal. Tests: a removal
of a present table (the element, the relationship, the override and
the entry gone, the other tables' chain byte-identical, the cells
untouched), of the last table (the container dropped), of an absent
name (refused), of a name this write adds (refused), matched with
the case folded; a removal and an addition of different names in
one write; a removal on a part whose table part is missing refused
at the write; a removal followed by an addition of the same name in
one write accepted (the name rule over the projected workbook). The
negative log: the sample on a fixture whose table relationship
names a missing part (refused), with a control. The run log:
`--table-sample` with the removal on both targets, `tables_removed`
and `parts_removed` asserted, recorded as
`records/m2p4c-p2-<utc-date>-<shortsha>.md` and `.txt`.

### P3: the catalogue and limits, the probe, the size record

The doc commit (the P2 verification rulings); the carry, if any;
then the `package` header's catalogue and limits for this part (the
messages marked as the feature's; the shapes this part writes and
reads; what it never writes: a totals row, a calculated column, a
custom style, a sort state, the `xr` attributes); `probe-emend-
tables-write-xml` with the size-probe manifest's feature `tables`;
the size log as "Size" says, recorded as `records/size-m2p4c-<utc-
date>-<shortsha>.md`; the negative log through the binary for every
message of the part reachable there, the unreachable ones stated
with the test that covers each; `--table-sample` again on both
targets with the size log's release binaries, byte-identical to
P2's run by sha256, recorded as `records/m2p4c-p3-<utc-date>-
<shortsha>.md` and `.txt`. The open record of the part's keep.

### Close-out push (after P3)

As part 4b's: the self-scan rule on the push and ci-run scripts,
`ci-run.log`, then the post-stage `scan.log` over the pushed range
with `push.log` and `ci-run.log` as its siblings, then one record
commit holding the three logs, the `ANTHROPIC_API_KEY` check; the
push script's change 8 kept (the maintainer's authorisation and its
date as labelled inputs, refused when empty); the push script's pre
mode needs a clean tree, so every planning ruling the push is to
carry is in a doc commit before it (part 4b's close-out ruling 11).
The push carries c143473 and this part's commits, on planning's
word and the maintainer's authorisation typed in the implementing
session. Before the push, the Excel check for Milestone 2 parts 1
to 4b (every corpus row of every sample, on the laptop), deferred
by the maintainer on 2026-09-27, 2026-09-29 and 2026-09-30
(`specs/0019` decision 6, `specs/0020` P4 verification ruling 10),
is run or deferred again by a decision recorded here; this part's
sample joins that check's list. Deferred again on 2026-09-30
(decision 6): the push proceeds without it. The push script prints
no maintainer decision (`specs/0020` close-out ruling 5); the
deferral's record is this spec in the pushed range, and the record
commit's message names it.

## Decisions for the maintainer

Decided by the maintainer 2026-09-30: 1 to 5 as recommended (A in
each), in the reply "great lets go" to planning's outline, typed in
the planning session; 6 recorded.

1. Order: A (recommended), 4c tables now, since the corpus has real
   tables to read back and remove and the crate already reads a
   table's `ref` and drops a sheet's tables; B, 4d comments and
   images first, which needs drawing and VML parts the crate has
   never written.
2. Packaging, as "Size and packaging" states: A (recommended), one
   feature `tables`, one probe; B, folded into `rules`.
3. Header cells: A (recommended), the add call records the column
   names as text on the header row in the same write, refusing at
   the write a header cell that holds a formula, so the file meets
   Excel's rule that a column name equals its header text; B, the
   caller sets them first and the call reads and checks them (a
   second read of the sheet); C, written as given, Excel repairs a
   mismatch on load.
4. Overlap: A (recommended), an added table is refused where its
   area overlaps a table, a pivot table, a merge or the sheet's own
   autofilter, the rule Excel enforces and part 4b's validation
   rule models; B, written as given.
5. Totals row and calculated columns: A (recommended), out of this
   part, a plain table with its autofilter and a built-in style
   name only, a custom style name refused at the call and read back
   as text; B, in, about a third more.
6. The Excel check for parts 1 to 4b: deferred on 2026-09-27,
   2026-09-29 and 2026-09-30 by the maintainer, to run before this
   part's close-out push at the latest, or be deferred again by a
   decision recorded here. Recorded, not asked. Deferred again on
   2026-09-30 ("defer excel check and i approve", typed in the
   planning session after P3's verification, before the close-out's
   doc commit): the check, now with this part's sample on its
   list, runs before the next part's close-out push at the latest
   or is deferred again by a decision recorded in the next part's
   spec.

## Planning rulings during P1 (2026-09-30)

1. The third carried phrase of `specs/0020` P4 verification ruling
   6 names a conditional compose test that does not exist: no test
   in `package/tests/conditional.rs` records a cell edit with a
   conditional format (checked by planning against 1fefcbf; the
   compose tests are the validation's, the dxf-with-cell-style one
   and the link's). Ruling: the first code commit adds the test,
   in `conditional.rs` behind `rules`, a conditional format and a
   `set_cell` on one part in one write, both written, the kept
   bytes untouched; the `//!` compose claim then cites the
   validation test and that test. The claim stays as wide as the
   pass makes it and is now borne out. A carry commit holding a
   test with its `//!` lines is within the phase rule (part 4b's
   P3 carry 03bbe82 was a test alone). The 0020 ruling's "cites
   the validation and conditional tests" stands as read this way;
   the doc commit is not remade.
2. A table on a sheet added in the same write: the spec's sentence
   pointed both ways. Ruled: refused with "no such part", as an
   external link is (`specs/0019` P2 verification ruling 4; a rels
   part is never created for a part the archive lacks). "The
   worksheet" corrected in place; a test pins it; the catalogue
   states it in P3.
3. The strict form: the spec said the transitional type always
   with part 4a's retag, which follows the sheet's form, a mixed
   pair on a strict sheet. Ruled: the relationship type and the
   retag's namespace both follow the form of the worksheet's own
   relationship in the workbook's rels part, as part 4a's ruling 2
   settled for a hyperlink. "The part" corrected in place; a
   strict-form test pins it, as 4a's does.
4. The readings impl stated stand: the name rule's reference test
   is `CellRef::parse` of the name in upper case, the defined-name
   rule's form; the defined names compared are the file's, read by
   this feature's own reader; a table part on another sheet that
   cannot be read refuses the name check with the read message; a
   root with no `id` is skipped by the id allocation (the read-back
   refuses it, as "The calls" says); the relative target is
   computed from the worksheet's directory. Recorded, no change.
5. The name rule's reference test, as ruling 4 accepted it,
   refuses `Table1`, Excel's own default table name: `CellRef::
   parse` takes any column that fits `u32`, so `TABLE1` parses.
   Ruled: a name is a reference only when the parsed cell lies on
   the grid (part 3's `in_grid`); "Names" corrected in place; tests
   pin `A1` and `abc1` refused, `Table1` and `xyz1` taken. The
   same defect is in part 4a's `check_defined_name`, which refuses
   a defined name `Table1` today: a finding about 4a, recorded
   here, fixed in P2's first code commit as the P1 verification
   carry (the same grid-bounded test, shared under both features,
   with a test that a defined name `Table1` is accepted), not in
   P1, so that P1's logs measure one change.
6. The entry-name allocation over the projected archive collides
   with the core's removed-part rule: a sheet removed in the same
   write records its table part as a removal, and an addition
   under that name is refused. Ruled (impl's (a)): an entry this
   write removes still counts as taken for the entry name, so the
   addition takes the next free one; the `id` stays projected, so
   a removed table's id is free and taken; the name rule over the
   projected workbook is unchanged. No core change. "The part"
   corrected in place; the P1 test that names a removed sheet's
   table expects the next entry name with the freed id.
7. Check 1 at the phase commit 40ae039, every log passed (given
   before the record commit). Ruled, impl's (a): no fourth code
   commit; the record commit follows on 40ae039.
   (A) `add_table`'s `///` doc still gives the pre-ruling-6 entry
   name rule ("no entry of the projected archive"; the code follows
   ruling 6): a contrary doc comment, carried to P2's first code
   commit beside ruling 5's `check_defined_name` fix, as part 4b's
   P4 verification ruling 6 carried catalogue phrases.
   (B) The tables test module's `//!` claims every test asserts the
   other parts untouched where 3 of 15 do, and the header-cell test
   appends no row after existing ones: both carried to the same
   commit (the `//!` narrowed to what holds; the test given
   existing rows before the appended one).
   (C) The header row's shape: 25 workbooks take a row appended
   after the last and 2 hold an empty row M + 1 that the cells fill.
   "Harness runs" corrected in place: `sheetData` differs by exactly
   one appended `<row>` holding the two cells, or, when the input
   holds an empty row M + 1, by that row gaining the two cells. The
   log's comparison builds the filled form and it held on both;
   its plant lands on the appended form alone, so the filled branch
   has no planted control: a control gap, carried to P2's run log
   (a plant on a filled case, or the check stated as unplanted
   where no corpus workbook lets one land). The log stands.
   (D) Recorded: no corpus workbook is strict, so ruling 3 rests on
   its crate test and the log's plant; a kept `mergeCell` or
   `autoFilter` whose `ref` does not parse is refused with the
   overlap message, now stated in "The calls"; the catalogue
   already names "no such part" for an added sheet, so P3's
   statement is in place.

## Planning verification and rulings on P1 (2026-09-30)

Verified by planning from primaries at the phase commit 40ae039
and the record commit 7176a34: the five commits over c143473 with
the doc commit's three blobs at the sent hashes, every message
ASCII with the Co-Authored-By line only, one author identity; the
record commit holding exactly the six logs and four records at the
reported hashes; every log pinning b96f97ac and printing the
working copy as not the pin; the three diffs read against
"Design", the P1 test list and the seven P1 rulings; planning's
own build of 40ae039 in a detached worktree: the crate's tests 512
off, 526 with `tables`, 575 with `metadata,tables`, 562 with
`rules,tables`, 663 all features, on both targets, 0 failed; the
workspace's 826 on both targets; clippy in five configurations and
rustdoc in three at 0 lines; its own release tools (the i686 pair
at the same hashes as the run log's, the host `emend-copy` not
bit-reproducible, as before); its own `--table-sample` run over
the 27 corpus workbooks on both targets, every output and every
printed line byte-identical to the run directory's, host and i686
outputs identical, none refused; its own check over every output
that exactly one table part is added, named and numbered by the
smallest-unused rules computed over the input, its bytes exactly
"The part"'s with the tool's constants, the only other entries
differing being the worksheet, its rels part and the content
types; its own `scripts/privacy.sh --range c143473..7176a34`, 5
commits, 832 blobs, 0 findings; `ANTHROPIC_API_KEY` unset; the
record's families (27 worksheets and 27 content-types parts
changed, 21 rels parts changed and 6 added, 27 table parts added,
every other family identical) as the sample implies.

1. The run log's attempt 1, exit 2 at its entry guard on the dry
   run's leftover records before any log line, kept as an empty
   file and named on the rerun lines: accepted as reported.
2. The independent check's answer holds one U+2212 written as `-`,
   declared in the file's header with the raw answer's size and
   hash, the raw kept on disk. Accepted, and the practice for a
   check answer the scan would refuse: the ASCII substitution
   declared with the raw's hash, nothing else changed.
3. The check's other weak points: the `rowcol`-gated shift test
   sits outside the feature-only counts (14 of the 15 tables tests,
   so 526 less 512), right by construction and pinned by the
   workspace run; the test log's control that the carry test's ok
   line is absent with the features off cannot fail, being the
   test's own gate; the report's "origin/main 116def3" is what git
   says and planning verified it, a report may state such a fact
   when no log prints it. Recorded, no change.
4. Planning's own reading of `check_table_name`: Excel also
   refuses a name in `R1C1` form (`R1C1`, `RC`, `R1C`, `RC1`, with
   any digits), which the rule as written takes, since
   `CellRef::parse` does not read that form. Carried to P2's first
   code commit beside ruling 5's `check_defined_name` fix: the one
   grid-bounded reference test, shared under both features, also
   refuses a name matching `R`, optional digits, `C`, optional
   digits, ignoring case; tests pin `R1C1` and `RC` refused for a
   table and a defined name. "Names" corrected in place.
5. P2's first code commit, the P1 verification carry, holds these
   and nothing else: ruling 5's `check_defined_name` fix with the
   test that a defined name `Table1` is accepted; ruling 4 above;
   ruling 7 (A), the `add_table` doc's entry-name sentence brought
   to ruling 6's rule; ruling 7 (B), the tables test module's `//!`
   narrowed to what holds and the header-cell test given existing
   rows before the appended one. P2's run log gains ruling 7 (C)'s
   plant on a filled case, or states the check unplanted where no
   corpus workbook lets one land. P2's doc commit carries this
   spec at the hash planning sends and `ROADMAP.md`.
6. P1 is DONE and P2 released.

## Planning verification and rulings on P2 (2026-09-30)

Verified by planning from primaries at the phase commit 6cfd504
and the record commit af43c51: the five commits over 7176a34 with
the doc commit's two blobs at the sent hashes, every message ASCII
with the Co-Authored-By line only, one author identity; the three
code diffs read before the report and byte-identical to the
committed ones; the record commit holding exactly the six logs and
four records at the reported hashes; every log pinning 3bea86d5;
planning's own build of 6cfd504 in a detached worktree: the
crate's tests 512 off, 531 with `tables`, 580 with `metadata,
tables`, 567 with `rules,tables`, 668 all features, on both
targets, 0 failed; the workspace's 831 on both targets; clippy in
five configurations and rustdoc in three at 0 lines; its own
release tools; its own `--table-sample` run over the 27 corpus
workbooks on both targets, every output and every printed line
(the removal's two counts included) byte-identical to the run
directory's, host and i686 outputs identical, none refused; its
own check of every output's added part as P1's; its own
`scripts/privacy.sh --range 7176a34..af43c51`, 5 commits, 840
blobs, 0 findings; `ANTHROPIC_API_KEY` unset; the record's
families as P1's, as the sample implies (the removal's output
stays in memory by design).

1. The carry commit 2b1cd7d holds exactly P1 verification ruling
   5's list, and was first made under a placeholder subject
   (1c1c3fc) and amended at once before anything referenced it:
   recorded; the amended commit is unreferenced and was never
   pushed, and nothing carries it.
2. Impl's readings stand and the spec says so now, in "The calls":
   a table named twice for removal in one write is refused with
   "no table of that name"; `remove_table` on a sheet added in the
   same write meets that message too, since the removal finds no
   file table first; a table part's own rels part, which no corpus
   workbook and no Excel writes, would be left by the removal:
   out of scope, stated in P3's catalogue.
3. The negative log's N6 is refused by the addition's write-time
   read before the removal's own is reached, so the removal's read
   refusal rests on its crate test: recorded; the sample adds
   before it removes by design.
4. The run log's `tables_removed` and `parts_removed` can only
   read as asserted on an ok row, the tool having asserted them,
   and the removal's output is never extracted on the corpus:
   recorded, the design of "Harness runs" (one harness output per
   workbook); the removal's bytes are pinned by the crate tests.
   The plants the check lists as absent (the relationship and
   entry removal, the removal-plus-addition, the missing part, the
   double removal; the R1C1 form on the defined-name side) are
   asserted by tests: recorded, no change.
5. Carried to P3's first code commit, `//!` and test edits only:
   the tables test module's `//!` cites P1 ruling 7 (B) where it
   cites "P1 verification ruling 5"; the removal test that claims
   the cells untouched gains a cell in its `sheetData`; a test
   pins `remove_table` on a removed part and on a replaced part
   (the messages a cell edit meets, as the addition's test has).
6. P2 is DONE and P3 released. P3's doc commit carries this spec
   at the hash planning sends and `ROADMAP.md`.

## Planning verification and rulings on P3 (2026-09-30)

Verified by planning from primaries at the phase commit 3c5f0b7
and the record commit e1f3736: the five commits over af43c51 with
the doc commit's two blobs at the sent hashes (61c125d0,
e487c725), every message ASCII with the Co-Authored-By line only,
one author identity; the three code diffs read before the report
(the carry exactly P2 verification ruling 5's list; the catalogue
`//!` lines only; the probe the format probe's source plus the
table work and its two column-name arguments); the record commit
holding exactly the seven logs, the size record and the four
records at the reported hashes; every log pinning 61c125d0;
planning's own build of 3c5f0b7 in a detached worktree: the
crate's tests 512 off, 532 with `tables`, 581 with `metadata,
tables`, 568 with `rules,tables`, 669 all features, on both
targets, 0 failed; the workspace's 832 on both targets; clippy in
five configurations and rustdoc in three at 0 lines; its own
release tools; its own size-profile builds of `probe-zip-xml`, the
format, tables, rules, metadata and rowcol probes on i686 musl at
the phase commit, each reproducing the size log's sha256 and so
its section, .text and file figures (the tables probe 1015300
sections, the format probe 968440, `probe-zip-xml` 533216), from
which the feature's cost 46860, the core's growth 0 against part
4b's 968440, the whole growth 556 against 967884 and the budget
435224 (23528 under 458752) follow by planning's own subtraction;
its own `--table-sample` run over the 27 corpus workbooks on both
targets with those release tools, every output and every printed
line byte-identical to the run directory's, host and i686
identical, none refused, and its own check of every output's
added part as P1's (the run made once more after a path error of
planning's own, the first attempt having run no binary); its own
`scripts/privacy.sh --range af43c51..e1f3736`, 5 commits, 849
blobs, 0 findings; `ANTHROPIC_API_KEY` unset; the independent
check read whole.

1. Impl's readings stand: the probe's table name and area are its
   constants, the spec fixing the area and naming no name; the
   probe's `//!` sentence "every other name ... and area comes
   from the command line" reads as exact, the two constants being
   named in the sentences above it: recorded, no change.
2. The catalogue's sentence that `remove_table` on a sheet added
   in the same write meets "no table of that name" (P2
   verification ruling 2) rests on a reading and a throwaway test
   no log printed, which is not a measurement: a test pinning it
   is carried to the next part's first code commit.
3. The feature's cost, 46860 section bytes, is above the
   estimate's 30000 to 45000 by 1860: recorded. It is outside the
   budget; the core's growth is 0 and the budget figure 435224
   is part 4b's unchanged. The metadata probe's +32, the one code
   verdict, is the P2 carry's grid-bounded defined-name rule
   (2b1cd7d, under `crates/emend/src`), a source cause the log
   names; accepted.
4. The check's weak points: two catalogue phrases are carried to
   the next part's first code commit as `//!` edits, "a header
   cell holding a formula is refused" narrowed to a kept header
   cell holding a formula (the spec's rule), and the entry-name
   comparison's ASCII case fold stated (the code folds it;
   planning read the allocation). The rest are recorded, no
   change: plant S1 bites at the removed-part half; plant S2's
   premise (the pre-carry test) was not run; the code-verdict
   accounting is per probe by cause path, the design part 4b's
   ruling 2 set; the negative log's rule is the part's own ten
   messages, and "an override for that part already exists" on
   an input holding an `Override` with no entry is the override
   rule refusing a malformed input, stated under "Errors" as
   reused; the doc commit's stat (63 insertions, 1 deletion) and
   the diff's 62 agree in the net.
5. The two findings are accepted as recorded, each fixed and
   rerun with the fix named: the run's attempt 1 (0 bytes, an
   80-byte stderr) refused at its entry guard by a status pattern
   without the size record, hashed on attempt 2's rerun lines;
   the scan's pre-check attempt 1 refused by the sibling count
   left at four, kept on disk under the phase's superseded
   directory and not committed, the final stage clean.
6. P3 is DONE. The close-out is released on the maintainer's
   decision 6, recorded here before its doc commit, and the
   maintainer's authorisation typed in the implementing session.
7. Decision 6 (2026-09-30): the maintainer, told the figures and
   rulings above, deferred the Excel check again ("defer excel
   check and i approve", typed in the planning session before the
   close-out's doc commit). Recorded under "Decisions for the
   maintainer" and in "Close-out push (after P3)". The close-out
   push proceeds without it; the check, for parts 1 to 4b's
   samples and this part's, runs before the next part's close-out
   push at the latest or is deferred again by a decision recorded
   in the next part's spec. This ruling goes in the close-out's
   doc commit with the rest of this section.

## Out of scope

- A totals row (`totalsRowCount`, `totalsRowFunction`,
  `totalsRowFormula`, `totalsRowLabel`), calculated columns
  (`calculatedColumnFormula`), a table's `dataDxfId`,
  `headerRowDxfId` and cell-style attributes, `sortState`, a
  filtered `autoFilter` (filter columns), and the `extLst` of a
  table: kept as they are on a table the part reads, never written
  (decision 5 A).
- Custom table styles (`<tableStyles>` in the styles part): read
  back by name, never written or named by an added table.
- Resizing a table, renaming one or changing its columns in place
  (remove and add); a table whose header row is off
  (`headerRowCount="0"`).
- A table's interaction with a row or column shift beyond part 3's
  refusal; moving a table.
- A sheet-level `<autoFilter>` of its own (openpyxl's
  `ws.auto_filter`); the `_xlnm._FilterDatabase` name.
- What Excel or LibreOffice shows for a table's banding: the file
  opens; the Excel check is the maintainer's.
