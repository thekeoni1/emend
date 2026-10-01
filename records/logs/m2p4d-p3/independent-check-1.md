**Independent check, M2 part 4d P3** (checker directory `<phase>/check1`, inputs only)

**Input integrity.** I ran `sha256sum -c` against `<phase>/check1/inputs.sha256`. All 14 inputs match: commit-messages.txt, gate.log, m2p4d-open-2026-10-01-b5244d6.md, m2p4d-open-2026-10-01-b5244d6.txt, m2p4d-p3-2026-10-01-b5244d6.md, m2p4d-p3-2026-10-01-b5244d6.txt, negative.log, phase.diff, report-draft.md, run.log, scan-pre-check.log, spec-committed-0022.md, spec-working-0022-p3-rulings.md, test.log.

The two spec copies differ only in the P3 rulings: "Images: the calls" (working lines 394-397), "Harness runs" (working lines 584-589), and the new section "Planning rulings during P3" (working lines 939-966).

All paths below are `<phase>/check1/inputs/<file>`. "diff N" means `<phase>/check1/inputs/phase.diff` line N.

---

### Question 1: does the diff do what the spec says for P3, and nothing else?

**Verdict: YES**, with the reservations listed at the end of this question. I judge none of them blocking.

**The carry (P2 verification ruling 1, three items)**
- (1) The record-order doc comment now gives the code's order: diff 384-392.
- (2) The stale "rewrites the worksheet only when ... created" comment now also names the drop: diff 272-276.
- (3) A test pins that a `v:shape` with no `ClientData` keeps the VML part and `legacyDrawing`: diff 1582-1614.
- gate.log 1954-1973 confirms that code commit 1 changes 12 lines, all comment lines, and adds the test.

**The doc commit**
- It carries the P2 rulings and verification into the spec (diff 2428-2514) and the ROADMAP P2 row (diff 9-10). P2 ruling 5 asks for both.
- gate.log 1802-1830 matches both blobs to the hashes in the commit message.

**Feature `images` and the types**
- `images = []`, off by default, with no dependency: diff 22-25. The tool turns it on: diff 2527-2531.
- `Image<'a>` derives Clone, Copy, Debug, PartialEq, Eq: diff 165-172.
- `Anchor { OneCell, TwoCell, Absolute }` derives the same: diff 175-184.
- `ImageEntry { from, anchor, part, bytes: u64 }` derives Clone, Debug, PartialEq, Eq: diff 188-201.
- lib.rs re-exports the three types: diff 34-35.

**The PNG and JPEG header reads (diff 997-1053)**
- PNG: the signature, `IHDR` at bytes 12..16, width and height at offsets 16 and 20. Fewer than 24 bytes is refused.
- JPEG: `FF D8`, then a segment walk that skips fill bytes and steps over standalone markers 01 and D0-D8. `SOF` C0-C2 is taken, with height at offset 5 and width at offset 7. DA, D9 or 00 before an `SOF` is refused.
- Size rule: a value of 0, or an EMU value above `i32::MAX`, is refused, using 9525 EMU per pixel (diff 445-452, 120).
- Order at the call: grid first, then format, then size (diff 440-452).

**`images` and `add_image`**
- `add_image` records the image at the call (diff 439-464).
- `images` (diff 468-494) returns an empty list when there is no `drawing` relationship (470-472). It refuses, with the cannot-be-read message, a missing drawing part (473), an `r:embed` with no relationship or an external one (479-482), and a missing media entry (485).
- `read_images` (diff 1390-1556) handles:
  - a `pic` held directly, inside a `grpSp`, or inside an `AlternateContent`'s first `Choice` (1447-1459);
  - P3 ruling 1, the first anchor in a root-level first `Choice` (1478-1498);
  - an absolute anchor, which gives (0, 0) (1542);
  - a `from` with no `col` or `row`, a missing `blip`, or a `blip` with no `r:embed`, which are refused (1537-1545);
  - a root other than `wsDr`, which is refused (1433-1436).
- The write's refusals (diff 539-577), in this order: removed part, replaced part, no such part, shift; then `element_rel_id` for two elements, out-of-order elements, or no `r:id`; then a relationship that is not a drawing relationship, or one the part calls touch.

**Media entry naming and Override**
- `free_media` (diff 975-988) finds the smallest N free over both `png` and `jpeg`, case-folded, against `taken_entries`, which includes removals and additions (683-689).
- The entry is added through `add_part` with the caller's bytes unchanged (590-593). An `Override` of `image/png` or `image/jpeg` is always written (594).

**The created drawing part, the append, I, and namespaces**
- Created bytes (diff 1309-1325, plus `anchor_bytes` 1182-1307):
  - the declaration, then a newline, then everything else on one line;
  - `xdr` and `a` bound on the root, and `r` bound on the `blip`;
  - `cNvPr id=I name="Image I"`, where I starts at 1 and goes up by 1 per image (632-634).
- Append (diff 597-620): `read_drawing` (1084-1170) keeps the input's bytes up to the root's end tag. It takes the largest `cNvPr` id over the whole part, which includes `Choice` and `Fallback` (P3 ruling 1). Each added `id` is that largest plus 1.
- Namespace prefixes:
  - The root's prefix is used for the spreadsheet-drawing namespace.
  - The root's DrawingML and relationships prefixes are used when the root binds them, including a default DrawingML binding.
  - Otherwise `xmlns:a` is bound on the added `oneCellAnchor` and `xmlns:r` on the `blip` (1198-1216).
- A root of `userShapes` is refused (1116-1118).

**The drawing's rels part**
- When the drawing is created, its rels part is built in the package's rels form with one `IMAGE` relationship per image, each with a relative target (630-671).
- When the drawing is kept, `add_relationship` adds the image relationship and creates the rels part if there is none (606-611). Test diff 2177-2181 covers this.
- The relationship type takes the sheet's form (`strict`, 578).

**The DRAWING and IMAGE constants**
- Both are `pub const` pairs, transitional then strict, behind the `images` cfg (diff 78-93).
- gate.log 2360-2367 shows these two are, with the two calls, exactly what the feature adds to the public list.

**Drawing element creation and retag**
- `ElementPass` (diff 912-972) is the old `LegacyPass`, now taking the element's name as a parameter.
- `drawing` is created ahead of `legacyDrawing`, before the first later child or at the root's end (diff 750-808).
- The root retag happens with the drawing's strict form (772-774).
- `links_needed` also covers a root that is empty (782-783).

**The four messages and the catalogue**
- The constants are at diff 730-740.
- The catalogue entries are at diff 47-69, each marked as belonging to the feature `images` and naming its tests.

**The tool's constant PNG and its producer**
- The 82 bytes are at diff 2655-2662. The six-line program is quoted, with Python 3.14.4 named, at diff 2641-2654.
- I re-ran that program, in memory only, with Python 3.14.4. It produces exactly these 82 bytes.
- The test copy (diff 1649-1656) is byte-identical to the tool's copy (diff 2655-2661).
- The pixels are red, green, blue, white, as P3 ruling 2 says.

**`--image-sample`, its line and its assertions (diff 2664-2818)**
- W is the first worksheet, and the cell is (M + 1, 1) with the image at its own size (2683-2690).
- The output line has `image` followed by the eight counts and `from_after=col,row` (2698-2712).
- It asserts images, anchors and media each +1, and `from` equal to (0, M) (2760-2786). A failure removes the output (2694-2696).
- Anchors are counted as root children of the drawing part (2790-2818).

**Every test the spec lists under P3 (diff 1873-2419)**
- Read cases:
  - twoCellAnchor, oneCellAnchor, grpSp, AlternateContent, a chart's frame, and an absoluteAnchor giving (0, 0): 1874-1911.
  - P3 ruling 1, the root Choice: 1914-1933.
  - A sheet with no drawing: 1936-1945.
- Read refusals: a blip with no embed, an embed with no relationship, a missing entry (plus no col, a non-wsDr root, a missing part): 1948-1998.
- Header reads on the tool's PNG, a baseline JPEG and a progressive JPEG: 2001-2014.
- Refused at the call: IHDR not first, a JPEG with no SOF, a 23-byte PNG, a GIF, size 0, EMU overflow (225458 pixels), and a cell off the grid: 2020-2068.
- The size override: 2071-2087.
- Created bytes with I = 1 and rId1: 2090-2126.
- Two kept anchors, so I = 3 on append: 2129-2156.
- Namespaces bound on the added elements, or taken from the root's prefixes: 2159-2200.
- `userShapes` refused: 2203-2211.
- Template part: retag, both rels parts created, drawing at the root's end: 2214-2247.
- Created before `legacyDrawing`: 2250-2272.
- `image1.png` and `image2.jpeg` taken gives `image3`: 2275-2285.
- Two images in one write: 2288-2323.
- Two drawing elements, or one out of order: 2326-2335.
- A drawing the write cannot read: 2338-2363.
- Removed, replaced or added part: 2366-2407.
- Shift refused: 2410-2419.
- The tool adds two tests: 2902-3000.

**Things the diff does that the spec does not literally ask for.** Each is documented in the doc comments or the commit message.
- **Refactors.** `legacy_drawing_id` becomes `element_rel_id`, `LegacyPass` becomes `ElementPass`, and `comment_part_untouched` becomes `edited_part_untouched` (diff 399-416, 426-427, 846-971). "Crate" asks for shared pieces to be reused, not duplicated, which supports this. gate.log 2435-2453 shows no base test edited or removed, and all base tests pass.
- **Refusals beyond the spec's list:**
  - a `pic` with no `blip` (diff 1538-1540);
  - an empty, self-closed `wsDr` root on append (diff 1116), which leaves an empty drawing that the spec would append to unwritable;
  - a non-numeric `cNvPr` id, or an id past `u32::MAX` (1148, 612-615);
  - an external drawing or embed relationship (505, 481).
  - Tests cover the non-numeric id (2355-2362). I found no test for the empty-root refusal or for a picture with no `blip`.

**Reservations and doubts**
1. **Unchecked arithmetic.** "Images: types" asks for "checked usize arithmetic". `image_header` uses unchecked `at + 1`, `at + 5`, `at + 7`, `at += 1` and `at += 2` (diff 1028-1045); only `+ 2` and the step are checked (1001, 1049). Overflow is not reachable in practice, because `at` stays below the slice length plus 65537. It is still a literal departure from the spec's wording.
2. **Two different ways to find the drawing.** `images` finds the drawing through the sheet's first `drawing` relationship (diff 500-513). `add_image` follows the `<drawing>` element's `r:id` (557-574). A sheet with two `drawing` relationships could be read differently by the two calls. The corpus does not exercise this.
3. **Strict sheets are untested.** For a strict sheet, the created drawing part uses transitional namespaces while the relationship types are strict (624-629, 640, 674). This agrees with the spec's fixed bytes and with "the sheet's form", but nothing tests it, and the corpus has strict=0 throughout (run.log 2507-2559).
4. **"Literal bytes".** The created-bytes test compares against a string assembled from helpers (`added_anchor`, `XDR_NS`), not a single literal. The run log's `drawing_bytes` flag checks the same bytes independently on 17 created parts (run.log 2563-2564).
5. **Ordering untested.** When both `drawing` and `legacyDrawing` are created in one write, the code orders them correctly (diff 761-767, 791-797), but there is no test. The P3 test list does not ask for one.

---

### Question 2: do the logs contain every line the spec requires?

**Verdict: YES.** The final `scan.log` is not among the inputs; only the pre-check stage is. That is expected, since the spec puts the scan last, after the checks.

**General rules ("Phases")**

| Requirement | Where it is met |
| --- | --- |
| Logs in `records/logs/m2p4d-p3/` | scan-pre-check.log 2005, 2015, 2025, 2035 |
| Every log pins the spec hash (1e5248c6, the committed blob, as P3 rulings preamble says) | gate.log 22-26; test.log 24-25; negative.log 64-65; run.log 4 and 1698-1701; scan-pre-check.log 8 and 51 |
| test.log on both targets with the feature off and on | test.log 3705-3747 (host and i686, off 512 and on 530); workspace runs 1612 and 2550 with every feature on (3488-3532) |
| A superseded attempt stays on disk (no rerun) | `RERUN_NOTE` unset: gate.log 4, test.log 4, negative.log 3, run.log 8 |
| No cargo running while a logged script runs | gate.log 61; negative.log 42 |
| Commit messages ASCII, ending with Co-Authored-By only | gate.log 1769-1800 |
| `ANTHROPIC_API_KEY` unset | gate.log 6, 74; negative.log 52-53 |
| Each log ends "all as expected: yes" | gate.log 2623, test.log 4232, negative.log 1231, run.log 4041 |
| Scan, pre-check stage | scan-pre-check.log 2223 "RESULT: PASS", total hits 0 (2219); no check answers yet (2060-2061, 2182-2183) |
| Final `scan.log` | **not in the inputs** (expected to come after the checks) |

**P3 negative log**

| Requirement | Where it is met |
| --- | --- |
| Fixture built from the template | negative.log 1002-1106 (generator), fixtures 1100-1106 |
| A drawing relationship naming a missing part is refused, with a control | I1 at 1152-1156 (`DRAWING_UNREADABLE`, no output), controls I2 at 1151 and I0 at 1154 |
| Two `drawing` elements refused, with a control | I3 at 1158-1162 (`TWO_DRAWINGS`), controls at 1157 and 1160 |
| Extras beyond the spec | I4, legacyDrawing then drawing (1164); I5, userShapes (1167) |
| Reached and unreachable as sets | 1179-1194 |

**P3 run log**

| Requirement | Where it is met |
| --- | --- |
| `--image-sample` on the host | run.log 2466-2571 |
| `--image-sample` on i686 | run.log 2573-2683 |
| Recorded as `records/m2p4d-p3-<date>-<sha>.md` and `.txt` | run.log 2853 and 3003-3008 |
| `--comment-sample` once more, byte-identical to P2's run by sha256 | run.log 2716-2849: host "outputs equal to part 4d P2's by sha256 27; lines equal byte for byte 27" (2782-2784); i686 the same (2847-2849); P2's run named by its record commit 0eadfec (2717-2718) |

**"Harness runs": expected results**

| Requirement | Where it is met |
| --- | --- |
| `lo` and `lo_in` ok, `order_preserved` yes, `files_identical` no | run.log 2501 and 2608 |
| Only `descriptor` allowed on a changed entry | run.log 2462 (control); entry totals 2503 and 2610 with flagged 43; the `.md` record's descriptor column equals its flagged column |
| Entry rows as P3 expects (W, W rels, drawing, drawing rels, media added, content types) | run.log 2561 and 2668 ("entry rows as P3 expects: 27"); created 17 and appended 10 at 2563-2564 and 2670-2671 |

**"Harness runs": the run log's own check, with a control per check**
- `imgcmp.py` uses zipfile, byte spans and ElementTree (run.log 1717-1720).
- Its flags cover W, W's rels, `[Content_Types].xml`, the exact drawing bytes or the appended prefix with one anchor, the drawing's rels, media bytes and the images read (run.log 1720-1830, flags at 2442).
- Results: 27 of 27 both targets (2561, 2668).
- Ten plants each turn their own flag (2685-2708).
- Host and i686 outputs are byte-identical (2711-2713).

**Other "Harness runs" requirements**

| Requirement | Where it is met |
| --- | --- |
| Edit open check over the phase's keep on the host | run.log 3213 onward; open record installed at 4010-4018 |
| Phase directories named by their record commit | run.log 33-36, 2717-2718, 3128-3130 |
| No corpus name, reference, text or image bytes printed | workbooks appear by 12-character id only |

One observation: the spec itself requires the tool to print `from_after` = (0, M). So M, the largest row number of each corpus sheet, appears in run.log (for example at 2525 and 2541). This is by the spec's design and is not a fault in the diff, but planning may want to confirm that it counts as a count and not as a reference.

---

### Question 3: does every number and quoted string in the draft report appear in a primary?

**Verdict: YES.** Every figure and quoted string below was found.

**Header and commits**

| Report line | Item | Found at |
| --- | --- | --- |
| L3 | b5244d6 (tool commit) | commit-messages.txt 1-2; gate.log 35 |
| L3 | base 0eadfec793dc71aa2a7b5f0319eb51fadc98f4d9, P2's record commit | gate.log 31 and 1735-1737 |
| L6 | d5d1124 and its subject | commit-messages.txt 67-68 |
| L6 | 1e5248c6... | commit-messages.txt 70; gate.log 1806 |
| L6 | 3d434cd0... | commit-messages.txt 71; gate.log 1811 |
| L7 | 16913cf and its subject | commit-messages.txt 50-51 |
| L7 | "comment lines alone in package.rs, and one test" | gate.log 1968 and 1973 |
| L8 | b65ea6d and its subject | commit-messages.txt 22-23 |
| L9 | b5244d6 and its subject | commit-messages.txt 1-2 |

**Rulings**

| Report line | Item | Found at |
| --- | --- | --- |
| L12 | P3 ruling 1 | spec-working 947-957 |
| L12 | "first Choice" | commit-messages.txt 29-30; diff 355-357 |
| L12 | test `an_anchor_in_a_root_choice_is_read` | diff 1914; test.log 3580 area and 4195 |
| L13 | 82 bytes, one stored deflate block, Python 3.14.4 | spec-working 958-965; commit-messages.txt 6-7; diff 2644 |

**Logs**

| Report line | Item | Found at |
| --- | --- | --- |
| L15 | "first logged attempt" | `RERUN_NOTE` unset in all four logs (gate 4, test 4, negative 3, run 8) |
| L15 | "all checks as expected" | gate 2623, test 4232, negative 1231, run 4041 |
| L16 | gate.log 7d8c7583... | inputs.sha256; scan-pre-check.log 2005; negative.log 961 |
| L17 | test.log bfa440e8... | inputs.sha256; scan-pre-check.log 2015 |
| L18 | negative.log 63528b65... | inputs.sha256; scan-pre-check.log 2025 |
| L19 | run.log aed59e8c... | inputs.sha256; scan-pre-check.log 2035 |
| L20 | combined diff fd105aa8... (gate.log) | gate.log 1842; matches phase.diff's own sha256 |
| L21 | "RESULT: PASS", run before any check answer | scan-pre-check.log 2223; no answers at 2060-2061 |

**Figures**

| Report line | Item | Found at |
| --- | --- | --- |
| L24 | 890 on host and i686 | test.log 3555 and 4216 |
| L24 | previous phase 868 | test.log 3561 |
| L24 | 22 added, 0 removed | test.log 3565 |
| L25 | "the carry's test, the images module's nineteen and the tool's two (22)" | test.log 3650 |
| L26 | Plants W1 to W8, found once, pass unplanted, fail planted | test.log 4121, 4132, 4145, 4158, 4169, 4182, 4195, 4208 |
| L27 | "adds exactly the two calls and the two pairs" | gate.log 2367 (counts 150/150/154 at 2360) |
| L28 | `DRAWING_UNREADABLE` and `TWO_DRAWINGS` reached | negative.log 1180-1182 |
| L28 | `IMAGE_NOT_PNG_OR_JPEG` and `IMAGE_SIZE_INVALID` unreachable, covered by `the_call_refuses_each_rule_and_records_nothing` | negative.log 1183-1194 |
| L29 | 27 summary rows, 27 ok, 0 refused per target | run.log 2499 and 2606 |
| L29 | read, flags, agree and line all 27 | run.log 2561 and 2668 |
| L29 | 17 created=1, 10 created=0 | run.log 2563-2564 and 2670-2671 |
| L29 | changed 72, added 70 | run.log 2503-2504 and 2610-2611 |
| L30 | comment sample 27 / 27 / 27 per target | run.log 2782 (host) and 2847 (i686) |

**Records**

| Report line | Item | Found at |
| --- | --- | --- |
| L33 | `m2p4d-p3` .md 71b611ea... | run.log 3005; scan-pre-check.log 2062; inputs.sha256 |
| L33 | `m2p4d-p3` .txt cc16c779... | run.log 3007; scan-pre-check.log 2072 |
| L34 | open record .md 16c344d2... | run.log 4013-4015; scan-pre-check.log 2082 |
| L34 | open record .txt ddd6448c... | run.log 4016-4018; scan-pre-check.log 2092 |

**Not checked against a primary.** "Nothing is pushed" (L37) is a statement about state; no input records push activity either way. The git status lines show only untracked records and logs (run.log 4024-4034), which is consistent with nothing pushed but does not prove it.

**Omissions, not errors.** The report does not mention the crate test counts (512 with the feature off, 530 with it on), clippy and doc at 0 warnings, or the extra negative fixtures I4 and I5. It also does not mention the reservations in Question 1.
