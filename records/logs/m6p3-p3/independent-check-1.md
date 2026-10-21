# Independent check: M6 part 3 P3 (specs/0040), draft report and logs

## Hash checks

Each hash below was computed with `sha256sum` on the file in `check1/`.

| File | Computed | Primary that gives it | Match |
|---|---|---|---|
| diff.patch | 253f6e26...4443c4 | gate.log 8301+ section "each commit's diff and the phase's combined diff" (`git diff ... 65703ee... 0707236...`, "the phase's combined diff") | yes |
| spec-0040-working.md | 9f5bb018...e5b885 | PLANNING_SPEC: gate.log 12 and 286, test.log 10, negative.log 5, size.log 5 and 126, run.log 14 | yes |
| logs/gate.log | 92b43ad4... | report 55, README 29 | yes |
| logs/test.log | 8afe97d5... | report 56, README 30 | yes |
| logs/negative.log | 833a4b8d... | report 57, README 31 | yes |
| logs/size.log | ce2f829e... | report 58, README 32, and run.log (the "sha256sum <the size log>" line in its release-binaries section) | yes |
| logs/run.log | bf8b41ca... | report 59, README 33 | yes |
| logs/scan-pre.log | 096fc85b... | report 73 | yes |
| logs/README.md | 96393023... | no hash in the report or in the README itself; scan-pre.log 5404 gives it ("scanned records/logs/m6p3-p3/README.md (7462 bytes, sha256 963930235a80...)") | yes |
| records/size-m6p3-...md | 788acec2... | report 60, size.log ("installed: ... sha256 788acec2...") | yes |
| records/api-review-m6p3-...md | 9e13ce65... | report 61, test.log 4274 and 4280 | yes |
| records/m6p3-p3-chartaxis .md / .txt | e91d8fbc... / dac383f2... | report 62 and 63 | yes |
| records/m6p3-p3-charttrend .md / .txt | be5fe9fa... / 032df328... | report 64 and 65 | yes |
| records/m6p3-open .md / .txt | ad86f0e0... / 2cff9eca... | report 66 and 67. The .txt hash also equals P2's committed open .txt (run.log, the line "the record: records/m6p3-open-2026-10-21-3736eea.txt (sha256 2cff9eca...)"). That fits 0 new and 0 changed pairs. | yes |

The committed spec blob a02a2089... (report line 7) appears in gate.log 283, 92 and 104, and in commit-messages.txt 78.

---

## 1. Does the diff do what the spec says for P3 (as amended by its rulings) and nothing else?

**Answer: YES**

Evidence:

- **Files.** The diff touches five files: CHANGELOG.md, ROADMAP.md, specs/0040, tools/emend-copy/src/main.rs and the charts probe (diff.patch 1, 16, 29, 247, 273; gate.log "the phase diff holds five files"). Nothing is under crates/ or scripts/, and no manifest or lock file is touched (gate.log section "each code commit touches its own files": "Rust sources in the phase diff: 2; files under crates/ ... 0", "files under scripts/ changed in the range: 0", "Cargo.lock ... 0; manifests ... 0"). The negative and test logs back this: no new message and no new NewPlot variant (test.log 4058-4059, 4076-4077; negative.log 7008-7011).
- **Doc commit.** The 207 lines it adds to the spec are exactly lines 627-833 of the working spec: the rulings J to S. I diffed the "+" lines of diff.patch 33-246 against spec lines 627-833 and they are identical. Rulings T and U (spec 834-860) are absent from the diff, as expected, since they are uncommitted. The ROADMAP change marks P2's row DONE (diff.patch 23-24), which is planning's text, consistent with the spec's statement that planning's docs are committed "as planning left them".
- **Carry (ruling Q, spec 815-819).** diff.patch 251-271 rewrites the module doc entry so it names "the line's and the scatter's against A2:A(L) and the column's without categories (ruling P)". Every changed line is a `//!` line (gate.log "the carry commit's change to main.rs is module doc lines alone ... as-expected").
- **Probe ("The probe", spec 394-413, with rulings L and T):**
  - PLOTS is unchanged (gate.log fourteen-line count: `const PLOTS: [NewPlot; 16]` is 1 at both the base and the phase commit).
  - TRENDLINE is Linear, show_r_squared true, forward 1 (diff.patch 657-665).
  - ERROR_BARS is Fixed 1, Both, end_cap true. XY_ERROR_BARS adds direction Y, used for both scatter and bubble, matching ruling T (2) (diff.patch 668-678, 807-813).
  - The analytic set is Bar, Line, Scatter, Area and Bubble, with no 3D kind, matching decision 1 A (diff.patch 757-765).
  - The combined line's series takes the trendline and error bars, matching ruling T (1) (diff.patch 830-831).
  - The plot lines are drop lines on Line, Area and Stock; high-low lines on Line and Stock; up-down bars on Line and Stock. That puts high_low_lines true on Stock, as ruling L requires (diff.patch 520-527).
  - VALUE_AXIS (min 0, max 100, units 25 and 5, reversed) applies on every kind except the round ones (diff.patch 698-705, 528).
  - SKIPPING_AXIS goes on Bar, Bar3D, Line3D, Area and Area3D, matching ruling T (3) (diff.patch 873-878).
  - DATE_AXIS (Days, major 1 Days) goes on Line alone, matching ruling L (diff.patch 872, 711-718).
  - LOG_AXIS (log base 10) goes on a scatter's x axis (diff.patch 879).
  - The header is one description with the index range 0 to 15, and the layered "second set" and "third set" paragraphs are removed (diff.patch 277-325; gate.log counts "index 0 to 12" 0 and "//! The third set (" 0 at the phase commit).
  - The chartsheet line is unchanged (gate.log `chartsheet_charts {}` count 1).
- **CHANGELOG.** One four-line bullet under "### Added" cites `specs/0040-analytic-chart-elements.md` (diff.patch 9-12; gate.log "the bullet lies under the "### Added" heading ... as-expected").
- **Commit order.** The ruling T commit is its own commit after 63c9af9, with the subject ruling T names (spec 851-854; commit-messages.txt 1-2; gate.log "roles_in_order").

Nothing outside the spec and its rulings was found in the diff.

---

## 2. Do the logs contain every line the spec requires for P3?

**Answer: YES** (one item is pending by design, see below)

The required lines and where they are:

- **Size log and size record (spec 464-472, "Size and packaging" 29-66, ruling U 856-860):**
  - The charts cost is 135784, its movement over 125060 is 10724, judged in words against the estimate and the 31000 line: size.log 10129 and 10228 ("grew 10724, below the estimate of 16500 to 26000 and under the finding line of 31000, told and not a finding"); record lines 136-142 and 194.
  - The arms the probe reaches are named: record 144-154.
  - Ruling U's note appears once: record 155, size.log 10093.
  - The images and comments cost movements are judged under their own names: size.log 10130-10131 and 10229-10230; record 204.
  - The two probes' own growth is printed and not judged: size.log "printed: probe-emend-comments-write-xml ... not judged".
  - The core's growth is 0 and judged: record 196-198, with "the record states the core's growth as a fact: yes" in size.log.
  - Findings: None (record 209; size.log 10240 "0").
  - The record's charts probe row is one description, 0 to 15: record 87, size.log 12700.
  - Kinds 1 to 15 exit 0 with their chart_series, and index 16 is refused with exit 2: size.log 12208-12244.
  - The record is installed at its hash (size.log "the size record, installed").
- **API listing record (spec 472-474):**
  - Items 815; 40 new and 1 gone; every verdict keep: test.log 3969, 4019-4020, 4272.
  - The row that lost Eq is marked: test.log 4063-4064 and 4074-4075; api-review record 80-84 ("## Rows that keep their key and lost Eq", NewSeries) and 694 ("keep; Eq dropped (specs/0040 "P3")").
- **CHANGELOG bullet:** gate.log 313-321 prints the four added lines and checks them under Added.
- **Negative log through the binary:**
  - The 13 cases are refused: negative.log 8972.
  - It states that P3 adds no message and no binary path: 8148.
  - The constant set equals the base's (132 each): 7008-7011.
- **Run log:**
  - The four release binaries are copied and checked against size.log's hash lines, 4 of 4 (run.log 6357-6383; size.log 12261-12264).
  - chartaxis and charttrend on host and i686 are byte-identical to P2's runs by sha256, 27 of 27 each, 4 of 4 (run.log 6934-7115).
  - The openpyxl fixture is rewritten by the pinned image. Its six sample lines and twelve field lines equal P2's run.log, byte for byte ("fixture: the six lines are P2's (6 of 6)", "the twelve field lines ... are P2's"). P2's kept fixture through the three samples is byte-identical, 6 of 6 (run.log 7116-7671).
  - The three record pairs are installed (run.log 10131-10133).
- **Open record of the keep:**
  - Keep: passed 1386, 2121 pairs, 0 in the keep only, 0 in the record only, 0 whose bytes differ, 0 tests added or edited (run.log 7672-7720 onward).
  - edit-open-check exits 0 with findings 0, and the record is installed (run.log 9955-9957, 10120-10128).
- **Test log:** 1386 on both targets; the six feature sets 515, 666, 708, 711, 769 and 1148 on both targets (test.log 2648-2745).
- **Gate:** every expected section is present, and the gate states that no self-test is required because no gate file changed (gate.log section "the gate files this phase changed").

Notes, none of them a gap in the logs:

- **scan.log is pending.** The spec lists `scan.log` among the logs (spec 426). Only the scan's pre-check is here, ending "RESULT: PASS" with "structural checks failed: 0" (scan-pre.log 5662-5664). The README (line 34) says scan.log is "written last, by the scan". It is pending by design, not missing.
- **The openpyxl op's output differs from P2's.** Its hash is 3817dccf... now against 70a3d4fd... in P2's run (run.log 7116 section and 7616 section), so the fixture's own bytes are not byte-identical. The spec asks for "the openpyxl fixture by part 1's P3 reading, `specs/0038` ruling P". I cannot read ruling P's text from these files. The log's form compares the lines, the field lines, and P2's kept fixture by sha256. Whether that is the reading ruling P intends cannot be decided here.
- **gate.log 268 is ambiguous but consistent.** It names c566a01 as "part 3 P1's crate commit". In the API record c566a01's rows cite specs/0034 (api-review 591-599), so this is Milestone 5 part 3's commit that added the feature, not M6 part 3 P1's (12f1314, test.log 4023). That is consistent with README line 90, but the label names no milestone.

---

## 3. Does every number and quoted string in report-draft.md appear in a primary?

**Answer: PARTLY**

Every number and quoted string I checked is in a primary:

- **Hashes:** every hash in report lines 5, 7, 18-22 and 55-73.
- **Times:** the five "run ended" times (gate.log 9980; test.log 4298; negative.log 9055; size.log 12729; run.log 10155).
- **Gate figures:**
  - commits 5, split 1 and 4;
  - distinct messages 154 at both commits;
  - test names 1389 at both, new tests 0 (gate.log 8611, 8617);
  - probe diff 147 lines added and 35 removed (gate.log 9457);
  - the probe figures 1107960, 884, 1107076, 10724 and 1097236 (gate.log 9464-9470).
- **Test and API figures:** 1386 and the six set figures; 815 items, 40 new, 1 gone, NewSeries.
- **Size figures:** the chart_series list "2 1 2 3 2 2 2 3 2 2 1 2 2 2 1" (size.log 12214-12242); index 0's 3 and index 16's exit 2; 135784 = 1107960 over 972176 (record 136-137); 125060; 10724.
- **Quoted strings:** both quoted size-record strings are verbatim (record 141 and 194; record 155).
- **Run figures:** 27 of 27, 4 of 4, 6 of 6, 2121, 0/0/0.
- **numstat table (report 26-32):** every figure is in gate.log's per-file "added lines" and "lines removed" lines, and in diff.patch.
- **Session's statements:** c566a01 and 12f1314 (README 90; test.log 4023). The base subject in report line 14 (gate.log, the role-subject list).

Disagreements and unmarked statements:

1. **Report line 10, "Each logged log is the first of its kind."** It is not marked as the session's statement. The README marks the same claim as the session's (README 42-43). The logs show RERUN_NOTE and RERUN_FILES unset (gate.log 4-5, test.log 3-4, negative.log 3-4, size.log 6-7, run.log 7-8), which is consistent with it but does not prove it. It should carry "(session's statement)".
2. **Report line 70, "The logs ran serially".** It is supported in part: each log's SERIALIZED line ("no other log run was in flight") and its start time following the previous log's end. Fine as written, but the ordering itself is the session's account.
3. **Report line 92, "cases, counts and messages as P2's".** negative.log does not compare against P2's committed negative.log. It shows the constant set equal to the base's (7008-7011), states that every statement is part 3 P2's read again and every case carried (8148), and refuses 13 cases (8972). "as P2's" for counts goes beyond what the log prints. Suggested wording: "its 13 cases refused, its 132 message constants the base's, every case carried".
4. **Report lines 105-107 omit a run-log result.** They cover P2's kept fixture (6 of 6) but leave out that the fixture rewritten in this phase gave P2's six lines and twelve field lines byte for byte (run.log, the openpyxl fixture section). This is not a false number, but the report does not state a result the spec requires; the README does (README 75).
5. **Report lines 115-116, the facts-file error.** The report's account rests on the README alone (session's). gate.log 268 also names c566a01, correctly as M5 part 3's crate commit; see question 2. The phrase "before any script used it" refers to the M6 P1 sense only, and the report might say so to forestall confusion.
6. **Report lines 69-72, "outer standard error 0 bytes".** It is correctly marked as the session's statement. Each log separately prints its own script's stderr as 0 bytes (e.g. test.log 4295).
7. **Report line 121, the independent check.** It is a placeholder, as expected.

No number in the report contradicts a primary.
