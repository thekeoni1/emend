Independent check 1

Implementer's header (not the checker's): a fresh general-purpose subagent, given only the files under <phase>/check1/, answered the three CLAUDE.md questions. Its inputs:

| File | sha256 |
|---|---|
| README.md | 15d42a1a3b324c5881c083511eaeadccca69bfd55bf47f2d1a8e1caac0efa668 |
| check.log | 41f34b4053312ead5c405c816053e7c9d4738dc1b70d23f6ca2c25261e792133 |
| compare-m5p6-2026-10-16-5e52758.md | 2faf8ff1e41ffec4182dc5481430350629a65a7da5f0e8629d9ce3f71cb626bc |
| gate.log | da7b322fc50f33c73727a73f34f966fd0c6af6769b0a901ef0e6a498081c1a5f |
| negative.log | b74a1500eb10f0a37a86732a2ea920b30cacefd2aebd32ba5fea0bda041fb237 |
| phase.diff | b3621abc71122227507eeea7aa1e016513efabb9a016687b158516a181414cd0 |
| report-draft.md | 2db72ef6989bc8d83c5121ad65bceba74344dd847ab4affc21422bf43c234c97 |
| run.log | 66e39a92f14a56e2ae27115678358c06995e7875ee906e1fa96e604b0222e5b4 |
| scan-pre-check.log | adf579dde9a4ad8c73c7eaea8d82906a293b21e23953c8c86f01ed9819f7ac2a |
| spec-0037-working.md | ce89ba9c957a650667ed1c856934be86d5327f6741db24e74e7d346c9ebff8d0 |
| test.log | 1c3bdc6b393d44889e26e7bb4b90b620b0fba5a1ff81626998f5c28b6e7a71f0 |

The answer, verbatim but for its two multiplication signs (U+00D7, in "x3" and "x11"), written here as the ASCII letter x so that the scan's non-ASCII form reads 0; the raw answer is kept under <phase>/superseded/scan-final-1:

I found no workbook cell content in any input.

**Q1. Does the diff do what the spec says for P2's code, and nothing else? PARTLY (mostly yes).**

- **Ruling I (EXIT trap).** `<phase>/check1/phase.diff` lines 589-631 of the diff replace the end-of-run count with `finish()` under `trap finish EXIT`. It prints "containers of the image remaining: N", sets rc=2 if any remain or the count cannot be made, keeps a non-zero status, and keeps the loop's "files ..." line. It is installed after `WORK=$(mktemp -d)`, so the argument refusals come before it. `<phase>/check1/negative.log` 2929-2955 (N7) shows a mid-loop die ("error: cannot append to summary.openpyxl.tsv", exit 2) that still prints the count line. One small point: a non-zero rc already on its way out is overwritten with 2 when containers remain. Planning's reading accepts that.
- **Rulings K/P (container start in `passthrough-record.py`).** `empty_timing()` and the "- Container start: ..." line appear in the .md only; `write_tsv` is unchanged. **Disagreement:** ruling P says the median is "carried into that directory's record pair", but the .txt half of the pair does not carry it. Nothing in the diff tests this path; it is tested only in `<phase>/check1/test.log` 2442-2505 (S1-S6).
- **`scripts/compare-record.py` and its test.** These match "The comparison runner" and rulings C, K, L, N and P: per-op rows, outcomes, entry totals, lo/lo_in (with lo_in taken from the host run), median and sum of wall time, median and maximum of RSS, exception classes, the scale table and the image sha256. Adversarial notes:
  - The scale section's prose "openpyxl under --memory 16384 --timeout 3600" is hard-coded, not read from run.openpyxl.tsv.
  - The op check on run.openpyxl.tsv is applied to corpus directories only, not to scale ones.
- **Nothing else?** The other changes are ROADMAP.md (1/1) and the spec (+150, rulings A-K). These belong to the doc commit 09d82ac, which planning verified. The per-file line counts I took from the diff match the report's numstat table.

**Q2. Do the logs contain every line the spec requires for P2? PARTLY.**

Present:
- **Ruling H:** one "non-empty stderr.openpyxl.txt files N; their total bytes M" line per openpyxl run directory in `<phase>/check1/run.log` (35 directories, e.g. line 4643).
- **Rulings K/P:** three empty runs and their median per openpyxl directory ("the empty runs' wall times: 0.51 0.54 0.52 s; median 0.52 s"), and the record check "names the container start ... once".
- **Ruling L:** the directory layout (`<phase>/check1/check.log` root lines).
- **Ruling M:** the headers at run.log 8416-11141, with tier c using the host xlsx-diff and tiers b-d recording lo "-".
- **Ruling N:** run.log 82 and the per-op headers.
- **Ruling O:** "tool argument none / --<op>-sample", and "the run row's tool_args is empty".
- **P2's assertion of every comparison-record row:** run.log 11202-11350 ("rows that differ 0" x3, "derived equal: yes", planted-cell control).
- **Ruling R items (1)-(5) in check.log:**
  - (1) the run log's sha256 and its end line (48-69);
  - (2) 143 of 143 records and the keep listing's sha256 (71-84);
  - (3) the restored `keep_pairs_ok` and its control (95-99), and "every record pair reads every check yes: 70 of 70: yes";
  - (4) the tree;
  - (5) the last line "...: yes" with "check.stderr: 0 bytes".
- **Ruling Q's two family controls:** check.log 88-94.
- **Disclosure of the failing run log:** `<phase>/check1/README.md` 17-35.

Missing or weak:
1. **Missing:** ruling Q's generator line, that every helper a `yn`/`ynnot` call names is defined in the script, with its planted undefined-name control. No primary contains it; I grepped "defined in" across all logs, the README and the scan.
2. **Unexplained discrepancy:** ruling Q puts the `keep_pairs_ok` call at "script line 3582", but check.log 67 quotes bash's "line 2850". No primary reconciles the two.
3. **Thin control:** for RK, check.log gives one labelled control and one positive case, not a pair of controls.
4. The .txt half of each openpyxl record pair is never shown to carry the median (see Q1).

**Q3. Does every number and quoted string in report-draft.md appear in a primary? PARTLY.**

Verified:
- All log sha256 values: I recomputed them from the check1 files and they match.
- `README.md` 15d42a1a is in `<phase>/check1/scan-pre-check.log`.
- The comparison record's 2faf8ff1 is in run.log and the scan.
- 160c2984 is in every log. ce89ba9c is in check.log, the README and the scan, and equals the hash of `<phase>/check1/spec-0037-working.md`.
- Commit hashes and subjects: test.log 23-35.
- Timings: the logs' end lines.
- Scan quotes: scan-pre-check.log 4416, 5878, 6444, 6446.
- The corpus, scale and ratio figures (870/0, 28/533, 0.10/0.81/8.10 x11, 673976, 8012, cap 137 320.13 16576384, tier d 344.36 9555796 0/8/1/2).
- 1435, 2870 and 0ad3c774; 143; 70; the 105-byte stderr and the quoted stderr line.
- 04ce3671, ffae65ef, 7c32179e, 84064a18, 568cd01f and b6dd7cee are in the README.
- The RERUN_NOTE items (refused at entry, "about 30 s") are in run.log line 4.
- The numstat line is marked "no log prints it"; the old-script `keep_pairs_ok` statement is marked "session's statement".

Not found in any primary, and not marked:
- c81d5833 (the generator as it was).
- 3a44144b, and `superseded/scan-1`.
- "gen/run.py" and "gen/defined.py" (check.log names only gen/check.py), and the sentence "Every generator now prints ..." (see Q2 item 1).
- "2026-10-16-5e52758-roundtrip-stopped-2" (run.log says only "moved aside under the corpus directory").
- Finding 2's "two-workbook subset without media". Only `RUN_SKIP_KEEP=yes` appears in run.log, and only in script text. The rest is in the spec, which is not a primary.
- Item 7, the waiter stopped for low memory. gate.log's "low memory" is an unrelated CARGO_BUILD_JOBS note.
- "check log ... exit 0, outer stderr 0 bytes". check.log prints only its own internal stderr count; no primary gives the outer exit or stderr.
- The scan pre-check's own hash adf579dd. It equals the file's sha256, but no primary prints it.

Differs or is overstated:
- "The empty-image median per run directory is 0.46 to 0.53 s" holds for the 11 corpus rows of the detail table only. The scale directories reach 0.55 and 0.56 (the comparison record's scale table; run.log medians up to 0.56).
- "the median in the record pair": only the .md carries it.
- "the run log failed three of its own checks": run.log has four ": no" check lines (6241, 6248, 6693, 11515). check.log counts three because it treats 6241 as a sub-check.

Placeholders still to fill: RECORD_COMMIT, SCAN_SHA, INDEPENDENT_CHECK.

**Overall verdict: PARTLY.** The code and the measurements check out against the primaries. The report needs these fixes:
- ruling Q's generator line is absent from every log;
- the median is not carried in the .txt half of the record pair;
- about nine report statements are unsourced and unmarked;
- the "0.46 to 0.53" range is overstated.
