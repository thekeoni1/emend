# T1 P6 independent check 1

- Checked commit: 5d98cdc640dae3f6334041e35b74a65a1b6b3590 (T1 P6 phase
  commit)
- HEAD when the check ran: e8280e3dffd0428a6f0405f05dd68d479df2d519
  (record commit with the logs at 5d98cdc, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, and asked to answer in plain ASCII
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    c03f9b28fd8df5bea2ee5b2ec79a6b2714f773ab9fd27a3c7d0f5eaeb3564cf2
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff, `git diff 4f6f347..5d98cdc`, saved as <scratch>/t1p6-diff.txt
    c61c1dbd5e2663cac6b2d8dba547fd2ab1ff3a3f13c35f7b3b776a127cc47aae
  - draft report 1, <scratch>/t1p6-report-draft1.md
    44e1f6840ad12b37cda326886e82654612d9ad59ffc5b65e4cd585e3462689c1
  - records/logs/t1-p6/gate.log
    3ef937b60d5404fe6e5ce81c6b94e7313f73da6a1d8f93e2800ecaf28e75cc2e
  - records/logs/t1-p6/test.log
    b8da806da13c68af75e6b916f90d38f6c12eec8ba3dd9a0ab6211daf5936f11c
  - records/logs/t1-p6/negative.log
    0313860c56d4be698f66ac576a39b9527958c8aa0b2f2eac3d353a45bf89975c
  - records/logs/t1-p6/scan.log
    a22de7aa82ced7e587a069e00d5aa73e9a4fc408807a8590bf3dcea10ff27a5d
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
I read only the eight listed inputs: spec 0004 in full, spec 0001 "How this task runs" and "Rules for every phase" (lines 22 to 123), the diff D, the draft report, and the logs G, T, N and S, including the scripts printed in each log. The harness also put CLAUDE.md and a memory index into my context; I did not use them as evidence. I ran read-only commands only. With sha256sum I hashed the four logs. With tr I counted control bytes (other than tab, LF and CR) and non-ASCII bytes in each log. With a small python3 counter I counted the insertions and deletions per file in D, and with grep I counted non-ASCII bytes in D and the draft. Results: G 3ef937b60d54...cc2e, T b8da806da13c...f11c, N 0313860c56d4...975c, all three equal to S 405, 414 and 423; S itself is a22de7aa82ce...7a5d. All four logs have 0 control bytes and 0 non-ASCII bytes. D and the draft have 0 non-ASCII bytes. D per file: ROADMAP.md +6/-1, specs/0004 +82/-10, lib.rs +29/-1, tests/common/mod.rs +42/-7, tests/p6.rs +197/-0; total +356/-19, doc files +88/-11. I did not run git, cargo or the tool. D does not show the definitions of `Meta`, `central_fixed`, `header_start`, `text`, `meta` or `raw_stored`'s local extra, so anything that depends on them is marked as inference.

## 1. Does the diff do what P6 says, and nothing else?

**Verdict: yes for the code and the required tests, with one edge-case hole in the order check and some weak test assertions (below).**

**`attrs` (D 180-191, 200).**
- A central record's fixed part: signature 0-3, version made by 4-5, version needed 6-7, flags 8-9, method 10-11, time 12-13, date 14-15, CRC 16-19, compressed size 20-23, uncompressed size 24-27. `central_fixed` holds bytes 6 to 27, so index k is byte 6+k.
- `differs(4, 6)` is bytes 10-11, the method; it is compared against `a.method == b.method`. Correct.
- `differs(10, 14)` is bytes 16-19, the CRC-32; compared against `crc`. Correct.
- `differs(14, 18)` is bytes 20-23, the compressed size; compared against `compressed`. Correct.
- `differs(18, 22)` is bytes 24-27, the uncompressed size; compared against `size`. Correct.
- Each range goes with its own decoded value. I am assuming `Meta.compressed` and `Meta.size` are the compressed and uncompressed sizes; the struct is not in D.
- The condition is added with OR to the existing attrs test. The flag order is unchanged: D shows no reordering, and the `set.push("attrs")` context is the same.
- Slicing up to index 22 is safe when `central_fixed` is 22 bytes, as the prompt states.
- Nothing new is printed. No offset arithmetic is added in lib.rs; the new indices are slice or vector indices.

**`order_preserved` (D 208, 216, 224-232).**
- Common pairs (i, j) are collected in input index order. `sort_by_key` (a stable sort) orders them by input `header_start`.
- Then any adjacent window where the output `header_start` of the first is strictly greater than the second sets `order_preserved = false`.
- This combines with the existing central check by ANDing into the same boolean, so either check can give `no`. That is correct for valid archives.
- The comparison is strict, and the tie handling is asymmetric:
  - A tie in the output is accepted.
  - A tie in the input is broken by input index, and the output is then held to that index order.
- Defect 1 below gives a contrived example where exit 0 is possible without "the same order in both files". Draft departure 3 names the output-tie half. The input-tie half is not mentioned.

**Is the exit-0 paragraph true by construction? Yes for valid archives, by my reading.**
- Central record bytes:
  - 0-3: fixed signature.
  - 4-7, 34-41: attrs.
  - 8-9: gpflags.
  - 10-11 and 16-27: status when the decoded values differ, `raw_only` when they are equal.
  - 12-15: mtime.
  - 28-29: equal, because entries are paired by name.
  - 30-31: extra.
  - 32-33: comment.
  - 42-45: the offset, excluded.
  - Then name, extra and comment.
- Local header to next header:
  - Local fixed fields: `local`, or the matching central flag or status.
  - Name: `local`.
  - Extra: `extra`.
  - Data: status.
  - Gap: `descriptor`.
- The local-and-central mirrored raw-size case that P5 left open is now closed by `raw_only`.
- Order: the central check plus the new file check.
- Remaining limits:
  - Defect 1, the shared-offset case.
  - The method and CRC branches of `raw_only` are never exercised. Draft departure 2 says they cannot fire, but that rests on the `Meta` source, which is not in D.

**Required tests (D 339-541).**
- **zip64 pair (D 399-451):**
  - Both sides get a byte-equal zip64 extra field (`zip64_extra(len, len)`, uncompressed then compressed). One side has 0xffffffff in both central size fields.
  - Preconditions: equal `meta`, different bytes.
  - Asserts status 1, empty stderr, `[("identical",""),("identical","attrs")]`, `flagged` 1 and `order_preserved` yes.
  - Placeholders on both sides: status 0, empty stderr, no flags.
  - Matches the spec. Weakness: both size fields change together, so a `raw_only` that tested only one size range would still pass. No test isolates each branch.
- **Permuted (D 453-509):**
  - Local order [2, 0, 1] with the central order kept.
  - Asserts exit 1, empty stderr, every row identical with an empty flags column (and output indices 0, 1, 2), `flagged` 0, `order_preserved` no, `files_identical` no, and the status counts.
  - Matches the spec, plus an extra positive control (the same permutation on both sides gives exit 0 and yes). That addition is harmless.
  - The "precondition" at D 461-465, that `raw_zip_with_local_order(.., [0,1,2])` equals `raw_zip`, is a tautology. `raw_zip` now just calls that function (D 268-269), so it proves nothing about pre-P6 bytes.
- **Overrun (D 511-541):**
  - `declared_compressed = raw.len() + 40`, identical on both sides.
  - Asserts exit 0, empty stderr, three identical rows with no flags, columns 10 and 11 equal to the declared size, and `order_preserved` yes.
  - Covers "no error" and "exit 0". It does not directly show "the overlapped bytes compared as data" or "the gap starting where the data ends": with both sides alike, any gap definition gives the same result. So the test is consistent with the spec but does not discriminate.

**Byte builder (D 237-338).**
- New fields: `central_size_placeholder` (default false) and `declared_compressed` (default None).
- `local_offsets[k]` is assigned in `local_order`. The permutation is asserted.
- `compressed = declared_compressed.unwrap_or(raw.len())` is used in the local header, the descriptor and the central record. With placeholder false and None, the central record writes `u32_of(raw.len())` and `u32_of(data.len())` as before.
- For the natural order, `local_offsets[k]` equals the old push order. The central loop is not in D, so it is unchanged.
- So `raw_zip` writes the same bytes as before for existing callers. T supports this: P1 to P5 still pass on both targets (T 100-136, 170-206).
- `declared_compressed` is `Option<usize>`. This is test code, consistent with the builder's existing `usize` lengths.

**Beyond scope:** nothing material. The lib.rs module doc comment (D 170-172) and the extra positive sub-test are within the phase. The ROADMAP.md and spec hunks are planning's doc commit 8a898b3 (S 316-319).

## 2. Do the logs contain every required line?

**Verdict: yes; no rule breach found.**

- **gate.log:** full `scripts/check.sh` from a cold cache, eight PASS lines in order (G 133-140, checked G 144), exit 0 and 0 stderr bytes (G 141-143).
- **test.log:** host and i686, `CARGO_TERM_COLOR=never` (T 3), test names and summary lines (T 84-143, 154-213), both counts (T 144, 214, 220), same names checked (T 221), P6 tests on both targets (T 138-140, 208-210).
- **negative.log:**
  - The tool is built and run from the main tree behind a HEAD and clean-tree guard (N 40-41), with archives under a mktemp directory (N 32-33).
  - Case (a): exit 1 (N 301, 303), `attrs` on exactly one row (N 277, 304).
  - Case (b): exit 1 (N 337, 339), `order_preserved` no (N 323, 342), no flags (N 340).
  - Fixture strings are built at run time (N 44-46) and counted in raw captured output with a positive control (N 215, 268, 302, 338).
- **scan.log:** phase diff files (S 334-379), both messages with the trailer check (S 382-402), the three other logs (S 405-432), controls (S 252-309), PASS (S 438).

**specs/0001 rules:**
- The first line of each log names the commit and HEAD (G1, T1, N1, S1).
- `check`, `note` and `structural` take values computed once, and each is printed once.
- Multi-condition checks are functions: `passed_matches` (T 26), `rows_flagged`, `same_length_differs` and `all_zero` (N 63-78), `files_complete`, `messages_complete` and `scan_passed` (S 196, 213, 233).
- 0 control bytes and 0 non-ASCII bytes in all four logs (my count).
- Paths are repo-relative: `<repo>`, `<scratch>`, `target/...`. G 5 prints a literal `$HOME`, not an expanded path.
- `/usr/bin/grep` is called explicitly.

**Minor points:**
- N's `rows_flagged` for case (a) does not pin which row carries `attrs`. The row itself is visible at N 277.
- S does not scan itself, as expected, and cannot. My count on it is 0 and 0.

## 3. Does every number and quoted string in the draft appear in a primary?

**Verdict: yes, with one imprecise description and several unverifiable items.**

**Checked against primaries:**
- **Primaries and Commits:**
  - The sha256 prefixes match the files and S 405, 414 and 423.
  - The 5d98cdc full hash matches S 313.
  - The trailer and message-count citations are right: S 383, S 393, S 402.
  - "5 files, 356 insertions, 19 deletions" and "2 files, 88 insertions, 11 deletions" are marked session output, but both match my count of D.
- **What changed:** byte ranges 10-11, 16-19, 20-23 and 24-27 are correct. "rising local header offset order" slightly overstates the code (ties are allowed, D 229); departure 3 covers this.
- **tests/p6.rs:** "columns 10 and 11" is correct (r[9] and r[10], D 536).
- **Acceptance G:** G 126, 133-140, 144, 141-143, 153 and 154 are all correct.
- **Acceptance T:** counts and lines 100, 112, 124, 131, 136, 141, 144, 138-140, 208-210, 170, 182, 194, 201, 206, 211, 214, 220-222, 224 and 225 are all correct.
- **Acceptance N:** N 4, 229-232, 233-237, 242-244, 270, 252, 253, 265, 267, 269, 271, 276-278, 304, 286, 305, 287, 306, 301, 303, 307, 312-314, 340, 322, 341, 323, 342, 335, 343, 337, 339, 344, 268, 302, 338, 346 and 348 are all correct. 21816 bytes and `xl/workbook.xml` are correct.
- **Acceptance S:** S 438, 309, 379, 402, 432, 434 and 437 are correct, including the timestamps.
- **Departure 4:** N 229-234 and the stored entries with zip64 extra fields (N 145-152) are correct.

**Imprecise:**
- "a stored entry whose declared compressed size takes in 40 bytes of the next local header" (draft 62-63). The diff says 40 bytes past the entry's data, "which is the start of the next local header" (D 515-516).
- The next header for "b.txt" is 30 + 5 = 35 bytes if its local extra is empty, so the 40 bytes would span the whole header and 5 bytes of data. D does not show `raw_stored`'s local extra, so this can't be confirmed either way.

**Unverifiable (session output or not in primaries):**
- Departure 1 (how zip 8.6.0 decodes the placeholder).
- Departure 2's claim that the decoded method and CRC equal the raw values, so those branches cannot fire. The `Meta` source is not in D.
- "Nothing is pushed".
- The record commit hash and "S itself: in the check header".

None of these contradicts a primary.

## Defects

1. **(Low; edge case, contrived archives)** The file-order check uses a strict `>` after a stable sort by input offset (D 226-229), so ties are handled asymmetrically.
   - Example (my reasoning, not run): the input has entries A and B at distinct offsets with byte-identical local headers and data (same local name). The output has both central records pointing at A's single local header. By my reading, that pair gives `order_preserved` yes and no flags, so exit 0.
   - Meanwhile `files_identical` and `central_start` differ, which contradicts the exit-0 paragraph's claim that such a difference can come only from a prefix or end records. That paragraph is the P6 acceptance claim.
   - A tie on one side only (equal offsets in one file, distinct in the other) should give `no`.
   - Under the P5 ruling's proportion rule, this likely belongs in the T1 backlog. Departure 3 names the output-tie half only.
2. **(Low; test gap)** No test isolates the individual branches of `raw_only`. The zip64 test changes both size fields at once, so dropping one size branch or mis-slicing between them would still pass. The method and CRC branches are never exercised.
3. **(Low; test gap)** The overrun test cannot tell whether the overlapped bytes were compared as data or where the gap starts, because both sides are identical. It proves only "no error, exit 0".
4. **(Low; draft wording)** "40 bytes of the next local header" (draft 62-63) is not what D says; see answer 3.
5. **(Cosmetic)** The permuted test's precondition (D 461-465) is tautological, since `raw_zip` now wraps `raw_zip_with_local_order`. The draft presents it (draft 55-56) as if it showed byte-for-byte equality with `raw_zip`. The real evidence that `raw_zip` writes the same bytes is the diff itself and the P1 to P5 tests still passing in T.
