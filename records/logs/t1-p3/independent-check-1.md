# T1 P3 independent check 1

- Checked commit: 9f124c7f0925c6a96dd1fecc3ccf72790b9c8e6e (T1 P3 phase
  commit)
- HEAD when the check ran: 171f980a45154dde5805a58ba1ea3d2ac8324027
  (record commit with the logs at 9f124c7, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, and asked to answer in plain ASCII
- Inputs, with sha256:
  - specs/0004-xlsx-diff.md
    3fa9b6a51a2e762cc4a9aae48d7122cbc6f8093d7cf4c1a9b44953d7633e9c3f
  - specs/0001-bootstrap.md
    4b829d12609e6c5624849a7a226a80188bb8bdf9deb39132406247a505d6e2cb
  - phase diff, `git diff f0adb45..9f124c7`, saved as <scratch>/t1p3-diff.txt
    d72bf90a97f5c6e0bc37a48b8c0638ab9874edb51b3c34c1c4d830458a26abd1
  - draft report 1, <scratch>/t1p3-report-draft1.md
    d6d70b7e2c52b26d850f343207dfc4a18a13d89e38e34556686e187fd6d96a17
  - records/logs/t1-p3/gate.log
    67146493830d4aa0397eb971c551cb7a10d465f55990bdad769e625da76216e3
  - records/logs/t1-p3/test.log
    b7538f89de51898682fb73ccbede2a0afa573355507555dd14dce22a70ee009b
  - records/logs/t1-p3/negative.log
    e8909301073afed785d8846bab98db4fcfb18c49b04796f4f055bcd2c84fa3ce
  - records/logs/t1-p3/scan.log
    af9c3f0aa7bf84185a1837b2635bff1c7ef06711c41f06356a704f4082615b91
- Answer: the checker's final message, extracted verbatim from its
  transcript with python; privacy-scanned before commit

---
I read the eight listed inputs: the P3 section of specs/0004 with its Output, Exit status, Streaming, "Planning rulings on P2" and Phases preamble; "How this task runs" and "Rules for every phase" in specs/0001; the phase diff D (1361 lines); the draft report; and G, T, N and S, including the scripts printed inside them. I ran read-only tools only. `sha256sum` over the four logs. Line counts over D (5 `diff --git` headers; 888 added and 122 removed lines, not counting the `+++`/`---` headers). Control-byte and non-ASCII byte counts over the four logs (all 0). A search for `/home/` in the logs (one hit, S 140, the scan's own control). CLAUDE.md and memory were put into my context by the harness; I did not use them as evidence. I did not read lib.rs or the test files outside D, so any code not shown in D's hunks cannot be checked here.

## 1. Does the diff do what P3 says, and nothing else?

**Verdict: yes on behaviour, with test gaps and minor additions noted below.**

**Archive rows (D 531-552).** All 21 keys are written in the spec's order, each exactly once:
- `flagged` counts entries whose flags column is not empty (D 493-496).
- `order_preserved` (D 475-485) walks the input in index order and sets `no` when an output index goes down. Duplicate names are rejected at open, so output indices are distinct and this matches "same relative order". Dropped and added entries do not take part, which matches the spec.
- `comment_equal` compares `zip.comment()` bytes (D 180, 526).
- `prefix_*` come from `zip.offset()` (D 181) and `central_start_*` from `central_directory_start()` (D 126).
- `size_sums` adds `u64` values with `checked_add` (D 423-431).
- `bytes_*` are the `u64` file length from metadata (D 116-119).
- `files_identical` (D 437-457) returns `no` when the lengths differ. Otherwise it reads both files through two 64 KiB buffers and fails on a different count or different bytes. It ends when a read comes back short, and a file whose length is an exact multiple of 64 KiB ends correctly on the following 0-byte read.

**Flags (D 339-362).** The order is mtime, extra, gpflags, descriptor, attrs, comment, as in the spec.
- `mtime`: central DOS time or date.
- `extra`: local or central extra field.
- `gpflags`: the full 16-bit central flags.
- `descriptor`: bit 3 differs.
- `attrs`: external attributes or version-made-by, whose high byte is the system.
- `comment`: raw central comment bytes.

Flags are computed only for entries on both sides; dropped and added rows pass "" (D 512, 520). All six definitions match the spec.

**Central record parsing (D 198-255).** The offsets are correct: version made by 4, flags 8, time 12, date 14, name length 28, extra length 30, comment length 32, external attributes 38-41. The loop now reads name, extra and comment one after another instead of seeking, which is the on-disk order, so the old `checked_add` on the position is no longer needed. A short field read is an error (D 231-237). The loop ends at the first non-central signature. That `break` sits in unchanged lines that D does not show (between D 215 and D 217), so it could not be checked here.

**Local header (D 259-283).** The fixed length 30, signature, name length at 26 and extra length at 28 are correct. A short header or wrong signature is `Error::Entry`, and a short extra read is an error. One edge: when the extra length is 0 and the name runs past the end of the file, no error is raised here. The later raw read would catch that, so it is not a real bug.

**Other points.**
- Column 14 (D 329): `first.unwrap_or(min)`. This is correct only if the unchanged loop never sets `first` beyond the shorter length. That loop is not in D, and the tests use single-chunk entries (1000 and 600 bytes), so the multi-chunk different-length case is unverified.
- Exit status (D 554-559): recompressed, changed, dropped, added and flagged all 0, plus order preserved and comments equal. This matches "Exit status".
- Error mapping:
  - a local header failure is `Error::Entry` (D 265);
  - a sum overflow is `CentralDirectory` (D 424);
  - a read error in `files_identical` is `Error::Open` (D 441-449);
  - the new guard that the crate's flags and external attributes equal the record's is `CentralDirectory` (D 148-153).
- Nothing new prints a comment, extra field or path: flags are fixed words and names go through `escape_name` (D 395).
- `u64` use: sizes, sums and file lengths are `u64`. `entries_in` and `entries_out` are now passed as `usize` (D 531-532), where P2 cast them `as u64`. The printed value is the same, but this is a literal departure from "every ... count is u64".

**Required tests, all present:**
- Reordered entries: D 1061-1105, asserts identical, `order_preserved no`, exit 1.
- Archive comment: D 1108-1131.
- mtime, extra, comment and external attributes each alone: D 1134-1237. Each case asserts the exact flag string, so "alone" is proven, plus `flagged 1` and exit 1.
- Descriptor on one side: D 1240-1263, in both directions, expects `gpflags,descriptor`.
- `files_identical`: D 1298-1327, over 64 KiB. Yes on a byte copy; no at equal length with the last byte different; no at a different length. The different-length case asserts nothing but `files_identical`, and no equal-length case has the difference in a middle chunk.
- Prefix: D 1330-1361, with `prefix_in 1000` and `central_start_in` equal to the recorded offset plus 1000.

**P2 ruling changes, all present.** Different lengths expects 600, 600 and 5 and checks columns 4-13 and both sizes on every row (D 922-932). The large entry gains `assert_columns` (D 950). `entry_of_exactly_two_buffers` gains stderr and status assertions (D 975-979).

**Test gap.** `every_archive_row_in_order` checks all 21 values, but only for identical files, so every in/out pair is equal. No test asserts a differing `uncompressed_out`, `compressed_out`, `bytes_out` or a non-zero `prefix_out`. A bug that printed the input's value for one of these output rows would pass every test (`leading_junk_bytes` asserts `bytes_in` only). The code is correct on reading (D 527-551), and N 213-214 shows distinct `bytes_in` and `bytes_out`.

**Byte builder `raw_zip` (D 729-798), correct.**
- Local header: 30 bytes, CRC then compressed size then uncompressed size. With bit 3 these are zeroed, and a descriptor with signature 0x08074b50 plus CRC and both sizes follows the data.
- Central header: 46 bytes. Disk start and internal attributes are the 4 zero bytes, and the offsets are relative to the end of the prefix, which is the "junk prepended" layout.
- EOCD: the fields are correct.
- The rewritten `single_deflate_entry_zip` produces the same bytes as the removed code: version 20, date 0x0021, 16 zero bytes, same EOCD.
- `recorded_central_offset` searches `0..len-21` backwards, which is correct; it would panic under 21 bytes, but it is a test helper only.

**Beyond scope, all inside `tools/xlsx-diff`:**
- the crate-vs-record guard and the local header read at open, which add new exit-2 paths (draft departures 1 and 3);
- extra tests (departure 5);
- the `recompressed_entry` precondition and `attrs` expectation (departure 2);
- two new helpers;
- doc comment edits.

**Spec-level observation, not a diff defect.** Differences only in a local header are not flagged, and "version needed to extract" is compared nowhere. This covers local time, local flags and version needed. Such a pair gives exit 0 with `files_identical no`, which is what the spec's wording allows.

## 2. Do the logs contain every required line?

**Verdict: yes, with one rule tension.**

**Gate and test logs.**
- G: eight PASS in order (G 132-139), and the exact-match check (G 143).
- T: host and i686 with colour off (T 3), test names and summary lines (T 83-123 and T 134-174), both counts (T 181), same names (T 182).

**Negative log.**
- Case (a) has exit 1 (N 217, 219), three identical unflagged rows (N 192-194, 220) and `comment_equal no` (N 204, 221).
- Case (b) has exit 0 (N 255, 257) and `files_identical yes` (N 253, 259).
- The fixture counters have positive controls (N 172-175) and show 0 fixture lines, 0 control bytes and 0 non-ASCII bytes (N 218, 256).
- Sentinels are built at run time and the phrase is split in the script (N 40-42), so no value is printed.
- The regex in `entry_rows_unflagged` (N 113) matches exactly 14 trailing columns with column 17 empty, which is correct.

**Scan log.** It covers the 8 files, both messages and the 3 other logs (S 411, 434, 464), with controls (S 308), and ends PASS (S 470).

**Rules from specs/0001.**
- The first line names the commit and HEAD in G 1, T 1 and N 1. S 1 names the range, whose end is the commit, and HEAD.
- Every check is evaluated once. Compound checks are functions: `passed_matches` (T 26), `all_zero`, `entry_rows_unflagged`, `equal_before_comment_length` (N 59, 111, 117), and `files_complete`, `messages_complete`, `scan_passed` (S 195, 212, 232).
- There are 0 control bytes and 0 non-ASCII bytes in all four logs (my count).
- Paths print as `<repo>` or `<scratch>`. `$HOME/.config/...` appears only as unexpanded literal text (G 5; S 20).

**Tension.** The negative script builds and runs in the main working tree (N 23, 32, 36-37, 83). specs/0004 allows "or against temporary files", which the archives are. specs/0001 "How this task runs" says a negative test never runs in the main working tree.

**Not in any scan.** The record commit message (171f980) is scanned by no log. This is inherent, since that commit is made after the scan runs.

## 3. Does every number and quoted string in the draft appear in a primary?

**Verdict: yes for every cited number, with one misleading item and several unverifiable ones.**

**Primaries section.**
- 5 files, 888 insertions and 122 deletions match my count of D.
- The sha256 values for G (67146493...), T (b7538f89...) and N (e8909301...) match both the files and S 437, 446 and 455.
- S's own sha256 is af9c3f0aa7bf84185a1837b2635bff1c7ef06711c41f06356a704f4082615b91 (not quoted in the draft).

**Commits section.**
- 9f124c7f... matches G 1.
- S 415, 425 and 434 are correct.
- "committed without editing" (session output) is unverifiable; S 315-319 shows only the three doc files.

**What changed.** Every statement matches D, except one. "`archive_rows` returns all rows" is listed as a tests/common change, but D does not change `archive_rows` (D 596-601 are context lines only).

**Acceptance.** Every citation checked is correct:
- G 125, 132-139, 143, 140-142, 152, 153;
- T 99, 111, 121, 124, 112, 163, 150, 162, 172, 175, 181-183, 186;
- N 4, 186-187, 192-194, 220, 202-204, 221-223, 215, 224, 217, 219, 225, 230-232, 258, 242, 260, 251-252, 253, 259, 255, 257, 261, 218, 256, 265;
- S 470, 308, 411, 434, 464, 466, 469.

**Departures.**
- 1: the read.rs line numbers are session output and unverifiable.
- 2: the values 0x0314 and 0x030a appear only in a D comment (D 859-860). The test asserts only equal and not-equal (D 865-874), so the values are unverifiable but consistent.
- 3 and 4: the quoted error texts come from the Error Display code, which is not in D. The texts are unverifiable; the mappings are confirmed (D 265, 441-449).
- 5: confirmed at D 1143-1197, 1266-1295 and 1096-1104. Bit 11 is 0x0800 at D 1177.
- 6: confirmed (D 1259).
- 7: session output, unverifiable.

No unverifiable item contradicts a primary.

## Defects

1. **Untested rows.** No test asserts a differing value for `uncompressed_out`, `compressed_out`, `bytes_out` or a non-zero `prefix_out` (D 1030-1056, 1346-1360). A bug that printed the input's value in one of these output rows would pass the suite. The code is correct on reading.
2. **Column 14 across chunks.** Column 14 for different lengths across more than one 64 KiB chunk is untested. The loop that sets `first` is outside D, so the P3 fix at D 329 is proven only for single-chunk entries.
3. **Negative run location.** The negative script builds and runs the tool in the main working tree (N 32-37, 83). specs/0004 permits temporary files; specs/0001 "How this task runs" says never in the main working tree. Planning should rule on this, or the report should note it.
4. **Draft wording.** The draft lists "`archive_rows` returns all rows" as a change; D does not change that function.
5. **Counts not `u64`.** `entries_in` and `entries_out` are written from `usize` (D 531-532), a regression from P2's `as u64` against "every ... count is u64". The output is unaffected.
6. **Weak `files_identical` cases.** The different-length case asserts only `files_identical`, with no status or stderr (D 1322-1326). No equal-length case has its difference in a non-final chunk.
7. **Spec-level observation.** Local-header-only differences and version-needed-to-extract are never compared, so exit 0 can come with `files_identical no`. This is as the spec is written, but worth planning's attention.
