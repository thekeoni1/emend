# Independent check 2: M1 part 6 P2

Written by the implementing session. The header is mine; everything after
the rule is the checker's answer as it was returned, followed by one line
feed this file adds.

The checker was a fresh subagent (not a fork) with no access to this
conversation. It was given the eighteen files below and nothing else (it
disclosed read-only git, sha256 and ls commands over the repository, the
scratch directory and the quick-xml source), and asked the three questions
CLAUDE.md names plus whether each of check 1's findings F1 to F9 is
resolved. It was told to write <repo>, <check>, <home>, <corpus> and
<scratch> in place of absolute paths and to answer in ASCII.

Commit checked: 71e643b949e42399cc5a87d195934aba490beba0
("M1 part 6 P2: a shared string before the first number in the A-only sample test"),
which was HEAD when the check ran and when this file was written. Range
read: 6d85828..71e643b. Each per-commit diff's producing command is named
beside it below (P1 verification ruling 2).

This is the second and last check the spec allows. No log was rerun after
it; its findings G1 to G10 are answered in the report.

The inputs, each as the checker received it (sha256, then the name it had in
the check directory and what it is):

- 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
  0001-bootstrap.md (specs/0001-bootstrap.md at 71e643b)
- 5a1b838df4e8e77f8c716c83fb4dd11f936e870cfccbc0a2326cba009e41e539
  0013-formulas.md (specs/0013-formulas.md from the working tree, with planning's uncommitted "Planning rulings during P2")
- efe044f66354bf7fcfbf9aa2c9a2476a47854d9e865afffb99917bdaefa2648c
  diff-3c932d9.patch (made by: git show --format=fuller 3c932d9)
- 99c88a0d352d85a511e27bb181954177ff32b485c596812d9cb17da7038300c0
  diff-463ccad.patch (made by: git show --format=fuller 463ccad)
- d3ebf11b12f959439bb333b7bcd4678e2c77de927c2b5bd3c5a0fbdff216afec
  diff-71e643b.patch (made by: git show --format=fuller 71e643b)
- e0fd77f577ab4dc112bf01f3252c8d0858f201da25a7f4d78002d5391aa64e01
  diff-code-combined.patch (made by: git diff 463ccad 71e643b)
- e17fe4e4cc74eb7b2678580785c620f0597039c6dd27990d99d6fbb7d9c8dcc2
  diff-commands.txt (the command that made each diff)
- 9a5085eedd0659992f92e6c7fd564ad816da6d55fe55fdb31b5722919a01a821
  diff-f1a5aba.patch (made by: git show --format=fuller f1a5aba)
- 37f8c0feb57c53f9d2b63a7a6ffc87b6fe33bd905efd62468f6eae07f890078b
  diff-phase.patch (made by: git diff 6d85828 71e643b)
- b44d0558e1299a1ef2391fd639484a26b99cc30c052a3677be9a4bfe0a568251
  gate.log (records/.../gate.log as installed and committed)
- cffba688560959a79376718a70b2269bc74bc473b60fb6d7502dad9d09b81180
  independent-check-1.md (records/logs/m1p6-p2/independent-check-1.md as installed)
- 6401e77f2303f4e81df21ca06ca6e36e55107027ced161f1b113a1597d22275e
  m1p6-p2-2026-09-23-71e643b.md (records/.../m1p6-p2-2026-09-23-71e643b.md as installed and committed)
- 8420c098616f2e4fcbfe328ddba288a048251474dd9ed96ff8b8bf40e09b18e1
  m1p6-p2-2026-09-23-71e643b.txt (records/.../m1p6-p2-2026-09-23-71e643b.txt as installed and committed)
- 84ba175d0b9b8c0332a6f7cb2382d1108b6e1c26ba22a53cb362d172185533cd
  negative.log (records/.../negative.log as installed and committed)
- 8187b3a0265ddcf986866185bf7181d5b7feaf5c17c719a65f7bb7fb71e11b55
  report-draft.md (the draft report, second version)
- c0ec41985f76461b861d56f1a6b69c603db43b7e34a9eae126ea7633d88b5cad
  run.log (records/.../run.log as installed and committed)
- 856e26b84528874609af8d636c2aef41e748f9b62a427b14bd819700369ca6d3
  scan-for-checkers.log (the scan run before this check, PASS)
- 45993ed7132e23c63e9e9e5330f834ea209bc8977eb0ef4dc81160b88c339442
  test.log (records/.../test.log as installed and committed)

The answer as returned: 14176 bytes, sha256
ea282465f320b4d4f25bf9d44167975f789b06c2a1e8251309a4e951af87f9da, all ASCII, no path
substituted.

---

# Independent check 2: M1 part 6 P2 at 71e643b

I read every file in <check>. I rebuilt the six diffs in <repo> with the commands in <check>/diff-commands.txt, and each is byte-identical (`cmp`). <check>/0013-formulas.md matches the working-tree spec. <check>/0001-bootstrap.md matches `git show 71e643b:specs/0001-bootstrap.md`. The four logs, the record pair and independent-check-1.md match the untracked files under <repo>/records. gate.log, test.log and negative.log still have check 1's sha256 values, so they were not rerun. run.log (c0ec4198...) and the record pair (6401e77f... and 8420c098...) are new. I also:

- hashed the superseded attempts under <scratch>/p2/out/superseded; they match the hashes in the log headers;
- read the quick-xml 0.42.0 source in <home>/.cargo, whose `escape` writes `\r` as `&#13;`;
- listed <corpus>/runs.

I edited nothing and ran no cargo.

## Q1. Does the diff do what the spec says for P2, including P1 verification ruling 3, finding F2 and the P2 rulings, and nothing else?

**Yes, with one qualification on ruling 3 (G4).**

- **Doc commit 463ccad** (diff-463ccad.patch): it touches only ROADMAP.md and specs/0013. gate.log:1099-1127 shows both blobs at "the hash planning's handoff gave" (8a301579..., dfd66174...). That matches ruling 12 (spec:579-582).
- **C1 f1a5aba** (diff-f1a5aba.patch):
  - **Finding F2:** patch lines 30-37 change comment lines only (test.log:3879-3890: 8 of 8 changed lines are comments). The new text is true of quick-xml 0.42.0 (src/escape.rs:106 and :109).
  - **Ruling 3:** patch lines 59-86. After `written(package)`, the test asserts that the override and the relationship each appear once in the input and 0 times in the output, and that the output has `parts.len() - 1` entries. With the existing `assert_chain_gone`, this covers all three removals.
  - **The qualification:** ruling 3 (spec:548-551) asks that the test show "no duplicate removal recorded". These assertions check the net result, not how many times the removal was recorded. test.log:3858-3872 (the M9 finding) shows the new assertions are never reached under M9, and that a second recording is refused by the write itself (the "two changes for one entry" error), not by C1's code.
  - **Nothing else:** test.log:3894 confirms every changed test line is inside that one function.
- **C2 3c932d9** (diff-3c932d9.patch) matches "Harness runs" (spec:237-257) point for point:
  - A, F and M are read in one pass over the first Worksheet (patch lines 154-187).
  - A is set to `Number(21)`; `(M+1,1)` gets `a1()*2`; `(M+1,2)` gets `1+1`; F gets `2+3` (lines 192-215).
  - `checked_add` fails with `NoNewRow` when M is the last row there is (line 193).
  - The line carries the nine keys after `formula` (lines 241-250).
  - `CellsAfter` and `ReadBack` are asserted, the output is removed and the exit is 2 (lines 235-240 and 282-293).
  - With no A nothing is recorded (lines 192 and 216-217).
  - Tests: A and F, A without F, and neither, plus an F-alone case, and the usage cases (lines 304-483).
  - The only extras are the module doc, `ReadBack` and one `unreachable!` arm, all required by the feature.
- **C3 71e643b** (diff-71e643b.patch): one test fixture and its expectations (lines 113-147), for M11. test.log:3962-3997 confirms.
- **P2 rulings 1 to 4** are not code rulings:
  - Ruling 1 is carried out in run.log (2349-2356 and 2489-2518).
  - Ruling 2's four-reference comparison is at run.log:161-164, and the rows show `changed=2 added=2`.
  - Ruling 3's items are in the report.
  - Ruling 4 is met by the report but not by the record (G2).
  - The rulings section is correctly absent from the committed spec: the gate compares the head blob with planning's hash (gate.log:1112-1121).

## Q2. Do the logs contain every line the spec and specs/0001's rules require for P2?

**No.** Two required items are missing:

- **Ruling 4 (spec:620-623):** "the record and the report state the corrected figures with this ruling as their source". The record pair does neither. `grep ruling|Planning` on m1p6-p2-2026-09-23-71e643b.md finds nothing, and the record has no totals line (G2).
- **specs/0001:219-225 on the C3 message:** C3's message claims that M11 "did not bite". No committed log prints that measurement (G3). It is disclosed, but it is not resolved.

What is present:

- **Lines 1-3** (command, invocation, about): in every log.
- **Printed scripts with `#!/usr/bin/env bash`:** gate.log:49, test.log:48, negative.log:47, run.log:84, scan-for-checkers.log:70.
- **Shape greps before each log, with controls:** gate.log:5-15, test.log:5-15, negative.log:5-22, run.log:12-29.
- **Commit ids from git:** gate.log:17-39, test.log:17-31, negative.log:24-35, run.log:31-43 and 2261-2264. No printed script types a commit id.
- **Phase commit compared with HEAD, with controls:** gate.log:30-35, test.log:41-44, run.log:4-9, negative.log:40-44.
- **Both targets:** test.log:1937 and 2475 (492/0/2 on each), and the per-binary breakdown at 2480-2500. run.log has the host run at 2365-2522, i686 at 2523-2680, and agreement between them at 2681-2685.
- **Commit-message counts:** C1's are derived at C1 (test.log:3906-3921). C2 and C3 state no count (3941-3961 and 3992-3997).
- **Mutations:** every one prints its plant, run and failure (test.log:3099-3835). The table is at 3836-3850. M13 is recorded as a finding (3851-3856) and M9 as measured (3858-3872).
- **Run-log checks and controls:** controls for each flag at run.log:2146-2258. Real-output plants at 2519-2521 and 2677-2679. The statement that the image recomputes regardless of the flag is at 71-76. Converted files are deleted after the read (2479 and 2635).
- **Ruling 1 expectations, derived, each with a one-off control:** run.log:2355-2356 and 2489-2518 (host), and the same block for i686. The id-set check with a swapped-id control is at 2515-2516.
- **Build directory:** named once as <scratch>/p2/target in the headers of gate.log:43, test.log:38, negative.log:3184 and run.log:43.
- **Placeholders:** no absolute path and no non-ASCII byte in any log, the record pair or the report. I checked with `grep -P '[^\x00-\x7F]'` and a path grep. The only hits are the scripts' own pattern text at run.log:1065 and 1070.

### Status of check 1's findings

| | Status | How |
|---|---|---|
| F1 | Resolved in the logs | run.log:2260-2305 hashes attempt 1's log, quotes its 4 `rm -rf` commands and verdicts verbatim, checks each directory is absent now, and prints P2's two superseded directories as KEPT. The report (62-66) asks planning to rule. Residuals: ruling 9 says "printed by P2's run.log header", but this is in the body. The removal it quotes is in an out-of-repository log. |
| F2 | Disclosed only | Report 22-26 names test-attempt2.log and its hash; <scratch>/.../test-attempt2.log:3602-3604 shows it. The C3 message still states it, and history is not amended. The report does not ask planning for a ruling like P1 ruling 10 (G3). |
| F3 | Resolved in the report | Disclosed item 3 (report 78-81) keeps the two kinds of hash apart. |
| F4 | Disclosed, not resolved | test.log:3868-3871 and report 38-43. |
| F5 | Resolved | Report 48-49: "CellsAfter, which predates P2, and the new ReadBack". |
| F6 | Resolved | run.log:2320-2322 reads manifest_rows, and the derived checks use it. The literal 27 left at script line 797 is a check against the spec's Facts, not a derived expectation. |
| F7 | Disclosed, inaccurately and incompletely | See G1. |
| F8 | Resolved | run.log:2515-2516 and 2673-2674 check the set of ids, with a swapped-id control. |
| F9 | Partly disclosed | Report item 6 names only negative.log. run.log:60 and scan-for-checkers.log:42 carry the same process-count-free SERIALIZED line (G5). The descriptor flag is mentioned (report 59-60), but not its count (30) or its cause (G6). |

## Q3. Does every number and quoted string in report-draft.md appear in a primary?

**No. There is one string I could not find in any primary, and one statement that is incomplete.**

1. **Report line 87: `<scratch>/mut/tree/target`** is in no log. In test.log, <scratch>/mut/tree is the mutation worktree (line 692). The mutations build with `CARGO_TARGET_DIR="$MUTTREE/target"` (lines 850 and 909), which `hide` prints as `<scratch>/mut/target` (line 158). The string appears only in independent-check-1.md, which is not a primary for this purpose.
2. **Report line 89:** "negative.log's SERIALIZED line carries no process count, unlike gate.log and test.log." The words are accurate, but run.log:60 and scan-for-checkers.log:42 have the same gap and go unmentioned.

Everything else I found:

- **Commits:** the ids and subjects are in gate.log:1053-1057.
- **Tests:** the test names are in test.log:2510-2512. The counts 492/0/2, i686 the same, and 489 are at test.log:1937, 2475 and 2502. The nine PASS lines are at gate.log:1351-1362.
- **Mutations:** M1-M14 with only M13 not biting is at test.log:3837-3851. "two changes for one entry" is at test.log:3866. That the older test predates P2 I confirmed in git: the test was added in 4b7f55c (part 4 P2).
- **Gate strings:** "the hash planning's handoff gave" is at gate.log:1102. `git show --no-color --no-ext-diff --format=` is at gate.log:1130.
- **negative.log:** 4294967295 is at 676-677. The refusal list matches H6-H12 at 4374-4428, and `CellsAfter`/`ReadBack` through two <sheetData> elements matches 778-789 and 4430-4452.
- **run.log counts:**
  - 19, 15, 9, 3 and 38, 15, 9, 6, 19 are at 2349-2350.
  - 42 is at 1343. The 2 and the 5 are in the spec at 272-273.
  - "verdict rows other than as-expected: 0" is at 2718.
  - The descriptor-only control is at 2311.
  - Run directory -c is at 43 and 2321, and the superseded -, -b and -c attempts at 2300-2305 and 45.
- **Test attempt:** 3f39fa87 is at test.log:34 and 37.
- **Attempt history:** the false SERIALIZED line in attempt 1 is at run.log:186-187. Ruling 3(a) and 3(e) are at spec:611-616. "chain_after 0" is at run.log:103-104.

## Findings

**G1 (medium-low): report disclosure 5 names a build directory no log prints, and understates the `<scratch>` ambiguity.**
- The mutations build in `<scratch>/mut/target` (test.log:158, 850 and 909), not `<scratch>/mut/tree/target`.
- `$MUTTREE` is a `mktemp -d` directory (test.log:119), and `$T` is also written `<scratch>/w` (test.log:159). So test.log, not only negative.log, uses `<scratch>` for more than one root.
- The report copied check 1's wording instead of checking it against the primary.

**G2 (medium): P2 ruling 4 is not met by the record, and this is not disclosed.**
- Ruling 4 says "the record and the report state the corrected figures with this ruling as their source" (spec:620-622).
- The record pair has no totals line and cites no ruling. Its "differs from the expectation" column lists all 19 workbooks with A as differing (m1p6-p2...md:78-118).
- "The runner and the record script are unchanged" (spec:259-260) makes this hard to meet. Planning needs to rule, or the report should say that the corrected figures are stated only in run.log and the report.

**G3 (medium-low): C3's commit message states a measurement no committed log prints.**
- "test.log's mutation M11 ... did not bite" (diff-71e643b.patch:98-99).
- specs/0001:215-225 says nothing is called measured unless a committed log prints it.
- The measurement is only in the out-of-repository test-attempt2.log:3602-3604. The committed test.log:34 says only that attempt 2 was superseded "for M11".
- It is disclosed (report 22-26), but the report does not ask for a ruling as P1 ruling 10 gave for M5.

**G4 (low): ruling 3's strengthened assertions are not shown to detect anything.**
- The ruling asks the test to show "no duplicate removal recorded". The new assertions check the net result: 1 in the input, 0 in the output, one entry fewer.
- A duplicate recording would be refused by the write first ("two changes for one entry"), so the assertions can only be reached when there is no duplicate.
- test.log:3868-3871 records this honestly. Planning should decide whether this satisfies ruling 3.

**G5 (low): disclosure 6 is incomplete.**
- run.log:60 and scan-for-checkers.log:42 also carry a SERIALIZED line with no process count.
- Only gate.log:43 and test.log:38 count cargo and rustc processes.

**G6 (low/info): the descriptor flag is described but not quantified.**
- 30 changed entries are flagged `descriptor` on each target: 15 workbook, 15 worksheet (record md:26-27 and 47).
- By the record's rows, these are the 15 LibreOffice-produced workbooks with A. None of the 4 excel-win ones is flagged.
- The report (59-60) says only that descriptor is the one flag allowed. The spec's expected results mention statuses, not flags.

**G7 (low): the report prints no sha256 or byte count for the logs or the record pair.**
- Planning's P1 verification checked the record commit "at the sha256 and byte counts the report prints" (spec:500-501).
- Draft 2 has none. The values exist: run.log:2701-2702 for the record, and the gate.log sha256 read in negative.log.

**G8 (low): ruling 9's removal print is in run.log's body, quoted from a superseded log.**
- Ruling 9 says each removal is "printed by P2's run.log header" (spec:563-565). It is printed at run.log:2260-2305, as a verbatim quote from run-attempt1.log (6499480f..., at <scratch>/p2/out/superseded/run-roles, lines 2139-2145), which is outside the repository.
- The substance is met. The placement and the dependence on a file outside the repository are left for planning.

**G9 (info): small inaccuracies in the independent-check-1.md header.**
- It says the answer is "byte for byte as it was returned". The committed body adds one trailing newline that <scratch>/p2/check1-answer.md (10051 bytes, 93087f81...) does not have.
- scan-for-checkers.log:2 writes the script as `<scratch>/m1p6-p2-scan.sh`, while every other log writes its script under `<scratch>/p2/`.

**G10 (info): this is the phase's second and last allowed independent check** (spec:305). If G1-G3 lead to changes in the logs or the report, there is no third check to confirm them. Planning's own verification from primaries has to cover them.
