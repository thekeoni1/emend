# M8 part 3 P2: independent check 1

The answer of a fresh subagent given only these materials, under
<scratch>/check1, and the three questions of CLAUDE.md ("Independent
check before reporting"), with the P2 points of specs/0049 and
specs/0044 named in its brief:
- specs/0049 at the phase commit (f3f18eb772f998298fb996769114eb9a956f0f1b1de75276a44fbf53ba3dab7c) and specs/0044's P2 sections;
- planning's rulings 1 to 8 given by message for the logged run (planning-word.txt);
- the doc commit's diff (git show 69a3227);
- gate.log, negative.log, run.log, the scan's pre-check output (7e7fbd12da2fe6d34e163bb41d31ae55e99ab67ac03117cc647cdfcabff060bd) and <phase>/logged/driver.out;
- the record as written into records/ and planning's copy of it (both e58daaf825efca6493438aa3f80af729e1790503a20e387291a8b49f62ce1ab9), and the by-hand file;
- the logs README as it stood before the check (sha256 0519e1e803648553d5b685c7ebe6c0da27cd1bfab636943b05d7281df3f730bd) and the draft report (43ae31a344ebbf57a2d8b06b4281baede4ce6d388f0237a326e3002b35257282).

The answer was extracted from the subagent's transcript by script into
<phase>/check1-answer.txt (sha256 13fa9427a93aa5b316e7f92f60f698c322e784a361e88b9623c0f4a0980d5876). It appears below verbatim. The
README's section "After the independent check" answers each exception.

root: <scratch> stands for <phase>, as in the gate and run logs
root: <phase> stands for <home>/.cache/emend-build/m8p3-p2, the phase's directory
root: <home> stands for the home directory of the account the runs ran under

---

My context included a CLAUDE.md, a memory index, a git status and user details from the harness. I did not use any of them as evidence.

**1. Yes, with one point for planning to rule on (see 2).**

- **Diff.** doc-commit.diff touches only ROADMAP.md and specs/0049.
  - ROADMAP: the P1 row is set to DONE and the P2 row to IN PROGRESS (diff 20-23). The ruling F item is marked DECIDED (diff 31-36).
  - specs/0049: it adds "Planning rulings during P1" A-D, "Planning verification of P1" and "Planning rulings before P2" E-G (diff 46-187).
  - The message names the blobs: the spec at f3f18eb7... (diff 7) equals the sha256 of spec-0049.md, and ROADMAP is at 1325fddf... (diff 8). run.log 2541-2553 confirms both against the phase commit. There is no code commit: run.log 39 and scan-pre-check.log 6676 area, "the range holds no code commit".
- **Record byte for byte.** record.md and record-from-planning.md are cmp-equal; both have sha256 e58daaf8...38aa3f80af729e1790503a20e387291a8b49f62ce1ab9 and 667640 bytes. run.log 2557-2558 pins planning's copy. run.log 2609-2611 shows the copy's sha256 equal and "the copy is byte-equal to planning's file (cmp): yes". driver.out 16 repeats the sha256.
- **Assembly rerun.** run.log 2615 runs the script at its pin 0ff31ed9 (run.log 2559-2560) with:
  - --list and --results from the stage, both pinned (run.log 2563-2566).
  - --byhand byhand.tsv, pinned at b6304939 (run.log 2561-2562).
  - --earlier m7-2026-10-26 then m7p2-2026-10-28, oldest first, each pinned (run.log 2568-2573).
  - --excel "Microsoft Excel LTSC MSO (Version 2408 Build 16.0.17932.21056) 64-bit" and --os "Microsoft Windows 11 Home 10.0.26300", the same as record.md 7-8.
  - --date 2026-11-12 and --byhand-date 2026-11-12.

  The result is exit 0 with 0 bytes of stderr (2616). The output's sha256 equals the record's (2617), and "by sha256 and cmp ... yes" (2618), with its control (2619).
- **Acceptance per the six rows.** run.log 2683 names the six as spec-0049 116-118 lists them. Each of 2705-2710 reads pass|pass|pass|pass with counts 1 1. Line 2711 reads "pass 6 of 6" and 2713 "yes". The control is at 2715-2716.
- **Acceptance in totals.** run.log 2741. The fixture pairs are recomputed and equal at 2726-2728, and the findings and acceptance agree at 2733-2737.
- **Gate prune.** The gate prunes only m8p3-p1's target:
  - "PRUNE_PREVIOUS: yes" (gate.log 191) and "the prune set, 1 names: target" (192).
  - 23 names kept (193); 6475256310 bytes removed (229); "rm -r exit 0" (231); the kept names are still there (233).
  - The rm -r form's control runs at 185-190.
  - driver.out 10 shows the target gone and keep-host present.
- **Planning's rulings.** The gate ran once, before the run (driver.out 7-14), as ruling 5 says. PLANNING_SPEC and PLANNING_ROADMAP were unset (run.log 6-7) because the working files equalled the blobs (run.log 2541-2553). Nothing was pushed or committed beyond the doc commit (driver.out 25-27).

**2. No, on a strict reading. One required set of lines is missing; everything else is there.**

- **Present:**
  - The gate, ending "gate log checks all as expected: yes" (gate.log 12262).
  - The assembly's two stops on scratch copies of the results file: A1 planted (negative.log 6441-6443, 6454) and A2 removed (6445-6447, 6455), with the control AC assembling to the record's sha256 (6452-6453).
  - The same stops on fictional fixtures: R1 (6413) and R2 (6414). Ruling E's added stop: R3 (6417) and A3 (6458), kept by planning ruling 1.
  - The row-found counts: run.log 2636-2639, with the control at 2640-2642.
  - The totals recomputed and shown equal: run.log 2659-2664, with corpus counts at 2669-2672.
  - The acceptance: run.log 2705-2741.
  - The FINDING summary: run.log 2761-2762.
- **Missing:** spec-0044-p2.md 117-118 (P2's negative: "the restored check's two cases") and 66-70 call for the restored Excel-record check run accepted at the tip and refused on a clone, with its extract's sha256. None of this is in negative.log. Its sections (6322, 6372, 6420) hold only the list script and assembly cases, and line 14 says it carries the cases "as M7 part 2 P2's record run ran them". spec-0049 130-132 moves the m8 Excel-record check into the close-out, so the function may not exist yet in P2. But no ruling in these materials (spec-0049 E-G, planning-word 1-8) says these lines are dropped in P2. Planning should confirm.

**3. No. Most figures match; these do not, or cannot be checked from these files:**

- **README 109:** "The inputs at their pins (lines 2557 to 2574)" lists P1's zip among them. The zip is at run.log 2575-2576, outside the cited range.
- **README 147-149:** the quote of P1's scan.log line 6716, "... and no test, size or negative log", is in no primary here. scan-pre-check.log 4586-4590 has only "The two sibling logs are always present: ..." and "no ... negative log". doc-commit.diff 80-81 has line 6716 and "The two sibling logs ...".
- **README 22:** "<scratch> stands for <phase>, as in the logs" is not true of negative.log, where <scratch>/tmp stands for <build>/tmp, that is <phase>/negative/tmp (negative.log 102, 104). It is true of gate.log 365.
- **Not in any primary here** (driver.out 24 only prints "<phase>/tmp names unchanged: no"):
  - README 171: the name tmp.P497hRVbkh.
  - README 174-176: "dry-d1-scripts-out".
  - README 177-180: dry d1's pre-check FAIL and its lines 95, 6797 and 6916.
  - README 143: "Dry run d1 printed none either".
  - README 184: 6597437998 bytes.
  - README 90: "the stop report's question 2".
  - README 185: superseded-R appears only in planning-word.txt ruling 8.
  - Draft report 10 and 23: the paths <phase>/logged/driver.out and <phase>/scan/pre-check-1.out.
- **Not a mismatch:** README 90 says "changes NA67 to NH67" while negative.log 14 says "NA67 to NG67". The script's own list has NH67 ("This header", negative.log 220), so both are backed.

**Every other figure and cited line checked matches its primary:**

- **README:**
  - Log bytes, lines and sha256 at README 42-44: wc -c and sha256sum; scan-pre-check.log 6721-6741.
  - Start and end lines: gate.log 357 and 12261, negative.log 92 and 6479, run.log 58 and 2776.
  - gate.log 183-237, 185-190, 193, 229, 231, 233, 238-256, 11226, 12232 and 12237.
  - negative.log 6159, 6336, 6349, 6358, 6367, 6406, 6413, 6414, 6417, 6453, 6454, 6455 and 6458.
  - run.log root lines 67 and 69, and lines 2568, 2569, 2611, 2616, 2617, 2637, 2659, 2660, 2664, 2705-2710, 2713, 2741, 2745-2748, 2758, 2761 and 2762.
  - The change ranges A106-U106 (gate.log 6151-6206), XM-XT (run.log 124-159) and NY-OE (scan-pre-check.log 4550-4599), including OE's CLOSEDOCC description.
  - driver.out's quoted lines and the logged.sh sha256 (driver.out 1, 6, 7, 10-13, 16).
- **Draft report:**
  - Commits, hashes and parent: doc-commit.diff 1-8; run.log 42.
  - Lines 11-20: gate.log 193, 229, 231, 233 and 12232; negative.log 6336-6458; run.log 2611, 2616, 2617, 2637, 2659, 2660, 2664, 2705-2711, 2713, 2737, 2741, 2745-2748, 2758 and 2762.
  - The last lines of all three logs, and the pre-check's "RESULT: PASS" (scan-pre-check.log last line).

**Privacy:** none found. No log, the README or the draft report prints a private workbook's cell content, file name, sheet name or table name, or any message or seen cell of the results file.

- run.log prints the results file's header (2583), the by-hand cell "repair (by hand)" (2674), and the record's findings and acceptance text (2733-2735).
- The real-data STOP lines in negative.log have their row keys cut (6443, 6447, 6451).
- The only .xlsx names in the logs are fictional fixtures (negative.log 6280-6309 and the tsix-* names).
- Workbook ids, test names, sample labels and counts appear, which is allowed.
