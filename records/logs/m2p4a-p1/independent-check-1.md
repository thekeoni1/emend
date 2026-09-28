# Independent check 1, M2 part 4a P1

Written by the implementing session: the checked phase commit is
e981810b880fab01acb3a52b4776fe9f54783b2f, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk, holding these files, by sha256
(the logs there are gate.log attempt 1, test.log attempt 1, negative.log
attempt 2, run.log attempt 2 and the scan pre-check attempt 2, which the
committed logs' rerun lines hash):

    1f537f78916672e0212eed541605370e902ab13a6a8bed762ed85c2e2d3b8b5a  commit-messages.txt
    dc4432b0314d76b12a16605f0f8afdb3b56e723d8b1ddae5d9b85faa0093ec6f  diff-2634ecd.patch
    c62b0fdcf663842c766f87d03c605206c2686832ba95f4830fb1890de2af4aa1  diff-bcaa31d.patch
    ccb3da9246e78c2ca0c6d234e92ba472a8e08b67fed91f119e3e3a84c96f9566  diff-commands.txt
    2d41a17a446965eb0818ce584f0152fdcb682b002f5451bd8749e0527ac683ae  diff-doc-07b359a.patch
    5ae6f322207e8b177dbb88c2e100c115c26c63024b7b4dd0c96d65c16b277324  diff-e981810.patch
    65438fdaa46d887351e89f9996b0c0742141a5b4964566f4e5744f2b7acced2e  diff-phase.patch
    6d71b57579374801cdcca3ee0aa5e0079af6b433d0d738866f261bba71f7d791  log-gate.log
    4792e152f8f51cd08a3b49065931623f5b7e80668ed6890eed54a21febddd440  log-negative.log
    d33f99e3582098bed56b8e69a7662c17bf18a7f072ea1a2d7be02971faceb38f  log-run.log
    f44a18ca119861d5e27dba042e70e685cab7722071d87230748d529422bf496f  log-scan-precheck.log
    eb0ba51fc7fd95ec2bbad6d00d2f23dd67d2e0459120a7bd43df4d8d91979df2  log-test.log
    6107e7196650b09d4d9512ade1863cfc783497b0fdf1ab85df13076265a77cc2  m2p4a-open-2026-09-28-e981810.md
    8fb909b8393ebfedd4ce9edf70d9033ed122556885ab63fc30726d99ea77e45d  m2p4a-open-2026-09-28-e981810.txt
    200c09934634116f9b938c169a80f6c01e77b36caf608ce05986307df8773e91  m2p4a-p1-2026-09-28-e981810.md
    e0dee987006fc10ee60144436d50fa3e8d17f1224e50d4c950f4de1a680957d8  m2p4a-p1-2026-09-28-e981810.txt
    fc6562689a927f445b2dd40c1d51e418fbca1b262ca64475f2c098bb48553275  report-draft.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    541003c1280e93fad39ec97767d539f1263b2799b9917156fd3c55f92bb9b4e1  specs-0018-rows-and-columns.md
    827b3e33955c108abf9f7604a8d0bdf019fc35578fdbab834fc933f872866244  specs-0019-names-links-properties-protection.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-gate.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-negative.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-run.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-pre-check.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-test.txt
    091266b4c9623a1be0041dbeaabc6ceea80212b80eeddaba23cf99854f165547  superseded/attempt1-m2p4a-open-2026-09-28-e981810.md
    8fb909b8393ebfedd4ce9edf70d9033ed122556885ab63fc30726d99ea77e45d  superseded/attempt1-m2p4a-open-2026-09-28-e981810.txt
    a4b0f8c3b3548010d7129bf91170e196705fdbb535a3054fcf478b5a4f46f146  superseded/attempt1-m2p4a-p1-2026-09-28-e981810.md
    ad2c425ee220c0d040b446b4b4acac36697029908a249a2f5b119e8dfeb51a90  superseded/attempt1-m2p4a-p1-2026-09-28-e981810.txt
    896b5a53b62e06acf8974ea5a1553af8f579b5d2a83e0ff14743b70f73b2ea57  superseded/m2p4a-p1-negative.attempt1.sh
    17c1b500b7784e420a3d1d09468feda797527c7a49a29390eefeafa950fcfaee  superseded/m2p4a-p1-run.attempt1.sh
    9930965b4bbe0e5d35c415a6db77a1c0f70907b3f348dd81199167cfe6edb57a  superseded/negative.attempt1.log
    1a71a6779205777926c473520c60f0a3aa7afb150aa0fb0dfb0c975df63d9a5c  superseded/run.attempt1.log
    6f86f6d8974a479042c4333d3d88790163cf6b396e82eedd154baa9020bb5925  superseded/scan-precheck.attempt1.log

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 45ba3e77b6730b6377604a997403bc78b37e9824ad57e3fcd55e4fbbbdefa46f,
9627 bytes; plain ASCII, nothing replaced).

---

Short answers

- **Q1:** Yes, with remarks. The four commits do what P1 and rulings 1 and 2 ask, and I found no change outside P1's scope. One gap: the spec says a created element goes "immediately before the first later root child, or before the root's end tag", but the code places it right after the last earlier child. The report does not raise this (F1). The other behaviour-neutral extras are listed in F2.
- **Q2:** Mostly yes. Every required block is present and every sibling log pins 827b3e33. The gaps: the old commit ids come in through an environment variable, not from git (F4); two plants print no assertion output (F5); the run log's spec check accepts two hashes (F6); the scan pre-check at attempt 2 has no rerun header (F7); and a few carried rules are unaddressed or open to more than one reading (F8, F9, F10).
- **Q3:** Every number, hash and quoted string was found in a primary except three items: the scan pre-check hash f44a18ca, the label "attempt 2" on the pre-check (report line 58), and the superseded location at report line 69 (F11, F12). F13 adds one remark on the old ids.

Harness note: my prompt included a CLAUDE.md, a memory index, git status and a commit-attribution reminder from the harness. I did not use any of it as evidence. I read only files in `<check>`. All 34 files match `../check1-inputs.sha256`.

Which diff I read for each "nothing else":
- **diff-doc-07b359a.patch:** ROADMAP.md, specs/0018 and specs/0019 only. The committed 0019 rebuilt from the patch hashes to 0748240e and equals the pinned spec minus the "Planning rulings during P1" section. The 0018 addition is the close-out section, and the given 0018 file is 541003c1.
- **diff-bcaa31d.patch:** the probes' USAGE strings, checked_add in both probes, and the package.rs `//!` sentence. No item-4 test, as ruling 1 says.
- **diff-2634ecd.patch:** crate files only.
- **diff-e981810.patch:** the tool's Cargo.toml, main.rs and null_rewrite.rs only.
- **diff-phase.patch:** the 15 files, which are the union of the four diffs above.

Findings

**F1 (gap, Q1).** The creation rule is implemented as the part 6 reading, not the spec's literal text.
- Spec: specs-0019 lines 390-393 say "immediately before the first root child ... whose name comes later ... or before the root's end tag".
- Code: diff-2634ecd.patch lines 635-642 and 951-955 say "immediately after the end of the last root child ... that WORKBOOK_CHILDREN puts before it, or after the root start tag".
- Where they differ: whitespace, comments or foreign-namespace children between two children. Test `a_created_container_goes_in_schema_position` (diff-2634ecd.patch lines 1342-1377) pins the code's reading. The created element goes before `"\n<extLst>"` and before `"<!-- c --><q:x .../>"`, where the spec's text would put it after them.
- The run log's independent check uses the same reading (log-run.log lines 1936-1941 and `after_last_before` at line 2185), so it cannot catch a departure from the spec's wording.
- The code reading is arguably forced by specs-0019 lines 402-403 (calcPr bytes on existing tests do not change, and test `an_absent_calc_pr_goes_after_the_last_of_the_nine`). So the spec text is internally inconsistent.
- report-draft.md lines 79-80 mention only the out-of-table rule, not this. Planning should rule.

**F2 (remark, Q1).** Behaviour-neutral extras in the diff, beyond the "one core change":
- `text_of` made `pub(crate)` (diff-2634ecd.patch line 23).
- A `regions()` and `BookEdits` refactor that is compiled in the core (lines 852 onward).
- A `#[cfg(test)]` wrapper that keeps the old `rewrite_workbook` name for the part 6 backstop test.
- Part 1's test gated with `cfg(not(feature = "metadata"))`, body unedited (diff-2634ecd.patch tests.rs hunk).

All are needed by P1 or are neutral. log-test.log lines 2814-2870 show the part 6 calcPr test bodies are byte-identical and pass, and the feature-off runs pass 512 tests.

**F3 (remark, Q1).**
- Spec "32-bit and streaming" (specs-0019 line 435) says every index is u32. The name ordinals and counts are usize: diff-2634ecd.patch lines 97, 505, 686, 930 and 999.
- The template "else before the root's end" case in P1's test list (specs-0019 lines 620-621) can never happen: a name edit always creates calcPr on the template. No test covers it and the report does not say so.
- The comment "both containers' names are read" (diff-2634ecd.patch line 1653) cannot be shown by that test, because its second container is empty.

**F4 (gap, Q2).** The gate's rewrite block takes the old ids from REWRITE_OLD (log-gate.log lines 173-175 and 343).
- They are typed by the operator, not read from git by a command the log prints. specs-0001 lines 191-192 require that for every commit id in a log.
- The invocation line (log-gate.log line 2) does not show REWRITE_OLD.
- The check shows only that each old id is a commit with an equal tree and is not an ancestor. It does not show the old ids were the phase's own pre-rewrite chain: no parent chain, no message comparison.

**F5 (gap, Q2).** Plants N1 and N5 print only the FAILED line and the panic location, with no assertion output.
- Locations: log-test.log lines 3008-3009 and 3057-3058. The `failure_lines` filter (line 825) drops the message line.
- specs-0001 lines 203-209 require "the failing assertion's output". N2, N3, N4 and N6 do print theirs.

**F6 (remark, Q2).** The run log's spec check accepts either the uncommitted hash or the committed blob (log-run.log line 141, change AJ, and line 1884). So it could pass on the committed spec. The hash printed is 827b3e33, so the pin holds in this run.

The scan's `pin_count` counts lines holding the hash anywhere, printed-script lines included (log-scan-precheck.log, "every sibling log pins the spec's hash"). A log whose only copy of the hash is its script's default value would still pass.

**F7 (gap, Q2/Q3).** log-scan-precheck.log has no rerun header (lines 1-50; RERUN_NOTE unset). It does not name attempt 1 (6f86f6d8), its FAIL, or its hash. Only log-negative.log (lines 34-38) and log-run.log (lines 28-36) hash the superseded pre-check.

**F8 (gap, Q2).** specs-0019 lines 552-553 ask for "calcPr as --formula-sample's check has it, exactly when the input has a chain or a formula".
- The comparison expects `fullCalcOnLoad="1"` in all 27 workbooks, whatever their chain or formulas (log-run.log line 1942 onward, and the rows from line 2475), which follows decision 2 A.
- No log prints whether an input has a formula.
- Neither the log nor the report addresses the conditional clause. Planning should rule whether it is obsolete.

**F9 (gap, Q2).** Carried rules possibly unmet:
- specs-0018 lines 607-608 carry "the unbound-prefix case in the first negative log". log-negative.log is part 4a's first negative log and has no unbound-prefix case. specs-0019's P1 text (lines 637-640) does not ask for one; P2 does.
- specs-0019 lines 636-637 say "every later log runs the crate's tests with the feature on". log-negative.log builds and tests nothing (line 39).

Both are for planning to rule.

**F10 (remark, Q2).**
- log-negative.log line 56 labels the pinned spec "with P1 ruling 1", but the file at 827b3e33 carries rulings 1 and 2.
- log-scan-precheck.log line 47 uses `<scratch>` for the phase's disk directory, which the other logs call `<phase>`. specs-0001 lines 177-178 keep `<scratch>` for scratch locations.
- log-gate.log line 91 and log-negative.log line 49 print ANTHROPIC_API_KEY as "no" with no check line. log-run.log does check it.

**F11 (defect, Q3).** report-draft.md lines 58-59, "the pre-check attempt 2 (f44a18ca...)":
- f44a18ca appears in no primary. It does equal the sha256 of log-scan-precheck.log in the check inputs list, but a log cannot hash itself.
- "attempt 2" is not stated in that log (see F7).

**F12 (defect, Q3).** report-draft.md line 69 says every attempt-1 file is kept under `<home>/.cache/emend-build/m2p4a-p1/superseded`. log-run.log lines 37, 45 and 65 put the superseded run directory and keep under `<scripts>/out/superseded`, which is `<home>/.cache/emend-build/m2p4a-p1/scripts/out/superseded`. No log states where the attempt-1 negative log, scripts, scan log or records are kept; they are hashed by basename only.

Also, line 64 calls negative.log's undeclared token a "path". It was the token `<placeholder>` in a comment (superseded/scan-precheck.attempt1.log line 591; log-negative.log line 34 says "token").

**F13 (remark, Q3).** report-draft.md lines 23-24 cite the old ids 50473b6, d73f183, cb493bf and 1902a06. All four appear in log-gate.log lines 59-71. P1 ruling 2 (specs-0019 lines 809-810) says the old ids appear in the gate's rewrite lines "and nowhere else". The ruling names logs and records; whether the report counts is for planning.

Every other figure in report-draft.md was found in a primary:
- The commit ids and the three doc blob hashes: log-gate.log.
- 9 PASS and the test counts 512/523: log-gate.log lines 2189-2272.
- 720/0/2 and the six plants: log-test.log.
- The negative messages and the case variant: log-negative.log lines 820-900.
- 816/45/9/0, 10 of 10, 160, 11, 27, 367, 336 and 0/0: log-run.log lines 2466-2670 and 2943-3580.
- The four record hashes: log-run.log lines 2832-3589 and log-scan-precheck.log lines 1721-1751.
- 6f86f6d8 and the 2 structural failures: superseded/scan-precheck.attempt1.log lines 606-608.
- Changes G and AM: log-negative.log line 119 and log-run.log.
- 94cf3b6 and the old ids: the spec and log-gate.log.
- The design-choice sentences and the test name: diff-2634ecd.patch.

No private corpus value was seen in any file.