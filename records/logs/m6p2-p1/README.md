# M6 part 2 P1 logs

specs/0039-chart-kinds-and-formatting.md, "P1", and planning's rulings A
to C written into it during P1. The doc commit 90ad3a1 committed the
spec; the rulings stand in planning's working copy, sha256
7dff6b253a64f8eed92acc6710d2e9bc86ad526f8804b76be610733f86fed745, which
the record commit commits. The gate, test and run logs take it as
PLANNING_SPEC and print it; the negative log takes no such input and
admits the modified spec by its path. The phase commit is
01a305947ef716ce4d56dcf7d0602a88ef311c28, ruling A's fix commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p2-p1`.

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 6d5aa4ba5eefa93a8d3a19d7dea621b7fa86a0a61a2699a014d1a5cbe0798dbc |
| test.log | test log checks all as expected: yes | 3f9a1c14f29bd14cb65276b2009f16af34589eafd75f46dedb261206b143a1f7 |
| negative.log | negative log checks all as expected: yes | 2c87a410836a3c9ab09d09b0d55f1486761163cbf46d22104f5bcf2f8cca77a4 |
| run.log | run log checks all as expected: yes | 6b8452b5e640ca84429eddca2558a47830c59307d51fa22918d8230fea498cb7 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, run, and ended at 2026-10-19T22:45:48Z, 22:47:14Z, 22:47:21Z
and 2026-10-20T00:34:34Z (each log's "run ended at" line), each with
outer standard error 0 bytes (the driver's count of the file each log's
standard error went to; session's statement). Each is the first logged
run of its kind; no logged attempt was refused or stopped. The gate pruned the four
target roots of M6 part 1 (P1, P2, P3 and the close-out), 12080832921,
5737364002, 7801598890 and 2018655097 bytes, each `rm -r exit 0`. The
run log wrote records/m6p2-p1-2026-10-19-01a3059.{md,txt} and
records/m6p2-open-2026-10-19-01a3059.{md,txt}.

## The session's statements (no log prints them)

1. **Dry gate 1 failed; change I71.** The first dry run's gate, at
   0a55f0b, ended "gate log checks all as expected: no" (gate.log sha256
   a98909dd50e5ccb5ea130e982f3455184acf29ae75679f22ad2b14feec7af94b,
   script 89cf7b8f6d511765e5d4873d5ba8f3f884dabee48d4e1c6cf2964122cf130fc4,
   both kept in `<phase>/superseded/dry-gate-1`). A carried check
   expected part 1's sixty-seven chart-write tests where the phase
   commit holds 74, the base's 67 and the 7 this phase adds. The gate
   generator's change I71 reads the count at the phase commit. Planning
   accepted I71.
2. **Dry run 2's run log failed: a finding, fixed by ruling A.** Dry run
   2, at 0a55f0b, ended yes for the gate, test and negative logs, and
   "run log checks all as expected: no" for the run log, sha256
   325bc0a9d9c6d621618e41dc148b725731ffe3e71d2327cfbc5dec488067ad35
   (kept in `<phase>/superseded/dry-run-2/d2`, with the gen1 run script
   that wrote it, sha256
   08a2bffa515224988dc93122092a1809746b795830533ba4c6d9b2777193a7c1, in
   `<phase>/superseded/gen1/scripts`). LibreOffice timed out at 600 s
   on the surface sample's outputs of the two large workbooks,
   9132ce5303d8 and ab0e6a529465, host and i686 alike: the rows lo
   error, the other 25 lo ok. The cause: the sample referenced rows 2 to
   max_row+1, `$2:$130002` on these two, so each surface held 3 series
   of 130001 points and each of-pie 130001 slices. The fix is ruling A,
   commit 01a3059 "M6 part 2 P1: bound the surface sample's references
   (ruling A)": the sample references at most 1000 data rows, the last
   row min(max_row.max(1)+1, 1001), and its line gains `rows`.
   --lo-timeout stays 600 (ruling B).
3. **Gen2.** The five generated scripts were regenerated after ruling
   A (the gen1 generators and scripts kept in `<phase>/superseded/gen1`;
   `<phase>/gen/gate.py.gen1`, which the gate hashes with the generator
   directory's every file, is the gate generator as dry gate 1 ran it,
   before I71, byte-identical to `<phase>/superseded/dry-gate-1/gate.py`):
   the fix commit as the fifth role and the phase commit, the working
   spec admitted at PLANNING_SPEC, the third tool test, the line's ten
   tab fields (the word `chartsurface` and nine keys), and the run log's
   rows checks (change NXR). Dry run 3, at 01a3059, ended yes for all four logs; no script was regenerated after
   it.

## Findings (specs/0039 ruling B)

**Dry run 2's run log, the finding ruling A fixes.** The run log of dry
run 2, sha256
325bc0a9d9c6d621618e41dc148b725731ffe3e71d2327cfbc5dec488067ad35, kept in
`<phase>/superseded/dry-run-2/d2`, failed on two rows, 9132ce5303d8 and
ab0e6a529465, host and i686: LibreOffice timed out at 600 s on the
surface sample's outputs. The cause: the sample referenced `$2:$130002`
on these two workbooks, 3 series of 130001 points per surface and 130001
slices per of-pie. The fix commit is
01a305947ef716ce4d56dcf7d0602a88ef311c28, "M6 part 2 P1: bound the
surface sample's references (ruling A)". No diagnosis file is committed.
The final scan's RERUN_NOTE and RERUN_FILES name the run log and the
superseded run script that wrote it, and hash both (rule F).
