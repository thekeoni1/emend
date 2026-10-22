# M6 part 4 P3 logs

specs/0041-chart-text-and-edits.md, "P3: the probe, the size record, the
API listing, the CHANGELOG", and planning's P3 brief. The doc commit
11e1715 committed the spec (sha256
edaeea436e468a7842d91c7692545d98b26a9e89b339d13104f53e8c15666fbb, holding
rulings A to I and planning's P2 verification) and ROADMAP.md (sha256
9b72d75f6e36f1f8800d9be57eac0fe342c31281b5c3e2e40ba5adb444e9f426).
Planning's rulings J, K, L and M, written during P3, are in the working
specs/0041, uncommitted; every log takes it at PLANNING_SPEC, sha256
c3a19a41652628f07266d5609b0aa745d6167a03212dfdd325caf12705c5a18c. The
phase commit is 1cc0de4ac9fc124bf7768a9da06acd90d0ebf63f, the CHANGELOG
commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p4-p3`.

The phase's commits, each the next one's parent, on M6 part 4 P2's record
commit 6203ddc:

| Role | Commit | Subject |
|---|---|---|
| doc | 11e1715 | M6 part 4 P3: commit planning docs (the P2 verification and rulings) |
| probe | 0217d1f | M6 part 4 P3: probe-emend-charts-write-xml gains the text properties, the style and the in-place edit |
| changelog (phase) | 1cc0de4 | M6 part 4 P3: the CHANGELOG names the chart text and in-place edits |

No carry commit: nothing was carried from P2's check and no doc slip was
found (ruling J's last sentence).

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | f358db593e857b0aee554e0af3b7b11d1e49f873281e67600e3fb641944c0d4b |
| test.log | test log checks all as expected: yes | 5f95b843fd60928260e72adce686a8b77e3d9e592c084fc2b0e4820afba994b9 |
| negative.log | negative log checks all as expected: yes | 53ffae05ad2f66228840c09af28dd16b7da50c4ee728d66cab8a3820e27ac348 |
| size.log | size log checks all as expected: yes | 72233d1fabac90ccd73a615557da30d3edecbfa5f0a8d3c4315814d0546e1f79 |
| run.log | run log checks all as expected: yes | d5ecdcde4cac9b7178801bf867f0cc10a0c46f144868e62a9b7bf60605844d82 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, size, run, and ended at 2026-10-22T08:12:18Z, 08:17:54Z,
08:18:08Z, 08:23:21Z and 11:01:08Z (each log's "run ended at" line), each
with outer standard error 0 bytes (the driver's count of the file each
log's standard error went to; session's statement). Each log is the first
logged run of its kind; no logged attempt was refused or stopped
(session's statement). The run log takes the size log as its second
argument and uses its release binaries.

The test log installed records/api-review-m6p4-2026-10-22-1cc0de4.md; the
size log records/size-m6p4-2026-10-22-1cc0de4.md; the run log
records/m6p4-p3-chartedit-2026-10-22-1cc0de4.{md,txt},
records/m6p4-p3-charttext-2026-10-22-1cc0de4.{md,txt} and
records/m6p4-open-2026-10-22-1cc0de4.{md,txt}.

## Rulings in the working spec

Quoted in full in specs/0041, "Planning rulings during P3 (2026-10-22)".

- **Ruling J** (Q12): after the second add's reopen the probe edits the
  worksheet's one chart, the second set's, by one set_chart_title and one
  set_chart_series; the line count and every printed figure unchanged.
  The doc commit verified; the carry skipped.
- **Ruling K**: the charts movement above 25000 is a finding by "Size and
  packaging", recorded in the size log and not fixed, the report naming
  it; its cause is planning's estimate. The probe commit and the
  CHANGELOG commit verified.
- **Ruling L** (Q13): the API record compares rows on the key without
  added and names separately, under "added-by re-dated by ruling V
  (change AV1)", every row whose added-by differs from the old program's
  at the same phase commit; api-listing.py bb1fba22 is the listing's from
  this part on.
- **Ruling M** (Q14): the size log's widened cause check (change QW,
  gated.py 34eb101a) is the check; the finding under ruling K built from
  the committed gate figures with the steps' sum checked; the scan
  writes to a file, never a pipe; the negative log's NC54 control.

## What the logs show

1. **gate.log:** four roles by full subject (base 6203ddc, doc 11e1715,
   probe 0217d1f, changelog 1cc0de4 the phase commit); of the 3 commits in
   the range 1 planning-doc and 2 code, each message ASCII ending with the
   Co-Authored-By line only; the phase diff the doc commit's two files
   and the code commits' two, four files, nothing under crates/ or
   scripts/, no manifest or lock, no file added; the CHANGELOG bullet of 5
   lines under "### Added" citing specs/0041, no line removed; test names
   1417 at the base and at the phase commit; 155 distinct messages, the
   base's; check.sh's nine PASS lines. No prune in this phase. The i686
   charts probe: sections 1138352; against part 3's size record's 1107960,
   difference 30392; against P1's committed gate.log's 1112212, 26140;
   against P2's committed gate.log's 1136996, 1356; printed, the size log
   judges it (ruling K).
2. **test.log:** ws-host and ws-i686 passed 1414, failed 0; the feature
   runs on both targets 515 (off), 688 (charts), 730 (charts,images), 733
   (charts,comments), 791 (charts,read) and 1170 (every feature), failed
   0, each P2's committed figure. The API block (ruling L): both listing
   programs at the phase commit, the rows whose added commit differs the
   44 typed, each moving to a descendant of its old commit; at part 3's
   phase commit 0707236, part 3's 42 typed, ruling V's four among them;
   the listing 834 rows, part 3's 815 with one gone and twenty new; the
   formatter's line "rows 834; {'keep': 834}; differing from M6 part 3
   P3: 1 gone, 20 new; re-dated by ruling V 44; gate-widened 0, 0 of them
   pub; new under the shared gate 0; Eq dropped 0".
3. **negative.log:** "the 15 cases were refused, N1 to N15"; the eleven
   fixtures written; P3 adds no case, no message and no binary path.
4. **size.log:** the 29 probes built on i686; the format probe 972176
   section bytes and the charts probe 1138352. The record gives the
   feature charts' cost 166176 against part 3's 135784, a move of 30392,
   above the finding line of 25000, a FINDING (estimate 12000 to 20500;
   ruling K), its three steps P1 4252, P2 24784 and this phase 1356 summing
   to 30392; the core's growth 0 (expected 0, accepted up to 2000); the
   images cost's movement 0 and the comments cost's 0, each "as expected";
   the budget 438960; 0 of the twenty-eight other probes grew or shrank;
   the one probe that moved caused by the part's two crate commits, their
   core-source changes comment lines and lines gated on charts alone
   (change QW, ruling M). The release binaries the run log uses are built
   here.
5. **run.log:** with the size log's release binaries, P2's
   --chart-edit-sample and --chart-text-sample corpus runs rerun on host
   and i686, every row's output and line byte-identical to P2's run by
   sha256, 27 of 27 per label and sample (4 of 4); every added chart part
   holds one c:style val 26 right after roundedCorners, the 4 kept chart
   parts retitled keep the input's style. The Excel-form fixture
   (c71ca2a4a1a9) is P2's byte for byte and its three samples' outputs
   and lines on both labels are P2's (6 of 6). A fresh openpyxl fixture
   from the pinned image (specs/0038 ruling P): the six openpyxl lines are
   P2's (6 of 6), the seven field lines P2's byte for byte; P2's kept
   working directory found and its fixture's three samples rerun
   byte-identical. The keep: passed 1414, failed 0; pairs 2215; against
   P2's open record no pair gone, 0 whose bytes differ, 0 new; no output
   refused on an accepted input, crate (2065 pairs) or tool (150).

## The session's statements (no log prints them)

1. **The generators were written by forked subagents** of this session,
   one per log, from P2's generated scripts (gen2, in
   `<phase>/model/m6p4-p2`) with part 3 P3's (`<phase>/model/m6p3-p3`)
   for the P3 shape: the size script from part 3 P3's, the test log's API
   block and formatter from part 3 P3's, the gate's roles, areas, base
   tests and crate totals from part 3 P3's form. Each generator checks its
   template byte-equal to the script printed in the committed log. Labels:
   gate A81 to Z81, test XM to XV, negative NA54 to NF54, size QS to QZ,
   run NZF to NZK, scan KI to KL.
2. **One generation.** Every script is gen1; the hashes are in
   `<phase>/scripts-gen1.sha256` (gate 242da511..., test e2450e7b...,
   negative c5054d3c..., size c44a9d71..., run 3a2387c4..., scan
   f10415bd..., the formatter make_review.py f960fb37...), all seven
   OK by `sha256sum -c` after the logged run. The background command's
   own check before it diffed full paths against the file's base names,
   so it printed a diff and not "scripts unchanged"; the hashes in that
   diff are the file's, line for line. The test fork moved its
   first draft pair (the same script, an earlier formatter) to its
   scratch's `superseded-draft1/` when one sentence of the formatter was
   narrowed. The gate fork moved three earlier outputs and its first hand
   harness run aside.
3. **The forks' departures.** QW (the size log's widened cause check) was
   not in the brief's words; planning ruled it (ruling M). The size log
   drops part 3's typed PLANNING_GAVE (P2's form). The gate fork corrected
   a stale comment carried from P2 ("the logged run gives no
   PLANNING_SPEC", J81). The negative log's messages control is replaced
   (NC54): the base now holds P2's message. The run log reruns against
   P2's two run directories, not P1's (P2's text run was P1's,
   byte-identical). The forks built in their own scratch targets under
   `<phase>/fork-scratch/` for hand checks; none touched `<phase>/target`
   or the repository.
4. **The rm uses.** No fork removed any file. The gate generator made and
   removed its own empty `<phase>/tmp/gate-gen`. The gate log's one `rm -r
   <target>/main/doc` and the other removals in the scripts are carried
   from their templates and remove only what the same script made.
5. **One dry run** ran before the logs, at 1cc0de4 with the same
   PLANNING_SPEC, into `<phase>/dry/d1` (the records into the dry
   directory), and ended yes for all five logs, outer standard error 0
   bytes, no line holding the home directory. The logged logs differ from
   it in the dry run's directory overrides, the times, the build output's
   cached lines and the status lines of the records the logged run
   installed.
6. **The development figures**, at 1cc0de4 (not a log): check.sh nine PASS
   at 0217d1f and at 1cc0de4; the probe at every index 0 to 15 exits 0 with
   31 lines and index 16 is refused, stdout and output files byte-identical
   to part 3 P3's; the dev i686 charts probe 1138352 at 0217d1f; the dev
   keep at 1cc0de4 2215 pairs, 0 new, 0 gone, 0 changed; the session's dev
   API run at 0707236 42 re-dated rows.
7. **The forks' other notes.** The scan's self-check (KL, ruling I) reads
   its standard output through /proc, so the scan runs with its standard
   output redirected to a file. The run log's fresh openpyxl fixture cannot
   match by sha256 across runs (specs/0038 ruling P's reading); its lines
   are compared instead.

## After the independent check

The check (independent-check-1.md, sha256
eb21e98da8dab3ebe9111fd68d2f7c9e5390bd773441fa8ef1b4e244348350fa) read
this README as it stood before this section; nothing above it changed
after the check. The answer is the checker's final message, extracted
verbatim from its transcript; it holds no path form and no non-ASCII
byte. A later note from the checker, after its answer, said only that
the blocked command below had ended and its answer was unchanged.

One of the checker's commands redirected a comparison's output to
/dev/stdin, whose standard input was /dev/null, and blocked; the
session killed that process by its PID. It wrote no file; the check's
directory was unchanged after it.

Planning's ruling N, in the working specs/0041 (sha256
029cb50cb366ecb274196f547581315ee760e9697c154ac27c0cf19735c2914b, which
the final scan takes at PLANNING_SPEC), rules all fourteen points with
no code, log or record change and no rerun:

1. **Stale wording** (points 1, 4 and 6): the gate, negative and run
   logs call the working spec c3a19a41 "rulings J to L" where it holds J
   to M (gate 8 lines, negative 9, run 2); the gate heading at line 9336
   says "two added style lines alone (Q80)" above the 81/19 check (Q81);
   size.log repeats a heading at lines 10648 and 10650. The checks under
   them hold.
2. **The programs** (point 2): api-listing.py bb1fba22 and gated.py
   34eb101a live under `<phase>/gen`, as M3 part 7's program did, pinned
   by test.log (AV1 printed as a diff -u against part 7's program) and
   size.log (gated.py printed whole). Committing them is a scripts/
   change outside this part.
3. **The probe's facts** (point 3): the diff and 0217d1f's message carry
   which series take label text and that round is the four round kinds;
   the sixteen index runs exiting 0 show the writer refused nothing.
4. **The rerun's baseline** (point 5): met through P2's two run
   directories, P2's text run being P1's byte-identical rerun.
5. **The open record's text** (point 7) equals P2's (920f5fdc) because 0
   pairs changed; scan.log (point 8) comes last by design.
6. **The report's wording** (points 9 to 14): corrected in the report, in
   a section after the verbatim answer.
