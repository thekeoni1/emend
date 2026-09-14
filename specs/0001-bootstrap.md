# 0001: T0 repo bootstrap

Status: READY for the implementing session (planning, 2026-09-14).
Context: `CLAUDE.md`, `specs/0000-step-zero-plan.md`.

## Goal

A repository that builds and tests on the host and on
`i686-unknown-linux-musl` from its first code commit. It needs one gate
script that enforces the CLAUDE.md rules a machine can check. This task
writes no workbook model and no harness logic.

## How this task runs

T0 is five phases. Do them in order. Each phase ends with:

1. The phase commit(s).
2. The phase's required logs written to `records/logs/t0-pN/` (listed
   per phase below), produced at the phase commit, then committed in a
   separate record commit. The first line of every log names the commit
   hash it ran at.
3. The independent check from CLAUDE.md. Use a fresh subagent, not a
   fork, so it inherits no conversation. Give it only: this spec, the
   output of `git diff <previous phase's record commit>..<this phase's
   commit>`, the logs in `records/logs/t0-pN/`, and the draft report.
   The draft report is included because the third question asks about
   the report's numbers and strings. Save its answer verbatim as
   `records/logs/t0-pN/independent-check.md`, in the record commit or a
   second record commit.
4. A report to planning: commit hashes, log paths, anything that
   surprised you, and the independent check answer quoted verbatim.
   Then STOP. The next phase starts only after planning's word.

If a phase fails its checks, record the failure in the phase log, fix
it, and re-run with the fix named in the log. Do not re-run until it
passes without saying what changed.

Every negative test below runs in a scratch copy (a `git worktree` in
the scratchpad, or a copied tree), never in the main working tree, and
is never committed. Its output goes in the phase's `negative.log`.

## P1: commit what exists

- Commit `CLAUDE.md`, `ROADMAP.md` and `specs/` exactly as they are in
  the working tree. Do not edit them; if something looks wrong, stop and
  report it.
- Logs:
  - `scan.log`: output of a U+2014 scan and a non-ASCII scan over the
    committed files, plus a check that the commit message is ASCII.
    Expected: no hits.
- Acceptance: `git show --stat` lists exactly `CLAUDE.md`, `ROADMAP.md`
  and the files under `specs/`; `scan.log` shows zero hits.

## P2: workspace skeleton, built and tested on both targets by hand

- `Cargo.toml`: a virtual workspace, `resolver = "3"`,
  `members = ["tools/xlsx-diff"]`. Shared `[workspace.package]` with
  `edition = "2024"` and `license = "MIT OR Apache-2.0"` (maintainer
  ruling 2026-09-14).
- `LICENSE-MIT` and `LICENSE-APACHE`: the standard MIT text with the
  copyright line `Copyright (c) 2026 emend contributors` (maintainer
  ruling 2026-09-14), and the unmodified Apache License 2.0 text. Both
  files must pass the U+2014 scan.
- `rust-toolchain.toml`: channel `1.98.1`, targets
  `i686-unknown-linux-musl`. MSRV is a later decision.
- `tools/xlsx-diff`: a binary crate skeleton. `main` prints usage and
  exits 2, plus one unit test asserting that behaviour. It has no
  dependencies; T1 adds `zip`.
- `.gitignore`: `/target`, and the office extensions `*.xlsx`, `*.xlsm`,
  `*.xltx`, `*.xltm`, `*.xlsb`, `*.xls`, `*.ods` ignored everywhere
  except under `tests/fixtures/` directories.
- Logs:
  - `build.log`: the commands and full output of
    `cargo build --workspace --all-targets` and `cargo test --workspace`
    on the host, then the same two commands with
    `--target i686-unknown-linux-musl`.
  - `binary.log`: `file` output for the i686 test binary that ran.
  - `ignore.log`: `git check-ignore -v` for `a.xlsx`,
    `sub/b.xlsm` (both ignored) and `tools/x/tests/fixtures/c.xlsx`
    (not ignored).
  - `scan.log`: as in P1, over the phase diff.
- Acceptance: `build.log` shows at least one executed test on each
  target with `test result: ok`, `binary.log` says ELF 32-bit, and
  `ignore.log` matches the expectations above.

## P3: gate script, build steps

Create `scripts/check.sh`: Bash, `set -euo pipefail`, runs from any
directory, prints one `PASS <step>` or `FAIL <step>: <reason>` line per
step, and exits non-zero on the first failure. This phase adds steps
1 to 4:

1. `host`: `cargo build --workspace --all-targets` and
   `cargo test --workspace`.
2. `i686`: the same with `--target i686-unknown-linux-musl`. Fail if
   the test output shows zero executed tests. Silently skipping tests
   is a failure, not a pass.
3. `fmt`: `cargo fmt --all --check`. It never runs formatting that
   writes.
4. `clippy`: `cargo clippy --workspace --all-targets -- -D warnings`,
   host only.

`--quick` may skip steps 2 and 4 for local iteration. Every log handed
to planning comes from a full run.

- Logs:
  - `gate.log`: a full `scripts/check.sh` run.
  - `negative.log`: in a scratch copy, (a) misformat a line, then show
    `FAIL fmt`; (b) make the i686 step run zero tests (for example with
    a test filter that matches nothing), then show `FAIL i686`;
    (c) add a failing assertion, then show `FAIL host`.
  - `scan.log`: as in P1.
- Acceptance: `gate.log` has `PASS` for steps 1 to 4 and exit 0;
  `negative.log` shows all three expected FAIL lines with non-zero exit.

## P4: gate script, dependency steps

Add to `scripts/check.sh`:

5. `native`: fail if
   `cargo tree --workspace --target all -e normal,build` contains `cc`,
   `cmake` or `pkg-config`. List every crate whose name ends in `-sys`
   and fail unless it appears in `scripts/sys-allowlist.txt` (created
   empty). Adding to the allowlist needs a planning ruling after its
   source is checked to be pure Rust.
6. `licences`: read `cargo metadata --format-version 1` (python3 is
   fine; it is not part of the build). Check every package's licence
   expression, workspace members included, against the allowed
   identifiers: MIT, Apache-2.0, `Apache-2.0 WITH LLVM-exception`,
   BSD-2-Clause, BSD-3-Clause, Zlib, Unicode-3.0, Unicode-DFS-2016. An
   `OR` expression passes if one alternative passes. An `AND`
   expression passes only if all parts pass. A missing licence fails.
   Any other identifier (ISC, 0BSD, CC0-1.0, MPL-2.0, and so on) fails
   and is reported for a planning ruling. Do not widen the list
   yourself.

- Logs:
  - `gate.log`: a full run, steps 1 to 6.
  - `negative.log`: in a scratch copy, (a) add a local path dependency
    crate with `license = "MPL-2.0"`, then show `FAIL licences` naming
    it; (b) one with no licence field, then show `FAIL licences`;
    (c) one with `license = "MIT AND ISC"`, then show `FAIL licences`;
    (d) one with `license = "ISC OR MIT"`, then show `PASS licences`;
    (e) one named `fake-sys` (no build script needed), then show
    `FAIL native`.
  - `scan.log`: as in P1.
- Acceptance: all six `PASS` in `gate.log`; every negative case shows
  its expected result.

## P5: gate script, hygiene step, and toolchain record

Add to `scripts/check.sh`:

7. `hygiene`: fail if any tracked text file contains U+2014, if any
   tracked file has an office extension outside a `tests/fixtures/`
   directory, if `git ls-files` shows a path containing
   `emend-testing`, or if `ANTHROPIC_API_KEY` is set in the
   environment.

Also add `records/toolchain-<date>.md`: rustc and cargo versions,
installed targets, kernel version, the `file` output for the i686 test
binary, and the commit the full gate passed at.

- Logs:
  - `gate.log`: a full run, all seven steps.
  - `negative.log`: in a scratch copy, (a) stage a file containing
    U+2014, then show `FAIL hygiene`; (b) force-add `x.xlsx` at the
    root, then show `FAIL hygiene`; (c) run with
    `ANTHROPIC_API_KEY=dummy` set for that one command only, then show
    `FAIL hygiene`. For (c), the value must be a literal non-secret
    placeholder, and it must not be exported to the session.
  - `scan.log`: as in P1.
- Acceptance: all seven `PASS`; every negative case shows its expected
  result; `git status` is clean afterwards.

## Done

T0 is done when planning has verified P5 from primaries, including
running `scripts/check.sh` itself at the final record commit. Nothing is
pushed; there is no remote.
