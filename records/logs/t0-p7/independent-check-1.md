# T0 P7 independent check 1

- Checked commit: af4b355e028e50f07e42c6cbc52bb451765868d1 (planning doc
  updates, the last P7 commit before its logs; P7 has no code commit)
- HEAD when the check ran: 9c635796374194ceb58180b1f0bee840dc26c4e6
  (push.log record commit, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  four inputs below
- Inputs, with sha256:
  - specs/0001-bootstrap.md
    17505f473a278ec4ffa5acc0f95533d1243c5c778ab49ad5c50e3ea5d58c3cc2
  - the phase diff, `git diff cf5240b..af4b355`, saved as <scratch>/p7-diff.txt
    d4f301fd6432733fb073b737e4716568cdbf7bfd8947f4d7cfe9740bd3261bf5
  - draft report 1, <scratch>/p7-report-draft.md
    af1ecae83e13ea3cc5f5c6e2a19260741ad8144e6a8c6c81ddcb2502f9024848
  - records/logs/t0-p7/push.log
    d014e3eb4c21e0c4d32fb2ef01b4eb8b51680e61574e42d7286c40a07262c4f5

---
I have a CLAUDE.md from the harness in my context (the project CLAUDE.md, plus the user's memory index and attribution notes). I did not use any of it as evidence. The only evidence below is the spec (S), the diff (D), the draft report (R) and push.log (L). My only other checks were byte counts, a sha256, a line count and string searches over those four files.

## 1. Does the diff do what the spec says and nothing else?

**Yes, with one addition the spec does not mention.**

- **Range:** D is cf5240b..af4b355. That matches the rule in S lines 33-35 (previous phase's last record commit to this phase's last phase commit).
- **Files:** D touches only ROADMAP.md and specs/0001-bootstrap.md.
- **Size:** D has 121 non-empty and 6 empty added lines (127 insertions) and 6 deletions. That matches R line 12.
- **spec changes:** each one is planning text that the final S contains word for word:
  - the status line
  - "P7 and P6c" in the phase order
  - the P6b rulings section
  - the new P7 precondition about the denylist review
  - the P6c section
  - the "Done" line
- **ROADMAP changes:** P6b is marked DONE, a P6c row is added, and a "Gate self-test script" backlog item is added (D lines 20-22).
  - The backlog item is not named or required anywhere in S.
  - Its claim that the maintainer approved it on 2026-09-15 has no support in any primary.
  - It is plausibly part of "planning doc updates", but planning should confirm it belongs to this commit.
- **What D cannot show:** it has no commit messages, so the trailer claims can't be checked from it. It also can't show that the text is planning's unedited wording.
- **Bytes:** D has no non-ASCII bytes, no U+2014 and no control bytes.

## 2. Does push.log have every required line, and does it follow the log rules?

**Required content: all present.**

| Requirement | Where in L |
|---|---|
| Step 1: gh visibility command, PRIVATE output, exit 0 | 164-169 |
| Step 1: ls-remote of the URL, exit 0, no refs | 170-174 |
| Step 2: remote add, exit 0, `remote -v` | 177-183 |
| Step 3: denylist unset | 186 |
| Step 3: hooksPath exit 1, 0 bytes | 187-190 |
| Step 3: hook executable, sha256 equal to the installer heredoc | 191-196 |
| Step 3: check.sh, eight PASS lines in order, stderr empty, exit 0, 0 control bytes, clean status | 197-214 |
| Step 4: `git push -u origin main` (no --no-verify), hook `PASS privacy`, exit 0 | 222-231 (PASS line at 225) |
| Step 5: ls-remote hash equals rev-parse | 237-244, again at 453-462 |
| Repository still private | 245-247, again at 463-469 |
| No authentication output | none in 223-230; header comment at 12-13 |

Two notes on this content:
- Step 3 compares the hook with an awk extraction of the installer's heredoc, not the installer's real output. The P6b ruling accepts the heredoc as the reference (S line 453), so this is minor.
- Part 2 ends with `part 2 checks all as expected: no` (248). Part 3 (250-472) is the fix-named re-run that S lines 54-56 require.

**Log rules: met, with minor points.**

Met:
- Line 1 names the commit and the HEAD. The part 2 and part 3 headers (216, 250) also name HEAD.
- Control bytes: 0. There are 4 tabs (lines 182, 183, 239, 455) and 0 CR.
- No non-ASCII bytes and no U+2014.
- No hits for the machine's hostname.
- No session ids or links, no temp session directory paths (the script prints only `mktemp -d` and `<scratch file>`), no token-like strings, no email addresses.
- The only 40+ hex strings are af4b355 and the hook sha256.

Minor points:
- **Local user and group names (line 192):** the `ls -l` line shows the local user and group names and a file time. That is not a hostname, but it is local account information.
- **Script label (line 255):** the heading says that script wrote parts 2 and 3. Part 2 actually ran from an earlier version without the verify block; the only difference is that block (log lines 405-444).
- **Scripts not under /usr/bin/grep rules:** the push script is not a scan script, so the scan-script rules do not strictly apply. It does call /usr/bin/grep anyway.

## 3. Does every number and quoted string in the report appear in a primary?

**Found in a primary:**
- Step 1 to 5 quotes and numbers in R lines 33-54, all in L: `{"visibility":"PRIVATE"}`, exit codes, `refs: 0`, `remotes before: 0`, `EMEND_PRIVATE_DENYLIST set: no`, `-rwxr-xr-x`, 3c5d2d79, the eight PASS lines, `stderr bytes: 0`, the hook lines, `52 commit(s), 108 blob(s)`, `[new branch]`, `still PRIVATE: yes`, the part 2 "no", and the part 3 lines.
- 52 commits: L 161 and 228.
- 51: S line 469.
- 7-entry denylist: S lines 479 and 994.
- 11 patterns: S line 1048.
- 472 lines: matches L's actual line count.
- 2 files, 127 insertions, 6 deletions: match D.
- The push.log sha256: matches L. R marks it as session output anyway.
- The full af4b355 hash: in L.

**Not in any primary:**

| Item in R | Marked as session output? |
|---|---|
| Full cf5240b hash (line 11) | Only loosely, through line 12. S and D have the short form only. |
| Record commit 9c635796... and "ahead by 1" (line 19) | Yes, though the marker is placed after "ahead by 1". |
| Trailer-only claim for both messages (line 21) | Yes |
| Maintainer's authorisation quote (lines 25-26) | Yes |
| Email settings confirmation relayed by planning (lines 27-28) | No |
| Planning's word, "your message of 2026-09-15" (line 29) | No |
| Re-run counts "five times" and "three times" (lines 60-62) | Yes |
| "plus the doc changes before commit" (line 65) | No. S line 469 says planning scanned exactly what P7 pushes, 51 commits. |
| Pre-commit scan results in Surprise 3: 0 hits, the "two P6c forms", controls, hostname, token-like strings | Yes. No scan.log exists, so there is no primary. |
| Credential helper used, no config changed (lines 73-75) | No |

**Overstatements and inaccuracies:**
- **Dangling "Unedited" (line 17):** R says `"Unedited" is session output`, but no sentence in R claims the commit is unedited.
- **"one second evaluation failed" (line 59):** L supports only "at least one". "Each part 2 check ran its command twice" is also too broad: the exit-code checks ran once. Only the three yes/no checks at L 131-132, 146-147 and 150-151 ran twice.
- **"Hook lines" (lines 44-47):** that list includes `* [new branch] main -> main`, which is git's output, not the hook's.
- **Cause "not identified" (line 62):** the log script leaves an obvious, untested candidate. Under `set -o pipefail`, `cat | grep -q` and `gh | grep -q` can fail when grep -q exits early and the writer gets SIGPIPE. A second network call to gh can also fail on its own. Re-running a few times and getting 0 each time does not rule out a race.
- **Part 3 coverage:** part 3 re-checks step 5 only. The failing second evaluation could have been the step 4 `PASS privacy` grep (L 132), which part 3 does not re-evaluate. Acceptance still holds, because the literal line is at L 225, but R does not point out this gap.
- **Surprise 4 (line 72):** R names the tab only in the `git remote -v` lines. L also has tabs at 239 and 455.

**Preconditions and steps only partly met, or not shown in a primary:**
- **Email settings:** S line 996 needs the maintainer's confirmation. R says it came only through planning's relay and the maintainer did not repeat it in the implementing session. That is partly met and needs a planning ruling.
- **Authorisation and planning's word:** neither is recorded in L or any other primary. R alone asserts them.
- **Commit count:** S line 469 says P7 pushes 51 commits. L shows 52. The doc commit af4b355 was pushed without being covered by planning's pre-push scan, though the hook did scan all 52. R reports this (Surprise 2) but calls it covered by "doc changes before commit", which no primary supports.
- **No scan.log:** S line 73 says each phase's scan.log counts control bytes with a positive control. P7's log list names only push.log, and R records the departure. The log scan claims have no committed primary. My own count found 0 control bytes, so the claim holds.
- **Record commit stays local:** R says so, but no primary shows it.

## Defects, most severe first

1. **Email settings precondition (S line 996):** met only through a relay from planning, not by the maintainer in the implementing session. It needs a planning ruling.
2. **Part 2 false "no":** the cause is unidentified, and a SIGPIPE-under-pipefail or second-gh-call hypothesis is untested. Part 3 does not re-evaluate the step 4 `PASS privacy` check, so the failing evaluation may be outside what part 3 re-ran. Acceptance still rests on L 225, 239, 243 and 247.
3. **52 vs 51 commits:** the pushed history differs from the one S line 469 says planning scanned (af4b355 added). R's "plus the doc changes before commit" has no support in a primary.
4. **Unrecorded claims:** authorisation, planning's word, the credential helper, and "no config changed" appear in no primary. The last two are not marked as session output.
5. **No scan.log:** the log scan is session output only. This departs from the every-phase rule in S line 73 and needs a planning ruling.
6. **ROADMAP backlog item (D lines 20-22):** added beyond anything S names, with an approval claim that has no primary.
7. **Wording problems in R:** the dangling "Unedited" (line 17), "one" rather than "at least one" failed evaluation, "each check" ran twice, `[new branch]` called a hook line, and tabs named only in the `remote -v` lines.
8. **Wording problems in L:** the line 255 heading implies part 2 ran from the script that includes the verify mode, and line 192 records local user and group names.
