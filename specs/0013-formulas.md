# 0013: Milestone 1 part 6, formulas and the calculation chain

Status: APPROVED by the maintainer 2026-09-23 (planning draft the
same day), the four decisions under "Decisions for the maintainer"
each taken as planning recommended; sent to the implementing session
the same day.
Context: `CLAUDE.md` ("What emend is": emend does not evaluate
formulas, like openpyxl; "Two sessions, one repository"),
`ROADMAP.md` ("Milestone 1", part 6: the calculation chain removed
when formulas change, so Excel rebuilds it), `specs/0011-editing-
cells.md` ("Module `package`": `drop_calc_chain`, the formula-removal
rule, decision 2; the Limits: nothing is recomputed, the `calcPr`
question left to part 6), `specs/0012-styles.md` ("Module `package`",
"Size", "Harness runs"), `specs/0010-reading-cells.md` ("Module
`cells`": `Formula`, `FormulaKind`), `specs/0009-package-layer.md`
("Module `xml`", "Module `package`": the edits, `write_to`; P3's
removal recipe), `specs/0004-xlsx-diff.md` (statuses and flags),
`specs/0001-bootstrap.md` ("How this task runs", "Rules for every
phase"). ECMA-376 Part 1, 18.2.2 (`calcPr`), 18.2.27 (`workbook`'s
child order), 18.3.1.40 (`f`) and 18.6 (the calculation chain) are
cited where the design leans on them; no text is copied from them.

## Goal

A formula can be written into a cell, new or existing, and the
workbook handed back in a state a spreadsheet application computes
on open: the cell carries `<f>` with the text and no cached value,
the calculation chain is dropped whenever a write adds, replaces or
removes a formula, and the workbook's `calcPr` asks for a full
calculation on load, so a consumer that changes an input of existing
formulas can ask for the same. Nothing is evaluated: emend never
writes a cached value it did not read, by design, as openpyxl does
not. The comparison point is openpyxl's `cell.value = "=..."`
followed by `save`, which writes `<f>` without `<v>`, drops
`calcChain`, and writes `<calcPr fullCalcOnLoad="1"/>`.

The harness is the acceptance: over the corpus, a sample that writes
two formulas and replaces one leaves every other entry
byte-identical, the worksheet's cells identical except at the three
references, the workbook part identical except at its `calcPr`
element, the chain gone where there was one, as an independent
parser in the run script shows; every output opens in LibreOffice,
and LibreOffice computes the two written formulas to the values the
sample expects, read from a second conversion of each output; emend's
own reader reads the formulas and the `calcPr` flag back. Whether
desktop Excel recalculates a workbook so flagged is the milestone's
Excel check, on the machine `ROADMAP.md` lists as unscoped; this part
records that the check is pending and claims nothing about Excel.

## Facts planning checked (2026-09-23)

Over the 27 corpus workbooks, by element shapes and counts with
Python's `zipfile` and regular expressions, never a formula's text or
a cell's value:

- 9 workbooks have a `calcChain` part, each named by a `calcChain`
  relationship and an override in the content types; the 9 parts
  list 1659 cells. The other 18 have neither. This is the population
  `specs/0011`'s `chain_dropped` measured.
- Every workbook's `workbook.xml` has exactly one `calcPr`, as an
  empty-element tag, unprefixed, with double-quoted attributes: 10
  carry `calcId` alone (the Excel-produced files; one of them also
  has `concurrentCalc`, one `calcMode`, counted in the 10), and 17
  carry `iterate`, `iterateCount`, `iterateDelta` and `refMode` (the
  LibreOffice-produced files). None carries `fullCalcOnLoad`. The
  element follows `definedNames` in 16 workbooks and `sheets` in 11,
  and is followed by `extLst` in 22, `pivotCaches` in 4 and
  `webPublishing` in 1. No `workbook.xml` starts with a byte order
  mark; none binds the main namespace to a prefix.
- 262879 `<f>` elements: 262538 normal, 336 shared, 5 array, none
  `dataTable`; 447 carry `ca="1"`. Every formula cell has a `<v>`.
  Their `<c>` types: `n` 261128, absent 1488, `str` 238, `e` 25.
- The pinned LibreOffice image (`localhost/emend-lo:25.2.3-2-deb13u6`,
  `--convert-to xlsx`, the harness's route) recomputes every formula
  on load: on planning's own fictional four-cell workbooks, a formula
  cell with no `<v>`, one with a stale `<v>`, and each of those under
  `fullCalcOnLoad="1"`, with and without a `docProps/app.xml` naming
  Microsoft Excel as the application, all came back with the computed
  value. So the LibreOffice check proves a written formula is valid
  and computable, and says nothing about whether a cached value or
  the flag would be honoured by Excel.
- Part 5 left 88804 section bytes of headroom (budget 304412 against
  the ceiling 393216).

## Design

### Crate

No new crate, feature or dependency. Everything below is in the core:
a formula is a cell value the format carries in `<f>`, and the
`calcPr` edit is one attribute on one element of a part the package
already reads. Part 6 is expected to cost less than any earlier part
("Size").

### Module `cells`: the formula value

`Value` gains one variant:

- `Value::Formula(String)`: a normal formula, written as `<c r="..."
  s="..."><f>text</f></c>`: no `t` attribute, no `<v>`, no `ca`, no
  `ref`, no `si`. The text is what `Formula::text` reads back: the
  expression without a leading `=`, escaped for XML as `Text` is
  (`&`, `<`, `>`; a carriage return as `&#13;`), so a text with a
  quoted string holding `&` or `<` survives. A text that is empty,
  that starts with `=`, or that holds a character XML 1.0 forbids is
  refused at write, with no output ("Errors"). Nothing else about the
  text is checked: emend does not parse formulas, and a text Excel
  rejects is the caller's (the LibreOffice conversion in the sample
  is the harness's check that a well-formed text computes).

`Value::element` writes the variant beside the three others, with the
same `prefix` rule; `Value::check` refuses the three cases. A `Cell`
read back from a written formula has `formula` `Some` with `kind`
`Normal`, `reference` `None`, and no value (`Kind` as the reader
already classifies a `<c>` with `<f>` and no `<v>`; `specs/0010`).

`Formula` and `FormulaKind` do not change: shared, array and data
table formulas are read, never written, and a cell in one is still
refused as an edit target (`specs/0011`).

### Module `package`: the formula-change rule and `calcPr`

`Package::set_cell` takes the new variant as any other; nothing
changes in what it records. `write_to` applies:

- The formula-change rule. A write changes formulas when any edit of
  it writes a `Formula` value (to a cell that had none, or one whose
  `<f>` was normal), or replaces or clears a cell whose `<f>` was a
  normal formula (part 4's rule, unchanged). When a write changes
  formulas and the workbook has a `calcChain` relationship, the chain
  is dropped in the same write by `drop_calc_chain`'s recipe, once,
  unless the caller has already recorded it; and the write asks for a
  full calculation on load ("`calcPr`" below), unless the caller has
  already recorded that. A write that changes no formula, including a
  style edit alone on a formula cell and a `Formula` value recorded
  and then overridden by a `Number` for the same reference, leaves the
  chain and `calcPr` alone. `ca` attributes on other cells are not
  touched: they are read-time hints the chain's absence makes moot.
- `calcPr`. `Package::recalculate_on_open(&mut self)` records that the
  workbook part gets `fullCalcOnLoad="1"`: nothing is read, nothing
  can fail, recording it twice records it once. `write_to` then
  rewrites `workbook.xml` through `xml::Stream` as the styles
  containers are rewritten (`specs/0012`): every byte outside the one
  `calcPr` start tag is copied raw, including the byte order mark,
  the declaration, whitespace and `extLst`; the `calcPr` tag is
  rebuilt with its attributes in their order and their quotes,
  `fullCalcOnLoad` replaced by `"1"` where present, else appended
  last; an empty-element tag stays one. When the workbook has no
  `calcPr` (no corpus witness; ECMA-376 18.2.27 orders it after
  `definedNames`, `externalReferences`, `functionGroups` and
  `sheets`), `<calcPr fullCalcOnLoad="1"/>` is inserted immediately
  after the end of the last of those four that is present, prefixed
  as `<workbook>` is prefixed, and after `<sheets>` at least, since
  the workbook reader requires `<sheets>`. A second `calcPr` in the
  part is refused at write ("a workbook has two calcPr elements"),
  as a second `cellXfs` is. The rewrite happens after the worksheet
  rewrites and before the content types and relationships are
  written, in the same temporary-and-replace pattern; a workbook part
  the caller has replaced or removed refuses the edit as `specs/0012`
  refuses a style addition on a replaced styles part.
- `Workbook` gains `full_calc_on_load: bool`, read from `calcPr`'s
  attribute as `date1904` is read from `workbookPr` (`1` or `true`;
  absent, `0`, `false` and no `calcPr` give `false`), so a caller and
  the sample can read the flag back. `Workbook::calc_chain` already
  says whether a chain is named.

The caller's picture, to state in `package`'s Limits: writing a
formula makes the file compute on its next open in an application
that honours `fullCalcOnLoad` or that recalculates on load anyway;
no cached value exists for that cell until then, so a reader that
shows cached values (emend's own, openpyxl's `data_only`) shows
nothing there; the cached values of the other formulas are unchanged
and possibly stale, which is why the flag is set; a caller whose
edit changed an input of existing formulas without touching any
formula calls `recalculate_on_open` itself, since emend cannot know
the dependency graph without the chain it drops. Excel's honouring
of the flag is not measured by this part (the Excel check is the
milestone's). This is what openpyxl's comparison point does and no
more.

### Errors

New `Error::Package` messages, each with a catalogue entry, a test
and, where a binary can reach it, a negative-log case:

- "a formula text is empty"
- "a formula text starts with an equals sign"
- "a formula text holds a character XML cannot carry"
- "a workbook has two calcPr elements"
- "the workbook part cannot be edited in a package that replaces or
  removes it"

The refusals of `specs/0011` ("a cell in a shared, array or data
table formula cannot be edited", the unknown formula type, the
shared formula without an index) apply to a `Formula` value as to
any other. A refused write leaves no temporary and no output.

### 32-bit and streaming

Nothing new: the worksheet rewrite is part 4's; the workbook rewrite
is one pass over a part that is 1 KB to 30 KB in the corpus, through
`xml::Stream`, copying raw. `u64` for every offset as before.

### Size

`tools/size-probe` gains one probe, built in the `size` profile on
i686 musl as the others:

- `probe-emend-formulas-write-xml`: `probe-emend-styles-write-xml`'s
  work, then `set_cell` of a `Formula` from its command line on a
  third cell, the write, and after it the output's `Workbook::
  full_calc_on_load` and `calc_chain`, and the third cell's `formula`
  through `Package::cells`, so no path is dead. The budget figure is
  it over `probe-zip-xml` against the ceiling of 393216 section bytes;
  part 6's own cost is it over `probe-emend-styles-write-xml`; the
  record states beside it the budget figure's whole growth since part
  5's 304412, with the drift of the earlier probes classified as
  `specs/0011` P3 verification ruling 4 says.

Planning's expectation, not a measurement: under 6000 section bytes
for the part (one `Value` arm, one attribute rewrite, one flag read).
The size log re-measures the earlier probes against part 5's record
by the byte classifier.

### Harness runs

`tools/emend-copy` gains two operations, at most one per invocation,
given after the two paths as the others are; neither takes a cell
reference or a formula from its command line, so no record names a
position in a corpus workbook or carries a formula of one.

- `--calc` (P1): reads the workbook and prints one line, the word
  `calc` then tab-separated `key=value`: `chain` (1 when the workbook
  has a `calcChain` relationship, else 0), `full_calc` (1 when
  `full_calc_on_load`, else 0). Expected totals over the corpus:
  `chain` 9, `full_calc` 0, the output byte-identical to the input.
- `--formula-sample` (P2): the first sheet of kind `Worksheet` in
  workbook order is read once through `Package::cells` to find A, the
  first cell of kind `Number` with a value and no formula; F, the
  first cell with a formula of kind `Normal`; and M, the largest row
  number over the sheet's cells (0 with none). It records A set to
  `Number(21)`, `(M + 1, 1)` set to `Formula` of A's reference
  followed by `*2` (the text composed from `CellRef::a1`, so `A1*2`
  when A is A1), `(M + 1, 2)` set to `Formula("1+1")`, and F, when it
  exists, set to `Formula("2+3")`; writes the package; reopens the
  output and reads the same sheet and the workbook; and prints one
  line, `formula` then `added` (2, or 0 when A is absent: then
  nothing is recorded and the output is byte-identical), `replaced`
  (0 or 1), `chain_before` and `chain_after` (from the two workbook
  reads; `chain_after` expected 0), `full_calc_before` and
  `full_calc_after` (expected 0 and 1 when `added` is 2),
  `cells_before` and `cells_after` (expected `cells_before + 2`,
  asserted by the tool: exit 2 with a message when it differs),
  `read_back` (the number of the written formulas that read back as
  `kind` `Normal` with the text written and no value; expected
  `added + replaced`, asserted by the tool). A workbook whose first
  worksheet has no `Number` cell gets `added` 0 and every other key
  as read.

The runner and the record script are unchanged; each run names its
operation through `--tool-arg`. The P2 run log adds two checks over
every output, each with a control that reports a planted difference
per flag it raises (`specs/0012` P2):

- The comparison: with `ElementTree` and byte spans as part 5's
  `pcmp.py` did, the worksheet's cells identical except at the three
  references (two when F is absent), the new cells' `<f>` text equal
  to the text the tool composes from the sheet's A, `workbook.xml`
  byte-identical outside the `calcPr` start tag, and that tag equal
  to the input's with `fullCalcOnLoad="1"` appended; the `calcChain`
  entry, its override and its relationship absent from the output
  exactly when `chain_before` is 1.
- The computation: each output converted once more through the
  pinned image with `--convert-to xlsx` into the scratch directory,
  the converted worksheet's `<v>` read at `(M + 1, 1)` and `(M + 1,
  2)`, expected `42` and `2`, and at F expected `5`; the converted
  files are deleted after the read and never enter the repository.
  This is the harness's proof that the written text computes; the
  log states, from the fact above, that this image recomputes on load
  regardless of the flag, so the check does not measure the flag.

Expected results, `lo` and `lo_in` `ok` in every case: every entry
outside the first worksheet, `workbook.xml`, the content types, the
workbook relationships and the chain `identical`; the first
worksheet and `workbook.xml` `changed` with the comparison's checks
holding; the content types and workbook relationships `changed` on
the 9 workbooks with a chain and `identical` on the other 18; the
chain entry `removed` on the 9; totals `added` 2 on every workbook
with a `Number` cell in its first worksheet, `replaced` 1 on every
workbook whose first worksheet has a normal formula, `chain_before`
9 and `chain_after` 0, `full_calc_before` 0 and `full_calc_after`
equal to the count of workbooks with `added` 2. The report states
the totals against these; a workbook where the comparison or the
computation differs is a finding.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m1p6-<phase>/` (`gate.log`, `test.log`
on both targets, `negative.log`, `run.log`, `scan.log`, and `size.log`
in P3); every rule the earlier parts added, the scan log last with
the self-scan and the narrowed `f_home`, every sha256 read from a
file that exists, a commit message's counts printed by a log and its
test claims existing at that commit, an independent check's
directory holding one diff per code commit, at most two independent
checks; scripts carry `#!/usr/bin/env bash` and their log prints the
invocation; the shape greps run over a script before its log does;
the check's hash label reads "the hash planning's handoff gave";
report; STOP. No push during the part; the close-out push comes
after P3 and carries the local commit b0c21ef.

Part 6's first doc commit carries, unedited: `ROADMAP.md`,
`specs/0012-styles.md` (the close-out rulings) and this spec.

### P1: the formula value, the rule, `calcPr`

P1's first code commit: `Value::Formula`, its element and its three
refusals; the formula-change rule for a written formula; `Package::
recalculate_on_open`, the workbook rewrite and `Workbook::
full_calc_on_load`; `--calc`; the catalogue entries. Tests on
byte-built packages, the output's parts compared with bytes the test
builds by hand (`specs/0011` P1 ruling 5): a formula written to an
absent cell in an existing row and in a new row; to an empty styled
cell (the `s` kept, the `<f>` after `r` and `s`); over a number, an
inline string, a shared string and a boolean (the `t` and `<v>`
gone); over a normal formula with `<v>` and with `ca="1"` (both
gone); to a cell in a shared group, an array `ref` and a `dataTable`
(each refused); a text with `&`, `<`, `>`, a quoted string, a
carriage return and a tab (escaped and read back equal); an empty
text, a text starting with `=` and a text with U+0001 (refused, no
output); a written formula overridden by a `Number` for the same
reference (the chain and `calcPr` untouched); a `Formula` and a style
edit on one cell (both apply); on a workbook with a chain: one
written formula drops it and sets the flag; one removed formula sets
the flag (part 4's test extended); `drop_calc_chain` then a written
formula (the recipe once); `recalculate_on_open` then a written
formula (the attribute once); on a workbook without a chain: the flag
set and nothing else changed in the package; `calcPr` with `calcId`
alone, with the four LibreOffice attributes, with `fullCalcOnLoad=
"0"`, with `fullCalcOnLoad='1'` single-quoted (kept), as a start tag
with an end tag (kept), absent after `definedNames`, absent after
`sheets` alone, absent in a prefixed workbook (`x:calcPr`), and
present twice (refused); every byte outside the tag identical on a
workbook with a byte order mark, `workbookPr`, `bookViews`,
`definedNames`, `pivotCaches` and `extLst`; `full_calc_on_load` read
as `1`, `true`, `0`, `false`, absent and no `calcPr`;
`recalculate_on_open` alone on a package that replaces or removes
the workbook part (refused at write); the tool's line on a fictional
workbook. The negative log: the tool on the refusals it can reach
through the built binary. The corpus run `--calc` on both targets,
recorded as `records/m1p6-p1-<utc-date>-<shortsha>.md` and `.txt`;
acceptance as "Harness runs".

### P2: the formula sample over the corpus

P2's first code commit carries the P1 verification rulings. Then
`--formula-sample` with an integration test on a fictional workbook
(the tool's existing fixtures) covering a sheet with A and F, with A
and no F, and with neither; and the run log's two checks with their
controls. The corpus run on both targets, recorded as
`records/m1p6-p2-<utc-date>-<shortsha>.md` and `.txt`; acceptance as
"Harness runs".

### P3: the catalogue and Limits, the probe, the size record

P3's first code commit carries the P2 verification rulings. Then the
catalogue and Limits in `package`'s and `cells`'s module headers (the
caller's picture above; the refusals; formulas written, never
evaluated; the Excel check pending); `probe-emend-formulas-write-xml`
measured as "Size" says, recorded as `records/size-m1p6-<utc-date>-
<shortsha>.md`, `size.log` printing every figure the record carries.
The negative log through the built binary: a workbook whose
`workbook.xml` has two `calcPr` elements through `--formula-sample`
and the release probe (exit 2 and the message); the messages no
binary can reach named as unreached from the source beside the test
that raises each. The corpus run `--formula-sample` again on both
targets with the release binaries the size log hashed, recorded as
`records/m1p6-p3-<utc-date>-<shortsha>.md` and `.txt`. The report
states part 6's own cost, the whole growth and the budget figure
against the ceiling.

### Close-out push (after P3)

As part 5's, with the close-out scripts as part 5 ran them, one
record commit after the push unless the maintainer chooses the split
again, the scan rerun last, the `ANTHROPIC_API_KEY` check. The push
carries b0c21ef and part 6's commits, on planning's word and the
maintainer's authorisation typed in the implementing session.

## Decisions for the maintainer

1. No cached value with a written formula (recommended). The
   alternative is a `Formula { text, cached: Option<Value> }` that
   writes `<v>` beside `<f>`, for a caller that computed the value
   itself. It would put emend in the position of writing a value it
   cannot check against the text, which is the one thing `CLAUDE.md`
   keeps emend out of; openpyxl writes none either. A later part can
   add it behind a second variant without changing this one.
2. `fullCalcOnLoad="1"` set by the formula-change rule AND offered
   as `recalculate_on_open` (recommended). The alternatives: the rule
   alone, which leaves a caller who changed a formula's inputs with
   stale cached values and no way to ask for a recalculation short
   of editing the workbook part by hand; or never touching `calcPr`,
   which is what emend does today and what the Limits of part 4
   called this part's question. openpyxl sets the flag on every
   save. Excel's honouring of it is unmeasured until the desktop
   machine, so the flag is a request, recorded as such.
3. A formula text starting with `=` refused (recommended), not
   stripped. `Formula::text` reads without the sign, so a caller that
   round-trips a read formula never sees one; a caller that types
   `=SUM(...)` gets a message at write, once, rather than a silent
   edit of the text. Stripping would make `=` and `==` write the
   same cell.
4. Acceptance without Excel (recommended: accept). This part's proof
   that a written formula computes is the pinned LibreOffice image,
   which recomputes on load regardless of the flag; Excel's behaviour
   on the flag and on a missing cached value is the milestone's
   Excel check, pending the machine. The alternative holds part 6
   until the machine exists; planning sees no gain, since the
   part's design is openpyxl's, which Excel handles, and the Excel
   check covers every part at once.

## Planning rulings during P1 (2026-09-23)

The doc commit 0de2507 carries the three files at the hashes
planning sent. The implementing session asked four questions before
code; planning checked each against the source (`package.rs`'s
`parse_workbook_sheets`, `cells.rs`'s `Value::element`, quick-xml
0.42.0's `escape.rs`) and rules:

1. The insertion point for an absent `calcPr` is corrected. "Module
   `package`" said the workbook reader requires `<sheets>`; it does
   not (`parse_workbook_sheets` returns no sheets for a workbook
   without them, and nothing refuses one), so the sentence "and
   after `<sheets>` at least" is withdrawn. The rule is: the new
   `<calcPr fullCalcOnLoad="1"/>` goes immediately after the end of
   the last root child present among the nine ECMA-376 18.2.27
   orders before `calcPr` (`fileVersion`, `fileSharing`,
   `workbookPr`, `workbookProtection`, `bookViews`, `sheets`,
   `functionGroups`, `externalReferences`, `definedNames`), and with
   none of them present immediately after the root start tag.
   "Immediately after" means before any whitespace or comment that
   follows that end, held back until the next root child shows it is
   not one of the nine. On every workbook with `<sheets>`, which is
   every corpus workbook, this is the rule the spec stated. Tests:
   absent after `definedNames`, after `sheets` alone, after
   `workbookPr` alone (no `sheets`), and in a root with no children.
2. A formula text is escaped with `quick_xml::escape::partial_escape`
   (`&`, `<`, `>`, and a carriage return as `&#13;`): the
   parenthetical governs, and "as `Text` is" is withdrawn, since
   `Text` uses `escape`, which also writes `&quot;` and `&apos;`. A
   formula's quoted strings stay literal, which is openpyxl's form;
   a reader unescapes both forms to the same text. The test with a
   quoted string asserts the quotes are literal in the output bytes.
3. Two `calcPr` elements are counted during the workbook read that
   reads `date1904`, in the main namespace under either spelling,
   kept privately in `Package`; `Package::workbook` does not refuse,
   and `full_calc_on_load` reads the first as `date1904` reads the
   first `workbookPr`. The write refuses in validation whenever it
   will rewrite the workbook part (the flag recorded, or the rule
   firing), before any temporary exists, and the rewrite refuses
   again as a backstop. Accepted as proposed.
4. The rule's reading is confirmed: a write changes formulas when
   the last value edit recorded for some reference is a `Formula`,
   or the worksheet rewrite removed a normal formula (part 4's
   flag). The flag is set whether or not a chain exists. The chain is
   dropped once, the flag recorded, and the whole set validated
   again, which is where the two workbook messages are raised for
   the rule's case. Part 4's formula-removal tests and the tool's
   `--edit-formula-sample` test now expect a changed workbook part;
   their expected bytes are updated in P1's first code commit, the
   commit message and the report naming each test so changed. The
   records of part 4's runs are history and are not rerun.
5. The defaults stated are accepted: a present `fullCalcOnLoad`
   keeps its quote character and gets the value `1`, so a present
   `'1'` or `"1"` leaves the tag byte-identical; a new attribute goes
   right after the last attribute's closing quote or the element
   name, so whitespace before `/>` or `>` is kept; a write with the
   flag recorded always rewrites the workbook part, even one already
   saying `1` (only a fixture reaches this: no corpus workbook sets
   the flag); the workbook rewrite runs after the styles rewrite and
   before the content types and relationships; `--calc` mirrors
   `--styles`, one line `calc` then `chain=N` and `full_calc=N`,
   tab-separated, the output byte-identical.
6. This section goes in a second doc commit before any code, as
   part 5's cbc691b did, carrying this spec alone, unedited, at the
   hash planning's message gives.

## Planning verification and rulings on P1 (2026-09-23)

Verified from primaries by planning, not from the report: the six
commits b0c21ef..6d85828 ASCII with the Co-Authored-By line alone
and no session link; the second doc commit 81991c3 carrying this
spec at the hash planning sent; the record commit 6d85828 touching
exactly the seven logs and the record pair at the sha256 and byte
counts the report prints; planning's own gate (nine PASS), `cargo
test --workspace` on both targets (489 passed, 0 failed, 2 ignored on
each, the ignored two the zip64 tests), the crate with the `dates`
feature off and on on both targets, and clippy on the crate, all at
6d85828; planning's own `scripts/privacy.sh --range b0c21ef..HEAD`
reading 6 commits, 455 blobs, 0 findings, PASS; planning's own
`--calc` run with a tool built in its own target directory over the
27 corpus workbooks agreeing with run.log's line for every workbook
and summing to `chain` 9 and `full_calc` 0, every output
byte-identical to its input; C1's `Value::Formula`, the rule, the
workbook rewrite, `full_calc_tag` and `parse_workbook_sheets` read
in full against "Module `cells`", "Module `package`" and rulings 1
to 5; C2 and C3 read in full; check 2's pinned inputs found in the
implementing scratch at the header's hashes, `specs/0001` at da8794b
and the combined code diff reproduced by planning's own `git`; the
superseded attempts the headers name found at their hashes; K9
reaching the replaced-workbook message; the about line on line 3 of
every log.

Findings:

- F1 (planning, low): `rewrite_workbook` holds a root child outside
  the main namespace that follows the last of the nine and writes
  the inserted `calcPr` before it. Ruling 1 spoke of whitespace and
  comments; a foreign child is treated the same way, which keeps the
  new element immediately after the last of the nine. Consistent
  with the ruling's intent; no corpus workbook lacks `calcPr`; no
  change, and the module comment names the case in P3.
- F2 (planning, low): `cells.rs`'s comment on `Value::Text` says
  quick-xml's `escape` leaves a carriage return as it is; in 0.42.0
  `escape` writes `&#13;` itself, so the `replace` after it is a
  no-op kept for safety. A stale comment from part 4, not P1's; it
  is corrected in P2's first code commit.

Rulings:

1. The three code commits are accepted as P1's: C1 the feature, C2
   the validation-order assertion that mutation M5 needed, C3 the
   chained fixture for the `recalculate_on_open` case and the
   comment. The five tests changed under ruling 4 are named in C1's
   message and the report.
2. G7: a check answer's header may type the commit ids it read; its
   binding part is the sha256 list of the inputs, which planning
   reproduces. Accepted practice from part 5 P3 on. From P2 the
   report or the check header states the command that produced each
   per-commit diff in the check directory, since planning's plain
   `git show` did not reproduce those four hashes while it did the
   combined diff and `specs/0001`.
3. G5: strengthened in P2's first code commit: after `write_to` on
   the chained fixture, the test asserts the three removals appear
   once each in the output (no duplicate removal recorded), beside
   the direct call it already checks.
4. G6: ruling 3's phrase "before any temporary exists" holds for a
   refusal validation can see before any rewrite (the flag recorded
   or a formula written); when the rule fires on a removed formula,
   found only in the worksheet rewrite, the refusal comes at the
   rule's re-validation with every temporary removed and no output,
   as K8 shows. Both are the design; the Limits state the second in
   P3.
5. G9: "Two new messages" in C1's message means the two `package`
   constants, as test.log reads it; five messages in all are P1's.
   History is not amended.
6. G8: from P2, a log that names its build directory names it once,
   as `<scratch>/<phase>/target`, the same in header and body.
   negative.log's two spellings are a prose defect, not a
   measurement.
7. The phase scan's single-trailer form is right for a range that
   carries one Co-Authored-By line, with the other line as a refusal
   control; the close-out's two-line form governs the close-out
   range.
8. C3's message quoting the spec's case without its backticks, and
   test.log's comparison with the lines joined and the backticks
   stripped, are accepted.
9. Superseded attempts stay outside the repository, named by file
   and hash in their successors' headers, as recorded. The four
   stale run directories under `<corpus>/runs` from superseded
   attempts are removed by the implementing session before P2's
   run, each name and its removal printed by P2's run.log header.
10. F11 and G4 are disclosure, accepted: the superseded first test
    attempt is the only record that M5 did not bite at C1, and the
    committed M5 block shows the converse at C3.
11. The Limits prose stays in P3, as the spec places it.
12. P1 is DONE. P2 (`--formula-sample`) is next: its first code
    commit carries this section and `ROADMAP.md` unedited, then
    ruling 3's assertion and finding F2's comment, then the sample
    as "### P2" and "Harness runs" say.

## Out of scope

- Writing shared, array or data table formulas, or extending a
  shared group; editing a cell inside one (refused, as part 4).
- Evaluating anything: a written formula has no cached value, and
  no dependant's cached value is recomputed.
- Rebuilding or editing the calculation chain (it is dropped and
  Excel rebuilds it), and `ca` attributes on other cells.
- `calcMode`, `iterate` and the other `calcPr` attributes, read or
  written; defined names; external references.
- Reference fix-up on row or column insertion (not done by design,
  `CLAUDE.md`).
- The names-only XML comparison: part 7.
