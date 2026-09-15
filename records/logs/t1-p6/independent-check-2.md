# T1 P6 independent check 2

- Checked commits: 5d98cdc640dae3f6334041e35b74a65a1b6b3590 (T1 P6 phase
  commit 1) and dbe108c52efeae823bdb4464a7a884d6526fdb1a (phase commit 2,
  the fixes for check 1)
- HEAD when the check ran: 814740778864f11e008dbf45adf4695fbb5979eb
  (record commit with the logs rerun at dbe108c, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, and asked to answer in plain ASCII
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    c03f9b28fd8df5bea2ee5b2ec79a6b2714f773ab9fd27a3c7d0f5eaeb3564cf2
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff 1, `git diff 4f6f347..5d98cdc`, saved as <scratch>/t1p6-diff.txt
    c61c1dbd5e2663cac6b2d8dba547fd2ab1ff3a3f13c35f7b3b776a127cc47aae
  - phase diff 2, `git diff 63c9b01..dbe108c`, saved as <scratch>/t1p6-diff2.txt
    2b94918cfefa0cf4f5300ac826575810f7ce01dcb67dc8876791bbbe58785477
  - draft report 2, <scratch>/t1p6-report-draft2.md
    a88635b615303d2bd94db130a8c1980fc97af042c589f36f486afb83fae85931
  - records/logs/t1-p6/gate.log
    918d0a0a83a149efeda9826c8290f70e7d0846b21dc2f2a72b5d92a16e422f49
  - records/logs/t1-p6/test.log
    629e1a07f19204e03c07c77129567e5a7fcf9e03d8fa2760c8613feb6f323f9e
  - records/logs/t1-p6/negative.log
    497a1cb0f2dc67f258c8c283f0e86321bd0d3a89011a90963787344633eb7df4
  - records/logs/t1-p6/scan.log
    99f53a0d1b24b85a4d710b94661241b51e806774dec2d7e49a5f5149b768f30a
  - records/logs/t1-p6/independent-check-1.md
    0ca738226187c3c37c4a5a06362dfd50dddab198a38c715b5f27fa3c0ffd1b97
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
I read the P6 material in specs/0004-xlsx-diff.md (Output, Exit status, Streaming, the P5 rulings, the Phases preamble and P6), specs/0001-bootstrap.md lines 22 to 123 ("How this task runs" and "Rules for every phase"), D1 (t1p6-diff.txt), D2 (t1p6-diff2.txt), draft 2, G, T, N, S and I1, including the scripts printed in G, T, N and S. The harness also put CLAUDE.md and a memory index into my context; I did not use them as evidence. I ran only read-only commands. With sha256sum I hashed the four logs, I1, both specs, both diffs and the draft. With tr I counted control bytes (other than tab, LF and CR) and non-ASCII bytes in each log, I1, both diffs and the draft: all 0. With a small python3 counter I counted insertions and deletions per file in D1 and D2. I did not run git, cargo or the tool. D1 and D2 do not show the definitions of `Meta`, `central_fixed`, `header_start`, `meta()`, `text()`, `rows`, or `raw_stored`'s local extra field, or the central loop of the byte builder. Anything that depends on them is marked as inference or taken from the prompt.

## 1. Do the two diffs together do what P6 says, and nothing else?

**Verdict: yes for the code. Partly for the tests: every required test exists and asserts what the spec requires, but some of the new branches are not tested in a way that tells right from wrong.**

**`attrs`: `raw_only` (D1 180-191, joined with OR into the attrs condition at D1 200)**
- The central fixed layout is: signature 0-3, version made by 4-5, version needed 6-7, flags 8-9, method 10-11, time 12-13, date 14-15, CRC 16-19, compressed size 20-23, uncompressed size 24-27.
- `central_fixed` holds bytes 6 to 27 (prompt), so slice index k is central byte 6+k. Each range is compared against its own decoded value:
  - `differs(4, 6)` is bytes 10-11, compared with `a.method == b.method`.
  - `differs(10, 14)` is bytes 16-19, compared with `a.crc == b.crc`.
  - `differs(14, 18)` is bytes 20-23, compared with `a.compressed == b.compressed`.
  - `differs(18, 22)` is bytes 24-27, compared with `a.size == b.size`.
- All four are correct. The slices stop at 22, which is safe if `central_fixed` is 22 bytes long (not shown). The flag order is unchanged. Nothing new is printed.
- The size branches really do compare against decoded values. The only byte difference between `plain` and `placeholder` is the 8 central size bytes (D1 330-335, extra fields byte-equal). The test asserts `identical` with exactly `attrs` (D1 431-434) and passed on both targets (T 140, 211). So `attrs` came from `raw_only`, and `Meta` sizes are decoded values.
- Method and CRC branches: the draft says they cannot fire (departure 2, session output). I cannot check that from primaries. Nothing in the primaries contradicts it.

**`order_preserved` as D2 leaves it (D1 208, 216; D2 15-29)**
- The common (i, j) pairs are collected in input index order (D1 216).
- They are sorted by the key `(a.header_start, b.header_start)` (D2 18).
- For every adjacent pair, `cmp` of the input offsets must equal `cmp` of the output offsets (D2 19-27). Any mismatch gives `no`, ANDed with the central-directory check.
- After a lexicographic sort the input comparison is Less or Equal:
  - Equal requires Equal in the output. A tie on one side only now gives `no`: input tie with output Less fails; input Less with output tie fails.
  - Less requires Less.
- The output sequence is then non-decreasing with strict steps exactly where the input steps. By transitivity, every pair (not only adjacent pairs) agrees. The check is exactly "the same weak order by offset in both files". It is symmetric, and the I1 defect 1 hole is closed in code.
- Offsets are only compared, never computed. The new indices are `usize` vector indices, which is correct. `header_start` is used as an offset; its type is not shown (presumably `u64` from earlier phases).

**Is the exit-0 paragraph true by construction? Yes, by my reading of the P6 increments on top of the P5 flags.**
- Central record bytes:
  - 10-11 and 16-27: status when the decoded values differ, `raw_only` when they are equal.
  - A raw-equal field whose decoded value differs can only come from a differing zip64 extra, which is `extra`.
  - The other central bytes are as P5 covered them (I1 answer 1 maps each one).
- Local header to next header: unchanged from P5.
- Order: the central-directory check plus the D2 file-order check.
- A mirrored placeholder in both the local and central headers gives `attrs` through `raw_only`. A local-only placeholder gives `local`.
- I found no remaining hole in the P6 part.

**Required tests**
- **zip64 pair (D1 399-451):**
  - Both sides get a byte-equal central zip64 field (`zip64_extra(len, len)`). One side has placeholders in both size fields.
  - Preconditions: `meta` equal and bytes different.
  - Asserts exit 1, empty stderr, `identical` with `attrs` alone, `flagged` 1 and `order_preserved` yes.
  - Placeholders on both sides give exit 0 with no flags.
  - Matches the spec.
- **Permuted (D1 453-509):**
  - Local order 2, 0, 1 with the central order kept.
  - Asserts exit 1, every row `identical` with no flags, output indices 0 to 2, `flagged` 0, `order_preserved` no, `files_identical` no and the status counts.
  - Matches the spec. The positive sub-case (same permutation on both sides) is an addition.
  - The tautological precondition at D1 461-465 is still in the file. It is cosmetic.
- **Overrun (D1 511-541; comment D2 112-114):**
  - Asserts exit 0, empty stderr, three `identical` rows with no flags, and the declared size in columns 10 and 11.
  - "No error" and "exit 0" are shown. Because `identical` requires the raw comparison, the 52 raw bytes were read without error.
  - "The gap starting where the data ends" is not discriminated, because both sides are alike (I1 defect 3, not changed).
  - The comment's "30 fixed bytes and the 5-byte name, with no extra field" depends on `raw_stored` and `text` definitions that are not in the diffs.

**D2 single-field placeholder cases (D2 80-101, builder D2 35-71)**
- Builder: the compressed-only flag writes 0xffffffff in central bytes 20-23 and the plain uncompressed size in 24-27; the uncompressed-only flag does the reverse.
- With compressed only, only `differs(14, 18)` can fire; with uncompressed only, only `differs(18, 22)`. So each size branch now has a test that fails if that branch is removed.
- How the crate reads the field, as far as the primaries show:
  - APPNOTE puts in the zip64 field only the fields whose 32-bit value is 0xffffffff, in the fixed order uncompressed, compressed, offset.
  - The fixture keeps a 16-byte field (uncompressed then compressed) with a single placeholder, which is not that layout.
  - A reader following the rule takes the first u64 for whichever field is the placeholder and leaves 8 bytes unread. For the compressed-only case that means it reads the uncompressed value as the compressed size.
  - The D2 comment (80-82) covers this: for a stored entry both values are equal, so the decoded size is right whichever slot is read.
  - The primaries show that this held for zip 8.6.0 without an error: the `meta` precondition (D2 92) and the test passed at dbe108c (T 140, 211). They do not show which slot the crate read or that leftover bytes are skipped (draft departure 1 is session output).
- What the single-field cases still do not catch:
  - Because compressed == uncompressed in every fixture, a swap of `a.compressed` and `a.size` between the two size branches would pass every test.
  - So would a merged `differs(14, 22) && a.compressed == b.compressed`.
- I1 defect 2 is therefore only partly closed.

**Byte builder (D1 267-338, D2 35-71)**
- `raw_zip` now calls `raw_zip_with_local_order` with the identity order.
- For the identity order, `local_offsets[k]` gets the same values the old in-order push gave.
- With `declared_compressed` None and all placeholder flags false:
  - the local header writes `u32_of(raw.len())` as before (D1 311);
  - the descriptor does the same (D1 320);
  - the central record writes `u32_of(raw.len())` then `u32_of(data.len())`, the same 8 bytes as before (D2 61-70).
- The permutation `assert_eq` only panics on an invalid order.
- The central loop is not in the diffs. P1 to P5 still pass on both targets (T 89-137, 159-207).
- Conclusion: `raw_zip` writes the same bytes as before for existing callers, by reading. `usize` in test builder code is consistent with earlier phases.

**Content leaks:** none. No new printing in lib.rs.

**Beyond scope:**
- ROADMAP.md and the spec hunks in D1 are planning's doc commit 8a898b3 (S 320-323).
- The lib.rs module doc comment (D1 170-172).
- The two positive sub-cases and the D2 single-field cases are additions to the spec's minimum test list, made in response to I1. They are not scope creep.
- Nothing else.

**I1 defect closure**
1. Closed in code (D2 15-29). No test builds a shared local header; the draft says so.
2. Partly closed: each size branch is isolated, but the pairing is not (above). Method and CRC stay untested.
3. Not closed. The draft's reason for leaving it is in no primary (answer 3).
4. Closed (D2 112-114, draft 72-74).
5. Closed in the draft. The assertion stays in p6.rs.

## 2. Do the logs contain every line the spec requires?

**Verdict: yes. I found no broken rule and one small labelling imprecision.**

**gate.log**
- Full `scripts/check.sh` from a cold cache (G 130-132).
- The eight PASS lines in order (G 134-141, checked at G 145), 0 stderr bytes and exit 0 (G 142-144).
- Tree check at G 127.

**test.log**
- Host and i686 runs with `CARGO_TERM_COLOR=never` (T 4).
- Test names and summary lines (T 85-144, 155-214).
- Both counts stated (T 145, 215, 221), with same names and equal counts checked (T 222-223).
- The P6 tests ran on both targets (T 139-141, 209-211).

**negative.log**
- HEAD and clean-tree guard, printed as N 41-42 in the script listing.
- Archives go under a mktemp directory (N 33-34) and are deleted on exit.
- Case (a): exit 1 (N 302, 304); `attrs` on exactly one row (N 278, 305); `flagged` 1 and `order_preserved` yes (N 306-307).
- Case (b): exit 1 (N 338, 340); `order_preserved` no (N 324, 343); no flags (N 341); `files_identical` no (N 344).
- Byte-copy control: exit 0 (N 268-272).
- Fixture strings are built at run time (N 45-48). The generator holds only a split phrase template (N 47, 137).

**scan.log**
- Controls for every form (S 253-310).
- The 10 files at dbe108c (S 350-450). These include the superseded logs; their hashes at S 369, 387, 396 and 405 equal I1's input hashes, so I1 saw exactly those.
- 5 messages with the trailer check (S 453-503).
- The three new logs (S 506-533).
- PASS (S 539).

**specs/0001 rules**
- **First line names the commit and HEAD:** G 1, T 1, N 1, S 1.
- **Rerun header:** G 3, T 3, N 3 and S 3 name record commit e8280e3 and the fix commit. The text comes from `RERUN_NOTE`, set outside the printed scripts; only the printed value is visible.
- **Each check evaluated once and printed once:** yes in all four scripts.
  - `check`, `note`, `structural`, `ctl` and `check_ctl` take one value each.
  - The `lines=$(f_ctrl ...)` inside labels (G 87-90) is informational, not a second evaluation of the checked value.
- **Compound checks in one function:**
  - T: `passed_matches` (T 27).
  - N: `all_zero`, `row_is`, `rows_flagged` and `same_length_differs` (N 64-79).
  - S: `files_complete`, `messages_complete` and `scan_passed` (S 197, 214, 234).
  - I found no `$(yn [ ] && ...)` form.
- **Control and non-ASCII bytes:** 0 in all four logs (my count). S's own `f_ctrl` result on G, T and N is 0 (S 508, 517, 526).
- **Paths:** repo-relative or `<scratch>`. `$HOME` appears only literally (G 6, 29, 46; S 21). `/tmp/claude` appears only inside the scan pattern (S 40) and the split control (S 133).
- **Fixture text in captured tool output:** none. The xlsx-diff output is counted at N 269, 303 and 339. gen.py output (N 230-233) is passed through `hide` (N 175) but not counted. It visibly holds only archive names and sizes.

**Limits, not breaches**
- S cannot scan itself, and it cannot scan the message of the later record commit that carries the rerun logs.
- `rows_flagged` does not pin which row carries `attrs`; the row is visible at N 278.

## 3. Does every number and quoted string in draft 2 appear in a primary?

**Verdict: yes for every line-number citation I checked, all of which are right. There are two imprecise statements and several unverifiable items. Nothing contradicts a primary.**

**Primaries (draft 11-22)**
- D1 is +356/-19 over 5 files (my count: ROADMAP.md +6/-1, spec +82/-10, lib.rs +29/-1, common/mod.rs +42/-7, p6.rs +197/-0). This matches I1 30. The D1 file's sha256 c61c1dbd...7aae equals I1's input hash (I1 15).
- D2 is +51/-10 over 3 files (my count: lib.rs +13/-6, common/mod.rs +12/-2, p6.rs +26/-2). It is marked session output but matches.
- The log hashes match both the files and S:
  - gate.log: my sha256 918d0a0a83a1...2f49 equals S 506.
  - test.log: 629e1a07f192...3f9e equals S 515.
  - negative.log: 497a1cb0f2dc...7df4 equals S 524.
- scan.log itself is 99f53a0d1b24b85a4d710b94661241b51e806774dec2d7e49a5f5149b768f30a. The draft defers it to the check header.
- I1 on disk is 0ca738226187...1b97, equal to S 378.

**Commits (draft 24-34)**
- Checked against S:
  - 8a898b3 is the doc commit (S 317, 320).
  - The 5d98cdc and dbe108c full hashes (S 314, 316).
  - e8280e3 holds the four logs (S 331-336).
  - 63c9b01 holds I1 (S 338-340).
- "2 files, 88 insertions, 11 deletions" is marked session output but matches D1's doc hunks.
- "Committed without editing" is unverifiable.
- Trailer checks at S 454, 464, 474, 484 and 494, and 5 messages at S 503: correct.
- The rerun record commit hash is unverifiable.

**What changed (draft 36-89)**
- Byte ranges 10-11, 16-19, 20-23 and 24-27 are correct given `central_fixed` = bytes 6-27.
- The D1 and D2 order descriptions are correct (D1 226-231; D2 18-27).
- The builder field descriptions are correct (D1 245-251; D2 39-42).
- The three test summaries are correct (D1 399-541; D2 80-101). "columns 10 and 11" is `r[9]`, `r[10]` (D1 536).
- The I1 defect numbers in the D2 list are correct.
- "35-byte local header and the first 5 bytes of that entry's data" matches D2 112-114. The premises (no local extra, `text(12)` is 12 bytes) are not in any primary.

**Acceptance, G:** lines 3, 127, 134-141, 145, 142-144, 154 and 155 are all correct. The timestamp 20:51:02Z is correct.

**Acceptance, T:**
- Lines 101, 113, 125, 132, 137, 142, 145, 139-141, 209-211, 171, 183, 195, 202, 207, 212, 215, 221-223, 225 and 226 are all correct.
- The counts 11, 10, 10, 4 (+1 ignored), 3, 3 and 41 are correct. 20:51:03Z is correct.

**Acceptance, N:**
- Lines 5, 230-233, 234-238, 243-245, 271, 253, 254, 266, 268, 270, 272, 277-279, 305, 287, 306, 288, 307, 302, 304, 308, 313-315, 341, 323, 342, 324, 343, 336, 344, 338, 340, 345, 347 and 349 are all correct.
- 21816 bytes and `xl/workbook.xml` are correct.
- Imprecise: "every captured output: 0 fixture lines, 0 control bytes, 0 non-ASCII bytes (N 269, 303, 339)" covers the three xlsx-diff runs only, not gen.py's output.

**Acceptance, S:** lines 539, 310, 450, 503, 533, 535 and 538 are correct. 20:51:04Z is correct.

**Departures (draft 132-154)**
- Departure 1 is session output and unverifiable. It is consistent with the passing precondition (D1 417-421). It does not mention that D2's single-placeholder fixtures keep a 16-byte field that departs from the APPNOTE layout.
- Departure 2 is session output and unverifiable.
- Departure 3 matches D2 and I1 defect 1.
- Departure 4 is correct: N 131-170 is the generator; N 230-235 are the testzip lines and notes.

**Findings and Independent checks (draft 156-182)**
- The defect list matches I1 162-172.
- The response to defect 3 ("making the overlapped bytes differ turns the entry into a decompression error rather than a gap difference") is in no primary and is not marked session output. The "(findings)" it points to does not explain it.
- "Committed verbatim in 63c9b01": the file is added in 63c9b01 (S 340). "Verbatim" is unverifiable.
- "Nothing is pushed" is unverifiable.

## Defects

1. **(Low, test gap)** `raw_only` pairs each size range with its own decoded value, but no test can tell. Every fixture is stored, with compressed == uncompressed. Swapping `a.compressed` and `a.size` between the two size branches (D1 189-190), or merging them into one `differs(14, 22)` test, would pass every test. I1 defect 2 is only partly closed. The method and CRC branches stay untested (acknowledged).
2. **(Low, test gap)** The D2 file-order change (ties on one side give `no`; ties on both sides give `yes`) has no test (acknowledged in draft departure 3). The builder cannot write a shared local header.
3. **(Low, test gap, carried)** The overrun test does not show where the gap starts (I1 defect 3). The draft's reason for leaving it (draft 173-174) is an unmarked claim that no primary supports.
4. **(Low, fixture and draft)** The D2 single-field fixtures put one placeholder beside a 16-byte zip64 field, which is not the APPNOTE layout. They pass only because both sizes are equal and zip 8.6.0 accepts the leftover bytes. The test comment half-says this (D2 80-82); the draft does not.
5. **(Cosmetic, draft)** Draft 125 "every captured output" counts only the three xlsx-diff runs (N 269, 303, 339). gen.py's output is filtered (N 175) but not counted.
6. **(Cosmetic, draft)** Draft 72-74 and D2 112-114 state a 35-byte header with no local extra field and 5 data bytes. Those premises (`raw_stored`, `text`) are not in any primary.
7. **(Cosmetic, code)** The tautological precondition is still in p6.rs (D1 461-465). The draft no longer cites it.
