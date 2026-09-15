Checked phase: T0 P5 (specs/0001-bootstrap.md)
Checked commit: 346385c8c8934c3485b27160a69fd032f35ef8b5 (P5 phase commit); logs in fadc53f422f0348c0a45b257382230f38daac0bb
HEAD when the check ran: fadc53f422f0348c0a45b257382230f38daac0bb
Checker: a fresh general-purpose subagent, not a fork
Inputs (sha256):
- specs/0001-bootstrap.md: d6e394c169016172b54af1d46c85fff2d39c26ca8ac41792b3a49693b1ff70b5
- <scratch>/p5-diff.txt (git diff a00d6fd27dea6b43e3f99f836c08e4cd1683ff13..346385c8c8934c3485b27160a69fd032f35ef8b5): c9697db0e6d39f8b6e60ce9c4d38991220cbdc11c07bc2f28f18f22c0bd1fd2d
- records/logs/t0-p5/gate.log: ba710008e8fbc52155d0f11755f96256b4fe76a9cc5d5ebc7c6982dd39cea552
- records/logs/t0-p5/negative.log: 4af428c1f44f4e3fe2feb09c5d18d832f140718a35e296feb9df20b047cf9cd8
- records/logs/t0-p5/scan.log: c7978f02d033990092984aa021dfaeecca12b6500244119339679d258747fefc
- records/toolchain-2026-09-15.md: 5a6834de5c46c10ae9d6bde992aa736659008c67985112788be11ff14d1ea013
- <scratch>/p5-report-draft.md (draft 1): 067710e72da3a3a8c0b96df05a8aef356aa23c8a3c57ce7a7ab7ab20044b3899
---
# T0 P5 independent check 1

Note on context: besides the seven inputs, the harness put other material into my context: the repository's CLAUDE.md, an index of the user's auto-memory, a git status snapshot with a list of recent commits, a user identity detail, commit attribution guidance and environment details such as a date. I did not use any of it as evidence, and I do not repeat the identity details. Line numbers for the diff are line numbers in the diff file as given to me.

## Q1. Does the diff do what the P5 spec says and nothing else?

PARTLY. Every item the spec names is implemented, and nothing unrelated was changed. But the hygiene step has real edge-case holes where it can pass when it should fail, and some conditions are proven by no primary.

What matches the spec:

- Doc-commit ruling. The diff's ROADMAP.md and specs/0001-bootstrap.md hunks (diff lines 1-14 and 126-225) are the planning updates. records/logs/t0-p5/scan.log lines 248-258 show d8c1ee1 is the first commit after a00d6fd and touches only those two paths, and 346385c touches only scripts/check.sh. scan.log lines 265-288 and 301-309 scan both files and the doc commit's message. The + side of the specs hunks matches the spec text I was given (status lines 3-6, "Planning rulings on P4", the P5 additions). The insertion and deletion counts in the hunk headers add up to 73 and 6 (ROADMAP +2/-2; specs +4/-4, +45, +22). No primary shows that the doc files were committed "unedited". The diff can only show what was committed, not what planning left.
- Step 7, positive control: it runs first (diff lines 75-79). It writes the E2 80 94 bytes with `printf '\342\200\224'` and detects them with `LC_ALL=C /usr/bin/grep -l -I -F`. `[ "$rc" -eq 0 ] || fail` fails on exit 1 and on exit 2 or more. This matches the spec.
- Condition 1, U+2014: diff lines 81-102. It uses the same grep form under LC_ALL=C. `rc -ge 2` gives a FAIL (line 96) and `rc -eq 0` gives a FAIL (line 99). `--` guards names that start with `-`. `git ls-files -z` plus `mapfile -d ''` keeps names with spaces intact.
- Condition 2, office extensions: diff lines 104-112. `grep -i -E '\.(xlsx|xlsm|xltx|xltm|xlsb|xls|ods)$'` covers the P2 list in any case. The `(^|/)tests/fixtures/` exclusion is case-sensitive, which fails closed. Both greps treat exit 2 or more as an error.
- Condition 3, `emend-testing` in a tracked path: diff lines 114-119. It is a fixed-string match and handles exit 2.
- Condition 4, the API key: diff lines 121-124. `${ANTHROPIC_API_KEY+set}` also catches an empty value, and the value is never printed.
- native: `--all-features` is added (line 31) and `pkg_config` is banned (line 37). The name match becomes `/[-_]sys$/` (line 45). On a pass it prints `PASS native (allowlisted -sys: <names or none>)` (lines 41-58). The FAIL text changed to name `_sys` (line 54). That change is a consequence of the fix, not scope creep.
- licences: `cargo metadata --format-version 1 --all-features` (line 64).

Defects and holes in step 7 (none of them is exercised by any primary):

1. Staged but deleted, or index different from the working tree. Line 88 keeps only paths where `[ -f "$f" ]` holds, and line 95 greps working-tree content. Suppose a file with U+2014 is staged and then removed from disk, or its working copy is cleaned while the index still holds the dash. The step passes, yet the content that would be committed contains U+2014. The same applies to skip-worktree or sparse entries. The draft report's note 6 discloses "working-tree content, not the index", but it does not call this a hole.
2. A newline in a path bypasses the fixtures exception. Line 87 writes paths to `tracked.txt` with `printf '%s\n'`, so the NUL-safe list becomes newline-delimited again. Take a tracked path whose first component is `a<LF>tests`, for example `a<LF>tests/fixtures/b.xlsx`. It becomes the lines `a` and `tests/fixtures/b.xlsx`. The second line matches `(^|/)tests/fixtures/` and is excluded, so the step passes although the file is not in a `tests/fixtures/` directory. Other newline splits fail closed.
3. The binary heuristic. `-I` (line 95) skips any file that GNU grep decides is binary, which in practice means a NUL byte early in the file. A text file with one NUL byte before the dash escapes the scan. The spec says "text file" but does not define it. The draft report presents `-I` as matching the spec without naming this.
4. Symlinks. `-f` follows a symlink, so a tracked symlink is scanned through its target's content, which can lie outside the repo and cause a false FAIL. A dangling symlink whose target string contains the dash is skipped.
5. Fail when it should pass: all present files go to one grep call (line 95). A very large tree could hit the argument-length limit. The exec error (126) is at least 2, so the step fails closed, but falsely.
6. An empty `tracked` array under `set -u` (line 86) is safe only on bash 4.4 or later. No primary records the bash version.
7. The `emend-testing` match is case-sensitive, so `Emend-Testing` passes. The spec does not specify case.
8. The scan rules say every scan form has a positive control, and P3 says every grep in check.sh follows the scan rules. Step 7 has an in-script control only for the U+2014 form. The office, fixtures-exclusion and `emend-testing` greps have none, although negative.log cases b1 and b2 act as external controls for the office grep.

Observations, not spec violations:

- Step 7 runs last, so an earlier failure hides hygiene findings, and an API key in the environment is visible to cargo build scripts and tests in steps 1-6 before hygiene fails.
- FAIL lines print tracked path names, including any path that contains `emend-testing`. That could carry private file names into a captured log.
- Whether `--quick` still runs step 7 is not visible in the diff.

What the primaries cannot show:

- The parts of check.sh outside the hunks: `set -euo pipefail`, `$OUT`, the change to the repo root, `fail()`, the colour settings, and whether `RUSTUP_AUTO_INSTALL` is set there.
- That the spec file I read is the committed blob. scan.log line 281 gives its sha256, but I could not compute a hash.
- Whether other logs exist under the phase's logs directory.

## Q2. Do the logs contain every line the P5 spec requires?

PARTLY. Every required log line is present. The weak points are "git status is clean afterwards", which a primary shows only at intermediate moments, and a few mislabels and inconsistencies.

gate.log (records/logs/t0-p5/gate.log):

- Line 1 names the phase commit and HEAD, as the first-line rule requires.
- The full run starts from `/` after `cargo clean` (lines 125-127). Lines 129-135 hold all seven PASS lines, including `PASS native (allowlisted -sys: none)` and `PASS hygiene`. `exit: 0` is line 137, and the exact seven lines in order are confirmed at line 138.
- Control bytes are 0 in stdout and stderr, with a positive control and a tab/CR control (lines 141-144).
- The script is printed (lines 7-117). `RUSTUP_AUTO_INSTALL=0` and `CARGO_TERM_COLOR=never` are exported (script line 19; log line 122).
- `status after the run (expect empty):` is empty (line 169).

negative.log (records/logs/t0-p5/negative.log):

- Line 1 follows the first-line rule. Line 4 says `DRYRUN_CHECK_SH is not set`, so each clone runs the committed check.sh. Each case uses a fresh clone at 346385c, and fixtures live in `<scratch>/ext`, outside the clone (line 3; script lines 54-67).
- (a): the file is written with printf octal escapes (script line 101) and staged. The log shows `FAIL hygiene: U+2014 in tracked file(s): notes.txt`, exit 1 (lines 224-229).
- (b): `x.xlsx` is force-added and gives FAIL, exit 1 (lines 231-245). `Y.XLSM` is force-added in a separate clone and gives FAIL, exit 1 (lines 247-261). Weak: unlike (a), b1 and b2 do not assert that `PASS hygiene` is absent. The exit code and first-failure behaviour cover this.
- (c): `env ANTHROPIC_API_KEY=dummy scripts/check.sh` runs inside a subshell (script line 84). The log shows FAIL (line 272), the value absent from the output (line 277), and `ANTHROPIC_API_KEY set in this shell after case c: no` (line 278). The log can show only that the script's own shell never had the variable. It cannot show the state of the parent session, although the mechanism makes export impossible.
- (d) to (i): each shows the expected FAIL line with a non-zero exit (lines 296, 318, 344, 368, 389, 410). (f) also shows `PASS native (allowlisted -sys: none)` first (line 343). Nothing shows that (d), (e) or (f) would have passed under the P4 script. The before state rests on planning's P4 rulings, which is acceptable.
- (j) goes beyond the spec. It supports "allowlisted names on pass": `PASS native (allowlisted -sys: fake-sys fake_sys)`, all seven PASS, exit 0 (lines 432-444). No case tests an office file under `tests/fixtures/` passing, or a tracked `emend-testing` path failing. The spec does not require either, but condition 3 is therefore proven by no primary.
- Ending lines: `main working tree status (expect empty):` is empty (line 447), and line 448 reads `all negative cases as expected: yes`.

scan.log (records/logs/t0-p5/scan.log):

- Scan rules: the script starts with `#!/usr/bin/env bash` and calls `/usr/bin/grep` explicitly (line 18). It matches bytes under LC_ALL=C and does not use `\x{2014}`. Exit 2 or more counts as an error (line 58). Every form has a positive control (lines 198-244), and the script is printed.
- Coverage: the three diff files at 346385c, both commit messages (each ending with the Co-Authored-By line, lines 293 and 302), gate.log, negative.log and the toolchain record. Control bytes are counted with a positive control and 0 is expected. Result: 0 hits, 0 errors, `RESULT: PASS` (lines 339-342).
- Mislabel: the toolchain record is printed and counted as a "log" (line 329, and "logs scanned: 3" at line 337).
- The record commit's message is scanned by no primary, because it did not exist yet.
- The message check proves the last line is the Co-Authored-By line and that no `^Claude-Session:` line exists. It does not prove there are no other trailers.

Toolchain record (records/toolchain-2026-09-15.md):

- It contains rustc (line 7), cargo (line 12), the installed targets (line 14), the kernel from `uname -r` only (line 15), `file` output with a repo-relative path saying ELF 32-bit and statically linked (line 16), and the commit the gate passed at (line 6). All of these match gate.log lines 153-167.
- Its first line names no commit. The first-line rule covers logs, and the record is not a log in the spec's list, so this is weak rather than failed.
- "No toolchain or target was installed in P5" (line 4) and the guard line (line 17) are fixed text in gate.log's script (script lines 101 and 111), not measurements.
- The file name date, 2026-09-15, is UTC and matches gate.log line 2.
- The binary comes from a `--no-run` build after the gate (gate.log line 147). The spec asks only for "the i686 test binary", so this is acceptable.

"git status is clean afterwards": gate.log line 169 and negative.log line 447 show a clean tree at the end of each run. Both runs started in the same second (gate.log line 2 and negative.log line 2), and neither covers the state after the logs were moved in or after the record commit. No primary shows the final clean state.

Log privacy and control bytes: scan.log finds 0 hits against the hostname, P6, denylist and control-byte forms for gate.log, negative.log and the record. Paths appear as `<repo>` and `<scratch>` or repo-relative. No check failed in any log.

## Q3. Does every number and quoted string in the draft report appear in a primary?

PARTLY. Most items trace to a primary. Several do not, and some of those are not marked as coming from outside the primaries.

Verified:

- Range a00d6fd27dea...346385c8c8934c3485b27160a69fd032f35ef8b5: scan.log lines 1 and 247.
- d8c1ee1741c29363b8d31d1e234d5d7e3668a817, its subject, parent a00d6fd, and `M ROADMAP.md` / `M specs/0001-bootstrap.md`: scan.log lines 249-254.
- 346385c8c8934c3485b27160a69fd032f35ef8b5, its subject, parent d8c1ee1, and `M scripts/check.sh`: scan.log lines 248 and 255-258.
- `2 files changed, 73 insertions(+), 6 deletions(-)`: the report says it read this stat in the session before `git add`. The same counts follow from the diff's hunk headers for the two doc files (see Q1).
- Step 7 items (`printf '\342\200\224'`, `LC_ALL=C /usr/bin/grep -l -I -F`, `git ls-files -z`, the extension regex, case-sensitive `tests/fixtures/`, `emend-testing`, the empty-string key case): diff lines 75-124. The grep in the extension quote is abbreviated (no `LC_ALL=C /usr/bin/`).
- Step 5 and 6 items: diff lines 31, 37, 45, 54, 58 and 64. The literal `scripts/sys-allowlist.txt` in the FAIL text is from negative.log line 318.
- Toolchain values (rustc and cargo strings, targets, kernel, binary path, the `file` excerpt, "Full gate (all seven steps) passed at: 346385c..."): toolchain record lines 6-16 and gate.log lines 153-167.
- The seven PASS lines, `exit: 0 (expect 0)`, `stdout is exactly the seven PASS lines in order: yes`, `stderr bytes: 0`, the stdout and stderr control lines, the controls `lines=1 bytes=3` and `lines=0 bytes=0`, and `ANTHROPIC_API_KEY set in this shell: no`: gate.log lines 4 and 129-144.
- Negative cases:
  - `DRYRUN_CHECK_SH is not set`: negative.log line 4.
  - `<scratch>/ext`: negative.log line 3.
  - (a): line 224.
  - (b): lines 241 and 257.
  - (c): command at line 265, FAIL at line 272, value absent at line 277, shell after case c at line 278.
  - (d): line 296.
  - (e): line 318.
  - (f): lines 343-344.
  - (g), (h), (i): lines 368, 389 and 410. The heading `foo = { package = "cc" }` is line 376.
  - (j): lines 432-439.
  - The final line, `all negative cases as expected: yes`: line 448.
- The two empty status lines: gate.log line 169 and negative.log line 447.
- `controls: all as expected` is scan.log line 244. The figure 46 is not printed, but counting lines 198-243 gives 46, and the report says it is a count.
- `files in diff: 3; files scanned: 3; files read: 3`, `messages scanned: 2; commits in range: 2`, `logs scanned: 3 (expected 3)`, `total hits: 0`, `grep or read errors: 0`, `structural checks failed: 0`, `RESULT: PASS`: scan.log lines 289, 310, 337 and 339-342.
- sha256 values: gate.log's ba7100... is scan.log line 313, negative.log's 4af428... is line 321, and the record's 5a6834... is line 329. check.sh's c722e9... is scan.log line 273 and gate.log line 120.
- Note 4: the record's date is record line 1; the spec's 2026-09-14 dates are spec lines 3 and 98.
- Note 7: `ANTHROPIC_API_KEY=dummy` is in negative.log lines 263 and 265, and spec line 504 prescribes it.
- In "Open": `RUSTUP_AUTO_INSTALL=0` in the gate.log and negative.log scripts (gate.log line 19, negative.log line 21) and the record's guard line (record line 17). "S's script runs no cargo" is true of scan.log lines 6-189.

In no primary, and marked as such: fadc53f422f0348c0a45b257382230f38daac0bb; scan.log's own sha256 c7978f02...; the pre-commit stat and pre-commit scan; "unedited"; the clean `git status --short` after the record commit; that the committed files match the working-tree hashes; the dry run at d8c1ee1 and its result; that gate.log and negative.log ran at the same time (marked as a session fact); and that there is no remote and no push.

Not supported by a primary and not marked:

1. "The primaries show `RUSTUP_AUTO_INSTALL=0` in check.sh (D)". The string does not appear anywhere in the diff. It appears only in the scripts printed in gate.log and negative.log, which export it before calling check.sh.
2. "Every case prints `workspace members: xlsx-diff` where fixtures are involved". This is false for case (j). It uses two fixture crates, but its script calls no `members` step (negative.log script lines 193-200) and its log has no such line (lines 418-444). The line appears only for (d) to (i).
3. Note 1: "P4's verify.log v6 showed this", and the claim that a feature of a crate outside the workspace stays out of every resolve even with `--all-features`. Neither is in any P5 primary.
4. Note 5: "check.sh captures its output" is not visible in the diff, and "Its name ... is the same as P2's" relies on a P2 log that is not a P5 primary.
5. Note 3: "their outputs were moved into the repo after both finished". The printed scripts say output goes to scratch and is moved afterwards (gate.log lines 10-11 and 3, negative.log lines 12-13), but "after both finished" is not shown.
6. Spec item 1, "`-I` skips binary files, which matches the spec's tracked text file". That is interpretation, stated without the heuristic's limits.

Honesty: the report is careful about the record commit, the "unedited" doc files, the committed-versus-working-tree hashes, the dry run and the push state. It discloses the working-tree-only scan and the case-sensitive fixtures match. It does not mention the holes from Q1 (staged but deleted files, newline paths, NUL bytes, symlinks), the lack of controls for the other step 7 grep forms, the absence of any test of the `emend-testing` condition, or that "git status clean afterwards" is shown only before the logs were moved in.
