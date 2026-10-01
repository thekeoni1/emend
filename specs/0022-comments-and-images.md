# 0022: Milestone 2 part 4d, comments and images

Status: APPROVED by the maintainer 2026-09-30 (planning outline the
same day): decisions 1 to 6 taken as planning recommended, in the
maintainer's reply "send spec 4d to impl", typed in the planning
session; decision 7 recorded. Sent to the implementing session the
same day.
Context: `CLAUDE.md` ("What emend is": like openpyxl; "Dependencies
and licences": anything beyond the core behind a Cargo feature off
by default; "The fidelity harness is the acceptance instrument";
"Corpus rules": committed fixtures documented with the tool that
produced them), `ROADMAP.md` ("Milestone 2", the outline approved
2026-09-23, item 4; "Part 4: the openpyxl features, in sub-parts";
"Standing items", the Milestone 2 ceiling), `specs/0009-package-
layer.md` (parts, relationships, overrides: `add_part`,
`add_relationship`, `add_override` and their removals, the created
rels part), `specs/0011-editing-cells.md` (`set_cell`, the cell
pass), `specs/0016-new-workbook-and-sheets.md` (a sheet's chain of
parts dropped with the sheet, P4 ruling 9: a comments part and its
VML, a drawing and its media), `specs/0018-rows-and-columns.md`
("Creating an element", the schema-position table; the shift rule,
under which comments, drawings and their anchors are copied raw),
`specs/0019-names-links-properties-protection.md` (the feature
pattern, the relationships prefix retag, decision 3 A),
`specs/0020-validation-and-conditional-formatting.md` (the
worksheet container pass held to its end tag), `specs/0021-
tables.md` (the part allocation rule with its ASCII case fold, the
projected-workbook rule, the close-out rulings 3, 8 and 10 carried
here), `specs/0001-bootstrap.md` ("Rules for every phase"). ECMA-
376 Part 1, 18.7 (`comments`, `authors`, `commentList`, `comment`),
18.3.1.52 (`legacyDrawing`), 18.3.1.36 (`drawing`), 20.5.2.24 and
20.5.2.33 (`oneCellAnchor`, `twoCellAnchor`), 20.5.2.25 (`pic`),
20.1.8.13 (`blip`), Part 4 (VML, transitional) are cited where the
design leans on them; no text is copied from them. openpyxl's
`Comment`, `Image`, `ws.add_image` and its comment writer are the
behavioural reference, read only.

## Why a part 4d

Parts 4a to 4c took the capabilities whose parts are XML in the
main namespace. This part takes the last two of the outline's nine,
whose parts are not: a cell comment lives in a comments part and
is shown through a VML drawing part Excel calls a legacy drawing,
and an image lives in a media entry placed by a DrawingML drawing
part. Both are chains the crate already drops with a sheet
(`specs/0016` P4 ruling 9) and copies raw through a shift
(`specs/0018`); this part writes them. It is the last sub-part of
part 4, and after it Milestone 2's outline is complete.

## Goal

A caller can add a comment to a cell (an author and a text), remove
the comment a cell has, and read a worksheet's comments; and can add
a PNG or JPEG image anchored at a cell and read a worksheet's images;
on an opened workbook or on `Package::new()`'s, with everything the
edit does not touch passed through byte for byte, the harness
proving the change on every corpus workbook, and each feature's
cost measured beside the core, which does not grow. What openpyxl's
`Comment` and `Image` with `ws.add_image` do, with openpyxl's shape
of the parts where Excel's own differs only in detail. Image removal
is out (openpyxl has no call for it; "Out of scope").

## Size and packaging (decision 1, before any code)

The budget figure after part 4c is 435224 section bytes against
the Milestone 2 ceiling of 458752 (`records/size-m2p4c-2026-09-30-
3c5f0b7.md`): 23528 of headroom, held for fixes. The outline puts
part 4 behind features, so nothing here is in the core budget.
Planning's estimate, with part 4c's ratio in view (30000 to 45000
against a measured 46860 for one created part and one worksheet
container), is 40000 to 55000 section bytes for each feature:
`comments` writes two parts and appends to two kinds of existing
part; `images` writes or appends to a drawing part, adds a media
entry and reads two image headers. The core's growth for the part
is what the feature gates add, printed by the size log and expected
near 0; above 2000 section bytes planning is told in the report.

- A (taken): two Cargo features `comments` and `images`, off by
  default, declared beside `tables` with no dependency of their own;
  one probe each measuring its cost beside the core; the core probe
  unchanged. A consumer that wants comments does not carry the
  image code.
- B: one feature. One figure fewer, and the smaller capability
  priced at the larger.

## Facts planning checked (2026-09-30)

Over the 27 corpus workbooks, by element shapes and counts with
Python's `zipfile` and `ElementTree` over the comments, VML,
drawing, worksheet, rels and content-types parts, never an author,
a text, a reference, a file name or an image's bytes:

- Comments parts (`xl/comments<N>.xml`) 9, in 6 workbooks, every
  root in the default namespace with the children `authors` then
  `commentList`, one author per part; 28 `comment` elements, every
  one with `ref` and `authorId`, 6 also with `shapeId` and a `uid`;
  every `text` holds runs (`r`) with `rPr` children among `b`,
  `color`, `family`, `rFont`, `sz`; no plain `t` child, which the
  schema allows and this part writes. 9 `Override` elements with
  the comments content type.
- VML parts (`xl/drawings/vmlDrawing<N>.vml`) 11, in 8 workbooks
  (P1 ruling 3 corrects planning's 7):
  8 open with an XML declaration, 3 do not; the root `xml` holds
  `shapelayout` (3 of 11), one or more `shapetype` and the shapes;
  31 `shape` elements, 28 with `ClientData` of `ObjectType` `Note`
  (the comment shapes, each with `Row` and `Column` children), 3
  of `Scroll` or `Spin` (form controls); the `Note` shapes carry
  `id`, `type`, `style`, `fillcolor` and either `insetmode` and
  `strokecolor` (Excel's) or `allowincell` and `stroked`
  (LibreOffice's). Shape ids: 8 of the form `_x0000_s<n>` with
  `n / 1024` equal to the part's `idmap` `data` (Excel's form, in
  3 parts with an `idmap` and no declaration), 22 of the form
  `shape_<n>` (LibreOffice's, in the 8 parts with a declaration
  and no `idmap`), 1 control shape with an id of another form and
  its number on `o:spid` (P1 ruling 1). 8 `Override` elements and
  3 `Default` elements (extension `vml`) give the VML content type:
  both forms occur, and this part writes an `Override` (below).
- Worksheets with `legacyDrawing` 11, every one with `r:id` alone,
  8 after `headerFooter` and 3 after `drawing`, schema order in
  every case; with `drawing` 37, after `headerFooter` 27,
  `ignoredErrors` 5, `pageSetup` 5; both in `WORKSHEET_CHILDREN`
  (`drawing` then `legacyDrawing`, before `tableParts`). The
  worksheet relationships: `comments` 9 (8 relative `../`, 1
  absolute), `vmlDrawing` 11 (8 relative, 3 absolute), `drawing`
  37 (14 relative, 23 absolute). No `threadedComment` relationship
  and no `xl/threadedComments/` entry anywhere (decision 3 is
  proven by a built fixture).
- Drawing parts (`xl/drawings/drawing<N>.xml`) 38, in 16
  workbooks: 37 roots `wsDr`, 1 `userShapes` (a chart's, not a
  worksheet's); 105 `twoCellAnchor`, 3 `AlternateContent` and 1
  `relSizeAnchor` children, no `oneCellAnchor` or `absoluteAnchor`;
  21 anchors hold a `pic` (22 `pic` in all, every `blip` with
  `r:embed` alone, every `cNvPr` with `id`, `name` and `descr`,
  every `spPr` with `xfrm` and `prstGeom`), 26 a `graphicFrame`
  (charts), 45 an `sp`, 7 a `grpSp`, 2 a `cxnSp`. The drawings'
  `image` relationships 34 (22 absolute, 12 relative), targets 22
  `.png` and 12 `.svg`; media entries 16 `.png` and 10 `.svg` in 8
  workbooks, no `.jpeg`, though 17 workbooks declare a `jpeg`
  `Default`; `png` `Default` in 21. This part writes PNG and JPEG,
  reads any `image` relationship back by its target.
- `remove_sheet` drops a sheet's comments part, its VML, its
  drawing and the media reached only from it (`specs/0016` P4,
  tested there); the shift pass copies `legacyDrawing`, `drawing`
  and their parts raw and says so in the limits (`specs/0018`).

## Design

### Crate

`zip` and `quick-xml` only; no new dependency: the PNG and JPEG
headers are read by this part's own code (two fixed layouts, below).
Under decision 1 A, everything below sits behind two Cargo features,
`comments` and `images`, off by default, each with the types and
`Package` calls of its own and its messages in the catalogue marked
as the feature's; the shared pieces (the part allocation rule, the
worksheet single-element creation, the `Override` through the
override rule) are the core's or part 4c's readers reused, never
duplicated. The core builds, tests and links without either; the
tool `emend-copy` turns both on as it does `tables`. Nothing
existing changes signature.

### Comments: types

`Comment<'a> { author: &'a str, text: &'a str }`, derived `Clone`,
`Copy`, `Debug`, `PartialEq`, `Eq`. The author: 1 to 255 characters
following part 4's character rule, refused at the call otherwise
("a comment author is not valid"); the text: 1 to 32767 characters
(Excel's limit on a cell's text, taken for a comment's), the
character rule, refused otherwise ("a comment text is not valid").
Formatting runs are out: the text is one run with no properties,
as openpyxl's `Comment` is.

`CommentEntry { cell: CellRef, author: String, text: String }`,
derived `Clone`, `Debug`, `PartialEq`, `Eq`: `text` is the
concatenation of every `t` descendant of the comment's `text`
element in document order, runs flattened; `author` the `authors`
child at `authorId` (an `authorId` past the list reads as an empty
author, not a refusal: a comment is still a comment).

### Comments: the calls

- `add_comment(&mut self, part, cell: CellRef, comment: Comment) ->
  Result<(), Error>`: records one comment on the worksheet part
  `part` (as `set_cell` names it) at `cell`. At the call: the author
  and text rules, and `cell` on the grid (part 3's index message).
  At the write: a kept comment on the same cell is replaced
  (decision 6 A, openpyxl's assignment semantics: its element and
  its shape are skipped as a removal skips them and the addition
  takes their place at the end); two additions on one cell in one
  write, the last wins, as `set_cell` resolves two edits. A
  worksheet whose rels part has a relationship of kind
  `THREADED_COMMENTS` is refused ("a worksheet has threaded
  comments"; decision 3 A: the legacy part is their fallback and
  this part would desynchronise the two). A comments relationship
  whose part is missing, a comments part whose root has no
  `commentList`, a `legacyDrawing` whose relationship or part is
  missing, a comments part on a sheet with no `legacyDrawing`, or a
  VML part that holds no `shape` element where the comments part
  holds a comment, is refused ("a comments part or its drawing
  cannot be read"). A comment on a sheet added in the same write is
  refused with part 1's "no such part" (as a table is; `specs/0021`
  P1 ruling 2), on a replaced or removed part with part 1's
  messages for a cell edit.
- `remove_comment(&mut self, part, cell: CellRef) -> Result<(),
  Error>`: records the removal of the comment at `cell`. At the
  write: the `comment` element whose `ref` parses to `cell` is
  skipped, and in the VML part the `shape` whose `ClientData` is of
  `ObjectType` `Note` with `Row` and `Column` equal to the cell's
  zero-based row and column is skipped; no comment at the cell at
  the write is refused ("no comment on that cell"), as is a cell
  this write also adds a comment on (the caller writes twice, as
  `specs/0021` P2 rules for a table) and a cell named twice for
  removal. A `commentList` left empty drops the comments part as
  `remove_table` drops a table part (the element `legacyDrawing`
  is kept when the VML part still holds a shape of another
  `ObjectType`; otherwise the VML part, its relationship, its
  override and the `legacyDrawing` element go too; the comments
  part's relationship, override and entry always go). Authors are
  never removed: an author no comment names is a valid part and
  Excel keeps such. An addition after a removal on one cell in one
  write is the replacement case (the removal is subsumed; refused
  as above only when the cell has no kept comment).
- `comments(&mut self, part) -> Result<Vec<CommentEntry>, Error>`:
  reads the part's `comments` relationship's part, the `authors`
  once and every `comment` in document order; a part with no such
  relationship gives an empty list; a relationship whose part is
  missing, a root without `commentList`, a `comment` without `ref`
  or `authorId`, or a `ref` that does not parse, refuses the read
  with the cannot-be-read message. Nothing is kept: the caller's
  snapshot, as `tables` is. Threaded comments are read as their
  legacy fallback: the entry shows the fallback text Excel writes
  there, and the limits say so.

### Comments: the parts

A sheet with no comments part gets one, and a sheet with no VML
part gets one; a sheet with either gets an appended element in it.
The entry names `xl/comments<N>.xml` and `xl/drawings/
vmlDrawing<N>.vml` take the smallest positive N such that no entry
of the input's archive and no addition of this write has that
name, compared ASCII case folded, a removed entry still counted as
taken (`specs/0021`'s rule, P1 ruling 6). The `Override` for each,
`application/vnd.openxmlformats-officedocument.spreadsheetml.
comments+xml` and `application/vnd.openxmlformats-officedocument.
vmlDrawing`, through the override rule (the corpus has 8 VML
overrides and 3 `vml` defaults; an `Override` is valid beside a
`Default` and the crate writes no `Default`; a part name that
already has an override meets part 1's message). The relationships
of kinds `COMMENTS` and `VML_DRAWING` (new constants in the pair
form, transitional and strict, as `TABLE`), in the form of the
worksheet's own relationship in the workbook's rels part, recorded
on the part's rels part with the relative targets `../comments<N>.
xml` and `../drawings/vmlDrawing<N>.vml`, the rels part created as
part 4a creates one for a hyperlink. In the worksheet, `<legacy
Drawing r:id=".."/>` created in schema position (part 3's rule:
before the first later child of `WORKSHEET_CHILDREN` present, which
in the corpus is `tableParts` or `extLst`, else before the root's
end tag) with the root's relationships prefix, retagged as part 4a
retags for a hyperlink; a second `legacyDrawing` is refused ("a
worksheet has two legacyDrawing elements"), as is one out of
schema order (part 4b's P2 verification ruling 2).

The comments part, created (the declaration on its own line, the
rest one line, broken here to fit):

```
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<comments xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
<authors><author>..</author></authors>
<commentList>
<comment ref="A1" authorId="0"><text><t>..</t></text></comment>
</commentList>
</comments>
```

No `shapeId` (22 of the corpus's 28 lack it), no `xr` namespace or
`uid`; the text escaped as part 4 escapes a cell's, with
`xml:space="preserve"` on `t` when the text begins or ends with
whitespace, as the cell pass writes an inline string. Appended to
an existing part: the `authors` container held to its end tag, the
author appended as `<author>` when no kept author's text equals it
(the index is then the kept count plus the additions before it),
else the kept index; the `commentList` container held to its end
tag, the kept `comment` elements copied (a replaced cell's skipped),
each addition appended before the end tag in record order, taking
the container's prefix. Two `commentList` elements are refused with
the cannot-be-read message. A kept part is streamed; its two
containers are bounded by the part's comments (28 in the corpus).

The VML part, created (one line, broken here to fit; the numbers
are openpyxl's defaults, in points):

```
<xml xmlns:v="urn:schemas-microsoft-com:vml"
 xmlns:o="urn:schemas-microsoft-com:office:office"
 xmlns:x="urn:schemas-microsoft-com:office:excel">
<o:shapelayout v:ext="edit"><o:idmap v:ext="edit" data="D"/></o:shapelayout>
<v:shapetype id="_x0000_t202" coordsize="21600,21600" o:spt="202"
 path="m,l,21600r21600,l21600,xe">
<v:stroke joinstyle="miter"/><v:path gradientshapeok="t" o:connecttype="rect"/>
</v:shapetype>
<v:shape id="_x0000_s<S>" type="#_x0000_t202"
 style="position:absolute;margin-left:59.25pt;margin-top:1.5pt;width:108pt;height:59.25pt;z-index:<K>;visibility:hidden"
 fillcolor="#ffffe1" o:insetmode="auto">
<v:fill color2="#ffffe1"/><v:shadow on="t" color="black" obscured="t"/>
<v:path o:connecttype="none"/>
<v:textbox style="mso-direction-alt:auto"><div style="text-align:left"/></v:textbox>
<x:ClientData ObjectType="Note"><x:MoveWithCells/><x:SizeWithCells/>
<x:Anchor><C1>, 15, <R1>, 2, <C2>, 15, <R2>, 2</x:Anchor>
<x:AutoFill>False</x:AutoFill><x:Row><R></x:Row><x:Column><C></x:Column>
</x:ClientData></v:shape>
</xml>
```

No XML declaration (Excel writes none; 8 of 11 corpus parts carry
one and both open). `R` and `C` are the cell's zero-based row and
column; the anchor's `C1` is `C + 1`, `C2` is `C + 3`, `R1` is `R`
less one when `R` is above zero else `R`, `R2` is `R1 + 4`, the
box Excel places beside a cell. `D` is the smallest positive
integer no VML part of the projected workbook uses as its `idmap`
`data` (the first integer of that attribute's list; a part with no
`shapelayout` uses none), `S` is `1024 * D + K` and `K` counts the
shapes of the part from one, Excel's convention that a part's
shape numbers lie in its `idmap` range, which makes every shape id
unique across the workbook while each part's `idmap` is distinct
(Excel's own invariant; the limits say so). Appended to an existing
part (P1 ruling 1): the root `xml` held to its end tag (the whole
part is held: 11 corpus parts, the largest 10 shapes), the kept
shapes copied (a replaced or removed cell's `Note` shape skipped
whole), each addition appended before the end tag with `D` the
part's own `idmap` `data` when it has one else allocated as above
and written into no kept element, and `K` the larger of one above
the kept shape count (every kept shape counted, whatever the form
of its id) and one above the largest `n - 1024 * D` over the
part's own kept shapes whose `id` or `o:spid` is `_x0000_s<n>`
with `n` in `D`'s range (above `1024 * D`, at most `1024 * D +
1024`); no other part is read. The `v`, `o` and `x` prefixes are
those the root binds (a root binding any of the three namespaces
to another prefix uses that prefix; one binding none of them is
refused with the cannot-be-read message). The shapetype
`_x0000_t202` is written once when the part has none. A VML part
that is not well-formed enough to find its root's end tag is
refused with the cannot-be-read message.

### Images: types

`Image<'a> { bytes: &'a [u8], size: Option<(u32, u32)> }`, derived
`Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`: `bytes` is the whole
file; `size` is the pixel width and height to place it at, or
`None` for the file's own (decision 4 A). The format is read from
the bytes: PNG when the first eight bytes are the PNG signature and
the first chunk is `IHDR` (width and height big-endian `u32` at
offsets 16 and 20); JPEG when the first two bytes are `FF D8` and a
marker among `C0`, `C1`, `C2` (baseline, extended, progressive
`SOF`) is found by walking the segments (each `FF xx` followed by a
big-endian `u16` length, `FF 00` and `FF D8` to `FF D9` as the
standard says) before the scan start `DA`, its height and width
big-endian `u16` at offsets 5 and 7 of the segment; anything else,
a PNG shorter than 24 bytes or whose `IHDR` is not the first chunk,
a JPEG with no `SOF` before the scan, is refused at the call ("an
image is not a PNG or JPEG"). A width or height of 0, or a size
whose EMU value exceeds `i32::MAX` (9525 EMU per pixel, Excel's 96
dpi), is refused ("an image size is out of range"). Every byte
count and offset is `u64` or checked `usize` arithmetic as
`CLAUDE.md` asks.

`ImageEntry { from: (u32, u32), anchor: Anchor, part: Vec<u8>,
bytes: u64 }`, derived `Clone`, `Debug`, `PartialEq`, `Eq`; `Anchor
{ OneCell, TwoCell, Absolute }`, derived `Clone`, `Copy`, `Debug`,
`PartialEq`, `Eq`: `from` is the anchor's `from` column and row,
zero-based (both 0 for an `absoluteAnchor`); `part` the media
entry's name resolved through the drawing's rels part; `bytes` the
entry's uncompressed size from the archive's index, never the
bytes.

### Images: the calls

- `add_image(&mut self, part, cell: CellRef, image: Image) ->
  Result<(), Error>`: records one image on the worksheet part
  `part` anchored at `cell`'s top-left corner. At the call: the
  format and size rules, `cell` on the grid. At the write: a
  `drawing` relationship whose part is missing, a drawing part
  whose root is not `wsDr`, a `drawing` element with no `r:id` or
  one binding no relationship, is refused ("a drawing part or its
  relationship cannot be read"); a second `drawing` element, or
  one out of schema order, is refused ("a worksheet has two
  drawing elements"). No overlap rule: images may cover anything
  (decision 6 A). An image on a sheet added in the same write,
  a replaced or a removed part: part 1's messages, as a comment.
- `images(&mut self, part) -> Result<Vec<ImageEntry>, Error>`:
  reads the drawing part's root children in document order; each
  `oneCellAnchor`, `twoCellAnchor` or `absoluteAnchor` holding a
  `pic` (directly, or inside a `grpSp` or an `AlternateContent`'s
  `Choice`, the first `pic` found) gives one entry, its `blip`'s
  `r:embed` resolved through the drawing's rels part to the media
  entry; an anchor without a `pic` (a chart, a shape) gives none.
  A part with no `drawing` relationship gives an empty list. A
  `blip` without `r:embed`, an `r:embed` with no relationship, a
  relationship whose entry is missing, or a `from` without `col`
  and `row`, refuses the read with the cannot-be-read message.
  Nothing is kept.

### Images: the parts

The media entry `xl/media/image<N>.png` or `.jpeg` (openpyxl's and
Excel's extension for each format) takes the smallest positive N
over both extensions such that no entry of the archive and no
addition of this write has either name, case folded, removed
entries counted; the bytes are the caller's, unchanged, stored
through `add_part` as any created part is (the archive writer's
choice of method applies). Its content type is an `Override`
(`image/png` or `image/jpeg`) through the override rule, whether or
not a `Default` for the extension exists (both are valid; the
crate writes no `Default`). A sheet with no drawing part gets
`xl/drawings/drawing<N>.xml` by the same allocation, its `Override`
`application/vnd.openxmlformats-officedocument.drawing+xml`, a
relationship of kind `DRAWING` (pair form) on the sheet's rels part
with the relative target `../drawings/drawing<N>.xml`, and
`<drawing r:id=".."/>` created in schema position with the root's
relationships prefix (retagged as a hyperlink's root is). The
drawing's own rels part `xl/drawings/_rels/drawing<N>.xml.rels`
gets a relationship of kind `IMAGE` (pair form) with the target
`../media/image<N>.<ext>`, created when the drawing is created or
when a kept drawing has none.

The drawing part, created (declaration on its own line, the rest
one line, broken to fit; `cx` and `cy` are the pixel size times
9525):

```
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<xdr:wsDr xmlns:xdr="http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing"
 xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
<xdr:oneCellAnchor>
<xdr:from><xdr:col>C</xdr:col><xdr:colOff>0</xdr:colOff>
<xdr:row>R</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:from>
<xdr:ext cx=".." cy=".."/>
<xdr:pic>
<xdr:nvPicPr><xdr:cNvPr id="I" name="Image I"/><xdr:cNvPicPr/></xdr:nvPicPr>
<xdr:blipFill>
<a:blip xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" r:embed="rIdN"/>
<a:stretch><a:fillRect/></a:stretch>
</xdr:blipFill>
<xdr:spPr><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></xdr:spPr>
</xdr:pic>
<xdr:clientData/>
</xdr:oneCellAnchor>
</xdr:wsDr>
```

openpyxl's shape, which Excel opens without repair (the open check
proves the opening). `I` is one above the largest `cNvPr` `id` in
the part (1 in a created part); `R` and `C` zero-based. Appended to
a kept drawing (decision 5 A, so the 37 corpus drawings exercise
the main case): the root held to its end tag (the whole part;
the largest corpus drawing holds the 105 anchors' share), the kept
children copied byte for byte, each addition appended before the
end tag with the root's prefix for the spreadsheet-drawing
namespace and the `a` and `r` namespaces bound on the added
elements themselves when the root does not bind them (the
`r:embed` form above; a root binding either to a prefix uses that
prefix); the relationship form of the drawing's rels part is the
sheet's (transitional or strict, read as the worksheet's own is).
A drawing root `userShapes` is a chart's and no worksheet names
one; a worksheet `drawing` relationship to such a part is refused
with the cannot-be-read message.

### Creating an element

Part 3's rule and table, unchanged: `drawing` goes immediately
before the first later child of `WORKSHEET_CHILDREN` present
(`legacyDrawing`, `legacyDrawingHF`, `drawingHF`, `picture`,
`oleObjects`, `controls`, `webPublishItems`, `tableParts`,
`extLst`), `legacyDrawing` before the first of that list after it;
else before the root's end tag. A part whose element comes after a
later child is out of schema order and refused with the
two-elements message, nothing written. Both elements in one write
on one part are created in that order.

### Errors

Ten new `Error::Package` messages, in the catalogue as the earlier
parts' are, one constant each, marked by feature:

`comments`:
- "a comment author is not valid" (at the call)
- "a comment text is not valid" (at the call)
- "a worksheet has threaded comments"
- "no comment on that cell"
- "a worksheet has two legacyDrawing elements"
- "a comments part or its drawing cannot be read" (at the read
  and the write)

`images`:
- "an image is not a PNG or JPEG" (at the call)
- "an image size is out of range" (at the call)
- "a worksheet has two drawing elements"
- "a drawing part or its relationship cannot be read" (at the read
  and the write)

Shared (constants under either feature, one definition): part 3's
index message for a cell off the grid, part 1's "an edit to a
removed part", "a cell edit to a part this write replaces" and "no
such part", its "an override for that part already exists" through
the override rule, part 4's "a text holds a character XML cannot
carry", part 4a's relationships-prefix message, part 2's unbound-
prefix refusal wherever an element this part reads carries one,
part 3's one-shift message for a comment or image edit with a
shift on one part, and `Error::Xml` for a value that is not UTF-8.
Nothing is checked twice.

### 32-bit and streaming

Every count is `u32` or `u64` as elsewhere; an element ordinal is
`usize`; an image's bytes are the caller's slice, never copied
twice. The worksheet pass streams as today; the comments part's
two containers, the VML root and the drawing root are held to their
end tags, each bounded by what the corpus shows (28 comments, 10
shapes, a drawing's anchors). The image header read touches at
most the bytes before the JPEG scan start. `images` streams the
drawing part once; `comments` streams the comments part once.

### Size

Two probes, under decision 1 A. `probe-emend-comments-write-xml`
(feature `comments` of the probe crate, turning on
`emend/comments`): `probe-emend-format-write-xml`'s work,
unchanged, then on the same package before its write `add_comment`
at a fixed cell past the format probe's edits with an author and a
text from the command line, then after the write the output opened
and the comment count read back through `comments` and printed.
`probe-emend-images-write-xml` (feature `images`): the same with
`add_image` of a PNG the probe holds as a constant (the tool's
fixture below, the same bytes) at a fixed cell, the image count
read back through `images` and printed. The size log measures them
as part 4c's did, classifies the nineteen probes of part 4c's
record against it, the twentieth and twenty-first listed as new,
prints each feature's cost (its probe minus `probe-emend-format-
write-xml` rebuilt at the same commit), the core's growth for the
part (the format probe at the commit minus the same probe rebuilt
at 3c5f0b7, both printed; expected near 0), the whole growth since
b728fdf, and the budget figure (the format probe minus `probe-zip-
xml`) against 458752, which this part does not move by design; the
cause control and the manifest cause listing of part 4c's size log
stay.

### Harness runs

`tools/emend-copy` gains two operations, given after the two paths
as the others are; neither takes a reference, name or number from
its command line; every constant below is the tool's own. W is the
first worksheet part in `<sheets>` order and M its largest row
number as the tool's cell read gives it; a cell past the grid
refuses the sample whole with part 3's index message. No reference
from the corpus is printed.

- `--comment-sample <in> <out>` (P1, extended in P2): `add_comment(W,
  (M + 1, 1), Comment { author: "emend", text: "emend comment" })`;
  writes; reopens the output and prints `comment` with
  `comments_before`, `comments_after` (`comments(W)`; `+ 1`,
  asserted, the output removed on a difference as `--style-sample`
  does), `parts_before`, `parts_after` (entries `xl/comments*.xml`
  plus `xl/drawings/vmlDrawing*.vml`, counted over the archive; `+
  2`, `+ 1` or `+ 0` as W had neither, one or both, asserted
  against `legacy_before`), `legacy_before` (1 when W has a
  `legacyDrawing`), `comments_part_before` (1 when W has a
  `comments` relationship), `r_bound_before`, `shapes_before`,
  `shapes_after` (`Note` shapes of W's VML part, counted by the
  tool; `+ 1`, asserted), `authors_before`, `authors_after` (`+ 1`
  when no kept author is "emend", asserted); then, from P2, on the
  reopened output in memory, `remove_comment(W, (M + 1, 1))`, a
  write into a `Cursor`, and that reopened for `comments_removed`
  (equal to `comments_before`, asserted) and `parts_removed` (equal
  to `parts_before`, asserted, the created parts gone and the kept
  ones kept). A corpus comment on the cell the tool picks would be
  the replacement case, which M + 1 rules out; a sheet with
  threaded comments would refuse the sample (none in the corpus),
  and the run log reports a refusal as a finding.
- `--image-sample <in> <out>` (P3): `add_image(W, (M + 1, 1), Image
  { bytes: PNG, size: None })` where PNG is the tool's constant, a
  2 by 2 RGB PNG of 69 bytes with one stored deflate block, built
  by Python 3.12's `zlib` and `struct` in a six-line program the
  tool's source quotes beside the constant (a committed fixture
  documented with its producer, `CLAUDE.md`); writes; reopens the
  output and prints `image` with `images_before`, `images_after`
  (`images(W)`; `+ 1`, asserted), `drawing_before` (1 when W has a
  `drawing`), `anchors_before`, `anchors_after` (root children of
  W's drawing part; `+ 1`, asserted), `media_before`, `media_after`
  (entries under `xl/media/`; `+ 1`, asserted), `r_bound_before`,
  `from_after` (the added entry's `from`, equal to (0, M),
  asserted).

The runner and the record script are unchanged; the run names its
operation through `--tool-arg` and one `--label`. Expected results,
`lo` and `lo_in` `ok` in every case; `descriptor` alone allowed on a
changed entry; `order_preserved` `yes`; `files_identical` `no`; and
for the comment sample: W `changed` when it had no `legacyDrawing`
else `identical` (the replacement never occurs on M + 1); W's rels
part `changed`, `added` when the input has none, or `identical`
when W had both parts; `[Content_Types].xml` `changed` by one or
two overrides, `identical` when W had both; the comments part and
the VML part each `added` or `changed`; every other entry
`identical`. For the image sample: W `changed` when it had no
`drawing` else `identical`; W's rels `changed`, `added` or
`identical` likewise; the drawing part `added` or `changed`, its
rels part `added` or `changed`; one media entry `added`;
`[Content_Types].xml` `changed` (the media override, and the
drawing's when created); every other entry `identical`.

The run log's own check over the extracted input and output parts,
with `ElementTree` and byte spans and a control that reports a
planted difference per check, printing no name, no reference, no
text and no image bytes from the corpus:

- W: every root child byte-identical except the created
  `legacyDrawing` or `drawing` element, in schema position when
  created (checked against the table), with the expected `r:id`
  (derived as the rels rule does).
- W's rels: the input's relationships a byte-identical prefix of
  the output's, the expected one or two more with the expected
  relative targets; created with the package's form when absent.
- `[Content_Types].xml`: the expected overrides more, the rest
  byte-identical.
- The comments part: created, exactly the bytes "Comments: the
  parts" gives; appended, the input's `author` elements a prefix
  of the output's (one more or none), the input's `comment`
  elements a byte-identical prefix of the output's, one more with
  the expected `ref`, `authorId` and `t`, everything outside the
  two containers byte-identical.
- The VML part: created, exactly the bytes given with `D`, `S` and
  the anchor derived by the log over the input's VML parts;
  appended, the input's bytes up to the root's end tag a
  byte-identical prefix of the output's, one `shape` more with the
  expected `id` (derived by the log under P1 ruling 1's rule from
  the input part alone), `Row` and `Column`, the `shapetype` added
  only when the input has none.
- The drawing part: created, exactly the bytes "Images: the parts"
  gives with `I` and the `r:id` derived; appended, the input's
  bytes up to the root's end tag a byte-identical prefix, one
  `oneCellAnchor` more with the expected `from`, `ext`, `cNvPr`
  `id` and `r:embed`. The drawing's rels: a prefix plus one
  `image` relationship with the expected target. The media entry:
  byte-equal to the tool's constant.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. Every phase's run log
also runs the edit open check on the host over that phase's keep.
The run log names the phase directories it compares against by
their record commit. LibreOffice's rendering of a comment or an
image is not checked, only that the file opens; the Excel check
(decision 7) will show them as Excel shows them, and this part's
two samples join its list.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m2p4d-<phase>/` (`gate.log`,
`test.log` on both targets with the features off and on,
`negative.log`, `run.log`, `scan.log`, `size.log` in P4); every
rule the earlier parts added (`specs/0021` "Phases" lists them and
its close-out ruling 8 carries these: every log pins the spec's
hash; a figure labelled another log's is read from that log or
labelled as measured here; an exemption counts tokens in the form,
never lines; a superseded attempt stays on disk under
`<home>/.cache/emend-build/<phase>/`; no regex stand-in is printed
in the placeholder form; planning runs no cargo while a logged
script or a pre-check runs; the push script's authorisation as
labelled inputs, change 8; the size log's cause control and the run
log's record-commit naming); the scan log last; at most two
independent checks, each check's prompt giving the checker its
directory as `<phase>/check<n>` and saying the answer writes paths
so; report; STOP. No push during the part; the close-out push
follows P4 and carries the record commit 5da68da of part 4c's
close-out with this part's.

Part 4d's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0021-tables.md` as planning left them after part 4c's
close-out (the close-out rulings 1 to 10 and the ROADMAP rows), and
this spec. Part 4d's first code commit carries, before any other
change, the three items of `specs/0021`'s P3 verification rulings 2
and 4: a test pinning `remove_table` on a sheet added in the same
write (refused with "no table of that name", the catalogue's
sentence; P1 ruling 5); two `//!` edits to the
`package` header's catalogue, that a header cell holding a formula
means a kept `<c>` with `<f>`, and that the entry-name allocation
compares ASCII case folded.

### P1: comments, the addition and the read

The doc commit; the first code commit as above; then the feature
`comments` declared in the crate and turned on by the tool,
`Comment`, `CommentEntry`, `comments`, `add_comment` with the
author and text rules, the comments part and the VML part created
or appended, the allocation of the two entry names and the VML
ids, the overrides and relationships, the `legacyDrawing` creation
in schema position and the retag, the threaded-comments refusal,
the messages of this phase (all but "no comment on that cell"),
and `--comment-sample` without its removal. Tests on byte-built
parts: the comments read from a part with two comments by two
authors (runs with `rPr`, flattened), with a plain `t`, and from a
sheet with none; a `comment` without `ref`, a relationship whose
part is missing and a root without `commentList` each refused; the
created parts' bytes checked against literal bytes (the comments
part, the VML part with `D` 1 and `S` 1025; a text with leading
whitespace getting `xml:space`); an addition to a sheet with both
parts (the comment and the shape appended, the kept ones
byte-identical, the author reused when equal and added when not,
`D` the part's own and `S` past the part's largest in its range);
to a sheet with both parts in LibreOffice's form (`shape_<n>` ids,
no `idmap`, a declaration: `D` allocated, `S` `1024 * D` plus one
above the kept count); to a part whose kept count is below its
largest `_x0000_s` number (the number wins); to a sheet with a VML
part of form controls and no comments part (the comments part
created, the shape appended, `D` read, a control's `o:spid` number
counted); to a sheet
with a comments part and no `legacyDrawing` (refused); to a VML
part with no XML declaration and prefixes of its own; to the
template part (both parts created, the rels part created, the root
retagged, `legacyDrawing` before the root's end; the log prints
the output); to a part with `tableParts` and `extLst` (created
before `tableParts`); a replacement of a kept comment (the kept
element and shape skipped, the addition appended); two additions
on one cell (the last wins); a sheet with a `THREADED_COMMENTS`
relationship (refused); two `legacyDrawing` elements (refused); the
call-time refusals (an empty author, a 256-character author, an
empty text, a 32768-character text, a cell off the grid); a comment
edit with a shift refused, and on a replaced part refused; the
tool's line. The gate: the crate built and tested with the feature
off and on, on both targets, and the `pub fn` and `pub const` first
lines listed with it off (the part's calls absent) and on
(present), as part 4c's P1 did; every later log that runs the
crate's tests runs them with both features on. The negative log
through the binary: `--comment-sample` on a fixture from the
template with a comments part and no `legacyDrawing` (refused) and
on one with two `legacyDrawing` elements (refused), each with a
control. The run log: `--comment-sample` on both targets, the
checks as "Harness runs", the keep and the edit open check, recorded
as `records/m2p4d-p1-<utc-date>-<shortsha>.md` and `.txt`.

### P2: the comment removal

The doc commit (the P1 verification rulings); the carry of P1's
verification, if any, as the first code commit; then
`remove_comment` with its skips and drops and its message, and
`--comment-sample`'s in-memory removal. Tests: a removal of one of
two comments (the element and the `Note` shape gone, the rest of
both parts byte-identical, the authors kept), of the last comment
on a sheet whose VML holds a control shape (the comments part, its
relationship and override gone; the VML part and `legacyDrawing`
kept with the control), of the last comment on a sheet whose VML
holds notes only (both parts, both relationships, both overrides
and the element gone), of an absent cell (refused), of a cell this
write adds a comment on (refused), named twice (refused); a removal
and an addition on different cells in one write; a removal on a
part whose comments part is missing refused at the write. The
negative log: the sample on a fixture whose comments relationship
names a missing part (refused), with a control. The run log:
`--comment-sample` with the removal on both targets, the removal's
counts asserted, recorded as `records/m2p4d-p2-<utc-date>-
<shortsha>.md` and `.txt`.

### P3: images

The doc commit (the P2 verification rulings); the carry, if any;
then the feature `images` declared and turned on by the tool,
`Image`, `ImageEntry`, `Anchor`, the header reads, `images`,
`add_image` with the media entry, the drawing part created or
appended, its rels part, the overrides and relationships, the
`drawing` creation in schema position and the retag, the messages
of this phase, the tool's constant PNG with its producer quoted,
and `--image-sample`. Tests: the images read from a drawing with a
`pic` in a `twoCellAnchor`, one in a `oneCellAnchor`, one in a
`grpSp`, one in an `AlternateContent`, a chart's `graphicFrame`
(no entry) and an `absoluteAnchor` with a `pic` (`from` (0, 0)),
and from a sheet with none; a `blip` without `r:embed`, an
`r:embed` with no relationship, a relationship to a missing entry,
each refused; the header reads on a PNG (the tool's constant), a
baseline JPEG and a progressive JPEG built as byte literals with
their size fields set, a PNG whose `IHDR` is not first, a JPEG
with no `SOF`, a 23-byte PNG, a GIF signature, each refused; the
size override; a size whose EMU value overflows (refused); the
created drawing's bytes against literal bytes with `I` 1 and the
`r:id` of a created rels part; an addition to a sheet with a
drawing of two anchors (`I` 3, appended, the kept bytes
byte-identical, the `a` and `r` namespaces on the added elements
when the root binds neither, the root's prefixes when it binds
them), to a drawing whose root is `userShapes` (refused), to the
template part (the drawing created, both rels parts created, the
root retagged, `drawing` before the root's end), to a part with
`legacyDrawing` (created before it); the media name skipping names
in use over both extensions (`image1.png` and `image2.jpeg` taken
gives `image3`); two images in one write; two `drawing` elements
(refused); an image edit with a shift refused. The negative log
through the binary: `--image-sample` on a fixture from the
template whose drawing relationship names a missing part (refused)
and on one with two `drawing` elements (refused), each with a
control. The run log: `--image-sample` on both targets, recorded
as `records/m2p4d-p3-<utc-date>-<shortsha>.md` and `.txt`, and
`--comment-sample` once more, byte-identical to P2's run by sha256.

### P4: the catalogue and limits, the probes, the size record

The doc commit (the P3 verification rulings); the carry, if any;
then the `package` header's catalogue and limits for this part
(the messages marked by feature; the shapes this part writes and
reads; what it never writes: formatting runs, a comment's size or
position beyond the fixed box, a two-cell anchor, an SVG, an image
removal, threaded comments; what the read gives for a threaded
comment); `probe-emend-comments-write-xml` and `probe-emend-
images-write-xml` with the size-probe manifest's features; the
size log as "Size" says, recorded as `records/size-m2p4d-<utc-
date>-<shortsha>.md`; the negative log through the binary for every
message of the part reachable there, the unreachable ones stated
with the test that covers each; both samples again on both targets
with the size log's release binaries, byte-identical to P3's runs
by sha256, recorded as `records/m2p4d-p4-<sample>-<utc-date>-
<shortsha>.md` and `.txt`. The open record of the part's keep.

### Close-out push (after P4)

As part 4c's: the self-scan rule on the push and ci-run scripts,
`ci-run.log`, then the post-stage `scan.log` over the pushed range
with `push.log` and `ci-run.log` as its siblings, then one record
commit holding the three logs, the `ANTHROPIC_API_KEY` check; the
push script's change 8 kept; the push script's previous-record
pattern naming part 4c's record commit 5da68da (`specs/0021`
close-out ruling 3), its base part 4c's close-out doc commit
c8bfaf9 resolved by subject, its tip this part's close-out doc
commit; pre mode on a clean tree, so every planning ruling the push
carries is in a doc commit before it; one independent check before
the report (`specs/0021` close-out ruling 6). The push carries
5da68da and this part's commits, on planning's word and the
maintainer's authorisation typed in the implementing session.
Before the push, the Excel check for Milestone 2 parts 1 to 4c
(every corpus row of every sample, on the laptop), deferred by the
maintainer on 2026-09-27, 2026-09-29 and twice on 2026-09-30, is run
or deferred again by a decision recorded here; this part's two
samples join that check's list.

## Decisions for the maintainer

Decided by the maintainer 2026-09-30: 1 to 6 as recommended (A in
each), in the reply "send spec 4d to impl" to planning's outline,
typed in the planning session; 7 recorded.

1. Packaging: A (recommended), two features `comments` and
   `images` with a probe each; B, one feature.
2. Comment display: A (recommended), the comments part and the VML
   part with one hidden shape per comment, openpyxl's shape, since
   Excel shows no comment without the VML; B, the comments part
   only, which Excel may repair.
3. Threaded comments: A (recommended), a sheet with a threaded-
   comments relationship is refused for comment edits, proven by a
   built fixture since the corpus has none; B, edit the legacy part
   anyway, which desynchronises the two.
4. Image size: A (recommended), PNG and JPEG only, as openpyxl; the
   pixel size read from the PNG `IHDR` and JPEG `SOF` headers by
   this part's own code, no new crate, with an optional caller
   override; B, the caller must always give the size.
5. Existing drawing: A (recommended), an image is appended as a
   one-cell anchor to a sheet's existing drawing part and its
   relationships, so the corpus sheets with charts exercise the
   main case; B, refused when a drawing exists.
6. Replace rule: A (recommended), adding a comment on a cell that
   has one replaces it, openpyxl's assignment semantics; images are
   never overlap-checked; B, refused.
7. The Excel check for parts 1 to 4c: deferred on 2026-09-27,
   2026-09-29 and twice on 2026-09-30 by the maintainer, to run
   before this part's close-out push at the latest, or be deferred
   again by a decision recorded here. Recorded, not asked.

## Planning rulings during P1 (2026-09-30)

Made on the implementing session's survey of the corpus's VML
parts (counts only, confirmed by planning's own count) after the
doc commit 77ed347, before any code. The P1 logs pin the spec at
77ed347's hash a207e175; these rulings go in P2's doc commit.

1. The shape id `S` of an appended shape. The spec's rule read "one
   above the largest `_x0000_s` number any shape of the workbook's
   VML parts carries", which has no value for the 5 workbooks whose
   VML parts carry LibreOffice's `shape_<n>` ids only (22 of the
   corpus's 31 shapes; every one of the run's 5 append cases).
   Ruling: the workbook-wide read is withdrawn. `S` is `1024 * D +
   K` in every case, `D` the part's own `idmap` `data` or the
   allocated one, `K` the larger of one above the kept shape count
   (every kept shape counted, whatever its id's form) and one above
   the largest `n - 1024 * D` over the part's own shapes whose `id`
   or `o:spid` is `_x0000_s<n>` within `D`'s range. Only the part
   being appended to is read. Uniqueness across the workbook rests
   on each part's `idmap` being distinct, Excel's own invariant,
   stated in the limits. "Comments: the parts", the P1 tests and
   the run log's VML check are corrected above to this rule.
2. `o:spid` counts. Excel puts a form control's number on `o:spid`
   with an `id` of another form (1 corpus shape); the read covers
   both attributes so an appended note never takes a control's
   number. In ruling 1 above.
3. The facts said VML parts in 7 workbooks; the count is 8 (the
   11 parts; planning's first count ran over 26 of the 27 files,
   the template-extension workbook missed). Corrected above.
4. The first code commit (the three `specs/0021` carries) does not
   depend on these rulings and may be made at once.
5. The carried `remove_table` test's expected message. "Phases"
   and planning's instruction said "no such part"; the catalogue's
   sentence that `specs/0021` P3 ruling 2 asks the test to pin
   says "no table of that name", the removal finding no file table
   before any part check, and the code does that (the implementing
   session measured it on a throwaway test, uncommitted). Planning's
   error; the test asserts "no table of that name". Corrected above.

## Planning rulings during P2 (2026-10-01)

Made on the implementing session's pre-log plant check after the
code commits 89045f2, 193aa84 and fcd254b on the doc commit 9523065
(this file at 3ef8b8a6, which the P2 logs pin), before any log.

1. The carry's removed-part branch (a comment edit on a sheet whose
   comments part this write removes) is load-bearing on one path
   only: a removal that empties the part, where the drop skips an
   already-removed part and nothing is replaced; every carried test
   adds a comment, and an addition replaces the part, which part
   1's check already refuses with the same message, so a plant
   turning the branch off left all six tests passing. Ruling: one
   more code commit before the logs, "the carry's removal case",
   adding the test in the probe's shape (`remove_part` of the
   sheet's comments part, then `remove_comment` of its only
   comment, refused with "an edit to a removed part"); the plant
   V1 targets that test; the gate takes four code commits and the
   test log the test. The gap is not left for a later carry: a
   branch no test holds is the kind of thing a plant check exists
   to find, and the fix is one test.

## Planning verification and rulings on P1 (2026-10-01)

Verified by planning from primaries: the doc commit 77ed347 holds
the three files at the hashes planning sent (a207e175, a0322eeb,
7f723b2a); the carry 39d7ab7, the crate 96dde75, the tool f7d6a7d
and the record 2d675da follow on 5da68da; 5 of 5 messages end with
the Co-Authored-By line only, ASCII; the five logs and four records
at the reported hashes, each log ending "all as expected: yes" and
the scan RESULT: PASS with 0 hits, every log pinning a207e175;
planning's own build at f7d6a7d (`<home>/.cache/emend-planning/
p4d-p1-build.txt`): the crate's tests 512 off and 530 on, 600 with
`metadata,tables,comments`, 587 with `rules,tables,comments`, 689
all features, on both targets, the workspace 854 on both, clippy 0
in five configurations, doc 0 in three; planning's own run of
`--comment-sample` with its own release binaries (`p4d-p1-run.txt`)
27 of 27 outputs and lines byte-identical to the phase's run
directory on both targets, host and i686 identical, and
planning's own check of every output: 22 with both parts created
at exactly the bytes "Comments: the parts" gives (the entry names
the smallest free, `D` and `S` as the rule gives, the anchor from
the row), 5 with both parts appended to (the kept bytes a prefix,
one comment and one shape more, the shape's id as P1 ruling 1
gives it from the input part alone); planning's own privacy scan
5da68da..2d675da, 5 commits, 871 blobs, 0 findings;
`ANTHROPIC_API_KEY` unset; the diff read: the comments and VML
writers, the appended forms, the `K` rule (`kept.max(highest) +
1` over `id` and `o:spid` numbers in the block's range), the
threaded refusal, the schema-position creation and the shared
allocation and retag.

1. The independent check's four observations. (1) The literal
   "the worksheet part's root is not worksheet" is the cell
   layer's catalogued message, written as a literal in two other
   places of `package.rs` already: recorded, no change. (2) `D`'s
   allocation reads the input's VML entries the write does not
   remove, not a VML or comments part the caller adds or replaces
   through the part calls in the same write; the appended forms
   likewise read the input's part. Ruling: a comment edit on a
   sheet whose comments part or VML part this write adds, replaces
   or removes is refused at the write with part 1's messages
   ("an edit to a removed part", "a cell edit to a part this write
   replaces", reused), the way a cell edit on a replaced worksheet
   is; a test pins each; carried to P2's first code commit, with
   the code where the refusal is missing. (3) `K` counts `v:shape`
   root children: Excel writes every object as one; stated in the
   P4 limits with (2)'s rule. (4) A `commentList` before `authors`
   and a root other than `comments` refused as cannot-be-read:
   accepted, the catalogue says so at P4.
2. The four unreferenced commits (284259a, 758e7d1, 5894b2c,
   776e08c), rebuilt by a soft reset before any log ran: a
   superseded attempt in "Phases" is a log's, and no log ran on
   them; they are the implementing session's git history, in no
   primary, as the check said. Recorded; no change. The
   implementing session's gotchas carry both slips.
3. The negative log reaches three messages and states the two
   call-time ones unreachable with their tests, and reaches the
   threaded refusal and a missing relationship beyond the spec's
   two cases: accepted.
4. The run: 27 of 27 on both targets, 22 created and 5 appended,
   every flag and plant, the keep 1200 files identical across
   targets, 600 open-check pairs with 0 refusals, the previous
   record named by e1f3736. Accepted.
5. P1 is DONE. P2 is released: its doc commit carries this file
   with the P1 rulings and this section, and `ROADMAP.md`'s P1
   row.

## Planning verification and rulings on P2 (2026-10-01)

Verified by planning from primaries: the doc commit 9523065 holds
this file at 3ef8b8a6 and `ROADMAP.md` at c0dda773, the hashes
planning sent; the carry 89045f2, the removal 193aa84, the tool
fcd254b, the removal case f255959 (P2 ruling 1) and the record
0eadfec follow on 2d675da; 6 of 6 messages end with the
Co-Authored-By line only, ASCII; the five logs and four records at
the reported hashes, each log ending "all as expected: yes", the
scan RESULT: PASS with 0 hits, every log pinning 3ef8b8a6;
planning's own build at f255959 (`p4d-p2-build.txt`): the crate's
tests 512 off and 544 on, 614, 601 and 703 with the other feature
sets, on both targets, the workspace 868 on both, clippy 0 in five
configurations, doc 0 in three; planning's own run of
`--comment-sample` with its own release binaries (`p4d-p2-run.txt`)
27 of 27 outputs and printed lines byte-identical to the phase's
run directory on both targets, the lines carrying
`comments_removed` and `parts_removed`, host and i686 identical,
planning's own check of every output's parts as at P1 (22 created,
5 appended), and the phase's 54 outputs byte-identical to P1's run
directory's, as an in-memory removal implies; planning's own
privacy scan 2d675da..0eadfec, 6 commits, 878 blobs, 0 findings;
`ANTHROPIC_API_KEY` unset; the diff read: `comment_part_untouched`
and its six tests, the removal's skips, the two VML drop cases, the
removal-case test.

1. The independent check's four reservations. (1) The doc comment
   on the record order (the part refusals "first") does not match
   the code, which refuses a cell this write adds or names twice
   while grouping, before any part check; a twice-named removal on
   a removed sheet gets "no comment on that cell". Both are
   refusals and nothing is written; the comment is wrong, not the
   code. Ruling: the doc comment corrected to the code's order,
   carried to P3's first code commit. (2) The stale comment "a
   comment rewrites the worksheet only when its `<legacyDrawing>`
   is created" (it is also rewritten on a drop): corrected, same
   carry. (3) A `v:shape` with no `ClientData` counts as "another
   `ObjectType`" and keeps the VML part and the element: a plain
   VML shape is a drawing object and keeping it is right. Ruling:
   the reading stands; a test pinning it, same carry; the P4 limits
   say "a shape of another `ObjectType` or none". (4) The tool
   asserts `parts_removed` by count: the removal's exactness is
   pinned by the P2 unit tests (the part equal to the input less
   one element; the two drop cases) and the count is the sample's
   assertion. No change.
2. The report's account of the pre-log plant check and probe, the
   option label "(a)" and the pre-check log's location are the
   implementing session's own words with no primary, as the check
   said; the outcome is P2 ruling 1 and f255959. Recorded.
3. The negative log reaches the missing-part refusal through the
   binary and states "no comment on that cell" unreachable with
   its test; the removal keys read as expected on the three
   fixtures. Accepted.
4. The run: 27 of 27 on both targets, every flag and plant, the
   open check against P1's record 2d675da. Accepted.
5. P2 is DONE. P3 (images) is released: its doc commit carries this
   file with the P2 rulings and this section, and `ROADMAP.md`'s
   P2 row; its first code commit carries ruling 1's three items.

## Out of scope

- Image removal: openpyxl has no call for it; an image's anchor
  and media stay as they are. Planning's choice, recorded here;
  a later part may add `remove_image` with the drawing-container
  skip and the media drop when nothing else references it.
- Formatting runs in a comment, a comment's box size, position or
  visibility, a comment's `shapeId`, and the `xr` namespace.
- Threaded comments (`xl/threadedComments/`, `xl/persons/`): read
  as their legacy fallback, never written; a sheet that has them
  refuses comment edits (decision 3 A).
- Two-cell and absolute anchors for an added image, an image's
  rotation, crop or effects, SVG (and the `AlternateContent` pair
  Excel writes for one), EMF, GIF, BMP, TIFF; an image's `descr`.
- Form controls, shapes, charts and groups in a drawing or a VML
  part: copied raw, read past, never written.
- A `Default` content-type element: this part writes overrides.
- What Excel or LibreOffice shows for a comment's box or an
  image's placement: the file opens; the Excel check is the
  maintainer's.
