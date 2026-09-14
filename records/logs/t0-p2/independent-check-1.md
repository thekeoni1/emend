Checked phase: T0 P2 (specs/0001-bootstrap.md)
Checked commit: c829494a05b78abcdfe4cf8337bd9b4e2505c94f (P2 phase commit); logs in 3965842b981f6bcfd0f8df32cdca80b71262196d
HEAD when the check ran: 3965842b981f6bcfd0f8df32cdca80b71262196d
Checker: a fresh general-purpose subagent, not a fork
Inputs (sha256):
- specs/0001-bootstrap.md: 9ebafdbe34163452234179bbe9fb784210c833c97700952c48adc4e46b920886
- <scratch>/p2-diff.txt (git diff 88d56443816d3cc747eadea8272ad04b1d0b1c40..c829494a05b78abcdfe4cf8337bd9b4e2505c94f): 1ae5f01ef12685c250965e8594a8469a9702ae7b25c3bb98367256083deb81b4
- records/logs/t0-p2/build.log: 416603aff00f5f30c6000f05054f4e8e0a71e8375771bcab07968c1a89c84a61
- records/logs/t0-p2/binary.log: 908b998afe38551bc414a279e18cb2b4d9e0ee986f25c4c50b15fc3473618411
- records/logs/t0-p2/ignore.log: fccf7d762972d173e5f76105b72c1f20485d831a82f004b56b15ea9655f67236
- records/logs/t0-p2/scan.log: e3de68aed0e6babbc3c50d3ffeede6ee0d8880441c16e31c4ffc6fed550d007a
- <scratch>/p2-report-draft.md (draft 1): 63f70ac3c807aea9886a9f8e1615f0fbd9ac5c071c65f369c73d1acd6e8bfefe
---
# T0 P2 independent check 1

Context note: the harness put several things into my context besides the seven inputs. They are a copy of CLAUDE.md, the memory index for this project, a git status snapshot with recent commit subjects, the user's email address, and a commit-attribution reminder. I did not use any of them as evidence. Everything below comes from the spec (specs/0001-bootstrap.md), the diff, records/logs/t0-p2/build.log, records/logs/t0-p2/binary.log, records/logs/t0-p2/ignore.log, records/logs/t0-p2/scan.log and the draft report. I ran no commands.

## Q1. Does the diff do what the P2 spec says and nothing else?

PARTLY. Every P2 deliverable is present and matches the spec text. The diff also adds some things the spec does not ask for. The unit test checks less than the spec's wording suggests. Several properties cannot be shown by the primaries at all.

Deliverable by deliverable:

- **Cargo.toml** (diff lines 37-43). It has `[workspace]`, `resolver = "3"` and `members = ["tools/xlsx-diff"]`. `[workspace.package]` has `edition = "2024"` and `license = "MIT OR Apache-2.0"`. There is no `[package]` table, so it is a virtual workspace. This matches spec lines 214-217.

- **LICENSE-MIT** (diff lines 258-278). The copyright line `Copyright (c) 2026 emend contributors` is at diff line 260. The body reads as the common "MIT License" text with a title line. The primaries hold no reference copy, so "standard MIT text" rests on the checker's reading only. scan.log lines 193-199 show 0 U+2014 and 0 non-ASCII hits, as spec line 221 requires.

- **LICENSE-APACHE** (diff lines 49-251, hunk header `+1,202`).
  - The text as rendered reads as the Apache License 2.0, including the leading blank line and the APPENDIX. scan.log lines 186-192 show 0 U+2014 hits.
  - The primaries CANNOT show that it is the unmodified text:
    - No primary holds a reference copy or a reference hash.
    - A rendered diff does not reveal trailing whitespace or line-ending differences.
    - The blob id `d645695` (diff line 46) and the sha256 `cfc7749b...3d30` (scan.log line 186) are never compared with anything upstream.
  - From memory, I believe the sha256 in scan.log is the commonly cited hash of apache.org's LICENSE-2.0.txt. That is recollection, not a primary. Planning should compare it against a fetched reference.

- **rust-toolchain.toml** (diff lines 300-302). It has `channel = "1.98.1"` and `targets = ["i686-unknown-linux-musl"]`, which matches spec lines 222-223. build.log line 185 shows rustup using this file as the override.

- **tools/xlsx-diff**
  - **Crate manifest** (diff lines 377-382). It has no `[dependencies]` table, and Cargo.lock (diff lines 24-30) lists only `xlsx-diff`. So "no dependencies" is shown.
  - **main** (diff lines 404-408). It calls `run()`, prints the usage string to stderr and returns `ExitCode::from(status)`, where `status` is `EXIT_ERROR = 2` (diff line 397). It ignores its arguments, which is acceptable for a skeleton.
  - **What the primaries can and cannot show about main.** They show this behaviour only by reading the code. No primary runs the binary or records its stderr or exit status.
  - **The unit test** (diff lines 414-419). There is one test, `prints_usage_and_exits_2`, as the spec requires.
    - It asserts that `run()` returns a message starting with `"usage: xlsx-diff "` and status 2.
    - It does not assert that `main` prints anything, and it does not check the process exit status.
    - Spec line 225 says the test asserts "that behaviour", meaning main prints usage and exits 2. The test covers that only indirectly, through the helper.

- **.gitignore** (diff lines 7-17).
  - It has `/target`, the seven extensions, and `!**/tests/fixtures/**`, which matches spec lines 227-229.
  - Adversarial note, not a spec violation: the patterns are lowercase. On a case-sensitive checkout, files such as `X.XLSX` or `Y.Xlsm` would not be ignored. The corpus privacy rule makes that worth a ruling.
  - Beyond the three paths in ignore.log, nothing shows the other five extensions being ignored. Only the pattern text does.

- **Doc-commit ruling** (spec lines 141-143).
  - scan.log lines 150-151 show two commits in the range. `1a985a1` has parent `88d5644`, and `c829494` has parent `1a985a1`. So a commit sits before the P2 commit.
  - The diff is a single combined range diff. It cannot show:
    - which files each commit contains;
    - that `1a985a1` holds exactly the ROADMAP.md and specs/0001-bootstrap.md changes and nothing else;
    - that `c829494` does not also touch those files;
    - that the doc changes were committed unedited, because no primary records the working tree before the commit.
  - What the primaries do show is that scan.log covers the doc files (lines 200-206 and 214-220) and the doc commit's message (lines 246-253).

**Files and content the spec does not ask for:**

1. `Cargo.lock` (diff lines 18-30). The draft report flags it.
2. `publish = false` and `version = "0.1.0"` in tools/xlsx-diff/Cargo.toml (diff lines 379 and 382).
3. The `run()` helper, the `USAGE` and `EXIT_ERROR` constants, and the doc comments in main.rs.
4. A comment line in .gitignore (diff line 9).

All of these are harmless. Item 1 needs a ruling.

The ROADMAP.md and specs/0001-bootstrap.md changes are covered by the doc-commit ruling. Their content matches the spec as it now stands: for example, the status at spec lines 3-5 and the P1R rulings at spec lines 112-143.

**Missing:** nothing from the P2 list.

## Q2. Do the logs contain every line the P2 spec requires?

PARTLY. Every required log exists and every acceptance check passes in the logs. But one log line is mislabeled and contradicts its own output. Several rule-conformance points are weak or not shown.

**Logs list (spec lines 230-239):**

- **build.log.** It has the commands and output for all four required steps:
  - host build: lines 195-199;
  - host test: lines 201-211;
  - i686 build: lines 213-217;
  - i686 test: lines 219-229.

  Each step is `exit: 0`. The extra `cargo clean` step (lines 190-193) is not required but strengthens the evidence. Stdout and stderr are merged through one file (script line 37), so "full output" is plausible, though cargo's stream order is not guaranteed.

- **binary.log.** The `file` output is at line 6. The binary path matches build.log line 222. `file` does not say "musl"; only the target triple in the path and "statically linked" point to it. That is acceptable.

- **ignore.log.** `git check-ignore -v` output is present for all three paths:
  - `a.xlsx`: lines 5-7;
  - `sub/b.xlsm`: lines 12-14;
  - `tools/x/tests/fixtures/c.xlsx`: lines 19-21.

- **scan.log.** It is present and covers:
  - every file changed in the range, at the phase commit (lines 153-235);
  - both commit messages (lines 237-254);
  - the other three logs (lines 256-278).

**Acceptance (spec lines 240-242):**

- build.log shows `running 1 test` and `test result: ok. 1 passed` for the host (lines 206 and 209) and for i686 (lines 224 and 227). PASS.
- binary.log line 6 says `ELF 32-bit LSB executable, Intel i386`. PASS.
- ignore.log lines 10, 17 and 24 give ignored, ignored and not ignored, each `-> as-expected`. PASS.

**Scan rules (spec lines 62-73):**

- `#!/usr/bin/env bash`: yes. scan.log line 6; the build.log script at build.log line 6 also has it.
- `/usr/bin/grep` called explicitly: yes, through `G=/usr/bin/grep` (scan.log line 15) and `"$G"`. build.log lines 81-82 call `/usr/bin/grep` directly.
- `LC_ALL=C` byte matching: yes. Every scan function sets it (scan.log lines 22-27).
  - Non-ASCII uses `-P '[^\x00-\x7F]'` (line 22).
  - U+2014 is `printf '\342\200\224'`, which is octal for E2 80 94 (line 18), used with `-F` (line 23).
  - `\x{2014}` is not used anywhere.
  - Weak: the build.log summary greps (build.log lines 81-82) do not set `LC_ALL=C` and have no zero-tests control. They check cargo output rather than scanning for content, so this is minor. P3 will need the zero-tests case anyway.
- Exit codes: yes. `run()` counts hits only on exit 0 and treats 2 or more as an error (scan.log line 34). Every result line prints its exit code, and the log shows `grep or read errors: 0` (line 281). `local n rc` is declared before the assignment, so `$?` is not masked.
- Positive controls in the same form: yes per form (lines 133-139), with clean-file negatives for all six forms (lines 140-145). Weak: two forms hold several patterns, but their controls each exercise only one.
  - `f_link` has 2 patterns. Its control (script line 58) matches only `claude[.]ai/code/session`, not `session[_][A-Za-z0-9]{20,}`.
  - `f_p6` has 7 patterns. Its control (script line 59) matches only `/tmp/claude[-]`. The `/mnt/[a-z]/`, `DESKTOP-`, `gh[pousr]_`, `github_pat_`, `sk-ant-` and `BEGIN ... PRIVATE KEY` patterns have no positive control.
- Exact script printed: yes, by `sed ... "$0"` (scan.log line 48 and build.log line 50).
  - Weak: build.log's toolchain sections 1-3 come from three input files (build.log script lines 8-10 and 14-17). Nothing in a log shows how those files were produced, so their provenance rests on the session's word.
  - Labeling inconsistency: the script calls input 1 an "auto-install extract" (line 16), but the log text says "rustup printed:" (line 128). That presents an extract as if it were the full output.

**Log privacy (spec lines 74-84):**

- No hostname: `f_host` shows 0 hits on all three other logs (scan.log lines 263, 270 and 277), with a working control (line 139). Weak: it matches whole words only (`-w`) and only the short `hostname` output.
- No `uname -a` output: none is visible.
- No temporary-session paths: `<scratch>` is used (build.log line 3 and scan.log line 3). `f_p6` covers `/tmp/claude-` with 0 hits.
- `/home/dev/emend/...` paths appear at build.log lines 185, 197 and 215. They are allowed by P1R ruling Question 2 (spec lines 130-132).
- NOT shown: no primary scans any log against the private denylist. The spec says logs must never contain a denylist entry (line 82). For P2 that holds only if planning's own scan confirms it.
- Tokens and keys are partly covered by `f_p6`, whose token patterns have no positive control (see above).
- scan.log does not scan itself. That was accepted in the P1 rulings (spec lines 99-102).

**First-line rule (spec lines 24-25):**

- build.log, binary.log and ignore.log each name `c829494...` and `HEAD at run: c829494...` on line 1.
- scan.log line 1 names the range `88d5644...c829494...` and the HEAD. This names a range rather than a single commit; I accept that as satisfying the rule.

**Mislabeled, inconsistent or weak:**

1. **ignore.log line 3 contradicts the log's own output.** It says check-ignore "exits 0 when the path is ignored and 1 when it is not". Lines 19-21 then show `git check-ignore -v tools/x/tests/fixtures/c.xlsx` matching the negation `!**/tests/fixtures/**` with `exit: 0`, for a path that is not ignored. The verdicts come from a `-q` call (build.log script line 117), but that call is never printed as a command and its exit code is never logged. The verdict line is therefore the script's interpretation of an unlogged exit code. The draft report flags the mislabel. The `--no-index --non-matching` calls, which the spec does not ask for, also have no exit code printed.
2. **No log records the build.log script's final `exit "$fail"`.** Failures found in binary.log or ignore.log after the summary was written would not appear in the summary (build.log line 234). Here all the verdicts are good, so this is a record gap rather than a failure.
3. **The "run at" time is not the actual run time.** binary.log and ignore.log reuse the start time `2026-09-14T23:39:44Z`, although they were written after build.log's steps. Minor.
4. **The scanned log files may not be the committed ones.** scan.log hashes the three logs as they were in the working tree at 23:40:07Z (lines 257, 264 and 271). No primary shows those hashes equal the committed files.

**Failed checks:** none found.

## Q3. Does every number and quoted string in the draft report appear in a primary?

PARTLY. Most numbers and quotes trace to a primary line. A few do not: they are misstated, paraphrased as if quoted, or unmarked session claims.

**Header**

- The range `88d5644...c829494...` is in scan.log lines 1, 149 and 154. The report says the diff file does not state its range, which is true.
  - The diff's 10 files match scan.log lines 155-164 in names and in A/M status.
  - That corroborates, but does not prove, that the diff is that range.

**Commits**

- `1a985a1de2284b45266850dac5c8baf20f9ef2e2` and its parent `88d5644` are in scan.log line 151.
  - "planning's doc updates (ROADMAP.md, specs/0001-bootstrap.md)" is in NO primary. scan.log lists files for the range only, not per commit, and the report does not mark this.
  - "Its subject is not printed in any primary" is true.
- `c829494...`, parent `1a985a1`, is in scan.log line 150 and on line 1 of build.log, binary.log and ignore.log. The label "P2 phase commit" is in build.log line 1.
- `3965842b981f6bcfd0f8df32cdca80b71262196d` is in no primary, and the report marks this. Its description ("record commit adding B, Bi, I and S") is also in no primary.

**Spec item 1**

- "S's range lists 1a985a1 then c829494" is inaccurate as written. scan.log lists `c829494` first (line 150) and `1a985a1` second (line 151). The order 1a985a1-first follows only from the parent links.
- ROADMAP.md and specs/0001-bootstrap.md scanned at c829494: scan.log lines 200 and 214. Present.
- "2 files, 42 insertions, 6 deletions" is marked as session output. It is consistent with the diff: ROADMAP.md is +2/-2 (diff lines 287-290), and specs/0001-bootstrap.md is +3/-2, +34/-1 and +3/-1 across its three hunks. Consistency assumes c829494 did not touch those files, which no primary shows.
- "unedited" is marked as not in a primary.

**Spec items 2-4**

- Item 2, the Cargo.toml strings: diff lines 38-43. Present.
- Item 3, the copyright line: diff line 260. "202 lines": diff line 49. The sha256 `cfc7749b...3d30`: scan.log line 186. "0 non-ASCII and 0 U+2014 hits": scan.log lines 187-188 and 194-195. The apache.org provenance is marked as not in a primary.
- Item 4, the toolchain strings: diff lines 301-302. Present.

**Spec item 5**

- "no dependencies" and "Cargo.lock lists only xlsx-diff": diff lines 28-30 and 377-382. Present.
- `ExitCode::from(2)` in code formatting does NOT appear verbatim. The diff has `ExitCode::from(status)` (line 407), with `EXIT_ERROR: u8 = 2` (line 397). This is a paraphrase formatted as a quote.
- `prints_usage_and_exits_2`: diff line 415 and build.log lines 207 and 225. Present.
- "asserts that `run()` returns the usage text" overstates the test. The test asserts only a prefix, `starts_with("usage: xlsx-diff ")` (diff line 417).
- "B shows it passing on both targets": build.log lines 207 and 225. Present.
- Running the built binary is marked as not in a primary.

**Spec item 6**

- The .gitignore strings: diff lines 7 and 10-17. Present.

**Acceptance section**

- `running 1 test`: build.log lines 206 and 224.
- `test tests::prints_usage_and_exits_2 ... ok`: lines 207 and 225.
- `test result: ok. 1 passed; 0 failed; ...`: lines 209 and 227.
- "Every step is `exit: 0`": lines 193, 199, 211, 217 and 229.
- The summary quote: line 234, exact.
- The binary.log quote: binary.log line 6, prefix exact.
- The path match with the `Running unittests` line: build.log line 222.
- The ignore.log quotes: ignore.log lines 10, 17 and 24.
- The scan.log items:
  - `controls: all as expected`: line 146.
  - "13 controls": 13 lines at 133-145.
  - `files in diff: 10; files scanned: 10; files read: 10`: line 235.
  - `messages scanned: 2; commits in range: 2`: line 254.
  - Co-Authored-By last line: lines 239 and 247.
  - `logs scanned: 3 (expected 3)`: line 278.
  - The final totals and `RESULT: PASS`: lines 280-283.

All of these are present.

**Logs (sha256)**

- The build.log, binary.log and ignore.log hashes are in scan.log lines 257, 264 and 271. The report says "all three are in S", which is true.
  - Not marked: those are working-tree hashes taken at scan time.
  - No primary shows the committed logs have those hashes.
- The scan.log hash `e3de68ae...007a` is in no primary, and the report marks this.

**Toolchain section**

- The probe command `rustup +1.98.1 target list --installed` is in build.log lines 127 and 164.
  - "ran outside the repo without the guard" comes only from build.log lines 162-163 and 167-170, inside section 3, which is a transcription and not a machine capture (lines 144-145).
  - The report marks section 3 as a transcription in a later bullet. It does not mark the "Finding" bullet, which states these details as if B recorded them.
- The quote "the missing active toolchain `1.98.1-x86_64-unknown-linux-gnu` has been auto-installed" is at build.log line 132. Present, but it comes from a session-supplied input file, not a capture made by the script (line 126).
- The target-add command is at build.log line 141. "after the maintainer chose it" is at line 140, but only as the session's own statement; no primary independently shows the maintainer's choice, and the report does not mark this.
- `info: downloading component rust-std` is at line 142.
- The labelling of section 3 is at lines 143-145.
- The `RUSTUP_AUTO_INSTALL=0 (exported ...)` quote is at line 182.
- The target list quote is at line 188, which has a trailing space.

**Notes**

- Note 1: Cargo.lock is in diff lines 18-30. "normally committed for a binary crate" is an opinion, not in a primary.
- Note 2: the quote matches ignore.log line 3, and the `-v` exit 0 claim matches lines 19-21. "That holds for the `-q` form" is NOT shown by any primary. The `-q` call appears only in the printed script (build.log line 117), and its exit codes are never logged. The report does not mark this.
- Note 3 says the pre-flight is "NOT in a primary". That is slightly inaccurate.
  - build.log lines 176-178 mention the pre-flight in the transcription, without results.
  - `Removed 135 files, 32.8MiB total` (build.log line 192) shows that an earlier build existed.
  - The results (all exit 0) are indeed in no primary.
- Note 4: build.log lines 190-193. Present.
- Note 5 ("201 lines", "8 of 40") is marked as session output.
- Note 6:
  - "No planning doc edits were committed during P2 other than 1a985a1" is unmarked. scan.log lines 150-151 support "no other commit", but nothing shows that c829494 itself contains no planning doc edits.
  - The clean working tree and "0 remotes" are marked as session output.

**Honesty about what is not shown.** The report is mostly honest. It marks the record-commit hash, the scan.log hash, "unedited", the Apache provenance, the binary run and the pre-flight results. It leaves these unmarked:

1. which files 1a985a1 contains;
2. the literal `ExitCode::from(2)`;
3. the `-q` exit-code claim;
4. "after the maintainer chose it" and "outside the repo without the guard" as session statements;
5. that the scanned log hashes are working-tree hashes, not proven to equal the committed files;
6. "No planning doc edits were committed during P2 other than 1a985a1".

The report also leaves out several weaknesses found in Q2:

- no denylist scan of the logs;
- single-pattern positive controls for `f_link` and `f_p6`;
- the unlogged final exit of the build.log script;
- the "extract" versus "rustup printed" labeling;
- lowercase-only extension patterns in .gitignore.
