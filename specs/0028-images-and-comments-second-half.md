# 0028: Milestone 3 part 6, images and comments, the second half

Status: APPROVED by the maintainer 2026-10-05 ("A on all
(Recommended)", chosen in the planning session's question:
decisions 1 to 8 as planning recommended; 9 taken as "Defer again
(11th)"). Sent to the implementing session the same day.

Context: `CLAUDE.md` ("What emend is": like openpyxl; "Dependencies
and licences": anything beyond the core behind a Cargo feature off
by default; "32-bit and sizes": stream where the format allows it),
`ROADMAP.md` ("Milestone 3", the outline approved 2026-10-01, item
6: image removal with the media drop, two-cell and absolute
anchors, a comment's box size and visibility, worksheet copy
within a workbook; the features `images` and `comments` grow;
25000 to 35000), `specs/0022-comments-and-images.md` (the comment
and image types, calls and parts this part grows; its "Out of
scope" names each item here), `specs/0016-new-workbook-and-
sheets.md` ("Adding a sheet", "Removing a sheet": the removal walk
this part reuses for the media drop, the workbook rewrite the copy
reuses), `specs/0024-rich-text.md` (`add_rich_comment`), `specs/
0027-read-back.md` (the close-out rulings, carried here), `specs/
0023-scale.md` (the decision record's carry for this part: the
whole VML and drawing reads, decision 8), `specs/0001-bootstrap.md`
("Rules for every phase"). ECMA-376 Part 1, 20.5.2 (the
spreadsheet drawing anchors `oneCellAnchor`, `twoCellAnchor`,
`absoluteAnchor`, `from`, `to`, `ext`, `pos`), 18.2.19 (`sheet`),
and Part 4's VML `ClientData` children (`Visible`, `Row`,
`Column`) are cited where the design leans on them; no text is
copied from them. openpyxl's `ws._images`, `OneCellAnchor`,
`TwoCellAnchor`, `AbsoluteAnchor`, `Comment.width`,
`Comment.height` and `Workbook.copy_worksheet` are the
behavioural reference, read only.

## Goal

The second half of what part 4d of Milestone 2 left out, each item
named in `specs/0022`'s "Out of scope": an image removed with its
media entry dropped when nothing else in the package references
it; an image added on a two-cell or an absolute anchor; a
comment's box given a size and a visibility when it is added, and
read back; and a worksheet copied within its workbook with its
cells, formats, merges, settings, hyperlinks and comments, as
openpyxl's `copy_worksheet` copies (its documentation: cells with
values, styles, hyperlinks and comments, and the sheet's
dimensions, format and properties; not images or charts). The
features `images` and `comments` grow; the copy goes behind a
feature of its own (decision 1). The core does not move.

## Size and packaging (decision 1, before any code)

The budget figure after part 5 is 436928 section bytes against
the ceiling of 458752 (`records/size-m3p5-2026-10-05-6b6125b.md`):
21824 of headroom, held for fixes. The outline's estimate for
this part is 25000 to 35000 for `images` and `comments` together.
Planning's, priced from parts 2 to 5 (`richtext` 13368 against an
estimate of 40000 to 55000; `sheetprops` 28268 against 35000 to
50000; `rowcol`'s growth 22888 against 20000 to 30000; `read`
51972 against 30000 to 45000: the actual ran from a third of the
estimate to a sixth above it), is 30000 to 45000 over the three
features: the removal with the walk reused 8000 to 12000 on
`images`, the two anchors 4000 to 7000 on `images`, the box 5000
to 8000 on `comments`, the copy 12000 to 20000 on its feature.
The core's growth is what the feature gates add, expected near 0;
above 2000 section bytes, or a feature's growth above 25000,
planning is told in the report.

- A (recommended): `images` and `comments` grow in place, their
  two probes measuring the growth; one new Cargo feature `copy`,
  off by default, declared beside `read` with no dependency of its
  own, holding `copy_sheet`; the comment carry of a copy needs
  `comments` too, measured as a pair probe (`probe-emend-
  copycomments-write-xml`, `copy` and `comments`, over the
  comments probe) as `richtext`'s comment pair is. One probe for
  `copy` alone (`probe-emend-copy-write-xml`, over the format
  probe). 28 probes.
- B: `copy_sheet` in the core, beside `add_sheet`: 12000 to 20000
  of the 21824 headroom gone for a call a consumer that only
  edits cells never makes.
- C: `copy_sheet` under `comments`: a consumer copying a sheet
  with no comments carries the comment writers, and the feature's
  figure stops meaning one thing.

## Facts planning checked (2026-10-05)

- The corpus, counted by planning with Python's `zipfile` and
  regular expressions over every regular file of the directory
  (27 workbooks, 95 worksheet parts), names and counts only. 38
  drawing parts in 16 workbooks hold 108 `twoCellAnchor` and no
  `oneCellAnchor` or `absoluteAnchor` (the `editAs` attribute:
  `oneCell` 43, `absolute` 34, `twoCell` 16, absent 15); 22 `pic`
  elements, every one in a `twoCellAnchor`; 30 `graphicFrame`
  (charts) and 69 `sp` shapes; 34 `r:embed` attributes and no
  `r:link`; 11 `xfrm` with `rot`, 1 `srcRect`, 54 `cNvPr` with
  `descr`, 36 `hlinkClick`. 26 media entries (16 `.png`, 10
  `.svg`); in 5 cases one media entry is the target of two
  drawings' rels parts of one workbook (so a drop must look past
  the drawing whose image goes); 34 drawing relationships to
  media and 26 to charts; 4 drawings have no rels part. No
  drawing references one media entry through two relationships.
- 11 VML parts in 8 workbooks hold 31 `v:shape`, 28 of them
  `Note` shapes (the corpus's 28 comments in 9 comments parts);
  22 `Note` shapes carry `width:` and `height:` in their `style`,
  all in `pt`, with `visibility:hidden`; 6 carry neither a size
  nor a visibility; none carries `visibility:visible` or an
  `x:Visible` child. Every `Note` shape carries `x:Anchor`,
  `x:MoveWithCells` and `x:SizeWithCells`.
- Of the 95 worksheet parts: 37 carry a `drawing` element, 11 a
  `legacyDrawing`, 3 both; 37 `tableParts`, 44 `dataValidations`,
  12 `conditionalFormatting`, 33 `hyperlinks`, 47 `mergeCells`, 2
  `autoFilter`, 12 `extLst` (7 with `x14:conditionalFormatting`,
  4 with sparklines), 10 `tabSelected="1"`, 31 a `codeName` on
  `sheetPr`, 50 a `pageSetup` with `r:id`, 2 `controls`; none
  `customSheetViews`, `oleObjects`, `picture` or
  `legacyDrawingHF`. Worksheet relationships by kind: hyperlink
  on 12 parts (198 external relationships in all), printerSettings
  100, table 37, drawing 37, vmlDrawing 11, comments 9,
  pivotTable 5, ctrlProp 2, slicer 2, timeline 1. 9 workbooks
  carry a calculation chain; 160 defined names, 38 with
  `localSheetId`. No threaded comments, no `persons` part.
- The crate (at 6aba171): `Image { bytes, size }`, `Anchor {
  OneCell, TwoCell, Absolute }`, `ImageEntry { from, anchor, part,
  bytes }`, `Comment { author, text }`, `CommentEntry { cell,
  author, text, runs (under richtext) }`; the drawing reader
  keeps, per picture, its anchor kind, its `from` and its
  `r:embed`; the VML reader keeps each shape's byte span, its
  numbers, whether it is a `Note` and its row and column; the
  note shape writer fixes the style at `width:108pt;height:
  59.25pt` and `visibility:hidden`; `Package::reachable(root,
  skipped, stops)` is the removal walk, taking relationships not
  to follow and parts not to leave; `add_sheet` allocates the
  entry, its `Override`, the workbook relationship and the
  `<sheet>` with `sheetId` one above the largest; `Sheet { name,
  sheet_id, rel_id, state, part, kind }`.

## Design

### Crate

Under `images`: `ImageAnchor`, `EditAs`, `add_image_at`,
`remove_image`. Under `comments`: `CommentBox`, `set_comment_box`,
the field `shape` on `CommentEntry`. Under `copy`: `copy_sheet`.
`add_image`, `images`, `add_comment`, `add_rich_comment`,
`remove_comment` and `comments` keep their signatures; `add_image`
is `add_image_at` with a one-cell anchor at the cell with zero
offsets, one definition.

### Anchors

```
pub enum ImageAnchor {
    OneCell { from: CellRef, offset: (u32, u32) },
    TwoCell { from: CellRef, to: CellRef,
              from_offset: (u32, u32), to_offset: (u32, u32),
              edit_as: EditAs },
    Absolute { position: (u32, u32) },
}
pub enum EditAs { TwoCell, OneCell, Absolute }
```

Both derived `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`. Every
offset and position is in pixels, written as EMU at 9525 per pixel
as the one-cell `ext` is; a value whose EMU form exceeds
`i32::MAX` is refused at the call ("an image anchor is out of
range"), as is a `to` cell before `from` in row or column, or
either cell off the grid (part 3's index message). `EditAs` writes
the `editAs` attribute (`twoCell`, `oneCell`, `absolute`; written
always, so the corpus's absent form is never produced).

`add_image_at(&mut self, part, anchor: ImageAnchor, image: Image)
-> Result<(), Error>`: `add_image`'s rules and parts, the anchor
element chosen by the variant. A one-cell anchor writes `specs/
0022`'s element with the offsets in `colOff` and `rowOff`. A
two-cell anchor writes `twoCellAnchor` with `editAs`, `from` and
`to` (each `col`, `colOff`, `row`, `rowOff`), then the `pic` and
`clientData` as the one-cell form; an `Image` whose `size` is
`Some` is refused at the call ("an image size does not apply to a
two-cell anchor"), since the cells and offsets set the box and
nothing is silently ignored. An absolute anchor writes
`absoluteAnchor` with `pos` (`x`, `y`) and `ext` from the image's
size as the one-cell form takes it. Every created element takes
the root's prefix and the `a` and `r` bindings as `specs/0022`
says. `images` reads each form as it does today (the entry's
`anchor` says which; `from` is (0, 0) for an absolute anchor as
`specs/0022` defines).

### Image removal

`remove_image(&mut self, part, index: usize) -> Result<(), Error>`:
records the removal of the image that `images(part)` lists at
`index` (decision 2). At the write: the drawing part is read as
`images` reads it; no entry at `index` is refused ("no image at
that position"), as is an index named twice, or an index on a
part this write also adds an image to (the caller writes twice,
as `specs/0021` P2 rules for a table). The anchor element holding
the picture is skipped whole in the drawing pass (the kept
children copied byte for byte, the part's root held to its end
tag as the append holds it); every `r:embed` and `r:link` inside
the skipped anchor is collected (a picture with an `svgBlip`
extension names a second media entry this way; the corpus has 10
`.svg` entries beside their `.png`); each collected id whose
relationship no kept element of the part references is removed
from the drawing's rels part; the media entry each removed
relationship targets is removed, with its `Override` when the
content types part holds one, when it is not in K, where K is the
set `Package::reachable` gives from the package relationships
with the removed drawing relationships in `skipped` (so a media
entry two drawings share stays: 5 corpus cases). A drawing left
with no root child other than `AlternateContent` holding none,
that is, no anchor at all, is dropped as `remove_comment` drops an
emptied comments part: the part, its rels part, its `Override`,
the sheet's `drawing` relationship and the `drawing` element go;
a drawing with any anchor left (a chart, a shape) is kept. An
addition after a removal on one part in one write is refused as
above; a removal on a sheet this write removes, replaces or
shifts meets part 1's and part 3's messages as an addition does.

### A comment's box

```
pub struct CommentBox { pub width: u32, pub height: u32,
                        pub visible: bool }
```

Derived `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`; `width` and
`height` in pixels, 1 to 65535 each (decision 4: openpyxl's
`Comment.width` and `height` are pixels, its defaults 144 by 79),
refused at the call otherwise ("a comment box is out of range").

`set_comment_box(&mut self, part, cell: CellRef, shape: CommentBox)
-> Result<(), Error>`: records the box of the comment this write
adds on `cell` through `add_comment` or `add_rich_comment`, in
either order. At the write: no addition on that cell is refused
("no comment addition on that cell"); a box set twice on one
cell, the last wins, as two additions do. The note shape is
written with `width:<W>pt;height:<H>pt` where `W` and `H` are the
pixel values times three quarters, printed as an integer when
exact and with at most two decimals otherwise (144 gives `108pt`,
79 gives `59.25pt`, which are the defaults the writer prints
today, so an unset box writes the bytes it writes today, asserted
by test); `visible` true writes `visibility:visible` in the style
and an `x:Visible` child (empty) in `ClientData` immediately
before `x:Row`; false writes `visibility:hidden` and no child, as
today. Planning read `ClientData`'s content model as a choice of
children in any order; the implementing session confirms this
against the schema before the writer is written, and if it is a
sequence, `x:Visible` goes where the sequence puts it (a P1
ruling records which). A box on a kept comment is not changed by
this call: the caller replaces the comment (`specs/0022` decision
6, the replacement case) and sets the box on the addition.

The read: `CommentEntry` gains `shape: Option<CommentBox>`, `Some`
when the sheet's VML part holds a `Note` shape whose row and
column are the cell's and whose `style` carries both `width:` and
`height:` in `pt` or `px` (`px` taken as is; `pt` times four
thirds, rounded to the nearest pixel; `108pt` reads 144 and
`59.25pt` 79, so this part's own writes read back equal), with
`visible` true when the style carries `visibility:visible` or
the shape carries an `x:Visible` child; `None` when the sheet has
no VML part, no `Note` shape for the cell, or the shape carries no
size (the corpus's 6). The read opens the VML part once per
`comments` call, after the comments part, held whole as the
append holds it (decision 8); a VML part the writer cannot read is
the cannot-be-read refusal as at the write.

### The copy

`copy_sheet(&mut self, name: &str, to: &str) -> Result<Vec<u8>,
Error>`: records a new worksheet named `to` at the end of the
list, a copy of the worksheet named `name` in the projected list,
and returns its entry name (decision 5: the caller names the copy;
openpyxl's `Copy` suffix is the caller's to write). `to` is
checked as `add_sheet` checks a name. Recorded as `add_sheet`
records its sheet, with the entry's bytes produced at the write by
one streaming pass over the source part instead of the template:

- Dropped: the `codeName` attribute of `sheetPr` (unique per
  workbook; 31 corpus sheets carry one), the `tabSelected`
  attribute of every `sheetView` (the copy is not selected), the
  `r:id` attribute of `pageSetup` (printer settings are not
  copied; openpyxl does not read them), and the root children
  `drawing`, `legacyDrawing`, `legacyDrawingHF`, `picture`,
  `oleObjects`, `controls`, `webPublishItems`, `tableParts` and
  `extLst` whole (images, charts, tables, controls and the
  extension list's conditional formats, validations, sparklines
  and slicer references are not copied, which openpyxl's copy
  does not copy either; the limits say so).
- Kept byte for byte: everything else, which carries the cells
  with their values, formulas and styles, the column and row
  formats, the pane, the merges, the hyperlinks, the data
  validations and conditional formats of the main namespace, the
  autofilter, the protection, the print options, margins, page
  setup and headers and footers. A `hyperlink` element's `r:id`
  resolves through the copy's rels part, created when the
  source's holds any relationship of kind `HYPERLINK`: those
  relationships copied with their ids and targets (external ones
  as external), nothing else. Any other element left in the copy
  that carries an `r:id` (a `customSheetView`'s `pageSetup`, none
  in the corpus; the pass checks every start tag kept) is refused
  ("a sheet copy meets a relationship it cannot carry"), nothing
  written.

Amendment (2026-10-08, `specs/0029` item 7, from P3 verification
ruling 2): in the dropped list above, "the `r:id` attribute of
`pageSetup`" reads "the relationship attributes of `pageSetup`
(the schema gives it `r:id` alone)"; the original line stands
above as written.
- Comments (decision 6): when the source has a `comments`
  relationship, the copy carries them when `comments` is on, else
  the copy is refused ("a sheet copy needs the comments feature").
  Carried as: the comments part copied byte for byte to a new
  comments part allocated as `specs/0022` allocates one, its
  `Override` and the copy's relationship; the VML part created
  by the note writer with one `Note` shape per `comment` of the
  part in document order, each shape's box the source shape's
  as the read gives it (the default when `None`); `legacyDrawing`
  created in schema position as an addition creates it. A source
  with threaded comments is refused as `add_comment` refuses
  ("a worksheet has threaded comments").
- The workbook: the `<sheet>`, relationship and `Override` as
  `add_sheet` records them (`state` not copied: the copy is
  visible). Defined names with the source's `localSheetId`
  (print titles, print area, the filter database) are not copied,
  as openpyxl's copy does not copy them; the limits say so. When
  the pass meets an `f` element, the write proceeds as a formula
  change (`specs/0013`'s rule: the chain dropped, a full
  calculation on load), since the copy's formulas are not in the
  chain.
- Refused at the call: `name` not in the projected list ("no
  sheet of that name"); a chartsheet ("a sheet copy needs a
  worksheet"); the name rules for `to`. Refused at the write:
  a source part with any cell, style, shift, settings, comment or
  image edit recorded in this write ("a sheet copy of a part this
  write edits"; the copy takes the file's bytes and nothing is
  silently left out; the caller writes twice), a source added in
  this write (part 1's "no such part"). Edits recorded on the
  returned name in the same write are refused with "no such part"
  as a comment on an added sheet is (decision 7); the caller
  writes, reopens and edits the copy.

### Errors

Nine new `Error::Package` messages, in the catalogue as the earlier
parts' are, one constant each, marked by feature:

`images`:
- "an image anchor is out of range" (at the call)
- "an image size does not apply to a two-cell anchor" (at the
  call)
- "no image at that position" (at the write)

`comments`:
- "a comment box is out of range" (at the call)
- "no comment addition on that cell" (at the write)

`copy`:
- "a sheet copy needs a worksheet" (at the call)
- "a sheet copy of a part this write edits" (at the write)
- "a sheet copy meets a relationship it cannot carry" (at the
  write)
- "a sheet copy needs the comments feature" (at the write)

Shared: part 3's index message, part 1's part messages, `specs/
0016`'s name rules and "no sheet of that name", `specs/0022`'s
cannot-be-read messages and "a worksheet has threaded comments",
the threaded refusal and the override rule. Nothing is checked
twice.

### 32-bit and streaming

Every count is `u32` or `u64` as elsewhere; an ordinal is `usize`;
EMU arithmetic is `u64` checked against `i32::MAX`. The copy's
pass streams the source part once, as the worksheet pass does. The
drawing and VML parts are held whole as `specs/0022` holds them,
and this part states that hold in the catalogue with the corpus
figures (38 drawings, the largest's anchor count printed by the
P2 run log; 11 VML parts, the largest 10 shapes) as the limit the
scale record carried here (decision 8): the scale tiers hold no
drawing or VML part, and a drawing is bounded by its anchors, not
by the sheet's cells.

### Size

Two probes grow and two are new, under decision 1 A. `probe-emend-
images-write-xml` adds, after its `add_image`, an `add_image_at`
on a two-cell anchor and a `remove_image` of the first image in a
second package opened on the first's output, the counts printed.
`probe-emend-comments-write-xml` adds a `set_comment_box` on its
addition and prints the read-back box. `probe-emend-copy-write-
xml` (feature `copy` of the probe crate, turning on `emend/copy`):
the format probe's work, then `copy_sheet` of the first sheet
under a fixed name, the sheet count printed after the write.
`probe-emend-copycomments-write-xml` (features `copy` and
`comments`): the comments probe's work, then the copy of the
sheet the comment went on, the copy's comment count printed. The
size log measures as part 5's did, classifies the twenty-six
probes of part 5's record against it, the two new ones listed as
new, prints each grown feature's cost and its growth (the probe at
the commit minus the same probe rebuilt at 6b6125b, both printed),
the `copy` feature's cost (the copy probe minus the format probe
rebuilt at the same commit) and the pair's (the copycomments probe
minus the comments probe at the same commit), the core's growth
(the format probe at the commit minus the same probe rebuilt at
6b6125b; expected near 0), the budget against 458752, and a cause
per probe; the cause control and the manifest cause listing stay.

Amendment (2026-10-08, `specs/0029` item 7, from P4 ruling 3): the
images probe's removal is made in a second package opened on the
first's output, written into memory and reopened, `images_removed`
printed; the comments probe's box is `set_comment_box` on AB10
(200 by 100, visible) in the addition's write, read back as
`box_width`, `box_height` and `box_visible`. The paragraph above
stands as written; this is how the probes read at 9cfd92e.

### Harness runs

`tools/emend-copy` gains four operations, given after the two
paths as the others are; none takes a reference, name or number
from its command line; every constant below is the tool's own. W
is the first worksheet part in `<sheets>` order and M its largest
row number as the tool's cell read gives it; a cell past the grid
refuses the sample whole with part 3's index message. No
reference and no text from the corpus is printed; the samples
print counts and flags.

- `--commentbox-sample <in> <out>` (P1): `add_comment(W, (M + 1,
  1), Comment { author: "emend", text: "emend box" })` then
  `set_comment_box(W, (M + 1, 1), CommentBox { width: 200,
  height: 100, visible: true })`; writes; reopens the output and
  prints `commentbox` with `comments_before`, `comments_after`
  (`+ 1`, asserted), `shapes_before`, `shapes_after` (`Note`
  shapes of W's VML part; `+ 1`, asserted), `sized_before` (kept
  entries whose `shape` reads `Some`), `box_equal` (1 when the
  added entry's `shape` equals the box set, asserted),
  `visible_after` (1, asserted). Expected results: as the comment
  sample's (the comments and VML parts `added` or `changed`, the
  rest `identical`), `lo` and `lo_in` `ok`.
- `--anchor-sample <in> <out>` (P2): `add_image_at(W, TwoCell {
  from: (M + 1, 1), to: (M + 3, 3), offsets zero, edit_as:
  OneCell }, Image { bytes: PNG, size: None })` and `add_image_at(
  W, Absolute { position: (96, 96) }, Image { bytes: PNG, size:
  Some((4, 4)) })`, PNG the tool's constant; writes; reopens and
  prints `anchor` with `images_before`, `images_after` (`+ 2`,
  asserted), `two_after`, `absolute_after` (entries by `anchor`
  kind; each `+ 1` over the input's, asserted), `anchors_before`,
  `anchors_after` (`+ 2`), `media_before`, `media_after` (`+ 2`:
  each addition is its own entry, as `specs/0022` allocates).
  Expected results: as the image sample's; `lo` and `lo_in` `ok`.
- `--remove-image-sample <in> <out>` (P2): `add_image(W, (M + 1,
  1), Image { bytes: PNG, size: None })`, a write into memory,
  that reopened; then `remove_image(W, n)` for `n` the last index
  (the addition) and, when the input's `images(W)` is not empty,
  `remove_image(W, 0)` too; writes the output; reopens it and
  prints `removeimage` with `images_before`, `images_after`
  (`images_before - kept_removed`, asserted), `kept_removed` (0
  or 1), `media_before`, `media_after`, `media_dropped`
  (`media_before - media_after`; 0 when nothing kept was removed,
  asserted; otherwise the count of media entries the removed
  picture referenced that no other relationship in the package
  targets, counted by the tool through the archive's rels parts
  and asserted equal), `drawing_dropped` (1 when the output has
  no drawing for W and the input had none, asserted), `anchors_
  before`, `anchors_after`. Expected results: when `kept_removed`
  is 0, every entry `identical` except the drawing's rels part,
  which may be `changed` with the same relationships (the run
  log's check), which proves the addition and removal leave the
  package as it was; when 1, W's drawing part `changed`, its rels
  part `changed`, each dropped media entry `removed`, the content
  types part `changed` when an `Override` went, the rest
  `identical`; `lo` and `lo_in` `ok`.
- `--copy-sample <in> <out>` (P3): S is the first worksheet in
  `<sheets>` order with a `comments` relationship, else W;
  `copy_sheet(S's name, "emend copy")`; writes; reopens and prints
  `copy` with `sheets_before`, `sheets_after` (`+ 1`, asserted),
  `cells_source`, `cells_copy` (through `cells`; equal, asserted),
  `comments_source`, `comments_copy` (equal, asserted),
  `merges_source`, `merges_copy`, `links_source`, `links_copy`
  (through `sheet_settings`; equal, asserted), `formulas` (1 when
  the source holds an `f`), `chain_before`, `chain_after` (0 when
  `formulas` is 1). Expected results: the copy's worksheet entry
  `added`, the workbook part, its rels part and the content types
  part `changed`, the comments part, VML part and rels part
  `added` when carried, `xl/calcChain.xml` `removed` when
  `formulas` and `chain_before` are 1, the rest `identical`; `lo`
  and `lo_in` `ok`.

The run log's own check over the extracted input and output
parts, with `ElementTree` and byte spans and a control that
reports a planted difference per check, printing no name, no
reference and no text from the corpus:

- commentbox: the added shape's `style` holds `width:150pt` and
  `height:75pt` and `visibility:visible`, and an `x:Visible`
  child; every kept shape byte-identical.
- anchor: the two added anchors' forms (`twoCellAnchor` with
  `editAs="oneCell"`, `absoluteAnchor` with `pos` 914400 by 914400
  and `ext` 38100 by 38100); the kept children byte-identical.
- removeimage: when a kept picture went, the output's anchors are
  the input's with that one left out, byte for byte; the drawing's
  rels part's relationships are the input's less the removed
  ids; for each media entry the input's drawing referenced, the
  log counts the rels parts of the whole input that target it and
  checks that `removed` happens exactly when that count is the
  removed relationships' count.
- copy: the copy's `sheetData`, `cols`, `mergeCells`,
  `hyperlinks`, `dataValidations`, `conditionalFormatting`,
  `sheetViews` (less `tabSelected`), `pageMargins` and `pageSetup`
  (less `r:id`) byte-identical to the source's; no `codeName`, no
  `drawing`, `legacyDrawing` (unless carried), `tableParts` or
  `extLst` in the copy; when carried, the copy's comments part
  byte-identical to the source's and its VML `Note` shapes one
  per comment at the source's rows and columns.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. Every phase's run
log runs the edit open check on the host over that phase's keep,
names the phase directories it compares against by their record
commit, prints the devices of `TMPDIR` and of the corpus
directory, and counts the calc-chain workbooks with the rule
covering them. The Excel check (decision 9): the four samples join
its list; the commentbox, anchor and copy outputs are the ones
whose Excel rendering the maintainer's check would show
(visibility, two-cell placement, the copied sheet's state).

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m3p6-<phase>/` (`gate.log`,
`test.log` on both targets with the feature off and on,
`negative.log`, `run.log`, `scan.log`, `size.log` in P4); every
rule the earlier parts added (`specs/0027` "Phases" lists them);
the scan log last; at most two independent checks; report; STOP.
No push during the part; the close-out push follows P4 and carries
the record commit 6aba171 of part 5's close-out with this part's.

Part 6's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0027-read-back.md` as planning left them after part 5's
close-out (the close-out rulings 1 to 4, the Close row and the
part 5 heading), and this spec. Nothing is carried into part 6's
first code commit (`specs/0027` close-out ruling 3).

### P1: a comment's box

The doc commit; then `CommentBox`, `set_comment_box`, the shape
writer's size and visibility, the `shape` read on `CommentEntry`,
the two messages, and `--commentbox-sample`. Tests on byte-built
parts: the default box writes today's bytes (the literal of
`specs/0022`'s shape); 200 by 100 visible written (`150pt`,
`75pt`, `visibility:visible`, the `x:Visible` child before
`x:Row`); the box set before and after the addition; set twice;
set with no addition (refused at the write); each range refusal;
the read of a `pt` shape, a `px` shape, a shape with no size
(`None`), a shape with `x:Visible` and no style visibility
(`Some`, visible), a sheet with no VML part (`None`); a round trip
of the sample's box through a byte-built part; with `richtext`
on, the box on an `add_rich_comment` addition; the tool's line.
The gate as part 5's P1. The negative log through the binary:
`--commentbox-sample` on a fixture whose VML part has no root end
tag (refused, with a control). The run log: `--commentbox-sample`
on both targets over the corpus, the checks as "Harness runs",
the keep and the edit open check, recorded as `records/m3p6-p1-
<utc-date>-<shortsha>.md` and `.txt`.

### P2: the anchors and the removal

The doc commit (the P1 verification rulings); the carry, if any;
then `ImageAnchor`, `EditAs`, `add_image_at` with `add_image` on
it, `remove_image` with the drawing skip, the relationship and
media drops through `reachable`, the emptied drawing's drop, the
three messages, `--anchor-sample` and `--remove-image-sample`.
Tests on byte-built parts: each anchor form's bytes against a
literal (offsets, `editAs` each value, `pos`, `ext`); `to` before
`from`, an offset overflowing, a size on a two-cell anchor, each
refused at the call; a removal from a drawing of three anchors
(the middle one, kept bytes byte-identical, its relationship and
media gone, the `Override` gone); a removal whose media another
drawing's rels part targets (the media kept, the relationship
gone); a removal whose picture holds an `svgBlip` (both media
gone); a removal leaving a chart (the drawing kept); a removal
emptying the drawing (the part, rels, `Override`, relationship
and element gone); an index past the list, twice, and with an
addition on the part, each refused at the write; a removal on a
removed, replaced or shifted part refused; the tool's lines. The
negative log through the binary: `--remove-image-sample` on a
fixture whose drawing's rels part names a missing media entry
(refused, with a control). The run log: both samples on both
targets, the largest drawing's anchor count printed (decision 8),
recorded as `records/m3p6-p2-<sample>-<utc-date>-<shortsha>.md`
and `.txt`, and `--commentbox-sample` once more, byte-identical
to P1's run by sha256.

### P3: the copy

The doc commit (the P2 verification rulings); the carry, if any;
then the feature `copy` declared and turned on by the tool,
`copy_sheet` with the pass, the hyperlink relationships, the
comment carry, the workbook recording through `add_sheet`'s path,
the formula-change rule, the four messages, and `--copy-sample`.
Tests on byte-built packages: a copy of a sheet with cells, cols,
merges, a pane, hyperlinks (internal and external relationships),
validations, a main-namespace conditional format, an autofilter,
protection, page setup with `r:id`, `codeName`, `tabSelected`, a
drawing, a legacy drawing with comments, `tableParts` and an
`extLst` (the copy's bytes against a literal; the parts added and
their relationships; the comments part equal; the `Note` shapes'
rows and columns; the workbook's new `<sheet>`); a source with a
formula (the chain dropped, `fullCalcOnLoad`); a source with
comments and the feature off (refused); a chartsheet (refused); a
kept `r:id` outside `hyperlink` (refused); a source with an edit
in the same write, a source added in the same write, an edit on
the copy in the same write, each refused; the name rules on `to`;
a copy then `move_sheet` of the copy in one write; a copy then
`remove_sheet` of the source in one write (the copy's parts
stay); the tool's line. The negative log through the binary:
`--copy-sample` on a fixture whose first sheet is a chartsheet
(refused, with a control). The run log: `--copy-sample` on both
targets, recorded as `records/m3p6-p3-<utc-date>-<shortsha>.md`
and `.txt`, and the three earlier samples once more,
byte-identical to their runs by sha256.

### P4: the catalogue and limits, the probes, the size record

The doc commit (the P3 verification rulings); the carry, if any;
then the `package` header's catalogue and limits for this part
(the messages by feature; what the copy drops, element by
element; the defined names not copied; the whole-part holds with
their figures, decision 8; the box read's rounding); the two
grown probes, the two new probes with the size-probe manifest's
features; the size log as "Size" says, recorded as `records/size-
m3p6-<utc-date>-<shortsha>.md`; the negative log through the
binary for every message of the part reachable there, the
unreachable ones stated with the test that covers each; the four
samples again on both targets with the size log's release
binaries, byte-identical to their runs by sha256, recorded as
`records/m3p6-p4-<sample>-<utc-date>-<shortsha>.md` and `.txt`.
The open record of the part's keep.

### Close-out push (after P4)

As part 5's: the self-scan rule on the push and ci-run scripts,
`ci-run.log`, then the post-stage `scan.log` over the pushed range
with `push.log` and `ci-run.log` as its siblings, then one record
commit holding the three logs, the `ANTHROPIC_API_KEY` check; the
push script's previous-record pattern naming part 5's record
commit 6aba171 (`^M3 part 5 close-out record`), its base part 5's
close-out doc commit 9d62635 resolved by its subject ("M3 part 5
close-out: commit planning docs (the P3 verification and rulings,
decision 7 deferred again)"), its tip this part's close-out doc
commit; pre mode on a clean tree; one independent check before
the report. The push carries 6aba171 and this part's commits, on
planning's word and the maintainer's authorisation typed in the
implementing session. Before the push, the Excel check for
Milestone 2 (every corpus row of every Milestone 2 sample, with
Milestone 3's samples on its list), deferred by the maintainer
eleven times, last at this part's scoping on 2026-10-05 (decision
9), is run (Milestone 2's acceptance record then follows Milestone
1's form) or deferred again by a decision recorded here.

## Planning verification and rulings on P1 (2026-10-06)

Verified by planning from primaries: the doc commit 7cfc6f5 holds
this file at 8adfe67c, `specs/0027-read-back.md` at 93db44a7 and
`ROADMAP.md` at 2b047990, the hashes planning sent (3 files, no
other change); the crate commit 69b5e0e (`CommentBox`, the
`DEFAULT_BOX` constant reproducing the shape written before,
`set_comment_box` and its body, `comment_boxes` recorded and
dropped with a removed part, the box joined to its addition at the
write with the last set winning, `note_shape` taking the box and
`points` printing three quarters of the pixels, `x:Visible` before
`x:Row`, `comments_vml` and the `shape` read through `style_box`
and `style_pixels`, the two messages catalogued in the same
commit, `boxes.rs` with 12 tests and 1 under `richtext`) and the
phase commit 2adf953 (`--commentbox-sample` with a 200 by 100
shown box, the read-back asserting the comment count, the shape
count, the box and its visibility, 2 tool tests) read in full by
planning; the record commit 9cec1de adds the six logs and the two
record pairs; 4 of 4 messages ASCII, ending with the
Co-Authored-By line only; the logs at the reported hashes, each
ending all as expected, the scan RESULT: PASS, every log pinning
8adfe67c at the phase commit and at the doc commit and printing
8adfe67c as planning's working copy (unedited through the phase);
the check answer read in full; the compound-shape and email greps
over the code diff 0; every live `rm -rf` in the phase's scripts
the guarded form, the run script carrying none; planning's own
build at 2adf953 in a detached worktree: 513 crate tests with the
features off, 557 with `comments`, 588 with `comments,richtext`,
591 with `comments,read` and 886 with every feature, on both
targets, 1075 workspace tests on both, clippy and doc 0 lines; the
i686 release `emend-copy` at the run log's hash 1efcb1b4 (the
host's differs by build path); planning's own run of
`--commentbox-sample` with its own binaries over all 27 corpus
workbooks on both labels, 54 outputs and 54 tool lines
byte-identical to the run directory's; planning's own privacy
scan 6aba171..9cec1de, 4 commits, 1330 blobs, 0 findings;
`ANTHROPIC_API_KEY` unset. P1 is DONE.

1. Item 1, the off-grid cell: `set_comment_box` refuses a cell off
   the grid at the call with part 3's "a row or column index or
   count is out of range", as `add_comment` does. Ruling: accepted;
   "Errors" reads with the index message at the call as well as
   the range message. No code change.
2. Item 2, the VML part the read takes: the sole internal
   `vmlDrawing` relationship, or with two or more the one the
   sheet's `legacyDrawing` names, `None` when that names no VML
   relationship; the tool's shape counts take the first VML
   relationship, which differs only with two VML relationships
   (none in the corpus). Ruling: accepted; P4's catalogue states
   the read's choice in one sentence; the tool's count stands.
3. Item 3, a size whose rounding falls outside 1 to 65535 reads
   `None`, as a shape carrying no size. Ruling: accepted; a box
   `set_comment_box` would refuse is not read back as one.
4. Item 4, any `x:Visible` child reads visible whatever its content.
   Ruling: accepted; the schema's `Visible` is an empty element and
   content, if any, is not a signal.
5. Check item 5, `px` parsed as a decimal and rounded where this
   file says "taken as is". Ruling: accepted; "as is" means no unit
   conversion, and a fractional `px` rounds as `pt` does (the
   corpus has no `px`). The doc comment's wording is the rule.
6. Check item 6, `comments()` now refuses a VML relationship whose
   target part is missing or does not resolve with the
   cannot-be-read message, where part 3's read opened the comments
   part alone. Ruling: accepted; "no VML part" above means no VML
   relationship, and a dangling one is what the write refuses too,
   so the read agrees with the write. Recorded here as the one
   behaviour change to an existing read in this phase.
7. Check item 7, `read_vml` marks visible on any shape with a
   `Visible` child, notes or not; only notes are looked up.
   Ruling: accepted, no change.
8. The `ClientData` content model: the implementing session read
   ISO/IEC 29500-4:2016's `vml-spreadsheetDrawing.xsd`, `CT_ClientData`
   a `choice` with `minOccurs 0` and `maxOccurs unbounded`, children in
   any order. Ruling: no sequence ruling is needed; `x:Visible`
   before `x:Row` stands as written.
9. The check's Q3 reservations: the per-commit `privacy.sh` runs
   are session runs with no log, and "passed midnight" is read
   from timestamps. Ruling: accepted; planning's own count-only
   scan over 6aba171..9cec1de is the record of the phase's privacy
   state (above), and the record pair's date is the run's UTC date
   as the naming rule says, whatever the commits' date.

## Planning rulings during P2 (2026-10-06)

Asked by the implementing session after the doc commit c46adcc
(this file at 5c3d3d1a, `ROADMAP.md` at 838de96f, verified by
planning on the blobs) and the crate commit 7976d9e, before the
tool commit and the scripts, on the unlogged development run of
`--remove-image-sample` over the 27 workbooks, every tool
assertion holding, and on six implementing choices.

1. F1, the recompressed entries. The sample writes twice, and
   the second write's input is the first write's output, so the
   entries the first write touched (W, W's rels part, the
   content types part, and W's drawing and its rels part where W
   had one) reach the output through emend's deflate of them,
   not the input's: `xlsx-diff` reports them `recompressed`
   (decompressed content equal) where "Harness runs" expects
   `identical`. The expectation was planning's error: the sample
   as written cannot return the input's compressed bytes for an
   entry the first write touched. Ruling: the proposal is taken.
   When `kept_removed` is 0, every entry is `identical` or
   `recompressed`, the `recompressed` ones a subset of those
   five names (the run log derives W and its drawing from the
   input and checks the set), with the drawing's rels part
   allowed `changed` with the same relationships as before;
   nothing `dropped` or `added` beyond rulings 2 and 3. Recorded
   as a finding about the rule, not the code; "Harness runs"
   reads as corrected by this ruling.
2. F2, the empty rels part left behind (7 rows: 6 where W had no
   rels part in the input, 1 where its drawing had none). The
   addition created the part for its one relationship and the
   removal leaves a `Relationships` root with no child. Ruling:
   option (a) for this part. The run log allows, per row, at
   most one `added` entry, W's rels part or W's drawing's, whose
   root has no child, where the input holds no part of that
   name. Option (b), dropping an edited rels part a write leaves
   with no relationship, is a change to the shared relationship
   path and goes to part 7's scoping as a candidate for the
   maintainer; the catalogue states in P4 that an emptied rels
   part is kept.
3. F3, the input's empty drawing (2 rows: W's drawing holds no
   root child and has no rels part). The addition gives it an
   anchor, the removal takes it, and "Image removal" drops a
   drawing left with no anchor at all, with its `Override`, W's
   relationship and `drawing` element, so the output differs
   from an input the sample cannot return to. Ruling: option (a),
   the rule's consequence. The run log allows it on exactly the
   rows whose input drawing has no root child (W, W's rels part
   and the content types part `changed` or `recompressed`, the
   drawing `dropped`), checking that the input's drawing was
   empty. Option (b) would not change these rows and is not
   taken. Stated in P4's catalogue in one sentence.
4. Choice 1, a picture in a root `AlternateContent`'s first
   `Choice`: the whole `AlternateContent` is skipped, its
   `Fallback` the same object, and the relationships the
   `Fallback` alone names go with it. Ruling: accepted; the root
   child holding the picture is the anchor element "Image
   removal" skips whole.
5. Choice 2, a picture in a group (`grpSp`): the whole anchor is
   skipped, the group's other shapes with it. Ruling: accepted
   as the rule reads; P4's catalogue states it as a limit (a
   picture removed from a group takes the group's anchor).
6. Choices 3 and 4, K computed once per write over every part's
   removed relationships, so two removals sharing a media entry
   release it, and a dropped drawing's rels part gone with the
   sheet's relationship to it left out of K. Ruling: accepted;
   both are what the walk from the package relationships with
   the removed relationships in `skipped` means.
7. Choices 5 and 6, `add_image_at`'s check order (the off-grid
   cell with the index message, the anchor range, a size on a
   two-cell anchor, then the image's own checks) and no check of
   the image's own size on a two-cell anchor since no extent is
   written. Ruling: accepted; the order is the catalogue's to
   state in P4.
8. Build pruning (the maintainer, 2026-10-06, after the host disk
   holding the WSL image ran out of space with 288 GB of finished
   phases' build targets under the implementing session's cache
   and 38 GB under planning's). Standing rule from this phase on,
   the one exception to the rule that a script removes only a
   directory it made in that run: each session's build scripts
   share one cargo target directory across a phase's dry and
   logged runs, and at a new phase's start remove the previous
   phase's build targets (the cargo target directories, the
   run outputs and the keep copies), listed with their sizes in
   the log before removal, by `rm -r` without `-f` or `cargo
   clean`. Never removed: `superseded/`, the logs, `gen/`,
   `scripts/`, `FACTS.md`, `logged.sh`, `logged.out` and the
   message files, that is, anything a committed log cites by
   hash. Planning's own script keeps one worktree and one target
   directory under its cache and removes the previous phase's at
   its start the same way. Goes into `specs/0001`'s workflow at
   part 7.
9. The run log's rerun (asked 2026-10-07 after attempt 2 passed
   and check 1 read its header). Attempt 1 was stopped by the
   host disk filling; attempt 2's rerun note gives the start time
   and the reason but not the last check attempt 1 completed, and
   its time is typed from the file's modification time, which no
   command in the log prints. `specs/0001` asks for all three.
   Ruling: option (A). Attempt 3 reruns the run log with the
   rerun note alone corrected: attempt 1's start time and the
   last check it completed quoted from the superseded log by
   command, attempt 2 named as superseded for the header gap with
   this ruling as the reason, both earlier logs in the rerun
   files. A fresh check 2 reads attempt 3's log (the second and
   last check), then the final scan. The gate, test and negative
   logs are not rerun. Attempt 2 is kept under `superseded/`.
10. The drawing count (check 1's R2). This file says 38 drawing
   parts; the run log prints 37 with root `wsDr`. Planning
   recounted (names and counts only): 38 parts named
   `xl/drawings/drawing<n>.xml`, of which 37 have the root `wsDr`
   and one the root `userShapes`, a chart's user shapes part that
   holds no anchor. Ruling: both figures stand, each by its own
   rule; "Facts" and "32-bit and streaming" read as "38 drawing
   parts by name, 37 worksheet drawings"; P4's catalogue states
   the limit with the 37.
11. The working hash across P2's logs (R3): the gate, test and
   negative logs print daef841a and the run and scan logs the
   later working copy, since rulings 8 and following were added
   between them. Ruling: expected; the pin is the committed
   5c3d3d1a in every log; no action.
12. The comment placement in 7976d9e (R4): the drawing drop block
   sits between the `legacyDrawing` comment and the code that
   comment describes. Ruling: no change in P2; P3's first code
   commit moves the comment to its code, a comment-only change,
   as the carry.

## Planning verification and rulings on P2 (2026-10-07)

Verified by planning from primaries: the doc commit c46adcc holds
this file at 5c3d3d1a and `ROADMAP.md` at 838de96f, the hashes
planning sent (2 files, no other change); the crate commit
7976d9e (`ImageAnchor`, `EditAs`, `add_image_at` with `add_image`
on it, `remove_image` and `image_removals`, `record_image_removals`
with `read_drawing_roots`, the relationship and media drops
through `reachable`, `drop_drawing` and the worksheet pass's
`drawing_drop`, `anchor_bytes` for the three forms, the three
messages catalogued in the same commit, 6 anchor tests and 11
removal tests) and the phase commit d3c4a9c (`--anchor-sample`
and `--remove-image-sample` with their read-backs, tool tests)
read in full by planning; the record commit d69e39c adds the five
logs, two check answers and three record pairs; 4 of 4 messages
ASCII, ending with the Co-Authored-By line only; the logs at the
reported hashes, each ending all as expected, the scan RESULT:
PASS, every log pinning 5c3d3d1a; the run log's attempt 3 header
quoting attempt 1's start and last completed check and hashing
both superseded attempts (ruling 9), its checks for rulings 1 to 3
each with a planted control, the survey's 37 `wsDr` drawings
(ruling 10); both check answers read in full; the two URLs in the
code diff the schema namespaces, no email shape; no live `rm -rf`
in the phase's scripts; planning's own build at d3c4a9c in one
shared worktree and target directory (ruling 8, the P1 output
directory pruned at start, 35M): 513 crate tests with the
features off, 549 with `images`, 594 with `images,comments`, 588
with `comments,richtext` and 902 with every feature, on both
targets, 1094 workspace tests on both, clippy and doc 0 lines; the
i686 release `emend-copy` at the run log's hash ea9b8968 (the
host's differs by build path); planning's own runs of
`--anchor-sample`, `--remove-image-sample` and
`--commentbox-sample` with its own binaries over all 27 corpus
workbooks on both labels, 162 outputs and 162 tool lines
byte-identical to the run directories' (`2026-10-07-d3c4a9c-b-*`);
planning's own privacy scan 9cec1de..d69e39c, 4 commits, 1344
blobs, 0 findings; `ANTHROPIC_API_KEY` unset.

1. Check 2's Q1 (b), the other `r:` references of a removed
   anchor: only `r:embed` and `r:link` are collected, as "Image
   removal" says, so a relationship a removed anchor names through
   another attribute, `hlinkClick`'s `r:id` for one, stays in the
   drawing's rels part with nothing naming it. Planning counted
   (names and counts only) 36 `hlinkClick` in 10 drawing parts of
   2 corpus workbooks, the check's figure sourced. Ruling: the
   behaviour stands for this part (an unreferenced relationship is
   harmless to a reader; a hyperlink's target is not a part);
   P4's catalogue states it in one sentence as a limit.
2. Check 2's Q1 (c), the tool's `dropped_media` taking the first
   root child holding a `pic` where the layer takes the root of
   `images()[0]`; the two differ only where `images` skips a
   picture `pic` matching finds (a grouped one). Ruling: accepted;
   the run log shows the tool and the comparison agreeing on all
   27 on both labels; the tool is a sample, not the rule.
3. Check 2's Q2 (a), what "by command" in ruling 9 means: the
   rerun note's start time and last check are built by the
   launcher's named commands and reach the log as its input, and
   the log hashes the superseded log they are read from, so a
   reader reproduces them. Ruling: ruling 9 is met; a typed value
   is one no command produced, and these were produced by the
   commands the note names.
4. Check 2's Q2 (b), the staged record copies named in the note
   but not hashed by the rerun files. Ruling: accepted; the
   installed copies are the ones the record commit carries and the
   log hashes them; the staged ones are the script's own.
5. Check 2's Q2 (c) and check 1's Q2 (d), ruling 8 leaving no
   trace in P2. Ruling: as read; the rule starts with P3's
   scripts, which planning verifies at P3 (the previous phase's
   build targets listed and removed, one target directory per
   phase).
6. Check 2's Q3, two figures of the report in no log (the
   pre-check scan's own hash, and the printed scripts' identity
   across attempts, which the check reproduced). Ruling: accepted;
   both are reproducible from the kept files.
7. The implementing session's disclosure: two directory names
   typed from memory while moving attempt 2 aside were wrong and
   the move stopped halfway; it was finished with the real names
   and checked by hash before attempt 3, whose rerun lines hash
   every moved file. Ruling: noted, nothing lost; the pattern (a
   path typed from memory) is in its gotchas.
8. The remaining choice of P2's logs: the gate, test and negative
   logs print daef841a as planning's working copy, the run and
   scan logs 1fca1264, as rulings 11 and 9 to 12 explain. The
   committed pin is 5c3d3d1a in all five. P2 is DONE.

## Planning rulings during P3 (2026-10-07)

Asked by the implementing session after the doc commit 2814a10
(this file at 38ab45c5, `ROADMAP.md` at a78ae477, verified by
planning on the blobs), the carry commit b68e584 (comment lines
alone, verified), the crate commit 4a8482c and the tool commit
6a90674, before the log scripts, on the unlogged development
run of `--copy-sample` matching this file's expected entries, and
on six implementing choices and one wording point.

1. W1, the copy's rels part. "Harness runs" expects it `added`
   "when carried" (comments); "The copy" creates it also when the
   source holds a hyperlink relationship (2 corpus rows with
   hyperlinks and no comments). Ruling: the proposal is taken; the
   run log expects the copy's rels part `added` exactly when the
   source has a hyperlink or a comments relationship; "Harness
   runs" reads as corrected.
2. C1, a root `mc:AlternateContent`: dropped whole when every
   element in its branches is one of the dropped root children (2
   corpus worksheets, each wrapping `controls` in a `Choice`);
   kept otherwise, its relationship attributes refused and its
   `f` counted; one with no branch element kept. Ruling: accepted,
   as P2 ruling 4 treats the wrapper as the element it wraps; P4's
   catalogue states it.
3. C2, a source this write replaces through `replace_part` is
   refused with "a sheet copy of a part this write edits"; a
   source `remove_sheet` removes in the same write is allowed, the
   copy staying. Ruling: accepted; a replacement is an edit of the
   part's bytes, and the removal case is this file's test.
4. C3, the copy's VML `idmap` block counted with this write's
   other allocations (`copy_idmaps`), so a comment added on another
   sheet in the same write takes a different block. Ruling:
   accepted; the allocation rule of `specs/0022` means every block
   a write takes.
5. C4, a `hyperlink` whose `r:id` names no relationship or one
   that is not a hyperlink is refused with "a sheet copy meets a
   relationship it cannot carry". Ruling: accepted; the pass
   carries hyperlink relationships and nothing else.
6. C5, the negative log fixture. S is chosen among worksheets, so
   the layer's "a sheet copy needs a worksheet" is unreachable
   through the tool. Ruling: the proposal is taken. The fixture's
   only sheet is a chartsheet; the tool refuses with its own
   message, exit 2, no output, with a control holding a worksheet;
   the four layer messages the tool cannot reach are stated
   unreachable with their covering crate tests, the comments one
   under a build without `comments`. "P3" reads as corrected.
7. C6, `COMMENTS`, `HYPERLINK` and the markup-compatibility
   constant compiled under `copy` alone, two more public constants
   under that feature. Ruling: accepted; P4's catalogue lists them
   with the feature's surface.
8. The crate commit's message amended before anything referenced
   it (the first wording cited a corpus count; the tree is
   unchanged). Ruling: noted; a count is not content, and the
   amend preceded every log and record, so nothing cites the old
   hash.
9. The development results reported (513, 523, 570 and 915 crate
   tests, 1109 workspace, 27 of 27 through the runner) are session
   runs; the logs decide. Proceed to the scripts: ruling 8's
   pruning first (P2's build targets listed with sizes and
   removed, the kept names untouched), one shared target directory
   across the dry and logged runs.
10. Ruling 8 under test (asked before any logged run). A shared
   cargo target poisons across source trees: the planted-defect
   builds compile from an archive copy with fresh mtimes, cargo's
   dep-info paths are relative to the package root, so the main
   tree's older sources then count as fresh against the mutant's
   artifacts and the main tree ran a mutant (one copy test failed
   5 of 5 by hand and in the gate's dry run; `cargo clean -p` of
   the three crates by hand repaired it, 570 and 1109 passing).
   Concurrent builds from two trees also overwrote each other.
   Ruling: ruling 8 means one target ROOT per phase, pruned at the
   next phase's start; under it each source tree builds into its
   own subdirectory (`CARGO_TARGET_DIR=<target>/main` for `<repo>`,
   `<target>/plant` for the planted copies, one more per further
   tree), the same profiles as before. Not the proposed per-tree
   cargo profile: a plant must be built as the main tree is, so a
   mutant's behaviour is compared like with like, and a profile is
   a build configuration, not a location. The test log checks the
   plant artifacts sit under their subdirectory and the main
   tree's tests pass after the plants; no two scripts build at
   once in a phase's dry or logged runs. The poisoned dry attempts
   stay as superseded attempts, and the report names the hand
   `cargo clean`. `specs/0001` carries this at part 7: one target
   root per phase, one subdirectory per source tree.
11. T1 to T3 (asked while the scripts regenerate under ruling
   10). T1: three plants (`codeName`, `tabSelected`, `pageSetup`'s
   `r:id`) fail at one line, the full copy test's single literal,
   the only test holding those three; the carried rule is a line
   of its own per plant. Ruling: option (a); the test log checks
   that exactly those three share one line and every other plant
   has its own, which is what the rule proves (each plant
   detected, no plant detected only by another's line). T2: the
   test log's crate feature sets are none, `comments`, `copy`,
   `copy,comments` and every feature, on both targets. Accepted.
   T3: the P3 report's "13 tests in `copy.rs` (10 without
   `comments`)" is corrected to 14 tests (9 plain, 4 under
   `comments`, 1 under not `comments`): 13 run with
   `copy,comments`, 10 with `copy` alone; the counts of ruling 9
   stand. Noted; the record says 14.

## Planning verification and rulings on P3 (2026-10-07)

Verified by planning from primaries: the doc commit 2814a10 holds
this file at 38ab45c5 and `ROADMAP.md` at a78ae477, the hashes
planning sent (2 files, no other change); the carry b68e584
changes three comment lines of `package.rs` and nothing else (P2
ruling 12); the crate commit 4a8482c (the feature `copy` with no
dependency, `copy_sheet`, `record_copies`, `carry_comments`,
`copy_idmaps`, the pass `copy_worksheet` with `COPY_DROPPED` and
`check_copy_relationships`, the four messages catalogued in the
same commit, `copy.rs` with 14 tests) and the phase commit
6a90674 (`--copy-sample` with its read-back, 2 tool tests) read
by planning; the record commit f73a91b adds the five logs, one
check answer and two record pairs; 4 of 4 messages ASCII, ending
with the Co-Authored-By line only; the logs at the reported hashes
(gate 757c51d7, test 20c59007, negative 5901a2cb, run cc788bb8,
scan 90257a58), each ending all as expected, the scan RESULT:
PASS, every log pinning 38ab45c5 and printing the working copy
5d4e611b; every header RERUN_NOTE unset (first attempts, no
rerun); the gate's prune under ruling 8 (P2's 12 build names
listed with their bytes, 25428651432 in all, removed with `rm -r`
and no `-f`, the kept names checked present, two controls) and
the P2 cache left with its logs, scripts, gen, FACTS.md, check
directories, message files, logged.sh and superseded; ruling 10
in the test log (the plants into `<target>/plant`, the main tree
into `<target>/main`, the two apart, 13 of 13 copy tests and 2
sample tests passing after the plants) and the negative log (its
one build into `<target>/negative`); ruling 11 T1 to T3 in the
test log (11 plants, 9 distinct lines, L1 to L3 sharing one;
513/523/557/570 by feature set and 915 with every feature, on
both targets; 1109 workspace tests on both); the negative log's
chartsheet-only refusal with its control; the run log's copy
sample (27 of 27 ok on both labels; identical 784, recompressed
0, changed 81, dropped 5, added 47, flagged 51) and the three
reruns byte-identical to P2's `2026-10-07-d3c4a9c-b-*` by sha256
on both labels, the keep 1092 pairs with P2's 1074 unchanged,
the open totals 1092 pairs, `lo_in` 1080 ok 12 fail, `lo` 1081
ok 11 fail; the check answer read in full; the one email shape
in the logs beside the Co-Authored-By address the privacy
self-test's `example.invalid` fixture, present since P1; no live
`rm -rf` in the phase's scripts; planning's own build at 6a90674
in the shared worktree and target (the P2 output directory
pruned at start, 105M): 513 crate tests with the features off,
523 with `copy`, 570 with `copy,comments`, 588 with
`comments,richtext` and 915 with every feature, on both targets,
1109 workspace tests on both, clippy and doc 0 lines; the i686
release `emend-copy` at the run log's hash eb566f3f (the host's
differs by build path); planning's own runs of `--copy-sample`,
`--anchor-sample`, `--remove-image-sample` and
`--commentbox-sample` with its own binaries over all 27 corpus
workbooks on both labels, 216 outputs and 216 tool lines
byte-identical to the run directories' (`2026-10-07-6a90674-*`);
planning's own privacy scan d69e39c..f73a91b, 5 commits, 1357
blobs, 0 findings; `origin/main` still 9d62635 with 14 local
commits above it (nothing pushed); `ANTHROPIC_API_KEY` unset.

1. Check 1's Q1 (a), the start tags of `sheetPr` and `sheetView`
   taking the attribute-stripping branch and never reaching
   `check_copy_relationships`. Planning read the code: so they do.
   The schema gives neither element a relationships attribute, so
   a schema-valid source cannot reach the gap. Ruling: stands for
   this part; "the pass checks every start tag kept" reads as
   every kept start tag that the schema lets carry one. Part 7's
   backlog: route the two stripped tags through the check as well
   (a tidy, no behaviour change on valid input), with one test on
   a byte-built part.
2. Check 1's Q1 (b), `pageSetup` losing every relationships
   attribute where "The copy" names `r:id`. The schema gives
   `pageSetup` `r:id` alone, so the two agree on valid input.
   Ruling: accepted as the code says it; P4's catalogue writes
   "the relationship attributes of `pageSetup`", and part 7 amends
   the sentence in "The copy".
3. Check 1's Q1 (c), the tool's `formulas` from `Package::cells`
   where the layer scans for an `f` element; the run log shows
   them agreeing on all 27. Ruling: accepted; the tool is a
   sample, not the rule (as P2 verification ruling 2).
4. Check 1's Q2 (a) and (b): the final scan log was not among its
   inputs because the scan is run last (`specs/0001`), and
   `specs/0027` was not given. Ruling: as designed; planning read
   the final scan (RESULT: PASS) and the gate's carried-rule lines
   itself. Q2 (c) is the report's error the record's header
   admits; the report to planning named the right files.
5. Check 1's Q3 items 1 to 4 (the amended hash, the failing test's
   name, the hand `cargo clean`'s crates and profiles, the four
   dry gate attempts under `<phase>/dry-gate`): the session's own
   history, disclosed as such and in no log. Ruling: accepted as
   disclosure, not as logged facts; the superseded dry attempts
   stay where the report says until P4's prune. Item 6, "nothing
   pushed", planning verified itself (above). Item 7 is met by
   every header's RERUN_NOTE unset.
6. The phase's target root holds, beside the subdirectories
   `main`, `plant`, `negative` and `consumer`, root-level `debug`,
   `release`, `i686-unknown-linux-musl`, `doc`, `tmp` and
   `CACHEDIR.TAG` whose times precede the logged runs: the
   pre-ruling-10 dry attempts' artifacts, which the hand clean
   emptied of the three crates. Ruling: harmless to the logged
   runs (each built in its own subdirectory, the SERIALIZED lines
   say so); P4's gate lists and removes the whole P3 root with the
   rest of P3's build targets under ruling 8, naming them.
7. The copy sample's entry totals read as the design predicts:
   changed 81 is the content types, workbook rels and workbook of
   27 workbooks; added 47 is 27 worksheets, 8 rels parts (the
   sources with a hyperlink or comments relationship, ruling 1),
   6 comments parts and 6 VML parts; dropped 5 is the calcChain
   of the 5 formula-bearing sources among the 9 workbooks holding
   one; flagged 51 is the known data-descriptor pattern on 17
   workbooks. P3 is DONE.

## Planning rulings during P4 (2026-10-07)

Asked by the implementing session while it writes the catalogue,
after the doc commit f54041f (this file at 81069c65, `ROADMAP.md`
at a416f53b, verified by planning on the blobs). P4's logs pin
81069c65 and print the working hash.

1. The `grpSp` limit. Planning's P4 brief said "a grouped picture
   is not in `images`"; the code and its test say otherwise:
   `read_images` takes a `pic` held directly, in a `grpSp` or in
   an `AlternateContent`'s first `Choice`, the first found, and
   `images.rs` reads a picture in a group as an entry. Ruling: the
   brief's sentence is withdrawn; the catalogue states P2 ruling
   5's limit exactly (a picture removed from a group takes the
   group's anchor, the group's other shapes with it) and nothing
   about grouped pictures being absent. P2 verification ruling 2's
   parenthetical "(a grouped one)" reads as "one `read_images`
   does not take, a `pic` beyond the first found or outside those
   three placements".
2. The constants under `copy`. In the code `COMMENTS` is public
   under `comments` or `copy`, `HYPERLINK` public under `metadata`
   or `copy`, and `MARKUP_COMPATIBILITY` private, compiled under
   `images` or `copy`. Ruling: as P3 ruling 7 reads ("two more
   public constants"); the catalogue names the two public ones
   with the feature's surface and says the markup-compatibility
   namespace constant is private.
3. The two new probes' copy. "Size" has the copy probe copy the
   first sheet after the format probe's work and the pair probe
   copy the comment's sheet after the comments probe's work; in
   the same write both are refused by the design's own rules (the
   first sheet is one that write added, "no such part"; the
   comment's sheet is one that write edits, "a sheet copy of a
   part this write edits"). Ruling: the images probe's pattern,
   already in "Size", applies to both: a second `Package` opened
   on the probe's second output, `copy_sheet` of that output's
   first sheet (by `sheets()[0]`'s name) under the fixed name
   `probe copy` for the copy probe, and of the sheet the comment
   went on for the pair probe; written into memory, reopened, the
   sheet count printed as `sheets_copied` and, for the pair, the
   copy's comment count through `comments` as `copy_comments`; no
   new command-line argument. The probe measures linked code, and
   the second package links the same code the one-write wording
   meant. The images probe's removal is written into memory and
   reopened the same way, `images_removed` printed; the comments
   probe's box is `set_comment_box` on AB10 (200 by 100, visible)
   in the addition's write, read back as `box_width`,
   `box_height` and `box_visible`. Part 7 amends "Size" to read
   this way.
4. Finding: the copy refuses a source whose kept start tags carry
   an attribute of the reserved `xml` prefix, since `Scope` knows
   only declared bindings and `check_copy_relationships` resolves
   every attribute; found by the pair probe on its fictional size
   fixture (`<t xml:space="preserve">` written by the format
   probe's `set_cell` of a `Text`) and reproduced by the tool;
   the corpus never reaches the path (no kept start tag of the 27
   sources carries `xml:*`), so P3's runs and verification stand.
   Planning read `xml.rs`: `lookup` and `bound` have no case for
   `xml`, which Namespaces in XML 1.0 section 3 binds by
   definition. Ruling: option A, the core fix. `Scope::lookup`
   answers `xml` with `http://www.w3.org/XML/1998/namespace` when
   no declaration binds it (a declaration of it is recorded as
   any other; emend does not validate what it binds). Every
   resolver that walks attributes gains the same reading, which
   is the standard's. Tests: one `Scope` test (an `xml:space`
   attribute resolves without a declaration) and one copy test on
   a byte-built package (an inline string's `<t
   xml:space="preserve">` copied byte for byte). The fix is a P4
   code commit of its own ("the xml prefix", the first code commit
   of P4) before the catalogue and probes commit; no new message,
   so the catalogue's list is unchanged, and the module header's
   sentence on the unbound-prefix message gains the `xml`
   exception. The size log prints the core's growth as "Size"
   says (the constant and the comparison, expected under 300
   section bytes; above 2000 planning is told). The four samples
   rerun byte-identical to P3's. The report names the finding, the
   fixture (fictional, generated by the probe) and the fix's
   commit, as `specs/0001` asks of a failing run fixed and rerun.
5. The negative log's shape, asked after the two code commits
   (42b5de8 "the xml prefix", ruling 4; 9cfd92e the catalogue and
   the probes, the phase commit; both read by planning: the
   `lookup` fallback to `XML_NAMESPACE` for `xml` alone, the two
   tests `the_xml_prefix_resolves_without_a_declaration` and
   `a_copy_keeps_an_attribute_in_the_xml_namespace`, the header
   sentence; privacy 0 on both). None of the part's nine messages
   is reachable through `emend-copy`, whose samples take
   constants. Ruling: as part 5 P3's negative log: (1) the part's
   three negative families rerun through the gate's host binary
   with their controls (P1's VML part with no root end tag under
   `--commentbox-sample`, P2's drawing rels part naming a missing
   media entry under `--remove-image-sample`, P3's chartsheet-only
   workbook under `--copy-sample`); (2) the nine messages read
   from the phase commit's source by constant name, each printed
   as unreachable through the tool with its covering crate test
   checked present once at the phase commit; (3) a check that the
   phase adds no message constant and removes none. Accepted.
6. The logged order, asked by the successor session (emend-01
   died with the user manager at 14:50:41 PDT, no logged run
   started; its complete dry runs stand, the stopped dry run of
   the samples kept superseded and relaunched). Planning's brief
   said test, negative, run, size; "P4" has the four samples run
   with the size log's release binaries, so the run script takes
   `size.log`. Ruling: gate (with the prune), test, negative,
   size, run, then the scan pre-check, the check, the scan last.
7. The dry size figures reported ahead of the log (the logged
   `size.log` decides): budget 436992 against 458752, the core's
   growth 64 (the `xml` prefix, ruling 4), `images` grown 27016,
   `comments` 6196, `copy` 13756, the pair 18864 over `comments`.
   `images` is above the 25000 the "Size" section names for the
   report and above planning's 12000 to 19000; the part's total,
   near 47000 over the three features, is above planning's 30000
   to 45000. Ruling: no action in P4; the report states the
   `images` growth with the log's cause for the probe; part 7's
   backlog (finer cause attribution per probe) takes the question
   of where the removal walk's bytes went.
8. The dry run's one failure (2026-10-08, the third attempt after
   two stopped by the environment): podman warns on its first
   call that `/` is not a shared mount, 149 bytes on the script's
   standard error, so "wrote nothing to its standard error" reads
   no. Planning checked: `/` is private in the one mount namespace
   every shell, the keepalive and the panes share (the earlier
   passing attempt of 2026-10-07 14:03 saw it shared; what made
   it private between then and 18:30 is not known). Ruling:
   option (i), the environment is restored, not the script: the
   maintainer runs `sudo mount --make-rshared /` in any shell of
   the session; the implementing session verifies `findmnt -o
   PROPAGATION /` says shared before launching, and the logged
   run's stderr check stands as written. No script filters a
   warning. If the maintainer declines, option (ii) with planning
   asked again.
9. The dry scan's two placeholder defects: `size.log`'s root line
   for `<build>` lacks "stands for" (line 1240 of the size
   script), and the run script's printed comment names `<i686>`
   with no root line (line 1038). Ruling: both scripts are fixed
   (the size root line gains "stands for"; the comment is
   reworded without the placeholder); the new hashes go in
   FACTS.md with the old ones and the diffs' line counts. The
   size script is dry-run again in full. The run script's change
   is one comment line: the diff between the two scripts is
   printed in FACTS.md and in the report, and the 1.5 hour dry
   run is not repeated for it; the dry scan runs over the third
   attempt's `run.log` and the new `size.log`, with (a) (the
   default directories, every dry scan's) and (c) (the one
   comment line, fixed by the diff) the only failures expected.
   The logged scan over the logged `run.log` is the proof.

10. Check 1's answer carries one machine path (2026-10-08): its
   first line names the inputs directory by its absolute path, the
   only such path in the answer (P3's had none). Ruling: the answer
   is recorded as `records/logs/m3p6-p4/independent-check-1.md`
   with that one path written `<phase>/check1/inputs`, the same
   substitution the logs make. The record's header states the one
   substitution and gives both sha256s: the raw extraction's (kept
   at `<phase>/check1-answer.raw`, not committed) and the recorded
   text's. Nothing else in the answer changes; the report quotes
   the recorded text. The raw file is listed in the report with its
   hash so the substitution can be checked from the cache.

## Planning verification and rulings on P4 (2026-10-08)

Verified by planning from primaries: the doc commit f54041f holds
this file at 81069c65 and `ROADMAP.md` at a416f53b, the hashes
planning sent (2 files, no other change); code commit 1, 42b5de8
(ruling 4: `XML_NAMESPACE` and the `xml` fallback in
`Scope::lookup`, which `bound` wraps, with the Scope test and the
copy test, the `package.rs` header sentence; no message constant
added or removed, 123 at the phase commit and at the base) and
the phase commit 9cfd92e (the catalogue and limits in
`package.rs`, the two grown and two new probes, the `copy`
feature of `tools/size-probe`) read by planning; the catalogue's
"9 anchors" is `records/logs/m3p6-p2/run.log` line 3322 and the
`grpSp` placement is in the read-back sentences at `package.rs`
lines 1848, 7069 and 20102 (check 1's Q1 items 1 to 3 answered
from the code); the record commit c616900 adds the six logs, one
check answer, the size record, four sample pairs and the open
pair (18 files); 4 of 4 messages ASCII, ending with the
Co-Authored-By line only; the logs at the reported hashes (gate
934c7a6d, test 854a6e2a, negative 13560ade, size f5e36109, run
a7044bf2, scan f8faeaac), each ending all as expected, the scan
RESULT: PASS with every placeholder declared and PLANNING_SPEC
the working copy 2a53400d; every log pinning 81069c65 and
printing the working copy 62762f0a (the scan 2a53400d, ruling
10 having been added between); every header RERUN_NOTE unset;
the logged order gate, test, negative, size, run (ruling 6) by
end time 15:27:58Z to 17:09:24Z, the scan ending 17:16:35Z; the
gate's prune under P3 verification ruling 6 (P3's 11 names
listed with their bytes, 9624749323 in all, removed with `rm -r`
and no `-f`, the 9 kept names present); 514/525/572 by feature
set and 917 with every feature on both targets, 1111 workspace
tests on both (1109 at P3, the two added tests named); the
negative log in P3's shape (three families through the gate's
host binary with controls, nine messages unreachable, each
covering test counted once); the size record's figures as the
log prints them: budget 436992, 21760 under 458752; core +64,
not above 300, six commits named as its cause and the 64 not
isolated; `images` +27016 (1011024 to 1038040) over the probe at
6b6125b, ABOVE 25000, cause "verdict: code" over six commits;
`comments` +6196; `copy` 13756 over the format probe; the pair
18864 over the comments probe; the sum 46968 beside the
30000 to 45000 estimate; the run log's four samples 27 of 27 ok
on both labels, each equal to P3's `2026-10-07-6a90674-*` by
sha256, by tool line and by diff table (the copy sample's entry
totals identical 784, recompressed 0, changed 81, dropped 5,
added 47, flagged 51, P3's); the four record pairs' tables equal
to their predecessors' (copy P3's, anchor and removeimage P2's,
commentbox P1's) apart from dates and hashes; the keep 1093 pairs
with P3's 1092 unchanged and the one new pair
`a_copy_keeps_an_attribute_in_the_xml_namespace.1` ok on both
opens; the check answer read in full and its recorded text's
hash 83cfd478 reproduced by planning from the raw extraction
f4404cd7 with ruling 10's one substitution; the report's every
figure found in a log or marked (session); the one email shape
in the commit the privacy self-test's `example.invalid` fixture;
the six bare `rm -rf "$d"` in the logs the committed
`scripts/counting.sh` control on its own `mktemp -d` directory,
present since P1 and on part 7's list; planning's own build at
9cfd92e in the shared worktree and target (the P3 output
directory pruned at start, 170M): 514/525/572/589/917 on both
targets, 1111 workspace tests on both, clippy and doc 0 lines;
the i686 release `emend-copy` at the run log's hash c2b87d65;
planning's own runs of the four samples with its own binaries
over all 27 corpus workbooks on both labels, 216 outputs and 216
tool lines byte-identical to the run directories'
(`2026-10-08-9cfd92e-*`); planning's own privacy scan
9d62635..HEAD, 18 commits, 1404 blobs, 0 findings; `origin/main`
still 9d62635 with 18 local commits above it (nothing pushed);
`ANTHROPIC_API_KEY` unset.

Rulings:

1. Check 1's Q2 item 1, ruling 8's `findmnt` check before the
   launch in no log: accepted for P4 on the logged run's own
   evidence (every stderr 0 bytes, the warning gone). Ruling 8 is
   read as satisfied by the maintainer's `wsl -u root mount
   --make-rshared /` from Windows, the same mount change by the
   other root the host offers; the report records the command as
   run. Part 7 adds to `specs/0001` an environment line the gate
   log prints before anything else: the root's propagation and
   the keepalive's presence, so the next phase's log holds them.
2. Check 1's Q3 item 12 and ruling 7's attribution: planning's
   ruling 7 attributed the core's 64 bytes to 42b5de8; the size
   log names six commits and does not isolate it, and the report
   now says so. The attribution is withdrawn as unproven, with
   the growth accepted at 64 (not above 300). Part 7's finer
   cause attribution (P3 verification, ruling 7's note) covers
   the core as well as `images`.
3. The `images` growth 27016 above 25000: stated in the report
   with the log's cause, as ruling 7 orders; accepted for the
   part, the feature being off by default and the budget probe
   unaffected (21760 under). The three features' sum 46968 is
   above planning's 45000 estimate, which was printed and not
   checked; the estimate was wrong by the `images` growth and
   nothing is owed to it.
4. The three disclosures (two dry runs stopped by the
   environment under `superseded/`, ruling 9's generator fix with
   its one-line diff, the dry run's `run.stderr` overwritten by
   the slip and noted): accepted; none touches a logged file, and
   the logged run was a first attempt throughout.
5. The check 1 record under ruling 10: the header's two hashes
   both reproduced; the form stands for later phases when an
   answer carries a machine path (the substitution, the raw kept
   in the phase cache, both hashes in the header).
6. The phase's build targets (`<phase>/target` and the rest of
   the prune set) are pruned under the standing rule at the next
   gate that builds, part 7 P1's (the close-out's push script
   runs the gate in the repository's own tree and builds no
   target of its own, as part 5's), listed with sizes and
   removed with `rm -r`.
7. P4 is DONE. The close-out follows (the doc commit carrying
   this file and `ROADMAP.md` unedited at command-substituted
   hashes; the push on planning's word and the maintainer's
   authorisation typed in the implementing session; decision 9
   asked again before it).

## Planning verification and rulings on the close-out (2026-10-08)

Verified by planning from primaries: `origin/main` at the
close-out doc commit 046cfb8, 19 commits over 9d62635 (6aba171,
this part's seventeen and the doc commit), the doc commit holding
this file at 377049e9 and `ROADMAP.md` at 1f82e47b, the hashes
planning sent; the CI run 37853950682 completed with success on
046cfb8, read by planning through `gh`; the record commit
bafd780, parent 046cfb8, exactly the three logs, the one local
commit ahead, its message naming decision 9's twelfth deferral
and the superseded first pre attempt; the logs at the reported
hashes (push.log bca1c175, ci-run.log 05d1ebe2, scan.log
a23b72c5), each equal to the implementing session's cache copy,
each ending all as expected, the scan RESULT: PASS with the
`ANTHROPIC_API_KEY` check and the two siblings scanned at the
committed hashes; push.log's first 969 lines equal to the pre
rerun's log (1d66f012, ruling 1), its push mode showing the
authorisation typed by the maintainer in the implementing
session ("push", 10/8/26) and git's own `9d62635..046cfb8`; 20
of 20 messages (the pushed 19 and the record) ASCII, ending with
the Co-Authored-By line only; no U+2014 in the three logs; the
repository PRIVATE; planning's own privacy scan 9d62635..bafd780,
20 commits, 1409 blobs, 0 findings; `ANTHROPIC_API_KEY` unset.

1. The pre run's location (check 1's Q1 reservation, the
   implementing session's item 4): the push script and its
   library copy ran from `<home>/.cache/emend-build/m3p6-close/`
   while the carried text (part 5's change 6) says the scratch
   copy is the one that runs, and the log's root line and line
   965 name `<scratch>/close`. Ruling: option (b). Pre mode is run
   again from a copy under the session's scratch directory,
   `<scratch>/close`, holding the script and the library copy, so
   that the root line is true and the stderr counted is the one
   the line names; push mode runs from the same copy, appended to
   the same log. The first pre attempt's log and stderr are kept
   under `<home>/.cache/emend-build/m3p6-close/superseded/
   pre-attempt-1/` with a NOTE.txt naming this ruling and both
   hashes; the report discloses it. The script text does not
   change (its hash stays 400f6015); FACTS.md records the scratch
   copy's hash equal to it. The independent check is not rerun:
   the deviation it found is the one corrected, and the rerun's
   log is read by planning. (Written during the close-out in
   planning's cache file, the tree having to stay clean for the
   push script; copied here after the record commit, as part 5's
   close-out practice.)
2. The independent check's other reservations (the doc commit's
   own files and `ROADMAP.md`'s hash not isolable from the
   cumulative diff; the ci-run and scan scripts' hashes, diff
   line counts and patterns not among its inputs; the carried
   library's equality to `scripts/counting.sh`): working-state
   facts planning verified itself from the repository; accepted,
   no change, as the earlier close-outs. The authorisation was
   typed in the implementing session, where the maintainer
   answers that script.
3. Milestone 3 part 6 is DONE and PUSHED. The record commit
   bafd780 goes with the next push; part 7's push script names
   it by `^M3 part 6 close-out record`, its base this close-out's
   doc commit 046cfb8 by its subject ("M3 part 6 close-out:
   commit planning docs (the P4 verification and rulings,
   decision 9 deferred again)").
4. Carried into part 7 (`specs/0029`), before anything else its
   `specs/0001` gate environment line (P4 verification ruling 1);
   then the backlog named under P3's and P4's verifications (the
   per-commit cause attribution for the core and `images`, the
   `sheetPr` and `sheetView` route through the copy's
   relationship check with a test, "The copy" pageSetup sentence
   and the "Size" probe wording, the committed scripts' `rm -rf`
   forms, the prune-rule text and the keepalive and mount notes
   in `specs/0001`, F2 option (b)). P4's build targets are pruned
   at part 7 P1's gate (P4 verification ruling 6). The Milestone
   2 Excel check, with this part's four samples on its list, is a
   hard requirement of part 7's close-out push (decision 9).
5. These rulings are written into the working spec after the
   record commit; they go in part 7's first doc commit.

## Decisions for the maintainer

Decided by the maintainer 2026-10-05: 1 to 8 as recommended (A in
each), in the reply "A on all (Recommended)" to planning's
question, typed in the planning session; 9 answered "Defer again
(11th)" to the same question.

1. Packaging: A (recommended), `images` and `comments` grow in
   place and `copy_sheet` goes behind a new feature `copy` with
   one probe and a pair probe with `comments`; B, the copy in the
   core; C, the copy under `comments`.
2. What names an image for removal: A (recommended), its index in
   the list `images` gives, openpyxl's `ws._images[i]`, stable at
   the write because the write reads the same list; B, the
   picture's `cNvPr` `id`, which the read would have to return and
   which Excel keeps unique per drawing, not per workbook.
3. The anchor API: A (recommended), a new `ImageAnchor` and
   `add_image_at`, `add_image` unchanged as the one-cell form; B,
   `add_image` takes an `ImageAnchor` and every caller changes.
4. The box's unit: A (recommended), pixels as openpyxl's
   `Comment.width` and `height`, written as points at three
   quarters (exact to two decimals) and read back at four thirds
   rounded, so this part's writes round-trip exactly and a
   corpus `pt` value rounds to the nearest pixel; B, points in
   hundredths, exact for every corpus value and matching nothing
   a caller holds.
5. The copy's name: A (recommended), the caller gives it, checked
   by the sheet name rules; B, `<name> Copy` as openpyxl, with a
   31-character limit the suffix may break on a long name.
6. A source with comments when `comments` is off: A
   (recommended), the copy is refused with its own message, so
   nothing is dropped silently; B, the comments are left out and
   the limits say so.
7. Edits on the copy in the same write: A (recommended), refused
   with "no such part" as a comment on an added sheet is, the
   caller writes and reopens (one pass over the source); B,
   applied, which means the copy's bytes pass through the cell
   pass a second time in the same write.
8. The scale record's carry for this part (the whole VML and
   drawing reads): A (recommended), the holds stated in the
   catalogue with the corpus figures and the reason (a drawing is
   bounded by its anchors, not the sheet's cells; the tiers hold
   neither part), no streaming form; B, a streaming rewrite of
   both readers, 10000 to 15000 more on the two features for a
   bound no corpus or tier file approaches.
9. The Milestone 2 Excel check, deferred ten times, last at part
   5's close-out: run before this part's close-out push (the four
   samples of this part on its list), or deferred again by a
   decision recorded here. Planning's standing recommendation was
   to run it. Deferred an eleventh time by the maintainer
   2026-10-05 at this part's scoping; due before part 7's
   close-out push at the latest; planning asks again at this
   part's close-out. Asked the twelfth time at this part's
   close-out on 2026-10-08 after P4's verification and deferred
   again by the maintainer (the twelfth deferral): the check is
   run before part 7's close-out push, with this part's four
   samples on its list; `specs/0029` carries it as a hard
   requirement of that push.

## Out of scope

- An image's rotation, crop, effects, `descr`, hyperlink
  (`hlinkClick`), SVG, EMF, GIF, BMP, TIFF; moving or resizing a
  kept image (remove and add).
- A comment's position (`margin-left`, `margin-top`, `x:Anchor`:
  the writer's placement beside the cell stays), `AutoFill`,
  the box of a kept comment without replacing it.
- The copy of images, charts, tables, pivot tables, controls,
  printer settings, the extension list (`x14` conditional formats
  and validations, sparklines, slicer and timeline references),
  defined names with the source's `localSheetId`, the source's
  `state`; a copy between workbooks; a chartsheet copy.
- Threaded comments: as `specs/0022`, read as their fallback,
  never written, a sheet that has them refuses comment edits and
  the copy.
- What Excel or LibreOffice shows for a box, an anchor or a
  copied sheet: the file opens; the Excel check is the
  maintainer's.
