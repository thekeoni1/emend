# M3 part 2 P2, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec and the working spec with planning's rulings during P2, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, each one's stderr, the phase's four records, the superseded attempts including those of the phase commit before its rebuild, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p2-p2. Checked commit (the phase commit): dc33002692911f3e7bde37f7952a904d0c5d4be0; HEAD when the check ran: dc33002692911f3e7bde37f7952a904d0c5d4be0. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to d33bd560df49694d91eface79ff9a0cfa479f8f591bfa958ae6e34b5a5d2750f. Its items on the report are corrected in the report; its observations on the code and the logs are reported to planning. No second check was run.

Inputs and their sha256:

- 7c574ff7da98f181cfbe6b49bd5d6c81f4cd5379db3b93afec84934b7f5a8a43 code-commit-1-ruling4.diff
- 1b9f63442ba49e9817aaa14022cccb41581f6435aad986976db679c0393d833f code-commit-2-crate.diff
- 957fde3db4b81c644d27830f99cec60a6cd965ba6158be5cc75783e48869cf9d code-commit-3-tool.diff
- 4d0c6d4813ee521f5b44ca25de17958a87759203d80c6552ef2dde31edea70c3 commit-messages.txt
- e05de71ecd9ab3439e74645d36ea2bf8a6a72986706dd0850aa21bac962b8f7d doc-commit.diff
- 5b088c100f6fbf87e3bed262860f43a5ecdb4b8b0a2a65a7162f549d11c963a1 gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- e56f86592f4ac46e117ef92eadcc0f5619da3422f634d6677b95f1b9a49b6cbd negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- a160f7d4f1c46526c7530def2957de3346340fd3b4689662908098e29552a48e phase.diff
- 12659d1ad7aae714e8aca84bb384fe5fa735ff80e6807e86c15dd85b598048a5 records/m3p2-open-2026-10-03-dc33002.md
- e40b0b3d468acb6f670060fcad14ff88b78158e00ac8155433127a8b3f9a8cbe records/m3p2-open-2026-10-03-dc33002.txt
- cc6142968e5c8cadb26d0c3bb85a7bfb19401ec2321a40e479934e1da0641e5a records/m3p2-p2-2026-10-03-dc33002.md
- 476989e23de69ba4ac18a89673245b08185d5226dca6d6263e4f1e31cc81325c records/m3p2-p2-2026-10-03-dc33002.txt
- 6a42479e4f6637f43ee0acf954bbaaeff9f0aa327d27aa1808cde9e09352d846 report-draft.md
- a09c5badefb999bc57b94133ed4522699ec2599c2d3f0b29c517c121aba88696 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- 535aba0b5ac3fa53b4bda004d6efe11715e84e1b7578e7f699991d44e852d903 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 23c6111c2608438c0a5fe2d7459742d198ea2b914e5791a5134fb304cf7a831e spec-0024-committed.md
- d6137d521f8b72e967fd43adfbc627865d630de57d01b7d32da4949ff01d0604 spec-0024-working-rulings.md
- f21228fb89a6aad5cf414dbc857bbd757285621088e22467e90bbc25dd3e477f superseded/at-4f29957/gate.log
- f74764c2ebc7ecae394f711c7a2cab3df597efe9c4bb8503cab43654f722079e superseded/at-4f29957/negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 superseded/at-4f29957/out/gate.stderr
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 superseded/at-4f29957/out/negative.stderr
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 superseded/at-4f29957/out/run.stderr
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 superseded/at-4f29957/out/test.stderr
- c8264503b885810b8857017a5080672d8e30669a64277ae86d843e47883c80f0 superseded/at-4f29957/records/m3p2-open-2026-10-03-4f29957.md
- 258b289d32c331e4adf96ab9a22dffa1d99f292086920fba4d4eb60179eecf21 superseded/at-4f29957/records/m3p2-open-2026-10-03-4f29957.txt
- e72066d4499d447e65275cdb0d88f0616afcb002b55143295e9556c4aad077e2 superseded/at-4f29957/records/m3p2-p2-2026-10-03-4f29957.md
- 1368370f8cce725c0ba236d7b0346e1942158e6dbce7bc26c54653f9f1386e54 superseded/at-4f29957/records/m3p2-p2-2026-10-03-4f29957.txt
- d243238a37c27d4bb757c84998ee19475c053ad5ec9b2491a096bb07fdd34c6e superseded/at-4f29957/run.log
- 81cc75db559fbe91a91bb2548837a9f1815c8b03904fff398e1eb941ffb62e8b superseded/at-4f29957/test.log
- f10b9a63ea0ebf539db98a51dfbd88d631ce20e4d44212bee87f60f7eddf3134 superseded/gate-attempt1.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 superseded/gate-attempt1.stderr
- 234899d392367363cf81fd7223639655918df696516f2424827ff8f325d7db5e superseded/m3p2-p2-gate.sh.attempt1
- 7dc201b9143d129dd4b46ad137cfb2c80ddb57c46c111e7be25407f2a031294d superseded/m3p2-p2-test.sh.attempt1
- db874dc81b05eb7b5f1c22a653f0b5be661702f703c3f08ba7cc0767a6db0e0f superseded/test-attempt1.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 superseded/test-attempt1.stderr
- 73d8c7848a5b8c9b4e3116752d33a20cf471d9c2cdb1bac4f09919e4a49207b8 test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

---

Note on context: the harness put a CLAUDE.md and an auto-memory index (MEMORY.md) into my context. I did not use either as evidence. I read only files under `<phase>/check1`. I ran no cargo, changed nothing, and wrote no file. My own measurements were sha256sum over the files there, greps, and a per-file comparison of the diffs.

## Question 1: does the diff do what the spec says for P2, and nothing else?

**Diff files against gate.log.** I hashed the five diff files. All five match the hashes gate.log prints at lines 1946-1958:
- phase.diff: a160f7d4
- doc-commit.diff: e05de71e
- code-commit-1-ruling4.diff: 7c574ff7
- code-commit-2-crate.diff: 1b9f6344
- code-commit-3-tool.diff: 957fde3d

**phase.diff against the commit diffs.** For each of the 9 files, the phase.diff section is byte-identical to the one commit diff that touches it. The exception is cells.rs, whose changed lines in phase.diff are exactly the union of code commits 1 and 2. So phase.diff holds nothing the four commit diffs do not.

**doc-commit.diff (planning's).** It changes only ROADMAP.md (the P1 row) and specs/0024 (the corrected facts, "Planning rulings during P1", "Planning verification and rulings on P1"). gate.log 1916-1944 checks both blobs against the hashes in planning's message. Nothing else: confirmed from doc-commit.diff.

**code-commit-1-ruling4.diff (P1 verification ruling 4).**
- `Collect::close_run` drops a run with empty text, and its font with it (lines 75-84).
- `rich()` returns None when no run is left (lines 66-68).
- The rustdoc of `runs_of` and `Cell::runs` is updated.
- One test, `an_empty_run_is_dropped_at_the_read`, covers both containers (lines 104-142).

The code also drops an empty `<t>` after the runs (line 92). Ruling 4 names only `<r/>` and a run whose `<t>` texts total empty; ruling 5 said a `<t>` after the runs reads as a run of its own. Dropping the empty one follows ruling 4's principle but is the implementing session's extension, which planning has not ruled on. Nothing else: confirmed from this diff.

**code-commit-2-crate.diff (Comments, P2 tests).**
- `Package::add_rich_comment` with the spec's signature (lines 108-117). Its checks are at lines 145-164: grid, forbidden character, author 1 to 255, no run / empty run / more than 32767 flattened characters, then `font.check()`.
- `CommentEntry::runs` under `richtext` (lines 53-61).
- The read in `read_comments` (lines 252-344) uses the shared `Collect` and `<rPr>` reader, skips `<rPh>` runs, and keeps `text` unchanged.
- The write gives one `<r>` per run, the `<rPr>` via `Font::composed("rPr", "rFont")` (line 199).

Every test the spec lists for P2 is present among the 9 tests in comments/rich.rs. The only gaps are the colour refusal at the call (report finding 7) and the "threaded comment's legacy fallback reads as its runs say" sentence, which has no test. That sentence is not in the P2 test list.

Beyond the spec:
- **(a)** The plain-comment write path is refactored into `comment_t` (lines 193-232). That is not required. The only evidence it is byte-preserving is indirect: P1's 646 keep pairs are unchanged (run.log 3925-3950), and only two base tests are edited (gate.log 2439-2512).
- **(b)** `Runs`, `Collect` and `Collect::rich` become pub(crate) (lines 14, 23, 32).
- **(c)** A behaviour change for existing callers. With comments and richtext both on, `Package::comments` now refuses a kept comment whose `<rPr>` it cannot read. The tool turns every feature on, so `--comment-sample` (P1's sample) would now refuse such a workbook too. This is within the spec's "Errors" line, and the corpus has no such comment (run.log 2266 survey check yes; 27 of 27 ok). Report finding 6 covers `Package::comments` but not the effect on the existing sample.

Otherwise nothing else: confirmed from this diff.

**code-commit-3-tool.diff ("Harness runs", `--richcomment-sample`).**
- The sample (lines 121-192) prints the fifteen fields the spec lists, in its order, and asserts +1 rich comments, 2 runs and the text (lines 229-243). A failure after the write removes the output (lines 167-170).
- Two tool tests (lines 327-430).
- The `comment_counts_after` refactor (lines 279-299) is declared in the commit message.

Small departures:
- Count failures in the rich sample are reported as `Error::CommentSample` ("...the comment implies") rather than `RichcommentSample`. Cosmetic.
- The rustdoc (line 23) lists a threaded-comments refusal that no P2 log exercises.

Nothing else: confirmed from this diff.

**`rm -rf` (P2 ruling 1).** Every new form in the printed scripts uses `${var:?}`:
- gate.log 542 and 600
- test.log 458 and 531
- scan-pre-check.log 732

The `rm -rf "$d"` at negative.log 974, gate.log 1812, test.log 1747 and run.log 1996 is the committed counting.sh, which stands under the ruling. scan-pre-check 2511-2535 classifies them: unruled 0.

**Forbidden shapes.** My own grep found 0 of each of the three forbidden forms in gate, test, negative, run and scan-pre-check, which agrees with scan-pre-check 2496-2509.

## Question 2: do the logs contain every required line, and do their checks support "yes"?

**Present as required.**
- test.log: the 12 added tests are listed and pass on both targets (3825-3916). Crate runs with features off, richtext, comments, and the pair, on both targets (3917-3978). Workspace runs with every feature, detected at 3727-3776. Nine plants, each passing unplanted and failing planted (4351-4480).
- negative.log: both 4d fixtures refused, N1 and N3, with controls N2 and N0. Also N4 (last row 1048576, control N4C) and N5 (unreadable `<rPr>`, control N5C). No output file for any refusal. Unreachable messages mapped to their test (1006-1353).
- run.log: every row ok (3418, 3530). The survey checks the val and colour forms with 12 planted controls (2266-2280). The survey takes `<corpus>`. `flagged` and `out_only` are checked against derived expectations (3478-3479, 3590-3591). The comparison has 14 flags, each with a plant that turns it (3608-3640). Host and i686 outputs are identical (3641-3645). The record's eighteen checks pass (3798). The keep is regenerated on both targets and compared with P1's record named by c53bbb3 (3808-4009). The edit open check and its acceptance pass (4010-4870).
- Every log ends "yes", and all five stderr files are 0 bytes.

**Weaknesses.**
1. **run.log 2284.** "distinct sequences over both kinds (the spec's fact names 17): 17" is printed, and no check reads it. The report's "The spec's 17 distinct sequences holds" rests on that unchecked figure.
2. **run.log 3609-3636.** All 14 planted-flag controls are applied to d8245b84086f, a created=2 workbook. No plant runs on any of the 5 appended (created=0) workbooks. So on the corpus there is no shown bite for the appended-path checks (comments_bytes, vml_bytes and rels_bytes on kept parts). Crate and tool tests do cover that path on fixtures.
3. **test.log 4441-4462.** Plants W7 and W8 both fail at rich.rs:406 with the same "refused: ()". The output cannot tell which case failed. The plants are found once and the controls pass, so they do bite.
4. **test.log runs with EMEND_KEEP_DIR unset throughout (line 6),** and gate.log never regenerates the keep. Only run.log can catch a keep-only failure, which is how finding 2 escaped three logs.
5. **superseded/at-4f29957/run.log 3801-3821** prints only "1 failed" per target. No line names the failing test or its panic. The identification exists only in authored RERUN_NOTE prose (for example run.log line 8).
6. **test.log 3746 and 3770** print the workspace comments module at 43 tests, while the crate pair run gives 42 (3942). The difference is not explained and no check reads it.
7. **negative.log 1279-1284** checks N1 twice, once against N2 and once against N0. Redundant, harmless.
8. **run.log 293 (change JI)** still says no `rm -rf` anywhere, while line 1996 runs counting.sh's. KI at line 374 qualifies it. This is the same stale-prose pattern P1 verification ruling 3 criticised.
9. **run.log 2340-2349.** The FINDING lines are deliberately not checks.
10. **scan-pre-check.log** is a pre-check: it scans no check answer (2509), so the final scan is still owed.

## Question 3: does every number, hash and quoted string in report-draft.md appear in a primary?

**Found and correct.** Each of these appears in a primary, and the file hashes match my own sha256 of the files in check1:
- Spec hashes: 23c6111c and d6137d52.
- Commit ids: fc24238, 8a268f2, 17d1e6b, dc33002, c53bbb3; also e1e9601 and 4f29957 (superseded/at-4f29957/gate.log 34-35).
- Log hashes: 5b088c10, 73d8c784, e56f8659, a09c5bad (scan-pre-check 2314-2354).
- Record hashes: cc614296, 476989e2, 12659d1a, e40b0b3d (run.log 3803-3805 and 4864-4867).
- test.log counts: 513, 532, 546, 574; modules 19 and 42; comments::rich 9; workspace 926 passed, 0 failed, 2 ignored.
- run.log counts: 27 rows; identical 800, changed 70, added 50, flagged 40; out_only 321; 27 of 27 identical outputs; eighteen checks; keep 1328 files and 664 pairs; 18 new pairs, 0 differ; open check 589 and 75 pairs, 0 refused outputs.
- The 6/5 count, ed2fbd304d51, and "7 runs in 6 comments" (run.log 2337-2349).
- I22 and QJ (gate.log 61 and 504; test.log 55 and 440).

**Not found, or inaccurate.**
1. **535aba0b (the scan pre-check's hash).** It matches the file, but no primary prints it (a scan cannot hash itself).
2. **"The commit messages are unchanged" (line 12) is contradicted.** The message line counts are:
   - e1e9601: 29 lines (superseded/at-4f29957/gate.log 1888) -> 17d1e6b: 30 lines (gate.log 1890)
   - 4f29957: 21 lines (superseded/at-4f29957/gate.log 1891) -> dc33002: 22 lines (gate.log 1893)

   commit-messages.txt lines 22 and 53 show a trailing blank line after the Co-Authored-By line that 8a268f2 (line 69) and fc24238 (line 79) do not have. RERUN_NOTE's "the same messages" has the same problem.
3. **"The only change is one line" (line 12) is not supported and probably wrong.** No primary shows the e1e9601-to-17d1e6b diff. comments/rich.rs grew from 15877 bytes (superseded/at-4f29957/gate.log 1963) to 15932 (gate.log 1965), +55. Replacing a `Package::open(...)` call with the shorter `open_fixture(output)` would shrink the file. The current test also has two comment lines just above the call (code-commit-2-crate.diff 921-922). Probably three lines changed, not one.
4. **Finding 2's test name and cause** appear only in authored prose. The superseded run.log prints no failing test name (see Question 2, item 5).
5. **Finding 3's "the forks' dry runs showed the same lines and their summaries did not report them"** has no primary.
6. **Finding 7: "The negative fork noted"** has no primary for the fork. The substance does: check_color returns FIELD_OUT_OF_RANGE (negative.log 1061-1066). "The colour path is tested for the cell write in P1" has no primary in this directory.
7. **Finding 6: what the catalogue sentence says** cannot be checked, because the catalogue is not in any diff.
8. **Finding 8: "inside a struct gated by comments".** The struct's cfg line is not visible in the diff. gate.log 2633-2671 supports the behaviour.
9. **"Not pinned" (line 3) is inaccurate** for the scan. scan-pre-check takes PLANNING_SPEC=d6137d52 (line 8) and asserts it at line 2452. negative.log 59 and run.log 2035 only print it.
10. **"The spec's 17 distinct sequences holds"** rests on an unchecked figure (Question 2, item 1).
11. **Minor wording.**
    - "Nothing is committed beyond the three code commits" leaves out the doc commit fc24238, which is also in the range.
    - "item 2 of its table" in finding 5 is zero-based: it means `runs_of(2)`, the third `<si>`.
