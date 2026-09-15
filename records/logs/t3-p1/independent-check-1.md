# T3 P1 independent check 1

- Checked commits: 9eaee4778f16f172d4a1b2414d147542b6bfcbc1 (T3 P1 phase
  commit 1) and 79d17ba81eca7b4a64633e26c799c776a5fecdef (phase commit 2)
- HEAD when the check ran: 8d202cda7fb89b209a305cde80adb99d7d813b43
  (record commit with the logs rerun at 79d17ba, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, and asked to answer in plain ASCII
- Inputs, with sha256:
  - specs/0006-passthrough.md
    c799f57083463552c5000279d6c3992b047d800913edefca498763a36c7a6cfc
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - specs/0004-xlsx-diff.md
    857b0be1937a817c4980f62ae251230e057f44b82d4f267787dd2d3004435edb
  - phase diff 1, `git diff 8c979bc..9eaee47`, saved as <scratch>/t3p1-diff.txt
    095111b950522f2df6df8d8465197d12da9cca7bd2eaf9875b1dbe1a8fd1383b
  - phase diff 2, `git diff ba79abf..79d17ba`, saved as <scratch>/t3p1-diff2.txt
    0e9576b961a8da3baa95b1e6007504dd55efbaf34bf6cadd6ebd4e146ebcb27a
  - draft report 1, <scratch>/t3p1-report-draft1.md
    f0bf26f88c7d647a1ebb7aded42557dff3b6134794c682c63175ae37336fe84f
  - records/logs/t3-p1/gate.log
    b21aad3a4a7e69493e4da0669be102324f4a2bd6ef6f9b0942d9af71fde50b17
  - records/logs/t3-p1/gate-attempt1.log
    965f7029d77d6cb42922a6e6ab8863e29cf7eb5e50c720e5713774b72ce4ba9d
  - records/logs/t3-p1/test.log
    ea9373a8c6ffd7022ddab1c31f3fc7410fe3eebd08dbb43f1c26dfb4bf720abc
  - records/logs/t3-p1/negative.log
    fd3bf138e6556280a7c046511db605cf5f9752180eb18076b2af813faa8413d9
  - records/logs/t3-p1/scan.log
    5bdd61c2c9f880a4836f70c0e63075aae454a0d36c976dc4242037052e1e1fa0
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
CLAUDE.md and a memory index (MEMORY.md) were in my context. I did not use them as evidence. My only evidence was the files listed in the task, which I read with read-only commands. I ran no git, cargo or tool under test, and wrote no files.

## 1. Do the two diffs do what the P1 spec says, and nothing else?

Mostly yes. The code follows the spec, and the draft's departures list names most of the places where it differs. I found one wrong statement in the report, a few departures the list does not name, and some small gaps in the code and tests.

**Scope.** Diff 1 has 13 files: planning's 4 doc files (commit d63ad31, per S 318-324) and the 9 files of phase commit 1 (S 326-336). Diff 2 changes one assertion in copies.rs. Nothing outside the P1 deliverables is touched. Cargo.lock adds only the `passthrough` package, which depends on xlsx-diff and zip.

**The copy tool (tools/passthrough/src/main.rs)**
- It matches the spec on these points:
  - `ZipArchive::new`, `ZipWriter::new`, then `raw_copy_file(by_index_raw(i))` for `0..len` in order, then `set_raw_comment(archive.comment())`, then `finish` and the BufWriter flush.
  - The temporary file is `.<name>.passthrough-<pid>.tmp` beside the target, opened with `create_new`, and renamed into place.
  - Nothing is printed on stdout.
  - Every error is one `error: ` line. The messages carry an io::ErrorKind or an entry index, never a path or zip crate text.
  - Exit is 0 or 2. Entry indexes are converted to u64.
  - The five faults are there. The guard refuses any set `PASSTHROUGH_FAULT` unless `PASSTHROUGH_FAULT_ALLOWED` is exactly `1`, and it does so before the input is opened.
- **Cleanup gap (not in the draft).** The temporary file is removed only when `copy` or `rename` returns Err. A panic inside the zip crate, a SIGKILL from the runner's timeout, or an abort during the copy leaves the `.tmp` file in the run directory. The target itself is never partial, so the spec's wording is met. But the draft's "a failure after the temporary file exists removes it" is true only for returned errors. The runner does not remove stale temporary files either. No fault fires after the temporary file exists, so nothing tests this.
- **Wording.** The draft says the faults are "read from PASSTHROUGH_FAULT after the input is open". In the code the variable is read and checked (guard, unknown value) before the input is opened; only the fault itself runs after opening.
- **Departures 2 and 3** (usage line, one fault point) match the code.

**The runner (scripts/passthrough-run.sh)**
- It matches the spec on these points:
  - The arguments and defaults. The corpus falls back from `EMEND_CORPUS_DIR` to `$HOME/emend-testing`; the output defaults to `<corpus>/runs/<utc-date>-<shortsha>`; timeout 300; memory 2048 as `ulimit -v` in KiB.
  - `--ids` keeps manifest order.
  - The per-file command line: `timeout --signal=KILL`, `/usr/bin/time -v -o`.
  - The summary header and printf order match the spec's 21 columns exactly. Eight tabs give nine empty archive values.
  - Entry rows are prefixed with id and label.
  - The stdout line and the final count line match.
  - The child's output goes to files, never to stdout.
- **Classification** checks, in order: timeout (rc 124 or 137 with no time report), then the signal line, then the time exit status (0 ok, 2 error, 101 panic, else crash), else crash with rc. Signal is checked before the time report's "Exit status" line, which is right, since GNU time writes both for a signalled child. The draft names this as departure 4. I found no classification bug.
- **Departures the draft does not name:**
  - (a) Stdin is redirected from /dev/null, not closed (the spec says "stdin closed").
  - (b) Entry rows are appended and archive values filled only when diff exit is 0 or 1. The spec says "appends every entry row of the diff". The choice is defensible under 0004 "Exit status", but it is not listed.
  - (c) `ulimit -v` also applies to `timeout` and `time`, not only the child.
  - (d) Empty values print as `-` on stdout.
  - (e) Running the same id twice in one run directory appends a second summary row. The record script then refuses that directory.
- **Stderr paths.** The header comment says errors print "without a path". But failures in `mkdir -p "$OUT"`, `head`, `awk` on the manifest, or a failed `>>` redirection print the tool's or bash's own message, including a path under the corpus directory, before `die`. This only happens on failure paths, and no negative case covers it.

**The record script (scripts/passthrough-record.py)**
- It matches the spec on these points:
  - The .tsv has the summary rows, a blank line, then the entry rows with names exactly as the harness wrote them.
  - The .md has commit, date, targets, rustc and zip versions, and outcome counts per label.
  - The family table columns are in the spec's order: family, entries, the five statuses, flagged, then the seven flags.
  - The not-ok list filters on outcome != ok or diff_exit != 0 and shows id, producer, tags and the values that differ from expected.
  - It never reads the manifest.
- **Family mapping.** Rules are tried in order, first match wins. `_rels` comes before `.vml`, which comes before the drawing rule. I found no mis-mapping for the usual OOXML part paths.
- **Not checkable.** I cannot confirm that the FAMILIES names and order match `specs/0000-step-zero-plan.md`, because that file's family list is not among my inputs.
- Departures 5 and 6 match the code.

**Tests**
- **Fixture list.** Every spec fixture is present. Stored and deflated entries are separate archives rather than one mixed archive. copies.rs adds MS-DOS made-by and duplicate names (departure 8). Fault injection and the guard are tested through the binary in faults.rs.
- **The report overstates copies.rs.** It says each test asserts "the (name, status, flags) of every entry row" and that no temporary file is left.
  - `archive_comment_is_kept` and `two_hundred_entries_are_copied_byte_identical` check archive values only, not entry rows.
  - `duplicate_names_keep_only_the_last_entry` does not go through `copy_and_compare`, so it checks neither entry rows nor the directory listing for a leftover temporary file.
  - The exit-0 assertions do imply clean rows under 0004 "Exit status".
- **Assertions that cannot fail.**
  - Phase commit 2 correctly fixed `assert_eq!(b[0].compressed, a[0].compressed.min(b[0].compressed))`. The new comparison, the output central record's sizes against the harness's `compressed_out` and `uncompressed_out` (sums over one entry), can fail.
  - In faults.rs `assert_error`, the "no path in stderr" check follows an exact equality against a fixed string, so it adds nothing. It is harmless.
  - I found no other assertion that cannot fail.
- **Not asserted.** In the zip64-marked test, the input's local extra field length is not asserted, although the finding table says "the zip64 extra fields dropped" (plural).

**Finding table**
- **Wrong statement.** The paragraph after the table says "every raw fixture other than the leading-junk one was built on a Unix deflated baseline". In copies.rs the leading-junk fixture uses `unix_deflated`. The raw fixture that does not is the MS-DOS one (`raw_stored`, version made by 20, attributes 0). The exception is misnamed.
- Every other table cell matches what its test asserts.

## 2. Do the logs contain every required line?

Yes, with the exceptions noted below.

**Negative cases the spec lists for P1** (all in negative.log):
- Normal file: `ok`, diff exit 0 recorded, 8 entry rows appended and equal to a by-hand run (N 414-427).
- Sleep with `--timeout 2`: `timeout`, exit 124, signal 9 (N 435, 441, 444, 445).
- Panic: exit 101 (N 450, 456).
- Abort: `crash` with signal 6 (N 463, 469).
- exit3: `crash` with exit 3 (N 476, 482).
- Alloc with `--memory 256`: recorded as `crash` with signal 6 (N 489, 495, 499).
- Fault variable without the allow variable: `error`, exit 2, and the refusal line on the child's stderr (N 504, 510, 514-515).
- Record script: exit 0; exactly the .md and .tsv under `<scratch>/record`; family table; not-ok list with the six fault ids; 18 tsv lines; zero sentinel, path, control and non-ASCII hits (N 569-626).
- The scratch manifest carries sentinels in `original_name` and `note` (gen.py in the script, N 185).

**Gaps in the negative log:**
- Only the name sentinel's presence in the manifest is checked (N 402). The note sentinel's presence is never counted, so the "0 note hits" part rests on reading gen.py.
- Case 2 checks "no output file left" as soon as the runner returns. It cannot show that the grandchild passthrough process was killed; a survivor would write its output after the 30 s sleep.
- The negative script builds and runs in the main working tree. Only the corpus and outputs are under `<scratch>`. The 0001 rule says negative tests run in a scratch copy, never in the main working tree. The 0006 P1 text only asks for a scratch corpus, and the tree was clean afterwards (N 628), but the draft does not name this.

**First-line rule.** G 1, GA 1, T 1 and N 1 name the commit and the HEAD. S 1 names the range 8c979bc..79d17ba and the HEAD.

**Rerun header rule.** Met.
- G 3, T 3, N 3 and S 3 each name record commit ba79abf.
- G 3 names gate-attempt1.log, its sha256 965f7029..., and the fix (awk NF). My sha256sum of that file matches, as do S 397 and S 578.
- GA is the only failed run committed as its own log.

**Each check evaluated once, compound checks inside one function.**
- Compound checks sit in functions: `passed_matches`, `summary_is`, `stdout_is`, `count_line_is`, `alloc_recorded`, `all_zero`, `files_complete`, `messages_complete`, `scan_passed`.
- Every other check is a single command run once through `yn` or `is`.

**No control bytes.** My `tr` count gives 0 control bytes and 0 non-ASCII bytes in all five logs, including scan.log, and in the draft.

**Paths.** The logs use `<repo>`, `<scratch>`, repo-relative paths, and a literal `$HOME`. The only `/tmp/`, `/mnt/` and `/home/` strings are in scan.log (lines 40-41 and 133-143), inside the scan patterns and split control strings.

**No fixture text or original name.** None appears in captured output. The content phrase is split in the script source (N 56), and the sentinels are random and never printed.

**Gate log.** It names both gate reruns' results (G 133, 140-151, 161). test.log covers both targets (T 179, 283, 289-291).

## 3. Does every number and quoted string in the draft appear in a primary?

Almost all do. Two items conflict with the primaries (the Unix-baseline sentence and the copies.rs description, both from section 1), and a few cannot be checked.

**Numbers I recomputed from the diffs:**
- D1: 13 files, 2136 insertions, 11 deletions.
- D2: 1 file, 8 insertions, 1 deletion.
- The four doc files: 279 insertions, 10 deletions; the three modified doc files: 55 and 10.
- Phase commit 1's nine files (per S 326-336): 1857 insertions, 1 deletion.

**Hashes.**
- Log sha256 prefixes match S 578, 587, 596 and 605, and match my own sha256sum of GA, G, T and N.
- Full commit hashes 9eaee47... and 79d17ba... match S 314-316.
- Messages: S 536, 546, 556, 566 and 575 are correct.

**Line citations.** All correct except:
- "a cold cache (G 136)": the heading is G 135; G 136 is the `cargo clean` command line. This is cosmetic.
- "GA 146, 148": GA 146 is "exit: 0" and GA 148 is the eight-PASS check. Correct.
- "GA 130, 158": 130 is the tree check's `no`; 158 is the summary `no`. Correct.

**Finding table and acceptance.** The citations T 87-101 and T 191-205 are right. Every value and the quoted duplicate-name stderr match copies.rs. The quoted strings `db311887995b host ok 0 0.00 3340`, the run-ended times, `PASS native (allowlisted -sys: none)`, `4 passed; 0 failed; 1 ignored`, "26 text files" and "18 tsv lines" all appear where cited.

**Conflicts with the primaries:** the Unix-baseline sentence and the copies.rs description (section 1, "Wrong statement" and the copies.rs overstatement).

**Items that cannot be checked from primaries:**
- Items the draft marks as session output:
  - The per-commit and diff stats. These are arithmetically consistent with my counts.
  - "the three modified files match your stat of 55 insertions and 10 deletions": planning's own stat is not a primary.
- Items not marked as session output:
  - "The record commit with the rerun logs: hash in the check header."
  - "S itself: in the check header."
  - "The real corpus directory was not touched."
  - "Nothing is pushed."
  - Departure 2's claim that "the harness prints its usage line bare": 0004's Output section does not say this, and xlsx-diff's source is not an input.
  - That the byte builder was copied from xlsx-diff's tests.
  - The families' relation to 0000.

## Defects

1. **Medium.** Negative case 2 does not show that the passthrough process under `time` was killed on timeout. It checks for a missing output only as the runner returns, so a surviving child would go unnoticed.
2. **Low.** The report says "every raw fixture other than the leading-junk one was built on a Unix deflated baseline". The leading-junk fixture uses `unix_deflated`; the exception is the MS-DOS fixture (`raw_stored`).
3. **Low.** The report says each copies.rs test asserts every entry row's (name, status, flags) and that no temporary file is left. `archive_comment_is_kept` and `two_hundred_entries_are_copied_byte_identical` do not assert entry rows. `duplicate_names_keep_only_the_last_entry` checks neither rows nor leftover temporary files.
4. **Low.** A temporary file is left behind when the copy is ended by a panic, abort or SIGKILL after the file exists. The draft's "a failure after the temporary file exists removes it" holds only for returned errors, and no test covers it.
5. **Low.** Departures not in the draft's list:
   - entry rows and archive values are recorded only for diff exit 0 or 1 (the spec says every entry row);
   - stdin is /dev/null, not closed;
   - `ulimit -v` also covers `timeout` and `time`;
   - the negative script builds and runs in the main working tree, against the 0001 scratch-copy rule.
6. **Low.** On failure paths the runner can print a corpus-directory path on stderr through mkdir, head, awk or bash redirection errors, despite its "without a path" comment. The spec's "never prints a path under the corpus directory" is not proven for these paths.
7. **Low.** Running the same id twice in one run directory appends a duplicate summary row, which makes the record script refuse the directory. Neither the draft nor a test covers this.
8. **Low.** The negative log never checks that the note sentinel is present in the manifest, so the zero-hit count for it could pass vacuously.
9. **Cosmetic.** The draft says faults are "read ... after the input is open". The variable is read and checked before opening; only the fault runs after.
10. **Cosmetic.** The citation "a cold cache (G 136)" should be G 135.
11. **Cosmetic.** In the zip64-marked test, the input's local extra field is not asserted, though the table says "extra fields" (plural). faults.rs `assert_error`'s path check cannot fail once the exact equality holds.
12. **Cosmetic (not checkable).** From my inputs I cannot check the FAMILIES list against 0000's family list, or departure 2's claim about the harness usage line.
