Independent check 1

Inputs read: `<phase>/check1/` spec-committed.md (sha256 4f9bc8ff..., recomputed), spec-working.md (sha256 6ecaf055..., recomputed), diff.patch (sha256 f2e0dc1c..., the same as gate.log's "the phase's combined diff" hash), commit-messages.txt, logs/gate.log, logs/test.log, logs/negative.log, logs/run.log, logs/scan-pre-check.log, the eight files under records/, and report-draft.md. FACTS.md was used only to find things, never as evidence.

## 1. Does the diff do what the spec (with rulings) says for P3, and nothing else?

**Verdict: YES WITH NOTES**

- **Types.** `NewLabels<'a>` and `LabelPosition` (nine variants) match "The call" exactly (diff.patch 682-758). So do `NewSeries::color` and `labels`, and `NewChart::category_axis_format` and `value_axis_format` (669-680). Both new types are re-exported beside `NewPlot` (123-131).
- **Rules (`plot_applies` and `takes_position`, diff.patch 556-640 of the chart_write hunk, file lines about 931-989):**
  - A colour must be at most 0xFFFFFF and is refused on `Stock` (ruling B).
  - `percent` is accepted only on Pie, Pie3D and Doughnut.
  - Bar takes Center, InsideEnd and InsideBase, plus OutsideEnd only when Clustered.
  - Line, Scatter, and Radar Standard or Marker take Center, Left, Right, Above and Below.
  - Pie and Pie3D take BestFit, Center, InsideEnd and OutsideEnd (ruling L).
  - Every other plot takes no position: Area, Area3D, Stock, Bubble, Bar3D, Line3D, Radar Filled and Doughnut (ruling L).
  - Axis formats are refused on Pie, Pie3D and Doughnut, and the category format on Scatter and Bubble.
  - The combined second plot goes through `plot_applies(..., AxisOptions::NONE)`, so its series' labels and colours follow its own kind (ruling L).
- **Check order.**
  - The texts run title, category axis title, value axis title, category format, value format, secondary title, secondary format (diff.patch 867-884).
  - Within each series: name, categories, values, sizes, then label format (1003-1022).
  - This matches "Applicability"'s order. The test `the_new_texts_follow_the_check_order` covers it.
- **Byte forms ("The chart part").**
  - `spPr`/`solidFill`/`srgbClr` is written as `{:06X}`, upper case, right after `tx`. The `a:ln` copy is added on Line, Line3D, Scatter, Radar and Stock (1106-1128).
  - The stock marker comes after `spPr`.
  - `dLbls` comes before `cat`/`xVal` in the order numFmt, dLblPos, showLegendKey, showVal, showCatName, showSerName, showPercent, showBubbleSize. The P values match: bestFit, ctr, inEnd, inBase, outEnd, l, r, t, b (1141-1180).
  - The axis `numFmt formatCode="F" sourceLinked="0"` goes through one helper. The series axis takes none (1259-1266, and the x/y axis edits at 1024-1053).
  - The ln form on a stock series cannot be reached under ruling B, and ruling L accepts that.
- **`remove_chart`.**
  - The signature is as specified. It records `Removal::Chart(index)` (807-835, 945-952).
  - The two constants are verbatim: "no chart at that position" and "a chart part cannot be read" (655-668).
  - `record_image_removals` became `record_removals`, shared under `any(images, charts)`. Root children are chosen by each list. Chart frames come from `chart::chart_frames`, the same `read_anchors` that `Package::charts` now uses (package.rs 275-430; chart.rs 526-628). That is the shared anchor predicate.
  - A frame whose relationship is missing, or whose part is absent or not `chartSpace`, raises CHART_PART_UNREADABLE at the write.
  - Decision 3 A: the worklist loop removes each unreached entry with its `Override`, then its rels part, and queues that rels part's internal targets (442-490). External relationships are filtered out.
- **Ruling K.**
  - `add_chart`'s doc now reads "P2 rulings G, H and J, P3 ruling L".
  - The first-plot comment names "P2 ruling G" for 3D and "P2 ruling J" for scatter (842-866).
  - The test module's doc reads "P2 rulings F to J" (1174-1185).
- **Tests.** Every test the P3 list asks for is present (diff.patch 1407-2395), and ruling B's stock-colour refusal moved here. Beyond the list there are user-shape reach, removed or replaced part refusals, the combined second plot's labels and colours, and the new-text order. All of these are in scope. Gate.log 6650-6672 lists 23 new tests (19 crate, 4 tool) and one base body edited by `color: None,` and `labels: None,`.
- **`--chart-labels-sample`.** It matches "Harness runs": the colours 0x1F77B4, 0xFF7F0E and 0x2CA02C, both NewLabels, the formats "#,##0", "@" and "#,##0.00", and the read_back conditions (diff.patch 2780-2960).
- **`--remove-chart-sample`.** It matches "Harness runs" (2960-3245):
  - It uses a two-cell anchor, adds the chart, writes into memory and reopens.
  - It removes the last chart, plus index 0 when W had a chart.
  - It prints the ten keys in spec order and asserts `charts_after` and `charts_other_sheets`.
  - Ruling M's assertion holds: with `kept_removed` 0, it checks uncompressed bytes per entry, allows created rels parts of W or W's drawing only when they hold no relationship, and allows a drawing that held no anchor to drop, with W, its rels and the content types changed.
- **Package header (diff.patch 18-122).**
  - The message catalogue gains the second set's option and text rules with their covering tests, and a "Removing a chart adds two" block.
  - "What it writes" is updated and points to "Out of scope".
  - A "What a removal touches" bullet is added, citing ruling M.
- **Other changes.** The probe and earlier samples only gain `None` fields. No manifest, lock or script changed (gate.log 6070-6083). Nothing else outside the spec.
- **Notes:**
  - (a) Possible gap in the limits. The header's "Texts and references" bullet still names only titles, axis titles, text names and references (unchanged context, diff.patch 2120-2125). The text-message bullet says format codes and label formats are "past its limit (Limits)", but the diff adds no limit wording for them, and the Limits section itself is not in the diff. I cannot confirm from these inputs that "limits for this part" is complete.
  - (b) The catalogue and the `remove_chart` doc say "no chart at that position" applies when the same write adds an image *or* a chart. The spec says "a chart", "as 'no image at that position'". The code refuses any addition on the part. That is consistent with the image path, but only the chart-addition case is tested.
  - (c) Doc nit: `category_axis_format` says "a pie, a doughnut, a scatter and a bubble take none", and `value_axis_format` says "a pie and a doughnut take none". Both leave out Pie3D, which the code refuses (diff.patch 672-680).

## 2. Do the logs contain every line the spec and brief require for P3?

**Verdict: YES**

- **gate.log.**
  - Clippy and doc are clean on six feature sets, and on the probe crate's sets (6742-6877).
  - Crate tests on six sets on host and i686 equal P2's committed figures plus this phase's: 515, 617, 551, 656, 720 and 1095 (6878-7140).
  - check.sh shows nine PASS lines, including host and i686 (6711-6721).
  - Probe: "sections 1071980; part 4's: 1055368; the difference ... 16612" and "against P2's: 1058244; ... 13736", told and judged nowhere (7175-7189).
  - "no prune this phase" (64-65).
  - It ends "gate log checks all as expected: yes".
- **test.log.**
  - W host and i686 each pass 1319 (3294, 3316, 3555-3557).
  - Clippy on six sets and the workspace, and doc on six sets, all have 0 diagnostic lines (3761-3803).
  - Plants P1 to P14 are each caught against an unplanted control (3883-4084).
  - It ends "test log checks all as expected: yes".
- **negative.log.**
  - N5 is refused through the binary: exit 2, "error: a chart cannot be read", no output (about 5003-5008, 5034-5035). The control B3 exits 0 with the tool test's line (4996-5001, 5025).
  - The two new messages are stated unreachable through the binary with exactly the two required covering tests (4537).
  - N1 to N4 are carried with B0 to B2.
  - It ends "negative log checks all as expected: yes".
- **run.log.**
  - Both samples ran on host and i686 over 27 workbooks: 27 ok, `lo` and `lo_in` ok 27 on each label, and outputs byte-identical across targets (4742-4921, 4961-5146).
  - Ruling M's harness rules come with controls, including the three "(ruling M): yes" accepts (4666-4731).
  - The keep was checked against `records/m5p5-open-2026-10-15-e576023.txt` (ceaf5e6a): 1435 against 1278, 157 new, 0 gone, 0 changed (5234-5402).
  - The open check shows findings 0, no verdict moved, and no new refusal (7025, 7163, 7324).
  - Six records were installed with hashes (4950-4960, 5175-5185, 7353-7356).
  - The working spec 6ecaf055 is printed.
  - It ends "run log checks all as expected: yes".
- **scan-pre-check.log.** Line 8 reads `PLANNING_SPEC=6ecaf055...`, equal to spec-working.md. It ends "structural checks failed: 0" and "RESULT: PASS".

## 3. Does every number and quoted string in report-draft.md appear in a primary?

**Verdict: YES WITH NOTES**

- **Recomputed sha256s all match.**
  - Logs: gate 1f07861f..., test 39bc06b2..., negative 2e11c1d2..., run eea06a2f....
  - Records: chartlabels .md b4d50241..., .txt 90ccabda...; removechart .md e5459272..., .txt fc3e318a...; open .md a0825862..., .txt 2144b942....
  - The record hashes also match run.log's install lines.
  - The spec hashes 4f9bc8ff and 6ecaf055 match the files. ROADMAP deab2d2f and the combined-diff hash f2e0dc1c are in gate.log and commit-messages.txt, and f2e0dc1c equals diff.patch's own hash.
- **Diff statistics.**
  - "7 files, 2469 insertions, 137 deletions" and the per-file numstat (report 27-36) are labelled "no log prints it", but diff.patch reproduces them exactly (code files: 248/77, 59/11, 305/41, 1081/4, 563/4, 209/0, 4/0).
  - The doc commit's 113/4 also follows from diff.patch.
  - The crate and tool commits' 6 and 3 files are in gate.log 6036-6052.
- **Other figures checked against the logs.** Every other figure and quote appears in the logs: 148/150 messages; edit lines 2 and base tests edited 1; 23 new tests; test names 1299/1322; the E figures; 62/65 chart-write tests; W 1319; plants P1 to P14; the N5 and B3 quotes; the run totals 798/0/72/0/97 and 784/56/18/12/7; 17/10 and 4/23; the families; the keep and open-check quotes; "three lines" for ruling M. The serialized timestamps (09:43:38 to 12:02:52) support "runs were serialized".
- **Labelled "no primary" (accepted as such):**
  - the by-commit shortstats 1667/122 and 802/15 (these sum to the diff's 2469/137);
  - disclosure 1 (250caaa and cfb83cc, the clippy lint, the amend and cherry-pick);
  - disclosure 2 (the dry-run failures);
  - disclosure 7 (the self-closing `wsDr`);
  - disclosure 8 (the dev runs).
- **No primary and NOT labelled:**
  - (i) Line 3, "Nothing pushed": no input attests it.
  - (ii) Line 22, the record commit's subject "M5 part 5 P3 record: logs, the labels, removal and open records, the check": prospective, and it sits beside the RECORD_COMMIT placeholder.
  - (iii) Lines 24-25, "replace an earlier local pair, 250caaa and cfb83cc": stated in the Commits section without its own label, though it points to disclosure 1, which is labelled.
  - (iv) Lines 128 and 224, "as the brief orders": the brief is not among the inputs. The spec's P3 order ("then the package header's catalogue") partly supports it.
- **Partial primary.** The split of the catalogue between the commits (lines 123-127, 224) is consistent with gate.log ("the tool commit changes package.rs's module comment alone", 63 lines of 75 changed `//!` lines in diff.patch). The existence of "the crate's catalogue test" is mentioned only in negative.log's script text (3254).
- **ASCII.** report-draft.md is pure ASCII: 0 non-ASCII bytes and 0 U+2014. commit-messages.txt is also ASCII, and gate.log 5850-5862 shows each message ending with the Co-Authored-By line only.

Disagreements with the implementer's report: none on substance. Notes 1(a) to 1(c) and the unlabelled statements 3(i) to 3(iv) are for planning to rule on.
