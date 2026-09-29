# Independent check 2, M2 part 4b P1

Written by the implementing session: the checked phase commit is
e79739a6391a2ab8353334e42bfcf15a92fbc771, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check2 in this phase's folder on the disk (its prompt named it
<phase>/check2 and asked for <home> and <repo> in place of machine paths),
holding these files, by sha256. The logs there are the four committed logs
(gate.log, test.log and negative.log at attempt 1, run.log at attempt 3),
independent-check-1.md as committed, and the scan pre-check's attempt 2,
which the final scan.log's rerun lines hash with attempt 1:

    e2fc27d1ee68ae35e0b7d474e787ebebf8af7872efff78ebe6b39d772267edda  commit-messages.txt
    dfcd595d8beb369ee235c750ee1f7cd3f551dfca13dd3e0562fcab084fa771e7  diff-code1-8b82262.diff
    9d9fa4960659abd948c98fc01ed680f76522ac0b1c08c18a76f28fa3875f932f  diff-code2-2ed9070.diff
    3d19686216591af7ece53f061a16395e17a35edeb08f9e1d80988a1eb81b9cdb  diff-code3-e79739a.diff
    05024287b8709fde26dbc81b58e84616746d064b355b43501ebbf15f939111db  diff-phase-335da53..e79739a.diff
    19071913eeda6b62e66e73477a626a39e574bc12dd824781a763fc3a122cf851  doc-commit-715177a.stat
    231a650b4b9d1323758e7f8c4833854396f2f3db2c2f6436e8c5efc8c00833de  gate.log
    0900c02851730322dd56d763a9b90ae9a052a49240a922893246d54a63f25a03  independent-check-1.md
    e7d5f141c7d367566776af98c26cb2e631aee3f09728cac56587271b64bdb16b  m2p4b-open-2026-09-29-e79739a.md
    77987ffcd4ec7998bce0d6278941acbf0ae73fc6d7a0ede51ced91234ddc3ce7  m2p4b-open-2026-09-29-e79739a.txt
    781d0fb5fff6ebcd8ede02a9e00156a12bc6c633a3152e0256a287b49536035d  m2p4b-p1-2026-09-29-e79739a.md
    6d524421df3602a9f45941ec40c3b0f68ae7e41fdef0854b27badb7e93e4ac23  m2p4b-p1-2026-09-29-e79739a.txt
    1e40a50aa9b9d9bd835acadffa3aa85bdc0da897e661f470016a567d6e173eac  negative.log
    97cfead3b050a5d34b40ff95cd3bb8d8376ff07b878961a3ce71d2b2e0efd35b  report-draft-2.md
    67abac791b2ebe37c970464a79a727062c1b0e5ed19855aad2ce2d5cabd8ace0  run.log
    aa94c30a7796af43d0fe9642643e8a57bc8b327045c199ec27721fe24d14a8a5  scan-pre-check.log
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  spec-0001-bootstrap.md
    dc4cb48509becb9d2c93f7e6e728f85151666f950feb9e154d096ca816c34bbe  spec-0020-committed.md
    1d22af0346fb32043f64f98dae22c63dc76732e01fd89b12463cbd5f30467776  spec-0020-working-tree-with-P1-rulings.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/test.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded/run-attempt1/run.log
    1a38770eef93e37a2daba35c5cbde0001a7e1c81aed471fd355b50c4674aa048  superseded/run-attempt1/run.stderr
    776da729b1565250f01805333a494b7dbddc5d5850139788a82ed671a7b26bc3  superseded/run-attempt2/m2p4b-p1-run.sh.attempt2
    f8da60b80c59fd3e497deb11ba53a08243639a0e37b6a00c22f846c72c921817  superseded/run-attempt2/records/m2p4b-open-2026-09-29-e79739a.md
    77987ffcd4ec7998bce0d6278941acbf0ae73fc6d7a0ede51ced91234ddc3ce7  superseded/run-attempt2/records/m2p4b-open-2026-09-29-e79739a.txt
    7b6ad072e38abfc24668bda7bd2a41a7bf976cebb8fe584322866ade21f35d30  superseded/run-attempt2/records/m2p4b-p1-2026-09-29-e79739a.md
    32c4b87005804947602ca7deb1044f05b8bc615ed58b81c50f55ac906e1679a7  superseded/run-attempt2/records/m2p4b-p1-2026-09-29-e79739a.txt
    77bea77d4d4735713e8d841c5390e3d8c45baaad7b49066a1fc7a768b06a5ef9  superseded/run-attempt2/run.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded/run-attempt2/run.stderr
    84b13f549912c97ea2e491b06ab97291d47516c8ecacc22f5e4f83760019e062  test.log

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 b575d7c3871c8eaf0a5a5c88a530dfeb625500bf70229d122351a243356d3f3c,
13309 bytes; plain ASCII; no machine path; nothing replaced; one line feed
added at its end). This is the second and last check the rules allow; what
it found goes to planning in the report.

---

I read everything in `<phase>/check2`. The harness also put a CLAUDE.md into my context; I did not use it as evidence. I read no repository file and ran no cargo. I printed no corpus content, only counts, column names and hashes.

**Summary:** The diff does what P1 and rulings 1 to 5 ask. Run attempt 3 does make check 1's Q2-8 finding go away on paper, and its rerun header hashes every superseded file correctly. But the new "containers created" equality and the totals check cannot fail unless an earlier per-row check has already failed, so the fix adds no independent evidence (item 1 under Question 2). Two smaller points on the spec's wording and the pre-check's attempt history follow. Every number in report-draft-2.md traces to a primary.

## Question 1: does the diff do what the spec says for P1, and nothing else?

**Q1-a. Code 1 is fine now** (read: diff-code1-8b82262.diff).
- The file's sha256 is dfcd595d...71e7, which is gate.log line 1582's hash for 8b82262.
- It holds only the properties.rs test. Check 1's Q1-1 is resolved.

**Q1-b. Code 2 holds nothing outside P1** (read: diff-code2-2ed9070.diff). The only files are:
- Cargo.toml (the feature `rules`);
- lib.rs (the five re-exports);
- package.rs (types, three calls, the pass, the read, seven constants, the catalogue);
- tests.rs (the module line);
- tests/validation.rs.

It has no P2 or P3 type (Rule, Cfvo and so on) and no dxfs change. The extra test `a_validation_and_a_cell_edit_on_one_part_compose` is beyond the P1 list; it is a test, not code, and it is harmless.

**Q1-c. Code 3 holds only the tool** (read: diff-code3-e79739a.diff): the tool's Cargo.toml feature, main.rs `--validate-sample`, and two tests in null_rewrite.rs. The phase diff is the union of the three code diffs plus the doc commit's three files (doc-commit-715177a.stat).

**Q1-d. Spec wording versus code for a removed part (new; for planning).**
- spec-0020 "Errors", in both copies: "Reused: part 1's 'a cell edit to a part this write replaces' for every worksheet edit here on a replaced or removed part".
- The code and test give a different message for a removed part. validation.rs `a_validation_edit_on_a_removed_or_replaced_part_is_refused` asserts "an edit to a removed part" after `remove_part` (phase diff line 2107). Only `replace_part` gets "a cell edit to a part this write replaces" (line 2114).
- The doc comment on `add_validation` says "what a cell edit to the part would meet". That is a reasonable reading, but it is not the spec's literal text.
- The P1 test list only asks for "on a replaced part refused", so no test is missing. The spec text and the behaviour differ, and planning should say which stands. Check 1's Q1-5 ("Replaced or removed parts are refused as tested") did not notice the different message.

**Q1-e. Refusals the catalogue claims but no test exercises (low).**
- The doc comment and `FORMULA_MISSING`'s entry cover "a `formula2` missing for `Between` or `NotBetween` (or no operator on a kind that takes one)". The refusal list in `the_call_refuses_each_rule_and_records_nothing` covers `formula1: ""`, Equal with formula2, and List with formula2. It has no case with `formula2: None` under Between, and none with `formula2: Some("")`.
- `FIELD_NOT_OF_KIND` is tested for List with an operator and for hide_drop_down on Whole, not for Custom with an operator. Ruling 1 names "both", meaning the operator case and the arrow case, and both are covered.
- The spec's P1 list does not require any of these.

**Q1-f. The "limits taken" block asserts less than its comment (low).**
- The comment reads: "and a missing operator on a kind that takes one reads as `between`" (test file, end of `the_call_refuses_each_rule_and_records_nothing`).
- The block only asserts that the call is accepted (`.expect("the limits are taken")`); it never writes or reads back.
- The "no operator" bytes are asserted elsewhere, in `every_kind_and_operator_is_written_as_its_literal_bytes`, so the behaviour is covered, just not by this block.

**Q1-g. One tool-test claim I cannot check from the diffs (low).**
- The test comment says "With no container one is created before pageMargins" (null_rewrite.rs, the new test).
- Whether the helpers `edit_specs` and `edited` put a pageMargins in the body is not in any diff I have.
- The test compares the whole sheet's bytes, so the placement is asserted, whatever it is.

**Q1-h. Checked and fine:**
- attribute order, including showDropDown after allowBlank;
- `operator` written only for Some;
- the flag read as `1` or `true`;
- normalisation, and the off-grid index refusal;
- the leading `=` refusal;
- removal matching file elements only, dropping every equal list;
- the overlap check against kept elements after removals;
- `count` set with the other start-tag attributes kept;
- an empty container dropped, and a second refused;
- the `no_rules` hook into `shift_alone`;
- the `no_rule_edits` hook into the null-rewrite test;
- the tool's W, M, constants, asserts, Cursor removal and output removal on failure.

## Question 2: do the logs contain every line the spec requires?

**Q2-1. Run attempt 3's Q2-8 fix cannot fail on its own (new; the main finding).**
- The comparison script printed in run.log computes `"container_created=%d" % (0 if dv_in else 1)` beside `"container_in=%d" % (1 if dv_in else 0)`. So container_created is exactly 1 minus container_in, read from the input only, not from the output.
- `tool_and_cmp` (run.log line 511ff) already requires, per row, the tool's `container_before` = the comparison's `container_in`. It passed for all 27 rows ("the tool and the comparison agree in every ok workbook").
- So `created_ok` (sum of container_created = number of rows with container_before=0) follows from that per-row check. It can fail only if `tool_and_cmp` has already failed.
- The same holds for `totals_ok`: `line_as_spec` requires dv_after = dv_before + 2 per row, and `tool_and_cmp` ties those to dv_in and dv_out. So t_dv_out = t_dv_in + 2 x n_ok is implied.
- The two new controls (`totals_ctl`, `created_ctl`) only show that the comparison functions refuse a count one off. They do not show that the quantity being compared is independent.
- So the Q2-8 finding is closed only on paper: the checks now have controls, but they add no evidence.
- The real evidence that a container was created in the output is the per-row flags `one_each`, `container_place` and `start_tag`. Those are checked, and their plants turned them ("plants: 10; ... 10").
- "15 containers created" in the logs and the report means 15 inputs with no container. That statement is true, but it is a count of inputs.
- The rules allow at most two checks, so this goes to planning, not to a third round.

**Q2-2. The rerun header matches every kept file (fine).**

| File (superseded/) | sha256 | Bytes |
|---|---|---|
| run-attempt1/run.log | e3b0c442... | 0 |
| run-attempt1/run.stderr | 1a38770e... | 129 |
| run-attempt2/m2p4b-p1-run.sh.attempt2 | 776da729... | 75758 |
| run-attempt2/run.log | 77bea77d... | 243710 |
| run-attempt2/run.stderr | e3b0c442... | 0 |
| run-attempt2/records md | 7b6ad072... | 15587 |
| run-attempt2/records txt | 32c4b870... | 324647 |
| run-attempt2/records open md | f8da60b8... | 129639 |
| run-attempt2/records open txt | 77987ffc... | 116354 |

- Each value matches my own sha256 of the kept file. 77bea77d and the four record hashes are also the ones check 1's header pinned.
- The script listing printed inside attempt 2's run.log matches the kept attempt-2 script (they differ only by one trailing blank line from my extraction).
- One gap (low): RERUN_NOTE says attempt 2's "staged records are kept", but no line hashes the staged copies. Attempt 2's own log does print them ("written <scripts>/out/records/..." with the same hashes as the installed ones). Under spec-0001 ("Every sha256 a log cites, a superseded attempt's included, is printed by a command run over a file that exists at run time"), this is a file named as kept but not hashed.

**Q2-3. Attempt 2 against attempt 3: nothing missing or changed beyond the named change (fine).** diff of the two run.logs: 337 lines, all accounted for:
- **Inputs:** RUN_SUFFIX=-b, SUPERSEDED_RUN, SUPERSEDED_KEEP, RERUN_NOTE, RERUN_FILES.
- **New moves:** the new move blocks for attempt 2's run directory (762 files) and keep (498 pairs), each hashed.
- **Run directory:** renamed `...-e79739a-b`.
- **Script listing:** only the BQ comment and the lines for `t_absent`, `totals_ok`/`ctl` and `created_ok`/`ctl`.
- **Old check:** the old range-check line is replaced by three lines per target.
- **Timings and names:** timings and RSS figures, the i686 keep pid and the open-check temp name differ.
- **Record hashes:** the p1 record hashes are new, as are the open .md hash and the end time.

The records differ the same way:
- p1 .txt: only the columns wall_s (3 rows) and peak_rss_kb (54 rows); every other column and line is identical.
- p1 .md: the run name, the times and 4 bytes ("-b").
- open .md: the timestamp only.
- open .txt: byte-identical (77987ffc).

No check line of attempt 2 is missing from attempt 3. The header does not mention the new run-directory suffix, but the inputs line shows it.

**Q2-4. The first scan pre-check is not named by the second (new; low, for planning).**
- scan-pre-check.log (aa94c30a) prints "RERUN_NOTE is not set" and "RERUN_FILES is not set".
- The first pre-check (5cb4833f, per check 1's header) appears only in independent-check-1.md's header. It is not in `<phase>/check2/superseded`.
- If a pre-check counts as an attempt under "a superseded attempt stays on disk", the second should name the first by hash. The script supports that (lines 654-674). Planning's call.
- Related: the pre-check requires "check answers present: 0 (stage pre-check: none before the checks...)". It passed after check 1 only because check 1's answer was not yet in the log directory. So check 1's answer has not been scanned by any stage; the final scan must cover both answers.

**Q2-5. Plants (fine, with one gap).**
- All eight show the unplanted pass, text found once, the planted FAILED and the assertion output (test.log 3417-3515).
- V1 disables only the check against kept validations. The write-time check between two additions (the `added[..at]` loop) has no plant, though a test asserts it.
- V2 only proves the order check, as check 1 said.

**Q2-6. Everything else is present (fine).**
- Every log pins dc4cb485 and prints the working copy 1d22af03 as not the pin.
- All five stderr files are 0 bytes.
- The crate is tested with the feature off and on, on both targets (test.log 3089-3116).
- The shift test runs with rowcol (test.log 1811, 2645).
- The keep regeneration in run.log exercised the validation tests (run.log 3121ff).
- The negative fixtures come from the template, with controls V0, V2 and V4.
- The logs, the report and the commit messages contain no non-ASCII byte.
- The commit messages end with Co-Authored-By only.
- This is the second and last check the rules allow.

## Question 3: does every number and quoted string in report-draft-2.md appear in a primary?

Found:
- dc4cb485 and 1d22af03: gate.log 24-25.
- 335da53, 715177a, 8b82262, 2ed9070, e79739a: gate.log and scan-pre-check.log.
- "no such part": code 1 diff.
- Two tool tests: code 3 diff.
- The four "... checks all as expected: yes" lines: the last line of each log.
- 779 / 0 / 2 on both targets: test.log 2087 and 2914.
- V1 to V8, "found once", passing unplanted and failing planted: test.log 3427-3510.
- Attempt 1 "0 bytes", exited 2, record pairs moved: the run.log header and the kept stderr.
- Attempt 2 "ended yes": the last line of the superseded run.log.
- "change BQ", and the hashes of attempt 1's log and stderr and attempt 2's script, log, stderr and records: run.log 41-50.
- 27 rows, 27 ok, 0 refused: run.log 2059 and 2167.
- 76 and 130, and 15 = 15: run.log 2121 and 2229, with the check lines.
- The record names: run.log install lines.
- The pre-check "RESULT: PASS", 0 hits: scan-pre-check.log for the second run; check 1's Q2-10 for the first.
- "committed=no": negative.log.
- The fifteenth message text: code 2 diff and the working spec.
- The Q1-1, Q2-8, Q1-6, Q1-10, Q2-6 and Q2-9 summaries: independent-check-1.md.
- "check 2 has the right diff": dfcd595d = gate.log 1582.

Could not find or verify:
1. "each RESULT: PASS, 0 hits" for "after run attempts 2 and 3": only the second pre-check is a file I have. For the first, the only evidence is check 1's answer (a primary under the rules), and no primary shows it ran after attempt 2. See Q2-4.
2. The quoted spec phrase "to the template part (created before hyperlinks when the part has one, else before pageMargins, else before the root's end)" is in the spec's P1 list, but with backticks around the element names that the report drops. The words match.
3. "15 containers created" is in run.log, but see Q2-1: it counts inputs without a container, and the check that "ties" it adds no evidence. The report's wording "ties the containers created to the tool lines ... and gives both checks a control" is accurate as a description of the script. It should not be read as closing Q2-8 in substance.

Every other number and string in report-draft-2.md appears in a primary.
