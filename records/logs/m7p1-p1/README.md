# M7 part 1 P1 logs

specs/0044-excel-check.md, "P1: the reruns, the keep, the list, the
stage", and planning's P1 brief. P1's doc commit 82c07fb, the phase
commit, committed the spec at sha256
d14a8fe15d3f132850a43b75815e62a76c44125d7b0899436d0dccbd35ac9c2d,
ROADMAP.md at sha256
45b841ae6d8525692a890f0a996de1f3772203ab570d79d9c94c0369655d3417 and
specs/0043-measurement-and-close.md at sha256
5503d96649d39c14de02359902e7cae008b39c9e6093501ae477f56c2ba8fa86.
Planning's rulings A to P, written during P1, are in the working
specs/0044, uncommitted; the gate, the negative and the run take it at
PLANNING_SPEC: sha256
7f767447310864661ce50e7c670483e58cb6ac1dfcd3322cf7c69028c70f201d (572
lines). The phase commit is 82c07fb3e3bc17d83ff381edca4670ab7f32c449;
P1 has no code commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m7p1-p1`.

root: <dry> stands for <phase>/dry, the phase's dry runs' directory (specs/0044 ruling I; specs/0043 ruling Q)
root: <superseded> stands for <phase>/superseded, the phase's superseded directory

The phase's one commit, on Milestone 6 part 6's close-out record commit
3f68f3b:

| Role | Commit | Subject |
|---|---|---|
| doc (phase) | 82c07fb | M7 part 1 P1: commit planning docs (Milestone 7 approved, the part 1 spec, Milestone 6's close-out verification) |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | d596d86ba2aba432c4e94993a528abb37c5c46a81a5a0221635c8a3d4f99f3f8 |
| negative.log | negative log checks all as expected: yes | bed29908b5ac98e30c50e24e2ff3150cb2df9b814c1ecb46d8a24f00bcc1317f |
| run.log | run log checks all as expected: yes | 8f487d3ba65d84f7eeaa776b22cf2e853ddf2366667b942939716b818364f0ee |

The scan's log, scan.log, is written last over these three, this
README, the check's answer and the 118 record files the run installed.
No test log and no size log (ruling F; the spec's "Size").

## Rulings in the working spec

- A to D: `--lo-timeout 600`; the modes (stage directories 700, staged
  files, the list and the zip 600); the zip stored, its departure from
  Milestone 3 part 7 P5's deflated zip printed in one line; the gate
  prunes `<home>/.cache/emend-build/m6p6-p4/target` and
  `m6p6-close/target` by `cargo clean --target-dir`.
- E and J: the flag list is the `USAGE` constant at 82c07fb, read from
  the gate's release binary's usage output; each sample's comparison
  record is the committed record whose "- Tool arguments:" line is
  exactly the flag, not named `-scale-<tier>-`, with the greatest
  "- Record written" time; the record E alone would pick is printed as
  information when it differs.
- F: no test log; the keep is the gate's workspace host test command
  run again by the run log with `EMEND_KEEP_DIR` on that command alone.
- G, H: the spec's line count; the run script's formula check before
  the list (negative case (c) is that function's refusal).
- I: the dry layout under `<dry>/croot`.
- K: the keep was expected to grow by one pair (Milestone 6 part 6 P4's
  test bb0a29e writes a pair through `written`); the difference is a
  FINDING, recorded, nothing fixed.
- L: the doc commit was first made as
  31841f74dbe6037a8a9282f1d34a2d1bbb3a503d and rebuilt as 82c07fb on
  the same tree 60b98e0 and parent 3f68f3b (message rules GN and AD).
- M: the generator readings accepted.
- N: change TA, after dry run d1 (below).
- O: a dry scan's four expected "no" lines; the logged scan prints none.
- P: planning's word for the logged run on d2's conditions.

## What the logs show

- The gate (M6 part 6 P4's gate, its changes continuing to Y89):
  - the one role by full subject, the doc commit the phase commit, its
    parent 3f68f3b; nothing under crates/ changed since M6 part 6 P4's
    phase commit;
  - the prune (ruling D): `<home>/.cache/emend-build/m6p6-p4/target`,
    11237987271 bytes, and `<home>/.cache/emend-build/m6p6-close/target`,
    2144181986 bytes, each carrying CACHEDIR.TAG, removed by
    `cargo clean --target-dir`;
  - check.sh nine PASS lines; the crate's test sets off 539, ch 713, im
    576, ci 755, cc 758, cr 820, all 1201 on host and i686, M6 part 6
    P4's committed figures;
  - the release builds the run reads: emend-copy sha256
    35814fb2d9deb4b5a4536a59a30799657ef4815673261021660210e5961f4bea,
    3954088 bytes; xlsx-diff sha256
    1445523df9522bff56d4f0e03d7bde2ab333bac294020da256de42c01ef71f6b,
    897296 bytes.
- The negative (M3 part 7 P5's list cases, M6 part 6 P4's header): the
  list script's control LC and its refusals L1, L2 and L3 (cases (a) and
  (b)); the run script's formula check, extracted by line range, its
  control FC and refusals F1 (case (c), another sample's directory) and
  F2; the stage assertion, extracted likewise, its control SC and
  refusals S1 (case (d), one byte changed in one staged file, on a copy)
  and S2.
- The run (M6 part 6 P2's run template and M3 part 7 P5's list and
  stage block, changes RU to RZ and TA), 2026-10-25T17:34:17Z to
  23:43:56Z:
  - the usage text's flags ending in `-sample`: 59; run 58, less
    `--new-sample`, in the usage text's order; the stale
    `<repo>/target/release/emend-copy` of 2026-10-11 (40 such flags)
    not used;
  - "reruns done 58 of 58; record pairs written 58; the information
    lines of ruling J: 13; FINDING lines so far: 0";
  - the keep: inputs 2241, outputs 2241; the open record
    `records/m7p1-open-2026-10-25-82c07fb`: pairs 2241, lo_in ok 2229,
    lo ok 2230, the open check's exit 0;
  - the one FINDING line (ruling K): "FINDING: the open record's totals
    differ from records/m6p6-open-2026-10-24-7ac8b74's (now: pairs 2241
    lo_in_ok 2229 lo_ok 2230; then: pairs 2240 lo_in_ok 2228 lo_ok
    2229); recorded, not fixed (ruling K)". The cause is the one pair
    Milestone 6 part 6 P4's test bb0a29e writes, as ruling K expects and
    d1 and d2 showed with the same totals;
  - 118 record files installed (the samples' 116, two per run
    directory, and the open record's pair);
  - the list: rows 6018; by source corpus 1560, fixture 2229, input
    2229; 6 error rows of the printarea directory left out by the list
    script's note; sha256
    46cb94535d01b7a437a06871be6e5f733c60ae91d65280424d5c80dd388d4ae4,
    1496245 bytes, mode 600; no row printed;
  - the stage `<stage>`: 6020 files (the list, the opener and 6018
    under files/), 0 differing from their list rows, directories 700 and
    files 600;
  - the zip `<stage>.zip`, stored: sha256
    d32483dd16074c98103924d4da4b34c8d2d79efc7e8dea37907703ae8db6c162,
    1075262198 bytes, mode 600; entries 6020, every one the staged
    file's bytes, testzip ok.

## The session's statements (no log prints them)

- Generators and scripts, sha256 (the gate's log hashes every file of
  the generator directory):

  | Log | Generator | Script |
  |---|---|---|
  | gate | 456b909ef4f32a0bc2eb1a648afb874e75ead17a6cf8d6eb63c452d76b9f2e19 | a88a5b4d9a94854c150faf16f2df95381a1a49ecd377d47a52b96a4d1d5b835d |
  | negative | 8b75ec05fe276525152a79508b6f5f67c5da3b5cea3f108242df4035a9185f0f | f2bbae1cf858bccc8d3f3c5573c32970ede2dac0b0478682fc7a003b71795131 |
  | run | 8378da341d6be3d85e151dbf7e05d6211dfc6d66cded706e6a9735f7480461f1 | 689dc7f9462cd8a1c59523f5afb2cc4d1691af8f44529cadb70bafa3b861fb94 |
  | scan | 97737c43824dd761ebebf5fb23a4ad662ae5005fb42dbfc6cd6212f3d63cba5d | dda514fab952a462f4507f5560a5dc87072aa7d6bd594c2d68ffea4faaa557de |

- Dry run d1 (PLANNING_SPEC
  0ca0630ca6dfe1ff417a35af436f847c99250b6e1a78adcace56f5d136cc29d8,
  three flags, ruling I's layout): gate and negative "yes"; the run
  exited 1 on two controls alone, "outcome_counts reads a fixture by its
  header" and "other_rows counts no row of the fixture" (the two-table
  fixture; ruling N). Its scan was run by hand after the driver stopped.
  The run script before change TA (sha256
  c1ad9291ed28069554730f2363a5cf6b33d12c335e56468825337688aa1c6e06) and
  its generator's parts are kept under `<superseded>`. d1 is kept under
  `<dry>/d1`.
- Dry run d2 (PLANNING_SPEC 7f767447..., change TA): gate, negative and
  run "yes", exit 0; its one FINDING line the keep growth with d1's
  totals; its scan pre-check RESULT: FAIL with exactly ruling O's four
  lines; outer standard error 0 bytes for every step. d2 is kept under
  `<dry>/d2`.
- The logged run (PLANNING_SPEC 7f767447..., started under ruling P
  2026-10-25T17:27:20Z): gate, negative and run each exit 0 and "yes",
  outer standard error 0 bytes each.

## After the independent check

The check (independent-check-1.md, a heading line then its answer
verbatim, sha256 b1fb793d46aed12e76294efecd3b9468e41f9495ee7e0b0e31621f823dfab1c1
for the answer alone) read this README as it stood before this section
(sha256 2d53f64ae6df1229a40fb6841dc587f983f2912fee0b3237aeac9dd64e31c2e1);
nothing above it changed after the check. Its prompt is the brief file
`<phase>/check1-brief.txt` (sha256
13c69ddf2f1e36c009fc42b82a5c6136cbf512cd1af90f632a3468a76bdbd79f) but for
that file's final newline; the draft inline in it is the frozen draft
byte for byte (session's statement). Its verdicts: 1 yes; 2 yes, with one
wording point; 3 yes with exceptions, no quoted string contradicted by a
primary. Its points, and what the report does with each:

1. **The FINDING lines' wording** (2 c, 3 exception 1): the report's
   FINDING section now lists every output line of run.log the grep finds
   (5788, 5898, 6205, 6206 and the 13 information lines of ruling J),
   beside the 14 lines of the printed script's source.
2. **"2223 lines to 2230" and the old run script's sha256 c1ad9291...**
   (3 exception 2): the session's statements, from the files under
   `<phase>/superseded` and `<phase>/scripts`; the report marks them so.
3. **d2's gate and negative** (3 exception 3): not among the check's
   primaries; d2's gate.log and negative.log are under `<dry>/d2/logs`,
   each ending "checks all as expected: yes" (session's statement).
4. **The dry directories' sizes** (3 exception 4): the session's
   statements; the check's `du -sb` read the same.
5. **The Milestone 2 stage** (3 exception 5): that nothing read, moved
   or removed it is the session's statement; the logs show only that
   this run's stage is another name.
6. **The run's start time** (3 exception 6): run.log's header reads
   17:34:18Z, the driver's line 17:34:17Z; the report gives both.
7. **The session's statements** (3 exception 7): the scan pre-check's
   outer standard error, 0 bytes, is the session's statement; the
   report marks it so.
8. **"Every log ends"** (3 exception 8): the report says the three logs.
9. **The negative's header** (2, other lines): "the run script's two
   functions, extracted by line range (change NH62; ruling H; specs/0044
   P1 case (d))" names case (d) for both extracts, case (c) under it;
   labelling only, the cases themselves named in their lines.
10. **The check's own departures**: it wrote one temporary file in its
   own scratch directory and ran `du -sb` on the two dry directories; it
   wrote nothing in the repository or `<phase>`.

No code, log or record changed after the check; nothing was rerun.
