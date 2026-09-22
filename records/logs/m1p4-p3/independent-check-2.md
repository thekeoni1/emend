# Independent check, M1 part 4 P3 (ed89a57..4b31e2a)

## 1. Does the diff do what the spec says for P3, and nothing else?

Yes, with the notes in the findings below. I read each commit on its own diff.

- **20fd731 (doc):** changes only ROADMAP.md (the P2 row) and specs/0011 (the P2 rulings, the P2 verification, and the corrected P2 clause). See diff-20fd731.diff:11-60. Its message is ASCII and ends with the Co-Authored-By line (commit-20fd731.txt:9).
- **18bfc36 (code 1):**
  - The one write site becomes a match whose Ok arm panics naming what was written, and the lock stays as it was (diff-18bfc36.diff:21-30). This is P3 ruling 1.
  - It adds `Error::ReadOutput` with "cannot read the output file" (diff-18bfc36.diff:42,50).
  - It moves the read-back into `read_back`, and the caller removes the output on any error after the rename, the `CellsAfter` refusal included (diff-18bfc36.diff:98-104,115-148). This is P2 ruling 3.
  - Nothing else changes.
- **858acf4 (code 2):**
  - It adds `--edit-formula-sample` as "Harness runs" and P3 rulings 4 to 6 describe it: Worksheet-kind sheets only, in `<sheets>` order; `FormulaKind::Normal`; `Number(1)`; chain_dropped measured from both reads; `CellsAfter` checked against the count before; read_back; every key 0 when there is no normal formula (diff-858acf4.diff:142-250).
  - It refactors `edit_line` (the message says so).
  - It adds four integration tests and three usage shapes (diff-858acf4.diff:264-484).
- **af5bba6 (code 3):** doc comments only. package.rs gets seven Limits items and cells.rs gets four (diff-af5bba6.diff:25-128).
- **4b31e2a (code 4):**
  - It adds the probe and its Cargo `[[bin]]` entry (diff-4b31e2a.diff).
  - The probe does the work "Size" lists: three sets from the command line, a clear, `drop_calc_chain`, a write to a second path, and the cell count read back through `Package::cells`.
  - One departure: it reopens the input rather than continuing from the first package. The probe's own doc comment explains this.
- **Combined diff:** exactly these 9 files (diff-combined:1-828). No non-ASCII byte appears in any message or diff.

## 2. Do the logs contain every line the spec requires?

Mostly yes. The gaps are findings 1 to 4.

- **Phase-commit controls (null commit and short sha):**
  - gate.log:7-8, test.log:14-15, size.log:7-8, negative.log:10-11 and run.log:7-8 all have both.
  - scan.log:6-7 has both, but line 6 is labelled "a commit that is not HEAD". Only its script, at scan.log:225-226, shows that commit is the null sha.
- **Planted fault per comparison flag:**
  - run.log:1413-1449 plants prefix, suffix, rows, refs, x_value and chain, and accept_one refuses each one.
  - The real-pair plant is at run.log:1579-1582 and 1697-1700 (added=1, refused).
- **`<row>` counts in input and output:** per workbook at run.log:1518-1571, totals 260616/260616 at run.log:1566. I re-added them and they match.
- **Path control fixtures assembled at run time:** only in run.log:938-942 and 1738, plus scan.log:932-950. See finding 1.
- **"Size":**
  - The nine probes are at size.log:1622-1668.
  - The eight earlier probes are re-measured with the byte classifier at size.log:1670-1900, each rebuild reproducing part 3's sha256.
  - The figures are at size.log:1901-1938.
  - Every figure in the record appears in the log (size.log:2039-2166).
- **Harness runs P3:**
  - Entry classes, descriptor-only on changed rows, and files_identical are checked by entries_ok and fid (run.log:661-690, 780-830).
  - Per-target counts of formulas replaced and chains dropped are at run.log:1564-1565 and 1682-1683.
  - The cell comparison (changed 1, added 0, removed 0) is at run.log:1518-1571.
  - lo and lo_in are 27 each (run.log:1502).
  - The record .txt confirms the 12 flagged entries per target are `changed descriptor` (m1p4-p3-...txt: 12 host and 12 i686 entry rows).
- **P3 paragraph and rulings 7 and 8:**
  - F4 and F5 go through `--edit-sample` and through the probe, with the probe's outputs' presence and byte counts and the source lines that explain them (negative.log:2757-2786).
  - F7 names the unreached message (negative.log:2802-2809).
  - P1's refusals are rerun (negative.log:2818-2975).
- **Ruling 1 counts:** test.log:2314-2363.
- **Ruling 3:** every log prints the spec's sha256 and every status check accounts for its line (gate.log:12,1052; test.log:11,2663; size.log:18,2173; negative.log:5,3308; run.log:17,1743; scan.log:1238).

## 3. Does every number and quoted string in report-draft.md appear in a primary?

I checked every sha256, byte count, test count, entry total, row total, size figure and message string against the logs, records and diffs. All of them match. I re-hashed the provided logs and records myself and they equal the printed hashes.

I re-did the size arithmetic and it holds:
- 811280 - 533216 = 278064
- 393216 - 278064 = 115152
- 811280 - 807180 = 4100
- 807180 - 780564 = 26616
- 278064 - 247348 = 30716 = 4100 + 26616
- 393216 - 247348 = 145868, and half of that is 72934
- 762100 - 717228 = 44872
- 811280 - 606004 = 205276

The report correctly marks as unlogged:
- the attempt-1 versus attempt-2 hash comparison (report-draft.md:73);
- attempt 1's run directory (report-draft.md:121);
- the build timings (report-draft.md:154-156).

The exceptions are findings 5 to 8.

## Findings

1. **(defect) Ruling 4's path control is missing from most P3 logs, and the report discloses only part of it.**
   - size.log:1142 and 2166 check the size record against a regex with no planted path. The report discloses this (report-draft.md:159-162).
   - gate.log:358-359 and 853 ("the printed tree carries no absolute path of this machine") are also a path check with no planted control. The report does not mention it.
   - test.log and negative.log have no path check at all.
   - Only run.log:938-942 meets the ruling.

2. **(note) The size reconciliation's checks cannot fail.**
   - size.log:1930-1931 and 1929 are algebraic identities of the figures they are computed from: (cells_now - zipxml_now) + (write_now - cells_now) = write_now - zipxml_now.
   - The "control" at size.log:1932 also always passes whenever part 4's own cost is above zero.
   - The arithmetic is right, but these lines assert nothing beyond the subtraction.

3. **(note) size.log:1919 and 1930 call 273964 "parts 1 to 3 measured at THIS commit".** That figure includes the 26616 of part 4 P1/P2 code, which the same log (size.log:1924) and the record (size record:84) attribute to part 4. The label contradicts both.

4. **(note) Planted faults do not cover every count.**
   - Nothing plants a removed cell, so `removed=0` is asserted but never shown to be detectable.
   - `new_cells`, which ruling 4 names, is not a flag of P3's comparison. Its role is covered only by the real-pair plant (added=1, run.log:1580).
   - The negative fixture for "a sheet lacks `r` on a row" (spec P3) is cells without `r` inside `<row r="1">` (negative.log:491). The row itself has `r`.

5. **(note) The cause of the drift is inferred, not measured.**
   - report-draft.md:89-91 says the drift "is part 4's code in write_to, linked by every probe that writes a package". Size record:94-100 says the same, adding "cost the consumers that already wrote a package 26616 more".
   - The classifier measures only "code" (size.log:1882-1893). It does not locate write_to.
   - probe-emend-package-xml, which also writes a package, moved +44872, not 26616 (size.log:1898). So the generalisation is contradicted by the log's own figure.
   - "6 of 6 commits are part 4's" rests on commit subjects (size.log:1922-1924). Of those six, 18bfc36 and af5bba6 touch only tests and doc comments under crates/emend/src.

6. **(note) report-draft.md:47-49 overstates what scan.log prints.** It says each log prints the spec hash "at its start and end", listing scan.log. scan.log prints it once, at scan.log:1238.

7. **(note) report-draft.md:17 says "The shared read_back function does this."**
   - `read_back` serves `--edit-sample` only. The formula sample has its own `formula_read_back` (diff-combined:404,534).
   - The output removal is done by the callers, not by `read_back` (diff-18bfc36.diff:100-104).

8. **(note) Two report statements have no primary.**
   - report-draft.md:41, "I checked by grep before writing code": spec:722-730 records only that the implementing session asked.
   - report-draft.md:4, "unedited": gate.log:811 and 815 print the hashes, but no log compares them with the hashes planning sent.

9. **(note) The mutation controls are asymmetric.** F4 and F4x show that the `CellsAfter` removal matters for `--edit-sample` only (test.log:2450-2465). The same removal in `edit_formula_sample` is exercised only through F5's reopen failure (test.log:2477), and has no control like F4x.

10. **(note) scan.log fails only the check the background names.** Its single failed structural check is scan.log:1299 ("check answers went through all three greps as prose: no"), which is expected in the first run. All controls pass (scan.log:924-964), there are 0 hits and 0 errors (scan.log:1307-1310), and the file, message, log and record counts are complete (scan.log:1075, 1134, 1219, 1231, 1249, 1274, 1302). Nothing else in it fails.
