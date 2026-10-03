# M3 part 3 P1, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec specs/0025, planning's working ruling section of it, the committed specs/0024, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, each one's stderr, the phase's four records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p3-p1. Checked commit (the phase commit): 9db5f4f7f60d08f99bf60a7762354f2f0c9ffbcd; HEAD when the check ran: 9db5f4f7f60d08f99bf60a7762354f2f0c9ffbcd. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 0c08c437214fbc6f5452cbdbd692f7f2a643a69c84d8fdebd0aac6dc7f61ee31. Its items on the report are corrected in the report; its observations on the code and the logs are reported to planning. No second check was run.

Inputs and their sha256:

- ff3493cfae4357dccae8b439189929caa66fc284bb545034cfcf7ac81f10e199 code-commit-1-carry.diff
- c563b81f9816b796308527f25a83a22ead6effdd2e1958fc72efed9f8a2b10f4 code-commit-2-crate.diff
- 37c351b70eb44e8b4ed311118b83408b366d23db846b61a24632a9ab517c9ace code-commit-3-tool.diff
- 7971e15aa63b0fbfd05bd47674808e8a9ea92dd40b5ef89fc365ba8fb4ee1f5d commit-messages.txt
- b49b661422ee41ded02c8eb702f46b1deff8da37ee1d8e4d76837100c6da01ff doc-commit.diff
- e349806a86b4f579722d1fc0507c44b7f8eb6336eac6a16141e5ea56fc45e941 gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- 3599c604007806756d575411b3ae0fdb9d1238368a9011d32189b0231e00c2bf negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- 07334a5682f41118c79571aecd1acac58bc8acad6bfd577b783d3867f43dd2dc phase.diff
- 6c712f4eb0e14042abe6c9cf64ab243b1ed22f71d21f4110eac8422444ebe07c records/m3p3-open-2026-10-03-9db5f4f.md
- 765a62c213e206bbe0c390406b25420bb16a68b4501532318e9d2b43e50d0252 records/m3p3-open-2026-10-03-9db5f4f.txt
- 1d6f83a32e6730e014f542a74429f0a7dc0e15db69ee54eec72d2cf118844bae records/m3p3-p1-2026-10-03-9db5f4f.md
- 63db3bca32a63240de8a1fefea4e4115ed45a23463f1591ea28c513babda2785 records/m3p3-p1-2026-10-03-9db5f4f.txt
- 1226203c49447ff438e2c1cb46ee2b07c6f327ebba7374ba9f60c38f42097884 report-draft.md
- 170f3d4879196ee2f2e59a62da01f60370609de43e8e56fbf555de150aa45817 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- ca722f25c317c96bdb7f1490aff9817beb81c64ac71ef523e5d83912bdacf01f scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- ae1ab201b5c7071c36235f2c5d8c253c6badfab95a9f163575ef7daa0e78863a spec-0024-committed.md
- c15d68d22af7832485d7bf7bcc39d89f94de526302fc1833aedd4873b498da8d spec-0025-committed.md
- 8fb1e5cedfc0354c2658de89f5f85f11ecf075804d7349434732ed765b08fdcb spec-0025-working-rulings.md
- c4df490880571675570a4e1b2ea33d72b7fd31c3ea18d787b4d74e7df37d538c test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

---

# Independent check 1: M3 part 3 P1

Inputs verified first: `sha256sum -c ../inputs.sha256` reported OK for all 25 files. That command's exit code of 1 came from `wc` being run on the `records` directory in the same call; it was not a checksum failure. I read only files under `<phase>/check1/inputs`. I ran no git commands and wrote nothing outside my scratch area.

Diffs used: for every "nothing else" judgement I read the per-commit diffs (`code-commit-1-carry.diff`, `code-commit-2-crate.diff`, `code-commit-3-tool.diff`) and `doc-commit.diff`. I confirmed that `phase.diff` adds and removes exactly the lines the three code-commit diffs do, compared as sorted line sets: 1659 added and 44 removed, versus 64+1251+344 added and 12+27+5 removed. `spec-0025-committed.md` hashes to c15d68d2... and is byte-identical to the file the doc commit adds (`doc-commit.diff` lines 127 to 774).

## 1. Does the diff do what the spec (with the working ruling) says for P1 and nothing else?

Verdict: YES with reservations.

Evidence that it does what P1 says:

- **Carry commit, `//!` lines only in package.rs.** In `code-commit-1-carry.diff`, every added or removed line in package.rs (lines 9-16, 24-37, 45-50) begins with `//!`. They carry the three corrections in `spec-0024-committed.md` P3 verification ruling 3 (lines 788-799). The commit also adds two unbound-prefix tests, each with a bound-prefix control: through the table at lines 106-121 and through a comment at lines 62-93 (ruling 4). gate.log line 2134 independently prints "lines added or removed 28; of them not a //! line 0".
- **The feature.** `sheetprops = []` is off by default with no dependency (`code-commit-2-crate.diff` lines 9-14), and the tool turns it on (`code-commit-3-tool.diff` line 14).
- **Attribute merge.** `merged_tag` (`code-commit-2-crate.diff` lines 662-690) writes the set attributes first, in the order given. It then copies every other attribute as raw bytes, from the name to the closing quote, in input order (lines 675-683). `SheetView::attributes` (lines 603-624) gives the order showFormulas, showGridLines, showRowColHeaders, showZeros, rightToLeft, view, zoomScale, which is 18.3.1.87's order. The test at lines 1171-1278 checks this against the sixteen attributes, and plant Y1 bites (test.log, plant Y1 block).
- **tabColor first; an emptied sheetPr dropped.** `sheet_pr_written` (lines 898-941) removes any kept tabColor and inserts the new one before the first outlinePr or pageSetUpPr. It returns nothing when the element is left with no attribute and no non-whitespace content (lines 917-923). Tests: lines 1069-1092 (first, the attribute and children kept), 1095-1105 (replaced), 1108-1131 (None drops it, and an empty sheetPr goes with it). Plants Y2 and Y3 bite.
- **Creation in schema position with the root's prefix.** `create_before` (lines 817-838) creates sheetPr before the first root child whose schema position is greater than sheetPr's, using the root prefix, and at the root's end or in an opened-up empty root (lines 455-460, 511-516). The tabColor takes the held prefix (line 912). Tests: lines 1010-1039 (each colour form, literal bytes, before `<dimension>`) and 1042-1066 (prefixed root; empty root). Created `<sheetViews>`: `created_element` (lines 337-356) yields `<sheetView ... workbookViewId="0"/>` with the set attributes first; tests at lines 1292-1314 and, with a freeze, 1319-1360.
- **Refusals.**
  - Zoom outside 10..=400 is refused at the call (lines 264-270).
  - Two sheetPr, or a sheetPr after a later root child, gets TWO_SETTINGS (lines 804-813). Tested at lines 1445-1467 for both Some and None, with a control; plant Y5 bites.
  - Removed, replaced and absent parts, and a pending shift, are refused through the existing write checks (lines 116-147, 286-295). Tested at lines 1485-1536.
  - Working ruling 1 (the part is not looked up at the call; it joins the edited-parts list): lines 142-147. The added-sheet test is at lines 1539-1557.
- **Tests P1 lists.** All are present (test.log lines 4114-4236 list the 21 added tests).
- **The tool sample.** The line's keys and order match "Harness runs" (`code-commit-3-tool.diff` lines 282-297). The assertions cover:
  - sheetPr count 1 and tabColor count 1 with rgb FF00AA55 (lines 311-316)
  - views equal, or 1 for a created view (lines 317-324)
  - the three set values (lines 326-334)
  - kept attributes equal in names and bytes, or exactly `workbookViewId="0"` for a created view (lines 335-344)
  - output removed on a failure after write (lines 277-281)

  Tool tests: lines 377-450. W is the first worksheet in `<sheets>` order (lines 256-262).
- **Commit areas.** Each commit touches only its own area (gate.log lines 2073-2198). No gate file changed (gate.log lines 2006-2071).

Reservations:

- **R1: a refusal the spec does not list (disclosed).** `set_tab_color` refuses a theme tint that is non-finite or outside -1.0..=1.0 with "a sheet setting is out of range" (`code-commit-2-crate.diff` lines 250-260). The spec's "Errors" list (`spec-0025-committed.md` lines 288-290) does not name a tint. The report discloses this (report-draft.md line 24); planning should rule on it.
- **R2: the merge routine's doc comment does not match the code.** The comment says an attribute counts as set "when its unprefixed name is" (`code-commit-2-crate.diff` line 658). The code compares the raw qualified name (lines 676-677), so a prefixed attribute of the same local name is kept, not replaced. The behaviour is defensible; the comment is inaccurate.
- **R3: whitespace between attributes is normalised.** The merge rewrites the separator before each kept attribute as one space and drops any whitespace before `>` or `/>` (lines 680, 684-688). Each attribute's own bytes are kept, as the spec says; the separators are not.
- **R4: the out-of-order refusal fires only on a tab-colour edit.** The two-sheetPr and out-of-order refusal runs only when the write edits sheetPr (lines 806-811). A view-only edit on a part with two sheetPr is not refused. I read "The pass" as scoping the refusal to an element with a recorded edit, so this matches, but no test pins the view-only case.
- **R5: an existing code path changed.** The freeze path now records the first view's depth only when a pane edit exists (`code-commit-2-crate.diff` line 489, was `if !empty`). I could not confirm from the hunk context alone that this arm was unreachable with no pane edit before this change. The test at lines 1350-1359 ("a view alone keeps the pane and selections") pins the new behaviour.
- **R6: types differ slightly from the spec's list.** SheetView and ViewKind also derive Copy and Eq (lines 543, 564), beyond the spec's Clone, Debug, Default, PartialEq. No separate `TabColor` name exists; `styles::Color` is used directly. Both are harmless additions or readings.
- **R7: one of the sixteen attributes is unconfirmed.** The test's sixteen attributes (lines 1135-1152) include zoomScaleSheetLayoutView. The run log's survey covers W only and shows 15 names (run.log lines 2059-2085). No primary shows the sixteenth name across the corpus's 95 parts.

## 2. Do the logs contain every line the spec and the binding rulings require?

Verdict: YES with reservations.

- **gate.log.**
  - Crate tests with the feature off and on, on both targets: lines 2734-2761 (513/529 on host and i686).
  - pub fn and pub const lists off and on: from line 2425; additions at lines 2590-2593.
  - Consumer crate: lines 2763-2803.
  - Spec pin: lines 21-27.
  - Ends with "gate log checks all as expected: yes" (line 2819).
- **test.log.**
  - Workspace on both targets: lines 2011 and 3007; 947 each (line 4797).
  - Crate with sheetprops off and on, both targets, with "failed tests:" lines: lines 4238-4281.
  - EMEND_KEEP_DIR run: lines 4283-4298.
  - The 17-versus-16 module count explained by the rowcol cfg: lines 4300-4311.
  - Six plants, each failing at its own source line (303, 121, 160, 441, 489, 4529), "distinct assertion lines they failed at: 6": lines 4677-4795.
  - Spec pin: lines 22-28.
- **negative.log.**
  - Two sheetPr (S1) refused, with control S1C: lines 1113-1115.
  - Also S2 and S3 with controls: lines 1116-1121.
  - The unreachable message stated, with its covering test: lines 1135-1153.
  - Spec pin: line 71.
- **run.log.**
  - Both targets: lines 2854 and 2969.
  - Every row ok, 27 of 27: lines 2889 and 3004.
  - Expected entry results: lines 2891-2895 and 3006-3010.
  - The comparison's 16 flags with controls on a created case (ed4c55f07c69) and a kept case: lines 3089-3159.
  - Survey: lines 1901-2113.
  - TMPDIR and corpus device lines: lines 2845-2852.
  - Keep on both targets: lines 3329-3444.
  - Edit open check compared against the previous record, named by its record commit b018e52: lines 34-36 and 3446-3477.
  - Verdicts: lines 4406-4453.
  - Spec pin: lines 4 and 1882.
- **scan-pre-check.log.**
  - Spec pin: lines 7 and 53.
  - Shape greps over every printed script: 0 (lines 2652-2680).
  - Recursive-removal classification: lines 2682-2707, unruled 0.
  - RESULT: PASS (line 2707).
- **rm -rf.**
  - Every new removal is written in the `"${var:?}"` form: gate.log lines 538 and 596; test.log lines 592 and 665; scan-pre-check.log line 799.
  - The remaining instances are the committed `scripts/counting.sh` line, as allowed to stand by `spec-0024-committed.md` "Planning rulings during P2" ruling 1: gate.log line 1812; test.log line 1977; negative.log line 884; run.log line 1844; scan-pre-check.log line 2108. That line removes a directory the library created in the same run.

Reservations:

- **L1: a stale sentence left in the change list.** run.log's change list still carries JI's sentence "No `rm -rf` anywhere" (line 276), with LI correcting it (lines 367-372). `spec-0024-committed.md` P2 verification ruling 6 asked that the change list say what the script does. LI does, but the stale sentence remains as "history".
- **L2: the open check's TMPDIR device is not printed.** The device line covers the runs' TMPDIR (`<work>/tmp-run`) and the corpus directory. The open check runs with a different TMPDIR, `<work>/tmp` (run.log line 3562), whose device is not printed. It is presumably the same device, but no line shows it.
- **L3: plants run on the host only.** The comparison's planted-flag controls run on the host label only (run.log line 3089). The spec does not require i686 for them.
- **L4: the created-view path never meets the corpus.** The corpus has no W lacking `<sheetViews>` (run.log lines 2953-2954), so the comparison's created-view branch never runs on corpus data. The report discloses this (report-draft.md line 25).

## 3. Does every number, hash and quoted string in report-draft.md appear in a primary?

Verdict: YES with reservations.

Found in primaries:

- **Commit ids and spec hashes.** 21f1434 with its subject, e5f998a, bf5e877, 4f53d49, 9db5f4f, c15d68d2, ae1ab201 and fb5d2ac4 appear in gate.log lines 29-49 and 1916-1955, and scan-pre-check.log lines 38-50.
- **Log hashes.** e349806a appears in negative.log; c4df4908 in run.log line 3330; 3599c604 and 170f3d48 in scan-pre-check.log.
- **Test counts.** 513 and 529 (gate.log line 2755); 947 (test.log line 4797); 17 and 16 (test.log lines 4302 and 4308); six plants (test.log line 4791).
- **Corpus figures.** 27 of 27 (run.log lines 2889 and 3004). The entry totals 843, 27, 17 and the zeros (run.log lines 2893 and 3008). 2096 (run.log line 2847). 16 flags (run.log lines 2817 and 3156). 706 pairs (run.log line 4400). b018e52 (run.log line 34). ed4c55f07c69 (run.log lines 2081 and 3090).
- **Other strings.** The quoted message strings, the records' file names (run.log lines 3324-3326 and 4479-4484, whose hashes match the input records), and "RESULT: PASS" (scan-pre-check.log line 2707) all appear.

Items with no primary:

1. **"ca722f25..."**, the hash of scan-pre-check.log (report-draft.md line 18). No log prints it. It matches the input file's sha256 as I computed it, but that computation is my own, not a primary.
2. **"Planning's question Q1"** (report-draft.md line 23). The label "Q1" appears in no primary. The working ruling says only "Asked by the implementing session".
3. **The fork's rm -rf disclosure** (report-draft.md line 26): a pre-existing `gen/__pycache__` directory removed by a fork, "no data lost". No log or record contains it; it is working state. The disclosure itself states that this broke the maintainer's rule. It is the absolute case in `spec-0024-committed.md` P2 ruling 1 (lines 643-652): never on anything that existed before the run. Planning should rule on it. The second half, "no logged script runs rm -rf on anything it did not create", is supported by scan-pre-check.log lines 2682-2706.
4. **"Nothing is pushed"** (line 3) and **"the final stage runs after the check"** (line 18) are working-state or future statements, not in any primary. Consistent with that, the status lines in every log show only untracked records and planning's spec.
5. **"each at its first attempt"** (line 12) rests on RERUN_NOTE being unset in each log's header (gate.log line 4, test.log line 4, negative.log line 3, run.log line 8, scan-pre-check.log line 5). That shows absence, not a positive record.

No disagreement with any figure: every number the report states that a primary prints matches that primary.
