# Independent check 1, M2 part 4b P3

Written by the implementing session: the checked phase commit is
c2fe42b7c554b37b4561b191b178eb4b7ee6c2dc, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk (its prompt named it
<phase>/check1 and asked for <home> and <repo> in place of machine paths),
holding these files, by sha256. Its answer's item 3 under question 1 led to
planning's P3 ruling 2 and a new code commit,
631c719615f1503448d489f5ca37f7412af190f5, now the phase commit; the logs,
records and pre-check stage this check read were written at c2fe42b, are
superseded by that commit, and are kept on the disk, each hashed by the
rerun lines of its successor in this directory:

    f29cb5127b34d1613dd7830ae51baa5567e8278d2df90c0dbe664f4f2844e583  commit-messages.txt
    576fff2d64ed7ed06cad0e74aa6fce61c8b9b1875546f493f747317d12010b75  diff-03bbe82.diff
    26934aa733177db22f102413ea00c603483e43d96025619ed3a959fc281a663f  diff-1265000.diff
    11d660246c8ce7b1fdb95209f0eaa5fc02bd91d9ae56184c1ff17074699f0b6f  diff-c2fe42b.diff
    1cbf8fc554a53f89b395d94926641b143edc801343f46ee4c340600cbd785dcd  diff-phase-0999a35..c2fe42b.diff
    351bcd2634937a58ea427ce7b25302fdd637f174b0b118eef912bbc7153d760a  doc-commit-b88f686.stat
    097c7566fa685415a6c3ce13b63d41aaa7a3d946b46588a9886e7f052382d7b8  gate.log
    48b00c1319e72400667564baa4de04c4890635c8244ad78a976fefa222b66da0  m2p4b-open-2026-09-29-c2fe42b.md
    599e2d136bb48ad9c614f21f3bad9ab69808a74844bf7b6b77096f5a0ec32bfc  m2p4b-open-2026-09-29-c2fe42b.txt
    2b72620988c8e14429ecf01c803d6651ebd0ff9f64b6bc47f8d1768d3a7bfac5  m2p4b-p3-2026-09-29-c2fe42b.md
    8a9ac79754a5ae724a98ca6d299f0035e6b78ee2445595ab9c63cb8d45833e9c  m2p4b-p3-2026-09-29-c2fe42b.txt
    b9bff6884dac17a545c3cba7a50d62c40dcb306949869faf7ae9725bcd673668  negative.log
    941efb765e4e6784aa640ea2360ca1c4aec2d1bdf9dcc1d8d8df89637eed02ec  report-draft.md
    7be2efa70a1d249dc320d3f05537edb42d5d6fce51ffda0491a68683cca39a5c  run.log
    23c7d955b22aaa309b78137d7f82fc412fc12909437027597ff590b950bfde95  scan-pre-check.log
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  spec-0001-bootstrap.md
    c98838a0433cb7347dbe61e3af3dfabecc8ad0f00fba2661760553da96cd0188  spec-0020-committed.md
    d5928edb509ccb60f1c216d330f6ee93d0bd67324a6361a1e1045461b205b917  spec-0020-working-tree-with-P3-rulings.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/test.stderr
    f12314c6eae71e99f3069ccc9d95f970435683580239852dce6e1d39c76dc633  test.log

The per-commit diffs and the other derived inputs were made by these
commands, in the repository (specs/0020 P1 verification ruling 6):

    git diff 03bbe82^ 03bbe82 > diff-03bbe82.diff
    git diff 1265000^ 1265000 > diff-1265000.diff
    git diff c2fe42b^ c2fe42b > diff-c2fe42b.diff
    git diff 0999a35 c2fe42b > diff-phase-0999a35..c2fe42b.diff
    git diff 0999a35 b88f686 --stat > doc-commit-b88f686.stat

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 9533b8d46842e79c8e87c029e94a0e51fdc101236c63d1996898890f7467ced8,
11043 bytes; plain ASCII; no machine path; nothing replaced; one line feed
added at its end).

---

The phase's code does what P3 asks, and all four logs end all as expected. There is one departure from the "Errors" section for planning to rule on (Q1 item 3). Every figure in the report draft appears in a primary.

The harness also put the repository's CLAUDE.md into my context. I did not use it as evidence. Everything below comes only from files in <phase>/check1. I printed no workbook content: corpus rows appear only as ids and counts.

**1. Does the diff do what the spec says for P3, and nothing else?**

Which diff I read for each "nothing else":
- **Code commit 1:** diff-03bbe82.diff.
- **Code commit 2:** diff-1265000.diff.
- **Code commit 3:** diff-c2fe42b.diff.
- **Doc commit:** diff-phase-0999a35..c2fe42b.diff for ROADMAP.md and the spec, with doc-commit-b88f686.stat. gate.log also confirms both blobs match the hashes planning sent (4c6bc56e..., c98838a0...).
- **The whole phase:** the combined diff lists 11 files, which are exactly the per-commit files plus those two docs.

1. **Code commit 1 against P2 verification ruling 6: fine.** diff-03bbe82.diff only adds the test `a_dxf_only_write_recounts_a_fonts_count_that_disagrees_with_its_children` to dxf.rs (16 lines added, 0 removed). It sets fonts `count="5"` on one child and expects the output to be BASE (`count="1"`) with the dxf appended. gate.log confirms it touches one path, removes no line, and that 03bbe82 is the doc commit's child.

2. **P3 test list, item by item:**
   - **Each visual kind against literal bytes: fine.**
   - **Every `CfvoKind` once: fine.** Min, Max, Num(-1.5), Percentile(50), Formula (in the colour scale) and Percent(90) (in the data bar) are all covered.
   - **Every `Color` form once: fine, with one gap.** Rgb, Theme with a tint, Indexed and Auto are covered. Theme with no tint is not.
   - **`showValue` shown and hidden, `reverse`, and three-, four- and five-icon sets: fine.** All 17 set names are checked, but only with `contains`.
   - **The value refusals: fine.** Min as a max, a percent of 101, an infinite number, an empty formula, four thresholds for a three-icon set, and five for a four-icon set (P3 ruling 1) are all present.
   - **A visual rule with a style refused: fine.**
   - **The read-back: fine.** It checks the kind, area, priority, empty formulas, no dxf, no operator and not extended.
   - **The tool's line: fine.** It is checked with no rule and after a kept, extended data bar, plus the refused-priority case.

3. **Departure from "Errors": equals sign and XML characters in a cfvo formula.** diff-1265000.diff (`cfvo_parts`) maps `rule_formula_ok(text).map_err(|_| VISUAL_VALUE)`. So a `CfvoKind::Formula` with a leading `=`, or with a character XML forbids, is refused with "a colour scale, data bar or icon set value is not valid". The spec's "Errors" reuse list says part 4's "a formula text starts with an equals sign" applies "at every formula's" call, and "a text holds a character XML cannot carry" "at every formula's and message's call" (P1 ruling 4). The CfvoKind bullet names only "a non-empty formula" under the value message. visual.rs asserts the value message for `Formula("=A1")`, so the test pins the departure. The commit message states it; the report draft does not. Planning should rule. There is also no test for an XML-forbidden character in a cfvo formula.

4. **Beyond the spec: a theme tint outside -1 to 1.** `visual_color` refuses it with VISUAL_VALUE. The spec's value rule says nothing about colours. The code does not reuse part 2's existing `check_color`, which is visible as context in diff-1265000.diff and has its own message. Report item 3 discloses this, but it is additional behaviour and needs a ruling.

5. **Interpretations, both disclosed as report item 2.** A colour scale's middle point may not be Min or Max, and neither may an icon-set threshold. The mid-point reading follows from "first and last". Refusing Min and Max anywhere in an icon set is a stricter reading. Only `mid: Max` and `Min` in an icon set are tested; `mid: Min` and Max in an icon set are not.

6. **Child element form ("The write"): fine.** The code writes the cfvo elements, then the colours. `val` is left out for min and max. `showValue="0"` and `reverse="1"` appear only when set. Attribute order is iconSet, reverse, showValue. There is no dxfId, and there is no formula for a visual rule.
   - **Gap:** "every element taking the root's prefix" has no test on a prefixed root for the visual children. `VisualChild::element` builds the colour name through `str::from_utf8(prefix).unwrap_or_default()`, which would silently drop the prefix on colours while keeping it on cfvo. It is harmless for UTF-8 prefixes, but no test exercises it.
   - Nothing tests Min or Max placement for `DataBar` specifically; it uses the same function as the colour scale.

7. **"Crate": fine.** The three types are behind `rules` with Clone/Copy/Debug/PartialEq (plus Eq on IconSetKind). The consumer in gate.log names 17 gated items, each refused with the feature off.
   - The lib.rs hunk also swaps the order of the `metadata` and `rules` `pub use` blocks. That looks like formatter ordering of a file the commit had to touch anyway. It is not a scope problem.
   - The `color_element` helper in styles is gated. gate.log shows it adds one hunk after the cfg line and removes no line.

8. **"Harness runs" `--scale-sample`: fine.**
   - The three rules match the spec: colours `Rgb(0xF8696B)`, `Rgb(0x63BE7B)` and `Rgb(0x638EC6)`; thresholds 0, 33 and 67 percent; no style and no priority; rows M+1 to M+3 in columns 1 to 3.
   - The line prints `cf_before`, `cf_after`, `visual_after` and `extended_before`. The tool asserts cf+3 and visual+3, and removes the output on a failure.
   - The past-the-grid refusal relies on the layer through `saturating_add`. As in P1 and P2, no P3 test covers it.

9. **"Nothing else": fine.** Code commit 2 touches only its six crate files, and code commit 3 only main.rs and null_rewrite.rs. The phase changes no manifest, no lock and no gate file. With `rules` on, no pub fn or pub const was added (gate.log).

**2. Do the logs contain every line the spec requires?**

1. **Spec pin: fine.** Every log pins c98838a0 and prints the working copy (d5928edb...) as not the pin: gate.log lines 21-27, test.log 22-28, negative.log 63-64, the run.log spec section, and the scan-pre-check.log inputs. RERUN_NOTE is unset everywhere, and the stderr files are 0 bytes.

2. **gate.log: fine.**
   - Commit roles and parentage, and every message ASCII, ending with the Co-Authored-By line only.
   - The count rule: each message carries the "diff alone" sentence.
   - The gate files listed with none changed (as spec-0001 asks), the nine PASS lines of check.sh, and the pub-line lists (134 off and 141 on, at the base and at the phase commit).
   - The crate's tests on both targets with the feature off and on: 512 and 546.
   - Clippy and doc off, on and with every feature. P2 was verified with five clippy configurations; this log runs three crate configurations plus four probe-crate ones, and `metadata,rules` is not separate. The spec does not require it.

3. **test.log: fine.**
   - Both targets with the feature off and on; the workspace at 807 passed, 0 failed, 2 ignored on each target (lines 2206 and 3061). The 8 added tests are named and each passed on both targets.
   - The carry test run alone.
   - The four plants of ruling 6 (Y1 to Y4), each found once, passing unplanted and failing planted, with the failing output printed (lines 3567-3740).
   - **Weakness:** X1 to X5 all target one test whose 13 cases share one message, and the failure prints only "refused: ()" at line 288. The log shows some case was let through, not which one.
   - No plant covers the empty formula, the leading `=`, NaN, the middle point, the tint, Min in an icon set, or `visual_after` in the tool.

4. **negative.log: fine.**
   - It states that none of this phase's refusals is reachable through the binary, and lists the 7 constants it says are unreachable, each with a reason.
   - It reruns the six P2 fixtures with `--scale-sample`: C3 is refused with RULE_UNPARSED and leaves no output, and C1 and C2 (extLst) run to their lines, as the spec expects.
   - **Duplicate checks:** lines 974-975 and 977-978 are the same two C3 checks printed twice (after C4 and after C0). They are not independent evidence.

5. **run.log: fine.**
   - Both targets: 27 of 27 ok, 0 refused, every row has LibreOffice ok on input and output, and one changed entry (W) per row. The two targets' outputs are byte-identical.
   - All 8 comparison flags hold on every workbook, and each flag has a plant that turned it.
   - The corpus record pair and the open-check record are installed. The edit open check ran over the keep (29 new pairs, all ok/ok, and no verdict of P2's pairs moved).
   - **Implied checks:** `added_tags` follows from `added_bytes`, and `cf_read` largely follows from `cf_prefix` plus `added_bytes`. They are controls on the script, not extra evidence.
   - `cf_place` compares `wout[:start] == win[:at_in]`. That would fail falsely on whitespace between the last kept conditionalFormatting and the next child; no corpus input had that shape.

6. **P1 verification ruling 1: fine.** The totals are labelled "printed, not checked; the per-row checks imply them".

7. **Carried rules: fine.**
   - scan-pre-check.log ends RESULT: PASS with 0 total hits; its placeholder check covers the sibling logs, and it runs the only_spec content control.
   - The final scan.log is not in the directory, as expected at this stage.

**3. Does every number and quoted string in report-draft.md appear in a primary?**

Every figure and quoted string I checked is in a primary. None is missing.
- **Hashes:** c98838a0... and d5928edb... (gate.log lines 23 and 25).
- **Commits:** b88f686, 03bbe82, 1265000, c2fe42b and 0999a35 (gate.log).
- **gate.log:** 134 and 141, at the base and at the phase commit.
- **test.log:** 807/0/2 on both targets; X1 to X9 and Y1 to Y4.
- **run.log:** 27, 27, 0; and 20/101, 10/91, 4.
- **Colour strings:** "00F8696B", "0063BE7B" and "00638EC6" are in null_rewrite.rs in diff-c2fe42b.diff and in the scalecmp source in run.log.
- **Record stems:** in run.log.
- **scan-pre-check.log:** "RESULT: PASS" and total hits 0.
- **Quotes:** the spec's "Min and Max only as the first and last of a scale or a bar" and the commit message's "-1 to 1".
- **The four "all as expected: yes" lines:** each is the last line of its log.

Three notes on wording:
1. "the priority fixture refused at the read": no log says where the refusal happened. It follows from diff-c2fe42b.diff, where `conditional_formats` is called before any addition.
2. The code 2 bullet puts "(crates/emend/src/package/tests/visual.rs)" right after "a feature-gated colour element writer", which reads as if that path were the writer. The writer is in styles/elements.rs.
3. The departure in Q1 item 3 is not among the report's "anything that looks wrong" items. It should be added there.
