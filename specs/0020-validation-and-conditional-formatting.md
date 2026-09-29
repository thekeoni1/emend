# 0020: Milestone 2 part 4b, data validation and conditional formatting

Status: APPROVED by the maintainer 2026-09-29 (planning draft the
same day): decisions 1 to 5 taken as planning recommended, in the
maintainer's reply "i agree with planning recs for all", typed in
the planning session; decision 6 recorded. Sent to the implementing
session the same day.
Context: `CLAUDE.md` ("What emend is": like openpyxl; "Dependencies
and licences": anything beyond the core behind a Cargo feature off
by default; "The fidelity harness is the acceptance instrument"),
`ROADMAP.md` ("Milestone 2", the outline approved 2026-09-23, item
4; "Part 4: the openpyxl features, in sub-parts"; "Standing items",
the Milestone 2 ceiling), `specs/0012-styles.md` and `specs/0017-
fuller-styles.md` (the styles rewrite, `Style` and its parts, the
`Font`, `Fill` and `Border` element writers), `specs/0018-rows-
and-columns.md` ("Creating an element", the schema-position table;
"The shift rule", where `dataValidation sqref` and
`conditionalFormatting sqref` already move; the one-shift rule;
the merge overlap rule as the model for a validation's), `specs/
0019-names-links-properties-protection.md` (the feature pattern,
the worksheet container pass for `<hyperlinks>`, the text rule for
a formula-like text, the P4 verification rulings 4, 7, 10 and 15
and the close-out rulings carried here), `specs/0001-bootstrap.md`
("Rules for every phase"). ECMA-376 Part 1, 18.3.1.32 and 18.3.1.33
(`dataValidation`, `dataValidations`), 18.3.1.18 and 18.3.1.10
(`conditionalFormatting`, `cfRule`), 18.3.1.11 (`cfvo`), 18.3.1.13
(`colorScale`), 18.3.1.28 (`dataBar`), 18.3.1.49 (`iconSet`),
18.8.14 (`dxf`), 18.8.15 (`dxfs`), 18.8.39 (`styleSheet`, the
order of its children), 18.18.15 (`ST_ConditionalFormattingOperator`),
18.18.21 (`ST_DataValidationOperator`), 18.18.20
(`ST_DataValidationType`), 18.18.12 (`ST_CfType`), are cited where
the design leans on them; no text is copied from them.

## Why a part 4b

Part 4a (`specs/0019`, DONE and pushed 2026-09-29) took the four
capabilities that live in parts the crate already rewrites. This
part takes the two that add rules to a worksheet: a data validation
constrains what a user may enter in a cell, and a conditional
format changes how a cell looks by a rule. Both are lists of
elements on the worksheet root, both name their cells by a `sqref`
list the crate already parses and moves under a shift, and the
second needs a differential style in the styles part, which the
part 2 rewrite grows as it grows fonts and fills. Tables (4c) and
comments and images (4d) follow, each with its own spec.

## Goal

A caller can add a data validation to one or more areas of a
worksheet (a list of allowed values, a whole number, a decimal, a
date, a time, a text length or a custom formula, with an operator
and one or two formulas, an input message and an error message),
remove one, and read the ones a worksheet has; and can add a
conditional format to one or more areas (a comparison of the cell's
value, a formula, blank or not blank, duplicate or unique values, a
top or bottom count or percent, above or below the average, each
with a differential style; or a colour scale, a data bar or an icon
set, which carry their look with them), remove one, and read the
ones a worksheet has; on an opened workbook or on `Package::new()`'s,
with everything the edit does not touch passed through byte for
byte. The comparison point is openpyxl's `DataValidation` and
`ws.add_data_validation`, `DifferentialStyle`, `ws.conditional_
formatting.add` with `CellIsRule`, `FormulaRule`, `ColorScaleRule`,
`DataBarRule` and `IconSetRule`, with the same limits: a formula is
stored as text, never parsed or evaluated; a rule changes nothing
about the cell's value or style; what Excel shows for a rule is
Excel's.

Two things are read back as typed values, since a caller cannot
remove a rule without knowing its areas: the validations (areas,
kind, operator, formulas, the flags and messages) and the
conditional formats (areas, rule, differential style index,
priority, stop-if-true). The differential styles are not read back;
the harness counts them.

The harness is the acceptance: over the corpus each sample leaves
every other entry byte-identical, the edited parts identical outside
the elements the edit names, and those elements as the rule gives
them, as an independent parser in the run script shows; every
output opens in LibreOffice; the rules read back.

## Size and packaging (decision 1, before any code)

The budget figure after part 4a is 435224 section bytes against
the Milestone 2 ceiling of 458752 (`records/size-m2p4a-2026-09-29-
3e67745.md`): 23528 of headroom, held for fixes. The outline puts
part 4 behind features, so nothing here is in the core budget.
Planning's estimate for the design below, made with part 4a's
measured ratio in view (its feature cost 50404 against a guess of
15000 to 20000, about 2.5 times; `specs/0019` P4 verification
ruling 11), is 40000 to 60000 section bytes whole: the validations'
read, write and overlap check with the container (12000 to 18000),
the differential styles through the styles rewrite (6000 to 9000),
the rule kinds with their attributes and children and the priority
rule (15000 to 22000), the read-back of both lists (7000 to 11000).
The core's growth for the part is what the feature gates and the
styles rewrite's `dxfs` hook add, printed by the size log and
expected near 0; above 2000 section bytes planning is told in the
report.

- A (recommended): one Cargo feature `rules`, off by default,
  declared beside `dates`, `rowcol` and `metadata` with no
  dependency of its own, holding both capabilities and the
  differential styles; one probe measuring its cost beside the
  core; the core probe unchanged. One feature, one figure, one
  measurement, as part 4a chose.
- B: two features, `validation` and `conditional`, two probes. A
  consumer that wants a drop-down list alone links less; twice the
  size-log work, and the differential styles belong to one of them
  while the styles rewrite hook serves both.
- C: both capabilities folded into `metadata`. One feature fewer;
  a consumer that wants a hyperlink then carries the rules, and
  part 4a's measured figure stops being a figure for anything.

## Facts planning checked (2026-09-29)

Over the 27 corpus workbooks (26 `.xlsx`, 1 `.xltx`), by element
shapes and counts with Python's `zipfile` and `ElementTree` over
the worksheet and styles parts, never a formula's text, a message's
text, a colour or a cell's text:

- `dataValidations` in 44 worksheets of 15 workbooks, 323
  `dataValidation` elements, 1 to 34 per container, `count` on every
  container, `xWindow` and `yWindow` on 2. On the elements: `sqref`,
  `allowBlank` and `showInputMessage` on all 323, `showErrorMessage`
  322, `xr:uid` 303, `prompt` 260, `type` 69 (`list` 29, `custom`
  24, `textLength` 7, `none` 6, `decimal` 3; the 254 without a
  `type` are input-message-only validations, the schema's default
  `none`), `errorStyle` 60, `error` 58, `errorTitle` 30, `operator`
  24 (`equal` 14, `lessThanOrEqual` 7, `greaterThanOrEqual` 3),
  `showDropDown` 20, `promptTitle` 6; `formula1` in 69, `formula2`
  in 20. 4 `sqref` values list more than one area. The container's
  preceding root child: `mergeCells` 32, `conditionalFormatting` 7,
  `phoneticPr` 3, `dataConsolidate` 1, `sheetData` 1: schema order
  in every case. No `x14:dataValidations` extension anywhere.
- `conditionalFormatting` in 12 worksheets of 10 workbooks, 29
  elements (1 to 6 per sheet), 31 `cfRule` (27 elements with one
  rule, 2 with two), `sqref` on all 29, one listing more than one
  area. Rule types: `cellIs` 10, `expression` 5, `dataBar` 5,
  `iconSet` 4, `colorScale` 2, `duplicateValues` 2, `top10` 2,
  `notContainsBlanks` 1. On the rules: `type` and `priority` on all
  31, `dxfId` 20, `operator` 10 (`lessThan` 6, `greaterThan` 2,
  `between` 2), the `top10` and `aboveAverage` attribute set
  (`rank`, `percent`, `bottom`, `aboveAverage`, `equalAverage`,
  `text`) on 12, spelled out by one producer on rules that do not
  need them; `formula` children 18, `dataBar` 5, `iconSet` 4,
  `colorScale` 2, `extLst` 5 (the five data bars, each linked by
  `x14:id` to an `x14:conditionalFormatting` in the sheet's
  `extLst`, which 7 sheets carry). The first element's preceding
  root child: `sheetData` 7, `mergeCells` 3, `sheetProtection` 1,
  `phoneticPr` 1; the last's following child: `dataValidations` 7,
  `printOptions` 5: schema order in every case. No `stopIfTrue`.
- `dxfs` in 16 styles parts (`count` 1 to 440), always after
  `cellStyles` and before `tableStyles` (10) or `colors` (6): the
  schema order of 18.8.39. Their children: `font` 964, `alignment`
  495, `fill` 459, `border` 260, `numFmt` 147, `protection` 14. The
  11 parts without `dxfs` include the template's.
- The worksheet rewrite's schema-position table (`WORKSHEET_
  CHILDREN`, part 3) already lists `conditionalFormatting` and
  `dataValidations` between `phoneticPr` and `hyperlinks`; the
  shift pass already moves, cuts and drops both by their `sqref`
  and keeps a `dataValidations` count right, so every corpus
  `sqref` of both kinds parses through `Area::parse`.
- Part 2's `Font`, `Fill` and `Border` each compose their element
  bytes with a prefix (`compose`, `element`); the styles rewrite
  grows `numFmts`, `fonts`, `fills`, `borders` and `cellXfs` by one
  table of containers, so a sixth container is one more entry.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. Under decision 1 A
everything below sits behind a Cargo feature `rules`, off by
default: the types `Validation`, `ValidationKind`, `Operator`,
`ErrorStyle`, `ValidationEntry`, `Rule`, `Cfvo`, `CfvoKind`,
`IconSetKind`, `RuleEntry`, seven `Package` calls, with the
messages of this part in the catalogue marked as the feature's.
The core builds, tests and links without it; the tool `emend-copy`
turns it on as it does `metadata`. Nothing existing changes
signature. One core change: the styles rewrite's table of grown
containers gains `dxfs`, with nothing recorded for it unless the
feature is on.

### Areas and formulas

Every call that names cells takes `areas: &[Area]`, written as the
`sqref` list, each area in `A1` form, space-separated, in the order
given; an empty list is refused at the call ("a validation or a
conditional format has no area"). A single cell is an `Area` whose
`first` and `last` are equal, written as one reference, as Excel
writes it. `Area` is on the grid by construction (part 3).

Every formula is text: `formula1`, `formula2` and a rule's
`<formula>` children hold what the caller gave, escaped as text,
without a leading `=`, never parsed, never evaluated, and not a
formula change under `specs/0013`'s rule, since no cached value
depends on a validation or a conditional format. An empty formula
where the kind needs one is refused at the call ("a validation or
a rule formula is empty or missing"), as is a formula holding a
character XML cannot carry (part 4's message); a formula given
where the kind takes none is refused with the same message. A list
validation's `formula1` is either a range reference or a quoted
list of constants (`"a,b,c"`), as openpyxl takes it; the crate does
not tell the two apart.

### Data validation

`ValidationKind`: `Whole`, `Decimal`, `List`, `Date`, `Time`,
`TextLength`, `Custom` (18.18.20's values other than `none`; a
validation of type `none`, which the corpus has 254 of, is an
input message alone and is read back as `None`, never written).
`Operator`: `Between`, `NotBetween`, `Equal`, `NotEqual`,
`GreaterThan`, `LessThan`, `GreaterThanOrEqual`,
`LessThanOrEqual` (18.18.21; the same names serve 18.18.15).
`ErrorStyle`: `Stop`, `Warning`, `Information`.

`Validation<'a>`: `kind: ValidationKind`, `operator: Option<
Operator>` (`Between` when `None` and the kind takes one; refused
at the call for `List` and `Custom`, which take none), `formula1:
&'a str`, `formula2: Option<&'a str>` (required by `Between` and
`NotBetween`, refused with any other operator), `allow_blank:
bool`, `hide_drop_down: bool` (`List` only; the attribute Excel
writes as `showDropDown="1"` means the arrow is HIDDEN, so the
field is named for what it does; refused at the call for another
kind), `input: Option<(&'a str, &'a str)>` (a
title and a text, written as `promptTitle` and `prompt` with
`showInputMessage="1"`; absent, `showInputMessage="0"`), `error:
Option<(ErrorStyle, &'a str, &'a str)>` (a style, a title and a
text, written as `errorStyle`, `errorTitle` and `error` with
`showErrorMessage="1"`; absent, `showErrorMessage="0"`). A title
longer than 32 characters or a text longer than 255 (Excel's
limits, which it enforces on load with a repair) is refused at the
call ("a validation message is too long"); every text follows part
4's character rule. Derived `Clone`, `Copy`, `Debug`, `PartialEq`.

- `add_validation(&mut self, part, areas: &[Area], validation:
  Validation) -> Result<(), Error>`: records one `<dataValidation>`
  on a worksheet part (`part` as `set_cell` names it). At the call:
  the area, formula, operator, message and kind rules above. At the
  write: an area that overlaps a cell of a `dataValidation` the part
  has, or of one recorded earlier in the same write, is refused
  ("a validation overlaps one the worksheet has, or a
  dataValidation sqref does not parse"; decision 4 A, the rule
  Excel enforces and part 3's merge rule models, one validation
  per cell), and so is a kept `sqref` that does not parse.
- `remove_validation(&mut self, part, areas: &[Area]) -> Result<(),
  Error>`: records the removal of the `<dataValidation>` whose
  `sqref`, parsed as a list, equals the given list, area for area
  in order; none at the write is refused ("no validation on those
  areas"). A removal and an addition of one list in one write is a
  replacement: the removal first, then the overlap check against
  the part without it.
- `validations(&mut self, part) -> Result<Vec<ValidationEntry>,
  Error>`: reads the part's `<dataValidation>` elements in part
  order. `ValidationEntry`: `areas: Vec<Area>`, `kind: Option<
  ValidationKind>` (`None` for a type `none` or absent), `operator:
  Option<Operator>`, `formula1: Option<String>`, `formula2: Option<
  String>`, `allow_blank`, `hide_drop_down`, `input: Option<(String,
  String)>` (present when `showInputMessage` is not `0`, the title
  and text empty when absent), `error: Option<(ErrorStyle, String,
  String)>` likewise. An attribute value the enumerations do not
  name, or a `sqref` that does not parse, refuses the read with the
  overlap message's second clause. Nothing is kept: the caller's
  snapshot, as `defined_names` is.

The write, in the worksheet pass: the `<dataValidations>` container
is held until its end tag, as part 3 holds `<mergeCells>` (bounded
by one container: 34 elements at most in the corpus); a kept
`<dataValidation>` whose parsed `sqref` a removal names is skipped
whole; every addition is appended before the end tag, in record
order, as `<dataValidation type=".." operator=".." allowBlank=".."
showInputMessage=".." showErrorMessage=".." errorStyle=".."
errorTitle=".." error=".." promptTitle=".." prompt=".." sqref="..">
<formula1>..</formula1><formula2>..</formula2></dataValidation>`,
each attribute only when the rule above gives it, `allowBlank`,
`showInputMessage` and `showErrorMessage` always, `showDropDown`
only when `hide_drop_down`, no `xr:uid`, the element and its
children taking the container's prefix; the `count` attribute set
to the kept plus the added, the other attributes of the start tag
kept. A container left empty is dropped; one that is absent is
created in schema position (part 3's table, before `hyperlinks`,
`printOptions` or whatever follows); a second is refused ("a
worksheet has two dataValidations elements"). Part 3's shift pass
already moves and drops these elements; on a worksheet with a row
or column shift in the same write a validation edit is "any other
edit" of the one-shift rule and refused with that message. A
validation edit to a part this write replaces or removes meets
part 1's message for a cell edit, as a hyperlink does.

### Differential styles

`add_differential_style(&mut self, style: &Style) -> Result<u32,
Error>`: records one `<dxf>` to append to the styles part's
`<dxfs>` and returns its index, the part's `dxfs` count (read from
the `count` attribute, or the children counted when it is absent
or does not parse) plus the additions before it. The `Style` is
part 2's: `font`, `fill` and `border` are written by their element
writers, in 18.8.14's order (`font`, `fill`, `border`), each
without the `applyX` flags a cell format carries; `number_format`,
`alignment` and `protection` are refused at the call ("a
differential style holds a field this part does not write"; a
`dxf` number format needs an inline code and an id of its own, and
Excel ignores the other two in a conditional format; a later part
if a caller needs them), and so is a style with none of the three
set. Nothing is deduplicated: two equal calls are two elements,
as `add_style` treats two equal cell formats. A `dxf` is never
removed, since a conditional format the part keeps may name it.

The styles rewrite grows `<dxfs>` as it grows `<fonts>`: the new
elements immediately before the container's end tag in record
order, taking the container's prefix, `count` replaced by the
children plus the additions, an empty container opened up; a part
with no `<dxfs>` that gains one gets it, with the root's prefix,
immediately before the first of `tableStyles`, `colors` and
`extLst` among the root's children, or before the root's end tag
when none is there (18.8.39's order; the corpus has both cases);
two `<dxfs>` refused ("a styles part has two dxfs elements").
Under decision 5 A a solid fill is written as the caller's `Fill`
gives it; the catalogue says that Excel reads a differential solid
fill's colour from `bgColor`, so a caller sets `background`, as
openpyxl's `PatternFill(bgColor=..)` does for a rule.

### Conditional formatting

`Rule<'a>`, one variant per 18.18.12 value this part writes:

- `CellIs { operator: Operator, formula1: &'a str, formula2:
  Option<&'a str> }` (`formula2` with `Between` and `NotBetween`
  only, as for a validation);
- `Expression(&'a str)`;
- `ContainsBlanks`, `NotContainsBlanks`, `DuplicateValues`,
  `UniqueValues` (no formula; Excel writes a `<formula>` for the
  blank rules, `LEN(TRIM(A1))=0` and its negation for the area's
  top-left cell, and reads them without; the write puts that
  formula in, built from the first area's top-left cell, so Excel
  and LibreOffice agree; the text never comes from the caller);
- `Top { rank: u32, percent: bool, bottom: bool }` (`top10`; `rank`
  1 to 1000, refused outside);
- `AboveAverage { above: bool, equal: bool }` (`aboveAverage`;
  written as `aboveAverage="0"` when below, `equalAverage="1"` when
  equal, the defaults omitted);
- `ColorScale { min: Cfvo, mid: Option<Cfvo>, max: Cfvo }`, each
  `Cfvo { kind: CfvoKind, color: Color }`;
- `DataBar { min: CfvoKind, max: CfvoKind, color: Color, show_value:
  bool }` (the 2007 form alone: no `x14` extension, so Excel 2010
  and later show a solid bar of one colour with no negative-value
  bar or axis, which is what openpyxl's `DataBarRule` writes);
- `IconSet { set: IconSetKind, thresholds: &'a [CfvoKind], reverse:
  bool, show_value: bool }` (`IconSetKind`: the 17 sets of
  18.18.42, `ThreeArrows` to `FiveQuarters`; `thresholds` must
  number as the set's icons, 3, 4 or 5, the first `Percent(0)` by
  Excel's convention or whatever the caller gives, refused when
  the count is wrong).

`CfvoKind`: `Min`, `Max`, `Num(f64)`, `Percent(f64)`, `Percentile(
f64)`, `Formula(&'a str)` (18.18.13; `Min` and `Max` only as the
first and last of a scale or a bar, `Percent` and `Percentile` 0 to
100, a finite number, a non-empty formula, else "a colour scale,
data bar or icon set value is not valid" at the call). The first
six rule kinds are the dxf rules; the last three are the visual
rules. Derived `Clone`, `Copy`, `Debug`, `PartialEq` throughout.

- `add_conditional_format(&mut self, part, areas: &[Area], rule:
  Rule, dxf: Option<u32>, priority: Option<u32>, stop_if_true:
  bool) -> Result<(), Error>`: records one `<conditionalFormatting
  sqref="..">` holding one `<cfRule>`. At the call: the area and
  formula rules; a dxf rule with `dxf` `None`, or a visual rule
  with `Some`, is refused ("a conditional format rule and its
  differential style do not agree"; Excel applies nothing for a
  dxf rule without one, and the visual rules carry their own look);
  `priority` 0 is refused (the index message of part 3). At the
  write: a `dxf` index at or past the part's `dxfs` count plus
  the additions of the same write is refused ("a differential
  style index is out of range"); a `priority` given is written as
  given (two rules of one priority are Excel's to order); one not
  given is the part's highest kept `priority` plus one, plus one
  again for each earlier addition without one, in record order,
  which is openpyxl's assignment; a kept `priority` that does not
  parse as a `u32` refuses the write ("a cfRule priority does not
  parse").
- `remove_conditional_format(&mut self, part, areas: &[Area]) ->
  Result<(), Error>`: records the removal of every `<conditional
  Formatting>` whose `sqref`, parsed as a list, equals the given
  list area for area; none at the write is refused ("no conditional
  format on those areas"); one whose `cfRule` holds an `extLst`
  (the five corpus data bars, each twinned with an `x14` element
  in the sheet's extension list this part never touches) is
  refused ("a conditional format has an extension this part does
  not remove"), so no twin is left without its rule.
- `conditional_formats(&mut self, part) -> Result<Vec<RuleEntry>,
  Error>`: reads every `<cfRule>` of every `<conditionalFormatting>`
  in part order. `RuleEntry`: `areas: Vec<Area>`, `kind: String`
  (the `type` attribute's text, so a kind this part does not write
  is still listed), `operator: Option<Operator>`, `formulas: Vec<
  String>`, `dxf: Option<u32>`, `priority: u32`, `stop_if_true:
  bool`, `extended: bool` (an `extLst` under the rule). A `sqref`
  that does not parse, an operator the enumeration does not name,
  or a `priority` that does not parse refuses the read with the
  messages above. The visual rules' children are not read back.

The write, in the worksheet pass: a kept `<conditionalFormatting>`
whose parsed `sqref` a removal names is skipped whole (its `cfRule`
read first for an `extLst`; the refusal above); every addition is
written immediately after the last kept `<conditionalFormatting>`
end tag, in record order, or, when the part keeps none, created in
schema position (part 3's table), as `<conditionalFormatting
sqref=".."><cfRule type=".." dxfId=".." priority=".." stopIfTrue=
"1" operator=".." rank=".." percent="1" bottom="1" aboveAverage="0"
equalAverage="1"><formula>..</formula>..</cfRule></conditional
Formatting>`, each attribute only when the kind gives it,
`stopIfTrue` only when true, and for the visual kinds the
`<colorScale>`, `<dataBar>` or `<iconSet>` child with its `<cfvo>`
and `<color>` children as 18.3.1.11 to 18.3.1.49 lay them out
(`cfvo type="min|max|num|percent|percentile|formula"` with `val`
except for `min` and `max`; `color` with the attribute part 2's
`Color` writes, `rgb`, `theme` and `tint`, `indexed` or `auto`;
`dataBar showValue="0"` only when hidden, `iconSet iconSet=".."
reverse="1" showValue="0"` likewise), every element taking the
root's prefix. On a worksheet with a shift in the same write, "any
other edit"; on a part this write replaces or removes, part 1's
message; nothing about the cells is touched.

### Creating an element

Part 3's rule and table, unchanged: `dataValidations` and the
first `conditionalFormatting` go immediately before the first root
child of the main namespace that comes later in `WORKSHEET_
CHILDREN`, or before the root's end tag; `dxfs` goes by the styles
rule in "Differential styles". When both a validation and a
conditional format are created on a part with neither, the
conditional format goes first, as the table orders them.

### Errors

Fourteen new `Error::Package` messages, in the catalogue as the
earlier parts' are, one constant each:

- "a validation or a conditional format has no area" (at the call)
- "a validation or a rule formula is empty or missing" (at the
  call)
- "a validation message is too long" (at the call)
- "a validation overlaps one the worksheet has, or a
  dataValidation sqref does not parse"
- "no validation on those areas"
- "a worksheet has two dataValidations elements"
- "a differential style holds a field this part does not write"
  (at the call)
- "a styles part has two dxfs elements"
- "a differential style index is out of range"
- "a conditional format rule and its differential style do not
  agree" (at the call)
- "a colour scale, data bar or icon set value is not valid" (at
  the call)
- "no conditional format on those areas"
- "a conditional format has an extension this part does not
  remove"
- "a cfRule priority does not parse"

Reused: part 1's "a cell edit to a part this write replaces" for
every worksheet edit here on a replaced or removed part, part 3's
"a row or column index or count is out of range" for `priority` 0
and a `rank` outside 1 to 1000, part 3's one-shift message, part
4's "a text holds a character XML cannot carry" at every formula's
and message's call, part 2's unbound-prefix refusal wherever an
element this part reads carries one, the styles rewrite's refusals
of a part without its collections, and `Error::Xml` for a value
that is not UTF-8. A `Validation` or `Rule` field named for a
count (`rank`, an icon set's thresholds) is checked at the call;
nothing is checked twice.

### 32-bit and streaming

Every count is `u32` or `u64` as elsewhere; an element ordinal
within a part is `usize`. The worksheet pass streams as today; the
`<dataValidations>` container is held until its end tag (bounded
by one container, as "Data validation" says); each `<conditional
Formatting>` is held until its end tag, one at a time, for the
`extLst` check and the `sqref` match (bounded by one element: two
rules at most in the corpus); the additions are the recorded edits.
The styles rewrite streams as part 2's does. The read-backs hold
what they read, the caller's snapshot.

### Size

One probe, under decision 1 A. `probe-emend-rules-write-xml`
(feature `rules` of the probe crate, which turns on `emend/rules`
as `metadata` does `emend/metadata`, built alone with its own
feature set): `probe-emend-format-write-xml`'s work, unchanged,
then on the same package before its write `add_validation` (a
whole-number `Between` validation of two formulas from the command
line on one area), `add_differential_style` (a bold font and a
solid fill of one colour), `add_conditional_format` (`CellIs
GreaterThan` of a formula from the command line with that style
and no priority) and a second with `ColorScale` of two `Cfvo`s,
then after the write the output opened and the validation count
and the conditional format count read back and printed, so both
read sides are linked and nothing is dead. The size log measures
it as part 4a's did, classifies the seventeen probes of part 4a's
record against it, the eighteenth, the new probe, listed as new,
prints the feature's cost (the rules probe minus `probe-emend-
format-write-xml` rebuilt at the same commit), the core's growth
for the part (the format probe at the commit minus the same probe
rebuilt at 3e67745, both printed; expected near 0), the whole
growth since b728fdf (the format probe at the commit minus 967884,
probe bytes included), and the budget figure (the format probe
minus `probe-zip-xml`) against 458752, which this part does not
move by design.

### Harness runs

`tools/emend-copy` gains three operations, given after the two
paths as the others are; none takes a reference, a formula or a
number from its command line, and every constant below is the
tool's own. W is the first worksheet part in `<sheets>` order and
M its largest row number as the tool's cell read gives it; a cell
past the grid refuses the sample whole with part 3's index message
(no corpus workbook has that shape). A refusal exits 2 with no
output, as every sample does. The areas below are built by the
tool from M; no reference from the corpus is printed.

- `--validate-sample <in> <out>` (P1): `add_validation(W, [(M + 1,
  1)], List of "\"emend,sample\"" with `allow_blank`, an input
  message ("emend", "pick one") and a `Stop` error ("emend", "not
  in the list"))`, `add_validation(W, [(M + 1, 2):(M + 2, 2)],
  Whole Between "1" and "10")`; writes; reopens the output and
  prints `validate` with `dv_before`, `dv_after` (the entries
  `validations(W)` returns; `+ 2`, asserted, the output removed on
  a difference as `--style-sample` does), `container_before` (1
  when W has a `<dataValidations>`), `count_after` (the container's
  `count` attribute in the output, read by the tool with `quick-
  xml`; equal to `dv_after`, asserted); then, on the reopened
  output in memory, `remove_validation(W, [(M + 1, 1)])`, a write
  into a `Cursor`, and that reopened for `dv_removed` (equal to
  `dv_before + 1`, asserted), so the removal is exercised on every
  corpus workbook without a second harness output. A corpus
  validation on the cells the tool picks would refuse the sample
  as an overlap; M + 1 is below every validated cell, since a
  validation on an empty row is possible and the run log reports
  a refusal as a finding.
- `--cf-sample <in> <out>` (P2): `add_differential_style(Style {
  font: bold, Rgb(0x9C0006); fill: solid, background Rgb(0xFFC7CE)
  })` (openpyxl's light red fill with dark red text), `add_
  conditional_format(W, [(M + 1, 1):(M + 1, 3)], CellIs GreaterThan
  "5", Some(dxf), None, false)`, `add_conditional_format(W, [(M +
  2, 1)], Expression of the tool's own text "TRUE", Some(dxf),
  None, true)`; writes; reopens; prints `cf` with `cf_before`,
  `cf_after` (the entries `conditional_formats(W)` returns; `+ 2`,
  asserted), `dxfs_before`, `dxfs_after` (`dxf` elements in the
  styles part, counted by the tool with `quick-xml`; `+ 1`,
  asserted), `dxfs_present_before` (1 when the part has a
  `<dxfs>`), `priority_max_before` (0 when W has no rule),
  `priority_first_after` (the first added rule's priority read
  back; `priority_max_before + 1`, asserted); then, on the reopened
  output in memory, `remove_conditional_format(W, [(M + 2, 1)])`, a
  write into a `Cursor`, and that reopened for `cf_removed` (equal
  to `cf_before + 1`, asserted).
- `--scale-sample <in> <out>` (P3): `add_conditional_format` three
  times on W, `ColorScale` of `Min` `Rgb(0xF8696B)` and `Max`
  `Rgb(0x63BE7B)` on `(M + 1, 1):(M + 3, 1)`, `DataBar` of `Min`
  and `Max` in `Rgb(0x638EC6)` with the value shown on `(M + 1,
  2):(M + 3, 2)`, `IconSet ThreeArrows` with thresholds `Percent(0)`,
  `Percent(33)`, `Percent(67)` on `(M + 1, 3):(M + 3, 3)`, each with
  no style, no priority; writes; reopens; prints `scale` with
  `cf_before`, `cf_after` (`+ 3`, asserted), `visual_after` (the
  entries whose `kind` is `colorScale`, `dataBar` or `iconSet`;
  `+ 3` over the input's, asserted), `extended_before` (the entries
  with `extended` true, the five corpus data bars among them).

The runner and the record script are unchanged; each run names
its operation through `--tool-arg` and one `--label`. Expected
results, `lo` and `lo_in` `ok` in every case; `descriptor` alone
allowed on a changed entry; `order_preserved` `yes`;
`files_identical` `no`; and per sample:

- validate: W `changed`; every other entry `identical`.
- cf: W and the styles part `changed`; every other entry
  `identical`.
- scale: W `changed`; every other entry `identical`.

The run log's own check over the extracted input and output parts,
with `ElementTree` and byte spans and a control that reports a
planted difference per check, printing no formula, no message, no
colour and no reference from the corpus:

- P1: every root child of W other than `dataValidations`
  byte-identical; the input's `dataValidation` elements a
  byte-identical prefix of the output's; two more, each with
  exactly the expected attributes (the log builds the expected
  start tags itself, the `sqref` from M) and the expected children;
  the container's `count` equal to its children and its other
  attributes the input's; the container created in schema
  position when absent (checked against the table).
- P2: every root child of W other than `conditionalFormatting`
  byte-identical; the input's `conditionalFormatting` elements a
  byte-identical prefix of the output's; two more, each exactly
  the expected bytes (the log builds them, the `dxfId` the input's
  `dxfs` count, the priorities the input's highest plus one and
  two); placed after the last kept one or in schema position; the
  styles part identical outside `<dxfs>`, its `dxf` children a
  byte-identical prefix of the output's, one more exactly the
  expected bytes, `count` equal to the children, the container
  created before `tableStyles`, `colors`, `extLst` or the root's
  end when absent.
- P3: as P2's worksheet check for three elements, each exactly
  the expected bytes; the styles part `identical`.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. From part 8's
decision 3, every phase's run log also runs the edit open check on
the host over that phase's keep. LibreOffice's rendering of a rule
is not checked, only that the file opens; the Excel check (decision
6 of `specs/0019`, deferred again 2026-09-29) will show the rules
as Excel shows them, and this part's three samples join its list.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m2p4b-<phase>/` (`gate.log`,
`test.log` on both targets with the feature off and on,
`negative.log`, `run.log`, `scan.log`, `size.log` in P4); every
rule the earlier parts added (`specs/0019` "Phases" lists them and
its close-out ruling 4 carries these: every log pins the spec's
hash; a figure labelled another log's is read from that log or
labelled as measured here; an exemption counts tokens in the form,
never lines; a superseded attempt stays on disk under
`<home>/.cache/emend-build/<phase>/`; no regex stand-in is printed
in the placeholder form; planning runs no cargo while a logged
script or a pre-check runs; the push script's authorisation as
labelled inputs, change 8); the scan log last; at most two
independent checks, each check's prompt giving the checker its
directory as `<phase>/check<n>` and saying the answer writes paths
so (`specs/0019` P4 verification ruling 10); report; STOP. No push
during the part; the close-out push follows P4 and carries the
record commit 335da53 of part 4a's close-out with this part's.

Part 4b's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0019-names-links-properties-protection.md` as planning left
them after part 4a's close-out (the close-out rulings and the
ROADMAP rows), and this spec. Part 4b's first code commit carries,
before any other change, the one item of `specs/0019`'s P4
verification ruling 4: a test that a write which adds a missing
core properties part with `add_part` and edits a property in the
same write is refused with "no such part" (behind `metadata`, in
`package/tests/properties.rs` beside the read-side test). Part
4b's P1 scan script carries ruling 7: a one-line control naming a
file that is not the spec, so the `only_spec` content comparison
is exercised and not refused on the count alone.

### P1: data validation

The doc commit; the first code commit as above; then the feature
`rules` declared in the crate and turned on by the tool,
`Validation`, `ValidationKind`, `Operator`, `ErrorStyle`,
`ValidationEntry`, `add_validation`, `remove_validation`,
`validations`, the container pass with the overlap check, the
creation in schema position, the messages of this phase, and
`--validate-sample`. Tests on byte-built parts: the validations
read from a part with three (a list, a whole `Between` with two
formulas and messages, an input message alone with no `type`) and
from one with none; every attribute and child of an added element
checked against literal bytes for each kind and operator; an
addition to a part with a container (appended, the kept children
byte-identical, `count` updated, `xWindow` kept), to the template
part (created before `hyperlinks` when the part has one, else
before `pageMargins`, else before the root's end; the log prints
the output), and to a part with `extLst` alone (created before it);
an overlap with a kept validation, with an earlier addition in the
same write, and a removal-then-addition of the same list accepted;
a removal of a present list (multi-area, in order), of an absent
one (refused), of a list in another order (refused), the last
element dropping the container; two containers refused; a kept
`sqref` that does not parse refused; the call-time refusals one by
one (no area, an empty formula, a formula on `List` with an
operator, `formula2` without `Between`, `hide_drop_down` on
`Whole`, a title of 33 characters, a text with a control
character); a validation edit with a shift refused, and on a
replaced part refused; the tool's line. The gate: the crate built
and tested with the feature off and on, on both targets, and the
`pub fn` and `pub const` first lines listed with it off (the part's
calls absent) and on (present), as part 4a's P1 did; every later
log that runs the crate's tests runs them with the feature on. The
negative log through the binary: `--validate-sample` on a fixture
from the template holding a validation on the sample's first cell
(refused as an overlap) and on one with two `<dataValidations>`
(refused), each with a control. The run log: `--validate-sample`
on both targets, the checks as "Harness runs", the keep and the
edit open check, recorded as `records/m2p4b-p1-<utc-date>-
<shortsha>.md` and `.txt`.

### P2: differential styles and the dxf rules

The doc commit (the P1 rulings and verification); the code
commits: `add_differential_style` with the styles rewrite's sixth
container and its creation rule; `Rule`'s six dxf variants,
`RuleEntry`, `add_conditional_format`, `remove_conditional_format`,
`conditional_formats`, the priority rule, the `extLst` refusal,
the creation and placement rule, the messages of this phase, and
`--cf-sample`. Tests on byte-built parts: a `dxf` appended to a
part with a `dxfs` (the kept children byte-identical, `count`
updated), created in a part without one before `tableStyles`,
before `colors`, before `extLst` and before the root's end (four
fixtures, the log printing each output), two `dxfs` refused; a
style with a number format, an alignment, a protection, and one
with nothing refused at the call; the index returned equal to the
count plus the earlier additions, and one past it refused at the
write; each dxf rule kind's element checked against literal bytes,
the blank rules' built formula for a top-left cell included; a
rule with no style and a visual rule with one refused at the call;
priorities: none given on a part with rules (highest plus one),
two given none (plus one and plus two), one given (written as
given), a kept priority that does not parse refused; additions
placed after the last kept element, and created in schema position
on a part with none (before `dataValidations`, `hyperlinks`, and
the root's end); a removal of a present list, of an absent one
(refused), of one whose rule holds an `extLst` (refused, the part
unchanged); the read-back of a part with a rule of a kind this part
does not write (`containsText`, listed by its `type` text) and of
one with an `extLst` (`extended` true); an edit with a shift and on
a replaced part refused; the tool's line. The negative log through
the binary: `--cf-sample` on a fixture whose one rule holds an
`extLst` on the cell the sample removes (refused) and on one whose
kept rule's `priority` is not a number (refused), each with a
control. The run log: `--cf-sample` on both targets, the checks as
"Harness runs", recorded as `records/m2p4b-p2-<utc-date>-
<shortsha>.md` and `.txt`.

### P3: the visual rules

The doc commit (the P2 rulings and verification); the code
commits: `Cfvo`, `CfvoKind`, `IconSetKind`, the three visual
variants with their children, the value rule, and `--scale-
sample`. Tests on byte-built parts: each visual kind's element
against literal bytes, with every `CfvoKind` and every `Color`
form once, `showValue`, `reverse`, a three-, four- and five-icon
set; the value refusals one by one (`Min` as a `max`, a percent of
101, an infinite number, an empty formula, four thresholds for a
three-icon set, a set of the wrong name); a visual rule with a
style refused; the read-back listing the three kinds with no
formulas and no style; the tool's line. The negative log through
the binary: none of this phase's refusals is reachable through the
binary, since the sample's values are the tool's constants, and
the log says so, running the P2 fixtures again with `--scale-
sample` for the refusals that are (the `extLst` case does not
apply, since the sample removes nothing; the priority case does).
The run log: `--scale-sample` on both targets, the checks as
"Harness runs", recorded as `records/m2p4b-p3-<utc-date>-
<shortsha>.md` and `.txt`.

### P4: the catalogue and limits, the probe, the size record

The doc commit (the P3 rulings and verification); then the first
code commit with the items P3's rulings carry, if any; then the
catalogue and the limits in the `package` module header (the
caller's picture: what a validation and a conditional format do
and do not touch, a formula is text, the priority rule, the
`bgColor` reading of a differential solid fill, the `x14` twins
left alone, what is read back, the Excel check pending), the probe
of "Size" measured as it says, recorded as `records/size-m2p4b-
<utc-date>-<shortsha>.md` with `size.log` printing every figure
the record carries, installing the record and printing its sha256,
and the seventeen probes of part 4a's record classified against
it, the eighteenth, the new probe, listed as new; the feature's
cost and the core's growth stated in the record as facts, the
budget figure against 458752 beside them; the negative log through
the binary with every refusal of this part reachable through the
binary run once (the fixture cases of P1 to P3, and the at-call
refusals named as not reachable, since their inputs are the tool's
constants); the corpus run of all three samples again on both
targets with the release binaries the size log hashed, every
output byte-identical to the phase run directories' outputs by
sha256, in three run directories, recorded as `records/m2p4b-p4-
<sample>-<utc-date>-<shortsha>.md` and `.txt`. The report states
the feature's cost, the core's growth, and the budget figure
against the ceiling.

### Close-out push (after P4)

As part 4a's: the self-scan rule on the push and ci-run scripts,
`ci-run.log`, then the post-stage `scan.log` over the pushed range
with `push.log` and `ci-run.log` as its siblings, then one record
commit holding the three logs, the `ANTHROPIC_API_KEY` check; the
push script's change 8 kept (the maintainer's authorisation and
its date as labelled inputs, refused when empty). The push carries
335da53 and this part's commits, on planning's word and the
maintainer's authorisation typed in the implementing session.
Before the push, the Excel check for Milestone 2 parts 1 to 4a
(every corpus row of every sample, on the laptop), deferred by the
maintainer on 2026-09-27 and again on 2026-09-29 (`specs/0019`
decision 6 and P4 verification ruling 15), is run or deferred
again by a decision recorded here; this part's three samples join
that check's list.

## Decisions for the maintainer

Decided by the maintainer 2026-09-29: 1 to 5 as recommended (A in
each), in the reply "i agree with planning recs for all"; 6
recorded.

1. Packaging, as "Size and packaging" states: A (recommended), one
   feature `rules`, one probe; B, two features; C, folded into
   `metadata`.
2. The visual rules (colour scales, data bars, icon sets): A
   (recommended), in this part as P3, in the 2007 form without the
   `x14` extension, which is what openpyxl writes and every Excel
   since 2007 reads, a data bar then a solid bar of one colour; B,
   left out, the dxf rules alone, P3 dropped and the estimate 8000
   to 12000 lower; a caller who wants a colour scale waits.
3. Read-back: A (recommended), validations and conditional formats
   read back as typed values (areas, kind, operator, formulas, the
   flags, the style index and priority), since a caller cannot
   remove a rule without its areas and openpyxl reads both back;
   planning guesses 7000 to 11000 of the estimate; B, no read-back,
   the harness counting elements with `quick-xml` as it counts
   hyperlinks, and a caller removing a rule only by areas it
   already knows.
4. Two validations on one cell: A (recommended), refused at the
   write, the rule Excel enforces (it repairs a file with two on a
   load) and part 3's merge overlap rule models; B, written as
   given, which openpyxl allows and Excel then repairs.
5. A differential style's solid fill: A (recommended), written as
   the caller's `Fill` gives it, the catalogue stating that Excel
   reads a differential solid fill's colour from `bgColor` so a
   caller sets `background`, as openpyxl's `PatternFill(bgColor=)`
   does; B, a solid fill with `foreground` alone swapped to
   `bgColor` at the write, a convenience that makes one `Fill`
   mean two things.
6. The Excel check for parts 1 to 4a: deferred on 2026-09-27 and
   on 2026-09-29 by the maintainer, to run before this part's
   close-out push at the latest, or be deferred again by a
   decision recorded here. Recorded, not asked.

## Out of scope

- The rule kinds this part does not write: `containsText`,
  `notContainsText`, `beginsWith`, `endsWith`, `containsErrors`,
  `notContainsErrors`, `timePeriod` (each expressible as an
  `Expression` by the caller); a `cfRule` with more than one
  `formula` beyond `Between`'s two; `pivot` rules.
- The `x14` extensions of both capabilities (`x14:dataValidations`,
  `x14:conditionalFormattings`: data bars with negative bars and
  axes, icon sets of custom icons, validations that list another
  sheet's range in an old Excel's form): kept as they are, never
  written, a rule twinned with one never removed.
- A differential style's number format, alignment and protection;
  removing a `dxf`; reading the `dxfs` back.
- `dataValidation`'s `imeMode` and the `xr:uid` revision
  attributes: kept on read elements, never written.
- Moving a validation or a conditional format on an insertion or
  deletion beyond what part 3's shift rule already does with its
  `sqref`; editing a rule in place (remove and add).
- What Excel or LibreOffice shows for a rule: the file opens; the
  Excel check is the maintainer's.
