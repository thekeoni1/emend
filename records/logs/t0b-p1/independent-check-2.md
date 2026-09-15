# T0b P1 independent check 2

- Checked commit: 401a2f73c3ac03d948e7c297ba117a5268b323af (T0b P1 phase
  commit, pushed; CI run 34977885012 concluded success)
- HEAD when the check ran: 49f134079703489ad53b0bb8aa1148517d6193f1
  (check 1 record commit, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below
- Inputs, with sha256:
  - specs/0003-ci.md
    437e249e22004828ac0cf2ac47f262aeddb3d188fab43ef3a80ad6b8ed4058e5
  - specs/0001-bootstrap.md
    ebb2545a358b6c38b14dd00c177a3c47e6f36a332c79dfa14fcd4f380b231a37
  - the phase diff, `git diff 0870c36..401a2f7`, saved as <scratch>/t0bp1-diff.txt
    8ee5e0df27c3164988538a2e11e2dc4edf827359085c43dc97a8b4f2970dcca7
  - draft report 2, <scratch>/t0bp1-report-draft2.md
    707c18e8551ce6eeeb8ef8c6c1e2ef717b27045d87340996abff05acdf5cfc75
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
  - records/logs/t0b-p1/independent-check-1.md
    8d55e73a93b7c7c1e7c25e1c1bd68c8a394e0f904514ce51a02abc84e906384c

---
**Independent check 2: T0b P1 (draft report 2)**

**Context disclosure.** My context includes a CLAUDE.md from the harness and an auto-memory index. I did not use either as evidence. My evidence was the six named inputs and nothing else. I ran no git or gh commands and changed nothing. My only commands were read-only ones on those files: sha256sum, byte counts, and grep counts that print no matched text. The hashes of specs/0003, specs/0001, the diff and the five logs match check 1's header.

**1. Does the diff do what specs/0003 P1 says, and nothing else?**

Yes. The diff has 5 files, 242 insertions and 3 deletions (my own count).
- **Planning's doc commit 71c44b8:** ROADMAP.md, specs/0002 and the new specs/0003. The specs/0003 in the diff has the same sha256 as the file on disk (scan.log line 362).
- **Phase commit 401a2f7:** ci.yml and check.sh. scan.log lines 307 to 325 give the same split.
- install-hooks.sh and privacy.sh are not in the diff, and gate.log line 87 shows install-hooks.sh at its recorded hash.

scripts/check.sh (diff lines 83 to 126):
- `--ci` sets CI=1. Giving both flags, in either order, prints the new usage line and exits 2.
- CI=0 is set before the arguments are read, so an inherited CI variable cannot turn on the skip.
- With `--ci`, the script prints the spec's SKIP line word for word and exits 0 after hygiene. This comes before step 8's denylist-variable refusal.
- Without `--ci`, step 8 is unchanged. gate.log lines 97 to 104 show nothing runs after PASS privacy, so `exit 0` skips only step 8.

.github/workflows/ci.yml (diff lines 7 to 38) meets every P1 item:
- push trigger on all branches; no other trigger;
- top-level `contents: read`; no secret referenced;
- one job on ubuntu-24.04 with a 30-minute timeout;
- checkout pinned to the spec's SHA, with the tag in a comment and `persist-credentials: false`;
- the exact rustup line, then `rustc --version` and the installed targets;
- the gate step in CI mode;
- no caches, no uploaded artifacts, no other actions.

Could the change weaken the gate, let a push through, or expose private data? I found no path:
- **The hook does not use check.sh.** push.log line 197 shows it runs privacy.sh directly, and the push output (push.log lines 196 to 202) holds no check.sh lines.
- **The push procedure needs PASS privacy.** It compares against exactly eight PASS lines, PASS privacy included (push.log lines 81 and 184). A `--ci` run cannot pass that comparison.
- **The denylist refusal is not weakened.** In `--ci` mode the script skips before that refusal, but it then runs no privacy scan and prints SKIP, never PASS.
- **CI sees only repository content.** The workflow has a read-only token, no persisted credentials and no secrets.
- **The run log stays out of the repository.** ci-run.log script lines 133 to 151 read it into a temp directory that is deleted on exit, and only the filtered lines are printed.

**2. Do the logs contain every required line and follow the log rules?**

Content:
- **gate.log:** eight PASS lines in order, exit 0, 0 stderr bytes, controls, and a clean tree (lines 97 to 117).
- **ci-local.log:**
  - the `--ci` run: seven PASS lines, then SKIP, exit 0 (lines 146 to 157);
  - both flag orders: usage line on stderr, exit 2 (lines 161 to 179);
  - the secrets grep exits 1, with a control that exits 0 (lines 218 and 219);
  - privacy.sh passes over 70 commits with the workflow file tracked (lines 239 to 248).
- **push.log:**
  - step 1: the doc commit is first in the pushed range (line 156);
  - step 2: the denylist variable, core.hooksPath, the hook executable and equal to the installer, and the full gate (lines 160 to 188);
  - step 3: the push, where the hook prints PASS privacy (lines 197 to 205);
  - step 4: ls-remote equals local main and the repository is still PRIVATE (lines 211 to 226).
- **ci-run.log:**
  - both Actions settings responses and their parsed checks (lines 159 to 166);
  - `gh run list` in plain form (line 177);
  - the run's facts as JSON (lines 186 to 198);
  - the runner label (line 203);
  - the filtered gate lines and the rustc line (lines 213 to 222);
  - the log's line count, without the log itself (line 211).
- **scan.log:** follows the scan rules. The script is printed, grep is called by full path under LC_ALL=C, every form has a positive control, and it reports 0 hits and RESULT: PASS.

Two departures are disclosed and do not weaken the evidence:
- The plain `gh run view` output is replaced by its JSON form, because the plain output held non-ASCII bytes.
- The runner label comes from the jobs API instead of the run log.

Rules:
- **First line:** every log's first line names the commit (for scan.log, the diff range) and HEAD. push.log part 2 repeats HEAD at line 190.
- **Control and non-ASCII bytes:** my own count is 0 control bytes, 0 carriage returns and 0 non-ASCII bytes in all five logs, check 1's file and draft 2.
- **Hostname:** a whole-word count of the hostname and FQDN is 0 in all seven files.
- **Session paths, links and tokens:** every hit from my pattern greps is script text or a scan control (scan.log lines 39, 40, 51, 124, 132, 140 and 141; ci-local.log line 90).
- **Email addresses:** the only email-shaped string besides the co-author address is the allowlisted example.invalid control at scan.log line 145.
- **Already-reviewed items:** the repository URL (push.log), the API URL with a numeric repository id (ci-run.log line 159) and the user and group names (push.log line 165) were already reviewed by check 1 or accepted in the P7 rulings. None is private.
- **Evaluate once:** each check() call prints one result from one evaluation. Part 2 of push.log re-tests the denylist variable and core.hooksPath at push time; that is a new evaluation, not a second print of the same one.
- **Full CI log:** never printed.

Below the scope filter, cosmetic only:
- The "command:" lines at ci-run.log lines 175, 182, 186 and 201 leave out the `--repo` argument that the printed script passes, and line 201 abbreviates the jq filter. The exact commands appear in the script at the top of the log.
- Check 1's own file (independent-check-1.md) is in no scan log. My byte and pattern counts on it are clean, but I cannot check it against the denylist. The pre-push hook will scan it when it is pushed.

**3. Are draft 2's numbers and quoted strings in a primary?**

Verified against a primary:
- the diff stat, the four log hash prefixes and scan.log's hash, and the three commit hashes;
- every gate.log, ci-local.log, push.log and ci-run.log line reference in the Acceptance section;
- 70 commits and 145 blobs; 9 commits and 105 blobs; the hook hash prefix;
- the push result line, run id 34977885012, 2 polls, 188 lines, 8 non-ASCII bytes and the rustc string;
- the Actions settings strings;
- scan.log's mode 600, 7 entries, 5 files, 2 messages, 4 logs, 0 hits and RESULT: PASS;
- the start times 13:42:33Z (gate.log line 2), 13:43:34Z (ci-local.log line 2), 13:43:49Z (push.log line 2), 13:52:37Z (push.log line 190) and 13:52:55Z (ci-run.log line 2);
- the dates 2026-07-17 and "push to any branch" (specs/0003).

Not in a primary, and marked as session output:
- planning's word, and the maintainer's authorisation quote at report lines 35 and 36;
- "committed without editing";
- everything in departure 1: the memory-stop quote, the first run's start time of 13:42:51Z, the gate file's last write at 13:42:51Z, the model server of about 5.4 GB, 16243 MB of free memory, "every check line ... was yes", and "no file ... changed";
- the published_at timestamp (departure 6);
- the findings sentence, now marked "my reading, not shown in a primary".

Overstatements, all minor:
- **"No file in the repository changed between the runs"** (departure 1): the stated basis, the same HEAD and a clean `git status`, covers tracked and unignored files only, not ignored build output. The claim is also session output for the first run.
- **"so the runs were sequential"** (departure 1): an inference from an unlogged file time. It is marked, but no primary shows it.
- **"every required log line present"** (report line 168), attributed to check 1: check 1 did not say this in those words. It recorded two substituted items and one gap, the unrecorded first run. The summary is a little generous; the numbered defect list after it is accurate.

**Does draft 2 correct or record every defect check 1 raised?**

Yes, and it represents check 1 fairly apart from the line 168 wording.
- **Defect 1 (push authorisation):** now stated under "Preconditions for the push", marked as session output, with part 2's start time. It remains unverifiable from primaries, so planning must confirm its own word and the maintainer's authorisation directly.
- **Defect 2 (stopped first run not in a log):** recorded in departure 1 and left for planning's ruling. The phase log is not corrected. That fits "record", and the stop was environmental, not a check failure.
- **Defect 3 (unmarked statements):** both statements now sit under departure 1's session-output marker.
- **Defect 4 (cosmetic):** the wording and the step range are fixed. The joined `stderr:` label stays disclosed as departure 5.
- **The overlap point:** answered with the start times, marked as session output.
- **The unmarked findings sentence:** now marked.

**Remaining defects, most severe first**

1. **Low (push authorisation evidence):** nothing in the primaries shows planning's word, the maintainer's typed authorisation, or that both came before the push at 13:52:37Z. Draft 2 states them as session output. Planning should confirm both from its own record and with the maintainer. This does not reopen the privacy path, because the hook's PASS privacy over the pushed range is in push.log lines 197 to 205.
2. **Low (log rules):** the first ci-local run, stopped by the environment, is recorded only in the report, not in a phase log. It awaits planning's ruling.
3. **Cosmetic:** the three minor overstatements under question 3; the abbreviated "command:" lines in ci-run.log; check 1's file is in no scan log (the hook will scan it at the next push).

I found nothing that could leak private data or let a push through without the privacy check.
