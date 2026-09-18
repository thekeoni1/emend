# Independent check answer: emend M1 part 3 P1 (check 1)

Note on inputs: the harness placed `CLAUDE.md` in my context. I have read it as
context for the project's conventions and I do not use it as evidence for any
finding below; every finding cites a repository file, a log line, git, or a
command I ran myself. I did not read anything under the corpus directory and I
did not run the corpus runner. Commands I ran myself: `git log`, `git show`,
`git diff`, `git status`, `git for-each-ref`, `wc -c`, `sha256sum`,
`/usr/bin/grep`, `cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`.

---

## 1. Does the diff do what the spec says, and nothing else?

### Commit structure

`git log --reverse --format='%H %s' 7702f6e..HEAD` gives exactly four commits,
matching the report's table row for row:

    7175092 M1 part 3 P1: commit planning docs (part 3 approved, the size decision)
    04113ac M1 part 3 P1: the items part 2 left, and nothing of P1's feature
    8f2364d M1 part 3 P1: the shared strings table, read through the package
    001fe24 M1 part 3 P1: the self-test log's invocation line names the launch form

`git show --stat 04113ac` touches only `crates/emend/src/package/tests.rs`,
`crates/emend/src/xml.rs`, `scripts/gate-self-test.sh`,
`tools/emend-copy/src/main.rs` - no `cells.rs`, no `lib.rs`, no
`package.rs`. So "the items part 2 left, and nothing of P1's feature" holds as
stated; I checked it by file list and not by the message.

**Disagreement, on the justification for the third code commit.** The draft
report says at lines 19-23: "`001fe24` is a third code commit, not in the
spec's two: part 2 P3 ruling 7 asks that the self-test log's invocation line
name the launch form 'from part 3'". That attribution is wrong about where the
obligation is written. `specs/0010-reading-cells.md:307-308` puts it inside
P1's FIRST code commit paragraph: "The run script's faulty fixture drives every
counter by name and the self-test invocation line names the launch form, from
this phase on (rulings 3 and 7)." So the CONTENT of `001fe24` is squarely
inside the spec's first code commit; what departs from the spec is only that it
landed as a separate commit rather than inside `04113ac`. The report's reason
for the separation (the obligation was reached after `04113ac` was already
made) is consistent with what I can see - both commits touch
`scripts/gate-self-test.sh`, `04113ac` for the rewrapped comments and `001fe24`
for `LAUNCH_NOTE`, and the self-test ran at `001fe24`
(`self-test.log:2`) - and the amend-freedom clause at
`specs/0001-bootstrap.md:76-79` would have allowed squashing them. I record the
departure as a departure and the misattribution as a defect in the report's
prose, not in the code.

### P1's first code commit, clause by clause (`specs/0010:292-308`)

1. **"the count removed from `xml.rs` ('the other package messages')"** -
   carried. `crates/emend/src/xml.rs:44-46` now reads "catalogued with the
   other package messages"; the diff removes "the fourteen other".
2. **"with the `include_str!` set-equality test over the catalogue and the
   `Error::Package` literals of `package.rs` and `xml.rs`, `\[` unescaped
   (ruling 2)"** - carried.
   `crates/emend/src/package/tests.rs:2118-2161`
   (`the_catalogue_and_the_source_hold_the_same_messages`) uses
   `include_str!("../package.rs")`, `include_str!("../xml.rs")` and (added in
   `8f2364d`) `include_str!("../cells.rs")`, compares `BTreeSet`s in both
   directions, and asserts calls == literals and non-emptiness.
   `string_literal` at `package/tests.rs:2057-2078` unescapes `\[` and `\]` and
   leaves every other escape with its backslash; the two extractors are
   `catalogued` at `package/tests.rs:2082` and `raised` at
   `package/tests.rs:2100`. Its control,
   `the_catalogue_extractors_can_fail` at `package/tests.rs:2163-2196`, is
   present and plants a message into each extractor.
3. **"the `Default`-spelling removal test, a content types part whose `Default`
   elements carry a space before the slash, one `Override` removed, every
   `Default` back byte for byte (ruling 4)"** - carried.
   `crates/emend/src/package/tests.rs:2199-2246`
   (`removing_an_override_keeps_both_default_spellings`). It runs both orders
   of the spaced and tight spellings and asserts the fixture still carries both
   forms at `package/tests.rs:2220-2224`, which is the thing that would
   otherwise rot. Strictly the spec says "`Default` elements carry a space"
   (plural); the test has one spaced and one tight, which covers more than the
   literal reading and is what ruling 4's own wording about "the two
   SPELLINGS" asks for. I see no defect here.
4. **"`--drop-calc-chain` reading the content types part before deciding the
   edit is empty, so a package with no `[Content_Types].xml` is refused ...
   (ruling 6)"** - carried. `tools/emend-copy/src/main.rs` adds
   `package.content_types()?;` in `drop_calc_chain` immediately after the
   workbook part is read, with the reason in the doc comment. The negative log
   proves the inversion at `records/logs/m1p3-p1/negative.log:1791-1803`: D5
   exits 2 with `error: no [Content_Types].xml part`, and the log reads part 2
   P3's opposite verdict out of that committed log
   (`negative.log:1802-1803`).
5. **"the two overlong comment lines in `scripts/gate-self-test.sh` rewrapped
   (ruling 7)"** - carried; the `04113ac` diff rewraps the H2 and H3 comment
   blocks and changes no code in them.
6. **"The run script's faulty fixture drives every counter by name (ruling
   3)"** - carried, and visible only in `run.log` because the run script is
   not a repository file: `records/logs/m1p3-p1/run.log:940-946` prints
   `strings_counts` over a five-line fixture that moves every counter and puts
   the pairs in the other order, with "strings_counts read the pairs by name
   and not by position: as-expected".
7. **"The gate-file change runs the self-test and commits `self-test.log`"** -
   carried; `records/logs/m1p3-p1/self-test.log` exists and
   `records/logs/m1p3-p1/gate.log:788-807` checks the gate file is in the diff
   and the self-test log covers it.

### P1's second code commit (`specs/0010:310-323`)

`cells::SharedStrings` (`crates/emend/src/cells.rs:60-206`),
`Package::shared_strings` (`crates/emend/src/package.rs:557-567`) and
`--shared-strings` (`tools/emend-copy/src/main.rs`, `Operation::SharedStrings`
and `shared_strings_line`) are all present, and `lib.rs` re-exports
`cells::SharedStrings` alone, which is the right subset for P1 given the
spec's full list at `specs/0010:85-88` names types P2 and P3 add. The corpus
run is recorded as `records/m1p3-p1-2026-09-18-001fe24.md` and `.txt`, the
naming `specs/0010:322` asks for.

### The spec's twelve test cases, one by one

All twenty tests are in `crates/emend/src/cells/tests.rs`. Every one of them
goes through `Package::shared_strings` via `table()`
(`cells/tests.rs:100-104`), `refused()` (`106-111`) or `opened()` (`95-97`), so
the report's claim that none of them calls `SharedStrings::read` directly is
correct.

| spec case (`specs/0010:313-322`) | test | verdict |
| --- | --- | --- |
| a plain item | `a_plain_item_is_its_text` (`cells/tests.rs:147-153`) | covered |
| an item of runs whose `<t>` texts concatenate | `the_runs_of_an_item_concatenate` (`155-165`) | covered; the fixture also carries an `<rPr><b/></rPr>` so a run's properties cannot be mistaken for its text |
| an item with `<rPh>` skipped | `a_phonetic_subtree_is_skipped` (`167-177`) | covered; `the_phonetic_skip_ends_at_its_own_end_tag` (`179-188`) adds the depth case |
| a character reference and a numeric reference unescaped | `references_are_expanded` (`190-198`) | covered; all five predefines plus `&#65;` and `&#x42;` |
| leading and trailing spaces kept with and without `xml:space` | `spaces_are_kept_with_and_without_xml_space` (`200-208`) | covered |
| an empty `<t/>` and an empty `<si/>` both an empty string | `an_empty_text_and_an_empty_item_are_both_the_empty_string` (`210-220`) | covered; the third item guards against index shift, which is more than asked |
| `count` and `uniqueCount` disagreeing and ignored | `count_and_unique_count_are_ignored` (`222-233`) | covered |
| a workbook with no shared strings relationship giving an empty table | `a_workbook_with_no_shared_strings_gives_an_empty_table` (`273-284`) | covered |
| the root of the wrong name | `a_root_of_the_wrong_name_is_refused` (`298-305`) | covered |
| non-UTF-8 text | `text_that_is_not_utf8_is_an_xml_error` (`339-350`) | covered, with one reservation below |
| an unknown entity refused | `an_unknown_entity_is_refused` (`329-337`) | covered |
| `get` at the length `None` | `get_past_the_end_is_none` (`265-271`) | covered; asserts `get(table.len())` and `get(u32::MAX)` |

Twelve for twelve. Nothing in the spec's P1 list is missing.

The eight tests beyond the twelve are `the_phonetic_skip_ends_at_its_own_end_tag`
(`179`), `text_in_another_namespace_is_not_the_item_s_text` (`235`),
`the_strict_namespace_is_accepted` (`255`),
`a_relationship_naming_no_entry_gives_an_empty_table` (`286`),
`a_root_in_another_namespace_is_refused` (`307`),
`a_part_with_no_root_at_all_is_refused` (`318`),
`the_fixture_builder_makes_a_package_the_reader_accepts` (`352`) and
`the_table_is_read_afresh_on_every_call` (`361`).

**Small disagreement with the report's accounting.** The report says at line 61
"The spec's twelve cases, plus five the writing raised", and names five. There
are eight beyond the twelve, not five. The three the sentence omits are all
described elsewhere in the report - the fixture-builder control in the very
next sentence (report line 67), the `<rPh>` depth test at report line 42 and
the extLst namespace test at report line 74 - so nothing is hidden, but
12 + 5 does not reach the 20 the same paragraph opens with, and a reader of the
report alone cannot reconcile the numbers. The count 20 itself is right: I
counted twenty `#[test]` functions in the file and `test.log:1389` prints
"cells module tests on the host: 20".

### Does any test assert a POSITION or a MECHANISM rather than a PROPERTY?

I went through all twenty with this specifically in mind. No test asserts an
index, an offset, an event order, a call count, or an internal field: `items()`
(`cells/tests.rs:141-145`) reads only through the public `len()` and `get()`,
`assert_package` (`124-129`) compares the message text, and `assert_xml`
(`133-138`) matches the variant and deliberately ignores the index. Two things
are worth putting in front of planning anyway:

- **`the_table_is_read_afresh_on_every_call` (`cells/tests.rs:361-373`) names a
  mechanism its assertions do not pin.** It asserts `first == second` and
  `items(&first) == ["x"]`. An implementation that cached the table and handed
  back a clone would pass this test unchanged; so would one that cached and
  lent, once the borrows were adjusted. What actually enforces "owned" is that
  `first` and `second` are two live values at once, which is a compile-time
  fact and not an assertion. This is not the recorded defect class (it asserts
  a property and names a mechanism, rather than asserting a mechanism), but the
  name promises cover the body does not give, and the report repeats the
  promise at line 63 ("the table read afresh on two calls"). A test that, say,
  read the table, then asserted a second call still succeeds after the first is
  dropped, would be no stronger; there may be no honest assertion available
  here, in which case the right fix is the name.
- **`text_that_is_not_utf8_is_an_xml_error` does not pin which entry index the
  error carries.** `assert_xml` (`cells/tests.rs:131-138`) matches `Error::Xml(_)`.
  The module header at `crates/emend/src/cells.rs:30` claims "[`Error::Xml`]
  carries the entry's index, as in every part", and
  `crates/emend/src/package.rs:565-566` passes `index` into `Stream::new` and into
  `SharedStrings::read`. `package::tests` has an `assert_xml_index` helper for
  exactly this (used at `package/tests.rs:2046`), so the tool existed and was
  not used. The claim in the doc comment is therefore unpinned by any test in
  this module. Minor, and the negative log does not cover it either.

Neither of these is a position/mechanism assertion of the kind the project has
recorded seven times. I did not find one.

### Things in the diff the spec did not ask for

- `SharedStrings::is_empty()` (`cells.rs:77-79`). Not in the spec's API list at
  `specs/0010:145-149`. It is effectively forced: `cargo clippy -D warnings`
  (which the gate runs, `gate.log:1036`) fires `len_without_is_empty` on a
  public `len()`. Benign, and used by two tests.
- `#[derive(Clone, Debug, Default, PartialEq, Eq)]` on `SharedStrings`
  (`cells.rs:60`). `Default` is what `Package::shared_strings` returns for the
  two empty-table cases; `PartialEq` is what
  `the_table_is_read_afresh_on_every_call` compares with. Not spec-named,
  benign.
- **A behaviour the spec did not rule on: a `sharedStrings` relationship whose
  target names no entry answers with an empty table.**
  `crates/emend/src/package.rs:562-564` returns `SharedStrings::default()` when
  `index_of` misses, and `a_relationship_naming_no_entry_gives_an_empty_table`
  (`cells/tests.rs:286-296`) pins it. `specs/0010:147-149` specifies only the
  no-RELATIONSHIP case. The spec's companion clause for `Package::cells`
  (`specs/0010:184-186`) says the missing entry case is refused with "no such
  part", so the two sibling methods will answer the same package differently.
  The report mentions the test (line 64) but does not raise the asymmetry as a
  question. I think planning should rule on it before P2 writes
  `Package::cells`, because P2's spec text commits to the other answer.
- `Error::Report`'s Display text changed from "cannot write the survey line" to
  "cannot write the report line" (`tools/emend-copy/src/main.rs`). A
  user-visible string the spec did not ask to change, though the shared
  reporting path makes it accurate. No log or test asserts the old text
  (`negative.log:1379-1390` reads the messages it asserts from the source), so
  nothing breaks. Worth naming because the project treats message strings as
  interface.
- `expand()` (`cells.rs:214-231`) and the `phonetic` depth are implementation,
  not additions to the interface.

I found nothing else in the diff outside the spec. `ROADMAP.md`,
`specs/0001-bootstrap.md`, `specs/0008-zip-layer.md` and
`specs/0009-package-layer.md` are all in the doc commit `7175092` only, which
is what `specs/0001-bootstrap.md:264-266` prescribes.

### Reader behaviour I checked by reading the code

I traced the `<rPh>` depth by hand over `<si><t>a</t><rPh sb="0" eb="1"><r><t>skipped</t></r></rPh><t>b</t></si>`
and over nested `<si>`/`<t>` inside `<rPh>`: every non-empty start inside the
skip increments (`cells.rs:132-134`) and every end decrements
(`cells.rs:169-170`), and empty elements increment nothing and produce no end,
so the depth is consistent for arbitrary nesting. The `Event::GeneralRef` guard
(`cells.rs:192`) means an unknown entity inside a skipped `<rPh>` is silently
ignored rather than refused; that matches the doc comment at
`cells.rs:189-191` and I do not think it is a defect.

One point supporting the report's question 2, from the code rather than the
report: a FOREIGN-namespace element inside `<si>` is not skipped with its
subtree - `cells.rs:116-152` simply matches no branch for it - so a
main-namespace `<t>` nested inside a foreign `<ext>` IS read as the item's
text. `specs/0010:165-167`, describing the cell reader, says an element in
another namespace "is skipped with its subtree". The shared strings reader does
not do that, and P2's reader is specified to. The report raises the question;
I add that the spec's own `Cells` clause is evidence for which answer planning
probably intends, and that the two readers are about to diverge on it.

### My own gate-equivalent runs

- `cargo test --workspace`: **280 passed; 0 failed; 2 ignored**. Independently
  confirms `test.log:895` and `test.log:1221`.
- `cargo fmt --all --check -- --color never`: exit 0, no output.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.

---

## 2. Do the logs contain every line the spec requires?

Five logs plus the self-test log are present; I list what I checked and then
what is missing.

### Satisfied

- **First lines.** Every one of the six logs opens `command: git rev-parse
  HEAD` then an `about:` line naming the commit and `HEAD at run:`
  (`gate.log:1-2`, `test.log:1-2`, `negative.log:1-2`, `run.log:1-2`,
  `self-test.log:1-2`, `scan.log:1-2`), satisfying
  `specs/0001-bootstrap.md:43-46` on the commit and HEAD - with the exception
  recorded below.
- **The script printed into the log.** `gate.log:8`, `test.log:7`,
  `negative.log:8`, `run.log` (`## script` section), `scan.log:24`. `self-test.log`
  prints no script; it is the output of the committed gate file
  `scripts/gate-self-test.sh`, whose bytes are hashed at `scan.log:880`, and
  `records/logs/m1p2-p3/self-test.log` prints none either, so this is
  established practice and not new.
- **Every compound check in a named function.** `scan.log:1100-1117` runs both
  forbidden-shape greps over all five siblings with six controls and reports
  `chained=0 leading-bang=0` on each. Because a scan cannot read itself, I ran
  the same two patterns myself with `/usr/bin/grep -E` under `LC_ALL=C` over all
  SIX logs including `scan.log`, with a positive control fixture that scored 1:
  every log scored 0 on both forms. So the rule holds for `scan.log` too, which
  its own log cannot say.
- **Every scan form with a positive control in the same command form.**
  `scan.log:747-786` gives a control for `f_nonascii`, `f_ctrl`, `f_emdash`,
  `f_session`, `f_p6`, `f_pre`/`f_pre1`/`f_pre2`/`f_pre3`, `f_host`, `f_home`,
  `f_deny`, `f_email`, plus a clean-file negative control for each, ending
  "controls: all as expected".
- **Scan log written last, hashing every committed sibling.** `scan.log` ran at
  16:56:06 (`scan.log:6`), after `run.log` at 16:54:56 (`run.log` result
  section), and hashes all five siblings at `scan.log:997`, `1007`, `1017`,
  `1027`, `1037` and the record pair at `1049`, `1059`. I recomputed all seven
  with `sha256sum` and every one matches. `check answers scanned: 0`
  (`scan.log:1048`) correctly marks this as the pre-check run; the committed
  scan must be the later one, per `specs/0001-bootstrap.md:34-37`.
- **Self-scan with the pre-P6d forms before printing itself, with a control.**
  `scan.log:19-22`: `pre-P6d forms over this script's source: count=0`, then the
  same form over a copy with one concatenated shape appended at `count=1`. This
  is the rule `specs/0001-bootstrap.md:162-168` added, and it is honoured.
- **Every sha256 over a file that exists at run time** - for the sibling hashes,
  the record pair (`run.log`, `sha256sum "$REC" "$RECT"`), the four binaries
  (`gate.log:1059-1062`, `negative.log:1342-1345`) and the diff files
  (`scan.log:810-930`). See the gap below for the rerun hashes.
- **The gate-file change requiring a committed self-test log.**
  `gate.log:788-807` names `scripts/gate-self-test.sh` as the one gate file in
  the diff, prints the self-test log's sha256 (`01162f86...`), bytes (15781)
  and first four lines, and asserts each of: the commit it ran at is in the
  phase range; it records a pass; **gate-file commits after the self-test's
  commit: 0**; its phase line names this phase. That is the full property, and
  it is checked from the log rather than asserted in prose.
- **The self-test run covers the phase.** `self-test.log:2` names
  `001fe24`, the phase's last commit; `self-test.log` ends
  `self-test: 50 cases, 0 unexpected, 0 errors` and `RESULT: PASS`; and
  `gate.log:802-803` prints `gate-file commits after the self-test's commit: 0`.
  All three conditions hold.
- **The absence of `size.log` is consistent with the spec.**
  `specs/0010-reading-cells.md:280-284` lists the per-phase logs as `gate.log`,
  `test.log`, `negative.log`, `run.log`, `scan.log`, "and `size.log` in P3";
  `specs/0010:206-234` ("Size") and the P3 phase text at `specs/0010:361-372`
  put part 3's size record at the part's end. `self-test.log` is additional and
  required by `specs/0001-bootstrap.md:113-122` because a gate file changed.
  So six files for P1, five of them scan.log's siblings, is exactly right. The
  scan script states the same reasoning at `scan.log:66-71` and asserts the
  count at `scan.log:1047` ("sibling logs scanned: 5 (expected 5)"), which is
  the right way round: a count left at part 2 P3's six would have failed here.
- **Negative tests in a scratch clone with the gate's binaries copied in**
  (`negative.log:1338-1347`), and `negative.log:1958-1962` shows the main
  working tree held nothing but this phase's logs.

### Missing or defective

**(a) `scan.log`'s first lines name the WRONG PHASE and the WRONG SPEC.** This
is the one finding I would put in front of planning first, and the draft report
does not contain it because it left scan.log as PENDING.

    scan.log:2   about: M1 part 2 P3 phase diff 7702f6e3...0..001fe24...
    scan.log:6   phase: Milestone 1 part 2 P3 (specs/0009-package-layer.md) scan.log; run at 2026-09-18T16:56:06Z

The commit range and HEAD are right; the phase label and the spec reference are
part 2 P3's, inherited from the script this one was derived from. The two
`echo`s are hard-coded in the printed script at `scan.log:210` and
`scan.log:214`. The script's own change list claims the opposite at
`scan.log:49`: "1. The phase, log directory and doc commit pattern are part 3
P1's." The doc commit pattern and log directory ARE part 3 P1's
(`scan.log:92`, `scan.log:383-412`); the phase line is not. Against
`specs/0001-bootstrap.md:43-46` ("The first lines of every log ... as every
phase has written them"), and against the precedent that every other log in
this directory names `specs/0010-reading-cells.md`, this is a defect in the
committed-to-be log. Because the scan must be re-run after this answer is saved
(`specs/0001-bootstrap.md:34-37`), the fix costs nothing if the two `echo`
lines are corrected before that run. If it is not corrected, the phase would
commit a scan log that tells a reader it is part 2 P3's.

**(b) The rerun sha256s are not printed by a command run over a file.**
`specs/0001-bootstrap.md:180-186` - a rule this very phase's doc commit added -
says "Every sha256 a log cites, a superseded attempt's included, is printed by a
command run over a file that exists at run time, never typed from memory."
`negative.log:4` cites `90379bfa...` and 115909 bytes, and `run.log:4` cites
`09b1cdca...`/61198, `76ef6f56...`/61267 and `db315fda...`/65205. Both come from
an environment variable the caller sets: `negative.log:161` and `run.log:326`
are both `if [ -n "${RERUN_NOTE:-}" ]; then echo "rerun: $RERUN_NOTE"; fi`, and I
find no `sha256sum` or `wc -c` over a kept attempt anywhere in either log
(`grep -n "09b1cdca\|76ef6f56\|db315fda\|90379bfa" *.log` finds only the header
lines). So four sha256s and four byte counts in committed logs have no command
in any log that produced them. I cannot tell from the files whether they were
computed with `sha256sum` and pasted into `RERUN_NOTE` or typed; that is exactly
the ambiguity the new rule exists to close. The fix that would satisfy the rule
as written is for the script to take the kept attempts' PATHS and hash them
itself. I raise this as a gap against the rule and not as an accusation:
nothing suggests the hashes are wrong, and the part 2 close-out ruling
(`specs/0009-package-layer.md`, the verification paragraph) shows the session
self-reported the previous instance of this class.

**(c) `negative.log`'s section heading is off by one.** `negative.log:1600`
reads `## N23 to N27: --shared-strings among the operations, refused in pairs`,
and the section then runs N23, N24, N25, N26, N27 and N28
(`negative.log:1601-1636`) - six cases. The report says "N23 to N28" and "all
six", which is what the log's CASES say; the log's own HEADING says N27. A
reader indexing the log by its headings would not find N28. Cosmetic, but it is
a heading that disagrees with its contents in a committed log.

I found no other required line missing. In particular: the `SERIALIZED` line is
on `gate.log:4`, `test.log:4`, `negative.log:5`, `run.log`, `scan.log:7`; colour
is off (`gate.log`/`test.log` env lines); the departure `CARGO_BUILD_JOBS=4` is
declared at `gate.log:6`; `ANTHROPIC_API_KEY set in this shell: no` is at
`gate.log:5`; and `scan.log:1082-1098` runs the pre-P6d bare-shape control over
all 13 diff files and all 5 siblings and the 2 records, 20 files, count 0, with
the controls at `scan.log:756-759`.

---

## 3. Does the report's every number and quoted string appear in a primary?

### Verified in a primary (I checked each one)

| report figure | primary |
| --- | --- |
| commits `7175092`, `04113ac`, `8f2364d`, `001fe24` and their subjects | `gate.log:752-755`, `scan.log:790-793`, and `git log` |
| "Twenty in `crates/emend/src/cells/tests.rs`" | `test.log:1389`, `test.log:1392`; I counted 20 `#[test]` in the file |
| `self-test.log` 15781 bytes, sha256 `01162f86...`, "PASS, 50 cases, 0 unexpected, 0 errors" | `gate.log:791` (sha256 and bytes), `self-test.log` last two lines; sha256 also `scan.log:1037`; I recomputed both |
| `gate.log` sha256 `b281900a...` | `scan.log:997`; recomputed |
| `test.log` sha256 `ecd8a908...` | `scan.log:1007`; recomputed |
| `negative.log` sha256 `a516159d...` | `scan.log:1017`; recomputed |
| `run.log` sha256 `c3205791...` | `scan.log:1027`; recomputed |
| "Nine PASS lines", doc step by name, licences with quick-xml, four binaries executable on both targets | `gate.log:1033-1046`, `gate.log:1063` |
| base `7702f6e` resolved from git by the doc commit's subject | `gate.log:740-747` |
| "Three terms in the working-tree accounting, not part 2 P3's four" | `gate.log:530-533` (the script's own comment) and the sum at `gate.log:538-542`; the printed line has a total plus three addends, so the sentence is right as a statement about the accounting equation |
| "280 tests passed on the host and 280 on i686 ... 0 failed, 2 ignored, the same test NAMES" | `test.log:895`, `test.log:1221`, `test.log:1393`, and `the i686 run executed the same test names as the host run: yes`; I reproduced 280/0/2 myself |
| "The previous phase's total, 257, is read from `records/logs/m1p2-p3/test.log`" | `test.log:1252-1253` |
| "A new named-test block prints the cells module's 20 tests case by case" | `test.log:1368-1393` |
| "the tool's threshold did not move" | `test.log:271` ("at least twelve"), `test.log:1230-1237` (`integration null_rewrite 12`) |
| "Eight families" | `negative.log:16` |
| N23-N28 refusals, and the check that the usage text names the operation | `negative.log:1601-1638` |
| T1's printed line, byte-identical output | `negative.log:1805-1812` (`strings	items=2	bytes=2`) |
| T2 and T3 reaching both messages through the built binary | `negative.log:1814-1828` |
| T4 reporting `items=0 bytes=0` and exiting 0 | `negative.log:1830-1838` |
| "D5 inverted ... the SAME message ... reads part 2 P3's opposite verdict out of that committed log" | `negative.log:1791-1803` |
| "reads three source modules and two test files" | `negative.log:1389` (package.rs, xml.rs, cells.rs), `negative.log:1840-1843` (the two test files, each with its sha256) |
| the superseded negative attempt `90379bfa...`, 115909 bytes, never committed | `negative.log:4` (but see gap (b) above) |
| "870 entry rows per target, all `identical` ... no crc and no size differing" | `run.log:1026` and `run.log:1120` |
| `files_identical` yes, `order_preserved` yes, `lo` and `lo_in` ok on all 27 | `run.log:1017-1024` (host), `run.log:1111-1118` (i686) |
| host and i686 both 27 / 110582 / 1497107 / 54472 | `run.log:1066`, `run.log:1160`, and the two-target comparison in the tail |
| "The total is 110582, exactly the `<si>` figure `specs/0010`, 'The corpus', gives" | `run.log:1070` and `specs/0010-reading-cells.md:65` |
| "No workbook reported an empty table, and the largest table is smaller than the total" | `run.log` (`empty_tables=0`, `max_items=54472`) and the two named checks in the tail |
| "The two targets' lines are identical file to file" | `run.log` ("the two targets' lines are identical file to file: as-expected") |
| "stderr empty" | `run.log:1014`, `run.log:1108` |
| the three superseded run attempts and their hashes/bytes | `run.log:4` (but see gap (b)) |
| record pair 1175 / 1839 bytes, sha256 `adaef6ef...` and `9396eb5e...` | `run.log` record-pair section prints both `sha256sum` and `wc -c`; sha256s also at `scan.log:1049`, `1059`; I recomputed all four |
| specs/0010 gives 2748976 as the largest PART's bytes and 110582 as the `<si>` count over 27 parts | `specs/0010:40` and `specs/0010:65` |
| the self-test invocation line, quoted | `self-test.log:4`, character for character (the report wraps it over two lines) |
| "Nothing has been pushed; every commit below is local" | `git for-each-ref`: `refs/remotes/origin/main` is at `f4f48e8`, the part 2 close-out; `refs/heads/main` is at `001fe24` |
| the record `.txt` carrying `id`, `items` and the independent count per workbook | `records/m1p3-p1-2026-09-18-001fe24.txt` header row `id	items	own_si_count` |

### Figures and strings that do NOT appear in a primary

**(i) "The independent counter has five controls" (report line 256) is wrong;
there are four.** `run.log:948-963` is the whole section, and it names three
zip fixtures plus one non-zip file: three plain items, a workbook declaring no
shared strings part, the faulty fixture with the `sic` element, and a file that
is not a zip. `grep -n "si_count" run.log` returns exactly those four `note`
pairs (log lines 954-963; script lines 465-477). The log's own prose says
"Three fixtures built here as zips" (`run.log:950`). Five appears nowhere. The
substance of the clause - that one control carries an element whose name begins
with `si`, which an undelimited pattern would count - is true and is at
`run.log:958-961`.

**(ii) "P2 added two for `--survey` and P3 three for `--drop-calc-chain`"
(report line 143) misattributes the counts.** From the repository:
`tools/emend-copy/tests/null_rewrite.rs` has THREE `--survey` integration tests
(`the_survey_counts_the_map_and_edits_nothing:666`,
`the_survey_counts_an_override_that_names_no_entry:699`,
`the_survey_refuses_a_package_it_cannot_read:735`) and TWO
`--drop-calc-chain` ones
(`the_recipe_drops_the_chain_and_leaves_every_other_entry_alone:809`,
`the_recipe_leaves_a_package_with_no_chain_byte_for_byte:907`). By
`git log -S` on each: lines 666 and 735 came from `5fc1b3d` (part 2 P2), and
lines 699, 809 and 907 from `11de26e`/`e04db06` (part 2 P3). So part 2 P2 added
two `--survey` tests, and part 2 P3 added THREE integration tests of which only
TWO are `--drop-calc-chain`. There are two `--drop-calc-chain` integration
tests in the repository, not three. The clause's conclusion - that
`--shared-strings` has no integration test - is correct: `grep -rn
"shared-strings"` over `tools/` and `crates/` hits only
`tools/emend-copy/src/main.rs`. I also confirmed that
`the_operations_are_refused_unless_exactly_one_follows_the_paths`
(`null_rewrite.rs:511-573`) enumerates operation pairs and was NOT extended with
`--shared-strings`, which is consistent with the report's position that the
spec asked for none, and is the concrete thing planning would be adding in P2.

**(iii) Four byte counts in the report's log table are in no primary.** 62739
(`gate.log`), 91949 (`test.log`), 116638 (`negative.log`) and 66202 (`run.log`)
appear in no log in the directory: `grep -rn "62739\|91949\|116638\|66202"
records/logs/m1p3-p1/` returns nothing. I verified all four with `wc -c` and
they are correct. Only `self-test.log`'s 15781 is printed by a command
(`gate.log:791`), and the record pair's 1175/1839 are (`run.log`). Under the
question as put to me these are numbers that appear only in the report. They
are right, and a reader can reproduce them in one command, so I would call this
a small gap rather than a defect - but if planning wants every figure in the
report anchored, the scan script is the natural place to print `wc -c` beside
each sibling's sha256, which it already computes.

**(iv) Claims about mutation runs that no log prints.** Three of the report's
strongest statements are about experiments that left no printed measurement:

- "it was shown to bite by dropping the namespace guard, which made it read
  'keptnot text'" (report line 79) - the string `keptnot text` appears in
  commit message `8f2364d` and in no log.
- "The widened test was shown to bite in both directions on the real files
  before the commit ... it reported 'raised but not catalogued' and 'catalogued
  but never raised' in turn" (report lines 106-109) - those two strings are
  assertion messages in `package/tests.rs:2159-2160` and are quoted in commit
  messages `04113ac` and `8f2364d`; no log prints them as an observed result.
- "the test that drives it reads 'ab' where a flag reads 'askippedb' and an
  early exit reads 'a'" (report line 43) - `"ab"` is asserted at
  `cells/tests.rs:187` and the two counterfactuals are stated in the comment at
  `cells/tests.rs:183`; neither counterfactual reading was produced by any
  recorded run.

`specs/0001-bootstrap.md:190-192` says "Nothing is called measured unless the
log prints the measurement; a figure in a script's prose or comments is not
one." The report does not use the word "measured" for these, and the commit
messages carry them, which is the project's normal place for a pre-commit
mutation check. I name them so planning can decide whether that is enough,
because they are doing real evidential work in the report: the extLst
rewrite's soundness rests on the first one.

**(v) A wording problem, not a wrong number: "Five, not part 2 P3's six"
(report line 151) sits above a table of six rows.** The five is scan.log's
SIBLING count, which is right: `scan.log:1047` prints "sibling logs scanned: 5
(expected 5)" and `records/logs/m1p2-p3/scan.log:1145` prints 6 for the
previous phase. But `records/logs/m1p3-p1/` holds six logs, part 2 P3's held
seven (`gate`, `negative`, `run`, `scan`, `self-test`, `size`, `test`, plus two
check answers), and the report's own table lists six. Read literally the
sentence contradicts the table under it. The reasoning it gives - that
`specs/0010`, "Size", defers part 3's size record to after P3 - is correct
(`specs/0010:280-284`, `specs/0010:361-372`).

**(vi) Unverifiable by construction, and stated as history rather than
measurement.** "`cells.rs`'s header read 'The corpus's largest is 2748976 bytes
of XML in 110582 items'" (report line 82) describes a pre-commit state that is
in no commit and no log; `git log -S` finds no committed form of that sentence.
The current text is at `crates/emend/src/cells.rs:34-41` and does separate the
two measurements, as the report says. Likewise "This was probed against the
crate before the reader was written, not assumed from the documentation"
(report line 39) has no primary; what I can confirm is that the code handles
`Event::GeneralRef` (`cells.rs:192-197`) and that `references_are_expanded`
passes, which is consistent with the claim.

### One thing the report found that planning should act on beyond the report

The report's second confession (lines 81-88) says `specs/0010` gives 2748976 as
a part's byte size and 110582 as the corpus `<si>` count, and that welding them
into one sentence was its own error. That is right about the facts
(`specs/0010:40`, `specs/0010:65`) - but the welded sentence is also in the
SPEC, not only in the code it fixed. `specs/0010-reading-cells.md:201` reads:

    `Vec`; the corpus's largest is 2748976 bytes of XML and 110582 items,

which says of "the corpus's largest" table both that it is 2748976 bytes and
that it holds 110582 items, when `run.log:1066` measures the largest single
table at 54472 items. The implementing session corrected `cells.rs` and left
the spec's sentence as planning wrote it, which is the right boundary to
respect, but the report does not say the spec carries the same conflation. It
does, and part 3's "32-bit and streaming" section will be read again in P2 and
P3.

---

## Summary

1. **The diff does what P1's spec says.** All seven clauses of the first code
   commit and all three deliverables of the second are present; all twelve of
   the spec's shared-strings cases are covered by a named test, with eight more
   beyond them; no test asserts a position or a mechanism in place of a
   property. Departures: `001fe24` is a third code commit whose content the
   spec put in the first (the report misattributes the obligation to ruling 7
   alone); `is_empty`, the derives, and the `Error::Report` string change were
   not asked for and are benign; and the "relationship naming no entry gives an
   empty table" behaviour is unruled and will contradict the spec's own
   `Package::cells` clause in P2.
2. **The logs satisfy the rules with three exceptions.** `scan.log`'s `about:`
   and `phase:` lines name "M1 part 2 P3" and `specs/0009-package-layer.md`
   instead of this phase and `specs/0010` (`scan.log:2`, `scan.log:6`, from
   hard-coded `echo`s at `scan.log:210` and `scan.log:214`), while the script's
   own change list claims the phase lines were updated (`scan.log:49`). The four
   superseded-attempt sha256s in `negative.log:4` and `run.log:4` reach the log
   through `RERUN_NOTE` and are printed by no command over a file, which is the
   rule this phase's own doc commit added (`specs/0001-bootstrap.md:180-186`).
   `negative.log:1600` is headed "N23 to N27" over six cases N23-N28. The
   absence of `size.log` is correct and well-argued.
3. **Most of the report's figures are anchored; four items are not.** "five
   controls" for the independent `<si>` counter should be four
   (`run.log:948-963`); "P2 added two for `--survey` and P3 three for
   `--drop-calc-chain`" should be two for `--survey` from P2 and two
   `--drop-calc-chain` tests from P3 (plus a third `--survey` one); the four log
   byte counts 62739/91949/116638/66202 are in no log, though correct; and three
   mutation-run results ("keptnot text", the two catalogue-test messages, the
   "askippedb"/"a" counterfactuals) live in commit messages and comments rather
   than in a printed measurement. Separately, the accounting "twelve cases plus
   five" omits three of the eight extra tests, and "Five, not part 2 P3's six"
   is a sibling count sitting above a six-row table.
