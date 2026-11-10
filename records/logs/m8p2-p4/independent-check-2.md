# M8 part 2 P4: independent check 2 (the report)

The answer of a fresh subagent given only these materials, under
<scratch>/check2:
- the spec as committed at 3bbac34 (e3aaf65402a1b8b22f06e4f28c5011ee033e5b0a0b9ac74babddc6109c8f85f8) and as working with ruling N (9a6e0db513d352beb46957a9212cf703492aec70a2e13c4c3f64b7a6dba4df31);
- the two commits of the range, 3bbac34 and f625b99 (git show);
- gate.log and run.log; the pre-check scan's log; the driver's output, <phase>/logged/driver.out;
- the reproduction's bytes.txt, run.stderr and the two workbooks' sha256; dry run d1's run.log;
- the logs README as it stood before the checks (sha256 e7928b72c16966adac3d8ba3d522fd7913acb9a0c2cab75375f2262bab703fa7);
- the draft report (<phase>/report-final-draft.md, sha256 e25aa9dd81ce0af388688834d605def4eca257503bc7ea8da6e0bb7752d4b156).

The answer was extracted from the subagent's transcript by script into
<phase>/check2-answer.txt (sha256 643cb7f3e15f2e2b8ef159e375f18d9a5dbde4b8399a31fc1844402d2aed2db8). It appears below verbatim, except
that the absolute paths it names are written with the roots declared
here. The README's section "After independent checks" answers each
exception.

root: <scratch> stands for <phase>, as in gate.log and run.log
root: <phase> stands for <home>/.cache/emend-build/m8p2-p4, the phase's directory
root: <home> stands for the home directory of the account the runs ran under
root: <check> stands for <scratch>/check2, the materials directory the subagent was given
root: <session-scratch> stands for the implementing session's own scratch directory, outside the repository

# Independent check 2, M8 part 2 P4

I used only the materials in `<check>`. I modified nothing and ran no git or cargo. Hashes I recomputed: spec-0048-committed.md is e3aaf654..., spec-0048-working.md is 9a6e0db5..., gate.log a27d8ee7..., run.log 4f148de2..., scan-pre-check.log 582bb6d1... and dry-d1-run.log 3ea115cf.... Each matches the value the logs, the README and the draft report give for it.

## 1. Does the diff do what the spec says for P4, and nothing else?

**Yes, with one observation and one claim I cannot check.**

**The doc commit, 3bbac34** (commit-3bbac34.diff)
- It touches only ROADMAP.md and specs/0048 (diff lines 12 and 34).
- The message names each file as "<path> sha256 <hash>" and has no count sentence (lines 7-8), which is the form "Phases" asks for.
- The spec blob e3aaf654 equals spec-0048-committed.md. gate.log lines 267-268 confirm it is the doc commit's blob.
- The working spec differs from the committed one only by the added "Planning rulings during P4" block, ruling N (working lines 880-914).
- The spec changes carry ruling H into "The documents" and "P4", record P3 as DONE, and add rulings E to M, P3's verification and decision 4.
- ROADMAP gets the Amendment 1 heading and the P3 row marked DONE.
- **Observation:** the P4 row stays "PLANNED" (diff line 31). The earlier doc commits set the row to "IN PROGRESS"; the old P3 row read that way (line 30). The spec does not require it, but this departs from the pattern.

**The phase commit, f625b99** (commit-f625b99.diff)
- It touches only CHANGELOG.md, README.md and crates/emend/src/package.rs (lines 24, 41, 59).
- **README:** the `rowcol` line now reads 76872 and the `comments` line 66388, each citing `records/size-m8p2-2026-11-10-8065795.md` (lines 50 and 55). These are the record's figures that ruling N names (working line 885). They also agree with ruling K's growths: 58244 + 18628 = 76872 and 63440 + 2948 = 66388. This satisfies ruling H.
- **CHANGELOG:** the "Changed" bullet for `remove_sheet` gains the `PIVOT_CACHE_DEFINITION` clause (lines 34-37), as ruling K (4) asks. The 0.1.0 entry is untouched.
- **package.rs:** only module doc comment lines (`//!`) change (lines 70-75), as ruling M (1) asks. No compiled code changes.
- The message is ASCII, has no U+2014, ends with the Co-Authored-By line alone, and carries "the diff alone carries what it changes".
- **Cannot check from these materials:** the new doc sentence says `TABLE` is public with `rowcol`, `tables`, `read`, or `sheetprops` and `metadata` together. That depends on the source's `cfg` attributes, which I do not have. The gate's clippy and doc sets all exit 0 with no warnings, including tables alone and metadata alone (gate.log lines 10732 onward). That rules out a broken doc link, but not a wrong list.

## 2. Do the logs contain every line the spec requires for P4?

**Yes. I found no required line missing, with the limits noted below.**

**The gate and its prune** (gate.log)
- Section header at line 181.
- The control on a target-shaped directory: lines 183-187.
- "PRUNE_PREVIOUS: yes": line 188.
- The 36 kept names: line 191, listed at lines 194-230.
- The target's size, 9018112251 bytes: lines 239 and 241.
- The `rm -r "${PREV_DIR:?}/${n:?}"` command: line 242, with "target: rm -r exit 0" at line 243.
- The checks after the removal: lines 244-248.
- The gate ends "gate log checks all as expected: yes" at line 12078.
- The working spec is pinned at 9a6e0db5: lines 269-271 and 12060.

**The test figures** (gate.log)
- Expected figures: line 11060.
- Host passed figures: lines 11066, 11093, 11120, 11147, 11174, 11201 and 11228.
- i686 passed figures: lines 11255, 11282, 11309, 11336, 11363, 11390 and 11417.
- Every set shows 0 failed.

**The run over the 39 with every output opened** (run.log)
- 59 runner commands. Each one passes `--libreoffice localhost/emend-lo:25.2.3-2-deb13u6 --lo-timeout 600`, and I counted all 59 matching.
- 59 lines "names the settings given (timeout 300, lo timeout 600): yes".
- 59 "files 39" tallies.
- "runs done 59 of 59": line 7200.
- **Limit:** run.log does not print each output's lo value per row. The rows show outcome, identical count, time and memory (for example lines 3359-3397). Whether each output opened is shown only through the outcome/lo/lo_in comparison, and the records that hold the lo values are not among these materials.

**The per-id comparison with part 1 P2's records** (run.log)
- 59 lines "compared with records/m8p1-...-2026-11-09-8816db0.txt per id".
- The six changed pairs read "same 38; changed 1": lines 3872, 4330, 4461, 5047, 5243 and 5894.
- Copy reads "same 39": line 6090.
- Every other summary line reads "same 39, changed 0, missing 0", with new 0 everywhere (driver.out lines 14-72).

**The expected-change table** (run.log)
- The table: lines 3317-3328, with its then-triples checked against part 1 P2's records at lines 3340-3347.
- The remove pair's reasons: line 3874 reads "- 26 last 12 pivot_source 1".
- The totals: line 7456, judged "yes" at line 7457.

**The records installed** (run.log)
- Section 5 starts at line 7266, "118; expected 118".
- 118 "installed" lines whose bytes I summed to 12409762.
- "every record file was installed, byte for byte: yes": line 7393.

**FINDING lines**
- run.log: "FINDING lines printed above: 0" at line 7464.
- gate.log: only lines 5723 and 5732, both "#" comment lines inside the printed script.
- No line ends with UNEXPECTED in gate.log or run.log (my grep found 0 in each; driver.out lines 76-78 agree).
- The scan's own log: structural checks failed 0 (line 8497), "RESULT: PASS" (line 8499).

**Two notes**
- driver.out's "FINDING lines (outside the printed scripts)" list (lines 13-75) is a loose grep. It catches 61 summary and total lines that contain the words "FINDING lines" but are not FINDING lines. The README and the draft report correctly say none, but a reader of driver.out could be misled.
- The dry run's 8 FINDING lines (dry-d1-run.log lines 6665-6690) are given in the README only as a count and a description. Their ids and line numbers are not listed. If the "list every FINDING line" rule (`specs/0046` ruling R) applies to the dry run in the README, this is a gap.

## 3. Does every number and quoted string appear in a primary?

### Disagreements

1. **README lines 124-125, "its ids differ from the 39": contradicted by the dry log.**
   - The dry corpus's 9 ids are all among the 39. dry-d1-run.log line 6698 reads "earlier 39 (same 1, changed 8, missing 30); new 0", and the dry rows at lines 6631-6639 use ids that also appear in the real run's passthrough (run.log lines 3359-3397).
   - The README's own rule (lines 24-25) says an id is the first 12 hex digits of the sha256. The reproduction's input, the "dry file 453a166dcdfe", hashes to fceb4ba3... (residue/sha256.txt). Its label is therefore not its sha256 prefix, and the same label names a real workbook.
   - This affects README line 128 and draft-report line 54 ("the dry file 453a166dcdfe"), and README line 148 (15205770ae5c "whose dry worksheet root..."). The label is ambiguous between a dry file and a real workbook.
   - Ruling N (2)'s actual reason is that "the 39 real workbooks declare the prefix", not that the ids differ.

2. **README lines 3-5, "ruling N given by planning's message ... (planning writes it into the spec after this record commit)": contradicted.**
   - Ruling N is already in the working spec (spec-0048-working.md line 882).
   - That file hashes to 9a6e0db5, the PLANNING_SPEC both logs pin and accept (gate.log lines 269-271; run.log lines 3141-3145).

3. **README line 56, "gate.log lines 181 to 246": the prune section runs to line 248.**
   - Line 248 is "control: cleaned_one accepts...", and line 250 starts the next section.

4. **README lines 62-63, "lines 11066 to 11228" (host) and "11255 to 11417" (i686): these are the first and last set's passed-figure lines.**
   - The "all" set's sub-counts continue to lines 11249 and 11438.
   - This is accurate for the passed figures, but the ranges are narrower than the sets' blocks.

5. **Draft report line 72, "outer stderr 0 bytes" for the pre-check scan: no primary shows an outer stderr for the scan.**
   - scan-pre-check.log line 8491 shows the scan's own stderr file at 0 bytes.
   - driver.out covers only the gate and the run.

### Confirmed against a primary
- **Working spec:** hash 9a6e0db5 and 1015 lines.
- **Commits and subjects:** 3bbac34, f625b99 and base 053d27c (gate.log lines 263 and 300). The record commit's subject matches ruling N (working line 911).
- **Driver timestamps:** 738G, 19:11:12Z, 19:23:18Z and 23:47:31Z.
- **Stderr:** outer and own stderr 0 bytes for the gate and the run.
- **Prune:** "gone; its names: 36", and 9018112251 bytes.
- **Test figures:** 544, 719, 581, 761, 768, 826 and 1244.
- **Log files:** gate.log and run.log sha256, bytes 924768 and 522611, lines 12078 and 7599, and their last lines.
- **Root mount propagation "shared":** gate.log line 19, which is in the gate, not before the driver.
- **Cited line numbers:** every run.log number the README cites (66, 79, 3317, 3328, 3869, 3872, 3874, 3877, 3942, 4330, 4333, 4461, 4464, 5047, 5050, 5243, 5246, 5894, 5897, 6090, 6093, 6741, 6743, 7200, 7266, 7393, 7396-7455, 7456, 7457, 7464 and 7598) and gate.log lines 191, 241, 243, 359, 5723, 5732, 10732, 11060 and 12077, each holding the quoted text.
- **Quoted lines:** the 7456 quote is exact in the README and the draft report.
- **Records:** 118 files and 12409762 bytes; "files under records/ matching m8p2-: 118" (driver.out line 79).
- **Dry run:** 8 FINDING lines (dry line 7411), all in remove-chart, on 8 of 9 files, each ok/ok/ok then error/-/ok; 15205770ae5c ok (dry line 6635).
- **Reproduction:** 6 entries in and 7 out, 5 of 6 identical, 262 to 340 bytes, xmlns:r False then True, no `<drawing>`, a 100-byte .rels, the error message, and both workbook hashes (residue/).
- **Driver:** logged.sh sha 658abb65 and "<phase>/tmp names unchanged: no".
- **Pre-check scan:** "RESULT: PASS" and structural checks failed 0.

### Cannot be checked from these materials
- **The reproduction's details:**
  - "Exit 2": run.stderr holds only the message.
  - "0 Relationship elements" / "no Relationship element": bytes.txt gives only the 100 bytes.
  - The command line and the use of `<work>/bin/emend-copy`.
  - That 15205770ae5c's dry worksheet root declares `xmlns:r`.
- **The run directory:** the name `tmp.ctmjlg2mGu` and "as in P3", and the phase-tmp-before/after listings.
- **The driver's scripts:** that logged.sh checked the three scripts cmp-equal with dry d1. driver.out prints only their hashes.
- **The tree state "before":** the only status printed is during the gate (gate.log lines 12057-12059), with specs/0048 modified and the new logs directory.
- **Session conduct:** that the working spec was "not edited by this session".
- **The leftover directories** the README lists.
- **package.rs:** the correctness of the `TABLE` feature list.
- **ROADMAP.md:** its hash a822f105. gate.log line 275 matches the commit message, but I had no copy of the file to hash.
- **The final scan:** it does not exist yet.
