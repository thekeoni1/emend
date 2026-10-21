# M6 part 3 P2 logs

specs/0040-analytic-chart-elements.md, "P2: plot lines, axis options,
the openpyxl fixture, the catalogue", and planning's P2 brief. The doc
commit d8479f5 committed the spec (sha256
f9d630765363ed68c97eb6dc5b618f1967c8071c2d4a9062df8bea29e2aefadc, rulings
A to I) and ROADMAP.md (sha256
33c2b578f5818ea0f719628af9f84d179d6f1ff71a4b0cfcf6bbfa6d2c479b3d).
Planning's rulings J to P, written during P2, are in the working
specs/0040, uncommitted (P3's doc commit carries them); every log takes
it at PLANNING_SPEC, sha256
b36f61656e592014a7fc0f65881bc0485ed4cc7ca85221a4225d7b1e7bef51e8. The
phase commit is 3736eea1792865f5b4b454b7afe6ba7f90d5a7fa, the second
tool commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p3-p2`.

The phase's commits, each the next one's parent, on the P1 record
74b63de:

| Role | Commit | Subject |
|---|---|---|
| doc | d8479f5 | M6 part 3 P2: commit planning docs (the P1 verification) |
| core | 3c1f1f3 | M6 part 3 P2: charts write, plot lines and axis options |
| tool | d367326 | M6 part 3 P2: emend-copy --chart-axis-sample |
| op | d1192d2 | M6 part 3 P2: the openpyxl chart-analytic op and its test line |
| op date column | 87db736 | M6 part 3 P2: the chart-analytic op's date column (finding F1) |
| tool, no categories (phase) | 3736eea | M6 part 3 P2: the axis sample's date-axis bar takes no categories (finding F2) |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | dae75dd9ff80cf574e03e11245bb73c9e7ed43f06c507686aac55993b266e742 |
| test.log | test log checks all as expected: yes | 195f2b029f53909ec34cc749c9b8185459a28138e576b651fc0bcdd82736d7b6 |
| negative.log | negative log checks all as expected: yes | d0c3a57b59029b870cfd0d184ca81928862b8b5991b68da9a8861db332813772 |
| run.log | run log checks all as expected: yes | 51253a45a0cd96a01e5ab9c442d3cf513f39c7a1999526485c9a39f5d8278f11 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, run, and ended at 2026-10-21T04:13:24Z, 04:15:21Z, 04:15:33Z
and 06:46:08Z (each log's "run ended at" line), each
with outer standard error 0 bytes (the driver's count of the file each
log's standard error went to; session's statement). The gate pruned
nothing (ruling I). Each log is the first logged run of its kind; no
logged attempt was refused or stopped (session's statement). Two dry runs ran before them:
dry run 1 at d1192d2 was refused by its run log (finding F1), and dry run
2 at 3736eea ended yes for all four logs.

The run log wrote records/m6p3-p2-chartaxis-2026-10-21-3736eea.{md,txt},
records/m6p3-p2-charttrend-2026-10-21-3736eea.{md,txt} (P1's trend sample
rerun) and records/m6p3-open-2026-10-21-3736eea.{md,txt}. The UTC date is
the run's (the logs ran on 2026-10-21 UTC; the phase's commits were
made on 2026-10-20, session's statement).

## The findings (LibreOffice facts, not emend's)

The spans, the column A figures and the two causes are in the spec's
rulings O and P. The bisection's variants, the five-cell sample and the
measurement before 3736eea are the session's statements; no log prints
them.

1. **F1, dry run 1's run log.** Gate, test and negative ended yes; the
   run refused its six openpyxl fixture rows (the charts, axis and trend
   samples on host and i686), each reading lo=error lo_in=error. The
   pinned LibreOffice (localhost/emend-lo:25.2.3-2-deb13u6) aborts with
   std::bad_alloc, exit 134, on the chart-analytic op's output itself,
   before emend touches it. Bisected by hand over openpyxl variants
   through scripts/lo-check.sh (session's scratch, not committed): the
   DateAxis chart alone aborts it, with or without its units and its
   logBase; the first chart (trendline, error bars, drop lines, bounds,
   skip) and logBase alone open. A date axis whose categories span about
   13M days or more aborts it; spans of 1M and 100k days open, and so
   does a 100-day span past 9999-12-31. The fixture's column A, the
   DateAxis chart's categories at d1192d2, holds 99 numeric cells in
   A2:A100 from 116233 to 42886443, a span of 42770210 days (planning's
   measurement). The session's first report to planning gave "about
   25.7M to 38.2M, about 13M days"; that figure was read from five cells
   (the first three and the last two of A1:A100), not a minimum and
   maximum, and is wrong. Ruling O: the op writes its own
   date column (87db736). Dry run 1 is kept in `<phase>/superseded/d1-F1`.
2. **F2, found while measuring ruling O's fixture.** With the input now
   opening, the pinned LibreOffice aborted the same way (exit 134; lo_in
   ok, lo error; both targets) on --chart-axis-sample's output: the
   sample's date-axis column chart took its categories from
   $A$2:$A$102, the same column A span. Ruling P (option a): that chart's
   series takes no categories (3736eea). Measured by hand before the
   commit, in a scratch worktree at 87db736 with that change alone (host
   release build), through scripts/passthrough-run.sh, which prints lo ok
   only when scripts/lo-check.sh's lo_check returns 0 (lo-check rc 0):
   the sample's output on the ruling O fixture
   read outcome ok, lo ok, lo_in ok, its line chartaxis charts_before=2
   charts_after=5 series=3 rows=101 date_axes=1 bounded_axes=2
   read_back=3 chart_parts_after=5, and its date-axis chart part held no
   cat element.

The negative log's cases, counts and messages are unchanged by both
rulings; ruling N's comparison is unaffected (12 of 12 fields).

## The session's statements (no log prints them)

1. **The generators were written by forked subagents** of this session,
   one per log, from the templates the facts file named, and updated by
   forks at each ruling (gen2 for O, gen3 for P); this session read each
   update's notes and ran the dry runs. Labels at gen3: gate A77, test WO,
   negative NA50, run NYO, scan JY. Each earlier generation's generator
   and script is kept under `<phase>/superseded/gen1/` and `gen2/`, not
   committed.
2. **The forks' rm uses (ruling G).** The forks changed every carried
   `rm -rf` to `rm -r` (gen1), a departure from the carried form. Beyond
   that, the forks removed files of their own making with plain `rm`
   (no -f): the run fork one empty file it made in dev/run-try/ (gen1);
   the gate fork two intermediate regenerations, try1 and try2 (gen2);
   the test fork one trial script (gen2); and, while moving copies into
   superseded/ (gen3), the gate fork and the negative fork one
   duplicate each, each first found byte-equal by cmp. Directories
   emptied by those moves were removed with rmdir. No fork used rm on
   anything it had not made.
3. **The forks' other notes.** The negative fork fixed the carried
   format and trend readers for the core's `high_low_lines: plot ==
   NewPlot::Stock` line (ND48). The gate's consumer crate gains the new
   types and fields (K75), needed to compile. The run's place rule
   covers all twelve fields, and its charts-line check on the openpyxl
   fixture also requires axes 4, axis_cat 1 and axis_val 2. The scan
   requires the sibling logs to print the committed pin f9d63076. At
   gen3 the negative fork ran the main.rs readers (ND43 to ND48) on
   main.rs at 87db736 and at 3736eea: their outputs are byte-identical,
   and so are their outputs on each of the 58 planted copies. The run
   fork measured that nothing the run pins changes with 3736eea (the
   field comparison's output byte-identical).
4. **The fixture's rows are 101** for the axis and trend samples (100
   at d1192d2): the op's date column ends at row M+1. The charts
   sample's digest_formulas changes with the new column; the run does
   not pin it.
5. **The tool commit was rebuilt** (development check.sh, not a log):
   the first tool commit 9a6a220 failed clippy type_complexity in
   null_rewrite.rs (the axis forms test's read type); the branch was
   soft reset to the core commit, a `type Axes` alias added, and the tool
   and op commits made again with the same messages (d367326, d1192d2).
   The range-diff is the alias alone. Planning accepted it (specs/0001).
6. **The no-categories commit's first assert was wrong.** Its first form
   required `<c:cat>` in every chart part but the column's and failed on
   the scatter, which writes xVal; it was corrected, before the commit,
   to cat or xVal. The test's assumption was wrong, not the code.
7. **The run log builds release binaries**, as the template does. The
   session's hand measurements for F1, F2 and ruling P used the release
   binaries dry run 1 built (`<phase>/superseded/d1-F1/build/bin`),
   except ruling P's emend-copy, built in its scratch worktree.
8. **min and max must be finite** (CHART_VALUE_RANGE): the spec names no
   rule for them, and a NaN or infinity would write an invalid number;
   a session choice.
9. **The axis sample's bounded_axes is 2** (ruling M): the session
   raised the spec's "(3)" as an arithmetic error, and planning ruled
   that the description governs.

## After the independent check

The check (independent-check-1.md) read this README before these
changes: the statements its Question 3 named as printed by no log are
marked the session's; "Ruling O (option a)" reads "Ruling O"; the
measurement before 3736eea names lo-check's rc 0 (its Question 2). Its
other points are raised to planning in the report, not fixed: the
axis-sample module doc in tools/emend-copy/src/main.rs still describes
the column chart's series against A2:A(L) (fixing it is a code commit
and the logs rerun); the finite min and max rule (item 8) awaits a
ruling; test.log's "of them this phase's logs: 1" labels the working
spec's status line.
