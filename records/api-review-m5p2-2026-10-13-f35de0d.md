# API review rerun, Milestone 5 part 2 P3 (2026-10-13, f35de0d)

The API listing rerun as Milestone 5 part 1 P3 did, after M5 part 2 added the survey (part kinds and sheet parts), the pivot tables and the external links under the feature `read`, and gated seven relationship type constants with `read` as well (`specs/0033-read-breadth-survey.md`, "P3"). M5 part 1 P3's record is `records/api-review-m5p1-2026-10-12-6f51899.md`.

- Listing: every `pub` item of `crates/emend` reachable from the crate root, as `records/logs/m5p2-p3/test.log` (section T1) prints it from the phase commit's sources with part 7 P3's listing program; its file `listing.tsv` sha256 2c7a1f91cbe3d21ddfad96084eadf3d9e098764c71d0b7d75e8214c4f0cbd0ad, `files.tsv` sha256 0f8dd21861bd91c4da772b2cfe55c9cdbe33482775f3fdfd188efb2f040d7d0b.
- Formatter: this record is assembled by `make_review.py`, generated from M5 part 1 P3's formatter, sha256 53602ff7bb7094f9fac9b2a0ad54d6e3a6f7741fc3ba3ba7b76a4a29e2ddc115 (printed in test.log, section R2), its table of non-keep verdicts empty and its spec map extended to M5 part 2.
- Items: 660, by kind: field 326, method 168, struct 70, enum 36, const 30, use 19, mod 6, fn 4, type 1.
- The doc build (test.log section T2): `cargo doc` with every feature and `-D warnings`, exit 0, no warning line; its index `all.html` holds 146 link lines, of which 5 are the page's section anchors, leaving 141 documented module-level items, against the listing's 141 reachable module-level types, functions, constants and aliases.
- Columns: as part 7's record: `item` is the item's name, `Container.name` for a method or field, `use ...` for a re-export; `feature` is the cfg in force at the item (`core` for none); `added by` is the commit the listing finds introducing the item's declaration (a heuristic the listing program states); `spec` is that commit's part's spec.
- Verdicts: every item keep.

## Rows that differ from M5 part 1 P3's listing

Compared by key (module, kind, name, container, cfg, added by): 7 of M5 part 1 P3's rows are not in this listing and 100 of this listing's rows, the new items, are not in M5 part 1 P3's; every other row is equal.

M5 part 1 P3's rows not here:

- `COMMENTS` const in `crate::package`, feature any(comments, copy), added by 96dde75
- `DRAWING` const in `crate::package`, feature images, added by b65ea6d
- `HYPERLINK` const in `crate::package`, feature any(metadata, copy), added by a791e2c
- `IMAGE` const in `crate::package`, feature images, added by b65ea6d
- `PIVOT_TABLE` const in `crate::package`, feature any(rowcol, tables), added by 7578ee0
- `TABLE` const in `crate::package`, feature any( rowcol, tables, all(sheetprops, metadata) ), added by 7578ee0
- `VML_DRAWING` const in `crate::package`, feature comments, added by 96dde75

This listing's rows not in M5 part 1 P3's (100 listed: 7 cfg-widened constants, 93 new):

The 7 cfg-widened constants are M5 part 1 P3's rows not here, each with its cfg widened to include `read` by M5 part 2 P1; the other 93 rows are the items M5 part 2 adds.

`CHART`'s added-by commit, 5fc1b3d, is the listing heuristic's misattribution: M5 part 2 adds `CHART` (`specs/0033-read-breadth-survey.md`, "Design"), and the heuristic dates its declaration to 5fc1b3d, whose part's spec the `spec` column then names.

- `CHART` const in `crate::package`, feature read, added by 5fc1b3d
- `COMMENTS` const in `crate::package`, feature any(comments, copy, read), added by 96dde75
- `DRAWING` const in `crate::package`, feature any(images, read), added by b65ea6d
- `EXTERNAL_LINK` const in `crate::package`, feature read, added by 338e91c
- `EXTERNAL_LINK_PATH` const in `crate::package`, feature read, added by 338e91c
- `HYPERLINK` const in `crate::package`, feature any(metadata, copy, read), added by a791e2c
- `IMAGE` const in `crate::package`, feature any(images, read), added by b65ea6d
- `PIVOT_CACHE_DEFINITION` const in `crate::package`, feature read, added by 338e91c
- `PIVOT_TABLE` const in `crate::package`, feature any(rowcol, tables, read), added by 7578ee0
- `PRINTER_SETTINGS` const in `crate::package`, feature read, added by 338e91c
- `SLICER` const in `crate::package`, feature read, added by 338e91c
- `TABLE` const in `crate::package`, feature any( rowcol, tables, read, all(sheetprops, metadata) ), added by 7578ee0
- `TIMELINE` const in `crate::package`, feature read, added by 338e91c
- `VML_DRAWING` const in `crate::package`, feature any(comments, read), added by 96dde75
- `use extlink::{ExternalLink, ExternalLinkKind}` use in `crate::package`, feature read, added by cdffef7
- `use pivot::{ Axis, DataField, FieldRef, PivotCache, PivotField, PivotSource, PivotTable, Subtotal, }` use in `crate::package`, feature read, added by ec9e58f
- `use survey::{PartKinds, SheetParts}` use in `crate::package`, feature read, added by 338e91c
- `ExternalLinkKind` enum in `crate::package::extlink`, feature read, added by cdffef7
- `ExternalLink.kind` field in `crate::package::extlink`, feature read, added by cdffef7
- `ExternalLink.part` field in `crate::package::extlink`, feature read, added by cdffef7
- `Package.external_links` method in `crate::package::extlink`, feature read, added by cdffef7
- `ExternalLink` struct in `crate::package::extlink`, feature read, added by cdffef7
- `Axis` enum in `crate::package::pivot`, feature read, added by ec9e58f
- `FieldRef` enum in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotSource` enum in `crate::package::pivot`, feature read, added by ec9e58f
- `Subtotal` enum in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotField.axis` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.cache` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.cache_id` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.column_fields` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotField.data_field` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.data_fields` field in `crate::package::pivot`, feature read, added by ec9e58f
- `DataField.field` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotCache.fields` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.fields` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.first_data_col` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.first_data_row` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.first_header_row` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.location` field in `crate::package::pivot`, feature read, added by ec9e58f
- `DataField.name` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotField.name` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.name` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.page_fields` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotCache.part` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.part` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotCache.record_count` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.row_fields` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable.sheet` field in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotCache.source` field in `crate::package::pivot`, feature read, added by ec9e58f
- `DataField.subtotal` field in `crate::package::pivot`, feature read, added by ec9e58f
- `Package.pivot_tables` method in `crate::package::pivot`, feature read, added by ec9e58f
- `DataField` struct in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotCache` struct in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotField` struct in `crate::package::pivot`, feature read, added by ec9e58f
- `PivotTable` struct in `crate::package::pivot`, feature read, added by ec9e58f
- `PartKinds.calc_chains` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.chartex` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.charts` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.charts` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.chartsheets` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.comments` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.comments` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.connections` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.custom_properties` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.custom_xml` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.dialogsheets` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.drawings` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.drawings` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.external_hyperlinks` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.external_links` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.images` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.images` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.macrosheets` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.other_typed` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.persons` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.pivot_cache_records` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.pivot_caches` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.pivot_tables` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.pivot_tables` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.printer_settings` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.printer_settings` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.query_tables` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.slicer_caches` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.slicers` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.slicers` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.sparkline_groups` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.tables` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.tables` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.threaded_comments` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.timeline_caches` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.timelines` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.timelines` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.untyped` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.vba_projects` field in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts.vml_drawings` field in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds.worksheets` field in `crate::package::survey`, feature read, added by 338e91c
- `Package.part_kinds` method in `crate::package::survey`, feature read, added by 338e91c
- `Package.sheet_parts` method in `crate::package::survey`, feature read, added by 338e91c
- `PartKinds` struct in `crate::package::survey`, feature read, added by 338e91c
- `SheetParts` struct in `crate::package::survey`, feature read, added by 338e91c

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
| 59 | `Area.parse` | method | core | 6c27167 | specs/0010 | keep |
| 60 | `Area.a1` | method | core | 6c27167 | specs/0010 | keep |
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
| 81 | `Run.text` | field | richtext | 6c27167 | specs/0010 | keep |
| 82 | `Run.font` | field | richtext | 1174bf1 | specs/0024 | keep |
| 83 | `RichText` | struct | richtext | 1174bf1 | specs/0024 | keep |
| 84 | `RichText.runs` | field | richtext | 1174bf1 | specs/0024 | keep |
| 85 | `RichText.text` | method | richtext | 1174bf1 | specs/0024 | keep |
| 86 | `SharedStringItem` | struct | read | c68f350 | specs/0030 | keep |
| 87 | `SharedStringItem.text` | field | read | 6c27167 | specs/0010 | keep |
| 88 | `SharedStringItem.runs` | field | read & richtext | 1174bf1 | specs/0024 | keep |
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
| 102 | `use extlink::{ExternalLink, ExternalLinkKind}` | use | read | cdffef7 | specs/0033 | keep |
| 103 | `use pivot::{ Axis, DataField, FieldRef, PivotCache, PivotField, PivotSource, PivotTable, Subtotal, }` | use | read | ec9e58f | specs/0033 | keep |
| 104 | `use sheet_read::{ ColumnFormat, HyperlinkEntry, HyperlinkTarget, RowFormat, Rows, SheetSettings, }` | use | read | bd02ce4 | specs/0027 | keep |
| 105 | `use survey::{PartKinds, SheetParts}` | use | read | 338e91c | specs/0033 | keep |
| 106 | `CONTENT_TYPES_PART` | const | core | 5fc1b3d | specs/0009 | keep |
| 107 | `CONTENT_TYPES_NS` | const | core | 5fc1b3d | specs/0009 | keep |
| 108 | `RELATIONSHIPS_NS` | const | core | 5fc1b3d | specs/0009 | keep |
| 109 | `MAIN` | const | core | 5fc1b3d | specs/0009 | keep |
| 110 | `RELATIONSHIPS_ATTRIBUTE` | const | core | 5fc1b3d | specs/0009 | keep |
| 111 | `OFFICE_DOCUMENT` | const | core | 5fc1b3d | specs/0009 | keep |
| 112 | `WORKSHEET` | const | core | 5fc1b3d | specs/0009 | keep |
| 113 | `CHARTSHEET` | const | core | 5fc1b3d | specs/0009 | keep |
| 114 | `DIALOGSHEET` | const | core | 5fc1b3d | specs/0009 | keep |
| 115 | `SHARED_STRINGS` | const | core | 5fc1b3d | specs/0009 | keep |
| 116 | `STYLES` | const | core | 5fc1b3d | specs/0009 | keep |
| 117 | `CALC_CHAIN` | const | core | 5fc1b3d | specs/0009 | keep |
| 118 | `THEME` | const | core | 5fc1b3d | specs/0009 | keep |
| 119 | `TABLE` | const | any( rowcol, tables, read, all(sheetprops, metadata) ) | 7578ee0 | specs/0018 | keep |
| 120 | `PIVOT_TABLE` | const | any(rowcol, tables, read) | 7578ee0 | specs/0018 | keep |
| 121 | `COMMENTS` | const | any(comments, copy, read) | 96dde75 | specs/0022 | keep |
| 122 | `VML_DRAWING` | const | any(comments, read) | 96dde75 | specs/0022 | keep |
| 123 | `THREADED_COMMENTS` | const | comments | 96dde75 | specs/0022 | keep |
| 124 | `DRAWING` | const | any(images, read) | b65ea6d | specs/0022 | keep |
| 125 | `IMAGE` | const | any(images, read) | b65ea6d | specs/0022 | keep |
| 126 | `HYPERLINK` | const | any(metadata, copy, read) | a791e2c | specs/0019 | keep |
| 127 | `EXTERNAL_LINK` | const | read | 338e91c | specs/0033 | keep |
| 128 | `EXTERNAL_LINK_PATH` | const | read | 338e91c | specs/0033 | keep |
| 129 | `PIVOT_CACHE_DEFINITION` | const | read | 338e91c | specs/0033 | keep |
| 130 | `CHART` | const | read | 5fc1b3d | specs/0009 | keep |
| 131 | `SLICER` | const | read | 338e91c | specs/0033 | keep |
| 132 | `TIMELINE` | const | read | 338e91c | specs/0033 | keep |
| 133 | `PRINTER_SETTINGS` | const | read | 338e91c | specs/0033 | keep |
| 134 | `CORE_PROPERTIES` | const | metadata | 51acd32 | specs/0019 | keep |
| 135 | `Property` | enum | metadata | 51acd32 | specs/0019 | keep |
| 136 | `Properties` | struct | metadata | 51acd32 | specs/0019 | keep |
| 137 | `Properties.get` | method | metadata | 51acd32 | specs/0019 | keep |
| 138 | `SheetProtection` | struct | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 139 | `SheetProtection.sheet` | field | any(metadata, read) | 5fc1b3d | specs/0009 | keep |
| 140 | `SheetProtection.objects` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 141 | `SheetProtection.scenarios` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 142 | `SheetProtection.format_cells` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 143 | `SheetProtection.format_columns` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 144 | `SheetProtection.format_rows` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 145 | `SheetProtection.insert_columns` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 146 | `SheetProtection.insert_rows` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 147 | `SheetProtection.insert_hyperlinks` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 148 | `SheetProtection.delete_columns` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 149 | `SheetProtection.delete_rows` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 150 | `SheetProtection.select_locked_cells` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 151 | `SheetProtection.sort` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 152 | `SheetProtection.auto_filter` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 153 | `SheetProtection.pivot_tables` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 154 | `SheetProtection.select_unlocked_cells` | field | any(metadata, read) | 51acd32 | specs/0019 | keep |
| 155 | `legacy_password_hash` | fn | metadata | 51acd32 | specs/0019 | keep |
| 156 | `ContentTypeDefault` | struct | core | ff0a480 | specs/0030 | keep |
| 157 | `ContentTypeDefault.extension` | field | core | 5fc1b3d | specs/0009 | keep |
| 158 | `ContentTypeDefault.content_type` | field | core | 5fc1b3d | specs/0009 | keep |
| 159 | `Override` | struct | core | 5fc1b3d | specs/0009 | keep |
| 160 | `Override.part_name` | field | core | 5fc1b3d | specs/0009 | keep |
| 161 | `Override.content_type` | field | core | 5fc1b3d | specs/0009 | keep |
| 162 | `ContentTypes` | struct | core | 5fc1b3d | specs/0009 | keep |
| 163 | `ContentTypes.defaults` | method | core | 5fc1b3d | specs/0009 | keep |
| 164 | `ContentTypes.overrides` | method | core | 5fc1b3d | specs/0009 | keep |
| 165 | `ContentTypes.of` | method | core | 5fc1b3d | specs/0009 | keep |
| 166 | `Relationship` | struct | core | 5fc1b3d | specs/0009 | keep |
| 167 | `Relationship.id` | field | core | 5fc1b3d | specs/0009 | keep |
| 168 | `Relationship.kind` | field | core | 5fc1b3d | specs/0009 | keep |
| 169 | `Relationship.target` | field | core | 5fc1b3d | specs/0009 | keep |
| 170 | `Relationship.external` | field | core | 5fc1b3d | specs/0009 | keep |
| 171 | `Relationships` | struct | core | 5fc1b3d | specs/0009 | keep |
| 172 | `Relationships.source` | method | core | 5fc1b3d | specs/0009 | keep |
| 173 | `Relationships.all` | method | core | 5fc1b3d | specs/0009 | keep |
| 174 | `Relationships.by_id` | method | core | 5fc1b3d | specs/0009 | keep |
| 175 | `Relationships.of_kind` | method | core | 5fc1b3d | specs/0009 | keep |
| 176 | `Relationships.first_of` | method | core | 5fc1b3d | specs/0009 | keep |
| 177 | `Relationships.target_part` | method | core | 5fc1b3d | specs/0009 | keep |
| 178 | `SheetKind` | enum | core | 5fc1b3d | specs/0009 | keep |
| 179 | `Sheet` | struct | core | 5fc1b3d | specs/0009 | keep |
| 180 | `Sheet.name` | field | core | 5fc1b3d | specs/0009 | keep |
| 181 | `Sheet.sheet_id` | field | core | 5fc1b3d | specs/0009 | keep |
| 182 | `Sheet.rel_id` | field | core | 5fc1b3d | specs/0009 | keep |
| 183 | `Sheet.state` | field | core | 5fc1b3d | specs/0009 | keep |
| 184 | `Sheet.part` | field | core | 5fc1b3d | specs/0009 | keep |
| 185 | `Sheet.kind` | field | core | 5fc1b3d | specs/0009 | keep |
| 186 | `Workbook` | struct | core | 5fc1b3d | specs/0009 | keep |
| 187 | `Workbook.part` | field | core | 5fc1b3d | specs/0009 | keep |
| 188 | `Workbook.sheets` | field | core | 5fc1b3d | specs/0009 | keep |
| 189 | `Workbook.shared_strings` | field | core | 5fc1b3d | specs/0009 | keep |
| 190 | `Workbook.styles` | field | core | 5fc1b3d | specs/0009 | keep |
| 191 | `Workbook.calc_chain` | field | core | 5fc1b3d | specs/0009 | keep |
| 192 | `Workbook.theme` | field | core | 5fc1b3d | specs/0009 | keep |
| 193 | `Workbook.date1904` | field | core | 35cd499 | specs/0012 | keep |
| 194 | `Workbook.full_calc_on_load` | field | core | 77c3016 | specs/0013 | keep |
| 195 | `Workbook.sheet` | method | core | 5fc1b3d | specs/0009 | keep |
| 196 | `Workbook.defined_names` | method | metadata | 2634ecd | specs/0019 | keep |
| 197 | `DefinedName` | struct | metadata | 2634ecd | specs/0019 | keep |
| 198 | `DefinedName.name` | field | metadata | 5fc1b3d | specs/0009 | keep |
| 199 | `DefinedName.text` | field | metadata | 2634ecd | specs/0019 | keep |
| 200 | `DefinedName.sheet` | field | metadata | 5fc1b3d | specs/0009 | keep |
| 201 | `DefinedName.hidden` | field | metadata | 2634ecd | specs/0019 | keep |
| 202 | `Package` | struct | core | 5fc1b3d | specs/0009 | keep |
| 203 | `LinkTarget` | enum | metadata | a791e2c | specs/0019 | keep |
| 204 | `ValidationKind` | enum | rules | 2ed9070 | specs/0020 | keep |
| 205 | `Operator` | enum | rules | 2ed9070 | specs/0020 | keep |
| 206 | `ErrorStyle` | enum | rules | 2ed9070 | specs/0020 | keep |
| 207 | `Validation` | struct | rules | 2ed9070 | specs/0020 | keep |
| 208 | `Validation.kind` | field | rules | 5fc1b3d | specs/0009 | keep |
| 209 | `Validation.operator` | field | rules | 2ed9070 | specs/0020 | keep |
| 210 | `Validation.formula1` | field | rules | 2ed9070 | specs/0020 | keep |
| 211 | `Validation.formula2` | field | rules | 2ed9070 | specs/0020 | keep |
| 212 | `Validation.allow_blank` | field | rules | 2ed9070 | specs/0020 | keep |
| 213 | `Validation.hide_drop_down` | field | rules | 2ed9070 | specs/0020 | keep |
| 214 | `Validation.input` | field | rules | 2ed9070 | specs/0020 | keep |
| 215 | `Validation.error` | field | rules | 2ed9070 | specs/0020 | keep |
| 216 | `ValidationEntry` | struct | rules | 2ed9070 | specs/0020 | keep |
| 217 | `ValidationEntry.areas` | field | rules | 2ed9070 | specs/0020 | keep |
| 218 | `ValidationEntry.kind` | field | rules | 5fc1b3d | specs/0009 | keep |
| 219 | `ValidationEntry.operator` | field | rules | 2ed9070 | specs/0020 | keep |
| 220 | `ValidationEntry.formula1` | field | rules | 2ed9070 | specs/0020 | keep |
| 221 | `ValidationEntry.formula2` | field | rules | 2ed9070 | specs/0020 | keep |
| 222 | `ValidationEntry.allow_blank` | field | rules | 2ed9070 | specs/0020 | keep |
| 223 | `ValidationEntry.hide_drop_down` | field | rules | 2ed9070 | specs/0020 | keep |
| 224 | `ValidationEntry.input` | field | rules | 2ed9070 | specs/0020 | keep |
| 225 | `ValidationEntry.error` | field | rules | 2ed9070 | specs/0020 | keep |
| 226 | `Rule` | enum | rules | 08729d3 | specs/0020 | keep |
| 227 | `RuleEntry` | struct | rules | 08729d3 | specs/0020 | keep |
| 228 | `RuleEntry.areas` | field | rules | 2ed9070 | specs/0020 | keep |
| 229 | `RuleEntry.kind` | field | rules | 5fc1b3d | specs/0009 | keep |
| 230 | `RuleEntry.operator` | field | rules | 2ed9070 | specs/0020 | keep |
| 231 | `RuleEntry.formulas` | field | rules | 08729d3 | specs/0020 | keep |
| 232 | `RuleEntry.dxf` | field | rules | 08729d3 | specs/0020 | keep |
| 233 | `RuleEntry.priority` | field | rules | 08729d3 | specs/0020 | keep |
| 234 | `RuleEntry.stop_if_true` | field | rules | 08729d3 | specs/0020 | keep |
| 235 | `RuleEntry.extended` | field | rules | 08729d3 | specs/0020 | keep |
| 236 | `CfvoKind` | enum | rules | 1265000 | specs/0020 | keep |
| 237 | `Cfvo` | struct | rules | 1265000 | specs/0020 | keep |
| 238 | `Cfvo.kind` | field | rules | 5fc1b3d | specs/0009 | keep |
| 239 | `Cfvo.color` | field | rules | 1265000 | specs/0020 | keep |
| 240 | `IconSetKind` | enum | rules | 1265000 | specs/0020 | keep |
| 241 | `TableStyle` | enum | tables | 2e84516 | specs/0021 | keep |
| 242 | `TableStyleInfo` | struct | tables | 2e84516 | specs/0021 | keep |
| 243 | `TableStyleInfo.style` | field | tables | 5fc1b3d | specs/0009 | keep |
| 244 | `TableStyleInfo.first_column` | field | tables | 2e84516 | specs/0021 | keep |
| 245 | `TableStyleInfo.last_column` | field | tables | 2e84516 | specs/0021 | keep |
| 246 | `TableStyleInfo.row_stripes` | field | tables | 2e84516 | specs/0021 | keep |
| 247 | `TableStyleInfo.column_stripes` | field | tables | 2e84516 | specs/0021 | keep |
| 248 | `Table` | struct | tables | 2e84516 | specs/0021 | keep |
| 249 | `Table.name` | field | tables | 5fc1b3d | specs/0009 | keep |
| 250 | `Table.columns` | field | tables | 2e84516 | specs/0021 | keep |
| 251 | `Table.style` | field | tables | 5fc1b3d | specs/0009 | keep |
| 252 | `TableEntry` | struct | tables | 2e84516 | specs/0021 | keep |
| 253 | `TableEntry.name` | field | tables | 5fc1b3d | specs/0009 | keep |
| 254 | `TableEntry.display_name` | field | tables | 2e84516 | specs/0021 | keep |
| 255 | `TableEntry.area` | field | tables | 2ed9070 | specs/0020 | keep |
| 256 | `TableEntry.columns` | field | tables | 2e84516 | specs/0021 | keep |
| 257 | `TableEntry.style` | field | tables | 5fc1b3d | specs/0009 | keep |
| 258 | `TableEntry.header_row` | field | tables | 2e84516 | specs/0021 | keep |
| 259 | `TableEntry.totals_row` | field | tables | 2e84516 | specs/0021 | keep |
| 260 | `TableEntry.calculated` | field | tables | 2e84516 | specs/0021 | keep |
| 261 | `TableEntry.part` | field | tables | 5fc1b3d | specs/0009 | keep |
| 262 | `Comment` | struct | comments | 96dde75 | specs/0022 | keep |
| 263 | `Comment.author` | field | comments | 96dde75 | specs/0022 | keep |
| 264 | `Comment.text` | field | comments | 2634ecd | specs/0019 | keep |
| 265 | `CommentEntry` | struct | comments | 96dde75 | specs/0022 | keep |
| 266 | `CommentEntry.cell` | field | comments | 96dde75 | specs/0022 | keep |
| 267 | `CommentEntry.author` | field | comments | 96dde75 | specs/0022 | keep |
| 268 | `CommentEntry.text` | field | comments | 2634ecd | specs/0019 | keep |
| 269 | `CommentEntry.runs` | field | comments & richtext | 17d1e6b | specs/0024 | keep |
| 270 | `CommentEntry.shape` | field | comments | 69b5e0e | specs/0028 | keep |
| 271 | `CommentBox` | struct | comments | 69b5e0e | specs/0028 | keep |
| 272 | `CommentBox.width` | field | comments | 69b5e0e | specs/0028 | keep |
| 273 | `CommentBox.height` | field | comments | 69b5e0e | specs/0028 | keep |
| 274 | `CommentBox.visible` | field | comments | 69b5e0e | specs/0028 | keep |
| 275 | `Image` | struct | images | b65ea6d | specs/0022 | keep |
| 276 | `Image.bytes` | field | images | b65ea6d | specs/0022 | keep |
| 277 | `Image.size` | field | images | b65ea6d | specs/0022 | keep |
| 278 | `Anchor` | enum | images | b65ea6d | specs/0022 | keep |
| 279 | `ImageAnchor` | enum | images | 7976d9e | specs/0028 | keep |
| 280 | `EditAs` | enum | images | 7976d9e | specs/0028 | keep |
| 281 | `ImageEntry` | struct | images | b65ea6d | specs/0022 | keep |
| 282 | `ImageEntry.from` | field | images | b65ea6d | specs/0022 | keep |
| 283 | `ImageEntry.anchor` | field | images | b65ea6d | specs/0022 | keep |
| 284 | `ImageEntry.part` | field | images | 5fc1b3d | specs/0009 | keep |
| 285 | `ImageEntry.bytes` | field | images | b65ea6d | specs/0022 | keep |
| 286 | `Package.new` | method | core | e7c7061 | specs/0016 | keep |
| 287 | `Package.open` | method | core | 5fc1b3d | specs/0009 | keep |
| 288 | `Package.archive` | method | core | 5fc1b3d | specs/0009 | keep |
| 289 | `Package.part` | method | core | 5fc1b3d | specs/0009 | keep |
| 290 | `Package.read_part` | method | core | 5fc1b3d | specs/0009 | keep |
| 291 | `Package.shared_strings` | method | core | 8f2364d | specs/0010 | keep |
| 292 | `Package.shared_string_items` | method | read | bd02ce4 | specs/0027 | keep |
| 293 | `Package.cells` | method | core | 6c27167 | specs/0010 | keep |
| 294 | `Package.content_types` | method | core | 5fc1b3d | specs/0009 | keep |
| 295 | `Package.relationships` | method | core | 5fc1b3d | specs/0009 | keep |
| 296 | `Package.workbook` | method | core | 5fc1b3d | specs/0009 | keep |
| 297 | `Package.styles` | method | core | 35cd499 | specs/0012 | keep |
| 298 | `Package.cell_style` | method | read | c019afa | specs/0027 | keep |
| 299 | `Package.named_style` | method | read | 0b3e46f | specs/0032 | keep |
| 300 | `Package.differential_styles` | method | read | 0b3e46f | specs/0032 | keep |
| 301 | `Package.default_font` | method | read | 0b3e46f | specs/0032 | keep |
| 302 | `Package.theme_colors` | method | read | 85bb33f | specs/0032 | keep |
| 303 | `Package.resolve_color` | method | read | 85bb33f | specs/0032 | keep |
| 304 | `Package.remove_relationship` | method | core | 5dba448 | specs/0009 | keep |
| 305 | `Package.add_relationship` | method | core | 5dba448 | specs/0009 | keep |
| 306 | `Package.remove_override` | method | core | 5dba448 | specs/0009 | keep |
| 307 | `Package.add_override` | method | core | 5dba448 | specs/0009 | keep |
| 308 | `Package.remove_part` | method | core | 5dba448 | specs/0009 | keep |
| 309 | `Package.replace_part` | method | core | 5dba448 | specs/0009 | keep |
| 310 | `Package.add_part` | method | core | 5dba448 | specs/0009 | keep |
| 311 | `Package.drop_calc_chain` | method | core | 2e3bb2a | specs/0011 | keep |
| 312 | `Package.set_cell` | method | core | aa87cbc | specs/0011 | keep |
| 313 | `Package.clear_cell` | method | core | aa87cbc | specs/0011 | keep |
| 314 | `Package.add_cell_style` | method | core | abf2618 | specs/0012 | keep |
| 315 | `Package.add_style` | method | core | f14885f | specs/0017 | keep |
| 316 | `Package.set_cell_style` | method | core | abf2618 | specs/0012 | keep |
| 317 | `Package.set_column_width` | method | core | d03ed7b | specs/0018 | keep |
| 318 | `Package.set_column_hidden` | method | core | d03ed7b | specs/0018 | keep |
| 319 | `Package.set_row_height` | method | core | d03ed7b | specs/0018 | keep |
| 320 | `Package.set_row_hidden` | method | core | d03ed7b | specs/0018 | keep |
| 321 | `Package.group_rows` | method | rowcol | 7b3d9b2 | specs/0026 | keep |
| 322 | `Package.group_columns` | method | rowcol | 7b3d9b2 | specs/0026 | keep |
| 323 | `Package.set_row_collapsed` | method | rowcol | 7b3d9b2 | specs/0026 | keep |
| 324 | `Package.set_column_collapsed` | method | rowcol | 7b3d9b2 | specs/0026 | keep |
| 325 | `Package.set_outline_summary` | method | rowcol | 7b3d9b2 | specs/0026 | keep |
| 326 | `Package.set_row_style` | method | rowcol | 9d266f0 | specs/0026 | keep |
| 327 | `Package.set_column_style` | method | rowcol | 9d266f0 | specs/0026 | keep |
| 328 | `Package.freeze_panes` | method | core | d03ed7b | specs/0018 | keep |
| 329 | `Package.set_tab_color` | method | sheetprops | 4f53d49 | specs/0025 | keep |
| 330 | `Package.set_sheet_view` | method | sheetprops | 4f53d49 | specs/0025 | keep |
| 331 | `Package.set_print_options` | method | sheetprops | 791a794 | specs/0025 | keep |
| 332 | `Package.set_page_margins` | method | sheetprops | 791a794 | specs/0025 | keep |
| 333 | `Package.set_page_setup` | method | sheetprops | 791a794 | specs/0025 | keep |
| 334 | `Package.set_header_footer` | method | sheetprops | 791a794 | specs/0025 | keep |
| 335 | `Package.set_print_area` | method | all(sheetprops, metadata) | 8faf378 | specs/0025 | keep |
| 336 | `Package.set_print_titles` | method | all(sheetprops, metadata) | 8faf378 | specs/0025 | keep |
| 337 | `Package.set_auto_filter` | method | all(sheetprops, metadata) | 8faf378 | specs/0025 | keep |
| 338 | `Package.merge_cells` | method | core | d03ed7b | specs/0018 | keep |
| 339 | `Package.unmerge_cells` | method | core | d03ed7b | specs/0018 | keep |
| 340 | `Package.insert_rows` | method | rowcol | 7578ee0 | specs/0018 | keep |
| 341 | `Package.delete_rows` | method | rowcol | 7578ee0 | specs/0018 | keep |
| 342 | `Package.insert_columns` | method | rowcol | c3b5bbd | specs/0018 | keep |
| 343 | `Package.delete_columns` | method | rowcol | c3b5bbd | specs/0018 | keep |
| 344 | `Package.add_defined_name` | method | metadata | 2634ecd | specs/0019 | keep |
| 345 | `Package.remove_defined_name` | method | metadata | 2634ecd | specs/0019 | keep |
| 346 | `Package.set_hyperlink` | method | metadata | a791e2c | specs/0019 | keep |
| 347 | `Package.remove_hyperlink` | method | metadata | a791e2c | specs/0019 | keep |
| 348 | `Package.properties` | method | metadata | 51acd32 | specs/0019 | keep |
| 349 | `Package.set_property` | method | metadata | 51acd32 | specs/0019 | keep |
| 350 | `Package.protect_sheet` | method | metadata | 51acd32 | specs/0019 | keep |
| 351 | `Package.unprotect_sheet` | method | metadata | 51acd32 | specs/0019 | keep |
| 352 | `Package.protect_workbook` | method | metadata | 51acd32 | specs/0019 | keep |
| 353 | `Package.unprotect_workbook` | method | metadata | 51acd32 | specs/0019 | keep |
| 354 | `Package.add_validation` | method | rules | 2ed9070 | specs/0020 | keep |
| 355 | `Package.remove_validation` | method | rules | 2ed9070 | specs/0020 | keep |
| 356 | `Package.validations` | method | rules | 2ed9070 | specs/0020 | keep |
| 357 | `Package.add_differential_style` | method | rules | fa3ad04 | specs/0020 | keep |
| 358 | `Package.add_conditional_format` | method | rules | 08729d3 | specs/0020 | keep |
| 359 | `Package.remove_conditional_format` | method | rules | 08729d3 | specs/0020 | keep |
| 360 | `Package.conditional_formats` | method | rules | 08729d3 | specs/0020 | keep |
| 361 | `Package.add_table` | method | tables | 2e84516 | specs/0021 | keep |
| 362 | `Package.remove_table` | method | tables | 4a68144 | specs/0021 | keep |
| 363 | `Package.tables` | method | tables | 2e84516 | specs/0021 | keep |
| 364 | `Package.add_comment` | method | comments | 96dde75 | specs/0022 | keep |
| 365 | `Package.add_rich_comment` | method | all(comments, richtext) | 17d1e6b | specs/0024 | keep |
| 366 | `Package.remove_comment` | method | comments | 193aa84 | specs/0022 | keep |
| 367 | `Package.set_comment_box` | method | comments | 69b5e0e | specs/0028 | keep |
| 368 | `Package.comments` | method | comments | 96dde75 | specs/0022 | keep |
| 369 | `Package.add_image` | method | images | b65ea6d | specs/0022 | keep |
| 370 | `Package.add_image_at` | method | images | 7976d9e | specs/0028 | keep |
| 371 | `Package.remove_image` | method | images | 7976d9e | specs/0028 | keep |
| 372 | `Package.images` | method | images | b65ea6d | specs/0022 | keep |
| 373 | `Package.recalculate_on_open` | method | core | 77c3016 | specs/0013 | keep |
| 374 | `Package.sheets` | method | core | 770c9dd | specs/0016 | keep |
| 375 | `Package.add_sheet` | method | core | 770c9dd | specs/0016 | keep |
| 376 | `Package.rename_sheet` | method | core | 770c9dd | specs/0016 | keep |
| 377 | `Package.move_sheet` | method | core | 770c9dd | specs/0016 | keep |
| 378 | `Package.remove_sheet` | method | core | 8abc666 | specs/0016 | keep |
| 379 | `Package.copy_sheet` | method | copy | 4a8482c | specs/0028 | keep |
| 380 | `Package.write_to` | method | core | 5dba448 | specs/0009 | keep |
| 381 | `rels_part_name` | fn | core | 5fc1b3d | specs/0009 | keep |
| 382 | `rels_part_source` | fn | core | 5fc1b3d | specs/0009 | keep |
| 383 | `SheetView` | struct | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 384 | `SheetView.zoom` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 385 | `SheetView.show_grid_lines` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 386 | `SheetView.show_headings` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 387 | `SheetView.show_zeros` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 388 | `SheetView.show_formulas` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 389 | `SheetView.right_to_left` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 390 | `SheetView.view` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 391 | `ViewKind` | enum | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 392 | `PrintOptions` | struct | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 393 | `PrintOptions.horizontal_centered` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 394 | `PrintOptions.vertical_centered` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 395 | `PrintOptions.headings` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 396 | `PrintOptions.grid_lines` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 397 | `PageMargins` | struct | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 398 | `PageMargins.left` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 399 | `PageMargins.right` | field | any(sheetprops, read) | 4f53d49 | specs/0025 | keep |
| 400 | `PageMargins.top` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 401 | `PageMargins.bottom` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 402 | `PageMargins.header` | field | any(sheetprops, read) | 2e84516 | specs/0021 | keep |
| 403 | `PageMargins.footer` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 404 | `PageSetup` | struct | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 405 | `PageSetup.orientation` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 406 | `PageSetup.paper_size` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 407 | `PageSetup.scale` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 408 | `PageSetup.fit_to_width` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 409 | `PageSetup.fit_to_height` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 410 | `PageSetup.fit_to_page` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 411 | `PageSetup.first_page_number` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 412 | `PageSetup.black_and_white` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 413 | `PageSetup.draft` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 414 | `PageSetup.copies` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 415 | `PageSetup.page_order` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 416 | `Orientation` | enum | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 417 | `PageOrder` | enum | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 418 | `HeaderFooter` | struct | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 419 | `HeaderFooter.odd_header` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 420 | `HeaderFooter.odd_footer` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 421 | `HeaderFooter.even_header` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 422 | `HeaderFooter.even_footer` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 423 | `HeaderFooter.first_header` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 424 | `HeaderFooter.first_footer` | field | any(sheetprops, read) | 791a794 | specs/0025 | keep |
| 425 | `HeaderFooter.section` | method | sheetprops | 791a794 | specs/0025 | keep |

### `crate::styles`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 426 | `use colors::ResolvedColor` | use | read | 85bb33f | specs/0032 | keep |
| 427 | `use elements::{ Alignment, Border, BorderStyle, Color, Fill, Font, Horizontal, Pattern, Protection, Scheme, Side, Style, Underline, Vertical, VerticalAlign, }` | use | core | f14885f | specs/0017 | keep |
| 428 | `use read::{CellStyle, NamedStyle}` | use | read | 0b3e46f | specs/0032 | keep |
| 429 | `Styles` | struct | core | 35cd499 | specs/0012 | keep |
| 430 | `Styles.num_fmts` | field | core | 35cd499 | specs/0012 | keep |
| 431 | `Styles.cell_xfs` | field | core | 35cd499 | specs/0012 | keep |
| 432 | `Styles.cell_style_xfs` | field | read | 0b3e46f | specs/0032 | keep |
| 433 | `Styles.cell_styles` | field | read | 0b3e46f | specs/0032 | keep |
| 434 | `Styles.indexed_colors` | field | read | 85bb33f | specs/0032 | keep |
| 435 | `NumFmt` | struct | core | 35cd499 | specs/0012 | keep |
| 436 | `NumFmt.id` | field | core | 35cd499 | specs/0012 | keep |
| 437 | `NumFmt.code` | field | core | 35cd499 | specs/0012 | keep |
| 438 | `Xf` | struct | core | 35cd499 | specs/0012 | keep |
| 439 | `Xf.num_fmt_id` | field | core | 35cd499 | specs/0012 | keep |
| 440 | `Xf.font_id` | field | core | 35cd499 | specs/0012 | keep |
| 441 | `Xf.fill_id` | field | core | 35cd499 | specs/0012 | keep |
| 442 | `Xf.border_id` | field | core | 35cd499 | specs/0012 | keep |
| 443 | `Xf.xf_id` | field | core | 35cd499 | specs/0012 | keep |
| 444 | `NumberFormat` | enum | core | 35cd499 | specs/0012 | keep |
| 445 | `Styles.fonts` | method | core | f14885f | specs/0017 | keep |
| 446 | `Styles.fills` | method | core | f14885f | specs/0017 | keep |
| 447 | `Styles.borders` | method | core | f14885f | specs/0017 | keep |
| 448 | `Styles.number_format` | method | core | 35cd499 | specs/0012 | keep |

### `crate::xml`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 449 | `BYTE_ORDER_MARK` | const | core | a9f2cad | specs/0009 | keep |
| 450 | `Token` | struct | core | a9f2cad | specs/0009 | keep |
| 451 | `Token.event` | field | core | a9f2cad | specs/0009 | keep |
| 452 | `Token.raw` | field | core | a9f2cad | specs/0009 | keep |
| 453 | `Stream` | struct | core | a9f2cad | specs/0009 | keep |
| 454 | `Stream.new` | method | core | a9f2cad | specs/0009 | keep |
| 455 | `Stream.bom` | method | core | a9f2cad | specs/0009 | keep |
| 456 | `Stream.next` | method | core | a9f2cad | specs/0009 | keep |
| 457 | `Scope` | struct | core | 5fc1b3d | specs/0009 | keep |
| 458 | `Scope.new` | method | core | a9f2cad | specs/0009 | keep |
| 459 | `Scope.start` | method | core | 5fc1b3d | specs/0009 | keep |
| 460 | `Scope.end` | method | core | 5fc1b3d | specs/0009 | keep |
| 461 | `Scope.resolve_element` | method | core | 5fc1b3d | specs/0009 | keep |
| 462 | `Scope.resolve_attribute` | method | core | 5fc1b3d | specs/0009 | keep |

### `crate::package::extlink`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 463 | `ExternalLink` | struct | read | cdffef7 | specs/0033 | keep |
| 464 | `ExternalLink.part` | field | read | cdffef7 | specs/0033 | keep |
| 465 | `ExternalLink.kind` | field | read | cdffef7 | specs/0033 | keep |
| 466 | `ExternalLinkKind` | enum | read | cdffef7 | specs/0033 | keep |
| 467 | `Package.external_links` | method | read | cdffef7 | specs/0033 | keep |

### `crate::package::pivot`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 468 | `PivotTable` | struct | read | ec9e58f | specs/0033 | keep |
| 469 | `PivotTable.part` | field | read | ec9e58f | specs/0033 | keep |
| 470 | `PivotTable.sheet` | field | read | ec9e58f | specs/0033 | keep |
| 471 | `PivotTable.name` | field | read | ec9e58f | specs/0033 | keep |
| 472 | `PivotTable.cache_id` | field | read | ec9e58f | specs/0033 | keep |
| 473 | `PivotTable.location` | field | read | ec9e58f | specs/0033 | keep |
| 474 | `PivotTable.first_header_row` | field | read | ec9e58f | specs/0033 | keep |
| 475 | `PivotTable.first_data_row` | field | read | ec9e58f | specs/0033 | keep |
| 476 | `PivotTable.first_data_col` | field | read | ec9e58f | specs/0033 | keep |
| 477 | `PivotTable.cache` | field | read | ec9e58f | specs/0033 | keep |
| 478 | `PivotTable.fields` | field | read | ec9e58f | specs/0033 | keep |
| 479 | `PivotTable.row_fields` | field | read | ec9e58f | specs/0033 | keep |
| 480 | `PivotTable.column_fields` | field | read | ec9e58f | specs/0033 | keep |
| 481 | `PivotTable.page_fields` | field | read | ec9e58f | specs/0033 | keep |
| 482 | `PivotTable.data_fields` | field | read | ec9e58f | specs/0033 | keep |
| 483 | `PivotField` | struct | read | ec9e58f | specs/0033 | keep |
| 484 | `PivotField.name` | field | read | ec9e58f | specs/0033 | keep |
| 485 | `PivotField.axis` | field | read | ec9e58f | specs/0033 | keep |
| 486 | `PivotField.data_field` | field | read | ec9e58f | specs/0033 | keep |
| 487 | `Axis` | enum | read | ec9e58f | specs/0033 | keep |
| 488 | `FieldRef` | enum | read | ec9e58f | specs/0033 | keep |
| 489 | `DataField` | struct | read | ec9e58f | specs/0033 | keep |
| 490 | `DataField.name` | field | read | ec9e58f | specs/0033 | keep |
| 491 | `DataField.field` | field | read | ec9e58f | specs/0033 | keep |
| 492 | `DataField.subtotal` | field | read | ec9e58f | specs/0033 | keep |
| 493 | `Subtotal` | enum | read | ec9e58f | specs/0033 | keep |
| 494 | `PivotCache` | struct | read | ec9e58f | specs/0033 | keep |
| 495 | `PivotCache.part` | field | read | ec9e58f | specs/0033 | keep |
| 496 | `PivotCache.source` | field | read | ec9e58f | specs/0033 | keep |
| 497 | `PivotCache.fields` | field | read | ec9e58f | specs/0033 | keep |
| 498 | `PivotCache.record_count` | field | read | ec9e58f | specs/0033 | keep |
| 499 | `PivotSource` | enum | read | ec9e58f | specs/0033 | keep |
| 500 | `Package.pivot_tables` | method | read | ec9e58f | specs/0033 | keep |

### `crate::package::sheet_read`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 501 | `ColumnFormat` | struct | read | bd02ce4 | specs/0027 | keep |
| 502 | `ColumnFormat.first` | field | read | bd02ce4 | specs/0027 | keep |
| 503 | `ColumnFormat.last` | field | read | bd02ce4 | specs/0027 | keep |
| 504 | `ColumnFormat.width` | field | read | bd02ce4 | specs/0027 | keep |
| 505 | `ColumnFormat.custom_width` | field | read | bd02ce4 | specs/0027 | keep |
| 506 | `ColumnFormat.hidden` | field | read | bd02ce4 | specs/0027 | keep |
| 507 | `ColumnFormat.style` | field | read | bd02ce4 | specs/0027 | keep |
| 508 | `ColumnFormat.outline_level` | field | read | bd02ce4 | specs/0027 | keep |
| 509 | `ColumnFormat.collapsed` | field | read | bd02ce4 | specs/0027 | keep |
| 510 | `RowFormat` | struct | read | bd02ce4 | specs/0027 | keep |
| 511 | `RowFormat.row` | field | read | bd02ce4 | specs/0027 | keep |
| 512 | `RowFormat.height` | field | read | bd02ce4 | specs/0027 | keep |
| 513 | `RowFormat.custom_height` | field | read | bd02ce4 | specs/0027 | keep |
| 514 | `RowFormat.hidden` | field | read | bd02ce4 | specs/0027 | keep |
| 515 | `RowFormat.style` | field | read | bd02ce4 | specs/0027 | keep |
| 516 | `RowFormat.custom_format` | field | read | bd02ce4 | specs/0027 | keep |
| 517 | `RowFormat.outline_level` | field | read | bd02ce4 | specs/0027 | keep |
| 518 | `RowFormat.collapsed` | field | read | bd02ce4 | specs/0027 | keep |
| 519 | `HyperlinkTarget` | enum | read | cce4285 | specs/0030 | keep |
| 520 | `HyperlinkEntry` | struct | read | bd02ce4 | specs/0027 | keep |
| 521 | `HyperlinkEntry.cell` | field | read | bd02ce4 | specs/0027 | keep |
| 522 | `HyperlinkEntry.target` | field | read | bd02ce4 | specs/0027 | keep |
| 523 | `HyperlinkEntry.display` | field | read | bd02ce4 | specs/0027 | keep |
| 524 | `HyperlinkEntry.tooltip` | field | read | bd02ce4 | specs/0027 | keep |
| 525 | `SheetSettings` | struct | read | bd02ce4 | specs/0027 | keep |
| 526 | `SheetSettings.tab_color` | field | read | bd02ce4 | specs/0027 | keep |
| 527 | `SheetSettings.outline_summary` | field | read | bd02ce4 | specs/0027 | keep |
| 528 | `SheetSettings.fit_to_page` | field | read | bd02ce4 | specs/0027 | keep |
| 529 | `SheetSettings.view` | field | read | bd02ce4 | specs/0027 | keep |
| 530 | `SheetSettings.freeze` | field | read | bd02ce4 | specs/0027 | keep |
| 531 | `SheetSettings.print_options` | field | read | bd02ce4 | specs/0027 | keep |
| 532 | `SheetSettings.margins` | field | read | bd02ce4 | specs/0027 | keep |
| 533 | `SheetSettings.page_setup` | field | read | bd02ce4 | specs/0027 | keep |
| 534 | `SheetSettings.header_footer` | field | read | bd02ce4 | specs/0027 | keep |
| 535 | `SheetSettings.auto_filter` | field | read | bd02ce4 | specs/0027 | keep |
| 536 | `SheetSettings.merges` | field | read | bd02ce4 | specs/0027 | keep |
| 537 | `SheetSettings.hyperlinks` | field | read | bd02ce4 | specs/0027 | keep |
| 538 | `SheetSettings.protection` | field | read | bd02ce4 | specs/0027 | keep |
| 539 | `SheetSettings.outline_max` | field | read | bd02ce4 | specs/0027 | keep |
| 540 | `Rows` | struct | read | bd02ce4 | specs/0027 | keep |
| 541 | `Package.columns` | method | read | bd02ce4 | specs/0027 | keep |
| 542 | `Package.rows` | method | read | bd02ce4 | specs/0027 | keep |
| 543 | `Package.sheet_settings` | method | read | bd02ce4 | specs/0027 | keep |

### `crate::package::survey`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 544 | `PartKinds` | struct | read | 338e91c | specs/0033 | keep |
| 545 | `PartKinds.worksheets` | field | read | 338e91c | specs/0033 | keep |
| 546 | `PartKinds.chartsheets` | field | read | 338e91c | specs/0033 | keep |
| 547 | `PartKinds.dialogsheets` | field | read | 338e91c | specs/0033 | keep |
| 548 | `PartKinds.macrosheets` | field | read | 338e91c | specs/0033 | keep |
| 549 | `PartKinds.charts` | field | read | 338e91c | specs/0033 | keep |
| 550 | `PartKinds.chartex` | field | read | 338e91c | specs/0033 | keep |
| 551 | `PartKinds.drawings` | field | read | 338e91c | specs/0033 | keep |
| 552 | `PartKinds.images` | field | read | 338e91c | specs/0033 | keep |
| 553 | `PartKinds.tables` | field | read | 338e91c | specs/0033 | keep |
| 554 | `PartKinds.pivot_tables` | field | read | 338e91c | specs/0033 | keep |
| 555 | `PartKinds.pivot_caches` | field | read | 338e91c | specs/0033 | keep |
| 556 | `PartKinds.pivot_cache_records` | field | read | 338e91c | specs/0033 | keep |
| 557 | `PartKinds.external_links` | field | read | 338e91c | specs/0033 | keep |
| 558 | `PartKinds.vba_projects` | field | read | 338e91c | specs/0033 | keep |
| 559 | `PartKinds.slicers` | field | read | 338e91c | specs/0033 | keep |
| 560 | `PartKinds.slicer_caches` | field | read | 338e91c | specs/0033 | keep |
| 561 | `PartKinds.timelines` | field | read | 338e91c | specs/0033 | keep |
| 562 | `PartKinds.timeline_caches` | field | read | 338e91c | specs/0033 | keep |
| 563 | `PartKinds.comments` | field | read | 338e91c | specs/0033 | keep |
| 564 | `PartKinds.threaded_comments` | field | read | 338e91c | specs/0033 | keep |
| 565 | `PartKinds.persons` | field | read | 338e91c | specs/0033 | keep |
| 566 | `PartKinds.custom_xml` | field | read | 338e91c | specs/0033 | keep |
| 567 | `PartKinds.connections` | field | read | 338e91c | specs/0033 | keep |
| 568 | `PartKinds.query_tables` | field | read | 338e91c | specs/0033 | keep |
| 569 | `PartKinds.custom_properties` | field | read | 338e91c | specs/0033 | keep |
| 570 | `PartKinds.printer_settings` | field | read | 338e91c | specs/0033 | keep |
| 571 | `PartKinds.calc_chains` | field | read | 338e91c | specs/0033 | keep |
| 572 | `PartKinds.other_typed` | field | read | 338e91c | specs/0033 | keep |
| 573 | `PartKinds.untyped` | field | read | 338e91c | specs/0033 | keep |
| 574 | `SheetParts` | struct | read | 338e91c | specs/0033 | keep |
| 575 | `SheetParts.drawings` | field | read | 338e91c | specs/0033 | keep |
| 576 | `SheetParts.charts` | field | read | 338e91c | specs/0033 | keep |
| 577 | `SheetParts.images` | field | read | 338e91c | specs/0033 | keep |
| 578 | `SheetParts.comments` | field | read | 338e91c | specs/0033 | keep |
| 579 | `SheetParts.vml_drawings` | field | read | 338e91c | specs/0033 | keep |
| 580 | `SheetParts.tables` | field | read | 338e91c | specs/0033 | keep |
| 581 | `SheetParts.pivot_tables` | field | read | 338e91c | specs/0033 | keep |
| 582 | `SheetParts.slicers` | field | read | 338e91c | specs/0033 | keep |
| 583 | `SheetParts.timelines` | field | read | 338e91c | specs/0033 | keep |
| 584 | `SheetParts.printer_settings` | field | read | 338e91c | specs/0033 | keep |
| 585 | `SheetParts.external_hyperlinks` | field | read | 338e91c | specs/0033 | keep |
| 586 | `SheetParts.sparkline_groups` | field | read | 338e91c | specs/0033 | keep |
| 587 | `Package.part_kinds` | method | read | 338e91c | specs/0033 | keep |
| 588 | `Package.sheet_parts` | method | read | 338e91c | specs/0033 | keep |

### `crate::styles::colors`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 589 | `ResolvedColor` | struct | read | 85bb33f | specs/0032 | keep |
| 590 | `ResolvedColor.rgb` | field | read | 85bb33f | specs/0032 | keep |
| 591 | `ResolvedColor.tint` | field | read | 85bb33f | specs/0032 | keep |
| 592 | `ResolvedColor.tinted` | method | read | 85bb33f | specs/0032 | keep |

### `crate::styles::elements`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 593 | `Style` | struct | core | f14885f | specs/0017 | keep |
| 594 | `Style.number_format` | field | core | f14885f | specs/0017 | keep |
| 595 | `Style.font` | field | core | f14885f | specs/0017 | keep |
| 596 | `Style.fill` | field | core | f14885f | specs/0017 | keep |
| 597 | `Style.border` | field | core | f14885f | specs/0017 | keep |
| 598 | `Style.alignment` | field | core | 4bd46a1 | specs/0017 | keep |
| 599 | `Style.protection` | field | core | 4bd46a1 | specs/0017 | keep |
| 600 | `Font` | struct | core | f14885f | specs/0017 | keep |
| 601 | `Font.name` | field | core | f14885f | specs/0017 | keep |
| 602 | `Font.size` | field | core | f14885f | specs/0017 | keep |
| 603 | `Font.bold` | field | core | f14885f | specs/0017 | keep |
| 604 | `Font.italic` | field | core | f14885f | specs/0017 | keep |
| 605 | `Font.strike` | field | core | f14885f | specs/0017 | keep |
| 606 | `Font.underline` | field | core | f14885f | specs/0017 | keep |
| 607 | `Font.vertical_align` | field | core | f14885f | specs/0017 | keep |
| 608 | `Font.color` | field | core | f14885f | specs/0017 | keep |
| 609 | `Font.family` | field | core | f14885f | specs/0017 | keep |
| 610 | `Font.scheme` | field | core | f14885f | specs/0017 | keep |
| 611 | `Underline` | enum | core | f14885f | specs/0017 | keep |
| 612 | `VerticalAlign` | enum | core | f14885f | specs/0017 | keep |
| 613 | `Scheme` | enum | core | f14885f | specs/0017 | keep |
| 614 | `Color` | enum | core | f14885f | specs/0017 | keep |
| 615 | `Fill` | struct | core | f14885f | specs/0017 | keep |
| 616 | `Fill.pattern` | field | core | f14885f | specs/0017 | keep |
| 617 | `Fill.foreground` | field | core | f14885f | specs/0017 | keep |
| 618 | `Fill.background` | field | core | f14885f | specs/0017 | keep |
| 619 | `Pattern` | enum | core | f14885f | specs/0017 | keep |
| 620 | `Border` | struct | core | f14885f | specs/0017 | keep |
| 621 | `Border.left` | field | core | f14885f | specs/0017 | keep |
| 622 | `Border.right` | field | core | f14885f | specs/0017 | keep |
| 623 | `Border.top` | field | core | f14885f | specs/0017 | keep |
| 624 | `Border.bottom` | field | core | f14885f | specs/0017 | keep |
| 625 | `Border.diagonal` | field | core | f14885f | specs/0017 | keep |
| 626 | `Border.diagonal_up` | field | core | f14885f | specs/0017 | keep |
| 627 | `Border.diagonal_down` | field | core | f14885f | specs/0017 | keep |
| 628 | `Side` | struct | core | f14885f | specs/0017 | keep |
| 629 | `Side.style` | field | core | f14885f | specs/0017 | keep |
| 630 | `Side.color` | field | core | f14885f | specs/0017 | keep |
| 631 | `BorderStyle` | enum | core | f14885f | specs/0017 | keep |
| 632 | `Alignment` | struct | core | f14885f | specs/0017 | keep |
| 633 | `Alignment.horizontal` | field | core | f14885f | specs/0017 | keep |
| 634 | `Alignment.vertical` | field | core | f14885f | specs/0017 | keep |
| 635 | `Alignment.wrap_text` | field | core | f14885f | specs/0017 | keep |
| 636 | `Alignment.shrink_to_fit` | field | core | f14885f | specs/0017 | keep |
| 637 | `Alignment.indent` | field | core | f14885f | specs/0017 | keep |
| 638 | `Alignment.text_rotation` | field | core | f14885f | specs/0017 | keep |
| 639 | `Horizontal` | enum | core | f14885f | specs/0017 | keep |
| 640 | `Vertical` | enum | core | f14885f | specs/0017 | keep |
| 641 | `Protection` | struct | core | f14885f | specs/0017 | keep |
| 642 | `Protection.locked` | field | core | f14885f | specs/0017 | keep |
| 643 | `Protection.hidden` | field | core | f14885f | specs/0017 | keep |

### `crate::styles::read`

| # | item | kind | feature | added by | spec | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| 644 | `CellStyle` | struct | read | c019afa | specs/0027 | keep |
| 645 | `CellStyle.number_format` | field | read | c019afa | specs/0027 | keep |
| 646 | `CellStyle.font` | field | read | c019afa | specs/0027 | keep |
| 647 | `CellStyle.fill` | field | read | c019afa | specs/0027 | keep |
| 648 | `CellStyle.border` | field | read | c019afa | specs/0027 | keep |
| 649 | `CellStyle.alignment` | field | read | c019afa | specs/0027 | keep |
| 650 | `CellStyle.protection` | field | read | c019afa | specs/0027 | keep |
| 651 | `NamedStyle` | struct | read | 0b3e46f | specs/0032 | keep |
| 652 | `NamedStyle.name` | field | read | 0b3e46f | specs/0032 | keep |
| 653 | `NamedStyle.xf_id` | field | read | 0b3e46f | specs/0032 | keep |
| 654 | `NamedStyle.builtin_id` | field | read | 0b3e46f | specs/0032 | keep |
| 655 | `NamedStyle.custom_builtin` | field | read | 0b3e46f | specs/0032 | keep |
| 656 | `NamedStyle.hidden` | field | read | 0b3e46f | specs/0032 | keep |
| 657 | `NamedStyle.outline_level` | field | read | 0b3e46f | specs/0032 | keep |
| 658 | `Styles.typed_fonts` | method | read | 85bb33f | specs/0032 | keep |
| 659 | `Styles.typed_fills` | method | read | 85bb33f | specs/0032 | keep |
| 660 | `Styles.typed_borders` | method | read | 85bb33f | specs/0032 | keep |

## Counts per verdict

- keep: 660
- rename before publication: 0
- hide: 0
- widen or narrow: 0
- in all: 660
