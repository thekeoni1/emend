# T0b P1 independent check 1

- Checked commit: 401a2f73c3ac03d948e7c297ba117a5268b323af (T0b P1 phase
  commit, pushed; CI run 34977885012 concluded success)
- HEAD when the check ran: b65203a4e3c2c2b3740d7e9d5625aac35cb9cab6
  (T0b P1 record commit with the five logs, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below
- Inputs, with sha256:
  - specs/0003-ci.md
    437e249e22004828ac0cf2ac47f262aeddb3d188fab43ef3a80ad6b8ed4058e5
  - specs/0001-bootstrap.md
    ebb2545a358b6c38b14dd00c177a3c47e6f36a332c79dfa14fcd4f380b231a37
  - the phase diff, `git diff 0870c36..401a2f7`, saved as <scratch>/t0bp1-diff.txt
    8ee5e0df27c3164988538a2e11e2dc4edf827359085c43dc97a8b4f2970dcca7
  - draft report 1, <scratch>/t0bp1-report-draft.md
    036ff1601bd60b0446c14726099f0a5dc182c76f8a05c053d41a0ea4e398809f
  - records/logs/t0b-p1/gate.log
    c76e9b736e71fba82e78f8a979c5ed8ca9b80420216e78dab66c9c03cbf14991
  - records/logs/t0b-p1/ci-local.log
    d352c2783a50560a3037c6be8afb7beefb1275d83dd12ca9b13e578ae7d1b479
  - records/logs/t0b-p1/push.log
    baa47f628b0d53daae64b39991e536aeffb23bf2047ebd0158f93782a5e68020
  - records/logs/t0b-p1/ci-run.log
    6da61995dc0e461bb5ab110990728360a3ffeb22beb1738ebefa87b7bc570075
  - records/logs/t0b-p1/scan.log
    d11b760e982c469194431172dee85c352775fa1757d11546dcd24037290481f3

---
**Independent check: T0b P1**

My context holds a CLAUDE.md from the harness (project instructions) and an auto-memory index. I did not use either as evidence. My only evidence was the files named in the brief: the spec 0003, the three 0001 sections, the diff, the draft report and the five logs. I ran no git or gh commands and changed nothing. The only commands I ran on files were read-only (sha256sum, byte counts and grep counts).

**1. Does the diff do what specs/0003 P1 says and nothing else?**

Yes. The diff covers 5 files. Three come from planning's doc commit 71c44b8 (ROADMAP.md, specs/0002 and a new specs/0003). The other two are the phase commit 401a2f7 (a new .github/workflows/ci.yml and a changed scripts/check.sh). scan.log lines 307 to 325 give the same split. There are no other files, and install-hooks.sh and privacy.sh are not touched. My own count from the diff is 242 insertions and 3 deletions.

scripts/check.sh (diff lines 83 to 126):
- **Arguments:** `--ci` sets CI=1. Giving both flags, in either order, prints the new usage line to stderr and exits 2. Unknown arguments still exit 2, now with the new usage line.
- **Step 8:** with `--ci` the script prints the SKIP privacy line, word for word as the spec gives it, and exits 0. That happens right after PASS hygiene and before step 8's denylist-variable refusal.
- **No other use of CI** appears in the diff. Without `--ci`, step 8 is unchanged.
- **Inherited CI variable:** CI is set to 0 unconditionally before the arguments are read. An inherited `CI` environment variable (GitHub sets one) therefore cannot turn on the skip locally.

.github/workflows/ci.yml (diff lines 7 to 38) meets every item in the brief:
- **Trigger:** push only, with a branches filter of "**". There is no pull_request, schedule or other trigger.
- **Permissions:** top level, contents: read. No secret is referenced.
- **Job:** one job, `check`, on runs-on ubuntu-24.04 with timeout-minutes 30.
- **Checkout:** pinned to the spec's exact SHA, with the v7.0.1 tag in a comment beside it and persist-credentials false.
- **Toolchain step:** the exact rustup install line, then rustc --version, then the installed targets for 1.98.1.
- **Gate step:** RUSTUP_AUTO_INSTALL=0 scripts/check.sh --ci.
- **Nothing else:** no caches, no uploaded artifacts, no other `uses` lines.

The workflow file, the check.sh diff and the new specs/0003 are all byte-consistent with ci-local.log lines 184 to 215 and with specs/0003-ci.md on disk.

Could the change weaken the local gate, let a push through without the privacy check, or expose private data? I found nothing that does.
- **The hook does not use check.sh.** push.log line 197 shows it runs scripts/privacy.sh --range directly, so `--ci` cannot affect it.
- **The push procedure demands the full gate.** It requires exactly eight PASS lines, including PASS privacy (push.log lines 81 to 82 and 184). A `--ci` run would fail that check.
- **CI sees only repository content.** The workflow has a read-only token, no persisted credentials and no secrets, and it runs only after a push.

One point is outside the scope filter: when CI is already exported, the value check.sh sets is passed on to child processes. That is harmless here.

**2. Do the logs contain every required line, and do they follow the log rules?**

Content:
- **gate.log:** the full gate, run from / after cargo clean. It shows eight PASS lines in order, exit 0, 0 stderr bytes, controls on the control-byte check, a clean tree, and a final line of all yes.
- **ci-local.log:** the `--ci` run (seven PASS lines, then SKIP, exit 0), and both orders of `--ci --quick` (usage line only, exit 2). It also has the secrets grep (exit 1) with a positive control, the workflow line checks, the tag lookup, and privacy.sh over 70 commits with PASS.
- **push.log:** the doc commit sits in the pushed range, which covers step 1. Step 2 covers the denylist variable, core.hooksPath, the hook being executable and equal to the installer heredoc, and the full gate with eight PASS lines. Step 3 is the push, where the hook prints PASS privacy over 9 commits. Step 4 shows ls-remote equal to local main and the repository still PRIVATE. Every check is printed once, from one evaluation.
- **ci-run.log:** it has both raw Actions permission responses and their parsed checks, `gh run list` in plain form (conclusion success), the run id, and the JSON form of `gh run view` with every step's conclusion. From the run log it prints only the PASS, SKIP and FAIL lines, in order, plus the rustc line and the log's line count. The full log is never printed.
  - **Departure A:** the plain `gh run view` output is not printed because it contains 8 non-ASCII bytes. The JSON form stands in for it.
  - **Departure B:** the runner label comes from the jobs API, not from the run log as the spec's wording implies.
  - The report discloses both (departures 3 and 4). Neither weakens the evidence.
- **scan.log:** follows the scan rules:
  - the script is printed, calls /usr/bin/grep and matches under LC_ALL=C;
  - every scan form has a positive control, including per-entry denylist controls that never print the entries;
  - it covers 5 files, 2 commit messages and the 4 other logs;
  - it checks the Co-Authored-By trailer on both commits;
  - it reports 0 hits and RESULT: PASS.

Rules:
- **First line:** every log's first line names the commit and HEAD.
- **Control and non-ASCII bytes:** by my own count, all five logs (scan.log included) and the draft report have 0 control bytes (beyond tab, LF and CR) and 0 non-ASCII bytes.
- **Log privacy:** I found no hostname, temporary session path, session link, token or Windows path.
  - Every hit from my own pattern greps is script text or a scan control: scan.log's pattern and control lines, and ci-local.log line 90, whose printf `\n` escape matches the drive-path shape.
  - The GitHub repository URL appears at push.log lines 20, 145, 146 and 201. An API URL carrying a numeric repository id is at ci-run.log line 159. Neither is private data.
  - push.log line 165 shows the ls -l user and group names, which were already accepted in the P7 rulings.

Gap: ci-local.log does not record the first run that the environment stopped. 0001 "How this task runs" says a failure is recorded in the phase log. The stop was environmental rather than a check failure, and it appears only in the report, as session output.

**3. Does every number and quoted string in the draft report appear in a primary?**

Everything I checked matches:
- **Diff and log hashes:** the diff stat (5 files, 242 insertions, 3 deletions) matches my count from the diff. The four log sha256 prefixes match scan.log and the files on disk.
- **Commits:** both commit hashes are in push.log and scan.log.
- **ci-local.log references:** every line reference (146 to 153, 155 to 157, 165 to 169, 175 to 179, 218 to 229, 236, 242, 244, 248, 251) points to the stated content.
- **push.log references:** every reference (188, 198, 200, 202, 217, 220, 224, 226) matches, as do the hook hash prefix and the 9 commits.
- **ci-run.log references:** every reference (159, 161, 164, 166, 169, 172, 174 to 180, 183, 197, 198, 203, 206, 211, 213 to 224, 227) matches, as do the run id, 2 polls, 188 lines, 8 non-ASCII bytes and the rustc string.
- **scan.log figures:** 7 entries, mode 600, 0 with whitespace, 5 files, 2 messages, 4 logs, 0 hits.

Not in a primary:
- **Marked as session output:** the memory-stop quote, "every check line of the partial first run was yes", 16243 MB, the published_at timestamp, and "committed without editing".
- **Not marked:** in departure 1, "Other workloads on this machine held most of the memory; I did not touch them" and "Nothing changed between the runs". Neither can be verified from the primaries.
- **Not marked:** the findings sentence, "CI receives only what the pre-push hook already let through, and its log holds only repository content and tool output". It is an assertion, reasonable given the rules against bypassing the hook, but no primary shows it.
- **Omitted:** the report never says planning's word and the maintainer's typed authorisation came before the push, and no primary records either. The spec (0003 lines 76 to 77) requires both.

Overstatements and imprecisions, all minor:
- "0 steps with another conclusion": the check at R 198 counts steps other than success or skipped. The claim is still true, because R 191 to 196 list only success.
- "R 188 to 196" for the steps: the step lines are R 191 to 196.
- Timing: gate.log started at 13:42:33, the ci-local rerun at 13:43:34 and push part 1 at 13:43:49. Unless the runs were very fast, the stopped first ci-local run overlapped the cold-cache gate run, and the report does not say so. This is no risk to passing: each script checks HEAD and a clean tree on its own.

**Defects, ranked by severity**

1. **Medium (push authorisation):** the draft report does not state that planning's word and the maintainer's typed authorisation came before the push, and no primary shows either. The spec requires both. It should be stated, marked as session output, or planning should confirm it.
2. **Low (log rules):** the stopped first ci-local run is not recorded in ci-local.log, as 0001 "How this task runs" asks for failures. It appears only as session output in the report.
3. **Low (report accuracy):** two unmarked, unverifiable statements in departure 1 (the other workloads, and "nothing changed between the runs").
4. **Cosmetic:** the "another conclusion" wording, the R 188 to 196 range, and the missing newline after the JSON before `stderr:` at R 159 and R 164 (already disclosed as departure 5).

Both ci-run.log departures (the plain `gh run view` and the runner label source) are disclosed and are not defects under the scope filter. I found nothing that could leak private data or let a push through without the privacy check.
