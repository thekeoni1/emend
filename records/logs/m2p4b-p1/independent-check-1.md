# Independent check 1, M2 part 4b P1

Written by the implementing session: the checked phase commit is
e79739a6391a2ab8353334e42bfcf15a92fbc771, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk (its prompt named it
<phase>/check1 and asked for <home> and <repo> in place of machine paths),
holding these files, by sha256. The logs there are gate.log, test.log and
negative.log at attempt 1, as committed; run.log at attempt 2 and the scan
pre-check at attempt 1, both superseded after this check (run.log attempt 3
answers the check's Q2-8; its rerun lines and the final scan.log hash both):

    e2fc27d1ee68ae35e0b7d474e787ebebf8af7872efff78ebe6b39d772267edda  commit-messages.txt
    09e758059a0ce993f5e7c24388037f68618e244c8ae61fb8926e7b972a9760eb  diff-code1-8b82262.diff
    9d9fa4960659abd948c98fc01ed680f76522ac0b1c08c18a76f28fa3875f932f  diff-code2-2ed9070.diff
    3d19686216591af7ece53f061a16395e17a35edeb08f9e1d80988a1eb81b9cdb  diff-code3-e79739a.diff
    05024287b8709fde26dbc81b58e84616746d064b355b43501ebbf15f939111db  diff-phase-335da53..e79739a.diff
    19071913eeda6b62e66e73477a626a39e574bc12dd824781a763fc3a122cf851  doc-commit-715177a.stat
    231a650b4b9d1323758e7f8c4833854396f2f3db2c2f6436e8c5efc8c00833de  gate.log
    f8da60b80c59fd3e497deb11ba53a08243639a0e37b6a00c22f846c72c921817  m2p4b-open-2026-09-29-e79739a.md
    77987ffcd4ec7998bce0d6278941acbf0ae73fc6d7a0ede51ced91234ddc3ce7  m2p4b-open-2026-09-29-e79739a.txt
    7b6ad072e38abfc24668bda7bd2a41a7bf976cebb8fe584322866ade21f35d30  m2p4b-p1-2026-09-29-e79739a.md
    32c4b87005804947602ca7deb1044f05b8bc615ed58b81c50f55ac906e1679a7  m2p4b-p1-2026-09-29-e79739a.txt
    1e40a50aa9b9d9bd835acadffa3aa85bdc0da897e661f470016a567d6e173eac  negative.log
    05ae325d6c12b0319af962dd2541883878bb9ca3e652b15e6e83f13dfca82e47  report-draft.md
    77bea77d4d4735713e8d841c5390e3d8c45baaad7b49066a1fc7a768b06a5ef9  run.log
    5cb4833f5012ea37968bb21a57bf6d5f0c8dde07a7af8869d146f6347542e397  scan-pre-check.log
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  spec-0001-bootstrap.md
    dc4cb48509becb9d2c93f7e6e728f85151666f950feb9e154d096ca816c34bbe  spec-0020-committed.md
    1d22af0346fb32043f64f98dae22c63dc76732e01fd89b12463cbd5f30467776  spec-0020-working-tree-with-P1-rulings.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/test.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded/run.log
    1a38770eef93e37a2daba35c5cbde0001a7e1c81aed471fd355b50c4674aa048  superseded/run.stderr
    84b13f549912c97ea2e491b06ab97291d47516c8ecacc22f5e4f83760019e062  test.log

One input was assembled wrong, as the answer's Q1-1 found:
diff-code1-8b82262.diff is git diff 335da53 8b82262, so it holds the doc
commit's three files as well as code commit 1; check 2's directory holds
git diff 715177a 8b82262 under that name instead.

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 b6051fdfeb810e0237315804065cd54f9537c5c5145909db821e5233a1e4fc42,
14236 bytes; plain ASCII; no machine path; nothing replaced; one line feed
added at its end).

---

I have read everything in `<phase>/check1`. The harness also placed a CLAUDE.md in my context. I have not used it as evidence.

**Summary:** The diff does what P1 and the rulings ask. Every P1 test item has a test, and every required log line is present. Two small defects of form: one diff file holds more than its code commit, and one run.log check has no control and could not realistically fail. Every number in the draft report traces to a primary, and I found none unsupported. Four items are for planning's attention (Q1-6, Q1-10, Q2-6 to Q2-8).

## Question 1: does the diff do what the spec says for P1, and nothing else?

**Q1-1. The code 1 diff file holds the doc commit as well (defect of form, check directory only).**
- `diff-code1-8b82262.diff` holds four file sections: ROADMAP.md, specs/0019 and specs/0020 as well as `package/tests/properties.rs`.
- Its three doc sections are byte-identical to the phase diff's, so the file is a diff from 335da53 to 8b82262, not the diff of 8b82262 alone.
- The properties.rs section on its own hashes to dfcd595d...71e7. That is the hash gate.log prints for `git show 8b82262` (gate.log, "each commit's diff ...").
- gate.log shows, from git, "code commit 1 touches: crates/emend/src/package/tests/properties.rs", "paths: 1", and "lines added 20; lines removed 0".
- So "code 1 is the carry test and nothing else" is confirmed from gate.log plus that one section, not from the file as named.
- The other diff files hash exactly to gate.log's figures: code 2 = 9d9fa496..., code 3 = 3d196862..., phase = 05024287....

**Q1-2. The per-commit diffs split the phase diff cleanly (fine).**
- Checked section by section against `diff-phase-335da53..e79739a.diff`.
- Their union is exactly the phase diff, and no file appears in two commits.
- Code 2 touches only the crate's Cargo.toml, lib.rs, package.rs, tests.rs and tests/validation.rs. Code 3 touches only the tool's Cargo.toml, main.rs and tests/null_rewrite.rs.
- I found nothing outside P1 (no P2/P3 types or calls) in either diff (diffs read: code2 and code3).

**Q1-3. The first code commit against "Phases" (fine).**
- The test `a_core_part_added_in_the_same_write_does_not_take_a_property_edit` is added beside the read-side test.
- It uses `add_part` on the missing core part, then `set_property` with `Some` and with `None`, and asserts "no such part".
- It sits behind `metadata`: test.log shows it running alone with `--features metadata,rules`, and "control: the carry test's ok line is absent from the crate's run with metadata off".
- Plant V7 bites (test.log).

**Q1-4. "Areas and formulas" and ruling 4 (fine).**
- `checked_areas` refuses an empty list with NO_AREA, normalises each area, and refuses an off-grid corner with INDEX_OUT_OF_RANGE (ruling 5).
- `rule_formula_ok` refuses a leading `=` with part 4's message, and a forbidden character with TEXT_NOT_XML.
- Formulas are escaped with `partial_escape`, and a single cell is written as one reference ("C3").

**Q1-5. "Data validation" (fine, with one reading noted).**
- Attributes are written in the spec's order, with `showDropDown` after `allowBlank` (ruling 5).
- `operator` is written only for `Some` (ruling 2). `hide_drop_down` or an operator on the wrong kind gives FIELD_NOT_OF_KIND (ruling 1).
- `formula2` is required exactly when the operator is Between/NotBetween, or absent on a kind that takes one.
- Title limit 32 and text limit 255 are counted in characters. No `xr:uid` is written.
- The read gives `input`/`error` only when the flag is `1` or `true` (ruling 3). A `type` of `none` or absent reads as `None`. An unknown type, operator or errorStyle, or a missing or unparsable sqref, refuses the read with VALIDATION_OVERLAP.
- In the write:
  - the container is held;
  - a removal skips every file element whose parsed list equals the given one, and never matches an addition (ruling 5);
  - the overlap check runs against kept elements only after removals;
  - two additions that overlap are refused in `write`;
  - `count` is set with the other attributes kept, an empty container is dropped, and a second container is refused;
  - a validation edit counts as "any other edit" in `shift_alone` via `no_rules`.
- Replaced or removed parts are refused as tested.

**Q1-6. Untested code path (low).** A `retain` of validation edits was added in the `Parts` function that already drops link and protection edits for a part (code2 diff hunk `@@ -4413,6 +4888,8`). No test covers a validation edit being dropped this way (for example by removing the sheet). The spec's P1 list does not ask for one.

**Q1-7. "Creating an element" (fine).** `ValidationsPass::create_before` compares against `schema_position(dataValidations)` and is called only for main-namespace root children at depth 2. It is called after sheetProtection's creation and before hyperlinks', and the root-end paths use `create_at_end`. It is tested before hyperlinks, before pageMargins (after conditionalFormatting), before extLst, and at the template's root end.

**Q1-8. "Errors" (fine).**
- The seven P1 constants match the spec's text exactly, the fourth coming from ruling 1.
- negative.log prints `FIELD_NOT_OF_KIND committed=no working=yes`, as the report says.
- The module-header catalogue cites a test for each message, and each cited test exists.

**Q1-9. "Harness runs", `--validate-sample` (fine).**
- W is the first sheet of kind Worksheet, and M comes from `sample(...).max_row`.
- The list goes on (M+1, 1) with `"\"emend,sample\""`, allow_blank, input ("emend", "pick one") and a Stop error ("emend", "not in the list").
- The whole-number validation is Between "1" and "10" with `Some(Between)` on (M+1, 2) to (M+2, 2).
- The output is written, reopened and asserted: dv_after = dv_before + 2, and count_after (read by the tool's own token pass) = dv_after.
- The removal goes through a Cursor, and dv_removed = dv_before + 1 is asserted.
- On any failure after the write, the output is removed.

**Q1-10. The P1 test list, item by item (code2 and code3 diffs).**
- Three validations read, and none read: `the_validations_of_a_part_with_three_and_of_one_with_none_are_read`. Present.
- Each kind and operator against literal bytes: `every_kind_and_operator_is_written_as_its_literal_bytes`. It covers 5 kinds x 8 operators plus None, List with each error style, and Custom. Present.
- Container append with kept bytes, count and xWindow: `an_addition_is_appended_...`, where THREE carries `xWindow="5"` and the expected string is exact. Present.
- Template, hyperlinks, pageMargins, root end, with the log printing output: `a_container_is_created_before_...`; test.log prints the four worksheets. **Reading to note (low):** the hyperlinks and pageMargins cases use byte-built bodies, not the template part, because the template has neither child. Planning may want to confirm that reading of "to the template part (created before hyperlinks when the part has one, else ...)".
- extLst alone: present.
- Overlap with a kept validation, with an earlier addition, and removal-then-addition accepted: present.
- Removal cases: multi-area in order, absent, other order, and last-drops-container are all present. There is also an extra ruling 5 case: removing an addition of the same write is refused.
- Two containers and an unparsable kept sqref: present.
- Call-time refusals: no area, empty formula, List with operator, formula2 without Between, hide_drop_down on Whole, 33-character title and control character are all present, and "records nothing" is asserted by writing the package afterwards.
- Shift refused: insert_rows only, but the whole path is one check and plant V6 bites.
- Replaced part: present.
- Tool line: present, two tool tests.

No test asserts less than its name, as far as I can tell.

## Question 2: do the logs contain every line the spec requires?

**Q2-1. Spec pin (fine).** Every log pins dc4cb485...6bbe, which matches `spec-0020-committed.md` by sha256:
- gate.log: "the spec at the phase commit is the blob planning's message hashed: as-expected", with a control.
- test.log: the same.
- negative.log: line 64, with a control.
- run.log: SPEC_SHA input, "at the one pinned hash: yes", with a control.
- scan-pre-check.log: line 8 and line 51.

Each log also prints the working copy's 1d22af03...7776 as "not the pin", which matches `spec-0020-working-tree-with-P1-rulings.md`.

**Q2-2. Commit ids and HEAD (fine).** Every log reads the commit ids from git and compares the given phase commit with HEAD, with controls (null commit, short sha). Commit messages are checked as ASCII, ending with Co-Authored-By only, and under the count rule, all with controls.

**Q2-3. Standard error (fine).** Each log counts its own stderr with a control. All five files in `stderr/` are 0 bytes, matching.

**Q2-4. Forbidden shapes (fine).** The three shape greps run over each script with fixture controls. scan-pre-check.log also runs them over its own source, reporting "chained=0 leading-bang=0 assignment=0".

**Q2-5. Gate requirements (fine).**
- Feature off and on, both targets: 512 / 523 passed.
- pub fn / pub const first-line lists with the feature off (134, the base's) and on (137, exactly the three calls).
- A consumer crate with the 8 gated items, one error line each when off and a clean check when on.
- Clippy and doc with the feature off, with rules, and with all features.
- Gate files listed against the diff with no self-test needed.
- test.log's crate runs and workspace run have rules on ("validation module tests passed 12" in the workspace).

**Q2-6. test.log (fine).**
- Workspace: 779 passed, 0 failed, 2 ignored on both targets.
- Previous total 764 is read from part 4a P4's test.log, and 15 tests were added.
- The eight plants V1 to V8 each show the unplanted pass, the planted run and the failing assertion.
- **Plant V2 (low):** its failure comes from the missing refusal of the other-order list, not from a byte mismatch. That is fine, but it only proves the order check.

**Q2-7. negative.log (fine).**
- V1 (validation on A1, M = 0) is refused with the overlap message, controlled by V0 and V2.
- V3 (two containers) is refused, controlled by V0 and V4.
- Reached versus stated-unreachable message sets are complete, and the fixtures are generated from the template at the phase commit.

**Q2-8. run.log: both targets, 27 of 27, every flag checked (fine), with one weak check.**
- The ten comparison flags each have a planted control that turned them ("plants: 10; ... turned their own flag: 10"). The keep, the open check against part 4a P4's record, and both records installed are all present.
- **Weak check, no control (defect of form):** "`<label>`: containers created, 15, lie between none and every ok workbook" is `created_le`, i.e. 0 <= t <= 27. It has no control and could not realistically fail.
- The 15 is not tied to the per-row data. I counted 12 rows with `container_in=1`, so 27 - 12 = 15 is correct, but no check reads that relation.
- Per row, `container_created` is also not checked against `1 - container_in`. `container_place` and `start_tag` do cover the created case.
- The totals check "76 + 2 x 27 = 130" has no control either, though unlike `created_le` it can fail.

**Q2-9. Rerun header (fine).**
- run.log names attempt 1's kept log and stderr by sha256: e3b0c442... (0 bytes) and 1a38770e... (129 bytes). Both match the files in `superseded/`.
- The kept stderr's message and the printed script's line 243 (`... move it aside first" >&2; exit 2`) support "exited 2".
- **Privacy (low):** that kept stderr holds an absolute machine path. It is not a log and must not be committed unscrubbed.
- **Dry run (low):** the dry run whose staged records caused attempt 1 is named, but it is not logged and its moved files are not hashed. Whether a dry run counts as an attempt is planning's call.

**Q2-10. Scan and "Phases" rules (fine).**
- scan-pre-check.log carries ruling 7: "control: the planning-file check refuses a one-line listing naming a file that is not the spec, on its content (change BB): yes". It reports "total hits: 0" and "RESULT: PASS".
- Placeholders: the only absolute-path strings are in scan/check source text (pattern literals, a split fixture) and a `<scratch>` path. No log has non-ASCII or control bytes (my own grep: 0 in each).
- A figure taken from another log is read from that log: test.log's 764, negative.log's gate hash, run.log's test.log hash 84b13f54....
- The SERIALIZED lines show no other cargo process was running.
- The final scan.log is pending, as the report says.

## Question 3: does every number and quoted string in report-draft.md appear in a primary?

**Q3-1.** Found in the primaries:
- Blob dc4cb485... and working copy 1d22af03...: gate.log.
- The commit ids and base 335da53: gate.log.
- "exactly at planning's hashes": gate.log.
- The carry test's wording and "no such part": code1 diff.
- Two tool tests: code3 diff.
- The gate, test and negative "... checks all as expected: yes" lines: last line of each log.
- 779 / 0 / 2 on both targets: test.log lines 2087 and 2914.
- Eight plants V1 to V8, each found once, passing unplanted and failing planted: test.log.
- run.log's "yes", attempt 1 "0 bytes" and "exited 2 at entry", and the two kept files hashed: run.log lines 8 and 40, and its hash lines.
- 27 rows, 27 ok, 0 refused, 76 / 130, 15 created on each target: run.log.
- The four record names: run.log install lines.
- The pre-check RESULT PASS and 0 hits: scan-pre-check.log.
- "committed=no": negative.log.
- The fifteenth message text: code2 diff and the working spec.
- Area's public corners: code2 diff and the test built with row 1_048_577.
- "no corpus validation covers the sample's cells": run.log, 0 refused.

**Q3-2. Labels not stated in the logs (low).**
- "run.log attempt 2" is not written in run.log; it follows from its rerun header naming attempt 1.
- "gate.log attempt 1" and the like follow from each log's "RERUN_NOTE: unset".

**Q3-3. A wording point (not a number).** Under "Logs", the report writes "scan.log" for a stage that does not exist yet. It says as much ("the final stage runs after the checks").

I found no number or quoted string in the report that I could not trace to a primary.
