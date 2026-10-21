# M6 part 4 P1 logs

specs/0041-chart-text-and-edits.md, "P1: text properties", and planning's
P1 brief. The doc commit 245831d committed the spec (sha256
ffe2db9ef1748be8ddcddc2996c7b1e5818e4d2cb1ab6d64fb64bad07ed8a5d9), the
M6 part 3 close-out verification in specs/0040 and ROADMAP.md (sha256
8e9dd34215ffc1c6ea94abfdce20e4fac26789991e09ec6c46c7a478dc8396fe).
Planning's rulings A and B, written during P1, are in the working
specs/0041, uncommitted; every log takes it at PLANNING_SPEC, sha256
8a0dc59ae497d7a65b598add15c8d25d36c4903d0aeecdd30209b043bfdb1f65. The
phase commit is f838d656b095676e59dc4c301977e9b70203b19e, the tool
commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p4-p1`.

The phase's commits, each the next one's parent, on M6 part 3's
close-out record 5d1c302:

| Role | Commit | Subject |
|---|---|---|
| doc | 245831d | M6 part 4 P1: commit planning docs (the spec, the M6 part 3 close-out verification) |
| core | 4d2cdc3 | M6 part 4 P1: charts write, text properties |
| tool (phase) | f838d65 | M6 part 4 P1: emend-copy --chart-text-sample |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | be22c6840a9061403475fccd2f465def8bbcba4e9d8741bed64abe3c2cee2b28 |
| test.log | test log checks all as expected: yes | 6ddf72ed93b06435d506271c3063b5435bcd3c62ea0082aebb8a4681adf537dc |
| negative.log | negative log checks all as expected: yes | 0e0f0940bc48aca96005bb885e7b39d73c751f100298e7e27f4f73a41b7e3b80 |
| run.log | run log checks all as expected: yes | 48522aa8bc39705d3bb9c3c8e7ae234d0565c65d46bb414fe8c9dc0a39496db0 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, run, and ended at 2026-10-21T17:59:55Z, 18:01:37Z, 18:01:50Z
and 20:31:53Z (each log's "run ended at" line), each with outer standard
error 0 bytes (the driver's count of the file each log's standard error
went to; session's statement). Each log is the first logged run of its
kind; no logged attempt was refused or stopped (session's statement).
One dry run ran before them, at f838d65, into `<phase>/dry/d1` (the
gate's prune in list mode, the run's records into the dry directory),
and ended yes for all four logs.

The run log wrote records/m6p4-p1-charttext-2026-10-21-f838d65.{md,txt}
and records/m6p4-open-2026-10-21-f838d65.{md,txt}. P1 writes no API
record and no size record.

## Rulings in the working spec

- **Ruling A** (the check order and the forms): the four chart texts are
  checked after the axis options and before the combined plot's rules,
  applicability first, then size, colour, font; the labels' text in the
  series options after the error bars; a colour above 0xFFFFFF takes the
  option-kind message; the font is checked by the 31-byte valid() rule
  and escaped as an attribute; a NewText given writes both pPr/defRPr
  and the run's rPr, the default both empty with no lang; the axis
  labels' text goes on the primary category and value axes only.
- **Ruling B** (the Excel-form fixture): --charts-sample on it reads
  charts 1, series 1, styles 1, the style read through the
  mc:AlternateContent's first Choice; the run log's own check confirms
  c14:style 102 and the fallback c:style 2 before and after the text
  sample, and the kept chart part and both style parts byte for byte.

## What the logs show

1. **gate.log:** three commits, one planning-doc and two code; messages
   154 distinct at the base and at the phase commit; test names 1389 at
   the base and 1399 at the phase commit, the ten new tests and the two
   base tests edited by their one added `text: None,` line alone; the
   core commit's change to the tool's main.rs its 46 added field lines
   alone, and the probe's its nine; check.sh's nine PASS lines; the
   figures the test log must give printed (off 515, charts 673,
   charts,comments 718, charts,read 776, every feature 1155). The i686
   charts probe: 1112212 section bytes, 4252 over part 3's size record's
   1107960, printed and judged nowhere there. M6 part 3's four target
   roots (m6p3-p1, m6p3-p2, m6p3-p3, m6p3-close) listed with du -sb,
   5902174134, 6085602975, 8018992563 and 2080261526 bytes, and pruned,
   their other names kept.
2. **test.log:** ws-host and ws-i686 passed 1396, failed 0 (P3's 1386
   plus 10); the feature runs on both targets 515 (off), 673 (charts),
   715 (charts,images), 718 (charts,comments), 776 (charts,read) and
   1155 (every feature), failed 0, each charts set P3's plus 7.
3. **negative.log:** 14 cases refused, N1 to N14, and the accepted runs
   B0 to B9 (ten outputs); N14 (DRAWING_UNREADABLE, no output) and B9
   (the text sample's line) are this phase's; the text rules shown
   unreachable from the binary by the readers.
4. **run.log:** --chart-text-sample over the corpus on host and i686,
   with release binaries built at the phase commit: 27 of 27 rows ok on
   each label, lo and lo_in ok 27, nothing dropped or recompressed; every
   line three charts more, series 4, texts 6, styles 0, read_back 3, rows
   1 to 1000 (the two large workbooks at 1000); outputs byte-identical
   across the labels, 27 of 27. The Excel-form fixture (id c71ca2a4a1a9,
   4909 bytes) through --charts-sample and --chart-text-sample on both
   targets: the charts line reads sheets 1, charts 1, series 1, labels 1,
   titles 1, axes 2, legends 1, styles 1 (ruling B), its twelve entries
   identical; the text line charts_before=1 charts_after=4 series=4
   rows=6 texts=6 styles=0 read_back=3, the fixture's chart, rels, style
   and worksheet parts identical; the forms' nine checks before and ten
   after hold. The keep: passed 1396, failed 0; pairs 2173, of them 2121
   the record's (records/m6p3-open-2026-10-21-0707236.txt) unchanged and
   52 new, exactly the 52 this phase's tests write; 0 gone, 0 changed.

## The session's statements (no log prints them)

1. **The generators were written by forked subagents** of this session,
   one per log, from the templates the facts file named: gate and run
   from part 3 P1's scripts (the gate's consumer crate from part 3 P3's
   gate, as part 3 P1's NewChart literal lacks P2's fields and would not
   compile at f838d65); test, negative and scan from part 3 P3's. Each
   generator checks its template byte-equal to the script printed in the
   committed log. This session read each fork's notes and ran the dry
   run. Labels: gate A79 to Z79, test WZ and XA to XE, negative NA52 to
   NF52, run NYU to NYX, scan KC, KD, KE. One generation each, but for
   the run's (statement 4).
2. **The facts file's error.** The session's facts file said the corpus
   run uses debug binaries; the run template builds release binaries at
   the phase commit (its change NVM), and the run fork kept the
   template's form. Planning was told. No script used the wrong text.
3. **The gate fork's hand harness** ran seven real `cargo doc -p emend
   --no-deps` builds and one consumer `cargo check` (three feature sets)
   in the session's scratchpad, its own target directories, though its
   brief said no cargo build; none touched `<phase>/target` or the
   repository.
4. **The run fork's first generation** (sha256 9b4bfc7b...) failed one of
   its own controls in the fork's hand run: "text_line_ok refuses the
   keys out of the tool's order" moved read_back, already the last key,
   to the end, so the order did not change. The control now moves texts
   last; regenerated. The first script was moved, not removed, to
   `<phase>/fork-scratch/run/superseded-gen1/`. No log ran it.
5. **The rm uses.** No fork removed any file or directory. The prune
   (`rm -r` of each part 3 root's `target` alone, no -f) is printed in
   gate.log. The other removals in the scripts are carried from their
   templates and remove only directories the same script made: the
   gate's `rm -rf "${W:?}" "${CONSUMER:?}"` trap and `rm -rf
   "${SHAPE_DIR:?}"` (its generator checks them equal to the
   template's), its `rm -r "${DOC_DIR:?}"` of the doc build's output, the
   negative's `rm -r "${B:?}"` and the scan's `rm -r "${T:?}"` trap.
6. **The development figures**, at f838d65 (not a log): the workspace
   1396 passed; the dev keep 2173 pairs, 2121 the record's unchanged and
   52 new, all from the ten new tests; test files chart_write.rs 97 to
   104, chartsheet.rs 20, null_rewrite.rs 150 to 153.
7. **The forks' other notes.** The negative log adds reach6.py, a reader
   of the text sample's literals, with 16 planted copies, each refused;
   reach5's "every other chart literal" count changes by the text
   sample's literals alone. The field-rules crate test is checked by
   body_holds on its error(RANGE) and error(OPTION) comparisons. The run
   fixture has a rels part and two style parts, so the charts sample's
   entries are 12 (part 3 P1's fixture had 9); the text sample prints no
   chart_parts_after key. The scan's doc commit holds three files and
   pins specs/0040 at the doc commit (SPEC40_PIN bb7ab719...).

## After the independent check

The check (independent-check-1.md) read this README as it stood before
this section; nothing above it changed after the check. Its points:

1. **The doc comment's citation** (its point 1 a): `NewChart::title_text`'s
   doc comment cites specs/0041 "The call", which has no such section;
   the fields are in "Types". A slip in the core commit 4d2cdc3, told to
   planning; not changed in this phase.
2. **The add_chart Errors prose** (1 b): it names the labels' text after
   the chart texts, "checked the same way after its error bars"; the code
   checks each series' labels' text in the series options, before the
   chart texts (ruling A). The sentence says where, but its place in the
   paragraph reads as an order. Told to planning; not changed.
3. **An axis title on a round kind** (1 c): refused before any text
   check, by `plot_applies` (an axis title or format on a pie, 3D pie,
   doughnut or of-pie, the option-kind message), which runs before the
   series options; so the case the check names takes the option-kind
   message (session's reading of chart_write.rs at f838d65).
4. **txcheck's parser** (2): txcheck.py imports `xml.etree.ElementTree`
   (printed in run.log).
5. **The gate's figures in this README** (3 b): item 1 of "What the logs
   show" leaves out two of the seven printed, images 552 and
   charts,images 715; gate.log prints all seven.
6. **The check's own departure:** it wrote one file, the committed spec's
   text taken from the diff to hash it, into the session's scratchpad,
   outside the repository and the check directory.

## Finding S1 and ruling C (the final scan's first run)

The final scan's first run ended "RESULT: FAIL", total hits 1, structural
checks failed 0: form f_p6, once, in independent-check-1.md, whose line 77
(the check's note on its own departure) quoted the absolute path of the
file it wrote in the session's scratchpad. The attempt is kept in
`<phase>/superseded/scan-final-try1/` (final.log sha256
41b5074012ce440242f7f288a905e7078649cd0489c63256f981ac1f2fae6c81).
Planning's ruling C, in the working specs/0041 (sha256
48cf2dc6aa50c1245bd7e8500bbf93c4b283afdda5cbf1a7a7b5671f5610d8c8): the
committed answer is the check's verbatim answer with that one path
replaced by `<scratch>/spec41.md` (change S1-a), a scrub altering no
judgment. The original, outside the tree, has sha256
f067043e1b2c3ba168a0e352dfbef1b815a66fde064152d8d4c16b9bac547b28; the
committed file has sha256
dd38d6ada4d5e8c5410a4068a8da899f1bfd06e880caba24dbe56d6ff54b2910 and
differs from it on line 77 alone. scan.log is the named rerun. Points 1 a
and 1 b of the check are carried to P2's first code commit (ruling C).

## Finding S2 and ruling D (the final scan's second run)

The named rerun ended "RESULT: FAIL", total hits 0, structural checks
failed 1: "every file RERUN_FILES names lies in <phase>/superseded
(change AP): no". The session had given RERUN_FILES the live scan script
beside try 1's log. The attempt is kept in
`<phase>/superseded/scan-final-try2/` (final.log sha256
5a7c9251e747e348011d61da41097a725db922b7ecbe7e19593a2812f2d5bcbf).
Planning's ruling D, in the working specs/0041: RERUN_FILES names the two
superseded final.logs alone (change S2-a), RERUN_NOTE both attempts and
their causes; the scripts unchanged. scan.log is the third run, given
the working spec's hash at PLANNING_SPEC.
