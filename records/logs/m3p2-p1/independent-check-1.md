# M3 part 2 P1, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec and the working spec with planning's rulings during P1, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, each one's stderr, the phase's four records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p2-p1. Checked commit (the phase commit): 7d2f40d5177275d7cac1cae4a2c1141950dd7ac6; HEAD when the check ran: 7d2f40d5177275d7cac1cae4a2c1141950dd7ac6. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 83998bf8bb5ceb56490dd42347015518e458803700c2b25fe1887c645f66feca. Its items on the report are corrected in the report; its observations on the code and the logs are reported to planning. No second check was run.

Inputs and their sha256:

- 078869ce284457ce3f71aaf215df29d20c0c2cee8a4e8f72bb42d50e4b43c891 code-commit-1-carry.diff
- 8b4a594fe722006880b91b1981c2353e7568054a981b1f751f6ba70e23e407ee code-commit-2-crate.diff
- 91afe1450afa2e11dc18ecdd863ed5ba3fae4ed80e75736721a1abfba7cb48a8 code-commit-3-tool.diff
- 76e3101e17c306bcccd5f5b492df4e956738293f9f3555b913256762ebd46d4a commit-messages.txt
- 4ef86716a5937edb0150939289fe9cb4511076a4c2d39a9481da8aa81a8cbcaf doc-commit.diff
- 694dd7c537f7973deb25a358c1d8b2577d266120285156d24682a4154d8cbe8f gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- cff35a6398a0175ce00069703fe5828ab46d25fb260f0004055c9863df5794f3 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- dae4d69c77c62b34f553ea8b3d021d7aa5f3e24fdb8c3c8e9b3fccc48962ac7d phase.diff
- 3581a908a05a566b77b5b871c43660244c1fa027aaf43a801d6f1351daeca9e3 records/m3p2-open-2026-10-02-7d2f40d.md
- e5ba490dce18e861f2c790c5900dd172ea468e9446acc9aeb480841650501020 records/m3p2-open-2026-10-02-7d2f40d.txt
- 8116dc46cae9b7f195f4a430fc6a0276be2ae7fb78d0bf079facc4102d32c3be records/m3p2-p1-2026-10-02-7d2f40d.md
- 265cdbddbf62bb15f60bb0c9ad888992a1e5382193184e1d5a06a8729329cb97 records/m3p2-p1-2026-10-02-7d2f40d.txt
- 67a1ae43ad4f6e11882cc30e892226e50b4599332fbdb11623930f16471ef824 report-draft.md
- 763dafa8d073d05f5f987dc9dbadb22f5ac8ed66ac7ada28450196aac25270ae run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- a3df3f43b1260532efac3d01c0602ee8022366fbcf5e50050ffbe627b266837c scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- ae37a294c82c9ef5e2c8e918fe79e487016c01ae357042256018cc030e4bff6a spec-0024-committed.md
- b0b73334185b44a8539ef4b8ad81c2f693f765a568f670706b699e8798406340 spec-0024-working-rulings.md
- 87e45d6f6b3460a447ef7c30dd498eb70765a20f386946619394430b961435b9 test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

---

I am the independent checker for M3 part 2 P1. I read only files in <phase>/check1.

The harness put two documents into my context: the repository's CLAUDE.md and the user's auto-memory index (MEMORY.md). I did not use either as evidence. Line numbers below are each named file's own numbering, as `cat -n` gives it. The directory's diff files hash to the values gate.log prints, which I confirmed with sha256sum: phase.diff dae4d69c, doc-commit.diff 4ef86716, code-commit-1-carry.diff 078869ce, code-commit-2-crate.diff 8b4a594f and code-commit-3-tool.diff 91afe145 (gate.log lines 1885-1897). phase.diff holds exactly the union of the four per-commit diffs' added and removed lines.

## 1. Does the diff do what the spec says for P1, and nothing else?

Yes, with the observations below. I compared against spec-0024-working-rulings.md lines 405-437 (P1), 133-246 (Design) and 500-538 (rulings 1 to 3).

**doc-commit.diff**
- It touches exactly ROADMAP.md, specs/0023-scale.md and specs/0024-rich-text.md.
- The added 0024 file is byte-identical to spec-0024-committed.md. I extracted the added lines and compared them with diff: no difference. Its sha256 is ae37a294.
- I cannot confirm from primaries that ROADMAP.md and 0023 are "as planning left them". The only evidence is that their blobs match the hashes in the doc commit's message (commit-messages.txt lines 78-80; gate.log lines 1844-1883). Planning's own message is not in this directory.

**code-commit-1-carry.diff**
- Only //! lines change in package.rs: 6 removed, 7 added.
- The block now names records/logs/m3p1-p3/run.log beside m3p1-p2's, as spec-0024 line 400-403 and 0023 P3 ruling 1 require. gate.log lines 2049-2058 confirm 0 non-//! lines.
- Nothing else: confirmed.

**code-commit-2-crate.diff** has every P1 item:
- The feature `richtext = []` (lines 1-15).
- `Run` and `RichText`, with hand-written `PartialEq` and `Eq`, f64 compared by `to_bits`, and a rustdoc line on `Run` (ruling 1; lines ~395-500). `Font`'s derive is untouched.
- `Value::Rich` and its write (lines 185-200): inline, one `<r>` per run, `<rPr>` from `font.composed("rPr","rFont")`, `<t xml:space="preserve">` through the shared escaper.
- The write check `RichText::check` (line 516), using the same constants as `add_style`.
- `Cell::runs`, `SharedStrings::runs_of` over a sorted sparse `Vec<(u32, RichText)>`, and the `<rPr>` reader `read_properties` / `read_property` (lines 646-800). Ruling 2's readings are all implemented.
- The two messages, one constant each, catalogued in package.rs (ruling 3).
- 18 tests, which together cover every test the P1 paragraph lists.

Extras in the crate diff that the spec does not name. Each is either implied by the spec or reported:
- (a) `Font::compose` is split into `compose_as`, `composed` and `check` (styles/elements.rs, lines ~1489-1581). This is needed for the rPr writer and leaves no signature changed.
- (b) `TEXT_NOT_XML` becomes pub(crate) and cells uses it (line 877). This matches "one definition" in "Errors"; it is report finding 3.
- (c) The core's `text_under` now takes a listener, with an ungated `trait Runs` and `impl Runs for ()` (lines 367-391). Ruling 3 notes this. I traced the `()` path: an `<r>` sets depth 1, and an `<rPr>` the listener does not take falls back to depth += 1, so the core walk is unchanged.
- (d) A `text_escaped` helper factored out of `Value::Text` (line 209).
- (e) `Run` and `RichText` re-exported at the crate root (lib.rs).

Unspecified behaviours I note, none contrary to the spec:
- (i) A `<t>` after the runs becomes a run of its own (line 617), where the spec only rules on a `<t>` before them.
- (ii) An empty `<r/>`, or `<r><t/></r>`, reads as a run with empty text. The write refuses such a run, so ruling 2's principle ("every value the reader gives is one the write accepts") does not cover run text. No test covers this.
- (iii) A `tint` beside `rgb` or `indexed` is dropped from the typed view without a message. The catalogue lists only the five unmodelled children as dropped.

**code-commit-3-tool.diff**
- The tool manifest adds `richtext` to emend's features.
- `--richtext-sample` follows "Harness runs" (spec lines 315-327). Every counter is printed and every assertion is made, and a failure removes the output.
- Two tool tests are added, and the usage string updated.
- `package.set_cell` returns nothing; gate.log line 2466 lists `set_cell` with no return type, so errors surface at `write_into`.
- Nothing else: confirmed.

Report-draft.md line 18 says the write uses "Font::compose_as with element rPr and child rFont". The write branch actually calls `composed` (code-commit-2-crate.diff line 190), after `RichText::check` has run. `compose_as` exists, but only `compose` calls it.

## 2. Do the logs contain every required line, and do their checks support "all as expected: yes"?

**gate.log** (ends line 2677, "gate log checks all as expected: yes"):
- Feature off and on, both targets, for tests (lines 2588-2615); clippy and doc off, on and all-features (lines 2543-2586).
- The pub fn and pub const listing: off equals the base's 154 lines, on adds exactly `runs_of` and `text` (lines 2296-2495).
- The consumer crate names the five gated items: it fails off with one error line per item and passes on (lines 2617-2661).
- check.sh on a cold cache, nine PASS lines (lines 2517-2534).
- Gate files checked against the diff, no self-test needed (lines 1930-1991).
- Commit-message form (lines 1811-1842); doc hashes (lines 1844-1883).
- Weakness: gate.log only checks "on runs more tests than off" (line 2612). test.log closes this: exactly 18 names added on each target (test.log lines 3830-3832, 3852-3854).

**test.log** (ends line 4368, "yes"):
- Workspace 914 on each target, the features read from the run (lines 3598-3647).
- 20 tests added, matched to the source, each passing on both targets (lines 3649-3812).
- The crate 513 off and 531 on, both targets (lines 3814-3856).
- Ten plants, each found once, passing unplanted and failing planted, with the failing assertion printed (lines 4207-4350). The plant texts are in the printed script (lines 1360-1390).
- Every feature on in the crate run (lines 3866-3880).

**negative.log** (ends line 1141, "yes"):
- The spec's required case: R1, a sheet whose last row is 1048576, refused with INDEX_OUT_OF_RANGE's message and no output; R1C is its control (lines ~1090-1094).
- Extras R2 and R3 (unreadable `<rPr>` in the table and inline), each with a control, and two usage shapes.
- The message set: one reached, and RICH_TEXT_INVALID stated unreachable with its covering test (lines 1096-1110).
- negative.log writes its stderr as <scripts>/out/negative.stderr (line 1137), while gate.log and test.log use <phase>/out. All are declared, per scan-pre-check.log line 2436.

**run.log** (ends line 3997, "yes"):
- Ruling 2's shape survey comes before the run (lines 1792-1857).
- The independent count prints its FINDING (lines 1859-1921).
- Both targets' corpus runs (lines 2485-2705) and 13 planted-flag controls, each turning its own flag (lines 2708-2737).
- The comparison covers every row byte-identical except the appended row (`rest_same`), the cell's shape (`one_cell`, `two_runs`, `first_bold`, `second_plain`, `t_preserve`, `texts`, `cell_bytes`), the shared strings part byte-identical (`sst_same`, meaningful because 27 of 27 inputs have one), and `entries_table` (W changed, descriptor at most).
- The two labels' outputs are byte-identical (lines 2739-2743).
- The record pair and its 18 checks (lines 2745-2905).
- The keep on both targets, 1292 files each and identical (lines 2907-3022). The keep is compared with M3 part 1 P3's record, named by commit fe3c19e: 620 unchanged, 26 new (lines 3024-3121).
- edit-open-check: no verdict moved; crate 574 pairs and tool 72, 0 outputs refused on an accepted input (lines 3123-3963). Records installed and their paths checked (lines 3965-3979).

Weaknesses in run.log:
- (a) `summary_ok` accepts layer refusals (line 903 of run.log's printed script: ok plus refused equals rows). The spec expects every case to open, so a refused workbook would still pass. The run had 0 refusals (lines 2492 and 2601), so the result is unaffected, but the check is looser than the spec.
- (b) `survey_ok` (lines 636-643) checks child names, NONMAIN and rgb length only. It would accept a shape such as `val:NOTNUM` or an unknown `u` keyword. For the shared strings table the corpus run itself shows the reader accepted every table. For comment `<rPr>`s, which P1 never reads, the survey is the only evidence, and it does not check values. This matters for P2.
- (c) Printed but not checked against an expectation: entry flagged 17 (bounded only by `entries_table`), the record's XML out_only 187, and "inputs holding a shared strings part 27 of 27" (informational).
- (d) Line 1801 holds the survey script's corpus glob in a scrubbed form that is not one of the declared root placeholders. The script hard-codes the corpus location instead of taking <corpus>. I name it rather than quote it. The scan's placeholder check does not catch this form, because no slash follows the placeholder.

Timing: gate ran 23:38:10 to 23:41:41 (gate.log lines 60 and 2676) and test ran 23:41:41 to 23:42:58 (test.log lines 54 and 4367). 77 s for test.log, with its ten plant pairs that each need a rebuild, is short. No log prints per-step times, so I can neither confirm nor dispute it.

**scan-pre-check.log**: RESULT: PASS, total hits 0 (lines 2449 and 2453). Every stderr file in the directory is 0 bytes.

## 3. Do the numbers, hashes and quoted strings in report-draft.md appear in a primary?

Most do:
- Commit ids: gate.log lines 33-38.
- Spec, ROADMAP and 0023 hashes: gate.log lines 21-28 and 1844-1883.
- Log hashes: scan-pre-check.log lines 2231, 2241, 2251, 2261.
- Record hashes: run.log lines 2896-2899 and 3965-3973.
- 914, 513 and 531, and the 18 tests: test.log.
- Run figures 27, 843, 27, 0, 25, 2, 574, 72, 0: run.log lines 2490-2583, 2681-2682 and 3944-3949.
- 22, 7, 29, 113, 8, ed2fbd304d51 7/23, and 27 of 27: run.log lines 1907-1921 and 2577.
- Finding 2's test name and its cases: code-commit-2-crate.diff, richtext.rs.

Not found in any primary in this directory. I searched every file with grep, report-draft.md excluded:
- "a3df3f43" (report line 43). No log prints it. It is the sha256 I computed for check1/scan-pre-check.log.
- "superseded/scan-pre-check-1" (report line 42): appears nowhere.
- "92f06df" and "Amended once before any log ran" (report lines 23-24): appear nowhere.
- "the gate and test script author found the gap" (report line 70): no primary.

Partly inaccurate:
- The parenthetical "(Font::compose_as with element rPr and child rFont)" at report lines 17-18; see question 1.

Incomplete:
- Finding 1 omits that the spec's own fact says "8 of the 26 workbooks" (spec line 76) while it names 7 and totals 22. Its workbook count is therefore right, and its item total misses ed2fbd304d51. The corpus has 27 manifest rows (run.log line 2475) against the spec's 26. The report does not address that difference.
