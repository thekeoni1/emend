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

## Planning rulings during P1 (2026-10-22)

A. (R1.) `Fill` losing `Copy` and gaining a field (decision 1 A)
touches one line in each struct literal outside the core, the fifteen
`probe-emend-*-write-xml` sources among them (`gradient: None,`), and
a handful of `.clone()` or `.as_ref()` forms in tests and the tool.
Accepted in P1: a literal's new field is not "The probe", which is
P3's addition of the gradient and the named style to the fullstyles
probe. The probes' figures may move in P1 through the shared
composer; P3's record measures against part 4's record, so the
movement is this part's cost wherever it lands. The report lists the
touched files by count per directory.

B. (R2.) The three tests that pinned "a gradient reads as `None`" are
rewritten to the typed read and renamed as the session proposes; the
superseded wording of `specs/0027` and `specs/0032` on a gradient
reading as no fill is the sentence this spec replaces, and the
catalogue says so in P2's limits.

C. (R3 to R5.) The samples that met a gradient as no fill now meet it
typed: `--namedstyles-sample`'s `dxf_gradients` counts the `dxf`s
whose bytes hold a `gradientFill`, each asserted to read with `Some`
gradient, and such a `dxf` passes the writable filter and is written
back, so `dxf_fills`, `dxf_written` and `dxf_equal` grow by them
(the null-rewrite test's figures 1, 3, 4, 4 as the session states);
`--read-sample`'s `gradients` is the distinct formats whose fill
reads with a gradient, the "reads as `None`" wording dropped since
it would count an empty fill; `--roundtrip-sample` gives a gradient
read back to `add_style`, so `fills_after` grows, with the old
`None`-fill case kept on an empty `<fill/>`. The corpus counts of the
three samples move on the 2 gradient workbooks against earlier runs:
the run log names each move beside the earlier figure, as an expected
move, not a regression, and the rerun check of later phases compares
against P1's figures.

D. (R6.) The typed read takes any finite number as a tint is read,
with no range check, and a `gradientFill` with no `stop` reads with
an empty list; the writer alone enforces the ranges and the two-stop
floor, so a part Excel wrote reads whatever it holds and nothing
written leaves this crate out of range. Both are tested, as stated.

E. (R7.) The gradient's checks (the one-child rule, then the kind's
numbers, the stop count, each stop's position and the order, its
colour) run after the alignment and protection checks of `add_style`,
and in a `dxf` after the colours, as "Order of checks" says. The test
pairing a border on a part without `<borders>` with an out-of-range
gradient, refusing with the collection message, pins the order.

F. (The session's correction to R3 to R5, confirmed by planning's own
count 2026-10-22: 6 `gradientFill` elements in 2 workbooks, all 6
inside `extLst` (an `x14:dxfs` extension's `dxf` fills), 0 in
`<fills>` and 0 in the main `<dxfs>`.) Ruling C's corpus clause is
replaced: the typed read reads `<fills>` and the main `<dxfs>` only,
so the three samples meet no corpus gradient and NO move is expected
on the corpus for `--namedstyles-sample`, `--read-sample` or
`--roundtrip-sample`. The run log reruns the three on the corpus on
both targets with the phase's binaries and compares each workbook's
stdout line field by field with the earlier run's (namedstyles
2026-10-12-6f51899, read and roundtrip 2026-10-16-5e52758), printing
per sample "27 of 27 lines equal" and any field that moved beside its
earlier figure (R9, accepted); the expected result is no move, and a
move is a finding. The run log also prints the extension count: 6
`gradientFill` inside `extLst`, 0 in `<fills>`, 0 in the main
`<dxfs>`, over the run, by its own count. `--gradient-sample`'s
`gradients_before` counts main-namespace `gradientFill` start tags
anywhere in the part's bytes, so it finds the six (2 workbooks), and
its `read_back` counts the two formats the sample adds alone. "Facts
planning checked" stands corrected by this ruling: the six are
extension `dxf` fills, which no reader of this crate reads
(`specs/0032`: an `x14:dxfs` is not read).

G. (R8.) The two stale labels of `specs/0041` ruling P occur in the
close-out push script alone, in none of the phase scripts; "Phases"'s
sentence on renaming them applies at the close-out generator, as its
first numbered changes after 55 (56 and 57). P1's generators rename
nothing.

H. (Finding M1, the session's, before any logged run.) The two P1
code commits 0f8d809 (core) and 4bb297d (tool) state counts in their
messages without the sentence the commit-message count rule requires
(`specs/0037`, change AD, carried), an implementer error the report
names. The fix accepted as proposed: both commits rewritten with
identical trees through `git commit-tree`, the sentence "No log
prints a count for this commit; the diff alone carries what it
changes." added before the Co-Authored-By line and nothing else
changed, the chain 893e12c -> core' -> tool' with main moved to
tool'; the doc commit unchanged. A local, unreported commit may be
rewritten before the report (`specs/0001`); the old hashes go in
FACTS and the report, the forks pin the new ones, and nothing ran
against the old ones. The run log's tree check (each rewritten
commit's tree equal to the old one's by `git rev-parse <hash>^{tree}`)
is printed in the gate. Verified by planning from git 2026-10-22: core
6a7644e (was 0f8d809) and tool 6285028 (was 4bb297d), each tree equal
to the old commit's, each message the old one plus the sentence and a
blank line, ASCII, the chain 893e12c -> 6a7644e -> 6285028.

I. (Finding M2, the test fork's hand run, before any logged run.)
`clippy -D warnings` fails on every feature set without `rules`
("method `compose` is never used", `styles/elements.rs`): after the
split of ruling E, `Fill::compose` is called by
`add_differential_style` alone, which is `rules`'s, while `add_style`
calls the colour check, the gradient check and the composer directly.
The session's own check ran clippy with every feature only, an
implementer error the report names. Fix accepted as proposed: one
attribute, `#[cfg(feature = "rules")]` on `Fill::compose`, its doc
line naming the feature, as `color_element` is gated; nothing else
changes; built and clippy'd on every set before the commit. Placement
(a): a third code commit on top, "M6 part 5 P1: Fill::compose under
the rules feature (finding M2)", the phase commit; 6a7644e and
6285028 stay as they are, failing clippy off `rules` in the history,
which the gate names beside M2. The count rule's sentence goes in its
message if it states a count.

J. (R10, departing from ruling I's literal form.) The attribute is
`#[cfg(any(test, feature = "rules"))]`, since the styles element unit
tests call `Fill::compose` in every feature set and the `rules`-only
form stops the crate's tests compiling off `rules` (measured and
reverted by the session before the commit). Accepted: one attribute,
the doc line naming the feature and the tests. Verified by planning
from git 2026-10-22: the phase commit 029400e, parent 6285028, one
file, four lines added and one removed, the attribute and the doc
line as stated, the message stating no count, ASCII.

K. (R11.) A gradient range IS reachable through the binary: under
ruling D the typed read takes any finite value, and
`--roundtrip-sample` and `--namedstyles-sample` give what they read
back to the writer, which refuses it with "a style field is out of
range". The negative log's new cases are accepted as the session
lists them: N16 (an unreadable degree, "a style cannot be read"), N17
(no `<fills>`, the collection message), N18 (the roundtrip sample on
a one-stop gradient), N19 (the named-styles sample on a `dxf`
gradient at degree 400), with the controls B11 to B13 (exit 0, the
expected lines the log's own constants). `--gradient-sample`'s own
gradients stay constants inside the ranges, checked over the tool's
source with planted controls.

L. (Finding M3, against ruling F's text; confirmed by planning from
the run directories 2026-10-22.) The roundtrip run F names,
2026-10-16-5e52758-roundtrip, is the null round trip (its run rows
carry empty `tool_args`, its stdout files are empty, its outputs
identical), not a `--roundtrip-sample` run; the latest
`--roundtrip-sample` corpus run is 2026-10-05-6b6125b-roundtrip,
whose stdout lines begin "roundtrip" with the sample's fields.
Ruling F's roundtrip reference is amended to 2026-10-05-6b6125b; the
named-styles and read references stand. The run log prints the
wrongly named directory's empty `tool_args` as a control. The
session's hand checks (27 of 27 equal on both targets for all three;
the extension count 6, 0, 0; `--gradient-sample` on all 27) are the
logged run's expectation. Planning's word for the logged run given
on this ruling.

## Planning verification of P1 (2026-10-22)

Verified from primaries, not the report: the chain 7359783, 893e12c,
6a7644e, 6285028, 029400e (PHASE), 02c467e (record) by `git log`;
`git diff --numstat 7359783 029400e` printed 33 files, 1721 added, 92
deleted, five of them under package/tests and sixteen under the size
probe's bin (the report's corrected count); the five logs at the
hashes the message gave (gate 847ab555, test 00795752, negative
4927b97f, run 00cf932b, scan 54f7337f), each of the four ending
"checks all as expected: yes" and the scan "RESULT: PASS" with
structural checks failed 0; each log printing PLANNING_SPEC
2041984d, the working copy's hash at this writing; the report at
5b52333a with every figure found in a log by grep: the gate's trees
(ruling H), "FINDING M2" named and not rerun, 4 commits, 155 and 155
messages, 1417 and 1432 test names with the three ruling B renames
the only loss, 16 probe sources with no added line but
`gradient: None,`, the charts probe 1140868 against the size record's
1138352, the format probe 974636 and the budget 441420 against
438960 (judged nowhere; P3's size log judges), the part 4 prune
sizes; the test log's 1429 on both targets and 523, 629, 559, 668,
1182 per set on both, the gradient module's 12 tests by name; the
negative log's N16 to N19 each "exit 2, no stdout" with the named
message, B11 to B13, 19 refused, 14 output files, 18 fixtures; the
run log's 27 ok on each label, 54 of 54 three ways, "gradients_before:
6 in 2 workbooks", 27 of 27 byte for byte across targets, the three
reruns of ruling F each "lines equal 27 of 27; fields moved 0" on
both targets with the ruling L control (the null round trip's empty
tool_args), "total extLst 6 fills 0 dxfs 0 other 0; workbooks 2 of
27", the Excel-form fixture's two sample lines on both targets, the
keep's pairs 2227 with 12 new (the phase's tests, by name) and 2
changed (ruling C's two, by name), the open record lo_in ok 2215 and
lo ok 2216. The four records, the README and the check's answer hash
as the report says; the check's brief and prompt name no absolute
path, the frozen draft is at 94d77302 and the spec copy the check
read is at 2041984d. The check's answer is in the report verbatim.
Planning's own scan, `scripts/privacy.sh --range 893e12c..02c467e`:
4 commits, 2179 blobs, 0 findings, PASS. No API key is in the
environment. Nothing is pushed: origin/main is f630f67. P1 is DONE.

M. (Check 1's 1 a.) Accepted as P1's. "Errors" gives the gradient's
ranges to the existing message "a style field is out of range", so
that entry's limits are the gradient's and belong with the gradient's
code, 6a7644e. The "catalogue entry and limits" that "P2" lists are
the new message's entry and the name limits; P2 adds those and the
sentence ruling B leaves to it. Nothing to change.

N. (Check 1's 1 b.) The session's reading is right. In a dxf the
colours are the fill's own: `Fill::compose` checks the pattern
colours, then the gradient, and the dxf writer composes the font, the
fill and the border in that order. Ruling E's "after the colours"
means that and nothing about the border.

O. (Ruling A's count.) Ruling A said "the fifteen" probe sources;
gate.log printed "probe sources changed: 16", the rules probe
holding two literals, seventeen lines in all. Sixteen is the count;
ruling A reads so from here. No log or commit names fifteen.

P. (The draft's directory counts.) The check found the draft's
"package/tests 6" and "crates/emend/src 5" wrong against numstat;
the final report carries the corrected counts and names the error.
An implementer counting error caught by the check as designed, not a
finding; no number. The rule stands: every count in a report is
printed by a command first, and a per-directory count is printed by
the command that groups it, not worked by hand.

## Planning rulings during P2 (2026-10-22)

Q. (R1.) (a). The two-containers message names every container it
refuses, so its text becomes "the styles part has more than one
numFmts, fonts, fills, borders, cellStyleXfs, cellXfs or
cellStyles"; a second `cellStyleXfs` or `cellStyles` refuses with it
in every feature set, the core's refusal before `read`'s; a second
`dxfs` stays as today. "Design" line "with the existing message"
reads "with the two-containers message, its text grown by the two
names". The set of messages is the base's plus one (the name
message) with one text changed; the catalogue, the two renamed tests
and the negative log carry the new text (ruling X). The spec's "one
new message" counts messages, not texts; `specs/0012` and
`specs/0017` keep their quotes as history.

R. (R2.) Accepted. `cell_style_xfs` is a `pub` core field by the
existing `xf` read; the API record moves in P3 as "P3" says. `Xf`'s
doc loses the feature clause. `read`'s check of a `cellStyle` naming
a format past the container stays `read`'s.

S. (R3, R4.) `text_attribute` moves to the core, one copy, no twin.
A `cellStyle` without `name` is counted in the core with no name and
no refusal; `read` refuses it as today. The comparison is ASCII-case-
folded equality of the unescaped names. "More than 255 characters"
counts Unicode scalar values; the empty name and any character
`forbidden_in_xml` refuses are refused with the name message; tab,
LF and CR are written as character references by `attribute_text`.

T. (R5.) The order at `add_named_style`: the name (empty, length,
characters), the duplicate (the part's names, then those recorded),
the containers (the collection message), then the style as
`add_style` checks one over no base with `named` first. "Design"'s
"the name, then the style" reads so. The order is pinned by one test
giving a bad name on a part without `cellStyles` (the name message)
and a good name on it (the collection message).

U. (R6 to R10.) Accepted as read. Over no base: each id the style's
where given, else the named format's, else 0; `xfId="k"` where
`xfId="0"` stands today; apply flags for the fields given only; the
alignment and protection the style's where given, else the named
format's children's bytes (a recorded one's composed form), no other
child copied. Over a base: the base's copy, `xfId` replaced in place
or added after the base's attributes and any added ids, before the
added apply flags; the base's ids kept. The index check after
protection and before the gradient; an index or count past `u32`
the same message. With no named style recorded both containers are
written byte for byte (P1's rerun pins it); an empty `<cellStyles/>`
is opened up as the empty containers are. A recorded named style
keeps its ids and its composed alignment and protection for a later
`named` naming it.

V. (The sample's note on N2.) The spec stands: over a base the
base's ids are kept and `xfId` alone names the style, which is
"Design"'s text and not openpyxl's `cell.style` (that copies the
named format's ids, the no-base form here). `Style::named`'s doc
says so in one sentence: a caller wanting openpyxl's form gives no
base. The openpyxl fixture's comparison covers the no-base form only.

W. (The sample.) "emend sample style" is the tool's constant, not
corpus content, and may be printed. The session's count over the
directory was not over the corpus's 27 alone; the run log prints its
own count of `cellStyle` entries equal to the constant ignoring
ASCII case over the 27 corpus workbooks, expected 0, as the
duplicate check's control, beside `custom_before` 36 (the "Facts"
count). The tool test's line and the forms on its part are accepted
as the session printed them; the line is pinned by the tool test.

X. (The negative log.) P2's cases continue from N19 with controls
from B13: the name message through the binary (a part holding the
constant's name in another case), the index message, the collection
message on a part without `cellStyles`, and the two-containers
message's new text on a part with two `cellStyles` (ruling Q), each
exit 2, no stdout, no output file.

Y. (Tests.) The list stands with ruling T's pairing added to the
collection test; R1's two renames are the only renames. The test
names are read by the gate as P1's were.

Z. (Z1.) Ruling Q reaches five base tests: the two renamed and three
more (`a_second_num_fmts_is_refused`, `a_second_cell_xfs_is_refused`,
`a_second_fonts_fills_or_borders_is_refused`) that take the new text
by the one string alone, names kept. The catalogue entry names the
five.

AA. (Z2.) As ruling A for P1: `Style` gaining `named` touches one
`named: None,` line in each full `Style` literal outside the core,
the sixteen probe sources (sixteen lines), the tool's round-trip
literal, one line each in `namedstyles.rs` and `readback.rs`, and
`same_style`'s destructuring in `read.rs`. The gate prints the probe
sources changed and the added lines other than `named: None,`, 16
and 0 expected.

AB. (Z3.) A `named` on a differential style refuses with the
existing "a differential style holds a field this part does not
write", as a number format does: a `dxf` has no `xfId`. "Design"
reads so; the test `a_differential_style_given_named_is_refused`
(`rules`) stays, one beyond ruling Y's list.

AC. (Z4, Z5.) The tests as written, with ruling T's pairing, are
accepted. Under `read` a part with a `cellStyles` entry and no
`cellStyleXfs` is unreadable, so the collection test's missing-
`cellStyleXfs` case carries an empty `<cellStyles count="0"/>`: a
fixture choice, no code change. With nothing recorded the two
containers pass through untouched, `count` included, unlike the
other grown containers whose `count` is normalised on every rewrite;
the byte-for-byte test pins it with counts that disagree with the
children.

AD. (Z6.) The op's constant "emend named style" differs from the
tool's "emend sample style" so the fixture's entry and the sample's
addition never collide; both are constants and may be printed.
Ruling W's count over the corpus is of the tool's constant.

AE. (Z7.) The run log compares, where both write the field: the
gradient's `degree`, each stop's `position` and `rgb`; the named
`xf`'s four ids present and its lack of `xfId`; the `cellStyle`'s
`name` present and its `xfId`; the cell format's `xfId`. The ids are
compared by what they name (the part's bold font, the gradient fill),
not by number. Named and not compared: `type="linear"`, `hidden="0"`,
`pivotButton`, `quotePrefix`, openpyxl's added empty `<border/>`, and
the apply flags (openpyxl writes none; emend writes `applyFont` and
`applyFill` on its named `xf` and `applyNumberFormat` on N1).

AF. (Z8.) "Harness runs"' "one custom entry" on the openpyxl fixture
reads as `named` less `builtin` equal to 1 on the `--namedstyles-
sample` line, since that line's `custom` counts `customBuiltin`. The
three lines the session hand-ran are what the run log prints and
checks: `named=2 ... builtin=1`, `--gradient-sample` `read_back=2
gradients_before=1`, `--namedstyle-sample` `named_after=3
read_back=3 custom_before=1`.

AG. (Z9.) The phase's files are as listed; no manifest, lock or
CHANGELOG change. PHASE is bf07f86 unless a finding lands a further
commit.

AH. (The word.) The facts file and the generator forks proceed on
bf07f86 against the working spec at this ruling's hash; the logged
runs follow the dry runs as in P1, then the record commit, the scan
and the report, then stop.

AI. (G1, the facts file's count.) The carried message reader matches
`Error::Package("...")` on one line, so it never counted the thirteen
base messages rustfmt wrote over two lines and now misses the new
two-containers text; the facts file's 156 was the session's error.
The gate keeps the carried reader with its set pinned as the base's
plus the name message less the old text (155 to 155, named as the
one-line reader's) and adds a multi-line reader whose whole set is
168 to 169 (the name message and the new text added, the old text
gone), both printed. The negative log's own reader holds the folded
text already. Planning's own count at b427e1d and bf07f86: 155
distinct one-line messages at each.

AJ. (G2, the prune.) P2's gate prunes P1's target root
`<home>/.cache/emend-build/m6p5-p1/target` in the `m6p4-p2` form
(listed with `du -sb`, removed with `rm -r "${var:?}"`), one target
root per phase as part 4's P2 did; the brief's "no prune" was
planning's omission. The gate fork regenerates from that form before
the dry run, the first generator moved aside as P1's was.

## Planning verification of P2 (2026-10-23)

Verified from primaries, not the report: the chain 02c467e, b427e1d,
d0e7b32, cbd5f81, bf07f86 (PHASE), d4d9eca (record) by `git log`;
`git diff --numstat 02c467e bf07f86` printed 31 files, 1684 added,
170 deleted, grouped by directory as the report gives; the five logs
at the hashes the message gave (gate 771d14dd, test a17c2c42,
negative fc8ad12e, run 616cbf86, scan 547c69af), each of the four
ending "checks all as expected: yes" and the scan "RESULT: PASS";
each log printing PLANNING_SPEC bda6c1f8, the working copy's hash at
this writing; the report at 53c1af95 with every figure found by grep:
the prune of P1's target root (11972721687 bytes, `rm -r` exit 0),
the one-line reader 155 and 155 and the several-line reader 168 and
169 with the thirteen base messages it alone sees listed, test names
1432 and 1450, the sets 537, 710, 574, 752, 755, 817, 1198 on both
targets, nine PASS lines, probe sources 16 with 0 other added lines,
the charts probe 1147896, the format probe 981312 and the budget
448096 against 438960 (judged nowhere; P3's size log judges); the
test log's 1447 on both targets and 537, 644, 574, 684, 1198 per set
on both, the module's 15 tests read, the ops test's 17 checks, 109 ok
lines and PASS; the negative log's N20 to N23 each "exit 2, no
stdout" with the named message, 23 refused, 15 output files, 23
fixtures; the run log's 27 ok on each label, read_back 81, custom
36 in 23, named_before 221, ruling W's count 0 with its planted
control 1, 27 of 27 byte for byte across targets, P1's gradient run
byte-identical by sha256 27 of 27 on each label, the openpyxl
fixture's three lines as ruling AF, "fields compared 10; the same
form 10", the keep's 2235 pairs with 8 new and 0 changed, the open
record lo_in ok 2223 and lo ok 2224. The six records, the README and
the check's answer hash as the report says; the check's text in the
report equals the committed answer byte for byte; the brief and the
prompt name no absolute path; the frozen draft is at a90cbe73 and
the spec copy the check read at bda6c1f8. Planning's own scan,
`scripts/privacy.sh --range 02c467e..d4d9eca`: 5 commits, 2219
blobs, 0 findings, PASS. No API key is in the environment. Nothing
is pushed: origin/main is f630f67. P2 is DONE.

The budget at bf07f86 is 448096, 9136 over part 4's 438960 and
10656 under the ceiling 458752; the movement is within the estimate
8000 to 15000 and under the 15000 line, with P3's probe additions
still to land. P3's size log judges it.

AK. (Check 1's 1 a.) The core commit's message names rulings Q to
Y; Z, AA and AB were ruled after the commit, on the session's
readings of it, and the message describes those changes without
naming them. Accepted as it stands: the logs ran against bf07f86 and
a rewrite would move every hash for a message's ruling range. The
README names it; this ruling records it.

AL. (Check 1's 1 b.) The opening of a self-closing `<cellStyles/>`
or `<cellStyleXfs/>` for a recorded named style is carried by the
rewrite's general empty-container branch and pinned by no test. P3
adds one test on a built package whose part holds both containers
self-closing, writing a named style and a format naming it, in a
test-only commit before the probe commit. It is a test of ruling U,
not a code change.

AM. (Check 1's 1 c.) The continuation line's indentation in
`a_second_fonts_fills_or_borders_is_refused` is cosmetic; `fmt`
passed and the value is unchanged. Left as it is; no commit touches
it for that alone.

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
