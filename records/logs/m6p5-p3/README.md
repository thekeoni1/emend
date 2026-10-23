# M6 part 5 P3 logs

specs/0042-gradient-fills-and-named-styles.md, "P3", and planning's P3
brief. The doc commit 693497c committed the spec with planning's P2
verification (sha256
7658019a1773a3fa9b169fedb2fc412112145a811e70bd20b74be2ac38be8566) and
ROADMAP.md (sha256
69c26b1b8bc78ae8e5915a7a012c6cbef1a94d3715d9c5cf33062324ede33aec).
Planning's rulings AN to BF, written during P3, are in the working
specs/0042, uncommitted; every log takes it at PLANNING_SPEC, sha256
7742fae1762057eb97e886cf2e986f2a3f15f51ddcea4c96f89518257ffe109b. The
phase commit is fe280ff7c76bb76c5e4a0c21a15df8e84320d3ea, the CHANGELOG
commit (ruling AZ). `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p5-p3`.

The phase's commits, each the next one's parent, on P2's record commit
d4d9eca:

| Role | Commit | Subject |
|---|---|---|
| doc | 693497c | M6 part 5 P3: commit planning docs (the P2 verification and rulings) |
| test | fd3c967 | M6 part 5 P3: a test of self-closing cellStyleXfs and cellStyles opened for a named style |
| probe | f4b249b | M6 part 5 P3: the size probes gain the gradient and the named style |
| changelog (phase) | fe280ff | M6 part 5 P3: the CHANGELOG names gradient fills and named styles |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 31d3a45351aa15212b8e3b80771c7bb201196f5602e079797bb9d561a7bbb9ed |
| test.log | test log checks all as expected: yes | bc8eefbcba4b59272ff6d0410a53f759f95986aa8c1f6af8874b06dd1f599496 |
| negative.log | negative log checks all as expected: yes | 1a5bd2caff838bcfa097d61e2796a36baa191028e1210120828326ba48c22222 |
| size.log | size log checks all as expected: yes | 4346db2e28e8302c29ed5026b4e36d91987e510463ca09a37d45758d7d52a113 |
| run.log | run log checks all as expected: yes | 526ea0e5cd36c9466d71424f62ffd0968dfb327bd9a9be7fe3ba6f847a0b0fd6 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, size, run, and ended at 2026-10-23T11:50:02Z, 11:52:22Z,
11:52:40Z, 11:58:15Z and 14:41:35Z (each log's "run ended at" line), each
with outer standard error 0 bytes (the driver's count of the file each
log's standard error went to; session's statement). Each log is the
first logged run of its kind; no logged attempt was refused or stopped
(session's statement).

The test log wrote records/api-review-m6p5-2026-10-23-fe280ff.md; the
size log wrote records/size-m6p5-2026-10-23-fe280ff.md; the run log
wrote records/m6p5-p3-namedstyle-2026-10-23-fe280ff.{md,txt} (P2's
corpus run rerun), records/m6p5-p3-gradient-2026-10-23-fe280ff.{md,txt}
(P1's corpus run rerun) and records/m6p5-open-2026-10-23-fe280ff.{md,txt}.

## Rulings in the working spec

- **AN** (P1): the size profile links what each probe's own source
  calls, so the same added lines go into all sixteen sources holding
  `fn full_style` (the session's list named sixteen and said fifteen).
- **AO** (P2): the four added lines `named_xfs`, `gradient_fill`,
  `named_fill`, `named_xf`; the gradient and named style literals; the
  moved lines printed against part 4's size log.
- **AP** (P3): one new last argument `<style name>`; the size log passes
  a constant.
- **AQ** (P4): the size fixture gains a one-entry `cellStyles`
  container, printed with the fixtures' byte counts before and after.
- **AR** (P5): the test commit's test, ungated, the first code commit.
- **AS** (P6): the API listing's extra rows read as the session gave
  them; added-by commits 6a7644e for P1's rows, d0e7b32 for P2's.
- **AT** (P7): the gate prunes P2's target root; the generators from
  P2's scripts and part 4 P3's size generator; the phase commit the
  CHANGELOG commit.
- **AU** (finding T1): a self-closing container without `count` opens
  without one; ruling AR's `count="1"` was the session's reading,
  wrong; no code changes; the test pins both forms.
- **AV**: `Style` derives `Default` (the session's "Style has no
  Default" was wrong); the four new lines after `full_border`; the
  probe commit accepted as committed.
- **AW** (T2): `fonts` stays 2; ruling AO's "fonts 2 to 3" was the
  session's reading, wrong, and is struck.
- **AX** (T3): the fixtures' byte counts 2496 to 2524, 2510 to 2538,
  2512 to 2540, 2781 to 2809 the expectation, the log the record.
- **AY** (T4): the listing 834 to 844 rows, 13 new, 3 lost; the three
  rows the diff dates otherwise carry both commits.
- **AZ** (T5): the generators as read; PHASE fe280ff7 (the session's
  first message typed it wrongly as fe280ffd and corrected it).
- **BA** (T6): the container goes after `cellXfs`, not after
  `cellStyleXfs`.
- **BB** (S1): each probe linking emend expected 0 up to the core's
  growth, accepted up to that plus 2000; the three linking no emend
  keep 0 and 2000.
- **BC** (S2): the `read` cost accepted up to 3000 either way; images,
  comments and charts expected 0, accepted 1500 either way.
- **BD** (S3): a budget above 458752 a printed line and a finding for
  decision 3, not a log failure; under it, the headroom printed.
- **BE** (S4): the rules probe's reader lists `cellStyles`, the
  reader's expectation; the size fork's `rm -r` under `fork-scratch`
  the session's recorded departure; the other forks' departures README
  matter.
- **BF**: the dry run, then on all "yes" the logged runs, the record
  commit, the scan and the report, then stop; the run's label series
  ends at NZZ.

## What the logs show

1. **gate.log:** the working spec at PLANNING_SPEC 7742fae1...; four
   roles by full subject; the doc commit's two files by blob. P2's
   target root `<home>/.cache/emend-build/m6p5-p2/target` listed at
   7740025302 bytes and removed (`rm -r`, no -f), its 29 kept names
   still there (ruling AT). Files in the diff 20; Rust sources 17
   (crates/ 1, tools/ 16). Distinct messages 155 and 155 by the one-line
   reader, 169 and 169 by the several-line reader. Test names 1450 at
   the base and 1451 at the phase commit. Nine PASS lines; clippy and
   doc clean on every set; the crate's tests on both targets off 538,
   ch 711, im 575, ci 753, cc 756, cr 818, all 1199, each "P2's
   committed figure plus the one test this phase adds". Probe sources
   changed 16, holding each new line once 16. The charts probe 1151328
   section bytes; the format probe 984664, 12488 over part 4's size
   record's 972176, its budget 451448 against the record's 438960.
2. **test.log:** ws-host and ws-i686 passed 1448, failed 0; the feature
   sets on both targets off 538, read 645, rules 575, read,rules 685,
   all 1199, failed 0, each P2's test.log plus 1. The named_style
   module's 16 tests (14 ungated, 1 rules, 1 read), ok lines 14, 15, 15,
   16, 16 per set and 16 in the workspace runs; the test commit's test
   passed once in every run. T1: the listing program is the one part 4
   P3's API record names, by hash; 844 rows, part 4 P3's 834 with 3 gone
   and 13 new; the three rows ruling AY names carry the heuristic's and
   the diff's commit; `Fill` loses `Copy` alone. The API record
   installed, sha256 04658d24..., 76382 bytes.
3. **negative.log:** "the 23 cases were refused, N1 to N23"; output
   files 15, B0 to B14; the twenty-three fixtures written. No new case
   (nothing new reachable through the binary).
4. **size.log:** 29 probes measured and compared with part 4's record,
   none new. This part's cost 12488, inside the estimate of 8000 to
   15000; the steps P1 2460, P2 6676 and the probe commit 3352 summing
   to it; the budget 451448, "the headroom left under the ceiling: 7304
   (specs/0042 ruling BD; changes RE and RK)". The costs' movements:
   images -484, comments 52, charts 488 (each inside 1500 either way),
   read 976 (inside 3000 either way). Every other probe linking emend
   inside its ruling BB band, five of them above the expected 12488
   and accepted up to 14488: fullstyles 12536, rowcol 12724, rules
   12844, richtext 12808, richcomment 12748. Findings 0. The moved lines
   3 of 3 as ruling AW gives; the fixtures' byte counts 4 of 4 as ruling
   AX; the container after `cellXfs` (ruling BA).
5. **run.log:** the size log's release binaries, both targets.
   --namedstyle-sample over the corpus: 27 of 27 rows ok on each label,
   lo and lo_in ok 27; read_back 81 of 81; custom_before 36 in 23
   workbooks; named_before 221; ruling W's count 0; every row's output
   and line byte-identical to P2's run by sha256, 27 of 27 on each
   label. --gradient-sample: 27 of 27 ok on each label, read_back 54 of
   54, byte-identical to P1's run, 27 of 27 on each label. The
   Excel-form fixture (id b9335e28dcc0, 2402 bytes): the three samples'
   outputs and lines on both labels byte-identical to P2's, 6 of 6. The
   openpyxl fixture (id 3137dc555cde, 27230 bytes): the six lines and
   the ten field lines P2's, byte for byte; P2's own openpyxl fixture,
   6 of 6. The keep: passed 1448, failed 0; pairs 2237, 2 new (exactly
   this phase's test's), 0 gone, 0 whose bytes differ; the open
   record's totals lo_in ok 2225, lo ok 2226; no output refused on an
   accepted input; no verdict of the named record's pairs moved.

## The session's statements (no log prints them)

1. **The generators were written by forked subagents** of this session,
   one per log, from P2's committed scripts as templates (each checked
   byte-equal to the script printed in P2's committed log) and part 4
   P3's size script. This session wrote the facts file, read each
   fork's notes and ran the dry run. Labels: gate A84 to Z84, test YI to
   YQ, negative NA57 to NF57, size RA to RJ, run NZW to NZZ, scan KT to
   KV. The run's NZ series ends at NZZ (ruling BF).
2. **The session's errors**, each ruled by planning: T1 (ruling AU, the
   test's expected `count="1"` on opened containers that had none); the
   probe count given as fifteen for sixteen names (ruling AN); "Style
   has no Default" (ruling AV); T2 (ruling AW, "fonts 2 to 3"); ruling
   AQ's placement as the session's facts gave it, corrected to after
   `cellXfs` (ruling BA); and PHASE first typed fe280ffd (ruling AZ).
3. **Departures the forks named.** The gate's B84 carries part 4 P3's
   B81 form (nothing under crates/ changed since bf07f86 but the test
   file). The test fork did not carry specs/0041 ruling L's two-program
   comparison (a one-time change of program in part 4); its T1 checks
   the program's hash against part 4's record instead; it copied part
   4 P3's listing.tsv and files.tsv into `<phase>/model/m6p4-p3/api/`,
   read only at the source; the script reads part 4's own file at run
   time, pinned by its record's hash. The scan fork dropped the token
   `scripts/openpyxl-ops` from its leftover sweep, because its refusing
   controls name those paths on purpose. The size fork cut part 4's
   charts-only blocks (QF/QW, the gated reader, ruling K's steps, M5
   part 6's close). The gate's and dry driver's own text say the
   working spec holds "rulings AN to BA" (generated at ruling BA); the
   spec they pin holds AN to BF. The gate's line "the charts probe's
   sections: 1151328; part 4's: 1055368" carries its "part 4's" from the
   template: 1055368 is a constant carried since M5 part 5 P4's
   committed gate.log, which prints it as its own "part 4's", and P2's
   committed gate.log prints the same constant in that line; M6 part 4's
   size record's charts figure, 1138352, is the gate's "part 4's size
   record's figures" line.
4. **Regenerations.** Size: gen1 (af0265de...) judged by the fork's
   readings S1 to S3; gen2 (98e325a1...) by rulings BB, BC and BD; gen1
   and its generator moved to
   `<phase>/fork-scratch/size/superseded-gen1/`. Every other script is
   one generation. No log ran a superseded script.
5. **The dry run**, at fe280ff, PLANNING_SPEC 7742fae1...:
   `<phase>/dry/d1`, all five "yes", outer stderr 0 bytes each, ended
   2026-10-23T08:45:40Z, 08:49:00Z, 08:49:18Z, 09:01:28Z and 11:42:04Z;
   its gate ran with PRUNE_PREVIOUS=list, so the logged gate found P2's
   target root still there. Before the logged run the session checked
   the working spec still hashed to PLANNING_SPEC, no ANTHROPIC_API_KEY
   in the environment, P2's directory held the gate's 30 names, and
   none of `<phase>/logs`, `gate`, `test`, `keep-host` or `api` was
   present.
6. **The rm uses.** No fork removed a file of the repository or of
   another phase. The prune (`rm -r` of P2's `target` alone, no -f) is
   printed in gate.log. The size fork ran one `rm -r "${F:?}/w"` of its
   own scratch output under `<phase>/fork-scratch/size/blk`, a departure
   from the facts file's move-aside rule (ruling BE). The other removals
   are carried from their templates and remove only what the same
   script made: the gate's doc build directory, its mktemp traps, the
   counting library's control directory, the negative's `rm -r
   "${B:?}"` and the scan's `rm -r "${T:?}"` trap.
7. **The development figures**, at fe280ff (not a log): workspace 1448;
   the crate off 538, read 645, rules 575, read,rules 685, every feature
   1199, i686 every feature 1199; the dev keep 2237 pairs, 2 new; the
   budget 451448, read +976, images -484, comments +52, charts +488.

## After the independent check

The check (independent-check-1.md, sha256
47099573787428fd4fa6d4eb32f93d464196f175ddd402235f898388bda125dd, its
answer verbatim) read this README as it stood before this section
(sha256 8a57e3d3be24fb91514a8f0afa40278aa7a58df87f800d7c4ed86a05eda1a7f5);
nothing above it changed after the check. Its prompt is the brief file
but for that file's final newline; the draft inline in it is the frozen
draft byte for byte (session's statement). Its points:

1. **S1 to S4 described too narrowly** (its 3 a): the draft's line
   named the bands, the read cost and the ceiling, which are S1 to S3;
   S4 (ruling BE) is the rules probe's reader listing `cellStyles` and
   the forks' departures. The final report names S4.
2. **gen2 before the dry run** (3 b): no primary stated the order. The
   size script the dry run's size log prints and the one the logged
   size log prints are the same text (the "## script" block, its
   four-space indent removed, sha256 050d90e9... for both), and the dry
   size log judges by ruling BB, which gen2 alone carries; the logged
   run ran gen2, 98e325a1... (session's statement).
3. **"the one derive change"** (3 c): true of the types that already
   existed; the listing also shows the three new types' derive lines,
   Gradient, GradientKind and Stop. The final report says so.
4. **The gate's "rulings AN to BA"** (2, a note): as item 3 of the
   session's statements above says.
5. **Nothing pushed** (3 d): the session's statement; origin/main is
   f630f67, which does not contain fe280ff.
