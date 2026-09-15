Checked phase: T0 P6 (specs/0001-bootstrap.md)
Checked commit: f1ac533129120d9801d41fd14d416976d03dc282 (P6 phase commit); logs in 0139a7ec6c83df9ae9e20dd34e10b4d7044e41d7
HEAD when the check ran: 0139a7ec6c83df9ae9e20dd34e10b4d7044e41d7
Checker: a fresh general-purpose subagent, not a fork
Inputs (sha256):
- specs/0001-bootstrap.md: 6519dfb7e36fb73043f466240d8a8dd6a0e57a44421f63f96bcd68449f191e5e
- <scratch>/p6-diff.txt (git diff 65cdeae89a6e3fc49c6f303ffde9c45a816ee953..f1ac533129120d9801d41fd14d416976d03dc282): 3d18778db74e1d0242c122f88505031a32b6e9d2e389dc8ef46a3e2adcc910b6
- records/logs/t0-p6/gate.log: 995d076e42b9819d4ad2c714e3d17efe3ac7f4c0beb53c2e814f8cd18575279c
- records/logs/t0-p6/privacy.log: c8e590263b08d81d8235d0933a04d6588fb4f3d4b142b6b978fbda55b667b89e
- records/logs/t0-p6/hook.log: 1847210b742deee9a4745676f4a074156134e206fbf65f498870ee6d8357e7fe
- records/logs/t0-p6/negative.log: 63b6697bd9cfa40937a068ef8fa5a3384289354371710b43fd9b53a83cdcd849
- records/logs/t0-p6/scan.log: a418f6074b9475829fd51471f444225534c445b6c87a5c2319cf5cb4ca436f2d
- <scratch>/p6-report-draft.md (draft 1): b479877f7b93daf950087fe38e1922357807c9770bc894068a3e11f000df3bcc
---
# Independent check: T0 P6 privacy gate

Inputs read: the spec, `<scratch>/p6-diff.txt`, `<scratch>/p6-report-draft.md`, and `records/logs/t0-p6/` gate.log, privacy.log, hook.log, negative.log and scan.log. Nothing else was read. The harness also put CLAUDE.md into this checker's context. I did not use it as evidence.

## Q1

**Deliverables, one by one**

1. **Repo-local identity.** Git config is not in a diff. hook.log shows it is met:
   - lines 162-165: `thekeoni1` and `55367154+thekeoni1@users.noreply.github.com`
   - line 166: `equal to scripts/privacy-identity.txt lines 1 and 2: yes`
2. **`scripts/privacy-identity.txt`.** Met. Diff lines 173-174 hold the name on line 1 and the address on line 2.
3. **`scripts/privacy-patterns.txt`.** Met.
   - Diff lines 181-189 hold patterns 1 to 9, in the spec's order, character for character.
   - The file does not match itself: scan.log line 315ff shows `form=f_p6 count=0 exit=1`.
   - scan.log line 205 shows the scan script's copy equals the file.
4. **`scripts/privacy.sh`.**
   - **Scope.** Met. The default is `RANGE=HEAD`, and `--range <rev>` is passed to `git rev-list` (diff lines 218-232, 477).
   - **Output and exit codes.** Met. It prints `PASS privacy` or one `FAIL` line per finding, and exits 0, 1 or 2 (diff lines 484-490; errors go through `error()`, exit 2).
   - **Self-test.** Met. It makes a throwaway repository with a run-time token and sentinel, runs the real per-commit code over it, and exits 2 if a planted hit is missed (diff lines 434-467).
   - **Scan rules.** Met, with one partial gap.
     - It uses `/usr/bin/grep` under `LC_ALL=C`, non-ASCII as `-P '[^\x00-\x7F]'`, and treats grep exit 2 or more as an error. The `hit_lines` pipeline checks `PIPESTATUS[0]`.
     - Partial gap: the rule "every scan form has a positive control that uses the same command form" is only partly met inside the script. The self-test controls only the pattern form (`-E`), the denylist form (`-i -w -F -f`) and the message pattern scan. These forms have no in-script control:
       - the non-ASCII message form;
       - the `^Claude-Session:` form;
       - the identity comparison;
       - the path-name scan;
       - the `<private path>` lookup (`-x -F`);
       - the `emend-testing` path grep.

       negative.log cases (b), (p), (m) and k2 exercise some of these at f1ac533. Nothing exercises `<private path>`. The report does not call this a scan-rule shortfall; it only notes that no case exercises `<private path>` or `info/grafts`.
   - **Check 1, identity.** Met. The local config is checked once (diff lines 472-475). Each commit's author and committer name and email are checked from the raw header (diff lines 363-374).
   - **Check 2, message.** Met: non-ASCII, `^Claude-Session:`, patterns and denylist (diff lines 386-392).
   - **Check 3, content.** Departure. The spec says "via `git grep -I -n`". The diff uses `git cat-file blob` plus `/usr/bin/grep -a -n` (diff lines 404-430). The "Also in this phase" ruling (every blob scanned as bytes, whatever attributes say) justifies it. **Disclosed** as Rule departure 1.
   - **Denylist path and override.** Met (diff line 251). Nothing restricts the override to negative tests. See Defects.
   - **Denylist existence, mode and entries.** Met: it must exist, be mode 600, and hold at least one entry, with `#` comments (diff lines 291-303). Beyond spec, entries are whitespace-trimmed and blank lines skipped. **Disclosed** in item 3 ("Entries are trimmed"), but not in the departures list.
   - **Matching.** Met: fixed string, case-insensitive, whole word (`-i -w -F`).
   - **Never printed or copied.** Met as far as the diff shows. Entries reach grep only through `-f <(printf ...)`.
   - **What findings print.** Met: check, number, commit, path and line. Only line numbers leave grep (`cut -d: -f1`).
5. **`check.sh` step 8.** Met (diff lines 97-106). It runs `scripts/privacy.sh` with the default scope.
6. **`install-hooks.sh`.** Met (diff lines 107-166).
   - It writes the pre-push hook.
   - The range is `remote..local`, or just `local` when the remote sha is all zeros.
   - Deletions are skipped, and any non-zero exit blocks the push.
   - The installer was run: hook.log lines 116-118.
7. **Ruling: bytes whatever attributes say, and read errors exit 2.** Met.
   - `cat-file` plus `grep -a` ignores attributes.
   - `run_git` makes a non-zero exit or any stderr output an exit 2.
8. **Ruling: git variables unset in throwaway repositories.** Met. `GIT_CLEAN` unsets the six variables for the self-test repository (diff lines 254-255, 438, 442).
9. **Ruling: `<corpus path>`.** Met (diff lines 423-425).
10. **Ruling: step 7 prints a count, and its control repository runs with the variables unset.** Met (diff lines 24-28, 60-74, 82-89).

**Beyond the spec, and whether each is disclosed**

- Doc commit 10a42d6 (ROADMAP.md and spec status/rulings). Required by the P2 ruling, so not a departure. The report says "unedited" is not shown by a primary. For what it is worth, the diff's hunks do add up to 80 insertions and 7 deletions.
- Path-name scanning with `FAIL paths ... (path not printed)`, and `<private path>`. Disclosed.
- `GIT_NO_REPLACE_OBJECTS=1`, the shallow-repository error and the `info/grafts` error. Disclosed.
- The `--range` dash guard. Disclosed.
- Raw-header identity parsing. Disclosed.
- ERE validity check on each pattern. Described in item 3, not listed as a departure.
- The patterns file skips blank and `#` lines (diff line 279). **Not disclosed.**
- The stderr summary line and the `ERROR privacy:` line format. Described in item 3, not listed as departures.
- Blob result caching by sha. Described.
- The installer refuses when `core.hooksPath` is set. Described in item 4, **not flagged** as beyond the spec.
- The hook blocks non-commit refs. Disclosed.
- The step 7 control changed to the `-c` form, and the `GITC` plumbing. Described.

## Q2

**gate.log**
- Full run, all eight steps: lines 95-102, `PASS host` through `PASS privacy`.
- Line 104: `exit: 0 (expect 0)`
- Line 105: `stdout is exactly the eight PASS lines in order: yes`
- Line 1 names the commit and HEAD.

**privacy.log**
- Line 116: `commits on main: 43`
- Line 117: `root commit(s): af5a2c6b279d3a9f68a0ff4dcfc1e4c84ec74d5c`
- Line 119: the rewritten P1 commit, first on main.
- Line 128: `PASS privacy`
- Line 130: `privacy: 43 commit(s), 86 blob(s) scanned, 0 finding(s)`
- Line 131: `exit: 0 (expect 0)`
- Line 133: `commits scanned equal commits on main: yes`

**hook.log**
- `ls -l`, line 122: `-rwxr-xr-x 1 dev dev 1161 Sep 14 19:48 .git/hooks/pre-push`
- Contents: lines 124-156.
- `git config --local --get`: lines 162-165.

**negative.log setup**
- Line 6: `scratch denylist: <scratch>/denylist, mode 600, 1 entry (the sentinel, 14 bytes, never printed)`
- Every case runs in a scratch clone (line 3).
- Counts only: every `captured output:` line shows `lines with the sentinel=0, lines with the token=0`.

**negative.log cases (a) to (k)**
- **(a)** Line 490: `FAIL identity: commit 34fea73645a9da0ce67d530297a98a9148d75972 author email differs from scripts/privacy-identity.txt`; line 491: `exit: 1`.
- **(b)** Line 502: `FAIL message: commit 4ebf91f0e6575d9af0d5f4c11b99a1f0f8d5d295 has a Claude-Session line`; `exit: 1`.
- **(c)**
  - Line 511: `first commit 1b4121cffaa35f199461fc8a890155857464c98c; second commit d55fa6091766cec8ffd5ec4321f99bd7e94632ee`
  - Line 514: `FAIL denylist: commit 1b4121cffaa35f199461fc8a890155857464c98c path c.txt denylist entry 1 line 1`
  - Line 516: sentinel=0.
  - Line 519: `the second commit: expect absent; count=0`
- **(d)** Line 528: `FAIL patterns: commit 84b5ca85c327a5524a2f5e2e5fae1d0724f93437 path d.txt pattern 1 line 2`; line 530: token=0. The token is built from pieces in the script (script line 38).
- **(e)** Each case exits 2:
  - Line 539: `ERROR privacy: the private denylist is missing`
  - Line 548: `ERROR privacy: the private denylist must be mode 600`
  - Line 557: `ERROR privacy: the private denylist holds no entries`
- **(f)** Line 566: `scripts/privacy-patterns.txt bytes: 0`; line 568: `ERROR privacy: self-test failed: a planted hit was not detected`; `exit: 2`.
- **(g)**
  - Before: lines 583-584, `refs: 0`.
  - g1, the clean push succeeds: lines 587-592 (`PASS privacy`, `* [new branch]`, `exit: 0`) and 598 (`f1ac533...	refs/heads/clean`).
  - g2, the branch from (c) is rejected: line 606 FAIL denylist, line 607 `error: failed to push some refs`, line 608 `exit: 1`, line 613 the ref is unchanged, line 615 `remote refs unchanged since g1: yes`.
  - g3 was also rejected (lines 618-630).
- **(h)**
  - Lines 659-660: `h.txt: binary: set`, `h.txt: diff: unset`
  - Line 663: `FAIL patterns: commit ed7200383dd00309cb5ff192810a0cad609c9484 path h.txt pattern 1 line 1`
- **(i)**
  - Lines 672-673: loose object `yes`, then `no`, and `git cat-file -e exit: 1`.
  - Line 675: `ERROR privacy: git failed: cat-file blob 1b98ea0a7c0116496c00f5c1b7f7b8bfa3b309ae`
  - Line 676: `exit: 2`
- **(j)**
  - Line 684: index sha256 `53b0da30...`, entries 62.
  - Line 687: `PASS privacy`.
  - Line 693: the same sha256 afterwards.
  - Line 694: `scratch index unchanged: as-expected`
- **(k)**
  - Line 706: `FAIL hygiene: 1 tracked path(s) containing emend-testing (paths not printed)`
  - Line 707: `exit: 1`
  - Line 712: `lines with name.txt in the captured output: 0 (expect 0)`
  - The path was staged, not committed. `git ls-files` counts a staged path as tracked, so this satisfies "tracked".

**scan.log**
- The script is printed.
- Controls, with the same forms: line 254, `controls: all as expected`.
- Control bytes in the other four logs are counted by `f_ctrl`, with controls at lines 209-210.
- Lines 339, 360, 395: `files in diff: 7; files scanned: 7; files read: 7`; `messages scanned: 2; commits in range: 2`; `logs scanned: 4 (expected 4)`.
- Lines 397-400: `total hits: 0`, `grep or read errors: 0`, `RESULT: PASS`.
- scan.log does not scan itself (accepted since P1).

**Acceptance**
- All eight PASS: gate.log lines 95-102.
- privacy.log passes over the full history: lines 130-133.
- Every negative case as expected: negative.log line 788, `all negative cases as expected: yes`.
- Hook installed and identity set: hook.log lines 117-118, 158-159, 163, 165-166.

Nothing required is missing.

## Q3

I checked the draft's numbers and quoted strings one by one. The following all appear in a primary as quoted:
- every commit hash, title, parent and file status (scan.log lines 257-272);
- the check.sh and privacy.sh sha256 values (gate.log lines 83, 85);
- all gate.log, privacy.log and hook.log strings;
- every negative.log string and hash prefix for cases 0, (a) to (q), k2, g1 to g5 and the closing lines;
- every scan.log figure: `mode 600`, `denylist entries: 6`, `denylist entries holding a non-ASCII byte: 0`, 7/7/7, 2/2, 4, `total hits: 0`, `RESULT: PASS`;
- the log hashes for G, P, H and N (scan.log lines 363, 371, 379, 387).

**Claims that do not appear in a primary, or appear differently**
1. **The Logs section heading says "NOT in a primary".** That is true only for scan.log's own hash (`a418f607...`). The G, P, H and N hashes do appear in scan.log. This is an understatement, not an overstatement.
2. **"Before anything else it sets GIT_NO_REPLACE_OBJECTS=1."** Overstated. Argument parsing, `git rev-parse --show-toplevel` and `cd` come first (diff lines 218-244).
3. **"Any git command that exits non-zero or writes to stderr ... makes the script exit 2."** Overstated.
   - It holds only for commands run through `run_git`.
   - A failing `git rev-parse --git-path info/grafts` (diff line 247) gives an empty string, and the grafts test then passes silently.
   - A non-zero `git config --local --get` is treated as a finding, not an error.
4. **"FAIL privacy: ... with the output tail on stderr".** The body of `fail()` is not in the diff, so "tail" cannot be verified.
5. **"check.sh does too"** (exports `RUSTUP_AUTO_INSTALL=0`). Not in the diff. gate.log and negative.log scripts do export it.
6. **"P5b verify.log v5"** (attributes behaviour of `git grep -I`). Not an input to this check.
7. **"Step 8 read the real denylist: mode 600, entries 6".** gate.log lines 5-6 show the override unset and the file present with those values. That step 8 read it is an inference, though a fair one.
8. **The g3 quote `--range f1ac533...d55fa609...`.** It drops the `..` separator; the log (line 618) has `f1ac533...(full)..d55fa609...`. This is cosmetic.

**Claims the report itself marks as NOT in a primary or session output only** (I could not verify them, as expected):
- record commit `0139a7ec6c83df9ae9e20dd34e10b4d7044e41d7`;
- the doc files unedited before `git add`, and the pre-commit scan with 0 hits. The shortstat `2 files changed, 80 insertions(+), 7 deletions(-)` is consistent with the diff's hunks, but "unedited" and "the stat planning gave" are not verifiable;
- Other notes 1, 2 and 3 (the dry run, the privacy.sh rewrite, concurrent runs);
- scan.log's sha256;
- `RUSTUP_AUTO_INSTALL=0` on every ad hoc command, and no installs;
- the maintainer's GitHub email settings.

Apart from items 2 and 3, the report does not overstate anything. Its Findings 1 to 7 match the code in the diff.

## Defects

Anything that could let private data reach GitHub, or let a push through, in the order I would rank them:

1. **The step 7 office-file FAIL line still prints paths, and it runs before the corpus check.** (check.sh, unchanged context line: `office file(s) tracked outside tests/fixtures/: $(quoted "$OUT/office.z")`.)
   - A force-added workbook under an emend-testing directory, the most likely corpus accident, prints its full path, file name included.
   - The count-only corpus line never runs because the office check fails first.
   - A captured log with that line could be committed, and privacy.sh would not catch the name unless it is on the denylist.
   - The P5b ruling's letter (the corpus line prints a count) is met, but the risk behind it is not closed. The report does not mention this.
2. **`EMEND_PRIVATE_DENYLIST` is honoured everywhere, silently.** privacy.sh, check.sh step 8 and the hook all accept it, although the spec says "for negative tests only". A variable left exported in a shell (for example after a negative run) makes the P7 pre-push hook check against a one-sentinel list and print `PASS privacy`. Nothing prints that an override is active, and neither the hook nor step 8 unsets it. Not disclosed.
3. **Nothing verifies at push time that the hook will run.** Pushes skip the hook when:
   - `core.hooksPath` is set after installation, in any config file;
   - `.git/hooks/pre-push` is removed or made non-executable;
   - `--no-verify` is used.

   `install-hooks.sh` checks `core.hooksPath` only when it runs, and neither privacy.sh nor check.sh checks the hook. P7's `PASS privacy` requirement in push.log only detects the problem after the push. The report's Finding 3 covers `--no-verify` only.
4. **`install-hooks.sh` writes to `$(git rev-parse --git-dir)/hooks`.** In a linked worktree that is the per-worktree directory, where git does not look for hooks (hooks come from the common directory). The installer prints `wrote ...` and exits 0, but the hook is inert. `git rev-parse --git-path hooks` would resolve correctly. This did not affect this phase: hook.log shows the main `.git/hooks`.
5. **The path-name scan works on the git-quoted form (`core.quotepath=true`).** A denylist word directly next to a non-ASCII byte in a path becomes adjacent to an octal escape ending in a digit, which is a word character, so `-w` fails. The raw bytes would have matched. The path is then not flagged, and a content finding in that file prints it. This is low severity, but it is a distinct miss from the report's Finding 5.
6. **`scan_file` has no consistency guard.** If the combined `screen` call hits but the per-pattern or per-entry `hit_lines` runs produce no lines, nothing is emitted and the commit passes. With a correct grep this cannot happen, but an error ("screen hit, zero result lines") would make it fail closed.
7. **The in-script positive controls are incomplete** (see Q1). The `<private path>` substitution is never exercised by the self-test or by any negative case. A regression in the index and line alignment would print a private path unnoticed.
8. **The hook's `git cat-file -t` does not set `GIT_NO_REPLACE_OBJECTS`.** A forced replace ref mapping a tag object to a commit would pass the non-commit block. privacy.sh would then scan the peeled commits, not the tag message. This needs deliberate misconfiguration and is very low severity.
9. **A missing `HOME` under `set -u` makes privacy.sh exit 1.** check.sh step 8 would call that "reported findings". It fails closed; only the classification is wrong.

## Verdict

The diff delivers every P6 deliverable and every item from the P5b rulings. The one substantive departure, content read with `cat-file` and `grep -a` instead of `git grep -I`, is disclosed and required by the ruling. The additions beyond the spec are almost all disclosed. The exceptions are that the patterns file skips comment and blank lines, and the installer's `core.hooksPath` refusal is not flagged as beyond the spec. The logs contain every required line for gate.log, privacy.log, hook.log, negative.log cases (a) to (k) and scan.log, and meet all four acceptance items. The report's numbers and quoted strings match the primaries, except two mild overstatements (the `GIT_NO_REPLACE_OBJECTS` ordering and "any git command") and a few claims with no primary (the `fail()` output tail, `RUSTUP_AUTO_INSTALL` in check.sh, the P5b verify.log reference). Planning should rule on defects 1 to 3 before P7, because each could let private data reach GitHub or let a push through unchecked:
- the step 7 office-file line prints corpus file names;
- a stray `EMEND_PRIVATE_DENYLIST` silently weakens the gate;
- nothing confirms at push time that the hook will run.

Defects 4 to 9 are lower-severity gaps that should be recorded.
