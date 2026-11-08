# M8 part 1 P1 logs

specs/0047-harness-findings.md, "The diagnosis record (P1)" and "### P1:
the diagnosis", with planning's rulings A and B. P1's doc commit bfcfb82
committed the spec at sha256
d966abd02fdeeafb1ff834858b214f23b758adcc304552bf1ca08923422c75a0 and
ROADMAP.md at sha256
da7ed1ac2421378c574b6b372bd7294ba9451a16f31a5d79df224e4632ddf6b1. Rulings
A and B, written during P1, are in the working specs/0047, uncommitted.
The gate took it at PLANNING_SPEC
eb859312f231d358c9ae403e8ab327b156bc47ea49264d712cd423a182fc7c3f (ruling
A); the rerun of the run step and the scan at
8bc1c7026f23c8d11f5de59fed6e76eeb9115b4aab49236d147a8d5023214c3e (ruling
B). `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m8p1-p1`.

root: <home> stands for the home directory of the account the runs ran under
root: <phase> stands for <home>/.cache/emend-build/m8p1-p1, the phase's directory
root: <corpus> stands for the private corpus directory the run reads
root: <runs> stands for <corpus>/runs, the corpus's run directories
root: <scratch> stands for the implementing session's scratch directory, which held independent check 1's materials
root: <work> stands for run.log's working directory, as run.log's own root line says

| Role | Commit | Subject |
|---|---|---|
| doc | bfcfb82 | M8 part 1 P1: commit planning docs (the spec and the ROADMAP blocks) |
| code (script) | 12a57fc | M8 part 1 P1: scripts/lo-tables.py counts a workbook's sheets and a conversion's tables |
| code (tool, phase) | 01b0896 | M8 part 1 P1: emend-copy keeps a failed read-back's output under EMEND_COPY_KEEP_OUTPUT=1 |

No file name, sheet or table name, note or cell content of any private
workbook is in these logs, the record or this README; workbooks are named
by id (the first 12 hex digits of the sha256). The texts quoted below are
emend-copy's own messages and the logs' own lines.

## The logs

| Log | Bytes | sha256 | Last line |
|---|---|---|---|
| gate.log | 878025 | d8503ba15a962b58475e81766884d1a90016ba08cad283f235a6092138e4db22 | gate log checks all as expected: yes |
| run.log | 242523 | 4770abadb0df37a63d4c7ed8e74adb7833e48bb3af9b8754d7cc8aee98957f09 | run log checks all as expected: yes |

scan.log, the final scan, is beside them and prints its own result.

The gate (gen/gate.py 20951cb0, script 43de576f) was written by the
logged driver, logged.sh, "run at 2026-11-08T03:05:50Z", ended 03:18:11Z,
with PRUNE_PREVIOUS=yes. Its prune removed the three target roots of
Milestone 7 part 3 by `rm -r "${PREV_DIR:?}/${n:?}"`, each printed "rm -r
exit 0" with every kept name checked still there: m7p3-p1/target
(6227787948 bytes by du -sb before), m7p3-p2/target (6227466908) and
m7p3-close/target (2146536849). Its template is part 3 P1's printed gate
(ruling A). The gate log holds no UNEXPECTED line and no FINDING line
outside its printed script, and its own standard error was 0 bytes.

run.log is the run step's rerun (gen/run.py 4a7c9081, script 16aa9688),
"run at 2026-11-08T03:22:13Z", ended 03:22:18Z; its rerun line names the
superseded attempt below. It holds no UNEXPECTED line and emits no
FINDING line; its own standard error was 0 bytes.

## The superseded run (ruling B)

The first logged run's run step (script 546cfe75), started by logged.sh
at 03:18:11Z, ended "run log checks all as expected: no" on one check
alone, change TK's "they are the five ids specs/0047 names, in that
order": the manifest gives the five google-sheets ids in another order
than the spec's list. Every other check of that log read yes. The cause
was the session's facts file, which called the spec's list the manifest's
order; the dry corpus wrote it in that order, so dry runs d1 and d2 could
not see it. Change TR compares the ids as a set, with a control that
refuses one id replaced, and the dry corpus now writes the manifest in
the real order. Dry run d3 ran the run step alone after the fix.

The rerun's driver, rerun.sh, moved the first attempt's three outputs by
`mv`, nothing removed, each printed before and after:

- `<runs>/2026-11-08-01b0896-sheet-kept` (23 files, 302161 bytes) to
  `<corpus>/superseded-runs/2026-11-08-01b0896-sheet-kept-run1`, the one
  write under `<corpus>` beyond the new run directory that ruling B
  allows;
- the first record, sha256
  7801689e7f489de6c05c9f3ea6471503fa9dcac6116fc539e984e10f11202066, 24333
  bytes, to `<phase>/superseded/run1/`;
- the first run.log, sha256
  1b9c723072bd06df0ae8e1d7df965f3880ed408ac3b2f7b21becdde5ca546c6c, 240530
  bytes, to `<phase>/superseded/run1/`.

The rerun then wrote `<runs>/2026-11-08-01b0896-sheet-kept` again and
the record below. The gate log stands as written.

## The run

- The self-test of scripts/lo-tables.py at the phase commit: 19 lines ok,
  0 FAIL, "lo-tables self-test: PASS".
- The five sheet-sample rebuilds, by scripts/passthrough-run.sh --ids
  with EMEND_COPY_KEEP_OUTPUT=1 and no conversion, into
  `<runs>/2026-11-08-01b0896-sheet-kept`: 15205770ae5c, 453a166dcdfe,
  6e0be3d0f2b1, 90a5b01f9541 and 91d2bdffa46a each read outcome error,
  exit 2, standard error "error: the output's active tab is not what the
  edits imply (output kept)", the output kept.
- scripts/lo-tables.py over part 3 P2's run directories under `<runs>`,
  each named 2026-11-05-d7a8196 and its sample, and the new one.
- The record, records/diagnosis-m8p1-2026-11-08-01b0896.md, sha256
  b44185521145d6a15a871c90d51c3a8029835517c59f92c0d8c47476819a1ef0, 24333
  bytes: statements against the corrected facts 0; the three groups'
  classifications supported. Nothing is fixed in P1.

## Leftovers under <phase>/tmp (ruling B: left in place)

Listed for the maintainer's deletion list, bytes by du -sb: tmp.dLnN1l8eiD
5466780 (dry run d1's run working directory); g 2609885, r 1196996 and s
1962816 (the generator forks' piece tests); d2check 49957 (the session's
check of the corrected dry-corpus builder before d2); trtest 192 (the
session's check of change TR's lines before d3); tmp.Fb4nfrUMUU 5475396
(the first logged run step's working directory); tmp.nGVjSR7RgH 5475513
(the rerun's working directory, the one run.log names).

## After independent check 1

The check (independent-check-1.md) answered yes with one exception, yes
with exceptions, and no. This section is added after it; the text above
is as the check read it.

- Q1, the keep acting for every sample's post-write checks: as ruling A
  accepts. No change.
- Q2 (a) and (b), and Q3 on the moves: the three moves' before and after
  lines (the run directory's 23 files and 302161 bytes, the first
  record's full sha256 7801689e7f489de6c05c9f3ea6471503fa9dcac6116fc539e984e10f11202066,
  the first run.log's) are printed by the rerun's driver, rerun.sh, whose
  output is kept, not committed, at <phase>/rerun-driver.out (sha256
  36eeb6295245f1e6de4fcf1640dd21bfcbcc5be75f781104278a43981616e8f4,
  2468 bytes, written with the home directory
  replaced). The first logged run's driver output is <phase>/logged-driver.out
  (sha256 ba23c3990b7db1275942a5d100c479718bc70c8037ad65900e2d20b066ab28c3). Ruling B lists
  the files of this commit, and neither driver output is among them; no
  committed log prints these figures. run.log prints the moves in its rerun
  note (lines 4 and 67) and hashes the superseded run.log (line 69).
- Q3, figures with no committed primary, each named here with where it is
  read from (all under <phase>, not committed):
  - d2's gate.log 54dcca67 and run.log 96009ca4, and d3's run.log 28ebf9b2:
    <phase>/dry/d2/logs and <phase>/dry/d3/logs.
  - gen/run.py 4a7c9081 and the run script 16aa9688: by sha256sum of
    <phase>/gen/run.py and <phase>/scripts/m8p1-p1-run.sh after change TR.
    gate.log line 175 prints gen/run.py as 8cf37324, the generator as it
    stood when the gate ran, before change TR; the gate does not read the
    run generator's output, so the gate stands (ruling B).
  - The superseded run script 546cfe75: <phase>/superseded/gen2.
  - The leftovers' sizes: by du -sb at the time this README was written.
    run.log names its working directory only as <work>, so which of the
    two working directories is the rerun's is the session's reading: the
    one made at the rerun's start.
  - The driver names logged.sh and rerun.sh: the files under <phase>.
