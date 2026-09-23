# Independent check 2: M1 part 6 P3

Written by the implementing session. The header is mine; everything after
the rule is the checker's answer. The answer was returned as a message
whose display escaped the characters < and > as HTML entities; this file
writes them back as < and >, and writes the answer's 8 ellipsis
characters (U+2026) as three full stops, so the file is ASCII as the scan
requires; it is otherwise the answer as returned, transcribed by me.

The checker was a fresh subagent (not a fork) with no access to this
conversation. It was given the 19 files below and nothing else, told not
to build or run tests, and asked the three questions CLAUDE.md names. It
was told to write <scratch> in place of the scratch directory. This is the
phase's second and last check; no log was rerun after it.

Commit checked: e0a7b951e493d3bac74fa01e1fb4c96199158ec6
("M1 part 6 P3: probe-emend-formulas-write-xml"),
which was HEAD when the check ran and when this file was written. Range
read: a29c3f9..e0a7b95. Each per-commit diff's producing command is named
beside it below (P1 verification ruling 2).

The inputs, each as the checker received it (sha256, then the name it had in
the check directory and what it is):

- 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
  0001-bootstrap.md (specs/0001-bootstrap.md at e0a7b95)
- c10c98d18eeb76f621e221577925745b971ea91f44bdb115669c9670afa8c59a
  0013-formulas.md (specs/0013-formulas.md at e0a7b95 (committed in 92dd877))
- 3ca14d6b21cb2e92f09bebfb36ddadbcb591d23f123ef6512003b81782062db9
  diff-61a7d53.patch (made by: git show --format=fuller 61a7d53)
- a6b0efbc1584904bd48dd51735cb5af038312e108540f99703a23d76d1199fa6
  diff-92dd877.patch (made by: git show --format=fuller 92dd877)
- e6f9dbfc1a999ccb5ece7e9ad1146e06dc6e736ca4d624066f06be28f264485b
  diff-code-combined.patch (made by: git diff 92dd877 e0a7b95)
- 4e49d5d07216677a8976b52f89a05d04455a0dd164958fb360ca50e654d88360
  diff-commands.txt (the command that made each diff)
- d27b26c9c868cf0f8b064cac77e8e30cc6263b13cf1099533f97c6ac2785e3a1
  diff-e0a7b95.patch (made by: git show --format=fuller e0a7b95)
- c57981450f486cbee0e7a09f73ad35d659a2a0a4d9f32b2cf2f576868cd2208a
  diff-phase.patch (made by: git diff a29c3f9 e0a7b95)
- 32aaeb4a939be98542e321f6fbaf9ec450f702b8b51210524adcca0a07000506
  gate.log (records/logs/m1p6-p3/gate.log (attempt 2))
- 9a9a6d7387617637e7b8b97353b460f525c58a4808e1e62e0817bebcf68134c4
  independent-check-1.md (records/logs/m1p6-p3/independent-check-1.md)
- 8de6b3012d6d48455898f34b85b0700cd9ac87f95a46eec4cea89966ba2c7699
  m1p6-p3-2026-09-23-e0a7b95.md (records/m1p6-p3-2026-09-23-e0a7b95.md)
- ba3839be1f3cb17b7c8864a235ee6cdb622dd7b574997ec19239fb3dc9f64746
  m1p6-p3-2026-09-23-e0a7b95.txt (records/m1p6-p3-2026-09-23-e0a7b95.txt)
- 040b818cb555042f689d4b5d26622650cc31b128f3ea8cb208f1fbca7919a15c
  negative.log (records/logs/m1p6-p3/negative.log (attempt 5))
- d587e68f48eb68f6bd35f26893e8f86dab19082650dcc14b37d597694f4f70b0
  report-draft.md (the draft report (draft 2))
- 732391888f5ae6dccea7b3c047627da2be5bcb61c35ddca8689fcf8bee5b51bc
  run.log (records/logs/m1p6-p3/run.log)
- 525f6a7c3928c28539a5a3aba7ff3431928b502c05e201c5733daf34a354c7f9
  scan-for-checkers.log (the scan run before this check, after negative attempt 5, RESULT PASS)
- 7e37457a4501a2c973f3cd0870a4256e3a297446bbd307c99943a90dfec5fe2c
  size-m1p6-2026-09-23-e0a7b95.md (records/size-m1p6-2026-09-23-e0a7b95.md)
- d1dee5143d58afa74123a7b6fe3b308fb4f197c92b4cd5a591c7c9bc7aa2c9ec
  size.log (records/logs/m1p6-p3/size.log)
- 79542e1c63694276bbd4ed901e80b6c9ec2782d697f597c2d6929dfd9897075f
  test.log (records/logs/m1p6-p3/test.log)

The answer as transcribed, before the ellipses were written as full stops:
9877 bytes, sha256 e0e08a2b8f2c2b795d4927e6e69fd86db4872aafb9413f4d071853943ff801ef;
as it stands below: 9877 bytes, sha256
abbfc5b07d9d0fca7e62e5ee1bbcf6def751f425362e614a2c3c59d86d518871.

---

# Independent check 2: M1 part 6 P3

I read only the files in `<scratch>`. I built nothing and ran no tests. Every input's sha256 and byte count matches the table at report-draft.md:25-32. The per-commit patches have the same hashes that independent-check-1.md:31-42 lists.

## 1. Does the diff do what the spec's P3 says, and nothing else?

**Yes.** Two points still need planning's word.

- G1 (scope). diff-phase.patch:1,14,40,105,235,270 touches six files. They are ROADMAP.md and the spec (doc commit 92dd877), cells.rs and package.rs (C1, comments only: test.log:3441-3446 counts 0 non-comment lines), and the size-probe manifest plus the new probe (C2). gate.log:1166-1173 lists the same six. Nothing else is touched.
- G2 (the doc commit; still open, report item 7). 0013-formulas.md:367 says "P3's first **code** commit carries the P2 verification rulings". Ruling 7 at :692-694 says the "first doc commit carries this spec". 92dd877 follows ruling 7, and gate.log:1125-1152 shows both files at the handoff hashes.
- G3 (the Limits cover what was asked). The caller's picture, "never evaluated" and Excel pending are at diff-61a7d53.patch:76-91. The calcPr rewrite with the foreign child (P1 F1) is at :92-101. The two refusal points (P1 verification ruling 4) are at :102-108. Two sheetData (P2 ruling 3(a)) is at :109-111. The cells Limits are at :35-46. All of it matches 0013-formulas.md:121-179 and P1 rulings 1, 2 and 5.
- G4 (a claim beyond the ruling, low). "A reader counts cells in both" (diff-61a7d53.patch:110) is not in ruling 3(a). The report (report-draft.md:110-113) cites P2's committed negative.log, which is not in this set. The same string does appear in P3's negative.log:4687 and :4692 (H13 on the `twosd` fixture), so a primary here supports it.
- G5 (the catalogue, informational). P3 adds no catalogue entries: the five messages date from P1. test.log:3453-3471 checks that the Limits' "five" agrees with the catalogue. The three formula-text messages are raised in cells.rs (negative.log:5418-5428) but catalogued only under package.rs. C1 adds no cells-side catalogue entry. The spec's "catalogue ... in package's and cells' module headers" (0013:369-370) could be read as asking for one.
- G6 (the probe). diff-e0a7b95.patch:267-303 matches "Size" (0013:208-219). As check 1 Q1 F7 found, `drop_calc_chain()` at :254 means the rule's own chain drop never runs on its unrecorded branch. The probe's comment at :63-66 says "so the write's formula-change rule runs", which is true only in that sense.
- G7 (a size point the report underplays, low). size.log:2638 says "the drift of the probes that link emend is part 6's", and all 5 commits are part 6's (:2640-2644). So part 6's real cost to the budget is 8400 section bytes. That exceeds planning's "under 6000 for the part" (0013:221-222). 968 is part 6's cost only under the spec's own definition. Report item 1 (report-draft.md:79-82) states the facts but frames the 7432 as drift.

## 2. Do the logs contain every line the spec requires?

**Yes**, with the low defects below.

- G8 (lines 1-3). In all five logs, line 1 is `command: git rev-parse HEAD`, line 2 is `invocation:` and line 3 is `about:` naming the phase commit and HEAD.
- G9 (SERIALIZED lines). All five carry the cargo and rustc process counts: gate.log:41, test.log:35, size.log:39, run.log:44, negative.log:41.
- G10 (second roots). Each log gives one line per root. gate.log:42, size.log:40 and test.log:36-37 spell a `/tmp` mktemp directory as `<scratch>/w` or `<scratch>/mut`, which P2 verification ruling 5 permits. negative.log:43 now uses a real `<scratch>/w` made by mkdir (script :104-105). run.log:46 uses a real `<scratch>/p3/w.bwqorB`. **Low:** `<scratch>/w` therefore means a real scratch subdirectory in negative.log and a `/tmp` directory in gate, test, size and scan-for-checkers.log:42. Also, negative.log's script comment (:116) still says "Two roots" for what is now one directory inside `<scratch>`.
- G11 (build directory spelled once). Every log spells it `<scratch>/p3/target`. A grep of all five logs, script text excluded, finds no bare `target/` path. negative.log:3431-3434 now print `<scratch>/p3/target/...`.
- G12 (no typed commit id). The phase commit comes in by argument and is checked against HEAD. Roles and base are resolved by subject (gate.log:17-35, 1072-1081). Two exceptions:
  - size.log:158 still types `PREV_REC=records/size-m1p5-2026-09-22-f565868.md` (check 1 Q2 F5; left as it stands).
  - **New:** each `rerun:` line comes from a `RERUN_NOTE` environment variable (gate.log:234-235, the same in test and negative). The invocation lines do not show it. The text is typed prose: gate.log:38 types "61a7d53", and negative.log:35 types the claims about each attempt's outcome. The log's printed invocation cannot reproduce these lines.
- G13 (P2's superseded run directories). run.log:71-103 prints each `rm -rf --` with present/absent before and after, and prints that `-71e643b-c` still stands. The two names match 0013:676-677 exactly. As check 1 Q2 F6 found, the set comes from a typed `-b` suffix, not from a listing of the runs directory.
- G14 (per-commit diff hashes beside their commands). gate.log:1154-1164 prints each hash beside its command. My sha256 of diff-phase.patch equals gate.log:1162 (c5798145...). The check directory's commands are in diff-commands.txt and report-draft.md:71-75.
- G15 (superseded attempts named by sha256). test.log:32-34 (1 attempt), gate.log:38-40 (1) and negative.log:35-40 (4) each name theirs. gate attempt 1's hash (aa299f...) and negative attempt 2's (64d286...) equal check 1's input hashes (independent-check-1.md:43,49). **Open:** the pre-run test copy of run.log's script (report item 6, out.txt cb3c96...) is named in no log header.
- G16 (commit messages). C2 says "Measured in P3's size.log, not here." (diff-e0a7b95.patch:15). C1 states only word counts, which test.log:3477-3495 accounts for. No message states an unprinted measurement.
- G17 ("Size" and P3). size.log has:
  - twelve builds (:2187-2242);
  - the 11 part 5 probes reproduced, 11 "yes" lines (:2307-2604);
  - the classification: 6 identical, 5 code (:2601-2608);
  - every figure (:2609-2658).

  The record printed at :3014-3150, with its indent stripped, is byte-identical to size-m1p6-...md. negative.log:5066-5190 covers the two-calcPr case on the host and i686 release tool (L2, L4) and the release probe (L8), with all five messages reached (:5450). run.log:2064-2098 uses the size.log-hashed binaries and builds nothing. The totals on both targets are asserted against the log's own read with a control each (:2238-2273, :2396-2432, :2554-2560).
- G18 (did the reruns after check 1 fix what they claim?). Yes:
  - Q2 F13 is fixed at gate.log:1099-1100.
  - Q2 F3 is fixed at negative.log:43, and Q2 F4 at :3431-3434.

  A side effect: gate attempt 2's `cargo clean` (gate.log:1374-1375) deleted the release binaries. The rebuild before negative attempt 5 is printed by no log; only the typed rerun prose at negative.log:35 mentions it. negative.log:5081 then prints "(nothing is rebuilt here)". The L-block hash matches (:5074-5076) show the rebuilt binaries are byte-identical, so the measurement stands. The rebuild commands themselves have no primary.

## 3. Does every number and quoted string in report-draft.md appear in a primary?

**No.** One item has no primary, and two rest on typed prose only.

- G19. The out.txt sha256 cb3c9670...1755 and "err.txt empty" (report-draft.md:103-104) appear in no primary. The report says so.
- G20. "I rebuilt the three by size.log's own printed commands" and "attempt 5 changes only its root line's wording" (report-draft.md:96-99) are supported only by negative.log:35. That line is RERUN_NOTE prose, not a measurement (see G12 and G18). The hash matches at :5074-5076 do support "all three match".
- G21. The quote "the output's sheet holds 5 cell(s) and the edits imply 4" (:112-113) is attributed to P2's committed negative.log, which is not in this set. The identical string is in P3's negative.log:4692.
- G22 (these match their primaries):

| Report (report-draft.md) | Primary |
| --- | --- |
| Commit ids and roles (:3-5) | gate.log:1083-1091 |
| All eight sha256 values and byte counts (:25-32) | the files themselves; run.log:2610-2611; negative.log:5068 |
| Nine PASS; ASCII; handoff hashes (:34-36) | gate.log:1109-1152, 1377-1391 |
| 492/0/2 on both; P2 492; no test added (:37-38) | test.log:1935, 2473, 2503, 2531-2532 |
| full_calc 1, chain 0, formula_read_back 1; PM1/2/4/5/6 bite; PM3 quote (:38-42) | test.log:3140-3179, 3418-3426 |
| Twelve probes; 6 identical, 5 code (:43-45) | size.log:2242, 2604-2608 |
| 312812, 80404, 393216, 315392 (:46-47) | size.log:2610-2614 |
| 968, file 0, .text 800 (:48-49) | size.log:2618-2619 |
| 304412, 8400, 7432, drift 0, 5 commits (:50-52) | size.log:2626-2651 |
| 4980 (:53) | size.log:2621 |
| 6000 yes/no; "is under 6000: no" (:54, :81) | size.log:2656-2657 |
| Two-calcPr message; all five reached (:57-61) | negative.log:5097, 5112, 5162, 5450 |
| 38/15/9/6/0/19; run directory (:65-67) | run.log:2229, 2396, 2554 |
| The three P2 directory names (:68-70) | run.log:86-103 |
| Four diff commands; phase hash (:71-75) | diff-commands.txt; gate.log:1162 |
| 4096-byte steps (:83-84) | size.log:2305 |
| Attempts and reasons (:86-92) | test.log:32; gate.log:38; negative.log:35 |
| gate attempt 2's cargo clean (:93-96) | gate.log:1374-1375 |
| Check 1 finding labels (Q1 F2/F4/F5/F7/F8; Q2 F3/F4/F5/F6/F12/F13; Q3 F2) | independent-check-1.md:78-118 |

- G23 (omission, low). The report gives no sha256 for independent-check-1.md (9a9a6d73...) or scan-for-checkers.log. It does not note that the scan (RESULT: PASS, scan-for-checkers.log:1444) ran after negative attempt 5: that log hashes negative.log as 040b818c..., the current file.
