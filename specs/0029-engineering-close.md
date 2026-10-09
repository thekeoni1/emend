# 0029: Milestone 3 part 7, the engineering close

Status: APPROVED by the maintainer 2026-10-08: the scope chosen
first ("Core close only (Recommended)", in the planning session's
question: the items carried from parts 1 to 6, the MSRV policy,
the API review and the README; the xlsx-diff, passthrough and
privacy-gate backlogs stay under `ROADMAP.md` "Unscoped"), then
decisions 1 to 4 each taken as A, the recommended option, in the
planning session's second question. Sent to the implementing
session the same day.

Context: `CLAUDE.md` ("The fidelity harness is the acceptance
instrument": a README sentence about fidelity, speed or size cites
a harness run in the records; "Dependencies and licences"; "32-bit
and sizes"), `ROADMAP.md` ("Milestone 3", the outline approved
2026-10-01, item 7: MSRV policy, the backlogs, a README whose every
fidelity, size and speed sentence cites a record, an API review
before the crate is published anywhere; "Unscoped"),
`specs/0001-bootstrap.md` ("Rules for every phase", which P1
amends), `specs/0023-scale.md` (close-out rulings 2 and 3: the
runner's `leftover` column, the record script's header hashes),
`specs/0024-rich-text.md` (P2 ruling 1: the `rm -rf` form; P4
verification ruling 5: a cause per probe), `specs/0025-sheet-and-
print-settings.md` (close-out ruling 3), `specs/0028-images-and-
comments-second-half.md` (P2 ruling 2, F2; P2 ruling 8 and P3
ruling 10, the target root; P3 ruling 11, T2; P3 verification
rulings 2 and 3; P4 rulings 3 and 7; P4 verification rulings 1, 2
and 6; the close-out rulings 3 and 4; decision 9), `specs/0015-
open-checks.md` ("The Excel check"). No external text is cited.

## Goal

Milestone 3 ends with the repository in the state the outline
promised: every rule the parts added to their own specs stands in
`specs/0001` where the next milestone reads it; the committed
scripts obey the `rm -rf` rule; the size log attributes growth to
the commits that touched each probe's own sources; the toolchain
pin, the crate's `rust-version` and the CI pin agree and a policy
says when they move; every public item of the crate is listed and
judged in a record before anything is published; a README exists
whose every fidelity, size and speed sentence cites a record that
exists at the commit it is read at; and the Milestone 2 Excel
check, deferred twelve times, is run before this part's push. No
feature is added; the core does not move beyond the one tidy in
P2, and that one under the feature `copy`.

## Size

The budget figure after part 6 is 436992 section bytes against the
ceiling 458752 (`records/size-m3p6-2026-10-08-9cfd92e.md`): 21760
of headroom. This part's P2 tidy sits in `check_copy_relationships`
under `copy`; its growth on `probe-emend-copy-write-xml` is
measured in P3's size log and expected at 0 to 200; the core's
growth is expected at 0 and accepted up to 100. The size record
`records/size-m3p7-<utc-date>-<shortsha>.md` is written in P3 with
the finer attribution below and stands as Milestone 3's closing
figure.

## The carried items

Each item names its source; nothing here is new.

1. `specs/0001`, the gate's environment line (`specs/0028` P4
   verification ruling 1): the gate log prints, before anything
   else, the root's mount propagation (`findmnt -no PROPAGATION /`,
   expected `shared`, since podman's first call warns on the
   script's standard error otherwise) and the keepalive's presence
   (the `sleep 2147483647` process the host's scheduled task
   starts, counted by `pgrep -fc`, expected 1), each as a check
   with a control. P1 writes the rule; P1's own gate log is the
   first to print the line.
2. `specs/0001`, one target root per phase (`specs/0028` P2
   ruling 8, P3 ruling 10, P3 verification ruling 6, P4
   verification ruling 6): under `<home>/.cache/emend-build/
   <phase>/target`, one subdirectory per source tree (`main`,
   `plant`, `negative`, `consumer`, one more per further tree),
   the same profiles for every tree; no two scripts build at once
   in a phase's dry or logged runs; the previous phase's build
   targets are listed with `du -sb` and removed with `rm -r`
   (never `-f`) at the next phase's first gate that builds, each
   named in the gate log, and never `superseded/`, the logs,
   `gen/`, `scripts/`, `FACTS.md`, `logged.sh`, `logged.out` or
   the message files (anything a committed log cites by hash).
   Planning's own script keeps one worktree and one target under
   its cache and removes the previous phase's at its start the
   same way. P1's gate prunes part 6 P4's targets under this rule.
3. `specs/0001`, the `rm -rf` form (`specs/0024` P2 ruling 1, the
   maintainer's decision of 2026-10-02): `rm -rf` only on a
   directory the same script created in that run, written
   `"${var:?}"` or `"${var:?}/<name>"`; never on anything that
   existed before the run. P1 writes the rule and brings the one
   committed form to it: `scripts/counting.sh` line 158's control
   on its own `mktemp -d` directory, `rm -rf "$d"`, becomes
   `rm -rf "${d:?}"`. That is a gate-file change, so P1 runs
   `scripts/gate-self-test.sh` and commits `self-test.log`.
4. `specs/0001`, the sessions' environment (`specs/0028` P4
   verification ruling 1, and the three ssh-drop deaths of
   2026-10-07): a session launched over ssh dies with its
   connection, so a logged run is started detached (`setsid
   nohup`), writes its exit code to a file, and is checked by the
   log, not by the session's memory; the keepalive and the
   propagation are the environment line's business, item 1.
5. The harness scripts (`specs/0023` close-out rulings 2 and 3):
   the record `scripts/passthrough-record.py` writes gains in its
   header the two binary hashes the runner writes into each run
   directory's `run.<label>.tsv` (`passthrough_sha256` and
   `xlsx_diff_sha256`, one pair per label), and
   `scripts/passthrough-run.sh`'s `leftover` column counts
   `TMPDIR` as well as the run directory, each a recorded choice
   here, not a question for the maintainer: the hashes are added;
   `leftover` counts both and the runner's comment says so. P1,
   with the test log showing the header on a fixture run and the
   column on a run whose `TMPDIR` holds a planted file.
6. The copy's relationship check (`specs/0028` P3 ruling 11, T2;
   P3 verification ruling 2): `sheetPr` and `sheetView`, whose
   `codeName` and `tabSelected` the copy strips before the check
   sees them, are routed through `check_copy_relationships` as
   every other kept start tag is, so a relationship attribute on
   either (none the schema allows, none in the corpus) is refused
   with the existing message rather than copied. No behaviour
   changes on valid input; one test on a byte-built part whose
   `sheetView` carries an `r:id` shows the refusal, and the full
   copy test's bytes do not move. P2.
7. The two sentences in `specs/0028` (P3 verification ruling 2,
   P4 ruling 3): in "The copy", "the `r:id` attribute of
   `pageSetup`" reads "the relationship attributes of `pageSetup`
   (the schema gives it `r:id` alone)"; in "Size", the images and
   comments probes' sentence reads as P4 ruling 3 states it (the
   removal written into memory and reopened, `images_removed`
   printed; the box set on AB10 in the addition's write and read
   back). P2's doc commit, as amendments dated and signed by this
   spec, the original lines left in place above them.
8. F2, the emptied relationships part (`specs/0028` P2 ruling 2):
   a write that removes the last relationship of a rels part it
   edits leaves a `Relationships` root with no child; the
   catalogue states that it is kept. Decision 2 below: kept
   (recommended), or dropped with its `Override`. If dropped, P2,
   under `images`, with the removal sample rerun in P3 and its
   seven `added` rows gone.
9. The size log's cause attribution (`specs/0024` P4 verification
   ruling 5, `specs/0025` close-out ruling 3, `specs/0028` P4
   ruling 7 and P4 verification ruling 2): the size log names,
   for each probe whose figure moved, the commits in the range
   whose diff touches a path among that probe's own sources
   (the crate's files that the probe's feature set compiles, read
   from the feature's `cfg` lines by a command the log prints),
   rather than every commit that touched the crate; a probe whose
   figure moved with no such commit is reported as "no cause
   among the sources" and is a finding. P3's size log is the
   first to use it, over the range from part 6's phase commit
   9cfd92e, and reports part 6's open question (where the removal
   walk's bytes went) as far as the range answers it: it does not,
   since part 6's commits precede the range, and the log says so;
   the method is what this part delivers.
10. The Milestone 2 Excel check (`specs/0028` decision 9, deferred
    the twelfth time 2026-10-08): a hard requirement of this part's
    close-out push, "The Excel check" below.

## MSRV (decision 1)

Today the toolchain is pinned three times and the crate declares
nothing: `rust-toolchain.toml` (`1.98.1`, the i686 target), the CI
workflow (`rustup toolchain install 1.98.1`), and `specs/0001`'s
rule (`RUSTUP_AUTO_INSTALL=0`, the pinned channel a separate
toolchain from `stable`). The workspace `Cargo.toml` carries
`edition = "2024"` and no `rust-version`. Option A (recommended):
`rust-version = "1.98.1"` in the workspace package table,
inherited by every crate; the policy, written into `specs/0001`,
is that the four agree (the toolchain file, the CI pin, the
workspace field and the rule's text), that the gate's hygiene step
checks the first three agree by reading them (with a control that
plants a disagreement), and that a move is a recorded decision in
the spec of the part that moves it, never a side effect of a
toolchain install. The crate builds on no older compiler than the
pin, which is what the field says. Option B: `rust-version` set to
the oldest compiler the crate builds with, found by trial, and
kept below the pin: a second toolchain to install and run the
tests on, outside this part's size.

## The API review (decision 3)

The record `records/api-review-m3p7-<utc-date>-<shortsha>.md`,
written by hand in P3 from a listing the test log prints: every
`pub` item of `crates/emend` reachable from the crate root (types,
functions, methods, fields, constants, the features that gate
them), listed by module by a command the log prints over the
sources (a `pub` line count per file with a control, and the
`cargo doc` build with no warnings, whose HTML index the record
cites by the doc log's count of documented items). Each item gets
one line: its name, the feature, the spec that added it, and a
verdict: keep; rename before publication (the proposed name);
hide (why nothing outside the crate needs it); widen or narrow
(what and why). The record ends with the counts per verdict and
with the items the review would change before publication, in
order. Option A (recommended): the record alone; no rename, hide
or widening is applied in this part, so the public surface that
Milestone 3's records measured is the one the milestone ends
with; the changes go to the next milestone's first part as its
carried items. Option B: the renames and hides applied in P3
under a second code commit, with the tests and the tool following
and every sample rerun in P4, which is a behaviour change on a
closing part and moves every size figure after the record.

## The README (decision 4)

`README.md` does not exist. P4 writes it. Option A (recommended):
a short file: what emend is in `CLAUDE.md`'s words (the openpyxl
comparison with its four exclusions); the core's two dependencies
and the features, each named with its spec; how to build and test
on both targets; the three measured sentences, each citing the
record that holds the figure by its repository path: fidelity (the
passthrough count over the corpus and the sample runs, citing part
6's four sample records and the open record), size (the budget
figure and the per-feature costs, citing the size record P3
writes), and speed (the scale record's tier figures, citing
`records/scale-m3p1-2026-10-02-6578925.md`); what is out by
design; and the licence. Option B: the same with a worked example
per feature, which means doctests the gate runs and the fixtures
they need, outside this part's size. Either way the run log in P4
checks the README: every sentence holding a digit cites a path
under `records/` that exists at HEAD (a command the log prints,
with a control on a planted sentence), and the file holds no
U+2014, no absolute path and no corpus content.

## The Excel check

The check is `specs/0015` "The Excel check" as the parts since
have grown it: the list `scripts/excel-check-list.py` writes from
the run directories named in the specs' "Harness runs" sections
(every part's samples through part 6's four), carried by the
maintainer to a machine with Excel, each row opened, the result
recorded from `records/excel-check-template.md` as
`records/excel-check-m2-<date>.md` (Milestone 2's acceptance; the
Milestone 3 rows on the same list are Milestone 3's). It is a hard
requirement of this part's close-out push: the push script's pre
mode checks that a file matching `records/excel-check-m2-*.md`
exists at the tip, holds a result line, and is named in the
close-out doc commit's message, with a control that refuses a tip
without one. The list is written in P4's run log (the paths and
the row count, never the rows, which name corpus cells: `specs/
0015` P2 ruling 7); the maintainer runs the check between P4's
verification and the close-out doc commit, and the record is the
doc commit's third file. Planning does not ask a thirteenth time:
without the record there is no push.

## Harness runs

P3 reruns part 6's four samples (`--copy-sample`,
`--anchor-sample`, `--remove-image-sample`, `--commentbox-sample`)
on both targets over the corpus, byte-identical to part 6 P4's
run directories by sha256 unless decision 2 takes B, in which case
the removal sample's rows read as item 8 says and the other three
are byte-identical; recorded as `records/m3p7-p3-<sample>-<utc-
date>-<shortsha>.md` and `.txt`. P4's run log holds the README
check and the Excel list's writing. No other corpus run.

## Phases

Each phase ends as `specs/0001` says: the doc commit, the code
commit(s), one record commit with the logs and the check answers;
logs in `records/logs/m3p7-<phase>/`; every rule the earlier parts
added, and from P1 the rules P1 writes into `specs/0001`; the scan
log last; at most two independent checks; report; STOP. No push
during the part; the close-out push follows P4 and carries the
record commit bafd780 of part 6's close-out with this part's.

Part 7's first doc commit carries, unedited: `ROADMAP.md` and
`specs/0028` as planning left them after part 6's close-out (the
close-out rulings 1 to 5, the Close row and the part 6 heading),
and this spec, at command-substituted hashes.

### P1: the rules and the scripts

The doc commit; then `specs/0001` "Rules for every phase" gains
items 1 to 4 as four bullets dated and signed by this spec (the
existing bullets unchanged); `scripts/counting.sh` line 158's form
(item 3); `scripts/passthrough-record.py`'s header hashes and
`scripts/passthrough-run.sh`'s `leftover` column (item 5). Tests:
the test log runs the record script on a fixture run directory
and shows the two hashes in the header; runs the runner on a
fixture with a file planted under `TMPDIR` and shows `leftover`
counting it, and 0 with nothing planted; shows `counting.sh`'s
control refusing an unset `d` (the `:?` form stops the command)
beside the directory removal on a set one. The gate: the
environment line first (item 1), then part 6 P4's targets listed
and pruned (item 2), then as part 6's P4, and `self-test.log`
(item 3). The negative log: a planted `rm -rf` on an unset
variable in a scratch copy of the library, stopped. The scan log.

### P2: the tidy and the amendments

The doc commit (the P1 verification rulings; item 7's two
amendments in `specs/0028`); then item 6 under `copy`, and item 8
if decision 2 takes B. Tests as item 6 says, and for B: a removal
leaving a rels part empty drops it with its `Override`, the test
on a byte-built package. The gate as P1's without the self-test
unless a gate file changes. The negative log through the binary:
`--copy-sample` on a fixture whose `sheetView` carries an `r:id`
(refused, with a control). The scan log.

### P3: the record, the size and the review

The doc commit (the P2 verification rulings); then decision 1's
`rust-version` and the hygiene check with its control (a gate
file changes: the self-test); the size log with item 9's
attribution and the size record; the API review record (decision
3); the four samples rerun ("Harness runs"). The test log prints
the `pub` listing and the doc build. The gate, the negative log
(the hygiene check's control: a planted disagreement between the
toolchain file and the CI pin, refused), the run log, the scan
log; one independent check on the review record's listing against
the sources (does every `pub` item appear once; is every verdict
one of the four).

### P4: the README and the list

The doc commit (the P3 verification rulings); then `README.md`
(decision 4) and the close-out push script's Excel-record check
(it is a carried script; its change is printed in the close-out
log as every close-out's changes are). The run log: the README
check and the Excel list's writing with the row count. The gate
(the README counts as a documentation file for hygiene: no
U+2014, ASCII or UTF-8 with no control bytes), the negative log
(the README check on a planted uncited sentence, refused), the
scan log. Then planning's verification; then the maintainer's
Excel check; then the close-out.

## Close-out push (after P4)

As part 6's (`specs/0028` "Close-out push (after P4)" and its
close-out rulings), with these differences: the doc commit holds
this spec, `ROADMAP.md` and the Excel check record; the base is
part 6's close-out doc commit 046cfb8, named by its subject ("M3
part 6 close-out: commit planning docs (the P4 verification and
rulings, decision 9 deferred again)"); `PREV_RECORD_PATTERN` is
`^M3 part 6 close-out record` (bafd780), in the range exactly
once; the pre mode's Excel-record check is in force; the scripts
run from `<scratch>/close` (part 6's close-out ruling 1); the
record commit's subject "M3 part 7 close-out record: the push, the
CI run and the scan". `ROADMAP.md`'s Milestone 3 heading reads
DONE and PUSHED with the Excel record named, and the close-out's
verification says what the next milestone carries.

## Planning rulings during P1 (2026-10-08)

Asked by the implementing session after the doc commit 6387e9b,
before any P1 code. Planning read the files at 6387e9b.

1. The `rm -rf` forms (Q1). The grep at 6387e9b finds unguarded
   forms in `check.sh` line 79, `passthrough-run.sh` lines 121 and
   205, `privacy.sh` lines 89 and 513, `lo-check.sh` line 47 and
   `gate-self-test.sh` lines 86, 191, 715, 808 and 844, beside
   `counting.sh` line 158; item 3's "the one committed form" was
   planning's error. Ruling: option (a). P1 changes `counting.sh`
   line 158 alone, and in `passthrough-run.sh` only what item 5
   asks; the `specs/0001` bullet item 3 writes names the other
   forms as standing until a part touches them (`specs/0024` P2
   ruling 1's words), listed by file and line as above, and says
   of `passthrough-run.sh` line 205 that its targets, an earlier
   run's conversion directories under the run directory, existed
   before the run: a behaviour the rule's absolute case forbids,
   carried to the next milestone as a question of what a rerun
   into a run directory may remove, not changed here. The report
   lists the same forms. Item 3 reads as amended.
2. The `leftover` mechanism (Q2). Ruling: the proposal is taken.
   Before each child the runner makes a per-child directory with
   `mktemp -d` under `$WORK` (so on the filesystem of the caller's
   `TMPDIR`, which the run scripts export on the disk: `specs/
   0023` ruling 4 still holds) and exports it as `TMPDIR` to the
   child alone; after the child ends, any entry in it is the
   child's leftover, counted, then the directory removed as
   `rm -rf "${ctmp:?}"`, allowed since the script made it in the
   run. Not a before-and-after diff of the shared `TMPDIR`. The
   runner's comment states the filesystem reasoning; the test log
   shows the child's `TMPDIR` under `$WORK` through a stand-in
   tool that prints it, and the planted file is written by the
   stand-in during the child, since a file present before the run
   is not this child's. Item 5 reads as amended.
3. The column's form (Q3). Ruling: `leftover` stays `yes` or `no`
   (`passthrough-record.py` line 265 refuses any other value and
   line 377 sums the yeses; old run directories hold yes/no):
   `yes` when the run directory's glob or the child's `TMPDIR`
   held anything, `no` otherwise, the runner's comment saying so.
   The brief's "0 with nothing planted" was planning's slip; the
   test shows `yes` with the planted file and `no` without. No
   new column, no record-script change beyond item 5's header.
4. Two form points (Q4). (i) The environment line is the first
   section after the header and input lines, `specs/0001`'s rule
   on a log's first lines standing; "before anything else" in
   item 1 reads so. (ii) The record header's binary line per
   label, "Binaries (label): passthrough sha256 <x>; xlsx-diff
   sha256 <y>", accepted; where a label's run rows disagree every
   distinct value is listed and the line says they disagree,
   rather than refused. The prune set, derived as part 6 P3's set
   was (`dry-gate`, `dry-negative`, `dry-run`, `dry-scan`,
   `dry-size`, `dry-test`, `gate`, `keep-host`, `negative`,
   `target` under part 6 P4's cache directory; `out`, empty,
   stays; `scan`, `check1`, `superseded`, `logs`, `gen`,
   `scripts`, `FACTS.md`, `logged.*`, `report-*` and
   `privacy.out` kept), accepted.

## Planning verification and rulings on P1 (2026-10-08)

Verified by planning from primaries: the five commits over
bafd780 (the doc commit 6387e9b; 271fd30 adding four bullets to
`specs/0001` and removing nothing; 34eea64 changing `counting.sh`
line 158 alone; the phase commit bcef9c3 changing the runner and
the record script as rulings 2 to 4 say; the record commit
bdaa890 with exactly the six logs), each diff read in full; the
six logs at the reported hashes, each ending all as expected, the
scan RESULT: PASS at the final stage with every changed file
hashed; the environment section first after the gate's header and
input lines (propagation shared, keepalive 1, each with controls);
the ten pruned names with sizes and `rm -r` exit 0 each, the
seventeen kept names present; the self-test at bcef9c3 with 50
cases and RESULT: PASS, asserted by the gate; T1 to T3 and N1 as
the spec asks; five messages ASCII with the Co-Authored-By line
only; no U+2014 in the changed files or the logs; the one
`/home/` string in scan.log the split control fixture; planning's
own privacy scan bafd780..bdaa890, 5 commits, 1384 blobs, 0
findings; `ANTHROPIC_API_KEY` unset; the keepalive and the shared
root still so at verification. Planning's own run of the record
script at bdaa890 over part 6 P4's real copy run directory
(`2026-10-08-9cfd92e-copy`) into scratch: the `.txt` identical
to the committed record's, the `.md` differing in the written
time, the stem and the two added Binaries lines alone, the i686
passthrough hash equal to part 6 P4's run log's `emend-copy`
hash; so an existing run directory carries the columns and the
header gains only what item 5 asks.

1. Check 1's Q1 reservation 1, the citation: `specs/0023`'s
   ruling 4 sits under "Planning verification and rulings on P1
   (2026-10-01)" (line 351), so the runner's comment "P1
   verification ruling 4" is the precise form of ruling 2's
   "`specs/0023` ruling 4". No change.
2. Q1 reservation 2, the line numbers: the `specs/0001` bullet
   says "at 6387e9b" and is accurate as worded; at bdaa890 the
   runner's two standing forms are at lines 122 and 206. Not
   restated in `specs/0001` now: the part that next touches the
   runner restates them, and this ruling records them meanwhile.
3. Q1 reservation 3 and 4, old run directories and `$WORK`:
   answered by planning's own run above and by the runner's
   unchanged lines 120 to 122 (`$WORK` from `mktemp -d`, so under
   the caller's `TMPDIR`). No change.
4. Q2's gaps (the kept names checked by presence alone, `out` not
   checked empty; `$WORK` not printed by name; the detached
   runner's stderr in no log): accepted as recorded. The rule
   asks that the kept names are never removed, which presence
   shows; `logged.stderr` is the session's file, 0 bytes, as the
   earlier phases'.
5. Q3's items: the committed logs' hashes are planning's reading
   of the committed files (above); the superseded script and
   generator exist under the phase's cache at the report's
   hashes (a6dedb13, 6f550d3c, 29dd819e; the first dry attempts'
   list f95d002a), read by planning; the timing of the
   regeneration against the logged run is disclosure, and the
   logged negative run's printed script is the one that ran. The
   dry-scan finding (two undeclared placeholders in the carried
   change history) and its fix in the generator are accepted;
   the checker's side observation on the superseded dry scan's
   absolute paths concerns a file that is never committed.
6. N2, the scan's removal classifier on a planted line 158, goes
   beyond the spec and is accepted.
7. P1 is DONE. P1's build targets are pruned at P2's gate under
   the standing rule. Nothing is carried into P2's code beyond
   item 6; item 7's two amendments are written into the working
   `specs/0028` by planning and go in P2's doc commit with these
   rulings.

## Planning verification and rulings on P2 (2026-10-08)

Verified by planning from primaries: the three commits over
bdaa890 (the doc commit 1d3c22b holding this file and `specs/0028`
at the hashes planning sent; the phase commit 8d54741 adding two
lines to `package.rs`, each `check_copy_relationships(start,
&scope, None, index)?;` at the head of `copy_worksheet`'s
`sheetPr` and `sheetView` branches before the strip, and one test
in `copy.rs` with two bodies, a `sheetView` and a `sheetPr` each
carrying `r:id`; the record commit d3faae6 with exactly the five
logs), the diffs read in full with context; the five logs at the
reported hashes, each ending all as expected, the scan RESULT:
PASS at the final stage; the environment section first; P1's four
build names pruned with sizes and `rm -r` exit 0 each, the
nineteen kept names present; ten gate files checked against the
diff, none changed, no self-test; the full copy test's two
literals hashed equal at the base and the phase commit with a
control; the two plants each failing the new test at its refusal
line; the negative log's RV and RP refused through the gate's
host binary with the existing message and no output, RC accepted;
three messages ASCII with the Co-Authored-By line only; planning's
own privacy scan bdaa890..d3faae6, 3 commits, 1387 blobs, 0
findings; `ANTHROPIC_API_KEY` unset. Planning's own build at
8d54741 in its worktree (the part 6 P4 output directory pruned at
start, 170M): 514/526/573/589/918 on both targets (P4's
514/525/572/589/917, the one new test in each `copy` set), 1112
workspace tests on both, clippy and doc 0 lines; planning's own
`--copy-sample` with its own release binaries over all 27 corpus
workbooks on both labels, 54 outputs and 54 tool lines
byte-identical to part 6 P4's run directory
(`2026-10-08-9cfd92e-copy`): no behaviour change on the corpus.

1. The implementing session's (a), P1's `dry-negative` and
   `dry-scan` kept at P2's prune because P1's report cites a file
   in each by hash: the rule's exemption is for what a committed
   log cites by hash, and a report is not one; the two are dry
   outputs. Ruling: pruned at P3's gate, listed with sizes and
   removed with `rm -r` beside P2's build targets, named. Check
   1's Q2 reservation 1 and Q3 item 5 read as answered so.
2. (b), the first dry scan with `PLANNING_SPEC` set on a clean
   tree, failing the two working-copy checks and the dry-only
   check (three, as the superseded log counts; the report's "two"
   reads as corrected to three, two of them the working-copy
   checks), rerun unset: accepted as disclosure; no script
   changed; the logged scans ran unset on a clean tree, which is
   the scan's own branch for that state. Q3 items 2 to 4 are
   session facts, disclosed; the superseded attempt's hash
   1d909136 is the session's reading, kept on the disk.
3. (c) and check 1's observation: a malformed `sheetPr` or
   `sheetView` (an unbound attribute prefix, say) is now refused
   where it was copied before, with the refusal every other kept
   start tag gets; valid input is unaffected, and planning's own
   copy run over the corpus moved no byte. Accepted; the
   catalogue's "the pass checks every start tag kept" is now true
   without the P3 ruling 11 reading, and P3's doc commit says so
   in `specs/0028` as an amendment under P3 ruling 11, T2.
4. Q2 reservations 2 and 3 (the typed prune and keep sets with
   no search for a committed log citing a pruned path; the
   detached runner's stderr in no log): accepted as part 6's and
   P1's practice (P1 verification ruling 4). Q3 item 1, the
   pre-check's hash, is the session's reading, as always.
5. P2 is DONE. P2's build targets and P1's two dry directories
   are pruned at P3's gate (ruling 1). Nothing is carried into
   P3's code; the amendment of ruling 3 goes in P3's doc commit.

## Decisions for the maintainer

Decided by the maintainer 2026-10-08: 1 to 4 as recommended (A in
each), four answers typed in the planning session.

1. MSRV: A (recommended), `rust-version = "1.98.1"` equal to the
   pin, the four places checked to agree by the gate, a move a
   recorded decision; B, the oldest compiler the crate builds
   with, found by trial and tested on, below the pin.
2. F2, the emptied relationships part: A (recommended), kept as
   the catalogue states, no change on a closing part; B, dropped
   with its `Override` when a write empties it, under `images`,
   the removal sample's seven `added` rows gone.
3. The API review's output: A (recommended), the record alone,
   changes carried to the next milestone's first part; B, the
   renames and hides applied in P3 with every sample rerun.
4. The README: A (recommended), the short file with the three
   measured sentences each citing a record; B, with a worked
   example per feature as doctests.

## Out of scope

- The xlsx-diff, passthrough and privacy-gate backlogs under
  `ROADMAP.md` "Unscoped" (the maintainer's choice of 2026-10-08).
- Charts; a second crate; publication to any registry (the review
  precedes it; nothing is published in this milestone).
- Any rename, hide or widening the review proposes (decision 3 A).
- A `CHANGELOG`, versioning beyond the workspace's `0.x`, and a
  licence change.
- What Excel shows for a sample: the check is the maintainer's;
  this part only refuses to push without its record.
