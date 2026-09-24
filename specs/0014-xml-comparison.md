# 0014: Milestone 1 part 7, the names-only XML comparison

Status: APPROVED by the maintainer 2026-09-23 (planning draft the
same day), the four decisions under "Decisions for the maintainer"
each taken as planning recommended; sent to the implementing session
the same day.
Context: `CLAUDE.md` ("The fidelity harness is the acceptance
instrument"; "Corpus rules": cell contents never appear in records;
"Dependencies and licences"), `ROADMAP.md` ("Milestone 1", part 7;
"T4: moved into Milestone 1"), `specs/0000-step-zero-plan.md`
("Moved to Milestone 1: names-only XML comparison (outline)"),
`specs/0004-xlsx-diff.md` ("Output", "Exit status", "Streaming", the
no-leak test of P4), `specs/0006-passthrough.md` (the runner and the
record script), `specs/0011` to `specs/0013` ("Harness runs": the
run logs' own `ElementTree` comparisons, which this part replaces
with one tool), `specs/0001-bootstrap.md` ("How this task runs",
"Rules for every phase"). XML Namespaces 1.0 (names as namespace URI
plus local name; unprefixed attributes in no namespace) and XML 1.0
section 2.11 (line-end normalisation) are cited where the design
leans on them; no text is copied from them.

## Goal

The harness says, for every changed XML entry, what changed in
terms of the schema and never in terms of the content: which element
and attribute names occur a different number of times on the two
sides, by how many, and whether the values (attribute values and
text) match as a multiset, with the number that do not. So a record
can show that an edit touched only what the edit requires (a cell
sample adds two `c`, one `f`, one `row`; a style sample adds one
`xf` and one `numFmt`; a formula sample adds one `fullCalcOnLoad`
attribute) without any run log parsing XML in Python, and without a
name, a code, a formula or a cell's text ever reaching a record. The
comparison is streaming, pure Rust, in `xlsx-diff`, which stays
independent of emend (it never links the crate it measures) and
gains `quick-xml`, already in the tree at the pinned version.

The harness is the acceptance: over the corpus, part 6's
`--formula-sample` rerun with the new tool shows, on every workbook
with an A, the worksheet's name deltas exactly those the run log
derives from its own read of the input, the workbook part's one
attribute delta, and on the chained workbooks the content types' and
relationships' one element delta each; on the 8 without an A no
`xml` row at all; and a fixture test shows that a value planted on
one side never appears in the tool's output.

## Facts planning checked (2026-09-23)

Over the 27 corpus workbooks, by entry shapes and counts with
Python's `zipfile` and `expat`, never a value:

- 792 entries end in `.xml`, `.rels` or `.vml` (565, 216 and 11) and
  78 do not. Every one of the 792 parses; 158 start with a byte order
  mark; 748 declare UTF-8 (542 upper case, 206 lower) and 44 have no
  declaration; 10 hold a processing instruction after the
  declaration; none holds a comment, a CDATA section or a document
  type declaration.
- Together they hold 4997199 elements, 8579845 attributes, 2467742
  text nodes with content and 3516 whitespace-only text nodes. The
  largest entry is 63745347 bytes and holds 7670196 values (elements'
  attributes plus text nodes), so a per-side vector of 64-bit value
  hashes for it is about 61 MB, within the runner's 2048 MiB on
  i686.
- `quick-xml` 0.42.0's `NsReader` resolves element and attribute
  names to a namespace (`Bound`, `Unbound` or `Unknown`) and
  unescapes attribute values and text; `xlsx-diff` has `escape_name`
  for printing raw bytes and the P4 no-leak test as the pattern for
  the new one.
- The run logs of parts 4, 5 and 6 each carried their own Python
  comparison of the changed entries; the P2 run logs of parts 5 and
  6 also carried planning's independent one. This part makes the
  measurement the tool's, so a run log asserts deltas it reads from
  `xlsx-diff` rows against expectations it derives from the input.

## Design

### Crate

`tools/xlsx-diff` gains `quick-xml = "=0.42.0"` with default
features off, as the core pins it. No change to the core, to
`emend-copy` or to any probe, so no size record: the tool is not in
the budget. `xlsx-diff` never depends on `emend`.

### Which entries

An `xml` comparison runs for an entry present on both sides whose
status is `changed` and whose raw name, lower-cased, ends in `.xml`,
`.rels` or `.vml`. Nothing runs for `identical`, `recompressed`,
`dropped` or `added` entries, or for other names: those are what the
entry row already says. The comparison reads both sides again
through the zip readers, streaming with a 64 KiB buffer as the byte
comparison does, never holding an entry whole.

### What is compared

Each side is read with `NsReader` into one stream of events. From it:

- Element names: the multiset of resolved names, written
  `{namespace}local` (`{}` empty when unbound), counted per name.
- Attribute names: the multiset of `{namespace}local@{ans}attr`,
  counted per pair; an unprefixed attribute is in no namespace, as
  XML Namespaces says; `xmlns` and `xmlns:p` declarations are not
  attributes here, and neither side's prefix choice matters.
- Values: a 64-bit hash per attribute (of the element name, the
  attribute name and the unescaped value) and per text node with
  content (of the enclosing element name and the unescaped text;
  CDATA as text), collected into a vector per side. Whitespace-only
  text, the declaration, comments, processing instructions and a
  document type declaration are ignored. A value that fails to
  unescape (an entity the tool does not know) is hashed raw, and the
  entry's `parse` row counts it, so a record can see that it
  happened.
- Namespace resolution failures (`Unknown` prefix) are a parse error
  for that side.

The hash is `std`'s `DefaultHasher` with its fixed keys (SipHash
1-3), deterministic across runs and targets; it is never printed.
After both sides are read, each vector is sorted and merged once:
`in_only` is the count of hashes on the input side without a match
on the output side, `out_only` the converse, counted as multisets.
Two sides whose vectors are equal have both counts 0. The counts say
how many values differ, never which or what.

### Output

`xml` rows go after every `entry` row and before the `archive` rows,
in the entry rows' order, for each compared entry; a consumer selects
them by kind as it does the others. The name column is escaped as an
entry name is. Every count is `u64` in decimal.

| Row | Columns after `xml` and the name |
|-----|----------------------------------|
| `parse` | `ok` or `error` for the input; the same for the output; the byte offset of the error, empty when none; the count of values hashed raw on the input; on the output |
| `element` | the resolved element name; its count on the input; on the output |
| `attribute` | the resolved element name, `@`, the resolved attribute name; its count on the input; on the output |
| `values` | values on the input; on the output; `in_only`; `out_only` |

One `parse` row per compared entry, first. `element` and
`attribute` rows only for names whose two counts differ, sorted by
the name's bytes, `element` rows before `attribute` rows; no row for
a name with equal counts. One `values` row last, unless a side
failed to parse, in which case the entry ends at its `parse` row and
the name and value multisets are not printed for either side (a
half-read side would make them meaningless). An error's byte offset
is `quick-xml`'s buffer position; its message is not printed, since
`quick-xml` quotes the bytes it did not expect.

New `archive` rows, after `files_identical`: `xml_compared` (the
compared entries), `xml_parse_errors` (compared entries with an
error on either side), `xml_names_equal` (compared entries with no
`element` and no `attribute` row) and `xml_values_equal` (compared
entries whose `values` row has both counts 0).

Nothing else changes: the `entry` and `archive` rows of
`specs/0004` keep their columns and order, and the exit status is
unchanged (a compared entry is `changed`, so the status is already
1; a parse error is not an exit-2 error, since the archives were
compared and the row says what happened).

### Privacy

The rows print names and counts. An element or attribute name is a
schema's, and the corpus's XML is ECMA-376's and its producers'
extensions; the one family where a third party's schema could name
something is `customXml`, which emend never changes, so no row of
it is ever printed by an emend run (a decision below). Values are
hashed and the hashes are never printed. The no-leak test plants a
distinct sentinel value on each side in an element's text, an
attribute and a CDATA section, and asserts the sentinel's bytes are
absent from stdout and stderr, as P4's test asserts for names.

### 32-bit and streaming

Every count and offset is `u64`. The two name multisets are maps
from name bytes to counts, bounded by the schema, not the data. The
value vectors are the one allocation proportional to the data: 8
bytes per value, about 61 MB per side on the largest corpus entry;
the tool reads one side to its end, then the other, then sorts and
merges, and frees both before the next entry. An entry the vectors
cannot fit is an allocation failure that aborts the process as `std`
does, which the runner records as it records any killed process (P1
verification ruling 1); none is expected on the corpus and the
runner's 2048 MiB.

### Harness runs

`scripts/passthrough-record.py` gains a third table per label, "XML
comparison of changed entries", by part family: entries compared,
parse errors, names equal, values equal, and the sum of `in_only`
and of `out_only`; and the `.txt` companion gains the `xml` rows
with the columns above, prefixed by id and label as the entry rows
are. The runner is unchanged: the tool's stdout already goes to the
per-workbook diff file it reads.

The run log of P2 reruns part 6's `--formula-sample` with the new
`xlsx-diff` and, for every workbook, reads the tool's `xml` rows and
asserts them against expectations it derives from its own read of
the input (a `Number` cell with a value and no formula in the first
worksheet; a normal formula there; whether row `M + 1` exists; a
`calcChain` relationship), with a control per assertion:

- Without an A: no `xml` row for the workbook.
- With an A: the first worksheet's `element` rows are exactly `c`
  with a delta of +2, `f` with +2 (the replaced F keeps its `f`),
  `row` with +1 when row `M + 1` did not exist, `v` with -1 when F
  exists (F's cached value goes) and no other;
  its `attribute` rows are those of the new cells' `r` and the new
  row's `r` and `spans`-free start, and F's lost `t` when it had
  one, derived from the input; its `values` row has `out_only` equal
  to the new values written and `in_only` equal to the values
  replaced. The workbook part: no `element` row, one `attribute` row
  for `calcPr@fullCalcOnLoad` with 0 and 1, and `values` 0 and 1. On
  a chained workbook: the content types with `Override` -1, the
  workbook relationships with `Relationship` -1, their `values` rows
  `in_only` equal to the attributes each lost element carried.
- Every `parse` row `ok` on both sides with 0 raw values.

The expected deltas above are planning's reading of the sample; the
run log's derivation from the input is the source of truth, and a
workbook where the tool's rows differ from it is a finding, not a
reason to type the observed value.

The record is compared with part 6 P3's: the same entry statuses on
every workbook (the tool's `entry` rows are unchanged), with the new
table added.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m1p7-<phase>/` (`gate.log`, `test.log`
on both targets, `negative.log`, `run.log` in P2, `scan.log`); every
rule the earlier parts added, including the rulings of `specs/0013`
(the about line third; one spelling per build directory and one
header line per root; superseded run directories removed before the
run and printed; SERIALIZED lines with a process count; the command
that made each check-directory diff stated; a commit message that
states an unprinted measurement says so); the scan log last; at most
two independent checks; report; STOP. No push during the part; the
close-out push comes after P2 and carries the local commit 03669ca.

Part 7's first doc commit carries, unedited: `ROADMAP.md`,
`specs/0013-formulas.md` (the close-out rulings) and this spec.

### P1: the comparison in `xlsx-diff`

P1's code commit(s): `quick-xml` added to `tools/xlsx-diff`; the
comparison, the `xml` rows and the four `archive` rows; the module's
Limits (what is ignored, the hash, the memory bound, the parse-error
form). Tests on byte-built archive pairs, the rows compared with
rows the test writes by hand: an entry equal after recompression
(no `xml` row); a changed entry with one element added, one removed,
one attribute added, one attribute's value changed (names equal,
values 1 and 1); prefix choice differing with the same namespace
(no name row, values 0 and 0); attribute order differing (the same);
whitespace-only text and a comment and a processing instruction
added (the same); a value written as an entity on one side and a
literal on the other (the same); a CDATA section against escaped
text (the same); a carriage return as `&#13;` against a raw one
(values differ, since a raw one is normalised away, and the Limits
say so); an unbound name against a bound one (an `element` row for
each); an unknown prefix on one side (a `parse` row with `error` and
the offset, no `values` row, the archive counts); malformed XML on
the output side (the same, the entry's `entry` row unchanged); an
entity the tool does not know (hashed raw, counted); a `.vml` and a
`.rels` name compared and a `.bin` name not; a `.XML` upper-case
name compared; a `dropped` and an `added` XML entry with no `xml`
row; a compared entry with 100000 values on each side differing in
one (counts 1 and 1, the vectors freed: the test asserts nothing
about memory, the Limits state the bound); the no-leak test; the
`archive` rows on an archive pair with two compared entries, one
with a parse error; and exit status 1, never 2, for every case but
a genuinely unreadable archive. The negative log: the tool on a
malformed entry, an unknown prefix and a non-zip through the built
binary on both targets, the rows and the exit status printed. No
corpus run in P1.

### P2: the record script and the corpus run

P2's first doc commit carries the P1 verification rulings. Then the
record script's table and rows, with its own test on a fixture diff
file (the script's existing test pattern); the corpus run of
`--formula-sample` on both targets with the new tool, the run log's
assertions as "Harness runs" says, recorded as
`records/m1p7-p2-<utc-date>-<shortsha>.md` and `.txt`; the entry
statuses compared with part 6 P3's record by the run log.

### Close-out push (after P2)

As part 6's, with the close-out scripts as part 6 ran them, one
record commit after the push unless the maintainer chooses the split
again, the scan rerun last, the `ANTHROPIC_API_KEY` check. The push
carries 03669ca and part 7's commits, on planning's word and the
maintainer's authorisation typed in the implementing session.

## Decisions for the maintainer

1. Values compared as multisets of hashes, reporting the two
   one-sided counts (recommended). The alternative, an ordered
   comparison, would count every value after an insertion as
   changed, which says nothing about an edit; a single yes-or-no
   (one hash per side) would lose the count the outline asked for.
   The cost is one 8-byte hash per value in memory, bounded above.
2. Names printed for every compared family, `customXml` included
   (recommended). A name is a schema's, not a value, and emend never
   changes a `customXml` part, so no emend run prints one; a
   third-party tool comparing two arbitrary workbooks would, which is
   not a flow this project has. The alternative prints counts only
   for `customXml`; planning sees no privacy gain for an emend run
   and a loss of information for the harness.
3. Two phases and no size record (recommended). The part changes a
   tool only; the core, the probes and the budget are untouched, and
   the size log's rebuild of the probes would measure nothing new.
   The alternative adds a P3 with a size record for form's sake.
4. `quick-xml` in `xlsx-diff`, and `xlsx-diff` never linking `emend`
   (recommended; a statement more than a choice). The harness stays
   independent of what it measures; the crate it gains is the one
   the core already pins, at the same version, with the same licence
   already checked.

## Planning rulings during P1 (2026-09-23)

The doc commit 3c4e7de carries the three files at the hashes
planning sent. The implementing session asked eight questions
before code; planning checked each against quick-xml 0.42.0's
source (`reader/mod.rs`, `errors.rs`, `events/attributes.rs`) and
the tool's test helpers, and rules:

1. Row layout confirmed: `xml`, the escaped entry name, the kind
   word, then the kind's columns; an `attribute` row has one name
   column, `{ns}local@{ans}attr`. Element and attribute names are
   escaped as entry names are (`escape_name`), so a tab, a line feed
   or a byte outside the printable range never breaks a row; `{`,
   `}` and `@` print as themselves, and no namespace URI holds `}`
   and no local name holds `@` or `{`.
2. The `parse` row keeps one offset column: the input's offset when
   the input failed, else the output's; the two ok-or-error columns
   say which side. The raw-value counts count up to the failure on
   a failed side.
3. `xml_names_equal` and `xml_values_equal` count only entries that
   parsed on both sides; a parse-error entry counts in
   `xml_parse_errors` alone. "Output" is read with that correction.
4. The offset is `error_position()` for an error the reader returns
   and `buffer_position()` after the event for an unknown prefix, an
   attribute error and ruling 5's end of input.
5. quick-xml returns end of input without an error while elements
   are open (`MissingEndTag` comes only from `read_to_end`), so the
   tool counts open elements and reports end of input with one open
   as a parse error at the end position. It does not check that a
   root exists or that nothing stands outside it; the Limits say so.
   A fixture with an unclosed element joins the P1 tests.
6. Text nodes are assembled from contiguous text, CDATA and
   reference events, with comments and processing instructions
   transparent (`x<!--c-->y` is one node); the tool normalises CR LF
   and a lone CR to LF, CDATA included, and `&#13;` stays a carriage
   return, so the CR test in "### P1" reads: a raw CR against `&#13;`
   differs, a raw CR LF against a raw LF does not. Attribute values
   go through `normalized_value` for XML 1.0. Hashed raw means, for
   an attribute, the value's bytes as written and, for a text node
   with an unknown or invalid reference, the node with its known
   references resolved and each unknown one kept as written; each
   counts once. Two fixtures join the tests: the transparent comment,
   and CR LF against LF.
7. The hash feeds `DefaultHasher` with a tag byte, then each part's
   length as `u64` and its bytes, so the value is the same on both
   targets. A unit test pins the hash of one fixed value to a
   constant and runs on both targets, which is the only place a
   hash is ever written, and it is a test's, not a record's.
8. The test helpers accept the `xml` kind with per-kind widths and
   the 25 archive keys; existing tests whose changed `.xml` entry now
   prints `xml` rows keep their assertions, and the report names any
   that needed more than the helper change.
9. This section goes in a second doc commit before any code, as
   part 6's 81991c3 did, carrying this spec alone, unedited, at the
   hash planning's message gives.

## Planning verification and rulings on P1 (2026-09-23)

Verified from primaries by planning, not from the report: the five
commits 03669ca..5fa6c8f ASCII with the Co-Authored-By line alone
and no session link; the second doc commit b45c524 carrying this
spec at the hash planning sent; the record commit 5fa6c8f touching
exactly the six logs at the sha256 and byte counts the report
prints; planning's own gate (nine PASS), `cargo test --workspace` on
both targets (508 passed, 0 failed, 2 ignored on each), the crate
with the `dates` feature off and on, and clippy, all at 5fa6c8f;
planning's own `scripts/privacy.sh --range 03669ca..HEAD` reading 5
commits, 486 blobs, 0 findings, PASS; `xml.rs` and the `lib.rs`
change read in full against "What is compared", "Output" and
rulings 1 to 7; `cargo tree` of `xlsx-diff` built in planning's own
target directory showing `zip` and `quick-xml` and no `emend`;
planning's own no-leak run of that binary on two fictional archives
with sentinel values in an attribute, a text node and a CDATA
section on each side, the sentinels absent from stdout and stderr
and the rows as "Output" says; and planning's own run of that
binary over the 27 corpus inputs against part 6 P3's outputs, with
expectations derived from each input: on the 8 without an A no
`xml` row and exit 0; on the 19 the worksheet's `element` deltas
exactly `c` +2, `f` +2, `row` +1 where row `M + 1` was new and `v`
-1 where F exists, the `attribute` deltas exactly the new cells'
and row's `r`, A's and F's lost `t` and each attribute F's `<f>`
carried, the `values` counts equal to the derivation, the workbook
part's one `attribute` row and `values` 0 and 1, the content types
and workbook relationships on the 3 chained workbooks with one
element each fewer and `in_only` equal to the removed element's
attributes, every `parse` row ok with 0 raw values, and the four
`archive` rows as expected.

Findings:

- F1 (planning, on the spec, not the code): planning's first
  derivation missed the attributes a replaced formula's `<f>`
  carries (`aca` on the LibreOffice-produced files) and the `t` the
  producers write on every cell; the tool was right on all 27 once
  the derivation counted them. "Harness runs" is amended for P2: the
  worksheet's expected `attribute` rows are the new cells' and the
  new row's `r`, A's lost `t` when it had one, F's lost `t` when it
  had one, and each attribute F's `<f>` carried, all read from the
  input; `in_only` and `out_only` are derived from the same reading.

Rulings:

1. R1 (G1): the Limits wording stands and the spec is corrected:
   "32-bit and streaming" no longer claims an allocation failure
   exits 2. A vector that cannot grow aborts the process as `std`
   does, the runner records the outcome as it records any killed
   process, and no corpus entry approaches the bound. No
   `try_reserve`; no code change.
2. R2 (G6): the negative cases running the built binary on fixtures
   in a made scratch directory need no worktree or clone; planning's
   word is given for this phase and the ones that follow, and the
   departure line is no longer needed.
3. R3 (G7): from P2, every root a log uses has its own root line,
   `<scratch>/p1`-style script-and-build roots included.
4. R4 (G5): the two hash values in M9's failing assertion on i686
   are a fictional fixture's and a test's output quoted by a log;
   accepted, and ruling 7 is read as naming the source of the pin.
   No filtering.
5. R5 (F4): a text node a parse error interrupts is not hashed and
   its raw reference not counted, as the Limits say. Accepted.
6. R6 (G3): two fixtures join P2's first code commit: a carriage
   return and line feed inside an attribute value against a line
   feed (equal after attribute-value normalisation, as XML 1.0
   3.3.3 says, and the test asserts what quick-xml does) and an
   entry starting with a UTF-16 byte order mark (a `parse` row with
   `error`, since the tool reads UTF-8 only, which the Limits then
   say; the corpus has no such entry, "Facts").
7. R7 (G4): the case map pairs the spec's parenthetical with the
   single-attribute-change fixture and lists the four-at-once
   fixture as its own case; corrected in P2's test.log script.
8. P1 is DONE. P2 is next: its first doc commit carries this spec
   and `ROADMAP.md` unedited, as part 6's 463ccad did; then ruling
   6's fixtures, the record script and the corpus run as "### P2"
   and the amended "Harness runs" say.

## Planning verification and rulings on P2 (2026-09-23)

Verified from primaries at the record commit a39c3cd (phase commit
ea37ad7): the five commits and their diffs; the doc commit's two
blobs at the hashes planning sent (spec 56db12e0..., `ROADMAP.md`
7ba4353b...); every hash the report cites against the committed
files; planning's own gate (nine PASS), workspace tests on both
targets (510 passed, 2 ignored, p7 17 on each), clippy, and privacy
scan over the five commits (491 blobs, 0 findings); planning's own
comparison over the run directory: on both labels the tool's `xml`
and `archive` rows agree with planning's derivation on all 27
workbooks (19 with an A, 8 with no row), the run directory's diff
file equals the stdout of planning's own P1 build of the tool on
every workbook, host and i686 diff files are byte-identical, and
every output is byte-identical to part 6 P3's; planning's own read
of the record pair: the entry table is byte-identical to part 6
P3's (1740 rows), the `.txt`'s third table holds exactly the 524
`xml` rows of the diff files (as a multiset), and planning's own
tally by family reproduces every row of both `.md` tables (44, 0,
0, 0, 104, 146 in total; content-types 6 and 0, rels 9 and 0,
workbook 0 and 19, worksheet 89 and 127 one-sided). Nothing is
pushed: main is eleven commits ahead of origin/main, the first of
them 03669ca. No API key is set in planning's environment.

Rulings:

1. The doc commit 4cb3c6a is as ruling 8 asked. The three code
   commits do what "### P2" and ruling 6 say: bacffa3 the two
   fixtures and the "UTF-8 only" Limits line; 04ba408 the third
   table, the `.md` table per label and the read filter (rows with
   outcome `ok` and diff exit 0 or 1, the only rows whose diff file
   is the tool's); ea37ad7 a two-line refusal of a repeated `xml`
   archive row, a defect the negative log found in 04ba408 and B3m
   now measures. Accepted, including the filter and the refusal.
2. F2: the record script's "existing test pattern" is the negative
   log's family B, as T3 P1 and part 1 P2 tested it; B1 compares
   the `.txt`'s third table and both `.md` tables with a derivation
   over fixture diff files, content included. The spec's wording is
   satisfied.
3. F1 and G1 (no row-order check in the record script): the spec
   asks for none; the tool's row order is P1's tested behaviour,
   and a diff file not of the tool's making stops the script with a
   nonzero exit, so no record is written from one. No change.
   Noted in the passthrough backlog of `ROADMAP.md` ("Unscoped").
4. G3 (five superseded hashes typed in rerun lines, not printed by
   a command): a gap, accepted as recorded. The files are outside
   the repository, committed nowhere, and planning did not find
   them on disk; the typed hashes stand as the pointers they are,
   and a rerun to print them would only add a further attempt. From
   part 8: a rerun line names only hashes the command under it
   prints, or marks a hash "typed from the predecessor's rerun
   line".
5. G4 (the run log's `.txt` check counts rows, its `.md` check
   compares the total row only): accepted for P2, since planning's
   own read above compared the content and every family row. From
   part 8: the run log's record check compares the third table's
   content and each family row, as B1 does on fixtures.
6. G5 (stale script comments and one stale printed line in
   test.log; header items 9 before 8 in run.log): accepted as
   recorded; no rerun for text alone.
7. G6: sourcing the committed `scripts/counting.sh` from the main
   tree and running a `git show` copy of the record script are
   within P1 ruling 2, which meant that nothing in the main tree is
   executed as the thing under test; the gate's status check shows
   the tree held nothing but the phase's own files. The root line's
   "nothing in it is run" is wrong as worded; from part 8 it says
   "sourced" where a library is sourced.
8. G7: "keeping the last" in ea37ad7's message is a reading of the
   code (a later assignment to the same key), not a measurement,
   and with equal values it cannot be one. Accepted as stated;
   B3m prints the acceptance, which is the claim that mattered.
9. G8: the failed rerun attempt's empty test.log is named as empty,
   which is why it has no hash worth printing; its stderr is
   hashed. Accepted.
10. G2 and G9 to G14 (report statements not printed by a primary):
    each was confirmed by planning from primaries on disk (the read
    filter from the diff; the run directory listing; origin/main;
    the outputs' identity). Accepted as the report's inferences,
    marked as such.
11. The record pair is named by the UTC date the run ran on
    (2026-09-24), one day after the phase's other dates; the stem
    rule in "### P2" is followed. `ROADMAP.md` cites both.
12. P2 is DONE and part 7's work is complete. The close-out is
    READY: planning's word for the push is given here; it carries
    03669ca and part 7's ten commits, on the maintainer's
    authorisation typed in the implementing session, as "Close-out
    push (after P2)" says, with the record commit after the push,
    the scan rerun last and the `ANTHROPIC_API_KEY` check. The
    close-out doc commit carries this spec and `ROADMAP.md`
    unedited, at the hashes planning sends.

## Planning verification and rulings on the close-out (2026-09-23)

Verified from primaries: origin/main is 4bc741e, twelve commits
after 6d2dc62, the first 03669ca; the doc commit carries the spec
and `ROADMAP.md` at planning's hashes (af420101..., ee4528d6...)
and nothing else; every commit in the range has the Co-Authored-By
line only, no session line, no non-ASCII byte, one author and
committer identity (the GitHub no-reply one); the record commit
9232e61 is local, one ahead; the three logs hash as the report
says; push.log shows `ANTHROPIC_API_KEY` unset, the hook equal to
what the installer writes, twelve commits scanned with no finding,
remote main equal to local main and the repository PRIVATE;
ci-run.log records run 35941782215 on 4bc741e, a push to main,
success, eight gate steps PASS and privacy skipped in CI mode;
scan.log PASS over the range plus the two logs. Planning confirmed
the run's head, conclusion and event and the repository's
visibility through `gh` itself.

Rulings:

1. The close-out is as "Close-out push (after P2)" says: part 6's
   scripts with names, spec and base pattern changed. Accepted.
2. Part 7 is DONE and PUSHED. 9232e61 goes with the next push.
   Part 8 is next; its spec is planning's next draft, for the
   maintainer's approval before any handoff.

## Out of scope

- Comparing values in the clear, or printing any value, hash or
  sample of one.
- Ordered or positional comparison; telling which element changed.
- Schema validation, or knowledge of which names matter.
- Parsing anything that is not XML: binary parts, VBA, media.
- Changing the runner, the entry rows, the flags or the exit status.
