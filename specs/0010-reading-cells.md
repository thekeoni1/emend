# 0010: Milestone 1 part 3, reading cells

Status: APPROVED by the maintainer 2026-09-18; sent to the implementing
session the same day.
Context: `CLAUDE.md` (what emend does not do: no evaluation, no
rendering), `ROADMAP.md` ("Milestone 1", part 3), `specs/0009-package-
layer.md` ("Module `xml`", "Module `package`", "Size", the close-out
rulings), `specs/0008-zip-layer.md` ("Size" with the maintainer's
budget ruling and the part 2 checkpoint), `specs/0001-bootstrap.md`
("How this task runs", "Rules for every phase").

## Goal

The core's third layer: read what a worksheet says without changing
anything. A caller opens a package, takes the workbook's shared
strings, and streams the cells of any sheet: each cell's reference,
style index, type, the value as written (a number's text, a shared
string's index, a cached formula result, a boolean, an error), and its
formula when it has one. Shared string indexes resolve to text.
Nothing is evaluated, nothing is converted to a date, nothing is
written: the output of every harness run is byte-identical to its
input. The comparison point is openpyxl's reader in `read_only` mode,
which likewise stores formula strings and reads cached values.

The harness is the acceptance: over the corpus the reader's counts of
sheets, cells, cells by type, values, formulas by kind and shared
string items equal counts planning made with another reader before
this spec was written, and every shared string index in every cell
resolves.

## Facts planning checked (2026-09-18)

By element and attribute shapes and counts over the 27 corpus
workbooks (26 `.xlsx`, one `.xltx`), with Python's `zipfile` and
regular expressions, never the cells' text:

- 95 worksheet parts, no chartsheet or dialogsheet part, 27 shared
  strings parts (one per workbook), `date1904` set in none. The
  largest worksheet part is 63745347 bytes, the largest shared strings
  part 2748976 bytes. No worksheet part starts with a byte order mark;
  all 95 declare `standalone="yes"`; 50 follow the declaration with
  CR LF and 45 with LF.
- 2094601 `<c>` elements, every one with `r`; 2094287 with `s`;
  2086368 with `t`, whose values are `n` 1302755, `s` 783275, `str`
  273, `b` 40, `e` 25, and never `inlineStr` or `d`. The 8233 cells
  without `t` are numbers. So one producer in the corpus writes
  `t="n"` explicitly on every number cell, which the reader treats as
  the absent form. The only attributes seen on `<c>` are `r`, `s` and
  `t`. 5410 cells are empty elements (`<c .../>`), 2089071 hold `<v>`
  with text and 67 hold `<v/>`. No `<is>` anywhere.
- 262879 `<f>` elements: 292 are empty (`<f t="shared" si="..."/>`,
  the followers of a shared formula), 44 carry `t="shared"` with `ref`
  (the masters), 5 carry `t="array"`, and the other attributes seen
  are `ca` (302) and `aca` (261240). 382 formula texts hold a
  character reference (`&amp;` and the like); the longest is 511
  bytes. The longest `<v>` text is 192 bytes. Type and formula
  combine as: `s` never with `<f>`; `n` 1041627 without and 261128
  with; `str` 254 with and 19 without; `e` 25 all with; `b` 40
  without.
- 261979 `<row>` elements, every one with `r`; the attributes seen are
  `r`, `customFormat`, `ht`, `hidden`, `customHeight`, `outlineLevel`,
  `collapsed`, `spans`, `x14ac:dyDescent`, `thickBot`, `thickTop` and
  `s`. Every worksheet has `<dimension>`; none has an empty
  `<sheetData/>`.
- Shared strings: 110582 `<si>`, 110553 beginning `<si><t`, 29
  beginning `<si><r>` (113 `<r>` runs in all, in 8 workbooks), 110666
  `<t>` elements of which 110010 carry `xml:space="preserve"` and 656
  nothing, 52 character references, no `<rPh>` and no
  `<phoneticPr>`; every part carries `count` and `uniqueCount`.
- The worksheet root binds the main namespace as the default and `r`
  in all 95, and `mc`, `xr2` in all 95; 50 carry `mc:Ignorable` with
  `x14ac`, `xr`, `xr3` and `xr:uid`.
- 160 `<definedName>` elements in 16 workbooks: out of scope here.

Identifiers this spec names were grepped in the pinned crates'
sources when written: `quick_xml::escape::unescape`,
`quick_xml::events::Event::{Start, End, Empty, Text, GeneralRef}`,
`BytesStart::attributes`, `Attribute::unescape_value`. The `xml`
module already uses the first three.

## Design

### Crate

No new dependency. `crates/emend` gains the module `cells`; `lib.rs`
re-exports `cells::{Cell, CellRef, Cells, Formula, FormulaKind, Kind,
SharedStrings}`. The `package` module gains `Package::shared_strings`
and `Package::cells`. Every offset and count is `u64` where it is a
byte position and `u32` where the format says so (row and column
numbers, style and string indexes).

### Module `cells`

`CellRef { row: u32, col: u32 }`, both 1-based. `CellRef::parse(&[u8])
-> Option<CellRef>` accepts one or more ASCII capital letters followed
by one or more ASCII digits with no leading zero, nothing else, and
returns `None` for any other bytes or for a value that does not fit
`u32`. `CellRef::a1(&self) -> String` writes it back. No sheet-size
limit is enforced: the limit is Excel's, not the format's, and a
consumer that needs it checks `row` and `col`.

`Kind`: `Number` (`t` absent or `n`), `SharedString` (`s`),
`InlineString` (`inlineStr`), `FormulaString` (`str`), `Boolean` (`b`),
`Error` (`e`), `Date` (`d`). Any other `t` is refused with
`Error::Package("a cell has a type this layer does not know")`; a
reader that guessed would report a wrong value as a right one.

`Cell { reference: CellRef, style: Option<u32>, kind: Kind, value:
Option<String>, formula: Option<Formula> }`. `value` is the text of
`<v>` unescaped (`Some("")` for `<v/>`), or for `InlineString` the
concatenation of the `<t>` texts under `<is>` with `<rPh>` subtrees
skipped, and `None` when the cell has neither. Nothing is trimmed:
the format's `xml:space` says preserve and the reader never trims
anyway. A `Number` cell's value is the number's text as written, so
`1.10` stays `1.10`; `number()` below parses it on request.

`Formula { text: String, kind: FormulaKind, reference: Option<String>
}`. `text` is the formula unescaped, empty for a shared follower.
`FormulaKind`: `Normal` (`t` absent or `normal`), `Shared { index:
u32, master: bool }` (`t="shared"`; `master` is whether `ref` is
present; `si` absent is refused with "a shared formula has no
index"), `Array` (`t="array"`), `DataTable` (`t="dataTable"`; no
corpus witness, accepted by name only, its attributes not modelled).
Any other `t` is refused with "a formula has a type this layer does
not know". `reference` is `ref` as written when present. `ca`, `aca`
and every other attribute of `<f>` are not modelled: part 4's rewrite
copies them raw.

Typed views, each `Option` and never an error: `Cell::number() ->
Option<f64>` parses `value` with `str::parse::<f64>` when `kind` is
`Number` and the value is present, else `None`; `Cell::boolean() ->
Option<bool>` gives `Some(true)` for `1` and `Some(false)` for `0`
when `kind` is `Boolean`, else `None`; `Cell::error() -> Option<&str>`
gives the value when `kind` is `Error`. Dates are not converted:
`Kind::Date` carries its ISO text, and a number with a date format is
a number here until part 5 and the feature the dependency rule
requires.

`SharedStrings { items: Vec<String> }`, with `len()`, `get(u32) ->
Option<&str>`, and `resolve(&self, cell: &Cell) -> Result<Option<&str>,
Error>`: for `Kind::SharedString` the value parsed as `u32` (else "a
shared string index is not a number") looked up (else "a shared string
index is out of range"), for every other kind the cell's own value.
Each `<si>` becomes the concatenation of every `<t>` text under it, the
runs' `<t>` included and `<rPh>` subtrees skipped, unescaped, not
trimmed. `count` and `uniqueCount` are ignored: the items are what is
there. A workbook with no shared strings relationship gives an empty
`SharedStrings`, so a cell of kind `SharedString` in such a workbook
is out of range rather than a special case; a relationship whose
target names no entry is refused with "no such part" (P1 ruling 5).

`Cells<'p, R>`: a streaming reader over one worksheet part, an
`Iterator<Item = Result<Cell, Error>>` built on `xml::Stream` and
`xml::Scope`. It reads forwards once, holds one cell at a time, and
never seeks. Elements are resolved through the scope and matched only
in the main namespace (`package::MAIN`, both spellings); an element in
any other namespace, and any element this layer does not know, is
skipped with its subtree, so `extLst`, conditional formatting, data
validation and the rest cost nothing. `<row>` without `r` is the
previous row plus one (1 for the first); `<c>` without `r` is the
previous cell's column plus one within its row (1 for the first); an
`r` on `<c>` that does not parse is refused with "a cell reference is
not of the form A1", an `r` on `<row>` that is not a decimal `u32`
with "a row number is not a number", and an `s` that is not a decimal
`u32` with "a cell's style index is not a number". A `<c>` whose row
disagrees with the enclosing `<row>` is not refused: the cell's own
reference wins, since that is what every reader does and the row's
attributes are not modelled here. Text inside `<v>`, `<f>` and `<t>`
is unescaped through the crate; a general reference the crate does
not know (an unknown entity) is refused as
`Error::Package("a cell holds a reference this layer cannot expand")`,
since a value with a hole in it is not the value. Non-UTF-8 is
`Error::Xml(index)` as in every part. The part's root must be
`worksheet` in the main namespace, else "the worksheet part's root is
not worksheet"; the shared strings part's root `sst`, else "the shared
strings part's root is not sst".

### Module `package`

`Package::shared_strings(&mut self) -> Result<SharedStrings, Error>`
parses the workbook's `sharedStrings` part on every call and returns
it OWNED, unlike `content_types()` and `workbook()`, which cache and
lend: a caller streams cells through a mutable borrow of the package,
so a lent table could not be used while reading, and cloning it would
cost a copy of every string. `Package::cells(&mut self, part: &[u8])
-> Result<Cells<'_, R>, Error>` opens the named entry (else "no such
part", the existing message) through `read_part` and `Stream`. Neither
records an edit; a package read this way and written writes a copy.

### Errors

The eleven new `Error::Package` messages above join the catalogue in
`package`'s module header, each with the test that pins it, and the
set-equality test carried from part 2 (P3 verification ruling 2) reads
`cells.rs` as well, so the catalogue cannot fall behind the three
modules.

### 32-bit and streaming

Cells stream: the bound is one token, as `xml` says, plus one `Cell`.
A shared strings table is held whole, one `String` per item plus the
`Vec`; the corpus's largest part is 2748976 bytes of XML, its largest
table 54472 items, and its 27 tables hold 110582 items between them
(P1 ruling 10 unwelded this sentence); a consumer on this target that
meets a table larger than its address space cannot read that workbook,
which the Limits section of the module header says. No count is
`usize` in the API.

### Size

`tools/size-probe` gains `probe-emend-cells-xml`, built in the `size`
profile on i686 musl as the others: `probe-emend-package-xml`'s work,
then the shared strings, then every cell of the sheet named on its
command line, printing the cell count, the sum of `number()` over the
cells and the total length of `resolve`d text, so no path is dead.
The budget is judged on it over `probe-zip-xml`, with the file delta
beside it; part 3's own cost is it over `probe-emend-package-xml`.
The size log also builds, in the scratch way part 1 P3 built its sort
candidates, one variant whose sum is a byte count of the values
instead of `number()`, so the float parser's own cost is a figure in
the record rather than a guess. The size script excludes the workspace
manifest and lock file from a probe's cause paths (part 2 P3
verification ruling 3).

The maintainer's answer to the part 2 checkpoint (`specs/0008`
"Size": a higher ceiling, 384 KiB named, or Cargo features off by
default for styles, dates and the like) is recorded here when given:

    Checkpoint ruling (2026-09-18, given to planning): the higher
    ceiling. The core's budget is 393216 section bytes (384 KiB)
    over `probe-zip-xml`; parts 1 and 2 spent 183980; 209236 remain
    for parts 3 to 8. No feature is added on this ground.

Part 3 adds no feature and nothing heavier than the two crates, so the
answer changes no code in this part; it sets the ceiling the part 3
size record is judged against.

### Harness runs

`tools/emend-copy` gains two operations, at most one per invocation,
given after the two paths as the others are; neither edits, so every
output is byte-identical. `--shared-strings` (P1) prints one line, the
word `strings` then tab-separated `key=value`: `items` (the table's
length) and `bytes` (the sum of the items' UTF-8 lengths). `--cells`
(P2) streams every sheet of the workbook in `<sheets>` order and
prints one line, the word `cells` then tab-separated `key=value`
counts: `sheets`; `cells`; `number`, `shared`, `inline`,
`formula_string`, `boolean`, `error`, `date` (cells by kind);
`values` (cells with a value); `empty` (neither value nor formula);
`formulas`, `shared_master`, `shared_follower`, `array`, `data_table`;
`numbers_parsed` (`Number` cells with a value where `number()` is
`Some`); `booleans_parsed`; `resolved` and `unresolved`
(`SharedString` cells whose `resolve` succeeds and fails, the run
continuing past a failure so the count is complete); `max_row`,
`max_col`; `styled` (cells with `s`). An error from any parse other
than `resolve` is the tool's exit 2 with the message. The runner and
the record script are unchanged; each run names its operation
through `--tool-arg`.

Expected results, every entry `identical` unflagged, `files_identical`
`yes`, diff exit 0, `lo` and `lo_in` `ok` in both phases, and the
totals over the 27 workbooks:

- P1: `items` 110582, and per workbook `items` equal to the count of
  `<si>` start tags the run script makes over the extracted part with
  its own tool (as part 2 P3 counted elements), asserted per file.
- P2: `sheets` 95; `cells` 2094601; `number` 1310988 (1302755 with
  `t="n"` and 8233 without `t`); `shared` 783275; `inline` 0;
  `formula_string` 273; `boolean` 40; `error` 25; `date` 0; `values`
  2089138 (2089071 with text and 67 empty); `empty` 5463 (the 5410
  empty elements and 53 cells with neither `<v>` nor `<f>`);
  `formulas` 262879; `shared_master` 44; `shared_follower` 292;
  `array` 5; `data_table` 0; `unresolved` 0 and `resolved` 783275;
  `booleans_parsed` 40; `styled` 2094287. `numbers_parsed` is
  expected to equal the `Number` cells with a value; a shortfall is a
  finding about a producer's number text, recorded, and planning
  rules on it. A departure from any fact above is a finding about the
  corpus or the rule, recorded, and planning rules which.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit, the
code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m1p3-<phase>/` (`gate.log`, `test.log`
on both targets, `negative.log`, `run.log`, `scan.log`, and `size.log`
in P3); the first lines of every log as every phase has written them;
every compound check a named function; the scan log last, hashing
every committed sibling, scanning its own source before printing
itself, carrying the narrowed `f_home` (part 2 close-out rulings 3 and
6); every sha256 read from a file that exists; at most two independent
checks; report; STOP. No push during the part; the close-out push
comes after P3 and carries the local commits 9bd9102 and 7702f6e.

### P1: the carried items; shared strings

P1's first code commit carries what part 2 left, and nothing of P1's
feature: the count removed from `xml.rs` ("the other package
messages") with the `include_str!` set-equality test over the
catalogue and the `Error::Package` literals of `package.rs` and
`xml.rs`, `\[` unescaped (part 2 P3 verification ruling 2); the
`Default`-spelling removal test, a content types part whose `Default`
elements carry a space before the slash, one `Override` removed, every
`Default` back byte for byte (ruling 4); `--drop-calc-chain` reading
the content types part before deciding the edit is empty, so a
package with no `[Content_Types].xml` is refused, with part 2's D5
fixture now a refusal in this phase's negative log (ruling 6); the two
overlong comment lines in `scripts/gate-self-test.sh` rewrapped
(ruling 7). The run script's faulty fixture drives every counter by
name and the self-test invocation line names the launch form, from
this phase on (rulings 3 and 7). The gate-file change runs the
self-test and commits `self-test.log`.

P1's second code commit: `cells::SharedStrings`,
`Package::shared_strings`, and `--shared-strings`. Tests on byte-built
packages (the part 2 builder, the shared strings part added as an
entry with its relationship and override): a plain item; an item of
runs whose `<t>` texts concatenate; an item with `<rPh>` skipped; a
character reference and a numeric reference unescaped; leading and
trailing spaces kept with and without `xml:space`; an empty `<t/>`
and an empty `<si/>` both an empty string; `count` and `uniqueCount`
disagreeing with the items and ignored; a workbook with no shared
strings relationship giving an empty table; the root of the wrong
name; non-UTF-8 text; an unknown entity refused; `get` at the length
`None`. The corpus run `--shared-strings` on both targets, recorded as
`records/m1p3-p1-<utc-date>-<shortsha>.md` and `.txt`; acceptance as
"Harness runs".

### P2: the cell reader

`cells::{Cell, CellRef, Cells, Formula, FormulaKind, Kind}`,
`Package::cells`, `SharedStrings::resolve`, and `--cells`. Tests on
byte-built worksheets, each cell's fields compared with what the test
wrote: every kind; `t="n"` and `t` absent both `Number`; `<v>` text,
`<v/>`, and no `<v>`; an inline string of one `<t>`, of runs, and with
`<rPh>`; a normal formula, a shared master with `ref` and its follower
with empty text, an array formula, `dataTable` accepted by name; `ca`
and `aca` present and not modelled; a character reference in `<v>`, in
`<f>` and in `<t>`; `CellRef::parse` on `A1`, `XFD1048576`, `a1`,
`A01`, `A`, `1`, `A1B`, a column beyond `u32`, and `a1()` back; a row
without `r` and a cell without `r`; a cell whose reference names
another row; a style index; elements in another namespace and unknown
elements skipped with their subtrees (`extLst` inside `<c>`, a
`conditionalFormatting` sibling of `sheetData`); the strict main
namespace; a worksheet with an empty `<sheetData/>` giving no cells;
the errors: an unknown cell type, an unknown formula type, a shared
formula without `si`, a bad reference, a bad row number, a bad style
index, the wrong root, an unknown entity, non-UTF-8; `resolve` on a
shared index that is not a number, one out of range, one in range, and
a non-shared cell; and one test that `Package::cells` then `write_to`
writes a byte-identical copy. The corpus run `--cells` on both targets,
recorded as `records/m1p3-p2-<utc-date>-<shortsha>.md` and `.txt`; the
run log prints every key's total per target and asserts each expected
figure by name; acceptance as "Harness runs".

### P3: the typed views, the probe, the size record

`Cell::{number, boolean, error}` with tests (`1.10`, `-0`, `1E+3`,
`inf` and `nan` as the standard library parses them and stated as such
in the doc comment, a non-number `Number` cell `None`; `1`, `0`, `TRUE`
as `None`; an error value); the catalogue and the Limits section in
`cells`'s and `package`'s module headers (the whole table in memory;
no seeking; no date conversion; no evaluation; the unmodelled
attributes); `probe-emend-cells-xml` and the float-parser variant,
measured as "Size" says, recorded as
`records/size-m1p3-<utc-date>-<shortsha>.md`, `size.log` printing
every figure the record carries and re-measuring the eight earlier
probes against part 2's record by the byte classifier; `--cells` gains
nothing. The negative log: the tool on a package whose named sheet
part is missing, on one whose shared strings part is not `sst`, on
one with an unknown cell type (exit 2, the message, no output file), and
the P1 refusals again through the built binary. The corpus run `--cells`
again on both targets with the release binaries the size log hashed,
recorded as `records/m1p3-p3-<utc-date>-<shortsha>.md` and `.txt`,
expecting the P2 totals. The report states part 3's own cost and the
budget figure against the ceiling the checkpoint ruling set.

### Close-out push (after P3)

As part 2's, with the close-out scripts as part 2 ran them, the
self-scan and the narrowed `f_home` included, and the range scan's
header naming part 2's recorded finding as the one it runs again
after. The push carries 9bd9102, 7702f6e and part 3's commits, on
planning's word and the maintainer's authorisation typed in the
implementing session.

## Planning verification and rulings on P1 (2026-09-18)

Verified from primaries at d1ebf1c, seven local commits over 7702f6e.
The doc commit 7175092 carries the five planning files at the sha256
planning recorded before sending them. The code commits are five
where this spec said two: 04113ac (the carried items), 8f2364d (the
table), 001fe24 (the self-test log's launch form, this spec's own P1
item landing late), 5233bf4 and 71ab06e (corrections the independent
checks forced). Accepted as they stand: `specs/0001` permits an amend
of an unpushed commit and does not require one, and a correction that
names what it corrects is the form this project prefers. Every commit
message is ASCII and ends with the Co-Authored-By line. The record
commit d1ebf1c carries six logs, three check answers and the record
pair; `scan.log`'s sibling table equals `sha256sum` over the committed
files; two superseded attempts were re-hashed from the kept files and
match their cited hashes and byte counts; the one attempt that was
not kept is named as having no hash, as the rule asks. Planning's own
runs at d1ebf1c: the nine-step gate, every step PASS; `cargo test -p
emend` 198 passed and 1 ignored on the host and on i686 musl; an
independent Python reader over the corpus, sharing no code with the
crate or with the run script's counter: 27 tables, `items` 110582 and
the largest table 54472, both equal to `run.log`; `bytes` 1497091
against `run.log`'s 1497107, finding P1-F1 below. Privacy over every
file of the phase diff: 0 bare shapes, 0 live pattern hits, 0
non-ASCII bytes, 0 U+2014; the one home-directory string in
`scan.log` is a positive control built by concatenation. P1 is DONE.

Finding P1-F1 (planning's, 2026-09-18). The 16-byte difference is
sixteen carriage returns inside `<t>` text, in 11 items over 10
workbooks: planning counted 26 CR LF pairs in the corpus's shared
strings parts, 16 of them inside `<t>`, 0 lone CR, 0 CDATA sections
and 0 `_x000D_` escapes (counts only; no item was printed). XML 1.0
section 2.11 requires a processor to normalise CR LF and lone CR to
LF before parsing, so expat-based readers (Python's ElementTree, lxml
and therefore openpyxl), Excel and LibreOffice all see LF; `quick-xml`
0.42 does not normalise, and the reader appends text chunks as they
arrive. Not a P1 defect: this spec expected `items` alone, and the P1
record stands as measured. It is a reading defect to fix before any
other text goes through the same path.

Rulings:

1. Line ends. Text this crate reads, shared strings now and in P2
   inline strings, `<v>` and `<f>`, follows XML 1.0 section 2.11: CR
   LF and lone CR become LF, in text and in CDATA alike; a CR reached
   through a character reference (`&#13;`) is kept, as 2.11 says.
   Tests: CR LF, lone CR, LF alone, and the reference, each compared
   with the string the test wrote. P2's `--shared-strings` run expects
   `items` 110582 and `bytes` 1497091, both asserted by name.
2. CDATA (question 8). A CDATA section inside a text element is that
   element's text, verbatim after line-end normalisation, read from
   `Event::CData` (`quick-xml` 0.42 `events/mod.rs`, grepped). Tests:
   a whole item in CDATA, and text on both sides of one. The corpus
   holds none, so the doc comment says the tests alone prove it.
3. Nested elements (question 9). A `Start` or `Empty` event inside
   `<t>`, `<v>` or `<f>` is refused with the new catalogued message
   "a text element holds an element": each is simple content in the
   schema, and a value with a hole in it is not the value. Test with
   an element inside `<t>`; the `text` flag goes.
4. One text-assembly function serves all four elements, reading
   `Text`, `CData` and `GeneralRef` until the matching end tag, so
   rulings 1 to 3 hold once. It lands in P2's first code commit with
   rulings 5, 8 and 9 and nothing of P2's feature, as P1's first
   commit carried part 2's items.
5. A missing target (question 6). A `sharedStrings` relationship whose
   target names no entry is refused with the existing "no such part":
   a relationship is a claim the package makes about itself, and a
   reader asked to follow it and finding nothing says so, as
   `Package::cells` will. The survey counts unresolved targets; the
   readers do not paper over them. The test becomes
   `a_relationship_naming_no_entry_is_refused` and the method's doc
   comment says this; the "Module `package`" text above is amended by
   this ruling.
6. The message wording (question 1). "a cell holds a reference this
   layer cannot expand" stands; P2 raises it from a cell as well, and
   the catalogue then lists both tests beside it.
7. A main-namespace `<t>` inside an `extLst` under `<si>` (question
   2) is the item's text, as the rule says; recorded as a known
   reading with no corpus witness and no change.
8. Integration tests (question 3). P2's first code commit adds one
   for `--shared-strings` through the built binary on a generated
   fixture of two items, asserting `items` and `bytes`; P2's feature
   commit adds one for `--cells`.
9. The test name (question 5). `the_table_is_read_afresh_on_every_call`
   asserts equality of two calls and nothing about caching; renamed
   `a_second_call_gives_an_equal_table`, its comment saying what it
   pins. In P2's first code commit.
10. Question 7: the welded sentence under "32-bit and streaming" is
    corrected in this doc commit, by planning.
11. The four process items are rules in `specs/0001` "Rules for every
    phase" from P2 on: a claim that a test bites is a printed
    measurement or is not made; the shape greps gain the assignment-
    prefix form with a control; a log's phase commit is given to the
    script and compared with HEAD, with a control, rather than read
    from HEAD and printed twice; and the self-test's place in the
    phase is a rule rather than each gate script's own condition.
    Check 2's finding 9 (an amend was available) asks for nothing.
12. P2's first code commit, in full: rulings 1 to 5 with their tests
    and the catalogue entry, ruling 8's `--shared-strings` test, and
    ruling 9's rename. P2's run expects the `bytes` figure of ruling
    1; every other expectation in "Harness runs" is unchanged.

## Out of scope

- Writing a cell, a row or a sheet: part 4.
- Styles, number formats and dates: part 5, and a feature for date
  conversion.
- Defined names, tables, hyperlinks, comments, merged cells,
  conditional formatting, data validation, row and column properties:
  read past, never modelled.
- Chartsheets and dialogsheets: their parts are not worksheets and
  `cells` refuses their root.
- Evaluation of any formula, and the calculation chain (part 6).
