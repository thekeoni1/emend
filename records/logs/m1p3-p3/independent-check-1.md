# Independent check 1: emend M1 part 3 P3

Phase commit 25750836734736cbca6701320b59361ad157400b (2575083). I read only
the files named in my instructions, ran git, grep, `cargo test --workspace`,
`cargo fmt --check --all` and `cargo clippy --workspace --all-targets -D
warnings` on the host, and one throwaway `rustc` program in my scratch
directory to measure what `u32::from_str` does. I read nothing under the
corpus directory and ran no corpus pass. I changed no file in the repository
and committed nothing.

Note on `scan.log`: I was told it is the run made before the check answers
exist, so its one failing structural check is their absence. I judged
everything else in it normally, and I say below what I think planning still
has to re-verify because of it.

---

## 1. Does the diff do what the spec says, and nothing else?

The diff is `git diff 79f1471^..HEAD`: 8 files, 770 insertions, 56 deletions,
in four commits (79f1471 docs, e127fc2 rulings, 8c8a0ca typed views and
Limits, 2575083 the probe). Every file in it is one the spec's P3 section or
the P2 rulings reach. Nothing else is touched: no gate file, nothing under
`tools/emend-copy`, no new dependency (`tools/size-probe/Cargo.toml` gains
only a `[[bin]]` stanza and two comment lines naming the new probe).

### 1a. `Cell::{number, boolean, error}` with tests

Every case the spec's P3 sentence names is present, and each is a real
assertion rather than a gesture:

| spec case | test | where |
| --- | --- | --- |
| `1.10` | `a_numbers_text_is_read_as_an_f64` | `crates/emend/src/cells/tests.rs:1459` (`assert_eq!(cells[0].number(), Some(1.1))`, 1471) |
| `-0` | same test, `crates/emend/src/cells/tests.rs:1474-1476` | asserted through `is_sign_negative`, not equality |
| `1E+3` | same test, `crates/emend/src/cells/tests.rs:1472` | |
| `inf` and `nan` "as the standard library parses them" | `the_parse_is_the_standard_librarys_and_wider_than_the_format` | `crates/emend/src/cells/tests.rs:1477`; `inf`, `INF`, `infinity`, `1e309` positive-infinite (1487-1492), `-inf`, `-1e309` negative-infinite (1493-1498), `nan`, `NaN` via `is_nan` (1499-1505), leading `+` (1508-1510) |
| a non-number `Number` cell `None` | `a_number_cell_whose_text_is_not_a_number_has_none` | `crates/emend/src/cells/tests.rs:1512`; `x`, `" 1"`, `"1 "`, `1,5`, `0x1`, `TRUE`, empty value, and a `<c r="A1"/>` with no value at all |
| `1`, `0`, `TRUE` as booleans | `a_booleans_text_is_read_as_a_bool` | `crates/emend/src/cells/tests.rs:1527`; `1` true, `0` false, `TRUE` None, plus `true`, `-1`, `2`, `01`, empty and no value |
| an error value | `an_error_cells_text_is_its_error` | `crates/emend/src/cells/tests.rs:1550`; `#REF!`, `#DIV/0!`, `#SOMETHING_NEW!`, and an error cell with no value |
| every view answers only for its own kind | `each_typed_view_answers_only_for_its_own_kind` | `crates/emend/src/cells/tests.rs:1565`; seven kinds, three questions each, one table |

The spec's phrase "`1`, `0`, `TRUE` as `None`" reads oddly against the design
section, which has `boolean()` give `Some(true)` for `1` and `Some(false)` for
`0`. Both readings are covered: `a_booleans_text_is_read_as_a_bool` pins the
design's answers, and `each_typed_view_answers_only_for_its_own_kind` pins
`number()` and `error()` as `None` on the Boolean cell. No defect; planning
may want the sentence tightened.

The doc comment on `Cell::number` (`crates/emend/src/cells.rs:470-496`) does
state what the parse takes, in five bullets, and every case in those bullets
is measured somewhere. **But the comment attributes all five to one test and
that attribution is wrong for two of them.** Lines 479-482 read "Each case
below is measured in `cells::tests`, by
`the_parse_is_the_standard_librarys_and_wider_than_the_format`". That test
(`crates/emend/src/cells/tests.rs:1477-1511`) does not touch `" 1"`, `"1 "`,
an empty value, `1.10` or `-0`: bullet 4 (`crates/emend/src/cells.rs:493-494`)
is measured by `a_number_cell_whose_text_is_not_a_number_has_none`
(tests.rs:1512) and bullet 5 (cells.rs:495-496) by
`a_numbers_text_is_read_as_an_f64` (tests.rs:1459). 8c8a0ca's commit message
repeats the error ("each line of it measured by the test above"). The cases
are measured; the pointer is false. This is the same class as the defect
`specs/0001` was amended for: a comment naming a measurement that is not
where it says it is.

One softer over-reach in the same comment: bullet 1 says the infinities and
NaNs parse "in any case", where the test measures three case spellings of
inf/infinity and two of nan. The statement is true of `f64::from_str`, but
"in any case" is wider than what is measured.

### 1b. The catalogue and the Limits sections

`cells.rs`'s Limits section now carries six entries. The whole-table entry
(`crates/emend/src/cells.rs:73-85`) is P1's and pre-dates this phase; this
phase added the five under "What this module does NOT do"
(`crates/emend/src/cells.rs:92-119`). Taking them one at a time:

- no SEEKING (95-98): a decision with a reason (the format keeps no index of a
  sheet's cells) and a stated cost (the caller pays the walk). Not a
  restatement. True of the code: `Cells` is an `Iterator` over `xml::Stream`
  and never seeks inside the part.
- no EVALUATION (99-103): a decision with a reason, and it states the
  consequence a restatement would leave out ("a stale cached value is reported
  as it was found"). True.
- no DATE CONVERSION (104-108): a decision with a reason (1900/1904 base and a
  calendar, part 5, the dependency rule). True, and measured:
  `each_typed_view_answers_only_for_its_own_kind` asserts
  `cells[5].value.as_deref() == Some("2026-09-18")` at tests.rs:1600.
- no LOOKUP of a style (109-111): a decision with a reason. True; `style` is
  `Option<u32>` and no styles part is read.
- unmodelled attributes (112-116): the list matches the code
  (`read_formula` reads only `t`, `si`, `ref`; `read_opening` only `r`, `s`,
  `t`; `row_number` only `r`; `count`/`uniqueCount` unread). True. The clause
  "a part 4 rewrite copies them raw" is a forward promise, but the spec makes
  the same one, so it is not the implementer's invention.
- no SHEET-SIZE limit (117-119): a decision with a reason. True;
  `CellRef::parse` takes `XFD1048576` and beyond, up to `u32`.

`package.rs`'s three new entries (`crates/emend/src/package.rs:149-165`) are
each a decision with a reason and each true of the code:
`shared_strings` parses on every call and returns owned
(`crates/emend/src/package.rs:621-629`, returns `SharedStrings` by value);
`cells` borrows mutably for the iteration
(`crates/emend/src/package.rs:654`, `Cells<'_, R>` with
`package: PhantomData<&'p mut Package<R>>` at
`crates/emend/src/cells.rs:588`); neither records an edit, which the P2
integration test `cells_counts_every_cell_and_edits_nothing` and
`negative.log`'s U1 ("U1 byte-identical to the input: yes") measure.

I found no false Limits entry.

The catalogue additions (`crates/emend/src/package.rs:89-113`) name seven test
references across five messages. I checked every name exists in
`crates/emend/src/cells/tests.rs` and that each named test really asserts the
message it is listed under; all seven do. Note for planning: 8c8a0ca's own
message says the set-equality test "cannot see a test name, which is why the
names are maintained by hand", so nothing in the suite would catch a wrong
name here. It happens to be right.

### 1c. `probe-emend-cells-xml` and the float-parser variant

`tools/size-probe/src/bin/probe-emend-cells-xml.rs` links what the spec's
sentence describes. I diffed it against `probe-emend-package-xml.rs`: the
package probe's body is carried over unchanged (open, workbook and sheet
count, the chain recipe of three calls, the two additions, the replacement
through `Stream`, the element count, the write), and the additions are the
shared strings read (line 153), `table.len()` (154) and `read_cells` (155,
183-203), which reads every cell, sums `number()` and totals `resolve`d text.
That is "the cell count, the sum of `number()` over the cells and the total
length of `resolve`d text", so no path is dead: the reference and the style
are needed to yield a cell at all, `number()` carries `str::parse::<f64>`, and
`resolve` carries the table lookup. Every name and part comes from
`args_os` (lines 33-43), so nothing is constant-folded away.

The documented departure from the spec's ORDER does not change what is
linked. The spec puts the reads after the write; the probe puts them before
it (`tools/size-probe/src/bin/probe-emend-cells-xml.rs:144-156`, and the
module head at 11-14) because `Package::write_to` takes the package by value.
The set of called functions is identical, and the reads run against a package
with a replacement, an addition and a removal already recorded, so if
anything more code is reachable, not less. I checked that
`probe-emend-package-xml.rs:118-126` has the element count before the
replacement in exactly the same place, so the two probes differ only by the
added block.

The variant is what the spec asks for: built in a scratch clone, one edit
only (the `number()` arm sums the values' byte lengths), not committed, and
its build, figures and its own run are printed
(`records/logs/m1p3-p3/size.log:1862-1885`). Its run prints `sum 9` where the
probe prints `sum 3.75` and agrees on every other figure, which is the
evidence that the edit is the one claimed.

One observation, not in the report: the probe uses `THEME[0]`
(`tools/size-probe/src/bin/probe-emend-cells-xml.rs:134`), a positional index
into a two-element constant. It is copied from `probe-emend-package-xml` and
`THEME` is a relationship-type list, not a namespace, and this is a tool
rather than a test assertion, so I do not think it falls in the class ruling 8
is about. Recording it so planning can decide.

### 1d. The size record and `size.log`

Every figure in `records/size-m1p3-2026-09-18-2575083.md` is printed by
`records/logs/m1p3-p3/size.log`. I verified this mechanically: I extracted the
indented block under `size.log`'s "## the size record, written to <scratch>"
heading, stripped the four-space indent, and diffed it against the committed
record. It is byte-identical (7068 bytes). So there is no figure in the record
that the log did not print.

I re-did the arithmetic from the probe table:

- 780564 - 533216 = 247348 (budget, sections); 789028 - 539172 = 249856
  (file); 596307 - 411155 = 185152 (.text). All three as recorded.
- 780564 - 606004 = 174560; 789028 - 612900 = 176128; 596307 - 461107 =
  135200. As recorded.
- 780564 - 717228 = 63336; 789028 - 723492 = 65536; 596307 - 551795 = 44512.
  As recorded.
- 780564 - 761392 = 19172; 789028 - 768548 = 20480; 596307 - 590003 = 6304.
  As recorded.
- 393216 - 247348 = 145868. As recorded.

The seven earlier probes are re-measured against part 2's record by the byte
classifier, and I checked part 2's record myself: every "Part 2 file" and
"Part 2 sections" cell in the P3 record's comparison table equals
`records/size-m1p2-2026-09-18-14c2e14.md:24-31`. Three probes are identical;
four moved 32 section bytes and 0 file bytes and are classified "code", with
`size.log` printing per-probe differing-byte counts, slot deltas and the
commits behind them, and asserting "if the verdict is code, a commit that
could have caused it is named above" per probe
(`records/logs/m1p3-p3/size.log:1770-1789`). The classifier has both controls
in the same command form (a file against itself, and a copy with one byte
flipped in `.text`, `size.log:506, 531-532`).

**On the count of earlier probes, the log is honest.** The spec's P3 section
says "re-measuring the eight earlier probes against part 2's record". Part 2's
record tabulates seven; the eighth is the one part 3 adds. `size.log`'s own
header says so in capitals before any measurement
(`records/logs/m1p3-p3/size.log:39-46`: "The spec says 're-measuring the eight
earlier probes'. There are SEVEN earlier probes ... This log measures all
eight and compares the seven, and the count is printed rather than asserted
from the sentence"), the record repeats it
(`records/size-m1p3-2026-09-18-2575083.md:40-42`), and the log prints "of the
seven earlier probes, 3 keep every part 2 figure" rather than a seven-against-
eight comparison. I count seven myself and agree with the log and with the
report's finding. The spec's wording wants correcting.

**Two arithmetic presentation problems, neither of which the report names.**

1. The budget paragraph invites a subtraction that does not come out. The
   record and the log say the ruling recorded 183980 spent by parts 1 and 2
   and 209236 remaining, that the remaining figure is now 145868, and that
   part 3's own share is 63336
   (`records/size-m1p3-2026-09-18-2575083.md:72-80`;
   `records/logs/m1p3-p3/size.log:1799-1803`). 209236 - 63336 = 145900, not
   145868. The 32 bytes are real and explained elsewhere in the same record:
   `probe-emend-package-xml` grew 32 section bytes since part 2, so parts 1
   and 2 now cost 184012 over `probe-zip-xml`, and 184012 + 63336 = 247348.
   Nothing is mis-measured; the two paragraphs are just never reconciled, and
   a reader who subtracts gets a different number. The report repeats the
   unreconciled pair verbatim.
2. `records/size-m1p3-2026-09-18-2575083.md:74-76` says the budget figure
   "leaves 145868 section bytes and 143360 file bytes under it". The ceiling
   (393216) is a section-byte ceiling; 143360 is 393216 minus the FILE delta,
   which mixes two units and is not headroom in any measured sense. The log
   prints the same pair (`size.log:1788`). The check
   `budget_under` compares the file delta against the section ceiling too
   (`size.log:614`). Harmless at this margin; worth fixing before the margin
   matters.

### 1e. `--cells` gains nothing

Confirmed independently. `git diff --stat 79f1471^..HEAD` lists eight files
and none is under `tools/emend-copy`: the tool's source and its integration
tests are untouched this phase. `test.log` measures the same from the other
end: the tool's `null_rewrite` binary runs 14 tests, the same 14 names as at
P2, read out of the committed P2 log
(`records/logs/m1p3-p3/test.log:2098-2117`).

### 1f. Rulings 2 and 8

**Ruling 2, one helper.** `cells::decimal`
(`crates/emend/src/cells.rs:858-867`) is called at exactly four sites, which
are the four the ruling names: the shared string index in
`SharedStrings::resolve` (cells.rs:269), a cell's `s` (908), a row's `r`
(942), a formula's `si` (981). I grepped `.parse()` across `cells.rs` and
found only two occurrences: inside `decimal` itself (866) and in
`Cell::number` (501, an `f64`). So no `u32` is parsed out of an attribute by
anything but `decimal`.

But the report's sentence is too strong. It says "`cells::decimal` is the only
thing that turns an attribute into a `u32` now". `CellRef::parse`
(`crates/emend/src/cells.rs:304-340`) also turns attribute bytes into two
`u32`s: it reads the digits of a `<c>`'s `r` into a row number with its own
loop and its own leading-zero rule (316, 329-335). The ruling's own wording -
"one helper serves the row `r`, the cell `s`, the formula `si` and the shared
string index" - is satisfied exactly, and `CellRef::parse` happens to apply
the same two rules, so the behavioural property holds. The report should say
what the commit message says (every number this layer takes out of an
attribute AS A DECIMAL TOKEN) rather than "the only thing".

A row number of 0 is refused: `crates/emend/src/cells.rs:942-944` filters
`*number > 0` and returns `Error::Package("a row number is not a number")`,
which is the message an unreadable row number already had. Pinned by
`a_row_number_of_zero_is_refused` (tests.rs:1322) and measured through the
built binary as U15 (`records/logs/m1p3-p3/negative.log:2342-2346`).

`CellRef::parse(a1(x)) == Some(x)` is asserted as a property over what the
reader emits, not over a list of spellings:
`every_reference_the_reader_emits_parses_back` (tests.rs:1336-1367) reads a
sheet holding an explicit `r`, a cell with no `r`, a row with no `r`, the
largest row and column the format writes, and the saturated column, then loops
over every cell and asserts `CellRef::parse(cell.reference.a1()) ==
Some(cell.reference)`. That is a property over the reader's output. The two
trailing assertions (tests.rs:1365-1366) index `cells[4]` and `cells[5]`,
which is a position in the FIXTURE's cell order, not in any implementation
structure; I do not count that as the position-versus-property defect.

**Ruling 8, "needed no change", is true of the code as it stands.** I checked
`git show 52ef4e5 -- crates/emend/src/cells/tests.rs`: it already replaced
`MAIN[1]` with the strict spelling written out as a `const STRICT` and
asserted to be one of `MAIN`'s two entries. In the current tree
`grep -rn "MAIN\[" crates/ tools/` returns exactly one line,
`crates/emend/src/cells/tests.rs:977`, and that is inside the comment that
explains why the position is not used. So there is no positional use of a
namespace constant left, and 8c8a0ca's only change for ruling 8 is the comment
naming C12 and the ruling (tests.rs:975-982). Nothing was invented to have
something to show.

### 1g. Position or mechanism instead of property

**I found no test in this diff that asserts a position or a mechanism instead
of a property.** Working through the eleven added tests: the five ruling tests
assert refusals by their catalogued message text, which is the contract;
`zero_is_a_number_where_zero_is_a_legal_index` asserts the values read;
`every_reference_the_reader_emits_parses_back` asserts a round-trip property
over the reader's whole output; the five typed-view tests assert answers, and
the two places where an equality would have passed wrongly use predicates
instead (`is_sign_negative` at tests.rs:1476, `is_nan` at tests.rs:1503). The
only positional dependence anywhere is on the order of cells in the test's own
fixture (`cells[4]`, `cells[5]`, and the seven-row answers table at
tests.rs:1580-1592), which is a property of the fixture the test wrote, not of
the implementation. The one removed test
(`a_row_number_this_layer_accepts_is_not_always_one_a1_form_parses`) was a
"today's answer" pin and is correctly replaced by the refusal the ruling asks
for.

### 1h. Claims no measurement supports

Three, all in the second recorded class.

**(i) `u32::from_str` does not admit a leading `-`.** The doc comment on
`decimal` says "its grammar admits a leading `+` or `-`, and it reads any
number of leading zeros" (`crates/emend/src/cells.rs:848-850`), and e127fc2's
commit message says "u32::from_str alone admitted a leading + or - and any
number of leading zeros". I measured this on the pinned toolchain
(`rustc +1.98.1`, a five-line program in my scratch directory):

    "+1"  -> Ok(1)
    "-1"  -> Err(ParseIntError { kind: InvalidDigit })
    "-0"  -> Err(ParseIntError { kind: InvalidDigit })
    "007" -> Ok(7)
    "+0"  -> Ok(0)

`u32::from_str` accepts `+` and leading zeros and REFUSES `-`. The `-` half of
both sentences is false. It is also unmeasured: mutation M1 plants the old
`decimal` and a witness asserting only `s="+1"`, `si="+0"` and `r="007"` read
as 1, 0 and 7 (`records/logs/m1p3-p3/test.log:2188-2213`, 2305-2320). Nothing
in any log measures the `-` claim, because it cannot be measured - it is not
true. The sentence should say "a leading `+`" and stop, or cite the documented
grammar of integer `FromStr` (which does spell `Sign ::= '+' | '-'`) while
saying that an unsigned parse rejects the minus.

**(ii) The eight spellings are not all spellings `from_str` would have taken.**
`crates/emend/src/cells/tests.rs:1373-1374` says "the spellings are what
`u32::from_str` would have taken and the format does not write", and the list
at 1379 is `["+1", "007", "0x1", " 1", "1 ", "1.0", "-1", ""]`. By the
measurement above, `from_str` would have taken two of the eight (`+1`, `007`)
and refused the other six. Those six were already refused before ruling 2, so
for them the test asserts behaviour this phase did not change - which is fine
as coverage, but the comment's account of why they are there is false. Note
that M1, the mutation that plants the old code, fails
`a_number_this_layer_reads_is_digits_alone` at tests.rs:587 (the
`assert_package` helper) on the FIRST spelling, so what M1 measures is `+1`,
not the list.

**(iii) The report claims a verification that no log prints.** The report's
paragraph on the doc commit says the three planning files were checked
"by command before it was made ... `git diff --stat` over the three paths, and
`sha256sum` over each, printed in the commit's own verification before it was
written". No log in `records/logs/m1p3-p3/` prints that pre-commit check. What
the logs do print is the hashes of the three files AT HEAD:
`records/logs/m1p3-p3/gate.log:822` (ROADMAP.md, sha256 93d0244d..., 24997
bytes) and `records/logs/m1p3-p3/scan.log:978, 1018, 1028` (ROADMAP.md,
specs/0001-bootstrap.md 0742e78a..., specs/0010-reading-cells.md 0a4ad183...),
plus `gate.log:801`'s classification of 79f1471 as a docs-only commit. Those
lines let planning check the files against the sha256 it recorded before
sending them, which is what matters. But by `specs/0001` ("Nothing is called
measured unless the log prints the measurement") the report's sentence should
point at those lines or drop the claim about a pre-commit verification.

Everything else I spot-checked in the comments and messages is supported. In
particular 8c8a0ca's "-0.0 == 0.0 is true and an equality would have passed
for either sign" is measured by M6 against M7, and "NaN != NaN would have
failed a correct answer" by M8; see question 2.

### 1i. The design clauses

- **`u64` where a count or offset could exceed 32 bits.** Yes in the new code.
  The probe's every count is `u64` (`Counts` at
  `tools/size-probe/src/bin/probe-emend-cells-xml.rs:76-83`), with only
  widening casts from `usize` (`sheets` 90, `text` 199) and `u64::from` for
  the `u32` item count (154). `CellRef::parse` accumulates in `u64` with a
  bound check per step so nothing wraps on a 32-bit target
  (`crates/emend/src/cells.rs:322-335`). `decimal` returns `u32` and delegates
  the range check to `from_str`, which is right: these are format-bounded
  indices. No count is `usize` in the API; the only `usize` in `cells.rs` are
  the zip entry index carried into errors and two nesting depths
  (cells.rs:571, 1036, 1039, 1117), none of them a byte count.
- **The readers stream.** `Cells` holds one token and one `Cell`
  (`crates/emend/src/cells.rs:561-589`), is an `Iterator` over `xml::Stream`,
  and never buffers a sheet. The shared strings table is the one whole-part
  cost, which the Limits section says with the corpus figures
  (cells.rs:73-85). Unchanged by this phase and still true.
- **`Cells` borrows the package for the life of the iteration.**
  `Package::cells(&mut self) -> Result<Cells<'_, R>, Error>`
  (`crates/emend/src/package.rs:654`) plus
  `package: PhantomData<&'p mut Package<R>>` (`crates/emend/src/cells.rs:588`)
  make the borrow part of the type. The new `package.rs` Limits entry states
  the consequence (one sheet at a time, nothing else may be asked of the
  package until the reader drops).

---

## 2. Do the logs contain every line the spec requires?

### The required lines, one at a time

- **First lines, phase commit given to the script and compared with HEAD, with
  a control.** Present in all six logs: `gate.log:1-8`, `test.log:1-12`,
  `negative.log:1-11`, `run.log:1-10`, `size.log:1-8`, `scan.log:1-7`. Each
  prints the commit it was given AND `git rev-parse HEAD` on separate lines,
  the comparison, and controls refusing the null commit and a short sha of the
  real commit. Nothing is read from HEAD and printed twice.
- **The script printed into each log.** Yes: `gate.log:14`,
  `negative.log:13`, `test.log:14`, `run.log:84`, `size.log:19`,
  `scan.log:29`.
- **Every compound check in a named function.** `scan.log`'s grep over every
  printed script reports `chained=0 leading-bang=0 assignment=0` for all five
  siblings and for its own source (`scan.log:1243-1249`). The one exception
  this phase HAD is finding P3-F1, caught by that grep and fixed to
  `variant_built_clean` (`size.log:860-861`). Single-condition `$(yn [ ... ])`
  calls remain (for example `size.log:615-617`), which the rule permits.
- **Every counting form a named function with a positive control.** All six
  logs source `scripts/counting.sh` and print a `## counting controls` block:
  `gate.log:759`, `negative.log:1633`, `test.log:1050`, `run.log:1979`,
  `size.log:1245`, `scan.log:880`. `scan.log:906` asserts "every counting form
  behaves on a match, a miss and an unreadable file: yes".
- **The scan log written LAST.** Yes, by the run timestamps: gate 21:34:12,
  negative 21:37:38, test 21:51:27, run 22:13:21, size 22:33:08, scan
  22:34:54.
- **Hashing every committed sibling.** `scan.log:1117-1166` prints `wc -c` and
  `sha256sum` for the five siblings, and 1176-1204 for the three records. I
  ran `sha256sum` and `wc -c` over the eight files myself and every one of the
  eight hashes and byte counts matches. It does NOT yet hash the check
  answers - see the caveat below.
- **Scanning its own source with the pre-P6d forms before printing itself,
  with a control.** Yes: `scan.log:16-27`, count=0 on the source and count=1
  on a copy with a concatenated shape appended.
- **The three forbidden-form greps, each with a fixture and a control, over
  every printed script including the scan script's own source.** Yes:
  `scan.log:1231-1260`. The three patterns are assembled at run time (1232-
  1234), each form has a positive fixture and a named-function negative
  (1235-1240), and there are six cross-controls showing each form is blind to
  the other two shapes (1241-1246). Nine files go through all three, the scan
  script's own source included (1248).
- **Every sha256 printed by a command over a file that exists, superseded
  attempts included.** Yes. `size.log:11-14` hashes three superseded attempts
  by command; `test.log:5-6` one; `run.log:13-14` one; `negative.log:5-6` one.
  `run.log:1155-1182` reads the four release hashes out of `size.log` by
  `sed` at run time.
- **Every claim that a test bites is a printed measurement in `test.log`.**
  Yes for the eleven claims the mutation block covers (below). The two
  `u32::from_str` sentences in 1h(i) and 1h(ii) are not test-bite claims, but
  they are claims of the same family with no measurement behind them and one
  of them is false.

### Five logs and no `self-test.log`

The spec's "## Phases" names six log files for P3: `gate.log`, `test.log`,
`negative.log`, `run.log`, `scan.log`, and `size.log`. Six exist. The report's
"FIVE logs" counts the five siblings and treats `scan.log` separately (its own
table lists five logs plus three records, and the paragraph beneath explains
why `scan.log`'s hash cannot be in a file it writes). Read that way the
report is consistent; read as a count of logs, "five" is one short. I would
prefer "five siblings and the scan log written last".

**No `self-test.log` is due, and I checked the gate-file claim myself rather
than taking it from the log.** The phase diff's eight files are ROADMAP.md,
crates/emend/src/cells.rs, crates/emend/src/cells/tests.rs,
crates/emend/src/package.rs, specs/0001-bootstrap.md,
specs/0010-reading-cells.md, tools/size-probe/Cargo.toml and
tools/size-probe/src/bin/probe-emend-cells-xml.rs. None of them is one of the
ten gate files `specs/0001-bootstrap.md:113-117` lists. So the phase changed
no gate file, `gate.log` is required to list the gate files it checked against
the diff instead, and it does; `scan.log:1168-1172` measures the same thing
("gate files checked against the diff: 10; of them in it: 0"), with a positive
control that the same search finds a filename the diff DOES hold, and asserts
no self-test log is present in the log directory. Consistent with the spec and
with the diff.

### The caveat planning must not skip

The `scan.log` on disk reports `structural checks failed: 1` and
`RESULT: FAIL` (`scan.log:1264-1266`), because "check answers scanned: 0" and
"the check answers went through all three greps as prose: no"
(`scan.log:1175, 1254`). I was told this is the pre-answers run, and the log
itself says so (`scan.log:1110-1116`). Two consequences:

1. The `scan.log` that will be committed is NOT the one I read. Planning has
   to verify the final one from scratch: that it scans and hashes the check
   answers, that `RESULT` is PASS, and that its sibling table still equals
   `sha256sum` over the committed files.
2. **`scan.log:11` names the wrong phase.** It reads "phase: Milestone 1 part 3
   **P2** (specs/0010-reading-cells.md) scan.log", where every other log says
   P3 (`gate.log:9`, `test.log:3`, `negative.log:3`, `run.log:11`,
   `size.log:9`). The string is hard-coded in the printed script at
   `scan.log:249`, carried over from the P2 scan script that
   `scan.log:50` says this one derives from. Nothing measured is affected - the
   phase commit, the range, the doc commit and the base are all read from git
   and correct - but it is exactly the class of fault that made `run.log`
   attempt 1 and `size.log` attempt 2 be rerun ("one wrong string it wrote
   into the record"). It should be fixed in the rerun that adds the check
   answers, and the report does not mention it.

### The eleven mutations

`test.log:2180-2502`. The applier refuses any replacement whose text does not
occur exactly once (`test.log:2298-2301`), each mutation runs in a detached
worktree at the phase commit, the clean worktree at the same commit passes
(263 + 14 tests, `test.log:2493-2499`), and the restored worktree is measured
to hold neither planted witness (`test.log:2500`). Taking each against the
claim it exists for:

| # | claim it is there for | what the log measures | verdict |
| --- | --- | --- | --- |
| M1 | e127fc2: `from_str` read `s="+1"`, `si="+0"`, `r="007"` as 1, 0, 7 | old `decimal` planted; `a_number_this_layer_reads_is_digits_alone` fails; the witness PASSES | measures the claim, for `+` and leading zeros only (not the false `-` half) |
| M2 | that M1's witness measures the mutation, not itself | witness alone, unmutated code: witness FAILS at tests.rs:559 | sound, not a tautology |
| M3 | e127fc2: `r="0"` gave a cell whose `a1()` was `A0`, which `CellRef::parse` refuses | row filter removed; `a_row_number_of_zero_is_refused` fails; the witness (A0 emitted, `parse(b"A0") == None`) PASSES | measures the claim |
| M4 | that M3's witness measures the mutation | witness alone: FAILS | sound |
| M5 | tests.rs:1344-1348: the saturated pair round trips, and the round-trip test covers it | buffer 7 -> 6 letters; `every_reference_the_reader_emits_parses_back` fails at the SATURATED cell (`col: 279062207` against `4294967295`), and `a_cells_own_reference_wins_over_its_row` still passes | measures the claim |
| M6 | 8c8a0ca: a lost sign in `number()` would be caught | `a_numbers_text_is_read_as_an_f64` fails | measures the claim |
| M7 | 8c8a0ca: "an equality would have passed for either sign" | M6's code change PLUS the assertion rewritten as `assert_eq!(..., Some(-0.0))`: the tree PASSES (exit 0) | a real differential, not a tautology: same defect, two assertion forms, opposite outcomes |
| M8 | 8c8a0ca: "NaN != NaN would have failed a correct answer" | predicate rewritten as `assert_eq!(..., Some(f64::NAN))` on UNMUTATED code: test FAILS, printing `left: Some(NaN)` / `right: Some(NaN)` | sound and unusually legible; not a tautology, because it shows the alternative assertion form rejects a correct answer |
| M9 | 8c8a0ca: a view that lost its kind check "would answer for a neighbour", and the shared-string cell "would read as the number 0" | `number()`'s kind check removed; `each_typed_view_answers_only_for_its_own_kind` fails, and a separate grep finds `(Some(0.0), None, None)` in M9's output with a control that M2's unmutated run holds none | measures both halves of the claim |
| M10 | the same, for `boolean()` | kind check removed; same test fails, `Some(true)`/`Some(false)` leaking to five other kinds | measures the claim |
| M11 | the same, for `error()` | kind check removed; same test fails, every cell's text leaking out as an error | measures the claim |

On the three arrangements the prompt asks about:

- **The four that plant a test as well as a defect (M1, M3, and their
  companions M2, M4).** Not tautologies. A witness that passes under the
  mutation and fails without it is precisely the measurement that turns
  "`from_str` read `007` as 7" from a recollection into a fact, and M2/M4 are
  what stop the witness from being a test of itself. The pairing is the right
  shape.
- **The one expected to leave the tests PASSING (M7).** Not a tautology, and
  the strongest thing in the block. M6 and M7 plant the SAME code defect and
  differ only in the assertion form, so the pair isolates the assertion as the
  cause of the difference. That is what the comment at tests.rs:1473-1475
  claims.
- **The one that changes only a test on unmutated code (M8).** Not a
  tautology. The claim is about the assertion form, not about the code, so the
  code must stay correct for the measurement to mean anything; a failure on a
  correct answer is exactly what is claimed, and the printed
  `left: Some(NaN) / right: Some(NaN)` shows why.

Two things the mutation block does not say that a reader might want. M5 also
fails `a_reference_writes_itself_back` (`test.log:2377-2380`), so the buffer
change is caught by a pre-existing test as well as the new one; the log prints
this, the report's table does not, and the report's line for M5 could be read
as implying the new test is the only guard. And M9 and M10 also fail the tool's
integration test `cells_counts_every_cell_and_edits_nothing`
(`test.log:2444-2448, 2466-2470`), which is worth knowing because it means
`numbers_parsed` and `booleans_parsed` in the corpus run are guarded too.

### The release-binary hash assertion in `run.log`

**It is a real comparison, and a reader can see the expected hashes were not
typed.** The mechanism is at `records/logs/m1p3-p3/run.log:1155-1182`:
`size_log_hash()` is a `sed` extraction from
`records/logs/m1p3-p3/size.log`, by the binary's own path, at run time; the
four expected values are never written in the run script. The comparison
output prints both values on adjacent labelled lines for each of the four
binaries (`run.log:2077-2090`), and the four "size.log" lines match
`size.log:1893-1896` exactly. There is a control in the same command form: the
same reader is asked for `target/release/no-such-binary` and must answer
nothing (`run.log:1177-1182`, output at 2092), which is what rules out a
reader that answers the same thing to everything. The rebuild happens before
any corpus pass and the run exits 1 if any hash differs
(`run.log:1183-1186`). All four reproduced byte for byte.

One structural remark: this makes `run.log` depend on `size.log` already being
at its committed path, which it was. If `size.log` were ever rerun after
`run.log`, the citation would dangle. The phase's order (size 22:33, run
22:13) means `run.log` read `size.log` attempt 4's content while attempt 4 was
still being produced - in fact `run.log` ran BEFORE the final `size.log`
finished (22:13 against 22:34:48). The hashes matched, so the binaries are the
same ones; but planning may want to confirm that the `size.log` `run.log` read
at 22:13 is the one committed, since only the final attempt is kept. The four
hashes in the committed `size.log:1893-1896` do equal the four in
`run.log:2079-2089`, which is the evidence that matters.

---

## 3. Does the report's every number and quoted string appear in a primary?

I checked every figure and quoted string in the draft. The great majority are
in a primary at the line I give. The exceptions are listed at the end of this
section.

### Verified against a primary

| report figure or string | primary |
| --- | --- |
| `25750836734736cbca6701320b59361ad157400b` / `2575083` | `git rev-parse HEAD`; `gate.log:1-4`, `size.log:1-4`, `run.log:1-4`, `scan.log:1-4`, `test.log:2` |
| the four commit hashes and subjects | `git log --reverse 79f1471^..HEAD`; `gate.log:796-801` |
| `9339dca` as the base | `scan.log:10`; `git log` |
| `cells::decimal` the only `u32` parse (with my correction in 1f) | `crates/emend/src/cells.rs:269, 858, 908, 942, 981` |
| `s="+1"`, `si="+0"`, `r="007"` read as 1, 0, 7 | `test.log:2188-2213, 2305-2320` (M1) |
| 261979 row `r`, 2094287 cell `s`, 336 `f` `si`, 783275 shared indices | e127fc2's commit message only; see below |
| styled 2094287, shared 783275, 44 + 292 | `specs/0010-reading-cells.md:47, 55, 57-58, 279-280`; `run.log:2257, 2225, 2243, 2245` |
| `52ef4e5` already replaced `MAIN[1]` | `git show 52ef4e5`; `crates/emend/src/cells/tests.rs:975-982`; no `MAIN[` use in the tree |
| the six typed-view test names | `crates/emend/src/cells/tests.rs:1459, 1477, 1512, 1527, 1550, 1565` |
| "the 5410 the corpus holds" | `specs/0010-reading-cells.md:279`; `run.log:2239`; `records/m1p3-p3-...md:54` |
| the eleven mutations and every outcome | `test.log:2305-2502` |
| the control is the unmutated worktree; the applier refuses; the restored worktree holds neither witness | `test.log:2298-2301, 2493-2500` |
| seven figures in 2575083's message; `elements 11` in `size.log`; `elements 3` in the message's run | `test.log:2157-2176`; `size.log:715, 1827-1834` |
| `sum 3.76` control; items of 5 and 2, own values 3 and 4, total 14 | `test.log:2174-2176` |
| 247348 / 249856 / 185152; 174560 / 176128 / 135200; 63336 / 65536 / 44512; 19172 / 20480 / 6304 | `size.log:1787-1792, 1959-1962, 1875-1885`; `records/size-m1p3-...md:67-70, 98` |
| ceiling 393216; 183980; 209236; 145868; 143360 | `size.log:1788, 1799-1804`; `records/size-m1p3-...md:72-80`; `records/size-m1p2-...md:62` for 183980 |
| three identical, four moved 32 section and 0 file bytes | `size.log:1783-1789`; `records/size-m1p3-...md:46-54` |
| the variant's figures and that it is not committed | `size.log:1862-1885`; `records/size-m1p3-...md:93-100` |
| the four release hashes reproduced | `size.log:1893-1896`; `run.log:2077-2091` |
| the 20 spec figures on both targets | `run.log:2219-2257` and `2379-2417`; the 23-key totals in `records/m1p3-p3-...txt:5, 38` |
| `values` 2089191, `empty` 5410 | `specs/0010-reading-cells.md:275-277` (as ruling 1 corrected); `run.log:2237, 2239, 2397, 2399` |
| `items` 110582, `bytes` 1497091 | `run.log:2531, 2534-2535, 2611`; `specs/0010-reading-cells.md:441` |
| every workbook's 23-key line equalled the python reader's; the two targets identical file to file | `run.log:2210, 2370, 2625` |
| `numbers_parsed` 1305578 = `number` 1310988 less `empty` 5410 | `run.log:2272, 2432, 2663-2669`; arithmetic checks |
| P3-F1's account of the `yn [ ] && [ ]` fault | `size.log:10` (rerun header), 860-861 (`variant_built_clean`), 1866-1868 |
| "the eight earlier probes" against seven | `specs/0010-reading-cells.md:374`; `size.log:39-46`; `records/size-m1p3-...md:40-42` |
| test.log attempt 1's `yn ! probe_control`; run.log attempt 1's wrong section; size.log attempt 2's wrong sentence | `test.log:4`; `run.log:12`; `size.log:10` |
| 347 tests both targets, 337 at 9339dca, 263 library, 85 cells against 75, 14 tool, 2 ignored | `test.log:1881-1891, 2091-2096, 2116-2117, 2505-2507`, and the ignored-test checks at `test.log:390-394` |
| fmt, clippy, rustdoc clean | `gate.log:1017-1019` (PASS fmt, clippy, doc) |
| nine PASS lines, the nine names | `gate.log:1015-1026` |
| negative log's four new cases, ruling 2's four refusals, catalogue accounted for, 0 UNEXPECTED | `negative.log:2332-2339` (U14), 2208-2221 (T2), 2273-2277 (U6), 2341-2365 (U15-U18), 2406-2408, 2420; `grep -c ": UNEXPECTED$"` over the log returns 0 |
| the eight log/record byte counts and sha256 | I ran `sha256sum` and `wc -c` over all eight: every one matches |
| the close-out push carries 9bd9102 and 7702f6e; nothing pushed | `specs/0010-reading-cells.md:299, 388-391`; `git merge-base --is-ancestor 9bd9102 origin/main` is false |

I also independently re-derived the corpus totals from
`records/m1p3-p3-2026-09-18-2575083.txt` rather than trusting its summary
line: summing the 54 per-workbook rows and halving gives sheets 95, cells
2094601, number 1310988, shared 783275, values 2089191, empty 5410, formulas
262879, numbers_parsed 1305578, resolved 783275, styled 2094287. Every one
equals the spec figure and the `.md` table. And I re-ran the host test suite
and formatter myself: 347 passed, 0 failed, 2 ignored; `cargo fmt --check
--all` clean; `cargo clippy --workspace --all-targets -D warnings` clean.

### What I could not find in a primary, or found to differ

1. **"Five local commits over `9339dca`, nothing pushed" (report line 2).**
   There are FOUR commits over 9339dca right now; the fifth is the records
   commit that has not been made. The report's own table marks it "(the
   records commit)" and its closing section says "the records commit
   included", so the intent is clear, but the opening sentence states as fact
   a commit count that git does not yet show. Everything under
   `records/logs/m1p3-p3/` and the three record files is untracked.
2. **The doc commit's pre-commit verification** - see 1h(iii). The `git diff
   --stat` and `sha256sum` "printed in the commit's own verification before it
   was written" are in no log. The at-HEAD hashes are at `gate.log:822` and
   `scan.log:978, 1018, 1028`.
3. **The corpus scan behind ruling 2 is in no log.** "261979 row `r` values,
   2094287 cell `s` values, 336 `f` `si` values and 783275 shared string
   indices, none non-canonical, no row numbered 0" appears only in e127fc2's
   commit message and in the report. I grepped all six logs for `261979` and
   for "non-canonical": no hit. Three of the four totals can be cross-checked
   against `specs/0010` and against `run.log`, and the measurement is a
   counts-only scan that names no content, so I do not think it is wrong - but
   `specs/0001`'s "nothing is called measured unless the log prints the
   measurement" makes the report's "measured before the commit rather than
   discovered in the run" a claim with no primary behind it. Planning should
   either accept the commit message as the record or ask for the scan's
   output.
4. **"Three of those totals equal the spec's figures exactly".** Four do: the
   spec also gives 261979 `<row>` elements, every one with `r`
   (`specs/0010-reading-cells.md:62`). An understatement, not an
   overstatement.
5. **The rerun account is incomplete.** Six superseded attempts are declared
   in the logs: `size.log:10` names three (attempt 1 "the feature map named in
   its own header", attempt 2, attempt 3), `test.log:4` one, `run.log:12`
   one, and `negative.log:4` one (its working-tree check did not count the
   size record, so one check read UNEXPECTED). The report names four and is
   silent about `size.log` attempt 1 and about `negative.log` attempt 1
   altogether. Its sentence "Every superseded attempt is named and hashed in
   the rerun header of the log that replaced it" is TRUE of the logs; it is
   the report's own enumeration that is short. I would add
   `negative.log`'s rerun to the findings, since unlike the other two it was
   superseded for a check that read UNEXPECTED, not for a wrong string.
6. **`scan.log`'s wrong phase line** (`scan.log:11, 249`, "part 3 P2") is in a
   primary and is not in the report. See question 2.
7. **The 32-byte gap in the budget paragraph** and the section-versus-file
   headroom figure (both in 1d) are in the primaries and are repeated in the
   report without reconciliation.
8. **"`scan.log`'s bytes are the one figure planning should take from
   `sha256sum` over the committed file"** (report, under "The logs") confuses
   two things: `wc -c` gives bytes and `sha256sum` gives the hash, and the
   figure missing from the table is `scan.log`'s own HASH, not its byte count.
   For the record, the file on disk now is 69873 bytes, sha256
   536ce47f7b5035cd2390fd793f102de94bf21c3b63268073aed8ce31bc148d61 - but that
   is the pre-answers run and will be replaced.
9. **Minor, on the negative log.** The spec's P3 sentence asks for the tool
   "on one whose shared strings part is not `sst`". `negative.log`'s T2
   (2209-2215) raises that message through `--shared-strings`, not through
   `--cells`, although `--cells` also reads the table
   (`tools/emend-copy/src/main.rs:556`) and would raise it too. The message is
   exercised through the built binary, which is the substance; the `--cells`
   path to it is not. The report's phrasing ("The negative log covers the
   spec's four new cases through the built binary") does not distinguish them.

---

## Summary

The diff does what the spec's P3 section and P2 rulings 2 and 8 ask and
nothing else. No test in it asserts a position or a mechanism instead of a
property - the two places where an equality would have passed wrongly use
predicates instead, and M6/M7 and M8 measure that choice rather than assert
it. The eleven mutations are real measurements, including the three awkward
arrangements, and none of them is a tautology. The size record is byte-for-byte
what `size.log` printed, and every one of its figures re-derives from the probe
table. The corpus figures re-derive from the record's own per-workbook rows.
The release-binary hash assertion is a genuine comparison with its expected
values read out of a file at run time and a control that proves the reader can
answer nothing.

What I would put in front of planning, in order of weight:

1. Two false statements about `u32::from_str`, one in a shipped doc comment
   (`crates/emend/src/cells.rs:848-850`) and one in a commit message
   (e127fc2), plus a test comment that mis-describes six of its eight
   spellings (`crates/emend/src/cells/tests.rs:1373-1374`). I measured
   `u32::from_str` on the pinned toolchain: it refuses a leading `-`.
2. `Cell::number`'s doc comment attributes all five of its bullets to one
   test; two of them are measured by two other tests
   (`crates/emend/src/cells.rs:479-482`, repeated in 8c8a0ca's message).
3. `scan.log:11` names the phase as P2, from a stale line in the printed
   script at `scan.log:249`.
4. The `scan.log` on disk is the pre-answers run: `RESULT: FAIL`, 0 check
   answers scanned. Planning must verify the committed one afresh.
5. The report claims a pre-commit doc verification and a corpus scan that no
   log prints, and names four of the six superseded attempts.
6. The budget paragraph's 209236 - 63336 does not reach 145868 (the 32 bytes
   are explained elsewhere in the same record but never reconciled here), and
   "143360 file bytes under" a section-byte ceiling mixes units.
7. The report's own overstatement that `decimal` is "the only thing that turns
   an attribute into a `u32`": `CellRef::parse` does too, with the same rules.

None of these changes a measured figure. Items 1 and 2 are the two that
`specs/0001` exists to catch.
