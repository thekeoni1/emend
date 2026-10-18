Independent check 1

Implementer's header (not the checker's): a fresh general-purpose subagent, given only the files under <phase>/check1/ (now kept in <phase>/superseded/logs-1/check1), answered the three CLAUDE.md questions on the first four logs, at the first README commit 7355d98. Planning's ruling AI superseded those logs; the answer stands as given. Its inputs:

| File | sha256 |
|---|---|
| README-repo.md | c29c860148cd8404c22e4f524a094c4f19ce56ef6899d6754134bfcd7ecce5e4 |
| gate.log | 203d4cdb4ab882ee182cac015fabf82cc7c1aae74080c34c531e99a234270d96 |
| logs-README.md | 8713caa5c0030b2e861c74bd9ea32443c04a5bcb1a8026f807cb590b191054f3 |
| negative.log | f83ec4c64c502037be265f3f59999e8cc71373bad990c70debc2390d16127133 |
| phase.diff | 2433accb5d950ffefee5325984c0056f7295d0d9aad0dd96c97554da168142a8 |
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
| records/size-m3p7-2026-10-09-09d2e0f.md | 30e1a5dcb24616469e4200f4d860e1f3b9fd5839c0ff7cc1bfc45b1bae713130 |
| records/size-m5p6-2026-10-18-ef9da37.md | 8d7e65ca3b01742345e083ca77c6d59e6ba674b3878a098f54959bd388415611 |
| records/toolchain-2026-09-15.md | 5a6834de5c46c10ae9d6bde992aa736659008c67985112788be11ff14d1ea013 |
| report-draft.md | 6dc171c1f70b03faa9fd82716795a84f0d244a40086646aab130d29572a33c56 |
| run.log | 4ce95aa47816afbf1f9b2012b2e9835b8ee8759ee4f6ccb0cb5f9ab496704c51 |
| scan-pre-check.log | 047cb762f5cb121dae44528b9abdf15eae069f2314684cb2fdd82a62aa7cd341 |
| spec-0037-working.md | 0b3dd2ce3c3e37a9b940615cf97864017169712f00f7087b24747d584a680982 |
| test.log | db66bdfb15c03283cff9c7a3e5fb7098d953df4afd38e2b6cf404d2eefe67e11 |

The answer, verbatim (sha256 f7142327df666e7ed9c3b9b8fd605ff94369e8da590aa4c834b5012db3f1c39d):

No workbook cell content found in the inputs. I read the README, the diff, the report, the logs README, the spec sections and the records' tables in full, and the logs by section. The records cite workbooks only by id, producer and tags. scan-pre-check.log ends "total hits: 0", "structural checks failed: 0", "RESULT: PASS" (lines 4686-4690).

Hashes, recomputed in `<phase>/check1`, match what the logs print:
- README-repo.md = c29c8601... (run.log, the files read at the phase commit; gate.log, the changed files at the phase commit).
- phase.diff = 2433accb..., "the phase's combined diff" (gate.log 6754-6755).
- The four logs and logs-README.md match report-draft.md lines 39-43.
- spec-0037-working.md = 0b3dd2ce... (PLANNING_SPEC).
- The 14 records match the hashes run.log prints at the phase commit.

## Q1. Does the diff do what the spec says for P4, and nothing else?

**Scope**
- phase.diff numstat, counted from the diff: README.md 8 added and 3 deleted, ROADMAP.md 72 and 5, specs/0037 121 and 0.
- **README** (phase.diff 1-33): the `read` line now cites the M5p6 size record, the `charts` line is added, a Milestone 5 fidelity sentence is added, the size sentence is re-cited, a tier-rerun sentence is added and the comparison paragraph is new. This matches "The README" (spec 259-275) and "P4: the README" (spec 342-346).
- **ROADMAP** (phase.diff 36-131): the P3 row is DONE, the Milestone 6 section is inserted before "Unscoped", the Unscoped charts bullet is rewritten, and the P4 row stays PLANNED. This is ruling AE (phase.diff 254-259), which widens P4's "Unscoped charts bullet, the part 6 rows" to include the Milestone 6 section.
- **specs/0037** (phase.diff 140-259): the P3 rulings W-Y, the P3 verification and Z-AE, as planning left them. The gate's blob checks show the doc commit's two blobs are the expected ones.
- **CHANGELOG.md** is unchanged: the same sha256 at base and phase (gate.log, the CHANGELOG lines in the "README commit touches README.md alone" section).
- Nothing else is in the range (gate.log: "the phase diff is specs/0037, ROADMAP.md and README.md, nothing else").

**Every digit-bearing README sentence, checked against its record in `<phase>/check1/records`**
- **`read` 112600 (README 32):** size-m5p6 line 238 (also 272 and 276). OK.
- **`charts` 105916 (README 34):** size-m5p6 lines 229 and 249. OK.
- **Size (README 66), 436992, 21760, 458752:** size-m5p6 lines 233, 284 and 290. OK.
- **Milestone 5 fidelity (README 60), 27 written and opened on host and static target, 27 inputs:** each of the five m5p5-p4 records' lines 19-20 read files 27, ok 27, lo ok 27, lo_in ok 27 on both host and i686. The tool arguments (line 11 of each) match the five samples named. OK.
- **Tier rerun (README 68), 1159829654, 2.10, 7896:** compare record line 133. OK.
- **Comparison (README 70):**
  - 27 workbooks, eleven ops, and 27 of 27 written and opened per op and tool: compare lines 9-41.
  - 870 identical, 0 dropped, against 28 and 533: lines 9 and 11.
  - Medians 0.10 for every emend host row and 0.81 for every openpyxl row: lines 9-41.
  - Roundtrip peak RSS max 8012 and 673976: lines 9 and 11.
  - "emend completed every run": 48 of 48 emend scale rows are ok.
  - openpyxl tier c: cap, 137, 320.13, 16576384 (line 135).
  - The paragraph cites the compare record only, as the spec requires.
- **Unchanged sentences, rechecked:**
  - The scale figures are all in scale-m3p1.
  - The size-m3p7 feature costs are all present there.
  - 784 and 917 are in the m3p6 copy record.
  - 1093 and 1082 are in m3p6-open (lines 8 and 14), which leaves 11 not opened.
- I found no digit the cited record does not hold.

**Adversarial notes (none is a figure error)**
1. **Stale `images` cost (README 29, not edited, outside P4's literal scope).** The line still gives 67832 from size-m3p7. The record the README now treats as Milestone 5's closing figure gives `images` 57620 (size-m5p6 lines 236 and 252; the shrink is at line 321). Only `images` and `read` differ between the two records; `read` was updated, `images` was not. The sentence is true to the record it cites, but the README now mixes vintages without saying so. This needs a ruling by planning, not a P4 fix on its own authority.
2. **Comparison paragraph is roundtrip-only for two figures.** The spec (265-268) asks for identical, dropped and opened "per op or for the set", plus peak RSS. The README gives identical/dropped and peak RSS for the roundtrip only. In the record, emend drops entries on other ops: insert-rows 2, names 9, copy 5 (lines 21, 24, 39). The wording "On the roundtrip" is accurate, and planning's read of 324c8ce accepted it (spec 806-814), but it is narrower than the spec's text.
3. **"Milestone 5's five chart samples" (README 60).** Ruling W (phase.diff 147-150) names seven distinct Milestone 5 chart samples, including part 4 P3's addchart and chartkinds. The spec's wording is "the five of part 5 P4's reruns". "Part 5's five chart samples" would be exact.
4. **README 21 says "from the size record" (singular)**, but two size records are now cited. Cosmetic.

## Q2. Do the logs contain every line the spec and the rulings in force require for P4?

**gate.log**
- Ruling AF:
  - The defined-helper and AD lines for all five scripts, each with two controls ("20 lines yes", line 148).
  - AD's planted script calling yn and defining neither (lines 151-152).
  - Only P3's `target` pruned: "the prune set, 1 names: target", "target: 7607601108 bytes", "target: rm -r exit 0" (lines 156-217).
  - The README commit's files are README.md alone; CHANGELOG is unchanged.
- Ruling Z: the gate's own source is checked inside its shape block (lines 32-77, "4 lines yes").
- Ruling AA: RERUN_NOTE and RERUN_FILES are printed, both unset (lines 4-5).
- Count rule: aad0b35 has no count; 7355d98 has counts with the diff-alone sentence.
- The last line is "yes".

**test.log**
- The W, F, E, R, C and D sections are present.
- Passed counts: 1322, 515, 618, 552, 659, 721 and 1098 on both targets, each equal to P3's test.log and gate.log.
- No API record and no plants.
- Ruling Z is in the shape block (line 102); RERUN lines at 3-4.
- The last line is "yes".

**negative.log**
- Ruling Z and AD block at lines 22-60. AD here is a grep form (`^(yn|ynnot)\(\) \{`), weaker than the run log's program, but it has a control.
- NC passes.
- N1 to N10 are each refused with their kind. N4 is ruling AF's existing record with a figure moved by one: 436993 against the size record (line 2365).
- RERUN lines at 3-4. The last line is "yes".

**run.log**
- Ruling Z and AD (lines 22-28) and RERUN lines.
- Item 1: 15 paths, readme_check exit 0 with stderr 0, 25 digit sentences all cited, and the planted-sentence control.
- Item 2: every figure is read by a printed command. Ruling AF's host and i686 rows are read for all five records, each with a control.
- Item 3: 42 equals 42, with a control.
- Item 4: 12 equals 12, with specs paths.
- Item 5: the hygiene checks with controls.
- The last line is "yes".

**scan-pre-check.log**
- Ruling Z and AD at lines 35-49; RERUN lines; PASS.

**Gaps**
- One line in the negative log reads "no" inside a passing check: the planted c-defined.sh's own "control: ... no" (line 55). The enclosing check reads yes (line 56), so it is cosmetic.
- The AG rewrite (old against new `git diff` empty) and the stopped dry gate are printed by no log. Both are session statements, disclosed as such.
- Ruling AA covers attempts refused at entry. The stopped gate (report item 4) is called "dry". Planning should confirm it wrote to no logged path; otherwise AA's RERUN form would apply.

## Q3. Does every number and quoted string in report-draft.md appear in a primary?

These are sourced:
- The commit hashes and subjects (run.log lines 30-37, gate.log 6750-6755).
- The spec blob 09ad50a1 and PLANNING_SPEC 0b3dd2ce.
- ROADMAP 1b5ef67c (gate.log, the changed files at the phase commit).
- The log hashes and last lines.
- The four end times (gate 13:56:40Z, test 13:57:56Z, negative 13:57:58Z, run 13:58:02Z).
- "paths the README cites: 15; copied into the root: 15".
- The (42) and (12) counts.
- The quoted AF row string.
- 1322, 515, 618, 552, 659, 721 and 1098.
- The "20 lines yes" quote, "4 lines yes", "target: 7607601108 bytes" and "target: rm -r exit 0".
- The N1-N10 kinds.
- QZ9 (negative.log 186-190).
- The AG and AH hashes ee2564b, a1c60f6, 324c8ce and 7355d98 (spec AG and AH).
- The superseded hash prefixes, given in full in logs-README.md.

The numstat (8/3, 72/5, 121/0) is marked "no log prints it" and is correct by count of phase.diff.

These are marked as the session's statement and are not sourced, as expected:
- Nothing pushed.
- Outer stderr 0 bytes.
- The branch p4-before-reword.
- The dry negative failure.
- The 14 s stop.
- The test generator repin from 2 to 4.

Unmarked or loosely marked:
- "reworded by `git commit-tree`" (report line 97). No primary states the mechanism (the spec says "ruling X's form"). The item's marker sits only on its last sentence.
- "the same class as P3's change PR" (line 110). No primary names a change PR. It sits inside an item marked at its head, so it is covered, but loosely.
- "Each log ran once; no logged attempt was refused or stopped" (lines 11-12) is unmarked. It is supported only indirectly: RERUN_NOTE is unset in all five logs, and logs-README.md says "once each".

Nothing in the report differs from a primary. Omission: the report does not mention notes 1-3 of Q1.

## Verdicts
- Q1: YES. Every figure matches its record, and the diff stays in scope with ruling AE. Notes 1-3 are for planning.
- Q2: YES.
- Q3: YES, with two loosely marked statements.

## Fixes
1. Ask planning to rule on the `images` line: re-cite it to size-m5p6 at 57620, or say the feature list mixes records.
2. Ask planning to rule on whether "per op or for the set" needs identical/dropped beyond the roundtrip, given emend's drops on insert-rows, names and copy.
3. Reword "Milestone 5's five chart samples" to "part 5's five chart samples", or similar.
4. In the report, mark "reworded by git commit-tree" and "change PR" as the session's statement in place, or drop the PR comparison.
5. In the report, state that the stopped dry gate wrote to no logged path. If it did, apply ruling AA.
6. Optional: report Q1 notes 1-3 as known limits in the report.