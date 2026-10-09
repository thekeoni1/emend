# Independent check 1, M3 part 7 P2

Written by the implementing session. Checked commit (the phase commit): 8d547419831a1f262cfdd8493d06a706a0350f77; HEAD when the check ran: 8d547419831a1f262cfdd8493d06a706a0350f77. The checker was a fresh general-purpose subagent given only the directory <phase>/check1/inputs (<phase> stands for <home>/.cache/emend-build/m3p7-p2), read-only; its inputs, by sha256 (sha256sum over each file, paths relative to that directory):

    e4c48dcb32cf54a3457fc3be422a26f357c0fa798915aa69c964af7c244aa6d1  ./commit-1d3c22b.diff
    707c798510c6e60b225d57825c9b52532cb750b05184dcd881d7898bb6428e13  ./commit-8d54741.diff
    c50938d299e8477954a85a929b5c9ab2d0910eccadd9df860186506954276333  ./gate.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./gate.stderr
    d949f507d9401f673a2986b3091171621eb3136dba00bdd7299b34ae81f72ad3  ./logged.out
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./logged.stderr
    6f05236a220ceb5760baedd5026078749b22a225df7a779c49bf29e9da4af967  ./messages.txt
    c1b5f704baeb09581738a6f5a704b9626407716aa02b0e2fd178484a7738dbf4  ./negative.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./negative.stderr
    12c4952ff8ade41548af621cdac2eb05b42717e99983aa280b036be39bb8db62  ./phase.diff
    2a9a5c1eb234ee834df62ed48fe45bff110ee61e36d3ebcd67f8619e088cdef5  ./report-draft.md
    9d3b6b7148adfb227969e691800538434aea3aba0ae6b18806d23f23a8304768  ./scan-pre-check.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scan-pre-check.stderr
    433958e8da6245194a91f2663c1210f1425ed2957966a55bd49b96150ececc3c  ./spec-0028-at-phase.md
    639b630c0e9b92acb2817fbd40c74247a979d8997c9f0957a4865bdceaa32633  ./spec-0029.md
    c8cb5ac995bbdda91ba7dac1688d05214cd176d4ffc05c539a2f30b52e8ff371  ./specs-0001-at-phase.md
    1d90913642188ebbc382b45c9f2486e538f72398d58686acea10970e6c415699  ./superseded/scan.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./superseded/scan.stderr
    af5915b7ede9f971010f128ec0c182ceb4f99d38c0400efb0f9be522ebe310c6  ./test.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./test.stderr

The answer below is the checker's final message, extracted from its transcript, verbatim (sha256 79638f3a148173057bb1fc2a0fed120cfc23eb3645b9f77dec81a98cdf4cc6f3); it holds no non-ASCII byte and no absolute machine path, so nothing in it is replaced.

---

The harness put a CLAUDE.md, a memory index, git status and a commit-attribution reminder into my prompt. I did not use any of it as evidence.

**Q1. Does the diff do what the spec says for P2 and nothing else? Yes.**

- **Scope.** spec-0029.md lines 290-299 and items 6 to 8 (lines 112-138) define P2. Decision 2 is A (lines 478-481), so item 8 needs no code. Nothing under it appears in any diff.
- **Item 6 (commit-8d54741.diff).** package.rs gains exactly two lines, both `check_copy_relationships(start, &scope, None, index)?;`. One sits in copy_worksheet's sheetPr branch and one in its sheetView branch, each before `without_attribute` strips codeName or tabSelected. No line is removed.
  - copy.rs gains one test with 18 added lines and none removed. Its first body is a sheetView carrying r:id beside tabSelected, which is the spec's case. Its second is a sheetPr carrying r:id beside codeName. Both are asserted refused with the existing message.
  - Item 6 routes both tags, so the sheetPr case is in scope. Only the "one test" wording is exceeded, and the report says so.
  - gate.log lines 3225-3265 confirm this independently: two hunks, both headed `fn copy_worksheet`. The line before each added line is the sheetPr branch, then the sheetView branch. No tools/, scripts/, Cargo.toml or Cargo.lock path changes. The crate's Error::Package messages are 139 at both the base and the phase commit.
  - negative.log lines 1886-1896 show 123 message constants at both commits, 0 new and 0 gone.
- **Item 7 (commit-1d3c22b.diff).** Both amendments are dated 2026-10-08 and signed by `specs/0029` item 7. They are added under the original lines, which stay in place.
  - The first sits in "The copy", at spec-0028-at-phase.md lines 302-306, after the dropped list at line 279.
  - The second sits in "Size", at lines 406-412. Its content matches P4 ruling 3 (spec-0028-at-phase.md lines 1201-1205): images_removed, AB10, 200 by 100, visible, box_width, box_height, box_visible.
  - The diff removes no line; gate.log line 3141 says "lines the doc commit removes: 0".
- **Doc commit contents.** It also adds the "Planning rulings during P1" and "Planning verification and rulings on P1" sections to specs/0029. Both files hash to the blobs planning's brief gave:
  - specs/0029: 639b630c...; my sha256sum of spec-0029.md agrees.
  - specs/0028: 433958e8...; my sha256sum of spec-0028-at-phase.md agrees.
  - gate.log lines 3110-3139 check both.
- **Which diffs I read for "nothing else".**
  - I read commit-8d54741.diff and commit-1d3c22b.diff in full.
  - I read phase.diff in full. Its added and removed lines, sorted, equal the two commit diffs' combined; its only `-` lines are the four `---` file headers.
  - Files in the phase: 4 (gate.log line 3167; scan-pre-check.log "files in the diff: 4").
- **Observation, not a defect.** The new check runs on the start tag before stripping. A malformed sheetPr or sheetView (for example, one with an unbound attribute prefix) could now be refused where it was copied before. Valid input is unaffected; the `xml` prefix was fixed in part 6 P4 (spec-0028-at-phase.md lines 1207-1235). No P2 log runs the copy over real corpus sheetViews; P3's sample reruns are where that shows.

**Q2. Do the logs contain every line the spec and specs/0001 require for P2? Yes with reservations.**

- **Environment line first.** gate.log lines 1-9 are the header and input lines. Line 11 opens the environment section: propagation shared, keepalive 1, with controls for private, empty, 0, 2 and error (lines 11-25).
- **Prune.** gate.log lines 39-57 name the four pruned directories (dry-gate, dry-test, gate, target). Each is listed with du -sb (4207142263 bytes in all), removed with `rm -r` and no -f (exit 0 each), and checked gone. 19 kept names are checked present, with controls.
- **Gate files.** gate.log lines 3179-3223 list ten gate files, 0 of them in the diff, with no self-test log, plus controls.
- **Test on a byte-built part whose sheetView carries r:id.**
  - The test passes alone (test.log lines 5553-5560).
  - It passes with copy alone, copy and comments, and every feature, on both targets, and is absent without copy (lines 5503-5508 and 5539-5541).
  - Plants L1 and L2 each fail at copy.rs:345 with exit 101, with different panic messages (4128 and 4067 bytes written), lines 5920-5988.
- **Full copy test's bytes.** test.log lines 5561-5577: the full test passes. FULL_BODY (d2899387...) and FULL_COPIED (108b44f5...) hash the same at the base and the phase commit.
- **Negative log.** It uses the gate's host binary, hash 8a96474f... checked against gate.log line 3684 (negative.log lines 1823-1830).
  - RV is the sheetView r:id fixture: exit 2, the exact message, no stdout, no output file.
  - RC is the control: exit 0, its result line, an output file.
  - RP (sheetPr r:id) is refused the same way (negative.log lines 1959-2001).
- **Each script's stderr.** All four logs print 0 bytes with a control: gate.log lines 3891-3894, test.log lines 6000-6003, negative.log lines 2040-2043, scan-pre-check.log lines 2826-2829. Every *.stderr file is 0 bytes.
- **Reservations:**
  1. dry-negative and dry-scan stay under P1's directory. The justification exists only in the gate script's comment (gate.log lines 1376-1378 and 1585-1586): "P1's report cites files in them by hash". specs-0001-at-phase.md line 273 exempts what "a committed log cites by hash". No input shows P1's report is committed or names those files. No log prints the kept directories' sizes, so any build output left in them is not visible.
  2. The prune and keep sets are typed (gate.log lines 1587-1588). No command checks that no committed log cites a path under a pruned name. Part 6's prune followed the same practice.
  3. No log prints the detached runner's stderr. logged.stderr is 0 bytes and logged.out shows the three exits. P1 ruling 4 accepted this.
  4. The final scan log is not among my inputs; only the pre-check is.
- **Forbidden shapes.** My grep for the three forbidden shapes finds 0 in each of the four logs. Every recursive removal in the printed scripts is in the guarded form (gate.log lines 1468, 1575, 1615, 3017; test.log lines 1265, 1339, 2855; negative.log line 1788).

**Q3. Does every number, hash and quoted string in report-draft.md appear in a primary? Yes with reservations.**

Everything below was found:
- the commit shas and the base subject (gate.log);
- 433958e8 and 639b630c (messages.txt, gate.log);
- the code line and the message (commit-8d54741.diff);
- c50938d2 (negative.log line 1823, scan-pre-check.log);
- af5915b7 and c1b5f704 (scan-pre-check.log);
- 4207142263, the four pruned names, 19 kept, 10 checked (gate.log);
- 514, 558, 526, 573, 918 (gate.log lines 3773-3829);
- L1, L2, 101, `<target>/plant` (test.log);
- RV, RP, RC, exit 2, exit 0, COPY_RELATIONSHIP (negative.log);
- "RESULT: PASS" and "structural checks failed 0" (scan-pre-check.log);
- the three "checks all as expected: yes" lines (the logs' last lines, logged.out);
- PLANNING_SPEC unset (scan-pre-check.log line 8), and PLANNING_SPEC set to 639b630c in superseded/scan.log line 8.

Items I could not find in a primary, or that differ from one:
1. **9d3b6b71... (the pre-check's hash).** No log prints it. My sha256sum of scan-pre-check.log matches it, but that is my measurement, not a primary.
2. **`<phase>/superseded/dry-scan-1`.** This path appears only in report-draft.md. The superseded scan's own invocation reads LOGDIR=`<phase>/dry-scan/logs` (superseded/scan.log line 3). The report gives no hash for this superseded attempt; its sha256 is 1d909136... by my count.
3. **"failed two structural checks".** superseded/scan.log line 2833 reads "structural checks failed: 3": two AV checks (lines 61 and 2737) plus the LOGDIR dry check (line 67). The report's later sentence makes it reconcilable, but the count of 3 is never stated.
4. **"Rerun with PLANNING_SPEC unset ...: only the dry-only LOGDIR check failed".** No input holds that second dry attempt.
5. **"P1's report cites files in them by hash".** No primary supports it beyond the script's own comment; see Q2 reservation 1.
