# specs/0033: read breadth B: the survey, pivot tables, external links

Status: APPROVED 2026-10-12 (Milestone 5 part 2; decisions 1 to 3
decided A by the maintainer 2026-10-12, typed in the planning
session).

Milestone 5, "charts and read breadth", was approved by the
maintainer 2026-10-11 (`ROADMAP.md`, "Milestone 5"): reading first,
then charts; everything behind a Cargo feature; the Milestone 2
ceiling kept for the core; the Milestone 2 Excel check gates no push
in this milestone. This part is the second of six, the last of the
two reading parts: a call that names which part kinds a package
holds, with counts, so a consumer learns what a workbook carries
before deciding what to read or edit; pivot table definitions read
typed; external link targets read typed. Reads only; no writer
changes. Part 3 (charts read, the feature `charts`) has its own spec
and builds on the survey's chart count.

## Goal

Three questions a consumer asks of a workbook it did not write,
answered without reading any part the question does not need: what
is in here (the survey, from the content types and the
relationships alone, with the sparklines counted by one streamed
pass over a sheet); what does each pivot table show and where does
its data come from (the name, the location, the source, the fields
by axis, read from the pivot table part and its cache definition);
what does the workbook link to outside itself (every external link
part's target, sheet names and defined-name count, or its DDE or OLE
identity). Every value is counted or digested by the samples on the
corpus, never printed; every read part is passed through byte for
byte.

## Size and packaging (decision 1, before any code)

The budget figure after part 1 is 436992 section bytes against the
ceiling of 458752 (`records/size-m5p1-2026-10-12-6f51899.md`): 21760
of headroom, held for fixes, and the core's growth in part 1 was 0.
The feature `read` costs 70044 over the core probe (the read probe
1040252 over the format probe 970208 in the same record).
Planning's estimate for this part, priced from part 1's three
readers (18056 for named styles, dxfs, the palette, a theme reader
and a resolver): 4000 to 8000 for the survey (a table of content
types and relationship kinds, counted), 12000 to 20000 for the pivot
reader (two parts, a dozen attributes, five enumerations), 5000 to
9000 for the external link reader; 21000 to 37000 in all, so `read`
ends between 91000 and 107000. Part 1's spec asked that planning be
told above 80000; this part crosses that line by design, and the
line moves: planning is told in the report at any figure, and a
feature cost above 110000, or core growth above 2000, is a finding
the report names. The core's growth is what the feature gates add,
expected 0.

- A (recommended): everything under the existing feature `read`, no
  new feature, no new probe: `probe-emend-read-write-xml` grows by
  the three new calls, so one figure moves and the catalogue stays
  one list. 28 probes. A consumer that reads anything typed wants
  the survey first, and the two readers are small next to the
  feature's own.
- B: the survey under `read`, a new feature `pivots` for the pivot
  and external link readers and a 29th probe
  (`probe-emend-pivots-write-xml`): a consumer that resolves styles
  but never looks at a pivot table carries 17000 to 29000 fewer
  bytes, at the cost of a second reading feature and the probe's
  classification.

## Facts planning checked (2026-10-12)

- The corpus, counted by planning with Python's `zipfile`,
  `ElementTree` and regular expressions over every regular file of
  the corpus directory that is a zip (27 workbooks), names and counts
  only, by content type over `[Content_Types].xml` (an `Override`
  per part, a `Default` per extension): worksheets 95 in 27; charts
  (`drawingml.chart+xml`) 26 in 11, at most 4 in one; `chartex` 0;
  drawings 37 in 16; an image extension `Default` in 21; tables 51 in
  11; pivot tables (`pivotTable+xml`) 9 in 4, at most 4 in one; pivot
  cache definitions 8 in 4; pivot cache records 2 in 2; external
  links (`externalLink+xml`) 0; `vbaProject` 0; slicers 2 in 2 and
  slicer caches 3 in 2; timelines 1 in 1 and timeline caches 1 in 1;
  threaded comments 0 and persons 0; comments 9 in 6; `customXml`
  item overrides 60 in 10 (90 entries under `customXml/` with their
  rels and item properties); connections 2 in 2; query tables 0;
  custom properties 18 in 18; a `printerSettings` `Default` in 10;
  calc chains 9 in 9; chartsheets, dialogsheets and macrosheets 0.
  `x14:sparklineGroup` elements in worksheet parts: 6 in 4
  workbooks. `records/corpus-coverage-2026-09-16.md`'s tag
  `external` (2 workbooks) names external DATA (`connections` and
  `powerPivotData` relationships, pivot caches of type `external`),
  not external link parts: no corpus workbook holds one, and no
  `xl/workbook.xml` holds `<externalReferences>`.
- Relationship kinds by source over the corpus, the type URI's last
  segment: from `xl/workbook.xml`: `theme` 27, `styles` 27,
  `worksheet` 95, `sharedStrings` 27, `calcChain` 9, `customXml` 30
  (10 workbooks), `slicerCache` 3, `pivotCacheDefinition` 8,
  `pivotTable` 1, `powerPivotData` 2, `connections` 2,
  `timelineCache` 1; from worksheets: `hyperlink` with `TargetMode`
  `External` 198 (7 workbooks), `printerSettings` 50, `table` 51,
  `drawing` 37, `vmlDrawing` 11, `comments` 9, `pivotTable` 8,
  `ctrlProp` 3, `slicer` 2, `timeline` 1; from the package:
  `officeDocument`, `core-properties` and `extended-properties` 27
  each, `custom-properties` 18, `classificationlabels` 9. So of the
  9 pivot table parts 8 are reached from a worksheet and 1 from the
  workbook part: a reader that walks worksheet relationships alone
  misses one, which is why the survey and the pivot reader find
  parts by content type and join the sheet afterwards.
- The 9 `pivotTableDefinition` parts (the largest 31286 bytes):
  every root carries `name` and `cacheId` (and the `apply*`,
  `dataCaption`, `updatedVersion`, `createdVersion`,
  `minRefreshableVersion`, `itemPrintTitles`, `indent`,
  `multipleFieldFilters` attributes on all 9, `outline` and
  `outlineData` on 8, `chartFormat` on 5, `compact`, `compactData`,
  `showDrill`, `rowGrandTotals`, `colGrandTotals`,
  `rowHeaderCaption` on 1 each, an `mc:Ignorable` and an `xr:uid` on
  all 9); children: `location` 9 (with `ref`, `firstHeaderRow`,
  `firstDataRow` and `firstDataCol` on all 9), `pivotFields` 9
  holding 32 `pivotField` (12 with `axis="axisRow"`, none with
  `axisCol`, `axisPage` or `axisValues`, none with `dataField`),
  `rowFields` 9 holding 12 `field`, `colFields` 1 holding 1,
  `pageFields` 0, `dataFields` 8 holding 10 `dataField`, `rowItems`
  9, `colItems` 9, `formats` 8, `pivotTableStyleInfo` 8,
  `chartFormats` 6, `pivotHierarchies` 6, `rowHierarchiesUsage` 6,
  `colHierarchiesUsage` 1, `extLst` 9. Every pivot table part's rels
  hold one `pivotCacheDefinition` relationship (9). The 8
  `pivotCacheDefinition` parts (the largest 8703 bytes; the 2
  records parts at most 2046): `cacheSource` with `type="worksheet"`
  in 2 (a `worksheetSource` with `name` alone: a table or defined
  name, no `sheet` and no `ref`) and `type="external"` in 6 (with
  `connectionId`; these hold `cacheHierarchies`, `kpis`,
  `dimensions`, `measureGroups` and `maps`: a data model); every one
  carries `refreshedBy`, `refreshedDate`, `createdVersion`,
  `refreshedVersion`, `minRefreshableVersion` and `recordCount`;
  `cacheFields` 8 holding 28 `cacheField`, `name` and `numFmtId` on
  all 28, `caption` 15, `hierarchy` 13, `level` 15; 2 cache
  definitions have a rels part (one `pivotCacheRecords` relationship
  each), 6 none. `<pivotCaches>` in 4 workbooks holding 8
  `pivotCache` (`cacheId`, `r:id`). `refreshedBy` is a person's name
  and is corpus content: the type never carries it.
- `Package` (`package.rs`): `content_types()` parses
  `[Content_Types].xml` once and lends it, `ContentTypes::of(name)`
  giving an entry's type through its override or its extension's
  default; `relationships(source)` parses a part's rels once and
  lends them, `Relationships::of_kind`, `first_of` and
  `target_part` resolving targets, `Relationship.external` the
  `TargetMode`; `workbook()` gives `Workbook { sheets: Vec<Sheet {
  name, sheet_id, rel_id, state, part, kind }>, ... }`;
  `archive().entries()` lists every entry by name. The relationship
  type constants `WORKSHEET`, `CHARTSHEET`, `DIALOGSHEET`, `THEME`,
  `TABLE` (features `rowcol`, `tables`, `sheetprops` with
  `metadata`), `PIVOT_TABLE` (features `rowcol`, `tables`),
  `COMMENTS`, `VML_DRAWING`, `THREADED_COMMENTS`, `DRAWING`,
  `IMAGE`, `HYPERLINK` exist, transitional and strict. The tool's
  `--survey` (`specs/0009`) counts entries, overrides, rels parts
  and resolved targets of the package layer and prints `survey`;
  it names no part kind. `Area { first: CellRef, last: CellRef }`
  (`cells.rs`) parses `A1:B2` and `A1`. With the feature `read`,
  `sheet_read.rs` streams a worksheet once for its settings
  (`specs/0027`). 108 `pub fn` on `Package`; the API listing holds
  567 rows (`records/api-review-m5p1-2026-10-12-6f51899.md`).
- ECMA-376 Part 1, read by planning for the element and attribute
  names below (the clause numbers are not cited here; the
  implementing session cites the ones it reads): `pivotTableDefinition`
  (`name` and `cacheId` required); `location` (`ref`,
  `firstHeaderRow`, `firstDataRow`, `firstDataCol` required,
  `rowPageCount`, `colPageCount` optional); `pivotField` (`axis` one
  of `axisRow`, `axisCol`, `axisPage`, `axisValues`, the simple type
  `ST_Axis`; `dataField` boolean; `name` optional) in `pivotFields`,
  whose order is the cache's `cacheFields` order; `rowFields` and
  `colFields` holding `field` (`x`, a signed integer, `-2` the
  values field); `pageFields` holding `pageField` (`fld`);
  `dataFields` holding `dataField` (`fld` required; `name`
  optional; `subtotal` one of `average`, `count`, `countNums`,
  `max`, `min`, `product`, `stdDev`, `stdDevp`, `sum`, `var`,
  `varp`, `ST_DataConsolidateFunction`, default `sum`);
  `pivotCacheDefinition` (`r:id` to the records part optional;
  `recordCount` optional); `cacheSource` (`type` one of
  `worksheet`, `external`, `consolidation`, `scenario`,
  `ST_SourceType`; `connectionId`); `worksheetSource` (`ref`,
  `name`, `sheet`, `r:id`, all optional); `cacheField` (`name`
  required); `externalReferences` in the workbook holding
  `externalReference` (`r:id` required) in order; `externalLink`
  holding one of `externalBook` (`r:id` to the link part's own
  relationship of type `externalLinkPath`, its `Target` the file as
  written, `TargetMode` `External`; children `sheetNames` of
  `sheetName` (`val`), `definedNames` of `definedName` (`name`,
  `refersTo`, `sheetId`), `sheetDataSet` of cached values),
  `ddeLink` (`ddeService`, `ddeTopic`, holding `ddeItems`) or
  `oleLink` (`r:id` as `externalBook`'s, `progId`, holding
  `oleItems`). The relationship types: `.../relationships/
  externalLink` (workbook to the link part), `.../relationships/
  externalLinkPath` (the link part to the file), `.../relationships/
  pivotCacheDefinition`, `.../relationships/pivotCacheRecords`,
  `.../relationships/chart`, `.../relationships/slicer`,
  `.../relationships/timeline`, each with its strict spelling
  under `purl.oclc.org/ooxml/officeDocument/relationships/`; the
  `vbaProject` content type is `application/vnd.ms-office.
  vbaProject`. Sparkline groups: `x14:sparklineGroups` (namespace
  `http://schemas.microsoft.com/office/spreadsheetml/2009/9/main`)
  inside a worksheet's `<extLst>`, each `sparklineGroup` holding
  its `sparklines`. Where the implementing session's reading of
  the standard differs from a line above, it says so in the report
  and planning rules.
- openpyxl, planning's reading of 3.1, read only: it keeps every
  pivot table as `PivotTable` objects on `ws._pivots` with the cache
  (`TableDefinition`, `CacheDefinition`) modelled attribute by
  attribute and written back; external links as `ExternalLink` on
  `wb._external_links` (`externalBook` with `sheetNames`,
  `definedNames`, `sheetDataSet`; `ddeLink`, `oleLink`), written
  back; it counts no part kinds, has no survey, and reads
  sparklines not at all (`extLst` is dropped on write). emend reads
  what the outline names, writes nothing back and passes every part
  through.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. Under decision 1 A
everything below sits behind the Cargo feature `read`, off by
default: three new calls on `Package`, their types, and the
relationship type constants the readers need (`EXTERNAL_LINK`,
`EXTERNAL_LINK_PATH`, `PIVOT_CACHE_DEFINITION`, `CHART`, `SLICER`,
`TIMELINE`, each transitional then strict; `PIVOT_TABLE` and
`TABLE` gain `read` to their feature gates), all gated. The core
builds, tests and links without it; the tool turns it on. Nothing
existing changes signature.

### The survey

`Package::part_kinds(&mut self) -> Result<PartKinds, Error>` reads
the content types (parsed once already) and every entry's name, and
counts each entry whose content type, through its override or its
extension's default, is one the table below names; it reads no other
part. `PartKinds` (derived `Clone`, `Debug`, `Default`, `PartialEq`,
`Eq`), every field `u32`:

| Field | Content type (the `application/vnd.` prefix dropped) |
| --- | --- |
| `worksheets` | `openxmlformats-officedocument.spreadsheetml.worksheet+xml` |
| `chartsheets` | `...spreadsheetml.chartsheet+xml` |
| `dialogsheets` | `...spreadsheetml.dialogsheet+xml` |
| `macrosheets` | `ms-excel.macrosheet+xml`, `ms-excel.intlmacrosheet+xml` |
| `charts` | `openxmlformats-officedocument.drawingml.chart+xml` |
| `chartex` | `ms-office.chartex+xml` |
| `drawings` | `openxmlformats-officedocument.drawing+xml` |
| `images` | the entry's type begins `image/` |
| `tables` | `...spreadsheetml.table+xml` |
| `pivot_tables` | `...spreadsheetml.pivotTable+xml` |
| `pivot_caches` | `...spreadsheetml.pivotCacheDefinition+xml` |
| `pivot_cache_records` | `...spreadsheetml.pivotCacheRecords+xml` |
| `external_links` | `...spreadsheetml.externalLink+xml` |
| `vba_projects` | `ms-office.vbaProject` |
| `slicers` | `ms-excel.slicer+xml` |
| `slicer_caches` | `ms-excel.slicerCache+xml` |
| `timelines` | `ms-excel.timeline+xml` |
| `timeline_caches` | `ms-excel.timelineCache+xml` |
| `comments` | `...spreadsheetml.comments+xml` |
| `threaded_comments` | `ms-excel.threadedcomments+xml` |
| `persons` | `ms-excel.person+xml` |
| `custom_xml` | `openxmlformats-officedocument.customXmlProperties+xml` (one per custom XML item) |
| `connections` | `...spreadsheetml.connections+xml` |
| `query_tables` | `...spreadsheetml.queryTable+xml` |
| `custom_properties` | `openxmlformats-officedocument.custom-properties+xml` |
| `printer_settings` | `openxmlformats-officedocument.spreadsheetml.printerSettings` |
| `calc_chains` | `...spreadsheetml.calcChain+xml` |
| `other_typed` | an entry with a type the table does not name, the content types part excluded |
| `untyped` | an entry with no type |

The strict content types (`purl.oclc.org`) count beside the
transitional for the kinds that have one, by the same dropped
prefix rule the relationship constants use; the type is compared
whole after the prefix, by exact bytes. Nothing is inferred from
an entry's name. The sum of every field equals the entry count
less one (the content types part), which a test asserts on a
built package and the sample asserts on every corpus workbook.

`Package::sheet_parts(&mut self, part: &[u8]) -> Result<SheetParts,
Error>` answers for one sheet part: from the sheet's relationships,
`drawings`, `comments`, `vml_drawings`, `tables`, `pivot_tables`,
`slicers`, `timelines`, `printer_settings` (each the count of
relationships of that kind) and `external_hyperlinks` (`hyperlink`
relationships whose `TargetMode` is `External`); through each
drawing's relationships, `charts` (`chart`) and `images` (`image`);
and `sparkline_groups`, counted by one streamed pass over the sheet
part that counts `sparklineGroup` elements in the 2009/9 namespace
anywhere in the part and holds nothing else (a chartsheet or a
sheet with no `<extLst>` counts 0). A sheet with no rels part
counts 0 of each. `SheetParts` derives as `PartKinds` does, every
field `u32`. A relationship target naming no entry is counted by
its relationship, not resolved (the survey asks what the package
says it holds); `part` naming no entry is refused, "no such part".

### Pivot tables

`Package::pivot_tables(&mut self) -> Result<&[PivotTable], Error>`
reads, once, every entry whose content type is the pivot table's,
in entry order, each with the cache definition its own rels name,
and lends the list. The types, each derived `Clone`, `Debug`,
`PartialEq`:

- `PivotTable { part: Vec<u8>, sheet: Option<Vec<u8>>, name:
  String, cache_id: u32, location: Area, first_header_row: u32,
  first_data_row: u32, first_data_col: u32, cache: Option<
  PivotCache>, fields: Vec<PivotField>, row_fields: Vec<FieldRef>,
  column_fields: Vec<FieldRef>, page_fields: Vec<FieldRef>,
  data_fields: Vec<DataField> }`. `part` is the entry name; `sheet`
  the first worksheet or chartsheet of `Workbook::sheets` whose
  relationships name the part through a `pivotTable` relationship,
  `None` when none does (the corpus's workbook-related one);
  `name`, `cache_id` and the four `location` attributes as written;
  `cache` the cache definition reached through the part's first
  `pivotCacheDefinition` relationship, `None` without one; `fields`
  the `pivotField` children of `<pivotFields>` in order; the three
  axis lists the `field` and `pageField` children in order;
  `data_fields` the `dataField` children in order.
- `PivotField { name: Option<String>, axis: Option<Axis>,
  data_field: bool }`: `name` the cache field at the same position
  (`None` without a cache or past its fields; a `pivotField`'s own
  `name` attribute is not read, the cache's is the field's name),
  `axis` from `axis`, `data_field` from `dataField` (absent false).
- `Axis { Row, Column, Page, Values }` (`axisRow`, `axisCol`,
  `axisPage`, `axisValues`); `Copy`, `Eq`.
- `FieldRef { Field(u32), Values }`: a `field`'s `x` or a
  `pageField`'s `fld`, `Values` for `-2`; `Copy`, `Eq`.
- `DataField { name: Option<String>, field: u32, subtotal:
  Subtotal }`; `Subtotal { Sum, Count, Average, Max, Min, Product,
  CountNums, StdDev, StdDevP, Var, VarP }` (`Copy`, `Eq`), `Sum`
  when absent.
- `PivotCache { part: Vec<u8>, source: PivotSource, fields:
  Vec<String>, record_count: Option<u64> }`: the cache definition's
  entry name, its `cacheSource`, its `cacheField` names in order,
  its `recordCount`.
- `PivotSource { Worksheet { sheet: Option<String>, reference:
  Option<Area>, name: Option<String> }, External { connection_id:
  Option<u32> }, Consolidation, Scenario }`: `type="worksheet"`
  with the `worksheetSource`'s `sheet`, `ref` and `name` as written
  (a `worksheetSource` with none of the three is `Worksheet` with
  three `None`; its `r:id`, an external workbook, is not followed);
  `external` with `connectionId`; the other two by name alone.

Both parts are streamed: the reader keeps the attributes above and
the four lists and walks every other child past with its subtree
(`rowItems`, `colItems`, `formats`, `chartFormats`, the hierarchies,
`extLst`, a cache's `cacheHierarchies`, `kpis`, `dimensions`,
`measureGroups`, `maps`, and every `cacheField`'s `sharedItems`),
so the hold is the model, bounded by the fields, not by the cached
records; the records part is never opened. A `cacheId` is not
joined to the workbook's `<pivotCaches>`: the cache is reached
through the part's own relationship, which every corpus pivot table
has, and the id is reported as written. Refused, "a pivot table
cannot be read": a root other than `pivotTableDefinition` or
`pivotCacheDefinition` in the main namespace; no `name`, no
`cacheId`, no `location` or a `location` lacking any of its four
attributes; a `ref` `Area::parse` refuses; an `axis`, `subtotal` or
`type` outside its enumeration; an `x`, `fld`, `cacheId` or
`connectionId` that is not an integer of its type (`x` is `-2` or
a `u32`); a `dataField` or `pageField` `fld`, or a `field` `x`,
past `fields`; a `cacheField` with no `name`; two `location`,
`pivotFields`, `cacheSource` or `cacheFields` children. A
`pivotCacheDefinition` relationship naming no entry is "no such
part".

### External links

`Package::external_links(&mut self) -> Result<&[ExternalLink],
Error>` reads, once, the workbook's `<externalReferences>` children
in order, each `r:id` resolved through the workbook's relationships
of type `externalLink`, and lends the list; a workbook with no
`<externalReferences>` gives an empty slice. `ExternalLink { part:
Vec<u8>, kind: ExternalLinkKind }`, `ExternalLinkKind { Book {
target: String, sheet_names: Vec<String>, defined_names: u32 },
Dde { service: String, topic: String }, Ole { prog_id: String,
target: String } }`, each derived `Clone`, `Debug`, `PartialEq`,
`Eq`. `Book`'s `target` is the `Target` of the link part's
relationship the `externalBook`'s `r:id` names, as written (the
form Excel writes is a relative or `file:///` path, percent-encoded
where it must be; nothing is decoded, as the package layer's limit
says), `sheet_names` its `sheetName` `val`s in order,
`defined_names` the count of its `definedName` children; `Ole`'s
`target` is the same relationship's `Target` for the `oleLink`'s
`r:id`; `Dde`'s two are `ddeService` and `ddeTopic`. The
`sheetDataSet`, `ddeItems` and `oleItems` are walked past. Refused,
"an external link cannot be read": an `externalReference` with no
`r:id`, or an `r:id` naming no workbook relationship or one of
another type; a root other than `externalLink` in the main
namespace; none or more than one of the three children; an
`externalBook` or `oleLink` with no `r:id`, or an `r:id` naming no
relationship of the link part or one whose `TargetMode` is not
`External`; a `sheetName` with no `val`; a `ddeLink` lacking either
attribute; an `oleLink` with no `progId`. A relationship naming no
entry is "no such part".

### Errors

Two new `Error::Package` messages, in the catalogue as the earlier
parts' are, marked by feature:

`read`:
- "a pivot table cannot be read" (the pivot reader's refusals
  above)
- "an external link cannot be read" (the external link reader's
  refusals above)

Shared: "no such part" for a relationship naming a part the
archive lacks and for `sheet_parts` of a name the archive lacks;
part 2's unbound-prefix refusal; `Error::Xml` for bytes that are
not UTF-8 or not well formed. The survey adds no message: it reads
nothing the content types read has not refused already. Nothing is
checked twice.

### 32-bit and streaming

Every count is `u32` or `u64`; `record_count` is `u64` because a
cache's records may exceed `u32` on paper. The pivot and external
link parts are streamed and the models held, bounded by their
fields, sheet names and defined-name counts (the corpus's largest
pivot table part is 31286 bytes, its largest cache 8703, with 28
cache fields in all); the records parts and the `sheetDataSet` are
never read. The survey holds the counts and reads no part beyond
the content types. The sparkline pass over a sheet streams the part
and holds a count.

### Size

Under decision 1 A `probe-emend-read-write-xml` grows: after its
reads today, `part_kinds` (the worksheet count printed),
`sheet_parts` of its first sheet (the drawing count printed),
`pivot_tables` (the count printed) and `external_links` (the count
printed). The size log measures as part 1's did, classifies the
twenty-eight probes of `records/size-m5p1-2026-10-12-6f51899.md`
against it, prints the feature's cost (the read probe minus the
format probe rebuilt at the same commit), the core's growth (the
format probe at the commit minus the same probe rebuilt at 6f51899,
both printed; expected 0), the budget against 458752, and a cause
per probe; the cause control and the manifest cause listing stay.

### Harness runs

`tools/emend-copy` gains three operations, given after the two
paths as the others are; none takes a reference, name or number
from its command line. No reference, no name, no path and no text
from the corpus is printed: the samples print counts, flags and
digests. A pivot table's name, a cache field's name, a sheet name
and a link target are corpus content and are never printed; the
samples count and digest them (64-bit FNV-1a over each value and a
newline, as `--colors-sample` digests its lists).

- `--kinds-sample <in> <out>` (P1): writes the input back unedited;
  `part_kinds`, then `sheet_parts` of every sheet of
  `Workbook::sheets`; prints `kinds` with every `PartKinds` field by
  its name, `entries` (the archive's entries), `kinds_sum` (every
  field summed, equal to `entries` less 1, asserted), `sheets`
  (sheets walked), then every `SheetParts` field summed over the
  sheets under the prefix `sheet_` (`sheet_drawings`,
  `sheet_charts`, ... `sheet_sparkline_groups`). Expected results:
  every entry `identical`, `files_identical` `yes`; `sheet_charts`
  at or below `charts` and `sheet_tables` equal to `tables` in
  every workbook (a table is reached from one sheet; a chart part
  may be a chartsheet's or unreferenced).
- `--pivot-sample <in> <out>` (P2): writes the input back unedited;
  `pivot_tables`; prints `pivots` with `pivot_tables`, `with_sheet`
  (`sheet` `Some`), `with_cache`, `source_worksheet`,
  `source_external`, `source_other`, `source_named`
  (`Worksheet` with a `name`), `source_ref` (with a `reference`),
  `cache_fields`, `records_known` (`record_count` `Some`), `fields`,
  `named_fields` (`name` `Some`), `axis_row`, `axis_column`,
  `axis_page`, `axis_values`, `data_field_flags`, `row_fields`,
  `column_fields`, `page_fields`, `values_refs` (`FieldRef::Values`
  across the three lists), `data_fields`, `named_data_fields`,
  `subtotal_sum`, `subtotal_other`, `location_cells` (the cells of
  every `location` area summed), `digest_names` (over the pivot
  table names), `digest_fields` (over the cache field names).
  Expected results: every entry `identical`, `files_identical`
  `yes`; `pivot_tables` equal to the content-type count the kinds
  sample printed for the same workbook; `with_cache` equal to
  `pivot_tables` (every corpus pivot table has the relationship);
  `named_fields` equal to `fields` where `with_cache` equals
  `pivot_tables` and the cache's field count covers the table's.
- `--extlink-sample <in> <out>` (P3): writes the input back
  unedited; `external_links`; prints `extlinks` with `links`,
  `books`, `dde`, `ole`, `sheet_names`, `defined_names`,
  `digest_targets` (over `Book` and `Ole` targets), `digest_sheets`
  (over the sheet names). Expected results on the corpus: every
  entry `identical`, `files_identical` `yes`, `links` 0 in every
  workbook (the fact above); the generated fixture below is where
  the reader is proven through the binary.

The run log's own check over the extracted input and output parts,
with `ElementTree`, `zipfile` and byte spans and a control that
reports a planted difference per check, printing no name, no
reference, no path and no text from the corpus:

- kinds: every entry byte-identical; every `PartKinds` field
  against the check's independent count over `[Content_Types].xml`
  and the entry names (the override and default rule typed into the
  check); every summed `SheetParts` field against the check's
  counts over the sheet rels parts, the drawing rels parts and the
  `sparklineGroup` elements of every worksheet; `kinds_sum` against
  the entry count.
- pivots: every entry byte-identical; every count against the
  check's own over the pivot table and cache definition parts (the
  attributes, children and relationships named in "Facts"); the
  check digests the names and cache field names itself with the
  same FNV-1a and compares the two digests with the line's.
- extlinks: every entry byte-identical; `links` 0 against the
  absence of `<externalReferences>` and of the content type in
  every workbook. Then the generated fixture: a workbook the run
  log's own generator writes with `zipfile` under the phase
  directory (fictional: one sheet of three cells; three external
  link parts in `<externalReferences>` order, an `externalBook` to
  a relative target of the generator's own naming with two sheet
  names and one defined name and a `sheetDataSet`, a `ddeLink`
  with a service and a topic, an `oleLink` with a `progId` and an
  external target; the generator's text printed in the log), the
  sample run on it on both targets, its line checked value by value
  against the generator's constants (`links` 3, `books` 1, `dde` 1,
  `ole` 1, `sheet_names` 2, `defined_names` 1, the two digests
  computed by the check from the constants) and the output
  byte-identical to the fixture; the fixture is not corpus content
  and its values may be printed. The fixture is written by the
  generator at run time and is not committed.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. Every phase's run log
runs the edit open check on the host over that phase's keep, names
the phase directories it compares against by their record commit,
prints the devices of `TMPDIR` and of the corpus directory, and
counts the calc-chain workbooks with the rule covering them. The
Excel check shows nothing new for a read; no sample of this part
joins its list.

## Phases

As `specs/0001`: a doc commit first, code commits, one record
commit, the scan last, at most two independent checks per phase, a
report, STOP. One target root per phase,
`<home>/.cache/emend-build/m5p2-<phase>/target`. P1's gate prunes
part 1's four roots (`<home>/.cache/emend-build/m5p1-p1/target`,
`m5p1-p2/target`, `m5p1-p3/target` and `m5p1-close/target`,
`specs/0032` close-out ruling 4), each listed with `du -sb` and
removed with `rm -r`; the close-out's `gen.py` and logs stay. The
three logged scripts run in the phase directory with
`scripts/counting.sh` copied into `<scratch>` by `git show`; a dry
run of each precedes its logged run; a refused or failed attempt is
hashed in the step's own log before a rerun (`specs/0032` P3
verification ruling 1). From this part's first logged script
(`specs/0032` close-out ruling 1 and P3 verification ruling 4):
every logged script prints `input: TMPDIR=<scratch>/tmp`, checks
the directory `mktemp` makes is under it with a control that
refuses one that is not, and every Python call is `python3 -I -B`;
the gate checks that no committed script under `scripts/` calls
`python3` without `-B`, with a control. Logs in
`records/logs/m5p2-<phase>/` (`gate.log`, `test.log` on both
targets with the feature sets off, `read`, `read,rules` and every
feature, `negative.log`, `run.log`, `scan.log`, `size.log` in P3);
every rule the earlier parts added; the scan's denylist form
whole-word. No push during the part; the close-out push follows P3
and carries the record commit 3f2082e of part 1's close-out with
this part's.

Part 2's first doc commit carries: this spec; `ROADMAP.md` with
part 1's Close row DONE (record 3f2082e) and this part's rows;
`specs/0032-read-breadth-styles.md` gaining "Planning rulings on
the close-out (2026-10-12)", the text of planning's
`m5p1-close-rulings.md` copied verbatim.

### P1: the survey

The doc commit; the prunes; the gate's two new checks; then the
relationship constants, `PartKinds`, `part_kinds`, `SheetParts`,
`sheet_parts`, and `--kinds-sample`. Tests on built packages: a
package holding one entry of each kind in the table (an override
each, an image by its extension's default, a strict-typed worksheet
counted with the transitional) with every field asserted and the
sum rule; an untyped entry and an entry of an unnamed type; a sheet
with each relationship kind and a drawing with a chart and an
image; an external and an internal hyperlink; a sheet with two
sparkline groups in `extLst` and one with none; a chartsheet; a
sheet with no rels part; `sheet_parts` of a name the archive lacks
refused; the tool's line. The gate as part 1 P1's (the crate built
and tested with the feature sets above on both targets, the `pub
fn` lines listed off and on) plus the two checks above. The
negative log through the binary: the sample on a fixture whose
`[Content_Types].xml` is not well formed (refused, with a control).
The run log: `--kinds-sample` on both targets over the corpus, the
checks as "Harness runs", the keep and the edit open check,
recorded as `records/m5p2-p1-<utc-date>-<shortsha>.md` and `.txt`.

### P2: pivot tables

The doc commit (the P1 verification rulings); the carry, if any;
then the pivot types, `pivot_tables`, the message, and
`--pivot-sample`. Tests on built packages: a pivot table with every
modelled attribute and child, a worksheet-sourced cache with
`sheet` and `ref`, one with `name` alone, an external cache, a
consolidation and a scenario source; the field join by position
and a cache with fewer fields than the table; every `Axis`, every
`Subtotal` and the default; `-2` as `Values` in each axis list; a
table with no cache relationship; the workbook-related table with
`sheet` `None` beside a sheet-related one; unmodelled children
walked past (a `rowItems` and an `extLst` with content); the
refusals (no `name`, no `cacheId`, a `location` lacking `ref`, a
`ref` of `A1:`, an `axis` of `axisX`, a `subtotal` of `mean`, a
`type` of `database`, an `x` of `-3`, a `fld` past the fields, a
`cacheField` with no `name`, two `location`); the strict namespace;
the tool's line. The gate as P1's. The negative log through the
binary: the sample on a fixture whose pivot table names a `fld`
past its fields (refused, with a control). The run log:
`--pivot-sample` on both targets over the corpus, the checks as
"Harness runs", the keep and the edit open check, recorded as
`records/m5p2-p2-<utc-date>-<shortsha>.md` and `.txt`.

### P3: external links, the catalogue, the probe, the size record

The doc commit (the P2 verification rulings); the carry, if any;
then the external link types, `external_links`, the message, and
`--extlink-sample`; then the `package` header's catalogue and
limits for this part (the messages by feature; the survey counts
what the package says it holds; the cache reached by relationship,
`cacheId` not joined; the `sheetDataSet` and records never read;
targets as written); the probe grown as "Size" says; the size log,
recorded as `records/size-m5p2-<utc-date>-<shortsha>.md`; the API
listing rerun as part 1 P3 did, recorded as
`records/api-review-m5p2-<utc-date>-<shortsha>.md` (the new items
listed, the count stated); `CHANGELOG.md`'s "Added" gains one
bullet citing this spec, under `specs/0031`'s citation check. Tests
on built packages: a workbook with three references in order, each
kind read; an `externalBook` with no sheet names and no defined
names; a `sheetDataSet` walked past; the strict namespace; the
refusals (an `externalReference` with no `r:id`, an `r:id` of the
wrong type, two children, an `externalBook` whose relationship is
internal, a `sheetName` with no `val`, a `ddeLink` with no topic,
an `oleLink` with no `progId`); no `<externalReferences>` giving an
empty slice; the tool's line. The negative log through the binary
for every message of the part reachable there, the unreachable
ones stated with the test that covers each; the extlink sample on
a fixture whose `externalBook` names an internal relationship
(refused, with a control). The run log: `--extlink-sample` on both
targets over the corpus and on the generated fixture, the checks as
"Harness runs"; then all three samples again on both targets with
the size log's release binaries, byte-identical to P1's, P2's and
this phase's debug runs by sha256, recorded as
`records/m5p2-p3-<sample>-<utc-date>-<shortsha>.md` and `.txt`. The
open record of the part's keep.

### Close-out push (after P3)

As part 1's (`specs/0032` "Close-out push (after P3)" and its
close-out rulings): the generator transforms part 1's scripts
(`gen.py` bb9d9d09 and the three it printed, carried cmp-equal),
its numbered changes continuing from 33; the doc commit holds this
spec and `ROADMAP.md`; the base is part 1's close-out doc commit
c02942e, named by its subject ("M5 part 1 close-out: commit
planning docs (the P3 verification and rulings)");
`PREV_RECORD_PATTERN` is `^M5 part 1 close-out record` (3f2082e),
in the range exactly once; the close-out doc commit's subject is
"M5 part 2 close-out: commit planning docs (the P3 verification and
rulings)"; `CARGO_TARGET_DIR` printed and checked as
`<home>/.cache/emend-build/m5p2-close/target`; no Excel-record
check, the stray-record count 0 kept; the scan's denylist form
whole-word; the record commit "M5 part 2 close-out record: the
push, the CI run and the scan" holding the three logs only; pre
mode on a clean tree; one independent check before the report. The
push carries 3f2082e and this part's commits, on planning's word
and the maintainer's authorisation typed in the implementing
session.

## Planning rulings on P1 (2026-10-12)

The implementing session's report (its file, sha256 f2c8c6d2...)
listed twelve departures and findings. Rulings, numbered as the
report numbers them:

1. `custom_xml` 30 in 10 workbooks: the sample is RIGHT and the
   "Facts" bullet above was wrong. The bullet's 60 counted every
   override under `customXml/` (the item and its properties part,
   two per item); the table's rule is one `customXmlProperties+xml`
   per item, and this section's own relationship line reads
   `customXml` 30. Planning recounted on 2026-10-12: 30
   `customXmlProperties+xml` overrides in 10 workbooks, 60
   overrides under `customXml/`. Not a finding against the code;
   no change. P2's run log carries 30 in 10 as planning's figure.
2. `SLICER` and `TIMELINE` written as one form twice: accepted.
   Microsoft's relationship types have no strict spelling; the
   "Facts" sentence "each with its strict spelling" is wrong for
   those two and is corrected by this ruling.
3. Strict content types: accepted. A strict package writes the
   transitional content types (only namespaces and relationship
   type URIs differ), so one spelling counts both; the "The survey"
   paragraph "The strict content types ... count beside the
   transitional" is superseded by this ruling, and the test
   `a_strict_package_counts_its_worksheet_with_the_transitional`
   covers the case P1's test list called "a strict-typed worksheet
   counted with the transitional".
4. `PRINTER_SETTINGS`: accepted, a constant `SheetParts` needs; it
   joins the catalogue in P3.
5. `COMMENTS`, `VML_DRAWING`, `DRAWING`, `IMAGE` and `HYPERLINK`
   gated with `read` as well: accepted, `sheet_parts` needs them
   under `read` alone.
6. The scripts commit (`python3 -I -B` in the three committed
   scripts): accepted as this phase's commit; the gate self-test
   was due and passed (50 cases).
7. The keep's one new pair from the tool test: accepted; the open
   record holds it (1100 pairs, findings 0).
8. TMPDIR under `<phase>/test/tmp` and `<phase>/negative/tmp`, and
   the gate's consumer directory under `<phase>/gate`: accepted,
   each under the phase directory and printed with its root line.
9. The P5 plant's wording: accepted (FACTS.md is not committed).
10. The tool test's wrong comment ("the workbook's external
    hyperlink is not a sheet's and is not counted"; the hyperlink
    is the sheet's and the test asserts 1): a FINDING against the
    tool commit. Fixed in P2's tool commit by rewriting the comment
    to what the test asserts; P2's report names the fix. No
    separate commit.
11. Release binaries in P1's corpus runs: the carried form is
    right. P3's sentence "byte-identical to P1's, P2's and this
    phase's debug runs" is corrected by this ruling to "P3's
    release binaries' outputs byte-identical to P1's and P2's
    recorded outputs and to this phase's own runs"; the profile
    the earlier runs used is whatever their logs state.
12. The negative log's plain `mktemp` file after the TMPDIR check:
    accepted for P1. From P2, every `mktemp` call of a logged
    script, file or directory, is checked under TMPDIR with the
    same control, or the script makes no plain file.

The report's own corrections after the independent check (the
self-test's last line, the run log's fixture reading 2, the rm
list, the end times) are accepted as made; nothing in the code or
the logs changed after the check.

### Planning verification of P1 (2026-10-12)

Verified from primaries: the doc commit a1f484c holds the three
docs at the hashes planning handed over (b9f13b40, 59caeef2,
f70d0ab3); the crate, tool and scripts commits change only the
files the report names (the scripts commit: four `python3` lines);
every commit message is ASCII and ends with the Co-Authored-By line
alone; the record commit's eleven files hash as the report says;
each log's last line is as reported; the gate pruned the four part
1 target roots (the du lines as reported) and read "files scanned:
18; Python calls: 4; calls without -B: 0" with its base control at
4; the run log's corpus totals match planning's recount field by
field (custom_xml per ruling 1); the scan read 0 hits over the four
commits. Planning's own build at 3865f30 (worktree, offline): crate
tests on both targets 515, 594, 632 and 966; workspace 1168;
clippy and doc 0 warnings; `--kinds-sample` over the 27 corpus
workbooks on both targets, every output and every line byte-equal
to the run directory's; planning's privacy scan of the five commits
through the record: 1624 blobs, 0 findings. P1 DONE.

## Planning rulings on P2 (2026-10-12)

The implementing session's report (its file, sha256 ee575534...)
listed ten departures and findings. Rulings, numbered as the
report numbers them:

1. `data_field_flags` 10 in 3 workbooks: the sample is RIGHT and
   the "Facts" bullet above ("none with `dataField`") was wrong.
   Planning recounted on 2026-10-12: 10 `pivotField` elements
   carry `dataField="1"`, in 3 workbooks, one per `dataField`
   child of `<dataFields>`, which is what the attribute marks.
   The bullet is corrected by this ruling; not a finding against
   the code; no change. P3's run log carries 10 in 3 as
   planning's figure, so every pivot figure is planning's (18 of
   18).
2. `PivotTable::sheet` holding the sheet's part name: that is the
   spec's type (`Option<Vec<u8>>`, "the first worksheet or
   chartsheet ... whose relationships name the part"); not a
   departure. The sheet's name is reached through
   `Workbook::sheets` by its part.
3. Booleans other than `1`, `true`, `0` and `false` refused:
   accepted. Those four are the whole lexical space of
   `xsd:boolean`; the refusal list's "as its type" covers it. No
   change.
4. The catalogue entry for "a pivot table cannot be read" deferred
   to P3: accepted, as the brief said; P3's catalogue commit adds
   it beside the external link message.
5. N2 (the cache relationship naming no entry, "no such part")
   beside the spec's N1: accepted, as P1's second case was.
6. TMPDIR: the gate, run and scan at `<phase>/tmp`, the test and
   negative at their carried `<build>/tmp` under the phase
   directory: accepted (ruling 8 of P1).
7. The counting library's control `mktemp -d` (scripts/counting.sh)
   with no under-TMPDIR line: a FINDING against the logs' form,
   not the code. Ruling 12 of P1 is read as the logged script's
   own calls; the library's one call lies under TMPDIR because
   every script exports TMPDIR before sourcing it, and that is
   stated, not shown. From P3 the gate log prints one line checking
   the library's directory under TMPDIR, with a control, once per
   phase; the other logs need no line for it.
8. A cache with no `cacheSource` refused: accepted; it follows from
   `PivotCache::source` not being an `Option`, and the refusal
   list is read to include it. No change.
9. Every kept numeric attribute that does not parse refused
   (`recordCount`, `firstHeaderRow`, `firstDataRow`,
   `firstDataCol` beside `x`, `fld`, `cacheId` and
   `connectionId`): accepted. The reader keeps no number it could
   not parse, and drops nothing silently; the refusal list is read
   to cover every numeric attribute the types carry. No change.
10. Per table against per cache (`cache_fields` and
    `digest_fields` per table, so one worksheet cache shared by
    two tables counts twice: 3 against 2, 32 against 28):
    accepted. The tool's line is per table by its definition; the
    run log compares each figure with its own kind. The shared
    cache is the session's reading, arithmetically consistent, and
    needs no log line.

Also ruled: the gate end time is read from the committed log
(20:20:30Z), the report's correction giving both stamps accepted;
the test fork's stray `/tmp/x`, removed with `rm -f` and disclosed,
is accepted as disclosed, and from P3 a fork's temporary files are
under the phase directory as the session's own are; the unlogged
dry runs, the target root sizes and the fork rm list stand as the
session's statements, as before. Nothing in the code or the logs
changed after the independent check.

### Planning verification of P2 (2026-10-12)

Verified from primaries: the doc commit a9354ec holds specs/0033
and ROADMAP.md at the hashes planning handed over (5067dfcc,
448a94ba) and leaves specs/0032 as a1f484c had it; the crate commit
adds `pivot.rs`, its tests and the `package.rs` lines the report
names and nothing else, the reader as "Pivot tables" says (content
type in entry order, the sheet join in `<sheets>` order, the first
`pivotCacheDefinition` relationship, "no such part" for its missing
entry, the records part never opened); the tool commit adds
`--pivot-sample`, its two tests and the comment fix of ruling 10;
every commit message is ASCII and ends with the Co-Authored-By line
alone; the record commit's ten files and the report hash as the
report says; each log's last line is as reported; the gate read 3
mktemp calls and 0 plain files, "files scanned: 18; Python calls:
4; calls without -B: 0" and no prune; the negative log's N1 line is
exactly the message, with B0 and N2; the run log's corpus totals
match planning's recount field by field (`data_field_flags` per
ruling 1), the kinds figures 24 of 24, the keep one new pair and 0
changed, the open check 0 findings; the scan read 0 hits over the
three commits. Planning's own build at 61d23fc (worktree, offline):
crate tests on both targets 515, 608, 646 and 980; workspace 1184;
clippy and doc 0 warnings; `--pivot-sample` with release binaries
over the 27 corpus workbooks on both targets, every output and
every line byte-equal to the run directory's, the totals planning's
own; planning's recount of `dataField` flags 10 in 3; planning's
privacy scan of the four commits through the record: 1633 blobs, 0
findings. P2 DONE.

## Decisions for the maintainer

Decided by the maintainer 2026-10-12: 1, 2 and 3 as recommended (A
in each), typed in the planning session.

1. Packaging ("Size and packaging"): A, everything under `read`
   (recommended); B, a new feature `pivots` for the two readers.
2. External links with no corpus coverage: A (recommended), the
   reader is built and proven on built packages in tests and on the
   generated fixture through the binary, the corpus proving the
   count 0, as "Harness runs" says; B, external links leave this
   part for "Out of scope" until a corpus workbook holds one, and
   P3 keeps the catalogue, probe, size and API work alone.
3. The pivot model's depth: A (recommended), the fields by axis,
   the data fields with their subtotal, the cache's source and
   field names, as "Pivot tables" says; B, the name, location and
   source alone, with the cache field names, no axis lists and no
   data fields (`PivotField`, `FieldRef`, `DataField`, `Subtotal`
   and `Axis` dropped), a smaller reader a later part can grow.

## Out of scope

- Writing or editing a pivot table, a cache or an external link;
  refreshing a cache; reading the cached records or a link's
  `sheetDataSet` values (part 6's openpyxl comparison may price a
  cached-value read).
- Reading a slicer, a timeline, a connection, a data model
  (`powerPivotData`), a custom XML item or a VBA project beyond
  counting its part; threaded comments and persons beyond their
  counts (the corpus holds none).
- A sparkline's own ranges and formatting; `x14` conditional
  formats and validations inside `extLst`.
- Joining a pivot table's `cacheId` to the workbook's
  `<pivotCaches>`; the `pivotField`'s own `name`, `items`,
  `sharedItems`; row and column items; pivot formats and styles.
- Charts: part 3 (`charts` feature) reads them; this part counts
  them.
