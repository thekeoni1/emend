# M3 part 1 P3, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec, specs/0001, the phase diff, the doc commit's and the code commit's diffs, the commit messages, gate.log, test.log, size.log attempt 2, run.log, the scan's pre-check, each one's stderr, size.log attempt 1, P2's run.log, the phase's records including the decision record as first written, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p1-p3. Checked commit (the phase commit): 6578925ae14c9811c1bb3e064b7423c4926e0bad; HEAD when the check ran: 6578925ae14c9811c1bb3e064b7423c4926e0bad. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 0220f9c0e32bbe0382a439d6bf2b1bb8bf986e13d95b20471cea0c9c2d82bca4. Its items on the decision record were corrected in it before the final scan (items 1, 2, 5 and 6: line 1051, the 7860 against ruling 6's 7864, the two sheets' cells, the labelled readings and the table's provenance); its items on the report are corrected in the report; its item on the catalogue's cells sentence is reported to planning, since that sentence is in the phase commit. No second check was run.

Inputs and their sha256:

- 96ef18b9a3e2bb5eb34ed0fcb7159cd13a4c52240a6010b7b9cf393a612aa9d4 code-commit-1.diff
- b62f738ab8333ab8370c898839481c889cbb8741171fc6fc5a62b562f7acafb0 commit-messages.txt
- aada31c834a77638f5ce4a63b9b8836af1011cdab6d3be3f78d521ff73b258ea doc-commit.diff
- 7272ade49cb94a7b11ab2e010babea3f444bbb4dcb13efdca35860a7719aa42d gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- fabe1f70cdf6074c72e5d5254f0c2ebd62e664376ad6b803908ed5f1b0ebc63d p2-run.log
- df0e766b4de383db1111b8a6da7d771c14e3b04d24f689b66b04e68e3fb97877 phase.diff
- 9e114b338ccee0801e938f3356fcdd80ffa5bc26064ba303005052186faeb697 records/m3p1-open-2026-10-02-6578925.md
- 5c0483f3ab8559d97e380b5cb90f1cc76f6a1b93d0ab468eeaf27ccccc83c37d records/m3p1-open-2026-10-02-6578925.txt
- 3b8f024463e0414cf63751fa6c888fe3738297c97514566b86f5359cd066380d records/m3p1-p3-comment-2026-10-02-6578925.md
- 824a5d8d5e7aaa0d8e00733df449f38d296ee22fb37b31675618d2d401c73909 records/m3p1-p3-comment-2026-10-02-6578925.txt
- d70cb17a78e87814f5c0a9d5b736563976366caae454f175bb0c5f9ac9fc94fe records/m3p1-p3-image-2026-10-02-6578925.md
- 9caa7708e69c0f4e641de372ae0bf3f570e768a33800106461d334e0d6987a11 records/m3p1-p3-image-2026-10-02-6578925.txt
- f0c37635ef808803105f28411828027396ac2f5bd77ff3704a56a6699a5a60c6 records/m3p1-scale-c-cells-2026-10-02-6578925.md
- f278baebb3fde7afdb4571e9b95fd30e7398489b5c2ca12624c055e368d433f3 records/m3p1-scale-c-cells-2026-10-02-6578925.txt
- e6c8f01385becd0129eedd73d69fe51f8cf2c398808f1724a16bd498c2c1ed73 records/m3p1-scale-d-cells-2026-10-02-6578925.md
- fb7250618bec1ab23b3e46bdd1d90d241bf8fb299838881f96c04e94012a268e records/m3p1-scale-d-cells-2026-10-02-6578925.txt
- 1353cbfa868481745d15dd4c15ebdca5d17efe3c5cc357264f75f3c03d696dbe records/m3p1-scale-d-shared-strings-2026-10-02-6578925.md
- f5b4cf3d714904904dd0daf6baff84ca3e244a789d0a7c07686d540d9331551d records/m3p1-scale-d-shared-strings-2026-10-02-6578925.txt
- c389195acb4a4c820e5840bdfda01f0e42c82377e0414e6051e938302ce12612 records/scale-m3p1-2026-10-02-6578925.md
- f9f2bd5f17120abe9d53632ea15dce22323b684419178fae4567781170ada262 records/size-m3p1-2026-10-02-6578925.md
- 6e48e0dd6cf7a2604b8b97f95fc064fa87f8cd04c6b347821b142ebb55053a42 report-draft.md
- a1bb5d9dcea90c0366f4de3410d2bf743a95efb23cc148e1e8b3b8097b91a660 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- ea17fda48f844daa3da70cb79dffd3e068749b7a58451bf5542b25bea8f8ef5c scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- a1acab6b0acc801d17d3ab0863cb1cd7ffb21a7582d8525f143d160533dd6ee1 size.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 size.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 4c46d46c07a0d5385ed55519a185936e9b2a02471c3fd9cc969acda780fe774c spec-committed-0023.md
- d2edf804fd549d3f1e27954d816eaf88ea0c757099ff86172eae9a1710b5caa1 superseded/size-attempt-1.log
- c0df84763b5904a99afedb98b9e00394de432d67c2be82dec2fb436d0609db5d test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

---

I was given the repository CLAUDE.md, a memory index and a git status snapshot in my prompt. I did not use any of them as evidence. I worked only from <K> and from read-only git in <repo>.

The diff is right, the logs are complete, and almost every figure checks out. Six items need fixing or a note before the report goes to planning; they are listed at the end.

**Q1. YES (one PARTLY on a source citation).**

- **The diffs are the committed ones.** `phase.diff` is byte-identical to `git diff 079db61 6578925` (sha256 df0e766b...). `doc-commit.diff` and `code-commit-1.diff` are identical to `git show` of c9a59be and 6578925.
- **"Nothing else" in code: I read `code-commit-1.diff`.** It adds 49 lines to `crates/emend/src/package.rs`, all of them `//!` lines, and removes none. gate.log:1783 counts the same, "lines added or removed 49; of them not a //! line 0", and has a control.
- **"Nothing else" in the doc commit: I read `doc-commit.diff`.** It changes only `ROADMAP.md` (the P2 row set to DONE) and `specs/0023`: "The runs" and "32-bit" corrected, plus the "rulings during P2" and "verification and rulings on P2" sections. That matches ruling 9. The committed spec's sha256 is 4c46d46c and ROADMAP's is 79769f0d, both as the commit message says.
- **Catalogue figures checked against p2-run.log, all supported:**
  - The 2048 MiB cap.
  - 5025024826 (line 1095).
  - Five rewrite paths, "at most 7860 kB": edit-formula 7860/7752, delete-rows 7604/7500, insert-columns 7800/7716, delete-columns 7492/7720, protect 7772/7572. The maximum is 7860.
  - Raw copy "under 4 s at most 7864 kB": props 2.10/2.40 s, sheet 2.60/2.40 s, remove 3.10/2.40 s. Peaks max 7864 (remove, host).
  - The refusal string matches the tier c rows.
- **"1048576 rows by 120 columns" is not in p2-run.log.** It comes from the spec's tier table (spec lines 98 and 101), which the block also cites, so it is acceptable.
- **PARTLY: the cells-pass sentence.** "A whole pass over tier c's sheet runs within the runs' peaks below" has no support in P2's run.log, which the block names as its source; no P2 sample's peak is a measured `cells` pass. Only P3's later run.log supports it: tier c `--cells` at 7724/7432 kB. The block was committed before that run.
- **The code descriptions are correct as code readings.** `cells` uses `archive.read` and is a streaming iterator (package.rs:4126). `shared_strings` hands back an owned table (4100). `TempFile` uses `env::temp_dir` (archive.rs:1793). The amended-away commit 3bd5095 exists, has parent c9a59be and the same tree as 6578925.

**Q2. YES.**

- **Spec pin:** 4c46d46c is checked with a control in gate.log:23-27, run.log:2106-2110 and scan-pre-check.log, and printed in test.log and size.log.
- **Phase commit is HEAD:** checked with null-sha and short-sha controls in test.log:65-67, size.log:43-45, run.log:45-50 and gate.log:53.
- **Rerun notes:** size.log:47-53 names attempt 1 and hashes its log as d2edf804..., which matches `superseded/size-attempt-1.log`. It also hashes the attempt's stderr, script and record (f9f2bd5f). gate, test and run all have RERUN_NOTE unset.
- **Size log:** it ends "all as expected: yes". The size record's sha256 is f9f2bd5f, as size.log:6588 says.
- **Part 4d's two samples:**
  - Image: 27/27 per label equal by sha256, and lines and diff tables too, with a control (run.log:3131-3203).
  - Comment: 27/27 per label equal by output sha256 and line.
- **The open record:** 0 verdicts moved, the new pair `ok ok`, installed byte-identical. Its sha256s are 9e114b33 and 5c0483f3, which match the records in <K>.
- **The three read runs (run.log:4505-4641):** for each label:
  - the output is the input by sha256;
  - TMPDIR shows 0 files before and after;
  - there is one figure row per label;
  - the summary reads "leftovers ... 0 run(s)".
- **The names stderr print (run.log:4643-4660):** two controls run first, then 0 path lines, then each file's sha256, bytes and lines.

**Q3. PARTLY.**

- **The per-sample table, all rows recomputed:** I compared every cell of the 24 x 4-tier table with a script against p2-run.log 3663-3869, 192 values, and 0 differ. Ten I checked by hand: style a 7912/7400, props a 7628/8004, names b 176476/173944, validate d 203316/200824, cf d 203236/200796, names c crash 1078036 / panic 1075732, scale c error 7804/7996, remove c 7864/7736, edit-formula c 7860/7752, image b 7988/7552. All match.
- **Recomputed figures that hold:**
  - The 7400 to 8004 range: min style i686 tier a, max props i686 tier a, across the ok streaming runs of all tiers.
  - 2.5 to 3 times the text: the text is 90196090 / 4800000 = 18.79 bytes per item; 46.2 / 18.79 = 2.46 and 56.8 / 18.79 = 3.02.
  - 46 million: 2147483648 / 46.2 = 46.5 million.
  - The 1.8 to 3.7 MB/s range: the i686 rewrites are 1.8 to 2.9 and the host's are 2.8 to 3.7 (protect).
  - The 7/7 grid split: the 14 out-of-range rows of tier c, and ruling 5 names edit, style, font, border, date, formula and link at the write. The other seven, at the recording call, are the complement; no primary in <K> states that directly.
  - The size figures: 128 / 0 / 128 (size.log:6510-6511), 435352, 23400, 458752, and the catalogue's 0 (5219).
  - The read-run table, times (102.53, 164.53), bytes= 90196090, the names quotes, 955 bytes / 2 files, out sizes 4985617758 to 5026073402, the throughput table, and xlsx-diff sha256 at p2-run.log:1047.

**What does not agree or is wrong:**

1. **Wrong line number:** `records/scale-m3p1-...md` finding 6 says "line 1052's prose there says otherwise". In the committed p2-run.log (sha256 fabe1f70, the same as HEAD's) that prose is at **line 1051**; line 1052 is blank. Ruling 3 makes the same mistake and the record copied it.
2. **The record contradicts ruling 6 without saying so:** ruling 6 says the five rewrites' "largest peak 7864 kB". The record and catalogue say 7860, which is correct; 7864 is remove, a raw copy. The record and the report should note this.
3. **Report overstates the comment comparison:** the report says "every ok output, line and diff table matches ... on both labels". The comment comparison in run.log (3204-3336) checks outputs and lines only, with no diff tables. Only the image run compares diff tables.
4. **Report overstates the rerun hashing:** the report says "Attempt 1 and its build directory are kept and hashed on the rerun lines". The rerun lines hash four files: log, stderr, script and record. The build directory is only described as "kept with it" in RERUN_NOTE, not hashed.
5. **Cell count spans two sheets:** the decision record says "Tier c's --cells streams 125829220 cells of a 5025024826-byte entry". The tool line reads `sheets=2 cells=125829220`, so the count covers both sheets; 1048576 x 120 = 125829120.
6. **Claims without a primary that should be labelled:**
   - The amend story (3bd5095, "the scan fork's dry run found") has no log behind it; git confirms only the commit and the identical tree.
   - "generated from P2's run.log": the values match, but no log shows the table being generated.
   - "On tier d both operations hold the shared strings table" and "`--cells` adds nothing measurable" are inferences from 266288 vs 266384 / 216572 vs 216572 kB and should be marked as the code's reading.

Both texts are ASCII-only with no U+2014. scan.log is not in <K> (the report says it is written last).