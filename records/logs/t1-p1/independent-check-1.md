# T1 P1 independent check 1

- Checked commit: 2df5f6e5c6d872a26f6f747b53bd7fd8467570c3 (T1 P1 phase
  commit)
- HEAD when the check ran: 4d17a68faca4c35e66887a6b85b75551307a95ee
  (T1 P1 record commit with the four logs, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    b0ca4ed8f884bd62ac33ed1ba18f864ffc96ecd90eaeadf6470de554e6437ca2
  - specs/0001-bootstrap.md
    46b0d7ff243e37af89ccf2384106700171b4898230b8802e52b903ed161418a0
  - the phase diff, `git diff 62c4e80..2df5f6e`, saved as <scratch>/t1p1-diff.txt
    023829d959c4b9b655e52cfc1a07756f298cf63bd9f02449fffd129514c7bbf0
  - draft report 1, <scratch>/t1p1-report-draft.md
    98c3d1374458602020a28a6ddb1a7dc87fbb645bf5f950ad067e806cec4e52c7
  - records/logs/t1-p1/gate.log
    fa0891d385cfd2c7d7ad7f8c8323b79cce1231bdb8e339d8b641417df0958229
  - records/logs/t1-p1/test.log
    f3d8adf6c3ed4fa85926a102e47503997c62db58e95167c73693b4c62a9ad2ad
  - records/logs/t1-p1/negative.log
    e46f868b13803b2cca7d8b891ebc033c623cde2f900282fbd7ce9cc6eb253ed5
  - records/logs/t1-p1/scan.log
    01916231dc9321244ff828543a989041412ebd692ebcc2207573e393d371de3b

---
The harness put a CLAUDE.md into my context. I did not use it as evidence. I read only the listed files, did not run git, gh or cargo, and changed nothing. (I did run awk, tr and sha256sum over the listed files to count lines, bytes and hashes.)

## 1. Does the diff do what specs/0004 P1 says, and nothing else?

It mostly does. Everything on the checklist is present, with one undeclared extra change and some small defects listed below.

**Checklist items that match:**
- **Dependency:** `tools/xlsx-diff/Cargo.toml` adds only the exact `zip` line (diff line 718), with no dev-dependencies.
- **Cargo.lock:** it gains exactly ten packages (cfg-if, crc32fast, equivalent, flate2, hashbrown, indexmap, memchr, typed-path, zip, zlib-rs) plus the xlsx-diff dependency block. Versions match the spec's Facts list.
- **Diff size:** my own count of the diff file gives 10 files, 1539 insertions and 42 deletions.
- **lib.rs:** `pub fn run(&Path, &Path, &mut dyn Write, &mut dyn Write) -> u8` is present.
- **Escaping:** the backslash arm comes before the 0x20 to 0x7e arm, and other bytes print as lowercase `\xNN`. This is correct.
- **Entry rows:** input index order first, then unmatched output entries. The format string (diff line 1019) fills columns 1 to 13 and leaves 14 to 17 empty. The method comes from `to_u16`.
- **Statuses:** `identical` needs method, CRC, both sizes and a raw `by_index_raw` comparison through two 64 KiB buffers. Otherwise the status is `differs`; `dropped` and `added` are also there.
- **Archive rows:** the P1 set is present; the order is disclosed as departure 2.
- **Exit status:** 0, 1 and 2 are implemented as the spec says.
- **Sizes:** they are `u64`, and the central-directory position is `u64` with `checked_add`. Indices are `usize`, which is fine for indices.
- **main.rs:** exactly two arguments, otherwise the usage line on stderr and exit 2.
- **Tests:** every required test exists (11 in total). The escaping unit test covers 0x00, 0x7f, 0xc3 and backslash, and the tool-level test covers 0x00, 0x7f, 0xc3 0xa9 and backslash.

**Leaks and error paths:**
- No path or entry content can reach stdout or stderr. Error variants carry only the side, `io::ErrorKind` and escaped names, and the missing-file test asserts that no path appears.
- No read error can come out as `identical`. Errors in `raw_equal` map to exit 2. A wrong central-directory offset or count fails safe with exit 2.
- No `usize` truncation of a size or offset matters on i686.

**Findings:**
- **a. Extra, undeclared change: file modes.** `tools/xlsx-diff/Cargo.toml` and `src/main.rs` go from 100644 to 100755 (diff lines 707-708 and 1102-1103). `src/lib.rs` and `tests/p1.rs` are added as 100755 (lines 720 and 1162). The spec does not ask for this and the report does not mention it.
- **b. main.rs can print two stderr lines.** If `run` has already written an `error:` line and the stdout flush then also fails (for example, stdout closed), main.rs writes a second `error:` line. The spec says one line.
- **c. Short raw streams are not checked.** `raw_equal` never checks that it read the declared compressed size. If both sides end early at the same point with equal bytes, the entry reads as `identical` instead of exit 2. This only happens with a malformed archive whose declared data runs past the end of the file, and I did not check whether zip 8.6.0 lets that through. Low risk.
- **d. Test coverage gaps (not spec violations).**
  - No test gets `differs` from the raw-byte comparison alone. In the recompression test the compressed sizes differ, so the raw comparison never runs; only negative case (a) covers it.
  - No raw stream longer than 64 KiB is compared, so the second pass of the buffer loop is never run.
  - The duplicate-as-output case does not assert that stdout is empty.

## 2. Do the logs contain every required line and follow the log rules?

**Required lines: all present.**
- **gate.log:** the dependency tree is xlsx-diff plus the ten crates, with the root printed as `<repo>` (lines 110-121). The cold-cache removal is at line 125. The eight PASS lines are in order, including `PASS native (allowlisted -sys: none)` and `PASS licences` (128-135). Then exit 0, 0 stderr bytes, and "all as expected: yes" (148). The tree is from the host only; the Facts mention both targets, but P1 asks only for the workspace tree.
- **test.log:** host and i686 runs, each with test names and summary lines, 11 tests on each. Same names (134), both counts stated (133), equal (135).
- **negative.log:** case (a) shows byte offset 597 before and after, and the rows (218-231). Case (b) shows 1154 bytes, the `error:` line, exit 2 and no rows (242-251).
- **scan.log:** the script is printed, it calls `/usr/bin/grep`, there is a positive control for every form, errors are not counted as zero hits, 10 files, 2 messages and 3 logs are scanned, and it ends with RESULT: PASS.

**Rules followed:**
- Each log's first line names the commit and HEAD.
- All four logs contain 0 control bytes and 0 non-ASCII bytes (my own `tr` count).
- My sha256 of gate.log, test.log and negative.log matches what scan.log lists.
- There are no session links, temp session paths or hostnames.
- The denylist location appears only as an unexpanded home variable (gate.log line 5, scan.log line 20).
- scan.log line 140 holds the fictional control string that the report mentions.

**Rule breaches and script bugs:**
- **e. Fixture text in negative.log.** The printed script contains the fixture content phrase (lines 36 and 86-88). This conflicts with "No fixture text may appear". Line 4's "values never printed" is true only of the two random sentinels. The text is fictional, so nothing private leaks.
- **f. One check does not test what its label says.** At negative.log line 171, `$(is [ ... ] && env ... grep -q '^error: ' ...)` captures only the output of `is`. So "stderr is one line starting with error: " actually checks only the line count. The literal stderr at line 246 does start with `error: `, so the outcome is still visible.
- **g. One count is evaluated twice.** At negative.log line 156, the byte count from `cmp -l` is worked out once for the label and again for the result, against the rule from the P7 rulings. test.log lines 56 and 58 do the same with `wc -l`, which is minor.
- **Observation, not a breach:** test.log and negative.log both started at 16:35:24Z, so they ran at the same time.

## 3. Does every number and quoted string in the draft report appear in a primary?

Almost all of them do:
- The diff counts (10 files, 1539 insertions, 42 deletions), marked as session output, match my count of the diff file.
- The log hashes and commit hashes are in scan.log.
- The file lists match.
- Every G, T, N and S line reference I checked is right: G 110-148, T 95/98/124/127/133-137, N 4/201-254, S 237-238/301/426/449/479-484.
- The usage text, error strings and test descriptions all match the diff.

Items not in any primary:
- **Marked as session output:** the first gate run's start time (16:35:00Z) and "the only other difference" between the two runs; the 10-byte-prefix check giving 73; that doc commit 8356f9a was committed without editing. Only the fix comment at G 56-58 is in a primary.
- **Not marked:** departure 9 says line 140 is "the same line as in the committed T0b P1 and P2 scan logs". That is not in any primary I was given.

Overstatements and omissions:
- Departure 4, "This is stricter than specs/0004, which allows an escaped entry name", reads as if entry names are never printed. The code does print escaped names in the duplicate and entry errors (diff lines 806-815). What is true is only that no paths and no zip crate text are printed.
- "What changed (D)" leaves out the mode changes (finding a).
- "Findings for planning: None" leaves out findings e and f.

## Defects, ranked by severity

None of these could leak private data or let a push through.

1. **Undeclared file-mode changes to 100755** on four files (finding a). This is a diff beyond the spec and is not reported.
2. **negative.log check that does not test what it says** (finding f). The "starts with error:" part is never tested; the result is still visible in the literal output.
3. **Fixture text in negative.log's printed script** (finding e), against "No fixture text may appear", together with the inaccurate "values never printed" on line 4. The text is fictional.
4. **main.rs can write two `error:` lines** when `run` has failed and the stdout flush also fails (finding b).
5. **Report wording:** departure 4 is misleading, departure 9 has an unsupported claim, and the findings section leaves out items 1 to 3.
6. **Double evaluation of a count** at negative.log line 156 (and the `wc -l` in test.log), against the evaluate-once ruling (finding g).
7. **Raw comparison does not check that it read the declared compressed size** (finding c). Only malformed archives are affected, and I did not confirm the zip crate lets them through.
8. **Coverage gaps (not spec violations):** no test of `differs` from the raw bytes alone, no raw stream longer than 64 KiB, and the duplicate-as-output case does not assert empty stdout (finding d).
