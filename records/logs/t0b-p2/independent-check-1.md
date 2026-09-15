# T0b P2 independent check 1

- Checked commits: 41a99a16d114c1ea7ba8105f7a3d6ac67c9a9207 (planning doc
  updates on local main, not pushed) and the throwaway
  afd3a8c7dd96cc34acf3ba02500f115e2c75bd58 on the deleted branch
  ci-negative (CI run 34991440664 concluded failure)
- HEAD when the check ran: 02d46cec9b5378b9134f66060b6ace2a4808bb81
  (T0b P2 record commit with the four logs, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below and a summary of planning's P2 instructions
- Inputs, with sha256:
  - specs/0003-ci.md
    33521535c1eb440bf9e8af765bd1bcba7aafa4c556ce5e216ff3983352bd591f
  - specs/0001-bootstrap.md
    46b0d7ff243e37af89ccf2384106700171b4898230b8802e52b903ed161418a0
  - the diff, `git diff ec7bf8c..41a99a1`, saved as <scratch>/t0bp2-diff.txt
    0d3cdb31d036f46596031e9d5394810660acfb9865555281db6a28cac913ddbc
  - draft report 1, <scratch>/t0bp2-report-draft.md
    9c86c3bbcdfe74870f7b7a5bee7c75aa20a6715d115a6c729ff78725f3c4a346
  - records/logs/t0b-p2/push.log
    98f373fb1c779943c9d74a6e25a76041d93b10866613116551b873f6192bdddd
  - records/logs/t0b-p2/ci-run.log
    7dad1e79cd429173bd4b473dcb09c6ef14fe5f7a626649fec5b0cc528859883c
  - records/logs/t0b-p2/cleanup.log
    a05d9a3f2501626d2cbbef857e37b9134764a46468b5190208c13c6fc77c5e56
  - records/logs/t0b-p2/scan.log
    9cbd063fba8344c95487bbbe8a278e8766a1b7635cab03d3807d2d88abf94a5e

---
**Independent check, T0b P2**

My context has a CLAUDE.md from the harness (the emend project instructions) and user memory entries. I did not use either as evidence. Evidence is the two specs, the diff file, the draft report and the four logs. I also ran a few byte counts over those same files; they are noted below.

**1. Does the diff do only planning's doc updates, and did nothing reach main on GitHub?**

Yes, on both counts.

- **The diff:** it touches exactly three files: ROADMAP.md, specs/0001-bootstrap.md and specs/0003-ci.md. My count is 43 added lines and 5 removed, which matches the report.
  - ROADMAP: the P1 row changes from READY to DONE with its hashes and run id. P2 stays READY.
  - 0001: one new rule. A run stopped by the environment is noted in the header of the log that reruns it.
  - 0003: a new Status line, the checkout date split into commit date and release date (the pin is unchanged), and the new section "Planning rulings on P1".
  - There is no code, workflow or script change.
- **The commit:** scan.log lines 306 to 313 show one commit in the range, with parent ec7bf8c and the same three files. Line 351 shows its message ends with the Co-Authored-By line, and lines 352 to 359 show zero hits on every scan.
- **Nothing reached main on GitHub:** remote main is 401a2f7 at every point: before the push (push.log 242, 245), after it (329), after the deletion (cleanup.log 119, 122, 125 to 129), and as the head of main's latest run (cleanup.log 132, 134). The doc commit 41a99a1 appears on no remote ref.
- **GitHub received only the throwaway commit:**
  - The branch was cut from 401a2f7, which GitHub already had (push.log 249, 270).
  - It is one commit ahead of that base (push.log 271, 299).
  - The push used an explicit single refspec (push.log 302), and git reported only the new branch (313, 319).
  - The remote heads afterwards were exactly main and ci-negative (329).
  - The hook scanned all 71 commits in the branch's history with 0 findings (305, 307, 317, 318). That matches planning's corrected expectation of 71.
  - The diff for that commit is one line in tools/xlsx-diff/src/main.rs (push.log 254 to 266, 274 to 276).

**2. Do the logs have every required line, and do they follow the log rules?**

**Required lines: all present.**

| Requirement | Where |
|---|---|
| Scratch clone with origin set to the GitHub URL, the only remote | push.log 214 to 222 |
| Identity from scripts/privacy-identity.txt | push.log 223 to 226 |
| install-hooks.sh run; hook matches what the installer writes | push.log 227 to 235 |
| core.hooksPath empty; EMEND_PRIVATE_DENYLIST unset | push.log 236, 237, re-checked at 298 |
| Branch from 401a2f7; rev-list count 1 | push.log 249, 271 |
| Misformatted line | push.log 262, 263 |
| cargo fmt --all --check fails locally | push.log 279 to 291 |
| Push of ci-negative only; PASS privacy; 71 commits scanned | push.log 302 to 319 |
| Still PRIVATE | push.log 334 |
| gh run list --branch ci-negative | ci-run.log 136 |
| JSON conclusion failure | ci-run.log 144, 152 |
| Failing step "Gate in CI mode" | ci-run.log 149, 153 |
| Filtered lines PASS host, PASS i686, then FAIL fmt; no privacy line | ci-run.log 169 to 171, 174, 175 |
| Runner label and rustc line (the P1 filter) | ci-run.log 159, 173 |
| git push origin --delete ci-negative; hook skipped it | cleanup.log 104 to 114 |
| gh api branches lists only main | cleanup.log 117 to 122 |
| Main's latest run is 34977885012, success | cleanup.log 132, 134 |
| Scratch clone deleted | cleanup.log 142, 143 |
| Scan per the scan rules | scan.log (controls, 3 files, 1 message, the 3 other logs, RESULT: PASS at 395) |

**Log rules:**

- **Control bytes:** zero in all four logs, including scan.log, which cannot scan itself. My own count found tab, LF and CR only, and the same count form caught a control byte in a positive test. None of the four logs has non-ASCII or U+2014 bytes.
- **Session links, temp session paths, tokens, hostname:** no session link, no path inside a temp session directory, no token and no hostname in any log. My hostname count was 0 in each log, with a positive control that matched.
  - In scan.log, one line matches a session-link form (143) and one matches a home-path form (142). Both are the script's own control strings: a split pattern name and a made-up home path. Neither is a real link or path.
  - Scratch paths show as `<scratch>`.
- **Email addresses:** only the allowlisted identity address (push.log 225, 270), the Anthropic noreply address, and an example.invalid control address in scan.log.
- **Evaluate once:** every check is evaluated once and printed once. I found no check computed twice.
- **Full CI log:** never printed. It is read into a temp file, filtered and deleted (ci-run.log 108 to 116). Only its line count, 193, is printed (167).
- **First line (small gap):**
  - ci-run.log and scan.log name the commit and HEAD.
  - push.log line 1 names the base commit (401a2f7) and HEAD. It cannot name afd3a8c, which did not exist yet. Part 2, appended at line 297, gives a timestamp but no HEAD. The later logs show HEAD was still 41a99a1, so this is minor.
  - **cleanup.log line 1 names HEAD but no commit.** It names the branch only, not 401a2f7 or afd3a8c. That is a small departure from the rule in specs/0001.

**3. Is every number and quoted string in the report in a primary?**

I checked every line citation (P, R, C and S numbers) and they all point to the right log lines. These all match primaries:

- The three log hash prefixes, which scan.log 363, 372 and 381 list and which match the files as they are now.
- Both full commit hashes and 401a2f7.
- 71, 1, 146, 193, run ids 34991440664 and 34977885012, 7 entries, mode 600, and the step numbers.
- The quoted lines `eprintln!(   "{message}"   );`, `PASS privacy`, the privacy count line, the FAIL fmt line, the rustc line, and the pre-push skip line.
- The `[new branch]` and `[deleted]` lines, which match the logs byte for byte, spacing included.

These do not appear in a primary:

- **"3 files, 43 insertions, 5 deletions":** marked as session output. My count of the diff agrees.
- **"committed without editing on local main":** marked as session output.
- **The preconditions section:** planning's word, the follow-up agreeing on 71, and the maintainer's quoted authorisation sentence. The heading marks all of it as session output with no primary.
- **17 lines ending in a space in push.log:** marked as session output. My count is also 17.
- **The quoted hook fragment `range=$local_sha` in Departure 1:** it appears in none of the specs, the diff or the logs, and the report does not mark it as session output. It is the only unmarked quoted string with no primary.
- **Departure 5's claim about GitHub keeping the commit:** marked as the session's understanding, not tested.
- **Departure 6, "No run was stopped":** not marked. It is consistent with the logs, whose headers note no stop.
- **"The record commit is local only" and "hash in the check header":** not in any primary I was given, and not marked.

Three statements go further than the evidence:

- **"P2's only commit on main is your doc commit":** the report's own Commits section lists a record commit, which will also be on main. The sentence is true only for the phase range ec7bf8c..41a99a1.
- **"the throwaway commit lives only in P and R":** Departure 5 says GitHub may still hold the commit, and the Actions run stays in history. The sentence is wording only, not a privacy matter.
- **Push timing:** "The maintainer typed in this session, before the first push" has no timestamp in any primary to order it against push.log 297.

**Scope-filtered findings (could leak private data or let a push through):** none.

- The push could not carry more than intended. It used an explicit single refspec from a clone guarded like main: hook matches the installer, hooksPath empty, denylist variable unset and refused in part 2.
- The hook scanned 71 commits with 0 findings.
- The throwaway commit that may stay on GitHub, and the run that stays in Actions history, hold only a one-line formatting change.
- Every log is free of hostname, tokens, session links and non-allowlisted addresses.

**Defects, most severe first**

1. The report quotes `range=$local_sha` in Departure 1 without a primary and without marking it as session output.
2. The report says "P2's only commit on main is your doc commit" and "the throwaway commit lives only in P and R". The first ignores the record commit it lists itself; the second conflicts with Departure 5.
3. cleanup.log line 1 names no commit, only the branch and HEAD. push.log part 2 (line 297) gives no HEAD.
4. "The record commit is local only" is not backed by any primary given to the checker and is not marked as session output.
