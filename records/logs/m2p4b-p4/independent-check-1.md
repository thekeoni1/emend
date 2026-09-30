# Independent check 1, M2 part 4b P4

Written by the implementing session: the checked phase commit is
5854e34bed600e8fb2952cd22f9cc77acc4142f1, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk (its prompt named it
<phase>/check1 and asked for <home> and <repo> in place of machine paths),
holding these files, by sha256. The logs there are the six this phase
committed, each at its first attempt, and the scan's pre-check stage, which
the final scan.log's rerun lines hash:

    9b7535231471c59cd3442757c63f6e7c1d905c8fa4f32875e9eba625676b8acb  commit-messages.txt
    6e467bb95823328e52da9f67d4f0dd5cabe263805495b4d5e18027a39bd9167b  diff-3e48f99.diff
    f169eeb414021b894f7eb7caebd3c72d5460b546db5c0d59e16d45706f558204  diff-51c32da.diff
    b0fba99186a40846ac16d5f74ed23def67f33a19a75a74a6e1981b85a69f4df2  diff-5854e34.diff
    25702da570f4253e18d0e3fa88116e95304aaceb08ed0889f91b51c236914bcd  diff-phase-dd1a415..5854e34.diff
    6e5a863668e00a1ce2d09ee8e69118c6cbac10157224cc3fd9aa3ead30e68731  doc-commit-2679b59.stat
    c53ede6f70eecfe9dde2428e50da677be81ca4d87d43b3ba52feae3dd3cdfd75  gate.log
    d2e4468fedbe69e4b5f239e409d70f24e01484f329c5eb3e9cf9355d9bd7018a  m2p4b-open-2026-09-30-5854e34.md
    e339a8748c98f789c3c23466ec4cf3514d9297378c13d390ef48e3992dc77fe1  m2p4b-open-2026-09-30-5854e34.txt
    2fc36ee038cd5781537df64dbbc139965ecc8dcc61b6a2a4fecead121a025a7e  m2p4b-p4-cf-2026-09-30-5854e34.md
    c50d4cf22a3f2be2b13e36c550c65dd33b6df63dcaa55b2b72021c82bd00c45c  m2p4b-p4-cf-2026-09-30-5854e34.txt
    ad1a8e7c79d88b4d428aed2f82d77ed2b7816f4068dfc1a6f60a31f81b0b9958  m2p4b-p4-scale-2026-09-30-5854e34.md
    8383e9cfe3ea1b50bc3dc667dfd06e8ec029ae45519b87028843275ba3c97b01  m2p4b-p4-scale-2026-09-30-5854e34.txt
    f85051a2102b3f21775f1683f16c4487104f5f3851eb8764d724002abe7df564  m2p4b-p4-validate-2026-09-30-5854e34.md
    ec362be1c3d47329da36d47013ffb2d44d79899308ca3d3cc65f09990db7a4ff  m2p4b-p4-validate-2026-09-30-5854e34.txt
    9f003450a3a3078e87b977a5cf9d0d3e74774ecf88c865789351cb896a610e12  negative.log
    8e584eacd3db8abdd63c7b31a2bb1f457fcdf2837659d39aaaed0ad9efc5aba2  report-draft.md
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

The per-commit diffs and the other derived inputs were made by these
commands, in the repository (specs/0020 P1 verification ruling 6):

    git diff 51c32da^ 51c32da > diff-51c32da.diff
    git diff 3e48f99^ 3e48f99 > diff-3e48f99.diff
    git diff 5854e34^ 5854e34 > diff-5854e34.diff
    git diff dd1a415 5854e34 > diff-phase-dd1a415..5854e34.diff
    git diff dd1a415 2679b59 --stat > doc-commit-2679b59.stat

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 8be6d0e09a3c1ac1833ec171b280323a92e5f175aab176985c5a45f560d3c236,
13430 bytes; plain ASCII; no machine path; nothing replaced; one line feed
added at its end).

---

I read the CLAUDE.md the harness put into my context but did not use it as evidence. Everything below comes from files in <phase>/check1.

**1. Does the diff do what the spec (with the rulings) says for P4, and nothing else?**

1.1 Diffs read. Each "nothing else" below names the per-commit diff it rests on. I also rebuilt the phase diff's +/- lines from the three per-commit diffs plus the doc commit's two files. They match line for line. diff-phase-dd1a415..5854e34.diff holds exactly six files: ROADMAP.md, package.rs, tests/visual.rs, specs/0020, size-probe/Cargo.toml and the new probe. spec-0020-committed.md hashes to 2b4afb62..., the pin.

1.2 Doc commit 2679b59. Read from the phase diff and doc-commit-2679b59.stat. The only ROADMAP change is the P3 row set to DONE. The spec file is at the pin (gate.log "doc1's specs/0020 ... is the blob planning's message gave: as-expected"). Fine.

1.3 First code commit 51c32da against P3 verification ruling 2 (diff-51c32da.diff). It removes `str::from_utf8(prefix).unwrap_or_default()` and writes `color_element("color", color).element(prefix)`, so the prefix is spliced on as bytes. It adds `a_prefixed_root_gives_its_prefix_to_the_child_its_points_and_its_colours`, which checks `x:colorScale`, `x:cfvo` and `x:color`. Nothing else is in the diff, and it is the first code commit. Fine, with two observations:
- (a) The new test would also pass at the base. The prefix "x" is valid UTF-8, so the old conversion wrote the same bytes. The test covers the prefix rule, as the ruling asks, but it cannot tell the carry's change from the old code. No test can: only a non-UTF-8 prefix would show the difference, and ruling 2 says no well-formed name produces one.
- (b) test.log plants Z2 (cfvo) and Z3 (end tag) go in code the carry did not change. Only Z1 targets the carry's own lines.

1.4 Catalogue commit 3e48f99 (diff-3e48f99.diff). It is `//!` lines only (gate: "lines added or removed 85; of them not a //! line 0"; my count is 87 including the two file headers). Its extra scope, rewriting the styles-limits lines that said "`dxfs` ... never touched", is needed because that text had become false. I accept it as part of "the limits". Claim by claim:
- **What the calls touch.** Matches "Design". "The other containers' counts" is P2 ruling 3's stated consequence. "Refused with the two-container message rather than written into" matches "Creating an element" and P2 verification ruling 2. Fine.
- **Cell edits compose.** "A cell edit in the same write composes with them" cites only `validation::a_validation_and_a_cell_edit_on_one_part_compose`. For conditional formats no test name in test.log shows it. The dxf case has `dxf::a_cell_style_and_a_dxf_in_one_write_both_go_in`, which is not cited. The claim covers more than its citation.
- **A formula is text.** Fine against "Areas and formulas". "A value point's `val` hold what the caller gave, escaped, without a leading `=`" reads as covering Num/Percent values, which are numbers. The wording is loose.
- **One validation per cell.** "Drops every such element" goes beyond the spec: "remove_validation" speaks of "the `<dataValidation>`" in the singular, and "The write" skips each kept match. "Never an addition of the same write" is also not stated in the spec. Neither is borne out by any test name I can see. The closest is `validation::a_removal_drops_its_list_in_order_and_the_last_drops_the_container`, whose body is not in my evidence.
- **Titles 32, texts 255.** Fine.
- **Priority rule.** "The main namespace's rules only; an `x14` rule's is not counted" is P2 ruling 5. Fine.
- **Differential styles.** Index, bgColor (decision 5 A), no deduplication, no removal: fine. But "A number format, an alignment and a protection are not written" understates the spec. Those fields are refused at the call with "a differential style holds a field this part does not write", and a caller could read "not written" as silently dropped. The refusal of a style with none of the three set is also not mentioned.
- **Visual rules and x14.** Fine against decision 2 A, "remove_conditional_format" and "Out of scope".
- **What is read back.** Fine against decision 3 A and P2 ruling 6.
- **Excel check pending.** Present. Fine.
- **Part 4 message citations (P3 verification ruling 4).** The entries for "a text holds a character XML cannot carry" and "a formula text starts with an equals sign" now cite the validation, conditional and visual call-refusal tests. All three names exist and pass in test.log (2 ok lines each). Whether those tests actually assert these messages cannot be checked from my files, since the test bodies are not in any diff.

1.5 Probe commit 5854e34 against "Size" (diff-5854e34.diff). It adds `rules = ["emend","xml","emend/rules"]` and one `[[bin]]` with `required-features = ["rules"]`. The probe does what "Size" lists, in order:
- after the format probe's work, a Whole Between validation with formula_one and formula_two from the command line on rule_area;
- a dxf of a bold font and a solid fill whose one colour is `background` (consistent with decision 5 A);
- CellIs GreaterThan of formula_one with `Some(dxf)` and priority `None`;
- a two-point ColorScale;
- after the write, `validations` and `conditional_formats` counts printed.

Nothing else is in the diff (the gate's area check passes). Two things I could not check:
- "probe-emend-format-write-xml's work, unchanged": the format probe's source is not in my files. size.log gives indirect evidence that it holds: "its first output is the fullstyles probe's first output, byte for byte", and the printed lines match the format probe's expectation plus the two new lines.
- The dxf colours and the scale colours are constants, not command-line values. "Size" does not require them from the command line, and the probe header does not claim they are.

1.6 The committed spec is pinned, but the run log implements P4 ruling 1, which exists only in the uncommitted working-tree spec. The working tree also rewrites P2 verification ruling 1 in place (spec lines 1110-1116), not just the appended section. P4 ruling 1's own text authorises both ("P4's logs pin 2b4afb62 ... committed with the close-out's doc commit"; "P2 verification ruling 1 ... is corrected"). This is recorded as consistent, not a defect.

**2. Do the logs contain every line the spec requires?**

2.1 Size figures, recomputed from the numbers size.log prints:
- Feature cost: 1016556-968440 = 48116; file 1022500-973348 = 49152; .text 794467-753459 = 41008. All correct.
- Core growth: 968440-968440 = 0, with both the phase-commit and 3e67745 builds printed (file 973348/973348, .text 753459/753459). Its 3e67745 build reproduces part 4a's sha 883da056... Correct.
- Whole growth: 968440-967884 = 556. File 973348-973348 = 0; .text 753459-753171 = 288. Correct.
- Budget: 968440-533216 = 435224; 458752-435224 = 23528; file 973348-539172 = 434176; .text 753459-411155 = 342304. Change against part 4a's figure: 0. Correct.
- Side figures: metadata 50404/53248/41648, rowcol 35488/40960/32976, dates 926628-921804 = 4824, 4096, 4384. All correct.
- Page remainder: 473636 mod 4096 = 2596. Correct.

2.2 Seventeen probes classified. size.log has "the probes measured are part 4a's seventeen and the new one, in that order: yes" and "new: probe-emend-rules-write-xml". Verdicts: identical 4, code 13. The record is installed and its sha printed (72d24c37...). That matches my hash of size-m2p4b-2026-09-30-5854e34.md. Present.

2.3 A minor wording inconsistency in the size record. It says "crates/emend/src was touched by 10 commits". The log's per-probe cause counts say 11 for every emend probe. Those also count the size-probe manifest commit 5854e34, so both numbers are right, but the record gives only the 10.

2.4 Negative log:
- Five constants are reached through the binary with controls: VALIDATION_OVERLAP (V1), TWO_VALIDATIONS (V3), RULE_EXTENDED (C1), RULE_UNPARSED (C3, and again S3), TWO_DXFS (C5).
- Ten are stated unreachable with reasons, and a set check shows reached plus unreachable equals the 15-message block.

Findings:
- (a) The unreachable list is not only the at-call refusals. NO_SUCH_VALIDATION, NO_SUCH_RULE and DXF_OUT_OF_RANGE are write-time and are listed with reasons. The report's "the at-call refusals stated unreachable" understates this.
- (b) RULE_UNPARSED is run twice (C3 and S3), not "once".
- (c) The set check covers only the fifteen part-4b constants. Reused refusals reachable through the binary with a fixture are neither run nor stated unreachable. Examples: "the workbook has no styles part" for `--cf-sample` (P2 verification ruling 4), part 3's index message for a past-the-grid M, and the unbound-prefix refusal. The spec's parenthetical, "the fixture cases of P1 to P3", arguably limits the scope, so this is a note, not a failure.
- (d) Both clauses of VALIDATION_OVERLAP and RULE_UNPARSED are reachable. Only the overlap clause and the priority clause are run.

2.5 Run log:
- All three samples run on both targets with the release binaries whose sha256 matches size.log's `hash:` lines (the check reads them by command).
- The three phase run directories are named and checked against planning's.
- For every sample and target: "27 of 27" byte-identical by sha256, tool lines, diff tables and entries tables the same, 0 refusals.
- The records are installed with hashes matching my files, and the edit open check runs on the host keep.

Present. Findings:
- (a) Checks implied by others. Once each output is byte-identical to a phase run that planning already verified, every comparison flag (14 for cf, 10 for validate) and "every output is the same on both targets" follow from that identity. They add no independent evidence beyond the P4 ruling 1 change.
- (b) Controls that could not bite on the corpus. The P4 ruling 1 change is controlled only for `dxfs_tag` (`--tag-test`, with the withdrawn reading planted: "tag_control=no"). `dxfs_count`'s new branch (`bool(dx_in) and not had_count`) has no control. By the spec's own facts, no corpus input has a `dxfs` without `count`, so no corpus input reaches either changed branch. The report's "with its control" is accurate only for `dxfs_tag`.

2.6 Carried rules:
- Every log pins 2b4afb62 and prints the working copy b1607ca9.
- No log is a rerun (RERUN_NOTE unset in all six).
- ANTHROPIC_API_KEY is unset.
- Commit messages are ASCII with only the Co-Authored-By line, each saying the diff alone carries its counts.
- No gate file changed, and the gate lists them.
- The three shape greps run with fixtures. The phase commit is compared with HEAD, with null-commit controls. Each script prints itself and counts its own stderr as 0.
- The stderr/ files are all 0 bytes.
- My scan found no control bytes, no non-ASCII, and no absolute path outside script literals or controls. The scan's final stage is not in my evidence (only scan-pre-check.log: "total hits: 0", "RESULT: PASS").

2.7 Counts printed that no check reads. In gate.log the "f_office" line for each modified file is printed with no check after it; the other five forms each get one. The size log's per-section differing-byte counts are informational by design.

2.8 Weak checks. "the rules probe is larger than the format probe, by sections" constrains almost nothing, and the reported 48116 is not compared with the spec's estimate by any check. Harmless.

**3. Does every number and quoted string in report-draft.md appear in a primary?**

Found:
- 2b4afb62...; b1607ca9...
- dd1a415, 2679b59, 51c32da, 3e48f99, 5854e34
- 808 / 0 / 2 on both targets; Z1 to Z3
- 48116, 49152, 41008; 968440 - 968440; 3e67745; 556; 968440 - 967884; 435224, 458752, 23528, 0
- records/size-m2p4b-2026-09-30-5854e34.md
- "27 summary rows" (as "27 rows"); the three run directories; the record names
- "RESULT: PASS", 0 hits; the five "... checks all as expected: yes" lines
- the estimate 40000 to 60000 (the spec)
- dxfs_tag and dxfs_count

Not found in any primary:
- 3.1 "the script and checker as dry-run are kept under <home>/.cache/emend-build/m2p4b-p4/superseded/pre-ruling1/". No log names "pre-ruling1" or a dry run. run.log's inputs say "superseded run directories named: none; superseded keep named: none".
- 3.2 "the run log's cf comparison was changed before the real run". run.log's header describes the ruling-1 comparison but nowhere says it was changed after a dry run. It is unsupported as a statement of sequence.
- 3.3 "each 27 summary rows, 27 ok". The log says "27 rows; every workbook ok but the phase run's refusals" with refusals 0. That is equivalent, but "27 ok" is not a printed string.
- 3.4 "the at-call refusals stated unreachable". Inaccurate rather than missing: three of the ten stated unreachable are write-time (2.4 a).
- 3.5 "plants Z1 to Z3 on the prefix carry". Z2 and Z3 plant code outside the carry's diff (1.3 b).

**Summary**

The diff does P4 and nothing else. All size figures add up. Every log ends "all as expected: yes". The substantive findings are:
- two catalogue claims no test is shown to bear out (1.4: "drops every such element" and "never an addition of the same write");
- the loose "not written" wording for the refused dxf fields (1.4);
- the P4 ruling 1 change: `dxfs_count`'s branch has no control, and no corpus input reaches either changed branch (2.5 b);
- the report's unsupported dry-run and superseded-directory sentence (3.1, 3.2).
