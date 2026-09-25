# Independent check 2, M2 part 1 P3

Checked commit: 09b42e479f9057b61eae8b2baacc37c24b2ea70d
HEAD at the check: 09b42e479f9057b61eae8b2baacc37c24b2ea70d
Checker: a fresh general-purpose subagent (not a fork), given only the files below in one scratch directory.
Inputs, each with its sha256 (as given to the checker):
- 0016-new-workbook-and-sheets.md: 38f2c1321b4029a9ef54d7a1bd89755eab42d28600d6889a23b98d23c7829b33
- 0001-bootstrap.md: 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
- commit-9ae2efe.diff: b66fa4d78ee0f49abb80bbf883c0505e2fe4095f3f3ff77d0ee31a1c79faf87f
- commit-421bd44.diff: bef676e0959d212fba15fe5a97e3d6da35978a076e1876801f1bb4e91dd776a9
- commit-770c9dd.diff: a9e75d55dc19f559b96106a67b3264b707641d2d6c65f605b25a9482f694b4cb
- commit-a7f7060.diff: edf9da742f2408700710a45519ee93713cfe08dff4b88aea8ed29a3bf2af56bb
- commit-09b42e4.diff: 4aa14e96cb29fdd429c02c504b554324f24149ab866a0305c6dea9cf0ded6f3a
- combined.diff: 10f9cf318749298c1a6f0eee1cab6863ed508eba3bfe8d07240c8badceff2742
- draft-report.md: b651ea6548d249be13beff3b781b21ee239357108857c44ff6b946c0c9fce5a0
- gate.log: df075b0f4f9c5ea49ef542b55f9d9a180a4e3a23437e65fec795c364677c65a3
- test.log: 97f50258314d281bf883e6554d0351a328b5dfcab4db22ccbfccc03bbdc821ff
- negative.log: 68bcb996d006b7759a3f6c0fea55e67af725ade62ebdd4b70e6e3f12c1c64232
- run.log: 9bbc2673ce7ca8f363870581a9c8f80fd531a11a24cbd363657ba826e2518d32
- scan-pre-check.log: 5b4214a27832a949fa795561b5d9701c36b64ce0bb7454334d13bc585e976818
- m2p1-p3-2026-09-25-09b42e4.md: 8bf06ca7ac4685e0344b811908d5cc62d14678808001025eceb159ac9d33707f
- m2p1-p3-2026-09-25-09b42e4.txt: c63e6db807639d50da5cebdf83aaa0ff490b004aacbdbb0304269cea0871297b
- m2p1-open-2026-09-25-09b42e4.md: 0565efe13dbbb6ce364134c2b93c4e27635ea4f48cda12a2d0d772aca9ff001f
- m2p1-open-2026-09-25-09b42e4.txt: cce20c0254f2c67b66b574cc890dbb911e356ec3d47b57d5d29dfef7fbfb2a87
- independent-check-1.md: 6ba035932a60c60def778ca41b1b5056f916406759f258348543dd3a8099dba8
The logs given are attempt 3, the committed ones.
The answer is ASCII and holds no absolute machine path; it is reproduced verbatim below (sha256 of the answer text cc8c861ee248f0732657f9dee415fa7aec42cae4c2a7046de0d01b4dc3172623, 12256 bytes).

---

Note: the harness put a CLAUDE.md into my context. I did not use it as evidence.

I read only the files in <scratch>/p3/check2/. Line numbers are that file's own line numbers; for a diff, they count from the top of that diff file. My sha256 of each input matches the value the draft and the logs give (gate df075b0f, test 97f50258, negative 68bcb996, run 9bbc2673, spec 38f2c132, and the four record files). combined.diff hashes to 10f9cf31..., the value at gate.log 1085. Per file, combined.diff is exactly what the five commit diffs add up to. For ROADMAP, the spec, the .ps1, the two .sh scripts and the tool, the hunks are identical. For package.rs and tests.rs, the only added lines missing from combined are the ones commit 09b42e4 replaced.

## Q1. Does the diff do what the P3 spec says, and nothing else?

Which diff I read for each "nothing else":
- **commit-9ae2efe.diff:** only ROADMAP.md and specs/0016. The blobs match planning's hashes (gate.log 1042-1070).
- **commit-421bd44.diff:**
  - The .ps1 appends the close-error after "; " (61-68, P2 ruling 2).
  - package.rs rewords the two sentences (21-41, P2 ruling 3).
  - Nothing else.
- **commit-a7f7060.diff:** the output's check passes `$output` as its own reference, in edit-open-check.sh 34-38 and passthrough-run.sh 50-54. The input's check still passes the input, as ruling 9 says. Otherwise comments only. Nothing else.
- **commit-770c9dd.diff:** checked point by point below.
- **commit-09b42e4.diff:** checked against ruling 10 below.

Spec points against commit-770c9dd.diff unless another diff is named:

- **The projected list and the record-time checks: yes.**
  - `project` (157-202) applies the edits in record order.
  - `sheets()` returns the projected list and `workbook()` stays the file's list (306-318). Test 1330 pins this.
  - An added sheet's fields are set at 174-185.
  - add, rename and move each check against the projected list and record nothing when refused (347-451).
  - The write replays the edits as its defence (560-566).
- **Name rules and ruling 3: yes.** `check_sheet_name` (220-251) checks, in order:
  - empty;
  - more than 31 `char`s;
  - the seven characters;
  - a leading or trailing apostrophe;
  - a character XML cannot carry, with the existing message (ruling 3; catalogue 40-45);
  - History, ignoring case;
  - a duplicate, ignoring case, skipping the renamed sheet's own position.

  Case folding is `char::to_lowercase` (212-214).
- **add_sheet's recorded order and values: yes (347-403).**
  - Every read happens before the first record.
  - The entry is the blank template worksheet at the first free `sheet<n>.xml`, skipping both existing entries and pending additions.
  - The Override is skipped when a Default for xml carries the worksheet type.
  - The relationship takes the kind of the first worksheet relationship, or transitional when there is none; its target is relative to the workbook part.
  - sheetId is the projected maximum plus one; overflow is refused (ruling 6).
- **Rename escaping (ruling 4): yes.**
  - `attribute_escaped` (1049-1055) turns tab, LF and CR into `&#9;`, `&#10;` and `&#13;`.
  - `with_attribute` replaces the value inside its own quotes (1060-1085).
  - The test now pins `>`, LF and CR (commit-09b42e4.diff 107-130).
- **Moves, definedNames and bookViews:**
  - Both regions are rewritten only when a file sheet is reindexed (630-633, 806-830). An add alone leaves them alone; test 1654-1668 pins this.
  - `activeTab` is remapped. An absent one is appended only when it maps away from 0 (1137-1143).
  - `firstSheet` is untouched by an add or a move, even past the end: commit-09b42e4.diff 80-90 returns early when no position is None. Test 168-187 pins this (rulings 8 and 10).
  - A non-numeric `localSheetId`, `activeTab` or `firstSheet` is refused with the defence message through `index_attribute` (1090-1110; rulings 5 and 10). Tests: 770c9dd 1671-1681 and 09b42e4 190-207.
  - A `localSheetId` past the file's count is copied unchanged (655-661). Test: 09b42e4 147-165.
- **Refusals in the rewrite and ruling 6: yes.**
  - An add is refused for a prefixed root or a root with no relationships prefix (780-789).
  - A non-sheet child or non-whitespace text in `<sheets>` is refused (791-794, 942-948).
  - An add with no `<sheets>` is refused at the write (958-963).
  - Renames and moves in a prefixed root are allowed (test 1753-1787).
  - For the limits clause of ruling 6, see F2.
- **A cell edit on an added sheet, and cells on a pending part (ruling 2): yes.**
  - The edit is applied to the blank bytes through `rewrite_worksheet_from`, and the result replaces the pending addition's slot (469-496, 1187-1199).
  - `validate` accepts the added part (533-537).
  - `cells` on a pending part is refused with "no such part" (commit-09b42e4.diff 216-226, docs 60-72).
- **Catalogue entries and limits: yes, apart from F2.**
  - The catalogue covers the 12 P3 messages, the ruling 3 reuse and WORKBOOK_REPLACED (40-80). commit-09b42e4.diff 24-46 adds the pending-part test and the book-view refusal.
  - The limits are at 89-112 and commit-09b42e4.diff 48-58.
  - The two remove messages belong to P4.
- **--sheet-sample (rulings 1 and 7): yes.**
  - The name is the first free of emend, emend2 and so on, compared by fold (1998-2013).
  - It records the add, (1,1)=7 and the move to 0, then writes, reopens and asserts (2014-2080).
  - The counts come from the tool's own token pass (2087-2168).
  - The line is pinned by the test at 2207-2248.
  - For a gap in the assertion, see F7.
- **Script change against ruling 9: matches** (commit-a7f7060.diff above).
- **Code commit 4 against ruling 10: F1, F2, F3 and F5 are each done as ruled, and nothing else.**
  - F1: the out-of-range name is removed from the move-remap test (132-138), and a new past-count test is added (147-165).
  - F2: code at 94-100, catalogue 34-46, limits 48-58, test 190-207.
  - F3: 76-90 and 168-187.
  - F5: 107-130 and 216-226, and the `cells` doc at 60-72.
  - gate.log 1175-1179 confirms the commit touches only package.rs and tests.rs.
- **Tests "P3" lists: all present except one.**
  - Present, in commit-770c9dd.diff: 1312, 1380, 1397, 1419, 1458, 1488 (six refusals, History in another case, duplicate in another case), 1526, 1539 plus 09b42e4, 1566, 1595, 1616, 1624, 1715, 1753, 1732, 1807, 1819, 1312 (fields), 2208 (the tool's line).
  - Missing: "add then remove of the pending sheet". Ruling 10 (F4) moves it to P4 with `remove_sheet`, so it is not a finding here.

## Q2. Do the logs contain every line the spec requires?

- **Corpus run on both targets, recorded: yes.**
  - Host: run.log 1704-1803, with lo and lo_in ok on 27 at 1737.
  - i686: 1805-1904, same result at 1838.
  - Record pair: written, checked by sixteen checks (2066) and installed (2067-2073).
- **Comparison per "Harness runs" P3 and ruling 8: yes.**
  - sheetcmp.py is printed at run.log 1424-1695.
  - Its flags cover everything the spec lists: the untouched entries, the one added entry and its bytes (ruling 1), the regions, sheets first, localSheetId +1, activeTab +1 with firstSheet unchanged, rels +1, and the content types +1 or covered.
  - All flags hold on 27 of 27 on both labels (1797-1802, 1898-1903).
  - One planted control per flag, 10 of 10 (1906-1918).
  - The targets' outputs are identical (1920-1923).
  - It uses expat rather than ElementTree; ruling 10 recorded that as no action.
- **Negative log through the binary: yes.**
  - Worktree, the binary copied from the gate, and its hash checked against gate.log: negative.log 692-699.
  - S0 control and S1-S4: 746-785.
  - The statement about the name refusals: 805-806.
  - See F5 for one route not exercised.
- **Ruling 9's two cases: yes.** negative.log 809-841: O1 is accepted, O2 is refused, and the base script is shown refusing the grown output. See F6.
- **Edit open check over the keep, and ruling 10's two pairs: yes.**
  - Totals: run.log 2431.
  - The move-remap pair is ok:ok:none:none and the past-count pair is fail:fail:refused:refused, each checked with a control (2466-2473).
  - P2's 155 pairs are unchanged (2573-2577).
- **Rerun headers name attempts 1 and 2 by printed hashes: yes.**
  - gate.log 52-57, test.log 45-50, negative.log 36-41, run.log 27-40. run.log also hashes attempt 2's records.
  - The superseded run directory is hashed through its two summaries, removed and printed (run.log 41-44).
  - The pre-check scan has no rerun header; see F1.
- **Tree-status checks print the lines: yes.** gate.log 1376-1387, test.log 2182-2188, negative.log 867-876, run.log 2638-2650.
- **Forbidden-shape rules: yes.**
  - Each log greps its own script before logging (lines 5-15 of each).
  - The scan covers every printed script (scan-pre-check.log 1464-1497).
  - My own count over the five logs, the draft and check 1's answer: 0 chained, 0 leading-bang, 0 assignment, 0 non-ASCII, 0 control bytes.

## Q3. Does every number, hash and quoted string in draft-report.md appear in a primary?

Everything matches a primary except the items below.
- **Line 41:** the check-1 answer's hash 6ba03593... is not printed in any primary. The pre-check scan read 0 answers (scan-pre-check.log 1364-1365). My own hash of the file agrees.
- **Line 44:** "passed" for attempt 1's gate, test and negative has no primary. The cause of attempt 1's run failure is stated in spec ruling 9, which is not a listed primary.
- **Line 45:** "each passed" for attempt 2 appears only as authored header text (gate.log 52) and in spec ruling 10.
- **Line 26:** "as in attempt 2" cannot be checked. independent-check-1.md 96-98 names families S and O, not P.
- **Lines 53, 56 and 58:** "27 of 27 exit 0", "0 errors" and "each was clean at the base" come from unlogged runs. They are disclosed as such.
- **Line 60:** the first pre-check scan's "RESULT: FAIL" on "two structural checks" has no primary; see F1.

## Findings

- **F1 (gap): the failed first pre-check scan has no hash anywhere.**
  - It is mentioned only in draft-report.md 60.
  - specs/0001 83-87 and 185-191 require a superseded attempt to be hashed and kept, or named as having no hash and why.
  - scan-pre-check.log 1-60 prints no rerun line and hashes nothing.
  - The report does not say whether that scan was kept.
- **F2 (nit): the limits list does not carry ruling 6.** Ruling 6 says "the limits say so" about the sheetId overflow and the missing `<sheets>`. Only the catalogue names them (commit-770c9dd.diff 69-76; commit-09b42e4.diff 34-46). The limits list (commit-770c9dd.diff 90-112; commit-09b42e4.diff 48-58) does not, and nothing states that neither case is reachable from the corpus.
- **F3 (nit): stale text in run.log.** run.log 1407 (script line 296) says planning's spec holds "P3 rulings 1 to 9". The file it hash-matches, 38f2c132, holds rulings 1 to 10.
- **F4 (nit): the open record's heading does not cover the past-count pair.** m2p1-open .md 21 lists the allowed reasons for a refused input, and none fits the past-count fixture listed at line 30. Ruling 10 allows it, and run.log 2466-2471 checks it explicitly.
- **F5 (nit): the new book-view refusal is not exercised through the binary.** --sheet-sample can reach the ruling 10 F2 route (a non-numeric `activeTab`). The negative log has no case for it. Its message is still covered by S4 (negative.log 776-785).
- **F6 (nit): the O2 control relies on a counting quirk.** The mismatch is built from a foreign-namespace sheet element in extLst, which the prefix-tolerant count counts as a sheet (negative.log 823-824). That shows the count is not namespace-aware. lo-check.sh is not changed in this phase.
- **F7 (nit): the tool's active-tab assertion assumes a workbook view.** It requires `active_after = active_before + 1` (commit-770c9dd.diff 2076-2078). A workbook with no bookViews would exit 2, since nothing is appended there. This is the spec's own expectation, and every corpus row shows views=1.
- **F8 (nit):** the Q3 items above.

Verdict: the diff matches the P3 spec and rulings 1 to 10, code commit 4 does exactly ruling 10's F1, F2, F3 and F5, and the logs hold every required line; P3 can be accepted once the report deals with F1 (the unhashed failed pre-check scan) and notes F2 to F8.
