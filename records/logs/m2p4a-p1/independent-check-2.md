# Independent check 2, M2 part 4a P1

Written by the implementing session: the checked phase commit is
e981810b880fab01acb3a52b4776fe9f54783b2f, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check2 in this phase's folder on the disk, holding these files, by sha256
(the logs there are gate.log attempt 3, test.log attempt 2, negative.log
attempt 3 and run.log attempt 3, as committed, and the scan pre-check
attempt 3, which the final scan.log's rerun lines hash):

    1f537f78916672e0212eed541605370e902ab13a6a8bed762ed85c2e2d3b8b5a  commit-messages.txt
    dc4432b0314d76b12a16605f0f8afdb3b56e723d8b1ddae5d9b85faa0093ec6f  diff-2634ecd.patch
    c62b0fdcf663842c766f87d03c605206c2686832ba95f4830fb1890de2af4aa1  diff-bcaa31d.patch
    ccb3da9246e78c2ca0c6d234e92ba472a8e08b67fed91f119e3e3a84c96f9566  diff-commands.txt
    2d41a17a446965eb0818ce584f0152fdcb682b002f5451bd8749e0527ac683ae  diff-doc-07b359a.patch
    5ae6f322207e8b177dbb88c2e100c115c26c63024b7b4dd0c96d65c16b277324  diff-e981810.patch
    65438fdaa46d887351e89f9996b0c0742141a5b4964566f4e5744f2b7acced2e  diff-phase.patch
    df5b877002af9a93d246a07760ddad38f29e5c9b38a2183e67d49bbed210f8d1  independent-check-1.md
    3d7cdaa4f62f4db0d8bb0ada5f870c0dbe9fc4cdafc87f969454315a3dd2252e  log-gate.log
    b5a2fb754d647cc74a5fde44017293e15714b94481796484851a2d81704a69fc  log-negative.log
    c7c4620fec550d56a9cb644d70d70c8cedb2a9ab590ad6e43d0f44fb04141627  log-run.log
    d712fb70b31446332ff9c0f81da9cd196e4106f25e08e5638b089cff55883514  log-scan-precheck.log
    d6bb68d758dad585c6d8a67ed808538a4f5ff12a51647958888fcae065d6ff05  log-test.log
    a1772c9d26df00cb3c6f13b2476885ac0fc82180b66e9ae54048a59ee4e11126  m2p4a-open-2026-09-28-e981810.md
    8fb909b8393ebfedd4ce9edf70d9033ed122556885ab63fc30726d99ea77e45d  m2p4a-open-2026-09-28-e981810.txt
    c6b3f1d22347c8d87651209b0d8b553971fb9cb892eaa8945c2dad8c47321a76  m2p4a-p1-2026-09-28-e981810.md
    a0ef79f346de2a8b96c48f6b70052d2feb2e7cba9792cce21ea51afc44489602  m2p4a-p1-2026-09-28-e981810.txt
    98a49684e3b1d2bc493c427f7dc5279b15bb9d07913804ea85d9b072d71504a4  report-draft-2.md
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
    6107e7196650b09d4d9512ade1863cfc783497b0fdf1ab85df13076265a77cc2  superseded/attempt2-m2p4a-open-2026-09-28-e981810.md
    8fb909b8393ebfedd4ce9edf70d9033ed122556885ab63fc30726d99ea77e45d  superseded/attempt2-m2p4a-open-2026-09-28-e981810.txt
    200c09934634116f9b938c169a80f6c01e77b36caf608ce05986307df8773e91  superseded/attempt2-m2p4a-p1-2026-09-28-e981810.md
    e0dee987006fc10ee60144436d50fa3e8d17f1224e50d4c950f4de1a680957d8  superseded/attempt2-m2p4a-p1-2026-09-28-e981810.txt
    6d71b57579374801cdcca3ee0aa5e0079af6b433d0d738866f261bba71f7d791  superseded/gate.attempt1.log
    d09ad6a83ade4e4bdf88914b8cc026b0606ffde3139d8f2010b42a1a2ee3f82e  superseded/gate.attempt2.log
    85f6b28d6d2cabbe112cb56a8358a5d7057cef386a57a5fd24a5c139b442ef8f  superseded/m2p4a-p1-gate.attempt1.sh
    fe401064d3265bd544791f7d14bec56366e57716208a58f64a11fd84978763ec  superseded/m2p4a-p1-gate.attempt2.sh
    896b5a53b62e06acf8974ea5a1553af8f579b5d2a83e0ff14743b70f73b2ea57  superseded/m2p4a-p1-negative.attempt1.sh
    80ff3cc401063ae29ac7e0943009c4c36a9f282ab3593f09a4afedd48072bf22  superseded/m2p4a-p1-negative.attempt2.sh
    17c1b500b7784e420a3d1d09468feda797527c7a49a29390eefeafa950fcfaee  superseded/m2p4a-p1-run.attempt1.sh
    6572136ea73e2970d41ab434c518f62b47acce0b688855ee3a24a8b789c7b979  superseded/m2p4a-p1-run.attempt2.sh
    8528c24ed813e1bf73f7a0994454f33a790406abb9c5abeecb0607bf6eef72c6  superseded/m2p4a-p1-test.attempt1.sh
    9930965b4bbe0e5d35c415a6db77a1c0f70907b3f348dd81199167cfe6edb57a  superseded/negative.attempt1.log
    4792e152f8f51cd08a3b49065931623f5b7e80668ed6890eed54a21febddd440  superseded/negative.attempt2.log
    1a71a6779205777926c473520c60f0a3aa7afb150aa0fb0dfb0c975df63d9a5c  superseded/run.attempt1.log
    d33f99e3582098bed56b8e69a7662c17bf18a7f072ea1a2d7be02971faceb38f  superseded/run.attempt2.log
    6f86f6d8974a479042c4333d3d88790163cf6b396e82eedd154baa9020bb5925  superseded/scan-precheck.attempt1.log
    f44a18ca119861d5e27dba042e70e685cab7722071d87230748d529422bf496f  superseded/scan-precheck.attempt2.log
    eb0ba51fc7fd95ec2bbad6d00d2f23dd67d2e0459120a7bd43df4d8d91979df2  superseded/test.attempt1.log

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 91a37a51bfa4cd504894dce3e194c8cf75bd78ac9aa371d6c135459f7725728b,
11342 bytes; plain ASCII, nothing replaced).

---

Short answers

- **Q1: Yes, with remarks.** The four commits do what P1 and rulings 1 and 2 ask. I found no change outside P1's scope in any diff. Three open points remain: F1 (where a created element goes), which the report carries; one behaviour of the new names read that the spec does not state (G3); and one spec-level remark on the name rule (G4).
- **Q2: Mostly yes.** Every line P1 requires is present. Every rerun line's hash matches its superseded file, and each described earlier result matches what that attempt printed. The reruns fix F4, F5, F7 and F9's first half. F10 is fixed only in part: the scan log still names the phase's directory `<scratch>` (G1). F6 was neither fixed nor carried (G2).
- **Q3: Yes, with four exceptions.** Every number and hash in report-draft-2.md is in a primary. Three claims have no primary, and one quotation differs from the spec in case and backticks (G9).

Harness note: my prompt included a CLAUDE.md, a memory index, a git status snapshot, a user email and a commit-attribution reminder. I used none of it as evidence. I read only files in `<check>`. All 52 files match `../check2-inputs.sha256`. In this answer I refer to the four pre-rewrite commit ids without writing them.

**Which diff I read for each "nothing else"**

- **diff-doc-07b359a.patch:** ROADMAP.md, specs/0018 and specs/0019 only.
  - Rebuilding the committed specs/0019 from the patch gives sha256 0748240e. That equals the pinned spec with lines 780-812 removed (the "Planning rulings during P1" section), and nothing else differs.
  - The specs/0018 hunk appends only the close-out section. Its blob, 541003c1, is gate-checked.
- **diff-bcaa31d.patch:** items 1 to 3 of close-out ruling 4 only:
  - the two USAGE strings;
  - checked_add in both probes;
  - the //! sentence in package.rs.
  - No item-4 test, as ruling 1 says.
- **diff-2634ecd.patch:** crates/emend only. The core-side changes are the four listed under F2 (`text_of` made pub(crate), the regions/BookEdits refactor, the cfg(test) wrapper, part 1's test gated) and WORKBOOK_CHILDREN replacing BEFORE_CALC_PR. Everything else is behind `metadata`.
- **diff-e981810.patch:** the tool's Cargo.toml, main.rs and null_rewrite.rs only.
- **diff-phase.patch:** 15 files, the union of the four diffs above.
- The gate hashes of the four per-commit diffs and the phase diff equal the sha256 of the check files (log-gate.log lines 1642-1653).

**Rerun verification (Q2)**

I extracted each printed script from the log that printed it:
- Each superseded log's printed script equals the superseded .sh file, except for a trailing newline.
- The gate attempt 3 script equals the attempt 2 script ("the script as this one", log-gate.log line 88).
- The three scan scripts are identical.
- The script diffs between attempts are exactly the changes each rerun line names: AF (test), G, H and I (negative), AM and AN (run), M3 and N3 (gate).

Every hash in every rerun block matches its file in superseded/:
- gate lines 89-93;
- test lines 50-52;
- negative lines 35-39;
- run lines 29-37;
- scan lines 45-47.

Each "printed yes / ended no / FAIL / PASS" claim matches the superseded log:
- gate.attempt2.log: line 2379 is "no", with exactly one UNEXPECTED check line, the status check at line 2371.
- scan-precheck.attempt1.log: lines 1865 and 1867 are "no", and it ended "RESULT: FAIL".

Findings

- **G1 (gap, Q2).** F10 is only partly addressed, but report-draft-2.md line 73 lists it as addressed.
  - Fixed: the negative log's spec label (log-negative.log line 59) and the API-key checks with controls (log-gate.log lines 107-109, log-negative.log lines 50-52).
  - Not fixed: log-scan-precheck.log still declares `<scratch>` as the phase's directory (lines 2 and 51-52). specs-0001-bootstrap.md lines 177-178 reserve `<scratch>` for scratch locations, and the other logs call this directory `<phase>`.
  - The scan script is byte-identical in all three attempts, and the report neither fixes nor carries this.
- **G2 (gap, Q2).** F6 is neither fixed nor carried.
  - log-run.log lines 1888-1890 still accept the uncommitted copy or the committed blob (`spec_ok`, printed script line 431). The value it compares against, SPEC_SHA, comes from the environment (line 180), and the invocation at line 2 does not show it.
  - The scan's `pin_count` (log-scan-precheck.log lines 1713-1716) still counts printed-script default lines: log-gate.log line 298, log-test.log line 229, log-negative.log line 161.
  - The report's claim "every P1 log pins it" (line 6) holds only in the sense that each log prints 827b3e33.
- **G3 (gap, Q1, for planning).** With `metadata` on, `Package::workbook` now reads every definedName's attributes and text: `read_defined_name` resolves attributes (diff-2634ecd.patch line 1105) and the text goes through `text_of` (line 1049).
  - So a workbook with an unbound attribute prefix on a definedName (negative N7, log-negative.log lines 907-946), or with a text the cell reader refuses, is refused at the read. This applies to every caller that enables the feature, even one that edits no name.
  - The spec moves only the localSheetId refusal to the read (specs-0019 lines 173-177). Its Errors section reuses the unbound-prefix refusal only "wherever an element this part retags carries one" (line 429-430).
  - The report (lines 120-122) says no crate test pins this refusal, but not that it now happens at the read.
- **G4 (remark, Q1, spec-level).** Names that are a single `R` or `C` are accepted. names.rs pins both as good (diff-2634ecd.patch lines 1594-1595), which follows the spec's literal R1C1 clause (specs-0019 lines 185-186). Excel is not known to accept R or C as a name. Planning may want to rule before P4's Excel check.
- **G5 (remark, Q2).** The rewrite block in log-gate.log (lines 56-86) compares trees and subjects, not message bodies, authors or dates.
  - So nothing shows that the tool commit was "rewritten only in its parent" with its message unchanged (specs-0019 line 811), which report lines 23-24 state.
  - The old tip comes from a reflog entry whose message the operator supplied (log-gate.log line 57). Git recorded it, but the reflog is local and expires.
- **G6 (remark, Q2).** Gate attempt 2's failure was cleared by moving the records aside, not by fixing the cause. The gate's status pattern still does not name the open-check record stem (log-gate.log line 88; superseded/gate.attempt2.log line 2370). Any later gate run with the open-check records present will fail the same way.
- **G7 (remark, Q2).**
  - log-run.log's invocation (line 2) shows only positional arguments. SPEC_SHA, RUN_SUFFIX, SUPERSEDED_RUN, SUPERSEDED_KEEP, RERUN_NOTE and RERUN_FILES come from the environment (printed script lines 180-233). Each value's effect is printed, but none is labelled as environment input. This is the same shape as F4, outside the rewrite block.
  - The gate's feature-off public list (log-gate.log lines 2051-2200) is an awk simulation over the source, reading one attribute line per item, not a compiled listing. The consumer crate (lines 2318-2361) covers the four gated items.
- **G8 (remark, Q1, for P4).**
  - bcaa31d changes probe-emend-format-write-xml's source (checked_add and two error strings). P4's "core's growth for the part (the format probe at the commit minus 967884)" (specs-0019 lines 461-463) will therefore include probe-side bytes, not only core bytes.
  - The Workbook struct gains two pub(crate) fields under the feature (diff-2634ecd.patch lines 90-97). I could not tell from the diff whether Workbook was constructible outside the crate before, so whether this touches "Nothing existing changes signature" is undetermined.
- **G9 (defect, Q3).** Items in report-draft-2.md not found in a primary:
  - (a) Line 8, "nothing pushed", and line 128, "No push": no log checks push state.
  - (b) Lines 82-83: the other superseded files are "kept on disk in the phase's directory". No primary names their location. Their hashes do match the files given to me in superseded/.
  - (c) Lines 23-24: "the tool commit in its parent alone". This is ruling 2's wording, but no log shows it (see G5).
  - (d) Lines 106-107 quote "the bytes calcPr produces on every existing test do not change". Spec line 402-403 reads "The bytes `calcPr` produces ...": the case and backticks differ.
  - Everything else was found:
    - 827b3e33, the four commit ids, 67d156ad, 541003c1 and 0748240e: log-gate.log.
    - 94cf3b6: the spec and commit-messages.txt.
    - 9 PASS, 512 and 523: log-gate.log lines 2233-2316.
    - 720, 0 and 2 on each target: log-test.log lines 1859 and 2627.
    - The six plants with their messages: lines 3004-3083.
    - The negative messages: log-negative.log lines 860-950.
    - 27, 816, 45, 9, 0, 10 of 10, 160, 11, 367, 336, and 0 twice: log-run.log lines 2466-2680 and 2942-3580.
    - The four record hashes: log-run.log lines 2832-2840 and 3580-3590.
    - 6f86f6d8, f44a18ca, "RESULT: PASS", 0 hits and 0 structural failures: log-scan-precheck.log lines 44-47 and 1880-1885.
    - "RESULT: FAIL" and 2 structural failures: superseded/scan-precheck.attempt1.log lines 1865-1881.
    - Changes G and AM: the printed scripts.
    - The design-choice sentences and the test name: diff-2634ecd.patch.
    - F1, F3, F8 and F9 quotations: specs-0019 lines 390-393, 402-403, 435, 552-553, 620-621 and 636-637.
- **G10 (remark, Q3).** Report line 28 cites no pre-rewrite id. But independent-check-1.md line 129 quotes all four, and it is a check answer the record commit will carry verbatim. Ruling 2 says the old ids appear in the gate's rewrite lines "and nowhere else" (specs-0019 lines 808-810). Planning should say whether committed check answers count.
- **G11 (remark, Q2).** "Each rerun log's rerun lines hash the attempts before it" (report line 78-79) holds transitively only. log-run.log lines 29-37 do not hash attempt 1's records or the scan pre-check's attempt 1; superseded/run.attempt2.log lines 29-36 hash those, and log-run.log hashes that log.

**Check 1's findings, one line each**

- F1: carried to planning (report lines 102-109).
- F2: neither fixed nor carried. It was a remark needing no fix, and the report's design choices (lines 95-98) list two of its four items.
- F3: carried (report lines 110-114, with the proposal at lines 120-122).
- F4: fixed. log-gate.log reads the old ids from the reflog (lines 56-86), and the invocation needs no environment (line 2).
- F5: fixed. Each plant now prints its message line (log-test.log lines 3024 and 3074, and the others).
- F6: neither fixed nor carried (G2).
- F7: fixed. log-scan-precheck.log lines 44-47 hold the rerun header and hashes.
- F8: carried (report lines 115-117).
- F9: first half fixed (negative N7 and N8, log-negative.log lines 907-950); second half carried (report lines 118-119).
- F10: partly fixed. The label and the API-key checks are fixed; the scan's `<scratch>` naming is neither fixed nor carried (G1).
- F11: fixed. f44a18ca and "attempt 2" now appear in log-scan-precheck.log lines 44-47.
- F12: fixed in the report's wording (lines 80-83). No primary states where the superseded logs, scripts and records are kept (G9b).
- F13: fixed in the report (line 28); independent-check-1.md line 129 still carries the ids (G10).

I saw no private corpus value in any file.