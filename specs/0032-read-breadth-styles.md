# specs/0032: read breadth A: named styles, dxfs and colours

Status: APPROVED 2026-10-11 (Milestone 5 part 1; decisions 1 to 3
decided A by the maintainer 2026-10-11, typed in the planning
session).

Milestone 5, "charts and read breadth", was approved by the
maintainer 2026-10-11 (`ROADMAP.md`, "Milestone 5"): reading first,
then charts; everything behind a Cargo feature; the Milestone 2
ceiling kept for the core; the Milestone 2 Excel check gates no push
in this milestone. This part is the first of six: the items
`specs/0027-read-back.md` left out of scope under its decision 3 and
its "Out of scope", read into the types the writers take, with no
writer changed. Part 2 (the workbook survey, pivot tables and
external links) has its own spec.

## Goal

What a cell's format inherits and what a conditional format applies,
read typed: the named styles of `<cellStyles>` with their
`<cellStyleXfs>` formats resolved as `cell_style` resolves a cell
format; the differential styles of `<dxfs>` as the `Style` values
`add_differential_style` takes, so a conditional format read back
through `conditional_formats` names a style a caller can read and
write again; a `Color` resolved to the RGB the workbook defines for
it, through `<colors>`, the default palette and the theme's colour
scheme, with the tint applied by the standard's formula; and the
default font. Every typed value is one a writer of this crate
accepts, which the samples prove on the corpus. Reads only; no
writer changes; everything behind the Cargo feature `read`.

## Size and packaging (decision 1, before any code)

The budget figure after Milestone 4 is 436992 section bytes against
the ceiling of 458752 (`records/size-m4p2-2026-10-10-2c31aad.md`):
21760 of headroom, held for fixes. The feature `read` costs 51988
over the core probe (the read probe 1022196 over the format probe
970208 in the same record). Planning's estimate for this part,
priced from `read`'s own readers (six element readers and a one-pass
sheet reader for 51988), is 14000 to 24000 for three small readers
over elements the styles reader already walks past, a 64-entry
palette, a theme reader of twelve values and a resolver. The core's
growth is what the feature gates add, expected 0; above 2000 section
bytes, or a feature cost above 80000 in all, planning is told in the
report.

- A (recommended): everything under the existing feature `read`, no
  new feature, no new probe: `probe-emend-read-write-xml` grows by
  the four new calls, so one figure moves and the catalogue stays
  one list. 28 probes.
- B: a new feature `theme` for the colour resolution (the theme
  reader and the palette), `read` for the rest, and a 29th probe: a
  consumer resolving no colour carries no palette, at the cost of a
  second feature a reader of styles always wants.

## Facts planning checked (2026-10-11)

- The corpus, counted by planning with Python's `zipfile` and
  prefix-tolerant regular expressions over every regular file of the
  corpus directory that is a zip (27 workbooks), names and counts
  only. `xl/styles.xml`: `<cellStyleXfs>` and `<cellStyles>` in all
  27, with 459 `xf` and 221 `cellStyle` children (at most 25 of each
  in one workbook); every `cellStyle` carries `name` and `xfId`, 185
  a `builtinId`, 71 `customBuiltin`, none `hidden` or `iLevel`; every
  one of the 992 `<cellXfs>` `xf` carries an `xfId`, the largest 24.
  `<dxfs>` in the main namespace in 16 workbooks (the 4 further
  matches are `x14:dxfs` inside `extLst`), 1360 `dxf` children in
  the main namespace (at most 440 in one; P1 ruling 2 corrected
  these from counts that included the 31 `x14` ones): 964 hold a
  `font`, 459 a `fill` (every one a `patternFill`, none a
  `gradientFill`; 179 with no `patternType`, 213 `none`, 67
  `solid`; `bgColor` on all 459, `fgColor` on 186), 260 a `border`
  (88 of them, in 9 workbooks, with `horizontal` or `vertical`
  children, 86 and 78, 103 of those with a `style` and a `color`;
  `outline` on 116), 495 an `alignment`, 147 a `numFmt`, 14 a
  `protection`, none an `extLst` or nothing. No `<borders>` border
  of the corpus has a `horizontal` or `vertical` child. Colour
  forms over the part: `rgb` 1513, `theme` 1783 (the largest index
  11; 218 in `<fonts>`, 64 in `<fills>`, 259 in `<borders>`, 1242
  in `<dxfs>`, none in `<cellStyleXfs>`), `indexed` 327 (the largest
  81; 4, 64, 4 and 255 in the same four), `auto` 194, `tint` 1082.
  `<colors>` in 23 workbooks, `<indexedColors>` in 18 with 1016
  `rgbColor` children in all, `<mruColors>` in 5. Every workbook has
  one `xl/theme/theme<n>.xml` and one `theme` relationship in
  `xl/_rels/workbook.xml.rels`; every theme part's root is `a:theme`
  with the DrawingML namespace declared, holding one `a:clrScheme`
  with its twelve children (324 in all), 318 `a:srgbClr` and 6
  `a:sysClr` (each with `lastClr`), and one `a:fontScheme` with 54
  `a:latin`; the largest theme part is 7716 bytes.
- `specs/0027`'s fact "1451 `xf` under `cellXfs`" counts both lists:
  992 under `<cellXfs>` and 459 under `<cellStyleXfs>`, as its own
  "`xfId` 992" shows.
- `Styles` (`styles.rs`) reads `<numFmts>`, `<fonts>`, `<fills>`,
  `<borders>` and `<cellXfs>` in one read (`Styles::read`, a
  dispatch on the root child's local name) and walks every other
  root child past with its subtree; its module header says
  `cellStyleXfs`, `cellStyles`, `dxfs`, table styles, colours and
  extensions are not modelled and nothing about them is read. `Xf`
  carries `xf_id: Option<u32>`, the index into `<cellStyleXfs>`,
  public. With the feature `read` the same read fills `read::Typed`
  (fonts, fills, borders typed, each `xf`'s alignment and
  protection) through `read_font`, `read_fill`, `read_border`,
  `read_alignment` and `read_protection` (`styles/read.rs`), and
  `Package::cell_style(index)` resolves a cell format into
  `CellStyle` (six fields). `read_fill` reads a `patternFill` with
  no `patternType` as `Pattern::None`; `Fill::compose` always writes
  `patternType`.
- `Color` has four forms: `Rgb(u32)`, `Theme { index, tint }`,
  `Indexed(u32)`, `Auto`. No code resolves a theme or an indexed
  colour to RGB; `set_tab_color` refuses a tint outside
  `-1.0..=1.0` ("a sheet setting is out of range"). The theme part
  is located already: `Workbook::theme` is the resolved part of the
  workbook's first `theme` relationship (`THEME`, transitional then
  strict), read by `Package::workbook`; nothing reads the part.
  The DrawingML main namespace is a constant in `package.rs`,
  transitional and strict spellings, used by the drawing readers.
- `Package::add_differential_style(&Style) -> Result<u32, Error>`
  (feature `rules`) appends a `<dxf>` holding the style's font, fill
  and border, in that order, as `add_style` writes them, and refuses
  a number format, an alignment or a protection ("a differential
  style holds a field this part does not write"); its doc says
  Excel reads a differential solid fill's colour from `bgColor`.
  `Style` has six `Option` fields; `NumberFormat` is `BuiltIn(u32)`
  or `Code(String)`; `Font` has ten fields, `name` and `size`
  optional.
- The gate's test counts after Milestone 4 (`records/logs/m4p2-p2/
  gate.log`): 514 with the features off, 526 with `copy`, 558 with
  `comments`, 573 with both, 918 with every feature, on each target.
  The API listing holds 544 items
  (`records/api-review-m4p1-2026-10-10-669fa91.md`).
- openpyxl, planning's reading of 3.1, read only: `wb._named_styles`
  carries each `cellStyle` with its `cellStyleXfs` format resolved
  into font, fill, border, alignment, protection and number format,
  and `builtinId`, `hidden`; `wb._differential_styles` carries each
  `dxf` as a `DifferentialStyle` of the same six parts; a `Color`
  keeps its `theme`, `indexed`, `tint` and `auto` as read and
  openpyxl resolves nothing to RGB (its `COLOR_INDEX` table is the
  legacy palette a caller consults by hand); the theme part is kept
  as bytes (`wb.loaded_theme`) and written back.
- ECMA-376 Part 1, 18.8.27 (`indexedColors`) defines the default
  palette of 64 entries, indexes 0 to 63, the first eight black,
  white, red, green, blue, yellow, magenta and cyan, and 8 to 15
  the same eight again; a `<colors><indexedColors>` replaces the
  entries from 0 in the order of its `rgbColor` children. 18.8.19
  (`tint`): a tint in `-1.0..=1.0` is applied in HLS space with
  HLSMAX 255: for a tint below zero, the luminance is multiplied by
  (1.0 + tint); above zero, by (1.0 - tint), then raised by HLSMAX
  times tint. 20.1.6.2 (`clrScheme`): the twelve children in order
  `dk1`, `lt1`, `dk2`, `lt2`, `accent1` to `accent6`, `hlink`,
  `folHlink`, each holding one `srgbClr` (`val`, six hex digits) or
  `sysClr` (`val` a system name, `lastClr` the RGB last seen). The
  `theme` attribute of a colour indexes the scheme in the order
  `lt1`, `dk1`, `lt2`, `dk2`, `accent1` to `accent6`, `hlink`,
  `folHlink`: the first two pairs swapped against the element
  order (18.8.3 `bgColor`/`fgColor`, "theme": "index into the
  `<clrScheme>` collection"; the swap is what Excel and openpyxl's
  documentation describe, and the corpus check below proves it on
  every workbook: theme 1 on a font is the text colour, dark, and
  theme 0 on a fill the window colour, light). Indexes 64 and 65
  are the system foreground and background, and the legacy indexes
  up to 81 name system colours: the file defines no RGB for them.

## Design

### Crate

`zip` and `quick-xml` only; no new dependency. Under decision 1 A
everything below sits behind the Cargo feature `read`, off by
default: new calls on `Package`, new types, and the typed fields
`Styles` gains, all gated. The core builds, tests and links without
it; the tool turns it on. Nothing existing changes signature;
`CellStyle`, `Style` and `Color` are unchanged.

### Named styles

`Styles` gains, with the feature, `cell_style_xfs: Vec<Xf>` (the
`xf` children of `<cellStyleXfs>`, read by `read_xf` as `<cellXfs>`'s
are, with their typed alignment and protection) and `cell_styles:
Vec<NamedStyle>`, both in part order, parsed in the same one read of
the part. `NamedStyle { name: String, xf_id: u32, builtin_id:
Option<u32>, custom_builtin: bool, hidden: bool, outline_level:
Option<u32> }` (the attributes `name`, `xfId`, `builtinId`,
`customBuiltin`, `hidden`, `iLevel`), derived `Clone`, `Debug`,
`PartialEq`, `Eq`. `Package::named_style(index: u32) ->
Result<CellStyle, Error>` resolves `cell_style_xfs[index]` exactly
as `cell_style` resolves `cell_xfs[index]`, through one shared
resolver; a caller joins a cell to its named style by
`styles().cell_xfs[s].xf_id`, public already, and reads the name
from `styles().cell_styles`. Nothing is added to `CellStyle`. A
`cellStyle` without `name` or `xfId`, or an `xfId` past
`<cellStyleXfs>`, is refused at the read, "a style cannot be read";
`hidden` and `customBuiltin` read as the schema's booleans, absent
false.

### Differential styles

`Styles` gains, with the feature, `dxfs: Vec<Style>`, the `dxf`
children of the main namespace's `<dxfs>` in part order (an
`x14:dxfs` inside `extLst` is not one and is walked past as today),
each read into the `Style` the writers take: `number_format` from
`<numFmt>`'s `formatCode` as `Code` (a `dxf` number format always
carries its code); `font`, `fill`, `border` through the three
readers `read` has; `alignment`, `protection` through its two; each
`None` when the `dxf` has no such child. A `dxf`'s `<fill>` holding
no `patternFill` (a gradient) reads as `fill: None`, as a cell
format's does (`specs/0027` decision 3). A `dxf`'s `patternFill`
with no `patternType` reads as `Pattern::Solid` (decision 2 A):
Excel renders it solid from `bgColor`, which is what
`add_differential_style`'s doc already says a caller writes; a
cell format's `patternFill` with no `patternType` keeps reading as
`Pattern::None`, unchanged. `Package::differential_styles(&mut
self) -> Result<&[Style], Error>` lends them (the Cargo feature
`read`; a `dxf` index a `Rule` names is its position). Every
`Style` read whose fields are a font, a fill and a border alone is
one `add_differential_style` accepts, and writing it back appends
a `<dxf>` equal child by child to the one read (the written
`patternType="solid"` against the read `patternFill` with none as
the one named difference), which the round-trip sample proves. A
`dxf` child outside the main namespace or unknown to the type, or
a value that does not parse, is refused at the read, "a style
cannot be read".

### Colours

- The palette. `Styles` gains, with the feature, `indexed_colors:
  Option<Vec<u32>>`, the `rgbColor` children of `<colors>`'s
  `<indexedColors>` in order, each `rgb` as `Color::Rgb` parses
  one, `None` without the element; `<mruColors>` is walked past.
  The default palette of ECMA-376 18.8.27 is a 64-entry constant
  under the feature, transcribed from the standard's table by the
  implementing session (not from any other project's source), its
  entries 0 to 15 asserted by a test from the facts above.
- The theme. `Package::theme_colors(&mut self) ->
  Result<Option<&[u32; 12]>, Error>` reads the workbook's theme
  part (`Workbook::theme`; `None` without a relationship) once and
  keeps the twelve scheme colours in element order (`dk1` first),
  each `srgbClr`'s `val` or `sysClr`'s `lastClr` as an opaque ARGB
  value; the part is streamed and only `<a:clrScheme>`'s subtree is
  read, everything else walked past, so the hold is 48 bytes. The
  DrawingML namespace, transitional or strict, is required on the
  element; a scheme child with neither colour element, a `val` that
  is not six hex digits, or a `sysClr` without `lastClr` is refused,
  "a theme cannot be read".
- The resolver. `Package::resolve_color(&mut self, color: &Color)
  -> Result<Option<ResolvedColor>, Error>` gives `ResolvedColor {
  rgb: u32, tint: Option<f64> }` (derived `Clone`, `Copy`, `Debug`,
  `PartialEq`): for `Rgb(v)`, `v` with no tint; for `Indexed(i)`,
  the palette's entry `i` (the part's `indexedColors` when it has
  one and `i` is within it, else the default's) with no tint, and
  `None` for an index past 63 (the system colours, decision 3 A);
  for `Theme { index, tint }`, the scheme colour at the theme index
  through the swap above (0 `lt1`, 1 `dk1`, 2 `lt2`, 3 `dk2`, 4 to 9
  the accents, 10 `hlink`, 11 `folHlink`) with the tint carried,
  `None` without a theme part, and a refusal "a colour cannot be
  resolved" for an index past 11 or a tint outside `-1.0..=1.0`
  or not finite; for `Auto`, `None`. `ResolvedColor::tinted(&self)
  -> u32` applies the tint by the standard's HLS formula with
  HLSMAX 255, each channel rounded to nearest, the alpha byte
  kept, and gives `rgb` unchanged for no tint. Nothing is cached
  but the theme; a resolve is a lookup.

### The default font

`Package::default_font(&mut self) -> Result<Font, Error>`: the
font the named style at `cellStyleXfs` index 0 names (the `Normal`
style in every workbook Excel or LibreOffice writes), font 0 when
that `xf` has no `fontId` or the part has no `<cellStyleXfs>`; the
`<font>` read through the font reader `read` has. A part with no
`<fonts>` is refused, "a style cannot be read".

### Errors

Two new `Error::Package` messages, in the catalogue as the earlier
parts' are, marked by feature:

`read`:
- "a theme cannot be read" (at the theme read: a root or a scheme
  child outside the DrawingML namespace, a scheme child with no
  colour element, a `val` that is not six hex digits, a `sysClr`
  with no `lastClr`, two `clrScheme` elements)
- "a colour cannot be resolved" (at the resolve: a theme index past
  11, a tint not finite or outside `-1.0..=1.0`)

Shared: "a style cannot be read" (`specs/0027`), now also for a
`cellStyle` with no `name` or `xfId`, an `xfId` past
`<cellStyleXfs>`, a `named_style` index past the list, a `dxf`
child the type cannot hold, and a part with no `<fonts>` at
`default_font`; "no such part" for a theme relationship naming a
part the archive lacks; part 2's unbound-prefix refusal;
`Error::Xml` for bytes that are not UTF-8. Nothing is checked
twice.

### 32-bit and streaming

Every count is `u32` or `u64`; the styles part is held as today,
its new vectors bounded by the part (the corpus's largest `<dxfs>`
holds 440 `dxf`, its largest `<cellStyles>` 25); the theme part is
streamed and 48 bytes kept; the palette is 256 bytes of constant.
No new whole-part hold.

### Size

Under decision 1 A `probe-emend-read-write-xml` grows: after its
reads today, `named_style` of every `cellStyleXfs` index,
`differential_styles` (the count printed), `resolve_color` of every
font colour read (the resolved count printed) and `default_font`
(its size printed). The size log measures as part 2's did,
classifies the twenty-eight probes of
`records/size-m4p2-2026-10-10-2c31aad.md` against it, prints the
feature's cost (the read probe minus the format probe rebuilt at
the same commit), the core's growth (the format probe at the commit
minus the same probe rebuilt at 2c31aad, both printed; expected 0),
the budget against 458752, and a cause per probe; the cause control
and the manifest cause listing stay.

### Harness runs

`tools/emend-copy` gains two operations, given after the two paths
as the others are; neither takes a reference, name or number from
its command line. No reference, no name and no text from the
corpus is printed: the samples print counts and flags. A style
name is corpus content and is never printed; the samples count
names.

- `--namedstyles-sample <in> <out>` (P1): reads every named style
  (`named_style` of every `cellStyleXfs` index) and every
  differential style, then for each differential style whose
  fields are a font, a fill and a border alone (at least one of
  them `Some`), `add_differential_style` of the value read, the
  package written; reopens the output and reads
  `differential_styles` again, asserting the appended ones equal
  the values read from the input. Prints `namedstyles` with `named`
  (`cellStyle` children), `named_xfs` (`cellStyleXfs` children),
  `builtin` (with a `builtinId`), `custom` (`customBuiltin`),
  `resolved` (named styles resolved, equal to `named_xfs`,
  asserted), `default_font_size` (the default font's size, 0 for
  none; a size is not corpus content), `dxfs`, `dxf_fonts`,
  `dxf_fills`, `dxf_borders`, `dxf_numfmts`, `dxf_alignments`,
  `dxf_protections`, `dxf_gradients` (fills read as `None`),
  `dxf_written` (written back), `dxf_equal` (read back equal to
  the input's, equal to `dxf_written`, asserted). Expected results:
  `xl/styles.xml` `changed` when `dxf_written` is above 0 and
  `identical` otherwise, every other entry `identical`; `lo` and
  `lo_in` `ok`.
- `--colors-sample <in> <out>` (P2): writes the input back unedited
  and resolves every colour of every font, fill (both), border
  side and differential style read, plus `theme_colors`; prints
  `colors` with `colors` (colour values met), `rgb`, `theme`,
  `indexed`, `auto` (by form, summing to `colors`), `resolved`
  (`Some`), `unresolved` (`None`: `auto`, system indexes, a theme
  colour with no theme part), `tinted` (with a tint), `theme_part`
  (0 or 1), `scheme_dark_text` (fonts with theme 1 whose resolved
  colour has every channel below 128), `scheme_light_fill` (fill
  colours with theme 0 whose resolved colour has every channel at
  or above 128), `palette_override` (0 or 1, the part's
  `indexedColors`). Expected results: every entry `identical`,
  `files_identical` `yes`; `scheme_dark_text` equal to the count of
  fonts with theme 1 and `scheme_light_fill` to the count of fill
  colours with theme 0 in every workbook (the swap's proof; a
  workbook where either differs is a finding about the theme, the
  record naming it by id).

The run log's own check over the extracted input and output parts,
with `ElementTree` and byte spans and a control that reports a
planted difference per check, printing no name, no reference and
no text from the corpus:

- namedstyles, `xl/styles.xml`: the input's `dxfs` children a
  byte-identical prefix of the output's, and every other root child
  byte-identical; for each appended `dxf`, the input `dxf` it was
  read from compared child by child as the type models them (the
  unmodelled font children and alignment attributes, a gradient
  absent on the written side, and `patternType="solid"` on the
  written side against none on the read side, by the catalogue's
  rule); the line's counts against the check's independent counts
  of `cellStyle`, `cellStyleXfs` `xf`, `builtinId`, `customBuiltin`
  and `dxf` children by child kind.
- colors: every entry byte-identical; the line's counts by form
  against the check's independent counts of `rgb`, `theme`,
  `indexed`, `auto` and `tint` attributes over the elements the
  sample reads; the check resolves every theme and indexed colour
  itself (the scheme read with `ElementTree`, the default palette
  from the standard's table typed into the check, the same tint
  formula) and compares value by value with the tool's resolved
  list, the two lists hashed and the hashes printed, not the
  values; `theme_part` and `palette_override` against the parts
  present.

A departure from any expectation is a finding about the corpus or
the rule, recorded, and planning rules which. Every phase's run log
runs the edit open check on the host over that phase's keep, names
the phase directories it compares against by their record commit,
prints the devices of `TMPDIR` and of the corpus directory, and
counts the calc-chain workbooks with the rule covering them. The
Excel check shows nothing new for a read; the namedstyles sample
joins its list.

## Phases

As `specs/0001`: a doc commit first, code commits, one record
commit, the scan last, at most two independent checks per phase, a
report, STOP. One target root per phase,
`<home>/.cache/emend-build/m5p1-<phase>/target`. P1's gate prunes
Milestone 4 part 2's roots (`<home>/.cache/emend-build/m4p2-p1/
target`, `m4p2-p2/target` and `m4p2-close/target`, `specs/0031`
close-out ruling 9), each listed with `du -sb` and removed with
`rm -r`; the close-out's `gen.py` and logs stay. The three logged
scripts run in the phase directory with `scripts/counting.sh`
copied into `<scratch>` by `git show` (`specs/0031` P1
verification ruling 5); a dry run of each precedes its logged run.
Logs in `records/logs/m5p1-<phase>/` (`gate.log`, `test.log` on
both targets with the feature sets off, `read`, `read,rules` and
every feature, `negative.log`, `run.log`, `scan.log`, `size.log` in
P3); every rule the earlier parts added; the scan's denylist form
whole-word (`specs/0031` P1 ruling 6). No push during the part; the
close-out push follows P3 and carries the record commit b0f0b1f of
Milestone 4 part 2's close-out with this part's.

Part 1's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0031-release-metadata.md` as planning left them after
Milestone 4 part 2's close-out (the close-out rulings 1 to 10, the
Milestone 4 and 5 headings and rows), and this spec.

### P1: named styles and differential styles

The doc commit; then `cell_style_xfs`, `cell_styles`, `NamedStyle`,
`named_style`, `dxfs`, `differential_styles`, the shared resolver,
the `Pattern::Solid` rule for a `dxf` (decision 2), `default_font`,
and `--namedstyles-sample`. Tests on byte-built parts: a
`cellStyle` with every attribute and with the two required alone;
`named_style` of an index resolving to the same `CellStyle`
`cell_style` gives for a `cellXfs` `xf` with equal ids; the
refusals (no `name`, no `xfId`, an `xfId` past the list, an index
past the list); a `dxf` with each child kind alone and with all
six; a `dxf` `patternFill` with no `patternType` as `Solid` beside
a `cellXfs` fill's as `None`; a gradient `dxf` fill as `None`; an
`x14:dxfs` in `extLst` not counted; a round trip of a font, fill
and border `dxf` through `add_differential_style` on a byte-built
part, the appended element equal to the read one child by child
with the one named difference; `default_font` through the `Normal`
style, through font 0 when the `xf` has no `fontId`, and refused
with no `<fonts>`; a part with 440 `dxf` read (the corpus's
largest, byte-built); the tool's line. The gate as `specs/0027`
P1's (the crate built and tested with the feature sets above on
both targets, the `pub fn` lines listed off and on). The negative
log through the binary: the sample on a fixture whose `cellStyles`
names an `xfId` past `<cellStyleXfs>` (refused, with a control).
The run log: `--namedstyles-sample` on both targets over the
corpus, the checks as "Harness runs", the keep and the edit open
check, recorded as `records/m5p1-p1-<utc-date>-<shortsha>.md` and
`.txt`.

### P2: colours

The doc commit (the P1 verification rulings); the carry, if any;
then `indexed_colors`, the palette constant, `theme_colors`,
`ResolvedColor`, `resolve_color`, `tinted`, the two messages, and
`--colors-sample`. Tests on byte-built parts: the palette's
entries 0 to 15 and its length; an `indexedColors` override read
and consulted, and an index past it falling to the default; an
index 64 and 81 as `None`; a theme part with `srgbClr` and
`sysClr` children read in order, the swap (theme 0 giving `lt1`,
1 `dk1`, 2 `lt2`, 3 `dk2`, 4 `accent1`, 11 `folHlink`), the strict
namespace accepted, no theme relationship giving `None`; the
refusals (an index past 11, a tint of 1.5 and of NaN, a `val` of
five digits, a `sysClr` with no `lastClr`, two `clrScheme`); the
tint formula on a known case each way (a tint of 0.5 on black and
of -0.5 on white, the values from the formula) and no tint giving
`rgb`; `Auto` as `None`; the tool's line. The gate as P1's. The
negative log through the binary: the sample on a fixture whose
theme part's `clrScheme` holds a `sysClr` with no `lastClr`
(refused, with a control). The run log: `--colors-sample` on both
targets over the corpus, the checks as "Harness runs", the keep
and the edit open check, recorded as
`records/m5p1-p2-<utc-date>-<shortsha>.md` and `.txt`.

### P3: the catalogue and limits, the probe, the size record

The doc commit (the P2 verification rulings); the carry, if any;
then the `package` header's catalogue and limits for this part
(the messages by feature; what the typed view drops; the `dxf`
pattern rule; the system indexes and `Auto` as `None`; the theme's
hold stated); the `styles` module header's sentence on what is not
modelled corrected; the probe grown as "Size" says; the size log,
recorded as `records/size-m5p1-<utc-date>-<shortsha>.md`; the API
listing rerun as Milestone 4 part 1 P3 did, recorded as
`records/api-review-m5p1-<utc-date>-<shortsha>.md` (the new items
listed, the count stated); `CHANGELOG.md`'s "Added" gains one
bullet citing this spec, under `specs/0031`'s citation check; the
negative log through the binary for every message of the part
reachable there, the unreachable ones stated with the test that
covers each; both samples again on both targets with the size
log's release binaries, byte-identical to P1's and P2's runs by
sha256, recorded as `records/m5p1-p3-<sample>-<utc-date>-
<shortsha>.md` and `.txt`. The open record of the part's keep.

### Close-out push (after P3)

As Milestone 4 part 2's (`specs/0031` "Close-out push (after P2)"
and its close-out rulings): the generator transforms part 2's
scripts (`gen.py` f60d8d69 and the three it printed, carried
cmp-equal), its numbered changes continuing from 28; the doc commit
holds this spec and `ROADMAP.md`; the base is Milestone 4 part 2's
close-out doc commit ac5dc0d, named by its subject ("M4 part 2
close-out: commit planning docs (the P2 verification and rulings,
decision 1 hold)"); `PREV_RECORD_PATTERN` is `^M4 part 2 close-out
record` (b0f0b1f), in the range exactly once; the close-out doc
commit's subject is "M5 part 1 close-out: commit planning docs
(the P3 verification and rulings)"; `CARGO_TARGET_DIR` printed and
checked as `<home>/.cache/emend-build/m5p1-close/target`; no
Excel-record check, the stray-record count 0 kept; the scan's
denylist form whole-word (change 27, kept); the record commit "M5
part 1 close-out record: the push, the CI run and the scan"
holding the three logs only; pre mode on a clean tree; one
independent check before the report. The push carries b0f0b1f and
this part's commits, on planning's word and the maintainer's
authorisation typed in the implementing session.

## Planning rulings during P1 (2026-10-11)

Raised by the implementing session after the doc commit cd30099
(the three files at 2f6fa9be, 7758479a and 9f9c12c9, checked by
planning), before any code commit; its survey was an unlogged
aside, so planning recounted every figure with a namespace-aware
parser over the 27 workbooks' styles parts (counts only) and the
figures below are planning's.

1. A `dxf`'s `<border>` may carry `horizontal` and `vertical`
   children, the inside-of-range sides no writer of this crate
   writes (88 borders in 9 workbooks); `read_border` refuses any
   child but the five it models, so the typed read would refuse
   those nine parts whole and break `cell_style` there. Ruling: in
   a `dxf`, `horizontal` and `vertical` are read past with their
   subtrees, as the five unmodelled font children are, and the
   border's `outline` attribute is read past (`specs/0027` P1
   ruling 3); the catalogue, the run log's check and the
   round-trip comparison list the two sides as unmodelled, absent
   on the written side. `<borders>` is unchanged: a `horizontal` or
   `vertical` child there keeps being refused (the corpus has
   none). Tests: a `dxf` border with both sides read past, and a
   `<borders>` border with one still refused.
2. The spec's `dxf` figures were planning's prefix-tolerant
   regular-expression counts, which took the 31 `dxf` children of
   the four `x14:dxfs` with the main namespace's: 1391, 992, 486,
   6 gradients and 289 against the main namespace's 1360, 964, 459,
   0 and 260 (the alignment, number format and protection counts
   were right). The facts bullet is corrected in place. On the
   corpus `dxf_gradients` is expected 0 and the gradient case is
   proven by P1's byte-built test alone; a run reading above 0 is a
   finding. The sample line gains `dxf_implied_solid` (fills read
   as `Solid` from an absent `patternType`, decision 2; 179 over
   the corpus) and `dxf_inside_sides` (`horizontal` and `vertical`
   sides read past, 164 over the corpus), and the run log's check
   counts both independently per workbook.
3. A second main-namespace `<cellStyleXfs>`, `<cellStyles>` or
   `<dxfs>` among the root's children, with the feature `read`:
   refused at the read, "a style cannot be read" (the indexes
   would be ambiguous), the catalogue saying so; without the
   feature, walked past as today. `TWO_CONTAINERS`'s text is not
   widened: a catalogued message does not change for a case the
   feature adds. A test for each of the three.
4. `Styles` derives `Eq` and `Style` is `PartialEq` alone (its
   `f64`s), so a public `dxfs: Vec<Style>` field would drop the
   derive, a signature change. Ruling: `dxfs` lives in the typed
   read (`read::Typed`), private, compared as the typed fonts are,
   and `Package::differential_styles` lends it as `&[Style]`;
   `cell_style_xfs: Vec<Xf>` and `cell_styles: Vec<NamedStyle>` are
   public fields of `Styles` under the feature, both `Eq`. "Styles
   gains `dxfs`" under "Differential styles" reads so.
5. Accepted as proposed: a `dxf` with an `extLst` child is refused
   as a child unknown to the type (the corpus has none); a `dxf`
   `numFmt` without `formatCode` is refused, "a style cannot be
   read"; `default_font` with an empty `<cellStyleXfs>` reads font
   0; a `fontId` past `<fonts>` at `default_font` is refused, "a
   style cannot be read". Each with a test.

## Planning verification and rulings on P1 (2026-10-11)

Verified from primaries: the three commits cd30099 (docs), 0b3e46f
(crate) and c8b9391 (tool, the phase commit) read in full against
this spec and rulings 1 to 5; the record commit 06f5cba holds the
five logs, the independent check and the four records at the
hashes and line counts the report gives (gate 525b6d00, test
337b1036, negative 85af306b, run 18ffd296, scan c3309174, check
d685000e; records b2d1a74b, 5d789a23, ed5da088, 5a498705). The
gate prunes Milestone 4 part 2's three roots (du -sb then rm -r,
gone and kept names checked) and carries the debug binaries; the
test log passes 514, 562, 600 and 934 on both targets (off and
all equal M4 part 2 P2's 514 and 918 plus the 16 namedstyles
tests), the namedstyles tests run being exactly the ones each set
compiles, and plants P1 to P8 each fail exactly their set; the
negative log shows N1 to N6 each exit 2 with "a style cannot be
read" and no output, B0 accepted; the run log shows 27 of 27 ok on
both targets, entries identical 854, changed 16 (the styles part
where a dxf is written back), flagged 6 descriptors, nothing
added, dropped or recompressed, the outputs and lines the same on
both targets, the totals equal to ruling 2's 14 figures,
dxf_written and dxf_equal 831, the keep 1097 pairs (4 new, 11
changed by the dxf.rs fixture edit, none gone), no verdict of the
named record moved and no new output refused on an accepted
input; the scan PASS with 0 hits and 0 structural failures.
Planning's own run: the phase commit built in a detached
worktree, crate tests on both targets with the four feature sets
514, 562, 600, 934 (1 ignored each), workspace tests 1131 on both,
clippy and rustdoc 0 warning lines (all features, and read
alone), release emend-copy on both targets, the i686 binary at
the record's hash 0a13f5ce; `--namedstyles-sample` over the 27
corpus files on both targets, every output and every line
byte-identical to the phase's run directory, 0 refused, planning's
own totals the 14 figures above; planning's privacy scan over
b0f0b1f..06f5cba: 4 commits, 1582 blobs, 0 findings.

Rulings on the independent check's points and the disclosures:

1. Ruling 1's catalogue wording (check point a). The catalogue
   lists the inside sides under "What cannot be written" already,
   and the dxf read-past stands in `differential_styles`'s doc.
   Ruling 1 is met for P1; P3's catalogue work adds, to the typed
   read's entry, that in a `dxf` the `vertical` and `horizontal`
   sides and the border's `outline` are read past and a `dxf`
   `numFmt`'s `numFmtId` is not read. No P1 fix.
2. A `dxf` `numFmt`'s `numFmtId` is not parsed (point b). Accepted:
   `Style` holds the code alone and the writer assigns its own id;
   "a value that does not parse" covers the values the type holds.
   Catalogued at P3 (ruling 1).
3. Ruling 3's "without the feature, walked past" has no test
   (point c). P2's code commit adds one test to
   `crates/emend/src/styles/tests.rs` (compiled with the features
   off): a part with a second `cellStyleXfs`, `cellStyles` and
   `dxfs` reads with the counts unchanged. The off figure grows by
   1 in P2's test log; the test log says so.
4. The two over-long comment lines (point e: the catalogue line
   in `package.rs` and the fixture doc in `null_rewrite.rs`) are
   wrapped in P2's code commit, text unchanged; `let _ = named;`
   stands.
5. Disclosure 2 accepted: under `read`, a `dxf` the typed read
   refuses makes the styles part unreadable, `cell_style` included,
   as "refused at the read" says and as a refused font already
   does; the two fixture edits (dxf.rs's BASE gaining the
   `cellStyleXfs` a `cellStyle` names; readback.rs's dxf font
   child) and the 11 changed keep pairs they explain are accepted.
6. The report text's corrections stand as corrected: the negative
   log's one shared control B0 (each case is B0 with one
   replacement, so one control serves); the four ": no" input
   lines; the totals labelled as FACTS.md's figures (they are
   ruling 2's, carried there). Accepted. A scan pre-check that is
   rerun sets RERUN_NOTE from the next phase on, pre-check or not.
7. The superseded local commits 8eeebc4 and e1cfb6b stay
   unreferenced; the dry-run fixes through the generators, scan
   attempt 1 and the fork's temporary file are disclosed and need
   nothing. The phase's target root `m5p1-p1/target` stays; the
   part's roots are pruned at the next part's first gate that
   builds, as Milestone 4 part 2's were.
8. P1 is DONE. P2's doc commit carries this spec as amended
   (rulings 1 to 5 and this section) and ROADMAP.md with the P1
   row DONE and the P2 row IN PROGRESS.

## Planning rulings during P2 (2026-10-11)

Raised by the implementing session after the doc commit 76f489f
(this spec at 01c1d194 and ROADMAP.md at 50e2bd01, checked by
planning), before any code commit. Planning recounted every
figure below with a namespace-aware parser over the 27 workbooks
(counts only) and checked `color_from`'s precedence and rgb form
in the source.

1. `<colors>` in the styles part, with the feature: "a style
   cannot be read" for a second main-namespace `<colors>` or a
   second `<indexedColors>` in it (as P1 ruling 3), an `rgbColor`
   with no `rgb` or an `rgb` that `color_from` would refuse (not
   eight hex digits), and a main-namespace child of
   `<indexedColors>` other than `rgbColor`. `<mruColors>` and
   anything outside the main namespace are walked past; without
   the feature everything is walked past as today. Corpus: 23
   `<colors>`, 18 `indexedColors` (1016 `rgbColor`, every one
   eight hex digits), 5 `mruColors`. A test for each refusal and
   one for `mruColors` walked past.
2. The scheme read is `a:theme/a:themeElements/a:clrScheme` alone;
   a second one at that path is refused, a `clrScheme` under
   `extraClrSchemeLst` is walked past, and a theme with none at
   the path is refused, each "a theme cannot be read". "Two
   `clrScheme` elements" under "Errors" reads so. Corpus: 27
   themes, one `clrScheme` each at the path and none elsewhere.
3. The scheme's children, "a theme cannot be read" for: one of the
   twelve missing or met twice; a DrawingML child the type does
   not know (`a:extLst` walked past); a colour element holding
   children (a transform such as `a:lumMod` would make `val` not
   the colour); a scheme child holding a colour form other than
   `srgbClr` or `sysClr`. Hex in either case. Corpus: 324 scheme
   children, each exactly one colour element with no children,
   318 `srgbClr` and 6 `sysClr`.
4. `tinted` computes in `f64`: RGB to HLS with HLSMAX 255, the
   tint applied to L as 18.8.19 says, back to RGB, each channel
   rounded to nearest with halves away from zero (`f64::round`),
   the alpha byte kept. The P2 tests' known cases read: tint 0.5
   on `FF000000` gives `FF808080` (L 127.5 rounds to 128), tint
   -0.5 on `FFFFFFFF` gives `FF808080`.
5. The swap's proof (`scheme_dark_text`, `scheme_light_fill`)
   tests `ResolvedColor::rgb`, the scheme colour before the tint
   (a tinted theme-1 font may be light). "Fonts" means every font
   colour read, cell fonts and `dxf` fonts both; "fill colours"
   means `fgColor` and `bgColor` both, cell fills and `dxf` fills.
6. A tint on a non-theme colour: `color_from` keeps a tint on
   `Theme` alone (the corpus has 1 indexed colour with a tint,
   which the type drops). `tinted` counts the resolved colours
   carrying a tint, theme only; the run log's check counts `tint`
   attributes on theme-form colours against `tinted` and prints
   the tints on other forms (expected 1) as a count beside it,
   not a failure. The check's counts by form follow
   `color_from`'s precedence (`auto`, `rgb`, `theme`, `indexed`)
   over font `color`, `patternFill` `fgColor` and `bgColor`, and
   border `left`, `right`, `top`, `bottom` and `diagonal` `color`
   in `<fonts>`, `<fills>`, `<borders>` and the main `<dxfs>`; not
   a `dxf` border's `vertical` or `horizontal` side (read past, P1
   ruling 1) and not gradient stops.
7. Code: `package.rs`'s `DRAWINGML` constant is gated
   `any(images, read)`, nothing else changed in it; the theme
   cache is a `cfg(read)` field `Option<Option<[u32; 12]>>` on
   `Package` and on its `Parts` view, read once.
8. The default palette's alpha byte. ECMA-376 18.8.27 gives each
   entry as `00RRGGBB`; the constant holds the RGB transcribed
   from the table with the top byte `FF` (`FF000000`, `FFFFFFFF`,
   ...), so a resolved indexed colour is the opaque colour a
   writer would write, as `theme_colors` gives and as `Color::Rgb`
   documents; its doc comment says so. The palette test asserts
   the `FF` form; the run log's typed-in palette holds the same.
   An `indexedColors` override keeps its `rgb` as written, with
   whatever alpha it carries.
9. "Resolves every colour of every font, fill (both), border side
   and differential style read" under "Harness runs" assumed the
   tool could list the typed fonts, fills and borders, which live
   in the crate-private `read::Typed` and are reached only through
   a resolved format (the corpus has 54 fonts and 27 fills no
   format names, and 1451 format references). Ruling (option B):
   `Styles` gains three read-only accessors with the feature, each
   lending its list in part order: `typed_fonts(&self) ->
   &[Font]`, `typed_fills(&self) -> &[Option<Fill>]` (a gradient
   `None`, as the read gives it) and `typed_borders(&self) ->
   &[Border]`; the count accessors `fonts`, `fills` and `borders`
   are unchanged. The sample reads each element once through
   them, plus every differential style; the run log's check counts
   elements directly; P3's API listing grows by three, noted in
   its record. Nothing existing changes. A test that each accessor
   lends the part's elements in order, with a gradient `None`.
10. The resolved list's comparison. "Hashed and printed, not the
   values" and "the samples print counts and flags" are met
   together by option A: the tool prints no value. The list, in a
   fixed order: each typed font's colour in part order; each typed
   fill's `fgColor` then `bgColor`; each typed border's `left`,
   `right`, `top`, `bottom`, `diagonal` colour; then each `dxf` in
   part order, its font colour, fill `fgColor` and `bgColor`, and
   border sides in the same order. Each present colour gives one
   entry, `rgb:tinted` as eight plus eight lower-case hex digits
   or `none` for an unresolved one; absent colours give nothing.
   The tool digests the entries with 64-bit FNV-1a (its own ten
   lines, no dependency; the algorithm's public constants), each
   entry's bytes followed by a newline, and prints four fields in
   its line, `digest_fonts`, `digest_fills`, `digest_borders` and
   `digest_dxfs`, each 16 lower-case hex digits, so a mismatch is
   located to a group. The check builds the same four lists
   independently, digests them the same way, and compares; the
   log prints the eight digests per workbook, never a value. A
   mismatch is a finding examined by hand outside the log. The
   tool's test asserts the digest of a known small list against
   a value computed by hand in the test.

## Planning verification and rulings on P2 (2026-10-12)

Verified from primaries: the three commits 76f489f (docs), 85bb33f
(crate) and 70b3e00 (tool, the phase commit) read in full against
"Colours", "Errors", "P2", P1 verification rulings 3 and 4 and P2
rulings 1 to 10; the record commit e39d028 holds the five logs,
the independent check and the four records at the hashes and line
counts the report gives (gate 8eded48f, test ba4b6db5, negative
6fb87668, run e81d9d30, scan 3e78407b, check 5496bed3; records
c89d8e9b, 8453c8d0, bb0fdf20, 1ca0a182). The gate builds under
one root with no prune; the test log passes 515, 584, 622 and 956
on both targets (P1's figures plus the 22 colors tests each set
compiles and the carry test under off), ten plants each failing
their set; the negative log shows N1 to N3 exit 2 with "a theme
cannot be read", N4 with "a colour cannot be resolved", N5 and N6
with "a style cannot be read", no output each, B0 accepted with
the tool test's line; the run log shows 27 of 27 ok on both
targets, 870 entries identical, nothing changed, added, dropped,
recompressed or flagged, files_identical yes 27, the outputs and
lines the same on both targets, the comparison (its own palette,
colorsys and FNV-1a) agreeing with the tool's counts and four
digests 27 of 27, the swap's proof 27 of 27 (scheme_dark_text 498
of 498 theme-1 font colours, scheme_light_fill 32 of 32 theme-0
fill colours), tints on other forms 1, the keep 1099 pairs (2 new,
none changed or gone), no verdict of the named record moved; the
scan PASS with 0 hits and 0 structural failures. The totals over
the corpus, recorded here: colors 2511, rgb 390, theme 1606,
indexed 326, auto 189, resolved 1998, unresolved 513, tinted 985,
theme_part 27, scheme_dark_text 498, scheme_light_fill 32,
palette_override 18.
Planning's own run: the phase commit built in a detached worktree,
crate tests on both targets with the four feature sets 515, 584,
622, 956 (1 ignored each), workspace tests 1156 on both, clippy
and rustdoc 0 warning lines (all features, and read alone),
release emend-copy on both targets, the i686 binary at the
record's hash 76b966da; `--colors-sample` over the 27 corpus files
on both targets, every output and every line byte-identical to
the phase's run directory, 0 refused; planning's own resolution
check (its own transcription of the palette, Python's colorsys,
its own FNV-1a, written without reading the phase's comparison
first) gives the tool's line, counts and four digests, in 27 of 27
workbooks and the 12 totals above; the seven digests the tool
tests assert recomputed by planning; planning's privacy scan over
06f5cba..e39d028: 4 commits, 1594 blobs, 0 findings.

Rulings on the independent check's points and the disclosures:

1. The `package.rs` catalogue carry (check point a). The sentence
   had to name ruling 1's refusals, so "text unchanged" could not
   hold for it; the rewording is accepted as the carry of P1
   verification ruling 4 and of ruling 1 together. The short line
   it leaves ("ruling 1). The") is reflowed by P3's catalogue
   work, which rewrites that paragraph; no P2 fix.
2. An `indexedColors` override longer than 64 entries (point b).
   `resolve` consults the override wherever the index is within
   it, so an override of 65 entries resolves index 64 to its entry
   and 65 to `None`. Ruling: that reading of "when it has one and
   `i` is within it" stands; Excel writes exactly 64, and a longer
   override is the part's own statement. P3's catalogue says so
   under the limits ("`None` for an index past the override and
   past 63"), `resolve_color`'s doc gains the clause, and P3's
   carry adds one test to `package/tests/colors.rs`: an override
   of 65 entries resolves 64 to its entry and 65 to `None`. The
   read, read,rules and all figures grow by 1 in P3's test log.
3. The digests "computed by hand in the test" (point c). Ruling 10
   meant a value not produced by the code under test; constants
   worked out by hand, commented so, and recomputed by the checker
   and by planning (all seven agree) meet it. No change.
4. The two messages catalogued in P2 (point d): as "Errors" says
   and as P1 did. Accepted; P3's catalogue work edits the entries
   as it needs.
5. Disclosures accepted: the message-only amend before any logged
   run (8469e5a unreferenced); the dropped binary tint case (the
   styles read refuses the tint first, and the crate test
   `a_theme_index_past_11_or_a_tint_out_of_range_or_not_finite_is_refused`
   covers `resolve_color`); the dry-run fixes O48 and MI through
   the generators; the development builds in the repository's
   ignored `target/` (the logged gate built under its own root
   alone); the report's two wording corrections, applied.
6. The `git stash --keep-index` that took planning's working spec
   (disclosure 5): restored with the hash unchanged, no harm done;
   the rule stands for both sessions: `git stash` is never run in
   the main tree while the working spec is uncommitted. The two
   hand `rm -rf` of `gen/__pycache__` (disclosure 7): against the
   rule, accepted this once; `PYTHONDONTWRITEBYTECODE=1` for every
   generator run from P3 on, the log printing that it is set.
7. The superseded commit stays unreferenced. The phase's target
   root `m5p1-p2/target` stays beside `m5p1-p1/target`; both are
   pruned at the next part's first gate that builds.
8. P2 is DONE. P3's doc commit carries this spec as amended (P2
   rulings 1 to 10 and this section) and ROADMAP.md with the P2
   row DONE and the P3 row IN PROGRESS. P3's carry: ruling 2's
   test and doc clause; P1 verification rulings 1 and 2's
   catalogue wording; ruling 1's reflow; the API listing names
   ruling 9's three accessors with `theme_colors`, `resolve_color`,
   `ResolvedColor` and `tinted`, the count stated.

## Decisions for the maintainer

Decided by the maintainer 2026-10-11: 1, 2 and 3 as recommended (A
in each), typed in the planning session.

1. Packaging: A (recommended), everything under `read`, the read
   probe grown, 28 probes; B, a feature `theme` for the resolution
   and a 29th probe.
2. A `dxf`'s `patternFill` with no `patternType`: A (recommended),
   read as `Pattern::Solid`, Excel's rendering, so a differential
   style read is written back as one Excel shows the same, the
   written `patternType="solid"` the one named difference in the
   bytes; B, read as `Pattern::None` as a cell format's is, the
   round trip then writing a fill Excel shows as no fill, which
   the sample would record as a finding on every such `dxf`.
3. Colours the file defines no RGB for (`Auto`, the system indexes
   64 to 81, a theme colour with no theme part): A (recommended),
   `resolve_color` gives `None`, a refusal kept for a theme index
   past 11 and a tint out of range; B, every such colour refused,
   so a caller resolving a whole workbook's colours fails on the
   first `auto`, which the corpus has 194 of.

## Out of scope

- Writing anything new: a named style added, a `dxf` edited, a
  palette or theme changed; a change to any writer.
- The theme's font scheme (`a:fontScheme`, the `minor` and
  `major` fonts a `scheme` attribute on a font refers to), the
  theme's format scheme, and any theme element but `clrScheme`.
- `tableStyles`, `mruColors`, `extLst` in the styles part;
  `x14:dxfs`.
- Luminance arithmetic beyond the standard's tint formula; any
  colour space conversion for display.
- Reading cells, settings, validations, conditional formats,
  tables, comments and images: their reads exist (`specs/0027`
  and the parts that added each).
- The workbook survey, pivot tables and external links (part 2);
  charts (parts 3 to 5); the openpyxl comparison (part 6).
- The Milestone 2 Excel check (gates no push in this milestone).
