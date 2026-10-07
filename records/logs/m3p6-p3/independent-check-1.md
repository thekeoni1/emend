# M3 part 6 P3, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1/inputs (the committed spec specs/0028, planning's working copy of it as it stood then, specs/0001, the phase diff, each of the phase's four commits' diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, the stderr files negative.stderr, run.stderr and scan-pre-check.stderr, logged.out, the records the run wrote, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p6-p3. Checked commit (the phase commit): 6a90674936f3ed15ebd0d46e0b0d4b27f9132244; HEAD when the check ran: 6a90674936f3ed15ebd0d46e0b0d4b27f9132244. Its Q3 item 5 is right: the draft report said gate's and test's standard error went to logged.out, which was wrong; gate.log and test.log name <phase>/out/gate.stderr and <phase>/out/test.stderr, both 0 bytes, and the report to planning says so. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 901bf7452f5e08c7b214f52e3c7ab8b5476a2e9bea2e2d5169086ff6a1e157c1. Nothing in the copy below is changed.

Inputs and their sha256:

- ce15f3f3dac34e21fd4ced0be75920e5f7d8d7897adfc222e57272dc481cf4a4 commit-1-2814a10.diff
- 6482a3e44b2c82c511d93d913b3c676210e3cc186f0a2149b8ddc0aaa6defac1 commit-2-b68e584.diff
- 996751926b987cb199a59bb8c45c54b665c09fe8896e6224674a560e7e8f2f6e commit-3-4a8482c.diff
- 6d9b2fdbdd58cd53d0f686676c15a6068a8d87b5eb2445adbe06c370d9a4b61e commit-4-6a90674.diff
- 16d4b20407d8fe607e31098e46e3f9ffbd75edc5375966c13359ba784ac160e0 commit-messages.txt
- 757c51d79e2ef1f69830c0e4933d16a4759507ab47d12134aec6b134db9ec21c gate.log
- 7e41fc024d8426db9295cd84ec3e765c1517670ba66e9ac42922f54ebea2ed10 logged.out
- 5901a2cb745b2519a2244811ce92d01922497fb733c764fda4502042a6717d51 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- c269e94ab78843895a7d365c0d9a95b2f2ebd44dfa95c3d51108e18ffeeab4f8 phase.diff
- c8f062f1240d77f002752d628914762aec9c1b58e26643ce0761de20dad26b3d records/m3p6-open-2026-10-07-6a90674.md
- bd02e14287e85e0d01b5824e927aef97b0fb436eba1a8fd65bb01245af2825da records/m3p6-open-2026-10-07-6a90674.txt
- cf458457ed1008836aa20f22dea637d0203105fa789853d9ff67ba586c2cf385 records/m3p6-p3-2026-10-07-6a90674.md
- a60b3b8fab336c5bb29414b8070c12d3a887b8bdb88e699417d5a42064763b4e records/m3p6-p3-2026-10-07-6a90674.txt
- 7acbe60e59189b32d69353e1512c454b94bdcd9566e750ee6396e14215aabf69 report-draft.md
- cc788bb8c5cbb13c6ca81029ae8f74384956d630053d72d2ed232bf200c2d726 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- 68d14cfa3a178bc95d409f8a57c955980f36f26259e5b1aeea2a178b25316c66 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 38ab45c5800db4aaf0f1dba9dcb9ea5f726fa48a31c5d47ea273f46ea8ee9772 spec-0028-committed.md
- 5d4e611b6d944e5dc07f78e83863ac6c5a667c813359d7a698dbdc13ac5978ad spec-0028-working.md
- 20c59007a59bef6171a309db65ecbd2cefdd94b74907d639524fb3612d1891f9 test.log

## The answer

Q1. Does the diff do what the spec (with the P3 rulings) says for P3, and nothing else?

Verdict: yes, with reservations.

Note: the harness put CLAUDE.md into my context. I did not use it as evidence. I read only the files under the inputs directory. I did not read specs/0027, so I could not check the earlier parts' rules that the spec says specs/0027 "Phases" lists. I did not run git or cargo. My own sha256sum of the input files matches every hash the logs and the report give: both specs, the four logs and the four records.

Diffs read for each "nothing else":
- commit-1-2814a10.diff (doc commit): ROADMAP.md changes one line, the P2 row going from PENDING to DONE. specs/0028 has additions only (phase.diff lines 1634-1854: 210 added lines, the sections "Planning rulings during P2" and "Planning verification and rulings on P2"). spec-0028-committed.md hashes to 38ab45c5..., which matches the commit message and gate.log ("the doc commit carries exactly the two files planning's message hashed (change B35): as-expected").
- commit-2-b68e584.diff (carry): three comment lines move from above the drawing-drop block to above the legacyDrawing block. Nothing else changes. This is P2 ruling 12. gate.log agrees: "code commit 1 changes comment lines alone in package.rs, and some (change C35): as-expected".
- commit-3-4a8482c.diff (crate):
  - Cargo.toml adds `copy = []` with no dependency, so it is off by default.
  - package.rs adds `copy_sheet` and its call-time checks: "no sheet of that name", "a sheet copy needs a worksheet", and the add_sheet name rules.
  - `record_copies` gives these refusals at the write:
    - a source this write adds: NO_SUCH_PART;
    - a source this write edits, including through replace_part (ruling 3): COPY_OF_EDITED;
    - an edit on the copy: NO_SUCH_PART (decision 7);
    - without `comments`, a source with comments: COPY_NEEDS_COMMENTS;
    - with `comments`, threaded comments: THREADED_COMMENTS_HELD.
  - `record_copies` also carries HYPERLINK relationships only, with their ids. With `comments` on, `carry_comments` copies the comments part byte for byte and adds a VML part with each source box (DEFAULT_BOX when there is none).
  - VML idmap blocks are counted in `copy_idmaps` and included in `vml_idmaps` (ruling 4).
  - The streaming pass `copy_worksheet`:
    - drops `codeName`, `tabSelected`, the relationship attributes of `pageSetup`, and the nine COPY_DROPPED root children;
    - drops a root AlternateContent whole when every element in its branches is a dropped element; otherwise keeps it and refuses any relationship attribute inside it (ruling 2);
    - accepts a relationship id on `hyperlink` only when it is one of the carried ids, and refuses every other relationship attribute (ruling 5);
    - creates legacyDrawing in schema position;
    - sets `formula` on an `f` element of the main namespace, which feeds `changed_formula` and so the chain drop.
  - Four constants are catalogued in the same commit. COMMENTS, HYPERLINK and MARKUP_COMPATIBILITY are now compiled under `copy` as well (ruling 7).
  - tests/copy.rs holds 14 tests: 9 plain, 4 under `comments` and 1 under not `comments`. That matches ruling 11 T3. Between them they cover every case the "P3" paragraph lists, ruling 3's removal case included.
- commit-4-6a90674.diff (tool): the `copy` feature is turned on and `--copy-sample` is added. S is the first worksheet with a comments relationship, else W. The name is "emend copy". The sample prints the 13 named counts, asserts what "Harness runs" asks for, and removes the output when the read-back fails. Two tool tests are added: a sample over comments, plain and formula sources, and the chartsheet-only refusal (ruling 6).

Reservations:

(a) In copy_worksheet, the start tags of `sheetPr` and `sheetView` take the `without_attribute` branch and never reach `check_copy_relationships`. A relationship attribute on either would be kept without a refusal. "The copy" says "the pass checks every start tag kept". The schema puts no r: attribute on those elements, so the effect is probably nil, but the code does not check what the spec's wording says it checks.

(b) For `pageSetup` the pass removes every relationship attribute (the "every relationships attribute of pageSetup" path). "The copy" says "the r:id attribute". This is a small widening. The commit message and the doc comment both state it, but planning has not ruled on it.

(c) The tool sets `formulas` from `Package::cells` (`cell.formula.is_some()`), not from a scan for an `f` element. run.log's "the figures the survey's: 27" shows the two agree on the corpus, but they are not the same definition.

I found nothing outside the P3 scope in any of the four diffs.

Q2. Do the logs contain every line the spec requires for this phase's gate, test, negative and run logs?

Verdict: yes, with reservations.

Gate (gate.log):
- Ruling 8 pruning comes first, with the names and sizes listed and removed by `rm -r` without -f (lines 23-55): "names of the prune set there: 12 of 12; their bytes in all: 25428651432", "every name of the prune set is gone: as-expected", "every kept name is still there: as-expected".
- The spec pin is 38ab45c5 and the working copy 5d4e611b (lines 59-65).
- Commits by role, the base resolved from git, and each message ASCII ending with Co-Authored-By only.
- The doc blobs match planning's hashes. Diffs are hashed. There is a gate-file listing with no self-test.
- Each code commit stays in its own area, and the carry is comment lines only.
- check.sh shows nine PASS lines.
- clippy and doc are clean with the features off, copy, comments, copy and comments, and every feature.
- Crate tests pass with off/cm/on/pair/all on both targets: 513/557/523/570/915 on each.
- The consumer check fails on `copy_sheet` with the features off and with comments alone, and passes with copy.
- The script wrote no stderr. The last line is "gate log checks all as expected: yes".

Test (test.log):
- "host executed 1109 tests; i686-unknown-linux-musl executed 1109 tests" (line 6037).
- Ruling 11 T2 feature sets: off, copy, comments and copy plus comments on both targets (lines 5301-5406), and every feature on host and i686 (lines 5408-5433).
- Ruling 11 T1 (line 6007): "exactly the three plants L1 to L3 ... failed at one shared line ... and every other plant at a line of its own ...: yes".
- Ruling 10:
  - line 6010: "the plants' builds went into their own subdirectory, <target>/plant (ruling 10, change LM): yes";
  - line 6023: "copy tests passed after the plants: 13 (before them, in the run with copy and comments: 13); copy sample tests passed: 2";
  - line 55: SERIALIZED.
- Ruling 11 T3 counts: 13 with every feature, of 14 in the file, 1 gated off (line 5432).
- The last line is "test log checks all as expected: yes".

Negative (negative.log), ruling 6:
- The chartsheet-only fixture NC exits 2 with "error: the workbook has no sheet of kind Worksheet" and leaves no output (lines 261-279 of the checks).
- The control B0 holds a worksheet and exits 0.
- The four layer messages are stated unreachable, each with its covering test.
- The comments-feature test runs without `comments` into <target>/negative.
- Two usage shapes are checked. The last line is "negative log checks all as expected: yes".

Run (run.log):
- `--copy-sample` runs on host and i686 (lines 4127-4433), with 15 planted-flag controls per case on a plain case and a carried case.
- Ruling 1's rels expectation is built into the comparison, with controls ("entries_copy accepts the copy's rels part added where S has a hyperlink relationship (ruling 1): yes").
- The calcChain rule holds (9 workbooks: 5 dropped, 4 identical).
- The record pair m3p6-p3 is installed. The three earlier samples are byte-identical to P2's runs, which the log names by record commit d69e39cb, on both labels.
- TMPDIR and the corpus directory are on one device (2096). The keep is regenerated with host equal to i686. The edit open check runs, and P2's pairs are unchanged.
- The last line is "run log checks all as expected: yes".

Reservations:

(a) scan.log is not among the inputs, only scan-pre-check.log ("RESULT: PASS"). The final scan the spec requires as the last log is still to come, so I could not check it.

(b) Without specs/0027 I cannot confirm that every carried rule from the earlier parts appears.

(c) The report says gate's and test's stderr "went to logged.out". gate.log and test.log each say their own stderr went to <phase>/out/gate.stderr and <phase>/out/test.stderr, 0 bytes each. Neither of those files is among the inputs.

I saw no spreadsheet cell contents in the parts of the logs I read. Survey and run lines show workbook ids and counts only, and the pre-check scan reports 0 hits.

Q3. Does every number and every quoted string in report-draft.md appear in a primary?

Verdict: no. Most do, but some do not.

These appear in primaries:
- All commit hashes (gate.log lines 67-75).
- The spec hashes 5d4e611b... and 38ab45c5....
- The four log sha256s, which match my sha256sum of the files.
- The four record sha256s (run.log install lines, and my sha256sum).
- The prune quotes (gate.log line 40 and the as-expected line).
- "RESULT: PASS" and PLANNING_SPEC (scan-pre-check.log lines 8 and the last line).
- The 1109 line (test.log line 6037).
- The copy-tests line and the plants line, which are substrings of lines 6023 and 6006.
- The L1-L3 quote (line 6007, with ellipses).
- The <target>/plant quote (line 6010).
- "the workbook has no sheet of kind Worksheet" (negative.log line 1820).
- The four run directories (run.log line 75).
- "summary rows: 27; outcome ok: 27", the entry totals line and the ok-workbooks line (copy host and i686).
- The XF quote for each label of the three reruns, and P2's directories 2026-10-07-d3c4a9c-b-*.
- "9 of the corpus's 27 workbooks hold xl/calcChain.xml".
- "pairs 1092; files 2184", the byte-for-byte keep line, "(1074 + 0 = 1074): yes".
- The open totals "pairs 1092; lo_in ok 1080 fail 12 error 0; lo ok 1081 fail 11 error 0" (run.log lines 6190 and 6319).
- The four step lines and the git status in logged.out.
- negative.stderr and run.stderr being empty (0 bytes).

These do not appear in any primary:
1. "amended from 134265b". The hash 134265b is in no log, diff, record or spec. P3 ruling 8 notes the amend without giving a hash. "the tree unchanged" is close to ruling 8's "the tree is unchanged".
2. "package::tests::copy::a_copy_carries_the_comments_with_their_boxes failed in a dry run". Ruling 10 says "one copy test failed 5 of 5 by hand and in the gate's dry run" and does not name the test.
3. The exact hand clean "cargo clean -p emend -p emend-copy -p xlsx-diff (debug, release, i686)". Ruling 10 says only "`cargo clean -p` of the three crates by hand". The crate names and "(debug, release, i686)" are not in a primary.
4. "Gate dry attempts 1 to 3 are superseded, kept under <phase>/dry-gate/attempt-1..3; attempt 4 and every dry run of test, negative and run passed before the logged runs". No primary records these attempts, their count, their location or any hash. The logs' root lines say superseded attempts go to <phase>/superseded, not to <phase>/dry-gate/.
5. "gate's and test's standard error went to logged.out". No primary supports this, and gate.log and test.log state <phase>/out/gate.stderr and <phase>/out/test.stderr (0 bytes).
6. "Nothing pushed". No primary records this. The inputs cannot show it either way.
7. "first logged attempt each ... Not rerun". This is only indirectly supported: every log header shows RERUN_NOTE unset.

Items 1 to 4 are about the session's own history, not about the logged runs. Planning may accept them as disclosure, but they cannot be checked against the inputs as written.