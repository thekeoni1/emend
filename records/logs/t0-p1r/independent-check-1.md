Checked phase: T0 P1R (specs/0001-bootstrap.md)
Checked commit: d495f0a477821d429984a4a86634adcaf13d9463 (P1R doc commit)
HEAD when the check ran: d424f81318cafe25b49b555883d9fdf836d546bd
Checker: a fresh general-purpose subagent, not a fork
Inputs (sha256):
- specs/0001-bootstrap.md: 0ff6147fb3c66031f634b3a523fa0aacafbc16ceb104b4b7b219c1df1e44d11b
- <scratch>/p1r-diff.txt (git diff d3754dcd49bedd6f2b37da5778a4db738511fe69..d495f0a477821d429984a4a86634adcaf13d9463): 2623815a7506e073306b81a2cca1248c4c5a0e10b1f7a1c1d3212cd78f9c6def
- records/logs/t0-p1r/rewrite.log: c0799ed84e0341d6f70b9ed86f465ae83c386e2cbb9abda54baa341ec2c4ec83
- records/logs/t0-p1r/scan.log: 62d56f63ffd0b567bbb52ed2d04454caf9f4a497b443949619e9ca82cb145e33
- <scratch>/p1r-report-draft.md: 3b728e7145611a51c16b188b3e798f571f1f7b1ab42bf4dacbcd2462b4bd9000
---
# T0 P1R independent check 1

Context note: the harness put a copy of CLAUDE.md into my context. It also added a git status snapshot with recent commit subjects, and a commit-attribution reminder. I did not use any of these as evidence. I read only the five named files and ran no commands.

## Q1. Does the diff, with rewrite.log for step 1, do what the P1R spec says and nothing else?

PARTLY. What the primaries show matches the spec, and I found no extra action. Several parts of the spec cannot be shown by these primaries at all.

Files in the diff:
- The diff touches exactly four files: CLAUDE.md (diff line 1), ROADMAP.md (31), specs/0000-step-zero-plan.md (61) and specs/0001-bootstrap.md (78).
- No file outside CLAUDE.md, ROADMAP.md and specs/ appears. No records/ file appears, which is expected because the range ends at the doc commit.
- The content fits the spec's starting state (spec 128-130):
  - CLAUDE.md gains the commit-message rule (diff 9-10) and the "Elevation and system changes" section (diff 18-27).
  - ROADMAP.md and both specs change.
- The specs/0001 post-image matches the spec file I was given, in the hunks I compared (status line, P1R section, rules sections).

Step 1, as shown by rewrite.log:
- Backup ref: created at the old head before anything else, create-only (R 173-174).
- Starting state: exactly five commits were replayed, oldest first (script line 53 uses `rev-list --reverse`; R 227 "commits replayed: 5"). Their hashes match the spec's five short hashes (R 178, 188, 198, 208, 218).
- Replay method: `git commit-tree` with all six GIT_AUTHOR_* and GIT_COMMITTER_* variables set from the parsed original header (script 71-77).
- Message filter: awk `/^Claude-Session:/`, which is exactly the spec's pattern (script 67).
- Pair checks, each re-read from the new object rather than assumed:
  - tree, author and committer fields;
  - the parent chain;
  - that the message bytes equal the filtered message;
  - a line-count delta of 1;
  - no extra header lines, so no gpgsig or encoding header was silently dropped (script 80-113; R 177-228).
- Ref update: `git update-ref refs/heads/main <new> <old>` with the old-value guard (R 231).
- Parent chain: R 237-241 is linear, af5a2c6 -> f41b87d -> 8b28d47 -> 582d8b6 -> d3754dc.
- Index and working tree: no command in the printed script writes to either. `git status --porcelain` is identical before and after (R 257).
- No filter-branch: `git filter-branch` does not appear in the printed script.
- Backup deleted: `update-ref -d` with the old-value guard, exit 0, then an empty for-each-ref (R 262-267).

What the primaries cannot show:
1. The backup deletion is not part of the printed script (script ends at R 157). R 262-267 are appended text with no script behind them. Nothing shows how "exit=0" and "(end of list)" were produced, or what HEAD was at that moment. The report admits this (note 2).
2. Nothing shows that no other command ran outside the two logged scripts, for example a filter-branch, an index write or an edit, before the doc commit. The logs show only what the scripts did.
3. "Unedited" (spec 150) is not shown. No primary records working-tree hashes of the four files before the commit to compare with the committed blobs. The diff index lines (2225aba, 92cea0c, bcc7fc8, f9c372e) have nothing to compare against.
4. The doc commit's parent, message, author and committer are not shown.
   - The diff is a tree-to-tree comparison. It would look the same if several commits sat between d3754dc and d495f0a.
   - S 148-171 lists six commits on main with d495f0a first. That makes a single commit on top of d3754dc very likely, but `rev-list` without `--parents` does not print parents.
5. That `git status` did not rewrite .git/index stat data cannot be shown. This is harmless, but "do not touch the index" is literally unprovable with `git status` in the loop.
6. That the removed line in each message really was the session line, and that no other session link remains in the message body, is not shown. The ^Claude-Session: form checks only lines that start with that text (see Q2).

## Q2. Do the logs contain every line the P1R spec requires?

PARTLY. Every listed item is present, all checks pass, and rewrite.log does not contain the removed lines. There are gaps against the scan rules and the first-line rule, and some checks are weak.

rewrite.log items (spec 154-164):
- Exact script: present (R 5-157), printed by the script itself with `sed "$0"`. The appended backup-deletion section is not covered by any printed script. Weak.
- Per pair, both hashes: present (R 178-179, 188-189, 198-199, 208-209, 218-219).
- Trees equal: present, "tree equal: yes" x5.
- Author and committer name, email and date equal: present, 6 fields x5.
- Message line counts before and after: present, 6/5, 6/5, 7/6, 11/10, 8/7 (R 186, 196, 206, 216, 226).
- Removed-line count, without the lines: present per pair and in total (R 227).
- Removed lines absent: rewrite.log does NOT contain the removed lines themselves.
  - I read it in full. It has no session URL or id. The only "Claude-Session" text is the pattern in code and in labels (R 7-9, 67, 106, 136, 140, 186 and so on).
  - scan.log's f_session scan of rewrite.log (S 174-177) cannot prove this alone. It matches only lines that start with `Claude-Session:`, and every line in R's script section is indented. The evidence is the reading, not the scan.
- `git rev-list --parents main`: present (R 236-241).
- Per-commit ^Claude-Session: count: present (R 242-247), count=0 exit=1 x5.
- `git diff --stat <old> <new>`: present (R 248-249). Emptiness is shown only by the absence of lines before the end marker, backed by `git diff --quiet exit: 0` (R 250).
- `git status --short` showing the doc changes survived: present (R 251-256), plus the before/after comparison (R 257).
- `git for-each-ref refs/backup` after deletion: present (R 266-267), but hand-appended as noted.

scan.log items (spec 165-167):
- Files in the doc commit: present (S 124-145). Blobs are read from the commit (`git show DOC:path`), not from the working tree, which is good.
  - Scope: the scan covers the four changed files only. The doc commit's tree also holds unchanged P1 files, such as records/logs/t0-p1/*. This is a reasonable reading of "files in the doc commit", but a narrower one.
- Every commit message on main: present, six messages (S 148-171).
- Forms: non-ASCII, U+2014 and ^Claude-Session: are all present.
- Positive controls: present (S 115-121), with negative controls and an em-dash-versus-other-non-ASCII control.
- Nothing asserts how many files or messages were scanned.
  - `set -e` is off, and an empty `git rev-list` or `git show --name-only` would loop zero times and still print RESULT: PASS.
  - The actual output shows the loops ran, so this is a script robustness weakness, not a failed check.

Acceptance (spec 169-176):
1. Equal trees, authors and committers: met (R 180-182 and the equivalents, R 228).
2. Exactly one line removed per message: met (R 186 and the equivalents; the script enforces removed==1 and lb-la==1 at 112-113).
3. Zero Claude-Session lines on main: met (R 243-247; S 151, 155, 159, 163, 167, 171). This is the ^-anchored form only.
4. `git diff --stat` empty: met (R 248-250).
5. The doc commit lists exactly the changed files: PARTLY.
   - S 126-129 lists 4 M entries, and R 164-167 shows the same 4 files modified at 22:46:17Z.
   - No status taken after the doc commit shows that no doc change was left uncommitted, and none ties the pre-commit working tree to the commit.
   - The R status comes from before the rewrite, not immediately before the commit.
6. Every scan zero hits, every control passes: met (S 122, 179-181; R 243-247).
7. refs/backup empty at the end: met (R 266-267), with the appended-section caveat.

Rules for every phase:
- /usr/bin/grep from a `#!/usr/bin/env bash` script: met in scan.log (S 6, 14, 111) and in rewrite.log (R 6, 140).
- Byte matching under LC_ALL=C: met.
  - Non-ASCII uses `-P '[^\x00-\x7F]'` (S 22).
  - U+2014 is built from bytes E2 80 94 with printf octal escapes and matched with -F (S 17, 23).
  - `\x{2014}` is not used.
- Exit codes: counts and exits are printed for every grep, exit 2 or more is tallied as an error, and hits are counted only on exit 0 (S 26-31; R 140-142).
- Same-form positive controls: met in scan.log (the controls call the same functions, S 51-64). NOT met in rewrite.log.
  - rewrite.log's per-commit count uses `LC_ALL=C /usr/bin/grep -c '^Claude-Session:' "$T/m"` (R 140), and rewrite.log has no positive control for it.
  - The nearest control is S's f_session, `LC_ALL=C "$G" -c -e '^Claude-Session:'`. That is a different spelling (-e, a variable path), so strictly not "the same command form".
  - The awk count of 1 per old message shows the pattern matched the old messages, but awk is a different tool.
- Exact script printed: met for both scripts. Not met for the backup-deletion commands (R 262-267).
- Log privacy: no hostname, no `uname` output, no Windows path, no /tmp/claude path ($T is never expanded into output, and `check_ctl` prints basename only), no session id or link, no token, and no author name or email.
  - The denylist cannot be checked from these files.
  - LANG=C.UTF-8 (S 112) and the git and grep versions are harmless.
- Commit-message rule: only partly scanned.
  - The scan covers ASCII, U+2014 and ^Claude-Session: in messages.
  - No check shows that messages end with the Co-Authored-By line, or that no other session link exists (for example a URL not at the start of a line).
  - The record commit d424f81's message is scanned by no log (the report admits this).

First-line rule (spec 23-24):
- scan.log line 1 names the doc commit and HEAD (S 1). Met.
- rewrite.log line 1 names the old head and "HEAD at run start" (R 1). Weak on two counts:
  - It names what the rewrite started from, not the result d3754dc. The result appears only at R 232-233 and 260.
  - HEAD moved during the run, and the appended deletion section (R 262) has no HEAD line of its own.
- Ordering depends on self-reported timestamps: rewrite 22:46:17Z, deletion 22:46:55Z, scan 22:47:22Z.
  - scan.log records no sha256 of the rewrite.log it scanned.
  - The scanned rewrite.log is therefore tied to the committed one only by time order, not by hash.

Failed checks: none. Every check in both logs reports the expected value.

## Q3. Does every number and quoted string in the draft report appear in a primary?

PARTLY. The commit hashes, counts and quoted log lines all check out. The sha256 values, the doc commit subject, the "parent d3754dc" citation and several notes are not in a primary, and some of those are not marked.

Checked against a primary:
- Table (report 13-17), all ten hashes: R 178-179, 188-189, 198-199, 208-209, 218-219. All match character for character.
- d495f0a477821d429984a4a86634adcaf13d9463 (report 19): S 1, 124-125, 148.
- "all pair checks: yes" (report 30): R 228.
- "removed ^Claude-Session: lines: 1" (report 32): R 186, 196, 206, 216, 226.
- Line counts 6->5, 6->5, 7->6, 11->10, 8->7 (report 32-33): R 186, 196, 206, 216, 226.
- "total removed lines: 5 (commits replayed: 5)" (report 33): R 227.
- "new message bytes equal old message minus removed lines: yes" (report 34): R 185 and the equivalents.
- "count=0 exit=1 for all five rewritten commits" (report 36-37): R 243-247.
- Six messages on main, `claude-session: ... count=0 exit=1` (report 37-38): S 148-171, quoted with an ellipsis for "form=f_session". Acceptable.
- "(end of git diff --stat output)" (report 40): R 249.
- "git diff --quiet exit: 0" (report 41): R 250 (truncated quote).
- The four M files (report 43-45): S 126-129; D lines 1, 31, 61, 78.
- "controls: all as expected" (report 46): S 122.
- "7 controls" (report 47): S 115-121, seven lines.
- "total hits: 0" (report 47): S 179.
- "grep errors (exit 2 or more): 0" (report 47): S 180.
- "RESULT: PASS" (report 48): S 181.
- "backup ref deletion", "exit=0" and the empty list (report 49-50): R 262, 265, 266-267.
- "status unchanged by the rewrite: yes" and the four files still M (report 53-54): R 252-255, 257.
- First-line claims for R and S (report 61, 64): R 1, S 1.
- Note 2 (report 73-74): consistent with R. The script ends at R 157, and R 258 says the backup was kept.
- Note 3 (report 75-78): 15c120a is at R 218, d3754dc at R 219, "tree equal: yes" at R 220. The D-shows-only-the-doc-commit claim is consistent with D.
- Note 4 (report 79): S 173-177.
- Note 7, pre-rewrite hashes in the specs/0001 status line (report 90-92): D 86-87. "hashes change in P1R": D 215.

In no primary:
- sha256 c0799ed84e0341d6f70b9ed86f465ae83c386e2cbb9abda54baa341ec2c4ec83 (report 60) and sha256 62d56f63ffd0b567bbb52ed2d04454caf9f4a497b443949619e9ca82cb145e33 (report 63).
  - A log cannot contain its own hash. The report does NOT mark these as outside a primary.
- Range "d3754dc..d495f0a" as the definition of D (report 5).
  - The diff file does not state its range; it holds only blob index hashes. Both hashes do appear in R and S.
  - The range is named only by the launching prompt. Unmarked.
- Subject "T0 P1R: commit maintainer and planning doc updates" (report 20).
  - S scans that message but never prints it. Unmarked.
- "parent d3754dc" (report 20-21).
  - The report cites "S first line and 'files in the doc commit'; D". None of those shows a parent. S line 1 names only the doc commit and HEAD, the name-status list shows files, and D is a tree diff.
  - S 148-152 ordering is consistent but prints no parents. The citation overstates what the primaries show.
- d424f81318cafe25b49b555883d9fdf836d546bd, subject "T0 P1R record: rewrite.log and scan.log", "parent d495f0a" (report 22-24).
  - In no primary. Marked "NOT in a primary". Honest.
- Note 1, dry run producing d3754dc (report 69-72): the d3754dc hash appears in R, but the dry run itself does not. Marked NOT a primary. Honest.
- Note 5 (report 80-84): marked "NOT logged". Honest, but it understates one point.
  - The first item, "raw commit headers held only tree/parent/author/committer", is in fact logged for all five old commits: "extra header lines besides tree/parent/author/committer: 0" at R 184, 194, 204, 214, 224.
  - The other items (trailer and template config, hooks, P6-pattern and hostname scan of the P1 trees) are in no primary, as marked.
- Note 6 (report 85-88): `/home/dev` paths "for example the corpus directory in CLAUDE.md and specs/0000".
  - D shows `/home/dev/emend-testing` only in ROADMAP.md context (D 51). No hunk of D shows a /home/dev path in CLAUDE.md or specs/0000.
  - The CLAUDE.md and specs/0000 examples are in no primary. Unmarked.
  - "No P6 pattern covers them" is checkable against the pattern list in D 381-388 and spec 326-333, and it is correct: none of those patterns matches /home/.
- Note 7, "the P1 logs cite pre-rewrite hashes" (report 90-91): the P1 logs are not among the inputs. Unmarked.
- Note 8, two sessions named "emend planning" and a refused message (report 93-95): in no primary. Unmarked.
- Note 9 (report 96-99):
  - "My harness still asks for a Claude-Session line" is in no primary. Unmarked.
  - "the P1R commit messages carry Co-Authored-By only (S, doc commit message scan...)" is NOT supported by S. S 148-151 shows only zero non-ASCII, zero U+2014 and zero ^Claude-Session: lines for d495f0a. It does not show that a Co-Authored-By line exists, that the message ends with it, or that no other session link exists.
  - The record-commit part is marked. The doc-commit part cites S for more than S shows.
- Note 10 (report 100-102): "The old commits remain as local unreachable objects, via the reflog" is in no primary. The parenthetical "(NOT in a primary given to the checker)" follows only the no-remote sentence, so it is unclear whether it covers the reflog claim.
- Report 52-55: parts go beyond what R shows.
  - "the index and working tree were not touched" rests on an unchanged porcelain status plus the content of the printed script. That supports "no index or working-tree content change by the script", not "not touched" in general, and it says nothing about commands run outside the script.
  - "the printed script uses `git commit-tree` and `git update-ref` only" is literally false. The script also runs rev-parse, symbolic-ref, status, for-each-ref, cat-file, rev-list and diff. The intended meaning is presumably that these are the only object and ref writers.
  - "git filter-branch was not used" can be shown only for the printed script. The report does not mark that limit.

Honesty overall: the report marks the largest gaps (the record commit, the dry run, the informal pre-checks, the record commit's message, pushing). It does not mark:
- the two sha256 values;
- the doc commit subject;
- the parent claim, whose citation is wrong;
- the Co-Authored-By claim, which cites S for more than S shows;
- the CLAUDE.md and specs/0000 path examples;
- the P1 logs claim;
- note 8;
- the reflog claim, which is ambiguously marked;
- the limit on "not touched" and "not used" to what the printed script shows.
