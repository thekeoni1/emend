# Independent check 2: M1 part 6 P1

Written by the implementing session. The header is mine; everything after
the rule is the checker's answer, byte for byte as it was returned.

The checker was a fresh subagent (not a fork) with no access to this
conversation. It was given the eighteen files below and nothing else (it
disclosed read-only git and sha256 commands over the repository), and asked
the three questions CLAUDE.md names plus whether each of check 1's findings
F1 to F11 is resolved. It was told to write <repo>, <check>, <home> and
<corpus> in place of absolute paths and to answer in ASCII.

Commit checked: da8794b85f2d97f97d8ee19f92abfe25b849eb1d
("M1 part 6 P1: recalculate_on_open then a formula on a workbook with a chain"),
which was HEAD when the check ran and when this file was written. Range
read: b0c21ef..da8794b.

This is the second and last check the spec allows. After it, and without a
further check, the four logs and the record pair were rerun at the same
commit for its G2 (the about line back on line 3), and negative.log gained
case K9 for its G1 (the replaced-workbook message is reached through the
binary). The report names each change.

The inputs, each as the checker received it (sha256, then the name it had in
the check directory and what it is):

- 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
  0001-bootstrap.md (specs/0001-bootstrap.md at da8794b)
- 5d7eddc0f9ad0c6263fa574664795e764e4352a76421128e341f5b23cf5ba77c
  0013-formulas.md (specs/0013-formulas.md at da8794b, with "Planning rulings during P1")
- 1b8e13542fc9ad03484f2fbacdca1511ca4dfc9fcc5b98369bc1ae577397fb50
  diff-0c8459b.patch (git show 0c8459b, C2)
- d3ab143313a2be74504042cf901eb74fb3e23a7f78c38ee311682ccc61989008
  diff-0de2507.patch (git show 0de2507, doc 1)
- 93ae5657d5300e2b7f2d12aafd2a2c326a49e860bb5594bd2be5b0d653c626be
  diff-77c3016.patch (git show 77c3016, C1)
- 45ba22cffbd18d3afd3942e3097d214072fe16d18bbf15caa452dbc686ae842d
  diff-81991c3.patch (git show 81991c3, doc 2)
- cf89b2d49f41f3ecc645b683c4c8b2cacaa443ae5eb3fea3ad042f2b4ea62a06
  diff-code-combined.patch (git diff 81991c3 da8794b, the three code commits)
- 9a77b8e096b4ea187786c584167382d6fef499b754b3a7a89f798debc5cdcba9
  diff-da8794b.patch (git show da8794b, C3)
- 3f61d3716aaf0988a4a505ce00c9faf311d65bc465991f88d2c2c33536127d3c
  diff-phase.patch (git diff b0c21ef da8794b, the phase)
- e992e6fc9c3cc2bed0929940f7d8c76e64eeac7906799a61e906ac9204f3a097
  gate.log (records/logs/m1p6-p1/gate.log as installed then; superseded after this check (G2, and G1 for negative.log))
- 170fb3a9ff6c5bf92fd43e22e53ba8a459f16b68a06dbbd92cfdd3a3c8f3e161
  independent-check-1.md (records/logs/m1p6-p1/independent-check-1.md as installed then)
- 24ad651da96d5b5debc606e5bd1412795895de2f210d19707146882e13233c3e
  m1p6-p1-2026-09-23-da8794b.md (records/m1p6-p1-2026-09-23-da8794b.md as installed then; superseded after this check (G2))
- 8fdbd4b0d8624c7e4ce705ea23130ad3cb84d631629821ff3bdfb6794908b4b6
  m1p6-p1-2026-09-23-da8794b.txt (records/m1p6-p1-2026-09-23-da8794b.txt as installed then; superseded after this check (G2))
- efd648ec14bcb83530a87aa71edaa8af809cb8c84a98ffafdbd1d23a814f6d61
  negative.log (records/logs/m1p6-p1/negative.log as installed then; superseded after this check (G2, and G1 for negative.log))
- 7d342433fee5d332d05b3a7627b589569e95458af7746321e6835682cc0b9fb1
  report-draft.md (the draft report, second version)
- 41448ad2c9caddb787b6e343b0f5d1a8321a922a90315f13676768653bab150d
  run.log (records/logs/m1p6-p1/run.log as installed then; superseded after this check (G2, and G1 for negative.log))
- 5380abeb357e718de401c5695a9fe0035c5a664e9b020440d4a3ba11a28ee247
  scan-for-checkers.log (the scan run before this check, PASS)
- b78992b9a69eebdf47bb65965e027f7e73fed09cc662f575448da6f354090039
  test.log (records/logs/m1p6-p1/test.log as installed then; superseded after this check (G2, and G1 for negative.log))

The answer as returned: 16755 bytes, sha256
4b11a82df73f2f9d7a604e51a66c7950d0e9a2066bedd9042d1ed334d704eee3, all ASCII, no path
substituted.

---

# Independent check 2: M1 part 6 P1 (b0c21ef..da8794b)

The harness put CLAUDE.md into my context. I did not use it as evidence.

**How I checked the inputs:**
- I read every file in `<check>`.
- Using read-only git in `<repo>`, I confirmed these diffs are byte-identical to git's output:
  - `diff-phase.patch` equals `git diff b0c21ef da8794b`.
  - `diff-code-combined.patch` equals `git diff 81991c3 da8794b`.
  - Each `diff-<id>.patch` equals `git show --format=fuller <id>`.
- `specs/0001` and `specs/0013` at da8794b are byte-identical to the check copies.
- The sha256 of each log, `independent-check-1.md` and the record pair in `<check>` equals the file in `<repo>` (untracked).
- The answer hash in check 1's header (abfb2c6a...) reproduces from the text after the `---` line, without the final newline.
- I also ran:
  - `git status --porcelain`: only the log directory and the record pair are untracked.
  - `git branch -vv`: main is 6 ahead of origin/main.
  - `git show da8794b:` for `package.rs` and `package/tests.rs`, to read `drop_calc_chain`, `validate` and the test helpers.
- I ran no cargo. I edited nothing.

## Q1. Does the diff do what P1 and its rulings say, and nothing else?

**Yes, with low-severity notes (G5, G6, G9).**

Which diff I read for each part:
- **`diff-77c3016.patch`**: all non-test code.
- **`diff-0c8459b.patch`**: tests only, 6 added lines inside `a_workbook_with_two_calc_prs_is_refused_at_write_with_no_output`.
- **`diff-da8794b.patch`**: tests only. It changes one comment and adds a chained case to `recalculate_on_open_then_a_written_formula_writes_the_attribute_once`.
- **`diff-0de2507.patch` and `diff-81991c3.patch`**: planning docs only. gate.log 1076-1122 shows each blob equals "the hash planning's handoff gave".

Line numbers below are in `diff-code-combined.patch` (= combined).

**`Value::Formula`** (0013 lines 99-115; ruling 2 at 449-455):
- It writes `<f>` via `partial_escape`, with no `t`, `v`, `ca`, `ref` or `si` (combined 61-66).
- It has the three refusals with the spec's exact messages (combined 45-53).
- The escaping test asserts that quotes stay literal (combined 840-870).

**Formula-change rule** (0013 lines 126-138; ruling 4 at 464-474):
- `writes_formula` reads the value edits per reference (combined 163-169).
- `changed_formula` is seeded from it and OR'd with the removal flag (combined 266-281).
- `formula_change_rule` drops the chain once, sets the flag and validates again (combined 326-342).

**`recalculate_on_open`**: present (combined 220-222). `is_empty` counts the flag (combined 156).

**Workbook rewrite** (rulings 1, 3 and 5):
- It runs after the styles rewrite and before the content types (combined 288-299).
- `full_calc_tag` keeps the value's own quotes, or appends the attribute after the last closing quote or the name (combined 524-545).
- Insertion goes after the last of the nine children, or after the root start tag. Whitespace and comments are held back, an empty root is opened up, and the root prefix is used (combined 404-517).

**Two `calcPr` elements** (ruling 3):
- They are counted in the read (combined 590-595) and kept privately (combined 137).
- `validate` refuses them for the flag or a written formula (combined 353-367), and the rewrite refuses again as a backstop (combined 447-451).

**Other points:**
- `WORKBOOK_REPLACED` is present (combined 379-380).
- `full_calc_on_load` accepts `1` or `true`, from the first `calcPr`.
- `--calc` is present (combined 1353-1422).
- The five catalogue entries are present (combined 90-103).

**Every P1 test case in 0013 lines 321-350, plus ruling 1's extra cases (446-448), has a matching test:**
- combined 758-1267 (package tests);
- combined 1552-1608 (the tool);
- ruling 4's updated tests at combined 637-700 and 1486-1518.

The F5 gap is closed: the chained case is at combined 1007-1013.

**Nothing extra.** `styles.rs` only gains crate visibility and `name_end`, which `full_calc_tag` needs. `dates/tests.rs` gains the one field the new struct member forces. The tool's doc comment gains a note for `--edit-formula-sample`. The Limits prose is left for P3, as 0013 lines 367-370 say.

## Q2 (including Q4). Do the logs contain every line the spec and 0001 require?

**No, narrowly.** Nearly everything the question lists is present. Two gaps remain: G2 (the header's first lines) and possibly G1 (a reach case).

**What is present:**
- **Both targets**:
  - test.log 1916 and 2451 (489 passed, 0 failed, 2 ignored each);
  - test.log 2564-2618 (each added test by name on both targets), and 3683-3686;
  - run.log 1282-1521 (host and i686, with the two labels agreeing at 1522-1525).
- **Invocation and script printed**, each script starting `#!/usr/bin/env bash`:
  - invocation on line 2 of gate, test, negative, run and scan;
  - scripts at gate 35, test 32, negative 51, run 80, scan 66.
- **Phase commit given, compared with HEAD, with controls**:
  - gate 15-21, run 36-43, scan 3-8;
  - test 26-29, negative 43-46.
- **Mutations M1-M8**: each prints the planted diff, the run and the failing assertion's values, plus a clean-worktree control and a table (test.log 3191-3680). M5 bites (3504-3539).
- **Placeholders**: no absolute machine path in any log (my grep found only concatenated fixture pieces and a path regex). There are 0 control bytes and 0 non-ASCII bytes in all logs and the record pair.
- **Commit ids**: every id is read from git by a printed command:
  - gate 1022-1036;
  - test 2520-2521 and 3172-3181;
  - negative 23-30;
  - run 23-35;
  - scan 9-25.

  The only typed ids are the full sha given as the argument, as the rule requires. No short id is typed in any script or rerun note (my grep).
- **Shape greps over each script before its log**, each with a positive control:
  - gate 4-14, test 4-14, negative 4-19, run 4-19;
  - scan 53-63, plus its pre-P6d self-scan at 44-51.
- **Commit messages' counts**:
  - C1's "Two", "five" and "nine" are derived and printed (test.log 3746-3778).
  - Every count word in C1-C3 is accounted for (3780-3817).
  - C2 and C3 claims are checked at their commits (3718-3744, 3819-3847).
- **Negative log's reach cases**:
  - N70-N75 are usage refusals; N76 is the accepted form (3957-4001).
  - K1-K4 are reads; K5 and K6 are the reading refusals; K7a and K7b read the flag the rule wrote (4003-4060).
  - K8 raises "a workbook has two calcPr elements" through `--edit-formula-sample` (4062-4071).
  - Q names four messages as unreached, beside their catalogue tests (4648-4704).
- **Gate files**: listed, with no self-test due (gate 1186-1201).
- **Doc blobs**: labelled "the hash planning's handoff gave" (gate 1076-1122).
- **Corpus run on both targets, and the record pair**:
  - chain 9 and full_calc 0 on each target, 1659 chain cells, 27 files identical (run 1375-1520);
  - the pair's sha256 (run 1541-1542) equals the `<check>` and `<repo>` copies.

**What is missing or wrong:**
- **G2 (about-line placement).** In gate, test, negative and run, the "about:" line (the phase commit and HEAD) now sits at lines 15, 15, 33 and 36. The shape-grep block was put in front of it, and in negative and run a commits-by-role block as well. 0001 lines 43-46 ask that the first lines be the command and then its result, naming the commit and HEAD. Line 1 is still "command: git rev-parse HEAD", but its result no longer follows it. Part 5 P3's logs had "about:" on line 3. The scan log still does.
- **G1 (possible missing reach case)**: see below.

### Q4. Check 1's findings F1 to F11

- **F1: resolved.**
  - negative.log's header (script lines 55-60) now says "one of which a P1 binary reaches (K8 ...) and four of which none does".
  - Header change 9 names F1.
  - The body (4695-4700) agrees. See G1 on the reasoning for the fifth message.
- **F2: resolved.** Report lines 20-26 name all five tests changed under ruling 4, including the rename. They match C1's message and test.log 3688-3697.
- **F3: resolved.** test.log's header change 4 now describes three code commits. The body at 3172-3188 checks C3 = the phase commit, C2 = C3^ and C1 = C2^. No "single code commit" claim remains outside the rerun note.
- **F4: resolved as a check, with a side effect.** Every log now greps its own script, with controls, before the body runs. The side effect is G2.
- **F5: resolved.**
  - da8794b adds the chained case (`diff-da8794b.patch`).
  - test.log 3811 shows `chain_with_formula()` is absent at C2 and present at C3.
  - The test passes on both targets (2604-2605).
- **F6: resolved for logs and scripts.**
  - Ids are read by printed commands, and rerun notes name attempts by role, file name and sha256 (gate 23-27, test 17-20, negative 36-41, run 45-58).
  - Residual: independent-check-1.md's own header types ids (G7).
- **F7: resolved.** The comment now names C1 (`diff-da8794b.patch`). test.log 3847 checks that the cell is the one set to a number last.
- **F8: resolved.** test.log 3746-3778 derives each count word in C1's message from the source at C1, with controls.
- **F9: resolved.** Report disclosed decision 1, last sentence (report line 69), states that run.log builds its own binaries.
- **F10: resolved.** run.log's TIMEOUT paragraph (66-70) no longer carries part 2 timings or sizes. A grep for "28 seconds", "about 30", "63745347" and "seconds" finds nothing.
- **F11: not resolved, and unchanged by design.** The only evidence that M5 did not bite at 77c3016 is still the uncommitted `test-attempt1.log`, cited by sha256 at test.log 18. The committed M5 block shows only the converse, at C3. This follows the superseded-attempt rules. See G4.

## Q3. Does every number and quoted string in report-draft.md appear in a primary?

**No.** These I could not find, or found wrong:

1. **Line 6, "Nothing pushed":** no log prints any remote or push state. I confirmed it only with `git branch -vv` (ahead 6). "b0c21ef stays local" is in the specs: close-out ruling 2 (`diff-0de2507.patch` line 106) and 0013 line 311. This is carried over from check 1's Q3 item 1.
2. **Line 50, "--calc usage refusals N70 to N76":** N76 is the accepted form (negative.log 3995-4001, and header "beside N76, its accepted form"). The refusals are N70-N75.
3. **Line 61, "Every log prints, first, the three shape greps over its own script":**
   - In the scan log the shape greps come after the about line, the commits block and the pre-P6d self-scan (scan 53-63).
   - In the other four logs they follow the command and invocation lines.
   - "First" is true of none of the five, strictly.
4. **Lines 66-67, "forks of this session", "The first round ran in parallel; its superseded logs say so":**
   - "Forks" appears in no primary. The logs say "the coordinator's arrangement" (gate 28, test 22).
   - The superseded logs are not among the inputs; only their hashes are.
   - Partial support: negative header change 9, attempt 4 (the corpus run was in flight), and run.log header change 2 (attempts 2-4 ran while other logs were being produced).
5. **Lines 74-75, "Three negative-log attempts failed only on the main tree's status":** contradicted by negative.log header change 9.
   - Attempt 1 failed three checks on script faults as well as the status.
   - Only attempts 2 and 3 failed on the status alone.
   - "While the forks ran" is not in any primary.
6. **Lines 76-77, "My first prompt for the negative log said ...":** no primary holds the prompt. The remaining clause ("spec line 184") checks out.

**Everything else checks out:**
- **Commit ids and roles**: gate 1026-1036.
- **"nine gate steps PASS"**: gate 1368-1382.
- **Test counts, 489 / 0 / 2 / 463**: test 1916, 2451, 2482.
- **M1-M8**: test 3673-3681.
- **"two new messages, five catalogued, nine children"**: test 3752, 3760, 3773.
- **The M5 explanation**: C2's message and test 3530-3535.
- **The quoted "a workbook has two calcPr elements"**: negative 4066.
- **Corpus figures (27 / ok 27 / lo 27 / lo_in 27 / files_identical 27, chain 9, full_calc 0, 1659, verdict rows 0)**: run 1316-1318, 1375-1377, 1491-1497, 1559.
- **Run directory `2026-09-23-da8794b`**: run 35.
- **Record pair path**: run 1539 plus the installed files.
- **"spec line 184"**: 0013 line 184, "and, where a binary can reach it, a negative-log case:".
- **Opus 5.5 only, with the Opus 5 control**: scan 1243-1244; ruling 1 at `diff-0de2507.patch` 97-105.
- **Decision 6**: test 3805-3806.
- **Decision 7**: 0013 367-370.
- **The five test names**: C1's message and test 3690-3696.

## Findings (most serious first)

**G1 (medium, found by reading the code; I ran nothing). Q's claim that no P1 binary reaches "the workbook part cannot be edited in a package that replaces or removes it" may be wrong.**
- negative.log's reason is that no binary names `Value::Formula` or `recalculate_on_open`, and that `--replace` takes no other operation (header lines 44-47; body 4692-4704). That misses the path K8 itself uses: `--edit-formula-sample` removes a normal formula, which fires the rule.
- In `<repo>` at da8794b, `drop_calc_chain` (`package.rs` 1193-1226) removes whatever part the calcChain relationship targets.
- Consider a generated package whose calcChain relationship targets `workbook.xml`:
  - The rule would record `xl/workbook.xml` for removal and set the flag.
  - Nothing earlier in `validate` (1629-1776) appears to refuse that set.
  - The new block (1777-1790) would then return `WORKBOOK_REPLACED`.
- If so, 0013 line 184 ("where a binary can reach it, a negative-log case") calls for a case. The "not reached" line at negative 4700 would be unsound. P3's negative log repeats this list (0013 376-377), so this should be settled before then.

**G2 (medium). The fix for check 1's F4 pushed the header's "about:" line down, against 0001 lines 43-46.**
- gate 15, test 15, negative 33 and run 36 now carry the commit and HEAD.
- Line 1's "command: git rev-parse HEAD" is not followed by its result.
- Part 5 P3's logs, and this phase's scan, have "about:" on line 3.

**G3 (low-medium). The draft report has the Q3 errors above.**
- Decision 3 (three negative attempts "failed only" on status) is contradicted by negative.log's header change 9.
- "N70 to N76" as refusals is inaccurate.
- "Every log prints, first" is inaccurate.
- "Forks", "its superseded logs say so" and "Nothing pushed" have no primary.

**G4 (low). F11 stands.**
- C2's message and the report say M5 did not bite at C1.
- The only record of that run is an uncommitted superseded attempt, cited by hash (test.log 18).
- 0001 lines 195-201 govern claims that a test bites, not this converse, so this is disclosure rather than a rule breach.

**G5 (low). `the_callers_drop_then_a_written_formula_records_the_recipe_once` asserts "once" at the wrong point.**
- It checks the removals (1,1,1) after a direct call to the private `formula_change_rule` (combined 985-994).
- It does not check them after `write_to`'s own second call of the rule.
- That second pass is covered only by `assert_chain_gone` on the output.
- 0013's case "`drop_calc_chain` then a written formula (the recipe once)" is met, but indirectly.

**G6 (low). Ruling 3 says validation refuses "before any temporary exists".**
- For a rule fired by a removed formula, the refusal comes from the rule's re-validation, after the worksheet temporaries exist. The code comment says so (combined 348-352), and K8 shows no temporary is left.
- Ruling 4 puts the refusal there, so the code follows the rulings read together.
- The report does not mention this nuance.

**G7 (low). `independent-check-1.md`'s header types commit ids with no printed command.**
- Examples: "Commit checked: 0c8459b5...", "Range read: b0c21ef..0c8459b", "written at da8794b".
- 0001 lines 183-184 cover "a log or check answer".
- Part 5 P3's check header did the same, so this may be accepted practice. Planning should say.

**G8 (low). `<scratch>` means a different directory in each log, so the "own target directory" claims cannot be compared.**
- gate 28 and test 22 each say "its own target directory, <scratch>/target".
- negative.log line 2 gives the gate's build directory as `<scratch>/p1`, but line 2971 calls it `<scratch>/target/`.
- Report decision 1's "own cargo target directories" is therefore not provable from the primaries.

**G9 (low). C1's message says "Two new messages", but P1 adds five.**
- The three formula-text messages are raised in `cells.rs`.
- test.log 3747-3752 reads the word as the error constants C1 adds to `package.rs` (2), which is consistent with the sentence's context.
- A reader could still take it as the phase's total.

**No privacy finding.** Workbooks appear by id only. Assertion outputs in test.log show fictional fixture bytes only. I found no corpus file name or cell content in any input.
