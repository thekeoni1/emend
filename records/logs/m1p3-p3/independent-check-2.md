# Independent check 2 of M1 part 3 P3

Phase: Milestone 1 part 3 P3, `specs/0010-reading-cells.md`, "P3: the typed
views, the probe, the size record".
Phase commit: `25750836734736cbca6701320b59361ad157400b`.
I read only the files the check prompt names. I did not read the corpus, did
not run the corpus runner, and changed nothing in the repository: `git status
--porcelain` at the end of my run holds the same four untracked entries it
held at the start. My own reruns were made in a copy of the tree produced by
`git archive 2575083 | tar -x` into my scratch directory, with its own
`CARGO_TARGET_DIR`, and that copy is deleted.

---

## 1. Does the diff do what the spec says, and nothing else?

The diff is eight files, `git diff --stat 79f1471^..HEAD`: `ROADMAP.md`,
`crates/emend/src/cells.rs`, `crates/emend/src/cells/tests.rs`,
`crates/emend/src/package.rs`, `specs/0001-bootstrap.md`,
`specs/0010-reading-cells.md`, `tools/size-probe/Cargo.toml`,
`tools/size-probe/src/bin/probe-emend-cells-xml.rs`. Four commits, the first
planning docs alone, three code. Nothing outside that set moved.

### `Cell::{number, boolean, error}` with tests

Every case the spec's P3 sentence lists is carried, and each has a test:

- `1.10` and `1E+3` and `-0`: `crates/emend/src/cells/tests.rs:1456-1474`,
  `a_numbers_text_is_read_as_an_f64`. `-0` is asserted through
  `is_sign_negative()` at `tests.rs:1473`, after an `assert_eq!(zero, 0.0)`
  that the test's own comment says would hold for either sign. That is the
  right assertion for the property.
- `inf` and `nan` "as the standard library parses them":
  `tests.rs:1476-1512`, `the_parse_is_the_standard_librarys_and_wider_than_
  the_format`. `inf`, `INF`, `infinity`, `1e309` asserted with
  `is_infinite()` plus `is_sign_positive()`; `-inf`, `-1e309` with
  `is_sign_negative()`; `nan`, `NaN` with `is_nan()`, the only predicate that
  can pass, with the reason in the comment at `tests.rs:1501-1502`; a leading
  `+` at `tests.rs:1510-1511`.
- a non-number `Number` cell `None`: `tests.rs:1514-1528`,
  `a_number_cell_whose_text_is_not_a_number_has_none`, over `x`, `" 1"`,
  `"1 "`, `1,5`, `0x1`, `TRUE`, the empty value, and a `<c r="A1"/>` with no
  value at all.
- `1`, `0`, `TRUE` as `None` for `boolean`: `tests.rs:1530-1550`,
  `a_booleans_text_is_read_as_a_bool`, plus `true`, `-1`, `2`, `01`, an empty
  value and no value.
- an error value: `tests.rs:1552-1565`, `an_error_cells_text_is_its_error`,
  three spellings including `#SOMETHING_NEW!` and an error cell with no value.
- and, beyond the spec's list, `each_typed_view_answers_only_for_its_own_kind`
  at `tests.rs:1567-1604`, which asks all three views of one cell of every
  kind and asserts the answers as one seven-row table.

Six tests, which is the report's count.

**The doc comment does state what the parse takes** (`cells.rs:477-492`) and
every claim in it is true. Two inaccuracies in it, both small, neither
wrong about behaviour:

- `cells.rs:479-482` says "Each case below is measured in `cells::tests`, by
  `the_parse_is_the_standard_librarys_and_wider_than_the_format`". Two of the
  five bullets are not measured by that test: the surrounding-space and
  empty-value bullet is measured by `a_number_cell_whose_text_is_not_a_number_
  has_none` (`tests.rs:1518-1527`), and the `1.10`/`-0` bullet by
  `a_numbers_text_is_read_as_an_f64` (`tests.rs:1469-1473`). The attribution
  names one test where three do the work.
- `cells.rs:484-486` says `inf`, `INF`, `infinity`, `nan`, `NaN` parse "in
  any case". Only those five spellings and `-inf` are measured; `Infinity`,
  `NAN` and mixed spellings are not. The generalisation is true of
  `str::parse::<f64>` but is wider than the measurement, which is the thing
  this project's rule is about.

Nothing in the doc comment is false, and nothing in it is asserted where a
test could have measured it and none does.

### The catalogue and the Limits sections

Catalogue: `crates/emend/src/package.rs:89-113` now names the P3 tests beside
the four messages they reach (`a_row_number_of_zero_is_refused`,
`a_number_this_layer_reads_is_digits_alone`,
`an_index_past_u32_is_refused_like_one_that_is_not_a_number`), and says why
one `si` message covers an absent and an unreadable index. The set-equality
test compares message strings, not test names, so the names are hand
maintained; the commit message says so (`8c8a0ca`), which is honest.

Limits, `cells.rs:92-119`. Six entries. Each is a decision with a reason, not
a restatement:

| entry | verdict |
| --- | --- |
| no SEEKING (`cells.rs:95-98`) | decision; reason given (the format keeps no cell index); true of the code, `Cells` is a forward `Iterator` over `xml::Stream` |
| no EVALUATION (`cells.rs:99-103`) | decision; reason given (openpyxl's contract, `CLAUDE.md`); true |
| no DATE CONVERSION (`cells.rs:104-108`) | decision; reason given (1900/1904 base, part 5, the dependency rule); true, and measured at `tests.rs:1601-1603` |
| no LOOKUP of a style (`cells.rs:109-111`) | decision; reason given; true |
| unmodelled attributes (`cells.rs:112-116`) | decision; see the imprecision below |
| no SHEET-SIZE limit (`cells.rs:117-119`) | decision; reason given (Excel's limit, not the format's); true, and measured by the `XFD1048576` and `4294967295` rows of `every_reference_the_reader_emits_parses_back` |

The whole-table-in-memory clause the spec asks for is the pre-existing text
at `cells.rs:73-85`, not added by this diff; the diff adds the six entries
above. That satisfies the spec's list but a reader of the diff alone would
not see the "whole table in memory" clause as new. That is fine.

**One Limits entry is imprecise.** `cells.rs:113` reads "`<f>`'s `ca`, `aca`
and the rest". `ref` is an attribute of `<f>` and it IS modelled: it is
`Formula::reference`, carried as written (`cells.rs` `read_formula`, the
`b"ref"` arm). So "and the rest" is false for exactly one attribute. The
spec's own "Module `cells`" text has the same looseness ("`ca`, `aca` and
every other attribute of `<f>` are not modelled" two sentences after
"`reference` is `ref` as written when present"), so this is inherited rather
than invented. Worth one word in the doc comment. Nothing else in either
Limits section is false.

`package.rs:150-165`: three entries, each a decision with a reason.
`shared_strings` parses on every call and returns owned - true
(`package.rs:621-625`, no cache). `cells` borrows mutably for the life of the
iteration - true, `pub fn cells(&mut self, ...) -> Result<Cells<'_, R>, Error>`
at `package.rs:654`. Neither records an edit - true, and measured in three
places: the P2 byte-identical-copy unit test, the integration test
`cells_counts_every_cell_and_edits_nothing`, and `size.log`'s "the cells probe
writes the same package the package probe writes, so its reads changed
nothing: yes" (`records/logs/m1p3-p3/size.log:1855`).

### `probe-emend-cells-xml` and the float-parser variant

`tools/size-probe/src/bin/probe-emend-cells-xml.rs` is
`probe-emend-package-xml` line for line through `package.replace_part` (I
compared it against `tools/size-probe/src/bin/probe-emend-package-xml.rs:59-118`;
the open, the workbook, the chain recipe, the two additions and the
stream replacement are the same code), then adds
`package.shared_strings()`, `package.cells(sheet)`, `cell.number()` and
`table.resolve(&cell)`, then writes. So it links what the spec's sentence
describes. It prints `sheets`, `elements`, `items`, `cells`, `sum`, `text`,
which covers the three the spec asks for plus the package probe's own two.

**The documented order departure changes nothing that is linked.** The reads
sit before `write_to` because `write_to` takes the package by value
(`probe-emend-cells-xml.rs:11-14` and `:139-147`). The set of symbols reached
is identical either way; only the sequence differs, and the reads run against
a package with a replacement, an addition and a removal pending, which is not
a weaker case. I agree with the commit message and the report on this.

**One thing the report and the spec's "so no path is dead" phrasing do not
say.** `Cell::boolean` and `Cell::error` have no caller in the probe; the
only typed view it reaches is `number()`. So the measured "part 3's own cost"
of 63336 section bytes does not include those two functions. The probe's own
doc comment is careful here - it names the three paths it keeps alive
(`probe-emend-cells-xml.rs:6-9`) and does not claim more - but the report's
sentence and the spec's "so no path is dead" are wider than what the binary
links. The omitted code is two kind comparisons and a clone-free return, so
the figure is not materially wrong; planning should know the boundary.

The probe uses `THEME[0]` positionally at
`probe-emend-cells-xml.rs:134`. That is what `probe-emend-package-xml.rs:112`
already does and is a deliberate choice of relationship-type spelling, not the
defect ruling 8 was about. No finding, noted for completeness.

### The size record and `size.log`

Every figure in `records/size-m1p3-2026-09-18-2575083.md` is printed by
`records/logs/m1p3-p3/size.log`. I checked all of them:

- the eight probes' file/section/.text figures: `size.log:1526-1560`.
- the seven-probe comparison table and the four `code` verdicts:
  `size.log:1562-1783`, with each `code` verdict's differing-byte counts and
  its `.data.rel.ro` slot deltas printed, and a check per probe that a commit
  which could have caused it is named.
- the three budget figures and the float parser's cost:
  `size.log:1786-1806` and `size.log:1858-1879`.
- the variant's own figures 768548 / 761392 / 590003 and its run
  (`sum 9` where the probe reads `sum 3.75`): `size.log:1868-1877`.
- the four release binaries' hashes: `size.log:1881-1890`.

I recomputed the arithmetic myself from `size.log:1526-1560`: 780564-533216 =
247348; 789028-539172 = 249856; 596307-411155 = 185152; 780564-606004 =
174560; 789028-612900 = 176128; 596307-461107 = 135200; 780564-717228 = 63336;
789028-723492 = 65536; 596307-551795 = 44512; 780564-761392 = 19172;
789028-768548 = 20480; 596307-590003 = 6304. All twelve agree with the record
and the report.

**The "eight earlier probes" count: I counted the probes myself and the log's
handling is honest.** `records/size-m1p2-2026-09-18-14c2e14.md:25-31`
tabulates exactly seven probes. The eighth, `probe-emend-cells-xml`, is added
by this part. `size.log:43-49` states the discrepancy in its own header, names
the seven, says why the eighth has nothing earlier to compare with, and the
log then measures eight (`size.log:1500`, `size.log:1526-1560`) and compares
seven (`size.log:1783`, "of the seven earlier probes, 3 keep every part 2
figure"). The size record repeats the reason at its own "The seven earlier
probes against part 2" heading. Nothing is quietly counted as eight. The
report's finding is accurate and I would raise the same one.

Two stale section headings inside `size.log`: `size.log:1479` reads "## the
seven size builds for i686-unknown-linux-musl" and then lists and checks
eight ("all eight size builds exit 0: yes", `size.log:1498`); the script's own
comment at `size.log:22` says eight. Cosmetic, but it is a wrong number in a
committed log.

### `--cells` gains nothing

Confirmed independently. No file under `tools/emend-copy/` is in the phase
diff at all, so neither the tool's source nor its integration tests changed,
and no ruling required a change there. The integration count is unchanged at
14 (`records/logs/m1p3-p3/test.log:1867` against
`records/logs/m1p3-p2/test.log:1634`). The report is right.

### Rulings 2 and 8, which `e127fc2` carries

**Ruling 2, one helper.** Yes. `cells::decimal` (`cells.rs:841-867`) is the
only path from an attribute to a `u32`. I grepped `cells.rs` for every parse:
the only two `.parse()` calls left are `cells.rs:501` (`number()`'s f64) and
`cells.rs:866` (inside `decimal`). The four sites now call it:
`SharedStrings::resolve` (`cells.rs:268-269`), `read_opening`'s `s`
(`cells.rs:907-908`), `row_number`'s `r` (`cells.rs:942-944`) and
`read_formula`'s `si` (`cells.rs:981`). `CellRef::parse` builds its own
accumulators rather than going through `decimal`, which is correct: it reads a
column as base-26 letters and a row as digits, both in `u64` with a bound
check per step (`cells.rs:319-337`).

**A row number of 0 is refused.** Yes, `cells.rs:942-944`, by
`.filter(|number| *number > 0)` before the `ok_or`, with the message an
unreadable row number already had. Pinned by
`a_row_number_of_zero_is_refused` (`tests.rs:1321-1332`) and reached through
the built binary as case U15 (`records/logs/m1p3-p3/negative.log:2342-2347`).

**`CellRef::parse(a1(x)) == Some(x)` is asserted as a property over what the
reader emits.** Yes. `every_reference_the_reader_emits_parses_back`
(`tests.rs:1334-1364`) reads a sheet holding an explicit `r`, a cell with no
`r`, a row with no `r`, `XFD1048576`, row `4294967295` and the saturated
column, then loops over EVERY cell the reader yielded and asserts
`CellRef::parse(cell.reference.a1().as_bytes()) == Some(cell.reference)`. It
is a loop over the reader's output, not a list of spellings, and it guards the
count first (`assert_eq!(cells.len(), 6)`). This is the right shape.

**Ruling 8: the claim that it needed no change is true of the code as it
stands.** I grepped the whole crate for a positional use of a namespace
constant (`MAIN[`, `RELS[`, `THEME[`, `CALC_CHAIN[` under
`crates/emend/src/`): the only hit is the words `MAIN[1]` inside the comment
at `tests.rs:977`, which describes the defect. `git show 52ef4e5` confirms
that P2 commit replaced `MAIN[1]` with the written-out `STRICT` constant and
added the assertion that it is one of `MAIN`'s spellings. So the ruling truly
needed no code change and nothing was invented for it.

**But the report's heading "Ruling 8 needed no change, and that is measured"
overstates.** No log prints a scan for positional uses of a namespace
constant; the only measurement is the test's own assertion that the literal is
one of the crate's two spellings, which is a different claim. The statement
"No positional use of a namespace constant is left anywhere in the crate" is
true - I verified it - but it is verified by me and by planning, not by a
primary this phase produced. Under `specs/0001` ("Nothing is called measured
unless the log prints the measurement") the word "measured" in that heading
does not hold.

### Does any test assert a POSITION or MECHANISM rather than a PROPERTY?

No. I went through all eleven new tests. The ones that could have gone wrong
do not:

- `the_parse_is_the_standard_librarys_...` uses `is_infinite`,
  `is_sign_positive`, `is_sign_negative` and `is_nan`, which are the
  properties; `assert_eq!` against `f64::NAN` would have been a mechanism
  error and `test.log`'s M8 measures that it would have failed.
- `a_numbers_text_is_read_as_an_f64` asserts the sign BIT of the zero, not an
  equality that holds for either sign.
- `each_typed_view_answers_only_for_its_own_kind` asserts the whole
  seven-row answer table in one `assert_eq!`, so a view that answered for a
  neighbour cannot hide behind a per-cell assertion that was not written.
- `the_other_main_namespace_spelling_is_read_too` still names the strict
  spelling by its bytes and asserts membership in `MAIN`, so the diff does not
  reintroduce the position it removed.
- The remaining `cells[n]` indexing (for example `tests.rs:1362-1363`) indexes
  the reader's own output in the order the test wrote the cells, with the
  length asserted first. That is not the recorded defect class.

### Does any comment or commit message assert something no measurement in the logs supports?

Yes, three. This is the second recorded class and I disagree with the report,
which presents the first of these as measured.

**(a) `e127fc2`'s corpus canonicality block.** The message says: "over the 27
workbooks' worksheet parts, 261979 row r values, 2094287 cell s values, 336 f
si values and 783275 shared string indices, and not one of them
non-canonical, no row numbered 0 ... measured before this was committed
rather than discovered in the phase's run". I grepped all of `records/` for
`261979`: it appears nowhere in `records/`, in any log or record of any phase.
`336` as an `si` count appears nowhere either. No log prints the scan, its
script, its counts, or the claim that it was made before the commit. Two of
the four figures are corroborated elsewhere - `261979` is the `<row>` count in
`specs/0010-reading-cells.md` ("Facts planning checked"), and `336` is that
spec's 44 masters plus 292 followers - and `2094287` and `783275` are
asserted totals in `run.log:2256` and `run.log:2224`. The substance of "no
corpus workbook is affected" IS measured after the fact, because `run.log`'s
corpus pass at this commit ran the strict `decimal` over all 27 workbooks and
every one of the 20 spec totals agreed with exit 0, which could not happen if
a corpus `r`, `s` or `si` were non-canonical. So the conclusion is sound. What
has no primary is the four counts and the "measured before the commit" claim.
The report reproduces the block verbatim in its "Ruling 2" section and adds
"measured before the commit rather than discovered in the run": that sentence
has no primary and should either be cut or backed by a printed scan.

**(b) The intermediate test counts.** `e127fc2` says "257 pass on the host and
on i686-unknown-linux-musl", and `8c8a0ca` says "263 pass ... against 257 at
`e127fc2`". `253` at `9339dca` is backed (`records/logs/m1p3-p2/test.log:1641`
and `:1654`) and `263` and `347` and `337` are backed
(`records/logs/m1p3-p3/test.log:1877`, `:1890-1891`), but `257` appears in no
log of any phase. I grepped `records/logs/m1p3-p3/*.log` for `257`: no hit.
Minor, and long-standing practice in this project's commit messages, but it is
a figure in a commit message with no printed measurement.

**(c)** The "no positional use of a namespace constant" claim in `e127fc2`,
covered above: true, unprinted.

Everything else I checked in the diff's comments and messages is backed.
`2575083`'s seven figures are measured at `test.log:2164-2178`.
`8c8a0ca`'s "one doc link was wrong and rustdoc caught it" is consistent with
the gate's `PASS doc`. `e127fc2`'s "one test removed and five added" and
`8c8a0ca`'s six are exactly what the diff shows.

### The design clauses

- **`u64` for anything that could exceed 32 bits.** Yes. `CellRef::parse`
  accumulates in `u64` with a bound check on each step and only then narrows
  (`cells.rs:319-337`), which is the case that actually matters on i686.
  `SharedStrings::len() -> u32` with `usize::try_from` on the lookup
  (`cells.rs:151`, `:163`). The probe's five counters are `u64` and its two
  `usize`-to-`u64` conversions are explicit casts of `len()`
  (`probe-emend-cells-xml.rs:74-80`, `:190`). The `usize` values left in
  `cells.rs` are zip entry indices and XML nesting depths
  (`cells.rs:185`, `:571`, `:1036`, `:1039`, `:1117`), none of them a byte
  count or a file position. No `usize` in the API.
- **The readers stream.** `Cells` is an `Iterator` holding one token and one
  `Cell` (`cells.rs:561-571`, `impl Iterator` at `cells.rs:804`); no whole
  sheet is buffered. The shared strings table is the documented exception and
  `cells.rs:73-85` says so with the corpus figures.
- **`Cells` borrows the package for the life of the iteration.** Yes,
  `Cells<'p, R>` at `cells.rs:561` returned as `Cells<'_, R>` from
  `Package::cells(&mut self, ...)` at `package.rs:654`, so the borrow checker
  enforces one sheet at a time, which is exactly what `package.rs:159-162`
  claims.

---

## 2. Do the logs contain every line the spec requires?

### The required lines, one by one

- **First lines and the phase commit given to the script and compared with
  HEAD, with a control.** Present in all six: `gate.log:1-8`, `test.log:1-11`,
  `negative.log:1-9`, `size.log:1-8`, `run.log:1-10`, `scan.log:1-9`. Each
  prints the commit given to the script AND `git rev-parse HEAD`, and the
  comparison. `gate.log`, `size.log`, `run.log` and `scan.log` carry two
  controls (a null commit and a short sha of the real commit). `test.log:11`
  and `negative.log:9` carry only the null-commit control. `specs/0001` asks
  for "a control that refuses a fabricated commit", singular, so one is
  enough; the inconsistency is worth a note, not a failure.
- **The script printed into each log.** Yes, all six: `gate.log:14`,
  `test.log:14`, `negative.log:13`, `size.log:19`, `run.log:84`,
  `scan.log:29`.
- **Every compound check in a named function.** The mechanised evidence is
  `scan.log:1259-1264`: `chained=0 leading-bang=0 assignment=0` for each of
  the five sibling logs and for the scan script's own source. I spot-read the
  printed scripts and found the same. This is the check that caught P3-F1, so
  it demonstrably works on this phase's material.
- **Every counting form a named function with a positive control.** Yes. Each
  log sources `scripts/counting.sh` and prints a "counting controls" block:
  `test.log:1050-1075`, `size.log:1245`, `scan.log:880`, `gate.log:759`,
  `negative.log:1633`, `run.log`. The controls include the error cases
  (unreadable file reports `error`, not 0), which is the point of the T0 rule.
- **The scan log written LAST, hashing every committed sibling, self-scanning
  before printing itself, with a control.** Yes. Timestamps: gate 21:34:12Z,
  negative 21:37:38Z, test 21:51:27Z, run 22:13:21Z, size 22:33:08Z, scan
  22:34:54Z, so the scan is last. `scan.log:1116-1206` hashes the five sibling
  logs and the three records; `scan.log:1174` "sibling logs scanned: 5
  (expected 5)". `scan.log:16-28` runs the pre-P6d forms over its own source
  before printing it, reports `count=0`, and shows the control
  (`count=1` on a copy with one concatenated shape appended). I verified the
  eight hashes and eight byte counts in the table myself with `sha256sum` and
  `wc -c`: all eight match to the byte.
- **The three forbidden-form greps, each with a fixture and a control, over
  every printed script including the scan script's own source.** Yes,
  `scan.log:1231-1268`. The three patterns are assembled at run time and
  printed; each has a positive fixture and a negative (named-function)
  control; and there are six cross-controls showing each grep reports 0 on the
  other two shapes. Coverage is 9 files, the scan script's own source included
  (`scan.log:1264`).
- **Every sha256 printed by a command over a file that exists at run time,
  superseded attempts included.** Almost. See the gap below.
- **Every claim that a test bites being a printed measurement in `test.log`.**
  Yes, eleven of them, `test.log:2180-2502`. See the mutation section.

### FIVE logs and no self-test log

The count needs correcting. `specs/0010-reading-cells.md`, "## Phases", asks
for `gate.log`, `test.log`, `negative.log`, `run.log`, `scan.log`, "and
`size.log` in P3": that is **six** logs for P3, and six is what
`records/logs/m1p3-p3/` holds. "Five" is the number of SIBLINGS `scan.log`
hashes (it cannot hash itself), which is what `scan.log:1174` reports. So:
the phase writes six logs, consistent with the spec, and the draft report's
commits table row "(the records commit) | the five logs, the check answers,
the run record pair and the part's size record" undercounts by one -
`scan.log` is a sixth log that the records commit must carry. The report's
log table also lists five and says why `scan.log`'s own hash is absent, which
is correct reasoning, but the commit-contents line should say six.

No `self-test.log` is correct. `specs/0001` requires one only for a phase that
changes a gate file. **I checked the no-gate-file claim myself** rather than
taking the log's word: the ten gate files are `scripts/check.sh`,
`scripts/privacy.sh`, `scripts/privacy-patterns.txt`,
`scripts/privacy-identity.txt`, `scripts/sys-allowlist.txt`,
`scripts/install-hooks.sh`, `scripts/counting.sh`,
`scripts/gate-self-test.sh`, `.gitignore` and the CI workflow, and none of
them is among the eight files of `git diff --stat 79f1471^..HEAD`. The claim
is true. `gate.log:832-838` lists the ten, reports "of them in the phase diff:
0", carries the positive control that the same search does find `ROADMAP.md`,
and asserts `self-test.log` is absent. `scan.log:1175-1180` repeats it.

### FINDING: `scan.log`'s phase line names the wrong phase

`records/logs/m1p3-p3/scan.log:11` reads:

    phase: Milestone 1 part 3 P2 (specs/0010-reading-cells.md) scan.log; run at 2026-09-18T22:34:54Z

It should read P3. The cause is in the printed script itself,
`scan.log:249`, which carries the P2 scan script's `echo` unchanged; the
script's derivation note at `scan.log:50` correctly says it was derived from
the P2 scan script, so the string was simply not retargeted. Every other log
has the right phase line (`gate.log:9`, `test.log:3`, `negative.log:3`,
`size.log:9`, `run.log:11`), and the scan log's own `about:` line at
`scan.log:2` names the correct P3 range, so this is one wrong string and no
wrong measurement. It is nevertheless exactly the defect class `run.log`
attempt 1 was superseded and rerun for ("the record's first line cited the
spec's 'P2: the cell reader' section where this is P3's run",
`run.log:12`), and this one is in the committed file and is not named
anywhere. The draft report does not mention it. Since `scan.log` must be
rerun anyway once the check answers exist, the fix is free.

### FINDING: the committed `scan.log` reads RESULT: FAIL and the report never says so

`scan.log:69862-69873` ends:

    the check answers went through all three greps as prose: no
    ...
    structural checks failed: 1
    RESULT: FAIL

with `check answers scanned: 0` at `scan.log:1183`. The launch note told me
this is the pre-check-answers run and I judge everything else in the log
normally, which I have; the one failing check is the absence of the answers,
this one among them. But a reader of the draft report would not learn that the
committed scan log fails, or that it has to be rerun after the answers land.
The report's only sentence about `scan.log` is about its hash. I would add one
line saying the scan log will be rerun last, with the answers present, and
that the committed one is that rerun.

### FINDING: one superseded run is neither hashed nor named as unhashed

`specs/0001` "Rules for every phase": "A superseded attempt is hashed and kept
before the next one starts; an attempt that was not kept is named as having no
hash, and why."

`size.log:10` says the size attempt 3 was superseded "for a DEFECT THIS
PHASE'S OWN SCAN CAUGHT ... scan.log's shape grep over every printed script
reported chained=1 for size.log". That scan run is itself a superseded
attempt: the committed `scan.log` reports `chained=0` for `size.log`
(`scan.log:1260`). The scan attempt that read `chained=1` is nowhere named,
nowhere hashed, and `scan.log:1-15` carries no `rerun:` header at all, unlike
`test.log:4`, `negative.log:4`, `size.log:10` and `run.log:12`. So the
measurement that is the whole substance of finding P3-F1 -
`chained=1` for `size.log` - exists only as prose in `size.log:10`, and the
run that produced it has no hash and no acknowledgement. The three superseded
SIZE attempts are properly hashed (`size.log:11-15`), so this is a gap about
the scan, not the size log. Planning should get either that scan attempt's
hash, or a line in the final `scan.log` naming it as an attempt that was not
kept and why.

### `test.log`'s eleven mutations, one by one

The block is `test.log:2180-2502`; the applier is printed at
`test.log:2185-2303` and refuses a mutation whose text it cannot find exactly
once, which is printed for every mutation as "the applier found its text
exactly once and replaced it: yes". The worktree is created detached at the
phase commit and that is checked (`test.log:2181-2183`). The unmutated control
passes (`test.log:2493-2500`) and the restored worktree is measured to hold
neither witness (`test.log:2501`).

**I reproduced two of the eleven myself**, in the scratch copy described at
the top of this answer, not in the repository's working tree:

- M6 (`number()` loses a zero's sign, no test change): I got
  `a_numbers_text_is_read_as_an_f64 ... FAILED`, panicking at
  `crates/emend/src/cells/tests.rs:1473:5`, and `test result: FAILED. 262
  passed; 1 failed; 1 ignored`. That is byte-for-byte what `test.log:2401-2409`
  records, panic line included.
- M7 (the same code defect plus the sign assertion rewritten as an equality):
  I got `test result: ok. 263 passed; 0 failed; 1 ignored`, and
  `a_numbers_text_is_read_as_an_f64 ... ok`. That is what `test.log:2413-2419`
  records.

So the block's two subtlest rows reproduce independently, which also rules out
the mutated runs having been produced by a broken build or a filtered test
selection.

| # | the claim it exists for | does it measure that claim? |
| --- | --- | --- |
| M1 | `decimal`'s doc (`cells.rs:848-853`) and `e127fc2`: `u32::from_str` alone read `s="+1"`, `si="+0"` and `r="007"` as 1, 0 and 7 | YES. The witness (`test.log:2194-2211`) asserts `style == Some(1)`, `FormulaKind::Shared { index: 0 }`, `resolve` of `+0` giving the item at 0, and `a1() == "A7"`. Those are the three answers named, and the witness PASSES on the old code while `a_number_this_layer_reads_is_digits_alone` fails (`test.log:2306-2318`). A defect alone could not have measured what the old code answered. |
| M2 | that M1's verdict is the mutation's and not the witness's own | YES, and it is not a tautology. The same witness with unmutated code FAILS at `tests.rs:559:10` (`test.log:2320-2330`). M1 and M2 together are a genuine two-sided measurement: the witness passes only where the defect is. |
| M3 | `e127fc2`: a row of 0 gave a cell whose `a1()` was `A0`, which `CellRef::parse` refuses | YES. The witness (`test.log:2214-2221`) asserts `a1() == "A0"` and `CellRef::parse(b"A0") == None`, passes with the row filter removed, and `a_row_number_of_zero_is_refused` fails (`test.log:2332-2345`). |
| M4 | that M3's verdict is the mutation's | YES, same shape as M2: the witness alone FAILS (`test.log:2347-2357`). Not a tautology. |
| M5 | `every_reference_the_reader_emits_parses_back`'s comment that the saturated column is covered | YES. The 7-letter buffer cut to 6 makes it fail with `left: Some(CellRef { row: 4294967295, col: 279062207 })` against `col: 4294967295` (`test.log:2367-2371`), which is the seventh letter going missing. It also breaks the pre-existing `a_reference_writes_itself_back` (`left: "WLQKWU1"` against `"MWLQKWU1"`), which is also a saturated-column assertion, so the report's "only the saturated column stops round tripping" is consistent: the still-passing control the log names is `a_cells_own_reference_wins_over_its_row` (`test.log:2377`). The report's table row is accurate, though it does not mention that a second, older test also bites. |
| M6 | `a_numbers_text_is_read_as_an_f64`'s comment that the sign bit is the assertion | YES. Reproduced by me. |
| M7 | the other half of the same comment: "an equality would hold for either sign" | YES, and it is NOT a tautology. It is the counterfactual: with the defect present and only the equality asserted, the tree passes (`test.log:2411-2419`, and my own run gave 263 passed). That is the only way to measure that the weaker assertion would have let the defect through, and the log says so in the verdict's own wording. |
| M8 | the comment that `assert_eq!` against `f64::NAN` "would fail on a correct answer" | YES, and not a tautology. No code is changed; the correct code fails with `left: Some(NaN) right: Some(NaN)` (`test.log:2421-2432`). The printed left and right are what make the point legible: the values are equal as text and unequal as `f64`. A test-only mutation is the right instrument here, because the claim is about the assertion, not the code. |
| M9 | `each_typed_view_answers_only_for_its_own_kind`'s comment that the shared-string cell "would be read as the number 0 if `number` went by the text alone" | YES, and it is the strongest row. The failing tuple printed at `test.log:2438` holds `(Some(0.0), None, None)` for the shared-string cell, the log counts that tuple (`test.log:2489-2490`), and there is a control that M2's unmutated run holds 0 such tuples (`test.log:2491`). The integration test `cells_counts_every_cell_and_edits_nothing` also bites, with `numbers_parsed=5` against 3. |
| M10 | the same comment, for `boolean` | YES. `boolean()` answers `Some(true)`/`Some(false)` for five foreign kinds (`test.log:2460`) and `booleans_parsed=2` against 1. |
| M11 | the same comment, for `error` | YES. `error()` answers for every kind including the date cell (`test.log:2482`). |

All eleven measure what they are there for. None is a tautology. The three
arrangements the prompt singles out - the witness pairs (M1/M2, M3/M4), the
passing mutation (M7) and the test-only mutation (M8) - are each the correct
instrument for a claim a plain defect could not measure, and each is paired
with the run that makes it two-sided.

One presentational note: for M1 and M3 the "the failures, with the values each
assertion saw" block prints only the panic location, because
`assert_package` panics on a message comparison rather than an `assert_eq!`.
That is a property of the helper, not a gap in the block.

### `run.log`'s release-hash assertion

**It is a real comparison, and a reader can see the expected values were not
typed.** The reader function is named and printed: `size_log_hash()` at
`run.log:1156-1158`, a `sed` over `records/logs/m1p3-p3/size.log` extracting
the hash from that log's own line for a given path. The output at
`run.log:2077-2090` prints both values, labelled "this run" and "size.log",
for each of the four binaries, and the verdict at `run.log:2091`. There is a
control that the reader answers nothing for a path the size log has no line
for (`run.log:2092`), which is what stops a silently empty `want` from
comparing equal. The script stops before the corpus if any differ
(`run.log:1183`). I verified the four pairs against the committed `size.log`
myself: all four are identical in `size.log:1885-1888` and in `run.log`.

**One thing neither the logs nor the report says.** `run.log` ran at
22:13:21Z; the committed `size.log` ran at 22:33:08Z. So the file `run.log`
read at `records/logs/m1p3-p3/size.log` was a size attempt that was later
superseded and is not committed. The four hashes are identical in the
committed `size.log`, which I checked, so the assertion holds against the
committed file too and nothing is wrong with the measurement. But the
chronology is invisible: a reader would take "read out of it by command" to
mean out of the committed log. One sentence in the report, or in the final
`scan.log`, would close it.

---

## 3. Does the report's every number and quoted string appear in a primary?

I checked every figure and every quoted message in the draft report. All but
the ones listed under "not in a primary" below are backed, and I found no
arithmetic error.

### Verified

- Commit hashes: `2575083`/full sha, `79f1471`, `e127fc2`, `8c8a0ca`,
  `9339dca`, `52ef4e5`, `9bd9102`, `7702f6e` all resolve in `git log`;
  the four-commit range and its classification are printed at
  `gate.log:794-800`, and `scan.log:9-10` resolves the base from the doc
  commit's subject.
- Ruling 2 prose: `cells::decimal` as the only attribute-to-`u32` path -
  verified by my own grep of `cells.rs`. "refuses a leading zero except the
  number 0 itself" - `cells.rs:861-864`, and the other half pinned by
  `zero_is_a_number_where_zero_is_a_legal_index` (`tests.rs:1367-1383`).
- Typed-view test names and cases: every one appears at the `tests.rs` lines
  cited in section 1. `5410` for "the 5410 the corpus holds":
  `run.log:2240` and the run record's table.
- Mutations table: every row matches `test.log:2306-2492` as tabulated above.
- Probe figures: `exit 0`, `sheets 1`, `elements 3`, `items 2`, `cells 4`,
  `sum 3.75`, `text 14` at `test.log:2170-2178`; the `sum 3.76` control at
  `test.log:2177`; the breakdown `5 2 3 4 14` at `test.log:2179`. The
  `elements 11` reading in the size run: `size.log:1834`.
- Size table: 247348 / 249856 / 185152, 174560 / 176128 / 135200, 63336 /
  65536 / 44512, 19172 / 20480 / 6304 - all at `size.log:1786-1793` and
  `size.log:1874-1876`, and all re-derived by me from the eight probes'
  figures at `size.log:1526-1560`.
- Ceiling arithmetic: 393216 (`size.log:1787`, and the checkpoint ruling
  quoted in `specs/0010-reading-cells.md`, "Size"); 183980 read out of part
  2's own record and checked against the ruling (`size.log:1801-1803`, and
  `records/size-m1p2-2026-09-18-14c2e14.md:62`); 209236 and the check that the
  ruling's own subtraction holds (`size.log:1805`); 145868 and 143360
  (`size.log:1788-1789`); 63336 as part 3's share (`size.log:1804`).
- "three keep every part 2 figure; four moved by 32 section bytes and 0 file
  bytes": `size.log:1779-1783`. (The four also moved 32 in `.text`, which the
  report omits; not a discrepancy.)
- Corpus figures: the whole 23-key line and the 20 asserted spec totals at
  `run.log:2211-2269` (host) and `run.log:2371-2429` (i686), identical at
  `run.log:2622-2623`; `values` 2089191 and `empty` 5410 asserted by name;
  `items` 110582 and `bytes` 1497091; `numbers_parsed` 1305578, and
  1310988 - 5410 = 1305578 checks out.
- Test counts: 347 both targets and 337 at `9339dca`
  (`test.log:1877`, `:1890-1891`, `:2504-2506`); 263 library
  (`test.log:1878`, `:1890`); `cells` 85 against 75
  (`test.log:2091`, `:2095`, and `records/logs/m1p3-p2/test.log:1847`);
  integration 14 (`test.log:1867`); two ignored, both named as the 4 GiB
  zip64 cases (`test.log:2123-2129`).
- Gate: nine `PASS` lines named individually (`gate.log:1015-1023`,
  `:1026`).
- Negative log: 0 UNEXPECTED verdicts - I grepped the whole file; the only
  occurrences of the word are the helper definitions
  (`negative.log:121`, `:126`) and the control label at `negative.log:1768`.
  The catalogue coverage block is `negative.log:2369-2421`, ruling 2's four
  refusals are U15 to U18 (`negative.log:2341-2366`), the missing sheet part
  is U14 (`negative.log:2332-2339`), and the non-`sst` root and unknown cell
  type are counted in the catalogue table.
- The eight log and record byte counts and sha256 values in the report's
  table: I recomputed all sixteen values with `sha256sum` and `wc -c`. Every
  one matches. `scan.log` is 69873 bytes, sha256
  `536ce47f7b5035cd2390fd793f102de94bf21c3b63268073aed8ce31bc148d61`, as of
  the file I read; the report correctly does not quote it.
- Part 2's record hash `06980ea0...` and part 2's size log hash
  `2a26160d...`, cited at `size.log:1563-1564`: both verified by my own
  `sha256sum`.

### Not in a primary, or stated in a way a primary does not support

1. **`261979`, `336`, and "none non-canonical, no row numbered 0" (report,
   "Ruling 2", paragraph 3).** Not in any log or record of any phase; I
   grepped all of `records/`. `261979` matches the `<row>` count in
   `specs/0010`'s "Facts planning checked", and `336` is that section's 44 +
   292, so two figures trace to the spec, and `2094287` and `783275` are
   asserted in `run.log`. The claim "measured before the commit rather than
   discovered in the run" has no primary at all. See finding (a) in section 1.
2. **"Ruling 8 needed no change, and that is measured" (report heading).** The
   claim is TRUE - I verified it by grepping the crate - but no log prints the
   measurement. The word "measured" should go, or the scan should be printed.
3. **The doc-commit verification (report, paragraph after the commits table).**
   "checked by command before it was made ... `git diff --stat` over the three
   paths, and `sha256sum` over each, printed in the commit's own verification
   before it was written". No log prints that. `gate.log:821-829` hashes the
   eight diff files at the phase commit, which is a different thing.
   Planning independently holds the sha256 it recorded before sending the
   files, so this will be verified from planning's side, but the report's
   sentence is not backed by a primary of this phase.
4. **"Five local commits over `9339dca`" (report, first paragraph and the last
   section).** `git rev-list --count 9339dca..HEAD` is 4. The fifth is the
   records commit, which does not exist yet. The last section does say "the
   records commit included", so the intent is clear, but the first paragraph
   reads as a statement of fact about the repository and is not one yet.
5. **"(the records commit) | the five logs, ..." (commits table).** Six logs,
   not five: `scan.log` is one of them. See section 2.
6. **"Two logs were rerun for a wrong string, not a wrong measurement"
   (findings).** Two problems with this heading. First, it then describes
   THREE logs: `test.log` attempt 1, `run.log` attempt 1 and `size.log`
   attempt 2. Second, `test.log` attempt 1's fault was not a wrong string.
   `test.log:4` says the control "was written 'yn ! probe_control', and bash
   answered '!: command not found', so the control printed no while comparing
   nothing": that is the SECOND shape `specs/0001` forbids, and the effect was
   a control that asserted nothing, which is a broken measurement. The report
   files the chained-form defect in `size.log` as P3-F1 with the right
   gravity; it should not then file the leading-bang defect in `test.log`
   under "a wrong string". I would name both as instances of the forbidden
   forms, one caught by the phase's own scan grep and one by the control's own
   output.
7. **"145868 section bytes and 143360 file bytes under it".** Both figures are
   printed (`size.log:1788-1789`) so the report is faithful to the log, but
   the arithmetic subtracts a FILE delta from a ceiling the ruling states in
   SECTION bytes, and `size.log:1795` then asserts "the budget figure is under
   393216, by sections and by file size". The file-size headroom is not a
   figure the ruling defines. Both the log and the report carry it; planning
   may want the sentence narrowed to sections, with the file delta beside it
   as the spec's "with the file delta beside it" asks, rather than as a second
   comparison against the same ceiling.

### Where I agree with the report's own findings

- **P3-F1** is a real finding and correctly described, except that its key
  measurement (`chained=1` for `size.log`) exists only as prose in
  `size.log:10` and the scan attempt that produced it is unhashed and
  unnamed. See section 2.
- **"The spec says 'the eight earlier probes'; there are seven."** Correct. I
  counted part 2's record myself: seven probes at
  `records/size-m1p2-2026-09-18-14c2e14.md:25-31`. The log's handling is
  honest and printed. Planning should correct the spec's wording.

---

## Summary of what I would change before this goes to planning

Ordered by weight.

1. `records/logs/m1p3-p3/scan.log:11` names the phase as P2 (from the script's
   own `echo` at `scan.log:249`). The scan log is being rerun anyway.
2. The report's "Ruling 2" paragraph presents four corpus counts and a
   canonicality claim that appear in no primary, with the words "measured
   before the commit". Either print the scan or drop the sentence; the
   conclusion survives on `run.log`'s corpus pass alone and should be rested
   on that.
3. "Ruling 8 needed no change, and that is measured": the claim is true, the
   measurement is not printed. Drop "and that is measured".
4. The superseded scan attempt that read `chained=1` is neither hashed nor
   named as unhashed, and the committed `scan.log` has no rerun header.
5. The report never says the committed `scan.log` reads `RESULT: FAIL` with
   one structural check failed, nor that it will be rerun with the check
   answers present.
6. Six logs, not five, in the records-commit row.
7. "Two logs were rerun for a wrong string" lists three logs and
   mischaracterises `test.log` attempt 1, whose defect was a forbidden form
   that made a control assert nothing.
8. "Five local commits over `9339dca`" is four until the records commit exists.
9. `cells.rs:113`, "`<f>`'s `ca`, `aca` and the rest": `ref` is modelled as
   `Formula::reference`. One word.
10. The size figure for "part 3's own cost" does not include `Cell::boolean`
    or `Cell::error`, which the probe never calls. The probe's own doc comment
    is careful; the report's "so no path is dead" is not.
11. `size.log:1479` and `:1562` say "seven" where eight probes are built and
    checked.
12. `run.log` read a `size.log` that was later superseded; the four hashes are
    identical in the committed one, which I verified, but the chronology is
    not stated anywhere.

Nothing I found contradicts the phase's substantive results. The code does
what the spec's P3 section asks and nothing else; the eleven mutations are
honest and two of them reproduce byte for byte in an isolated tree; the
release-binary hash comparison is a real comparison; and every size, corpus,
test-count, byte-count and sha256 figure in the report that I could trace to a
primary traced correctly.
