# API review rerun, Milestone 6 part 4 P3 (2026-10-22, 1cc0de4)

The API listing rerun as Milestone 6 part 3 P3 did, after Milestone 6 part 4 added text properties on a written chart's title, axis titles, axis labels, data labels and legend, a chart's style value, and the in-place edits of a kept chart's title and series, under the feature `charts` (`specs/0041-chart-text-and-edits.md`, "P3"). Every change against part 3's listing is named: twenty rows new and one gone, below, compared by key without the added-by column (ruling L). The gone row and one of the new are the `charts` re-export of `chart_write` in `crate::package`, its name list grown by `NewText`, so its key changed; `ChartSeriesEdit` is re-exported from the new module `chart_edit` by a row of its own. The listing's program is this part's, part 7 P3's with change AV1 (`specs/0040` ruling V; ruling L): a member's added-by is the oldest commit of its file whose copy holds the member in its type; the rows whose added-by it gives otherwise than part 7 P3's program are listed below under their own heading. No row keeps its key and loses `Eq`; the part adds one message, `no series at that position` (`NO_SERIES_AT`), crate-private and in no row, and no `NewPlot` variant (test.log section T1 checks each). M6 part 3 P3's record is `records/api-review-m6p3-2026-10-21-0707236.md`.

- Listing: every `pub` item of `crates/emend` reachable from the crate root, as `records/logs/m6p4-p3/test.log` (section T1) prints it from the phase commit's sources with this part's listing program, sha256 bb1fba224a763acac9049d18798bb774820898b2018ad68899c5332076a8f38b (part 7 P3's, sha256 59f4043c3014a64e671054fcfd4dce565e210426d4d4847ae448cb29b48b987c, with change AV1); its file `listing.tsv` sha256 98401415c161344579ce277e74e7e6c0825c109f55a875bec73e5ef8022bdc60, `files.tsv` sha256 6420b3d0dafe0aec6ad2dc8e362a054b48ce32ae55861b0fb00f49f177ac83af.
- Formatter: this record is assembled by `make_review.py`, generated from M6 part 3 P3's formatter, sha256 f960fb37f10046a7b38c7b7c48604b39d6883fb3fc92a828ce4f7733846d2679 (printed in test.log, section R2), its table of non-keep verdicts empty and its spec map extended to M6 part 4.
- Items: 834, by kind: field 450, method 174, struct 89, enum 58, const 30, use 22, mod 6, fn 4, type 1.
- The doc build (test.log section T2): `cargo doc` with every feature and `-D warnings`, exit 0, no warning line; its index `all.html` holds 187 link lines, of which 5 are the page's section anchors, leaving 182 documented module-level items, against the listing's 182 reachable module-level types, functions, constants and aliases.
- Columns: as part 7's record: `item` is the item's name, `Container.name` for a method or field, `use ...` for a re-export; `feature` is the cfg in force at the item (`core` for none); `added by` is the commit the listing finds introducing the item's declaration: for a member (a field, method or associated item of a type) the oldest commit whose copy of its file holds it in its type (change AV1), for any other row the oldest commit changing the count of its declaration text in its file (a heuristic the listing program states); `spec` is that commit's part's spec.
- Verdicts: every item keep; no row lost Eq.

## Rows that differ from M6 part 3 P3's listing

Compared by key (module, kind, name, container, cfg; ruling L, the added-by column no part of it): 1 of M6 part 3 P3's rows are not in this listing and 20 of this listing's rows are not in M6 part 3 P3's; every other row is equal by key.

M6 part 3 P3's rows not here (added by as M6 part 3 P3's listing gives it):

- `use chart_write::{ DashStyle, ErrorBarKind, ErrorDirection, ErrorValue, LabelPosition, MarkerSymbol, NewAxis, NewChart, NewCombined, NewDateAxis, NewErrorBars, NewLabels, NewLine, NewMarker, NewPlot, NewPoint, NewSeries, NewSeriesName, NewTrendline, OfPieKind, OfPieSplit, RadarStyle, TimeUnit, TrendlineKind, }` use in `crate::package`, feature charts, added by 4c36f05

This listing's rows not in M6 part 3 P3's (20 listed: 0 cfg-widened items, 20 new by key):

The 0 cfg-widened items are M6 part 3 P3's rows not here with their cfg widened; the other 20 rows are new by key, the items M6 part 4 adds and any row whose key changed (the rows not here above).

- `use chart_edit::ChartSeriesEdit` use in `crate::package`, feature charts, added by 075abca
- `use chart_write::{ DashStyle, ErrorBarKind, ErrorDirection, ErrorValue, LabelPosition, MarkerSymbol, NewAxis, NewChart, NewCombined, NewDateAxis, NewErrorBars, NewLabels, NewLine, NewMarker, NewPlot, NewPoint, NewSeries, NewSeriesName, NewText, NewTrendline, OfPieKind, OfPieSplit, RadarStyle, TimeUnit, TrendlineKind, }` use in `crate::package`, feature charts, added by 4c36f05
- `ChartSeriesEdit.categories` field in `crate::package::chart_edit`, feature charts, added by 075abca
- `ChartSeriesEdit.name` field in `crate::package::chart_edit`, feature charts, added by 075abca
- `ChartSeriesEdit.values` field in `crate::package::chart_edit`, feature charts, added by 075abca
- `Package.set_chart_series` method in `crate::package::chart_edit`, feature charts, added by 075abca
- `Package.set_chart_title` method in `crate::package::chart_edit`, feature charts, added by 075abca
- `ChartSeriesEdit` struct in `crate::package::chart_edit`, feature charts, added by 075abca
- `NewChart.axis_label_text` field in `crate::package::chart_write`, feature charts, added by 4d2cdc3
- `NewChart.axis_title_text` field in `crate::package::chart_write`, feature charts, added by 4d2cdc3
- `NewText.bold` field in `crate::package::chart_write`, feature charts, added by 4d2cdc3
- `NewText.color` field in `crate::package::chart_write`, feature charts, added by 4d2cdc3
- `NewText.font` field in `crate::package::chart_write`, feature charts, added by 4d2cdc3
- `NewText.italic` field in `crate::package::chart_write`, feature charts, added by 4d2cdc3
- `NewChart.legend_text` field in `crate::package::chart_write`, feature charts, added by 4d2cdc3
- `NewText.size` field in `crate::package::chart_write`, feature charts, added by 4d2cdc3
- `NewChart.style` field in `crate::package::chart_write`, feature charts, added by 075abca
- `NewLabels.text` field in `crate::package::chart_write`, feature charts, added by 4d2cdc3
- `NewChart.title_text` field in `crate::package::chart_write`, feature charts, added by 4d2cdc3
- `NewText` struct in `crate::package::chart_write`, feature charts, added by 4d2cdc3

## Items the feature gates widened to `charts`

0 items, each gated with `charts` as well since M6 part 3 P3's listing, read from the diff over `crates/emend/src/package.rs` and declared there at M6 part 3 P3's phase commit (test.log section T1): 0 pub, in this listing as the cfg-widened rows above; 0 crate-private, not in the pub listing.


## Items new under the shared gate

0 items, each gated with `charts` and another feature since M6 part 3 P3's listing and not declared in `crates/emend/src/package.rs` at its phase commit (test.log section T1): new items, crate-private, not in the pub listing.


## Rows that keep their key and lost Eq

0 rows equal by key in both listings, whose derive drops `Eq` since M6 part 3 P3's phase commit (test.log section T1 reads each derive line at both commits).


## added-by re-dated by ruling V (change AV1)

44 rows whose added-by this listing's program (sha256 bb1fba224a763acac9049d18798bb774820898b2018ad68899c5332076a8f38b) gives otherwise than part 7 P3's program (sha256 59f4043c3014a64e671054fcfd4dce565e210426d4d4847ae448cb29b48b987c) at the same phase commit (test.log section T1 runs both; specs/0040 ruling V, ruling L): each a member whose declaration text an earlier commit's copy of the file already held elsewhere, which part 7's heuristic took; each moves to a later commit (test.log checks each new commit descends from the old), the first whose copy of the file holds the member in its type. Every other column of the two listings is the same.

- `SharedStringItem.runs` field in `crate::cells`, feature read & richtext: 1174bf1 by part 7 P3's program, c68f350 by this one
- `Run.text` field in `crate::cells`, feature richtext: 6c27167 by part 7 P3's program, 1174bf1 by this one
- `SharedStringItem.text` field in `crate::cells`, feature read: 6c27167 by part 7 P3's program, c68f350 by this one
- `Area.a1` method in `crate::cells`, feature core: 6c27167 by part 7 P3's program, d03ed7b by this one
- `Area.parse` method in `crate::cells`, feature core: 6c27167 by part 7 P3's program, d03ed7b by this one
- `NewChart.category_axis` field in `crate::package::chart_write`, feature charts: 4c36f05 by part 7 P3's program, 3c1f1f3 by this one
- `NewLabels.category` field in `crate::package::chart_write`, feature charts: 4c36f05 by part 7 P3's program, 784e3c0 by this one
- `NewMarker.color` field in `crate::package::chart_write`, feature charts: 784e3c0 by part 7 P3's program, 1637495 by this one
- `NewPoint.color` field in `crate::package::chart_write`, feature charts: 784e3c0 by part 7 P3's program, 1637495 by this one
- `NewText.color` field in `crate::package::chart_write`, feature charts: 784e3c0 by part 7 P3's program, 4d2cdc3 by this one
- `NewTrendline.name` field in `crate::package::chart_write`, feature charts: 4c36f05 by part 7 P3's program, 12f1314 by this one
- `NewCombined.plot` field in `crate::package::chart_write`, feature charts: 4c36f05 by part 7 P3's program, a5823b7 by this one
- `NewCombined.series` field in `crate::package::chart_write`, feature charts: 4c36f05 by part 7 P3's program, a5823b7 by this one
- `NewMarker.size` field in `crate::package::chart_write`, feature charts: 4c36f05 by part 7 P3's program, 1637495 by this one
- `NewText.size` field in `crate::package::chart_write`, feature charts: 4c36f05 by part 7 P3's program, 4d2cdc3 by this one
- `NewChart.value_axis` field in `crate::package::chart_write`, feature charts: 4c36f05 by part 7 P3's program, 3c1f1f3 by this one
- `NewErrorBars.value` field in `crate::package::chart_write`, feature charts: 4c36f05 by part 7 P3's program, 12f1314 by this one
- `NewLabels.value` field in `crate::package::chart_write`, feature charts: 4c36f05 by part 7 P3's program, 784e3c0 by this one
- `ChartAxis.kind` field in `crate::package::chart`, feature charts: c566a01 by part 7 P3's program, 60ad190 by this one
- `ChartAxis.title` field in `crate::package::chart`, feature charts: c566a01 by part 7 P3's program, 60ad190 by this one
- `RuleEntry.areas` field in `crate::package`, feature rules: 2ed9070 by part 7 P3's program, 08729d3 by this one
- `TableEntry.area` field in `crate::package`, feature tables: 2ed9070 by part 7 P3's program, 2e84516 by this one
- `ContentTypeDefault.content_type` field in `crate::package`, feature core: 5fc1b3d by part 7 P3's program, ff0a480 by this one
- `ContentTypeDefault.extension` field in `crate::package`, feature core: 5fc1b3d by part 7 P3's program, ff0a480 by this one
- `PageMargins.header` field in `crate::package`, feature any(sheetprops, read): 2e84516 by part 7 P3's program, 791a794 by this one
- `Cfvo.kind` field in `crate::package`, feature rules: 5fc1b3d by part 7 P3's program, 1265000 by this one
- `RuleEntry.kind` field in `crate::package`, feature rules: 5fc1b3d by part 7 P3's program, 08729d3 by this one
- `ValidationEntry.kind` field in `crate::package`, feature rules: 5fc1b3d by part 7 P3's program, 2ed9070 by this one
- `Validation.kind` field in `crate::package`, feature rules: 5fc1b3d by part 7 P3's program, 2ed9070 by this one
- `DefinedName.name` field in `crate::package`, feature metadata: 5fc1b3d by part 7 P3's program, 2634ecd by this one
- `TableEntry.name` field in `crate::package`, feature tables: 5fc1b3d by part 7 P3's program, 2e84516 by this one
- `Table.name` field in `crate::package`, feature tables: 5fc1b3d by part 7 P3's program, 2e84516 by this one
- `RuleEntry.operator` field in `crate::package`, feature rules: 2ed9070 by part 7 P3's program, 08729d3 by this one
- `ImageEntry.part` field in `crate::package`, feature images: 5fc1b3d by part 7 P3's program, b65ea6d by this one
- `TableEntry.part` field in `crate::package`, feature tables: 5fc1b3d by part 7 P3's program, 2e84516 by this one
- `PageMargins.right` field in `crate::package`, feature any(sheetprops, read): 4f53d49 by part 7 P3's program, 791a794 by this one
- `DefinedName.sheet` field in `crate::package`, feature metadata: 5fc1b3d by part 7 P3's program, 2634ecd by this one
- `SheetProtection.sheet` field in `crate::package`, feature any(metadata, read): 5fc1b3d by part 7 P3's program, 51acd32 by this one
- `TableEntry.style` field in `crate::package`, feature tables: 5fc1b3d by part 7 P3's program, 2e84516 by this one
- `TableStyleInfo.style` field in `crate::package`, feature tables: 5fc1b3d by part 7 P3's program, 2e84516 by this one
- `Table.style` field in `crate::package`, feature tables: 5fc1b3d by part 7 P3's program, 2e84516 by this one
- `CommentEntry.text` field in `crate::package`, feature comments: 2634ecd by part 7 P3's program, 96dde75 by this one
- `Comment.text` field in `crate::package`, feature comments: 2634ecd by part 7 P3's program, 96dde75 by this one
- `Scope.new` method in `crate::xml`, feature core: a9f2cad by part 7 P3's program, 5fc1b3d by this one

## The items, by module

### `crate`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `archive` | mod | core | e14dbee | specs/0008 | keep |
| 2 | `cells` | mod | core | 8f2364d | specs/0010 | keep |
| 3 | `dates` | mod | dates | c08bf76 | specs/0012 | keep |
| 4 | `package` | mod | core | 5fc1b3d | specs/0009 | keep |
| 5 | `styles` | mod | core | 35cd499 | specs/0012 | keep |
| 6 | `xml` | mod | core | a9f2cad | specs/0009 | keep |
| 7 | `use archive::{Archive, Edits, Entry, Error, Replacement, replacement_from_reader}` | use | core | ac1bd12 | specs/0008 | keep |
| 8 | `use cells::{Area, Cell, CellKind, CellRef, Cells, Formula, FormulaKind, SharedStrings, Value}` | use | core | f7bb23e | specs/0030 | keep |
| 9 | `use cells::{RichText, Run}` | use | richtext | 1174bf1 | specs/0024 | keep |
| 10 | `use dates::{DateTime, Epoch}` | use | dates | c08bf76 | specs/0012 | keep |
| 11 | `use package::{Anchor, EditAs, Image, ImageAnchor, ImageEntry}` | use | images | 7976d9e | specs/0028 | keep |
| 12 | `use package::{ Cfvo, CfvoKind, ErrorStyle, IconSetKind, Operator, Rule, RuleEntry, Validation, ValidationEntry, ValidationKind, }` | use | rules | a791e2c | specs/0019 | keep |
| 13 | `use package::{Comment, CommentBox, CommentEntry}` | use | comments | 69b5e0e | specs/0028 | keep |
| 14 | `use package::{ DefinedName, LinkTarget, Properties, Property, SheetProtection, legacy_password_hash, }` | use | metadata | a791e2c | specs/0019 | keep |
| 15 | `use package::{ HeaderFooter, Orientation, PageMargins, PageOrder, PageSetup, PrintOptions, SheetView, ViewKind, }` | use | sheetprops | a791e2c | specs/0019 | keep |
| 16 | `use package::{Table, TableEntry, TableStyle, TableStyleInfo}` | use | tables | 2e84516 | specs/0021 | keep |
| 17 | `use styles::{ Alignment, Border, BorderStyle, Color, Fill, Font, Horizontal, NumFmt, NumberFormat, Pattern, Protection, Scheme, Side, Style, Styles, Underline, Vertical, VerticalAlign, Xf, }` | use | core | 35cd499 | specs/0012 | keep |
| 18 | `use xml::{BYTE_ORDER_MARK, Scope, Stream, Token}` | use | core | 5fc1b3d | specs/0009 | keep |

### `crate::archive`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 19 | `Error` | enum | core | e14dbee | specs/0008 | keep |
| 20 | `Entry` | struct | core | e14dbee | specs/0008 | keep |
| 21 | `Entry.name` | method | core | e14dbee | specs/0008 | keep |
| 22 | `Entry.method` | method | core | e14dbee | specs/0008 | keep |
| 23 | `Entry.crc32` | method | core | e14dbee | specs/0008 | keep |
| 24 | `Entry.compressed_size` | method | core | e14dbee | specs/0008 | keep |
| 25 | `Entry.uncompressed_size` | method | core | e14dbee | specs/0008 | keep |
| 26 | `Entry.local_offset` | method | core | e14dbee | specs/0008 | keep |
| 27 | `Entry.has_descriptor` | method | core | e14dbee | specs/0008 | keep |
| 28 | `Entry.is_dir` | method | core | e14dbee | specs/0008 | keep |
| 29 | `Entry.is_encrypted` | method | core | e14dbee | specs/0008 | keep |
| 30 | `Replacement` | type | core | e14dbee | specs/0008 | keep |
| 31 | `replacement_from_reader` | fn | core | ac1bd12 | specs/0008 | keep |
| 32 | `Edits` | struct | core | e14dbee | specs/0008 | keep |
| 33 | `Edits.new` | method | core | e14dbee | specs/0008 | keep |
| 34 | `Edits.replace` | method | core | e14dbee | specs/0008 | keep |
| 35 | `Edits.remove` | method | core | e14dbee | specs/0008 | keep |
| 36 | `Edits.add` | method | core | e14dbee | specs/0008 | keep |
| 37 | `Edits.is_empty` | method | core | e14dbee | specs/0008 | keep |
| 38 | `Archive` | struct | core | e14dbee | specs/0008 | keep |
| 39 | `Archive.open` | method | core | e14dbee | specs/0008 | keep |
| 40 | `Archive.entries` | method | core | e14dbee | specs/0008 | keep |
| 41 | `Archive.index_of` | method | core | e14dbee | specs/0008 | keep |
| 42 | `Archive.comment` | method | core | e14dbee | specs/0008 | keep |
| 43 | `Archive.read` | method | core | e14dbee | specs/0008 | keep |
| 44 | `Archive.write_to` | method | core | e14dbee | specs/0008 | keep |

### `crate::cells`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 45 | `SharedStrings` | struct | core | 8f2364d | specs/0010 | keep |
| 46 | `SharedStrings.len` | method | core | 8f2364d | specs/0010 | keep |
| 47 | `SharedStrings.is_empty` | method | core | 8f2364d | specs/0010 | keep |
| 48 | `SharedStrings.get` | method | core | 8f2364d | specs/0010 | keep |
| 49 | `SharedStrings.runs_of` | method | richtext | 1174bf1 | specs/0024 | keep |
| 50 | `SharedStrings.resolve` | method | core | 6c27167 | specs/0010 | keep |
| 51 | `CellRef` | struct | core | 6c27167 | specs/0010 | keep |
| 52 | `CellRef.row` | field | core | 6c27167 | specs/0010 | keep |
| 53 | `CellRef.col` | field | core | 6c27167 | specs/0010 | keep |
| 54 | `CellRef.parse` | method | core | 6c27167 | specs/0010 | keep |
| 55 | `CellRef.a1` | method | core | 6c27167 | specs/0010 | keep |
| 56 | `Area` | struct | core | d03ed7b | specs/0018 | keep |
| 57 | `Area.first` | field | core | d03ed7b | specs/0018 | keep |
| 58 | `Area.last` | field | core | d03ed7b | specs/0018 | keep |
| 59 | `Area.parse` | method | core | d03ed7b | specs/0018 | keep |
| 60 | `Area.a1` | method | core | d03ed7b | specs/0018 | keep |
| 61 | `Area.cells` | method | core | d03ed7b | specs/0018 | keep |
| 62 | `CellKind` | enum | core | f7bb23e | specs/0030 | keep |
| 63 | `FormulaKind` | enum | core | 6c27167 | specs/0010 | keep |
| 64 | `Formula` | struct | core | 6c27167 | specs/0010 | keep |
| 65 | `Formula.text` | field | core | 6c27167 | specs/0010 | keep |
| 66 | `Formula.kind` | field | core | 6c27167 | specs/0010 | keep |
| 67 | `Formula.reference` | field | core | 6c27167 | specs/0010 | keep |
| 68 | `Cell` | struct | core | 6c27167 | specs/0010 | keep |
| 69 | `Cell.reference` | field | core | 6c27167 | specs/0010 | keep |
| 70 | `Cell.style` | field | core | 6c27167 | specs/0010 | keep |
| 71 | `Cell.kind` | field | core | 6c27167 | specs/0010 | keep |
| 72 | `Cell.value` | field | core | 6c27167 | specs/0010 | keep |
| 73 | `Cell.formula` | field | core | 6c27167 | specs/0010 | keep |
| 74 | `Cell.runs` | field | richtext | 1174bf1 | specs/0024 | keep |
| 75 | `Cell.number` | method | core | 6c27167 | specs/0010 | keep |
| 76 | `Cell.boolean` | method | core | 6c27167 | specs/0010 | keep |
| 77 | `Cell.error` | method | core | 6c27167 | specs/0010 | keep |
| 78 | `Value` | enum | core | aa87cbc | specs/0011 | keep |
| 79 | `Cells` | struct | core | 6c27167 | specs/0010 | keep |
| 80 | `Run` | struct | richtext | 1174bf1 | specs/0024 | keep |
| 81 | `Run.text` | field | richtext | 1174bf1 | specs/0024 | keep |
| 82 | `Run.font` | field | richtext | 1174bf1 | specs/0024 | keep |
| 83 | `RichText` | struct | richtext | 1174bf1 | specs/0024 | keep |
| 84 | `RichText.runs` | field | richtext | 1174bf1 | specs/0024 | keep |
| 85 | `RichText.text` | method | richtext | 1174bf1 | specs/0024 | keep |
| 86 | `SharedStringItem` | struct | read | c68f350 | specs/0030 | keep |
| 87 | `SharedStringItem.text` | field | read | c68f350 | specs/0030 | keep |
| 88 | `SharedStringItem.runs` | field | read & richtext | c68f350 | specs/0030 | keep |
| 89 | `SharedStringItems` | struct | read | 5e7bb62 | specs/0030 | keep |

### `crate::dates`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 90 | `Epoch` | enum | dates | c08bf76 | specs/0012 | keep |
| 91 | `Epoch.of` | method | dates | c08bf76 | specs/0012 | keep |
| 92 | `DateTime` | struct | dates | c08bf76 | specs/0012 | keep |
| 93 | `DateTime.year` | field | dates | c08bf76 | specs/0012 | keep |
| 94 | `DateTime.month` | field | dates | c08bf76 | specs/0012 | keep |
| 95 | `DateTime.day` | field | dates | c08bf76 | specs/0012 | keep |
| 96 | `DateTime.hour` | field | dates | c08bf76 | specs/0012 | keep |
| 97 | `DateTime.minute` | field | dates | c08bf76 | specs/0012 | keep |
| 98 | `DateTime.second` | field | dates | c08bf76 | specs/0012 | keep |
| 99 | `DateTime.millisecond` | field | dates | c08bf76 | specs/0012 | keep |
| 100 | `DateTime.serial` | method | dates | c08bf76 | specs/0012 | keep |
| 101 | `DateTime.from_serial` | method | dates | c08bf76 | specs/0012 | keep |

### `crate::package`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 102 | `use chart::{ AxisKind, AxisPosition, BarDirection, Chart, ChartAnchor, ChartAxis, ChartEntry, ChartKind, Data, Grouping, LegendPosition, Plot, ScatterStyle, Series, SeriesName, }` | use | charts | c566a01 | specs/0034 | keep |
| 103 | `use chart_edit::ChartSeriesEdit` | use | charts | 075abca | specs/0041 | keep |
| 104 | `use chart_write::{ DashStyle, ErrorBarKind, ErrorDirection, ErrorValue, LabelPosition, MarkerSymbol, NewAxis, NewChart, NewCombined, NewDateAxis, NewErrorBars, NewLabels, NewLine, NewMarker, NewPlot, NewPoint, NewSeries, NewSeriesName, NewText, NewTrendline, OfPieKind, OfPieSplit, RadarStyle, TimeUnit, TrendlineKind, }` | use | charts | 4c36f05 | specs/0035 | keep |
| 105 | `use extlink::{ExternalLink, ExternalLinkKind}` | use | read | cdffef7 | specs/0033 | keep |
| 106 | `use pivot::{ Axis, DataField, FieldRef, PivotCache, PivotField, PivotSource, PivotTable, Subtotal, }` | use | read | ec9e58f | specs/0033 | keep |
| 107 | `use sheet_read::{ ColumnFormat, HyperlinkEntry, HyperlinkTarget, RowFormat, Rows, SheetSettings, }` | use | read | bd02ce4 | specs/0027 | keep |
| 108 | `use survey::{PartKinds, SheetParts}` | use | read | 338e91c | specs/0033 | keep |
| 109 | `CONTENT_TYPES_PART` | const | core | 5fc1b3d | specs/0009 | keep |
| 110 | `CONTENT_TYPES_NS` | const | core | 5fc1b3d | specs/0009 | keep |
| 111 | `RELATIONSHIPS_NS` | const | core | 5fc1b3d | specs/0009 | keep |
| 112 | `MAIN` | const | core | 5fc1b3d | specs/0009 | keep |
| 113 | `RELATIONSHIPS_ATTRIBUTE` | const | core | 5fc1b3d | specs/0009 | keep |
| 114 | `OFFICE_DOCUMENT` | const | core | 5fc1b3d | specs/0009 | keep |
| 115 | `WORKSHEET` | const | core | 5fc1b3d | specs/0009 | keep |
| 116 | `CHARTSHEET` | const | core | 5fc1b3d | specs/0009 | keep |
| 117 | `DIALOGSHEET` | const | core | 5fc1b3d | specs/0009 | keep |
| 118 | `SHARED_STRINGS` | const | core | 5fc1b3d | specs/0009 | keep |
| 119 | `STYLES` | const | core | 5fc1b3d | specs/0009 | keep |
| 120 | `CALC_CHAIN` | const | core | 5fc1b3d | specs/0009 | keep |
| 121 | `THEME` | const | core | 5fc1b3d | specs/0009 | keep |
| 122 | `TABLE` | const | any( rowcol, tables, read, all(sheetprops, metadata) ) | 7578ee0 | specs/0018 | keep |
| 123 | `PIVOT_TABLE` | const | any(rowcol, tables, read) | 7578ee0 | specs/0018 | keep |
| 124 | `COMMENTS` | const | any(comments, copy, read) | 96dde75 | specs/0022 | keep |
| 125 | `VML_DRAWING` | const | any(comments, read) | 96dde75 | specs/0022 | keep |
| 126 | `THREADED_COMMENTS` | const | comments | 96dde75 | specs/0022 | keep |
| 127 | `DRAWING` | const | any(images, read, charts) | b65ea6d | specs/0022 | keep |
| 128 | `IMAGE` | const | any(images, read) | b65ea6d | specs/0022 | keep |
| 129 | `HYPERLINK` | const | any(metadata, copy, read) | a791e2c | specs/0019 | keep |
| 130 | `EXTERNAL_LINK` | const | read | 338e91c | specs/0033 | keep |
| 131 | `EXTERNAL_LINK_PATH` | const | read | 338e91c | specs/0033 | keep |
| 132 | `PIVOT_CACHE_DEFINITION` | const | read | 338e91c | specs/0033 | keep |
| 133 | `CHART` | const | any(read, charts) | 5fc1b3d | specs/0009 | keep |
| 134 | `SLICER` | const | read | 338e91c | specs/0033 | keep |
| 135 | `TIMELINE` | const | read | 338e91c | specs/0033 | keep |
| 136 | `PRINTER_SETTINGS` | const | read | 338e91c | specs/0033 | keep |
| 137 | `CORE_PROPERTIES` | const | metadata | 51acd32 | specs/0019 | keep |
| 138 | `Property` | enum | metadata | 51acd32 | specs/0019 | keep |
| 139 | `Properties` | struct | metadata | 51acd32 | specs/0019 | keep |
| 140 | `Properties.get` | method | metadata | 51acd32 | specs/0019 | keep |
| 141 | `SheetProtection` | struct | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 142 | `SheetProtection.sheet` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 143 | `SheetProtection.objects` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 144 | `SheetProtection.scenarios` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 145 | `SheetProtection.format_cells` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 146 | `SheetProtection.format_columns` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 147 | `SheetProtection.format_rows` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 148 | `SheetProtection.insert_columns` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 149 | `SheetProtection.insert_rows` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 150 | `SheetProtection.insert_hyperlinks` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 151 | `SheetProtection.delete_columns` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 152 | `SheetProtection.delete_rows` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 153 | `SheetProtection.select_locked_cells` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 154 | `SheetProtection.sort` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 155 | `SheetProtection.auto_filter` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 156 | `SheetProtection.pivot_tables` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 157 | `SheetProtection.select_unlocked_cells` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 158 | `legacy_password_hash` | fn | metadata | 51acd32 | specs/0019 | keep |
| 159 | `ContentTypeDefault` | struct | core | ff0a480 | specs/0030 | keep |
| 160 | `ContentTypeDefault.extension` | field | core | ff0a480 | specs/0030 | keep |
| 161 | `ContentTypeDefault.content_type` | field | core | ff0a480 | specs/0030 | keep |
| 162 | `Override` | struct | core | 5fc1b3d | specs/0009 | keep |
| 163 | `Override.part_name` | field | core | 5fc1b3d | specs/0009 | keep |
| 164 | `Override.content_type` | field | core | 5fc1b3d | specs/0009 | keep |
| 165 | `ContentTypes` | struct | core | 5fc1b3d | specs/0009 | keep |
| 166 | `ContentTypes.defaults` | method | core | 5fc1b3d | specs/0009 | keep |
| 167 | `ContentTypes.overrides` | method | core | 5fc1b3d | specs/0009 | keep |
| 168 | `ContentTypes.of` | method | core | 5fc1b3d | specs/0009 | keep |
| 169 | `Relationship` | struct | core | 5fc1b3d | specs/0009 | keep |
| 170 | `Relationship.id` | field | core | 5fc1b3d | specs/0009 | keep |
| 171 | `Relationship.kind` | field | core | 5fc1b3d | specs/0009 | keep |
| 172 | `Relationship.target` | field | core | 5fc1b3d | specs/0009 | keep |
| 173 | `Relationship.external` | field | core | 5fc1b3d | specs/0009 | keep |
| 174 | `Relationships` | struct | core | 5fc1b3d | specs/0009 | keep |
| 175 | `Relationships.source` | method | core | 5fc1b3d | specs/0009 | keep |
| 176 | `Relationships.all` | method | core | 5fc1b3d | specs/0009 | keep |
| 177 | `Relationships.by_id` | method | core | 5fc1b3d | specs/0009 | keep |
| 178 | `Relationships.of_kind` | method | core | 5fc1b3d | specs/0009 | keep |
| 179 | `Relationships.first_of` | method | core | 5fc1b3d | specs/0009 | keep |
| 180 | `Relationships.target_part` | method | core | 5fc1b3d | specs/0009 | keep |
| 181 | `SheetKind` | enum | core | 5fc1b3d | specs/0009 | keep |
| 182 | `Sheet` | struct | core | 5fc1b3d | specs/0009 | keep |
| 183 | `Sheet.name` | field | core | 5fc1b3d | specs/0009 | keep |
| 184 | `Sheet.sheet_id` | field | core | 5fc1b3d | specs/0009 | keep |
| 185 | `Sheet.rel_id` | field | core | 5fc1b3d | specs/0009 | keep |
| 186 | `Sheet.state` | field | core | 5fc1b3d | specs/0009 | keep |
| 187 | `Sheet.part` | field | core | 5fc1b3d | specs/0009 | keep |
| 188 | `Sheet.kind` | field | core | 5fc1b3d | specs/0009 | keep |
| 189 | `Workbook` | struct | core | 5fc1b3d | specs/0009 | keep |
| 190 | `Workbook.part` | field | core | 5fc1b3d | specs/0009 | keep |
| 191 | `Workbook.sheets` | field | core | 5fc1b3d | specs/0009 | keep |
| 192 | `Workbook.shared_strings` | field | core | 5fc1b3d | specs/0009 | keep |
| 193 | `Workbook.styles` | field | core | 5fc1b3d | specs/0009 | keep |
| 194 | `Workbook.calc_chain` | field | core | 5fc1b3d | specs/0009 | keep |
| 195 | `Workbook.theme` | field | core | 5fc1b3d | specs/0009 | keep |
| 196 | `Workbook.date1904` | field | core | 35cd499 | specs/0012 | keep |
| 197 | `Workbook.full_calc_on_load` | field | core | 77c3016 | specs/0013 | keep |
| 198 | `Workbook.sheet` | method | core | 5fc1b3d | specs/0009 | keep |
| 199 | `Workbook.defined_names` | method | metadata | 2634ecd | specs/0019 | keep |
| 200 | `DefinedName` | struct | metadata | 2634ecd | specs/0019 | keep |
| 201 | `DefinedName.name` | field | metadata | 2634ecd | specs/0019 | keep |
| 202 | `DefinedName.text` | field | metadata | 2634ecd | specs/0019 | keep |
| 203 | `DefinedName.sheet` | field | metadata | 2634ecd | specs/0019 | keep |
| 204 | `DefinedName.hidden` | field | metadata | 2634ecd | specs/0019 | keep |
| 205 | `Package` | struct | core | 5fc1b3d | specs/0009 | keep |
| 206 | `LinkTarget` | enum | metadata | a791e2c | specs/0019 | keep |
| 207 | `ValidationKind` | enum | rules | 2ed9070 | specs/0020 | keep |
| 208 | `Operator` | enum | rules | 2ed9070 | specs/0020 | keep |
| 209 | `ErrorStyle` | enum | rules | 2ed9070 | specs/0020 | keep |
| 210 | `Validation` | struct | rules | 2ed9070 | specs/0020 | keep |
| 211 | `Validation.kind` | field | rules | 2ed9070 | specs/0020 | keep |
| 212 | `Validation.operator` | field | rules | 2ed9070 | specs/0020 | keep |
| 213 | `Validation.formula1` | field | rules | 2ed9070 | specs/0020 | keep |
| 214 | `Validation.formula2` | field | rules | 2ed9070 | specs/0020 | keep |
| 215 | `Validation.allow_blank` | field | rules | 2ed9070 | specs/0020 | keep |
| 216 | `Validation.hide_drop_down` | field | rules | 2ed9070 | specs/0020 | keep |
| 217 | `Validation.input` | field | rules | 2ed9070 | specs/0020 | keep |
| 218 | `Validation.error` | field | rules | 2ed9070 | specs/0020 | keep |
| 219 | `ValidationEntry` | struct | rules | 2ed9070 | specs/0020 | keep |
| 220 | `ValidationEntry.areas` | field | rules | 2ed9070 | specs/0020 | keep |
| 221 | `ValidationEntry.kind` | field | rules | 2ed9070 | specs/0020 | keep |
| 222 | `ValidationEntry.operator` | field | rules | 2ed9070 | specs/0020 | keep |
| 223 | `ValidationEntry.formula1` | field | rules | 2ed9070 | specs/0020 | keep |
| 224 | `ValidationEntry.formula2` | field | rules | 2ed9070 | specs/0020 | keep |
| 225 | `ValidationEntry.allow_blank` | field | rules | 2ed9070 | specs/0020 | keep |
| 226 | `ValidationEntry.hide_drop_down` | field | rules | 2ed9070 | specs/0020 | keep |
| 227 | `ValidationEntry.input` | field | rules | 2ed9070 | specs/0020 | keep |
| 228 | `ValidationEntry.error` | field | rules | 2ed9070 | specs/0020 | keep |
| 229 | `Rule` | enum | rules | 08729d3 | specs/0020 | keep |
| 230 | `RuleEntry` | struct | rules | 08729d3 | specs/0020 | keep |
| 231 | `RuleEntry.areas` | field | rules | 08729d3 | specs/0020 | keep |
| 232 | `RuleEntry.kind` | field | rules | 08729d3 | specs/0020 | keep |
| 233 | `RuleEntry.operator` | field | rules | 08729d3 | specs/0020 | keep |
| 234 | `RuleEntry.formulas` | field | rules | 08729d3 | specs/0020 | keep |
| 235 | `RuleEntry.dxf` | field | rules | 08729d3 | specs/0020 | keep |
| 236 | `RuleEntry.priority` | field | rules | 08729d3 | specs/0020 | keep |
| 237 | `RuleEntry.stop_if_true` | field | rules | 08729d3 | specs/0020 | keep |
| 238 | `RuleEntry.extended` | field | rules | 08729d3 | specs/0020 | keep |
| 239 | `CfvoKind` | enum | rules | 1265000 | specs/0020 | keep |
| 240 | `Cfvo` | struct | rules | 1265000 | specs/0020 | keep |
| 241 | `Cfvo.kind` | field | rules | 1265000 | specs/0020 | keep |
| 242 | `Cfvo.color` | field | rules | 1265000 | specs/0020 | keep |
| 243 | `IconSetKind` | enum | rules | 1265000 | specs/0020 | keep |
| 244 | `TableStyle` | enum | tables | 2e84516 | specs/0021 | keep |
| 245 | `TableStyleInfo` | struct | tables | 2e84516 | specs/0021 | keep |
| 246 | `TableStyleInfo.style` | field | tables | 2e84516 | specs/0021 | keep |
| 247 | `TableStyleInfo.first_column` | field | tables | 2e84516 | specs/0021 | keep |
| 248 | `TableStyleInfo.last_column` | field | tables | 2e84516 | specs/0021 | keep |
| 249 | `TableStyleInfo.row_stripes` | field | tables | 2e84516 | specs/0021 | keep |
| 250 | `TableStyleInfo.column_stripes` | field | tables | 2e84516 | specs/0021 | keep |
| 251 | `Table` | struct | tables | 2e84516 | specs/0021 | keep |
| 252 | `Table.name` | field | tables | 2e84516 | specs/0021 | keep |
| 253 | `Table.columns` | field | tables | 2e84516 | specs/0021 | keep |
| 254 | `Table.style` | field | tables | 2e84516 | specs/0021 | keep |
| 255 | `TableEntry` | struct | tables | 2e84516 | specs/0021 | keep |
| 256 | `TableEntry.name` | field | tables | 2e84516 | specs/0021 | keep |
| 257 | `TableEntry.display_name` | field | tables | 2e84516 | specs/0021 | keep |
| 258 | `TableEntry.area` | field | tables | 2e84516 | specs/0021 | keep |
| 259 | `TableEntry.columns` | field | tables | 2e84516 | specs/0021 | keep |
| 260 | `TableEntry.style` | field | tables | 2e84516 | specs/0021 | keep |
| 261 | `TableEntry.header_row` | field | tables | 2e84516 | specs/0021 | keep |
| 262 | `TableEntry.totals_row` | field | tables | 2e84516 | specs/0021 | keep |
| 263 | `TableEntry.calculated` | field | tables | 2e84516 | specs/0021 | keep |
| 264 | `TableEntry.part` | field | tables | 2e84516 | specs/0021 | keep |
| 265 | `Comment` | struct | comments | 96dde75 | specs/0022 | keep |
| 266 | `Comment.author` | field | comments | 96dde75 | specs/0022 | keep |
| 267 | `Comment.text` | field | comments | 96dde75 | specs/0022 | keep |
| 268 | `CommentEntry` | struct | comments | 96dde75 | specs/0022 | keep |
| 269 | `CommentEntry.cell` | field | comments | 96dde75 | specs/0022 | keep |
| 270 | `CommentEntry.author` | field | comments | 96dde75 | specs/0022 | keep |
| 271 | `CommentEntry.text` | field | comments | 96dde75 | specs/0022 | keep |
| 272 | `CommentEntry.runs` | field | comments & richtext | 17d1e6b | specs/0024 | keep |
| 273 | `CommentEntry.shape` | field | comments | 69b5e0e | specs/0028 | keep |
| 274 | `CommentBox` | struct | comments | 69b5e0e | specs/0028 | keep |
| 275 | `CommentBox.width` | field | comments | 69b5e0e | specs/0028 | keep |
| 276 | `CommentBox.height` | field | comments | 69b5e0e | specs/0028 | keep |
| 277 | `CommentBox.visible` | field | comments | 69b5e0e | specs/0028 | keep |
| 278 | `Image` | struct | images | b65ea6d | specs/0022 | keep |
| 279 | `Image.bytes` | field | images | b65ea6d | specs/0022 | keep |
| 280 | `Image.size` | field | images | b65ea6d | specs/0022 | keep |
| 281 | `Anchor` | enum | any(images, charts) | b65ea6d | specs/0022 | keep |
| 282 | `ImageAnchor` | enum | any(images, charts) | 7976d9e | specs/0028 | keep |
| 283 | `EditAs` | enum | any(images, charts) | 7976d9e | specs/0028 | keep |
| 284 | `ImageEntry` | struct | images | b65ea6d | specs/0022 | keep |
| 285 | `ImageEntry.from` | field | images | b65ea6d | specs/0022 | keep |
| 286 | `ImageEntry.anchor` | field | images | b65ea6d | specs/0022 | keep |
| 287 | `ImageEntry.part` | field | images | b65ea6d | specs/0022 | keep |
| 288 | `ImageEntry.bytes` | field | images | b65ea6d | specs/0022 | keep |
| 289 | `Package.new` | method | core | e7c7061 | specs/0016 | keep |
| 290 | `Package.open` | method | core | 5fc1b3d | specs/0009 | keep |
| 291 | `Package.archive` | method | core | 5fc1b3d | specs/0009 | keep |
| 292 | `Package.part` | method | core | 5fc1b3d | specs/0009 | keep |
| 293 | `Package.read_part` | method | core | 5fc1b3d | specs/0009 | keep |
| 294 | `Package.shared_strings` | method | core | 8f2364d | specs/0010 | keep |
| 295 | `Package.shared_string_items` | method | read | bd02ce4 | specs/0027 | keep |
| 296 | `Package.cells` | method | core | 6c27167 | specs/0010 | keep |
| 297 | `Package.content_types` | method | core | 5fc1b3d | specs/0009 | keep |
| 298 | `Package.relationships` | method | core | 5fc1b3d | specs/0009 | keep |
| 299 | `Package.workbook` | method | core | 5fc1b3d | specs/0009 | keep |
| 300 | `Package.styles` | method | core | 35cd499 | specs/0012 | keep |
| 301 | `Package.cell_style` | method | read | c019afa | specs/0027 | keep |
| 302 | `Package.named_style` | method | read | 0b3e46f | specs/0032 | keep |
| 303 | `Package.differential_styles` | method | read | 0b3e46f | specs/0032 | keep |
| 304 | `Package.default_font` | method | read | 0b3e46f | specs/0032 | keep |
| 305 | `Package.theme_colors` | method | read | 85bb33f | specs/0032 | keep |
| 306 | `Package.resolve_color` | method | read | 85bb33f | specs/0032 | keep |
| 307 | `Package.remove_relationship` | method | core | 5dba448 | specs/0009 | keep |
| 308 | `Package.add_relationship` | method | core | 5dba448 | specs/0009 | keep |
| 309 | `Package.remove_override` | method | core | 5dba448 | specs/0009 | keep |
| 310 | `Package.add_override` | method | core | 5dba448 | specs/0009 | keep |
| 311 | `Package.remove_part` | method | core | 5dba448 | specs/0009 | keep |
| 312 | `Package.replace_part` | method | core | 5dba448 | specs/0009 | keep |
| 313 | `Package.add_part` | method | core | 5dba448 | specs/0009 | keep |
| 314 | `Package.drop_calc_chain` | method | core | 2e3bb2a | specs/0011 | keep |
| 315 | `Package.set_cell` | method | core | aa87cbc | specs/0011 | keep |
| 316 | `Package.clear_cell` | method | core | aa87cbc | specs/0011 | keep |
| 317 | `Package.add_cell_style` | method | core | abf2618 | specs/0012 | keep |
| 318 | `Package.add_style` | method | core | f14885f | specs/0017 | keep |
| 319 | `Package.set_cell_style` | method | core | abf2618 | specs/0012 | keep |
| 320 | `Package.set_column_width` | method | core | d03ed7b | specs/0018 | keep |
| 321 | `Package.set_column_hidden` | method | core | d03ed7b | specs/0018 | keep |
| 322 | `Package.set_row_height` | method | core | d03ed7b | specs/0018 | keep |
| 323 | `Package.set_row_hidden` | method | core | d03ed7b | specs/0018 | keep |
| 324 | `Package.group_rows` | method | rowcol | 7b3d9b2 | specs/0026 | keep |
| 325 | `Package.group_columns` | method | rowcol | 7b3d9b2 | specs/0026 | keep |
| 326 | `Package.set_row_collapsed` | method | rowcol | 7b3d9b2 | specs/0026 | keep |
| 327 | `Package.set_column_collapsed` | method | rowcol | 7b3d9b2 | specs/0026 | keep |
| 328 | `Package.set_outline_summary` | method | rowcol | 7b3d9b2 | specs/0026 | keep |
| 329 | `Package.set_row_style` | method | rowcol | 9d266f0 | specs/0026 | keep |
| 330 | `Package.set_column_style` | method | rowcol | 9d266f0 | specs/0026 | keep |
| 331 | `Package.freeze_panes` | method | core | d03ed7b | specs/0018 | keep |
| 332 | `Package.set_tab_color` | method | sheetprops | 4f53d49 | specs/0025 | keep |
| 333 | `Package.set_sheet_view` | method | sheetprops | 4f53d49 | specs/0025 | keep |
| 334 | `Package.set_print_options` | method | sheetprops | 791a794 | specs/0025 | keep |
| 335 | `Package.set_page_margins` | method | sheetprops | 791a794 | specs/0025 | keep |
| 336 | `Package.set_page_setup` | method | sheetprops | 791a794 | specs/0025 | keep |
| 337 | `Package.set_header_footer` | method | sheetprops | 791a794 | specs/0025 | keep |
| 338 | `Package.set_print_area` | method | all(sheetprops, metadata) | 8faf378 | specs/0025 | keep |
| 339 | `Package.set_print_titles` | method | all(sheetprops, metadata) | 8faf378 | specs/0025 | keep |
| 340 | `Package.set_auto_filter` | method | all(sheetprops, metadata) | 8faf378 | specs/0025 | keep |
| 341 | `Package.merge_cells` | method | core | d03ed7b | specs/0018 | keep |
| 342 | `Package.unmerge_cells` | method | core | d03ed7b | specs/0018 | keep |
| 343 | `Package.insert_rows` | method | rowcol | 7578ee0 | specs/0018 | keep |
| 344 | `Package.delete_rows` | method | rowcol | 7578ee0 | specs/0018 | keep |
| 345 | `Package.insert_columns` | method | rowcol | c3b5bbd | specs/0018 | keep |
| 346 | `Package.delete_columns` | method | rowcol | c3b5bbd | specs/0018 | keep |
| 347 | `Package.add_defined_name` | method | metadata | 2634ecd | specs/0019 | keep |
| 348 | `Package.remove_defined_name` | method | metadata | 2634ecd | specs/0019 | keep |
| 349 | `Package.set_hyperlink` | method | metadata | a791e2c | specs/0019 | keep |
| 350 | `Package.remove_hyperlink` | method | metadata | a791e2c | specs/0019 | keep |
| 351 | `Package.properties` | method | metadata | 51acd32 | specs/0019 | keep |
| 352 | `Package.set_property` | method | metadata | 51acd32 | specs/0019 | keep |
| 353 | `Package.protect_sheet` | method | metadata | 51acd32 | specs/0019 | keep |
| 354 | `Package.unprotect_sheet` | method | metadata | 51acd32 | specs/0019 | keep |
| 355 | `Package.protect_workbook` | method | metadata | 51acd32 | specs/0019 | keep |
| 356 | `Package.unprotect_workbook` | method | metadata | 51acd32 | specs/0019 | keep |
| 357 | `Package.add_validation` | method | rules | 2ed9070 | specs/0020 | keep |
| 358 | `Package.remove_validation` | method | rules | 2ed9070 | specs/0020 | keep |
| 359 | `Package.validations` | method | rules | 2ed9070 | specs/0020 | keep |
| 360 | `Package.add_differential_style` | method | rules | fa3ad04 | specs/0020 | keep |
| 361 | `Package.add_conditional_format` | method | rules | 08729d3 | specs/0020 | keep |
| 362 | `Package.remove_conditional_format` | method | rules | 08729d3 | specs/0020 | keep |
| 363 | `Package.conditional_formats` | method | rules | 08729d3 | specs/0020 | keep |
| 364 | `Package.add_table` | method | tables | 2e84516 | specs/0021 | keep |
| 365 | `Package.remove_table` | method | tables | 4a68144 | specs/0021 | keep |
| 366 | `Package.tables` | method | tables | 2e84516 | specs/0021 | keep |
| 367 | `Package.add_comment` | method | comments | 96dde75 | specs/0022 | keep |
| 368 | `Package.add_rich_comment` | method | all(comments, richtext) | 17d1e6b | specs/0024 | keep |
| 369 | `Package.remove_comment` | method | comments | 193aa84 | specs/0022 | keep |
| 370 | `Package.set_comment_box` | method | comments | 69b5e0e | specs/0028 | keep |
| 371 | `Package.comments` | method | comments | 96dde75 | specs/0022 | keep |
| 372 | `Package.add_image` | method | images | b65ea6d | specs/0022 | keep |
| 373 | `Package.add_image_at` | method | images | 7976d9e | specs/0028 | keep |
| 374 | `Package.remove_image` | method | images | 7976d9e | specs/0028 | keep |
| 375 | `Package.images` | method | images | b65ea6d | specs/0022 | keep |
| 376 | `Package.recalculate_on_open` | method | core | 77c3016 | specs/0013 | keep |
| 377 | `Package.sheets` | method | core | 770c9dd | specs/0016 | keep |
| 378 | `Package.add_sheet` | method | core | 770c9dd | specs/0016 | keep |
| 379 | `Package.rename_sheet` | method | core | 770c9dd | specs/0016 | keep |
| 380 | `Package.move_sheet` | method | core | 770c9dd | specs/0016 | keep |
| 381 | `Package.remove_sheet` | method | core | 8abc666 | specs/0016 | keep |
| 382 | `Package.copy_sheet` | method | copy | 4a8482c | specs/0028 | keep |
| 383 | `Package.write_to` | method | core | 5dba448 | specs/0009 | keep |
| 384 | `rels_part_name` | fn | core | 5fc1b3d | specs/0009 | keep |
| 385 | `rels_part_source` | fn | core | 5fc1b3d | specs/0009 | keep |
| 386 | `SheetView` | struct | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 387 | `SheetView.zoom` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 388 | `SheetView.show_grid_lines` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 389 | `SheetView.show_headings` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 390 | `SheetView.show_zeros` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 391 | `SheetView.show_formulas` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 392 | `SheetView.right_to_left` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 393 | `SheetView.view` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 394 | `ViewKind` | enum | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 395 | `PrintOptions` | struct | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 396 | `PrintOptions.horizontal_centered` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 397 | `PrintOptions.vertical_centered` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 398 | `PrintOptions.headings` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 399 | `PrintOptions.grid_lines` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 400 | `PageMargins` | struct | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 401 | `PageMargins.left` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 402 | `PageMargins.right` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 403 | `PageMargins.top` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 404 | `PageMargins.bottom` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 405 | `PageMargins.header` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 406 | `PageMargins.footer` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 407 | `PageSetup` | struct | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 408 | `PageSetup.orientation` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 409 | `PageSetup.paper_size` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 410 | `PageSetup.scale` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 411 | `PageSetup.fit_to_width` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 412 | `PageSetup.fit_to_height` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 413 | `PageSetup.fit_to_page` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 414 | `PageSetup.first_page_number` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 415 | `PageSetup.black_and_white` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 416 | `PageSetup.draft` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 417 | `PageSetup.copies` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 418 | `PageSetup.page_order` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 419 | `Orientation` | enum | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 420 | `PageOrder` | enum | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 421 | `HeaderFooter` | struct | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 422 | `HeaderFooter.odd_header` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 423 | `HeaderFooter.odd_footer` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 424 | `HeaderFooter.even_header` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 425 | `HeaderFooter.even_footer` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 426 | `HeaderFooter.first_header` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 427 | `HeaderFooter.first_footer` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 428 | `HeaderFooter.section` | method | sheetprops | 791a794 | specs/0025 | keep |

### `crate::styles`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 429 | `use colors::ResolvedColor` | use | read | 85bb33f | specs/0032 | keep |
| 430 | `use elements::{ Alignment, Border, BorderStyle, Color, Fill, Font, Horizontal, Pattern, Protection, Scheme, Side, Style, Underline, Vertical, VerticalAlign, }` | use | core | f14885f | specs/0017 | keep |
| 431 | `use read::{CellStyle, NamedStyle}` | use | read | 0b3e46f | specs/0032 | keep |
| 432 | `Styles` | struct | core | 35cd499 | specs/0012 | keep |
| 433 | `Styles.num_fmts` | field | core | 35cd499 | specs/0012 | keep |
| 434 | `Styles.cell_xfs` | field | core | 35cd499 | specs/0012 | keep |
| 435 | `Styles.cell_style_xfs` | field | read | 0b3e46f | specs/0032 | keep |
| 436 | `Styles.cell_styles` | field | read | 0b3e46f | specs/0032 | keep |
| 437 | `Styles.indexed_colors` | field | read | 85bb33f | specs/0032 | keep |
| 438 | `NumFmt` | struct | core | 35cd499 | specs/0012 | keep |
| 439 | `NumFmt.id` | field | core | 35cd499 | specs/0012 | keep |
| 440 | `NumFmt.code` | field | core | 35cd499 | specs/0012 | keep |
| 441 | `Xf` | struct | core | 35cd499 | specs/0012 | keep |
| 442 | `Xf.num_fmt_id` | field | core | 35cd499 | specs/0012 | keep |
| 443 | `Xf.font_id` | field | core | 35cd499 | specs/0012 | keep |
| 444 | `Xf.fill_id` | field | core | 35cd499 | specs/0012 | keep |
| 445 | `Xf.border_id` | field | core | 35cd499 | specs/0012 | keep |
| 446 | `Xf.xf_id` | field | core | 35cd499 | specs/0012 | keep |
| 447 | `NumberFormat` | enum | core | 35cd499 | specs/0012 | keep |
| 448 | `Styles.fonts` | method | core | f14885f | specs/0017 | keep |
| 449 | `Styles.fills` | method | core | f14885f | specs/0017 | keep |
| 450 | `Styles.borders` | method | core | f14885f | specs/0017 | keep |
| 451 | `Styles.number_format` | method | core | 35cd499 | specs/0012 | keep |

### `crate::xml`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 452 | `BYTE_ORDER_MARK` | const | core | a9f2cad | specs/0009 | keep |
| 453 | `Token` | struct | core | a9f2cad | specs/0009 | keep |
| 454 | `Token.event` | field | core | a9f2cad | specs/0009 | keep |
| 455 | `Token.raw` | field | core | a9f2cad | specs/0009 | keep |
| 456 | `Stream` | struct | core | a9f2cad | specs/0009 | keep |
| 457 | `Stream.new` | method | core | a9f2cad | specs/0009 | keep |
| 458 | `Stream.bom` | method | core | a9f2cad | specs/0009 | keep |
| 459 | `Stream.next` | method | core | a9f2cad | specs/0009 | keep |
| 460 | `Scope` | struct | core | 5fc1b3d | specs/0009 | keep |
| 461 | `Scope.new` | method | core | 5fc1b3d | specs/0009 | keep |
| 462 | `Scope.start` | method | core | 5fc1b3d | specs/0009 | keep |
| 463 | `Scope.end` | method | core | 5fc1b3d | specs/0009 | keep |
| 464 | `Scope.resolve_element` | method | core | 5fc1b3d | specs/0009 | keep |
| 465 | `Scope.resolve_attribute` | method | core | 5fc1b3d | specs/0009 | keep |

### `crate::package::chart`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 466 | `ChartEntry` | struct | charts | c566a01 | specs/0034 | keep |
| 467 | `ChartEntry.part` | field | charts | c566a01 | specs/0034 | keep |
| 468 | `ChartEntry.anchor` | field | charts | c566a01 | specs/0034 | keep |
| 469 | `ChartEntry.chart` | field | charts | c566a01 | specs/0034 | keep |
| 470 | `ChartAnchor` | struct | charts | c566a01 | specs/0034 | keep |
| 471 | `ChartAnchor.kind` | field | charts | c566a01 | specs/0034 | keep |
| 472 | `ChartAnchor.from` | field | charts | c566a01 | specs/0034 | keep |
| 473 | `ChartAnchor.to` | field | charts | c566a01 | specs/0034 | keep |
| 474 | `ChartAnchor.edit_as` | field | charts | c566a01 | specs/0034 | keep |
| 475 | `Chart` | struct | charts | c566a01 | specs/0034 | keep |
| 476 | `Chart.title` | field | charts | c566a01 | specs/0034 | keep |
| 477 | `Chart.auto_title_deleted` | field | charts | c566a01 | specs/0034 | keep |
| 478 | `Chart.plots` | field | charts | c566a01 | specs/0034 | keep |
| 479 | `Chart.axes` | field | charts | 60ad190 | specs/0034 | keep |
| 480 | `Chart.legend` | field | charts | 60ad190 | specs/0034 | keep |
| 481 | `Chart.pivot_source` | field | charts | 60ad190 | specs/0034 | keep |
| 482 | `Chart.style` | field | charts | 60ad190 | specs/0034 | keep |
| 483 | `Plot` | struct | charts | c566a01 | specs/0034 | keep |
| 484 | `Plot.kind` | field | charts | c566a01 | specs/0034 | keep |
| 485 | `Plot.bar_direction` | field | charts | c566a01 | specs/0034 | keep |
| 486 | `Plot.grouping` | field | charts | c566a01 | specs/0034 | keep |
| 487 | `Plot.scatter_style` | field | charts | c566a01 | specs/0034 | keep |
| 488 | `Plot.vary_colors` | field | charts | c566a01 | specs/0034 | keep |
| 489 | `Plot.series` | field | charts | c566a01 | specs/0034 | keep |
| 490 | `Plot.axis_ids` | field | charts | c566a01 | specs/0034 | keep |
| 491 | `ChartAxis` | struct | charts | 60ad190 | specs/0034 | keep |
| 492 | `ChartAxis.id` | field | charts | 60ad190 | specs/0034 | keep |
| 493 | `ChartAxis.kind` | field | charts | 60ad190 | specs/0034 | keep |
| 494 | `ChartAxis.position` | field | charts | 60ad190 | specs/0034 | keep |
| 495 | `ChartAxis.crosses` | field | charts | 60ad190 | specs/0034 | keep |
| 496 | `ChartAxis.title` | field | charts | 60ad190 | specs/0034 | keep |
| 497 | `ChartAxis.number_format` | field | charts | 60ad190 | specs/0034 | keep |
| 498 | `ChartAxis.min` | field | charts | 60ad190 | specs/0034 | keep |
| 499 | `ChartAxis.max` | field | charts | 60ad190 | specs/0034 | keep |
| 500 | `ChartAxis.deleted` | field | charts | 60ad190 | specs/0034 | keep |
| 501 | `ChartAxis.major_gridlines` | field | charts | 60ad190 | specs/0034 | keep |
| 502 | `AxisKind` | enum | charts | 60ad190 | specs/0034 | keep |
| 503 | `AxisPosition` | enum | charts | 60ad190 | specs/0034 | keep |
| 504 | `LegendPosition` | enum | charts | 60ad190 | specs/0034 | keep |
| 505 | `ChartKind` | enum | charts | c566a01 | specs/0034 | keep |
| 506 | `BarDirection` | enum | charts | c566a01 | specs/0034 | keep |
| 507 | `Grouping` | enum | charts | c566a01 | specs/0034 | keep |
| 508 | `ScatterStyle` | enum | charts | c566a01 | specs/0034 | keep |
| 509 | `Series` | struct | charts | c566a01 | specs/0034 | keep |
| 510 | `Series.index` | field | charts | c566a01 | specs/0034 | keep |
| 511 | `Series.order` | field | charts | c566a01 | specs/0034 | keep |
| 512 | `Series.name` | field | charts | c566a01 | specs/0034 | keep |
| 513 | `Series.categories` | field | charts | c566a01 | specs/0034 | keep |
| 514 | `Series.values` | field | charts | c566a01 | specs/0034 | keep |
| 515 | `Series.sizes` | field | charts | c566a01 | specs/0034 | keep |
| 516 | `Series.fill` | field | charts | c566a01 | specs/0034 | keep |
| 517 | `Series.smooth` | field | charts | c566a01 | specs/0034 | keep |
| 518 | `Series.marker` | field | charts | c566a01 | specs/0034 | keep |
| 519 | `Series.labels` | field | charts | c566a01 | specs/0034 | keep |
| 520 | `SeriesName` | enum | charts | c566a01 | specs/0034 | keep |
| 521 | `Data` | enum | charts | c566a01 | specs/0034 | keep |
| 522 | `Package.charts` | method | charts | c566a01 | specs/0034 | keep |

### `crate::package::chart_edit`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 523 | `ChartSeriesEdit` | struct | charts | 075abca | specs/0041 | keep |
| 524 | `ChartSeriesEdit.name` | field | charts | 075abca | specs/0041 | keep |
| 525 | `ChartSeriesEdit.categories` | field | charts | 075abca | specs/0041 | keep |
| 526 | `ChartSeriesEdit.values` | field | charts | 075abca | specs/0041 | keep |
| 527 | `Package.set_chart_title` | method | charts | 075abca | specs/0041 | keep |
| 528 | `Package.set_chart_series` | method | charts | 075abca | specs/0041 | keep |

### `crate::package::chart_write`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 529 | `NewChart` | struct | charts | 4c36f05 | specs/0035 | keep |
| 530 | `NewChart.plot` | field | charts | 4c36f05 | specs/0035 | keep |
| 531 | `NewChart.title` | field | charts | 4c36f05 | specs/0035 | keep |
| 532 | `NewChart.series` | field | charts | 4c36f05 | specs/0035 | keep |
| 533 | `NewChart.category_axis_title` | field | charts | 4c36f05 | specs/0035 | keep |
| 534 | `NewChart.value_axis_title` | field | charts | 4c36f05 | specs/0035 | keep |
| 535 | `NewChart.category_axis_format` | field | charts | 784e3c0 | specs/0036 | keep |
| 536 | `NewChart.value_axis_format` | field | charts | 784e3c0 | specs/0036 | keep |
| 537 | `NewChart.legend` | field | charts | 4c36f05 | specs/0035 | keep |
| 538 | `NewChart.size` | field | charts | 4c36f05 | specs/0035 | keep |
| 539 | `NewChart.combined` | field | charts | a5823b7 | specs/0036 | keep |
| 540 | `NewChart.drop_lines` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 541 | `NewChart.high_low_lines` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 542 | `NewChart.up_down_bars` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 543 | `NewChart.category_axis` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 544 | `NewChart.value_axis` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 545 | `NewChart.title_text` | field | charts | 4d2cdc3 | specs/0041 | keep |
| 546 | `NewChart.axis_title_text` | field | charts | 4d2cdc3 | specs/0041 | keep |
| 547 | `NewChart.axis_label_text` | field | charts | 4d2cdc3 | specs/0041 | keep |
| 548 | `NewChart.legend_text` | field | charts | 4d2cdc3 | specs/0041 | keep |
| 549 | `NewChart.style` | field | charts | 075abca | specs/0041 | keep |
| 550 | `NewAxis` | struct | charts | 3c1f1f3 | specs/0040 | keep |
| 551 | `NewAxis.min` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 552 | `NewAxis.max` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 553 | `NewAxis.log_base` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 554 | `NewAxis.reversed` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 555 | `NewAxis.major_unit` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 556 | `NewAxis.minor_unit` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 557 | `NewAxis.tick_label_skip` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 558 | `NewAxis.tick_mark_skip` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 559 | `NewAxis.date` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 560 | `NewDateAxis` | struct | charts | 3c1f1f3 | specs/0040 | keep |
| 561 | `NewDateAxis.base_unit` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 562 | `NewDateAxis.major` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 563 | `NewDateAxis.minor` | field | charts | 3c1f1f3 | specs/0040 | keep |
| 564 | `TimeUnit` | enum | charts | 3c1f1f3 | specs/0040 | keep |
| 565 | `NewCombined` | struct | charts | a5823b7 | specs/0036 | keep |
| 566 | `NewCombined.plot` | field | charts | a5823b7 | specs/0036 | keep |
| 567 | `NewCombined.series` | field | charts | a5823b7 | specs/0036 | keep |
| 568 | `NewCombined.secondary_axis` | field | charts | a5823b7 | specs/0036 | keep |
| 569 | `NewCombined.secondary_axis_title` | field | charts | a5823b7 | specs/0036 | keep |
| 570 | `NewCombined.secondary_axis_format` | field | charts | a5823b7 | specs/0036 | keep |
| 571 | `NewPlot` | enum | charts | 4c36f05 | specs/0035 | keep |
| 572 | `OfPieKind` | enum | charts | 3affd4a | specs/0039 | keep |
| 573 | `OfPieSplit` | enum | charts | 3affd4a | specs/0039 | keep |
| 574 | `RadarStyle` | enum | charts | 01a5eba | specs/0036 | keep |
| 575 | `NewSeries` | struct | charts | 4c36f05 | specs/0035 | keep |
| 576 | `NewSeries.name` | field | charts | 4c36f05 | specs/0035 | keep |
| 577 | `NewSeries.categories` | field | charts | 4c36f05 | specs/0035 | keep |
| 578 | `NewSeries.values` | field | charts | 4c36f05 | specs/0035 | keep |
| 579 | `NewSeries.sizes` | field | charts | 01a5eba | specs/0036 | keep |
| 580 | `NewSeries.color` | field | charts | 784e3c0 | specs/0036 | keep |
| 581 | `NewSeries.labels` | field | charts | 784e3c0 | specs/0036 | keep |
| 582 | `NewSeries.marker` | field | charts | 1637495 | specs/0039 | keep |
| 583 | `NewSeries.line` | field | charts | 1637495 | specs/0039 | keep |
| 584 | `NewSeries.points` | field | charts | 1637495 | specs/0039 | keep |
| 585 | `NewSeries.trendline` | field | charts | 12f1314 | specs/0040 | keep |
| 586 | `NewSeries.error_bars` | field | charts | 12f1314 | specs/0040 | keep |
| 587 | `NewMarker` | struct | charts | 1637495 | specs/0039 | keep |
| 588 | `NewMarker.symbol` | field | charts | 1637495 | specs/0039 | keep |
| 589 | `NewMarker.size` | field | charts | 1637495 | specs/0039 | keep |
| 590 | `NewMarker.color` | field | charts | 1637495 | specs/0039 | keep |
| 591 | `MarkerSymbol` | enum | charts | 1637495 | specs/0039 | keep |
| 592 | `NewLine` | struct | charts | 1637495 | specs/0039 | keep |
| 593 | `NewLine.width` | field | charts | 1637495 | specs/0039 | keep |
| 594 | `NewLine.dash` | field | charts | 1637495 | specs/0039 | keep |
| 595 | `DashStyle` | enum | charts | 1637495 | specs/0039 | keep |
| 596 | `NewPoint` | struct | charts | 1637495 | specs/0039 | keep |
| 597 | `NewPoint.index` | field | charts | 1637495 | specs/0039 | keep |
| 598 | `NewPoint.color` | field | charts | 1637495 | specs/0039 | keep |
| 599 | `NewPoint.explosion` | field | charts | 1637495 | specs/0039 | keep |
| 600 | `NewPoint.marker` | field | charts | 1637495 | specs/0039 | keep |
| 601 | `NewTrendline` | struct | charts | 12f1314 | specs/0040 | keep |
| 602 | `NewTrendline.kind` | field | charts | 12f1314 | specs/0040 | keep |
| 603 | `NewTrendline.forward` | field | charts | 12f1314 | specs/0040 | keep |
| 604 | `NewTrendline.backward` | field | charts | 12f1314 | specs/0040 | keep |
| 605 | `NewTrendline.intercept` | field | charts | 12f1314 | specs/0040 | keep |
| 606 | `NewTrendline.show_equation` | field | charts | 12f1314 | specs/0040 | keep |
| 607 | `NewTrendline.show_r_squared` | field | charts | 12f1314 | specs/0040 | keep |
| 608 | `NewTrendline.name` | field | charts | 12f1314 | specs/0040 | keep |
| 609 | `TrendlineKind` | enum | charts | 12f1314 | specs/0040 | keep |
| 610 | `NewErrorBars` | struct | charts | 12f1314 | specs/0040 | keep |
| 611 | `NewErrorBars.direction` | field | charts | 12f1314 | specs/0040 | keep |
| 612 | `NewErrorBars.kind` | field | charts | 12f1314 | specs/0040 | keep |
| 613 | `NewErrorBars.value` | field | charts | 12f1314 | specs/0040 | keep |
| 614 | `NewErrorBars.end_cap` | field | charts | 12f1314 | specs/0040 | keep |
| 615 | `ErrorDirection` | enum | charts | 12f1314 | specs/0040 | keep |
| 616 | `ErrorBarKind` | enum | charts | 12f1314 | specs/0040 | keep |
| 617 | `ErrorValue` | enum | charts | 12f1314 | specs/0040 | keep |
| 618 | `NewLabels` | struct | charts | 784e3c0 | specs/0036 | keep |
| 619 | `NewLabels.value` | field | charts | 784e3c0 | specs/0036 | keep |
| 620 | `NewLabels.category` | field | charts | 784e3c0 | specs/0036 | keep |
| 621 | `NewLabels.series_name` | field | charts | 784e3c0 | specs/0036 | keep |
| 622 | `NewLabels.percent` | field | charts | 784e3c0 | specs/0036 | keep |
| 623 | `NewLabels.position` | field | charts | 784e3c0 | specs/0036 | keep |
| 624 | `NewLabels.number_format` | field | charts | 784e3c0 | specs/0036 | keep |
| 625 | `NewLabels.text` | field | charts | 4d2cdc3 | specs/0041 | keep |
| 626 | `NewText` | struct | charts | 4d2cdc3 | specs/0041 | keep |
| 627 | `NewText.size` | field | charts | 4d2cdc3 | specs/0041 | keep |
| 628 | `NewText.bold` | field | charts | 4d2cdc3 | specs/0041 | keep |
| 629 | `NewText.italic` | field | charts | 4d2cdc3 | specs/0041 | keep |
| 630 | `NewText.color` | field | charts | 4d2cdc3 | specs/0041 | keep |
| 631 | `NewText.font` | field | charts | 4d2cdc3 | specs/0041 | keep |
| 632 | `LabelPosition` | enum | charts | 784e3c0 | specs/0036 | keep |
| 633 | `NewSeriesName` | enum | charts | 4c36f05 | specs/0035 | keep |
| 634 | `Package.add_chart` | method | charts | 4c36f05 | specs/0035 | keep |
| 635 | `Package.remove_chart` | method | charts | 784e3c0 | specs/0036 | keep |
| 636 | `Package.add_chartsheet` | method | charts | 56b2937 | specs/0038 | keep |

### `crate::package::extlink`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 637 | `ExternalLink` | struct | read | cdffef7 | specs/0033 | keep |
| 638 | `ExternalLink.part` | field | read | cdffef7 | specs/0033 | keep |
| 639 | `ExternalLink.kind` | field | read | cdffef7 | specs/0033 | keep |
| 640 | `ExternalLinkKind` | enum | read | cdffef7 | specs/0033 | keep |
| 641 | `Package.external_links` | method | read | cdffef7 | specs/0033 | keep |

### `crate::package::pivot`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 642 | `PivotTable` | struct | read | ec9e58f | specs/0033 | keep |
| 643 | `PivotTable.part` | field | read | ec9e58f | specs/0033 | keep |
| 644 | `PivotTable.sheet` | field | read | ec9e58f | specs/0033 | keep |
| 645 | `PivotTable.name` | field | read | ec9e58f | specs/0033 | keep |
| 646 | `PivotTable.cache_id` | field | read | ec9e58f | specs/0033 | keep |
| 647 | `PivotTable.location` | field | read | ec9e58f | specs/0033 | keep |
| 648 | `PivotTable.first_header_row` | field | read | ec9e58f | specs/0033 | keep |
| 649 | `PivotTable.first_data_row` | field | read | ec9e58f | specs/0033 | keep |
| 650 | `PivotTable.first_data_col` | field | read | ec9e58f | specs/0033 | keep |
| 651 | `PivotTable.cache` | field | read | ec9e58f | specs/0033 | keep |
| 652 | `PivotTable.fields` | field | read | ec9e58f | specs/0033 | keep |
| 653 | `PivotTable.row_fields` | field | read | ec9e58f | specs/0033 | keep |
| 654 | `PivotTable.column_fields` | field | read | ec9e58f | specs/0033 | keep |
| 655 | `PivotTable.page_fields` | field | read | ec9e58f | specs/0033 | keep |
| 656 | `PivotTable.data_fields` | field | read | ec9e58f | specs/0033 | keep |
| 657 | `PivotField` | struct | read | ec9e58f | specs/0033 | keep |
| 658 | `PivotField.name` | field | read | ec9e58f | specs/0033 | keep |
| 659 | `PivotField.axis` | field | read | ec9e58f | specs/0033 | keep |
| 660 | `PivotField.data_field` | field | read | ec9e58f | specs/0033 | keep |
| 661 | `Axis` | enum | read | ec9e58f | specs/0033 | keep |
| 662 | `FieldRef` | enum | read | ec9e58f | specs/0033 | keep |
| 663 | `DataField` | struct | read | ec9e58f | specs/0033 | keep |
| 664 | `DataField.name` | field | read | ec9e58f | specs/0033 | keep |
| 665 | `DataField.field` | field | read | ec9e58f | specs/0033 | keep |
| 666 | `DataField.subtotal` | field | read | ec9e58f | specs/0033 | keep |
| 667 | `Subtotal` | enum | read | ec9e58f | specs/0033 | keep |
| 668 | `PivotCache` | struct | read | ec9e58f | specs/0033 | keep |
| 669 | `PivotCache.part` | field | read | ec9e58f | specs/0033 | keep |
| 670 | `PivotCache.source` | field | read | ec9e58f | specs/0033 | keep |
| 671 | `PivotCache.fields` | field | read | ec9e58f | specs/0033 | keep |
| 672 | `PivotCache.record_count` | field | read | ec9e58f | specs/0033 | keep |
| 673 | `PivotSource` | enum | read | ec9e58f | specs/0033 | keep |
| 674 | `Package.pivot_tables` | method | read | ec9e58f | specs/0033 | keep |

### `crate::package::sheet_read`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 675 | `ColumnFormat` | struct | read | bd02ce4 | specs/0027 | keep |
| 676 | `ColumnFormat.first` | field | read | bd02ce4 | specs/0027 | keep |
| 677 | `ColumnFormat.last` | field | read | bd02ce4 | specs/0027 | keep |
| 678 | `ColumnFormat.width` | field | read | bd02ce4 | specs/0027 | keep |
| 679 | `ColumnFormat.custom_width` | field | read | bd02ce4 | specs/0027 | keep |
| 680 | `ColumnFormat.hidden` | field | read | bd02ce4 | specs/0027 | keep |
| 681 | `ColumnFormat.style` | field | read | bd02ce4 | specs/0027 | keep |
| 682 | `ColumnFormat.outline_level` | field | read | bd02ce4 | specs/0027 | keep |
| 683 | `ColumnFormat.collapsed` | field | read | bd02ce4 | specs/0027 | keep |
| 684 | `RowFormat` | struct | read | bd02ce4 | specs/0027 | keep |
| 685 | `RowFormat.row` | field | read | bd02ce4 | specs/0027 | keep |
| 686 | `RowFormat.height` | field | read | bd02ce4 | specs/0027 | keep |
| 687 | `RowFormat.custom_height` | field | read | bd02ce4 | specs/0027 | keep |
| 688 | `RowFormat.hidden` | field | read | bd02ce4 | specs/0027 | keep |
| 689 | `RowFormat.style` | field | read | bd02ce4 | specs/0027 | keep |
| 690 | `RowFormat.custom_format` | field | read | bd02ce4 | specs/0027 | keep |
| 691 | `RowFormat.outline_level` | field | read | bd02ce4 | specs/0027 | keep |
| 692 | `RowFormat.collapsed` | field | read | bd02ce4 | specs/0027 | keep |
| 693 | `HyperlinkTarget` | enum | read | cce4285 | specs/0030 | keep |
| 694 | `HyperlinkEntry` | struct | read | bd02ce4 | specs/0027 | keep |
| 695 | `HyperlinkEntry.cell` | field | read | bd02ce4 | specs/0027 | keep |
| 696 | `HyperlinkEntry.target` | field | read | bd02ce4 | specs/0027 | keep |
| 697 | `HyperlinkEntry.display` | field | read | bd02ce4 | specs/0027 | keep |
| 698 | `HyperlinkEntry.tooltip` | field | read | bd02ce4 | specs/0027 | keep |
| 699 | `SheetSettings` | struct | read | bd02ce4 | specs/0027 | keep |
| 700 | `SheetSettings.tab_color` | field | read | bd02ce4 | specs/0027 | keep |
| 701 | `SheetSettings.outline_summary` | field | read | bd02ce4 | specs/0027 | keep |
| 702 | `SheetSettings.fit_to_page` | field | read | bd02ce4 | specs/0027 | keep |
| 703 | `SheetSettings.view` | field | read | bd02ce4 | specs/0027 | keep |
| 704 | `SheetSettings.freeze` | field | read | bd02ce4 | specs/0027 | keep |
| 705 | `SheetSettings.print_options` | field | read | bd02ce4 | specs/0027 | keep |
| 706 | `SheetSettings.margins` | field | read | bd02ce4 | specs/0027 | keep |
| 707 | `SheetSettings.page_setup` | field | read | bd02ce4 | specs/0027 | keep |
| 708 | `SheetSettings.header_footer` | field | read | bd02ce4 | specs/0027 | keep |
| 709 | `SheetSettings.auto_filter` | field | read | bd02ce4 | specs/0027 | keep |
| 710 | `SheetSettings.merges` | field | read | bd02ce4 | specs/0027 | keep |
| 711 | `SheetSettings.hyperlinks` | field | read | bd02ce4 | specs/0027 | keep |
| 712 | `SheetSettings.protection` | field | read | bd02ce4 | specs/0027 | keep |
| 713 | `SheetSettings.outline_max` | field | read | bd02ce4 | specs/0027 | keep |
| 714 | `Rows` | struct | read | bd02ce4 | specs/0027 | keep |
| 715 | `Package.columns` | method | read | bd02ce4 | specs/0027 | keep |
| 716 | `Package.rows` | method | read | bd02ce4 | specs/0027 | keep |
| 717 | `Package.sheet_settings` | method | read | bd02ce4 | specs/0027 | keep |

### `crate::package::survey`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 718 | `PartKinds` | struct | read | 338e91c | specs/0033 | keep |
| 719 | `PartKinds.worksheets` | field | read | 338e91c | specs/0033 | keep |
| 720 | `PartKinds.chartsheets` | field | read | 338e91c | specs/0033 | keep |
| 721 | `PartKinds.dialogsheets` | field | read | 338e91c | specs/0033 | keep |
| 722 | `PartKinds.macrosheets` | field | read | 338e91c | specs/0033 | keep |
| 723 | `PartKinds.charts` | field | read | 338e91c | specs/0033 | keep |
| 724 | `PartKinds.chartex` | field | read | 338e91c | specs/0033 | keep |
| 725 | `PartKinds.drawings` | field | read | 338e91c | specs/0033 | keep |
| 726 | `PartKinds.images` | field | read | 338e91c | specs/0033 | keep |
| 727 | `PartKinds.tables` | field | read | 338e91c | specs/0033 | keep |
| 728 | `PartKinds.pivot_tables` | field | read | 338e91c | specs/0033 | keep |
| 729 | `PartKinds.pivot_caches` | field | read | 338e91c | specs/0033 | keep |
| 730 | `PartKinds.pivot_cache_records` | field | read | 338e91c | specs/0033 | keep |
| 731 | `PartKinds.external_links` | field | read | 338e91c | specs/0033 | keep |
| 732 | `PartKinds.vba_projects` | field | read | 338e91c | specs/0033 | keep |
| 733 | `PartKinds.slicers` | field | read | 338e91c | specs/0033 | keep |
| 734 | `PartKinds.slicer_caches` | field | read | 338e91c | specs/0033 | keep |
| 735 | `PartKinds.timelines` | field | read | 338e91c | specs/0033 | keep |
| 736 | `PartKinds.timeline_caches` | field | read | 338e91c | specs/0033 | keep |
| 737 | `PartKinds.comments` | field | read | 338e91c | specs/0033 | keep |
| 738 | `PartKinds.threaded_comments` | field | read | 338e91c | specs/0033 | keep |
| 739 | `PartKinds.persons` | field | read | 338e91c | specs/0033 | keep |
| 740 | `PartKinds.custom_xml` | field | read | 338e91c | specs/0033 | keep |
| 741 | `PartKinds.connections` | field | read | 338e91c | specs/0033 | keep |
| 742 | `PartKinds.query_tables` | field | read | 338e91c | specs/0033 | keep |
| 743 | `PartKinds.custom_properties` | field | read | 338e91c | specs/0033 | keep |
| 744 | `PartKinds.printer_settings` | field | read | 338e91c | specs/0033 | keep |
| 745 | `PartKinds.calc_chains` | field | read | 338e91c | specs/0033 | keep |
| 746 | `PartKinds.other_typed` | field | read | 338e91c | specs/0033 | keep |
| 747 | `PartKinds.untyped` | field | read | 338e91c | specs/0033 | keep |
| 748 | `SheetParts` | struct | read | 338e91c | specs/0033 | keep |
| 749 | `SheetParts.drawings` | field | read | 338e91c | specs/0033 | keep |
| 750 | `SheetParts.charts` | field | read | 338e91c | specs/0033 | keep |
| 751 | `SheetParts.images` | field | read | 338e91c | specs/0033 | keep |
| 752 | `SheetParts.comments` | field | read | 338e91c | specs/0033 | keep |
| 753 | `SheetParts.vml_drawings` | field | read | 338e91c | specs/0033 | keep |
| 754 | `SheetParts.tables` | field | read | 338e91c | specs/0033 | keep |
| 755 | `SheetParts.pivot_tables` | field | read | 338e91c | specs/0033 | keep |
| 756 | `SheetParts.slicers` | field | read | 338e91c | specs/0033 | keep |
| 757 | `SheetParts.timelines` | field | read | 338e91c | specs/0033 | keep |
| 758 | `SheetParts.printer_settings` | field | read | 338e91c | specs/0033 | keep |
| 759 | `SheetParts.external_hyperlinks` | field | read | 338e91c | specs/0033 | keep |
| 760 | `SheetParts.sparkline_groups` | field | read | 338e91c | specs/0033 | keep |
| 761 | `Package.part_kinds` | method | read | 338e91c | specs/0033 | keep |
| 762 | `Package.sheet_parts` | method | read | 338e91c | specs/0033 | keep |

### `crate::styles::colors`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 763 | `ResolvedColor` | struct | read | 85bb33f | specs/0032 | keep |
| 764 | `ResolvedColor.rgb` | field | read | 85bb33f | specs/0032 | keep |
| 765 | `ResolvedColor.tint` | field | read | 85bb33f | specs/0032 | keep |
| 766 | `ResolvedColor.tinted` | method | read | 85bb33f | specs/0032 | keep |

### `crate::styles::elements`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 767 | `Style` | struct | core | f14885f | specs/0017 | keep |
| 768 | `Style.number_format` | field | core | f14885f | specs/0017 | keep |
| 769 | `Style.font` | field | core | f14885f | specs/0017 | keep |
| 770 | `Style.fill` | field | core | f14885f | specs/0017 | keep |
| 771 | `Style.border` | field | core | f14885f | specs/0017 | keep |
| 772 | `Style.alignment` | field | core | 4bd46a1 | specs/0017 | keep |
| 773 | `Style.protection` | field | core | 4bd46a1 | specs/0017 | keep |
| 774 | `Font` | struct | core | f14885f | specs/0017 | keep |
| 775 | `Font.name` | field | core | f14885f | specs/0017 | keep |
| 776 | `Font.size` | field | core | f14885f | specs/0017 | keep |
| 777 | `Font.bold` | field | core | f14885f | specs/0017 | keep |
| 778 | `Font.italic` | field | core | f14885f | specs/0017 | keep |
| 779 | `Font.strike` | field | core | f14885f | specs/0017 | keep |
| 780 | `Font.underline` | field | core | f14885f | specs/0017 | keep |
| 781 | `Font.vertical_align` | field | core | f14885f | specs/0017 | keep |
| 782 | `Font.color` | field | core | f14885f | specs/0017 | keep |
| 783 | `Font.family` | field | core | f14885f | specs/0017 | keep |
| 784 | `Font.scheme` | field | core | f14885f | specs/0017 | keep |
| 785 | `Underline` | enum | core | f14885f | specs/0017 | keep |
| 786 | `VerticalAlign` | enum | core | f14885f | specs/0017 | keep |
| 787 | `Scheme` | enum | core | f14885f | specs/0017 | keep |
| 788 | `Color` | enum | core | f14885f | specs/0017 | keep |
| 789 | `Fill` | struct | core | f14885f | specs/0017 | keep |
| 790 | `Fill.pattern` | field | core | f14885f | specs/0017 | keep |
| 791 | `Fill.foreground` | field | core | f14885f | specs/0017 | keep |
| 792 | `Fill.background` | field | core | f14885f | specs/0017 | keep |
| 793 | `Pattern` | enum | core | f14885f | specs/0017 | keep |
| 794 | `Border` | struct | core | f14885f | specs/0017 | keep |
| 795 | `Border.left` | field | core | f14885f | specs/0017 | keep |
| 796 | `Border.right` | field | core | f14885f | specs/0017 | keep |
| 797 | `Border.top` | field | core | f14885f | specs/0017 | keep |
| 798 | `Border.bottom` | field | core | f14885f | specs/0017 | keep |
| 799 | `Border.diagonal` | field | core | f14885f | specs/0017 | keep |
| 800 | `Border.diagonal_up` | field | core | f14885f | specs/0017 | keep |
| 801 | `Border.diagonal_down` | field | core | f14885f | specs/0017 | keep |
| 802 | `Side` | struct | core | f14885f | specs/0017 | keep |
| 803 | `Side.style` | field | core | f14885f | specs/0017 | keep |
| 804 | `Side.color` | field | core | f14885f | specs/0017 | keep |
| 805 | `BorderStyle` | enum | core | f14885f | specs/0017 | keep |
| 806 | `Alignment` | struct | core | f14885f | specs/0017 | keep |
| 807 | `Alignment.horizontal` | field | core | f14885f | specs/0017 | keep |
| 808 | `Alignment.vertical` | field | core | f14885f | specs/0017 | keep |
| 809 | `Alignment.wrap_text` | field | core | f14885f | specs/0017 | keep |
| 810 | `Alignment.shrink_to_fit` | field | core | f14885f | specs/0017 | keep |
| 811 | `Alignment.indent` | field | core | f14885f | specs/0017 | keep |
| 812 | `Alignment.text_rotation` | field | core | f14885f | specs/0017 | keep |
| 813 | `Horizontal` | enum | core | f14885f | specs/0017 | keep |
| 814 | `Vertical` | enum | core | f14885f | specs/0017 | keep |
| 815 | `Protection` | struct | core | f14885f | specs/0017 | keep |
| 816 | `Protection.locked` | field | core | f14885f | specs/0017 | keep |
| 817 | `Protection.hidden` | field | core | f14885f | specs/0017 | keep |

### `crate::styles::read`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 818 | `CellStyle` | struct | read | c019afa | specs/0027 | keep |
| 819 | `CellStyle.number_format` | field | read | c019afa | specs/0027 | keep |
| 820 | `CellStyle.font` | field | read | c019afa | specs/0027 | keep |
| 821 | `CellStyle.fill` | field | read | c019afa | specs/0027 | keep |
| 822 | `CellStyle.border` | field | read | c019afa | specs/0027 | keep |
| 823 | `CellStyle.alignment` | field | read | c019afa | specs/0027 | keep |
| 824 | `CellStyle.protection` | field | read | c019afa | specs/0027 | keep |
| 825 | `NamedStyle` | struct | read | 0b3e46f | specs/0032 | keep |
| 826 | `NamedStyle.name` | field | read | 0b3e46f | specs/0032 | keep |
| 827 | `NamedStyle.xf_id` | field | read | 0b3e46f | specs/0032 | keep |
| 828 | `NamedStyle.builtin_id` | field | read | 0b3e46f | specs/0032 | keep |
| 829 | `NamedStyle.custom_builtin` | field | read | 0b3e46f | specs/0032 | keep |
| 830 | `NamedStyle.hidden` | field | read | 0b3e46f | specs/0032 | keep |
| 831 | `NamedStyle.outline_level` | field | read | 0b3e46f | specs/0032 | keep |
| 832 | `Styles.typed_fonts` | method | read | 85bb33f | specs/0032 | keep |
| 833 | `Styles.typed_fills` | method | read | 85bb33f | specs/0032 | keep |
| 834 | `Styles.typed_borders` | method | read | 85bb33f | specs/0032 | keep |

## Counts per verdict

- keep: 834
- rename before publication: 0
- hide: 0
- widen or narrow: 0
- in all: 834
