# M8 part 2 P4: independent check 1 (the records against the run)

The answer of a fresh subagent given only these materials, under
<scratch>/check1:
- the working spec specs/0048 (9a6e0db513d352beb46957a9212cf703492aec70a2e13c4c3f64b7a6dba4df31), with ruling N;
- run.log;
- the 118 new records and the 118 records of part 1 P2 they were compared with;
- the logs README as it stood before the checks (sha256 e7928b72c16966adac3d8ba3d522fd7913acb9a0c2cab75375f2262bab703fa7).

The answer was extracted from the subagent's transcript by script into
<phase>/check1-answer.txt (sha256 628e27c470438c1535d79c35bfbceeaf353dea8339af379b310588aab265a846). It appears below verbatim, except
that the absolute paths it names are written with the roots declared
here. The README's section "After independent checks" answers each
exception.

root: <scratch> stands for <phase>, as in gate.log and run.log
root: <phase> stands for <home>/.cache/emend-build/m8p2-p4, the phase's directory
root: <home> stands for the home directory of the account the runs ran under
root: <check> stands for <scratch>/check1, the materials directory the subagent was given
root: <session-scratch> stands for the implementing session's own scratch directory, outside the repository

# Independent check 1, M8 part 2 P4

I used only the materials in `<check>` = <phase>/check1. I modified nothing and ran no git or cargo. The one script I wrote is at <session-scratch>/q2.py. No corpus path, sheet name or cell content is quoted below.

## 1. Do the 118 record files match run.log section 5?

**Yes: 118 of 118 match, and none differs.**

- run.log section 5 starts at line 7266. Its "installed records/..." lines are 7275 to 7392, 118 of them.
- For each file in records-m8p2/ I computed the sha256 and the byte count. Each one equals its line in run.log, by name, hash and bytes.
- No file is missing from the log and no log line lacks a file.
- Spot check: the remove pair's hashes at run.log line 3869 (a050ee46...4711d3 for the .md, b53c84a2...cb48f249 for the .txt) equal the files' computed hashes and their section 5 line 7369.
- run.log line 7393 reads "every record file was installed, byte for byte: yes", which agrees.
- The 118 files total 12409762 bytes.

## 2. Comparing the records themselves, per id

**Layout.** Each .txt opens with a summary table. Its header is line 1: `id label producer tags outcome ... lo lo_in`, 24 tab-separated columns. Then come a blank line, an entry table and an xml table. I compared the summary rows whose label is `host`. Every one of the 4602 summary rows in the 236 files is `host` (118 x 39), and no id appears twice.

**Counts per name.** Every name has 39 earlier ids and 39 new ids, with missing 0 and new 0.

- Six names read same 38, changed 1: comment, commentbox, delete-columns, delete-rows, remove and richcomment.
- The other 53 names read same 39, changed 0. They are passthrough, add-chart, anchor, border, cf, chart-3d, chart-axis, chart-edit, chart-format, chart-kinds, chart-labels, chart-more, chart-surface, chart-text, chart-trend, charts, chartsheet, colors, copy, date, edit, edit-formula, extlink, font, format, formula, gradient, group, image, insert-columns, insert-rows, kinds, linestyle, link, namedstyle, namedstyles, names, pagesetup, pivot, printarea, props, protect, read, remove-chart, remove-image, richtext, roundtrip, scale, sheet, sheetview, style, table and validate.

**The changed ids** (outcome/lo/lo_in, then and now, with the row's line number in both .txt files):

| Name | Id | Then | Now | Line |
|---|---|---|---|---|
| delete-rows | 90a5b01f9541 | error/-/ok (exit 2) | ok/ok/ok (exit 0) | 30 |
| delete-columns | 91d2bdffa46a | error/-/ok (exit 2) | ok/ok/ok (exit 0) | 29 |
| comment | 15205770ae5c | error/-/ok | ok/ok/ok | 32 |
| richcomment | 15205770ae5c | error/-/ok | ok/ok/ok | 32 |
| commentbox | 15205770ae5c | error/-/ok | ok/ok/ok | 32 |
| remove | 495c7b79b183 | ok/fail/ok | ok/ok/ok | 36 |

copy 15205770ae5c reads error/-/ok (exit 2) in both records, at line 32.

**Against the expected-change table: the records show exactly that, with one caveat.**

- Every pair changes as the spec says, copy is kept, and nothing else changes.
- The caveat: the records do not carry the `removed` or `removed_reason` fields. Neither "pivot_source" nor "removed" appears in any of the 236 record files. So the records alone cannot prove "removed 0 and removed_reason pivot_source", or that no other id reads pivot_source.
- That evidence is in run.log only:
  - line 3874: "- 26 last 12 pivot_source 1"
  - line 3877: "their lines met 1"
  - line 7456: "other ids reading pivot_source: 0"
- What the records do show is consistent with a removal of 0 on 495c7b79b183: its new remove row has changed 0, dropped 0, added 0 and files_identical yes.

## 3. Does the README say only what run.log and the records support?

I checked these against run.log and found them correct:
- run.log's sha256, 522611 bytes, 7599 lines and its last line.
- The 118 files and 12409762 bytes.
- Lines 66, 79, 3869, 7200, 7266, 7393, 3317 to 3328, 3872/3877, 4330/4333, 4461/4464, 5047/5050, 5243/5246, 5894/5897, 6090/6093, 6741/6743, 3874, 3942, 7396 to 7455, 7456, 7457, 7464 and 7598, with their quoted strings.
- The commit roles and subjects (run.log lines 41 to 61 and 2276 to 2278).
- The PLANNING_SPEC hash. It is at run.log lines 6, 3141 and 7588, and it equals the sha256 of spec-0048-working.md.
- All 59 summary lines end "FINDING lines so far 0". The 53 unchanged samples read "same 39, changed 0, missing 0".
- run.log has no line ending UNEXPECTED.
- The claim that the counters are cumulative is borne out by the printed script: `n_line_met` and `n_pivot` are set to 0 once (lines 2680 and 2681), and `n_met_here` is reset per run (line 2776).
- remove-chart reads ok/ok/ok on all 39 ids in both the m8p1 and the m8p2 record (README lines 149 and 150).

**Discrepancies:**

1. **README lines 124 and 125** say "the dry corpus is generated, and its ids differ from the 39". The README itself contradicts this:
   - Line 131 names the dry file 453a166dcdfe.
   - Line 148 speaks of 15205770ae5c's dry worksheet root.
   - Both are ids of the real 39: they are rows 31 and 32 of every m8p2 and m8p1 .txt.
   - The cited dry comparison "same 1, changed 8, missing 30" means all 9 dry files matched ids of the 39-id record.

   So the dry ids are not distinct from the 39. The sentence is wrong, or at least misleading.
2. **README lines 4 and 5** say ruling N was given by message and that "planning writes it into the spec after this record commit". But ruling N is already present at lines 882 to 913 of spec-0048-working.md. That file's sha256 (9a6e0db5...4df31) is the PLANNING_SPEC the run hashed and checked (run.log lines 6 and 3141), so N was in the working spec when the run took it. The parenthesis may mean "commits it". As written, it does not match the materials.
3. **README line 52** gives run.log's end as line 7598 ("run ended at ..."). The log's last line is 7599 ("run log checks all as expected: yes"), which is what the README's own table at line 33 cites. This is small, but the two places point at different lines for the log's end.
4. **README lines 94 and 95** say "The pair's line reads removed 0 and removed_reason pivot_source". run.log never prints that line's fields directly. Its only evidence is the counters (lines 3877 and 7456, "met 1 of 1"), and the records do not carry the fields. Line 3875 is the expected value, not a reading. This is supported only indirectly, by the run's own check.
5. **README line 4** lists rulings "H, K (4), L and M". run.log's phase-commit line (line 45) names "rulings H, K (4) and M (1)", and the spec's P4 section names H, K (4) and M (1). L is the P3 interrupted-run ruling, so its relevance to P4 is not shown in these materials. This is minor.

**Skipped as out of scope:**
- Everything sourced from gate.log: line 32's row, line 50 and the gate half of line 51, lines 56 to 65, and lines 113 to 115 for gate.log.
- The dry run d1: lines 119 to 126 and line 148's dry claim, except the inconsistency noted above.
- The residue reproduction: lines 128 to 146.
- The driver output and the tmp listings: lines 156 to 173.
- The scripts and cmp claim: lines 44 to 48.
- The leftovers list: lines 175 to 183.
- The scan.log row.
- The privacy claim at lines 23 to 26 beyond the records I read, which show only ids, part names and XML element and attribute names.
