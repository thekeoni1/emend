# 0007: T6 gate self-test

Status: P1 DONE 2026-09-16 (code 0a337fb and f7d9dc7, records 4d2ece2
and 6f4764d); P2 PLANNED, after Milestone 1 part 1 (scoped 2026-09-16;
the maintainer approved scoping a committed gate self-test on
2026-09-15). Context: `CLAUDE.md`,
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
  cold in 43 s on this box (527 MB peak) and then fails step 8, `FAIL
  privacy`, because a clone has no local `user.email` and the step's
  output holds `FAIL identity` lines (the identity check is part of
  step 8, not a step; corrected after P1). The
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
  any use). It was printed into the logs, never committed. P1
  committed it as `scripts/counting.sh` and added `count_matches_i`,
  `run_lines` and `run_matches` (a command's line count or match
  count, `error` when the command itself fails), each with controls.

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

`scripts/gate-self-test.sh [--full] [--fast] [--case <id>]...`

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
- Output: one line per case, `case <id> source <log and case> expect
  [<shape>]; got [<shape>]; as-expected` or `UNEXPECTED`, where the
  shape is the exit status and the FAIL line's wording as the source
  recorded it (commit ids normalised to `<sha>`); a case whose source
  recorded several FAIL lines compares all of them, joined with ` + `
  (V5, V8, V18); a count-only line is compared as a count. Then
  `self-test: <n> cases, <u> unexpected, <e> errors`, and exit 0 only
  when both are 0 and every counting control was as expected. It never
  prints the sentinel, a denylist line, a corpus path or a fixture's
  content; captured outputs are counted for control bytes, non-ASCII
  bytes and the sentinel, and the counts printed.
- The default run is the file-level cases (hygiene, privacy, hook) and
  `fmt`; `--full` adds the build-level cases, which need cargo builds
  in the clone. Every privacy case runs `privacy.sh` over full
  history, the invocation every source log used; `--fast` runs them
  over the iteration range only and is for local iteration, never for
  a committed log.
- Measured in P1: 808 s for 46 cases (`self-test.log` at 4d2ece2) and
  805 s for 50 (`self-test.log` at 6f4764d), about 34 s per privacy
  case; the log records the wall time of each case and of the run.
  The sessions' command tool stops a background task after a few
  minutes and caps a foreground command at ten, so a committed run is
  produced detached (`setsid` and `nohup` into a scratch file, then
  polled) and its header says so; `negative.log` case N6 proves that a
  detached process outlives the window.

### Case catalogue

Every case names the log and case it comes from and takes its expected
result from there, never from memory. Three principles (P1 rulings,
2026-09-16):

1. A source is a primary: a log where the case ran, or, when no log
   holds the line, the gate script's own regex, control or message,
   cited as such. Memory is not a source.
2. Where two logs disagree, the latest log that records the class
   rules, and the case names both.
3. A line no log anywhere records makes the case FAIL, and the missing
   measurement is the finding. The case is then established in the
   phase's `negative.log` first (N7 for H3, N8 for K3) and sourced
   there by case name: a forward regression pin, not a present-tense
   validation of an older primary.

A case may draw on more than one source when its fixture and its
wording come from different logs (V8, V9a). Where the source log's
expectation is not clear, the case is left out and listed in the
report for a planning ruling.

Hygiene, through `check.sh --only hygiene` in the case's copy
(`FAIL hygiene`, with the count-only wording where the gate prints
only a count):
- H0 control: the unchanged copy passes.
- H1 a force-added office file at the root, lower and upper case
  (T0 P5 b1, b2).
- H2a, H2b, H2c the lock, tilde and `.bak` forms, one fixture per case
  (T0 P6c a1 to a3 for the forms; T0 P6c i L843 for the count-only
  wording).
- H3 `data.csv` and a `.tsv` (established as `records/logs/t6-p1/
  negative.log` case N7; T0 P6c b recorded a privacy line, not a
  hygiene line, and no earlier log holds this one).
- H4a a Zone.Identifier path at the root, with a non-office base name
  since the office rule runs first; H4b under `tests/fixtures/`
  (T2 P0 g L837 for both).
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
  naming that commit, the whole FAIL line compared (T0 P6 c L514).
- V4 a fake GitHub-style token assembled at run time: FAIL patterns
  (T0 P6 d).
- V5 a drive-letter Windows user path, a Claude Code project path and
  the third path form, each with a following segment, built at run
  time: three FAIL patterns lines (8, 10 and 11), all compared
  (T0 P6d a L352-354).
- V6 the bare shapes of the three path patterns in prose: PASS
  (T0 P6d b).
- V7a to V7d an address at example.com in content (FAIL email, T0 P6b
  g L789), in a commit message (FAIL message, T0 P6b g2 L800), in a
  path (FAIL paths naming the tree entry, its ordinal normalised, T0
  P6c e L651) and in an extra commit header (FAIL header, T0 P6c d
  L639).
- V8 a gzip-compressed office blob committed then deleted: three FAIL
  lines, all compared, two FAIL paths (corpus name, office path) and
  one FAIL archives (T0 P6c g-a L686-688 for the case, which recorded
  "FAIL lines: count=3"; T2 P0 b for the archives wording).
- V9a a zip signature in `notes.txt` (T0 P6c c L590 for the case; T2
  P0 b1 L524 for the current wording); V9b a compound-file signature
  in `x.bin` (T2 P0 b1 L524): FAIL archives.
- V10 an office file under `tests/fixtures/`: PASS (T0 P6b c).
- V11missing, V11mode644, V11empty, V11space: the denylist missing,
  mode 644, empty, and an entry with a space: exit 2 each, over full
  history as the sources ran (T0 P6 e1 to e3, T0 P6b f L776).
- V12remove, V12add: the patterns file with one pattern removed, and
  with one added: exit 2, full history, the count read at run time
  (T0 P6b e L589, L677).
- V13 a blob made unreadable: exit 2, full history (T0 P6 i L675).
- V14 a shallow clone: exit 2 (T0 P6 o L760).
- V15 a non-ASCII byte in a message: FAIL message (T0 P6 p L770).
- V16 `--range -x`: the usage line (T0 P6 q L779, which passed
  `--range --all`).
- V17a to V17d an office file path outside `tests/fixtures/` in the
  lock, tilde, `.bak` and plain forms: FAIL paths (T0 P6c a1 to a3
  and b1).
- V18 an office file and a Zone.Identifier path in one commit: two
  FAIL paths lines, both compared (T2 P0 a L487-488).

Hook:
- K1 `install-hooks.sh`, then a push of a clean branch to a local bare
  remote succeeds and a push of a branch holding a token shape
  assembled at run time is blocked with FAIL patterns (T0 P6 g
  L578-L630 for the structure; T0 P6 d L528 for the pattern class).
  The self-test never reads the private denylist, by any path (P1
  ruling; `negative.log` N9 greps the committed script and expects 0
  executable reads). The source's denylist route is refused by the
  hook since P6b, so it is not available to any push.
- K2 the hook refuses when `EMEND_PRIVATE_DENYLIST` is set (T0 P6b
  d3 L555).
- K3 `install-hooks.sh` refuses when `core.hooksPath` is set
  (established as `records/logs/t6-p1/negative.log` case N8; T0 P6b d
  holds no hooksPath case).

Build-level, `--full` only, in the case's copy with cargo:
- B1 a misformatted line: FAIL fmt (T0 P3 a L100; the wording with
  `--color never` from T0 P4 L319); run by default too.
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

### Planning rulings on P1 (2026-09-16)

Verified from primaries at 6f4764d: the record commit (five files,
527 insertions, 89 deletions; ASCII; Co-Authored-By last; check 2's
answer unchanged since 4d2ece2); the six file hashes; every result
line the report pinned; the repair diff f7d9dc7 against the
repair-round ruling (K1's run-time-assembled pattern hit, H2 as three
cases, V3 and V5 through `all_fails`, V11 to V13 over full history,
the corrected citations, `run_lines` and `run_matches` with
controls); the committed script holds no executable read of the
private denylist (one comment names the path, as N9 counts); the four
rerun headers name the log they replace by the record commit 4d2ece2,
three of them by that file's sha256 as well; `scan.log` ran last
(23:08:21Z, after `negative.log`) and hashes both check answers at
their committed bytes; the runs were serialized (gate 17:05 to
17:07Z, self-test 17:08 to 17:21Z, negative 23:01Z, scan 23:08Z, all
at f7d9dc7).

Planning's own runs at 6f4764d: the full gate (eight PASS, exit 0,
empty stderr); `scripts/privacy.sh --range 731d6ff..HEAD` (5 commits,
247 blobs, 0 findings, which covers the record commit that no phase
scan can cover); a bare-shape control with the pre-P6d forms over the
five record files, the record commit's message, the added lines of
4d2ece2..6f4764d and every tracked file at HEAD (0 hits; the one hit
in the removed lines is the replaced original); and eight self-test
cases (H2a to H2c, V3, V5, K1, K3, B1: 8 as expected, 191 s, tree
clean afterwards).

The reversible replacement in check 1's answer reproduces: the answer
at 6f4764d, reversed with the header's hex at the header's offset, is
byte for byte the answer at 4d2ece2 (25392 bytes, sha256
3da5c879...), and the shape occurs nowhere else in either version.

Residues, accepted as recorded:

- The header's "line 357, column 39 of the answer" is the line in the
  whole file as committed at 4d2ece2; within the extracted answer it
  is line 328, and in the file at 6f4764d the placeholder sits at line
  377. The byte offsets and the hashes are right, and the reversal is
  defined by them.
- `scan.log`'s rerun header names the replaced log by the record
  commit only; the other three give the sha256 as well. Both forms
  are allowed by `specs/0001-bootstrap.md`.
- The report counted 15 verdicts in `negative.log`; the log holds 23
  lines ending `: as-expected` (15 for N1 to N9, 4 for F1 to F4, 4
  structural). None fails.
- Three faults the implementing session found in the round, with no
  checker, stand as recorded: N6's capture provenance (the 400 s probe
  ran 16:03 to 16:09Z at 39b2249 and is now printed with its own
  verdict); the SERIALIZED line printed by the negative script rather
  than commented; and the malformed placeholder, repaired before the
  final scan. The round's first scan (23:02:34Z) is superseded and
  named, with the hash it recorded, in the final `scan.log`.

T6 P1 is DONE. Nothing is pushed; the next push follows Milestone 1
part 1 P3.

Carried to P2 from independent check 2 of P1: gap 8 (H3 and K3 are
sourced to this phase's negative log, principle 3 above); finding 17
(V12's arithmetic on a `count_matches` result, which fails loudly but
is the construct the phase removed elsewhere); a coverage regression
check that every form the catalogue names reaches its step, the class
of gap 7; a parallel mode, with the wall time re-measured, since 34 s
per privacy case puts the default run at 805 s; and the note that
`privacy.sh --range <rev>` with a single revision scans that
revision's whole ancestry, so a range is always written
`<base>..HEAD` (planning's own slip, 2026-09-16, caught by the commit
count).

### P2: the build-level cases

`--full` with B2 to B5, timed, in `records/logs/t6-p2/self-test.log`;
the rule added to `specs/0001-bootstrap.md` that a phase changing a gate
file runs `scripts/gate-self-test.sh` (default, or `--full` when steps
1 to 6 change) and commits the output as `self-test.log`; the items
carried from P1 above. P2 runs after Milestone 1 part 1.

## Out of scope

- Rerunning the harness (T1) or passthrough (T3) negatives; they belong
  to their own tools' tests.
- Running the self-test in CI.
