# T1 P2 independent check 2

- Checked commits: 8d3c2aaab93ef9ed36502edef09a1a7297be4d6a (T1 P2 phase
  commit 1), ac10ff6298707eb93802d21b003276d101f0e7b9 (phase commit 2, the
  ZipCrypto fixture) and ed5f1f7c28a3be3f06d2673e115515e4f3d92de6 (phase
  commit 3, the fixes for check 1)
- HEAD when the check ran: c5142835a8110893ac559e0eac9b53d780465737
  (record commit with the logs rerun at ed5f1f7, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    7e73fd5a8de81509e7d7170ad36d6f2c8bc46f248e3675533766ddc77175b275
  - specs/0001-bootstrap.md
    a003b8aa23dbf5c9e2a8eae71d1728fb047414ef968bd89b14dfe4fcfecf7f23
  - phase diff 1, `git diff a3166a5..8d3c2aa`, saved as <scratch>/t1p2-diff.txt
    efa5130f733f1dd4a92ad79fe48e1aa1deddeb19b586067a25b52787477bf17b
  - phase diff 2, `git diff a809f55..ac10ff6`, saved as <scratch>/t1p2-diff2.txt
    4a61c416922e20046be4afea65dfaffeb08acef9e311bf59eff78ebfcbfd2f59
  - phase diff 3, `git diff e472c39..ed5f1f7`, saved as <scratch>/t1p2-diff3.txt
    e807cb0551c275c5db749cf0d5839b1921ede432b271ce470a97c8b9a076be0f
  - draft report 2, <scratch>/t1p2-report-draft2.md
    6d5e1ebcf6733650db0a4255dff687bd17b11c73b1a0f88d5521ca14e87f8a96
  - records/logs/t1-p2/gate.log
    3c98e1b9ecf1b731f825579a6c9b1fc392f3329b705cdeb1dfa038ac18a645ca
  - records/logs/t1-p2/test.log
    2baa6c5269f3b2759ad81c3bd27ac79740fbfff778ba320d4b8163896c4f29e4
  - records/logs/t1-p2/negative.log
    07173755cd336f48a507d2d4f4a800c61966caec94f4823947860a37c9342d69
  - records/logs/t1-p2/scan.log
    fee944b4cb5f2fe9e3b8710e2db63a5171a583e24a772b6adff610291c790252
  - records/logs/t1-p2/scan-failed-run.log
    94bdf04ac43f17fa1fc3fa18b2c1cbc19644b73fd54b719276dff4e0f141ecf8
  - records/logs/t1-p2/independent-check-1.md
    506903fe3947b55aafc14d43a4d07b1ed6190b9487ae9c124e79f99f4dc0b807
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
I read everything I was asked to and ran only read-only text tools on those files (`cat -n`, `sed`, `awk`, `sha256sum`, `tr`, `wc`, `/usr/bin/grep`) and `python3` for one column count. The files: all of specs/0004-xlsx-diff.md; specs/0001-bootstrap.md lines 22 to 115 ("How this task runs", "Rules for every phase"); D1, D2 and D3 in full; draft 2; gate.log, test.log, negative.log, scan.log, scan-failed-run.log and independent-check-1.md, including the scripts printed in the logs. With `awk` I counted diff insertions and deletions and found where the four arrows sit in I1. I took the sha256 of every input and counted control, non-ASCII and U+2014 bytes in the logs and the draft. I rebuilt the pre-fix I1 bytes to check its header, and grepped for session paths. I ran no git, cargo or build command and changed no file. CLAUDE.md and the memory index were in my context because the harness put them there; I did not use them as evidence. `read_full`, `CHUNK`, most of `raw_equal`, `run`, `Meta` and `main.rs` are not in any diff, so anything that depends on them is marked as inferred.

## 1. Do D1+D2+D3 do what P2 says, and nothing else?

**Verdict: yes for the code. No correctness bug found. A few tests are looser than they could be. Some additions go beyond scope. D3 closes I1 defects 1, 3, 4, 5 and the stdout part of 7.**

**Statuses, columns and archive rows**
- `differs` is gone. An entry on both sides is `identical` only when method, CRC and both sizes are equal and `raw_equal` holds; the `&&` short-circuits, so the raw comparison runs only when the metadata is equal (D1 261-268). Otherwise `content()` decides `recompressed` (equal bytes) or `changed` (D1 275-282).
- Columns 14 to 16 come from `offset_columns` (D1 205-213). The row format ends `{first}\t{count}\t{last}\t`, so column 17 is present and empty (D1 230). No flags.
- Archive rows are the seven keys in spec order (D1 320-324). No P3 row exists.
- `compare` returns true only when the recompressed, changed, dropped and added counts are all 0 (D1 325). That is right for P2; order and comment are left to P3.

**Offset arithmetic across 64 KiB buffers (D1 146-180)**
- It is correct, provided `read_full` fills the buffer unless the stream ends. That helper is not visible, but P1 planning verified it.
- Until one side ends, both sides deliver CHUNK bytes per pass, so `total_a` is the chunk offset on both sides (D1 163-164).
- In the pass where a side ends, only `min(got_a, got_b)` bytes are compared.
- After that, the ended side reads 0, so `common` is 0 and nothing more is compared.
- `first`, `count`, `last` and the totals are u64, and `k as u64` is widened before adding (D1 148-151, 169).

**Different lengths**
- Declared-size checks come first (D1 181-186), then `Length(min)` (D1 187-189). Column 14 is the shorter length and 15 and 16 are empty, as Streaming says (spec 142-144). This holds even when an earlier byte differs (draft departure 4).

**CRC checking**
- Neither stream stops early. A stream whose length is an exact multiple of CHUNK takes one more read that returns 0, which is where the reader checks the CRC.
- D3's `entry_of_exactly_two_buffers` (D3 240-288) exercises exactly that case: a 131072-byte entry with a bad CRC gives exit 2, and it passed on both targets (T 129, 170).

**Error mapping and leaks**
- `entry_error`: password gives Encrypted, an unsupported method gives Method, anything else gives Entry (D1 78-86). Any read error gives Data (D1 156, 161).
- D3 adds `Short`, used only at the two short-stream branches of `raw_equal` (D3 9, 17-21, 28-34). A grep of D1 and D3 finds no other use.
- Every error line carries only the escaped name, the side and the method number (D1 47-61, D3 17-21). No path, content or crate text.
- Exit mapping lives in `run` and `main.rs`, which are unchanged. The tests assert status 2 through `run`.

**Required P2 tests**

| Test | Verdict | Evidence |
|---|---|---|
| Short raw stream | Yes, and now tied to `raw_equal` | Cut data, central sizes kept (precondition `meta` equal), exit 2, empty stdout, exactly one line with the Short wording (D1 1133-1152, D3 132). |
| Recompressed | Yes | Level 1 vs 9 and deflate vs stored (D1 1155-1189), plus D3's CRC-equal precondition and method columns 8/8 and 8/0 (D3 140-158). D3 adds the equal-metadata, raw-differs path (D3 202-238). |
| Changed, equal length | Yes | Offsets 0, 999 and 0/500/999 in 1000 bytes; columns 14 to 17 asserted exactly (D1 1192-1233). |
| Different lengths | Yes | 1000 vs 600 both ways, `["600","","",""]`; D3 adds a byte differing at 5 (D3 166-188). Uncompressed sizes are asserted for row 0 only (D1 1264-1268). |
| Larger than the buffer | Yes | 300000 bytes, flips at 10 and 65535 (chunk 0), 65536 (chunk 1), 131077 (chunk 2), 299999 (chunk 4, the last); asserts `10`, `5`, `299999` (D1 1272-1291). Middle offsets are covered only through the count, and columns 4 to 13 are not asserted. |
| Wrong CRC, exit 2 | Yes | Two forms (D1 1294-1343). |
| Encrypted, exit 2, fixture rule | Yes | ZipCrypto writer through `with_deprecated_encryption` (D2 69-80), precondition `[false, true]` (D2 82-95), empty stdout both ways (D1 1374, D3 193), and T 186-199 says which fixture. The spec's first branch applies, so hand-set bit 0 is correctly unused. |

**D3's hand-built test helpers are correct**
- `crc32` (D3 47-60): reflected IEEE polynomial 0xEDB88320, initial value 0xFFFFFFFF, final inversion. Standard.
- `stored_blocks` (D3 65-79):
  - Each block is one header byte (BFINAL in bit 0, BTYPE 00, then padding to the byte boundary), then LEN and NLEN little-endian, then the data.
  - BFINAL is set only on the last window (`k + 2 == bounds.len()`).
  - For 3000 bytes, `[3000]` gives 3005 + 5 = 3010 bytes and `[1000]` gives 1005 + 2005 = 3010 bytes. The sizes are equal and the bytes differ, as the test asserts (D3 212-213).
- `single_deflate_entry_zip` (D3 84-118):
  - The shared 26-byte field run is right: version 20, flags 0, method 8, time 0, date 0x0021 (1980-01-01), CRC, compressed size, uncompressed size, name length.
  - Local header: extra length 0, then name, then data.
  - Central header: version made by 20/0, then the shared fields, then 16 zero bytes (extra, comment, disk, internal 2+2+2+2, external 4, offset 4), then the name.
  - EOCD: disk 0/0, entries 1/1, CD size, CD offset, comment length 0.
  - All offsets check out. The test also decompresses each archive with the crate and compares the bytes (D3 218-224), which independently confirms the helpers.

**Weaker points (not bugs)**
- In `entry_of_exactly_two_buffers`, the changed section asserts only the exit and the tail, not stderr or the status word (D3 269-274). There is no precondition that the level 1 and level 9 streams differ, though the `recompressed` assertion covers that.
- A truncated stored entry whose bytes differ earlier gives the Data line rather than Short (`raw_equal` returns `Ok(false)` before it reaches the short end). The exit status is still 2.

**Beyond scope**
- `Error::Method` and `unsupported_method_is_an_error`. The error is required; the test is extra.
- The tests/common/mod.rs refactor and the p1.rs rewrite (40 insertions, 353 deletions), including the new `stored_entry_across_several_buffers_is_identical`.
- The helpers `set_method`, `pattern` and `flipped`, and the lib.rs doc comment (D1 9-19).
- D3's two new tests, which I1 asked for, not the spec.
- None of these touches main.rs, Cargo or the P3 surface.

## 2. Do the logs contain every required line?

**Verdict: partly. Every required content line is present. One rule is not met: the scan rerun does not name its fix in the log. The fixture template text is still printed.**

**Present**
- **gate.log:** first line names ed5f1f7 and HEAD (G 1). The eight PASS lines in order (G 132-139) and the check (G 143), stderr 0 bytes and exit 0 (G 140-142), the tree check (G 125), end time (G 152).
- **test.log:**
  - Colour off (T 3). Test names and summary lines for the host (T 104-134) and i686 (T 145-175).
  - Both counts (T 182), same names (T 183), equal counts (T 184).
  - Which encrypted fixture was used, with its supporting evidence (T 186-199).
- **negative.log:**
  - Case (a): column 14 `123457` and column 15 `1` (N 254, 267-268), exit 1 (N 263-265).
  - Case (b): a 268435456-byte entry (N 274-275), the archives differ (N 276), `recompressed` (N 279), RSS 3492 kB under 32768 on the host with `/usr/bin/time -v` (N 295, 297).
  - Archives live under `<scratch>` and are deleted on exit (N 3, script N 31-32).
- **scan.log:**
  - Controls (S 251-308).
  - 14 files in the phase diff, each with its sha256 (S 385-511); 8 messages, each ending with the Co-Authored-By line (S 515-594); the 3 other logs (S 597-624).
  - RESULT PASS (S 630).

**Rules from specs/0001**
- **First line names the commit and HEAD:** all five logs do. scan.log names the record commit b052bc3, not the phase commit ed5f1f7 (S 1). That is acceptable, since b052bc3 is the end of its range.
- **Each check evaluated once and printed once:** yes in all four scripts.
- **Multi-condition checks inside one function:** yes. `passed_matches`, `all_zero` and `both_ran` (T 26-30); `all_zero`, `entry_rows_are` and `rss_under_limit` (N 64, 119-122, 206); `files_complete`, `messages_complete` and `scan_passed` (S 195, 212, 232). Every other `[ ] && echo` form I found tests a single condition (G 43, 45, 94; N 79; S 82, 171).
- **Control bytes:** 0 in all six log files and the draft, and 0 non-ASCII bytes and 0 U+2014 by my count. That includes the current scan.log, which no log scans.
- **Paths:**
  - Repo-relative, `<repo>` or `<scratch>` (G 114, N 234, T 104). `$HOME` is printed literally, not expanded (G 5).
  - The only `/home/`, `.claude` or `Claude-Session` strings are scan patterns and controls (S 51, 124, 140, and the same lines in the failed run).
- **Stopped run noted in the rerun log's header:** the stopped gate run was at ac10ff6. Its note was in the 27b1ed7 gate.log, which I cannot see; I1 91 cites it. The ed5f1f7 gate run was not a rerun of a stopped run, so its header correctly has no note (script G 41).
- **Fixture text:** not met in spirit. The printed script still holds `"fictional neg""ative content"` (N 47), `" row %d %s %s\n"` (N 129) and `" generated bytes "` (N 169), against spec 0004 line 163. The draft accepts this as departure 11, relying on the P1 gap 2 ruling (spec 244-245). Planning should decide.
- **Scratch copy:** 0001 lines 61-62 require a scratch copy for negative tests. N builds and runs `target/debug/xlsx-diff` from the main tree (N 35, 88), guarded by HEAD and clean-tree checks (N 41-42) and a clean status afterwards (N 299). 0004 line 162 allows "against temporary files", so this is defensible.

**The failed scan run**
- It is recorded: scan-failed-run.log is committed, and I checked it (397 `form=f_nonascii count=2 exit=0`, 600 `total hits: 2`, 604 `RESULT: FAIL`).
- The rerun passes (S 630) over a range that includes the fixed file (S 403, sha 506903fe, matching the file on disk) and the failed-run log itself (S 421, sha 94bdf04a, matching).
- **But the fix is not named in the rerun log.** 0001 lines 57-59 say "re-run with the fix named in the log". scan.log's header (S 1-3) and script say nothing about the earlier failure or the transliteration. The fix appears only in the b052bc3 subject (S 364), in I1's header (I1 31-38) and in the draft.
- The rerun's timestamps and HEAD are consistent with the draft's account:

| Log | Start | End | HEAD |
|---|---|---|---|
| G | 17:27:27 | 17:27:54 | ed5f1f7 |
| T | 17:27:54 | 17:27:54 | ed5f1f7 |
| N | 17:27:54 | 17:28:01 | ed5f1f7 |
| failed scan | 17:28:01 | 17:28:02 | ed5f1f7 |
| S | 17:30:18 | 17:30:19 | b052bc3 |

- The G, T and N hashes are the same in the failed run and in S (3c98e1b9, 2baa6c52, 07173755), so the logs did not change between the two scans.
- Transliterating a "verbatim" answer (0001 line 48) still needs planning's ruling, as the draft asks.

## 3. Does every number and quoted string in draft 2 appear in a primary?

**Verdict: mostly yes. I found no contradiction of a primary. There is one duplicated citation, one incomplete list, and a few unverifiable items.**

**Primaries section**
- D1 is 4 files, 825 insertions and 380 deletions; D2 is 2 files, 36 and 26. Both match my awk counts.
- D3 is 3 files, 185 insertions and 5 deletions. The draft marks this as session output, but it matches my count (lib.rs 8/2, common/mod.rs 74/0, p2.rs 103/3).
- "Four U+2192 written as `->`" matches I1 33-36.

**Log hashes**
- The draft's G `3c98e1b9...`, T `2baa6c52...` and N `07173755...` match S 597, 606 and 615, and my sha256 of the files.
- "S's own hash is in the check header": no primary holds this yet. scan.log on disk is `fee944b4cb5f2fe9e3b8710e2db63a5171a583e24a772b6adff610291c790252`. Do not use S 430's `044da30e...`: that is the superseded 27b1ed7 scan.log.

**Commits**
- The full hashes and subjects of 8d3c2aa, a809f55, ac10ff6, 27b1ed7, e472c39, ed5f1f7 and b052bc3 match S 311-367.
- "S 515 to 585, eight messages, S 594" is correct.
- Unverifiable:
  - c514283: its hash, its role and its message are not in any primary; its message is outside S's range by construction.
  - "a3166a5 committed without editing" (session output).

**What changed**
- Every error string matches D1 49-61 and D3 19, with the `error: ` prefix as the tests assert.
- The archive order matches D1 320-324, and the exit rule matches D1 325.
- The test names and figures all match D1 1133-1403, D2 45-95 and D3 136-288: the 8 tests; 0/999 and 0/500/999; 1000 vs 600; 300000 bytes with flips at 10, 65535, 65536, 131077 and 299999 and expected 10/5/299999; method 12; `stored_blocks` split at 3000 vs 1000; 131072 and `131071`/`1`/`131071`; byte 5; methods 8/8 and 8/0.
- "Asserts empty stdout in both directions" is true of the resulting test. D3 adds only the reverse direction (D3 193); the forward assertion was already in D1 1374.

**Acceptance section**
- Every citation checked is correct:
  - G: 125, 127-129, 132-139, 143, 140-142, 152, 153.
  - T: 120, 132, 135, 161, 173, 176, 124, 129, 165, 170, 182-184, 190-196, 197, 198, 199, 202.
  - N: 4, 236, 247-249, 230, 254, 268, 265-270, 274-276, 279, 290-292, 295, 297, 246, 264, 289, 301.
  - S: 630, 308, 511, 403-411 (all eight forms 0), 594, 624, 626, 629.
- Quoted values match: 278890, 123457, 268435456, 805983, 1207063, 3492, 32768, and both end times.

**Departures**
1. The line "T 186 to 199 says which fixture is used (T 186 to 199)" cites the same range twice; it is an editing slip. Unverifiable: the body of 8d3c2aa's message (S shows only the subject), the `#[cfg(feature = "aes-crypto")]` claim, and the probe. T 196 and T 199 support the conclusion. The old test comment is confirmed at D1 1347-1349.
2. "G 3 there" refers to the 27b1ed7 gate.log, which is not an input; only I1 91 supports it. The current G 3 is a different line. That the ed5f1f7 runs were not stopped is consistent with G.
3. N 276 is correct, and the script comment at N 175-176 backs the dry-run finding. The dry run itself is session output.
4. Matches D1 187-189 and D3 174-188.
5. Matches N 35 and N 277.
6. Matches D1 104-106 and I1 62.
7. Matches I1 63.
8. Matches D1 and D3.
9. Every function named exists at the lines cited (S 232 correct). The list leaves out `entry_rows_are` (N 119-122), which is also a two-condition function. The claim itself holds.
10. G 152, T 201, N 300, S 629 and N 41-42 are all correct.
11. Matches N 47.
12. Lines 397, 600 and 604 of the failed run are correct.

**I1's header claims**
- The four arrows sit at answer line 20 (file line 60), columns 32, 66 and 92, and answer line 21 (file line 61), column 20. These are columns in the original text, where the characters before each arrow are ASCII. In the committed file they fall at 32, 67 and 94 because each `->` adds a byte. The header's numbers are right for the extracted text.
- The only `->` pairs below the header are those four (file lines 60 and 61; the other hits are in the header itself, lines 33-34).
- Restoring U+2192 in file lines 41 to the end gives 14726 bytes with sha256 `93f7b71de9627ad98048aec0c323cb3f2bb4db896f2f31a2ce105bafd00bd874`. Both match the header exactly.
- Header lines 1-30 plus lines 39 to the end, with the arrows restored, hash to `ee265b71b701d34735e78a64eda97b8d377d666c8faaa68e9c5c749ff612071f`. That is the e472c39 hash in the failed run (line 396). So b052bc3 changed only the four arrows and inserted header lines 31-38.
- The I1 input hashes for D1 (`efa5130f...`), D2 (`4a61c416...`), specs/0004 (`7e73fd5a...`) and specs/0001 (`a003b8aa...`) match the files I was given.

**Response to I1 defect 2**
- The timing account is session output, but it is consistent with I1's own figures and with the new logs. The gate took about 26 to 27 s, T under a second, and N 7 s, back to back. That explains I1's concern without any overlap.

## Defects, most serious first

1. **The scan rerun does not name its fix in the log.** 0001 lines 57-59 require it. scan.log's header and script (S 1-3) never mention the failed run or the transliteration; the fix is named only in the b052bc3 subject (S 364), I1's header and the draft.
2. **I1 is no longer byte-verbatim**, which 0001 line 48 requires. The change is fully documented, and I confirmed it reverses exactly, but it needs planning's ruling (draft departure 12).
3. **negative.log still prints fixture template text** (N 47, 129, 169), against 0004 line 163 (draft departure 11, still open).
4. **The draft cites things no primary holds:**
   - "S's own hash is in the check header". The on-disk scan.log is `fee944b4...`; S 430's `044da30e...` is the superseded file.
   - c514283's hash and role, and its message, which no scan covers.
   - "G 3 there", which points into a superseded log that only I1 91 supports.
5. **Editorial slips in draft 2:** the duplicated citation in departure 1 ("T 186 to 199 ... (T 186 to 199)"), and departure 9's function list, which leaves out `entry_rows_are` (N 119-122).
6. **Minor test looseness:**
   - `entry_of_exactly_two_buffers`'s changed section asserts neither stderr nor the status word (D3 269-274).
   - The different-lengths test checks sizes for row 0 only (D1 1264-1268).
   - The large-entry test does not assert columns 4 to 13 (D1 1286).
7. **Minor code wording:**
   - A mid-stream I/O error, or a truncated entry whose earlier bytes differ, prints the decompress/CRC line (Data) rather than a more specific one. The exit status is still 2.
   - The negative binary is built in the main tree rather than a scratch copy (0001 lines 61-62; 0004 line 162 allows it).
