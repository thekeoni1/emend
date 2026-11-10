# M8 part 2 P3 logs

specs/0048-the-two-refusals.md, "Size", "Harness runs", "### P3: the
pivot cache source, the size record, the documents", Amendment 1
(decision 4 A) and planning's rulings E to K, with ruling L given by
planning's message of 2026-11-10 (planning writes it into the spec after
this record commit). The gate, the size log and the run took the working
spec at PLANNING_SPEC 37a8987d9558edd8863166359bc7a5876ff340136658e4809fe04ae900ffd155
(each log prints it). `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m8p2-p3`.

root: <home> stands for the home directory of the account the runs ran under
root: <phase> stands for <home>/.cache/emend-build/m8p2-p3, the phase's directory
root: <corpus> stands for the private corpus directory the run reads
root: <target> stands for <phase>/target, the phase's one cargo target root
root: <build> stands for <phase>/gate, as gate.log line 15 defines it
root: <work> stands for run.log's working directory, as run.log's root line 90 defines it
root: <scratch> stands for <phase>, as in the logs; independent check 1's materials are under <scratch>/check1

| Role | Commit | Subject |
|---|---|---|
| base | 572e71f | M8 part 2 P2 record: the logs, the README, the check |
| doc | a1a0ae0 | M8 part 2 P3: commit planning docs (the P2 verification and rulings) |
| test | ea6dc9f | M8 part 2 P3: the threaded part's external target and XML error have tests |
| code 1 | 5ab0e96 | M8 part 2 P3: remove_sheet refuses a sheet a pivot cache sources |
| code 2 (tools) | 6b13c52 | M8 part 2 P3: the removal sample names the class of a refusal that removes nothing |
| fix (Amendment 1, rulings F and I) | 37e800f | M8 part 2 P3: the pivot name cases move behind tables and metadata |
| documents (phase) | 8065795 | M8 part 2 P3: the CHANGELOG and the README name the part's changes |

No file name, sheet or table name, note or cell content of any private
workbook is in these logs or in this README. Workbooks are named by id,
the first 12 hex digits of the sha256. The texts quoted below are the
logs' own lines.

## The logs and the record

| File | Bytes | Lines | sha256 | Last line |
|---|---|---|---|---|
| gate.log | 970313 | 12509 | 3528ea39400c77d680af1627e69026df210d1962385293163fb4926f0503dd6c | gate log checks all as expected: yes |
| size.log | 981321 | 14327 | f65e0cef26f397cd68835439e3ae20b6c377af7f8316d07c8b3ad5b12a219358 | size log checks all as expected: yes |
| run.log | 237221 | 3298 | 3b72e757e10519dbc9b0b2592e4818e436a4cd7166466bcd0838655859fec482 | run log checks all as expected: yes |
| scan.log | | | | the final scan, written after independent check 1 and before the record commit, which holds it |

The size record is `records/size-m8p2-2026-11-10-8065795.md`: 41413 bytes, 424 lines, sha256
b4a386c35aec373e7342aea208aba35fc6c09f582228e10a057d4cc7e139124d. size.log wrote it and installed it ("## the size record,
installed", line 14303).

The scripts are <phase>/scripts/m8p2-p3-gate.sh, -size.sh, -run.sh and
-scan.sh. Each log prints the script that wrote it. Dry run d2, under
<phase>/dry/d2, ran the same four scripts; the logged driver checked
them cmp-equal before starting.

Each log's own start: gate.log line 393, "run at 2026-11-09T21:24:44Z";
size.log line 95, "run at 2026-11-10T17:43:58Z"; run.log line 77, "run
at 2026-11-10T17:49:50Z".

## The interruption and the size step's attempt 1 (ruling L)

- Attempt 1 ran the gate, then the size step, on 2026-11-09 under the
  driver now kept as <phase>/superseded/size-attempt-1/logged.sh
  (sha256 dfaf3db08e6a83b2113c3ddbaa16426474239a8e4f3d021f0d5efa0b2a55c3eb).
  The gate ended "gate log checks all as expected: yes". During the size
  step the host's disk filled and WSL stopped, at about 13:44 local time
  (21:44Z). This was an environment failure, not a failed check. size.log
  was cut off mid-line after 10710 lines; no size record was installed; the
  run and the scan had not started.
- Per ruling L, gate.log stands and was not rerun. Before attempt 2,
  every binary under <build>/bin was checked against the sha256 and byte
  count gate.log prints for it (lines 11139 to 11142, 11170 and 11172):
  6 named, 0 mismatched or missing. The new driver checks this again and
  prints the result to <phase>/logged/driver-2.out.
- Attempt 1's output was moved, not deleted, to
  <phase>/superseded/size-attempt-1/:
  - size.log, the truncated log: sha256
    6339d013ba93ee16dacd76362d0c4db16d5b514b2a33852cf891539b327a97b4, 749568 bytes;
  - size.outer.stderr, size.stderr and driver.out, each 0 bytes;
  - tmp.IiLrnwbJe1, the size script's scratch directory, whose exit trap
    never ran (206221070 bytes by du -sb);
  - a byte-identical copy of the size script that wrote the attempt, sha256
    e86178478f74c56a685a3d386abe7f5b4d95e6dfd31ece91f05109797cd52be7, because the size script's rerun
    check needs RERUN_FILES to name that script in the superseded
    directory.
- Attempt 2 ran the size script from its start under <phase>/logged.sh
  (sha256 42c1bad62584614779581e98a5a1b44221a00655defd2ee79101131a5d0d9dbe), attempt 1's driver
  with four numbered changes L1 to L4 in its header: the gate step skipped
  and checked, the entry state, RERUN_NOTE and RERUN_FILES on the size
  step alone, and line-buffered driver output. The build roots under
  <target> were kept and the build reused them without error.
- size.log prints the rerun note (lines 6 and 96) and hashes the six
  superseded files (lines 99 to 104). It prints "files named 6; not there
  0; outside the superseded directory 0" (line 105) and both rerun checks
  "yes" (lines 106 and 107).
- The two log hashes: attempt 1's size.log
  6339d013ba93ee16dacd76362d0c4db16d5b514b2a33852cf891539b327a97b4; attempt 2's size.log
  f65e0cef26f397cd68835439e3ae20b6c377af7f8316d07c8b3ad5b12a219358.
- Because attempt 2 ran on 2026-11-10 (UTC), the record is named for that
  date, as ruling K says ("with the run's UTC date"). Dry run d2's name
  carried 2026-11-09.

## The gate

- The prune (gate.log lines 191 to 246): P2's build target, 6691206691
  bytes (line 240), was removed by `rm -r "${PREV_DIR:?}/${n:?}"`,
  "target: rm -r exit 0" (line 242). The 26 other names were kept (line
  201). The driver printed "m8p2-p2's target: gone; its names: 26".
- The crate's tests: "the figures this run must give: off 544, ch 719, im
  581, ci 761, cc 768, cr 826, all 1244" (line 11491). Each set passed
  those figures on the host (lines 11497 to 11659) and on i686 (lines
  11686 to 11848), with 0 failed.

## The run and the expected change

The remove sample (--remove-sample) ran over the 39 workbooks with every
output opened. Each was compared per id on outcome, lo and lo_in with
records/m8p1-remove-2026-11-09-8816db0.txt.

- run.log line 3240: "summary --remove-sample: earlier 39 (same 38,
  changed 1, missing 0); new 0; the table's pairs met here 1; their lines
  met 1; other ids reading pivot_source 0; FINDING lines so far 0".
- run.log line 3264: "the pair of the table is met: remove 495c7b79b183
  ok/fail/ok to ok/ok/ok, its line removed 0 and removed_reason
  pivot_source, no other id reading pivot_source (WQ): yes". That is the
  change "Harness runs" names for P3, and nothing else changed.
- By removed_reason (line 3237): - 26, last 12, pivot_source 1.
- run.log cites the size log by hash (line 3270): sha256
  f65e0cef26f397cd68835439e3ae20b6c377af7f8316d07c8b3ad5b12a219358, the file above.
- The run's pair stays in <work>/records, hashed in run.log line 3232:
  m8p2-p3-remove-2026-11-10-8065795.md 6f92d068..., .txt 6f00c93f... No
  file is under records/ from the run.

## The size figures (size.log)

- Budget (line 11463): 456264, the format probe 989480 over probe-zip-xml
  533216; under the ceiling 458752, "yes"; the headroom left 2488
  (line 11462).
- This part's core movement: 4752 over M7 part 2's record's 451512
  (lines 11440 and 11441: the steps 0, 0 and 4752, the last this
  phase's code and fix commits).
- Costs (ruling G, probe minus format probe), each moved over M7 part 2's
  record: rowcol 18628 (5e, line 11489; raw 23380); comments 2948 (5b',
  line 11486; raw 7700); tables 2380 (5f, line 11490; raw 7132); metadata
  2268 (5d, line 11488; raw 7020). Printed, not judged: charts -336 (5a),
  images 168 (5b), read -352 (5c).
- 22 of the 28 probes other than the charts probe grew or shrank (line
  11604).

## FINDING lines

run.log: none ("FINDING lines printed above: 0", line 3282). gate.log's
two (lines 5769 and 5778) are inside the printed script's carried
history. size.log's findings list, lines 11611 to 11621, 11 lines, each
quoted with its line number:

- size.log 11611:   FINDING: probe-emend-package-xml: its growth over M7 part 2's record, -16 section bytes, shrank 16, a FINDING (expected 0 to 4752, accepted up to 6752)
- size.log 11612:   FINDING: probe-emend-cells-xml: its growth over M7 part 2's record, -16 section bytes, shrank 16, a FINDING (expected 0 to 4752, accepted up to 6752)
- size.log 11613:   FINDING: probe-emend-cells-write-xml: its growth over M7 part 2's record, -16 section bytes, shrank 16, a FINDING (expected 0 to 4752, accepted up to 6752)
- size.log 11614:   FINDING: probe-emend-styles-write-xml: its growth over M7 part 2's record, -16 section bytes, shrank 16, a FINDING (expected 0 to 4752, accepted up to 6752)
- size.log 11615:   FINDING: probe-emend-dates-write-xml: its growth over M7 part 2's record, -32 section bytes, shrank 32, a FINDING (expected 0 to 4752, accepted up to 6752)
- size.log 11616:   FINDING: probe-emend-formulas-write-xml: its growth over M7 part 2's record, -16 section bytes, shrank 16, a FINDING (expected 0 to 4752, accepted up to 6752)
- size.log 11617:   FINDING: probe-emend-rowcol-write-xml: the rowcol cost's growth over M7 part 2's record, 18628 section bytes, cost grew 18628, above the accepted 16000, a FINDING; the probe grew 23380, the core growth with the cost growth (expected 3000 to 12000, accepted up to 16000; specs/0048 "Size", Amendment 1 and ruling G)
- size.log 11618:   FINDING: probe-emend-richcomment-write-xml: its growth over M7 part 2's record, 7556 section bytes, grew 7556, above 6752, a FINDING (expected 0 to 4752, accepted up to 6752)
- size.log 11619:   FINDING: probe-emend-printnames-write-xml: its growth over M7 part 2's record, 7656 section bytes, grew 7656, above 6752, a FINDING (expected 0 to 4752, accepted up to 6752)
- size.log 11620:   FINDING: probe-emend-copycomments-write-xml: its growth over M7 part 2's record, 7700 section bytes, grew 7700, above 6752, a FINDING (expected 0 to 4752, accepted up to 6752)
- size.log 11621:   FINDING: this part's core movement, the core's growth, 4752 section bytes, grew 4752, above the accepted 4000, a FINDING (expected 300 to 2500, accepted up to 4000)

What each finding is (ruling K):
- The six shrinks (11611 to 11616) are probes that link the core without
  the format probe's calls. They stay FINDINGs, as "Size" says and as
  M7 part 2's record listed its 16-byte shrinks (ruling K (2)).
- This part's core movement, 4752 (11621), comes from 5ab0e96 and
  37e800f, the core's pivot-source check; Amendment 1 (decision 4 A)
  kept the ceiling and the budget is under it.
- The rowcol cost, 18628 (11617), is P1's FINDING (18644 at P1's check),
  attributed in the record.
- The three raw-growth probes stay FINDINGs because "Size" judges them in
  the "every other probe" band (ruling K (1)). Each one's growth is the
  core's 4752 plus a feature's cost growth from 5b' or 5d:
  - probe-emend-richcomment-write-xml (features comments and richtext,
    line 10647): grew 7556 (line 11586). The core's 4752 plus the
    comments cost's 2948 (5b', line 11486) is 7700; the probe is 144
    under that sum. The richtext probe, whose cost is not judged, grew
    4672, 80 under the core's growth (line 11585).
  - probe-emend-printnames-write-xml (features metadata and sheetprops,
    line 10649): grew 7656 (line 11588). The core's 4752 plus the
    metadata cost's 2268 (5d, line 11488) is 7020; the probe is 636 over
    that sum. The sheetprops probe, whose cost is not judged, grew 4832,
    80 over the core's growth (line 11587).
  - probe-emend-copycomments-write-xml (features copy and comments, line
    10652): grew 7700 (line 11591). That equals the core's 4752 plus the
    comments cost's 2948 (5b', line 11486). The copy probe grew 4776, 24
    over the core's growth (line 11590).

## Figures from files outside this directory

Each is named with its source; none is in a committed log:
- Attempt 2's driver, <phase>/logged.sh, wrote
  <phase>/logged/driver-2.out:
  - "size started 2026-11-10T17:43:57Z" and "run started
    2026-11-10T17:49:50Z";
  - "outer stderr 0 bytes" for each step; the size and the run scripts'
    own standard error 0 bytes each;
  - the gate binaries' check: "gate binaries named by gate.log: 6;
    mismatched or missing: 0; files under <phase>/gate/bin: 6";
  - "m8p2-p2's target: gone; its names: 26".
- The run's working directory is <phase>/tmp/tmp.VV0GNtYFQb, the one new
  name in <phase>/tmp, from the driver's before and after listings
  (<phase>/logged/phase-tmp-before-2.txt and -after-2.txt).
- Attempt 1's driver.out is empty: its output was lost in the crash.
  Attempt 2's driver line-buffers its output (change L4).

## A slip found reading the logs

size.log line 95 reads "phase: Milestone 8 part 2 P1
(specs/0048-the-two-refusals.md) size.log". The phase is P3. The label
was carried by the size generator. Dry run d2's size.log line 95 reads
the same, so it went unseen there too. The log's about line (line 3),
its record and its checks name the P3 phase commit 8065795. Nothing was
regenerated after the logged run.

## Leftovers (left in place for the maintainer's deletion list)

- <phase>/superseded/size-attempt-1, attempt 1's output (ruling L).
- <phase>/dry/d1 and d2 with their build roots.
- <phase>/dev-target, <phase>/measure, <phase>/devtmp, where the
  session built and measured.
- <phase>/tmp/G, R, S and Z, the forks' temporary files.
- <phase>/tmp/tmp.VV0GNtYFQb, the run's working directory.
- <phase>/target, this phase's target root. The next phase's gate prunes
  it.

## After independent check 1

The body above is kept as the check read it (sha256 1a1830f28ac13799aeb9b5c02bf4fc36adc6db2fc9b339ab3f5e67e4e3522f87), as in P1 and
P2. Each point the check raised, answered from the primaries:

- Question 1, the package module docs (a defect in the code range, for
  planning's word): crates/emend/src/package.rs line 40 at 8065795 reads
  "`TABLE` and `PIVOT_CACHE_DEFINITION` are public with no feature",
  added by 5ab0e96. 37e800f put `TABLE` back under its cfg (line 2599 is
  `pub const TABLE`, with the cfg on the lines before it) and did not
  change line 40. The sentence is plain text, not an intra-doc link, so
  the gate's doc builds pass. It is wrong for `TABLE` since 37e800f.
  This record commit changes no code; the fix is named in the report for
  planning (P4's docs-only commit is one place for it).
- Question 2, the pre-check scan: it ran as
  `m8p2-p3-scan.sh 8065795... pre-check` with PLANNING_SPEC alone, into
  <phase>/scan/pre-check.log (sha256 41c2321eb168d2c3b04bddd206fe39989d10f7c625786a890b0b82d6ff70d170,
  "RESULT: PASS", outer stderr 0 bytes). The word FINDING appears in it
  only inside its printed script (lines 4305 and 4330). The final scan is
  scan.log in this directory.
- Question 2, the record repeats the 11 FINDING lines: the size record's
  findings list is the same 11 lines, printed in size.log as the record
  is written (lines 14062 to 14072).
- Question 2, "Each one's growth is the core's 4752 plus a feature's
  cost growth": that sentence is too strong. Only copycomments equals
  the sum (7700). richcomment is 144 under it and printnames 636 over
  it, as the bullets say. The remainders sit with the features whose
  costs are not judged (richtext and sheetprops).
- Question 2, the scratch directory in RERUN_NOTE: the note names it as
  kept, and RERUN_FILES names files only, so it is not hashed in
  size.log. Its size, 206221070 bytes by du -sb, is in
  <phase>/logged/driver-2.out line 24.
- Question 2, the scan in the driver: P2's driver form runs the scan after
  the README (pre-check, then final), so the scan is not a driver step
  here. Planning to rule.
- Question 3, line citations: 989480 and 533216 are at size.log lines
  10314 and 10262 (line 11463 has 456264 against 458752); 451512 is at
  line 11483 (lines 11440 and 11441 are the steps and their sum).
- Question 3, figures from outside the directory:
  - "about 13:44 local time" is the superseded size.log's modification
    time, 2026-11-09 13:44:01 local, 2026-11-09 21:44:01 UTC
    (stat).
  - "cut off mid-line": the superseded size.log's last byte is not a
    newline (its last line, size.log line 10710, ends "commits
    touching its own sour").
  - "no size record was installed": attempt 1's log has 0 lines reading
    "## the size record, installed", and attempt 2's driver refused to
    start if any size-m8p2- record was under records/ (driver change L2).
  - "the run and the scan had not started": records/logs/m8p2-p3 held
    gate.log alone at attempt 2's start (driver change L2).
  - tmp.IiLrnwbJe1 and its bytes: driver-2.out line 24. The new
    logged.sh's sha256: driver-2.out line 6.
  - "the build reused them without error": size.log has 29 lines with
    "build exit 0" and none with a non-zero build exit.
  - "6 named, 0 mismatched or missing": driver-2.out.
  - The private-then-shared mount propagation is the session's
    `findmnt -no PROPAGATION /` before and after the maintainer's fix.
    No log prints it for attempt 2.
  - 17:43:57Z is the driver's "size started" line; size.log's own start is
    17:43:58Z (line 95).
  - "Identical to dry d2": dry d2's size.log
    (<phase>/dry/d2/logs/size.log, sha256 733c8028a4675644c97621b882247642449170173ff8f29cea8c946e6cb9fb4b, the
    figure ruling K cites) and this size.log have the same
    39 "judged", "printed" and 5a to 5f lines and the
    same 11 FINDING lines, compared with diff.
  - Dry d2's size.log line 95 reads "phase: Milestone 8 part 2 P1
    (specs/0048-the-two-refusals.md) size.log"; its record is
    <phase>/dry/d2/records/size-m8p2-2026-11-09-8065795.md.
