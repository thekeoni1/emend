## Independent check, M5 part 4 P3 (inputs under <phase>/check1/)

### 1. Does the diff do what the spec says for P3 and the rulings, and nothing else?

**YES WITH NOTES.**

- diff.patch touches three files only, which matches gate.log ("the phase's files: specs/0035-charts-write-first-set.md CHANGELOG.md tools/size-probe/src/bin/probe-emend-charts-write-xml.rs").
- **Doc commit.** spec-committed.md hashes to 01652f49..., as e87e1ba's message says. Its 68 added lines are the P2 rulings J to N and the P2 verification section, which is what P3's "The doc commit (the P2 verification rulings)" asks for.
- **Probe.** It adds one `add_chart` call:
  - one clustered column chart with one series, from in-source constants;
  - a two-cell anchor, a title, both axis titles and a legend at Bottom;
  - then `write_to` into a `Cursor<Vec<u8>>`, a reopen, and `charts`.

  That matches "Size and packaging" and ruling P. size.log confirms "its second output is the format probe's second output, byte for byte" and "prints exactly the expected thirty lines", ending `charts 1` and `chart_series 1`.
- **CHANGELOG.** One "Added" bullet, citing only `specs/0035-charts-write-first-set.md`.
- **Note.** The probe's carried doc comment still says "Every name, part, reference ... comes from the command line". The new series references and texts are now in-source constants. The spec allows that ("the probe's own constants"), but the comment is now partly inaccurate.

### 2. Do the logs contain every line the spec requires?

**YES WITH NOTES.**

- **Size (size.log):**
  - Feature cost: "5a. ... sections 1055368 - 970208 = 85160 ... above planning's estimate of 65000 to 79000 by 6160".
  - Writer cost: "5a'. ... sections 85160 - 44428 = 40732 ... above 40000 section bytes, a FINDING the report names: yes", and the FINDING line under ruling O.
  - Images: "moved by 384 ... expected 0, accepted up to 1500 either way ...: yes".
  - Core growth: "sections 970208 - 970208 = 0 ... above 2000 ... : no".
  - Every other probe: 28 lines "... expected 0 and accepted up to 2000 ...: yes". The images probe shows 384, all the others 0.
  - Budget: "21760 under the ceiling". The log ends "size log checks all as expected: yes".
- **API listing (test.log 4509):** "rows 734; {'keep': 734}; differing from M5 part 3 P3: 1 gone, 17 new; gate-widened 13, 1 of them pub". The gone and new rows are listed by name.
- **CHANGELOG check (run.log 5575-5576 and 5600):** all three lines are present.
- **Negative:**
  - N1 and N2 each print "exit 2, no stdout, the error line exactly the DRAWING_UNREADABLE message: yes" and "no output file: yes".
  - B0 and B1 are their exit-0 controls.
  - The four CHART_* messages are stated as unreachable, each with its covering test (lines 3735-3739).
  - **Note.** The spec's "Errors" section also names "a worksheet has two drawing elements" as a message raised at the write under either feature. negative.log neither shows it through the binary nor states it unreachable with a covering test. It appears only as a control helper inside the script (line 3351). Part 1's messages for sheets and parts added, replaced or removed in the same write are not addressed either.
- **Run:**
  - The size log's release binaries are copied and their hashes matched (4570-4583).
  - chartkinds and addchart are each "byte-identical to P2's run's, by sha256, 27 of 27" on host and i686.
  - charts reads "every row's output and line are part 3's charts run's, by sha256, 27 of 27" on both targets.
  - **Note.** The comparison is against part 3's run directory. It ties to records/m5p3-p3-charts-2026-10-13-d74ea74.txt (fe73d15b...) only through "the record's summary table is the run directory's: yes". The new .txt (5042c002...) is not byte-identical to part 3's .txt: cmp shows 142 differing bytes, in timing columns. That is acceptable if the spec means the outputs and lines, but it is indirect.
  - All four record pairs are installed (7144). The open record shows "pairs 1201; lo_in ok 1189 fail 12 error 0; lo ok 1190 fail 11 error 0 ... findings 0".
- **Gate:**
  - Six sets on both targets pass: 515, 578, 551, 616, 681 and 1055 on host and on i686.
  - Clippy and doc are clean for all six.
  - **Note.** The gate checks each figure against "the figure FACTS.md gives", which is the implementer's notes and not a primary. test.log checks the same figures against P2's committed test.log, so the figures are still covered.

### 3. Does every number and quote in report-draft.md appear in a primary?

**NO** (one wrong number). Everything else was found.

- **Wrong.** The diff-stat table gives the probe file "57" insertions. diff.patch has 48 insertions and 9 deletions for that file, and size.log 8255 says "1 file changed, 48 insertions(+), 9 deletions(-)". The report's own total of 120 is 68 + 48 + 4, so the table row conflicts with the total.
- **Verified:**
  - All ten record sha256s and all five log sha256s.
  - The working-spec hash 2dd19218..., in size.log line 5 and the scan input.
  - 85160, 40732, 6160, 6732, 68216/67832/384, 970208, 436992, 21760.
  - test.log: 734, 1 gone, 17 new, ImageAnchor, "1; ... 12", 1269 on both targets, 40/40 and 23/23.
  - "pub first lines: at the base 204; at the phase commit 204" and "counts left after the identifiers: none".
  - 162 of 162; created 17, appended to 10; 27 of 27; 1201 and the zeros; the open-check totals; CHANGELOG 33 and 33, uncited 0 missing 0.
  - fe73d15b...; the open .txt matching e86da06's 613cbf0b...; L58.
  - The scan pre-check's "total hits: 0", "structural checks failed: 0" and "RESULT: PASS".
- **Small paraphrases, not verbatim quotes:** "40 of 40" (the log says "40 of the 40"), and the B0/B1 "exit 0, ... an output" ellipsis. Both match the substance.
- **Claims the report itself says have no primary:**
  - nothing pushed (no log checks the remote);
  - each script exiting 0 (shown only in logged.out, outside the repository);
  - finding 2 (the 1055308 file-reopen figure, from FACTS.md);
  - finding 3's dry-run failure and the "fatal: ambiguous argument" stderr line (no committed log has them, though gate.log does show the L58 fix at 5287-5288);
  - finding 4's superseded generator outputs.

  For finding 3, the report's closing sentence ("No primary in the repository shows findings 2 to 4, except where size.log is cited (finding 1) and the superseded logs outside the repository (finding 3)") muddles the exception. Findings 2 to 4 have no primary in the repository; finding 1 is shown by size.log.
- **Other non-primary sources:**
  - "The crate's tests passed ... at the figure FACTS.md gives" cites the notes, but gate.log and test.log carry the figures.
  - The bare figures in finding 1 ("feature charts 85160", "65000 to 79000") are in size.log.

**Disagreements to fix before the report goes out:**
1. Change 57 to 48 in the diff-stat table.
2. Either address the "two drawing elements" message in negative.log, or have planning rule that it is out of P3's negative scope.
