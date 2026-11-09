# M8 part 1 P2 logs

specs/0047-harness-findings.md, "The fixes (P2)", "The reruns (P2)" and
"### P2: the fixes and the reruns", with planning's rulings C to G. P2's
doc commit 4296d03 committed the spec at sha256
03144cde646b966b187794d6858e0eaa36e2546702d94d47f54ddb4ba97ed13e and
ROADMAP.md at sha256
50daf0436324cb04598045889799fb48bacc75df73105b6470aa20deb7d542bd. Rulings
E, F and G, written during P2, are in the working specs/0047, uncommitted;
the gate, the run and the scans took it at PLANNING_SPEC
5c56f5adc3e1ea53fd36c708fd3e882d779fbe6a1e3d4b0a586de8853f5250c0.
`<phase>` is the phase's directory, `<home>/.cache/emend-build/m8p1-p2`.

root: <home> stands for the home directory of the account the runs ran under
root: <phase> stands for <home>/.cache/emend-build/m8p1-p2, the phase's directory (in records/logs/m8p1-p1/ it is m8p1-p1's, as that README says)
root: <corpus> stands for the private corpus directory the run reads
root: <runs> stands for <corpus>/runs, the corpus's run directories
root: <scratch> stands for the implementing session's scratch directory, which held independent check 1's materials
root: <work> stands for run.log's working directory, as run.log's own root line says

| Role | Commit | Subject |
|---|---|---|
| doc | 4296d03 | M8 part 1 P2: commit planning docs (the P1 verification and rulings) |
| code a (count rule) | 182482a | M8 part 1 P2: the open check tells LibreOffice's external-link cache tables from sheets |
| code b (read-back) | bbcb69a | M8 part 1 P2: the sheet sample reads the active tab against the input's bookViews |
| code c (documents) | 67310df | M8 part 1 P2: the README cites the size figure at the tag and the CHANGELOG says so |
| fix (phase) | 8816db0 | M8 part 1 P2: the open check lets an external sheet name stand without a cache table |

No file name, sheet or table name, note or cell content of any private
workbook is in these logs, the records or this README; workbooks are
named by id (the first 12 hex digits of the sha256). The texts quoted
below are the logs' own lines.

## The logs

| Log | Bytes | sha256 | Last line |
|---|---|---|---|
| gate-prune.log | 906338 | 2aad3fff4154cfbee17a59b808acacaf60e5c51d5c2ce000d9f91ebc6c75d8e1 | gate log checks all as expected: yes |
| gate.log | 920327 | 8066a4c5c25629a566689ef0033d3b1bc7483d043f192c44347f81af8a3be4d6 | gate log checks all as expected: yes |
| run.log | 491135 | d275146e90c44b45bb4742e64d0d5d29fe50f22f02808104e8bd27aab042cd1a | run log checks all as expected: yes |
| moves.out | 14663 | cb0a6b4d5e66365fe5efaa3bbd5185c4a9e4da4944eafcbe937e8dde18261a70 | the moves' output (below) |

scan.log, the final scan, and independent-check-1.md are beside them.

gate-prune.log is the first logged gate, at code c 67310df, "run at
2026-11-08T20:09:45Z", with PRUNE_PREVIOUS=yes: its prune found P1's
build target `<home>/.cache/emend-build/m8p1-p1/target` (6251746515
bytes by du -sb), removed it by `rm -r "${PREV_DIR:?}/${n:?}"` with the
control of specs/0044 ruling AD, and kept P1's 28 other names (lines 189
to 235). It stands as the prune's primary (ruling F) and is otherwise a
gate of the superseded attempt below.

gate.log is the third logged gate, at the phase commit 8816db0 (script
m8p1-p2-gate.sh 6cfbfcc2, from gen/gate.py ca974255), "run at
2026-11-09T01:40:58Z", with PRUNE_PREVIOUS=list: "names of the prune set
there (M8 part 1 P1's directory): 0 of 1; their bytes in all: 0", "nothing
of the prune set is there: nothing to remove (change M100)", P1's 28 names
unchanged (lines 189 to 232). Its rerun line (366) names the second gate's
attempt, gate2 below, and its line 372 reads "RERUN_FILES names the
superseded script that wrote the attempt, a file ending .sh in
<phase>/superseded (ruling F; change W71): as-expected". It holds no
UNEXPECTED line; its own standard error was 0 bytes.

run.log is the run (script m8p1-p2-run.sh 4de98728, from gen/run.py
670316b3), "run at 2026-11-09T01:48:18Z", its 59 runs from 01:48:20Z to
06:29Z by the driver's line. Its RERUN_NOTE (lines 4 and 74) names the
first run's run.log b0ae5da8, its FINDING at line 3092 and the fix commit
8816db0; its RERUN_FILES are `<phase>/superseded/run1/run.log` and
`<phase>/superseded/run1/logged.sh` (line 5), both there (line 81). It
holds no UNEXPECTED line and no FINDING line ("FINDING lines printed
above: 0", line 7123); its own standard error was 0 bytes; "every record
file was installed, byte for byte: yes".

The two steps were run by the third logged driver, `<phase>/logged3.sh`
(sha256 2bf6d5999f385b76894af4d3fbadca15f6f9ee0be8bb5d66acdb23ad48c61d85),
each with its own RERUN_NOTE and RERUN_FILES through `env` on that step
alone. Its output, `<phase>/logged3/driver.out` (sha256
33c4d48904fa81d2dc654df28d60c38d76050fbd7a16de6ab4e88c8baa426124, kept,
not committed), reads "gate exit 0; gate log checks all as expected: yes;
outer stderr 0 bytes", "run exit 0; run log checks all as expected: yes;
outer stderr 0 bytes", "UNEXPECTED lines:" 0 and 0, and "records written:
59 .md, 59 .txt".

## The records and the expected changes

59 record pairs, `records/m8p1-<name>-2026-11-09-8816db0.md` and `.txt`:
the passthrough and the 58 flags ending in `-sample` that the usage text
names, less `--new-sample`. run.log section 5 prints each installed
file's sha256 and bytes. Each record is compared per id on (outcome, lo,
lo_in) with part 3 P2's `records/m7p3-<name>-2026-11-05-d7a8196.md`.

The expected-change table (run.log lines 7054 to 7120):

| Expected change | Samples | Ids | Read |
|---|---|---|---|
| group 1, decision 1 A: lo_in fail to ok; lo fail to ok where it failed; outcome unchanged | all 59 runs | 9956034c4326 | met in 59 runs of 59; lo_in fail to ok in 59; lo failed in the earlier record in 58 and went fail to ok in 58: yes |
| decision 3 A: outcome error to ok, lo ok | --sheet-sample | 91d2bdffa46a, 90a5b01f9541, 453a166dcdfe, 15205770ae5c, 6e0be3d0f2b1 | met 5: yes |
| unchanged, not this part's findings (change TX) | delete-rows, delete-columns, comment, richcomment, commentbox, copy, remove | 90a5b01f9541; 91d2bdffa46a; 15205770ae5c (four); 495c7b79b183 | unchanged 7: yes |

The seven pairs are read from records/logs/m7p3-p2/README.md's sections
3, 4 and 5 (run.log lines 2941 to 2947): group 3's two formula-cut
refusals and group 4's four threaded-comment refusals error/-/ok, group
5's remove line ok/fail/ok. Each per-sample summary reads "changed 1"
(group 1's id) except --sheet-sample's "changed 6" (group 1's and the
five google-sheets ids).

FINDING lines: none. The logged run printed 0; the driver's grep for the
word finds it only in the rerun notes and the summaries' "FINDING lines
so far 0".

## The superseded runs

### run1: the first logged run, at code c 67310df (ruling F)

The first logged driver, `<phase>/logged.sh` (sha256
f1bfae6d91e35d197c459dc90e4f2beac3406b08147b1b3a55d981a79b964e40, copied
to `<phase>/superseded/run1/logged.sh`), ran gate-prune.log above, then
the run: run.log sha256
b0ae5da8225ff6541623eba8e9e80b943b2f2bc35c403caf4ef7762303f6f71c (7200
lines), 59 runs and 59 record pairs, 0 UNEXPECTED, ending "run log checks
all as expected: no" on one check, group 1 met in 58 runs of 59. Its one
FINDING, line 3092: --edit-formula-sample, id 9956034c4326, outcome/lo/lo_in
ok/ok/fail in part 3 P2's record, ok/fail/ok then, with the reason "the
ods holds 1 table elements, 0 of them external-link caches, and the
workbook 1 sheet elements and 1 external sheet names".

The cause is the rule's edge (below): the sample sets the first
Normal-formula cell to a number, and in that workbook it was the one
formula that used the external sheet, so the output keeps an external
sheet name no formula uses and LibreOffice writes no cache table for it.
The fix is commit 8816db0 (ruling F), whose message names run.log
b0ae5da8 and its line 3092. The run was made again with the fix named in
its RERUN_NOTE; this directory's run.log is that run.

### gate2: the second logged gate, at the phase commit 8816db0 (ruling G)

The second logged driver, `<phase>/logged2.sh` (sha256
bf38e113a657c6a1e681be9c576aca67308ae8f94bdbd567ba23c404c649c7b9, copied
to `<phase>/superseded/gate2/logged2.sh`), ran the gate alone: gate.log
sha256 e872e83d901ddece4fa35ffc3633a9009783863a565bc13b2da0a5b89c7110ad
(11933 lines), now `<phase>/superseded/gate2/gate.log`, exit 1, "gate log
checks all as expected: no" on one UNEXPECTED line, its line 371: "RERUN_FILES
names the superseded script that wrote the attempt, a file ending .sh in
<phase>/superseded (ruling F; change W71): UNEXPECTED". The cause was the
driver's: it exported RERUN_NOTE and RERUN_FILES, meant for the run
step, to the whole driver, so the gate received them, and RERUN_FILES
named the run1 run.log alone. Every other gate check read as expected;
no run directory or record was made. The fix (ruling G): that gate.log,
`<phase>/gate` and `<phase>/out` moved to `<phase>/superseded/gate2/` by
`mv`, hashed (`<phase>/superseded/gate2/moves-gate2.out`, kept, not
committed); the third driver gives each step its own inputs through
`env` and exports nothing driver-wide. gate.log above names this attempt
in its rerun line.

## The moves (ruling F item 2; ruling G)

moves.out is the output of `<phase>/moves.sh` (sha256
b90e64bae6aed292386167af659ce81aa227e7900342a5f8f36598973ccf4077), run
once at the fix commit, before dry d3. Each by `mv`, each printed:

- the first run's 118 record files (`records/m8p1-*-2026-11-08-67310df.md`
  and `.txt`) to `<phase>/superseded/run1/records/`, and its run.log
  b0ae5da8 to `<phase>/superseded/run1/run.log`, each file's sha256
  printed before;
- its 59 run directories `<runs>/2026-11-08-67310df-*` to
  `<corpus>/superseded-runs/`, their 32309 files listed with sha256 in
  `<phase>/logged2/rundirs.sha256` (sha256 printed in moves.out);
- the first gate's `<phase>/gate`, `<phase>/out` and `<phase>/scripts/out`
  to `<phase>/superseded/run1/`;
- gate.log 2aad3fff renamed gate-prune.log, unchanged.

After the moves moves.out reads that the files and the run directories'
files match their sha256 at the new place, and that no first-run record
or run directory is left. The departure, accepted by ruling G: ruling F
had the logged driver make the moves; they ran as their own logged script
before d3, because the run script admits only records at the phase
commit's short sha, so the 118 first-run records would have failed d3 on
layout.

## The rule's edge (ruling E, ruling F)

Ruling E: "LibreOffice writes a cache table only for an external sheet a
formula references, so a workbook with an externalLink sheet name no
formula uses fails the rule (caches fewer than names)." Ruling E kept the
decided equality and said that "a corpus file showing the edge makes a
FINDING that reopens it with a real fixture". The first run's FINDING on
9956034c4326 was that corpus file.

Ruling F's change, in commit 8816db0: `lo_count_check`'s third
comparison is that the caches do not exceed the external sheet names
(`-gt` where it read `-ne`). A cache with no name still fails. A name
with no cache does not. The reason's wording is unchanged, and the
comment says why. `scripts/lo-check-test.sh` gains case (vi): one sheet,
one external sheet name, one displayed table and no cache, ok. Case (iii)
still fails. The dry corpus's fictional external workbook
(gen/make-dry-corpus.py 2c5de985) carries a second external sheet name
that no formula uses. Dry d3 (`<phase>/dry-d3.out` 7929c82b) read as d2
did.

## P1's driver outputs (rulings C and D)

Copied unchanged from `<home>/.cache/emend-build/m8p1-p1/` into
records/logs/m8p1-p1/, the moves' primary for P1 (their `<phase>` is
m8p1-p1's):

| File | Bytes | sha256 |
|---|---|---|
| records/logs/m8p1-p1/rerun-driver.out | 2468 | 36eeb6295245f1e6de4fcf1640dd21bfcbcc5be75f781104278a43981616e8f4 |
| records/logs/m8p1-p1/logged-driver.out | 1298 | ba23c3990b7db1275942a5d100c479718bc70c8037ad65900e2d20b066ab28c3 |

## Leftovers (left in place for the maintainer's deletion list)

Under `<phase>/tmp`, bytes by du -sb:
- tmp.xL4AeFD8GJ, 29552724: this run's `<work>`, kept by design;
- tmp.XQ2hAd9Chs, 29552736: the first logged run's `<work>`;
- G, 4646960; R, 1273383; S, 4595934: the generator forks' piece tests;
- dc2, 20173, and lo2, 9453: the session's checks of the corrected
  dry-corpus builder and of its fictional 9956034c4326's conversion,
  before d2;
- session-f, 29862: the session's conversion of ruling F's fictional
  workbook, before d3;
- tmp.uDLoJr2BMA, 23: a mktemp directory of the session's.

Also left: `<phase>/dry`, `<phase>/dev-target`, `<phase>/devtmp`,
`<phase>/target`, `<phase>/superseded` and `<corpus>/superseded-runs`'s 59
first-run directories.

## Deviations

- The moves ran as their own logged script before d3, not inside the
  logged driver (ruling G accepts this).
- Commit b's doctoring negative case is a unit test in the tool's main.rs
  (ruling E accepts this, with the other deviations of the first stop
  report).
- The scan gains change MW: moves.out is checked at its pinned sha256 and
  read by the scan's privacy, home-path and ASCII forms, with a control.

## After independent check 1

The check (independent-check-1.md) answered yes with exceptions, yes
with exceptions, and no. This section was added after the check; the
text above is as the check read it.

- Q1 a, the fix commit's message says "ruling F, option A": option A is
  the first of the options in the stop report after the first run, which
  ruling F took; the ruling's text does not name the options. The message
  stands as committed.
- Q1 b, "The fixes (P2)" keeps "a negative case where the cache count
  and the external sheet names differ": that paragraph is planning's,
  and the session does not edit it. Under ruling F, case (iii) (a cache
  with no name) is that negative case.
- Q1 c and Q2 a: accepted by rulings E and G.
- Q2 a, the moved files' hashes: the 32309 run-directory files' sha256
  are in `<phase>/logged2/rundirs.sha256` (sha256 a016c0a7, printed in
  moves.out), and the 118 record files' and run.log's are in
  `<phase>/logged2/files-before.sha256`. Both are kept but not committed.
  `<phase>/gate`, `<phase>/out` and `<phase>/scripts/out` hold the first
  gate's build copies and standard error, moved whole without per-file
  hashes.
- Q2 b and Q3 k: gate2's moves are printed with their hashes in
  `<phase>/superseded/gate2/moves-gate2.out`, which is kept but not
  committed. Ruling G's list of the record commit's logs does not include
  it.
- Q3 a: the privacy run over the range is logged in
  `<phase>/privacy-range.out`, rerun after the check ("privacy: 5
  commit(s), 2796 blob(s) scanned, 0 finding(s)", "PASS privacy"; sha256
  7e0271129dd62830ab4040e087801e08885f93ec151c26bcc2f5196103121897). The
  run after the fix commit was the session's own and is not logged.
- Q3 b, d, e, f: the hashes of logged3.sh, the gate and run scripts
  (6cfbfcc2, 4de98728) and moves.sh are from sha256sum of the files under
  `<phase>` (`scripts/m8p1-p2-gate.sh`, `scripts/m8p1-p2-run.sh`,
  `logged3.sh`, `moves.sh`). No committed log prints them.
- Q3 c: the previous scan script is
  `<phase>/superseded/gen-scan-3/`'s output (f818982b). The session read
  the difference by diff; it is not logged.
- Q3 g and m: `<phase>/dry-d3.out` (7929c82b) is kept but not committed.
  The second external sheet name is in gen/make-dry-corpus.py, at its
  comment on line 57 and the externalLink part's sheetNames on lines 89
  and 90.
- Q3 h and j: the second driver's output, `<phase>/logged2/driver.out`
  (sha256 39cc878e0b3fb2540d2c6c88ad45a3ab2317207cb391a552f1d67425450435df),
  reads "gate exit 1; gate log checks all as expected: no; outer stderr 0
  bytes" and "the gate did not pass; the run is not made".
- Q3 i: the reason is in the first run's `lo-reason.host.txt` for
  9956034c4326 under
  `<corpus>/superseded-runs/2026-11-08-67310df-edit-formula/`. It is
  emend's own message, and it is not in the run.log.
- Q3 l: the leftovers' names and sizes come from ls and du -sb under
  `<phase>/tmp` when this README was written.
- Q3 n and o: the kept-name lines are gate-prune.log line 236 and gate.log
  line 233, so the ranges above should read 189 to 236 and 189 to 233.
- Q3 p: moves.out's last line is its git status; the table's entry for it
  describes the file rather than quoting a line.
- Q3 q: 01:48:20Z is run.log line 2956, the first run's start. The driver
  reads 01:48:18Z and 06:29:29Z.
