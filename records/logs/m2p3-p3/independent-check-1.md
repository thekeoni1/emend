# Independent check 1, M2 part 3 P3

Written by the implementing session: the checked phase commit is
f52684c0fadc4f77485b6113f8fee03c357defcd, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
<scratch>/p3/check1, holding these files, by sha256:

    b5dc3ae3a735174fd0d6047a7c9a7ca2627b8ecca9a7deabde56da9c2abadbad  diff-b675f3d.patch
    8562f3c4a5b95048399bbe7605b301c8ab5aba0254a6a5c300c7d22c9279367c  diff-c3b5bbd.patch
    a55e7d49e0e53bb8e16f800ae0f7b7582a9b3569a45dac511c5f2752e2f06c72  diff-commands.txt
    23e7738ce0a3126174a0361be8739c454c4773bddfa13c3aa3bfa2cbbe124a26  diff-doc-ec68ccd.patch
    ff9d878e2fd3f6e29aaa333ffd2d3191754a3673d216ffc5a1ecd31b2f70aeb3  diff-f52684c.patch
    5ba58fc1c28f0f48ec752409c32f6a07584acd711b5b289d787799c694385794  diff-phase.patch
    b0daa22083a1760b7425ba6ab70af81c492b17b49c2b30f56cb1f3d62a6e598f  log-gate.log
    14aefcd7173b9a0a0ac2a00ae33a79064fa5c3eefc06ffba147316594a657952  log-negative.log
    bb2547aa65246344f6b2ec2d210c1e05f9da65ad290582184f31d33fdc997328  log-run.log
    17e487cc1e767ce125685decb7934347f210d36a0319ea6d37fa046d0bdae9e5  log-scan-precheck.log
    6b0d6ea68002990f3b10de7d0a2ad5578c04de78517de5ced43f59b3c43d57f6  log-size.log
    d797a135fe21a9c5013ce6427a3a2089bb431bde2f0e3b79133fb11ee68dfa82  log-test.log
    85d32022f216baf7a47c6f063519ddec81b36a62dd5f60fe432c1dd3957a33aa  m2p3-open-2026-09-26-f52684c.md
    c069b012421321e491a5534319594cb633854afa725d435cc1c925e919c625bb  m2p3-open-2026-09-26-f52684c.txt
    ba385c6df546083e5275b0bcc69a3e0c881775acb41e7dc1101f16b3166e0b52  m2p3-p3-delete-2026-09-26-f52684c.md
    82e43e31d88f7609c184699a6dd9c777f364ed9a1fc06af6aa5e38a7d2d66d51  m2p3-p3-delete-2026-09-26-f52684c.txt
    942fe8622010c9e787b8dda72df41d3d9468dacf54de27571672551d9ccc6d2a  m2p3-p3-insert-2026-09-26-f52684c.md
    b68dba95b3c3592691c9b82ca6665431321984fb1da47a993558a5d826442603  m2p3-p3-insert-2026-09-26-f52684c.txt
    6a70f5cb3f7ac9e37fb86f6c8bd2a56496d0ab6bc35fe5bf93be6b1fa4082d44  report-draft.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    59c50efe872074dac10f239a19a9e45740393a467fa8a6e2c0615de01d3ff914  specs-0018-rows-and-columns.md

It was not given the stderr files or test.log's superseded attempt 1;
check 2 is. The answer below the rule is the checker's final message,
verbatim, taken from its transcript (sha256 2297eefa62e20a750364b4fbe8c4618bd163b911c5f284e10574f7db3c04c205, 13979 bytes).

---

The harness put a CLAUDE.md project instructions file into my context. I have not used it as evidence.

I read only the files in `<scratch>/p3/check1`. Line numbers are those of the named file.

## Question 1: Does the diff do what the spec says for P3, and nothing else?

**Which diffs I read.** I read all four per-commit diffs in full, plus the parts of `diff-phase.patch` I needed. The blob indices agree: `diff-phase.patch` = `diff-doc-ec68ccd.patch` + `diff-b675f3d.patch` + `diff-c3b5bbd.patch` + `diff-f52684c.patch`, pre-image to post-image for each of the 9 files. No other commit is in the range.

**F1 (remark), `diff-doc-ec68ccd.patch`.** "Nothing else" confirmed. It touches only `ROADMAP.md` and `specs/0018-rows-and-columns.md` (blob 4050c0c). The hunks are the decision 7 A texts, "Crate", the clip/emptied sentence, "32-bit and streaming", "Size", P3/P4, decision 7 and the rulings through "Planning verification and rulings on P2". "Planning rulings during P3" is not in it; that section is in planning's uncommitted working copy, which is expected.

**F2 (remark), `diff-b675f3d.patch`.** "Nothing else" confirmed against the P3 first-code-commit list:
- `rowcol = []` in `crates/emend/Cargo.toml` (lines 9-13), and `features = ["dates", "rowcol"]` for the tool (line 558).
- `#[cfg(feature = "rowcol")]` on `insert_rows`, `delete_rows`, `TABLE`, `PIVOT_TABLE` (lines 86, 95, 154, 162).
- The five message constants gated (lines 301-314), and the shift pass items gated (lines 321-418).
- `mod shift` gated (line 429). The P1 `mod rowcol` is untouched.
- The one-shift check moved into `shift_alone` (lines 263-285). Its body is textually the removed block (lines 230-249), with `&part` becoming the `part` parameter.
- The P2 ruling 13 header sentence (lines 73-76).
- Six carried tests appended to `shift.rs` (lines 443-545): F4 to F8 and F10 as P2 verification rulings 1 and 3 list them. No existing test line is edited.

I checked the six tests' expected outputs against the shift rule and they agree. For example, `A3:A6` under delete(3,2) becomes `A3:A4`; `A1048575:B1048576` under insert(2,2) is dropped, and the dimension becomes `A1`.

**F3 (remark), `diff-c3b5bbd.patch`: the code against the shift rule on the column axis.** It agrees:
- **Arithmetic.** `AxisShift` wraps the P2 `Shift` and transposes areas through `rowwise` (lines 228-307), as P3 ruling 5 allows. The P2 test literals are unedited: the `shift.rs` hunk is append-only (lines 620-901).
- **Cells.** `on_axis = shift.of(reference)` (line 496). A move past the grid is refused `OFF_GRID` (line 498). A deleted column's non-empty cell sets `dropping` at the cell's depth, and the cell's end tag clears it (lines 512-518, 570-572), so the row survives. The row's `r` is untouched on the column axis (lines 436-441).
- **spans.** `spans_tag` (lines 314-346) matches P3 ruling 2: items through the rule and the clip, a removed item dropped, the attribute dropped when the list empties, the raw bytes kept when nothing changes, and a malformed item refused with `RANGE_UNPARSED`.
- **`<col>`.** `col_tag` (lines 354-361) refuses a malformed `min`/`max` through `col_interval` and does not check order or overlap (P3 ruling 1). `<cols>` is streamed through `LinksBuffer`, and an emptied container is dropped (lines 531-566). Clipping is in `interval_clipped` (lines 297-306): an interval wholly past the grid returns None, and `b` is clipped to the limit.
- **Formula refs.** `shifted_formula` transposes the area and moves the ref back (lines 591, 612). Grow and cut are refused as on P2's axis.
- **Tables and pivots.** `tables_before` uses `shift.of(area.last) >= at` (line 217), which is correct for "wholly before".
- **Other ranges.** dimension, merges and the lists go through `AxisShift::area_clipped`. Plant S7 in `log-test.log` shows the transposition is live for them.
- **Grid check.** It uses `GRID_COLUMNS` for the column calls (line 170).

"Nothing else" confirmed for this diff, apart from F4 and F5.

**F4 (remark), `diff-c3b5bbd.patch` lines 5-93.** This commit also extends the `package` module header's error catalogue and limits with the column calls and P3 rulings 1 and 2. "The catalogue and limits" is P4's line item. It is documentation and consistent with the code, but strictly it is ahead of its phase.

**F5 (remark), `diff-c3b5bbd.patch` lines 436-523.** The row-axis branch was re-indented and now shares the cell code. Two row-axis behaviours are new but have no effect:
- a cell `to > limit` check, which is unreachable because the row is refused first;
- a `None` arm that cannot fire while `dropping` is already set by the deleted row.

The P2 tests pass unedited on both targets (`log-test.log` lines 1661 and 2405ff), so row behaviour looks unchanged. I read this from the diff only; I cannot see the unchanged surrounding code.

**F6 (gap), `diff-c3b5bbd.patch` lines 697-749 against spec line 716.** The spec asks for tests of `<col>` "moved, grown, cut, dropped and the container dropped". The test drops a `<col>` only in the case where the container is also dropped. No case drops one element while others survive. Also, no column-axis test covers `autoFilter ref`; the other three lists are covered at lines 802-827. The run log's `cols_ok` and `lists_ok` flags hold over the corpus, which partly offsets this.

**F7 (remark), `diff-f52684c.patch`.** "Nothing else" confirmed:
- The tool's two flags and the usage string.
- `shift_sample`, `axis_counts`, `shift_read_back` and `table_last` generalise the row code over the axis. The row line keeps `from_k2_after` (line 200) and the row samples keep count 2 (line 142).
- The column lines carry exactly P3 ruling 3's fields (lines 158-190). c is `max(2, U+1)` with count 1.
- Three tests appended (lines 361-415).
- Three lines added inside the existing usage test (lines 347-350). The gate records this as the allowed "flag test's four" (`log-gate.log` lines 1885-1890). The spec's "P2 tests unedited" clause is about the shift tests, so this is only a note.

**F8 (remark), `diff-c3b5bbd.patch` lines 392-404 and 85-88.** `<cols>` is streamed under a shift and never held. The earlier module-header text from P1 verification ruling 4, which says `cols` is held whole, is not in any diff I have. I cannot confirm the header now reads consistently: `cols` held by the format write, streamed by a shift. Report disclosure 8 says it does.

## Question 2: Do the logs contain every line the spec requires?

**F9 (remark), `log-gate.log` lines 1940-1967.** Present: the crate's tests with the feature off and on, host and i686 (512, 543, 512, 543; 0 failed).

**F10 (remark), `log-gate.log` lines 1734-1880.** Present for P3 ruling 4(a). The `pub fn` and `pub const` first lines are listed at the phase commit, each beside its attribute line. Exactly the six items carry `#[cfg(feature = "rowcol")]`, none did at the base, and the two new first lines are `insert_columns` and `delete_columns`.

**F11 (remark), `log-gate.log` lines 1969-2017.** Present for P3 ruling 4(b). The consumer crate's `Cargo.toml` and `lib.rs` are printed. With the feature off, `cargo check` exits 101 with 6 error lines, each item named once. With it on, it exits 0 with no error lines. A control refuses output that names one item of six.

**F12 (remark), `log-test.log` lines 2418-2423.** Present: the later logs run the tests with the feature on. The workspace run proves it from its own output (31 shift tests, 8 with "column" in the name, on both targets).

A side note: the workspace run shows `unittests emend 557` (line 1478), while the gate's feature-on crate run shows 543. The difference is presumably the `dates` tests unified in through `emend-copy`. No log states that, and the report does not mention it.

**F13 (remark), `log-negative.log` lines 657-820.** Present: the column forms of P2's three fixtures, each with a control:
- S0/S1: the table case of ruling 9 on the column axis.
- S2/S3: the shared-formula case, with D2/D3 repeating it under the delete sample.
- S4/S5: the cell at column 16384.

Each refusal shows exit 2, the exact message, no stdout and no output file. The two usage shapes are there too.

**F14 (remark), `log-size.log` lines 551-622.** Present:
- the two probes rebuilt at f52684c with `rowcol` off, shown by `cargo tree` (0 "rowcol" lines for size-probe, 1 for the emend-copy control);
- the budget figure 963588 - 533216 = 430372 printed beside P1's 430372, as a comparison and not a check;
- no record file.

**F15 (remark), `log-run.log`.** Present:
- both column samples on both targets in two run directories;
- the refusal set and the unchanged-W set computed before the run (both 0);
- the per-flag plants, 10 of 10;
- host and i686 outputs byte-identical in 27 of 27;
- both record pairs and the keep;
- the open check and its record installed with sha256.

**F16 (gap), `log-run.log` lines 262-280 (script) and 300-305 (script) / output line 28.** The spec's P3 line (spec lines 734-739) and P2 verification ruling 4 require a moved run directory to be "listed and hashed as the keep is". The script has that code (change U), but P3 moved nothing, so it never ran. Unlike the keep rule, it has no control: the only related output line is line 28, "control: the moved-keep pair count reads 1 pair in a planted listing of one pair". So for the run-directory listing, the log shows neither a run nor a control.

**F17 (remark), `log-run.log` script line 291.** The "pairs times two equals files" check exists, but it sits inside the branch that only runs when a keep is moved, and none was. What ran is a control of the fixed count pattern (line 28). That is enough to show the P2 ruling 14 pattern defect is fixed. It does not show the check itself running.

**F18 (remark), `log-gate.log` lines 1383-1387.** Present: the commit-message count rule. Each of the three code commits' messages contains "No log prints a count for this commit; the diff alone carries what it changes." A folded-line control and a refusal control are there too.

Two limits:
- The doc commit ec68ccd's message is not checked for counts.
- No log prints any message body, so I cannot see which counts the messages state.

**F19 (remark), `log-test.log` lines 90-92, `log-negative.log` lines 109-111, `log-run.log` lines 120-121.** The printed scripts repeat `set -uo pipefail` three times (twice in run). It is harmless, but it points to carried-script accretion.

**F20 (remark).** The P3 test gate covers the feature-off configuration by test pass counts only. No clippy or warnings check is run with the feature off. The size log's probe builds, which use emend with the feature off, report "warnings or errors 0" (`log-size.log` lines 555, 559, 585, 589), which covers the library build but not the test configuration.

## Question 3: Does every number and quoted string in the report appear in a primary?

**F21 (defect), `report-draft.md` line 12.** "first attempt b7c83d9" and the quote "rulings during P2, 9 to 14" appear in no primary. The claim "the whole section 1 to 14 is new" is also unsupported by the inputs. The spec (line 1180) supports only that the message was "amended once before any log pinned it, the tree unchanged, which is accepted".

**F22 (defect), `report-draft.md` line 94.** The quoted count "three integration tests" appears in no primary; no log prints f52684c's message. The diff does add exactly three `#[test]` functions to `null_rewrite.rs` (`diff-f52684c.patch` lines 361, 379, 404), so the claim is consistent, but the quoted string is unsourced. The companion quote "the diff alone carries what it changes" is in `log-gate.log` line 459 and `log-scan-precheck.log` lines 1405-1408.

**F23 (defect), `report-draft.md` lines 86-89 (disclosure 3).** It says the moved-keep pair count and the moved run directory listing "are exercised only by their controls in this log". Only the pair count has a control (`log-run.log` line 28). The run-directory listing has none (see F16).

**F24 (gap), `report-draft.md` line 33 and lines 97-98.** "(renames listed in the message)" and disclosure 6's "which P2's did not; those files hold no pub fn" rest on the f52684c message, P2's gate log and the whole test files. None of those is among the inputs, so neither statement can be checked here. The gate command text (`log-gate.log` line 1735, "outside the tests") supports only the current behaviour.

**F25 (remark).** Every other figure and quoted string I checked appears in a primary:

| Report figure or string | Where it appears |
|---|---|
| spec sha256 59c50efe... | gate lines 2023-2025, negative line 50, size line 27, run line 2269 |
| doc blob hashes cd524d29..., 2efb06e1... | gate lines 1389-1413 |
| 512 / 543, host and i686; six error lines | gate lines 1964, 2002-2011 |
| 704 passed, 0 failed, 2 ignored per target | test lines 1661, 2405 |
| test attempt 1 and "header change Q" | test lines 42-45, 104-106 |
| shift table: 20 lines, 18 rows, 2 clip calls | test line 2588 |
| eight plants S1 to S7 and T1, each found once | test lines 2685-2793 |
| S0 at=3; S4 moved from column 16383 to 16384 | negative lines 761-802 |
| 963588, 533216, 430372, 457044, 26672 (as -26672), drift 0, 31196, 28380, 458752 | size lines 591, 587, 615-620 |
| 27 of 27 ok; identical 829, recompressed 0, changed 40, dropped 1 | run lines 3300-3302, 3702 |
| 10 of 10 plants; 11 with a formula, 1 chained; 27 byte-identical | run lines 3493, 3361, 3497 |
| keep: 664 files per target, 332 pairs; P2's 297 all present, 35 new | run lines 4167-4180 |
| open check: crate 280, tool 52; P2 verdicts unmoved | run lines 4760-4766, 4719 |
| record stems | run lines 3501, 3901, 4781 |
| "change S" | run line 102 |
| scan pre-check: 0 hits, 0 structural failures, stderr 0 bytes | scan-precheck lines 1654, 1658-1662 |
| stderr 0 bytes | gate 2033, test 2809, negative 836, size 633, run 4810 |
| `cargo tree` 0/1 (disclosure 7) | size line 580 |

That is the whole answer.
