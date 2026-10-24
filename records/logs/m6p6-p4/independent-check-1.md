Check 1, M6 part 6 P4 (specs/0043 "P4: the README", rulings T and U)

I checked the draft report against the primaries. Verdicts: Q1 yes, Q2 yes, Q3 yes with exceptions. The exceptions are wording points. I found no figure that is missing or differs.

**Q1. Does the diff do what the spec says and nothing else? Verdict: yes.**

Evidence (git, diff.patch, commit-messages.txt, the records):
- diff.patch has sha256 89cd4d13..., the same as `git diff 42cf079 37062b7`. The chain is 42cf079, then 92c6b38, then bb0a29e, then 37062b7, each commit the parent of the next.
- **Doc commit 92c6b38** touches only `specs/0043` and `ROADMAP.md`.
  - The committed blobs hash to the de2ec815... and 7ae37244... its message names.
  - The spec gains rulings M to S and "Planning verification of P3". The working spec (9c6c8dac..., equal to the working tree's copy) differs from HEAD's copy only by the added "Planning rulings during P4" (T and U).
  - ROADMAP: the P3 row is DONE, the Unscoped `<sheet>` bullet and the Unscoped charts bullet are gone, and P4 and Close stay PLANNED. This matches ruling S.
- **Test commit bb0a29e** adds one file, `crates/emend/src/package/tests.rs`, 33/0, and nothing else.
  - The one test is `a_strict_workbook_root_binding_no_relationships_prefix_takes_an_add`.
  - Its input is a `MAIN_STRICT` root whose existing `<sheet>` binds the strict relationships namespace under `q`, so the root binds no relationships prefix.
  - It asserts that the new `<sheet>` carries `xmlns:r="{ATTR_STRICT}" r:id="rId3"`, then reads back the names, the part and the kind. This is ruling R / F7.
  - `MAIN_STRICT` and `ATTR_STRICT` are existing constants (tests.rs lines 31-32).
- **README commit 37062b7** changes only `README.md`, 19/15.
  - The twelve feature costs match `records/size-m6p6-2026-10-24-7ac8b74.md` "The figures" exactly: dates 4840, rowcol 58260, metadata 49748, rules 48888, tables 46196, comments 63424, images 56956, richtext 13704, sheetprops 27092, read 113528, copy 13452, charts 166680.
  - The budget is 451528, and 458752 - 451528 = 7224, both as the record gives them.
  - The charts line names specs/0038 to specs/0041 beside 0034 to 0036.
  - A new core sentence names gradient fills and named styles with `specs/0042-gradient-fills-and-named-styles.md`. The size record's fullstyles probe text does describe a gradient and a named style.
  - The six Milestone 6 sample records each show host and i686 with 27/27 ok, lo ok 27 and lo_in ok 27, as the fidelity sentence states.
  - The speed sentence keeps the scale-m3p1 citation. Its tier figures match the compare record: c host 2.30 s at 7872 kB, i686 3.60 s at 7560 kB, 1159829654 bytes; d 201769244 bytes, 0.40 s on each label.
  - The comparison paragraph cites only `compare-m6p6-2026-10-24-d88f735.md`. Its figures match: 16 ops, every corpus row 27/27 with lo ok 27; roundtrip identical/dropped 870/0 for emend and 28/533 for openpyxl; median wall 0.10 for every emend host row and 0.81 for every openpyxl row; roundtrip peak RSS max 7992 and 674516; tier c openpyxl cap, exit 137, 290.83 s, 16554684 kB; all 68 emend scale rows "ok".
  - Every `records/` path the README cites (20) and every specs path checked exists at 37062b7.
  - "faster" and "better": 0 matches. U+2014 and non-ASCII: 0.
  - CHANGELOG.md is not in the range.
  - All three commit messages are ASCII and end with Co-Authored-By alone, with no Claude-Session line.

Doubt: the spec asks the comparison paragraph to state "per op or for the set what each tool left byte-identical, dropped and opened". The paragraph gives identical/dropped for the roundtrip only, and opened for the set. This is the Milestone 5 paragraph's form carried forward. I read it as within the spec, but planning may want to confirm it.

**Q2. Do the logs contain every line the spec requires? Verdict: yes.**

- The four logs in check1/logs are byte-identical to `<repo>/records/logs/m6p6-p4/`.
- Each log ends "<name> log checks all as expected: yes" and prints PLANNING_SPEC 9c6c8dac....
- No output line in the four logs ends UNEXPECTED. FINDING appears only in run.log's printed script comments (two matches, indented), and not at all in the scan pre-check.

Per log:
- **Gate:**
  - **Roles:** the four roles are read by subject and full hash, with the order checked (A88) and swap controls.
  - **Prune:** P3's target, "1 of 1; their bytes in all: 13541824503", and gone after it.
  - **Diff and tests:** numstat for the test and README commits; test names 1452 to 1453, the one new name and none lost.
  - **CHANGELOG and check.sh:** CHANGELOG is the base's blob (R88), and check.sh gives nine PASS lines.
  - **Sets:** off 539, ch 713, im 576, ci 755, cc 758, cr 820, all 1201 on host and i686.
  - **Size figures:** the charts probe 1151424, the format probe 984744 and the budget 451528, "judged nowhere".
- **Test:** ws 1450 on host and i686 against the previous 1449. The sets are off 539, read 646, rules 576, read,rules 686, all 1201 on both. The strict test and the transitional test each pass by name in every run, and no crate source names the README.
- **Negative:** "the 22 cases were refused, N1 to N7 and N9 to N23". N8 has 6 accepted runs, 4 active-tab refusals and 4 controls. The README control NC passes, and plants N1 to N10 are each refused by kind: N5 for the dash, N6 for an absolute path, N7 and N8 for the two comparatives, N2 for a missing path.
- **Run:**
  - **Target:** HEAD equals the phase commit.
  - **Printed commands:** each readme_check.py call and each record read is printed.
  - **Summary:** "sentences 45 with-digit 27 cited 27 uncited 0 missing 0 figure 0 number 0 feature 0 dash 0 abspath 0 word 0 figures-checked 28".
  - **Planted control:** the planted uncited sentence is refused (uncited 1).
  - **Records:** "the nineteen records" are read at the phase commit, and missing 0 is checked against the phase commit's tree.
  - **Number tokens:** 46 tokens, equal to the 46 figures checked.
  - **Content checks:** no U+2014, no absolute path, no corpus directory name, no denylist word.
- **Logs' README:** it states F8, its cause and the fix (ruling U).

Note: the run log reads 19 records for figures, while the README cites 20 paths. The twentieth, `records/toolchain-2026-09-15.md`, is cited in a sentence that holds no checked figure, and it is present at HEAD. This is consistent, not a gap.

**Q3. Does every number and quoted string in the draft report appear in a primary? Verdict: yes with exceptions.**

Every hash, count and quoted string in the draft matches a primary:
- **Git:** origin/main is ee78ffa; the commit hashes and subjects; the numstat.
- **The five logs:** their sha256 values.
- **The scan pre-check:** `<phase>/scan-pre/scan.log` has sha256 70a31694... and holds "total hits: 0", "structural checks failed: 0" and "RESULT: PASS".
- **F8:** the d1 strings "gate log checks all as expected: no", the UNEXPECTED check text, and "the new tests FACTS.md names: 7".
- **The two gate diffs:** gate-F8.diff shows a single changed line, and gate-gen-U.diff shows the concatenation check with d1's line as its planted control.
- **Gate, test, negative and run figures:** all as given under Q2.

Exceptions:
- (a) "Dry run d2 then passed all four steps" comes only from the logs' README, which lists it under "The session's statements (no log prints them)". No d2 log is among the primaries. The draft states it as fact in the F8 bullet without marking it as the session's statement.
- (b) "Nothing pushed (session's statement)" is consistent with git: origin/main is ee78ffa and main is 13 commits ahead. The statement itself is the session's.
- (c) "the gate's two regenerations" paraphrases the README's "The gate's generator was changed twice, its script once", where the first regeneration was byte-identical. The wording could be read as two script changes. It is not wrong, but it is looser than the primary.
- (d) "7 words" matches the README's "7 words" and the log's count of 7. Of the seven lines, one is a single space, so "words" is loose. Ruling U also says "seven junk words".
