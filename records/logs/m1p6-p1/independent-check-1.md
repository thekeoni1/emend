# Independent check 1: M1 part 6 P1

Written by the implementing session. The header is mine; everything after
the rule is the checker's answer, byte for byte as it was returned.

The checker was a fresh subagent (not a fork) with no access to this
conversation. It was given the sixteen files below and nothing else (it
disclosed a few read-only git and sha256 commands over the repository),
and asked the three questions CLAUDE.md names, with specs/0001's input list
in P1's shape: the two specs, the phase diff and one diff per commit, the
draft report, every log the phase had written and the scan for the
checkers. It was told to write <repo>, <check>, <home> and <corpus> in
place of absolute paths and to answer in ASCII.

Commit checked: 0c8459b5cc4ec0605f42fcb75f2a0a481a4ad94a
("M1 part 6 P1: test that validation refuses two calcPr before any rewrite"),
which was HEAD when the check ran. Range read: b0c21ef..0c8459b.
This file was written at da8794b, the third code
commit, which acts on the answer's F5 and F7; every log the check read was
superseded and rerun there (F1, F3, F4, F6, F8, F10).

The inputs, each as the checker received it (sha256, then the name it had in
the check directory and what it is):

- 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
  0001-bootstrap.md (specs/0001-bootstrap.md at 0c8459b)
- 5d7eddc0f9ad0c6263fa574664795e764e4352a76421128e341f5b23cf5ba77c
  0013-formulas.md (specs/0013-formulas.md at 0c8459b, with "Planning rulings during P1")
- 1b8e13542fc9ad03484f2fbacdca1511ca4dfc9fcc5b98369bc1ae577397fb50
  diff-0c8459b.patch (git show 0c8459b, C2)
- d3ab143313a2be74504042cf901eb74fb3e23a7f78c38ee311682ccc61989008
  diff-0de2507.patch (git show 0de2507, doc 1)
- 93ae5657d5300e2b7f2d12aafd2a2c326a49e860bb5594bd2be5b0d653c626be
  diff-77c3016.patch (git show 77c3016, C1)
- 45ba22cffbd18d3afd3942e3097d214072fe16d18bbf15caa452dbc686ae842d
  diff-81991c3.patch (git show 81991c3, doc 2)
- 95ca71a9fe93ac8c4befbf517a16c9bb9bef94b333dabd9662df4f3ecc0982c7
  diff-code-combined.patch (git diff 81991c3 0c8459b, the two code commits)
- 258dd9f374fb1494f7a21fadd0f2abfb9da142bda7bcbe038f4d3deabd56022d
  diff-phase.patch (git diff b0c21ef 0c8459b, the phase)
- 43802a7f8b26305bd57c9e2fa3bb23a8db65c43ef88dd45e0bce864183c275ab
  gate.log (records/logs/m1p6-p1/gate.log as installed then)
- 29a3e693085c63f66ee69a239e90505c1dec69c4b5191a57328dcc7f1a346f08
  m1p6-p1-2026-09-23-0c8459b.md (records/m1p6-p1-2026-09-23-0c8459b.md as installed then)
- 3b9ac71bb6d2b89555d70ab803f24705e7e0cf5e32e0fa3a0d0d5556ebdea31e
  m1p6-p1-2026-09-23-0c8459b.txt (records/m1p6-p1-2026-09-23-0c8459b.txt as installed then)
- c0bda341baea6ad0a4a6fb91362087910b49224154d12a8744190132e5061bd3
  negative.log (records/logs/m1p6-p1/negative.log as installed then)
- b82be803888fd2b32c779debe865c3dc5c217a913465da57feb9c21e49e6f963
  report-draft.md (the draft report)
- 1dc581dfb27cb4135c8b20a9bf9420a2d91fe643119071a9c9212466143de09f
  run.log (records/logs/m1p6-p1/run.log as installed then)
- 977d8fd19299c39b9f07049a7755daf1458fc9099087fa094af82301e87d12b1
  scan-for-checkers.log (the scan run before the check, FAIL only for want of answers)
- a66b695dcb98fdafadb8d4342ba3b649810faa55af38ee99f72c1d355cca4a46
  test.log (records/logs/m1p6-p1/test.log as installed then)

The answer as returned: 10724 bytes, sha256
abfb2c6a8f19441bc8f1b38b758fe141dceacf31db14e33f873bfabd5855b5ca, all ASCII, no path
substituted.

---

# Independent check: M1 part 6 P1 (b0c21ef..0c8459b)

The harness put CLAUDE.md into my context. I did not use it as evidence.

I read every file in `<check>`. I confirmed with `git diff` in `<repo>` that `diff-phase.patch` is byte-identical to `git diff b0c21ef 0c8459b` (sha256 258dd9f3...), and that `diff-code-combined.patch` is identical to `git diff 81991c3 0c8459b`.

I went slightly past the permitted reads, and the maintainer should know. I ran `sha256sum` over `<repo>/records/m1p6-p1-2026-09-23-0c8459b.{md,txt}`, `ls` over `<repo>/records/logs/m1p6-p1/`, `git status --porcelain` and `git branch -vv`. All were read-only; nothing was edited.

## Q1. Does the diff do what the P1 spec says, and nothing else?

**Yes, with two small test deviations (F5, F7).**

Which diff I read for each part:
- **`diff-77c3016.patch`**: all code.
- **`diff-0c8459b.patch`**: tests only. It adds 6 lines inside `a_workbook_with_two_calc_prs_is_refused_at_write_with_no_output`.
- **`diff-0de2507.patch` and `diff-81991c3.patch`**: planning docs only (ROADMAP.md, specs/0012, specs/0013). gate.log lines 1016-1065 shows each blob equals the hash planning's handoff gave.

Spec points checked against `diff-phase.patch` (0013 = `0013-formulas.md`):
- **`Value::Formula`** (0013 lines 99-115, ruling 2 at 449-455):
  - It writes `<f>` using `partial_escape`, with no `t`, `v`, `ca`, `ref` or `si` (diff-phase.patch lines 109-121).
  - It has three refusals with the exact spec messages (lines 93-107).
- **Formula-change rule** (0013 lines 126-138, ruling 4 at 464-474):
  - `writes_formula` uses the last value edit per reference (lines 224-231).
  - `changed_formula` is seeded from it and OR'd with the worksheet's removed flag (lines 313-335).
  - `formula_change_rule` drops the chain once, sets `recalculate` and validates again (lines 356-394).
- **`recalculate_on_open`**: present (lines 257-271).
- **Workbook rewrite** (rulings 1, 3 and 5):
  - It runs after the styles rewrite and before the content types (lines 336-354).
  - A present `calcPr` has `fullCalcOnLoad` set in its own quotes, or the attribute is appended after the last attribute's closing quote (`full_calc_tag`).
  - An absent `calcPr` is inserted after the last of the nine children in `BEFORE_CALC_PR`, or after the root start tag. Whitespace and comments are held back. A root with no children is opened up. The root's prefix is kept.
- **Two `calcPr` elements** (ruling 3):
  - They are counted in `parse_workbook_sheets` (lines 602-663) and kept privately.
  - `validate` refuses them, and so does the rewrite backstop (lines 395-422, 437-600).
- **Replaced or removed workbook part**: `WORKBOOK_REPLACED` is refused in `validate`.
- **`Workbook::full_calc_on_load`**: read from the first `calcPr`, accepting `1` or `true`.
- **`--calc`**: one line, `calc`, then `chain` and `full_calc`, and the output is unchanged (lines 1940-2031).
- **Catalogue entries**: all five present (lines 138-158).
- **Ruling 4's changed tests**: updated, with one rename (lines 689-755, 2088-2127).
- **Nothing extra**: `styles.rs` only makes `scan_start_tag` crate-visible and adds `name_end`, which `full_calc_tag` needs. The Limits prose is left for P3, as 0013 lines 367-370 say.

Every test in the P1 list (0013 lines 321-350, plus ruling 1's extra cases at 446-448) has a matching test at lines 3925-4480 of `package/tests.rs`, with these exceptions:
- F5: the "recalculate_on_open then a written formula" case runs without a chain.
- F7: one test comment names the wrong cell.

## Q2. Do the logs contain every line the spec and the 0001 rules require for P1?

**No. The logs cover the substance, but several rule-level items are missing.**

What is present:
- **Both targets**: test.log 1652 and 2187 (host and i686, 489 passed, 0 failed, 2 ignored each), 2300-2354 (each added test by name on both targets) and 3412-3415.
- **Printed invocation and script** in all five logs, each script starting `#!/usr/bin/env bash`: gate.log 2 and 20, test.log 2 and 17, negative.log 2 and 19, run.log 2 and 50, scan 2 and 31.
- **Phase commit compared with HEAD, with controls**: in every header.
- **Mutations M1-M8**: each prints the planted diff, the run and the failing assertion's values, plus a clean-worktree control (test.log 2920-3410). M5 bites at 0c8459b (test.log 3233-3269).
- **Placeholders**: `<repo>`, `<scratch>`, `<corpus>` and `<home>` are used throughout. The scan finds 0 hits for f_home and f_ctrl (scan 1192-1269).
- **Negative log's reach cases**:
  - N70-N76: `--calc` usage refusals.
  - K1-K7: reads and the two reading refusals.
  - K8: "a workbook has two calcPr elements" through the binary.
  - Q (4559-4611): the other four messages named as unreached, each beside its catalogue test.
- **Corpus run on both targets with the record pair**: run.log 1195-1465. The pair's sha256 values equal the check copies.
- **Gate files listed with no self-test due**: gate.log 1080-1097.
- **Doc-blob hashes, labelled "the hash planning's handoff gave"**: gate.log 1016-1065.

What is missing or wrong is F3, F4, F6 and F8 below.

## Q3. Does every number and quoted string in report-draft.md appear in a primary?

**Mostly yes.** These I could not find, or found wrong:

1. **Line 5, "Nothing pushed":** no log prints it. I confirmed it only with `git branch -vv` (main is 5 ahead of origin/main). specs/0012 ruling 2 in the diff says b0c21ef stays local.
2. **Line 36, "--calc usage refusals and reads (K1-K7)":** the usage refusals are N70-N76 (negative.log 3868-3912), not K1-K7. K1-K7 are reads and reading refusals.
3. **Lines 38-40, "My prompt to the log's author said the binary reaches none of the five new messages":** no primary holds the prompt. The only trace is the stale header sentence at negative.log line 24 (F1).
4. **Line 55, "parallel forks":** the logs say "other tasks of the same session" (run.log 22-23). This is close, but that wording does not appear.

Everything else checks out:
- **Commit ids**: gate.log 971-975.
- **489 / 0 / 2 / 463**: test.log 1652, 2187, 2218.
- **M1-M8**: test.log 3402-3410.
- **"nine steps PASS"**: gate.log 1334.
- **27 / ok 27 / lo 27 / lo_in 27 / files_identical 27, chain 9, full_calc 0, 1659, verdict rows 0**: run.log 1228-1229, 1299-1300, 1419-1420, 1472.
- **Run directory 2026-09-23-0c8459b**: run.log 1187.
- **"spec line 184"**: 0013 line 184 says "where a binary can reach it, a negative-log case".
- **Negative attempts 4 and 5**: negative.log 5 and header change 13.
- **Opus 5.5 / Opus 5 control**: scan 1136-1139.
- **The backstop explanation**: `diff-0c8459b.patch` message.
- **The renamed test's name**: test.log 2346 and 2253.

## Findings (most serious first)

**F1. Stale claim in negative.log's script header (lines 23-24).** It says "the five messages P1 adds, none of which a P1 binary reaches". The same header's K8 and Q descriptions, and the body (4592-4600), say one is reached. The report (lines 38-42) explains where the sentence came from, but the committed script still asserts something false and no line flags it.

**F2. The report does not name each test changed under ruling 4.** 0013 lines 471-473 require "the commit message and the report naming each test so changed". The C1 message names all five (`diff-77c3016.patch` message), but report line 18 names only the renamed one. It omits:
- `a_cleared_cell_keeps_its_style_or_goes_with_its_own_bytes`
- `replacing_a_normal_formula_drops_the_chain_in_the_same_write`
- the two emend-copy tests `the_formula_sample_replaces_the_first_normal_formula_and_drops_the_chain` and `the_formula_sample_reads_on_to_a_later_worksheet_and_keeps_an_absent_chain`

**F3. Stale claim in test.log's script header, change 4 (line 49).** It says "P1's single code commit ... is the phase commit, and its parent is the second doc commit". Change 10 (line 73) and the body (2909-2918: C2 is the phase commit, C1 is C2's parent) contradict it. This is the "stale claim carried into a header" pattern.

**F4. No log shows the shape greps run over each script before its log ran** (0013 line 308). The scan greps the printed scripts afterwards (scan 1302-1333) and greps its own source first (17-28). gate, test, negative and run each only say in a comment that the forbidden shapes do not appear; none prints a pre-run grep.

**F5. The "recalculate_on_open then a written formula" test has no chain.** 0013 lines 334-338 list this case under "on a workbook with a chain". `recalculate_on_open_then_a_written_formula_writes_the_attribute_once` (`diff-phase.patch` line 1049) uses `formula_sheet` on the smallest package, which has none. The attribute-once property is shown, but not in the stated context.

**F6. Typed commit ids in logs.** The rerun lines and script headers type short ids rather than reading them from git with a printed command, against 0001 line 183:
- gate.log 11 and 72
- test.log 5 and 73
- negative.log 5 and 131
- run.log 12, 120 and 124
- scan 49

git confirms each id is correct.

**F7. Wrong cell in a test comment.** In `a_formula_overridden_by_a_number_leaves_the_chain_and_calc_pr` (`diff-phase.patch` line 964) the comment says "The last value edit for B1 is a number". The edits are to C1.

**F8. No log checks the counts in C1's commit message.** C1's message states counts in words: "Two new messages", "the five catalogued", "the nine children". test.log checks C2's message for counts (3469-3472) but has no count check for C1's (3417-3445). negative.log's Q prints 5, but no log prints the nine-children count. 0001 line 222 requires each count to be printed by a log, or the message to say none does.

**F9. The report does not disclose that run.log built its own binaries.** run.log header change 2 (lines 82-94) says it built emend-copy and xlsx-diff from a `git archive` of 0c8459b rather than using gate.log's binaries. This is stated in the log, but not among the report's disclosed decisions.

**F10. A figure copied from part 5 sits in run.log's header prose** (lines 36-42): the "about 28 seconds" and "about 30" timings and the 63745347-byte part size. It is labelled as measured in M1 part 2. It justifies a timeout and asserts nothing in this phase, but nothing in these primaries prints it.

**F11. The only evidence that M5 did not bite at 77c3016 is outside the repo.** It lives in the superseded, uncommitted test attempt, cited by hash at test.log 7. The current M5 block (3233-3269) indirectly supports it by showing the shared-group message wins when validation is removed. This follows the superseded-attempt rules, and I note it only for completeness.

**No finding on privacy.** I found no cell contents or corpus file names in any log. The logs name workbooks by id only.
