# Independent check 2, M2 part 4b P3

Written by the implementing session: the checked phase commit is
631c719615f1503448d489f5ca37f7412af190f5, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check2 in this phase's folder on the disk (its prompt named it
<phase>/check2 and asked for <home> and <repo> in place of machine paths),
holding these files, by sha256 (superseded/at-c2fe42b/ is a copy of the
superseded logs, records and staged records kept on the disk; the scan
pre-check's predecessor was also copied, byte for byte, to
<phase>/superseded/scan-pre-check-at-c2fe42b.log, the path the new
pre-check's rerun lines name, which answers the checker's Q2 item 2):

    3b8204ee49eb8815957c55c83b70f55184b887e7ce8fe7cd7c877668925f7676  commit-messages.txt
    576fff2d64ed7ed06cad0e74aa6fce61c8b9b1875546f493f747317d12010b75  diff-03bbe82.diff
    26934aa733177db22f102413ea00c603483e43d96025619ed3a959fc281a663f  diff-1265000.diff
    723a747dd530a8f991a50b4fb7ebda09509c7fcdf6d425e5d5012c2e60e4e0c6  diff-631c719.diff
    11d660246c8ce7b1fdb95209f0eaa5fc02bd91d9ae56184c1ff17074699f0b6f  diff-c2fe42b.diff
    97af50da906abbcb516922a479fa30ca72d75153e7d378a454fc440d1708f1b4  diff-phase-0999a35..631c719.diff
    351bcd2634937a58ea427ce7b25302fdd637f174b0b118eef912bbc7153d760a  doc-commit-b88f686.stat
    07776a1b0f5f0288494f4884c9b310cd6ce3bf8a8be9ff60f907beaff3f8cab2  gate.log
    27d0a2d787d79bd998b441e36336e87399f23b46b9046272f86099c87034ee60  independent-check-1.md
    0c815c7f79c4846c3799d1dc9d89eb0c1b398a813361cf0643c22761694ac06c  m2p4b-open-2026-09-30-631c719.md
    599e2d136bb48ad9c614f21f3bad9ab69808a74844bf7b6b77096f5a0ec32bfc  m2p4b-open-2026-09-30-631c719.txt
    d7de08ba10feb2fbe2cbba6ff13397c06a397f9f652d3c55261499e4827f4095  m2p4b-p3-2026-09-30-631c719.md
    750b87dadda036101db1d7016b26913a2ea12d9c148a47bdf62ec4f289a92d18  m2p4b-p3-2026-09-30-631c719.txt
    9cb49fe88adac9861127dba70ac3a8b9a46c8741bdf35a1d9826ff8757585ad6  negative.log
    13be2bfe3b9d254e8979182814564dcfe5d6995e4788acdb3ae2c368b043b475  report-draft-2.md
    85e789efe1cc244c463b264d0954eabcabfd6f17d7d7a46d9b6c2445f2eeddbc  run.log
    2321a7fda84e5d60fa2c2fc2769fc6b82998cd758785ff3cbe403c6a6c611496  scan-pre-check.log
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  spec-0001-bootstrap.md
    c98838a0433cb7347dbe61e3af3dfabecc8ad0f00fba2661760553da96cd0188  spec-0020-committed.md
    ab4bc4a693b426c18143d1e4248dc47067f5641dbd302075ccb42c6126bbb167  spec-0020-working-tree-with-P3-rulings.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/test.stderr
    097c7566fa685415a6c3ce13b63d41aaa7a3d946b46588a9886e7f052382d7b8  superseded/at-c2fe42b/logs/gate.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded/at-c2fe42b/logs/gate.stderr
    b9bff6884dac17a545c3cba7a50d62c40dcb306949869faf7ae9725bcd673668  superseded/at-c2fe42b/logs/negative.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded/at-c2fe42b/logs/negative.stderr
    7be2efa70a1d249dc320d3f05537edb42d5d6fce51ffda0491a68683cca39a5c  superseded/at-c2fe42b/logs/run.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded/at-c2fe42b/logs/run.stderr
    23c7d955b22aaa309b78137d7f82fc412fc12909437027597ff590b950bfde95  superseded/at-c2fe42b/logs/scan-pre-check.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded/at-c2fe42b/logs/scan-pre-check.stderr
    f12314c6eae71e99f3069ccc9d95f970435683580239852dce6e1d39c76dc633  superseded/at-c2fe42b/logs/test.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded/at-c2fe42b/logs/test.stderr
    48b00c1319e72400667564baa4de04c4890635c8244ad78a976fefa222b66da0  superseded/at-c2fe42b/records/m2p4b-open-2026-09-29-c2fe42b.md
    599e2d136bb48ad9c614f21f3bad9ab69808a74844bf7b6b77096f5a0ec32bfc  superseded/at-c2fe42b/records/m2p4b-open-2026-09-29-c2fe42b.txt
    2b72620988c8e14429ecf01c803d6651ebd0ff9f64b6bc47f8d1768d3a7bfac5  superseded/at-c2fe42b/records/m2p4b-p3-2026-09-29-c2fe42b.md
    8a9ac79754a5ae724a98ca6d299f0035e6b78ee2445595ab9c63cb8d45833e9c  superseded/at-c2fe42b/records/m2p4b-p3-2026-09-29-c2fe42b.txt
    48b00c1319e72400667564baa4de04c4890635c8244ad78a976fefa222b66da0  superseded/at-c2fe42b/staged/m2p4b-open-2026-09-29-c2fe42b.md
    599e2d136bb48ad9c614f21f3bad9ab69808a74844bf7b6b77096f5a0ec32bfc  superseded/at-c2fe42b/staged/m2p4b-open-2026-09-29-c2fe42b.txt
    2b72620988c8e14429ecf01c803d6651ebd0ff9f64b6bc47f8d1768d3a7bfac5  superseded/at-c2fe42b/staged/m2p4b-p3-2026-09-29-c2fe42b.md
    8a9ac79754a5ae724a98ca6d299f0035e6b78ee2445595ab9c63cb8d45833e9c  superseded/at-c2fe42b/staged/m2p4b-p3-2026-09-29-c2fe42b.txt
    750592698a095e6840b44c5a1e02f5c3ba43960ab0102a47daf8409f45863bb3  test.log

The per-commit diffs and the other derived inputs were made by these
commands, in the repository (specs/0020 P1 verification ruling 6):

    git diff 03bbe82^ 03bbe82 > diff-03bbe82.diff
    git diff 1265000^ 1265000 > diff-1265000.diff
    git diff c2fe42b^ c2fe42b > diff-c2fe42b.diff
    git diff 631c719^ 631c719 > diff-631c719.diff
    git diff 0999a35 631c719 > diff-phase-0999a35..631c719.diff
    git diff 0999a35 b88f686 --stat > doc-commit-b88f686.stat

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 ee2d6ce3617f65ce4c84824f3c35b2e3a1f018dddab3430b2507e762f39aa258,
10158 bytes; plain ASCII; no machine path; nothing replaced; one line feed
added at its end). This is the second and last check the rules allow; what
it found goes to planning in the report.

---

I read only the files in <phase>/check2. The harness also put the repository's CLAUDE.md into my context; I did not use it as evidence. No workbook content appears below. Corpus rows appear only as the ids and counts the logs already print.

Overall: I found no errors in the diff, the logs or the report's numbers. The problems I did find are in paths and in what the report discloses: Q2 #2, Q2 #9 and Q3 #2.

**1. Does the diff do what the spec says for P3, and nothing else?**

Which diff I read for each "nothing else":
- **Code commit 4:** diff-631c719.diff (sha256 723a747d..., which matches the hash gate.log prints for `git show 631c719`).
- **Code commits 1 to 3:** diff-03bbe82.diff, diff-1265000.diff and diff-c2fe42b.diff. Their hashes equal the ones check 1 listed, so they are unchanged.
- **The whole phase:** diff-phase-0999a35..631c719.diff (97af50da..., which matches gate.log).

1. **The combined diff is exactly the four code commits plus the doc commit: fine.** I counted lines per file:
   - package.rs: 324+/6- in 1265000, then 8+/3- in 631c719, gives 329+/6- in the combined diff.
   - visual.rs: 373 + 11 - 2 = 382.
   - Every other file's count equals its single commit's.
   - There are 11 files and no others. gate.log's "added lines: 329" (up from 324) agrees.

2. **diff-631c719.diff against P3 ruling 2: it does exactly what the ruling says, and nothing else.**
   - The code change is one line in `cfvo_parts`: `rule_formula_ok(text).map_err(|_| VISUAL_VALUE)?` becomes `rule_formula_ok(text)?`.
   - The empty formula still falls to `_ => return Err(VISUAL_VALUE)` through the `if !text.is_empty()` guard. That matches "the value message is the empty formula's alone".
   - The "=A1" case now asserts "a formula text starts with an equals sign". A new case, `Formula("A\u{1}")`, asserts "a text holds a character XML cannot carry". The source stays ASCII because the character is written as an escape. That matches "a forbidden-character case is asserted beside it".
   - The only other changes are the catalogue comment and the `cfvo_parts` doc comment. The commit message names both.
   - gate.log confirms code commit 4 touches only package.rs and package/tests/visual.rs, with a control, and that the pub-line counts are unchanged.

3. **One limit I could not check.** `rule_formula_ok`'s body appears in no diff. That it returns exactly part 4's two messages rests on the test's assertions and on test.log: 8 "... ok" lines for the refusal test, and X10's planted output showing `right: "a formula text starts with an equals sign"`. The forbidden-character message is proved only by the test passing; no plant targets it on its own. That is acceptable, because X10's single line covers both paths.

4. **The catalogue entries for part 4's two messages.** 631c719 notes the reuse under the VISUAL_VALUE entry. Those two messages' own entries, which are outside the diff, presumably do not list the visual test as a place they are raised. Nothing requires it, but P4 (the catalogue) should check.

5. **P3 rulings 3 and 4 need no code, and none was changed: fine.** `visual_color` refuses a NaN tint, because `(-1.0..=1.0).contains(NaN)` is false, so "not finite" in ruling 3 holds. Only `tint: Some(1.5)` is tested; a NaN or infinite tint is untested. Check 1 did not raise this.

6. **The spec's working tree matches the rulings: fine.** It differs from the committed spec only in the three corrected sentences (the CfvoKind bullet, the Errors reuse list and the P3 test list) plus the new "Planning rulings during P3" section.

**2. Do the logs contain every line the spec requires?**

1. **Each new log names its predecessor by a hash that matches the kept file: fine.**

   | New log | Hash it names for its predecessor |
   |---|---|
   | gate | 097c7566... |
   | test | f12314c6... |
   | negative | b9bff688... |
   | run | 7be2efa7... (log), plus 2b726209..., 8a9ac797..., 48b00c13... and 599e2d13... (records) |
   | scan-pre-check | 23c7d955... |

   Each equals the sha256 of the file under superseded/at-c2fe42b/, and each byte count matches. The staged/ copies hash the same as records/, but no log names the staged/ directory.

2. **Path mismatch (finding, minor).** scan-pre-check.log lines 7 and 58 name the predecessor as `<phase>/superseded/scan-pre-check-at-c2fe42b.log`. The kept file in this directory is at superseded/at-c2fe42b/logs/scan-pre-check.log, and the other four logs use that at-c2fe42b/logs/ layout. The hash matches, so the content is the same, but the named path differs from where the others say superseded files live. Either a second copy exists or the file was moved after the scan; no primary here says which.

3. **gate.log's RERUN_FILES names only gate.log, not gate.stderr.** Both stderr files are 0 bytes, so this has no effect.

4. **Differences from the superseded logs: everything is explained, with two harmless extras.** I compared all five logs and all four records, normalising timings, thread ids and keep-directory pids for test.log.
   - **What the new commit explains:** the phase sha and role lines (~5 in place of ~4); a new 631c719 message check; the new spec working-tree hash (ab4bc4a6... in place of d5928edb...); the new package.rs and visual.rs hashes; the binary hashes and sizes; "added lines" 324 to 329; X10 and "fourteen plants"; the rerun lines; the record stems and dates.
   - **Unchanged:** test totals (807/0/2; the crate 512 and 546), pub lines (134 and 141), every run.log per-row verdict, the totals (20/101, 10/91, 4), and the negative log's reached and unreachable sets.
   - **The p3 records** differ only in the "seconds" and memory columns, plus one timing (4.51 to 4.41). The input and output byte counts are identical.
   - **The open .txt record is byte-identical** at 599e2d13... under both stems, so it carries no commit or date. The .md differs only in stem and time.
   - **Two differences the commit does not explain, both environmental:**
     - gate.log line 2199: "Removed 5882 files, 1.5GiB total", where the superseded log had "Removed 0 files". That is the clean step, and it simply reflects a warm target directory.
     - The emend-copy memory column in run.log varies.

5. **The plants still land where their descriptions say at 631c719: fine.**
   - Each plant is "found 1 time(s)", passes unplanted and fails planted, on an archive of 631c719 (782 files).
   - X1 to X4 now panic at visual.rs:297:18 and X5 at 312:14. Before, they panicked at 288 and 303. The shift of 9 lines equals the test hunk's net +9 (-2/+11).
   - X6 to X9 and Y1 to Y4 panic at the same lines as before; their files and the code around them are untouched.
   - X10's replacement text is exactly the c2fe42b line from diff-1265000.diff. It fails at 294:9 with left "a colour scale, data bar or icon set value is not valid" and right "a formula text starts with an equals sign", which is what its description says.
   - The planted files are restored.

6. **The run log names the commit it runs (ruling 2): fine.** It does so in the about line, in "Commit: 631c719..." in the p3 record, and with a new check that the tool commit is the phase commit's parent, which has a control. The old keep was moved, not removed: 561 pairs, listing hash 01193267....

7. **negative.log: fine.** Its note AT explains why the reached and unreachable sets do not change: part 4's messages lie outside the block the log reads, and the sample holds no formula.

8. **Check 1's findings on the logs, now resolved:**
   - The leading "=" now has a plant (X10). The empty formula, NaN, the middle point, the tint, Min in an icon set and `visual_after` still have none.
   - The report's "at the read" wording has been removed.
   - The report's code 2 path wording has been fixed.
   - Check 1's Q1 item 3, the Errors departure, is resolved by ruling 2 and 631c719.

9. **Check 1's findings still open:**
   - X1 to X5 still print only "refused: ()" (test.log around lines 3620 to 3660), so the log does not show which case was let through.
   - negative.log lines 991-992 and 994-995 still print the same two C3 checks twice.
   - `cf_place`'s whitespace assumption and the implied run.log checks stand as check 1 described them.
   - There is still no test for Theme with no tint, the prefixed root, a middle Min, a Max threshold, or DataBar Min/Max placement.

**3. Does every number and quoted string in report-draft-2.md appear in a primary?**

I found every one:
- **Hashes:** c98838a0... and ab4bc4a6... (gate.log and scan-pre-check.log).
- **Commits:** b88f686, 03bbe82, 1265000, c2fe42b, 631c719 and 0999a35.
- **gate.log line 2118:** 134 and 141.
- **test.log lines 2226 and 3081:** 807/0/2 on both targets. X1 to X10 and Y1 to Y4 each "found once".
- **run.log lines 2049, 2153, 2111 and 2215:** 27, 27, 0; and 20/101, 10/91, 4.
- **scan-pre-check.log:** "RESULT: PASS" and 0 total hits.
- **The four "all as expected: yes" lines.**
- **Colour strings:** "00F8696B", "0063BE7B" and "00638EC6" are in diff-c2fe42b.diff. "P2 verification ruling 7" is item 7 of the spec's P2 verification section.
- **Ruling contents:** rulings 2, 3 and 4 as the report states them. From the diff: the middle Max case, the Min threshold (MIN_IN_SET), and "-1 to 1".
- **The record stems and the date turning:** gate.log ran at 2026-09-30T00:06:59Z, and the superseded run at 2026-09-29.

I could not find none of them. Two wording points:
1. "pinned at the committed blob c98838a0..." calls that value a blob. It is the sha256 of the file's content as `git show <commit>:path | sha256sum` gives it (gate.log), not a git blob id.
2. Report item 4, "Check 1 also found:", lists four items but leaves out the rest of check 1's open findings. The missing ones are the duplicate C3 lines in negative.log, the untested past-the-grid `saturating_add`, the untested DataBar Min/Max placement, the plant gaps and the `cf_place` assumption. Read as it stands, the report suggests those four are everything. It should list them all or say "among others". The report also does not say that the tint test covers only 1.5 (Q1 #5) or that the scan-pre-check superseded path differs (Q2 #2).
