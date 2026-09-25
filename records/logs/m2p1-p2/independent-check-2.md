# Independent check 2, M2 part 1 P2

Checked commit: e7c7061ba07c0616377bb1abbecf6c7b062da123
HEAD at the check: e7c7061ba07c0616377bb1abbecf6c7b062da123
Checker: a fresh general-purpose subagent (not a fork), given only the files below in one scratch directory. It is the phase's second and last check.
Inputs, each with its sha256 (as given to the checker):
- 0016-new-workbook-and-sheets.md: 3d8b8aa8872d5a25e664ceca73fa9dcd614b38cbd03f135e5678947ac0f69e6f
- 0001-bootstrap.md: 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
- commit-fc82ca7.diff: 9c048fc95809bf4795faf61de1bd81b692534102281198ee5fdacc3bb8b2d408
- commit-5f6505a.diff: 3bd6d95a7099582b5e5a2a7f3567e463423925d9f8f30a5bcd554e9a10adb998
- commit-e7c7061.diff: b70f44e7b94da260adc390b661059f19439df002bae7108f01fa1b26aa0d490f
- combined.diff: 689d63d5813431a5c64ccbee6a074922ec23c5419eaf2a57ae27350620979696
- draft-report.md: 085237caf17998563b64499e0c60b1f02f2c60289b093281578e9db2f2098849
- gate.log: 0891a0f395fb9d7ed36175b47cfaf3b231ebf427f7cada22112845f63770fb2a
- test.log: e37491d431e27a3dd7de7d0e4466ed4e044387ef1a4bc1f0f0f8bc59c29d7feb
- negative.log: 9c4c239125d9a0b66ece253cf633fa7ccdbfbff5d57e479ee2e61d62c943c310
- run.log: ad90c73a9eb38c45206ff818778187bfde008595804038174ed1ea4ccf9676e2
- scan-pre-check.log: bacd1b4e4ef1dbab72c186c246c73aaa3568184962b960be84d303c8d5826957
- independent-check-1.md: bcf04ff6a2b6669ee022694ae62f6e3d33deca70aa26c3356cd92a9bb86ad135
- m2p1-open-2026-09-25-e7c7061.md: 668c716f7b01843399151a323488e02ac1bb20c94793b9b4378f257dea426827
- m2p1-open-2026-09-25-e7c7061.txt: c62398c89614ddaaa950d30ab3f46dc3c57ce44a258d9e829c6bb504acb6428c
The answer is ASCII and holds no absolute machine path; it is reproduced verbatim below (sha256 of the answer text fb3f0c9024bf7e855e321c7dc67d9c1f99145f603d94b43326215df8a9527054, 10481 bytes).

---

Note: the harness put a CLAUDE.md into my context. I did not use it as evidence.

I read only the files in `<scratch>/p2/check2/`. The sha256 I computed for each file matches the draft report's prefixes: gate 0891a0f3, test e37491d4, negative 9c4c2391, run ad90c73a, the record .md 668c716f and .txt c62398c8, and spec 3d8b8aa8. The four diffs have the same sha256 as check 1's inputs (independent-check-1.md 9-12). I split combined.diff into its file blocks by script: there are 14, and each is byte-identical to the block in one of the three per-commit diffs.

## Q1. Does the diff do what the spec says for P2, and nothing else?

Yes. I found no defect.

**commit-fc82ca7.diff (doc commit).**
- It touches two files only: ROADMAP.md (lines 13-27) and specs/0016 (lines 28-190).
- The spec lines it adds are byte-identical to the working-tree spec's lines 651-802. I compared them by script.
- The only thing the working-tree spec holds beyond that is "Planning rulings during P2" (spec 803-838), which spec 837-838 says stays uncommitted until P3.
- gate.log 1021-1045 shows both blobs are at planning's hashes.
- Nothing else.

**commit-5f6505a.diff (code commit 1).**
- styles.rs gains the precondition sentence (lines 26-31), as spec 487-488 and 527-528 ask.
- In excel-check-open.ps1, Close is now in a try (lines 56-60). The catch writes close-error into the message, leaves the result alone, and the loop goes on. That is P1 verification ruling 3 (spec 774-780). The comment is updated at lines 44-47.
- gate.log 1137-1142 shows these are the only two paths.
- Nothing else.

**commit-e7c7061.diff (code commit 2).**
- The six template files are at lines 426-479. Each is the declaration, one LF, then the root, with no newline at the end (marked at 434, 443, 452, 461, 470, 479). This matches ruling 1.
- styles.xml (460) has the counts, the Calibri 11 font, `none` and `gray125` fills, five empty border children, the `xf` entries and `Normal` with builtinId 0 that ruling 2 lists.
- The content types (433) and the rels (442, 451) match ruling 3 and "The template" (spec 103-116). workbook.xml (469) has no calcPr.
- `EMPTY_ARCHIVE` (86-88) and `TEMPLATE` (105-127) are private, and the table is `include_bytes!` in the spec's order.
- `impl Package<Cursor<Vec<u8>>> { pub fn new() }` (129-156) opens the empty archive, adds the six parts, writes them to a Vec and reopens it. This matches spec 137-141 and ruling 6.
- The catalogue note is at 37-42. The limits and the template documentation are at 52-67 and 90-104, which is ruling 5; no README is added.
- The tests are at 211-417 and cover every item in spec 529-538. The tool test is at 699-744.
- `--new-sample` (575-674) writes each file through a temporary moved into place. It removes `<b>` on a failed assertion and keeps `<a>` (594-600). This is ruling 4.
- The usage string and parser are updated (509, 547-549, 557-559, 567), and the refusal vectors are added (687-690).
- gate.log 1143-1163 shows 10 paths, all inside the area, with no lock or manifest change.
- Nothing else, apart from G10.

**combined.diff.** It is the union of the three per-commit diffs above, and holds nothing else.

## Q2. Do the logs contain every line the spec requires?

Yes, and check 1's F1 and F9 are fixed in negative.log attempt 2.

**F1 and F9 in negative.log attempt 2:**
- **Copied, not rebuilt.**
  - negative.log 614 prints the cp command, from `<gatebuild>` into `<tree>/bin`.
  - Line 613 reads the hash from gate.log. It also prints gate.log's own sha256, 0891a0f3..., which equals the file.
  - Line 615 hashes the copy: bc793bcc..., the same as gate.log 1379.
  - Line 616 asserts the match, and line 617 is the control.
- **Nothing built.**
  - Line 36 says "this log builds nothing".
  - The printed script (lines 58-412) contains no cargo build, test or run command. I grepped for it.
- **Root lines.**
  - Lines 41-42 define `<tree>/bin` and `<gatebuild>` accurately, which fixes F9.
  - Line 38 has one wording problem (G3).
- **Rerun header.**
  - Line 32 names attempt 1 and says why it was rerun.
  - Lines 33-35 print the sha256 and size of attempt 1's log and script, from files that existed at run time. The log hash, f459b1e6..., equals check 1's input (independent-check-1.md 16).
- **Printed commands name the copy that runs.** Every N case prints `<tree>/bin/emend-copy` (lines 620, 632, 639, 646, 655, 662), and run_bin prints the path it executes (lines 279-288).
- **The spec's own requirement.** The statement that nothing new is reachable is at 671-676, as spec 539-540 asks.
- **Commits and HEAD.** The phase commit is compared with HEAD, with a control (52-54). The commit ids are read from git (17-29).

**Other P2 requirements, checked in the other logs:**
- Template sha256s: run.log 966-979.
- `--new-sample` on both targets, with the line: run.log 988-1005.
- The two targets compared: run.log 1006-1008.
- The pair diffed: run.log 1030-1070.
- All four files opened through the pinned image, with a truncated-file control: run.log 1072-1080.
- The keep: run.log 1188-1213.
- The edit open check: run.log 1222 to about 1535.
- The record pair installed: run.log 1537-1556.
- The stage rebuilt from the same list (bcda16d9... unchanged) and the new zip's sha256: run.log 1557-1576.
- The gate files checked against the diff: gate.log 1100-1135.
- 531 tests on each target: test.log 1291 and 1870, with 523 for the previous phase at 1901.

**Requirements check 1 did not look at:**
- The runner is not run over the corpus (spec 365-366). run.log only reads corpus files for the stage (root line 39 and 1562). Present.
- The scan log comes last (specs/0001 22-38). It does not exist yet: scan-pre-check.log 1373 hashes negative.log attempt 1, f459b1e6. No scan has covered attempt 2 or either check answer (G2).

**My own greps of negative.log attempt 2:**
- 0 lines of each of the three forbidden shapes.
- 0 non-ASCII bytes and 0 control bytes.
- One drive-letter form, at line 688. It is the planted control of family P, not a real path.

## Q3. Does every number, hash and quoted string in draft-report.md appear in a primary?

Almost all do. Each item below is followed by where I found it:
- Report line 4 (commit ids, parent, 75479a6b, 9e10adfe): negative.log 19-27; gate.log 1025-1030.
- Line 15 (3d8b8aa8): gate.log 1389-1390; run.log.
- Line 18: gate.log 1354-1365 and 1137-1163. Line 19: test.log 1291, 1870, 1901.
- Lines 21-27: negative.log 613-617, 620-700; the message is at 659 and 666.
- Line 29: run.log 977. Lines 30-31: run.log 991-1005. Line 32: run.log 1022-1024. Line 33: run.log 1055-1068. Line 34: run.log 1074-1079.
- Line 35: run.log 1188-1201. Line 36: the run.log verdicts section and the open-check summary line.
- Line 37: run.log 38 (the stage's root line) and 1557-1576. 99b3376d is in commit-fc82ca7.diff 165.
- Line 40: run.log 1550-1554.
- Line 45: scan-pre-check.log 1505.
- Line 48: negative.log 32-35; independent-check-1.md 215.
- Line 51: the quotes are at spec 99-100 and 543.
- Line 55: commit-5f6505a.diff 8-9. Line 58: commit-e7c7061.diff 11.

Items not in a primary:
- The negative.log hash on line 20, 9c4c2391. No log prints it: it matches my own hash of the file, but a log cannot hash itself. It should appear in this check's header and in the final scan (G7).
- Line 44's four rustfmt'd files, with "styles.rs needed none", is an unlogged disclosure. It is consistent with test.log 1923, which counts 5 changed .rs files.
- Line 17's "first attempts" cannot be confirmed from a primary. No log carries a rerun header, which is consistent with it.

**Does the report state check 1's findings fairly?** Mostly:
- F1 and F9 are stated, with what was changed (lines 20-21, 47-48).
- F2, F3, F4, F5 and F8 are stated fairly, each with its proposal or why it stands.
- F6 and F7 are left out. Lines 14 and 44 were rewritten to fix them, but the report does not say so (G1).

## Findings

- **G1 (gap).** The report's list of check 1's findings (draft-report.md 50-58) leaves out F6 and F7 (independent-check-1.md 204-208). Lines 14 and 44 were rewritten to answer them, but the report does not say so. It should name both findings and say they were fixed in the report text.
- **G2 (gap).** Report line 45 gives the pre-check scan's PASS without saying that the scan hashes negative.log attempt 1 (scan-pre-check.log 1373). No scan has yet covered attempt 2 or either check answer. The final scan must hash attempt 2 (9c4c2391...) and both answers, and the report should say this is still to come.
- **G3 (nit).** negative.log 38 says `<repo>` has "nothing in it ... run". Yet scripts/counting.sh is sourced from `<repo>` (lines 117, 248-249), and the log does not check it against the phase commit the way it checks the .ps1 (lines 255-262).
- **G4 (nit).** The printed invocation (negative.log 2) leaves out the RERUN_NOTE and RERUN_FILES environment the run used (lines 210-221). The superseded files are named by basename only (lines 34-35), so the log does not say where attempt 1 is kept.
- **G5 (nit).** negative.log still carries build-oriented leftovers that do nothing now: the cargo exports (121), the cp output captured as `build.out` (272), and that file counted as a captured output (394).
- **G6 (nit).** `roots_distinct` (negative.log 233) does not include `<gatebuild>`, although line 42 defines it as a root.
- **G7 (nit).** The negative.log hash on report line 20 (9c4c2391...) is not yet printed by any primary; see Q3.
- **G8 (nit).** Report line 1 still reads "draft, before the independent check", but this draft was revised after check 1.
- **G9 (nit).** negative.log 672 repeats the claim check 1 flagged as F5, that neither write_to nor open can refuse. The report's F5 proposal (line 57) should reword this log sentence's source too, or note that it stands as recorded.
- **G10 (nit).** The tool edits the first sheet whose kind is Worksheet (commit-e7c7061.diff 583-589), where the spec says "its first sheet" (spec 359). On the fixed template the two are the same.
- **G11 (nit).** In the negative script's header comment, the ATTEMPT 2 paragraph sits between items 2 and 3 of the numbered "What differs" list (negative.log 98-107).

Verdict: the P2 diff does what the spec and P2 rulings 1-6 ask and nothing else, and negative.log attempt 2 fixes F1 and F9. What remains are two gaps (G1: F6 and F7 missing from the report; G2: attempt 2 and the answers not yet scanned) and nits, none of which changes a result.
