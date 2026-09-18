# 0009: Milestone 1 part 2, the package layer

Status: approved by the maintainer 2026-09-17 (planning draft the
same day). P1 DONE 2026-09-18 (doc 0e6c60d, 9b87536, b97d01c,
248fdd8; code a9f2cad, 4725795; record d6fa91d); P2 DONE 2026-09-18
(doc bbde39e, 77504d0, 59f6668, 69bb8db, 759e827, c8b8fec; code
82d4e77, 5fc1b3d, 1334830; record eacffa2); P3 DONE 2026-09-18 (doc
0a884ee, f45a3e3, 6adccce, c9c25c3, d6fcc5f; code 11de26e, 5fdf65a,
5dba448, e04db06, 14c2e14; record 711fc8b); the close-out PUSHED
2026-09-18 (doc f4f48e8, 30 commits, CI run 35341839926 success); its
record commit carries a recorded finding ("Planning rulings on the
close-out").
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
helpers) move into `crates/emend/src/testing.rs`, a `#[cfg(test)]`
module, so the package tests build their fixtures and read their
outputs through code that shares nothing with either layer. The move
changes no test and no test's behaviour; the one change it makes is
that the ten zip format constants the moved code took from the layer
through `use super::*` are declared in `testing.rs` from the zip
specification instead (P2 ruling 1). The move is P2's first code
commit.

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
  `resolve_element(name) -> Result<(Option<&[u8]>, &[u8]), Error>`
  gives the namespace and the local name; `resolve_attribute` the
  same, an unprefixed attribute having no namespace. A prefix no
  ancestor binds is `Error::Package("an element or attribute uses a
  prefix its document does not bind")`, which is why the result is a
  `Result` (P2 ruling 2). The null rewrite never resolves a name.
- Escaping: attribute values the package layer writes are escaped with
  the crate's `escape` (`&`, `<`, `>`, `"` and `'`); values it
  interprets (a namespace declaration's value, a sheet name) come
  through its `normalized_value(XmlVersion::Implicit1_0)`, which
  unescapes and applies the attribute-value normalization XML requires
  (a tab, CR, LF or CRLF in a value becomes one space), so a sheet name
  written with a tab is reported with a space (P2 ruling 2; the draft
  named the deprecated `unescape_value`). Values it returns or
  compares as bytes (ids, types, targets, part names, extensions,
  states) are as written, so a target holding a character reference
  resolves to no entry and the survey counts it `unresolved` (P2
  verification ruling 5). Both need `str`, so a value the layer must
  interpret that is not UTF-8, or that holds a reference the crate
  does not know, is `Error::Xml(index)`.

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
  part name keeps its leading slash). A `Default`, `Override` or
  `Relationship` element lacking one of its required attributes is
  `Error::Package("a Default, Override or Relationship element lacks a
  required attribute")` (P2 ruling 4). `ContentTypes::of(&self, name:
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
  and in either form `.` and `..` segments resolved (P2 ruling 5);
  `..` above the root, or an empty result, is `Error::Package("a
  relationship target outside the package")`; an external
  relationship is `Error::Package("an external relationship has no
  part")`. No percent-decoding and no case folding
  (the corpus needs neither; Limits). Two relationships with one id
  are `Error::Package("two relationships with the same id")`; a root
  other than `Relationships` in
  `http://schemas.openxmlformats.org/package/2006/relationships` is
  `Error::Package("a relationships part whose root is not
  Relationships")`.
- Namespaces the layer knows, each in its transitional and its strict
  form, public constants of `package` as (transitional, strict) pairs,
  looked up in that order: the main spreadsheet namespace
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
  one officeDocument relationship")`, the transitional and strict
  forms counted together (P2 ruling 6); `("the officeDocument
  relationship names no entry")` when no entry has the resolved name
  (P2 ruling 3)); its root must be `workbook` in
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
  target, external) -> Result<Vec<u8>, Error>` (the new id; it reads
  the source's relationships, which can fail as `relationships` does;
  P3 ruling 7), `remove_override
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
- `write_to(self, out: impl Write) -> Result<(), Error>`, taking the
  package by value: a package is written at most once, and a refused
  or failed write spends it too, since a `Replacement` is a reader
  that cannot be read twice; the caller reopens (P3 ruling 12). Every
  edit is validated first, so a refused set writes nothing: removing an
  id or an override that is not there (`Error::Package("no such
  relationship")`, `("no such override")`), adding an override for a
  part name that has one and keeps it (`("an override for that part
  already exists")`; one this write removes may be added again with
  another content type, which is how a content type changes, P3
  ruling 11), editing a part also removed (`("an edit to a removed
  part")`), removing or replacing a part no entry carries, or a
  relationship edit whose source is neither the package nor an entry
  (`("no such part")`; P3 rulings 9 and 13), a part name or value that
  is not UTF-8 (`Error::Xml`, carrying the index of the part the value
  would be written into, or the source part's index when its rels part
  does not exist yet, or `[Content_Types].xml`'s when that source is
  the package itself; P3 ruling 13).
  Then, for `[Content_Types].xml` and each rels part with edits, the
  part is read through `Stream` and written to a temporary file (the
  layer's `TempFile`, made `pub(crate)`): every token's `raw` is
  copied except a removed element's, whose own bytes alone are dropped
  (surrounding whitespace stays), and additions go immediately before
  the root's end tag as `<Override PartName="..." ContentType="..."/>`
  or `<Relationship Id="..." Type="..." Target="..."/>`, with
  ` TargetMode="External"` before the slash when external, values
  escaped, no whitespace added. A root written as an empty element
  (`<Relationships xmlns="..."/>`, or `<Types .../>`) has no end tag:
  its own raw bytes are written with the `/` of its closing `/>`
  dropped, then the additions, then `</` + the root's name as written
  + `>`, so the prolog and the root's attributes are kept byte for
  byte (P3 ruling 8). A relationship added to a part with no
  rels part creates one in the form the package's own rels parts use
  (P3 ruling 10): the bytes of `_rels/.rels` before its root start tag
  (the mark if any, the declaration, whatever whitespace follows),
  then `<Relationships xmlns="http://schemas.openxmlformats.org/
  package/2006/relationships">` on one line, the new elements,
  `</Relationships>` and nothing after; when the package has no
  `_rels/.rels`, the prefix is the corpus's majority form (the mark,
  `<?xml version="1.0" encoding="utf-8"?>`, no separator). The part
  is added as an entry, its content type
  coming from the package's Default for `rels`
  (`Error::Package("no Default covers rels")` otherwise). Ids:
  `rId<n>`, `n` one more than the largest among the part's ids of the
  form `rId<digits>`, `rId1` when there is none; the part's ids are
  every one parsed, those pending removal included, plus the ids this
  package has already generated for the source, so two calls give
  `rId5` then `rId6` and a removed id is not reused within a write
  (P3 ruling 7); an id of another form is never generated and never
  collides. The temporary files become
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
`overrides`, `overrides_unresolved` (overrides whose part name, without
its slash, is no entry by exact bytes; P2 ruling 11, added in P3);
`rels_parts` (entries named as a rels part, each parsed
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

### Planning rulings on P1 (2026-09-18, verification)

Verified from primaries at d6fa91d: the seven commits and their
messages (ASCII, Co-Authored-By last, four doc then two code then the
record); the four doc commits hold planning's files unedited (the
working tree at 248fdd8 differs from planning's copies by nothing);
the code diff 248fdd8..4725795 is nine files and every one is P1's
(the dependency, `xml.rs` and its tests, the tool and its test, the
`Error::Xml` variant and its catalogue lines, the `probe-emend-copy`
residue, the lock file); `xml.rs` read in full against "Module `xml`"
and rulings 1, 2 and 4 (the whole input fed, the origin of 3 or 0, the
tee releasing behind the caller, `Error::Io` carrying the kind, the
four Limits classes); the 23 module tests and 7 tool tests read
against P1's amended list, every case present; `scan.log` last,
hashing the four sibling logs and both answers at their committed
bytes, 0 compound shapes in every printed script, 0 control bytes and
0 non-ASCII bytes in the seven files; both check headers pinning every
input by sha256, check 1 naming its inputs superseded and by what.

Planning's own runs at d6fa91d: the full gate (eight PASS, exit 0);
`scripts/privacy.sh --range c414445..HEAD` (7 commits, 301 blobs, 0
findings); `cargo test --workspace` on the host and on i686 (178
passed, 0 failed, 2 ignored on each); the null rewrite over all 27
corpus workbooks with the gate's host binaries, classified by the
harness: 870 entry rows, 781 XML-named of which 771 `recompressed`
and 10 `identical`, 242 carrying `descriptor` and no other flag, CRC-32
and uncompressed size equal on every one, the 89 others `identical`
and unflagged, in every file; the record pair's entry table recounted
to the same figures on both labels.

Rulings:

1. P1's acceptance is met and measured: the token stream reproduced
   every XML part of the corpus byte for byte on both targets, marks
   included, and the pass-through left every other entry untouched.
   The eleven departures are accepted as recorded; the report's own
   corrections (prose that was not a measurement, typed counts that
   went stale) are the right kind and are why the record holds.
2. Independent check 2's three observations: the report's departure 4
   is closed by ruling 6, nothing to do; the tool test's own copy of
   the byte-level builder and independent parser stays, since a
   `#[cfg(test)]` module cannot cross crates and a dev-dependency on
   the core's test code would tie the tool's proof to the code under
   proof, so P2's `testing.rs` consolidates the core's two modules
   only; `Tee`'s `usize` values are in-memory buffer indices, which
   the 32-bit rule does not reach, and every input position is `u64`.
3. The negated-check defect (departure 10) becomes a rule in
   `specs/0001-bootstrap.md` beside the compound-shape rule, in force
   from P2: a negated condition goes through a helper that negates
   (`isnot`), never `$(is ! ...)`, which runs a command named `!` and
   prints UNEXPECTED whatever the condition does; the scan log's grep
   covers `isnot` in the compound shape and adds the `!` form.
4. The two unlogged asides (departures 1 and 7) stand as the report
   labels them: history from working notes, cited by nothing.

P1 is DONE. P2 starts with the doc commit carrying this ruling, the
`specs/0001` rule and the ROADMAP row.

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

### Planning rulings during P2 (2026-09-18)

1. The move of the builder and the parser. As written, "move
   unchanged" does not compile: through `use super::*` the moved code
   takes ten constants `archive.rs` declares module-private
   (`CENTRAL_FIXED`, `U32_MAX_MARKER`, `ZIP64_EXTRA_ID`,
   `ZIP64_EOCD_SIG`, `LOCAL_SIG`, `EOCD_SIG`, `CENTRAL_SIG`,
   `ZIP64_LOCATOR_SIG`, `ZIP64_EOCD_FIXED`, `GP_DESCRIPTOR`; planning
   counted the declarations and the uses), while it already carries its
   own `METHOD_STORE` and `METHOD_DEFLATE`. Worse than the build error,
   the independent parser was reading the layer's idea of a central
   record's size, so a wrong `CENTRAL_FIXED` would have been the same
   mistake on both sides, which is exactly what that parser exists to
   rule out. Ruling: `testing.rs` declares the ten from the zip
   specification (APPNOTE 6.3.x, section 4.3, cited beside each
   value), as it already does the two method constants; nothing in
   `archive.rs` becomes `pub(crate)` for the tests' sake. The "Crate"
   text is amended above. The move set is `crc32`, `deflate_stored`,
   `METHOD_STORE`, `METHOD_DEFLATE`, `Spec`, `Build`, `OutEntry`,
   `Parsed`, `parse_independently` and `names` (it takes a `Parsed`);
   everything that asserts something about the archive layer or uses
   its types (`round_trip`, `assert_identical`, `check_round_trip`,
   `open_err`, `bytes_replacement`, `apply`, `REPLACEMENT_LOCK`,
   `FailingWriter`, `replacement_temporaries`, `at_u16`, `at_u32`,
   `data_start_of` and the rest) stays in `archive/tests.rs`. The
   move changes no test: the evidence is the move commit's diff, whose
   removed and added lines differ, once `pub(crate) ` and blank lines
   are stripped, only by the module header and the ten cited
   constants (both P2 checks made that comparison), and planning's own
   test run at the move commit 82d4e77, recorded in the verification
   ruling. (The draft said `test.log` shows 178; the phase's test log
   runs at its last commit, where two later commits had added tests,
   so no log holds that count. Amended after the checks, ruling 7.)

2. Reading attribute values. `Attribute::unescape_value` is
   deprecated in 0.42.0 (`events/attributes.rs`, "use
   `Self::normalized_value()`"), and the gate runs clippy with warnings
   denied, so the method the draft named fails the gate. Ruling: the
   layer reads values through `normalized_value(XmlVersion::
   Implicit1_0)`, the crate's successor, which unescapes and applies
   the attribute-value normalization the XML specification requires.
   Correction (2026-09-18, after doc commit 59f6668): planning first
   wrote `XmlVersion::Version10`, a variant that does not exist; the
   crate's variants are `Implicit1_0`, `Explicit1_0` and
   `Explicit1_1`. `Implicit1_0` is what the deprecated method itself
   passes (`events/attributes.rs`), and the two 1.0 variants take the
   same end-of-line path, only `Explicit1_1` folding U+0085 and U+2028
   as well; planning checked both in the source. The implementing
   session used `Implicit1_0` and reported the error. The
   implementing session measured the difference over the corpus
   before recommending it: 8584258 attribute values in the 781
   XML-named parts, 0 holding a tab, CR or LF, 253 holding an
   ampersand, so on this corpus the choice changes no value and
   unescaping itself is load-bearing. The Design text is amended
   above, since a normalized value is a behaviour of the layer. Noted
   without a ruling: `Error::Package(&'static str)` is added as
   "Errors" says; `Scope::resolve_element` and `resolve_attribute`
   return `Result`, since the unbound-prefix error has to travel
   (amended above); the crate's `QName`, `LocalName` and `Prefix` are
   `str`-based and the spec's byte slices come from `as_bytes()`.

Rulings on the four gaps raised after code commit 5fc1b3d:

3. The officeDocument relationship resolving to a name no entry has
   is `Error::Package("the officeDocument relationship names no
   entry")`, with its test; the Design text is amended. A sheet's
   target need not be an entry: `sheets_resolved` measures that.

4. A `Default`, `Override` or `Relationship` element lacking a
   required attribute (`Extension` and `ContentType`; `PartName` and
   `ContentType`; `Id`, `Type` and `Target`) is refused, not recorded
   as nothing: `Error::Package("a Default, Override or Relationship
   element lacks a required attribute")`, one message, with a test per
   element kind. The implementing session had recorded such an element
   as declaring nothing; the ruling goes the other way because the
   spec already refuses a `<sheet>` lacking `name`, `sheetId` or
   `r:id`, and one rule for a structurally incomplete element is
   easier to reason about than two. A part that hits it is corrupt as
   a package, which a later edit could not have preserved meaningfully
   either. The Design text is amended.

5. `target_part` resolves `.` and `..` in an absolute target as well
   as a relative one: `a/../b` is not an entry name whichever form it
   arrives in, and on the corpus no absolute target carries a dot
   segment, so the unified rule changes no resolution. The Design
   text is amended.

6. "more than one officeDocument relationship" counts the
   transitional and the strict form together: a package carrying one
   of each does not say which is the workbook. The first-of-a-kind
   rule stays for the workbook's four options. The Design text is
   amended.

Noted without a ruling: `Scope` has no method for an empty element,
since a parser calls start, resolves, then end before the next token,
which the Design text allows; the namespaces and the eight
relationship types are public constants of `package`, each a
(transitional, strict) pair, since P3's recipe needs the calcChain
URI (the Design text now says so).

Rulings on the findings of P2's two independent checks (2026-09-18,
before the record commit):

7. Ruling 1's last sentence is amended as above: the property is
   witnessed by the diff comparison both checks made and by planning's
   own test run at 82d4e77 in a throwaway worktree on the host
   (2026-09-18: 178 passed, 0 failed, 2 ignored, the P1 count), not by
   a phase log. No rerun at an older commit.

8. Four inaccurate doc comments in the code (the `archive.rs`
   catalogue entry pointing at `xml.rs` for the `Scope` message, which
   `package.rs` catalogues; `xml.rs`'s header still saying the module
   produces one variant; `package/tests.rs`'s header counting the
   smallest package as five parts where `smallest()` writes six; the
   backticks in `package.rs`'s first catalogue entry that the message
   does not carry) are fixed in P3's first code commit, not in P2: a
   fourth P2 code commit would invalidate every log and both answers,
   and a third check round is not allowed. The report records them as
   found and deferred; P3's diff is expected to carry exactly these.

9. `gate.log` and `negative.log` are rerun with the `isnot true`
   control the `specs/0001` rule requires, since the rule is this
   phase's own and in force from P2; the reruns name the superseded
   rounds. The gate's binary hashes were shown equal across the two
   existing gate rounds, so `test.log` and `run.log` stand.

10. `columns_resolve` in `run.log` reports as-expected on an empty
    file (its only rule is `NR == 1`). As used it is not vacuous: the
    entries file's existence is asserted before it and the same file
    yielded 781 and 89 rows. `run.log` stands with the report naming
    the weakness; P3's run script fixes the form (an `END` rule that
    fails when no header line was seen) with a control on an empty
    file.

11. Check 2's observation that the fact "every one of the 614 Override
    part names resolves exactly" is witnessed by no survey key: the
    survey gains `overrides_unresolved` (overrides whose part name,
    without its slash, is no entry by exact bytes; `case_only` keeps
    counting the case-folded subset) in P3's first code commit with a
    unit test, and P3's run log runs `--survey` on both targets beside
    the `--drop-calc-chain` runs, expecting 0. Until then the fact
    stands as planning's own measurement, made by names only.

The `scan.log` script's inherited "P1" about-line and its misattributed
comment are the implementing session's to fix in the final scan pass,
as it proposed.

### Planning rulings on P2 (2026-09-18, verification)

Verified from primaries at eacffa2: the ten commits and their
messages (ASCII, Co-Authored-By last; six doc, three code, the
record; the rulings that arrived mid-phase each committed before the
code that cites them); the six doc commits hold planning's files
unedited (the working tree at c8b8fec differs from planning's copies
by nothing, and each touches only `specs/` and `ROADMAP.md`); the
code diff d6fa91d..1334830 is nine files and every one is P2's (the
move into `testing.rs`, `package.rs` and its tests, `Scope` in
`xml.rs`, `Error::Package` and its catalogue lines, the tool's
`--survey` and its two tests, `lib.rs`); `package.rs` and `Scope`
read in full against "Module `xml`", "Module `package`" and rulings
1 to 6 (names as bytes; the Override-then-Default lookup with the
extension folded; `target_part` resolving dot segments in both forms
and refusing `..` above the root; the officeDocument forms counted
together; the first of a kind for the four options; an unprefixed
attribute having no namespace; a namespace declaration's value and a
sheet name normalized, every other value as written; the
incomplete-element message of ruling 4); the 37 package tests and 9
tool tests read against P2's list, every case present, the chartsheet
and dialogsheet kinds covered in `sheet_states` and
`several_sheets_in_order`; the count 217 as 178 plus 37 plus 2;
`scan.log` last, hashing the six sibling files at their committed
bytes, 0 compound shapes and 0 `!` shapes in every printed script, 0
control bytes and 0 non-ASCII bytes in the seven files; the seven
hashes the report names equal to the committed files; both check
headers pinning 1334830; the `isnot true` control present in
`gate.log` and `negative.log`, and the superseded rounds named in each
by byte count and sha256, the failed third gate attempt included.

Planning's own runs at eacffa2: the full gate (eight PASS, exit 0);
`scripts/privacy.sh --range d6fa91d..HEAD` (10 commits, 318 blobs, 0
findings); `cargo test --workspace` on the host and on i686 (217
passed, 0 failed, 2 ignored on each); `--survey` over all 27 corpus
workbooks with release binaries of both targets: every run exit 0,
every output byte-identical to its input, nothing on stderr, and the
totals equal to the run log's on both targets (entries 870, typed
843, untyped 0, defaults 110, overrides 614, rels_parts 216,
relationships 878, external 234, resolved 644, unresolved 0,
case_only 0, sheets 95, sheets_resolved 95, shared_strings 27, styles
27, calc_chain 9, theme 27); the record pair's tables recounted (54
file rows, every one ok with diff exit 0, unflagged, files_identical
yes and lo ok; 1740 entry rows, every one identical).

Rulings:

1. P2's acceptance is met and measured: the package layer reads the
   content types, every relationships part and the workbook of every
   corpus file on both targets, every internal target and every sheet
   resolves to an entry by exact bytes, and the survey edits nothing.
   The five departures are accepted as recorded.
2. Departure 2: the absolute target with dot segments is pinned by a
   fixture, not by the argument alone. `target_absolute` gains one
   relationship whose target is absolute and carries a `.` and a `..`
   segment, resolving to the same entry as its plain absolute
   neighbour, in P3's first code commit. No P2 rerun: the argument
   that both forms share one loop reads as true in the code, and the
   corpus has no such target to disagree.
3. A fifth inaccurate doc comment: `Scope`'s header says the ruling
   names `Version10`, which was true of doc commit 59f6668 and false
   since 69bb8db. It says the ruling names `Implicit1_0` and the
   sentence about the correction goes. Fixed in P3's first code commit
   with ruling 8's four; P3's diff is expected to carry exactly these
   five and the one-line fixture of ruling 2.
4. Departure 5: a control asserts a property, never a position that a
   later commit can change; the committed gate's form (the first doc
   commit, by hash, is a doc commit) is the rule from P3, and the
   failed attempt stands recorded as the log names it.
5. Design, "Escaping", is amended above to say which values are
   normalized: those the layer interprets (a namespace declaration's
   value, a sheet name) and no other. Ids, types, targets, part names,
   extensions and states are bytes as written, so a target holding a
   character reference resolves to no entry and the survey counts it
   `unresolved`, which is 0 on the corpus. The code does this already;
   nothing changes.
6. Departure 3's "twelve cases" against check 1's eleven: the sentence
   describes an uncommitted round, and the committed script's N1 to
   N11 are the count that stands. Nothing to do.

P2 is DONE. P3 starts with the doc commit carrying this ruling and the
ROADMAP row, then the first code commit as "P3" says.

### P3: the edits, the first real edit, the size record

P3's first code commit carries the five doc-comment fixes P2 ruling 8
and P2 verification ruling 3 name, the absolute-target fixture of
verification ruling 2, the `overrides_unresolved` survey key of
ruling 11 with its unit test, and the `columns_resolve` form of
ruling 10 in the run script; the survey then runs on both targets in
P3's run log beside the edit runs, expecting `overrides_unresolved` 0
in every file. P3's gate controls assert properties, never a position
(verification ruling 4). P3's second code commit adds the gate's `doc`
step and the one link fix it needs (P3 ruling 4), before the edits, so
the gate that checks the edit commits already carries it.

The edit methods and `write_to`; `TempFile` shared; `--drop-calc-
chain`; the two probes; the catalogue and Limits in the module docs
(exact-bytes part names; no percent-decoding; the first relationship
of a kind; whitespace kept around a removed element; additions before
the end tag without whitespace; ids generated as `rId<n>`; the rels
part created in the form the package's own rels parts use, which no
producer name describes). Tests: each edit alone on the smallest
package, the output read by the independent parser and the edited
part's bytes compared with bytes the test builds by hand (the removed
element gone, its surrounding whitespace kept, the addition before the
end tag); removal in both `Default` forms, since the raw copy keeps
whichever the input has (the two spellings of "Facts", with and
without a space before the slash: P3 delivered the empty-element and
start-tag forms instead, and the spelling case is carried into part
3's first code commit, P3 verification ruling 4); a relationship
added to a part with no rels
part; the id rule (`rId1` on an empty set, one more than the largest,
an id of another form ignored); several edits to one part; the recipe
on a package with a chain and on one without (byte-identical); the
errors above; a replacement that fails leaving no temporary; every
message in the catalogue tested or marked untested. The corpus run
`--drop-calc-chain` on both targets, recorded as `records/m1p2-p3-
<utc-date>-<shortsha>.md` and `.txt`. The size log and record as
"Size"; the report states part 2's own cost and asks the maintainer
the checkpoint question.

### Planning rulings during P3 (2026-09-18)

On the four items the implementing session raised after the first
code commit 11de26e (planning read the diff, ran `cargo doc` at
11de26e and reproduced the one warning it names):

1. The catalogue entry for "no [Content_Types].xml part" escapes its
   brackets rather than dropping the backticks: rustdoc reads bare
   `[Content_Types]` as an intra-doc link, which is why the backticks
   were there. Accepted; the doc text now equals the message byte for
   byte and makes no link. Recorded because check 1 called the fix
   cosmetic and it was not.
2. Two of the five doc-comment fixes were applied in two places each
   (the five-parts sentence in `package/tests.rs`'s header and in
   `smallest()`'s doc; the message in `package.rs`'s catalogue and in
   `content_types`'s doc). Accepted as done: "exactly these five"
   names five defects, and a file that states one string two ways is
   one defect in two places. P3's checks expect the five in seven
   places; nothing is reverted. The diff, read by planning, is the
   evidence for that; no log line is asked for (verification ruling
   4).
3. `overrides_unresolved` sits directly after `overrides` in the
   survey line, as committed and as the tool tests pin it; the Design
   list of keys is amended above. The order the tests pin is the order
   P3's run log quotes.
4. The gate gains a step. `cargo doc` was no step of the gate, so no
   committed check read a doc comment, and the tree has carried
   "public documentation for `xml` links to private item
   `Stream::origin`" since P1; planning confirmed at 11de26e that
   `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` exits
   101 on it, in under a second when the build is warm. Ruling: step
   5 `doc`, placed after `clippy`, runs exactly that command through
   the gate's `quiet` wrapper and prints `PASS doc`; `native`,
   `licences`, `hygiene` and `privacy` become steps 6 to 9, so `--ci`
   runs steps 1 to 8 and skips step 9, and `--quick` keeps the step,
   since it is cheap. The script's header, `STEPS` and `--only` follow;
   the workflow needs no change, since `--ci` selects steps by name.
   The `Stream::origin` link is fixed in the same commit, which would
   otherwise fail its own gate. P3's negative log adds one case: a
   throwaway worktree in which a public doc comment links a private
   item fails the gate at `FAIL doc` with a non-zero exit, as a named
   function asserting the exit, the FAIL line and no PASS line (the
   `isnot` helper is for a negated condition, and this is a positive
   one; wording amended in the verification ruling).
   `specs/0003-ci.md`'s `--ci`
   description is amended to the new numbers; every earlier "eight
   PASS" is the record of the gate as it was and stands. From this
   commit a phase's `gate.log` is a full run of nine steps. The
   commit changes a gate file, so the `specs/0001` rule for that
   applies: `scripts/gate-self-test.sh` runs in P3 and its log is
   committed with the phase's other logs.
5. Ruling 10's `columns_resolve` form lands in P3's run log's script,
   as the commit message says; the run log is where the rule lives.

On the second code commit 5fdf65a and the three edit-layer questions
(planning read the three diffs, ran the nine-step gate at 5fdf65a with
exit 0, and reproduced the negative case: the 5fdf65a script run in a
throwaway worktree at 11de26e reports `FAIL doc` and exits 1 on the
`Stream::origin` link):

6. `scripts/gate-self-test.sh` naming its phase from an input
   (`PHASE_NOTE`, beside `RERUN_NOTE`) rather than from its own text
   is accepted, with the two "step 7" comments renamed. It is the
   class this phase has ruled on twice, and a committed log whose
   first line names the wrong phase is worse than a script edit; the
   script is a gate file, and the self-test in flight runs it at
   5fdf65a, which is what the `specs/0001` rule asks.
7. `add_relationship` returns `Result<Vec<u8>, Error>`: the id rule
   needs the source's relationships, which are parsed on demand and
   can be refused, and the caller must learn the id it will reference.
   Ids count every parsed relationship, pending removals included, and
   every id this package has generated for the source, so consecutive
   calls give consecutive ids and a removed id is not reused within a
   write. The Design text is amended.
8. A root written as an empty element is rewritten as it is copied
   (option b), for a rels part and for `[Content_Types].xml` alike:
   the root's raw bytes with the `/` of its `/>` dropped, the
   additions, then `</` + the root's name as written + `>`. The Types
   case is reachable: `<Types xmlns="..."/>` parses to empty lists, so
   `add_override` on such a package followed by `write_to` reaches the
   root, and its test builds the package that way. One test per part
   kind, the expected bytes built by hand. Refusing (a) rejects a
   legal package; a fresh part (c) discards bytes the input had. The
   Design text is amended.
9. `remove_part` or `replace_part` naming no entry is refused in
   `write_to`'s validation as `Error::Package("no such part")`, with
   its test, and never a silent no-op: the recipe asks the caller to
   check the entry exists, and a package that lost its calcChain entry
   but kept the relationship would otherwise write out unchanged and
   report success. P3 adds and catalogues six messages: "no such
   relationship", "no such override", "an override for that part
   already exists", "an edit to a removed part", "no Default covers
   rels", "no such part". The Design text is amended.
10. The created rels part's form. The Design named an Excel form
    (`standalone="yes"`) that no corpus rels part writes; the
    implementing session measured this and planning repeated the
    measurement over all 216 rels parts of the 27 files, reading only
    the bytes before and after the root: 148 parts in 10 files begin
    with the mark, `<?xml version="1.0" encoding="utf-8"?>` and no
    separator; 68 parts in 17 files begin with `<?xml version="1.0"
    encoding="UTF-8"?>` and one LF, no mark; no file mixes the two;
    every root start tag is the one line the Design writes; nothing
    follows any end tag; and all 662 gaps between consecutive
    Relationship elements are empty. Ruling: option (c). The created
    part takes its prefix from the package's own `_rels/.rels` (the
    mark and every token's raw bytes before the root start tag, read
    through `Stream`), then the root line, the elements, the end tag
    and nothing after; with no `_rels/.rels` to copy, the prefix is
    the majority form above. One test per corpus form and one for the
    fallback, expected bytes built by hand. The Limits line says the
    rule, not a producer. Nothing in a log claims a producer's form.

On the code commits 5dba448 and e04db06 (planning read both in full
and ran the workspace tests on the host at e04db06: 253 passed, 0
failed, 2 ignored, of which the library's binary is 171 and the
tool's 12):

11. An override this write removes may be added again with another
    content type; "an override for that part already exists" refuses
    a name that keeps its override. Accepted as the implementing
    session judged it, since that is how a part's content type
    changes; the Design text is amended and the test
    `an_override_removed_and_added_again_is_not_a_duplicate` stands.
12. `write_to` takes `self` by value. A `Replacement` is a reader
    that cannot be read twice or cloned, so a second write on the same
    package would silently lack the part edits; the type refuses the
    second write instead of a runtime check, and a refused or failed
    write spends the package too, so the caller reopens rather than
    retrying on one whose replacements are gone. The tool's
    `write_package` and the probe adapt; a test that writes twice opens
    twice. The Design text is amended.
13. Two small things in the same code: a relationship edit whose
    source is neither the package (the empty name) nor an entry is
    refused in `validate` as "no such part", so a rels part is never
    created for a part that does not exist; and the elements built for
    a rels part that does not exist yet carry the source part's index
    in `Error::Xml`, not index 0, which names the wrong entry; when
    the source is the package itself, which has no entry, the index is
    `[Content_Types].xml`'s, which the created part's `Default` check
    requires to exist and which runs before the elements are built.
    Each with a test. The Design text is amended.
14. The zip layer's test lock and temporary listing moved into
    `crate::testing`, held by the package tests too: accepted, for the
    race the implementing session saw (a package test's temporary
    caught in flight by the zip layer's no-temporary-left snapshot).

The corpus run waits for the code commit that carries 12 and 13, and
runs at that commit.

### Planning rulings on P3 (2026-09-18, verification)

Verified from primaries at 711fc8b: the eleven commits and their
messages (ASCII, Co-Authored-By last; five doc, five code, the record,
none mixed); the five doc commits hold planning's files unedited (the
working tree at 711fc8b differs from planning's copies by nothing);
the code diffs read in full as each landed (11de26e, 5fdf65a,
5dba448, e04db06 earlier in this section; 14c2e14 now: `write_to` by
value with 33 call sites passing the package rather than a borrow, the
"no such part" refusal for a source that is no entry, the error index
falling back to the source part then the content types part, and the
four tests); the nine committed files' byte counts and sha256s equal
to the report's; `scan.log` last, hashing the eight siblings and the
three records at their committed bytes, 0 compound shapes and 0 `!`
shapes in every printed script, 0 control bytes and 0 non-ASCII bytes
in the nine files; the `isnot true` control in `gate.log` and
`negative.log`; every log's about line naming 14c2e14, `self-test.log`
naming 5fdf65a with `gate.log` asserting no gate file changed after
it; both check answers read in full and every finding either acted on
(the absolute path in `negative.log`, rerun and named; the `f_home`
scan form with its exemption and seven controls; the report's prose
corrections) or ruled on below.

Planning's own runs at 711fc8b: the full gate (nine PASS, exit 0);
`scripts/privacy.sh --range eacffa2..HEAD` (11 commits, 343 blobs, 0
findings); `cargo test --workspace` on the host and on i686 (257
passed, 0 failed, 2 ignored on each); `--drop-calc-chain` over all 27
corpus workbooks with release binaries of both targets, every entry
classified by an independent zip reader (Python's `zipfile`, not the
harness): 27 runs exit 0 with nothing on stderr on each target, 18
outputs byte-identical to their inputs and 9 differing, in those 9
one entry dropped (`calcChain.xml` under `xl/` in every one) and two
changed (`[Content_Types].xml` and the workbook's rels part in every
one), 843 identical, 0 added, order kept in all 27, on both targets;
the record pair recounted (54 file rows all ok, 36 with diff exit 0
and 18 with 1, 36 files_identical yes and 18 no, lo and lo_in ok on
all 54; entry rows 843 identical, 18 changed, 9 dropped per label, no
flag on any row); the size record's three figures recomputed from its
seven probe rows.

Rulings:

1. P3's acceptance is met and measured on both targets: the first
   real edit drops exactly the three things the recipe names and
   nothing else, the eighteen workbooks without a chain come back byte
   for byte, and every output opens in LibreOffice. The budget figure
   is 183980 section bytes over `probe-zip-xml`, under 256000 by
   72020. Part 2's own cost is 51136 section bytes, above the
   checkpoint's 40960: the choice `specs/0008` "Size" gives the
   maintainer is open and is put to the maintainer by planning; no
   session decides it.
2. The stale count in `xml.rs` ("the fourteen other package
   messages", 21 in the catalogue at the phase head) is not fixed in
   P3: an eleventh commit would invalidate every log and both answers,
   and the defect is a number in prose. Part 3's first code commit
   fixes it by removing the count (the sentence says "the other
   package messages") and adds a unit test in `package/tests.rs` that
   reads `package.rs` and `xml.rs` with `include_str!`, collects every
   `Error::Package("...")` literal and every catalogue entry (the
   `\[` escapes unescaped), and asserts the two sets are equal, so
   the catalogue can no longer fall behind the source. Check 1's
   observation that nothing pins the equality is what this closes.
3. The two checks that could not have failed stand as recorded, with
   the weakness named here, and are not rerun: `size.log`'s per-probe
   causes accounting is vacuous whenever the lock file moved (it
   prints 1, 1, 8, 8, 8), and `run.log`'s faulty fixture drives five
   of twelve counters. The acceptance they guard is witnessed by
   planning's own independent classification above, which measures
   the same dropped, changed, identical and added counts by another
   reader. Part 3's size script excludes the workspace manifest and
   lock file from a probe's own cause paths, and its run script's
   faulty fixture drives every counter, each asserted by name.
4. The three spec readings: "removal in both `Default` forms" names
   the two spellings of "Facts", with and without a space before the
   slash, and P3 delivered the empty-element and start-tag forms
   instead; the spelling case (a content types part whose `Default`
   elements carry the space, one `Override` removed, every `Default`
   back byte for byte) is one test in part 3's first code commit, and
   no bug is behind the gap, since a removal drops the whole token.
   Ruling 2's "seven places" asks for no log line; the diff read by
   planning is the evidence. Ruling 4's "through the `isnot` helper"
   was planning's wording error: `is g1_ok`, a named function
   asserting a non-zero exit, a FAIL line and no PASS line, is the
   form the rule requires for a positive multi-condition check, and
   the ruling is amended. All three sentences are amended above.
5. The redaction in check 1's answer is right: an absolute machine
   path a checker quoted is written as its placeholder with the
   replacement declared in place, and verbatim quotation does not
   override the T0 P6d rule. The rule is added to `specs/0001`,
   "Rules for every phase". The superseded `negative.log` that printed
   the path was never committed, and planning's privacy scan over the
   eleven commits finds nothing.
6. D5: a tool operation that edits a package validates it is a
   package before deciding the edit is empty. `--drop-calc-chain`
   reads the content types part first, as `--survey` does, so a
   package with no `[Content_Types].xml` is refused by both; the
   library's own rule stands (a `Package` with no edits recorded
   writes a copy, since the edit touched nothing). Part 3's first code
   commit, with D5 becoming a refusal in that phase's negative log.
7. The smaller notes: the self-test log's invocation line names the
   launch form from part 3 (the property is that no build of the
   session ran during it, which the log's header states); the corpus
   run using binaries left by the superseded first gate attempt is
   fine, since the post-clean rebuild reproduces the same four hashes;
   the two overlong comment lines in `scripts/gate-self-test.sh` are
   rewrapped in part 3's first code commit; the ROADMAP P2 row riding
   in P3's first doc commit is planning's housekeeping and breaks no
   rule. The report itself is not committed, as in every phase: the
   logs, the answers and the records are the record.

P3 is DONE, and with it part 2's three phases. The close-out push has
planning's word; it runs when the maintainer authorises it in the
implementing session, and the maintainer's checkpoint answer is
recorded in `specs/0008` "Size" and in part 3's spec when given.

### Close-out push (after P3)

As `specs/0005-corpus.md`, "Close-out push (after P7)", with the
close-out scripts as part 1 ran them.

Planning rulings on the close-out (2026-09-18):

1. The push is verified from primaries. origin/main equals local main
   at f4f48e8 (fetched by planning). The doc commit f4f48e8 holds
   exactly planning's three files (141 insertions, 9 deletions, no
   code). push.log records the nine-step gate before the push, the
   hook's range 7e9846a..f4f48e8 with 30 commits, 376 blobs and 0
   findings, remote main equal to local main and the repository
   PRIVATE. ci-run.log records run 35341839926 on f4f48e8, completed
   success, PASS host to hygiene with doc among them, SKIP privacy.
   Planning counted 0 control or non-ASCII bytes and 0 bare shapes in
   both logs.
2. The scan's structural failure is a true finding, not a conflict of
   rules. T0 P6d narrowed the GATE, so a real check answer that names a
   pattern's shape is not refused. `specs/0001` "Rules for every phase"
   kept the requirement that the project's own files write no bare
   shape, proved by every scan script's pre-P6d control expecting 0,
   with a documented reversible replacement as the only exception. The
   two instruments govern different things and both stand; the ROADMAP
   P6d row now says "passes the gate". Planning reproduced the count:
   over the 56 files of the pushed range at f4f48e8 the three pre-P6d
   forms hit 2 lines, both in `records/logs/m1p2-p3/scan.log` (lines
   79 and 130, the same comment in the printed script's change list
   and at the f_home definition), and 0 lines elsewhere in the whole
   tracked tree. Not a leak: the line names no account and no machine.
   It is a form defect of the committed log, of the class the rule
   forbids; the same script builds that shape by concatenation for its
   own controls, so the technique was at hand. The expectation stays
   at 0.
3. Why no phase scan saw it: a scan script prints itself into the log
   it writes, and no scan reads its own log. The gap is closed by a
   rule, not by another check round: a scan script runs the pre-P6d
   forms over its own source file before printing itself, as a
   structural check with a control (`specs/0001`, new bullet).
4. The scan output as it stands is NOT installed. Its own printed
   script carries the same comment (planning counted 1 hit, at output
   line 36), so committing it would seed part 3's close-out range with
   a third hit. The close-out scan is rerun, its header naming both
   superseded attempts by sha256, with exactly these changes: (a) the
   comment names no shape ("a Windows user path", or the concatenated
   form); (b) the self-scan of ruling 3, a structural check whose
   control plants one concatenated shape in a copy of the source and
   counts 1; (c) the range's tip is read from git as the close-out doc
   commit by subject, the way the base is, never as HEAD, because HEAD
   is past f4f48e8 once this ruling's doc commit lands; the header
   still prints HEAD at run; (d) the correction log of ruling 5 is a
   sibling, hashed and scanned like push.log and ci-run.log. Nothing
   else changes, and the pre-P6d expectation of 0 is not touched. The
   rerun ends RESULT: FAIL on the same two hits and is committed as
   such: a failing run is a finding to record (CLAUDE.md), and this
   one cannot pass for this range because the file is committed and
   pushed. The fix is rulings 3 and 5; the run again with the fix
   named is part 3's close-out scan, whose range does not contain that
   file and whose header names this finding.
5. A correction log, `records/logs/m1p2-close/correction.log`, in the
   T3 P4 `record.log` form: it names the superseded lines by number
   and by place (the two comment lines of the printed script, lines 79
   and 130 of `records/logs/m1p2-p3/scan.log`) and the record commit
   711fc8b they were committed at, resolved from git by subject; it
   prints the replacement wording; and it proves what it names by a
   printed command that counts the pre-P6d hits in that file (2) and
   lists the hit line numbers (79 130), never the matched text. The
   log writes no shape.
6. f_home's narrowing in the close scan (attempt 1 superseded, sha256
   8f6315f8...) is accepted. The three hits were deliberate prose:
   `specs/0001` recording the maintainer's ruling that `/home/dev`
   identifies no person, the same file's list of what the scans look
   for, and a part 2 P1 check answer asserting the form does not
   occur. Flagging the home directory only where a path continues into
   it is the P6d principle applied to this form, and its ten controls
   use the same command form. Part 3's phase scan carries this
   narrowed f_home, not P3's first form, which passed only because
   P3's files name no path.
7. Order: this ruling goes in a doc commit holding planning's three
   files unedited (ROADMAP.md, specs/0001-bootstrap.md,
   specs/0009-package-layer.md); then correction.log; then the scan
   rerun; then ONE record commit holding push.log, ci-run.log,
   correction.log and scan.log. Both commits go with the next push. No
   independent check, as the close-out spec says. Report and STOP;
   part 3's spec follows.

## Out of scope

- Reading cells, shared strings or styles: parts 3 to 5.
- Creating or renaming sheets or parts other than through `add_part`;
  case-insensitive part names; percent-encoded targets; a rewrite of
  any part other than the content types part and the rels parts.
- The names-only comparison (part 7) and LibreOffice checks on
  generated fixtures (part 8).
- The stored-replacement mode: a note in the size record, no code.
