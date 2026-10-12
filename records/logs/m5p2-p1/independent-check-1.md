# M5 part 2 P1: independent check 1

A fresh subagent, given only the directory <home>/.cache/emend-build/m5p2-p1/check1. It held spec.md (specs/0033), diff.patch (3f2082e..3865f30), commit-messages.txt, report-draft.md, the gate, test, negative, run and self-test logs, the scan pre-check log and the four record files. Their sha256 values are in <phase>/check1-inputs.sha256. It answered the three questions (CLAUDE.md, "Independent check before reporting"). Its answer follows verbatim, except that two non-ASCII characters are written in ASCII so the record stays ASCII: "less than or equal" as <=, and "times" as x. The angle brackets are written as the characters; the delivery escaped them as HTML entities.

---

1. Does the diff do what the spec says for P1 and nothing else? PARTLY.

The diff implements survey.rs with all 29 PartKinds fields matching the spec's table, the 12 SheetParts fields, part_kinds, and sheet_parts with NO_SUCH_PART on an absent name. --kinds-sample asserts kinds_sum+1==entries (diff.patch 2049-2157). There are 9 crate tests and 2 tool tests. The doc commit's specs/0033 (extracted from the diff) hashes to b9f13b40..., matching spec.md. The scripts commit changes exactly 4 python3 lines.

Departures disclosed in the report: strict content types not counted (spec "The survey" says they "count beside the transitional"; report item 3); SLICER/TIMELINE single Microsoft URI written twice (item 2); PRINTER_SETTINGS added beyond the spec list (item 4); COMMENTS/VML_DRAWING/DRAWING/IMAGE/HYPERLINK gates widened with `read` (item 5); extra scripts commit (item 6); the spec's "strict-typed worksheet" is not in the every-kind test, replaced by a separate strict-package test with transitional types (item 3 covers it); gate consumer mktemp under <build>, not TMPDIR (item 8). N2 is an extra negative case, listed in Figures.

Not disclosed:
- The tool test comment (diff.patch, in the kinds_sample test) says "the workbook's external hyperlink is not a sheet's and is not counted". The test asserts `sheet_external_hyperlinks=1`, and the hyperlink is in the sheet's rels. The comment contradicts the code.
- The run log's P1 corpus runs use release binaries (run.log 3031-3038, 3578). Spec P3 says release runs "byte-identical to P1's, P2's and this phase's debug runs", which implies P1's run is debug. This is a possible departure for planning to rule on.
- negative.log 2496 creates a plain `mktemp` file (CE) after the TMPDIR check, with no check of it. Minor.

2. Do the logs contain every line the spec requires? YES, with minor gaps.

- Gate:
  - prune of the four roots, with du -sb lines and rm -r (no -f), each name gone (gate.log 91-239); the kept names include gen.py and logs;
  - the TMPDIR line and controls (10, 46-52) and PYB (42-45);
  - the committed-scripts check "files scanned: 18; Python calls: 4; calls without -B: 0", with the base control (5306, 5320-5326);
  - pub fn and cfg lines at base and phase;
  - the consumer check with off, rules, read and read,rules;
  - tests on four sets on both targets (6030-6100).
- Test log: f-host and f-i686 off, read, read,rules and all at 515, 594, 632 and 966; workspace 1168 on both targets (1714-1786). Plants P1-P7 all "yes" (2073-2155).
- Negative log: N1 refused with the exact message, N2 refused, control B0 exit 0, and the refused_with and line_is controls (2914-2957).
- Run log:
  - both targets 27/27, tool and check agree 27 (3674, 3784);
  - the spec expectations sheet_charts <= charts and sheet_tables = tables hold;
  - device lines (3570, 4079); calc-chain 9; record commit 2dfca6c named (45);
  - the keep of 1100 pairs and the open check (5185).
- python3 -I -B and TMPDIR: PYB is 0 in every logged script, and each prints "input: TMPDIR=<scratch>/tmp" with controls.

Gaps: scan.log (final) is not yet available; only the pre-check exists, which is expected. The self-test log has no TMPDIR check, only a statement in its invocation line.

3. Does every number and quoted string in the report appear in a primary? PARTLY.

Verified:
- Every log and record sha256 matches `sha256sum` on the files here.
- The printed scripts, extracted and hashed: gate 6e14bb36, test a3d2fa28, negative 5f6ccb02, run d9b97593, scan d89af7d1. kindscmp 4c81e977 matches run.log 3052. The self-test 940ea8a6 matches.
- These also match: commit hashes and subjects; the test counts; 870/843; "23 of 24; the departed fields: custom_xml"; custom_xml 30 in 10 against 60 in 10; the four du -sb bytes; the open-check line; the N1 and N2 messages; the scan pre-check results; the 29/12/9/2/4 counts.

Wrong or inexact:
- The self-test row gives the last line as "RESULT: PASS (50 cases, 0 unexpected, 0 errors)". The actual last line is "RESULT: PASS", and the counts are on the preceding line (self-test.log 108-109).
- "PYB scores ... 1 in its fixture" is false for run.log, which reads "in the fixture 2" (run.log 34).
- The rm -rf list is incomplete:
  - gate.log 2694 `trap 'rm -rf "${W:?}" "${CONSUMER:?}"'` is not listed;
  - negative.log 2667 `rm -rf "${d:?}"` is in the counting library, but the report names only gate, test and run for it;
  - scan-pre-check.log 1712 and 3132 (`rm -rf "${T:?}"` and `"${d:?}"`) are not mentioned;
  - negative.log 2499 `rm -f "$CE"` is not mentioned.
- The times "gate 16:51:53Z, test 16:53:42Z, negative 16:53:44Z, run 18:10:48Z" are not labelled as end times.

Not in any primary, and planning should verify them from the phase directory:
- `let _ = target;` (item 9; FACTS.md is not supplied) and "FACTS.md had said 0" (item 7);
- exit 144 and the pkill story, the heredoc slip, the fork's rm -f, and scan-fpfq.py;
- the prior script hashes 45955cc7, 62258939 and 3373efdc, and the superseded SHA256SUMS hashes 329ce7ec, 780700e8 and 1de3dd9a;
- "apart: command not found" x5 and the target size 4415291340;
- the outer exit, the outer stderr and the setsid launch for gate, test, negative and run (only self-test.log states its launch form);
- the dry-run sequence.

These are session claims.

---

Session's response: the report was corrected for every "wrong or inexact" item above. The comment contradiction and the release-versus-debug question are added as report items 10 and 11, and the unchecked mktemp file as item 12, all left for planning to rule. The unsourced items are marked as the session's statements, with their files in <phase>. Nothing in the code or the logs was changed after this check.
