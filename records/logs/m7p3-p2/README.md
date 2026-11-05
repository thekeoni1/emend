# M7 part 3 P2 logs

specs/0046-corpus-maintainer-items.md, "The runs over the grown corpus
(P2)" and "### P2: the runs over the grown corpus", with planning's
rulings A to O. P2's doc commit d7a8196 committed the spec at sha256
e41e5b4531f02a7cc1f632a6e8eba2f26b42b71539491f9703969d6800e5fe3a and
ROADMAP.md at sha256
6de20108fc48e0399bd6593275c5f0db184da11fd880d33be7e376763c399ac2; it is
also the phase commit, d7a81968f39ffccd6b4ce6ebb0afc16fd97af582 (no code
commit). Rulings M to O, written during P2, are in the working
specs/0046, uncommitted. The gate and the run took it at PLANNING_SPEC
061dc48bf19031d72111932f99ed1723740da4a43ffce13f68551df62b6f6bc1 (ruling
N); the scan took it at
4fc4371f241a79ffac352921d9ca13425fdbe5f2062d680f5cfd7a40046515d6 (ruling
O). `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m7p3-p2`.

root: <dry> stands for <phase>/dry, the phase's dry runs' directory (specs/0043 ruling Q)
root: <superseded> stands for <phase>/superseded, the phase's superseded directory
root: <runs> stands for the corpus's runs directory, which holds the 59 run directories
root: <scratch> stands for the implementing session's scratch directory, which held independent check 1's materials

| Role | Commit | Subject |
|---|---|---|
| doc (phase) | d7a8196 | M7 part 3 P2: commit planning docs (the P1 verification and rulings) |

No file name, note or cell content of any private workbook is in these
logs, the records or this README; workbooks are named by id (the first
12 hex digits of the sha256), producer and tags alone. The texts quoted
below are emend-copy's own messages and the open check's own reasons.

## The logs

| Log | Bytes | sha256 | Last line |
|---|---|---|---|
| gate.log | 835285 | 6a3e7ed3420bfce8e6533337956d089b5a5fb59896a54d2dcaeaaf0e5ac12638 | gate log checks all as expected: yes |
| run.log | 477964 | 4e6337566cccdb0e16dea7c970cff93905d3199c5351ac6f6dfdcecefc3a0433 | run log checks all as expected: yes |

scan.log, the final scan, is beside them and prints its own result.

Both logs were written by one driver run, 2026-11-05, logged.sh
a19d89ce: the gate (gen/gate.py 6e5b325f, script 46fa607b), "run at
2026-11-05T15:39:51Z", ended 15:52:04Z, with PRUNE_PREVIOUS=list and an
empty prune set; then the run (gen/run.py 8fdb538c, script 352f55f1),
"run at 2026-11-05T15:52:04Z", ended 20:08:42Z. Each log prints the
script that wrote it. Every log's own standard error was 0 bytes, as
each log's last check prints. In run.log each of the 59 runs reads "its
stderr is empty: yes", and 118 lines read "stderr bytes: 0": 59 for
the runner scripts and 59 for the record scripts. The driver's captures
of the outer standard error (in <phase>/logged, not committed) were 0
bytes too; no log prints them. The gate log holds no UNEXPECTED line
and no FINDING line outside its printed script.

## The runs

59 runs at the phase commit on the host, over the 39-file corpus: the
passthrough, then the 58 flags ending in `-sample` that emend-copy's
usage text names, less `--new-sample`, in the usage text's order, each
into `<runs>/2026-11-05-d7a8196-<name>` and recorded as
`records/m7p3-<name>-2026-11-05-d7a8196.md` and `.txt` (59 pairs). The
open check opened in localhost/emend-lo:25.2.3-2-deb13u6 every input
(lo_in) and every output a run wrote (lo; "-" where emend-copy wrote
none).

Outcomes, each run's summary.host.tsv at 39 rows: 51 runs ok 39;
`--sheet-sample` error 5 ok 34; `--printarea-sample` error 6 ok 33;
`--delete-rows-sample`, `--delete-columns-sample`, `--comment-sample`,
`--richcomment-sample`, `--commentbox-sample` and `--copy-sample` each
error 1 ok 38. The six printarea errors are on earlier ids and equal
their earlier record, so they are no FINDING.

Each record was compared per id on (outcome, lo, lo_in) with the
sample's latest committed record (the passthrough with
records/m6p6-roundtrip-2026-10-24-d88f735.md, ruling M). Every one of
the 59 comparisons reads "earlier 27 (same 27, changed 0, missing 0)".
run.log prints 71 FINDING lines, all on new ids (7 of the 12), all recorded
and none fixed (Milestone 7 decision 3; ruling O).

## The findings

The new ids, their producer and tags as the manifest gives them:

| Id | Producer | Tags | FINDING lines |
|---|---|---|---|
| 9956034c4326 | excel-win | plain,strings,formulas,external | 59 |
| 15205770ae5c | google-sheets | plain,strings,formulas,freeze,comments | 5 |
| 90a5b01f9541 | google-sheets | plain,strings,formulas,hyperlinks | 2 |
| 91d2bdffa46a | google-sheets | plain,strings,formulas,freeze | 2 |
| 453a166dcdfe | google-sheets | plain,strings,formulas,freeze,images | 1 |
| 6e0be3d0f2b1 | google-sheets | plain,strings,formulas,hyperlinks,freeze | 1 |
| 495c7b79b183 | excel-win | plain,strings,names,pivots,slicers,timelines | 1 |
| 4bd861696651 | excel-win | plain,strings,macros | 0 |
| 7055450ada76 | excel-win | plain,comments | 0 |
| 0d7a34c16e9d | excel-win | plain,strings,sparklines | 0 |
| 9de2ee20fc6a | excel-win | plain,strings,names,charts | 0 |
| 2adafe76df11 | excel-win | plain | 0 |

The tables below give each FINDING line's sample, id, the summary's
outcome words and the run.log line. A quoted error is the id's
stderr.host.txt in that run directory; a quoted reason is its
lo-reason.host.txt (the output) or lo-in-reason.host.txt (the input).
Those files are in the private run directories under <runs>, outside
the repository and not committed, and no committed log or record holds
the quoted texts; the implementing session read them with a script
whose output independent check 1 took as their primary, and planning
read the same files (ruling O).

### 1. The open check's sheet count on 9956034c4326 (59 lines)

The open check fails on the input itself in all 59 runs (lo_in) and on
the output in 58 (lo); lo is ok for `--edit-formula-sample` alone. The
input fails the same count rule as the output, so the finding is on the
open check's sheet-count rule against this workbook's ods conversion,
not on an edit (ruling O). The sheet, copy and chartsheet samples add a
sheet, so their output reason counts one more of each.

| Sample | Id | Outcome | lo | lo_in | lo reason | lo_in reason | run.log line |
|---|---|---|---|---|---|---|---|
| passthrough | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 2589 |
| --edit-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 2667 |
| --edit-formula-sample | 9956034c4326 | ok | ok | fail | - | "the ods holds 2 table elements and the workbook 1 sheet elements" | 2745 |
| --style-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 2823 |
| --date-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 2901 |
| --formula-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 2979 |
| --sheet-sample | 9956034c4326 | ok | fail | fail | "the ods holds 3 table elements and the workbook 2 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3062 |
| --remove-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3141 |
| --font-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3219 |
| --border-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3297 |
| --gradient-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3375 |
| --namedstyle-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3453 |
| --format-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3531 |
| --insert-rows-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3609 |
| --delete-rows-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3688 |
| --insert-columns-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3766 |
| --delete-columns-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3845 |
| --names-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 3923 |
| --link-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4001 |
| --props-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4079 |
| --protect-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4157 |
| --validate-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4235 |
| --cf-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4313 |
| --scale-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4391 |
| --table-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4469 |
| --comment-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4548 |
| --image-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4626 |
| --richtext-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4704 |
| --richcomment-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4783 |
| --sheetview-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4861 |
| --pagesetup-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 4939 |
| --printarea-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5017 |
| --group-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5095 |
| --linestyle-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5173 |
| --roundtrip-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5251 |
| --read-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5329 |
| --namedstyles-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5407 |
| --colors-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5485 |
| --commentbox-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5564 |
| --anchor-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5642 |
| --remove-image-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5720 |
| --copy-sample | 9956034c4326 | ok | fail | fail | "the ods holds 3 table elements and the workbook 2 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5799 |
| --kinds-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5877 |
| --pivot-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 5955 |
| --extlink-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6033 |
| --charts-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6111 |
| --add-chart-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6189 |
| --chart-kinds-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6267 |
| --chart-more-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6345 |
| --chart-3d-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6423 |
| --chart-labels-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6501 |
| --remove-chart-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6579 |
| --chartsheet-sample | 9956034c4326 | ok | fail | fail | "the ods holds 3 table elements and the workbook 2 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6657 |
| --chart-surface-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6735 |
| --chart-format-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6813 |
| --chart-trend-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6891 |
| --chart-axis-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 6969 |
| --chart-text-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 7047 |
| --chart-edit-sample | 9956034c4326 | ok | fail | fail | "the ods holds 2 table elements and the workbook 1 sheet elements" | "the ods holds 2 table elements and the workbook 1 sheet elements" | 7125 |

### 2. The sheet sample on the five google-sheets ids (5 lines)

emend-copy's own check refuses the output, exit 2; the input opens
(lo_in ok).

| Sample | Id | Outcome | Exit | lo | lo_in | Error | run.log line |
|---|---|---|---|---|---|---|---|
| --sheet-sample | 15205770ae5c | error | 2 | - | ok | "error: the output's active tab is not what the edits imply" | 3048 |
| --sheet-sample | 453a166dcdfe | error | 2 | - | ok | "error: the output's active tab is not what the edits imply" | 3051 |
| --sheet-sample | 6e0be3d0f2b1 | error | 2 | - | ok | "error: the output's active tab is not what the edits imply" | 3055 |
| --sheet-sample | 90a5b01f9541 | error | 2 | - | ok | "error: the output's active tab is not what the edits imply" | 3058 |
| --sheet-sample | 91d2bdffa46a | error | 2 | - | ok | "error: the output's active tab is not what the edits imply" | 3060 |

### 3. Row and column deletion across a shared formula (2 lines)

The documented refusal, exit 2; the input opens (lo_in ok).

| Sample | Id | Outcome | Exit | lo | lo_in | Error | run.log line |
|---|---|---|---|---|---|---|---|
| --delete-rows-sample | 90a5b01f9541 | error | 2 | - | ok | "error: an insertion or deletion cuts a shared, array or data table formula" | 3685 |
| --delete-columns-sample | 91d2bdffa46a | error | 2 | - | ok | "error: an insertion or deletion cuts a shared, array or data table formula" | 3843 |

### 4. Threaded comments on 15205770ae5c (4 lines)

The documented refusal, exit 2; the input opens (lo_in ok).

| Sample | Id | Outcome | Exit | lo | lo_in | Error | run.log line |
|---|---|---|---|---|---|---|---|
| --comment-sample | 15205770ae5c | error | 2 | - | ok | "error: a worksheet has threaded comments" | 4538 |
| --richcomment-sample | 15205770ae5c | error | 2 | - | ok | "error: a worksheet has threaded comments" | 4773 |
| --commentbox-sample | 15205770ae5c | error | 2 | - | ok | "error: a worksheet has threaded comments" | 5554 |
| --copy-sample | 15205770ae5c | error | 2 | - | ok | "error: a worksheet has threaded comments" | 5789 |

### 5. The remove sample on 495c7b79b183 (1 line)

The edit succeeds and the input opens; the output fails the open
check's sheet count after the last sheet's removal.

| Sample | Id | Outcome | lo | lo_in | lo reason | lo_in reason | run.log line |
|---|---|---|---|---|---|---|---|
| --remove-sample | 495c7b79b183 | ok | fail | ok | "the ods holds 2 table elements and the workbook 1 sheet elements" | - | 3134 |

## Slips and superseded versions

- Dry run d1 under <dry> (ruling M): the gate's office-literal check
  found ruling J's name of the superseded ruling file in the committed
  spec, accepted for this phase alone by change K98 with a planted-literal
  control; the run's passthrough baseline was the spec's naming error,
  fixed by change TG (the selection rule's pick, m6p6-roundtrip); the
  first scan's two real failures were fixed by change MK (PLANNING_SPEC
  equal to the phase commit's blob reads as unset); a third was the dry
  driver's, which ran a copy of the scan (dry.sh d8903793, now in
  <superseded>), rerun from the phase's scripts. The rerun's four
  structural failures are the dry layout's alone (ruling M).
- The first K98 version had two control defects, a planted name the
  office pattern could not match and a Python newline reaching the
  shell; both were fixed before dry run d2 (ruling N).
- Dry run d2 under <dry> (ruling N): gate yes, run yes with 0 FINDING
  lines, scan with the four dry-only structural failures.
- In <superseded>, as the implementing session lists it (no log prints
  this listing): gate-gen1 (gate.py cd4ca8ac, script 4f10daa0, and the
  first K98 script 68355fc9), run-gen1 (run.py defb9e5c, script
  e9bacd1b), scan-gen1 (scan.py ed360c54, script 0b6c8b34), dry.sh
  d8903793, and five k98-test directories of the K98 control's trials.

## Independent check 1

independent-check-1.md holds the answer unedited (ruling L).
Dispositions:

The line numbers in the answer are the README's before these edits.

- Q1 (a), the placeholder: filled here.
- Q1 (b), the quoted texts' primary: the paragraph above the tables now
  says the run directories are private and uncommitted and no committed
  file holds the texts.
- Q1 (c), "all on the 12 new ids": reworded to "all on new ids (7 of the
  12)".
- Q2 and Q3 2, the 118 "stderr bytes: 0" lines: they are 59 runner-script
  lines and 59 record-script lines, as the check found; the README is
  corrected. Ruling O's "118 record-script lines" carries the same
  wording; that is planning's to correct.
- Q3 1, the gate's start: the README now quotes gate.log's "run at
  2026-11-05T15:39:51Z"; 15:39:50Z was the driver's own line.
- Q3 3, the driver captures: reworded to say no log prints them.
- Q3 4 and 5, the superseded listing: accepted as the session's statement;
  the bullet now says no log prints it.
- Q3 6, the report's hashes: the report gives this README's final hash
  and the extraction script's and output's hashes as the session's.
