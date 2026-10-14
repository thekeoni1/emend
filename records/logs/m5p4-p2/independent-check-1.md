Independent check 1

Scope: only the files under `<phase>/check1/`. I used only sha256sum, grep, sed, awk and wc, and changed nothing. I did not read git or any other file on disk.

## Question 1: Does the diff do what the spec's P2 and rulings G to K say, and nothing else?

**Verdict: YES with notes.**

What the spec asks for (spec-working.md lines 541-559, "Applicability" 223-232, "The chart part" 291-386, "Harness runs" 407-473, rulings G-I 692-713, J-K 717-730), and where the diff does it:

- **The three new chart kinds.** `NewPlot` gains `Pie`, `Scatter { style }` and `Area { grouping }` (phase.diff 129-147).
  - Pie: `<c:pieChart><c:varyColors val="1"/>`, then the series, then `firstSliceAng 0`, and no axes (phase.diff 194-198, 260).
  - Scatter: the `scatterStyle` with all six values, `xVal`/`yVal` as `numRef` (phase.diff 199-212, 239-251), and two `valAx` elements. The x axis is id 10 at `b` with no gridlines and `crossBetween` midCat. The y axis is id 20 at `l` with gridlines and midCat (phase.diff 260-283, `value_axis_bytes` 318-343).
  - Area: the grouping is written, followed by axis ids 10 and 20 (phase.diff 213-219).
  - All of this matches spec lines 343-386.
- **Applicability.** Area `Clustered` is refused, as Line's is. A pie is refused unless it has exactly one series and neither axis title (phase.diff 178-187). Scatter accepts every style. This matches spec 223-232.
- **Bar and line output unchanged.** The value-axis bytes were refactored into `value_axis_bytes`. For bar and line the call passes `b"20"`, gridlines on, `crossAx` 10 and `between` (phase.diff 300-309), which gives the same bytes as before. run.log 4391 and 4461 confirm it: the add-chart rerun is byte-identical to P1's on both targets, 27 of 27.
- **Ruling G and the catalogue.** In package.rs the diff adds:
  - the four chart messages with their tests (phase.diff 21-53);
  - ruling B's pairing under "no image at that position" (phase.diff 9-12);
  - a charts-write Limits section covering what is written and touched, the text and reference limits, no cache, and the shared anchor writer (phase.diff 64-89).

  The `CHART_TEXT_INVALID` doc comment is corrected (phase.diff 112). The catalogue test now reads `chart_write.rs` (phase.diff 364-426). negative.log shows each of the four messages in the catalogue at the phase commit and absent at the base.
- **Ruling H.** The Limits section states the strict case: transitional drawing and chart part (phase.diff 85-88).
- **Ruling I.** This is not a diff item. It is carried in the logs: test.log 57 and 186-187 keep the `<api>` root line.
- **Ruling J.** References use `max_row.max(1)` (phase.diff 827). Anchors use the real M: `max_row + 2 + 16k` (phase.diff ~858-862). The test `the_chart_kinds_sample_on_an_empty_sheet_takes_row_2` asserts A2:A2 and B2:B2 and anchor rows 1+16k counted from 0 (phase.diff 1139-1184).
- **Ruling K.**
  - `one_after` counts the `charts(W)` entries with `Anchor::OneCell` (phase.diff 926).
  - The tool test for the not-wsDr refusal renames `xdr:wsDr` to `xdr:userShapes`, then asserts exit 2, the DRAWING_UNREADABLE message, and no output file (phase.diff 1186-1214).
- **Tests the spec requires.** All are present:
  - in package/tests/chart_write.rs: the pie with one series, the pie refused for two series or an axis title, the scatter with each style, the scatter with no categories, and the area chart with each grouping and Clustered refused;
  - the tool's line test (phase.diff 1083-1137).
- **The kinds sample matches "Harness runs".** It makes six calls in the specified plot order, titled "emend kind k", sized 480 by 288, legend Right, with no axis titles on the pie and "emend x"/"emend y" on the others (phase.diff 808-900).
- **Nothing else.** gate.log shows:
  - the tree outside the phase's files equals the base's;
  - no manifest, lock file or `scripts/` change;
  - package.rs changes only `//!` lines (69 of 69);
  - package/tests.rs changes only the catalogue test (19 lines);
  - the `Error::Package` message set is unchanged at 148;
  - the pub fn/const list is unchanged at 204 on both sides (gate.log 5944);
  - exactly eight new tests, no test gone (gate.log 6066).

Notes:

1. **The read-back check is stricter than the spec.** The tool's `kinds_read_back` compares the whole `Chart` plus the anchor (phase.diff ~950-970 and `chart_as_read`). The spec lists a subset of fields. This exceeds the spec rather than departing from it.
2. **The refactor is inside the phase's scope.** `element_bytes`/`value_axis_bytes` refactor P1 code rather than only adding to it. Byte identity is proven for bar/line (run.log 4391, 4461) and the P1 tests still pass, so I count it in scope.
3. **Doc-comment edits.** The doc comments on the `NewChart` and `NewSeries` fields are reworded for scatter and pie. These are documentation of the new kinds and fall within P2.

## Question 2: Do the logs contain every line the spec requires for P2?

**Verdict: YES with notes.**

- **Gate, as P1's** (spec 536-538 and ruling D):
  - The six feature sets on both targets each match the FACTS figure: 515, 578, 551, 616, 681, 1055 (gate.log ~6305-6560). Chart tests are 40 and chart-write tests 23 or 25 where expected.
  - Clippy and doc are clean for all six sets. check.sh shows nine PASS lines.
  - The pub fn/const listing matches at base and phase commit (gate.log 5944).
  - Every commit message is ASCII with the Co-Authored-By line only, and passes J3 (gate.log 5341-5346).
  - Stderr is 0 bytes and the last line is "gate log checks all as expected: yes" (gate.log 6675).
  - No prune is required in P2, and gate.log 64-65 says none was done.
- **Test** (both targets, six sets):
  - Each set equals P1's figure plus the tests added that the set compiles (test.log ~2864 onward, the E lines for host and i686).
  - Workspace: 1269 = 1261 + 8 (test.log 2923).
  - Plants P1-P15 each read "found once ... as a set: yes".
  - The unplanted control passes 25 of 25 and 3 of 3 (test.log 3138).
  - The tool's line test is among the three tool tests.
  - Last line "yes" (test.log 3401).
- **Negative** (spec 552-554, ruling K):
  - N1, the kinds sample on a drawing with a userShapes root: exit 2, empty stdout, stderr exactly "error: a drawing part or its relationship cannot be read", no output file (negative.log 3772-3782).
  - Control B0: exit 0 with the expected `chartkinds` line (negative.log 3767-3768, 3781).
  - Last line "yes" (negative.log 3815).
- **Run** (spec 554-558):
  - Both samples on both targets over 27 workbooks, with lo and lo_in ok 27 for each label.
  - Kinds read back 162 of 162 (run.log 4265).
  - Host and i686 outputs identical, 27 of 27 (run.log 4271).
  - Families as expected (run.log 4281).
  - Add-chart rerun byte-identical (run.log 4391, 4461).
  - The keep's 19 new pairs, 0 gone and 0 changed (run.log 4525 onward).
  - Open check: no verdict moved (run.log 6005).
  - Four records installed. Last line "yes" (run.log 6085).
- **Records:** `m5p4-p2-2026-10-14-e86da06.md` and `.txt` hold the chartkinds run, and `m5p4-open-...` holds the open check (records/). No cell contents appear. Workbooks are cited by id, producer and tags.

Notes:

1. **scan.log is still pending.** The spec's "Phases" list requires it, and only the unlogged scan-pre-check.log is here (PASS, 0 hits, 0 structural failures, and PLANNING_SPEC matches 40f0dec4 at line 72).
2. **The add-chart rerun has no record pair of its own.** Its proof is the per-row sha256 comparison in run.log (4351-4461). The spec names one record pair, `m5p4-p2-...`, so I read this as compliant.

## Question 3: Does every number and quoted string in report-draft.md appear in a primary?

**Verdict: YES with notes.**

**Recomputed sha256** (all match the report):

| File | Computed sha256 | Report line |
|---|---|---|
| gate.log | cd8d7b9c2afa427b51cef11679e713b13c7c247bc95792a88a2434f567a36e72 | 63 |
| test.log | 8f60961132fb4fede0b0bedc4a9edbabe9f2c0cc557d976f3e9705df3342f469 | 64 |
| negative.log | 04ec8e0d4b8484dc865afa5e462527b7c328d99f650afe071a163b463a2e5a3c | 65 |
| run.log | 7a4e820320c63fcf48ea8e053738f0d71a7eab9a4220993045de0d02be03556f | 66 |
| m5p4-p2-2026-10-14-e86da06.md | f7d6ba2e3de6252647b6235fee4471e1557323c9d554fe9b958c2b19783ba5a9 | 114 |
| m5p4-p2-2026-10-14-e86da06.txt | ae03a95c7ae83a5a954e11b427d79359338d732819c3345fcc10fdf7f268415e | 115 |
| m5p4-open-2026-10-14-e86da06.md | 73e3dd4dfa38a268bde16e31fe634ff5d69733bd4c5c23a4c3c9fa2395dd544f | 116 |
| m5p4-open-2026-10-14-e86da06.txt | 613cbf0b6ae1bb68a5cd8f71a65b093c3e305b6be2aa06fa146bfd05552e518c | 117 |
| spec-committed.md | d9d65515e3b72ab297c98b6fa257d24f8b8176fcafed509414ffab62e5e6233a | 5 |
| spec-working.md | 40f0dec4f791d00931ad3522649bcf12136d6fbd09ab20294becdd4d2607575f | 7 |

**ASCII:** report-draft.md has no non-ASCII bytes and 0 occurrences of U+2014. commit-messages.txt is also ASCII.

**Statements checked against a primary:**

- **Line 11:** base d8d4e66 and its subject: gate.log; scan-pre-check.log 46.
- **Lines 15-17:** commit hashes and subjects: commit-messages.txt 1, 24, 58; gate.log.
- **Lines 20-28:** 6 files, 919 insertions and 36 deletions, and each per-file pair. I recounted these from phase.diff: package.rs 68/1, chart_write.rs 151/27, tests.rs 14/5, tests/chart_write.rs 252/2, main.rs 305/1, null_rewrite.rs 129/0.
- **Line 37:** 204 pub first lines: gate.log 5944.
- **Line 44:** 5 tests, none gated: gate.log 6066 onward; test.log 1257.
- **Line 45:** the 3 tool tests and the ruling J and K tests: the diff and the gate.
- **Lines 72-73:** the scan pre-check figures: scan-pre-check.log tail.
- **Lines 77-79:** gate figures, the J3 lines and clippy: gate.log.
- **Lines 81-89:** test figures 573+5, 611+5, 676+5, 1050+5, 515, 551, 1269 = 1261+8, 40, 23 or 25, the plant sentence, and 25/25 with 3/3: test.log.
- **Lines 91-93:** the quoted negative lines: negative.log 3781-3782, plus "N1: no output file: yes" and ND35.
- **Lines 96-109:** the run figures (27, 162, 17 and 10, the families, the quoted lines, 1201, 1182, 19, 0, and 1189/12/0 with 1190/11/0): run.log 4265-4466, 4525 onward, 5867, 6005; the open-check .md Totals.
- **Line 56:** P15 drops `.max(1)`: test.log 2250 and 1748.
- **Finding 2's "46 insertions":** this matches the test's span in phase.diff (1139-1184, 46 lines), though the superseded commit itself has no primary.

**Statements with no primary:**

1. **Findings 1-5 (lines 121-132).** These cover the superseded commits a80b8e00, 80bfe179 and ca3b9551, dry run 1 of the gate and of the run, the `<phase>/superseded/...` directories, the catalogue-test dev failure, and the 26-workbook dev run. The report declares it at line 134: "No primary in the repository shows findings 1 to 5, except where test.log or run.log is cited." Partial support exists:
   - P15 in test.log 1748;
   - the four helper functions in run.log 2797-2803, marked "carried verbatim (change NVA)";
   - 27 manifest rows in run.log.

   The claim in finding 4 that "the generator now checks that every function is defined before use" has no primary in run.log, and the line-134 disclaimer covers it.
2. **Line 3, "Nothing pushed."** No log checks the remote, and the report does not declare that this has no primary.
3. **Line 70, "each exited 0".** The logs print their final "... checks all as expected: yes" line and 0 stderr bytes, but none prints the script's own exit status. gate.log's "exit: 0" is check.sh's status, not the gate script's. The report does not declare this.
4. **Line 37, "No new pub item".** The gate checks only pub fn and pub const first lines. phase.diff adds no `pub` line, which supports the statement. However, three variants are added to the existing pub enum `NewPlot`. These are public API additions, though not new items. The wording is defensible but could mislead a reader about API growth. Nothing declares a missing primary here, and the diff serves as the primary.
5. **Line 47, "the first sheet".** The spec and the tool use W, "the first worksheet part in `<sheets>` order". This is a small imprecision in wording, not a missing primary.

**Disagreements:** none on substance. The statements in items 2 and 3 above lack a primary and are not declared as such; the implementer should either cite a primary or mark them as having none.
