# M3 part 6 P2, independent check 2

The answer of a fresh subagent (not a fork) given only <phase>/check2/inputs (the committed spec specs/0028, planning's working copy with rulings 1 to 12, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log attempt 3 (the committed run.log), run.log attempts 1 and 2 (superseded), the scan's pre-check over attempt 3, the stderr files, logged.out, the records attempt 3 wrote, independent check 1's answer, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p6-p2. Checked commit (the phase commit): d3c4a9c8dc7dbc1b3c5ce1f11894fe5bd0cc5144; HEAD when the check ran: d3c4a9c8dc7dbc1b3c5ce1f11894fe5bd0cc5144. This is the second and last independent check of P2 (planning's ruling 9). The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 871f715e8285e479a0fc9bbc43451e258965de277b2f80c13be0e12406b18eda. Nothing in the copy below is changed. Its reservations are reported to planning.

Inputs and their sha256:

- 91348bb00a4bcc1dc2abc5fe4e0759bdae6bd61d9362676f740838c6ca31da97 code-commit-1-crate.diff
- 860c1f9b1ac87925fbc593d937ce4f066c61509a977b64e0cc371efe046b5a60 code-commit-2-tool.diff
- db0bfd0109fa59142de6a7ea8906363196704e6018460ac5d6a6d57e4c22d003 commit-messages.txt
- 666641af0b48e7078c4294512aa705c03dd119be6e0438f20140020ced2b4b47 doc-commit.diff
- ef4dbee46785ed838ba0eb7a8d12c8ae30939533a349731f7bb3551bc2e659ec gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- 3718c8281eb71f3f0268955df393914d6d2e547ca5fc555447326e13d84f32fb independent-check-1-answer.md
- 565c314981ba8cbea65ce81ce92589b82d545b6aa212160a1517036fcd15baf3 logged.out
- 7be4d841111cb559762bd1d197107483e5998964fe4f9812185887936cd31722 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- c322a37ba800636266b01f7f99ced5ee9f5b3a18c432451d20a73b2f97cfce93 phase.diff
- d06368fcf64644820ed3a1133abe5abb92cb3474ee47ca5fb5038af99f68ac1e records/m3p6-open-2026-10-07-d3c4a9c.md
- 30d916cabffc4a4f484b182fbc084669159f84e9d7f5ab1b959551d10e3d77d7 records/m3p6-open-2026-10-07-d3c4a9c.txt
- 47d36d40d5114c72da2bc6a0991598a2a858e8e6c1f52c0a0edf1450d959328f records/m3p6-p2-anchor-2026-10-07-d3c4a9c.md
- d689ccd2fdbaf08a05472cd9f4ee6cc4a9964f303b5ec932d034a96ccc1d7a07 records/m3p6-p2-anchor-2026-10-07-d3c4a9c.txt
- 90e7c359ac1f9234266a1fadb389cce0752dad69d1856501b94d3c7045c61cfb records/m3p6-p2-removeimage-2026-10-07-d3c4a9c.md
- a29eda4ce08013cceb88f2dcfd4b966551809452d4528e92bca3a4880d79ca42 records/m3p6-p2-removeimage-2026-10-07-d3c4a9c.txt
- 12fbecaa557c5abf1ac91d8967e655096a2e30fa6306e926d695abe9adabac74 report-draft.md
- 3df393a5c6a0bcc59759387bedfc29977b0a5f4f7da4755970bb81eacc255e0f run-attempt-1-superseded.log
- c22852ffb8c0a70a29affac2e200b7fdb3f9da698c10fc64d80ac86ec544ae8a run-attempt-2-superseded.log
- ab860da2df98723eb488aef2468585d0d780d13d552ca49b35c7c915ec611295 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- a8e47d736b92881598b9f88c70cfb966f2d969e6ea6d59c16081922f4407dc40 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 5c3d3d1aa4d5bad2dac42cf7c0836a1d80d93a2dfc7e76cf94a48aacd444e6b0 spec-0028-committed.md
- 1fca1264cfddf49a9db9cac44d473945b5abcf0670329ad81315da1d4077b772 spec-0028-working.md
- 78c174817d8ee2456bfa1274f4c202fe9a64551afd7a1b2c608cc3188999870b test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

## The answer

Independent check 2, M3 part 6 P2 (inputs: <phase>/check2/inputs)

Scope: I read only the files in <phase>/check2/inputs and ran only read-only commands (cat, grep, sed -n, awk, diff, sha256sum, wc, python3 -I). I modified, created and removed nothing. The project CLAUDE.md was also placed in my context by the harness; I did not use it as evidence.

## Q1. Does the diff do what the spec (with planning's P2 rulings) says for P2 and nothing else?

YES WITH RESERVATIONS

**phase.diff against the three commits.** I split phase.diff by file and compared each file section with the per-commit diffs. All 9 files are byte-equal to the one commit diff that holds them (doc: ROADMAP.md and specs/0028; crate: lib.rs, package.rs, tests/images.rs, images/anchors.rs, images/removal.rs; tool: main.rs, tests/null_rewrite.rs). So phase.diff is exactly the three commits.

**Doc commit (doc-commit.diff).**
- ROADMAP.md: one row changes, P1 from PENDING to DONE (lines 9-10).
- specs/0028: adds "Planning verification and rulings on P1" (lines 22 onward). Nothing else changes.
- The resulting spec hashes to 5c3d3d1a, which is the sha256 of spec-0028-committed.md and the SPEC_SHA pinned in run.log line 4.

**Crate commit (code-commit-1-crate.diff).** This matches "Anchors", "Image removal", "Errors" and the P2 bullet.
- lib.rs (line 9) re-exports `EditAs` and `ImageAnchor` under `images`.
- Lines 69-126 define `ImageAnchor` (OneCell, TwoCell, Absolute) and `EditAs`. Both derive Debug, Clone, Copy, PartialEq and Eq, with the fields the spec gives.
- Lines 181-210: `add_image` is `add_image_at` with a one-cell anchor and offsets (0, 0), one definition. Lines 259-341 hold the call checks in ruling 7's order:
  1. the index message for an off-grid cell;
  2. IMAGE_ANCHOR_OUT_OF_RANGE for an offset above i32::MAX EMU or a `to` before `from`;
  3. IMAGE_SIZE_TWO_CELL for a size on a two-cell anchor;
  4. then `image_header` and the extent checks.
  No extent is checked or written for a two-cell anchor (ruling 7).
- Lines 650-835, `anchor_bytes`: `editAs` is always written; `from` and `to` each get col, colOff, row and rowOff; an absolute anchor gets `pos` and `ext`; EMU is 9525 per pixel. A zero offset prints "0", so `add_image`'s bytes are unchanged.
- Lines 342-354: `remove_image` only records the removal at the call. Lines 427-607, `record_image_removals`, then does the following at the write:
  - An index named twice, or on a part this write also adds to, gives NO_IMAGE_AT.
  - A removed part gives REMOVED_PART_EDITED; a replaced part gives CELL_EDIT_REPLACED; a shifted part gives SHIFT_ALONE under rowcol.
  - An index past the list that `images` reads gives NO_IMAGE_AT.
  - The root child holding the picture is skipped whole, by byte spans from `read_drawing_roots` (lines 855-945). An AlternateContent whose first Choice holds the anchor goes whole (ruling 4); a group goes with its anchor (ruling 5).
  - Each `embed` or `link` id that no kept root names is removed from the drawing's rels part.
  - K is computed once through `reachable(b"", &skipped, &[])`, with the removed relationships in `skipped`. A dropped drawing's sheet relationship is also added to `skipped` (ruling 6).
  - Each media entry outside K is removed with its Override.
  - A drawing left with no anchor is dropped through `drop_drawing` (lines 589-604): its part, rels part, Override and the sheet's relationship go. The `<drawing>` element is skipped in the worksheet pass (lines 631-646).
  - `anchor` is true for any root child that is or holds a oneCell, twoCell or absolute anchor, so an AlternateContent with no anchor does not keep the drawing. This matches the spec's "no root child other than AlternateContent holding none".
- Lines 617-629: the three message constants, worded exactly as "Errors" gives them.
- Tests (lines 1006-1631, anchors.rs and removal.rs): every case the P2 bullet lists is there, plus the AlternateContent case (ruling 4), a kept-reference case and the missing-media case.

**Tool commit (code-commit-2-tool.diff).**
- Lines 155-212, `anchor_sample`: the two-cell anchor runs from (M + 1, 1) to (M + 3, 3) with zero offsets, editAs OneCell and size None. The absolute anchor is at (96, 96) with size Some((4, 4)). It prints the eight fields the spec names and asserts +2 images, anchors and media, and +1 of each anchor kind.
- Lines 272-342, `remove_image_sample`: an addition at (M + 1, 1), a write into memory, a reopen, a removal of the last index, and a removal of index 0 when the input had images. It prints the nine fields. `images_after` is asserted against `images_before - kept_removed`. `media_dropped` is asserted against `dropped_media` (lines 382-480), which counts through every rels part of the archive. `drawing_dropped` is asserted when the input had no drawing.
- Lines 485-622: three tool tests.

**Commit messages (commit-messages.txt).** All three are ASCII and end with the Co-Authored-By line only. None has a Claude-Session line. phase.diff, commit-messages.txt and report-draft.md each hold 0 non-ASCII bytes.

Reservations:
- (a) Ruling 12 already covers this: the drawing-drop block (crate diff lines 631-646) sits between the `legacyDrawing` comment and its code. It carries to P3.
- (b) This is an observation, not a departure. Only `r:embed` and `r:link` are collected, as "Image removal" says. A relationship that a removed anchor names through another `r:` attribute, such as `hlinkClick`'s `r:id`, therefore stays in the drawing's rels part with nothing referencing it. The corpus has 36 `hlinkClick`. Planning may want P4's catalogue to say so.
- (c) The tool's `dropped_media` takes "the first root child that holds a `pic`" (tool diff lines 385-387, 455-458). The layer takes the root of `images()[0]`. These could differ in a drawing where `images` skips a picture that `pic` matching finds, for example a grouped picture. On the corpus they agree: run.log "the tool and the comparison agree: 27" on each label, lines 4731 and 5076 of the per-label sections.

## Q2. Do the logs contain every line the spec requires?

YES WITH RESERVATIONS

**All logs.** All four logs end "... log checks all as expected: yes":
- gate.log, line 3340;
- test.log, line 5740;
- negative.log, line 1827;
- run.log, line 7055.

scan-pre-check.log ends "RESULT: PASS". All five stderr files are 0 bytes. Counting check lines ending ": no": 0 in each of the four logs and in scan-pre-check.log. The only UNEXPECTED strings are in printed script source and in gate.log line 2458's control. All logs hold 0 non-ASCII bytes.

**test.log.** Each new crate test (lines 5062-5092) and each tool test (lines 5094-5098) is printed as "passed on both targets: yes". The runs with the features off, images alone, rowcol alone and every feature on are listed in the script header (lines 77-86, 1008-1014). Six planted-change checks pass (lines 5624-5709).

**negative.log.** `--remove-image-sample` runs through the gate's binary in a worktree:
- NM, whose drawing's rels part names a missing media entry, exits 2 with "a drawing part or its relationship cannot be read" and leaves no output (lines 1764-1778).
- NMC is the control and exits 0 (lines 1757-1773).
- The three part messages are stated unreachable, each with its covering test (lines 1789-1794).
- Usage controls U1 and U2 are at lines 1796-1811.

**run.log (attempt 3), against "P2" and "Harness runs".**
- Both samples run on both targets: anchor at lines 4713 and 4830, removeimage at 5060 and 5176. Each prints "summary rows: 27; outcome ok: 27" and "lo ok 27; lo_in ok 27".
- The run log's own comparison has flags `two_form`, `absolute_form` and `kept_children` for anchor; for removeimage it has `drawing_children`, `drawing_rels_set`, `media_dropped`, `types_set`, `wrels_set` and `added_empty`. Each flag has a planted control on a created and a kept case (lines 4952-5014; removeimage 5297-5345). Each ruling 1-3 allowance has an accept-and-refuse control (lines 4676-4693).
- The largest drawing's anchor count is printed (decision 8): line 3322 gives 9, over 37 wsDr parts. Ruling 10 settles 37 against 38.
- The record pairs `m3p6-p2-anchor-` and `m3p6-p2-removeimage-2026-10-07-d3c4a9c` are written and installed (lines 5022-5058 and 5347-5383). Their sha256s match the six files in records/.
- The commentbox rerun matches P1's run by sha256, line and diff table on both labels (lines 5449 and 5517). P1's run directory is named by record commit 9cec1de (lines 4697-4702).
- The TMPDIR and corpus devices are printed (lines 4704-4711).
- The calc chain: 9 of 27 workbooks, counted by both the survey and zipfile (line 3327), with the rule summarised at line 5521.
- The keep is compared against P1's record named by its commit (line 5644 onward). The edit open check runs on the host keep (line 5744).
- The phase commit is compared with HEAD, with controls (lines 41-46).
- At entry and after the run, the status lists only this phase's files and planning's spec at 1fca1264 (line 7048).

**The rerun line against specs/0001 ("its start time, the last check it completed, and the reason given") and ruling 9.**
- Reason: "the host disk holding the WSL image filled" (run.log line 48).
- Start time: 2026-10-06T06:26:28Z. This equals run-attempt-1-superseded.log line 47, "phase: ... run at 2026-10-06T06:26:28Z".
- Last check completed: "(its line 5631)", quoted as "every pair of P1's record is in the keep, unchanged or changed (1046 + 0 = 1046): yes". My `grep -nE '^  .*: (yes|no)$'` over run-attempt-1-superseded.log gives line 5631 as the last match, with exactly that text. The log's tail is cut mid-word ("a_removal_reading_a_missing_m"), as the note says.
- "logged.out holds no run exit line for it": true. logged.out has run lines only for attempt 2 (line 4) and attempt 3 (line 12).
- Attempt 2 is named as superseded for its header gap, with ruling 9 as the reason. Its start, 2026-10-07T12:14:08Z, equals run-attempt-2-superseded.log line 47.
- Both earlier logs are in the rerun files and are hashed by command (lines 49-67):
  - 3df393a5..., 438272 bytes, and c22852ff..., 564604 bytes. Both equal my sha256sum and wc -c of the two superseded inputs.
  - Both script copies hash to b4ab5ff7....
- The typed modification time (06:50:50Z) is gone.
- "Same script": the "## script that wrote this log" blocks of all three attempts are identical, 2615 lines each, sha256 85a0f040... over the extracted block. That is my own computation; no log prints it.
- Apart from the header, attempt 3's body differs from attempt 2's only in timings, run directory names (suffix -b), the working-spec hash (81c98d34 to 1fca1264) and the moved-directory section.

Reservations:
- (a) The quoted start time and last check reach the log as a typed input: RERUN_NOTE, run.log line 8, echoed by script lines 1175-1176. The log does not itself run the `grep -m1 '^phase:'` or `grep -nE` commands the note names. Each quoted item does match the superseded log, which the log hashes by command, so a reader can reproduce it. Whether ruling 9's "quoted from the superseded log by command" means "the log runs the command" or "obtained by a named command" is for planning to say.
- (b) The note says the attempts are kept with "staged and installed records". RERUN_FILES (line 9) lists and hashes only `records-installed/` files.
- (c) Ruling 8 (build pruning) leaves no trace in any P2 log: no listing or removal of the previous phase's targets. Line 116 says run.log used its own `<build>/target`. Check 1's Q2 (d) raised this; I read ruling 8 as taking effect at P3's start, but neither the report nor a log says so.

## Q3. Does every number and quoted string in report-draft.md appear in a primary?

YES WITH RESERVATIONS

**Found in a primary.**
- Commits: d3c4a9c8..., 9cec1de6... and c46adcc9... are in run.log lines 33-35. 7976d9ed... is in gate.log, test.log, negative.log, scan-pre-check.log and commit-messages.txt line 33.
- Spec hashes: 5c3d3d1a... is in run.log line 4 and commit-messages.txt line 68. 1fca1264... is in run.log line 7048 and scan-pre-check.log line 8, and equals sha256 of spec-0028-working.md.
- gate.log ef4dbee4... is in scan-pre-check.log and negative.log; test.log 78c17481... is in run.log and scan-pre-check.log; negative.log 7be4d841... is in scan-pre-check.log. All three equal my sha256 of the inputs.
- run.log ab860da2... is in scan-pre-check.log and equals my sha256.
- The logged.out strings: lines 1-3 and line 12, verbatim. run.stderr is 0 bytes.
- Attempt 1: 3df393a5... and 438272 bytes are at run.log line 49. 2026-10-06T06:26:28Z, "line 5631" and the quoted check are at line 48, confirmed above.
- Attempt 2: c22852ff... and 564604 bytes are at run.log line 57.
- The superseded paths and the moves into `<scripts>/out/superseded`:
  - attempt 2's are at run.log lines 69-124;
  - attempt 1's were made by attempt 2, at run-attempt-2-superseded.log lines 57-86.
- The run directories `-b-anchor`, `-b-removeimage` and `-b-commentbox` are at run.log line 117.
- "summary rows: 27; outcome ok: 27" is at lines 4748, 4865, 5095 and 5211. "lo ok 27; lo_in ok 27" is at lines 4749, 4866, 5096 and 5212.
- "rows: this run 27; P1's run 27; outcome ok here 27" is at lines 5420 and 5488. The commentbox host line is line 5449; i686 is line 5517.
- The survey string is line 3322, verbatim. The "9 of the corpus's 27 workbooks hold xl/calcChain.xml" string is line 5521; the per-label chain checks are lines 4826, 4943, 5172 and 5288.
- "the host keep's listing: pairs 1074; files 2148" is line 5640. "(1046 + 0 = 1046): yes" is line 5681. The open totals are at lines 6826 and 6955.
- The six record hashes are at run.log's install lines (5052-5057, 5377-5382 and the open pair near 7025-7028) and in scan-pre-check.log. All equal my sha256 of records/.
- "RESULT: PASS" is scan-pre-check.log's last line; PLANNING_SPEC is at its line 8.
- R2, R3 and R4 are check 1's Q2 (c), (e) and Q1 (a). Their rulings are 10, 11 and 12 in spec-0028-working.md. daef841a is at gate.log line 25.

**Not in a primary, or weaker than stated.**
- (a) "scan-pre-check.log a8e47d736b92881598b9f88c70cfb966f2d969e6ea6d59c16081922f4407dc40" is printed by no log or record. It equals my sha256sum of the input file, but it is not quoted in any primary.
- (b) "Attempt 3's script prints identically to attempts 1 and 2": no log prints this, and the report says so itself. I reproduced it: 2615 lines each, the same sha256.
- (c) "each hashed on attempt 3's rerun lines": the hashes are on the command output lines 49-67 that follow the rerun line 48, not on the rerun line itself. This is wording only.
- (d) "first attempt each" for gate, test and negative rests on logged.out having one line each (lines 1-3) and on the rerun note's statement. No log asserts it directly.
- Corpus content: the report quotes none. It has no workbook id, name, reference or text.