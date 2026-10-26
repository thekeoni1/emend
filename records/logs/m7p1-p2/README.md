# M7 part 1 P2 logs

specs/0044-excel-check.md, "P2: the record", "The record (P2)", "The
restored Excel-record check (P2 and the close-out)", and planning's
rulings on the P2 results, T to AD. P2's doc commit ad139c3, the phase
commit, committed the spec at sha256
5b32ead8fb72dba0880c5e82ed217f5e9bf85c2ebec8baa44a31e6991fafb8f9 and
ROADMAP.md at sha256
2ebcd1e29e04ce08bad5013782d64ba220b0d0940f1ab195b2f76751ed325217.
Planning's rulings Z to AD, written during P2, are in the working
specs/0044, uncommitted; the gate, the negative and the run of the logged
attempt 2 take it at PLANNING_SPEC: sha256
9de4dd8e569bfd0a383ec303d74c8b40387887b40fcf9d15c2bf5912f59b71e6 (757
lines). The phase commit is ad139c3e0cc9b6cff545ffaf7bfe1f0c26e8643f; P2
has no code commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m7p1-p2`.

root: <dry> stands for <phase>/dry, the phase's dry runs' directory (specs/0043 ruling Q)
root: <superseded> stands for <phase>/superseded, the phase's superseded directory
root: <close> stands for <home>/.cache/emend-build/m7p1-close, the close-out's generator directory (ruling X)

The phase's one commit, on P1's record commit a72467d:

| Role | Commit | Subject |
|---|---|---|
| doc (phase) | ad139c3 | M7 part 1 P2: commit planning docs (the P1 verification and rulings) |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 8d21c89dc7a55868cac909561ba25cdb5e1442b698b4e3caa7e19ffcf00349e7 |
| negative.log | negative log checks all as expected: yes | 7bd8e0397d688892de2e828e718a18bf147eebc78d99d954c98de6099a9246b8 |
| run.log | run log checks all as expected: yes | c23251d00beee23ae6329c1c62359c506f09655adf20b0fcafed061769dc2115 |

The record `records/excel-check-m7-2026-10-26.md` (sha256
47e6f360966e03500419c665037add00712e5c1c9765c508403d6cd8c0000d25, 844290
bytes) was assembled by planning (ruling V) and written by the run log's
step 2 alone (ruling W). The scan's log, scan.log, is written last over
the three logs, this README, the check's answer and the record. No test
log and no size log.

## Rulings in the working spec

- T, U: the by-hand fallback on the 32 refused corpus rows; a row Excel
  offered to repair reads `repair`, the cell read by its first word.
- V: planning's assembly script, by-hand file and record, under
  `<phase>/record-from-planning`; the script is not committed.
- W: the logs: the record copied and its sha256 printed equal; the
  assembly run again into a scratch file, equal; the pair counts; the
  totals by awk; no FINDING line for the record's own findings.
- X: the close-out generator written now under `<close>`, its push script
  generated and not run; the negative log extracts the restored check by
  a printed line range and runs it on two scratch clones.
- Y: the doc commit's form; the check's two questions; the report.
- Z: the gate prunes P1's `target/main` and `target/consumer`, the
  CACHEDIR.TAG condition printed as information.
- AA: the totals awk reads the results file and the by-hand file, the
  results file's counts alone printed as information.
- AB: the pairs matched per source: 1560 and 2229, 3789 of 3789; the
  input pairs 2229 of 2229.
- AC: change 64's heading "no Milestone 2 Excel check record at the tip".
- AD: the prune by `rm -r "${PREV_DIR:?}/${n:?}"` (change R90), after
  logged attempt 1's cargo clean refused both roots; a control removes a
  target-shaped directory of the gate's own by the same form.

## What the logs show

- The gate (P1's gate, its changes continuing to K90 and R90),
  2026-10-26T17:45:43Z to 17:52:39Z:
  - the one role by full subject, the doc commit the phase commit, its
    parent a72467d, its two files and their two "<path> sha256" pairs;
  - the rerun line naming logged attempt 1 (below) and its two files;
  - the prune (rulings Z and AD): R90's control "target: rm -r exit 0;
    output bytes 0"; `<home>/.cache/emend-build/m7p1-p1/target/main`
    6142303428 bytes and `target/consumer` 73379815 bytes, 6215683243 in
    all, each "rm -r exit 0", every name of the prune set gone and every
    kept name still there;
  - check.sh nine PASS lines; the crate's test sets off 539, ch 713, im
    576, ci 755, cc 758, cr 820, all 1201, M6 part 6 P4's committed
    figures;
  - the release builds: emend-copy sha256
    35814fb2d9deb4b5a4536a59a30799657ef4815673261021660210e5961f4bea,
    3954088 bytes; xlsx-diff sha256
    1445523df9522bff56d4f0e03d7bde2ab333bac294020da256de42c01ef71f6b,
    897296 bytes (no P2 log runs them).
- The negative (P1's negative's header and rules, changes NA63 to NH63),
  to 17:52:53Z:
  - the assembly's control AC: exit 0, its output the record's bytes
    (sha256 47e6f360...); A1, a planted row: exit 2, the one STOP line
    naming no list row; A2, a removed row: exit 2, the one STOP line for
    a list row with no result;
  - the close-out generator under `<close>` run again into scratch, its
    three scripts byte-equal to `<close>`'s;
  - the restored check extracted from `<close>/m7p1close-push.sh` lines
    865 to 888, 24 lines, sha256
    7c70651dba081ab1967033cf7be6503243bbb459973710ef30aa080c3d299045,
    equal to the script's lines by sed, with its control;
  - scratch clones of the tip: XA, the record added and committed,
    accepted (3789 result cells); XB, the tip as it is, refused; XC, the
    Results heading renamed, refused; XD and XE, every result cell
    rewritten to "unread" and to "passed", refused; XF, every cell
    "fail", accepted; XG, a second matching record, refused. No clone's
    commit enters the repository.
- The run (P1's run's header and rules, changes TB to TF), to 17:52:55Z:
  - the inputs at their pins; the stage listed before and after, the
    same listing (sha256
    98175b45f2b53c3f2369133ccc1d9d8ccda02910aa39f1a2a38f310e98c3e24c),
    nothing written under it;
  - the record copied: sha256 47e6f360..., 844290 bytes, cmp-equal to
    planning's file;
  - the assembly run again: exit 0, standard error 0 bytes, its output
    the record by sha256 and cmp;
  - the pairs (ruling AB): corpus 1560 of 1560, fixture 2229 of 2229,
    3789 of 3789; the input pairs 2229 of 2229; the planted control
    1559 and 2228;
  - the totals (ruling AA): "| 6018 | 1528 | 32 | 0 | 1864 | 0 | 365 |
    1874 | 0 | 355 |", recomputed and the record's equal; the results
    file alone, as information, "| 6018 | 1528 | 0 | 32 | 1864 | 0 |
    365 | 1874 | 0 | 355 |", the 32 by-hand rows replacing 32 results;
  - "FINDING lines printed above: 0".

The record's five findings (the chart-surface sample on 27 of 27
workbooks, edit on 2, protect on 2, remove on 1, and 32 fixture pairs
pass on the input and refused on the output) are its content, carried to
part 2's scoping (ruling W); no log prints a FINDING line for them.

## The session's statements (no log prints them)

- Generators and scripts, sha256:

  | Log | Generator | Script |
  |---|---|---|
  | gate | 2c796dd64f0057528bb80f22c0cbef016b65fb79f2b85e8f97280e6590ad0ff5 | 2979d334044888279d176bd7a280b769c949b1db2f0f75ff3ac80967a92c88e2 |
  | negative | 126dc0e2a6a7d9ed553e0852a283d0434ebbec4171d558f6b867797920d59b52 | e8198231ab77c1be697e3dea0d411303ee009c5631a2ce3c2d828dc2a10340db |
  | run | b67fc69a628e1792403e83f16e3feab1cf6a5bf82c963d7d34ba62ac5a9ff793 | af16269a85ce923b64d778f318b5317119ede5aa2cd53f501741a69825cda0ff |
  | scan | 8fcb0ebe7c2f64fcd394959269b04fcb802da0b1ecc8b6de2243b3f5aa7b3a43 | 973ff0f6d83ee43e64cad31025eedc0383b60f11328ae094db968ae5ce1ed516 |

- The close-out generator `<close>/gen.py` a43b4ddac692691c0a4ef41d22597f62703c597d871e833282d5fec241671cfd;
  `m7p1close-push.sh`
  3e85a0b6fa1c37c13a45eb9c0d939d2d0875a63db566539765aa04384e10714a,
  `m7p1close-ci.sh`
  a07ee2a8dafbe4d557991b408a869d20d1ec8edaeb04fb612baee4a6b980f888,
  `m7p1close-scan.sh`
  b7b2a4ba1d9774bf4a987bfa9115446de4a916fc0ebe38754e2590dab589a719;
  changes 63 to 65; not run.
- The scan script's first generation (P1's stale top header) was
  superseded before any run: `<superseded>/m7p1-p2-scan.gen1.sh`.
- Dry run d1 (PLANNING_SPEC 4eed9808...): gate, negative and run "yes";
  the scan pre-check RESULT: FAIL on the two dry-layout lines and one
  more, the gate printing `<corpus>` (from the doc commit's ROADMAP.md
  text) with no root line. Fixed by gate change K90 (a root line); the
  old gate script at `<superseded>/m7p1-p2-gate.gen1.sh`. d1 is kept under
  `<dry>/d1`.
- Dry run d2: gate, negative and run "yes", exit 0; the scan pre-check
  RESULT: FAIL with the two dry-layout lines alone (LOGDIR and RECDIR not
  the defaults; the home-directory line, every hit a `<dry>` path); no
  FINDING line. Kept under `<dry>/d2`.
- Logged attempt 1 (PLANNING_SPEC 4eed9808..., 2026-10-26T17:28:10Z): the
  gate exited 1, "gate log checks all as expected: no": cargo clean
  refused P1's `target/main` and `target/consumer` (exit 101 each,
  missing CACHEDIR.TAG), nothing removed; the negative and the run did not
  start. Its files are kept in `<superseded>/logged-attempt-1` (gate.log
  sha256 6bc560170b3002aadfd239e32095ba80fcf47193fccd2ab2a115f853933df30a),
  the gate script that wrote it in `<superseded>/gen-before-AD`
  (cfb79205...). The empty `records/logs/m7p1-p2` directory that attempt
  made was removed with rmdir before attempt 2. Ruling AD followed.
- Dry run d3 (PLANNING_SPEC 9de4dd8e..., change R90): gate, negative and
  run "yes", exit 0; R90's control printed; the scan pre-check with the
  two dry-layout lines alone; no FINDING line. The other three scripts,
  regenerated at the new pin, were byte-equal to those of d2. Kept under
  `<dry>/d3`.
- Logged attempt 2 (PLANNING_SPEC 9de4dd8e..., started on planning's
  conditional word after d3, 2026-10-26T17:45:41Z): gate, negative and
  run each exit 0 and "yes", outer standard error 0 bytes each.
- The dry directories d1, d2 and d3 (117006417, 117006593 and 117009544
  bytes by du -sb) are the maintainer's to reclaim, with P1's.

## After the independent check

The check (independent-check-1.md, a heading line then its answer
verbatim, sha256 acb217d0a6dda6e9e4819cc4967f9e2d25177e2501a1b189dfe72acc531b031c
for the answer alone) read this README as it stood before this section
(sha256 2a8fa9097ff2310967749c5e249ef2feabf051953543ff70e3013bac3659a24e);
nothing above it changed after the check. Its prompt is the brief file
`<phase>/check1-brief.txt` (sha256
173d69460e9c35ad32c77908086f8d351e0777f65a0e888a5b47a2b1993e025a) but for
that file's final newline. Its verdicts: 1, 2 and 3 yes; 4 and 5 yes with
one exception each. Its points, and what follows from each:

1. **Ruling V's hashes in the README** (4, exception): ruling V says the
   run log and the README print the assembly script's sha256. run.log
   does; this README did not. They are:
   `assemble-excel-record.py` sha256
   54f344a7588640dd79095340640b1fa5440974e5324c5022f60ea61cc701ad45, and
   the by-hand file `byhand-2026-10-26.tsv` sha256
   95a69b8a0912cbc4f5e6f90921f90bdeae919b49c8d4b9f264348f79973fde01, both
   under `<phase>/record-from-planning` and not committed.
2. **The FINDING-word lines** (5, exception): run.log holds the word on 16
   lines; the report lists them all (the printed script's comment lines
   116, 118, 152, 186, 191, 250, 259 and 614 and echo lines 1838, 1876,
   1882, 1934, 1955 and 1956, the heading 2283 and the count line 2284).
   Emitted FINDING lines: 0.
3. **RERUN_NOTE and RERUN_FILES** (5, imprecise): set for the gate alone
   (gate.log 4 to 5), naming attempt 1's gate.log and the gate script that
   wrote it; the negative and the run never started in attempt 1 and carry
   none.
4. **The record's findings 1 to 4 wording** (1, disagreement, planning's
   text): "whose host row and every other sample pass" is not literally
   true where a workbook carries two findings (the chart-surface sample
   repairs on all 27; 5 of them also carry an edit, protect or remove
   repair). The record is planning's, committed byte for byte (ruling W);
   the point goes to planning, nothing here changes.
5. **The record's line 39 parenthesis** (1, minor): "(the list holds
   them; they are not copied here)" refers to the references, not the
   seen values; planning's text, unchanged, to planning.

No code, log or record changed after the check; nothing was rerun.
