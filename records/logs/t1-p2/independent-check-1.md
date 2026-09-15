# T1 P2 independent check 1

- Checked commits: 8d3c2aaab93ef9ed36502edef09a1a7297be4d6a (T1 P2 phase
  commit 1) and ac10ff6298707eb93802d21b003276d101f0e7b9 (phase commit 2,
  the encrypted fixture made with the crate's ZipCrypto writer)
- HEAD when the check ran: 27b1ed765bef4c2e24dcd39c60095c7f53b0f169
  (record commit with the logs rerun at ac10ff6, local only)
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
  - draft report 1, <scratch>/t1p2-report-draft1.md
    f7ec38f8499ec0955202498df5de8a7d3b58e99ccb0863f6fedbb6a4177c2c0b
  - records/logs/t1-p2/gate.log
    e80b23ac09ff9c7049f591cfdc679bd3d394c7d952244d37a6724d0cad54e4a6
  - records/logs/t1-p2/test.log
    e59b713e854858317ae8da517cf50728b456559d79f6cd1a60c7f18bf0734256
  - records/logs/t1-p2/negative.log
    60de473990573ee93ff748beb3e58ac166cac17a7468039b00d704a8c1d59797
  - records/logs/t1-p2/scan.log
    044da30ef3c8d3bc80c2909763ff961447836edf2b11cdeec107b82cd7b032ca
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit
- One change from the extracted bytes: the answer held four U+2192
  characters (answer line 20, columns 32, 66 and 92; answer line 21,
  column 20), and each is written here as `->`. The two characters `->`
  appear nowhere else in the answer, so replacing every `->` below with
  U+2192 gives back the extracted bytes exactly: 14726 bytes, sha256
  93f7b71de9627ad98048aec0c323cb3f2bb4db896f2f31a2ce105bafd00bd874. The
  first commit of this record (e472c39) held the extracted bytes and made
  the phase scan fail (records/logs/t1-p2/scan-failed-run.log).

---
**What I read and ran.** I read the whole of specs/0004-xlsx-diff.md, lines 1 to 120 of specs/0001-bootstrap.md ("How this task runs", "Rules for every phase"), both diffs (D1, 1403 lines; D2, 110 lines), the draft report and all four logs in records/logs/t1-p2/, including the scripts printed in them. I ran only read-only text tools on those files: `cat -n`, `sed`, `awk` (to count diff files, insertions and deletions), `sha256sum` on the four logs, `tr`/`wc` (control and non-ASCII byte counts) and `/usr/bin/grep` (U+2014, absolute paths, `&&` forms). I ran no git, cargo or build command and changed no file. CLAUDE.md and the memory index were in my context because the harness put them there. I did not use them as evidence. lib.rs is only partly visible in D1: `read_full`, `CHUNK`, `raw_equal`, `run` and `Meta` come from P1 and are not in either diff, so claims that depend on them are marked unverifiable below.

---

## 1. Do D1+D2 do what P2 says, and nothing else? **Partly (the code is correct as far as it is visible; one required test is weaker than it looks; small additions beyond scope)**

**What matches the spec**
- **Statuses.** `differs` is gone. Both-sided entries are `identical` (`same_meta && raw_equal`, D1 268), otherwise `content()` decides `recompressed` or `changed` (D1 275-282). The raw comparison runs only when method, CRC and both sizes are equal, because `&&` short-circuits (D1 261-268), as Streaming requires.
- **Columns 14-16.** `offset_columns` fills first/count/last for equal lengths and the shorter length alone for different lengths (D1 205-213). The row format ends `{first}\t{count}\t{last}\t`, so column 17 is present and empty (D1 230). There are no flags.
- **Archive rows.** In spec order: entries_in, entries_out, identical, recompressed, changed, dropped, added (D1 313-324). No P3 rows.
- **Exit status.** Returns true only when the recompressed, changed, dropped and added counts are all 0 (D1 325). This is P1's rule plus the new statuses, and it leaves order and comment to P3, as it should.
- **u64.** `total_a`, `total_b`, `count`, `last` and `first` are u64 (D1 148-151); `k as u64` (D1 169).
- **Offset arithmetic across 64 KiB buffers is correct, given that `read_full` fills the buffer unless the stream ends.**
  - Every iteration before either stream ends delivers `CHUNK` bytes on both sides, so `total_a == total_b` at the top of each loop.
  - In the iteration where the first stream ends, `common = min` is compared at `total_a + k`.
  - After that, the ended side reads 0, `common` is 0, and nothing more is compared (D1 152-180).
- **Different lengths.** Return `Length(min)` and drop first/count/last (D1 187-189). This matches Streaming "column 14 set to the shorter length" (spec 142-144).
- **CRC.** Neither stream stops early: each loop reads a side until `got < CHUNK`, and a stream whose length is an exact multiple of `CHUNK` takes one more read that returns 0. So the zip reader reaches EOF on both sides, which is where it checks the CRC. Declared sizes are checked afterwards (D1 181-186). Whether the crate checks the CRC on that 0-byte read, and whether `read_full` loops, is not visible in the diffs. The two wrong-CRC forms passing on both targets (T 126, 165) are the only evidence.
- **Error mapping.**
  - Opening an entry: password -> `Encrypted`, unsupported method -> `Method`, anything else -> `Entry` (D1 78-86).
  - Any read error -> `Data` (D1 156, 161).
  - An encrypted entry is rejected in `open` from metadata (D1 104-106), so the `PASSWORD_REQUIRED` arm of `entry_error` cannot be reached in practice.
  - A plain I/O error mid-stream is reported as "failed to decompress or its CRC-32 does not match". The wording is slightly wrong, but the exit status is right.
- **No leaks.** The error lines (D1 47-61) carry only the escaped name, the side and the method number: no path, no content, no crate text.

**Required tests (D1 1132-1403, D2)**
- **Short raw stream: partly.** `short_raw_stream_is_an_error` (D1 1133-1152) cuts 1000 bytes of stored data, keeps the central sizes (precondition `meta` equal), and asserts exit 2, empty stdout and exactly one line `error: cannot read entry a.bin in the output archive`. That meets the letter of the spec. However, `raw_equal` is not in the diff, and after D1 `Error::Entry` is also the fallback for any other failure to open an entry (D1 84). Nothing in the primaries shows that this message comes from `raw_equal`'s short-read branch, which is the gap P1 carried over (spec 242-243). Planning should confirm this from `raw_equal`'s source.
- **Recompressed: yes.** `recompressed_entry` (D1 1155-1189) covers a level change and a method change.
  - It is weaker than the P1 version: it dropped the equal-CRC precondition and the `("8","0")` method assertion that P1 had (D1 955-958, 974).
  - No test covers the path where method, CRC and sizes are equal but the raw bytes differ, leading to `recompressed` (raw_equal false, content Equal). The only test that reaches `raw_equal == false` ends in a CRC error.
- **Changed, equal length: yes.** Offsets 0, 999 and 0/500/999 in 1000 bytes, columns 14-17 asserted exactly, including column 17 empty (D1 1192-1233).
- **Changed, different lengths: yes.** 1000 against 600 in both directions, `["600","","",""]` (D1 1236-1269). The data is a prefix, so the case in draft departure 4 (a byte differs before the shorter length) is not exercised.
- **Larger than the buffer: yes.** 300000 bytes, flips at 10, 65535, 65536, 131077 and 299999: first chunk, a chunk boundary, the third chunk and the last chunk. Asserts `10`, `5`, `299999` (D1 1272-1291). `CHUNK` = 64 KiB is not visible in the diffs.
- **Wrong CRC, exit 2: yes.** Two forms (D1 1294-1343).
- **Encrypted, exit 2: yes.** After D2 the fixture comes from `with_deprecated_encryption` (D2 69-80), with the precondition `[false, true]` (D2 82-95). The reverse direction does not assert stdout empty (D2 107-110).
- **Test counts.** T shows 11 + 8 on both targets (T 119/129, 158/168).

**Beyond scope (not asked for by P2)**
- The `Error::Method` variant and `unsupported_method_is_an_error`. The error itself is required by Exit status; the test is extra.
- Moving P1's helpers into the new tests/common/mod.rs (323 lines), and rewriting p1.rs onto them (40 insertions, 353 deletions). This removes `recompressed_content_differs` and `raw_bytes_alone_differ_across_several_buffers` from p1.rs and adds `stored_entry_across_several_buffers_is_identical`.
- The extra helpers `set_method` and `pattern`/`flipped`.
- The doc-comment update in lib.rs (D1 9-19).
- None of these touches main.rs, Cargo or the P3 surface.

## 2. Do the logs contain every required line? **Partly (every required content line is there; two rule departures and one timing question)**

**Present**
- **gate.log.**
  - First line names the commit and HEAD (G 1).
  - The eight PASS lines in order (G 132-139, check G 143), exit 0 (G 141-142).
  - The stopped run is in the header with start time, last completed check and reason (G 3).
- **test.log.**
  - Host and i686 runs with colour off (T 3), with test names and summary lines (T 103-131, 142-170).
  - Both counts (T 177), same names (T 178).
- **Which encrypted fixture.** T 182, backed by T 185-194.
- **negative.log.**
  - (a): column 14 `123457`, column 15 `1` (N 248, 261-262).
  - (b): a 268435456-byte entry (N 268-269), `/usr/bin/time -v` "Maximum resident set size (kbytes): 3484" (N 289), under 32768 (N 291) on the host.
  - The archives really differ, and the row is `recompressed`, so the entry was decompressed (N 270, 273).
- **scan.log.**
  - The phase range 6ccae09..ac10ff6 (12 files, S 462).
  - 4 messages ending with the Co-Authored-By line (S 466/476/486/496, S 505).
  - The 3 other logs (S 535), controls (S 305), RESULT PASS (S 540).
- **Byte and path checks (my own).** Control bytes 0, non-ASCII 0 and U+2014 0 in all four logs and in the draft. The only `/tmp/` or `/home/` strings are scan patterns or controls (S 39, 131, 140, 141). Paths print as `<repo>` or `<scratch>`.

**Rule departures and gaps**
- **S 231: a check with four conditions is not in a function.** `if [ "$ctl_fail" -eq 0 ] && [ "$err" -eq 0 ] && [ "$hits" -eq 0 ] && [ "$bad" -eq 0 ]` decides RESULT inline. It evaluates correctly (it is not the `$(yn [..] && ..)` bug), but it breaks 0001's "evaluated inside one function" rule.
- **negative script has no guard before the run.** Unlike the gate and test scripts (G 31-32, T 33-34), it does not check HEAD == commit or a clean tree before `cargo build`. It builds and runs `target/debug/xlsx-diff` from the main tree (N 35, 83), with only a status check after the run (N 293). Spec 0004 (line 162) allows "against temporary files", so this is allowed, but the binary is not tied to the commit before it is built.
- **Fixture template text is printed.** The script printed in the log includes the generator template: the phrase in split form (N 42), `" row %d %s %s\n"` (N 124) and `" generated bytes "` (N 164). The counter only looks for the joined phrase. The text is fictional, and P1 gap 2 set a precedent, but it is in tension with "No fixture text may appear".
- **Timing does not add up.** G started 17:16:10Z (cold `cargo clean` plus the full gate); T and N both started 17:16:36Z (T 2, N 2); S started 17:16:43Z (S 2). S hashes the new T and N logs in the working tree (S 517, 526). T and N each require a clean tree at start and end (T 34/195, N 293), so none of the three logs can have been moved in until both had finished. That leaves at most 7 s for N, which includes two 256 MiB Python deflate runs and a timed run of 5.10 s (N 288). It is barely possible, and it suggests the runs overlapped: G cold while T and N ran, the same load that got the first gate run killed. The run order is recorded nowhere. Planning should confirm it.
- **Last record commit message not scanned.** The message of the record commit holding these logs is outside S's range. Unavoidable, but no primary covers it.
- **scan.log's control bytes are not counted by any log.** S is "a log cannot scan itself". My own count is 0.

## 3. Does every number and quoted string in the draft appear in a primary? **Mostly yes; one contradiction, several unverifiable items**

**Verified against primaries**
- **Primaries.** D1 is 4 files, 825 insertions, 380 deletions; D2 is 2 files, 36 insertions, 26 deletions. My awk counts over the diff files match, though the draft marks these as session output.
  - Per file, D1: lib.rs 179/27, common/mod.rs 323/0, p1.rs 40/353, p2.rs 283/0.
  - Per file, D2: p2.rs 36/18, common/mod.rs 0/8.
- **Log hashes.** Full sha256 and prefixes match S and the files:
  - G `e80b23ac09ff9c70...` (S 508, file identical)
  - T `e59b713e854858...` (S 517, file identical)
  - N `60de473990573e...` (S 526, file identical)
  - scan.log's own sha256 is `044da30ef3c8d3bc80c2909763ff961447836edf2b11cdeec107b82cd7b032ca`; compare it with the check header.
  - S 363/372/390 show different hashes (e4daaa67, 560d054f, e7236485). Those are the superseded a809f55 logs as committed at ac10ff6, not these logs, so there is no contradiction.
- **Commits.** The full hashes 8d3c2aa... and ac10ff6..., and the subjects and files, match S 309-338.
- **What changed.**
  - Every error string matches D1 49, 54 and 59, with the `error: ` prefix as asserted in D1 1309, 1377 and 1401.
  - Archive order matches D1 320-324; `content` matches D1 125-194; the encrypted check matches D1 104-106 and 80-82.
  - "`raw_equal` is unchanged" is consistent: no hunk edits it.
  - The test names, offsets 0/999/0,500,999, 1000 against 600, 300000 bytes with flips 10/65535/65536/131077/299999 and expected 10/5/299999, and method 12 all match D1 1133-1403. The D2 items match D2 9-15 and 69-95.
- **Acceptance lines.** Every citation checked is correct:
  - G: 125, 132-139, 143, 140-142, 152, 3.
  - T: 119, 129, 132, 158, 168, 171, 177-179, 185-191, 192-193, 194, 196.
  - N: 4, 230, 241-243, 224, 248, 262, 259-264, 268-270, 273, 284-286, 289, 291, 240, 258, 283, 294.
  - S: 540, 305, 462, 505, 535, 537.
- **Departures 2-7.** Match G 3, N 170-171/270, D1 187-189, N 35/271, D1 104-106 and D1 1167-1168.

**Contradicted by a primary**
- **Departure 8** says "the compound checks in the P2 log scripts are functions". S 231 is a compound check that is not a function. Also, `archives_differ` (N 192) has a single condition, so it is not an example of a compound check.

**Unverifiable from primaries (none contradicts one)**
- Line 3 and line 29, the record commit with the logs at ac10ff6.
- Line 23, "committed without editing". S 314 shows only the files.
- Departure 1:
  - The body of 8d3c2aa's message (S shows only the subject, S 321).
  - The claim about `#[cfg(feature = "aes-crypto")]` in the crate source.
  - The throwaway probe.
  - T 191 and the passing test do support the conclusion.
- Departure 3's dry run, though N 170-171 is consistent with it.

---

## Defects, most serious first

1. **The carried-over short-stream test cannot be tied to `raw_equal` from the primaries.** It asserts `error: cannot read entry ...`, which after D1 is also the catch-all for other failures to open an entry (D1 84). `raw_equal` is not in the diff. Planning should confirm from source which branch produces this message.
2. **Run timing is inconsistent or unrecorded.** G 17:16:10Z, T and N 17:16:36Z, S 17:16:43Z. N had at most 7 s for 512 MiB of generation plus a 5.10 s timed run. The logs probably overlapped, and the run order is not recorded anywhere.
3. **S 231 breaks the 0001 rule** that a check with more than one condition is evaluated inside one function, and draft departure 8 claims otherwise.
4. **Test coverage gaps within P2's definitions:**
   - No test of equal method, CRC and sizes with different raw bytes giving `recompressed`.
   - No test with a stream length that is an exact multiple of 64 KiB, which is where the extra EOF read checks the CRC.
   - No test of different lengths with an earlier differing byte (departure 4's behaviour).
   - `recompressed_entry` lost P1's equal-CRC and method-column assertions.
5. **The negative script has no HEAD or clean-tree guard before the run** and builds the binary it measures in the main tree.
6. **negative.log prints the fixture generator template text** (N 42, 124, 164). It is fictional, and P1 gap 2 set a precedent.
7. **Minor:**
   - The `PASSWORD_REQUIRED` arm is unreachable because `open` rejects first.
   - A mid-stream I/O error is reported as a decompress/CRC failure.
   - The encrypted test's reverse direction does not assert stdout empty.
   - The final record commit message is not covered by any scan.
