# M4 part 2 P1 independent check 1

Checked commit (the phase commit): d41c3906d423dc00ec9155c195b57dcca705ef86
HEAD when the check ran: d41c3906d423dc00ec9155c195b57dcca705ef86
Checker: a fresh general-purpose subagent (not a fork), given only the files below in <check>, where <check> stands for <home>/.cache/emend-build/m4p2-p1/check1.
Inputs (sha256, name):
  3043c951a00cefb7eaf604983fc4b0bae297ed8b23fba78e8944bf01ab6d2579  diff-10685ce.diff
  5199207fbfef35d46de7de7dc41b16a807a5b529fda2cd88fb6b6463109797f5  diff-139139a.diff
  6837e9d5634751624da8214f294e4d639e7dbb204b9ce3d0aabbb712aed47c50  diff-d41c390.diff
  bfbeebf8e1edbf6e225f564e81235443c559058642161ba22402c6b17b96f23a  diff-phase.diff
  226de5b03cc175bf6af4b4996fb0e975b6e0f0e72b1209bf098dfb1332868a69  gate.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.stderr
  37229999dcccf4d263235f73d98749b921f841422007b093e6f04a9fc8bc1dec  negative.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.stderr
  7909fe67b70db7fe85496c95e02cfd93fc8e74c0207be9aa49d0216baca4d81d  report-draft.md
  9ca5bf7fa5a70eed00efc436bb1a1830033f88fdff1533b43644b44e0fa94945  run.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  scan-pre-check.stderr
  59d4ac67572736ee48ad5e28c660896c29dda3d9abb678a1f1ea2cc164f9f6d9  scan-pre.log
  18cab49d4c6906a7a2b37ecddcc88da03b372410832cdf0484f7e1247bb4c44a  spec-0031-at-28729bb.md
  f00ff7314200e66b4d3c437d60af79e84b4c0832b7d349c974e347d705c2fde6  spec-0031-working.md
  88a72d93a4dfd064f11581354bf877c24f85ff91d9c7d09741fc6f147ec2de73  specs-0001.md
  6550e4b99bbb0c5fd948a13de1b6c2acac115d0ca01f82dcd41beec7eb4e90c5  test.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.stderr
Answer: 8686 bytes, sha256 52c7739799bf3ec243bbe3f64b2bd6ea86f781a25736a8726c031571b1d5951c; plain ASCII, no machine path, committed verbatim.

---

**Note on inputs:** CLAUDE.md was in my context from the harness. I did not use it as evidence. I read only the files in check1/. I ran no git, cargo or script. I used ls, sed, grep, awk, diff and sha256sum on the input files to read them and to hash the logs.

## 1. Does the diff do what the spec says for P1, and nothing else?

Yes, as far as these files can show it.

**Which diffs I read for each "nothing else"**
- diff-139139a.diff adds CHANGELOG.md and nothing else.
- diff-10685ce.diff changes crates/emend/src/lib.rs only, one hunk. Every added or removed line starts with `//!`.
- diff-d41c390.diff changes crates/emend/Cargo.toml (one line added) and adds crates/emend/LICENSE-MIT and crates/emend/LICENSE-APACHE.
- diff-phase.diff holds those three changes with the same index hashes (8ed05e8; bbc3f34..261a649; a21f807..416c9db; d645695; 0174a49). Its only other files are the doc commit's ROADMAP.md, specs/0030 and specs/0031. Its specs/0031 is byte-equal to spec-0031-at-28729bb.md: I extracted it and compared it with diff.

**CHANGELOG, checked item by item (diff-139139a.diff)**
- **Headings:** one "# Changelog" (line 7) and one "## 0.1.0 (unreleased)" (line 9). Confirmed.
- **Subsections:** Added, Changed, Measured, Not supported by design, in that order. Confirmed.
- **Added:** 18 bullets in spec order, 0008 to 0028 without 0014, 0015 and 0023 (ruling 4b).
  - Each cites its spec's full path in backticks (ruling 4a).
  - The comments-and-images bullet cites specs/0022 and adds the "grown by specs/0028-images-and-comments-second-half.md (removal, anchors, the comment box)" text (ruling 4c).
  - The grouping bullet names `rowcol` (ruling 4b). Copy cites specs/0028.
  - All 11 manifest features are named in backticks. Tables is named `tables`, which the manifest has.
- **Changed:** six bullets, each citing specs/0030.
  - The five renames come in specs/0030 order, and bullet 3 names the crate root's re-export.
  - Bullet 6 reads word for word as ruling 5 requires.
- **Measured:**
  - Five bullets (ruling 2). Joined, they are byte-equal to the README's five paragraphs: the run.log sha256 fb12cb9f... is the same for both.
  - The paragraph sizes are 466, 166, 194, 203 and 412 bytes, matching ruling 1.
  - The README text itself is not printed in any log, so the equality rests on the hash comparison.
- **Not supported by design:** one bullet with lower-case "not" (ruling 5), checked as a part of the README's second paragraph.
- **80 columns:** no diff line is longer than 81 characters including the "+", which I checked myself. run.log reports the longest line as 80.
- **ASCII:** I found no non-ASCII byte. run.log also reports 0 U+2014 and 0 control bytes.

**lib.rs, against decision 3**
- Doc lines only: confirmed by eye and by p1_doc_lines_only.
- Paragraphs 2 and 3 equal the README's first two (run.log section 3).
- Paragraph 4 opens with the dependency sentence.
- There is one line per feature: 11 lines, the manifest's set.
- No added doc line holds a digit.
- The harness and u64 paragraphs are unchanged; their sha256 is the same at the base and at the phase commit.

**Manifest and licences, against decision 4 and ruling 3**
- `readme = "../../README.md"` sits directly after `publish = false`.
- No `license-file` line was added.
- Both licence copies are cmp-equal to the root files (exit 0). MIT is ab1c75b3..., Apache is cfc7749b...

## 2. Do the logs contain every line the spec requires, and follow specs-0001?

Yes.

**gate.log**
- The prune block lists every name in each directory, `du -sb` per name and `rm -r` per name (no -f), and checks each name gone and each kept name present.
- The m4p1-p3 directory: dry-gate, gate, test and target, 6053758313 bytes in all. I re-added the four figures and they sum to that.
- The m4p1-close directory: target, 1779553675 bytes.
- The nine PASS lines are at lines 4527 to 4535.
- The gate files are listed against the diff (lines 3988 to 4019).

**test.log**
- 514, 526, 558, 573 and 918 on host and i686, each read by grep from m4p1-p3/gate.log at the phase commit and compared.
- Doc-tests 0 passed, 0 failed.

**run.log**
- Both citation checks are printed with their programs.
- readme_check.py is pinned to m3p7-p4's hash f87526a3...
- changelog_check.py differs from it only in the two additions: the specs/ paths and the `## [0-9]` heading exemption.
- The summary lines are present for both files.
- The Measured bullets are compared under ruling 1, with a 784-to-785 control.
- The cmp lines are present.

**negative.log**
- N1: a planted uncited sentence, refused.
- N2: a missing specs/ path, refused. N3: a missing records/ path, refused.
- N4 and N5: the README check refuses a specs/-only citation and a release heading, while the CHANGELOG check accepts both.
- N6 to N8 are refused, each with its control.

**scan-pre.log**
- RESULT: PASS, with 0 structural failures.
- Change EQ is in the change list (line 1413) with both of ruling 6's controls (lines 2838 to 2842).

**specs-0001 rules I could check**
- Every log compares the phase commit with HEAD, with null, fabricated and short-sha controls.
- Every log prints its script.
- The placeholders are declared; the scan finds 0 undeclared.
- No absolute machine path appears except fragments split up to build controls (gate.log line 2950; scan-pre.log lines 1883, 1935, 1936, 1967).
- Every `rm -rf` is in the `"${var:?}"` form.
- Every stderr file is 0 bytes.

## 3. Does every number, hash and string in report-draft.md appear in a primary?

**Matches**
- All commit ids and subjects.
- The sha256 values 18cab49d, f00ff731, 07886b16 and 9f1fd354. "f00ff731 printed by every log" is true.
- Every log's line count and sha256. I recomputed the sha256s myself.
- The prune names and byte totals.
- The citation summaries.
- The licence hashes.
- N0 to N8.
- gate.log line 3851.
- The scan's 3234 lines, PASS and 0 structural failures.

**Not found in any primary**
- Departure 2: the three dry-run defects, the placeholder fix, and `<phase>/scripts/superseded-run/` and `superseded-scan/`. The report also says these files are "kept with their hashes" but gives no hash.
- Departure 4: the temporary file the script-writing fork wrote and removed.
- The path `<phase>/scan/scan-pre.log`.
- The scan-pre.log sha256 59d4ac67... is correct, but no log prints it (I computed it).
- The spec's "a dry run of each precedes its logged run" has no primary at all.

**Imprecise**
- Departure 3 says each log has "ANTHROPIC_API_KEY set in this shell: no". scan-pre.log does not have that line; it prints "input: ANTHROPIC_API_KEY is not set".
- scan-pre.log also has a ": no" line of its own (line 3019, 28729bb's diff-alone sentence), which the report does not mention.

## Defects and doubts, most serious first

1. **report-draft.md lines 23 and 25 (Departures 2 and 4):** the claims are unsupported by any primary, and the superseded scripts' hashes are not stated. specs-0001 asks for every superseded attempt to be hashed. Give the hashes in the report.
2. **lib.rs (diff-10685ce.diff line 33):** "Everything heavier is a Cargo feature, off by default, with no dependency of its own" is not README text. No log checks it, and no input shows the manifest's dependency tables, so it is an unverified factual claim.
3. **run.log lines 57, 2395 and 2396:** the status lines say "planning's two files" and "planning's uncommitted spec and ROADMAP.md". The pattern actually used (line 1280, OURS) accepts only specs/0031, and its own control (line 70) refuses ROADMAP.md. The labels are wrong; the check is stricter than they say, so the result is unaffected.
4. **CHANGELOG Added and Changed:** every bullet holds a digit only because its specs/ path does, so "with-digit 29, cited 29" is partly circular. The descriptions themselves are not checked against specs 0008 to 0028, which I could not see.
5. **CHANGELOG Size bullet (diff-139139a.diff line 95):** it cites records/size-m3p7-2026-10-09-09d2e0f.md. The spec names records/size-m4p1-2026-10-10-669fa91.md as the source of the same 436992 figure. Equality with the README forces the older citation; this is not wrong, but it is worth noting.
6. **diff-phase.diff lines 108 to 185:** the doc commit's ROADMAP.md changes go beyond "part 1's rows". It also edits the Milestone 2 and 3 headings (the fourteenth deferral) and adds the Part 2 section and table. This is planning's text, so I note it rather than call it a defect.
7. **The spec's "three logged scripts run in `<scratch>` copies":** the logs run from `<scripts>` and copy only counting.sh into `<scratch>/w`. If the spec meant the scripts themselves, that does not hold; the sentence is ambiguous.
