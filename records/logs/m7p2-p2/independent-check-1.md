# M7 part 2 P2, independent check 1 of the stage run (its answer verbatim, but the home directory's path written <home>, once, and the harness session's scratch directory written <session>, once; the answer as given is sha256 6ef88673d23c50ade0bc38f914b62cda713030b4ad23a3a0e4e4bfc401bc4692 in <phase>/logged/check1-answer.md)

I found two disagreements, both small. Report line 35 says run.log matches the word FINDING once, but it matches twice outside the printed script. Report line 55 gives script hashes it does not mark as the session's, and the gate script's hash is in no log or spec. Nothing else in the report, logs or diff disagrees with the spec or the stage.

**1. Does the diff do what the spec says P2's doc commit holds, and nothing else? Yes, with one exception in the subject line.**

- **What the commit holds:** the diff (58c4f51 to 5ae8367) changes two files: `ROADMAP.md` (+2/-2) and `specs/0045-publish.md` (+346/-0). That matches spec line 344 ("P1's verification and rulings; the P2 row IN PROGRESS") and ruling P, which names this spec and `ROADMAP.md`.
- **ROADMAP:** the P1 row goes from PLANNED to "DONE 2026-10-28 (...)". The P2 row goes from PLANNED to "IN PROGRESS 2026-10-28 (`specs/0045` ruling P)". No other row changes.
- **Hashes in the message:** the message names both files with their full blob sha256 (spec b3121df7..., ROADMAP 466fc0da...). I computed both from `git show 5ae8367:<path> | sha256sum` and they match. gate.log 8681 to 8732 confirms the same, with controls. The message is ASCII and ends with one Co-Authored-By line.
- **Exception, the subject line:** it reads "P1's verification and rulings M to P", but the spec diff also adds the whole "Planning rulings during P1 (2026-10-27)" section, rulings A to L. That section was never committed before: 58c4f51's spec has 398 lines and ends at "P4: the publication verified". Ruling P says the commit holds "this spec", so the content is right; the subject understates it. The ROADMAP P1 row itself says "rulings A to O".
- **Rulings Q, R and S** are only in the working copy (94 lines, uncommitted), as the README says.

**2. Do the logs contain every line the spec and rulings require? Yes, with exceptions.**

- **(a) Directory list and flag order: yes.**
  - My own read of the USAGE constant (`main.rs` line 1170, 17 lines) gives the order edit, formula, remove, protect, chartsheet, chart-surface. It names 59 `-sample` flags, none twice.
  - run.log 2370 to 2380 prints the same: 59 flags, the six in that order.
  - Section 5 (2422 to 2427) runs `--run` formula, then `--run-plain` edit, remove, protect, chartsheet, chart-surface, which is the USAGE order with formula taken out.
  - All six directories are `2026-10-27-43471ed-*`, each with commit 43471ed and its own flag as tool_args (2381 to 2397).
  - Minor: the command line at 2430 is printed as a template ("--run-plain <each directory above>"), not expanded. The order rests on the list at 2422 to 2427 and the check at 2428.
- **(b) Per-source counts: yes.**
  - The log (2448, 2468 to 2473) states corpus 162, fixture 2239, input 2239, 4642 files staged.
  - My own counts: 4642 files under the stage, files/corpus 162, files/fixture 2239, files/input 2239. The list's first column (awk, header skipped) gives corpus 162, fixture 2239, input 2239, with 4641 lines.
  - The list's sha256 is dd796c22... and its size 1241251, mode 600. The zip's sha256 is df824832..., size 124132653, mode 600. All files are mode 600 and all directories 700. These agree with the log.
  - I could not count the zip's entries myself because `unzip` is not installed. The 4642 entries rest on the log alone.
- **(c) Every emitted FINDING line listed in the report: yes.** No log emits a line that begins with FINDING. In uppercase, gate, negative and scan-pre-check have 0 matches anywhere; their lowercase matches are all inside printed scripts. run.log has 11 matches: 9 inside its printed script (lines 95 to 2143) and 2 emitted, the heading at 2493 and the summary at 2494.
- **Other required lines, all present:**
  - Gate: the prune set empty, P1's directory and keep checked unchanged at the prune and at the end (10686 to 10689), and nine PASS lines.
  - Negative: LC, L1 to L3, FC, F1, F2, SC, S1, S2, and NH65's line at 6062. It extracts the run script's functions by line range, and their hashes match the run log's.
  - Scan pre-check: EXPECT_RECORDS=0 with its control (6122 to 6124). Hits 0, errors 0, structural checks failed 2, at lines 96 and 6251.
- **Gap:** ruling P says the Milestone 2 and part 1 stages and zips are not read, moved or removed. No log proves that; run.log 85 to 86 only refuses their names. My own listing shows both still present with modification times from 2026-10-09 and 2026-10-25/26.

**3. Does every number and quoted string in the report appear in a primary? Yes, with exceptions.**

- **Line 35:** "The one match for the word in run.log, at line 2494". Line 2493 ("## the FINDING lines of this log ...") also matches, and there are 9 more inside the printed script. It should say two matches outside the printed script.
- **Line 55, "Script sha256":** not marked as the session's statement; the README marks the same table as one.
  - negative 085e8876, run f9ebe043 and scan 663febe1 appear as prefixes in rulings Q and R; the negative log also prints run f9ebe043 in full.
  - gate 451365b0... appears in no log and not in the spec, only in the README's session table. I hashed `<phase>/scripts/*.sh` and all four full hashes match.
- **Line 41:** "line 71, is the LOGDIR input itself". Line 71 is the echo "LOGDIR=... RECDIR=records (as used)"; the input line is line 3, which prints `<phase>/logged`. The count is right (27 lines, 26 under `<phase>/logged/` plus line 71); only the wording is off.
- **Line 48:** "d2: all yes". Ruling R says d2's scan pre-check read FAIL on its two dry-layout lines. The report says this for d3 on line 49 but not for d2.
- **Lines 47 to 51, "Dry runs":** unmarked. Their figures do appear in rulings Q, R and S (TN, LV, NH65, d823f16e, the zip and list hashes). "count_matches -xF --" is in the README's session section, not in a ruling.
- **Line 20:** "Outer standard error is 0 bytes for each step" is a separate sentence outside the session's parenthetical. The four `*.outer.stderr` files are 0 bytes, but they are not listed primaries. The driver times are marked as the session's; the log times agree (gate 02:44:45 to 02:51:44, negative to 02:51:46, run to 02:53:26).
- **Everything else matched a primary:**
  - The phase commit's sha and subject, and PLANNING_SPEC 2dd9a9b4: equal to the working spec's sha256 at 838 lines.
  - All three log hashes, which I recomputed.
  - The gate and negative items.
  - The list-script stdout quote (run.log 2432), the list, stage and zip figures, and keep 41afdbbf at entry and end.
  - The pre-check quotes (lines 96, 6251 and 6122) and its totals.

**What I did beyond reading**

- Wrote the USAGE constant's lines to a scratchpad file: `<session>/scratchpad/usage.txt`.
- Ran sha256sum on `<phase>/scripts/*.sh`, which is not a listed primary.
- Compared run.log's printed script with `<phase>/scripts/m7p2-p2-run.sh`. They are identical apart from one trailing blank line.
- Listed `<home>/emend-testing/work/` with sizes and modification times, names only.
- Printed the list's header column count and first column name ("13 source"), a header and not a row. No row was printed.
- Tried `unzip -Z1` to count zip entries; the command was not installed and printed nothing.
- Modified nothing in the repository, under `.cache/emend-build` or under `emend-testing`.
