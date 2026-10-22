# M6 part 4 P2 logs

specs/0041-chart-text-and-edits.md, "P2: the style value and the in-place
edits", and planning's P2 brief. The doc commit 8faf930 committed the spec
(sha256 fd01e97ea3469a12f1ae72fc51daf44f8ef542d6ccf1e862f3890b21727e71f6,
holding rulings A to D and planning's P1 verification) and ROADMAP.md
(sha256 897ae22b8223127b30226c99ceea53a9e57e849132c022b8099e6f98825dfb2d).
Planning's rulings E, F and G, written during P2, are in the working
specs/0041, uncommitted; every log takes it at PLANNING_SPEC, sha256
d59ed185c37ddfca356e4fb90fb36a647857e69d2af9853fef03be2e3e7a6a44. The
phase commit is f22955376c099d2e3e12d2a6bf39b4a6219a472f, the style
commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p4-p2`.

The phase's commits, each the next one's parent, on M6 part 4 P1's record
commit 1156042:

| Role | Commit | Subject |
|---|---|---|
| doc | 8faf930 | M6 part 4 P2: commit planning docs (the P1 verification and rulings) |
| core | 075abca | M6 part 4 P2: charts write, the style and the in-place edits |
| tool | 5548bcc | M6 part 4 P2: emend-copy --chart-edit-sample |
| op | 1adc74d | M6 part 4 P2: the openpyxl chart-text op |
| style (phase) | f229553 | M6 part 4 P2: the edit sample's chart takes style 26 (ruling G) |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 4ecd52bc76ce6c9f398381e1bbed1ef8d4f130fb0dec06a5d83b4535bc6e68d7 |
| test.log | test log checks all as expected: yes | bef63747476209563ad8c67dec5b430b0e550dffe7cebfd33c42c08ada3964a1 |
| negative.log | negative log checks all as expected: yes | 647e3945bf98ae9e2c23ba5c0a3615932259f25f76f38de99c157c1a2daf6102 |
| run.log | run log checks all as expected: yes | be00488e4c3cb78a82d7d3fc0ab73d8207caed3cc09dc0bcc64d371a397d9cee |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, run, and ended at 2026-10-22T01:03:35Z, 01:05:37Z, 01:05:51Z
and 03:44:22Z (each log's "run ended at" line), each with outer standard
error 0 bytes (the driver's count of the file each log's standard error
went to; session's statement). Each log is the first logged run of its
kind; no logged attempt was refused or stopped (session's statement).
One dry run ran before them, at f229553 with the same PLANNING_SPEC, into
`<phase>/dry/d1` (the gate's prune in list mode, the run's records into
the dry directory), and ended yes for all four logs. The run crossed
midnight UTC, so the records carry the date 2026-10-22.

The run log wrote records/m6p4-p2-chartedit-2026-10-22-f229553.{md,txt},
records/m6p4-p2-charttext-2026-10-22-f229553.{md,txt} (P1's text sample
rerun) and records/m6p4-open-2026-10-22-f229553.{md,txt}. P2 writes no
API record and no size record.

## Rulings in the working spec

- **Ruling E** (the readings, Q1 to Q9): the title's text and the edit's
  references are checked at the call, nothing recorded on an error; at the
  write, per part: an edit twice, of an index this write removes or beside
  an addition to the part, "no chart at that position"; remove_chart's part
  refusals; an index past the frame list, "no chart at that position"; the
  frame's chart part, "a chart part cannot be read"; the series index, "no
  series at that position" (the one new message); the walk's failures, "a
  chart part cannot be read". A removal of another index on the same part
  is allowed; a chartsheet takes "the worksheet part's root is not
  worksheet". cat/val, or xVal/yVal under scatterChart and bubbleChart; a
  parent outside the sixteen plots the writer knows cannot be read. The
  reference form found is kept, the writer's where absent or a literal. A
  kept rich title keeps bodyPr, lstStyle, the first paragraph's pPr and
  the first run's rPr, its t alone replaced, further runs and paragraphs
  dropped (the session also keeps the first paragraph's endParaRPr;
  planning: within ruling E); a strRef tx or a rich with no run takes the
  writer's rich text. None removes the title and sets autoTitleDeleted 1;
  Some on a chart with no title inserts one and autoTitleDeleted 1 becomes
  0. The chart prefix found; DrawingML by the root's binding, else xmlns:a
  on the inserted rich.
- **Ruling F** (the style's order): the style's range check (1 to 48, "a
  chart value is out of its range") right after the four texts' checks,
  before the combined plot's rules; pinned by
  the_texts_follow_the_check_order's two added cases.
- **Ruling G** (Q11, amended): the edit sample's added chart takes style
  26, in the style commit f229553, the phase commit. The run log's field
  comparison takes emend's c:style from that added chart part on the
  openpyxl fixture's edit output: the val equal (26 = 26), required; each
  side's preceding sibling printed as found (emend's roundedCorners,
  openpyxl's none, its style the root's first child), the difference
  named and not judged. The charts sample's labels 0 on openpyxl's chart
  is named as a reading fact. P1's text sample is unchanged and its rerun
  byte-identical.

## What the logs show

1. **gate.log:** six roles by full subject; five commits, one
   planning-doc and four code, each message ASCII ending with the
   Co-Authored-By line alone; the phase diff the doc commit's two files and
   the code commits' eleven; messages 154 distinct at the base and 155 at
   the phase commit, the base's set plus "no series at that position"; test
   names 1399 at the base and 1417 at the phase commit; the base tests of
   the four changed test files 528 (tests.rs 251, tests/chart_write.rs 104,
   tests/chartsheet.rs 20, the tool's 153), 2 edited (35 edit lines), none
   gone; the core commit's change to the tool's main.rs its twelve added
   style lines alone, the style commit's one line each way (None to
   Some(26)), the probe's two added style lines; check.sh's nine PASS
   lines; the figures the test log must give printed (off 515, ch 688, im
   552, ci 730, cc 733, cr 791, all 1170). The i686 charts probe: sections
   1136996, 29036 over part 3's size record's 1107960 and 24784 over P1's
   committed gate.log's 1112212, printed and judged nowhere there. M6 part
   4 P1's directory: its prune set (target alone) listed with du -sb,
   5979082811 bytes, and removed; its 33 kept names still there.
2. **test.log:** ws-host and ws-i686 passed 1414, failed 0 (P1's 1396
   plus 18); the feature runs on both targets 515 (off), 688 (charts), 730
   (charts,images), 733 (charts,comments), 791 (charts,read) and 1170
   (every feature), failed 0, each charts set P1's plus 15. Block O:
   scripts/openpyxl-ops-test.sh on the pinned image, "ok the parts reader
   made its 16 checks" and "openpyxl-ops-test: PASS".
3. **negative.log:** 15 cases refused, N1 to N15, and the accepted runs
   B0 to B10 (eleven outputs); N15 (the edit sample on N1's fixture,
   DRAWING_UNREADABLE, no output) and B10 (the edit sample's line on B0's
   fixture) are this phase's; the new message and the edit rules shown
   unreachable through the binary by the readers (reach7.py among them).
4. **run.log:** --chart-edit-sample over the corpus on host and i686, with
   release binaries built at the phase commit: 27 of 27 rows ok on each
   label, lo and lo_in ok 27, nothing dropped or recompressed; entry totals
   identical 794, changed 76, added 70 per label; every line series 1,
   read_back 3; charts 33 and kept_edited 4 in all, kept_edited 1 exactly
   in the four workbooks whose W held a chart; rows 1 to 1000 (the two
   large workbooks at 1000); outputs byte-identical across the labels, 27
   of 27. Every added chart part holds one c:style val 26 right after
   roundedCorners, 27 of 27, and the four kept chart parts retitled keep
   the input's style. P1's --chart-text-sample rerun: 27 of 27 per label
   ok, every output and line byte-identical by sha256 to P1's run
   directory, 27 of 27 per label. The Excel-form fixture (id c71ca2a4a1a9,
   4909 bytes) on both targets: the charts line as P1's (styles 1, ruling
   B), its twelve entries identical; the text line charts_before=1
   charts_after=4 series=4 rows=6 texts=6 styles=0 read_back=3; the edit
   line chartedit charts=2 series=1 rows=6 read_back=3 kept_edited=1, the
   kept chart part the fixture's with its title's run text alone replaced,
   byte for byte, its rels part and both style parts identical, the edit's
   nine checks yes. The openpyxl fixture (written by the pinned image,
   28875 bytes, id e6dcd6ce27d0): the charts line labels 0 (the reading
   fact) and styles 1; the text line charts_after=4 rows=100; the edit line
   chartedit charts=2 series=1 rows=100 read_back=3 kept_edited=1; the
   field comparison seven of seven the same form. The keep: passed 1414,
   failed 0; pairs 2215, of them 2173 the record's
   (records/m6p4-open-2026-10-21-f838d65.txt) unchanged and 42 new,
   exactly the 42 this phase's tests write; 0 gone, 0 changed; the open
   check: no output refused on an accepted input, crate (2065 pairs) or
   tool (150).

## The session's statements (no log prints them)

1. **The generators were written by forked subagents** of this session,
   one per log, from P1's generators and scripts (copied to
   `<phase>/model/m6p4-p1`, diff -r identical), with part 3 P2's
   (`<phase>/model/m6p3-p2`) for the op commit's forms, the openpyxl
   fixture and the second sample's rerun. Each generator checks its
   template byte-equal to the script printed in the committed log at
   1156042. This session read each fork's notes and ran the dry run.
   Labels: gate A80 to Z80, test XF to XL, negative NA53 to NF53, run NYY,
   NYZ and NZA to NZE, scan KF, KG, KH.
2. **Two generations.** The first (gen1) named the op commit 1adc74d the
   phase commit. Ruling G added the style commit, and each fork
   regenerated (gen2); each gen1 script was moved, not removed, to
   `<phase>/superseded/gen1/` with its generator copied beside it. No log
   ran a gen1 script. The test fork's first gen2 output, never handed
   over, was moved to `<phase>/superseded/gen2-draft/` when its header
   still called the op commit the phase commit. The negative fork's first
   gen1 script was moved to `<phase>/fork-scratch/negative/superseded-gen1/`.
   The gen2 hashes are in `<phase>/scripts-gen2.sha256`: gate 65a67d13...,
   test 2d080c06..., negative ec8015d9..., run da6768b7..., scan
   90c3dcce....
3. **The forks' departures.** The test fork's block O comes from part 3
   P2's test script as printed in 65703ee:records/logs/m6p3-p2/test.log
   (the model tree holds no part 3 P2 test script), pinned by sha256
   dbc3805f.... The scan fork dropped P1's specs/0040 pin block: P2's doc
   commit holds two files, and the base's specs/0040 equals P1's pin, so
   its control could not hold. The gate's prune holds the 33 kept names
   read from m6p4-p1 when the generator ran. The forks built in their own
   scratch target directories under `<phase>/fork-scratch/` for hand
   checks (the gate fork one consumer `cargo check`, the test fork `cargo
   test -- --list` per set, the negative and run forks debug and release
   emend-copy); none touched `<phase>/target` or the repository.
4. **The rm uses.** The scan fork removed one file,
   `<phase>/fork-scratch/scan/fixtures-src.sh`, a scratch copy it had made
   seconds before in its own scratch directory: a departure from its
   brief, which said to move aside. No other fork removed any file or
   directory. The prune (`rm -r` of m6p4-p1's `target` alone, no -f) is
   printed in gate.log. The other removals in the scripts are carried from
   their templates and remove only directories the same script made.
5. **The development figures**, at f229553 (not a log): check.sh nine PASS
   at 075abca, 5548bcc and f229553; the dev keep at 1adc74d 2215 pairs, 42
   new against P1's open record, 0 gone, 0 changed; the ops test PASS
   with 16 checks at 1adc74d.
6. **The forks' other notes.** The negative log adds reach7.py, a reader
   of the edit sample's calls and literals, with fourteen planted copies,
   each refused; reach5's and reach6's literal counts change by the edit
   sample's one literal alone. NO_SERIES_AT's covering tests give the
   message as a string literal; the log reads it from chart_write.rs and
   finds it in their bodies. The run log's openpyxl fixture is fresh each
   run and cannot match by sha256 across runs (specs/0038 ruling P's
   reading). The run fork's gen1 question (the comparison's style field,
   which no sample then wrote) is answered by ruling G.

## After the independent check

The check (independent-check-1.md) read this README as it stood before
this section; nothing above it changed after the check. Planning's
ruling H, in the working specs/0041 (sha256
dc8a959be5b96fa10326371554efd883870da17b5afbcd4d309ff5483b671f25, which
the final scan takes at PLANNING_SPEC), answers its points:

1. **The kept chart's title** (its point 1a): ruling E named the
   constant "emend edit" for the first kept chart's new title; the tool
   (5548bcc) writes "emend kept". The edit sample's three titles are
   distinct: the added chart "emend edit" at the first write and "emend
   edited" in place, the kept chart "emend kept". The departure began in
   the session's facts file, which read "retitled emend kept", and every
   generator followed it; the tool and the logs agree on the string, so
   no check failed. Ruling H: (a), ruling E amended in place to "emend
   kept"; no code or log changes.
2. **The order across parts** (1b): `record_chart_edits` applies the
   twice, removal and addition refusals over every part's edits first,
   then each part's other checks in order; no test pins the order across
   two parts. An accepted reading (ruling H).
3. **The overlay on an inserted title** (1c): the writer's own title form
   (`title_bytes`, chart_write.rs at f229553) writes `<c:overlay
   val="0"/>` too, so an inserted title takes the writer's form. Accepted
   (ruling H).
4. **The prefix** (1d): the chart elements take the prefix of the element
   they sit in or replace, which is the root's binding unless an inner
   element rebinds the chart namespace. Accepted (ruling H).
5. **The entry identity** (2a): "every other entry identical to the first
   write's output" is enforced by the tool itself, which exits 2 where an
   entry other than the edited chart parts differs; no log line prints
   the comparison, and every ok row of the corpus and fixture runs is
   that check passed. Accepted (ruling H).
6. **The answer's form:** extracted verbatim from the check's transcript;
   it names no path form and holds no U+2014; it held two U+2026
   characters, in quoted ellipses. Ruling H: committed as they stand, the
   scan the arbiter, and a named scrub if the scan refuses them (finding
   S1 below).

## Finding S1 and ruling H (the final scan's first run)

The final scan's first run, given PLANNING_SPEC dc8a959b..., ended
"RESULT: FAIL", total hits 6, structural checks failed 0: form
f_nonascii, count 6, in independent-check-1.md, the two U+2026
characters of three bytes each (its lines 43 and 55). The attempt is kept
in `<phase>/superseded/scan-final-try1/` (final.log sha256
3d47f32e407b02f13bcd7cb4acdc7313174f3d30dcfd752534890ad2a2b12a79).
Ruling H: the committed answer is the check's verbatim answer with each
U+2026 replaced by "..." (change S1-a), a scrub altering no judgment. The
original, outside the tree, has sha256
3aaf0a7db097119a16ef0bf827c035b812b2eb9c04840992869f954f49c7ff6d; the
committed file has sha256
26f35b4c72f5b7a36b5c0fb90d352871b45b20d1a4b853c74b4d2d4d840a7d30 and
differs from it on lines 43 and 55 alone. scan.log is the named rerun,
RERUN_FILES naming the superseded final.log alone.

## Finding S2 and ruling I (the final scan's second run)

The named rerun ended "RESULT: PASS", total hits 0, structural checks
failed 0, but its line 80 ("rerun: " and RERUN_NOTE verbatim) held the
home directory: the session had given RERUN_NOTE the phase directory's
expanded path, where P1's note wrote the literal `<phase>`. The scan
does not scan its own output, so it passed. The attempt is kept in
`<phase>/superseded/scan-final-try2/` (final.log sha256
29ad1ae676447c0a6f43902a7cdd94056e79af5e6662c55e09f91ab485bfa5fd), not
committed. Planning's ruling I, in the working specs/0041: RERUN_NOTE
with literal placeholders naming both attempts and their causes,
RERUN_FILES the two superseded final.logs (change S2-a), the scripts
unchanged; the log grepped for the home directory before it is
installed, 0 required. scan.log is the third run, given the working
spec's hash at PLANNING_SPEC. Carried to P3's scan generator: a
self-check that no line of the scan's own output holds the home
directory, the RERUN_NOTE line included.
