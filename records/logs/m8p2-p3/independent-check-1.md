# M8 part 2 P3: independent check 1

The answer of a fresh subagent given only these materials, under
<scratch>/check1:
- the spec as committed at a1a0ae0 (a5368c96...) and as working
  with rulings E to K (37a8987d...);
- ruling L, verbatim from planning's message (ruling-L-message.txt);
- the six commits of the range, a1a0ae0 to 8065795 (git show);
- gate.log, size.log and run.log;
- the pre-check scan's log;
- the size record;
- the logs README as it stood before the check (sha256 1a1830f28ac13799aeb9b5c02bf4fc36adc6db2fc9b339ab3f5e67e4e3522f87);
- the draft report.

The answer was extracted from the subagent's transcript by script into
<phase>/check1-answer.txt (sha256 786a8375e33b91333348354e530d37e3a0547efbe9f77fcfd79645e6de234cc5). It appears below verbatim, and
nothing in it is edited. The README's section "After independent check
1" answers each exception.

root: <scratch> stands for <phase>, as in gate.log, size.log and run.log
root: <phase> stands for <home>/.cache/emend-build/m8p2-p3, the phase's directory
root: <home> stands for the home directory of the account the runs ran under
root: <check> stands for <scratch>/check1, the materials directory the subagent was given

Hashes computed: gate.log 3528ea39400c77d680af1627e69026df210d1962385293163fb4926f0503dd6c; size.log f65e0cef26f397cd68835439e3ae20b6c377af7f8316d07c8b3ad5b12a219358; run.log 3b72e757e10519dbc9b0b2592e4818e436a4cd7166466bcd0838655859fec482; scan-pre-check.log 41c2321eb168d2c3b04bddd206fe39989d10f7c625786a890b0b82d6ff70d170; README.md 1a1830f28ac13799aeb9b5c02bf4fc36adc6db2fc9b339ab3f5e67e4e3522f87.

# Independent check 1, M8 part 2 P3 (attempt 2 under ruling L)

All materials are in `<check>`. I also computed these hashes:
- spec-0048-committed.md: a5368c96...2078. This matches the hash in a1a0ae0's message (commit-a1a0ae0.diff line 7) and size.log line 139.
- spec-0048-working.md: 37a8987d...d155, 933 lines. This matches PLANNING_SPEC (size.log line 5).
- size-m8p2-2026-11-10-8065795.md: b4a386c3...124d, 41413 bytes, 424 lines. This matches size.log line 14306.

## 1. Does the diff do what the spec says for P3, and nothing else?

**Yes, with one documentation defect the range leaves in place.**

- **a1a0ae0 (doc commit).** It touches two files, ROADMAP.md and specs/0048, and gives each as "<path> sha256 <hash>" (commit-a1a0ae0.diff lines 7-8). The ROADMAP change marks P2 DONE and P3 IN PROGRESS. Ruling E confirms it (spec-0048-working.md line 655).
- **ea6dc9f.** It adds one test and changes no code: the external target and the XML error, as ruling D (1) asks (commit-ea6dc9f.diff lines 23-56).
- **5ab0e96.** This is the decision 1 A refusal as "The pivot cache source" describes it (spec-0048-working.md lines 276-316):
  - `pivot_sourced`, `workbook_pivot_caches`, `cache_source` and `root_names` (commit-5ab0e96.diff lines 136-200 and 263-400);
  - the two error constants (lines 214-219);
  - the call after `reached` and before the first record (lines 205-210);
  - `pivot_remove.rs`, whose tests cover the spec's list (lines 431-650).
- **6b13c52.** It changes emend-copy alone. `removed_reason` takes `last`, `last_visible`, `pivot_source` or `-`, and the three refusals remove nothing (commit-6b13c52.diff lines 63-75 and 79-107). That is decision 3 A.
- **37e800f (the fix).** It matches ruling F and Amendment 1 (spec-0048-working.md lines 682-711 and 846-857):
  - the `sheet` case stays in the core;
  - the name cases sit under `cfg(any(tables, metadata))`, `metadata` and `tables` (commit-37e800f.diff lines 141-180);
  - the struct fields carry cfgs (lines 194-209);
  - `TABLE`'s cfg is restored exactly as 5ab0e96 removed it (37e800f lines 52-62 against 5ab0e96 lines 80-86);
  - `PIVOT_CACHE_DEFINITION` stays unconditional;
  - the table test and its helper are under `tables`, and the defined-name test is under `metadata` (lines 343-369);
  - the `remove_sheet` Errors paragraph no longer links `TABLE` (lines 66-90). That is ruling I.
  - The message names Amendment 1 and the stop "ruling E" (line 7).
- **8065795.** It adds the three CHANGELOG bullets under "## Unreleased" and leaves the 0.1.0 entry alone. The README gets the removal sentence only, and the rowcol and comments figures are deferred to P4 (commit-8065795.diff lines 36-80). That is "The documents" with ruling H.
- No commit after a1a0ae0 touches ROADMAP.md, as ruling F requires. There is no U+2014 in any diff.

**Disagreement (doc defect not caught by rulings I or K).** 5ab0e96 added this to the package module docs (commit-5ab0e96.diff lines 47-49):

```
//! ... `TABLE` and `PIVOT_CACHE_DEFINITION` are public
//! with no feature, the core's sheet removal reading them
```

37e800f puts `TABLE`'s cfg back but has no hunk at that place; its first hunk starts at `@@ -269` (commit-37e800f.diff line 39). So at 8065795 the module docs still say `TABLE` is public with no feature. That is false after the fix. Ruling I repaired only the `remove_sheet` paragraph. Ruling K (4) speaks of `PIVOT_CACHE_DEFINITION` only.

## 2. Do the logs contain every required line?

**Mostly yes. There are two gaps and one disclosed deviation.**

- **Last lines.**
  - gate.log: "gate log checks all as expected: yes" (line 12509).
  - size.log: "size log checks all as expected: yes" (line 14327).
  - run.log: "run log checks all as expected: yes" (line 3298).
  - scan-pre-check.log: "RESULT: PASS" (line 7001).
- **Rerun lines (ruling L).**
  - RERUN_NOTE is at size.log line 6 and the rerun note at line 96. RERUN_FILES is at line 7.
  - The six superseded files are hashed at lines 99-104, including attempt 1's size.log (6339d013...97b4, 749568 bytes) and the old logged.sh (dfaf3db0...c3eb).
  - Line 105: "files named 6; not there 0; outside the superseded directory 0". Lines 106 and 107 read "yes".
  - I rebuilt the size script printed at size.log lines 149-8613, with the 4-space indent removed and the final blank line dropped. It hashes to e8617847...2be7, the same as the superseded copy at line 99. So the copy is byte-identical to the script that wrote attempt 2.
  - Small inconsistency: the note (lines 6 and 96) also names the scratch directory as kept, and line 106 says "every file of the attempt RERUN_NOTE names is hashed above". The scratch directory is not hashed.
- **Remove pair.** run.log line 3264 has "remove 495c7b79b183 ok/fail/ok to ok/ok/ok, its line removed 0 and removed_reason pivot_source ... : yes". Line 3240 has earlier 39, same 38, changed 1, missing 0, new 0. Line 3237 has "- 26 last 12 pivot_source 1".
- **Budget and costs.**
  - The format probe is 989480 sections (line 10314) and the zip probe is 533216 (line 10262).
  - Budget 456264 and "2488 under the ceiling" (lines 11480-11482). The base 451512 and the growth 4752 are at line 11483. The ceiling check is at line 11463.
  - Costs: 5a charts -336 (line 11484), 5b images 168 (11485), 5b' comments 2948 with raw 7700 (11486), 5c read -352 (11487), 5d metadata 2268 with raw 7020 (11488), 5e rowcol 18628 with raw 23380 (11489), 5f tables 2380 with raw 7132 (11490).
- **FINDING lines.**
  - size.log has exactly 11 lines that begin with "FINDING:", at lines 11611-11621. The printed record repeats them at lines 14062-14072.
  - gate.log has the word only inside the printed script (lines 5769 and 5778).
  - run.log prints "FINDING lines printed above: 0" (line 3282).
  - scan-pre-check.log has the word only inside its printed script (lines 4305 and 4330).
- **README's FINDING list.**
  - It quotes size.log lines 11611-11621 byte for byte; I compared them by diff (README.md lines 154-164).
  - It covers gate.log (line 150) and run.log (line 149).
  - **Gap:** it says nothing about scan-pre-check.log. The README does not mention that log, its hash or its two FINDING-word lines. Its table (line 42) has only an empty row for the final scan.log.
  - **Gap (minor):** it does not mention that the record section of size.log repeats the 11 lines at 14062-14072.
- **Three raw-growth probes (rulings K (1) and L 4).** Each is given with the core's 4752 and the cost growth it carries (README.md lines 175-191):
  - richcomment: 4752 + 2948 = 7700 against 7556 (line 11586);
  - printnames: 4752 + 2268 = 7020 against 7656 (line 11588);
  - copycomments: 4752 + 2948 = 7700 against 7700 (line 11591).
  - The companion probes match: 4672 (line 11585), 4832 (line 11587) and 4776 (line 11590).
  - The arithmetic is correct. But the lead sentence, "Each one's growth is the core's 4752 plus a feature's cost growth" (README.md lines 176-177), is contradicted by the README's own bullets: richcomment is 144 under that sum and printnames 636 over. The draft report repeats the framing (draft-report.md line 31).
- **Disclosed deviation.** Ruling L 3 says "then the run, then the scan, by the driver's own form", and ruling K says "the logged driver in the order gate, size, run, scan". The scan did not run in the driver. A pre-check scan ran after the README instead (draft-report.md point 2, line 37). It is disclosed, but it departs from the letter of both rulings and needs planning's word.

## 3. Does every number and quoted string appear in a primary?

**Confirmed against primaries:**
- the log and record table: bytes, lines, hashes and last lines (README.md lines 39-46);
- the commit table subjects (lines 22-28 against each diff header, and gate.log line 308 for 572e71f);
- the start lines gate.log 393, size.log 95 and run.log 77;
- the root lines (gate.log 15, run.log 90);
- the prune: gate.log lines 201, 240, 241 and 242;
- the test figures: gate.log lines 11491, 11497, 11659, 11686 and 11848;
- the binary lines 11139-11142, 11170 and 11172;
- run.log lines 3232, 3237, 3240, 3264, 3270 and 3282;
- the size figures listed above, and 22 of 28 (size.log line 11604);
- the feature lines 10647, 10649 and 10652;
- 18644 (commit-a1a0ae0.diff line 25);
- the slip at size.log line 95;
- the 10710 lines (size.log line 6);
- the working spec hash and its 933 lines;
- the draft's figures for the gate, size and run hashes, the record, the budget, the costs, the remove sample, the tests and the 11 FINDINGs;
- the size step's end 17:49:50Z (size.log line 14325), and the run's start 17:49:50Z and end 17:53:17Z (run.log lines 77 and 3297).

**Imprecise line citations (the figures exist, but not on the cited lines):**
- README.md line 133 cites line 11463 for 989480 and 533216. They are at size.log lines 10314 and 10262; line 11463 has only 456264 and 458752.
- README.md lines 136-137 cite lines 11440-11441 for 451512. It is at line 11483.

**In no primary, and not marked as coming from outside the directory:**
- README.md line 64: "21:44Z" and "cut off mid-line". The log has only "about 13:44 local time".
- README.md lines 65-66: "no size record was installed; the run and the scan had not started".
- README.md lines 77-78: "tmp.IiLrnwbJe1" and "206221070 bytes by du -sb".
- README.md line 84: the logged.sh hash 42c1bad6...9dbe. It is also in draft-report.md line 14.
- README.md lines 87-88: "the build reused them without error".
- README.md lines 68-70 and draft-report.md line 11: "6 named, 0 mismatched or missing". The README points to driver-2.out; the draft gives no source.
- draft-report.md line 14: "root mount propagation was private ... the maintainer set it shared". gate.log line 19 shows "shared" for attempt 1's gate only.
- draft-report.md line 14: "Size 17:43:57Z". size.log line 95 says 17:43:58Z; the README credits 17:43:57Z to driver-2.out.
- draft-report.md line 26: "Identical to dry d2". Ruling K (spec-0048-working.md lines 778-781) confirms budget, core, rowcol, comments, tables and metadata. The charts, images and read figures for d2 are in no primary.
- README.md lines 215-216 and draft-report.md line 36: dry d2's size.log line 95 reads the same. There is no primary for this.
- README.md line 98 and draft-report.md line 39: dry d2's record name carried 2026-11-09. There is no primary for this.

**Marked as from outside the directory, so I could not check them** (README.md lines 193-209):
- the driver-2.out quotes;
- the run directory tmp.VV0GNtYFQb and the phase-tmp listings;
- attempt 1's empty driver.out. Its 0 bytes is confirmed at size.log line 103.

No home-directory path, non-ASCII character or U+2014 appears in README.md or draft-report.md.
