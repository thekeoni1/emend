Checked phase: T0 P5 (specs/0001-bootstrap.md)
Checked commit: 346385c8c8934c3485b27160a69fd032f35ef8b5 (P5 phase commit); logs through 1c57a2a0f88261e3f99800a27122d3c2fa24325e
HEAD when the check ran: 1c57a2a0f88261e3f99800a27122d3c2fa24325e
Checker: a fresh general-purpose subagent, not a fork
Inputs (sha256):
- specs/0001-bootstrap.md: d6e394c169016172b54af1d46c85fff2d39c26ca8ac41792b3a49693b1ff70b5
- <scratch>/p5-diff.txt (git diff a00d6fd27dea6b43e3f99f836c08e4cd1683ff13..346385c8c8934c3485b27160a69fd032f35ef8b5): c9697db0e6d39f8b6e60ce9c4d38991220cbdc11c07bc2f28f18f22c0bd1fd2d
- records/logs/t0-p5/gate.log: ba710008e8fbc52155d0f11755f96256b4fe76a9cc5d5ebc7c6982dd39cea552
- records/logs/t0-p5/negative.log: 4af428c1f44f4e3fe2feb09c5d18d832f140718a35e296feb9df20b047cf9cd8
- records/logs/t0-p5/scan.log: c7978f02d033990092984aa021dfaeecca12b6500244119339679d258747fefc
- records/logs/t0-p5/verify.log: 1913a6efa22a860320a9fdff13b994be2b2db973369055581f658293fdfb1a3e
- records/toolchain-2026-09-15.md: 5a6834de5c46c10ae9d6bde992aa736659008c67985112788be11ff14d1ea013
- records/logs/t0-p5/independent-check-1.md: 952f92eaa138497f810b2742a5792f0c891da46e5072e7842eb8c5d788d2e0e2
- <scratch>/p5-report-draft.md (draft 2): cb394c515bfdf8c92f7b0baf2296778087b8e5c75cfc363ebc2fd5ef4407fd5b
---
# T0 P5 independent check 2

Note on context: the harness gave me more than the nine inputs. It also gave me the repository's CLAUDE.md, an index of the user's auto-memory, a git status snapshot listing recent commit subjects and short hashes, a user identity detail, commit attribution guidance, and environment details that include a date. I did not use any of it as evidence, and I do not repeat the identity details. I ran no commands. Line numbers for the diff and the draft report are line numbers in those files as I received them. Short hashes stand for the full hashes printed in the primaries.

## Q1. Does the diff do what the P5 spec says and nothing else?

PARTLY. Every item the spec names is present in the diff, and nothing unrelated changed. But records/logs/t0-p5/verify.log shows that the committed step 7 passes at least one case the spec's wording says must fail (v4). Several branches the spec names are never exercised.

### What matches the spec

- **Doc-commit ruling.**
  - records/logs/t0-p5/scan.log lines 248-258: d8c1ee1 is the first commit after a00d6fd and changes only ROADMAP.md and specs/0001-bootstrap.md. Spec line 6 names a00d6fd as the end of P4. 346385c changes only scripts/check.sh.
  - The + side of the spec hunks matches the spec text I was given: diff lines 137-140 against spec lines 3-6, diff lines 148-191 against spec lines 235-278, and diff lines 200-221 against spec lines 471-491. The hunk offsets agree too (old 423 + 45 = new 468).
  - Hunk arithmetic: ROADMAP +2/-2; specs +4/-4, +45 and +22. That totals 73 insertions and 6 deletions.
  - scan.log scans both doc files at 346385c (lines 265-272 and 281-288), not at d8c1ee1. They are the same content, because 346385c touches only scripts/check.sh (scan.log line 258). It scans the doc commit's message at lines 301-309.
  - No primary shows that the files were "unedited", or that every pending planning change was included.
- **Step 7 positive control** (diff lines 75-79). The E2 80 94 bytes are written with `printf '\342\200\224'` and detected with `LC_ALL=C /usr/bin/grep -l -I -F`. Any non-zero exit fails the step, so a failed control and a grep exit of 2 or more both fail. It runs before the four conditions.
- **Condition 1** (diff lines 81-102). Tracked paths come from `git ls-files -z` and `mapfile -d ''`. The grep form is the same as the control's, with `--`. Exit 2 or more gives a FAIL labelled as an error (lines 96-98), and exit 0 gives a FAIL (lines 99-101).
- **Condition 2** (diff lines 104-112). `grep -i -E '\.(xlsx|xlsm|xltx|xltm|xlsb|xls|ods)$'` under LC_ALL=C covers the P2 list in any case. The `(^|/)tests/fixtures/` exception is case-sensitive. Both greps treat 2 or more as an error.
- **Condition 3** (diff lines 114-119). A fixed-string match, with exit 2 handled. verify.log v1 (line 336) now exercises it.
- **Condition 4** (diff lines 121-124). `${ANTHROPIC_API_KEY+set}` also catches an empty value, and the value is never printed. No run exercises the empty-value case.
- **native.**
  - `--all-features` (diff line 31).
  - `pkg_config` (line 37).
  - `/[-_]sys$/` (line 45).
  - `PASS native (allowlisted -sys: ...)` with the names or `none` (lines 41-58).
  - The FAIL text now names both forms (line 54).
- **licences.** `cargo metadata --format-version 1 --all-features` (diff line 64).
- **Colour and guard.** verify.log lines 300-302 show `22:export RUSTUP_AUTO_INSTALL=0` and `24:export CARGO_TERM_COLOR=never` in check.sh at 346385c. verify.log line 298 ties the committed check.sh to the checked hash.

### Nothing else

The diff touches three files (scan.log lines 262-264). The check.sh changes stay within steps 5 to 7. The only unrequested items are consequences of the fixes:
- the native FAIL wording;
- `_sys` names appearing under the `allowlisted -sys:` label (records/logs/t0-p5/negative.log line 436), which is cosmetic.

records/toolchain-2026-09-15.md is not in the diff. verify.log line 487 shows it committed at fadc53f, which is reasonable, since it must name the commit the gate passed at.

### Defects that verify.log demonstrates (observed, not fixed)

- **v4** (verify.log lines 402-416): a force-added `a<LF>tests/fixtures/b.xlsx` gives `PASS hygiene`. That file is a tracked office file that is not inside a `tests/fixtures/` directory, so this contradicts condition 2 as written (spec lines 462-463). It is a spec miss in the committed script, not just an edge case.
- **v3** (lines 385-400): staged content holding the dash passes once the file is removed from the working tree.
- **v6b** (lines 449-462): git stores a symlink's target string as its blob, so tracked content holds E2 80 94, and the step passes.
- **v5** (lines 418-432): a text file with a NUL byte before the dash passes.

v3, v5 and v6b are misses only if "tracked text file" means the tracked content rather than the working-tree file. The spec does not define it.

- **v6a** (line 445): a false FAIL, reached by reading a file outside the repository through a symlink.
- **v1b** (line 350): `Emend-Testing` passes. This is literal compliance with the spec.
- **v2b** (line 380): a false FAIL for `Tests/Fixtures/`, which fails closed. .gitignore also needed `-f` there (script line 203).

### New gaps not raised in records/logs/t0-p5/independent-check-1.md

1. **Raw path bytes in FAIL lines.** Diff lines 100, 111 and 118 put tracked path names into the FAIL line verbatim. A path holding ESC or other control bytes, or non-ASCII bytes including E2 80 94, would put those bytes into check.sh output. A captured gate or negative log would then break the control-byte rule. No primary exercises this. Relatedly, a path name that contains U+2014 is not checked at all, because condition 1 reads content only. The spec asks only about content.
2. **Error branches never run.** No primary shows the step failing because the positive control fails, or because a step 7 grep exits 2 or more (for example, an unreadable tracked file). No primary shows the `git ls-files` failure branch either. All of these are shown by the code only.
3. **Empty present list skips the scan.** When no tracked file is present in the working tree, diff line 93 skips the U+2014 scan silently. This extends item 1 of independent-check-1.md.
4. **Features of non-member crates.** `--all-features` with `--workspace` enables features of workspace members only. Cases (d) and (f) use a feature of xlsx-diff (negative.log lines 280 and 327). No P5 primary covers a feature of a non-member dependency that would pull in `cc` or a non-allowed licence. Draft report note 1 gives this as reasoning. The spec's words, "a feature nothing enables", do not say whose feature.
5. **Permanent negative tests.** Ruling Q2 on P4 (spec lines 262-264) says cases (g) to (i) "become permanent negative tests in P5". The diff adds no committed negative-test script. They exist as one run printed in negative.log. The P5 section itself asks only for negative.log entries (spec lines 482-491), so this is an ambiguity for planning, not a clear miss.
6. **Item 5 of independent-check-1.md is untouched.** It is the single grep call over every present file and the argument-length limit. verify.log does not address it, and the draft report does not mention it.

### What the primaries can and cannot show

**They can show:**
- the committed hunks;
- which commits and files the range holds;
- that every verify.log and negative.log run used the check.sh committed at 346385c (negative.log line 4; verify.log lines 298-299 and script line 93);
- the observed behaviour above.

**They cannot show:**
- that the doc files were unedited;
- the rest of check.sh outside the hunks, apart from the two export lines: `set -euo pipefail`, the cd, `fail()`, `$OUT`, and `--color never` for rustfmt;
- that the spec I read is the committed blob (scan.log line 281 gives a hash I cannot compute);
- whether other logs exist under records/logs/t0-p5/;
- behaviour on a bash older than 5.3.9 (verify.log line 295).

## Q2. Do the logs contain every line the P5 spec requires?

PARTLY. Every line the P5 Logs and Acceptance lists require is present, and no check in any log failed. The gaps are in coverage under "Rules for every phase" for records written after scan.log, plus a few weak or fixed-text lines.

### gate.log (records/logs/t0-p5/gate.log)

- Line 1 names the commit and HEAD.
- `cargo clean` runs first (lines 125-126), then `(cd / && <repo>/scripts/check.sh)` (line 127).
- All seven PASS lines appear (lines 129-135), including `PASS native (allowlisted -sys: none)` and `PASS hygiene`.
- `stderr bytes: 0` (line 136), `exit: 0 (expect 0)` (line 137), and the exact seven lines in order (line 138).
- Control bytes are 0, with positive and tab/CR controls (lines 141-144).
- The script is printed (lines 7-117). Status afterwards is empty (line 169).

Present and complete.

### negative.log (records/logs/t0-p5/negative.log)

- Lines 1-5 carry the header. Each case runs in a fresh clone at 346385c, with fixtures in `<scratch>/ext` outside the clone.
- **(a)** A file written with printf escapes and staged (script line 101) gives `FAIL hygiene: U+2014 in tracked file(s): notes.txt`, exit 1 (lines 224-229).
- **(b)** `x.xlsx` gives FAIL, exit 1 (line 241). `Y.XLSM` gives FAIL, exit 1, in a separate clone (line 257). This is weak: neither case asserts that `PASS hygiene` is absent. The exit code and first-failure behaviour cover it.
- **(c)** `env ANTHROPIC_API_KEY=dummy` runs in a subshell (script line 84; log line 265).
  - The log shows the FAIL line (line 272) and the value absent from the output (line 277).
  - It also shows `ANTHROPIC_API_KEY set in this shell after case c: no` (line 278).
  - The parent session's state cannot be shown by a log.
- **(d) to (i)**: `FAIL native` or `FAIL licences`, each non-zero (lines 296, 318, 344, 368, 389 and 410). (f) shows `PASS native (allowlisted -sys: none)` first (line 343).
- **(j)**, beyond the spec: all seven PASS and the allowlisted names (lines 432-444).
- It ends with an empty main-tree status (line 447) and `all negative cases as expected: yes` (line 448).

### scan.log (records/logs/t0-p5/scan.log)

- The script starts with `#!/usr/bin/env bash` and uses `G=/usr/bin/grep` (lines 6 and 18).
- Bytes are matched under LC_ALL=C. U+2014 is matched as printf bytes with `-F`. There is no `\x{2014}`. Exit 2 or more counts as an error (line 58).
- Every form has a positive control, with negatives (lines 198-243; 46 lines).
- It scans the three diff files, both messages with the trailer check, and G, N and R. The result is 0 hits, 0 errors, `RESULT: PASS` (lines 339-342).
- Weak points:
  - The control-byte form counts lines, not bytes.
  - Its positive control holds only ESC and SI; there is no NUL, VT, FF or DEL control.
  - R is labelled a "log" (lines 329 and 337).

### Toolchain record (records/toolchain-2026-09-15.md)

All required contents are there:
- rustc (line 7) and cargo (line 12);
- the installed targets (line 14);
- `uname -r` only (line 15);
- `file` output with a repo-relative path, ELF 32-bit and statically linked (line 16);
- the commit the gate passed at (line 6).

These match gate.log lines 153-167. Weak points:
- Line 1 names no commit. The record is not a log, so this is not a failure.
- Lines 4 and 17 are fixed text (gate.log script lines 101 and 111).
- The binary comes from a `--no-run` build after the gate (gate.log line 147). `Finished ... in 0.06s` (line 148), after `cargo clean`, suggests the gate built into target/, but it does not prove this file is the one step 2 executed.

### git status clean afterwards

gate.log line 169, negative.log line 447, and verify.log line 508 (at HEAD 1a95419).

### Rules for every phase

- **Commit messages.**
  - d8c1ee1 and 346385c: scan.log lines 292-309.
  - fadc53f: verify.log lines 494-500.
  - 1a95419 (HEAD in verify.log) and verify.log's own record commit: no primary scans their messages.
  - For every commit, "Co-Authored-By line only", meaning no other trailers, is shown by no primary; only the last line and the absence of `^Claude-Session:` are shown.
- **Toolchain.** RUSTUP_AUTO_INSTALL=0 is exported by check.sh (verify.log line 301) and by the gate, negative and verify scripts (gate.log script line 19; negative.log script line 21; verify.log script line 33). Ad-hoc commands are not shown.
- **Control bytes.**
  - G, N and R: scan.log (lines) and verify.log lines 481-489 (bytes).
  - S: verify.log lines 490-491.
  - independent-check-1.md: verify.log line 506.
  - verify.log itself: no primary.
- **Scans.** verify.log's script follows the rules (script lines 18 and 28). The `expect` and `observe` forms in negative.log and verify.log have no dedicated positive control. Their "present" results act as controls (for example, verify.log line 340).
- **Privacy.**
  - G, N and R: scan.log lines 313-336.
  - independent-check-1.md: verify.log lines 501-505.
  - scan.log and verify.log: no primary runs the hostname, P6 or denylist forms over them. This is the same class as the P1 ruling that a log cannot scan itself, but it is not stated in the draft report.
  - I found no `uname -a` output, no Windows path and no session path. Paths appear as `<scratch>`, `<repo>` or repo-relative.
- **First line.**
  - gate.log line 1, negative.log line 1 and verify.log line 1 comply.
  - scan.log line 1 names a range whose end is the commit, which is acceptable.
  - independent-check-1.md lines 1-2 comply under the P1R ruling (Q3).

### Gaps from independent-check-1.md, and the draft report's account of them

| Gap in independent-check-1.md | Status in verify.log | Draft report |
|---|---|---|
| Q1 item 1, staged but deleted | Partly closed: v3 shows only the deletion variant. A working copy modified to drop the dash, and skip-worktree, are not shown. Observed. | Finding 1 is accurate. |
| Q1 item 2, newline in a path | Closed as observation (v4). | Finding 2 is accurate, but it does not say that this contradicts condition 2. |
| Q1 item 3, NUL byte | Closed as observation (v5). v5 does not measure the file's content, unlike v3 (line 389). | Finding 3 states the mechanism as fact. |
| Q1 item 4, symlinks | Closed as observation (v6a, v6b). | Finding 4 is accurate. It does not say v6a is a false FAIL, reached by reading outside the repo. |
| Q1 item 5, argument length | Open. | Not mentioned. |
| Q1 item 6, bash version | Closed for this machine (line 295). | Reported. |
| Q1 item 7, emend-testing case | Closed as observation (v1b). | Finding 5 is accurate. |
| Q1 item 8, in-script controls | Open by design. v1, v2 and v2b act as external controls. | Finding 6 is accurate. |
| Q1 observation, `--quick` | Closed (v7, lines 466-478). | Reported. |
| Q1 observation, step order and path names in FAIL lines | Not a log matter. | Note 7. |
| Q1 cannot-show, check.sh outside the hunks | Partly closed: only the two export lines. | Spec item 4 is accurate, but it does not say the rest stays unseen. |
| Q2 (b) does not assert PASS absent | Open. | Not mentioned. Minor. |
| Q2 no emend-testing or fixtures test | Closed (v1, v2). | Reported. |
| Q2 R mislabelled as a log | Open. | Acknowledged. |
| Q2 record commit message unscanned | Closed for fadc53f. Open for 1a95419 and verify.log's record commit. | Reports fadc53f only. Silent on the other two. |
| Q2 no other trailers | Open. | Not mentioned. |
| Q2 R's first line names no commit | Open. | Not mentioned. |
| Q2 git status clean afterwards | Partly closed: clean at HEAD 1a95419, not after verify.log's own record commit. | See Q3. |
| Q3 unmarked items 1-6 | Items 1, 2, 3, 4 and 6 are fixed in the draft report. Item 5 is marked as session-only. | See Q3. |

### New issues in verify.log

1. **Start-state guard.** Line 3 says the run starts "after record commit fadc53f... and the check 1 record commit". That is fixed text. The guard (script line 112) checks only that the tree is clean. Nothing checks that HEAD is the check 1 record commit, or that fadc53f is an ancestor of HEAD. The files are read from fadc53f by hash (lines 480-490).
2. **v7 runs `--quick`.** It sits inside a log handed to planning, and P3 says "Every log handed to planning comes from a full run" (spec lines 396-397). I read that rule as being about gate logs, and v7 is a deliberate demonstration. I flag it without calling it a violation.
3. **Controls.** Control bytes are counted by bytes. The P6 control uses one pattern only (line 309), where scan.log controls all nine. That is enough for a message and a check file.
4. **Cosmetic.** The command lines read `scripts/check.sh  (in the clone)` with a double space.

## Q3. Does every number and quoted string in the draft report appear in a primary?

PARTLY. Nearly every item traces to a primary line. The draft report marks most items that have no primary. A few are unmarked or only weakly supported, and some known gaps are left out.

### Traced, section by section

**Primaries and range (draft report line 14).** scan.log lines 1 and 247. "The diff file does not state its range" is true of the diff.

**Commits (draft report lines 19-27).**
- d8c1ee1, its subject "T0 P5: commit planning doc updates (P4 rulings)", its parent, and `M ROADMAP.md` / `M specs/0001-bootstrap.md`: scan.log lines 249-254.
- 346385c, its subject "T0 P5: gate step hygiene; close native and licences gaps", its parent, and `M scripts/check.sh`: scan.log lines 248 and 255-258.
- fadc53f: verify.log line 1 and lines 480-490.
- 1a95419: verify.log line 1. Its role is marked as not in a primary.
- 1c57a2a0f88261e3f99800a27122d3c2fa24325e: in no primary, and marked.

**Doc commit (draft report lines 31-36).**
- "S scans both files and its message": scan.log lines 265-288 and 301-309. The files are scanned at 346385c, which is the same content (scan.log line 258).
- `2 files changed, 73 insertions(+), 6 deletions(-)`: the quoted string is in no primary. The counts follow from the diff's hunk headers, as independent-check-1.md line 24 recounts.
- "a pre-commit scan found 0 hits": in no primary.

**Spec items (draft report lines 40-74).**
- `printf '\342\200\224'`: diff line 75. `LC_ALL=C /usr/bin/grep -l -I -F`: diff line 78.
- `git ls-files -z`: diff line 81. The working-tree presence test: diff line 88.
- `grep -i -E` (abbreviated): diff line 105. The case-sensitive exception: diff line 108.
- The v1 FAIL string: verify.log line 336. v2 `PASS hygiene`, exit 0: verify.log lines 366-370.
- `SKIP i686: --quick` and `PASS hygiene`: verify.log lines 468 and 473.
- `--all-features`, `pkg_config`, `/[-_]sys$/`: diff lines 31, 37 and 45.
- The FAIL reason with `scripts/sys-allowlist.txt`: negative.log line 318.
- `PASS native (allowlisted -sys: none)`: gate.log line 133.
- The `cargo metadata` line: diff line 64.
- `22:export ...` and `24:export ...`: verify.log lines 301-302. "Outside D's hunks" is true: the first hunk starts at line 86.
- Toolchain strings, the binary path, the `file` excerpt, and the "Full gate ... passed at" line: toolchain record lines 6-16; gate.log lines 153-167.

**Acceptance (draft report lines 78-128).**
- The seven PASS lines, `exit: 0 (expect 0)`, `stdout is exactly the seven PASS lines in order: yes`, `stderr bytes: 0`, and `stdout: lines=0 bytes=0` / `stderr: lines=0 bytes=0`: gate.log lines 129-144.
- "cold cache, from /": gate.log lines 124-127.
- `DRYRUN_CHECK_SH is not set`: negative.log line 4.
- (a): negative.log lines 216 and 224. (b): lines 241 and 257.
- (c): lines 265, 272, 277 and 278.
- (d): line 296. (e): line 318. (f): lines 343-344.
- (g), (h), (i): lines 368, 376/389 and 397/410. (j): lines 436-439.
- `workspace members: xlsx-diff` in (d) to (i) and not in (j): negative.log lines 290, 312, 337, 362, 383 and 404, and lines 418-444.
- `control characters in the captured output: lines=0 bytes=0` in all 11 cases: negative.log lines 226, 243, 259, 274, 300, 322, 349, 372, 393, 414 and 440.
- `all negative cases as expected: yes`: line 448.
- The status lines: gate.log line 169, negative.log line 447, verify.log line 508.
- scan.log: `controls: all as expected` (line 244; 46 is a count of lines 198-243, which I confirm), and lines 289, 310, 337, 339 and 342.
- verify.log `controls: all as expected`, `checks failed: 0` and `RESULT: PASS`: lines 323, 509 and 510.
- The hash equality with scan.log: lines 482, 485 and 488. Zero control bytes in G, N, R and S: lines 483-491.
- fadc53f's message: lines 494-500. The bash string: line 295.

**Logs, sha256 (draft report lines 132-140).**
- ba7100... (G): scan.log line 313; verify.log line 481.
- 4af428... (N): scan.log line 321; verify.log line 484.
- 5a6834... (R): scan.log line 329; verify.log line 487.
- c7978f02... (S): verify.log line 490; independent-check-1.md line 10.
- 952f92ea... (independent-check-1.md): verify.log line 501.
- 1913a6ef... (verify.log): in no primary, and marked.
- c722e9e8... (check.sh): scan.log line 273, verify.log line 298, and gate.log line 120.

**Findings (draft report lines 146-171).**
- `index blob of notes.txt holds the U+2014 bytes: 1`: verify.log line 389, then `PASS hygiene` at line 397.
- v4 PASS: line 413. v5 PASS: line 429.
- `FAIL hygiene: U+2014 in tracked file(s): link-outside`: line 445. v6b PASS: line 459.
- v1b PASS: line 350. v2b FAIL: line 380, where the path is abbreviated in the draft report.

**Other notes (draft report lines 175-197).**
- The 2026-09-15 UTC date: toolchain record line 1; gate.log line 2.
- `cargo test --no-run` after the gate: gate.log line 147 and script lines 48 and 76.
- The rustc extras: toolchain record lines 8-11.

### Not in a primary, and marked

- the stat string and the pre-commit scan (marked only indirectly; see below);
- "unedited", and "all pending changes";
- the roles of 1a95419 and 1c57a2a;
- verify.log's sha256;
- the dry run at d8c1ee1 and its hash;
- concurrent G and N runs and the move of their outputs;
- the P4 records on non-member features;
- the P2 binary name;
- no install, no remote, and no push.

### Unmarked or weakly supported claims

1. **Draft report lines 113-115**, "V starts only on a clean tree after both record commits for G, N, S, R and I1 (its guard exits 2 otherwise)". The guard checks cleanliness only (verify.log script line 112). "After both record commits" rests on verify.log line 3, which is fixed text. It also conflicts with the draft report's own line 26, which marks 1a95419's role as not in a primary.
2. **Draft report line 23**, "fadc53f: record commit for G, N, S and R". verify.log shows that fadc53f contains those files with the recorded hashes. That it is the commit that added them rests on fixed text: verify.log line 3 and independent-check-1.md line 2.
3. **Draft report lines 33-36.** Only "My review ... is session output only" is labelled. The quoted stat and "a pre-commit scan found 0 hits" sit under the "NOT shown by any primary" sentence but are not themselves labelled as session output.
4. **Draft report line 156**, "`grep -I` skipped it as binary". This is inference. verify.log shows PASS, but it does not measure v5's file content and does not run a comparison without `-I`.
5. **Draft report line 53**, "even to an empty string". This is shown by the diff's code (line 122), not by any run. The line is not marked as untested.
6. **Draft report lines 194-196**, "visible to cargo build scripts and tests in steps 1 to 6". This is reasoning from step order, not an observation.
7. **Note 3 under-claims.** gate.log line 169 and negative.log line 447 show a clean tree at the end of each script. scan.log (script lines 171-172; log lines 313-329) found G, N and R in the working tree at 01:22:36Z. Untracked files would have appeared in `git status --porcelain`. So the outputs were moved in after both final status checks, and the primaries partly show what the draft report calls a session fact.

### Omissions

The draft report does not say that:
- item 5 of independent-check-1.md (argument length) is still open;
- the messages of 1a95419 and verify.log's record commit are scanned by no primary;
- no primary shows the absence of other trailers on any commit;
- scan.log and verify.log get no privacy-form scan in any primary, and verify.log gets no control-byte count;
- the toolchain record's first line names no commit;
- v4 contradicts condition 2's wording rather than being only an edge case;
- the positive control's failure branch and the grep exit 2 or more branches are never exercised;
- FAIL lines print raw path bytes;
- the rest of check.sh outside the hunks (`set -euo pipefail`, `fail()`, the cd, rustfmt `--color never`) is still unseen.

Note 6's "Beyond the spec" list is also incomplete. The toolchain record adds the active toolchain line, the "No toolchain or target was installed" sentence and the guard line (lines 4, 13 and 17). The native pass label lists `_sys` names under "-sys".

The draft report is honest about what it marks: the doc-commit state, the dry run, concurrency, the push state and verify.log's hash. The edge cases are correctly described as observed and not fixed.
