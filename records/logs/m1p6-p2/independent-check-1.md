# Independent check 1: M1 part 6 P2

Written by the implementing session. The header is mine; everything after
the rule is the checker's answer, byte for byte as it was returned.

The checker was a fresh subagent (not a fork) with no access to this
conversation. It was given the seventeen files below and nothing else (it
disclosed read-only git, sha256 and ls commands over the repository and
the scratch directory), and asked the three questions CLAUDE.md names. It
was told to write <repo>, <check>, <home> and <corpus> in place of
absolute paths and to answer in ASCII.

Commit checked: 71e643b949e42399cc5a87d195934aba490beba0
("M1 part 6 P2: a shared string before the first number in the A-only sample test"),
which was HEAD when the check ran and when this file was written. Range
read: 6d85828..71e643b. Each per-commit diff's producing command is named
beside it below (P1 verification ruling 2).

After the check, run.log and the record pair were rerun at the same commit
for its F1, F6 and F8; the report names each change.

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
  gate.log (records/logs/m1p6-p2/gate.log as installed then)
- 40e3307ef322553221596b5260084eb5974ec3d810b478c94431660640c7a55c
  m1p6-p2-2026-09-23-71e643b.md (records/m1p6-p2-2026-09-23-71e643b.md as installed then; superseded after this check with run.log)
- 334bce81f3e3de732da355ece9c4286e03ad0624541e68031b3a107adfef460b
  m1p6-p2-2026-09-23-71e643b.txt (records/m1p6-p2-2026-09-23-71e643b.txt as installed then; superseded after this check with run.log)
- 84ba175d0b9b8c0332a6f7cb2382d1108b6e1c26ba22a53cb362d172185533cd
  negative.log (records/logs/m1p6-p2/negative.log as installed then)
- 6ad796872c4b432576f28d4dd493b4b7972d78a9a0fa019c2f151455829432f8
  report-draft.md (the draft report)
- c35be04fd6f059b181b3510c83613ced5ebce59fd609f7bdecfe73a5df631e14
  run.log (records/logs/m1p6-p2/run.log as installed then; superseded after this check (F1, F6, F8))
- bfc7dfcbf17f90a4ad07ace00277c4fdb0127a319e64f8f35432b5c110e2947e
  scan-for-checkers.log (the scan run before this check, FAIL only for want of answers)
- 45993ed7132e23c63e9e9e5330f834ea209bc8977eb0ef4dc81160b88c339442
  test.log (records/logs/m1p6-p2/test.log as installed then)

The answer as returned: 10051 bytes, sha256
93087f818b7025ccde32f3076d07f677ff672df1fa148683a77681dff8c7568a, all ASCII, no path
substituted.

---

# Independent check: M1 part 6 P2 at 71e643b

I read everything in <check>. I rebuilt every diff in <check> from git in <repo> using the commands in <check>/diff-commands.txt, and all six sha256 values match. I also rebuilt the four per-commit hashes that gate.log prints (gate.log:1128-1141) with its own `git show --no-color --no-ext-diff --format=` command, and those match too. <check>/0013-formulas.md is byte-identical to <repo>/specs/0013-formulas.md in the working tree; its sha256 is 5a1b838d..., the value run.log:2040 prints. The four logs and the record pair in <check> are byte-identical to the untracked files under <repo>/records. gate.log's sha256 is b44d0558..., the value negative.log:3190 reads. The superseded attempts exist in <scratch>/p2/out/superseded at the hashes the headers print. I edited nothing and ran no cargo.

## Q1. Does the diff do what the P2 spec says, including P1 verification rulings 3 and F2 and the P2 rulings, and nothing else?

**Yes.** The one qualification is that ruling 3's strengthened test has no mutation showing it can fail (F4).

- **Doc commit 463ccad** (diff-463ccad.patch): it touches ROADMAP.md and specs/0013 only. gate.log:1099-1127 shows both blobs at the hashes planning's handoff gave, and I reproduced both (8a301579..., dfd66174...).
- **C1 f1a5aba** (diff-f1a5aba.patch):
  - Finding F2: cells.rs changes comment lines only (patch lines 29-35; test.log:3879-3890 confirms 8 of 8 changed lines are comments).
  - Ruling 3: the_callers_drop_then_a_written_formula_records_the_recipe_once now also asserts, after `write_to`, that the override and the relationship each appear once in the input and zero times in the output, and that the output has one entry fewer (patch lines 52-81). With the existing `assert_chain_gone`, that covers all three removals.
  - Nothing else in C1: gate.log:1142-1149 and test.log:3882-3895 show every other changed line lies inside that one test.
- **C2 3c932d9** (diff-3c932d9.patch): it matches "Harness runs" point for point.
  - The first sheet of kind Worksheet, read once for A, F and M (patch lines 139-170).
  - A set to 21, `(M+1,1)` set to `a1()*2`, `(M+1,2)` set to `1+1`, F set to `2+3`.
  - One line with the ten keys.
  - `cells_after` and `read_back` asserted, exit 2, output removed.
  - With no A, nothing is recorded.
  - The integration tests cover A and F, A without F, and neither (plus F alone), and the usage cases.
  - The only additions beyond that are the module doc, the `ReadBack` error and the `unreachable!` arm.
- **C3 71e643b** (diff-71e643b.patch): it changes one test fixture, adding a shared string ahead of the number (test.log:3962-3997).
- **P2 rulings 1 to 4:** these are expressed in run.log, not in code, which is right because ruling 1 corrects the expectation, not the tool. The rulings section is correctly absent from the diff; ruling 4 puts it in P3's doc commit.

## Q2. Do the logs contain every line the spec and specs/0001's phase rules require?

**No. There is one gap.** P1 verification ruling 9 requires each stale P1 run directory's removal to be printed by P2's run.log. The committed run.log:2199-2202 prints "absent already; nothing to remove" for all four (see F1). Everything else asked about is present:

- **Both targets:**
  - test.log:1937 and 2475 (492/0/2 on each target).
  - run.log:2260-2571 (host and i686 runs).
  - run.log:2572-2577 (the two targets agree, with a control).
- **Lines 1-3** (command, invocation, about): present in gate.log, test.log, negative.log, run.log and scan-for-checkers.log.
- **Scripts printed:** gate.log:48, test.log:47, negative.log:46, run.log:74 and run.log:1201; the counting library is printed in each log.
- **Shape greps over each script before its log:** gate.log:5-15, test.log:5-15, negative.log:5-22 and run.log:12-29, all with controls and all ahead of the "## script" section.
- **Commit ids from git:** gate.log:17-39 and 1052-1079, test.log:17-31, negative.log:24-44, run.log:31-43 and 2183-2187. I found no typed commit id in any printed script.
- **Commit-message counts:**
  - C1's word counts are derived at C1 (test.log:3896-3921).
  - C2 states no count (test.log:3941-3961).
  - C3 states no count (test.log:3992-3997).
- **Mutations:** test.log:3099-3835, each with the planted diff, the run and the failing assertion. M13 is measured as a finding (3758-3783). M9's scope is measured (3858-3872).
- **Placeholders:** no absolute path, and no non-ASCII byte, in any log, the record pair or the report.
- **Build directory (ruling 6):** named as `<scratch>/p2/target` in gate.log:43, test.log:38, negative.log:3184 and run.log:43 (see F7 for side builds).
- **Comparison and computation checks:**
  - Run on every output: run.log:2305-2379 and 2461-2535.
  - Controls per flag on three fictional pairs: run.log:2069-2182.
  - Real-output plants: run.log:2412-2414 and 2568-2570.
  - The log states the image recomputes regardless of the flag: run.log:59-63.
- **Corrected expectations derived, not typed (P2 ruling 1):**
  - The log's own read: run.log:2225-2253.
  - Each assertion with a one-off control: run.log:2389-2411 and 2545-2567.
  - Two derivations use a typed 27 for the corpus size (F6).

## Q3. Does every number and quoted string in report-draft.md appear in a primary?

**No.** Every number appears in a primary. Three statements do not match the committed primaries:

1. **Report line 53-54:** "The four stale P1 run directories were removed first (ruling 9), each printed." The committed run.log:2199-2202 says each was "absent already". The removals are printed only in the superseded run-attempt1.log (sha256 6499480f..., its lines 2133-2140), which is outside the repository and which the rerun header (run.log:45) does not say performed them.
2. **Report line 22-23:** "M11 ... did not bite". This appears only in C3's authored message (diff-71e643b.patch:10). No committed log prints the measurement. It is in the superseded test-attempt2.log (3f39fa87..., lines 3602-3604).
3. **Report line 43:** "both new assertion messages". `CellsAfter` predates P2 (diff-3c932d9.patch:77 shows it as unchanged context); only `ReadBack` is new.

These were found as stated:

- **Counts:** 492/0/2 and 489; the nine PASS lines; M1-M14 with M13 not biting; "two changes for one entry"; 4294967295 (negative.log:676); 19/15/9/3 and 38/15/9/6/19; 42, 2 and 5 (run.log:1266 and 1905-1908, and the spec).
- **Verdict line:** "Verdict rows other than as-expected: 0".
- **Names and references:** 2026-09-23-71e643b-b; the commit ids; the test names; ReadBack; NoNewRow; "the hash planning's handoff gave"; rulings 3(a) and 3(e); "chain_after 0".

## Findings

- **F1 (medium): ruling 9's print is not in the committed run.log.**
  - The four P1 directories were removed by the superseded attempt 1. The committed run.log only shows them already absent, and its rerun header does not say attempt 1 removed them. So the record as committed cannot show the removal, and the report overstates it (report:53).
  - P2's own superseded run directory, <corpus>/runs/2026-09-23-71e643b, is still present (I saw it with `ls`), and neither the logs nor the report mention it.
- **F2 (medium-low): the M11 "did not bite" claim rests only on a superseded, uncommitted log.**
  - It appears in C3's message and in report:22-23. specs/0001:219-225 requires a count or claim in a commit message to be one a committed log prints, or the message must say none does.
  - The precedent is P1 ruling 10, which accepted the same situation for M5 as disclosure. Here it should be disclosed explicitly, naming test-attempt2.log and its hash.
- **F3 (low): the report cites ruling 2 for hashes that don't cover the check directory's diffs.**
  - The report (lines 30-31) attaches ruling 2 to gate.log's diff hashes. gate.log hashes `git show --format=` output, but the check directory's diffs were made with `git show --format=fuller` (diff-commands.txt).
  - No log hashes the diffs in the check directory. Ruling 2 itself is met by diff-commands.txt; the report should keep the two apart.
- **F4 (low): nothing shows C1's new assertions can fail.**
  - test.log:3859-3861 shows M9 fails the test before `write_to`, and :3869-3872 shows the write refuses a second recording itself.
  - So "no duplicate removal" is enforced by the library's own refusal, not by the new assertions, which largely overlap `assert_chain_gone`. This breaks no rule, since no bite is claimed, but ruling 3's strengthening is not shown to catch anything.
- **F5 (low): report:43 calls both assertion messages "new"; only `ReadBack` is.**
- **F6 (low): run.log uses a typed 27 in derived checks.**
  - It appears at run.log:864, 866-868 and 883 (the `n_added_ok` target), and in the files_identical expectation `27 - n_a` at run.log:894.
  - The log reads the manifest's row count itself at run.log:737. Ruling 1 says "never typed".
- **F7 (low): the `<scratch>` placeholder is ambiguous in negative.log.**
  - It stands for two different roots: the `mktemp` directory B and the scratch root SROOT (negative.log:159-175, `hide()`).
  - So `<scratch>/clone` and `<scratch>/gate-clone` (which runs a cargo doc build, negative.log:5009) look like siblings of `<scratch>/p2/target` but are not.
  - test.log's mutations also build in `<scratch>/mut/tree/target`. These side build directories are arguably outside ruling 6, but planning should rule.
- **F8 (low): one check's label overstates what it checks.**
  - "files_identical is yes on exactly the workbooks without A" is a count equality (run.log:894), not an identity check per workbook.
  - The per-workbook side rests on `entries_p2` refusing changed rows for a workbook with no A (control at run.log:2211).
- **F9 (info): two small gaps with no rule attached.**
  - negative.log:38's SERIALIZED line has no process-count evidence, unlike gate.log:43-44 and test.log:38-39.
  - The record shows 30 changed entries flagged `descriptor` (workbook 15, worksheet 15). run.log accepts these, but the report does not mention them.
