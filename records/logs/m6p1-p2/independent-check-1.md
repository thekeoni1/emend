# M6 part 1 P2: independent check 1

A fresh subagent given only the working spec, the phase diff, the draft
report, the four logs, the scan pre-check, the logs' README and the four
records, in `<phase>/check1`. Its answer, verbatim:

---

Independent check of M6 part 1 P2 (specs/0038), base f948fd9 to phase commit 2a706dc. Every file cited is under the check directory, <home>/.cache/emend-build/m6p1-p2/check1. Line numbers are from those copies.

**1. Does the diff do what the spec says for P2 and nothing else? PARTLY.**

What matches the spec:
- **The files.** The diff touches six files and no others: ROADMAP.md, specs/0038-chartsheets.md, crates/emend/src/package.rs, crates/emend/src/package/tests/chartsheet.rs, scripts/openpyxl-ops-test.sh and scripts/openpyxl-ops.py. The per-commit split is printed in gate.log 7248-7275: the carry is chartsheet.rs alone, the op commit is the two scripts, and the limits commit is package.rs alone.
- **The doc commit (planning's).** It changes only specs/0038 (+111) and ROADMAP.md (+1/-1). The 111 added spec lines are rulings A to C and "Planning verification of P1" with rulings D to I (phase.diff 238-359). They are a verbatim substring of spec-0038-working.md, whose sha256 is 6ca55be2..., and ruling J is not committed. The ROADMAP hunk marks the P1 row DONE (phase.diff 9-10). Blob hashes are checked in gate.log 7097-7140.
- **The carry (ruling E).** phase.diff 94-150 adds `a_chartsheet_renamed_and_moved_in_the_write_that_added_it_is_written_so`. It adds a chartsheet, renames and moves it in the same write, checks the projected list (kind, sheetId 2), writes, and reads back both the sheet list (name, kind, part, sheetId) and `charts`. This is what ruling E asks for.
- **The op.** scripts/openpyxl-ops.py adds `chartsheet` to OPS and to the docstring, and runs create_chartsheet().add_chart of a BarChart over B2:B(M+1) and A2:A(M+1) (phase.diff 205-236). scripts/openpyxl-ops-test.sh adds the op to OPS, adds one parts check (3 sheets, the chartsheet part and relationship, one absoluteAnchor at 0,0 with ext 0 by 0, one bar chart), raises the check count from 12 to 13, and updates the header comment (phase.diff 159-197). This is what the spec asks for.
- **The limits.** phase.diff 29-66 adds the chartsheet limits block to the package header.

What falls outside the letter of the spec:
- **(a) A second header hunk.** phase.diff 18-25 (package.rs @@ -2088) rewrites the charts reading description: "A chartsheet is not read (P1 ruling B)" becomes a sentence saying a chartsheet's charts are read through its one drawing element. That is a correction to the reading text of the catalogue, not a limit. Ruling G says P2's "catalogue and limits" reads "the limits alone".
  - The report does not disclose this hunk. Finding 2 covers only the doc comment move, and finding 3 speaks of "the paragraph", singular, as if one hunk carried the change.
  - It is low risk, since it removes a statement P1 made false, but it is outside ruling G as written.
- **(b) The doc comment move.** phase.diff 74 and 82 put "The transitional worksheet content type, which the strict form shares." back on WORKSHEET_CONTENT_TYPE. This is outside P2's list too. The report discloses it (finding 2), and it is a doc-only fix of P1's misplacement.

Nothing else is outside the spec.

**2. Do the logs contain every line the spec requires for P2? YES.**

Run log, the openpyxl fixture:
- **Plan printed first:** run.log 5601.
- **Not root:** run.log 5603-5605 (uid 1001, with a control).
- **Rootless:** run.log 5604 and 5607.
- **Image id:** checked against records/openpyxl-image-2026-10-16.md at run.log 5610-5612, with a control.
- **Op exit and output:** exit 0, standard error 0 bytes, openpyxl=3.1.5 (run.log 5614-5617).
- **Ten structural checks:** all yes, with a control on the committed fixture (run.log 5733-5745).

Run log, the samples on the fixture (as ruling J reshapes them):
- **--charts-sample, both targets:** each row is ok with lo_in and lo ok (run.log 5786-5796). The line is the same on both targets and the output is byte-identical (5797-5798). The line reads sheets 3, charts 1, series 1, plots 1 a bar, anchor_abs 1, with a control (5799-5800). The input is written back with all 15 entries identical (5801-5803).
- **--chartsheet-sample, both targets:** exit 2 with no standard output. The one line "error: the workbook root binds no prefix for the relationships namespace" is checked against the NO_RELATIONSHIPS_PREFIX literal (run.log 5804-5817). There is no output file and no temporary (5818-5821), and there are controls.
- **lo and lo_in on fixture outputs:** the only fixture output is the charts-sample output, covered by run.log 5790 and 5796. The refused run writes no output.

Run log, the corpus rerun, records and open record:
- **The corpus run, both targets:** 27 of 27 ok, lo and lo_in ok 27 (run.log 5315 and 5394).
- **Byte-identical to P1's run by sha256:** 27 of 27 on each target (run.log 5585-5590).
- **Records:** m6p1-p2-2026-10-19-2a706dc.{md,txt} written and installed (run.log 5457-5494). Their sha256 values match the files in records/.
- **The open record:** 1474 pairs (run.log 7560-7630).
- **The keep against P1's record:** 1472 pairs unchanged, 2 new pairs (run.log 5870-5898).

Test log:
- **Feature sets:** off, charts, charts,images, charts,comments, charts,read and all, on host and i686 (test.log 1616-1688), plus the workspace runs on both targets (1570-1614).
- **The carry by name:** it passes in every charts set on both targets and does not run with features off (test.log 1775-1846).
- **The ops test in the image:** block O at test.log 1930-2046.

Rulings:
- **Ruling D:** each generator prints its own placeholder check with controls: gate.log 82-87, test.log 123-128, negative.log 72-79, run.log 100-106. The gate has two controls and no scrub-target control; the other three logs have three.
- **Ruling F:** no rerun took place. RERUN_NOTE and RERUN_FILES are unset in all four logs (gate.log 4-5, test.log 3-4, negative.log 3-4, run.log 8-9). Test, negative and run each print a control that RERUN_FILES must name a script (test.log 149, negative.log 100-102, run.log 143). The gate prints no such control. That is not required while there is no rerun.
- **Ruling J:** applied as above, plus N8 in negative.log 5894 and 5933-5934.

No required line is missing.

**3. Does every number and quoted string in report-draft.md appear in a primary? NO.**

These do not appear in a primary:
- **(i) The end time 08:33:51Z (report line 51).** negative.log 6000 reads "run ended at 2026-10-19T08:33:50Z". 08:33:51Z is the run log's start time (run.log 142). logs-README.md line 19 repeats the same wrong figure, so the README contradicts the log.
- **(ii) The path "<phase>/pre1/scan-pre-check.log" (report line 53).** No primary names this location. The sha256 3b1693fc... and the quoted lines "RESULT: PASS" and "structural checks failed: 0" do match the file.
- **(iii) "The commit message says so" (finding 2, line 124).** The body of 2a706dc's message is not printed in any primary. gate.log 7077-7079 gives only its line count (18), ASCII form and trailer. The statement is not marked as the session's statement.
- **(iv) Finding 5's claims (lines 133-136).** The statements that scripts/compare-run.sh and scripts/compare-record.py "name the comparison ops" and that "the chartsheet op is a fixture op, not a comparison" appear in no primary. Only "neither file is in the diff" is verifiable (phase.diff, and gate.log 7268-7274). The gate shows only that those scripts parse or compile (gate.log 7342 for compare-record.py). These claims are not marked as the session's statement.
- **(v) "N1 to N7 as P1's" (line 83), a soft case.** negative.log 5915-5928 shows N1 to N7 refused with their messages, labelled as the earlier phases' cases. No line compares them with P1's negative log.

Further notes, not failures:
- Line 8 says the gate, the run log and the scan received PLANNING_SPEC, and the scan received PLANNING_ROADMAP. The gate also received PLANNING_ROADMAP (gate.log 13 and 8651). The sentence is incomplete, not false.
- Finding 3's "the diff alone carries the paragraph" overlooks the second hunk at phase.diff 18-25 (see 1a).

Everything else checks out against a primary:
- **Hashes:** every sha256 matches the file or its README row. The spec hash 6ca55be2... matches the working spec, and the ROADMAP hash b0fd10df... matches gate.log 235 and scan-pre-check.log 93.
- **Commits:** the hashes and subjects match run.log 110-114 and the gate.
- **Numstat:** the table matches phase.diff when counted.
- **Test log figures:** 515, 635, 677, 680, 738, 1117 and 1343.
- **Gate figures:** probe 1086440, 1076124, 10316, difference 0 and the quoted phrase (gate.log 8458-8477), plus nine PASS lines (gate.log 7903).
- **Block O:** the id 894b239f..., 6 and 85 ok lines.
- **Corpus:** 789, 81, 135, 0, 0 and 27 of 27.
- **Fixture:** ten checks and the quoted error line.
- **Keep:** 2948 and 1474, 1472 + 0 = 1472, 2 new pairs, lo_in 1462 and lo 1463. The 12 and 11 failures are in the doc commit's spec text.
