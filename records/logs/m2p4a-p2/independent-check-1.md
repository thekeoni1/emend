# Independent check 1, M2 part 4a P2

Written by the implementing session: the checked phase commit is
cced544f57b8027193de004ec785d72c614ad208, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk, holding these files, by sha256
(the logs there are the four committed logs, each attempt 1, and the scan
pre-check, which the final scan.log's rerun lines do not need to hash
since it is the only pre-check):

    f6c888eb30707436478a2042c02d91b01b1305620bc2827cf2f7f334f4cb1629  commit-messages.txt
    ecb698a11a1836798f74730d42257c669032776e98895de7df77344388e6a5d3  diff-4e62e76.patch
    54a587f1a3bf860b801c3aac2d820524ea32ff542af4fcfbfebbb9889441dddb  diff-a791e2c.patch
    4061bb2570cc9e64fa3a4391331e8e3dd84551df3c407372f307a9e44daaccae  diff-cced544.patch
    e29fffc71a86d74d42c53fe96abc940af1410757a63d74698422b778aead2754  diff-commands.txt
    6e1c02863da7c288d45f1d44216ac835107bc5ee1107d0cd4dc6005f46af558d  diff-doc-95e5fe3.patch
    5b7ba024d57b6fab2fc0d3c6230e1c6328fe8f9ef527b050908ab240dd7077ab  diff-phase.patch
    3ed9c21efb3bd9eb66d4a2c65288c23c6b1457012e55d3788ca5003ed443e5fb  log-gate.log
    0cc1ec7bc96c1e403657d05c06ac04ef66d102ebeacd33bb8ebe1852fb13cd0a  log-negative.log
    f4dfa2797970a115a9420adb634b49f6f09a7d9383c37027cc5140cdd101ff75  log-run.log
    bea1e63fda74d314f078984e223f5b6e0c42ae849b28edf44f32394485c4549d  log-scan-precheck.log
    2ee6c3382cd0b10da10e3b27bacd0031677218a467e819a4566f7c688cba59fd  log-test.log
    5320e5be6ccd74d0ae31016256ad8a82d8e0a7a925cd30e13e4700ace27b1280  m2p4a-open-2026-09-28-cced544.md
    563d34649d677a94f608efd28858359a15c6f21448d774fcb5216d6fb8762573  m2p4a-open-2026-09-28-cced544.txt
    0be6ddc50a5c7e3d191b7e2d2feed16ce89640861da991c17bc0f5078eccf90c  m2p4a-p2-2026-09-28-cced544.md
    3c7b83357aae07148615ba9ddb8097ceec5d27afdfc31c7904a8438cb8dd8936  m2p4a-p2-2026-09-28-cced544.txt
    5ea35b1341204872c22bc86fb5bf9c95f716f56faf64dcdc5482e0e5cfea68e7  report-draft.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    541003c1280e93fad39ec97767d539f1263b2799b9917156fd3c55f92bb9b4e1  specs-0018-rows-and-columns.md
    3c4c26c75d0ce204036423055455c2a5600c3f34a77680b22c9e4b61a6f6c971  specs-0019-names-links-properties-protection.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-gate.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-negative.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-run.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-pre-check.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-test.txt

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 d8d8343d565874caae9ae1dd7bf4392805c7740d42454c78c8638b4b05412c04,
12192 bytes; plain ASCII, nothing replaced).

---

Independent check, M2 part 4a P2 (check 1)

Context note: the harness put a CLAUDE.md, a memory index and a git-status snapshot into my prompt. I did not use any of it as evidence. Everything below comes from the files in <check>. I checked them against ../check1-inputs.sha256 and all of them match. Every stderr-*.txt file is 0 bytes.

Short answers

Q1. Mostly yes. The four commits do what "Hyperlinks", "P2" and P1 verification rulings 3, 4 and 6 ask. I read diff-doc-95e5fe3.patch, diff-4e62e76.patch, diff-a791e2c.patch and diff-cced544.patch, each in full, and used diff-phase.patch for the file list. The hyperlink write in diff-a791e2c.patch does all of these as the spec states:
- skips a link whose ref is exactly an edited cell, and appends the sets in record order;
- removes the relationship a dropped r:id names;
- retags a root that binds no relationships prefix;
- creates an absent container in schema position, drops one left empty, and refuses a second;
- refuses a link beside a row or column shift with part 3's message;
- refuses an empty target at the call, and an absent link and a second container at the write.

The "nothing else" is not clean. The crate commit adds behaviour the spec does not state: refusals at the call that the spec does not list, a strict relationship type form, and an r1/r2 prefix fallback on the retag. The report discloses these, but planning has not ruled on them. It also leaves one decision 3 case open: a link on a sheet added in the same write. See F1 to F5.

Q2. The logs hold every line P2 requires, and the rules carried from specs/0001, specs/0018 "Phases" and close-out ruling 4, and P1 rulings 7 and 8 are met. Every log pins 3c4c26c7..., the scan declares <phase>, the run log takes the spec hash on a labelled input line, the scan counts pins outside the printed scripts, and the gate's status check names the open-check stem. Environment inputs are on labelled lines after the invocation line. Every control uses the same command form. There are some weak spots:
- two range checks that cannot realistically fail;
- one self-consistency check presented as a cross-check;
- the comparison's retag branch has no plant, and the corpus never exercises it;
- the gate's feature-on crate count leaves out the rowcol-gated link test, which runs only in the workspace run.

See F6 to F11.

Q3. Every number, hash and quoted message in report-draft.md is in a primary, with four exceptions: "each attempt 1" is inferred, not printed; the P1 scan.log statement rests only on a script comment; and two design-choice claims have no test or log line. See F12 to F15.

Details

Q1: diff against spec (which diff I read for each)

- **Doc commit (diff-doc-95e5fe3.patch):** only ROADMAP.md (the P1 row marked DONE) and specs/0019 (the rulings added and the in-place corrections the rulings name). log-gate.log lines 1478-1506 check both blobs against planning's hashes: 3c4c26c7... and fe00ad86... Nothing else is in it. Confirmed.
- **Code commit 1 (diff-4e62e76.patch):**
  - Ruling 4: R and C alone are refused, ignoring case (package.rs hunk at +1435), and the test pins for "R" and "C" move from the accepted list to the refused list.
  - Ruling 6: the read-refusal test with its control is added (names.rs +105..125).
  - Ruling 3: the names.rs comment is corrected.
  - "c" becomes "cn" in two fixtures. This is needed because the rule now refuses "c".
  - One extra: a catalogue line in the package.rs header (+61..66). Ruling 6 assigned that to P4. It is harmless and the commit message says so.
  - Nothing else. Confirmed.
- **Code commit 2 (diff-a791e2c.patch):**
  - Skip and append: LinksPass::child (+808..858) skips a main-namespace `<hyperlink>` whose ref parses to an edited cell and records its r:id. A range ref does not parse, so it is never matched. set_elements appends in record order through links_of, which keeps each cell's last record (+179..221).
  - Relationship removal: drop_link_relationships (+413..426) runs after the part is replaced (+398..400). It removes the id when the rels part has it and no removal is already pending. An r:id the rels part lacks is ignored silently. The spec says nothing about that case.
  - Root retag: root_tag (+663..706) uses with_attribute with the namespace form of the root's own namespace. It runs only when an external set exists and the root binds no relationships prefix.
  - Container: create_before and create_at_end (+710..730) in schema position, including the empty-root path. close (+878..894) drops a container left with no hyperlink and no set. open (+776..786) refuses a second container.
  - One-shift: no_links is added to SHIFT_ALONE's "alone" test (+437).
  - Call versus write: at the call, an empty target (spec) plus the extras in F1. At the write, NO_SUCH_LINK (finish, +896), TWO_SHEET_ELEMENTS and the one-shift message.
  - The cell is not touched. No `display` is written.
- **Code commit 3 (diff-cced544.patch):**
  - --link-sample follows "Harness runs": W, S, M+1, the three edits, the line fields, and the three asserted counts, with the output removed on failure.
  - It adds two tool tests and three usage-test lines.
  - Nothing else. Confirmed.

F1 (gap, for planning's ruling). diff-a791e2c.patch lines 296-313 and 325-327: set_hyperlink refuses a target or tooltip holding a character XML forbids ("a text holds a character XML cannot carry") and a cell off the grid at the call. remove_hyperlink also refuses off-grid at the call. "Hyperlinks" says "nothing else about the text is checked", and "Errors" does not list either message as reused for links. "Harness runs" does anticipate part 3's index message for M+1, and P1 ruling 10 accepted the same forbidden-character choice for names. No ruling yet covers links. report-draft.md lines 59-62 disclose it.

F2 (gap, for planning's ruling). diff-a791e2c.patch lines 356-365: the relationship type takes the strict form when the part's rels part already has a strict hyperlink relationship. The spec says "of type HYPERLINK" and gives no rule for the form. No test covers the strict branch. Disclosed at report-draft.md lines 65-66.

F3 (gap, for planning's ruling). diff-a791e2c.patch lines 686-705: when the root declares `r` for another namespace, the retag binds `r1`, `r2` and so on. Decision 3 A and "Hyperlinks" say the retag binds `xmlns:r` and the attribute is written `r:id`. This is a reasonable extension, and links.rs a_root_whose_r_prefix_is_taken_is_retagged_with_the_next_free_one pins it, but it departs from the wording. Disclosed at report-draft.md lines 67-69.

F4 (gap). Decision 3 names "every sheet add_sheet creates" as a root the retag serves. report-draft.md lines 70-71 say a link on a sheet added in the same write is refused by the existing relationship-source rule. That can only apply to an External link. The diff does not show what a Location link on a sheet added in the same write does: the added-sheet branch at diff-a791e2c.patch line 402 is context only. No test in links.rs or null_rewrite.rs covers either case. Planning should rule whether same-write refusal meets decision 3 A, and the behaviour should be pinned by a test.

F5 (remark). diff-a791e2c.patch lines 126-127: the doc comment on the metadata-on no_links says "always, without the feature `metadata`", which describes the other variant. Lines 713 and 880 hold two edge cases the spec does not cover:
- `schema_position(HYPERLINKS).unwrap_or(0)` falls back silently to position 0.
- A container whose only children are not `<hyperlink>` (for example mc: markup) counts as empty and is dropped, children and all.

Also, drop_link_relationships removes a relationship even when a kept `<hyperlink>` names the same r:id. That would leave a dangling r:id, though schema-valid files should not share an id this way.

Q2: logs

F6 (gap). log-run.log line 2809: the root_tag plant is applied to a workbook with r_bound_in=1, and every corpus workbook has r_bound_in=1 ("roots retagged 0", line 2686). So the comparison's retag branch (the added xmlns:r declaration and the r1 choice) has never been shown to report a difference. Only the crate test and test.log plant L4 exercise the retag.

F7 (remark: checks that could not realistically fail). log-run.log lines 2694-2695 and 2800-2801 check that containers created (20) and roots retagged (0) "lie between none and every workbook". Both are bound checks only. The 20 could have been checked against the workbooks with links_in=0, which is also 20 in the per-workbook lines.

F8 (remark). log-run.log lines 2693 and 2799, "(21 = 21)": both sides come from linkscmp.py, one from its --parts mode and one from its rels_in field. This is a self-consistency check of one script, not an independent count.

F9 (remark). log-gate.log line 2050 and log-test.log lines 2826-2830 report 535 tests with --features metadata. That count excludes links::a_link_on_a_part_with_a_row_insertion_is_refused, which is cfg(rowcol). The test runs and passes only in the workspace run (log-test.log lines 1457 and 2240, and the added-test list at 2725). This meets the spec, but the P2 one-shift test is not in the gate's feature-on figure.

F10 (remark). log-test.log lines 2684-2689 print "links module tests passed 12", but the check only asks that the run holds the links tests. No check compares 12 with the 12 tests the source adds. The count is printed and never read. The source-derived per-test check at 2742-2815 does cover each test by name.

F11 (remark). log-run.log lines 1968 and 3790-3792 label the spec "planning's uncommitted spec", but it is committed at 95e5fe3 and the status lines show it unmodified. The label is stale. The check itself holds.

Other rules I checked and found met:
- Every log pins the spec: gate lines 20-27, test lines 20-27, negative line 57, run lines 4 and 1974, scan precheck line 8. The scan counts pins outside the printed scripts (lines 1733-1741).
- The negative log's two refusal cases each have a control on the same binary form (L0, L2, L4). The binary is the gate's, checked by hash.
- The run log checks every P2 comparison element, each with a plant (12 of 12). It compares entries, installs the records and prints their hashes, and runs the keep and the open check.
- The scan's control-byte count, the three shape greps and the placeholder declarations are all present.
- Commit messages are ASCII with the one trailer (gate lines 1445-1476; commit-messages.txt).

Q3: report numbers and quotes

Found in primaries: 3c4c26c7 (all logs); the four commit ids and 4c3485a (gate); fe00ad86 (gate line 1491); 3ed9c21e, 2ee6c338, 0cc1ec7b, f4dfa279 (scan precheck lines 1702-1736 and the check list); 9 PASS; 512 and 535 on both targets (gate); 735 passed, 0 failed, 2 ignored per target, and seven plants (test log); both quoted messages (negative log and diff); 822, 48, 0, 6; 12 of 12; 176 + 2 x 27 = 230; 21, 20 and 0; both targets the same; 381 pairs, 14 new, none gone; outputs refused 0 and checks that did not run 0, crate and tool (run log); record hashes 0be6ddc5, 3c7b8335, 5320e5be, 563d3464 (run log lines 2957-2960 and 3772-3776); scan "RESULT: PASS" with 0 hits and 0 structural failures; "(rulings 3, 4 and 6)" (commit-messages.txt); the three log end lines quoted.

F12 (gap). report-draft.md line 24, "each attempt 1": no log prints an attempt number. It follows from RERUN_NOTE and RERUN_FILES being unset in every header, and the report should say that.

F13 (gap). report-draft.md lines 77-78, the statement about P1's committed scan.log: the only primary here is the comment for change AR in P2's scan script (log-scan-precheck.log lines 261-263). P1's scan.log is not among my inputs, and no log line shows the fused line.

F14 (gap). report-draft.md lines 70-71, that a link on a sheet added in the same write is refused: no test and no log line shows it (see F4).

F15 (remark). report-draft.md lines 63-66: "a replaced set leaves no relationship behind" and "its type takes the strict form" follow from the code (diff-a791e2c.patch lines 179-221 and 356-365). No test covers two External sets on one cell, or the strict form. The existing last-record test uses Location targets.