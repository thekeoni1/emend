# Independent check 2, M2 part 4b P2

Written by the implementing session: the checked phase commit is
20d7d934a3094545db35f41ad5d09ec943e9053a, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check2 in this phase's folder on the disk (its prompt named it
<phase>/check2 and asked for <home> and <repo> in place of machine paths),
holding these files, by sha256: check 1's inputs, the draft report
replaced by report-draft-2.md, and independent-check-1.md as committed.
The derived inputs were made by the commands check 1's header prints.

    96515fd43179a0860f2c7c3b0dca2a9bc4d3975d9748d75bfb6adf8bf33f2eaa  commit-messages.txt
    7336e2f010ee35eac7ce934c8f8208bbb43077b09d21bf27342d1e9d2a665f89  diff-08729d3.diff
    0a71a4075cd9c136d69ab72b8771912719724edc333768ff3982be573fac588f  diff-20d7d93.diff
    e3ef48074a0823947f6f86cdf8a07911e12f317e9963627ab1c3b0745a93d055  diff-5ad72b5.diff
    851026049df93ca4c09eeed1edbe95d51de16702b6b0432192f843800bd835d2  diff-fa3ad04.diff
    ad3bdce66ca6f2dfe5c15b4dc7b7f8b33bc8edd7f1da0ceefa049284eed8764f  diff-phase-157c11c..20d7d93.diff
    2a2838b4ce3c4c37f7dc06252820be13359e9831ca834293d259583157236b87  doc-commit-609eb7f.stat
    63e3fe0211eccc6bf3f300f75d20e9ee5887e26492bd5031209d663883b337c7  gate.log
    5c82029da431149d150473e38dbe4fa7d27e99a326ab5983b67e8832936f4063  independent-check-1.md
    5d2654a03ce141c2a85edba9067736b4602c25997e7875bf27d1fe131441cbaa  m2p4b-open-2026-09-29-20d7d93.md
    9289dffe3c5e86dae740e61741ab55c48906f70a6506648921592194bb7c7b5d  m2p4b-open-2026-09-29-20d7d93.txt
    5ddab8aaf2bb0103f47ab1c8af45646390eae3465b9fbbdf32217929caaff276  m2p4b-p2-2026-09-29-20d7d93.md
    d20f58852fd7be59134e03f315e0ad677851722c97b93630d79b41312f8c05ab  m2p4b-p2-2026-09-29-20d7d93.txt
    05dc49ff82a4b4d809a3ab6cd5f2ada7f9894bfd3b32a0051ecad172ed422c24  negative.log
    33391637d08fc88a6c1a58a3983065bac742f6796e5e324ae29c0fb655e8bf8c  report-draft-2.md
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

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 21f0ba4c337981ee5f3cdc2c72903390ea1c7e1ea4acf95fd1b59cab02c15905,
11256 bytes; plain ASCII; no machine path; nothing replaced; one line feed
added at its end). This is the second and last check the rules allow; what
it found goes to planning in the report.

---

I read only the files in <phase>/check2. The harness also put a CLAUDE.md into my context, along with memory and git-status context; I did not use any of it as evidence. I have not printed or guessed any workbook content. Below I list only what check 1 did not already cover, or where I disagree with it.

## Q1. Does the diff do what the spec (with the rulings) says for P2, and nothing else?

Which diff I read for each "nothing else":
- **diff-5ad72b5.diff**: touches only tests/validation.rs.
- **diff-fa3ad04.diff**: package.rs, tests.rs (the `mod dxf` line) and tests/dxf.rs. Its only removed lines are the `no_rule_edits` body and the styles-rewrite condition.
- **diff-08729d3.diff**: lib.rs (the `Rule` and `RuleEntry` export), package.rs, tests.rs and tests/conditional.rs. The removed lines are all feature-gated or doc text.
- **diff-20d7d93.diff**: tools/emend-copy main.rs and null_rewrite.rs only.
- **diff-phase-157c11c..20d7d93.diff**: its ROADMAP and specs hunks are the doc commit's, matching doc-commit-609eb7f.stat (2 files, +148/-15). No code commit touches specs.

Every new line in package.rs sits under `cfg(feature = "rules")`, apart from the constant `dxfs_added = false` branch that check 1 described. So I confirm "nothing else" for all four code commits.

Findings check 1 did not make:

1. **An out-of-order styles part with one `<dxfs>` is refused as "two dxfs" (diff-fa3ad04.diff).**
   - `DxfsPass::create_before` runs at every main root child in `BEFORE`. On a part whose single `<dxfs>` comes after `colors`, it creates a new container at `<colors>`. When the real one is reached, `open` refuses with TWO_DXFS: "a styles part has two dxfs elements".
   - The test `two_dxfs_are_refused_at_the_call_and_a_created_then_met_one_at_the_write` asserts exactly this (`<colors/><dxfs/>`).
   - The spec says a part that has a `<dxfs>` grows it, and ruling 3 covers only two containers. So a part with one container gets a message that is untrue of it. The call already knows `containers == 1`.
   - No output is corrupted, but this is outside the spec's words, and planning should rule on it. It is the styles-side twin of check 1's 3(b).

2. **Ruling 4's wording is narrower than the code (diff-08729d3.diff, `FormatsPass::open`).**
   - Ruling 4 says an unparsed kept `sqref` is refused "at the write where a removal matches kept elements".
   - The code parses every kept `sqref` whenever the part has any rule edit, additions only included. The test (`a_removal_drops_...`, last assertion) asserts the refusal on an addition-only write.
   - The corrected "The write" paragraph ("like a kept sqref that does not parse, refuses the write") supports the code. The ruling's parenthetical does not.
   - Planning should say which text governs.

3. **The kept-priority rule is fine.** Ruling 5 (the kept elements only; `x14` rules not counted) matches `settle`, which returns before counting a removed element. `priorities_...` asserts priority 4 once the second element is removed.

4. **The tool's count comes from the crate's parser (diff-20d7d93.diff).** `dxf_counts` runs over the crate's own `Stream` (`Stream::new(package.read_part(..))`). So the tool's `dxfs_before` and `dxfs_after` are not independent of the crate's tokenizer; only cfcmp is. The spec asks for a count "by the tool with quick-xml", which this may be, since check 1 could not confirm what `Stream` is built on.

## Q2. Do the logs contain every line the spec requires?

**cfcmp.py (run.log lines 1474-2130) against "Harness runs" P2, item by item:**

5. **Coverage of the spec's list is complete.**
   - "every root child of W other than conditionalFormatting byte-identical": `rest_same`.
   - "the input's elements a byte-identical prefix": `cf_prefix`.
   - "two more, each exactly the expected bytes, dxfId the input's dxfs count, priorities highest plus one and two": `added_bytes`. D is the count of dxf children, per ruling 3.
   - "placed after the last kept one or in schema position": `cf_place`, per ruling 5.
   - "styles part identical outside dxfs": `styles_rest`.
   - "dxf children a byte-identical prefix": `dxf_prefix`.
   - "one more exactly the expected bytes": `dxf_bytes`.
   - "count equal to the children": `dxfs_count` (and `dxfs_tag`).
   - "created before tableStyles, colors, extLst or the root's end": `dxfs_place` (created branch).
   - No spec item is missing.

6. **Checks another check implies (not raised by check 1).**
   - `dxf_prefix` is strictly implied by `dxf_bytes`. Line 627 is `flags["dxf_bytes"] = content.startswith(dxf_in) and ...`, so `dxf_prefix` can never be no while `dxf_bytes` is yes.
   - `added_tags` and `cf_read` parse bytes that `added_bytes` plus `cf_prefix` already fix exactly, so they cannot fail alone. Their plants (`planted="1"`, sqref A->B) also turn `added_bytes`.
   - `root_order` follows from `rest_same` + `cf_prefix` + `cf_place`.
   - `dxfs_count` follows from `dxfs_tag` (count D+1) + `dxf_bytes`.
   - The existing-container branch of `dxfs_place` follows from `styles_rest` + `one_dxfs`. Check 1 noted only that this branch has no plant.
   - So of the 14 flags, about 6 are controls on the script, not evidence. The "14 of 14" plant count overstates how many independent checks there are.

7. **Two flags contradict each other on one input shape (cfcmp lines 617-633).** For an input `<dxfs>` with no `count`, `dxfs_tag` expects the start tag unchanged (no count), while `dxfs_count` requires `cval is not None`. Both cannot hold, so such an input would always fail. No corpus input hit it (27/27 all yes), but the comparison encodes two expectations for one case. The spec ("`count` replaced by the children plus the additions") does not say which is right.

8. **Counts printed that no check reads.**
   - `dxfs_created` is computed from the input alone (`0 if dx_in else 1`, line 651), not seen in the output. So the report's "11 dxfs created" means inputs with no `dxfs` (P1 verification ruling 1's reading), and the report does not say so.
   - "entry totals ... flagged 34" (run.log, host and i686) is read by no check. `entries_cf` checks flags per row.

**Test log plants against the code (test.log 3605-3745; plant texts at 1036-1052):**

9. **Where each plant lands.** Every P2 plant text is in the phase diff exactly once:
   - W2 at line 782 (`DxfsPass::count`)
   - W3 at 766
   - W4 at 560
   - W5 at 576 (`dxf_children`, the call)
   - W6 at 1073
   - W7 at 1112
   - W8 at 1209
   - W9 at 1391
   - W10 at 2861

   Each lands in the code its description names.
   - W2's description, "the dxfs count is left at the children", is accurate but incomplete. The planted `count()` also sets a created container's count to 0, which no named test observes.
   - W5 covers only the call half of a test named "...at the call and a created then met one at the write". The write-time refusal (`if self.met { return Err(TWO_DXFS) }`) has no plant.

   Plants missing beyond check 1's list: the write-time TWO_DXFS; the removed element's priorities left out of `highest` (ruling 5); and the conditional format written before a created `dataValidations` (`formats.create_before` called ahead of `rules.`).

10. **The negative log reaches C3's message at the read, not the write (negative.log "C3: exit 2 ... RULE_UNPARSED's message").**
    - `cf_sample` calls `package.conditional_formats(&part)?` before any edit (diff-20d7d93.diff). So `priority="high"` is refused by `read_rule`, and the write's kept-priority refusal is never reached through the binary.
    - The tool test `the_cf_sample_refuses_..._unparsed_priority` has the same limit.
    - Likewise C5 is refused at `add_differential_style` (the call), not at the write.
    - The spec asks only for "refused", so the logs meet it. But "reached 3" in the reached/unreachable sets means reached at some point in the run, not at the write the spec text describes. The write-side refusal is shown only by the unit test `priorities_...`.

11. **Run-log lines the spec requires are present, as check 1 said.** The two targets 27/27; entries (W and the styles part changed, 54; 816 identical); descriptor only; lo and lo_in ok; host and i686 outputs identical; the keep and the edit open check; the record pair. Nothing to add.

12. **Spec requirements with no test or check:**
    - Ruling 3's "a write whose only styles edit is a dxf rewrites the other five containers' counts". Every dxf.rs fixture has correct counts (`BASE`), the corpus showed 0 corrected, and the fallback is exercised only by cfcmp's own `--self-test`. So the behaviour is unobserved, as the ruling allows, but it should be said.
    - "the styles part is read once per package" (the `dxf_children` cache) has no test.
    - The `rules.retain` on a pending sheet has no conditional-format test. P2 ruling 1 names only validations, so this is not required.

## Q3. Does every number and quoted string in report-draft-2.md appear in a primary?

Found in a primary:
- The hashes 95daff46... and 09643f94... (logs).
- The six commit ids and what each contains (commit-messages.txt and the diffs). "Two tool tests" matches null_rewrite.rs (two `#[test]`).
- "134 ... 141 ... 137" (gate.log 2041).
- "799, failed 0, ignored 2" on both targets (test.log 2195, 3042).
- W1 to W10, "each found once".
- The four "... checks all as expected: yes" lines.
- 27 rows, 27 ok, 0 refused.
- 20/74, 1360/1387, 11 and 0. I also re-added the per-row values and they match.
- "RESULT: PASS" with 0 hits (scan-pre-check.log 2099, 2103).
- "009C0006" and "00FFC7CE" (cfcmp.py, null_rewrite.rs).
- C5, and "a styles part has two dxfs elements" (negative.log).
- `committed=no` (negative.log 847).
- The empty `cfRule` for the four kinds (conditional.rs literals and the doc comment).
- Check 1's item numbers 3(a) to 3(c), 5, 11 and 13 (independent-check-1.md).
- The record names.

Could not be found, or not as quoted:

13. "each prints RERUN_NOTE: unset". Only gate.log, test.log and negative.log print that string. run.log line 8 prints "RERUN_NOTE=unset", and scan-pre-check.log line 6 prints "RERUN_NOTE is not set". The substance holds; the quoted form does not.
14. "(Unlogged aside, no primary: openpyxl writes a six-digit colour the same way.)" The report admits it has no primary. It is still an unsupported claim inside the report, and it should be dropped or evidenced.
15. "so that the reached and unreachable message sets are complete". The sets would be complete with TWO_DXFS stated unreachable instead, since negative.log checks only that reached plus unreachable equals the block. The stated purpose of C5 is the implementer's reasoning, not something a log shows.
16. "Independent check 1 ... found no defect that forces a rerun". Check 1's answer contains no such sentence. It is a reasonable summary but not a quote, and it should be labelled as the implementer's reading.
17. "11 dxfs created". The number is in run.log, but it counts inputs with no `dxfs` (finding 8). Under P1 verification ruling 1 the report should say so, as P1's record did for "containers created".

Check 1's Q3 items 15 to 19 are resolved in draft 2 except item 18, which now stands flagged as unsupported (my item 14).

Everything else in the draft traces to a primary.
