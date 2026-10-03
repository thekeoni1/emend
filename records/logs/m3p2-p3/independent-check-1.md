# M3 part 2 P3, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, size.log, run.log, the scan's pre-check, each one's stderr, the phase's seven records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p2-p3. Checked commit (the phase commit): 0c3e4faee512b48065b1c2bfa54218210a1320a3; HEAD when the check ran: 0c3e4faee512b48065b1c2bfa54218210a1320a3. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 1739558c71e43d24b586ba43464d9ea7b65940089a79da48247f9583f64b6588. Its items on the report are corrected in the report; its observations on the code and the logs are reported to planning. No second check was run.

Inputs and their sha256:

- fe973908f917cb7049c8ba56c453618ec51825e0427f8816b9739b5b06dbf738 code-commit-1-carry.diff
- 76536234eb1d558966a062e57610b137ebf6b6bd09a0a3fa446a31cd6749062f code-commit-2-catalogue.diff
- 2ba55ea68111f701733c3f0ab7ee73b3dbdedbeab93fe884e55d2b2f9d4fd794 code-commit-3-probes.diff
- 43f65b42e8defff8b7fb3826ccb3e5147f22525b8cc65c7af4c5a9b365ae81e3 commit-messages.txt
- 09dce1f73f35e658c540274b9dbef4d4fe914922ad6514f19174e760f14dc8e4 doc-commit.diff
- 6b29359b7cfed7139e0e94c1d85f7594f83b9a76e48369d7aa1a5747539bc779 gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- 31e32ae3598be5af9c27efcab28865463f1064a5f8d81ba8799f22758c0dbe10 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- 0f11049e280bd3ff642a4fc2a374d66566de0293408f5ff8b4f8d2999841481e phase.diff
- 4d930c7b31069da78db88ca390bc2c6c63a242648222eca390af2a3e7ef181cf records/m3p2-open-2026-10-03-0c3e4fa.md
- 76feea849f9c707985ebb0c264024fd6f48ea70db0c13130c164e2bbed33fe7c records/m3p2-open-2026-10-03-0c3e4fa.txt
- bd91e40bef182125a76f71e95389416e2bf18be27b83467cd85008b4547d09b5 records/m3p2-p3-richcomment-2026-10-03-0c3e4fa.md
- 32c0b92041b018550ae600d4550a6ae95df8274f1f00b28c4540256818a0eea1 records/m3p2-p3-richcomment-2026-10-03-0c3e4fa.txt
- 4ca12ec16a228ff4c82c12602624b4b3f59b88e683a91b62388e8ca0b2deb54c records/m3p2-p3-richtext-2026-10-03-0c3e4fa.md
- 56490fddcdef31c5c8645816cda24a5f071383611020e1cf46ae028bbc0d0ce4 records/m3p2-p3-richtext-2026-10-03-0c3e4fa.txt
- a8bd470844e32ec5f2b68757c84623d9945253459fcd928ce68433da348e1eb8 records/size-m3p2-2026-10-03-0c3e4fa.md
- 04f330d7a4e3b94ac0f9dc1a1824253e31b0b4e435e2df84629c2cc0064a742c report-draft.md
- bcf0fbbf9db6872f1ffe4826b746effa2372020b831ffec2e57b314e28ed2488 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- 5b3b750faade29820ef0427f7c021d4df8afcfdf88028acc7ba55897b1e2d2d6 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 04c2fb8a5c8a259b0f5f6170a7487673189fe8ab814633ac8492cde0d7b85fa8 size.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 size.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- ca2e40a7467271df645defb025e76ee15fec7e03da54720b177b25bf5a17df11 spec-0024-committed.md
- e2cabc5ea1e2ce77fd70a0080250437fda81fbdb68de077b9d1c1fa9a74acb08 test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

---

I'm the P3 independent checker. Everything below comes from the files in <phase>/check1. The harness also put the project's CLAUDE.md and the user's memory index (MEMORY.md) in my context; I used neither as evidence. I ran no cargo command and changed no file. My only commands were sha256sum, grep, sed and a line-set comparison of the diffs.

Hash check: every log, record, diff and spec hash the report gives matches sha256sum of the file in <phase>/check1. That covers spec-0024-committed.md ca2e40a7, gate 6b29359b, test e2cabc5e, negative 31e32ae3, size 04c2fb8a, run bcf0fbbf, scan-pre-check 5b3b750f, and the seven records. The four commit diffs and phase.diff also match the hashes at gate.log 1978-1990. All six stderr files are 0 bytes.

## 1. Does the diff do what the spec says for P3, and nothing else?

Yes, as far as these diffs can show, with the gaps below.

**phase.diff is exactly the four commit diffs.** I took the sorted +/- lines of phase.diff and of doc-commit.diff plus the three code-commit diffs, and the two sets are identical. The phase touches 7 files (gate.log 1992-2001).

**Doc commit (doc-commit.diff): nothing else, confirmed.**
- It changes only ROADMAP.md and specs/0024.
- In ROADMAP.md only the P2 row changes, PENDING to DONE (doc-commit.diff 9-10).
- The spec blob is ca2e40a7 and ROADMAP is 021cdad3, the hashes planning gave (gate.log 1948-1976).

**Carry, code commit 1 (code-commit-1-carry.diff): nothing else, confirmed. This is P2 verification ruling 5.**
- The catalogue sentence now names `Package::comments` "with the Cargo feature `comments` too" (lines 14-19), and the bullet adds `comments::rich::a_comment_s_run_properties_that_cannot_be_read_are_refused` (line 27). That test passes on both targets (2 ok lines in test.log).
- The refusal test gains a `Color::Theme { index: 1, tint: Some(1.5) }` case, refused with "a style field is out of range" (lines 39-53).
- Ruling 5 also asks that nothing is recorded. The new case sits in `the_call_refuses_each_rule_and_records_nothing`, whose shared body presumably checks that. The body is not in the diff, so I cannot confirm it from primaries.
- The gate confirms 14 //! lines in package.rs and 15 test lines (gate.log 2120-2126).

**Catalogue, code commit 2 (code-commit-2-catalogue.diff): nothing else, confirmed.** All 80 changed lines are //! lines (gate.log 2121, 2124). Against P3 and P2 verification ruling 4 the coverage is good:
- **Written shape:** lines 101-111.
- **Never written:** lines 112-117. This covers the shared strings table, the five unmodelled properties, the phonetic run and the empty run.
- **Read drops:** lines 127-136. This covers the tint beside rgb or indexed (P1 verification ruling 5), the trailing `<t>`, `<r/>` and empty runs (P1 verification ruling 4 and P2 verification ruling 4), and the `CommentEntry::text` phonetic difference.
- **Leading plain `<t>` read as the only run:** lines 123-124.
- **`Package::comments` refusal with both features on:** lines 143-145.
- **Three earlier sentences and the shared strings hold now point to the block:** lines 65-67, 80-89 and 164-166.

Departures and claims I cannot check against code:
- **(a) No measured cost.** P3 asks for "the runs table's mechanism and its measured cost". Lines 146-152 give the mechanism and the contract, but only point to "M3 part 2 P3's run log" for the measure. No figure is stated, such as the six peak deltas (+84 to +200 kB) or the 2048 kB tolerance. That is a sequencing artifact (d07793a predates the run log), but the catalogue as committed does not carry the measured cost.
- **(b) Claims about cells.rs, which is not in any diff.** Three cannot be checked against code here:
  - "found by binary search" (line 147);
  - "the reader's listener allocates nothing until an `<r>` is met" (lines 148-149);
  - "one this layer does not know or holding an element" (line 139).
- **(c) Internal wording conflict.**
  - Lines 118-122 say each read is `Some` "when the container holds at least one `<r>` with text".
  - Lines 123-124 then say a container whose only text is a leading plain `<t>` reads as one run. That container has no `<r>` with text, yet reads `Some`.
  - P2 verification ruling 4 describes the second case; the first sentence as written excludes it.
- **(d) vertAlign is not corpus-measured.** Line 107 lists vertAlign inside "the order the corpus measured". The spec's fact (spec 91-101) puts vertAlign in `Font::compose`'s order, not the corpus's. This mirrors spec line 182, so it is consistent with the spec but not strictly accurate.
- **What I can confirm.** The message sets in the read-drops and cannot-read lines match what negative.log 1128-1148 reads from source: `RichText::check` returns RICH_TEXT_INVALID and TEXT_NOT_XML, and `Font::check` plus `check_color` return FONT_NAME_REFUSED and FIELD_OUT_OF_RANGE.

**Probes, code commit 3 (code-commit-3-probes.diff).**
- The manifest adds `richtext = ["emend", "xml", "emend/richtext"]` and two [[bin]] blocks, with required-features ["richtext"] and ["comments", "richtext"] (lines 13, 22-30). That matches "Size" and decision 1 A. The gate confirms these are the only non-comment manifest changes and there is no lock change (gate.log 2131-2167).
- **richtext probe, as "Size" describes it.** It calls `set_cell` at AB10 with a `Value::Rich` of two runs: the first bold with the command-line font name, the second with no font (lines 1108-1131). It then reads `runs_of` over the output table and `Cell::runs` (lines 1162-1186).
- **richcomment probe, as "Size" describes it.** It calls `add_rich_comment` at AC10 with two runs, the first bold (lines 401-425), and reads back through `comments(...).runs` (lines 478-482).
- **"Nothing else" cannot be confirmed for the rest of the two new files.** The base probes' sources are not in the directory, so I cannot verify the claim at report line 17 and commit-messages.txt 111-112 that each probe is "generated from its base probe's source with each change applied exactly once".
- The only indirect evidence is behavioural. Each probe's first and third outputs are byte-identical to the fullstyles and sheets probes' outputs, and its printed lines equal the base probe's plus the additions (size.log 6831-6849, 6913-6933).

## 2. Do the logs contain every line P3 requires, and do their checks support "all as expected: yes"?

Every required item is present, and each log's checks back its final verdict, with the weaknesses below.

**size.log has every figure "Size" lists:**
- the feature cost 12672 (5606);
- the pair cost 14820 (5608);
- the core growth with both figures, 968936 - 968568 = 368 (5609-5611);
- the budget figure 435720 against 458752 (5614-5617);
- the classification of the 21 probes: identical 6, code 15 (5553-5577, 6970);
- the two new probes listed as new (4826);
- the cause control and the manifest-cause listing (5060-5066, 5132-5134);
- the record checked against the printed figures (7148-7152, 7154-7157).

Weaknesses in size.log:
- **(a) The pair cost has no check.** 14820 is printed but never checked against the spec's expected 5000 to 15000 (spec 57); no check line names that range. The only check is "richcomment probe is larger than comments probe" (5620). The figure is 180 bytes under the top of the range, and the report (line 41) does not say it is within it.
- **(b) The cause check cannot fail for an emend probe.** "if the verdict is code, a commit that touched one of its source causes is named above" passes for every emend probe, because each has the same 6 crates/emend/src commits (5067). Two of those six (7c1e11e, d07793a) change only docs and tests and cannot move a binary. The accounting does not tell a probe whose code moved from one whose source causes are only doc commits. Controls at 5136-5138 show the form itself can fail.
- **(c) `sst_rich 0` is fixed by construction.** The fixture's table has no rich item and the write never appends one, so `sst_rich` is 0 regardless. The only output control plants `rich_runs 0` (6833). A `runs_of` that always returned None would pass the probe run unchanged. negative.log R2C (`sst_rich_before=1`, line 1308) shows the tool's `runs_of` works, but not the probe's.
- **(d) "Does not move by design."** The spec's Goal (spec 42-43) and "Size" (spec 307) say the core probe and budget do not move by design. They moved 368. This is within the 2000 threshold, which the log checks (5611, 5630). Nothing flags it as a departure from "by design"; the report's attribution (line 91) is labelled as a reading.

**negative.log:**
- Every message of the part reachable through the gate's binary is reached, each with a control:
  - RUN_PROPERTIES_UNREADABLE three ways: R2 table, R3 inline, N5 kept comment (1305-1340, 1436);
  - the index message through both samples (R1, N4; 1435).
- The binary is the gate's hashed debug build, not a rebuild (1077-1083).
- Six messages are stated unreachable, each with a reason and a named test whose body holds the message (1441-1482).

Weaknesses in negative.log:
- **(a) "Errors" shared messages left unaccounted.** Three messages from "Errors" (spec 265-272) are printed as "not accounted" and left to "earlier parts' negative logs" (1171), with no test named: part 1's removed or replaced part messages, part 2's unbound-prefix refusal, and Error::Xml for non-UTF-8 bytes. P3 asks for "every message of the part reachable there". The unbound-prefix refusal on a kept `<rPr>` child is plausibly reachable through these samples and is not tried.
- **(b) Unreachable messages are checked by text, not execution.** For example, FIELD_OUT_OF_RANGE for the comment call is covered by `the_call_refuses_each_rule_and_records_nothing`, which already held that message before this phase. The check could not tell whether the new tint case exists.

**run.log:**
- **Three read runs, as "Harness runs" and decision 4 require (5598-5741):**
  - both labels, `--memory 2048`, TMPDIR at <scaletmp>;
  - output sha256 equal to input sha256;
  - tool line byte-identical to part 1's;
  - peak RSS within 2048 kB, with controls (5621-5623).
- "On the disk" for TMPDIR is asserted by a root line (75) but never measured.
- **Both samples are byte-identical to P1's and P2's runs:** richcomment 27/27 on both labels (4171-4240), richtext 27/27 on both labels (4242-4375).
- **Records and the open record:** the record pairs and the open record are installed (5588-5596); the keep has 665 pairs, 1 new and 0 changed (4697-4750); the open check shows crate 590 and tool 75 pairs, 0 outputs refused on an accepted input, and no P2 verdict moved (5556-5574).

P2 verification ruling 6, item by item:
- **Planted-flag controls on created-parts and appended-parts workbooks:** present, 28 plants (run.log 4099-4163).
- **EMEND_KEEP_DIR run:** present, 755 names and 1180 files (test.log 4076-4091).
- **Failing test's name printed:** present (test.log 4473).
- **43 against 42 explained:** present, by the rowcol cfg (test.log 4093-4104).
- **Change list corrected:** present (run.log 478-485).
- **Weakness, flag plants:** each flag plant shows only that its own flag turned to no (for example 4129, 4159). No line shows the other 13 flags stayed yes, so "each plant fails at its own check" is shown only one way.
- **Weakness, test plant:** X1 fails at rich.rs:421 `.expect_err("refused")` (test.log 4475). That line is shared by every case of the table loop, and the panic message "refused: ()" does not name the case. "Distinct assertion lines: 1" for 1 plant (4477) cannot fail.
- **Mislabel:** run.log 2514 and 5761 call the committed spec "planning's uncommitted spec" or "planning's" copy, but git status shows no modified spec (5753-5760, scan-pre-check.log 2489-2503). This is harmless.

Forbidden shell shapes: I re-ran the scan's three patterns (scan-pre-check.log 989-991) over every file in the directory and got 0 hits in each. Every `rm -rf` in the logs is a ruled `${VAR:?}` form or a committed scripts/ line (scan-pre-check.log 2571-2596).

## 3. Does every number, hash, commit id and quoted string in report-draft.md appear in a primary?

These I could not find, or find inaccurate or mislabelled:
- **Line 17 and commit-messages.txt 111-112:** "Each is generated from its base probe's source." No log or diff in the directory verifies this.
- **Line 92:** "The scan's PLANNING_SPEC: FACTS said to give the working copy's hash ... The scan fork found that its control then fails on a clean tree."
  - Neither the planning message called FACTS nor any record of the fork's finding is in the directory.
  - The mechanism can be inferred from the scan script: with PLANNING_SPEC set equal to the committed copy, `planning_spec_ctl` returns 1 (scan-pre-check.log 1106-1109), and the script says to leave it unset (731-733).
  - The sentence is not labelled as the implementing session's working state, as P1 verification ruling 7 asks.
- **Line 19 ("first attempt"):** supported only indirectly, by RERUN_NOTE and RERUN_FILES unset in each log's header. No log states an attempt number.
- **Line 41:** the report quotes the spec's 5000 to 15000 but does not say 14820 lies within it, and no log checks it (see section 2).
- **Line 55 ("P2's 43 against 42"):** the figures printed here are this phase's (test.log 4094, 4100). The label "P2's" refers to where ruling 6 found them; this is slightly ambiguous, not wrong.

Everything else in the report traces to a primary:
- **Lines 3 and 7:** ca2e40a7 and 021cdad3 (gate.log 1948-1976).
- **Commit ids:** 7e14eab, 7c1e11e, d07793a, 0c3e4fa and ea4a090 (gate.log 1876-1895).
- **Size figures, lines 40-49:** 12672, 8192, 11168, 14820, 12288, 13232, 968936, 968568, 368, 435720, 458752, 435352 and 23400 (size.log 5602-5617), 6 and 15 (6970), 576 (5655), 30000 to 45000 (spec 54).
- **New probe lines:** sst_rich 0, rich_runs 2, comments 2 and rich_comment_runs 2 (size.log 6796-6912).
- **Test figures:** 513, 532, 546, 574, 926, 0, 2 (test.log 2877, 3852, 3991-4055); 755 and 1180 (4087); the rowcol cfg (4095).
- **Negative messages:** as quoted (negative.log 1440-1447).
- **The six RSS rows and their differences:** each recomputed (run.log 5734-5741).
- **27/27 equality** (run.log 4235-4240, 4370-4374).
- **Keep and open check:** 665, 1, 0 (4697-4746); 590, 75, 0 (5566-5569).
- **Record hashes:** a8bd4708, 4ca12ec1, 56490fdd, bd91e40b, 32c0b920, 4d930c7b and 76feea84 (sha256sum; scan-pre-check.log 2405-2475).
- **Scan:** "RESULT: PASS" and "structural checks failed: 0" (scan-pre-check.log 2628-2631).
- **6578925** (size.log 5587).
