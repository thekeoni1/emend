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
