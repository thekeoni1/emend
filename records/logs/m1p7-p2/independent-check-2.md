# Independent check 2, M1 part 7 P2

Checked commit: ea37ad786edf2798ced1e603c78aa136289db212 (the phase commit; HEAD when the check ran).
Checker: a fresh general-purpose subagent given only the files below in <scratch>/p2/check2 (written <check> in the answer), not this session's conversation. This is the phase's last check; no log was rerun after it.
Inputs, sha256 as hashed before the check (<scratch>/p2/check2-inputs.sha256):
  84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f  0001-bootstrap.md
  56db12e066acf4f5b10bb4cba30f6e17e101cf78b444a8b741f33c9a26e128fe  0014-xml-comparison.md
  6509a53d6643076212aac9937886a1091904127865cc5242dd4fb790c16c8f1a  diff-code-04ba408.patch
  52cffee05203607887665c631f1a4dfc57742c475b26277f0f23dddd41391933  diff-code-bacffa3.patch
  1605daa690b1e19ed8bbe913f800c0f5bf714a4608e28d8617027bb903e9228d  diff-code-ea37ad7.patch
  7ceb33f6bb8780d0dcc3653a3cc952672126923886d0d6182dbce2ae3424826b  diff-commands.txt
  e61f238f2ae393bc54d1f659cd02f5cef1482270cc8f09149733f10023acd372  diff-phase.patch
  722d6715a4fcccf6e7afcd8a542bee4d97bbb67e74588dd2b01c67b0ed53a314  gate.log
  f9099f92c288fa0e111026898214c931be4abdaa77e1a05c0b06a3b51932510f  independent-check-1.md
  5141d65dfdfd9f4aea0ab81f52feab88632ca690c6fcece7ee098d4a0ffed091  m1p7-p2-2026-09-24-ea37ad7.md
  56c311c0bf5df100f8afae508bd8608302d3d05bd934f545afc47c376e79cce4  m1p7-p2-2026-09-24-ea37ad7.txt
  b5c9f8c398771d2854fc04a59ad1af9bb9df7349f43e3e403ed6b480b4850d5f  negative.log
  a05a5fcf78eb5e4d66dbee2597e001987fe540acbb3fe5fcdabf73de8576c0ab  report-draft.md
  63d575948d11fe4dc935f9f7808c11e61ae222f7f951f9579ff8cecdb85fbc34  run.log
  64c4a3693f2e62d4ff92e09d95d4cd9ad669d512449cab2a66b639c5b66e6632  scan-for-checkers.log
  e73baaefb20ccf615f4a76fd7b6f8ddaf4e02b618973eacf40cce0754f362e59  test.log
The answer below is the checker's final message, extracted from its transcript, byte count 13107, sha256 1eb32e5277c861d3b96cf565a7ab79cfeffdc8f2ae9620cb09ca9f962ef07797; it is all ASCII and holds no machine path, so it is unaltered.

---

**Independent check 2: M1 part 7 P2 (phase commit ea37ad7, range 5fa6c8f..ea37ad7)**

I worked only from the files in `<check>` and did not run cargo, a build or a test. CLAUDE.md was in my context through the harness; I do not use it as evidence.

Before relying on any file, I hashed every file in `<check>`:
- The four diffs match gate.log lines 1217-1229: phase e61f238f, bacffa3 52cffee0, 04ba408 6509a53d, ea37ad7 1605daa6.
- The spec matches gate.log line 1234 (56db12e0, 24952 bytes).
- The four logs, the record pair and independent-check-1.md match scan-for-checkers.log lines 1294-1362 and report-draft.md line 14.
- The answer inside independent-check-1.md, taken after its `---` line without the leading newline, is 11788 bytes with sha256 4b86bf53, as its header line 22 says.

---

**Question 1: Does the diff do what the spec says for this phase, and nothing else?**

Yes, with the notes below. Which diff I read for each part:

- **4cb3c6a (doc commit).** There is no separate diff file for it. I read its effect in diff-phase.patch lines 1-12 (ROADMAP.md) and 202-305 (the spec). gate.log lines 1187-1215 show both files at the hashes the script calls planning's, at the doc commit and at the phase commit, and show that the commit carries exactly those two files. This satisfies ruling 8. The spec hunk is planning's ruling-1 text (diff-phase.patch lines 205-215).
- **bacffa3 (read in diff-code-bacffa3.patch).** Ruling 6's two fixtures are added:
  - `an_attribute_cr_lf_against_an_lf_compares_equal` (values 1 1 0 0);
  - `a_utf16_entry_is_a_parse_error` (parse `ok error 0 0 0`, archive 1 1 0 0, exit 1, empty stderr).

  The Limits line "reads UTF-8 only" is added to xml.rs. Nothing else. gate.log lines 1279-1284 show the commit touches only tools/xlsx-diff.
- **04ba408 (read in diff-code-04ba408.patch).** The .txt gains the third table: xml rows prefixed by id and label, padded to five columns (lines 28-30 and 135-138). The .md gains a table per label, "XML comparison of changed entries", by family, with compared, parse_errors, names_equal, values_equal, in_only and out_only (lines 158-168). Both are what "Harness runs" asks for.

  The xml_tally logic (lines 86-111) counts names_equal and values_equal only on a values row, so a parse-error entry counts only in parse_errors. That is ruling 3.

  It also makes changes the spec does not name, each a direct consequence of the spec's changes:
  - the docstring;
  - the .md intro sentence;
  - an "xml rows" count on stdout;
  - xml rows are read only for rows with outcome ok and diff_exit 0 or 1 (lines 122-126).

  gate.log lines 1286-1297 show no runner, Cargo.lock or manifest change.
- **ea37ad7 (read in diff-code-ea37ad7.patch).** A two-line refusal of a repeated xml archive row. The spec does not name this commit. It is a defect fix inside the record script this phase adds, and nothing else.

**G1 (note).** Check 1's F1 is still open, as the report says (report line 54). xml_tally assumes each entry's rows start with a parse row. A values row before any parse row would raise a Python exception, not a Failure refusal. The spec asks for no order check.

**G2 (note).** Report line 9 and the 04ba408 message say the script "reads each compared workbook's" diff file. The code reads the diff file of every workbook whose outcome is ok and whose diff_exit is 0 or 1 (diff-code-04ba408.patch lines 124-126). That includes the 8 corpus workbooks with no compared entry. The message states the filter; the report does not.

---

**Question 2: Do the logs contain every line the spec and 0001 require?**

Present and verified:
- The command, invocation and about lines, in that order, in all four logs.
- The phase-commit guard with controls in all four; run.log lines 4-11 add a fabricated-commit control.
- SERIALIZED lines with process counts in all four.
- A root line for every placeholder root each log prints:
  - gate.log lines 55-58;
  - test.log lines 49-53;
  - negative.log lines 43-46 (now with `<repo>`);
  - run.log lines 54-57.

  I listed every `<...>` placeholder per log. Outside printed script text, no root is used without a root line.
- Gate: nine PASS lines (gate.log lines 1447-1471), the gate-file listing with no self-test (lines 1239-1276), and the dependency tree and licences (lines 1307-1360).
- test.log on both targets:
  - 510 passed, 0 failed, 2 ignored (lines 1881 and 2439);
  - p7 17 on each and value_hash_is_pinned on both (lines 2525-2554);
  - the ruling 7 case map (lines 2563-2568 and 2626-2627);
  - M1 to M12 (lines 3458-3474).
- negative.log:
  - family A, 12 cases on both targets (lines 1370-2260);
  - family B: B1 (lines 2564-2863), R1 to R8 (lines 2864-2951), B3m (lines 2953-2965);
  - the main-tree check (lines 2967-2971).
- run.log:
  - release builds of the phase commit on both targets (lines 1648-1692);
  - superseded run directories removed and printed (lines 1694-1713);
  - the per-workbook derivation with a plant control per flag on both labels (lines 1768-1829 and 1865-1926);
  - labels agree and outputs are byte-identical to part 6 P3's (lines 1927-1936);
  - entry statuses compared with part 6 P3's (lines 1964-1968).
- The scan: scan-for-checkers.log ends "RESULT: PASS" and "structural checks failed: 0", with the one check answer scanned (lines 1338-1350).

**G3 (gap). Superseded-attempt hashes typed, not printed by a command.** 0001 line 185 says every sha256 a log cites, a superseded attempt's included, is printed by a command. The rerun lines come from a typed environment variable (RERUN_NOTE; gate.log lines 275-276). Five hashes cited there are not printed by any command in any file in `<check>`:
- 63ef5c36 (gate.log line 49);
- 0618829d and f0d1495e (test.log line 43);
- ab68677f and 09cb2c7a (negative.log line 36).

The commands under those lines claim more than they hash:
- gate.log line 50 and test.log line 44: "over each superseded attempt named above";
- negative.log line 37: "the hashes it points at".

They hash only the ea37ad7 attempt, its script and the attempt-1 file.

**G4 (gap). Two run.log record checks test less than they claim.**
- "the .txt's third table holds every xml row of the diff files" (run.log line 1960) compares only row counts, 524 against 524 (script lines 776-804), not content.
- The .md check (run.log lines 1954-1957) compares only the total row. So the per-family rows the report quotes (content-types 3, rels 3, workbook 19, worksheet 19) are not checked against the diff files by any corpus log.

Mitigations:
- negative.log B1 compares full content on fixtures (lines 2847-2851).
- My own count over the .txt matches run.log's tool_rows on all 54 workbook-label lines.
- My own totals over the .txt give 104 and 146 per label.

**G5 (note). Stale text remains in test.log, the same class as check 1's F9.**
- Script comments at lines 75 and 89 still say "the two code commits".
- Printed line 3542 says "(its case B3, to be rerun there as a refusal case)". B3 is now R8, and the measurement is B3m.

run.log's header items are also numbered 9 before 8 (lines 135-139).

**G6 (note). negative.log's `<repo>` root line contradicts itself.** Line 46 says "the shared counting library sourced from it ... nothing in it is run". The script cds to the top level and sources `scripts/counting.sh` from the main tree (lines 148-150), which runs code from it.

Ruling 2 covers "the built binary on fixtures in a made scratch directory". Family B runs a git-show copy of a Python script (line 2264), which goes beyond that wording. Planning may confirm.

**G7 (note). Part of ea37ad7's message is not measured.** The message says the old script accepted the repeated row "keeping the last" (test.log lines 3531-3534). B3m prints only exit 0, stderr bytes 0 and files written 2 (negative.log lines 2961-2962). Which copy is kept is not measured, and with equal values it cannot be observed. The acceptance itself is now printed.

**G8 (note). test.log's attempt 1 of the rerun is not hashed.** It is named as "an empty test.log" with only its stderr hashed (line 43 and line 47). 0001 lines 188-189 ask that an attempt not kept be named as having no hash, and why. The empty log is neither hashed nor named that way.

Status of check 1's findings that the report says were fixed (report lines 47-53):
- **F3: fixed.** negative.log line 46.
- **F4: fixed.** run.log lines 1694-1713: 42 directories, 1 superseded, removed, part 6 P3's directory still stands.
- **F5 and F15: fixed in substance.** negative.log lines 2953-2963, with G7's caveat.
- **F6: fixed.** The script is run from its git-show copy (negative.log lines 2263-2265). The closing check compares status before and after (lines 2967-2971). G6's caveat applies.
- **F7: fixed.** run.log line 1935 no longer ends in a colon.
- **F9: fixed for the lines check 1 named, but not complete.**
  - gate.log line 93 now says m1p7-p1.
  - gate.log lines 99-101 say "three since item 9".
  - test.log lines 806-807 name part 7 P1's second code commit.
  - G5 lists the stale text that remains.
- **F10 to F14 (report wording): fixed.**
  - Line 41 now says "single-attribute-change fixture".
  - Rulings 1 to 8 are listed (lines 35-42).
  - "whose checks held" and "the UTC date had turned" are gone.
  - Line 19 says "N0 to N7b and N10 to N12".

---

**Question 3: Does every number and quoted string in report-draft.md appear in a primary?**

Verified against primaries:
- **Commit ids and subjects:** gate.log lines 1140-1146.
- **Commit message content** of bacffa3, 04ba408 and ea37ad7: test.log lines 3476-3535.
- **Line 14 hashes:** all match my sha256sum and scan-for-checkers.log.
- **Line 18:**
  - 510/0/2 on both targets: test.log lines 1881 and 2439;
  - p7 17 on each: line 2615;
  - value_hash_is_pinned on both: line 2621;
  - M1 to M12, with M12 as "hashed as written, not through normalized_value": line 796;
  - M9 bites on i686 only: lines 3467-3468;
  - ruling 8's probe: line 2643.
- **Line 19:** 12 cases (negative.log line 2247), eight refusals and six message forms (lines 2864-2890), B3m.
- **Line 20:**
  - run.log removal: lines 1694-1713;
  - byte-identical to the derivation, 27 of 27: line 1797;
  - statuses: line 1967;
  - outputs identical to part 6 P3's: line 1934;
  - host and i686 identical: line 1928.
- **Line 25:** ef3e93f4, 85d4f3bd, aabd5001, cc6846dc, bec8f62d and 11a9b69c are printed by commands (gate.log line 51, test.log line 45, negative.log line 38, run.log lines 48-50). 63ef5c36, 0618829d and ab68677f appear only as typed text (G3).
- **Line 27:** 627fac9c (gate.log line 53), 38452b33 (test.log line 47), f3843752 (negative.log line 40).
- **Line 31:**
  - 27 workbooks with outcome, lo and lo_in ok: record .md lines 17-18;
  - 44 0 0 0 104 146: run.log lines 1954 and 1956, record .md lines 86 and 96;
  - families 3, 3, 19, 19: record .md lines 82-85.
- **Line 61:** run.log line 42 prints run-at 2026-09-24T00:39:02Z; the run directory is 2026-09-24-ea37ad7.

Statements that are the author's inference, or that no file in `<check>` supports:
- **G9 (note).** Line 27, "(the same bytes as the kept copies)", appears only in the typed rerun note (gate.log line 49). No command compares the removed files with the kept copies.
- **G10 (note).** Line 27, "run.log ran once in the rerun, between them", is inference. The primaries support it: run.log ran 00:39:02-00:46:38Z, before the final gate at 00:47:09Z, and run.log line 1650 counts 3 phase files in the tree. But no primary prints when the failed attempts ran.
- **G11 (note).** Line 48, "The only one was the first attempt's own", is inference. run.log line 1706 prints one superseded directory with this run's own name.
- **G12 (note).** Line 5, "nothing pushed": no primary checks a remote.
- **G13 (note).** Line 55, "as specs/0006's pattern ran it": specs/0006 is not in `<check>`. Line 54, "a Python traceback", comes from reading the code (check 1's reading), not from a measurement.
- **G14 (note).** Line 7, "at the hashes planning sent": gate.log compares against hashes the script labels as planning's. Planning's message itself is not a primary here.
- G2 applies to line 9.

I found no number in the report that disagrees with its primary.

---

**Privacy**

- Workbooks appear by 12-hex id, producer and tags only (record .md lines 102-139; run.log lines 1768-1794).
- I reduced every entry name in the record .txt to its shape. All are generic package part names such as `xl/worksheets/sheetN.xml` and `customXml/itemN.xml`.
- The xml rows carry 6 element names and 11 attribute names, all in ECMA-376 or OPC namespaces.
- No cell value, formula text, sheet name or original workbook file name appears in any log, in the record pair or in the report.
- The corpus directory's default name appears in run.log's printed script (line 163) and in its path-control pattern (lines 833-837, which pair it with a fictional file name). The log states this at lines 76-79. It is a directory name, not a workbook file name.
- test.log lines 3268-3269 print the two hash values of M9's failing assertion, as ruling 4 accepts.