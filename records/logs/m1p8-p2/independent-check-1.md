# M1 part 8 P2 independent check 1

A fresh subagent, given only the spec (specs/0015 as it stands, sha256 03fa7a11...), the phase diff, the draft report, the four logs, the pre-check scan and the record pair, never the conversation. Its answer follows verbatim, unedited.

---

**Independent check: M1 part 8 P2 (phase commit da080f1, base 50d2c4a)**

I read only the files in the check directory. The input hashes are below. phase.diff hashes to 237975ab..., the same combined-diff hash gate.log prints. spec-0015.md hashes to 03fa7a11..., the uncommitted spec every log names.

| File | sha256 |
|---|---|
| gate.log | b1e79f78... |
| test.log | 9a9d649f... |
| negative.log | 6c30e708... |
| run.log | f070278f... |
| m1p8-open-2026-09-24-da080f1.md | 43c768e5... |
| m1p8-open-2026-09-24-da080f1.txt | 70684a4e... |

All six match the report.

### Q1. Does the diff do what the spec says and nothing else?

**Verdict: YES.** The code matches ruling 3 (a) and (b) and P2 rulings 1 to 10. The only additions are a few defensive refusals (F3).

**P1 verification ruling 3 (a).** lo-check.sh adds `LO_SHEET_PATTERN` (phase.diff 617-625), with any prefix or none; `<sheets` and `<sheetData` are still excluded. lo_check uses it (633-648). The comment carries the foreign-namespace caveat, as P2 ruling 4 asks.

**Ruling 3 (b) and P2 rulings 1 to 3:**
- `TXT_HEADER` now has nine columns (112).
- `reason_class` maps the fixed opening words to none, refused, count or error. An unknown opening exits 2 (120-145).
- A count-class input goes under "did not apply" only when the script's own sheet count is 0. Otherwise it goes under "counts differ (a finding)", which exits 1 (147-155, 199-201, 248-263).
- There is one class-count row per side (214-217), and the reason columns are in the Pairs table (240-244).
- The headings follow ruling 3 (b), with "Inputs ... to" wording that differs slightly.

**P2 rulings 5 to 10 (excel-check-list.py, phase.diff 265-608):**
- `--record` is required. The record must name exactly the keep's pairs, by name, bytes and both sha256s, or the script exits 2 (537-566).
- Only pairs with `lo_in ok` are listed (567).
- A, M and F are derived from the corpus input, found through `manifest.tsv` in the grandparent of the run directory (493-534, 409-482).
- Corpus rows come first in manifest order, then fixture rows in name order. The expected values are 42, 2 and 5. The six reference columns are empty when there is no A, and empty on fixture rows.
- The script prints counts only, and error lines carry no path.

**Template (ruling 7).** It is at `records/excel-check-template.md` (16-70). It has the version, build, OS and date, pass/repair/fail, and seen1 to seen3. It holds no references.

**Commit order and scope (ruling 10).** gate.log 1004-1016 and 1169-1191 show the order ed7c80f (ruling 3), then f66b4c5 (the list script), then da080f1 (the template). Each commit stays in its own area, and no crate, tool, lock or manifest file changed. The spec text in the diff ends at P2 ruling 7 (843), which fits ruling 11.

**Beyond the spec:**
- edit-open-check.sh refuses a class that does not fit its side's verdict (141-144; tested as S4).
- excel-check-list.py refuses:
  - a run outcome other than ok (521)
  - a workbook named twice in the summary (510)
  - an `--out` that is already there (587)
  - a malformed record (553-555)

These are defensive and consistent with the spec. The report does not call them extras (F3).

### Q2. Do the logs contain every line the spec requires?

**Verdict: YES**, with the weak spots in F1, F4 and F5.

**P2's negative cases:**
- a missing host output: negative.log 1347-1354 (L1, exit 2, exact line, no list)
- a record hash the file does not match: 1356-1367 (L2)
- a keep with an odd file: 1369-1376 (L3)
- the sound-layout control: L0 at 1322-1345

**Ruling 3's prefixed-workbook fixture case:** S1 at negative.log 1399-1490, with the base library refusing the same workbook as control at 1488-1490. S2 (1492-1562) shows the foreign-namespace consequence.

**T3 P3's runner cases rerun at the phase commit and at the base:** negative.log 1653-1740. C1 to C9, C11 and C12 all pass. Runs match 5 of 5 and reason files 7 of 7.

**The run log's dry run:**

| Requirement | run.log lines |
|---|---|
| Keep regenerated on both targets, compared byte for byte | 922-1026 |
| Keep equal to P1's record hashes, with control | 1028-1032 |
| New counts | 1187 |
| Prefixed fixture's two pairs move fail/fail to ok/ok, with control | 1253-1264 |
| Install command printed, installed copies hashed | 1215-1222 |
| Left-out tests named with their class | 1280-1291 |
| List from part 6 P3's run directory, found via its committed record | 1293-1297 |
| Every hash checked (165 of 165), with control | 1319-1323 |
| p2check.py extracted and matched to the hash its log printed | 1326-1330 |
| References match the derivation 27 of 27, planted control 26 | 1364-1367 |
| Formula lines: added and replaced match 27 of 27, with control | 1370-1377 |

No corpus reference is printed anywhere. The logs hold counts, test names, hashes and the fictional fixtures only.

**Structural rules:**
- Lines 1 to 3 are the command, invocation and "about:" in all five logs.
- Shape greps have controls: run.log 5-15, negative.log 5-15, and the same in gate.log and test.log.
- Root lines are present, and "sourced" is stated at run.log 28 and negative.log 38.
- The SERIALIZED line is present: run.log 26, negative.log 36, test.log 45, and gate.log and the scan log.
- The pre-check scan (12:42:24Z) runs after run.log ends (12:42:06Z).

### Q3. Does every number and quoted string in the report appear in a primary?

**Verdict: PARTLY.** Every number checks out:
- all hashes and commit shas
- 523/0/2 on both targets; tests added 0, removed 0
- nine PASS
- 149 pairs; 138/11/0 and 139/10/0; 8 refused, 3 did not apply, 0 differ, 0 findings
- 27 corpus rows (19 with A, 15 with F); 138 of 149 fixture rows, 11 left out; 165 rows
- 27 of 27 references and formula lines; 5 of 5 runs; 7 of 7 reason files
- the list at 36697 bytes and 166 lines
- scan totals of 0

**One statement is not supported:** report line 14 says "Every log prints that path as planning's uncommitted spec." That holds for gate.log (1333-1339), run.log (908, 1386-1389) and the scan log (1367-1383). test.log and negative.log never print the path or its hash. They only count status lines against a pattern that would equally accept a record-pair or log line (test.log 56 and 1950; negative.log 47 and 1746). See F1.

The Notes section's claims (unlogged dry runs, the two fixed script defects, `__pycache__`) are not primaries. They are consistent with the logs: the fixed-time comment is at negative.log 1172-1173, the `mkdir -p "$2"` in S3/S4 at 1566 and 1576, and the status counts show 1 line.

### Other findings

**F1 (low-medium): the spec path is not printed in two logs.** test.log and negative.log do not print "M specs/0015-open-checks.md" or its hash, which P1 ruling 8's tree-status form requires. Their check `status lines 1; of them ... 1` cannot tell the spec from any other allowed path, and has no control in those two logs. The report's line 14 overstates this.

**F2 (medium): the record mislabels two refused inputs.** The heading "Inputs refused by LibreOffice (allowed: a zip-level fixture)" lists two crate inputs that are not zip-level:
- `package.tests.a_created_rels_part_falls_back_when_the_package_has_no_rels_part.1`
- `package.tests.an_empty_root_in_content_types_gains_an_end_tag.1`

run.log's own wb.py shows each holds `xl/workbook.xml` with one sheet element (run.log 1277-1279). run.log says ruling 3 "names them zip-level fixtures". It does not: ruling 3 describes them as refused by LibreOffice, with no relationships part and an empty content-types root. The heading text comes from ruling 3 (b), so the code follows the spec. Still, this is the same wording-defect class P1 verification F3 raised, and the report says nothing about it. Planning should rule on the label.

**F3 (low): extra refusals are undisclosed and partly untested.** Listed under Q1. Untested: outcome other than ok, missing manifest, missing summary, half a pair, a malformed or repeated record row. L4 and L5 test two others.

**F4 (low): several run.log checks have no control.** Four have none:
- corpus rows first (1309)
- corpus file naming (1314)
- empty exactly when added is 0 (1376)
- constants 42/2/5 (1378)

Two more are only partly covered: fixture labelling (1317) and "fixture rows are the lo_in-ok pairs" (1316). The hash-check control (605-607) only exercises a corpus row, and it would fail spuriously if that hash began with "0".

**F5 (low): the class map is only partly exercised.** Tested: refused (the stderr opening), count, error (podman exit 125), and unknown. Not tested: "the conversion exited", timeout, killed, and the other error openings. The design is fail-safe (an unknown opening exits 2), so the risk is a spurious stop, not a wrong class.

**F6 (low, privacy hardening): the list is world-readable.** The Excel list holds corpus cell references and was written mode 644 under the home cache (run.log 1304). edit-open-check.sh uses umask 077, but excel-check-list.py sets no mode.

**F7 (low): one error path breaks the script's contract.** excel-check-list.py does not catch IndexError, which a short manifest or summary row would raise. It would then print a traceback and exit 1, not one "error:" line and exit 2 as its docstring says.

**F8 (info): committed code cites uncommitted rulings.** The docstring of committed f66b4c5 cites P2 rulings 8 to 10, which are uncommitted until close-out (ruling 11).

**F9 (info): run.log prints reason-file contents.** It cats them (run.log 365, 1193-1213) even though the script never prints them. Here they are fixed one-line openings for fictional fixtures, but any extra line in a reason file would be printed too.

**F10 (cosmetic):** negative.log 1652-1653 has no blank line before "## family R".
