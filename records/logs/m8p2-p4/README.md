# M8 part 2 P4 logs

specs/0048-the-two-refusals.md, "Harness runs", "### P4: the reruns",
rulings H, K (4), L and M, with ruling N given by planning's message of
2026-11-10 (planning writes it into the spec after this record commit).
The gate and the run took the working spec at PLANNING_SPEC
9a6e0db513d352beb46957a9212cf703492aec70a2e13c4c3f64b7a6dba4df31 (each log
prints it). `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m8p2-p4`.

root: <home> stands for the home directory of the account the runs ran under
root: <phase> stands for <home>/.cache/emend-build/m8p2-p4, the phase's directory
root: <corpus> stands for the private corpus directory the run reads
root: <work> stands for run.log's working directory, as run.log's root line 79 defines it
root: <scratch> stands for <phase>, as in the logs; the independent checks' materials are under <scratch>/check1 and <scratch>/check2

| Role | Commit | Subject |
|---|---|---|
| base | 053d27c | M8 part 2 P3 record: the size record, the logs, the README, the check |
| doc | 3bbac34 | M8 part 2 P4: commit planning docs (the P3 verification and rulings) |
| documents (phase) | f625b99 | M8 part 2 P4: the README's figures, the CHANGELOG clause and the crate docs' sentence |

No file name, sheet or table name, note or cell content of any private
workbook is in these logs or in this README. Workbooks are named by id,
the first 12 hex digits of the sha256. The texts quoted below are the
logs' own lines.

## The logs and the records

| File | Bytes | Lines | sha256 | Last line |
|---|---|---|---|---|
| gate.log | 924768 | 12078 | a27d8ee7380e9338c926976e93d3a508b90530457dc1f2ed55a6575f0d42336a | gate log checks all as expected: yes |
| run.log | 522611 | 7599 | 4f148de2476026143caee63435352e861145c4f39a3eb8c78ba2995ee16307ac | run log checks all as expected: yes |
| scan.log | | | | the final scan, written after the independent checks and before the record commit, which holds it |

The records are the 118 files `records/m8p2-<name>-2026-11-10-f625b99.md`
and `.txt`, a pair for the passthrough and for each of the 58 sample
flags, 12409762 bytes in all. run.log wrote each in <work>/records,
hashed it under its run (for example the remove pair, line 3869) and
installed it into records/ in section 5 (line 7266), each with its sha256
and bytes; "every record file was installed, byte for byte: yes" (line
7393).

The scripts are <phase>/scripts/m8p2-p4-gate.sh, -run.sh and -scan.sh.
Each log prints the script that wrote it. Dry run d1, under
<phase>/dry/d1, ran the same three scripts; the logged driver,
<phase>/logged.sh, checked them cmp-equal before starting. There is no
size log in P4.

Each log's own start: gate.log line 359, "run at 2026-11-10T19:11:14Z";
run.log line 66, "run at 2026-11-10T19:23:18Z". Each log's end: gate.log
line 12077, run.log line 7598 ("run ended at 2026-11-10T23:47:31Z").

## The gate

- The prune (gate.log lines 181 to 246): P3's build target, 9018112251
  bytes (line 241), was removed by `rm -r "${PREV_DIR:?}/${n:?}"`,
  "target: rm -r exit 0" (line 243). The 36 other names were kept (line
  191).
- The crate's tests: "the figures this run must give: off 544, ch 719, im
  581, ci 761, cc 768, cr 826, all 1244" (line 11060), P3's figures, the
  phase adding no test. Each set passed those figures on the host (lines
  11066 to 11228) and on i686 (lines 11255 to 11417), with 0 failed.
- The doc and clippy sets (line 10732) include no feature, tables alone
  and metadata alone (ruling I).

## The run and the expected change

The passthrough and the 58 samples ran over the 39 workbooks with every
output opened. Each was compared per id on outcome, lo and lo_in with
part 1 P2's record of the same name, records/m8p1-<name>-2026-11-09-8816db0.txt.
"runs done 59 of 59 (the passthrough and 58 flags); record pairs written
59" (line 7200).

The table (run.log lines 3317 to 3328) holds the six changes "Harness
runs" names for P1, P2 and P3 and the one pair it names unchanged:
seven pairs, which is what "the seven changes" means (ruling N (1)).

| Pair | Then, now | Comparison line | Summary line |
|---|---|---|---|
| delete-rows 90a5b01f9541 | error/-/ok, ok/ok/ok | 4330 | 4333 |
| delete-columns 91d2bdffa46a | error/-/ok, ok/ok/ok | 4461 | 4464 |
| comment 15205770ae5c | error/-/ok, ok/ok/ok | 5047 | 5050 |
| richcomment 15205770ae5c | error/-/ok, ok/ok/ok | 5243 | 5246 |
| commentbox 15205770ae5c | error/-/ok, ok/ok/ok | 5894 | 5897 |
| remove 495c7b79b183 | ok/fail/ok, ok/ok/ok | 3872 | 3877 |
| copy 15205770ae5c (kept) | error/-/ok, error/-/ok | 6090 | 6093 |

- Each of the six changed samples reads "earlier ids 39; the same 38;
  changed 1; missing from this run 0; new ids 0" at its comparison line.
  Copy reads "the same 39; changed 0". Every other sample reads "the same
  39, changed 0, missing 0" in its summary line.
- The remove sample, by removed_reason (line 3874): "- 26 last 12
  pivot_source 1". The pair's line reads removed 0 and removed_reason
  pivot_source.
- Line 7456: "the table's pairs met: 6 of 6; the remove pair's line met
  (removed 0, removed_reason pivot_source): 1 of 1; other ids reading
  pivot_source: 0; the pair named unchanged and kept: 1 of 1". Line 7457
  judges it "yes".
- The remove-chart sample on the 39 (lines 6741 and 6743): "the same 39;
  changed 0". The dry finding below does not show on any real id.

The summary lines' counters "their lines met" and "other ids reading
pivot_source" are cumulative: once the remove sample has met its line,
every later summary reads "their lines met 1" (for example the font
sample, line 3942). They were left as they are (ruling N (3)). The
per-run counts (lines 7396 to 7455) and the totals (line 7456) are not
cumulative.

## FINDING lines

run.log: none ("FINDING lines printed above: 0", line 7464; every summary
line reads "FINDING lines so far 0"). gate.log: none outside its printed
script; its two lines with the word (5723 and 5732) are comments in the
printed script's carried history. No log has a line ending UNEXPECTED.

## A finding of this part: the namespace declaration left after a chart's removal

Dry run d1's run.log (<phase>/dry/d1/logs/run.log, sha256
3ea115cf5a01711c0ec9eeb9b07ff61cc91edcee156653df5ac8284c603824a9) has 8
FINDING lines, all in the remove-chart sample, on 8 of the 9 dry files.
Each reads ok/ok/ok in part 1 P2's record, error/-/ok in the dry run
(line 6698: "same 1, changed 8, missing 30"). The dry FINDINGs are a dry
cut (ruling N (2)): the dry corpus is generated, and its ids differ from
the 39. The behaviour behind them is a finding of this part, not fixed
here; planning adds it to ROADMAP "Unscoped" at the close-out.

Reproduction, after the logged run, in <phase>/residue, on the dry file
453a166dcdfe with the logged run's binary (<work>/bin/emend-copy) and
`EMEND_COPY_KEEP_OUTPUT=1 emend-copy in.xlsx out.xlsx --remove-chart-sample`:

- Exit 2, with the message "error: the output's entry bytes is not what
  the removal implies (output kept)".
- The input has 6 entries and the output 7. 5 of the 6 input entries are
  byte-identical in the output.
- xl/worksheets/sheet1.xml is 262 bytes in the input and 340 in the
  output. The input's worksheet root does not declare `xmlns:r`; the
  output's root does, and the output has no `<drawing>` element. The
  sample adds a chart, which declares `xmlns:r` on the root for its
  `<drawing r:id>`, then removes it. The removal drops the `<drawing>`
  element but keeps the declaration.
- The output also holds xl/worksheets/_rels/sheet1.xml.rels, 100 bytes,
  with 0 Relationship elements. The input has no such part. The sample's
  check does not read it.
- in.xlsx sha256 fceb4ba369fc65821ca9d31bb7d6e94dca54d3a7f59f0ee114fa81f043aa1ceb;
  out.xlsx sha256 56f239ed0bba477ca5d81b784238fd4e775db870472b2e21c4387b7777f93f99.

15205770ae5c, whose dry worksheet root declares `xmlns:r`, stays ok in
dry d1. Part 1 P2's records give remove-chart ok/ok/ok on all 39 real
files, and this run gives the same. So adding then removing a chart is
not byte-identical on a sheet whose root lacked the namespace.

## Figures from files outside this directory

Each is named with its source; none is in a committed log:
- <phase>/logged.sh (sha256
  658abb65963042d42120f41015ef9891c7ba7175bd7f304a8739cf69fadfa942)
  wrote <phase>/logged/driver.out:
  - "gate started 2026-11-10T19:11:12Z" and "run started
    2026-11-10T19:23:18Z";
  - "outer stderr 0 bytes" for each step; the gate and the run scripts'
    own standard error 0 bytes each;
  - "m8p2-p3's target after the gate: gone; its names: 36";
  - "free space 738G" at the start; the root mount's propagation was
    checked shared before the driver ran;
  - "files under records/ matching m8p2-: 118".
- The run's working directory is <phase>/tmp/tmp.ctmjlg2mGu, the one new
  name in <phase>/tmp, from the driver's before and after listings
  (<phase>/logged/phase-tmp-before.txt and -after.txt). The driver
  prints "<phase>/tmp names unchanged: no" for it; the run keeps <work>
  by design (run.log line 79).
- The reproduction above: <phase>/residue/bytes.txt, run.stderr and the
  two workbooks' hashes (sha256sum).

## Leftovers (left in place for the maintainer's deletion list)

- <phase>/dry/d1 with its build roots and its dry corpus.
- <phase>/dev-target, where the session built the doc sets.
- <phase>/tmp/G, R and S, the forks' temporary files.
- <phase>/tmp/tmp.ctmjlg2mGu, the run's working directory.
- <phase>/residue, the reproduction above.
- <phase>/target and <phase>/gate, this phase's target and build roots.
  The next phase's gate prunes the target.

## After independent checks

The body above is kept as the checks read it (sha256 e7928b72c16966adac3d8ba3d522fd7913acb9a0c2cab75375f2262bab703fa7),
as in P1 to P3. independent-check-1.md (the records against the run) and
independent-check-2.md (the report) hold the answers verbatim. Check 1
found the 118 files equal to run.log's section 5, by name, sha256 and
bytes, and the records' own per-id comparison gives the six changes, the
copy pair kept and nothing else. Check 2 found the two commits as the
spec says and no required log line missing. Each point either check raised is
answered from the primaries:

- Both checks, "the dry corpus is generated, and its ids differ from the
  39" (the section on the finding): wrong. The dry corpus's nine
  workbooks are generated, with fictional content, but each is labelled
  with one of nine ids of the real 39, so the dry comparison meets real
  ids (<phase>/gen/make-dry-corpus.py, sha256 da1db571757fbf8c3711b30eff7e9c090620ff658c4e991a9f1a1cc31387208a,
  lines 8 to 13). "The dry file 453a166dcdfe" is the generated workbook
  under that label. Its own sha256 is fceb4ba3... (the reproduction's
  in.xlsx), not the label. Ruling N (2)'s reason for the dry cut is that
  the 39 real workbooks declare the prefix: part 1 P2's remove-chart
  records and this run read ok on all 39.
- Both checks, "planning writes it into the spec after this record
  commit": wrong. Ruling N was already in the working spec (line 882)
  that both logs took at 9a6e0db5. Planning commits it with its next doc
  commit.
- Check 1, run.log's end: line 7598 is the "run ended at" line; line
  7599, the table's last line, is the verdict. Both are right.
- Check 1, "the pair's line reads removed 0 and removed_reason
  pivot_source": the records do not carry those fields. run.log reads
  them from the sample's own lines: the reasons by id at line 3874 ("-
  26 last 12 pivot_source 1"), the pair's line met at 3877 ("their lines
  met 1") and the totals at 7456 ("1 of 1"; "other ids reading
  pivot_source: 0"). Line 3875 is the expected value. The line_field
  control is at line 3350.
- Check 1, ruling L in the opening list: the driver keeps ruling L's
  attempt-2 form, its output through sed -u (L4). The rulings the
  phase's code follows are H, K (4) and M (1).
- Check 2, the prune section: it runs from gate.log line 181 to 248 (line
  248 is its last control), not 246.
- Check 2, the test lines: the cited ranges are the passed-figure lines
  of the first and the last set. The "all" set's block runs on to lines
  11249 (host) and 11438 (i686).
- Check 2, the pre-check scan's outer standard error: it is
  <phase>/scan/pre-check.outer.stderr, 0 bytes (wc -c), not among the
  check's materials.
- Check 2, the dry run's FINDING lines, listed here as dry d1's run.log
  gives them, in the remove-chart sample, each "outcome/lo/lo_in ok/ok/ok
  in m8p1-remove-chart-2026-11-09-8816db0.md, error/-/ok now; recorded,
  not fixed": line 6665 id 2adafe76df11; 6668 453a166dcdfe; 6669
  495c7b79b183; 6673 6e0be3d0f2b1; 6680 90a5b01f9541; 6682 91d2bdffa46a;
  6683 9956034c4326; 6690 d8245b84086f. Line 7411 reads "FINDING lines
  printed above: 8".
- Check 2, the driver's "FINDING lines" list: its grep also catches the
  summary lines that hold the words "FINDING lines so far". None of those
  is a FINDING line.
- Check 2, the reproduction's details it could not check: a second run
  after the checks, in <phase>/residue/second, gave "emend-copy exit 2"
  (exit.txt) and an output byte-identical to the first (cmp). rels.txt
  reads "xl/worksheets/_rels/sheet1.xml.rels bytes 100 Relationship
  elements 0". The dry 15205770ae5c's xl/worksheets/sheet1.xml root
  declares xmlns:r (read with zipfile from the dry corpus).
- Check 2, the TABLE list in package.rs: the constant's cfg at
  f625b99 is any(rowcol, tables, read, all(sheetprops, metadata))
  (crates/emend/src/package.rs, the lines before `pub const TABLE`),
  as the doc sentence says.
- Check 2, ROADMAP's P4 row: it stays "PLANNED" in 3bbac34. That is
  planning's document, not changed here; for planning.
