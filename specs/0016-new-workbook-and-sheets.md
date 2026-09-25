# 0016: Milestone 2 part 1, a new workbook and the sheet list

Status: APPROVED by the maintainer 2026-09-24 (planning draft the
same day). Decisions 1 to 4 under "Decisions for the maintainer"
were taken as planning recommended in the maintainer's reply of
2026-09-24 to the plan ("i agree with recs for now, no supplying
yet"); decision 5 was taken as recommended with the maintainer's
one-word approval of the spec the same day. Sent to the
implementing session the same day.
Context: `CLAUDE.md` ("What emend is": like openpyxl; "Architecture":
parse only what an edit touches, pass everything else through;
"The fidelity harness is the acceptance instrument"), `ROADMAP.md`
("Milestone 2", the outline approved 2026-09-23; "Milestone 1" part
8, the items carried from `specs/0015`; "Standing items", the size
ceiling), `specs/0009-package-layer.md` (the recorded edits and the
rels and content-types rewrites), `specs/0011-editing-cells.md`
(the worksheet rewrite), `specs/0013-formulas.md` (the workbook
rewrite, `calcPr`, the formula-change rule, the size probes),
`specs/0015-open-checks.md` (the keep, the edit open check, the
Excel check and its rulings), `specs/0001-bootstrap.md` ("Rules for
every phase").

## Goal

Two things openpyxl does that emend cannot: make a workbook that
did not exist, and change the list of sheets. Both stay inside the
read-modify-write architecture. A new workbook is a fixed, fictional
template of six parts carried in the crate as XML text, written
through the existing archive write and opened back: no writer is
added. Sheet edits are recorded like every other edit and applied
by `write_to` as one rewrite of the workbook part's `<sheets>`,
`<definedNames>` and `<bookViews>` regions, with the relationship,
content-type and entry edits the package layer already applies.
Removal takes with it the parts only that sheet reaches.

The part's first phase is scripts only: it lands every item part 8
carried and produces the list for Excel check 2, so that check runs
on Milestone 1's code while the later phases are built.

## Facts planning checked (2026-09-24)

- The archive write is deterministic: an added entry is written with
  flags 0, deflate, and DOS time and date zero (`archive.rs`, the
  local and central header writers), and the zip crate path used for
  a temporary sets `zip::DateTime::default()`. Two writes of the same
  edits on the same input give the same bytes; the crate's snapshot
  tests rest on it.
- `Archive` opens an archive with no entries (`central_start` when
  there are none is handled, and the write with no entries is
  covered), so a 22-byte end record is a valid input.
- `Package::add_part`, `add_override` and `add_relationship` record
  additions; `write_to` creates a rels part for a source that has
  none; `add_relationship` numbers ids past the part's own and the
  pending ones. A text value is written as an inline string
  (`Value::Text`, `t="inlineStr"`), so a new workbook needs no shared
  strings part.
- `Package::workbook` reads `<sheets>` in document order with each
  sheet's `name`, `sheetId`, `r:id`, `state`, resolved part and kind;
  `rewrite_workbook` (part 6) rewrites the workbook part in one pass
  through `xml::Stream`, copying raw outside the `calcPr` tag, with
  the root's children ordered by `BEFORE_CALC_PR`, which names
  `bookViews`, `sheets` and `definedNames`.
- `write_to` refuses a workbook edit on a package that replaces or
  removes the workbook part (`REMOVED_PART_EDITED`, part 6).
- The privacy gate refuses a blob with an archive signature outside
  the fixtures directory, and the hygiene step refuses office file
  extensions in any case (`specs/0001` P5, P6c). A template cannot
  be a zip in the repository; XML text files can.
- The list script takes one `--run` directory and derives the three
  formula-sample references for every corpus row from the input;
  the fixture rows come from the keep with `lo_in ok`; the result
  column is empty (`scripts/excel-check-list.py`, docstring).
- Excel check 1 (`specs/0015`, "Planning rulings on the Excel
  check"): Excel refuses a cell with `s` of 1 or more when the styles
  part lacks the font, fill, border and cell-style collections;
  Excel opened 138 fixture outputs and 138 inputs through COM in
  well under a minute; the 27 corpus rows were opened by hand.
- The size budget figure after part 6 is 312812 section bytes of a
  393216 ceiling (`records/size-m1p6-2026-09-23-e0a7b95.md`):
  headroom 80404. The outline's estimate for this part is 25 KB.
- The corpus has 27 workbooks; the maintainer's P5 and P6 items (a
  Google Sheets export, a macro file, threaded comments, chartex)
  are not supplied and will not be before this part (decision 4).

## Design

### Crate

No new crate, feature or dependency. Six template files under
`crates/emend/template/`, included with `include_bytes!`, and one
22-byte constant. The rest is in `package`: a constructor, four
sheet edits, one read of the projected sheet list, the `<sheets>`,
`<definedNames>` and `<bookViews>` rewrite, and the reachability
walk for removal.

### The template

Six files, XML text with a declaration (`version="1.0"`,
`encoding="UTF-8"`, `standalone="yes"`), each pinned by its sha256
in P2's run log and record. Their entry names in the written
package, in this order:

1. `[Content_Types].xml`: `Default` for `rels` and for `xml`;
   `Override` for the workbook, the one worksheet and the styles
   part, with the transitional content types.
2. `_rels/.rels`: one relationship, transitional `officeDocument`,
   target `xl/workbook.xml`.
3. `xl/workbook.xml`: the main namespace as default, the
   relationships namespace with prefix `r`; `<workbookPr/>`;
   `<bookViews>` with one `<workbookView activeTab="0"/>`;
   `<sheets>` with one `<sheet name="Sheet1" sheetId="1"
   r:id="rId1"/>`. No `calcPr`: the formula-change rule inserts one
   as it does today.
4. `xl/_rels/workbook.xml.rels`: `rId1` worksheet to
   `worksheets/sheet1.xml`, `rId2` styles to `styles.xml`,
   transitional types.
5. `xl/worksheets/sheet1.xml`: the main namespace as default and an
   empty `<sheetData/>`, nothing else. The same bytes are the blank
   part of every added sheet ("Adding a sheet"), so no added sheet
   carries a `tabSelected` view.
6. `xl/styles.xml`: `fonts` with one font (a size and a name),
   `fills` with the two fills Excel requires (`none` and `gray125`),
   `borders` with one empty border, `cellStyleXfs` with one `xf`,
   `cellXfs` with one `xf` naming font 0, fill 0, border 0 and `xfId`
   0, `cellStyles` with the one built-in `Normal`. No `numFmts`,
   `dxfs` or `tableStyles`. This is the shape Excel check 1 found
   Excel requires and the precondition `specs/0012`'s limits now
   state.

No `docProps` parts: Excel and LibreOffice open a package without
them, and emend never writes properties (part 4 of the outline).
The template is fictional and holds no cell; it is documented in
the crate as generated by hand for this spec.

### `Package::new`

`Package::new() -> Result<Package<Cursor<Vec<u8>>>, Error>`: opens
the empty archive constant (the 22-byte end record with zero
entries and an empty comment), records the six template parts as
additions in the order above, writes through `write_to` into a
`Vec<u8>`, and opens the result. The output is byte-identical
across calls and across targets (a test compares two calls; the
run log compares host and i686). Every later edit is the ordinary
path: `set_cell`, styles, formulas, sheets, then `write_to`. A
caller who wants the bare workbook on disk writes it with no
edits, which is a verbatim round trip. Errors: whatever the write
and the open refuse, which no test can reach on the fixed bytes;
the catalogue says so.

### The projected sheet list

`Package::sheets(&mut self) -> Result<Vec<Sheet>, Error>`: the
workbook's sheets with the recorded sheet edits applied in the
order they were recorded. `Package::workbook` keeps returning the
file's state, as `Package::cells` reads the file and not the pending
cell edits. A sheet added and not yet written has `part` set to the
entry name it will get, `sheet_id` and `rel_id` to the values it
will get, `state` `None` and `kind` `Worksheet`.

Every sheet edit validates against this projected list at record
time and returns `Result`, so a caller learns of a bad name or a
missing sheet at the call and not at the write. `write_to` applies
the recorded edits again in order and refuses any that no longer
holds (a defence, unreachable through the public calls, named in
the catalogue as such).

### Sheet names

The rules Excel enforces on entry, checked on every name a caller
gives (add and rename): not empty; at most 31 characters (`char`s,
not bytes); none of the seven characters colon, backslash, slash,
question mark, asterisk, left bracket, right bracket; not starting
or ending with an apostrophe; not equal to `History` ignoring case;
not equal, ignoring case, to any other sheet's name in the projected
list. Case is compared through `char::to_lowercase` on both sides;
Excel's rule is case-insensitive and this is the no-dependency
approximation, documented in the limits. Nothing else about a name
is checked. Names are written escaped as `Value::Text` is.

### Adding a sheet

`Package::add_sheet(&mut self, name: &str) -> Result<Vec<u8>,
Error>` returns the new worksheet's entry name. It records, in the
order the write applies them: the entry, the template's blank
worksheet bytes, at the name `<dir>/worksheets/sheet<n>.xml` where
`<dir>` is the workbook part's directory and `<n>` the smallest
positive integer whose name is neither an entry nor a pending
addition; an `Override` for it with the transitional worksheet
content type, unless the content types part has a `Default` for
`xml` whose value is that type (then none); a relationship on the
workbook part of the worksheet kind, in the form (transitional or
strict) of the workbook's first existing worksheet relationship or
transitional when there is none, target relative to the workbook
part; and a `<sheet>` at the end of the projected list with the
given name, `sheetId` one more than the largest in the projected
list, and the relationship's id. The new sheet goes last; a caller
places it with `move_sheet`.

### Renaming and moving

`Package::rename_sheet(&mut self, name: &str, to: &str) ->
Result<(), Error>`: the sheet named `name` in the projected list
gets the name `to` (checked as above; renaming a sheet to its own
name, or to its own name in another case, is allowed and changes
the attribute). Formulas, defined names, pivot sources, chart
series and anything else that names the sheet in text are not
touched: openpyxl's limit, stated in the limits.

`Package::move_sheet(&mut self, name: &str, to: usize) ->
Result<(), Error>`: the sheet is taken out of the projected list
and inserted at position `to` (0 first; `to` equal to the list's
length minus one is last; larger is refused). Moving a sheet to its
own position records nothing.

### Removing a sheet

`Package::remove_sheet(&mut self, name: &str) -> Result<(), Error>`.
Refused when the projected list has one sheet, or when every other
sheet of the projected list has a `state` (hidden or veryHidden):
Excel needs one visible sheet. Otherwise it records, in this order:

1. the `<sheet>` gone from the projected list;
2. the workbook relationship the sheet's `r:id` names removed;
3. the walk: R is the set of entries reachable from the sheet's
   part through internal relationships, transitively, the sheet's
   part included; K is the set of entries reachable the same way
   from the package relationships (`_rels/.rels`) with the removed
   workbook relationship left out and no edge out of the sheet's
   part. Every entry of R that is not in K is removed, with its own
   rels part when there is one and its `Override` when the content
   types part holds one. A relationship whose target is no entry
   contributes nothing. External relationships are never followed.
   Entries are compared by exact name. The walk reads every rels
   part it reaches through `Package::relationships`, which parses
   each once.
4. the calculation chain: when the workbook has a `calcChain`
   relationship, `drop_calc_chain`'s three removals, and
   `fullCalcOnLoad` set as the formula-change rule sets it; the
   chain names sheets by index and cells the sheet held. A workbook
   with no chain is left as it is on this point.

At the write, `<definedName>` elements whose `localSheetId` is the
removed sheet's index in the file's list are removed, and the
others' `localSheetId` values are remapped ("The workbook rewrite").
A defined name whose text names the removed sheet is not touched.
A pending sheet (added, not written) can be removed: its recorded
additions are dropped and nothing is written for it.

What the walk does not do, stated in the limits: it does not read
worksheet text (a formula in a kept sheet naming the removed sheet
becomes `#REF!` when Excel recalculates, as with openpyxl); it does
not touch pivot caches, external links, `definedNames` text, the
`docProps` title list or `tabSelected` in kept sheets; and a part
reachable from the removed sheet and from a kept part is kept
whole.

### The workbook rewrite

`rewrite_workbook` grows from one region to four, still one pass,
raw copy outside them, and runs when any sheet edit or the `calcPr`
edit is recorded:

- `<sheets>`: the start tag copied as is; the children replaced by
  the projected list in its order: an existing sheet's `<sheet>`
  element copied byte for byte, with its `name` attribute's value
  replaced between its own quotes when renamed; a new sheet as
  `<sheet name=".." sheetId=".." r:id=".."/>` where the prefix is
  the one bound to the relationships namespace on the workbook root
  (refused when the root binds none). Whitespace between children
  is not reproduced: the children are written back to back, as
  Excel writes them. A workbook whose `<sheets>` holds anything but
  `<sheet>` elements and whitespace is refused.
- `<definedNames>`: each `<definedName>` copied byte for byte with
  its `localSheetId` value remapped between its own quotes from the
  file's index to the projected index of the same sheet, or the
  element removed when that sheet was removed. An element without
  the attribute is copied as is. The element is left alone when
  no sheet was removed or moved.
- `<bookViews>`: every `<workbookView>`'s `activeTab` (absent means
  0) and `firstSheet` (absent means 0) remapped from the file's
  index to the projected index of the same sheet; when that sheet
  was removed, to the projected index of the first sheet with no
  `state`. An absent attribute that would map to 0 stays absent;
  one that would map elsewhere is appended after the last attribute
  as `calcPr`'s flag is. Left alone when no sheet was removed or
  moved.
- `calcPr`: as part 6.

Everything else, byte order mark and declaration included, is
copied. The rewrite refuses a prefixed workbook root (as the
`calcPr` rewrite refuses `x:calcPr`) only where it would have to
write an element: an added sheet in a prefixed workbook is refused
with a catalogued message; renames, moves and removals in one are
attribute edits on copied elements and are allowed.

### Errors

New `Error::Package` messages, each with a catalogue entry, a test
and, where a binary can reach it, a negative-log case:

- "a sheet name is empty"
- "a sheet name is longer than 31 characters"
- "a sheet name holds a character Excel refuses"
- "a sheet name starts or ends with an apostrophe"
- "a sheet name is the reserved name History"
- "a sheet name is already in use"
- "no sheet of that name"
- "a sheet position is past the end"
- "the last sheet cannot be removed"
- "the last visible sheet cannot be removed"
- "the workbook root binds no prefix for the relationships
  namespace"
- "a sheets element holds something other than sheet elements"
- "a sheet cannot be added to a prefixed workbook"
- "a sheet edit no longer applies at the write" (the defence)

The existing "the workbook part cannot be edited in a package that
replaces or removes it" covers sheet edits too. A refused write
leaves no temporary and no output.

### 32-bit and streaming

The walk holds entry names and rels parts, not part contents; the
rewrite streams the workbook part as part 6 does; the template is
under 4 KB. `u64` for every offset as before. Nothing in this part
reads a worksheet.

### Size

`tools/size-probe` gains one probe, built in the `size` profile on
i686 musl as the others:

- `probe-emend-sheets-write-xml`: `probe-emend-formulas-write-xml`'s
  work, then on the same package `add_sheet` of a name from its
  command line, `move_sheet` of it to 0, `rename_sheet` of it,
  `remove_sheet` of the sheet that was first, the write; after it
  the output's `sheets` count, the first sheet's name length and
  part length printed; and `Package::new()` written to a third path
  with its byte count printed, so the template and the constructor
  are linked. The budget figure is it over `probe-zip-xml` against
  the ceiling of 393216 section bytes; the part's own cost is it
  over `probe-emend-formulas-write-xml`; the record states the whole
  growth since part 6's 312812 with the earlier probes' drift
  classified as `specs/0011` P3 verification ruling 4 says.

Planning's expectation, not a measurement: under 25000 section
bytes for the part. Above 30000 the report says so and the
maintainer re-rules before P4's record commit; nothing moves behind
a feature in this part on planning's own word.

### Harness runs

`tools/emend-copy` gains three operations, given after the two paths
as the others are; none takes a sheet name, a cell reference or a
value from the command line, so no record names a sheet of a corpus
workbook.

- `--new-sample <a> <b>` (P2): writes `Package::new()` with no edits
  to `<a>`; opens `<a>`, records on its first sheet `(1, 1)`
  `Number(7)`, `(1, 2)` `Text("emend")`, `(1, 3)` `Boolean(true)`,
  writes `<b>`; reopens `<b>` and prints one line, `new` then
  `entries` (6), `sheets` (1), `cells` (3), `read_back` (the number
  of the three that read back with the value written, 3), the last
  two asserted by the tool (exit 2 on a difference). `<a>` and `<b>`
  are a read-modify-write pair for `xlsx-diff` and for the keep. The
  runner does not run this operation over the corpus; the run log
  runs it once on each target, diffs the pair, compares the two
  targets' `<a>` and `<b>` byte for byte, and opens both `<b>` files
  through the pinned image.
- `--sheet-sample <in> <out>` (P3): `add_sheet("emend")` (when a
  sheet of that name exists, ignoring case, `emend2`, and so on to
  the first free), `set_cell` on the new part of `(1, 1)`
  `Number(7)`, `move_sheet` of it to 0, the write; reopens the
  output and prints one line, `sheet` then `sheets_before`,
  `sheets_after` (expected `sheets_before + 1`), `first` (1 when the
  output's first sheet is the added one, read back by part), `cells`
  (the new part's cell count, expected 1), `names` (the count of
  `definedName` elements before and after, equal), `active_before`
  and `active_after` (the first `workbookView`'s `activeTab`, absent
  read as 0; expected `active_before + 1`), the expectations
  asserted by the tool.
- `--remove-sample <in> <out>` (P4): `remove_sheet` of the last
  sheet of the projected list, the write; when the workbook has one
  sheet, or its removal is refused for visibility, nothing is
  recorded and the output is byte-identical. Prints one line,
  `remove` then `sheets_before`, `sheets_after`, `removed` (0 or 1),
  `parts` (the entries the write removed, the chain counted),
  `chain_before`, `chain_after` (0 when `removed` is 1),
  `names_before`, `names_after`, `local_removed` (the `definedName`
  elements dropped for the removed index), `active_before`,
  `active_after`.

The runner and the record script are unchanged; each run names its
operation through `--tool-arg`. The run logs' checks over every
output, each with a control that reports a planted difference per
flag it raises (`specs/0012` P2), with `ElementTree` and byte spans
as `pcmp.py` did:

- P3: every entry other than the workbook part, its rels part and
  the content types `identical`; one entry `added`, the blank part
  byte-identical to the template's worksheet file; the workbook part
  identical outside the `<sheets>`, `<definedNames>` and
  `<bookViews>` regions; inside them, the sheets equal to the
  input's with the new one first, every `localSheetId` one more than
  the input's, `activeTab` and `firstSheet` one more; the rels part
  equal to the input's plus one relationship; the content types
  equal plus one `Override` (or equal, when the `Default` covers
  it, counted).
- P4: the removed entries exactly the set an independent walk in
  Python computes from the input (R minus K as defined, the rels
  parts and the chain added); every kept entry other than the
  workbook part, its rels part and the content types `identical`;
  the sheets equal to the input's minus the last; `localSheetId`
  values as defined; the chain absent from the output exactly when
  `chain_before` is 1, its override and relationship with it.

Expected results, `lo` and `lo_in` `ok` in every case; the report
states totals against the expectations above and the sheet counts
the corpus coverage record implies; a workbook where a check
differs is a finding. LibreOffice's table count in the open check
is the harness's check that the sheet list the output declares is
the one a reader sees (`lo-check.sh` counts tables against the
workbook's sheet count, prefix-tolerant since part 8 P2).

From part 8's decision 3, every phase's run log also runs the edit
open check on the host over that phase's keep, so each new test's
pair is opened in the phase that writes it.

### The carried items (P1)

Scripts and records only; the crate is not touched, so the keep and
the corpus runs P1 produces are Milestone 1's code. One code commit
holds:

- `scripts/edit-open-check.sh`: the "refused by LibreOffice" heading
  reads "(allowed: a fixture LibreOffice cannot load: zip-level, or
  a package with no relationships part or an empty content-types
  root)" (`specs/0015` P2 verification ruling 1).
- `scripts/excel-check-list.py`: `main` sets umask 077 before
  writing and IndexError joins the caught exceptions (ruling 4);
  `--run` stays the formula-sample run directory, given once, and a
  new repeatable `--run-plain <dir>` adds corpus rows from other
  run directories with the label column holding the directory's
  basename and the six reference columns empty (Excel ruling 6);
  every fixture pair with `lo_in ok` gets a second row, source
  `input`, the same test name, file `<test>.in.xlsx` (Excel ruling
  2); the docstring says all of it.
- `scripts/excel-check-open.ps1`: the fixture procedure as it ran
  (Excel ruling 1): reads the list beside it, opens every row of
  source `fixture` and `input` through COM with alerts on,
  read-only, closes without saving, writes `open-results.txt` with
  `source`, `id_or_test`, `result` (`pass` or `fail`) and Excel's
  message with tabs, carriage returns and line feeds replaced by
  spaces; and, under decision 5, every row of source `corpus` too,
  reading the value at each non-empty reference through COM into
  `seen1` to `seen3`. Paths are built from the script's own
  directory. The script is ASCII, carries no path and no name, and
  is scanned as every committed file is.
- `records/excel-check-template.md`: the automated procedure, the
  input column and the seen columns, the by-hand fallback for a
  corpus row the script refused, and the sentence that an input
  Excel refuses is allowed and an output refused on an accepted
  input is a finding (Excel rulings 1, 2, 9).

The P1 run log: the style-sample corpus run on the host with the
release binary at HEAD (the tool unchanged since part 6 P3's
record), recorded as `records/m2p1-p1-<utc-date>-<shortsha>.md` and
`.txt` with part 5 P3's expectations; the tests on both targets into
two keep directories, compared, and the edit open check over the
host keep; the list from part 6 P3's run directory (`--run`), the
style-sample run (`--run-plain`) and the keep, every hash checked
against the file it names, counts printed: corpus rows 54, fixture
and input rows two per accepted pair. The list is staged for the
maintainer as check 1 was: one directory holding the list, the
PowerShell script and the files under `files/`, zipped, outside
the repository. Excel check 2 is then the maintainer's, whenever
the laptop is at hand; its filled record is retained, assembled by
planning and committed with a scan as one record commit; it
confirms Milestone 1 when every corpus row passes with its values
and no fixture output is refused on an accepted input.

Two doc edits go with this part and are planning's, not P1's:
`specs/0012`'s limits gain the styles-part precondition sentence in
the first doc commit of this part; `specs/0001` gains the push and
ci-run scripts' self-scan rule in the close-out doc commit
(`specs/0015` close-out ruling 5), and the close-out scripts apply
it. The crate's `styles` module header gains the precondition
sentence in P2's first code commit, where the crate is next edited.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m2p1-<phase>/` (`gate.log`,
`test.log` on both targets, `negative.log`, `run.log`, `scan.log`,
`size.log` in P4); every rule the earlier parts added (`specs/0013`
"Phases" lists them; `specs/0014` P2 verification rulings 4, 5 and
7; `specs/0015` P2 verification ruling 2, every tree-status check
printing the lines; close-out ruling 6, a shape hit named by file,
line and count only); the scan log last; at most two independent
checks; report; STOP. No push during the part; the close-out push
follows P4 and carries the four local commits f6c2dba, 065f3e4,
a49f2d3 and d4ab367 with this part's.

Part 1's first doc commit carries, unedited: `ROADMAP.md`,
`specs/0012-styles.md` (the precondition), `specs/0015-open-checks.md`
if planning amends it, and this spec.

### P1: the carried items and the Excel check 2 list

Scripts and records only, as "The carried items" says, one code
commit. Tests: the list script's negative cases rerun plus a run
directory given as `--run-plain` that has no `summary.host.tsv`
(refused), a `--run-plain` directory whose output hash differs from
the summary (refused), a keep pair whose input LibreOffice refused
(no rows for it), and the PowerShell script parsed by a control
that checks it is ASCII and holds no absolute path (a grep with a
control, since no PowerShell runs here). The run log as "The carried
items". Acceptance: the style-sample record as part 5 P3's; the
list's row counts as stated; every hash checked; the edit open
check over the keep with no output refused on an accepted input.
The staged zip's sha256 printed by the log, the zip never in the
repository.

### P2: the template and `Package::new`

P2's first code commit carries the P1 verification rulings and the
`styles` header sentence. Then the six template files, the constant,
`Package::new`, the catalogue entries and `--new-sample`. Tests:
`new()` twice byte-identical; its six entries by name and order,
each byte-identical to its template file; opened back, the workbook
has one sheet named `Sheet1` of kind `Worksheet`, a styles part with
one cell format, no shared strings, no chain, `date1904` false,
`full_calc_on_load` false; `set_cell` of a number, a text, a boolean
and a formula on the new sheet, written and read back (the formula
sets the flag and inserts `calcPr` after `sheets`); `add_cell_style`
on base 0 and `set_cell_style`, written and read back; a write with
no edits byte-identical to `new()`'s bytes; the tool's line. The
negative log: nothing new is reachable through the binary; the log
says so. The run log as "Harness runs" for `--new-sample`, the
template files' sha256 printed, the two targets' outputs compared,
the pair opened through the pinned image, the edit open check over
the phase's keep. No corpus record.

### P3: `sheets`, `add_sheet`, `rename_sheet`, `move_sheet`

P3's first code commit carries the P2 verification rulings. Then
the projected list, the names check, the three edits, the
`<sheets>`, `<definedNames>` and `<bookViews>` regions of the
rewrite, the catalogue entries and `--sheet-sample`. Tests on
byte-built packages, the output's parts compared with bytes the
test builds by hand (`specs/0011` P1 ruling 5): add to a one-sheet
workbook (the entry, the override, the relationship, the `<sheet>`,
the ids); add when `sheet2.xml` exists but is not a sheet (the name
skips it); add with a `Default` for `xml` of the worksheet type (no
override); add in a workbook whose worksheet relationships are
strict (strict form); add in a workbook with `sheetId` gaps (max
plus one); two adds (ids and names distinct); add then remove of
the pending sheet (nothing written); each of the six name
refusals, and `History` in another case, and a duplicate in another
case; a 31-character name of multi-byte characters accepted; rename
kept byte for byte outside the attribute, with escaping of `&`, `<`
and a quote; rename to the same name in another case; move to 0,
to last, to its own position (nothing recorded), past the end
(refused); move with `localSheetId` on two defined names and
`activeTab` and `firstSheet` present and absent; a `<sheets>` with
a comment inside (refused); a prefixed workbook root: rename and
move allowed, add refused; whitespace between `<sheet>` elements
dropped and a byte order mark kept; a sheet edit with the workbook
part replaced (refused at write); `sheets()` after each edit; a
`Sheet` of an added sheet's fields; the tool's line. The negative
log through the binary: the refusals `--sheet-sample` can reach
(none by name, since the tool names its own sheet; the log says
so). The corpus run `--sheet-sample` on both targets, recorded as
`records/m2p1-p3-<utc-date>-<shortsha>.md` and `.txt`; acceptance
as "Harness runs".

### P4: `remove_sheet`, the catalogue and limits, the size record

P4's first code commit carries the P3 verification rulings. Then the
walk, `remove_sheet`, the chain rule, the catalogue and the limits
in `package`'s module header (the caller's picture; what the walk
does not do; the name rule's approximation; the Excel check
pending), `probe-emend-sheets-write-xml` measured as "Size" says,
recorded as `records/size-m2p1-<utc-date>-<shortsha>.md`, `size.log`
printing every figure the record carries; `--remove-sample`. Tests
on byte-built packages: remove the last of two (the `<sheet>`, the
relationship, the override, the entry, its rels part); remove a
sheet with a drawing whose chart is reachable only from it (four
entries and two rels parts gone, overrides gone); the same drawing
also reached from a kept sheet (the sheet's part and rels part gone,
the drawing kept); a comments part and its VML reached only from
the sheet; a relationship to no entry; an external relationship;
the sheet with formulas and a chain (the recipe, the flag); no
chain (nothing on that point); the first sheet removed with
`localSheetId` 0 and 1 (0 gone, 1 becomes 0) and `activeTab` 0
(becomes 0, the first visible), `activeTab` 1 (becomes 0);
`activeTab` naming the removed sheet with the first remaining sheet
hidden (the first visible's index); the last sheet (refused); the
last visible sheet (refused); a name that is not there (refused);
a chartsheet removed; remove then add of the same name (allowed;
the entry name may be reused, the ids are new); every byte outside
the regions identical on a workbook with `extLst` and
`pivotCaches`; the tool's line. The negative log through the
binary: `--remove-sample` on a one-sheet workbook (identical
output, `removed` 0) and on a workbook whose only other sheet is
hidden. The corpus run `--remove-sample` on both targets with the
release binaries the size log hashed, recorded as
`records/m2p1-p4-<utc-date>-<shortsha>.md` and `.txt`; acceptance
as "Harness runs". The report states the part's own cost, the whole
growth and the budget figure against the ceiling, and names the
corpus gap (decision 4) as recorded.

### Close-out push (after P4)

As part 8's, with the self-scan rule applied to the push and
ci-run scripts and the scan header naming the m1p8-close finding
(`specs/0015` close-out rulings 4 and 5), one record commit after
the push, the scan rerun last, the `ANTHROPIC_API_KEY` check. The
push carries the four local commits and this part's, on planning's
word and the maintainer's authorisation typed in the implementing
session.

## Decisions for the maintainer

1. The template as six hand-authored XML text files pinned by hash
   (taken as recommended, 2026-09-24). The alternative, a recorded
   script that generates them, adds a script that would write the
   same six files once.
2. P1 first, scripts only, so Excel check 2 runs on Milestone 1's
   code while P2 to P4 are built (taken as recommended). The
   alternative runs the check after the part closes.
3. Removal takes the parts only the removed sheet reaches (taken as
   recommended). Leaving them as orphans is less code but leaves
   parts no relationship names, which the Excel check has not seen.
4. Corpus P5 and P6 are not supplied before P4 (the maintainer's
   choice, 2026-09-24: "no supplying yet"). P4 runs with the gap
   recorded: no macro, threaded-comment, chartex or Google Sheets
   workbook has had a sheet removed. The gap closes when the files
   arrive, in the next part that runs `--remove-sample`.
5. Corpus rows opened by the PowerShell script too, values read at
   the references through COM, the maintainer opening by hand only
   a corpus row the script refused (taken as recommended with the
   spec's approval, 2026-09-24).
   This amends Excel ruling 1's "the corpus rows stay by hand" for
   check 2 on. The alternative is 54 corpus rows by hand. The values
   the script reads are the sample's constants and never a corpus
   value; a refused row's error text stays in the results file the
   maintainer brings back and never enters the record.

## Out of scope

Cell styles beyond part 5's (part 2); rows, columns, freeze panes,
merges (part 3); defined names as an edit, hyperlinks, comments,
data validation, conditional formatting, tables, protection,
document properties, images (part 4, behind features or a second
crate); copying a sheet with its cells; a template of the caller's
own (open it and edit it, which is what emend is); fixing formula
text, defined-name text or pivot sources when a sheet is renamed or
removed; `docProps/app.xml`'s title list; formula evaluation,
rendering, autofit and reference fixup (`CLAUDE.md`).
