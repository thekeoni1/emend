# 0011: Milestone 1 part 4, editing cells

Status: APPROVED by the maintainer 2026-09-22 (planning draft the
same day), the three decisions under "Decisions for the maintainer"
each taken as planning recommended; sent to the implementing session
the same day.
Context: `CLAUDE.md` (what emend does not do: no evaluation, no
reference fix-up), `ROADMAP.md` ("Milestone 1", parts 4 and 6),
`specs/0010-reading-cells.md` ("Module `cells`", "Facts planning
checked", the P3 and close-out rulings), `specs/0009-package-layer.md`
("Module `xml`", "Module `package`": the edits, `write_to`, the recipe
that drops the calculation chain), `specs/0008-zip-layer.md`
("Touched entries", "Size"), `specs/0004-xlsx-diff.md` (statuses and
flags), `specs/0001-bootstrap.md` ("How this task runs", "Rules for
every phase").

## Goal

The core's first edit of content: set a cell to a number, a text or a
boolean, or clear it, in any worksheet of an existing workbook, and
write the workbook back with every other byte intact. The edited
worksheet part is rewritten by streaming: every token the edit does
not touch is copied from the input's own bytes, an edited cell's
element is replaced, a new cell is inserted where the format's order
puts it, and a new row is created when the cell's row does not exist.
Nothing else in the part changes: not `<dimension>`, not a row's
attributes, not the elements after `<sheetData>`. No other part is
touched, with one exception: a cell edit that removes a formula
changes the formulas, so the calculation chain is dropped by the part
2 recipe, which moves into the crate for that. Formulas are not
written in this part; they are part 6's, with the calculation rule
that goes with them.

The harness is the acceptance: over the corpus, an edit of four cells
in one worksheet leaves every other entry byte-identical, the edited
part's bytes outside `<sheetData>` identical, and its cells identical
except at the four references, as an independent parser in the run
script shows; every output opens in LibreOffice; emend's own reader
reads the values back. The comparison point is openpyxl's
`cell.value = x` followed by `save`, without the parts openpyxl
regenerates.

## Facts planning checked (2026-09-22)

Over the 27 corpus workbooks, by element shapes and counts with
Python's `zipfile`, `expat` and regular expressions, never the cells'
text, in addition to the facts `specs/0010` recorded:

- In every one of the 95 worksheet parts the rows are in ascending
  order and the cells of every row are in ascending column order,
  with no row number and no reference repeated (0 descending rows, 0
  descending cells, 0 duplicates). Every `<row>` and every `<c>`
  carries `r` (`specs/0010`), so an insertion point is determined by
  the neighbours' references and never by counting. No `<c>` occurs
  outside a `<row>`.
- Every worksheet has `<dimension>` and no `<sheetData/>` is empty
  (`specs/0010`). The elements that follow `</sheetData>` at the root,
  by part count: `pageMargins` 95, `pageSetup` 95, `printOptions` 84,
  `headerFooter` 78, `mergeCells` 47, `dataValidations` 44,
  `tableParts` 37, `drawing` 37, `hyperlinks` 33,
  `conditionalFormatting` 29, `extLst` 12, `legacyDrawing` 11,
  `sheetProtection` 7, `ignoredErrors` 7, `phoneticPr` 4,
  `mc:AlternateContent` 2, `autoFilter` 2, `dataConsolidate` 1. The
  rewrite copies all of them raw.
- The texts of number cells (`t` absent or `n`) with a `<v>`: 1583695
  are digits with an optional sign, 505253 carry a decimal point, 94
  carry an exponent, written `E` followed by digits (61) or `E-` and
  digits (33), never `E+`; 269 are longer than 17 bytes. So the
  exponent forms this part writes have producers' witnesses in the
  corpus and the plus sign, which xsd:double allows, has none.
- The only attributes on `<c>` are `r`, `s` and `t`; no cell carries
  `cm`, `vm` or `ph`. No `inlineStr` cell exists, so the inline form
  this part writes has no corpus witness; LibreOffice's opening in
  the harness is its witness here, and desktop Excel's at the
  milestone check.
- Rust 1.98.1 (the pinned toolchain): `f64`'s `Display` prints the
  shortest digits that read back to the same value, never an
  exponent, so `1e21` prints as a 1 and 21 zeros and `f64::MAX` as
  309 digits; `{:E}` prints `1E21`, `1.5E-8`, `2.5E0`. `0.1 + 0.2`
  prints `0.30000000000000004`; `-0.0` prints `-0`.
- Identifiers this spec names were grepped in the pinned crate's
  source when written: `quick_xml::escape::escape` (0.42.0,
  `src/escape.rs`), which `package` already uses for attribute
  values. `Package::write_to` rewrites every edited content types and
  relationships part to a temporary file before any byte of the
  output archive is written, so a refusal found while a part streams
  leaves no output.

## Design

### Crate

No new dependency. The `cells` module gains `Value`; `lib.rs`
re-exports it. The `package` module gains `Package::set_cell`,
`Package::clear_cell` and `Package::drop_calc_chain`, and `write_to`
learns to rewrite a worksheet part. Every row and column number is
`u32`, every byte position `u64`, as before.

### Module `cells`: what can be written

`Value`: `Number(f64)`, `Text(String)`, `Boolean(bool)`. Nothing else
in this part: a formula is part 6's, a date is part 5's (a number
with a date style is written as the number it is), an error value has
no use a caller has asked for.

The forms written, each on one line with no whitespace added, so the
new bytes sit between the input's own bytes exactly as a cell would:

- `Number(v)`: `<c r="A1"><v>2.5</v></c>`, no `t` (the schema's
  default, the form 8233 corpus cells use). The text is `v`'s
  `Display` when `v` is zero or `1e-7 <= |v| < 1e21`, else `{:E}`,
  so a value prints as `2.5`, `100`, `-0`, `0.30000000000000004`,
  `1E21` or `1.5E-8`, and never as hundreds of digits; the boundary
  is ECMAScript's `Number::toString`, chosen because it is a
  published rule, not because Excel needs it. A value that is not
  finite is refused at write with `Error::Package("a number that is
  not finite cannot be written")`: the format has no NaN and no
  infinity.
- `Text(s)`: `<c r="A1" t="inlineStr"><is><t xml:space="preserve">s
  </t></is></c>` (one line; the break is this document's). The text
  is escaped with the crate's `escape` (`&`, `<`, `>`, `"`, `'`);
  a carriage return is written as `&#13;`, since a raw one would be
  normalised away by any reader (part 3 P1 finding P1-F1), and
  `xml:space="preserve"` is always written, as 110010 of the corpus's
  110666 `<t>` elements carry it. A text holding a character XML 1.0
  forbids (U+0000 to U+0008, U+000B, U+000C, U+000E to U+001F, U+FFFE,
  U+FFFF) is refused at write with `Error::Package("a text holds a
  character XML cannot carry")`; a tab and a line feed are kept as
  written. The inline form touches the worksheet alone; appending to
  the shared strings table is a decision below.
- `Boolean(b)`: `<c r="A1" t="b"><v>1</v></c>` or `<v>0</v>`.
- Clear: an existing cell with `s` becomes `<c r="A1" s="3"/>`; one
  without `s` is removed, its own bytes alone, the whitespace around
  it kept (the rule `package` applies to a removed element); a cell
  that does not exist is nothing. A row left with no cells stays, with
  its attributes.

When an existing cell is replaced, its `s` is kept byte for byte and
written after `r`; its `t`, its `<f>`, its `<v>` and its `<is>` are
gone. A new cell has no `s`; styling a cell is part 5. An edited
cell carrying any attribute other than `r`, `s` and `t` is refused
with `Error::Package("a cell carries an attribute this layer does not
model")`: `cm`, `vm` and `ph` bind the cell to parts this layer does
not read, and a rewrite that dropped them would break what they name;
an unedited cell is copied raw, whatever it carries (P1 ruling 4). No
corpus cell carries one.

### Module `package`: recording and applying cell edits

`Package::set_cell(&mut self, part: &[u8], reference: CellRef, value:
Value)` and `Package::clear_cell(&mut self, part: &[u8], reference:
CellRef)` record, as the part 2 edits do: nothing is read, nothing can
fail, and the last edit recorded for a reference wins. `part` is the
worksheet's entry name, as `Package::cells` takes it; the caller finds
it through `Workbook::sheet`.

`Package::drop_calc_chain(&mut self) -> Result<bool, Error>` is the
part 2 P3 recipe moved from `tools/emend-copy` into the crate: it
reads the workbook (so it can fail as `workbook()` does), and when the
workbook has a `calcChain` relationship it records the three removals
(the relationship, the override when one exists, the part when the
entry exists) and returns `true`; otherwise it records nothing and
returns `false`. `--drop-calc-chain` calls it and its output does not
change (P1's corpus run shows that against part 2 P3's expectations).

`write_to` applies the cell edits after the validation part 2 defined
and before the content types and relationships parts are rewritten:

- Validation adds: a cell edit names an entry that exists (else "no
  such part"), that no edit of this write removes ("an edit to a
  removed part") and that no edit of this write replaces
  (`Error::Package("a cell edit to a part this write replaces")`);
  every `Number` is finite and every `Text` carries no forbidden
  character, the two messages above.
- Then each worksheet with edits is read through `Stream` and `Scope`
  and written to a temporary file, with the element resolution
  `Cells` uses (the implementing session may share that code): the
  root must be `worksheet` in the main namespace (the `cells`
  message); every token's `raw` is copied except as follows. The
  edits are held sorted by row then column. A `<c>` whose reference
  has an edit is replaced: its start tag, its children and its end
  tag (or its one empty-element tag) are dropped and the edit's form
  is written in their place. Before a `<c>` whose column is greater
  than a pending edit's in the same row, and before `</row>` for
  every edit still pending in that row, the pending cells are
  written. Before a `<row>` whose number is greater than a pending
  edit's row, and before `</sheetData>` for every row still pending,
  a new row is written as `<row r="n">`, its cells and `</row>`. An
  empty `<sheetData/>` is rewritten as `<sheetData>`, the rows and
  `</sheetData>`, the rest of its bytes kept as part 2's rule for an
  empty root keeps them. Clears of cells that do not exist write
  nothing. Everything before the root, everything outside
  `<sheetData>`, every `<row>` start tag with its attributes, and
  every untouched `<c>` are the input's bytes.
- Refusals found while a part streams, each leaving no output: a
  `<row>` or a `<c>` without `r` (`Error::Package("a worksheet
  without explicit references cannot be edited")`; an implied position
  would move when a cell is inserted before it); a row number not
  greater than the previous row's, or a cell's column not greater than
  the previous cell's in its row (`Error::Package("a worksheet whose
  rows or cells are out of order cannot be edited")`; an insertion
  point would be ambiguous, which is also why a `<c>` whose
  reference names another row than its `<row>` is refused this way,
  P1 ruling 4); an edit to a cell whose `<f>` is a shared master, a
  shared follower, an array master or a data table master, or whose
  reference lies inside the `ref` of an array or data table master
  already seen (`Error::Package("a cell in a shared, array or data
  table formula cannot be edited")`; the other cells of the formula
  would be left pointing at nothing, and rewriting them needs a
  formula parser this crate does not have; data tables by P1 ruling
  3); the unmodelled-attribute refusal above; a set on a part with no
  `<sheetData>` (`Error::Package("a worksheet without sheetData
  cannot be edited")`, P1 ruling 2; a clear alone is a no-op there).
  The ranges seen are a few `ref` values held as row and column
  bounds; a `ref` that does not parse as `A1:B2` or `A1` is refused
  with the same message, edited or not, since its extent is unknown
  to every later edit (P1 verification ruling 7). The three `cells`
  messages for a reference not of the form A1, a row number that is
  not a number and, on an edited cell, a formula type this layer
  does not know are raised here too (ruling 1). New elements
  take the qualified-name prefix of the element they go into; an
  empty `<row/>` gaining a cell is opened up as an empty
  `<sheetData/>` is; a new row goes immediately before the next
  `<row>` start tag or `</sheetData>`, after the whitespace the input
  wrote there (P1 ruling 4).
- A replaced or cleared cell whose `<f>` was a normal formula removes
  a formula. When any edit of the write does so and the workbook has
  a `calcChain` relationship, the recipe is applied in the same
  write, unless the caller has already recorded it (the relationship
  is already pending removal): the chain lists that cell, and Excel
  repairs a workbook whose chain names a cell with no formula. This
  is part 6's rule applied in the one direction this part can cause;
  see the decisions below.
- The temporary becomes the part's replacement and the rest of
  `write_to` runs as before. A worksheet is rewritten at most once per
  write; all its edits go in the one pass.

Consequences a caller should know, stated in the module's Limits: the
cached values of formulas that depend on an edited cell are not
recomputed (emend does not evaluate; whether the workbook asks Excel
to recalculate on open is part 6's `calcPr` question); `<dimension>`
is not updated when a new cell lies outside it (a decision below);
`<row>`'s `spans` is a hint and is not updated; a cell written in a
row that had `customFormat` takes no style from the row.

### Errors

Eight new `Error::Package` messages (seven in the draft; the eighth
and the rewording by P1 rulings 2 and 3): "a number that is not
finite cannot be written", "a text holds a character XML cannot
carry", "a cell carries an attribute this layer does not model", "a
cell edit to a part this write replaces", "a worksheet without
explicit references cannot be edited", "a worksheet whose rows or
cells are out of order cannot be edited", "a cell in a shared, array
or data table formula cannot be edited", "a worksheet without
sheetData cannot be edited". Each joins the catalogue in
`package`'s module header with the test that pins it, and the
set-equality test reads the module that raises it.

### 32-bit and streaming

The rewrite holds one token, the sorted edits, the open row's number
and last column, and the array ranges seen; the part streams to a
temporary file as every rewritten part does, so the 63.7 MB corpus
part costs no memory. Edits are recorded in a `Vec` in call order and
sorted into a map from `CellRef` to `Option<Value>` per part at
write, the last recorded winning (P1 verification ruling 2); a caller
that records more edits than its address space holds cannot, and the
Limits say so.

### Size

`tools/size-probe` gains `probe-emend-cells-write-xml`, built in the
`size` profile on i686 musl as the others: `probe-emend-cells-xml`'s
work, then on the sheet named on its command line `set_cell` of a
number, a text and a boolean at three references from its command
line and `clear_cell` of a fourth, `drop_calc_chain`, and the write to
a second path, printing the cell count of the output read back
through `Package::cells`, so no path is dead. The budget figure is it
over `probe-zip-xml` against the ceiling of 393216 section bytes
(`specs/0010` "Size"); part 4's own cost is it over
`probe-emend-cells-xml`. The size log re-measures the eight earlier
probes against part 3's record by the byte classifier. Part 3 left
145868 section bytes of headroom; planning's expectation, not a
measurement, is that part 4 costs less than half of it.

Measured (P3, `records/size-m1p4-2026-09-22-4b31e2a.md`, verified by
planning 2026-09-22 by its own rebuild): the budget figure is 278064
section bytes (282624 file, 211664 `.text`), 115152 under the
ceiling; part 4's own cost as defined above is 4100 sections (8192
file, 3488 `.text`); the budget figure's whole growth since part 3's
247348 is 30716, the other 26616 being the rewrite's cost to
`probe-emend-cells-xml`, which writes a package and so links
`write_to`, with the six commits behind that drift all part 4's.
Both figures stand; see "Planning verification and rulings on P3",
ruling 4.

### Harness runs

`tools/emend-copy` gains two operations, at most one per invocation,
given after the two paths as the others are; neither takes a cell
reference from its command line, so no record names a position in a
corpus workbook.

- `--edit-sample` (P2): the first sheet of kind `Worksheet` in
  workbook order is read once through `Package::cells` to find A, the
  first cell of kind `Number` or `SharedString` with a value and no
  formula; B, the second such cell; and M, the largest row number
  (0 for a sheet with no cells). It then records A set to
  `Number(42.5)`, B cleared, `(M + 1, 1)` set to `Text("emend")`,
  `(M + 1, 2)` set to `Boolean(true)`, writes the package, reopens
  the output and reads the same sheet through `Package::cells`, and
  prints one line, the word `edit` then tab-separated `key=value`:
  `set` (2 or 3, by whether A exists), `cleared` (0 or 1), `dropped`
  (1 when B existed without `s`, else 0), `rows_added` 1,
  `cells_before` (the sheet's count on the first read), `cells_after`
  (the output's count, expected `cells_before + 2 - dropped` and
  asserted by the tool: exit 2 with a message when it differs),
  `read_back` (the number of the three set values the output reader
  returns with the value written: `number()` 42.5, the inline text
  `emend`, `boolean()` true; expected `set`). A workbook whose first
  worksheet has no cells gets A and B absent, M 0 and the two new
  cells in row 1 of a `<sheetData>` that was empty.
- `--edit-formula-sample` (P3): the sheets of kind `Worksheet` are
  read in `<sheets>` order until a cell with a formula of kind
  `Normal` is found; that cell is set to `Number(1)` and the package
  written, and the output is reopened and that sheet read back; the
  line `edit` carries `formula_replaced` (0 or 1), `chain_dropped`
  (1 when the input's workbook has a `calcChain` relationship and the
  reopened output's has none, measured from both reads, else 0),
  `cells_after` (the read-back count, expected equal to the sheet's
  count before, asserted by the tool as `--edit-sample` asserts its
  own) and `read_back` (1 when the edited cell reads back as
  `number()` 1 with no formula, else 0; expected equal to
  `formula_replaced`). A workbook with no normal formula gets no
  edit, a byte-identical output, and the line with every key 0, the
  doc and the test saying a `cells_after` of 0 means no sheet was
  read (P3 rulings 4 to 6).

The runner and the record script are unchanged; each run names its
operation through `--tool-arg`.

Expected results, `lo` and `lo_in` `ok` in every case:

- P1: `--drop-calc-chain` through `Package::drop_calc_chain`, on both
  targets, with part 2 P3's expectations exactly (the nine chains
  `dropped`, the two parts `changed`, the eighteen `files_identical`
  `yes`).
- P2: in every workbook the first worksheet part `changed`, every
  other entry `identical` unflagged (no formula changes, so the chain
  stays), `descriptor` alone allowed on the changed entry as
  `specs/0008` says for a replaced one (P2 verification ruling 6),
  `order_preserved` `yes`, `files_identical` `no`; the run
  log's own check over the extracted input and output parts, with
  `expat` as part 2 P3 counted elements: the bytes before
  `<sheetData` and the bytes from `</sheetData>` to the end are
  identical; the cells, each as (reference, `s`, `t`, formula
  presence, value text), differ from the input's exactly at the
  edited references, printed as counts `unchanged`, `changed`,
  `added`, `removed` per file and asserted against the tool's line
  (`changed` equal to `set` minus 2 plus `cleared` minus `dropped`,
  since a cleared cell with `s` stays and counts as changed; `removed`
  equal to `dropped`; `added` 2; `changed + removed + unchanged` equal
  to `cells_before`; P2 ruling 1, finding P2-F1), and from the
  comparison's own pick of A, B and M: the changed references are
  exactly A and, when B carries `s`, B, which then has no value and
  keeps its `s` bytes; the added references are exactly `(M + 1, 1)`
  and `(M + 1, 2)`; `read_back` equal to `set` in every file; totals
  over the 27 of `rows_added` 27 and `cells_after` equal to the sum
  of `cells_before + 2 - dropped`. The check prints no reference and
  no text.
- P3: in every workbook with a normal formula, the sheet holding it
  `changed`; in those of the nine with a chain, the chain `dropped`
  and the workbook's rels part and `[Content_Types].xml` `changed`,
  with `descriptor` alone allowed as `specs/0008` says; every other
  entry `identical` unflagged; a workbook with no normal formula
  `files_identical` `yes`. The run log prints, per target, how many
  workbooks had a formula replaced and how many chains were dropped,
  and the same cell comparison as P2 with `changed` 1 and `added` and
  `removed` 0. A departure from any expectation is a finding about
  the corpus or the rule, recorded, and planning rules which.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit, the
code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m1p4-<phase>/` (`gate.log`, `test.log`
on both targets, `negative.log`, `run.log`, `scan.log`, and `size.log`
in P3); every rule the earlier parts added, the scan log last with
the self-scan and the narrowed `f_home`, every sha256 read from a
file that exists, a commit message's counts printed by a log, at most
two independent checks; report; STOP. No push during the part; the
close-out push comes after P3 and carries the local commit b58e894.

Part 4's first doc commit carries, unedited: `CLAUDE.md` (the
maintainer's one-line fix of line 1, planning's close-out ruling 4),
`ROADMAP.md`, `specs/0010-reading-cells.md` (the close-out rulings)
and this spec.

### P1: the recipe in the crate; the rewrite

P1's first code commit: `Package::drop_calc_chain` and
`--drop-calc-chain` calling it, nothing else, with a test that the
tool's recipe and the method record the same edits on a package with
a chain and none on one without.

P1's second code commit: `Value`, `set_cell`, `clear_cell`, the
rewrite in `write_to`, and the seven messages. Tests on byte-built
worksheets in byte-built packages (the part 2 builder), the output's
edited part compared with bytes the test builds by hand (ruling 5):
replace a number cell, `s` kept, `t="n"` gone;
replace with text (the inline form; `&`, `<`, `>` escaped; leading
and trailing spaces kept; a tab and a line feed kept; a carriage
return as `&#13;`); replace with a boolean, both values; the number
forms `2.5`, `1`, `-0`, `0.1 + 0.2`, `1e21`, `1.5e-8`, `1e-7`
(`Display`), `f64::MAX` (`{:E}`), and NaN and both infinities
refused; insert before an existing cell, between two, and at the end
of a row; a new row between rows, before the first, after the last,
and into an empty `<sheetData/>`; several edits in one row and in
several rows in one write; the same cell set twice (the last wins);
set then clear; clear of an absent cell (the part byte-identical,
the entry still replaced, ruling 3); clear of a
styled cell (`<c r s/>`) and of an unstyled one (dropped, whitespace
kept); a cell with `cm` refused; a shared master, a follower, an
array master and a cell inside an array `ref` each refused, and a
cell outside the `ref` accepted; a normal formula replaced (the
`<f>` gone) on a package with a chain (the three removals applied,
the output reopened and read) and without (the sheet alone
changes); a
formula replaced when the caller already called `drop_calc_chain`
(applied once); a row without `r`, a cell without `r`, a descending
row, a descending cell and a repeated reference each refused; edits
to two sheets in one write; an edit to a part that is not a worksheet
refused by root; an edit to a missing part, to a removed part and to
a replaced part; every byte outside `<sheetData>` identical on a
worksheet with `<dimension>`, `<cols>`, `<mergeCells>` and
`<pageMargins>`; the strict main namespace; a text read back through
`Cells` as `InlineString` with the text written, a number through
`number()`, a boolean through `boolean()`; a refused write leaving no
temporary and no output. The negative log: the tool on the same
refusals through the built binary where an operation reaches them.
The corpus run `--drop-calc-chain` on both targets, recorded as
`records/m1p4-p1-<utc-date>-<shortsha>.md` and `.txt`; acceptance as
"Harness runs".

### P2: the sample edit over the corpus

P2's first code commit carries the P1 verification rulings: the
three `cells` messages in `write_to`'s doc with one write-path test
each (ruling 1); the dropped-once test counting the removals
recorded (ruling 4); the four refusal assertions printing the error
they refused on (ruling 8). Then `--edit-sample` with an
integration test on a fictional workbook (the tool's existing
fixtures), and the run log's `expat` cell comparison with a control
that reports a planted difference. The corpus run on both targets,
recorded as `records/m1p4-p2-<utc-date>-<shortsha>.md` and `.txt`;
acceptance as "Harness runs".

### P3: the formula sample, the probe, the size record

P3's first code commit carries the P2 verification rulings: the
one write-path `expect_err("refused")` site made to name what it
wrote (ruling 2 as P3 ruling 1 amends it); a `CellsAfter` refusal
removing the output and a reopen failure reported as the output's
(ruling 3). P3's logs carry ruling 4: a planted fault per comparison
flag, rows counted in input and output, both phase-commit controls
in every log, path fixtures assembled at run time. Then
`--edit-formula-sample` with its integration test; the catalogue and
Limits in `package`'s and `cells`'s module headers (the consequences
above; the refusals; inline strings only; no style on a new cell);
`probe-emend-cells-write-xml`, measured as "Size" says, recorded as
`records/size-m1p4-<utc-date>-<shortsha>.md`, `size.log` printing
every figure the record carries. The negative log through the built
binary: a workbook whose sheet lacks `r` on a row and one with a
cell inside an array formula, each through both `--edit-sample` and
the release `probe-emend-cells-write-xml` the size log builds (exit
2, the message; no output file from the tool, while the probe, a
size consumer with no temporary-and-rename step, leaves its outputs
and the log prints their presence and byte counts as measured, P3
ruling 8); the forbidden-text message, which
no binary can reach since every binary writes its own constant text,
named as unreached from the source beside the package test that
raises it (P3 ruling 7); and P1's refusals again. The corpus
run `--edit-formula-sample` on both targets with the release binaries
the size log hashed, recorded as `records/m1p4-p3-<utc-date>-
<shortsha>.md` and `.txt`. The report states part 4's own cost and
the budget figure against the ceiling.

### Close-out push (after P3)

As part 3's, with the close-out scripts as part 3 ran them, one
record commit after the push unless the maintainer chooses the split
again (`specs/0010` close-out ruling 2), the scan rerun last. The
push carries b58e894 and part 4's commits, on planning's word and the
maintainer's authorisation typed in the implementing session.

## Decisions for the maintainer

Decided by the maintainer 2026-09-22: each as recommended.

1. Text as inline strings (recommended, and what this draft says).
   The alternative appends each new text to the shared strings table
   and writes `t="s"` with its index: Excel's own form, but it
   rewrites a second part (up to 2.7 MB in the corpus) on every text
   edit, must decide whether to reuse an equal item, and leaves
   `count` and `uniqueCount` to update or ignore. Both forms open in
   LibreOffice and Excel; Excel converts inline strings to shared on
   its next save. The inline form can be joined later by a shared
   form behind a method name, with no change to this part's API.
2. Overwriting or clearing a normal-formula cell allowed, with the
   chain dropped in the same write (recommended). The alternative
   refuses every edit to a formula cell until part 6. Refusing makes
   `set_cell` fail on a cell the caller can see holds a formula,
   which is the commonest reason to overwrite one; and the recipe
   already exists. Part 6 then keeps writing `<f>`, the `calcPr`
   question and whatever the Excel check says about recalculation.
3. `<dimension>` left as it is (recommended). Updating it needs the
   sheet's extent before `<sheetData>` is reached, that is a second
   pass or a seek, for an element the specification calls an
   optimisation hint. LibreOffice ignores it; the milestone Excel
   check tells whether Excel tolerates a cell outside it, and if it
   does not, a later part adds the pass.

## Planning rulings during P1 (2026-09-22)

The implementing session asked, after the code commits 2e3bb2a and
aa87cbc and with an uncommitted `gate.log` at aa87cbc, how to treat a
claim in aa87cbc's message: seven choices "each pinned by a test", of
which planning confirmed five named tests exist in
`crates/emend/src/package/tests.rs` and two have none (a worksheet
with no `<sheetData>`; `t="dataTable"` as a removable formula). This
is finding P1-F1. Rulings:

1. The message stands as history; nothing is amended or rewritten.
   The fix is a THIRD P1 code commit, which departs from "P1" above
   (two code commits) and says so in the report. The rule in
   `specs/0001` for a count in a message is widened to a claim about
   a test: a message that says a test pins something names a test
   that exists at that commit.
2. A worksheet with no `<sheetData>` given a set is refused as
   `Error::Package("a worksheet without sheetData cannot be edited")`,
   an eighth message, not `Error::Xml`: the part is well-formed XML
   this layer read whole, and `Error::Xml` is documented as bytes it
   cannot read. A clear-only edit set on such a part stays a no-op,
   as the code already has it. "Errors" below counts eight.
3. A `dataTable` formula is refused as an array formula is, not
   removed: a data table's `<f>` carries a `ref` whose other cells
   hold its results, exactly the shape that makes an array range
   unsafe to edit into. Its `ref` is tracked as an array's is, and
   one that does not parse is refused the same way. The message is
   reworded to "a cell in a shared, array or data table formula
   cannot be edited" in the design, the catalogue and the tests that
   pin it. No corpus workbook has a data table (`specs/0010`); the
   change touches no corpus run.
4. The other five choices are accepted and the design is corrected
   where it said less: new elements take the qualified-name prefix of
   the element they go into; a `<c>` whose reference names another
   row than its `<row>` is refused as out of order (the reader lets
   the cell's reference win, `specs/0010`; an editor cannot, since
   the insertion order would be ambiguous); the unmodelled-attribute
   refusal applies to an EDITED cell, an unedited one being copied
   raw; an empty `<row/>` gaining a cell is opened up as an empty
   `<sheetData/>` is; a new row goes immediately before the next
   `<row>` start tag or `</sheetData>`, after whatever whitespace the
   input wrote there.
5. The third code commit holds rulings 2 and 3 with their tests (the
   two behaviours, and the reworded message everywhere it is pinned)
   and nothing else. The mutation block the implementing session
   proposed for `test.log`, one named mutation per pinned choice with
   its bite printed, is accepted under the existing rule that a bite
   claim is measured.
6. Logs after this ruling run with two tracked files modified in the
   working tree by planning and uncommitted: this spec and
   `specs/0001-bootstrap.md`. They cannot be committed in P1 (no doc
   commit follows a code commit) and go in P2's first doc commit. Each
   log whose status check would count them says so in its header,
   names both files with their sha256 at run time, and accounts for
   exactly those two lines and no other. `gate.log` is rerun at the
   third commit with a rerun header hashing the kept attempt at
   aa87cbc by command.

## Planning verification and rulings on P1 (2026-09-22)

Verified from primaries by planning, not from the report: the range
b58e894..57d2334 (8d865e9 doc, 2e3bb2a, aa87cbc, d7a7a7f code,
57d2334 record, nine files under `records/`), every message ASCII
with the Co-Authored-By line alone; every committed log, check answer
and record file at the sha256 the report and scan.log print; every
superseded attempt (eleven files) present in the implementing scratch
at the hash and byte count its successor's header prints; planning's
own gate at 57d2334 nine PASS; planning's own tests 382 passed, 2
ignored on each target; planning's own `privacy.sh --range
b58e894..57d2334` 5 commits, 381 blobs, 0 findings; the phase's
record pair compared by planning with part 2 P3's on every column
but the two timings: 54 summary rows equal, entry rows equal; the
diffs of the three code commits read: 2e3bb2a is the recipe's move
and the tool's call, aa87cbc the feature, d7a7a7f the eighth message,
`dataTable` joining the array arm and the reworded constant, nothing
else. The seven mutations bite their named tests with the clean
control passing. P1 is DONE at d7a7a7f, records 57d2334, local.

Findings: P1-F1 (ruled during P1, closed by d7a7a7f). Rulings on the
seven points and four smaller ones the report raised:

1. F-a. The rewrite raises three `cells` messages on the write path
   ("a cell reference is not of the form A1", "a row number is not a
   number", and for an edited cell "a formula has a type this layer
   does not know"). Right reuse; the design above now names them.
   P2's first code commit adds them to `write_to`'s doc and one
   package test each on the write path.
2. F-b. Edits are recorded in a `Vec` in call order and sorted into
   a map per part at write, the last recorded winning. Accepted;
   "32-bit and streaming" is corrected to say so. The `Vec` grows
   with every call, which the Limits state.
3. F-c. A clear of an absent cell rewrites the part to identical
   bytes and the entry is still replaced, so the harness may show it
   `identical` or `recompressed` as `specs/0008` allows for a
   replaced entry. Accepted for this part; a no-op detection is not
   required. "P1" is corrected: the part is byte-identical.
4. F-d. The dropped-once test asserts the final state. P2's first
   code commit adds the count: the three removals recorded once, by
   reading what was recorded, not the output alone.
5. F-e and F-f. "The output surveyed" and "read by the independent
   parser" were the draft's words; the tests reopen the output
   through the package and compare the part's bytes with bytes built
   by hand, which is stricter. Accepted; the words are corrected.
6. F-g. The independent checks were given one combined diff and could
   not confirm per-commit scope; planning did, from `git show` of
   each commit. From P2 on, a check directory holds one diff per
   code commit as well as the combined diff, and the answer says
   which it read for each "nothing else" (rule added to `specs/0001`
   "Rules for every phase").
7. An unedited array or data table master whose `ref` does not parse
   refuses the whole worksheet's edit, since its extent is unknown to
   every later edit. Accepted; the design and the Limits say so.
8. Smaller points: the four mutation failures that print only
   "refused: ()" are made to print the error in P2's first code
   commit (the assertion names the value it refused on); a mutation
   breaking a second test is fine; the header numbering and the
   disclaimed inherited prose are cosmetic and P2's logs are written
   fresh for this phase.
9. The uncommitted files (this spec, `specs/0001`, and `ROADMAP.md`
   from this verification) go in P2's first doc commit, unedited.

## Planning rulings during P2 (2026-09-22)

The implementing session asked, with 4b7f55c and 595cf8b committed
and no log written, about the acceptance clause "`changed` equal to
`set` minus 2, `removed` equal to `dropped`": a cleared cell that
carries `s` stays as `<c r s/>` (the design's clear rule), so the
comparison's tuple sees it changed, not removed, and the clause holds
only for an unstyled B. Nearly every corpus cell carries `s`
(2094287 of 2094601, `specs/0010`), so the clause as drafted would
fail in every workbook. This is finding P2-F1, planning's, in the
draft's arithmetic. Rulings:

1. The clause is corrected in "Harness runs": `changed` equal to
   `set` minus 2 plus `cleared` minus `dropped`; `removed` equal to
   `dropped`; `added` 2; the sum unchanged. The run log also asserts,
   from the comparison's own pick of A, B and M (not the tool's), that
   the changed references are exactly A and, when B carries `s`, B;
   that B then has no value in the output and keeps its `s` bytes;
   and that the added references are exactly `(M + 1, 1)` and
   `(M + 1, 2)`. No reference is printed.
2. The operation stays as specified (B is the second such cell,
   styled or not); the alternative of choosing an unstyled B would
   make the sample avoid the corpus's common case.
3. The uncommitted smoke run over the debug binary is an unlogged
   aside, as the report should call it; the run log is the measure.

## Planning verification and rulings on P2 (2026-09-22)

Verified from primaries by planning, not from the report: the range
57d2334..ed89a57 (4cf3671 doc carrying planning's three files at the
hashes planning sent, 4b7f55c and 595cf8b code, ed89a57 record, nine
files under `records/`), every message ASCII with the Co-Authored-By
line alone; every committed log, check answer and record file at the
sha256 the report and scan.log print; the three superseded attempts
present in the implementing scratch at their printed hashes and byte
counts; planning's own gate at ed89a57 nine PASS; planning's own
tests 390 passed, 2 ignored on each target; planning's own
`privacy.sh --range 57d2334..ed89a57` 4 commits, 385 blobs, 0
findings; the two code diffs read, 4b7f55c the three carried items
and 595cf8b the operation with its tests, nothing else. Planning's
own `expat` comparison over the run directory, written without
reading the run script, picking A, B and M itself from each input's
first worksheet in workbook order, agrees with the run log on all 27
workbooks: prefix and suffix bytes identical, the changed references
exactly A and the styled B, the added references exactly `(M + 1, 1)`
and `(M + 1, 2)`, A's value `42.5` with its `s` kept, B without a
value with its `s` kept, the text and the boolean in their forms,
every other entry equal in CRC and sizes, the host and i686 outputs
byte-identical, and the totals `set` 81, `cleared` 27, `dropped` 0,
`cells_before` 2084915, `cells_after` 2084969. P2 is DONE at
595cf8b, records ed89a57, local.

Findings: P2-F1 (planning's clause, ruled during P2). Rulings on
what the report left open:

1. F-1, F-3, F-6, and check 2's notes 8 and 9: accepted as they
   stand. R4 shows `write_to` reaches the rule; no corpus workbook
   has an unstyled second value cell (2094287 of 2094601 cells carry
   `s`), so the `dropped` arm is shown on generated input alone, and
   "Harness runs" says so; a parsed `s` equal to its bytes for a
   decimal is the same check.
2. F-2: the twenty remaining `expect_err("refused")` sites move to
   the helper that names what was not refused, in P3's first code
   commit. AMENDED by P3 ruling 1 (finding P3-F1): one site is a
   write and changes; nineteen are reads whose panic already names
   the value, and stand.
3. F-4 and F-5: a `CellsAfter` refusal removes the output file, as
   every other refusal of the tool leaves none, and a failure to
   reopen the output is reported as the output's, not the input's;
   both in P3's first code commit.
4. Check 2's findings 2, 3 and 11: P3's run log plants one fault per
   flag of its comparison, `new_cells` included; its comparison
   counts `<row>` elements in input and output so a row total is
   measured, not restated; every P3 log controls the phase commit
   with the null commit and a short sha. Check 1's note 14: a path
   control fixture is assembled at run time, as the scan's fixtures
   are, from P3 on.
5. 4b7f55c's message says only the edited cell's formula is read;
   every cell's formula tag is read and only an edited cell's unknown
   type refuses. The message stands as history; R1d measures the
   behaviour.
6. `descriptor` on the changed worksheet entry, as `specs/0008` allows
   for a replaced entry, was not named in P2's acceptance and is
   named now.
7. This spec and `ROADMAP.md` go in P3's first doc commit, unedited.

## Planning rulings during P3 (2026-09-22)

The implementing session asked, after the doc commit 20fd731 and
before any code commit, about P2 verification ruling 2: the P2
report, carrying independent check 1's wording, called the twenty
`expect_err("refused")` sites writes, and the ruling took that as its
premise. Planning confirmed from `crates/emend/src/package/tests.rs`
at 20fd731: twenty sites, one a `write_to` (the test that a refused
edit leaves no temporary and no output), nineteen reads of the
workbook, the content types or the relationships, whose `Ok` values
derive `Debug`, so their `expect_err` panic already prints what was
returned; "refused: ()" can only come from a write. This is finding
P3-F1, in the P2 report's description and in planning's ruling that
rested on it. Rulings:

1. Ruling 2 of the P2 verification is amended: the one write site
   replaces `expect_err` with a match whose `Ok` arm panics naming
   what the write wrote, its lock and its temporaries check kept as
   they are; the nineteen read sites stand. `test.log` prints the
   counts by grep: the sites, the writes among them, the reads.
2. The rest of P3's first code commit is as "P3" says: the
   `CellsAfter` refusal removing the output and the reopen failure
   reported as the output's.
3. This spec stays uncommitted through P3 and goes in the close-out's
   doc commit; each P3 log whose status check would count it names it
   with its sha256 at run time and accounts for its one line.

The implementing session then asked, with 18bfc36 committed, about
two gaps in `--edit-formula-sample` and noted a third; "Harness runs"
is corrected for all three:

4. With no normal formula there is no sheet to read back: the line
   keeps its shape and carries every key 0, the doc and the test
   saying a `cells_after` of 0 means no sheet was read.
5. Only sheets of kind `Worksheet` are read, in `<sheets>` order, as
   `--edit-sample` reads; `chain_dropped` is measured from the two
   reads (the input's workbook has a `calcChain` relationship, the
   reopened output's has none), not inferred from the edit.
6. The line gains `read_back`, 1 when the edited cell reads back as
   `number()` 1 with no formula, expected equal to `formula_replaced`
   and asserted per file by the run log, beside the comparison's
   `changed` 1; `cells_after` is asserted by the tool against the
   sheet's count before the edit, as `--edit-sample` does.

Asked again, with code committed through 4b31e2a and the gate run,
about the negative log's third case:

7. "A text that XML forbids" cannot be brought through any built
   binary, since every binary writes its own constant text and no
   workbook can carry a forbidden character into what is written:
   the draft's case was planning's error. The negative log runs the
   missing-`r` and array cases through both `--edit-sample` and the
   release probe, and names the forbidden-text message as unreached
   from the source beside the package test that raises it. No code
   changes; "P3" is corrected.

Asked again, with `size.log` running at 4b31e2a:

8. The probe is a size consumer, not a tool: it creates its second
   output before `write_to` and has no temporary-and-rename step, so
   a refusal leaves both outputs in place. "No output file" is
   asserted for the `--edit-sample` runs alone; for the probe the
   negative log asserts exit 2, the exact message and empty stdout,
   and prints both outputs' presence and byte counts as measured
   with the source lines that explain them. No code change, so the
   size record stands.

## Planning verification and rulings on P3 (2026-09-22)

Verified from primaries by planning, not from the report: the range
ed89a57..df83dab (20fd731 doc carrying planning's two files at the
hashes planning sent; 18bfc36, 858acf4, af5bba6, 4b31e2a code;
df83dab record, eleven files under `records/`), every message ASCII
with the Co-Authored-By line alone; every committed log, check
answer, record and size record at the sha256 the report and scan.log
print; the seven superseded attempts present in the implementing
scratch at their printed hashes and byte counts; planning's own gate
at df83dab nine PASS; planning's own tests 394 passed, 2 ignored on
each target; planning's own `privacy.sh --range ed89a57..df83dab` 6
commits, 400 blobs, 0 findings; the four code diffs read (the one
write site and `ReadOutput`; the formula sample as rulings 4 to 6
say; the Limits; the probe as "Size" says). Planning's own rebuild
of `probe-zip-xml`, `probe-emend-cells-xml` and
`probe-emend-cells-write-xml` in the `size` profile on i686 musl at
df83dab reproduced the record's section totals 533216, 807180 and
811280 and the probe's sha256 prefix 8591be14, so the budget figure
278064 and the own cost 4100 are planning's arithmetic too, as are
the `.text` deltas 211664 and 3488. Planning's own `expat`
comparison over the run directory, picking the first worksheet in
`<sheets>` order that holds a normal formula itself, agrees with the
run log on all 27 workbooks: 21 replaced and 6 with none; in the 21,
prefix and suffix bytes identical, exactly that cell changed to the
value `1` with its `s` kept and no formula, no cell added or removed,
the row count unchanged, the entries changed exactly the sheet plus,
in the 9 with a chain, the workbook's rels part and
`[Content_Types].xml` with the chain entry gone; in the 6,
byte-identical outputs and an all-zero line; host and i686 outputs
byte-identical. P3 is DONE at 4b31e2a, records df83dab, local.

Findings: P3-F1 (ruled during P3, planning's and the report's).
Rulings on the six items the report left open:

1. Path controls. Only `run.log` plants a path fixture; `size.log`'s
   and `gate.log`'s zero-path checks have no control, and `test.log`
   and `negative.log` check no paths. Accepted for P3: the privacy
   instrument is the scan, which passed with its controls, and
   planning's own scan agrees. The rule already in `specs/0001`, that
   a zero is meaningful only beside a control that can be nonzero,
   applies to every such check from the close-out on, the fixture
   assembled at run time.
2. The size reconciliation's identity checks and the label "parts 1
   to 3 measured at this commit" on a figure that holds part 4's
   drift (`size.log`, two lines): accepted as they stand and named
   here as wording faults; the record's own text says the drift is
   part 4's, and the ceiling comparison and planning's rebuild are
   the checks that can fail.
3. Planted faults not covering `removed` 0; the missing-`r` fixture
   holding cells without `r` in a numbered row (the reader's rule is
   the same for both); no F4x-style control for the formula sample's
   removal; a parse failure of the reopened output reported in the
   library's words: accepted as they stand, each a note in the record
   and none a defect in what was measured.
4. Size. The budget figure 278064 section bytes is accepted against
   the ceiling of 393216, leaving 115152. "Part 4's own cost" as
   "Size" defines it, 4100, understates part 4's spend: the whole
   growth since part 3's record is 30716, of which 26616 is the
   rewrite's cost to every consumer that already writes a package,
   measured as the drift of `probe-emend-cells-xml` with the six
   commits behind it all part 4's. "Size" records both figures, and
   from part 5 on a part's size record states its own cost and the
   budget figure's whole growth side by side.
5. This spec and `ROADMAP.md` go in the close-out's doc commit,
   unedited. The close-out runs as "Close-out push (after P3)" says,
   on planning's word given with this verification and the
   maintainer's authorisation typed in the implementing session.

## Out of scope

- Formulas, `calcPr` and recalculation on open: part 6.
- Styles on new cells, number formats, dates: part 5.
- Shared strings appended or edited; rich text (`<r>` runs) written.
- Inserting or deleting rows or columns, and the reference fix-up
  that would need (not done by design, as `CLAUDE.md` says).
- Editing a cell that belongs to a shared, array or data table
  formula, and rewriting the formula's other cells.
- `<dimension>`, `spans`, `<cols>`, merged cells, tables, data
  validation and conditional formatting: copied raw, never adjusted.
- Adding a worksheet, or writing a workbook from nothing.
