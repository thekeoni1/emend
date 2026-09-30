# Independent check 2, M2 part 4b P4

Written by the implementing session: the checked phase commit is
5854e34bed600e8fb2952cd22f9cc77acc4142f1, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check2 in this phase's folder on the disk (its prompt named it
<phase>/check2 and asked for <home> and <repo> in place of machine paths),
holding these files, by sha256: check 1's inputs, the draft report replaced
by report-draft-2.md, and independent-check-1.md as committed. The derived
inputs were made by the commands check 1's header prints.

    9b7535231471c59cd3442757c63f6e7c1d905c8fa4f32875e9eba625676b8acb  commit-messages.txt
    6e467bb95823328e52da9f67d4f0dd5cabe263805495b4d5e18027a39bd9167b  diff-3e48f99.diff
    f169eeb414021b894f7eb7caebd3c72d5460b546db5c0d59e16d45706f558204  diff-51c32da.diff
    b0fba99186a40846ac16d5f74ed23def67f33a19a75a74a6e1981b85a69f4df2  diff-5854e34.diff
    25702da570f4253e18d0e3fa88116e95304aaceb08ed0889f91b51c236914bcd  diff-phase-dd1a415..5854e34.diff
    6e5a863668e00a1ce2d09ee8e69118c6cbac10157224cc3fd9aa3ead30e68731  doc-commit-2679b59.stat
    c53ede6f70eecfe9dde2428e50da677be81ca4d87d43b3ba52feae3dd3cdfd75  gate.log
    39eb95793b37723fcd2fe3880080f0dbe7d93536761ee7c20914db364f4d2e84  independent-check-1.md
    d2e4468fedbe69e4b5f239e409d70f24e01484f329c5eb3e9cf9355d9bd7018a  m2p4b-open-2026-09-30-5854e34.md
    e339a8748c98f789c3c23466ec4cf3514d9297378c13d390ef48e3992dc77fe1  m2p4b-open-2026-09-30-5854e34.txt
    2fc36ee038cd5781537df64dbbc139965ecc8dcc61b6a2a4fecead121a025a7e  m2p4b-p4-cf-2026-09-30-5854e34.md
    c50d4cf22a3f2be2b13e36c550c65dd33b6df63dcaa55b2b72021c82bd00c45c  m2p4b-p4-cf-2026-09-30-5854e34.txt
    ad1a8e7c79d88b4d428aed2f82d77ed2b7816f4068dfc1a6f60a31f81b0b9958  m2p4b-p4-scale-2026-09-30-5854e34.md
    8383e9cfe3ea1b50bc3dc667dfd06e8ec029ae45519b87028843275ba3c97b01  m2p4b-p4-scale-2026-09-30-5854e34.txt
    f85051a2102b3f21775f1683f16c4487104f5f3851eb8764d724002abe7df564  m2p4b-p4-validate-2026-09-30-5854e34.md
    ec362be1c3d47329da36d47013ffb2d44d79899308ca3d3cc65f09990db7a4ff  m2p4b-p4-validate-2026-09-30-5854e34.txt
    9f003450a3a3078e87b977a5cf9d0d3e74774ecf88c865789351cb896a610e12  negative.log
    9ff862ae0ae819edb12a79fdb543f41c4f556a29cd94ab1ed9205523df901c45  report-draft-2.md
    b3667d2a008a811f27ddb1a5ad01d1b8e9741fcd72fa5f35ea84e3d2ab5bdbff  run.log
    b3eed2962adf78c59f19b942d61ef52bc170b49ee70773d1d02e4cab0dcca0d7  scan-pre-check.log
    72d24c37313df02e41df123bba44042a2241e9f5c4add91374e176a1f58b0efe  size-m2p4b-2026-09-30-5854e34.md
    58917a98966caf041af1067ed3c2821ff1b509018430ccb6b3e1cb1d634d7249  size.log
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  spec-0001-bootstrap.md
    2b4afb62a1f9dddbda03bdeb2c3a8d04df5126e8544d91ca418482c603d12d7a  spec-0020-committed.md
    b1607ca9686fa57fc20c255041af5ecd0b24c5620357526ced910aef927bf7c8  spec-0020-working-tree-with-P4-rulings.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/size.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/test.stderr
    b9d1947e7b2c9ade43ff0ec4a87736362d26b3449192f676e2add7cd4df933b8  test.log

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 c30349b3ee6e5d8931de935ed2d13190c2cc7f938bcde9ee612d5d322e86e132,
11455 bytes; plain ASCII; no machine path; nothing replaced; one line feed
added at its end). This is the second and last check the rules allow; what
it found goes to planning in the report.

---

I read the CLAUDE.md the harness put into my context. I did not use it as evidence. Everything below comes from files in `<phase>/check2`. The logs, records, specs and diffs hash the same as in check 1's table, apart from the report, which is now draft 2. I do not repeat check 1's findings unless noted.

## 1. Does the diff do what the spec (with the rulings) says for P4, and nothing else?

1. **51c32da (the carry), read in `diff-51c32da.diff`.** P3 verification ruling 2 is done: `str::from_utf8(prefix)` is gone and the line now reads `color_element("color", color).element(prefix)`. The prefixed-root test is added, and it is the first code commit. Nothing else is in the diff. Fine.

2. **3e48f99 (the catalogue), read in `diff-3e48f99.diff`.** It is all `//!` lines. Every item on P4's list is present: what the calls touch, a formula is text, the priority rule, `bgColor`, the `x14` twins, what is read back, and the Excel check pending. Ruling 4's citation is added to both entries. I checked it against spec lines 194-204, 364-368, 494-499 and 1180-1186: the spec calls "a text holds a character XML cannot carry" part 4's message for formulas, so citing that entry rather than "a formula text holds a character XML cannot carry" is correct. The claims "the blank rules carry Excel's formula for the first area's top-left cell" (spec 337-341) and "Both reads are snapshots" (spec 268, 517) are supported. The edits to the styles limits follow from the feature. Fine, apart from check 1's open wording items.

3. **5854e34 (the probe), read in `diff-5854e34.diff` (Cargo.toml hunks, probe lines 387-452 and the read-back).** It matches "Size" item by item. The Validation has `allow_blank: true` and no messages, which "Size" does not constrain. Nothing else is in the diff. Fine.

4. **The phase as a whole, read in `diff-phase-dd1a415..5854e34.diff` (six files) and `doc-commit-2679b59.stat`.** gate.log lines 1600-1615 show both of the doc commit's blobs as "the blob planning's message gave". Fine.

## 2. Do the logs contain every line the spec requires?

5. **Size log: the seventeen probes are classified, but the cause check cannot fail.**
   - The verdicts are present (size.log about 3597-3955): 4 identical (probe-zip, probe-zip-xml, probe-zip-xml-write, probe-emend-copy) and 13 code. Each rebuild at 3e67745 reproduced part 4a's sha256.
   - Each probe is "accounted for" by `cause_paths`, printed at size.log 811-815. That list gives `tools/size-probe/Cargo.toml` to every probe, and 5854e34 touched that file. So every probe has at least 1 cause: probe-zip shows "touched by 1 commits". The check "if the verdict is code, a commit that could have caused it is named above" would therefore pass for any probe, even if no emend commit existed.
   - Read against commit-messages.txt, none of P4's commits can explain a code verdict in a probe built without `rules`:
     - 3e48f99 is `//!` only.
     - 5854e34 only adds a feature and a bin. The four identical probes, probe-emend-copy included, show the manifest change moves nothing.
     - 51c32da changes `impl VisualChild`, which is the visual-rules code behind `rules`.
   - Yet the 11 causes counted for each emend probe include 3e48f99 and 5854e34.
   - `accounted` has no control: nothing plants a code verdict with zero causes.

6. **Size log: the core probe changed its code at zero size, and neither the record nor the report says so.**
   - `probe-emend-format-write-xml`, built with `emend,xml` and none of the part's features, is "code", with "differing bytes in .text: 59933" and a sha change from 883da056 to 8fcb683a. probe-emend-edit (49594 .text bytes) and ten others are the same.
   - So some commit between 3e67745 and P3 changed machine code in the featureless core at 0 net bytes. The candidates are among 8b82262 to 631c719, and 2ed9070's `crates/emend/Cargo.toml`. The log does not narrow it.
   - The record says the core's growth is 0 and that "the difference is the core's alone". That is true by size, which is the spec's definition. But the record's "all behind the feature and outside the core" and the report's "core's growth is 0" leave out that core code did change. Planning may want the cause named.

7. **A misleading line in the size record.** `size-m2p4b-...md`: "Among the seventeen the largest growth is 0 section bytes, `probe-zip`." Every one of the seventeen is 0, so naming probe-zip only reflects list order.

8. **Run log: how the phase run directories are found.**
   - `by_subject` (run.log 328) is `git log -n 1 -F --grep=...`. It matches anywhere in a commit message, newest first, not the subject line.
   - The results are right today: 157c11c, 0999a35 and dd1a415, with parents e79739a, 20d7d93 and 631c719. They are guarded by "exactly one matching corpus record .md" and by the comparison with planning's constants (run.log 3484-3491, 3767-3775, 4062-4070).
   - The weak points:
     - The check "the phase run directory the record names is the one planning named" has no control.
     - A later commit whose body quotes "M2 part 4b P1 record:" would silently win the lookup.
     - `phase_dir_ok` reads only the first row of each run.tsv.
     - The count of phase refusals is taken from `summary.host.tsv` alone.

9. **Run log: the byte identity rests on the on-disk phase directories.**
   - The committed phase records carry no sha256 for the outputs. The .txt has counts, byte counts and flags.
   - "Every output byte-identical by sha256" is therefore checked against whatever is on disk now. Only the entries and xml tables are also checked against the committed phase record (reccheck). The summary is compared with the on-disk phase summary, not the committed one.
   - This is sound only if nothing changed those directories after planning verified them.

10. **Run log: cfcmp.py under P4 ruling 1** (run.log 2603-2650, 2775-2803).
    - `want_dxfs_tag` now replaces `count` only when the input tag has one. The `--tag-test` cases cover four forms: no count, prefixed with a count, empty self-closing, and another attribute. `--plant-p2` fails them as it should (run.log 3347).
    - `dxfs_count`: when `cval` is present it only checks that `count` equals the number of children. So an output that wrongly adds a count to an uncounted input still passes `dxfs_count`; only `dxfs_tag` enforces the ruling. That is acceptable, but the docstring "or absent when the input dxfs had none" (run.log 2175-2177) reads as if `dxfs_count` enforced absence, and it does not.
    - Check 1's point that the new branch `bool(dx_in) and not had_count` has no control is still unresolved.
    - The script's top header (run.log 79) cites "Planning verification and rulings on P2, ruling 1" and not P4 ruling 1, while change CD cites both. This is minor.
    - The comparison runs under the pin 2b4afb62, whose P2 verification ruling 1 still reads the other way. That is authorised by P4 ruling 1's own text, as check 1 said.

11. **Negative log: the reasons for the ten unreachable messages, one by one** (negative.log 1180-1190).
    - NO_AREA, FORMULA_MISSING, MESSAGE_LONG, FIELD_NOT_OF_KIND, DXF_FIELD, RULE_STYLE and VISUAL_VALUE: raised at the call from the tool's constants. Each is consistent with "Harness runs". Fine.
    - NO_SUCH_VALIDATION: "removes only the list it has just written". This is correct: the validate sample removes (M+1,1), the List. Fine.
    - **NO_SUCH_RULE is wrong in its wording.** It says "the conditional format sample removes only the list it has just written". The cf sample removes the Expression rule on (M+2,1) (spec "Harness runs"), not a list. The text was copied from the validation reason.
    - DXF_OUT_OF_RANGE: it is both a write-time and a call-time refusal. The reason is plausible; the u32 claim cannot be checked from these files.
    - **The claim that every at-call refusal is unreachable is contradicted by the same log.**
      - negative.log header BB (lines 131-133) says the ten unreachable are "the other ten, every at-call refusal among them".
      - But negative.log 99-100 says C5 gives "the two-dxfs message, at the differential style's call", and TWO_DXFS is reached.
      - So not every at-call refusal is in the unreachable ten. The report's "(the at-call refusals and three write-time ones)" repeats this. Check 1 found the converse (write-time messages inside the unreachable list), not this.
    - The set check, 15 = 5 + 10 with none in both, is fine.

12. **Everything else the spec requires is present and fine.** That covers:
    - test.log on both targets, feature off and on, 808/0/2, with the Z1-Z3 plants found once each;
    - every log pinning 2b4afb62;
    - RERUN_NOTE unset;
    - the size record installed and hashed;
    - the release-binary hashes;
    - the edit open check.

## 3. Does every number and quoted string in report-draft-2.md appear in a primary?

Found:
- The hashes 2b4afb62 and b1607ca9, and all five commit hashes.
- The five "... checks all as expected: yes" lines.
- 808/0/2 on both targets, and Z1-Z3 as described (test.log 1009-1027, 3501-3527).
- 5 of 15 messages reached, with RULE_UNPARSED's priority case through C3 and S3.
- 48116/49152/41008, 968440-968440 and 3e67745, 556 = 968440-967884, and 435224/458752/23528/0.
- The record names, the three run directories, and 27 rows with no refusal.
- "RESULT: PASS" with 0 hits (scan-pre-check.log 2166, 2170).
- 40000 to 60000 (spec line 91).
- dxfs_tag and dxfs_count, and change CD.
- Check 1's quoted strings, and 10 against 11 (size.log 3594, 3602).
- The test name `a_removal_drops_its_list_in_order_and_the_last_drops_the_container` (in test.log).

Not found, or not supported:

13. "(the at-call refusals and three write-time ones)" is inaccurate: TWO_DXFS is at-call and reached (finding 11).

14. "no corpus input has the shape" comes only from the spec's text (P2 verification ruling 1) and check 1's answer. No log counts inputs whose `dxfs` has no `count`: the cmp lines print `dxfs_in` and `dxfs_present_in`, not `had_count`.

15. Report item 3: "validation::a_removal_drops_... asserts both" and "visual.rs is not it" rest on test bodies that are in no primary. The "unlogged aside" label sits after "validation.rs lines 486 and 507". Read literally it covers only the line numbers, so its scope should be made to cover the whole claim.

16. The report's "the core's growth is 0" is supported. But the report is silent on the 13 code verdicts, the core probe among them (finding 6). That is not a missing number; the report just leaves it out.

17. "scan.log: the final stage runs after the checks" is a statement of what will happen, with no primary yet. It is expected at this point.

18. Labelled unlogged asides, listed as such: "the comparison was changed after its dry run and before the real run; the dry-run copies are kept on the disk".

**Summary:** The diffs do P4 and nothing else. The substantive findings check 1 missed are:
- the size log's code-verdict accounting cannot fail (finding 5);
- the core probe's machine code changed at zero size, and neither the record nor the report says so (finding 6);
- the negative log's NO_SUCH_RULE reason names the wrong thing, and its "every at-call refusal" claim is contradicted by C5 (finding 11), which the report repeats (finding 13);
- the run log's phase-directory lookup has no control, and its byte identity rests on directories on disk that no committed hash anchors (findings 8 and 9).
