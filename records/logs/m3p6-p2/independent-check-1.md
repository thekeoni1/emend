# M3 part 6 P2, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1/inputs (the committed spec specs/0028, planning's working copy of it as it stood then, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log ATTEMPT 2, run.log attempt 1 (superseded), the scan's pre-check over attempt 2, the stderr files, logged.out, the records attempt 2 wrote, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p6-p2. Checked commit (the phase commit): d3c4a9c8dc7dbc1b3c5ce1f11894fe5bd0cc5144; HEAD when the check ran: d3c4a9c8dc7dbc1b3c5ce1f11894fe5bd0cc5144. The answer read run.log attempt 2 (sha256 c22852ffb8c0a70a29affac2e200b7fdb3f9da698c10fc64d80ac86ec544ae8a), which is superseded: its Q2 reservation (a), the rerun header missing the last check attempt 1 completed, led to planning's ruling 9 (specs/0028 "Planning rulings during P2") and run.log attempt 3, the committed run.log; its records and the pre-check it read are superseded with it. Its reservations R2 to R4 were ruled by rulings 10 to 12. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 3718c8281eb71f3f0268955df393914d6d2e547ca5fc555447326e13d84f32fb. Nothing in the copy below is changed. Independent check 2 read attempt 3.

Inputs and their sha256:

- 91348bb00a4bcc1dc2abc5fe4e0759bdae6bd61d9362676f740838c6ca31da97 code-commit-1-crate.diff
- 860c1f9b1ac87925fbc593d937ce4f066c61509a977b64e0cc371efe046b5a60 code-commit-2-tool.diff
- db0bfd0109fa59142de6a7ea8906363196704e6018460ac5d6a6d57e4c22d003 commit-messages.txt
- 666641af0b48e7078c4294512aa705c03dd119be6e0438f20140020ced2b4b47 doc-commit.diff
- ef4dbee46785ed838ba0eb7a8d12c8ae30939533a349731f7bb3551bc2e659ec gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- d0c35bf0376e4af88a851a48c24d80f02cc6908173d5507a7a21be6f452054d6 logged.out
- 7be4d841111cb559762bd1d197107483e5998964fe4f9812185887936cd31722 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- c322a37ba800636266b01f7f99ced5ee9f5b3a18c432451d20a73b2f97cfce93 phase.diff
- a27e4067d9f8604245ee01291537a895aaed1aea18379b157e4fb68ec0f0a1fb records/m3p6-open-2026-10-07-d3c4a9c.md
- 30d916cabffc4a4f484b182fbc084669159f84e9d7f5ab1b959551d10e3d77d7 records/m3p6-open-2026-10-07-d3c4a9c.txt
- 9ebe9951b9a2ee78f2fc2169ef133d784dc0612febb05672ad5ad23f14e19200 records/m3p6-p2-anchor-2026-10-07-d3c4a9c.md
- cec628f11caa9a081da90cc8b07762962e7293029542c37cebda58ebabf29485 records/m3p6-p2-anchor-2026-10-07-d3c4a9c.txt
- 231f8052991d99d56138b6d6dbd7e85d4d3716a7ea2ff202eaa16cf66712782c records/m3p6-p2-removeimage-2026-10-07-d3c4a9c.md
- 9b8c5c1faeea9a2dbe39e9bf97de22bf6bf4e90495d1eb4e44ecca80f62caf42 records/m3p6-p2-removeimage-2026-10-07-d3c4a9c.txt
- d5f7dbcf88b0e46801b8d7adf7fd46bed3362aa7d05b3f8ac6c5de781ff6c797 report-draft.md
- 3df393a5c6a0bcc59759387bedfc29977b0a5f4f7da4755970bb81eacc255e0f run-attempt-1-superseded.log
- c22852ffb8c0a70a29affac2e200b7fdb3f9da698c10fc64d80ac86ec544ae8a run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- cff5cbb3c0bc3fe4b87ee9fd24dca1d38af4120caa55eeca489c63bb1506e371 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 5c3d3d1aa4d5bad2dac42cf7c0836a1d80d93a2dfc7e76cf94a48aacd444e6b0 spec-0028-committed.md
- 81c98d34ead0928c062456ff369f3687c969454ee431def8cebc01b460742d2a spec-0028-working.md
- 78c174817d8ee2456bfa1274f4c202fe9a64551afd7a1b2c608cc3188999870b test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

## The answer

**Independent check 1, M3 part 6 P2 (inputs: <phase>/check1/inputs)**

Scope: I read only the files in <phase>/check1/inputs and ran only read-only commands. The harness also put the project CLAUDE.md into my context. I did not use it as evidence.

Q1. Does the diff do what the spec (with planning's P2 rulings) says for P2 and nothing else?
YES WITH RESERVATIONS.

Diffs read: doc-commit.diff, code-commit-1-crate.diff, code-commit-2-tool.diff and phase.diff. I split phase.diff by file and compared it with the three per-commit diffs, ignoring `index` lines. The two hold the same 9 files with identical hunks, so phase.diff contains exactly the three commits.

- **Doc commit (doc-commit.diff).** Two files only. ROADMAP.md changes one line: the P1 row goes to DONE. specs/0028 adds "Planning verification and rulings on P1". gate.log and the scan pin specs/0028 at 5c3d3d1a, which matches the sha256 of spec-0028-committed.md. Nothing else changed.
- **Crate commit, `ImageAnchor`, `EditAs`, `add_image_at` (code-commit-1-crate.diff).**
  - Lines 1-13: `lib.rs` re-exports the two new types under `images`.
  - Lines 125-162: the enums carry the derives "Anchors" names.
  - Lines 205-210: `add_image` is now `add_image_at` with a one-cell anchor and offsets (0, 0).
  - Lines 291-364 hold the call checks, in the order ruling 7 accepts: an off-grid cell gives INDEX_OUT_OF_RANGE; the range checks against i32::MAX EMU and `to` before `from` give IMAGE_ANCHOR_OUT_OF_RANGE; then a size on a two-cell anchor gives IMAGE_SIZE_TWO_CELL; then the image's own checks. No extent is checked on a two-cell anchor (ruling 7).
  - Lines 777-891, `anchor_bytes`: a one-cell anchor writes its offsets as EMU through `marker`, so offset 0 prints "0" and `add_image`'s bytes are unchanged. A two-cell anchor always writes `editAs`, then `from` and `to`. An absolute anchor writes `pos` and `ext`.
- **Crate commit, `remove_image` (code-commit-1-crate.diff).**
  - Lines 248-251 and 372-377: `remove_image` only records the removal at the call.
  - Lines 434-583, `record_image_removals`, does what "Image removal" says:
    - an index named twice, or on a part this write also adds to, gives NO_IMAGE_AT;
    - a removed, replaced or shifted part gives part 1's and part 3's messages;
    - an index past the list that `images` reads gives NO_IMAGE_AT;
    - the root child holding the picture is skipped whole, an AlternateContent included (ruling 4) and a group's anchor included (ruling 5);
    - an `embed` or `link` id that no kept root names is removed from the rels part;
    - K is computed once by `reachable` with the removed relationships in `skipped`, and a dropped drawing's sheet relationship is skipped too (ruling 6);
    - an emptied drawing is dropped with its rels part, its Override, the sheet's relationship and the `<drawing>` element (lines 569-583, 637-648).
  - Lines 606-617: the three message constants are added.
- **Crate tests (code-commit-1-crate.diff, from line 1001).** anchors.rs and removal.rs cover every test the "P2" section lists, plus an AlternateContent test and a missing-media test.
- **Tool commit (code-commit-2-tool.diff).**
  - Lines 1-622: `--anchor-sample` and `--remove-image-sample` with the spec's constants, printed fields and assertions. `media_dropped` is checked against the tool's own pass over every rels part of the archive (`dropped_media`).
  - Lines 485-622: tool tests for created and kept fixtures, and the missing-media refusal.
- **Commit messages (commit-messages.txt).** All 3 are ASCII. Each ends with the Co-Authored-By line only and says that no log prints a count for that commit.

Nothing outside P2's scope was found.

Reservations:
- (a) Cosmetic, in the crate diff at lines 619-632. The new `drawing_drop` block sits between the existing `<legacyDrawing>` comment and the code that comment describes. The legacyDrawing comment now reads as if it belonged to the drawing branch.
- (b) The three new messages are catalogued in the crate commit's `package.rs` header (diff lines 18-36), ahead of P4's catalogue work. This follows P1's precedent, where messages were catalogued in the same commit, so it is not a scope breach.

Q2. Do the logs contain every line the spec requires?
YES WITH RESERVATIONS.

All five logs end "... all as expected: yes". scan-pre-check.log ends "RESULT: PASS". All stderr files are 0 bytes. Grepping for ": UNEXPECTED" and for a final ": no" check finds 0 in every log.

- **gate.log.** Pins the spec blob at 5c3d3d1a (lines 21-27). Reads the commits by role from git and checks the phase commit against HEAD, with controls (lines 29-63). Also has the commit-message ASCII section (2461), the gate-files section (2559), the per-commit areas with the three messages catalogued (2614), clippy and doc runs (3113), crate tests with features off and on on both targets (3191), and the consumer crate (3257).
- **test.log.** Runs the workspace on host (2549) and i686 (3692). Runs the crate with features off, images alone, rowcol alone and images with rowcol on both targets (5104), and every feature on (5205). Prints six planted changes, each against its named test (5599).
- **negative.log.** Runs `--remove-image-sample` through the gate's binary in a worktree (lines 1604-1611) on three fixtures. NM, whose rels part names a missing media entry, exits 2 with DRAWING_UNREADABLE's message and leaves no output. NMC, the same package with the entry, is the control (lines 1750-1786). The three part messages are stated unreachable through the tool, each with its covering test (lines 1650-1655).
- **run.log (attempt 2).**
  - Both samples run on both targets (4701-5335). Both record pairs are written and installed under the named stems (5010, 5335).
  - The largest drawing's anchor count is printed, as decision 8 requires: line 3310 gives 9.
  - The commentbox rerun on both labels matches P1's run by sha256, line and diff table, with P1's run named by its record commit 9cec1de (5373-5441).
  - The calc-chain workbooks are counted with the rule (3313-3318, plus the summary after 5441).
  - The TMPDIR and corpus devices are printed (4692-4699).
  - The keep and its comparison against P1's record are named by commit 9cec1de (5515-5731), and the edit open check runs on the host keep (5732).
  - Rulings 1 to 3 are implemented as the `entries_remove` rules, with a control per allowance (script lines 1655-1796, run-log line numbers). Their outcome is "entry rows as the rules give: 27" on both labels.
- **The rerun line (run.log line 48, also RERUN_NOTE at line 8).** It names the cause (the host disk filled), the start ("started after negative.log at 06:26:28Z"; attempt 1's own header at line 47 says it ran at 2026-10-06T06:26:28Z) and the reason. Lines 50-56 hash each superseded file with sha256sum and wc. Attempt 1's log is listed as 3df393a5... with 438272 bytes, which matches the sha256 of run-attempt-1-superseded.log in my inputs. Its stderr, its script copy b4ab5ff7... and its four installed records are hashed the same way. The superseded run directories and the keep are moved, each with "moved, not removed ...: yes" (lines 57-82).

Reservations:
- (a) specs/0001 ("A run stopped by the environment ... its start time, the last check it completed, and the reason given") also asks for the last check completed. The rerun line does not name it. In run-attempt-1-superseded.log, the last check line is line 5631, "every pair of P1's record is in the keep, unchanged or changed (1046 + 0 = 1046): yes"; after it, the log is cut off mid-word in the test-name listing. The rerun line gives only "its last write, 06:50:50Z". That time is not printed by any command in any input; it appears only in the typed note.
- (b) No log prints a hash of the script attempt 2 ran, so "the same script" rests on the note. As my own evidence, I compared the "## script that wrote this log" sections of the two run logs: they are identical, 2615 lines each.
- (c) Spec "32-bit and streaming" gives the corpus figure as 38 drawings. run.log line 3310 prints 37 drawing parts "whose root is wsDr", in 16 workbooks. Neither the log nor the report reconciles 37 with 38. The P4 catalogue will state this figure, so planning should rule on which is right.
- (d) Ruling 8 (build pruning, "from this phase on ... at a new phase's start") leaves no trace in any P2 log. gate.log, test.log and negative.log predate the ruling. I read it as starting with P3; planning may want to confirm that reading.
- (e) The logs print two different hashes for the working-tree spec. gate.log line 25, test.log and negative.log line 69 (and attempt 1) print daef841a. run.log lines 2931 and 7033 and scan-pre-check.log line 8 print 81c98d34, which is spec-0028-working.md's hash. This is consistent with planning adding ruling 8 after the disk filled, but no line records the change.

Q3. Does every number and quoted string in report-draft.md appear in a primary?
YES WITH RESERVATIONS.

Checked and found:
- **Commits.** The 4 commit shas and the base are in gate.log lines 33-41 and commit-messages.txt.
- **Spec hashes.** 5c3d3d1a... is in gate.log line 23. 81c98d34... is in run.log line 2931, in scan-pre-check.log line 8, and is the sha256 of spec-0028-working.md.
- **Log hashes.** The gate, test, negative and run hashes match the input files and scan-pre-check.log lines 2608-2638.
- **Records.** The six record hashes match the files in records/ and run.log's install lines (5040-5046, 5365-5371); the open pair matches scan-pre-check.log lines 2705-2715.
- **logged.out.** All four lines in the report are verbatim, including "13:52:08Z".
- **Attempt 1.** 3df393a5... and 438272 bytes are in run.log line 50; the script copy b4ab5ff7... is at line 52; the 2026-10-06 run directories and "moved, not removed" are at lines 57-82.
- **The run.**
  - "summary rows: 27; outcome ok: 27" and "lo ok 27; lo_in ok 27" appear in each of the four anchor and removeimage sections.
  - "rows: this run 27; P1's run 27; outcome ok here 27" and the commentbox (change VF) line are verbatim at both labels.
  - The survey line is verbatim at run.log line 3310 (37, 16, 9).
  - "9 of the corpus's 27 workbooks hold xl/calcChain.xml" is in run.log after the commentbox i686 section.
  - "the host keep's listing: pairs 1074; files 2148" is in the keep comparison section. "(1046 + 0 = 1046): yes" is at run.log line 5669.
  - The open totals "pairs 1074; lo_in ok 1062 fail 12 error 0; lo ok 1063 fail 11 error 0" are at run.log lines 6814 and 6943.
  - "RESULT: PASS" is at scan-pre-check.log's last line.

Not in a primary, or weaker than stated:
- (a) "planning's message of 2026-10-07 reports the WSL image compacted": not in any primary. The report says so itself.
- (b) "attempt 2 ran the same script (b4ab5ff7...)": b4ab5ff7 is printed only as the hash of attempt 1's kept copy (run.log line 52). No log prints attempt 2's script hash. Only the identical printed script text (Q2 reservation b) supports the claim.
- (c) "Its 'rerun:' line ... hashes attempt 1's log ...": the hashes are on the command lines after the rerun line (run.log lines 49-56), not on the rerun line itself. This is wording only.
- (d) "not edited, staged or reverted by this session" cannot be checked from the primaries. The report also leaves out that gate.log, test.log and negative.log print a different working-copy hash, daef841a (Q2 reservation e).
- (e) The report does not mention the 37-versus-38 drawing count (Q2 reservation c).
- **Corpus content.** The report quotes none: no workbook id, name or text.