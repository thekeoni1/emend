# 0024: Milestone 3 part 2, rich text

Status: APPROVED by the maintainer 2026-10-02 ("A on all
(Recommended)", chosen in the planning session's question:
decisions 1 to 6 as planning recommended, 7 recorded). Sent to the
implementing session the same day.

Context: `CLAUDE.md` ("What emend is": like openpyxl; "Dependencies
and licences": anything beyond the core behind a Cargo feature off
by default; "The fidelity harness is the acceptance instrument";
"32-bit and sizes"), `ROADMAP.md` ("Milestone 3", the outline
approved 2026-10-01, item 2: runs with their own font in a cell,
shared and inline strings, and in a comment, written and read
back, as openpyxl's `CellRichText`; feature `richtext`; 40000 to
55000), `specs/0010-reading-cells.md` (the shared strings table
held whole, `text_under` over `<si>` and `<is>` with the `<rPh>`
rule, `Cell` and `Kind`), `specs/0011-editing-cells.md` (`Value`,
a text written as an inline string, never a shared string, never
rich text), `specs/0012-styles.md` and `specs/0017-fuller-
styles.md` (`Font`, `Color`, `Underline`, `VerticalAlign`,
`Scheme`; `Font::compose`), `specs/0022-comments-and-images.md`
(`Comment`, `CommentEntry`, the comments part's `<text>` written
as one run with no properties, the read flattening runs),
`specs/0023-scale.md` (the decision record's finding 2, the shared
strings table's hold at 46.2 bytes per item on i686; the close-out
rulings 2 to 4, carried here), `specs/0001-bootstrap.md` ("Rules
for every phase"). ECMA-376 Part 1, 18.4.4 (`r`, rich text run),
18.4.7 (`rPr`, run properties), 18.4.8 (`si`), 18.4.12 (`t`),
18.3.1.53 (`is`, rich text inline), 18.7 (a comment's `text`,
the same `CT_Rst`) are cited where the design leans on them; no
text is copied from them. openpyxl's `CellRichText`, `TextBlock`
and `InlineFont` are the behavioural reference, read only.

## Goal

A cell value made of runs, each run its text and optionally a
font, written as an inline string with one `<r>` per run and read
back as runs from an inline string and from the shared strings
table; the same runs in a comment's text, written and read back;
a corpus sample per capability with the open checks; the size
probe; the catalogue; the close-out push. Everything behind the
Cargo feature `richtext`, off by default. The core probe does not
move by design.

## Size and packaging (decision 1, before any code)

The budget figure after part 1 is 435352 section bytes against
the ceiling of 458752 (`records/size-m3p1-2026-10-02-6578925.md`):
23400 of headroom, held for fixes. The outline's estimate for
this part is 40000 to 55000; planning's, with the measured costs
in view (`tables` 46904 for one created part and one worksheet
container, `comments` 57016 for two created parts, `images` 40944
for one, `rowcol` 35472 for a shift over one part), is 30000 to
45000 section bytes for `richtext` alone: one new write branch,
one reader of run properties, the table's sparse runs, and no
part created. The comment runs, linked only when `comments` is on
too, are expected at 5000 to 15000 over the `comments` probe. The
core's growth for the part is what the feature gates add, printed
by the size log and expected near 0; above 2000 section bytes, or
a feature cost above 55000, planning is told in the report.

- A (recommended): one Cargo feature `richtext`, off by default,
  declared beside `comments` with no dependency of its own; two
  probes, `probe-emend-richtext-write-xml` (the feature alone,
  over the core probe) and `probe-emend-richcomment-write-xml`
  (`comments` and `richtext`, over the `comments` probe), so a
  consumer that wants rich cells and not comments knows its
  figure, and the pair's cost is its own figure. 23 probes.
- B: one probe with both features, one figure, 22 probes. The
  cell capability is priced with the comment code in it.

## Facts planning checked (2026-10-02)

- The corpus, counted by planning with Python's `zipfile` and a
  regular expression over the raw parts, names and counts only,
  as corrected by P1 verification ruling 1 (planning's first
  count used a glob that missed the one `.xltx`): 8 of the 27
  workbooks carry `<r>` runs in the shared strings table
  (18d1e8ba72b5 21 runs in 3 items, 9ada43d71c80, ed2fbd304d51
  and f56906d42e0a 23 runs in 7 items each, f9efcac9808f 11 in
  2, 80adab651c77 and d8245b84086f 5 in 1 each, ed4c55f07c69 2
  in 1), 29 rich items and 113 runs in all, 2 to 9 runs per item,
  no plain `<t>` before or after the runs and no empty run; no
  workbook has an inline string (`t="inlineStr"` 0 everywhere);
  5 workbooks carry runs in a comments part (75387404b8af 7,
  9ada43d71c80 and f56906d42e0a 6 each, 054df34b1472 and
  f8459281d739 2 each), every comment run with an `<rPr>`; no
  `<rPh>` anywhere (`specs/0010`'s fact still holds).
- The corpus's `<rPr>` children, by name and order, 17 distinct
  sequences over the table and the comments of all 27 workbooks
  (ruling 1's recount): every one is a subsequence of `b`, `i`,
  `strike`, `u`, `sz`, `color`, `rFont`, `family`, `charset`,
  `scheme`; `charset` occurs 28 times and no `tint` sits beside
  an `rgb` or `indexed`. `Font::compose`
  (`crates/emend/src/styles/elements.rs`) writes `b`, `i`,
  `strike`, `u`, `vertAlign`, `sz`, `color`, `name`, `family`,
  `scheme`: the same order with `name` in `rFont`'s place and
  `charset` absent. The corpus is Excel's order; the schema's
  `CT_RPrElt` is not consulted for the order, the measurement is.
- `Value::Text` is written as an inline string, `xml:space=
  "preserve"` always, a carriage return as `&#13;`; nothing is
  appended to the shared strings table and no run is written
  (`crates/emend/src/cells.rs`, the module header and
  `Value::element`; the package header's catalogue line 679).
- `text_under` reads both run containers, `<si>` and `<is>`,
  concatenating every `<t>` and skipping `<rPh>` subtrees;
  `Cell::value` and `SharedStrings::get` give the flattened text;
  `Cell` has `reference`, `style`, `kind`, `value`, `formula`.
- No reader of font children exists: the styles part is read for
  its counts (`Styles::fonts`), and part 5 of the outline is the
  typed read-back of styles. The `<rPr>` reader this part writes
  is the first reader of those children.
- `Comment { author, text }` is `Copy`; `comment_elements` writes
  `<text><t>` with no `<r>`; `read_comments` flattens every `<t>`
  under `<text>` into `CommentEntry::text`; `NewComment { author,
  text }` is what the write holds.
- `tools/emend-copy` has 25 `--<name>-sample` flags and the reads
  `--cells`, `--shared-strings`, `--styles`, `--survey`; none is
  named rich. The tool turns every feature on.
- `records/logs/m3p1-p3/run.log` prints the tool's lines for the
  three read runs (tier d `--shared-strings` and `--cells`, tier c
  `--cells`, lines 4528 to 4621), the peaks 266288 / 216572,
  266384 / 216572 and 7724 / 7432 kB host / i686 in the decision
  record; the scale run directories were deleted in the cleanup
  of 2026-10-02, so the lines in the log are the comparison.
- The feature costs after part 1 (section bytes over the core
  probe): `dates` 4776, `rowcol` 35472, `metadata` 50404, `rules`
  48100, `tables` 46904, `comments` 57016, `images` 40944.
- openpyxl, planning's reading of 3.1, read only: `CellRichText`
  is a list whose items are a `str` or a `TextBlock(InlineFont,
  text)`; `InlineFont` carries `rFont`, `charset`, `family`, `b`,
  `i`, `strike`, `outline`, `shadow`, `condense`, `extend`,
  `color`, `sz`, `u`, `vertAlign`, `scheme`; rich text is read
  only with `load_workbook(rich_text=True)` and otherwise
  flattened to its text; a written `CellRichText` goes to the
  shared strings table; `Comment` is plain text.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. Under decision 1 A,
everything below sits behind the Cargo feature `richtext`, off by
default, with the types and calls of its own and its messages in
the catalogue marked as the feature's; the comment runs sit under
`richtext` and `comments` together. The core builds, tests and
links without it; the tool `emend-copy` turns it on as it does
`comments`. Nothing existing changes signature: the additions to
`Value`, `Cell`, `SharedStrings` and `CommentEntry` are
feature-gated variants, fields and calls, absent when the feature
is off.

### Types

In `cells`, feature `richtext`:

- `Run { text: String, font: Option<Font> }`, derived `Clone`
  and `Debug`, `PartialEq` and `Eq` by hand (P1 ruling 1): one
  `<r>`; `font` is the `<rPr>`,
  `None` for a run without one. `Font` is `styles::Font`
  (decision 3): `name` is the run's `rFont`, every other field
  the child of the same name.
- `RichText { runs: Vec<Run> }`, derived `Clone`, `Debug` and
  `Default`, `PartialEq` and `Eq` by hand (P1 ruling 1), with
  `text(&self) -> String`, the runs'
  texts concatenated, which is what `Cell::value` and
  `SharedStrings::get` hold for the same cell.
- `Value::Rich(RichText)`, a variant of `Value` present with the
  feature, taken by `set_cell` as the other variants are.

### The cell write

`Value::Rich` is written as an inline string, `t="inlineStr"`,
whose `<is>` holds one `<r>` per run, in order: `<rPr>` first when
the run has a font, then `<t xml:space="preserve">` with the text
escaped exactly as `Value::Text` escapes, a carriage return as
`&#13;`. Nothing is appended to the shared strings table
(decision 2). The `<rPr>` is `Font::compose`'s element with the
element name `rPr` and the name child `rFont`, so the children
come in the order the corpus measured; a `Font::default()` font is
written `<rPr/>`. The cell edit's rules of `specs/0011` hold
unchanged: a new cell carries no `s`, a replaced one keeps its
own, the cell loses its `t`, formula and value, the off-grid
refusal, the one-rewrite streaming pass.

Checked at the write, as `Value::Text` is: a rich text with no run
or with a run whose text is empty is refused, "a rich text is not
valid"; a run's text holding a character XML 1.0 forbids is
refused with `Value::Text`'s message; a run's font is refused as
`add_style` refuses one (the font-name message, "a style field is
out of range", the colour checks), the same constants.

### The read

- `Cell` gains `runs: Option<RichText>`, present with the feature:
  `Some` for an inline string whose `<is>` holds at least one
  `<r>`, else `None`; `value` is the flattened text as today, so
  a caller that ignores the field sees what it saw. A plain `<t>`
  before the runs (the schema's first child; the corpus has none)
  reads as a first run with no font. The cell pass holds one cell
  at a time as today, the runs part of that cell.
- `SharedStrings` gains `runs_of(&self, index: u32) ->
  Option<&RichText>`, present with the feature: `Some` for an
  item with at least one `<r>`, else `None`; `get` and `resolve`
  unchanged. The contract (decision 4): the table's memory for an
  item WITHOUT runs does not grow, measured on tier d (4800000
  items, none rich) as "Harness runs" says; the mechanism is the
  implementing session's (a sparse list indexed by item is the
  obvious one) and is named in the catalogue.
- The `<rPr>` reader: one function reading the font children into
  a `Font`, given the local name of the name child (`rFont` here;
  part 5 may give it `name`), shared by the two containers and
  the comment read. `b`, `i`, `strike` with no `val`, or `val`
  `1` or `true`, read as true, with `0` or `false` as false; `u`
  with no `val` reads as `Single`, `val="none"` as `None`; `sz`
  as the number; `color` through the forms `Color` has (`rgb`,
  `theme` with `tint`, `indexed`, `auto`); `family` as the
  number; `scheme` and `vertAlign` by keyword. The five children
  `Font` has no field for (`charset`, `outline`, `shadow`,
  `condense`, `extend`) are not read and not kept: the typed view
  drops them, and the catalogue says so; an unedited cell or item
  is copied raw, so the file keeps them. A `val` that does not
  parse, an unknown child, a child outside the main namespace:
  refused, "a run's properties cannot be read". `<rPh>` is
  skipped as today.

### Comments

Features `richtext` and `comments` together:

- `Package::add_rich_comment(part, cell, author: &str, text:
  &RichText)`: `add_comment` with the `<text>` holding one `<r>`
  per run, the `<rPr>` as the cell write's and the `<t>` with
  `xml:space="preserve"` when its text begins or ends with
  whitespace, as `comment_elements` does for one `<t>`. The
  author rule, the replacement rule, `remove_comment`, the parts,
  the VML shape and every refusal of `specs/0022` hold unchanged.
  Checked at the call, as `add_comment` checks: no run, a run
  with empty text, or the flattened text past 32767 characters,
  "a comment text is not valid"; a forbidden character,
  `Value::Text`'s message; a font, `add_style`'s refusals.
- `CommentEntry` gains `runs: Option<RichText>`, present with the
  feature: `Some` when the `<text>` holds at least one `<r>`,
  read by the same reader; `text` the flattened text as today. A
  threaded comment's legacy fallback reads as its runs say.

### Creating an element

None. This part creates no worksheet element and no part; the
cell write is `specs/0011`'s pass and the comment write
`specs/0022`'s.

### Errors

Two new `Error::Package` messages, in the catalogue as the earlier
parts' are, one constant each, marked by feature:

`richtext`:
- "a rich text is not valid" (at the write for a cell)
- "a run's properties cannot be read" (at the read of a cell, the
  table or a comment)

Shared (constants under the feature, one definition): `Value::
Text`'s "a text holds a character XML cannot carry"; `add_style`'s
"a font name is empty or holds a character XML cannot carry" and
"a style field is out of range" and its colour refusals; part
4d's "a comment text is not valid" and "a comment author is not
valid"; part 3's index message for a cell off the grid; part 1's
messages for an edit to a removed or replaced part; part 2's
unbound-prefix refusal; `Error::Xml` for bytes that are not UTF-8.
Nothing is checked twice.

### 32-bit and streaming

Every count is `u32` or `u64` as elsewhere. The worksheet pass
streams as today, one cell's runs held with the cell; the shared
strings table is held whole as today, its rich items' runs beside
it and nothing for the others (the measured contract above); a
comments part is streamed once as today. No new whole-part hold.
The shared strings table's hold itself stays carried to part 5
(`specs/0023` close-out ruling 3).

### Size

Two probes under decision 1 A. `probe-emend-richtext-write-xml`
(feature `richtext` of the probe crate, turning on
`emend/richtext`): `probe-emend-format-write-xml`'s work,
unchanged, then on the same package before its write `set_cell`
at a fixed cell past the format probe's edits of a `Value::Rich`
of two runs, the first bold with a name from the command line,
the second with no font; after the write the output opened, its
table's rich item count and the cell's run count read back through
`runs_of` and `Cell::runs` and printed. `probe-emend-richcomment-
write-xml` (features `comments` and `richtext`): `probe-emend-
comments-write-xml`'s work, then `add_rich_comment` of two runs at
a second fixed cell, the comment's run count read back through
`comments` and printed. The size log measures them as part 1's
did, classifies the twenty-one probes of part 1's record against
it, the two new ones listed as new, prints the feature's cost
(the richtext probe minus the format probe rebuilt at the same
commit), the pair's cost (the richcomment probe minus the comments
probe), the core's growth for the part (the format probe at the
commit minus the same probe rebuilt at 6578925, both printed;
expected near 0), and the budget figure against 458752, which
this part does not move by design; the cause control and the
manifest cause listing stay.

### Harness runs

`tools/emend-copy` gains two operations, given after the two paths
as the others are; neither takes a reference, name or number from
its command line; every constant below is the tool's own. W is
the first worksheet part in `<sheets>` order and M its largest row
number as the tool's cell read gives it; a cell past the grid
refuses the sample whole with part 3's index message. No
reference and no text from the corpus is printed. The tool's
`--cells` and `--shared-strings` lines are unchanged, so part 1's
printed lines stay the comparison.

- `--richtext-sample <in> <out>` (P1): `set_cell(W, (M + 1, 1),
  Value::Rich(...))` of two runs, "emend" with `Font { bold:
  true, ..Default::default() }` and " rich" with no font; writes;
  reopens the output and prints `richtext` with
  `sst_rich_before`, `sst_rich_after` (items of the table with
  `runs_of` `Some`; equal, asserted, the output removed on a
  difference as `--style-sample` does), `inline_rich_before`,
  `inline_rich_after` (cells of W with `runs` `Some`; `+ 1`,
  asserted), `runs_after` (the written cell's run count; 2,
  asserted), `text_after` (1 when the cell's `value` is
  "emend rich", asserted), `bold_first` (1 when the first run's
  font is bold and the second run has none, asserted),
  `r_bound_before`.
- `--richcomment-sample <in> <out>` (P2): `add_rich_comment(W,
  (M + 1, 1), "emend", ...)` of two runs, "emend:" bold and
  " rich comment" with no font; writes; reopens the output and
  prints `richcomment` with the comment sample's counters as
  `specs/0022` lists them (`comments_before`, `comments_after`,
  `parts_before`, `parts_after`, `legacy_before`,
  `comments_part_before`, `r_bound_before`, `shapes_before`,
  `shapes_after`, `authors_before`, `authors_after`, each
  asserted as there) and `rich_comments_before`,
  `rich_comments_after` (entries with `runs` `Some`; `+ 1`,
  asserted), `runs_after` (2, asserted), `text_after` (1 when the
  entry's `text` is "emend: rich comment", asserted).

The runner and the record script are unchanged; the run names its
operation through `--tool-arg` and one `--label`. Expected
results, `lo` and `lo_in` `ok` in every case; `descriptor` alone
allowed on a changed entry; `order_preserved` `yes`;
`files_identical` `no`; for the richtext sample W `changed` and
every other entry `identical`, the shared strings part included;
for the richcomment sample exactly the comment sample's
expectations in `specs/0022`, "Harness runs".

The run log's own check over the extracted input and output
parts, with `ElementTree` and byte spans and a control that
reports a planted difference per check, printing no name, no
reference and no text from the corpus:

- W, richtext: every row byte-identical except the appended row,
  which holds one `<c>` with `t="inlineStr"`, an `<is>` of
  exactly two `<r>`, the first with `<rPr><b/></rPr>`, the second
  without `<rPr>`, each `<t>` with `xml:space="preserve"`, the
  texts the tool's constants; the shared strings part
  byte-identical.
- The comments part, richcomment: as the comment sample's checks,
  and the appended `<comment>`'s `<text>` holding exactly two
  `<r>` of the same shape.

Every phase's run log also runs the edit open check on the host
over that phase's keep, and names the phase directories it
compares against by their record commit. P3 repeats part 1's
three read runs (tier d `--shared-strings` and `--cells`, tier c
`--cells`, both labels, under the 2048 MiB cap, `TMPDIR` on the
disk) with the size log's release binaries: each output
byte-identical to its input, each tool line byte-identical to the
one `records/logs/m3p1-p3/run.log` prints, and each peak RSS
within 2048 kB of the decision record's figure for that run and
label, which is the measured form of decision 4's contract. A
departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. LibreOffice's
rendering of a run is not checked, only that the file opens; the
Excel check (decision 7) shows them as Excel shows them, and this
part's two samples join its list.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m3p2-<phase>/` (`gate.log`,
`test.log` on both targets with the features off and on,
`negative.log`, `run.log`, `scan.log`, `size.log` in P3); every
rule the earlier parts added (`specs/0022` "Phases" lists them;
`specs/0023`'s close-out carries the scan script's
`ANTHROPIC_API_KEY` check and the ci-run newline); the scan log
last; at most two independent checks, each check's prompt giving
the checker its directory as `<phase>/check<n>`; report; STOP. No
push during the part; the close-out push follows P3 and carries
the record commit 3cbd6fd of part 1's close-out with this part's.

Part 2's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0023-scale.md` as planning left them after part 1's
close-out (the close-out rulings 1 to 4 and the ROADMAP rows,
`specs/0023` close-out ruling 4), and this spec. Part 2's first
code commit carries, before any other change, `specs/0023` P3
verification ruling 1: the `package` header's block "What streams
and what is held" names P3's run log beside P2's in its first
paragraph, `//!` lines alone.

### P1: the cell write and the read

The doc commit; the first code commit as above; then the feature
`richtext` declared in the crate and turned on by the tool, `Run`,
`RichText`, `Value::Rich`, the write, `Cell::runs`,
`SharedStrings::runs_of`, the `<rPr>` reader, the two messages,
and `--richtext-sample`. Tests on byte-built parts: a rich value
of two runs written and read back equal, through an inline string
(the bytes checked against literal bytes, the `<rPr>` children in
the measured order, `<rPr/>` for a default font); every `Font`
field written and read back (name, size, bold, italic, strike,
each underline, each vertical alignment, each colour form, family,
each scheme); a run with a leading space and one with a carriage
return and a tab; a table item with runs read through `runs_of`
and `get` (flattened), an item without runs giving `None`, a
plain `<t>` before runs read as a first run with no font, `<rPh>`
inside a rich item skipped, `b val="0"` and `u val="none"` read as
false and `None`; `charset` in an `<rPr>` dropped from the typed
view; a `val` that does not parse and an unknown child each
refused; the write's refusals (no run, an empty run, a forbidden
character, an empty font name, a size of zero); a replaced cell
keeping its `s`; a `Value::Rich` on a sheet with the feature's
off-grid refusal; the tool's line. The gate: the crate built and
tested with the feature off and on, on both targets, and the
`pub fn` and `pub const` first lines listed with it off (the
part's calls absent) and on (present), as part 4d's P1 did; every
later log that runs the crate's tests runs them with every
feature on. The negative log through the binary: the sample on a
fixture from the template whose first sheet's last row is
1048576 (refused with the index message, a control). The run log:
`--richtext-sample` on both targets over the corpus, the checks as
"Harness runs", the keep and the edit open check, recorded as
`records/m3p2-p1-<utc-date>-<shortsha>.md` and `.txt`.

### P2: the comment runs

The doc commit (the P1 verification rulings); the carry, if any;
then `add_rich_comment`, `CommentEntry::runs` and
`--richcomment-sample`. Tests on byte-built parts: a rich comment
created in a part with none (the bytes checked against literal
bytes) and appended beside kept ones (the kept bytes identical);
the read of a part with runs (the corpus's shape, every run with
an `<rPr>`) through `comments` giving `runs` `Some` and the
flattened `text`, and of a plain `<t>` comment giving `None`;
the call-time refusals (no run, an empty run, 32768 flattened
characters, a bad font); a rich comment replacing a kept one;
`remove_comment` of a rich comment; the tool's line. The negative
log through the binary: the sample on the two 4d fixtures (a
comments part with no `legacyDrawing`, two `legacyDrawing`
elements), each refused, with a control. The run log:
`--richcomment-sample` on both targets, recorded as
`records/m3p2-p2-<utc-date>-<shortsha>.md` and `.txt`.

### P3: the catalogue and limits, the probes, the size record

The doc commit (the P2 verification rulings); the carry, if any;
then the `package` header's catalogue and limits for this part
(the messages marked by feature; the written shape; the runs
table's mechanism and its measured cost; what it never writes: a
run into the shared strings table, the five unmodelled
properties, a phonetic run; what the read drops); the two probes
with the size-probe manifest's features; the size log as "Size"
says, recorded as `records/size-m3p2-<utc-date>-<shortsha>.md`;
the negative log through the binary for every message of the part
reachable there, the unreachable ones stated with the test that
covers each; the three read runs of part 1 repeated as "Harness
runs" says, recorded in the run log; both samples again on both
targets with the size log's release binaries, byte-identical to
P1's and P2's runs by sha256, recorded as `records/m3p2-p3-
<sample>-<utc-date>-<shortsha>.md` and `.txt`. The open record of
the part's keep.

### Close-out push (after P3)

As part 1's: the self-scan rule on the push and ci-run scripts,
`ci-run.log`, then the post-stage `scan.log` over the pushed range
with `push.log` and `ci-run.log` as its siblings, then one record
commit holding the three logs, the `ANTHROPIC_API_KEY` check; the
push script's previous-record pattern naming part 1's record
commit 3cbd6fd (`^M3 part 1 close-out record`), its base part 1's
close-out doc commit 21b89bb resolved by its subject ("M3 part 1
close-out: commit planning docs (the P3 verification and rulings,
decision 6 deferred again)"), its tip this part's close-out doc
commit; pre mode on a clean tree; one independent check before
the report. The push carries 3cbd6fd and this part's commits, on
planning's word and the maintainer's authorisation typed in the
implementing session. Before the push, the Excel check for
Milestone 2 (every corpus row of every Milestone 2 sample, on the
laptop), deferred by the maintainer six times, last on 2026-10-02,
is run (Milestone 2's acceptance record then follows Milestone
1's form) or deferred again by a decision recorded here (decision
7). The push script prints no maintainer decision; the deferral's
record is this spec in the pushed range, and the record commit's
message names it.

## Planning rulings during P1 (2026-10-02)

Asked by the implementing session after the doc commit e11d0f5
and the carry commit e51ec15, before any feature code.

1. `Eq` on the three types that gain `runs`. `Cell`,
   `SharedStrings` and `CommentEntry` derive `Eq` today and keep
   it: the feature stays additive and no existing derive moves.
   `Run` and `RichText` therefore implement `Eq`, and honestly: a
   hand-written `PartialEq` for `Run` compares `text` and then
   the font field by field, each `f64` (`size`, a theme colour's
   `tint`) by `f64::to_bits`, so the relation is reflexive for
   every value the type can hold (a NaN size equals itself, and
   `0.0` differs from `-0.0`), and `impl Eq for Run {}` and for
   `RichText` (its `Vec<Run>`) are sound, not documented caveats.
   `Font`'s own derived `PartialEq` is untouched. A rustdoc line
   on `Run` says how it compares. Not taken: dropping `Eq` under
   the feature (a consumer's `Cell: Eq` would break when any crate
   in the build turns the feature on); a marker `Eq` over the
   derived float comparison (false for a hand-built NaN); runs
   kept outside `Cell` (the spec's field stands).
2. Readings accepted: `scheme val="none"` reads as `None`, as `u
   val="none"` does; `color` reads `auto` (`1` or `true`) as
   `Auto`, else `rgb` of exactly 8 hex digits as `Rgb`, `theme`
   with its `tint` as `Theme`, `indexed` as `Indexed`; a `color`
   with none of these, an `rgb` of another length, a `sz` or
   `tint` that does not parse or is not finite, or a `tint`
   outside `-1.0..=1.0` is refused with "a run's properties cannot
   be read", so every value the reader gives is one the write
   accepts. The implementing session's shape survey of the
   corpus's `<rPr>` children (shapes and counts only, no text)
   goes in P1's run log before the run.
3. Noted, no change: the two messages enter the catalogue in P1
   (the catalogue test requires it; P3 writes the limits); one
   run-container walk with a listener, the core's listener a
   no-op, the table's rich items in a sparse list sorted by index,
   nothing per plain item. The core's growth from the
   parameterised walk is what P3's size log prints; "Size"'s 2000
   threshold applies.

## Planning verification and rulings on P1 (2026-10-02)

Verified by planning from primaries: the doc commit e11d0f5 holds
this file at ae37a294, `specs/0023-scale.md` at edd14565 and
`ROADMAP.md` at 0850dc68, the hashes planning sent; the carry
commit e51ec15 changes `//!` lines alone (0 other lines); the
crate commit 1174bf1 and the tool commit 7d2f40d read in full by
planning; the record commit c53bbb3 adds the six logs and four
records; 5 of 5 messages ASCII, ending with the Co-Authored-By
line only; the logs at the reported hashes, each ending all as
expected, the scan RESULT: PASS, every log pinning ae37a294; the
compound-shape grep over the five logs 0; planning's own build at
7d2f40d in a detached worktree: 513 crate tests with the feature
off, 531 with it on and 745 with every feature, on both targets,
914 workspace tests on both, clippy (the crate with and without
features, the workspace) and doc 0 lines; the i686 release
`emend-copy` at the run log's hash a282e9f4 (the host's differs
by build path, as in every phase); planning's own run of
`--richtext-sample` with its own binaries over all 27 corpus
workbooks on both labels, 54 outputs and 54 tool lines
byte-identical to the run directory's; planning's recount of the
corpus's rich items over all 27 workbooks, 29 items and 113 runs,
equal to the run log's independent count; planning's own privacy
scan 3cbd6fd..c53bbb3, 5 commits, 1153 blobs, 0 findings;
`ANTHROPIC_API_KEY` unset.

1. Finding 1 is planning's error: the fact in "Facts planning
   checked" was counted with a glob that took 26 of the 27
   workbooks and missed ed2fbd304d51, the `.xltx`, so it named 7
   workbooks and 22 items where there are 8 and 29 (113 runs).
   The bullet reads as corrected above, with the recounted
   `<rPr>` sequences; the run log's count and the tool's
   `sst_rich_before` agree with planning's in 27 of 27. Recorded
   as a gotcha: a corpus count names its glob and checks its
   file count against the manifest's 27.
2. Finding 2 accepted: the write's font check runs on every font
   read, so a size of zero or below and an empty `rFont` are
   refused at the read too; that is ruling 2's principle, stated
   now for every field. Finding 3 accepted: one definition of the
   text message.
3. Finding 4, the `rm -rf` disclosure. Planning read every
   occurrence in the five logs: each removes a directory the
   script made in the same run (a `mktemp -d` directory, the
   trap's own, the gate's consumer crate and its target under the
   gate's own build root); nothing that existed before a run was
   removed. But the run log's change JI says "No `rm -rf`
   anywhere" while its printed script holds one (the counting
   controls' directory, line 1735), as the negative log's does
   (line 851); the report's sentence that both scripts were
   rewritten without it is wrong. Ruling: P1's logs stand as
   recorded. From P2 on, no script of this project contains
   `rm -rf` in any form, the maintainer's literal rule
   (2026-09-27): a directory a script made in its own run is
   removed by name with `rm -r`, traps included, or kept; the
   carried gate, test and scan scripts change so, the change
   named in their headers; and the scan log counts `rm -rf` over
   every log of the phase, expecting 0. Disclosed to the
   maintainer in planning's report; if the maintainer exempts a
   script's own temporaries, this ruling is amended here first.
4. Check observation O1, an empty run: ruled for P2's first code
   commit. An `<r/>`, and a run whose `<t>` texts total empty, is
   dropped at the read; a container whose every run is empty
   reads with `runs` `None` (its flattened text is empty and it
   holds nothing the write could carry). So every rich value the
   reader gives is one the write accepts, runs and fonts alike.
   A test on each container, and in P2 on a comment.
5. O2 and O3 accepted as the code does: a `tint` beside `rgb` or
   `indexed` is dropped from the typed view (the corpus has none;
   the write never writes one), and a `<t>` after the runs, which
   the schema does not allow, reads as a run of its own in its
   place, so the flattened text stays the runs' text. P3's
   catalogue names both beside the five dropped children.
6. O4, the run log's checks, for P2's run script: the summary
   requires every row ok (a refusal by the layer is a finding, not
   an accepted outcome); the survey checks the `val` shapes
   (flags, keywords, numbers) and the colour attribute forms,
   since P2's comment `<rPr>` is its subject; the survey script
   takes the corpus directory as `<corpus>` and hard-codes no
   location; `flagged` and `out_only` are checked against an
   expectation the log derives (the descriptor rows of the entries
   table; the appended elements per changed worksheet).
7. The tool commit's amend before any log ran (92f06df, the same
   change plus the two tool tests): recorded, no change, as
   `specs/0023` P3 ruling 3. The check's items with no primary:
   the pre-check hash a3df3f43 is printed by scan.log line 58; the
   rest are the implementing session's working state, labelled so
   in its report. The check's reservation on `ROADMAP.md` and
   `specs/0023`: their hashes in the doc commit are the ones
   planning sent, verified by planning. Accepted.
8. P1 is DONE. P2 is released: its doc commit carries this file
   with "Planning rulings during P1" and this section, and
   `ROADMAP.md`'s P1 row; its first code commit is ruling 4's
   drop of empty runs with its tests; its scripts follow rulings
   3 and 6.

## Decisions for the maintainer

Decided by the maintainer 2026-10-02: 1 to 6 as recommended (A in
each), in the reply "A on all (Recommended)" to planning's
question, typed in the planning session; 7 recorded.

1. Packaging: A (recommended), one feature `richtext` with two
   probes, the feature's own cost and the pair's cost over the
   `comments` probe; B, one probe with both features.
2. The written form: A (recommended), a rich value is always an
   inline string, never appended to the shared strings table, as
   every text write is today (no table rewrite, which would hold
   the table whole and rewrite its part on every write; finding
   2 of the decision record); B, appended to the table as
   openpyxl writes it, a new table rewrite in this part. Either
   way every reader, openpyxl included, shows the same text.
3. The run's font type: A (recommended), `styles::Font` reused,
   its `name` written as `rFont`, the five `rPr`-only children
   (`charset`, `outline`, `shadow`, `condense`, `extend`) not
   modelled and dropped from the typed view (the corpus's rich
   runs use `charset` 28 times and none of the other four, and
   Excel infers it from the font); B, a new `RunFont` type with
   all fifteen children, a second font writer and reader.
4. The read-back's placement: A (recommended), `Cell::runs` and
   `SharedStrings::runs_of` under the feature, with the contract
   that an item without runs costs nothing more, measured on tier
   d at P3 within 2048 kB of part 1's peaks; B, a separate
   `rich_shared_strings` read that parses the part a second time
   and holds the runs apart, the core's table untouched.
5. Comments: A (recommended), `add_rich_comment` beside
   `add_comment` under both features, and `CommentEntry::runs`;
   `Comment` stays `Copy` with its `&str`; B, `Comment::text`
   becomes an enum of plain or rich, a signature change to a
   shipped call.
6. Samples: A (recommended), two samples, one per capability and
   phase (`--richtext-sample`, `--richcomment-sample`); B, one
   sample doing both, one corpus run per phase fewer.
7. The Excel check for Milestone 2 (and now Milestone 3 part 1's
   fix): deferred by the maintainer on 2026-09-27, 2026-09-29,
   twice on 2026-09-30, 2026-10-01 and 2026-10-02 (`specs/0023`
   decision 6), to run before this part's close-out push at the
   latest or be deferred again by a decision recorded here.
   Recorded, not asked now; asked at the close-out.

## Out of scope

- Writing to the shared strings table (decision 2); a phonetic
  run (`<rPh>`, `<phoneticPr>`) written or typed; the five
  unmodelled run properties (decision 3); a rich text in a
  defined name, a header or footer, a table column, a validation
  prompt or a data bar (none of those elements takes runs in the
  format except headers and footers, which part 3 writes as
  their own codes).
- A rich formula string result (`t="str"` is a cached value, not
  written by emend); a rich value with a style on the cell (a
  style edit sits beside a cell edit as today).
- A streaming shared-strings resolver (part 5); rendering; what
  LibreOffice or Excel shows for a run beyond opening the file.
- The comment box, its size and visibility (part 6).
