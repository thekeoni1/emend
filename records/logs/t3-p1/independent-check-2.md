# T3 P1 independent check 2

- Checked commits: 9eaee4778f16f172d4a1b2414d147542b6bfcbc1 (T3 P1 phase
  commit 1), 79d17ba81eca7b4a64633e26c799c776a5fecdef (phase commit 2) and
  cb94097d4d4b4dbd0f296760b70036bda0093731 (phase commit 3, the fixes for
  check 1)
- HEAD when the check ran: 4a9e929314aebbb55bcc1130203544bccb5604c6
  (record commit with the logs rerun at cb94097, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, and asked to answer in plain ASCII
- Inputs, with sha256:
  - specs/0006-passthrough.md
    c799f57083463552c5000279d6c3992b047d800913edefca498763a36c7a6cfc
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - specs/0004-xlsx-diff.md
    857b0be1937a817c4980f62ae251230e057f44b82d4f267787dd2d3004435edb
  - specs/0000-step-zero-plan.md
    59d5ac883ba0a4a9b567339323cb551c3e935ef1e7d45ebb0198e7bbbce537da
  - phase diff 1, `git diff 8c979bc..9eaee47`, saved as <scratch>/t3p1-diff.txt
    095111b950522f2df6df8d8465197d12da9cca7bd2eaf9875b1dbe1a8fd1383b
  - phase diff 2, `git diff ba79abf..79d17ba`, saved as <scratch>/t3p1-diff2.txt
    0e9576b961a8da3baa95b1e6007504dd55efbaf34bf6cadd6ebd4e146ebcb27a
  - phase diff 3, `git diff 4e38962..cb94097`, saved as <scratch>/t3p1-diff3.txt
    676b6fde032569af6b1bf2320b017dcaa6a4e4488da5a412eb88681f88008640
  - records/logs/t3-p1/independent-check-1.md
    3d0558212a9754bc8122b9f9041ab0214edb02f5f1937c99b9a7bd8a5056c882
  - draft report 2, <scratch>/t3p1-report-draft2.md
    3a06f35408f2075d9484c5992cba6e9eae93e21418f0497462dfa731467d4dca
  - records/logs/t3-p1/gate.log
    028a210a9150b7d4907625b40b8e71b24270a3284bd2650234127a46e6b0ec5a
  - records/logs/t3-p1/gate-attempt1.log
    965f7029d77d6cb42922a6e6ab8863e29cf7eb5e50c720e5713774b72ce4ba9d
  - records/logs/t3-p1/test.log
    7dc2d63757a6de6704238dcb04c27276e1a90a25171610fb9a026b05ccb70936
  - records/logs/t3-p1/negative.log
    302bb223e85c6b874c1d5d946538463bc8ed95a7d4e05edebba6ab0f57720424
  - records/logs/t3-p1/scan.log
    d4672ba3204d04a207110eda79dc1ea961ec90e80247c7bc0bf5c2668cd878c0
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
CLAUDE.md and a memory index (MEMORY.md) were in my context. I did not use them as evidence. My only evidence was the files listed in the task, read with read-only commands (cat, sed, grep, wc, sha256sum, tr, python3). I ran no git, cargo or tool under test, and wrote no files.

Input hashes as I read them:
- specs/0006: c799f570...
- specs/0001: 5b7ed9aa...
- specs/0004: 857b0be1...
- specs/0000: 59d5ac88...
- diff 1: 095111b9...
- diff 2: 0e9576b9...
- diff 3: 676b6fde...
- draft 2: 3a06f354...
- I1: 3d055821...
- the logs: GA 965f7029..., G 028a210a..., T 7dc2d637..., N 302bb223..., S d4672ba3...

## 1. Do the three diffs do what P1 says, and nothing else?

Yes, with the gaps below. Every change is inside the P1 deliverables:
- **Diff 1:** planning's 4 doc files (S 322 to 327) and phase commit 1's 9 files (S 329 to 339).
- **Diff 2:** one assertion in copies.rs.
- **Diff 3:** only scripts/passthrough-run.sh and tools/passthrough/tests/copies.rs (S 364 to 367).

**Copy tool (main.rs)**
- It matches the spec's design:
  - `ZipArchive::new`, `ZipWriter::new`, then `raw_copy_file(by_index_raw(i))` in index order, then `set_raw_comment(archive.comment())`, then `finish` and the BufWriter flush.
  - The output goes to a `create_new` temporary file beside the target and is renamed into place.
  - Nothing is printed on stdout. Every error is one `error: ` line naming at most an entry index. Exit is 0 or 2.
  - The five faults are there. The guard and the unknown-value check run before the input is opened; the fault itself runs after opening.
- Draft departures 2 and 3 describe this correctly.
- The temporary file is still removed only when an error is returned (I1 defect 4). The draft now says so (lines 57 to 60).
- Now that the timeout's SIGKILL reaches the tool, a timeout during a real copy leaves `.out.<label>.<ext>.passthrough-<pid>.tmp` in the run directory. The runner neither removes nor records it, and the draft does not name that consequence.

**Runner: arguments, steps and output**
- The arguments and defaults match the spec:
  - the `EMEND_CORPUS_DIR` then `$HOME/emend-testing` fallback;
  - `<corpus>/runs/<utc-date>-<shortsha>`;
  - 300 s, and 2048 MiB as `ulimit -v` in KiB;
  - `--ids` kept in manifest order.
- The per-file steps match: mkdir, the input path, xlsx-diff on `ok` with stdout to `diff.<label>.tsv`, and the appends.
- SUMMARY_HEADER is exactly the spec's 21 columns in order. Twelve values plus a 9-value tail give 21 fields.
- The stdout file line and the count line match the spec.
- Nothing from the child reaches stdout.

**Runner: phase commit 3, classification.** GNU time now runs `timeout`, which runs the tool.
- `ok`, `error`, `panic`: time reports the exit status that timeout passes through (0, 2, 101). Correct; N 451, 539, 485.
- `crash` with a signal: timeout re-raises the child's signal, and time prints `Command terminated by signal N`. Checked first. Correct; N 498, 524.
- `crash` with an exit: any other status, e.g. 3. Correct; N 511.
- No report at all (ulimit failure, exec failure): `crash` with rc. Correct.
- `timeout`: only an `Exit status: 124` line. On this machine that is what timeout produced (N 469, 601), and the survivor check (N 474) shows the tool is gone.
- Before commit 3 the code also accepted 137. The spec says "exit 124 or 137 from `timeout`". A timeout that ends as 137, or one where timeout dies of its own process-group SIGKILL (time would then print signal 9), is now recorded as `crash`.
  - The timeout manual describes 128+9 rather than 124 with `--signal=KILL`. That is my knowledge of coreutils, not a primary, and this machine's log shows 124.
  - Departure 4 lists "124 `timeout` ... any other status ... `crash`" but does not say that the spec's 137 was dropped.
- A tool exiting 124 by itself reads as a timeout; the draft names this.

**Runner: phase commit 3, stderr**
- `exec 3>&2 2>/dev/null` runs after the argument checks, and `die` writes to fd 3. Every die after that point still reaches the caller: case 9 (N 550, 555) and case 10 (N 561, 566) show one error line and no mkdir text. The dies before the exec print only option names and the usage text.
- So the mkdir, head, awk and redirection paths from I1 defect 6 are covered.
- Side effects the draft does not name:
  - fd 3 is not close-on-exec, so time, timeout, passthrough, xlsx-diff and awk all inherit the caller's stderr. Nothing writes to it.
  - Commands without `|| die` now fail silently. Examples: `cp "$WORK/rows" "$WORK/selected"`, `: >"$WORK/wanted"`, `rm -f`, and `header_ok`'s write of a new header. A failed cp would give an empty loop, `files 0 ...` and exit 0 with no error line. A failed header write is reported as "has another header". All of these are in a fresh mktemp directory or the run directory, so the risk is low.

**Runner: phase commit 3, duplicate-id refusal**
- `header_ok` runs first.
- On a new summary file it writes only the header. The awk skips FNR 1, finds no id, and the run proceeds; N case 1 ran in a new directory.
- On an existing file, the first selected id with a row is printed and the run is refused before anything is appended. Case 10 shows exit 2, the exact line, and an unchanged summary sha (N 564 to 567). The row counts at N 616 and 617 still show seven rows.
- The check is per label, as designed.

**Record script**
- FAMILIES equals the specs/0000 list exactly: 31 names, same order, compared with python.
- Every rule maps to a listed family; `other` has no rule, as intended. Rules are first-match:
  - `_rels` before `.vml`, and `.vml` before drawing;
  - threadedComments does not match the comments rule;
  - chartsheets, dialogsheets and macrosheets go to worksheet; slicerCaches to slicer; timelineCaches to timeline; `vbaProject*.bin` to vbaProject.
  - Departure 6 names these additions. I found no mis-mapping for the usual part paths.
- The .tsv (summary rows, blank line, entries header and rows) and the .md (commit, dates, targets, rustc, zip, outcome counts, a family table per label with the spec's columns plus a total row, the not-ok list) match the spec with departures 5 and 6.
- The script never reads the manifest.
- The default stem also refuses a run directory name that is not `<date>-<sha>`. That is not listed; cosmetic.

**Tests**
- Every spec fixture is present. The additions (MS-DOS made-by, duplicate names) are named in departure 8.
- Diff 2's comparison of the output central sizes with the harness's `compressed_out` and `uncompressed_out` can fail.
- Diff 3's `locals(&input)[0][5] == 20` and the duplicate test's directory listing can both fail.
- The N case 2 survivor check (`pgrep -f -c -- "$BASE"`) can fail: the tool's arguments are absolute scratch paths, and an empty result fails the numeric test.
- Assertions that cannot fail:
  - the faults.rs `assert_error` path check (acknowledged in the draft);
  - N case 1's "one run directory" check, which takes `basename` of `find`'s output and would pass with two directories if the last matched. Vacuous in a fresh scratch directory.

**Check 1's defects**
1. Closed: command order swapped, survivor check at N 474. The 137 remark above is new.
2. Closed: draft line 119 now names the MS-DOS fixture, which matches `raw_stored` in copies.rs.
3. Closed. Draft lines 78 to 85 match the tests: 13 tests call `copy_ok`, 12 call `copy_and_compare`, 9 assert entry triples, 3 assert archive values, and the duplicate test asserts exit 2 plus the stderr line and now the listing.
4. Partly closed: documented, not fixed in code.
5. Closed: departures 5 and 8 name all four items. Departure 8 does not say it departs from 0001's "Every negative test runs in a scratch copy".
6. Closed for the paths named, with the side effects above.
7. Closed.
8. Closed (N 206, 430).
9. Closed (draft lines 63 to 65).
10. Closed (G 135, 136).
11. Partly closed: the local extra field is now asserted; the `assert_error` path check stays, acknowledged.
12. Closed: families checked above. Departure 2 is marked session output.

## 2. Do the logs contain every required line?

Yes.

- **P1 negative cases:**
  - normal: N 442, 450 to 455;
  - sleep with `--timeout 2`: N 463, 469, 472 to 474;
  - panic, exit 101: N 479, 485;
  - abort, `crash` with signal 6: N 492, 498;
  - exit3, `crash` with exit 3: N 505, 511;
  - alloc with `--memory 256`, recorded as crash with signal 6 and the child's stderr kept: N 518, 524 to 528;
  - unguarded, `error` exit 2 with the refusal line: N 533, 539, 543, 544;
  - the record script: .md and .tsv under `<scratch>/record`, the family table, the non-ok list of exactly the six fault ids, 18 lines, zero sentinel, path, control and non-ASCII hits: N 626 to 678;
  - sentinels in both `original_name` and `note`: N 429, 430.
- **Added cases:** 9 at N 546 to 555; 10 at N 557 to 567.
- **First-line rule:** G 1, GA 1, T 1 and N 1 name the commit and the HEAD; S 1 names the range and the HEAD.
- **Rerun header rule:**
  - G 3, T 3, N 3 and S 3 name record commit 8d202cd, whose logs they replace.
  - G 3 names gate-attempt1.log (the failed run committed as its own log), its sha256 (matches my sha256sum and S 417, 637) and the fix.
  - No other failed run was committed as its own log.
- **Each check once, compound checks in functions:**
  - N: `all_zero`, `summary_is`, `stdout_is`, `count_line_is`, `alloc_recorded`, `md_heading`, `md_total`; cases 2, 9 and 10 add only single-condition checks.
  - T: `passed_matches`. S: `files_complete`, `messages_complete`, `scan_passed`. G: single conditions only.
- **Control bytes:** my `tr` counts give 0 control and 0 non-ASCII bytes in all six log files and in draft 2. S 254 is the positive control.
- **Paths:**
  - The only `/tmp/`, `/home/` and `/mnt/` strings are S 40, 41, 133, 134, 142 and 143 (scan patterns and split control strings) and I1 line 134 (prose about them).
  - The logs otherwise use `<repo>`, `<scratch>`, repo-relative paths and a literal `$HOME`.
- **Fixture text:** `<fixture>` appears only at N 68, in the script source, so `hide` never replaced a sentinel in captured output. The phrase is split in the source (N 60). No `zq` sentinel appears.
- test.log covers both targets with the same 69 names (T 179, 283, 289 to 291).

## 3. Does every number and quoted string in the draft appear in a primary?

Yes for everything I could check; the exceptions are listed at the end of this section.

**Numbers I recomputed from the diffs**
- D1: 13 files, 2136 insertions, 11 deletions.
- D2: 1 file, 8 insertions, 1 deletion.
- D3: 2 files, 35 insertions, 13 deletions. The draft marks this as session output, but it is checkable from diff 3 and is correct.
- d63ad31's four doc files: 279 insertions and 10 deletions; the three modified files: 55 and 10.
- 9eaee47's nine files: 1857 insertions and 1 deletion.

**Hashes**
- Full commit hashes match S 314 to 320.
- Log sha256 prefixes match S 637, 646, 655 and 664 and my own sha256sum.
- The Co-Authored-By lines are S 565 to 625 (seven, S 634).

**Line citations.** All correct:
- G 3, 116 to 133, 135, 136, 140 to 147, 148 to 150, 151, 160, 161;
- GA 130, 146, 148, 158. GA's only `: no` lines are 130 and 158;
- T 87 to 101, 101, 118, 135, 147, 159, 166, 171, 176, 179, 191 to 205, 205, 222, 239, 251, 263, 270, 275, 280, 283, 289 to 291, 293, 294;
- N 5, 401 to 407, 419 to 430, 442, 446, 447, 449, 450, 451, 454, 455, 463, 467, 469, 472 to 474, 479, 483, 485, 492, 496, 498, 505, 509, 511, 518, 522, 524, 528, 533, 537, 539, 544, 552, 553 to 555, 561, 563, 564 to 567, 618, 626 to 678, 680, 682;
- S 310, 561, 634, 673, 675, 678, 679.

**Quoted strings.** Each appears where cited:
- the stdout lines, the run-ended times, `PASS native (allowlisted -sys: none)`, `4 passed; 0 failed; 1 ignored`, the refusal and error lines, and the duplicate-name stderr.

**Finding table.** Every cell matches what copies.rs asserts:
- 0xcafe is 8 bytes; "fictional entry comment" is 23 bytes; the file is 16 bytes shorter;
- 0x0014 is `raw_stored`'s made-by 20.
- One small overstatement: "plain sizes instead of 0xffffffff" for the zip64-marked entry. The test asserts the input's compressed-size placeholder only, not the uncompressed one.

**Items marked session output.** These cannot be checked from primaries:
- the d63ad31 and 9eaee47 stats (consistent with my counts);
- "match your stat of 55 insertions and 10 deletions" (planning's stat is not a primary);
- departure 2's claim about the harness usage line;
- departure 4's hand confirmation of the surviving tool.

**Unmarked claims no primary supports**
- "The real corpus directory was not touched" (N shows only the override for the negative run).
- "Nothing is pushed".
- The byte builder "copied from `xlsx-diff`'s tests" (source comment only).
- "as the earlier negative logs did" (departure 8).
- "S itself: in the check header" and "hash in the check header": not yet checkable. scan.log's sha256 is d4672ba3204d04a207110eda79dc1ea961ec90e80247c7bc0bf5c2668cd878c0.
- The rerun header's "copies.rs asserts two more fields": one of the two is a directory listing, not a field.

## Defects

1. **Low.** The timeout classification now accepts only `Exit status: 124` from time's report. The spec's 137 (and a timeout killed by its own group SIGKILL, reported as signal 9) would be recorded as `crash`. The code before commit 3 accepted 137. Departure 4 does not say the 137 case was dropped. The log shows 124 on this machine.
2. **Low.** When the tool dies of a panic, an abort or a SIGKILL during the copy, its temporary file stays behind. This is documented, not fixed. With the timeout now killing the tool, a timeout during a real copy leaves `.out.<label>.<ext>.passthrough-<pid>.tmp` in the run directory. The runner neither removes nor records it, and the draft does not say so.
3. **Low.** With stderr sent to /dev/null, commands without `|| die` fail silently: `cp` to selected, `: >wanted`, `rm -f`, and `header_ok`'s new-header write (misreported as "has another header"). A failed `cp` gives `files 0` and exit 0 with no error line. The draft says only "a failed append is an error".
4. **Cosmetic.** The saved stderr on fd 3 is inherited by every child (time, timeout, passthrough, xlsx-diff, awk). Nothing writes to it, but a surviving child would hold the caller's stderr open.
5. **Cosmetic.** Departure 8 names the build in the main working tree but not that it departs from 0001's scratch-copy rule. "as the earlier negative logs did" has no primary.
6. **Cosmetic.** Unmarked claims without a primary: "The real corpus directory was not touched", "Nothing is pushed", the builder "copied from `xlsx-diff`'s tests". Also, "copies.rs asserts two more fields" in the rerun headers (G 3, T 3, N 3) counts a directory listing as a field.
7. **Cosmetic.** Assertions that cannot fail or overstate:
   - N case 1's "one run directory" check takes `basename` of `find`'s output and would not detect a second directory;
   - the faults.rs `assert_error` path check (acknowledged);
   - the zip64-marked table row says "plain sizes instead of 0xffffffff", but only the input's compressed-size placeholder is asserted.
8. **Cosmetic.** Departure 6 does not mention that the default stem refuses a run directory name that is not `<utc-date>-<shortsha>`.
