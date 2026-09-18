# Independent check 2: emend M1 part 3 P1

I have not seen the conversation that produced this work. Everything below
comes from the files named. CLAUDE.md is in my context because the harness
puts it there; I have read it for the working rules but I do not use it as
evidence of what was done.

Inputs read: `specs/0010-reading-cells.md`, `specs/0001-bootstrap.md`
("Rules for every phase"), `specs/0008-zip-layer.md` and
`specs/0009-package-layer.md` (the rulings this phase carries),
`git diff 7702f6e..HEAD` and the four commits individually, the six logs
in `records/logs/m1p3-p1/`, the record pair, and the draft report.

I also did work of my own, in a copy of the tree made with `git archive
HEAD` into `<scratch>` with a separate `CARGO_TARGET_DIR`, so that nothing
in the repository and none of the binaries the logs hash were touched:
I ran the workspace tests, `cargo fmt --all -- --check` and
`cargo clippy --workspace --all-targets -- -D warnings`; I ran four
behaviour probes against `SharedStrings::read`; and I ran three source
mutants past the committed tests. Those runs are named where they matter.
I changed nothing in the repository and committed nothing.

---

## 1. Does the diff do what the spec says, and nothing else?

### The commit structure

`specs/0010-reading-cells.md:291-323` describes P1 as one doc commit and
TWO code commits. The range holds one doc commit and THREE code commits
(`git log --reverse 7702f6e..HEAD`):

- `7175092` planning docs, five files, unedited - matches the spec's doc
  commit and `specs/0001-bootstrap.md:264-266` (planning's pending doc
  changes ride as the first commit of the next phase).
- `04113ac` the carried items. Touches
  `crates/emend/src/package/tests.rs`, `crates/emend/src/xml.rs`,
  `scripts/gate-self-test.sh`, `tools/emend-copy/src/main.rs` and nothing
  else. I checked the `main.rs` half of it (`git show 04113ac --
  tools/emend-copy/src/main.rs`): it carries the content-types read and
  the doc text, and NOT `--shared-strings`. The commit's claim to hold
  "nothing of P1's feature" is true.
- `8f2364d` the feature: `cells.rs`, `cells/tests.rs`, `lib.rs`,
  `package.rs`, `package/tests.rs` (+20 lines, the `CELLS` `include_str!`)
  and `main.rs`.
- `001fe24` the self-test invocation line, `scripts/gate-self-test.sh`
  alone.

**Disagreement with the report, section "What P1 asked for, and what was
done".** The report justifies the third code commit like this: "It is
separated from `04113ac` because that commit was already made when the
obligation was reached." That justification does not hold.
`specs/0001-bootstrap.md:76-80` says in terms: "Before the report, the
implementing session may amend or squash its own unpushed, unverified
commits freely". Nothing was pushed and nothing was verified, so amending
`04113ac` (and rebasing `8f2364d` onto it) was available and is what the
spec's two-commit shape asked for; `specs/0010:306-308` puts the
launch-form change in the first code commit's list, not in a third. The
departure is harmless in substance - `gate.log:756-759` classifies all
three as code-alone, the structure check passes, and the self-test ran at
`001fe24` with no gate file changed after it - but the reason given for it
is wrong, and planning should rule on the shape rather than accept the
stated reason. Everything else about the commit structure is as the spec
asks.

### The first code commit, clause by clause (`specs/0010:292-308`)

1. "the count removed from `xml.rs` ('the other package messages')" -
   carried. `crates/emend/src/xml.rs:45-46` now reads "catalogued with the
   other package messages"; the words "fourteen other" are gone.
2. "with the `include_str!` set-equality test over the catalogue and the
   `Error::Package` literals of `package.rs` and `xml.rs`, `\[`
   unescaped" - carried, `crates/emend/src/package/tests.rs:2049-2116`
   (`string_literal`, `catalogued`, `raised`) and `:2118-2161`
   (`the_catalogue_and_the_source_hold_the_same_messages`). The bracket
   unescaping is at `:2053-2054` and is pinned at `:2193-2196`.
   I reproduced the report's claim that the test bites in both
   directions rather than taking it on trust. In `<scratch>` I planted
   `Error::Package("a planted uncatalogued message")` in `cells.rs`: the
   test failed with `raised but not catalogued: ["a planted uncatalogued
   message"]`. I then restored `cells.rs` and planted a catalogue line in
   `package.rs`'s header: the test failed with `catalogued but never
   raised: ["a planted catalogue entry"]`. The report's sentence in "The
   catalogue" is accurate. Note that the run itself is recorded in no
   primary; the only record of it before mine was the commit message of
   `04113ac`.
3. "the `Default`-spelling removal test, a content types part whose
   `Default` elements carry a space before the slash, one `Override`
   removed, every `Default` back byte for byte" - carried,
   `crates/emend/src/package/tests.rs:2199-2246`. Both orders are run
   (`:2226`) and the fixture is asserted to carry both spellings
   (`:2220-2224`), which answers the obvious way for this test to rot.
   Nothing wrong found.
4. "`--drop-calc-chain` reading the content types part before deciding
   the edit is empty" - carried, `tools/emend-copy/src/main.rs:342-344`
   (`package.content_types()?` before the chain is looked for), with the
   doc text at `:23-28` and `:329-337`. The corresponding negative case
   is `negative.log:1791-1804` (D5 now a refusal, with the same message
   as `--survey` on the same fixture, and part 2 P3's opposite verdict
   read out of that committed log). Nothing wrong found.
5. "the two overlong comment lines in `scripts/gate-self-test.sh`
   rewrapped" - carried in `04113ac`, two whole comment blocks reflowed
   with no word changed (I diffed them: the only changes are line breaks).
6. "The run script's faulty fixture drives every counter by name" - the
   run script lives outside the repository, and `run.log:928-963` prints
   its three by-name control blocks (the entry classifier, the strings
   line reader, the independent counter), each comparing a full expected
   line as one string. Carried.
7. "the self-test invocation line names the launch form" - carried, in
   `001fe24` rather than the first code commit (above);
   `scripts/gate-self-test.sh:247` and `self-test.log:4`.
8. "The gate-file change runs the self-test and commits `self-test.log`" -
   carried; see question 2.

### The second code commit, clause by clause (`specs/0010:310-323`)

`cells::SharedStrings` with `len()` (u32), `get(u32)`,
`SharedStrings::read`, `Package::shared_strings`, `--shared-strings`, and
the module in `lib.rs`: all present
(`crates/emend/src/cells.rs:60-231`,
`crates/emend/src/package.rs:532-565`,
`tools/emend-copy/src/main.rs:421-448`,
`crates/emend/src/lib.rs:28,34`). `resolve`, `Cell`, `CellRef`, `Cells`,
`Formula`, `FormulaKind` and `Kind` are correctly absent: the spec puts
them in P2 (`specs/0010:325-350`).

The output line's shape matches `specs/0010:243-245` exactly
(`main.rs:447`: `strings\titems={items}\tbytes={bytes}`), `bytes` is the
sum of `str::len` (UTF-8 bytes) over the items, and both counters are
`u64` in the tool as `specs/0010:89-90` asks.

### The spec's twelve test cases

`specs/0010:312-322`. Every one is covered by a test in
`crates/emend/src/cells/tests.rs`:

| # | spec case | test | line |
| --- | --- | --- | --- |
| 1 | a plain item | `a_plain_item_is_its_text` | 148 |
| 2 | an item of runs whose `<t>` texts concatenate | `the_runs_of_an_item_concatenate` | 156 |
| 3 | an item with `<rPh>` skipped | `a_phonetic_subtree_is_skipped` (and `the_phonetic_skip_ends_at_its_own_end_tag`) | 168, 180 |
| 4 | a character reference and a numeric reference unescaped | `references_are_expanded` | 191 |
| 5 | leading and trailing spaces kept with and without `xml:space` | `spaces_are_kept_with_and_without_xml_space` | 201 |
| 6 | an empty `<t/>` and an empty `<si/>` both an empty string | `an_empty_text_and_an_empty_item_are_both_the_empty_string` | 211 |
| 7 | `count` and `uniqueCount` disagreeing and ignored | `count_and_unique_count_are_ignored` | 223 |
| 8 | no shared strings relationship gives an empty table | `a_workbook_with_no_shared_strings_gives_an_empty_table` | 274 |
| 9 | the root of the wrong name | `a_root_of_the_wrong_name_is_refused` | 299 |
| 10 | non-UTF-8 text | `text_that_is_not_utf8_is_an_xml_error` | 340 |
| 11 | an unknown entity refused | `an_unknown_entity_is_refused` | 330 |
| 12 | `get` at the length `None` | `get_past_the_end_is_none` | 266 |

Nothing missing. Twenty tests in all, confirmed by my own count and by
`test.log:1368-1393` ("cells module tests on the host: 20", same on i686).
Each goes through `Package::shared_strings` on a byte-built zip
(`tests.rs:95-104`), which is what the spec asks for, and the fixture
builder has its own positive control at `:353`.

**A small report inaccuracy.** The report's "The tests" section accounts
for the twenty as "The spec's twelve cases, plus five the writing raised"
plus the fixture control - that is eighteen. The two it does not name
there are `the_phonetic_skip_ends_at_its_own_end_tag` (tests.rs:180) and
`text_in_another_namespace_is_not_the_item_s_text` (tests.rs:236). Both
are discussed elsewhere in the report, so nothing is concealed; the
arithmetic in that one sentence does not add up.

### POSITION or MECHANISM rather than PROPERTY

This is where I found the real problems.

**FINDING 1 (the important one). The `<rPh>` depth is not tested, and the
counterfactual the test comment, the commit message and the report all
state is false.**

`crates/emend/src/cells/tests.rs:180-188`
(`the_phonetic_skip_ends_at_its_own_end_tag`) says in its comment: "The
skip is a depth and not a flag ... Without the depth this reads
'askippedb' or 'a'." Commit `8f2364d`'s message says the same
("reading 'ab' where a flag reads 'askippedb'"), and the draft report
repeats it in "The second code commit": "the test that drives it reads
'ab' where a flag reads 'askippedb' and an early exit reads 'a'".

I built the flag. In `<scratch>` I replaced the depth in
`crates/emend/src/cells.rs` with a flag in the most natural way: the
nested-start increment at `cells.rs:132-134` deleted, and the decrement at
`cells.rs:169-170` turned into a clear (`phonetic = 0`), which is exactly
the "a flag would end the skip at the first end tag inside it" the commit
message describes. Result:

- `the_phonetic_skip_ends_at_its_own_end_tag` PASSES under the flag. It
  reads "ab", not "askippedb".
- ALL TWENTY committed `cells::tests` pass under the flag (`cargo test -p
  emend --lib cells::tests::` in the scratch copy: 20 passed; the four
  failures in that run are my own probe tests, which panic by design).

So the depth is a mechanism nothing in the module tests distinguishes from
the flag, and the specific string "askippedb" does not arise from the flag
at all. The fixture is
`<si><t>a</t><rPh sb="0" eb="1"><r><t>skipped</t></r></rPh><t>b</t></si>`:
the nested `<t>` is the LAST child of `<rPh>`, so the flag's premature
clear happens after the only inner `<t>` has already been skipped. The
fixture pins a POSITION (where the nested element sits inside `<rPh>`)
where the property is that the whole subtree is skipped whatever follows
inside it.

The code is right; the test is what is wrong. A fixture that bites, which
I ran both ways:
`<si><t>a</t><rPh sb="0" eb="1"><r><t>x</t></r><t>y</t></rPh><t>b</t></si>`
gives `["ayb"]` under the flag and `["ab"]` under the committed code. One
extra `<t>` sibling inside `<rPh>`, after the nested element, turns the
comment's claim into a measurement.

This is the seven-times-recorded defect class, and it is in the one test
the report singles out as proving a mechanism.

**FINDING 2. CDATA inside `<t>` is dropped silently, which contradicts the
module's own stated principle, and no test or spec clause covers it.**

`crates/emend/src/cells.rs:181-198` handles `Event::Text` and
`Event::GeneralRef` and falls through `_ => {}` for everything else.
`quick-xml` 0.42 has a separate `Event::CData` variant
(`quick-xml-0.42.0/src/events/mod.rs:1791-1792`), so character data
written as CDATA never reaches the item. I ran it in `<scratch>`:

- `<si><t><![CDATA[hello]]></t></si>` gives `""`.
- `<si><t>a<![CDATA[b]]>c</t></si>` gives `"ac"`.

No error, no refusal: the reader returns a wrong value as a right one.
That is the opposite of the principle `cells.rs:20-21` states ("a value
with a hole in it is not the value", the stated reason for refusing an
unknown reference) and of the reasoning `specs/0010:103-105` gives for
refusing an unknown cell type ("a reader that guessed would report a
wrong value as a right one"). `specs/0010:145-146` says an `<si>` is "the
concatenation of every `<t>` text under it"; CDATA text is `<t>` text.
The spec's corpus facts (`specs/0010:66-69`) do not mention CDATA either
way, so there is no measurement saying the corpus is free of it and the
corpus run cannot have caught it: a dropped item text still counts as one
item, so `items` per workbook would still have agreed. The spec asked for
none of this, so it is a gap rather than a departure, but it is a real
one and planning should rule: refuse, read, or record as out of scope
with a test that pins the choice.

**FINDING 3. The `text` flag has exactly the weakness the `<rPh>` depth
was made a depth to avoid, in the mirror position.**

`crates/emend/src/cells.rs:112,146-151,171-172`: `text` is a bool set at
a main-namespace `<t>` start and cleared at any main-namespace `</t>`.
It is not cleared for a child element of `<t>`, so text belonging to a
child element is read as the item's. Probe in `<scratch>`:
`<si><t>a<foo>bar</foo>b</t></si>` gives `["abarb"]`. `<t>` is simple
content in the schema so no producer writes this, and the impact is far
smaller than finding 2; I raise it only because the module argues in its
own comment (`cells.rs:105-107`) that a flag is the wrong shape for
nesting, and then uses one here.

**Checks that are sound, stated so explicitly.** I looked for the same
defect in each of the other tests and did not find it:
`text_in_another_namespace_is_not_the_item_s_text` (tests.rs:236) asserts
a property, and I verified the report's claim about it by dropping the
`ours &&` guard at `cells.rs:146` in `<scratch>`: the test fails with
`left: ["keptnot text"] right: ["kept"]`, exactly the string the report
quotes. `an_empty_text_and_an_empty_item_are_both_the_empty_string`
(tests.rs:211) puts a third item after the empty ones so an index shift
cannot hide. `count_and_unique_count_are_ignored` (tests.rs:223) asserts
the length as well as the items, so an attribute becoming the length
fails. `get_past_the_end_is_none` (tests.rs:266) reads `get(table.len())`
rather than a literal index. `the_table_is_read_afresh_on_every_call`
(tests.rs:362) compares two owned tables. The refusal tests are all
paired with the fixture-builder control at `:353`, which is the right
shape.

### In the diff and not asked for

- `SharedStrings::is_empty` (`cells.rs:77-79`). Not in
  `specs/0010:140-142`. Clippy's `len_without_is_empty` forces it once
  `len()` exists; I confirmed clippy passes on the committed tree. Fine,
  but it is API the spec did not name.
- `Clone` in the derive at `cells.rs:60` is used nowhere in the phase.
  `PartialEq`/`Eq` are used by `tests.rs:371`.
- The `Error::Report` display text changed from "cannot write the survey
  line" to "cannot write the report line"
  (`tools/emend-copy/src/main.rs:122`). Not asked for. I grepped the
  whole repository: the old string is asserted in no test and no log, so
  no primary is falsified by the change.
- A behavioural decision the spec did not make: a `sharedStrings`
  relationship whose target names no entry answers with an empty table
  (`package.rs:558-560`, pinned by `tests.rs:287-296`).
  `specs/0010:148-150` rules only on "no shared strings relationship". The report
  mentions this in passing, in the list of extra tests, rather than
  putting it in "Questions for planning" - and it is a question: part 2's
  `--survey` counts unresolved targets as a defect of the package, while
  this layer reads such a workbook as if it had no table at all.

Apart from those, I found nothing in the diff the spec did not ask for.
`git diff --stat 7702f6e..HEAD` touches 13 files and every one is
accounted for by the spec's P1 text or by the doc commit.

### Build state, checked myself

In the scratch copy of the committed tree: `cargo test --workspace`
totals 280 passed, 0 failed (the sum of the `test result` lines), which is
the figure `test.log` and the report give; `cargo fmt --all -- --check`
exits 0; `cargo clippy --workspace --all-targets -- -D warnings` exits 0.

---

## 2. Do the logs contain every line the spec requires?

`specs/0010:279-289` and `specs/0001-bootstrap.md:95-215`.

**First lines of every log.** All six open with `command: git rev-parse
HEAD` and then an `about:` line naming the commit the log is about and the
HEAD at run time: `gate.log:1-2`, `test.log:1-2`, `negative.log:1-2`,
`run.log:1-2`, `self-test.log:1-2`, `scan.log:1-2`.

**FINDING 4. `scan.log`'s header names the wrong phase and the wrong
spec.** `scan.log:2` reads "about: M1 part 2 P3 phase diff
7702f6e..001fe24" and `scan.log:6` reads "phase: Milestone 1 part 2 P3
(specs/0009-package-layer.md) scan.log". This is part 3 P1 and its spec is
`specs/0010-reading-cells.md`. The commit range and the HEAD on those
lines are correct and are resolved from git (`scan.log:3-5`, by the part 3
doc-commit subject), so only the labels are wrong - but the first-line
rule exists so that a log says what it is about, and this one says it is
about the previous phase. The draft report lists `scan.log` as PENDING and
so does not repeat the error; the final pass must fix it, and since a
committed log is never edited (`specs/0001:187-189`) that repair has to be
a fresh run, which is due anyway (below).

**The script printed into the log.** All five of this phase's own logs
print their source with `sed 's/^/    /' "$0"`: `gate.log:94`,
`test.log:81`, `negative.log:166`, `run.log:360`, `scan.log:244`. The
self-test log prints its cases rather than a script, as in every phase.

**FINDING 5. `run.log`'s printed runner command does not match the command
that ran, in the one argument the phase's third rerun was about.**
`run.log:521` echoes `--out <corpus>/runs/$DATE-$SHORT`, while
`run.log:523` runs `--out "$OUT"` with
`OUT=$CORPUS/runs/$DATE-$SHORT${RUN_SUFFIX:+-$RUN_SUFFIX}`
(`run.log:148`). At `run.log:980-981` the log prints the real directory,
with the suffix; at `run.log:984` and `:1078` it prints the command
without it. A reader who reproduced the printed command would run into the
directory of an earlier attempt - which is precisely the collision that
made attempt (2) unsound (`run.log:4`). The printed SCRIPT is faithful
(the defect is inside the script's own echo, not in the printing), so this
is a log-accuracy defect rather than an unsound measurement, and the
report does not mention it.

**Every compound check in a named function.** `specs/0001:126-135`. I ran
the two forbidden-shape greps myself, from `/usr/bin/grep` under `LC_ALL=C`
with the patterns assembled the way the scan assembles them, over all six
logs: zero hits for the chained form and zero for the leading-bang form in
every one, including `scan.log`, which no scan reads. That independently
confirms `scan.log:1100-1115`. The two compound checks I read in full are
both named functions: `fresh_dir` at `run.log:507-509` used as
`is fresh_dir`, and `status_accounted` at `gate.log:539-543` used as
`is status_accounted`. The `$(yn [ ... ])` single-test form used
throughout is the permitted one.

**FINDING 6. The two forbidden-shape greps do not cover the scan script's
own source, though the log claims otherwise.** `scan.log:493-495` says
"Every printed script goes through both greps, which for this phase means
FIVE logs", and `scan.log:1114` reports "files run through the two shape
greps: 5". The scan script is itself a printed script (`scan.log:244`) and
is not one of the five. The file is available at run time - the self-scan
block already greps `"$0"` (`scan.log:228,236`) - so extending the two
greps to it was one line. The scope is a LIST OF FILE POSITIONS where the
label states a property ("every printed script"). It hides nothing this
time: my own run of both greps over `scan.log` returns 0 and 0. It is the
same shape of gap as the part 2 P3 finding that produced the self-scan
rule, one pattern family later.

**Every scan form has a positive control in the same command form.**
`scan.log:747-785` lists 39 controls, all as-expected (`:786`), including both directions for
`f_home`, `f_p6` versus `f_pre` on the bare forms, and clean-file
negatives for every form. `scan.log:1103-1108` gives the two shape greps
their own positive, negative and cross controls. `gate.log:865-901` and
`negative.log:1311-1337` carry their own. Nothing missing found.

**The scan log written LAST and hashing every committed sibling.**
`scan.log:989-1069` hashes the five sibling logs and the two record files
and prints nine form counts for each, all zero, and asserts the sibling
count (5) and the record count (2). The hashes match the files on disk
exactly (I ran `sha256sum` over all of them). But `scan.log:1048` reads
"check answers scanned: 0", and the log directory holds no
`independent-check-N.md`. So this scan.log is NOT the final pass;
`specs/0001:33-42` requires the committed scan log to come after the check
answers are saved and to list each with its sha256, and
`specs/0001:38-41` requires a new final pass whenever a sibling is rerun
after a scan pass. The report marks `scan.log` PENDING, which is
consistent; I record it so planning knows what the final pass still owes:
the check answers scanned and named, and the header of finding 4 fixed.

**FINDING 7. `scan.log` carries no `rerun:` line although the report
describes a superseded attempt of it.** The report's "A bare shape in a
scan-script comment" says the scan script "reported count=1 on the first
run of this phase's scan script", which was then changed. That is a failed
structural check followed by a fix and a rerun, and
`specs/0001:83-87` says the rerun's header names the log it replaces by
its sha256 when it was not committed. `scan.log` has no `rerun:` line
(the hook exists at `scan.log:215`, unset), and the attempt's sha256
appears nowhere. The session's `<scratch>` `superseded/` directory, which
holds this phase's other kept attempts, contains no scan attempt for this
phase either (its m1p3-p1 files are the two negative attempts and the four
run attempts), so as things stand the only record of that attempt is the
report's prose, and `specs/0001:180-186`'s fallback - "an attempt that was
not kept is named as having no hash, and why" - is not satisfied anywhere.
The final pass can carry it.

**FINDING 8. No log prints a command producing the sha256 of any
superseded attempt, which is the rule this phase's own doc commit added.**
`specs/0001-bootstrap.md:180-186`, added in `7175092`, says "Every sha256
a log cites, a superseded attempt's included, is printed by a command run
over a file that exists at run time, never typed from memory." The four
attempt hashes this phase cites appear exactly once each, as prose inside
a `RERUN_NOTE` environment variable printed by `negative.log:4` and
`run.log:4` (`grep -c` for each hash in its log: 1). No log runs
`sha256sum` over the kept attempt files. Every other sha256 in these logs
IS printed by a command over a file (`gate.log:773-786`, `run.log:966-970`
and `:1178-1183`, `negative.log:189` and `:944`, `scan.log:997` onward),
so this is the one class of hash that still cannot be checked from a
primary - and it is the class the rule was written for, after a fabricated
hash in part 2's close-out.

I went looking rather than leaving it at that. The session's `<scratch>`
`superseded/` directory holds the kept attempts, and I hashed them myself:
`m1p3-p1-negative-attempt1.log` is 115909 bytes and
`90379bfa659eacd5f8b1980c63803963035b8e0322c3b14376dd21e95f7d7017`;
`m1p3-p1-run-attempt1.log` 61198 and `09b1cdca77b3...`;
`m1p3-p1-run-attempt2.log` 61267 and `76ef6f5647c4...`;
`m1p3-p1-run-attempt3.log` 65205 and `db315fdad753...`. All four values and
all four byte counts are exactly what the two rerun lines cite, so NO hash
in this phase is invented and the substance of the reruns is sound. What is
missing is only that a reader of the committed logs cannot establish that:
I had to reach into an uncommitted scratch directory, which is not a
primary. Planning may want the rule spelled as "the log prints the hashing
command", since a note the session composes satisfies the sentence's letter
only if one assumes the session ran the command.

**The gate-file change requires a committed `self-test.log`.** Satisfied,
and checked well. `gate.log:788-807` names the one gate file in the diff
(`scripts/gate-self-test.sh`) out of ten checked, hashes `self-test.log`
at its committed bytes, and echoes its first four lines. Nothing missing.

**Does the self-test run cover the phase?** Yes, on all three conditions,
each asserted in the log rather than left to a reader:
`gate.log:797-800` - "the commit that self-test was run at:
001fe24...", "the self-test ran at a commit inside this phase's range:
as-expected"; `gate.log:801-802` - "RESULT: PASS lines in the self-test
log: 1", "the self-test log records a pass: as-expected";
`gate.log:803-804` - "gate-file commits after the self-test's commit: 0",
"no gate file changed after the commit the self-test ran at: as-expected".
I checked the timing independently: `001fe24` was committed at
2026-09-18T16:05:43Z (`git log --format='%cI'`) and the self-test's header
records its start at 16:05:48Z and its end at 16:24:20Z
(`self-test.log:3` and the tail), five seconds after the commit and
before `gate.log` started at 16:24:57Z. The last condition passes partly
because the self-test's commit IS the phase tip, so nothing could have
come after it; the property still holds, and the check is the right one.

**Observation, not a defect: the launch form is a declaration, not a
measurement.** `self-test.log:4` reads "launch form: detached background
task of the calling session; no build and no other run of the session
during it". That string comes from `LAUNCH_NOTE`, an input
(`scripts/gate-self-test.sh:247` (the comment at `:43-51`)), and `001fe24`'s own message concedes
"a script cannot see whether the session that started it went on to
build". Nothing in the log measures the property the line asserts. I note
it only so planning is not misled about what the line is worth;
`specs/0009-package-layer.md:1217-1219` (ruling 7) asked for the launch
form to be STATED in the header, so the implementation complies with the
ruling as written. The same applies to `run.log:13-17` ("no cargo runs
during it"), except that there the four binary hashes at `run.log:966-970`
match `gate.log:1059-1062` exactly, which is real evidence no rebuild
happened.

**FIVE logs and no `size.log`.** Consistent with the spec, and the report's
citation is slightly off. `specs/0010:283-285` ("## Phases") is the
governing sentence: "`gate.log`, `test.log` on both targets,
`negative.log`, `run.log`, `scan.log`, and `size.log` in P3". The report
cites `specs/0010` "Size" instead; that section
(`specs/0010:207-220`) does put the probe and the record in P3, so the
report's conclusion is right, but "Phases" is where the log list lives.
`scan.log:990-996` and `gate.log:530-533` both state the absence and give
the reason, so no check reports a term that can only be zero.

**Other per-phase rules I checked and found satisfied.** Colour off and
`RUSTUP_AUTO_INSTALL=0` declared (`test.log:5`); zero control bytes and
zero non-ASCII in every sibling and both records (`scan.log:997-1070`);
every commit id resolved from git at run time (`gate.log:739-747`,
`scan.log:3-5`, `run.log:323-325`); negative tests in a scratch clone with
the gate's binaries copied in, never the main tree
(`negative.log:1338-1348` and `:1959-1963`); no absolute machine path, the
repository written `<repo>`, scratch `<scratch>`, the corpus `<corpus>`;
the range scan written `<base>..HEAD`.

---

## 3. Does the report's every number and quoted string appear in a
primary?

Verified and correct:

- The four commit hashes and subjects: `git log --reverse 7702f6e..HEAD`,
  also printed at `gate.log:752-755` and `scan.log:789-793`.
- Log bytes and sha256s, all five rows of the report's table: I ran
  `sha256sum` and `wc -c` over the files. `self-test.log` 15781 /
  `01162f86...`; `gate.log` 62739 / `b281900a...`; `test.log` 91949 /
  `ecd8a908...`; `negative.log` 116638 / `a516159d...`; `run.log` 66202 /
  `c3205791...`. All match the report and `scan.log:997`, `:1007`,
  `:1017`, `:1027`, `:1037` and `gate.log:792`.
- Record pair: 1175 bytes / `adaef6ef...` and 1839 bytes /
  `9396eb5e...`, matching the report, `run.log:1179-1183` and
  `scan.log:1049,1059`.
- "PASS, 50 cases, 0 unexpected, 0 errors": `self-test.log` tail
  ("cases performed: 50; cases selected: 50", "self-test: 50 cases, 0
  unexpected, 0 errors", "RESULT: PASS").
- "Nine PASS lines, the `doc` step asserted by name ... the `licences`
  step passing with `quick-xml` in the core's tree, and all four binaries
  left executable on both targets": `gate.log:1033-1046` and `:1058-1063`.
  The licence listing at `gate.log:843-861` shows 12 packages, all
  permissive, `quick-xml 0.42.0: MIT` among them.
- "Three terms in the working-tree accounting": the three summands of
  `status_accounted` at `gate.log:539-543`, with the reason at
  `gate.log:530-533`. The printed sentence has four numbers (the total
  plus the three terms), which is what confused me at first; the report's
  word "terms" is the script's own and is right.
- "280 tests passed on the host and 280 on `i686-unknown-linux-musl`, 0
  failed, 2 ignored, the same test NAMES on both": `test.log:895`,
  `:1221`, `:1451-1453`, `:1419`. I reproduced 280 myself.
- "The previous phase's total, 257, is read from
  `records/logs/m1p2-p3/test.log` rather than typed": `test.log:1252-1253`.
- "A new named-test block prints the cells module's 20 tests case by
  case": `test.log:1368-1392`.
- "Eight families" in the negative log: N, S, D, T, C, X, R, G
  (`negative.log` section headings at 1418, 1647, 1761, 1805, 1839, 1881,
  1897, 1919).
- "T1 is the positive control: a two-item fixture whose printed line is
  asserted in full and whose output is the input's bytes":
  `negative.log:1805-1812`, the line being `strings items=2 bytes=2`.
- "T4 asserts that a workbook with no shared strings relationship reports
  `items=0 bytes=0` and exits 0": `negative.log:1830-1837`.
- "D5 inverted ... with the SAME message ... and it reads part 2 P3's
  opposite verdict out of that committed log": `negative.log:1791-1804`.
- "The catalogue coverage reads three source modules and two test files":
  `negative.log:1389` (three sources named in one command) and
  `:1840-1843` (two test files, each hashed).
- Negative rerun: sha256 `90379bfa...`, 115909 bytes, never committed:
  `negative.log:4`.
- Run reruns: `09b1cdca...` 61198, `76ef6f56...` 61267, `db315fda...`
  65205, with the three reasons: `run.log:4`. The report's account of
  attempt (1) matches `run.log:97-103` and the fix at `run.log:704-724`;
  of attempt (2), `run.log:105-112`; the `RUN_SUFFIX` and the
  before-either-pass guard, `run.log:142-148` and `:501-515`.
- "870 entry rows per target, all `identical`", and every zero the report
  lists: `run.log:1026` and `:1120`.
- "`files_identical` yes, `order_preserved` yes, `lo` and `lo_in` ok on
  all 27": `run.log:1015-1024` and `:1109-1118`.
- The totals table (27 / 110582 / 1497107 / 54472 on both targets):
  `run.log:1066`, `:1160`, `:1174-1175`, and the record pair.
- "Every workbook's `items` equalled the independent count, on both
  targets": `run.log:1065` and the per-workbook rows above it, and again
  `:1159`. The `.txt` record's 27 rows per target agree column for
  column, and their `items` column sums to 110582, which I added up.
- "The total is 110582, exactly the `<si>` figure `specs/0010`, 'The
  corpus', gives": `specs/0010:66-67`; asserted by name at
  `run.log:1070`.
- "No workbook reported an empty table, and the largest table is smaller
  than the total": `run.log:1074-1075`.
- "2748976 ... the largest shared strings PART's byte size and 110582 as
  the `<si>` count across all 27 parts; the largest single table holds
  54472 items": `specs/0010:40-41`, `:66-67`, and `run.log:1066` and `:1073`. The
  corrected header is at `crates/emend/src/cells.rs:34-38` and does name
  the two measurements separately.
- "110010 of the corpus's 110666 `<t>` elements" (in the module header,
  which the report quotes indirectly): `specs/0010:67-69`.
- "the self-scan reports 0, and its control reports 1":
  `scan.log:19-22`.
- "twenty in `crates/emend/src/cells/tests.rs`": counted, and
  `test.log:1389`.
- "wall time: 300s" and "RESULT: PASS": `run.log:1189-1191`.
- "the same defect class that was committed in part 2 P3's `scan.log`":
  `specs/0009-package-layer.md:1319-1350` and the ROADMAP Close row.
- The rulings the report cites by number: ruling 7 at
  `specs/0009:1217-1226`, ruling 6 at `specs/0009:1210-1216`. The
  report's quoted fragment "from part 3" is on `specs/0009:1219`.
- "`\"a cell holds a reference this layer cannot expand\"` is the message
  `specs/0010` gives": `specs/0010:167-170`, and the spec does attribute
  it to text inside `<v>`, `<f>` and `<t>` in the cell reader, so question
  1 of the report is correctly grounded.

Numbers and quoted strings in the report that do NOT appear in a primary,
or that a primary contradicts:

- **"The independent counter has five controls".** `run.log:949-963`
  prints FOUR control checks over three built fixtures and one non-zip
  file: the three-plain-items count, the "none" answer where no
  shared strings part is declared, the faulty fixture (the element whose
  name begins with `si`, plus a self-closing and an attributed item), and
  the error on a file that is not a zip. I counted the `as-expected`
  lines in that block: 4. The figure five appears nowhere.
- **"a regular expression finds the `Override` ... and that part's `<si>`
  start tags are counted with `grep -o`".** The counter uses no grep at
  all: `run.log:298-320` shows `si_count` as a single `python3` heredoc
  using `zipfile` and `re.findall(rb"<si[ />]", body)`. The report
  misdescribes its own mechanism. The substance of the claim - a route
  sharing no code with the library, with a delimiter so that an element
  whose name merely begins with `si` is not counted - is correct.
- **"a flag reads 'askippedb' and an early exit reads 'a'".** Disproven;
  see finding 1. The natural flag reads "ab" and passes the test.
- **"P2 added two for `--survey` and P3 three for `--drop-calc-chain`".**
  The "two" is right; the "three" is misattributed. At `eacffa2` (part 2
  P2's record commit) `tools/emend-copy/tests/null_rewrite.rs` holds 9
  `#[test]`s, two of them naming `--survey`. At `711fc8b` (part 2 P3's) it
  holds 12. The three P3 added are
  `the_survey_counts_an_override_that_names_no_entry`,
  `the_recipe_drops_the_chain_and_leaves_every_other_entry_alone` and
  `the_recipe_leaves_a_package_with_no_chain_byte_for_byte`: THREE tests,
  of which TWO are `--drop-calc-chain` and one is another `--survey`
  test. The same wording is in `test.log:30-31`, which is a script
  comment, and `specs/0001:190-192` says a figure in a script's prose is
  not a measurement - so this number has no primary behind it either way.
  The threshold itself, twelve, is measured: `test.log:1231` ("integration
  null_rewrite 12").
- **"N23 to N28".** The six case lines are there
  (`negative.log:1601-1636`) and the report's range matches them, but the
  log's own section heading at `negative.log:1600` reads "N23 to N27:
  --shared-strings among the operations, refused in pairs", which
  understates the range by one and describes only the first four cases
  (N27 is the misplaced flag and N28 the flag with an argument, neither
  "in pairs"). The report is right and the log's heading is wrong.

Two report claims that are true but rest on no primary, which planning
should know:

- "This was probed against the crate before the reader was written, not
  assumed from the documentation" (the `GeneralRef` splitting). The
  BEHAVIOUR is real - `references_are_expanded` passes and
  `quick-xml-0.42.0/src/events/mod.rs:1801-1803` documents the variant -
  but that a probe was run first is recorded only in the report and in
  `8f2364d`'s message.
- "The widened test was shown to bite in both directions on the real
  files before the commit". Recorded only in `04113ac`'s message. I
  reproduced both directions myself (question 1, clause 2), so the claim
  is true; no log carries it.

---

## Summary of findings, in the order I would fix them

1. `the_phonetic_skip_ends_at_its_own_end_tag` does not distinguish the
   depth from a flag: all twenty cells tests pass with the depth replaced
   by a flag, and the "askippedb" counterfactual in the test comment, in
   `8f2364d`'s message and in the report is false. One extra `<t>`
   sibling inside `<rPh>`, after the nested element, fixes it. The code is
   correct; the test and three prose claims are not.
2. CDATA inside `<t>` is dropped silently (`""` for
   `<si><t><![CDATA[hello]]></t></si>`), contradicting the module's own
   stated reason for refusing an unknown reference. Untested, unruled,
   and invisible to the corpus run.
3. `scan.log`'s `about:` and `phase:` lines name M1 part 2 P3 and
   `specs/0009` instead of part 3 P1 and `specs/0010`.
4. `run.log` prints a runner command whose `--out` lacks the
   `RUN_SUFFIX` the run actually used - the one argument the third rerun
   existed for.
5. The two forbidden-shape greps skip the scan script's own source while
   the log says "every printed script". Nothing hidden this time (I ran
   both greps over `scan.log`: 0 and 0).
6. No log prints a command producing any superseded attempt's sha256, the
   rule `7175092` itself added; and `scan.log` carries no `rerun:` line
   for the attempt the report describes, which is also kept nowhere. The
   four hashes the logs do cite are CORRECT - I hashed the kept attempt
   files in the session's `<scratch>` `superseded/` directory and all four
   values and byte counts match - but only a reader with access to that
   uncommitted directory can establish it.
7. `run.log`'s record-privacy check runs over the `.md` only
   (`run.log:715`, `"$REC"`); the `.txt`, which carries the per-workbook
   rows, is never passed to it, though the check is labelled "the record
   names no corpus file name". `scan.log:1049-1069` does scan both files
   with `f_deny` and the rest, so the property holds; the check as
   labelled is narrower than its label. Its pattern is also
   `[.](xlsx|xltx|xlsm)`, which covers this corpus exactly but is
   narrower than "a corpus file name".
8. Report numbers: "five controls" (four), "grep -o" (python `re`),
   "P3 three for --drop-calc-chain" (three tests, two of them for that
   flag), and the twelve-plus-five arithmetic that leaves two of the
   twenty tests unnamed.
9. The third code commit's stated reason ("that commit was already made")
   is contradicted by `specs/0001:76-80`, which lets an unpushed,
   unverified commit be amended.
10. A behavioural decision the spec did not make - a `sharedStrings`
    relationship naming no entry reads as an empty table - is mentioned in
    the report's test list rather than put to planning as a question.

The phase's substance holds up: the feature matches the spec, the twelve
cases are all covered, the corpus run's per-workbook agreement against a
genuinely independent counter is the right acceptance and it passed on
both targets, and the logs are unusually careful about controls. Findings
1 and 2 are the two I would not let through: the first because a
mechanism the report puts forward as proven is not tested and the quoted
counterfactual is wrong, the second because the reader returns a wrong
value silently in a case nothing in the repository covers.
