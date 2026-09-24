# 0015: Milestone 1 part 8, open checks and the Excel check

Status: APPROVED by the maintainer 2026-09-23 (planning draft the
same day), the four decisions under "Decisions for the maintainer"
each taken as planning recommended; sent to the implementing session
the same day.
Context: `CLAUDE.md` ("The fidelity harness is the acceptance
instrument": a change is correct when the result opens in
LibreOffice headless and in Excel without a repair prompt; "Corpus
rules"), `ROADMAP.md` ("Standing items": Excel once per milestone,
outputs listed by sha256, pass or fail per hash recorded; "Unscoped":
the desktop Excel machine, without which Milestone 1 cannot be
accepted; "Milestone 1" part 8), `specs/0006-passthrough.md` P3 (the
open check's pinned definition, `lo` and `lo_in`, the image),
`records/libreoffice-2026-09-15.md` (the pins), `specs/0011` to
`specs/0013` ("Harness runs", where each part's corpus outputs were
open-checked), `specs/0001-bootstrap.md` ("How this task runs",
"Rules for every phase").

## Goal

Every edit the tests make is opened, not only the corpus samples.
Today the LibreOffice open check runs on the 27 corpus outputs of
each part's sample operation and nowhere else; the crate's and the
tool's edit tests assert bytes and never open their output. Part 8
gives them an opt-in keep of each test's input and output, a script
that opens every kept pair in the pinned image with the T3 P3
definition and records the result by test name and hash, and the
per-milestone Excel check as a procedure the maintainer runs on the
desktop machine, with the list the script writes and the record the
result fills. No core code changes; no size record.

## Facts planning checked (2026-09-23)

- The crate's package tests reach `write_to` through one helper,
  `written` (`crates/emend/src/package/tests.rs`), from 29 tests;
  the cells tests call `write_to` once (`cells/tests.rs`); the
  archive tests call it 19 times on zip-level fixtures that are not
  workbooks. The tool's 37 integration tests
  (`tools/emend-copy/tests/null_rewrite.rs`) run the built binary on
  generated files through one helper, `emend_copy`, over seven
  operations (`--drop-calc-chain` 7, `--edit-sample` 8,
  `--edit-formula-sample` 6, `--style-sample` 11, `--date-sample`
  9, `--calc` 10, `--formula-sample` 6 mentions).
- libtest runs each test on a thread named after the test's path
  (`package::tests::<name>`), so a helper can name a kept file after
  the test that made it without any change at the call sites.
- The package tests' smallest fixture (six parts, a `styleSheet`
  root with no namespace, an empty `sheetData`) converts in the
  pinned image with only the allowed `javaldx` line on stderr: one
  table in the ODS. The same fixture with a number and a formula
  written converts with the number and one formula present. So the
  fixtures can be open-checked as they are.
- The open check's definition and code live in
  `scripts/passthrough-run.sh` (`lo_check`): the conversion under a
  timeout, the stderr scan, the table count against the workbook's
  sheet count, `ok`, `fail` and `error` told apart (T3 P3 rulings).
- No Excel is on this machine (`specs/0000`); the standing ruling
  wants desktop Excel once per milestone on outputs listed by
  sha256. The machine is an open maintainer item.

## Design

### The keep (crate and tool tests)

`crates/emend/src/testing.rs` gains `keep(input, output)`: when the
environment variable `EMEND_KEEP_DIR` names a directory, it writes
`<dir>/<test>.in.xlsx` and `<dir>/<test>.out.xlsx`, `<test>` being
the current thread's name with `::` replaced by `.`; when the
variable is unset it does nothing and no behaviour changes. `written`
calls it with the fixture bytes `opened` built (kept in a thread
local) and the output; the cells test's write does the same. A test
whose write fails keeps nothing. Writing refuses (panics, failing
the test) when a file of the name is already there, so a directory
holds one run. The tool's `emend_copy` helper keeps the input and
output paths it was given after the run, under the same variable
and naming, when the run exited 0 and wrote an output.

The gate never sets the variable; `check.sh` is unchanged. With the
variable unset every test runs as today, which the test log shows
by a test run with and without it and the same counts.

### The open-check library

`lo_check` and its constants move from the runner into
`scripts/lo-check.sh`, sourced by the runner and by the new script;
the runner's behaviour is unchanged, shown by T3 P3's negative cases
rerun on it (`ok`, `fail`, `error` on the same fixtures).

### The script and the record

`scripts/edit-open-check.sh --keep <dir> --libreoffice <image>
[--lo-timeout <secs>] --out <records dir> --stem <stem>` runs the
open check on every `.in.xlsx` and `.out.xlsx` pair in `<dir>` and
writes `<stem>.md` and `<stem>.txt`: a row per test (test, in bytes,
in sha256, out bytes, out sha256, `lo_in`, `lo`), reasons kept in a
scratch directory and never printed, totals, and two lists by test
name: inputs LibreOffice refuses (`lo_in` `fail`, a fixture that is
not a workbook, allowed and named) and outputs it refuses on an
input it accepted (`lo` `fail` with `lo_in` `ok`, a finding). It
exits 0 when the second list is empty and no check errored, 1 when
the list is not empty, 2 on any `error`. The record's stem is
`m1p8-open-<utc-date>-<shortsha>`.

The script does not run the tests: the run log runs them, on the
host and on i686 into two keep directories, asserts the two hold the
same names with byte-identical files, and runs the script on the
host directory.

### Privacy

Kept files are fictional test fixtures; they are never committed
(the hygiene gate refuses the extension anyway) and the record holds
test names, byte counts, hashes and results only.

### The Excel check

`scripts/excel-check-list.py --run <run dir> --keep <dir> --out
<file>` writes the list the maintainer carries: one row per file
(source `corpus` with the workbook id and label, or `fixture` with
the test name; bytes; sha256; an empty `result` column; for the
formula sample, the three cell references the run log derived and
their expected computed values, which are the sample's constants),
from the milestone's final corpus run directory (part 6 P3's, host
label) and P1's keep directory. The procedure, in the record
template `records/excel-check-m1-<date>.md`: Excel's version and
build, the OS, the date; every file opened from the list, `pass`
(opens with no repair prompt and no recovery notice), `repair` (Excel
offered or made repairs), `fail` (Excel did not open it), and for
the formula sample the three values seen; nothing else typed. The
maintainer copies the files to the machine, fills the list there and
brings the list back; planning verifies each hash against the run
directory and the keep directory before the record commits. No
corpus file, content or path leaves except to the maintainer's own
machine, which the corpus rules allow.

Milestone 1 is accepted when the record shows `pass` on every row
and the sample values as expected. Until the machine exists the
record is absent and the milestone stays open; part 8 P2 delivers
the script, the template and a dry run of the list on the corpus run
directory, its hashes checked by the run log.

### 32-bit

The script runs on the host only; the i686 keep is compared byte
for byte with the host keep, so the check covers both targets'
outputs.

## Phases

Each phase ends as `specs/0001-bootstrap.md` says, with every rule
the earlier parts added (the rulings of `specs/0013` and the three
rules of `specs/0014` P2 verification rulings 4, 5 and 7: rerun
lines name only hashes the command under them prints; the run log's
record check compares content and each family row; a root line says
"sourced" where a library is sourced); logs in
`records/logs/m1p8-<phase>/`; the scan log last; at most two
independent checks; report; STOP. No push during the part; the
close-out push follows P2 and carries the local commit 9232e61.

Part 8's first doc commit carries, unedited: `ROADMAP.md`,
`specs/0014-xml-comparison.md` (the close-out rulings) and this
spec.

### P1: the keep, the library, the script and the first record

The keep in `testing.rs`, `written`, the cells test and the tool's
helper; `scripts/lo-check.sh` and the runner sourcing it;
`scripts/edit-open-check.sh`. Tests: the keep writes the pair named
after the test and refuses a second write of the name (a test that
sets the variable to a temporary directory for its own thread,
which is the only test that sets it); unset, nothing is written.
The negative log: the script on a keep directory with a planted
output that cannot open (a truncated archive, `lo` `fail`, exit 1),
with a planted input that cannot open (listed, exit 0), with an
image that is not there (`error`, exit 2), and with an empty
directory (refused); the runner's T3 P3 negative cases rerun. The
run log: the tests on both targets into two keep directories with
the variable set, the same test counts as the gate's run without it,
the two directories compared, the script on the host directory,
recorded as `records/m1p8-open-<utc-date>-<shortsha>.md` and `.txt`;
acceptance: no output refused on an accepted input, every kept test
named in the record, and the number of kept pairs printed beside
the number of tests that reach `written` and `emend_copy` with a
successful write, counted from the source by the log.

### P2: the Excel check list and procedure

`scripts/excel-check-list.py`, the record template, and the run
log's dry run: the list written from part 6 P3's run directory and
P1's keep directory, every hash in it checked against the file it
names, the three formula-sample references per workbook equal to
the run log's derivation of part 6 P2. The negative log: a run
directory with a missing output, a hash the file does not match, a
keep directory with an odd file. The Excel record itself is written
only when the maintainer has run the procedure; P2 does not wait for
it.

### Close-out push (after P2)

As part 7's, carrying 9232e61 and part 8's commits, on planning's
word and the maintainer's authorisation typed in the implementing
session.

## Decisions for the maintainer

1. Which tests are kept. Planning recommends: every test that
   reaches `written` in the crate, the one cells write, and every
   tool test that runs the binary to exit 0 with an output; the
   archive-level tests are excluded because their fixtures are zip
   files, not workbooks, and a refusal there would say nothing.
2. Naming kept files by the test thread's name. Planning recommends
   it: no call site changes, and a renamed test renames its files.
   The alternative, a name passed at each call, is 30 edits and a
   drift risk.
3. From Milestone 2 part 1 on, every phase's run log runs the edit
   open check on the host beside the corpus run, so a new edit test
   is opened in the same phase it is written. Planning recommends
   yes; the cost is one conversion per kept pair (about a second
   each in the pinned image).
4. The Excel list's contents: part 6 P3's 27 host outputs and P1's
   kept outputs, with the formula sample's three values to read.
   Planning recommends this set; earlier parts' sample outputs are
   superseded by part 6 P3's, which carry every operation's effect
   the samples make together (the formula sample writes a number, a
   formula and a replacement).

## Planning rulings during P1 (2026-09-23)

Asked by the implementing session after the doc commit e566de1
(verified: parent 9232e61, the three blobs at planning's hashes),
with figures from an unlogged scratch prototype that the phase logs
measure again; none of those figures is a primary until a log prints
it.

1. Q1, tests that write more than once: the pair is named
   `<test>.<n>.in.xlsx` and `<test>.<n>.out.xlsx`, `n` counting from
   1 in the order that thread writes (a thread-local counter); a file
   of the name already there is still refused. The record's test
   column is `<test>.<n>`. The acceptance line prints three counts:
   kept pairs, distinct kept tests, and the source count of tests
   that reach a successful write, and the set of distinct kept names
   must equal the set the log derives from the source (never a typed
   list). "Facts" said 29 tests reach `written`; that was the count
   of call sites, and tests reach it through helpers too. The
   prototype's 85, 1 and 28 are what the log is to measure.
2. Q2, the one test that opens bytes without `opened`: accepted.
   `open_fixture(bytes)` records the bytes and opens them; `opened`
   calls it and the test at `package/tests.rs` (the no-chain round
   trip) calls it in place of `Package::open`, the only call site
   that changes. `written` takes and clears the recorded input and,
   when a keep is on, panics if none is recorded or if the recorded
   input's entry names and CRC-32s (read with `parse_independently`)
   differ from the package's entries.
3. Q3, setting the variable from a test: the crate forbids unsafe
   code and `std::env::set_var` is unsafe in edition 2024, and the
   variable is process-wide. `keep` reads a thread-local override
   first and the environment variable only when there is no
   override; the keep tests set the override alone, to a temporary
   directory or explicitly to none; no test touches the variable.
   With the variable naming a directory that does not exist, `keep`
   panics. "P1" is read with this in place of "a test that sets the
   variable".
4. Q4, the tool tests: their own keep in `null_rewrite.rs` with the
   same naming, counter, refusal and override, keeping the input and
   output paths of a run that exited 0 whose output is a file
   afterwards, with its own keep tests. So the record tells the two
   crates apart, the tool's kept names carry the prefix
   `emend-copy.` before the bare thread name; the crate's carry
   their dotted module path as they are.
5. The prototype was a copy of the tree in scratch, never
   committed; nothing left the machine. Accepted as an unlogged
   measurement, to be named as such if the report mentions it.
6. These rulings go in a second doc commit before code, this spec
   unedited at the hash planning sends.

7. Finding before the logs (asked after the four code commits
   67ad8c1, 10aff01, ab26a7f, e9a63e2): LibreOffice loads none of
   the tool tests' kept inputs, so the tool side of the check would
   be vacuous. Planning measured the cause on six variants of the
   tool's package fixture in the pinned image: the fixture as the
   tests build it is refused ("source file could not be loaded");
   the same fixture with the real Office Open XML content types for
   the workbook, worksheet and relationships parts loads, one table
   in the ODS, with the bare `<sheetData/>` sheet part kept, with
   or without the `styles.xml` part, with or without the untyped
   media entry; the real sheet root alone, with the made-up types,
   is still refused. So the cause is the made-up content types
   (`application/workbook` and the like, 19 attributes in
   `null_rewrite.rs`), not a missing styles part. Ruling: option
   (b), narrowed to that cause. One test-only commit before the
   logs gives every tool fixture builder the real content types
   (workbook, worksheet, relationships, shared strings, styles; the
   untyped media entry stays untyped); no styles part is added and
   no test assertion changes, since none asserts on the type
   strings. The logs then run; the acceptance stands as written,
   and the record must show every tool input accepted, so a refused
   tool input is a finding from here on. The crate's seven refused
   fixture inputs are named in the record as the spec allows.

8. After d9315b8 (ruling 7 applied: all 19 attributes, the
   calcChain override counted among them, now real types), six
   tool inputs are still refused: the five tests built on the
   zip-level `specs()` fixture (no `xl/workbook.xml`) and the
   no-XML-parts round trip. They are the tool's counterpart of the
   crate archive tests decision 1 excluded and cannot become
   workbooks without changing what they test. Ruling: option (a).
   Ruling 7's last sentence is read as: a refused tool input whose
   fixture holds an `xl/workbook.xml` entry is a finding. The record
   names the refused tool inputs as allowed zip-level fixtures, as
   it names the crate's, and the run log asserts that the set of
   refused tool inputs equals the set of tool tests whose kept input
   has no `xl/workbook.xml` entry, derived by the log from the kept
   files (never a typed list), and that no crate or tool output is
   refused on an accepted input. No code change. The tree-status
   form of part 6 stands for this phase's logs: the one modified
   path is printed as planning's uncommitted spec and nothing else
   may differ.

## Planning verification and rulings on P1 (2026-09-23)

Verified from primaries at the record commit 50d2c4a (phase commit
e2b8a81): the nine commits and their diffs against "Design" and
rulings 1 to 8; the two doc commits' blobs at planning's hashes;
every hash the report cites against the committed files; planning's
own gate (nine PASS), workspace tests on both targets (523 passed,
2 ignored each), clippy, and privacy scan over the nine commits
(509 blobs, 0 findings); planning's own keep run with its own build
on both crates: 149 pairs with the record's 149 names, and both
hashes of every pair equal to the record's, so the keep is
deterministic across builds; planning's own conversion of every
kept file in the pinned image with its own sheet and table count:
the same `lo_in` and `lo` verdict as the record on all 149 pairs.
Planning's first pass converted all 298 files in one LibreOffice
call, which silently produced no ODS for 54 of them with no error
line; converted one file per call they all behaved as the record
says. The script converts one file per call, so the record is not
affected; the quirk is noted for anyone reproducing by hand. No API
key is set in planning's environment.

Rulings:

1. The six code commits do what "Design" and rulings 1 to 8 say and
   nothing else: the crate keep (67ad8c1, amended once from 27afb8c
   for a count in its message, trees equal, disclosed), the tool
   keep (10aff01), the library move (ab26a7f, a pure move but for
   one variable's name), the script (e9a63e2, e2b8a81) and ruling
   7's types (d9315b8). Accepted, including the script's extra
   refusals and its fourth list.
2. F1: ruling 7's sentence "no test assertion changes" was wrong as
   worded: the recipe test's expected content-types string changed
   in step with its builder (three of the nineteen attributes). No
   property changed. The sentence is withdrawn; d9315b8 stands.
3. F3, a real defect in the record's wording, and a gap in the
   check: the seven crate inputs listed as refused all hold
   `xl/workbook.xml`. Five were loaded by LibreOffice and failed only
   the sheet count, which matches an unprefixed `<sheet` element and
   so counts 0 in the prefixed-workbook fixture and in the fixtures
   whose workbook root has no children; two were refused by
   LibreOffice (the fixture with no relationships part; the
   empty content-types root). So in P1 seven crate edit tests were
   not open-checked, five of them for a reason in the check, not in
   the fixture. Ruling, for P2's first code commit: (a) the sheet
   count in `scripts/lo-check.sh` matches a `sheet` element with any
   prefix or none, amending the T3 P3 definition by that much (the
   table count already names its prefix; a fixture-level case in
   P2's negative log shows a prefixed workbook counted, and T3 P3's
   runner cases rerun unchanged); (b) the record's `.txt` gains
   `reason_in` and `reason_out` columns holding a class word taken
   from the reason file's fixed opening words (`none`, `refused`,
   `count`, `error`; never the reason text), and the `.md` lists
   refused inputs under two headings, "refused by LibreOffice
   (allowed: a zip-level fixture)" and "the sheet count did not
   apply (allowed: a workbook fixture with no sheet element)", each
   derived from the class; (c) the run log of P2 reruns the script
   on P1's keep (kept, or regenerated and shown byte-identical to
   the record's hashes) and records the new counts, expecting the
   prefixed fixture's two pairs to move to accepted.
4. F6 (the root line said the library was run): fixed by the
   negative log's rerun; accepted.
5. F7: the run log compares its counts with the test log's unset
   run, since the gate prints no counts; "P1" is read so.
6. F4 and F5 (no mutation on the tool keep's own refusal, counter
   or override; the "nothing written" tests watch a directory the
   keep never targets): notes, accepted; K3's measured non-bite is
   the exit-0 condition being redundant with the output-file test,
   left as it is and disclosed.
7. F9: the record pair was installed by hand; its hashes equal the
   run log's. Accepted; from P2 the run log prints the install
   command and hashes the installed copies.
8. F10 (the /tmp figure) is the implementer's observation, not a
   primary; accepted as marked. Planning removed 1.4 GB of its own
   scratch build directories the same day; the maintainer was told
   of the rest.
9. The gate's first attempt stopped for want of space on the
   temporary file system; every log now builds on disk under the
   home cache, with a root line. Accepted, and the rule from here:
   build directories live on disk, never on the temporary file
   system.
10. P1 is DONE. P2 is next: its first doc commit carries this spec
    and `ROADMAP.md` unedited at the hashes planning sends; its
    first code commit is ruling 3's (a) and (b); then "### P2" as
    written, with ruling 3's (c) in the run log.

## Planning rulings during P2 (2026-09-23)

Asked by the implementing session after the doc commit f9a0291
(verified: parent 50d2c4a, the two blobs at planning's hashes).

1. Q1, the class words: the map as proposed. `none` is a side with
   no reason file; `refused` is every return-1 opening but the
   count mismatch (the conversion exited, the stderr line, no ODS
   written); `count` is the count-mismatch opening; `error` is
   every return-2 opening. The class is read from the reason file's
   fixed opening words alone, and an opening the map does not know
   stops the script with exit 2.
2. Q2: the first alternative. A `count`-class input is listed under
   "the sheet count did not apply" only when the script's own count
   of `sheet` elements in the kept input's `xl/workbook.xml` is
   zero; any other `count`-class input goes under a third list,
   "Inputs whose sheet and table counts differ (a finding)", and
   the script exits 1 for it as it does for a refused output on an
   accepted input.
3. Q3, the layout as proposed: `reason_in` and `reason_out` after
   `lo`, nine columns; the same two in the Pairs table; the totals
   gain one row per side with the four class counts; the error
   list stays.
4. Q4: the regex as proposed, any prefix or none, `<sheets` and
   `<sheetData` still excluded. A `sheet` element of a foreign
   namespace would count; no corpus file or fixture has one, and
   the check is a count, not a schema. Accepted, and the library's
   comment says so.
5. Q5: the Excel list holds the corpus outputs and only the kept
   outputs whose input LibreOffice accepted (`lo_in` `ok` after
   ruling 3's count). The left-out tests are named in the run log
   with their class. "Pass on every row" then means every row of
   the list, which is what the acceptance intends; a fixture Excel
   cannot open because the fixture is not a workbook proves
   nothing.
6. Q6: P2's run log regenerates the keep on disk under
   `<home>/.cache/emend-build/m1p8-p2-keep-host` and shows it
   byte-identical to P1's record hashes (P1 verification ruling 3
   (c)); the list names fixture files relative to that keep and
   corpus outputs relative to the corpus root, part 6 P3's run
   directory, host label. Nothing is written under the corpus
   directory for this: fixtures stay out of it.
7. Q7: the template is committed as
   `records/excel-check-template.md` and a filled copy becomes
   `records/excel-check-m1-<date>.md`. The list is a tab-separated
   `.txt` with the thirteen columns proposed; it is NOT committed:
   the run log hashes it, it is regenerated on demand, and the
   filled record carries every row's source, name, bytes, sha256,
   result and the three values seen. The three references per
   formula-sample workbook are derived by the P2 run log from its
   own reading of each corpus input by part 6's definition (A, M +
   1 and F), with a control, and the derived `added` and `replaced`
   counts per workbook are asserted equal to the run directory's
   `formula` line for that workbook, which part 6 P3 recorded (the
   tool prints counts, not references). Expected values are the
   sample's constants (A times 2, 1 + 1, 2 + 3).

8. Q8 (asked after ed7c80f, the ruling 3 commit, verified): the
   script gains `--record <.txt>`, the edit open check's record of
   the same keep, and refuses with exit 2 a record whose rows are
   not exactly the keep's pairs by name, bytes and both hashes; it
   lists only rows with `lo_in` `ok`. The negative log's three
   cases land as proposed: a run directory with a missing host
   output, a record whose hash the file does not match, a keep with
   an odd file, each exit 2. "The Excel check" is read with
   `--record` added.
9. Q9: the script derives A, M and F itself from each corpus
   input, found through the corpus root's `manifest.tsv` (the run
   directory's grandparent, checked to hold it), by part 6's
   definition. The run log derives them independently with part 6
   P2's own comparison script, extracted from the committed
   `records/logs/m1p6-p2/run.log` where it is printed and checked
   against the hash that log printed, and compares cell for cell,
   with the planted-reference control, and asserts the derived
   `added` and `replaced` against each workbook's `formula` line.
   Privacy as proposed: no reference is printed by the script or
   any log; the run log prints counts and yes/no; references live
   only in the uncommitted list, which goes to the maintainer's own
   machine.
10. Q10: the row details as proposed (corpus rows first in
    manifest order, then fixture rows in name order; the six
    reference and value columns empty where `added` is 0, asserted
    by the run log against the `formula` line; fixture rows' six
    columns empty; expected values 42, 2 and 5). Accepted.
11. Rulings 8 to 10 follow a code commit, so no doc commit carries
    them within P2 (`specs/0001`: no doc commit after a code commit
    in a phase); they go unedited in the close-out doc commit. The
    report may cite them.

## Planning verification and rulings on P2 (2026-09-24)

Verified from primaries, not the report: the four P2 commits and the
record commit (ed7c80f, f66b4c5, da080f1, 686ba6d on 50d2c4a; each
Co-Authored-By only, one no-reply author); the six logs and the
record pair at the hashes the report gives; planning's own gate
(nine PASS), tests on both targets in both feature states, and
clippy, all built on disk; planning's own comparison of the
regenerated keep and of its own P1 keep with the record (298 files,
every byte count and hash equal) and its own LibreOffice verdicts
with the prefix-tolerant count agreeing with the record on all 149
pairs; planning's own reading of the uncommitted list (sha256
0e7e19f4..., 36697 bytes, 166 lines, mode 600): the header, 27
corpus rows in manifest order then 138 fixture rows in name order,
every byte count and hash the named file's, the fixture set exactly
the record's `lo_in` ok pairs, and the three references and three
values of every corpus row equal to planning's own derivation with
part 6's reader logic (19 with A, 15 with F, 27 of 27, each fitting
its `formula` line), with no reference printed; and a count of the
list's 24 distinct corpus references over the 14 files of the phase
diff, which found only phase, finding and case labels and fictional
fixture cells, no corpus reference. The template holds no reference
and names the acceptance rule.

1. F2: the record is right that the two crate inputs are refused by
   LibreOffice, and their class is right. The parenthetical
   "(allowed: a zip-level fixture)" is ruling 3 (b)'s wording and
   too narrow: each of the two fixtures holds a workbook part but
   no relationships part, or an empty content-types root, so
   LibreOffice loads nothing of it. run.log's remark that ruling 3
   names them zip-level fixtures is wrong and stands as written
   (a committed log is not rewritten). The heading becomes
   "(allowed: a fixture LibreOffice cannot load: zip-level, or a
   package with no relationships part or an empty content-types
   root)" in the phase ruling 6 names; the committed record is not
   regenerated for that.
2. F1: test.log and negative.log count the tree's status lines
   against a pattern and print neither the spec's path nor its
   hash; gate.log, run.log and the scan print both, and gate.log's
   hash is the one planning sent. Accepted for P2. From the
   close-out on, every log's tree-status check prints the status
   lines themselves, as gate.log and run.log do.
3. F3, F4 and F5: the refusals beyond the spec fail closed and fit
   it; the four run-log checks without a control are covered by
   planning's own derivation above; a reason class the negative log
   did not exercise can only cause a spurious exit 2, never a wrong
   class. Accepted; no rerun and no log added.
4. F6 and F7: the list script writes with the process umask (644
   here) and lets an IndexError on a short manifest or summary row
   escape as a traceback with exit 1. Neither reaches the
   repository: the list is never committed and the manifest is the
   corpus's own file. Carried: `main` sets umask 077 before writing
   and IndexError joins the caught exceptions, in the phase ruling
   6 names. Until then the list is set to mode 600 by hand, as it
   was.
5. F8, F9 and F10: the docstring's citation of rulings 8 to 10 is
   right once the close-out doc commit lands (ruling 11); the
   reason files hold only `scripts/lo-check.sh`'s fixed lines,
   never a line of stderr, so printing them discloses nothing and
   the run log may keep doing so; the missing blank line stands.
6. The carried items (rulings 1 and 4) are scripts-only. They are
   fixed in one code commit of the first phase that next edits
   `scripts/edit-open-check.sh` or `scripts/excel-check-list.py`,
   and in any case before the list is regenerated for the Excel
   check. They do not hold part 8's close-out; `ROADMAP.md` carries
   them under part 8.
7. Disclosures accepted: the unlogged dry runs preceded every
   logged run; the two script fixes before any logged run are
   ordinary development; `scripts/__pycache__` appeared after
   gate.log and test.log had read a clean tree, was removed before
   the scan, which ran last and passed, and the record commit holds
   eight files and no such directory.
8. P2 is DONE. The close-out runs as part 7's: one doc commit
   carrying this spec and `ROADMAP.md` at the hashes planning
   sends, unedited; the push on planning's word and the
   maintainer's typed authorisation in the implementing session,
   carrying 9232e61; the record commit after the push; the scan
   last; the `ANTHROPIC_API_KEY` check.

## Planning verification and rulings on the close-out (2026-09-24)

Verified from primaries: the doc commit b06e0a7 holds the two files
at the hashes planning sent; origin/main and local main are both
b06e0a7 after the push, the range 4bc741e..b06e0a7 is 17 commits
starting at 9232e61, each with the one no-reply author and
committer identity and no session link; CI run 36006157721 (push,
main, head b06e0a7) completed with conclusion success; the
repository is PRIVATE; planning's own blob scan over the range
found 0 findings in 528 blobs; `ANTHROPIC_API_KEY` is unset in
both sessions. The push script's identity section prints the exit,
the count of commits read (17 of 17) and the mismatches (0), and
its control prints its three counts against their expectations;
those printed values are what the two defective checks meant to
assert, and they agree with planning's own reading of the range.

1. The finding is real and is the compound-shape defect
   `specs/0001` forbids: in each of the two added checks only the
   first condition decides the printed answer. It is a form defect
   of the record, not of the pushed content, which planning read
   independently. Cause as reported: the two checks were added to
   part 7's script without the shape greps being run over the new
   source before its logged run.
2. push.log stands as written and is committed unedited: a log is
   never rewritten, and pre mode cannot run again once origin/main
   has moved. Attempt 1's scan (sha256 0f343e34...) is superseded,
   not committed, and is named by hash in the rerun's header.
3. A correction log, `records/logs/m1p8-close/correction.log`, in
   the form of `records/logs/m1p2-close/correction.log`: it names
   the two superseded checks by their line numbers in push.log's
   printed script and by place, never by their text; it prints the
   fixed script, in which each check goes through a named function
   (proposal (a)); it reruns the identity read, the mismatch count
   and the three controls over 4bc741e..b06e0a7 through those
   functions; it runs the three shape greps over its own source and
   over the fixed script before printing them, each with a control;
   and it cites push.log, ci-run.log and attempt 1's scan by
   sha256. It writes no shape.
4. The scan is rerun as attempt 2 (proposal (c), option (iii), as
   part 2's close-out ruling 4): the header names attempt 1 by
   sha256; correction.log is a sibling, hashed and scanned like
   push.log and ci-run.log; nothing is excluded and the expectation
   of 0 is not touched, so the rerun ends RESULT: FAIL on the same
   two hits in push.log's printed script and is committed as such.
   A failing run is a finding to record (CLAUDE.md); this one
   cannot pass for this close-out because the log that holds the
   shapes is the record of the push. Options (i) and (ii) are
   refused: the scan does not learn to forgive a known hit, and no
   file is left out of it. The run again with the fix named is the
   next close-out's scan, whose header names this finding.
5. Rule from the next close-out on, added to `specs/0001` with
   that close-out's doc commit: the push and ci-run scripts run the
   three shape greps over their own source before printing
   themselves, as a structural check with a control, the way the
   phase logs already do. A script carried from an earlier
   close-out is not exempt: the greps run over the file as it is
   about to run, additions included.
6. The report quoted the two forms verbatim in a message. A report
   or message names a shape hit by file, line number and count,
   never by its text, as the logs do. This is the rule for every
   report from now on.
7. Order: this ruling goes in a doc commit holding planning's two
   files unedited (this spec and `ROADMAP.md`, which closes part 8
   and carries the finding); then correction.log; then the scan
   rerun; then ONE record commit under `records/logs/m1p8-close/`
   holding push.log, ci-run.log, correction.log and scan.log. Both
   commits go with the next push. No independent check, as the
   close-out phase says. Report and STOP. Part 8 is DONE and
   PUSHED; Milestone 1's acceptance waits on the Excel check.

## Planning rulings on the Excel check (2026-09-24)

Facts, from the maintainer's run on the desktop machine and the
result files planning read: the 27 corpus rows were opened by hand
from the file-open dialog; the 138 fixture outputs were opened by
a PowerShell script driving desktop Excel through COM, alerts on,
read-only, closed without saving, each refusal recorded with
Excel's error text; the same script then opened the 138 fixture
inputs, which planning staged from the keep. Outputs: 80 pass, 58
refused, each with the one generic error, and a second run refused
the same files. Inputs against outputs: 76 pass/pass, 54
refused/refused, 4 refused/pass, 4 pass/refused. Planning's
structural reading of the 276 files explains every refusal it
looked into: a cell whose `s` is 1 or more in a styles part with
no font collection, or with no styles part at all (62 sides); a
worksheet part with no `worksheet` root; an empty `bookViews`; a
shared-string cell with no shared strings part; a calculation chain
naming a cell with no formula; a relationship whose target is no
entry; and the tool's counting fixtures, built to exercise the
survey, not to be workbooks.

1. The automated open is accepted as the fixture procedure and the
   record says so: Excel through COM with alerts on, so a repair
   prompt would have stopped the run, read-only, never saved. The
   corpus rows stay by hand from the dialog. The template gains the
   procedure and an input column in the phase ruling 8 names.
2. Decision 4 assumed LibreOffice's acceptance predicts Excel's. It
   does not: Excel refuses skeletal fixtures LibreOffice tolerates.
   The Excel check's rule for fixtures is therefore LibreOffice's
   (P1 verification ruling 3): an input Excel refuses is allowed and
   listed with its output's result; an output refused on an input
   Excel accepted is a finding. The list carries an input row for
   every fixture pair from now on, and the record an input result
   beside the output's.
3. The 54 pairs refused on both sides are allowed under ruling 2:
   the fixture's own shape, unchanged by the edit, is what Excel
   refuses. 25 of the tool's 26 pairs are among them.
4. The 4 outputs refused on an accepted input, each read by
   planning against its input:
   (a) `several_edits_to_one_part_apply_together.1`: the test's own
   relationship edits leave targets that are no entry. The fixture.
   (b) `an_index_at_the_count_plus_additions_is_refused_and_below_
   it_accepted.1`: emend's added `xf` names font 0, fill 0, border
   0 and cell style 0, which the fixture's styles part does not
   hold. (c) `a_formula_and_a_style_edit_on_one_cell_both_apply.1`
   and (d) `a_style_edit_alone_on_an_absent_cell_writes_it_or_
   nothing.1`: the edit gives a cell an `s` of 1 or more in a
   styles part of bare `xf` entries and no font collection, the
   shape Excel refuses in 62 sides here; the inputs referenced style
   0 or nothing. None is a defect against a workbook Excel or
   LibreOffice wrote, which always carries those collections; each
   is the same precondition, stated nowhere. Ruling: the
   precondition goes into `specs/0012`'s limits with Milestone 2
   part 2 (fuller styles), where the styles part is next touched:
   a style edit assumes the styles part carries the font, fill,
   border and cell-style collections and `cellXfs` entries that
   name them. The fixtures stay as they are; they test the layer.
5. The 4 inputs refused whose outputs Excel accepted are explained
   and recorded, no action: two shared-string cells with no shared
   strings part, which the edit replaced; one `s="1"` with no styles
   part, which the edit dropped; one calculation chain naming a
   cell with no formula, which the recipe dropped.
6. A gap this check shows: the corpus list holds part 6's
   formula-sample outputs, so Excel has not opened a corpus output
   of part 4's style sample, the one edit that adds an `xf`. The
   next Excel list includes a style-sample corpus run's outputs.
7. Milestone 1's acceptance reads, for the fixtures, as ruling 2
   says; for the corpus, as the template says. Planning recommends
   acceptance once the filled record shows every corpus row pass
   with its values, since the fixture findings are all ruled here.
   The maintainer decides.
8. The record: planning merges the three result files the
   maintainer brought back (the record with the corpus rows, the
   fixture output results with Excel's error text, the input
   results) into `records/excel-check-m1-2026-09-24.md`, checks
   every hash against the run directory and the keep, and the
   implementing session commits it with a scan as one record
   commit; the ruling's list-script, template and `specs/0012`
   changes are carried with P2 verification ruling 6's items.

9. The maintainer chose (2026-09-24) not to retain the filled
   record: 7 corpus rows were pasted to planning with their values
   and are recorded as measured; the other 20 the maintainer
   reported to planning as pass with the expected values, and the
   record marks them so. The fixture results are measured in full
   from the two result files. Milestone 1 is PROVISIONALLY accepted
   on this record, `records/excel-check-m1-2026-09-24.md`, which
   planning assembles and the implementing session commits with a
   scan. Excel check 2, on a list with input rows and a
   style-sample corpus run's outputs, after the carried fixes, is
   the one that confirms it; its filled record is retained. Any
   README sentence about Excel cites check 1 as provisional until
   then.

## Out of scope

- Any core crate change; the size record.
- Running LibreOffice or Excel in CI.
- Comparing the converted ODS content beyond the sheet count and,
  for the formula sample, the computed values already checked by
  part 6 P2.
- Excel for the web, or any Excel not on the maintainer's desktop
  machine.
