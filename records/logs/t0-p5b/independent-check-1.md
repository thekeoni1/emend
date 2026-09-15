Checked phase: T0 P5b (specs/0001-bootstrap.md)
Checked commit: aca5d3a20b9e1eb92eaf12b08ee1a98e1d4a8511 (P5b phase commit); logs in 4f696ed01ed8b35fdbd071a34a3107eb0935c4c4
HEAD when the check ran: 4f696ed01ed8b35fdbd071a34a3107eb0935c4c4
Checker: a fresh general-purpose subagent, not a fork
Inputs (sha256):
- specs/0001-bootstrap.md: e47c9f41397feac91bbbca72b222d7bfcd3f5ebe69b460332d8b7263f394e803
- <scratch>/p5b-diff.txt (git diff 9b7aef1f690a6be5fc198b326ab36b992bf5e2d7..aca5d3a20b9e1eb92eaf12b08ee1a98e1d4a8511): aa202ea2c2e7ea63540678791619b663b4b88fb4d43e9202dc4e3fcf3aade868
- records/logs/t0-p5b/gate.log: 7ab349545f714d1800dc523db2fca7cd6b39051ee894996ad4669583d5fa2724
- records/logs/t0-p5b/negative.log: 1ceeedfe33a582320b7aa12f849315c2e51edb8cc1a9f68a8176d7bcf2f9d3e0
- records/logs/t0-p5b/scan.log: cdde5f359658ce5cb6b91ef7f20e2e5b0d82f4f0804f66897997e30de9db6f49
- <scratch>/p5b-report-draft.md (draft 1): bf0e19e92984f1ce0d34f06dccd06f24d7773f1092711e2bbfcb0e18478881f9
---
# T0 P5b independent check 1

Note on context: besides the six inputs, the harness put other material into my context. That material was the repository's CLAUDE.md, an auto-memory index, a git status snapshot (branch, a clean status and recent commit subjects), user identity details, and a commit attribution reminder. I did not use any of it as evidence and I do not repeat the identity details. In particular, the git status snapshot lists a record commit hash that the draft report also gives. I treated that hash as appearing in no primary. I read only the six named inputs and ran no commands.

Line references: "D n" is line n of the diff file. "G n", "N n" and "S n" are lines of `records/logs/t0-p5b/gate.log`, `negative.log` and `scan.log`. "R n" is line n of the draft report. "Spec n" is line n of `specs/0001-bootstrap.md`.

## Q1. Does the diff do what the P5b spec and the rulings on P5 say, and nothing else?

YES, as far as the diff can show. I found no case where step 7 passes or fails against the letter of the spec. Two latent defects and several weak controls are listed below. Planning should rule on them.

### Scope of the diff

- There are three files: ROADMAP.md (D 1-13), scripts/check.sh (D 15-202) and specs/0001-bootstrap.md (D 203-337). S 247-257 shows two commits. 873851f touches only ROADMAP.md and the spec. aca5d3a touches only scripts/check.sh. That matches the P1R ruling that planning's doc updates are committed first (Spec 155-157).
- Doc commit size, computed from the hunk headers: ROADMAP.md +2 -1 (D 5). The spec has +5 -4 (D 207), +2 -1 (D 222), +46 (D 232) and +45 (D 285). In total that is 100 insertions and 6 deletions.
- The post-image of the spec hunks matches the spec I was given at Spec 3-7, 20-21, 282-326 and 559-602. That shows the committed doc text is the text of the spec input. It does not show that the text is unedited from planning's draft. No primary can show that.

### P5b bullets

1. **Index read with `git grep --cached -I` under `LC_ALL=C`.** Done at D 82: `LC_ALL=C git grep --cached --no-color -I -l -z -F -e "$EMDASH"`, run from the repo root via `cd`. Nothing in step 7 reads the working tree any more. The old `[ -f "$f" ]` filter is removed (D 52-62).
2. **Symlinks checked as their target string and never followed.** Done at D 90-114. Index entries with mode 120000 come from `git ls-files -s -z`. Each one's blob is read with `git cat-file blob` and matched with `LC_ALL=C /usr/bin/grep -q -F`. Splitting on the first tab (D 95-96) is correct, because the metadata field has no tab. The D 76 comment says `git grep` skips symlinks. That is an assertion, not evidence. N case (h) (N 349) shows only the combined result.
3. **Binary files are not text files.** Handled by `-I` at D 82. See defect A below.
4. **Control bytes in paths fail first and report only a count.** Done at D 170-174: `-z -c -P '[\x01-\x1f\x7f]'` over `git ls-files -z`. It runs before the office check, the corpus check and the U+2014 check. The FAIL text is the count with NUL and LF stripped. The class covers newline, tab and DEL. NUL cannot occur inside a git path, since it is the separator.
5. **`emend-testing` case-insensitive, `tests/fixtures/` exception case-sensitive.** Done: `-z -i -F -e emend-testing` at D 187 and `-z -v -E '(^|/)tests/fixtures/'` at D 181, with no `-i`.
6. **FAIL lines print git-quoted paths.** Done by `quoted()` at D 120-126, which runs `git -c core.quotepath=true ls-files -- ":(literal)$p"` per path. It is used for the office, corpus and U+2014 FAIL lines (D 184, 191, 200).
7. **`ANTHROPIC_API_KEY` check before step 1.** Done at D 23-28, placed before `OUT=$(mktemp -d)` (D 32). It uses `${ANTHROPIC_API_KEY+set}`, so an empty value also counts. It prints the exact spec line and exits 1, and it never prints the value. The old in-step check is removed (D 194-197).
8. **In-script positive control for every grep form.** Each form has one, and each uses the same call as the real check:
   - control byte: D 131-133;
   - office extension: D 134-136;
   - fixtures exception, with an exact-output check: D 137-141;
   - corpus: D 142-144;
   - `git grep --cached` plus the symlink `grep -q -F`, through the same `index_dash` function on a throwaway repo with a deleted staged file and a dash-bearing symlink target, exact output `text.txt link `: D 145-155.

   Exit 2 or more fails every real check: D 171, 180, 182, 188, and 198 through `index_dash` setting rc=2 at D 84-93 and 101-110.

### Real or latent defects

- **A. The binary decision depends on local, uncommitted state.** `git grep -I` decides "binary" from the `diff` attribute before it looks at content. As I understand git, attributes come from untracked working-tree `.gitattributes` files, `.git/info/attributes` and a user-global attributes file, not only from the index. So a local `* -diff` or `* binary` would make a committed U+2014 in a text file pass step 7. That matches the literal ruling ("a file git treats as binary"). It conflicts with "read what would be committed, not the working tree". No primary exercises it, and I could not test it. A NUL byte in the first bytes of any file also exempts it (N case (i)), which the ruling accepts.
- **B. The control repository is not isolated from git environment variables.** D 145-152 runs `git init`, `git -C "$CTL/repo" add` and `git grep --cached` with whatever `GIT_DIR`, `GIT_WORK_TREE` or `GIT_INDEX_FILE` the caller exported. Git exports these to hooks: a pre-commit hook gets `GIT_INDEX_FILE`. If check.sh runs in such an environment, the control's `git add text.txt link` writes into the real repository's index. P5b does not ask for this protection, and P6 runs privacy.sh from the hook, not check.sh. It is a trap for any later hook use.
- **C. Weak controls.** None of these is required beyond "one control per grep form":
  - The control-byte control uses byte 0x01, not a newline. Only N case (b) shows that `-P` with `-z` matches an embedded LF on this grep.
  - The fixtures control does not show case-sensitivity. Nothing shows that `Tests/Fixtures/x.xlsx` fails.
  - `ln -s`, `rm` (D 148, 151) and `mkdir` are unguarded. Whether `set -e` turns their failure into a silent non-zero exit cannot be seen in the diff.
- **D. Output details.**
  - `quoted()` joins paths with spaces, and git does not quote spaces, so a multi-path FAIL line with spaces in names is ambiguous.
  - In an unmerged index, `git ls-files` repeats paths, which could put an LF inside a FAIL line.
  - The third argument to `fail` passes `hygiene-err` unfiltered to stderr on error branches. That file accumulates stderr from every git and grep call in step 7, and grep errors name files under `$OUT`, which sit under TMPDIR. This was accepted by the P3 Q6 ruling. On success the file is not printed (G 93: `stderr bytes: 0`).
- **Order change.** U+2014 is now checked last, after the path checks (D 197). It used to be checked first. This is not a spec issue.

On raw bytes and path leaks: I found no route in normal operation by which a FAIL line prints a raw control byte or non-ASCII byte. Control-byte paths are counted only, and every other path goes through `core.quotepath=true`. The only unfiltered route is the `hygiene-err` dump on error branches noted under D.

### Beyond the spec

- The `hgrep` helper, `readonly` regex variables, and one shared stderr file.
- A reworded error message at D 198.

All of this is refactoring inside step 7. I found nothing outside step 7 and the key check.

### What the primaries cannot show

The diff shows only hunks. It does not show:

- `set -euo pipefail`;
- `fail()` and its prefix and exit code (N 204 and similar lines show the format);
- the `cd` to the repo root, which both `git grep` and `git ls-files` rely on to cover the whole repo;
- argument parsing;
- where step 1 begins;
- whether `--quick` runs step 7.

The hunk header at D 19 shows only that `export RUSTUP_AUTO_INSTALL=0` precedes line 23. That the key check precedes every cargo call is supported by its position before `mktemp` and by N 267 and N 274 (one output line). It is not shown from the full file.

## Q2. Do the logs contain every line the P5b spec requires?

PARTLY. Every required observation is present and as expected. One required first line is mislabeled, and several checks are weaker than they look.

### gate.log

- A full run from `/` on a cold cache (G 82-84).
- All seven PASS lines in order (G 86-92, G 95), exit 0 (G 94), stderr 0 bytes (G 93).
- Zero control bytes in stdout and stderr, with ESC/SI and tab/CR controls (G 98-101).
- Clean status afterwards (G 103-104).
- Gate file hashes at the commit (G 77-78).

**Mislabel.** G 1 reads `about: P5 phase commit aca5d3a...`, but this is the P5b phase commit. The script prints that text at G 28. N 1 correctly says `P5b phase commit`. The line still names the commit and HEAD, so the first-line rule is met in substance, but the label is wrong in a committed log.

### negative.log

Each case runs in a fresh clone at aca5d3a, and `DRYRUN_CHECK_SH is not set` (N 4).

- **(a)** Staged then deleted: `notes.txt present in the working tree: no` (N 196), `FAIL hygiene: U+2014 in tracked file(s): notes.txt` (N 204), exit 1 (N 205). Present.
- **(b)** The path is `"evil\ntests/fixtures/x.xlsx"` (N 214). `FAIL hygiene: 1 tracked path(s) contain control bytes (paths not printed)` (N 222). Control bytes 0 (N 224). Present.
- **(c)** The symlink to `<scratch>/outside.txt` gives all PASS, `PASS hygiene` (N 240), exit 0. Present. Weak: the log shows only `A link-outside` (N 232), not mode 120000.
- **(d)** `FAIL hygiene: tracked path(s) containing emend-testing: Emend-Testing/x.txt` (N 258). Present.
- **(e)** The only output line is the FAIL line (N 267). `first output line is the hygiene FAIL line: as-expected` (N 271), no `PASS host` (N 272), value absent (N 273), one line (N 274), not set in the shell afterwards (N 275). Present.
- **(f)** `FAIL hygiene: office file(s) tracked outside tests/fixtures/: "caf\303\251.xlsx"` (N 288), non-ASCII bytes 0 (N 290). Present.
- **(g)** g1 (N 304), g2 `x.xlsx` (N 318) and g3 `Y.XLSM` (N 332) each FAIL. Present.
- **Extra cases** (h) and (i) are labeled as beyond the spec (N 338, 355).
- **Ending:** clean main status (N 373) and `all negative cases as expected: yes` (N 374).

### scan.log

- Printed script and `/usr/bin/grep` (S 191).
- Controls for every form, including U+2014 bytes, control bytes, non-ASCII, each P6 pattern, host and each denylist entry (S 197-243).
- The doc commit's files and messages are covered (S 250-308), as the P1R ruling requires.
- Both other logs are scanned (S 312-328).
- `RESULT: PASS` (S 333).

### Weaknesses and inconsistencies

- **Counters with no control or error detection.** negative.log's `ctrl_bytes` and `nonascii_bytes` counters (N 30-31) use `tr | wc` with no positive control in negative.log and no error detection: a failed `tr` reads as 0. The same holds for `ctrl_bytes` in gate.log, which does have a positive control (G 98). The requirements of (b) and (f) are therefore backed properly only indirectly: S 321 (`f_ctrl`) and S 313 (`f_nonascii`) find 0 in negative.log with controls, and the printed case output is the captured output after path substitution.
- **Case (i)** shows no evidence of git's binary decision other than the PASS.
- **Timing.** gate.log and negative.log both record a start of `2026-09-15T01:56:59Z` (G 2, N 2). scan.log starts at `01:57:30Z` (S 2) and hashes both logs as complete files. That implies one cold full gate plus 11 cloned gate runs finished within 31 seconds. Neither log records an end time. This is possible for a crate with no dependencies, but the primaries cannot confirm that S hashed the final files. Planning should compare the sha256 values in S 312 and S 320 with the committed files. The check file header will list them.
- **Toolchain rule.** Met in the printed scripts (G 17, N 21). check.sh's own export is visible only as hunk context (D 19).
- **Log privacy.** Scratch paths are written as `<scratch>`. The host, P6 pattern and denylist scans are 0 on both logs (S 312-327). scan.log does not scan itself, which the P1 rulings accept.

## Q3. Does every number and every quoted string in the draft report appear in a primary?

PARTLY. Almost every item is in a primary. The report marks most of the exceptions, but it has several unmarked or inaccurate claims and one omission.

### Items found in a primary

**Commits and hashes**

- Range `9b7aef1f...`..`aca5d3a2...` (R 9): S 1, S 246.
- 873851f with its subject, parent 9b7aef1, and `M ROADMAP.md` and `M specs/0001-bootstrap.md` (R 14-15): S 248-253.
- aca5d3a with its subject, parent 873851f, and `M scripts/check.sh` (R 16-17): S 247, S 255-257.
- sha256 `19345cf7...` (R 18): S 272, G 77.

**Code quoted from the diff**

- `FAIL hygiene: ANTHROPIC_API_KEY is set in the environment` and exit 1 (R 34): D 26-27.
- The control-byte form (R 39): D 132. The input is written `plain\0bad\001path\0`, but D 131 has `plain\000bad\001path\000`. That is equivalent but not verbatim.
- The office, fixtures and corpus controls, `x/b.XLSX`, `b.xlsx` and `docs/Emend-Testing/x.txt` (R 40-44): D 134-143.
- The control repo and the two paths (R 45-49): D 145-155.
- `git ls-files -z` and the control-byte FAIL text (R 50-53): D 165, D 173.
- `LC_ALL=C git grep --cached --no-color -I -l -z -F` (R 58): D 82.
- Mode 120000, `git cat-file blob` and `LC_ALL=C /usr/bin/grep -q -F` (R 60-62): D 97, 101, 106.
- The `git -c core.quotepath=true ls-files` form (R 64): D 123.
- Exit 2 or more fails (R 66): D 171-198, as reviewed in Q1.

**gate.log lines (R 70-77)**

- The seven PASS lines: G 86-92.
- `exit: 0 (expect 0)`: G 94.
- `stdout is exactly the seven PASS lines in order: yes`: G 95.
- `stderr bytes: 0`: G 93.
- `stdout: lines=0 bytes=0` and `stderr: lines=0 bytes=0`: G 100-101.
- `cargo clean`: G 82-83.

**negative.log lines (R 78-104)**

- `DRYRUN_CHECK_SH is not set`: N 4.
- `control lines=0 control bytes=0 non-ASCII bytes=0` appears in all 11 cases: N 206, 224, 242, 260, 269, 290, 306, 320, 334, 351, 368.
- (a): N 196, N 204.
- (b): N 214, N 222.
- (c): N 234-241.
- (d): N 258.
- (e): N 266-274.
- (f): N 280, N 288, N 290, and `git add -f` in the script at N 140.
- (g): N 304, 318, 332.
- (h): N 349.
- (i): `blob.bin`, PASS and exit 0 at N 358, 366-367.
- The ending: N 373-374.

**scan.log lines (R 106-112)**

- `controls: all as expected`: S 243.
- "46 control lines": S 197-242 has 46 lines. Correct.
- `files in diff: 3; files scanned: 3; files read: 3`: S 288.
- `messages scanned: 2; commits in range: 2`: S 309.
- Both messages end with Co-Authored-By: S 292, S 301.
- `logs scanned: 2 (expected 2)`: S 328.
- The totals and `RESULT: PASS`: S 330-333.

**Log hashes**

- G `7ab34954...` and N `1ceeedfe...` (R 116-117): S 312, S 320.

### Items in no primary, marked as such

- The record commit hash `4f696ed0...` (R 20).
- The scan.log sha256 `cdde5f35...` (R 120).
- The pre-commit scan result (R 27).
- The throwaway `git grep` symlink test (R 129-132).
- The dry run at 873851f (R 133-136).
- That the committed logs match the scanned hashes (R 118-119).
- No install, no remote and no push (R 152-153).

### Inaccurate, unmarked or undersold

- **The stat line undersells the diff (R 27).** `2 files changed, 100 insertions(+), 6 deletions(-)` is marked session-only, but the numbers follow from the diff's hunk headers for the two doc files (see Q1). Only the "before `git add`" timing is session-only.
- **R 32-33** says the key check comes "after argument parsing, the `cd` and the two exports". The diff shows only the exports (D 19 hunk context and D 21). Argument parsing and the `cd` are not in any primary, and the claim is not marked.
- **Note 5 (R 139-140)** marks only that no primary exercises the bad-argument path. The ordering it relies on is also not shown.
- **Note 6 (R 141-142)**, that git paths cannot hold a NUL, is reasoning and is not in a primary. It is unmarked, but it is harmless.
- **Note 7 (R 143-144) is mislabeled.** It lists "U+2014 hits printed git-quoted like the other FAIL lines" as beyond the spec. The P5b bullet "FAIL lines print paths the way `git ls-files` prints them" (Spec 573-574) covers every FAIL line, so this is required, not extra.
- **R 150-151 is inaccurate.** It says "The diff does not show check.sh's own export line". D 19's hunk header shows `export RUSTUP_AUTO_INSTALL=0` as context before line 23. "Unchanged since P5" is shown only for this diff range.
- **Note 4 (R 137-138)** is called a "Session fact", but the simultaneous start is in fact in G 2 and N 2. The finish order is not.
- **R 102 (case (i))** says git treats the file as binary. That is an inference from the PASS and the N 176 heading, not a separate observation.
- **R 82-86** cite case (b)'s single FAIL line. The report does not mention that case (b) cannot show the office check would also have caught the path.

### Omissions

- The report does not mention the G 1 mislabel ("P5 phase commit"), which the spec's "anything that looks wrong" item (Spec 47-49) would require.
- It does not mention the lack of a control for the `tr | wc` counters in N.
- It does not mention the defects in Q1 (local attributes affecting `-I`, no isolation of the control repo from git environment variables).

Otherwise the report is careful to separate primary evidence from session output.
