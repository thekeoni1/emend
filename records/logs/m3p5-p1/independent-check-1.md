# M3 part 5 P1, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1/inputs (the committed spec specs/0027, planning's working specs/0027 with "Planning rulings during P1" ruling 1, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, the stderr files and the runner's summary, the phase's records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p5-p1. Checked commit (the phase commit): e727b255d16f4f31130860af8d5ddfefaf34cd64; HEAD when the check ran: e727b255d16f4f31130860af8d5ddfefaf34cd64. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 8b14b7d712f69847da4eb17cbe4aaa2f302e171268df714983fb7e9417a4ebfd. Its reservations are reported to planning; no second check was run.

Inputs and their sha256:

- 8f0784fbdff8cca99211216c25e9c58b031ae6f3d0d6b8dffc8acdba36c3b71b code-commit-1-carry.diff
- 091030f8d26a3325e113dc2e3a3f14d62d438acdb9ec78c0308e5c257b2d0e66 code-commit-2-crate.diff
- fe51b9695e2bb9b58fb2ced5d233183da798f360398ea6f62c4e8aba24db3dbc code-commit-3-tool.diff
- 9d1183b39cc6cc3153eb6c0790871d16e69e95e88efedd8222fc0d2a9165c386 code-commit-4-catalogue.diff
- d351ce9ea886ffa38fe7044d5f4f2c3b3343d06184354a2bbbd1edd0343e5736 commit-messages.txt
- a4038c3c7cf3754fa76817e665a9d1db1496fdee241148fa5749cced520e7fcd doc-commit.diff
- b00165fd8dc81b8beeb4cdeab21695654c2d9fcac8eccb5283dc9ff7d6a63a38 gate.log
- 7600ca9c2a507356ba97ca812b4fd9be140c6a47746efa0c6fad5fa941819bd8 logged.out
- 553b2f139d0fedc1ac158f13dbd38e767de0fe06adaf33c504a4f77135b0c3e5 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- 3ebf91d6600c2565cfeee66950639b54e2ed9b71b18b2de5724b90f37dc25239 phase.diff
- 71cc6b5d1cd6c23e06c9910efb16ed3406ae011521287d61799a33934fe2f5ad records/m3p5-open-2026-10-05-e727b25.md
- e7501ecacaa4ae609faf7cf1605d4715c8edfd976f3582de0d081a75b0bf866a records/m3p5-open-2026-10-05-e727b25.txt
- 84644eab18dd4772e8b7856c1fcddb0c1268f78f18f04947d4d0b6330a095a4f records/m3p5-p1-2026-10-05-e727b25.md
- 5eccdc8dda8b36d3be62a5b6b05cbd948f971d60244c7cfa6aab66172b02d73b records/m3p5-p1-2026-10-05-e727b25.txt
- 71e3cfef3a5d189c1da9c5df7be69a39bb592545983bdf14a61c782ddf197f01 report-draft.md
- 9fc6c384daf5f50a548d79e37ea1417a78a1cd3147c285311fb86e4d33faa585 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- 434be7771529fc18c480d04cca7128752f73558430fdd834ac50cd0f831e24ce scan-pre-check.log
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- a655b5fdb04a184d8d7137ebec64e89fe4ff0124ef8750ae17cf313d89082f69 spec-0027-committed.md
- 5209aecb3d237b642019a6e962cdfece9fd05fbffc7102bf7c2fb5aab52f09b3 spec-0027-working.md
- 6791a8a2f361d9e711ab24614837798c9cc8d85f5a189a3d265f2c1424110886 test.log

---

# Independent check 1: M3 part 5 P1

The harness put a CLAUDE.md (the project's instructions, plus a user memory index) into my context. I have not used it as evidence. Everything below comes from the 23 input files.

**Input integrity.** I ran `sha256sum -c ../inputs.sha256` in the inputs directory. All 23 files reported OK and the exit status was 0. I read only the input files. I ran no git, cargo, gh or network command; I used text tools and Python over the inputs.

---

## 1. Does the diff do what the spec says and nothing else?

**Verdict: YES with reservations.**

**Which diffs I read.** I read every per-commit diff in full: doc-commit.diff, code-commit-1-carry.diff, code-commit-2-crate.diff, code-commit-3-tool.diff and code-commit-4-catalogue.diff. I checked phase.diff mechanically against them: I counted every added and removed line per file across the five commit diffs and compared the counts with phase.diff. The difference was 0 lines in each direction, so phase.diff holds the commits and nothing else. Every "nothing else" below was read in the per-commit diff named.

**Doc commit (doc-commit.diff).**
- It carries ROADMAP.md, specs/0026 and specs/0027 only (gate.log 2335-2374).
- I rebuilt specs/0027 from the diff's added lines and it hashes to a655b5fd..., the same as spec-0027-committed.md.
- I could not hash ROADMAP and 0026 myself without their bases. For those I rely on gate.log 2350-2357 (aecc629a..., 53d2a173... against planning's message, with controls).
- spec-0027-working.md differs from the committed spec only by the added section "Planning rulings during P1" (working lines 500-518).

**Carry commit fbb171e (code-commit-1-carry.diff; only that diff read for this item).**
- It touches only package.rs (lines 26-41) and the rowcol probe (lines 49-68).
- Every added and removed line is a //! line. gate.log 2601-2635 confirms this: 12 and 16 lines, 0 of them not //!.
- The catalogue line now says "a column group" (diff line 36).
- The probe doc now reads "written to the fourth output with `insert_rows` ... then that opened and written to the fifth output with `delete_columns` ..." (lines 61-64). That is ruling 1's wording (working 505-518), which overrides the committed spec's "(`//`)" at committed 410.
- No manifest change, as ruling 1 defers that to P3. Nothing else is in the commit.

**Feature `read` (code-commit-2-crate.diff 38-43; code-commit-3-tool.diff 30-31).**
- The crate manifest adds `read = []` and nothing else besides comments. There is no dependency and no default-feature line (gate.log 2639-2650).
- The tool adds "read" to its emend features. No Cargo.lock changes (gate.log 2627-2632).

**One font reader (crate diff 48-323).**
- `read_properties` and `read_property` move from `cfg(feature = "richtext")` to `cfg(any(feature = "richtext", feature = "read"))`. They take the typeface child name (`b"rFont"` for runs at line 90, `b"name"` in read.rs at line 1502) and a `refusal` function.
- `color_from` (lines 266-310) is the single colour reading. The run path keeps its own message.
- Nothing else in cells.rs changes beyond visibility: `same_font`, `same_color` and `skip_subtree` become pub(crate).

**Decision 3 handling (crate diff, read.rs).**
- Font: charset, outline, shadow, condense and extend are read past by the shared reader (test lines 529-549).
- Alignment: justifyLastLine, readingOrder and relativeIndent are read past (1655-1689; test 679-696).
- A gradientFill reads as None and its subtree is skipped (1526-1530; test 615-622).

**`cell_style` and `CellStyle`.**
- `pub fn cell_style(&mut self, index: u32) -> Result<crate::styles::CellStyle, Error>` is under `#[cfg(feature = "read")]` (crate diff 351-354).
- `CellStyle` has exactly the six fields the spec names, derives Clone, Debug and PartialEq, and is re-exported under read (1110-1111, 1359-1375).
- `fontId` absent reads as 0, and `numFmtId` absent reads as BuiltIn(0) (1465-1479; test 760-787).
- `Cell` is unchanged.
- gate.log 2896-2905: the public list gains exactly this one call, under the read cfg line.

**One new message.**
- `STYLE_UNREADABLE` = "a style cannot be read" (crate diff 1350). negative.log reports one new constant and none removed.
- It is catalogued under read in e727b25 (catalogue diff 26-37).
- The catalogue test now reads styles/read.rs (catalogue diff 49-98).

**Tests in P1's list (crate diff 377-1078): all present.**

| Spec item | Where |
| --- | --- |
| Each font field, each underline / vertical alignment / colour form / scheme, b val="0", charset read past | 440-549 |
| Each pattern and both colours | 553-612 |
| Gradient as None | 615-625 |
| Each side and both diagonals | 629-675 |
| Each alignment field with justifyLastLine read past | 679-734 |
| Both protection fields | 737-757 |
| Built-in and coded number formats; an xf with no ids | 760-787 |
| Round trips through add_style | 1005-1078 |
| The refusals | 812-937 |
| 473 fonts | 939-964 |
| The tool's line | code-commit-3-tool.diff 288-331 (plus a refusal test, 333-348) |

There are 18 readback tests (test.log 4709-4727).

**`--roundtrip-sample` (code-commit-3-tool.diff 137-257).**
- W is the first worksheet in `<sheets>` order.
- It walks the distinct `s` values in increasing order (BTreeSet) and adds each with `add_style(None, ...)`, passing fill None as no fill.
- It reopens the output and checks every assertion "Harness runs" names, printing the fields in the spec's order.
- It takes nothing from the command line beyond the two paths.

**Reservations (changes beyond the spec's wording, or spec items not met).**
- **R1. Empty `<fill>` reads as None.** A `<fill/>` or `<fill></fill>` with no patternFill reads as None (read.rs 1514-1517, 1534; test 623-624). The spec names only the gradient for None (committed 180-184, 515-521). This is disclosed in c019afa's message and in the report.
- **R2. A border side with `style="none"` reads as None** (read.rs 1614-1616; test 649-656). The spec says "Some(Side) when the element carries style" (committed 185-187). This is the reasonable reading, since BorderStyle has no none variant, but it is undisclosed in the report and the commit message. The run's comparison applies the same rule (run.log 2702: "a side with no style or style none None, its colour dropped"), so the corpus check is not independent on this point.
- **R3. Unknown attributes are silently ignored.** Any attribute not named on alignment, protection, patternFill, border, side or colour elements is ignored, not refused (`Child::value`, read.rs 1750-1755). Child elements inside `<alignment>` or `<protection>` are not refused either (they are read as empty-element tags, 1663 and 1697). The spec names only the three alignment attributes as read past (committed 188-190), while the refusal list (278-280) speaks of "a child ... unknown to the type". Nothing is tested either way.
- **R4. The round-trip test covers only some variants.** It runs 120 combinations (crate diff 1010-1057) that cover each field kind. It does not cover every enum variant:
  - underline: singleAccounting is missing;
  - vertical alignment: subscript is missing;
  - patterns: only none, gray125, solid and darkTrellis;
  - border styles: only thin, mediumDashDot, double and hair;
  - horizontal and vertical alignment: only right and top.

  If "a round trip of each" (committed 425-427) means each value tested above, this falls short.
- **R5. The gradient line is not yet in the catalogue.** "Cell formats" says the catalogue states that a gradient reads as None (committed 184). e727b25 catalogues the message only. "Phases" P3 (committed 465-468) puts the gradient line in P3, so this is consistent with the phases, but it is not done in P1.
- **R6. The roundtrip sample is not on the Excel check list.** "Harness runs" says "the roundtrip sample joins its list" (committed 389-390). No phase file touches the Excel check list (gate.log 2669-2671: no Python script in the diff). P1's own item list does not name this, so it may belong to the close-out. Planning should rule. The report does not mention it.
- **R7. Behaviour equivalence for runs is not shown by the diff.** The run reader's colour path changed from `number()` and `flag()` to `color_from` (crate diff 199-236, 278-292). I cannot see `number()`'s body in the inputs, so I cannot confirm from the diff alone that run tint parsing is unchanged. The richtext tests pass (gate.log 3317-3339; test.log 4551).
- **R8. Disclosed behaviour change.** With read on, `Package::styles` refuses parts it accepted before (styles.rs hunk, crate diff 1179-1215), and one core test is gated `not(read)` (1810-1817). The report discloses both, and the spec's "the same one read" (committed 167) implies it.

---

## 2. Do the logs contain every line the spec requires?

**Verdict: YES with reservations.**

**Hash pins.** Each log pins a655b5fd:

| Log | Lines |
| --- | --- |
| gate.log | 21-27, 2335-2367 |
| test.log | 22-28 |
| negative.log | 73 |
| run.log | 4, 2313-2317 |
| scan-pre-check.log | 7, 55 |

The scan counts the pin outside each log's printed script: gate 7, test 2, negative 1, run 2 (scan-pre-check.log, "every sibling log pins").

**Header lines.** Each log's first lines name the phase commit and HEAD, compare them with a control, and print the script that wrote it:

| Log | Script printed at |
| --- | --- |
| gate.log | 82-2066 |
| test.log | 78-2158 |
| negative.log | 78-1149 |
| run.log | 80-2117 |
| scan-pre-check.log | from about line 106 |

The run log also prints its Python helpers, rtsurvey.py and rtcmp.py, at 2332 and 2674.

**Gate (gate.log).**
- The cold-cache full gate gives nine PASS lines (3213-3230).
- Crate tests:

  | Features | Host | i686 |
  | --- | --- | --- |
  | off | 513 | 513 |
  | richtext | 533 | 533 |
  | read | 530 | 530 |
  | richtext + read | 550 | 550 |

  (3308-3361)
- Clippy and doc ran with each feature set (3239-3306).
- Public fn and const lines are listed with their cfg attribute, base against phase: 174 to 175, with exactly `cell_style` added under read (2893-2958).
- Consumer crate:
  - features off: exit 101, naming cell_style and CellStyle once each;
  - richtext alone: the same;
  - read alone, and read with richtext: exit 0;
  - with controls (3363-3410).
- Code areas, //! only for the carry and the catalogue commit, manifests (2498-2659).
- Gate-file listing with no self-test due (2429-2496).
- Commit messages are ASCII and end with Co-Authored-By only (2297-2333).
- Stderr is counted as 0 bytes with a control (3421-3424).

**test.log.**
- Workspace 1039 on host and on i686 (5414-5416). That equals 1020 + 20 - 1, derived from the previous log (4613-4640).
- Crate runs with features off, read, richtext and both, on both targets. The readback module counts 0/18/0/18, and the core count test runs only without read (4795-4894).
- Six planted changes, each failing at its own assertion with the unplanted control passing (5287-5412).
- Stderr is counted (5425-5428).

**negative.log.**
- The gate's binary is copied and matched to gate.log's hash. The run happens in a worktree (1339-1352).
- NF (a font past `<fonts>`): exit 2, no stdout, "a style cannot be read", no output file. Control NFC: exit 0, its line printed.
- The same pattern holds for NI (fill) and NC (colour with no form) with their controls (the case blocks and "the checks").
- Two usage shapes.
- The message set is derived from source with one new constant.
- Captured outputs hold no control bytes. Stderr is counted (1566-1578).

**run.log.**
- `--roundtrip-sample` on host (3571-3686) and i686 (3688-3809): 27 of 27 ok, lo and lo_in ok 27, totals identical 843, changed 27, flagged 17, recompressed, dropped and added 0.
- The comparison (rtcmp.py, using zipfile, byte spans and ElementTree) gives every flag yes in all 27 on both labels.
- Planted-flag controls: 12 flags, each turned on a created case and a kept case (3811-3865).
- Host and i686 outputs are byte-identical (3867-3871).
- The survey (2332-2665) is checked against the run.
- TMPDIR and the corpus share device 2096, against the /proc control (3562-3569, 4348-4352).
- Calc chain: 9 of 27, each identical and unflagged on both labels, with the rule stated (2667-2672, 3801-3802).
- Record pair checked and installed (3873-4033).
- Keep regenerated on both targets, 1922 files, byte-identical (4035-4150).
- Compared against M3 part 4 P3's record, named by commit 23f9589: 961 pairs = 839 + 122, 0 gone, 0 changed (4152-4346).
- Edit open check: findings 0 (5317-5320), and no output refused on an accepted input (5452-5603).
- Installed record hashes match the inputs under records/. I recomputed all four.
- Stderr is counted (5632-5635).
- Corpus content: none printed. Workbooks appear as 12-hex ids with counts and flags only.

**spec-0001 per-phase rules.**
- My own greps over all five logs found 0 instances of each of the three forbidden shapes, 0 non-ASCII bytes and 0 control bytes. The scan's pre-check agrees: 0 hits, 0 structural failures, RESULT: PASS.
- Placeholders: every one each log prints is declared by a root line (scan-pre-check.log, placeholder section).
- negative.stderr and run.stderr are 0 bytes.
- No absolute machine path appears outside fixtures built from pieces inside printed scripts.

**Reservations.**
- **(a) The public-fn listing is source-level, not compiled.** The "pub fn lines listed off and on" item is met by one source listing that carries each item's cfg attribute (gate.log 2893-2958) plus the consumer crate compiled with features off and on. It is not two compiled listings. I take this as part 4's form, which the log's change notes cite (E28).
- **(b) A departure line the report does not relay.** gate.log line 80 carries "DEPARTURE: CARGO_BUILD_JOBS=4". It looks like a standing departure carried from earlier phases.
- **(c) The final scan has not run.** The scan's final stage is not among the inputs, so the scan log's coverage of the check answers cannot be confirmed here.

---

## 3. Does every number, hash and quoted string in report-draft.md appear in a primary?

**Verdict: YES with reservations.**

**Found in primaries:**
- **Commits:** d64b4e8 with its subject (scan-pre-check.log 38-39); 6d49977, fbb171e, c019afa, 8d4d468, e727b25 (gate.log 2267-2276).
- **Doc hashes:** a655b5fd, aecc629a, 53d2a173 (gate.log 2335-2367; commit-messages.txt).
- **Log hashes:** b00165fd, 6791a8a2, 553b2f13, 9fc6c384 (scan-pre-check.log, "scanned" lines; they match my sha256 of the inputs).
- **Crate test counts:** 513, 533, 530, 550 on host and i686 (gate.log 3354).
- **Workspace count:** 1039 on both targets (test.log 5415).
- **Readback tests:** 18 (test.log 4709).
- **Tool tests:** two (test.log 4728-4731).
- **"sixth module":** catalogue diff 49.
- **Negative cases:** NF, NI, NC with exit 2, no output and controls (negative.log, "the checks").
- **Corpus run:** 27 ok on each label; 843, 27, 17, 0, 0, 0 (run.log 3612-3614, 3730-3731).
- **Survey:** 362 used styles, 0 gradient, 0 empty fill, 0 new codes (run.log 2662). I also summed the survey columns myself: 362, 0, 0, 0.
- **Calc chain:** 9 of 27 (run.log 2668).
- **Open check:** 961 pairs, 122 new, findings 0 (run.log 4157-4158, 5317).
- **Scan:** "RESULT: PASS" (scan-pre-check.log, last line).
- **Record names:** run.log 3876-3884, 5605-5613.
- **The message:** "a style cannot be read" (crate diff 1350).
- **Message references:** "specs/0026 close-out ruling 3" and "0027 P1 ruling 1" (commit-messages.txt; working spec 505).
- **Disclosure 2's fillId 0:** run.log 2711 ("a None fill none, fillId 0").
- **Disclosures 3 and 4:** supported by the crate diff (1179-1215, 1810-1817) and the catalogue diff.

**Not printed in any primary among the inputs:**
- **434be777...**, the scan log's hash. It equals my sha256 of scan-pre-check.log and the inputs.sha256 entry, but no log prints it. A log cannot hash itself, so this is expected.
- **"One HOLD".** The word does not appear in any primary. The working spec's ruling 1 preamble supports the substance (working 502-503: "Asked by the implementing session after the doc commit 6d49977, before any code"). The report flags the disclosures as having no primary unless named.

**Disagreements:** none found.

**Omissions planning may want in the report:**
- R2: `style="none"` on a side reads as None, applied the same way by rtcmp.
- R3: unknown attributes are read past.
- R4: the round-trip test's variant coverage.
- R6: the Excel-check list item is not done.
- The gate.log line 80 DEPARTURE.
