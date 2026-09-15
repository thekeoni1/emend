# T3 P2 independent check 1

- Checked commit: 2845fab9ba5ebbea759313de66c7eda2699b84e6 (T3 P2 phase
  commit)
- HEAD when the check ran: 434e714574c4d8667b75faf90b0b0c2e5d6571c4
  (record commit with the logs and the size record, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below and planning's two rulings quoted in its prompt, and asked
  to answer in plain ASCII
- Inputs, with sha256:
  - specs/0006-passthrough.md
    f1fd88f355b438782081c1af63d78fcbd490458c9c3e5b34ad82d8682245a2b6
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - specs/0004-xlsx-diff.md
    857b0be1937a817c4980f62ae251230e057f44b82d4f267787dd2d3004435edb
  - phase diff, `git diff 33c0198..2845fab`, saved as <scratch>/t3p2-diff.txt
    7fa2e5f92cda077c959c63d85f23d56ce381d1713bc72e70a8eaabf6dce8c0e3
  - draft report 1, <scratch>/t3p2-report-draft1.md
    b875470482ecf8b8280f0b9cf6e3daa61fa946fbc23a7c752492530b55263458
  - records/size-baseline-2026-09-15-2845fab.md
    41e0d747458a9ba08677b8e810b1f4634d125f281298aa8897f1f89de19eece8
  - records/logs/t3-p2/gate.log
    c9944cedaa21497be5b93d817aa8aecb65406dd2d5f7dd43335c6841e466a6b3
  - records/logs/t3-p2/test.log
    c662e995147dc35bb89b842ccd4ec65c285813a5ad65c2aa83f0fef7689a3bb3
  - records/logs/t3-p2/negative.log
    742dce0e284ab1fa87f7aa549ecaaa3904f1b8c9efa4bba6f6fed3e5a31516b2
  - records/logs/t3-p2/size.log
    2dd9ba9aa118a97167b91539c731d43a2b35a129d47e82b48fdd2eba0d5625c2
  - records/logs/t3-p2/scan.log
    0fb036789b5d9304c07f3409c7927948062cef73d2c94bc506a934950311d309
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
I noticed that the harness put CLAUDE.md, a memory index (MEMORY.md), a git status snapshot and commit-attribution text into my context. I did not use any of them as evidence. The evidence was only the files listed in the task: specs/0006-passthrough.md, specs/0001-bootstrap.md ("How this task runs", "Rules for every phase"), specs/0004-xlsx-diff.md ("Output", "Exit status"), the two quoted rulings, <scratch>/t3p2-diff.txt, <scratch>/t3p2-report-draft1.md, records/size-baseline-2026-09-15-2845fab.md and the five logs under records/logs/t3-p2/. I ran only read-only commands: cat, sed, grep, wc, sha256sum, tr and python3 for counting.

## 1. Does the diff do what P2 says (with the two rulings), and nothing else?

**Scope.** The diff touches 11 files. My python3 count gives 332 insertions and 47 deletions.
- Planning's doc files (ROADMAP.md and specs/0006-passthrough.md) account for 81 insertions and 9 deletions.
- The other 9 files account for 251 insertions and 38 deletions: Cargo.toml, Cargo.lock, the two scripts, tools/passthrough/src/main.rs, tools/passthrough/tests/faults.rs and the three new tools/size-probe files.
- Nothing outside P2 and the two rulings is in the code diff.
- Cargo.lock adds only quick-xml 0.42.0 (depending on memchr) and size-probe.

**tools/size-probe and the size profile.**
- The crate is `publish = false` and has two binaries.
  - `probe-zip` opens the archive with `ZipArchive::new` and prints `entries <n>`.
  - `probe-zip-xml` also reads `by_name(entry)` through `quick_xml::Reader::from_reader` / `read_event_into`. It counts Start and Empty events and prints `entries <n>` and `elements <n>`. This matches "Size baseline".
- Dependencies:
  - zip is `=8.6.0`, `default-features = false`, `deflate-flate2-zlib-rs`.
  - quick-xml is `=0.42.0`, `default-features = false`, optional, behind feature `xml`.
- `[profile.size]` has exactly the seven keys the spec lists.
- Departures from the spec text:
  - The spec says the `xml` feature is "on for that binary only". The code uses `required-features = ["xml"]`, so the feature has to be passed explicitly. Draft departure 3 names this.
  - The labelled output and the exit 2 behaviour are named in departure 4.
- Two claims cannot be checked from primaries:
  - "zip exactly as xlsx-diff": tools/xlsx-diff/Cargo.toml appears only as a hash (gate.log 130), never as content. The gate's crate-set check (G 155) is consistent with it but does not prove the features match. The size record's "(as `xlsx-diff`)" rests on the same gap.
  - "quick-xml at the current release": no primary shows 0.42.0 is current. Ruling (2) names 0.42.0, so I accept it.

**Runner fixes (scripts/passthrough-run.sh).**
- *Leftover column.*
  - The glob `"$dir/.out.$LABEL.$ext.passthrough-"*.tmp` is quoted except for the `*`. The leading dot is literal, so dotfiles match.
  - With no match, the literal pattern fails `[ -e ]` and the loop continues.
  - Each match is removed with `|| die`, and `leftover=yes` is set.
  - The pattern matches the tool's temporary name as shown by the faults.rs test (`.out.zip.passthrough-<pid>.tmp`).
- *Column order.* The printf now has 14 `%s`: the 12 leading fields, `$values` (9 tab-joined fields), then `$leftover`. That is 22 columns, matching the 22-name SUMMARY_HEADER. The summary rows at N 809 to 828 show this. The spec's column list does not place `leftover`; departure 6 names it as last.
- *Guards.* All the commands the spec lists are guarded:
  - `: >wanted`, each append, and the read loop (`done <"$IDS" || die`);
  - the awk select and the `cp`;
  - the `rm -f` of earlier outputs and of leftovers;
  - a new header's write (header_ok now dies itself).
  - Also guarded: the two sha256sum pipelines and the archive-value awk.
  - I checked the `while ... done || die` form for false dies. The loop's status is the last body command (the guarded printf, or `continue`), so it cannot die spuriously.
- *Descriptor closed.* `3>&-` is on the tool's subshell and on the xlsx-diff call.
  - The diff never shows where fd 3 is created, so "fd 3 is the saved stderr" cannot be confirmed from the diff. It is consistent with the die lines in N 727 and 738.
  - Other short-lived children still inherit fd 3 (awk, stat, sha256sum, rustc --version). The spec wording is ambiguous; the draft limits its claim to the two children.
- *Correctness note (low).* Stale temporary files from an earlier invocation are not removed before the child runs. Suppose a runner is killed while its child is running. That id then has no summary row, so a rerun into the same run directory is allowed, and it would report `leftover yes` for a file the new child did not leave. Removing matches in the earlier `rm -f` step would avoid this.
- *Not exercised.* No negative case or test covers a guarded failure or the closed fd 3. The spec does not require one.

**Record script (scripts/passthrough-record.py).**
- SUMMARY_HEADER gains `leftover` last, the same order as the runner.
- load refuses a `leftover` other than yes or no.
- The files-by-outcome table gains a leftover count. The header and separator widths agree (3 + len(OUTCOMES)).
- The non-ok list now also includes `leftover == yes`. The header, separator and "none" row are all 10 cells.
- `--allow-single` is a store_true flag. The refusal `len(labels) == 1 and not args.allow_single` runs after load and the output-directory check, and before any write. The message names only the label.
- The zero-label case depends on load, which the diff does not show.
- No bug found.

**sleep-copying fault.**
- It is parsed in `fault()` next to the other values, so it passes the same guard as the other faults. `inject` does nothing for it.
- `copy` sleeps 30 s after `raw_copy_file` of index 0. This matches ruling (1).
- **Unrecorded addition:** the code also sleeps for an empty archive ("or before the comment"). This is beyond ruling (1). The draft mentions it under "What changed" but not under departures.
- The faults.rs test (one test, through the binary):
  - spawns with the fault and the allow variable;
  - after 1 s asserts the child is still running and the directory holds exactly `[.out.zip.passthrough-<pid>.tmp, in.zip]`;
  - after kill and wait asserts the same two names.
  - The first two assertions can fail. The last one documents that SIGKILL leaves the file, which cannot really fail for a kill (low).
  - The test uses a fixed 1 s wait, so a slow debug start could make it flaky (low).
- Timing in the negative case:
  - The kill lands during the sleep after the first entry, which is the small `[Content_Types].xml` (gen.py parts order, N 184 to 206).
  - The 32 MiB stored parts come after it and are never reached before the kill.
  - So the "large generated archive" of the P2 text plays no part in the timing. Ruling (1) allows this, but the draft does not say it (cosmetic).

**Log-script assertions that are weak or cannot fail.**
- size.log line 84, "without the xml feature quick-xml is not in the tree", greps a separate `cargo tree ... 2>/dev/null` output and ignores that command's exit status. If cargo tree failed, the count would be 0 and the check would print yes. The printed tree just above (Z 227 to 238) came from a separate invocation (low).
- negative.log case 12, "the single-label md has the host family table and no i686 table", counts headings matching `(host|i686)` equal to 1. It would also pass with only an i686 table (cosmetic, since the input copy holds only host files).
- negative.log case 13 compares file sizes only. With page-aligned stripped ELF sizes, a 0-byte delta does not show identical code. A sha256 comparison would. The feature listing at N 949 and 950 (only `quick-xml feature "default"`) does support an empty default set (low).
- gate.log's licence section is headed "the same cargo metadata call and allowed list as check.sh step 6". The script applies no allowed list: it only greps for `quick-xml 0.42.0: MIT`. That check.sh step 6 uses `--all-features`, and so actually sees quick-xml, rests on this comment. check.sh is recorded only by hash (G 120) (low; planning should confirm from check.sh).

## 2. Do the logs contain every required line?

- **First-line rule:** met.
  - gate, test, negative and size logs each start with `about: T3 P2 phase commit 2845fab...; HEAD at run: 2845fab...`.
  - scan.log starts with `about: phase diff 33c0198..2845fab...; HEAD at run: 2845fab...`.
- **Reruns:** no log carries a `rerun:` or "stopped by the environment" line. Nothing in the primaries indicates a rerun.
- **i686 runner runs:** cases 1 to 8 with `--label i686` and the i686 debug builds (N 622 to 721). The binaries are shown as `ELF 32-bit LSB executable, Intel i386` (N 493).
- **Merged record:** case 11 (N 833 to 914).
  - stdout reads `labels host,i686; files 8,8; entry rows 8,8`.
  - Both outcome rows, both family tables, the non-ok list with the leftover column, and `35 lines` for the tsv.
- **Size record** (R, echoed at Z 297 to 326) has everything the spec lists:
  - the commit 2845fab9ba5ebbea759313de66c7eda2699b84e6;
  - the date;
  - the toolchain `rustc 1.98.1 (48a229cea 2026-09-01); cargo 1.98.1 (797e8a9bc 2026-08-05)`;
  - the target and the profile;
  - zip 8.6.0 with features and quick-xml 0.42.0 with none;
  - 473636 and 539172 bytes, and the difference 65536.
  - The measured values are at Z 172 and 173, Z 222 to 224, and Z 279 to 281.
  - Feature lines are at Z 239 to 245 and Z 260 to 267.
  - My own sha256 of R is 41e0d747..., matching S 507.
- **Licences with quick-xml in the tree:**
  - G 170 to 177: the all-features tree adds exactly quick-xml v0.42.0.
  - G 190: `quick-xml 0.42.0: MIT`.
  - G 196: the check.
  - G 208: `PASS licences`.
  - The indirect link to check.sh is noted under question 1.
- **Zip feature and licence tree "from the gate's own listing":**
  - The tree listing is in gate.log (G 136 to 155), run by the gate-log script rather than printed by check.sh.
  - The zip feature listing is only in size.log (Z 239 to 245), not in gate.log. This is not named as a departure (low).
- **Negative cases P2 lists:**
  - Defaults-on build in a scratch clone with its own target directory: N 933 to 952, `delta 0 bytes` at N 947, as ruling (2) asks.
  - Single-label refusal and `--allow-single`: N 916 to 931.
  - Leftover case under `--timeout 2` for both labels: N 606 to 620 and N 707 to 721. Each shows timeout, exit 124, signal 9, leftover yes, no temporary file, no output and no survivor.
  - The P1 cases and cases 9 and 10 are present as well.
- **Scratch-copy rule:**
  - Cases 1 to 12 run the main tree's scripts and binaries, with all outputs under <scratch>, and the tree status is checked clean (N 952, 954).
  - specs/0001 "How this task runs" says every negative test runs in a scratch copy. The draft's departure 5 mentions only the clone for case 13 (low).
- **Checks evaluated once, compound checks in one function:** met in all five scripts. Examples: summary_is, stdout_is, alloc_recorded, refused_single, single_written, passed_matches, static_i686_stripped, probe_output_is, files_complete, messages_complete, scan_passed.
  - I found no `$(yn [ ... ] && cmd)` form.
  - Case 13's "the main tree is unchanged" (N 952) and the final status check (N 954) test the same thing twice under different labels (cosmetic).
- **Control and non-ASCII bytes:** my tr counts over all five logs and R give 0 control bytes and 0 non-ASCII bytes. scan.log counts the other four logs and R with positive controls (S 252 to 309).
- **Paths:**
  - Only `<repo>`, `<scratch>`, repo-relative paths and system tool paths appear.
  - The only home-like strings are in scan.log's printed script: line 39 (a pattern) and line 141 (a fictional `/home/u/` control).
- **Fixture text and original names:**
  - The sentinels and phrase are built at run time. The phrase is split in the printed script (N 63).
  - Every runner output and record reports 0 fixture lines (N 528 and the `runner output` line of each case, N 830, N 914).
  - I saw no original name or content phrase in any log.
- **Hashes:** my sha256 of gate, test, negative and size logs equals S 471, 480, 489 and 498.

## 3. Does every number and quoted string in the draft appear in a primary?

**Checked and found (the draft's labels: G gate.log, T test.log, N negative.log, Z size.log, S scan.log, R the size record, D the diff):**
- **Diff and commit counts:**
  - D: 11 files, 332 insertions, 47 deletions.
  - By file split: 8bbda4d 2 files 81/9; 2845fab 9 files 251/38. Commit file lists at S 315 to 331.
- **Hashes:** S 471, 480, 489, 498, 507.
- **Messages:** Co-Authored-By at S 449 and 459; two messages at S 468.
- **What changed and Size baseline sections:** consistent with D and with Z 172, 173, 222 to 224, 268, 269, 279 to 283, 293 to 295 and 327.
- **G citations:** 135 to 155, 170, 171, 177, 179 to 196, 198, 199, 203 to 210, 214, 223, 224.
- **T citations:** 118, 134, 136, 155, 167, 179, 186, 191, 196, 199, 225, 241, 243, 262, 274, 286, 293, 298, 303, 306, 314, 317, 321, 324, 328 to 330, 332, 333.
- **N citations:** 4, 492, 493, 498 to 511, 521 to 620, 622 to 721, 531 to 534, 632 to 635, 545 to 547, 646 to 648, 558, 659, 569, 670, 580, 681, 591, 592, 692, 693, 603, 604, 704, 705, 609, 615 to 620, 710, 716 to 721, 723 to 744, 818 to 831, 839 to 914, 916 to 931, 933 to 952, 954, 956.
- **Values:** 33557382 (N 505); 35 tsv lines (N 903); 539172 and delta 0 (N 937, 947).
- **S citations:** 309, 445, 468, 516, 518, 521, 522.

**Citation slips (cosmetic):**
- "0 stderr bytes, exit 0 (G 211, 213)": the `exit: 0` line is G 212. G 213 is the check.
- "case 11 ... (N 839 to 914)": the command, stdout and exit lines start at N 834.

**Marked as session output, so not checkable from primaries:**
- nothing pushed;
- the real corpus not touched;
- "committed without editing" for 8bbda4d (its counts can be checked; "without editing" cannot);
- the counts beside the two commits (these can in fact be checked, above);
- the page-alignment reading of the 65536 difference.

**Unmarked claims no primary supports:**
- Line 3: "The T3 P2 phase commit and its logs are committed locally". The record commit is not in any primary; its hash is left for the check header.
- Lines 5 and 6: "the P5, P6, T3 P1 and T3 P2 records go with the next authorised push".
- Lines 167 and 168: "you said you would add it after this report". Neither quoted ruling says this.
- Line 171: "the spec wording to follow".
- Departure 3: "the gate's native and licences steps (which use `--all-features`)". Only the gate-log script's comments support this (G 69, 77); check.sh content is not a primary.
- Size record "(as `xlsx-diff`)" and the report's "`zip` exactly as `xlsx-diff`" (line 42): xlsx-diff's manifest content is not in any primary.

## Defects

1. **low:** The runner does not remove stale `.out.<label>.<ext>.passthrough-*.tmp` files before starting the child. A file left by an earlier, killed invocation into the same run directory would be recorded as `leftover yes` for a child that left nothing.
2. **low:** The acceptance item "licences with quick-xml in the tree" is shown only indirectly.
   - gate.log's check greps the listing for `quick-xml 0.42.0: MIT` and applies no allowed list, although its heading says "allowed list".
   - That check.sh step 6 reads `--all-features` rests on a script comment; check.sh content is not a primary.
3. **low:** The empty-archive sleep in `sleep-copying` ("or before the comment when there is none") goes beyond ruling (1) and is not listed as a departure.
4. **low:** The zip feature listing is cited from size.log, not from the gate's own listing as P2 says. Not listed as a departure.
5. **low:** "zip exactly as xlsx-diff" (spec, report line 42, size record) cannot be checked from any primary.
6. **low:** Negative cases 1 to 12 run the main tree's scripts and binaries, and case 13 builds in the main tree. Only outputs and the defaults-on build go to scratch, while specs/0001 says every negative test runs in a scratch copy. Departure 5 does not name this.
7. **low:** size.log's check "without the xml feature quick-xml is not in the tree" ignores the exit status of the cargo tree run it greps (stderr discarded), so a failed command would print yes.
8. **low:** Case 13 compares file sizes only. Given page-aligned stripped sizes, a 0-byte delta does not show identical output; a sha256 comparison of the two binaries would.
9. **low:** No test or negative case exercises the new `|| die` guards or the `3>&-` closing. The diff never shows that fd 3 is the saved stderr descriptor.
10. **low:** Unmarked draft claims without a primary: report line 3 (record commit made), lines 5 and 6 (P5, P6, T3 P1 and T3 P2 records go with the next push), lines 167 and 168 ("you said you would add it"), line 171 ("the spec wording to follow").
11. **cosmetic:** In the leftover case the kill lands after the first small XML entry, so the 32 MiB archive plays no part in the timing. Ruling (1) allows this, but the draft does not say so.
12. **cosmetic:** Case 12's single-label heading check (count of host-or-i686 headings equal to 1) would also pass with only an i686 table. The faults.rs assertion that files remain after SIGKILL cannot really fail. The faults.rs test's fixed 1 s wait could make it flaky on a slow host.
13. **cosmetic:** Citation slips: G 211, 213 for "exit 0" (the exit line is G 212), and N 839 for the start of case 11 (it starts at N 834). negative.log checks main-tree status twice (N 952 and 954).
