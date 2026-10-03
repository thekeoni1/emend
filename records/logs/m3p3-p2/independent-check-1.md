# M3 part 3 P2, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec specs/0025, planning's working ruling section of it for P2, the committed specs/0024, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, each one's stderr, the phase's four records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p3-p2. Checked commit (the phase commit): a6cf46ff893e8eb2117435a24692eb5787f49370; HEAD when the check ran: a6cf46ff893e8eb2117435a24692eb5787f49370. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 3d77fd0b977d177be3376f910dff5c9959f3556cfce6a08a5a07992347335fe5. Its items on the report are corrected in the report; its observations on the code and the logs are reported to planning. No second check was run.

Inputs and their sha256:

- 4fa2421f9d1d308fad6dbe1a22e73973896c5d273a301129b3dd7d2dbf01f629 code-commit-1-carry.diff
- 616fb35eee39f904f889182872ba138d96a506b2d0c818233fb19fc8aaa44d22 code-commit-2-crate.diff
- 17271eefebbeee320aa6be1d1673fc341851cce76db19186f467031006001b72 code-commit-3-tool.diff
- 5b479fc28bfd9e3a5f573fd4305f43ab150ce9e5f5573e5d6db9848b18830b9f commit-messages.txt
- d937d7af9763a84707979a2c7e70a80997980f0ce6dc32a3ac55e3687ebdcb05 doc-commit.diff
- b585f829b0584a41c13e1a85e14c8e2bff41d5d7eb8736a0de88f73519e2bb82 gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- b5822723b21b9d39b447f318fbbc0c91abe66941a2f488ca6c78c279e21f4ca2 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- 6ce75fae596a63922468de19c3c4e81448a403281630d347e0b6acadc7991e95 phase.diff
- 596ce7c7195bf8ddb3d0d3029de8ccbf70f63c3b6c0c1c69ff8a6db7fb36f460 records/m3p3-open-2026-10-03-a6cf46f.md
- 3e1ce13a913a56b112cb3c856e183e50720d3b78b56f57f16558d403920edbce records/m3p3-open-2026-10-03-a6cf46f.txt
- 0ab6e1f3b013fbba6c30d1cfa4db88a3ae3d7e9ae0cb8ed60fe646fb71cef333 records/m3p3-p2-2026-10-03-a6cf46f.md
- 230ebd432e90d8ed6f02c77474e17acc120abdb296bb3d4c0bc59f1706f961d8 records/m3p3-p2-2026-10-03-a6cf46f.txt
- 365e7235fbc057af1dc9b44343929e968d2ac71e8bf39a783c7d88d1f789872d report-draft.md
- bd0293bc94760d37c69a0621bb0aa544d4f43070ced9203121ef402e17f23a60 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- b132e6bb20cdc107d8005143442a245b2f361fda41af984b59f8ea1471c7cc71 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- ae1ab201b5c7071c36235f2c5d8c253c6badfab95a9f163575ef7daa0e78863a spec-0024-committed.md
- 6c6e87e3c6ca75ac23f2f52488cd95ae8f294c361e12a5cf9d4a1b9e0f0c026f spec-0025-committed.md
- f80803c87e4edf1092e817ed9fe56f313bc32c5b13a19d02af5f66c27226a5d6 spec-0025-working-rulings-p2.md
- 675711098c8cc79f5af9405856aa3f712e0d353b8945939bce3344d593996c35 test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

---

# Independent check 1: M3 part 3 P2

Inputs verified first: `sha256sum -c ../inputs.sha256` reported OK for all 25 listed files and 0 failures. I read only files under the inputs directory and ran no git command.

One limit applies to everything below. The input spec-0025-working-rulings-p2.md has sha256 f80803c8... It is planning's "Planning rulings during P2" section on its own. Every log hashes the whole working spec as 75fed424... (gate.log, test.log line 26, negative.log line 71, run.log line 2041, scan-pre-check.log line 8). I cannot confirm from the inputs that the excerpt matches that section of the working copy.

## Question 1: does the diff do what the spec and working rulings say for P2, and nothing else?

Verdict: YES with reservations.

### Evidence that it does

- **Commits and files.** The three code-commit diffs together equal phase.diff exactly: I sorted the +/- lines of each and the diff was empty. Each commit stays in its own area (gate.log 2078-2097). No manifest or lock file changes (gate.log 2125-2128).
- **The carry commit.**
  - package.rs changes only `///` lines: 12 lines added or removed, 0 of them other than `///` (gate.log 2104-2119; code-commit-1-carry.diff 5-20).
  - The new wording matches P1 verification ruling 2: the qualified name as written, so a prefixed attribute of the same local name is kept.
  - The view-only test is at code-commit-1-carry.diff 32-53. It writes a view on a part with two `<sheetPr>`, keeps both, and has the tab-colour refusal as its control.
  - Ruling 2 says "the `//` fix"; the comment is `///`, which is the doc comment it names.
- **Types.**
  - The six types are at code-commit-2-crate.diff 466-579 and are exported at lines 10-12.
  - The derives include `Copy` and `Eq` beyond the spec's list, as R6 accepted for `SheetView`.
- **Attribute merge, ruling 3 order.** Each element writes its set attributes in this order:
  - `printOptions`: headings, gridLines, horizontalCentered, verticalCentered (621-631).
  - `pageMargins`: left, right, top, bottom, header, footer (651-660).
  - `pageSetup`: paperSize, scale, fitToWidth, fitToHeight, pageOrder, orientation, blackAndWhite, draft, firstPageNumber, useFirstPageNumber, copies (704-734). This is the survey's order, run.log 2391-2392.
  - `headerFooter`: differentFirst, differentOddEven (799-808).
  - Every set goes through `merged_tag` (1196).
  - The test at 1465-1530 pins it: set attributes first, then every kept attribute in input order, `r:id` last as found (line 1516), and `gridLinesSet`, `cellComments` and the Dpi pair kept.
  - The run log confirms the order against all 95 parts (run.log 2400-2402).
- **Creation in schema position.**
  - `create_before` gained a `below` bound (1095-1127). It is called with `VIEWS_POSITION` before the `CREATED` elements and with `usize::MAX` after the hyperlinks and before the drawing (347-365, 402-428, 432-450).
  - The test at 1419-1443 puts all four after a created `<mergeCells>`. Plant Z2 shows that without the bound they would land before it (test.log 4760-4769).
  - On a prefixed empty root they are created with the root's prefix (1444-1461; `Held::created` 905-920).
- **pageMargins defaults on creation.** All six attributes are written with 0.7, 0.7, 0.75, 0.75, 0.3, 0.3 (651-677; `created` is `held.depth == 0` at 1177/1187). Test 1370-1382, plant Z5.
- **fit_to_page.**
  - It touches or creates `<sheetPr>` (1008-1013, 1028-1033).
  - It merges `fitToPage` into an existing `<pageSetUpPr>`, or creates one in 18.3.1.82 order after `<tabColor>` (1276-1295).
  - Tests: under a created `<sheetPr>`, under an existing one, on an existing `<pageSetUpPr>`, and with a tab colour (1533-1575). Plant Z4.
- **first_page_number.** It writes `useFirstPageNumber="1"` with it (727-731), tested at 1399 and 1516.
- **Header and footer.**
  - Children follow `HEADER_FOOTER_CHILDREN` order through `insert_ordered` (874-881, 932-951, 1302-1320).
  - `Some("")` drops a child (1311-1312); `None` keeps it (1307-1309).
  - An even or first text sets its flag (799-808); the flags are otherwise kept by the merge.
  - An element left with no attribute and no child is dropped (1215-1233).
  - Tests at 1578-1688 cover replace, drop, keep, flags kept, element dropped, and escaping. Plant Z3.
- **section per ruling 2.** It adds `&L`, `&C` and `&R` before non-empty parts and passes the parts through with no doubling (754-763). Tested at 1691-1713, including "R&&D" passed through and an all-empty section giving no element.
- **Bounds per ruling 1.**
  - Scale is 10..=400; paper size, first page number and copies are at least 1; fit counts are any `u32` (740-745).
  - Margins must be finite and at least 0 (247-257).
  - The refusals and the boundaries 400, 1, `u32::MAX`, 0.0 and 255 characters are tested at 1716-1794. Plant Z6.
- **Header message.**
  - The constant is at 319-322 and it is raised at 271-284.
  - It is catalogued with its tests (20-44).
  - negative.log 1080-1083 reads exactly three sheetprops constants.
- **Refusals.**
  - Two of an edited element, or one out of order, gives the existing message (`observe`, 1073-1087).
  - This is scoped to elements the write edits (`touches`, 1005-1021), as R4 ruled.
  - Tests at 1797-1842, with the control at 1835-1841.
- **Other P2 tests.** Two calls on one part (1845-1888) and a sheet added in the same write (1891-1912).
- **Tool.**
  - The sample's settings are the spec's constants (code-commit-3-tool.diff 282-313).
  - Its line has `pagesetup`, the four before/after pairs, `rid_kept`, the five ok flags, `hf_kept` and `r_bound_before` (321-343).
  - Its assertions are at 357-393. `rid_kept` compares the `:id` attribute's bytes before and after (228-233, 363-365).
  - The output is removed on failure (316-319).
  - The tool tests cover a created case, a kept case (with `r:id` and a `firstHeader` kept), and the refusal of two pageSetup (425-502).
- **Nothing else.** The other changes all serve P2:
  - `ScannedTag::end` made crate-visible (1917-1922), as its commit message states.
  - The `Recorded` plumbing (47-63, 67-90, 289-300).
  - Comment wording.

### Reservations

1. **Replace test lacks one attribute.** P2 asks for "each replaced on a part holding it with every corpus attribute". The `<headerFooter>` in the replace test carries `differentFirst` and `alignWithMargins` but not `differentOddEven` (code-commit-2-crate.diff 1469), which 45 corpus parts carry (run.log 2405). `differentOddEven` is kept in another test (1650-1662), but not in the replace test. The other three elements carry every attribute the survey shows.
2. **Placement against other created elements is untested.** Creation relative to other created elements is tested only against `<mergeCells>` (1419-1443). Nothing tests it against a created `<hyperlinks>`, `<drawing>`, or `conditionalFormatting` / `dataValidations` from the rules feature.
   - The hunks show the settings call placed after `links.create_before` and before the drawing (355-369, 418-431, 440-453), which matches the schema for those two.
   - Where the rules feature creates its elements relative to that call is outside the diff's context lines, so I cannot verify it from the inputs.
3. **pageSetUpPr order may not be what ruling 3 meant.** Ruling 3 lists `pageSetUpPr` as "`autoPageBreaks` (never written by this part) then `fitToPage`". The code writes the set `fitToPage` first and the kept `autoPageBreaks` after, so `<pageSetUpPr autoPageBreaks="0" fitToPage="0"/>` becomes `fitToPage="1" autoPageBreaks="0"` (1548-1550).
   - This follows "The pass" (set first, kept after).
   - It reverses the input order on corpus parts shaped `autoPageBreaks,fitToPage`: 31 sequences (run.log 2408).
   - The run log's order check covers only the attributes the part writes (run.log 2400). Planning should confirm this reading.
4. **Catalogue text promised for P2 is only in `///` comments.**
   - P1 ruling 3 (R3) says "P2's catalogue sentence" states that separators are not kept.
   - Ruling 1 says "the catalogue says so" for Excel's limits above the bounds.
   - Neither sentence is in the `//!` catalogue in this diff (phase.diff 26-43). Both appear only as `///` comments (carry lines 13-15; crate diff 206-207).
   - This may be deferred to P4's "catalogue and limits"; it should be stated.
5. **Minor.**
   - The 256-character refusal uses `first_footer`, not a header (crate diff 1759-1761).
   - P2's "a negative margin, a NaN margin" is covered by -0.1, NaN and infinity (1746).

## Question 2: do the logs contain every required line?

Verdict: YES with reservations.

- **Gate.**
  - Off and on, both targets: 513/540 on host and i686 (gate.log 2601-2628).
  - The `pub fn` / `pub const` lists off and on, with the additions being exactly the four calls and `section` (gate.log 2283-2460).
  - The consumer crate check for the ten items (2630-2692).
  - Ends "all as expected: yes" (2708).
- **Test.**
  - Workspace 960 on both targets (test.log 4072, 4081, 4853-4855).
  - Crate tests off and on, both targets, with "failed tests:" lines (4297-4340).
  - One `EMEND_KEEP_DIR` run: 785/785 with 1332 kept files (4342-4357).
  - The module count of 27 against 28 is explained by a printed line, as `specs/0024` P2 ruling 6 requires (4359-4370).
  - Six plants, each failing at its own assertion line, 6 distinct lines (4729-4840); each planted run prints the failing test's name.
  - Ends yes (4869).
- **Negative.**
  - Two `<pageSetup>` (P1, control P1C), `<pageSetup>` before `<pageMargins>` (P2, control P2C), and two `<headerFooter>` (P3, control P3C).
  - Each refusal is exit 2 with the exact message and no output (negative.log 1115-1189).
  - The two unreachable messages are stated with their covering tests (1199-1218).
  - Ends yes (1243).
- **Run.**
  - Both targets: 27 of 27 ok; lo and lo_in ok 27; `order_preserved` yes; `files_identical` no.
  - Entry totals: identical 843, changed 27, flagged 17 (descriptor only), recompressed, dropped and added 0 (run.log 3297-3305 and the i686 equivalents).
  - The comparison flags hold in 27 of 27 (3497-3500).
  - 19 plants, each turning its own flag on the created case (f9efcac9808f) and on the kept case (d8245b84086f) (3508-3576).
  - The survey covers 95 parts and 27 workbooks and prints attribute names and counts (2241-2420).
  - TMPDIR devices: the runs' TMPDIR, the open check's own TMPDIR and the corpus directory are all 2096, with a /proc control (run.log 3258-3265 and 3984-3986). This settles P1's L2.
  - The keep is compared against P1's record, named by its record commit 138fe80 (3877-3931).
  - The open check: 745 pairs, findings 0 (open-check section, line 754).
  - Ends yes (4972).
- **Spec pin.** Every log pins 6c6e87e3: gate.log 23-24, test.log 24-25, negative.log 71/393, run.log 2040, scan-pre-check.log 7/53.
- **Scan.** The pre-check ends RESULT: PASS (scan-pre-check.log 2644).
- **`rm -rf`.** Every occurrence in the printed scripts is either `"${var:?}"` on a directory made by `mktemp -d` in the same run (gate.log 575/579/595, 640/653; test.log 642-643, 703/716; scan-pre-check.log 819-820) or `counting.sh`'s own (gate.log 1856, test.log 2038, negative.log 932, run.log 2002, scan-pre-check.log 2129). P1's verification accepted `counting.sh`'s form.
- **First attempts.** `RERUN_NOTE` and `RERUN_FILES` are unset in every log's header.

### Reservations

1. **The survey prints workbook ids.** Its part lines carry the 12-hex workbook id and a part ordinal beside the names (run.log 2243-2365). Ruling 3 says "names and counts only". The ids are the corpus's hash ids, which the corpus rules allow, but this is broader than names and counts. Planning should confirm.
2. **The `JI` entry is history, not a current statement.** run.log 281 still carries the change entry "JI. No `rm -rf` anywhere", corrected by later entries (373-376, 449) under P1's L1 ruling. Read alone, the 281 entry is stale.
3. **Scan stage.** The final scan stage is not in the inputs, only the pre-check.

## Question 3: does every number, hash and quoted string in report-draft.md appear in a primary?

Verdict: YES with reservations.

These all match a primary:

- The commit hashes and subjects (commit-messages.txt; run.log 33-38).
- The doc hashes 6c6e87e3 and 75e558a2 (commit-messages.txt 95-96; gate.log 1960-1987).
- The log hashes, recomputed: b585f829, 67571109, b5822723, bd0293bc (also scan-pre-check.log 2401-2431).
- 513/540 (gate.log 2626) and 960 (test.log 4855).
- Six plants (test.log 4840).
- The negative cases and the two quoted messages (negative.log 1199-1210).
- 27 of 27, and 843/27/17/0/0/0 (run.log 3297-3305).
- 19 plants (run.log 3572), 95 parts (run.log 2367), device 2096 (run.log 3260, 3984).
- 745 pairs, no finding, 138fe80 (run.log 3866-3880; open-check section line 754).
- RESULT: PASS (scan-pre-check.log 2644).
- The record names (records/).
- 27 and 28 (test.log 4359).
- Rulings 1 to 3 asked "before any log" (working rulings, lines 3-4).
- No W lacks pageMargins or pageSetup (run.log 2413-2419).

These have no primary or need correcting:

1. **Line 3, "is done to the record commit".** No record commit is in the inputs; the records are uncommitted. This is working state.
2. **Line 3, "Nothing is pushed".** Working state.
3. **Line 18, "the final stage runs after the check".** A future step with no primary. The log is also named "scan.log", but the input is scan-pre-check.log.
4. **Line 25, "wrote 28 and 29".** The FACTS file is not an input.
5. **Line 26, the hung `cat` disclosure.** No primary.
6. **Line 24, inaccurate.** It says the created-pageMargins defaults and the created-pageSetup path "are exercised by the crate and tool tests only".
   - The tool tests do not create `<pageMargins>`: the created case already holds one (code-commit-3-tool.diff 434).
   - The negative log's control runs P0, P1C and P3C do create `<pageMargins>` (P0 and P3C also create `<pageSetup>`) through the binary (negative.log 1115-1166, `pageMargins_before=0`). They check only `margin_ok`, not the defaults' bytes.
   - The defaults' bytes are proven only by the crate test (crate diff 1375-1382) and plant Z5.
7. **Line 8, the carry wording.** "/// lines only" matches the gate. P1 ruling 2's text says "the `//` fix"; the report should make clear these are the same thing.
