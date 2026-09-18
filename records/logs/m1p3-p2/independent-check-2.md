<!-- Committed verbatim but for three characters: this repository's text is ASCII (CLAUDE.md), and the answer as written held 32 em dashes, five ellipses and two minus signs. Each em dash became ' - ', each ellipsis '...', each minus sign '-'. Nothing else is changed; the answer as written has sha256 29f41b6f3e13c3e5e5cff9f15680ad24bfedf4041d9fd932b6d71d364df0fdc6. -->

# Independent check 2: emend M1 part 3 P2 (the cell reader)

Fresh reviewer, no access to the session that produced the work. Judged from
`specs/0010-reading-cells.md`, `specs/0001-bootstrap.md`, `CLAUDE.md`, the diff
`1f4481d^..HEAD`, the five logs in `records/logs/m1p3-p2/`, the record pair, and
the draft report. I ran `git`, `grep`, `cargo fmt --check`, `cargo clippy`,
`cargo test` (in throwaway trees under `/tmp`), `sha256sum`, and the built
`target/debug/emend-copy` on fifteen-plus adversarial packages I generated in
`/tmp/p2check`. I read no file under the corpus directory and did not run the
corpus runner. I changed nothing in the repository.

**Verdicts in one line each.**

1. **Does the diff do what the spec says, and nothing else? MOSTLY YES, with one
   spec clause not carried (the `lib.rs` re-exports) and one public method the
   spec puts in P3 that nothing in P2 needs and nothing anywhere tests
   (`Cell::error`).** Every one of the P2 test-list items is covered by a named
   test. No test asserts a position or a mechanism.
2. **Do the logs contain every line the spec requires? NO.** `gate.log` fails the
   phase-commit rule this phase's own doc commit added, in exactly the form the
   rule names as worthless; two bite claims in test comments (one new in this
   phase) are not measured in `test.log`; and one verdict in `run.log` cannot
   fail. `run.log`'s account of P2-F1 is a measurement, not a story, with one
   caveat named below.
3. **Does the report's every number and quoted string appear in a primary? NO,
   with two exceptions and several overstatements.** "Twenty-one of the
   twenty-three agree exactly" is contradicted by `run.log` (eighteen agree, two
   depart, three have no figure to agree with); "Three logs were rerun" is two;
   "the cell tests that reach them" reaches nothing. Every other figure, hash and
   quoted string I checked appears in a primary and is correct.

Findings are numbered C2-n and ordered by what I think matters.

---

## C2-1. `gate.log` does not satisfy the phase-commit rule this phase added, and
## the report says every log does

`specs/0001-bootstrap.md` (added by this phase's own doc commit `1f4481d`):

> A log's phase commit is given to the script, by argument or environment, and
> compared with `git rev-parse HEAD` before the run, with a control that refuses
> a fabricated commit; the header prints both values and the comparison. **A
> header that reads the commit from HEAD and prints it twice asserts nothing**
> (M1 part 3 P1 ruling 11 [...]).

`gate.log` does precisely the forbidden thing:

- `records/logs/m1p3-p2/gate.log:92`  -  `C=$(git rev-parse HEAD)`
- `records/logs/m1p3-p2/gate.log:94`  -  `echo "about: M1 part 3 P2 phase commits, last is $C; HEAD at run: $C"`
- output, `records/logs/m1p3-p2/gate.log:2`  -  the same sha printed twice.
- Its usage line (`gate.log:14`) is `m1p3-p2-gate.sh > <scratch file>`: no commit
  is given to the script. `grep -n '^    C=\|rev-parse HEAD\|fabricat\|head_is'`
  over `gate.log` returns only lines 1, 92 and 93. There is no comparison and no
  control anywhere in it.

The other four logs carry the rule properly: `test.log:60` (`C=${1:?...}`),
`:70-78` (hard guard), `:120-122` (printed comparison plus the null-commit
control); `negative.log:115,131-137,167-168`; `run.log:193,696` and its
"compared with HEAD before this run started" lines at `run.log:3-4`;
`scan.log:101-107,236-238` with two controls (a non-HEAD commit and a short sha
of the real one). `scan.log`'s own derivation note (`scan.log:64-67`) states the
rule in full and says P1's scan violated it, so the rule was in hand when
`gate.log` was written for the third time.

**Disagreement with the report.** "The logs" section claims:

> - a log's phase commit is given to the script and compared with HEAD, both
>   printed, with a control that refuses a fabricated commit: **every log in this
>   phase does this**;

That is false. Four of the five do; `gate.log` does none of it. This is the same
defect class the report itself names twice (a check or a label that asserts a
position or a mechanism rather than a property): the gate header's second field
is not a measurement of anything, it is the first field again.

## C2-2. Two bite claims in test comments are not measured in `test.log`, and one
## of them is new in this phase

`specs/0001-bootstrap.md`, added by `1f4481d`:

> A claim that a test bites, that it would fail under a named change to the code,
> is a measurement: `test.log` prints the planted change, the run and the failing
> assertion's output, **or the claim is not made anywhere, comments and commit
> messages included**.

Two comments make such a claim about a named change to the code (a flag in place
of the `phonetic` depth in `text_under`):

- `crates/emend/src/cells/tests.rs:198-205` (carried unchanged from P1; present
  already at `d1ebf1c`): "A flag cleared at the first `</t>` inside the subtree is
  live again by the time `y` arrives, so **it reads "ayb" where the depth reads
  "ab"** [...] independent check 2 of this phase built the flag and proved it."
- `crates/emend/src/cells/tests.rs:685-689`  -  **new in `6c27167`**: "a skip that
  ended at the first end tag inside the subtree **would read "ayb"**."

`test.log`'s mutation block plants seven mutations (M1 line ends, M2 the CData
arm, M3 the self-closing refusal, M4 CDATA appended at the end, M5 the
unresolved-target answer, M6 and M7 the two test expectations:
`test.log:1871-1946`). **None of them is the flag.** So a bite claim stands in
this phase's own new code with no printed measurement, which the phase's own new
rule forbids. The appeal in the older comment to "independent check 2 of this
phase built the flag and proved it" is an appeal to an earlier check's private
work, which is exactly what the rule replaced.

I measured it myself, twice, in throwaway trees (`git archive 6c27167 | tar -x -C
/tmp/p2check/tree{,2}`, separate `CARGO_TARGET_DIR`):

- Minimal substitution (the `phonetic > 0` arm emptied, `phonetic -= 1` becomes
  `phonetic = 0`): both tests fail, with
  `left: Some("a")` / `right: Some("ab")` and `left: ["a"]` / `right: ["ab"]`.
  The `</r>` then reads as the container's own end tag and `text_under` returns
  early, so the item is truncated rather than contaminated.
- The other natural flag (the `rPh` flag suppresses only text; `depth` keeps
  counting the nesting): both tests fail with `left: Some("ayb")` and
  `left: ["ayb"]`, exactly the value the comments name.

So the tests do bite, and the comments' claimed value is right for one plausible
flag and wrong for the other. The finding is procedural, not a false claim: the
rule was not applied to these two comments, and the report's

> - a bite claim is a printed measurement in `test.log` or is not made anywhere:
>   the seven mutations above;

is therefore incomplete. There were nine such claims in reach, not seven. (I
checked the rest of the phase's counterfactual comments: the doc comment at
`tools/emend-copy/tests/null_rewrite.rs:935-939`, "without it the pair survives
and `bytes` reads 9 rather than 8", IS measured  -  `test.log:1973-1976` prints
`left: "strings\titems=2\tbytes=9\n"` against `right: ...bytes=8...`. The
remaining "a reader that folded them would ..." comments name a hypothetical
different reader rather than a named change to this code, which I read as
outside the rule's letter.)

## C2-3. A spec clause the diff does not carry: the `lib.rs` re-exports

`specs/0010-reading-cells.md`, "Crate":

> `crates/emend` gains the module `cells`; `lib.rs` re-exports `cells::{Cell,
> CellRef, Cells, Formula, FormulaKind, Kind, SharedStrings}`.

`crates/emend/src/lib.rs:35` re-exports `cells::SharedStrings` alone (P1's), and
`lib.rs` is not in the phase diff at all (`git diff --stat 1f4481d^..HEAD`: eight
files, none of them `lib.rs`). The six types P2 adds are reachable only as
`emend::cells::Cell` and so on, which is how `tools/emend-copy/src/main.rs:74`
imports them. Nothing in the diff, in any log, or in the report mentions this. It
is a one-line omission with no behavioural consequence, but it is a sentence of
the spec that the phase did not carry and did not raise.

## C2-4. One awk bug in `run.log`, three consequences: a dead verdict, split log
## lines, and a malformed committed record

`records/logs/m1p3-p2/run.log:410-413`:

```
key_of() {
  printf '%s\n' "$1" | awk -v want="$2" '
    { for (i = 1; i <= NF; i++) { split($i, kv, "="); if (kv[1] == want) print kv[2] } }'
}
```

The tool's line begins with the bare word `cells`. `split("cells", kv, "=")`
gives `kv[1] == "cells"`, `kv[2] == ""`, so `key_of "$line" cells` prints an
empty line first and the number second. I reproduced it:

```
$ line=$(printf 'cells\tsheets=1\tcells=6\tvalues=5')
$ v=$(printf '%s\n' "$line" | awk -v want=cells '{for(i=1;i<=NF;i++){split($i,kv,"="); if(kv[1]==want) print kv[2]}}')
$ printf '[%s]\n' "$v"
[
6]
```

Three consequences, all visible in the committed files:

1. **A verdict that cannot fail.** `run.log:1114` is
   `if [ "$(key_of "$line" cells)" = none ]; then ... no_line=$((no_line+1))`.
   `key_of` never emits `none`  -  a missing key yields the empty string (I checked:
   `[ "" = none ]` is false). So `no_line` is always 0 and the verdict "host every
   line carried a cells key: as-expected" (`run.log:1861`, `:2050`) asserts
   nothing. This is the recorded defect class again. Mitigation, which is real:
   the same property is covered by `cells_counts`' own counters, asserted at
   `run.log:1858-1860` (`lines=27`, `not_cells=0`, `malformed=0`), so no coverage
   is lost  -  but the check at `:1144` is dead and the counter it prints is dead
   with it.
2. **Split lines in the log.** `run.log:1805-1806`, and every other per-workbook
   "agree" line on both targets:
   `    054df34b1472  agree  sheets=4 cells=` / newline / `504 values=501 formulas=72 shared=284`.
3. **A malformed committed record.** `records/m1p3-p2-2026-09-18-6c27167.txt` has
   a literal newline inside every one of the 27 data rows of each target's table,
   right after the `sheets` field (`sed -n 9,10p ... | cat -A` shows
   `054df34b1472^I4^I$` then `504^I220^I...`). The record's per-workbook table is
   not parseable as TSV; a consumer reading it gets 54 half-rows per target.

Nothing in the corpus totals is wrong because of this: the totals come from
`cells_counts` over the whole file, whose output starts `lines=` and so has no
bare-word collision, and the substantive per-workbook comparison is
`[ "$line" = "$own" ]` on the whole strings (`run.log:1121`). I checked every
other `key_of ... cells` use (`run.log:865,1198,1200,1202,1204,1230,1244,1260`):
all are over `cells_counts` output, so none is affected.

## C2-5. Two check labels assert more than their arithmetic pins, and the report
## and the record repeat one of them as fact

(a) `run.log:1216-1217` asserts `number - numbers_parsed == empty` under the label
"every Number cell holding a value parsed as a number"; the output is
`number - numbers_parsed = 5410; empty = 5410` (`run.log:1926`, `:2115`). Write
`Nnv` for Number cells with no value and `U` for Number cells whose value does not
parse: the identity gives `Nnv + U = empty`, not `U = 0`. `U = 0` follows only if
every one of the 5410 cells with neither value nor formula is a `Number` cell and
no `Number` cell carries a formula without a cached value. Neither is measured
anywhere in the logs. The record
(`records/m1p3-p2-2026-09-18-6c27167.md`) and the report both state the stronger
thing as measured:

> It does not: 1310988 minus 1305578 is exactly the 5410 `Number` cells that hold
> no value at all, and every one of the remaining 1305578 parses.

It is very probably true. It is not measured. The honest form is the identity the
log prints plus the premise it needs.

(b) `run.log:1205-1207` asserts `resolved == shared` under the label "every
shared-string cell resolved to an item". A `SharedString` cell with **no value**
resolves to `Ok(None)` (`crates/emend/src/cells.rs:231-233`) and is counted
`resolved` by the tool (`tools/emend-copy/src/main.rs:609-613`) with no item
looked up. I built such a package (`<c r="A1" t="s"/>`) and ran the phase's own
debug binary and the phase's own independent python reader over it:

```
tool: cells=1 shared=1 values=0 empty=1 resolved=1 unresolved=0
own:  cells=1 shared=1 values=0 empty=1 resolved=0 unresolved=1
```

So the two routes disagree on that input, and the label "resolved to an item" is
true of the corpus only because the corpus holds no such cell  -  which the
file-by-file agreement of the two routes does establish, but which the log never
says. The spec's own wording ("`SharedString` cells whose `resolve` succeeds and
fails") is on the tool's side, so this is a label problem, not a behaviour one.

## C2-6. `Cell::error` is code the spec put in P3, that nothing in P2 needs and
## nothing anywhere exercises  -  and the report says otherwise

`grep -rn "\.number()\|\.boolean()\|\.error()" --include=*.rs` over the tree
finds exactly three call sites of these methods:
`tools/emend-copy/src/main.rs:602` (`number()`), `:605` (`boolean()`), and three
unrelated `e.error()` calls on a `zip` error type. **No test in
`crates/emend/src/cells/tests.rs` calls any of the three, and
`Cell::error()` (`crates/emend/src/cells.rs:461-466`) has no caller in the
repository at all.**

`number()` and `boolean()` are genuinely forced by P2's acceptance: the spec's
`numbers_parsed` key is defined through `number()`. `error()` is not: the `error`
key counts `Kind::Error` cells, not `error()` results. So `error()` is a public
method, added ahead of its phase, with no caller, no test and no measurement.

**Disagreement with the report:**

> Their dedicated tests, the ones P3's section lists [...] are NOT written here
> and remain P3's work. The methods are exercised only through the corpus run and
> through the cell tests that reach them.

The second sentence is wrong twice over: no cell test reaches them, and `error()`
is exercised by nothing whatsoever. The right sentence is "`number()` and
`boolean()` are exercised only through `--cells`, its integration test and the
corpus run; `error()` is not exercised at all and its first exercise is P3's."
The commit message of `6c27167` is more accurate here than the report ("No test
of the three is in this commit"), but it too claims all three "cannot be counted
without them", which is true of two.

## C2-7. "Twenty-one of the twenty-three agree exactly" is contradicted by
## `run.log`  -  and the same sentence is in the committed record

`run.log:217` (the script's own words): "**Twenty of the 23 keys carry a
figure**." `SPEC_TOTALS` (`run.log:218-223`) holds those twenty; `SPEC_NONE`
(`run.log:229`) holds `max_row max_col numbers_parsed`. The output
(`run.log:1869-1916`, and identically `:2058-2105` for i686) shows **eighteen**
keys equal to the spec's figure, **two** departing (`values`, `empty`), and
**three** with no figure printed under "the three keys specs/0010 gives no figure
for".

The report says "Twenty-one of the twenty-three agree exactly:" and then lists
eighteen key/value pairs. Twenty-one is the number of keys that did not depart,
three of which had nothing to agree with. The same sentence stands in
`records/m1p3-p2-2026-09-18-6c27167.md` ("Twenty-one of the twenty-three agree
exactly"), so the figure does appear in a primary  -  but that primary is this
phase's own prose, and the measurement under it says 18 / 2 / 3.

## C2-8. The record cites the wrong section of the spec for its figures

`records/m1p3-p2-2026-09-18-6c27167.md`: "each key's corpus total is asserted by
name against the figures specs/0010 gives under **"Facts planning checked"**".
The 23-key totals are in "Harness runs" ("Expected results [...] P2:"), which is
what `run.log:37-38` and `run.log:215-217` cite correctly. Wrong provenance in a
committed record.

## C2-9. The module's Limits and `Cells` doc claim more streaming than holds

`crates/emend/src/cells.rs:78-80`: "the cells themselves stream, **so the table is
the only whole-part cost in this module**", and `:471-473`: "It reads forwards
once, holds one cell at a time and never seeks, **so a worksheet larger than the
address space still streams**."

`text_of` (`cells.rs:1065-1100`) accumulates one element's entire text into a
`String`, and `text_under` concatenates every `<t>` under a container into
another. So a single `<v>`, `<f>` or `<is>` is also a whole-element cost, and a
worksheet whose largest value exceeds the address space does not stream. The
corpus makes this unreachable (longest `<v>` 192 bytes, longest formula 511
bytes, `specs/0010`, "Facts planning checked") and the spec's own model says "one
token [...] plus one `Cell`", which is the honest bound. The two sentences above
assert a property that the reader does not have; the fix is a clause, not code.

## C2-10. A crate doc sentence this phase made false

`crates/emend/src/lib.rs:12`: "Still nothing here reads a cell." The crate doc
describes part 1 and part 2 P1 only. `lib.rs` is not in the phase diff. `specs/
0010` gives the two module headers to P3, not the crate header, so this is not a
missed clause  -  but it is a published doc sentence that the phase falsified and
nobody flagged.

## C2-11. Smaller report overstatements

- "**Three logs were rerun**, each keeping and hashing its superseded attempts in
  its own header: `test.log` [...] and `gate.log` twice". Two logs were rerun,
  with three superseded attempts between them (`gate.log:4-9`: attempts 1 and 2;
  `test.log:4-7`: attempt 1). `scan.log` is run twice by design, which the report
  treats separately and correctly.
- "both were first run on **four fixtures** the log builds as zips, **whose
  answers were written into the script before either route saw them**".
  `run.log:988-990` says "The expected line for **the first two** was written into
  this script before either route was run on them"; the unresolved and
  no-workbook fixtures carry property assertions (`run.log:1003-1010`), not
  written-down lines. The claim as phrased is stronger than the log.
- "`specs/0001` says the same of a failing run". `specs/0001-bootstrap.md:83-87`
  says the opposite emphasis: "record the failure in the phase log, fix it, and
  re-run with the fix named". The rule the report wants is `specs/0010`'s ("a
  departure from any fact above is a finding [...] and planning rules which") and
  `CLAUDE.md`'s ("Nothing is re-run 'until it passes'"). The phase's conduct is
  right; the citation is not.
- "the package is borrowed for the life of the iteration **through a
  `PhantomData<&'p mut Package<R>>`**, so a second reader cannot be opened". The
  exclusive borrow comes from the signature `Package::cells(&mut self) ->
  Result<Cells<'_, R>, Error>` (`package.rs:610-614`); the `PhantomData` field
  (`cells.rs:524`) names whose borrow the type holds and makes it invariant. The
  property is real and enforced; the mechanism named is not what enforces it.
- "Four, in `records/logs/m1p3-p2/`: `gate.log`, `test.log`, `negative.log`,
  `run.log`, and `scan.log` written last"  -  five files, four of them siblings of
  the scan. The check prompt uses the same convention, so I raise it only as
  wording.

## C2-12. What I could not break in the reader, and the five holes I found

I read `crates/emend/src/cells.rs` line by line and then attacked the built
`target/debug/emend-copy` (sha256 `23dcaa32...`, the binary `gate.log:1021`
hashes) with generated packages in `/tmp/p2check`. Nothing made it loop, panic,
overflow, or return a cell it should refuse. Specifics worth recording:

- **No unbounded loop or recursion.** `advance`, `read_cell`, `text_under`,
  `text_of` and `skip_subtree` each consume at least one token per turn and each
  ends on exhaustion with `Error::Xml(index)` (`cells.rs:653,947,1043,1099`).
  Nesting is tracked with depth counters, not recursion, so a pathologically deep
  part costs memory and not stack.
- **Nesting cannot confuse the state machine.** A `<row>` inside a `<row>`, a
  `<sheetData>` inside a `<sheetData>`, a `<c>` inside an `extLst` inside a `<c>`,
  a `<row>` inside a `conditionalFormatting`, a `<row>` inside an
  `mc:AlternateContent`, a `<c>` directly under `<sheetData>` with no row open,
  and a `<v>` under an unknown child of `<c>` are each skipped whole, and the
  cell count stayed at what the fixture called for in every case. The comment at
  `cells.rs:608-611` ("Only the elements this reader ENTERED can reach here") is
  true: the reader enters only the root, `sheetData` and `row`, and everything
  else is consumed whole by `read_cell` or `skip_subtree`.
- **Namespace bookkeeping survives a skipped subtree.** A prefix bound inside a
  skipped element and used after it is refused ("an element or attribute uses a
  prefix its document does not bind"); a default namespace redefined on a skipped
  element does not leak onto the following `<c>` (that cell was still counted).
- **`CellRef::parse` is sound at the boundary.** I recomputed the base-26
  spellings: `MWLQKWU` = 4294967295, `MWLQKWV` = 4294967296, `FXSHRXW` =
  2147483647 (= `i32::MAX`), `FXSHRXX` = 2147483648. The corrected M6 pair is
  right and the first draft's was an `i32` boundary, exactly as the report says.
  The `u64` accumulators with a check per step cannot wrap (`col` is bounded by
  `2^32` before each `* 26`), and `a1()`'s seven-letter buffer is exactly enough
  for `u32::MAX`, so no column in range is truncated.

The holes, none of them spec violations and none reachable in the corpus, all
unmentioned in the logs and the report:

1. **A reference the reader can emit that its own parser refuses.** `<row r="0">`
   with a cell that has no `r` yields `CellRef { row: 0, col: 1 }` (measured:
   `max_row=0`), whose `a1()` is `"A0"`, which `CellRef::parse` rejects for its
   leading zero. `a_reference_writes_itself_back` pins the round trip only for
   spellings that parse, so the property "`parse(a1(x)) == x` for every `x` this
   reader produces" does not hold. `CellRef`'s doc says both fields are 1-based;
   nothing enforces it.
2. **Column saturation has no comment and no test.** `cells.rs:830` uses
   `previous.saturating_add(1)` where the row's saturation at `:572-576` carries
   a three-line justification. Measured: a row `<c r="MWLQKWU1">` followed by two
   cells with no `r` gives `cells=3` with `max_col=4294967295` and **two distinct
   cells carrying the identical `CellRef`**  -  a caller keying by reference would
   silently collapse them.
3. **`u32::from_str` accepts a leading `+`.** `<row r="+3">`, `s="+1"`, `si="+0"`
   and a shared index `"+1"` are all accepted where the spec says "a decimal
   `u32`" (measured: `max_row=3 styled=1 resolved=1`). Permissive rather than
   wrong-valued, and `run.log`'s python route is equally permissive
   (`int("+1")`), so the cross-check would not catch it.
4. **An out-of-range `si` gets the "no index" message.** `<f t="shared"
   si="4294967296">` is refused with "a shared formula has no index" although it
   has one. The code owns the choice in a comment (`cells.rs:864-866`) and a test
   pins the non-numeric spelling; the spec rules on an absent `si` only. Worth a
   ruling, not a fix.
5. **A residual limit of the two-reader cross-check.** The library and
   `run.log`'s python reader diverge on a `<sheetData>` in a place the schema does
   not allow: nested in another `sheetData` (library 1 cell, python 2) or inside
   an `extLst` (library 0, python 1), because the python route finds `sheetData`
   through `root.iter()` while the library skips unknown subtrees whole. The
   library is the stricter of the two, no corpus workbook triggers it (all 27
   lines agree), and the log's four fixtures do not cover it. I mention it because
   `run.log:98-107` rests its case on the two routes not sharing a mistake, and
   this is the shape of input that separates them.

---

# Question 1, clause by clause

## The named surfaces

| spec clause | in the diff |
| --- | --- |
| `cells::{Cell, CellRef, Cells, Formula, FormulaKind, Kind}` | yes, `cells.rs:250-525` |
| `Package::cells` | yes, `package.rs:610-614`; refuses with the existing "no such part" |
| `SharedStrings::resolve` | yes, `cells.rs:227-241`, two distinct refusals |
| `--cells` | yes, `main.rs:467-623`, 23 keys in the spec's own order |
| `lib.rs` re-exports the seven names ("Crate") | **NO  -  C2-3** |
| `Package::cells` opens the entry "through `read_part` and `Stream`" | via `self.archive.read(index)`, which is literally what `Package::read_part` (`package.rs:552-554`) wraps. Same call one layer down; no behavioural difference. |
| the eleven/twelve new messages join `package`'s catalogue with the test that pins each | yes: one in `da20c6b` ("a text element holds an element"), **nine** in `6c27167`. The report's "Nine messages were added" is right for the feature commit; the phase added ten. `package::tests::the_catalogue_and_the_source_hold_the_same_messages` reads `cells.rs` (`package/tests.rs:2131`) and passes on both targets (`test.log:1063`, `:1428`). |
| `ca`, `aca`, `dataTable`'s attributes not modelled | yes |
| the run script and record script unchanged, operation named through `--tool-arg` | yes: no `scripts/` file is in the diff |

## The P2 test list, item by item

All in `crates/emend/src/cells/tests.rs` unless said otherwise. 71 tests in the
module (`test.log:1800`), 42 of them new in `6c27167` (20 at `1f4481d`, 29 at
`da20c6b`, 71 at `6c27167`  -  counted with `git show <c>:...|grep -c '^#\[test\]'`).

| spec item | test |
| --- | --- |
| every kind | `every_cell_type_reads_as_its_kind` (kinds, references and values all three compared) |
| `t="n"` and `t` absent both `Number` | `a_number_is_a_number_with_or_without_its_type` |
| `<v>` text, `<v/>`, no `<v>` | `a_value_may_be_text_empty_or_absent` (four cases: text, `<v/>`, `<c/>`, `<c></c>`) |
| inline string of one `<t>` | `an_inline_string_is_the_text_under_its_is` |
| inline string of runs | `the_runs_of_an_inline_string_concatenate` |
| inline string with `<rPh>` | `a_phonetic_subtree_inside_an_inline_string_is_skipped` |
| normal formula | `a_normal_formula_is_its_text` (both `t` absent and `t="normal"`) |
| shared master with `ref`, follower with empty text | `a_shared_master_carries_its_range_and_its_follower_carries_no_text` |
| array formula | `an_array_formula_carries_its_range` |
| `dataTable` accepted by name | `a_data_table_formula_is_accepted_by_name` |
| `ca`, `aca` present and not modelled | `a_formulas_other_attributes_are_not_modelled` (equality against the same cell without them) |
| a character reference in `<v>`, `<f>`, `<t>` | `a_character_reference_is_expanded_in_a_value_a_formula_and_a_text` |
| `CellRef::parse` `A1` | `a_cell_reference_parses_only_the_a1_form` |
| `XFD1048576` | same |
| `a1`, `A01`, `A`, `1`, `A1B` | same (plus `b""`) |
| a column beyond `u32` | same (`MWLQKWU1` accepted, `MWLQKWV1` `None`; and the row axis too) |
| `a1()` back | `a_reference_writes_itself_back` (7 spellings incl. `Z1`/`AA1` and `MWLQKWU1`) |
| a row without `r` | `a_row_without_a_number_follows_the_last` (running count, not position) |
| a cell without `r` | `a_cell_without_a_reference_follows_the_previous` (restart per row, explicit `r` moves the count) |
| a cell whose reference names another row | `a_cells_own_reference_wins_over_its_row` |
| a style index | `a_style_index_is_carried` (`0`, `164`, absent) |
| unknown elements skipped with subtrees: `extLst` in `<c>`, `conditionalFormatting` beside `sheetData` | `an_unknown_element_is_skipped_with_its_subtree` (both places in one fixture, with a `<v>` and a `<c>` inside that must not be read) |
| elements in another namespace | `a_cell_in_another_namespace_is_not_a_cell` |
| the strict main namespace | `the_other_main_namespace_spelling_is_read_too` (reads `MAIN[1]`, not a literal) |
| empty `<sheetData/>` gives no cells | `an_empty_sheet_data_gives_no_cells` (and no `<sheetData>` at all) |
| error: unknown cell type | `a_cell_type_this_layer_does_not_know_is_refused` |
| error: unknown formula type | `a_formula_type_this_layer_does_not_know_is_refused` |
| error: shared formula without `si` | `a_shared_formula_without_an_index_is_refused` (+ `a_shared_index_that_is_not_a_number_is_refused`) |
| error: a bad reference | `a_cell_reference_that_is_not_a1_is_refused` |
| error: a bad row number | `a_row_number_that_is_not_a_number_is_refused` |
| error: a bad style index | `a_style_index_that_is_not_a_number_is_refused` |
| error: the wrong root | `a_worksheet_root_of_the_wrong_name_is_refused`, `a_worksheet_root_in_another_namespace_is_refused` |
| error: an unknown entity | `an_unknown_entity_in_a_cell_is_refused` (in `<v>`, `<f>` and `<t>`) |
| error: non-UTF-8 | `a_worksheet_that_is_not_utf8_is_refused` |
| `resolve`: an index that is not a number | `resolving_an_index_that_is_not_a_number_is_refused` |
| `resolve`: out of range | `resolving_an_index_out_of_range_is_refused` (and the empty-table case) |
| `resolve`: in range | `resolving_a_shared_string_gives_the_item_it_names` (also holds the owned table across the streaming borrow) |
| `resolve`: a non-shared cell | `resolving_a_cell_of_another_kind_gives_its_own_value` (Number, inline, and no value at all) |
| `cells` then `write_to` is byte-identical | `reading_cells_then_writing_copies_the_package_byte_for_byte` |
| P1 ruling 3 on `<v>`/`<f>` | `an_element_inside_a_cells_value_is_refused` |
| ruling 8's `--cells` integration test | `tools/emend-copy/tests/null_rewrite.rs:1001` `cells_counts_every_cell_and_edits_nothing` |

Nothing on the list is missing. Beyond it the diff adds
`a_worksheet_cut_off_mid_cell_is_refused`, `the_iterator_is_spent_after_a_refusal`,
`cells_on_a_part_that_is_not_there_is_refused`, the extra `resolve` and root
cases, and the two-operations-at-once rows in
`the_operations_are_refused_unless_exactly_one_follows_the_paths`  -  all within the
spec's "Errors" and "Harness runs" clauses, none of them scope creep. The only
thing the spec did not ask for in P2 is `Cell::{number, boolean, error}` (C2-6).

I checked the integration test's asserted line by hand against its own fixture
(three cells in row 1, a shared master and follower and a `<c/>` in row 2):
`cells=6 number=4 shared=1 boolean=1 values=5 empty=1 formulas=2
shared_master=1 shared_follower=1 numbers_parsed=3 booleans_parsed=1 resolved=1
max_row=2 max_col=3 styled=1` is exactly right, and the same line appears from
the built binary at `negative.log:2166` (U1). `shared_strings`' `items=2
bytes=8` is right too (`"alpha"` 5 + `"a\nb"` 3).

## Does any test assert a POSITION or a MECHANISM rather than a PROPERTY?

I read all 71. **No.** Every assertion compares a returned value, field, kind,
reference, error message or whole printed line with what the fixture wrote.
Specifically:

- The two tests whose NAMES sound like mechanisms
  (`the_phonetic_skip_ends_at_its_own_end_tag`,
  `a_phonetic_subtree_inside_an_inline_string_is_skipped`) assert values (`["ab"]`,
  `Some("ab")`), and their fixtures put a `<t>y</t>` after a nested `<r>` so that a
  first-end-tag skip is distinguishable from a matched one. I confirmed by
  building both flag variants that the fixtures do distinguish them (C2-2).
- `a_second_call_gives_an_equal_table` has already been through this defect class
  (P1 ruling 9) and its comment now says what it does and does not pin. Correct
  as it stands.
- `a_formulas_other_attributes_are_not_modelled` asserts equality of the two
  `Formula` values, which is the property, not the absence of a field.
- `the_iterator_is_spent_after_a_refusal` asserts the observable sequence
  `Some(Err)`, `None`, `None` rather than the `done` flag.
- `an_unknown_element_is_skipped_with_its_subtree` asserts the references and the
  surviving value, not that a skip happened.

The M6 and M7 corrections: `test.log:2067-2101` prints the planting, the run
(exit 101) and the assertion output verbatim, so both bite claims in the report
are measurements. I verified the arithmetic behind M6 independently (base-26
values above) and I agree with M7's corrected expectation: `specs/0010`, "Module
`cells`", derives a reference-less `<c>`'s **column** from "the previous cell's
column plus one within its row" and its **row** from the enclosing `<row>`'s `r`,
so `["B7", "C1"]` is what the spec implies and `C7` would require the row to come
from the previous cell, which the spec never says. The report is right to flag it
for planning, and right about which way it reads.

## The design clauses

- **`u64` where a count or offset could exceed 32 bits.** Yes. `CellRef::parse`
  accumulates in `u64` with a bound check per step (`cells.rs:283-296`); row,
  column, style and `si` are `u32` because the format says so; all 23 tool
  counters are `u64` (`main.rs:471-495`) and `max_row`/`max_col` widen through
  `u64::from`. `SharedStrings::len` returns a saturating `u32` and `get` takes
  `u32`. No byte offset is introduced in this module at all. Two cosmetic notes:
  `main.rs:557` uses `parts.len() as u64` where the crate elsewhere prefers
  `u64::try_from` (lossless either way), and `cells.rs:311` casts `(col-1)%26`
  with `as u8` (in range by construction). The `usize` values in
  `SharedStrings::read`, `Cells::new` and `Error::Xml` are zip entry indexes, not
  counts, and follow the existing convention.
- **Streams rather than buffering a sheet.** Yes: `advance` returns one cell per
  call, nothing collects, nothing seeks, and the entry arrives as
  `Box<dyn Read>`. The qualification is C2-9: one value, formula or inline string
  is held whole.
- **`Cells` borrows the package for the life of the iteration.** Yes, and it is
  the signature that enforces it (`Package::cells(&mut self) -> Result<Cells<'_,
  R>, Error>`); the `PhantomData<&'p mut Package<R>>` documents whose borrow it is
  and pins variance. `resolving_a_shared_string_gives_the_item_it_names` shows the
  owned table being held across that borrow, which is the reason
  `shared_strings` returns owned.

Housekeeping I ran myself at HEAD: `cargo fmt --all -- --check` exit 0;
`cargo clippy --workspace --all-targets` exit 0; all three commit messages are
ASCII and end with the `Co-Authored-By` line and nothing else; `da20c6b` contains
no mention of `Cell`, `CellRef`, `Cells` or `FormulaKind`, so "the P1 rulings, and
nothing of the cell reader" holds.

---

# Question 2, item by item

| spec requirement | verdict |
| --- | --- |
| first lines of every log | present in all five (command / about / phase / SERIALIZED / key-not-set or environment), **except that `gate.log`'s "about" line is the forbidden double print  -  C2-1** |
| the phase commit given to the script and compared with HEAD, with a control | four of five; **`gate.log` fails  -  C2-1** |
| the script printed into each log | yes, all five print `sed 's/^/    /' "$0"` |
| every compound check in a named function | yes. I grepped all five logs for a helper call carrying a second condition (`&&`, `||`, `-a`, `-o`, a leading `!`): the only hits are `$(is [ ! -e ... ])` forms where the `!` is `test`'s own, which evaluate correctly. `gate.log`'s `ctl()` uses the single-condition `[ ... ] && echo || echo` idiom. `structure_sound`, `commit_class`, `entry_clean`, `passed_matches`, `own_un_ok`, `names_no_office_file`, `is_head`, `full_sha` are all named. |
| every counting form a named function with a positive control | yes: all five logs source `scripts/counting.sh`, run `counting_controls`, and fail on `counting_controls_ok != yes` (`gate.log:127-129`, `test.log:133-135`, `negative.log:180-182`, `run.log:780-782`, `scan.log:302-304`) |
| scan log written LAST, hashing every committed sibling | yes for the version given: `scan.log:1063-1132` hashes the four sibling logs and the two records; "check answers scanned: 0" marks this as the pre-check run, as its own header says it would. The final run must hash the two answers as well. |
| scan scans its own source with the pre-P6d forms before printing itself, with a control | yes, `scan.log:16-27`: count=0 over its own source, then count=1 over a copy with one concatenated shape appended |
| the three forbidden-form greps, each with a fixture and a control, over every printed script including the scan's own source | yes, `scan.log:1156-1181`: the three patterns are assembled at run time, each has a matching fixture (count=1) and a named-function control (count=0), plus six cross controls, and they run over the four sibling logs, the scan script's own source and the two records (7 files, all zero). I checked the patterns catch the real shapes and do not false-positive on `$(is [ ! -e f ])`. |
| every sha256 printed by a command over a file that exists at run time | yes. `grep -nE '\b[0-9a-f]{64}\b'` over the five logs finds no hash outside printed command output. I re-verified every one I could: the four sibling logs and two records match `scan.log`'s table byte for byte and count for count; the four binaries match across `gate.log:1021-1024`, `negative.log:1598-1601` and `run.log:1711-1714` and match the files on disk now; and all three superseded attempts still exist and re-hash to their cited values (`fbdae014... 58764`, `039f7c97... 59231`, `4bf8cd44... 129734`), so planning can repeat the check. |
| every claim that a test bites is a printed measurement in `test.log` | seven are (`test.log:1947-2108`, each with the planted text, the exit status, the failing names and the `left`/`right` values, plus two "still passes, which is the point of it" negative controls and an unmutated-worktree control at 249 + 14 passing). **Two are not  -  C2-2.** |

**Four logs, not five or six.** Consistent with the spec and with the diff, and I
checked it myself rather than taking the log's word:

- No `self-test.log`: `specs/0001`'s new rule ties the self-test to a gate-file
  change, and `specs/0010`, "Phases", does not list one at all. The phase diff is
  `ROADMAP.md`, `crates/emend/src/cells.rs`, `crates/emend/src/cells/tests.rs`,
  `crates/emend/src/package.rs`, `specs/0001-bootstrap.md`,
  `specs/0010-reading-cells.md`, `tools/emend-copy/src/main.rs`,
  `tools/emend-copy/tests/null_rewrite.rs`  -  eight files, not one of which is a
  gate file under any reading. `gate.log:788-814` lists ten gate-file names,
  scores each against the diff, asserts the count is zero, and controls the search
  form with a name taken from the diff itself (`ROADMAP.md`, found once). The
  assertion is sound and its conclusion is true. `gate.log:816-822` and
  `scan.log:1109` also assert that no `self-test.log` is present, which is the
  right second half.
- No `size.log`: `specs/0010`, "P3", carries the size record and the probe. Right.

**`run.log`'s finding P2-F1 is a measurement, not a story  -  with one caveat.**

- The figures printed support the claim. `values` 2089191 and `empty` 5410 are
  printed per target (`run.log:1888-1893`, `:2077-2082`), asserted against the
  departure constants AND asserted to differ from the spec's figures  -  so if a
  later run agreed with the spec, the log would fail rather than quietly pass
  (`run.log:1170-1176`). The composition comes from the independent reader, not
  the library: `values_text=2089124`, `values_empty=67`,
  `neither_with_children=0`, `t_n=1302755`, `t_absent=8233`
  (`run.log:2145-2164`).
- The arithmetic holds: 2089124 + 67 = 2089191 = `values`; 2089191 + 5410 =
  2094601 = `cells`; 2089138 + 5463 = 2094601 too, so both partitions are
  self-consistent and no cell is lost; 1302755 + 8233 = 1310988 = `number`. Each
  of those identities is printed and asserted (`run.log:2156-2161`,
  `:1918-1921`).
- The cause is measured where it can be. The spec's parenthetical posits 53 cells
  "with neither `<v>` nor `<f>`"; the independent reader counts zero cells that
  hold neither while holding any child element at all, and finds the same +53 in
  `values_text`, which is one finding and not two. The spec's own "Facts planning
  checked" arithmetic corroborates it independently of this run: 5410 + 2089071 +
  67 = 2094548 = 2094601 - 53, so the 53 were a **residual** planning's
  element-shape regex could not classify, and the "Harness runs" parenthetical
  assigned them to a class rather than measuring them. What the log does **not**
  measure is *why* the earlier count missed 53 `<v>` elements  -  and it does not
  claim to.
- **Is anything in the reader wrong in a way that would produce exactly +53?** I
  looked hard and found no mechanism. The reader sets `value` only from a direct
  `<v>` or `<is>` child of the `<c>` (`cells.rs:665-690`); a `<v>` inside a skipped
  subtree cannot reach it (I verified with a fixture holding `<v>999</v>` inside an
  `extLst` inside a `<c>`: the value stayed `1`); a `<v></v>` pair yields
  `Some("")`, which would land in `values_empty` and so would have moved the 67,
  which it does not; and the expat route, which decides "text vs empty" by
  `child.text or ""`, agrees key for key on all 27 workbooks and on both targets.
  My judgement: **P2-F1 is a finding about the spec's figure, not about the
  reader.** The caveat is C2-5(a): the `numbers_parsed` paragraph of the finding
  states more than the identity pins.

Other things I checked in the logs and found sound: the two-reader agreement is
on the **whole** 23-key line per workbook (`run.log:1121`), the four control
fixtures include the `<c>`-in-`extLst` and `<row>`-in-`conditionalFormatting`
package on which both routes say two cells where four `<c>` elements exist
(`run.log:1698-1701`, `:1743`), the acceptance figures are 870 of 870 entries
`identical` with zero flags on each target and 27 of 27 for
`files_identical`/`order_preserved`/`lo`/`lo_in` (`run.log:1787-1797`,
`:1976-1986`), the two targets' cells files are compared with `cmp`
(`run.log:1255-1256`), and `negative.log` proves the seven fatal cell-reader
messages, the two P1 messages raised from a `<v>` this time, and three
unresolved-index cases that exit 0 and count (`negative.log:2176-2262`), each with
"output present: no" and a temporaries count  -  which is more than P2's spec
section asked for.

---

# Question 3: the report's numbers and quoted strings

Every figure I could trace, and where:

| report figure or quote | primary |
| --- | --- |
| `1f4481d`, `da20c6b`, `6c27167` and their subjects | `git log --reverse --format='%H %s' 1f4481d^..HEAD` |
| eight files, 2383 insertions, 107 deletions | `git diff --stat 1f4481d^..HEAD` (I ran it); `gate.log:796-805` lists the same eight files with sha256 and byte counts, and "files in the diff: 8" |
| no gate file in the diff | `gate.log:809-814`; verified against the diff myself |
| nine messages added to the catalogue | `git diff da20c6b..6c27167 -- crates/emend/src/package.rs`  -  exactly nine catalogue bullets; a tenth ("a text element holds an element") came in `da20c6b` |
| `the_catalogue_and_the_source_hold_the_same_messages` still passes | `test.log:1063` (host), `:1428` (i686) |
| M6 `left Some(CellRef { row: 1, col: 2147483647 })` / `right ... 4294967295` | `test.log:2076-2077`, verbatim |
| M6 `FXSHRXW` is `i32::MAX`; `MWLQKWU`/`MWLQKWV` the real pair | `test.log:1935-1938` (the planted strings) + my own base-26 recomputation |
| M7 `left ["B7", "C1"]` / `right ["B7", "C7"]` | `test.log:2096-2097`, verbatim |
| the seven mutations and what each breaks | `test.log:1871-1946` (the applier) and `:1947-2108` (the runs) |
| both mutation runs and the control use `--no-fail-fast`; attempt 1 did not | `test.log:1950` and the rerun header `test.log:4-7`; the attempt is hashed `4bf8cd44...`, 129734 bytes, and the file still exists and re-hashes to that |
| 333 host, 333 i686, same names | `test.log:1601-1607`, `:2111-2113` |
| 280 in P1's log | `test.log:1613` |
| 249 library tests | `test.log:1602`, and the control run at `test.log:2104` |
| cells 71, P1's 20 | `test.log:1800-1802` |
| tool 14, P1's 12 | `test.log:1822-1826`; the two new names are in the list at `:1810,1812` |
| previous figures read from the committed log, not typed | `test.log:1614`, `:1802`, `:1825` |
| 23 keys in the spec's order | `main.rs:498-522` against `specs/0010`, "Harness runs"; `negative.log:2172` counts 23 key=number fields with a control |
| a `resolve` failure counted, never fatal | `negative.log:2241-2262` (U11-U13), exit 0 each, with a control |
| any other parse error is exit 2, no output file | `negative.log:2176-2240` (U2-U10) |
| 458 seconds wall | `run.log:2176` |
| 870 entries identical, 27 of 27 four times, both targets | `run.log:1787-1797`, `:1976-1986` |
| the 18 agreeing key totals | `run.log:1869-1916` (host), `:2058-2105` (i686) |
| max_row 130001, max_col 28, numbers_parsed 1305578 | `run.log:1913-1916` |
| values 2089191 / spec 2089138; empty 5410 / spec 5463; 53 | `run.log:1888-1893` |
| values_text 2089124 / spec 2089071; values_empty 67; zero "neither with children" | `run.log:2152-2165` |
| t_n 1302755, t_absent 8233 | `run.log:2148-2151` |
| 1310988 - 1305578 = 5410 | `run.log:1926` prints the identity; **the interpretation is not measured  -  C2-5(a)** |
| the four fixtures, "four `<c>` elements, two cells", both routes said two | `run.log:1690-1701` and `:1743`; "written into the script before either route saw them" is true of the first two only |
| the record pair names | the files exist; `run.log:2168-2174` hashes them |
| the four binary hashes cited rather than rebuilt | `gate.log:1021-1024`, `negative.log:1598-1601`, `run.log:1711-1714`; identical to the files on disk |
| every hash printed over a file that exists | verified, see Question 2 |
| gate rerun twice: a P1 header, then a doc-commit pattern matching outside the range plus a comment claiming two gate files changed | `gate.log:4-9`, with both attempts hashed and both files present |
| "Twenty-one of the twenty-three agree exactly" | **appears only in `records/m1p3-p2-...md`; contradicted by `run.log`  -  C2-7** |
| "Three logs were rerun" | **nowhere; two were  -  C2-11** |
| "the cell tests that reach them" | **nowhere; no cells test calls `number`, `boolean` or `error`  -  C2-6** |
| "`specs/0001` says the same of a failing run" | **`specs/0001:83-87` says record, fix, and re-run with the fix named  -  C2-11** |

Two report claims that are not numbers but that I checked and found **true**:
"they are byte-identical across the gate's `cargo clean` rebuild and every run"
(the final `gate.log` ran at 19:48 with a `cargo clean` first, after
`negative.log` at 19:18 and `run.log` at 19:39 had already cited those hashes, and
produced the same four hashes  -  so the citation holds and the build is
reproducible), and "the unmutated worktree passes as the control"
(`test.log:2103-2108`).

---

# What I did not check

- Nothing under the corpus directory, and I did not run the corpus runner, so
  every corpus figure in this answer is quoted from `run.log` and the record pair,
  never re-measured. The two-reader agreement and the 27 per-workbook lines are
  taken as printed.
- The final `scan.log` (the one that will hash the check answers): I judged the
  first run I was given, which says so of itself.
- Whether the ten names in `gate.log`'s gate-file list are exhaustive of "gate
  files" as `specs/0001` and `specs/0007` define them. I verified instead that no
  file in the phase diff is plausibly a gate file, which is what the conclusion
  needs.
- `specs/0001`'s rules other than the four this phase added, and the earlier
  phases' logs.
