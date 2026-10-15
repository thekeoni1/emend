# Independent check 1

## Inputs read (sha256)

| File | sha256 |
|---|---|
| spec-committed.md | 157f785d886afc27bbef2f05bdf8b43e5acb55dbcca37d76766e357efd857cb2 |
| spec-working.md | 27a100abc5694ebf88c7493dd233569bb06f551fe09b089bda10382f28fbb5dc |
| diff.patch | a318e2b225b0c28fd662e1781e478fa19595d997ece7cade11486fec952508d1 |
| commit-messages.txt | f2718f5d0d02e6e1a3f84c2409bbaa336b6f95b42ba9c4e049ea07d10279833a |
| logs/gate.log | b6138cb8fb9332fd5e98b8e3987184e5c06122302521d4844e8288d69f4a86bb |
| logs/test.log | b8565e6ff72a700f53aedc0f026e9152f77a5ef715848ddd7228f747d542ee28 |
| logs/negative.log | bc488c73cbbba0b3ed96b31414991de815419bd067d57b1ccae8fa922ca0f99b |
| logs/size.log | a4c727733bc6dd2ce4bb86e8dc658cfa98c7b54359a2c6884995fa81b11cbc74 |
| logs/run.log | d3890e76fae760f6bbdeb722c5ed371d9f267440ab124c366930d1b7e96c3d18 |
| logs/scan-pre-check.log | b4535cdba3e584af13dfe1bca429ff7623265c89e08cfc129f12a928d5450ad1 |
| records/size-m5p5-2026-10-15-a8bfe71.md | fa7b6452f3e0f6bcee14880c01a08c09cba448f36479f6cd9146d6b9f10e65e3 |
| records/api-review-m5p5-2026-10-15-a8bfe71.md | 5e0a6b60bdeddb3d0e4c66fbff55130b3638b9e500f828102f3c1c4789ea1902 |
| records/m5p5-p4-chartmore-...md / .txt | c618417d4c756fc5e68898a7f8d754e5a05d270e4fc3384d2537a2841333fc22 / fd96db4930f33a9811b369b03128ac2ce71ab8d03d4eb12525044c8c63b2b205 |
| records/m5p5-p4-chart3d-...md / .txt | ec17e28364fdd6af839c5fc39b57947920a56c89e0f8d33ce24a6e8fbf7cdecc / 76b651895186e6b045a801ac7a8edf0fcfbe849f11c76ed418ff8cb327c2f5a3 |
| records/m5p5-p4-chartlabels-...md / .txt | 3d7dcda158f603c5b324dc67eca20662fabd0a339fd980b3884d303b36dde7ed / f8df60292b1fc0722c2d36ee8359be59635e2786191cc32353bc183d08858c17 |
| records/m5p5-p4-removechart-...md / .txt | fec28063bebcf10c965fdc01d469586c4584ae2d7d3302f31e3761c2d4659e80 / 9cbe40314a71874c1008dcd180d92d774b7167f45fb3e0e20eeb1981d482c322 |
| records/m5p5-p4-charts-...md / .txt | 0c5c177ab1e031fc2480a219c2d7d21da65caf783ba7c6eef59871d7c70f5fbc / 6c4c365e5be7c56e1dbccdcebe107143b53379ba06b39075e6dd8a9ef0324adf |
| records/m5p5-open-2026-10-15-a8bfe71.md / .txt | 4d4ee8bcfb4f0303364aedb94720f3b145ee0865ccf56510f274e904420fd5eb / 2144b94268ac934b4ecde949cce19606299a9afbb19b22f882cc253c5fa6c385 |
| report-draft.md | 6402fb21b5cb98c38f137b0c270f66d4f68f780b16608fe8f34471f36fda239c |

## 1. Does the diff do what the spec (with its rulings) says for P4, and nothing else?

**Verdict: YES WITH NOTES**

The diff covers 7 files, 411 insertions and 38 deletions. I recomputed this from diff.patch, and the per-file counts match the report's numstat.

- **Doc commit (0bd800d).** The specs/0036 diff (diff.patch 137-361) and the ROADMAP P3 row DONE (diff.patch 17-28) are planning's documents. Gate.log 6027-6040 hashes the doc commit's spec blob as 157f785d..., which equals spec-committed.md. Commit-messages.txt 57-58 gives the same hashes.
- **Ruling O carry, all in the probe commit.**
  - (a) The package.rs Limits bullet now names a format code and a label format as texts of at most 32767 bytes, "checked as a text and nothing more", and the colour as a `u32` at most `0xFFFFFF` written as six upper-case hex digits (diff.patch 45-66).
  - (c) The `category_axis_format` and `value_axis_format` doc comments name the 3D pie (diff.patch 71-85).
  - (b) One new test, `a_chart_removal_on_a_part_the_same_write_adds_an_image_to_is_refused`, sits under `#[cfg(feature = "images")]` in the chart_write tests. It covers both orders and asserts NO_CHART (diff.patch 99-136). The catalogue's NO_CHART_AT entry names it (diff.patch 34-42). The module header also gains "rulings L and O" (diff.patch 90-98).
- **The probe, under "The probe" and ruling R** (diff.patch 362-644).
  - It takes a last `<chart kind>` argument, index 0 to 12, over `PLOTS: [NewPlot; 13]` in declared order. A bad index is refused with exit 2.
  - It runs four steps: part 4's add then `reopened`; `remove_chart(&first, 0)` then `reopened`; the second `add_chart` then `reopened`; then `charts`.
  - Options per kind:
    - **Combined plot:** `Line { Standard }` on a secondary axis, with title and format, only on Bar or Area.
    - **OutsideEnd:** only on clustered Bar, Pie and Pie3D.
    - **Colour:** on every kind except Stock.
    - **Series count:** 1 on a pie, 3 on a stock, 2 otherwise.
    - **Sizes:** only on Bubble.
    - **Axis titles and formats:** none on pie, 3D pie or doughnut; no category format on scatter or bubble.
    - **Labels:** value labels with a format on every kind.
  - Index 0 gives 2 series plus the combined line's 1, which is 3. That matches `charts 1` and `chart_series 3`.
- **CHANGELOG:** one "Added" bullet citing only `specs/0036-charts-write-second-set.md` (diff.patch 5-16).
- **Nothing else:** the gate reports no manifest and no Cargo.lock in the phase diff (gate.log 6131) and no base test edited (gate.log 6684).

Notes:
- (i) Ruling R says "every index writes and reads back", but no primary runs indices 1 to 12. The size log runs index 0 and the refusal of index 13 (size.log 11342, 11464-11468). The report labels its per-index figures as dev measurements with no primary (report 117).
- (ii) The probe sets each kind's options with one 13-entry `PLOTS` array plus `const fn` and `matches!` per kind, rather than thirteen separate per-variant option constants. This does what the ruling requires; it only bears on the report's wording "thirteen per-variant constants" (report 40).

## 2. Do the logs contain every line the spec and its rulings require for P4?

**Verdict: YES**

- **Charts cost and writer cost:**
  - size.log 9397: "sections 1075632 - 970208 = 105424 ... in M5 part 4's record 85160".
  - size.log 9398: "sections 105424 - 85160 = 20264 ... above 48000 section bytes, a FINDING the report names: no".
  - Gate tells the figure: gate.log 7233 "the difference ... 20264; told to planning here, judged in the size log".
- **Images movement, judged per ruling S:**
  - size.log 9399: "moved by -11152; expected 0, a growth bound of 2000, a shrink accepted (ruling S): shrank 11152, accepted". Also size.log 9418 and 9494, and the control at 9420.
  - Measurement across P3's crate commit (size.log 11471-11553):
    - at 1bb8e02: 1038424, sha256 29c9d2d1..., matching M5 part 4 P3's binary byte for byte;
    - at 4836d14: 1027272, sha256 255fce29...;
    - "P3's tool commit changes comment lines alone among the images probe's sources: yes";
    - "the two builds print the same lines and write the same three outputs, byte for byte (ruling S): yes".
  - The size record attributes the shrink to 784e3c0 with figures and sha256s (size record 216-233).
- **Core growth:** size.log 9388-9389 "sections 970208 - 970208 = 0 ... above 2000 section bytes, a FINDING the report names: no".
- **Budget:** size.log 9394-9396 gives 436992, "21760 under the ceiling".
- **Other probes:** size.log 9495 "1 of 28 (expected 0 but for the images probe's accepted movement)".
- **Probe output:** the expected thirty lines with `charts 1` and `chart_series 3` (size.log 11371-11372, 11455). The second output is the format probe's byte for byte (size.log 11461).
- **API listing:**
  - test.log 5146, 5180 and 5396: "rows 756; {'keep': 756}; differing from M5 part 4 P3: 1 gone, 23 new; gate-widened 3, 0 of them pub".
  - The record names the 1 gone row and the 23 new rows, and lists the gate-widened items (api record 12-60).
- **Negative log:**
  - 128 constants at the phase commit and at the base (negative.log 4442).
  - N1 to N5 with controls B0 to B3 (negative.log 5121-5137).
  - Unreachable messages stated with their covering tests, including the new ruling O test (negative.log 4618-4648).
- **Run log, samples:**
  - All five samples ran on both targets with the size log's release binaries (run.log 5268, 5347, 5487, 5566, 5819, 5861, 6008, 6050, 6197, 6239).
  - Each reads "byte-identical to the earlier run's, by sha256, 27 of 27 ...: yes" (run.log 5672, 5702, 5739, 5769, 5894, 5924, 6083, 6113, 6272, 6302).
- **Run log, keep and open check:**
  - Keep: run.log 8655-8657, "rows: the regenerated keep 1435, the record 1435"; "... 0; ... 0"; "... differ in the keep: 0".
  - Open check: run.log 8198 and 8327, "pairs 1435; lo_in ok 1423 fail 12 error 0; lo ok 1424 fail 11 error 0"; run.log 8336, "no verdict and no class ... moved: yes".
  - Open record installed: run.log 8372.
- **CHANGELOG check:** run.log 6598 "cites specs/0036-charts-write-second-set.md alone and names the feature charts ...: yes", and run.log 6371 "sentences 41 with-digit 34 cited 34".
- **Test log:**
  - E figures 515/617/551/657/720/1096 on both targets (test.log 4025-4072).
  - W 1320 against P3's 1319 plus 1 (test.log 4090-4093).
  - All fifteen plants caught (15 "plant P*: found once ... yes" lines; P15 at test.log 5431 and 5706).
- **Run endings:** each of the five logs ends "... log checks all as expected: yes" with stderr 0 bytes. The scan pre-check ends "RESULT: PASS" with "structural checks failed: 0" (scan-pre-check.log 4474-4476).

## 3. Does every number and quoted string in report-draft.md appear in a primary?

**Verdict: YES WITH NOTES**

- **Recomputed sha256s.** Every sha256 the report gives for a file I hold matches my recomputation:
  - spec-committed 157f785d (report 5);
  - spec-working 27a100ab (report 10);
  - diff a318e2b2 (report 30);
  - the five logs (report 66-70);
  - all fourteen records (report 91-104).
- **Hashes I cannot recompute but that appear in a primary:** ROADMAP b582c49a (commit-messages.txt 58) and the full commit hashes (commit-messages.txt 1, 16, 54).
- **Numbers and quotes:** each one checked against its primary as listed under questions 1 and 2:
  - diff counts: 7 files, 411/38, per-file counts (my recomputation from diff.patch);
  - size figures: 105424, 85160, 20264, 57064, 68216, -11152, 1038424, 1027272, 436992, 21760, 458752, "1 of 28";
  - test figures: E, W 1320/1319, 66/62;
  - catalogue: 128;
  - keep and open-check counts: 1435, 1423/12, 1424/11;
  - end times;
  - every quoted log line.
- **The base commit's subject** appears at run.log 58 and scan-pre-check.log 46.
- **ASCII and trailers.** report-draft.md and commit-messages.txt have 0 non-ASCII bytes and 0 U+2014. Each of the three commit messages ends with the Co-Authored-By line only, with no Claude-Session line (commit-messages.txt 14, 52, 60; gate.log 5925-5934 agrees).
- **Placeholders:** RECORD_COMMIT, SCAN_SHA and INDEPENDENT_CHECK are present as expected.

Unsupported statements not labelled "no primary":
- Report 3: "Nothing pushed." No log or record shows the push state.
- Report 21: the record commit's subject "M5 part 5 P4 record: logs, the size and API records, the sample and open records, the check" is not in any primary yet. It is a planned row, like RECORD_COMMIT.
- Report 118 (finding 6): "My notes said 150 messages by another count." This is the implementer's own note with no primary. The log's figure of 128 is supported.
- Report 40: "thirteen per-variant constants" overstates the code's form (see note (ii) under question 1). The code has one 13-entry `PLOTS` array, with the options derived per kind.

Additional note, not in the report: the 17eaa26 commit message says "No log prints a count for this commit", but size.log prints that probe's `charts 1` and `chart_series 3`. The gate's and the scan's count rule accepted the message (gate.log 5942-5943; scan-pre-check.log 4064-4065), so I record this as a wording observation only.

Disagreements with the implementer's report:
- Report 40 says "thirteen per-variant constants"; the code has one 13-entry `PLOTS` array with the options derived per kind by `const fn` and `matches!`.
- Report 3 ("Nothing pushed.") and report 118 ("150 messages by another count") have no primary and should be labelled "no primary".
- Ruling R's "every index writes and reads back" rests only on the report's dev measurements (report 117). No logged run covers indices 1 to 12; planning may want that stated where the report presents the probe.
