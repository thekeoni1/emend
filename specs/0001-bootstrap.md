# 0001: T0 repo bootstrap

Status: P1, P1R and P2 DONE, verified by planning 2026-09-14. After the
rewrite, P1 is af5a2c6 to d3754dc; P1R is doc commit d495f0a with
records through 88d5644; P2 is 1a985a1 to 9daa4c6. P3 to P7 READY.
Context: `CLAUDE.md`, `specs/0000-step-zero-plan.md`.

## Goal

A repository that builds and tests on the host and on
`i686-unknown-linux-musl` from its first code commit. It needs one gate
script that enforces the CLAUDE.md rules a machine can check, including
a privacy gate, and a first push to the private GitHub repository. This
task writes no workbook model and no harness logic.

## How this task runs

T0 is P1, P1R, then P2 to P7, in that order. Each phase ends with:

1. The phase commit(s).
2. The phase's logs, written to `records/logs/t0-<phase>/` (for example
   `records/logs/t0-p1r/`) and committed in one or more record commits
   after the phase commit. Each record commit's message says what it
   adds. The first line of every log names the commit the log is about
   and the HEAD at the time it ran.
3. The independent check from CLAUDE.md, run by a fresh subagent (not a
   fork). Its inputs:
   - this spec;
   - the phase diff, `git diff <previous phase's last record
     commit>..<this phase's last phase commit>` (for a root commit, the
     diff from the empty tree);
   - the draft report;
   - the gate and scan logs, meaning every log this phase wrote under
     its logs directory.

   The harness also puts CLAUDE.md into a subagent's context. The
   checker says so and does not use it as evidence. Save the answer as
   `independent-check-N.md`, with N counting from 1. The file starts
   with a header written by the implementing session: the checked
   commit, HEAD, and each input file with its sha256. Then comes a line
   `---`, then the answer verbatim. Run at most two checks per phase. If
   check 2 still finds gaps, record them in the report and stop; do not
   start a third round.
4. A report to planning: commit hashes, log paths, anything that
   surprised you, anything that looks wrong in the files being committed
   (look before the phase commit, not after), and the latest check
   answer verbatim or its committed path and sha256. Then STOP. The next
   phase starts only after planning's word.

If a phase fails its checks, record the failure in the phase log, fix
it, and re-run with the fix named in the log. Do not re-run until it
passes without saying what changed.

Every negative test runs in a scratch copy (a `git worktree` or a clone
outside the repo), never in the main working tree, and is never
committed. Its output goes in the phase's `negative.log`.

## Rules for every phase

- Commit messages are ASCII and end with the Co-Authored-By line only,
  with no Claude-Session line or other session link (CLAUDE.md).
- Toolchain: run every rustup and cargo command with
  `RUSTUP_AUTO_INSTALL=0`, so a missing toolchain or target fails instead
  of downloading. The pinned channel `1.98.1` is a separate rustup
  toolchain from `stable`, even though both are rustc 1.98.1. Any install
  is a planned, logged step under the CLAUDE.md Elevation section.
- Scans:
  - Run them from a script that starts with `#!/usr/bin/env bash` and
    calls `/usr/bin/grep` explicitly. In the Claude Code tool shell,
    `grep` is a different program.
  - Match bytes under `LC_ALL=C`: non-ASCII as `-P '[^\x00-\x7F]'`, and
    U+2014 as its UTF-8 bytes E2 80 94. Never use `\x{2014}`, which GNU
    grep rejects under `LC_ALL=C` with exit 2.
  - Exit 0 means a match, 1 means no match, and 2 or more is an error.
    An error is never reported as zero hits.
  - Every scan form has a positive control that uses the same command
    form.
  - Print the exact script into the log.
- Log privacy: logs are committed and will be pushed. They must never
  contain:
  - the hostname, or `uname -a` output (the kernel version is
    `uname -r` only);
  - Windows host paths, or paths inside Claude Code's temporary session
    directories;
  - session ids or session links;
  - tokens or keys;
  - any entry of the private denylist.

  Use repo-relative paths, and write scratch locations as `<scratch>`.

## Planning rulings on P1 (2026-09-14)

- P1 content is verified from primaries:
  - Exactly four files were committed.
  - ROADMAP.md and specs/ are unedited: planning wrote them, and the
    working tree still matched the commit.
  - CLAUDE.md is as the maintainer left it at commit time.
  - Every committed file and every commit message has zero non-ASCII
    bytes, by planning's own scan with a positive control.
  - The GitHub repository had no branches, so nothing was pushed.
- Accepted as they stand: the empty-tree diff for the root commit, four
  record commits, and `independent-check-2.md`. The rules above now
  cover these cases.
- Accepted as recorded, with no further P1 fix rounds: the run 1
  heading, scan.log not scanning itself, the addendum's unprinted
  controls, and runs made at a later HEAD. A log cannot scan itself;
  planning's verification scan covers record files.
- CLAUDE.md formatting (trailing spaces, long lines, unindented
  continuation lines) is the maintainer's text. It is not a defect for
  any gate or check.
- `memory/` in CLAUDE.md means Claude Code's memory folder for this
  project, outside the repo (maintainer ruling, recorded in
  specs/0000).
- The P1 commit messages carry a Claude-Session line. The maintainer
  ruled that commit messages carry none, so P1R rewrites them.

## Planning rulings on P1R (2026-09-14)

- P1R is verified from primaries by planning's own checks:
  - All five old and new pairs have equal trees, authors, committers and
    dates. Each message lost exactly one line, the Claude-Session line,
    and gained none.
  - Doc commit d495f0a holds exactly the pending maintainer and planning
    changes: 4 files, 327 insertions, 41 deletions.
  - Every commit message on main ends with the Co-Authored-By line and
    has no session line or link. The body of d424f81 mentions the word
    in a sentence; git parses no such trailer there.
  - Every file in every commit on main, and every message, has zero
    hits against the private denylist, the P6 patterns and the
    non-ASCII scan, each with a positive control.
  - The GitHub repository still has no branches.
- Question 1, the backup ref deleted before the later logs and checks:
  accepted. rewrite.log had already shown the rewrite correct, which is
  what the spec asked for. Do not expire the reflog before T0 is done.
- Question 2, `/home/dev` paths: accepted. The Linux user name `dev`
  identifies no person. The P6 goal now says which host paths are
  covered. No further rewrite.
- Question 3: a check file whose header names the phase on line 1 and
  the commit on line 2 satisfies the first-line rule.
- Question 4: planning updated this status line and ROADMAP.md to the
  new hashes. Committed P1 logs keep their original hashes as history;
  rewrite.log maps old to new.
- The messages of ce8c6b6 and 88d5644 have no scan in a primary;
  planning's own scan covers them. The form weaknesses listed in the
  report are accepted as recorded.
- From P2 on, planning's doc updates waiting in the working tree are
  committed unedited as the first commit of the next phase, and that
  phase's scan.log covers them.

## Planning rulings on P2 (2026-09-14)

- P2 is verified from primaries by planning's own checks:
  - Planning built and tested the workspace itself, on the host and on
    `i686-unknown-linux-musl`, with auto-install off and a separate
    target directory. One test passed on each target. The i686 binary is
    a static 32-bit ELF. Both binaries exit 2 with the usage line on
    stderr and nothing on stdout.
  - `LICENSE-APACHE` has the same sha256 as the text apache.org
    publishes. `LICENSE-MIT` is the standard MIT text with the ruled
    copyright line.
  - `cargo tree` shows no dependencies. The package licence is
    `MIT OR Apache-2.0`, with `publish = false`.
  - All six P2 commit messages end with the Co-Authored-By trailer only.
  - Every file in every commit on main, and every message, has zero hits
    against the private denylist (each entry tested as a control), the
    shape-based P6 patterns and non-ASCII bytes. The GitHub repository
    still has no branches.
- Question 1: `Cargo.lock`, `version = "0.1.0"`, `publish = false`, the
  `run()` helper with its constants, and the `.gitignore` comment are
  accepted. `publish = false` also guards against an accidental publish.
- Question 2: yes. Office extensions must be ignored in any letter case.
  P3 updates `.gitignore`, and the P5 hygiene step matches extensions
  case-insensitively.
- Question 3: the 80-column wrap of `LICENSE-MIT` is the standard text.
  Line wrapping is not part of the licence.
- Question 4: a unit test on `run()`, plus verify.log's process-level
  check, meets "one unit test asserting that behaviour".
- The unplanned toolchain install is recorded as a finding. Its root
  cause was this spec's pin; "Rules for every phase" now covers it.
- The remaining check 2 gaps are accepted as recorded. Planning's own
  scans cover the denylist, the hostname and the record commits'
  messages.
- The P6 patterns are now shape-based (see P6). Check answers are
  committed verbatim.

## P1: commit what exists (DONE; rewritten in P1R)

- Commit `CLAUDE.md`, `ROADMAP.md` and `specs/` exactly as they are in
  the working tree. Do not edit them; if something looks wrong, stop and
  report it.
- Logs:
  - `scan.log`: output of a U+2014 scan and a non-ASCII scan over the
    committed files, plus a check that the commit message is ASCII.
    Expected: no hits.
- Acceptance: `git show --stat` lists exactly `CLAUDE.md`, `ROADMAP.md`
  and the files under `specs/`; `scan.log` shows zero hits.

## P1R: rewrite the P1 commit messages, then commit the doc updates

Starting state:
- `main` holds the five P1 commits 5f15d55, a1246af, 8caddb4, 1fffb0d
  and 15c120a.
- The working tree has uncommitted changes. In CLAUDE.md, the maintainer
  added the Elevation section and planning added the commit-message
  rule. Planning also changed ROADMAP.md and `specs/`.

Step 1, rewrite:
- First create `refs/backup/t0-p1-pre-rewrite` at the old head.
- Replay the five commits oldest first with `git commit-tree`. Each new
  commit keeps:
  - the same tree;
  - the same author and committer names, emails and dates (set
    `GIT_AUTHOR_*` and `GIT_COMMITTER_*` from the original);
  - the original message with only the lines matching
    `^Claude-Session:` removed.

  Its parent is the previous new commit.
- Then run `git update-ref refs/heads/main <new head> <old head>`.
- Do not touch the index or the working tree. Do not use
  `git filter-branch`.
- Delete the backup ref at the end of P1R, once the logs show the
  rewrite is correct. The reflog keeps the old objects locally.

Step 2, doc commit:
- Commit `CLAUDE.md`, `ROADMAP.md` and `specs/` as they stand, unedited.
- If something looks wrong, stop and report it before committing.

Logs:
- `rewrite.log`:
  - the exact script;
  - for each old and new pair: both hashes, whether the trees are equal,
    whether author and committer names, emails and dates are equal, and
    the message line counts before and after;
  - the count of removed lines, never the removed lines themselves;
  - afterwards: `git rev-list --parents main`, a per-commit count of
    `^Claude-Session:` lines on main, `git diff --stat <old head> <new
    head>`, `git status --short` showing the uncommitted doc changes
    survived, and `git for-each-ref refs/backup` after the backup ref
    is deleted.
- `scan.log`: per the scan rules, over the files in the doc commit and
  over every commit message on main. It covers non-ASCII bytes, U+2014
  bytes and `^Claude-Session:` lines, with positive controls.

Acceptance:
- Every old and new pair has equal trees, authors and committers.
- Exactly one line was removed from each message.
- Zero Claude-Session lines remain on main.
- `git diff --stat` between the old and new heads is empty.
- The doc commit lists exactly the changed files.
- Every scan shows zero hits and every control passes.
- `refs/backup` is empty at the end.

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
  - `scan.log`: per the scan rules, over the phase diff.
- Acceptance: `build.log` shows at least one executed test on each
  target with `test result: ok`, `binary.log` says ELF 32-bit, and
  `ignore.log` matches the expectations above.

## P3: gate script, build steps

Create `scripts/check.sh`: Bash, `set -euo pipefail`, runs from any
directory, prints one `PASS <step>` or `FAIL <step>: <reason>` line per
step, and exits non-zero on the first failure. Any grep in it follows
the scan rules. This phase adds steps 1 to 4:

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

Also in this phase, change `.gitignore` so the office extensions match in
any letter case (for example `*.[xX][lL][sS][xX]`), keeping the
`tests/fixtures/` exception. Add `ignore.log` to this phase's logs:
`X.XLSX`, `sub/Y.Xlsm` and `Z.ODS` are ignored, and
`tools/x/tests/fixtures/C.XLSX` is not.

- Logs:
  - `gate.log`: a full `scripts/check.sh` run.
  - `negative.log`: in a scratch copy, (a) misformat a line, then show
    `FAIL fmt`; (b) make the i686 step run zero tests (for example with
    a test filter that matches nothing), then show `FAIL i686`;
    (c) add a failing assertion, then show `FAIL host`.
  - `scan.log`: per the scan rules.
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
  - `scan.log`: per the scan rules.
- Acceptance: all six `PASS` in `gate.log`; every negative case shows
  its expected result.

## P5: gate script, hygiene step, and toolchain record

Add to `scripts/check.sh`:

7. `hygiene`: fail if any of these holds:
   - a tracked text file contains U+2014, matched by bytes per the scan
     rules;
   - a tracked file has an office extension, in any letter case,
     outside a `tests/fixtures/` directory;
   - `git ls-files` shows a path containing `emend-testing`;
   - `ANTHROPIC_API_KEY` is set in the environment.

   The step first runs its own positive control: a temp file holding
   the U+2014 bytes must be detected. If the control fails or grep
   exits 2 or more, the step fails.

Also add `records/toolchain-<date>.md`: rustc and cargo versions,
installed targets, kernel version from `uname -r` only, the `file`
output for the i686 test binary (repo-relative path), and the commit the
full gate passed at.

- Logs:
  - `gate.log`: a full run, all seven steps.
  - `negative.log`: in a scratch copy, (a) stage a file containing
    U+2014 (write it with printf byte escapes, so the log and script
    stay ASCII), then show `FAIL hygiene`; (b) force-add `x.xlsx` at
    the root, then show `FAIL hygiene`, and again with `Y.XLSM`; (c) run with
    `ANTHROPIC_API_KEY=dummy` set for that one command only, then show
    `FAIL hygiene`. For (c), the value must be a literal non-secret
    placeholder, and it must not be exported to the session.
  - `scan.log`: per the scan rules.
- Acceptance: all seven `PASS`; every negative case shows its expected
  result; `git status` is clean afterwards.

## P6: privacy gate

Goal: nothing personal reaches GitHub. That covers names, email
addresses other than the GitHub noreply address, host paths, session
links, and tokens. Host paths means Windows host paths, paths inside
Claude Code's temporary session directories, and hostnames. `/home/dev`
identifies no person and is allowed.

Deliverables:
- Repo-local identity: `git config --local user.name thekeoni1` and
  `git config --local user.email` set to the noreply address in
  `scripts/privacy-identity.txt`.
- `scripts/privacy-identity.txt` (committed): line 1 the name
  `thekeoni1`, line 2 the address
  `55367154+thekeoni1@users.noreply.github.com`. Both already appear in
  every commit.
- `scripts/privacy-patterns.txt` (committed): one ERE per line. Write
  each pattern so the file does not match itself, by putting one
  character of every literal in brackets, for example `sk[-]ant[-]`.
  Each pattern matches the shape of a real secret or path, not a bare
  prefix, so docs, logs and check answers may name a pattern without
  tripping it (planning ruling 2026-09-14). Cover:
  - GitHub tokens: `gh[pousr][_][A-Za-z0-9]{20,}` and
    `github[_]pat[_][A-Za-z0-9_]{20,}`;
  - Anthropic keys: `sk[-]ant[-][A-Za-z0-9_-]{20,}`;
  - private key headers: `-----BEGIN [A-Z ]*PRIVATE[ ]KEY-----`;
  - Claude Code session links: `claude[.]ai/code/session[_][A-Za-z0-9]+`;
  - session ids: `session[_][A-Za-z0-9]{20,}`;
  - Claude Code temporary directories: `/tmp/claude[-][0-9]+/`;
  - Windows user paths: `/mnt/[a-z]/[Uu]sers/`;
  - default Windows hostnames: `DESKTOP[-][A-Z0-9]{5,}`.

  The self-test and the negative tests use realistic fake values built at
  run time, never bare prefixes.
- `scripts/privacy.sh`:
  - Default scope: every commit reachable from HEAD. `--range A..B`
    narrows it.
  - It prints `PASS privacy` or one FAIL line per finding. Exit codes:
    0 pass, 1 findings, 2 error.
  - It starts with a self-test. In a temp git repo, a commit holding a
    pattern hit and a denylist sentinel must be detected. Otherwise the
    script exits 2.
  - Its greps follow the scan rules.
  - Checks, for every commit in scope:
    1. Identity: the local `user.name` and `user.email` equal
       `scripts/privacy-identity.txt`, and so do the commit's author
       and committer name and email.
    2. Message: ASCII, zero `^Claude-Session:` lines, zero pattern
       hits, zero denylist hits.
    3. Content: the commit's whole tree via `git grep -I -n`, checked
       against the patterns and against the denylist.
  - Private denylist:
    - Path: `$HOME/.config/emend/private-denylist.txt`, overridable by
      `EMEND_PRIVATE_DENYLIST` for negative tests only.
    - It must exist, be mode 600, and hold at least one entry, or the
      script exits 2. Lines starting with `#` are comments.
    - Entries match as fixed strings, case-insensitive, whole words.
    - It is never committed, copied, or printed.
  - Findings name only the check, the pattern or entry number, the
    commit, the path and the line number. Never print the matched text
    or the line. Strip `git grep` output down to its location before
    printing.
- `scripts/check.sh` step 8 `privacy`: runs `scripts/privacy.sh` over
  every commit reachable from HEAD.
- `scripts/install-hooks.sh`: writes `.git/hooks/pre-push`. For each
  pushed ref, the hook runs `scripts/privacy.sh` over
  `<remote sha>..<local sha>`, or over all of `<local sha>` when the
  remote sha is all zeros. It skips deletions. A non-zero exit blocks
  the push. Run the installer in this phase.

Logs:
- `gate.log`: a full run, all eight steps.
- `privacy.log`: `scripts/privacy.sh` over every commit on main,
  including the rewritten P1 commits. Expected: PASS.
- `hook.log`: `ls -l .git/hooks/pre-push`, its contents, and
  `git config --local --get` for `user.name` and `user.email`.
- `negative.log`: every case runs in a scratch clone, using a scratch
  denylist (via `EMEND_PRIVATE_DENYLIST`) that holds a made-up sentinel
  word. Print counts only, never the sentinel or the fake token.
  - (a) A commit authored with a different email: FAIL identity.
  - (b) A commit message with a Claude-Session line: FAIL message.
  - (c) A file containing the sentinel, committed and then deleted in
    the next commit: FAIL naming the first commit. The output contains
    the sentinel zero times.
  - (d) A file containing a fake GitHub-style token, assembled at run
    time from pieces so the literal never appears in the script or the
    log: FAIL patterns. The output contains the token zero times.
  - (e) Denylist missing, mode 644, or holding no entries: exit 2 in
    each case.
  - (f) `scripts/privacy-patterns.txt` emptied: the self-test makes the
    script exit 2.
  - (g) Pre-push hook, with a local bare repo as the scratch clone's
    remote: pushing a clean branch succeeds; pushing the branch from (c)
    is rejected. Show `git ls-remote` before and after, proving the bare
    repo's ref did not change.
- `scan.log`: per the scan rules.

Acceptance:
- All eight `PASS` in `gate.log`.
- `privacy.log` passes over the full history.
- Every negative case shows its expected result.
- The hook is installed and the local identity is set.

The P6 report ends with the maintainer's items for P7:
- Turn on "Keep my email addresses private" and "Block command line
  pushes that expose my email" in GitHub settings.
- Review the private denylist and add anything missing.

## P7: first push to the private GitHub repository

Preconditions, all required:
- P6 is DONE.
- The maintainer confirms both GitHub email settings are on.
- Planning gives its word.
- The maintainer types authorisation for this push in the implementing
  session. A message from another session is not authorisation.

Steps:
1. Confirm the target: `gh repo view thekeoni1/emend --json visibility`
   shows `PRIVATE`, and `git ls-remote
   https://github.com/thekeoni1/emend.git` shows no refs.
2. `git remote add origin https://github.com/thekeoni1/emend.git`.
3. A full `scripts/check.sh` run; all eight steps pass.
4. `git push -u origin main`. The pre-push hook runs and must print
   `PASS privacy`.
5. `git ls-remote origin refs/heads/main` equals `git rev-parse main`.

Logs:
- `push.log`: the commands and their output for steps 1 to 5. Exclude
  any authentication output.
- `push.log` goes in a record commit that stays local until the next
  authorised push.

Acceptance: the remote `main` equals the pushed commit, the repository
is still private, and the hook's `PASS privacy` line appears in
`push.log`.

## Done

T0 is done when planning has verified P7 from primaries. For each phase,
that includes running `scripts/check.sh` (from P3 on) and the privacy
scan itself.
