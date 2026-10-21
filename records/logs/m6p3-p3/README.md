# M6 part 3 P3 logs

specs/0040-analytic-chart-elements.md, "P3", "The probe" and "Size and
packaging", and planning's P3 brief. The doc commit d5399c6 committed the
spec (sha256
a02a208982fd79027a27988a409b1ad85e1b457c66b4f662cc5f3964a8b6af8e, rulings
A to S) and ROADMAP.md (sha256
6db2432692439384f45d89dabc4e0e8280c874fc61fac5d4550db58d8ea82ece).
Planning's rulings T and U, written during P3, are in the working
specs/0040, uncommitted; every log takes it at PLANNING_SPEC, sha256
9f5bb018e0d29ff76104807dbdff148bcd2a7221580107eb3ccfad37a0e5b885. The
phase commit is 07072364296777f164afd7673f25e46b065b6f9f, the ruling T
probe commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p3-p3`.

The phase's commits, each the next one's parent, on the P2 record
65703ee:

| Role | Commit | Subject |
|---|---|---|
| doc | d5399c6 | M6 part 3 P3: commit planning docs (the P2 verification and rulings) |
| carry | 0765fee | M6 part 3 P3: the axis sample's module doc names the column's series without categories (ruling Q) |
| probe | 8100ae3 | M6 part 3 P3: probe-emend-charts-write-xml gains the trendlines, error bars, plot lines and axis options |
| changelog | 63c9af9 | M6 part 3 P3: the CHANGELOG names the analytic chart elements |
| 3D line (phase) | 0707236 | M6 part 3 P3: the 3D line's category axis takes the skips (ruling T) |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 92b43ad4ca51630e2b834e4017fb1c9972b55f92c902f0eafa68b5315ffcf7a8 |
| test.log | test log checks all as expected: yes | 8afe97d544f88861648d471d830827081e0ccd6692801a718a77730dde723a50 |
| negative.log | negative log checks all as expected: yes | 833a4b8d765b221df0ae223032128c890f6972b216a7bb7af50e304c0f32e6a9 |
| size.log | size log checks all as expected: yes | ce2f829ecd3faa65203a1b1c64b0d0d7f0b15937bd3bf81908adc6c22f0a47da |
| run.log | run log checks all as expected: yes | bf8b41ca1418ec21f1e43d6aa878f832ea7658b0277ea1fefa61a953f0a79491 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, size, run, and ended at 2026-10-21T10:43:08Z, 10:46:00Z,
10:46:11Z, 10:51:26Z and 13:26:52Z (each log's "run ended at" line), each
with outer standard error 0 bytes (the driver's count of the file each
log's standard error went to; session's statement). The gate pruned
nothing (ruling I). The run log takes the size log's release binaries.
Each log is the first logged run of its kind; no logged attempt was
refused or stopped (session's statement). One dry run ran before them,
at 0707236, into `<phase>/dry/d1`, and ended yes for all five logs.

The size log wrote records/size-m6p3-2026-10-21-0707236.md; the test log
wrote records/api-review-m6p3-2026-10-21-0707236.md; the run log wrote
records/m6p3-p3-chartaxis-2026-10-21-0707236.{md,txt},
records/m6p3-p3-charttrend-2026-10-21-0707236.{md,txt} (P2's two corpus
samples rerun) and records/m6p3-open-2026-10-21-0707236.{md,txt}.

## What the logs show

1. **Size** (size.log, the size record): the charts probe's growth over
   M6 part 2's record is 10724 section bytes (1107960 over 1097236), the
   feature `charts` costing 135784 against part 2's 125060. Below the
   estimate of 16500 to 26000 and under the finding line of 31000: told,
   not a finding (ruling U); the record names the arms the probe reaches
   and records the 5776 below the estimate's low end as the estimate's.
   The `images` and `comments` costs moved 0 each, and the core's growth
   is 0 (ruling N's form). Every other probe grew 0. The gate prints the
   same figure, 884 over P2's committed gate.log's 1107076, judged
   nowhere there.
2. **The API listing** (test.log, the API record): 815 items against M6
   part 2 P3's 776, 40 new and 1 gone (the `charts` re-export, whose key
   grows by the part's nine types); every verdict keep; one row, `NewSeries`,
   marked as having lost `Eq` (P1's core commit 12f1314, its `NewTrendline`
   and `NewErrorBars` holding `f64`). No new message, no new `NewPlot`
   variant.
3. **The negative log**: cases, counts and messages unchanged; P3 adds no
   message and no binary path.
4. **The run** (run.log): --chart-axis-sample and --chart-trend-sample
   over the corpus, with the size log's release binaries, byte-identical
   by sha256 to P2's runs, 27 of 27 rows on each label, 4 of 4; the
   openpyxl fixture's lines and fields as P2's, and P2's kept fixture
   rerun through the three samples byte-identical; the keep 1386 tests
   passed, 2121 pairs, 0 new, 0 gone, 0 changed against
   records/m6p3-open-2026-10-21-3736eea.txt.

## The session's statements (no log prints them)

1. **The generators were written by forked subagents** of this session,
   one per log, from the templates the facts file named (gate, test,
   negative, run and scan from P2's gen3 scripts, size from M6 part 2
   P3's, the API block and its formatter from M6 part 2 P3's test
   script); this session read each fork's notes and ran the dry run.
   Labels: gate A78 to Z78, test WP to WY, negative NA51, ND51, NF51,
   size QJ to QR, run NYP to NYT, scan JZ, KA, KB. One generation each.
2. **The facts file's error.** The session's facts file named P1's core
   commit c566a01, a commit of M5 part 3; P1's core commit is 12f1314.
   The size fork found it before any script used it; the facts file was
   corrected and the test fork told. The API record cites 12f1314.
3. **Ruling T's commit.** Planning's ruling T (3) came after the probe
   commit: the 3D line's category axis takes the skips, made as its own
   commit, 0707236, the phase commit. The i686 charts probe measured
   1107960 section bytes at 8100ae3 and at 0707236 alike (development
   builds, not a log).
4. **The rm uses (ruling G).** The size fork changed its template's two
   carried `rm -rf` (directories the script itself makes with mktemp -d)
   to `rm -r "${var:?}"`. No fork removed any file; two first outputs
   were moved aside, not removed: the negative fork's (its first form
   admitted 15 status lines, not 11: 13 arguments to a 12-place format)
   into `<phase>/superseded/neg-gen-try1/`, and the size fork's into
   `<phase>/superseded/size-gen-dev/`.
5. **The size record's wording.** Its carried check "no unfilled figure"
   counts the word "error", so the record's new text names the error bars
   by their element names (`errBars`, `errValType`, `errBarType`).
6. **The forks' other notes.** The gate cuts P2's check of a fixed
   ten-line probe diff: it prints the probe's whole diff over the phase
   (147 lines added, 35 removed) and checks the probe by tokens. One gate
   label reads "the 3D-line commit's files", hyphenated, as the loop's
   label cannot hold a space. The test log drops block O (no scripts/
   change in P3) and restores the API block. Its static run is
   `<phase>/gen/work-test/`, inside the phase directory the gate lists.
   Its status label counts the phase's own paths (ruling S). Five rows of
   the API record carry 4c36f05 as their added-by commit by the listing's
   heuristic (an equal declaration line earlier in the file), not because
   M5 part 4 added them. The run's "some test added or edited" check
   becomes "no test added or edited", with a control over P2's range that
   finds P2's date-axis test. The negative fork ran the main.rs readers
   (ND43 to ND48) on main.rs at 3736eea and at 0707236: all 68 outputs
   byte-identical.

## After the independent check

The check (independent-check-1.md) read this README as it stood before
this section; nothing above it changed after the check. Its points on
the report are taken into the report. Its note that gate.log names
c566a01 as "part 3 P1's crate commit" without a milestone: that commit is
M5 part 3 P1's ("charts read"), as statement 2 above says. Its note on
the fixture's bytes: `specs/0038` ruling P's reading, that a fresh
openpyxl file cannot match by sha256, is the run log's form.
