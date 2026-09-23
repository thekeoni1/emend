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
cannot fit is an allocation failure the process reports as an error
on stderr with exit 2, as any other; none is expected on the corpus
and the runner's 2048 MiB.

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

## Out of scope

- Comparing values in the clear, or printing any value, hash or
  sample of one.
- Ordered or positional comparison; telling which element changed.
- Schema validation, or knowledge of which names matter.
- Parsing anything that is not XML: binary parts, VBA, media.
- Changing the runner, the entry rows, the flags or the exit status.
