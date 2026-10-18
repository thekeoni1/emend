Independent check 2

Implementer's header (not the checker's): a fresh general-purpose subagent, given only the files under <phase>/check2/, answered the three CLAUDE.md questions on these logs, at the phase commit 9e171da. Its inputs:

| File | sha256 |
|---|---|
| README-repo.md | 9773fa581f7c12e933f81a55da094ab0fd8e734c00f4a0526a40922e5d954ce7 |
| gate.log | 6b80688727c3b972aa2fad1059d963f0280ac060012d2f28f54de60cb2ce226e |
| logs-README.md | 13f07159665158f313588e68afa7308ccf662ad7f55d78b5e931cc75c11339d7 |
| negative.log | 2523186bd01c3fd7fee9e9c80b2bc72170708f98f0788944189c1fe971ffb7aa |
| phase.diff | 63e4d38b422771482977a3cc0e697077a296b2e92583f6b94fd1b3940885a93c |
| records/compare-m5p6-2026-10-16-5e52758.md | 2faf8ff1e41ffec4182dc5481430350629a65a7da5f0e8629d9ce3f71cb626bc |
| records/m3p6-open-2026-10-08-9cfd92e.md | f9f564cbc230c19a14a170302665ac0d3553338d98de3c8cf120fb69c423819c |
| records/m3p6-p4-anchor-2026-10-08-9cfd92e.md | 1e06e59a73716087d30679230a528b2fc0e933c9ceb8788d072b4735f14c0acc |
| records/m3p6-p4-commentbox-2026-10-08-9cfd92e.md | 6a73d57022c232fb2eb6d120f83ae8ca771716d8ba824eb8a575bbb68549daa6 |
| records/m3p6-p4-copy-2026-10-08-9cfd92e.md | 86670a673839343219bfa83a68fd16de3d21b67e155505703aa0e77e3336311e |
| records/m3p6-p4-removeimage-2026-10-08-9cfd92e.md | f93d9a97e7268b5fbf55fa262ad2ed457ab50cddde9ef7ccdb326129f4919759 |
| records/m5p5-p4-chart3d-2026-10-15-a8bfe71.md | ec17e28364fdd6af839c5fc39b57947920a56c89e0f8d33ce24a6e8fbf7cdecc |
| records/m5p5-p4-chartlabels-2026-10-15-a8bfe71.md | 3d7dcda158f603c5b324dc67eca20662fabd0a339fd980b3884d303b36dde7ed |
| records/m5p5-p4-chartmore-2026-10-15-a8bfe71.md | c618417d4c756fc5e68898a7f8d754e5a05d270e4fc3384d2537a2841333fc22 |
| records/m5p5-p4-charts-2026-10-15-a8bfe71.md | 0c5c177ab1e031fc2480a219c2d7d21da65caf783ba7c6eef59871d7c70f5fbc |
| records/m5p5-p4-removechart-2026-10-15-a8bfe71.md | fec28063bebcf10c965fdc01d469586c4584ae2d7d3302f31e3761c2d4659e80 |
| records/scale-m3p1-2026-10-02-6578925.md | 0db5d43b5f3ed6164786adff13586a06b6f56e29789934516fe7dbfe098a9e49 |
| records/size-m5p6-2026-10-18-ef9da37.md | 8d7e65ca3b01742345e083ca77c6d59e6ba674b3878a098f54959bd388415611 |
| records/toolchain-2026-09-15.md | 5a6834de5c46c10ae9d6bde992aa736659008c67985112788be11ff14d1ea013 |
| report-draft.md | bac26c4a7699f301b9b0eb73fc17d68636dbb1f478045a0fe4eafdef98ead276 |
| run.log | b95091210393b1940858f124dc48cdf5ed8a85679d58d530a625ba491f913d05 |
| scan-pre-check.log | 7ae55b24d516a5e481ad1ee3eafc19dfd539db50c64ca3ed581ab32b4be765db |
| spec-0037-working.md | 8019cc257ec22efd399b4ef095719d36fd3e9e6df7b3c9ff07b6b34c55dff55b |
| test.log | 744bc54a209c6b282002d34c1d67ea62b78f4b64a865e0b73dd96d10353c7e42 |

The answer, verbatim (sha256 dc0190ea9868efa027bdd23fa52b084dc27040fd21fa98a0ff0683d923821e6b):

No workbook cell content appears in any input. The records, logs, diff and report give only entry names, counts, byte counts, hashes, test names and pass/fail.

All references are to files in `<phase>/check2`, with line numbers.

## Q1. Does the diff do what the spec says for P4, and nothing else?

**Scope.** phase.diff touches three files: README.md, ROADMAP.md and specs/0037-measurement-and-close.md.
- The ROADMAP edits are the ones ruling AE names (spec-0037-working.md:796-801):
  - the P3 row set to DONE (phase.diff:63-64);
  - the Milestone 6 section placed before "Unscoped" (phase.diff:68-130);
  - the Unscoped charts bullet rewritten (phase.diff:138-149).
- The spec hunk adds only the P3 rulings W to Y, the P3 verification and the rulings Z to AE (phase.diff:159-279). Rulings AF to AI stay uncommitted, which matches the working spec's hash 8019cc25 (sha256 of spec-0037-working.md recomputed: equal).
- CHANGELOG.md is unchanged (gate.log:6884).
- Each README commit touches README.md alone (gate.log:6846, 6849).
- The line counts I recomputed from phase.diff are README 18/13, ROADMAP 72/5 and spec 121/0. They equal report-draft.md:28-30.

**Spec "The README" (spec:259-275) and rulings AH and AI, each checked against records/:**
- **Feature lines.** All twelve now cite records/size-m5p6-2026-10-18-ef9da37.md (README-repo.md:23-34). Ruling AI (1) asks for exactly this. Every cost is in the record's Sections column (size record:229-247): dates 4840 (over the styles probe, as the README says), rowcol 58152, metadata 50036, rules 48468, tables 46536, comments 63356, images 57620, richtext 13384, sheetprops 28284, read 112600, copy 13820, charts 105916.
- **Fidelity.** The new sentence (README-repo.md:60) says "part 5's five chart samples", as ruling AI (3) asks. The host and i686 rows of all five m5p5-p4 records read 27/27/27/27 (each record, lines 19-20), and each record's tool argument fits the description given.
- **Copy sentence.** 784 of 917 matches m3p6-p4-copy:47.
- **Open sentence.** 1093 pairs, 1082 opened and eleven not opened match m3p6-open:8 and :14.
- **Size sentence.** 436992, 21760 and 458752 match the size record:233 and :284-290.
- **Speed sentence.** The scale-record figures match scale-m3p1:59-60 and :125-131. The added tier c sentence matches compare:133: 1159829654 bytes, 2.10 s, 7896 kB.
- **Comparison paragraph** (README-repo.md:70):
  - eleven ops, and every row reads files 27, ok 27, lo ok 27 (compare:9-41);
  - on the roundtrip, 870 identical and 0 dropped for emend against 28 and 533 for openpyxl (compare:9, 11);
  - medians 0.10 s on every emend host row and 0.81 s on every openpyxl row;
  - the roundtrip's maximum RSS 8012 kB for emend and 673976 kB for openpyxl (compare:9, 11);
  - "including its container's start" matches compare:3;
  - every emend scale row ok, and openpyxl's tier c roundtrip a cap: 137, 320.13 s, 16576384 kB (compare:135).
  - All four sentences cite only the comparison record, as ruling AH requires.
- No "faster" or "better" appears, and no U+2014.

**Notes (not defects in the diff):**
- (a) Ruling AI's text says "the other ten equal to M3 part 7's" (spec:864). Twelve features less images, read and charts leaves nine, which is the count the diff shows and the report gives (report-draft.md:59). The ruling's "ten" is off by one.
- (b) ROADMAP's Unscoped list still holds "README, where every fidelity, speed or size sentence cites a record" (phase.diff:150). That is planning's text, not in P4's scope.

## Q2. Do the logs contain every line the spec and the rulings in force require?

**Present:**
- **Ruling AA.**
  - gate.log:4-5 and 305-309 name the superseded files and hash them: 203d4cdb, 600501 bytes.
  - test.log:3-4 and 129-132 do the same: db66bdfb.
  - negative.log:3-4 and 79-82 do the same: f83ec4c6.
  - run.log:3-4 and 54-60 name and hash both earlier run logs (4ce95aa4 and 932d74dd) and assert it (line 60).
- **Ruling AF, gate.**
  - P3's target root is checked absent (gate.log:208-210).
  - The defined-helper lines cover all five scripts, "(20 lines yes ...)" (gate.log:127-148).
  - The gate's own source is checked, "(4 lines yes ...)" (gate.log:66-71).
  - Ruling AD's yn/ynnot control is there (gate.log:69, 77).
  - The phase diff's files and each README commit as README.md alone are checked (gate.log:6843-6861).
- **Ruling AF, test.** The workspace passed 1322 on each target, and the six feature sets passed 515, 618, 552, 659, 721 and 1098 per target, each equal to P3's figure (test.log:1151-1289). There is no API record.
- **Ruling AF, negative.**
  - NC passes (negative.log:3021).
  - N1 to N10 are each refused with their kind (negative.log:3033-3042).
  - N4 is the moved-by-one figure, 436993 cited against the size record (negative.log:2389, 3036).
- **Ruling AF, run.**
  - 14 paths cited and 14 copied (run.log:2815).
  - The README check program exits 0 (run.log:2884), with its planted control (run.log:2887-2889).
  - Every figure is read by a printed command (run.log:2891-3110).
  - Both labels' rows are read for the five chart records (run.log:2984-3012).
  - Tokens 42 equal the figures checked, 42 (run.log:3114-3116).
  - Features 12 equal the manifest's 12, each with its specs/ paths (run.log:3123-3140).
  - The hygiene checks each have a control (run.log:3143-3166).
- **Rulings Z and AD in the other scripts.** test.log:96-110, negative.log:44-60, run.log:23-29 and scan-pre-check.log:37-49.
- **Endings.** All four logs end "checks all as expected: yes", and the scan pre-check ends "RESULT: PASS".

**Gaps:**
1. **Ruling Z is vacuous for the gate.** The gate's own check reports "0 names called" (gate.log:66, 127), because the gate calls its helpers through `$(is ...)` (for example gate.log:4279 and 4304). The defined-helper program only matches `yn|ynnot` calls (negative.log:46, the CALL pattern). So the gate's own helpers are never checked for being defined. The line is printed but checks nothing.
2. **Only the run log asserts its RERUN_FILES hashing.** run.log:60 checks it with a line of its own. gate.log, test.log and negative.log print the hashes but have no check line that every named file was hashed. This is weaker than "the run log's form" that ruling AA cites.
3. **The scan pre-check prints "RERUN_NOTE is not set" (scan-pre-check.log:5-6).** By the report's own account it is the third pre-check: the first is in superseded/logs-1 and the second in superseded/run-rro-1. The scan's carried comments say "a rerun pre-check sets RERUN_NOTE" (scan-pre-check.log:1593, 1627, 1675). Ruling AA's literal text covers attempts refused at entry, so this is not strictly required for an uncommitted pre-check. The final scan.log should name and hash both earlier pre-checks.
4. **The prune evidence is not in these logs.** The removal of P3's target is only in the superseded first gate log, which is not among the inputs. This gate shows "target: absent" alone (gate.log:209).

## Q3. Does every number and quoted string in report-draft.md appear in a primary?

**Verified:**
- the four log sha256s and the logs README's sha256, all recomputed and equal (report-draft.md:41-45);
- PLANNING_SPEC 8019cc25, recomputed from spec-0037-working.md;
- the spec pin 09ad50a1 (gate.log, scan-pre-check.log);
- ROADMAP 1b5ef67c (gate.log:236, 6780);
- the full commit hashes (test.log:113-122);
- the end times 14:23:57Z, 14:25:13Z, 14:25:15Z and 14:26:32Z (the logs' tails);
- every quoted run, negative, test and gate line in report-draft.md:68-87;
- the superseded hashes 203d4cdb, db66bdfb, f83ec4c6, 4ce95aa4 and 932d74dd;
- ee2564b, 324c8ce, a1c60f6, p4-before-reword and p4-before-cite (spec:836-855);
- the numstat table (marked as no log printing it, and correct against phase.diff).

**Unmarked and not found in the inputs:**
1. **report-draft.md:88-90:** "target: 7607601108 bytes" and "target: rm -r exit 0". Neither string nor the number appears in any input. gate.log:3880 is only a comment saying "rm -r exit 0". The source is attributed to the superseded first gate log, which was not provided. Mark it as the session's statement, or include that log.
2. **report-draft.md:104-105:** the claim that the first check "answered Q1, Q2 and Q3 YES". The only source is logs-README.md:55-56, which sits in its own "the session's statements; no log prints them" section. Spec AI says only that the logs passed. Mark it.
3. **report-draft.md:110-112 and :117-118:** what superseded/logs-1 and superseded/run-rro-1 hold (the first pre-check, the first check's inputs and answer, "that pre-check"). The RERUN_FILES lines confirm only the log files. Mark it.
4. **report-draft.md:119-121:** "RRO adds one echo line and no yn call". This is partly supported: the gate counted 69 names for the run script (gate.log:139), run.log counts 69 (run.log:28), and the RRO root line is one echo (run.log:68). "One echo line" itself is not shown in any input. Mark it or cite those lines.

**Overstatement:** report-draft.md:12-13 says each log "hashes them by RERUN_FILES". That is true, but only run.log asserts it (see Q2 gap 2).

Nothing else differs.

## Verdicts
- **Q1: YES.** The diff does what P4 and rulings AE, AH and AI ask and nothing else, and every README figure is in the record its sentence cites.
- **Q2: PARTLY.** Every line the spec and ruling AF name is present. But the gate's ruling Z check is vacuous, the gate, test and negative logs print their RERUN_FILES hashes without asserting them, and the scan pre-check has no RERUN_NOTE.
- **Q3: PARTLY.** Four statements are unmarked and unsourced in these files.

## Fixes
1. Mark report-draft.md:88-90, the "YES" claim at :104-105, the contents of superseded/logs-1 and run-rro-1 (:110-112, :117-118) and the RRO line count (:119-121) as the session's statement, or cite primaries for them.
2. Make the defined-helper check also scan `is`/`isnot` calls, at least for the gate, or disclose that the gate's ruling Z line covers 0 names.
3. Add run.log's assertion line ("every file of the attempt RERUN_NOTE names is hashed above ...") to the gate, test and negative scripts in future logs, or disclose that only the run log asserts it.
4. Run the final scan with RERUN_NOTE and RERUN_FILES naming the two superseded pre-checks.
5. Tell planning that ruling AI's "the other ten" (spec:864) should read nine.