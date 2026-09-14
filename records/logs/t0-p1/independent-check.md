# T0 P1 independent check

## Q1. Does the diff do what the P1 spec says and nothing else?

YES for the file set. PARTLY for "unedited": the diff cannot show that. I also disagree with the report's statement that nothing looked wrong.

File set:
- The diff has exactly four file headers:
  - `diff --git a/CLAUDE.md b/CLAUDE.md` (p1-diff.txt line 1)
  - `diff --git a/ROADMAP.md b/ROADMAP.md` (line 139)
  - `diff --git a/specs/0000-step-zero-plan.md ...` (line 252)
  - `diff --git a/specs/0001-bootstrap.md ...` (line 491)
- All four are `new file mode 100644`.
- No file outside {CLAUDE.md, ROADMAP.md, specs/*} appears. There is no records/, no .gitignore, no Cargo or script file.
- The hunk headers give 132, 107, 233 and 180 lines (lines 6, 144, 257, 496). These sum to 652. That matches the `--stat` block in scan.log lines 32-36 (`4 files changed, 652 insertions(+)`).
- The acceptance wording ("exactly CLAUDE.md, ROADMAP.md and the files under specs/") is met, assuming specs/ held only these two files in the working tree. The diff cannot show whether specs/ had other files that were left unstaged, for example untracked or ignored ones.

What I can verify about "unedited":
- specs/0001-bootstrap.md in the diff (lines 497-676) matches, line for line, the copy of /home/dev/emend/specs/0001-bootstrap.md I read. That copy is the current working tree after the commit, not the tree before it. It only shows the working tree and the commit agree now for that one file.

What I cannot verify:
- For CLAUDE.md, ROADMAP.md and specs/0000-step-zero-plan.md, I had no copy from before the commit. Nothing shows they were not edited before `git add`.
- The report's pre-add sha256 comparison is not recorded in any primary (see Q3). Unedited is asserted, not shown.

Things that "look wrong". The spec (line 45-46) says to stop and report these. The report (line 37) says there were none. I disagree:
1. Two lines of CLAUDE.md end in a trailing space: diff line 131 (`...to memory/progress.md `) and line 134 (`...For large `).
2. In the same section (diff lines 131-138), the wrapped list lines are not indented. Every other section indents them by two spaces. The last section reads as added by a different hand.
3. CLAUDE.md line 111-112 (diff) gives the checker "the gate and scan logs". The spec's "How this task runs" item 3 gives it "the logs in records/logs/t0-pN/". Later phases also have build.log, binary.log, ignore.log and negative.log, so the two lists do not match.
4. CLAUDE.md line 131 and line 136 (diff) point to `memory/progress.md` and `memory/gotchas.md` as relative paths. Nothing says whether that `memory/` directory is inside the repository.

None of these is a reason to edit the files in P1, since the spec forbids edits. Items 1 and 2 are exactly the "report it" case, though. The claim that nothing looked wrong is too strong.

## Q2. Do the logs contain every line the P1 spec requires?

PARTLY. Every required section is there and every count is zero. Several parts are weak.

Present:
- The first line names the commit: `commit: 5f15d55641d77b2087ec9686700829d5a7897181` (scan.log line 1). This meets "How this task runs" item 2.
- The U+2014 scan: its command (line 18) and `U+2014 hits: 0` (line 19).
- The non-ASCII scan: its command (line 22) and `non-ASCII hits: 0` (line 23).
- The commit message ASCII check: its command (line 26) and `commit message non-ASCII hits: 0` (line 27).
- Also present, though not required: the list of scanned files (lines 6-9), sha256 values (lines 12-15) and a `--stat` block (lines 29-36).

Missing or weak:
1. The commands are templates, not what actually ran. `git show <hash>:<path> | grep ...` has a `<path>` placeholder. The log does not show the loop, the output for each file, or grep's exit status for each file.
   - grep exits 1 for no match and 2 for an error. A count of 0 alone does not tell those apart.
   - The log also does not say how the "hits" number was computed.
2. The U+2014 command (line 18) is `grep -nP '\x{2014}'` with no locale recorded. In a C or POSIX locale, GNU grep -P rejects `\x{2014}` as a code point too large and exits 2 with no matches. That would still be written as "hits: 0". Nothing in the log rules this out.
   - The non-ASCII scan (line 22) runs under LC_ALL=C and matches bytes above 0x7F. That covers the UTF-8 bytes of U+2014 (E2 80 94), so the result is protected indirectly, if that scan really ran on all four files.
   - The U+2014 scan alone does not prove anything.
3. No positive control. Nothing shows either scan finding a known hit. P1 does not require a negative test, so this is a weakness, not a gap.
4. The heading on line 29 says `git show --stat 5f15d55...`, but lines 30-36 are not default `git show --stat` output. There is no "commit"/"Author:"/"Date:" header, and the subject sits on the hash line. Some `--format` or `--oneline`-style option was used and not recorded, so the heading does not match the output.
5. The sha256 lines (12-15) are hashes of committed content only. The matching pre-add working-tree hashes that the report relies on are not in the log.
6. Out of P1's stated scope, but worth noting:
   - Nothing checks that the record commit's message is ASCII.
   - scan.log itself is not scanned.
   - The spec covers only the phase commit, so this is not a failure against it.

## Q3. Does every number and quoted string in the draft report appear in a primary?

PARTLY. The phase commit data and the scan results match primaries. The record commit, the empty-tree hash, "root commit", the pre-add hash comparison, the independent-check.md commit and "no remote" appear in no primary I was given.

Item by item (R = p1-report-draft.md, D = p1-diff.txt, S = scan.log):

1. `5f15d55641d77b2087ec9686700829d5a7897181` (R line 7): in S lines 1, 3, 18, 22, 26, 29, 30. Not in D; the diff file contains no commit hashes.
2. `"T0 P1: commit CLAUDE.md, ROADMAP.md and specs as they stand"` (R line 8): in S line 30. D line 174 (ROADMAP) has only a near-match without the "T0 P1:" prefix.
3. `(root commit; the repository had no commits before it)` (R line 9): NOT in any primary.
   - A diff from the empty tree does not prove a root commit. It would look the same for a non-root commit if someone chose to diff against the empty tree.
   - The spec gives the diff as <previous record commit>..<phase commit>, so root status matters. It is unverified.
4. `a1246af5019035782e02cb18e2971291338f5696` (R line 10): NOT in any primary. grep for `a1246af` gives 0 hits in D and S.
5. `"T0 P1 record: scan.log at 5f15d55"` (R line 11): NOT in any primary (0 hits in D and S).
6. File names `CLAUDE.md`, `ROADMAP.md`, `specs/0000-step-zero-plan.md`, `specs/0001-bootstrap.md` (R lines 15-16): in D lines 1, 139, 252, 491 and S lines 6-9 and 32-35.
7. "unedited from the working tree" (R line 16): NOT verifiable from primaries (see Q1).
8. "exactly those 4 files" and "652 insertions" (R lines 17-18): in S line 36 (`4 files changed, 652 insertions(+)`). Consistent with D hunk headers (132+107+233+180 = 652).
9. "The sha256 of each committed blob (in scan.log) matches the sha256 of the working-tree file taken before `git add`" (R lines 19-20):
   - The committed hashes are in S lines 12-15.
   - The pre-add working-tree hashes and the comparison appear in NO primary. This key "unedited" claim cannot be verified.
   - Minor wording point: S line 11 calls these "sha256 of committed content". They are sha256 of file content, not of git blob objects. The report's "sha256 of each committed blob" is loose but not wrong in substance.
10. `records/logs/t0-p1/scan.log` (R line 24): the path of the log file I was given. The log does not name its own path. It matches the spec pattern `records/logs/t0-pN/`.
11. First line quote `` `commit: 5f15d55641d77b2087ec9686700829d5a7897181` `` (R line 24): exactly S line 1.
12. `U+2014 hits: 0` (R line 25): S line 19.
13. `non-ASCII hits: 0` (R line 25): S line 23.
14. `commit message non-ASCII hits: 0` (R lines 25-26, wrapped): S line 27.
15. `git show <commit>:<path>` (R line 26): a paraphrase of S line 3 (`git show 5f15d55...:<path>`). Acceptable. "not the working tree" is also in S line 3.
16. `records/logs/t0-p1/independent-check.md` "added in a second record commit" (R lines 28-29):
    - This describes something that had not happened when the draft was written.
    - It is in no primary. The spec (line 27-29) allows either commit, so it is not a violation. It is still an unverified claim at draft time.
17. `4b825dc642cb6eb9a060e54bf8d69288fbee4904` (R line 34): NOT in any primary (0 hits in D and S). It is the well-known git empty-tree id, and the task handed to me states it. It is not in the diff file or the log.
18. `5f15d55` (R line 34): as a prefix in S lines 1 and 30 and elsewhere.
19. `git diff <previous record commit>..<phase commit>` (R line 36): a paraphrase of spec lines 24-25 (`git diff <previous phase's record commit>..<this phase's commit>`). It is in the spec, not in D or S.
20. "Nothing in the four files looked wrong" (R line 37): a judgment. I dispute it (see Q1: trailing spaces at D lines 131 and 134, unindented list lines at D lines 131-138, and the checker input list at D line 112 not matching the spec).
21. "ROADMAP.md still says P1 READY" (R line 38): in D line 174 (`| P1 | ... | READY |`).
22. "No remote exists and nothing was pushed" (R line 39): NOT in any primary. Spec line 180 says "there is no remote", but that is a statement in the spec, not evidence from the phase.

What the report needs before it goes to planning:
- Evidence for items 3, 4, 5, 9 and 22, for example a log section with `git rev-list --parents`, the record commit's `git log -1`, `git remote -v`, and the sha256 values taken before `git add`.
- Its "nothing looked wrong" statement revised to mention the CLAUDE.md whitespace and formatting observations.
