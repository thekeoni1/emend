# T1 P5 independent check 1

- Checked commit: 1c8eca19d055a11dea1b83613ff7125b865ef13d (T1 P5 phase
  commit)
- HEAD when the check ran: 97cb52db5d6a712c625cc974a463a8c1cf587f85
  (record commit with the logs at 1c8eca1, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, and asked to answer in plain ASCII
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    b7f25f430ac9ef10bc00bd34b2b0852f1c8563d9ee318d68c2b22f5706e2d498
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff, `git diff 343e91f..1c8eca1`, saved as <scratch>/t1p5-diff.txt
    04763197706bd6224c3e4b7a87bc82b9ab435f1d126b90dbcd481636533d11f0
  - draft report 1, <scratch>/t1p5-report-draft1.md
    2d82c29157db15af988d5f7efad9a0291dbaded98d2f2109634dd26a0526ba45
  - records/logs/t1-p5/gate.log
    362fe1e189e1d8f87a1793c9c025656b67048e1e9e13bbf67feac6dee7d4fd3d
  - records/logs/t1-p5/test.log
    7ef422c4c89d08ac0716eeb1decd704893ac2fc9b3c6fda20a9b3f072ec952b2
  - records/logs/t1-p5/negative.log
    2e1d7de0f3a667369d84821cf9ebf44fe882894a767c559c38cb9444ed9ab982
  - records/logs/t1-p5/scan.log
    6ebd047daca4b8b88827acdf7c59d1c1ce66157adc3aebd47ca106e2849342e5
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
I read the P5 spec, the 0001 rules, the diff, the draft report and the four logs. I ran only read-only checks on those files: sha256sum on the logs, byte counts for control and non-ASCII characters, a path grep, a per-file count of added and removed lines in the diff, and a search for one string. I did not read lib.rs or any other file outside the list, and I ran no git, cargo or build commands. My context also held CLAUDE.md and memory notes; I did not use them as evidence.

## 1. Does the diff do what P5 says, and nothing else?

Verdict: yes, with one wrong code comment, one unconfirmed possible gap in the exit-0 claim, and a few untested edges.

**`local` narrowing (D 183-205, 272-273).**
- Correct. The central record's bytes 6 to 27 are kept as `central_fixed` (D 149-150), 22 bytes.
- `SHARED_FIELDS` (D 187-196) splits the 22 bytes into eight ranges: 0-2, 2-4, 4-6, 6-8, 8-10, 10-14, 14-18, 18-22. These are version needed, flags, method, time, date, CRC-32, compressed size and uncompressed size, at local bytes 4 to 25 and central bytes 6 to 27.
- `local_only` sets the flag when a field differs between the local headers while the same central field is equal. The local name check stays as it was (D 273).
- The claim that `Local.fixed` holds local bytes 4 to 25 comes from P4. The diff only asserts it in a comment (D 75-77); I could not confirm it.

**`attrs` (D 158-159, 262-266).**
- Correct: disk number start is `le16(&header, 34)` and internal attributes are `le16(&header, 36)`.

**`descriptor` gap (D 123-141, 168-170, 207-245, 259).**
- `data_start` is header start + 30 + local name length + local extra length, with a checked add. Data end is `data_start + compressed`, also checked; an overflow maps to `Error::Entry`.
- The next header is taken from the sorted header starts, so file order, several entries and the last entry (falling back to `central_start`) are handled.
- An entry's own header is never chosen, because end is at least header start + 30. A data end exactly on the next header gives an empty gap.
- **The comment at D 124-127 is wrong, and so is the draft's departure 1.** `partition_point(|&s| s < end)` returns the first header at or after the data end, not the next header after this entry's header. When data runs past the next local header but stays before the central directory, the gap is not empty. Example: headers at 0, 100 and 200, central directory at 300, entry 0's data ending at 150. The gap is (150, 200). The gap is empty only when the data end is past `central_start`.
- This does not break coverage. Bytes 100 to 150 are inside entry 0's compared data, and 150 to 200 are in the gap. It is still a mismatch between the code and both descriptions, and no test covers overlapping data.
- `gaps_equal` is sound: unequal lengths give false and zero gives true. It reads with `take(u64)` through two `CHUNK` buffers and stops on a short read, and a length that is an exact multiple of the chunk ends with a 0-byte read. `a.1 - a.0` cannot underflow, because `end.min(next) <= next`. Everything is `u64`.
- Seek and read errors are reported as `Error::Open(side, kind)`. That label may read as an open failure for what is a read failure (the message text is not in the diff). It is minor.
- Nothing prints gap bytes, so entry content cannot leak through this path.

**Prefix.**
- The code treats `header_start`, `data_start` and `central_start` as absolute offsets. `central_start` is defined outside the diff, so I cannot confirm this from the primaries; the P1 ruling (departure 7) and the P4 prefix reproduction point that way.
- No P5 test combines a prefix with padding or a descriptor difference.

**Is exit 0 true by construction?** Mostly.
- Local header bytes 0 to 3 are the signature. Bytes 4 to 25 are covered by `local` or by the central flag or status. Bytes 26 to 29 plus the name and extra are covered by the name check and `extra`. The data is covered by the raw comparison inside `identical`. From data end to the next header is covered by the gap.
- Central record: bytes 4 to 5 and 6 to 7 are `attrs`, 8 to 9 is `gpflags`, 12 to 15 is `mtime`, 34 to 41 is `attrs`. The name and the three length fields follow from name pairing, `extra` and `comment`.
- **Possible gap, not confirmed:** central bytes 10 to 11 and 16 to 27 (method, CRC-32 and sizes) are compared only as values the zip crate derives, not as raw bytes.
  - Two central records could hold the same zip64 extra, with one storing 0xFFFFFFFF in a size field and the other the plain value.
  - If the crate reads the zip64 extra only when the field is 0xFFFFFFFF, both give the same size. The status would be `identical` with no flag, while central bytes differ.
  - I could not check the crate's behaviour from the primaries. Planning should decide whether to compare `central_fixed` directly.

**Tests (D 468-723).** Every required case is present with the required assertions:
- local-only time and local-only flags each give `local` alone;
- internal attributes alone and disk number start alone each give `attrs`. Each case asserts exit 1, empty stderr, the exact status and flags of both rows, and `flagged 1`.
- Descriptors that differ only in the signature: bit 3 on both sides, both rows `identical` with flags exactly `descriptor`, so `gpflags` is absent. This case only exercises gaps of unequal length (16 against 12 bytes); equal-length gaps with different bytes are covered by the long padding case.
- Padding between entries on one side: `descriptor` on the first row, both rows `identical`, exit 1, counts checked.
- "Padding removed" is just unpadded against unpadded, exit 0. That meets the wording, but it is a weak test.
- Extra cases beyond the spec: time in both headers (`mtime`), flags in both headers (`gpflags`), equal padding, padding after the last entry, and a 200000-byte gap differing in its last byte. The last one is longer than three 64 KiB buffers (196608 bytes). All are in scope.

**P2 and P3 expectation changes (D 354-467)** follow from the narrowed `local`: in each case the central record mirrors the local change. P2 `recompressed_entry` keeps `attrs` on the method entry.

**Byte builder (D 296-353).** Four new fields. The signature is left out when `descriptor_signature` is false, and padding goes after the descriptor. Disk start is written before internal attributes, which is the correct order at bytes 34 and 36.

**Beyond scope in the code:** nothing. The `header_start` field and the module doc lines are supporting changes. ROADMAP.md and the spec changes are planning's doc commit b58c086.

## 2. Do the logs contain every required line?

Verdict: yes.

- **gate.log:** eight PASS lines in order (G 133-140), checked at G 144; 0 stderr bytes and exit 0 (G 141-143).
- **test.log:**
  - host and i686 runs with colour off (T 3), with the test names and summary lines;
  - both counts at T 210, the same names at T 211 and equal counts at T 212.
- **negative.log:**
  - Case (a): exit 1 (N 246, 248), `descriptor` on one row (N 221, 249).
  - Case (b): exit 0 (N 282, 284), `files_identical yes` (N 280, 286).
  - The fixture, control-byte and non-ASCII counts are 0 (N 247, 283).
  - Built in the main tree behind the HEAD and clean-tree guards (script lines 40-41) and a status check after the run (N 289), as the 0004 preamble allows.
- **scan.log:** covers the 7 diff files, the 2 messages and the 3 other logs; controls pass; `RESULT: PASS`.

0001 rules:
- The first line names the commit and HEAD in all four logs.
- I found 0 control bytes (other than tab, LF and CR) and 0 non-ASCII bytes in all four logs.
- The only `/home/` and `/tmp/` strings are the scan script's own patterns and controls (S 39, 132, 141, 142), which the P2 ruling accepts. Other paths print as `<repo>`, `<scratch>` or relative.
- Every compound check runs inside a function: `passed_matches`, `rows_flagged`, `padded_by`, `all_zero`, `files_complete`, `messages_complete`, `scan_passed`. Each check is evaluated and printed once.

Small weaknesses, not rule breaks:
- The gen.py exit (N 211) is printed but not checked.
- The expected row at N 170 takes the insertion offset from gen.out itself.
- gen.out is shown through `hide` but is not included in the fixture count; it holds only counts.

## 3. Does every number and quoted string in the draft appear in a primary?

Verdict: yes, apart from one wrong description and a few items that cannot be checked.

**Primaries section.**
- 7 files, 432 insertions and 29 deletions match my count of D.
- The three log sha256 prefixes match both the files and S 427, 436 and 445. The draft does not quote scan.log's hash; mine is 6ebd047daca4b8b88827acdf7c59d1c1ce66157adc3aebd47ca106e2849342e5.

**Commits section.**
- b58c086 with 2 files, 22 insertions and 2 deletions matches D (1/1 and 21/1).
- The full hash 1c8eca19... matches S 313 and 321.
- S 405, 415 and 424 are correct.

**What changed section.** Matches D.

**Tests section.** Matches D, including 64 padding bytes and 200000 bytes.

**Acceptance section.** Every citation is on the right line:
- G 126, 133-140, 144, 141-143, 153, 154;
- T 100, 112, 124, 131, 136, 139 (11+10+10+4+3 = 38), T 133-135, 198-200, 165, 177, 189, 196, 201, 204, 210-212, 214, 215;
- N 4, 211-216, 221-223, 249, 231, 250, 244, 251, 246, 248, 252, 257-259, 285, 267, 280, 286, 282, 284, 287, 247, 283, 289, 291;
- S 460, 309, 401, 424, 454, 456, 459.
- Numbers quoted there also match: 37 bytes, 3100 and 3137 bytes, and the timestamps.

**Departures section.**
- **Departure 1 is wrong** about data running past the next local header (see question 1). Its claim that the gap is empty is true only for the central directory.
- Departure 2's claim that `data_descriptor_on_one_side` keeps `gpflags,descriptor,local` is not in any primary: the string does not appear in D and the test is not in the diff. Its "P4 needed no change" matches D, which has no p4.rs hunk.
- Departure 2's "recompressed_entry back to `` and `attrs`" has an empty code span, which will render badly. D shows `["", "attrs"]`.
- Departure 4's citation N 209 to 216 starts at the section header; the draft's other citation of the same content, N 211 to 216, is more exact.
- The pad-insertion description matches the printed gen.py (N 151-157).

**Unverifiable (session output).** The diff stat, the doc commit stat (both agree with D), the first test run in departure 1, and the dry run in departure 4. None contradicts a primary.

## Defects

1. **Possible hole in the exit-0 guarantee (not confirmed).** Central method, CRC-32 and size bytes (10 to 11, 16 to 27) are compared only as crate-derived values, never as raw bytes. With zip64, two central records could differ in raw size fields and still exit 0. This needs checking against the crate, or a direct `central_fixed` comparison.
2. **Wrong code comment and draft departure 1.** The comment at D 124-127 and departure 1 say overrunning data leaves an empty gap. The code does that only past the central directory; past a local header it measures the gap from the data end to the first header at or after it. The behaviour keeps coverage, but the text should be fixed and a test added.
3. **Unverifiable claim.** Departure 2's `gpflags,descriptor,local` for P3 `data_descriptor_on_one_side` is not in any primary.
4. **Untested edges.** No test has overlapping data or a prefix combined with a gap difference. The "padding removed" test is a trivial unpadded-against-unpadded comparison.
5. **Minor.**
   - A gap seek or read failure is reported as `Error::Open`.
   - An empty code span in departure 2.
   - The gen.py exit is not checked in negative.log.
   - The `gaps_equal` parameter in `flags` has the same name as the `gaps_equal` function.
