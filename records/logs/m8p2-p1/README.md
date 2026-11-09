# M8 part 2 P1 logs

specs/0048-the-two-refusals.md, "The unshare (`rowcol`)", "The
translator", "Errors", "Size", "Harness runs" and "### P1: the unshare",
with planning's ruling A. P1's doc commit 02ba09d committed the spec at
sha256 7585b92479c8b30e677cda1999e1b7bd74a56e1a14b17789fcfaf20c27f48a1f.
Ruling A, written after the dry run, is in the working specs/0048,
uncommitted. The gate, the run and the scan took it at PLANNING_SPEC
e53fcd31bd6e542c0e85e704979ff257fe877db42555db8121722cce5294f654.
`<phase>` is the phase's directory,
`<home>/.cache/emend-build/m8p2-p1`.

root: <home> stands for the home directory of the account the runs ran under
root: <phase> stands for <home>/.cache/emend-build/m8p2-p1, the phase's directory
root: <corpus> stands for the private corpus directory the run reads
root: <runs> stands for <corpus>/runs, the corpus's run directories
root: <target> stands for <phase>/target, the phase's one cargo target root
root: <work> stands for run.log's working directory, <phase>/tmp/tmp.X1KdtRYBQx, as run.log's own root line says
root: <scratch> stands for the implementing session's scratch directory, which held independent check 1's materials

| Role | Commit | Subject |
|---|---|---|
| base | 454dea3 | M8 part 1 close-out record: the push, the CI run and the scan |
| doc | 02ba09d | M8 part 2 P1: commit planning docs (the spec, the part 2 block, part 1's close-out verification) |
| code (phase) | 2740ce7 | M8 part 2 P1: an insertion or deletion unshares a shared formula group it cuts |

No file name, sheet or table name, note or cell content of any private
workbook is in these logs or in this README. Workbooks are named by id,
the first 12 hex digits of the sha256. The texts quoted below are the
logs' own lines.

## The logs

| Log | Bytes | Lines | sha256 | Last line |
|---|---|---|---|---|
| gate.log | 929267 | 12084 | ba3e318fb6e8b831bbfb9f4db356a033e16bc491916611dd09ee39afca0e46e2 | gate log checks all as expected: yes |
| run.log | 233713 | 3284 | 194f6dd4081b59d6ad2b11d29ba8fe29a31608aa1afb17dae4d0245d0bd93e07 | run log checks all as expected: yes |
| scan.log | | | | the final scan, written after independent check 1 and before the record commit, which holds it; its figures are in the report |

The scripts: <phase>/scripts/m8p2-p1-gate.sh,
-run.sh and -scan.sh. Each log prints the script that wrote it. Each was
written by the generator under <phase>/gen, from part 1 P2's scripts as
numbered changes. Both runs were started by <phase>/logged.sh on
2026-11-09:
- the gate from 14:19:59Z to 14:32:40Z, with PRUNE_PREVIOUS=yes;
- the run from 14:32:40Z to 14:49:13Z.

Each exited 0 with 0 bytes on its outer and own standard error. Neither
log holds an UNEXPECTED line. Dry run d1, under <phase>/dry/d1, ran the
same three scripts first.

## The gate

- The prune (gate.log lines 181 to 350; specs/0044 ruling AD):
  - Part 1 P1's target was already gone ("names of the prune set there
    (M8 part 1 P1's directory): 0 of 1").
  - Part 1 P2's target, 6336300051 bytes (line 289), was removed by
    `rm -r "${PREV_DIR:?}/${n:?}"`, exit 0.
  - Part 1's close-out target, 2255072075 bytes (line 348), was removed
    the same way, exit 0.
  - Every other name of the three directories was kept.
- The crate's tests on both targets:
  - "the figures this run must give: off 539, ch 714, im 576, ci 756, cc
    759, cr 821, all 1229" (line 11067).
  - The set all passed 1229 on the host (line 11233) and on i686 (line
    11422): part 1 P2's 1203 with the 26 tests this phase adds, 15
    translator tests and 11 shift tests. Only all turns on rowcol.
- The new modules by name (lines 10620 to 10769): shift 44 passed,
  translate 15 passed.

## The run and the expected changes

The four samples, --insert-rows-sample, --delete-rows-sample,
--insert-columns-sample and --delete-columns-sample, ran over the 39
workbooks with every output opened. Each was compared per id with part 1
P2's records `records/m8p1-<name>-2026-11-09-8816db0`.

- Summaries (run.log lines 3010, 3076, 3141 and 3207):
  - insert-rows: same 39, changed 0;
  - delete-rows: same 38, changed 1;
  - insert-columns: same 39, changed 0;
  - delete-columns: same 38, changed 1;
  - new 0 and missing 0 in each.
- run.log line 3243: "the two pairs of the table are met: delete-rows
  90a5b01f9541 and delete-columns 91d2bdffa46a error/-/ok to ok/ok/ok
  (WG): yes". These are the two changes "Harness runs" P1 names, and
  nothing else changed.
- The run's tables are this phase's evidence, not records; records are
  P4's (change WE). The four pairs stay in <work>/records, each hashed:
  - m8p2-p1-insert-rows-2026-11-09-2740ce7.md 5b605179..., .txt 25d5644b...
  - m8p2-p1-delete-rows-2026-11-09-2740ce7.md 7012e6f5..., .txt ef3d5e7d...
  - m8p2-p1-insert-columns-2026-11-09-2740ce7.md a71f9414..., .txt d72a9af1...
  - m8p2-p1-delete-columns-2026-11-09-2740ce7.md b263b330..., .txt 0e08aca1...
  - No file is under records/.

## Every FINDING line

One FINDING, run.log line 3265:

"FINDING: the rowcol cost's growth over Milestone 7 part 2's size record,
18644 section bytes, is outside the band (expected 3000 to 12000,
accepted up to 16000; specs/0048 "Size"); recorded, not fixed"

The figures and their source (run.log lines 3250 to 3264):
- `size -A` of the size-profile i686 musl builds at the phase commit,
  under <target>/main.
- probe-emend-rowcol-write-xml: 1061616 section bytes, file 1067556,
  sha256 357e53fc925303f2cdbd3f0370dc27e485b1aecf502c82eab2e6923da8ca48cf.
- probe-emend-format-write-xml: 984728 section bytes, file 993828,
  sha256 16850afa3212b35c1c6eee30c1b9c628d9f51a57abf2b66744ccade18dd9d36e.
- The rowcol cost is 76888 = 1061616 - 984728.
- Milestone 7 part 2's record, records/size-m7p2-2026-10-28-43471ed.md,
  gives the rowcol probe 1042972 and the format probe 984728, a cost of
  58244. The growth is 18644.
- The format probe equals the record's figure, so the core did not move.
- Dry run d1 printed the same figures.

Planning's ruling on it, given before the logged run: the FINDING stands
as recorded, not fixed, and the band is not moved. P3's size record
attributes the growth by file.

The other lines holding the word FINDING are run.log's summaries and
counts, "FINDING lines so far 0" and "FINDING lines printed above: 1".
gate.log's two (lines 5834 and 5843) are inside the printed script's
carried history.

## Leftovers (left in place for the maintainer's deletion list)

- <phase>/dry/d1 with its build roots.
- <phase>/dev-target, where the session built.
- <phase>/tmp/G, R and S, the forks' temporary files.
- <phase>/tmp/tmp.X1KdtRYBQx, the run's working directory, which holds
  the run's tables.
- <phase>/target, this phase's target root. The next phase's gate prunes
  it.

## After independent check 1

The body above is kept as the check read it, as in part 1's P1 and P2.
Each point it raised, answered from the primaries:

- Line 18, `tmp.X1KdtRYBQx`:
  - No log prints the name; run.log's root line 79 says only "made by
    mktemp -d under TMPDIR, <scratch>/tmp".
  - The name was read from the disk afterwards: it is the one new name in
    <phase>/tmp, from logged.sh's before and after listings (<phase>/logged/
    phase-tmp-before.txt and -after.txt).
  - "as run.log's own root line says" applies to the mktemp form, not to
    the name.
- Line 19, `<scratch>`:
  - In gate.log (474), run.log (77) and the scan's log (76), <scratch> is
    <phase>. Read the logs by their own root lines.
  - Check 1's materials were under <phase>/check1, so the README's line
    and the logs' agree on the directory.
- Line 45, 14:19:59Z: this is logged.sh's "gate started" line
  (<phase>/logged/driver.out). gate.log 466 gives its own start,
  "run at 2026-11-09T14:20:01Z".
- Line 48, the outer standard errors: logged.sh's driver.out gives
  "outer stderr 0 bytes" for the gate and the run. The logs give each
  script's own standard error: gate.log 12080, and run.log near its end.
- Lines 49 to 50 and 116, dry run d1: its logs are under <phase>/dry/d1/,
  not in this commit. Its run.log line 3258 holds the same FINDING with
  the same figures (1061616, 984728, growth 18644).
- Line 54: the prune section runs to gate.log line 355, not 350.
- Lines 48 to 49, UNEXPECTED: no line reports an UNEXPECTED result.
  gate.log 9597 is a control naming the word, and 6981 is inside the
  printed script.
- Question 1 (a), coverage gaps:
  - The column-axis refusal test covers array formulas but not data
    tables.
  - The column-axis tests include no member with an attribute or text of
    its own.
  - Those cases run through the same code as the row axis. This is a test
    gap for planning's word; no code is changed in this record commit.
- Question 1 (b), a shared group's end pushed past the grid:
  - shift.rs's
    `a_ref_pushed_past_the_grid_is_a_grow_and_one_moved_past_it_off_the_grid`
    uses array formulas only, so no test covers the shared case. The
    session checked this with grep at the phase commit.
  - In the code, an insertion at or before the master moves the interval
    whole (`Moved::Whole`), so a shared group's end past the grid is
    refused with the cuts message, as for an array.
  - An insertion inside the group changes the interval (`Moved::Changed`),
    so the group is unshared. A cell then pushed past the grid is refused
    with "a row or column would be moved off the worksheet".
  - This is a test gap for planning's word.
- Question 1, beyond the spec's wording (not ruled on):
  - A row written with a leading zero (`A01`) is not read as a reference.
  - A member in a deleted row before its master still counts as early.
- Question 2, the per-id comparison: it reads outcome, lo and lo_in, not
  the exit code. That is fork R's reading, listed in the stop report's
  point 6; ruling A (6) accepts the forks' calls without naming this one.
  The run rows print the outcome. A question for planning's word.
- The draft report's contradictions:
  - "STOPPED after the record commit" against HEAD at the phase commit:
    the draft was written before the record commit, and the final report
    states the order.
  - "origin/main is e1fc6e4" is not in these logs; the final report gives
    it from git.
