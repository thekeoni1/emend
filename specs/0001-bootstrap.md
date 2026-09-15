# 0001: T0 repo bootstrap

Status: T0 DONE (P1 to P7 and P6c), verified by planning 2026-09-14 and 15. After
the rewrite, P1 is af5a2c6 to d3754dc; P1R is doc commit d495f0a with
records through 88d5644; P2 is 1a985a1 to 9daa4c6; P3 is ced7f91 to
53b99d8; P4 is 977635b to a00d6fd; P5 is d8c1ee1 to 9b7aef1; P5b is
873851f to 65cdeae; P6 is 10a42d6 to 28a44aa; P6b is 690c0f5 to
cf5240b; P7 pushed af4b355, with records 9c63579 to 302b1b2; P6c is
a371cff to 8e7e665; the close-out push is e0a5175 (pushed), with local
records 00c64bb and 41e2919. Next task: T2 P0
(`specs/0002-gate-corpus-additions.md`).
Context: `CLAUDE.md`, `specs/0000-step-zero-plan.md`.

## Goal

A repository that builds and tests on the host and on
`i686-unknown-linux-musl` from its first code commit. It needs one gate
script that enforces the CLAUDE.md rules a machine can check, including
a privacy gate, and a first push to the private GitHub repository. This
task writes no workbook model and no harness logic.

## How this task runs

T0 is P1, P1R, P2 to P5, P5b, P6, P6b, P7 and P6c, in that order. Each
phase ends with:

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
- Logs contain no control bytes other than tab, line feed and carriage
  return. Tools run with colour off (`CARGO_TERM_COLOR=never`, and
  `--color never` for rustfmt). Each phase's scan.log counts control
  bytes in its logs, with a positive control, and expects 0. The three
  lines with escape and shift-in bytes in
  `records/logs/t0-p3/negative.log` are accepted as recorded.
- Pushes: never use `git push --no-verify`, never set `core.hooksPath`,
  and never export `EMEND_PRIVATE_DENYLIST` except inside a negative
  test.
- A run stopped by the environment (for example for low memory) is noted
  in the header of the log that reruns it: its start time, the last
  check it completed, and the reason given.
- A check with more than one condition is evaluated inside one function
  that returns yes or no. The form `$(yn [ ... ] && cmd ...)` captures
  only the first test (T0b P2, T1 P1 check 1).
- A push script records the base (the remote ref before the push) and
  compares against that afterwards; the tracking ref moves with a
  successful push (T0b close-out).
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

## Planning rulings on P3 (2026-09-14)

- P3 is verified from primaries by planning's own checks:
  - Doc commit ced7f91 holds exactly the five pending paths. Its
    decision record is byte-identical to planning's draft.
  - Planning ran `scripts/check.sh` itself, from `/` by absolute path:
    PASS host, i686, fmt and clippy, exit 0. `--quick` prints PASS and
    SKIP lines and exits 0. A bad argument prints the usage line and
    exits 2.
  - Planning reproduced the three negative tests in fresh clones, and
    each exits 1:
    - a misformatted line gives FAIL fmt;
    - a test compiled out on x86 gives FAIL i686 with zero tests
      executed;
    - a failing assertion gives FAIL host.
  - `.gitignore` ignores `X.XLSX`, `sub/Y.Xlsm`, `Z.ODS` and `q.XlSb`.
    It does not ignore office files under `tests/fixtures/`, in either
    case.
  - Every file in every commit on main, and every message, has zero
    hits against the denylist, the shape-based P6 patterns and non-ASCII
    bytes.
  - The only control bytes in the tree are escape and shift-in bytes on
    three lines of `records/logs/t0-p3/negative.log`.
  - The GitHub repository still has no branches.
- Question 1: the SKIP lines under `--quick`, and exit 2 on a bad
  argument, are accepted.
- Question 2: the escape and shift-in bytes in the P3 negative.log are
  accepted as recorded. From P4 on, logs carry no colour and scans count
  control bytes (see "Rules for every phase"). P4 turns colour off in
  `scripts/check.sh`.
- Question 3: invoking `check.sh` through a symlink is out of scope. The
  script is run by its path.
- Question 4: a failure before the first step (for example a failed
  `mktemp`) exits non-zero without a FAIL line. Accepted.
- Question 5: the executed-tests count summed across the workspace is
  accepted while the workspace has one member. The T1 spec revisits it
  if T1 adds members.
- Question 6: `fail()` passing tool output to stderr unfiltered is
  accepted. A log that captures such output is committed only after its
  phase scan passes, and P6 scans the whole history before any push.
- Question 7: the P3 Logs list now names `ignore.log`.

## Planning rulings on P4 (2026-09-14)

- P4 is verified from primaries by planning's own checks:
  - Doc commit 977635b holds exactly the two pending paths.
  - Phase commit a713c93 changes only `scripts/check.sh` and adds an
    empty `scripts/sys-allowlist.txt`. The five record commits touch
    only `records/logs/t0-p4/`. All seven messages end with the
    Co-Authored-By trailer only.
  - Planning ran `scripts/check.sh` from `/`: all six steps PASS, exit
    0, zero control bytes in the output.
  - Planning tested step 5 with local fixture crates in fresh clones.
    `cc` as a build-dependency, a renamed dependency with
    `package = "cc"`, and `cc` under a Windows-only target table each
    give `FAIL native`.
  - Two cases passed unseen. `cc` behind a feature nothing enables
    passed `native`, and a crate named `fake_sys` passed `native`.
  - An `MPL-2.0` crate behind a feature nothing enables passed
    `licences`, because plain `cargo metadata` lists only the default
    resolve. With `--all-features`, `cargo tree` shows both hidden
    crates and `cargo metadata` lists the `MPL-2.0` crate. Both commands
    still succeed on the real workspace.
  - Every file in every commit on main, and every message, has zero
    hits against the denylist, the shape-based P6 patterns and non-ASCII
    bytes. The only control bytes in the tree remain the accepted P3
    lines. The GitHub repository still has no branches.
- Question 1: yes. `native` and `licences` run with `--all-features`
  (P5).
- Question 2: build-dependencies, renamed dependencies and target
  tables are caught, as shown above. They become permanent negative
  tests in P5.
- Question 3: yes. `native` also matches `pkg_config` and names ending
  in `_sys` (P5).
- Question 4: matching the allowlist by name only is accepted. The
  allowlist is empty, and any addition needs a planning ruling after a
  review of that crate's source, version and origin.
- Question 5: yes. On success, `native` names the allowlisted `-sys`
  crates it saw (P5).
- Question 6: lowercase SPDX identifiers and the slash form failing
  closed is accepted.
- Question 7: the negative.log final tree check ignoring
  `records/logs/t0-p4/` is accepted.
- The attempt logs, the self-naming slip in verify-attempt1.log and the
  weak controls are accepted as recorded. Planning's own scans cover
  both verify logs.

## Planning rulings on P5 (2026-09-14)

- P5 is verified from primaries by planning's own checks:
  - Doc commit d8c1ee1 holds exactly the two pending paths.
  - Phase commit 346385c changes only `scripts/check.sh`. The record
    commits touch only `records/logs/t0-p5/` and
    `records/toolchain-2026-09-15.md`. All six messages end with the
    Co-Authored-By trailer only.
  - Planning ran `scripts/check.sh` from `/`: all seven steps PASS,
    including `PASS native (allowlisted -sys: none)`, exit 0, zero
    control bytes in the output.
  - Planning reproduced negatives (a) to (f) in fresh clones. Each gives
    its expected FAIL: a staged U+2014 file, `Y.XLSM`, a dummy
    `ANTHROPIC_API_KEY` for one command, `cc` behind a feature nothing
    enables, `fake_sys`, and an `MPL-2.0` crate behind a feature nothing
    enables.
  - The toolchain record gives the kernel from `uname -r` and no
    hostname.
  - Every file in every commit on main, and every message, has zero
    hits against the denylist, the shape-based P6 patterns and non-ASCII
    bytes. The GitHub repository still has no branches.
  - Planning also reproduced two gaps with the committed script. A
    staged U+2014 file deleted from the working tree passes. An office
    file under a directory whose name contains a newline before
    `tests/fixtures/` passes.
- Questions 1 and 6: any tracked path containing a control byte fails.
  Paths in FAIL lines are printed git-quoted (P5b).
- Questions 2, 3 and 4: the U+2014 check reads the index with
  `git grep --cached -I`. A symlink is then checked as its target
  string, never followed. A file git treats as binary is not a text file
  for this rule (P5b).
- Question 5: match `emend-testing` case-insensitively. The
  `tests/fixtures/` exception stays case-sensitive, so it fails closed
  (P5b).
- Question 7: check `ANTHROPIC_API_KEY` before step 1 (P5b).
- Question 8: every grep form in step 7 gets an in-script positive
  control (P5b). Negative tests for grep error branches are not
  required.
- Question 9: not in T0. Logged negative runs are the evidence for every
  phase. A committed self-test script for the gate is an idea to put to
  the maintainer.
- Question 10: accepted. `--all-features` covers every feature a
  workspace member can enable. A feature that only a dependency's own
  configuration could enable is in no build of this workspace.
- P5 as committed stays as recorded; the changes above are P5b.

## Planning rulings on P5b (2026-09-14)

- P5b is verified from primaries by planning's own checks:
  - Doc commit 873851f holds exactly the two pending paths.
  - Phase commit aca5d3a changes only `scripts/check.sh`. The record
    commits touch only `records/logs/t0-p5b/`. All six messages end
    with the Co-Authored-By trailer only.
  - Planning ran `scripts/check.sh` from `/`: all seven steps PASS, exit
    0, zero control bytes in the output.
  - Planning reproduced these cases in fresh clones:
    - a staged U+2014 file deleted from the working tree gives FAIL
      hygiene;
    - an office file under a directory whose name holds a newline gives
      the count-only FAIL line, with zero control bytes;
    - a dummy `ANTHROPIC_API_KEY` for one command gives a single hygiene
      FAIL line before any other step.
  - Every file in every commit on main, and every message, has zero
    hits against the denylist, the shape-based P6 patterns and non-ASCII
    bytes. The GitHub repository still has no branches.
  - Planning also confirmed the risk behind question 7. A tracked path
    `Emend-Testing/corpus/private-name.txt` is printed in full in the
    hygiene FAIL line, so a captured log could carry a corpus file name.
- Scope of the gates, from now on:
  - The gates catch accidental mistakes by sessions on this machine.
    Deliberate repository misconfiguration, corrupted objects and a
    hostile environment are out of scope for `scripts/check.sh`.
  - The exception is the privacy gate: anything that could let private
    data reach GitHub is in scope.
- The hygiene step is closed with P5b. Edge cases that later checks find
  in it are recorded as findings and accepted, unless they could leak
  private data or let a push through. Those go to planning.
- Question 1, read errors: accepted for step 7. Required for
  `scripts/privacy.sh` (P6).
- Question 2: a bad argument with the key set gives the usage line and
  exit 2. Accepted, since nothing builds.
- Question 3, git attributes: accepted for step 7. `scripts/privacy.sh`
  ignores attributes (P6).
- Question 4, git environment variables: the throwaway repositories in
  `scripts/privacy.sh` and in step 7 run with them unset (P6). Git sets
  some of them when it runs hooks.
- Question 5: the raw stderr dump in error branches is accepted, as in
  P3 question 6.
- Question 6: unguarded commands in the control block are accepted.
- Question 7: yes. The corpus-path FAIL line prints a count, not the
  paths (P6).

## Planning rulings on P6 (2026-09-14)

- P6 is verified from primaries by planning's own checks:
  - Doc commit 10a42d6 holds the two pending paths (80 insertions, 7
    deletions).
  - Phase commit f1ac533 changes `scripts/check.sh` and adds
    `scripts/install-hooks.sh`, `scripts/privacy-identity.txt`,
    `scripts/privacy-patterns.txt` and `scripts/privacy.sh`. The record
    commits touch only `records/logs/t0-p6/`. All five messages end with
    the Co-Authored-By trailer only.
  - The repo-local identity matches `scripts/privacy-identity.txt`.
    `.git/hooks/pre-push` is executable and equal to the installer's
    output. `core.hooksPath` is unset.
  - Planning ran `scripts/check.sh` from `/`: all eight steps PASS, zero
    control bytes. `scripts/privacy.sh` over the full history: PASS, 46
    commits, 93 blobs.
  - Planning reproduced these cases in scratch clones, and no sentinel
    or token text appeared in any output:
    - a different author and committer email gives FAIL identity;
    - a Claude-Session line gives FAIL message;
    - a sentinel in a file added and then deleted gives FAIL naming the
      first commit only;
    - a fake token under `* binary` in `.git/info/attributes` gives FAIL
      patterns;
    - a removed loose object gives exit 2 with an ERROR line;
    - with the hook installed, a clean push to a local bare remote
      succeeds, while a push carrying a fake token is rejected and the
      remote never gets the ref.
  - Planning's own history scan found zero denylist and pattern hits,
    and no office or `emend-testing` path in any commit. The only email
    addresses are the two noreply addresses and addresses at
    `example.invalid`.
  - Planning confirmed two leak paths:
    - a gzip-compressed file at `emend-testing/corpus/private.xlsx`,
      committed and then deleted, gives PASS privacy;
    - with one scratch denylist holding an entry that a commit contains,
      exporting `EMEND_PRIVATE_DENYLIST` to a different scratch list
      makes that commit PASS, with no notice.
- Departure 1 (`git cat-file blob` with `grep -a` instead of `git grep`):
  accepted. It meets the ruling that attributes cannot exempt a file.
- Departure 2 (controls that do not cover every form): the pattern
  controls are replaced by the per-pattern self-test in P6b. The other
  uncovered forms are accepted.
- Departure 3 (the phase scan script's temporary denylist copies):
  accepted. They sit in the session scratch directory and are deleted
  on exit. `scripts/privacy.sh` itself never copies the denylist.
- Findings 1, 2, 4, 5, 6 and 7 could let private data reach GitHub, so
  they are in scope. P6b fixes them before any push:
  - 1: step 7's office-file FAIL line prints a count, not paths.
  - 2: the denylist override is announced, and refused by the hook and
    by step 8.
  - 4: every commit in scope is checked for corpus and office paths.
  - 5: every pattern is proven by its own fake value in the self-test.
  - 6: denylist entries must be single tokens.
  - 7: email addresses, drive-letter Windows user paths and Claude Code
    project paths are checked. The private denylist already holds the
    machine's hostname. Bare UUIDs are not added: they would flag
    legitimate text and reveal little on their own.
- Finding 3 (bypassing the hook): no code change. P7 checks the hook,
  `core.hooksPath` and `EMEND_PRIVATE_DENYLIST` before pushing, and runs
  the full gate itself. "Rules for every phase" forbids `--no-verify`.
- The lower-severity items are accepted as recorded. Compressed
  workbooks are covered by P6b's path check. Committed fixtures are
  fictional or generated, per CLAUDE.md.

## Planning rulings on P6b (2026-09-15)

- P6b is verified from primaries by planning's own checks:
  - Doc commit 690c0f5 holds the two pending paths (159 insertions, 6
    deletions).
  - Phase commit a087307 changes `scripts/check.sh`,
    `scripts/install-hooks.sh`, `scripts/privacy-patterns.txt` and
    `scripts/privacy.sh`. The record commits touch only
    `records/logs/t0-p6b/`. All five messages end with the
    Co-Authored-By trailer only.
  - The installed hook is identical to the installer's heredoc,
    `core.hooksPath` is unset, and the local identity matches.
  - Planning ran `scripts/check.sh` from `/`: all eight steps PASS, zero
    control bytes. `scripts/privacy.sh` over the full history: PASS, 51
    commits, 106 blobs.
  - Planning reproduced these cases in scratch clones, and no fixture
    text appeared in any output:
    - a gzip-compressed corpus workbook committed and then deleted gives
      two count-only FAIL lines naming the first commit;
    - an address at `example.com` in a file gives FAIL email;
    - patterns 10 and 11 each fire under their own number;
    - removing one pattern gives exit 2 at the count check;
    - with `EMEND_PRIVATE_DENYLIST` set, the NOTICE line comes first;
    - with it set, the pre-push hook prints the FAIL line and a local
      bare remote gets no ref;
    - without it, the push succeeds.
  - Planning scanned exactly what P7 pushes: one ref and 51 commits.
    - No commit has a header field beyond tree, parent, author and
      committer.
    - There are zero denylist, pattern and non-ASCII hits in raw
      commits, paths and blobs.
    - There is no office, CSV or archive path, and no zip or gzip
      content.
    - The only email addresses are the two noreply addresses and
      addresses at `example.invalid`.
  - On 2026-09-15 the maintainer reviewed the private denylist and added
    an entry (now 7 entries, none with whitespace). Neither that entry
    nor any other appears anywhere in the history or the working tree.
- Departures 1 to 7 are accepted.
- Findings 1, 2, 3, 5, 7 and 9 are fixed in P6c, after P7:
  - 1: email addresses in path names;
  - 2: more Windows path spellings;
  - 3: more spreadsheet types;
  - 5: the full commit header;
  - 7: office names followed by a suffix;
  - 9: archives detected by content signature.
  None of these forms exists in the history P7 pushes, and no push after
  P7 happens until P6c is verified (maintainer ruling 2026-09-15).
- Finding 4 (a redirected HOME) is accepted. It is a deliberate change
  to the environment and out of scope.
- Finding 6 (addresses next to non-ASCII bytes or in UTF-16) is accepted.
- Finding 8 (the U+2014 FAIL line prints paths) is accepted. It cannot
  print a path containing `emend-testing`, as check 2 corrected, and the
  hygiene step is closed.
- Finding 10 (unusual address forms) is accepted.

## Planning rulings on P7 (2026-09-15)

- P7 is verified from primaries and from GitHub by planning's own
  checks:
  - GitHub: `thekeoni1/emend` is PRIVATE, with one branch, `main` at
    af4b355, 52 commits and no tags. Every author and committer email
    GitHub reports is the noreply address in
    `scripts/privacy-identity.txt`.
  - Local `origin/main` is af4b355. The three record commits 9c63579,
    5e6d629 and 302b1b2 touch only `records/logs/t0-p7/`, end with the
    Co-Authored-By trailer only, and are not pushed.
  - `push.log`: the hook ran `scripts/privacy.sh` over 52 commits with 0
    findings (`PASS privacy`). Git reported `[new branch] main -> main`.
    Part 3 re-checks give `remote main equals local main: yes` and
    `still PRIVATE: yes`.
  - The three P7 record files have zero hits against the private
    denylist, the 11 patterns, non-ASCII bytes, control bytes and the
    email allowlist. `scripts/privacy.sh` over the 55 local commits:
    PASS.
- The GitHub email settings precondition was met before the push. The
  maintainer confirmed both settings to planning on 2026-09-14, and the
  spec asks for the maintainer's confirmation, not in-session wording.
  The maintainer confirmed again in the implementing session afterwards.
- The false "part 2 checks all as expected: no" is accepted as recorded.
  Part 3 and planning's GitHub check confirm the outcome. From now on,
  logs evaluate each check once and print that single result.
- 51 versus 52 commits is accepted. Planning's 51-commit scan came before
  doc commit af4b355. Planning scanned both doc files before that commit
  and ran `scripts/privacy.sh` over all 52 commits before the push.
- The missing P7 scan.log is accepted, because planning scanned all
  three P7 record files. P6c's scan.log also covers
  `records/logs/t0-p7/`.
- The rest are accepted as recorded:
  - the `ls -l` user and group names (`dev` identifies no person);
  - the new remote and tracking configuration;
  - the ROADMAP item not in the P6b spec;
  - the quoted search terms in check 2.

## Planning rulings on P6c (2026-09-15)

- P6c is verified from primaries by planning's own checks:
  - Doc commit a371cff holds the two pending paths (44 insertions, 4
    deletions).
  - Phase commit d8ca1bd changes `.gitignore`, `scripts/check.sh`,
    `scripts/privacy-patterns.txt` (still 11 lines) and
    `scripts/privacy.sh`. The record commits touch only
    `records/logs/t0-p6c/`. All five messages end with the
    Co-Authored-By trailer only.
  - Planning ran `scripts/check.sh` from `/`: all eight steps PASS, zero
    control bytes. `scripts/privacy.sh` over the 60 local commits: PASS.
  - Planning scanned the P6c logs, both P6c check answers and the three
    P7 files: zero hits against the denylist, the patterns, non-ASCII
    bytes, control bytes and the email allowlist. GitHub is unchanged at
    af4b355.
  - Planning reproduced these cases in scratch clones, with no fixture
    text in any output:
    - a LibreOffice lock-file name committed and deleted gives a
      count-only office FAIL, and an untracked `u.xlsx~` is ignored;
    - `data.csv` at the root gives FAIL, and the same under
      `tests/fixtures/` gives PASS;
    - a zip-signature blob named `notes.txt` gives FAIL archives, and
      the same under `tests/fixtures/` gives PASS;
    - an extra commit header line holding an address gives FAIL header;
    - an address in a file name gives FAIL paths, with no path printed;
    - the widened patterns 10 and 11 each fire.
  - Planning also confirmed two gaps: a tracked
    `r.xlsx:Zone.Identifier` gives PASS, and a blob with an OLE2
    signature gives PASS.
- Departures 1 to 9 are accepted.
- These go to T2 P0 (`specs/0002-gate-corpus-additions.md`), which runs
  before T1 and before any corpus file is collected (maintainer ruling
  2026-09-15):
  - finding 1, Zone.Identifier files;
  - finding 2, OLE2 compound files, which covers old-format and
    password-protected workbooks;
  - finding 3, other archive signatures;
  - finding 4, both forms of patterns 10 and 11 in the self-test;
  - the unchecked exit statuses in the email pipeline and in the
    signature helper.
- Renamed CSV, TSV or FODS files without a content signature are
  accepted. There is no reliable signature to detect them.
- T0 is DONE. The close-out push follows the section below.
- After T2 P0, new privacy-gate findings go to a backlog in ROADMAP.md
  unless they concern a data flow this project actually has.

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
  - `ignore.log`: the case-insensitive ignore results described above.
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

Also in this phase, turn colour off for every tool `check.sh` runs:
export `CARGO_TERM_COLOR=never`, and pass `-- --color never` to
`cargo fmt`. Rerun the fmt negative case, and show that its captured
output has zero control bytes.

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

Also in this phase, close the gaps planning found in steps 5 and 6
(rulings on P4):

- Run `cargo tree` in `native` and `cargo metadata` in `licences` with
  `--all-features`, so crates behind a feature nothing enables are
  seen.
- In `native`, match `pkg_config` as well as `pkg-config`, and names
  ending in `_sys` as well as `-sys`.
- On success, `native` names the allowlisted `-sys` crates it saw, for
  example `PASS native (allowlisted -sys: none)`.

Add these cases to this phase's `negative.log`. Each runs in a scratch
clone, with local fixture crates outside the clone:

- (d) `cc` behind a feature nothing enables: `FAIL native`.
- (e) a crate named `fake_sys`: `FAIL native`.
- (f) an `MPL-2.0` crate behind a feature nothing enables:
  `FAIL licences`.
- (g) `cc` as a build-dependency: `FAIL native`.
- (h) a renamed dependency with `package = "cc"`: `FAIL native`.
- (i) `cc` under `[target.'cfg(windows)'.dependencies]`: `FAIL native`.

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

## P5b: hygiene hardening

This phase applies the planning rulings on P5 to step 7 of
`scripts/check.sh`:

- Read what would be committed, not the working tree. Find U+2014 with
  `git grep --cached -I` on the byte pattern, under `LC_ALL=C`. A
  symlink is then checked as its target string and never followed. A
  file git treats as binary is not a text file for this rule.
- Before any other path check, fail if any tracked path contains a
  control byte, newline included. Report how many such paths exist, not
  the paths themselves.
- Match `emend-testing` in tracked paths case-insensitively. The
  `tests/fixtures/` exception stays case-sensitive.
- FAIL lines print paths the way `git ls-files` prints them without
  `-z`, so non-ASCII bytes appear quoted.
- Check `ANTHROPIC_API_KEY` before step 1, so a key never reaches a
  build script or test. It prints `FAIL hygiene: ANTHROPIC_API_KEY is
  set in the environment` and exits 1 before any other step runs. The
  value is never printed.
- Every grep form in step 7 has an in-script positive control, as the
  U+2014 scan already does.

- Logs:
  - `gate.log`: a full run, all seven steps.
  - `negative.log`: each case in a scratch clone.
    - (a) Stage a file containing U+2014, then delete it from the
      working tree: `FAIL hygiene`.
    - (b) Track an office file under a directory whose name contains a
      newline, before `tests/fixtures/`: `FAIL hygiene`. The captured
      output has zero control bytes.
    - (c) Track a symlink to a file outside the repository that
      contains U+2014: `PASS hygiene`.
    - (d) Track `Emend-Testing/x.txt`: `FAIL hygiene`.
    - (e) Run with `ANTHROPIC_API_KEY=dummy` for that one command only.
      The first output line is the hygiene FAIL line, and no `PASS
      host` line appears.
    - (f) Track an office file with a non-ASCII byte in its name:
      `FAIL hygiene`, naming the path in quoted form. The captured
      output has zero non-ASCII bytes.
    - (g) Repeat P5 negatives (a) and (b), and show they still fail.
  - `scan.log`: per the scan rules.
- Acceptance: all seven `PASS` in `gate.log`; every negative case shows
  its expected result.

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

Also in this phase (planning rulings on P5b):

- `scripts/privacy.sh` scans every blob as bytes, whatever git
  attributes say, so no `.gitattributes` or `.git/info/attributes` entry
  can exempt a file. A read error, or git reporting that it cannot read
  an object, makes the script exit 2. It never counts as zero hits.
- Its self-test, and any throwaway repository it creates, run with
  `GIT_DIR`, `GIT_WORK_TREE`, `GIT_INDEX_FILE`, `GIT_OBJECT_DIRECTORY`,
  `GIT_ALTERNATE_OBJECT_DIRECTORIES` and `GIT_COMMON_DIR` unset.
- A finding in a path containing `emend-testing`, in any case, prints
  `<corpus path>` instead of the path.
- In step 7 of `scripts/check.sh`, the corpus-path FAIL line prints a
  count, not the paths. The step's throwaway control repository also
  runs with the git variables above unset.

Add to `negative.log`:
- (h) With `* binary` in `.git/info/attributes`, a committed file
  holding a realistic fake token: FAIL patterns.
- (i) A blob made unreadable in a scratch clone (for example by removing
  its object file): exit 2.
- (j) Run `scripts/privacy.sh` with `GIT_INDEX_FILE` exported to a
  scratch path: the self-test still passes, and the scratch index is
  unchanged.
- (k) A tracked path `Emend-Testing/corpus/name.txt`: the hygiene FAIL
  line gives a count, and the captured output does not contain
  `name.txt`.

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

## P6b: privacy gate hardening

This phase applies the planning rulings on P6. Nothing is pushed until
it is verified.

Changes to `scripts/privacy.sh`:
- Paths in history: for every commit in scope, fail if any tree path
  contains `emend-testing` in any case. Also fail if a path has an
  office extension in any case (`xlsx`, `xlsm`, `xltx`, `xltm`, `xlsb`,
  `xls`, `ods`) outside a `tests/fixtures/` directory. Print a count per
  commit, never the paths.
- Denylist override: when `EMEND_PRIVATE_DENYLIST` is set, the first
  output line is `NOTICE privacy: using the denylist from
  EMEND_PRIVATE_DENYLIST`.
- Self-test:
  - For every pattern in `scripts/privacy-patterns.txt`, a realistic fake
    value built at run time must be detected under that pattern's own
    number.
  - The script exits 2 unless the file holds exactly the expected number
    of patterns, kept as a constant in the script.
- Denylist entries: an entry containing whitespace makes the script exit
  2. Each name, address or word is its own entry.
- Email addresses: any address in a message or in content fails, with
  three exceptions:
  - the address in `scripts/privacy-identity.txt`;
  - `noreply@anthropic.com`;
  - addresses at `example.invalid`.
  Findings print the location only, never the address.
- New patterns, bracketed like the others:
  - Windows user paths with a drive letter:
    `[A-Za-z]:[\\/][Uu]sers[\\/]`;
  - Claude Code project and transcript paths: `[.]claude/projects/`.

Changes to `scripts/check.sh` and `scripts/install-hooks.sh`:
- Step 7's office-file FAIL line prints a count, not paths.
- Step 8, and the pre-push hook written by the installer, refuse to run
  when `EMEND_PRIVATE_DENYLIST` is set. Each prints `FAIL privacy:
  EMEND_PRIVATE_DENYLIST is set` and exits 1.
- Rerun the installer in this phase.

- Logs:
  - `gate.log`: a full run, all eight steps.
  - `privacy.log`: `scripts/privacy.sh` over every commit on main.
    Expected: PASS.
  - `hook.log`: the installed hook, compared with the installer's
    output.
  - `negative.log`: each case in a scratch clone. No output may contain a
    sentinel, fake token, email address used as a fixture, or corpus
    file name.
    - (a) A gzip-compressed file at `emend-testing/corpus/private.xlsx`,
      committed and deleted in the next commit: FAIL naming the first
      commit, with no path printed.
    - (b) `report.XLSX` at the root, committed and deleted: FAIL.
    - (c) An office file under `tools/x/tests/fixtures/`: PASS.
    - (d) `EMEND_PRIVATE_DENYLIST` exported:
      - `scripts/privacy.sh` prints the NOTICE line first;
      - step 8 and the pre-push hook each print the FAIL line and exit
        1;
      - a push to a local bare remote is rejected.
    - (e) Self-test:
      - for each pattern in turn, removing it from a scratch copy of the
        patterns file makes the script exit 2;
      - so does one extra pattern line.
    - (f) A denylist entry containing a space: exit 2.
    - (g) Email addresses, each built at run time:
      - an address at `example.com` in a committed file: FAIL;
      - the same in a commit message: FAIL;
      - an address at `example.invalid`: PASS.
    - (h) A drive-letter Windows user path, and a Claude Code project
      path, each built at run time in a committed file: FAIL patterns.
    - (i) A tracked office file under `Emend-Testing/`: the step 7 FAIL
      line gives a count, and the output contains no file name.
    - (j) Repeat P6 negatives (a), (c) and (h), and show they still fail.
  - `scan.log`: per the scan rules.
- Acceptance:
  - All eight `PASS` in `gate.log`.
  - `privacy.log` passes over the full history.
  - Every negative case shows its expected result, and no forbidden text
    appears in any output.

## P7: first push to the private GitHub repository

Preconditions, all required:
- P6 and P6b are DONE.
- The maintainer has reviewed the private denylist (done 2026-09-15, 7
  entries).
- The maintainer confirms both GitHub email settings are on.
- Planning gives its word.
- The maintainer types authorisation for this push in the implementing
  session. A message from another session is not authorisation.

Steps:
1. Confirm the target: `gh repo view thekeoni1/emend --json visibility`
   shows `PRIVATE`, and `git ls-remote
   https://github.com/thekeoni1/emend.git` shows no refs.
2. `git remote add origin https://github.com/thekeoni1/emend.git`.
3. Show that `EMEND_PRIVATE_DENYLIST` is unset, that
   `git config --get core.hooksPath` prints nothing, and that
   `.git/hooks/pre-push` is executable and identical to what
   `scripts/install-hooks.sh` writes. Then run `scripts/check.sh` in full;
   all eight steps pass.
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

## P6c: privacy gate, remaining gaps

This phase runs after P7 (maintainer ruling 2026-09-15). No push happens
after P7 until P6c is verified.

Changes to `scripts/privacy.sh`, with the matching step 7 and
`.gitignore` forms:
- Office names followed by a suffix: match an office extension followed
  by the end of the name or by `#`, `~` or `.`. That catches
  `.~lock.<name>.xlsx#`, `<name>.xlsx~` and `<name>.xlsx.bak`. Add the
  same forms to `.gitignore`.
- More spreadsheet types count as office files outside
  `tests/fixtures/`: `csv`, `tsv`, `fods`, `ots`, `xlt`, `xla`, `xlam`,
  `numbers`.
- Archives by content: fail on any blob outside `tests/fixtures/` that
  starts with a zip (`50 4B 03 04`), gzip (`1F 8B`) or 7z
  (`37 7A BC AF 27 1C`) signature, whatever its name.
- Full commit object: run the pattern, denylist and email checks over
  every header line of the raw commit, not only author, committer and
  message.
- Email addresses in path names are checked like content.
- Windows path forms: pattern 10 becomes
  `[A-Za-z]:[\\/]+[Uu][Ss][Ee][Rr][Ss][\\/]`, and pattern 11 becomes
  `[.]claude[\\/]+projects[\\/]`. The pattern count stays 11.

- Logs:
  - `gate.log`: a full run, all eight steps.
  - `privacy.log`: `scripts/privacy.sh` over every commit on main.
    Expected: PASS.
  - `hook.log`: the installed hook, compared with the installer's
    output.
  - `negative.log`: each case in a scratch clone. No output may contain
    any fixture text.
    - (a) `.~lock.name.xlsx#`, `name.xlsx~` and `name.xlsx.bak`, each
      committed and deleted: count-only FAIL. Untracked, each is
      ignored.
    - (b) `data.csv` at the root: FAIL. `tests/fixtures/data.csv`:
      PASS.
    - (c) A blob with a zip signature named `notes.txt` outside
      `tests/fixtures/`: FAIL. The same under `tests/fixtures/`: PASS. A
      gzip signature blob and a 7z signature blob: FAIL.
    - (d) A commit object carrying an extra header line with an address
      at `example.com`, built at run time: FAIL.
    - (e) A file whose name contains an address at `example.com`: FAIL,
      with the name not printed.
    - (f) Each built at run time, each FAIL patterns:
      - a drive-letter path with doubled backslashes;
      - one with `USERS` in capitals;
      - a Claude Code project path with backslash separators.
    - (g) Repeat P6b negatives (a), (d) and (g), and show they still
      fail.
  - `scan.log`: per the scan rules. It also covers the committed P7 logs
    in `records/logs/t0-p7/`.
- Acceptance:
  - All eight `PASS` in `gate.log`.
  - `privacy.log` passes over the full history.
  - Every negative case shows its expected result, and no fixture text
    appears in any output.

## Close-out push (after P6c)

Pushes the local P7 and P6c record commits. It adds no code.

Preconditions, all required:
- P6c is DONE (planning, 2026-09-15).
- Planning gives its word.
- The maintainer types authorisation for this push in the implementing
  session.

Steps:
1. Commit planning's pending doc updates, unedited, as the first commit.
2. Show that `EMEND_PRIVATE_DENYLIST` is unset, that
   `git config --get core.hooksPath` prints nothing, and that
   `.git/hooks/pre-push` is executable and identical to what
   `scripts/install-hooks.sh` writes. Run `scripts/check.sh` in full; all
   eight steps pass.
3. `git push origin main`. The pre-push hook must print `PASS privacy`.
4. `git ls-remote origin refs/heads/main` equals `git rev-parse main`,
   and the repository is still PRIVATE.

Logs:
- `records/logs/t0-close/push.log`, with every check evaluated once. It
  goes in a local record commit that is pushed with the next push.
- Report and STOP. The next task is T2 P0.

Planning ruling (2026-09-15): the close-out push is verified and DONE.
- GitHub `thekeoni1/emend` is PRIVATE, with one branch, `main` at
  e0a5175, 61 commits and no tags. All 122 author and committer emails
  are the noreply address.
- `push.log` shows 22 checks and 2 part summaries, each evaluated once
  and each yes. The hook printed `PASS privacy` over 9 commits, and git
  reported `af4b355..e0a5175  main -> main`.
- The two record files have zero hits on every scan. `scripts/privacy.sh`
  over the 63 local commits: PASS.
- Planning's message gave "83 insertions" for the two edited doc files,
  but the diff planning had run showed 85. The error was planning's; the
  committed files are unaffected.
- The double colon `push.log` prints for an empty status is accepted as
  cosmetic.

## Done

T0 is done when planning has verified P6c from primaries. That happened
on 2026-09-15. For each phase,
that includes running `scripts/check.sh` (from P3 on) and the privacy
scan itself.
