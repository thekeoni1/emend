## Independent check: M1 part 4 P3 (ed89a57..4b31e2a)

### 1. Does the diff do what the spec says for P3, and nothing else?

Yes, with two notes. Each commit read on its own diff:

- **20fd731 (doc).** Carries `ROADMAP.md` and `specs/0011` with P2's rulings, as P2 verification ruling 7 requires (spec:719). The only ROADMAP change is P2 marked DONE (diff-20fd731.diff:15-22).
- **18bfc36 (code 1).**
  - The one write-path site becomes a match whose `Ok` arm panics with `out.len()` (diff-18bfc36.diff:25-30). This is P3 ruling 1 (spec:740-744). The 19 read sites are untouched.
  - The new `Error::ReadOutput` has the message "cannot read the output file: {kind}" (diff-18bfc36.diff:42,50). The `read_back` function now makes the `CellsAfter` check, and a failure there removes the output (diff-18bfc36.diff:98-104,115-148). This is P2 verification ruling 3 (spec:694-697).
  - Nothing else changes.
- **858acf4 (code 2).** `--edit-formula-sample` matches "Harness runs" (spec:311-328) and rulings 4 to 6 (spec:757-770):
  - reads only Worksheet-kind sheets, in `<sheets>` order;
  - takes the first cell with a Normal formula and sets it to `Number(1)`;
  - writes through a temporary and a rename;
  - measures `chain_dropped` from both reads, asserts `cells_after` against the count before, and reports `read_back` as number 1 with no formula;
  - sets every key to 0 when no normal formula is found, and the doc says so (diff-858acf4.diff:581-705).

  There are four tests plus three usage shapes (diff-858acf4.diff:539-545, 1274-1487). The one extra is moving the line printing into `edit_line`, which the commit message discloses (commit-858acf4.txt:14-15).
- **af5bba6 (code 3).** Seven Limits items in the package module header and four in the cells module header (diff-af5bba6.diff:98-117, 199-230). They cover the consequences, the refusals, inline strings and no style on a new cell, and they agree with the spec (spec:111-146, 233-244).
- **4b31e2a (code 4).** The probe does what "Size" (spec:273-281) describes:
  - it reopens the input;
  - it sets a number, a text and a boolean and clears a fourth reference, all four references from the command line;
  - it runs `drop_calc_chain`, writes to a second path and prints `written N` read back through `Package::cells` (diff-4b31e2a.diff:135-157).
  - It also adds a Cargo `[[bin]]` entry.
- The combined diff touches exactly these 9 files (diff-combined:1-826; gate.log:811-820).

### 2. Do the logs contain every line the spec requires?

Yes, except finding 1.

- **Phase-commit controls (null commit and short sha) in every log:**
  - gate.log:7-8
  - test.log:14-15
  - size.log:7-8
  - negative.log:10-11
  - run.log:7-8
  - scan.log:6-7 (the null commit is shown by the script at scan.log:225-226)
- **A planted fault per comparison flag:**
  - on generated pairs, for prefix, suffix, rows, refs, xvalue and chain: run.log:1422-1433;
  - `accept_one` refuses each one: run.log:1437-1445;
  - an added cell planted on a real pair: run.log:1519-1522 and 1637-1640.
- **`<row>` counts in input and output:** 260616 in and 260616 out on each target (run.log:1492, 1610), plus per-workbook `rows_in` and `rows_out`.
- **Path control fixtures built at run time:** gate.log:445, run.log:940-942, scan.log:328-360.
- **"Size":**
  - nine size builds (size.log:1577-1598);
  - the eight earlier probes rebuilt at 13fe4a4, each reproducing part 3's sha256, then classified (size.log:1758-1899);
  - the budget figure, part 4's own cost and 145868 (size.log:1901-1937);
  - every figure in the size record appears in size.log.
- **Harness runs P3:**
  - both targets, 27 files, `lo` and `lo_in` ok (run.log:1473-1478);
  - formulas replaced 21, chains dropped 9 (run.log:1490-1491);
  - one cell changed per replaced formula and `files_identical` where none was (run.log:1494-1498);
  - the entry totals (record .md, "total" rows).
- **P3 paragraph and rulings 7 and 8:**
  - F4a/F5a: no output (negative.log:2763-2780);
  - F4b/F5b: exit 2, the exact message, empty stdout, and the outputs' presence and byte counts, with the source lines that explain them (negative.log:2758-2786);
  - F7 unreached, named from the source (negative.log:2802-2809);
  - P1's refusals again (negative.log:2659-2694).
- **P3 ruling 1 grep counts:** test.log:2315, 2336, 2356-2357.
- **Ruling 3:** every log prints the spec's sha256 at its start and end (gate.log:12,1052; test.log:11,2663; size.log:18,2173; negative.log:5,3308; run.log:17,1743; scan.log:1238).
- **scan.log:** besides the known "check answers ... as prose: no" (scan.log:1299), nothing else fails:
  - "structural checks failed: 1" (scan.log:1309);
  - total hits 0 and errors 0 (scan.log:1307-1308);
  - the shape, bare-shape and file-count checks all read yes (scan.log:1041, 1134, 1274, 1304-1305).

### 3. Does every number and quoted string in report-draft.md appear in a primary?

Most do: every log, record and size hash, and every count and figure. The exceptions are findings 3 to 9.

### Findings

1. **defect.** size.log runs a path check on the size record, with the path regex written out literally, and has no control fixture at all (size.log:1142-1143, 2166). It is the one P3 log with a path check but no planted path, which ruling 4 as the task states it requires in every P3 log. Compare run.log:940-942.
2. **note.** 18bfc36: when the reopened output fails to parse (not an IO failure), the error still reaches the user as the plain layer message, with nothing saying it was the output (diff-18bfc36.diff:124,127). F5 tests only the IO-open case (test.log:2474-2477). This is arguably inside ruling 3's wording; planning may want to rule on it.
3. **note.** report-draft.md:13-14 quotes the panic as "...it wrote {n} byte(s)". The source says `{} byte(s)` with `out.len()` (diff-18bfc36.diff:28). The quoted string is not verbatim.
4. **note.** report-draft.md:42 says "the three gaps". The spec says "two gaps ... and noted a third" (spec:754-755).
5. **note.** report-draft.md:41, "Recorded in my gotchas": no primary shows this.
6. **note.** report-draft.md:47-48 places 9bf44966 at "size.log attempt 1's start" and 68838d70 at "attempt 1's end". No committed primary prints attempt 1's hashes: size.log:10 says only that they differ. The paragraph also leaves out that the spec changed during test.log attempt 1 (test.log:4).
7. **note.** report-draft.md:69, "Both attempts passed every check with the same binary hashes": size.log:10 supports "passed every check" but not "the same binary hashes". The attempt 1 log is hashed (size.log:12) but was not given to this check.
8. **note.** report-draft.md:117 names attempt 1's directory as `<corpus>/runs/2026-09-22-4b31e2a`. run.log:11 says only "its own run directory (RUN_TAG -b)" for attempt 2, and never prints attempt 1's path.
9. **note.** report-draft.md:64 says "E3 puts cells_after off by one". The mutation changes the assertion to `before + 1` (test.log:2396), not `cells_after`.
10. **note.** report-draft.md:122-123, "my count, not a log line": the claim is supported by a primary. The record .txt has 24 flagged entry rows, 12 per label, every one `changed` with `descriptor` alone. It could cite the record.
11. **note.** report-draft.md:149 and 151 still hold the placeholders "SCAN" and "CHECKS".
12. **note.** test.log runs from 14:51:01 to 14:51:30, 29 s (test.log:3, 2664). In that time it ran two workspace test runs, four clippy runs, two size-probe builds, and a fresh worktree with its own `CARGO_TARGET_DIR` (test.log:540, 662) followed by ten mutation builds or test runs plus clean controls. No per-command durations are printed, so the log alone cannot show how that fit in 29 s. Planning may want to check this against the primaries.
