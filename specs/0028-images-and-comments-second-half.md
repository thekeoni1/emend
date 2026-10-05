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
   part's close-out.

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
