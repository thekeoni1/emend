# M2 part 4d P1, independent check 1

The answer of a fresh subagent given only <phase>/check1/inputs (the committed spec, the working spec with planning's P1 rulings, the phase diff, the commit messages, the four logs, the four records and the draft report; their sha256s in <phase>/check1/inputs.sha256), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m2p4d-p1. The answer is ASCII as written.

---

**Independent check, M2 part 4d P1** (checker directory `<phase>/check1`, inputs only)

I read only the files in `<phase>/check1/inputs` and modified nothing. I hashed every input with sha256sum. Results:
- `spec-committed-0022.md` is a207e17567206dd86ba367472e19f8f9972b300ab10ec9f35e6b49e2143a1b33.
- `spec-working-0022-p1-rulings.md` is 0cfa5163655bd288471a873a158cef30e94d281a2ce70f34886965deded325ef.
- `phase.diff` is 21d8855461403dd1532a74f74df3782b95f3628d6dbf87686e71b03df1237c31, the hash gate.log gives for "the phase's combined diff" (`<phase>/check1/inputs/gate.log` lines 1806-1818).
- The specs/0022 file that phase.diff adds (`<phase>/check1/inputs/phase.diff` lines 3000-3879), extracted, hashes to a207e175...1b33, the committed spec.

---

**Question 1. Does the diff do what the spec (with the P1 rulings) says for P1, and nothing else?**

Verdict: YES. There are minor observations, listed after the items.

Each required P1 item against `<phase>/check1/inputs/phase.diff`:

- **The feature.** `comments = []` is off by default with no dependency (line 65). The tool turns it on (lines 3880-3892). gate.log lines 2060-2075 confirm the crate manifest's only non-comment change is the feature, the tool's only change is the added feature, and no Cargo.lock changed.
- **The types.**
  - `Comment<'a>` derives Debug, Clone, Copy, PartialEq and Eq (lines 260-270).
  - `CommentEntry` derives Debug, Clone, PartialEq and Eq (lines 271-284).
  - Both are re-exported at the crate root behind the feature (lines 96-97).
- **The calls.** `add_comment` (lines 395-403) and `comments` (lines 420-423) are public and gated. No `remove_comment` exists, as P1 requires.
  - The call-time checks are at lines 478-508: off-grid first, then a character XML forbids, then author 1..=255 characters, then text 1..=32767.
  - The read is at lines 1270-1366. It refuses a missing ref or authorId, an unparsable ref, a missing part, and none or two `commentList` elements. An authorId past the authors list reads as an empty author (lines 1369-1377).
- **The parts' bytes.**
  - The created comments part matches "Comments: the parts": the declaration on its own line, no `shapeId`, `xml:space` on leading or trailing whitespace (lines 1073-1112).
  - The created VML part matches the spec's shape: no declaration, openpyxl's numbers, the anchor rule, and R1 = R - 1 when R > 0 (lines 1643-1771).
  - An appended comments part is streamed: a kept equal author is reused, a replaced cell's comment is skipped, and additions take the container's prefix (lines 1122-1265).
- **The allocation.** `free_entry` and `taken_entries` give the smallest N, ASCII case folded, with a removed entry still counted as taken (lines 708-742 and 849-859). The idmap D is the smallest unused (lines 694-715 and 1631-1638). The test `the_names_and_the_idmap_skip_what_is_taken` (lines 2620-2648) covers both.
- **Ids per ruling 1.** S is 1024 * D + K, with K = max(kept count, largest n - 1024*D over kept shapes' `id` or `o:spid` with n in (1024D, 1024D+1024]) + 1 (lines 1781-1823, the key line 1817; ids and `o:spid` read at lines 1490-1499).
  - Only the appended part is read for S. When the part has no idmap, other VML parts are read only to allocate D (lines 651-660), which the spec allows ("else allocated as above").
  - Tests cover the LibreOffice form at 2051 (line 2305), a kept number winning at 1031 (line 2347), and a control's `o:spid` counted (line 2391).
- **Overrides and relationships.** These are added for created parts only. They use the worksheet's own form (`COMMENTS[strict]` or `VML_DRAWING[strict]`) and relative targets (lines 627-635 and 664-683).
- **`legacyDrawing` creation and retag.** `LegacyPass` creates the element before the first later schema child or at the root's end, and retags through the shared `rels_prefixed` (lines 1825-1888, 787-829 and 876-913). Two elements, or one out of schema order, are refused (lines 961-1023, check at line 988). The worksheet is rewritten only when the element is created (lines 3421-3428 of the source hunk, at diff lines 321-329).
- **The threaded refusal.** Lines 587-595.
- **Messages.** Five constants (lines 769-778) and five catalogue entries (lines 136-150). "no comment on that cell" is absent, as P1 requires.
- **The tool sample without the removal.** Lines 4021-4080 print the line with all eleven fields in the spec's order. `parts_more` is 2, 1 or 0 by `legacy_before` and `comments_part_before` (lines 4052-4056). A failed read-back removes the output (lines 4058-4061). There is no removal step.
- **The listed tests.** All present in `crates/emend/src/package/tests/comments.rs` (lines 2133-2856), and the tool's two tests are at lines 4355-4450:

  | Spec item | Diff line |
  | --- | --- |
  | read with two authors and runs, a plain t, and none | 2135 |
  | read refusals | 2166 |
  | created bytes with D 1 and S 1025, and xml:space | 2208 |
  | both parts | 2252 |
  | LibreOffice form | 2305 |
  | kept number wins | 2347 |
  | form controls | 2391 |
  | no-legacyDrawing refusal | 2684, first case |
  | own prefixes | 2449 |
  | template (prints its output) | 2485 |
  | before tableParts and extLst | 2546 |
  | replacement | 2580 |
  | last wins | 2599 |
  | threaded | 2650 |
  | two legacyDrawing elements | 2667 |
  | call-time refusals | 2761 |
  | removed, replaced or added part | 2811 |
  | shift | 2840 |

- **The carried items of "Phases".** The `remove_table` test asserting "no table of that name", per ruling 5 (lines 2858-2866), and the two `//!` phrases (lines 109-110 and 1081-1110). gate.log lines 1948-1957 confirm code commit 1 changed only `//!` lines in package.rs plus that test.

Nothing outside P1: the only other code changes are refactors the spec asks for ("the shared pieces ... reused, never duplicated").
- Table entry allocation and the root retag were moved into the shared helpers (lines 6046-6130 hunk, 12137-12171 hunk).
- `cells.rs` `push_text` and `expand` became pub(crate); gate.log lines 1958-1962 show these are the only two changed lines.
- The tool's `table_container` became `container_and_binding` with a `name` parameter (lines 4258-4296).
- The base's tests are unedited (gate.log, "the base's tests, unedited": 0 gone, 0 edits).

Observations (not spec violations as far as the inputs show):
1. **An uncatalogued literal message.** `legacy_drawing_id` raises `Error::Package("the worksheet part's root is not worksheet")` as a literal (line 984). This is its only occurrence in the inputs, so I cannot confirm it is an existing catalogued message rather than a new, uncatalogued one. Planning should check it against package.rs.
2. **"Projected workbook" for D.** `vml_idmaps` (lines 694-715) reads only `.vml` entries of the input archive that the write does not remove. A `.vml` part the caller adds, or the replacement bytes of one the caller replaces, is not consulted. The spec says "the projected workbook"; no test covers this edge.
3. **What counts as a shape.** `read_vml` counts only `v:shape` children of the root (lines 1488-1510). Other VML shape elements would not enter K's "every kept shape counted". This is harmless in the corpus per the run, but it is a narrower reading of ruling 1.
4. **Two refusals the spec does not list.** `comments_appended` refuses a `commentList` that comes before `authors` (line 1180), and `read_comments` refuses a root other than `comments`. Both are documented in the doc comments (lines 385-392 and 430-434).

---

**Question 2. Do the logs contain every line the spec requires for P1?**

Verdict: YES.

- **Spec pin.** Every log pins a207e175...1b33:
  - `<phase>/check1/inputs/gate.log` lines 23-27
  - `<phase>/check1/inputs/test.log` lines 24-25
  - `<phase>/check1/inputs/negative.log` line 64 (it also prints the working file's 0cfa5163...)
  - `<phase>/check1/inputs/run.log` lines 4 and 1579-1582
- **gate.log.**
  - The full gate passes with nine PASS lines (lines 2493-2508).
  - Clippy and doc are clean with the feature off, on and all-features (lines 2516-2533).
  - The crate's tests with the feature off and on, host and i686: 512, 530, 512, 530 (lines 2544-2568).
  - The pub fn and pub const first lines are listed with the feature off (144 lines, the base's line for line) and on (149, adding exactly COMMENTS, THREADED_COMMENTS, VML_DRAWING, add_comment and comments) (lines 2333-2350).
  - The consumer crate check names the seven items; it fails with the feature off and passes with it on (lines 2570-2625).
  - The last line is "gate log checks all as expected: yes" (line 2610).
- **test.log.**
  - The workspace runs 854 on host and on i686 (lines 2428 and 3330), with the comments module on (lines 3355-3376).
  - 22 test names were added and 0 removed against 832 (lines 3404-3431).
  - The crate's own tests run with comments off and on, both targets (lines 3547-3590).
  - The carry test runs alone (lines 3591-3598).
  - The template test prints its output (lines 3735-3747).
  - Plants U1 to U8 are each found once and fail when planted (lines 3924-4041).
  - The last line is "test log checks all as expected: yes" (line 4060).
  - The figures 530 and 18 differ from the workspace's 19 comments tests because the shift test is gated on `rowcol`. This is consistent.
- **negative.log.**
  - The binary used is the one gate.log hashed (lines 915-924).
  - C1, a comments part with no `legacyDrawing`, is refused with COMMENTS_UNREADABLE, with controls C2 and C0. C3, two `legacyDrawing` elements, is refused with TWO_LEGACY_DRAWINGS, with controls C4 and C2 (lines 1117-1127).
  - Beyond the spec, it also reaches THREADED_COMMENTS_HELD (C5) and a missing relationship (C6) (lines 1129-1133).
  - Messages reached: 3; stated unreachable: 2, with their covering test (lines 1146-1165).
  - The last line is "all as expected: yes" (line 1199).
- **run.log.**
  - **The comparison.** cmtcmp.py covers "Harness runs": the W, rels, content types, comments part and VML part checks. Its VML id follows ruling 1, from the input part alone (lines 1601-1700 and 2225-2275).
  - **Corpus results.** On host and on i686: 27 summary rows, 27 ok, 0 refused, and every flag holds on 27 (lines 2382-2488 and 2489-2600).
  - **Plant controls.** 10 plants, 10 turned their flag (lines 2601-2625).
  - **Cross-target outputs.** Host and i686 outputs are identical on 27 (lines 2626-2631).
  - **Record naming.** The record pair is `records/m2p4d-p1-2026-10-01-f7d6a7d.md` and `.txt` (lines 2632-2788).
  - **The keep.** Regenerated on both targets, 1200 files identical (lines 2789-2905).
  - **Previous record.** Compared against part 4c P3's record, named by its record commit e1f3736 (lines 2906-3001).
  - **Edit open check.** It ran on the host keep: 600 pairs, no output refused on an accepted input (lines 3002-3800).
  - The last line is "all as expected: yes" (line 3824).
- One display note: my own output truncation first made the host command look like `--timeout 30`. The full lines 2383 and 2490 read `--timeout 300`. That is not a finding.
- scan.log is not among the inputs, which is expected since it runs last.

---

**Question 3. Does every number, hash and quoted string in report-draft.md appear in a primary?**

Verdict: PARTLY. Everything checks out except the line about superseded commits.

These all match primaries (lines refer to `<phase>/check1/inputs/report-draft.md`):
- **Spec hash** (line 3): gate.log line 23.
- **Commit shas and roles** (lines 5-9): 5da68da, 77ed347, 39d7ab7, 96dde75 and f7d6a7d match `<phase>/check1/inputs/commit-messages.txt` and gate.log lines 1693-1744.
- **Doc commit hashes** (line 6): a0322eeb... and 7f723b2a... match commit-messages.txt lines 91-93 and gate.log lines 1771-1796.
- **"no table of that name"** (line 7): `<phase>/check1/inputs/phase.diff` line 2865.
- **Log hashes** (lines 13-16): my sha256sum of each input matches. gate.log's hash also appears in negative.log line 918, and test.log's in run.log line 2790.
- **Test counts:** 512 and 530 match gate.log lines 2565-2568; 854, 22 added and 0 removed match test.log lines 2428, 3330 and 3408; U1 to U8 found once match test.log lines 3949-4036.
- **negative.log messages:** the three reached and two unreachable match negative.log lines 1147-1162.
- **run.log figures:**
  - 27/27 with 0 refused: run.log lines 2386-2387 and 2493-2494.
  - "22 created and 5 appended": the log's "22 created=2" and "5 created=0" (lines 2408-2410). Reading created=0 as "both appended to" follows from the tool's logic (diff lines 4052-4056) and the record's comments and vml families (5 changed, 22 added, `<phase>/check1/inputs/m2p4d-p1-2026-10-01-f7d6a7d.md` lines 37-38).
  - Ten plants: run.log line 2623. Byte-identical outputs: lines 2627-2628.
- **Record hashes** (line 17): match my sha256sum and run.log lines 2784, 2786, 3797 and 3800.

Not supported by any primary:
- **Line 10** names four superseded local commits: 284259a, 758e7d1, 5894b2c and 776e08c. None of these shas, nor the stated reasons ("no lib.rs re-export", "lib.rs not formatted"), appears in any input except the report itself.
- No log records a rerun: RERUN_NOTE and SUPERSEDED_* are unset (gate.log line 4, test.log line 4, run.log lines 6-9).
- The claim "never referenced, local only" cannot be checked from the primaries.
- The report also does not say whether these attempts are kept on disk under `<phase>/superseded`, which "Phases" requires of a superseded attempt.

Planning should either have these commits recorded in a primary, or have the report state that they come from the implementing session's own git history and were not logged.

Omissions, not errors: the report's run.log line does not mention the keep (1200 files, identical on both targets) or the edit open check result (600 pairs, 0 outputs refused on an accepted input). Both are in run.log.
