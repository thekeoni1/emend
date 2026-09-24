# Independent check 1, M1 part 7 P2

Checked commit: ea37ad786edf2798ced1e603c78aa136289db212 (the phase commit; HEAD when the check ran).
Checker: a fresh general-purpose subagent given only the files below in <scratch>/p2/check (written <check> in the answer), not this session's conversation.
Inputs, sha256 as hashed before the check (<scratch>/p2/check1-inputs.sha256):
  84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f  0001-bootstrap.md
  56db12e066acf4f5b10bb4cba30f6e17e101cf78b444a8b741f33c9a26e128fe  0014-xml-comparison.md
  6509a53d6643076212aac9937886a1091904127865cc5242dd4fb790c16c8f1a  diff-code-04ba408.patch
  52cffee05203607887665c631f1a4dfc57742c475b26277f0f23dddd41391933  diff-code-bacffa3.patch
  1605daa690b1e19ed8bbe913f800c0f5bf714a4608e28d8617027bb903e9228d  diff-code-ea37ad7.patch
  7ceb33f6bb8780d0dcc3653a3cc952672126923886d0d6182dbce2ae3424826b  diff-commands.txt
  e61f238f2ae393bc54d1f659cd02f5cef1482270cc8f09149733f10023acd372  diff-phase.patch
  ef3e93f467f82fe608b3a932a95a8b9f2b908a2d86d52e8bc9b77a47d3711bbe  gate.log
  bec8f62d9f998ecf556f5e00c5a5b2709949b421bedbef6f80b8ab8d5ec17c1b  m1p7-p2-2026-09-24-ea37ad7.md
  11a9b69cda36e05260b3df07a2f3ba37c1d3e0fa7649283e422cd3f0605a6a98  m1p7-p2-2026-09-24-ea37ad7.txt
  aabd5001c70ea4011c03a6dff008dd64646b5c33647133a1025d4a36be1d900b  negative.log
  ca4f3f38f3232f38c305d62ad37970209341af18b033a60c7a5674794689d9a7  report-draft.md
  cc6846dcbf27b3d0b3e51ea607868af01324e17854e44fa11241ea795007c739  run.log
  f7f2f6970e943196936f2d69f3ec75e8e91998a63fea8abeeee2953a309bac39  scan-for-checkers.log
  85d4f3bd2c42f2cc707efd8f3cbbaf15bd10250a91df53bfcf71c6b62309f6f4  test.log
The logs and record pair it read are superseded: no code changed after it, but the log scripts changed for its F3 to F7 and F9, and all four logs were rerun serialized at the same phase commit; the record pair was rewritten by the rerun run.log. The superseded files were committed nowhere; each is named by sha256 in its successor's rerun line.
The answer below is the checker's final message, extracted from its transcript, byte count 11788, sha256 4b86bf53d887178b74a1da846cd93c6f5131ba574d5f32decb5ce68d1b02903d; it is all ASCII and holds no machine path, so it is unaltered.

---

**Independent check: M1 part 7 P2 (phase commit ea37ad7, range 5fa6c8f..ea37ad7)**

I worked only from the files in `<check>`. CLAUDE.md was also in my context through the harness; I do not use it as evidence. Before relying on any file, I hashed every file in `<check>` and compared it with the hashes the logs print:
- The four diffs match gate.log lines 1212-1222: phase e61f238f..., bacffa3 52cffee0..., 04ba408 6509a53d..., ea37ad7 1605daa6....
- The spec matches gate.log line 1228 (56db12e0..., 24952 bytes).
- The four logs match scan-for-checkers.log: gate ef3e93f4..., test 85d4f3bd..., negative aabd5001..., run cc6846dc....
- The record pair matches run.log lines 1858-1859 (bec8f62d..., 11a9b69c...).

---

**Question 1: Does the diff do what the spec says for this phase, and nothing else?**

Yes, with the findings below. Which diff I read for each "nothing else":

- **Doc commit 4cb3c6a.** There is no separate diff file for it in `<check>`. I read its effect in diff-phase.patch lines 1-12 (ROADMAP.md) and 202-301 (the spec). gate.log lines 1181-1209 show both files at the hashes planning's message gave, at the doc commit and at the phase commit. gate.log lines 1205-1206 show the commit carries exactly those two files. That satisfies ruling 8 ("carries this spec and ROADMAP.md unedited").
- **bacffa3 (read in diff-code-bacffa3.patch).**
  - Ruling 6's two fixtures are added to tests/p7.rs: `an_attribute_cr_lf_against_an_lf_compares_equal` asserts values 1 1 0 0, and `a_utf16_entry_is_a_parse_error` asserts a parse row `ok error 0 0 0` and exit 1.
  - The Limits line "the tool reads UTF-8 only" is added to xml.rs.
  - Nothing else. gate.log lines 1273-1277 show it touches only tools/xlsx-diff.
- **04ba408 (read in diff-code-04ba408.patch).**
  - The record script gains the .txt third table (the xml rows prefixed by id and label, padded to five columns) and a .md table per label, "XML comparison of changed entries", by family, with compared, parse_errors, names_equal, values_equal, in_only and out_only. This is what "Harness runs" asks for.
  - It also makes changes the spec does not name, each a direct consequence:
    - the module docstring;
    - the .md intro sentence;
    - an "xml rows" count on stdout;
    - xml rows are read only for rows with outcome ok and diff_exit 0 or 1 (diff-phase.patch lines 137-141).
  - The runner is unchanged, and there is no Cargo.lock or manifest change (gate.log lines 1278-1291).
- **ea37ad7 (read in diff-code-ea37ad7.patch).** A two-line refusal of a repeated xml archive row. Nothing else.

**F1 (note).** The record script does not check row order. `xml_tally` (diff-phase.patch lines 101-126) assumes each entry's rows start with a `parse` row.
- If a diff file has a `values` row before any `parse` row, the script stops with a Python UnboundLocalError or TypeError and a traceback, not a clean `Failure` refusal.
- It also accepts element or attribute rows that are out of order or unsorted.
- The spec does not ask the script to check order, and the archive-agreement check limits the damage. No refusal case covers this.

**F2 (note).** The spec's P2 text asks for the record script's "own test on a fixture diff file (the script's existing test pattern)". The test is in negative.log family B (lines 2223-2913), not in test.log. test.log line ~3519 says: "the record script's test on a fixture run directory is negative.log's (the coordinator's word for this phase)". negative.log's script comment says this is specs/0006's pattern. specs/0006 is not in `<check>`, so I cannot confirm that is the existing pattern.

---

**Question 2: Do the logs contain every line the spec and 0001 require?**

Present and verified:
- The first three lines (command, invocation, about) in all four logs.
- The phase-commit guard with controls in all four (run.log lines 4-11 add a fabricated-commit control).
- SERIALIZED lines with process counts in all four.
- Rerun lines with superseded hashes printed by a command (gate.log lines 49-51, test.log lines 43-46, negative.log lines 38-43).
- The gate-file listing with no self-test (gate.log lines 1233-1270).
- The shape greps with controls, and the scan's control-byte form.
- test.log on both targets: 510/0/2 on each, at lines 1875 and 2433.
- run.log on both targets, with a control plant for each assertion flag (lines 1700-1733 and 1797-1830).
- The entry-status comparison with part 6 P3 (lines 1871-1875).
- scan-for-checkers.log fails only on the two check-answer conditions ("check answers present: 0 ... no" and "the check answers went through all three greps as prose: no"; "structural checks failed: 2"), as expected.

**F3 (defect).** Ruling 3 (R3) says "every root a log uses has its own root line". negative.log prints `<repo>` in its output at line 2526 (`command: python3 <repo>/scripts/passthrough-record.py ...`) and line 2916 (`command: git -C <repo> status --porcelain`). Its root lines (44-46) define only `<scratch>/w`, `<scratch>/p2` and `<scratch>/p2/target`. There is no `root: <repo>` line. The report's "3. Every root a P2 log uses has a root line." (report-draft.md line 31) is therefore not true for negative.log.

**F4 (gap).** The spec's "Phases" section carries forward specs/0013's rules, including "superseded run directories removed before the run and printed". run.log's script header change 3 (run.log line 106) says instead: "Part 6 P3's removal of P2's superseded run directories is gone (no ruling asks for one here)". The log only checks that the current run directory holds no summary (lines 1632-1633). It prints nothing about whether any earlier run directory for this phase exists. I disagree with the implementer's reading: 0014 names this rule as one that applies.

**F5 (gap).** Commit ea37ad7's message (test.log line 3524) says: "The P2 negative log measured that scripts/passthrough-record.py accepted a diff file holding an xml archive row twice". The report repeats this at lines 10 and 39.
- The log that made that measurement (the 04ba408 attempt, ab68677f) is "committed nowhere" (negative.log rerun line, line 38).
- The committed negative.log prints only R8, the refusal at the fixed code (lines 2896-2903). It never prints the acceptance before the fix.
- The Phases section of 0014 says "a commit message that states an unprinted measurement says so", and 0001 has the matching rule. The message does not say it.
- Planning may accept the printed sha256 of the superseded log as the pointer, but its content is not in any committed primary.

**F6 (note).** Under the heading "the main working tree was never used" (negative.log line 2915), the only check is `git status --porcelain` showing 0 lines. That shows the tree is unchanged, not unused. Line 2526 runs `<repo>/scripts/passthrough-record.py` in place, with its output going to scratch. Ruling 2 covers running the built binary on scratch fixtures; it does not clearly cover running a repository script from the main tree.

**F7 (note).** run.log line 1842, "control on the first workbook's diff file, which gained xml and archive rows since part 6 P3:", ends in a colon with nothing printed after it. The comparison itself is the next line.

**F8 (note).** The check of the .txt third table in negative.log (check_b.py) pads the rows the same way the record script does. It does not check the padding independently, but it does compare against the raw diff-file rows.

**F9 (note).** Stale comments in the printed scripts:
- gate.log line 91 says P1's final gate script is "printed in records/logs/m1p7-p2/gate.log"; this presumably means m1p7-p1.
- gate.log line 98 says "the two code commits"; header change 9 corrects this to three.
- test.log lines 801 and 2704 (the M11 comment) say "the second code commit's fix undone". That refers to P1's second code commit, which in P2's context reads as 04ba408.

---

**Question 3: Does every number and quoted string in report-draft.md appear in a primary?**

Verified against primaries:
- The commit ids and subjects: gate.log lines 1135-1139.
- The 04ba408 summary: diff and message, test.log lines 3497-3513.
- "510 passed, 0 failed, 2 ignored" on both targets: test.log lines 1875 and 2433.
- "p7 17 on each": test.log line 2538.
- value_hash_is_pinned on both targets: line 2544.
- M1-M12, with M12 as "attribute values not normalised" and M9 biting on i686 only: lines 3452-3467.
- The ruling 7 case map and the ruling 8 probe: lines 2549-2822.
- N11 and N12, and R1 to R8: negative.log.
- The superseded hashes 63ef5c36, 0618829d and ab68677f: the rerun headers of gate.log, test.log and negative.log.
- Run directory 2026-09-24-ea37ad7: run.log line 41.
- xml rows equal the derivation, and outputs identical to part 6 P3: run.log lines 1737-1843.
- "27 workbooks, outcome, lo and lo_in ok": the record .md "Files by outcome" table and run.log line 1675.
- "compared 44, parse errors 0, names equal 0, values equal 0, in_only 104, out_only 146": record .md lines 86 and 96, and run.log lines 1860-1863.

**F10 (note).** "44 = 19 + 19 + 2 x 3" is labelled as the author's arithmetic, but a primary does print it: the record .md family rows at lines 82-85 give content-types 3, rels 3, workbook 19, worksheet 19. "The 3 where run.log's rels plant applied" matches run.log line 1719 ("rels_ok: applied on 3").

**F11 (note).** Rulings applied item 7 (report line 35) says "single-value-change fixture". Spec ruling 7 says "single-attribute-change fixture", and test.log line 2556 names it "one attribute's value changed". The substance is the same; the report should use the spec's words.

**F12 (note).** Report line 3 says the phase follows rulings 1 to 8, but "Rulings applied" lists only 1 to 7. Ruling 8's substance (the doc commit carries the spec and ROADMAP.md unedited, then the fixtures, the script and the run) is covered by the commits section and gate.log lines 1181-1209.

**F13 (note).** Author's inference or references outside `<check>`:
- Finding 2, "whose checks held", refers to part 6 P3's records. No file in `<check>` prints those.
- "(the UTC date had turned)" is inferred from gate.log line 48 (2026-09-23T23:59:43Z) against run.log line 42 (2026-09-24T00:16:31Z).
- Line 21, "each named by sha256 in its successor's header": the earlier attempts f0d1495e and 09cb2c7a are named in the final logs' headers (test.log line 43, negative.log line 38). Their immediate successors are not in `<check>`, so the "successor's header" part cannot be checked beyond the final logs.

**F14 (note).** Report line 16 says "N0 to N12". The cases are N0 to N7b and N10 to N12, 12 in all (negative.log line 2217, "cases: 12"). N8 and N9 are checks across cases, not cases. "Each with its message extracted from the source" is right in substance: negative.log lines 2833-2838 extract six message forms for eight refusals, and R7 and R8 share one.

**F15 (note, supports F5).** Report line 10, "The P2 negative log at 04ba408 measured that it accepted one (its case B3, a measured-only case)", has no primary in `<check>`. See F5.

---

**Privacy**

- Workbooks appear by id only, with producer and tags.
- Entry names in the record .txt are generic package part names.
- The xml name rows in the .txt are schema names only. There are 6 element names and 11 attribute names, all in ECMA-376 or OPC namespaces.
- I found no cell values, formulas, sheet names or original file names in any log, in the record pair, or in the report.
- The printed run script (run.log line 150) contains the default corpus directory name. The log says so itself (lines 69-72). It is a directory name, not a workbook file name.
- test.log lines 3263-3264 print two hash values from the M9 failing assertion, which ruling 4 accepts.
- The thread numbers in the panic lines are process-local and harmless.