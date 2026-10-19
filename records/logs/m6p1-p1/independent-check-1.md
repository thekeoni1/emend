# Independent check 1

Implementer's header (not the checker's): a fresh general-purpose subagent, given only the files under <phase>/check1/, answered the three CLAUDE.md questions on these logs at the phase commit f917618, before the record commit. Its inputs:

| File | sha256 |
|---|---|
| gate.log | 42f4e9a7fc2fec7b24fcbd4bdecd62fb9d175d44d169a5a91e2c8641aa2b11e9 |
| logs-README.md | 5ab2c4c1fb898af7a52bee9903539256a86d2d04dd88263acc6383035628abb0 |
| negative.log | a18805e60bd77d55a00291535f2da6541a1214a367d2df84403ddcf122b8d992 |
| phase.diff | 734531b16faa541fd7198d0ef55cc28ea14d8980534191a563d5ae809fcbf81a |
| records/m6p1-open-2026-10-19-f917618.md | 3725a2ecb08b49280c26e78263377f8811d33aa59999b9775119be783197393f |
| records/m6p1-p1-2026-10-19-f917618.md | 38d4cec57fe8fd180f7200cf766ccd9ed9c2321009c4c42161626b99a5cb4b4f |
| report-draft.md | f9fc8ca9ff5db750201776ef70887ba4595c8f2b7dc989b9f60dc327172c1a24 |
| run.log | c89128302725ecb86f65cc9e3062b4cd2e4ae40e36c833dffca3ac3253ffdaf8 |
| scan-pre-check.log | e82d3800059d31b00b658cb0c338616829c785b2b7431bd223ca8bb116c6f784 |
| spec-0038-working.md | 29536c9203415cbd01665a3dc0a7563ed2c38d3f45eef1289a8ca7ef1c880cb2 |
| test.log | 3941559d54b712a61afcf41145e3dba35d6aa8bc4e3c0ddedca06e79cf89d2e3 |

The answer, verbatim (sha256 98418aa9411f8b1a75f6720792ffb5f1a9595b9ccc48434a2c89142046f9fee4):

**Independent check 1, M6 part 1 P1 (specs/0038 chartsheets)**

This check read only the files under check1/. The sha256 of each of check1/gate.log, test.log, negative.log, run.log, records/m6p1-p1-2026-10-19-f917618.md and records/m6p1-open-2026-10-19-f917618.md matches what report-draft.md and logs-README.md give. I recomputed the numstat table in report-draft.md:22-36 from check1/phase.diff, and all 11 rows match.

**1. Does the diff do what the spec says for P1, and nothing else? PARTLY.**

The diff carries P1's whole list (check1/spec-0038-working.md:323-341):
- the root rule and `CHARTSHEET_CHILDREN` with fourteen names (phase.diff:494-520, 541-609);
- `charts` on a chartsheet (phase.diff:634);
- the blank chartsheet form, decision 3 A, with no pageMargins (phase.diff:128);
- `SheetEdit::Add` with a kind (phase.diff:86-115);
- `add_chartsheet` (phase.diff:334-458, 721-723);
- `add_chart` and `remove_chart` on a chartsheet: anchor rule, one-chart rule, drawing kept when emptied (phase.diff:203-237, 270-276);
- the three messages, each with its exact spec text (phase.diff:646-656);
- `--chartsheet-sample` (phase.diff:2360-2506).

Ruling A is honoured: remove and add in one write is refused with NO_CHART_AT (phase.diff:1564-1572). Ruling B is honoured: the existing size message is reused (phase.diff:716-718). Ruling C is honoured: chart calls on a chartsheet added in the same write give "no such part" (phase.diff:1294-1306). Every test P1 lists is present in tests/chartsheet.rs (phase.diff:1088-1688), plus the two tool tests (phase.diff:2533-2607). The doc commit holds exactly specs/0038, specs/0037 and ROADMAP.md (gate.log:7317).

Gaps:
1. The spec says a chartsheet added in a write "cannot be removed, renamed or moved in the same write", with the rule to be "named in P1's report" (spec-0038-working.md:205-208). The diff allows removing it: it drops the carried parts (phase.diff:463-486), and the test `a_chartsheet_removed_in_the_write_that_added_it_leaves_nothing` passes (phase.diff:1281-1292). No ruling covers this. Nothing in the diff shows or tests what happens on a rename or move in that write.
2. The diff adds the three messages to the `package` header catalogue (phase.diff:67-82). The spec gives "the package header's catalogue and limits" to P2 (spec-0038-working.md:354-356). This is early, not wrong, and the report does not name it.
3. Changes beyond the spec, which the report discloses as choices (report-draft.md:122-132):
   - the `kept_charts` key on the sample line (phase.diff:2227-2228, 2436);
   - `--charts-sample` now reads chartsheets too (phase.diff:2520). The spec's fixture run needs this.
4. Observation, not a disagreement: on a strict workbook, `add_chartsheet` writes strict relationship types but writes the chartsheet, drawing and chart in transitional XML (phase.diff:128, 374-380). That follows the spec ("as the worksheet template declares them", spec-0038-working.md:189). The strict test checks only the relationships (phase.diff:1213-1250).
5. The sample builds its chart in its own code (phase.diff:2382-2420) rather than calling add_chart_sample. Whether that chart is the same as the add-chart sample's ("below its last row", spec-0038-working.md:251-253) cannot be told from this diff.

**2. Do the logs contain every line the spec requires of P1's logs? YES, with notes.**

Gate:
- The prunes of the five M5 part 6 roots, each listed with du -sb and removed with rm -r (gate.log:212-481).
- check.sh with nine PASS lines (gate.log:8222-8235).
- Clippy and doc for each feature set (gate.log:8255-8402).
- The test figures (gate.log:8403-8773).
- The probe figure, printed and not judged (gate.log:8774-8797).
- The defined-helper checks under rulings Z, AD and AK a, with a count that is not 0 (gate.log:82-168).

Test log:
- Both targets, for off, charts, charts,images, charts,comments, charts,read and every feature (test.log:1364-1436).
- New tests by name (test.log:1493-1563).
- Clippy and rustdoc (test.log:1594-1645).

Negative log:
- N6, a chartsheet relationship with no part, refused with "no such part", with B4 as its control (negative.log:5620-5632, 5684-5685).

Run log:
- The corpus on both targets (run.log:5031-5207).
- The record pair (run.log:5209-5246). Changed 81 is 27 times 3 and added 135 is 27 times 5; lo and lo_in are ok on 27 of 27 (records/m6p1-p1-2026-10-19-f917618.md:18-19, 48).
- The Excel-form fixture, with its program printed, `--charts-sample` (count 1, series 1) and `--chartsheet-sample` (sheet count 3, the first chartsheet identical, the ElementTree checks) on both targets (run.log:5248-5460).
- The keep and the open record (run.log:5462-7381).
- The rerun asserting its RERUN_FILES with a check line, under ruling AK b (run.log:139-140).

Notes:
6. scan.log is not among the files checked; it is written last.
7. Ruling AA asks that every superseded attempt be "hashed by RERUN_FILES" (spec-0038-working.md:318). RERUN_FILES hashes the first run log, its stderr and its pre-check (run.log:8, 135-138). It does not hash the first run's records or its run script; the moved run directory and keep are hashed separately (run.log:145-153). The failed dry test and dry negative attempts are hashed only in logs-README.md:58-66, and test.log:3-4 and negative.log:3-4 show RERUN_NOTE unset. Planning should rule whether dry attempts and the first run's records fall under ruling AA.
8. logs-README.md:43-45 says the gate's defined-helper lines read the run script as it was before NWZ. The rerun script was checked only by its own log (run.log:44-50). The report does not say this.

**3. Does every number and quoted string in report-draft.md appear in a primary? PARTLY.**

All of these match their primaries:
- the commit hashes and subjects (gate.log:539-546, 7241);
- the logs' hashes and last lines;
- the end times (gate.log:8979, test.log:1659, negative.log:5733, run.log:7405);
- the prunes 5140700796, 5072230849 and 2005737121 (gate.log:259, 427, 476);
- the test figures, gate and test (gate.log:8418; test.log:1369-1435, 1455);
- the probe figures 1086440, 10316, 1076124 and the quoted phrase (gate.log:8794);
- the corpus figures 789, 81, 135 (records/m6p1-p1-2026-10-19-f917618.md:48);
- the keep figures 2944, 35, 0, 0 (run.log:5506, 5518-5519);
- the open record figures 1472, 1460, 1461 (records/m6p1-open-2026-10-19-f917618.md:14);
- the fixture's sheet2.xml, ten checks, and 9144000 by 6096000 (run.log:5446-5455);
- N6 and N7 (negative.log:5627-5639, 5686-5689);
- the first run log hash 143e8e9a... and the first pre-check hash 6e0e7a31... (run.log:136-138);
- PLANNING_SPEC 29536c92... (gate.log:12, run.log:15).

Gaps:
9. report-draft.md:105 gives the first run script's sha256 as 335c74f5314a975476ddd49652a3fb58081ff2652f8914bbc380f442a0c952eb. No primary contains it, and it is not marked as a session's statement.
10. report-draft.md:99-100 says the widened check has "a control on the old fixture part" that "refused "dir id"". No primary contains this control. run.log:322-325 describes the widened check but shows no such control.
11. report-draft.md:104-106 says the first records are kept in `<phase>/superseded/run-nwz-1`. No primary hashes or lists them. RERUN_FILES names only the three files (run.log:8).
12. report-draft.md:128 gives "Removing a pending chartsheet drops its carried parts" as an implementer's choice. It does not say that this departs from spec-0038-working.md:205-208, which says such a chartsheet "cannot be removed". It also does not name the rule for a rename or move in the same write, which that spec passage asks P1's report to name (see item 1).
13. The session's statements checked do not contradict a primary. Note on finding 2 (report-draft.md:113): "check.sh covers every-feature only" holds for check.sh, but the gate itself runs clippy and doc for each feature set (gate.log:8255-8402) and the dry failure was in the test log. Both statements can be true.
