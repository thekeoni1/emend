# T1 P4 independent check 2

- Checked commits: 74b2bf0f2328b33a729d46105a33712e495992f1 (T1 P4 phase
  commit 1) and c35112b396407414d9321c1e5eb9d31ff9e76e9d (phase commit 2,
  the fixes for check 1)
- HEAD when the check ran: 53544057f463a72f621920aa45e2e800a9a0b52a
  (record commit with the logs rerun at c35112b, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, and asked to answer in plain ASCII
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    9bbf2b97e0089e0337bf25ba86baf11d77dc00546e43eddde32ed358affc289a
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff 1, `git diff 827766a..74b2bf0`, saved as <scratch>/t1p4-diff1.txt
    69cc2f72e2a5ffb335254b07fbe07d7ad2c678f1a555e86be15bcf191e8f2719
  - phase diff 2, `git diff d09c354..c35112b`, saved as <scratch>/t1p4-diff2.txt
    58b64f16e4b75dd6e3475eb7484515309032c3142fdb03afa07c88863a137894
  - draft report 2, <scratch>/t1p4-report-draft2.md
    a50368b2a86ebb3affb7312c68b43591a6224aaad8f9329630b84b0310d1ef25
  - records/logs/t1-p4/gate.log
    490a8642e3ab60254f69061bc794a2d26a94380abeaa8c2db4afd215bd60d75c
  - records/logs/t1-p4/test.log
    0a076c1ded5eb3aa6afe6c604d5f195624309f0a3b423d86a9608f87f2067a60
  - records/logs/t1-p4/negative.log
    391e6b0ce3cd4d5f68dea1adae134757d590dd9b72b8b149c66b9c634543a414
  - records/logs/t1-p4/scan.log
    4c1d964deb7e21d9e8d3aa16547a66122415d3e155d72357974baf0ddce5756e
  - records/logs/t1-p4/independent-check-1.md
    e27bfa5e0683fe864de53baa93061a50f5c75a5bb62fc6fad2c28ad2bf83fc30
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
I read the ten listed inputs in full: specs/0004-xlsx-diff.md, the "How this task runs" and "Rules for every phase" sections of specs/0001-bootstrap.md (lines 22 to 122), D1 (1130 lines), D2 (128 lines), draft 2 (219 lines), G, T, N, S (with the scripts each prints) and I1. I ran only read-only tools on those files: sha256sum on the four logs, I1 and both specs; grep counts of files, insertions and deletions in D1 and D2; tr counts of control and non-ASCII bytes in the logs, I1 and the draft; grep for /home/ and /tmp/ in the logs; a python count of the sentinel string (built from pieces, never printed), its hex form and the two fictional templates in every log; and a python rebuild of tests/p4.rs from D1 plus D2, to map the panic line in N 222. CLAUDE.md and memory were in my context. I did not use them as evidence. tests/common helpers that are not in the diffs (TempDir, meta, set_crc, cut_entry_tail, patch, flipped, pattern) and the constant LOCAL_HEADER_LEN are not visible, so any claim that depends on them is marked as inferred or unverifiable.

## 1. Do the two diffs together do what P4 says, and nothing else?

**Verdict: partly.** The code and tests deliver every P4 item, D2 is correct, and nothing is out of scope. Three things keep this from a yes: the exit-0 paragraph of the Output section is still not fully met, the sentinel test departs from the spec's wording, and the leak checks remain verbatim-pattern checks.

**lib.rs against `local` and `attrs`**
- **Central version needed:** D1 253 reads `le16(&header, 6)`. In the central header, offset 4 is version made by, 6 is version needed and 8 is flags, so this is correct.
- **`attrs`:** D1 324-329 now compares external attributes, version made by and version needed. That matches spec 98-99.
- **Local fixed part:** D1 291-292 copies `header[4..26]`, which is 22 bytes: version needed, flags, method, time, date, CRC-32, compressed size and uncompressed size. The signature (0-3) and the name and extra lengths (26-29) are excluded, as spec 101-104 requires.
- **Local name and extra:** the name is read with the length at 26-27, then the extra with the length at 28-29 (D1 300-301). Both go through `read_full` with a length check, so a short read is `Error::Entry` (D1 293-299). Allocation is bounded at 65535 bytes.
- **`local` flag:** D1 333-335 compares the fixed bytes and the local name bytes. It is set last, matching the order in spec 94-105. The lengths are covered implicitly because the name and extra bytes are compared.
- **Bugs:** I found no offset or ordering bug. The 30-byte header length is inferred from the indices; LOCAL_HEADER_LEN is not in either diff. The prefix test (D1 655-698, T 171) passing with exit 0 and no flags shows that `header_start` is read correctly with a 200000-byte prefix.

**Exit-0 paragraph (spec 107-114), still open.** No field compares the central record's disk number start (offset 34) or internal attributes (offset 36). No flag compares data descriptor bytes after the data. Two archives that differ only there exit 0, which contradicts "central record are byte-equal". This is I1 defect 1. D2 does not change it, and the draft passes it to planning (draft 187-193). The code follows the flag definitions but not the guarantee paragraph.

**Sentinel test (D1 700-1074, D2 25-97)**
- **Fixtures:**
  - The sentinel string is built from pieces (D1 702-704) and there are six sentinel bytes (D1 707). Both are in every entry's data (D1 722-730).
  - Only the string, not the bytes, goes into the entry comments, the archive comments and the local and central extra fields (D1 811-817, 915-938).
- **Combinations:**
  - Every status in one comparison (D1 840-866), plus the sentinel entry alone as identical, recompressed, changed (equal length and shorter), dropped and added (D1 867-914).
  - Comments and extra fields (D1 925-943).
  - Error paths, each on the output side: encrypted with the ZipCrypto writer (D1 964-981), CRC corruption (D1 983-993), short stream, method 12, duplicate name, and an output file of sentinel content (D1 995-1041).
  - A missing file and a non-zip file whose paths hold the sentinel string (D1 1053-1073).
- **Absence checks:**
  - `assert_no_leak` works on bytes. In stdout and stderr it rejects the string, the bytes and the raw name (D1 757-772), plus, after D2, the bytes in escaped form and the string as lowercase hex (D2 29-51).
  - SENTINEL_BYTES_ESCAPED (D2 30) is correct for the tool's escaping; 0x7f escapes because the printable range ends at 0x7e.
  - The escaped name must be the whole of column 2 of an `entry` row (D1 773-785). Stderr may hold at most one line and at most one occurrence of the escaped name (D1 786-794).
  - SENTINEL_ESCAPED (D1 715) is correct: U+00FE is c3 be and U+0080 is c2 80. The name is 11 bytes, the same as `other-a.bin`.
- **Would it catch a leak?**
  - Verbatim leaks of content, comments or extra fields: yes.
  - The escaped six bytes and the hex of the string: yes, after D2.
  - A name printed unescaped: yes, and N proves it.
  - Stderr is strong. Every comparison asserts it is empty (D1 951), and every error path asserts the exact line (D1 1049), except the missing-file case, which checks only a prefix (D1 1059-1062).
  - Not caught: the hex of the six bytes, uppercase hex, base64, or partial fragments.
- **Departure (draft departure 2):** no entry name contains the sentinel string (spec 431-435 says "whose name is the sentinel"). The escaped name is allowed once in stderr, although the spec says "nowhere else". It is a defensible reading of two sentences that pull against each other, but it needs planning's ruling.

**zip64 test (D1 1076-1130)**
- `#[ignore]` (D1 1110). ZIP64_SIZE is 4294967297 (D1 1077). Deflated with `large_file(true)` (D1 1085-1088).
- Written in 1 MiB chunks from one reused buffer, refilled with zeros each pass (D1 1092-1105). The one-byte difference comes from regenerating at 4294967296 (D1 1115).
- Asserts exit 1, empty stderr, one row, `changed`, row[7] and row[8] (columns 8 and 9) both 4294967297, and columns 14 to 16 `4294967296`, `1`, `4294967296` (D1 1117-1129).
- Deleting the temporary files depends on TempDir dropping its directory, which is not in either diff, so that part is unverifiable.

**`local` tests on the byte builder (D1 570-653, builder D1 346-420)**
- Five cases each give `local` alone: local version needed, local flags 0x0800 with the central flags unchanged, local time, local date and local name. The central version needed alone, with the local value held at 20, gives `attrs` (D1 613-621).
- Each case asserts:
  - equal crate metadata as a precondition;
  - exit 1 and empty stderr;
  - entry 1 `identical` with no flags, and entry 2 `identical` with the exact flag string;
  - `flagged` 1 and the archive counts.
- The defaults (version needed 20 in both headers) reproduce the old hard-coded bytes (old D1 385 and 416), so earlier fixtures are byte-unchanged.

**Prefix-only files_identical test.** Both sides carry a 200000-byte prefix. After D2 the one differing byte is at 150000, in the third 64 KiB buffer (131072 to 196607), proven by a whole-file precondition (D2 9-21). The test asserts exit 0, empty stderr, both entries `identical` with no flags, `flagged` 0, `prefix_in` and `prefix_out` 200000 and `files_identical` `no` (D1 686-697). That meets spec 451-454.

**D2 against the I1 defects**
| I1 defect | Status after D2 |
|---|---|
| 1 | Not changed; passed to planning |
| 2 | Closed. D2 68-96 reruns the case with equal archive comments and asserts column 17 is exactly `extra,comment`. Correct: the local extras are equal, the central extras and entry comments differ, and the local fixed fields are equal. The rerun does not assert the status word or empty stderr. |
| 3 | Left for a ruling |
| 4 | Partly closed (escaped bytes and hex string only) |
| 5 | Closed |
| 6 | Closed in the draft. My rebuild of p4.rs puts the raw-name assertion at line 229, which matches N 222. |
| 7 | Closed (D1 starts at 827766a) |
| 8 | Test added (D2 106-128). It is correct, but it cannot show whether the error comes from the new checked read or from the zip crate, since both give `cannot read entry`. |
| 9 | Template count closed (N 5). TempDir is still unverifiable. |

**P2 and P3 expectation changes.** All follow from the `local` definition:
- recompressed entries: the compressed size differs, and for the method entry also the method and version needed (D1 430-432);
- changed entries: the CRC and sizes differ (D1 443-445, 456-458, 467, 476, 536-538);
- time, date and bit 11: `raw_zip` repeats the central value in the local header (D1 488-507, builder D1 391-396);
- data descriptor: the local flags, CRC and sizes are zero on one side (D1 516);
- `c.txt`: time and CRC (D1 525).
The unchanged `local-extra` case stays `extra`, since the extra length sits outside `fixed`.

**Beyond scope.** Nothing. D1 1-173 is planning's doc commit (ROADMAP, specs), which lies in the range. D2's past-the-end test and extra leak patterns answer I1.

## 2. Do the logs contain every line the spec requires?

**Verdict: yes.** Minor notes follow the list.

**"Every phase writes"**
- **gate.log:** full check.sh from a cold cache (G 130-132), eight PASS in order (G 134-141, checked G 145), 0 stderr bytes and exit 0 (G 142-144).
- **test.log:**
  - host and i686 with colour off (T 4), test names and summary lines (T 126-175, 186-235);
  - counts 35 and 35 (T 176, 236, 242), same names and equal counts (T 243-244);
  - the zip64 test is ignored in both default runs (T 168, 228).
- **zip64 runs:** both runs' output (T 254-258, 274-278), wall time (T 261, 281), peak RSS (T 262, 282), and checks under 65536 kB (T 265, 285). The executables match the default runs (T 167 = T 249, T 227 = T 269). finished-in 670.91 s and 864.84 s agree with 11:10.91 and 14:24.84.
- **negative.log:**
  - a clone under mktemp -d at c35112b (N 126-133, 174-177);
  - the control passes (N 180-186);
  - the escaping is replaced (N 191-217) and the test fails with exit 101, FAILED, the raw-name message and 0 passed; 1 failed (N 219-228);
  - the captured outputs hold 0 fixture lines, 0 control bytes and 0 non-ASCII bytes (N 185, 226).
- **scan.log:** range 827766a..c35112b, which is correct for specs/0001 now that c35112b is the last phase commit. It covers 13 files (S 481), 5 messages (S 534) and the 3 new logs (S 564), with controls all as expected (S 310), total hits 0 (S 566) and RESULT: PASS (S 570).

**specs/0001 rules**
- **First line:** G 1, T 1 and N 1 name c35112b and HEAD. S 1 names the range and HEAD.
- **Rerun header:** G 3, T 3, N 3 and S 3 each name b11c221, the record commit being replaced.
- **Each check evaluated once:** yes in all four scripts.
- **Compound checks:** each sits inside one function: `passed_matches`, `built_executable`, `rss_under` and `zip64_passed` (T 27-38); `all_zero`, `passed_once`, `failed_once` and `escaping_disabled` (N 57-79); `files_complete`, `messages_complete` and `scan_passed` (S 197, 214, 234). No `$(yn [ ] && cmd)` form remains.
- **Control and non-ASCII bytes:** 0 in every log by my own tr count.
- **Paths:** repo-relative. The only /tmp/ and /home/ strings are the scan's own patterns and controls (S 40, 133, 142, 143).
- **Fixture text:** my count of the sentinel string, its hex and the two templates is 0 in G, T, N and S.
- **Hashes:** the sha256 values in S 537, 546 and 555 match the committed logs byte for byte. S 382, 418 and 427 match the current I1 and both specs.

**Minor notes**
- (a) The label `exit is 0` at T 124 and T 184 does not name the target.
- (b) T 287's status check cannot see git-ignored files, so no log line shows that the zip64 temporary files were deleted.
- (c) I1 line 96 quotes the two fictional template strings. That is not captured tool output, but no scan looks for them.
- (d) The message of the record commit holding these logs is outside every scan range. This is inherent to the design.
- (e) N covers a clone plus two cold builds in a fresh CARGO_TARGET_DIR in 6 seconds (N 2, 231). That is plausible on a many-core host but cannot be confirmed from primaries. I do not count it as a defect.

## 3. Does every number and quoted string in draft 2 appear in a primary?

**Verdict: partly.** Nearly everything matches; the mismatches are listed first.

**Mismatches and overstatements**
- **Draft 141-145:** says the six bytes are "in every entry's data, in entry and archive comments and in local and central extra fields". Only the string is in comments and extra fields (D1 811-817, 915-938). The bytes are only in entry data (D1 722-730).
- **Draft 159-160:** "each run asserts its exit status, statuses or exact error line" is too strong. The missing-file run asserts only the error-line prefix (D1 1059-1062). D2's equal-comment run asserts neither statuses nor empty stderr (D2 84-96).
- **Draft 17-18:** calls D1 "the range specs/0001 names". With phase commit 2, the specs/0001 range is 827766a..c35112b, which is what S uses (S 1, 350). D1 plus D2 leave out b11c221 and d09c354, the record commits for the logs and I1.
- **Draft 47-48:** "(D2 tests it)". D2 shows an error with that message but not that the new checked read raised it.
- **Draft 45:** "30-byte fixed part" is inferred from the indices; LOCAL_HEADER_LEN is not in either diff.
- **Draft 133-135:** the P2 list does not name `large_entry_with_differences_in_first_middle_and_last_chunk` (D1 467), unless "the changed-entry tests" is meant to cover it.

**Verified**
- **Primaries:** D1 has 8 files, 770 insertions and 58 deletions; D2 has 1 file, 74 insertions and 4 deletions. Both match my counts, although the draft marks them as session output.
- **sha256 (draft 22):** G 490a8642..., T 0a076c1d... and N 391e6b0c... match the files and S 537, 546 and 555. scan.log itself is 4c1d964deb7e21d9e8d3aa16547a66122415d3e155d72357974baf0ddce5756e.
- **Commits (draft 27-34):** 564af3b (S 320), 74b2bf0f... (S 326), b11c221 (S 334), d09c354 (S 341), c35112b3... (S 345), and the message checks at S 485-525 and S 534.
- **What changed (draft 38-86):** matches D1 and D2, including bytes 6 and 7, the builder fields, the 4 tests, 200000, 100000 and 150000, 4294967297, 1 MiB, 4294967296, `\xfe\x01\x80\xff\x00\x7f`, 0xffff and `error: cannot read entry a.txt in the output archive`.
- **G (draft 92-96):** G 127, 134-141, 145, 142-144, 154 and 155 are all correct.
- **T (draft 98-109):**
  - T 142, 154, 166, 173, 176, 168 and 169 are correct.
  - The i686 lines T 202, 214, 226, 228, 229, 233 and 236 are correct, as are T 242-244.
  - Host: 11:10.91 and 5752 kB at T 251-265. i686: 14:24.84 and 3344 kB at T 271-285.
  - T 288 and T 289 are correct.
- **N (draft 111-122):** N 5, 176-177, 181, 183, 184, 186, 217, 219, 221, 223, 224, 227, 228, 185, 226, 230 and 232 are correct. N 176 is the exit check and 177 the commit line; both are relevant.
- **S (draft 123-125):** S 570, 310, 481, 534, 564, 566 and 569 are correct.
- **Departures:**
  - Draft 163-166: T 246-286 and the debug profile (T 249, 269; spec 350) are correct.
  - Draft 169-173: N 188-228, the replacement string (N 213) and "the third assertion" (p4.rs:229 at N 222, the raw-name check after D2) are correct.
  - Draft 177-178: the `Comparison` alias exists (D1 822).
- **Findings (draft 187-191):** offsets 34 and 36 are the correct central-header positions.
- **Independent checks (draft 197-214):** items 1-9 match I1 defects 1-9, and d09c354 adds I1 (S 341-343).

**Unverifiable (session output or not in primaries).** None contradicts a primary.
- 564af3b "committed without editing".
- The earlier host run at 11:06.76 and 5676 kB. I1 reports different numbers (11:09.18, 5728) for the first logged run, which is a different run, so this is not a contradiction.
- The dry run before the phase commit.
- The clippy `type_complexity` lint name.
- TempDir removing its directory.
- "verbatim" for I1.
- "Nothing is pushed".

## Defects

1. **The exit-0 guarantee is still not met.** Spec 107-114 says exit 0 means central records are byte-equal, but the central disk number start (offset 34), internal attributes (offset 36) and data descriptor bytes are compared nowhere. The draft passes this to planning (draft 187-193), but the code does not meet the spec text as written.
2. **The sentinel test departs from the spec's wording.** No entry name contains the sentinel string, and the escaped name is allowed once in stderr despite "nowhere else" (spec 431-435). It needs planning's ruling.
3. **The leak checks are still pattern-based.** They miss the hex of the six bytes, uppercase hex, other encodings and fragments. D2 closes only part of I1 defect 4; the draft states the limit.
4. **Draft 141-145 misstates where the sentinel bytes are.** Comments and extra fields carry only the string (D1 915-938).
5. **Draft 159-160 overstates the assertions.** The missing-file run checks only an error-line prefix (D1 1059-1062); D2's equal-comment rerun asserts neither statuses nor empty stderr (D2 84-96).
6. **D2's past-the-end test cannot tell which code path raised the error,** so "(D2 tests it)" at draft 47-48 is slightly strong.
7. **Draft 17-18 calls D1 "the range specs/0001 names".** That range is now 827766a..c35112b, as S uses. D1 plus D2 omit the record commits b11c221 and d09c354.
8. **Unverifiable from primaries:** the zip64 temporary files being deleted (TempDir is not in the diffs, and T 287 cannot see ignored files), the "30-byte fixed part", and the session-output items listed in section 3.
9. **Minor log notes:** `exit is 0` labels without the target (T 124, 184); I1 line 96 quotes the two fictional template strings, which no scan checks for.
