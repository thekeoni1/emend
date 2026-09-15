# T0 P7 independent check 2

- Checked commit: af4b355e028e50f07e42c6cbc52bb451765868d1 (planning doc
  updates, the last P7 commit before its logs; P7 has no code commit)
- HEAD when the check ran: 5e6d629c289de73c21f13d1269e1ce4daf25edc8
  (check 1 record commit, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  five inputs below
- Inputs, with sha256:
  - specs/0001-bootstrap.md
    17505f473a278ec4ffa5acc0f95533d1243c5c778ab49ad5c50e3ea5d58c3cc2
  - the phase diff, `git diff cf5240b..af4b355`, saved as <scratch>/p7-diff.txt
    d4f301fd6432733fb073b737e4716568cdbf7bfd8947f4d7cfe9740bd3261bf5
  - draft report 2, <scratch>/p7-report-draft2.md
    fc887f594c78abb576ade7c5f3d67f4824bd242c3593968f61595e9d45c44542
  - records/logs/t0-p7/push.log
    d014e3eb4c21e0c4d32fb2ef01b4eb8b51680e61574e42d7286c40a07262c4f5
  - records/logs/t0-p7/independent-check-1.md
    cd276687c8325b4120458c69eb405d7a7c418a46b426a69774ffd0bf61bb366f

---
**P7 independent check 2: GitHub push log and draft report 2**

**Context note.** My context has a CLAUDE.md from the harness: the project CLAUDE.md, the user's memory index and attribution notes. I did not use it as evidence. My evidence is only the five files:
- S: the spec
- D: the phase diff
- R2: draft report 2
- L: push.log
- I1: check 1's answer

Beyond reading them, I ran only byte counts, sha256s, line counts and string searches over those files. The hostname search compared L and R2 against the output of `hostname`. I ran no git command.

## 1. Does the diff do what the spec says and nothing else?

**Yes, with one ROADMAP item S does not name.**

- **Files and size:**
  - D touches two files, ROADMAP.md and specs/0001-bootstrap.md.
  - My count is 127 added lines and 6 removed lines.
  - D has 0 control bytes, 0 CR, 0 non-ASCII bytes and 0 U+2014.
- **Hashes:**
  - D's sha256 matches I1's header (header line 13).
  - S's sha256 matches I1's header (header line 11).
  - So check 1 and check 2 read the same spec and diff.
- **spec changes:** every added spec line in D appears word for word in S:
  - the status lines (S 3-8)
  - the phase order (S 21)
  - the P6b rulings (S 443-497)
  - the new denylist precondition (S 994-995)
  - the P6c section (S 1025-1081)
  - the Done line (S 1085)
- **ROADMAP changes:** P6b becomes DONE with the range 690c0f5 to cf5240b, which matches S 7-8. A P6c row is added, which matches S 1025. Both are consistent with S.
- **The "gate self-test script" backlog item (D lines 20-22):**
  - S does not name it anywhere.
  - Its maintainer approval on 2026-09-15 has no primary.
  - R2 records this only as session output (Surprise 7).
- **What D cannot show:** it has no commit messages, so the trailer claims and "committed without editing" cannot be checked from D.

## 2. Does push.log have every required line, and does it follow the log rules?

**Required content: all present.**

| Requirement | Where in L |
|---|---|
| Step 1: gh visibility command, output PRIVATE, exit 0 | 164-169 |
| Step 1: ls-remote of the URL, exit 0, 0 refs | 170-174 |
| Step 2: remote add command, exit 0, `remote -v` | 177-183 |
| Step 3: denylist variable unset | 186 |
| Step 3: hooksPath exit 1 with 0 output bytes | 187-190 |
| Step 3: hook executable, sha256 equal to the installer heredoc | 191-196 |
| Step 3: check.sh, eight PASS lines in order, stderr 0 bytes, exit 0, 0 control bytes, clean status | 197-214 |
| Step 4: `git push -u origin main` (no `--no-verify`), exit 0, hook `PASS privacy` | 222-231 (PASS line at 225) |
| Step 5: remote main equals local main | 237-244, re-run at 453-462 |
| Repository still private | 245-247, re-run at 463-469 |
| No authentication output | none in the push or gh output; stderr of the gh calls is empty |

Caveats:
- **Hook comparison:** step 3 compares the hook with an awk extraction of the installer's heredoc (L 89), not the installer's real output. S 453 accepts the heredoc as the reference, so this is minor.
- **Part 2 closing line:** part 2 ends `part 2 checks all as expected: no` (L 248).
- **Part 3:** the fix-named re-run (L 250-251) covers only step 5 and visibility. It does not re-evaluate the step 4 `PASS privacy` check.
  - The literal line at L 225 still meets the acceptance wording.
  - The step 4 check is also the most exposed of the three double-evaluated checks. L 132 pipes `cat` of two separate files into `grep -q` under pipefail. If grep exits after the first file, cat can take SIGPIPE on the second write.
  - This is my reading of the script, not a test.

**Log rules: met.**
- **First line:** L line 1 names the commit and the HEAD. The part 2 and part 3 headers (L 216, 250) also give HEAD.
- **Bytes:** 0 control bytes and 0 CR. Tabs are at lines 182, 183, 239 and 455 only. No non-ASCII bytes and no U+2014.
- **Paths, sessions and hostname:**
  - No temp session paths: 0 hits for `/tmp/`, `.claude` or `claude-`.
  - 0 hits for `session`.
  - 0 hits for the machine's hostname. This search had no positive control.
- **Emails and tokens:**
  - No email addresses: 0 hits for `@`.
  - 0 hits for common token prefixes or `token`.
  - The only long hex strings are the full commit hash (10 times) and the hook sha256 (twice).
- **Minor, not forbidden by S 92-100:** line 192 shows the local user name, group name and a file time.
- **Imprecise heading:** the heading at line 255 says one script wrote parts 2 and 3. Part 2 cannot have run the verify block, so this is imprecise.
- **Hash:** L's sha256 matches I1's header (line 17) and R2 line 10.

## 3. Does every number and quoted string in draft report 2 appear in a primary?

**Found in a primary:**
- **Hashes and commands:**
  - L's sha256: I1 header
  - `git diff cf5240b..af4b355`: I1 header line 12
  - 2 files, 127 insertions, 6 deletions: D, and I1 line 28
  - the full af4b355 hash: L 1
- **Part 1 quotes:** L 166, 174, 177, 178, 180, 186, 188, 190, 192, 195, 199-214.
- **Part 2 quotes:**
  - the pre-push line: L 224, abbreviated with "..."
  - `PASS privacy`: L 225
  - the privacy line with 52 commits and 108 blobs: L 228
  - `* [new branch]      main -> main`: L 230
  - `still PRIVATE: yes`: L 247
  - the part 2 "no": L 248
- **Part 3 quotes:** L 462, 469, 471 and 472. The fix is named at L 251.
- **Surprises 1:** "Three part 2 checks ran twice" matches L 131-132, 146-147 and 150-151. These are the only fail-setting double evaluations. "L line 225" is correct. The heading at line 255 is quoted correctly.
- **Surprises 2:** 51 is at S 469 and 52 is at L 161/228. "The extra one is af4b355" is inferred, and the blob counts fit: S 456 says 106 blobs at P6b, L says 108 now, and D changes 2 files.
- **Surprises 3:**
  - the 7-entry denylist: S 479 and 994
  - the 11 patterns: S 1048
  - the two P6c forms: S 1046-1048
  - I1's 0 control bytes: I1 line 69
- **Surprises 4 to 7:**
  - tabs at 182, 183, 239 and 455: L
  - line 192: L
  - remote origin and tracking: L 182-183 and 226
  - the ROADMAP note: I1 lines 37-38

**Not in any primary:**

| Item in R2 | Marked as session output? |
|---|---|
| "committed without editing"; record commits local only; trailer-only messages (R2 lines 4, 20-23) | Yes (line 4 only through line 21) |
| Planning's word, the maintainer's authorisation quote, and the email-settings quote with its timing (lines 27-32) | Yes, by the section heading |
| "The pushed commits carry only the noreply identity" (line 33) | Yes, by the heading. S 476 supports it only for the 51 commits planning scanned, not for af4b355. The hook's 0 findings (L 228) is partial support. |
| Re-run counts, five and three times (line 68) | Yes |
| Planning scanned both doc files before commit (line 83) | Yes |
| The pre-commit scan of L (line 86 on) | Yes. My own counts agree for control bytes, non-ASCII, U+2014, the hostname and `/tmp/`, but no scan.log exists. |
| Credential helper already configured, no config changed (lines 97-99) | Yes. L 12-13 partly supports the helper. |
| "P6b's hook.log already records the same `ls -l` form" (lines 94-95) | **No.** P6b's hook.log is not an input, and this is not marked. |
| "awk extraction, as in P6b" (line 45) | **No.** S 453 says "heredoc" and names no awk method. The awk command itself is in L 89. |
| Hook output as `privacy.sh --range af4b355e...` | Abbreviated with "...", not verbatim. Minor. |

**Overstatements:**
- **"T0 P7 (first push) is done" (line 3):** by R2's own account the email-settings precondition was confirmed in the implementing session only after the push. S 992 says "Preconditions, all required", and planning has not verified the phase. "Done" overstates it.
- **"Draft 2 corrects the points check 1 raised" (line 5):** defects 1, 5 and 6 are recorded or deferred, not corrected.
- **The mitigation (line 33):** "carry only the noreply identity, and the hook passed" reads as though it offsets the precondition. The hook's email check does not test the GitHub account settings.

**Preconditions and steps unmet or only partly met:**
- **Email settings (S 996):** before the push this came only through planning's relay.
  - S 996 does not say the confirmation must be typed in the implementing session (only S 998, authorisation, says that). Whether the relay counts is for planning to decide.
  - Check 1 asked for a planning ruling. R2 no longer asks for one.
- **Authorisation and planning's word:** these are session output only, with no primary.
- **Step 5 and acceptance:** met in L.
- **Record commit stays local:** unshown.
- **scan.log:** S 73 requires a control-byte scan.log for each phase, and P7 has none. R2 records this and asks for a ruling.

**Does draft 2 handle check 1 fully and fairly?**
- **Defect 1 (email settings):** recorded, with the later in-session confirmation. The request for a planning ruling is dropped, and "now confirmed" could read as closing the issue.
- **Defect 2 (part 2 "no" and part 3 coverage):** recorded accurately. The SIGPIPE and second-gh-call hypotheses are named and marked untested, and the part 3 gap is stated.
- **Defect 3 (52 vs 51):** now attributed to planning's message as session output.
- **Defect 4 (unrecorded claims):** now marked. The full cf5240b hash and "ahead by 1" are gone.
- **Defect 5 (scan.log):** deferred for a ruling.
- **Defect 6 (ROADMAP item):** recorded fairly in Surprise 7.
- **Defect 7 (R wording):** all five points are fixed at R2 lines 20, 66, 64, 52 and 92.
- **Defect 8 (L wording):** recorded in Surprises 1 and 5.

Overall, R2 represents I1 fairly. Its summaries match I1 lines 37-38, 76-77, 110-114 and 125-132.

## Remaining defects, most severe first

1. **Email-settings precondition (S 992, 996):** by R2's own account it was not confirmed by the maintainer before the push. R2 calls P7 "done", drops check 1's request for a planning ruling, and offers a mitigation that no primary shows as equivalent. Planning must rule.
2. **Part 2 false "no":** the cause is still unidentified. Part 3 does not re-evaluate the step 4 `PASS privacy` check, the likeliest place for the pipefail race. Acceptance rests on the literal line at L 225.
3. **No scan.log:** the log scan exists only as session output, against S 73. It awaits a ruling. My counts found no issue, but they are not a committed primary either.
4. **Unmarked claim with no primary:** "P6b's hook.log already records the same `ls -l` form" (R2 lines 94-95). A smaller one is "as in P6b" for the awk extraction (line 45).
5. **ROADMAP backlog item and approval (D lines 20-22):** not in S, and supported only by session output.
6. **R2 wording:** "corrects the points check 1 raised" overstates, since three points are deferred. The hook line quote is abbreviated.
7. **L, accepted as recorded:** the line 255 heading is imprecise, and line 192 shows local user and group names.
