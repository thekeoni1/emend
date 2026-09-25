# Independent check 1, M2 part 1 P3

Checked commit: a7f7060d2e1b99625658d496d4be1afaf15b5bdf
HEAD at the check: a7f7060d2e1b99625658d496d4be1afaf15b5bdf
Checker: a fresh general-purpose subagent (not a fork), given only the files below in one scratch directory.
Inputs, each with its sha256 (as given to the checker):
- 0016-new-workbook-and-sheets.md: 25244842d7e440fb91a5f73a1636e7905bbcf83f249abb6ff303169d78fd0dfb
- 0001-bootstrap.md: 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
- commit-9ae2efe.diff: b66fa4d78ee0f49abb80bbf883c0505e2fe4095f3f3ff77d0ee31a1c79faf87f
- commit-421bd44.diff: bef676e0959d212fba15fe5a97e3d6da35978a076e1876801f1bb4e91dd776a9
- commit-770c9dd.diff: a9e75d55dc19f559b96106a67b3264b707641d2d6c65f605b25a9482f694b4cb
- commit-a7f7060.diff: edf9da742f2408700710a45519ee93713cfe08dff4b88aea8ed29a3bf2af56bb
- combined.diff: 6e619a03f2356cd0bb050b87be4c9f01aaa45ee9bd2b81010097b4e556aa6306
- draft-report.md: 336c204897df2e1031f4593ecb8dc753962dafb6e6ab50c64851f73396f74bf3
- gate.log: 789bd86bcb09fb2aca0681ce98bb536b5240db669a79fd6b1f33726277c69ee8
- test.log: 1d60fea80a554dd4a00fd522ce7016d6cadbd899f3d04511067ff50512e1524e
- negative.log: 684913aa9ab438e80978cac480924b7f15630b81850cf0ac2dd9fc74eeb79ad2
- run.log: 44f3094b565e3aa8a97605ae10eceb06f76dacca2b037ab207b4d41542c5595b
- scan-pre-check.log: 151b4182c356d7a24e4c0bb9f7251b333409ff4ecde40ed2f684a1a479ce2d4f
- m2p1-p3-2026-09-25-a7f7060.md: 1248c106c28aeb702eddf761301caae266a2609e14d3791b48c0e2f0a5278567
- m2p1-p3-2026-09-25-a7f7060.txt: b764e05d5ff23c1adfb93bba7139a61545e6385de68dd5c6a9b64863fe4e9c0b
- m2p1-open-2026-09-25-a7f7060.md: f1d29b0a1ce4d39db8603df9644eca59e60121a1143b1134f920ceca8081cf53
- m2p1-open-2026-09-25-a7f7060.txt: 2c7de4cd674f13220e86a22c7c827c0db1307d57f4e1cb38430132619276d7b5
The four logs given were attempt 2, at a7f7060. The committed logs are attempt 3, at code commit 4, rerun under P3 ruling 10 for this check's F1, F2, F3 and F5; the attempt 2 logs and records are hashed in the attempt 3 rerun headers.
The answer is ASCII and holds no absolute machine path; it is reproduced verbatim below (sha256 of the answer text 71cca04efb410572bffac28fbf7434fc9dcb638eacaa833f74e81855697801e3, 9501 bytes).

---

Note: the harness put a CLAUDE.md into my context. I did not use it as evidence.

Sources: every file named below is in the check directory. Line numbers are file line numbers; for a diff, they count from the top of that diff file.

## Q1. Does the diff do what the P3 spec says, and nothing else?

**Diffs read for each "nothing else":**
- **Doc commit 9ae2efe (commit-9ae2efe.diff):** only ROADMAP.md and specs/0016. Its two hunks match combined.diff lines 1-15 and 1964-2067 byte for byte; I compared them. gate.log 1021-1045 also checks both files against planning's hashes.
- **Code commit 421bd44 (commit-421bd44.diff):**
  - The .ps1 appends close-error after "; " (lines 61-68, P2 ruling 2).
  - package.rs rewords two comments (lines 21-41, P2 ruling 3).
  - Nothing else.
- **Code commit a7f7060 (commit-a7f7060.diff):** passthrough-run.sh (lines 50-54) and edit-open-check.sh (lines 34-38) now pass `$output` as the output's reference workbook. The input's check is unchanged. This matches ruling 9 and nothing else changed. The one cosmetic issue is F8.
- **Code commit 770c9dd (commit-770c9dd.diff):** checked point by point below. The extras I found are F2, F3 and F9.

**Spec points against commit-770c9dd.diff:**

- **The projected list and the record-time checks: yes.**
  - `project` (157-202) applies the edits in record order.
  - `sheets()` returns the projected list and `workbook()` still returns the file's (306-318).
  - add, rename and move each check against the projected list when called and record nothing on a refusal (347-451).
  - `validate` replays the edits as the write's defence (560-566).
- **The name rules and ruling 3: yes.** `check_sheet_name` (220-251) checks, in order: empty, more than 31 chars, the seven characters, a leading or trailing apostrophe, a character XML cannot carry (ruling 3, the existing message), History ignoring case, and a duplicate ignoring case. Case is folded with `to_lowercase`. A rename skips its own position.
- **add_sheet's recorded order and values: yes (347-403).**
  - The first free sheet<n>.xml, skipping existing entries and pending additions.
  - The Override is skipped when a Default for xml carries the worksheet type.
  - The relationship kind follows the first worksheet relationship, or is transitional when there is none. Its target is relative to the workbook part.
  - sheetId is the projected maximum plus one; overflow is refused (ruling 6).
- **Rename escaping (ruling 4): yes.** `attribute_escaped` (1049-1055) uses the quick-xml escape plus `&#9;`, `&#10;` and `&#13;`. `with_attribute` replaces the value inside its own quotes (1060-1085).
- **Moves, definedNames and bookViews:**
  - The regions are rewritten only when a file sheet is reindexed (630-633, 807-828).
  - activeTab is remapped; an absent one is appended when it maps away from 0 (1137-1143).
  - firstSheet is left alone by an add or a move (666-674, 1144-1149), apart from the clamp in F3.
  - A non-numeric localSheetId is refused with the defence message (1090-1110, ruling 5).
  - A localSheetId past the file's count is copied unchanged (655-661).
- **Refusals in the rewrite and ruling 6: yes.**
  - Prefixed root plus an add: refused. Root with no relationships prefix plus an add: refused (780-789).
  - A non-sheet child in `<sheets>`: refused (791-794, 942-948).
  - An add with no `<sheets>`: refused at the write (958-963).
  - Renames and moves in a prefixed root are allowed.
- **A cell edit on an added sheet (ruling 2): yes.** It is applied to the blank bytes through `rewrite_worksheet_from`, and the result replaces the pending addition (469-496, 1187-1199). `validate` accepts the added part (533-537).
- **Catalogue entries: yes.** 51-80 and 89-112 list the 14 messages with tests, plus the reuse of the ruling 3 message.
- **--sheet-sample (rulings 1 and 7): yes (1993-2168).**
  - The name is the first free of emend, emend2 and so on, compared by case fold.
  - It sets (1,1)=7, moves the sheet to 0, writes, reopens and asserts.
  - It counts with its own pass over the part's tokens.
  - The line format is pinned by the tool test (2207-2248).
- **The script change against ruling 9:** matches, as above.
- **Tests P3 lists:**
  - Present: tests at commit-770c9dd.diff 1312-1891.
  - Missing:
    - "add then remove of the pending sheet". The draft discloses this as moved to P4 (F4).
    - The ruling 4 test does not pin `>`, line feed or carriage return (F5).

## Q2. Do the logs contain every line the spec requires?

- **Corpus run on both targets: yes.**
  - host: run.log 1680-1779; i686: 1781-1880.
  - Record pair installed: 2046-2049.
  - Sixteen record checks: 2025-2042.
- **Comparison per "Harness runs" P3 and ruling 8: yes.**
  - sheetcmp.py is printed at run.log 1399-1673, with firstSheet expected unchanged (1425-1427, 1633-1642).
  - Planted controls: one per flag, 10 of 10 (1882-1894).
  - Targets identical: 1896-1899.
  - Deviation from the letter of ruling 1 ("with ElementTree"): expat is used instead (F7).
- **Negative log through the binary: yes.**
  - Worktree, the binary copied from the gate, and its hash checked against gate.log: negative.log 686-693.
  - S1 to S4: 752-782.
  - The name-refusal statement: 799-800.
- **Ruling 9's two negative cases: yes.** negative.log 803-834, O1 and O2, with the base as the control.
- **Edit open check over the phase's keep: run, but see F1.** run.log 2222-2591.
- **Rerun headers name attempt 1 by printed hashes: yes.**
  - gate.log 49-52, test.log header, negative.log 34-37, run.log header.
  - The superseded run directory is printed and removed (run.log header), with only its two summary files hashed (F6).
- **Tree-status checks print the lines: yes.** gate.log 1319-1326, test.log 2146-2152, negative.log 865-869, run.log 2597-2607.
- **Forbidden-shape rules: yes.**
  - Each log greps its own script before logging: gate.log 5-15, and the same block in the others.
  - The scan covers every sibling's printed script: scan-pre-check.log 1449-1482.
  - My own count over the five logs is 0 chained, 0 leading-bang, 0 assignment and 0 non-ASCII.

## Q3. Does every number, hash and quoted string in draft-report.md appear in a primary?

Everything else matches a primary. The exceptions:

1. **Line 44, "each passed"** (attempt 1 of gate, test and negative): no primary shows those results. The headers only hash them.
2. **Line 45, "Every other check held" and "9 findings":** these appear only as authored header text (gate.log 49). The attempt-1 logs are not among the primaries.
3. **Lines 51, 54 and 55:** "27 of 27 exit 0", "0 errors" and "each was clean at the base" come from unlogged runs. No primary holds them. They are disclosed as unlogged.
4. **Line 57:** the quoted "the tool's own quick-xml pass" is not in the spec. Ruling 7 reads "with its own quick-xml pass over the workbook part".

## Findings

- **F1 (gap): the key P3 pair has no LibreOffice evidence, and the report is silent on it.**
  - The pair from the move-remap test (move with localSheetId, activeTab and firstSheet) failed on both sides: lo_in fail and lo fail, class refused and refused (run.log 2545, 2577; m2p1-open .txt 67; .md 120).
  - The record lists it under "Inputs refused by LibreOffice (allowed: ... zip-level, or ... no relationships part or an empty content-types root)" (.md 21, 30). None of those reasons applies to this fixture.
  - The run log checks the count class only (2567-2568), so it never tests whether a refused input is one of the allowed kinds.
  - As a result, the remapped definedNames and bookViews output is never shown to open. The draft (line 37) gives totals only.
- **F2 (gap): an extra, uncatalogued refusal.** A non-numeric activeTab or firstSheet is refused with the defence message (commit-770c9dd.diff 1139, 1144, via 1100-1106). Ruling 5 covers localSheetId only. The catalogue (69-76) and the limits do not mention it, and no test covers it.
- **F3 (nit): the firstSheet clamp runs on a move too.** `first_sheet` clamps a present value to count-1 even when nothing was removed (commit-770c9dd.diff 666-674). Ruling 8 says a move or an add leaves it as it is. This only matters for a file whose firstSheet is already past its sheets.
- **F4 (gap, disclosed):** the "add then remove of the pending sheet" test is deferred to P4 (draft line 56).
- **F5 (nit):** the rename test (commit-770c9dd.diff 1539-1563) pins `&`, `<`, the quote, the apostrophe and tab, but not `>`, line feed or carriage return, which ruling 4 lists. Separately, no test covers `cells` on a pending part being refused (ruling 2).
- **F6 (nit):** for the superseded run directory, the log hashes only the two summary TSVs plus a file count, not every file (run.log header).
- **F7 (nit):** sheetcmp.py uses expat and regular expressions, not ElementTree as ruling 1 words it (run.log 1441, 1575). It also has dead code at 1512 (`if False else`).
- **F8 (nit):** a header comment line in edit-open-check.sh is longer than the file's wrap width (commit-a7f7060.diff 26).
- **F9 (nit):** the sheet-number loop's overflow reuses the defence message and is uncatalogued (commit-770c9dd.diff 375). It cannot be reached.
- **F10 (nit):** the Q3 items 1 to 4 above.

Verdict: the code matches the P3 spec and rulings 1 to 9, apart from one extra refusal (F2) and one test deferred to P4 (F4), and the logs hold every required line. P3 can be accepted once the report addresses F1, since the move-remap pair was never shown to open in LibreOffice.
