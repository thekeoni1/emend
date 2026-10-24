# M6 part 6 P1 logs

specs/0043-measurement-and-close.md, "P1: the scripts", and planning's P1
brief. The part's doc commit 9c62247 added the spec (sha256
2085cb2ac8ecf98e04a335b1e428ac945b1c01216c3d7c89236ffeb925f52345) with
specs/0042 (sha256
0cb1cd471e67f10d538f62473250faa56fedcecc24948ea703871780011c4789) and
ROADMAP.md (sha256
53c89cd40363cec79693cd843b822dab9d62b9a9a3cf5183a19b524e149c1cab); P1's doc
commit 55e6363 committed ruling A (sha256
ceed814eeed7d206819e985126d5fae66b1ee8a0b51ad9188c23b839b07a8430).
Planning's rulings B to F, written during P1, are in the working
specs/0043, uncommitted; every log takes it at PLANNING_SPEC, sha256
17adfe7737ea3bc7b608bdb8379550805d0baf2d5bc6973c4c0b701c7ba43cc1 for
the gate and the test, and sha256
38bd78f475c5c04f2cfd855d22aa113467944c2157f7de615d2f1754571fa367 (ruling F
added) for the negative's rerun and the scan. The phase commit is 99b671d06bcc5dd6d32bd1c69204e1841f4e9825, the code commit.
`<phase>` is the phase's directory, `<home>/.cache/emend-build/m6p6-p1`.

The phase's commits, each the next one's parent, on part 5's close-out
record commit f15a11b:

| Role | Commit | Subject |
|---|---|---|
| part doc | 9c62247 | M6 part 6: commit the spec and planning docs (part 5 close-out verification, part 6 spec) |
| doc | 55e6363 | M6 part 6 P1: commit planning docs (ruling A, the ops table's pairing) |
| code (phase) | 99b671d | M6 part 6 P1: the comparison scripts gain the five ops and the stem compare-m6p6- |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 3a242a7b27288b6159ea84ef9b702e5b84832aad7651c129552c9863c97d857e |
| test.log | test log checks all as expected: yes | be7ac53682d5a2c48661e51c87df7b9cc0efde462a08a9aec54a1086af7007ae |
| negative.log | negative log checks all as expected: yes | 530f3630787f79fe3ab264c6b97ccb8fcc174f4bc1fffb7e0fbd613e68dcfe7e |

The scan's log, scan.log, is written last over these three, this README
and the check answers.

## Rulings in the working spec

- A (the ops table's pairing, committed in 55e6363): the five new
  pairings are planning's reading of the sample lists the docstring of
  scripts/openpyxl-ops.py cites; the docstring governs each op's content;
  scripts/openpyxl-ops.py unchanged.
- B (Q1 to Q4, each (a)): the runner's refusal reads "sixteen", that line
  and OPS_LIST its only changes; the record script's docstring, the
  record's title and its opening paragraph name Milestone 6 part 6 and
  specs/0043; the record-script half of "an op outside the sixteen" is a
  run.openpyxl.tsv naming an op outside them, refused "names another op",
  no new refusal code; part 5 P3's generators are the templates;
  compare-record-test.py takes OPS, the stem, the counts and the
  docstring's "eleven" only.
- C (G1 (a)): P1's gate prunes part 5's target roots in part 5 P1's
  multi-root form; it also records the session's stray rm line (below).
- D (S1 (a), S2): the scan's range from f15a11b with 9c62247 classed a doc
  commit; the scan's dropped code-path controls accepted as a departure
  this README records.
- E (finding F1 of dry run d1): the gate generator writes the three fixture
  lines naming the comparison record's test with the name built from its
  piece variable; the check and its control unchanged.
- F (finding F2 of the scan pre-check): the negative generator adds the
  two root lines declaring <corpus-runs> and <scale-runs> in M5 part 6 P2's
  change RB wording (change NH58), the case code unchanged; a dry rerun of
  the negative, then its logged rerun in the carried form (RERUN_NOTE,
  RERUN_FILES); the gate and the test stand as logged.

## What the logs show

- The gate (template part 5 P3's gate, changes A85 to Z85 and X85):
  - the four roles by full subject; nothing under crates/ changed since
    part 5 P3's phase commit; the phase diff six files, the code commit's
    three scripts each modified; specs/0043 the one added file;
  - the prune (ruling C): part 5 P1's and P2's directories hold no target
    (0 of 1 each); P3's target, 9583979959 bytes, and the close-out's,
    2143316010 bytes, removed by `rm -r` without -f, every kept name still
    there;
  - check.sh nine PASS lines; the crate's test sets the figures part 5
    P3's committed gate.log gives: off 538, ch 711, im 575, ci 753, cc 756,
    cr 818, all 1199;
  - the crate's messages the base's, 155 by the one-line reader and 169 by
    the several-line reader;
  - ruling BI (specs/0042) and item 5 (specs/0043): the charts probe's
    sections 1151328 against part 5's size record's 1151328 alone, the
    difference 0; the format probe's 984664 against the record's 984664,
    the budget 451448 against the record's 451448; judged nowhere;
  - the self-check that the gate runs neither test nor the container tool:
    "lines: 0" (change X85).
- The test (template part 5 P3's test, changes YR to YZ): no cargo run;
  block P hashes the four scripts against their blobs and prints the two
  test scripts; block O, scripts/openpyxl-ops-test.sh over the image, "109
  ok lines, no FAIL, the last line PASS"; T1, scripts/compare-record-test.py
  at the phase commit, "34 ok lines, no FAIL, the last line PASS".
- The negative (template part 5 P3's negative, changes NA58 to NH58), the
  logged rerun of ruling F: its "rerun:" line names finding F2, the first
  attempt's three files hashed and kept in <superseded>/negative-1; no
  binary case; C0, the runner's unplanted control over the generated tier
  t; R0, the record script's unplanted control on fictional runs of the
  sixteen ops; N1, --op no-such-op refused by the runner and a
  run.openpyxl.tsv naming no-such-op refused by the record script; N2, a
  run.openpyxl.tsv naming another of the sixteen refused; N3, runs not
  covering the same workbooks refused; N4, a planted path stays in the run
  directory's stderr file and no run prints it. The fictional run
  directories are built by the log's rfx.py with the phase commit's
  compare-record-test.py's own build(), that file printed whole.

## The session's statements (no log prints them)

- The generators were written by four forks from `<phase>/FACTS.md` and the
  models under `<phase>/model/` (part 5 P3's gate, test, negative and
  scan, part 5 P2's test, part 5 P1's gate, M5 part 6 P2's test and
  negative), each model checked byte-equal to its committed print.
  Generators and scripts, sha256:

  | Log | Generator | Script |
  |---|---|---|
  | gate | 12cc147a54bcd277457a2ead65721ed31bfe834b1a7782f445176262824e7a19 | c90b67bc0893045bee4ece7c8f749b1207e161a3912ac6e9f73aeafbd10c5ae6 |
  | test | 12cc7f6f5116d362c7b834b1ae5e119623d669340bd22018f3ec1cf739669f2b | 847bf84649eae37df91e99925f6c253102008c1abf47145b558fb5359f0e6f84 |
  | negative | da9f576de41d8bb709e349c9b992f71c4da8a2f5ff0811114015824b5765f6df | 8dc36b3c1ba9a0b2cd4e827cce66abc624d6031f9c4e1875b647fc880219eb31 |
  | scan | fd2649287d8c157f1cde4168606707b4587c8dd4962562b8ca7d46262dcb3eef | 7b5effcf15e8e1ab60c8d6da4bc6aedf19e4a028ca4c1e37becf9507f9a05189 |

- Dry run d1 (PLANNING_SPEC 0015aed2...): the gate exited 1 with one check
  "UNEXPECTED", the self-check that the gate runs neither test nor the
  container tool, "lines: 3": the three fixture lines E85, H85 and I85
  wrote naming scripts/compare-record-test.py as data (finding F1). The
  test and the negative did not run. d1 is kept under `<phase>/dry/d1`.
  Ruling E's fix is change X85; the first gate generator and its script
  (c90b67bc's predecessor, 12973567...) are under `<phase>/superseded`.
- Dry run d2 (PLANNING_SPEC 17adfe77...): gate, test and negative each
  "yes", outer standard error 0 bytes. The logged runs followed, each
  "yes", outer standard error 0 bytes, `git status` showing the working
  spec alone; the first logged negative.log was sha256
  3958c67d839c864692c01c8b784cd6f54794337f9bd7236d20a23fde56a81d14.
- The scan pre-check over those logs (`<phase>/scan-pre/scan.log`, exit 1)
  ended "RESULT: FAIL" with "structural checks failed: 1", total hits 0:
  "every placeholder records/logs/m6p6-p1/negative.log prints is declared
  by a root line: no", the words <corpus-runs> and <scale-runs> of the
  printed scripts/compare-record.py (finding F2). Ruling F's fix is change
  NH58 (the first negative generator a97f127c... and its parts under
  `<phase>/superseded/negative-gen1`, its script d2d9dedc... with the first
  logged negative.log and outer stderr under
  `<phase>/superseded/negative-1`).
- Dry run d3 (PLANNING_SPEC 38bd78f4...): the negative alone, given the
  logged gate log, "yes", outer standard error 0 bytes. The logged rerun
  followed, "yes", outer standard error 0 bytes.
- The session's error (ruling C): a stray `rm -rf /dev/null 2>/dev/null;
  true` line went into a dev command copying the models. It failed (not
  root) and removed nothing; /dev/null is intact (crw-rw-rw- 1,3). No other
  rm was typed in a dev command.
- The forks' departures: the gate generator leaves its reader files in
  `<phase>/tmp/gate-gen-<pid>` and removes nothing; its trial outputs are
  in `<phase>/fork-scratch`. The test's T1 expected lines are fixed at
  generation (M5 part 6 P2's change RC form); block P prints the two test
  scripts and only hashes compare-record.py and openpyxl-ops.py; the stem
  control is the log's, not a new test assertion. The negative carries
  none of M5 part 6 P2's N2 to N5, N7 or the direct ops-script runs, and
  removes its own scratch work directory at the end as part 5 P3's
  negative does. The scan drops about a dozen of its template's code-path
  controls naming older phases' commits (ruling D), the remaining verdicts
  computed from git and asserted; its stale comments ("four siblings") are
  carried as the template had them.
- The code commit's dev checks before the commit: compare-record-test.py
  exit 0, 34 ok lines, 0 FAIL; compare-run.sh --op no-such-op refused with
  the "sixteen" message, three of the sixteen passing the op check.

## After the independent check

The check (independent-check-1.md, a heading line then its answer
verbatim, sha256 67e4ccfd9fbbad2a065c8df5796dfd528e9aa64222d26f7beb908c98ff3432f5
for the answer alone) read this README as it stood before this section
(sha256 01db796321cd730496bbbb30a71c2b49b1f3f4ea18145c14fdd88b9090a9fcbc);
nothing above it changed after the check. Its prompt is the brief file
but for that file's final newline; the draft inline in it is the frozen
draft byte for byte (session's statement). Its verdicts: 1 yes; 2 yes;
3 yes with exceptions, no figure differing from its primary. Its points:

1. **The second pre-check's path** (its 3 a): `<phase>/scan-pre2/scan.log`
   is the passing pre-check over the logged rerun (sha256
   cf1b10d3f72bfa16e8fc5c3ae631dbbd799969785b0c347ae53b59a2816ad1a9);
   `<phase>/scan-pre/scan.log` is the first, failing one (F2). The final
   report names both.
2. **The stray line shortened** (3 b): in full it is `rm -rf /dev/null
   2>/dev/null; true`, as above; the final report quotes it whole.
3. **"Both ruled by planning before any fix"** (3 c): the session's
   statement; rulings E and F prescribe the fixes; the first generators
   and the first logged negative are kept under `<phase>/superseded`.
4. **The dry runs and generator hashes** (3 d): the session's
   statements, as above; d1 the check read itself.
5. **Nothing pushed** (3 e): the session's statement; origin/main is
   ee78ffa, which does not contain f15a11b.
6. **The prune visiting P1's and P2's directories** (2, an observation):
   part 5 P1's multi-root form, which ruling C names, lists every
   directory of the part; P1's and P2's hold no target (0 of 1) and
   nothing was removed there.
7. **The doc commits' content** (1, a doubt): ROADMAP.md, specs/0042 and
   specs/0043 are planning's text, committed as planning gave them.

No code, log or record changed after the check; nothing was rerun.
