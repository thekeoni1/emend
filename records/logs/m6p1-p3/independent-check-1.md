# M6 part 1 P3: independent check 1

A fresh subagent given only the spec, the phase diff, the draft report,
the five logs, the scan pre-check, the logs' README and the records, in
`<phase>/check1`. Its answer, verbatim:

---

# Independent check of M6 part 1 P3 (specs/0038): diff ef645e2..4cc4f2e, the five logs, the scan pre-check, the records and report-draft.md

All the hashes I could check match the files in the check directory: the five logs, the pre-check (29ee3bf7...), specs/0038 (f15be1a0...), and the seven 4cc4f2e record files plus P2's open .txt. The P3 open .txt has the same sha256 as P2's, ec23fef1...

## 1. Does the diff do what the spec says for P3 and nothing else?

**YES**, with two points the diff alone cannot settle and one spec item the report never mentions.

- **The diff touches four files.** The changed-line counts match report-draft.md lines 21-26: CHANGELOG +4/-0, ROADMAP +8/-1, spec +83/-0, probe +33/-5. The gate confirms there are no other files: no manifest, no lock file, nothing under crates/ or scripts/ (gate.log 7772-7799).
- **Doc commit (phase.diff 16-133).** It adds the P2 rulings and verification J to M, marks the P2 row of ROADMAP DONE and adds the "Unscoped" item. This matches spec line 366, "the doc commit (the P2 verification rulings)". No log checks that ROADMAP.md is the file planning left: PLANNING_ROADMAP is unset (gate.log 13). The spec is pinned at f15be1a (run.log 5433).
- **Probe commit (phase.diff 137-234).** It adds `add_chartsheet`, writes into memory and reopens, then reads `charts` on the new part and prints `chartsheet_charts`. That is what "Size and packaging" asks (spec 44-47). The size `(960, 640)` agrees with ruling C (9144000 / 9525 = 960 and 6096000 / 9525 = 640).
  - **Cannot be verified from the diff:** that `..chart` (phase.diff 222) is "the first set's column chart" (phase.diff 146) and not the chart for `kind`. `chart` is bound outside the hunks. size.log 12028-12051 shows only that the chart count is 1 at every kind index, not which chart was used.
- **CHANGELOG commit (phase.diff 9-12).** It adds one bullet that cites specs/0038, as spec 371-372 asks. Gate 7761 checks that it sits under Added.
- **"The carry" is never addressed.** Spec line 366 lists "the carry" as a P3 item. There is no carry commit, and neither report-draft.md nor ruling M says there is nothing to carry. The only explanation is in the test script's comment VA (test.log 249-250, "the carry was P2's"). The report should say so.

## 2. Do the logs contain every line the spec requires for P3?

**PARTLY.**

**Present:**
- **Probe and size record.** The charts probe prints 31 lines ending `chartsheet_charts 1` (size.log 12012-12051). The feature's whole cost is 119344, against part 6's 105916 (probe 1076124) and part 5's 105424 (size.log 9945, 9969-9970). The growth of 15396 is judged against the 6000-11000 estimate and the 15000 finding line (size.log 9946, 10044).
- **The core's growth** is 1968, judged against 2000 (size.log 10030; record 286-295).
- **API listing.** One row is new and none are gone; the three messages are crate-private; SheetKind keeps its row (test.log 3237-3292). The record is installed at c3c09f61 (test.log 3483).
- **Negative log.** N6-N8 are shown through the binary (negative.log 6211-6216). CHARTSHEET_ANCHOR and ONE_CHART are stated unreachable, with the reasoning and five planted controls (5517-5544, 5560).
- **Run log.** The corpus is rerun with the size log's binaries, which are verified by hash (run.log 5455-5467). It is 27 of 27 byte-identical on each target (5839-5840, 5869-5870). The open record and the keep check are there (6178-6197, 7937-7945).

**Missing or weak:**
- **Stale labels.** Ruling M (spec 127-128) requires the size generator's stale labels to be corrected. No log line asserts it. I count 0 occurrences in the size record, but the only evidence offered is the session's grep (report-draft.md 125-126).
- **images and comments bands.** The spec (65-67) applies the 1500 band to the `images` and `comments` *costs*. The size log judges the *probes'* growth instead: 1852 and 1904, which include the core's 1968 (size.log 10035-10036). The cost movements, -116 and -64, are printed (size.log 9947-9948) but never judged against the band. Both FINDING labels come from the quantity chosen, not from the costs.
- **copy.** The size record (279-282) calls copy's 1792 and the pair's 1932 "the growth specs/0038 'Size and packaging' expects, a finding the report states". specs/0038 "Size and packaging" (spec 40-70) says nothing about `copy`. size.log 10042-10043 judges both inside their 2000 band, and they are not among the log's 3 findings (size.log 10051). The label was relabelled from part 6's "specs/0037 'Size'" (size-m5p6 record 256-259) and is now wrong.
- **Fresh openpyxl fixture.** Ruling M asks for "byte-identical by sha256". The fresh fixture is compared by its lines and standard error only (run.log 6094-6098). The sha256 condition is met only by rerunning P2's kept fixture (6101-6130). This is disclosed (report finding 3).
- **N9.** N9 runs `--add-chart-sample` on B4, but the log has no accepted run of `--add-chart-sample` on B4 as its control. B4's control run is `--chartsheet-sample` (negative.log 6148-6198), and B1 runs on B0. N9 has only refusal-text controls (6263-6265).
- **Formatting.** Some output lines are indented four spaces (gate.log check.sh PASS lines; run.log 5764-5766, 7940-7944), which contradicts the logs' own convention. This is cosmetic.

## 3. Does every number and quoted string in report-draft.md appear in a primary?

**PARTLY.** Most figures check out: the commits, hashes, times, test totals, probe figures, binary hashes, corpus totals, keep counts and quoted messages. These claims do not:

- **Pre-check location (report 52-54).** The report says the pre-check is "kept in `<phase>/superseded/pre-check-1` and named by the final scan's RERUN_NOTE". No primary supports this. The final scan is not in the package, logs-README.md 35 says "Nothing is superseded", and the pre-check itself has RERUN_NOTE and RERUN_FILES unset (scan-pre-check.log 5-6). The sha256 is correct; the location and the naming are an unmarked plan.
- **copy as a size finding (report 148-151).** Finding 1 lists "the feature `copy` grew 1792 against an expected 0" among the findings. The size log judges it inside band and not a finding (size.log 10042, 10051). The only source is the size-record sentence flagged in question 2, which misattributes the expectation to specs/0038. The pair's 1932, which that record says "the report states", is missing from finding 1 and appears only at report 122-124.
- **Cause of the core's growth (report 155-156).** The report says "P1's chartsheet work in the core (56b2937) is the source of the shared 1968 is the record's git reading". The record names both 2a706dc and 56b2937 (record 293-295). size.log 9888 shows 2a706dc changed 0 code lines, which supports 56b2937 as the source, but the report's citation of the record is inexact.
- **Dry runs (report 173-174).** "One dry run of each log, all passing ... no script was regenerated after the dry runs" is not marked as the session's statement. The README heads that section as the session's statements (logs-README.md 29), and no log prints it.
- **API listing wording (report 77-78).** "The API listing restored from M5 part 6 P3's test log" is loose. The listing program is M3 part 7 P3's, checked by hash (test.log 2334-2337). The T1/T2/R2 blocks follow M5 part 6 P3's form (script comment VB), and the comparison is against M5 part 6 P3's listing (test.log 3230-3237).
- **PLANNING_* inputs (report 4-5).** "No log received PLANNING_SPEC or PLANNING_ROADMAP" is shown by gate.log 12-13, size.log 5 and run.log 5438. test.log and negative.log print neither input.
- **Placeholders.** README_SHA, ICHK1_SHA, SCAN_SHA, RECORD_COMMIT and INDEPENDENT_CHECKS (report 17, 39-41, 178) are still unfilled.

The marked items are properly marked: "Nothing pushed", outer standard error 0 bytes, the charts-cost attribution, and the open .txt reading. The last one is in fact supported by run.log 6186 and 7943-7944.
