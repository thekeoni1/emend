# T1 P1 independent check 2

- Checked commits: 2df5f6e5c6d872a26f6f747b53bd7fd8467570c3 (T1 P1 phase
  commit 1) and dc22f78167f188441098c94d982a94f031635ea5 (phase commit 2,
  the fixes for check 1)
- HEAD when the check ran: d8b30e7079a1dd04401bfad2aa38c4a25a0ad378
  (record commit with the logs rerun at dc22f78, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    b0ca4ed8f884bd62ac33ed1ba18f864ffc96ecd90eaeadf6470de554e6437ca2
  - specs/0001-bootstrap.md
    46b0d7ff243e37af89ccf2384106700171b4898230b8802e52b903ed161418a0
  - phase diff 1, `git diff 62c4e80..2df5f6e`, saved as <scratch>/t1p1-diff.txt
    023829d959c4b9b655e52cfc1a07756f298cf63bd9f02449fffd129514c7bbf0
  - phase diff 2, `git diff 7577304..dc22f78`, saved as <scratch>/t1p1-diff2.txt
    694b5d4946458ac0c1dc0086d8901100269ffb824dfe7952ddd0742adcda5466
  - draft report 2, <scratch>/t1p1-report-draft2.md
    973b0763e6ecd740cac8116323eaee2138d09c060cb885602fe20dc67378af44
  - records/logs/t1-p1/gate.log
    fd6abfc25059306bd49d1239ea37e66b55a041bb05467f61a855c5ddf2d8f907
  - records/logs/t1-p1/test.log
    eeb26a7c0d9774386b55a41a478e46296570b8085ef50ca6e0fc8d3683dc02ca
  - records/logs/t1-p1/negative.log
    87df7fca6a27240c5e5a1068429fa77b51f208a46e75e3c05229099361570760
  - records/logs/t1-p1/scan.log
    3d15bd1d17baa5965dabfbc24c7b3514e675dd3c351a2262495c596dc55d8605
  - records/logs/t1-p1/independent-check-1.md
    b6825ec07cd73ec8468dfb336a0d6a0d8fb43073e74340441b2c60cdfb53ebdb

---
My context does contain a CLAUDE.md from the harness, plus a user auto-memory index. I did not use either as evidence. I read only the seven listed inputs. I ran no git, gh or cargo command and changed nothing. On the listed files I ran sha256sum, tr and grep to check hashes, count bytes and find patterns, and ran hostname once so I could search the logs for it without printing it.

## 1. Do the two diffs do what specs/0004 P1 says, and nothing else?

**Yes.** Diff 2 fixes every code defect check 1 raised, and I found no new correctness bug against the spec.

**Scope**
- **Dependency:** D1 line 718 adds the exact `zip` line and no dev-dependencies.
- **Cargo.lock:** D1 lines 1-91 add only the ten crates from the Facts list, at the listed versions, plus the xlsx-diff dependency block.
- **Planning's doc commit:** D1 also carries it (ROADMAP.md, specs 0000, 0003, 0004, 0005). The report discloses this.
- **Diff 2:** it touches only the four tools/xlsx-diff files. It has 98 insertions and 4 deletions (93 `+` lines with text and 5 empty `+` lines), which matches the report's session-output count.
- **File modes:** the only mode lines in D1 are the four 100755 lines (706-708, 720, 1102-1103, 1162); the two spec files are new at 100644. D2 lines 1-3, 4-6, 47-49 and 66-68 set all four files back to 100644. No file is left at 100755.

**Check 1's code defects**
- **b (two error lines):** fixed. D2 lines 58-62 print the flush error only when `run` did not already return EXIT_ERROR, and the exit status still becomes 2. Correct.
- **c (short raw stream):** fixed, correctly.
  - D2 lines 30-43 add a `u64` running total per side. Any read shorter than a full buffer whose total differs from the declared compressed size is an error, and this is checked before the byte comparison.
  - A stream that is too long is also caught: its last, partial read gives a total above the declared size.
  - An empty entry and an entry that is an exact multiple of 64 KiB both pass correctly.
  - Both declared sizes are already equal when this code runs, because the raw comparison happens only when the metadata matches.
  - No test makes this error path fire, and the report's "fixed in D2" does not say so. This is a coverage gap, not a spec violation.
- **d (coverage):** fixed.
  - The new test `raw_bytes_alone_differ_across_several_buffers` uses a stored 200000-byte entry (four reads). The equal case runs every buffer.
  - A byte flipped at data offset 150000, in the third buffer, gives `differs`, with a precondition that the metadata is unchanged (D2 136-140). So only the raw comparison can produce it.
  - The duplicate-as-output case now asserts empty stdout (D2 line 76).

**Errors reported as a result**
- Every `raw_equal` read failure and short stream returns an error, so exit 2.
- `open` calls `by_index_raw` on every entry before any row is written.
- A mismatch in the central directory count or names is an error. The duplicate check runs before the count check, so a collapsed duplicate is named correctly.
- When the metadata differs, the status is `differs` without reading the entry. P1 allows this, because it does no decompression.

**Leaks**
- Rows carry only escaped names and metadata.
- Error text is made only of the side, the `io::ErrorKind` wording and escaped names (D1 795-819).
- No file path, zip crate text or entry bytes reach stdout or stderr, and I found no panic that formats content.

**i686**
- Sizes, the compressed-size totals and the central directory position are `u64`, and the position uses `checked_add`.
- `usize` is used only for indices, the 64 KiB buffer length and u16-sized name lengths. None of these matters on i686.

## 2. Do the rerun logs contain every required line and follow the log rules?

**Required lines: all present.**
- **gate.log:**
  - the tree is the root printed as `<repo>` plus the ten crates, and the tree check says yes (lines 110-121);
  - the cold-cache removal is at line 125;
  - the eight PASS lines are in order, including native and licences (128-135);
  - stderr 0 bytes and exit 0 (136-138), and all as expected (148).
- **test.log:**
  - 12 tests each on the host and on i686 (99, 129), with every test name listed;
  - the new test runs on both (97, 127);
  - both counts are stated (138), the names are the same (139) and the counts are equal (140).
- **negative.log:**
  - case (a): byte offset 596 inside a stream that starts at 54, 1c before and 1d after, exactly one differing byte, the rows, `differs`, exit 1 (226-247);
  - case (b): 1153 bytes, the `error:` line, exit 2, no rows (250-259).
- **scan.log:**
  - the printed script calls `/usr/bin/grep` explicitly;
  - every form has a positive control;
  - it covers 15 files, 5 messages and the 3 rerun logs, with 0 hits, and ends with RESULT: PASS.

**Rules**
- Each log's first line names the commit and HEAD.
- My byte counts: 0 control bytes, 0 non-ASCII bytes and no em dash in all four logs and in check 1's file.
- The sha256 of gate.log, test.log and negative.log match scan.log lines 553, 562 and 571.
- There are no temp session paths, Windows paths or hostname in any log.
- scan.log lines 51, 124, 140 and 141 match session or home patterns only as split fictional pattern and control strings in the printed script.
- The denylist location appears only as an unexpanded home variable (gate.log line 5, scan.log line 20).

**The named fixes are present**
- negative.log lines 21-25 name them:
  - the phrase is split at line 41;
  - `one_error_line` at line 178 tests both the line count and the `^error: ` prefix;
  - `NDIFF` is evaluated once (lines 162-163).
- test.log lines 14-15 name its fix: `nok` is evaluated once at line 58 and reused at 59 and 61.
- Each of those checks now tests what its label says.

**Remaining weak points (low severity)**
- **Fixture text is still recoverable from negative.log.** The full phrase no longer appears as one string (my grep count is 0). But line 41 shows it split by one quote pair, and the embedded gen.py at lines 92-94 holds the content line templates. So "values never printed" (line 4) and the report's "fixed" are generous. The text is fictional; nothing private leaks.
- **One label claims more than it checks.** negative.log line 142, "three entry rows, all identical", counts only rows marked identical, not the total number of entry rows. A fourth row that differs would still pass this check, though exit 0 at line 141 would catch it.
- **Two items are not scanned by any committed log:** the rerun scan.log itself (a log cannot scan itself) and the message of the record commit that added the reruns. My own byte counts on scan.log are clean. Planning or the push hook covers the rest.

## 3. Does every number and quoted string in draft report 2 appear in a primary?

Almost all of them do:
- D1 counts: in check 1's file.
- Log hashes: scan.log lines 553-571.
- Commit hashes: scan.log lines 304-343.
- Every G, T and N line reference.
- The `run` signature, the usage line, "200000", 596, 54, 1c/1d, 1153 and the error string.

**Not in a primary, marked as session output**
- D2 counts. I confirmed them myself: 98 insertions, 4 deletions, 4 mode changes.
- The doc commit made without editing.
- The 10-byte prefix check giving 73.
- The chmod loop.
- The end times 16:47:17Z, 16:47:17Z and 16:47:18Z.
- The note on the T0b P2 log scripts and "you verified those facts on GitHub".
- That the fictional control path is not real.

**Not in a primary, not marked**
- Departure 8, "its replacement is crate private". D1 line 896 says only that the deprecated call is the only way to get the method number.

**Inaccuracies and overstatements**
- "the findings section lists the fixes" (Independent checks, item 5) is false. The findings section still says "None in T1 P1"; the fixes are listed in the Independent checks section.
- The quoted string "12 passed; 0 failed; 0 ignored" is cited at T 102 and T 132. Those lines use a different format ("passed 12; failed 0; ignored 0"); the quoted form is at T 99 and T 129 only.
- "3 fixture strings, all built at run time" and "values never printed is now accurate" overstate the fixture-text fix (see question 2).
- "Raw comparison ... fixed in D2" is true of the code, but no test makes the new error fire.
- The description of S's 15 files leaves out Cargo.lock, ROADMAP.md and check 1's file. This is minor.

**Treatment of check 1:** draft 2 corrects or records all eight of check 1's defects in check 1's own order, and represents check 1 fairly. It also records the gap check 1 found in how the "values never printed" line was worded. For check 1's timing observation, draft 2 explains only the rerun, not the first run.

## Remaining defects, ranked by severity

None of these could leak private data or let a push through.

1. **No test exercises the new short-stream error in `raw_equal`.** The code is correct, but "fixed" is unproven by a test.
2. **Fixture text is still recoverable in negative.log's printed script** (the split phrase at line 41, the gen.py templates at 92-94). This makes the report's "values never printed" and "fixed" generous. The text is fictional.
3. **Report inaccuracy:** "the findings section lists the fixes" is false.
4. **Report citation mismatch:** the quoted test summary string is cited at T 102 and T 132, which use a different format.
5. **Unmarked claim:** departure 8's "replacement is crate private" is not in a primary.
6. **Label wider than its check:** negative.log line 142 counts only identical rows.
7. **Scan coverage gap:** the rerun scan.log and the message of the record commit that added the reruns are not covered by any committed scan.
