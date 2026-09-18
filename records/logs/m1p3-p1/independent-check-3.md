# Independent check 3: emend M1 part 3 P1, the final tree

I have not seen the conversation that produced this work. Everything below comes
from the repository, from git, and from runs I made myself in a detached
`git worktree` under `<scratch>`, with a separate `CARGO_TARGET_DIR`. I read
nothing under the corpus directory and did not run `scripts/passthrough-run.sh`.
The worktree was removed at the end; `git -C <repo> status --porcelain` is empty
and `HEAD` is still `bc5aba5`, so the real repository is unchanged (stated again
at the end of this answer).

Commands I ran: `git log/show/diff/status/worktree/for-each-ref/rev-parse`,
`sha256sum`, `wc -c`, `/usr/bin/grep`, `awk`, `sed`, `bc`, `python3`,
`cargo test`, `cargo fmt --all --check`, `cargo clippy --workspace
--all-targets -- -D warnings`.

---

## 1. Does `5233bf4` do what its message says, and nothing else?

**Yes, every clause, and I reproduced both directions myself.**

### The flag, built as the prompt specifies

In the scratch worktree at `5233bf4` I deleted the nested-start increment
(`crates/emend/src/cells.rs:133`, `phonetic += 1;`) and changed the end-tag
decrement (`crates/emend/src/cells.rs:170`) from `phonetic -= 1;` to
`phonetic = 0;`. That is the flag.

**New fixture under the flag: FAILS.** `cargo test -p emend --lib
cells::tests::` gave 19 passed, 1 failed:

    ---- cells::tests::the_phonetic_skip_ends_at_its_own_end_tag stdout ----

    thread 'cells::tests::the_phonetic_skip_ends_at_its_own_end_tag' panicked at
    crates/emend/src/cells/tests.rs:196:5:
    assertion `left == right` failed
      left: ["ayb"]
     right: ["ab"]

    test result: FAILED. 19 passed; 1 failed; 0 ignored; 0 measured; 179 filtered out

That is `left ["ayb"] right ["ab"]`, exactly as the commit message and the
report (draft line 58) state.

**Old fixture under the same flag: PASSES.** I then replaced only
`crates/emend/src/cells/tests.rs` with `git show
8f2364d:crates/emend/src/cells/tests.rs`, keeping the flagged reader:

    test cells::tests::the_phonetic_skip_ends_at_its_own_end_tag ... ok
    test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 179 filtered out

So the old fixture really did pass under a flag - all twenty - and the new one
really does fail. Check 2's finding is confirmed and so is the fix.

With the reader restored, the committed tree passes 20 of 20.

### `cells.rs` untouched

    git show 8f2364d:crates/emend/src/cells.rs | sha256sum
    git show 5233bf4:crates/emend/src/cells.rs | sha256sum
    git show bc5aba5:crates/emend/src/cells.rs | sha256sum

all three give `741c8a7e2cf341bbb0b3c5c602a2e5d1186456d6350cfe22cb13a925de043f2b`.
The same value is printed at `records/logs/m1p3-p1/gate.log:800`. Byte-identical,
as the message says.

### Nothing else changed

`git diff --stat 001fe24 5233bf4` is one file, `crates/emend/src/cells/tests.rs`,
11 insertions and 2 deletions. Nine of the eleven added lines are comment and two
are fixture, which is exactly what the report says at draft line 212.
`git diff --stat 8f2364d 5233bf4` additionally shows `scripts/gate-self-test.sh`,
which is `001fe24`'s change and not this commit's. The commit message is ASCII
and ends with the Co-Authored-By line only, with no session link
(`CLAUDE.md`, `specs/0001-bootstrap.md:96-97`). `cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings` are both clean at
`5233bf4`.

### The retraction

`git log --all -S"askippedb"` touches `8f2364d`, `5233bf4` and `bc5aba5` only.
`8f2364d`'s message still carries the false sentence; `5233bf4`'s message names
and retracts it. No amend or rewrite happened: `git log --oneline 7702f6e..HEAD`
gives the same six commits the report's table lists, in that order.

### One false claim NO LONGER in the repository but STILL IN THE REPORT

The prompt asks whether any other file still states the false version. In the
repository, no: the only occurrences are `8f2364d`'s message (retracted),
`5233bf4`'s message (the retraction), and the two committed check answers, which
quote it in order to disprove it
(`records/logs/m1p3-p1/independent-check-1.md:496`,
`independent-check-2.md:177-180,190,196,606,650`).

**But the draft report itself repeats the retracted counterfactual as fact.**
Draft report lines 85-86 read:

    1. `the_phonetic_skip_ends_at_its_own_end_tag` - the skip is a depth, so the
       test reads "ab" where a flag reads "askippedb" and an early exit reads "a".

That is the same sentence the report retracts twenty-five lines earlier (draft
lines 50-53: "That was FALSE") and that `5233bf4`'s message retracts. It is now
false twice over: the string `skipped` does not occur anywhere in the new
fixture (`crates/emend/src/cells/tests.rs:193-194`), so no implementation can
produce `askippedb` from it, and I measured what the flag actually reads from the
new fixture: `["ayb"]`. A report that confesses this defect in section "The
second code commit" and then commits it again in section "The tests" has not
finished retracting it. This is the single thing I would fix before the report
goes to planning.

---

## 2. Are the six committed logs sound and consistent with each other?

### `about:` lines

Verified by reading line 2 of each log. Five name
`5233bf49066001d38402dee0c294b6ff229c3b76` as the phase commit and as HEAD at
run (`gate.log:2`, `test.log:2`, `negative.log:2`, `run.log:2`, `scan.log:2`;
`gate.log` spells it "phase commits, last is" and `scan.log` spells it as a
diff range `7702f6e..5233bf4`). `self-test.log:2` names
`001fe24c56a4eef1926e5d80cc82a6510db77e16`. That matches the report.

**The self-test exception: the conclusion holds, the justification overstates
the spec.** `specs/0001-bootstrap.md:113-122` is the whole of the rule, and it
says only that a phase changing a gate file "runs `scripts/gate-self-test.sh`
(the default run, detached) and commits its output as `self-test.log` with the
other logs". `grep -n "self-test" specs/0001-bootstrap.md` shows no other line
in "Rules for every phase" touching this. So:

- The report's negative half is right: `specs/0001` does NOT ask the self-test to
  run at the phase's final commit.
- The report's positive half is not in the spec. Draft report lines 279-282 say
  "what `specs/0001` asks is not that the self-test run at the phase's final
  commit but that it run at a commit inside the phase's range with no gate file
  changed afterwards, and `gate.log` asserts exactly that". `specs/0001` asks
  neither of those things; they are `gate.log`'s own conditions, described in
  its script comment at `gate.log:302-303` as "The property that makes ONE
  self-test run cover the phase". The same overstatement is in the record commit
  message (`bc5aba5`: "specs/0001 asks that it run at a commit inside the
  phase's range with no gate file changed after it"). A rule attributed to a spec
  should be quotable from it.

`gate.log` does assert all four things the report names, at exactly the lines it
cites:

    823:  the commit that self-test was run at: 001fe24c56a4eef1926e5d80cc82a6510db77e16
    824:  the self-test log names the commit it ran at: as-expected
    825/826: that commit appears in this phase's range: 1 time(s) / as-expected
    827/828: RESULT: PASS lines in the self-test log: 1 / as-expected
    829/830: gate-file commits after the self-test's commit: 0 / as-expected

`gate.log:818` prints `self-test.log`'s sha256 and byte count, as the report
says at draft line 259. `5233bf4` touches one test file and no gate file
(question 1 above), so the property gate.log checks does still hold. The
substantive conclusion is sound; only its attribution to `specs/0001` is wrong.

### `rerun:` lines and the superseded hashes

Every rerun line is followed by a `command:` line and by hash rows printed by a
loop that runs `sha256sum` and `wc -c` over each file at run time. I read the
loop in each printed script: `gate.log:99-111`, `test.log:85-97`,
`negative.log:172-184`, `run.log:339-351`, `scan.log:226-238`. The rule cited is
`specs/0001-bootstrap.md:180-186`, which I read and which is quoted accurately
by all five rerun notes. No log cites a sha256 with no command producing it, as
far as I can establish - see the next paragraph for what "establish" can mean
here.

**What I can and cannot re-derive.** The superseded attempt files are not in the
repository, so the eleven attempt hashes at `gate.log:6-7`, `test.log:6`,
`negative.log:6-8`, `run.log:6-10` and `scan.log:10` are unverifiable by me: the
command that produced them is printed, the file it ran over is gone. Check 2
reports having hashed the kept files in its own scratch and found them to match
(`independent-check-2.md:669-672`), which is the only corroboration that exists
and which no reader without that directory can repeat. What I CAN and did
re-derive, all matching to the byte:

- All eight committed log and check-answer sha256 and byte counts
  (`scan.log:1048,1058,1068,1078,1088,1099,1109` plus `scan.log`'s own row from
  my `sha256sum`/`wc -c`).
- The record pair: `96d1991c...` / 1175 and `7892e74d...` / 1839
  (`scan.log:1120,1130`; `run.log:1205-1208`).
- All thirteen diff-file sha256, byte counts and modes at `gate.log:799-811`,
  recomputed with `git show 5233bf4:<path> | sha256sum` and `| wc -c`. Thirteen
  for thirteen.
- The two test-file hashes at `negative.log:1865,1867` agree with
  `gate.log:801,804`.
- The four binary hashes agree across three logs (`gate.log:1085-1088`,
  `negative.log:1366-1369`, `run.log:993-996`).

**The unkept attempt is handled as `specs/0001` requires.** `scan.log:9` prints

    NO HASH EXISTS for m1p3-p1-scan-attempt0-self-scan-hit-NOT-KEPT.log: the file is not there at run time

and the rerun note at `scan.log:7` gives the why ("it was never written to a
file, so it was NOT KEPT and has no hash, and the command below says so rather
than inventing one"). `specs/0001-bootstrap.md:182-184` asks that "an attempt
that was not kept is named as having no hash, and why". Met. A pedantic note:
the printed line's own reason ("the file is not there at run time") is the
`else` branch's generic text and would print for any missing file; the actual
reason lives in the prose above it. Adequate, but the two are in different
places.

### `scan.log` and the two forbidden-shape greps

`scan.log` is written last (`scan.log:6`, run at 17:35:55Z, after `run.log`'s
17:29:36Z at `run.log:3`), hashes every committed sibling with a byte count
(five rows, `scan.log:1098` asserts "sibling logs scanned: 5 (expected 5)"),
scans the two check answers (`scan.log:1099,1109`, "check answers scanned: 2" at
1119), the record pair (`scan.log:1120,1130`, "records scanned: 2" at 1140), and
puts its own SOURCE through the two shape greps (`scan.log:540-541` in the
script, result at `scan.log:1187`). The patterns are assembled at run time and
printed at `scan.log:1174-1175`.

**I ran both greps myself** over all six committed logs and both check answers,
with `/usr/bin/grep -c -E` under `LC_ALL=C`:

    pattern [$][(](yn|is|isnot) .*(&&|[|][|])
      gate.log 0  negative.log 0  run.log 0  scan.log 0  self-test.log 0
      test.log 0  independent-check-1.md 0  independent-check-2.md 0

    pattern [$][(](yn|is|isnot) [!]
      gate.log 0  negative.log 0  run.log 0  scan.log 0  self-test.log 0
      test.log 0  independent-check-1.md 0  independent-check-2.md 0

My own positive controls, written as separate fixtures, scored 1 on each
pattern. So the scan's zeros are right, and the one file the scan cannot cover -
`scan.log` the log, as opposed to its source - is also 0. Note precisely what
`scan.log:1187` covers: "this script's own source", eight files in all
(`scan.log:1190`), not the log text. That is the most a scan can do and the
report states it correctly at draft lines 249-252.

The self-scan block is sound: `scan.log:23` prints `count=0` over the script's
own source and `scan.log:25` prints `count=1` for a copy with one concatenated
shape appended, so the zero means the form looked. The report's account at draft
lines 139-143 is accurate as far as it goes. **But see the finding below about
what else is in that same comment.**

### `run.log`: the printed command and the run directory

**The `--out` defect is fixed.** `run.log:1006` prints `run directory:
<corpus>/runs/2026-09-18-5233bf4-r6`, and the two runner commands at
`run.log:1010` and `run.log:1104` both print `--out
<corpus>/runs/2026-09-18-5233bf4-r6`. They cannot drift: the script echoes
`$OUT` (`run.log:547`) and passes `$OUT` (`run.log:549`), which is the fix the
report describes at draft lines 422-425. The fresh-directory guard runs before
either pass (`run.log:536`, `run.log:1007`).

**`items` and `own` agree for all 27 on both targets, and I recomputed the
totals.** Host rows at `run.log:1061-1087`, i686 rows in the i686 section. I
summed both columns myself:

    host: 27 rows, items sum 110582, own sum 110582
    i686: 27 rows, items sum 110582, own sum 110582

The literal string `DISAGREE` occurs exactly once in the whole log, at
`run.log:625`, which is the script's own `printf` template - so no workbook
disagreed. `agree` occurs 54 times, which is 27 x 2. The printed totals
`items_total=110582 bytes_total=1497107 empty_tables=0 max_items=54472` appear
for host at `run.log:1092` and for i686 at the matching line, identical. The
report's table at draft lines 372-375 reproduces them exactly. The independent
counter is Python throughout (`run.log:304-326`: `zipfile`, a regex over
`[Content_Types].xml`, then `re.findall(rb"<si[ />]", body)`), with four
controls (`run.log:491-503`, results at `run.log:980-989`) - three zip fixtures
and one non-zip. The report's description at draft lines 366-370 and 384-389 is
accurate.

**The totals do match the spec's figure, but the spec section the report names
does not exist.** `specs/0010-reading-cells.md:65` gives "110582 `<si>`" and
`specs/0010-reading-cells.md:39-40` gives "the largest shared strings part
2748976 bytes"; `specs/0010-reading-cells.md:68-69` gives "no `<rPh>` and no
`<phoneticPr>`". All three figures are right. But `grep -n "^#"
specs/0010-reading-cells.md` shows the section is called **"Facts planning
checked (2026-09-18)"** (`specs/0010-reading-cells.md:31`). There is no section
titled "The corpus" in that file, and `git log --all -- specs/0010-reading-cells.md`
shows the file has only ever existed in one version, so there never was one. The
citation `specs/0010`, "The corpus" is wrong everywhere it is used:

- draft report lines 59-60 and 378-379;
- `crates/emend/src/cells.rs:37` and `crates/emend/src/cells.rs:97` - committed
  source;
- `records/logs/m1p3-p1/run.log:39` and `run.log:157` - a committed log.

Neither earlier check caught this, and the report does not raise it. The figures
are sound; the pointer is not.

### Recomputation of the record pair and `bc5aba5`'s message

`records/m1p3-p1-2026-09-18-5233bf4.md` and `.txt` carry no sha256 at all. Their
counts I recomputed from the file itself: 27 rows per target, `items` column sums
to 110582 and `own_si_count` to 110582 on both, which matches
`records/m1p3-p1-2026-09-18-5233bf4.txt:5,38` (`items_total=110582
bytes_total=1497107 max_items=54472`) and `.md:19-20`. `rows=870
identical=870 ... crc_differs=0 size_differs=0` at `.txt:6,39` matches
`run.log:1052,1146`. Their own bytes are 1175 and 1839 and their sha256 are
`96d1991c...` and `7892e74d...` - I recomputed all four and all four match the
report's table at draft lines 515-516 and `scan.log:1120,1130`.

`bc5aba5`'s message cites six byte counts: 64895, 93697, 118488, 68250, 64986,
15781. All six recomputed with `wc -c` and all six correct.

**A defect in the committed record.** `records/m1p3-p1-2026-09-18-5233bf4.md:13`
says the independent count was "taken over the extracted part by python3 and
grep". There is no grep in the counter: `run.log:304-326` is a single `python3`
heredoc. This is the same misdescription check 2 raised
(`independent-check-2.md:599-605`) and the report claims to have corrected -
draft lines 369-370, "An earlier draft said `grep -o`; independent check 2 was
right that the counter is Python throughout". The report fixed its own wording
and left the wording in the committed record, which is the primary a later
reader will cite. Neither the report nor any log mentions it.

### Other findings in the logs that the report does not raise

**(a) An inherited comment in `scan.log`'s printed script makes a false claim
about THIS phase's independent checks.** `records/logs/m1p3-p1/scan.log:118-119`
reads:

    # f_home is new in this run. Both independent checks of this phase found an
    # absolute path in negative.log and both observed that no form here could
    # catch it

Neither is true of this phase. `records/logs/m1p2-p3/scan.log:128-129` carries
the identical sentence, so the text is inherited from the previous phase, where
it WAS true (`records/logs/m1p2-p3/independent-check-1.md:30` and
`m1p2-p3/independent-check-2.md:32` both report an absolute path in that phase's
`negative.log`). This phase's checks report the opposite:
`records/logs/m1p3-p1/independent-check-2.md:490` says "no absolute machine
path", and check 1 reports none. "f_home is new in this run" is also false -
check 1 lists `f_home` among the forms with controls in the version it read
(`independent-check-1.md:290`), and `specs/0010-reading-cells.md:286` requires
the scan to carry "the narrowed `f_home`" as an existing thing.

This matters because it is the same defect class, in the same comment block, as
the one the report is proud of catching. Draft report lines 139-143 describe
fixing the literal user-directory prefix that used to sit at
`m1p2-p3/scan.log:130` - and that half is genuinely fixed
(`m1p3-p1/scan.log:120-121` now describes the shape instead of writing it). The
false sentence three lines above it survived the rewrite, uncaught by the
self-scan (which looks for path shapes, not for stale claims) and unmentioned in
the report.

**(b) `test.log:9` contradicts `test.log:1479`.** `test.log:9` prints "at entry:
status lines 3, all of them this phase's logs". `test.log:1479` prints "status
lines: 3; untracked under the log directory: 1; tracked and modified under it: 0;
the phase's record pair: 2". Two of the three are the record pair, not logs. The
sentence at line 9 is hard-coded prose around a computed count
(`test.log:101`), so nothing checks it.

**(c) Check 2's finding 7 is not fixed and not mentioned.**
`independent-check-2.md:673-680` reports that `run.log`'s record-privacy check
runs over the `.md` only. In the committed log it still does: `run.log:741`
passes `"$REC"`, and `"$RECT"` - the file that carries the per-workbook rows - is
never handed to it, while the check is labelled "the record names no corpus file
name" (`run.log:1210`). The property does hold, because `scan.log:1120,1130` puts
both files through `f_deny` and the rest, but the check as labelled is narrower
than its label. The report does not list this among the checks' findings and does
not answer it.

**(d) Check 1's finding (iv) is not answered anywhere.**
`independent-check-1.md:485-508` names three of the report's strongest claims as
resting on mutation runs that no log prints: the `keptnot text` probe, the
catalogue test biting in both directions, and the `<rPh>` counterfactual. The
third was dealt with by `5233bf4`. The first two are still asserted in the draft
report as bare history - draft lines 122-124 ("shown to bite by dropping the
namespace guard, which made it read 'keptnot text'") and draft lines 151-155
("shown to bite in both directions on the real files before the commit ... it
reported 'raised but not catalogued' and 'catalogued but never raised' in turn").
I confirmed that the two assertion strings exist at
`crates/emend/src/package/tests.rs:2159-2160` and that the test does read all
three modules (`crates/emend/src/package/tests.rs:2127-2135`), so the claims are
plausible, but no log prints either observation. The report's own section header
says "I have not softened anything, and where I disagree I say so below" (draft
line 202); this finding is neither accepted nor disagreed with - it is dropped.

### Things I checked that hold

- `cargo test --workspace` in my scratch copy: 280 passed, 0 failed, 2 ignored,
  summing the per-binary `test result` lines (198+12+13+16+11+10+10+4+3+3=280).
  `test.log:918` and `test.log:1244` print the same for both targets;
  `test.log:1475` asserts identical test names on both. `cargo fmt --all
  --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean.
- The cells module's 20 tests, named one by one at `test.log:1392-1411`, are
  exactly the 20 in `crates/emend/src/cells/tests.rs`, and split 12 spec cases
  (`specs/0010-reading-cells.md:312-321`) plus the 8 the report lists at draft
  lines 85-100. 12 + 8 = 20. The arithmetic check 1 complained about is fixed.
- The integration-test threshold did not move: "at least twelve" at
  `test.log:1432` and at `records/logs/m1p2-p3/test.log:1312`. The previous
  phase's total 257 is read from the committed log, not typed
  (`test.log:1275-1276`).
- `gate.log`: nine PASS lines printed twice (`gate.log:1059-1067,1074-1082`),
  asserted at `gate.log:1070`; the `doc` step by name (`gate.log:1071`); the
  `licences` step with quick-xml in the core's tree (`gate.log:1072`); all four
  binaries executable on both targets (`gate.log:1089`); base resolved from git
  as `7702f6e`, the parent of `7175092` (`gate.log:770-771`); three terms in the
  working-tree accounting summing to the total (`gate.log:1092`, 1+0+2=3).
- `negative.log`: eight families (`negative.log:20`), N23 to N28 present as six
  cases with the usage-text check (`negative.log:1624-1662`) - check 2's "N23 to
  N27" heading complaint is fixed; T1 to T4 (`negative.log:1829-1861`), with
  T1's line asserted in full (`negative.log:1833`, `strings items=2 bytes=2`) and
  T4's `items=0 bytes=0` (`negative.log:1858`); D5 inverted with the same message
  and the previous phase's opposite verdict read out of its committed log
  (`negative.log:1815-1827`); catalogue coverage over three source modules
  (`negative.log:1413`) and two test files (`negative.log:1864-1867`).
- `run.log`: the no-edit acceptance, 870 entry rows per target all `identical`
  with every other counter 0 (`run.log:1052,1146`), `files_identical`,
  `order_preserved`, `lo` and `lo_in` 27 of 27 on both targets
  (`run.log:1043-1050` and the i686 equivalents).
- No log or check answer holds a non-ASCII byte, a control byte outside tab/LF/CR,
  or a path under a home or Windows-host prefix. My own greps: 0 for all eight
  files on the first two, and the single `/home/` hit
  (`scan.log:346`) is the documented `f_home` negative control, built by
  concatenation with a fictional account name, which is not a leak.
- `--shared-strings` is as described: `tools/emend-copy/src/main.rs:446` formats
  `strings\titems={items}\tbytes={bytes}`, `bytes` being the sum of
  `item.len()` (UTF-8 lengths, `main.rs:441-445`), and both reading operations
  yield one line through the same reporting path (`main.rs:184-188`).
- `Package::shared_strings` returns an owned table and parses on every call
  (`crates/emend/src/package.rs:557-567`), where `content_types`
  (`package.rs:576`) and `workbook` (`package.rs:628`) return cached references.
- Questions 8 and 9 are real, and I reproduced both in the scratch copy with the
  unmodified reader:
  `cdata_only=[""] cdata_mixed=["ac"] child=["abarb"]` for
  `<si><t><![CDATA[hello]]></t></si>`, `<si><t>a<![CDATA[b]]>c</t></si>` and
  `<si><t>a<foo>bar</foo>b</t></si>`. Exactly the values the report gives at
  draft lines 486-488 and 504-505.
- Question 4 is real: I ran both shape patterns over
  `z=$(isnot LC_ALL=C "$G" -q -E pat "$REC")` and both scored 0, so neither
  `f_shape` nor `f_bang` sees an assignment prefix, as the report says at draft
  lines 433-437.
- Question 5's reasoning is right: `the_table_is_read_afresh_on_every_call`
  (`crates/emend/src/cells/tests.rs:371-382`) asserts only `first == second` and
  the item text, which a caching-plus-clone implementation would satisfy.
- Question 7 is right: `specs/0010-reading-cells.md:201` does read "the corpus's
  largest is 2748976 bytes of XML and 110582 items", and `cells.rs:35-38` now
  separates the two.
- Six logs is right, and part 2 P3 did have seven:
  `records/logs/m1p2-p3/` holds `gate`, `negative`, `run`, `scan`, `self-test`,
  `size`, `test` plus two check answers. `specs/0010-reading-cells.md:281-283`
  lists `size.log` as P3's. Five of the six print a script; `self-test.log` does
  not.

---

## 3. Does the report tell the truth?

Mostly, and it is unusually forthcoming about its own errors. Every figure and
quoted string I could locate is below. The exceptions are grouped at the end.

### Figures and strings that check out

| report claim | where it is |
| --- | --- |
| the six commits and their subjects (draft 16-21) | `git log --oneline 7702f6e..HEAD`; `gate.log:775-785` |
| four code commits where the spec gives two (draft 11, 23) | `gate.log:781-785`; `specs/0010-reading-cells.md:293,310` |
| `len()` `u32`, saturating; `get()` `None` past the end; `count`/`uniqueCount` not read; no relationship = empty table (draft 37-44) | `crates/emend/src/cells.rs:72-85,54-59` |
| quick-xml splits a run at every general reference (draft 45-47) | `crates/emend/src/cells.rs:14-21,187-197` |
| fixture put the nested `<t>` last; new fixture fails `["ayb"]` vs `["ab"]`; reader unchanged (draft 54-60) | measured by me, question 1 above |
| corpus carries no `<rPh>` or `<phoneticPr>` (draft 61-62) | `specs/0010-reading-cells.md:68-69`; doc comment at `cells.rs:95-99` |
| `shared_strings` owned, not cached and lent (draft 65-69) | `crates/emend/src/package.rs:557,576,628` |
| twenty tests, every one through `Package::shared_strings`, zip builder from part 2's `crate::testing` (draft 71-78) | `test.log:1392-1412`; `crates/emend/src/cells/tests.rs:1-19,100-104` |
| twelve spec cases plus eight others, all eight named (draft 80-100) | `specs/0010-reading-cells.md:312-321`; the 20 names at `test.log:1392-1411` |
| 2748976 is a part's bytes, 110582 the corpus `<si>`, 54472 the largest table (draft 128-131) | `specs/0010-reading-cells.md:39-40,65`; `run.log:1099` |
| scan self-scan 0 with control 1 (draft 141-142) | `scan.log:23,25` |
| catalogue: two new messages, set-equality over three modules, call-vs-literal over all three (draft 147-150) | `crates/emend/src/package.rs:73,77`; `package/tests.rs:2127-2144` |
| `--shared-strings` prints one line, two counts, output byte-identical (draft 158-163) | `main.rs:433-447`; `negative.log:1833-1836` |
| all eight log/check-answer byte counts and sha256 (draft 263-270) | recomputed by me; `scan.log:1048-1109` |
| `self-test.log` PASS, 50 cases, 0 unexpected, 0 errors (draft 263) | `self-test.log:106-108` |
| the four `gate.log` claims and three-term accounting (draft 297-301) | `gate.log:1070-1072,1089,770-771,1092` |
| 280/280, 0 failed, 2 ignored, same names, previous 257 read from the committed log, 20 cells tests named, threshold unmoved (draft 305-309) | `test.log:918,1244,1475,1275,1392-1412,1432`; and my own run |
| negative.log's N23-N28, T1-T4, D5, three modules and two test files (draft 313-331) | `negative.log:1624-1662,1829-1861,1815-1827,1413,1864-1867` |
| run.log PASS, 870 rows, all identical, no flags, 27 of 27 on `lo`/`lo_in` (draft 355-360) | `run.log:1043-1055,1141-1146` |
| the counter is Python, four controls, both targets' totals (draft 366-389) | `run.log:304-326,491-503,980-989,1092`; my own sums |
| the five run.log attempts and their four hashes / byte counts (draft 396-425) | `run.log:6-10` - see the caveat below |
| record pair bytes and sha256 (draft 515-516) | recomputed by me; `scan.log:1120,1130` |
| six commits, all local, working tree clean, `7702f6e..HEAD` gives six (draft 534-540) | `git status --porcelain` empty; `git log --oneline 7702f6e..HEAD` = 6. `origin/main` is at `f4f48e8`, so the local branch is eight ahead, but `specs/0010-reading-cells.md:288-289` says the close-out push "carries the local commits 9bd9102 and 7702f6e", so the two extra unpushed commits are by design and the report's claim about this phase is right. |

### Claims that are false or overstated

1. **"a flag reads 'askippedb'" (draft 86).** False, and false in the same
   sentence the report retracts at draft 50-53. See question 1. This is the most
   serious item in this answer.

2. **"P2 added three for `--survey` and P3 two for `--drop-calc-chain`" (draft
   189).** Half wrong. There are three `--survey` integration tests and two
   `--drop-calc-chain` ones in
   `tools/emend-copy/tests/null_rewrite.rs`, but `git log -L` on each line shows
   P2 (`5fc1b3d`) added TWO `--survey` tests (lines 666, 735) and P3 added one
   more `--survey` (line 699, `11de26e`) plus the two `--drop-calc-chain` ones
   (lines 809, 907, `e04db06`). So "P3 two for `--drop-calc-chain`" is now right
   and "P2 added three for `--survey`" is newly wrong: the report fixed the half
   check 1 flagged (`independent-check-1.md:451-464`) and broke the other half.
   The clause's conclusion - `--shared-strings` has no integration test - is
   correct: `grep -rn "shared-strings" tools/ crates/` hits only
   `tools/emend-copy/src/main.rs:37,69,148`.

3. **"`specs/0010-reading-cells.md:307-308` places this very content in P1's
   FIRST code commit" (draft 25-26).** Two problems. The quoted sentence is not
   at 307-308: `grep -n` puts "the self-test invocation line names the launch
   form" at `specs/0010-reading-cells.md:306`, the sentence running 305-307.
   More substantively, the spec does not place it in a commit. The sentence is
   "The run script's faulty fixture drives every counter by name and the
   self-test invocation line names the launch form, from this phase on (rulings 3
   and 7)" - a phase-level obligation about the run script and the log, sitting
   after the semicolon-separated list of the first commit's contents, which ends
   at "(ruling 7)." on line 305. The ruling it carries is explicit about this
   distinction: `specs/0009-package-layer.md:1217-1219` says the invocation line
   "names the launch form from part 3", while the very next clause,
   `specs/0009-package-layer.md:1222-1223`, says the overlong comment lines "are
   rewrapped in part 3's first code commit". One item is assigned a commit and
   the other is not. Check 1 put this more carefully - "puts it inside P1's FIRST
   code commit PARAGRAPH" (`independent-check-1.md:36-37`) - and the report
   upgraded "paragraph" to "commit". The report's conclusion (the content is what
   the spec asked for; only the shape departs) survives; its cited authority does
   not say what it is said to say.

4. **"Every other log's `about:` line names the phase commit and asserts HEAD
   equals it" (draft 216-218), and "`negative.log`, `run.log`, `test.log` and
   `gate.log` each assert that HEAD equals the phase commit" (draft 454-456).**
   No log asserts this. In `gate.log:88-90` and `scan.log:102,214` the phase
   commit and "HEAD at run" are the SAME variable, printed twice from one
   `git rev-parse HEAD`, so the halves cannot ever disagree. In `test.log:51,76`,
   `negative.log:144,163` and `run.log:142,330` the phase commit is a required
   argument and HEAD is read from git, so a reader can see whether they agree -
   but there is no `note`, `check` or `structural` line comparing them anywhere:
   `grep -n "HEAD equals\|HEAD is the phase commit"` over all six logs returns
   nothing. If HEAD had moved, all five logs would still have printed "all checks
   as expected". The logs being re-run at `5233bf4` is the project's convention
   and the rerun notes say so, but the report's stated cost of the rename in
   question 5 ("a rename costs a re-run of four logs for a name") rests on an
   assertion that does not exist.

5. **`specs/0010`, "The corpus" (draft 59-60, 378-379).** No such section; it is
   "Facts planning checked (2026-09-18)" at `specs/0010-reading-cells.md:31`.
   The figures are right. Same wrong citation in committed code
   (`crates/emend/src/cells.rs:37,97`) and in a committed log
   (`run.log:39,157`).

6. **"It records no edit, so the output is byte-identical" (draft 164).** True
   as measured (`negative.log:1835`, `run.log:1052`), but "so" overstates: the
   byte-identity comes from the write path, not from the absence of an edit; the
   report elsewhere makes the same point correctly when it says an empty table
   would satisfy the acceptance too (draft 362-364). Minor.

7. **"the first one where I put the false claim in a commit message" and "the
   eighth recorded instance of this project's most-recorded defect class" (draft
   106, 113-114).** No register of that class exists in the repository:
   `grep -rn "defect class" specs/ ROADMAP.md records/*.md` returns nothing.
   `5233bf4`'s message says "eight times" and is the only source. Unverifiable
   from primaries either way. The same goes for "This was probed against the
   crate before the reader was written" (draft 47-48) and the two mutation runs
   in item (d) of question 2.

### "The independent checks, and what they actually read"

I checked each of the three provenance claims.

1. **"Both checks read the tree at `001fe24`, not at `5233bf4`" (draft 207).**
   TRUE. `independent-check-1.md:18-24` lists exactly four commits ending at
   `001fe24`; `independent-check-2.md:11,30-31` likewise ("one doc commit and
   THREE code commits", from a `git archive HEAD`). Neither can have seen
   `5233bf4`. The rest of the paragraph is right too: the diff is nine comment
   lines and two fixture lines (I counted), and no independent reader had checked
   it before this answer.

2. **"Five of the six logs they read have been superseded" (draft 213-224).**
   TRUE, and the line numbers prove it independently of the rerun notes. Check 1
   cites `negative.log:1791-1803` for D5 and `:1805-1812` for T1; the committed
   file has them at 1815-1827 and 1829-1836 - a uniform offset of 24. Check 2
   cites `negative.log:1338-1348` and `:1959-1963`; committed, those sections are
   at 1362 and 1983 - the same offset of 24. So both checks read the same
   `negative.log`, or two versions with identical numbering, which is mildly
   reassuring about claim 3. Offsets differ for `gate.log` (check 1's 788-807
   against committed 814-832) and `scan.log` (check 1's 997-1037 against
   committed 1048-1088), as supersession implies.

3. **"I replaced `negative.log` while check 2 was still reading ... landed at
   17:14 and check 2 was working until 17:18" (draft 225-232).** UNVERIFIABLE
   from any primary. No committed file carries either time. The latest run time a
   check answer cites is 16:56:06 (`independent-check-1.md:294`), and the
   committed `negative.log:3` is 17:28:55, after both checks; so the ordering is
   consistent with the story but the two specific timestamps rest on the report
   alone. The uniform 24-line offset in (2) is positive evidence that check 2 did
   not straddle two versions of that file. I would have planning read the
   disclosure as honest but unprovable rather than as a measurement.

4. **"between them they found ... four wrong figures in this report" (draft
   199-201).** The "four" is anchored: `independent-check-2.md:681-684` lists
   exactly four ("five controls", "grep -o", the drop-calc-chain attribution, and
   the twelve-plus-five arithmetic). Three are fixed. The drop-calc-chain one is
   half-fixed and half newly broken - item 2 above.

5. **"I have not softened anything, and where I disagree I say so below" (draft
   201-202).** Overstated. Two committed-check findings are neither accepted nor
   disagreed with: check 1's (iv) on unprinted mutation runs
   (`independent-check-1.md:485-508`) and check 2's 7 on the record-privacy
   check's scope (`independent-check-2.md:673-680`, still unfixed at
   `run.log:741`). Neither appears anywhere in the draft report.

6. **The disagreement with check 2 about amending (draft 234-242).** Fairly
   stated. `specs/0001-bootstrap.md:76-77` does say "Before the report, the
   implementing session may amend or squash its own unpushed, unverified commits
   freely", and `independent-check-2.md:53-55,685-687` reads it that way. The
   report concedes the finding and defends only the decision not to rewrite. I
   would add that the same sentence continues at
   `specs/0001-bootstrap.md:78-79`: "the doc commit, the code commit(s) and the
   record commit stay separate, so a phase lands in three or four commits." This
   phase landed in six. The report accounts for the departure from `specs/0010`'s
   two code commits and does not mention this second sentence at all.

### "State, and what I have not done"

All eight statements hold as far as I can check them.

- "all LOCAL", "Nothing has been pushed": `origin/main` is at `f4f48e8` and the
  local branch is eight commits ahead, so none of this phase's six is on the
  tracking ref. Two earlier commits (`9bd9102`, `7702f6e`) are also unpushed,
  which `specs/0010-reading-cells.md:288-289` says is deliberate. I cannot query
  the remote itself; the tracking ref is the best primary available.
- "The working tree is clean": `git status --porcelain` is empty.
- "`git log --oneline 7702f6e..HEAD` gives the six": it does.
- The four things not changed (CDATA, `text` flag, absent target, the rename):
  confirmed unchanged in `crates/emend/src/cells.rs` and
  `crates/emend/src/cells/tests.rs:371`, and I reproduced the CDATA and
  child-element behaviours myself.
- "I did not edit `specs/0010:201`": the file's only version is the one committed
  at `7175092` (`git log --all -- specs/0010-reading-cells.md`).
- "I did not amend or rewrite any commit. `8f2364d` still carries the false
  counterfactual": true; `git log --all -S"askippedb"` and the message itself.
- "I did not add an integration test for `--shared-strings`": true,
  `grep -rn "shared-strings" tools/ crates/`.
- "I did not re-run `self-test.log`": true, `self-test.log:2` names `001fe24`.
- "I did not launch a third independent check on `5233bf4`. That is the gap I
  would most want closed": this answer is that check, and my verdict on `5233bf4`
  is in question 1 - it does what its message says and nothing else.

---

## What I would put in front of planning, in order

1. The retracted `<rPh>` counterfactual is still stated as fact at draft report
   lines 85-86. Measured false: the flag reads `["ayb"]` and the string does not
   arise from the new fixture.
2. `records/m1p3-p1-2026-09-18-5233bf4.md:13` still says the independent count
   was taken "by python3 and grep". It is Python only. This is committed.
3. `records/logs/m1p3-p1/scan.log:118-119` asserts, in an inherited comment,
   something false about this phase's independent checks, and says `f_home` is
   new when it is not. Same defect class as the one that produced the rule that
   block exists to satisfy.
4. `specs/0010`, "The corpus" is not a section of that file. The wrong citation
   is in committed code (`crates/emend/src/cells.rs:37,97`) and a committed log
   (`run.log:39,157`) as well as in the report.
5. The report says the logs "assert" HEAD equals the phase commit. They print it;
   nothing asserts it. Question 5's cost argument depends on the assertion.
6. Two committed-check findings are unanswered: check 1's (iv) and check 2's 7
   (the latter still live at `run.log:741`).
7. Draft line 189 fixed one half of check 1's finding (ii) and broke the other:
   P2 added two `--survey` tests, not three.
8. The report's and `bc5aba5`'s attribution to `specs/0001` of a rule about
   WHERE in the phase the self-test may run. `specs/0001-bootstrap.md:113-122` is
   silent on it; those conditions are `gate.log`'s.

None of these touches the phase's substance. The feature matches
`specs/0010-reading-cells.md:310-321`, the twenty tests are real zips read
through the package, the corpus run's per-workbook agreement against a genuinely
independent Python counter is the right acceptance and it passed on both targets
with totals I recomputed, and `5233bf4` is a correct, minimal fix to the one
test that did not bite.

---

## Repository state

`git -C <repo> status --porcelain` is empty and `git -C <repo> log --oneline -1`
is `bc5aba5`. All of my experiments ran in a detached `git worktree` under
`<scratch>` with `CARGO_TARGET_DIR` pointed there; the worktree was removed with
`git worktree remove --force` and `git worktree prune`, and `git worktree list`
now shows the main tree only. I modified, committed and fixed nothing. I read no
file under the corpus directory and did not run `scripts/passthrough-run.sh`.

I ran the two forbidden-shape greps over this answer before finishing: 0 and 0.
