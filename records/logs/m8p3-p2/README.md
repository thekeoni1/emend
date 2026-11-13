# M8 part 3 P2 logs

specs/0049-excel-check-and-release.md, "The maintainer's check and the
record (P2)", "Phases" and "### P2: the record", with specs/0044's P2
sections and planning's rulings A to G, all committed in the phase
commit (specs/0049 at sha256
f3f18eb772f998298fb996769114eb9a956f0f1b1de75276a44fbf53ba3dab7c,
ROADMAP.md at sha256
1325fddfbfa5c842f0de60d18b87047a2ecb31ca8e93fbe7b0bb3fa63bb6bce5).
The working spec and ROADMAP.md equalled their blobs during the run, so
neither PLANNING_SPEC nor PLANNING_ROADMAP was given. Planning's rulings
1 to 8 on the dry run's stop report were given by message and are to be
written into specs/0049 as ruling H after this record commit; this README
names the ones that bear on these logs. `<phase>` is the phase's
directory, `<home>/.cache/emend-build/m8p3-p2`.

root: <home> stands for the home directory of the account the runs ran under
root: <phase> stands for <home>/.cache/emend-build/m8p3-p2, the phase's directory
root: <corpus> stands for the private corpus directory the run reads
root: <work> stands for run.log's working directory, as run.log's root line 67 defines it
root: <stage> stands for P1's stage, as run.log's root line 69 defines it
root: <scratch> stands for <phase>, as in the logs; the independent check's materials are under <scratch>/check1

| Role | Commit | Subject |
|---|---|---|
| base | c15561b | M8 part 3 P1 record: the eight records, the open record, the logs, the README, the check |
| doc (phase) | 69a3227 | M8 part 3 P2: commit planning docs (P1's verification and rulings A to G) |

There is no code commit in this phase: the doc commit is the phase
commit, and the code is P1's (4f8fdfc).

No file name, sheet or table name, note or cell content of any private
workbook is in these logs or in this README. Workbooks are named by id,
the first 12 hex digits of the sha256. No list or results row is
printed, and no message or seen cell of the results file. The texts
quoted below are the logs' own lines.

## The logs and the record

| File | Bytes | Lines | sha256 | Last line |
|---|---|---|---|---|
| gate.log | 938802 | 12262 | a8bcff0b5b050f4af30e99e423b007dc156aa7f0a581dc2fec1ca01deb7f4995 | gate log checks all as expected: yes |
| negative.log | 458415 | 6480 | 84329391d003f883dca2eb75db345224d0952bfc094771e08c89b6938d298fe8 | negative log checks all as expected: yes |
| run.log | 206794 | 2777 | 5457ac9d5f9298c6f27fd714979bafacaac5fa5cba3a4f0264ba08bb207c2ca5 | run log checks all as expected: yes |
| scan.log | | | | the final scan, written after the independent check and before the record commit, which holds it |

The record is records/excel-check-m8-2026-11-12.md, planning's,
667640 bytes, sha256
e58daaf825efca6493438aa3f80af729e1790503a20e387291a8b49f62ce1ab9,
written byte for byte by run.log's section 2: "the copy is byte-equal to
planning's file (cmp): yes" (line 2611). No other record is made.

The scripts are <phase>/scripts/m8p3-p2-gate.sh, -negative.sh, -run.sh
and -scan.sh. Each log prints the script that wrote it. Dry run d1, under
<phase>/dry/d1, ran the same four scripts; the logged driver,
<phase>/logged.sh, refuses to start unless each is cmp-equal to d1's.

Each log's own start and end: gate.log line 357 "run at
2026-11-13T00:21:11Z", line 12261 "run ended at 2026-11-13T00:33:59Z";
negative.log line 92 "run at 2026-11-13T00:33:59Z", line 6479 "run ended
at 2026-11-13T00:34:02Z"; run.log line 58 "run at 2026-11-13T00:34:02Z",
line 2776 "run ended at 2026-11-13T00:34:03Z".

## The gate

P1's gate with changes A106 to U106.
- The prune (gate.log lines 183 to 237): P1's build target alone,
  "their bytes in all: 6475256310" (line 229), removed by
  `rm -r "${PREV_DIR:?}/${n:?}"`, "target: rm -r exit 0" (line 231);
  "the kept names, 23" (line 193), keep-host among them, "every kept
  name is still there: as-expected" (line 233). The prune form's control
  ran on a directory the gate made (lines 185 to 190).
- The tree at the start and the older stages (change U106, lines 238 to
  256): the tree held "?? records/logs/m8p3-p2/" alone, no record; the
  eight stage and zip names (Milestone 2, Milestone 7 parts 1 and 2, P1)
  by stat, each there; at the end "the older stages and zips are as at
  the start: names, sizes and times equal (change U106): as-expected"
  (line 12232) and "M8 part 3 P1's keep, keep-host, is still there at the
  end" (line 12237).
- The crate's tests: "the figures this run must give: off 544, ch 719, im
  581, ci 761, cc 768, cr 826, all 1244" (line 11226), P1's figures, the
  phase adding no test.
- Ruling 5: the gate refuses an untracked record at its start and its
  end, so it ran before the run step and is not rerun after it.

## The negative

P1's negative as the frame (its run-script cases (c) and (d) not
carried) with Milestone 7 part 2 P2's record negative's assembly cases,
changes NA67 to NH67 (named in the stop report's question 2).
- The list script: LC (line 6336) and L1 to L3 (lines 6349, 6358, 6367).
- The assembly script assemble-excel-record-m8.py at its pin, on
  fictional fixtures: RC (line 6406), R1 and R2 (lines 6413, 6414) and R3
  (line 6417), the stop ruling E added.
- On scratch copies of the maintainer's results file and the by-hand
  file: AC, "the unmodified copy assembles to planning's record" (line
  6453); A1 and A2 (lines 6454, 6455); A3, "the by-hand file's header
  alone" (line 6458). Each STOP: line is printed with its row key cut.
- R3 and A3 are new cases for ruling E's stop; planning kept both
  (ruling 1).
- Line 6159, carried from P1, still says "... so case (c) is the run
  script's", though no case (c) is in this log. Planning ruled it stands
  as carried here and is reworded in the next negative that carries the
  frame (ruling 2).

## The run

Milestone 7 part 2 P2's record run with changes XM to XT.
- The inputs at their pins (lines 2557 to 2574): planning's record
  (2655 lines, 667640 bytes), the assembly script, the by-hand file (one
  row), the stage's list and results file, P1's zip, and the two earlier
  Excel records read at the phase commit,
  records/excel-check-m7-2026-10-26.md (line 2568) and
  records/excel-check-m7p2-2026-10-28.md (line 2569).
- The assembly run again, with the two earlier records oldest first, the
  machine section's three lines read from the record and both dates
  2026-11-12: "exit: 0; standard error: 0 bytes" (line 2616); its output
  sha256 e58daaf825efca6493438aa3f80af729e1790503a20e387291a8b49f62ce1ab9 (line
  2617), equal to the record.
- The record's rows in the list by bytes and sha256: "the record's corpus
  rows 312, found 312; fixture rows 2268, found 2268; together 2580,
  found 2580; the fixture rows' input pairs 2268, found 2268" (line 2637).
- The totals recomputed by awk from the results file with the by-hand
  cell applied: "| 4848 | 311 | 1 | 0 | 1905 | 0 | 363 | 1907 | 0 | 361 |"
  (line 2659), equal to the record's totals line (lines 2660, 2664).
- The acceptance (change XS): per row (lines 2705 to 2710), each of the
  six rows reads pass in the results file, after the by-hand file, in
  the record's six-row table and in its corpus row; "the six changed rows
  read pass, 6 of 6" (line 2713). In totals: "the acceptance in totals:
  the six changed rows pass 6 of 6; corpus (pass, repair, fail) 311 1 0;
  fixture pairs pass/pass 1883, refused/refused 339, refused/pass 22,
  pass/refused 24; the record's findings: None." (line 2741). Ruling 7:
  the comparison of the other corpus rows with the earlier records is
  covered by the byte-equal rerun alone, not recomputed.
- The stage's listing equal before and after (line 2745); the list, the
  results file and P1's zip still at their pins (lines 2746 to 2748); the
  eight stage and zip names as at the start (line 2758).

## FINDING lines

None. The only lines holding the word are run.log 2761 and 2762, its
section heading and "FINDING lines printed above: 0". The gate and the
negative print none. Dry run d1 printed none either.

## The scan

P1's scan with changes NY to OE. Change OC is ruling D's correction:
P1's scan.log line 6716 said "The two sibling logs are always present:
... and no test, size or negative log"; this scan prints that there are
three sibling logs, gate, negative and run. Change OE gives Milestone 7
part 1's close-out doc commit its own variable: P1's scan stored it in
the variable that already held part 2's close-out doc commit, so P1's
control labelled "refuses part 2's close-out doc commit's paths" tested
Milestone 7 part 1's. Both commits are refused, so no result changed.
Planning ruled that P1's committed line stands as printed (ruling 3).

## Figures from files outside this directory

Each is named with its source; none is in a committed log:
- <phase>/logged.sh (sha256 f36e718582fa9fb25888d0967029883cfe55ea9545e0fcbf7f03a7b68e09cd59) wrote
  <phase>/logged/driver.out:
  - "gate started 2026-11-13T00:21:09Z", "negative started
    2026-11-13T00:33:59Z", "run started 2026-11-13T00:34:02Z";
  - "outer stderr 0 bytes" for each step; the gate's and the run's own
    standard error 0 bytes each;
  - "m8p3-p1's target after the gate: gone; its names: 23; keep-host:
    present";
  - "free space 720G" at the start;
  - "records/excel-check-m8-2026-11-12.md: sha256
    e58daaf825efca6493438aa3f80af729e1790503a20e387291a8b49f62ce1ab9".
- The run's working directory is <phase>/tmp/tmp.P497hRVbkh, the one new
  name in <phase>/tmp (the driver's before and after listings), so the
  driver prints "<phase>/tmp names unchanged: no".
- Before the logged run, <phase>/scripts/out (dry d1's scan pre-check
  stderr, 0 bytes) was moved to <phase>/superseded/dry-d1-scripts-out, so
  the logged run made its own.
- Dry run d1's scan pre-check read RESULT: FAIL on three lines only a
  dry layout produces (its scan.log lines 95, 6797 and 6916: the stand-in
  directories, the gate run with list, and 27 lines holding the dry
  directories' paths), the same three P1's dry run printed.

## Leftovers (left in place for the maintainer's deletion list)

- <phase>/dry/d1 with its build roots (6597437998 bytes by du -sb).
- <phase>/superseded-R, an empty directory a generator fork made by a
  stray mkdir outside the directories forks write; nothing removed
  (ruling 8).
- <phase>/tmp/G, N, R and S, the forks' temporary files;
  <phase>/tmp/dev, the session's dev trial of the assembly (its output
  compared by cmp alone); <phase>/tmp/tmp.P497hRVbkh, the run's working
  directory.
- <phase>/target, <phase>/gate and <phase>/negative, this phase's build
  roots; the next phase's gate prunes the target.
- <home>/.cache/emend-build/m8p3-p1/keep-host, P1's keep, read by nothing
  in this phase.
- The stages and zips under <corpus>/work, the maintainer's.

## After the independent check

The body above is kept as the check read it (sha256 0519e1e803648553d5b685c7ebe6c0da27cd1bfab636943b05d7281df3f730bd).
independent-check-1.md holds the answer verbatim: question 1 yes;
question 2 no "on a strict reading"; question 3 no, for the points
below. It found no private content. Its points, answered from the
primaries:

- Question 2, the restored Excel-record check's two cases (specs/0044
  "### P2: the record", "the restored check's two cases"): this
  negative does not run them. Milestone 7 part 2 P2's record negative,
  the template, did not run them either (no such case in
  records/logs/m7p2-p2/record-negative.log at 16c083f), and specs/0049
  "The maintainer's check and the record (P2)" gives the
  `records/excel-check-m8-*.md` check to the close-out by a numbered
  change. No ruling in specs/0049 or among planning's rulings 1 to 8
  names it for P2; the report raises it for planning.
- "The inputs at their pins (lines 2557 to 2574)": P1's zip is at
  run.log lines 2575 and 2576, outside that range; the inputs run from
  line 2557 to line 2576.
- The quote of P1's scan.log: the sentence is at lines 6715 and 6716 of
  records/logs/m8p3-p1/scan.log at c15561b ("The two sibling logs are
  always present: this phase writes the gate log (the" and "logged gate,
  PRUNE_PREVIOUS yes; change NB) and the run log, and no test, size or
  negative log: ..."), a committed log the check was not given. Ruling D
  names its line as 6716.
- The root line for <scratch>: it holds for gate.log and run.log; in
  negative.log, <scratch>/tmp and <scratch>/w stand for what that log's
  own root lines 104 and 105 say (<build>/tmp and its working directory).
- The figures under "Figures from files outside this directory" and
  "Leftovers", and "the stop report's question 2", come from files not
  given to the check, none committed: <phase>/logged/phase-tmp-before.txt
  and -after.txt (the name tmp.P497hRVbkh); the session's mv of
  <phase>/scripts/out to <phase>/superseded/dry-d1-scripts-out;
  <phase>/dry/d1/scan.log (lines 95, 6797 and 6916) and
  <phase>/dry-d1.out (FINDING lines); du -sb of <phase>/dry/d1 run by the
  session before this README; <phase>/report-stop.md (sha256
  e21f5091378d9585aedac76904b79e55ef6db71bff94a97935e9717b55f9c6d7),
  its question 2. superseded-R is named in planning's ruling 8.
