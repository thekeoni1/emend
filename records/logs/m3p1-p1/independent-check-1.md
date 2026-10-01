# M3 part 1 P1, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec, the working spec with planning's P1 rulings, specs/0001, the phase diff, the doc commit's and the code commit's diffs, gate.log, test.log attempt 1, negative.log attempt 3, the scan's pre-check attempt 2, each one's stderr, the record, the superseded negative attempts 1 and 2 and scan pre-check attempt 1, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p1-p1. Checked commit (the phase commit): a509ad397344d85a34d51470847624701ce01110; HEAD when the check ran: a509ad397344d85a34d51470847624701ce01110. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 7dc67a84c051121c9f0ceb4366ed76f1c1333a61d521bdf7b223a4db9826b901. Its findings were acted on before the final scan: test.log attempt 2, the record's corrections, the report's corrections, and scan pre-check attempt 3; independent check 2 reviewed the result.

Inputs and their sha256 (the draft report as the checker read it):

- 3d4327cc25c4bd94b25623cf56e0729eae298b593794269ef556638111f9fe54 code-commit-1.diff
- 10a4d67d02bde0f463a077b4d19976a7ce47afd6e449a4217d95bd8213dbd572 doc-commit.diff
- df72dad8f67a1e4236ddd2c635d090fae76fd26296db334e1c592630e6e02ceb gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- 13cc1de3ea51b3676b997b8b7e495b3da299ea0126e8395aab800ec9bc5b8553 m3p1-gen-2026-10-01-a509ad3.md
- 52444b363028243d0b9a08feb578ddc69e88f65d157223b14656e57af825055a negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- b148ba4eb19b1f380df286b9bedf2eaf6b4e27ab24df07e8e9e9a96a340e1352 phase.diff
- ab68e415a8c928fafeb332cc6038c3a66feab9deaf2b760c69108bcdf17df137 report-draft.md
- c738614c5a1551998757ed351e5c853c8289d76d47f84863de66deeeaa5c8f77 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 7b1eb997d68cd3c6ca04bbda66cc55acc10656f9ac549fdd281a3e6bf746f7ef spec-committed-0023.md
- 9a7bcee74b9b45f5314f7a6bd690ee3eafd8dd265e16b91d91ac0be4cc78e860 spec-working-0023.md
- 966647484f7791630220b9ca910d2ac9f34f842334658c91a79016d1b15605d4 superseded/negative-attempt-1/m3p1-p1-negative.sh
- 277a15269adb8a50e05f47b9084b7d787124257724e1132592601bb016c402c9 superseded/negative-attempt-1/negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 superseded/negative-attempt-1/negative.stderr
- 26ce897b168b1459c6a0b4e72e7f2f31f389f6d8abb9b7d4669935f1860d4141 superseded/negative-attempt-2/m3p1-p1-negative.sh
- c3fc1aa87790298c4cd958de71831eafd693041d05e4e2119229b84105ab3e84 superseded/negative-attempt-2/negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 superseded/negative-attempt-2/negative.stderr
- a40f5bd212aa95e019ae79a00e2d2b8faa4743cd2366f2d534da50ea7163c251 superseded/scan-pre-check-1/scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 superseded/scan-pre-check-1/scan-pre-check.stderr
- 7332279e4ca705c7a8a686fa5e6ababbb586eeeea3529fad37547450ce3e47b6 test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

---

Context note: the harness put the project CLAUDE.md, an auto-memory index, a git-status snapshot and a user email into my prompt. I did not use any of it as evidence. I used read-only git in <repo> only to confirm commit ids, parents and blob hashes.

**Q1. PARTLY: the diff is right, but one report sentence about it is wrong.**
- **The diffs are the real commits.** phase.diff, doc-commit.diff and code-commit-1.diff are byte-equal to `git diff` ab5b3aa..a509ad3, ab5b3aa..48c6e0d and 48c6e0d..a509ad3. Parents are a509ad3 -> 48c6e0d -> ab5b3aa.
- **Doc commit, "nothing else" read from doc-commit.diff and gate.log 1594-1632.** It holds exactly ROADMAP.md, specs/0022 and specs/0023. Their sha256s are ed534cce, bfec1e8f and 7b1eb997, and spec-committed-0023.md hashes to 7b1eb997. The git blob ids are different (specs/0023 is ee577dcc, ROADMAP is ae7494c3). So when the report says "Its three blobs: ... 7b1eb997", it means content sha256s, not blob ids. That is minor wording.
- **Code commit, "nothing else" read from code-commit-1.diff and phase.diff.** It adds only scripts/scale-gen.py and scripts/scale-gen-test.py. There is no crate, tool or Cargo change (gate.log "paths: 2; outside ...: 0").
- **The generator matches the spec and rulings 1 to 4:**
  - It uses the standard library only, a fixed seed and streamed rows.
  - The block is 12 numbers, 6 strings, 3 formulas, 2 dates, 1 boolean. Tier c has 5 blocks and 1048576 rows; tier t has 100 rows.
  - The second sheet is 10 by 10. It adds a date xf (numFmtId 14) and no calc chain.
  - Core properties use "corpus". Tags are `scale,tier<letter>` and source is `generated`.
  - It refuses a target under the repo, an unknown tier, and a tier already present without --force.
  - The test asserts 2 sheets, 2500 cells, 1000 strings and byte-identical output on both labels.
- **Beyond the spec, but defensive:** four extra refusals (bad manifest header, partial file, missing template anchor, file already in corpus), the manifest app column "none", and zip64 forced on tier c's sheet1. The 1000-word vocabulary is generated from the seed, not written out literally. I read that as "held in the script".
- **Disagreement (report-draft.md, Commits bullet 2):** "That first form created corpus/ before two of its refusals" is not what `git diff 79419fc a509ad3` shows. In 79419fc, `os.makedirs(corpus)` ran before `read_manifest` (the header refusal, R9), the manifest-row refusal (R6), the partial-file refusal (R8) and `parts()` (the anchor refusal, R11). That is at least three of the logged cases, R8, R9 and R11, which is also the set negative attempt 2 tightened. The record's wording, "a refusal could leave an empty corpus/", is safe.

**Q2. PARTLY: everything the spec requires is printed, except one rerun naming.**
- **Spec pin:** gate.log 21-27, test.log 23-29, negative.log 69-71 and scan-pre-check.log 7 all pin 7b1eb997. The working copy hashes to 9a7bcee7 and is printed only.
- **Phase commit compared with HEAD, with controls:** printed in all four logs (null sha and short sha). The gate's controls use the same command form ("count=1 expected=1").
- **Tier t test through the gate's emend-copy on both targets:** test.log 4171-4229. Both binaries' sha256s are matched to gate.log, every line is ok, and the log ends PASS. Plants G1 to G3 are at 4231-4272, and the workspace runs are 893 and 893 tests.
- **Refusals in the negative log:** negative.log 772-880 covers R1 to R11, all 8 source messages are reached, and it runs from a worktree.
- **Each tier:** test.log 4274-4337 prints the sha256, byte count, generator seconds, GNU time wall and peak RSS. It also prints the zip64 extra field, the over-2^32 check for tier c with its 32-bit truncation control, and the manifest rows.
- **Negative rerun note:** negative.log 37-45 hashes attempts 1 and 2. The hashes match the superseded/ files (277a1526, 9666474, c3fc1aa8, 26ce897b, e3b0c442).
- **Gap: the scan pre-check rerun is not named by hash.** superseded/scan-pre-check-1 ended "RESULT: FAIL" with 2 structural failures. scan-pre-check.log has "RERUN_NOTE is not set" and "RERUN_FILES is not set", and no log prints its sha256 (a40f5bd2...). spec-0001 lines 84-87 and 193-197 require the rerun's header to name it by sha256.
- **Stated only in the record or report, with no log printing it:**
  - The 79419fc dry run and the first generation that "made the same four sha256s" (no log names 79419fc). Only the reflog shows the amend at 20:23:51Z.
  - /tmp as tmpfs, "11G, 9.6G free" (the record admits this).
  - "167 MB" for tier d's sharedStrings.xml. It is only in the working spec as 166996261.
  - "2002424253" and "1.9 GiB" for 48 columns. They are only in working-spec ruling 1.
  - The survey's file and line numbers. No primary in <K>; I did not verify them against source.
  - The tiers were generated by the wrapper <phase>/gen-tiers.sh (20:23:54Z to 20:28:24Z), before any log ran. test.log prints the wrapper's sha256 and one line of it, not the whole script.

**Q3. PARTLY: most figures check out; a few have no log behind them and one disagrees.**
- **Match a log:**
  - The four tier ids, sha256s, byte counts, sheet bytes, seconds, peak RSS and wall times.
  - 2^32 = 4294967296.
  - 893 tests; 2 sheets, 2500 cells, 1000 strings.
  - 7b1eb997, ed534cce and bfec1e8f.
  - The negative attempt narrative: attempt 1 R11 "no"; the attempt 2 placeholder at superseded scan line 2100.
  - ulimit -v 2097152.
- **Match the record's own tables:** the counts 20 streams / 5 loads, 15/7/1/1/1, six past row 1048576, and 8 refused.
- **No log primary:**
  - 79419fc and the first generation's "same four sha256s".
  - "1.9 GiB" and 2002424253.
  - "about 4.7 GiB plus about 1.1 GiB" and "11G, 9.6G free". These are a reading of the code plus df, not logged.
  - "167 MB".
  - The survey's line numbers.
- **Disagree or imprecise:**
  1. "two of its refusals" (report). See Q1.
  2. "under 45 MB" (record, The tiers). 44108 kB is 45.17 MB decimal; it is only under 45 if MiB is meant.
  3. "Attempt 2 ... which the scan pre-check refused" leaves something out. The superseded pre-check also failed "the spec in the working tree hashes to PLANNING_SPEC ...: no", because PLANNING_SPEC was given as a path, not a hash. The report never says the pre-check itself was rerun.

**Other things that look wrong:**
- test.log line 4278 prints the wrapper's target as `<repo>-testing/scale`. That is a scrub artifact: the <repo> prefix was replaced inside a sibling path. The header on line 4274 says <home>/emend-testing/scale.
- negative.log still labels its R8, R9 and R11 checks "(attempt 2)" in attempt 3. This is cosmetic.
- The record's "names on tier C ... UNSURE: depends on tier C's archive size" was left open, though the size (1159829654 bytes, over 1 GiB) was known when the record was written.
- No negative case checks that a sibling directory sharing the repo's name prefix is accepted. It is not required; the real generation into <home>/emend-testing shows it in effect.
- The record and logs in <K> equal the working files: the record is 13cc1de3 and was scanned at that hash, and gate, test and negative match records/logs/m3p1-p1.