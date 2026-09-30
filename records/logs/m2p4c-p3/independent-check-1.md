# Independent check 1, M2 part 4c P3

Written by the implementing session: the checked phase commit is
3c5f0b73b7df20d1f5d97283a42ede5424ac5577, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk (its prompt named it
<phase>/check1 and asked for plain ASCII), holding these files, by sha256.
The logs there are the five this phase committed (gate, test, negative and
size at their first attempt, run at attempt 2) and the scan's pre-check
stage (attempt 2), which the final scan.log follows:

    3cdcbb866f7053fbf282a5837abb1f2367991128353cc09704487e699b92f96f  commit-messages.txt
    37b53efb679aac63081f9cad5c76d22bc17fe047cd136e49f5535a6b65286fc7  diff-3c5f0b7.diff
    6248cafc19acb27e7b9dc9878fa14d3b84015af6ab48ce52b281ffcc94ade5cf  diff-b7e341c.diff
    6eeb83e39ccb949aa0a991dcf8313aedcb72d07104b91e714dd3eafdade0188b  diff-f70495e.diff
    7434bf9b168576a1ec82678039d31060fe74dc3aecf097e2f5b3bf01ce9f3c42  diff-phase-af43c51..3c5f0b7.diff
    ca41f123c727b112f380661982ed36e9e205b894ef293cc22d30ae42bea9479f  doc-commit-1b8fe5a.stat
    104749a716d7ca929e9d84ae92e4ad84dcfe7ee13e0f98e30f2b712933596b7c  gate.log
    b992d7948adda0661b82a1b5065462aa650d467dddf35192e2a5a53900535a01  m2p4c-open-2026-09-30-3c5f0b7.md
    545616db3f342ee771f2d7c62335e04460a34d967af9723a5faafa8fb80dfdf9  m2p4c-open-2026-09-30-3c5f0b7.txt
    9b0a717e08a5247338f55b963b4725ba6cc5b1fb29004475ccbb9f386f5afcc3  m2p4c-p3-2026-09-30-3c5f0b7.md
    7eb9429315f3ec2b283bfc1d600525b30971155613efd587ba21744d44263cd8  m2p4c-p3-2026-09-30-3c5f0b7.txt
    40d9c8e77804112ad78a6833f2d59fb7112f2b48914263109716709baa4fbdf6  negative.log
    712a8b5ad3144df5bcde97f81c09551a7e696ae2a2bb476c5024a0ca6ac16c15  report-draft.md
    e02d50a9615af4eda14720df3257d03303398857b8395579bef16085fe960182  run.log
    e9958d34297f7b5d25b81b8d90e236a677512ad50532da86555fb4fab1045cb2  scan-pre-check.log
    fe014459635a931543c86ed4c5441edd1cd95d25913942a5bf704e9d579cb6f1  size-m2p4c-2026-09-30-3c5f0b7.md
    b9790c6f0f6a5fe2ed16272a6f66650c71d955e4e8c9c13e04ac48507a578525  size.log
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  spec-0001-bootstrap.md
    61c125d0d4b51aa93182a8b99567060d452b293e68d03afc3cab0aba72570ca1  spec-0021-committed.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/size.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/test.stderr
    cd0dd023a04fb455779efb7a46185a26af8e752a8fbfc3cf9446f7c9430ededc  test.log

The answer below is the subagent's final message, verbatim. It is ASCII as
written (raw answer, kept on the disk as check1-answer.raw: 13310 bytes, sha256
a69d0c132260283f894c6dd4a3be56c1c7234aac1480e2cbdf0ead63981feef7); no character was substituted.

---

**Independent check 1, M2 part 4c P3, phase commit 3c5f0b7**

I read only the files in <phase>/check1. For each code commit I read its own diff: <phase>/check1/diff-f70495e.diff, <phase>/check1/diff-b7e341c.diff and <phase>/check1/diff-3c5f0b7.diff. For the doc commit's changes I read the combined diff <phase>/check1/diff-phase-af43c51..3c5f0b7.diff. The spec at <phase>/check1/spec-0021-committed.md hashes to 61c125d0d4b51aa9..., the pin every log gives. None of the six stderr files has any content (all 0 bytes).

## Question 1: does the diff do what the spec says for P3 and ruling 5's carry, and nothing else?

**Yes, on the whole. I found nothing done outside the spec.**

**Doc commit 1b8fe5a** (combined diff, lines 1-13 and 166-246)
- It carries only ROADMAP.md (P2 row marked DONE) and specs/0021-tables.md (the "Planning verification and rulings on P2" section, plus the "The calls" wording from ruling 2).
- gate.log lines 1700-1728 match both blobs to planning's hashes (61c125d0..., e487c725...) at the doc commit and at HEAD.
- Minor, not a finding against the diff: <phase>/check1/doc-commit-1b8fe5a.stat says "specs/0021-tables.md | 63 +++...", i.e. 63 insertions and 2 deletions. The combined diff's hunks for that file ("@@ -245,7 +245,10", "@@ -827,6 +830,64") give 62 and 1. The net (+61) agrees. I could not tell from the directory which diff options made the stat. The report does not cite the stat.

**Code commit 1, f70495e: the carry** (diff-f70495e.diff). It touches only crates/emend/src/package/tests/tables.rs, and makes each of ruling 5's three edits:
- Line 9-10: the `//!` now cites "(P1 ruling 7 (B))" in place of "(P1 verification ruling 5)".
- Lines 18-42: the removal test gains `ONE_ROW` (a cell A1) in its sheetData, and its expected worksheet now holds `{ONE_ROW}`.
- Lines 50-66: a new test `a_removal_on_a_removed_or_replaced_part_is_refused` asserts "an edit to a removed part" and "a cell edit to a part this write replaces".
- Nothing else. gate.log confirms: "code commit 1 touches: crates/emend/src/package/tests/tables.rs"; "the edits typed here: 11; the edits read: 11".

**Code commit 2, b7e341c: the catalogue** (diff-b7e341c.diff, lines 75-149). Only `//!` lines in package.rs; gate.log reports "lines added or removed 64; of them not a //! line 0". The spec's P3 items are all there:
- The feature is marked: "Tables add these, all the Cargo feature `tables`'s".
- The shape written (lines 124-132) and what is read back (lines 140-145).
- The never-written list (lines 133-139): "a totals row ..., a calculated column, a custom style ..., a sort state, ... the `xr` namespace and its `xr:uid`".
- P2 verification ruling 2's rels-part out-of-scope sentence (lines 104-106).
- The same-write added-sheet statements, from P1 ruling 2 and P2 ruling 2 (lines 110-113).
- The message list's test citation gains the carry's new test (lines 80-81).

**Code commit 3, 3c5f0b7: the probe** (diff-3c5f0b7.diff)
- Manifest: the feature `tables = ["emend", "xml", "emend/tables"]` and the `[[bin]]` entry, nothing else (gate.log: "the probe manifest's only non-comment changes are the tables feature and the probe's [[bin]] entry").
- Probe source: `add_table` on `AB10:AC12` with `TableStyle::Medium(2)`, row stripes only (lines 375-393). The two column names come from argv (lines 102-103, 161-164). After the write, `written.tables(sheet)` is read back and printed as `tables` (lines 444, 195).
- The size log shows the result: "tables 1" (size.log, around line 5200, section "the nineteen probes on generated packages"), the part's bytes, and the header cell AB10.

**Weak points for question 1**
1. **"Unchanged" format work cannot be checked here.** The spec says the probe does "`probe-emend-format-write-xml`'s work, unchanged". That probe's source is not in the directory, so I could not diff the copy against it. The evidence is indirect: size.log expects "the format probe's lines with its own third output's size and written two higher, then the one" and reports "exits 0 and prints exactly the expected twenty-nine lines: yes".
2. **The probe doc contradicts itself.** It says "Every other name, part, reference ... and area comes from the command line", yet `TABLE_NAME` and `TABLE_AREA` are constants. The spec asks for a fixed area and says nothing of the name (the report's stated reading 1). It is harmless for the measurement, but the doc sentence is loose.
3. **"Marked as the feature's" for the ten-message list.** That list (package.rs around line 476) predates this phase. The diff context does not show its heading, so I cannot confirm from the directory that the list itself carries the feature marking. The new section does.
4. **One catalogue sentence is broader than the spec.** It says "a later `set_cell` on a header cell wins and a header cell holding a formula is refused". The spec refuses only a header cell whose *kept* `<c>` holds an `<f>`. A reader could take the catalogue to mean a caller's later formula on a header cell is also refused.
5. **One omission in the catalogue.** It leaves out that the entry-name comparison is ASCII case folded (spec "The part").

## Question 2: do the logs hold every line the spec requires, and does each end all as expected?

**Yes, as far as the pre-check stage goes.**

**Spec pin and endings.** Every log pins 61c125d0:
- gate.log line 23, test.log line 24, negative.log line 63, size.log lines 4 and 68, run.log lines 4 and 1685, scan-pre-check.log lines 8 and 51.

Each log ends as expected:

| Log | Final line |
| --- | --- |
| gate.log | "gate log checks all as expected: yes" |
| test.log | "test log checks all as expected: yes" |
| negative.log | "negative log checks all as expected: yes" |
| size.log | "checks not as expected: 0" / "size log checks all as expected: yes" |
| run.log | "run log checks all as expected: yes" |
| scan-pre-check.log | "RESULT: PASS" |

**size.log, per "Size"**
- 19 probes built; 18 compared with part 4b's record. The new one is listed: "new: probe-emend-tables-write-xml".
- Each rebuild at 5854e34 "reproduces part 4b P4's log's sha256: yes".
- Verdicts: "identical: 6", "build id and panic-location line numbers: 11", "code: 1" (probe-emend-metadata-write-xml, "moved: file 0, sections 32, .text 32").
- The figures, around lines 4279-4295:

| Figure | Log line |
| --- | --- |
| Feature's cost | "sections 46860, file 49152, .text 39520" |
| Core's growth | "sections 968440 - 968440 = 0", both values printed; "above 2000 section bytes ...: no" |
| Whole growth since b728fdf | "968440 - 967884 = 556" |
| Budget | "sections 435224", "23528 under the ceiling", "this one less it: 0" |

- Cause control: the manifest cause is listed apart ("commits that touched the size-probe manifest between the two measurements: 1"). The control line reads "control: a code verdict with no source cause fails the accounting, the manifest cause (1) notwithstanding: yes".
- The size record at <phase>/check1/size-m2p4c-2026-09-30-3c5f0b7.md, lines 104-135, carries the same figures.

**negative.log, per P3**
- It runs through the gate's binary, whose hash it matches (806c1fcf...).
- It reads the ten messages from source. Four are reached, each with a control:
  - TABLE_OVERLAP by N1, N7 (merge) and N8 (autofilter)
  - TWO_TABLE_PARTS by N3
  - TABLE_NAME_IN_USE by N5 and N9 (defined name)
  - TABLE_UNREADABLE by N6
- The six unreachable ones are each stated with a reason and a named test, and the log confirms each test's fn holds the message. It closes: "reached and stated unreachable together are exactly the source block's constants: yes".

**run.log, per P3 and "Harness runs"**
- It uses size.log's release binaries, hash for hash (lines 1697-1712).
- 27 of 27 ok on both targets, with "lo ok 27; lo_in ok 27".
- Against P2's run directory, named by P2's record commit af43c51 (lines 2386-2394): "every output byte-identical to P2's run's for the same workbook, by sha256: yes", with the tool lines, diff tables, entries table and summary (but for wall_s and peak_rss_kb) the same, on both targets.
- Flag plants: "plants: 12; plants that turned their own flag: 12". The filled-row data_rows plant on 75387404b8af gives "data_rows=no".
- The keep was compared against P2's open record, and the edit open check ran: "no output refused on an accepted input" for the crate and the tool. The open record is installed at the reported hashes (lines 3753-3761).

**test.log**
- Workspace: 832 on both targets.
- One test added, none removed: "added: package::tests::tables::a_removal_on_a_removed_or_replaced_part_is_refused".
- Crate tests: 512 with the feature off, 532 on, both targets.
- Plants S1 and S2 each pass unplanted and fail planted.

**Weak points for question 2**
1. **Only the pre-check scan is here.** The final scan.log stage does not exist yet, so I cannot check that it comes last or passes.
2. **Plant S1 shows only the removed-part half biting.** Planted, the test panics at tables.rs:984:26, which is the first (removed-part) refusal. The replaced-part half of the new test is never shown to bite on its own.
3. **Plant S2's premise is unmeasured.** Its description says the dropped rows are something "which the carry's cell meets", implying the pre-carry test would not catch the plant. The plant was never run against the pre-carry test, so that is not measured. The report does not repeat the claim.
4. **The code-verdict accounting is coarse.** Any commit under crates/emend/src counts as a source cause for every probe that links emend, and 6 did. The metadata probe's +32 is therefore "accounted" without naming which commit caused it. The control shows only that a verdict with zero causes fails, so a wrong attribution could not fail this check.
5. **Reused messages were not tried through the binary.** The negative log treats "every message of the part" as the ten new constants. The spec's "Errors" section also lists reused messages, such as the override rule's "an override for that part already exists". That one looks reachable through the binary with a fixture holding an `Override` for /xl/tables/table1.xml but no entry, since the entry-name allocation looks at archive entries. The log does not attempt it and does not state it unreachable. This is arguably within the spec's wording, but it is a gap planning may want to rule on.
6. **The keep's new test adds no pair.** The new test writes nothing, so "tests with a new pair: 0". That is correct, but the open check says nothing about it.

## Question 3: does every number and quoted string in report-draft.md appear in a primary?

**Every number and quoted string appears in a primary, except one unverifiable claim below.** I found no wrong figure.

The table below lists the items I checked and where I found each one.

| Report item | Where it appears |
| --- | --- |
| Hashes 61c125d0, e487c725 | gate.log lines 1700-1728 |
| Log hashes 104749a7, cd0dd023, 40d9c8e7, b9790c6f, e02d50a9, e9958d34 | sha256 of the files as delivered |
| Record hashes 9b0a717e, 7eb94293, b992d794, 545616db, fe014459 | sha256 of the files as delivered; run.log lines 3753-3761 for the open record |
| "(64)" | gate.log "lines added or removed 64" |
| 512 / 532 | gate.log and test.log |
| "eleven" | gate.log "the edits typed here: 11" |
| 832 | test.log |
| S1 and S2 descriptions | test.log plant table; S1 paraphrased from "a table removal is not among the parts a write edits" |
| "4 of the part's 10", N7, N8, N9, "the 6 unreachable" | negative.log |
| Nineteen / eighteen; verdicts 6 / 11 / 1; +32 | size.log |
| 46860, 49152, 39520, 0, 556, 435224, 23528, 458752, "1 commit" | size.log and the size record |
| 27 of 27, 2026-09-30-6cfd504, "wall_s and peak_rss_kb" | run.log |
| 12 of 12, 75387404b8af, "one changed pair" | run.log |
| Finding 1 text and "0 bytes" | run.log lines 8 and 46-49 |
| Finding 2: "every sibling log pins the spec's hash: 5 of 4", change BM | scan-pre-check.log lines 6 and 53 |
| probe_table, AB10:AC12 | diff-3c5f0b7.diff lines 72 and 75 |
| 30000 to 45000 | spec line 70 |

**Items no primary supports or that I disagree with**
1. **Stated reading 2 cannot be verified.** The report says "no committed test pins that case. I confirmed it with a throwaway test that I did not commit." No log prints that throwaway test or its run, and the directory does not hold the whole test file, so I cannot check that no committed test pins it. Under specs/0001 ("Nothing is called measured unless the log prints the measurement"), "I confirmed it" is not a measurement. The catalogue sentence it supports (diff-b7e341c.diff lines 110-112) therefore rests on reasoning, not a pinned test.
2. **The attempt-1 wording is looser than the log's.** The report says "(attempt 1 was refused at its entry guard, below)" and, in finding 1, "0 bytes". The log (run.log line 48) shows the 0 bytes is the .log file; the kept .stderr is 80 bytes. The report says "its stderr" is kept, without its size. That is consistent with the log, just loose.
3. **"at the first attempt" rests on an unset variable.** For gate, test, negative and size, it is supported only by "RERUN_NOTE: unset" in each header. No primary can prove that no attempt was left unrecorded. The same was true in earlier phases.

I found no number in the report that contradicts a primary.
