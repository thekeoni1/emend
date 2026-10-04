# M3 part 4 P2, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec specs/0026 with "Planning verification and rulings on P1", planning's working specs/0026 with "Planning rulings during P2" 1 and 2, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, the stderr files and the runner's summary, the phase's records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p4-p2. Checked commit (the phase commit): 8416a1eff26d80194df775cb2ce97933f9f5a75e; HEAD when the check ran: 8416a1eff26d80194df775cb2ce97933f9f5a75e. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to c9af26fcb5abef5f489a76529a7696d04860b64fbe72bee2701d10a23929a0be. Its items on the report are corrected in the report; its observations on the code and the logs are reported to planning. No second check was run.

Inputs and their sha256:

- 83b4fa0f9c945fdf3b659b5a91c2f74c3fb74beed3dc1f389d92c5733dc462d7 code-commit-1-carry.diff
- 239bfc3a6be0a20b56430feb24b22821ca8b55c1509d16de948a1da3738bb3c6 code-commit-2-crate.diff
- 19ca9a3b6424796f939e3ec9f40905c1c10729e4fd1dd18566c29afa10879548 code-commit-3-tool.diff
- 0f15d2f47c12f67fabc06d8ebd30920c67d5a70cefd9647df12f21e6d371aaba commit-messages.txt
- 16c7e0fa97e94ef8a0f2fc1f84d6ac014731d9541240b31cffddf63af84c04e6 doc-commit.diff
- 0bfcde5dd67af80bf2ac0c950b18114db67394cf34bc3809285daf8300af3f1f gate.log
- 7b826b3c7236d99584a0ad52c5a60a6682176ecd1e6225269b11d3c9aaecf6eb logged.out
- 1f27d1ba8a6e7ca13348e9e44ad2c2623499eee84bbbdb4e264c9a70bc2d8fa8 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- 3cf6b3a557645ffb31705867cb3802d79333b84981612d3e1d940a109bc739ff phase.diff
- a6c1ecc39af4ba3c6a10e6df161162442ae95ca91a5c455dac0e1bb8f7ded406 records/m3p4-open-2026-10-04-8416a1e.md
- 9130feaeeeacd5bc76b5e082b20687123a11e2841ecb69b16540f079ae0c2e13 records/m3p4-open-2026-10-04-8416a1e.txt
- 1a22dd0e9a302ac9c7d9472c930c2a94e249f58c00e0db0af4b2511c9d9e66aa records/m3p4-p2-2026-10-04-8416a1e.md
- 2f24b44bc0a12f0fe5b7d3814fb02bad8a466fa376af151bb4aca776600d53a7 records/m3p4-p2-2026-10-04-8416a1e.txt
- 58feaa5172638c8eff69606c1b14954ff538fb88edfe02d5d595d27721f7da32 report-draft.md
- 53cf2cb40b5c3ad9544ae07be5ab34e975100a7f187e1c298f82159e0d709489 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- dfcf9a54e7d25f099617eb9f881d3958d647ae12384d866badd172bbf78ffa02 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 5f0a819260188346141e78d0c7dde14a092367283cbcf30a1a5f72aee848ebe2 spec-0026-committed.md
- b69da8a333d5b74333f552b11c1edde2cb7390145cd8a27543f6f0cea63bafb7 spec-0026-working.md
- ed1b5376576b78730d84a43eb2b8f371bd8b2a142a6a00c1e9bd9916d610baa9 test.log

## The answer

**Hash verification.** All 23 inputs match `inputs.sha256` ("OK" for each). I also recomputed the diffs. The tail of each per-commit diff (from its first `diff --git` line) and the whole of phase.diff hash to the five values gate.log prints at lines 2177-2186 (41250ae2, 22ef40e4, ac164d08, 565701c2, 3cf6b3a5). The four commit diffs together hold 150 + 74 + 401 + 259 = 884 changed lines, the same as phase.diff. phase.diff touches 8 files, the same 8 gate.log lists at lines 2191-2198.

**Corpus content.** None found in the inputs. run.log and the records name workbooks only by 12-hex ids and print counts, `<col>` attribute names and yes/no flags. No cell text appears.

---

## Question 1. Does the diff do what the spec says for P2, including the carry, and nothing else?

**Verdict: yes on substance, partly on "nothing else".** There are two benign ungated core changes (both disclosed), one documentation gap in the tool, and a few small notes.

**What is present and matches:**

- **Doc commit c7c9e39.**
  - It touches only `ROADMAP.md` and `specs/0026`.
  - gate.log 2146-2174 checks both blobs against planning's hashes 5f0a8192... and 540b78b1....
  - My own sha256 of spec-0026-committed.md is 5f0a8192..., which is the pinned value.
- **The carry, d1278c3 (P1 verification rulings 2 and 4).**
  - Ruling 2, the whitespace merge: `ColsOut::between` holds a whitespace-only run in `gap` while a piece waits.
    - `gap` is cleared when the next piece merges.
    - It is appended after the piece in `flush` otherwise.
    - A comment goes through `kept` and ends the merge.
    - The indented-`<cols>` test is `pieces_merge_across_whitespace_alone_and_not_across_a_comment`, covering the merge, the comment and the unequal case.
  - Ruling 4, finding 5: the group sample's `//!` comment gains "with the rows M + 1 to M + 3 all present after the write". gate.log 2304-2306 confirms that every changed line of the tool's main.rs in this commit is a `//!` line.
  - Ruling 4, finding 6: test `hidden_flags_alone_write_today_s_split_without_the_coalescing`.
- **The crate, 9d266f0.**
  - `set_row_style` and `set_column_style` are both `#[cfg(feature = "rowcol")]`.
  - `LineField::Style` and `LineFormat::style` are gated.
  - `writes_when_absent` creates a row or `<col>` only for `Some(Some(_))`.
  - The coalescing trigger gains `LineField::Style`, as P1 ruling 2 lists.
  - In `line_formatted`, `s` then `customFormat` (row) or `style` (col) is applied before size, hidden, outline and collapsed. That is P2 ruling 2, and the test `a_row_style_with_a_group_on_the_same_row_writes_the_style_first` shows `s="1" customFormat="1" hidden="1" outlineLevel="1"`.
  - The index check: `line_styles()` is chained into the existing `STYLE_OUT_OF_RANGE` loop against `cell_xfs.len() + additions`. This is decision 5 A, with no new message (negative.log 1324-1325: "new 0; gone 0").
  - The P2 test list is all present in linestyle.rs, 13 tests:
    - present row
    - absent row created between present rows
    - None removing both attributes
    - split in the middle, at each end, and over the whole element coalesced back to one
    - uncovered columns given one created element
    - None removing `style`, and a bare `<col>` dropped
    - an index past `cellXfs` refused at the write
    - an index `add_style` returns, accepted
    - refusals at the call
    - a shift in the same write, and a removed part
    - kept through `insert_rows` and then `insert_columns`
- **The tool, 8416a1e.**
  - `--linestyle-sample` adds a bold font on format 0, then `set_row_style(W, M+1, i)` and `set_column_style(W, 9, i)`.
  - It prints the spec's line in the spec's order. `rows_present` comes after `rows_before` (P2 ruling 1), and it asserts `rows_after == rows_before + 1 - rows_present`.
  - It asserts `row_style_ok`, `col_style_ok`, `kept_cols`, and `xfs_after = xfs_before + 1`.
  - W is reread with the tool's own token pass (`group_facts`).
  - The two tool tests cover the split case, the present-row case and a refused full sheet.

**Beyond the spec, or worth recording:**

1. **Ungated core changes, built without rowcol.** Both keep the same behaviour, and the report discloses both.
   - `ColsOut::between` exists in every build. Without rowcol it is just `self.kept(raw, false)`, which is identical to the old `out.kept(raw, false)`.
   - The write's style check now always calls `self.edits.line_styles()`. The `#[cfg(not(feature = "rowcol"))]` stub returns `Vec::new()`. The loop was rewritten from `if let Some(style) = style && ...` to `filter_map(...).chain(line_styles)`.
   - The spec gates "everything below" behind rowcol. These are small additions to the core path, outside the gate. Nothing measures them until P3's size log.
2. **The tool's doc comment omits an assertion.** `linestyle_read_back` asserts `after.in_window != 1` ("row count") and `split` in 1..=3 ("col split"). Its "Asserted:" list names neither. This is the same kind of gap that P1 finding 5 / ruling 4 had fixed for the group sample in this very phase.
3. **Index check counts overridden calls.** `line_styles()` collects every recorded `Style(Some(_))`, not only the last per index. An out-of-range index that a later call replaced still refuses the write, although "the last call for an index wins". This matches how cell `style_edits` are already checked (the loop iterates all of them), so it is consistent with "as it is for a cell". The spec does not address it.
4. **`is_ascii_whitespace` includes form feed (0x0C),** which is not XML whitespace. It is illegal in XML 1.0, so it is unreachable in practice.
5. **Untested case:** a `<col>` that already covers exactly column 9, given `Some(i)`. It is exercised only through the None test (`<col min="3" max="3" style="3"/>`) and the corpus "alone" cases (5 workbooks, case=2 in run.log).
6. **Commit message wording.** The 8416a1e message says "Two tool tests" and also "No log prints a count for this commit". test.log 4582 does later print that count ("tests added to tools/emend-copy/tests/null_rewrite.rs: 2"). The rule is satisfied as worded at commit time, but the sentence is now inaccurate.

---

## Question 2. Do the logs contain every line the spec requires for P2?

**Verdict: partly.** Every requirement has a matching log line except the scan log's final stage, which does not exist yet. One control covers less than it could.

**Requirements and their log lines:**

| Requirement | Matching log lines |
| --- | --- |
| Doc and code structure, spec pin | gate.log 22-27, 2085-2111, 2146-2174 |
| Spec pinned in every log | test.log 24-27; negative.log 71-72; run.log 2253-2256; scan-pre-check.log 53, 2544-2548 ("4 of 4") |
| Commit messages: ASCII, Co-Authored-By only, count rule | gate.log 2113-2144; scan-pre-check.log 2413-2483 |
| Gate files listed against the diff, no self-test | gate.log 2211-2262 |
| Crate built and tested with features off/on on both targets, plus sheetprops alone (P1 verification rulings 3 and 6) | gate.log 2819-2869, line 2860 "host off 513, host sp 540, host on 583, host pair 611, i686 ..." |
| `pub fn` lines off and on; the two calls under rowcol only | gate.log 2634-2654; consumer crate 2908-2922 |
| test.log on both targets with the feature off and on | test.log 3317, 4385 (1019 each); 4631-4701 (513/583/611 per target, linestyle 13) |
| Added tests listed and passing | test.log 4554-4625 (17 added: 2 carry + 13 linestyle + 2 tool) |
| Negative: last row 1048576 refused, with a control | negative.log 1419-1461 (N1 exit 2 with the index message; N1C control exit 0) |
| No new message; unreachable message with its test | negative.log 1324-1325, 1483-1493 |
| Run on both targets, lo / lo_in ok, order_preserved yes, files_identical no | run.log 3731-3737, 3852-3858 |
| Only W and xl/styles.xml changed; descriptor flag only | run.log 3793, 3806, 3677-3679 |
| Calc chain never dropped, counted with its rule | run.log 2603, 3812-3813, 3938 |
| Present row edited in place / absent row created (P2 ruling 1) | run.log 3810-3811, 3931-3932 ("2; the survey's: 2"); row_window and row_attrs flags |
| `<col>` split checked as for part 3's width sample | cols_split, cols_bytes flags |
| styles.xml: cellXfs and fonts one longer, prefix byte-identical | styles_xfs, styles_fonts, styles_rest flags |
| A planted control per check | run.log 3944-4017 (17 of 17 on each case) |
| Edit open check over the keep, naming P1's record commit | run.log 4305-4338, 4399-4418, 5380-5410 |
| TMPDIR and corpus devices | run.log 3689, 4401 |
| Records written as `records/m3p4-p2-<date>-<sha>` | run.log 4181-4186 |
| Shape greps, rm -rf form, control bytes, privacy scan | each log's first block; scan-pre-check.log 2271-2311, 2656-2707 |
| Standard error empty | each log, "this script wrote nothing to its standard error" |

**Gaps:**

- **The scan log is not final.** "Phases" requires `scan.log` as the last log, with the check answers in it. Only the pre-check stage exists. Its line 2555 says "check answers present: 0 (stage pre-check ...)". This is expected at this point, but the requirement is not yet met.
- **The planted-flag controls never hit a present row.** Both control workbooks have `present=0`: ae7e17525a35 is a created column and 0fbf4e5e3be9 is a split column (run.log 3945; their lines at 3742-3749). So the plants never exercise the present-row branch of row_window/row_attrs (P2 ruling 1).
  - Each flag still has a control, which is what the spec says.
  - The ruling-1 path is shown only by the 2 corpus workbooks with edited_rows=1 (75387404b8af, 7935c5119065) and by `line_as_spec`'s control at run.log 3661.
- **P1 verification ruling 1** ("the survey counts `hidden` as `1` or `true` from P2 on") has no P2 log line. The linestyle sample does not touch `hidden`, so I read this as applying to the group sample, which P3 reruns. Flagged only so planning can confirm.

---

## Question 3. Does every number and quoted string in report-draft.md appear in a primary?

**Verdict: partly.** Almost every figure matches a primary. The exceptions:

1. **"scan.log: pre-check RESULT: PASS (dfcf9a54...)".** "RESULT: PASS" is the last line of scan-pre-check.log. The hash dfcf9a54 appears in no log, record, diff or spec. It is only the hash of the input file (inputs.sha256, and my sha256sum), since a scan cannot hash itself. The report also calls the file "scan.log", but the primary is the pre-check stage (scan-pre-check.log, line 2 "pre-check"), not the final scan.log.
2. **The disclosures** (the stray `cat >`, the pkill that killed the session's own shell, the plain `rm` of an empty file) have no primary. The report says so ("no primary unless named").
3. **"The negative script's stale-token check runs from its usage line on, the carried P1 header naming P1 by design."** No primary records this check. negative.log's carry header (lines 104-110) describes the carry but no token grep. gate.log 674-676 and test.log 748 describe such greps for their own scripts, not the negative script's.
4. **P2 ruling 2 paraphrased.** The report says "written before the size, hidden and outline attributes (P2 ruling 2)". The ruling reads "before the size, hidden, outline and collapsed attributes". The code puts style first, so before collapsed too. The paraphrase is incomplete, not wrong.

**Checked and matching:**

- **Commits and their messages.**
  - The base subject "M3 part 4 P1 record: logs, the run and open-check records, the independent check" is at scan-pre-check.log 39.
  - c7c9e39 / d1278c3 / 9d266f0 / 8416a1e and their contents match the diffs and commit-messages.txt.
  - 5f0a8192 and 540b78b1 are at gate.log 2150-2156.
- **Log hashes.** 0bfcde5d, ed1b5376, 1f27d1ba and 53cf2cb4 are at scan-pre-check.log 2499-2529, and my own sha256 of each file agrees.
- **Figures and strings by log:**

| Log | Report figures and strings found there |
| --- | --- |
| gate.log | 513 / 540 / 583 / 611 on host and i686 (line 2860); exactly the two calls (2638-2641) |
| test.log | 1019 on both targets (3317, 4385); six plants at six distinct assertion lines (5198-5199) |
| negative.log | N1/N2 with controls; no new message; "a style index is out of range" stated unreachable with its test (1483-1493) |
| run.log | 27 ok on each label; identical 816, changed 54, flagged 34, recompressed 0, dropped 0, added 0 (3735, 3856; the records' total rows agree); 2 workbooks edited in place (2598, 3810); column 9 created 11 / split 11 / alone 5 (2599); calc chain 9 of 27 (2603, 3938); device 2096 (3689, 4401); 838 pairs (4301, 5275); no output refused on an accepted input (5413-5417); P1's pairs unmoved (5382-5383) |
| records/ | the four record names (scan-pre-check.log 2556-2586; git status) |

---

## Summary

1. **Diff vs spec: yes on substance, partly on "nothing else".**
   - Everything P2, the carry and P2 rulings 1 and 2 require is present, and the diffs reproduce gate.log's hashes.
   - Beyond the rowcol gate: `ColsOut::between` and a `line_styles()` stub with a rewritten check loop in every build. Both keep the same behaviour and both are disclosed.
   - The tool's doc comment omits two assertions it makes (`in_window == 1`, `col_split` in 1..=3).
   - Minor: overridden style calls are still index-checked (consistent with cells); the exact-cover `Some` case is tested only on the corpus.
2. **Logs: partly.**
   - Every P2 requirement has a log line, except the final `scan.log`, which does not exist yet (pre-check only).
   - The planted-flag controls never use a present-row workbook.
   - P1 verification ruling 1's survey note has no P2 line; it probably applies to P3.
3. **Report: partly.**
   - Every count matches a primary.
   - dfcf9a54 appears in no primary, and "scan.log" names a stage that is not final.
   - The stale-token check and the shell incident have no primary (the latter is self-declared as such).
   - P2 ruling 2 is paraphrased without "collapsed".
