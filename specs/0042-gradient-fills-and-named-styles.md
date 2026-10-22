# specs/0042: gradient fills and named styles (Milestone 6 part 5)

Status: APPROVED 2026-10-22 (Milestone 6 part 5; decisions 1 to 4 A,
typed in the planning session 2026-10-22: the gradient a field on
`Fill`; `Style::named`; over the ceiling, a finding then a feature;
names equal ignoring ASCII case). Planning session. Builds on
`specs/0017` (fuller styles: the composers, the five-container
rewrite, the messages), `specs/0027` (the typed read) and
`specs/0032` (named styles and differential styles read); what they
name stands. The part is proven on built packages, on the corpus run
of two new samples, on two generated fixtures and by the LibreOffice
open check, as part 4 was; the Milestone 2 Excel check gates no push.

## Goal

The two styles items of Milestone 6's list (`ROADMAP.md`, "Part 5:
gradient fills and named styles"; `specs/0017` "what can be added"
put gradient fills out of scope at 0 in the corpus of the day, and
`specs/0032` reads `cellStyleXfs` and `cellStyles` without writing
them): a cell format takes a gradient fill, written as the schema's
`gradientFill`, and the typed read reads one back instead of reading
it as no fill; a named style is created (`add_named_style`), writing
a `cellStyleXfs` format and a `cellStyles` entry, and a cell format
names it through `xfId`. openpyxl's `GradientFill`, `NamedStyle`,
`Workbook.add_named_style` and `cell.style = name`. Both go in the
core (Milestone 6 decision 2 A), within the Milestone 2 ceiling.

## Size and packaging

The budget figure after part 4 is 438960 section bytes against the
ceiling of 458752 (`records/size-m6p4-2026-10-22-1cc0de4.md`): 19792
of headroom, all of it this part's to spend. Everything here is core,
measured by the budget probe `probe-emend-format-write-xml` over
`probe-zip-xml`; `probe-emend-fullstyles-write-xml`, which it builds
on, gains the gradient fill and the named style ("The probe"), so the
budget's movement over part 4's record is this part's cost. The
`read` feature's cost (112552 after part 4) moves by the gradient's
typed read, outside the budget, printed and judged under its own name
(expected under 3000).

Planning's estimate, from measured precedent: `specs/0017` added five
composers, three containers to the styles rewrite and two refusals
for a whole growth of 23192 (P3 record 1fc8327), about 3000 per
composer with its container. The gradient composer with its checks,
one composer and no container: 2000 to 4000. The named style: two
containers in the rewrite (`cellStyleXfs` and `cellStyles`, grown as
`fonts` is, never created), the core read of both (the existing `xf`
read reused for `cellStyleXfs`, one attribute read per `cellStyle`),
the name checks, the `named` arm of the derived format and the call:
6000 to 11000. In all 8000 to 15000. Planning is told the figure in
the report at any value; a cost above 15000 is a finding the report
names; a budget above 458752 is a finding that stops the part at P3
for decision 3. `images`, `comments` and `charts` costs may move
(expected 0, accepted up to 1500 each), the size log judging the
movements under their own names (`specs/0038` ruling N).

## Facts planning checked (2026-10-22)

- The corpus (27 workbooks, counted by planning's script over
  `xl/styles.xml`, never committed; counts only): every part has
  `cellStyleXfs` and `cellStyles`; 459 `cellStyleXfs` formats and
  221 `cellStyle` entries, at most 25 per part, 26 parts with more
  than one; every part has a `builtinId="0"` entry (Normal); 185
  entries carry `builtinId`, 36 are custom (no `builtinId`). 6
  `gradientFill` elements in 2 workbooks, every one linear with a
  `degree` and two `stop` children (12 stops), none `type="path"`.
- The crate today: `Fill { pattern, foreground, background }` is
  `Copy`; `Fill::compose` writes `patternFill` alone
  (`crates/emend/src/styles/elements.rs`); the typed read skips a
  `gradientFill` with its subtree and gives `None`
  (`crates/emend/src/styles/read.rs`, `read_fill`), in a `dxf` too;
  the core read walks `cellStyleXfs` and `cellStyles` past, the
  `read` feature reads them into `cell_style_xfs: Vec<Xf>` and
  `cell_styles: Vec<NamedStyle>`; `derived_xf` writes `xfId="0"` on a
  format over no base and keeps the base's `xfId` otherwise;
  `rewrite_styles` grows five containers (`numFmts` created when
  absent, `fonts`, `fills`, `borders`, `cellXfs`) and, under `rules`,
  `dxfs`.
- openpyxl 3.1.5 (the pinned image, to be confirmed on its fixture in
  P2): `GradientFill(type="linear"|"path", degree, left, right, top,
  bottom, stop=[colours])` writes `<fill><gradientFill
  degree="90"><stop position="0"><color rgb=.../></stop>...`;
  `NamedStyle(name, font, fill, border, alignment, number_format,
  protection)` with `add_named_style` writes one `cellStyleXfs` `xf`
  (ids, no `xfId`), one `cellStyle` with `name` and `xfId` and no
  `builtinId`, and `cell.style = name` a `cellXfs` `xf` copying the
  named format's ids with `xfId` set.
- The messages in force: "a style addition needs a collection the
  styles part lacks", "a style field is out of range", "a style index
  is out of range", "a font name is empty or holds a character XML
  cannot carry", "a style cannot be read" (`read`).

## Design

### Crate

Core, in `styles` and the styles rewrite of `package`; the typed read
under `read` for the gradient. No new dependency, no new feature
(decision 3 names the one case a feature would appear).

### Types

- `Gradient { kind: GradientKind, stops: Vec<Stop> }`, `Clone`,
  `Debug`, `PartialEq`. `GradientKind::Linear { degree: f64 }` and
  `GradientKind::Path { left: f64, right: f64, top: f64, bottom: f64
  }`, `Clone`, `Copy`, `Debug`, `PartialEq`; `Stop { position: f64,
  color: Color }`, `Clone`, `Copy`, `Debug`, `PartialEq`. `degree` is
  finite and in `0.0..=360.0`; each of the four is finite and in
  `0.0..=1.0`; a position is finite and in `0.0..=1.0`, the stops'
  positions non-decreasing; at least two stops; a stop's colour as a
  fill colour is checked. Else "a style field is out of range".
- `Fill` gains `gradient: Option<Gradient>` and loses `Copy`
  (decision 1 A). A fill with `Some` gradient must have `pattern`
  `Pattern::None` and both colours `None`, since the schema gives a
  fill one child; else "a style field is out of range". A pattern
  fill is unchanged. Under decision 1 B (not recommended) `Fill`
  becomes an enum and every use changes.
- `Style` gains `named: Option<u32>` (decision 2 A): the named style
  the new cell format derives from, by `cellStyleXfs` index, one the
  part has or one `add_named_style` returned. `Default` `None`.
- `Package::add_named_style(&mut self, name: &str, style: &Style) ->
  Result<u32, Error>`: records a named style and returns the
  `cellStyleXfs` index it will have, the part's count plus the number
  recorded earlier by this package (no deduplication of formats; a
  name is checked, "Errors").
- `Styles` gains, in the core, `cell_style_xfs` as the `read`
  feature has it today (the `xf` children of `<cellStyleXfs>`, read by
  the existing `xf` read, with their bytes) and the count of
  `<cellStyles>`'s `cellStyle` children with each child's `name`
  (names kept privately for the duplicate check; `cell_styles:
  Vec<NamedStyle>` stays `read`'s). The two containers join the
  two-containers rule (a second main-namespace one refuses the part
  with the existing message).

### The styles part

- A gradient fill: `<fill><gradientFill ATTRS><stop position="P">
  COLOR</stop>...</gradientFill></fill>`, ATTRS `degree="D"` for a
  linear gradient whose degree is not 0 (none for 0, the schema's
  default, Excel's form), `type="path" left="L" right="R" top="T"
  bottom="B"` for a path gradient, the four always written; each
  number in the decimal form the writer uses for a tint; `COLOR` the
  `<color>` element as `fgColor` is written. Appended to `<fills>` as
  a pattern fill is, one element per distinct gradient.
- The typed read (`read`): a `gradientFill` reads as `Some(Fill {
  pattern: Pattern::None, foreground: None, background: None,
  gradient: Some(..) })`, `type` absent or `linear` as `Linear` with
  `degree` absent read as 0, `path` with each of the four absent read
  as 0, each `stop`'s `position` and its `color` child read as a
  colour is; an unreadable number, a stop without a colour or a
  second colour refuses with "a style cannot be read". The same in a
  `dxf`, so `add_differential_style` (`rules`) writes a gradient
  back through the same composer; the sample's `dxf_gradients` count
  becomes "read typed", its name kept.
- A named style: `<xf numFmtId="N" fontId="F" fillId="I" borderId="B"
  APPLY>CONTENT</xf>` appended to `<cellStyleXfs>`, the given ids or
  0, no `xfId`, the apply flag of each thing given, the `alignment`
  and `protection` given as its content (the no-base form of
  `derived_xf` less `xfId`); `<cellStyle name="NAME" xfId="K"/>`
  appended to `<cellStyles>`, `NAME` escaped as an attribute, `K` the
  returned index; both `count` attributes updated as `fonts`'s is. A
  part without either container refuses with the collection message
  (every corpus part has both; nothing is created). `builtinId`,
  `customBuiltin`, `hidden` and `iLevel` are not written.
- A cell format with `named: Some(k)`: over no base, the named
  format's `numFmtId`, `fontId`, `fillId` and `borderId` for each the
  style leaves `None`, its `alignment` and `protection` as the base's
  would be, `xfId="k"`, the apply flags per `Style` field (openpyxl's
  `cell.style` form); over a base, the base's copy with `xfId`
  replaced by `k` (added where absent), the rest as today. `k` past
  the part's count plus the ones recorded refuses with "a style index
  is out of range". `named: None` is today's behaviour exactly.
- Order of checks at the call: `add_style`'s existing order, then
  `named`, then the fill's gradient fields in declaration order;
  `add_named_style`: the name, then the style as `add_style` checks
  one over no base (a `named` on a named style refuses with "a style
  field is out of range").

### Errors

One new message, "a style name is empty, too long or taken": empty,
more than 255 characters, holding a character XML cannot carry, or
equal ignoring ASCII case to a name the part holds or this package
recorded (decision 4 A; Excel repairs a part with two equal names).
The gradient's ranges and the one-child rule use "a style field is
out of range"; `named` uses "a style index is out of range"; the
containers use the collection message. The catalogue in the
`package` header gains the entry and the limits.

### Harness runs

`--gradient-sample` (P1): two cell formats are added through
`add_style` over no base, one with a linear gradient of two stops at
90 degrees in two `Rgb` colours, one with a path gradient at the
centre of two stops, and two new cells one row past the last take
`Number(7)` and `Number(8)` with them; the output is reopened and,
with `read`, both cell formats' fills are read back. One line: the
word `edit` then `added`, `xfs_before`, `xfs_after`, `fills_added`,
`cells_before`, `cells_after`, `read_back` (fills read with a
gradient, 2) and `gradients_before` (the part's own `gradientFill`
elements counted from its bytes beside the typed read). COUNTS ONLY:
the colours and numbers are the tool's constants. `xfs_after`,
`cells_after` and `read_back` are asserted as `--style-sample`'s are.
Expected on the corpus: `gradients_before` 6 over the run, the styles
part `changed`, `lo` and `lo_in` `ok`; the open check of the keep.

`--namedstyle-sample` (P2): `add_named_style` of the tool's constant
name with a bold font and a solid fill; a cell format over no base
with `named` that index and the code `0.000`; a second over the first
sampled cell's own format with `named` the same index; the cells are
`--gradient-sample`'s two; the output reopened and, with `read`,
`named_style` of the new index read back equal to what was set and
both cell formats' `xf_id` equal to it. One line: the word
`namedstyle` then `named_before`, `named_after`, `named_xfs_before`,
`named_xfs_after`, `xfs_before`, `xfs_after`, `cells_before`,
`cells_after`, `read_back` (3: the named style, the two `xf_id`s)
and `custom_before` (entries without `builtinId`). COUNTS ONLY: the
name is the tool's constant and no name of the part is printed.
Asserted as above. Expected on the corpus: `named_after` one more
than `named_before` on every workbook, `custom_before` 36 over the
run, `lo` and `lo_in` `ok`; the open check of the keep.

Then the two generated fixtures, each under the phase directory, never
committed, read on both targets, byte-identical across them:

- Excel's form (P1), written by the run log's own generator with
  `zipfile`: one worksheet of four cells, a styles part in the
  corpus's shape (counts only) with a linear `gradientFill` of two
  stops with `degree`, a path `gradientFill` of two stops, three
  `cellStyleXfs` formats, three `cellStyle` entries (Normal with
  `builtinId="0"`, one with another `builtinId`, one custom) and a
  cell format with `xfId="2"`. `--namedstyles-sample` on it reads the
  three; `--gradient-sample` adds two fills beside the two and reads
  both back (`read_back` 2, `gradients_before` 2);
  `--namedstyle-sample` adds a fourth named style and the run log's
  own check with `ElementTree` confirms the three kept entries and
  formats byte for byte, the new `xf` without `xfId`, the new
  `cellStyle` with `name` and `xfId="3"`, both `count`s, and the new
  cell formats' `xfId="3"`.
- openpyxl's form (P2), written by the pinned image through
  `scripts/openpyxl-ops.py named-style <in> <out>` (the op grows the
  script and its test: a `NamedStyle` of the script's constant name
  with a bold `Font` and a linear `GradientFill` of two stops, added
  with `add_named_style`, set on the cell at (M+1, A) through
  `cell.style`): `--namedstyles-sample` reads it (one custom entry);
  `--gradient-sample` reads its gradient among `gradients_before` 1;
  `--namedstyle-sample` adds a second custom entry; the run log's own
  check confirms emend's forms against openpyxl's field by field where
  both write the field (`gradientFill` `degree`, each `stop`'s
  `position` and `rgb`; the `cellStyleXfs` `xf`'s four ids and its
  lack of `xfId`; the `cellStyle`'s `name` and `xfId`; the cell
  format's `xfId` and apply flags), each printed; what openpyxl
  writes and emend does not (a `cellStyle` `builtinId` on none, the
  `applyX="0"` flags if any) is named in the log and not compared.

`lo` and `lo_in` `ok` on every fixture output. The run log's own
checks print no corpus name, reference, path or text; a style name
is counted, never printed.

### The probe

`probe-emend-fullstyles-write-xml` gains, before its write, a second
`add_style` over no base with a linear gradient of two stops (the
degree and the colours from its constants) on the text cell, and
`add_named_style` of a name from its command line with the first
style's font, then a third `add_style` over no base with `named` the
returned index on the boolean cell; after the write, the output's
`cellStyleXfs` count and the two cells' `fillId` and `xfId` give four
more lines, so no composer and no arm is dead. Every probe above it
in the chain (`format`, `read`, the feature probes) is rebuilt as
part 4's record did; the budget row is the figure.

## Phases

Each phase as `specs/0001` "Rules for every phase" and part 4's
"Phases" paragraph: a doc commit first, the code commits, one record
commit, the scan last, at most two independent checks, the report
under `<phase>`, then stop. Target root
`<home>/.cache/emend-build/m6p5-pN/target`; P1's gate prunes Milestone
6 part 4's roots (`m6p4-p1`, `m6p4-p2`, `m6p4-p3`, `m6p4-close`, listed
with `du -sb`, removed with `rm -r "${var:?}"`, `specs/0040` ruling
G). Logs in `records/logs/m6p5-<phase>/` (`gate.log`, `test.log` on
both targets with the feature sets off, `read`, `rules`, `read,rules`
and every feature, `negative.log`, `run.log`, `scan.log`, `size.log`
in P3); the scan's denylist form whole-word. The generator rules of
`specs/0041` and its rulings (A to P) are carried; the generator
renames the two stale labels of `specs/0041` ruling P as its first
numbered changes. No push during the part; the close-out push follows
P3.

Part 5's first doc commit carries: this spec; `specs/0041` with its
close-out verification and rulings O and P; `ROADMAP.md` with part
4's heading DONE and PUSHED and this part's rows; all three as
planning left them, by hash.

### P1: gradient fills

The doc commit; `Gradient`, `GradientKind`, `Stop`, `Fill::gradient`,
the composer, the checks, the typed read in a cell format and a
`dxf`; `--gradient-sample`; the tests on built packages (the two
forms, the degree-0 form, the one-child rule, each range, the read of
each form and its refusals, the `dxf` round trip); the negative log
through the binary; the corpus run; the Excel-form fixture through
`--namedstyles-sample` and `--gradient-sample`; the open record of the
part's keep. P1 ships no named style.

### P2: named styles

The doc commit (the P1 verification rulings); the core read of the
two containers; `Style::named`, `add_named_style`, the `named` arm,
the two containers in the rewrite, the name checks;
`--namedstyle-sample`; the `named-style` op of
`scripts/openpyxl-ops.py` with its test line; the catalogue entry and
limits; the openpyxl fixture through the three samples; the
Excel-form fixture through `--namedstyle-sample`; P1's corpus run
rerun byte-identical; the open record.

### P3: the probe, the size record, the API listing, the CHANGELOG

The doc commit (the P2 verification rulings); the carry; the probe as
"The probe" says; the size log, recorded as
`records/size-m6p5-<utc-date>-<shortsha>.md`, with the budget's
movement over part 4's 438960 judged against the estimate, the 15000
line and the ceiling, the `read` cost's movement and the `images`,
`comments` and `charts` costs' movements each printed and judged
(`specs/0038` ruling N's form); the API listing rerun, recorded as
`records/api-review-m6p5-<utc-date>-<shortsha>.md` (`specs/0040`
ruling V's added-by rule); `CHANGELOG.md`'s "Added" gains one bullet
citing this spec. The negative log through the binary for every
message reachable there. The run log: the P1 and P2 runs rerun with
the size log's release binaries, byte-identical by sha256 (the
openpyxl fixture by part 1's P3 reading, `specs/0038` ruling P),
recorded as `records/m6p5-p3-<sample>-<utc-date>-<shortsha>.md` and
`.txt`; the open record of the part's keep. A budget above the
ceiling is a finding recorded in the size record and the report, the
record commit made, the part stopped for decision 3.

### Close-out push (after P3)

As part 4's (`specs/0041` "Close-out push (after P3)"): the generator
transforms part 4's close-out scripts, its numbered changes
continuing from 55; the pre driver passes HEAD; the doc commit holds
this spec and `ROADMAP.md`; the base is part 4's close-out doc
commit, named by its subject "M6 part 4 close-out: commit planning
docs (the P3 verification and rulings)"; `PREV_RECORD_PATTERN` is
`^M6 part 4 close-out record`, in the range exactly once; the
close-out doc commit's subject is "M6 part 5 close-out: commit
planning docs (the P3 verification and rulings)"; `CARGO_TARGET_DIR`
printed and checked as `<home>/.cache/emend-build/m6p5-close/target`;
no Excel-record check; the scan's denylist form whole-word; the range
holds no archive; the record commit "M6 part 5 close-out record: the
push, the CI run and the scan" holding the three logs only; pre mode
on a clean tree; one independent check before the report, briefed
with the frozen draft inline (`specs/0041` ruling P). The push carries
part 4's close-out record commit and this part's commits, on
planning's word and the maintainer's authorisation typed in the
implementing session. The hashes are filled in at the briefing.

## Decisions for the maintainer

1. The gradient's place in the API: A (recommended), `Fill` gains
   `gradient: Option<Gradient>` and loses `Copy` (a `Vec` of stops);
   every existing use of `Fill` compiles except an implicit copy and
   a struct literal without the new field, and the typed read keeps
   its `Option<Fill>`. B, `Fill` becomes an enum of a pattern fill
   and a gradient fill, cleaner in the type and a break for every
   consumer that names `Fill`'s fields.
2. Applying a named style to a cell: A (recommended), `Style` gains
   `named: Option<u32>`, so `add_style` and `add_cell_style` keep
   their signatures and a cell format names its style the way the
   part does (`xfId`). B, a new call `add_styled_format(named,
   style)` beside `add_style`, one more method and one more arm.
3. If P3's budget exceeds the ceiling of 458752: A (recommended), the
   part stops at P3 with the finding recorded, and a P4 moves
   `add_named_style`, `Style::named` and the two containers behind a
   feature `namedstyles` off by default, measured as `dates` is, the
   gradient staying core as the smaller item; the ceiling stands. B,
   the ceiling is raised to the measured figure and the part closes
   on it. Nothing is decided if the budget fits.
4. Name equality for the duplicate check: A (recommended), equal
   ignoring ASCII case is taken, since a stricter check never gives a
   consumer a part to repair and no Excel is here to test the looser
   one. B, byte-equal only.

## Out of scope

- Creating `cellStyleXfs` or `cellStyles` where a part lacks them
  (0 in the corpus); `builtinId`, `customBuiltin`, `hidden` and
  `iLevel` on a written entry; removing or renaming a named style;
  changing a kept named style's format in place.
- A gradient in a chart (`specs/0036`'s list, a chart item, taken by
  no part of this milestone); a theme-coloured stop's resolution
  beyond what `resolve_color` does today.
- The apply flags' `0` forms openpyxl writes; the reader's
  `NamedStyle` flags are read, not written.
