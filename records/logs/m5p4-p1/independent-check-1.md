Independent check 1

Scope: I read only the files under `<phase>/check1/`. I did not use git, read nothing else on the disk and modified nothing. Line numbers below refer to these files.

## Hashes, recomputed with sha256sum

Every log and record hash in the report (`report-draft.md` lines 74-77 and 136-139) matches.

| File | Recomputed sha256 | Matches the report |
|---|---|---|
| gate.log | 6146e79adc9609c6f0ff1ea0b0a9a6994bd836442336df6f523dd6e0513c3bc9 | yes |
| test.log | 4961216a9f1c1ff542cc3f286e549953d4f92491abe6b4704016477093527449 | yes |
| negative.log | b45df4eb316d6d33869acaf1f31030f6bf84ce11ecd69b070ba40df6c454d06c | yes |
| run.log | edec3db57cf42ff9c2afa05ee590e30763844f81c2647e0970ec433f5a5b268f | yes |
| records/m5p4-p1-2026-10-14-981c00c.md | c4f4ac2fc5ebe3985c54f16c1de70b315933f237f0869f19a6f3c8a454cbadfd | yes |
| records/m5p4-p1-2026-10-14-981c00c.txt | e98f3507640e1dd8d09a46597bc1951ae71e78190ea8cb5893c4e61a23f059e6 | yes |
| records/m5p4-open-2026-10-14-981c00c.md | 1b4a1bf01fd867b3687316c714905c6a7cf1c43e6e1ecdf3207604cb7073a1a8 | yes |
| records/m5p4-open-2026-10-14-981c00c.txt | 8a5b81806779999af9fc7c94dfa12fd095a7ec2366ef3c3190371f2dbedfc404 | yes |

- **Spec files:** spec-committed.md hashes to 71f0bb68... (report line 5) and spec-working.md to 2353e988... (report line 12). Both match.
- **ASCII:** report-draft.md and the records contain no non-ASCII byte and no U+2014.

## 1. Does the diff do what P1 and rulings A to F say, and nothing else?

Verdict: YES with notes.

**What the diff contains**
- Code, counted from `phase.diff`: 7 files, 2466 insertions, 193 deletions.
  - package.rs 354/191
  - chart.rs 1/1
  - chart_write.rs 545/0
  - tests.rs 2/0
  - tests/chart_write.rs 1174/0
  - main.rs 305/1
  - null_rewrite.rs 85/0
- Doc commit: ROADMAP.md 17/6, specs/0034 54/0, specs/0035 641/0, which is 712/6.
- No Cargo.lock or manifest change (gate.log 5836-5839).

**Checked against the spec and rulings**
- **Regating and ruling A:**
  - Every drawing-writer item moves from `images` to `any(images, charts)` (phase.diff 113-1003).
  - One `drawing_edits: Vec<(part, ImageAnchor, NewDrawing)>` in record order (phase.diff 112-116, 177-238).
  - `NewDrawing` is Image or Chart, with `kind`, `extent` and `body`.
  - `anchor_bytes` now takes `(cx, cy)` and `body` from its caller (phase.diff 855-866, 928-933).
  - The picture body was moved into `picture_bytes` byte for byte (phase.diff 795-853 against the removed 891-933).
  - `CHART_NAMESPACE` becomes `pub(super)` (phase.diff 1000).
- **The call:**
  - `NewChart`, `NewPlot` (Bar and Line only), `NewSeries` and `NewSeriesName` derive Clone, Copy, Debug, PartialEq and Eq (phase.diff 1052-1115).
  - `add_chart` has the spec's signature (phase.diff 1180-1187).
  - The four messages are worded as the spec gives them (phase.diff 1027-1039).
  - The limits are 32767 and 8192 (phase.diff 1047-1049).
- **Ruling C:** the check order in phase.diff 1199-1254 is grid, then offsets/position/to-before-from, then size applicability, then size range, then no series, then grouping, then texts in field order. That is the ruled order.
- **Ruling F:** texts use `escape` and references use `partial_escape` (& < >). U+FFFE and U+FFFF are refused with the text message (phase.diff 1275-1300).
- **Chart part and graphicFrame:** they match "The chart part" and "Anchors" (phase.diff 1302-1532). That includes `overlap` only when stacked, the axes swapped for direction Bar, `macro=""`, and the zero `xfrm`.
- **Ruling E:**
  - The relationship uses `addition.kind()[strict]` with the sheet's `strict` (phase.diff 488, 534).
  - The chart part uses `part_bytes(form.strict)`, where `form` is the kept drawing's form, or the created transitional one (phase.diff 436, 466).
  - The frame uses `CHART_NAMESPACE[form.strict]` for both `uri` and `c:chart` (phase.diff 1508-1511).
  - The content type is unchanged.
- **Ruling B:** `record_image_removals` now takes `PartDrawings` (phase.diff 553), so a chart addition plus a removal on one part meets the existing refusal. Test at phase.diff 2735-2748.
- **Tool:** `--add-chart-sample` builds exactly the spec's call. Two-cell anchor from (M+2,1) to (M+18,9), editAs OneCell, and the spec's series, titles and legend, with no size (phase.diff 3627-3699). It asserts +1 on `charts`, anchors, chart parts and `two_after`, and checks that the chart reads back equal (3724-3743). It prints the nine fields (3706-3719).
- **Tests:** 20 tests (18 under charts, 2 also gated on images) cover every item on P1's list (phase.diff 1969-2748). The tool has 2 tests (3841-3925).

**Notes (nothing outside the spec, but)**
- **(a) Premature doc comment.** phase.diff 1034 says, for CHART_TEXT_INVALID, "[`crate::package`]'s catalogue lists it". The diff adds nothing to any catalogue, and the catalogue is P2's work (spec-committed.md 541; ROADMAP row, phase.diff 47). The comment is wrong at this commit.
- **(b) Ruling B's catalogue clause is not delivered in P1.** Ruling B says "the catalogue names the pairing under that message" (spec-working.md 621), but nothing in the diff does this. That is consistent with the catalogue being P2's, but planning should confirm it is carried to P2.
- **(c) Chart part form on a strict sheet.** When the sheet is strict and the drawing is created, the chart part is transitional (phase.diff 436; test at 2622-2642). This follows ruling E's "as the image does" and is noted only for planning's awareness.

## 2. Do the logs contain every line the spec requires for P1?

Verdict: YES with notes.

**Endings:** all four end with the required line.
- gate.log 7007: "gate log checks all as expected: yes"
- test.log 3416: "test log checks all as expected: yes"
- negative.log 3646: "negative log checks all as expected: yes"
- run.log 6458: "run log checks all as expected: yes"

**gate.log**
- Prunes of the four part 3 roots:
  - `du -sb` and `rm -r` with exit 0 for each (129-132, 193-196, 264-267, 311-314).
  - The kept-names check passes for each (125, 189, 260, 307).
- Six feature sets on host and i686, with pass counts against FACTS.md (6642-6891).
- Clippy and doc results (6504-6571).
- The `pub fn` and `pub const` lines with attributes, at the base and at the phase commit: 203 to 204, `add_chart` added under the charts cfg line (6114, 6285-6300).
- Commit-message counts "none" for all three commits (5621-5626).
- TMPDIR and mktemp checks with controls (10, 49-62, 5561-5562); PYB `-I -B` check (42).

**test.log**
- Rerun inputs and the superseded hashes (3-4, 48-53).
- `<api>` declared (63).
- W, F, I and E on both targets for the six sets (2611-2860).
- N, the added tests derived from the source (2864ff).
- P, plants P1 to P14, each "found once ... yes" (offset +3100: lines 37-270), with an unplanted control.

**negative.log**
- The N1 refusal with the exact message, plus control B0 (3594-3625).
- The four messages and their covering tests (3376-3425).
- The mktemp checks.

**run.log**
- `--add-chart-sample` on host and i686:
  - 27 rows, lo and lo_in ok 27 (4087-4092, 4166-4171).
  - 17 drawings created and 10 appended to (4121, 4200).
  - Read back equal 27 of 27 (4225).
  - Host and i686 outputs identical (4229-4232).
  - Families (4239-4240).
- The three image samples compared by sha256 against the three named records, 27 of 27, on both targets (4276-4713).
- The keep against records/m5p3-open-2026-10-13-d74ea74.txt: 1182 pairs, 79 new, 0 gone, 0 differ (4759-4856).
- The edit open check (6180, 6309-6318).
- The installed records (6426-6439).

**Notes**
- **(a) Plants are host-only.** They run on host only, with charts,images (test.log section P, line 3053 and the command at about 3117). The spec does not require them on both targets.
- **(b) No primary for dry runs.** No log shows the dry runs that "Phases" requires before each logged run. The only reference is the report's own finding 2, which says it has no primary.

## 3. Does every number and quoted string in report-draft.md appear in a primary?

Verdict: YES with notes.

**Confirmed against primaries**
- **Header:** lines 3-15, the hashes and the four logs printing the working spec (gate.log 336, test.log 96, negative.log 88, run.log 3960). PLANNING_SPEC is at scan-pre.log 8.
- **Commits:** lines 17-23, the hashes and subjects (test.log 34-41; gate.log 360-363; scan-pre.log 3572-3575).
- **Diff stat and the doc commit:** lines 26-37, from phase.diff by count and spec-working.md 604-608.
- **Delivery:** lines 41-59, from phase.diff.
- **Rulings A to D and F:** lines 63-66 and 68.
- **Gate:** lines 86-98. The prune bytes add up to 30565695093, which matches.
- **Test:** lines 100-107 (test.log 2632-2860).
- **Negative:** lines 110-116 (negative.log 3422-3425, 3610-3611).
- **Run:** lines 118-131 (run.log as cited under question 2).
- **Records:** lines 136-139.
- **Finding 3:** lines 154-163, including the truncated hashes and byte counts (test.log 51-53) and changes YS, YW and XY (test.log 183-206, 574-577).
- **Scan pre-check:** line 84 (scan-pre.log tail).

**Misstatements and statements without a primary**
- **Line 67 (ruling E) misstates its primary.** "The chart relationship and the chart part take the sheet's form" is wrong for the chart part.
  - The chart part takes the drawing's form (spec-working.md 636-641; phase.diff 436, 466).
  - On a strict sheet with a created drawing, the chart part is transitional (phase.diff 2622-2642).
  - "One test reads back a strict package" is also off: two tests do, at phase.diff 2622 and 2645.
- **Lines 99-108: the plants are placed under "Test (both targets)".** The plants ran on host only (see question 2, note a).
- **Line 164 (finding 4) has no primary and the report does not say so.** "My unlogged dev run read 26, those at the corpus top" appears in no log. The 27 is supported (run.log 3979).
- **Findings 1 and 2 (lines 143-153) have no primary.** This covers the superseded hashes e68c346..., 3195cfb..., bb23abc... and d572cb3..., the "20 insertions and 3 deletions", and the dry-run failure. The report says so at line 153, so these are acceptable as declared.
  - Partly supported: the current test titles its charts "First" and "Second" (phase.diff 2687-2694), and P10 now fails that test beside two image tests (test.log about 3237-3254).
- **Placeholders:** RECORD_COMMIT, SCAN_SHA, SCAN_RESULT, CHECK_SHA and INDEPENDENT_CHECK are unfilled, as intended.

**Disagreements, stated plainly**
- The report's line 67 should say the relationship takes the sheet's form and the chart part takes the drawing's form, read back by two tests.
- The doc comment at phase.diff 1034 claims a catalogue entry this phase does not write.
