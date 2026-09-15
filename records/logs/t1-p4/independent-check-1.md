# T1 P4 independent check 1

- Checked commit: 74b2bf0f2328b33a729d46105a33712e495992f1 (T1 P4 phase
  commit)
- HEAD when the check ran: b11c221246100537e46463c97c8fa920699651d9
  (record commit with the logs at 74b2bf0, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, and asked to answer in plain ASCII
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    9bbf2b97e0089e0337bf25ba86baf11d77dc00546e43eddde32ed358affc289a
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff, `git diff 564af3b..74b2bf0`, saved as <scratch>/t1p4-diff.txt
    93f1d47be92abb5034c2f8b099965ee5cd9194551576ac32f4a0abe32ee824d2
  - draft report 1, <scratch>/t1p4-report-draft1.md
    467f5fdb302b404eaf10b59fd41f06055fc9e96317d0ba6441abd79903dc4f7f
  - records/logs/t1-p4/gate.log
    47877feed833d0c14f8edcb427ffb64966833c68a8edc60865c391f12c6f4643
  - records/logs/t1-p4/test.log
    561967c4aed19f994b63718c59a507fb12716b34c237c1d37ce8ebcd7f923817
  - records/logs/t1-p4/negative.log
    87c8fc7471f40a306640ed4cc4e5f030ea2882dc1e9718049dc031786770eaf3
  - records/logs/t1-p4/scan.log
    67bfe29ecfcef311273a9b3c7d92772d8f1b5a4cf8211a7d8649dd2f4b3f5ade
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
I read the eight inputs in full: specs/0004-xlsx-diff.md, the "How this task runs" and "Rules for every phase" sections of specs/0001-bootstrap.md, the diff D (957 lines), the draft report (162 lines) and gate.log, test.log, negative.log and scan.log, including the scripts printed in each log. I ran only read-only tools on those files: sha256sum on the four logs, grep counts of files, insertions and deletions in D, tr counts of control and non-ASCII bytes in the logs and the draft, a grep for absolute paths in the logs, a grep for one flag string across D, the logs and the draft, and a line lookup that maps p4.rs line numbers to D (p4.rs line N is D line 374+N). No other file was used as evidence. CLAUDE.md and memory were in my context; I did not use them. Some tests/common helpers (TempDir, meta, set_crc, cut_entry_tail, patch, flipped) and the constant LOCAL_HEADER_LEN are not in D, so any claim that depends on them is marked as inferred or unverifiable.

## 1. Does the diff do what P4 says, and nothing else?

**Verdict: partly.** The code and tests match the P4 deliverables closely and nothing is out of scope. There are three reservations: the exit-0 guarantee in the Output section is not fully delivered, the sentinel test departs from the spec's literal wording, and a few checks are weaker than they look.

**lib.rs against the Output section**
- **Central version needed:** D 80 reads `le16(&header, 6)`. That is correct: central offset 4 is version-made-by, 6 is version needed, 8 is flags. `attrs` now compares external attributes, version-made-by and version needed (D 151-153), as the spec says.
- **Local header fixed part:** D 118-119 copies `header[4..26]`, 22 bytes. That covers version needed (4-5), flags (6-7), method (8-9), time (10-11), date (12-13), CRC-32 (14-17), compressed size (18-21) and uncompressed size (22-25). The signature and the name and extra lengths (26-29) are excluded. This matches spec line 101-104.
- **Name and extra fields:** the name length is read from 26-27 and the extra length from 28-29. The name is read before the extra field (D 127-128), which is the correct order. Both go through `read_full` with a length check, so a read past the end of the file returns `Error::Entry` (D 120-126). Allocations are bounded at 65535 bytes. The 30-byte header size is inferred from the indices, since LOCAL_HEADER_LEN is not in D. I found no offset bug.
- **Flag order:** mtime, extra, gpflags, descriptor, attrs, comment, local (D 138-162), matching spec lines 95-105. `local` compares both the fixed bytes and the local name bytes across the two archives (D 160).

**Gap against the exit-0 paragraph (spec 109-114).** The spec says exit 0 means every common entry's central record is byte-equal. The Record struct D shows has name, version_made_by, version_needed, flags, time, date and comment. The two hidden old lines 184-185 are presumably extra and external_attributes, since flags() uses both. I see no field for the central disk number start (offset 34) or internal file attributes (offset 36), so a difference in internal attributes alone would give exit 0. Separately, no flag compares the data descriptor bytes that follow the data (optional signature, CRC, sizes). The paragraph's list of allowed causes (prefix, padding, EOCD fields) covers neither case. This is inferred from D alone. The draft's "Findings: None" does not mention it.

**Sentinel test (D 527-901)**
- **Content sentinel:** a string built from pieces (D 529-531) plus six bytes (D 534), in every entry's data (D 549-557). It is also in entry and archive comments and in local and central extra fields (D 742-770).
- **Combinations covered:** identical, recompressed, changed with equal and different length, dropped and added, both in one "every status" comparison (D 667-693) and for the sentinel entry alone (D 694-741). Error paths: encrypted with the crate's ZipCrypto writer (D 791-808), CRC corruption (D 810-820), short stream, unsupported method 12, duplicate name, an output file of sentinel content, and a missing file and a non-zip file whose paths hold the sentinel string (D 822-900). Each run asserts its exit status and either the statuses or the exact error line (D 777-779, 875-876).
- **Absence checks:** assert_no_leak checks bytes, not UTF-8, for the string, the bytes and the raw name, in both stdout and stderr (D 584-599). The escaped name must be exactly column 2 of an `entry` row (D 601-611). Stderr may have at most one line and at most one occurrence of the escaped name (D 613-621). SENTINEL_ESCAPED is written out by hand and is correct: U+0001 is `\x01`, U+00FE is `\xc3\xbe`, U+007F is `\x7f`, U+0080 is `\xc2\x80`, U+0002 is `\x02`. The name is 11 bytes, the same as `other-a.bin`.
- **Departure from the spec text:** spec 431-435 asks for "an encrypted or corrupt entry whose name is the sentinel" and "a name containing the sentinel". No entry name in the test contains the sentinel string. The name sentinel is a separate non-printable name. This is arguably the only way to satisfy "absent from stdout" and "name printed escaped" together, and the draft records it as departure 2. Planning still has to rule on it.
- **Would it catch a leak?** It catches any verbatim leak of content, comment or extra bytes, and any failure to escape the name. The negative run proves the second case. It would not catch content printed in an encoded form (hex, or `\xNN` of the six bytes alone), or a leak of a few content bytes that are not the whole string or all six bytes.
- **"Sentinel comments and extra fields" case (D 752-770):** it asserts only exit 1 and status `identical`, not the flags column. Exit 1 would follow from the archive comment difference alone, so this case does not prove the entry flags were produced.

**zip64 test (D 903-957)**
- `#[ignore]`, one entry of 4294967297 bytes (D 904), deflated, `large_file(true)` (D 912-915).
- Written in 1 MiB chunks, with the chunk refilled with zeros on every pass (D 919-932), so the entry is never held whole.
- The output differs only at offset 4294967296, the last byte (D 942). It is regenerated with the one-byte difference rather than copied and patched, which the spec allows.
- Asserts exit 1, empty stderr, one row, `changed`, columns 8 and 9 both `4294967297` (row[7], row[8]), and columns 14 to 16 `4294967296`, `1`, `4294967296`.
- Temporary files are deleted only if TempDir removes its directory on drop. TempDir is not in D, so that part is unverifiable.

**`local` tests on the byte builder (D 397-480).** Six cases: local version needed, local flags 0x0800 with central flags unchanged, local time, local date, local name, and central version needed with the local value held at 20. Each asserts the exact flag string on entry 2, an empty flag on entry 1, `flagged` 1, exit 1, empty stderr, equal crate metadata as a precondition, and the archive counts. The builder changes (D 184-247) write the local header from the overrides and the central record from `version_needed`, and nothing else changed there.

**Prefix test (D 482-525).** A 200000-byte prefix on both sides with one differing byte at 100000, proven by a precondition. It asserts exit 0, both entries `identical` with no flags, `flagged` 0, `prefix_in` and `prefix_out` 200000, and `files_identical` `no`. That meets spec 451-454. One weakness: the byte is in the second 64 KiB buffer, so a loop that stopped after two buffers would still pass. P3 gap 1 was worded "longer than two buffers".

**P2 and P3 expectation changes.** All of them follow from the `local` definition:
- recompressed: the compressed size differs, and for the method entry the method and version needed also differ (D 259).
- changed with equal length: the CRC differs (D 270-272).
- different lengths: the sizes differ (D 283-285, 363-365).
- two-buffer and large entries: the CRC differs (D 294, 303).
- time, date and gpflags: raw_zip repeats the central value in the local header (D 316, 324, 334).
- data descriptor: the local CRC and sizes are zero on one side only (D 343).
- `c.txt`: the CRC and time differ (D 352).
- Untouched cases (extra, comment, attributes) do not change local fixed fields.

**Nothing beyond scope.** One side effect: a local name or extra field that runs past the end of the file is now an error. No test covers it.

## 2. Do the logs contain every required line?

**Verdict: yes.** Two minor notes are at the end of this section.

**"Every phase writes" (specs/0004 173-189)**
- **gate.log:** the full `scripts/check.sh` run with eight PASS in order (G 132-139, checked at G 143), exit 0 (G 141-142), and the dependency tree (G 125).
- **test.log, both runs:** host and i686 runs with colour off (T 3, T 18), test names and summary lines (T 124-172, 183-231). Both counts are stated (T 173, 232, 238), with same-names and equal-count checks (T 239-240).
- **test.log, zip64:** both runs' output (T 249-255, 269-275), wall time (T 257, 277), peak RSS (T 258, 278), and checks under 65536 kB for both (T 261, 281).
- **negative.log:** runs in a clone under `<scratch>` (N 172-175). The control passes (N 179-184). With escaping disabled (N 189-215) the test fails, with exit 101, FAILED and the raw-name message (N 217-226).
- **scan.log:** covers the 8 changed files (S 411), 2 messages (S 434) and the 3 other logs (S 464). Controls all as expected (S 308), total hits 0 (S 466), `RESULT: PASS` (S 470).

**specs/0001 rules**
- **First line** names the commit and HEAD in G 1, T 1 and N 1. S 1 names the range 827766a..74b2bf0 plus HEAD, which is acceptable.
- **Each check evaluated once and printed once:** yes in all four scripts.
- **Compound checks inside one function:** passed_matches, built_executable, rss_under and zip64_passed (T 26-37); all_zero, passed_once, failed_once and escaping_disabled (N 56-78); files_complete, messages_complete and scan_passed (S 195, 212, 232). No `$(yn [ ] && cmd)` form remains.
- **Control and non-ASCII bytes:** 0 in all four logs by my own tr count.
- **Paths:** repo-relative. The only `/tmp/` and `/home/` strings are the scan's own patterns and controls (S 39, 131, 140, 141).
- **Scan script:** starts with `#!/usr/bin/env bash`, calls /usr/bin/grep, runs under LC_ALL=C, matches the em dash as bytes, handles exit 2 as an error, has a positive control for every form, and prints itself.
- **No sentinel in captured output:** counted as 0 (N 183, 224).

**Minor notes**
- **(a) Fixture text count:** negative.log counts only the sentinel string. The fictional templates in p4.rs ("fictional row", "fictional-password") are not counted, although only filtered test lines are printed.
- **(b) Scan range vs D:** specs/0001 defines the phase diff as the previous phase's last record commit up to this phase's last phase commit, which is 827766a..74b2bf0. S scans that range, but D as given is 564af3b..74b2bf0 and leaves out planning's doc commit.
- **Labels:** `exit is 0` appears at T 122 and T 181 without naming the target. The headings tell the two apart.

## 3. Does every number and quoted string in the draft appear in a primary?

**Verdict: partly.** Almost everything matches. The exceptions are below.

**Mismatches**
- **Draft 128:** "comments and extra fields holding the sentinel (`comment,extra`)". This string is in no primary (my grep found it only in the draft). The implementation's flag order would print `extra,comment` (D 141-159), and the test does not assert the flags for that case (D 767-769).
- **Draft 146:** "the test fails on its first leak check". N 220 panics at p4.rs:221, which is D 595, the third assertion in assert_no_leak (raw name in stdout). The first two, string and bytes, passed. Say "the raw-name check" instead.

**Verified**
- **D summary (draft 13):** 5 files, 674 insertions and 47 deletions match my count of D, although the draft marks this as session output.
- **sha256 (draft 15-16):** my computed values match S 437, 446 and 455 exactly: gate 47877feed833..., test 561967c4aed1..., negative 87c8fc7471f4.... scan.log itself is 67bfe29ecfcef311273a9b3c7d92772d8f1b5a4cf8211a7d8649dd2f4b3f5ade; the draft defers it to the check header.
- **Commits (draft 21-24):** the full hash matches G 1 and S 312. S 415, 425 and 434 are correct. 564af3b touches only the three doc files (S 314-318).
- **What changed (draft 28-62):** matches D.
  - Bytes 4 to 25: D 118-119. Bytes 6 and 7: D 80.
  - RawEntry fields: D 184-191.
  - Four tests, one ignored: D 397, 482, 651, 936-938.
  - 200000 and 100000: D 486-487, 522-523.
  - 4294967297, 1 MiB, 4294967296: D 904, 910, 942, 951-955.
  - Bit 11 is 0x0800.
  - "30-byte fixed part" is inferred, not shown.
  - "the `cannot read entry` error" and "closes the edge I1 of P3 noted" are not in any primary and are unverifiable.
- **Acceptance G (draft 66-70):** G 125, G 132-139 and 143, G 140-142, G 152 and G 153 all check out.
- **Acceptance T (draft 72-82):**
  - Counts: 11, 10, 10, 3 and 1 ignored (T 140, 152, 164, 170). 34 (T 173). The ignored test is at T 166.
  - i686 citations: T 199, 211, 223, 225, 229, 232, and T 238-240, all correct.
  - Host zip64: T 247, 252, 254, 257, 258, 260 and 261, with 11:09.18 and 5728.
  - i686 zip64: T 267, 272, 274, 277, 278, 280 and 281, with 14:32.06 and 3364.
  - 65536: T 261 and 281. The end time at T 284 and the final line at T 285 are correct.
- **Acceptance N (draft 84-95):**
  - N 4, N 173-175, and N 179, 181, 182 and 184 are correct.
  - N 215. N 217, 219, 221, 222, 225 and 226 with the quoted message and exit 101. N 183 and 224, N 228, N 230. All correct.
- **Acceptance S (draft 96-98):** S 470, 308, 411, 434, 464, 466 and 469 (19:05:07Z) are all correct.
- **Departures:**
  - Draft 115-124: the six bytes match D 534 and the name description matches D 536-542.
  - Draft 136-139: T 242-282, the debug profile (T 245, 265) and the P2 ruling (spec 350) are correct.
  - Draft 144-145: the replacement string matches N 211, and the clone's own build directory is at N 114.
  - Draft 150-151: the `Comparison` alias exists (D 649). The clippy lint name is session output.

**Unverifiable, session output.** None of these contradicts a primary.
- 564af3b "committed without editing".
- The earlier host run: 11:06.76 and 5676 kB.
- The dry run before the phase commit.
- clippy `type_complexity`.
- The D file, insertion and deletion counts, although they match D.

**"Findings: None" (draft 155).** I disagree. See defects 1, 3 and 4 below.

## Defects

1. **Exit-0 guarantee not met (spec 109-114).** From the visible Record fields, the central internal attributes (offset 36) and disk number start (offset 34) are compared nowhere, and neither are data descriptor bytes after the data. A difference there gives exit 0 even though the paragraph says the central records and entries are byte-equal. Planning should rule on this; the draft does not report it.
2. **Wrong flag string in the draft (line 128).** `comment,extra` appears in no primary, would print as `extra,comment`, and that test case does not assert the flags. Its exit 1 would follow from the archive comment alone.
3. **Sentinel names depart from the spec.** No entry name contains the sentinel string; the encrypted and corrupt entries use a separate non-printable name sentinel. This is a reasonable reading and is recorded as departure 2, but it needs a planning ruling.
4. **Leak checks match only verbatim bytes.** A leak printed in hex or escaped form, or a short fragment of content, would not be caught. The draft's statement that "the sentinel test is the leak check the spec asks for" should carry this limit.
5. **Prefix test difference in the second buffer.** The differing byte (100000) is in the second 64 KiB buffer, so a comparison that stopped after two buffers would still pass. P3 gap 1 was worded "longer than two buffers".
6. **Wrong description of the negative failure (draft 146).** It says "first leak check"; the failure at p4.rs:221 is the third assertion, the raw-name check.
7. **D range differs from specs/0001.** D is 564af3b..74b2bf0; specs/0001 defines the phase diff as 827766a..74b2bf0. S scans the correct range.
8. **Untested new error path.** The error for a local name or extra field that runs past the end of the file has no test. The draft's `cannot read entry` wording and the "closes the edge I1 of P3 noted" claim are not in any primary.
9. **Unverifiable from primaries.** Deletion of the zip64 temporary files depends on TempDir, which is not in D. negative.log counts only the sentinel string, not the other fictional templates.
