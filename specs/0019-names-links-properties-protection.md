# 0019: Milestone 2 part 4a, names, links, properties, protection

Status: APPROVED by the maintainer 2026-09-27 (planning draft the
same day): decisions 1 to 5 taken as planning recommended, in the
maintainer's reply "i agree with recs"; decision 6 recorded. Sent
to the implementing session the same day.
Context: `CLAUDE.md` ("What emend is": like openpyxl; "Dependencies
and licences": anything beyond the core behind a Cargo feature off
by default; "The fidelity harness is the acceptance instrument"),
`ROADMAP.md` ("Milestone 2", the outline approved 2026-09-23, item
4, "behind features or in a second crate"; "Standing items", the
Milestone 2 ceiling), `specs/0009-package-layer.md` (relationships
and content types, the eight type names), `specs/0011-editing-
cells.md` (the worksheet rewrite, the text rule for a written cell),
`specs/0013-formulas.md` (the formula-change rule and `calcPr`),
`specs/0016-new-workbook-and-sheets.md` (the workbook rewrite and
its regions, `localSheetId` remapping, the template), `specs/0017-
fuller-styles.md` (`Protection` on a cell style), `specs/0018-rows-
and-columns.md` ("Creating an element", the schema-position rule;
the one-shift rule; the close-out rulings carried here),
`specs/0001-bootstrap.md` ("Rules for every phase"). ECMA-376 Part
1, 18.2.27 (`workbook`, the order of the root's children), 18.2.5
(`definedName`), 18.3.1.47 (`hyperlink`), 18.3.1.48 (`hyperlinks`),
18.3.1.85 (`sheetProtection`), 18.2.29 (`workbookProtection`);
ECMA-376 Part 2, 11 (core properties, `coreProperties` and its
Dublin Core elements); ECMA-376 first edition Part 4, the legacy
password hash under `sheetProtection`, are cited where the design
leans on them; no text is copied from them.

## Why a part 4a

The outline's part 4 lists nine capabilities. Planning split it on
2026-09-27 into sub-parts with their own specs, each behind its own
Cargo feature, measured beside the core as `dates` and `rowcol`
are: 4a (this spec) defined names, hyperlinks, document properties
and protection, which live in parts the crate already rewrites or
in one small new part; 4b data validation and conditional
formatting; 4c tables; 4d comments and images, each of which needs
a drawing part. The order is the maintainer's to change.

## Goal

A caller can define and remove a workbook-level or sheet-level
name, read the names a workbook has, put a hyperlink on a cell or
take one off (to a URL, a file or a mail address outside the
package, or to a place inside it), read and set the document's
core properties (title, author, keywords, dates and the rest), and
protect or unprotect a worksheet or the workbook's structure with
or without a password, on an opened workbook or on
`Package::new()`'s, with everything the edit does not touch passed
through byte for byte. The comparison point is openpyxl's
`wb.defined_names`, `cell.hyperlink`, `wb.properties`,
`ws.protection` and `wb.security`, with the same limits: a name's
text is stored, never parsed or evaluated; a hyperlink changes
nothing about the cell's value or style; protection is Excel's
advisory lock with the legacy hash, not encryption.

Two things are read back as typed values, since a caller cannot
use a name or a property without knowing what is there: the
defined names (name, text, scope, hidden) and the core properties.
Hyperlinks and protection are not read back; the harness counts
their elements.

The harness is the acceptance: over the corpus each sample leaves
every other entry byte-identical, the edited part identical outside
the elements the edit names, and those elements as the rule gives
them, as an independent parser in the run script shows; every
output opens in LibreOffice; the names and properties read back.

## Size and packaging (decision 1, before any code)

The budget figure after part 3 is 434668 section bytes against
the Milestone 2 ceiling of 458752 (`records/size-m2p3-2026-09-27-
b728fdf.md`): 24084 of headroom, held for fixes. The outline puts
part 4 behind features, so nothing here is in the core budget.
Planning's estimate for the design below is 15000 to 20000 section
bytes whole: the names' read and write with the workbook table
(4000), the hyperlinks with the relationship and the root retag
(5000), the properties with the part's creation and namespace
resolution (5000), the two protections with the hash (3000). Part
3 measured 57868 for its rows alone against a guess of 30000 to
40000; the guess is a guess, and nothing hangs on it here since
the cost sits outside the budget.

- A (recommended): one Cargo feature `metadata`, off by default,
  declared beside `dates` and `rowcol` with no dependency of its
  own, holding all four capabilities; one probe measuring its cost
  beside the core; the core probe unchanged. The core's growth for
  the part is what the workbook table generalisation and the
  feature gates add, printed by the size log and expected near 0;
  above 2000 section bytes planning is told in the report. One
  feature, one figure, one measurement.
- B: four features (`names`, `links`, `properties`, `protection`),
  four probes, four figures. Finer for a consumer that wants links
  alone; four times the size-log work, and a consumer that wants
  two of them pays the shared code twice in reasoning about it.
- C: properties and protection in the core (planning guesses 8000
  of the 24084) with names and links behind `metadata`. Spends the
  fix headroom on the two capabilities least asked for.

## Facts planning checked (2026-09-27)

Over the 27 corpus workbooks (26 `.xlsx`, 1 `.xltx`), by element
shapes and counts with Python's `zipfile` and `ElementTree` over
the workbook, worksheet, rels and `docProps` parts, never a name's
text, a link's target, a property's value or a cell's text:

- `definedNames` in 16 workbooks, 160 names: `name` on all, 38 with
  `localSheetId`, 30 with `hidden`, 24 with `function` and
  `vbProcedure` (one workbook's macro names), 33 built-in
  (`_xlnm.` prefix). No other attribute. The workbook rewrite of
  part 1 already remaps `localSheetId` and drops a removed sheet's
  names.
- The workbook root: `<workbookProtection>` in 17 workbooks, 16 of
  them with no attribute at all (a producer's empty element), one
  with `lockWindows`; none with a password. Every root's children
  in the order of 18.2.27; `BEFORE_CALC_PR` (part 6) names the
  first nine of that order.
- `hyperlinks` in 33 worksheets, 265 `hyperlink` elements: `ref`
  on all (one a range, the rest one cell; one sheet has two on the
  same `ref`), `display` 264, `r:id` 198, `location` 67, none with
  both, `tooltip` 61, an `xr:uid` 65. All 198 `r:id` targets are
  hyperlink relationships with `TargetMode="External"`. The
  container's preceding root child is `dataValidations` (28),
  `sheetProtection` (2) or `sheetData` (3): schema order in every
  case. Every worksheet root binds a prefix to the relationships
  namespace; the template worksheet (`specs/0016`) binds none.
- `sheetProtection` in 7 worksheets, none with a password: `sheet`,
  `objects` and `scenarios` on 6, one with `insertColumns`,
  `insertRows`, `deleteColumns`, `deleteRows` and `autoFilter`. No
  `algorithmName` anywhere; no `protectedRanges`.
- `docProps/core.xml` in all 27 (an `Override` for it in 18, a
  `Default` for `xml` covering the rest), `docProps/app.xml` in all
  27, `docProps/custom.xml` in 18 (11 properties, every value
  `lpwstr`). The package relationship's target is relative
  (`docProps/core.xml`) in 17 and absolute (`/docProps/core.xml`)
  in 10. Core children by count: `title` 24, `description` 18,
  `subject` 18, `language` 17, `created` 10, `modified` 10,
  `creator` 9, `lastModifiedBy` 9, `keywords`, `category` and
  `contentStatus` 1 each. `app.xml`'s `TitlesOfParts` (10
  workbooks) names every sheet in each; part 1 leaves it alone on
  a rename, and so does this part.
- Cell protection: part 2's `Protection` on a `Style` writes
  `locked` and `hidden` on a cell format, so a caller can unlock
  cells before protecting a sheet; nothing here adds to that.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. Under decision 1 A
everything below sits behind a Cargo feature `metadata`, off by
default: the types `DefinedName`, `LinkTarget`, `Property`,
`Properties` and `SheetProtection`, ten `Package` calls, one
`Workbook` accessor, one function `legacy_password_hash`, and two
`package` constants, `HYPERLINK` (the relationship type name the
link writes) and `CORE_PROPERTIES` (P3 verification ruling 3),
with the messages of this part in the catalogue
marked as the feature's. The core builds, tests and links without
it; the tool `emend-copy` turns it on as it does `dates` and
`rowcol`. Nothing existing changes signature. One core change: the
workbook rewrite's creation of an element generalises from
`BEFORE_CALC_PR` to the one table and rule of "Creating an element
in the workbook", with `calcPr` unchanged in behaviour.

### Defined names

`DefinedName { name: String, text: String, sheet: Option<u32>,
hidden: bool }`, derived `Clone`, `PartialEq`, `Eq`, `Debug`:
`sheet` is the `localSheetId` as written, an index into the file's
sheet list (`Workbook::sheets`), `None` for a workbook-level name;
`text` is the element's content, unescaped, stored and never
parsed. `Workbook::defined_names(&self) -> &[DefinedName]` answers
with the file's names in document order; `Package::workbook` reads
them with the sheets, so a workbook whose `<definedName>` carries
a `localSheetId` that is not a decimal `u32` is refused there with
part 1's message (P3 ruling 10 of `specs/0016`), which today is
refused at the write.

- `add_defined_name(&mut self, name: &str, text: &str, sheet:
  Option<&str>) -> Result<(), Error>`: records a name. `name` is
  checked at the call: 1 to 255 characters; the first a letter, `_`
  or `\`; the rest letters, digits, `.`, `_`, `\` or `?`; not
  `_xlnm.`-prefixed (the built-in names have rules of their own and
  are out of scope); not a cell reference (`CellRef::parse` accepts
  it, ignoring case) and not of the `R1C1` form (`R` or `r`,
  optional digits, `C` or `c`, optional digits, ignoring case),
  nor `R` or `C` alone, which Excel reserves (P1 verification
  ruling 4);
  "letter" and "digit" by `char::is_alphabetic` and
  `char::is_numeric`, an approximation of Excel's rule as part 1's
  case folding is. `text` is non-empty and is written as a cell's
  text is (`specs/0011`), escaped; a leading `=` is the caller's to
  leave out, as Excel stores names without one. `sheet` names a
  sheet in the projected list (`Package::sheets`), refused with
  part 1's "no sheet of that name" otherwise; the name is recorded
  against the sheet's name and resolved to its index in the output
  list at the write, so it composes with an add, a move or a
  removal in the same write. Two additions of one name and scope in
  the same write, and an addition of a name and scope the file has
  (ignoring case, as Excel compares), are refused at the write.
- `remove_defined_name(&mut self, name: &str, sheet: Option<&str>)
  -> Result<(), Error>`: records a removal; `sheet` as above. The
  write drops the `<definedName>` whose `name` equals (ignoring
  case) and whose scope matches (a `localSheetId` naming that sheet
  in the file's list, or none for `None`); none is refused ("no
  defined name of that name and scope"). A name recorded for
  removal and again for addition in one write is refused as a
  duplicate; the caller writes twice.

The write: in the workbook rewrite's `<definedNames>` region, the
file's elements are copied as part 1 leaves them (remapped or
dropped with their sheets), a removed one skipped whole, and the
additions written back to back before the container's end tag as
`<definedName name="..">text</definedName>` or `<definedName
name=".." localSheetId="n">text</definedName>`, `name` escaped, in
the order recorded, with the root's prefix. A `<definedNames>` left
empty is dropped; one that is absent is created ("Creating an
element in the workbook"); a second is refused. A name added or
removed is a formula change under `specs/0013`'s rule (the chain
dropped once when the workbook has one, `fullCalcOnLoad` set),
since a formula's cached value may depend on the name (decision
2). Nothing that uses a name (a formula, a validation's list, a
print setting) is touched or checked, as openpyxl.

### Hyperlinks

`LinkTarget<'a>`: `External(&'a str)` (a URL, a file path or a
`mailto:` address, written as a relationship with
`TargetMode="External"`, as all 198 corpus links are) or
`Location(&'a str)` (a place in the workbook, `'Sheet 1'!A1` or a
defined name, written as the `location` attribute). Derived
`Clone`, `Copy`, `PartialEq`, `Eq`, `Debug`.

- `set_hyperlink(&mut self, part, reference: CellRef, target:
  LinkTarget, tooltip: Option<&str>) -> Result<(), Error>`: records
  a link on one cell of a worksheet part (`part` as `set_cell`
  names it). An empty target or location is refused at the call
  ("a hyperlink target or location is empty"), as are a target or
  tooltip holding a character XML cannot carry (part 4's message)
  and a cell off the grid (part 3's index message; P2 verification
  ruling 1); nothing else about the text is checked (a URL is not
  parsed), and it is written escaped as an attribute. The last
  record for a cell in one write
  wins.
- `remove_hyperlink(&mut self, part, reference: CellRef) ->
  Result<(), Error>`: records the removal of the link whose `ref`
  is exactly that one cell; none at the write is refused ("no
  hyperlink on that cell"). A link whose `ref` is a range (one in
  the corpus) is never matched by a cell and is copied raw.

The write, in the worksheet pass: an existing `<hyperlink>` whose
`ref` is exactly a recorded cell is skipped whole (a set replaces,
a removal drops), and when it carries an `r:id` the relationship
that id names is removed from the part's rels part, so no
relationship is left without an element (part 1's rule for removed
parts, applied to a removed link). Every set is appended before the
container's end tag, in the order recorded, as `<hyperlink
ref="A1" r:id="rIdN" tooltip=".."/>` for an external target, the
relationship of type `HYPERLINK` recorded on the worksheet's rels
part by `add_relationship` (`external` true; the rels part created
when absent, as that path does today), or `<hyperlink ref="A1"
location=".." tooltip=".."/>` for a location; `tooltip` only when
given; no `display`, which is optional in 18.3.1.47 and which Excel
fills from the cell when it saves. The `r:id` attribute takes the
prefix the root binds to the relationships namespace; when the
root binds none (the template worksheet, and any sheet an earlier
write's `add_sheet` created), the root's start tag is retagged
(`with_attribute`, part 3's helper) with `xmlns:r` bound to it and
the attribute is written `r:id` (decision 3), or `xmlns:r1`,
`r2` and so on when `r` is bound to another namespace (P2
verification ruling 3). The relationship type's form and the
declaration's namespace form (transitional or strict) both follow
the form of the worksheet's own relationship in the workbook's
rels part, one signal for both (P2 verification ruling 2). A
sheet `add_sheet` adds in the same write takes a location link,
written onto its template part, and refuses an external one with
the existing rule that a relationship's source is a part the
package holds, since the part is not yet in the archive; the
caller writes twice, as for a shift (P2 verification ruling 4).
A `<hyperlinks>`
left empty is dropped; one that is absent is created in schema
position (part 3's table); a second is refused. The cell itself is
not touched: no value, no style (Excel's `Hyperlink` cell style is
the caller's to apply with part 2's font, and openpyxl applies
none either); a value on the cell composes with `set_cell` in the
same write. On a worksheet with a row or column shift in the same
write the link is "any other edit" of part 3's one-shift rule and
refused with that message; `hyperlink ref` moves under the shift
rule as decision 2 A of part 3 already has it.

### Document properties

`Property`, an enum of thirteen core properties, derived `Clone`,
`Copy`, `PartialEq`, `Eq`, `Debug`, `Hash`, each mapped to its
element (ECMA-376 Part 2, 11): `Title`, `Subject`, `Creator`,
`Description` and `Language` in the Dublin Core elements namespace
(`dc`); `Keywords`, `LastModifiedBy`, `Revision`, `Category`,
`ContentStatus` and `Version` in the core-properties namespace
(`cp`); `Created` and `Modified` in the Dublin Core terms namespace
(`dcterms`), each written with `xsi:type="dcterms:W3CDTF"`.
`Properties` holds the values read, `Properties::get(&self,
Property) -> Option<&str>`, derived `Clone`, `Default`, `PartialEq`,
`Eq`, `Debug`.

- `properties(&mut self) -> Result<Properties, Error>`: reads
  `docProps/core.xml`, found through the package relationship of
  type core-properties (`_rels/.rels`, source empty; the type
  constant joins the catalogue's eight as a two-form array like
  them, the package form every corpus package writes first, then
  the `officeDocument` form; the creation writes the first; P3
  ruling 4), the target resolved by `target_part` (both corpus
  forms); an empty `Properties` when there is no such relationship
  (`Package::new()`); a relationship whose part is missing refused
  with part 1's message, by the read and by a write that edits a
  property (P3 ruling 1). Each element
  of the thirteen is read by namespace and local name, its text
  unescaped; unknown children are ignored; a second element of one
  name is refused ("a core properties part has two elements of one
  property"). The `Properties` is a snapshot of the file, not of
  the pending edits.
- `set_property(&mut self, property: Property, value: Option<&str>)
  -> Result<(), Error>`: `Some(v)` records the value, `None` its
  removal; the last record for a property in one write wins. A
  `Created` or `Modified` value must be exactly the form
  `YYYY-MM-DDThh:mm:ssZ` (digits in every digit position, the
  letters as written), checked at the call ("a property value is
  not valid"); any other property takes text as a cell does, a
  forbidden character refused at the call (P3 ruling 5).

The write: when the workbook has a core-properties relationship,
the part is rewritten in one pass, the root's start tag copied
raw, each child of one of the thirteen names replaced (its start
tag kept byte for byte, an empty-element tag opened up to take
content, its content the new text escaped) or
skipped whole (a `None`), every other child copied raw, and each
recorded property the part lacks appended before the root's end
tag as `<p:name>text</p:name>`, or `<dcterms:created
xsi:type="dcterms:W3CDTF">..</dcterms:created>`, with the prefixes
the root binds to those namespaces (the order of children is free,
`coreProperties` being an `all` group); a root that binds no
prefix for a namespace an appended element needs is refused ("a
core properties part binds no prefix for a namespace the edit
needs"); the `xsi` namespace bound only as the root's default
namespace is no prefix an attribute can take, so an appended date
is refused the same way (P3 ruling 6). A core part this write
removes or replaces: "an edit to a removed part", as a cell edit
would be. When the workbook has no such relationship (decision 5),
the write adds `docProps/core.xml` with `add_part` as a fixed root
binding `cp`, `dc`, `dcterms`, `dcmitype` and `xsi` (the prefixes
Excel writes) around the recorded elements, the relationship on
the package with a relative target as the template's own are, and
the `Override` with the core-properties content type. `app.xml`
and `custom.xml` are never touched: `TitlesOfParts`, `Company` and
the custom properties stay as they are, as openpyxl leaves them.

### Protection

`SheetProtection`, the options of 18.3.1.85 as sixteen `bool`
fields named as the attributes are (`sheet`, `objects`,
`scenarios`, `format_cells`, `format_columns`, `format_rows`,
`insert_columns`, `insert_rows`, `insert_hyperlinks`,
`delete_columns`, `delete_rows`, `select_locked_cells`, `sort`,
`auto_filter`, `pivot_tables`, `select_unlocked_cells`), each
`true` when that action is locked; `SheetProtection::default()` is
the schema's default (everything locked but `objects`, `scenarios`
and selecting locked and unlocked cells, so it writes exactly
`<sheetProtection sheet="1"/>`; P3 ruling 3, where the earlier
wording said Excel's dialog default, which locks `objects` and
`scenarios` too), derived `Clone`, `Copy`, `PartialEq`, `Eq`,
`Debug`.

- `protect_sheet(&mut self, part, password: Option<&str>, options:
  SheetProtection) -> Result<(), Error>`: records the protection of
  a worksheet part; `unprotect_sheet(&mut self, part) -> Result<(),
  Error>` its removal; the last record for a part wins.
- `protect_workbook(&mut self, password: Option<&str>, structure:
  bool, windows: bool) -> Result<(), Error>` and
  `unprotect_workbook(&mut self)`, likewise for the workbook.
- `legacy_password_hash(password: &str) -> u16`: the hash Excel
  stores in `password` and `workbookPassword`, the legacy algorithm
  of ECMA-376 first edition Part 4 that openpyxl and LibreOffice
  write, restated here: starting from zero, for each character at
  1-based position i, take its code point shifted left by i, fold
  the bits above bit 14 back into the low fifteen (a rotation
  within fifteen bits), and exclusive-or it in; then exclusive-or
  the length, then the constant `0xCE4B`. The function is public
  so a test can pin it against known values the log prints. A
  password is 1 to 255 characters, every one printable ASCII,
  refused at the call otherwise ("a password is not valid"): the
  legacy hash is defined on single-byte characters and this keeps
  every input inside that (decision 4).

The write: `<sheetProtection>` written in the worksheet pass with,
in the schema's order, for each option locked by default in the
schema that the caller unlocked, `name="0"`, and for each option
unlocked by default that the caller locked, `name="1"` (`sheet`
is one of the latter, so `sheet="1"` when `sheet` and nothing
when the caller clears it: the element is then the schema's
unprotected one with options, written as asked; P3 ruling 3), so
`SheetProtection::default()` with no password is exactly
`<sheetProtection sheet="1"/>` and the corpus's commonest form
(`sheet`, `objects`, `scenarios`) is the default with `objects`
and `scenarios` locked; `password="XXXX"` (four upper-case hex
digits of the hash) first when given. `objects` and `scenarios`
default unlocked in 18.3.1.85; the other thirteen default locked
except `selectLockedCells` and `selectUnlockedCells`. An existing
`<sheetProtection>` is replaced whole (a protect) or dropped (an
unprotect; none is not an error, the call says what the output
has); an absent one is created in schema position; a second is
refused. `<workbookProtection>` likewise in the workbook rewrite,
as `workbookPassword="XXXX"` (when given), `lockStructure="1"`
(when `structure`), `lockWindows="1"` (when `windows`), an element
with neither lock and no password still written (Excel writes it
so); the 16 empty corpus elements are the shape an unprotect
drops. Neither is a formula change. Protection and a row or column
shift on one worksheet in one write: refused as "any other edit".

### Creating an element in the workbook

`<definedNames>` and `<workbookProtection>` can be absent (11 and
10 corpus workbooks; the template has neither). An absent element
the write needs is created in schema position by part 3's rule
applied to the workbook root: written immediately after the end of
the last root child of the main namespace whose name comes earlier
in the sequence of 18.2.27, or after the root's start tag when
none does, which is part 6's placement for `calcPr`, kept so its
bytes do not change (P1 verification ruling 1: whitespace, a
comment or another namespace's child between two children stays
after the created element, where the earlier wording put it
before).
The sequence is one static table of the 19 names (`fileVersion`,
`fileSharing`, `workbookPr`, `workbookProtection`, `bookViews`,
`sheets`, `functionGroups`, `externalReferences`, `definedNames`,
`calcPr`, `oleSize`, `customWorkbookViews`, `pivotCaches`,
`smartTagPr`, `smartTagTypes`, `webPublishing`, `fileRecoveryPr`,
`webPublishObjects`, `extLst`); `calcPr`'s creation moves to the
same rule (its place is the first nine of the table, which is what
`BEFORE_CALC_PR` says), so `BEFORE_CALC_PR` goes and the held-back
tokens of part 6 are held for the general rule. The bytes `calcPr`
produces on every existing test do not change; the tests say so.
This is the one core change of the part. A created element takes
the root's prefix.

### Errors

Eleven new `Error::Package` messages, in the catalogue as the earlier
parts' are, one constant each:

- "a defined name is not valid" (at the call)
- "a defined name of that name and scope already exists"
- "no defined name of that name and scope"
- "a hyperlink target or location is empty" (at the call)
- "no hyperlink on that cell"
- "a property value is not valid" (at the call)
- "a password is not valid" (at the call)
- "a workbook has two definedNames or workbookProtection elements"
- "a worksheet has two hyperlinks or sheetProtection elements"
- "a core properties part binds no prefix for a namespace the edit
  needs"
- "a core properties part has two elements of one property"

Reused: part 1's "no sheet of that name" (a scope or a removal
naming no sheet), "a cell edit to a part this write replaces"
(every worksheet edit here on a replaced or removed part), the
missing-part refusal of a relationship whose target is absent,
part 4's "a text holds a character XML cannot carry" at a name's,
a link's or a property's call and part 3's index message at a
name's or a link's call, part 3's one-shift message, part 2's
unbound-prefix refusal wherever an element this part reads or
retags carries one (a kept `<hyperlink>` is read for its `r:id`
since P2 ruling 6; P3 ruling 7), and `Error::Xml` for a value that
is not UTF-8.

### 32-bit and streaming

Every sheet index is `u32`; an element ordinal within a part is
`usize`, as elsewhere. The worksheet and workbook passes stream as
today, one token at a time through the temporary file; a removed
`<definedName>` is skipped as its tokens arrive; the kept content
of an edited `<hyperlinks>` is held until its end tag, as part 3
holds `<mergeCells>`, so a container left empty can be dropped
(bounded by one container: 60 links at most in the corpus; P2
verification ruling 5);
the appended elements are the recorded edits, held as they were
recorded. The core properties part is small (under 2 KB in every
corpus workbook) and is still streamed by the same pass shape,
since nothing needs the whole. `properties` and `defined_names`
hold what they read, which is the caller's snapshot.

### Size

One probe, under decision 1 A. `probe-emend-metadata-write-xml`
(feature `metadata` of the probe crate, which turns on
`emend/metadata` as `rowcol` does `emend/rowcol`, built alone with
its own feature set): `probe-emend-format-write-xml`'s work,
unchanged, then on the same package before its write
`add_defined_name` (workbook-level, the text from the command
line), `set_hyperlink` (external, on the cell from the command
line), `set_property(Title, ..)`, `protect_sheet` with the default
options and a password from the command line, and
`protect_workbook`, then after the write the output opened and the
defined-name count and the title read back and printed, so both
read sides are linked and nothing is dead. The size log measures
it as part 3's did, classifies every probe against `records/size-
m2p3-2026-09-27-b728fdf.md` (sixteen probes), prints the feature's
cost (the metadata probe minus `probe-emend-format-write-xml`
rebuilt at the same commit), the core's growth for the part (the
format probe at the commit minus the same probe rebuilt at
bcaa31d, the commit that changed the probe's own source, both
printed; expected near 0; P1 verification ruling 8), the whole
growth since b728fdf (the format probe at the commit minus 967884,
probe bytes included), and the budget figure (the format
probe minus `probe-zip-xml`) against 458752, which this part does
not move by design.

### Harness runs

`tools/emend-copy` gains four operations, given after the two
paths as the others are; none takes a reference, a name or a
number from its command line, and every constant below is the
tool's own. W is the first worksheet part in `<sheets>` order and
S its sheet name (as `--sheet-sample` finds them), M its largest
row number as the tool's cell read gives it; a reference to S in a
name's text or a location is built by the tool as `'S'!` with
every apostrophe in S doubled, the form Excel accepts for every
name, and is never printed. A refusal exits 2 with no output, as
every sample does. M comes from the file, so a worksheet whose M
sits at the grid's last row puts the link cell past the grid and
the sample is refused whole with part 3's index message (P4 ruling
5 of `specs/0018`); no corpus workbook has that shape.

- `--names-sample <in> <out>` (P1): `add_defined_name("emend_global",
  "'S'!$A$1", None)`, `add_defined_name("emend_local", "'S'!$B$2",
  Some(S))`; writes; reopens the output and prints `names` with
  `names_before`, `names_after` (`+ 2`, asserted, the output
  removed on a difference as `--style-sample` does), `local_after`
  (the names with `sheet` equal to W's index, `+ 1` over the input,
  asserted), `chain_before`, `chain_after` (1 or 0), `full_calc`
  (the output's flag); then, on the reopened output in memory,
  `remove_defined_name("emend_local", Some(S))`, a write into a
  `Cursor`, and that reopened for `names_removed` (equal to
  `names_before + 1`, asserted), so the removal is exercised on
  every corpus workbook without a second harness output.
- `--link-sample <in> <out>` (P2): `set_cell(W, (M + 1, 1),
  Text("emend link"))`, `set_hyperlink(W, (M + 1, 1),
  External("https://example.com/"), Some("emend sample"))`,
  `set_hyperlink(W, (M + 1, 2), Location("'S'!A1"), None)`; writes;
  reopens; prints `link` with `cells_before`, `cells_after` (`+ 1`,
  asserted), `links_before`, `links_after` (`hyperlink` elements in
  W, counted by the tool with `quick-xml`; `+ 2`, asserted),
  `external_rels_before`, `external_rels_after` (relationships of
  type `HYPERLINK` with `TargetMode="External"` in W's rels, all
  198 corpus ones; `+ 1`, asserted), `r_bound_before`
  (1 when W's root binds a relationships prefix, else 0).
- `--props-sample <in> <out>` (P3): `set_property(Title,
  Some("emend sample"))`, `set_property(Keywords, Some("emend"))`,
  `set_property(Modified, Some("2026-01-01T00:00:00Z"))`; writes;
  reopens; prints `props` with `core_before` (1 when the input has
  a core-properties relationship), `title_after` (`1` when the
  reopened title equals the constant, asserted), `keywords_after`,
  `modified_after` likewise, `props_before` and `props_after` (the
  count of the thirteen present).
- `--protect-sample <in> <out>` (P3): `protect_sheet(W,
  Some("emend"), SheetProtection { sort: false, auto_filter: false,
  ..Default::default() })`, `protect_workbook(Some("emend"), true,
  false)`; writes; reopens; prints `protect` with
  `sheet_prot_before`, `sheet_prot_after` (`sheetProtection`
  elements in W, counted with `quick-xml`; `1`, asserted),
  `book_prot_before`, `book_prot_after` (`workbookProtection`
  elements; `1`, asserted), `hash` (the hash of the constant, as
  four hex digits, a fixed value the run log checks).

The runner and the record script are unchanged; each run names
its operation through `--tool-arg` and one `--label`, two run
directories for P3. Expected results, `lo` and `lo_in` `ok` in
every case; `descriptor` alone allowed on a changed entry;
`order_preserved` `yes`; `files_identical` `no`; and per sample:

- names: the workbook part `changed`; when the input has a chain,
  the chain entry `removed` and the workbook rels part and
  `[Content_Types].xml` `changed` as `--formula-sample` expects
  them; every other entry `identical`.
- link: W `changed`; W's rels part `changed` when the input has one
  and `added` when it does not; every other entry `identical`.
- props: the core part `changed` when the input has one (all 27
  do), else `added` with `_rels/.rels` and `[Content_Types].xml`
  `changed`; every other entry `identical`.
- protect: W and the workbook part `changed`; every other entry
  `identical`.

The run log's own check over the extracted input and output parts,
with `ElementTree` and byte spans and a control that reports a
planted difference per check, printing no name's text, no target,
no value and no reference from the corpus:

- P1: the workbook part identical outside `<definedNames>` and
  `<calcPr>`; the input's `definedName` elements a byte-identical
  prefix of the output's; two more, each with exactly the expected
  attributes (the log builds the expected start tags itself, the
  `localSheetId` W's index) and text of the expected length; the
  container created in schema position when absent (checked
  against the table); `calcPr` as `--formula-sample`'s check has
  it, in every workbook (decision 2 A: the flag set whether or not
  the input has a chain or a formula; P1 verification ruling 5).
- P2: every root child of W other than `hyperlinks`, `sheetData`
  and (when created) the root start tag byte-identical; the input's
  `hyperlink` elements a byte-identical prefix; two more with the
  expected attributes, the `r:id` naming a relationship the output
  rels part has with the expected type, target and mode; the root
  start tag identical, or identical but for one added `xmlns:r`
  declaration when `r_bound_before` is 0; `sheetData` identical
  but for the one created cell (part 4's check).
- P3 props: the core part's root start tag identical; every child
  other than `title`, `keywords` and `modified` byte-identical and
  in the input's order; those three present once each with the
  expected text, in place when the input had them and appended
  when not; the `xsi:type` on `modified`. P3 protect: every root
  child of W other than `sheetProtection` byte-identical and the
  one `sheetProtection` exactly the expected bytes (the log builds
  them, `password` the pinned hash); the workbook part identical
  outside `workbookProtection` and that element exactly the
  expected bytes, replacing the input's when it had one.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. From part 8's
decision 3, every phase's run log also runs the edit open check on
the host over that phase's keep.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m2p4a-<phase>/` (`gate.log`,
`test.log` on both targets with the feature off and on,
`negative.log`, `run.log`, `scan.log`, `size.log` in P4); every
rule the earlier parts added (`specs/0018` "Phases" lists them,
and its close-out ruling 4 carries these: every log pins the spec's
hash; a figure labelled another log's is read from that log or
labelled as measured here; an exemption counts tokens in the form,
never lines; a superseded attempt stays on disk under
`<home>/.cache/emend-build/<phase>/`; no regex stand-in is printed
in the placeholder form; planning runs no cargo while a logged
script or a pre-check runs); the scan log last; at most two
independent checks; report; STOP. No push during the part; the
close-out push follows P4 and carries the record commit 5e57db1 of
part 3's close-out with this part's.

Part 4a's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0018-rows-and-columns.md` as planning left them after part
3's close-out (the close-out rulings and the ROADMAP row), and this
spec. Part 4a's first code commit carries, before any other
change, the four items of `specs/0018`'s close-out ruling 4: the
two probes' `USAGE` strings naming their own binaries; the
`package` header's sentence that a shift is a formula change when
it moves or removes an `<f>`, in the formula-change paragraph;
checked additions in place of the two probes' `+ 1`; and the
`Area::cells` saturation test.

### P1: defined names, creating an element in the workbook

The doc commit; the first code commit as above; then the feature
`metadata` declared in the crate and turned on by the tool,
`DefinedName` and `Workbook::defined_names`, the workbook table and
creation rule with `calcPr` moved onto it, `add_defined_name` and
`remove_defined_name` with the name rule, the scope resolution
against the projected list, the formula-change rule, the messages
of this phase, and `--names-sample`. Tests on byte-built parts:
the names read from a part with two names (one local, one hidden)
and from one with none; a name added to a part with a container
(appended, the file's children byte-identical), to the template
part (created before `calcPr` when the write also creates one,
else before the root's end; the log prints the output), and to a
part with `extLst` (created before it); a local name resolved
against a projected list after an add and a move in the same
write; a removal of a present name (case-insensitively), of a local
name by scope (the global of the same name kept), of an absent one
(refused), the last name dropping the container; a duplicate
against the file and against a pending addition (refused), an add
and a remove of one name in one write (refused); the name rule on
each clause (a reference, an `R1C1` form, a built-in, a bad first
character, 256 characters); a name added and the chain dropped and
the flag set, and `calcPr` created where the file has none, with
every part 6 `calcPr` test unedited and passing; the tool's line.
The gate: the crate built and tested with the feature off and on,
on both targets, and the `pub fn` and `pub const` first lines
listed with it off (the part's calls absent) and on (present), as
part 3's P3 did; every later log that runs the crate's tests runs
them with the feature on. The negative log through the binary:
`--names-sample`
on a fixture from the template holding `emend_global` already
(refused as a duplicate) and on one with two `<definedNames>`
(refused), each with a control. The run log: `--names-sample` on
both targets, the checks as "Harness runs", the keep and the edit
open check, recorded as `records/m2p4a-p1-<utc-date>-<shortsha>.md`
and `.txt`.

### P2: hyperlinks

The doc commit (the P1 rulings and verification); then
`LinkTarget`, `HYPERLINK`, `set_hyperlink` and `remove_hyperlink`,
the element skipping and appending in the worksheet pass, the
relationship removal for a dropped `r:id`, the root retag for an
unbound relationships prefix, the messages of this phase, and
`--link-sample`. Tests on byte-built parts: an external link
appended with its relationship (the rels part's bytes compared, the
id one past the largest), a location link, both on a part with a
container and on the template part (the container created before
`pageMargins` and at the root's end, the root retagged with
`xmlns:r`, the rels part created), a tooltip written escaped; a set
on a cell that has a link (the old skipped, its relationship
removed, the new appended); a removal of a present link with and
without `r:id`, of an absent one (refused), of one whose `ref` is a
range (not matched: refused), the last link dropping the container;
a second `<hyperlinks>` refused; a link on a part with a pending
row insertion refused with part 3's message; a link and a cell set
on the same cell in one write; the tool's line. The negative log:
`--link-sample` on a fixture from the template with two
`<hyperlinks>` (refused) and on one whose `<hyperlinks>` carries a
prefix no declaration binds (refused, part 2's message), each with
a control. The run log: `--link-sample` on both targets, recorded
as `records/m2p4a-p2-<utc-date>-<shortsha>.md` and `.txt`.

### P3: properties and protection

The doc commit (the P2 rulings and verification); then `Property`,
`Properties`, `properties`, `set_property`, the core part's
rewrite and creation with its relationship and override,
`SheetProtection`, `legacy_password_hash`, the four protection
calls, both elements' writing in the two passes, the messages of
this phase, and the two samples. Tests: the thirteen properties
read from a part with all of them and from the template (empty);
each replaced in place (the start tag byte-identical), removed,
appended with the right prefix, the dates with `xsi:type`; a root
binding no `dcterms` prefix (refused on an append, allowed on a
replace); the part created for the template with its relationship
and override (the output's three parts compared to expected
bytes); a second `title` refused; the date form on each digit
position and on a missing `Z`; the hash pinned on three passwords
of known hash, printed by the log; a password of 256 characters
and one with a non-ASCII character refused; `sheetProtection`
written for the default options with and without a password, for
every option flipped one at a time (sixteen expected elements the
log prints), replacing an existing one and created in schema
position before `protectedRanges`, `mergeCells` and `pageMargins`;
an unprotect dropping it and doing nothing when absent; a second
refused; `workbookProtection` written in each of the four lock
forms with and without a password, replacing an empty existing
one, created after `workbookPr` and before `bookViews`, dropped;
the protection on a part with a pending shift refused; the tool's
two lines. The negative log: `--props-sample` on a fixture whose
core part binds no `dcterms` prefix (refused), `--protect-sample`
on one with two `<sheetProtection>` (refused), each with a control.
The run log: both samples on both targets in two run directories,
recorded as `records/m2p4a-p3-props-<utc-date>-<shortsha>.md`,
`records/m2p4a-p3-protect-...` and `.txt`.

### P4: the catalogue and limits, the probe, the size record

The doc commit (the P3 rulings and verification); then the first
code commit with the items P3's rulings carry (the missing core
part refused on the write, ruling 1, first among them); then the
catalogue and the limits in the `package` module header (the
caller's picture: what a name, a link, a property and a protection
do and do not touch, `app.xml` and `custom.xml` untouched, the
legacy hash is not encryption, what is read back, the Excel check
pending), the probe of "Size" measured as it says, recorded as
`records/size-m2p4a-<utc-date>-<shortsha>.md` with `size.log`
printing every figure the record carries, installing the record
and printing its sha256, and the seventeen probes classified
against part 3's record; the feature's cost and the core's growth
stated in the record as facts, the budget figure against 458752
beside them; the negative log through the binary with every refusal
of this part reachable through the binary run once (the fixture
cases whose refusals P1 to P3's negative logs reached, five of the
eleven messages, and the missing core part, `--props-sample` on a
fixture whose core relationship names a part the archive lacks,
refused with "no such part", P3 ruling 1; the at-call refusals are
not reachable, since their inputs are the tool's constants, and
the log says so; the bare template's created core part compared
by sha256 to the bytes the crate test pins, P3 verification
ruling 7); the
corpus run of all four samples again on both targets with the
release binaries the size log hashed, every output byte-identical
to the phase run directories' outputs by sha256, in four run
directories, recorded as `records/m2p4a-p4-<sample>-<utc-date>-
<shortsha>.md` and `.txt`. The report states the feature's cost,
the core's growth, and the budget figure against the ceiling.

### Close-out push (after P4)

As part 3's: the self-scan rule on the push and ci-run scripts,
`ci-run.log`, then the post-stage `scan.log` over the pushed range
with `push.log` and `ci-run.log` as its siblings, then one record
commit holding the three logs, the `ANTHROPIC_API_KEY` check. The
push carries 5e57db1 and this part's commits, on planning's word
and the maintainer's authorisation typed in the implementing
session. Before the push, the Excel check for Milestone 2 parts 1
to 3 (every corpus row of every sample, on the laptop), deferred
by the maintainer on 2026-09-27 (decision 6), is run or deferred
again by a decision recorded here; this part's four samples join
that check's list.

## Decisions for the maintainer

Decided by the maintainer 2026-09-27: 1 to 5 as recommended (A in
each), in the reply "i agree with recs"; 6 recorded.

1. Packaging, as "Size and packaging" states: A (recommended), one
   feature `metadata`, one probe; B, four features; C, properties
   and protection in the core.
2. A defined name added or removed: A (recommended), a formula
   change under `specs/0013`'s rule (the chain dropped, the flag
   set), so a cached value that depended on the name is
   recomputed when Excel opens the file; B, nothing, which is
   openpyxl's behaviour and leaves a stale cached value until the
   user recalculates.
3. A hyperlink on a worksheet whose root binds no relationships
   prefix (the template's, and every sheet `add_sheet` creates): A
   (recommended), the root start tag retagged with `xmlns:r`; B,
   refused, so a link on a new sheet needs a template change
   first.
4. The password hash: A (recommended), the legacy sixteen-bit hash
   openpyxl and LibreOffice write, which Excel accepts in every
   version, printable ASCII passwords only; B, the `SHA-512`
   form Excel 2013 and later write (`algorithmName`, `hashValue`,
   `saltValue`, `spinCount`), which needs an in-crate SHA-512
   behind the feature, planning guesses 3000 to 6000 section bytes,
   with the salt from the caller since the core has no randomness;
   C, both, the caller choosing. Neither is encryption; A's hash
   is trivially reversed and B's is not, but the lock is advisory
   either way and Excel removes it on request.
5. Properties on a workbook with no core part (`Package::new()`):
   A (recommended), the part created with its relationship and
   override; B, refused, so a new workbook never gets a title.
6. The Excel check for parts 1 to 3: deferred on 2026-09-27 by the
   maintainer ("can i skip laptop check for now"), to run before
   this part's close-out push at the latest, or be deferred again
   by a decision recorded here. Recorded, not asked.

## Planning rulings during P1 (2026-09-27)

1. Q1, carried item 4 (the `Area::cells` saturation test): A. The
   test exists in the tree, `an_area_cell_count_saturates_rather_
   than_overflowing` in `package/tests/rowcol.rs`, added in 94cf3b6
   (part 3 P2, from P1 verification ruling 6), compiled without the
   feature and passing in `records/logs/m2p3-p4/test.log` (lines
   1344 and 2099); planning verified all three from the tree and
   the log. Check 1's F3 spoke of the P4 diff, and planning's
   close-out ruling 4 of `specs/0018` carried it as if it spoke of
   the tree: planning's error. The first code commit carries items
   1 to 3 alone and its message names the existing test and 94cf3b6
   as the reason item 4 adds nothing; no duplicate test. Carried:
   a carried item is checked against the tree before it is carried,
   not against the check's remark alone.
2. Q2, counts in three P1 commit messages (the rule of `specs/0018`
   P2 verification ruling 5): A. The messages are rewritten now,
   while every commit is local and no log cites one, by a
   non-interactive rebase that changes messages alone; `specs/0001`
   P1R is the precedent for rewriting an unpushed message, and
   part 3's 7578ee0 was kept only because its logs already cited
   it. Each rewritten commit's tree hash is checked equal to its
   original's, the old and new ids both printed, in `gate.log`, and
   the doc commit's three blobs rechecked against planning's hashes
   there. The doc message drops its count; each code message says
   "No log prints a count for this commit; the diff alone carries
   what it changes."; a clause citation is worded so no numeral
   stands where the scan's form counts one (the workbook root's
   children in schema order, by name). From the gate on, every log
   and record cites the new ids alone; the old ids appear in the
   gate's rewrite lines and nowhere else. The tool commit, whose
   message passes, is rewritten only in its parent.

## Planning verification and rulings on P1 (2026-09-28)

Verified by planning from primaries: five commits over 5e57db1
(07b359a doc, exactly the three files at planning's hashes;
bcaa31d the three carried items and nothing else; 2634ecd the
crate; e981810 the tool; 4c3485a the record, 11 files), every
message ASCII with the one trailer, no session link; the seven
logs and checks at the reported hashes, every log pinning the spec
at 827b3e33, stderr 0 bytes in each; gate.log's rewrite lines with
the four old ids from the reflog and each new commit's tree equal
to its old one, nine PASS lines; test.log 512 off and 523 on per
target, the workspace 720 passed, 0 failed, 2 ignored per target,
six plants each failing its test; negative.log's four cases with
controls; run.log 27 of 27 on both targets, entries identical 816,
changed 45, dropped 9, added 0, 10 of 10 flags turned by a plant,
160 names in the inputs, the container created in 11, the outputs
identical across targets, the keep 367 pairs with part 3 P4's 336
unchanged, 0 outputs refused in the open check; scan.log PASS, 0
hits, 0 structural failures. Planning's own privacy scan over
5e57db1..HEAD: 5 commits, 714 blobs, 0 findings; origin/main still
6dbb4a0; `ANTHROPIC_API_KEY` unset. Planning's own build of e981810
in a detached worktree: the crate's tests 512 off and 523 on, on
both targets; the workspace 720, 0, 2; clippy off and on and doc
on clean; its own release build of the tool run with
`--names-sample` over the 27 corpus workbooks, every output byte
for byte the phase run's host output.

1. F1 (check 1), the creation rule's wording: the code's placement
   (part 6's, immediately after the last earlier child) stands and
   the spec's sentence is corrected in place; the two readings
   differ only in where whitespace, a comment or a foreign child
   between two children lands. The run log's check and the test
   `a_created_container_goes_in_schema_position` pin the placement.
2. F2 (check 1), the core-side extras (`text_of` `pub(crate)`, the
   `BookEdits` refactor, the test wrapper, part 1's test gated):
   accepted; P4's size log measures what they cost.
3. F3 (check 1): ordinals are `usize`, the "32-bit" sentence
   corrected in place; the "before the root's end" case is the
   root-end path part 6's `calcPr` tests already take, accepted; the
   `names.rs` comment claiming both containers' names are read
   where the second is empty: corrected in P2's first code commit.
4. G4 (check 2): `R` and `C` alone are Excel's reserved shorthand
   and the rule refuses them; the rule is corrected in place and
   the code and the test's two pins follow in P2's first code
   commit.
5. F8 (check 1): the "exactly when" clause was written before
   decision 2 A and is obsolete; corrected in place. The flag is set
   in every workbook, the chain dropped in the 9 that have one.
6. F9 (check 1): the first half was met by the rerun (N7 and N8);
   the second half's sentence meant every log that runs the crate's
   tests, corrected in place. G3 (check 2): with the feature on the
   read refuses a `definedName` with an unbound attribute prefix or
   a text the cell reader refuses, for every caller: accepted as
   the feature's read, consistent with part 2's rule; a crate test
   pinning the refusal at the read goes in P2's first code commit,
   and P4's catalogue states it.
7. G1 (check 2): the scan script declares the phase's directory as
   `<scratch>`, which `specs/0001` reserves; accepted for P1, since
   the root line declares it, and from P2 the scan declares it
   `<phase>` as the other logs do. G2, F6: from P2 the run log's
   spec check accepts one hash, given on a labelled input line, and
   the scan's pin count counts lines outside the printed scripts.
   G6: from P2 the gate's status pattern names the open-check
   record stem. G7: from P2 every environment input a script reads
   is printed on a labelled line after its invocation line; the
   awk public list is accepted beside the consumer crate. G9(b):
   from P2 a rerun line names the directory its superseded files
   are kept in. G5, G11: accepted as they stand.
8. G8 (check 2): bcaa31d changed the format probe's own source, so
   "Size" now measures the core's growth against the probe rebuilt
   at bcaa31d, corrected in place; `Workbook` gaining `pub(crate)`
   fields under the feature is accepted and P4's catalogue says it
   is not constructible outside the crate with the feature on.
9. G9(a), (c), (d) and G10 (check 2): the push state is planning's
   to verify (nothing pushed, verified); "in its parent alone" is
   ruling 2's wording, shown by the equal subject and tree; the
   quotation's case is a report matter, accepted; a committed check
   answer is verbatim by `CLAUDE.md` and is outside ruling 2's
   "nowhere else", which covers the logs, records and messages the
   implementing session writes.
10. The design choices of 2634ecd (the scope stored as the sheet's
    part; an empty text refused as "not valid"; a forbidden
    character refused at the call with part 4's message; a
    main-namespace child outside the table counting as after every
    name in it; part 1's test gated with a feature-on sibling):
    accepted. P1 is DONE.

## Planning verification and rulings on P2 (2026-09-28)

Verified by planning from primaries: five commits over 4c3485a
(95e5fe3 doc, exactly the two files at planning's hashes; 4e62e76
the three carried items with the fixture renames the name rule
forced and one catalogue line; a791e2c the crate; cced544 the
tool; a9e5cb6 the record, 11 files), every message ASCII with the
one trailer, no session link; the seven logs and checks at the
reported hashes, every log pinning the spec at 3c4c26c7, no rerun,
stderr 0 bytes in each; gate.log's doc blobs checked, nine PASS
lines, 512 off and 535 on per target; test.log 735 passed, 0
failed, 2 ignored per target, seven plants each failing its test;
negative.log's two cases with controls; run.log 27 of 27 on both
targets, entries identical 822, changed 48, dropped 0, added 6,
12 of 12 flags turned by a plant, links 176 in and 230 out, the
container created in 20, no root retagged (every corpus root
binds a prefix), the outputs identical across targets, the keep
381 pairs with one P1 pair changed by the fixture rename, 0
outputs refused in the open check; scan.log PASS, 0 hits, 0
structural failures. Planning's own privacy scan over
4c3485a..HEAD: 0 findings; origin/main still 6dbb4a0;
`ANTHROPIC_API_KEY` unset. Planning's own build of cced544 in a
detached worktree: the crate's tests 512 off and 535 on, on both
targets; the workspace 735, 0, 2; clippy and doc on clean; its own
release build of the tool run with `--link-sample` over the 27
corpus workbooks, every output byte for byte the phase run's host
output.

1. F1 (check 1), at-call refusals the spec did not list (a
   forbidden character in a target or tooltip, a cell off the
   grid): accepted, consistent with `set_cell` and P1 ruling 10;
   "Hyperlinks" and "Errors" corrected in place.
2. F2, G3 (both checks), the relationship type's form taken from
   the rels part's existing relationships while the retag's
   namespace form comes from the root: two signals, and a strict
   root with no hyperlink relationship gets a mixed pair. Ruled:
   both follow the form of the worksheet's own relationship in the
   workbook's rels part, which the package reads already; corrected
   in place, the change and a strict-form test in P3's first code
   commit. `set_elements` choosing by `rel_id` rather than
   `external`: accepted, unreachable by construction.
3. F3 (check 1), `r1`, `r2` and so on when `r` is bound elsewhere:
   accepted, corrected in place; the crate test pins it.
4. F4, F14, G8 (both checks), a link on a sheet added in the same
   write: ruled as "Hyperlinks" now reads (a location link written,
   an external one refused by the existing source rule, the caller
   writes twice); two tests pinning each case, with the message the
   external case meets read from the test and not from the report,
   in P3's first code commit; P4's catalogue states it.
5. G1 (check 2), the kept content of `<hyperlinks>` held until its
   end tag: accepted, as part 3 holds `<mergeCells>`; "32-bit and
   streaming" corrected in place.
6. F5 (check 1): a relationship removed while a kept `<hyperlink>`
   names the same `r:id` leaves a dangling id; the pass holds the
   container, so it knows the kept ids. Ruled: a relationship is
   removed only when no kept `<hyperlink>` names it; the change and
   a test in P3's first code commit. A container whose only
   children are not `<hyperlink>` elements is dropped with them:
   accepted as stated (18.3.1.48 admits no other child). The
   `unwrap_or(0)`: unreachable, the name is in the table; accepted.
   The `no_links` doc comment (G10): corrected in P3's first code
   commit.
7. G2 (check 2): `write_to` consumes the package, so no second
   write exists; accepted, no action. G4: the tool counts external
   hyperlink relationships, "Harness runs" corrected in place. G7:
   lower-case `r` and `c` pinned in P3's first code commit. F15: a
   test of two external sets on one cell in P3's first code commit.
8. Logs. F6 (both checks): from P3 the run script plants the retag
   on a byte-built workbook whose root binds no relationships
   prefix, since the corpus never exercises that branch. G5: from
   P3 every script prints `HOME` and `ANTHROPIC_API_KEY` on
   labelled input lines. F11: the stale "uncommitted" label fixed
   in P3's run script. F7, F8, F9, F10, G6: accepted as remarks; the
   `rowcol`-gated link test runs in the workspace run, which is
   where the gate's figure does not reach, accepted.
9. P1's committed scan.log carried `set -uo pipefail` inside a
   comment, so it never took effect there; its checks ran and
   printed as they did, and P2's scan puts the line on its own.
   Accepted as measured; P1 is not rerun. The catalogue line placed
   in 4e62e76 rather than P4: accepted. F12, F13, G9, G11, G12:
   report matters, accepted.
10. P2 is DONE.

## Planning rulings during P3 (2026-09-28)

The implementing session held before recording P3 (phase commit
372bc21; the logs installed, uncommitted, all at attempt 1; check
1 answered) on check 1's F1, a defect it confirmed in the source
and planning confirmed again from the source at 51acd32: a package
whose core-properties relationship names a part the archive lacks
takes a property edit and writes without it. `record_core_creation`
sees the relationship and creates nothing, the write's core
rewrite is skipped when the part has no index, and `validate`
checks a core part only when it is present; `properties` refuses
the same package with "no such part". No test covers the write
side, and no corpus package has the shape (all 27 hold their core
part), so the logged runs are unaffected. Planning had not yet
read check 1 itself when ruling; the items below other than F1
are ruled on the implementing session's statement of them, and
planning's verification of P3 reads the check and revisits any
whose statement it finds differs.

1. F1. Ruled: P3 is recorded as measured at 372bc21, the defect
   stated in the report and the record as a defect of 51acd32 not
   fixed at the phase commit, as part 3's usage strings were. The
   fix goes in P4's first code commit: `validate` refuses a write
   that edits a property (a set or a removal, either alone) when
   the core relationship names a part the archive lacks, with "no
   such part", the message the read gives; two tests, one a set
   and one a removal, each reading the message from the test and
   not from the report; P4's negative log gains the seventh fixture
   case through the binary. "Document properties" and "P4" are
   corrected in place. The logs are not rerun and check 2 runs on
   372bc21 as planned. The spec's reading (refuse, not create) is
   kept: the relationship says a part exists, and a write that
   invented one would hide a broken package.
2. Why not fix now: the phase's logs measure the tree as built,
   every one at attempt 1; a fourth code commit would rerun all of
   them for a branch no corpus package reaches; the fix lands
   before the close-out push either way, and P4's own logs cover
   it. The precedent is part 3 P4's usage strings.
3. F2. `SheetProtection::default()` is the schema's default and
   writes exactly `<sheetProtection sheet="1"/>`; the spec's
   "Excel's dialog default" contradicted its own exact-bytes
   clause (the dialog locks `objects` and `scenarios`, the corpus's
   commonest form). Accepted as built, "Protection" corrected in
   place. `sheet: false` writing `<sheetProtection/>` with the
   options: accepted as built, the schema's unprotected element
   with options, written as asked; corrected in place; P4's
   catalogue states it.
4. F3. `CORE_PROPERTIES` a two-form array with the `officeDocument`
   form second: accepted, the shape of the catalogue's other eight;
   "Document properties" corrected in place.
5. F4. The at-call forbidden-character refusal on a property value:
   accepted, consistent with P1 ruling 10 and P2 ruling 1;
   corrected in place. The removed or replaced core part refused as
   "an edit to a removed part" is in `validate` and in the spec's
   reading but untested: one test each (a `remove_part` and a
   `replace_part` of the core part with a property set) in P4's
   first code commit, the message read from the test.
6. F5. A root binding the `xsi` namespace only as its default
   namespace would give an appended date an unprefixed `type`
   attribute, which is in no namespace: ruled a refusal with the
   no-prefix message, the change and a test in P4's first code
   commit; corrected in place. Unreachable by any corpus package.
7. F7. A kept `<hyperlink>` is read for its `r:id` since P2 ruling
   6, so an unbound prefix on one is refused where P2 passed it:
   accepted, the part is not namespace-well-formed and the refusal
   is part 2's; "Errors" corrected in place.
8. F8. Five of the thirteen properties never written in a write
   test, where "P3" asks each replaced, removed and appended: one
   test in P4's first code commit writing all thirteen in each of
   the three forms against expected bytes.
9. Scripts and report. F10, the run script's link comparison still
   encoding the two form signals P2 ruling 2 replaced: fixed in
   P4's run script, which runs `--link-sample` again. F9 (the
   protect comparison's hash flag a self-test), F11 (two printout
   runs without the feature): fixed in P4's scripts where cheap,
   else stated in P4's report as remarks. F12 to F15, report
   wording: accepted as report matters, fixed in P3's report where
   cheap.
10. The order stands: check 2 on 372bc21; scan last; one record
    commit; report; STOP. No push.

## Planning verification and rulings on P3 (2026-09-28)

Verified by planning from primaries: five commits over a9e5cb6
(c9b313f doc, exactly the two files at planning's hashes e2763566
and f5c74f60; bc46388 the six carried P2 items; 51acd32 the crate;
372bc21 the tool; d3bb207 the record, 13 files, its message
stating F1 as ruling 1 asks), every message ASCII with the one
trailer, no session link; the seven logs and checks at the
reported hashes, every log pinning the committed spec at e2763566;
the four logs at attempt 1; the scan's pre-check at attempt 2
(change AV admitting planning's modified spec, hashed on a
labelled input line) and its final at attempt 2 (attempt 1 failed
one structural check on a cargo process of another project's
build on the machine; rerun once idle, both attempts hashed in its
rerun lines); stderr 0 bytes in each; gate.log nine PASS, 512 off
and 556 on per target; test.log 760 passed, 0 failed, 2 ignored
per target, the three password hashes printed, the sixteen option
elements, eight plants each failing its test; negative.log the
no-`dcterms` root, the doubled title and the doubled
`<sheetProtection>` refused with their exact messages, each with a
control, the bare template creating the part through the binary;
run.log props 27 of 27 on both targets, entries identical 843,
changed 27, dropped 0, added 0, 9 of 9 flags turned by a plant;
protect 816, 54, 0, 0, 8 of 8; the outputs identical across
targets; the retag plant on a byte-built workbook whose root binds
no relationships prefix, its flag turned by a plant; the keep 431
pairs, 50 new, none gone; 0 outputs refused in the open check;
scan.log PASS, 0 hits, 0 structural failures. Planning's own
privacy scan over a9e5cb6..HEAD: 5 commits, 738 blobs, 0
findings; origin/main still 6dbb4a0; `ANTHROPIC_API_KEY` unset.
Planning's own build of 372bc21 in a detached worktree: the
crate's tests 512 off and 556 on, on both targets; the workspace
760, 0, 2 on both targets; clippy off, on and over the workspace
clean; doc clean; its own release build of the tool run with both
samples over the 27 corpus workbooks, every output and every
printed line byte for byte the phase run's host output, and the
run directories' host and i686 outputs identical 27 of 27 for
each sample.

1. F1 stands as rulings 1 and 2 read; the record commit's message
   states it (G4 verified).
2. F6, G1 (both checks): `no_link_edits` widened to properties
   and both protections. Its one caller is `Recorded::is_empty`,
   which `validate` uses to return early when nothing is recorded,
   so the widening is what gets a properties-only or
   protection-only write validated at all; verified from the
   source at 372bc21. Accepted, required; its doc comment states
   the wider meaning.
3. G2 (check 2): "Crate" corrected in place, two `package`
   constants.
4. G7 (check 2): "P4" corrected in place, the fixture cases
   counted by the messages P1 to P3's negative logs reached (five
   of the eleven) plus the missing core part.
5. F9 (check 1): the protect comparison's hash flag compares the
   log's own hash with a typed value, so no output can turn it.
   Ruled: in P4's run script the flag compares the output's
   password attributes, both elements, to the log's own hash of
   the tool's constant. F10: fixed as ruling 9 of the P3 rulings
   reads. F11: the two single-test printouts run with the feature
   on from P4; P1 ruling 6 covers them.
6. G3 (check 2): from P4 the scan's planning-file check has a
   control of its own (a listing with another planning file
   modified refused) and the two labels read as the tree stands.
   Accepted as a remark for P3.
7. F12 (check 1): the bare template's created core part goes
   through the binary unchecked; P4's negative log compares it by
   sha256 to the bytes the crate test pins; "P4" corrected in
   place. F15: an unprotect of a sheet with no element rewrites
   the part to its own bytes (a recompressed entry), the
   workbook's does not; accepted as built, P4's catalogue states
   it.
8. Check 2's remark on `opened_up`: an empty-element tag given
   content cannot keep its start tag byte for byte; accepted, the
   only reading; "Document properties" corrected in place.
9. F13, F14, G5, G6, G8, G9 (both checks): report matters,
   accepted; the `rowcol`-gated protection test runs in the
   workspace run, as P2 ruling 8 accepted for the link test.
10. P3 is DONE. The items P4's first code commit carries: the
    missing-core-part refusal with two tests (P3 ruling 1); the
    `remove_part` and `replace_part` core tests (ruling 5); the
    default-namespace `xsi` refusal with a test (ruling 6); the
    thirteen-property write test (ruling 8). P4's scripts: F9,
    F10, F11, G3 and F12 as rulings 5 to 7 above read.

## Out of scope

- Built-in names (`_xlnm.Print_Area`, `_xlnm._FilterDatabase`, the
  print titles): their sheet scope and `hidden` rules are their
  own; a later part with page setup if a caller needs them.
- Parsing or checking a name's text, a location or a URL: stored
  as given, as openpyxl stores them.
- Reading hyperlinks or protection back as typed values; the
  `Hyperlink` cell style; `display` on a link.
- `app.xml` (`TitlesOfParts`, `Company`, `HyperlinksChanged`) and
  `custom.xml`; `lastPrinted`; `dcmitype` typed values.
- `protectedRanges` (ranges unlocked by password), workbook
  encryption (`EncryptedPackage`, not an OOXML package), the
  `fileSharing` password.
- Formula evaluation, rendering, autofit and reference fixup, by
  `CLAUDE.md`'s definition.
