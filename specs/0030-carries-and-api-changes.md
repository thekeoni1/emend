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

## Planning rulings during P1 (2026-10-09)

Asked by the implementing session after the doc commit a1f5479
(three files at the hashes the body names, read back from the
commit's blobs by planning).

1. Item 2's "changes the script only if decision 1 takes B" and
   P1's "only under B" were planning's error: under A the runner
   gains the refusal (a script change) and B would have kept the
   removal with no script change. Both sentences read from now:
   "under A the runner refuses a reused run directory; under B
   the script does not change". Decision 1 is A.
2. The refusal's scope: per label. Before the loop, beside the
   existing per-label guard (line 173, an id already in
   `summary.<label>.tsv`), the runner checks every subdirectory
   of the run directory, selected or not, for `lo.<label>` or
   `lo-in.<label>` of THIS `--label`, and dies with exactly
   "error: the run directory holds an earlier run; name a fresh
   one" if any exists. The other label's conversion directories
   never trigger it, since a run directory holds both labels'
   passes by design.
3. Line 206 (the old 205): kept, rewritten in the ruled form
   `"${dir:?}/lo.$LABEL" "${dir:?}/lo-in.$LABEL"`; with ruling 2
   it removes nothing that existed before the run, and
   `specs/0001`'s bullet gains the dated sentence saying so. Line
   122's trap is rewritten `"${WORK:?}"` in the same commit, the
   part touching the script; the bullet's list of standing forms
   drops both lines. The negative log plants the old form on a
   scratch copy with the variable unset and shows the `:?` stop.
4. The run log's list invocation: `--run` the formula sample's
   directory `<corpus>/runs/2026-10-09-f39bc7b-formula`,
   `--run-plain` the printarea directory, `--keep <home>/.cache/
   emend-build/m3p7-p5/keep-host`, `--record` the committed
   `records/m3p7-open-2026-10-09-f39bc7b.txt`, `--out` a scratch
   file. The log prints the total row count, the printarea
   directory's share (21), the formula directory's share (27),
   the fixture and input shares, the stderr note "note: 6 error
   row(s) left out of 2026-10-09-f39bc7b-printarea", never a row,
   and the file's removal. The script writes no log of its own;
   "in the run log" means the run log prints what the script
   wrote to stderr.
5. The prune: of the six Milestone 3 part 7 roots only `m3p7-p5`
   holds a build target; its prune set is part 7 P5's (`dry-gate
   dry-negative dry-run dry-scan gate negative run target`),
   listed with `du -sb` and removed with `rm -r`, every other
   name kept (`keep-host` among them, ruling 4 reads it); for
   `m3p7-p1` to `p4` and `m3p7-close` the gate log lists each
   directory and asserts no prune-set name is in it. Planning's
   own `m3p7-p3-out-09d2e0f` is planning's to prune.
6. The note is printed only for a run directory with one or more
   error rows; a directory with none prints nothing, so the run
   log's one stderr line is the printarea note and the test log's
   fixture without a planted row shows an empty stderr.

## Planning verification and rulings on P1 (2026-10-09)

Verified by planning from git and the logs: five commits over
582dd2c (a1f5479 doc, 6d5df98 rules, 267e689 runner, 37ab66b
list script, 828b7dc record with six files at the reported
hashes); the runner's diff is the refusal before the loop by
`find -mindepth 2 -maxdepth 2` on `lo.$LABEL` and `lo-in.$LABEL`
with the ruled message, the trap and the loop line in the ruled
form, nothing else; `specs/0001`'s bullet drops the two lines and
carries the dated sentence; the list script's diff is item 1 and
ruling 6; every log ends all as expected or RESULT: PASS, each
line ending ": no" an expected form (the API key unset, the
diff-alone sentence, a planted control); the gate prunes
`m3p7-p5`'s eight names (4270182350 bytes) and finds none in the
other five roots; planning's own run of the committed list script
with ruling 4's arguments: exit 0, 2210 rows, the one note of 6;
planning's `privacy.sh` over 582dd2c..828b7dc: 5 commits, 1475
blobs, 0 findings; no API key in the environment.

1. The dry runs (check 1, Q3): the first test, negative and run
   dry runs ended "no" on defects of the log scripts (the error
   ids counted over every list row, where the formula directory's
   rows legitimately name those workbooks; the note form grepped
   in the docstring as well as the format string), corrected
   before any logged run; the logged runs are first attempts of
   the corrected scripts, kept under `dry-logs/attempt-1` in the
   phase cache. Accepted as a disclosure; `specs/0001` requires
   no log of a dry run.
2. N5's reading of ruling 3 (check 1, Q1): the old forms planted
   as `printf` of the extracted line, never run as `rm`, never
   printed verbatim, named by line and hash, the new forms run
   and stopped by `:?`. Accepted: it shows what the ruling asked
   (the widening and the stop) without a live `rm -rf` of the
   unruled form in any script.
3. The note's label for a `--run` directory is "host", the list's
   label for those rows (check 1's note): as item 1 says.
4. The doc commit's hashes (check 1, item 4): planning read the
   three blobs from the commit, equal to the hashes the body
   names.
5. P1 is DONE. P2 follows the spec: its doc commit carries this
   spec at its working hash, then the six API changes.

## Planning rulings during P2 (2026-10-10)

Asked by the implementing session after the doc commit 04b7070
(both files at the hashes the body names, read back from the
blobs by planning).

1. P1's prune set under `<home>/.cache/emend-build/m4p1-p1`:
   `dry-gate gate negative run test target`, listed with `du -sb`
   and removed with `rm -r`; every other name kept (`dry-logs`
   among them, P1 verification ruling 1 cites it), every name in
   exactly one set, as the P1 gate's form.
2. Said in advance and accepted: one independent check, the
   record commit "M4 part 1 P2 record: logs, the independent
   check"; one code commit per item, six, in the record's order;
   `Target` renamed at its definition in `package/sheet_read.rs`
   so the public path follows with no `as`; `rustfmt` on a
   touched file only, and only if a rename leaves it unformatted,
   the report naming any such file.
3. Item 6, a finding (asked after c68f350, 5e7bb62, f7bb23e,
   ff0a480 and cce4285): with `Archive::prefix_len` made
   `pub(crate)` the crate's non-test build warns it and the
   `Core::prefix_len` it delegates to are never used, since only
   `archive/tests.rs` calls it; the gate's clippy runs with
   warnings denied. Ruling: option A. Both methods gain
   `#[cfg(test)]`, the `Archive` one kept `pub(crate)`; no allow
   attribute; the rustdoc says the method exists for the crate's
   own tests. The review record's item 6 reads with this
   qualification ("the crate's own unit tests" is the whole set of
   callers). The size log expects 0 growth still: the method was
   never linked. The negative log's old-name case may use item 6
   as well as a rename: a test outside `cfg(test)` of the archive
   module calling `prefix_len` is refused.

## Planning verification and rulings on P2 (2026-10-10)

Verified by planning from git, the logs and its own check: eight
commits over 828b7dc (04b7070 doc; c68f350, 5e7bb62, f7bb23e,
ff0a480, cce4285 the five renames in the record's order; 8902f87
item 6; 84c40da record, five files at the reported hashes); the
diff 04b7070..8902f87 touches eleven files of the crate and
`emend-copy` only; no old name declared at the phase commit, each
renamed type's rustdoc naming its old name once; `prefix_len` on
`Archive` and `Core` under `#[cfg(test)]`, the `Archive` one
`pub(crate)`, the rustdoc saying why; every log ends all as
expected or RESULT: PASS, each line ending ": no" an expected
form; workspace tests 1112 on both targets, the crate's five
feature sets 514/526/558/573/918 on both; clippy and doc 0 lines;
the negative log's old-name and non-test-call plants refused
(E0433, E0599) with their controls; the gate prunes `m4p1-p1`'s
six names (5418344195 bytes); planning's own `cargo check
--workspace --all-targets --all-features` at 8902f87 in a
detached worktree clean, `cargo check -p emend` 0 warnings;
planning's `privacy.sh` over 828b7dc..84c40da: 8 commits, 1493
blobs, 0 findings; no API key in the environment.

1. The report's quoted rustc warning and "twice" (check 1, Q3):
   a session observation no committed log captures, labelled so
   in the report; ruling 3 records the finding and 8902f87 its
   fix, and the test log's clean clippy is the primary. Accepted.
2. "Every feature set the gate runs" (check 1, Q2) means the
   crate's five sets the gate's clippy and doc steps use; the
   gate's clippy over `size-probe`'s feature sets is the gate's
   own and not a test set. The test log is as the spec meant.
3. NG2's plant, a non-test public method in `archive.rs` built
   with `cargo build` (check 1, item 3): the call from outside
   `cfg(test)` ruling 3 asked for; accepted.
4. The test log's first dry run, ended "no" on a control's leaked
   grep message, corrected before the logged run (check 1, item
   4): accepted as a disclosure, as P1 verification ruling 1.
5. P2 is DONE. P3 follows the spec: its doc commit carries this
   spec and `ROADMAP.md` at their working hashes; then the size
   log and record, the listing rerun and its record, the harness
   runs, one independent check.

## Planning rulings during P3 (2026-10-10)

Asked by the implementing session after the doc commit 669fa91
(both files at the hashes the body names, read back from the
blobs by planning).

1. The listing's figures (Q1): the spec's "545 items, 545 keep"
   and "every row identical but the five names and the one
   missing method" were planning's arithmetic from the old
   record without the program's rules in view: the listing holds
   `pub` items only, so the hidden method is gone (544), and a
   rename moves the rows the renamed container owns and the
   re-export rows that named it. The record and the log expect
   544 items, 544 keep, 0 rename, 0 hide; rows equal to part 7's
   by (module, kind, name, container, cfg, added) but exactly
   eleven changed and one gone, each printed by its key: the five
   renamed types with `added` at their P2 commits, the four
   fields under the renamed containers, the two re-export rows,
   and `Archive.prefix_len` gone. "The API changes" reads with
   this ruling. The trial that found it was unlogged, in scratch,
   and is disclosed here and in the report; the logged listing
   is the test log's. Planning's own run of the same program
   (59f4043c) over `git archive 669fa91` in its scratch directory
   gave 544 rows, twelve old-only and eleven new-only by that
   key, the rows this ruling names.
2. The listing goes in a test log as part 7 P3's (T1 the listing
   and its per-file pub-line counts, T2 the all-features doc
   build with warnings denied); the phase's logs are gate, test,
   size, run, scan.
3. The formatter: part 7's `make_review.py` copied into the phase
   directory, its verdict table emptied (every item keep), its
   spec map gaining "M4 part 1" to `specs/0030` for the five
   renamed rows, its header prose rewritten for this record (the
   rerun after P2, the counts per verdict, the eleven and one
   listed); run from the test log's listing, its hash printed in
   the test log and the record's header.
4. Said in advance and accepted: the records' shortsha 669fa91;
   every corpus run with `--libreoffice`, compared as part 7 P3's
   run log to the 09d2e0f and f39bc7b directories, printarea's
   six refused the same; the size log as part 7 P3's with the base
   rebuilt at 09d2e0f and growth 0 required on every probe;
   `passthrough-record.py` with `--allow-single` for the 23
   host-only samples.

## Planning verification and rulings on P3 (2026-10-10)

Verified by planning from git, the logs, the records and its own
build: two commits over 84c40da (669fa91 doc, the phase commit;
ed4082d record, 62 files at the reported hashes); every log ends
all as expected or RESULT: PASS, each line ending ": no" an
expected form; the size record: 0 of 28 probes grew or shrank,
the budget 436992, 21760 under the ceiling; the listing record:
544 items, 544 keep, the log's twelve gone and eleven new by key
equal to planning's own run of the program at 669fa91 (P3 ruling
1); the run log: the four samples on both targets and the 23 host
samples each 27 outputs, lines and diff tables equal to the
earlier run's, printarea 21 equal and the six refused the same;
planning's own build at 669fa91 in its detached worktree: no old
name declared, `prefix_len` twice under `cfg(test)`, the crate's
five feature sets 514/526/558/573/918 and the workspace's 1112 on
both targets, clippy and doc 0 lines, and its own release
`emend-copy` run over the corpus on both labels for the four
samples, 27 of 27 outputs and tool lines identical to the phase's
run directories each; planning's `privacy.sh` over
84c40da..ed4082d: 2 commits, 1539 blobs, 0 findings; no API key
in the environment.

1. The unlogged scratch trial of the listing program, before the
   log scripts were written (check 1): disclosed by P3 ruling 1
   and by the report's final form; the logged listing is the test
   log's. Accepted.
2. `probe-emend-read-write-xml`'s verdict "code" (312 differing
   bytes, 88 in `.text`, 163 in `.data.rel.ro` as slot deltas of
   +8, +2 and +6, every figure equal to part 7's): the six P2
   commits touched the probe's own sources, and a rename moves
   the type names the compiler embeds, which is what slot deltas
   of a few bytes look like; attributed to the six commits by
   file, as decision 2 A allows. Growth 0 holds on every probe.
   Accepted as the record states it.
3. Six listing rows carry `specs/0030`, the crate root's
   re-export row among them (its `added` moved to f7bb23e): the
   formatter maps by commit, and that row changed in P2. Accepted;
   "the five renamed rows" in P3 ruling 3 reads as six.
4. The record's doc-build sentence names "types, functions,
   constants and aliases" where the test log's line names
   statics too: the test log is the primary and the record's
   sentence is a summary that leaves one kind out; noted, no
   commit for it. The draft's "ruling 4 (a)" and the joined scan
   lines were corrected in the final report.
5. The dry runs, all passed first time, the run dry run's staged
   pair moved aside before the logged run: session facts;
   accepted as a disclosure, as P1 and P2.
6. P3 is DONE, and with it part 1's phases. The close-out push
   follows the Close-out section: the doc commit holds this spec
   and `ROADMAP.md`; the scripts are generated from part 7's by
   the same generator with the base ebec51d by its subject,
   `PREV_RECORD_PATTERN` `^M3 part 7 close-out record` once, the
   tip's subject "M4 part 1 close-out: commit planning docs (the
   P3 verification and rulings)" by change 21's form, no
   Excel-record check and the stray-record count 0 kept; pre
   mode on the clean tree, one independent check, the push on
   planning's word and the maintainer's authorisation typed in
   the implementing session, the CI run, the post scan, the
   record commit with the three logs only.

## Planning rulings before the close-out (2026-10-10)

Asked by the implementing session after the scripts were
generated (gen.py a3017138, push b28ca4a5, ci 9bfc4432, scan
5b491376; read by planning: `bash -n` clean, no non-ASCII byte,
the base pattern resolving to ebec51d alone and the previous
record pattern to 582dd2c alone, the lines differing from part
7's scripts below their headers the names, patterns and part-7
wording alone).

1. The generated scripts are accepted as the close-out's.
2. The pre and push modes run with `CARGO_TARGET_DIR=<home>/
   .cache/emend-build/m4p1-close/target` in their environment,
   printed in the log's environment lines, so the gate's builds
   have the close-out's own target root as `specs/0001` asks; the
   carried script text does not change.
3. P3's prune set (`dry-gate gate test target` under
   `<home>/.cache/emend-build/m4p1-p3`, `dry-logs` and the rest
   kept) is left to Milestone 4 part 2's first gate, as part 7's
   close-out left P5's to this part's P1: the push script has no
   prune block and gains none.

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
