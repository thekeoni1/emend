# T1 P5 independent check 2

- Checked commits: 1c8eca19d055a11dea1b83613ff7125b865ef13d (T1 P5 phase
  commit 1) and b3534cc5af2a046da821f10ff73edbb75367d43b (phase commit 2,
  the fixes for check 1)
- HEAD when the check ran: 7d25ca230f9bf3766e365268aa82aa37c2007a2c
  (record commit with the logs rerun at b3534cc, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, and asked to answer in plain ASCII
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    b7f25f430ac9ef10bc00bd34b2b0852f1c8563d9ee318d68c2b22f5706e2d498
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff 1, `git diff 343e91f..1c8eca1`, saved as <scratch>/t1p5-diff.txt
    04763197706bd6224c3e4b7a87bc82b9ab435f1d126b90dbcd481636533d11f0
  - phase diff 2, `git diff a9a78de..b3534cc`, saved as <scratch>/t1p5-diff2.txt
    135857112b4dc572ecba0b9da0111479fb8e702a24db26bdf7bec713a1a1775f
  - draft report 2, <scratch>/t1p5-report-draft2.md
    3ab50f18f5e8b9e4de60c71eb01b6c6a4cd9e4a01ff9cf9e5d10e14247664684
  - records/logs/t1-p5/gate.log
    74c53edbb1ff98c26727baf6c4417dbaeec9545fefa31f5e35a58f3cbb258120
  - records/logs/t1-p5/test.log
    6c3a5ae0c863aff83072742e8e06265268ff111cd4effa7406868e54637ef33a
  - records/logs/t1-p5/negative.log
    9c8c6509b36b265586d47efbc720fa18a5d8e2160a2d9917e447b8a803df7ebd
  - records/logs/t1-p5/scan.log
    900faa4689bf5db6c3229554d152c49e91076089d86382d8df78691ed47f1ab0
  - records/logs/t1-p5/independent-check-1.md
    ec7cff16ecb2c4b182408fec4b2551dc57ff13d62c9d5375f423e9e485ca77d6
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
I read the P5 spec (specs/0004-xlsx-diff.md: Output, Exit status, Streaming, the Phases preamble, the P1 to P4 rulings, P5 and the close-out ruling) and specs/0001-bootstrap.md ("How this task runs" and "Rules for every phase"). I also read D1 (t1p5-diff.txt), D2 (t1p5-diff2.txt), draft 2, the four logs with the scripts printed in them, and I1. The only commands I ran were read-only: sha256sum on the logs, both specs and D1; an awk count of added and removed lines per file in D1 and D2; tr byte counts for control and non-ASCII bytes in the logs and the draft; and greps for path strings and a few quoted phrases. I did not read lib.rs or any file outside the list, and I ran no git, cargo or build command. My context also held CLAUDE.md and memory notes, and I did not use them as evidence.

## 1. Do the two diffs do what P5 says, and nothing else?

Verdict: partly. The code does what the Output section's flag definitions say, and I found no arithmetic bug. But the "true by construction" acceptance is not met in two places. One is the size fields under zip64 decoding, which the draft reports too narrowly. The other is a file-order case that no one has raised.

**`local` narrowing (D1 149-150, 183-205, 273).**
- `central_fixed` copies central bytes 6 to 27, which are 22 bytes.
- `SHARED_FIELDS` splits them into the eight shared fields at the right boundaries: version needed 0-2, flags 2-4, method 4-6, time 6-8, date 8-10, CRC-32 10-14, compressed size 14-18, uncompressed size 18-22.
- `local_only` sets the flag when a local field differs and the same raw central field is equal. The local name check is kept. This matches the Output text.
- That `Local.fixed` holds local bytes 4 to 25 is only asserted in a comment (D1 75-77). It comes from P4.

**`attrs` (D1 158-159, 262-266).** Disk number start is `le16(&header, 34)` and internal attributes are `le16(&header, 36)`, both correct. The test builder writes them in that order (D1 349-350).

**`descriptor` gap (D1 128-141, 168-170, 210-245, 259, 281-282, 291).**
- `data_start` = header start + 30 + local name length + local extra length, with a checked add. The local lengths are the right ones, not the central lengths.
- The data end is `data_start + compressed`, checked, and an overflow maps to `Error::Entry`.
- `partition_point(|&s| s < end)` on the sorted header starts gives the first header at or after the data end. The entry's own header can never be chosen, because end >= header_start + 30. `.min(central_start)` caps the result.
- The gap is `(end.min(next), next)`, so `a.1 - a.0` cannot underflow.
- Data that runs past the central directory gives an empty gap. Data that runs over a later header is compared in the raw pass, and its gap starts at its end. D2's new comment (D2 14-20) now says exactly this.
- `gaps_equal`:
  - unequal lengths give false and zero gives true;
  - it reads through `take(u64)` into two `CHUNK` buffers;
  - a short read ends the loop, and a length that is an exact multiple of `CHUNK` ends with a 0-byte read.
- Offsets and lengths are `u64` throughout. The `usize` values are only array indices and buffer counts.
- Seek and read errors map to `Error::Open(side, kind)`. The label is loose, but that is not a correctness bug.
- `CHUNK` = 64 KiB is defined outside the diffs.
- D2's prefix test (D2 50-64) supports the use of absolute offsets. With relative header starts against an absolute `central_start`, the last entry's gap would read shifted bytes, and the test would very likely fail. That depends on `pattern()` not repeating every 64 bytes, which I cannot see.

**Is the exit-0 paragraph true by construction?**

These parts hold:
- Local header bytes 4 to 25 are covered by `local`, or by the mirroring central flag or status.
- Bytes 26 to 29, the name and the extra are covered by name pairing and `extra`.
- The data is covered by the raw comparison, and the bytes from the data end to the next header by the gap.
- Central bytes 4-9, 12-15 and 34-41 are covered by `attrs`, `gpflags` and `mtime`. The name, extra and comment fields and their lengths are covered by pairing, `extra` and `comment`.

It fails in two places:

(a) **Sizes under zip64.** Central method, CRC-32 and sizes (bytes 10-11 and 16-27) are compared only through the values the crate derives (method, CRC, size, compressed size in `identical`).
- Suppose both sides carry a byte-equal zip64 extra (so `extra` stays clear), and one side stores 0xFFFFFFFF where the other stores a plain value. Then the decoded sizes can be equal, and the entry shows `identical` with no flag.
- If the draft's reading of the crate is right (zip64 extra read "whenever one is present"), the plain values need not even equal the real size.
- The narrowing carries the same hole into the local header. If the local size bytes differ and the central size bytes also differ in raw form, `local_only` is false. The status is still `identical` if the decoded values agree. So local bytes 18 to 25 can differ at exit 0 too.
- Every field `central_fixed` holds is already in memory, so comparing it directly (when the status is `identical`) would close the hole. Which flag should carry that is a question for planning.

(b) **File order (not raised by I1 or the draft).** Take two archives whose central records are in the same order (so `order_preserved` is `yes`), but whose local entries sit in a different order in the file. Example: side A holds X then Y, side B holds Y then X, no padding.
- Every entry is byte-equal from its header to the next header. All gaps are empty on both sides.
- `central_start` is equal, and the central records differ only in the offset field, which is excluded.
- Exit is 0 and `files_identical` is `no`, and the cause is neither a prefix nor the end-of-central-directory or zip64 records.
- So the paragraph's last sentence (spec lines 124-126) is false as written. The P5 acceptance clause itself ("no byte from a local header to the next header ... is outside") still holds.

**The draft's finding on raw central bytes (draft 170-179) is partly accurate.**
- The byte positions are right.
- It leaves out the condition that the zip64 extra fields must be byte-equal on both sides.
- It says the values must be "the same size", which is narrower than its own crate claim allows.
- It does not mention that the narrowed `local` carries the hole into the local size bytes.
- It includes method and CRC-32 without any evidence that the crate transforms those. The read.rs citation covers sizes and the header offset only.
- It frames the issue as an observation rather than as P5's "true by construction" acceptance not being met.

**Tests.**
- Every required P5 test exists and asserts what the spec requires:
  - local-only time and local-only flags each give `local` alone (D1 523-538, asserted at 580-587);
  - internal attributes and disk number start each give `attrs` (D1 539-554);
  - bit 3 on both sides with only the signature different gives `descriptor` alone and no `gpflags` (D1 591-615);
  - padding between entries on one side gives `descriptor` on the first row, all rows `identical`, exit 1, and the counts (D1 639-657);
  - padding removed gives exit 0 (D1 659-671).
- "Padding removed" is an unpadded-against-unpadded comparison. That meets the wording but is weak.
- The extra cases (time and flags in both headers, equal padding, padding after the last entry, a 200000-byte gap, and the prefix test in D2) are in scope.
- The P2 and P3 expectation changes (D1 362-467) all follow from the narrowed `local`: in each, the central record mirrors the local change. `attrs` stays on the recompressed method entry.
- Nothing tests data that runs over a later local header.
- The zip64 test was not rerun, so the new gap path has not been exercised past 4 GiB on i686. The spec does not require that run.

**Does D2 close I1's defects?**
- Defect 2: closed. The comment is corrected (D2 14-20).
- Defect 4: partly. The prefix case is added, and the overlap case is still untested (acknowledged in the draft).
- Defect 5: closed. The parameter is renamed `same_gap` (D2 28-38), and the gen.py exit is now checked (N 169, 215; that is in the script, not in D2).
- Defect 1: deferred to planning.
- Defect 3: the draft now cites passing runs but still quotes a string no primary contains.

**Beyond scope.** Nothing. ROADMAP.md and the spec hunks in D1 are the doc commit b58c086 (S 319-323). The module doc lines and the `header_start` field support the change.

## 2. Do the logs contain every required line?

Verdict: yes. No 0001 rule is broken. There are minor weaknesses.

**"Every phase writes":**
- **gate.log:** the full `check.sh` with eight PASS lines in order (G 134-141), checked at G 145; 0 stderr bytes and exit 0 (G 142-144).
- **test.log:**
  - colour off (T 4);
  - host and i686 test names and summary lines (T 85-139, 150-204);
  - both counts stated (T 211), the same names (T 212) and equal counts (T 213).
- **negative.log:**
  - built and run in the main tree behind the HEAD and clean-tree guards (script lines at N 41-42), with a status check afterwards (N 293), as the 0004 preamble allows;
  - archives in temporary directories;
  - fixture strings built at run time and never printed (N 5, N 44-48).
- **scan.log:** the 12 diff files (S 471), the 5 messages (S 524) and the 3 other logs (S 554), with a positive control per form (S 253-310).

**P5 negative cases:**
- (a): exit 1 (N 250, 252), three `identical` rows with `descriptor` on one (N 225-227, 253), `flagged 1` (N 235, 254).
- (b): exit 0 (N 286, 288), no flags (N 289), `files_identical yes` (N 284, 290).

**0001 rules:**
- The first line names the commit and HEAD in G 1, T 1, N 1 and S 1.
- Each log's line 3 names record commit 97cb52d as the one it replaces.
- I counted 0 control bytes (other than tab, LF and CR) and 0 non-ASCII bytes in all four logs and in I1.
- Compound checks are evaluated inside functions: `passed_matches`, `all_zero`, `rows_flagged`, `padded_by`, `row_is`, `files_complete`, `messages_complete` and `scan_passed`. The inline `[ x ] && echo || echo` forms (G 45, 47; N 95; S 83) each test a single condition.
- Paths print as `<repo>` or `<scratch>`, or as repo-relative. The only `/home/`, `/tmp/` and `/mnt/` strings are the scan script's own patterns and controls (S 40, 41, 133, 134, 142, 143).
- The captured tool output holds 0 fixture strings (N 251, 287).

**Weaknesses (not rule breaks):**
- The "zipfile reads every entry" check at N 173 builds its expected line from gen.out's own offset, so it cannot catch a wrong offset.
- gen.out is shown through `hide` but is not counted for fixture strings.
- The scan of the phase diff reads the logs as they were at b3534cc, which are the superseded first-run files (S 372, 390, 399 and 408, matching the hashes in I1's header). The rerun logs are scanned separately (S 527-553). scan.log itself is scanned by nothing; I found 0 control and 0 non-ASCII bytes in it.

## 3. Does every number and quoted string in draft 2 appear in a primary?

Verdict: mostly yes. Every line citation I checked is on the right line. There are a few unmarked unverifiable strings and two wording mismatches.

**Primaries.**
- D1: 7 files, 432 insertions, 29 deletions. My count matches (lib.rs 123/6, p5.rs 250/0, common 19/3, p2 11/11, p3 7/7, spec 21/1, ROADMAP 1/1).
- D2: 2 files, 25 insertions, 7 deletions, marked session output. My count matches (lib.rs 9/7, p5.rs 16/0).
- S 527, 536 and 545 are correct. G 74c53edb, T 6c3a5ae0 and N 9c8c6509 match both sha256sum of the files and S.
- The draft does not give S's hash. Mine is 900faa4689bf5db6c3229554d152c49e91076089d86382d8df78691ed47f1ab0.
- I1 is ec7cff16ecb2c4b182408fec4b2551dc57ff13d62c9d5375f423e9e485ca77d6, which matches S 381.
- The spec's sha256 matches S 417 and I1's header. D1's sha256 matches I1's header.

**Commits.**
- b58c086 with 2 files and 22/2 matches D1.
- The full hashes of 1c8eca1 and b3534cc match S 316-317 and S 1.
- The subjects of 97cb52d and a9a78de match S 333 and S 340.
- S 475 to 515, five messages, S 524: correct.
- "committed without editing" cannot be checked from the primaries.

**What changed.**
- It matches D1 and D2, with two mismatches:
  - Draft 48-50 still describes the gap as running "to the next local header in file order". That is the wording I1 flagged, and it contradicts departure 1 and the D2 comment.
  - Draft 91 lists the negative-script fix under D2, but it is not in D2. Its evidence is N 169 and 215.
- "64 KiB" relies on `CHUNK`, which is defined outside the diffs.

**Acceptance.** Every citation checks out:
- G 3, 127, 130-131, 134-141, 145, 142-144, 154, 155.
- T 3, 101, 113, 125, 132, 137, 140 (11+10+10+4+3 = 38), T 134-136, 199-201, 166, 178, 190, 197, 202, 205, 211-213, 215, 216.
- N 3, 5, 214-215, 216 (3 entries, 37 bytes, testzip ok), 217 (3096, 3133, 3096), 218-220, 225-227, 253, 235, 254, 248, 255, 250, 252, 256, 261-263, 289, 271, 284, 290, 286, 288, 291, 251, 287, 293, 295.
- S 3, 560, 310, 471, 524, 554, 556, 559.
- The timestamps match.

**Departures.**
- Departure 2's T 117 and 181 are correct.
- Departure 4's N 216 and 219 are correct, and the description of the padding insertion matches gen.py (N 152-158).
- The following strings are in no primary and are not marked as session output:
  - `cannot read entry` and `ends before its declared compressed size` (departure 1);
  - `cannot read the <side> file` (draft 196);
  - `gpflags,descriptor,local`, and the claim that the local CRC-32 and sizes are zero on the descriptor side (departure 2).

**Findings.** The byte positions 10-11 and 16-27 are right. The read.rs 661-673 citation is session output and cannot be checked. The finding's accuracy is discussed under question 1.

**Items marked session output.** The D2 stat (it agrees with my count), the doc commit stat (it agrees with D1), the first test run in departure 1, the dry run in departure 4, and read.rs. None contradicts a primary.

## Defects

1. **The exit-0 guarantee is not true by construction for size fields.** Central size bytes are compared only as crate-decoded values. The narrowed `local` defers to the raw central bytes, so under zip64 decoding the local size bytes can also differ at exit 0. The draft reports this as a spec observation, limits it to central bytes, leaves out the condition that the zip64 extras must be byte-equal, understates it ("the same size"), and includes method and CRC-32 without evidence. `central_fixed` is already in memory, and a direct comparison of it when the status is `identical` would close the hole.
2. **The last sentence of the Output paragraph is false (spec 124-126).** When local entries are permuted in the file but the central order is kept, the result is exit 0 with `files_identical no`, caused by neither a prefix nor the EOCD or zip64 records. Planning should rule on whether to reword the paragraph or add a check on local file order.
3. **Quoted strings in the draft with no primary and no "(session output)" mark.** `cannot read entry`, `ends before its declared compressed size`, `cannot read the <side> file`, and `gpflags,descriptor,local` with its zero-local-fields explanation.
4. **Draft wording mismatches.** "What changed" (draft 48-50) keeps the imprecise gap description that departure 1 and D2 corrected. Draft 91 attributes the generator-exit fix to D2, but it is in N 169 and 215, not in D2.
5. **Untested or weak points.** Data that runs over a later local header; the gap code on a zip64 archive (the zip64 test was not rerun, which P5 does not require); "padding removed" is unpadded against unpadded; the offset check at N 173 is self-referential; gen.out is not counted for fixture strings.
