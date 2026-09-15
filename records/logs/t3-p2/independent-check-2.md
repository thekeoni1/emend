# T3 P2 independent check 2

- Checked commits: 2845fab9ba5ebbea759313de66c7eda2699b84e6 (T3 P2 phase
  commit 1) and 6a7533e6e47656111f69e2b08adab9454a75dbc4 (phase commit 2,
  the fix for check 1)
- HEAD when the check ran: ca4f1954b5b967bd364b3a975aa8965b9645bc31
  (record commit with the logs rerun at 6a7533e and the size record, local
  only)
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
  - phase diff 1, `git diff 33c0198..2845fab`, saved as <scratch>/t3p2-diff.txt
    7fa2e5f92cda077c959c63d85f23d56ce381d1713bc72e70a8eaabf6dce8c0e3
  - phase diff 2, `git diff dd1c4cd..6a7533e`, saved as <scratch>/t3p2-diff2.txt
    95cc315cf514a530e78932fd40ecd2a794ddb075e2fcf904416a16b91189d745
  - records/logs/t3-p2/independent-check-1.md
    a876f2b51e8cdb0932d456d180685cb0cf35dbc81e8b1d13b9c9b03d1e670a2a
  - draft report 2, <scratch>/t3p2-report-draft2.md
    8696451eae8556dd19328efada1bef9c145c25a409f196dff92e3ad08929e159
  - records/size-baseline-2026-09-15-6a7533e.md
    0e0dd78edcec34ef0efbcb3b116158c751c69949c44834916796fa7f976aeea9
  - records/logs/t3-p2/gate.log
    344716ff2e5bacf3b14ec5a60d917bacffa9a15988bca5d812a3c3f947284719
  - records/logs/t3-p2/test.log
    683c935e120d1f4ba2f588610c8365bfe0951344f46ee7b678349b4615a51497
  - records/logs/t3-p2/negative.log
    cb2507920d23c89cd1d08ecb2bd41556e1d5191d3971a1de1e08cbfeb5fa8e48
  - records/logs/t3-p2/size.log
    973865ce10a8214501940a4b0c18677c000e7ed181fef585416a2da40baff0f2
  - records/logs/t3-p2/scan.log
    9e85eadca5f58991243351c06680e174411b6c7bb730ae7fbb6dadd39a740ffe
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
The harness put CLAUDE.md, a memory index (MEMORY.md), a git status snapshot, commit-attribution text and a user email address into my context. I did not use any of them as evidence. My evidence was only the files named in the task and the two quoted rulings. I ran only read-only commands (Read, sed, grep, wc, sha256sum, tr, python3 for counting).

Hashes I computed of the inputs as they stand:
- G gate.log 344716ff..., T test.log 683c935e..., N negative.log cb2507920d23..., Z size.log 973865ce..., R size record 0e0dd78e.... These match S 592, 601, 610, 619 and 628.
- S scan.log 9e85eadca5f58991...
- I1 a876f2b5... (matches S 410).
- D1 7fa2e5f9... (the same diff I1 checked).
- specs/0006 f1fd88f3... (unchanged since I1).
- Every log, R, I1, D1, D2 and the draft have 0 control bytes and 0 non-ASCII bytes.

## 1. Do the two diffs do what P2 says (with the two rulings), and nothing else?

**Scope.** My python3 counts:
- D1: 11 files, 332 insertions, 47 deletions. ROADMAP.md (8/3) and specs/0006 (73/6) make 81/9. The other 9 files make 251/38.
- D2: 1 file (scripts/passthrough-run.sh), 6 insertions, 0 deletions.
- S 319 to 352 shows the record commits 434e714 and dd1c4cd touch only records/. Nothing outside P2 and the two rulings is in the code.
- Cargo.lock adds only quick-xml 0.42.0 (depending on memchr) and size-probe.

**tools/size-probe and the size profile.**
- Two binaries, `publish = false`.
  - probe-zip: `ZipArchive::new`, prints `entries <n>`.
  - probe-zip-xml: `by_name`, then `quick_xml::Reader::from_reader` and `read_event_into`, counting Start and Empty events.
- zip is `=8.6.0`, `default-features = false`, `deflate-flate2-zlib-rs`. G 219, 221 and 224 now show that line is identical in all three tools, and G 226 checks it.
- quick-xml is `=0.42.0`, `default-features = false`, optional, behind feature `xml`, with `required-features = ["xml"]` on probe-zip-xml.
- `[profile.size]` has exactly the seven keys the spec lists.
- Departures: required-features instead of "on for that binary only" (draft departure 3); labelled output and exit 2 (departure 4). Both are named.

**Runner fixes.**
- *Leftover column.*
  - After the child ends, the glob `"$dir/.out.$LABEL.$ext.passthrough-"*.tmp` is removed with `|| die` and sets `leftover=yes`. With no match, the literal pattern fails `[ -e ]`.
  - The printf has 14 `%s`: 12 fields, then `$values` (9 tab-joined), then `$leftover`. That is 22 columns, matching the 22-name header (N 886 to 905).
- *Stale temporary files (D2).*
  - The same glob is removed before the child starts, after the earlier-output `rm -f`, without touching `leftover`. This closes I1 defect 1 in code.
  - No test or negative case puts a stale file in place first, so the fix is not exercised.
- *Guards.* Every item the spec lists is guarded:
  - the `--ids` scratch file (creation, each append, and the read loop via `done <"$IDS" || die`);
  - the selected-list awk and `cp`;
  - the removals (earlier outputs, stale temporaries, leftovers);
  - a new header's write (header_ok now dies itself);
  - also the two sha256sum pipelines (pipefail makes these work) and the archive-value awk.
  - The `while ... done || die` form cannot die falsely: the loop status is its last body command, which is guarded.
  - Commands outside the diff hunks cannot be confirmed from primaries: the per-id `mkdir -p`, the run.<label>.tsv append and the WORK directory creation.
- *Descriptor.*
  - `3>&-` is on the tool's subshell and on the xlsx-diff call.
  - N case 15 (N 1046 to 1073) shows neither child holds a descriptor on the runner's stderr file, with a control (N 1048 to 1055) proving the check detects one. It shows descriptor 3 in the tool position is time's report file (N 1062).
  - The check could fail if 3>&- were missing. It cannot tell "closed" apart from "never saved", but that ambiguity is harmless.
  - Short-lived helpers (awk, stat, sha256sum, rustc) still inherit fd 3. The draft limits its claim to the two children.

**Record script.**
- `leftover` is appended last to SUMMARY_HEADER, and load refuses values other than yes or no.
- The outcome table gains a leftover count; header and separator are both 3 + len(OUTCOMES) wide.
- The non-ok list also includes `leftover == yes`, with 10 cells in the header, separator and "none" row.
- `--allow-single` is store_true. The refusal runs after load and the output-directory check, and before any write.
- No bug found.

**sleep-copying fault and its test.**
- The value is parsed in `fault()` with the others, and `inject` does nothing for it.
- `copy` sleeps 30 s after `raw_copy_file` of index 0, which matches ruling (1).
- It also sleeps for an empty archive. That goes beyond the ruling and is named in departure 1.
- The guard ordering (the ALLOWED check before the value match) is not visible in the D1 hunk. N 681 shows the refusal only for exit3.
- faults.rs has one test through the binary, as ruling (1) asks.
  - The first two assertions (still running after 1 s; exactly the temporary file and in.zip) can fail.
  - The post-kill assertion effectively cannot fail.
  - The fixed 1 s wait could be flaky on a slow host.
  - The draft keeps both on purpose (I1 item 12).

**I1 defects.**
1. Closed in code (D2); untested.
2. Closed. G 229 and 230 show check.sh's own `cargo tree ... --all-features` and `cargo metadata --format-version 1 --all-features`, G 231 to 240 its ALLOWED set, and G 271 `PASS licences`.
3. Closed (departure 1).
4. Closed (G 208 to 217).
5. Closed (G 218 to 226).
6. Partly closed. Departure 5 names cases 1 to 12, 14 and 15 running in the main tree. It does not name that the "## build" step (N 562 to 570) and case 13's first build (N 1011) run cargo in the main working tree. Its "which you accepted" rests on no primary: specs/0006 says only "Departures 1 to 3 and 5 to 9 accepted as recorded".
7. Closed (Z 39 to 44, 91 to 97, 280 to 282).
8. Closed (N 1014, 1020, 1021, 1028, 1029).
9. Partly closed. The closed descriptor is now exercised (case 15). The new `|| die` guards are not:
   - Case 14 exercises the "another header" refusal, and that message and die already existed in P1 (D1 removed lines 247 to 249).
   - Only the refactoring into header_ok is new.
   - None of the new failure guards is triggered: the cp, the scratch-file writes, the removals, a header write failure, the hashes and the awk.
   - The draft's "case 14 (a summary with another header, one of the new guards)" is inaccurate.
10. Partly closed. See question 3 for the unmarked claims that remain.
11. Closed (departure 1).
12. Case 12's heading check is closed (N script 140 to 143). The faults.rs points are left open by choice, with a reason given.
13. Closed. N has one tree-status check (N 1075). One new slip is listed under question 3.

**Assertions that cannot fail, or that are weak.**
- The faults.rs post-kill assertion (above).
- Case 15's check passes whether stderr was closed or never saved (harmless).
- Case 12 has no fixture, path, control or non-ASCII count over its outputs (single.out and .err, and the written md and tsv). Case 11 and every runner case have one.
- Z 85 and 88 print trees from invocations whose exit status is ignored. The checks use separate invocations that require exit 0, so only the display is unchecked.
- R's feature text ("features `deflate-flate2-zlib-rs` (as `xlsx-diff`)", "no features") is literal in the size script (Z 158 to 161), not derived from the listing. G 211 to 226 and Z 251 to 279 agree with it at the same commit.

## 2. Do the logs contain every required line?

**First-line rule: met.**
- G, T, N and Z line 1 read `about: T3 P2 phase commit 6a7533e...; HEAD at run: 6a7533e...`.
- S line 1 reads `about: phase diff 33c0198..6a7533e...; HEAD at run: 6a7533e...`.

**Rerun header rule: met.** G 3, T 3, N 3 and Z 3 name record commit 434e714 (runs at 2845fab), I1's record commit dd1c4cd and what changed. S 3 names 434e714 and the replaced size record. No "stopped by the environment" line appears, and no primary suggests one was needed.

**i686 runs and the merged record.**
- Cases 1 to 8 with `--label i686` and the i686 debug binaries: N 699 to 798.
- ELF 32-bit i386: N 570.
- Merged record: N 910 to 991. It includes `labels host,i686; files 8,8; entry rows 8,8`, both outcome rows with the leftover column, both family tables, the non-ok list with the leftover column (copying rows `yes`) and 35 tsv lines.

**Size record.** R, echoed at Z 311 to 339, has:
- commit 6a7533e6e47656111f69e2b08adab9454a75dbc4;
- date;
- toolchain (Z 184, 185);
- target and profile;
- zip 8.6.0 with its features and quick-xml 0.42.0 with none;
- 473636, 539172 and difference 65536 (Z 292 to 294);
- ASCII (Z 340).

The sizes equal I1's reading of the 2845fab record (I1 line 128). The main-tree probe-zip-xml sha 52d1932f... in N 1014 equals Z 293.

**Licences with quick-xml in the tree.**
- G 185 to 206: the all-features tree adds exactly quick-xml.
- G 229 and 230: check.sh's own commands use `--all-features`.
- G 231 to 240: the allowed list.
- G 253: `quick-xml 0.42.0: MIT`.
- G 271: `PASS licences`; G 266 to 277: eight PASS lines.

**Zip features.** G 211 to 217 (check at G 217), and Z 251 to 279.

**Negative cases.**
- P1 cases for both labels.
- Case 8, sleep-copying: timeout, 124, 9, leftover yes, no temporary file, no output, no survivor (N 683 to 697 and 784 to 798).
- Defaults-on case 13: delta 0 bytes shown at N 1028, as ruling (2) asks.
- Single-label refusal and `--allow-single`: N 993 to 1008.
- Added case 14: N 1035 to 1044. Added case 15: N 1046 to 1073.

**Each check evaluated once; compound checks in one function.** Met in all five scripts (summary_is, stdout_is, alloc_recorded, host_table_only, refused_header, refused_single, single_written, lacks_target, tree_has, probe_output_is, features_ok, zip_lines_equal, passed_matches, files_complete, messages_complete, scan_passed). I found no `$(yn [ ... ] && ...)` form.

**Control bytes.** 0 in every log and in R by my count. S 592 to 637 counts the other four logs and R, with controls at S 252 to 310.

**Paths.**
- Only `<repo>`, `<scratch>`, repo-relative paths, target/ paths and system tool paths appear.
- Home-like strings are only in scan.log's printed script: a pattern at S 40, controls at S 133 and 143, and a fictional `/home/u/` at S 142.
- N 1051 prints the basename `t3p2-negative.log` (not a path).

**Fixture text and original names.**
- My grep for sentinel-shaped strings `zq[a-z0-9]{14}` over all logs and R: 0. The joined phrase: 0.
- The phrase is split in the printed script (N 70).
- All 21 runner command lines in N carry `EMEND_CORPUS_DIR=<scratch>/corpus`.
- The one gap is case 12's outputs, which are not counted (question 1).

**Scratch-copy rule.** Departed from (I1 defect 6 above).

## 3. Does every number and quoted string in the draft appear in a primary?

**Checked and found.**
- D1: 11 files, 332/47; by file 81/9 and 251/38; S 319 to 335.
- D2: 1 file, 6 insertions. Marked session output but checkable, and I confirm it. The same holds for 8bbda4d's 2 files, 81/9.
- Hashes at S 592, 601, 610, 619 and 628; messages at S 540 to 580 and S 589.
- Z 184, 185, 234 to 236, 280 to 282, 292 to 296, 306 to 308, 340, 343.
- G 164 to 184, 186 to 206, 208 to 217, 226, 228 to 241, 242 to 259, 261, 262, 266 to 277, 286, 287.
- T 119, 136, 137, 156, 168, 180, 187, 192, 197, 200, 226, 243, 244, 263, 275, 287, 294, 299, 304, 307, 315, 318, 322, 325, 329 to 331, 333, 334.
- N 5, 569, 570, 572 to 588, 598 to 697, 699 to 798, 608 to 611, 709 to 712, 616, 622, 624, 717, 723, 725, 635, 736, 646, 747, 657, 758, 668, 669, 769, 770, 680, 781, 686, 692, 694, 697, 787, 793, 795, 798, 895 to 908, 910 to 991, 993 to 1008, 1010 to 1033, 1035 to 1044, 1046 to 1073, 1075, 1077.
- Values: 58 text files (N 907); 8 files 1/1/2/1/3, 1 leftover (N 933, 934); sha 52d1932f... and 1c0dc0ec... (N 1014, 1020, 1028).
- S 310, 536, 589, 637, 639, 642, 643.
- "sixth fault value": five P1 faults in specs/0006.
- "no high or medium defect" in I1.
- "65536 is exactly 64 KiB".
- "the sizes equal the first measurement at 2845fab": marked session output, but I1 line 128 supports it.

**Citation slips.**
- "case 9 and case 10 on the host (N 800 to 820)": case 10's last check is N 821. Cosmetic.

**Marked as session output, so not checkable from primaries:**
- the record commits exist;
- nothing pushed;
- which records go with the next push;
- the real corpus untouched (partly supported: all runner calls in N use the override);
- "committed without editing";
- "your message said the spec wording follows after this report" (departure 1);
- the metadata-hash reading in departure 2;
- the page-alignment reading of 65536.

**Unmarked claims that no primary supports.**
- Departure 2: "your message said the spec wording follows". Not marked, and neither quoted ruling says it.
- Departure 5: "as the T3 P1 negative log did, which you accepted".
- Departure 3: "the gate's workspace build, test and clippy steps, which pass no features". check.sh's build, test and clippy command lines are not printed (G 103 greps only `cargo tree` and `cargo metadata`). T 138 and the T 70 heading are consistent with it for `cargo test`.
- Case 14 described as "one of the new guards": contradicted by D1 lines 247 to 249.
- I1 item 9: "A first version of case 15 ... the dry run showed it open".
- I1 item 12: "the wait has not failed on either target". Only this run's T 136 and T 243 are in primaries.
- Lines 64 and 192 cite "your ruling (a)". The quoted rulings are numbered (1) and (2).
- Lines 16 and 17: "replaces the one measured at 2845fab". The deletion of records/size-baseline-2026-09-15-2845fab.md is in no primary; S 3 only states it, and S 365 shows the file still present at 6a7533e.

## Defects

1. **low:** The new `|| die` guards are still not exercised (I1 defect 9, partly closed). Case 14 triggers the "another header" refusal that existed in P1, and the draft calls it "one of the new guards". No case triggers a failed cp, scratch write, removal, header write, hash or awk.
2. **low:** The D2 stale-temporary-file removal has no test or negative case that places a stale file and expects `leftover no`.
3. **low:** Departure 5 does not name the main-tree cargo builds: the "## build" step (N 562 to 570) and case 13's first build (N 1011). Its "which you accepted" has no primary.
4. **low:** Unmarked draft claims without a primary: departure 2's "your message said the spec wording follows"; departure 3's claim about check.sh's build, test and clippy steps; I1 item 9's dry-run history; I1 item 12's "has not failed on either target"; the deletion of the 2845fab size record.
5. **low:** Case 12 does not count fixture strings, the scratch path, control bytes or non-ASCII bytes in its outputs or its written md and tsv, unlike case 11 and every runner case.
6. **low:** Guards on runner commands outside the diff hunks (the per-id `mkdir -p`, the run.<label>.tsv append, the WORK directory creation) cannot be confirmed from primaries. Also not visible: that the PASSTHROUGH_FAULT_ALLOWED check comes before the value match for `sleep-copying` (N 681 covers only exit3).
7. **cosmetic:** The faults.rs assertion after SIGKILL cannot really fail, and the fixed 1 s wait could be flaky. The draft keeps both with reasons.
8. **cosmetic:** R's feature text and "(as `xlsx-diff`)" are literal in the size script rather than derived from the listing. G 211 to 226 confirms them at the same commit.
9. **cosmetic:** Citation slip "N 800 to 820" (should end at 821). The draft names the ruling "(a)" where the quoted rulings are (1) and (2).
