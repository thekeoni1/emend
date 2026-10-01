**Independent check, M2 part 4d P2** (checker directory `<phase>/check1`, inputs only)

**Input integrity.** I ran `sha256sum -c` against `<phase>/check1/inputs.sha256`. All 14 inputs match (OK): commit-messages.txt, gate.log, m2p4d-open-2026-10-01-f255959.md, m2p4d-open-2026-10-01-f255959.txt, m2p4d-p2-2026-10-01-f255959.md, m2p4d-p2-2026-10-01-f255959.txt, negative.log, phase.diff, report-draft.md, run.log, scan-pre-check.log, spec-committed-0022.md, spec-working-0022-p2-rulings.md, test.log. I read no other path.

---

### Question 1. Does the diff do what the spec says for P2, and nothing else?

**Verdict: YES, with minor reservations (below).**

**Doc commit.**
- The spec diff (`<phase>/check1/inputs/phase.diff` 1118-1336) adds the P1 rulings (1232-1269) and the P1 verification section (1271-1335). It also makes the corrections those rulings mark "Corrected above" (1126-1128, 1139-1146, 1150-1187, 1192-1196, 1204-1206, 1214-1224).
- ROADMAP's P1 row goes to DONE at phase.diff 9-10. That is what P1 verification ruling 5 asks (spec-working 997-999).

**Carry, P1 verification ruling 1 (2) (spec-working 967-977).**
- `comment_part_untouched` is at phase.diff 537-563. Its outcomes:
  - a removed part gets "an edit to a removed part" (548-555);
  - a replaced or added part gets "a cell edit to a part this write replaces" (556-561).
- It is called for both the comments part and the VML part (359-362). That is after the relationships are resolved and before either part's index or bytes are read (363-373).
- The six tests, one per case, are at 807-875.
- The catalogue cites the tests at 30-37.

**Planning ruling during P2, ruling 1 (spec-working 918-932).**
- The test `a_removal_emptying_a_comments_part_this_write_removes_is_refused` is at 1105-1117. It calls `remove_part` and then `remove_comment` of the only comment, and asserts "an edit to a removed part".
- The removed-part guard in `drop_with_relationship` (526-533) shows why only the carry's check can catch this path.

**`remove_comment`.**
- Public call: 177-201. Body: 224-229. It records the removal as a `None` edit at 227.
- Comment element skipped when its `ref` parses to the cell: 654-666, via `skipped` built at 380.
- VML `Note` shape skipped when `Row`/`Column` equal the cell's zero-based values: `skips` at 699-707, used at 765-769.
- `is_note` is now recorded: 695, 716, 724.

**"no comment on that cell".**
- The constant is at 581.
- A cell this write adds a comment on, or one named twice, is refused at 314-322.
- A cell with no comment in the part's file is refused at 399-401, using `found` (659). When the sheet has no comments part, the refusal is at 414-416.
- An addition after a removal of the same cell is the replacement case (305-310). Its cell goes into `wanted` (381-386), so it needs a kept comment (399-401).
- Catalogued at 45-47.

**An empty `commentList` drops the parts.**
- The comments part is dropped when nothing is kept and nothing is added (403-405). Its relationship, Override and entry go through `drop_with_relationship` (509-535).
- Case 1, the VML part holds no shape of another ObjectType: the VML part, its relationship and its Override are dropped (461-472). A `LegacyWork { drop: true }` is pushed, and the worksheet pass skips `<legacyDrawing>` (590-601, 85-88, 165-169).
- Case 2, the VML part still holds another shape: it is kept with the note left out and `legacyDrawing` untouched (473-476).
- Authors are never removed: the authors path in `comments_appended` is unchanged. Test 914-934 asserts the comments part equals the input minus the one element.

**`--comment-sample` in-memory removal.**
- `tools/emend-copy` diff, 1433-1452:
  - `remove_comment(part, cell)` on the reopened output;
  - `write_to` into a `Cursor`, then reopened;
  - `comments_removed` asserted equal to `comments_before` and `parts_removed` to `parts_before`.
- The keys are printed at 1403-1404. A failure removes the output (1391-1393). The module doc is at 1351-1357.
- The tool test's three expected lines gain the two keys (1460-1484).

**P2 tests listed in the spec (spec-working 748-757).**

| Spec test | phase.diff lines |
| --- | --- |
| Removal of one of two comments | 913-934 |
| Last comment beside a control | 936-975 |
| Last comment, notes only | 977-1015 |
| Absent cell / a cell this write adds / named twice | 1017-1036 |
| Removal and addition on different cells | 1038-1059 |
| Comments part missing, refused at the write | 1084-1103 |

The negative case (C7) is in negative.log, see Question 2.

**Beyond the spec's literal list, all judged in scope:**
- `a_removal_then_an_addition_of_one_cell_replaces_and_needs_a_kept_comment` (1061-1082). It is not in the P2 test list, but it pins a rule the spec states under `remove_comment` ("An addition after a removal ... refused as above only when the cell has no kept comment").
- The shapetype guard `!cells.is_empty()` (775). Without it, a removal-only write would add a shapetype. Needed.
- `block` = 0 when the VML part has no `idmap` and nothing is added (444-447). No allocation for a write that numbers nothing. The spec is silent; this is benign.
- `comments_rel` refactor (241-258) and the `RelTarget` cfg widened to `comments` (128-129). Internal, needed for the relationship id.

**Reservations and doubts.**
1. The doc comment on `record_comments` (phase.diff 276-280) does not match the code's order. It says the removed, replaced, absent or shifted part refusals "come first, then a removal of a cell this write adds a comment on or names twice". In the code, the adds/twice refusal (317-322) returns during the grouping loop, before any per-part check (335 onward). So a twice-named removal on a removed worksheet gets "no comment on that cell", not "an edit to a removed part". The spec sets no order between these, so this is a comment inaccuracy, not a spec breach.
2. A stale comment is left unchanged at phase.diff 143-144: "A comment rewrites the worksheet only when its `<legacyDrawing>` is created". The worksheet is now also rewritten when it is dropped.
3. The "another ObjectType" test (457-460) counts every non-`Note` shape as other, including a `v:shape` with no `ClientData`. Such a shape keeps the VML part and `legacyDrawing`. The spec says "a shape of another `ObjectType`"; a shape without `ClientData` has none. This is an edge reading. No test covers it, and the corpus facts do not say whether it occurs.
4. The tool asserts `parts_removed` by count only (1447-1451). The spec's "the created parts gone and the kept ones kept" is implied by equal counts, not checked by entry name.

---

### Question 2. Do the logs contain every line the spec requires?

**Verdict: YES for the four logs given. PARTLY overall: the final `scan.log` is not among the inputs (only its pre-check stage is), as expected at this point.**

**"Phases" (spec-working 654-687).**

| Requirement | Where it is met |
| --- | --- |
| Logs: gate, test, negative, run | All present |
| scan.log written last | Only scan-pre-check.log is present; its 2005 says "check answers present: 0 (stage pre-check ...)". The final scan is not in the inputs. |
| Every log pins the spec's hash | gate.log 23, 26; test.log 24, 27; negative.log 66-67; run.log 4, 1619-1621; scan-pre-check.log 8, 53, 1998 ("4 of 4") |
| Superseded attempts / reruns | RERUN_NOTE unset: gate.log 4, test.log 4, negative.log 3, run.log 8. No superseded run or keep: run.log 45 |
| Planning runs no cargo while a log runs | SERIALIZED lines: test.log 58, negative.log 44, scan-pre-check.log 55. gate.log 76: no cargo/rustc at entry |
| Run log names the record commit it compares against | run.log 33-37, 2949-2959 |
| Check prompt names `<phase>/check<n>` | This prompt |

**P2 (spec-working 741-762), with P2 ruling 1.**

| Requirement | Where it is met |
| --- | --- |
| Gate takes four code commits | gate.log 1733 ("1 planning-doc, 4 code"), 1899-1925 (each code commit's area; code commit 4 adds the removal case, 1922-1925) |
| Carry's check and six tests | gate.log 1926-1931 |
| Test log has the ruling-1 test | test.log 3556; carry tests run alone 3639-3683 (seven, incl. 3675-3681); V1 targets it 4013, 4024-4033 |
| Doc commit files at planning's hashes | gate.log 1787-1815 |

**Test log on both targets, feature off and on (P1's rule, carried).**
- Workspace: test.log 2498 and 3414 (868 each).
- Crate off/on, both targets: test.log 3595-3637 (512 off, 544 on; on adds 32 names, lacks 0).
- Repeated in gate.log 2378-2402.
- Comments module present in the workspace run: test.log 3437, 3459.

**Public fn/const lists with the feature off and on:** gate.log 2058-2234. "adds remove_comment alone" at 2224.

**Negative log: comments relationship naming a missing part, refused, with a control.**
- Fixture c7 (comments_part:0 with the comments rels): negative.log 1109.
- Run: 1161-1166.
- Check: 1188 ("C7: exit 2 ... COMMENTS_UNREADABLE's message"). Control: 1187 ("C2 ... the control for C7").
- The removal keys in lines: C0 1116, C2 1130, C4 1144.

**Run log: `--comment-sample` with the removal on both targets, removal's counts asserted, recorded.**
- Host 2425-2531; i686 2532-2642.
- Line-as-spec check covers the removal keys (script 600-601), with control 2406.
- All 27 lines as spec: 2520, 2627. Tool/comparison agreement on removed vs input counts (script 579-580): 2520, 2627.
- Record pair written: 2676-2678, installed 2827, 2829.

**"Harness runs" (spec-working 547-652).**

| Requirement | Where it is met |
| --- | --- |
| lo and lo_in ok, order preserved, files not identical | run.log 2464, 2571 |
| Entry expectations | "entry rows as P1 expects: 27", run.log 2520, 2627, 2479, 2635 |
| Comparison flags | run.log 2520, 2627 |
| A plant per flag | run.log 2644-2667 (10 of 10) |
| Host and i686 outputs identical | run.log 2669-2673 |
| Refusals reported as findings | 0 refused: run.log 2458, 2565 |
| Edit open check on the host over the keep | run.log 2832-3810 (keep 2938-2947; against P1's record 2949-3020; verdicts 3766-3784; installed 3802-3810) |

---

### Question 3. Does every number and quoted string in report-draft.md appear in a primary?

**Verdict: PARTLY. Every figure and hash is found. Four items in the "Finding before the logs" paragraph, one in the Logs section and one in Not done are not in any primary (list at the end).**

Line numbers below refer to `<phase>/check1/inputs/report-draft.md` (rd).

**Commits and base.**
- rd 3, f255959d494de9...: gate.log 1712, 1725, 1731; commit-messages.txt 1.
- rd 3, base 2d675da6... "(part 4d P1's record commit)": gate.log 1713-1716.
- rd 6-10, the five subjects and short shas: commit-messages.txt 2, 17, 34, 59, 77; gate.log 1721-1731.
- rd 6, specs/0022 3ef8b8a6... and ROADMAP.md c0dda773...: commit-messages.txt 79-80; gate.log 1791, 1796.
- rd 7, "comment_part_untouched, with the six tests": phase.diff 544; gate.log 1927.
- rd 10, "one test, per ... ruling 1": spec-working 918-932; phase.diff 1105-1117.

**Finding paragraph (rd 13).**
- "V1": test.log 4013.
- "pre-log plant check": spec-working 914.
- "left ... passing": spec-working 923-924 says "left all six tests passing". This is consistent with the report naming one of the six.
- "An addition replaces the part ... refused ... with the same message": spec-working 921-923.
- "In test.log, V1 targets the new test and bites": test.log 4013, 4024-4033.

**Log hashes (rd 16-19).**

| Log | Hash | Found at |
| --- | --- | --- |
| gate.log | 39e53b40... | scan-pre-check.log 1949; negative.log 958 |
| test.log | 3c586493... | scan-pre-check.log 1959; run.log 2833 |
| negative.log | d8bcaf1a... | scan-pre-check.log 1969 |
| run.log | 746db66a... | scan-pre-check.log 1979 |

- rd 20, combined diff 69d56850... "(gate.log)": gate.log 1829.
- rd 21, "RESULT: PASS": scan-pre-check.log 2162. "Before any check answer existed": scan-pre-check.log 2005.

**Figures.**
- rd 24, 868 on host and i686: test.log 2498, 3414, 4124. Previous 854: test.log 3488, 3490. "14 ... none removed": test.log 3492.
- rd 25, the quoted sentence: test.log 3556. "No test file is new": test.log 3558.
- rd 26, V1 to V8 found once, pass unplanted, fail planted: test.log 4033, 4046, 4057, 4068, 4079, 4090, 4103, 4116.
- rd 27, "adds remove_comment alone": gate.log 2224.
- rd 28, the tool test's three lines gain the two keys: gate.log 2291-2297. "No other base test is edited": gate.log 2297-2302.
- rd 29, C7 and the quoted message: negative.log 1166, 1188, 1109. "C2 its control": negative.log 1187.
- rd 29, "C0 0 0, C2 1 2 and C4 0 1": negative.log 1116, 1130, 1144.
- rd 29, NO_SUCH_COMMENT unreachable, covered by the named test: negative.log 1211, 1221-1222.
- rd 30, 27 rows, 27 ok, 0 refused, both targets: run.log 2458, 2565.
- rd 30, flags, agreement, line as spec on 27: run.log 2520, 2627.
- rd 30, "22 created=2 and 5 created=0": run.log 2522-2523, 2629-2630.
- rd 30, "70 and 50": run.log 2462-2463, 2569-2570.

**Records (rd 33-34).**

| Record | Hash | Found at |
| --- | --- | --- |
| m2p4d-p2 .md | ab644483... | run.log 2827; scan-pre-check.log 2006 |
| m2p4d-p2 .txt | 2f7ebdd9... | run.log 2829; scan-pre-check.log 2016 |
| m2p4d-open .md | eb85e156... | run.log 3806; scan-pre-check.log 2026 |
| m2p4d-open .txt | 38acfd01... | run.log 3809; scan-pre-check.log 2036 |

**Noted for P4 (rd 37-39).**
- "K counts v:shape only": spec-working 977-979.
- "D's allocation reads only the input's .vml entries": spec-working 967-971.
- "commentList before authors, or a root other than comments ... cannot-be-read": spec-working 979-981.
- The heading at rd 36 cites "rulings 1 (3) and (4)", but item rd 38 comes from ruling 1 (2), which (3) refers to ("with (2)'s rule", spec-working 979). This is a labelling imprecision only.

**Not done (rd 42).**
- fd31866c...: gate.log 25; test.log 26; negative.log 66; run.log 1620, 3824; scan-pre-check.log 9, 2060.
- "Nothing is pushed": not checkable from primaries (state claim).

**Not found in any primary:**
- rd 13, "validate()". The function name appears in no input (diff, spec, messages or logs).
- rd 13, "planning ruled (a)". No primary records an option "(a)". The ruling (spec-working 918-932) does not label options.
- rd 13, "a pre-log plant check in a scratch archive copy", and "A scratch probe showed a removal that empties the comments part takes the drop path". These describe pre-log work that no log holds, as the report itself says ("not a log"). The ruling corroborates the outcome ("the probe's shape", spec-working 926), but not the details.
- rd 15, "each at its first logged attempt". Corroborated, not stated as such: RERUN_NOTE/RERUN_FILES are unset in all four logs, and run.log 45 names no superseded run.
- rd 21, "It is kept uncommitted on the disk". This is a state claim with no primary. No primary shows the pre-check log's own location.
