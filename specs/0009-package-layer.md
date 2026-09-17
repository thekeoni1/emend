# 0009: Milestone 1 part 2, the package layer

Status: approved by the maintainer 2026-09-17 (planning draft the
same day). P1 READY; P2 and P3 READY; the close-out push PLANNED.
Context: `CLAUDE.md` (architecture, dependencies, 32-bit and sizes),
`ROADMAP.md` ("Milestone 1"), `specs/0008-zip-layer.md` (the zip
layer's API, the harness expectation for a replaced entry, "Size" with
the maintainer's budget ruling), `specs/0004-xlsx-diff.md` (statuses
and flags), `specs/0006-passthrough.md` (the runner, `--tool-arg`, the
record script), `specs/0001-bootstrap.md` ("How this task runs", "Rules
for every phase").

## Goal

The core's second layer, in two modules. `xml` is a streaming reader
over a part's bytes that hands back every token together with the
exact input bytes it covers, so a rewrite copies what it does not
change and writes only what it does. `package` is the Open Packaging
Conventions layer: it finds parts through `[Content_Types].xml` and the
`.rels` parts, maps the workbook's sheets to their worksheet parts, and
edits content types and relationships. Nothing here reads a cell. The
harness is the acceptance: a null rewrite of every XML part across the
corpus comes back with the same bytes (`recompressed` or `identical`,
never `changed`); the package map resolves every relationship and
content type in the corpus; and the first real edit, dropping a
workbook's calculation chain, changes exactly the three entries it
must and opens in LibreOffice.

## Facts planning checked (2026-09-17)

- `quick-xml` 0.42.0 with `default-features = false` depends on
  `memchr` alone (`Cargo.lock`; both are already in the workspace tree
  through `size-probe`, gate PASS since T3 P2). Licence MIT; `memchr`
  is Unlicense OR MIT.
- With no features the crate's reader assumes UTF-8, strips a leading
  UTF-8 byte order mark silently, does not count it in
  `buffer_position()` and never emits it as an event
  (`reader/buffered_reader.rs`, `remove_utf8_bom`; the crate's own
  `bom_from_reader` test). An event round trip through the crate's
  writer would therefore drop the mark. `buffer_position()` is the
  byte position just after the last emitted event, counted from the
  byte after the mark.
- `BytesStart` keeps the raw bytes between `<` and `>` (a space before
  `/>` included); text is raw and unescaped unless asked; general
  references (`&amp;` and the like) are events of their own in 0.42;
  `escape` and `unescape` work on `str`.
- Corpus, 27 workbooks, by entry names and attribute shapes only: 781
  entries named `.xml` or `.rels` (565 and 216), 158 of them starting
  with a byte order mark, 41 with no XML declaration, none with bytes
  after the root element; the largest XML part is 63745347 bytes.
  `_rels/.rels` names the workbook `xl/workbook.xml` in 17 files and
  `/xl/workbook.xml` in 10. Of 878 relationships 234 are
  `TargetMode="External"` (all hyperlinks); of the 644 internal
  targets 435 are absolute (leading slash), 155 relative and 54 start
  with `../`; joined to the source part's directory every one resolves
  to an entry by exact bytes, none carries `%` or `&`, and none needs
  a case-insensitive match. Every relationship id has the form
  `rId<digits>`. Every one of the 614 `Override` part names resolves
  exactly, and under the rule below every entry of every file has a
  content type. `Default` elements cover xml, rels, png, jpeg, bin,
  vml, svg and data; ten files write `<Default ... />` with a space
  before the slash, seventeen `<Default .../>`. Every content types
  root, relationships root and workbook root carries the transitional
  namespace; every workbook binds `r` to the relationships namespace
  on its root and its 95 sheets write `r:id`; 48 sheets carry `state`;
  72 worksheets have a rels part; 9 workbooks carry `xl/calcChain.xml`
  with a relationship to it; every workbook has `xl/sharedStrings.xml`
  and `xl/styles.xml`.
- The runner keeps the tool's stdout per workbook and target as
  `stdout.<label>.txt` in the run's per-id directory, so a measurement
  the tool prints can be aggregated by the phase's run log by id.
- `Archive::read` builds the `zip` crate's archive by moving the
  layer's reader in, and `write_to` takes the reader for the whole
  write, so a replacement cannot read the archive while the write runs
  (`emend-copy --replace` extracts to a temporary file first). Part 2
  keeps that shape: an edited XML part is rewritten to a temporary
  file before the write and handed to the layer as a replacement.

## Design

### Crate

`quick-xml = { version = "=0.42.0", default-features = false }` joins
`crates/emend`'s dependencies; nothing else does. Two residues of part
1 go into P1's first code commit: the doc sentence in
`probe-emend-copy.rs` that calls its delta over `probe-zip-xml` the
budget's figure (the budget is judged on `probe-emend-edit-xml`), and
the duplicate-name candidates comment, which part 2's size script does
not carry since it measures no candidates.

The byte-level archive builder and the independent parser of
`archive/tests.rs` (`Spec`, `Build`, `parse_independently` and their
helpers) move unchanged into `crates/emend/src/testing.rs`, a
`#[cfg(test)]` module, so the package tests build their fixtures and
read their outputs through code that shares nothing with either layer.
The move is P2's first code commit and changes no test.

### Module `xml`

- `Stream<R: Read>`: `new(reader, index) -> Result<Self, Error>`
  reads the first three bytes and reports a UTF-8 byte order mark as
  `bom() -> bool`; the WHOLE input, mark included, is fed to
  `quick-xml` through a 64 KiB `BufReader` over a tee that keeps a
  copy of every byte the parser consumes until the caller has seen it.
  The crate drops a leading mark without counting it, so a parser
  position `p` is the input position `origin + p`, `origin` being 3
  when `bom()` is true and 0 otherwise (P1 ruling 1 below, which
  replaced the draft's own strip of the mark). `index` is the entry's
  index, carried into errors. `next(&mut self) -> Result<Option<Token
  <'_>>, Error>` gives the next token, or `None` at the end of the
  input. `Token { event: Event<'_>, raw: &[u8] }`: the event as the
  crate parses it, and the input's own bytes from the end of the
  previous token to the crate's position after this one, taken from
  the tee and never re-serialised. Consequence, and P1's acceptance:
  the mark when present, followed by every token's `raw` in order, is
  the input byte for byte. `xml::BYTE_ORDER_MARK`, the three bytes, is
  public and re-exported, since any rewriter must re-emit the mark
  `bom()` reports (P1 ruling 6).
- Memory: the tee holds at most the reader's read-ahead plus the
  current token. A token is one tag, one text run, one general
  reference, one comment, one CDATA section, one processing
  instruction or one document type declaration, so the bound is the
  largest single token in the part, never the part. Every position and
  count is `u64`.
- The parser runs with the crate's defaults: no trimming, end names
  checked, empty elements not expanded, a dangling `&` refused. A
  parse error is `Error::Xml(index)`; the position is not reported,
  since the bytes near it are content.
- `Scope`: the prefix stack the package layer resolves names through.
  It records every `xmlns` and `xmlns:<p>` declaration on each start
  tag with the depth it was declared at and drops them at the matching
  end tag (an empty element's declarations apply to itself alone).
  `resolve_element(name) -> (Option<&[u8]>, &[u8])` gives the
  namespace and the local name; `resolve_attribute` the same, an
  unprefixed attribute having no namespace. A prefix no ancestor binds
  is `Error::Package("an element or attribute uses a prefix its
  document does not bind")`. The null rewrite never resolves a name.
- Escaping: attribute values the package layer writes are escaped with
  the crate's `escape` (`&`, `<`, `>`, `"` and `'`); values it reads
  are unescaped with its `unescape_value`. Both need `str`, so a value
  the layer must interpret that is not UTF-8, or that holds a
  reference the crate does not know, is `Error::Xml(index)`.

### Module `package`

- `Package<R: Read + Seek>`: `open(reader) -> Result<Self, Error>`
  opens the archive and parses nothing. `archive() -> &Archive<R>`;
  `part(name: &[u8]) -> Option<usize>` (exact bytes, as
  `Archive::index_of`); `read_part(index) -> Result<impl Read + '_,
  Error>`.
- Content types: `content_types(&mut self) -> Result<&ContentTypes,
  Error>`, parsed from `[Content_Types].xml` on the first call and
  kept. `Default { extension, content_type }` and `Override {
  part_name, content_type }` in document order, bytes as written (the
  part name keeps its leading slash). `ContentTypes::of(&self, name:
  &[u8]) -> Option<&[u8]>`: the Override whose part name is `/`
  followed by the entry name, byte for byte; else the Default whose
  extension equals the bytes after the name's last `.` compared ASCII
  case-insensitively (the convention makes extensions
  case-insensitive; the corpus writes them in lower case); else
  `None`. No `[Content_Types].xml` entry is `Error::Package("no
  [Content_Types].xml part")`; a root other than `Types` in
  `http://schemas.openxmlformats.org/package/2006/content-types` is
  `Error::Package("a content types part whose root is not Types")`.
- Relationships: `relationships(&mut self, source: &[u8]) ->
  Result<&Relationships, Error>`, the relationships of the part
  `source`, or of the package when `source` is empty, parsed on the
  first call for that source and kept. The rels part of `<dir>/<base>`
  is `<dir>/_rels/<base>.rels`, the package's is `_rels/.rels`; an
  absent rels part gives an empty set, since most parts have none.
  `Relationship { id, kind, target, external: bool }` bytes as
  written, `kind` the `Type` attribute in full. `Relationships::by_id
  (&self, id) -> Option<&Relationship>`; `of_kind(&self, kind) ->
  impl Iterator<Item = &Relationship>` on the full URI;
  `target_part(&self, rel) -> Result<Vec<u8>, Error>`: for an internal
  relationship, the entry name: the target without its leading slash
  when it has one, else the target joined to the source's directory,
  with `.` and `..` segments resolved; `..` above the root, or an empty
  result, is `Error::Package("a relationship target outside the
  package")`; an external relationship is `Error::Package("an external
  relationship has no part")`. No percent-decoding and no case folding
  (the corpus needs neither; Limits). Two relationships with one id
  are `Error::Package("two relationships with the same id")`; a root
  other than `Relationships` in
  `http://schemas.openxmlformats.org/package/2006/relationships` is
  `Error::Package("a relationships part whose root is not
  Relationships")`.
- Namespaces the layer knows, each in its transitional and its strict
  form, looked up in that order: the main spreadsheet namespace
  (`http://schemas.openxmlformats.org/spreadsheetml/2006/main`,
  `http://purl.oclc.org/ooxml/spreadsheetml/main`); the relationships
  namespace of the `r:id` attribute (`http://schemas.openxmlformats.org/
  officeDocument/2006/relationships`, written here across two lines,
  and `http://purl.oclc.org/ooxml/officeDocument/relationships`); and
  the relationship types `officeDocument`, `worksheet`, `chartsheet`,
  `dialogsheet`, `sharedStrings`, `styles`, `calcChain` and `theme`,
  each the relationships namespace followed by `/` and the name.
- The workbook: `workbook(&mut self) -> Result<&Workbook, Error>`.
  The package's officeDocument relationship gives the workbook part
  (`Error::Package("no officeDocument relationship")`, `("more than
  one officeDocument relationship")`); its root must be `workbook` in
  the main namespace (`Error::Package("a workbook part whose root is
  not workbook")`). `Workbook { part, sheets: Vec<Sheet>,
  shared_strings, styles, calc_chain, theme: Option<Vec<u8>> }`, the
  four options the resolved part names of the workbook's relationships
  of those kinds, the first in document order when there are several.
  `Sheet { name: String, sheet_id: u32, rel_id: Vec<u8>, state:
  Option<Vec<u8>>, part: Vec<u8>, kind: SheetKind }` in `<sheets>`
  order: `name` unescaped; `state` as written; `rel_id` the attribute
  with local name `id` in the relationships namespace, resolved through
  the workbook's relationships to `part`; `kind` from that
  relationship's type: `Worksheet`, `Chartsheet`, `Dialogsheet` or
  `Other`. A sheet lacking `name`, `sheetId` or the id attribute is
  `Error::Package("a sheet element lacks name, sheetId or r:id")`; a
  `sheetId` that is not a decimal `u32` is `Error::Package("a sheetId
  that is not a number")`; an id with no relationship is
  `Error::Package("a sheet's relationship is missing")`.
  `Workbook::sheet(&self, name: &str) -> Option<&Sheet>` matches the
  unescaped name exactly. Everything in the workbook part other than
  `<sheets>` and its `<sheet>` children is skipped.
- Edits, recorded on the `Package` and applied by `write_to`:
  `remove_relationship(source, id)`, `add_relationship(source, kind,
  target, external) -> Vec<u8>` (the new id), `remove_override
  (part_name)`, `add_override(part_name, content_type)`,
  `remove_part(name)`, `replace_part(name, Replacement)` and
  `add_part(name, Replacement)` (the entry alone; the caller adds an
  override or relies on a Default). Nothing is removed implicitly: a
  removed relationship's target, a removed part's own rels part and
  its override are each their own call, so the caller says exactly
  what changes and the harness checks exactly that. The recipe P3's
  tool applies, dropping the calculation chain: `remove_relationship`
  of the workbook part's calcChain relationship, `remove_override` of
  its target when an override for it exists, `remove_part` of the
  target when the entry exists.
- `write_to(&mut self, out: impl Write) -> Result<(), Error>`: every
  edit is validated first, so a refused set writes nothing: removing an
  id or an override that is not there (`Error::Package("no such
  relationship")`, `("no such override")`), adding an override for a
  part name that has one (`("an override for that part already
  exists")`), editing a part also removed (`("an edit to a removed
  part")`), a part name or value that is not UTF-8 (`Error::Xml`).
  Then, for `[Content_Types].xml` and each rels part with edits, the
  part is read through `Stream` and written to a temporary file (the
  layer's `TempFile`, made `pub(crate)`): every token's `raw` is
  copied except a removed element's, whose own bytes alone are dropped
  (surrounding whitespace stays), and additions go immediately before
  the root's end tag as `<Override PartName="..." ContentType="..."/>`
  or `<Relationship Id="..." Type="..." Target="..."/>`, with
  ` TargetMode="External"` before the slash when external, values
  escaped, no whitespace added. A relationship added to a part with no
  rels part creates one: `<?xml version="1.0" encoding="UTF-8"
  standalone="yes"?>` then `<Relationships xmlns="...">`, the new
  elements, `</Relationships>`, added as an entry, its content type
  coming from the package's Default for `rels`
  (`Error::Package("no Default covers rels")` otherwise). Ids:
  `rId<n>`, `n` one more than the largest among the part's ids of the
  form `rId<digits>`, `rId1` when there is none; an id of another form
  is never generated and never collides. The temporary files become
  the entries' replacements and `Archive::write_to` runs with the
  assembled `Edits`; the temporaries are removed however the write
  ends.

### Errors

`Error` gains `Xml(usize)`, an entry whose bytes the layer cannot read
as XML (not well-formed; not UTF-8 anywhere, since the crate's reader
decodes text and markup as `str` (P1 ruling 2); an unknown reference
in a value the layer interprets), carrying the entry's index and never
a position or content; and `Package(&'static str)` with the messages
above. Every message goes into the module catalogue in the docs with
its test, or marked untested with the reason, as part 1 P3 did.

### 32-bit and streaming

As part 1: every offset and count is `u64`; the tee's bound is the
largest token; an edited part streams through a temporary file, never
memory; P1's corpus run passes the largest corpus part (63.7 MB)
through `Stream` on i686.

### Size

`tools/size-probe` gains two probes, built as the others are (the
`size` profile, i686 musl, the features the manifest names):

- `probe-zip-xml-write` (feature `xml`): `probe-zip-xml`, then one
  archive with one deflated entry written through the `zip` crate's
  `ZipWriter` to a second path, the entry's bytes being those it read.
  The deflate encoder and the crate's writer are thus in this baseline,
  which is what the maintainer's ruling asks for: an emend probe
  measured over it shows the layer's own code apart from the
  compressor's.
- `probe-emend-package-xml` (features `emend` and `xml`): `<archive>
  <output> <part to replace> <override part name> <content type>`.
  It opens with `Package`, resolves the workbook and prints the sheet
  count, drops the calculation chain when the workbook has one (the
  recipe), adds the override named on its command line and a
  relationship of kind `theme` to that part name from the workbook
  part, replaces the named part with itself passed through `Stream`
  (mark and every `raw` copied), writes the package, then makes the
  `quick-xml` element count `probe-emend-edit-xml` makes over the
  replaced part's bytes. It is the consumer the budget describes with
  the whole core to date linked.

P3's `size.log` prints, for all seven probes, the file size, the
section total and `.text` from `size -A`; compares the five earlier
probes with `records/size-m1p1-2026-09-17-6cec416.md` and classifies
every difference as the part 1 P3 repair-round ruling says (build id,
panic-location line numbers, or code, the last with the commit and the
lines that caused it); and gives three figures in section bytes with
the file delta beside each: the budget figure, `probe-emend-package-
xml` over `probe-zip-xml`, against 256000; the same probe over
`probe-zip-xml-write`, the layer's cost to a consumer that already
links the writer; and part 2's own cost, `probe-emend-package-xml`
over `probe-emend-edit-xml`, which the checkpoint judges. The record
`records/size-m1p2-<utc-date>-<shortsha>.md` carries those figures
and a note on a stored-replacement mode: what a consumer accepting
larger files would skip (the encoder's cost, `probe-zip-xml-write`
over `probe-zip-xml`, measured here), what the layer would need (a
replacement written with method 0, which the writer's header and
descriptor code already handles), and that it is a proposal for the
maintainer, not work in this part.

Checkpoint (the maintainer's ruling in `specs/0008-zip-layer.md`,
"Size"): if part 2's own cost is more than about 40 KB of sections,
the maintainer chooses between a higher ceiling (384 KiB was named)
and Cargo features off by default for the later parts. P3's report
states the figure and asks.

### Harness runs

`tools/emend-copy` gains three operations, at most one per invocation,
given after the two paths as `--replace <name>` is: `--xml-null-
rewrite` (P1: every entry whose name ends in `.xml` or `.rels`, ASCII
case-insensitively, is replaced by its own bytes passed through
`Stream`, mark and every `raw` copied); `--survey` (P2: no edit, the
output byte-identical; one line on stdout, the word `survey` then
tab-separated `key=value` pairs, counts only); `--drop-calc-chain`
(P3: the recipe when the workbook has a calcChain relationship,
otherwise no edit). The runner and the record script are unchanged;
each run names its operation through `--tool-arg` and the record's
`Tool arguments:` line.

The survey's keys: `entries`; `typed` and `untyped` (every entry other
than `[Content_Types].xml`, by `ContentTypes::of`); `defaults`,
`overrides`; `rels_parts` (entries named as a rels part, each parsed
for the source its name derives); `relationships`, `external`,
`resolved`, `unresolved` (internal targets that do and do not resolve
to an entry by exact bytes) and `case_only` (unresolved ones, and
override part names, that match an entry only after ASCII case
folding, so the exact-bytes rule is measured rather than assumed);
`sheets`, `sheets_resolved`; `shared_strings`, `styles`, `calc_chain`,
`theme` (0 or 1). An error from any parse is the tool's exit 2 with
the message, so the runner records it as an error outcome.

Expected results:

- P1: every `.xml` and `.rels` entry `recompressed` or `identical`,
  never `changed`, with `descriptor` alone allowed as `specs/0008`
  says for a replaced entry (an entry whose recompressed bytes equal
  the input's is `identical` and unflagged); every other entry
  `identical` unflagged; `lo` and `lo_in` `ok`; diff exit 0 or 1. The
  run log counts the XML-named entries from the diff rows and asserts
  every one has one of the two statuses.
- P2: every entry `identical` unflagged, `files_identical` `yes`, diff
  exit 0, `lo` and `lo_in` `ok`; the run log aggregates the survey
  lines per id and in total, expecting `untyped` 0, `unresolved` 0,
  `case_only` 0, `sheets_resolved` equal to `sheets` in every file,
  `shared_strings` and `styles` 1 in every file, `calc_chain` 1 in
  nine, and the totals of the facts above (781 XML-named entries are
  not a survey key; 878 relationships, 234 external, 644 resolved, 95
  sheets, 216 rels parts, 614 overrides are). A departure from a fact
  is a finding about the corpus or the rule, recorded, and planning
  rules which.
- P3: in each of the nine workbooks with a chain, the chain's entry
  `dropped`, the workbook's rels part and `[Content_Types].xml`
  `changed`, each with `descriptor` alone allowed as above, every
  other entry `identical` unflagged, `order_preserved` `yes`,
  `files_identical` `no`, `lo` `ok`; in each of the eighteen without,
  every entry `identical` unflagged, `files_identical` `yes`. The run
  log also shows, for one dropped case per target, the element counts
  of the two changed parts in input and output (`Override` and
  `Relationship` elements, each one fewer), counted on the extracted
  parts and printed as counts only.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit, the
code commit(s), one record commit with the logs and the check answers;
logs in `records/logs/m1p2-<phase>/` (`gate.log`, `test.log` on both
targets, `negative.log`, `run.log`, `scan.log`, and `size.log` in P3);
the first lines of every log as every phase has written them; every
compound check a named function; the scan log last, hashing every
committed sibling and grepping every printed script for the compound
shape; at most two independent checks; report; STOP. No push during
the part; the close-out push comes after P3, and the local record
c414445 goes with it.

### P1: quick-xml joins; the token stream; the null rewrite

The dependency; `xml::{Stream, Token}`; `emend-copy
--xml-null-rewrite`; the two residues, all in P1's code commit(s).
Unit tests on byte-built parts, no fixture files: the reconstruction
property on a part holding every form (a declaration; a comment; a
processing instruction; a document type declaration; a CDATA section;
whitespace-only text; text with the five predefined references and
decimal and hexadecimal character references; attributes in both quote
kinds holding references; `<a/>` and `<a />`; nested elements; a text
run and an attribute value each longer than 64 KiB; CRLF line ends;
non-ASCII UTF-8 text), each with and without a byte order mark, with
and without a declaration, with and without whitespace after the root;
a part starting with two marks, the second coming back inside a text
token (P1 ruling 1); an empty part (no tokens, empty output); each
token's `raw` beginning where the previous ended, checked by position;
a part that is not well-formed (a tag with no `>`, a mismatched end
tag, a `<` in text with no later `>`, a dangling `&`; the readings of
P1 ruling 2) as `Error::Xml` with the entry's index; a part with a
byte that is not UTF-8 inside otherwise well-formed text as
`Error::Xml`; a binary part, meaning bytes that are not valid UTF-8
(a PNG header, a high-byte blob, a lone surrogate, an overlong NUL),
as `Error::Xml`; a part of valid UTF-8 holding characters XML forbids
(NUL and one other C0 control, in text) accepted and reconstructed,
pinning the fourth Limits entry (P1 ruling 4); the tool's operation
applied to a byte-built archive of three XML parts and one binary
part, each deflated XML part's CRC-32 and uncompressed size equal to
the input's, a stored XML part's raw bytes equal, and the binary entry
untouched, asserted by the independent
parser. Tests on both targets. The corpus run on both targets with the
LibreOffice check, recorded as `records/m1p2-p1-<utc-date>-
<shortsha>.md` and `.txt`; acceptance as "Harness runs". The gate's
licences step with `quick-xml` in the core's tree. `negative.log`: the
tool's refusals (two operations at once, an unknown operation, an
operation before the paths) and the runner's refusal of a tool
argument holding a tab.

### Planning rulings during P1 (2026-09-17)

Both questions the implementing session raised after the doc commit
0e6c60d came with measurements; planning reproduced every figure in a
scratch crate on `quick-xml` =0.42.0 with no features before ruling,
and read the mechanisms in the crate's source.

1. The byte order mark. The crate strips a leading mark in its `Init`
   state only, once, by consuming from the `BufRead` directly, so the
   three bytes never reach `buffer_position()`
   (`reader/buffered_reader.rs`, the `XmlSource` impl for `R:
   BufRead`; `reader/mod.rs`, `ParseState::Init`). The draft had the
   stream strip the mark itself and feed the rest, which loses three
   bytes on a part that starts with TWO marks: the crate strips the
   second one uncounted, and no token's `raw` holds it. Measured, as
   (bytes fed, final parser position, bytes consumed uncounted): one
   mark stripped by the stream 4/4/0; two marks with one stripped by
   the stream 7/4/3; one mark fed whole 7/4/3; two marks fed whole
   10/7/3, the second mark coming back as a text event. No corpus part
   starts with two marks (158 start with one, 0 with two, both
   sessions' counts); a byte-built test would. Ruling: the stream
   feeds the whole input and maps the crate's positions with an origin
   of 3 when a mark was read and 0 otherwise; the contract is
   unchanged (`bom()` true, the mark in no token's `raw`, the mark
   followed by every `raw` the input byte for byte), and a second mark
   survives inside a text token. The Design sentence is amended above
   and the two-mark part joins P1's test list. The tee's front cursor
   being amortised rather than drained per token is internal and
   changes no bound.

2. What "not well-formed" means for this crate. With its defaults the
   reader refuses a tag with no `>` (`<a`, `<a b="1"`), a mismatched
   end tag (`<a></b>`), an end tag with no start (`</a>`), a `<` in
   text with no later `>` (`a < b`), a dangling `&` (`<a>&</a>`,
   `<a>&`), and any byte that is not UTF-8, in markup or in text
   (`<a>\xc3</a>` and a binary part), since 0.42 decodes text as
   `str` (`ReadTextResult::UpToMarkup(&str)`). It accepts an element
   left open at the end of the input (`<a>`, `<a>hi`), a `<` in text
   followed by a later `>` (`<a>x < y</a>`, read as a tag), an
   unknown entity in text (`<a>&foo;</a>`, a general reference event),
   more than one root (`<a/><b/>`), an empty input and whitespace
   alone. Ruling: P1's four malformed cases are read as the refused
   forms above, now written into the test list; the non-UTF-8 case is
   added to the test list and the Errors section widened, since the
   refusal is not limited to values the layer interprets; the three
   accepted classes that XML calls not well-formed (an element left
   open at the end of input, an unknown entity in text, more than one
   root) go into the `xml` module's Limits, and whether a package part
   with an unclosed root is refused is P2's parsers' question, decided
   in P2's spec text if a test needs it, not in P1. A corpus part the
   crate refuses for any of the refused forms would be a P1 finding;
   none is expected.

3. Order of commits. This ruling is its own doc commit, before any
   code commit; the code commit(s) then carry everything P1 names,
   the dependency line, the residue and the tool's argument parsing
   included. Nothing is held back once the ruling is in.

4. A fourth accepted class, found by the implementing session's own
   test after doc commit 9b87536 and reproduced by planning: the
   crate checks UTF-8 validity and never the XML character range. It
   refuses a PNG header, a high-byte blob, a lone surrogate (`ed a0
   80`) and an overlong NUL (`c0 80`); it accepts, as valid UTF-8, 64
   NUL bytes alone (one text token), NUL in text, in an attribute
   value and inside a tag name, the C0 controls 0x01 to 0x08, 0x0b,
   0x0c, 0x0e and 0x1f, ESC, DEL, `&#0;` and U+FFFE. "A binary part is
   refused" therefore holds for every realistic binary part, whose
   bytes are not valid UTF-8, and not for a contrived part of valid
   UTF-8; the stream's own contract is unaffected, since such a part
   reconstructs byte for byte. Ruling: the class joins the `xml`
   module's Limits as its fourth entry (a character XML forbids that
   is valid UTF-8 is accepted and reproduced; the parser checks the
   encoding, not the character range, so nothing in this layer
   refuses what a spreadsheet application would call corrupt); one
   test pins it with NUL and one other C0 control in text, accepted
   and reconstructed, as the test list now says; the binary-part test
   names its four refused shapes and the UTF-8 reason. The test list
   is amended above.

Rulings on the implementing session's questions after independent
check 1 (code commits e6559d8 and 7a856ad, to be redone):

5. The tool test's fidelity pin. The test crate has no inflater, and
   its fixtures are deflate streams of stored blocks while the tool's
   output is deflated by the `zip` crate, so the test cannot compare
   decompressed bytes directly. Ruling: for a deflated XML entry the
   test asserts CRC-32 and uncompressed size equal to the input's, for
   a stored one the raw bytes; the test list is amended above. The
   decompressed-bytes proof is the harness's `recompressed` status
   over the corpus, which reads both sides through its own inflater;
   no dependency is added to the test crate.

6. `BYTE_ORDER_MARK` stays public and re-exported: the tool needs the
   three bytes now and P3's `write_to` will, and a private copy in
   each consumer would be a second definition of one fact. The Design
   text is amended above.

7. The run log's printed script may carry the unexpanded default
   `CORPUS=${EMEND_CORPUS_DIR:-$HOME/emend-testing}` and the fictional
   `/home/someone/...` positive control for the record-privacy grep,
   as part 1's run logs do: the corpus directory's NAME is not private
   (CLAUDE.md gives it), its PATH is, and neither line is a path on
   this machine nor appears in output. The rule the check cites is
   about real paths; a log that printed the expanded value would be
   the defect.

8. Residue 2, the duplicate-name candidates comment, lives only in
   the committed log script `records/logs/m1p1-p3/size.log` and in no
   source file (planning's grep over the tree finds the word only in
   `archive.rs`'s end-record search, an unrelated use), so P1's code
   commit had nothing to edit and the residue is discharged by P3's
   size script not carrying it. Confirmed.

Noted without a ruling: the module header's sentence on `Error::Io`
being returned unchanged was wrong (the crate holds its I/O error in
an `Arc`, so the kind is copied out) and is corrected in the redone
commit; the run log's column-name resolution now prints the entries
header and asserts the four columns it uses resolve, which is what
specs/0001 asks of a check that can fail. Both code commits are redone
and every log rerun at the new commit, each header naming the
superseded log by sha256 and why; check 1 goes in verbatim.

### P2: content types, relationships, the workbook

Code commit 1: the move of the test builder and the independent parser
into `testing.rs`, the test count unchanged. Code commit 2:
`package::{Package, ContentTypes, Default, Override, Relationships,
Relationship, Workbook, Sheet, SheetKind}`, `xml::Scope`, and
`--survey`. Tests on byte-built packages (the smallest: the content
types part, `_rels/.rels`, the workbook, its rels, one worksheet):
the map resolves; absolute, relative and `../` targets; an external
target; a rels part absent; an Override winning over a Default; a
`.XML` name matched by `Extension="xml"`; a part with no content
type; the strict namespaces; the relationships prefix bound on the
root as `r`, as another name, and on `<sheets>` rather than the root;
a sheet name holding `&amp;` and a character reference; states; a
chartsheet; several sheets in order; and the errors: no content types
part; no officeDocument relationship; two of them; a sheet lacking
`r:id`; an id with an unbound prefix; an id with no relationship; a
target with `..` above the root; two relationships with one id; a root
of the wrong name in each of the three parts; a non-UTF-8 sheet name.
One test shows `Package::open` reads no XML: a package whose content
types part is malformed opens, and the first `content_types()` call
fails. The corpus run `--survey` on both targets, recorded as
`records/m1p2-p2-<utc-date>-<shortsha>.md` and `.txt`, with the run
log's survey aggregation; acceptance as "Harness runs".

### P3: the edits, the first real edit, the size record

The edit methods and `write_to`; `TempFile` shared; `--drop-calc-
chain`; the two probes; the catalogue and Limits in the module docs
(exact-bytes part names; no percent-decoding; the first relationship
of a kind; whitespace kept around a removed element; additions before
the end tag without whitespace; ids generated as `rId<n>`; the rels
part created in Excel's form). Tests: each edit alone on the smallest
package, the output read by the independent parser and the edited
part's bytes compared with bytes the test builds by hand (the removed
element gone, its surrounding whitespace kept, the addition before the
end tag); removal in both `Default` forms, since the raw copy keeps
whichever the input has; a relationship added to a part with no rels
part; the id rule (`rId1` on an empty set, one more than the largest,
an id of another form ignored); several edits to one part; the recipe
on a package with a chain and on one without (byte-identical); the
errors above; a replacement that fails leaving no temporary; every
message in the catalogue tested or marked untested. The corpus run
`--drop-calc-chain` on both targets, recorded as `records/m1p2-p3-
<utc-date>-<shortsha>.md` and `.txt`. The size log and record as
"Size"; the report states part 2's own cost and asks the maintainer
the checkpoint question.

### Close-out push (after P3)

As `specs/0005-corpus.md`, "Close-out push (after P7)", with the
close-out scripts as part 1 ran them.

## Out of scope

- Reading cells, shared strings or styles: parts 3 to 5.
- Creating or renaming sheets or parts other than through `add_part`;
  case-insensitive part names; percent-encoded targets; a rewrite of
  any part other than the content types part and the rels parts.
- The names-only comparison (part 7) and LibreOffice checks on
  generated fixtures (part 8).
- The stored-replacement mode: a note in the size record, no code.
