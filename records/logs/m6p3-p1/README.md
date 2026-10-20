# M6 part 3 P1 logs

specs/0040-analytic-chart-elements.md, "P1: trendlines and error bars",
approved 2026-10-20 with decisions 1 to 4 A, and planning's P1 brief.
The doc commit 07cc871 committed the spec (sha256
f1919878c0f54cdd57460f9498aa9e0ec81584d2ad12d4d051dc29b752fce16b), specs/0039 (the M6 part 2 close-out verification) and
ROADMAP.md; the tree is clean at the phase commit, so no log takes
PLANNING_SPEC. The phase commit is
a22f6d7c0f0e27ef182b194dab4ba9a3e488e6d2, the tool commit. `<phase>` is
the phase's directory, `<home>/.cache/emend-build/m6p3-p1`.

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 441b5adfeb3457c673960afe03188524404d66083dbe784c7aa628204b2d2886 |
| test.log | test log checks all as expected: yes | 1f7d9a916d9f8e28f6c47c092071fd75db52f539bf39895a0dc0c210c8cb42e3 |
| negative.log | negative log checks all as expected: yes | 48b9b970dc9dc74497d8c071d44c8de4a69e31fdd212ca86bf6421699b589d01 |
| run.log | run log checks all as expected: yes | 485572f048b1272e50a93a4fb3627aa46dc0b1c86264f09284d74c60e1758d7f |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, run, and ended at 2026-10-20T18:59:04Z, 19:00:43Z, 19:00:52Z
and 21:08:17Z (each log's "run ended at" line), each with outer standard
error 0 bytes (the driver's count of the file each log's standard error
went to; session's statement). The gate pruned Milestone 6 part 2's four
target roots (m6p2-p1, m6p2-p2, m6p2-p3, m6p2-close), each listed with
du -sb and removed with rm -r. Each log is the first logged run of its
kind; no logged attempt was refused or stopped. One dry run (d1) ran
before them and ended yes for all four logs; no script was regenerated
after it.

The run log wrote records/m6p3-p1-charttrend-2026-10-20-a22f6d7.{md,txt}
and records/m6p3-open-2026-10-20-a22f6d7.{md,txt}.

## The session's statements (no log prints them)

1. **The generators were written by forked subagents** of this session,
   one per log, each from the template FACTS named; this session read
   each generator's output and ran the dry run. The forks' notes follow.
2. **The negative generator's development dry runs.** Two earlier
   versions of the negative script were dry-run by its fork and refused;
   both are kept in `<phase>/superseded/` (neg-gen1, neg-gen2), not
   committed. The first refused on a reader fault in its own check of
   the trend sample's plots (change ND47's line) and on a cargo process
   running at its entry; the second on a cargo process running at its
   entry alone (another fork's build). Neither is a logged run; the
   script that ran d1 and the logged run is the third.
3. **A scratch directory removed with rm -rf.** The negative fork
   removed its own scratch build directory under `<phase>/dev/` with
   `rm -rf`. The practice is to move such a directory aside; nothing
   outside the fork's own scratch was touched.
4. **The run log builds release binaries**, as part 2 P2's run template
   does; the session's facts file had named the debug binaries. The
   template's form was kept.
5. **The development keep's first run failed** because the directory
   named by EMEND_KEEP_DIR did not exist; it was made and the keep run
   again. It is not a logged run; the run log regenerates the keep
   itself (1896 pairs, 148 new, none of part 2 P3's record changed).
6. **NewSeries loses Eq** (it now holds `NewTrendline` and
   `NewErrorBars`, which hold f64); PartialEq is kept. `NewTrendline`,
   `NewErrorBars` and `ErrorValue` have no Eq, as the spec gives. P3's
   API listing will show NewSeries changed.
7. **The charts probe's figure** is 1103064 section bytes, 5828 over
   part 2's size record's 1097236 (gate.log), told to planning at any
   value and judged nowhere in this phase; P3's size log measures and
   judges the part's cost.
8. **The six backup branches** were deleted with `git branch -D` per the
   close-out verification's ruling U; git printed each name deleted:
   backup-p3-cfb83cc, p1-before-fix, p1-before-g, p3-before-reword,
   p4-before-cite and p4-before-reword.
