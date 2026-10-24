# M6 part 6 P2 logs

specs/0043-measurement-and-close.md, "P2: the runs", and planning's P2
brief. P2's doc commit d88f735, the phase commit (ruling H, R1 (a)),
committed the spec at sha256
a18af81dcd4d2f839617b596a829bbb058e8233a5116eafe4cc14a8ec8fe0672 (rulings
A to G and "Planning verification of P1") and ROADMAP.md at sha256
0aa530c7c96a00787d7ab664f00ff8fe1fb5ad81ec7e7ff510038969a90477ac.
Planning's rulings H to J, written during P2, are in the working
specs/0043, uncommitted; every log takes it at PLANNING_SPEC: sha256
d761db90dab3d39664e417225b8f7829b55151bed96296b98c55625364ab86e7 (rulings
H and I) for the gate, the test and the negative, and sha256
4a971987a6b742997d77f1fc21a76369fc255a1f7f88ac18d9d7087209bcc4ee (ruling J
added) for the run's rerun; the scan takes it at sha256
ad36059f86afce63f5bf005f1a06915485d737670945584b923de5ff43fc5eb9 (ruling K
added). The phase commit is
d88f7359f323da40da287d8f16ad9e1a8b8b9bd1; P2 has no code commit.
`<phase>` is the phase's directory, `<home>/.cache/emend-build/m6p6-p2`.

root: <superseded> stands for <phase>/superseded, the phase's superseded directory (specs/0043 ruling K, finding F4)

The phase's one commit, on P1's record commit 65d4c5a:

| Role | Commit | Subject |
|---|---|---|
| doc (phase) | d88f735 | M6 part 6 P2: commit planning docs (the P1 verification and rulings) |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 64cb2e1580aedf6b260c8b6211bb65c7321cf0ce214177178a2dc590eaf4c03a |
| test.log | test log checks all as expected: yes | 6c23f3374196c91602324ea4865097e60693df4c769ecd9a5932ce4d5ff985d8 |
| negative.log | negative log checks all as expected: yes | f9cbfb2a9f40518321e8ec135c7d600eca6324af92611fd59a53e5aea85044a8 |
| run.log | run log checks all as expected: yes | af820041923cff7fe3c0e5070a59d30e8ed127b95c64a2cfcd4cb18e75014255 |

The scan's log, scan.log, is written last over these four, this README,
the check answers and the 203 record files the run installed.

## Rulings in the working spec

- H (the generators' readings R1 to R9, each as recommended): five logs,
  roles base 65d4c5a and doc d88f735 the phase commit, no check log; the
  run's template M5 part 6 P2's regenerated run script 7c32179e; the
  settings of R3; the ops table governs emend's sample per op
  (chart-analytic `--chart-axis-sample`, named-style
  `--namedstyle-sample`); 203 record files; 0 new open-check pairs
  expected; the gate prunes P1's target root; a restricted dry run; a cap
  or timeout an outcome, nothing rerun for it.
- I (G-R1, the departures, RUN-R1): the gate's structure check takes M4
  part 1 P3's form for a range of doc commits only (change A44), the
  carried check requiring a code commit kept as a control that refuses
  this range; departure NC59 accepted; no exemption added for the new
  ops; a failure of the comparison record's row check at the end of the
  run would be a finding, redone in the check-log form.
- J (finding F3, below): d1's staged files moved to the dry run's
  directory, the refused attempt's three files to the phase's superseded
  directory, the run step alone rerun into the phase's logs with
  RERUN_NOTE naming F3 and RERUN_FILES naming the three; gate, test and
  negative stand as logged.
- K (finding F4, below): the session's slip in the RERUN_NOTE, not a
  defect; the measurements and run.log stand, no rerun; the scan
  generator's change LC lets a placeholder a log prints be declared by a
  root line in this README as well as in the log, with a control planting
  a token declared in neither and shown refused; this README declares
  <superseded> in the root line above. A RERUN_NOTE or RERUN_FILES value
  uses only tokens the script it is given to declares.

## What the logs show

- The gate (template P1's gate, changes A86 to Z86):
  - the two roles by full subject; the range a doc commit alone, "every
    commit is planning docs alone, the range opens with a doc commit and
    holds no code commit (change A86): as-expected" (ruling I, G-R1);
    nothing under crates/, tools/ or scripts/ changed;
  - the prune (ruling H, R7): P1's target, 1 of 1, 6167760653 bytes,
    removed by `rm -r` without -f, every kept name still there;
  - check.sh nine PASS lines; the crate's test sets off 538, ch 711, im
    575, ci 753, cc 756, cr 818, all 1199 on host and i686, part 5 P3's
    committed figures;
  - part 5's size record's figures read alone: the charts probe 1151328,
    the format probe 984664, the budget 451448.
- The test (template P1's test, changes ZA to ZC): block O,
  scripts/openpyxl-ops-test.sh over the image, "109 ok lines, no FAIL, the
  last line PASS"; T1, scripts/compare-record-test.py at the phase commit,
  "34 ok lines, no FAIL, the last line PASS".
- The negative (template P1's negative, changes NA59, NC59, NI59): C0, R0,
  N1 to N4 as P1's.
- The run (template 7c32179e, changes RM to RT), the rerun of ruling J,
  its "rerun:" line naming F3:
  - the sixteen ops over the corpus, both emend labels and openpyxl, and
    the scale tiers a to d; settings chosen before the run and printed;
  - the open check: the host keep against
    records/m6p5-open-2026-10-23-fe280ff.txt, rows 2237 and 2237, pairs
    new 0 and gone 0, no pair's bytes changed, no verdict or class moved
    (ruling H, R6);
  - the comparison record records/compare-m6p6-2026-10-24-d88f735.md
    (sha256 56673747ab8611f3a8c95dd3ad1e14e80b4f0e6a1100a311ccb09eb22bd1c0f4),
    every row derived again from the summaries: yes (RUN-R1 did not
    arise);
  - "the whole run installed 203 record files, the set specs/0043 ruling
    H (R5) names (change RR): yes"; no record file names a home, mount,
    temporary or corpus path.

## The session's statements (no log prints them)

- The generators were written by four forks from `<phase>/FACTS.md` and the
  models under `<phase>/model/` (P1's gate, test, negative and scan and
  their generators; M5 part 6 P2's run template 7c32179e, run-script-3
  04ce3671 and its generator), each printed model checked byte-equal to
  its committed print. Generators and scripts, sha256:

  | Log | Generator | Script |
  |---|---|---|
  | gate | 425afd9d2fa994d35199dc5a0d5b5d73b46335742ab5dc1ffc6d460820de919c | f6e6988de27fb706c8d3895f6afae913228c637f2b7216f8bc3fdf1a67e038f7 |
  | test | b21b688f635dc7b67f9e52da3a13e615aadf8f3dd78f587f3057042fa5044581 | 7672c91761e4c7cfec33de763f175c8568b10ff93146511800ee8546a796ad0d |
  | negative | b65f5ae366ec43aad78e1d9d3ff0716954a87448b305bd4bc93bcd5aa6d7f784 | 196d6d2e34dc221e6f094f64942a23c69e26c6c86cacbb5ab91e6d71a2b20690 |
  | run | f5a6dd1b01786ab1896f657f6be10693f81676399adc65f982ffcafa5fa66ee8 | 7d33927f6ba30378a88293df860a495a6181db042631ab1c2a5959b2d00fb6fa |
  | scan | 5e2cf0dddb9b969daf2fb8c3e77e4fa73d09e54960f188ea660ba64f1928c557 | c63cb0aad5b77fba715d8e0df3d426de1f93de6486bc95f0473d3b97cc6aaf19 |

  The scan's generator and script are those of change LC (ruling K); the
  first, 30ec892e... and c99f8315..., are kept under
  `<phase>/superseded/scan-1`.

- Dry run d1 (PLANNING_SPEC 161f8ed6..., ruling H), the restricted dry run
  of R8: gate, test and negative each "yes"; the run over two ids, the ops
  roundtrip, chartsheet, chart-analytic and named-style, tier a, no keep,
  stand-in directories, "yes"; outer standard error 0 bytes each. d1 is
  kept under `<phase>/dry/d1`.
- The logged runs (PLANNING_SPEC d761db90...): gate, test and negative
  each "yes", outer standard error 0 bytes. The run exited 2 at its
  pre-flight: d1 had left its 32 staged record files in the run script's
  staging directory under the same stems, and the script refused to
  overwrite them ("... is already there; move it aside first"), writing
  an empty run log and measuring nothing (finding F3). Under ruling J,
  the 32 staged files were moved to `<phase>/dry/d1/staged-records` and
  the refused attempt's run.log (0 bytes), run.outer.stderr (0 bytes) and
  run.stderr (132 bytes, sha256
  5359017fef98552a26d66b8e7908df672d4f6795f834740c124c4e73ee9532b9) to
  `<phase>/superseded/run-refused-1`. The run step alone was then rerun
  at PLANNING_SPEC 4a971987... (the working spec had moved by ruling J's
  addition; the run admits only the working file's own hash): exit 0,
  outer standard error 0 bytes, ended 2026-10-24T12:18:28Z.
- The first scan pre-check (PLANNING_SPEC 4a971987..., script c99f8315...)
  exited 1 with "RESULT: FAIL", total hits 0, "structural checks failed:
  1": "every placeholder records/logs/m6p6-p2/run.log prints is declared
  by a root line: no", undeclared 1, the token <superseded>. It is printed
  only in run.log's lines 4 (the RERUN_NOTE input) and 69 (the "rerun:"
  line), from the note the session wrote in the rerun driver ("... kept in
  <superseded>/run-refused-1 ..."); the run script declares no such token
  (finding F4). That pre-check's log (sha256
  61ab485ddc972c3d6937526f6caab8c603d3eb5250bb2658b69dfbb6b69c006f) is kept
  under `<phase>/superseded/scan-1/scan-pre`.
- The forks' departures: the gate prunes through P1's prune_dir for one
  directory rather than part 5 P3's older single-root block, the checks
  the same; the gate's structure check is M4 part 1 P3's A44 form (G-R1,
  ruled under I); the negative's NC59 reads the runner's "eleven" at P1's
  doc commit 55e6363 (ruled under I); the run adds PLANNING_SPEC in P1's
  form where the template only printed the working spec, and adds no
  exemption for the new ops; the scan keeps its template's stale comments
  and replaces the code-commit checks with LB's no-code-commit checks.
  Earlier generator outputs were moved aside, none run:
  `<phase>/superseded/gate-gen-try1` to `try3` and `run-gen1` (the run
  generator's first output, byte-identical, kept to show it is
  deterministic).

## After the independent check

The check (independent-check-1.md, a heading line then its answer
verbatim, sha256 75e72493efacac6df49a13dfe3be6f98796ef0b74f3445208a98d80caa7cb035
for the answer alone) read this README as it stood before this section
(sha256 7b304e0f1334b8964631c6c939f63578fac4c833b860dd3060ddd2e99f5f50ed);
nothing above it changed after the check. Its prompt is the brief file
but for that file's final newline; the draft inline in it is the frozen
draft byte for byte (session's statement). Its verdicts: 1 yes with
exceptions; 2 yes; 3 yes with exceptions, no figure differing from its
primary. Its points:

1. **LC's second control** (its 1 a): the acceptance case beside ruling
   K's refusal case; a control of the same change, adding checking only.
2. **The self-source line's label** (1 b): the scan prints "declared by
   root lines (the log's and the logs README's, change LC)" for this
   script's own source too, while LC checks that source against its own
   root lines alone, as its change list says; the verdict is right, the
   wording of that one line is the template loop's shared label.
3. **The prune's form** (1 c, 3 g): ruling H R7 names the single-root
   form; the gate calls P1's prune_dir for one directory, a departure the
   forks' list above records, the checks the same (du -sb before, rm -r
   without -f, kept and gone checks). No ruling names this departure;
   it is raised to planning with this record.
4. **F4's quotes and the first pre-check** (3 a): carried by
   `<phase>/superseded/scan-1/scan-pre/scan.log` (sha256 61ab485d...),
   outside the check's primaries; the check read it and found the quote
   at its line 8486 and "RESULT: FAIL".
5. **The refused run's exit 2** (3 b): the session's statement, from the
   logged driver's output; ruling J records the refusal without a code.
6. **"Both ruled by planning before any fix"** (3 c): the session's
   statement; rulings J and K record planning's verification of the state
   before each fix.
7. **"H to J" in this README's first paragraph** (3 e): written before
   ruling K; that paragraph names ruling K's spec hash for the scan, and
   "Rulings in the working spec" lists K. The working spec holds H to K.
8. **The generator and script hashes** (3 f): the session's statements;
   the check found them equal to the files under `<phase>`.

No code, log or record changed after the check; nothing was rerun.
