# 0017: Milestone 2 part 2, fuller cell styles

Status: APPROVED by the maintainer 2026-09-25 (planning draft the
same day): decision 1 taken as A and decisions 2 to 4 as planning
recommended, in the maintainer's reply "A, rest as recommended".
Sent to the implementing session the same day.
Context: `CLAUDE.md` ("What emend is": like openpyxl; "Dependencies
and licences": anything heavier lives behind a Cargo feature that is
off by default; "32-bit and sizes"), `ROADMAP.md` ("Milestone 2",
the outline approved 2026-09-23, part 2; "Standing items", the size
ceiling), `specs/0012-styles.md` ("Module `styles`: what is read",
"what can be added", "Module `package`: recording and applying style
edits", the limits and the styles-part precondition),
`specs/0016-new-workbook-and-sheets.md` (P5, the erased reader and
writer; the close-out rulings carried here), `specs/0013-formulas.md`
(the size probes), `specs/0001-bootstrap.md` ("Rules for every
phase"). ECMA-376 Part 1, 18.8 (`fonts`, `fills`, `borders`,
`cellXfs`, `alignment`, `protection`) and 18.18 (the simple types
for pattern, border style, alignment and underline keywords) are
cited where the design leans on them; no text is copied from them.

## Goal

A cell can be given a font, a fill, a border, an alignment and a
protection, alone or together and with a number format, without
renumbering any style the workbook already has. Part 5 (`specs/0012`)
appends a cell format derived from an existing one with its number
format changed; this part lets the same addition name a new font, a
new fill and a new border, each appended to its own collection in
the styles part, and carry a new `alignment` and `protection` in the
cell format itself. Nothing in the part moves: `fonts`, `fills`,
`borders` and `cellXfs` grow at their ends and their `count`
attributes follow, so every `s`, `fontId`, `fillId` and `borderId` in
the workbook keeps its meaning. Nothing new is read back: the part's
existing fonts, fills and borders stay unmodelled, as part 5 left
them, and a cell format's `font_id`, `fill_id` and `border_id`, which
part 5 reads, are how a caller and the harness see what was applied.

The comparison point is openpyxl's `cell.font = Font(...)`,
`cell.fill = PatternFill(...)`, `cell.border = Border(...)`,
`cell.alignment = Alignment(...)` and `cell.protection =
Protection(...)` followed by `save`, without the styles part openpyxl
regenerates. The harness is the acceptance: over the corpus, a sample
that adds one font and one fill (P1), then one border, one alignment
and one protection (P2), to an existing cell and to a new one leaves
every other entry byte-identical, the styles part identical outside
the containers it grows, the containers' existing children identical,
and the worksheet's cells identical except at the two references, as
an independent parser in the run script shows; every output opens in
LibreOffice; emend's own reader reads the new cell formats' ids back.

## The size ceiling (decision 1, before any code)

The budget figure after part 1 is 375984 section bytes against the
ceiling of 393216 (`records/size-m2p1-2026-09-25-975c974.md`): 17232
of headroom. The approved outline estimated this part at about
20000 and part 3 at about 25000, and said of part 3 "where the
headroom runs out; a feature if the measurement says so". Planning's
estimate for this part as designed below is 15000 to 20000 section
bytes (the element composers for five elements, three more
containers in the styles rewrite, the keyword tables, the two
refusals; no generic path is instantiated twice, the P4 lesson);
the estimate is a guess, as part 1's was, and part 1 measured 36876
against a guess of 25000. Whatever the figure, the part does not
fit under the ceiling with room for part 3, so the maintainer
chooses before P1's code:

- A (recommended): a Milestone 2 ceiling of 458752 section bytes
  over `probe-zip-xml` (448 KiB, the Milestone 1 ceiling plus 64
  KiB), judged at the close of part 3 on the last core probe, with a
  checkpoint after this part's size record: above 25000 for this
  part alone, the maintainer re-rules before part 3's spec. The
  Milestone 1 ceiling of 393216 stays as the record of Milestone 1.
  Planning's arithmetic, guesses beside a measurement: 375984 plus
  20000 (this part) plus 25000 (part 3) plus about 10000 of drift
  over two parts is about 431000, leaving about 27000 under
  458752. The 2026-09-18 checkpoint ruling took a higher ceiling
  over features when part 2 of Milestone 1 overran, and the outline
  put parts 1 to 3 in the core.
- B: the ceiling unchanged; this part behind a Cargo feature
  `styles` off by default, outside the budget figure as `dates` is,
  its cost measured beside the budget on its own probe. The core
  keeps part 5's derived formats only. A consumer that wants a font
  turns the feature on. Parts 2 and 3 both become features, so the
  core is what part 1 left and the ceiling stops being a working
  figure.
- C: the ceiling unchanged; build in the core and, should P3's
  record land above 393216, a P4 moves the part behind the feature,
  as P5 moved part 1's duplication. Two designs for one measurement;
  not recommended.

Carried from part 1's close-out (specs/0016 close-out ruling 4) and
not counted in any figure above: `Stream`'s two instantiations are
an unmeasured size candidate. Should A be taken and the checkpoint
trip, measuring that candidate comes before re-ruling.

Taken: A, 2026-09-25. The Milestone 2 ceiling is 458752 section
bytes over `probe-zip-xml`, judged at the close of part 3 on the
last core probe, with the checkpoint after this part's size record.

Under A this spec stands as written. Under B, "Crate" below names
the feature and every item of "Design" is behind it; the probes,
tests and samples run with the feature on, and the P3 size record
carries the feature's cost beside the budget figure as part 5 P3's
carries `dates`.

## Facts planning checked (2026-09-25)

Over the 27 corpus workbooks, by element shapes and counts with
Python's `zipfile` and `ElementTree` and regular expressions over the
styles parts, never a font name, a format code or a cell's text; the
part 5 facts (`specs/0012`) stand, and these add to them:

- Every part has `fonts`, `fills` and `borders`, each with `count`
  equal to its children (27 of 27 each); `fonts` carries the Office
  2010 `x14ac:knownFonts` attribute in 9 parts. The root's children
  are in schema order in every part, `numFmts fonts fills borders
  cellStyleXfs cellXfs cellStyles` then the tail part 5 lists.
- `fonts`: 399 `font` elements, 6 to 42 per part. Every one has `sz`
  and `name`; `family` 388, `color` 271, `scheme` 221, `b` 124,
  `charset` 39, `i` 18, `u` 6, `strike` 3; no `vertAlign`,
  `outline`, `shadow`, `condense` or `extend`. The children come in
  29 distinct orders, every one a subsequence of the schema's `b i
  strike u sz color name family charset scheme`. `b` is written
  without `val` 86 times and with `val="true"` 38; `i` 11 and 7; `u`
  without `val` 3 times and `val="single"` 3; `strike` always
  `val="true"`. `sz` is an integer 396 times and a decimal 3.
  `color` takes the forms `rgb` 49 (always 8 upper-case hex digits),
  `theme` 101, `theme` with `tint` 117 (71 positive, 47 negative),
  `indexed` 3, `indexed` with `tint` 1. `family` is 0, 1 or 2 (141,
  10, 237); `scheme` is `minor` 173 or `major` 48. 44 fonts are
  byte-equal to an earlier font of the same part: producers do not
  deduplicate. Font 0 has `sz name family` with `charset` in 8 parts,
  `color` and `scheme` in 8, `scheme` in 2.
- `fills`: 180 `fill` elements, 3 to 27 per part, every one a
  `patternFill` (no `gradientFill`); fill 0 is `patternType="none"`
  and fill 1 `gray125` in all 27, as Excel writes them. 126 are
  `solid`, every one with `fgColor` (`rgb` 62, `theme` with `tint`
  39, `theme` 25) and 120 with `bgColor` too (`indexed` 64 in 53,
  `indexed` 65 in 11, `rgb` 56). No two fills of a part are
  byte-equal.
- `borders`: 182 `border` elements, 1 to 52 per part, every one with
  exactly the children `left right top bottom diagonal` in that
  order; 32 carry `diagonalUp="false"` and `diagonalDown="false"`
  (border 0 of 17 parts among them). Of 910 sides, 545 are empty
  elements and `diagonal` is empty in all 182; the styles seen are
  `thin` 196, `medium` 89, `hair` 60, `thick` 16, `double` 3,
  `dashed` 1; a styled side always has a `color`, in the forms
  `theme` 143, `theme` with `tint` 116, `rgb` 99, `indexed` 4,
  `auto` 3. No two borders of a part are byte-equal.
- `cellXfs`: of the 992 `xf`s, 692 have an `alignment` child alone,
  220 `alignment` then `protection`, 7 `protection` alone, 73 none;
  `alignment` always precedes `protection` and both are always
  empty elements. `alignment` attributes: `horizontal` 729 (`left`
  352, `general` 156, `center` 135, `right` 86), `vertical` 799
  (`center` 557, `bottom` 172, `top` 70), `wrapText` 468 (`1` 296,
  `true` 8, `false` 164), `indent` 433 (0 to 6), `textRotation` 172
  (0 and 90), `shrinkToFit` 172 (all `false`); no `readingOrder`,
  `justifyLastLine` or `relativeIndent`. `protection`: `locked` 227
  (`true` 166, `0` 55, `false` 6), `hidden` 172. Of the `xf`s whose
  `fontId` is not 0, 730 carry `applyFont` `1` or `true` and 62 no
  `applyFont`; of those with an `alignment` child, 690 carry
  `applyAlignment` `1` or `true`, 82 `false`, 140 none.
- No part has a space before `/>` or a single-quoted attribute
  (part 5's fact, checked again).
- The crate's template styles part (`crates/emend/template/xl/
  styles.xml`, pinned in part 1 P2's run log) has one font (`sz`,
  `name`), the two required fills, one border of five empty sides,
  one `cellStyleXfs` entry, one `cellXfs` entry and one `cellStyle`.
- The crate today: `Styles` reads `numFmts` and `cellXfs` and skips
  every other root child raw (`styles.rs`, `read`); `derived_xf`
  writes a new `xf` start tag from the base's attributes with
  `numFmtId` replaced and `applyNumberFormat` set, keeping the
  base's content raw; `rewrite_styles` copies every token raw except
  the `numFmts` and `cellXfs` containers, whose `count` is rewritten
  and whose additions go before the end tag, an empty container
  opened up (`package.rs`, `rewrite_styles`). `Value::Number`'s
  text form is `number_text` (`cells.rs`); a text is checked against
  `forbidden_in_xml` (`cells.rs`) and escaped by the crate's
  `escape`.
- The size budget figure after part 1 is 375984 section bytes of a
  393216 ceiling: headroom 17232 (`records/size-m2p1-2026-09-25-
  975c974.md`). The outline's estimate for this part is 20 KB.

## Design

### Crate

Under decision 1 A, in the core: the new types in `styles.rs` (or a
`styles/elements.rs` submodule, the implementing session's choice,
with the tests beside them), the additions to `Styles`, and the
`package` changes below. No new dependency. Under B the same code
is behind `[features] styles = []` in `crates/emend/Cargo.toml`,
`#[cfg(feature = "styles")]` on the new public items and on the
three extra containers of the rewrite, with `add_cell_style`
unchanged and available without the feature.

### Module `styles`: the counts

`Styles` gains three private counts read from the part, `fonts`,
`fills` and `borders` (the number of `font`, `fill` and `border`
children of the collection among the root's children; `None` when
the collection is absent), with public accessors `Styles::fonts(&
self) -> Option<u32>`, `fills`, `borders`: the count, or `None` for
a part without that collection. A second collection of the same
name at the root is refused as a second `cellXfs` is ("the styles
part has more than one numFmts or cellXfs" widens to "the styles
part has more than one numFmts, fonts, fills, borders or cellXfs";
the message text changes, the catalogue and the test that pins it
with it). The reader counts the children at depth 3 and skips their
subtrees raw as it skips the collections today; nothing inside a
`font`, `fill` or `border` is read. A count `u32` cannot hold is
refused with "a style index is out of range".

### Module `styles`: what can be added

The types, all `Clone`, `Debug`, `PartialEq`, with `Default` where
every field has one:

- `Style { number_format: Option<NumberFormat>, font: Option<Font>,
  fill: Option<Fill>, border: Option<Border>, alignment:
  Option<Alignment>, protection: Option<Protection> }`, `Default`
  all `None`. Each `Some` field is what the new cell format gets;
  each `None` field is what the base has (or the plain form's value
  with no base).
- `Font { name: Option<String>, size: Option<f64>, bold: bool,
  italic: bool, strike: bool, underline: Option<Underline>,
  vertical_align: Option<VerticalAlign>, color: Option<Color>,
  family: Option<u32>, scheme: Option<Scheme> }`, `Default` all
  `None` and `false`. `Underline { Single, Double,
  SingleAccounting, DoubleAccounting }`; `VerticalAlign {
  Superscript, Subscript, Baseline }`; `Scheme { Major, Minor }`.
- `Color { Rgb(u32), Theme { index: u32, tint: Option<f64> },
  Indexed(u32), Auto }`. `Rgb` is ARGB, written as eight upper-case
  hex digits, the form every corpus `rgb` takes; a caller who wants
  an opaque colour puts `FF` in the top byte. `tint` is a fraction
  in `-1.0..=1.0`.
- `Fill { pattern: Pattern, foreground: Option<Color>, background:
  Option<Color> }`; `Pattern` the nineteen keywords of ECMA-376's
  `ST_PatternType` as variants (`None`, `Solid`, `MediumGray`,
  `DarkGray`, `LightGray`, `DarkHorizontal`, `DarkVertical`,
  `DarkDown`, `DarkUp`, `DarkGrid`, `DarkTrellis`, `LightHorizontal`,
  `LightVertical`, `LightDown`, `LightUp`, `LightGrid`,
  `LightTrellis`, `Gray125`, `Gray0625`). Gradient fills are out of
  scope (0 in the corpus). A solid fill shows its foreground, as in
  Excel; the doc says so, since openpyxl users trip on it.
- `Border { left: Option<Side>, right: Option<Side>, top:
  Option<Side>, bottom: Option<Side>, diagonal: Option<Side>,
  diagonal_up: bool, diagonal_down: bool }`, `Default` all `None`
  and `false`; `Side { style: BorderStyle, color: Option<Color> }`;
  `BorderStyle` the thirteen non-`none` keywords of `ST_BorderStyle`
  (`Thin`, `Medium`, `Thick`, `Dashed`, `Dotted`, `Double`, `Hair`,
  `MediumDashed`, `DashDot`, `MediumDashDot`, `DashDotDot`,
  `MediumDashDotDot`, `SlantDashDot`). A `None` side is written as
  the empty element, the schema's `none`. `vertical` and
  `horizontal` sides (inside-of-range borders) are out of scope (0
  in the corpus).
- `Alignment { horizontal: Option<Horizontal>, vertical:
  Option<Vertical>, wrap_text: bool, shrink_to_fit: bool, indent:
  Option<u32>, text_rotation: Option<u32> }`, `Default` all `None`
  and `false`; `Horizontal { General, Left, Center, Right, Fill,
  Justify, CenterContinuous, Distributed }`; `Vertical { Top,
  Center, Bottom, Justify, Distributed }`. `text_rotation` is 0 to
  180, or 255 for vertical text, as the schema says.
- `Protection { locked: Option<bool>, hidden: Option<bool> }`.

The keyword each variant writes is ECMA-376's, lower camel case
(`mediumDashDotDot`, `centerContinuous`, `singleAccounting`); the
implementing session's tests pin every one against the schema's
spelling, and the log prints the table so a misspelling is a
measurement, not a review.

`Package::add_style(&mut self, base: Option<u32>, style: &Style) ->
Result<u32, Error>` records one new cell format and returns the
index it will have, as `add_cell_style` does, and `add_cell_style(
base, format)` becomes `add_style(base, &Style { number_format:
Some(format), ..Default::default() })` with its behaviour and its
refusals unchanged (the tests of part 5 pass unedited). It reads the
styles part first when it is not cached, so it fails as `styles()`
fails, and it refuses, recording nothing:

- `base` at or beyond `cell_xfs.len()` ("a style index is out of
  range"), as before; a base names an existing `xf` only.
- The number format's refusals of part 5, unchanged.
- A `font` when the part has no `fonts`, a `fill` when it has no
  `fills`, a `border` when it has no `borders`
  (`Error::Package("a style addition needs a collection the styles
  part lacks")`): the part 5 precondition (a style edit assumes the
  font, fill, border and cell-style collections are present, as
  every workbook Excel or LibreOffice writes) made a refusal, since
  a cell naming a font in a part without `fonts` is what Excel
  refuses. The template has all three.
- A `Font::name` that is empty or holds a character XML cannot carry
  (`forbidden_in_xml`'s set) (`Error::Package("a font name is empty
  or holds a character XML cannot carry")`).
- A `Font::size` that is not finite or not above zero; a `tint` that
  is not finite or outside `-1.0..=1.0`; a `text_rotation` above 180
  that is not 255 (`Error::Package("a style field is out of
  range")`).
- An index or id `u32` cannot hold ("a style index is out of
  range"), as before.

The ids: a `Some(font)` gets the id of the first equal `Font`
recorded earlier by this package, else `fonts` plus the number of
distinct fonts recorded earlier; `fill` and `border` the same over
their collections. Two additions with equal fonts share one `font`
element; nothing is compared against the part's own fonts, which
are not read (decision 3). The new `xf`'s bytes, composed at the
call:

- With `base` `Some(i)`: the start tag is written anew from the
  base's attributes as part 5 does, with these values replaced:
  `numFmtId` when a number format is given (and
  `applyNumberFormat` set to `1`), `fontId` when a font is given
  (and `applyFont` set to `1`), `fillId` when a fill is given
  (`applyFill`), `borderId` when a border is given (`applyBorder`),
  and `applyAlignment` set to `1` when an alignment is given and
  `applyProtection` when a protection is. An attribute the base
  lacks is added: `numFmtId` first, as part 5 does; each other id
  appended after the base's attributes in the order `fontId fillId
  borderId`, then each apply flag the base lacks in the order
  `applyNumberFormat applyFont applyFill applyBorder applyAlignment
  applyProtection`. Every other attribute and its value are the
  base's bytes, as part 5 (P1 ruling 5 on quotes stands).
- The content: the base's direct children with its `alignment`
  replaced by the new one when an alignment is given and its
  `protection` likewise; a new `alignment` the base lacks goes
  first, a new `protection` after any `alignment` and before any
  other child; every other child of the base is the base's bytes,
  raw. The reader records where each `xf`'s `alignment` and
  `protection` children lie in its bytes (both are empty elements
  in every corpus `xf`; the schema gives them no children, and a
  start-to-end span is handled the same way), so no child is
  re-parsed. An element with no content is written empty, `/>`.
- With `base` `None`: `<xf numFmtId="N" fontId="F" fillId="I"
  borderId="B" xfId="0"` then the apply flags for what was given in
  the order above, each `="1"`, then `/>` or the new `alignment`
  and `protection` children and `</xf>`. `N`, `F`, `I` and `B` are
  the given ids or 0.

The elements, composed at the call and held with the record, each
taking the qualified-name prefix of its container at the rewrite as
the `numFmt` and `xf` elements do; every attribute double-quoted,
no space before `/>`, children in the schema's order, and only the
children and attributes the `Some` and `true` fields ask for:

- `font`: `<b/>`, `<i/>`, `<strike/>` (each present for `true`,
  the `val`-less form the corpus writes most), `<u/>` for `Single`
  and `<u val="..."/>` for the other three, `<vertAlign val="..."/>`,
  `<sz val="..."/>` with `number_text` of the size (`12` for 12.0,
  `11.5` for 11.5), `<color .../>`, `<name val="..."/>` with the
  name escaped by the crate's `escape`, `<family val="..."/>`,
  `<scheme val="..."/>`; a `Font` with every field `None` and
  `false` writes `<font/>`, which the schema allows and no corpus
  font does (the doc says a consumer shows its default).
- `color`, wherever it appears: `rgb="AARRGGBB"`, `theme="n"` with
  `tint="..."` (`number_text`) when given, `indexed="n"`, or
  `auto="1"`.
- `fill`: `<fill><patternFill patternType="..."><fgColor .../>
  <bgColor .../></patternFill></fill>` with each colour when given,
  `<patternFill patternType="..."/>` with neither.
- `border`: `<border` with `diagonalUp="1"` and `diagonalDown="1"`
  when `true`, `>`, then `left`, `right`, `top`, `bottom`, `diagonal`
  in that order, each `<left/>` for `None` or `<left style="...">
  <color .../></left>` (`<left style="..."/>` with no colour), then
  `</border>`: the five-child form every corpus border takes.
- `alignment`: `<alignment` then `horizontal`, `vertical`,
  `wrapText="1"`, `shrinkToFit="1"`, `indent`, `textRotation` in
  that order for what is given, `/>`; a `Some(Alignment::default())`
  writes `<alignment/>`.
- `protection`: `<protection locked="0|1" hidden="0|1"/>` for what
  is given.

### Module `package`: recording and applying

`Edits` gains the recorded fonts, fills and borders (each a `Vec` of
the composed element bytes beside the `Font`, `Fill` or `Border` it
came from, for the equality lookup). `set_cell_style` is unchanged.
`write_to`'s validation is unchanged: a style edit's index is
checked against the `cellXfs` count plus the additions, as part 5.

The styles rewrite treats five containers as it treats two: among
the root's children, `numFmts`, `fonts`, `fills`, `borders` and
`cellXfs` each have their `count` value replaced by their children
plus that container's additions (a container with no additions and
a `count` equal to its children comes out byte-identical, as today;
a `count` that disagreed with the children is corrected, as today),
their additions written immediately before their end tag in record
order (`numFmt`s in id order, as today), and an empty container that
gains children opened up. A part without `numFmts` gaining a code
gets one as today. `fonts`, `fills` and `borders` are present when
their additions exist, by the refusal at `add_style`; the rewrite
still refuses, once the stream ends and leaving no output, a part
whose `cellXfs` is missing ("a styles part without cellXfs cannot
take a style", as today) and one whose `fonts`, `fills` or `borders`
went missing between the read and the write, which no path of this
crate can cause (the same message, the collection named in the doc,
one test through a byte-built part that swaps the cached read).

Consequences a caller should know, stated in the Limits: the part's
own fonts, fills and borders are not read, so an added font equal to
one the part already has is a second copy (producers write such
copies too: 44 in the corpus); `x14ac:knownFonts` on `fonts` is
kept as it was; `dxfs`, `cellStyles`, `cellStyleXfs` and the theme
are never touched, so a `Theme` colour names whatever the workbook's
theme defines at that index and `Indexed` whatever its `colors` or
the default palette defines; whether a consumer shows a font name it
does not have is the consumer's; a `charset`, an `outline`, a
`shadow`, a `condense` or an `extend` on a font, a gradient fill, an
inside-of-range border side and `readingOrder`, `justifyLastLine`
and `relativeIndent` on an alignment cannot be written; nothing is
deduplicated at the `xf` level (as part 5).

### Errors

Three new fixed strings in the `package` catalogue, each beside the
test that pins it: "a style addition needs a collection the styles
part lacks", "a font name is empty or holds a character XML cannot
carry", "a style field is out of range"; one changed string, the
two-containers message widened as "The counts" says. The catalogue
count in the module header is not typed (the gotcha of 2026-09-18):
the log asserts the catalogue and the source agree as sets.

### 32-bit and streaming

The recorded elements are bounded by the caller's additions. The
reader's new state is three counts and, per `xf`, two byte ranges.
Nothing is read into memory that part 5 did not read. `u32` ids
throughout, `u64` counts at the rewrite as today.

### Size

`tools/size-probe` gains one probe, built in the `size` profile on
i686 musl as the others:

- `probe-emend-fullstyles-write-xml`: `probe-emend-sheets-write-xml`'s
  work, then on the same package before its write `add_style` of a
  `Style` with a `Font` (a name from its command line, size, bold,
  an `Rgb` colour), a solid `Fill` with two colours, a `Border` of
  four thin sides with a `Theme` colour and both diagonals, an
  `Alignment` with every field and a `Protection`, on base 0, and
  `set_cell_style` of it on the number cell; after the write the
  output's `styles()` counts through the three accessors and the
  cell's `style` and that `xf`'s three ids through `Package::cells`
  and `Styles::cell_xfs`, so no composer is dead. The budget figure
  is it over `probe-zip-xml` against the Milestone 2 ceiling of
  458752 section bytes (decision 1, A);
  the part's own cost is it over `probe-emend-sheets-write-xml`; the
  record states the whole growth since part 1's 375984 with the
  earlier probes' drift classified as `specs/0011` P3 verification
  ruling 4 says, the fourteen probes re-measured against part 1's
  record by the byte classifier. Under decision 1 B (not taken) the
  probe enables the feature and its figure is the feature's cost,
  recorded beside the budget figure as
  `probe-emend-dates-write-xml`'s is.

Planning's expectation, not a measurement: under 20000 section
bytes for the part. Above 25000 the report says so and the
maintainer re-rules before P3's record commit.

### Harness runs

`tools/emend-copy` gains two operations, given after the two paths
as the others are; neither takes a name, a colour or a reference
from its command line, and every constant below is the tool's own.
A, M and X are as `--style-sample` defines them (`specs/0012`,
"Harness runs": the first `Number` or `SharedString` cell with a
value and no formula, the largest row number, the `cellXfs` count),
F, I and B the `fonts`, `fills` and `borders` counts.

- `--font-sample <in> <out>` (P1): records S1 = `add_style(A's
  style, &Style { font: Some(F1), fill: Some(L1), ..Default })` with
  F1 = `Font { name: Some("emend"), size: Some(12.0), bold: true,
  color: Some(Color::Rgb(0xFF336699)), ..Default }` and L1 = `Fill {
  pattern: Pattern::Solid, foreground: Some(Color::Rgb(0xFFDDEEFF)),
  background: Some(Color::Indexed(64)) }` (a solid fill with an
  `rgb` foreground and an `indexed` 64 background is the corpus's
  commonest shape, 53 of 126), A's style set to `Some(S1)` when A
  exists; S2 = `add_style(None, &Style { font: Some(F1), ..Default
  })` (the same font, so `fonts` grows by one, not two), `(M + 1, 1)`
  set to `Number(7)` with style `Some(S2)`; writes; reopens; prints
  `edit` with `styled` (1 or 0), `added` 1, `xfs_before` X,
  `xfs_after` (expected X + 2, asserted by the tool as
  `--style-sample` asserts, the output removed on a difference),
  `fonts_added` (the output's `fonts()` minus F, expected 1),
  `fills_added` (expected 1), `cells_before`, `cells_after`
  (expected `cells_before + 1`, asserted), `read_back` (the number
  of these that hold: (a) A's `value` text unchanged, `style`
  `Some(X)`, and `cell_xfs[X]` has `font_id` `Some(F)` and `fill_id`
  `Some(I)`; (b) the new cell `number()` 7, `style` `Some(X + 1)`,
  `cell_xfs[X + 1]` has `font_id` `Some(F)` and `fill_id` `Some(0)`;
  expected `styled + 1`).
- `--border-sample <in> <out>` (P2): S1 = `add_style(A's style,
  &Style { border: Some(B1), alignment: Some(G1), protection:
  Some(Protection { locked: Some(false), hidden: Some(false) }),
  ..Default })` with B1 = four `Side { style: Thin, color:
  Some(Color::Theme { index: 1, tint: None }) }` and no diagonal, and
  G1 = `Alignment { horizontal: Some(Center), vertical:
  Some(Center), wrap_text: true, indent: Some(1), ..Default }`; S2 =
  `add_style(None, &Style { border: Some(B1), alignment:
  Some(Alignment { horizontal: Some(Right), ..Default }), ..Default
  })`; the same cells as `--font-sample`; prints `edit` with
  `styled`, `added`, `xfs_before`, `xfs_after` (X + 2, asserted),
  `borders_added` (expected 1), `cells_before`, `cells_after`
  (asserted), `read_back` ((a) A as above with `border_id` `Some(B)`;
  (b) the new cell with `border_id` `Some(B)`; expected `styled +
  1`).

The runner and the record script are unchanged; each run names its
operation through `--tool-arg`. Expected results, `lo` and `lo_in`
`ok` in every case; in every workbook the styles part and the first
worksheet part `changed`, every other entry `identical` unflagged,
`descriptor` alone allowed on a changed entry, `order_preserved`
`yes`, `files_identical` `no`. The run log's own check over the
extracted input and output parts, with `ElementTree` and byte spans
and a control that reports a planted difference per check:

- P1: the styles part's bytes before `<fonts`, from `</fonts>` to
  `<fills`, from `</fills>` to `<cellXfs`, and from `</cellXfs>` to
  the end are identical; inside `fonts` the input's children's
  bytes are a prefix of the output's, one `font` follows, byte-equal
  to the element the design gives for F1 with the container's prefix
  (the log builds the expected bytes itself from the constants and
  prints them once; they hold no corpus content), and `count` grows
  by one; inside `fills` likewise with L1's element; inside
  `cellXfs` two `xf`s follow, the first with `fontId` F, `fillId` I,
  `applyFont="1"` and `applyFill="1"` and every other attribute
  A's base's, the second `<xf numFmtId="0" fontId="F" fillId="0"
  borderId="0" xfId="0" applyFont="1"/>`, and `count` grows by two;
  the worksheet as `specs/0012` P2's comparison with `changed` equal
  to `styled`, `added` 1 at `(M + 1, 1)` with `s` X + 1, `removed`
  0; `read_back` equal to `styled + 1`; totals `xfs_after` 1046,
  `fonts_added` 27, `fills_added` 27.
- P2: as P1 over `borders` and `cellXfs` (the bytes before
  `<borders`, from `</borders>` to `<cellXfs` and after `</cellXfs>`
  identical): one `border` byte-equal to B1's element; the first
  `xf` with `borderId` B, `applyBorder="1"`, `applyAlignment="1"`
  and `applyProtection="1"`, its content G1's `alignment` element
  followed by the `protection` element then, when A's base had
  children other than those two, those children's bytes (the log
  checks the base's content by `ElementTree` element identity: the
  same children as the base with `alignment` and `protection`
  replaced), the second `<xf numFmtId="0" fontId="0" fillId="0"
  borderId="B" xfId="0" applyBorder="1" applyAlignment="1">
  <alignment horizontal="right"/></xf>`; `borders_added` 27 in
  total; the worksheet as P1. The check prints no reference, no
  code and no text.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. From part 8's decision
3, every phase's run log also runs the edit open check on the host
over that phase's keep.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says: the doc commit,
the code commit(s), one record commit with the logs and the check
answers; logs in `records/logs/m2p2-<phase>/` (`gate.log`,
`test.log` on both targets, `negative.log`, `run.log`, `scan.log`,
`size.log` in P3); every rule the earlier parts added (`specs/0013`
"Phases" lists them; `specs/0014` P2 verification rulings 4, 5 and
7; `specs/0015` P2 verification ruling 2 and close-out ruling 6;
`specs/0016` P4 ruling 8, a claim that a planted change is caught
is a measurement that appears in `test.log` or is not made; the
carried rules of `specs/0016`'s close-out ruling 4: every
placeholder a log prints has its own root line, checked by the scan
as a structural failure; a superseded run directory is moved, never
removed; a per-commit claim needs a logged run); the scan log last;
at most two independent checks; report; STOP. No push during the
part; the close-out push follows P3 and carries the record commit
e81bf5b of part 1's close-out with this part's.

Part 2's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0016-new-workbook-and-sheets.md` as planning left them after
part 1's close-out (the close-out rulings and the two ROADMAP rows),
and this spec. Part 2's first code commit carries the sentence in
`archive.rs`'s module header that `Archive` and the reader
`Archive::read` returns are not `Send` or `Sync` since P5
(`specs/0016` close-out ruling 4), before any other change.

### P1: the types, fonts and fills, the five-container rewrite

The doc commit; the first code commit as above; then the counts and
accessors, the two-containers message widened, `Style`, `Font`,
`Color`, `Fill` and their keyword types, `Border`, `Alignment` and
`Protection` as types with their composers (so P2 adds no type),
`add_style` with `add_cell_style` over it, the recorded collections,
the rewrite over five containers, the three messages, and
`--font-sample`. Tests on byte-built parts: the counts on the
template part and on a part with no `fonts` (`None`, and a font
addition refused, nothing recorded); a second `fonts` refused; a
font with every field, written and the part's bytes compared to
the expected element; each keyword table pinned against the
schema's spelling, printed by the log; a colour in each of the four
forms; `number_text` forms for a size and a tint; the name refusals
and the range refusals (size zero, negative, NaN; tint 1.5;
rotation 181 refused and 180, 0 and 255 accepted); two equal fonts
one element and one id, two fonts differing in one field two; a
fill with both colours, one, none; the derived `xf` from a base
with all five ids (values replaced, flags set, order kept), from a
base lacking `fontId` and `applyFont` (appended in the stated
order), from no base; the base's content kept raw when no
alignment or protection is given; the rewrite growing `fonts`,
`fills` and `cellXfs` with `numFmts` and `borders` byte-identical,
an empty `<fonts/>` opened up, a `count` disagreeing with the
children corrected, a part with `cellXfs` before `fonts` (a
non-schema order: each container still found by name); the part 5
tests unedited and passing; the tool's line. The negative log
through the binary: `--font-sample` on a workbook whose styles part
lacks `fonts` (refused, exit 1, no output; a fixture built from the
template with the collection removed). The run log: `--font-sample`
on both targets, the checks as "Harness runs", the keep and the
edit open check, recorded as `records/m2p2-p1-<utc-date>-<shortsha>.
md` and `.txt`.

### P2: borders, alignment and protection

The doc commit (the P1 rulings and verification); then the `xf`
content replacement (the reader's two byte ranges per `xf`, the
derived `xf`'s content rule), the `border`, `alignment` and
`protection` composers wired into `add_style`, and
`--border-sample`. Tests: a border with every side and both
diagonals, with one side, with none (five empty children); each
border style keyword pinned; an alignment with every field, with
none (`<alignment/>`); a protection with each field; the derived
`xf` from a base with `alignment` alone (replaced), with `alignment`
and `protection` (both replaced), with `protection` alone and an
alignment given (`alignment` inserted first), with an `extLst`
child (kept raw after the two), from a base with no children
gaining both (written with content and an end tag), from a base
with children given nothing (bytes identical to part 5's result);
the rewrite growing `borders`; the part 5 and P1 tests unedited;
the tool's line. The negative log: `--border-sample` on a part
lacking `borders` (refused). The run log: `--border-sample` on both
targets, the checks as "Harness runs", recorded as
`records/m2p2-p2-<utc-date>-<shortsha>.md` and `.txt`.

### P3: the catalogue and limits, the probe, the size record

The doc commit (the P2 rulings and verification); then the
catalogue and the limits in the `package` and `styles` module
headers (the caller's picture; what is not read; what cannot be
written; the Excel check pending), `probe-emend-fullstyles-write-
xml` measured as "Size" says, recorded as
`records/size-m2p2-<utc-date>-<shortsha>.md` with `size.log`
printing every figure the record carries and the fourteen probes
classified against part 1's record; the negative log through the
binary with every refusal of this part reachable through the
binary run once (the two collection refusals; a name or a range
refusal is not reachable, since the tool's constants are fixed, and
the log says so); the corpus run `--font-sample` and
`--border-sample` again on both targets with the release binaries
the size log hashed, every output byte-identical to P1's and P2's
run directories' outputs by sha256, recorded as
`records/m2p2-p3-<utc-date>-<shortsha>.md` and `.txt`. The report
states the part's own cost, the whole growth and the budget figure
against the ceiling decision 1 set, and the headroom left for part
3.

### Close-out push (after P3)

As part 1's, with the self-scan rule on the push and ci-run scripts
(`specs/0001`), one record commit after the push, the scan rerun
last, the `ANTHROPIC_API_KEY` check. The push carries e81bf5b and
this part's commits, on planning's word and the maintainer's
authorisation typed in the implementing session.

## Decisions for the maintainer

Decided by the maintainer 2026-09-25: 1 as A, 2 to 4 as
recommended.

1. The size ceiling, as "The size ceiling" states: A, a Milestone 2
   ceiling of 458752 section bytes judged at the close of part 3
   with a checkpoint after this part (recommended); B, the ceiling
   unchanged and this part behind the feature `styles`; C, build in
   the core and move behind the feature only if P3's record
   overruns. Taken before P1's code: A.
2. The field set: the ten `Font` fields, pattern fills with two
   colours, five border sides, six alignment fields and two
   protection fields as designed (recommended); the alternative
   adds openpyxl's remaining fields (`charset`, `outline`,
   `shadow`, `condense`, `extend`; gradient fills; inside-of-range
   border sides; `readingOrder`, `justifyLastLine`,
   `relativeIndent`), of which the corpus writes only `charset` (39
   fonts of 399), at a cost planning guesses at 3000 to 5000
   section bytes and a wider surface to keep.
3. Deduplication among this package's additions only (recommended):
   two equal fonts recorded by one package share one element, and
   nothing is compared against the part's own fonts, fills and
   borders, which stay unread. The alternative reads the three
   collections' elements as raw bytes and compares a composed
   element against them; since a producer's spelling (attribute
   order, `val="true"` against no `val`) differs from the crate's,
   the comparison would rarely match, and the corpus shows producers
   write duplicates themselves.
4. A missing `fonts`, `fills` or `borders` refused at `add_style`
   (recommended), the part 5 precondition made a refusal; the
   alternative creates the collection at the rewrite in schema
   position, as part 5 creates `numFmts`, which is more code for a
   part Excel refuses anyway (Excel check 1) and no corpus part
   needs.

## Planning rulings during P1 (2026-09-25)

Asked by the implementing session before P1's dependent code, after
it checked every code fact in "Design" against the source and found
none false.

1. The rewrite's refusal for a `fonts`, `fills` or `borders`
   collection missing at the write ("Module `package`", "the same
   message") is the new string "a style addition needs a collection
   the styles part lacks", the one `add_style` raises, not the
   `cellXfs` message, whose text names `cellXfs`. The catalogue
   entry for the new string lists both tests: the refusal at
   `add_style` on a part without the collection, and the rewrite's
   refusal through a byte-built part with the cached read swapped.
   The `cellXfs` message and its test are unchanged.
2. "The part 5 tests unedited and passing" (P1's test list) and the
   widened two-containers message ("The counts") are read together:
   the two part 5 tests that pin the old string, `styles::tests::
   a_second_num_fmts_is_refused` and `a_second_cell_xfs_is_refused`,
   change in their expected string alone, and every other part 5
   test is unedited. The catalogue entry keeps naming those two
   tests beside the new ones for `fonts`, `fills` and `borders`.
   The old string in six committed Milestone 1 negative logs stays:
   a log records what its commit printed.

3. What P1's `add_style` does with `border`, `alignment` and
   `protection`, which "P1" and "P2" left unsaid. Neither a silent
   no-op nor a temporary refusal: a committed state has no
   behaviour a test does not pin and no message a later phase
   removes. `border` is wired end to end in P1 (the id with the
   equality lookup, the append to `borders`, `borderId` and
   `applyBorder` on the derived `xf`, the refusal for a part
   without `borders` at `add_style` and at the rewrite), since the
   composer and the five-container rewrite are P1 work already; P1's
   tests cover it as they cover fonts and fills (a border addition
   refused on a part without `borders`; two equal borders one id).
   `Style` has no `alignment` and no `protection` field at P1; P2
   adds the two fields with the content replacement, and P1's
   `Alignment` and `Protection` types and composers land with their
   tests as "P1" says. A `Style` built with `..Default::default()`
   compiles unchanged across the two phases; the P1 doc of `Style`
   says the two fields come with P2. `--border-sample` stays in P2,
   since it needs alignment. "P1" and "P2" read with this ruling.

4. A correction of "P1": the negative log's `--font-sample` on a
   workbook whose styles part lacks `fonts` is refused with exit 2,
   the message "a style addition needs a collection the styles part
   lacks" on stderr, and no output. The tool exits 2 on every error
   (`tools/emend-copy/src/main.rs`, `EXIT_ERROR`), as part 1 P4's
   negative log and `specs/0012` and `specs/0016` expect; "exit 1"
   was planning's slip, written without reading the constant. The
   tool is unchanged. P2's negative log reads the same way.
5. The implementing session's addition of `styles/elements.rs` as a
   fifth source to the test that holds the catalogue and the source
   to the same set of messages is accepted: the two field messages
   raised there would otherwise be unchecked. It is an edit to an
   existing test beyond ruling 2's two strings, named in the report
   as such.

## Planning verification and rulings on P1 (2026-09-25)

Verified by planning from primaries, never from the report alone:

1. Commits, from git: b3152ca (docs, the three files at planning's
   hashes), 05cc43a (the archive header sentence alone), f14885f
   (the crate), cc81810 (the tool, the phase commit), 6080f2a (the
   record, ten files), a chain from e81bf5b; every message ASCII,
   ending with the Co-Authored-By line, no session link; each commit
   planning docs alone, code alone or records alone; nothing pushed.
   The six committed logs and the check answer at the sha256 the
   report gives. Planning's own `scripts/privacy.sh --range
   e81bf5b..HEAD`: 5 commits, 605 blobs, 0 findings; no log or record
   names the home directory; the three `specs/0001` shape greps over
   every printed script find 0, 0 and 0; `ANTHROPIC_API_KEY` unset.
2. The logs, read at their verdict lines: gate attempt 2 "checks all
   as expected: yes", its header naming attempt 1's log, stderr and
   script by hash; test "yes", host and i686 each 619 passed, 0
   failed, 2 ignored, 34 test names added and 0 removed, 54 keyword
   lines against 54 pinned rows; negative "yes", F0 the control, F1
   and F2 exit 2 with the collection message, F3 exit 2 with the
   widened message, no output in each; run "yes", 27 of 27 on both
   targets with every comparison flag holding, totals `xfs_after`
   1046, `fonts_added` 27, `fills_added` 27, 22 plants of 22 turning
   their own flag, host and i686 outputs byte for byte the same in
   27, the keep 210 pairs with 201 found in part 1 P5's record by
   name, bytes and hashes and 9 new, the open record 210 pairs and 0
   findings; scan final stage PASS, 0 hits, 0 structural failures.
3. Planning's own measurements: the workspace tests on planning's
   build at cc81810, 619 passed, 0 failed, 2 ignored; clippy with
   `-D warnings` clean; planning's own comparison (an `ElementTree`
   and byte-span reader written before the run, with a negative
   control over part 1 P5's run directory reporting a problem on all
   27) over the phase's run directory: 27 of 27 with no problem on
   both labels, `xfs_after` 1046, `fonts_added` 27, `fills_added`
   27, `read_back` 54, and the host and i686 outputs equal by sha256
   in 27 of 27.
4. Check 1's F1, the font name's escape: the check read "the crate's
   `escape`" as a function that also writes a tab, a line feed or a
   carriage return as a character reference, which only the number
   format code's path does (`specs/0012` P1 ruling 3). The spec meant
   what `specs/0012` names by those words, quick_xml's `escape`, and
   the code does that; the check's reading is refuted as a defect.
   The point under it stands: a name holding one of those three
   characters would be written raw and read back as a space by any
   reader, a silent change. Ruled: P2's first code commit gives the
   font name the number format code's path, `escape` then the three
   references, with a test that a name holding a tab writes `&#9;`;
   "the elements" reads so from P2.
5. Check 1's F2: the rewrite passes every one of the five containers
   through the count rewrite, grown or not, so a write that adds
   only a number format corrects a disagreeing `fonts`, `fills` or
   `borders` count. That is what "Module `package`" asks ("as
   today") and the module header's limits bullet says "grows",
   which is narrower. Ruled: the wording is corrected and a test
   pins the no-addition, disagreeing-count case, both in P2's first
   code commit.
6. Check 1's F7, the placeholder rule: `specs/0016` P5 verification
   ruling 3 says a log "declares each placeholder it uses in a root
   line", and this spec's "Phases" paraphrased it as "its own root
   line". The ruling's wording holds: a declaration `<word> stands
   for` anywhere in a root line declares the word, and `negative.log`
   line 39 declares `<tree>` inside the `<build>` root line lawfully.
   The pre-check scan's attempt 2, which tests that form and names
   the change in its header, is the rule's instrument; "Phases" is
   read with this ruling. Not a loosening: the first form tested a
   paraphrase.
7. Check 1's F8 and the implementing session's proposal: no log
   records its own script's stderr, so the failure that forced the
   gate rerun is invisible in a log. Ruled, from P2: every log script
   redirects its own standard error to a file under `<scratch>` at
   its start (`exec 2>"$ERR"`), and its last block prints that file's
   byte count as a check expecting 0, with a control that writes one
   byte to a copy and reports it; a rerun header that cites a
   superseded attempt's stderr cites that file by hash.
8. Check 1's F9: `removed_in` maps every `git show` failure to an
   empty file. Ruled: P2's gate script asks `git cat-file -e` whether
   the path exists at the base and treats only that as "no lines";
   any other failure is a failed check.
9. Check 1's F12: the superseded scripts a rerun header hashes are
   given to the checker with the superseded logs from P2, so the
   hashes can be verified.
10. The eight disclosed decisions are accepted: composition at the
   call with the prefix filled at the rewrite; `expect(dead_code)`
   under `cfg_attr(not(test))` on the alignment and protection
   composers and the two keyword tables, removed in P2 when they are
   wired; `Copy` where possible and `Eq` on the field-less enums,
   `Alignment` and `Protection` (not on `Color`, which holds an
   `f64`); the refusal order base, format, font, fill, border; the
   header sentences the change made false corrected in P1; the
   negative log's F2 and F3; each plant on the first workbook where
   it applies; the placeholder reading of ruling 6. F3, F4, F5, F10,
   F11, F13 and F14 are remarks, answered in the report as the
   report says.
11. P1 is DONE. P2's first doc commit carries this spec (rulings 1 to
   5 during P1 and this section) and `ROADMAP.md`, unedited at the
   hashes planning sends; P2's first code commit carries rulings 4,
   5, 7 (the scripts) and 8 before any P2 change, and the dead-code
   attributes go when the composers are wired.

## Planning verification and rulings on P2 (2026-09-25)

Verified by planning from primaries, never from the report alone:

1. Commits, from git: 06d485b (docs, the two files at planning's
   hashes), 24ce03b (P1 verification rulings 4 and 5), 4bd46a1
   (alignment and protection, the child spans, the content
   replacement), 09c941f (the tool, the phase commit), ebfe711 (the
   record, ten files), a chain from 6080f2a; every message ASCII,
   ending with the Co-Authored-By line, no session link; each commit
   planning docs alone, code alone or records alone; nothing pushed.
   The six committed logs and the check answer at the sha256 the
   report gives. Planning's own `scripts/privacy.sh --range
   6080f2a..HEAD`: 5 commits, 617 blobs, 0 findings; no log or record
   names the home directory; the three `specs/0001` shape greps over
   every printed script find 0, 0 and 0; `ANTHROPIC_API_KEY` unset.
2. The logs at their verdict lines, every one attempt 1: gate "checks
   all as expected: yes"; test "yes", host and i686 each 633 passed,
   0 failed, 2 ignored, 14 test names added and 0 removed, 54 keyword
   lines against 54 pinned rows; negative "yes", B0 the control, B1
   exit 2 with the collection message, B2 exit 2 with the widened
   message, no output in each; run "yes", 27 of 27 on both targets
   with every comparison flag holding, `xfs_after` 1046,
   `borders_added` 27, 18 plants of 18 turning their own flag, host
   and i686 outputs byte for byte the same in 27, the keep 220 pairs
   with P1's 210 found by name, bytes and hashes, the open record 220
   pairs and 0 findings; scan final stage PASS, 0 hits. Each of the
   five logs prints its own stderr as 0 bytes with the one-byte
   control at 1 (P1 verification ruling 7, in force).
3. Planning's own measurements: the workspace tests on planning's
   build at 09c941f, 633 passed, 0 failed, 2 ignored; clippy with
   `-D warnings` clean; planning's own comparison (the P1 reader
   extended to borders and the two children, with a negative control
   over P1's run directory reporting a problem on all 27) over the
   phase's run directory: 27 of 27 with no problem on both labels,
   `xfs_after` 1046, `borders_added` 27, `read_back` 54, host and
   i686 outputs equal by sha256 in 27 of 27. Planning's first pass
   reported every file on the second cell format; that was
   planning's expectation carrying the first format's alignment, and
   the output holds the spec's `horizontal="right"` alone.
4. Check 1's F1 and disclosed decision 1: the two P1 tests that named
   every `Style` field gained `..Style::default()` when P2 added two
   fields. Accepted, pinned by gate.log as the only lines added
   inside base tests with the tool test's four. P1 ruling 3's
   "compiles unchanged across the two phases" held for a literal
   built with `..Default::default()` and was written without reading
   the two literals that were not: planning's slip, corrected here.
5. Check 1's F3, a behaviour change P1 did not have: the reader now
   resolves the name of every direct child of a kept `xf` to find
   its `alignment` and `protection`, so a child whose prefix no
   declaration binds is refused where P1 copied it raw. The refusal
   stands. The crate refuses an unbound prefix wherever it resolves a
   name (`xml.rs`, `resolve_element`; `package::tests::
   rel_id_prefix_unbound`); a part with one is not namespace-well-
   formed and no corpus part has one. The implementing session's
   proposal to read a failed resolution as "not our child" is
   declined: it would have one reader path accept what every other
   refuses. P3's first code commit adds a `styles` limits sentence
   and a test through `Package::styles` on a byte-built part.
6. Check 1's F7: the gate script's carry note names P1's log by the
   P2 path, a rename slip in a comment; gate.log stands as it ran,
   and P3's scripts carry the corrected note. F9: the absent-path
   control checks the exit status too from P3's gate. F10: P3's run
   log, which runs the border comparison again, adds a plant that
   reorders the base's other children so `first_xf_content`'s
   `ElementTree` half is shown alone.
7. Disclosed decisions 2 to 5 accepted: the content placement as
   implemented is the spec's rule read case by case (a replaced
   child stays in place; a new protection follows the first
   alignment, else goes first); only the first main-namespace direct
   `alignment` and `protection` children are recorded and one inside
   `extLst` is kept raw, stated in the limits with P3's catalogue
   work; the carried summary wording; `head_same` where the input
   counts agree. F2, F4, F5, F6, F8 and F11 are remarks, answered in
   the report.
8. P2 is DONE. P3's doc commit carries this spec and `ROADMAP.md`,
   unedited at the hashes planning sends; P3's first code commit
   carries ruling 5's test and sentence before any P3 change; P3's
   scripts carry ruling 6.

## Planning rulings during P3 (2026-09-25)

1. A correction of "P3": the runner takes one `--label`, host or
   i686, and the record script reads one run directory with those
   two labels (`scripts/passthrough-run.sh`, the `--label` check;
   `scripts/passthrough-record.py`, `LABELS`), so the two samples
   cannot share a run directory and one record pair cannot cover
   both. P3 runs `--font-sample` into `runs/<date>-<sha>-font` and
   `--border-sample` into `runs/<date>-<sha>-border`, on both
   targets each, from one run log; the records are
   `records/m2p2-p3-font-<utc-date>-<shortsha>.md` and `.txt` and
   `records/m2p2-p3-border-<utc-date>-<shortsha>.md` and `.txt`;
   every font output is byte-identical by sha256 to P1's run
   directory's output for the same corpus file and every border
   output to P2's; the run log runs the font and border comparisons
   again, the border one with ruling 6's reorder plant; the keep and
   the edit open check run once, recorded as
   `records/m2p2-open-<utc-date>-<shortsha>.md` and `.txt`. No
   script changes.

2. The checkpoint figure. "Size" names the part's own cost as the
   new probe over `probe-emend-sheets-write-xml`, which the size log
   measured at 2036 section bytes; that figure understates the part,
   since `add_cell_style` now runs through `add_style` and every
   probe that adds a style links the composers, so the part's cost
   shows as drift of the earlier probes. The figure the checkpoint
   and the maintainer's decision 1 judge is the whole growth of the
   budget figure since part 1's record, as `specs/0013` P3
   verification ruling 9 defines a part's cost: 399176 less 375984,
   23192 section bytes (2036 over the sheets probe plus that probe's
   drift of 21156; `probe-zip-xml`'s drift 0), planning's arithmetic
   from the log's figures. 23192 is under the 25000 checkpoint by
   1808 and above planning's expectation of 20000; the checkpoint is
   not tripped and no re-ruling is needed. The budget figure 399176
   is 59576 under the Milestone 2 ceiling of 458752. The committed
   record states 2036 as "the part's own cost" with the split
   beside it; this ruling, not a size-log rerun, says which figure
   is judged, and the ROADMAP row carries 23192.

## Planning verification and rulings on P3 (2026-09-26)

Verified by planning from primaries, never from the report alone:

1. Commits, from git: 1f85cba (docs, the two files at planning's
   hashes), ff76e34 (the unbound-prefix sentence and test), f64063e
   (the limits, doc only), 1fc8327 (the probe, the phase commit),
   76f4461 (the record, fourteen files), a chain from ebfe711; every
   message ASCII, ending with the Co-Authored-By line, no session
   link; each commit planning docs alone, code alone or records
   alone; nothing pushed. The seven committed logs, the check answer
   and the size record at the sha256 the report gives. Planning's
   own `scripts/privacy.sh --range ebfe711..HEAD`: 5 commits, 626
   blobs, 0 findings; no log or record names the home directory;
   the three `specs/0001` shape greps over every printed script find
   0, 0 and 0; `ANTHROPIC_API_KEY` unset.
2. The logs at their verdict lines: gate attempt 2 "checks all as
   expected: yes" (attempt 1 failed three checks of its own script,
   one caught by the stderr count, kept and hashed); test "yes", 634
   passed on each target, 1 added, 0 removed; size attempt 2 "yes"
   with root lines for `<repo>` and `<home>` (attempt 1 lacked them,
   the gap `specs/0016` P5 verification ruling 3 named in the script
   this one was carried from, kept and hashed, its five hash lines
   and every figure identical); negative "yes", the two controls, the
   three collection refusals and the widened message, exit 2 and no
   output each; run attempt 2 "yes" (attempt 1 failed only its own
   new-pairs check, widened to "no new pair, or every new pair kept
   by a test this phase adds"), both samples 27 of 27 on both
   targets, every output byte-identical to P1's or P2's run directory
   by sha256, 22 of 22 and 18 of 18 plants turning their own flag,
   the reorder plant showing the tree half alone, the keep 220 pairs
   with P2's 220 found and 0 new, the open record 220 pairs and 0
   findings; scan final stage PASS, 0 hits. Each of the six logs
   prints its own stderr as 0 bytes with the one-byte control.
3. Planning's own measurements: the workspace tests on planning's
   build at 1fc8327, 634 passed, 0 failed, 2 ignored; clippy with
   `-D warnings` and `cargo doc` clean; planning's own comparison
   readers over both run directories, 27 of 27 with no problem on
   both labels for each sample, and every output equal by sha256 to
   P1's or P2's, 54 of 54 for each; planning's own rebuild of
   `probe-emend-fullstyles-write-xml` and `probe-zip-xml` at 1fc8327
   in the `size` profile reproducing the log's sha256 for the new
   probe (021d75bf...) and the figures 932392 and 533216 section
   bytes, 399176 apart. The budget figure is 399176, 59576 under the
   Milestone 2 ceiling of 458752; the part's whole growth 23192 is
   under the 25000 checkpoint (P3 ruling 2); the headroom for part 3
   is 59576.
4. Check 1's F7 and F17: "P3" said the negative log runs every
   refusal of the part the binary reaches, and its parenthesis left
   out the unbound-prefix refusal, which P2 introduced and either
   sample reaches on a fixture with such an `xf` child; the log's
   own line says so and defers to the crate test. The parenthesis
   was planning's, wrong as exhaustive. Ruled: the case goes into
   part 3's first negative log (a fixture from the template with an
   `xf` child whose prefix no declaration binds, either sample, exit
   2, the message, no output); the close-out adds no log.
5. Check 1's F10 and F19: `run.log` ran against size attempt 1 and
   cites its hash; the committed size log is attempt 2, whose five
   hash lines are byte-identical to attempt 1's, as the implementing
   session and the checker each found by diff and planning by
   reading the four binary hashes out of both logs. Accepted, with
   this ruling as the record of the comparison; from part 3, a size
   log rerun after the run log prints, as a check, that its hash
   lines equal those of the attempt the run log cites.
6. Check 1's F9 and F16: attempt 1's keep was moved, never removed,
   and named but not hashed; the run directories are represented by
   their summary hashes and file counts; the installed record copies
   were removed after a byte comparison with the moved copies, which
   are kept and hashed. Accepted; from part 3 a moved keep is listed
   by its pair count and the hash of its listing, as a run directory
   is.
7. Check 1's F11: the logs pinned the spec at the hash planning gave
   when each started, 609a94f9 (rulings 1 to 5 during P1 and P3
   ruling 1) for gate, run and size attempt 1, and bcab8a1f (P3
   ruling 2 added) for size attempt 2 and the scan; planning restored
   the earlier copy for the run log's duration and re-applied the
   ruling after it, as the messages of the day record. Lawful: a
   mid-phase ruling edits the uncommitted spec while the implementing
   session holds, and each log names the copy it checked.
8. Check 1's F14: the size record's install into `records/` was
   unlogged and the checker found it identical to the record the
   size log printed; from part 3 the size log installs its record
   and prints the installed file's sha256, as the run log installs
   its record pairs.
9. The remaining items are remarks, answered in the report; the `-b`
   suffix on the rerun's directories was unneeded once the date had
   rolled and is harmless. The carried gap in the P5 size script is
   the implementing session's finding, recorded in the size log's
   rerun header.
10. P3 is DONE and the part's code is complete. The close-out
   follows "Close-out push (after P3)": a doc commit with this spec
   (P3 rulings 1 and 2 and this section) and `ROADMAP.md` unedited
   at the hashes planning sends; the push of e81bf5b and this part's
   commits on planning's word and the maintainer's authorisation
   typed in the implementing session, with the push and ci-run
   scripts' self-scan; the ci-run log; one record commit after the
   push under `records/logs/m2p2-close/`; the scan rerun last; the
   `ANTHROPIC_API_KEY` check. Carried into part 3's spec: rulings 4
   to 6 and 8 here, and the Milestone 2 ceiling's headroom of 59576.

## Planning verification and rulings on the close-out (2026-09-26)

1. Verified by planning from git and `gh`: origin/main is
   42588cc4e5bc0d6d7cd15d14a49947c87edf012a, the close-out doc
   commit; the pushed range c115d45..42588cc holds 17 commits, part
   1's record commit e81bf5b among them; 18 of 18 messages from
   c115d45 to the record commit end with the Co-Authored-By line
   only, no session link, ASCII throughout; CI run 36209039533 on
   42588cc, `push` on `main`, completed with conclusion success; the
   repository PRIVATE; planning's own `scripts/privacy.sh --range
   c115d45..HEAD`, 18 commits, 0 findings; `ANTHROPIC_API_KEY` unset
   in planning's shell; the working tree clean.
2. The three logs at the reported hashes. `push.log`: pre mode over
   the base found by its subject, 17 commits, the identity check, the
   hook's 654 blobs with 0 findings, the full gate, the self-scan 0
   of each shape with the controls at 1, "pre checks all as expected:
   yes"; push mode after the maintainer's typed authorisation,
   ls-remote at the recorded base before and at 42588cc after, still
   PRIVATE, "push checks all as expected: yes". `ci-run.log` attempt
   2, the run polled to success, "checks all as expected: yes";
   attempt 1 superseded because a comment in its printed script
   carried a placeholder-shaped path the post scan reads as
   undeclared, reworded and rerun, the attempt kept and hashed.
   `scan.log` post stage attempt 2, 0 hits, 0 structural failures,
   RESULT: PASS, after attempt 1 failed on that comment. Each log
   counts its own stderr at 0 bytes. No log names the home
   directory.
3. The record commit d0f8f72, parent 42588cc, exactly the three logs,
   trailer correct, local: it goes with the next push. Accepted.
4. Carried into part 3's spec: P3 verification rulings 4 (the
   unbound-prefix case in the first negative log), 5 (a size rerun
   after the run log prints the hash-line comparison), 6 (a moved
   keep listed by count and listing hash) and 8 (the size log
   installs its record and prints its hash); a comment in a script
   never spells a placeholder-shaped path it does not declare; the
   Milestone 2 headroom of 59576 section bytes for part 3 against
   458752, judged at the close of part 3 on the last core probe.
5. The close-out is DONE and Milestone 2 part 2 is DONE.

## Out of scope

- Reading fonts, fills, borders, alignment or protection back as
  typed values: `Xf`'s ids and the part's bytes are the read side.
  A later part adds `Styles::font(id)` and its siblings if a caller
  needs them; the harness does not.
- Named styles (`cellStyles`, `cellStyleXfs`), differential formats
  (`dxfs`), table styles, the theme and the indexed palette.
- Editing or removing an existing font, fill, border or cell
  format: everything is appended, as part 5.
- Row and column styles (`<row s>`, `<col style>`): part 3.
- Conditional formatting: part 4.
- Formula evaluation, rendering, autofit and reference fixup, by
  `CLAUDE.md`'s definition.
