# T1 P3 independent check 2

- Checked commits: 9f124c7f0925c6a96dd1fecc3ccf72790b9c8e6e (T1 P3 phase
  commit 1) and 401bb98f19d666475448e03c292aff038e7182c8 (phase commit 2,
  the fixes for check 1)
- HEAD when the check ran: 62eb360807129ee4a3fe0342e945de2fb3831c18
  (record commit with the logs rerun at 401bb98, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, and asked to answer in plain ASCII
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    3fa9b6a51a2e762cc4a9aae48d7122cbc6f8093d7cf4c1a9b44953d7633e9c3f
  - specs/0001-bootstrap.md
    4b829d12609e6c5624849a7a226a80188bb8bdf9deb39132406247a505d6e2cb
  - phase diff 1, `git diff f0adb45..9f124c7`, saved as <scratch>/t1p3-diff.txt
    d72bf90a97f5c6e0bc37a48b8c0638ab9874edb51b3c34c1c4d830458a26abd1
  - phase diff 2, `git diff 88eaaeb..401bb98`, saved as <scratch>/t1p3-diff2.txt
    075d3a49e222da2b1bf3fd244d3d08f4952fa1c7d21e7d7824f6e7036ce58f3e
  - draft report 2, <scratch>/t1p3-report-draft2.md
    a76b122601e26c7a68b3ece0d8d58071cdb0aab370f6755f660cc41d958952f1
  - records/logs/t1-p3/gate.log
    04d8533098308bd02f18362a9eccb520142bc5dae4474cdc1e305daff7a3fcc3
  - records/logs/t1-p3/test.log
    24fbe227a9a4d0b00fe78b4a7c498b113f1d705dfb7d0eccf237a95581389857
  - records/logs/t1-p3/negative.log
    d4a85b48e37585c8f6682d705ffba54db86c7e83c57c3b30309a1bc50cf34c6f
  - records/logs/t1-p3/scan.log
    e66b0087819b38a094c9ab20fe9977c89f184de2c2b3a44ffc9387d8e42d83de
  - records/logs/t1-p3/independent-check-1.md
    b6e1f6da547d08fb32bf3a4b9387abb770168dcb46e9de870d797a917b08b74e
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
I read the ten listed inputs and nothing else as evidence: the P3 section of specs/0004 (with Output, Exit status, Streaming, "Planning rulings on P2" and the Phases preamble), "How this task runs" and "Rules for every phase" in specs/0001, D1 (1361 lines), D2 (152 lines), draft 2, G, T, N, S and I1, including the scripts printed in the four logs. The harness also put CLAUDE.md and memory into my context; I did not use them as evidence. I ran only read-only tools. I ran `sha256sum` over the five log files and both specs. I counted the diffs: D1 has 5 `diff --git` headers with 888 added and 122 removed lines; D2 has 2 headers with 124 added and 2 removed lines (lib.rs +2/-2, p3.rs +122/-0); `+++`/`---` headers are excluded. I counted control and non-ASCII bytes in the five log files and the draft (all 0). I searched the logs for `/home/`, `/tmp/`, `/mnt/` and `.claude` (hits only in S's own pattern and control lines, S 39, 40, 131, 132, 137, 140, 141, and in I1 30's prose). I used a short python3 calculation to work out byte offsets and 64 KiB buffer numbers for a D2 fixture. I did not read lib.rs or the test files outside the diff hunks, so unchanged code (the `break` in `central_records`, the content loop, `meta()`, `pattern()`, `flipped()`, `assert_columns`, the Error Display texts) cannot be checked here.

## 1. Do D1 and D2 together do what P3 says, and nothing else?

**Verdict: yes for the code. Partly for the tests: one D2 test does not prove what it claims (defect 1 below).**

**Archive rows (D1 531-552, D2 11-12).** All 21 keys are written once each, in the spec's table order.
- `flagged` counts entries whose flags string is not empty (D1 493-496).
- `order_preserved` (D1 475-485) walks the input in index order and sets `no` when a matched output index is lower than the previous one. Duplicate raw names are rejected at open (D1 132-136), so output indices are distinct, and "rising" equals "same relative order". Dropped and added entries do not take part.
- `comment_equal` compares `zip.comment()` bytes (D1 180, 526).
- `prefix_*` comes from `zip.offset()` (D1 181); `central_start_*` from `central_directory_start()` (D1 126).
- `uncompressed_*` and `compressed_*` are `u64` `checked_add` sums of the crate's `size()` and `compressed_size()` (D1 423-431, 162).
- `bytes_*` is the `u64` metadata length (D1 116-119).
- `files_identical` (D1 437-457) is `no` on unequal lengths. Otherwise it compares two 64 KiB buffers, stops at the first short read, and handles an exact multiple of 64 KiB through the following 0-byte read.

**Column 17 (D1 339-362).** The order is mtime, extra, gpflags, descriptor, attrs, comment, as in the spec.
- `mtime`: central (time, date).
- `extra`: local extra field or central extra field.
- `gpflags`: all 16 central flag bits.
- `descriptor`: bit 3 differs.
- `attrs`: external attributes or version-made-by (the system is its high byte).
- `comment`: raw central comment bytes.

Flags are computed only for matched entries; dropped and added rows pass "" (D1 512, 520). Each definition matches the spec. `descriptor` uses bit 3 rather than the crate's `using_data_descriptor`, which is the same bit.

**Central record parsing (D1 198-255).** The offsets are right: version-made-by 4, flags 8, time 12, date 14, name length 28, extra length 30, comment length 32, external attributes 38-41. The name, extra field and comment are read one after another from the stream, which is their on-disk order, and a short read is an error (D1 231-237). The per-iteration seek was removed safely, because the records are contiguous.

**Local header (D1 259-283).** The fixed length 30, the signature, name length at 26 and extra length at 28 are right. It is positioned by `header_start()`, which must be absolute: `leading_junk_bytes` (D1 1330-1361) would hit a signature error (exit 2) if it were not, and it expects exit 0. One edge: with an extra length of 0 and a name past EOF, no error is raised there (as I1 said); harmless.

**Exit status (D1 554-559).** 0 needs recompressed, changed, dropped, added and flagged all 0, plus `order_preserved` and `comment_equal`. That equals "every row identical with empty flags, order yes, comment yes". Prefix and `files_identical` do not affect it, which matches the spec.

**Column 14 (D1 329, 372).** `first.unwrap_or(min)`, per the P2 ruling. Columns 15 and 16 stay empty for `Length`.

**Error mapping.**
- A local header failure is `Error::Entry(side, name)` (D1 265).
- A sum overflow is `CentralDirectory` (D1 424).
- A read error in `files_identical` is `Error::Open` (D1 441-449).
- The crate-versus-record guard is `CentralDirectory` (D1 148-153).

Nothing new prints content, a comment, an extra field or a path: flags are fixed words and names go through `escape_name`.

**u64.** Sizes, sums, prefix, central start and file lengths are `u64`. D2 restores `as u64` for `entries_in` and `entries_out` (D2 11-12). `usize` remains only for indices and buffer lengths.

**Required tests.** All are present.
- Reordered entries (D1 1061-1105): every entry `identical`, `order_preserved no`, exit 1.
- Changed archive comment (D1 1108-1131).
- mtime, extra field, comment and external attributes, each alone (D1 1134-1237). Each case asserts the exact flag string, so "alone" is proven.
- Data descriptor on one side (D1 1240-1263), in both directions.
- `files_identical` yes and no (D1 1298-1327, D2 20-48).
- Non-zero `prefix_in` (D1 1330-1361).

**P2-ruling test changes.**
- `changed_entry_with_different_lengths` expects 600, 600 and 5, with both sizes and `assert_columns` on every row (D1 922-932).
- The large-entry test adds `assert_columns` (D1 950).
- `entry_of_exactly_two_buffers` adds stderr, row-count and status assertions (D1 975-979).

**Byte builder (D1 729-798).** Correct.
- Local header: 30 bytes; with bit 3, 12 zero bytes, then a descriptor with signature, CRC and both sizes after the data.
- Central header: 46 bytes.
- EOCD: 22 bytes plus the comment.
- Offsets are relative to the end of the prefix.
- The rewritten `single_deflate_entry_zip` gives the same bytes as the removed code (version 20, date 0x0021, the same 16 trailing bytes, the same EOCD).

**Does D2 close I1's defects?**
- **I1 defect 1: closed.** `output_side_rows_take_the_output_values` (D2 56-112) makes every in/out pair differ, checked at D2 104-106, and catches a swapped `uncompressed_out`/`compressed_out` (D2 107-111). A swap of `uncompressed_in` with `compressed_in` is not caught there (the input is one stored entry, so both are 12), but `every_archive_row_in_order` catches it (D1 1057).
- **I1 defect 2: closed.** D2 115-151 covers 140000, and 69999 on the last byte of the shorter stream past the first buffer. That catches a first offset set too early or too late. The 150000 "agree" case cannot tell `unwrap_or` from a loop that sets `first` at the shorter length itself, but both give the same output.
- **I1 defect 5: closed** (D2 11-12).
- **I1 defect 6: only partly closed.** The different-length case now asserts exit 1, empty stderr and unequal `bytes_*` (D2 24-30). The new "middle buffer" case (D2 32-47) does not isolate a middle-buffer difference. Flipping data byte 100000 of a stored entry also changes its CRC-32, which `raw_zip` writes into the local header at file offset 14 (buffer 0, D1 741, 750) and into the central record at offset 200053 (the final buffer, D1 775). My calculation: file length 200123, buffers 0-65535, 65536-131071, 131072-196607 and 196608-200122; the flip is at 100037 (buffer 1). So every "no" case in the test also differs in the first or the last buffer. A `files_identical` that compared only the first and last buffers would pass every test. A difference placed inside a long prefix, which no CRC covers, would isolate it.
- **I1 defect 3:** not changed; recorded as departure 8.
- **I1 defect 7:** recorded as a finding.

**Beyond scope, all inside tools/xlsx-diff:**
- The crate-versus-record guard.
- Reading every entry's local header at open, a new exit-2 path even for dropped or added entries (departure 3).
- Extra tests (departure 5).
- The two helpers `recorded_central_offset` and `central_version_made_by`.
- Doc comments.
- The `recompressed_entry` precondition and its `attrs` expectation (D1 859-886). That expectation is a consequence of the P3 `attrs` definition, not an unrelated change.

## 2. Do the logs contain every line the spec requires?

**Verdict: yes for the required content. Partly for the rules, on two points.**

**Gate log.** Full check.sh with eight PASS in order (G 132-139), exact-match check (G 143), `PASS native` (G 136), `PASS licences` (G 137).

**Test log.** Host and i686 with colour off (T 3). Test names and summary lines (T 83-125, T 136-178). Both counts (T 185), same names (T 186).

**Negative log.**
- Case (a), files differing only in the archive comment: exit 1 (N 217, 219); three rows `identical` with an empty flags column (N 192-194, 220); `comment_equal no` (N 204, 221). N 142 and N 187 establish "differ only in the comment".
- Case (b): exit 0 (N 255, 257) and `files_identical yes` (N 253, 259).
- The fixture counters have positive controls (N 172-175). Captured outputs show 0 fixture lines, 0 control bytes and 0 non-ASCII bytes (N 218, 256). Sentinels and the phrase are built at run time (N 40-42).
- The `entry_rows_unflagged` regex (N 113) consumes exactly 16 tabs with column 17 empty, which is correct.

**Scan log.** It covers the 13 diff files (S 480), the 5 messages (S 533) and the 3 other logs (S 563), with controls (S 308), and ends `RESULT: PASS` (S 569).

**Rules from specs/0001.**
- **First line:** G 1, T 1 and N 1 name the commit and HEAD. S 1 names the range ending at the commit, plus HEAD.
- **Each check evaluated once and printed once:** yes in all four scripts.
- **Compound checks in one function:** yes. `passed_matches` (T 26); `all_zero`, `entry_rows_unflagged`, `equal_before_comment_length` (N 59, 111, 117); `files_complete`, `messages_complete`, `scan_passed` (S 195, 212, 232).
- **Control bytes:** 0 in G, T, N, S and I1 (my count).
- **Paths:** `<repo>`, `<scratch>` or repo-relative. `$HOME/.config` appears only as literal script text (G 5, S 20).
- **Fixture text:** none in captured output. Entry names are metadata from the printed generator.

**Rule points that are only partly met.**
- **Negative runs in the main working tree.** The build and run happen in the main working tree (N 23, 32, 83, 92). specs/0001 lines 67-69 say a negative test never runs there. specs/0004 line 164 allows "or against temporary files", and the archives are temporary (N 28). Draft departure 8 leans on P2 gap 6; the primaries show only that gap 6 was accepted (specs/0004 332-336), not what it was.
- **Rerun headers.** The four logs replace the committed first-run logs from 171f980 (S 332-337), but no header says so or names the fix commit's purpose. The specific rule ("the rerun's header names that log") applies to a failed run, and nothing in the primaries shows the first run failed, so this is not a strict breach. specs/0001 lines 62-64 ("re-run with the fix named in the log") arguably applies, because the rerun follows check 1's defects.
- **Minor.** T prints the label `exit is 0` identically for both targets (T 81, T 134); only the section header tells them apart. The rerun record commit's message is scanned by no log, which is inherent.

## 3. Does every number and quoted string in draft 2 appear in a primary?

**Verdict: nearly all do, and every line citation I checked points at the right line. One statement is misleading, one is overstated, and several items cannot be checked from primaries.**

**Primaries section.**
- D1: 5 files, 888 insertions and 122 deletions match my count and I1 30.
- D2: 2 files, 124 insertions and 2 deletions match my count of D2. The draft marks this "(session output)", but it can be checked and is correct.
- The sha256 values match both the files and S: G 04d85330... = 04d8533098308bd02f18362a9eccb520142bc5dae4474cdc1e305daff7a3fcc3 (S 536); T 24fbe227... = 24fbe227a9a4d0b00fe78b4a7c498b113f1d705dfb7d0eccf237a95581389857 (S 545); N d4a85b48... = d4a85b48e37585c8f6682d705ffba54db86c7e83c57c3b30309a1bc50cf34c6f (S 554).
- S itself hashes to e66b0087819b38a094c9ab20fe9977c89f184de2c2b3a44ffc9387d8e42d83de; the draft defers this to the check header.
- I1 hashes to b6e1f6da547d08fb32bf3a4b9387abb770168dcb46e9de870d797a917b08b74e, which matches S 381.

**Commits section.**
- 9f124c7f0925... matches S 314/315.
- 171f980, 88eaaeb and 401bb98f19d6... match S 312-316, 332, 339, 343 and G 1.
- "S 484 to 524, five messages, S 533" is correct.
- "f0adb45 committed without editing" is session output and unverifiable. S 318-322 shows only the three doc files, which does not contradict it.

**What changed.**
- Every D1 statement matches: D1 116-119, 126, 180-181, 339-362, 423-457, 475-485, 526-559, 571-624, 674-815, 829-979, 1004-1361.
- D2 bullets 1 and 2 match D2 11-12 and 115-151.
- D2 bullet 2's "values read independently of the tool" is only partly checkable: the sums come from `meta()`, which lies outside the hunks.
- **D2 bullet 3 is misleading (draft 105-108).** "differs by one byte at entry offset 100000, in a middle buffer of the file": the files also differ in the local CRC-32 (buffer 0) and the central CRC-32 (final buffer), as shown in answer 1.
- **The response to I1 defect 6 is overstated (draft 206).** "fixed in D2" should be "partly fixed".

**Acceptance.** Every citation is correct.
- G: 125, 128-129, 132-139, 143, 140-142, 152, 153.
- T: 99, 111, 123, 126, 112, 165, 117, 121, 169, 174, 152, 164, 176, 179, 185-187, 189, 190.
- N: 4, 186-187, 192-194, 220, 202-204, 221-223, 215, 224, 217, 219, 225, 230-232, 258, 242, 260, 251-252, 253, 259, 255, 257, 261, 218, 256, 264, 265.
- S: 569, 308, 480, 533, 563, 565, 568.

**Departures.**
- **1:** the zip read.rs line numbers are session output and unverifiable. The guard claim matches D1 148-153.
- **2:** 0x0314 and 0x030a appear only in the D1 comment (D1 859-860). The test asserts only not-equal and equal (D1 865-874), so the values are unverifiable but consistent.
- **3 and 4:** the quoted error texts are not in any primary and are unverifiable. The mappings match D1 265 and 441-449.
- **5:** matches D1 1143-1197, 1266-1295, 1096-1104; bit 11 is 0x0800 at D1 1177.
- **6:** matches D1 1259.
- **7:** session output, unverifiable.
- **8:** matches N 23, 28, 36-37, 83. "P2 gap 6, which you accepted" is confirmed only as "accepted" (specs/0004 332-336); what gap 6 was cannot be checked.

**Findings.** Two imprecisions.
- The draft files "version needed to extract" under the local file header only. It is not compared in the central record either (I1 96: "compared nowhere").
- "That is what the Output section says" is stronger than the primaries support. The Output section does not say which header `mtime` or `gpflags` read; I1 said only that the wording "allows" it.

**Independent checks section.** Matches I1 157-165, and the draft 1 wording about `archive_rows` is gone from draft 2.

No unverifiable item contradicts a primary.

## Defects

1. **The `files_identical` "middle buffer" case does not test a middle buffer** (D2 32-47). Flipping a data byte changes the CRC-32 in the local header (file offset 14, buffer 0) and in the central record (offset 200053, final buffer). Every `no` case therefore differs in the first or the last buffer, and an implementation comparing only those two would pass. I1 defect 6 is only partly closed, and the draft (105-108, 206) overstates it. Suggested fix: put the one-byte difference inside a prefix longer than 131072 bytes, which no CRC covers.
2. **The negative script builds and runs in the main working tree** (N 23, 32, 83, 92), against specs/0001 lines 67-69. It is carried as departure 8, and the primaries confirm only that "gap 6" was accepted, not its content. Planning should confirm that ruling covers P3.
3. **The rerun logs do not say what they replace.** None of G, T, N or S says it supersedes the 171f980 logs or names the I1 fixes. This is not a strict breach of the failed-run rule, but specs/0001 lines 62-64 arguably asks for the fix to be named.
4. **The draft's findings paragraph is imprecise.** It says version-needed-to-extract is a local-header-only gap (it is compared in neither header), and that the behaviour "is what the Output section says" (the section only allows it).
5. **Minor.** T labels both targets' exit checks `exit is 0` (T 81, T 134). The draft marks D2's file and line counts "(session output)", but they can be checked from D2 and are correct.
