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

## Out of scope

- Any core crate change; the size record.
- Running LibreOffice or Excel in CI.
- Comparing the converted ODS content beyond the sheet count and,
  for the formula sample, the computed values already checked by
  part 6 P2.
- Excel for the web, or any Excel not on the maintainer's desktop
  machine.
