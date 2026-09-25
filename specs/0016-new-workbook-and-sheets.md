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

### P5: one copy of the read and write paths

Ruling 11's cause, removed inside the crate. The public types keep
their parameters (`Package<R>`, `Archive<R>`, `Cells<'_, R>`,
`Stream<R>`, `write_to(self, out: impl Write)`); underneath, the
reader is erased once, at `Archive::open`, and the writer once, at
`write_to`, so every body below that boundary is compiled once,
over trait objects, and a second reader or writer type costs a
thin shell. Planning's preference, not a ruling: `Archive` holds
`Option<zip::ZipArchive<Box<dyn ReadSeek>>>` with a private
`trait ReadSeek: Read + Seek` and its blanket impl, a `PhantomData`
keeping the parameter, and the functions under `write_to` take
`&mut dyn Write` from the `impl Write` shell. Before its code
commit the session proposes the mechanism with the list of every
function whose signature changes, and planning rules. No new
dependency, no `unsafe`, no public signature changed, no change to
what any function reads, writes or refuses, no test edited except
for a type name a changed private signature forces, each such edit
named in the report.

The doc commit first (the P4 rulings and verification, this section,
ROADMAP), then the code commit or commits, then the logs: gate;
test on both targets; size.log as P4's with P4's record as the
previous record, the thirteen probes rebuilt, the classification
read off the bytes as before, and the sheets probe's difference
from P4's 1012208 printed with every earlier probe's difference
from its P4 figure; recorded as
`records/size-m2p1-<utc-date>-<shortsha>.md`, the part's size
record from then on; run.log: `--remove-sample` on both targets
with the binaries size.log hashed, every output byte-identical to
P4's run directory's output for the same corpus file by sha256,
printed 27 of 27 per label, with the open check as P4's; recorded
as `records/m2p1-p5-<utc-date>-<shortsha>.md` and `.txt`. No
negative log: the phase adds no refusal. Acceptance: the tests pass
unchanged; the sheets probe at least 100000 section bytes below
1012208; the budget figure under 393216; the corpus outputs
identical. The report states the budget figure against the
ceiling, the headroom left, the largest growth among the twelve
earlier probes (a trait object can cost a single-reader consumer a
little; the figure is reported, not hidden), and the whole growth
since part 6 restated from P5's record. Should the figure land
above the ceiling still, the report says so and the maintainer
re-rules before P5's record commit.

### Close-out push (after P5)

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
6. The size finding (P4 rulings 10 and 11): P4's record committed
   as measured with the excess a finding; a P5 that removes the
   second copy of the read and write paths and measures the
   thirteen probes again; the ceiling unchanged until P5's figure
   exists (taken as recommended, 2026-09-25). The alternatives were
   raising the ceiling to cover a duplication that should not
   exist, or changing the probe to hide the cost.

## Planning rulings during P1 (2026-09-24)

Asked by the implementing session before P1's code commit, after
the doc commit 1450dbd (verified by planning: parent d4ab367, the
three files unedited at the hashes sent).

1. Q1: the spec's negative case named a hash the summary does not
   hold; `summary.host.tsv` carries `out_bytes` and no sha256. The
   list script refuses, for `--run` and `--run-plain` alike, a
   corpus output whose size differs from the summary's `out_bytes`
   ("error: a corpus output's size differs from the summary"), and
   the negative case plants a one-byte append on a copied output.
   Every hash in the list is still checked against its file by the
   run log; the spec's P1 sentence reads as this ruling from now.
2. Q2: `open-results.txt` carries a label column, since each
   corpus id appears once per run directory: `source`, `id_or_test`,
   `label`, `result`, `seen1`, `seen2`, `seen3`, `message`; a row is
   unique by the first three. The record template's corpus and
   fixture tables carry the label too.
3. Q3: row order as proposed. Corpus rows from `--run` first, then
   each `--run-plain` directory's rows in the order the flags were
   given, each in manifest order; fixture rows in name order, each
   followed at once by its input row (source `input`, label `-`,
   file `<test>.in.xlsx`, the record's `in_bytes` and `in_sha256`).
4. Q4: the stage layout as proposed: the list beside the script as
   `excel-check-list.txt`; every file at `files\<source>\<file>`,
   the list's forward slashes turned to backslashes by the script; a
   corpus row's file column relative to the corpus root as in check
   1. The script refuses a `--run-plain` directory whose corpus root
   differs from `--run`'s. The run directory's name in a path
   (date and short sha) is not a corpus path in the privacy sense:
   only the directory above the corpus root is.
5. Q5: the value read as proposed. The first worksheet is
   `Worksheets.Item(1)` (chart sheets excluded, hidden sheets
   included, matching the list's definition); each value is
   `Range(ref).Value2`; a whole-number double is written without
   decimals, another double in the invariant culture, an error
   value (`Value2` an `Int32`) as the cell's `.Text`. A read that
   throws on a workbook that opened writes `read-error` in the seen
   cell, leaves `result` `pass` and puts the exception text, tabs
   and line ends replaced by spaces, in `message`. `Value2` and not
   `.Text`, which depends on the column's format and width.
6. Q6: as proposed: `Workbooks.Open(path, 0, $true)` (links not
   updated, read-only), Excel visible with alerts on, `Quit` at the
   end, results written as ASCII.
7. Q7: the edit open check's record for P1's regenerated keep is
   `records/m2p1-open-<utc-date>-<shortsha>.md` and `.txt`,
   installed by the run log and committed with P1's record commit,
   as part 8 P2's was; the list's `--record` names its `.txt`.
8. The style-sample run is host only, so the record script runs
   with `--allow-single` and the record says host only; part 5 P3's
   expectations apply to the host label.

9. Q8: the spec named the wrong phase, and so did ruling 8. Part 5
   P3's corpus run was `--date-sample`; the `--style-sample` run and
   its expectations are part 5 P2's (`specs/0012` "Harness runs",
   bullet P2; `records/m1p5-p2-2026-09-22-7c96575.md`), which
   planning confirmed from `records/logs/m1p5-p2/run.log` and
   `m1p5-p3/run.log`. Every "part 5 P3's expectations" in this spec
   and in ruling 8 reads as part 5 P2's `--style-sample`
   expectations on the host label: the styles part and the first
   worksheet part changed, every other entry identical and
   unflagged, `descriptor` alone allowed on a changed entry,
   `order_preserved` yes; `lo` and `lo_in` ok; the tool's asserted
   `xfs_after` and `cells_after`; `read_back` equal to `styled` plus
   2 in every file; totals `xfs_before` 992, `xfs_after` 1046,
   `formats_added` 27, `styled` 27.
10. Q9: yes. The run log reruns part 5 P2's comparison over the 27
   host pairs as part 8 P2 reused part 6 P2's: the printed script is
   extracted from `records/logs/m1p5-p2/run.log`, its sha256 checked
   against the value that log prints, `cmp.py` taken from its
   heredoc, run on each host pair with its planted-flag controls on
   the first workbook, every per-workbook line asserted as
   expected; no reference, code or text printed.
11. The two unlogged development checks on the `.ps1` (a syntax
   parse by Windows PowerShell through the interop, and the value
   formatting on mock cells) are disclosed in the report and stay
   unlogged: a logged run through the interop would print Windows
   host paths, which a log must never hold. The logged check is the
   spec's grep (ASCII, no absolute path) with its control.

These rulings go in the next doc commit (P2's first), with P1's
verification; P1's code commit cites them by number.

## Planning verification and rulings on P1 (2026-09-25)

Verified from primaries: the doc commit 1450dbd (the three files at
the hashes sent, unedited), the code commit ade3c49 (four files:
the heading, the list script, the `.ps1`, the template; no crate,
tool, manifest or gate file) and the record commit 56e7c34 (eleven
files), each with the one no-reply identity and the Co-Authored-By
line only; every log and record hash the report gives read from the
files; planning's own privacy scan over d4ab367..HEAD, 3 commits,
532 blobs, 0 findings; `ANTHROPIC_API_KEY` unset. Planning's own
checks: the `.ps1` read in full and found as the rulings say; the
list (mode 600, sha256 bcda16d9...) holds 54 corpus rows (27
labelled `host`, 27 labelled by the style run's directory), 138
fixture and 138 input rows, every `--run-plain` and input row with
empty reference columns, each fixture row followed by its own input
row, and the 27 `host` rows equal cell for cell to check 1's list;
the zip (sha256 99b3376d..., 33010705 bytes, mode 600) holds 332
entries under one directory, the list and the script at its root
(the script byte-identical to the committed file), and every one
of the 330 listed files at its path with the list's size and
sha256, nothing else; all 298 keep files match the open record,
whose `.txt` is byte-identical to part 8 P2's; the 27 style-sample
outputs of the new run directory are byte-identical to part 5 P2's
(`2026-09-22-7c96575-c`, the `xltx` included), which is planning's
check that the tool at ade3c49 is part 6 P3's on this operation.

1. G1: attempt 3's form, per-file `git show` copies of the scripts
   under test with their hashes printed and compared with the phase
   commit, the main tree read but never written, meets the rule's
   purpose (the committed bytes are what runs; nothing in the main
   tree is changed or committed) and is accepted for P1. It is a
   departure from the rule's letter, which asks planning's word
   before the run; that word is given here after it, once. From P2
   on, a negative log that runs only repository scripts may use
   this form, and says so in its header with the hashes; one that
   runs a built binary uses a worktree or clone as before.
2. G2: closed by the final scan, whose header names both pre-check
   runs, their results and their hashes; neither is committed and
   nothing else is needed.
3. G3: option (b). P2's first code commit moves `$wb.Close($false)`
   inside a `try`; a `Close` that throws writes `close-error` in
   `message`, keeps `result`, and the run goes on to the next row,
   so a results file is always written. P2's run log then rebuilds
   the stage from the same list (its hash unchanged, since the
   script is not a listed file) and prints the new zip's sha256;
   Excel check 2 runs on that zip and not on 99b3376d....
4. G8: a negative log's printed command is the invocation that
   runs (`specs/0013` "Phases": the log prints the invocation).
   Printing the repository path while running the scratch copy is
   a form defect, accepted as recorded for P1; from P2 on the
   printed command names the copy.
5. G10: the template's by-hand `repair` row stands: alerts on means
   a repair prompt stops the script at that file, and the maintainer
   answers it and notes the row. The script cannot see a repair the
   maintainer accepted, which the template says.
6. F5, F7, G4 to G7, G9 and G11 are report and log nits, recorded
   in the committed answers, none changing a result; accepted.
7. Disclosures accepted: the two unlogged interop checks (ruling
   11); the run log running repository scripts from the main tree
   behind its guards, as part 8 P2's did; the three superseded run
   attempts and two superseded negative attempts, each hashed in a
   rerun header; the external cargo process at attempt 2's entry.
8. P1 is DONE. Excel check 2 waits for P2's first code commit and
   the rebuilt zip (ruling 3); planning then gives the maintainer
   the staging steps. P2 starts on this section, which goes in P2's
   first doc commit with rulings 1 to 11 during P1 and the
   `ROADMAP.md` row.

## Planning rulings during P2 (2026-09-25)

Asked by the implementing session before P2's first code commit,
after the doc commit fc82ca7 (verified by planning: parent 56e7c34,
the two files unedited at the hashes sent). The implementing
session measured the corpus's 781 XML and rels parts as a witness
for the byte form (counts only); planning accepts each proposal.

1. Q1: each template file is the declaration the spec gives, then
   one LF, then the root with no whitespace between elements, and
   no newline at the end; the committed file is the entry's bytes
   exactly. LF, never CR, in a tracked text file.
2. Q2: `styles.xml` as proposed, in Excel's own forms: every
   collection with its `count` (`fills` 2, the rest 1); one font of
   size 11 named Calibri (Excel's default; a fictional template may
   name it, and the font-name rule concerns corpus content); the
   two pattern fills `none` and `gray125`; one border with the five
   empty children Excel writes; the one `cellStyleXfs` and `cellXfs`
   entries as proposed; the `Normal` cell style with `builtinId` 0.
3. Q3: the content types and rels as proposed: `Default` for `rels`
   and `xml`, the three `Override`s with the transitional types, the
   `Relationship` attributes in the order `Id`, `Type`, `Target`.
4. Q4: `--new-sample` writes `<a>` and `<b>` each through a
   temporary in its own directory, moved into place, an existing
   file replaced as the other operations replace their output; on a
   failed assertion `<b>` is removed and `<a>` stays.
5. Q5: the template is documented in `package`'s doc comments (the
   six files, the constant, fictional, no cell, generated by hand
   for this spec); no README in the template directory.
6. Q6: `impl Package<Cursor<Vec<u8>>> { pub fn new() -> Result<Self,
   Error> }`, the constant private, the six files included with
   `include_bytes!` into a private table of entry name and bytes in
   the spec's order.

These rulings go in the next doc commit (P3's first), with P2's
verification; P2's first code commit cites them by number.

## Planning verification and rulings on P2 (2026-09-25)

Verified from primaries: the doc commit fc82ca7 (the two files at
the hashes sent, unedited); the code commits 5f6505a (the `.ps1`
Close in a `try`, the `styles` header sentence) and e7c7061 (the
six template files, the constant, the table, `Package::new`, the
catalogue and limits, the tests, `--new-sample`); the record commit
3a1f0ec (nine files); every log and record hash the report gives
read from the files; planning's own privacy scan over
56e7c34..HEAD, 4 commits, 550 blobs, 0 findings; `ANTHROPIC_API_KEY`
unset. Planning's own checks: the six template files read, each
the declaration, one LF, the root with no whitespace and no
trailing newline, no CR, with the elements the spec and P2 rulings
2 and 3 give; the `package.rs` and tool diffs read; the crate's 386
tests and the tool's 43 passing on planning's own host build;
`--new-sample` run from planning's own release build reproducing
both files byte for byte (sha256 ed286608... and 45963b48..., 2020
and 2095 bytes), `xlsx-diff` over the pair showing the worksheet
alone changed and five entries identical, the first file's six
entries equal to the six template files with zero times and
deflate, and both files converting in the pinned image with the
sheet count agreeing; the rebuilt zip (sha256 b25fdf16...,
33012799 bytes, mode 600) holding 332 entries under one directory,
the script byte-identical to HEAD's and the list unchanged at
bcda16d9..., every one of the 330 listed files at its path with
the list's sha256; all 310 keep files matching the open record.

1. F2: option (a). The run log is the pin; "record" in "The
   template" reads as the record commit that holds it. No separate
   record of the six hashes; P4's size record cites the run log.
2. F4: in P3's first code commit, a `close-error` after a
   `read-error` is appended to the message with a `; ` between,
   never replacing it. The stage is not rebuilt for it: the case
   loses a message's text, never a result or a value, and Excel
   check 2 runs on b25fdf16....
3. F5 and check 2's G9: in P3's first code commit, `package.rs`'s
   two sentences that say neither the write nor the open refuses
   the template's bytes read that the bytes themselves are never
   refused, and that the temporary's I/O can fail as in any write.
   The catalogue's "no test reaches an error from it" stands.
4. F1 and F9 were fixed in negative.log attempt 2 (the gate's host
   binary copied in and checked against gate.log's hash); attempt
   1 is named by hash in the header. Accepted.
5. F3, F6, F7, F8, check 2's G1 and G3 to G11: report and log nits,
   in the committed answers, none changing a result; F8's "22-byte"
   stands as disclosed (the spec's figure, asserted by a test).
   Accepted.
6. Disclosures accepted: the unlogged gate, tests, one `xlsx-diff`
   run and `rustfmt` on four changed files during development,
   each diff inside this phase's code; the one pre-check scan,
   hashed in the final scan's header.
7. P2 is DONE. Excel check 2 runs on the rebuilt zip; planning
   copies it to the maintainer's Windows drive and gives the
   steps. P3 starts on this section, which goes in P3's first doc
   commit with the P2 rulings 1 to 6 and the `ROADMAP.md` row.

## Planning rulings during P3 (2026-09-25)

Asked by the implementing session before P3's main code commit,
after the doc commit 9ae2efe and the first code commit 421bd44
(both verified by planning: the two files unedited at the hashes
sent; the close-error appended after "; "; the two `package.rs`
sentences reworded). The corpus measurements the session gave
(counts only: every workbook part binds the relationships prefix,
none has a prefixed root or a non-sheet child in `<sheets>`; 95
sheets, 48 with `state`; 160 defined names, 38 with `localSheetId`;
27 book views, 17 with `activeTab` and 17 with `firstSheet`; no
`Default` for `xml` of the worksheet type) are the witness below.

1. Q1: the spec conflicted with itself; the sample's cell wins. The
   run log checks the added entry byte for byte against bytes it
   builds itself: the template's worksheet with `<sheetData/>`
   replaced by the one row the cell rewrite writes, and with
   `ElementTree` that it holds one cell, A1, value 7. The crate
   tests keep the blank-bytes check for `add_sheet` alone. "Harness
   runs" P3 reads so from now.
2. Q2: a cell edit on a pending added part is applied at the write
   to the template's blank worksheet bytes through the existing
   worksheet rewrite, and the result is the added entry's bytes:
   one entry, no separate replacement, the rewrite's refusals
   unchanged. `Package::cells` on a pending part reads the file
   state and is refused "no such part", as today.
3. Q3: option (a). A name holding a character XML 1.0 cannot carry
   is refused at record time, with the name rules, by the existing
   message "a text holds a character XML cannot carry"; catalogued
   with the sheet edits as reached from add and rename, with a test.
4. Q4: a name written into an attribute (a new `<sheet>`, or a
   renamed name between its own quotes) is escaped with quick-xml's
   attribute escape: `&`, `<`, `>`, the apostrophe and the quote as
   entities; tab, line feed and carriage return as `&#9;`, `&#10;`
   and `&#13;`, so the name reads back as given. The rename test
   pins it.
5. Q5: a `localSheetId` that is not a decimal `u32` is refused with
   "a sheet edit no longer applies at the write"; one past the
   file's sheet count names no sheet and is copied unchanged, and
   the limits say so.
6. Q6: a new `sheetId` past `u32::MAX` is refused at record time and
   at the write, and a `<sheet>` added to a workbook part with no
   `<sheets>` element is refused at the write, both with "a sheet
   edit no longer applies at the write"; neither is reachable from
   the corpus, and the limits say so.
7. Q7: as proposed. The sample's name is the first free of `emend`,
   `emend2`, `emend3` and so on, compared as the name check compares;
   the tool counts `definedName` elements and reads the first
   `workbookView` with its own quick-xml pass over the workbook
   part, and the crate's API stays as the spec lists it.
8. Q8: the comparison as proposed, one independent Python script
   with a planted control per flag, counts and yes/no only, the
   byte spans measured on the raw part. But not the `firstSheet`
   remap: the spec's rule was wrong for that attribute. `firstSheet`
   is the tab strip's scroll position, not a reference to a sheet:
   an add or a move never changes it, and appending `firstSheet="1"`
   after the sample would scroll the strip past the new first sheet.
   Rule from now, replacing "The workbook rewrite"'s sentence on
   `firstSheet`: `activeTab` is remapped as written there; a present
   `firstSheet` is decremented by the number of removed sheets that
   stood before it and clamped to the projected count minus one, an
   absent one stays absent, and a move or an add leaves it as it
   is. The P3 expectation reads `activeTab` one more and `firstSheet`
   unchanged; P4's reads `firstSheet` as this rule gives.

9. Harness gap, found by run.log attempt 1 after the main code
   commit 770c9dd and confirmed by planning from the scripts: both
   open checks (`scripts/passthrough-run.sh`, `scripts/
   edit-open-check.sh`) compare the output's ODS table count with
   the INPUT's `<sheet>` count, which no earlier operation could
   tell from the output's. `--sheet-sample` adds a sheet, so every
   output read `lo` fail on the count while the ODS held exactly
   the sheets the output declares. Option (a): a P3 code commit 3,
   scripts only. The output's check takes the output as its
   reference workbook; the input's check keeps the input. Earlier
   records read the same, since every earlier operation kept the
   count, and each run log's comparison checks the sheet list
   against the input on its own. The negative log gains two cases:
   an output whose declared count differs from the input's is
   accepted when LibreOffice shows that many tables, and one whose
   table count differs from its own declared count is refused, as
   a control. Then attempt 2 of every log from the gate on, attempt
   1's logs hashed in the rerun headers, attempt 1's run directory
   hashed, removed and printed. The alternative, expecting `lo`
   fail on a sheet-changing operation, is refused: it would blind
   the check to a real miscount. From now the open check's
   definition (`specs/0006` P3) reads: the table count against the
   checked workbook's own sheet count.

10. After code commit 3 (a7f7060) and attempt 2 of the logs (every
   log "yes": `lo` 27 of 27 on both targets, 10 of 10 plants, 173
   pairs and 0 findings in the edit open check, the two ruling 9
   cases holding), independent check 1 raised F1 to F10. Ruled: a
   code commit 4 carrying F1, F2, F3 and F5 as proposed, then
   attempt 3 of every log from the gate on, attempts 1 and 2
   hashed in the rerun headers, attempt 2's run directory hashed,
   removed and printed; check 1's answer kept and committed as
   `independent-check-1.md`; check 2 on attempt 3.
   (F1) The move-remap fixture's deliberate out-of-range
   `localSheetId` makes LibreOffice refuse input and output alike,
   so the remapped output was never seen to open (the session
   bisected it, unlogged, in the pinned image). The test splits:
   the move-remap test loses the out-of-range name and its pair
   opens; a new test keeps ruling 5's past-count case, its pair
   expected refused on both sides, which the open check allows.
   (F2) An `activeTab` or `firstSheet` that is not a decimal `u32`
   is refused at the write with "a sheet edit no longer applies at
   the write", as ruling 5's `localSheetId` is; the catalogue entry
   and the limits name it and a test covers it. Copying such a
   value unchanged is refused: the remap cannot say what it means.
   (F3) `firstSheet` is decremented and clamped only when a sheet
   was removed, as ruling 8 says; a present value past the end is
   left unchanged by a move, with a test.
   (F5) The rename test adds `>` and the LF and CR character
   references; a test covers `Package::cells` on a pending part
   refused (ruling 2).
   (F4) "add then remove of the pending sheet" moves to P4 with
   `remove_sheet`. (F6 to F10) Recorded, no action: the superseded
   run directory hashed through its two summaries; the comparison
   on expat and byte spans, as ruling 8 asked; a long comment line;
   an unreachable overflow; the disclosed unlogged items.

These rulings go in the next doc commit (P4's first), with P3's
verification; P3's code commits cite them by number.

## Planning verification and rulings on P3 (2026-09-25)

Verified from primaries: the doc commit 9ae2efe (the two files at
the hashes sent, unedited); the code commits 421bd44, 770c9dd,
a7f7060 and 09b42e4, each held to its area; the record commit
2797fe9 (eleven files); every log and record hash the report gives
read from the files; planning's own privacy scan over
3a1f0ec..HEAD, 6 commits, 565 blobs, 0 findings; `ANTHROPIC_API_KEY`
unset. Planning's own checks: the whole `package.rs` diff read (the
projected list, the name rules, the three edits, the plan, the
four-region rewrite, the attribute escaping and index reading, the
worksheet rewrite on the blank part); the workspace tests on
planning's own host build (429 crate tests among them), the crate's
415 on i686, and clippy clean; planning's own `--sheet-sample`
corpus run through the runner with the pinned image: 27 ok, `lo`
and `lo_in` ok on 27, every output byte-identical to the phase's
run directory; planning's own comparison over the 27 pairs, every
check on 27 of 27: every entry other than the workbook part, its
rels part and the content types identical and exactly one entry
added; the rels part one relationship longer; the content types
one `Override` longer or covered; the sheets equal to the input's
with the new sheet first and its `sheetId` the largest plus one;
every `localSheetId` one more; every `activeTab` one more;
`firstSheet` unchanged; the added entry the template's worksheet
with one cell A1 holding 7; the workbook part identical outside
the three regions; 122 sheets after; all 350 keep files matching
the open record.

1. F2: `package.rs`'s limits gain a bullet in P4's first code
   commit naming the two refusals ruling 6 gives (a `sheetId` past
   `u32::MAX`, a sheet added to a workbook part with no `<sheets>`),
   each unreachable from the corpus. The catalogue alone does not
   satisfy "the limits say so".
2. F1: the failed pre-check scan (an answer installed before the
   pre-check stage) is hashed in the final scan's header with the
   two others; accepted, nothing else needed.
3. F3 to F8: report and log nits, in the committed answer, none
   changing a result; F7 (`--sheet-sample` exits 2 on a workbook
   with no book view) is the spec's expectation and every corpus
   workbook has one. Accepted.
4. Disclosures accepted: the unlogged bisection of check 1's F1 in
   the pinned image; tests, fmt and clippy before code commit 4;
   check 1's F4 deferred to P4 with `remove_sheet`.
5. P3 is DONE. P4 starts on this section, which goes in P4's first
   doc commit with the P3 rulings 1 to 10 and the `ROADMAP.md` row.
   P4's first code commit carries ruling 1 and check 1's F4 (add
   then remove of the pending sheet) beside `remove_sheet`.

## Planning rulings during P4 (2026-09-25)

Asked by the implementing session before P4's first code commit,
after the doc commit d6396bb (verified by planning: parent 2797fe9,
the two files unedited at the hashes sent). The corpus counts the
session gave (95 sheets: `state` absent on 47, `visible` on 43,
`hidden` on 5, no other value) are the witness for ruling 1.

1. Q1: the spec's "no state" was wrong; the corpus writes
   `state="visible"` on 43 of 95 sheets. A sheet is visible when
   its `state` is absent or exactly `visible`; any other value
   (`hidden`, `veryHidden`, or anything else) counts as hidden.
   This reading governs the last-visible refusal, the first visible
   sheet an `activeTab` of a removed sheet goes to, and
   `SheetPlan.first_visible`, changed in P4's first code commit,
   with the test proposed (the first sheet removed when the second
   says `visible`: allowed, `activeTab` 0). "Removing a sheet" and
   "The workbook rewrite" read so from now.
2. Q2: as proposed. A file entry recorded for removal still counts
   as taken, so a later `add_sheet` takes the next free
   `sheet<n>.xml`; a pending sheet's removal frees its name; the
   `sheetId` is the projected maximum plus one and may equal a
   removed sheet's; a relationship id is never reused, since the
   file's ids still count. The remove-then-add test asserts exactly
   that; the spec's "the entry name may be reused" reads as this.
3. Q3: as proposed. K leaves out every workbook relationship
   recorded for removal so far and follows no edge out of any
   removed sheet's part; an entry already recorded for removal is
   not recorded again, so a part reached only from two removed
   sheets goes with the second; the chain recipe is recorded once,
   through the once-guard the formula-change rule shares, and
   `fullCalcOnLoad` is set. With one removal this is the spec's
   definition, which the run log's walk exercises. The two-sheets-
   one-drawing test covers it.
4. Q4: as proposed. Removing a pending sheet records a `Remove`
   edit so the projection and later edits by name stay coherent,
   and drops its part, `Override` and relationship additions and
   the cell and style edits on its part; no walk and no chain rule.
   F4's test asserts the output byte-identical to the input when
   the add and the remove are the only edits.
5. The stated order and reads are accepted: "no sheet of that
   name", then the last sheet, then the last visible sheet; every
   read before the first record; a reached rels part that cannot
   be read refuses the removal; a target outside the package or
   naming no entry contributes nothing.

6. Q6, after code commit 1 (8abc666): the probe is as "Size"
   writes it, and the size log runs it on a generated two-sheet
   package whose first sheet has no edits and whose second is the
   one the command line names for the cell, style and formula
   edits; removing "the sheet that was first" then removes an
   unedited sheet, as the spec meant, and the write's refusal of an
   edit to a removed part (ruling 3 and the limits) is not
   exercised by the probe. The expected lines are the formulas
   probe's, then `sheets` (2), `first_name`, `first_part` and
   `new_bytes`. The other twelve probes run on part 6's fixtures
   unchanged.
7. Q7: "name length" is the renamed name's length in UTF-8 bytes
   and "part length" the added part's entry name length in bytes,
   both read from the reopened output's `Workbook`, so the rename
   and the add are read back and nothing more is linked.
8. The report's claim that a planted mutation of `is_visible`, the
   `firstSheet` guard and the K filter is caught is a measurement
   (`specs/0001`, "Rules for every phase"): it appears in
   `test.log` with the planted change, the run and the failing
   assertion's output, or it is not made anywhere.

9. Q8, from an unlogged dry run before any log (disclosed): 17 of
   the 27 corpus workbooks carry `Override` elements for rels
   parts (68 in all), and the spec's wording removed only a
   removed entry's own `Override`, leaving overrides naming rels
   parts that no longer exist in 4 outputs. Ruled as proposed:
   every entry the walk removes, rels parts included, loses its
   `Override` when the content types part holds one, so no
   `Override` names a removed entry. Code commit 3 in `package.rs`
   and its tests, with a limits sentence and the test proposed (a
   sheet whose rels part has an `Override`: both dropped); the run
   log's walk expects it. Leaving dangling overrides is refused:
   nothing has shown Excel accepts them. "Removing a sheet" step 3
   reads so from now.

10. The size finding, measured by size.log attempt 3 at the phase
   commit 3bd4257 (i686 musl, the `size` profile, section bytes):
   `probe-zip-xml` 533216 (drift 0); `probe-emend-formulas-write-
   xml` 876240 against part 6's 845212 (drift 30212, 8 commits
   touching the crate, 7 of them this part's); `probe-emend-
   sheets-write-xml` 1012208; the budget figure 478992 against the
   ceiling of 393216, 85776 over; the part's own cost 135968
   against planning's expectation of under 25000. Attempt 3 also
   failed on the session's own defect (the formulas probe's run
   block reading the sheets probe's variable), fixed for the next
   attempt. The cause is a hypothesis, not a measurement: the
   probe holds a `Package<BufReader<File>>` and a
   `Package<Cursor<Vec<u8>>>` from `Package::new`, and calls
   `write_to` on both, so the generic write path is compiled twice.
   Ruled: before anything else, the cause is measured, as a logged
   step (`size-attribution.log`, committed with the phase's logs):
   in a scratch worktree at 3bd4257, three variants of the sheets
   probe built and measured as size.log measures, each variant's
   diff printed: (a) no `Package::new` call; (b) no sheet edits;
   (c) `Package::new`'s bytes written to a file and reopened
   through `BufReader<File>`, so one reader type is instantiated.
   The log prints each variant's section bytes and its difference
   from the probe as committed. No crate code changes and no
   record commit until the maintainer has re-ruled on those
   figures, as "Size" says; planning takes them to the maintainer
   with a recommendation. run.log finishes first; the attribution
   runs with no other cargo.

11. The attribution, `size-attribution.log` at 3bd4257, reported by
   the session at sha256 929f8152... and committed with P4's logs,
   where planning verifies it. The control rebuilt the committed
   probe at 1012208 section bytes. The variants: (a) no
   `Package::new` call, 903196, a difference of -109012; (b) no
   sheet edits, 986700, -25508; (c) `Package::new`'s bytes reopened
   through `BufReader<File>`, 1012384, +176. The cause is now a
   measurement: the sheet edits and their rewrite cost about 25500,
   near the expectation; `Package::new` costs about 109000, and (c)
   shows the probe cannot avoid it, because the constructor's own
   body opens and writes a `Package<Cursor<Vec<u8>>>`, so `Archive`,
   `Package` and everything under `write_to` are compiled a second
   time for that reader and for `Vec<u8>` as the writer, beside the
   copy over `BufReader<File>` every consumer already carries. With
   that copy gone the budget figure would be about 369980, under the
   ceiling by about 23000, and the part's own cost with the drift
   about 56000, above the expectation and reported as "Size" asks.
   The maintainer re-ruled on 2026-09-25 (decision 6): P4's record
   is committed as measured, the excess a finding; P5 removes the
   second copy and measures again; the ceiling is not changed before
   P5's figure exists. So P4 finishes as follows. size.log attempt 4
   at 3bd4257 with the script's own defect fixed and nothing else it
   measures changed; its record,
   `records/size-m2p1-2026-09-25-3bd4257.md`, states that the budget
   figure is above the ceiling by 85776 (the generated sentence
   reads "above the ceiling by" when the figure is above it, never a
   negative number "under" it), names this finding, ruling 10's
   three figures and P5 as the fix, and stands as the measurement
   P5's record is compared with. run.log attempt 3 stands when the
   binaries attempt 4 hashes are the ones it ran (the same sha256
   lines); if the rebuild's hashes differ, run.log reruns. Then the
   independent check or checks, the scan, and one record commit
   with `size-attribution.log` among the logs. No crate change in
   P4.

These rulings go in P5's doc commit, with P4's verification;
P4's code commits cite them by number.

## Planning verification and rulings on P4 (2026-09-25)

Verified from primaries: the three code diffs read (8abc666,
b61fde5, 3bd4257: `is_visible`, `remove_sheet`, `reachable`,
`drop_pending_sheet`, the chain reads before the first record, the
rels-part overrides of ruling 9, `SheetPlan::changes`); the record
commit c7d7870 (parent 3bd4257, 14 files, every sha256 the report
gave reproduced); planning's own privacy scan of c7d7870 (570
blobs, 0 findings); the tests on planning's own build (585 on the
host, 585 on i686, clippy clean); planning's own rebuild of three
probes in the `size` profile on i686 (the sheets probe 1012208
section bytes at the same sha256 as size.log's, `probe-zip-xml`
533216, the formulas probe 876240, so the budget figure 478992 and
the finding are reproduced); planning's own `--remove-sample` run
on the host with its own binaries, 27 ok and all 27 outputs
byte-identical to the run directory's; and planning's own walk, a
script of its own over both labels' outputs: 27 of 27, 3 outputs
identical to their inputs, 24 sheets removed, 93 entries removed,
the removed set exactly R minus K with the rels parts and the
chain, no `Override` naming a removed entry, the workbook
relationships, `<sheets>`, `<definedNames>`, `activeTab` and
`fullCalcOnLoad` as "Removing a sheet" says, every other entry
byte-identical. The two check answers were read from the committed
files.

1. The check answers given by committed path and sha256 rather
   than pasted are accepted; `specs/0001` step 4 allows it, and a
   retyped answer could differ from the committed bytes. The same
   holds for later phases.
2. The ceiling check stays a check, and size.log ends "no" with
   that check alone listed. A failing measurement is a finding the
   log states itself; a printed comparison would hide it. The line
   "the 80404 part 6 left becomes -85776" is arithmetic, not a
   claim, and stands.
3. Check 1 F12 and check 2 G9: test.log's heading "each caught by
   a test this phase adds" is wrong for the first-sheet mutation,
   which `a_move_leaves_a_first_sheet_past_the_end_unchanged`, a P3
   test from 09b42e4, catches. Accepted as recorded with this ruling
   as the correction: the planted change, the run and the failing
   assertion are in the log, which is what ruling 8 requires, and
   the heading reads "caught by a test in the tree". No rerun for a
   heading.
4. Check 1 F13, check 2 G10 and G12: attempts 1 and 2 of size.log
   and run.log never ran, the gate's failure having stopped those
   rounds, and test.log's attempts 1 and 2 were stopped by the
   driver for the same reason, not by the environment, so
   `specs/0001`'s stopped-run header does not apply. "Attempt 3"
   is a round label. Recorded here; no log changes.
5. Check 1 F3 and check 2 G15: the name rule's approximation
   sentence is in the `package` header from 770c9dd (P3), verified
   by planning with `git log -S`. "P4" listed it as if new; nothing
   is missing.
6. Check 1 F20: the corpus gap of decision 4 is named in the
   session's report, as "P4" asks, and in ROADMAP; the generated
   records do not carry it and need not. The gap stands: no macro,
   threaded-comment, chartex or Google Sheets workbook has had a
   sheet removed. It closes in the first `--remove-sample` run
   after those files arrive.
7. F11 is fixed by attempt 6 (check 2 G7); F16 and F18 are fixed.
   F17, G14, G16, G17, G18 and G19 name report sentences with no
   primary behind them, each disclosed as unlogged or carried by
   the log and the record (G19: the part's own cost 135968 is
   above 30000, as size.log and the record say). Accepted; nothing
   to change.
8. P4 is DONE as measured, with the finding. P5 follows as its
   section says. P5's doc commit carries the rulings during P4 (1
   to 11), these, decision 6, the P5 section and ROADMAP, with
   planning's two files unedited at the hashes sent.

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
