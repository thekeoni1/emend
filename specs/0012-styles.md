# 0012: Milestone 1 part 5, styles

Status: APPROVED by the maintainer 2026-09-22 (planning draft the
same day), the three decisions under "Decisions for the maintainer"
each taken as planning recommended; sent to the implementing session
the same day.
Context: `CLAUDE.md` ("Dependencies and licences": dates live behind
a Cargo feature that is off by default; "32-bit and sizes"),
`ROADMAP.md` ("Milestone 1", part 5), `specs/0011-editing-cells.md`
("Module `cells`", "Module `package`", "Size", the verification
rulings), `specs/0010-reading-cells.md` ("Module `cells`": `Cell::
style`, `Kind::Date`), `specs/0009-package-layer.md` ("Module `xml`",
"Module `package`": the edits, `write_to`), `specs/0008-zip-layer.md`
("Size"), `specs/0004-xlsx-diff.md` (statuses and flags),
`specs/0001-bootstrap.md` ("How this task runs", "Rules for every
phase"). ECMA-376 Part 1, 18.8 (styles: `numFmt`, `cellXfs`, `xf`)
and 18.17.4 (dates as serial numbers) are cited where the design
leans on them; no text is copied from them.

## Goal

A cell written or already present can be given a number format,
including a date format, without renumbering any style the workbook
already has. The styles part is read for what it defines (its number
formats and its cell formats), a new cell format is appended to
`<cellXfs>` as a copy of an existing one with its number format
changed, a new custom number format is appended to `<numFmts>` with
an id no existing format uses, and the two `count` attributes are
updated; nothing else in the part moves, so every `s` in every
worksheet keeps its meaning. A cell takes a format through a style
edit that composes with part 4's value edits: the value of a cell
can be set, its style set, or both, in one write. Dates are the
`dates` feature, off by default: a civil date and time to and from
the serial number a cell holds, in the workbook's own epoch, with no
new dependency.

The harness is the acceptance: over the corpus, a sample that adds
two cell formats and one custom number format, styles one existing
cell and writes one dated cell leaves every other entry
byte-identical, the styles part identical outside the two containers
it grows, the containers' existing children identical, and the
worksheet's cells identical except at the two references, as an
independent parser in the run script shows; every output opens in
LibreOffice; emend's own reader reads the styles and the values back.
The comparison point is openpyxl's `cell.number_format = "..."`
followed by `save`, without the styles part openpyxl regenerates.

## Facts planning checked (2026-09-22)

Over the 27 corpus workbooks, by element shapes and counts with
Python's `zipfile` and `ElementTree` and regular expressions over the
styles parts, never a format code, a font name or a cell's text:

- Every workbook has exactly one `styles` relationship and the part
  it names exists: 27 parts, 7479 to 160215 bytes, 765995 in all.
  None starts with a byte order mark; every one has a declaration
  with `standalone` and exactly one line feed, after the declaration,
  so no whitespace sits between any two elements; the root is
  `styleSheet` unprefixed in every part, with `mc:Ignorable` on 10;
  no attribute is single-quoted and no empty-element tag has a space
  before its slash.
- The root's children come in the schema's order in every part:
  `numFmts fonts fills borders cellStyleXfs cellXfs cellStyles` then
  `colors` (11 parts), `dxfs colors` (6), `dxfs tableStyles colors
  extLst` (6) or `dxfs tableStyles extLst` (4). So `numFmts` and
  `cellXfs` are present in all 27. Every container with a `count`
  attribute (`numFmts`, `fonts`, `fills`, `borders`, `cellStyleXfs`,
  `cellXfs`, `cellStyles`, `dxfs`, `tableStyles`) has it equal to its
  number of children in every part; one `tableStyles` is empty.
- `numFmts`: 109 `numFmt` elements, 100 with an id of 164 or more
  and 9 below 164 (a producer's own spelling of a built-in format);
  the custom ids are contiguous from 164 in every part and the
  largest is 173; 40 distinct codes over the corpus, none repeated
  within a part; 27 codes carry a double quote and one a `<`, both
  escaped in the part; 30 are date-like by the survey's test (a
  letter among `d m h y s` in either case outside quoted runs,
  bracketed runs and backslash escapes, over the whole code; a test
  modelled on openpyxl's, which planning did not run, P1 ruling 1).
- `cellXfs`: 992 `xf` elements, 4 to 142 per part. Every one carries
  `numFmtId`, `fontId`, `fillId`, `borderId` and `xfId`, and starts
  with `numFmtId` (as do the 459 in `cellStyleXfs`); the apply flags
  seen are `applyFont` 854, `applyAlignment` 776, `applyFill` 575,
  `applyBorder` 511, `applyProtection` 226, `applyNumberFormat` 205,
  and `pivotButton` once. 919 have children (`alignment` 912,
  `protection` 227) and 73 are empty. 699 name a built-in format and
  293 a custom one; 106, in 18 parts, name a date format by the test
  above or a built-in date id (14 to 22, 27 to 36, 45 to 47, 50 to
  58). No `xf` names a custom id its part does not define.
- The other tables, not modelled here: `fonts` 6 to 42 per part,
  `fills` 3 to 27, `borders` 1 to 52, `dxfs` 1360 in 16 parts.
- Cells: 2094287 carry `s` and 314 do not (`specs/0011`); no `s` is
  at or beyond its part's `cellXfs` count; 261057 cells in 18
  workbooks carry a date format (two workbooks hold 130000 each); no
  cell has `t="d"`. `workbookPr` sets `date1904` in 2 workbooks.

## Design

### Crate

No new dependency. A `styles` module (read and add), `Package::
styles`, `Package::add_cell_style`, `Package::set_cell_style`, a
styles-part rewrite and a worksheet start-tag rewrite in `write_to`,
`Workbook::date1904`, and the `dates` module behind the Cargo feature
`dates`, off by default, with no dependency of its own. `lib.rs`
re-exports `Styles`, `Xf`, `NumFmt`, `NumberFormat` and, under the
feature, `dates::{DateTime, Epoch}`. `Styles` and `Xf` each carry a
private field (the `xf` bytes; whether the part has a `numFmts`), so
neither is built by a struct literal outside the crate (P1
verification ruling 3). Every index and id is `u32`, every byte
position `u64`, as before.

### Module `styles`: what is read

`Package::styles(&mut self) -> Result<&Styles, Error>` reads the
workbook's styles part once and caches it as `workbook()` does. The
part is read through `Stream` and `Scope` with the element resolution
`Cells` uses: the root must be `styleSheet` in the main namespace
(`Error::Package("the styles part's root is not styleSheet")`);
under it, `numFmts`'s `numFmt` children and `cellXfs`'s `xf` children
are read and every other element is skipped raw at any depth. A
second `numFmts` or a second `cellXfs` at the root is refused
(`Error::Package("the styles part has more than one numFmts or
cellXfs")`, P1 ruling 4): the schema allows one of each, and a
second `cellXfs` would make every `s` ambiguous.

`Styles { num_fmts: Vec<NumFmt>, cell_xfs: Vec<Xf> }`, both in part
order, so `cell_xfs[i]` is what a cell's `s="i"` names.

- `NumFmt { id: u32, code: String }`: `numFmtId` as `decimal` reads a
  digit string (`specs/0010`) and `formatCode` read as sheet names
  are, through `normalized_value(XmlVersion::Implicit1_0)`
  (`specs/0009` P2 ruling 2; P1 ruling 3), so a reference such as
  `&#9;` survives and a literal tab, line feed or carriage return in
  the attribute becomes a space as XML 1.0 3.3.3 says. A `numFmt`
  lacking either is refused (`Error::Package("a numFmt lacks
  numFmtId or formatCode")`); an id that is not a digit string is
  refused (`Error::Package("a style attribute is not a number")`).
- `Xf { num_fmt_id: Option<u32>, font_id: Option<u32>, fill_id:
  Option<u32>, border_id: Option<u32>, xf_id: Option<u32> }`: each
  its attribute when present, read as `decimal` (the same refusal),
  `None` when absent, as the schema leaves them optional. The apply
  flags, `alignment` and `protection` are not modelled; the reader
  keeps every `xf`'s own bytes (start tag through end tag, or its
  empty-element tag) privately, so an addition can copy one.
- `Styles::number_format(&self, xf: u32) -> Option<NumberFormat>`:
  `None` when `xf` is out of range; else `NumberFormat::Code(code)`
  when a `NumFmt` in `num_fmts` has the `xf`'s `num_fmt_id` (the
  first such, including an id below 164 a producer spelled out), else
  `NumberFormat::BuiltIn(id)` with the `xf`'s id, or `BuiltIn(0)`
  when the `xf` has none.

`NumberFormat { BuiltIn(u32), Code(String) }`, `PartialEq`. The
built-in ids are ECMA-376 Part 1, 18.8.30's table: 0 `General`, 1 to
4 the integer and two-decimal forms with and without a thousands
separator, 9 and 10 percentages, 11 scientific, 12 and 13 fractions,
14 to 17 dates, 18 to 21 times, 22 date and time, 37 to 40
accounting-like integer forms, 45 to 47 durations, 48 scientific, 49
`@` (text); 5 to 8, 23 to 26, 41 to 44 and the ranges 27 to 36 and 50
to 58 are locale-defined. The module's doc lists them; this layer
does not check that an id below 164 is one the table defines, and
what a consumer shows for one it does not is not this layer's to
say.

A workbook without a `styles` relationship gets `Error::Package("the
workbook has no styles part")` from `styles()`; a relationship whose
part does not exist gets "no such part" as `cells()` does. The
corpus has neither case.

### Module `styles`: what can be added

`Package::add_cell_style(&mut self, base: Option<u32>, format:
NumberFormat) -> Result<u32, Error>` records one new cell format and
returns the index it will have: the number of `xf` elements
`cellXfs` holds plus the number already recorded by this package. It
reads the styles part first when it is not cached, so it fails as
`styles()` fails, and it refuses:

- `base` at or beyond `cell_xfs.len()` (`Error::Package("a style
  index is out of range")`); a base names an existing `xf` only,
  never one recorded by this package.
- `BuiltIn(id)` with `id >= 164` (`Error::Package("a built-in number
  format id is 164 or more")`).
- `Code(code)` empty or holding a character XML 1.0 forbids (the set
  `specs/0011` names) (`Error::Package("a number format code is empty
  or holds a character XML cannot carry")`).

The number format's id: `BuiltIn(id)` is `id`, and nothing is added
to `numFmts` whether or not the part spells that id out. `Code(code)`
is the id of the first `NumFmt` whose `code` equals it, else the id
of the first code equal to it recorded earlier by this package, else
a new id, the largest of 164, one more than the largest id in
`num_fmts` and one more than the largest id recorded earlier; the
new `(id, code)` is recorded. Two additions with equal codes share
one id; two additions with an equal base and format are two `xf`s
(nothing is deduplicated at the `xf` level, and the doc says so). An
id or an index that cannot be allocated (a `numFmt` id at `u32::MAX`,
or `u32::MAX` cell formats) is refused with "a style index is out of
range" (P1 verification ruling 1); no part is near either bound.

The new `xf`'s bytes, composed at the call from the base's own bytes
and held with the record:

- `base` `Some(i)`: the start tag is written anew as `<xf` then the
  base's attributes in the base's order, each as a space, the name,
  `=`, the quote character the base used for it, the value's bytes as
  the base wrote them (still escaped) and that quote again (P1 ruling
  5: a single-quoted value may hold a double quote), with
  `numFmtId`'s value replaced by the id (or added first
  when the base had none) and `applyNumberFormat`'s replaced by `1`
  (or appended last when the base had none); then, when the base had
  children, the base's bytes from after its start tag to before its
  end tag, raw, and `</xf>`; else `/>`. The base's tag is a template
  for a new element, so its spacing is not what the output shows;
  every unedited `xf` is the input's bytes.
- `base` `None`: `<xf numFmtId="N" fontId="0" fillId="0" borderId="0"
  xfId="0" applyNumberFormat="1"/>`, the form 73 corpus `xf`s take
  with their own ids.

Elements take the qualified-name prefix of the container they go
into, as part 4's new cells do.

### Module `package`: recording and applying style edits

`Package::set_cell_style(&mut self, part: &[u8], reference: CellRef,
style: Option<u32>)` records, as `set_cell` does: nothing is read,
nothing can fail, and the last style edit recorded for a reference
wins, independently of the last value edit for it. `Some(i)` names an
`xf` by index, existing or returned by `add_cell_style`; `None`
removes the cell's `s`.

`write_to` adds, after part 4's validation and before any part is
rewritten:

- Validation: a style edit names an entry that exists, that no edit
  removes and that no edit replaces (the three part 4 messages, a
  style edit being a cell edit); every `Some(i)` has `i` below the
  `cellXfs` count plus the additions recorded (`Error::Package("a
  style index is out of range")`), for which the styles part is read
  when it is not cached, so a `Some` edit on a workbook with no
  styles part is refused with "the workbook has no styles part" and
  a `None` edit needs no styles part at all (a decision below); when
  an addition is recorded, the styles part is not removed and not
  replaced by this write (`Error::Package("a style addition to a part
  this write replaces")`, and "an edit to a removed part"), and it
  has a `<cellXfs>` (`Error::Package("a styles part without cellXfs
  cannot take a style")`, found while it streams, leaving no output).
- The styles part, when an addition is recorded, is read through
  `Stream` and `Scope` and written to a temporary file: every token's
  `raw` is copied except as follows. Immediately before `</numFmts>`
  the recorded `<numFmt numFmtId="N" formatCode="..."/>` elements are
  written in id order, the code escaped with the crate's `escape` (a
  double quote as `&quot;`, the form 27 corpus codes take) and then a
  tab, a line feed and a carriage return as `&#9;`, `&#10;` and
  `&#13;`, so the code reads back equal to itself and a later equal-
  code lookup finds it (P1 ruling 3). The
  `numFmts` start tag is written with its `count` attribute's value
  bytes replaced by the number of `numFmt` children plus the number
  added and every other byte of the tag kept; a `numFmts` without
  `count` gets none; a value that is not a digit string is refused
  ("a style attribute is not a number"). When the part has no
  `numFmts` and a code is added, `<numFmts count="k">`, the elements
  and `</numFmts>` are written immediately before the start tag of
  the first root child, in the part's own order, that is one of
  `fonts`, `fills`, `borders`, `cellStyleXfs` and `cellXfs`: the
  schema's position for a part whose children are in schema order,
  and for a part that is not, before whichever of the five comes
  first (P1 verification ruling 5); `cellXfs` is present by the
  refusal above. Immediately before
  `</cellXfs>` the recorded `xf`s are written in record order, and
  the `cellXfs` start tag has its `count` treated as `numFmts`'s. An
  empty `<numFmts/>` or `<cellXfs/>` is opened up as part 2 opens an
  empty root. The output's `count` equals the container's children,
  so an input whose `count` disagreed with its children comes out
  corrected: a consequence, stated in the Limits, that the corpus
  never exercises (0 mismatches).
- The worksheet rewrite of part 4 takes style edits into its
  per-reference map, now `(Option<Option<Value>>, Option<Option<
  u32>>)`: a value edit, a style edit, or both. With a value edit,
  `Value::element` takes the edit's style when there is one (`Some(i)`
  as `s="i"`, `None` as no `s`) and the existing cell's `s` bytes
  otherwise, as part 4. A clear with a style edit `Some(i)` leaves
  `<c r="A1" s="i"/>`, whether or not the cell existed (P1
  verification ruling 2); a clear with `None` removes the cell as an
  unstyled clear does. A style edit alone on an existing cell writes
  its start tag anew, `<c r="A1" s="i" t="..."/>` from the cell's own
  `r` and `t` (when it had one) and the edit's style, `/>` when the
  cell was empty, else `>`, the children and the end tag raw: a
  formula, a value or an inline string stays as it was, so a style
  edit removes no formula and never drops the chain, and the formula
  refusals of part 4 (shared, array, data table, unknown type) do not
  apply to it; the unmodelled-attribute refusal does, as to every
  edited cell. A style edit alone on a cell that does not exist
  writes `<c r="A1" s="i"/>` where part 4 would insert a new cell,
  creating the row as part 4 does, and writes nothing for `None`.
  The out-of-range check above is the only reading of the styles part
  a style edit causes.
- The temporaries become the parts' replacements and the rest of
  `write_to` runs as before. The styles part is rewritten at most
  once per write, as a worksheet is.

Consequences a caller should know, stated in the Limits: a new `xf`
copies its base's font, fill, border, alignment and protection and
changes its number format only; a style set on a cell whose row has
`customFormat` or whose column has a `<col style>` overrides those as
any `s` does; a cell shown as a date is a number with a date format,
and what a consumer shows for a format code is the consumer's;
`dxfs`, `cellStyles`, `fonts`, `fills` and `borders` are never
touched; a number format's `count`, when it disagreed with the
children, is corrected.

### Module `dates` (feature `dates`)

Behind `[features] dates = []`, off by default, no dependency: the
arithmetic is a few dozen lines, and the feature keeps the core's
budget figure the core's own and lets the record show the feature's
cost (a decision below). ECMA-376 Part 1, 18.17.4.1 and 18.17.4.2
define the two epochs: in the 1900 system serial 1 is 1900-01-01 and
serial 60 is the day 1900-02-29 that the calendar does not have, so
every later day is one more than the civil count; in the 1904 system
serial 0 is 1904-01-01 and every date is 1462 less than in the 1900
system. The time of day is the fraction.

- `Epoch { Y1900, Y1904 }`, `Copy`, `PartialEq`. `Workbook::date1904:
  bool` (core, not the feature: one attribute) is `<workbookPr>`'s
  `date1904` read as `true` for `1` or `true` and `false` otherwise or
  when absent; `Epoch::of(&Workbook) -> Epoch` maps it.
- `DateTime { year: i32, month: u32, day: u32, hour: u32, minute:
  u32, second: u32, millisecond: u32 }`, `Copy`, `PartialEq`, `Debug`.
- `DateTime::serial(&self, epoch: Epoch) -> Option<f64>`: `None` when
  the fields are not a proleptic Gregorian civil date and time
  (1900-02-29 included: it is no civil date), when the date is before
  the epoch's first day (1900-01-01, 1904-01-01) or after 9999-12-31;
  else the day count in the epoch as above plus `(hour * 3600000 +
  minute * 60000 + second * 1000 + millisecond) / 86400000`.
- `DateTime::from_serial(epoch: Epoch, serial: f64) -> Option<Self>`:
  `None` when `serial` is not finite, below 1 in the 1900 system or
  0 in the 1904 system, at or above the day after 9999-12-31, or is
  the day 60 of the 1900 system; else the civil date of the integer
  part and the time of the fraction rounded to the nearest
  millisecond, carrying into the next day when the rounding reaches
  it.
- Pinned values, computed by planning from Python's `date` ordinals
  with the rule above: 1900-01-01 is 1, 1900-02-28 is 59, 1900-03-01
  is 61, 1904-01-01 is 1462 (1900) and 0 (1904), 1970-01-01 is 25569
  and 24107, 2000-01-01 is 36526 and 35064, 2024-02-29 is 45351 and
  43889, 9999-12-31 is 2958465 and 2957003; noon adds 0.5.

A dated cell is `Value::Number(serial)` with a cell format whose
number format is a date (built-in 14 to 22, or a code); nothing in
the core knows which numbers are dates, and `Kind::Date` (`t="d"`,
an ISO text no corpus cell uses) stays as part 3 left it.

### Errors

Ten new `Error::Package` messages (nine in the draft; the tenth by
P1 ruling 4). Raised by the reading, so carried by P1's first code
commit with their catalogue entries, the set-equality test reading
`styles.rs` from that commit (P1 ruling 2): "the workbook has no
styles part", "the styles part's root is not styleSheet", "a numFmt
lacks numFmtId or formatCode", "a style attribute is not a number",
"the styles part has more than one numFmts or cellXfs". Raised by
the additions and the write, so carried by the second: "a style index
is out of range", "a built-in number format id is 164 or more", "a
number format code is empty or holds a character XML cannot carry",
"a style addition to a part this write replaces", "a styles part
without cellXfs cannot take a style". Each joins the catalogue in
`package`'s module header with the test that pins it, and the
set-equality test reads the module that raises it.

### 32-bit and streaming

`Styles` holds every `numFmt`'s id and code and every `xf`'s five ids
and own bytes: 160215 bytes for the corpus's largest part, bounded by
the part's size, and the doc says a styles part larger than memory
cannot be read (no corpus part is a hundredth of that). The styles
rewrite holds one token, the recorded additions and two counts; the
worksheet rewrite adds one `Option<u32>` per edited reference. Every
rewritten part streams to a temporary file as before.

### Size

`tools/size-probe` gains two probes, built in the `size` profile on
i686 musl as the others:

- `probe-emend-styles-write-xml`: `probe-emend-cells-write-xml`'s
  work, then `styles()` (printing the `xf` and `numFmt` counts), two
  `add_cell_style` (base 0 with a `Code` from its command line, base
  `None` with `BuiltIn(14)`), `set_cell_style` of the first on the
  number cell and of the second on the text cell, and after the
  write the output's `styles()` counts and the two cells' `style`
  through `Package::cells`, so no path is dead. The budget figure is
  it over `probe-zip-xml` against the ceiling of 393216 section bytes
  (`specs/0010` "Size"); part 5's own cost is it over
  `probe-emend-cells-write-xml`, and the record states beside it the
  budget figure's whole growth since part 4's 278064, with the drift
  of the earlier probes classified (`specs/0011` P3 verification
  ruling 4).
- `probe-emend-dates-write-xml`: the same with the `dates` feature,
  writing the serial of a `DateTime` from its command line in the
  workbook's epoch and printing `from_serial` of the number read
  back. The feature's cost is it over `probe-emend-styles-write-xml`,
  recorded beside the budget figure and outside it: a feature off by
  default is what `CLAUDE.md` keeps out of the core's size.

Part 4 left 115152 section bytes of headroom. Planning's expectation,
not a measurement: the styles work under 30000, the feature under
4000. The size log re-measures the nine earlier probes against part
4's record by the byte classifier.

### Harness runs

`tools/emend-copy` gains three operations, at most one per
invocation, given after the two paths as the others are; none takes a
reference, a code or a date from its command line. The tool enables
the `dates` feature of the crate (the probes above measure the core
without it).

- `--styles` (P1): reads the workbook and the styles part and prints
  one line, the word `styles` then tab-separated `key=value`: `xfs`
  (`cell_xfs.len()`), `num_fmts`, `custom` (ids of 164 or more),
  `dated` (`xf`s whose `number_format` is `BuiltIn` 14 to 22, 27 to
  36, 45 to 47 or 50 to 58, or a `Code` that is date-like by the
  survey's test as "Facts" states it, over the whole code with every
  bracketed run stripped, implemented in the tool and documented
  there as the survey's test and no library's), `date1904` (0 or 1).
  The
  output is the input copied with no edit, byte-identical. Expected
  totals over the 27: `xfs` 992, `num_fmts` 109, `custom` 100,
  `dated` 106, `date1904` 2, and 18 workbooks with `dated` above 0.
- `--style-sample` (P2): the styles part is read (X `xf`s, F
  `numFmt`s); the first sheet of kind `Worksheet` is read once
  through `Package::cells` to find A, the first cell of kind `Number`
  or `SharedString` with a value and no formula, and M, the largest
  row number (0 for no cells). It records S1 = `add_cell_style(A's
  style, Code("0.000"))` (base `None` when A has no `s` or does not
  exist; no corpus part defines that code, so `numFmts` grows by one
  in every workbook), S2 = `add_cell_style(None, BuiltIn(14))`, A's
  style set to `Some(S1)` when A exists, and `(M + 1, 1)` set to
  `Number(45000.5)` with style `Some(S2)`; writes; reopens; reads the
  output's styles and the same sheet; and prints `edit` with `styled`
  (1 or 0 by whether A exists), `added` 1, `xfs_before` X,
  `xfs_after` (expected X + 2, asserted by the tool: exit 2 with a
  message when it differs, the output removed as part 4's assertion
  removes it), `formats_added` (the output's `numFmt` count minus F,
  expected 1), `cells_before`, `cells_after` (expected `cells_before
  + 1`, asserted), `read_back` (the number of these three that hold:
  (a) A's `value` text unchanged and `style` `Some(X)`; (b) the new
  cell `number()` 45000.5 and `style` `Some(X + 1)`; (c) the output's
  `number_format(X)` is `Code("0.000")` and `number_format(X + 1)` is
  `BuiltIn(14)`; expected `styled + 2`, (a) failing only when A does
  not exist).
- `--date-sample` (P3): as `--style-sample`'s new cell alone: `(M +
  1, 1)` set to `Number` of `DateTime` 2024-02-29 12:00:00.000 in
  `Epoch::of` the workbook, with style `add_cell_style(None,
  BuiltIn(22))`; prints `edit` with `date1904` (0 or 1), `serial`
  (the number written, expected 45351.5 or 43889.5 by `date1904`),
  `xfs_after` (expected X + 1, asserted), `cells_after` (expected
  `cells_before + 1`, asserted), `read_back` (1 when the cell reads
  back `number()` equal to `serial`, `style` `Some(X)`, and
  `from_serial` of it in the same epoch equal to the `DateTime`
  written, else 0).

The runner and the record script are unchanged; each run names its
operation through `--tool-arg`.

Expected results, `lo` and `lo_in` `ok` in every case:

- P1: `--styles` on both targets: every entry `identical` unflagged,
  `files_identical` `yes` in all 27, the per-workbook lines summing
  to the totals above, and the run log's own count of the same
  figures with `ElementTree` agreeing on every key.
- P2: in every workbook the styles part and the first worksheet part
  `changed`, every other entry `identical` unflagged, `descriptor`
  alone allowed on a changed entry as `specs/0008` says for a
  replaced one, `order_preserved` `yes`, `files_identical` `no`; the
  run log's own check over the extracted input and output parts: for
  the styles part, the bytes before `<numFmts`, the bytes from
  `</numFmts>` to `<cellXfs`, and the bytes from `</cellXfs>` to the
  end are identical; inside `numFmts` the input's children's bytes
  are a prefix of the output's, one `numFmt` follows with an id equal
  to the largest input id plus one (or 164 when the input had no id
  of 164 or more) and `count` grows by one; inside `cellXfs` the
  input's children's bytes are a prefix of the output's, two `xf`s
  follow, the first with `numFmtId` equal to that new id and
  `applyNumberFormat="1"` and the second `<xf numFmtId="14" fontId=
  "0" fillId="0" borderId="0" xfId="0" applyNumberFormat="1"/>`, and
  `count` grows by two; for the worksheet, the comparison of
  `specs/0011` P2 with `changed` equal to `styled` (A's `s` alone
  differs; its `t`, formula presence and value are the input's),
  `added` 1 at `(M + 1, 1)` with `s` equal to X + 1, `removed` 0;
  `read_back` equal to `styled + 2` in every file; totals `xfs_after`
  1046 and `formats_added` 27. The check prints no reference, no
  code and no text.
- P3: in every workbook the styles part and the first worksheet part
  `changed` and every other entry `identical` unflagged, as P2;
  `xfs_after` totalling 1019; `date1904` totalling 2, `serial`
  43889.5 in those two and 45351.5 in the other 25 (the run log
  asserts the pairing per workbook); `read_back` 1 in every file; the
  styles check as P2 with no `numFmt` added and one `xf` with
  `numFmtId="22"`; the worksheet check with `changed` 0, `added` 1,
  `removed` 0. A departure from any expectation is a finding about
  the corpus or the rule, recorded, and planning rules which.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit, the
code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m1p5-<phase>/` (`gate.log`, `test.log`
on both targets, `negative.log`, `run.log`, `scan.log`, and `size.log`
in P3); every rule the earlier parts added, the scan log last with
the self-scan and the narrowed `f_home`, every sha256 read from a
file that exists, a commit message's counts printed by a log and its
test claims existing at that commit, an independent check's directory
holding one diff per code commit, at most two independent checks;
report; STOP. No push during the part; the close-out push comes after
P3 and carries the local commit 7cc4f13.

Part 5's first doc commit carries, unedited: `ROADMAP.md`,
`specs/0011-editing-cells.md` (the close-out rulings) and this spec.

### P1: the styles part read and grown; style edits

P1's first code commit: `Workbook::date1904`, the `styles` module's
reading (`Styles`, `NumFmt`, `Xf`, `NumberFormat`, `number_format`),
`Package::styles`, `--styles` and the five reading messages with
their catalogue entries (P1 ruling 2), with tests on byte-built
styles parts: the five ids present and absent; a `numFmt` with an
escaped quote, one with `<` and one with `&#9;` (read as a tab) and
a literal tab (read as a space); a second `numFmts` and a second
`cellXfs` each refused; a part without `numFmts`; a part without
`cellXfs`; an `xf` with children and one empty; the root prefixed
(`x:styleSheet`) and the strict main namespace; each refusal;
`number_format` for a custom id, a producer-spelled built-in, a
built-in with no `numFmt`, an `xf` with no `numFmtId` and an index
out of range; `date1904` as `1`, `true`, `0`, absent and `workbookPr`
absent; and the tool's line on a fictional workbook.

P1's second code commit: `add_cell_style`, `set_cell_style`, the
styles rewrite and the worksheet start-tag rewrite in `write_to`, and
the other five messages. Tests on byte-built packages, the output's
parts compared with bytes the test builds by hand (`specs/0011` P1
ruling 5): an `xf` added from a base with children (the children
raw, the tag rebuilt, `numFmtId` replaced, `applyNumberFormat`
replaced when present and appended when not); from a base with a
single-quoted attribute whose value holds a double quote (the quote
kept); from an empty base; from `None`; a code with a tab, a line
feed and a carriage return written as references and read back
equal; a
base without `numFmtId` gaining one first; a code new (id 164 on a
part with no custom id; largest plus one otherwise), a code equal to
an existing `numFmt` (nothing added, its id used, including one
below 164), two additions with equal codes (one `numFmt`, two `xf`s),
a `BuiltIn` (nothing added); `count` rewritten on both containers, a
container without `count`, an empty `<numFmts/>` and `<cellXfs/>`
opened up, a part without `numFmts` gaining one before `fonts` and,
on a part with `cellXfs` alone, before `cellXfs`; a part without
`cellXfs` refused at write with no output; a prefixed part's new
elements prefixed; every byte outside the two containers identical
on a part with `fonts`, `dxfs` and `extLst`; every existing child
byte-identical, including one whose spacing a rebuild would change;
a value edit with a style, with `None`, and without a style edit on
a styled cell (the `s` kept); a clear with `Some` and with `None`; a
style edit alone on a cell with a formula (the `<f>` and `<v>` raw,
the chain untouched), on an inline string, on an empty styled cell,
on a cell in an array `ref` (allowed), on an absent cell in an
existing row and in a new row, and `None` on an absent cell (nothing
written); the same cell's style set twice (the last wins) and a
value edit recorded after a style edit for it (both apply); a `Some`
edit on a workbook with no styles part refused and a `None` edit on
it accepted; an index equal to the count plus additions refused and
one below it accepted; a style edit to a replaced part, and an
addition on a package that replaces or removes the styles part,
refused; a refused write leaving no temporary and no output; the
sample's every read-back through `Package::styles` and
`Package::cells`. The negative log: the tool on the refusals it can
reach through the built binary. The corpus run `--styles` on both
targets, recorded as `records/m1p5-p1-<utc-date>-<shortsha>.md` and
`.txt`; acceptance as "Harness runs".

### P2: the style sample over the corpus

P2's first code commit carries the P1 verification rulings: the
clear-alone-on-an-absent-cell case added to the test
`a_style_edit_alone_on_an_absent_cell_writes_it_or_nothing`
(ruling 8). Then
`--style-sample` with an integration test on a fictional workbook
(the tool's existing fixtures), and the run log's `ElementTree` and
byte-span check with a control that reports a planted difference per
flag it raises. The corpus run on both targets, recorded as
`records/m1p5-p2-<utc-date>-<shortsha>.md` and `.txt`; acceptance as
"Harness runs".

### P3: dates, the probes, the size record

P3's first code commit carries the P2 verification rulings. Then the
`dates` feature: `Epoch`, `DateTime`, `serial`, `from_serial`, with
tests for the pinned values in both epochs, noon and a millisecond
in each, 1900-02-29 and day 60 refused, the day before each epoch
and the day after 9999-12-31 refused, an invalid civil date
(2023-02-29, month 13, day 0) refused, a non-finite serial refused,
the rounding that carries into the next day, and every pinned value
round-tripped through both functions; the tool's `--date-sample`
with its integration test; the catalogue and Limits in `package`'s,
`styles`'s and `dates`'s module headers (the consequences above; the
refusals; number formats only; the feature's absence from the core);
`probe-emend-styles-write-xml` and `probe-emend-dates-write-xml`,
measured as "Size" says, recorded as
`records/size-m1p5-<utc-date>-<shortsha>.md`, `size.log` printing
every figure the record carries and asserting the crate builds with
the feature off and on (`cargo build -p emend` and `--features
dates`) on both targets. The negative log through the built binary:
a workbook whose styles part lacks `cellXfs` and one whose `xf` has a
non-numeric `numFmtId`, each through `--styles` or `--style-sample`
and the release `probe-emend-styles-write-xml` the size log builds
(exit 2 and the message; the probe's outputs treated as `specs/0011`
P3 ruling 8 treats them); the messages no binary can reach named as
unreached from the source beside the test that raises each. The
corpus run `--date-sample` on both targets with the release binaries
the size log hashed, recorded as `records/m1p5-p3-<utc-date>-
<shortsha>.md` and `.txt`. The report states part 5's own cost, the
whole growth, the feature's cost and the budget figure against the
ceiling.

### Close-out push (after P3)

As part 4's, with the close-out scripts as part 4 ran them, one
record commit after the push unless the maintainer chooses the split
again (`specs/0010` close-out ruling 2), the scan rerun last, the
`ANTHROPIC_API_KEY` check. The push carries 7cc4f13 and part 5's
commits, on planning's word and the maintainer's authorisation typed
in the implementing session.

## Decisions for the maintainer

Decided by the maintainer 2026-09-22: each as recommended.

1. Scope: number formats and cell formats derived from existing ones
   now; new fonts, fills, borders and alignments in a later part
   (recommended, and what this draft says). A derived `xf` already
   gives a cell any combination of font, fill, border and alignment
   the workbook has, with the number format changed, which covers
   dates and numeric display, the two asks the roadmap names. The
   alternative adds in this part a `Font` derived from an existing
   `<font>` with bold and italic toggled, appended to `fonts` by the
   same copy-and-append rule: one more element grammar (`<b/>`,
   `<i/>` among `sz`, `color`, `name`, `family`, `scheme`), a third
   container rewritten, and a third sample; planning estimates a
   phase's worth of work and would rather see the append rule proven
   on two containers first.
2. Dates as the Cargo feature `dates`, off by default, with no
   dependency (recommended). `CLAUDE.md` lists dates among what lives
   behind a feature, and the feature keeps the budget figure the
   core's own with the feature's cost recorded beside it. The
   alternative puts the two functions in the core, since they are
   arithmetic alone and planning expects them under 4000 section
   bytes; every consumer then carries them. A crate such as `chrono`
   or `time` is not proposed under either choice: nothing here needs
   more than the civil calendar.
3. Style indices validated against the styles part at write
   (recommended). A write with a `Some` style edit reads the styles
   part once (160215 bytes at most in the corpus) to know the
   `cellXfs` count, and refuses an index at or beyond it, so emend
   never writes an `s` a consumer would repair. The alternative
   trusts the caller and reads the styles part only when an addition
   is recorded, saving that read on a write that only reuses existing
   indices.

## Planning rulings during P1 (2026-09-22)

On the implementing session's five questions before the first code
commit, each written here before it is cited. The doc commit 489d4e2
was verified by planning from primaries: parent 7cc4f13, exactly the
three files at the hashes the commit message prints, the message
ASCII with the trailer alone, the amended first attempt 5154b2b with
an equal tree (the amendment of an unpushed, unverified commit's
message is what `specs/0001` allows; disclosed, accepted).

1. The date-like test is this spec's prose: a letter among `d m h y
   s` in either case anywhere in the code, after quoted runs, every
   bracketed run and backslash escapes are removed. "Facts" no longer
   calls it openpyxl's test, since planning did not run openpyxl and
   the implementing session's recollection has it reading the first
   section only and keeping the elapsed-time brackets; the
   implementing session's unlogged count shows both readings give
   106 in 18 workbooks over the corpus, so the corpus does not decide
   it, and the tool's doc names the test as the survey's.
2. The messages split by the code that raises them: the five raised
   by the reading (with ruling 4's) in the first code commit, with
   their catalogue entries and the set-equality test extended to
   `styles.rs`; the other five in the second. "Errors" and "P1" say
   so. Planning's handoff message said nine in the second commit;
   the spec governs.
3. `formatCode` is read through `normalized_value(XmlVersion::
   Implicit1_0)` as sheet names are, and a code is written with a
   tab, a line feed and a carriage return as character references
   after `escape`, so a code reads back equal to itself and the
   equal-code lookup holds. Refusing the three would have been the
   other choice; nothing in the format forbids them, and the corpus
   has no witness either way.
4. A second `numFmts` or `cellXfs` at the root is refused by the
   reader with the tenth message, "the styles part has more than one
   numFmts or cellXfs", pinned by two tests; the corpus has none.
5. The `xf` template keeps each attribute's own quote character, so
   a single-quoted value holding a double quote stays well-formed;
   one test pins it. The corpus has no single-quoted attribute.

## Planning verification and rulings on P1 (2026-09-22)

Verified by planning from primaries at c3d1adb (doc 489d4e2, cbc691b;
code 35cd499, abf2618; record c3d1adb): its own gate at HEAD (nine
PASS), its own tests on both targets (441 passed, 2 ignored, each),
its own privacy scan over 7cc4f13..c3d1adb (5 commits, 418 blobs, 0
findings), the nine committed files at the hashes the report gives
and the scan's own table, the run directory's tool line for every
workbook on both targets against planning's own `ElementTree` count
(54 of 54 agree on every key; totals 992, 109, 100, 106, 2), no
forbidden check shape in the five logs, the commits in gate order.
The report's six items and two offers, and one matter of its own:

1. An id or index that cannot be allocated is refused with "a style
   index is out of range"; "what can be added" says so. No message
   of its own: nothing reaches it.
2. A clear with `Some(i)` on a cell that does not exist writes
   `<c r="A1" s="i"/>`, as the rule "a cleared cell survives only
   with a style" reads; "Module `package`" says so.
3. `cells::forbidden_in_xml` is `pub(crate)`, and `Styles` carries
   a private `has_num_fmts` read through a `pub(crate)` method, so
   `Styles`, like `Xf`, is not built by a struct literal outside the
   crate; "Crate" says so. No public item changed between the two
   code commits in a way any consumer saw.
4. The Limits prose and `write_to`'s consequences written with the
   code in the second commit stand; P3 completes the catalogue and
   the Limits for the part as "P3" says, and does not move them.
5. A created `<numFmts>` goes before the first, in the part's own
   order, of the five elements named; "Module `package`" now says so
   and what a part out of schema order gets.
6. `ROADMAP.md`'s P1 row said nine messages; planning corrected it
   to ten (ruling 4 of the P1 rulings) in its working tree, for P2's
   first doc commit. The report's "51 in all" is its own sum of the
   negative log's 35 and 16, recomputed by planning; no log prints
   it and none needs to.
7. The first attempt of run.log stays in the implementing session's
   scratch, hashed in the rerun header as every superseded attempt
   is; it is not committed. Its host totals are cited by the report
   from that file and by nothing committed; the committed attempt
   measures the same thing on both targets.
8. Check 2's suggestion is taken in P2's first code commit: the
   clear-alone-on-an-absent-cell case joins
   `a_style_edit_alone_on_an_absent_cell_writes_it_or_nothing`, so a
   phase diff shows it. No P1 amendment.
9. The one substitution in each check answer (the working copy's
   absolute path to `CLAUDE.md`, quoted by the checker's own
   disclosure sentence, written as `<repo>/CLAUDE.md`) is the
   exception `specs/0001` already allows for an absolute machine
   path a checker quoted; the declaration sits in each file's header
   with the offset, the byte counts and the sha256 of the answer as
   returned, which serves the rule's purpose as a bracketed note at
   the place would. The answers stand; scan attempts 2 (FAIL) and 3
   (PASS) are both hashed. Planning notes for the harness, not for
   this project to fix: a checker's context carried the project
   instruction file as a system reminder, which each checker
   disclosed and did not use.
10. P1 is DONE. P2 is READY: its first doc commit carries this spec
    and `ROADMAP.md` unedited, then the first code commit with
    ruling 8's test, then `--style-sample` as "P2" says.

## Planning rulings during P2 (2026-09-22)

Check 1 of P2 (its answer 1822672adf83fe18, on the ruled inputs at
7c96575) found the defects below. Planning confirmed each in the log
it names before ruling. A rerun is a new attempt whose header hashes
every superseded attempt, as `specs/0001` says.

1. `run.log` did not print its script. `specs/0001` requires the
   exact script in the log. Attempt 3 prints the assembled script
   in full, as P1's run log did, and the report's claim that it was
   printed is corrected to name attempt 3 as the first that did.
2. The corpus check's `dated_xf_ok` tested six substrings, not the
   bytes "Harness runs" gives for the second added `xf`. Attempt 3
   tests byte equality with that exact string, and its controls gain
   a plant that reorders the attributes, which the equality must
   catch.
3. `negative.log` said it compared the message list by name with
   P1's log but read part 4 P3's. Attempt 3 compares with
   `records/logs/m1p5-p1/negative.log` by name: none removed, none
   added by P2's library code, and its header says so.
4. The run log's comparison reads with `expat`, not `ElementTree`.
   Both are independent of the library, and `expat` supplies the
   byte offsets the span check needs. It stays. This spec's
   "ElementTree" in "Harness runs" P2 and in "P2" is read as "an
   independent Python XML reader", and the run log's heading says
   which one.
5. `gate.log` and `test.log` carry stale header comments (a
   self-referencing source path; a control described from a
   superseded attempt; a phase directory named where the code reads
   the other). Their checks are sound and are not rerun. The report
   lists each stale comment with its line. `negative.log`'s header
   is corrected in its attempt 3.
6. Every figure the report states is printed by a log or is
   dropped. Attempt 3 of the run log prints the count of rows
   marked unexpected, so "0 UNEXPECTED" can be cited; line numbers
   in code, the hash prefixes planning gave, superseded-attempt
   details and the removal of a run directory are not measurements
   and leave the report or are marked as the implementer's own
   statement. The mutation caught by ruling 8's case is reported
   with every test the log names as catching it.
7. The record pair is rewritten from attempt 3 and the report names
   the superseded pair's hashes. The pre-check scan is rerun, the
   report revised, and check 2 is given check 1's answer with the
   attempt-3 inputs. Check 1's answer is committed as it was
   written, its header pinning the attempt-2 inputs it read.
8. This section is not committed inside P2: a doc commit may not
   follow a code commit within a phase. The report cites it by this
   file's hash, and P3's first doc commit carries it.

## Planning verification and rulings on P2 (2026-09-22)

Verified by planning from primaries at 747a96e (doc 6e6062c; code
07b84be, 7c96575; record 747a96e): its own gate at HEAD (nine PASS),
its own tests on both targets (445 passed, 2 ignored, each), its own
privacy scan over c3d1adb..747a96e (4 commits, 420 blobs, 0
findings), every committed file at the hash the report gives and
`scan.log`'s hash table agreeing with each, the doc commit's two
blobs at the hashes planning gave, the four commits in gate order
with the Co-Authored-By line only, and the two code diffs read
against "P2" and "Harness runs". Planning's own comparison over the
attempt-3 run directory, written without reading the run log's, read
every workbook's input, both outputs and both edit lines: on all 27
workbooks and both targets every entry but the styles part and the
first worksheet is byte-identical, the head, middle and tail spans of
the styles part are identical (the created-container case cut at the
creation point), the input's children are a prefix of the output's
in both containers, one `numFmt` follows at the expected unused id,
two `xf`s follow with the first coded and the second the exact bytes
"Harness runs" gives, both `count` attributes equal their children
before and after, the worksheet differs only in A's start tag (its
`s` alone) and the one added cell at (M + 1, 1) with the number and
`s` equal to X + 1 (in four cases into a row element that existed
without cells, its start tag kept), A is the first value cell with no
formula, the edit line's arithmetic holds, and the host and i686
outputs are byte-identical. Totals on both targets: `xfs_before` 992,
`xfs_after` 1046, `formats_added` 27, `styled` 27. A control with the
`xf` bytes reordered and the number changed turned those two flags in
all 54. The record pair's outcome, open-check and part-family tables
agree with that comparison.

1. Ruling 4's stated reason was wrong and is withdrawn: the run
   log's spans come from regular expressions and its own tokenizer,
   and `expat` reads relationships, cells and attributes. The ruling
   stands on its other ground, a reader independent of the library.
   The run log's header item C repeats the withdrawn reason; it is a
   stale statement in a committed log, listed here, not rerun. The
   tokenizer ends a tag at the first `>`; planning counted attribute
   values holding a raw `>` in every styles and worksheet part of
   the corpus with `ElementTree`: none. The hazard did not arise, and
   a later comparison that keeps the tokenizer says so in its header.
2. The label "the hash planning's P2 rulings give" in the run and
   negative logs is wrong: this file cannot hold its own hash, and
   the value was typed from planning's message. The value is right
   (the file at 397da302 is the one the rulings were written into),
   so the check's substance stands. From P3 the label reads "the
   hash planning's handoff gave" and the report says so.
3. The aggregate "every flag falsified" check (a sum of at least 17)
   is accepted for P2 on the sample pair's 17 of 17 at run.log:2207.
   From P3 a comparison's controls assert per flag that at least one
   pair turned it.
4. The run script's `#!/bin/sh` line with bash syntax, run as
   `bash <script>`, is accepted on the implementer's statement. From
   P3 every script the phase prints carries `#!/usr/bin/env bash`
   and the log prints the invocation.
5. The two stale docstrings in the comparison, the ten stale header
   comments the report lists for `gate.log`, `test.log` and
   `negative.log`, and check 1's "eleven" for twelve inputs are
   recorded here and not rerun (ruling 5 during P2).
6. Check 1's two harmless code notes: `sample()` cloning A's value
   for every caller stays (a tool, not the core); the test comment at
   the refusal test that says "the first addition refuses" where the
   styles read refuses first is corrected in P3's first code commit,
   as P1 verification ruling 8 placed such a correction.
7. Decision 1 stands: "count grows by one" and "by two" in "Harness
   runs" P2 are read as the output's `count` equalling its children,
   which are the input's children plus the additions, as "Module
   `package`" already says of the writer; whether the input's own
   attribute matched its children is reported beside the flags. On
   this corpus every input attribute matched (planning counted the
   same).
8. Decisions 3 to 5 of the report are accepted: the pre-check and
   final scans cover planning's uncommitted file whenever one stands
   in the tree; a Co-Authored-By line names the model that wrote the
   commit and nothing else (the record commit's differs from the
   phase's earlier three); check 2 was rightly given the working-tree
   spec carrying the rulings that governed the reruns.
9. The rulings during P2 and this section are committed by P3's
   first doc commit, with `ROADMAP.md`, both unedited.
10. P2 is DONE. P3 is READY.

## Planning rulings during P3 (2026-09-22)

Asked by the implementing session after the doc commit 6256267 and
before any code. Each is ruled as proposed unless it says otherwise,
and a second doc commit carries this file before the first code
commit, as P1's cbc691b did.

1. Five code commits, in this order: C1 the test-comment correction
   of P2 verification ruling 6, alone; C2 the `dates` feature (the
   Cargo feature, `dates.rs` with `Epoch`, `DateTime`, `serial`,
   `from_serial` and their tests, the re-export under the feature);
   C3 `emend-copy` enabling `dates` and `--date-sample` with its
   integration test and flag-combination cases; C4 the catalogue and
   Limits in the module headers of `package`, `styles` and `dates`;
   C5 `size-probe`'s `dates` feature and the two probes. C4 stays
   its own commit so its diff is prose alone.
2. `from_serial` rounds first: after the finiteness check, the
   total milliseconds are `round(serial * 86400000)` as a signed
   64-bit integer, and every refusal in "Module `dates`" (below the
   epoch's first day, at or above the day after 9999-12-31, the day
   60 of the 1900 system) applies to that rounded value. So a serial
   a hair under 60 is refused as day 60, a hair under 1 in the 1900
   system reads as 1900-01-01 00:00:00.000, and a hair under the day
   after 9999-12-31 is refused. Tests pin those three edges in both
   directions of each boundary.
3. "A proleptic Gregorian civil date and time" in `serial` means:
   month 1 to 12, day 1 to the month's length in that year
   (1900-02-29 excluded), hour 0 to 23, minute 0 to 59, second 0 to
   59 (no leap second), millisecond 0 to 999; anything else is
   `None`. Tests add hour 24, minute 60, second 60 and millisecond
   1000, each refused, beside the date refusals "P3" lists.
4. The gate is unchanged: once `emend-copy` enables `dates`, the
   workspace test and clippy runs unify the feature on. `test.log`
   runs `cargo test -p emend` on both targets twice, plain and with
   `--features dates`, asserting the `dates` tests absent from the
   first and present in the second, and runs `cargo clippy -p emend
   --all-targets -- -D warnings` with the feature off, once, on the
   host. `size.log` builds the crate off and on as "P3" says.
5. `--date-sample` prints eight keys in this order: `date1904`,
   `serial`, `added` (1), `xfs_before`, `xfs_after`, `cells_before`,
   `cells_after`, `read_back`. "Harness runs" P3 is read with the
   three bases added so the run log's comparison checks the two
   asserted figures against what the line itself carries. `serial`
   is the tool's own constant written, never a corpus number.
6. The negative log asserts each route's own message on the
   no-`cellXfs` fixture: `--style-sample`, with A carrying no `s`
   (base `None`), reaches "a styles part without cellXfs cannot take
   a style" at the write; the release probe, whose first addition
   takes base 0, reaches "a style index is out of range" from
   `add_cell_style` before anything is written. The log names the
   source line of each, as `specs/0011` P3 ruling 8 did for the
   probe's outputs. The non-numeric `numFmtId` fixture reaches "a
   style attribute is not a number" by both routes.
7. The defaults stand: the dates probe takes its `DateTime` as seven
   integers parsed with `str::parse`; `probe-emend-styles-write-xml`
   takes its `Code` as one argument; `size-probe`'s `dates` feature
   is `["emend", "xml", "emend/dates"]`; each probe is built with its
   own `-p size-probe --features` set and the size log prints every
   build command, so no probe is unified with another's features;
   the size log's nine earlier probes are part 4's nine.

## Planning verification and rulings on P3 (2026-09-22)

Verified by planning from primaries at 39baf23 (doc 6256267, 130f7ff;
code 805526d, c08bf76, b2bf6e3, 478da97, f565868; record 39baf23):
its own gate at HEAD (nine PASS); its own workspace tests on both
targets (463 passed, 2 ignored, each) and the crate's tests with the
feature off (347, no `dates` test) and on (361, fourteen `dates`
tests) on both targets, with clippy on the crate with the feature off;
its own privacy scan over 747a96e..39baf23 (8 commits, 443 blobs, 0
findings); every committed file at the hash the report gives; the two
doc commits' blobs at the hashes planning gave; the eight commits in
gate order with the Co-Authored-By line only; `dates.rs` and its tests
read against "Module `dates`" and rulings 2 and 3, with the pinned
serials and both end-of-range serials recomputed from Python's `date`
ordinals and agreeing; `--date-sample` and both probes read against
"Harness runs" P3 and "Size". Planning rebuilt `probe-zip-xml`,
`probe-emend-cells-write-xml`, `probe-emend-styles-write-xml` and
`probe-emend-dates-write-xml` in the `size` profile on i686 musl with
the size log's commands: each reproduced the size log's file bytes,
section bytes, `.text` bytes and sha256 exactly, so the budget figure
304412, part 5's own cost 4224, the whole growth 26348 and the
feature's cost 4980 are planning's measurements as well as the log's.
Planning's own comparison over the attempt-2 run directory, written
without reading the run log's: on all 27 workbooks and both targets
every entry but the styles part and the first worksheet is
byte-identical, the styles part differs only by one `xf` of the exact
bytes "Harness runs" P3 gives with `count` up by one, `numFmts` is
untouched, the worksheet differs only by the one added cell at
(M + 1, 1) holding the serial the line prints with `s` equal to X,
`date1904` matches `workbookPr`, the serial is paired with it, the
eight keys stand in ruling 5's order, the arithmetic holds, and the
host and i686 outputs are byte-identical. Totals on both targets:
`xfs_after` 1019, `date1904` 2, `read_back` 27.

1. Finding 1: the feature's cost, 4980 section bytes, exceeds
   planning's expectation of 4000 by 980. The expectation was an
   estimate and "Size" calls it one; the measurement stands, the
   figure is outside the budget as decision 2 places it, and no
   change is asked. The record's reading is the record.
2. Findings 2 to 4 are accepted: the negative log finds the raise
   sites by grep; the size log's generated fixture carries one `xf`
   and a `date1904` variant, the reason stated as the implementer's
   unlogged observation; the two record stems carry the UTC dates
   their runs fell on.
3. Finding 5, the two chained checks in the test script that the
   pre-check scan caught, is the implementer's own defect, found by
   the project's own gate before any checker and fixed with a
   refusing control for each. Recorded; the shape grep runs over a
   script before its log does, from part 6 on.
4. Findings 6 and 7 are accepted: every superseded attempt is hashed
   in its successor's header, and the run log's attempt 2 changed
   header text alone, with the identity of its figures the
   implementer's statement.
5. Finding 8: the two commit messages with small inaccuracies (C3's
   "no log prints a count at" it; C4's "ten styles messages") stand
   unamended, since the logs cite their hashes; this section records
   the corrections.
6. Check 2's note that no P3 log compares a file with a hash
   planning gave is right: P3's doc commits were verified by planning
   from the blobs, and P2 verification ruling 2's label had nothing
   to attach to. It attaches when a later phase's log makes such a
   comparison.
7. The prose commit 478da97 was read: the catalogue and Limits say
   what "Module `dates`", "Decisions for the maintainer" and the
   rulings say, and nothing else.
8. P3 is DONE. Part 5's close-out is READY: a doc commit carrying
   this file and `ROADMAP.md` unedited, as part 4's 35e38a5 did; then
   the push of 7cc4f13 and part 5's commits on planning's word, given
   here, AND the maintainer's authorisation typed in the implementing
   session; then the close-out record commit after the push, the scan
   rerun last, the `ANTHROPIC_API_KEY` check, as "Close-out push"
   says.

## Planning verification and rulings on the close-out (2026-09-23)

Verified from primaries by planning, not from the report: `git
fetch` and `git ls-remote` show origin/main at 77d2b50, equal to
local main at the push; `gh repo view` says PRIVATE; `gh run view
35884307712` reads head 77d2b50, event push, branch main, conclusion
success, every step success, the only ci.yml run on that commit; the
record commit b0c21ef touches exactly records/logs/m1p5-close/
push.log, ci-run.log and scan.log at the sha256 the report prints,
with the scan hashing both siblings at those values and naming the
superseded pre-push scan by hash, which planning found in the
implementing scratch at that hash and byte count; the 19 messages
7cc4f13..77d2b50 ASCII, 9 ending with the Opus 5 Co-Authored-By
line and 10 with the Opus 5.5 line, no session link in any; the
only home-shaped strings in the three logs are the path form's own
regex and the scan's run-time control fixture; denylist entries
counted in the three logs: 0; planning's own `scripts/privacy.sh
--range 35e38a5..77d2b50` reads 19 commits, 464 blobs, 0 findings,
PASS, and over 77d2b50..b0c21ef 1 commit, 435 blobs, 0 findings,
PASS. Planning diffed each printed script against part 4's: the
push and CI scripts differ only in comments, names, the spec and
the base; the scan script differs in those and in the two-line
check the report declares. The maintainer's authorisation was typed
in the implementing session and the record is the single commit
after the push, as "Close-out push (after P3)" says.

Findings: none. Rulings:

1. The close-out's script changes are accepted: the base resolved
   as part 4's close-out doc commit by subject; the derivation notes
   condensed; and, in the scan, the Co-Authored-By check holding
   each message to one of the two lines the range carries, with the
   line printed per message, a count per line, the two counts
   summed against the messages scanned, and four controls (each
   line accepted; a similar third line and a session trailer
   refused). A later range with a third line adds it the same way,
   with its control, and never widens the check to a pattern.
2. b0c21ef stays local and goes with the next push, as 7cc4f13 went
   with this one.
3. Milestone 1 part 5 is DONE and PUSHED. Part 6 (formulas) is next
   and starts with its spec, for the maintainer's approval.

## Out of scope

- New fonts, fills, borders, alignments, protection and named cell
  styles (`cellStyles`, `cellStyleXfs`); `dxfs`; the theme.
- Column and row styles (`<col style>`, `<row s>`), conditional
  formatting and table styles.
- Reading which cells are dates by their format, or converting a
  cell's number to a date in the reader: a caller with the feature
  does it with `number_format` and `from_serial`.
- Removing or rewriting an existing `xf` or `numFmt`, and reclaiming
  unused ones.
- Formulas, `calcPr` and recalculation on open: part 6.
- A workbook without a styles part, or writing one from nothing.
