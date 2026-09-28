# Independent check 2, M2 part 4a P2

Written by the implementing session: the checked phase commit is
cced544f57b8027193de004ec785d72c614ad208, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check2 in this phase's folder on the disk, holding these files, by sha256
(the logs there are the four committed logs, each attempt 1, and the only
scan pre-check; p1-scan.log is P1's committed scan.log; no log was rerun
after check 1):

    f6c888eb30707436478a2042c02d91b01b1305620bc2827cf2f7f334f4cb1629  commit-messages.txt
    ecb698a11a1836798f74730d42257c669032776e98895de7df77344388e6a5d3  diff-4e62e76.patch
    54a587f1a3bf860b801c3aac2d820524ea32ff542af4fcfbfebbb9889441dddb  diff-a791e2c.patch
    4061bb2570cc9e64fa3a4391331e8e3dd84551df3c407372f307a9e44daaccae  diff-cced544.patch
    e29fffc71a86d74d42c53fe96abc940af1410757a63d74698422b778aead2754  diff-commands.txt
    6e1c02863da7c288d45f1d44216ac835107bc5ee1107d0cd4dc6005f46af558d  diff-doc-95e5fe3.patch
    5b7ba024d57b6fab2fc0d3c6230e1c6328fe8f9ef527b050908ab240dd7077ab  diff-phase.patch
    5cf65589a44b8774226d29e8f957a714939f0f36f55a33a61c7640468b295f51  independent-check-1.md
    3ed9c21efb3bd9eb66d4a2c65288c23c6b1457012e55d3788ca5003ed443e5fb  log-gate.log
    0cc1ec7bc96c1e403657d05c06ac04ef66d102ebeacd33bb8ebe1852fb13cd0a  log-negative.log
    f4dfa2797970a115a9420adb634b49f6f09a7d9383c37027cc5140cdd101ff75  log-run.log
    bea1e63fda74d314f078984e223f5b6e0c42ae849b28edf44f32394485c4549d  log-scan-precheck.log
    2ee6c3382cd0b10da10e3b27bacd0031677218a467e819a4566f7c688cba59fd  log-test.log
    5320e5be6ccd74d0ae31016256ad8a82d8e0a7a925cd30e13e4700ace27b1280  m2p4a-open-2026-09-28-cced544.md
    563d34649d677a94f608efd28858359a15c6f21448d774fcb5216d6fb8762573  m2p4a-open-2026-09-28-cced544.txt
    0be6ddc50a5c7e3d191b7e2d2feed16ce89640861da991c17bc0f5078eccf90c  m2p4a-p2-2026-09-28-cced544.md
    3c7b83357aae07148615ba9ddb8097ceec5d27afdfc31c7904a8438cb8dd8936  m2p4a-p2-2026-09-28-cced544.txt
    f26deb9d13b84b2e9372d7954440578f8d225743e3e436d29d44c1541012998c  p1-scan.log
    d230f5f437d2cc2b681b25d5e2c4c550acda93a8cf26f3c27cdea6454747bbd8  report-draft-2.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    541003c1280e93fad39ec97767d539f1263b2799b9917156fd3c55f92bb9b4e1  specs-0018-rows-and-columns.md
    3c4c26c75d0ce204036423055455c2a5600c3f34a77680b22c9e4b61a6f6c971  specs-0019-names-links-properties-protection.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-gate.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-negative.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-run.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-pre-check.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-test.txt

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 ce9686733a068350c1b1bef9c9fde05106e423d8b87926e2b73e705f320b36e3,
11709 bytes; plain ASCII, nothing replaced).

---

**Independent check 2, M2 part 4a P2**

Context note: the harness put a CLAUDE.md, a memory index, a git-status snapshot and a user email into my prompt. I did not use any of it as evidence. Everything below comes from the files in <check>. All of them match ../check2-inputs.sha256. The spec's sha256 begins 3c4c26c7, and every stderr-*.txt file is 0 bytes. Line numbers are in the frame of the named file in <check>.

**Short answers**

Q1. Yes, with the same caveats check 1 raised. The four commits do what P2 and P1 rulings 3, 4 and 6 ask. I agree with check 1's F1 to F5. I add one departure outside "Hyperlinks" that goes to planning (G1): the held `<hyperlinks>` against "32-bit and streaming". I also add three remarks: a write-time mutation of the recorded edits (G2), the strict-form mismatch between retag and relationship type (G3), and the tool counting only External relationships (G4).

Q2. Yes. The logs hold every line P2 requires, and every check I traced tests what it says it tests. The carried rules and ruling 7 (G1, G2/F6, G6, G9(b)) are met, with one exception. Ruling 7 G7 is not met evenly: run.log reads ANTHROPIC_API_KEY and HOME with no labelled input line, and gate, test and the scan read HOME unlabelled, while negative.log labels both (G5). Ruling 8 asks nothing of P2's logs (it shapes P4's size log only). I agree with check 1's F6 to F11. I add G6 and G7 as remarks.

Q3. No, with one defect and two gaps. Every number and hash in report-draft-2.md appears in a primary. One quoted string does not: "no such part" (G8, defect). The report carries F1 to F11 and F15. It fixes F12 and F13 and addresses F14 in its text, but never names F12, F13 or F14 as fixed (G9). Its F5 summary drops check 1's doc-comment point (G10).

**Details**

**Q1: diff against P2 (which diff I read for each "nothing else")**

- **Doc commit (diff-doc-95e5fe3.patch, read in full):** only ROADMAP.md (the P1 row) and specs/0019 (the in-place corrections and the two rulings sections). log-gate.log checks both blobs against planning's hashes and checks the file list. Nothing else.
- **Code commit 1 (diff-4e62e76.patch, read in full):**
  - Ruling 4: R and C alone are refused (lines 36-37), and the test pins for "R" and "C" move from the accepted list to the refused list (lines 108-109 and 117-118).
  - Ruling 6: the read-refusal test with its control (lines 49-69).
  - Ruling 3: the comment (lines 126-128).
  - The c-to-cn fixture renames.
  - The catalogue line (lines 9-16), which ruling 6 put in P4. It is harmless and the commit message names it.
  - Nothing else. log-gate.log's per-commit area check confirms two paths.
- **Code commit 2 (diff-a791e2c.patch, read in full):** matches "Hyperlinks" and the P2 test list clause by clause:
  - an external link appended with its relationship, id one past the largest;
  - a location link and an escaped tooltip;
  - the container created before pageMargins and at the root's end, with the template retagged and its rels part created;
  - a set on a linked cell;
  - removals with and without r:id, of an absent link and of a range link;
  - the last link dropping the container;
  - a second `<hyperlinks>` refused;
  - the row-insertion refusal;
  - a link and a cell set on one cell.

  Nothing outside crates/emend (log-gate.log, the area check).
- **Code commit 3 (diff-cced544.patch, read in full):** `--link-sample` as "Harness runs" states it, plus two tool tests and three usage-test lines. Nothing else.
- **Combined diff (diff-phase.patch):** used for the file list only. Its nine files are the union of the four per-commit diffs, and no manifest or lock file is changed.

I agree with check 1:
- **F1 (gap, for planning's ruling):** diff-a791e2c.patch lines 296-313.
- **F2 (gap, for planning's ruling):** diff-a791e2c.patch lines 356-365.
- **F3 (gap, for planning's ruling):** diff-a791e2c.patch lines 686-705.
- **F4 (gap):** agreed, with one addition. `record_link_relationships` runs before `validate` (diff-a791e2c.patch lines 391-392) and calls `self.relationships(&part)?` first (line 357). So which refusal an added sheet's external link meets, and at which step, cannot be read from the diff. See also G8.
- **F5 (remark):** agreed.

My own findings:

- **G1 (gap, for planning's ruling).** specs-0019 "32-bit and streaming" (lines 442-447) says the worksheet pass streams "one token at a time" and that a removed `<hyperlink>` "is skipped as its tokens arrive". The pass instead holds the whole kept content of `<hyperlinks>` in memory until its end tag (diff-a791e2c.patch lines 627-634, 848 and 870), so that a container left empty can be dropped.
  - The commit message and report-draft-2.md lines 78-79 state this choice, but neither check 1 nor the report puts it to planning as a departure.
  - It is bounded by one container: the corpus's largest input container, from the run's own counts, has 60 links.
- **G2 (remark).** `record_link_relationships` adds a relationship to the recorded edits and pushes to `link_ids` at write time, before validation (diff-a791e2c.patch lines 369 and 391).
  - If a caller can call `write_to` again on the same Package after a refused or completed write, a second relationship would be recorded for the same cell. `links_of` takes the first `link_ids` match (lines 197-201), so the second relationship would be left without an element.
  - The diff does not show whether `Package::write_to` consumes the package, so this is conditional. No test covers it.
- **G3 (remark, next to F2 and F3).** The retag chooses the relationships namespace form from the root's own namespace (diff-a791e2c.patch line 699). The relationship's type form is chosen from the rels part's existing hyperlink relationships (lines 356-365).
  - A strict-namespace root with no hyperlink relationships would therefore get a strict `xmlns:r` and a transitional HYPERLINK type.
  - Separately, `set_elements` chooses between r:id and location by whether a `rel_id` is present, not by `set.external` (lines 749-757). An external set without an id would be written as a `location` attribute holding the URL. That looks unreachable today, but nothing pins it.
- **G4 (remark).** "Harness runs" defines `external_rels_*` as "relationships of type HYPERLINK in W's rels". The tool counts only those with `TargetMode="External"` (diff-cced544.patch line 264). This is documented in the tool header (lines 20-21), and all 198 corpus hyperlink relationships are External, so no figure changes.

**Q2: logs**

Carried rules I checked and found met:
- Every log pins 3c4c26c7 outside its printed script: log-scan-precheck.log lines 1733-1741 give 8, 3, 1 and 3 lines.
- The run log's spec pin comes from a labelled SPEC_SHA input (log-run.log line 4).
- The scan declares `<phase>` (log-scan-precheck.log line 54).
- The gate's status check names the open-check stem (log-gate.log, "status after the run").
- The three shape greps, the placeholder check (tokens, not lines), and the "read from its committed log" figure (log-test.log line 2718).
- SERIALIZED lines, and the run times do not overlap.
- Each script's own stderr is 0 bytes.
- The negative log uses the gate's binary, checked by hash; every refusal has a control on the same form.
- The run log has 12 of 12 plants, the entry expectations, the keep, and the open check.

I agree with check 1:
- **F6 (gap):** the root_tag plant was applied on a workbook with `r_bound_in=1` (log-run.log line 2809).
- **F7 (remark):** the containers-created figure (20) equals the count of `links_in=0` rows per target, which I counted, but the log only checks the bound.
- **F8 (remark).**
- **F9 (remark):** 535 = 523 + 11 links tests + 1 names test. The rowcol-gated test is outside the gate's figure.
- **F10 (remark):** log-test.log lines 2684 and 2687.
- **F11 (remark):** log-run.log lines 1968 and 3790, and script lines 405 and 1756.

New findings:

- **G5 (gap, ruling 7 G7).**
  - log-run.log reads ANTHROPIC_API_KEY (script line 388) and HOME (script lines 155, 166 and 167: the defaults for the corpus, build and keep directories). Its input lines, 4-13, name neither.
  - log-gate.log, log-test.log and log-scan-precheck.log read HOME (log-gate.log line 180, log-test.log line 185, log-scan-precheck.log lines 274 and 387) with no labelled input line.
  - log-negative.log does label both (lines 5-6).
  - Check 1's statement that environment inputs are all labelled holds only for the variables the scripts treat as inputs.
- **G6 (remark).** Negative case L3 (`<q:hyperlinks/>`, log-negative.log) meets P2's letter: refused, part 2's message, with control L4. But the tool reads W's cells before any link edit (diff-cced544.patch lines 125-127). The log cannot show whether the refusal comes from the link pass or from that earlier read. The L4 control does not separate the two either.
- **G7 (remark).** Ruling 4's rule is "ignoring case", but only "R" and "C" are pinned (diff-4e62e76.patch lines 108-109), and plant L6 drops only the R clause (log-test.log). Lower-case "r" and "c" are refused only implicitly, through the fixture renames.

**Q3: report-draft-2.md against primaries**

Found in primaries:
- 3c4c26c7, fe00ad86, the five commit ids and the four log hashes (log-gate.log and log-scan-precheck.log).
- 9 PASS, 512 and 535, and 735/0/2 on each target.
- The seven plants, and both quoted messages (log-negative.log).
- 822/48/0/6, 12 of 12, 176 + 2 x 27 = 230, 21, 20 and 0.
- 381 pairs, 14 new, none gone; 0 outputs refused and 0 checks that did not run, crate and tool (log-run.log).
- The four record hashes (log-run.log lines 2991-2993 and 3772-3776).
- "RESULT: PASS", 0 hits, 0 structural failures (log-scan-precheck.log).
- "differ.set -uo pipefail" at p1-scan.log line 217. That is the only `set -uo pipefail` in P1's printed script, so the claim holds.
- "(rulings 3, 4 and 6)" (commit-messages.txt).
- The design-choice strings at report lines 60-71 (diff-a791e2c.patch).
- "each attempt 1", with its reason (RERUN_NOTE and RERUN_FILES unset in every log).

Findings:

- **G8 (defect).** report-draft-2.md line 73 quotes "no such part". That string is in no primary: no diff, log, spec or record holds it, and the spec says only "the missing-part refusal" (specs-0019 line 435). The rest of the sentence at lines 73-76 is also not in the diff: the added-sheet branch at diff-a791e2c.patch line 402 is context only. Either drop the quotation or pin it with the F4 tests, and say "read from the source" only for what the diff shows.
- **G9 (gap).** Check 1's F12, F13 and F14 are handled in the text:
  - F12, by lines 24-25;
  - F13, by p1-scan.log now being an input;
  - F14, by "read from the source, not measured" at line 72.

  But the "From independent check 1" section (lines 92-110) names only F1 to F11, plus F15 at line 68. It should say that F12 to F14 were fixed in draft 2, and that F14 still rests on G8's unsourced quotation.
- **G10 (gap).** The report's F5 summary (lines 98-101) leaves out check 1's first F5 point: the doc comment on the metadata-on `no_links` says "always, without the feature `metadata`" (diff-a791e2c.patch lines 126-127). That comment is still wrong in the tree and is carried nowhere.
- **G11 (remark).** Line 92 says "the answer is in the record verbatim". None of the four records holds it yet; it will be in the record commit, as independent-check-1.md. The report should name the file.
- **G12 (remark).** Lines 47-48 give 381 pairs, 14 new and none gone, but leave out the one P1 pair whose bytes changed: `a_name_added_to_a_container_is_appended_and_the_file_children_kept.1`, from the c-to-cn rename (log-run.log, "pairs of P1's record whose bytes differ in the keep: 1").