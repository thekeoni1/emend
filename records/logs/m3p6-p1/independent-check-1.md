# M3 part 6 P1, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1/inputs (the committed spec specs/0028, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, the stderr files and the runner's summary, the phase's records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p6-p1. Checked commit (the phase commit): 2adf953f2d2379f7eafd761f01bfde544da79d78; HEAD when the check ran: 2adf953f2d2379f7eafd761f01bfde544da79d78. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to d1b8766a1ea314a6e76f1c5899e45a7e4a9f8c5e013206ea78980341e5e34d68. In the copy below one path is written as <phase> (the answer's first line names the inputs directory by its full path); nothing else is changed. Its reservations are reported to planning; no second check was run.

Inputs and their sha256:

- 775dbee50bdbbb58a5b022ccd6f543a14d485a445417a3c6c9c05b5a49464074 code-commit-1-crate.diff
- f9febd97be6e4a9743f11c9aa643637f1ebed0b5384da67423db8316eca1cac0 code-commit-2-tool.diff
- b5bfafd228be8e357972bf9926387318adfa8a762019eef45f4c533066f2f57e commit-messages.txt
- 3ba6d09c436a7b9354b7c63cf97f647288527c717f2bfa1f0f61dd8df30cb332 doc-commit.diff
- 3a45f8a59b1279d9d92bc6a177148912bc70b15de43decedd7e37fe969539e79 gate.log
- 06240d207011e7c51e3739a9e3f3c6abd2c6821cbce700d642adee8e70f3ed71 logged.out
- d29c541586d38ebbd5600c6991ab2b5452e058b3d5203912c1c99b44df0339f6 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- 34148363255a01c5c8214a5de32c0d3c5628a3e1012bbb9245e12ab93819e3d0 phase.diff
- c8306bcd7ecab9dc480be79e16653cb50d39572bbf2bbb0335237c05bf0f1b99 report-draft.md
- 4580eeb89ae0f728626019f066f08c216a22e33478101ab816efb1f8e67a629c run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- aef49c502af547a637c93df15115730e40f6b6287b51f3ce6689b963a81f9035 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 8adfe67c2360f1373be6447a1ef4f69529cfdbc16b8153806946cb8b1bc33c7b spec-0028-committed.md
- 5a136a8aa2853d699cf14a32e4a4bcd282a872a1a9d7f5493f6bb590a8f9ccf5 test.log
- f46e42a940d2345f0ee5a3040670a2ee1c43d7c4224a31857721eaaa66109eb2 records/m3p6-open-2026-10-06-2adf953.md
- ba5b680aa59e99e4a40789fead3ac652afc3a0532f7a223249918458b5fd1fb7 records/m3p6-open-2026-10-06-2adf953.txt
- 31209a2cd59d08c46529a4a7221339fc40559948ab5da65741b67a1ccb5bcb50 records/m3p6-p1-2026-10-06-2adf953.md
- 2e67511b7ac8b9628eb6ee11752980d420f3764d127348168c07f6d25d7292c5 records/m3p6-p1-2026-10-06-2adf953.txt

## The answer

**Independent check: M3 part 6 P1 (inputs in <phase>/check1/inputs/)**

Nothing was modified. I ran only read-only commands and reconstructed one file in my scratchpad.

**Q1. Does the diff do what the spec says for P1 and nothing else? YES WITH RESERVATIONS**

Evidence that it matches the spec:
- **What changed.** `phase.diff` touches 10 files: ROADMAP.md, specs/0027, specs/0028, lib.rs, package.rs, tests/comments.rs, tests/comments/boxes.rs (new), tests/comments/rich.rs, tools/emend-copy/src/main.rs and tests/null_rewrite.rs. Nothing from P2 to P4 appears: no ImageAnchor, add_image_at, remove_image or copy_sheet. The gate (gate.log:2599-2645) shows each code commit stays in its own area, with no manifest, no lock, no styles file, and `comments` still off by default.
- **Doc commit.** I rebuilt specs/0028 from doc-commit.diff. It hashes to 8adfe67c... and is byte-identical to spec-0028-committed.md. ROADMAP and 0027 match the hashes planning sent, per gate.log:2463-2485. I can only check those two by hash; their full text is not in the inputs.
- **CommentBox** (code-commit-1-crate.diff:105-115). Fields width, height and visible; derives Debug, Clone, Copy, PartialEq and Eq; under the `comments` feature.
- **set_comment_box** (diff:485-500). Refuses a width or height outside 1 to 65535 at the call with "a comment box is out of range".
- **The write** (diff:220-233). A box goes with the addition on its cell, the last one set wins, and a box with no addition is refused with "no comment addition on that cell".
- **The writer** (diff:403-433). Size is written as pt at three quarters of the pixels, exact quarters to two decimals at most. A shown box writes `visibility:visible` and `<x:Visible/>` immediately before `x:Row`. The default 144 by 79 reproduces today's bytes, and a test asserts it (boxes.rs, `an_unset_box_...`).
- **The read** (diff:135-170, 300-345). pt is times four thirds, rounded; `visible` comes from the style or an `x:Visible` child. It returns None with no VML part, no shape, or no size. A VML part it cannot read is refused with COMMENTS_UNREADABLE.
- **The two new messages** are constants in package.rs (diff:252-256) and are catalogued in the same commit (diff:55-65; gate.log "I33" check).
- **P1 test list.** Every item is present in boxes.rs:
  - default bytes; 200 by 100 shown; box set before and after the addition; set twice
  - no addition, refused; each range refusal
  - read of pt, px, no size, Visible child, and no VML part
  - round trip of the sample's box; the richtext box
  - the tool's line and its refusal (null_rewrite.rs)
- **Tool** (code-commit-2-tool.diff). The constants are author "emend", text "emend box" and the box 200 by 100 shown. It asserts +1 comment, +1 note shape, box_equal and visible_after, and prints the seven named counts.

Reservations. The implementer flagged items 1 to 4 for planning; items 5 to 7 they did not flag.
1. set_comment_box also refuses an off-grid cell at the call with part 3's index message. "Errors" names only the range message at the call (report choice 1).
2. style_pixels accepts decimal `px` and rounds it. The spec says "`px` taken as is" (diff:300-345; the doc comment says "in `px`, rounded").
3. A size outside 1 to 65535 reads None (choice 3).
4. Any `x:Visible` child reads as visible, whatever its content (choice 4).
5. `comments()` now refuses with COMMENTS_UNREADABLE when the VML relationship's target is missing from the archive or cannot be resolved (diff:141, 170). The spec says None when there is no VML part. A call that read fine before P1 can now fail on such a package.
6. With two or more VML relationships the read follows legacyDrawing (choice 2). The tool's shapes_before and shapes_after instead count the first VML relationship (`related(...VML_DRAWING)`). These can disagree only on unusual packages, and none did on this corpus.
7. In `read_vml`, `visible` is set on any shape whose ClientData has a Visible child, notes or not. It is harmless because only notes are looked up.

Corpus content: nothing in the diff prints or embeds corpus content. Fixture text ("Ann", "Bob", "one", "kept", "emend box") is fictional, the tool prints counts only, and its error strings are static.

**Q2. Do the logs hold every line the spec requires, and does each end as expected? YES**

- **gate.log**
  - Last line: "gate log checks all as expected: yes" (00:31:07Z); stderr empty.
  - check.sh shows nine PASS lines (3098-3115).
  - Crate tests, host and i686: off 513, richtext 533, comments 557, comments+richtext 588, all 886 (3253).
  - The consumer crate fails with the features off and with richtext alone, and passes with comments (3268-3317).
  - Messages are ASCII and end with the Co-Authored-By line only (2428-2455); the doc blobs match planning's hashes.
  - It reaches 3 of the 4 gate log hashes the report cites, plus the spec pin.
- **test.log**
  - Last line: "test log checks all as expected: yes".
  - 1075 on host and on i686; the previous phase had 1061, so 14 tests were added (4825, 4830-4843).
  - Feature-off and feature-on runs on both targets (4969 onward).
  - Six planted changes, N1 to N6 (5468 onward).
- **negative.log**
  - Last line: "negative log checks all as expected: yes".
  - Uses the gate's binary by hash: add816ad... (1541-1549).
  - NU, the VML part with no root end tag, gives exit 2 and exactly "a comments part or its drawing cannot be read", with no output. NUC, the same part with its end tag, is the control and gives exit 0 with its line (1665-1690).
  - The two new messages are stated unreachable, each with its covering test (1600-1603).
- **run.log**
  - Last line: "run log checks all as expected: yes".
  - Both targets (3806, 3922): 27 summary rows, 27 ok; lo ok 27 and lo_in ok 27; entry totals identical 800, recompressed 0, changed 70, dropped 0, added 50, flagged 40 (3841-3845, 3957-3961).
  - The checks are named box_style (width 150pt, height 75pt, visibility visible; 3685), visible_child (an empty Visible child right before Row; 3690) and kept_shapes (byte spans; 3698). Each holds on all 27 workbooks, and each has a planted control on a created and a kept case (4044-4110).
  - TMPDIR and corpus devices: 2096 and 2096, with /proc at 61 as the control (3797-3804, 4493-4497).
  - Calc chain: 9 of 27 workbooks, each calcChain row identical and unflagged on both labels (2744-2749, plus the per-label lines).
  - Phase directory named by record commit: bec28eb7..., M3 part 5 P3's (36, 4397).
  - Keep and edit open check: 26 new pairs, all ok on input and output; 0 earlier pairs moved (4402-4430, 5682-5715).
  - Records are installed as records/m3p6-p1-2026-10-06-2adf953.{md,txt} and m3p6-open-2026-10-06-2adf953.{md,txt}. Their hashes in run.log:4275-4277 and 5741-5745 match the files in records/.
- **scan-pre-check.log** ends "RESULT: PASS", total hits 0.
- All stderr files are 0 bytes, and RERUN_NOTE is unset in every log.

**Q3. Does every number, hash and quoted string in report-draft.md appear in a primary? YES WITH RESERVATIONS**

Everything I checked is found:
- Commit hashes 6aba171, 7cfc6f5, 69b5e0e and 2adf953.
- The three doc sha256 values.
- Log hashes 3a45f8a5, 5a136a8a, d29c5415, 4580eeb8 and aef49c50, which match `sha256sum` of the logs.
- Counts 513/533/557/588/886, 1075, 27, 800/70/50/40/0/0, lo 27 and lo_in 27, 9 of 27, and 26 new pairs.
- The "all as expected: yes" lines and RESULT: PASS.
- The ISO/IEC 29500-4:2016 citation (commit message 69b5e0e).
- The record names, and the date (runs ended 2026-10-06 UTC).

Not found in any primary:
- "privacy.sh after each code commit: 0 findings". No log records privacy.sh being run per commit. What the primaries do show:
  - check.sh's "PASS privacy" at the phase commit only (gate.log:3110).
  - The gate's 11-pattern scan of new and modified sources: 0 hits (gate.log:2683, 2690).
  - The scan pre-check: total hits 0.
- "the UTC date had passed midnight" is an inference from the log timestamps, not a primary line. It is consistent with them: the commits are 23:46 and 23:57 UTC on 10-05 and the runs are on 10-06.

Note: the report's implementing choices 1 to 4 match the code. Item 2 under Q1 (rounding of decimal `px`) is not among them.
