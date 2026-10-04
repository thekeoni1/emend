# 0026: Milestone 3 part 4, outline grouping and row and column styles

Status: APPROVED by the maintainer 2026-10-04 ("A on all
(Recommended)", chosen in the planning session's question:
decisions 1 to 6 as planning recommended, 7 recorded). Sent to the
implementing session the same day.

Context: `CLAUDE.md` ("What emend is": like openpyxl; "Dependencies
and licences": anything beyond the core behind a Cargo feature off
by default; "The fidelity harness is the acceptance instrument"),
`ROADMAP.md` ("Milestone 3", the outline approved 2026-10-01, item
4: outline levels and collapsed state, a style on a whole row or
column, kept through part 3's shifts; the feature `rowcol` grows;
20000 to 30000), `specs/0018-rows-and-columns.md` ("Module
`package`: the calls": the row and column formats, the `<col>`
split rule, the row creation path; "The shift rule": `outlineLevel`
and `collapsed` travel with their row or column; "Out of scope":
styles on rows and columns and outline grouping left to a later
part), `specs/0012-styles.md` (`set_cell_style` and the style
index check at the write), `specs/0025-sheet-and-print-settings.md`
("The pass": the settings pass with its attribute merge, `<sheetPr>`
and its children in 18.3.1.82's order; the close-out rulings 2 to
4, carried here), `specs/0001-bootstrap.md` ("Rules for every
phase"). ECMA-376 Part 1, 18.3.1.73 (`row`: `outlineLevel`,
`collapsed`, `hidden`, `s`, `customFormat`), 18.3.1.13 (`col`:
`outlineLevel`, `collapsed`, `hidden`, `style`), 18.3.1.81
(`sheetFormatPr`: `outlineLevelRow`, `outlineLevelCol`), 18.3.1.60
(`outlinePr`: `summaryBelow`, `summaryRight`) are cited where the
design leans on them; no text is copied from them. openpyxl's
`RowDimension` and `ColumnDimension` (`outline_level`, `hidden`,
`collapsed`, a style through the dimension), `DimensionHolder.group`
and `sheet_properties.outlinePr` are the behavioural reference,
read only.

## Goal

Outline levels on rows and on columns, with the hidden and
collapsed flags that make a group fold; the sheet's outline
maximums and its summary position; a style on a whole row or a
whole column; each written through the row and column format path
of `specs/0018` and the settings pass of `specs/0025`, kept
through a row or column shift as the attributes already travel.
Two corpus samples with the open checks; the `rowcol` probe grown;
the catalogue; the close-out push. Everything behind the Cargo
feature `rowcol`, which grows as the outline says. Writes only:
the typed read-back is part 5's.

## Size and packaging (decision 1, before any code)

The budget figure after part 3 is 436456 section bytes against
the ceiling of 458752 (`records/size-m3p3-2026-10-03-01b14c0.md`):
22296 of headroom, held for fixes. `rowcol` costs 35248 after
part 3. The outline's estimate for this part is 20000 to 30000;
planning's, priced from parts 2 and 3 (`richtext` 12672 for a
write branch and a reader; `sheetprops` 28148 for seven merged
elements and a pass), is 10000 to 20000 section bytes of growth
of `rowcol`: four fields on the existing line-format path, a
column coalescing step, two merged elements through the pass
already written. The core's growth is what the feature gates add,
expected near 0; above 2000 section bytes, or a growth of `rowcol`
above 25000, planning is told in the report.

- A (recommended): the feature `rowcol` grows, as the outline
  says, and `probe-emend-rowcol-write-xml` grows with it: its
  cost over the format probe after this part, less 35248, is the
  part's figure, printed by the size log. No new feature, no new
  probe; a consumer that shifts rows carries grouping too, which
  openpyxl's dimensions bundle the same way.
- B: a new feature `outline` with its own probe, 26 probes; the
  line-format path then carries two features' fields.

## Facts planning checked (2026-10-04)

- The corpus, counted by planning with Python's `zipfile` and
  regular expressions over every regular file of the directory
  (27 workbooks, 95 worksheet parts), names and counts only: no
  `<row>` or `<col>` carries `outlineLevel` above 0 (260843 rows
  and 172 columns carry `outlineLevel="0"`, LibreOffice's habit
  of writing every attribute), none carries `collapsed="1"`; no
  `<outlinePr>` anywhere; every `sheetFormatPr` with the outline
  maximums has them at 0 (45 parts); so the corpus holds no
  grouping and the samples write the first witnesses. Rows: 7
  hidden; 140 rows in 6 workbooks carry `s` with
  `customFormat="1"`, 20 carry `customFormat="1"` with no `s`,
  none carries `s` without `customFormat`. Columns: 11 hidden;
  388 `<col>` in 26 workbooks carry `style`, 296 of them above 0;
  the commonest attribute orders are `min, max, width, style,
  customWidth` (176) and LibreOffice's alphabetical one (172).
- `set_column_width`, `set_column_hidden`, `set_row_height` and
  `set_row_hidden` record a `LineField` per part and index
  (`line_format`), the last per field winning (`LineFormat`); the
  write applies them to a `<row>` by `line_formatted` (an
  attribute set or removed, every other byte kept) and to a `<col>`
  by the split rule of `specs/0018` (`cols_content`), creating a
  row or a `<col>` that is absent. All four are in the core;
  `insert_rows` and the rest are the feature's.
- `set_cell_style` records an index checked at the write against
  the styles part's `cellXfs` count ("a style index is out of
  range"), and `add_style` returns the index it appends.
- The settings pass of `specs/0025` (`SettingsPass`, `merged_tag`,
  `SHEET_PR_CHILDREN` in 18.3.1.82's order `tabColor`,
  `outlinePr`, `pageSetUpPr`) is `cfg(feature = "sheetprops")`;
  `sheetFormatPr` is a root child the rewrite copies raw.
- The shift rule copies `outlineLevel`, `collapsed`, `s`,
  `customFormat` and `style` with their row or column (`specs/
  0018`, "The shift rule", decision 4), so a grouped or styled row
  moves whole.
- Excel's outline: at most 7 levels (`outlineLevel` 1 to 7); a
  group's rows carry the level and, when folded, `hidden="1"`; the
  summary row after (or before, by `summaryBelow`) the group
  carries `collapsed="1"`; `sheetFormatPr`'s `outlineLevelRow` and
  `outlineLevelCol` hold the sheet's maximums, which Excel uses to
  size the outline bar. Planning's reading of the format; the
  corpus has no witness, and the tests and the LibreOffice open
  check are the proof this part has.
- openpyxl, planning's reading of 3.1, read only:
  `ws.row_dimensions.group(start, end, outline_level=1,
  hidden=False)` sets `outline_level` and `hidden` on each row in
  the range; `ws.column_dimensions.group("A", "D", hidden=True)`
  likewise, writing one `<col>` for the range; `collapsed` is a
  dimension attribute the caller sets; the writer puts the
  maximum levels into `<sheetFormatPr>`; a style on a dimension is
  written as the row's `s` with `customFormat` or the column's
  `style`; `ws.sheet_properties.outlinePr` carries `summaryBelow`
  and `summaryRight`.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. Under decision 1 A,
everything below sits behind the Cargo feature `rowcol`: the new
fields of `LineField` and `LineFormat` and their calls are gated
variants and methods, absent when the feature is off; the
settings pass's machinery for `<sheetPr>` and the attribute merge
becomes `cfg(any(feature = "sheetprops", feature = "rowcol"))`,
one definition, so `<outlinePr>` and `<sheetFormatPr>` go through
the pass already measured. The core builds, tests and links
without it; the tool turns it on. Nothing existing changes
signature.

### The calls

Each records an edit on the worksheet part `part` as the format
calls of `specs/0018` do: an index outside the grid, a level
outside 0 to 7 or a range with `first > last` refused at the call
("a row or column index or count is out of range" for the index
and the range, "an outline level is out of range" for the level),
nothing recorded; the part never looked up at the call; a part
the archive lacks, removed or replaced, refused at the write with
part 1's messages; a shift on the part in the same write refused
with part 3's one-shift message, as every format edit is. The
last call for an index wins per field.

- `group_rows(part, first: u32, last: u32, level: u8, hidden:
  bool)`: on each `<row>` from `first` to `last`, `outlineLevel="L"`
  for a level 1 to 7 and `hidden="1"` when asked, the attributes
  placed as the row format places `ht`; level 0 removes
  `outlineLevel` (and leaves `hidden` as the call gives it). A row
  that is absent is created by the row creation path, as a height
  creates one; a range over absent rows creates each, which is the
  caller's cost. `group_columns(part, first, last, level, hidden)`:
  the same on the `<col>` elements covering the range through the
  split rule, then coalesced (decision 2, P1 ruling 2): on a part
  where the write records one of this part's column calls, the
  pieces the split produced and the elements the write created
  are merged with an adjacent piece or created element whose
  attributes other than `min` and `max` are byte-identical, so a
  group over a range the file covered with one element is still
  one element, and a range no element covered gets one element
  with `min`, `max` and the edited attributes; an element the
  write did not touch is never merged into or changed, and the
  core's width and hidden calls alone write today's bytes.
- `set_row_collapsed(part, row, collapsed: bool)` and
  `set_column_collapsed(part, column, collapsed)`: `collapsed="1"`
  set on, or removed from, the `<row>` or the covering `<col>`,
  as the hidden flag is. Which row is the summary row is the
  caller's (decision 4): openpyxl's shape, nothing automatic.
- `set_outline_summary(part, below: Option<bool>, right:
  Option<bool>)`: each `Some` written as `summaryBelow` or
  `summaryRight` on `<sheetPr>`'s `<outlinePr>`, merged into one
  the part has or created in 18.3.1.82's order, `<sheetPr>`
  created in schema position when absent, every other attribute
  and child kept; the pass of `specs/0025`.
- The outline maximums: a write that sets a level above 0 on a
  part sets `<sheetFormatPr>`'s `outlineLevelRow` (rows) or
  `outlineLevelCol` (columns) to the greater of the attribute's
  value and the highest level the write sets, every other
  attribute kept, the element created in schema position (with
  the schema's required `defaultRowHeight="15"`) when absent;
  never lowered (decision 3), since the attribute only sizes the
  outline bar and a lower true maximum costs nothing but an empty
  bar level.
- `set_row_style(part, row, style: Option<u32>)`: `Some(i)` writes
  `s="i" customFormat="1"` on the `<row>`, `None` removes both;
  `set_column_style(part, column, style: Option<u32>)`: `Some(i)`
  writes `style="i"` on the covering `<col>` through the split
  rule and the coalescing, `None` removes it. The index is checked
  at the write against the styles part's `cellXfs` count, with
  the cell style's message (decision 5); an index `add_style`
  returns in the same write is valid, as it is for a cell. A cell's
  own `s` is never changed by a row or column style, as the
  catalogue already says; what a consumer shows for a cell with
  no `s` in a styled row is the consumer's.

### Errors

One new `Error::Package` message, in the catalogue as the earlier
parts' are, marked by feature:

`rowcol`:
- "an outline level is out of range" (at the call: a level above 7)

Shared (constants under the feature, one definition): part 3's
index message (an index or a range off the grid, `first > last`);
part 1's "no such part", "an edit to a removed part" and "a cell
edit to a part this write replaces"; part 3's one-shift message,
"a worksheet whose cols are out of order or overlap cannot be
edited" and the two-elements message; `specs/0012`'s "a style
index is out of range"; `specs/0025`'s settings message for two
`<sheetPr>`; `Error::Xml` for bytes that are not UTF-8. Nothing is
checked twice.

### 32-bit and streaming

Every index is `u32`, a level `u8`; the worksheet pass streams as
today, the rows edited in place as they are met and the created
ones in order; `<cols>` is held whole as the column formats hold
it (bounded by the element, never by the sheet's rows; the
coalescing walks it once); `<sheetPr>` and `<sheetFormatPr>` are
held to their end tags by the settings pass. No new whole-part
hold.

### Size

One probe under decision 1 A: `probe-emend-rowcol-write-xml` gains,
on its fourth write before the shift, `group_rows` of two rows at
level 1 hidden, `set_row_collapsed` of the row after, `group_columns`
of two columns at level 1, `set_outline_summary` below false,
`set_row_style` and `set_column_style` of a style the probe adds,
and after the write the row and column counts carrying
`outlineLevel`, `collapsed` and a style read back through the
probe's own `quick-xml` walk and printed; its fifth write, the
shift, then moves them. The size log measures as part 3's did,
classifies the twenty-five probes of part 3's record against it,
none new, prints `rowcol`'s cost (the rowcol probe minus the
format probe rebuilt at the same commit) beside part 3's 35248 and
the difference as the part's figure, the core's growth (the format
probe at the commit minus the same probe rebuilt at 01b14c0, both
printed; expected near 0), the budget figure against 458752, and a
cause per probe; the cause control and the manifest cause listing
stay.

### Harness runs

`tools/emend-copy` gains two operations, given after the two paths
as the others are; neither takes a reference, name or number from
its command line; every constant below is the tool's own. W is
the first worksheet part in `<sheets>` order and M its largest row
number as the tool's cell read gives it; a row past the grid
refuses the sample whole with part 3's index message. No reference
and no text from the corpus is printed. Each sample reopens its
output and reads the attributes it wrote with the tool's own
`quick-xml` walk.

- `--group-sample <in> <out>` (P1): `group_rows(W, M + 1, M + 2,
  1, true)`, `set_row_collapsed(W, M + 3, true)`, `group_columns(W,
  2, 3, 1, false)`, `set_outline_summary(W, Some(false), None)`;
  prints `group` with `rows_before`, `rows_present` (the input's
  `<row>` elements numbered M + 1 to M + 3, rows with no cell past
  the cell read's M, which the write edits in place; P1 ruling 6),
  `rows_after` (`<row>` of W; `rows_before + 3 - rows_present`,
  asserted), `level_rows` (rows carrying
  `outlineLevel="1"` after; 2, asserted), `hidden_rows` (of those
  two, 2, asserted), `collapsed_rows` (1, asserted), `cols_before`,
  `cols_after` (`<col>` of W), `level_cols` (`<col>` elements
  carrying `outlineLevel="1"` after, covering exactly columns 2 to
  3 between them; 1 or 2, asserted against the input's coverage:
  1 when the input covered 2 and 3 with equal attributes or with
  none, else 2), `max_row_ok`, `max_col_ok` (1 when
  `sheetFormatPr` carries `outlineLevelRow` and `outlineLevelCol`
  at least 1, asserted), `summary_ok` (1 when `<outlinePr>` under
  `<sheetPr>` carries `summaryBelow="0"`, asserted),
  `kept_cols` (the `<col>` attributes other than the three the
  sample sets, over the columns the sample touches, equal before
  and after by name and bytes, asserted), `r_bound_before`.
- `--linestyle-sample <in> <out>` (P2): `add_style(0, bold font)`
  giving `i`, `set_row_style(W, M + 1, Some(i))`,
  `set_column_style(W, 9, Some(i))`; prints `linestyle` with
  `rows_before`, `rows_present` (the input's `<row>` numbered
  M + 1, 0 or 1; P2 ruling 1), `rows_after` (`rows_before + 1 -
  rows_present`, asserted), `row_style_ok` (1
  when row M + 1 carries `s="i"` and `customFormat="1"`, asserted),
  `cols_before`, `cols_after`, `col_style_ok` (1 when the `<col>`
  covering column 9 alone carries `style="i"`, asserted),
  `col_split` (the number of `<col>` elements the input's covering
  element became, 1 to 3), `kept_cols` (as the group sample's),
  `xfs_before`, `xfs_after` (`+ 1`, asserted), `r_bound_before`.

The runner and the record script are unchanged; the run names its
operation through `--tool-arg` and one `--label`. Expected results,
`lo` and `lo_in` `ok` in every case; `descriptor` alone allowed on
a changed entry; `order_preserved` `yes`; `files_identical` `no`;
for the group sample W `changed` and every other entry
`identical`; for the linestyle sample W and `xl/styles.xml`
`changed` and every other entry `identical`; the calc chain is
never dropped.

The run log's own check over the extracted input and output
parts, with `ElementTree` and byte spans and a control that
reports a planted difference per check, printing no name, no
reference and no text from the corpus:

- W, group: every `<row>` of the input byte-identical in the
  output except a present row numbered M + 1 to M + 3, which keeps
  every byte but the attributes the sample sets, appended or
  replaced in place; an absent one of the three created with
  exactly `r` and those attributes (P1 ruling 6); `<cols>`: every
  `<col>` not
  covering columns 2 or 3 byte-identical, those covering them
  split and coalesced as "The calls" says with the other
  attributes kept; `<sheetFormatPr>`'s other attributes
  byte-identical and the two maximums as expected; `<sheetPr>`
  created or kept with `<outlinePr>` in 18.3.1.82's position and
  every other child kept; every other root child byte-identical.
- W, linestyle: a present row M + 1 keeping every byte but `s`
  and `customFormat`, set or replaced in place; a created one
  carrying `r`, `s` and `customFormat` alone (P2 ruling 1); the
  `<col>` split as the width sample of
  part 3 is checked; `xl/styles.xml`: `cellXfs` one longer, the
  input's entries a byte-identical prefix, `fonts` one longer.

Every phase's run log also runs the edit open check on the host
over that phase's keep, names the phase directories it compares
against by their record commit, prints the devices of `TMPDIR`
(the runs' and the open check's) and of the corpus directory, and
counts the calc-chain workbooks with the rule covering them. A
departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. LibreOffice's
rendering of a group is not checked, only that the file opens;
the Excel check (decision 7) shows the groups as Excel shows them,
and this part's two samples join its list.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m3p4-<phase>/` (`gate.log`,
`test.log` on both targets with the feature off and on,
`negative.log`, `run.log`, `scan.log`, `size.log` in P3); every
rule the earlier parts added (`specs/0025` "Phases" lists them
with `specs/0024`'s rulings on `rm -rf` and the scripts' checks,
and `specs/0025`'s verification rulings on the run log's lines);
the scan log last; at most two independent checks; report; STOP.
No push during the part; the close-out push follows P3 and carries
the record commit 0caa390 of part 3's close-out with this part's.

Part 4's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0025-sheet-and-print-settings.md` as planning left them
after part 3's close-out (the close-out rulings 1 to 4, the Close
row and the part 3 heading), and this spec. Part 4 carries no code
item from part 3.

### P1: outline levels, collapsed, the maximums and the summary

The doc commit; then the gating of the settings pass under either
feature, `group_rows`, `group_columns` with the coalescing,
`set_row_collapsed`, `set_column_collapsed`, `set_outline_summary`,
the maximums on `<sheetFormatPr>`, the one message, and
`--group-sample`. Tests on byte-built parts: a row group over
present rows (the attributes added in place, every other byte
kept) and over absent rows (created in order, between present
ones); level 0 removing the attribute; hidden set and kept; a
collapsed row and column; a column group over a range one `<col>`
covers with other attributes (split then coalesced back to one
with the group attributes), over a range two elements cover with
different widths (two elements, each with the level), over
columns no element covers (one element created), over a range
half covered (two elements); the maximums set on a present
`<sheetFormatPr>` (the other attributes kept, a higher file value
kept), on a created one; `<outlinePr>` created under a present
and a created `<sheetPr>` (between `tabColor` and `pageSetUpPr`
when both exist), merged into a present one; the refusals (level
8, `first > last`, a row past the grid, a part with two
`<sheetPr>`, a group with a shift in the same write, on a removed
part); a group kept through `insert_rows` above it in a second
write (the attributes travel); the tool's line. The gate: the
crate built and tested with the feature off and on, on both
targets, the `pub fn` lines listed off and on. The negative log
through the binary: the sample on a fixture whose first sheet's
last row is 1048574 (M + 3 past the grid, refused with the index
message, with a control). The run log: `--group-sample` on both
targets over the corpus, the checks as "Harness runs", the keep
and the edit open check, recorded as `records/m3p4-p1-<utc-date>-
<shortsha>.md` and `.txt`.

### P2: row and column styles

The doc commit (the P1 verification rulings); the carry, if any;
then `set_row_style`, `set_column_style` with the index check at
the write, and `--linestyle-sample`. Tests on byte-built parts: a
row style on a present row (the two attributes added, every other
byte kept) and on an absent one (created); `None` removing both; a
column style through each split case and coalesced; `None`
removing `style`; an index past `cellXfs` refused at the write
with the cell style's message, and one `add_style` returns in the
same write accepted; a styled row kept through a shift; the
tool's line. The negative log through the binary: the sample on a
fixture whose first sheet's last row is 1048576 (refused, with a
control). The run log: `--linestyle-sample` on both targets,
recorded as `records/m3p4-p2-<utc-date>-<shortsha>.md` and `.txt`.

### P3: the catalogue and limits, the probe, the size record

The doc commit (the P2 verification rulings); the carry, if any;
then the `package` header's catalogue and limits for this part
(the message by feature; the coalescing rule; the maximums never
lowered; what is never written: an automatic `collapsed`, a
summary row chosen by the write, `outlineLevel` above 7, a style
on a cell; what the read of part 5 will give); the rowcol probe
grown; the size log as "Size" says, recorded as
`records/size-m3p4-<utc-date>-<shortsha>.md`; the negative log
through the binary for every message of the part reachable there,
the unreachable ones stated with the test that covers each; both
samples again on both targets with the size log's release
binaries, byte-identical to P1's and P2's runs by sha256, recorded
as `records/m3p4-p3-<sample>-<utc-date>-<shortsha>.md` and
`.txt`. The open record of the part's keep.

### Close-out push (after P3)

As part 3's: the self-scan rule on the push and ci-run scripts,
`ci-run.log`, then the post-stage `scan.log` over the pushed range
with `push.log` and `ci-run.log` as its siblings, then one record
commit holding the three logs, the `ANTHROPIC_API_KEY` check; the
push script's previous-record pattern naming part 3's record
commit 0caa390 (`^M3 part 3 close-out record`), its base part 3's
close-out doc commit a8dd1d3 resolved by its subject ("M3 part 3
close-out: commit planning docs (the P4 verification and rulings,
decision 7 deferred again)"), its tip this part's close-out doc
commit; pre mode on a clean tree; one independent check before
the report. The push carries 0caa390 and this part's commits, on
planning's word and the maintainer's authorisation typed in the
implementing session. Before the push, the Excel check for
Milestone 2 (every corpus row of every Milestone 2 sample, with
Milestone 3's samples on its list), deferred by the maintainer
eight times, last on 2026-10-04, is run (Milestone 2's acceptance
record then follows Milestone 1's form) or deferred again by a
decision recorded here (decision 7).

## Planning rulings during P1 (2026-10-04)

Asked by the implementing session after the doc commit 56ebb3f,
before any code.

1. A part with two `<sheetFormatPr>`, or one out of schema order,
   is refused with the settings message as it stands (option a),
   since `<sheetFormatPr>` is the pass's fourth held element and
   the part allows one new message; the catalogue says the
   message covers it. No existing message changes.
2. The coalescing's scope: option (a) within option (b), as "The
   calls" now reads. Only the pieces the split produced and the
   elements the write created are merged, each with an adjacent
   piece or created element whose tag is byte-identical once
   `min` and `max` are set aside; an element the write did not
   touch is never merged into or changed; and the merge runs only
   on a part where the write records one of this part's column
   calls (`group_columns`, `set_column_collapsed`,
   `set_column_style`), so `set_column_width` and
   `set_column_hidden` alone write exactly today's bytes with the
   feature on or off, and part 3's and the earlier parts' outputs
   stay what they were.
3. `hidden: false` in a group call removes `hidden="1"` from a row
   or column that carries it, as `set_row_hidden(false)` does, and
   the sample's `group_columns(W, 2, 3, 1, false)` does so on a
   corpus column that is hidden (the run log reports the count).
   The group sample's `kept_cols` compares every attribute other
   than `min`, `max`, `outlineLevel`, `hidden` and `collapsed`;
   the spans are checked through `level_cols` and the run log's
   own check (P1 verification ruling 4).
4. An `outlineLevelRow` or `outlineLevelCol` whose value is not a
   whole number from 0 to 255 counts as 0 and is replaced by the
   write's level, since the maximum only sizes the outline bar
   and the output is then well-formed; the catalogue says so. The
   corpus's 45 carry 0.
5. The unasked proposals accepted: a created `<sheetFormatPr>` is
   `defaultRowHeight="15"` then the two maximums, a present one
   through the merge (set first, kept after); on a `<row>` the new
   attributes are appended after `ht` and `hidden` as
   `outlineLevel` then `collapsed`, or replaced in place; a column
   range records one field per column, the range's length the
   cost, and absent rows are created.
6. The group sample's row count (asked after the crate commit
   7b3d9b2 and the tool commit 7693267, before any log, from a
   dry run): two corpus workbooks carry `<row>` elements with no
   cell past the cell read's M, inside M + 1 to M + 3, which the
   write edits in place as "The calls" says; the sample's "+ 3,
   the three created" assumed all three absent, planning's error.
   Ruling (option a): the line gains `rows_present` after
   `rows_before`, the input's `<row>` elements numbered M + 1 to
   M + 3, and asserts `rows_after = rows_before + 3 -
   rows_present`; the other asserts stand; the run log's check
   reads as "Harness runs" now says, so the corpus exercises the
   edit in place and the creation. The tool change goes in a new
   phase commit on top of 7693267 before any log; M stays the
   cell read's, as every sample's.

## Planning rulings during P2 (2026-10-04)

Asked by the implementing session after the doc commit c7c9e39
and the carry commit d1278c3, before the tool.

1. The linestyle sample's row count meets P1 ruling 6's case
   (the same two workbooks carry an empty `<row>` numbered M + 1):
   the same ruling. The line gains `rows_present` after
   `rows_before`, the input's `<row>` numbered M + 1 (0 or 1), and
   asserts `rows_after = rows_before + 1 - rows_present`;
   `row_style_ok` stands; the run log checks a present row as
   edited in place (every byte kept but `s` and `customFormat`)
   and a created one as `r`, `s` and `customFormat` alone; M stays
   the cell read's. "Harness runs" reads as corrected; the "+ 1"
   was planning's, the gotcha of ruling 6 not yet applied to this
   sample.
2. On a `<row>`, `s` and `customFormat` are replaced in place or
   appended before the size, hidden, outline and collapsed
   attributes the same write sets, so a created row reads
   `<row r="N" s="i" customFormat="1"/>`, near the schema's order.
   Confirmed.

## Planning verification and rulings on P1 (2026-10-04)

Verified by planning from primaries: the doc commit 56ebb3f holds
this file at 36fa82a5, `specs/0025-sheet-and-print-settings.md`
at ef2fe222 and `ROADMAP.md` at 86d03fb9, the hashes planning
sent; the crate commit 7b3d9b2, the tool commit 7693267 and the
phase commit 93a92d5 (ruling 6's `rows_present`) read in full by
planning; the record commit d26c470 adds the six logs and four
records; 5 of 5 messages ASCII, ending with the Co-Authored-By
line only; the logs at the reported hashes, each ending all as
expected, the scan RESULT: PASS, every log pinning 36fa82a5 and
printing c62113dc as planning's working copy; the check answer
ASCII with its raw hash in its header; the compound-shape grep 0;
every `rm -rf` in the logs the guarded form on a directory the
script made or `counting.sh`'s own; planning's own build at
93a92d5 in a detached worktree: 513 crate tests with the features
off, 568 with `rowcol`, 596 with `rowcol` and `sheetprops`, 540
with `sheetprops` alone and 822 with every feature, on both
targets, 1002 workspace tests on both, clippy and doc 0 lines; the
i686 release `emend-copy` at the run log's hash e8ef8757 (the
host's differs by build path); planning's own run of
`--group-sample` with its own binaries over all 27 corpus
workbooks on both labels, 54 outputs and 54 tool lines
byte-identical to the run directory's; planning's own privacy scan
0caa390..d26c470, 0 findings; `ANTHROPIC_API_KEY` unset.

1. Finding 1: a group's `hidden: false` drops a `hidden` attribute
   of any value, as `set_column_hidden(false)` does, and the
   sample dropped 28 such attributes in 17 workbooks (LibreOffice
   writes `hidden="false"`); the value is the run log's "any
   value", the report's "false" unmeasured. Accepted; the
   attribute's meaning is unchanged. The survey counts `hidden`
   as `1` or `true` from P2 on (the check's note).
2. Finding 2, the latent mismatch: whitespace between two `<col>`
   pieces ends the merge, where the sample's `level_cols` and the
   spec's "one element" expect a merge. Ruling: a whitespace-only
   run between two pieces the write produced is dropped and the
   pieces merge across it; a comment or any other content ends
   the merge as a kept element does. A test on an indented
   `<cols>` goes in P2's first code commit. The corpus's `<cols>`
   carry no whitespace between elements (27 of 27 ok).
3. Findings 3 and 4: the core's `cols_content` now writes through
   `ColsOut` in every build with the same bytes (the keep's pairs
   that differ: 0), and the `sheetprops`-only build changed in
   its settings table; accepted, P3's size log measures the core.
   P2's gate runs the crate's tests with `sheetprops` alone as
   well (planning's own run: 540 on both targets).
4. Finding 5: the sample's `in_window` assertion is listed in its
   doc comment in P2's first code commit; `kept_cols` as coded
   (every attribute but `min`, `max`, `outlineLevel`, `hidden`
   and `collapsed`, the spans checked through `level_cols` and the
   run log) is what P1 ruling 3 meant, and the ruling reads so.
   Finding 6: a test that `set_column_hidden` alone writes
   today's bytes goes in P2's first code commit with ruling 2's.
5. The scan's first final attempt failed on one en dash in the
   check's answer, recorded as a hyphen with the raw answer's hash
   kept in the header; the script history (a usage comment, a
   control that could not hold, two dry runs, an `rmdir`) is
   working state. Accepted.
6. P1 is DONE. P2 is released: its doc commit carries this file
   with "Planning rulings during P1" and this section, and
   `ROADMAP.md`'s P1 row; its first code commit is rulings 2 and 4's
   items (the whitespace merge with its test, the doc comment, the
   hidden-alone test); its gate adds the `sheetprops`-alone test
   run; then "P2: row and column styles".

## Planning verification and rulings on P2 (2026-10-04)

Verified by planning from primaries: the doc commit c7c9e39 holds
this file at 5f0a8192 and `ROADMAP.md` at 540b78b1, the hashes
planning sent; the carry commit d1278c3 (the whitespace merge
with its test, the hidden-alone test, `//!` lines alone in the
tool), the crate commit 9d266f0 and the tool commit 8416a1e read
in full by planning; the record commit 4a23c4b adds the six logs
and four records; 5 of 5 messages ASCII, ending with the
Co-Authored-By line only; the logs at the reported hashes, each
ending all as expected, the scan RESULT: PASS, every log pinning
5f0a8192 and printing b69da8a3 as planning's working copy; the
check answer ASCII; the compound-shape grep 0; every `rm -rf` in
the logs the guarded form on a directory the script made or
`counting.sh`'s own; planning's own build at 8416a1e in a
detached worktree: 513 crate tests with the features off, 583
with `rowcol`, 611 with `rowcol` and `sheetprops`, 540 with
`sheetprops` alone and 837 with every feature, on both targets,
1019 workspace tests on both, clippy and doc 0 lines; the i686
release `emend-copy` at the run log's hash be8edbc2 (the host's
differs by build path); planning's own run of
`--linestyle-sample` with its own binaries over all 27 corpus
workbooks on both labels, 54 outputs and 54 tool lines
byte-identical to the run directory's; planning's own privacy
scan d26c470..4a23c4b, 0 findings; `ANTHROPIC_API_KEY` unset.

1. Observation 1 (more ungated core code with the same
   behaviour: `ColsOut::between`, the style check's loop over
   `line_styles`): accepted, P3's size log measures the core.
   Observation 3 (every recorded `Style(Some(i))` index-checked,
   an index a later call replaced included): consistent with how
   cell style edits are checked; accepted and stated in the
   catalogue. Observations 6 and 7 (form feed, which XML 1.0
   forbids anyway; the commit message's sentence true when
   written): accepted.
2. Observation 2: the linestyle sample's doc comment names its
   two unlisted assertions (the present row after the write; the
   split in 1 to 3) in P3's first code commit, `//!` lines alone.
   Observation 6's untested case (a `<col>` already covering
   exactly column 9 given `Some(i)`): a crate test in the same
   commit.
3. Observation 4: P3's rerun of both samples adds a planted-flag
   control on a workbook whose row M + 1 is present (two of the
   corpus's are), so the in-place branch is shown by a plant as
   well as by the two corpus rows. Observation 5: P1 verification
   ruling 1 (the survey counting `hidden` as `1` or `true`)
   applies to P3's rerun of the group sample; the linestyle sample
   touches no hidden flag.
4. The disclosure (a stray `cat >` line, the killed shell, an
   empty file of its own removed; the gotcha recorded): working
   state, accepted; no hand `rm -rf`.
5. P2 is DONE. P3 is released: its doc commit carries this file
   with "Planning rulings during P2" and this section, and
   `ROADMAP.md`'s P2 row; its first code commit is ruling 2's
   items; then "P3: the catalogue and limits, the probe, the size
   record", the rerun with ruling 3's plant and the survey's
   `hidden` rule.

## Decisions for the maintainer

Decided by the maintainer 2026-10-04: 1 to 6 as recommended (A in
each), in the reply "A on all (Recommended)" to planning's
question, typed in the planning session; 7 recorded.

1. Packaging: A (recommended), the feature `rowcol` grows and its
   probe grows, the part's figure the probe's growth; B, a new
   feature `outline` with its own probe.
2. Column ranges: A (recommended), after a range edit adjacent
   `<col>` elements with identical attributes are coalesced into
   one, Excel's and openpyxl's form, so a group over twenty
   columns is one element; B, the per-column split alone, one
   element per column.
3. The outline maximums: A (recommended), `<sheetFormatPr>`'s
   `outlineLevelRow` and `outlineLevelCol` raised to the highest
   level the write sets and never lowered, since the file's value
   is already its maximum and no scan of the rows is needed; B,
   recomputed from every row and column, a second pass over the
   part.
4. Collapsing: A (recommended), openpyxl's shape, `hidden` as a
   flag of the group call and `collapsed` a separate call on the
   row or column the caller names, nothing automatic; B, the write
   sets `collapsed` on the row after a hidden group itself, which
   needs `summaryBelow` read and Excel's rule for nested groups.
5. The style index: A (recommended), checked at the write against
   `cellXfs` as a cell's is, with the same message; B, unchecked,
   Excel repairing a bad one.
6. Reads: A (recommended), writes only; part 5 reads dimensions
   back; B, read calls here.
7. The Excel check for Milestone 2: deferred eight times, last on
   2026-10-04 (`specs/0025` decision 7), to run before this part's
   close-out push at the latest or be deferred again by a decision
   recorded here. Recorded, not asked now; asked at the close-out.

## Out of scope

- Reading levels, flags or styles back (part 5); row and column
  breaks; `sheetFormatPr`'s other defaults; `thickTop`,
  `thickBot`; `bestFit`.
- Choosing the summary row, nested-group folding and the outline
  symbols' display: Excel's and the caller's (decision 4).
- A style applied to the cells of a row or column (the cells keep
  their own `s`; `set_cell_style` does cells).
- Moving a group with a table or a drawing on a shift: part 3's
  limits stand.
