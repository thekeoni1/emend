# M8 part 1 P1: independent check 1

The answer of a fresh subagent given only the spec (as committed and as
working, with rulings A and B), specs/0001, the per-commit and combined
diffs, gate.log, run.log, the superseded run.log, the logs README as it
stood before the check, the scan pre-check log, the record and the draft
report, under <scratch>/check1. Extracted from the subagent's transcript
verbatim (sha256 90539cf62afe5d82f458e79fdf9e67e6ed84ff9b1b985ff95962032128592e6c); nothing is edited.
The README's section "After independent check 1" answers each exception.

root: <scratch> stands for the implementing session's scratch directory
root: <work> stands for run.log's working directory, as run.log's own root line says

1. **Diff: yes, with one minor exception.**
- Scope: the per-commit diffs match the combined diff line for line. They touch only five files: `ROADMAP.md`, the spec 0047 file, `scripts/lo-tables.py`, `tools/emend-copy/src/main.rs` and `tools/emend-copy/tests/keep_output.rs` (diffs/combined-06b96e5..01b0896.diff, lines 1, 104, 548, 897, 1454). The spec in bfcfb82 hashes to d966abd0..., the same as spec-0047-committed.md. The doc commit's ROADMAP block has the P1 row IN PROGRESS (diffs/bfcfb82.diff, line 111).
- `lo-tables.py`, workbook columns (diffs/12a57fc.diff, lines 137-175): sheet count, each sheet's state with visible as the default, externalReference count, sheetName count over externalLink parts, bookViews present, first view's activeTab or `absent`, pivotCache count, and per definition kept, missing or none.
- `lo-tables.py`, conversion columns (lines 200-216): table count, name length, url (`://` or `file:`), hash, `table:print` or absent, and the style's `table:display` or absent.
- Only lengths and booleans are printed (lines 69-71). `--root` makes paths relative to the corpus (lines 223-230).
- The printable-ASCII refusal is at lines 219-220 and 254-256: exit 2, one "refused: " line, nothing on stdout.
- The self-test builds fixtures in a temporary directory (line 373). It covers every column, including a name with `://` and `#` and names without (lines 348-354, 406-415). A marker string must never appear in the output (lines 399, 416). The refusal is tested with a fixture name holding byte 0xE9, and its control is the same file under a printable name (lines 423-438).
- emend-copy: `discard_output` keeps the output only when the variable is exactly "1" (diffs/01b0896.diff, lines 72-84). The error line gains " (output kept)" (lines 89-100). Every earlier `fs::remove_file(output)` after a written output is replaced, 56 calls in all; the only remaining `remove_file` is inside `discard_output` (line 71).
- The test uses a fictional workbook with no bookViews (lines 584-868). It covers four cases: variable unset, set to 1, other values, and a refusal before any output is written.
- Exception: the keep also applies to the post-read-back count checks (gradient, namedstyle and remove; lines 334-355, 365-386, 512-516) and to every sample, not only "a failed read-back" in the narrow sense. Ruling A accepts this ("the keep-output variable acts for every sample"), so I don't count it as a defect. Nothing else in the diff goes beyond the spec.

2. **Logs: yes, with exceptions.**
- Self-test:
  - In the gate: logs/gate.log lines 9681-9711 (19 ok, PASS), plus a planted-defect control that ends FAIL.
  - In the rerun: logs/run.log lines 2709-2738 ("lines 20: ok 19, FAIL 0", PASS).
- Tool test in the gate: logs/gate.log lines 10201-10221, "4 passed".
- Prune:
  - The control with the same `rm -r` form is at logs/gate.log lines 184-189.
  - The three roots are removed by `rm -r "${PREV_DIR:?}/${n:?}"` at lines 239-243, 278-282 and 335-339, with du sizes 6227787948, 6227466908 and 2146536849.
  - Each removal shows "gone" and "every kept name is still there". PRUNE_PREVIOUS is yes (line 11).
- Five rebuilds into the named run directory: logs/run.log lines 2740-2780.
  - Each id reads error, exit 2, the composed "(output kept)" line, and the output is kept with bytes and sha256 printed.
  - Change TR's set comparison and its "one id replaced" control are at lines 2747-2749.
- The script over the listed directories, with paths relative to the corpus: logs/run.log lines 2789-2958, 27 of 27 files. The group 2 identity checks are at lines 2959-2964.
- The corpus is written nowhere else: logs/run.log lines 2966-2971.
- Record assembled by the run: logs/run.log lines 2973-3324, sha256 b4418552..., 24333 bytes. This matches the record file's sha256sum and wc.
- Superseded run: logs/superseded-run1-run.log line 2734 ("in that order (change TK): no") and its last line "no".
- Rerun note and change TR named: logs/run.log lines 4 and 67. The superseded run.log's sha and bytes are at line 69.
- FINDING lines: logs/run.log line 3328 reads 0. The gate has none outside its printed script, and no UNEXPECTED line outside it either.
- Exceptions:
  - (a) Ruling B asks for "all three moves by the rerun's logged driver with mv and printed before and after". No supplied log has the mv commands or the before/after listings. run.log only names the moves in its note (lines 4 and 67) and hashes the superseded run.log (line 69).
  - (b) No log prints the first record's sha256 7801689e... or the moved run directory's contents.
  - The rerun driver's own output (rerun.sh) is not among the logs; if it exists it should be committed or cited.

3. **Numbers and quoted strings: no, several do not appear in a primary.**
- Verified:
  - Every sha and byte count of gate.log, run.log, the superseded run.log and the record.
  - d966abd0 and 8bc1c702 (the spec files), eb859312 (gate.log line 12), da7ed1ac (diffs/bfcfb82.diff header).
  - The run times 03:05:50, 03:18:11 (gate.log 448/11514; superseded run.log 66), 03:22:13 and 03:22:18 (run.log 66/3343).
  - gen/gate.py 20951cb0 (gate.log line 169). 43de576f, d5bfe37b, c6db7afc, 7801689e (prefix) and the d1 leftover sizes come from the working spec's rulings A and B.
  - 19 ok; the composed error line; "statements against the facts 0" and 3 classifications supported (run.log); scan RESULT PASS with 0 structural failures and 0 hits (scan-pre.log lines 6561-6565).
- Not found in any primary:
  - report-draft.md line 15: d2 gate.log 54dcca67, run.log 96009ca4, "scan the same 3".
  - report-draft.md line 16: d3 run.log 28ebf9b2.
  - report-draft.md line 25 and logs/README.md line 110: "23 files, 302161 bytes".
  - logs/README.md line 89: gen/run.py 4a7c9081 and script 16aa9688. The gate log hashes gen/run.py as 8cf37324 (gate.log line 175), which is a different value, presumably from before change TR.
  - logs/README.md line 96: superseded script 546cfe75.
  - logs/README.md lines 114-115: the full first-record sha256 7801689e7f48...; only the 8-digit prefix is in a primary (the working spec).
  - logs/README.md lines 146-149: trtest 192, tmp.Fb4nfrUMUU 5475396, tmp.nGVjSR7RgH 5475513. run.log names its work directory only as `<work>`, so "the one run.log names" cannot be checked.
  - logs/README.md lines 79-80 and 107: the driver names logged.sh and rerun.sh, and "printed before and after", appear in no log.
- Non-ASCII check: none found in report-draft.md, logs/README.md or the record. No private sheet, table or file name appears; workbooks are named only by 12-hex ids.
