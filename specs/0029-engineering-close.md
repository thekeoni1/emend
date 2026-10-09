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
    close-out push, "The Excel check" below. Deferred a thirteenth
    time 2026-10-09 by the maintainer ("Planning rulings before the
    close-out"): carried to the next milestone's first part.

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
Amended 2026-10-09 (P4 ruling 2): the list is P5's, not P4's,
and P5 reruns on the host every sample whose run directory the
disk no longer holds, as the ruling says; "no other corpus run"
reads with that exception.

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
Amended 2026-10-09 (P4 ruling 2): P4 keeps the README, the push
script's Excel-record check and the gate's doc-directory rule (P3
verification ruling 2); the Excel list moves to P5 below.

### P5: the Excel list (added 2026-10-09, P4 ruling 2)

The doc commit (the P4 verification rulings); no code commit
unless the ruling names one. The run log: the formula sample on
the host over the corpus with the phase's release binary (the
list's `--run`), the host keep regenerated and opened by
`scripts/edit-open-check.sh` into a new open record
`records/m3p7-open-<utc-date>-<shortsha>` (the list's `--keep`
and `--record`), every sample of `specs/0013` to `specs/0022`
whose run directory is gone rerun on the host into
`<corpus>/runs/<utc-date>-<shortsha>-<name>` and recorded as
`records/m3p7-p5-<name>-<utc-date>-<shortsha>.md` and `.txt`,
each Milestone 3 sample's newest directory on the disk taken as it
stands, and the list written from them all into a stage under
`<corpus>/work` in `specs/0016` P1 ruling 4's layout, the log
printing the directories, the stage's path and the row count,
never a row. The gate, the negative log (the list script refusing
a run directory whose summary is missing, and a `--run-plain`
directory under another corpus root), the scan log; one
independent check on the run log's directory list against the
specs' "Harness runs" sections. Then planning's verification; then
the maintainer's Excel check; then the close-out.

## Close-out push (after P4)

Amended 2026-10-09 (P4 ruling 2): after P5. The close-out doc
commit's subject was "M3 part 7 close-out: commit planning docs
(the P5 verification and rulings, the Excel check record)";
amended by the rulings before the close-out (2026-10-09) to "M3
part 7 close-out: commit planning docs (the P5 verification and
rulings, decision 9 deferred a thirteenth time)", the Excel record
out of the doc commit and change 20 out of the push script.

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

## Planning rulings during P3 (2026-10-09)

1. The implementing session changed mid-phase. The session that
   ran P3 (the one that took the P1, P2 and P3 briefs) ended at
   about 15:02Z on 2026-10-09 when its terminal multiplexer
   session was killed by accident, after the logged runs had
   finished (`logged.exit` 0, 14:58:46Z), the records were
   installed, the scan pre-check had passed and the independent
   check's inputs had been assembled under `<phase>/check1/inputs`
   with `inputs.sha256`; the check's subagent was in flight and
   died with it. A fresh session continues the phase from that
   point. Ruling: what the dead session wrote stands as it lies on
   the disk and in the tree (the logs under `<phase>/logs` and
   their copies under `records/logs/m3p7-p3/`, the nine untracked
   records, the review's listing and formatter, the report draft);
   the new session verifies `inputs.sha256` over the inputs before
   it launches check 1, launches it afresh (no answer from the
   dead one is used), records it in the ruling-10 form, runs the
   final scan with `PLANNING_SPEC` set to this working file's
   hash, makes the one record commit, reports and stops. The
   report's departures name the session change with the times
   above; no log is rerun for it. A terminal multiplexer session
   is never killed while a logged run or a check is in flight;
   the planning session's note of 2026-10-09 to the maintainer
   explains the two grouped sessions.

## Planning verification and rulings on P3 (2026-10-09)

Verified from the primaries: the four commits over d3faae6 (the
doc commit db70ecd at the hashes sent; f065758 one bullet in
`specs/0001` and nothing else; the phase commit 09d2e0f the
workspace field, five inherited lines and the hygiene step's
`pins` with its two controls, no `.rs` and no `Cargo.lock`
change; the record commit ed0a9ef, eighteen files, nothing
else), the seven logs and the check record at the hashes the
report states, the nine records, the check's answer read in full.
The planning session's own build at 09d2e0f (its log beside the
earlier ones, not in the repository): the three pins read 1.98.1
and every member inherits; the lifted `pins` refuses a moved CI
pin; the crate's tests 514, 526, 573, 589 and 918 on both targets
and the workspace's 1112, as P2's; clippy and doc clean; the
release `emend-copy` the same binaries as P2's on both targets;
the four samples over the corpus on both targets 216 of 216
outputs and lines identical to the run directories the run log
names, which it shows equal to part 6 P4's. Commit messages ASCII
with the Co-Authored-By line last; `privacy.sh` over
d3faae6..ed0a9ef 4 commits, 1411 blobs, 0 findings; no API key in
the environment.

1. The session change (ruling 1 above) went as ruled: the inputs
   verified against their list, the check launched afresh, the
   final scan with `PLANNING_SPEC` set, one record commit. The
   check's inputs hold the spec as the doc commit committed it,
   without ruling 1; accepted, since ruling 1 governs the handover
   and changes nothing the diff or the logs are judged against,
   and the check's header says so.
2. The doc build (check 1, Q2): the test log's T2 printed a build
   finished in 0.05 seconds, so no rustdoc ran in the logged test
   run; the gate's five doc builds earlier in the same logged run
   and the same target carry the same question. A fresh unit under
   `-D warnings` can only follow a build that passed under the same
   flags and sources, so the logged result is sound but not shown.
   The planning session ran `cargo clean --doc` and the same build
   in its own target: rustdoc ran ("Documenting emend"), exit 0,
   no warning line, 120 item links in `all.html` with the five
   section anchors excluded, the listing's 120 module-level items.
   Accepted on that measurement. From P4 on, a log that claims a
   doc build removes the target's `doc` directory first and asserts
   the "Documenting" line, with a control; P4's gate does so.
3. The index count: the test log's "documented items in the index:
   125" counts five section anchors; the record states both 125
   and 120. Accepted as disclosed; the P4 gate's form counts item
   links only.
4. The API review record: 545 items, every `pub` item once (check
   1, Q4, by a second parser), the verdicts in the four forms, the
   counts 539, 5, 1, 0 equal to the table. Three wording defects,
   each true against the sources: the change list's item 5 names
   the private path `emend::package::sheet_read::Target`, whose
   public path is `emend::package::Target` (the row's own text has
   it right); row 63's reason says `SheetKind` sits at the crate
   root, and `lib.rs` re-exports no `SheetKind`; rows 12 and 15
   attribute two multi-line re-export blocks to a791e2c and
   `specs/0019`, the commit that opened each block, where the
   names in them came under `specs/0020` and `specs/0025` (the
   listing's commit search matches the block's first line). None
   moves a verdict or a count. Ruling: the record is not rewritten;
   P4's doc commit appends to it a dated section "Corrections
   (2026-10-09)" stating the three items in those words, the table
   untouched, and the P4 run log's README check reads the record
   as it then stands. The assembly of the table by a formatter
   with the six non-keep verdicts typed, disclosed at the record's
   line 9, is what "written by hand" means here: the judgment is
   the hand's, the typing is not. The hide verdict's premise holds:
   outside the archive's own tests nothing in the workspace calls
   `Archive::prefix_len` (planning's grep over `crates` and
   `tools`). The six changes go to the next milestone's first part
   as its carried items, the order of the record's list.
5. Item 9's attribution is per file, as the item specifies, so a
   commit touching `package.rs` is named by every probe that
   compiles it; the log and the record say so. Accepted; a method
   that attributes by the items a probe reaches is a question for
   the next milestone, noted under `ROADMAP.md` "Unscoped".
6. The two session facts without a committed log (the self-test's
   files moved into their directory after the launch line named
   the phase directory; `logged.sh`'s run arguments corrected
   before any logged run, the first form kept superseded) and the
   test log's API-key input line without a control are accepted as
   disclosures; `specs/0001` requires none of the three.
7. Size: the copy probes grew 64 and 128 section bytes (expected 0
   to 200, cause 8d54741), the core 0 (expected 0), the budget
   figure 436992 unchanged, 21760 under the ceiling. The record
   `records/size-m3p7-2026-10-09-09d2e0f.md` stands as Milestone
   3's closing size record.
8. P3 is DONE. P4 follows: the README (decision 4, A), the push
   script's Excel-record check, the Excel list written, the gate
   with the doc-directory rule of ruling 2.

## Planning rulings during P4 (2026-10-09)

Asked by the implementing session after P4's doc commit 2f05914
(this spec at 2ddd2b06, `ROADMAP.md` at 105a0827, the review
record's corrections), before any P4 code.

1. The README's fidelity sentence (decision 4, A) reads its
   figures from part 6 P4's records, not from the two pure
   passthrough records of 2026-09: from each of the four sample
   records' "Files by outcome" host row (files 27, ok 27, lo ok
   27, lo_in ok 27), from the copy record's host family table's
   total row (entries 917, identical 784: the entries the copy
   did not touch came back byte for byte), and from the open
   record's totals (pairs 1093, lo ok 1082, the eleven failures
   the record's own sections explain). The sentence names what
   the figures are, and the run log reads each from the record it
   cites by a command it prints.
2. The Excel list cannot be written from the disk: the formula
   sample's run directories and every sample run of `specs/0013`
   to `specs/0022` are gone (the corpus `runs/` holds 2026-10-01
   onward), part 6 P4's host keep was pruned at P1's gate, and
   the list script stops without a `--run` directory, a keep and
   its record. Ruling, option B with the list moved to its own
   phase: P5 (the Phases section, as amended) reruns on the host,
   with the phase's release binary, the formula sample and every
   sample whose directory is gone, one run per sample flag, into
   fresh directories, each recorded as the samples are; the host
   keep is regenerated and opened into a new open record; each
   Milestone 3 sample's newest directory is taken as it stands.
   The reruns are the harness's measurements at the closing
   commit, not repeats of the old ones, and their records say so.
   The printarea sample's directories hold six error rows of 27,
   which the list script refuses by `specs/0016` P1 ruling 1; the
   printarea sample is rerun with the others, and if error rows
   remain it is left out of the list, the count printed as a
   finding in the run log and named in the Excel record's header,
   the script's handling of error rows a carried item for the
   next milestone. P4 is the README, the push script's check and
   the gate's doc rule; nothing of the list is in P4.
3. The push script's Excel-record check, pre mode: at the tip
   exactly one file matches `records/excel-check-m2-*.md`; it
   holds a "## Results" heading and at least one table row whose
   result cell reads pass, repair or fail; the tip commit's
   message names the file's path exactly once; the control is a
   clone's tip without the record, refused. Acceptance of what the
   record shows is planning's verification, not the script's.
4. The close-out generator: all three scripts (push, CI, scan)
   are generated now from part 6's, under
   `<home>/.cache/emend-build/m3p7-close/`, with the subject the
   Close-out section names as amended; their diffs against part
   6's scripts are printed in P4's test log; a change at the
   close-out, if any, is printed then as every close-out prints
   its changes.
5. The negative log's two cases: the check lives in a named
   function of the generated push script; the negative log
   extracts it by a printed line range, prints the extract's
   sha256 and shows it equal to the script's lines, then runs it
   against the current tip (refused) and against a scratch clone
   whose tip holds a planted record in the template's form and a
   message naming it (accepted). Accepted.

## Planning verification and rulings on P4 (2026-10-09)

Verified from the primaries: the three commits over ed0a9ef (the
doc commit 2f05914 at the hashes sent, the review record's eight
added lines; the phase commit ad7cfec adding `README.md` and
nothing else; the record commit a199fe4, six files), the six logs
at the hashes the report states, the check's answer read in full,
the three generated close-out scripts and their diffs against
part 6's read in full. The planning session's own reading of the
README: every file it names exists at the phase commit
(`LICENSE-MIT`, `LICENSE-APACHE`, `records/toolchain-2026-09-15.
md`, the records); every feature is `[]` in the manifest, as "no
dependency of its own" says; the eleven feature costs, the budget
figure and the ceiling equal the size record's; the sample, copy
and open figures equal ruling 1's sources; the scale figures equal
the record's table; the README check program run by planning
over the README and the phase commit's tree: 36 sentences, 18
holding a digit, 18 cited, 0 uncited, 0 missing. The scan's final
stage passed with `PLANNING_SPEC` at 3d6390e7. `privacy.sh` over
ed0a9ef..a199fe4 (the report's figure, 3 commits, 1410 blobs, 0
findings).

1. The phase commit was amended once before any logged run (its
   first form 20999b4, the same tree, its message's diff-alone
   sentence carrying an appended clause the gate refused in the
   dry run; only the message changed). Accepted as a disclosure;
   the first form was never reported to planning or cited by a
   log.
2. The README's measured sentences (check 1, Q1): (a) is a
   defect: of the 1093 kept pairs, 100 are the copy tool's tests'
   and one is `cells`', so "the crate's own tests" is wrong; (d)
   is a defect: the scale record says tier c's throughput includes
   the raw-copy samples and its rewriting samples ran at 1.8 to
   3.7 MB/s, and the README's 4.8 leaves that out; (b) is no
   defect: "opened 1082" is the record's `lo ok`, and the sentence
   claims nothing about why the eleven did not open; (c) is as
   ruling 1 worded it. Ruling: P5's first code commit, "M3 part 7
   P5: the README's two sentences", changes line 61 and line 65
   and nothing else: line 61 reads "Of the 1093 output files the
   crate's and the copy tool's tests keep, LibreOffice opened
   1082; the record's own sections explain the eleven it did not"
   with the same citation; line 65 gains, after "on the static
   target", the clause ", the largest tier's figures including the
   raw-copy samples, whose rewriting samples ran at 1.8 to 3.7
   MB/s" before the citation. P5's run log repeats the README
   check with the same program (its hash printed equal to P4's)
   and P5's gate the README hygiene; P4's logs stand as they are.
3. The push script's check reads whatever the tip is (check 1,
   (g)). Ruling: the close-out's push script gains change 21, pre
   mode checking that the tip's subject is the close-out doc
   commit's subject as the Close-out section names it, with a
   control refusing another subject, printed in the close-out log
   as the close-out's changes are; the scan script's `TIP_PATTERN`
   already holds the subject.
4. The pre-mode control refuses only the no-record branch (check
   1, (f)); the negative log's cases N3 to N7 exercise the
   heading, the row and the message branches. Accepted; the
   control stays as it is.
5. The gate's own `check.sh` doc step prints no "Documenting"
   line, so the gate asserts that `all.html` was written again
   after the removal, with a control; the five crate builds assert
   the line. Accepted as the form of P3 verification ruling 2 for
   that step.
6. The report's session facts without a primary (the dry runs'
   two superseded attempts, the amend) are accepted as
   disclosures.
7. P4 is DONE. P5 follows as the Phases section, as amended,
   and P4 ruling 2 say, with ruling 2 above as its first code
   commit.

## Planning rulings during P5 (2026-10-09)

Asked after P5's doc commit 515c3e2 (this spec at dad50b9c,
`ROADMAP.md` at 15e8ba04) and the phase commit f39bc7b (the
README's two lines), before any P5 script.

1. The rerun set reaches back to `specs/0011` and `specs/0012`:
   option A. P4 ruling 2 and the P5 phase text name `specs/0013`
   to `specs/0022` because those were the gone directories the
   implementing session listed; "The Excel check" names every
   part's samples, and the edit layer's `edit` and
   `edit-formula` (`specs/0011`) and the styles part's `style` and
   `date` (`specs/0012`) are corpus samples whose directories are
   gone too. All four are rerun on the host as the others are,
   recorded as `records/m3p7-p5-<name>-<utc-date>-<shortsha>`,
   and the list's `--run-plain` order is spec order from
   `specs/0011` to `specs/0028`. `new` stays out, as `specs/0016`
   says the runner does not run it over the corpus; `comment` and
   `image` are found in `specs/0023`'s rerun directories.

## Planning verification and rulings on P5 (2026-10-09)

Verified from the primaries: the three commits over a199fe4 (the
doc commit 515c3e2 at the hashes sent; the phase commit f39bc7b
changing the README's two lines in ruling 2's words and nothing
else; the record commit daab8c7, 53 files: five logs, the open
record pair, 23 rerun record pairs), the logs at the hashes the
report states, the check's answer read in full. The run log's
sample table: 37 samples in spec order from `specs/0011` to
`specs/0028`, 23 rerun on the host at f39bc7b, 14 found in their
newest directories (the flag read from each directory's
`run.host.tsv`); 22 reruns 27 of 27 ok with LibreOffice opening
all 27 outputs and all 27 inputs (planning read every record's
host row); the printarea rerun 21 ok and 6 error, as its three
earlier directories were, left out of the list with the finding
printed. The keep: 1093 pairs, the open record
`records/m3p7-open-2026-10-09-f39bc7b` with lo_in ok 1081 and lo
ok 1082, part 6 P4's figures. The stage
`<corpus>/work/excel-check-m2-2026-10-09/` read by planning by
counts only: the list 3134 rows (corpus 972, which is 36
directories of 27; fixture 1081; input 1081) under its header,
sha256 aabf420a, mode 600; `files/corpus`, `files/fixture` (1081)
and `files/input` (1081) beside it with the opener script; the
zip 604025974 bytes, sha256 38037fc8, neither committed. The
README check repeated with P4's program: 36, 18, 18, 0, 0. The
scan's final stage passed with `PLANNING_SPEC` at 6ba07ae5;
`privacy.sh` over a199fe4..daab8c7: 3 commits, 1463 blobs, 0
findings; no API key in the environment.

1. `--drop-calc-chain` (`specs/0011` "Harness runs"; check 1,
   Q1): not a sample and never on an Excel list; it stays out,
   and this ruling says so. The Excel check is of the samples.
2. The order inside `specs/0028` (anchor, commentbox, copy,
   remove-image against the section's commentbox, anchor,
   remove-image, copy): P5 ruling 1 fixed the order between
   specs, not within one; accepted as it stands.
3. The rerun records carry the closing commit on their `Commit:`
   line, written by the committed record script; that line is
   what "say so" means, since no record script change was in
   scope. The Excel record's header names the reruns as
   measurements at f39bc7b and names the printarea sample's six
   error rows as the reason it is absent from the list.
4. The printarea sample's six error rows at f39bc7b are the same
   count its three directories of `specs/0025` show; a finding
   carried to the next milestone's first part with the list
   script's handling of error rows (P4 ruling 2).
5. P5 is DONE. What follows is the maintainer's Excel check on
   the stage, its record `records/excel-check-m2-2026-10-09.md`
   (or the date the check is run) assembled by planning from the
   opener's results file and verified row by row against the
   stage by hash, then the close-out doc commit and the push as
   the Close-out section says, with change 21 (P4 verification
   ruling 3).

## Planning rulings before the close-out (2026-10-09)

Decided by the maintainer 2026-10-09, typed in the planning
session, asked after P5 was verified: the Milestone 2 Excel
check is skipped for this part and carried to the next
milestone's first part, its thirteenth deferral ("Skip, carry to
M4 part 1", the recommended option of three; the others were to
hold the close-out until the check ran, and to drop the check).
The stage `<corpus>/work/excel-check-m2-2026-10-09/` and its zip
stay on disk, the list pinned to f39bc7b, so the check runs later
against the same files. Nothing in the README claims Excel.

1. "The Excel check" and item 10 of the scope are amended: the
   check is a hard requirement of the next milestone's first
   part's close-out push, not of this one. The push script's
   change 20 (`excel_record_ok`, `excel_record_refused`,
   `excel_record_show`, its pre-mode block and the control clone)
   is removed from the generated script by `gen.py`: `EXCEL_FUNCS`
   and `EXCEL_PRE` dropped from the replacements. In their place
   the pre mode prints, after the part 6 record check, one line
   "files at HEAD matching records/excel-check-m2-*.md: 0" from
   `git ls-tree -r --name-only HEAD`, and checks the count is 0
   (a stray record is a finding); its control: the same count
   over a `git ls-tree` of a clone of the tree with an empty file
   added at `records/excel-check-m2-control.md` and committed
   there is 1, so the form counts. The committed negative log
   `records/logs/m3p7-p4/negative.log` and its cases N3 to N7 of
   the removed check stand as the record of a check that was
   written, tested and not used; the close-out log says so.
2. The close-out doc commit's subject is "M3 part 7 close-out:
   commit planning docs (the P5 verification and rulings,
   decision 9 deferred a thirteenth time)"; `TIP_NEW` in
   `gen.py` and the scan script's `TIP_PATTERN` hold it. The doc
   commit holds this spec and `ROADMAP.md` only; `ROADMAP.md`'s
   Milestone 3 heading reads DONE and PUSHED with the Excel check
   deferred a thirteenth time and named under the next
   milestone's first part; part 7's Close row says the same.
3. Change 21 (P4 verification ruling 3) is added by `gen.py` as
   `TIP_CHECK`: pre mode, after the HEAD-is-main check, reads
   `git log -1 --format=%s HEAD`, checks it equals the subject of
   ruling 2 exactly (`[ "$subj" = "$TIP_SUBJECT" ]`, the subject
   a single-quoted constant beside `BASE_PATTERN`), and its
   control: the same comparison refuses the subject with one
   character appended. The close-out log prints the generated
   scripts' hashes, `gen.py`'s hash, and a numbered list of the
   changes 19 to 21 as the P4 run log printed 19 and 20, the
   removal of 20 named.
4. The implementing session's context was cleared 2026-10-09
   after P5; the close-out runs in the same session, emend-6e,
   from a fresh brief. Its first step is to read this spec's
   Close-out section and this ruling, regenerate the three scripts
   with `gen.py` into `<home>/.cache/emend-build/m3p7-close/`,
   report their hashes, and wait for planning's word before the
   doc commit; no corpus run, no cargo beyond the pre mode's gate.
5. The previous P5 verification ruling 5 is amended: the Excel
   check and its record are out of this part; what follows P5 is
   the close-out doc commit and the push as the Close-out section
   says, with rulings 1 to 3 here.

6. The regenerated scripts, read by planning from the files (gen.py
   cfa5c819, push ba036b85, ci c3feb170 unchanged, scan 719894ba;
   `bash -n` clean; no non-ASCII byte): rulings 1 to 3 are met.
   The implementing session's four choices are accepted: (a) the
   change 21 control compares the subject with one character
   appended through the same form, independent of HEAD; (b) a
   "git ls-tree exit is 0" check precedes the count, so a failed
   listing cannot read as 0; (c) the hashes block reads `gen.py`
   at its home path and the three scripts beside the running one,
   so all three are copied to `<scratch>/close`; (d) the control
   clone commits as "control". The superseded scripts stay in
   `superseded-2026-10-09-gen-a2a26258/` since the P4 logs cite
   their hashes.
7. The doc commit holds this spec and `ROADMAP.md` at the hashes
   its body names by command substitution; the record commit's
   files are the three close-out logs and the independent check,
   as part 6's.

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
