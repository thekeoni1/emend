# 0044: Milestone 7 part 1, the Excel check

Status: APPROVED 2026-10-25 (Milestone 7 part 1; decisions 1 to 3 A,
the recommended option in each, typed in the planning session
2026-10-25; drafted the same day). Written by the planning session
for the implementing session. The P1 brief follows Milestone 6's
close-out, DONE and PUSHED 2026-10-24.

Milestone 7, "release", was approved by the maintainer 2026-10-25
(`ROADMAP.md`, "Milestone 7"): no feature is added; the Excel check
runs before anything is published (decision 1); the Milestone 2
ceiling is decided at the next milestone that touches the core
(decision 2); the corpus's maintainer items follow the publish
(decision 3); the version is 0.1.0 (decision 4). This part is the
first of three: the Excel check of `specs/0015-open-checks.md`, "The
Excel check", as the parts since grew it, last staged 2026-10-09 at
Milestone 3 part 7 P5 (`specs/0029-engineering-close.md`, P5 and its
rulings) and skipped by the maintainer the same day, deferred
fourteen times in all. Every milestone's acceptance since Milestone
1 rests on LibreOffice headless alone (`records/excel-check-m1-2026-
09-24.md` is the one Excel record). After this part a record names,
for every sample of Milestones 2 to 6 over every corpus workbook and
for every kept fixture pair, whether desktop Excel opened emend's
output without a repair prompt.

Context: `CLAUDE.md` ("The fidelity harness is the acceptance
instrument": a change is correct when the result opens in LibreOffice
headless and in Excel without a repair prompt; "Corpus rules"; "Two
sessions, one repository"), `specs/0001-bootstrap.md` ("Rules for
every phase"), `specs/0015-open-checks.md` ("The Excel check", "The
keep", the Excel-check rulings), `specs/0016-new-workbook-and-
sheets.md` ("The carried items", decision 5, the P1 rulings 1 to 7:
the stage's layout, the list's rows), `specs/0029-engineering-
close.md` (P5, P4 ruling 2, the P5 verification rulings 1 to 5, the
rulings before the close-out), `specs/0030-carries-and-api-changes.md`
(item 1: the list script's error rows), `specs/0043-measurement-and-
close.md` (the close-out form; rulings Q and R as standing rules),
`records/excel-check-template.md`. No external text is cited.

## Goal

At the close-out, HEAD holds:

- one run directory per sample flag of `emend-copy` under
  `<corpus>/runs`, every sample run on the host at P1's phase commit,
  each recorded as `records/m7p1-<sample>-<utc-date>-<shortsha>.md`
  and `.txt` by the committed record script;
- the host keep regenerated at the same commit and opened by
  `scripts/edit-open-check.sh` into `records/m7p1-open-<utc-date>-
  <shortsha>.md` and `.txt`;
- the list written by `scripts/excel-check-list.py` from them all
  and staged under `<corpus>/work` with the opener script and the
  files, zipped; neither the list nor the stage nor the zip
  committed (`specs/0015` P2 ruling 7);
- the Excel record `records/excel-check-m7-<utc-date>.md`, in the
  template's form, assembled by planning from the opener's results
  file and the machine section the maintainer brings back, every row
  checked by hash against the stage, committed by the implementing
  session with a scan;
- the close-out push's Excel-record check restored, reading exactly
  one file matching `records/excel-check-m7-*.md` at the tip;
- `ROADMAP.md`'s part 1 rows DONE.

No line of code the probes link changes. The crate, the tools and
the scripts are at Milestone 6's close; a script change, if one is
needed, is a numbered change printed in the log that runs it, and the
spec names it in a ruling first.

## Size

No size record in this part: no line the probes link changes, so no
probe can move. The gate builds as every gate does. Part 2's size
record, the milestone's one, is expected to read the budget 451528
and every feature cost equal to `records/size-m6p6-2026-10-24-
7ac8b74.md`, a movement a finding.

## Facts planning checked (2026-10-25)

- `tools/emend-copy/src/main.rs` names 59 flags ending in `-sample`
  (`grep -o -- '--[a-z0-9-]*-sample' | sort -u | wc -l`). `--new-
  sample` writes its own input (`specs/0016`, P2) and has never been
  on an Excel list; `--drop-calc-chain` is not a sample (`specs/0029`
  P5 verification ruling 1). Both stay out.
- The Milestone 2 stage `<corpus>/work/excel-check-m2-2026-10-09/`
  stands with its zip (604025974 bytes): the list 3134 rows under
  its header, 972 corpus rows from 36 directories of 27, 1081
  fixture and 1081 input rows. It covers the samples of `specs/0013`
  to `specs/0028`; every sample since (`specs/0032` to `specs/0042`)
  is absent from it. It is superseded by this part's stage and is
  not read.
- Milestone 3 part 7 P5's run log (`records/logs/m3p7-p5/run.log`)
  ran 23 reruns, the keep and its open check in 2 h 12 min
  (2026-10-09T16:42:04Z to 18:53:50Z). This part's 58 reruns (59
  less `--new-sample`) are expected to take about two and a half
  times that on the host; the logged run is one script and runs
  unattended.
- `scripts/excel-check-list.py` leaves a summary row whose outcome
  is error out of the list and prints the count once per directory
  (`specs/0030` item 1), so the printarea sample's error rows no
  longer keep it off the list (`specs/0029` P5 verification ruling
  4, the carried item, delivered by Milestone 4 part 1).
- The close-out push script of Milestone 6 part 6 counts files at
  HEAD matching `records/excel-check-m2-*.md` and expects 0, with a
  control (`specs/0029` rulings before the close-out, ruling 1;
  change 56). This part's record is named `excel-check-m7-` so that
  count stays 0; the restored check (below) has its own pattern.
- The LibreOffice image `localhost/emend-lo:25.2.3-2-deb13u6` is
  present; the corpus manifest holds 27 workbooks; `<corpus>/runs`
  holds 230 directories; the disk has 627 GB free.
- `records/excel-check-template.md` is the record's form: the
  machine section, the totals, corpus rows (`seen1` to `seen3`
  against the list's expectations) and fixture rows with the input's
  result beside each. `scripts/excel-check-open.ps1` writes
  `open-results.txt` with `source`, `id_or_test`, `label`, `result`,
  `seen1` to `seen3` and `message`; it stops at a repair prompt.

## The reruns (P1)

Every flag ending in `-sample` that the tool's usage text names at
the phase commit, less `--new-sample`, is run once on the host over
the 27 corpus workbooks by `scripts/passthrough-run.sh` with the
phase's release `emend-copy` and `xlsx-diff`, `--label host`,
`--tool-arg --<name>-sample`, the pinned image, `--lo-timeout` and
`--timeout 300`, `TMPDIR` under the run's working directory on the
disk, into `<runs>/<utc-date>-<shortsha>-<name>`, `<name>` the flag
without `--` and `-sample`; and recorded by
`scripts/passthrough-record.py` as `records/m7p1-<name>-<utc-date>-
<shortsha>.md` and `.txt`. The run log prints the flag list as the
usage text gives it, the two exclusions, the directory each run
wrote and its outcome counts from `summary.host.tsv` (ok, error,
and any other outcome), and asserts that every directory's
`run.host.tsv` names the phase commit and its flag. The reruns are
the harness's measurements at the phase commit, not repeats of
earlier ones, and their records say so on their `Commit:` line.

Each sample's record is compared with that sample's latest committed
record by the outcome counts of its host row (files, ok, lo ok, lo_in
ok) and the count of its summary's identical column; a difference is
printed as a FINDING and recorded, not fixed (a sample's outputs at
Milestone 6's close are what Excel is asked about; what changed since
its last record is for part 2's scoping to read). The run log names
the latest committed record it compared each sample with.

The i686 label is not run: every part's records show its outputs
byte-identical to the host's by sha256, and Excel opens files, not
binaries. The Excel record says so in its header.

## The keep and its open record (P1)

The host test run of the crate and the tools at the phase commit
with `EMEND_KEEP_DIR` set to `<phase>/keep-host` (`specs/0015`, "The
keep"), the directory created by the script; then
`scripts/edit-open-check.sh --keep <phase>/keep-host --libreoffice
<image> --out records --stem m7p1-open-<utc-date>-<shortsha>`. The
run log prints the pair count and the totals line; the open record's
totals are compared with `records/m6p6-open-2026-10-24-7ac8b74`'s
(pairs, lo_in ok, lo ok), a difference a FINDING, recorded.

## The list and the stage (P1)

`scripts/excel-check-list.py --run <runs>/<utc-date>-<shortsha>-
formula`, then `--run-plain` for every other rerun directory in the
order the usage text names the flags, `--keep <phase>/keep-host`,
`--record records/m7p1-open-<utc-date>-<shortsha>.txt`, `--out
<stage>/excel-check-list.txt`, where `<stage>` is
`<corpus>/work/excel-check-m7-<utc-date>/`, created by the script
with mode 700. The run log prints the command with every directory
named by its placeholder form, the script's stderr notes (the error
rows left out, per directory), the list's row count and its counts
by source and by label, its sha256 and mode, never a row.

The stage, in `specs/0016` P1 ruling 4's layout: the list as
`excel-check-list.txt`, `scripts/excel-check-open.ps1` copied beside
it (its sha256 printed and equal to the repository's), and every
file the list names at `files/<source>/<file>`: corpus rows' outputs
from their run directories, fixture rows' outputs and input rows'
inputs from the keep. The run log prints the count of files under
each of the three `files/` directories and asserts each equals its
source's row count, and that every file's bytes and sha256 equal
the list's columns for it (the assertion's command printed; a
mismatch stops the script). The zip is written beside the stage as
`<corpus>/work/excel-check-m7-<utc-date>.zip` by Python's `zipfile`
(no `zip` binary is assumed), stored, its byte count and sha256
printed. Nothing of the corpus but the files the list names is
copied; the stage and the zip are mode 600 and 700 and are never
committed.

The Milestone 2 stage and its zip are not read, not removed and not
moved by any script of this part: their removal is the maintainer's,
by a method the maintainer names, after this part's zip is on the
Excel machine (decision 3).

## The maintainer's check (P2)

As `records/excel-check-template.md`, "Procedure": the zip carried
to the machine with Excel and unzipped outside any synced folder;
Excel closed; `powershell -ExecutionPolicy Bypass -File
.\excel-check-open.ps1` from the staged directory; a repair prompt
stops the run at that file, the row noted, the prompt answered, the
row recorded `repair`; `open-results.txt` and the machine section
(Excel version and build, operating system and version, the date in
UTC) brought back. The by-hand fallback for a corpus row the opener
recorded `fail` or `read-error` as the template says, the row marked
`(by hand)`. Nothing is saved on the Excel machine; Excel's message
on a refused corpus row stays in `open-results.txt`.

The results file comes back to the development machine as a file
under `<corpus>/work/excel-check-m7-<utc-date>/` (the maintainer
copies it; no session fetches anything from the Excel machine). The
maintainer tells the implementing session its path by placeholder
and the machine section's three lines in the implementing session.

## The record (P2)

`records/excel-check-m7-<utc-date>.md`, the date the check was run,
assembled by planning from `open-results.txt` and the stage's list,
in the template's form: the header naming this spec, the stage, the
reruns as measurements at P1's phase commit, the i686 label's
absence and the exclusions; the machine section as brought back; the
totals table (rows; corpus pass, repair, fail; fixture pass,
repair, fail; input pass, repair, fail) and the fixture pairs by
(input, output); the corpus rows with `seen1` to `seen3` and the
fixture rows with the input's result beside each, every row's
`bytes` and `sha256` the list's. The cell references never enter the
record. A row of the results file that names no list row, or a list
row with no result, stops the assembly and is a FINDING.

Planning checks every row's `bytes` and `sha256` against the stage
by a command it prints in its verification, and the totals against
the results file by counts. The implementing session commits the
record in P2's record commit (with the logs), the scan over it; a
corpus id or a test name is not private (every record since T3
names them), and a `seen` value is a number the sample wrote, never
a workbook's own cell.

Acceptance, as the template's: every corpus row reads `pass` and
every `seen` value equals its `expect`; an input Excel refuses is
allowed and listed with its output's result; an output Excel refuses
or repairs on an input Excel accepted is a FINDING, listed by test
name or corpus id and sample. Each FINDING is carried to part 2's
scoping, where the maintainer decides per finding: fixed before the
publish, or documented in the README's limits and the CHANGELOG as a
known limit (decision 2). This part fixes nothing.

## The restored Excel-record check (P2 and the close-out)

The close-out generator transforms Milestone 6 part 6's scripts
(`gen.py` and the three it printed, carried cmp-equal), its numbered
changes continuing from the last change Milestone 6 part 6 printed.
Change 56's count of `records/excel-check-m2-*.md` stays, expecting
0 with its control. Beside it, a numbered change restores the check
`specs/0029` P4 ruling 3 describes, with this part's pattern: in pre
mode, at the tip, exactly one file matches `records/excel-check-
m7-*.md`; it holds a "## Results" heading and at least one table
row whose result cell reads `pass`, `repair` or `fail`; the control
is a clone's tip with the record removed, refused. The record is
committed in P2's record commit, so no message names it; ruling 3's
message clause is not restored. The negative log of P2 extracts the
function by a printed line range, prints the extract's sha256 equal
to the script's lines, and runs it against the current tip
(accepted, once the record is committed) and against the clone
(refused). The check stays in part 2's push script (part 2's spec
says so); whether it stays beyond is part 2's close-out's to say.

## Harness runs

P1: every sample as "The reruns (P1)", host only; the keep and its
open record as "The keep and its open record (P1)". P2: none. The
scale tiers are not run (`specs/0023` "The tiers": the tiers are
the comparison's and the memory classes', not the Excel list's).

## Phases

Each phase as `specs/0001` "Rules for every phase": a doc commit
first, the code commits if any, one record commit, the scan last, at
most two independent checks, the report under `<phase>`, then stop.
Target root `<home>/.cache/emend-build/m7p1-pN/target`. The gate
generator is Milestone 6 part 6 P4's with the part's names, as a
numbered change. A dry driver sets every build root a script
defaults under `<phase>` to the dry run's own directory (`specs/
0043` ruling Q); every dry and logged run report lists every FINDING
line found by grepping the word (ruling R).

### P1: the reruns, the keep, the list, the stage

The doc commit holds this spec, `ROADMAP.md` (Milestone 7's section,
part 1's P1 row IN PROGRESS, Milestone 6's close-out edits) and the
working `specs/0043` (its close-out verification and rulings W and
X), on 3f68f3b. No code commit unless a ruling names one. The logs:
the gate; the negative log (the list script refusing a run directory
whose summary is missing, a `--run-plain` directory under another
corpus root, and a `--run` directory that is not the formula
sample's; the stage assertion refusing a planted byte change in one
staged file, on a copy); the run log as the three P1 sections say,
every FINDING printed; the README; one independent check on the run
log's directory list against the usage text's flag list and the two
exclusions, and on the record pairs' count against the directories;
the scan. The report prints the stage's and the zip's paths by
placeholder, the row count and the zip's byte count and sha256.
Then planning's verification; then the maintainer's check.

### P2: the record

After the maintainer's check. The doc commit (P1's verification and
rulings; the P2 row IN PROGRESS). Planning assembles the record from
the results file and sends its sha256; the implementing session
writes it byte for byte at `records/excel-check-m7-<utc-date>.md`,
prints its sha256 equal, and commits it in the record commit with
the logs. The logs: the gate; the negative log (the restored check's
two cases, "The restored Excel-record check"; the assembly's two
stops shown on a scratch copy of the results file with a planted
row and a removed row); the run log (the record's every row's
`bytes` and `sha256` found in the stage's list, by a command that
prints counts; the totals recomputed from the results file and shown
equal to the record's); the README; one independent check (does the
record's every row and total come from the results file and the
list; does the restored check's extract equal the script's lines);
the scan over the record among the rest.

## Close-out push (after P2)

As Milestone 6 part 6's (`specs/0043` "Close-out push" and rulings W
and X): the generator transforms part 6's scripts, its numbered
changes continuing; the doc commit holds this spec and `ROADMAP.md`;
the base is Milestone 6 part 6's close-out doc commit 42ddf3b, named
by its subject ("M6 part 6 close-out: commit planning docs (the P4
verification and rulings)"); `PREV_RECORD_PATTERN` is `^M6 part 6
close-out record` (3f68f3b), in the range exactly once; the
close-out doc commit's subject is "M7 part 1 close-out: commit
planning docs (the P2 verification and rulings)";
`CARGO_TARGET_DIR` printed and checked as `<home>/.cache/emend-
build/m7p1-close/target`; the visibility read PUBLIC with its
control; the restored Excel-record check in pre mode and change 56's
count 0; the record commit "M7 part 1 close-out record: the push,
the CI run and the scan" holding the three logs only; pre mode on a
clean tree; one independent check before the report. The push
carries 3f68f3b and this part's commits, on planning's word and the
maintainer's authorisation typed in the implementing session.
`ROADMAP.md`'s part 1 heading then reads DONE and PUSHED, and the
close-out's verification says what part 2 carries: the FINDING list
for decision 2's per-finding choices.

## Planning rulings during P1 (2026-10-25)

Asked by the implementing session after the doc commit 82c07fb
(three files, the blobs' sha256 equal to the hashes the brief sent,
the parent 3f68f3b, the message ASCII with the one trailer; planning
read it from git; first made as 31841f7 and rebuilt on the same tree
60b98e0 and parent by ruling L, planning reading both from git),
before any dry run. With no code commit, 82c07fb is P1's phase
commit.

A. R1: `--lo-timeout 600`, the value of Milestone 3 part 7 P5's
   reruns and Milestone 6 part 6 P2's corpus runs (23 and 97 command
   lines in their run logs). "The reruns (P1)" reads with it.
B. R2: modes. The stage's directories 700, every staged file 600,
   the list 600 (the script's umask), the zip 600 as a file. "The
   list and the stage (P1)"'s "600 and 700" reads that way.
C. R3: the zip is stored (`zipfile.ZIP_STORED`), as the brief says;
   the run log prints the departure from Milestone 3 part 7 P5's
   deflated zip in one line. The staged outputs are zip archives
   already.
D. R4: the gate prunes `<home>/.cache/emend-build/m6p6-p4/target`
   and `m6p6-close/target`, each with a `du -sb` line before and the
   `cargo clean --target-dir` form (`specs/0028` P2 ruling 8, ruling
   C's form), both carrying `CACHEDIR.TAG`; no record cites a target
   root. Nothing else is pruned.
E. R5: accepted. The flag list is the `USAGE` constant of
   `tools/emend-copy/src/main.rs` at 82c07fb, read by the gate's
   release binary's own usage output (the stale binary of 2026-10-11
   is not used, and the log says so); the comparison record for each
   sample is the committed record whose "- Tool arguments:" line is
   exactly the sample's flag and whose "Record written" time is the
   greatest, its path printed beside the sample's counts.
F. R6: accepted. No test log: the keep step is the gate's workspace
   host test command run again by the run log with `EMEND_KEEP_DIR`
   set on that command alone, the keep directory created by the run
   script and empty at entry. The logs are gate, negative, run, the
   README, one check, the scan.
G. The brief's "370 lines" was the spec's count before the approval
   edits (the status paragraph and the decisions' preamble, seven
   lines); 377 is right, and the sha256 is what binds. The brief's
   number is corrected here, not there.

H. N1, a spec claim not checked against the code: P1's negative
   case (c) says the list script refuses a `--run` directory that is
   not the formula sample's; `scripts/excel-check-list.py` at
   82c07fb never reads `run.host.tsv`'s `tool_args` (planning read
   lines 244 to 250 and grepped the script: the flag's name appears
   in two docstrings only). Ruling: option (b). No script change.
   The run script, before it calls the list, reads the `--run`
   directory's `run.host.tsv` and refuses unless its `tool_args` is
   exactly `--formula-sample` (it already asserts every directory's
   flag, "The reruns (P1)"); the negative log extracts that function
   by a printed line range, prints the extract's sha256 equal to the
   script's lines, and shows it refusing another sample's directory.
   Case (c) is the run script's refusal, and the negative log's
   README says so. The list script's own check of its `--run`
   directory is a carried item for part 3's spec, which reruns the
   list over the grown corpus. Case (c)'s sentence in "P1: the
   reruns, the keep, the list, the stage" reads with this ruling.
I. The dry run's layout, as proposed: the restricted dry run writes
   its run directories under `<dry>/croot/runs`, where
   `<dry>/croot/manifest.tsv` and `<dry>/croot/corpus` are symbolic
   links to the corpus's own, so the list script's grandparent rule
   holds; no corpus file is copied; nothing is written under
   `<corpus>/runs` or `<corpus>/work` by a dry run; the dry stage,
   its zip and the dry run's record files are written under `<dry>`,
   never under `records/` (`specs/0043` ruling Q's standing rule).
   The dry run's report prints the links' targets by placeholder.

J. N2: ruling E's rule, applied by the implementing session over
   the 58 samples, picks a scale-tier record (`records/m6p6-scale-b-
   <op>-2026-10-24-d88f735.md`, one generated workbook, timeout
   3600 s) for 13 of them, which would print 13 FINDINGs on the
   files count alone (planning read one such record: host row files
   1, settings timeout 3600 s). Ruling: E with one exclusion. A
   record whose file name carries `-scale-<tier>-` is not a
   candidate; the name, not the result, selects. With it every
   sample's chosen record has a host row of 27 files and the reruns'
   settings line. When the record E alone would have picked differs,
   the run log prints it beside the chosen one, as information, not
   a FINDING.
K. The keep: Milestone 6 part 6 P4's test (bb0a29e) writes a pair
   through `written` and no open record has been made since
   `records/m6p6-open-2026-10-24-7ac8b74` (2240 pairs), so the keep
   is expected to grow by one pair. "The keep and its open record
   (P1)"'s rule stands: a difference in the totals is printed as a
   FINDING and recorded; the README names the test as the cause
   when the dry run shows the growth is that one pair, and the pair
   is on the list as every pair is. Nothing is fixed.

L. D1: the doc commit 31841f7's message, written to the brief's
   form, breaks the carried message rules GN (a doc commit names
   each file it holds as "<path> sha256 <hash>"; every doc commit
   since 266c410 does, 42ddf3b and 92c6b38 read by planning) and AD
   (`specs/0037` change AD: a count in a message needs the
   diff-alone sentence). The brief asked for a paragraph no doc
   commit carries; the error is planning's brief first and the
   commit second. Ruling: option (a). The local, unpushed commit
   (no remote branch contains it) is rebuilt with `git commit-tree`
   on the same tree and the same parent 3f68f3b; the message is the
   briefed subject, a blank line, the three lines
   "specs/0043-measurement-and-close.md sha256 <blob sha256>",
   "ROADMAP.md sha256 <blob sha256>" and "specs/0044-excel-check.md
   sha256 <blob sha256>", each hash by command substitution from
   `git show HEAD:<path>`, a blank line, "No log prints a count for
   this commit; the diff alone carries what it changes.", a blank
   line and the Co-Authored-By line; the paragraph is dropped, as
   no doc commit carries one (the spec and `ROADMAP.md` say what the
   commit carries). The implementing session prints the old and the
   new hash, the two trees equal, the parent, and the diff of the
   two messages; `main` is moved to the new commit by `git reset
   --soft`, nothing else changing, the tree clean before and after.
   The old hash is named in FACTS and the report, and every "31841f7"
   above was amended to the new hash, 82c07fb, by planning when it
   arrived (2026-10-25; tree 60b98e0 on both, parent 3f68f3b, the
   message's two added blocks read from git), with the new
   PLANNING_SPEC. No exemption (option b).

M. The generator readings (gate G1 to G4, negative N1 to N3, run
   R1 to R7, scan S1 to S3, the dry driver), sent before any run:
   accepted as read. In particular: the gate builds the two release
   binaries host-only and prints their sha256 and byte lines for
   the run to check (change Y89); the gate's doc-only form is
   Milestone 6 part 6 P2's; the negative log's three extra cases
   (L3, F2, S2) are kept; the open check writes under `<work>/
   records` and its pair is installed into `records/` before the
   list, Milestone 3 part 7 P5's form, which "The keep and its open
   record (P1)" reads with; the open check's own exit 1 is a FINDING
   and its exit 2 fails the log; a rerun outcome other than ok or
   error is a FINDING and the directory stays off the list; the
   printarea sample's comparison record is `records/m4p1-p3-
   printarea-*` (21 ok of 27); the scan derives the 118 record file
   names from the phase commit's `USAGE` constant and the open
   record's stem. The dry run as designed (ruling I's layout, three
   corpus ids by their manifest ids, three flags, the keep made, the
   scan pre-check in `<dry>`) has planning's word.

N. Dry run d1 (PLANNING_SPEC 0ca0630c, phase commit 82c07fb): the
   gate and the negative exit 0, all as expected; the run exits 1
   on two controls alone, "outcome_counts reads a fixture by its
   header" and "other_rows counts no row of the fixture". Planning
   reproduced the cause from the run script's lines: the control
   fixture `c-summary.tsv` is two tables parted by a blank line,
   written for `host_counts`, which exits at the blank; the two
   other helpers do not, and count the blank line and the second
   table's two rows as an empty outcome (" 3 error 1 ok 3 ", and
   3). On the fixture's first five lines they print "error 1 ok 3 "
   and 0. The three real `summary.host.tsv` files hold no blank
   line. Ruling: change TA (proposed as SA; the run generator
   refused that label, M3 part 7 P5's run script in its carried
   history naming an SA at its line 747, which planning read; no
   carried run script names a TA). The two controls read a
   one-table fixture, the two-table fixture's first five lines as a
   file of their own; the `host_counts` control keeps the two-table
   fixture; nothing else in the run script changes, and the d2
   report prints the diff of the two run scripts' line counts and
   the new script's sha256. The real-input helpers were right, so
   the three reruns, the three record pairs, the keep, the open
   check, the stage (0 differing, modes as ruled) and the zip
   (stored, every entry byte-equal, testzip ok, mode 600) stand as
   read from d1's log by planning; they are re-made by d2 all the
   same. The one FINDING line of d1 is the keep growth ruling K
   expects (pairs 2241, lo_in ok 2229, lo ok 2230 against the
   m6p6-open record's 2240, 2228, 2229); the gate, the negative and
   the scan print none.

O. The scan pre-check in a dry run. The dry driver read by planning
   (sha256 28ccda25) requires PLANNING_SPEC as its second argument
   and exports it, so d2's scan step runs inside the driver; d1's
   scan was run by hand with it after the driver stopped at the
   run, and with it the pin and the planning-file checks pass. The
   four "no" lines that remain are the dry layout's and are
   accepted for a dry run only: LOGDIR and RECDIR not the defaults
   (change AK); 3 of 58 names with a run directory (change LK); 8
   of 118 records scanned; and the home-directory line, every hit
   a dry LOGDIR or RECDIR path. A dry scan's RESULT: FAIL with
   exactly those four lines (the planted controls inside c-defined
   aside) is the expected dry result; any fifth line is a stop and
   a question. The logged run's scan pre-check prints none of them
   and RESULT: PASS.

P. Planning's word. d2 runs by the same driver into `<dry>/d2`;
   d1 stays in place until P1's report is sent (planning verifies
   from it), and both dry directories are named in the report as
   disk the maintainer reclaims by a method they name. Planning's
   word for the logged run is given now on these conditions, each
   read from d2's own logs: the run log ends "all as expected: yes"
   and exits 0; its only FINDING line is ruling K's keep growth with
   d1's totals; the gate and the negative pass as in d1; the scan
   pre-check prints the four lines of ruling O and no other. If d2
   meets them all, the logged run starts without a further message,
   the d2 report sent at its start with the four scan lines and the
   FINDING line grepped out; if any condition fails, nothing starts
   and the report comes first.

## Planning verification of P1 (2026-10-26)

Read from the primaries, not the report: the record commit a72467d
(parent 82c07fb, 124 files added, 118 records and the six files
under `records/logs/m7p1-p1/`, the message ASCII with the
Co-Authored-By line only, the count sentence carried); the gate,
negative and run logs each ending "checks all as expected: yes", the
only non-comment "no" line in each the API-key line; the one
FINDING line (run.log 5898) ruling K's keep growth, pairs 2241 and
lo_in ok 2229 and lo ok 2230 against the m6p6-open record's 2240,
2228 and 2229; 118 record files named `-2026-10-25-82c07fb.`, the
open pair among them; 58 run directories `-82c07fb-` under
`<corpus>/runs`; the stage 6020 files, directory 700, every file
600, the list 6018 rows (corpus 1560, fixture 2229, input 2229,
printarea's 6 error rows left out) at sha256 46cb9453; the zip
1075262198 bytes, mode 600, sha256 d32483dd, 6020 entries all
stored, testzip clean; the final scan PASS with 0 hits over 118
records and the pre-check PASS; no committed file holds the home
path, the maintainer's address or U+2014; check 1's answer verbatim
in the report but for its title line, which the report sets as a
heading. The old Milestone 2 stage and zip stand untouched beside
the new ones.

Rulings. Q. Check 1's wording point: ruling R's "every FINDING line"
means the lines the logs emit beginning FINDING, the scripts' own
comment and echo source lines aside, as both reports read it. R.
P1 is DONE. Dry d1 and d2 under `<phase>/dry` (160334494 and
159893566 bytes by the check's du) are the maintainer's to reclaim
with the old Milestone 2 stage and zip, by a method they name in
the implementing session after the new zip is copied (decision 3).
S. P2 waits on the maintainer: the zip to the Excel machine, the
opener run, `open-results.txt` and the machine section back, as
"The maintainer's check (P2)" says. The implementing session does
nothing until then.

## Planning rulings on the P2 results (2026-10-26)

Facts, from `open-results.txt` in the stage (sha256 9bf8bc30, 6018
rows, every row matched to the list in order, no non-ASCII byte, no
read-error or close-error cell) and the machine section the
maintainer gave (Microsoft Excel LTSC MSO, Version 2408 Build
16.0.17932.21056, 64-bit; Microsoft Windows 11 Home 10.0.26300;
2026-10-26): corpus 1528 pass and 32 fail; fixture 1864 pass and
365 fail; input 1874 pass and 355 fail; every fail with the one
generic open error; no repair prompt appeared. Identical files far
apart in the run always share an outcome (105 such hashes, none
mixed), so the refusals follow the file. The 32 corpus refusals:
the chart-surface sample on all 27 workbooks, the edit sample on 2,
protect on 2, remove on 1; every host row and every other sample on
those workbooks passes. None of the 32 rows carries a reference.
The 53 `seen` values compared all equal their `expect`. Fixture
pairs by (input, output): pass/pass 1842, refused/refused 333,
refused/pass 22, pass/refused 32; four of the 32 are the pairs
`specs/0015` Excel-check ruling 4 ruled. By hand (ruling T), the
maintainer reported the 32 refused corpus rows on 2026-10-26: Excel
offered a repair on every one; on the two protect rows the repair
then failed and the file did not open.

T. The template's by-hand fallback applies to the 32 refused corpus
   rows before the record is assembled. Opening each by the shell
   (a double-click, or `Invoke-Item` on the path) is the dialog's
   equivalent: both go through Excel's own open with its prompts.
   The outcome per file is `pass`, `repair` or `fail` as the
   template says, the row marked `(by hand)`; no value is read,
   since none of the 32 rows carries a reference; Excel's message
   stays out of the record. The maintainer reports the outcomes in
   the planning session, which carries them into the record.
U. A corpus row Excel offered to repair reads `repair`, the
   template's first clause, whether or not the repair succeeded;
   the two whose repair failed read `repair (by hand; repair
   failed)`, the others `repair (by hand)`. A result cell is read by
   its first word; the restored check (and any check on a result
   cell) matches a cell that begins with `pass`, `repair` or
   `fail`.
V. The record. Planning assembled it by a script,
   `assemble-excel-record.py` (sha256 54f344a7), from the stage's
   list and results file, the by-hand file `byhand-2026-10-26.tsv`
   (sha256 95a69b8a, 32 rows, columns `id_or_test`, `label`,
   `cell`) and the machine lines as `--excel`, `--os` and `--date`
   arguments; the record is 3845 lines, 844290 bytes, sha256
   47e6f360 (the full hashes in planning's brief), ASCII, no home
   path, no U+2014; planning read it for the maintainer's name, address,
   machine names and the laptop's address (none). The three files
   sit in `<home>/.cache/emend-build/m7p1-p2/record-from-planning/`
   (the phase directory exists with that directory alone in it; the
   driver tolerates it). The script stops with exit 2 and a line
   beginning `STOP:` on a results row that names no list row, a
   list row with no results row, a repeated results row, or a
   by-hand row that is not a refused corpus row. The script is not
   committed; the run log and the README print its sha256.
W. P2's logs, as "P2: the record" with these readings. The run log
   copies the record to `records/excel-check-m7-2026-10-26.md`,
   prints its sha256 equal to ruling V's; then runs the script with
   the stage's list and results file, the by-hand file and the same
   three arguments into a scratch file and prints that file's
   sha256 equal to the record's (the totals recomputed and every
   row's `bytes` and `sha256` shown to be the list's, in one); and,
   as the spec asks, a command that counts the record's table rows
   whose `bytes` and `sha256` pair is found in the list (3789 of
   3789 corpus and fixture rows; the fixture rows' input pairs 2229
   of 2229) and the totals line recomputed from the results file by
   `awk` equal to the record's. The negative log: the two assembly
   stops on scratch copies of the results file (a planted row,
   exit 2 "names no list row"; a removed row, exit 2 "list row with
   no result"); the restored check's two cases as ruling X. The
   record is written in the run log step only, so the driver's
   dirty-tree allowance stays " M specs/0044-excel-check.md" at its
   start; the record commit holds the record and the logs. No
   FINDING line is printed for the record's five findings: they are
   the record's content, carried to part 2; a FINDING line is for a
   stop, a mismatch or a count that differs.
X. The restored Excel-record check at P2: the close-out generator
   is written now (its numbered changes continuing from part 6's
   last), its push script generated into the close-out's directory
   and not run; P2's negative log extracts the check's function by
   a printed line range, prints the extract's sha256 equal to the
   script's lines, and runs it against a scratch clone of the tip
   with the record added and committed in the clone alone
   (accepted) and against a scratch clone of the tip without it
   (refused). Neither clone commit enters the repository. The
   close-out's pre mode then runs the check against the real tip
   once the record commit is in.
Y. The P2 doc commit holds this spec and `ROADMAP.md` (the P2 row
   IN PROGRESS), on a72467d, in the doc commit's form (rule GN).
   The independent check's two questions are the spec's; the report
   under `<phase>` prints the record's sha256, the totals line and
   the five findings' headings by sample or count, never a cell
   reference.

Z. P1's target root `<home>/.cache/emend-build/m7p1-p1/target` is
   pruned at P2's gate under the standing rule (`specs/0028` P2
   ruling 8), the two cargo directories under it, `main` and
   `consumer`, by `cargo clean --target-dir` from `<repo>`, `du -sb`
   printed before each; the `CACHEDIR.TAG` condition is printed as
   information for this prune, the root and those two directories
   carrying no tag (only `main/i686-unknown-linux-musl` does), the
   directories named here instead. Everything else under m7p1-p1
   stays (dry d1 and d2 per ruling R, keep-host, logs, scripts,
   superseded). The dry gate lists only (`PRUNE_PREVIOUS=list`).
AA. Ruling W's totals by `awk`: the record's totals take the by-hand
   file, so the `awk` reads the results file and the by-hand file,
   a by-hand row's cell's first word replacing that row's result,
   and prints the results-alone counts beside them as information
   (corpus fail 32 there, repair 32 in the record).
AB. Ruling W's pair counts are matched per source: the record's
   corpus rows' (`bytes`, `sha256`) against the list's corpus rows,
   its fixture rows against the list's fixture rows, and the fixture
   rows' input pairs against the list's input rows: 1560 and 2229
   (3789 of 3789), and 2229 of 2229.
AC. The close-out generator's change 20 block heading reads "no
   Milestone 2 Excel check record at the tip" where it read "no
   Excel check record at the tip", since the restored block expects
   this part's record there.

AD. The first logged attempt stopped at the gate (17:28:10Z to
   17:35:06Z): ruling Z's `cargo clean --target-dir` refused both
   roots, exit 101, "missing or invalid CACHEDIR.TAG", so nothing
   was removed (planning read the gate log, sha256 6bc56017, lines
   244 to 252; `du -sb` of P1's target still 6215683243; the 35
   names of m7p1-p1 all there; the tree " M specs/0044" alone). The
   attempt's files stay under `<phase>/superseded/logged-attempt-1`;
   the empty log directory the driver made was removed by `rmdir`,
   disclosed. The dry gate lists only, so a dry run cannot show the
   removal; the reading was planning's to check and it accepted
   the proposal, so the slip is shared. Ruling: the prune of the two
   names is by `rm -r "${PREV_DIR:?}/${n:?}"` in the generated gate
   (`specs/0043` ruling C's form; the form `specs/0028` P2 ruling 8
   names beside `cargo clean`), `du -sb` printed before, each name
   gone after and every kept name still there; no `CACHEDIR.TAG` is
   written and no `-f` is used. The gate gains a control that
   builds a target-shaped directory of its own under its build
   root and removes it by the same form, printed, so the dry gate
   (still `PRUNE_PREVIOUS=list`) shows the form removes. A new
   numbered change replaces change C90's clean. The four scripts
   are regenerated at the moved PLANNING_SPEC; a third dry run
   under `<phase>/dry/d3` is reported with every FINDING line;
   planning's word for the second logged attempt is given here on
   condition: d3's three logs "all as expected: yes" with exit 0,
   its scan the two dry-layout lines only, 0 emitted FINDING lines,
   the control's removal line printed; then the logged run starts
   without a further message, from the start, with RERUN_NOTE
   naming attempt 1 by its superseded path and RERUN_FILES its gate
   log. The maintainer is told of the method in the planning
   session.

## Planning verification of P2 (2026-10-26)

Read from the primaries, not the report: the doc commit ad139c3
(parent a72467d, two files at the hashes ruling Y's commit names,
the message ASCII with the Co-Authored-By line only); the record
commit e538e0a (parent ad139c3, 7 files added: the record and the
six under `records/logs/m7p1-p2/`, the message ASCII with the one
trailer, the count sentence carried); the record in the tree at
sha256 47e6f360; the gate, negative and run logs (8d21c89d, 7bd8e039,
c23251d0) each ending "checks all as expected: yes", the prune
lines "rm -r exit 0" for target/main and target/consumer (gate.log
250 and 251) and the R90 control (192), P1's target 0 bytes and its
35 names kept; 0 emitted FINDING lines (run.log 2284; gate and
negative 0); the final scan RESULT: PASS, the privacy range of 2
commits 0 findings; no committed file of the commit holds the home
path, the maintainer's address, a machine name or U+2014 (the one
`/home/` hit is the scan's own split control string, as in P1);
check 1's 69 non-empty lines verbatim in the report but for its
title line, set as a heading. Dry runs d1 to d3 and the stopped
attempt 1 verified before the logged run (ruling AD; the d2 and d3
readings: the three logs "yes", the scan's two dry-layout lines
only, 0 emitted FINDING lines, the R90 control printed in d3).

AE. Check 1's two points on the record's text, which stands as
   committed: in findings 1 to 4, "whose host row and every other
   sample pass" reads "every sample not named in findings 1 to 4":
   five of the 27 chart-surface workbooks also carry an edit,
   protect or remove repair (`054df34b1472`, `f8459281d739`,
   `18d1e8ba72b5`, `40d747fb5d9e`, `f3a462783621`); and on the
   record's line 39, "the list holds them" means the references,
   not the `seen` values. Part 2's scoping reads the findings with
   this ruling; the record is not amended.
AF. P2 is DONE 2026-10-26 (ad139c3, record e538e0a, local, not
   pushed). The dry directories d1 to d3 of P2 (about 117 MB each)
   and attempt 1's superseded files are the maintainer's to reclaim
   with P1's dry directories and the Milestone 2 stage and zip, by a
   method the maintainer names in the implementing session
   (ruling R). The close-out follows as "Close-out push (after P2)":
   the doc commit (this spec and `ROADMAP.md`, the P2 row DONE and
   the Close row IN PROGRESS), the generator regenerated at the
   moved PLANNING_SPEC with its push script in pre mode on a clean
   tree, the report to planning, planning's word, the maintainer's
   authorisation typed in the implementing session, the push, the
   CI run, the post scan, the record commit, one independent check,
   the report; `ROADMAP.md`'s part 1 heading then DONE and PUSHED.
   What part 2 carries: the record's findings 1 to 5 for decision
   2's per-finding choices, the assembly script and the by-hand
   file kept under the P2 phase directory (not committed), and the
   prune rule's wording (`specs/0028` P2 ruling 8: `rm -r` in the
   generated gate, `cargo clean` only where the root carries
   `CACHEDIR.TAG`, as ruling AD found).

## Decisions for the maintainer

Decided by the maintainer 2026-10-25: 1, 2 and 3 as recommended (A
in each), typed in the planning session.

1. The reruns: A (recommended), every sample rerun on the host at
   P1's phase commit, one commit for the whole list, about five to
   six hours unattended; B, each sample's newest run directory on the
   disk taken as it stands and only the gone ones rerun (`specs/
   0029` P5's form), the list spanning commits from 2026-10-02 to
   2026-10-24.
2. An output Excel refuses or repairs on an input Excel accepted, or
   a `seen` value that is not its `expect`: A (recommended), a
   FINDING carried to part 2's scoping, decided there per finding
   (fixed before the publish, or documented as a known limit); B,
   every such finding fixed before the publish, in a part inserted
   before part 2.
3. The Milestone 2 stage `<corpus>/work/excel-check-m2-2026-10-09/`
   and its zip (1273745890 bytes together): A (recommended), removed
   by the maintainer after this part's zip is on the Excel machine,
   by a method the maintainer names in the implementing session (no
   script of this part touches them); B, kept.

## Out of scope

- Any change to `crates/emend`, the tools or the scripts beyond a
  change a ruling names; fixing anything Excel refuses (decision 2);
  the i686 label's reruns; the scale tiers; the openpyxl comparison.
- Publishing, the tag, the manifest's `include` list, the CHANGELOG
  and the README (part 2); the corpus additions (part 3); the
  Milestone 2 ceiling (Milestone 7 decision 2).
- Fetching anything from the Excel machine by a session; any Excel
  automation beyond `scripts/excel-check-open.ps1` as committed.
- The xlsx-diff, passthrough and privacy-gate backlogs and the MSRV
  policy under "Unscoped".
