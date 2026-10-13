# Independent check 1: M5 part 3 P1 (specs/0034 charts read)

Checker: a fresh subagent with only the files the brief named. I read the spec (both copies), phase.diff (sha256 e4ad2c83..., equal to `git diff 017e480..54bf510`), the draft report, the four logs, the pre-check scan log, the four records and the superseded SHA256SUMS. I ran read-only git commands (`log`, `show`, `diff --stat`, `rev-parse`), `sha256sum` and `diff`. I ran no build. No workbook cell content appears below.

## 1. Does the diff do what the spec says for P1 and nothing else?

**Verdict: yes. There is one disclosed extra widening, a few refusals beyond the spec's text, and one ruling C test that is not in P1 and that the report does not mention.**

What matches (phase.diff line numbers):
- **Feature:** `charts = []` with no dependency, so it is off by default (diff 59-71). `mod chart` and the `pub use` are gated `#[cfg(feature = "charts")]` (diff 76-95). The tool enables it (diff 3029-3041).
- **Call:** `Package::charts(&mut self, part: &[u8]) -> Result<Vec<ChartEntry>, Error>` (chart.rs, diff ~555). It does the following:
  - It resolves the drawing through `drawing_of`.
  - It streams the anchors, taking the root anchors and the first anchor of a root `AlternateContent`'s first `Choice`, never the `Fallback`.
  - In each anchor it reads `from`, `to` and `editAs`, and the `graphicFrame/graphic/graphicData/c:chart` `r:id`.
  - It follows the drawing rels, filtering `!rel.external && CHART.contains(kind)`. A missing r:id, an unknown id, the wrong type, an external target or a missing part each give `CHART_UNREADABLE` (diff ~575-597, 919-951).
- **Types:** `ChartEntry`, `ChartAnchor`, `Chart` with only `title`, `auto_title_deleted` and `plots`, as P1 requires (no P2 field present). Also `Plot` (with `axis_ids: Vec<i64>`), `ChartKind` (16 variants), `BarDirection`, `Grouping`, `ScatterStyle`, `Series`, `SeriesName` and `Data`, all matching the spec's listing (diff 258-498).
- **Message:** `CHART_UNREADABLE = Error::Package("a chart cannot be read")` is the only new constant (diff 252; negative.log 156 confirms one constant).
- **Ruling A:** `Anchor`, `EditAs`, `drawing_of`, `element_rel_id`, `DRAWING_ELEMENT`, `TWO_DRAWINGS`, `DRAWING_UNREADABLE`, `SPREADSHEET_DRAWING`, `MARKUP_COMPATIBILITY`, `digits_value` and `DRAWING` each gain `feature = "charts"` (diff 97-216). `read_drawing_roots`, `DrawingRoot` and `read_images` are untouched, so they stay under `images`. `CHART` gains charts (diff 130-139).
- **The extra widening:** `DRAWINGML` goes from `any(images, read)` to `any(images, read, charts)` (diff 121-128). Ruling A's list does not name it. chart.rs needs it (`Node::of`, diff ~640), and it adds nothing to an `images` build. It is disclosed in the report (finding 3) and in the crate commit message.
- **Ruling B:** a chartsheet meets `element_rel_id`'s "the worksheet part's root is not worksheet". This is tested (tests/chart.rs `a_chartsheet_part_is_refused_as_not_a_worksheet`, diff ~1792). The sample filters on `SheetKind::Worksheet` (main.rs, diff ~3170).
- **Ruling C:** `Plot::axis_ids: Vec<i64>` is parsed by `Node::val::<i64>()`, which is `i64::from_str` (diff ~1178). There are tests for -2028358176, +7 and 4294967295 (accepted) and for "x", "1.0" and 2^63 (refused).
- **Sample:** `--charts-sample` prints 38 counts and 3 digests (diff 3142-3335). They are exactly the spec's "Harness runs" list minus the P2 fields: `axes`, `axis_*`, `gridlines`, `legends`, `pivot_charts`, `styles`, `digest_numfmts` and `digest_sources`. It prints counts and FNV digests only and writes the input back via `write_into`.
- **Tests:** every test that "P1" lists is present in the 27 crate tests, plus the tool line test and the tool refusal test.
- **Trees:** 266c410/c566a01/54bf510 sit on 017e480. `fbb11a8^{tree}` equals `54bf510^{tree}` (50fa03d4...).

Departures and gaps (none blocking, all for planning):
1. **Ruling C's tests are incomplete in P1.** Ruling C asks for "a negative `axId` joined to a negative `crossAx`". That needs P2's `ChartAxis`, so it cannot be in P1. Nothing in the diff or the report says it is carried to P2. It should be named as a carry.
2. **Refusals beyond the spec's "Errors" text.** These are documented in the `charts` doc comment and tested, but the spec does not name them:
   - an `srgbClr` `val` that is not six hex digits (diff ~1279-1284);
   - a boolean element's `val` other than `0`, `1`, `true` or `false` (`Node::flag`; e.g. `varyColors val="maybe"`);
   - an `idx` or `order` that is not a `u32`.

   The anchor refusals (no `col`/`row`, or `editAs` outside its set, given as DRAWING_UNREADABLE) are also not in the spec's text. "the drawing part is read as `images` reads it" arguably covers them.
3. **Integer form.** `idx`, `order` and `ptCount` use `from_str` like `axId`, so a leading `+` is accepted there too. Only the `axId` form is named in the report.
4. **Test gap.** "each data form with and without a cache": `multiLvlStrRef` is tested only with a cache (tests/chart.rs `each_data_form_reads_with_and_without_a_cache`). `numRef` and `strRef` are tested both ways.
5. **Sample additions.** It asserts the six kind counts sum to `plots` and the name counts sum to `series` (exit 2 otherwise). It also maps `Bar3D`, `Line3D` and `Pie3D` into `kind_bar`, `kind_line` and `kind_pie`. The spec does not say how 3D kinds are counted. The corpus has none.
6. **Planning's docs, committed unedited (not the implementer's error):**
   - ROADMAP.md's new Part 3 paragraph still says "39 series" (diff line ~36), against ruling D's 38.
   - Ruling A in the working spec cites "the doc commit 37cf313". That commit was superseded by 266c410 and is not an ancestor of the phase.

Nothing else is in the diff: there are 11 files, 3210 insertions and 27 deletions, and every hunk belongs to the doc commit, the crate commit or the tool commit as described.

## 2. Do the logs contain every line the spec requires?

**Verdict: yes.**

- **Test sets, both targets (test.log, output section, at file line offset 2000+):**
  - Host: `f-host-off` 515, `charts` 542, `charts,images` 578, `charts,read` 645, `all` 1017.
  - i686: the same five figures.
  - Workspace: `ws-host` and `ws-i686` 1225.
  - The E-checks against part 2 P3's figures (515, 618, 990, plus 27) all pass, and chart tests ran 27 of 27 in every charts set. The 11 plants P1-P11 each fail exactly their expected tests.
  - gate.log 6494-6533 also lists per-module counts on i686 and the "host and i686 run the same tests" lines for all five sets.
- **pub fn listing, off and on (gate.log):**
  - 5808-6093: the listing at the phase commit. `charts` is the only item under `#[cfg(feature = "charts")]`, and `DRAWING` and `CHART` are under the widened cfgs.
  - 6573-6654: the consumer `cargo check` with the features off (fails, 16 items), `images` alone (fails, 13), `charts` (passes) and `charts,images` (passes).
- **Negative (negative.log 209-232):**
  - B0 (the control, exit 0 with the tool test's line).
  - N1, "a chart whose c:chart names no relationship": exit 2, no stdout, the error exactly "a chart cannot be read".
  - Lines 186-189 say which refusals are covered only by crate tests.
- **Run log (run.log, output after the embedded script, ~3590+):**
  - Both targets ran over 27 files: 27 ok, 870 entries identical, `files_identical` yes 27.
  - "the charts line's charts is the kinds sample's sheet_charts in every workbook, 27 of 27" is printed for host and for i686, read from `records/m5p2-p3-kinds-2026-10-13-f35de0d.md`'s run `2026-10-13-f35de0d-size-kinds`.
  - Planning's figures: 16 of 16 met, departed fields none, `series` 38.
  - The largest chart part is 28215.
  - The host and i686 outputs are byte-identical, 27 of 27.
  - The keep was regenerated: 1103 pairs, of which 1102 are the part 2 record's pairs unchanged and 1 is new (the charts tool test).
  - The open check exits 0, with no output refused on an accepted input.
  - The record pair and the open pair are installed, 4 files.
- **Prune (gate.log 65-280):** the four part 2 roots m5p2-p1, p2, p3 and close are each listed with `ls -1A`, sized with `du -sb` (4415291340, 4499862550, 8972372078, 1855617624) and removed with `rm -r` (exit 0, no -f). The close-out's kept names, `gen.py` and the logs among them, are confirmed still present.
- **Rerun:**
  - test.log 3-4 and 48-53 hash the superseded test.log 933e17a2, test.sh 4681cab2 and scan-pre-check.log 142923d7.
  - run.log 4-8 and 75-85 hash run.log a54a7903, run.sh 71321444, chartscmp.py a913a3fb, the pre-check log and the four superseded records. It also moves the old run directory and keep to `<scripts>/out/superseded`.
  - `sha256sum -c` of superseded/logged-1/SHA256SUMS: all 14 OK.
- **Scan pre-check (scan-pre2/scan.log 647-651):** total hits 0, structural checks failed 0, RESULT: PASS.

Minor observation: the run log's sample uses release binaries (`<target>/main/release/emend-copy`). P3 later expects identity with "P2's debug run". P1 sets no build mode, so this is not a P1 defect, but planning may want P2's mode stated.

## 3. Does every number and quoted string in the draft report appear in a primary?

**Verdict: partly. Every hash and figure is correct. Four items are not printed in any primary, or are worded inaccurately.**

Verified against primaries:
- Commit hashes and subjects (git log). The base subject.
- The committed spec blob 730693f3... (gate.log 300; `git show 54bf510:specs/0034...`).
- The working spec dd5030c6... (run.log "status after the run"; sha256 of the working file).
- 11 files, 3210 insertions, 27 deletions (`git diff --stat`). chart.rs 1159 lines, tests/chart.rs 27 tests, null_rewrite 2 tests (diff).
- The four log hashes (sha256sum of records/logs/m5p3-p1/*) and their trailer lines.
- The four record hashes (sha256sum).
- The prune figures (gate.log 105, 154, 222, 277).
- Test counts 515, 542, 578, 645, 1017 and 1225 (test.log). 27 chart tests and plants P1-P11 (test.log).
- 27 of 27 on both targets, 16 of 16, the sixteen figures, and 28215 (run.log).
- The single new keep pair (run.log keep section).
- RUN_SUFFIX=-b (run.log 4).
- The superseded hashes 933e17a2, a54a7903, 142923d7, 4681cab2, 71321444 and a913a3fb (test.log 51-53, run.log 77-80, SHA256SUMS).
- chartscmp.py's new hash 819cd2be (run.log ~3595).
- Change letters XY and NUH (RERUN_NOTE).
- The second pre-check's 0 hits and 0 structural failures (scan-pre2 647-651).
- The test names quoted under rulings B and C (diff).
- 0edc2df->ca48df1, cbc5664->fbb11a8 and 37cf313->266c410. Their tree equality and the one-line P11 test fix are confirmed by `git diff cbc5664 fbb11a8`, which changes only tests/chart.rs (+3/-1). 37cf313's message lacks the three sha256 lines.

Not in a primary, or inaccurate:
1. **"In all, 19743143592 bytes were freed" (report line 62).** The sum is arithmetically right, but no log prints it; gate.log prints only the four per-root figures. "Freed" also overstates: `du -sb` measures apparent bytes, not the disk space released. Suggest "the four roots totalled 19743143592 apparent bytes (du -sb), summed from gate.log".
2. **"test.sh 4681cab2 became bdfe1ec6" and "run.sh 71321444 became 00f77502" (report lines 97-100).** The new hashes bdfe1ec6 and 00f77502 are correct for `<phase>/scripts/m5p3-p1-test.sh` and `m5p3-p1-run.sh` on disk (I hashed them). They appear in no log, record or listed primary; neither test.log nor run.log prints its own script's sha256. Only chartscmp.py's new hash is in run.log. Either cite the on-disk files or drop the two new hashes.
3. **"specs/0034 is not committed in P1" (report line 3).** This contradicts line 17 and the doc commit 266c410, which commits specs/0034 (blob 730693f3). The intended meaning, that planning's working copy with rulings A-D is not committed, should be said exactly.
4. **"The drawing helpers and constants in package.rs are gated `any(images, charts)`" (report lines 34-35).** This is imprecise:
   - `DRAWING` is `any(images, read, charts)`.
   - `MARKUP_COMPATIBILITY` is `any(images, copy, charts)`.
   - `element_rel_id` and `digits_value` are `any(comments, images, charts)`.

   Each gained `charts`; not all are `any(images, charts)` (diff 97-216).

Also missing from the report, though no number is wrong:
- Ruling C's "negative `axId` joined to a negative `crossAx`" test is not delivered in P1 and not named as carried to P2.
- The refusals beyond the spec's text (`srgbClr`, boolean `val`) are not named.
- Ruling B's "the sample calls `charts` on worksheets only" is implemented (SheetKind::Worksheet filter) but not stated.
- The report's finding 2 says a check found plant P11 uncaught. That claim's own primary (FACTS.md or the first check) is outside this checker's file list, so I could confirm only the resulting code change.
