# M3 part 3 P3, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec specs/0025, planning's working ruling section of it for P3, the committed specs/0024, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, each one's stderr, the phase's four records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p3-p3. Checked commit (the phase commit): 5c1a6b37695a56fe031c11db24663a26a7bc6210; HEAD when the check ran: 5c1a6b37695a56fe031c11db24663a26a7bc6210. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 89da0ca5c52fb2dd05f3aa39dbec6f13fc2dbfe00ca0b978cea4ce6d88c75562. Its items on the report are corrected in the report; its observations on the code and the logs are reported to planning. No second check was run.

Inputs and their sha256:

- b879a90ce653c5198b501b14a01180ce370e271e84f5bb68dd6e2c453126e99e code-commit-1-carry.diff
- af04908a20cd7dbd26edbc290fb5c3906a044ede291a53c9d3298b0e276f3848 code-commit-2-crate.diff
- 05519eaf73f7ac9569c8f0d6be115f54c832b0ecb74982be452f8fc7cce80edd code-commit-3-tool.diff
- 6d20fbb303233c3121e13988e6e94ef72da5874ecd1a7f6a7f401b1b1c43c89a commit-messages.txt
- 68de1069ddc6f714eebf4bb0d68b64f99fbb94359e26dda3cdf304941b4c4d88 doc-commit.diff
- b8e9827dba73d8b736a5b3947fc1c3151349756d0012cf38271c61211369ae95 gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- e46a8b6cf4961c49fd2ce3ad2732587e11e850ed832a24c8a96d206c7ef39ed1 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- 5679ade23d03232a22e9d2a3f17d3afdbdd99a8dc9901a1eafd2abb7359b8ca9 phase.diff
- 942a1e95988c7422b689927ad2a4f84f3d8a293dd306104fc0187a138b454794 records/m3p3-open-2026-10-03-5c1a6b3.md
- ed3b9da0b8564fb8f20a4b5dae46687c14aa8b1938bb64d1a46e6793b56095d3 records/m3p3-open-2026-10-03-5c1a6b3.txt
- 05b60d2b78c7ee408a8a0af9539475cc7e24a3d270eb15305cdd9313abb17746 records/m3p3-p3-2026-10-03-5c1a6b3.md
- 61a848d62e8e2509a0ea7f20881663aeafd5cb8a87e4bc7a4dc1f98c9a9d5b9e records/m3p3-p3-2026-10-03-5c1a6b3.txt
- 63a3535b30c7e273680dbb98308843453582c426fb2f25bfeabc29ef5552f929 report-draft.md
- ae2553531308d25fd2573ccb710024af62eab1889030938f5fefd90873902a0e run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- ac5cdc5b10b057ce9f219850de44b4c1f1cc9dcbc3826d34581f1db0426e4119 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- ae1ab201b5c7071c36235f2c5d8c253c6badfab95a9f163575ef7daa0e78863a spec-0024-committed.md
- 338f0e8ba7d696d0a955cc842256638998827d0370c06680050e0cbc71305bb7 spec-0025-committed.md
- 151bbec0df369d28aa164b6741c8bc4c09f5d3d0af52c7830ae126a065db5b9a spec-0025-working-rulings-p3.md
- dee9b8a3f7531b494120def17b2c2ff57eeb3a1c689be79db02e7e44fa7b47e7 test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

---

# Independent check 1: M3 part 3 P3

I checked the inputs first. `sha256sum -c ../inputs.sha256` reports OK for every file and 0 failures. I read only files under `<home>/.cache/emend-build/m3p3-p3/check1/inputs`. phase.diff holds the same 1212 added and removed lines as the three code-commit diffs put together. No input diff, commit message or report-draft.md contains a non-ASCII byte.

## 1. Does the diff do what the spec, with the working rulings, says for P3 and nothing else?

**Verdict: YES with reservations.**

What the diff does:

- **The carry is test code only.**
  - code-commit-1-carry.diff lines 9-69 add one test to `images.rs` under `cfg(all(sheetprops, metadata))`. It creates a `<hyperlinks>` and a `<drawing>` in the same write and expects the four page elements to land between them (line 64), both at the root's end and before a later root child (line 21).
  - gate.log lines 2175 and 2179 confirm that the commit modifies only `images.rs`, adds one test and removes no line.
- **The three calls sit under `sheetprops` and `metadata` together.**
  - code-commit-2-crate.diff lines 271-272, 289-290 and 315-316 carry the cfg on the three calls.
  - gate.log lines 2497-2514 show that exactly these three calls are added under the pair's cfg, and that the list without them is the base's, line for line.
- **The built-in check is lifted only for the three names.**
  - The three names get their own `NameEdit::Builtin` kind (crate diff lines 387-155 region, from line 387 onward; the enum and its text builder are at lines 39-155). `add_defined_name` is not touched.
  - The test at crate diff lines 1191-1206 shows that `_xlnm.Print_Area`, `_xlnm._FilterDatabase` and `_xlnm.Criteria` are still refused with "a defined name is not valid".
- **The file's name for the sheet is replaced, and the empty and `None` forms drop it.**
  - name_plan at crate diff lines 525-571 removes the file's name of that name and scope, then appends the new one.
  - An empty slice drops the print area (line 337). Both `None` drop the titles (line 360). `None` drops the filter (line 381).
  - The tests are at lines 873-915 and 917-950, plus `None` in the autofilter test at 1038-1074.
- **Only `_FilterDatabase` is hidden.** See crate diff line 566 and `name_elements` at lines 594-633.
- **The text form follows rulings 3 and 6.**
  - `sheet_in_formula` (lines 137-155) writes the name bare for a letter or underscore start, then letters, digits, `_` or `.`, and not a cell name; otherwise quoted with an apostrophe doubled.
  - `absolute` always writes two corners. Columns come before rows (lines 109-121).
  - The text uses the projected name, so a rename in the same write is followed (lines 556-560).
  - The tests are at lines 917-950 and 952-979, the second covering eight names including a rename.
- **The formula-change exemption follows ruling 2.**
  - `names_under_rule` (lines 182-200) returns false only for `Builtin`, and it replaces `no_name_edits` at lines 403 and 439.
  - The workbook part is still rewritten through `naming` (lines 406-422).
  - The test at lines 1121-1142 checks the chain and `[Content_Types].xml` are untouched, with a control in which `add_defined_name` drops the chain. So every other name edit stays under the rule.
- **The part checks follow ruling 4.**
  - `builtin_parts` joins the edited-part check (lines 423-432).
  - The tests at lines 1178-1188 cover "no such part" and "an edit to a removed part".
- **The autofilter is in schema position, its children dropped and other attributes kept.**
  - `AutoFilter` is added to `SETTINGS` (lines 705-715). Its children are cleared at line 769, and creation goes through `create_before` (lines 634-677).
  - The tests are at lines 1007-1036 (after `<sheetData>`, before `<mergeCells>`) and 1038-1074 (`x="1"` kept, children dropped).
- **The overlap check follows ruling 1.**
  - `TABLE_OVERLAP` carries specs/0021's exact text, with its cfg widened (lines 572-580). `TABLE` (lines 14-26) and `part_ref` (lines 775-787) are widened too, one definition each.
  - `filters_clear_of_tables` (lines 455-500) refuses a missing part, an unparsable or absent `ref`, or an overlapping table. With `tables` on, it also checks the tables the same write adds (lines 475-484).
  - The test at lines 1076-1119 covers an overlap, a bad `ref` and an empty `ref`, with a control.
- **The refusals follow ruling 5.**
  - `PRINT_INVALID` is raised for a title pair out of order, or with a 0 or off the grid (lines 354-358).
  - The index message is raised for an area off the grid (lines 331-336 and 373-374). Corners are normalised (lines 330 and 372).
  - The test is at lines 1144-1177.
- **localSheetId is remapped after a sheet removal.** The test at lines 981-1005 removes S1 and expects all three names at localSheetId 1.
- **The tool sample matches "Harness runs".**
  - code-commit-3-tool.diff lines 121-223 make the three calls on W with A1:(M+1, 8) and rows (1, 1).
  - The line's fields match spec lines 401-408. It asserts the name count arithmetic, each text bare or quoted, `hidden_ok` and `autofilter_after`=1, and removes the output on a failure.
  - The two tool tests are at lines 253-354.

Reservations:

- **R1. A missing table part and a table added in the same write are never exercised.** The code is there (crate diff lines 475-484 and 496), but no test reaches either refusal. Ruling 1 does not demand a test.
- **R2. No test pins the shift or replaced-part refusal for the three calls.** spec-0025 "The calls" (lines 214-216) gives both refusals for every call. The diff only shows the built-in parts joining the "no such part" loop (crate diff lines 425-432). The rest of that loop is not in the diff, so from the inputs I cannot confirm that `set_print_area` and `set_print_titles` reach the shift and replaced-part checks. Ruling 4 names only the absent-part and removed-part cases.
- **R3. The `add_defined_name` refusal test covers three of the `_xlnm.` names.** It does not cover `Print_Titles`, `Extract`, `Database` or `Sheet_Title` (crate diff lines 1193-1206). That code is unchanged, so this is coverage only.
- **R4. The tool matches names before and after differently.** `had` compares without regard to case (tool diff line 140), while the read-back compares exactly (line 209). Harmless, since the crate writes the canonical name.

Nothing outside P3 is in the diff:

- gate.log lines 2154-2186: each commit touches only its own area, and there is no manifest, lock or styles change.
- gate.log lines 2557-2575: no base test is edited or removed.

## 2. Do the logs contain every line the spec and binding rulings require?

**Verdict: YES with reservations.**

**gate.log**

- The spec is pinned at 338f0e8b (lines 21-27).
- Crate tests on both targets: 513 with the features off, 540 with sheetprops and 599 with the pair (lines 2667-2696).
- The `pub fn` and `pub const` lists with and without the pair's items (lines 2330-2548), and `TABLE` alone under the widened cfg (lines 2517-2519).
- clippy and doc with the features off, sheetprops alone, the pair and every feature.
- check.sh shows nine PASS lines.
- The log ends "all as expected: yes".

**test.log**

- The spec is pinned (lines 22-28).
- The workspace runs 973 on host and on i686 (lines 4993-4995).
- The crate runs with the features off, sheetprops and the pair on both targets print "failed tests: none" (lines 4409-4480).
- The `EMEND_KEEP_DIR` run passes 796 against 796 (lines 4481-4496).
- The 28 against 27 module count is explained by a printed line (lines 4498-4510; P2 ruling 6).
- Six plants each fail at their own assertion line: 165, 122, 218, 323, 240 and 305 (lines 4868-4990). Each prints the failing test's name.

**negative.log**

- The spec is pinned (lines 71-72).
- The T1 table over A1:H2 is refused with the exact overlap message, with T1C as its control (lines 1300-1360).
- A2 and F3 are refused, each with a control.
- `PRINT_INVALID` is stated unreachable, with its covering test checked (lines 1376-1394).

**run.log**

- The spec is pinned (around lines 2100-2104).
- Both labels run 27 files: 21 ok and 6 refused (lines 3175-3240 and 3288-3325).
- The six refusals are derived independently from the survey's overlap prediction (lines 2382-2388 of the survey section). They match on both labels.
- The ok rows' entry tables are checked by `entries_table`: W and the workbook changed, every other entry identical (lines 1033-1055). This covers the calc chain being kept. The record shows calcChain 6 identical and 0 dropped (record .md line 33).
- The comparison has 15 plants, each turning its flag on the created case f56906d42e0a and the kept case d8245b84086f (lines 3406-3472).
- The survey is at lines 2117-2388.
- The devices are printed: the runs' TMPDIR, the corpus directory and the open check's TMPDIR are all 2096, with /proc as the control (lines 3166-3173 and 3858-3861).
- The keep and the open check name P2's record commit 25f9bba (lines 31-41 and 3762-3770).

**`rm -rf`**

- Every occurrence is either the guarded `"${var:?}"` form on a directory the script made with mktemp, or `counting.sh`'s own standing line.
  - gate.log: lines 627, 631, 647, 692 and 705.
  - test.log: lines 696-697, 757 and 770.
- scan-pre-check.log line 2626 counts 0 unruled, and line 2664 reads RESULT: PASS.

Reservations:

- **L1. No line names the calc chain.** The calc chain being kept is shown only through `entries_table`'s rule that every other row is identical, plus the record's family table. No run.log line counts the six chain workbooks by name.
- **L2. The survey prints each workbook's M (its largest row number) beside its id.** These are run.log survey lines around 2340-2360. They are counts, not cell content, but "Harness runs" (spec line 368) says no reference is printed. Planning should say whether a per-workbook row count is allowed.
- **L3. The run log's change list still carries JI's "No `rm -rf` anywhere"** (run.log line 287). It is corrected by later entries (lines 379-382, 455 and 527), the history form accepted at P1 and P2.

## 3. Does every number, hash and quoted string in report-draft.md appear in a primary?

**Verdict: YES with reservations.**

These all appear in a primary:

- The commits and the base subject: commit-messages.txt, and run.log line 39.
- The spec and ROADMAP hashes: commit-messages.txt lines 87-88.
- The four log hashes: sha256 of the inputs.
- 513, 540 and 599: gate.log line 2696.
- 973 and the six plants: test.log.
- The negative cases: negative.log.
- 27, 21 and 6, the six ids, and the totals 623, 42, 30, 0, 0 and 0: run.log lines 3210-3225.
- 21 of 21, the 15 plants, 2096 and 774 pairs.
- "no finding": open record .md lines 39-45.
- 25f9bba and the record names.
- The message "a print area or title is not valid".
- `&apos;`: print.rs `local()`, and the W2 plant output.
- `TABLE` listed by the gate: gate.log line 2519.
- Rulings 1 to 6 coming after the carry commit: the working rulings, lines 3-4.

Items without a primary, or that a primary contradicts:

- **Q3-1 (report line 24, "The name writer is specs/0019's, unchanged").** The diff changes `name_elements` and `NamePlan` to carry and write `hidden="1"` (crate diff lines 594-633). The escaping line is unchanged, but "unchanged" overstates it.
- **Q3-2 (report line 25, "Area's own text gives a single cell for one").** No input shows what `Area`'s text gives for a one-cell area. Ruling 6 (working rulings lines 53-54) reads "a one-cell area `$A$1:$A$1`, as `Area`'s text", which on its plain reading says the opposite. The code's own corners (crate diff lines 128-133) and the test (line 956) do give `$A$1:$A$1`, so the behaviour matches the ruling. Only the report's explanation is unsupported.
- **Q3-3 (working state, no primary).** These have no primary in the inputs: "nothing pushed" (line 3); "each at its first attempt" (line 12), which rests on RERUN_NOTE being unset and on no superseded run being named; the final scan stage being run after the check (line 18); and disclosure line 27 (the hand re-run of the run script, the fork's dry run, the plant description corrected in text).
