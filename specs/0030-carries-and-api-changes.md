# 0030: Milestone 4 part 1, the carries and the API changes

Status: APPROVED by the maintainer 2026-10-09 under the Milestone
4 outline chosen the same day ("Release readiness", the
recommended option of four in the planning session's question),
with the Milestone 2 Excel check skipped again by the maintainer's
word of the same day ("skip excel check", the fourteenth
deferral): it gates no push in this milestone and stands as a
pending acceptance item, its stage kept. Decisions 1 and 2 taken
as A, the recommended option of each, typed in the planning
session. Sent to the implementing session the same day.

Context: `CLAUDE.md` ("The fidelity harness is the acceptance
instrument"; "Two sessions, one repository"), `ROADMAP.md`
("Milestone 4", the outline; "Unscoped"), `specs/0001-
bootstrap.md` ("Rules for every phase"), `specs/0029-engineering-
close.md` (P1 ruling 1: the `rm -rf` line-205 question; P3
verification rulings 4 and 5: the six API changes and item 9's
attribution; P5 verification ruling 4: the printarea sample's six
error rows and the list script; the close-out rulings 5 and 6),
`records/api-review-m3p7-2026-10-09-09d2e0f.md` ("What the review
would change before publication, in order"), `specs/0025-sheet-
and-print-settings.md` (the close-out verification: the six
workbooks refused with the overlap message), `specs/0016` (P1
ruling 1: what the list script refuses). No external text is
cited.

## Goal

Milestone 4 part 1 leaves the crate's public surface as the API
review said it should be before publication, with every sample's
output unchanged by it, and clears the four items Milestone 3
carried: the Excel list script's handling of a run directory with
error rows, the `rm -rf` line-205 question, the size log's
attribution note, and the record commit 582dd2c carried into the
next push. No feature is added; the core's size does not move
(renames and a visibility change compile to the same code, and
the size log proves it).

## Size

The budget figure after Milestone 3 is 436992 section bytes
against the ceiling 458752 (`records/size-m3p7-2026-10-09-
09d2e0f.md`): 21760 of headroom. Every probe's growth in this part
is expected at 0 and accepted up to 0; a nonzero growth is a
finding to attribute before P3's record is written. The size
record `records/size-m4p1-<utc-date>-<shortsha>.md` is written in
P3.

## The carried items

1. The Excel list script, `scripts/excel-check-list.py`: a run
   directory with error rows (a corpus output absent because the
   tool refused the input, exit 2, the summary row `error`) is
   accepted by `--run` and `--run-plain`; the error rows are left
   out of the list and their count per run directory is printed
   to stderr as "note: <n> error row(s) left out of <label>" and
   in the run log; a run directory whose every row is an error is
   refused ("error: a run directory holds no ok row"). The six
   error rows of the printarea sample at f39bc7b are the six
   workbooks `specs/0025` records as refused with the overlap
   message (054df34b1472, 40d747fb5d9e, 6778a242d1f9,
   75387404b8af, ed4c55f07c69, f8459281d739), an expected refusal
   and not a finding against emend; with item 1 the printarea
   sample joins any later list by its 21 ok rows.
2. The `rm -rf` line-205 question (`specs/0029` P1 ruling 1):
   `scripts/passthrough-run.sh` line 205 removes an earlier run's
   conversion directories under the run directory, which existed
   before the run; decision 1 settles what a rerun into a run
   directory may remove, and P1 writes the answer into
   `specs/0001`'s `rm -rf` bullet as a dated sentence and changes
   the script only if decision 1 takes B.
3. Item 9's attribution (`specs/0029` P3 verification ruling 5):
   the size log attributes a probe's growth to every commit that
   touched a file the probe compiles; a finer method, by the items
   a probe reaches, is noted under `ROADMAP.md` "Unscoped" and
   stays there (decision 2 A) or is specified in P3 (B).
4. The record commit 582dd2c (Milestone 3 part 7's close-out
   record) goes with this part's push; the push script names it
   by `^M3 part 7 close-out record`, in the range exactly once,
   its base ebec51d by its subject.

## The API changes

The six changes of the review record, in its order, each in P2:

1. `emend::cells::Item` renamed `SharedStringItem`.
2. `emend::cells::Items` renamed `SharedStringItems`.
3. `emend::cells::Kind` renamed `CellKind` (the crate root's
   re-export follows the name).
4. `emend::package::Default` renamed `ContentTypeDefault`.
5. `emend::package::Target` renamed `HyperlinkTarget` (its public
   path; the record's correction 1).
6. `Archive::prefix_len` made `pub(crate)`.

No alias, deprecation shim or re-export under the old name is
kept: nothing is published yet, so there is no consumer to keep
compiling. Every use inside the workspace (the crate, its tests,
the tools) follows the rename; `cargo fmt` runs on no file the
rename did not touch. The rustdoc of each renamed item names the
old name once ("formerly `Kind`") so the review record's rows
stay findable. The API listing of part 7 P3 is rerun as
`records/api-review-m4p1-<utc-date>-<shortsha>.md` with the same
formatter, expected 545 items, 545 keep, 0 rename, 0 hide, every
other row identical to the part 7 record's but the five names and
the one missing method.

## Harness runs

P3 reruns on both targets part 6's four samples (`--copy-sample`,
`--anchor-sample`, `--remove-image-sample`, `--commentbox-sample`)
over the corpus, byte-identical by sha256 to part 7 P3's run
directories, and on the host every sample part 7 P5 reran (the
23 of `records/m3p7-p5-*`), byte-identical to those directories;
recorded as `records/m4p1-p3-<sample>-<utc-date>-<shortsha>.md`
and `.txt`. P1's run log writes the list once from the printarea
sample's f39bc7b directory to a scratch file (the row count and
the error-row note, never a row) and deletes it. No other corpus
run.

## Phases

Each phase ends as `specs/0001` says: the doc commit, the code
commit(s), one record commit with the logs and the check answers;
logs in `records/logs/m4p1-<phase>/`; the standing prune rule
(one target root per phase, Milestone 3 part 7's targets listed
and removed at P1's gate); the scan log last; at most two
independent checks; report; STOP. No push during the part; the
close-out push follows P3 and carries 582dd2c.

Part 1's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0029` as planning left them after Milestone 3 part 7's
close-out (the close-out rulings 1 to 6 and the Milestone 4
outline), and this spec, at command-substituted hashes.

### P1: the list script and the two rulings

The doc commit; then `scripts/excel-check-list.py` as item 1
says, with its own negative cases in the negative log (a run
directory of error rows only, refused; a planted error row on a
copied directory, left out and counted); `specs/0001`'s `rm -rf`
bullet gains decision 1's sentence and `passthrough-run.sh`
changes only under B (item 2); `ROADMAP.md` "Unscoped" reads as
decision 2 leaves it (item 3). The test log: the script on a
fixture run directory with and without a planted error row, the
counts shown. The run log: the list written from the printarea
directory, 21 rows and the note of 6, the file removed. The gate,
with the prune first. The scan log.

### P2: the API changes

The doc commit if P1 left rulings; then the six changes in one
code commit per item or one for all (the implementing session's
choice, said in the report), the workspace's every use following;
the test log on both targets with every feature set the gate
runs; clippy and doc 0 lines; the gate; the negative log: a
scratch build with one old name restored in a test, refused by
the compiler with the error line counted. The scan log.

### P3: the samples, the size and the listing

The doc commit if P2 left rulings; the size log with every probe's
growth 0, the size record; the API listing rerun and its record;
the harness runs above, byte-identical; one independent check;
the scan log.

## Close-out push (after P3)

As Milestone 3 part 7's (`specs/0029` "Close-out push (after P4)"
as amended and its close-out rulings), with these differences:
the doc commit holds this spec and `ROADMAP.md`; the base is
Milestone 3 part 7's close-out doc commit ebec51d, named by its
subject ("M3 part 7 close-out: commit planning docs (the P5
verification and rulings, decision 9 deferred a thirteenth
time)"); `PREV_RECORD_PATTERN` is `^M3 part 7 close-out record`
(582dd2c), in the range exactly once; the close-out doc commit's
subject is "M4 part 1 close-out: commit planning docs (the P3
verification and rulings)", checked by change 21's form; no
Excel-record check, the stray-record count 0 kept; the record
commit "M4 part 1 close-out record: the push, the CI run and the
scan" holding the three logs only. `ROADMAP.md`'s part 1 rows
read DONE and PUSHED.

## Decisions for the maintainer

Decided by the maintainer 2026-10-09: 1 and 2 as recommended (A in
each), typed in the planning session.

1. A rerun into an existing run directory (item 2): A
   (recommended), the runner refuses a run directory that holds
   an earlier run's conversion directories ("error: the run
   directory holds an earlier run; name a fresh one"), so the rule
   has no exception and line 205 removes only what the same run
   made; B, the removal kept and the rule's bullet gains the
   exception, listed by file and line.
2. The size log's attribution (item 3): A (recommended), the
   per-file method stays, the finer method stays under
   "Unscoped"; B, P3 specifies attribution by the items a probe
   reaches, from the probe's own `use` lines.

## Out of scope

- The Milestone 2 Excel check (skipped by the maintainer
  2026-10-09; the stage `<corpus>/work/excel-check-m2-2026-10-09/`
  and its zip kept; the push script's Excel-record check, tested
  in `records/logs/m3p7-p4/negative.log`, is restored by the part
  whose push the maintainer makes it gate).
- `CHANGELOG.md`, the package metadata, `publish`, any registry
  (part 2).
- Charts; a second crate; the xlsx-diff, passthrough and
  privacy-gate backlogs under "Unscoped".
- Any widening or narrowing of an item the review kept.
