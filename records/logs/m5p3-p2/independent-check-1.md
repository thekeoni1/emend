Independent check 1

Inputs read: everything in <phase>/check1/. Nothing was modified.

1. Does the diff do what the spec (P2 plus rulings M to S) says, and nothing else? Verdict: yes, with minor notes.

Evidence:
- The API is as "The call" says: the four Chart fields, plus ChartAxis (id and crosses i64), AxisKind, AxisPosition and LegendPosition (phase.diff lines 136-190). They are re-exported in package.rs (line 2096 hunk).
- read_axis requires axId, axPos and crossAx. It refuses an axPos outside b/l/r/t, a crosses outside autoZero/max/min or with no val, and a numFmt with no formatCode (N). read_plot_area refuses a plot axId that names no axis (C, the "joined" check).
- Legend: `None | Some("r") => LegendPosition::Right` (O).
- pivotSource with no name is refused: `name.ok_or(CHART_UNREADABLE)` (N).
- Style: read_style_choice enters only the first Choice and takes c:style or c14:style; the Fallback is walked past (R).
- min and max are parsed by the generic val(), which is f64 (P).
- digest_titles still collects chart titles only (Q).
- M: text_of's mapping is now `Error::Xml(_) | Error::Io(_) => error, _ => refusal`. tests.rs adds chart.rs as the tenth source.
- Header catalogue: it gains the message by feature, and Limits gains the title rules, the cached values, the style/colour/user-shapes parts and the Fallback.
- Carry: a_negative_axis_id_joins_a_negative_cross_axis (E) and a_multi_level_string_reference_without_a_cache_reads_none_cached (G).
- S: plots_xml and simple_body gain catAx 10 and valAx 20. The signed-id test gains three axes, and its assertion is unchanged. The tool test's P1 fields are unchanged in the expected line.
- Every P2 test the spec lists is present.
- The tool adds 12 counts (38 to 50), digest_numfmts and digest_sources in the spec's order, and asserts axis_cat + axis_val + axis_other = axes.

Notes (none breaks the spec):
- (a) Ruling R allows c:style inside the Choice, but only c14:style is tested.
- (b) The "Fallback not read" assertion holds only when the Choice has no style.
- (c) The header's Errors test list does not name the three ruling-N tests.
- (d) A repeated legend or pivotSource is last-wins, which the spec does not address.
- (e) M's claim that the behaviour is unchanged cannot be confirmed from the diff, because text_of's body is not in it.

2. Do the logs contain every line the spec requires for P2? Verdict: yes, except scan.log, which the spec runs after this check.

- gate.log: all five feature sets on both targets (lines 6039-6183), the pub fn listing at base and phase commit (5455), TMPDIR and mktemp with controls (45-61), the counting library's mktemp line (4964), PYTHONDONTWRITEBYTECODE and the PYB shape check (41, 74), no prune (11), the spec pin (81-86), and "gate log checks all as expected: yes".
- test.log: 515, 553, 589, 656 and 1028 on both targets, workspace 1237, "chart tests passed 38 of the 38", plants P1 to P14 each "exactly the table's", tool sample tests 3, clippy and doc.
- negative.log: B0 exits 0 with the tool test's line. N1 exits 2 with stderr "error: a chart cannot be read" and no output file (3570-3597).
- run.log:
  - "27 of 27" for kinds sheet_charts and for the P1 fields, host (4718-4719) and i686.
  - "the figures met: 22 of 22; the departed fields: none".
  - "files_identical yes 27" (4650) and largest_chart 28215.
  - Keep: 1103 pairs, 0 new, 1 changed, 0 gone, "no verdict and no class ... moved: yes", plus the edit open check.
  - Records named m5p3-p2-2026-10-13-27569d2.
- All four logs show RERUN_NOTE unset.

3. Does every number and quoted string in report-draft.md appear in a primary? Verdict: partly.

Verified:
- All log and record sha256s, which match sha256sum of the copies.
- Spec hashes 02485398... and 0db9d7d8..., and ROADMAP 424abcc7... (gate.log line 85).
- Commit hashes and subjects.
- 6 files, 871 insertions, 84 deletions (counted from phase.diff).
- 50 counts and 5 digests.
- Every test count, plant P13/P14, and every corpus figure and "in N" count.
- 28215; 1103; the changed pair and "ok ok none none"; records/m5p3-open-2026-10-13-54bf510.txt.
- The superseded shas f659089... and b30ebbd... (gate.log line 3133 and the spec); change NUM (run.log).
- Scan pre-check "total hits: 0", "structural checks failed: 0", "RESULT: PASS" (scan-pre.log lines 3884-3888).

Not found in any primary:
- "PLANNING_SPEC" ("the logged scripts were run with PLANNING_SPEC set to it"). No log contains the string; the logs only print the working spec's hash.
- Finding 2: "the script expected 21", "P1's 16 plus six new is 22", "run.sh 0dac7ccb became c47c757c" and "<phase>/superseded/run-dry-1". None of these appears. run.log says "superseded run directory named: none".
- Finding 1: "The tree change is chart.rs, package.rs's header and tests/chart.rs only". No primary shows a diff between the superseded and re-made commits.
- Ruling P: "`1e3`", "`inf`", "`infinity`" and "in any case". Spec ruling P names only `INF`, `-INF`, `NaN` "and some more", and no test or log shows these spellings.
- The per-commit file split ("The crate commit: ... The tool commit: ..."). phase.diff is a single combined diff; the split is plausible but not shown.

Wording issues:
- Negative: "the error line exactly "a chart cannot be read"". The logged line is "error: a chart cannot be read".
- Ruling N: "each also a plant" is loose. The crosses refusal has no plant of its own; it is caught by P11, which also covers the out-of-set value.
- "Chart tests 38 (P1's 27, the two carried, six new, three for ruling N)": 38 and 27 are in test.log, but the 2 + 6 + 3 breakdown is my own count from the diff. It is correct.
