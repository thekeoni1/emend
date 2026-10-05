# M3 part 5 P2, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1/inputs (the committed spec specs/0027, planning's working specs/0027 with "Planning rulings during P2" rulings 1 to 3, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, the stderr files and the runner's summary, the phase's records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p5-p2. Checked commit (the phase commit): 6ec00556ce9ce27da163ced8f3b0238de8e50581; HEAD when the check ran: 6ec00556ce9ce27da163ced8f3b0238de8e50581. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 893ca7a8e300207ea7f4c2131cd9279019f3a28bafa962e11d3502ab8ac7856f. Its reservations are reported to planning; no second check was run.

Inputs and their sha256:

- 6458a7ff7340151cf7c9a4596712e7d10b290d703f09f07fe9b264ef72e32032 code-commit-1-carry.diff
- 5154b0ff9b0a5c923a0f9fd68062ca37178d1db0c8e6e0f4c344820c16ff7652 code-commit-2-crate.diff
- 703bd27e1761dc6e5dc1927a0c75d8fb49856abcd517c79b8dea4a78eb8932de code-commit-3-tool.diff
- 74ff387a19b8f454fd0595df515ed52a8292f5da678d86a89a045ea5622090d1 commit-messages.txt
- 97004c6777156b1f121866afab7e6fe7f2dd16f58c47fa9e8a1a7ec173c12d22 doc-commit.diff
- 3c9ab08642b7506d868a431f4a6cf632c5e5f45294fef11920793f6c7f9a3061 gate.log
- ecdda2002e1d23bfe0a33a0bc0717e693763e2cb4d63ad6a0101311195f87005 logged.out
- 9e02657a4bc9419565742dacdf35cbb0489ec4d62884b61fa88dcbe6e49e84ac negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- bae094c4dd49e75c74668e56bdb58602fc350deb07d79e6af7b907b42eef6c96 phase.diff
- 6c14cc54e415ab0b790048b41c560f0c8327e3c87cbdfe2bf9f94c185351b6e3 records/m3p5-open-2026-10-05-6ec0055.md
- c0a2e5916fc4dd31dc43f9d12e4c84eaba891ac2e5fce124989c9bd0920e55f4 records/m3p5-open-2026-10-05-6ec0055.txt
- fb8e51d104cff9e4f680d4e8345912d640cd1cbddd4f189c7f83927980d7cf0d records/m3p5-p2-2026-10-05-6ec0055.md
- 721314e5cb6fd56dea2643dddb260b174c68642876288d532aac96f4f3cc5fdf records/m3p5-p2-2026-10-05-6ec0055.txt
- e1f7b6417e3ffdd5a56ecbf71b71def7cb1065ccd68d384e79e8c733f671e0d1 records/m3p5-scale-d-items-2026-10-05-6ec0055.md
- ef9ed1f3e6c18e328b4cee2141e6a0de75578ac2bf8c3bdb1d3febf3ad6a1604 records/m3p5-scale-d-items-2026-10-05-6ec0055.txt
- fac1f9ca675918a443975cb1dde76b3d6d6d221dc857cf23d55b952c7bdf12a0 report-draft.md
- 73fcbfd5d874b62eec30dbd36716baf4c20d10ebbb85a17ad0b7a2610fd72198 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- 544cfe277edfff2f1e399a86c6ea6527657ceaf979043b6ba4246710fe69baaa scan-pre-check.log
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 5acfcd85d65eb54f4f6c68cf4b1b079695f4c2373e2b07d23a125d547ec8499c spec-0027-committed.md
- 439a1e0dfb26af950bdf34cb9ec11817611aacd4e635c9645b8445c9e91d4fdd spec-0027-working.md
- 4eda68478e7771cb698a77dbe95e684007fb5f4af9d9626c8dbef818a4584706 test.log

---

# Independent check 1: M3 part 5 P2

Preliminaries:
- The harness put CLAUDE.md, the user's MEMORY.md index and git status into my context. I did not use any of them as evidence. Everything below comes from the 24 input files.
- I ran `sha256sum -c ../inputs.sha256` in the inputs directory. All 24 files report OK.
- The committed spec's own hash is 5acfcd85...499c, the hash the doc commit message and the logs pin. spec-0027-working.md hashes to 439a1e0d..., the value the logs print as planning's uncommitted copy.
- Line numbers below are line numbers within the named input file.

## 1. Does the diff do what the spec says and nothing else?

**Verdict: YES with reservations.**

I read every diff in full: doc-commit.diff, code-commit-1-carry.diff, code-commit-2-crate.diff and code-commit-3-tool.diff, plus phase.diff's file list and hunk headers. phase.diff adds 2519 lines and removes 44. The four per-commit diffs add 99+257+1958+205 = 2519 and remove 3+2+38+1 = 44. phase.diff touches the same 10 files the four commits touch, so it holds nothing beyond them. gate.log 2430-2442 hashes the combined diff at bae094c4..., which is phase.diff's own sha256.

**Doc commit (doc-commit.diff).** It changes two things:
- ROADMAP's P1 row (line 23).
- The spec: the R2 correction (lines 34-39) and the two sections "Planning rulings during P1" and "Planning verification and rulings on P1". I extracted the added lines; they are byte-identical to spec-0027-committed.md 501-595.

gate.log 2400-2428 matches both blobs to the hashes in the message and confirms the commit holds exactly those two files. Nothing else: confirmed, from doc-commit.diff.

**The carry (code-commit-1-carry.diff), first code commit, before any other change.** It holds the three items of P1 rulings 1 to 3:
- **Ruling 1, the None fill.** The None fill is compared with fill 0's read value (lines 213-238), the module doc is updated (196-205), and the tool test `the_roundtrip_sample_compares_a_none_fill_with_fill_0_s_value` is added (249-295).
- **Ruling 2, R3.** `an_attribute_the_types_do_not_name_is_read_past` covers font, fill, border, alignment and protection (lines 36-88 of the file).
- **Ruling 3, R4.** `every_enum_variant_round_trips_through_add_style` covers 4 underlines, 3 vertical alignments, 19 patterns, 13 border styles, 8 horizontal and 5 vertical alignments (lines 90-217).

gate.log 2526-2559 confirms the commit's three paths and the three tests by name. Nothing else: confirmed, from the carry diff.

Reservation: the variant lists are hand-written, and the test asserts a hard-coded total (line 202), not exhaustiveness. Whether 19 patterns and 13 border styles are every variant cannot be seen from these inputs.

**Columns and rows (code-commit-2-crate.diff, sheet_read.rs).**
- `ColumnFormat` has the eight spec fields (lines 574-592).
- `columns` is bounded by `<cols>`, returns one entry per `<col>` in part order, and refuses two `<cols>` with TWO_ELEMENTS (lines 794-847).
- `RowFormat` has the eight fields (597-616).
- `Rows`/`rows` stream: one start tag at a time, a borrow, and the iterator is spent after an error (682-778, 860-875).
- A `<row>` without `r` takes the previous row plus one, or 1 for the first, as P2 ruling 1 requires (739-743).
- The test `every_row_field_is_read_and_a_row_without_r_is_numbered_as_cells_numbers_it` also checks `cells()` on the same part.

**sheet_settings in one pass (lines 898-941, 979-1304).**
- Every SheetSettings field is present (644-676): tab_color, outline_summary, fit_to_page, view, freeze, print_options, margins, page_setup, header_footer, auto_filter, merges, hyperlinks, protection, outline_max.
- `view` comes from the first `<sheetView>`.
- `freeze` is set only for `state="frozen"` on the first view's first pane.
- page_setup has its eleven fields, with fit_to_page copied from `<sheetPr>` (916).
- Hyperlinks resolve through the part's rels (917-932). An `r:id` naming no relationship is refused. A range `ref` takes its top-left cell (1254-1257).
- Protection reads all sixteen flags from `options()`, each defaulting to the struct's value (1052-1060, 1306-1328), as P2 ruling 2 requires. The password is never read.

**Types lifted (crate diff 254-470).** SheetView, ViewKind, PrintOptions, PageMargins, PageSetup, Orientation, PageOrder, HeaderFooter (from `sheetprops`) and SheetProtection (from `metadata`) each move to `any(feature, read)` with one definition. Writer-only impl blocks stay under `sheetprops` (486-503).

**Duplicates.**
- TWO_SHEET_ELEMENTS and TWO_SETTINGS are lifted (317-330). TWO_ELEMENTS is used unchanged. I infer it is already ungated, because the gate builds and tests with `read` alone.
- READ_CHILDREN maps each element to one of the three messages (965-977, 1036-1042).
- Schema order is not checked, and a test pins that (two_of_an_element... test, around lines 1930-1975).
- negative.log 1474-1478 reads the three messages from source.

**shared_string_items.** `cells::Item` (text, plus runs with `richtext`) and `Items` hold one `<si>` at a time (crate diff 58-211, 286-308).

**The second message (P1 ruling 4).** It is raised in sheet_read.rs (562) and catalogued in package.rs (233-238), and the catalogue test reads sheet_read.rs (1513-1561), all in the crate commit. gate.log 2586-2590 shows the counts 1 1 1 at code commit 2 and 0 0 0 at code commit 1.

**Tests against the P2 list.** All present in sheetread.rs: every column and row field, a `<col>` range, the no-`r` row (now numbering, per ruling 1), each element, the defaults, a split pane, both target kinds, an `r:id` naming no relationship, two `<mergeCells>`, the table iterator with runs, and the writers' forms. The tool's lines are in code-commit-3-tool.diff 252-309.

Reservation on "a hyperlink of each target with a range ref": only the Location hyperlink has a range `ref` (crate diff 1757). The Url one uses a single cell (1756).

**--read-sample and --items (code-commit-3-tool.diff 131-245).**
- Every Worksheet's cells are read and resolved.
- Per P2 ruling 3, fonts, fills and borders are distinct values over the used formats, gradients counts `fill None`, and codes counts `Code`.
- W's columns, rows, merges, links, frozen and protected are read.
- items is asserted equal to the held table's length, and sst_items_equal is asserted.
- `--items` prints only `items\titems=N`.
- Both write the input back unedited (lines 122-129).

Nothing beyond the spec in the tool diff, apart from (g) and (h) below.

**Changes the spec does not call for, or interpretations it does not state:**
- (a) **Range refusals.** Values outside a writer's range are refused, not only parse failures. Examples: zoom (1202), column width above 255 (1333), height above 409 (1351), header/footer text over 255 characters or with forbidden characters (1281), and the tab tint. The report discloses this (report-draft.md 25); planning should rule on it.
- (b) **First page number.** `firstPageNumber` is read only when `useFirstPageNumber` is true (1108), and `orientation="default"` reads as None (1094). Both are tested, but neither is in the spec or disclosed in the report.
- (c) **sheetFormatPr duplicate.** A second `<sheetFormatPr>` is refused with TWO_SETTINGS, whose text does not name sheetFormatPr. This follows the writers' precedent cited in the crate diff at 323-324.
- (d) **Url wins.** A hyperlink carrying both an `r:id` and a `location` reads as Url, and the location is dropped silently (918).
- (e) **Rels per link.** `self.relationships(part)` is called once per hyperlink inside the loop (920). Whether that re-reads the rels part each time cannot be seen from the inputs. It is a cost question, not a spec breach.
- (f) **Doc links.** Five writer doc links were de-linked from intra-doc links to plain backticks (crate diff 335, 372, 387, 401, 438). This is necessary so `cargo doc --features read` builds, but the spec does not mention it.
- (g) **Per-sheet buffer.** The read sample collects each sheet's styled indices into a Vec before resolving them (tool diff 151), because `cells` borrows the package. Memory grows with a sheet's styled cells. This is the tool, not the crate.
- (h) **Vacuous assertion.** The `resolved == styled` assertion (tool diff 167) cannot fail on its own: a failed resolution returns an error first.
- (i) **Extra derives.** SheetSettings derives Default (643), and the types also derive Copy and Eq. Harmless.

**Misses:** none found against the committed spec as corrected by P2 rulings 1 to 3.

## 2. Do the logs contain every line the spec requires?

**Verdict: YES with reservations.**

**gate.log**
- Features off and on, both targets: 513, 533, 546, 566, 540 and 562 on host and i686 (3180-3258).
- clippy and doc with -D warnings over off, read, richtext, the pair, sheetprops, metadata and all features (3099-3179).
- The `pub fn` lines: 175 at the base and 179 at the phase commit, exactly the four calls under the read cfg line (2752-2805).
- The consumer crate fails with the features off and with richtext alone, and passes with read (3259-3364).
- The gate files are listed against the diff, with no self-test due (2469-2524).
- Commit messages are ASCII and end with the Co-Authored-By line only (2367-2398).
- The spec pin and the phase commit equal HEAD, with null and short-sha controls (21-77).
- The script is printed from line 79. The log ends with "gate log checks all as expected: yes".

**test.log**
- Workspace runs: 1059 on both targets (5576-5578). That is 20 tests added and 0 removed against P1's 1039 (4760-4787).
- The crate runs off, read, richtext and read with richtext on both targets, and all features (872) on host and i686 (4939-5076).
- The writers'-forms test runs only with every feature on, and the log accounts for it (5078-5090).
- Six planted changes, each passing unplanted and failing at its own assertion line (5449-5574), which satisfies the "test bites" rule.

**negative.log**
- Through the gate's debug binary, its hash matched to gate.log (1417-1430).
- Two `<sheetViews>` (NV) is refused with exit 2, no stdout and no output file; its control NVC passes.
- NZ, NR, NH and NP likewise, each with a control (1560-1660).

**run.log**
- --read-sample on host and i686 (2869-3030). Each shows 27 ok, files_identical yes 27, entry totals identical 870 with 0 changed, flagged, recompressed, dropped and added. Every field equals the survey's, and outputs are the same on both labels (3071-3076).
- The survey counts W's col, row, mergeCell, hyperlink, pane, protection and `<si>` independently (2390-2834), with sixteen planted per-field differences, all refused (3032-3070).
- No corpus content is printed; only 12-hex ids and counts appear. A filename grep over run.log's output finds only `.in.xlsx`/`.out.xlsx` stems.
- TMPDIR and corpus are on the same device, 2096, with /proc as the control (2860-2867, 3486-3490).
- Calc chain: 9 of 27 (2836-2841, 3022-3023).
- The keep, compared against P1's record commit 4264cd05: 1017 pairs, 56 new (3345-3420). The edit open check reports findings 0 and no new pair's output refused (4511, 4646-4731).
- Tier d --items under `--memory 2048` on both labels (4743-4803): items=4800000, output sha256 equal to input sha256, peak RSS 7720 kB (host) and 7536 kB (i686), both under 16384 kB.
- Records m3p5-p2, m3p5-open and m3p5-scale-d-items are installed (4733-4741, 4790-4796).

**Pins and scripts.** Every log pins 5acfcd85: gate 14 occurrences, test 4, negative 3, run 2, scan 5. Each prints its own script.

**spec-0001 rules**
- Control bytes, non-ASCII and U+2014: 0 by my grep across all logs and records, and by the scan's f_ctrl, f_nonascii and f_emdash (scan-pre-check.log 2577-2690).
- Forbidden shapes, chained, leading bang and assignment prefix: my grep finds 0 in every input file, and the scan agrees (2760-2789).
- Recursive removals: all in the ruled form (2791-2815).
- ANTHROPIC_API_KEY is reported unset in every log header.
- RUSTUP_AUTO_INSTALL=0 and CARGO_TERM_COLOR=never appear in every cargo-running log.
- Absolute-path grep hits are only script text building controls by concatenation and placeholders; there are no machine paths.

**Reservations**
- The final scan stage has not run yet, so only the pre-check is available (RESULT: PASS, line 2848).
- The planted-difference control runs on host only (3032), and only on the first ok workbook.
- The survey uses expat for worksheets and ElementTree only for rels (run.log 2403-2414). The spec names ElementTree; the report discloses this.
- The tier d record's .md table shows "lo not-run 1" but lists "none" under "could not run" (records/m3p5-scale-d-items...md). This is cosmetic.
- The standing CARGO_BUILD_JOBS departure is stated (gate.log 77).

## 3. Does every number, hash and quoted string in report-draft.md appear in a primary?

**Verdict: YES with reservations.**

Verified:
- **Line 5:** base 4264cd0 and its subject (gate.log 985, scan 39).
- **Line 7:** 5acfcd85 and cccada0e (gate.log 2400-2428).
- **Lines 8-10:** the commit ids, the three carry test names (gate.log 2550-2556), the seventh module (crate diff 1519-1523), and two tool tests (test.log names).
- **Line 14:** 3c9ab086 (scan 2577); the crate counts (gate.log 3180-3250); 175 and 179 (gate.log 2756).
- **Line 15:** 4eda6847 and 1059.
- **Line 16:** 9e02657a; NV, NZ (zoomScale 9), NR, NH and NP (negative.log 1560-1660).
- **Line 17:** 73fcbfd5; 27 ok; 870 identical and the zeros; 1017 pairs and 56 new (run.log 3361-3362); findings 0 (run.log 4511); 2048; 4800000; 7720, 7536 and 16384.
- **Line 18:** 544cfe27 is the sha256 of scan-pre-check.log as given; RESULT: PASS.
- **Line 19:** the record names (run.log 4811-4818).
- **Line 28:** 1040008, two workbooks (run.log 2778, 2787).

Without a primary, or disagreeing:
- **Line 24:** the amendment of the tool commit has no primary among the inputs. The report marks it as such.
- **Line 27:** "the schema's default is false" for `sheet` is stated in no input. That `sheet` defaults to true in the struct is supported by test.log 5490-5492 and the crate diff.
- **Line 28:** "reads the parts with expat ..., not ElementTree" partly disagrees with the survey's own docstring (run.log 2403-2414), which says rels are read with zipfile and ElementTree. The claim "to stay small" is not measured anywhere.
- **Omissions, not numbers:** the report does not mention interpretations (b) and (d) from question 1, the de-linked doc comments (f), or the tool's per-sheet index buffer (g).
