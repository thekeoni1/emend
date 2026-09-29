# Independent check 1, M2 part 4b P2

Written by the implementing session: the checked phase commit is
20d7d934a3094545db35f41ad5d09ec943e9053a, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk (its prompt named it
<phase>/check1 and asked for <home> and <repo> in place of machine paths),
holding these files, by sha256. The logs there are gate.log, test.log,
negative.log and run.log, each at attempt 1 as committed, and the scan's
pre-check stage, which the final scan.log's rerun lines hash:

    96515fd43179a0860f2c7c3b0dca2a9bc4d3975d9748d75bfb6adf8bf33f2eaa  commit-messages.txt
    7336e2f010ee35eac7ce934c8f8208bbb43077b09d21bf27342d1e9d2a665f89  diff-08729d3.diff
    0a71a4075cd9c136d69ab72b8771912719724edc333768ff3982be573fac588f  diff-20d7d93.diff
    e3ef48074a0823947f6f86cdf8a07911e12f317e9963627ab1c3b0745a93d055  diff-5ad72b5.diff
    851026049df93ca4c09eeed1edbe95d51de16702b6b0432192f843800bd835d2  diff-fa3ad04.diff
    ad3bdce66ca6f2dfe5c15b4dc7b7f8b33bc8edd7f1da0ceefa049284eed8764f  diff-phase-157c11c..20d7d93.diff
    2a2838b4ce3c4c37f7dc06252820be13359e9831ca834293d259583157236b87  doc-commit-609eb7f.stat
    63e3fe0211eccc6bf3f300f75d20e9ee5887e26492bd5031209d663883b337c7  gate.log
    5d2654a03ce141c2a85edba9067736b4602c25997e7875bf27d1fe131441cbaa  m2p4b-open-2026-09-29-20d7d93.md
    9289dffe3c5e86dae740e61741ab55c48906f70a6506648921592194bb7c7b5d  m2p4b-open-2026-09-29-20d7d93.txt
    5ddab8aaf2bb0103f47ab1c8af45646390eae3465b9fbbdf32217929caaff276  m2p4b-p2-2026-09-29-20d7d93.md
    d20f58852fd7be59134e03f315e0ad677851722c97b93630d79b41312f8c05ab  m2p4b-p2-2026-09-29-20d7d93.txt
    05dc49ff82a4b4d809a3ab6cd5f2ada7f9894bfd3b32a0051ecad172ed422c24  negative.log
    1b54edfd878cff9ff9057b4c49f1a3cbbe96b654397a31ec249aefac37af876b  report-draft.md
    1abf57209d92f67fd45364facec526260de965afc6c8b365450e4ae0322ea3e0  run.log
    be190acd40de2422515da30c955f54957c93b5230da0c93c91a18e09c5bc7eed  scan-pre-check.log
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  spec-0001-bootstrap.md
    95daff46e164f3552df4aff671d800a31347fde40319fc1667bc9aba75900d8d  spec-0020-committed.md
    09643f944ad03dbe28632f09c83ee8514355a2b76f15212b5e2f074abc551600  spec-0020-working-tree-with-P2-rulings.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/test.stderr
    35a98cc9aedf81cf69fb4369dfdba7824891c98a6bc14e99ffc74cf00dec697f  test.log

The per-commit diffs and the other derived inputs were made by these
commands, in the repository (specs/0020 P1 verification ruling 6):

    git diff 5ad72b5^ 5ad72b5 > diff-5ad72b5.diff
    git diff fa3ad04^ fa3ad04 > diff-fa3ad04.diff
    git diff 08729d3^ 08729d3 > diff-08729d3.diff
    git diff 20d7d93^ 20d7d93 > diff-20d7d93.diff
    git diff 157c11c 20d7d93 > diff-phase-157c11c..20d7d93.diff
    git diff 157c11c 609eb7f --stat > doc-commit-609eb7f.stat

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 7f38abfa3b920dc565a94ebf7023f9a318c445dcf09cadeeb080f09230f3f9a7,
10680 bytes; plain ASCII; no machine path; nothing replaced; one line feed
added at its end).

---

I read only the files in <phase>/check1. The harness also put a CLAUDE.md, plus memory and git-status context, into my context. I did not use any of it as evidence. I have not printed or guessed any workbook content.

## Q1. Does the diff do what the spec (with the rulings) says for P2, and nothing else?

**Diff provenance.** Each per-commit diff file hashes to the value gate.log prints for `git show --no-color --no-ext-diff --format= <commit>`, and the combined diff hashes to its `git diff <base> <phase commit>` line. The five values are e3ef4807..., 85102604..., 7336e2f0..., 0a71a407... and ad3bdce6.... So each diff is its commit against its parent, as P1 verification ruling 6 asks. The files themselves carry no header line naming the command; only gate.log does.

1. **Code commit 1 against P1 verification ruling 5** (diff-5ad72b5.diff; confirmed as tests only). It touches only package/tests/validation.rs, and gate.log confirms "paths: 1" and "lines added 65; lines removed 1". It is the doc commit's child.
   - The call-time test gains four cases: `formula2: None` under the `whole()` operator; `NotBetween` with `None`; `Some("")`; and `Custom` with `Operator::Equal`, which gets "does not apply to its kind".
   - One new test covers a pending sheet (add_sheet, then add_validation and remove_validation, then remove_sheet) and asserts output == input. This matches P2 ruling 1. Its `remove_validation(B2)` would otherwise be refused, so the test can fail.
   - Cannot confirm: that `whole()` is `Between`. Its body is not in any diff; the comment says so, and test.log's printed validation output shows `operator="between"`.
   - No plant shows that the pending-sheet test fails if the `retain` is removed. The spec does not require one.

2. **Differential styles** (diff-fa3ad04.diff; confirmed nothing else).
   - Every addition is under `#[cfg(feature = "rules")]`. With the feature off, the rewrite condition reduces to the old `!xf_additions.is_empty()` (`let dxfs_added = false`). gate.log shows the off-list is "the base's with the feature off, line for line".
   - The index is `dxf_children()? + dxf_additions.len()`, counting children, per P2 ruling 3.
   - A call with a number format, an alignment, a protection or no field is refused (`DXF_FIELD`).
   - Growth, the empty-container open, and creation before `tableStyles|colors|extLst` or the root end (`DxfsPass::BEFORE`) match "Differential styles". Two `<dxfs>` are refused at the call (`read_dxfs` counts containers) and at the write (`open` when `met`).
   - Extra beyond the spec's wording: "a differential style index is out of range" is also raised at the call for a `u32` overflow. The spec lists that message only at the write. It is harmless, and the catalogue states it.

3. **Conditional formatting** (diff-08729d3.diff; confirmed nothing else outside the feature).
   - Attribute order is type, dxfId, priority, stopIfTrue, operator, rank, percent, bottom, aboveAverage, equalAverage, as "The write" specifies.
   - The blank rules take `LEN(TRIM(<first area's first cell>))=0` or `>0`.
   - At the call: no `dxf` gives `RULE_STYLE`; priority 0 and rank outside 1 to 1000 give `INDEX_OUT_OF_RANGE`; the formula rules match "Areas and formulas".
   - `FormatsPass` holds one element, skips a matched removal, refuses an `extLst` (depth+2) and an unparsed kept priority or sqref, and counts main-namespace `cfRule` priorities of kept elements only (ruling 5). `read_rules`/`read_rule` implement rulings 4 and 6 (`TEXT_OPERATORS`).
   - The CF-before-DV ordering matches "Creating an element": `formats.create_before` and `create_at_end` are called before `rules.`.
   - Findings:
     - (a) The parse is stricter than the spec's words. `crate::cells::decimal` refuses `priority="01"` (the priorities test asserts this). The spec says "does not parse as a `u32`", which "01" does in Rust. It matches ruling 4 only for `dxfId` ("decimal u32").
     - (b) An untested edge. On a part not in schema order, a kept `conditionalFormatting` after the insertion point is not in `highest` when the additions are written. Ruling 5 frames the rule "on a part in schema order", so this is not a failure.
     - (c) `DXF_OUT_OF_RANGE` at the write calls `dxf_children()`. On a workbook with no styles part it therefore gives "the workbook has no styles part", not the range message.

4. **Tool, `--cf-sample`** (diff-20d7d93.diff; confirmed tool-only).
   - The constants match "Harness runs": `Rgb(0x9C0006)`, `background Rgb(0xFFC7CE)`, CellIs GreaterThan "5" on (M+1,1):(M+1,3), Expression "TRUE" with stop on (M+2,1), no priority.
   - The four assertions match, the output is removed on a failure, and the removal runs through a Cursor.
   - Cannot confirm: that `dxf_counts` uses quick-xml, as the spec says. It uses `Stream`, whose origin is not in the diff.

5. **P2 test list, item by item** (conditional.rs, dxf.rs, null_rewrite.rs).
   - dxf appended, with kept bytes and count: yes.
   - Four creation fixtures, printed: yes (test.log "dxfs created before ..." lines).
   - Two `dxfs` refused: yes.
   - The four call refusals (number format, alignment, protection, nothing): yes.
   - Index = children + earlier additions: yes. One past it refused at the write: yes (`Some(2)` with none added, `Some(3)` with one).
   - Eight variants against literal bytes, blank formula from the first area: yes.
   - No style refused: yes. The visual-rule case moves to P3 (ruling 2).
   - The four priority cases: yes.
   - Placement after the kept rules, and before dataValidations, hyperlinks and the root end: yes.
   - Removal present, absent, extLst: yes.
   - containsText and extended read-back: yes.
   - Shift and replaced part: yes.
   - Tool line: yes.
   - Weaker than their names:
     - `a_cell_style_and_a_dxf_in_one_write_both_go_in` uses `contains` only.
     - `..._below_it_taken` asserts only `contains("dxfId=\"2\"")`.
     - "The part unchanged" for the extLst refusal rests on the refusal alone.
     - `a_conditional_format_edit_with_a_row_insertion_is_refused` is `rowcol`-gated. It never runs in the crate-level `--features rules` runs (28 rules-module tests there, 30 in the workspace). It runs only in the workspace run (test.log line 1681).

## Q2. Do the logs contain every line the spec requires?

6. **Spec pin: fine.** gate, test, negative and run pin 95daff46 and print the working copy 09643f94 as "not the pin". The scan pre-check counts "4 of 4".

7. **Commit ids: fine.** They are read from git by printed commands. The phase commit is compared with HEAD, with null or short-sha controls, in all five logs.

8. **Standard error: fine.** It is counted with a one-byte control in all five logs, and stderr/*.stderr are all 0 bytes.

9. **Shapes, placeholders and control bytes: fine.** The three shape greps run over each script with fixtures. The scan pre-check finds 0 over 9 files, declares every placeholder, and counts control bytes with positive controls.

10. **Gate: fine.** Feature off and on, both targets: 512/540. The pub lists read "off 134, on 141", with the base on at 137. The consumer shows 14 errors off and 0 on. Clippy and rustdoc are clean off, on and with every feature. No gate file changed, so no self-test was due.

11. **Test log.**
    - Ruling 5's plant W1 is present, with an unplanted pass and a planted failure.
    - Cannot confirm: that the plant text `.any(|before| lists_overlap(before, areas))` sits in the `added[..at]` loop. The P1 code is not in these diffs.
    - Plants W2 to W10 each fail their named test. There is no plant for:
      - `DXF_OUT_OF_RANGE` at the write;
      - `RULE_STYLE`;
      - the write's unparsed-priority or sqref refusal;
      - `NO_SUCH_RULE`;
      - the blank formula;
      - the pending-sheet `retain`.

      The spec requires none of these, and the log makes no claim for them.

12. **Negative log: fine.**
    - It covers both required cases (C1 extLst on A2, C3 `priority="high"`), each with a control (C2, C4, C0), plus C5.
    - It shows `RULE_UNPARSED committed=no working=yes`, the state P2 ruling 4 predicts.
    - Duplicated lines: "C1: exit 2..." and "C3: exit 2..." each print twice, once per control. These are the same check repeated.

13. **Run log.**
    - It covers both targets, 27 of 27 ok, the 14 flags each turned by a plant (14 of 14), W and the styles part changed (54), everything else identical (816), and host and i686 outputs identical.
    - The keep and the edit open check are present (532 pairs; 0 outputs refused on an accepted input).
    - Ruling 1 is followed: totals are marked "printed, not checked".
    - Adversarial points:
      - `dxfs_place`'s plant applies only when a `dxfs` was created. The existing-container branch (a root-order comparison) has no plant.
      - "the same styles parts are recounted on both targets: yes" compares two empty lists, and its listed control is about refusals, not recounts.
      - "every line is what the edits imply" re-checks what the tool already asserts before printing. This is implied by exit 0, and so is close to a duplicate.
      - `styles_rest`'s recount fallback is exercised only by `--self-test`, never by a plant on a real output.

14. **Scan: fine.** Only the pre-check stage is present, which is expected. It shows RESULT: PASS, total hits 0, and check answers 0 at pre-check.

## Q3. Does every number and quoted string in report-draft.md appear in a primary?

These all appear in a primary:
- The hashes 95daff46... and 09643f94....
- The commit ids and roles.
- "134 ... 141 ... 137".
- "799, failed 0, ignored 2" on both targets.
- W1 to W10, "each found once".
- 27 summary rows, 27 ok, 0 refused.
- The totals 20/74, 1360/1387 and 11, and the recount count 0.
- The four "... checks all as expected: yes" lines.
- RESULT: PASS with 0 hits.
- "009C0006" and "00FFC7CE" (null_rewrite.rs, cfcmp.py).
- The quoted messages, and `committed=no`.
- The empty `cfRule` for the four kinds (conditional.rs literals).
- The record names.

These could not be found:

15. "each at attempt 1" (report line 22). No log prints an attempt number; they show only `RERUN_NOTE: unset`.
16. "the pre-check stage ran once" (line 37). There is no counter in scan-pre-check.log.
17. "nothing pushed" (line 9). No primary in the directory states it.
18. "as openpyxl writes a six-digit colour" (line 43). Nothing in the primaries supports this. The commit message and tool doc say only "openpyxl's light red fill with dark red text".
19. "a fifth fixture" (line 45). negative.log says "the six fixtures were written" (c0 to c5), which makes C5 the sixth file.

Everything else in the draft traces to a primary.
