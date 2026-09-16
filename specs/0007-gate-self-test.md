# 0007: T6 gate self-test

Status: P1 READY (scoped 2026-09-16; the maintainer approved scoping a
committed gate self-test on 2026-09-15). Context: `CLAUDE.md`,
`specs/0001-bootstrap.md` (the gate and "Rules for every phase"),
`specs/0002-gate-corpus-additions.md` (the privacy gate's later
additions), `specs/0003-ci.md`.

## Goal

A committed script, `scripts/gate-self-test.sh`, that proves the gate
still catches what it was built to catch: one case per detection class
that the T0, T2 P0 and T0 P6d negative tests established, run against
scratch clones, with the unchanged clone as the control. It runs
whenever a phase changes a gate file (`scripts/check.sh`,
`scripts/privacy.sh`, `scripts/privacy-patterns.txt`,
`scripts/privacy-identity.txt`, `scripts/sys-allowlist.txt`,
`scripts/install-hooks.sh`, `.gitignore`, the CI workflow) and its
output is committed as that phase's `self-test.log`. It is not part of
`check.sh` and does not run in CI: it needs a scratch denylist and, for
the build-level cases, cargo builds in a clone.

## Facts planning checked (2026-09-16)

- The gate's negative cases live only in nine phase logs
  (`records/logs/t0-p3/` to `records/logs/t0-p6d/` and
  `records/logs/t2-p0/`), about 340 results, each script printed into
  its log. Nothing reruns them; a change to the gate is checked only by
  the negatives of the phase that makes it.
- A `git clone --shared` of this repository runs `check.sh --quick`
  cold in 43 s on this box (527 MB peak) and then fails step 8 with
  `FAIL identity`, because a clone has no local `user.email`. The
  self-test sets the clone's `user.name` and `user.email` from
  `scripts/privacy-identity.txt` before any case, as the T0 negatives
  did.
- `check.sh` runs its steps in order and stops at the first failure,
  so a step 7 or 8 case would pay for steps 1 to 6 first. Hence
  `--only <step>` below.
- `scripts/privacy.sh` accepts `EMEND_PRIVATE_DENYLIST` and prints a
  NOTICE; `check.sh` and the pre-push hook refuse it. Cases that need a
  sentinel denylist call `privacy.sh` directly, as T0 P6 to P6c did.
- The hook is tested with a local bare repository as the remote (T0 P6
  case g).
- The T2 P7 logs sourced a shared counting library (count_matches,
  count_from_file, count_bytes, count_lines, each reading the exit and
  printing `error` on a read failure, with fifteen controls run before
  any use). It was printed into the logs, never committed.

## Design

### `check.sh --only <step>`

`--only` runs one step (`host`, `i686`, `fmt`, `clippy`, `native`,
`licences`, `hygiene`, `privacy`) and prints its `PASS` or `FAIL` line;
the `ANTHROPIC_API_KEY` check still runs first, whatever the step. It
cannot be combined with `--quick` or `--ci`; a bad step name prints the
usage line and exits 2. The rule stands that logs handed to planning
come from full runs; `--only` is for the self-test and local iteration.

### `scripts/counting.sh`

The T2 P7 counting library, committed: the four functions and a
`counting_controls` function that runs every form on a match, a miss
and an unreadable file and prints one line per control. The self-test
sources it and runs the controls before any case. Later log scripts
source it too instead of carrying their own copy.

### `scripts/gate-self-test.sh`

`scripts/gate-self-test.sh [--full] [--case <id>...]`

- Clones the repository at HEAD (`git clone --shared`) into a scratch
  directory, sets the identity there, and runs every case in a fresh
  copy of that clone (a `git worktree` or a second clone per case, so
  one case cannot leak into the next). The main tree is never written.
  Everything under the scratch directory is removed on exit, including
  on a signal.
- Refuses to run if `EMEND_PRIVATE_DENYLIST` is set in its own
  environment; each privacy case sets its own scratch denylist (mode
  600, holding one sentinel `zq` followed by random hex, as
  `privacy.sh`'s self-test does).
- Every fixture is built at run time from pieces: the script's own text
  holds no token shape, no path shape, no address and no office file
  name, so the script itself passes the gate it tests.
- Output: one line per case, `case <id>: expect <shape>; got <shape>;
  as-expected` or `UNEXPECTED`, where the shape is the FAIL line's step
  name and, where the source case asserted wording, that wording; a
  count-only line is compared as a count. Then
  `self-test: <n> cases, <u> unexpected, <e> errors`, and exit 0 only
  when both are 0 and every counting control was as expected. It never
  prints the sentinel, a denylist line, a corpus path or a fixture's
  content; captured outputs are counted for control bytes, non-ASCII
  bytes and the sentinel, and the counts printed.
- The default run is the file-level cases (hygiene, privacy, hook) and
  `fmt`; `--full` adds the build-level cases, which need cargo builds
  in the clone. The default run should finish in a few minutes; the
  log records the wall time of each case.

### Case catalogue

Every case names the log and case it comes from and takes its expected
result from there, never from memory. Where the source log's
expectation is not clear, the case is left out and listed in the report
for a planning ruling.

Hygiene, through `check.sh --only hygiene` in the case's copy
(`FAIL hygiene`, with the count-only wording where the gate prints
only a count):
- H0 control: the unchanged copy passes.
- H1 a force-added office file at the root, lower and upper case
  (T0 P5 b1, b2).
- H2 lock, backup and further-extension forms (T0 P6c a, i).
- H3 `data.csv` and a `.tsv` (T0 P6c b; T3 P4 ruling).
- H4 a Zone.Identifier path, at the root and under `tests/fixtures/`
  (T2 P0 a, g).
- H5 a tracked path containing the corpus directory name in any case
  (T0 P5b d).
- H6 a tracked path holding a control byte (T0 P5b b).
- H7 U+2014 in a staged file deleted from the working tree (T0 P5b a).
- H8 U+2014 in a symlink's target string, dangling (T0 P5b c, h).
- H9 an office file under `tools/x/tests/fixtures/` passes (T0 P6b c).
- H10 `ANTHROPIC_API_KEY` set for the one command: the hygiene FAIL
  line first, before any step (T0 P5 c, P5b e).

Privacy, through `scripts/privacy.sh` in the case's copy with the
scratch denylist:
- V0 control: the unchanged copy prints the NOTICE, then PASS.
- V1 a commit authored with a different address: FAIL identity
  (T0 P6 a).
- V2 a Claude-Session line in a message: FAIL message (T0 P6 b).
- V3 the sentinel committed, then deleted in the next commit: FAIL
  naming that commit (T0 P6 c).
- V4 a fake GitHub-style token assembled at run time: FAIL patterns
  (T0 P6 d).
- V5 a drive-letter Windows user path and a Claude Code project path,
  each with a following segment, built at run time: FAIL patterns
  (T0 P6b h, T0 P6d a).
- V6 the bare shapes of the three path patterns in prose: PASS
  (T0 P6d b).
- V7 an address at example.com in content, in a path and in an extra
  commit header: FAIL email (T0 P6b g, T0 P6c d, e).
- V8 a gzip-compressed office blob committed then deleted: FAIL
  (T0 P6b a).
- V9 archive and compound-file signatures in files named `x.bin`: FAIL
  (T0 P6c c, T2 P0 b).
- V10 an office file under `tests/fixtures/`: PASS (T0 P6b c).
- V11 the denylist missing, mode 644, empty, and an entry with a space:
  exit 2 each (T0 P6 e, T0 P6b f).
- V12 the patterns file with one pattern removed, and with one added:
  the self-test exits 2 (T0 P6 f, T0 P6b e).
- V13 a blob made unreadable: exit 2 (T0 P6 i).
- V14 a shallow clone: exit 2 (T0 P6 o).
- V15 a non-ASCII byte in a message: FAIL message (T0 P6 p).
- V16 `--range` starting with a dash: the usage line (T0 P6 q).

Hook:
- K1 `install-hooks.sh`, then a push of a clean branch to a local bare
  remote succeeds and a push of a branch holding the sentinel is
  blocked (T0 P6 g).
- K2 the hook refuses when `EMEND_PRIVATE_DENYLIST` is set (T0 P6b d).
- K3 `install-hooks.sh` refuses when `core.hooksPath` is set (T0 P6b
  d).

Build-level, `--full` only, in the case's copy with cargo:
- B1 a misformatted line: FAIL fmt (T0 P3 a); run by default too.
- B2 a failing assertion: FAIL host (T0 P3 c).
- B3 a test compiled out on i686 so zero tests run: FAIL i686 (T0 P3
  b).
- B4 licences: a path crate with `MPL-2.0`, one with no licence field,
  `MIT AND ISC`: FAIL licences naming it; `ISC OR MIT`: PASS (T0 P4 a
  to d).
- B5 native: `fake-sys`, `fake_sys`, `cc` behind an unused feature, as
  a build-dependency, renamed, and under a target cfg: FAIL native; both
  fake crates allowlisted in the copy: PASS (T0 P4 e, T0 P5 d to j).

## Phases

Each phase ends as in `specs/0001-bootstrap.md`: the doc commit, the
code commit(s), one record commit with the logs and the check answers
(maintainer decision 2026-09-16), logs in `records/logs/t6-<phase>/`
(`gate.log`, `negative.log`, `scan.log`, `self-test.log`; `test.log` if
a crate changes), at most two independent checks in plain ASCII,
report, STOP. No push during T6.

### P1: the file-level self-test

`check.sh --only`; `scripts/counting.sh`; `scripts/gate-self-test.sh`
with H0 to H10, V0 to V16, K1 to K3 and B1;
`records/logs/t6-p1/self-test.log` is a full default run from the main
tree at the phase commit.
negative.log covers the self-test's own failure modes: a case forced to
the wrong result is reported UNEXPECTED and the run exits 1; an
unreadable capture reads `error` and the run exits 1; a run with
`EMEND_PRIVATE_DENYLIST` set is refused; `--only` with a bad step name
and combined with `--quick` or `--ci` is refused by `check.sh`; and
`check.sh --only hygiene` on the unchanged tree prints exactly the one
PASS line. The self-test's wall time is in the log. Acceptance: every
case as expected, no unexpected, no errors; the gate still eight PASS;
the scan clean, the pre-P6d bare-shape control 0 over the new script.

### P2: the build-level cases

`--full` with B2 to B5, timed, in `records/logs/t6-p2/self-test.log`;
the rule added to `specs/0001-bootstrap.md` that a phase changing a gate
file runs `scripts/gate-self-test.sh` (default, or `--full` when steps
1 to 6 change) and commits the output as `self-test.log`.

## Out of scope

- Rerunning the harness (T1) or passthrough (T3) negatives; they belong
  to their own tools' tests.
- Running the self-test in CI.
