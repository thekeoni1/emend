Checked phase: T0 P5b (specs/0001-bootstrap.md)
Checked commit: aca5d3a20b9e1eb92eaf12b08ee1a98e1d4a8511 (P5b phase commit); logs through ee0168ae8cf4d2821931362b181e0e6c7b98b0f1
HEAD when the check ran: ee0168ae8cf4d2821931362b181e0e6c7b98b0f1
Checker: a fresh general-purpose subagent, not a fork
Inputs (sha256):
- specs/0001-bootstrap.md: e47c9f41397feac91bbbca72b222d7bfcd3f5ebe69b460332d8b7263f394e803
- <scratch>/p5b-diff.txt (git diff 9b7aef1f690a6be5fc198b326ab36b992bf5e2d7..aca5d3a20b9e1eb92eaf12b08ee1a98e1d4a8511): aa202ea2c2e7ea63540678791619b663b4b88fb4d43e9202dc4e3fcf3aade868
- records/logs/t0-p5b/gate.log: 7ab349545f714d1800dc523db2fca7cd6b39051ee894996ad4669583d5fa2724
- records/logs/t0-p5b/negative.log: 1ceeedfe33a582320b7aa12f849315c2e51edb8cc1a9f68a8176d7bcf2f9d3e0
- records/logs/t0-p5b/scan.log: cdde5f359658ce5cb6b91ef7f20e2e5b0d82f4f0804f66897997e30de9db6f49
- records/logs/t0-p5b/verify.log: 954f2d62529f25737c928798cfafeaf5219194f10e72314d9a26befe4ed2a6dc
- records/logs/t0-p5b/independent-check-1.md: be059545961f2c4386adb65105d9a8e3770f9fbf1e4e2e6753c0bb7784195fc0
- <scratch>/p5b-report-draft.md (draft 2): 8ed748b20e61d194ad1995e367154fb458c26330897c416b24ad806fe99599b8
---
# T0 P5b independent check 2

Note on context: the harness also put other material into my context. It included the repository's CLAUDE.md, an auto-memory index, a git status snapshot, user identity details, a commit attribution reminder, and environment and tool listings. The snapshot shows the branch, a clean status, and five recent short hashes with their subjects. I did not use any of it as evidence, and I do not repeat the identity details. The snapshot gives short forms of two record commits that the draft report names, and subjects that bear on what those commits are. I treated both as appearing in no primary. I read only the eight named inputs and ran no commands.

Line references: "D n" is line n of the diff. "G n", "N n", "S n" and "V n" are lines of `records/logs/t0-p5b/gate.log`, `negative.log`, `scan.log` and `verify.log`. "I1 n" is a line of `records/logs/t0-p5b/independent-check-1.md`. "R n" is a line of the draft report. "Spec n" is a line of `specs/0001-bootstrap.md`.

## Q1. Does the diff do what the P5b spec and the rulings on P5 say, and nothing else?

PARTLY. Every P5b bullet is implemented as written, and nothing outside step 7 and the key check changes. verify.log shows one defect that independent-check-1.md did not raise. In the real scan, a `git grep --cached` read error is treated as "no hit", which breaks the scan rule "An error is never reported as zero hits" (Spec 83-84, applied to check.sh by Spec 431-432). Defect A is now demonstrated and conflicts with "Read what would be committed" (Spec 564). One sentence of the key-check bullet is not met in one input case.

### Scope

- The diff has two commits: 873851f (ROADMAP.md, specs/0001-bootstrap.md) and aca5d3a (scripts/check.sh) (S 247-257). That matches the P1R ruling that planning's doc updates are committed first (Spec 155-157).
- The doc hunks add 100 lines and delete 6. I recounted: D 5 gives +2 -1; D 207, 222, 232 and 285 give +5 -4, +2 -1, +46 and +45. The post-image matches the spec input at Spec 3-7, 20-21, 282-326 and 559-602. No primary can show that this text is unedited from planning's working tree.
- Changes the spec does not ask for, all inside step 7:
  - the `hgrep` helper;
  - the `readonly` regexes;
  - one shared stderr file;
  - moving the U+2014 check after the path checks (D 197);
  - an error message at D 198 that no longer gives an exit code.

  The draft report no longer lists any of these (independent-check-1.md did, at I1 65-66).

### P5b bullets

1. **Read the index with `git grep --cached -I` under `LC_ALL=C`.** Done at D 82. The working-tree `[ -f ]` filter is removed (D 52-62). Evidence: N case (a) at N 196 and N 204, and case g1 at N 304.
2. **A symlink is checked as its target string and never followed.** Done at D 90-114.
   - V 367-368 shows mode 120000 for both links.
   - V 370-375 shows `git grep --cached` lists no symlink, with or without `-I`. This closes I1 32.
   - N (c) at N 240 shows the target file's content is not read. N (h) at N 349 shows the target string is read.
3. **A file git treats as binary is not a text file.** Done by `-I`.
   - V 371 and V 373 show git grep's own binary decision for blob.bin: it is listed without `-I` and not with it. That is more direct than the numstat at V 377, which is git diff's decision.
   - V v5 (V 395) and V v6 (V 410) show that the decision follows `.git/info/attributes` and an untracked working-tree `.gitattributes`. So "what would be committed" is not what the step reads. See defect A below.
4. **Tracked paths with a control byte fail first, and only a count is printed.** Done at D 170-174, ahead of the office, corpus and U+2014 checks. N (b) at N 222 shows an embedded LF matched. The in-script control uses 0x01 only (D 131).
5. **`emend-testing` matched case-insensitively; the fixtures exception stays case-sensitive.** Done at D 187 and D 181. N (d) at N 258 shows the corpus match. V v3 at V 357 shows `Tests/Fixtures/` failing closed, which closes I1 53.
6. **FAIL lines print git-quoted paths.** Done by `quoted()` at D 120-126 and used at D 184, 191 and 200. N (f) at N 288 and N 290 is the evidence.
7. **`ANTHROPIC_API_KEY` is checked before step 1.** Done at D 23-28.
   - V 289-295 shows the order: `set -euo pipefail` (6), the argument loop (9), `cd` (19), the exports (22, 24), the key check (28), then step 1 (68).
   - Not met in one case. The spec says the check "prints `FAIL hygiene: ANTHROPIC_API_KEY is set in the environment` and exits 1 before any other step runs" (Spec 575-578). V v1 (V 324-326) shows that with the key set and a bad argument, the script prints only the usage line and exits 2. No step runs and the value is not printed, so nothing leaks. Still, the required line is absent and the exit code differs. V labels exit 2 as the expected result (V 176). That expectation is the log's own, not the spec's, and needs a planning ruling.
   - The V grep prints only matching lines, so check.sh lines 1-5, 7-8, 10-18, 20-21 and 25-27 are not shown by any primary. A cargo call there is implausible but not excluded.
8. **An in-script positive control for every grep form.** One control exists for each form (D 131-155), and each uses the same call as its real check. See the next section for what the U+2014 control does not detect.

### Defects and gaps beyond independent-check-1.md

- **New: a `git grep --cached` read error is reported as zero hits.** The mechanism is shown by D 82-89 read with V 422-486.
  - `index_dash` treats only exit 2 or more from git grep as an error (D 84). All git grep stderr goes to `hygiene-err` and is never examined.
  - In V v7, the real scan's git grep could not read `text.txt` (V 480-481). That blob holds U+2014: it is the control file (D 147) added to the scratch index (V 486).
  - The next line, V 482, is `fatal: git cat-file ...: bad file`. It can only come from the symlink loop (D 101). That loop runs only after git grep returned less than 2 and `ls-files -s` succeeded. So git grep's read error did not stop the scan, and `text.txt` was silently not a hit.
  - The step failed, with the D 198 message at V 422, only because the control's symlink blob was also unreadable. If the unreadable blob had been a text file with no symlink beside it, `index_dash` would have returned rc 1 and step 7 would have printed `PASS hygiene`.
  - The draft report says only that "The step then failed closed" (R 156-157). V's observations (V 487-488) do not record this either.
- **New: the U+2014 positive control passes despite its own read errors.** V 422 is the real-scan message, not "positive control failed" (V 487 counts that as absent). So in the same v7 run, the control at D 152-155 passed.
  - V 423-479 are 57 "unable to read" lines for 30 distinct clone paths. They cannot come from the real scan, because the clone's own objects hold those blobs. They come from the control run in the throwaway repository.
  - The control checks only its exact output (D 153), so it cannot detect a read error.
  - The error branch then prints all of those control-run lines under the real-scan FAIL (V 423-479), which blames the wrong run.
  - Why only 30 of the 52 clone paths appear is not explained by any primary.
- **Defect A is wider than V shows.** V v6 shows that git grep uses a working-tree `.gitattributes` for `--cached`. Nothing in step 7 examines attributes, so a committed `.gitattributes` holding `* -diff` would exempt every file from the U+2014 rule in every clone. That file is exactly what gets committed and pushed. This matches the ruling's letter (Spec 311) and is untested.
  - An untested question for planning, not a finding: some git versions may limit `git grep --cached` to the sparse-checkout cone unless `--sparse` is given. `git ls-files` lists every path, so a sparse checkout could hide dash-bearing blobs from the scan and from nothing else. No primary covers this.
- **A raw non-ASCII byte through the error branch (inference).** The error branch prints `hygiene-err` unfiltered (D 198, as I1 58 noted). V 423-481 shows that this file holds git's per-path error lines with the path in plain form. A non-ASCII path with an unreadable blob would therefore print raw bytes. Control-byte paths cannot reach it, because D 170-174 fails first. The P3 Q6 ruling covers stderr pass-through but was made before paths reached that output.
- **An unguarded command exits with no FAIL line.** V 289 shows `set -euo pipefail`, so a failing `mkdir -p`, `ln -s` or `rm` in the control block (D 130, 148, 151) exits non-zero with no FAIL line. That fails closed but departs from "one PASS or FAIL line per step" (Spec 430-431). The P3 Q4 ruling covered only failures before the first step.
- **`quoted()` can lose its error.** It runs inside a command substitution in `fail`'s argument (D 184, 191, 200), where errexit is not inherited by default. A failing `git ls-files` there would give a FAIL line with an empty or partial path list. The step still fails.
- **The corpus-path FAIL line prints the whole path** (D 191). For a real corpus path, that could put a private file name into a log. The spec asks for the path, and only the log-privacy rule (Spec 88-98) bears on it. This is for planning to weigh.
- **Still as recorded:** the 0x01-only control, spaces in joined paths, and repeated paths in an unmerged index (which also inflates the control-byte count), per I1 52-58.

### What the primaries can and cannot show

They show:

- the changed hunks;
- the line order around the key check (V 289-295);
- `--quick` still running step 7 (V 345-346);
- symlink and binary behaviour (V 365-377);
- the latent defects as behaviour.

They do not show:

- the body of `fail()` (only its output format and exit 1, at N 204-205);
- the unprinted early lines of check.sh;
- git grep's exit code itself (it is inferred above from which code path ran);
- a tracked `.gitattributes` or sparse checkout;
- any grep error branch;
- that the doc text is unedited.

## Q2. Do the logs contain every line the P5b spec requires?

PARTLY. Every required observation is present and as expected. G 1 is still mislabeled. verify.log itself has no scan in any primary, and neither do the messages of the two later record commits. V v7 records the wrong things about its most important outcome.

### gate.log (Spec 583)

- A full run from `/` on a cold cache: G 82-84.
- All seven PASS lines in order: G 86-92 and G 95. Exit 0 (G 94), stderr 0 bytes (G 93).
- Zero control bytes, with controls: G 98-101.
- Clean status afterwards: G 103-104.
- The first-line rule is met in substance, but G 1 reads `P5 phase commit` for the P5b commit. It stays mislabeled.

### negative.log (Spec 584-599)

Each case runs in a fresh clone (N 3-4, and the `clone at` lines).

- **(a)** Staged, then deleted (N 195-196): `FAIL hygiene: U+2014 in tracked file(s): notes.txt` (N 204), exit 1. Present.
- **(b)** The path `"evil\ntests/fixtures/x.xlsx"` (N 214) gives the count FAIL line (N 222). Control bytes are 0 (N 224). Present.
- **(c)** A symlink to `<scratch>/outside.txt` gives `PASS hygiene` (N 240) and exit 0. Present. The mode is not shown in N; V 368 shows it for a link built the same way (V 203-204 against N 107-108), in a different clone.
- **(d)** N 258. Present.
- **(e)** The first line is the FAIL line (N 267, N 271), with no `PASS host` (N 272). Present. N 273-275 are extra.
- **(f)** The quoted path (N 288), with 0 non-ASCII bytes (N 290). Present.
- **(g)** g1 (N 304), g2 `x.xlsx` (N 318) and g3 `Y.XLSM` (N 332) repeat P5 (a) and both parts of P5 (b) (Spec 548-551). Present.
- **(h) and (i)** are labeled beyond the spec (N 338, N 355).
- **Ending:** clean status (N 373) and `all negative cases as expected: yes` (N 374).

### scan.log and the rules for every phase

- **Scan rules.**
  - The script is printed and `/usr/bin/grep` is named (S 6-188, S 191).
  - `LC_ALL=C` byte forms (S 42-50).
  - Each form has a control with the same command form (S 197-243).
  - Exit codes are handled (S 57).
  - The phase files, both messages, and G and N are scanned (S 264-328), including the doc commit per the P1R ruling (S 250-253). The result is `RESULT: PASS` (S 333).
- **Control-byte rule.**
  - S counts control bytes in G and N (S 314, S 322).
  - V counts them in the committed G, N and S (V 493, 496, 498) and in independent-check-1.md (V 507).
  - Nothing counts them, or scans for non-ASCII, P6 patterns, host or denylist entries, in verify.log. It is a committed phase log. "A log cannot scan itself" was accepted at P1, but the draft report does not mark verify.log as unscanned.
- **Toolchain rule.** Met in the printed scripts (G 17, N 21, V 31) and in check.sh (V 292-293).
- **Commit messages.**
  - S 292 and S 301 cover the two phase commits.
  - V 501-506 covers 4f696ed.
  - The messages of 4d3065a (HEAD at V 1) and of the commit that holds verify.log have no scan or trailer check in any primary. The draft report does not say so.
- **Log privacy.**
  - Scratch paths print as `<scratch>` (N 231, V 415), and V 40 also hides the repo path.
  - V 423-482 prints repo-relative paths and blob ids only.
  - V 284 prints the bash build triple, not a hostname.
  - The denylist and hostname are covered for G and N (S) and for independent-check-1.md (V 508-511), not for V.
- **First-line rule.**
  - G 1: commit and HEAD given, but mislabeled.
  - N 1: correct.
  - S 1: names the range and HEAD.
  - V 1: names the phase commit, the record commit and HEAD.
  - independent-check-1.md: phase on line 1 and commit on line 2, per the P1R Q3 ruling.

### Each independent-check-1.md gap against verify.log

| independent-check-1.md gap | verify.log | Does the draft report say so accurately? |
|---|---|---|
| G 1 mislabel (I1 95) | Open; V 13 says it cannot be fixed | Yes (R 160-162). But "derived from P5's script" is in no primary and is not marked. |
| N counters have no control (I1 121) | Partly closed. V 302-304 validates the same `tr` byte classes (V 45 and 50 against N 30-31). N's own functions still read a failed `tr` as 0. | Yes (R 173) |
| Case (c) mode not shown (I1 103) | Partly closed: V 368, in another clone | Yes (R 105-106) |
| Case (i) binary decision not shown (I1 122) | Closed by V 371 and 373 | Partly. R 114-115 cites the numstat, which is diff's decision, not grep's. |
| Timing, whether S hashed the final files (I1 123) | Closed. V 491-496 shows the committed G and N equal S's hashes. S read them from the working tree, so they were complete and moved in before S ran. | No. R 183-185 still calls this a session fact. |
| "git grep skips symlinks" was only asserted (I1 32) | Closed (V 370-375) | Yes (R 77-80) |
| Fixtures exception case not shown (I1 53) | Closed (V 357) | Yes (R 69-70) |
| `set -e` and unguarded commands not visible (I1 54) | Partly closed. V 289 shows `set -euo pipefail`; the missing FAIL line on such a failure is not noted. | Partly (R 166-167) |
| Order, `cd`, argument parsing and `--quick` not visible (I1 74-81) | Closed (V 289-295, V 332-346). `fail()` is still not shown. | Yes (R 40-53), except the spec tension in v1 (Q1, bullet 7) |
| Defect A (I1 49) | Open by design, observed (V 395, V 410) | Yes (R 142-151) |
| Defect B (I1 50) | Open by design, observed (V 414, 485-486) | Partly. The read-error outcome and the control's pass (Q1) are missing. |
| Output details and stderr dump (I1 56-58) | Open | Yes (R 168-172) |

### New issues in verify.log

- **V v7's observations miss what matters.** V 487-488 record only that neither a control FAIL nor `PASS hygiene` appeared. They do not record that the control passed with 57 read errors, or that the real scan's read error on `text.txt` did not stop the scan (Q1).
- **V 176 expects exit 2 without noting the spec.** Spec 577 says the key check exits 1.
- **V's host control covers `hostname` only** (V 158); S also covers the FQDN.
- **Cosmetic:** `env  scripts/check.sh` has a double space at V 334, 350, 385 and 400.
- **No failed check.** V 514-515 shows `checks failed: 0` and `RESULT: PASS`.

## Q3. Does every number and every quoted string in the draft report appear in a primary?

PARTLY. Almost everything traces to a primary. A few claims are unmarked, one citation is ambiguous, one claim undersells V, and the new read-error finding is missing.

### Found in a primary

**Commits and hashes**

- Range `9b7aef1f...`..`aca5d3a2...` (R 13): S 1 and S 246. The diff states no range; that is correct.
- 873851f00ccf... with its subject "T0 P5b: commit planning doc updates (P5 rulings)", parent 9b7aef1, `M ROADMAP.md` and `M specs/0001-bootstrap.md` (R 18-19): S 248, S 250-253.
- aca5d3a... with its subject "T0 P5b: harden step hygiene", parent 873851f and `M scripts/check.sh` (R 20-21): S 247, S 255-257.
- The check.sh sha256 `19345cf7...` (R 22): S 272, G 77 and V 286.
- 4f696ed01ed8... (R 23-24): V 1 and V 490-500, I1 2-3. "Reads those logs from it": V 253-265.
- 4d3065ae2ae7... as HEAD at run (R 25): V 1.

**The doc commit**

- 100 insertions and 6 deletions (R 32): the D hunk headers, and I1 26. My recount agrees.
- S scans both files and the message (R 31): S 264-287 and S 300-308.

**Line order and the key check**

- The seven line-order entries (R 42-48): V 289-295. `19:cd ...` and `68:quiet host ...` are abbreviated.
- The FAIL line and exit 1 (R 50-51): D 26-27.
- `usage: scripts/check.sh [--quick]` and exit 2 (R 52): V 325-326.

**Code quoted from the diff**

- The controls (R 55-64): D 129-155. `grep -z -c -P '[\x01-\x1f\x7f]'` is D 132 without the `hgrep` wrapper, which adds `LC_ALL=C /usr/bin/grep` at D 71.
- `git ls-files -z`, the byte range and count-only reporting (R 65-67): D 165, D 170 and D 173.
- `LC_ALL=C git grep --cached --no-color -I -l -z -F` (R 73): D 82.
- Mode 120000, `git cat-file blob` and `LC_ALL=C /usr/bin/grep -q -F` (R 74-76): D 97, D 101 and D 106.
- `":(literal)<path>"` (R 83): D 123 has `":(literal)$p"`. This is a placeholder, not verbatim.
- The spec covers every FAIL line (R 84): Spec 573-574.

**verify.log cases v3 and v4**

- `Tests/Fixtures/x.xlsx` (R 70): V 357, which gives the full path `tools/xlsx-diff/Tests/Fixtures/x.xlsx`.
- `120000 link-dangling` and `120000 link-outside`, the two git grep listings, and the numstat line for blob.bin (R 77-81): V 367-377.

**gate.log lines (R 90-95)**

- The seven PASS lines: G 86-92.
- `exit: 0 (expect 0)`: G 94.
- `stdout is exactly the seven PASS lines in order: yes`: G 95.
- `stderr bytes: 0`: G 93.
- `stdout: lines=0 bytes=0` and `stderr: lines=0 bytes=0`: G 100-101.
- Cold cache from `/`: G 81-84.

**negative.log lines (R 97-116)**

- `DRYRUN_CHECK_SH is not set`: N 4.
- `control lines=0 control bytes=0 non-ASCII bytes=0` in all 11 cases: N 206, 224, 242, 260, 269, 290, 306, 320, 334, 351 and 368.
- (a): N 196 and N 204.
- (b): N 214 and N 222.
- (c): N 231, N 240 and N 241.
- (d): N 258.
- (e): N 267, 272, 273 and 274 (`output lines: 1 (expect 1)`).
- (f): N 288 and N 290.
- (g): N 304, 318 and 332.
- (h): N 349.
- (i): N 358 and N 366-367.
- The ending: N 374.

**scan.log lines (R 118-122)**

- `controls: all as expected` with "46 control lines": S 243, and S 197-242 is exactly 46 lines.
- `files in diff: 3; files scanned: 3; files read: 3`: S 288.
- `messages scanned: 2; commits in range: 2`: S 309.
- `logs scanned: 2 (expected 2)`: S 328.
- `total hits: 0` and `RESULT: PASS`: S 330 and S 333.

**verify.log summary (R 123-128)**

- `controls: all as expected`, `checks failed: 0` and `RESULT: PASS`: V 320, V 514 and V 515.
- Hash equality and 0 bytes: V 491-498.
- `c-ctrl=3`, `c-nonascii-bytes=2` and `ERROR`: V 302-304.
- The only trailer: V 501.

**Log hashes (R 132-136)**

- G `7ab34954...`: S 312 and V 491.
- N `1ceeedfe...`: S 320 and V 494.
- `equal to the hash scan.log recorded: yes`: V 492 and V 495.
- S `cdde5f35...`: V 497, also I1 10.
- independent-check-1.md `be059545...`: V 507.

**Findings (R 142-173)**

- `* binary`: V 222 and V 380.
- `notes.txt: binary: set` and `notes.txt: diff: unset`: V 383-384.
- `PASS hygiene`: V 395 and V 410.
- `*.txt -diff`: V 233 and V 397.
- The ruling's words: Spec 311 and Spec 564.
- `scratch index entries before: 52` and `after: 54`: V 414 and V 485.
- `control repository paths now in the scratch index (text.txt or link): 2`: V 486.
- `FAIL hygiene: error while scanning the index for U+2014`: V 422.
- `about: P5 phase commit aca5d3a...`: G 1.
- The weak points: I1 52-58; line 6: V 289.

**Other notes and open items**

- Other note 1, every other step 7 grep calling `/usr/bin/grep` under `LC_ALL=C` (R 177-179): D 71 and D 106.
- `2026-09-15T01:56:59Z` (R 184): G 2 and N 2.
- The beyond-spec items (R 188-189): N 167, N 176 and N 74.

### In no primary, and marked as such

- ee0168ae8cf4... (R 27).
- The role of 4d3065a (R 25-26).
- "Unedited", and the pre-add stat and scan (R 34-36, R 193).
- The dry run at 873851f (R 180-182). N 36-40 shows only that a dry-run mode exists.
- The V sha256 `954f2d62...` (R 137-138).
- NUL reasoning (R 186-187).
- No install, no remote, no push (R 197-198).

### Unmarked, inaccurate or undersold

- **R 161:** "The gate-log script was derived from P5's and kept the old label" is in no primary and is unmarked.
- **R 195:** "V line 22" is ambiguous. Line 22 of verify.log is `set -uo pipefail`. The export is check.sh line 22, as printed at V 292.
- **R 183-185:** undersold. That G and N were complete and final when S hashed them is shown by V 491-496 (see the table in Q2).
- **R 25-26:** undersold. V 273 and V 507 show independent-check-1.md is committed at 4d3065a. Only "this commit added it" is not shown.
- **R 114-115:** cites the numstat (V 377), which is git diff's binary decision. Git grep's own decision is V 371 and V 373.
- **R 153-154:** "as a git hook would see it" is a claim about git, asserted in V 241 and I1 50 but shown by no primary. It is unmarked.
- **R 50-53:** presents v1 as a pass. It does not say that the spec's "prints the FAIL line and exits 1" (Spec 576-577) is not met when the argument is bad.
- **R 156-159:** "failed closed" is true of the run but omits that it failed only because of the symlink blob. The draft report also does not say that git grep's read error did not stop the scan, or that the control passed with read errors (V 422-487).

### Omissions

- **The read-error defect and the control's pass** (Q1).
- **Unscanned logs and messages.** verify.log has no scan in any primary. The messages of 4d3065a and the verify.log record commit are unscanned.
- **Beyond-spec diff changes.** The draft report no longer lists the refactors, the reordering and the error message that lost its exit code (I1 65-66).

Otherwise the draft report separates primary evidence from session output carefully, and its Q3 fixes from check 1 (the stat line, note 7's mislabel and the export line) are applied.
