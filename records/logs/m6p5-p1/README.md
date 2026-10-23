# M6 part 5 P1 logs

specs/0042-gradient-fills-and-named-styles.md, "P1: gradient fills", and
planning's P1 brief. The doc commit 893e12c committed the spec (sha256
68f67c4a95194ea34fb00f972e8a527abc4a109a45fdedc3b49c29d7a59666c0), the
M6 part 4 close-out verification in specs/0041 (sha256
82d246d830d18407eaae5ffd70541adecab1a860841283bf18af0f769f097c99) and
ROADMAP.md (sha256
c927a44e3d739327d3c795c3dd66e98c993c5486527e5a61ad2d80a51440bb34).
Planning's rulings A to L, written during P1, are in the working
specs/0042, uncommitted (carried by P2's doc commit); every log takes it
at PLANNING_SPEC, sha256
2041984dc2671f9962c551b1064262cba3c19e9aaa305a2d115d7afac018fec0. The
phase commit is 029400ee3ddace261988192b4faca394edea3eb3, the fix commit
(ruling I). `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p5-p1`.

The phase's commits, each the next one's parent, on M6 part 4's
close-out record 7359783:

| Role | Commit | Subject |
|---|---|---|
| doc | 893e12c | M6 part 5 P1: commit planning docs (the spec, part 4's close-out verification) |
| core | 6a7644e | M6 part 5 P1: Fill gains a gradient: the types, the composer, the checks and the typed read |
| tool | 6285028 | M6 part 5 P1: emend-copy --gradient-sample |
| fix (phase) | 029400e | M6 part 5 P1: Fill::compose under the rules feature (finding M2) |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 847ab555bcc634870de65670ea5d439d17a653ed15e7e9e2aeb03a2199f0b0ee |
| test.log | test log checks all as expected: yes | 007957527d4ee07d8f0f6d7962fcd822ca9938825f4c2168f05ed3d2fca3d137 |
| negative.log | negative log checks all as expected: yes | 4927b97f5a69a98f9e82ec6cd0344d72e0ad8adef8810389b69b4a2475fb83cc |
| run.log | run log checks all as expected: yes | 00cf932bfcb3b3068f9fb45c2b3cbbc318b4bdd7b7cf01ac042a8d3baf1b2ab3 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, run, and ended at 2026-10-22T21:13:03Z, 21:14:20Z, 21:14:36Z
and 23:48:54Z (each log's "run ended at" line), each with outer standard
error 0 bytes (the driver's count of the file each log's standard error
went to; session's statement). Each log is the first logged run of its
kind; no logged attempt was refused or stopped (session's statement).

The run log wrote records/m6p5-p1-gradient-2026-10-22-029400e.{md,txt}
and records/m6p5-open-2026-10-22-029400e.{md,txt}. The three reruns of
ruling F write no record. P1 writes no API record and no size record.

## Rulings in the working spec

- **A** (R1): `Fill` losing `Copy` and gaining `gradient` forces one-line
  edits outside the core, `gradient: None,` in each struct literal (the
  size probes' among them) and `.clone()` or `.as_ref()` where Copy
  went; not "The probe" (P3's). The ruling's text says "the fifteen
  `probe-emend-*-write-xml` sources"; the count is sixteen files and
  seventeen lines, two in the rules probe (gate.log, change Q82). The
  session first told planning "fifteen"; its own count is sixteen.
- **B** (R2): the three tests that pinned "a gradient reads as None"
  rewritten to the typed read and renamed (gate.log and test.log name
  the three gone/new pairs).
- **C** (R3 to R5): the namedstyles, read and roundtrip samples meet a
  gradient typed; the null-rewrite test's namedstyles figures 1, 3, 4,
  4; the roundtrip's None-fill case kept on an empty `<fill/>`. Its
  corpus clause is replaced by F.
- **D** (R6): the typed read takes any finite number, no range check; a
  `gradientFill` with no stop reads with an empty list.
- **E** (R7): the gradient's checks run after alignment and protection
  in `add_style`, after the colours in a dxf.
- **F**: the corpus's six `gradientFill` elements sit inside `extLst`,
  none in `<fills>` or the main `<dxfs>`, so no move is expected on the
  corpus for the three samples; the run log reruns them on both targets
  and compares each line with the earlier run's, and prints its own
  extension count.
- **G** (R8): the stale labels of specs/0041 ruling P are renamed at
  part 5's close-out (changes 56, 57); P1's generators rename nothing.
- **H** (finding M1): the two code commits rewritten with identical
  trees, the count rule's sentence added.
- **I** (finding M2): `Fill::compose` gated, in a third code commit on
  top, the phase commit; 6a7644e and 6285028 stay, failing clippy off
  `rules`.
- **J** (R10): the attribute is `#[cfg(any(test, feature = "rules"))]`,
  since the element unit tests call `Fill::compose` in every set.
- **K** (R11): a gradient range is reachable through the binary by the
  roundtrip and namedstyles samples; negative cases N16 to N19 and
  controls B11 to B13.
- **L** (finding M3): ruling F's roundtrip reference amended from
  2026-10-16-5e52758-roundtrip (a null round trip, empty tool_args) to
  2026-10-05-6b6125b-roundtrip; planning's word for the logged run.

## What the logs show

1. **gate.log:** five roles by full subject, the parent chain
   7359783, 893e12c, 6a7644e, 6285028, 029400e; ruling H's tree check
   (core tree dfbde7d2... equal to 0f8d809's, tool tree be8917bb...
   equal to 4bb297d's); finding M2 named at 6a7644e and 6285028 and not
   rerun; commits in the range 4, one planning-doc and three code;
   distinct messages 155 at the base and at the phase commit; test names
   1417 at the base and 1432 at the phase commit, the three ruling B
   renames the only names lost; the fix commit's change its four added
   lines and one removed; the crate commit's main.rs change 36 added and
   17 removed lines; check.sh's nine PASS lines; clippy and doc clean on
   every set; the crate's tests on both targets off 523, charts 696,
   images 560, charts,images 738, charts,comments 741, charts,read 802,
   every feature 1182, each part 4 P3's committed figure plus the
   tests the set compiles. The sixteen probes' change: seventeen added
   `gradient: None,` lines, none other, none removed. The i686 charts
   probe 1140868 section bytes, 2516 over part 4's size record's
   1138352; the format probe 974636, 2460 over the record's 972176, its
   budget 441420 against the record's 438960; printed and judged
   nowhere (P3's size log judges). Part 4's target roots pruned (`rm -r`
   of `target` alone): m6p4-p1 none there, m6p4-p2 10806750084 bytes,
   m6p4-p3 8167491881, m6p4-close 2118373673.
2. **test.log:** ws-host and ws-i686 passed 1429, failed 0; the feature
   sets on both targets off 523, read 629, rules 559, read,rules 668,
   every feature 1182, failed 0; the gradient module's tests per set;
   the three ruling B names under their new names only; clippy -D
   warnings clean on the crate's five sets, the workspace and the size
   probe's emend,xml, charts and images; rustdoc clean on the five.
3. **negative.log:** 19 cases refused, N1 to N19, and 14 accepted
   outputs, B0 to B13; this phase's N16 (an unreadable degree, "a style
   cannot be read"), N17 (no `<fills>`, the collection message), N18
   (the roundtrip sample on a one-stop gradient) and N19 (the
   namedstyles sample on a dxf gradient at degree 400), both "a style
   field is out of range", each exit 2, no stdout, no output file; B11
   to B13 their controls; 18 fixtures.
4. **run.log:** release binaries built at the phase commit, both targets.
   --gradient-sample over the corpus on host and i686: 27 of 27 rows ok
   on each label, lo and lo_in ok 27, nothing dropped, added or
   recompressed; entry totals identical 816, changed 54; read_back,
   added and fills_added 54 of 54; gradients_before 6 in 2 workbooks
   (18d1e8ba72b5 4, f9efcac9808f 2); outputs byte-identical across the
   labels, 27 of 27. Ruling F: namedstyles (against
   2026-10-12-6f51899), read (2026-10-16-5e52758) and roundtrip
   (2026-10-05-6b6125b, ruling L) each 27 of 27 lines equal on both
   targets, no field moved; the extension count "total extLst 6 fills 0
   dxfs 0 other 0; workbooks 2 of 27". The Excel-form fixture (id
   b9335e28dcc0, 2402 bytes): --namedstyles-sample named=3 named_xfs=3
   builtin=2 custom=0 resolved=3, the output the input unedited;
   --gradient-sample added=2 xfs_before=4 xfs_after=6 fills_added=2
   cells_before=4 cells_after=6 read_back=2 gradients_before=2; both the
   same on both targets byte for byte, lo and lo_in ok; the forms' ten
   checks hold, two refused with a planted stop. The keep: passed 1429,
   failed 0; pairs 2227, 12 new (exactly this phase's), 0 gone, 2 whose
   bytes differ (exactly ruling C's two, by name); the open record's
   totals lo_in ok 2215, lo ok 2216, no output refused on an accepted
   input, no verdict of the named record's pairs moved.

## The session's statements (no log prints them)

1. **The generators were written by forked subagents** of this session,
   one per log, from the templates the facts file named: gate and run
   from part 4 P1's scripts (the gate's consumer crate from part 4 P3's
   gate, gen2); test, negative and scan from part 4 P3's. Each generator
   checks its template byte-equal to the script printed in the committed
   log. This session read each fork's notes and ran the dry runs. Labels:
   gate A82 to Z82 (and K82), test XW to YB, negative NA55 to NF55, run
   NZL to NZP, scan KM to KP.
2. **Implementer errors.** M1 (ruling H): the code commits' messages
   first stated counts without the count rule's sentence (0f8d809,
   4bb297d); found by the scan fork, rewritten, nothing ran against the
   old hashes. M2 (ruling I): the session's development clippy ran with
   every feature only, and missed `Fill::compose` dead without `rules`;
   found by the test fork's hand run. M3 (ruling L): the facts file and
   the session's proposal named 2026-10-16-5e52758-roundtrip, a null
   round trip, as the earlier --roundtrip-sample run; found by the run
   fork.
3. **The negative fork's correction** (ruling K): the facts file said no
   gradient range is reachable through the binary; the fork found the
   roundtrip and namedstyles samples reach the writer's ranges and
   added N18 and N19.
4. **Regenerations.** Gate: gen1 (71f4b045...) failed the first dry run
   on two blocks (the format probe built without `--features
   emend,xml`; the consumer crate's NewChart literal lacking part 4 P2's
   `style`); gen2 (eb87a812...) fixes both, gen1 moved to
   `<phase>/fork-scratch/gate/superseded-gen1/`. Test: three
   generations (the old pins, ruling H's, the fifth role), the earlier
   two moved aside. Negative: gen1 and gen2 failed the fork's own hand
   harness (a one-line literal reader; known.tsv lacking the styles
   messages; the fixture count), moved aside. Scan: regenerated after
   M1 and after M2, the earlier scripts moved aside. No log ran a
   superseded script.
5. **The dry runs**, at 029400e, PLANNING_SPEC the working spec's hash:
   `<phase>/dry/d1` the gate alone, "no" (the two blocks above; the
   driver stops at the first failure); `<phase>/dry/d1b` the test and
   the run, both "yes"; `<phase>/dry/d2` all four with the gate's gen2,
   all "yes", outer stderr 0 bytes each.
6. **The rm uses.** No fork removed a file of the repository or of
   another phase. The prune (`rm -r` of each part 4 root's `target`
   alone, no -f) is printed in gate.log. The other removals are carried
   from their templates and remove only what the same script made: the
   gate's doc build directory (`rm -r`), the counting library's `rm -rf
   "${d:?}"` of its own control directory, the negative's `rm -r
   "${B:?}"` and the scan's `rm -r "${T:?}"` trap.
7. **The development figures**, at 029400e (not a log): workspace 1429;
   the dev keep 2227 pairs, 12 new, 2 differing by ruling C.

## After the independent check

The check (independent-check-1.md, sha256
0ca007354bf499fe7678f186639b744c4fb8f0aa75e82ce0deddcf8a96ad0198, its
answer verbatim) read this README as it stood before this section;
nothing above it changed after the check. Its points:

1. **The catalogue entry** (its 1 a): package.rs's catalogue entry for
   "a style field is out of range" gains the gradient's limits in the
   core commit 6a7644e; "P2" lists "the catalogue entry and limits"
   among P2's work. An addition to an existing entry, made early. Told
   to planning; not changed in this phase.
2. **The dxf order** (1 b): in `add_differential_style` the fill's
   gradient is checked inside `Fill::compose`, after the fill's own
   colours and before the border's; the session read ruling E's "in a
   dxf after the colours" as the fill's colours. Told to planning.
3. **scan.log and the record commit** (its 2): both come after the
   check, as the order in "Phases" gives.
4. **The report's directory counts** (3): the draft gave
   crates/emend/src/package/tests 6 and folded crates/emend/src/styles
   into crates/emend/src, summing to 34. By `git diff --numstat 7359783
   029400e`: crates/emend/src 3, crates/emend/src/styles 2,
   crates/emend/src/package 1, crates/emend/src/package/tests 5,
   crates/emend/src/styles/elements 1, tools/emend-copy/src 1,
   tools/emend-copy/tests 1, tools/size-probe/src/bin 16, specs 2, the
   top level 1: 33. The session's counting error; the final report
   carries the corrected counts.
5. **Quotes from the spec alone** (3): "method `compose` is never used"
   is the test fork's hand run's clippy output (ruling I quotes it);
   gate.log names M2 and does not rerun it. `git commit-tree` is the
   session's statement of how M1 was fixed (ruling H quotes it).
6. **Nothing pushed** (3): the session's statement; origin/main is
   f630f67, the part 4 close-out doc commit.
7. **M3's wording** (3): ruling F named 5e52758 for the round trip on
   the session's proposal; ruling L amends it.
