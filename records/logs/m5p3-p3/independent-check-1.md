Independent check 1

Scope: I read only the files under <phase>/check1/: spec-committed.md, spec-working.md, phase.diff, report-draft.md, gate.log, test.log, negative.log, size.log, run.log, scan-pre.log and records/. I modified nothing. I recomputed the sha256 of every log and record file there, and I counted the diff's added and removed lines per file. Line numbers refer to those files.

## Question 1. Does the diff do what the spec's P3 section and rulings T, U, X, Z and AA say, and nothing else?

Verdict: YES, with one note on wording (1h).

Evidence, item by item against spec-working.md 440-463 (P3), 714-731 (T, U, X) and 744-762 (Z, AA):

a. **Doc commit (91c1cee).** It touches ROADMAP.md and specs/0034 only (gate.log, the "doc commit carries" lines in 5123-5172).
   - phase.diff 87-226: the P2 rulings M to Y, the P2 verification, the ruling-N addition to "Errors", and the P3 text naming the carry.
   - phase.diff 15-27: the P2 row set to DONE.
   - Both blobs match the hashes in the commit message and the brief's prefixes (gate.log 5123-5164). The lines it removes are planning's own (gate.log 5165-5171).

b. **Ruling T** (one test, a plain `c:style` inside the first Choice). Done: phase.diff 59-67. The test asserts the Choice's value and that the Fallback's is not read. Plant P15 catches it (test.log 4569).

c. **Ruling U** (header edit, no code). Done: phase.diff 28-43, which adds four `//!` lines and changes one, naming the three ruling-N tests. Those three tests exist: plants P11, P13 and P14 fail them (test.log 4318-4560).

d. **Ruling X** (INF, -INF, NaN and an exponent form as min or max). Done: phase.diff 69-84 covers all four spellings, on both min and max. Plant P16 catches it (test.log 4581).

e. **The probe.**
   - Manifest feature `charts = ["emend", "xml", "emend/charts"]` and a `[[bin]]` with `required-features = ["charts"]`: phase.diff 227-250. The gate shows these are the only added lines (gate.log 5302-5314).
   - The source is new: phase.diff 251-931.
   - Classification: it is the twenty-ninth probe in the size record's probes table, "compared with nothing" (records/size-m5p3-2026-10-13-d74ea74.md 72, 114). Its own sources include package/chart.rs (size.log 8214, 8218). Its feature set turns on no other feature (size.log 7506).
   - The source reads `charts` of the second output's first sheet and prints `charts` and `chart_series` (phase.diff 659-679).

f. **Size log, API listing, CHANGELOG, negative log, run log, open record.** All present (see Question 2).
   - The CHANGELOG gains exactly one bullet, citing specs/0034 only (phase.diff 1-14; gate.log 5329-5334; run.log, CHANGELOG section).

g. **Nothing else.**
   - The tree outside the phase's seven files equals the base's (gate.log 5286-5290).
   - No Cargo.lock, nothing under scripts/, package/tests.rs and README.md untouched, the Error::Package messages unchanged at 144 (gate.log 5294-5345).
   - Exactly two new test names, and no base test lost or edited (gate.log 5800-5830).
   - Every commit message is ASCII and ends with the Co-Authored-By line only (gate.log 5090-5121).

h. **Note on 1e (rulings Z and AA).** "Size and packaging" (spec 37-39) says "then `charts` of its first sheet". The probe reads the first sheet of its own second output, not of the generated input. Ruling AA (spec 754-762) speaks of "the generated size-log package's first sheet" and accepts `charts 0`. size.log 10837-10838 shows the second output is byte-for-byte the format probe's second output, so the two readings agree on the figure. I record this as a wording difference, not a departure.

## Question 2. Do the logs contain every line the spec requires for P3?

Verdict: YES for the five logs given. scan.log is not yet written; scan-pre.log is its pre-check.

Spec 344-374 and 440-463 require the following. Each is present:

- **RERUN_NOTE unset, at d74ea74, in every log.** gate.log 2-5, test.log 2-3, negative.log 2-3, size.log 2-6, run.log 2-7.
- **TMPDIR input and mktemp checks with controls.** gate.log 10, 45-62; test.log 10, 63-69; negative.log 7, 60-67, 3410-3426; size.log 15, 32-34; run.log 18-19.
- **The gate's line on counting.sh's own mktemp.** gate.log 5043-5044.
- **The `-B` check.** gate.log 5375-5400.
- **Gate, as part 2 P1's:**
  - five feature sets on both targets, 515/555/591/658/1030 (gate.log 6175-6199);
  - clippy and doc clean (gate.log 5900-6060 region, all as-expected);
  - pub fn/const lines at the base and the phase commit (gate.log 5514);
  - nine PASS lines from check.sh;
  - "no prune this phase" (gate.log 11, 64).
- **test.log on both targets with the five sets.**
  - Section F: 2894-2958.
  - Section E: 2959-3040, each line P2's figure plus 2, chart tests 40 of 40.
  - Workspace: 1239 on both targets (2848-2892, 3036-3040).
  - API listing and record: T1, T2 and R2 at 3155-4316.
  - Plants P1 to P16: 4318-4605.
- **Negative log, every message of the part reachable through the binary.**
  - N1 to N7, each refused with the exact message and no output file (negative.log 3921-3944).
  - B0 accepted as the control.
  - "unreachable through the binary: none" (3529).
  - The covering test of each case located (negative.log, the "tests that cover the cases" section).
- **Size log.**
  - Charts cost printed and judged: size.log 8792, 8819.
  - Images cost's movement printed and judged: 8793, 8812.
  - Core growth printed and judged: 8782-8784.
  - The 28 compared probes: 8692.
  - Charts probe output and the `charts 1` control: 10726-10841.
  - Release binaries hashed: 10846-10880.
  - Record installed: 10893-11185.
- **Run log.**
  - Release binaries copied from size.log's hashes: run.log 4251-4274.
  - `--charts-sample` on both targets, 27 of 27 equal to P2's run by sha256: 5499-5510.
  - Planning's 22 figures met: 5408-5432.
  - `--kinds-sample` 27 of 27 equal to part 2's run, both targets: 5703-5717.
  - The record pairs: 5566-5602, 5770-5802.
  - CHANGELOG citation and form checks: 5804 onward.
  - The keep: 1103 pairs, 0 new, 0 changed, 0 gone (6111-6170).
  - The open check and its verdicts: 7462-7502.
- **Every log ends "... checks all as expected: yes".** gate.log 6291, test.log 4624, negative.log 3975, size.log 11203, run.log 7530.

Observations (no missing line):

- size.log 4 prints `input: SPEC_SHA=unset`, while line 98 calls the pin "the input SPEC_SHA". The script comments (size.log 454) say the generator gives SPEC_SHA a default, so the value is right; only the label is loose.
- The spec's "a dry run before each logged run" leaves no trace in the logs given. That is expected, but it cannot be verified from these files.

## Question 3. Does every number and quoted string in report-draft.md appear in a primary?

Verdict: NO. One statement misstates its primary, and several statements have no primary (most are already disclosed as the implementer's own). Everything else checks.

**Misstated:**

1. **report-draft.md 10-11:** "My driver passed that hash to the gate and size scripts as PLANNING_SPEC; the logs do not print the variable's name."
   - size.log prints the variable's name: line 5 (`input: PLANNING_SPEC=d86a043f...`) and lines 101-103 ("PLANNING_SPEC is set: ...", "... is at PLANNING_SPEC, planning's copy (change OD): yes").
   - gate.log, test.log, negative.log and run.log do not print it (grep count 0).
   - So the sentence is true of the gate log and false of the size log.
   - No primary shows that the gate script received PLANNING_SPEC: gate.log has no input line for it.

**No primary (stated as the implementer's own or unlogged):**

2. **Line 3:** "Nothing pushed." An assertion with no log line.
3. **Lines 127-133 (finding 1).**
   - The full 40-digit hashes of 07e3b7b, ff86a0f and e41e547 appear nowhere. Their 7-digit forms appear in spec-working.md 740.
   - The gate dry run's `cargo fmt --check` failure and "tree diff: tests/chart.rs, 4 insertions, 1 deletion; messages unchanged" have no primary. The report says so ("No primary in the repository shows this").
4. **Lines 134-137 (finding 2).** The failed negative dry run, the shape check flagging `[ ... ] && [ ... ]`, the fix to `[ ... -a ... ]`, and the kept files under <phase>/superseded have no primary in these inputs. The report calls the dry run unlogged.
5. **Line 23:** "`git diff --stat 91c1cee d74ea74`: 5 files, 723 insertions, 1 deletion", with per-file 3, 5, 32, 9, 675. No log prints this stat.
   - I recomputed it from phase.diff and it is correct: CHANGELOG +3; package.rs +4/-1 (5 changed lines); chart.rs +32; Cargo.toml +9; probe +675.
   - The same holds for line 26, "2 files, 114 insertions, 3 deletions": spec +113/-2, ROADMAP +1/-1. gate.log 5165-5171 confirms the 3 removed lines.

**Wording that could mislead (minor):**

6. **Lines 74-75:** "charts,images 591, charts,read 658, all 1030 (P2's figures plus 2); charts 555".
   - The parenthesis attaches to three sets, but charts 555 is also P2's 553 plus 2 (test.log 2996).
   - The features-off figure, 515 on both targets, is not stated (test.log 2993-2995 region; gate.log 6187).
   - Nothing is wrong; the grouping just reads as if charts differed.

**Checked and found in a primary:**

- **Commits.** Doc, carry, probe and CHANGELOG commits with their full hashes and subjects, and the base subject (gate.log 104-118, 5047-5065).
- **Spec and ROADMAP hashes.**
  - specs/0034 e27803e8... and ROADMAP 96af6582... (gate.log 81-85, 5130-5164).
  - Working copy d86a043f... (gate.log 83; run.log, scripts section; size.log 98).
  - The quoted gate wording "the working tree's copy, planning's ..., not the pin" matches gate.log 83 with an ellipsis.
- **Carry test names and ruling-N test names.** phase.diff 37-41, 61, 81; test.log 4318-4360.
- **Probe feature and required-features.** phase.diff 236, 249.
- **Rulings Z and AA.**
  - 44428, 20000 to 35000, 9428, 45000: size.log 8792, 8819.
  - charts 0 and chart_series 0: size.log 10755-10756, 10802.
- **Log sha256s.** All five match my recomputation:
  - gate fbc7bf97...
  - test c4e4d60f...
  - negative df2544b3...
  - size 6de64352...
  - run 0953b015...
  Each "checks all as expected: yes" line is present.
- **Scan pre-check.** "total hits 0", "structural checks failed 0", "RESULT: PASS": scan-pre.log 4020, 4022, 4024.
- **Test figures.**
  - 591, 658, 1030, 555, 1239, 1237: test.log 2996-3040.
  - "40 of 40" and "3 of 3": test.log 4374.
  - P1 to P16 each "found once ... the failed tests are exactly the table's": test.log 4380-4581.
  - P15 and P16 descriptions: test.log 4335-4336.
  - "new, for T and X": consistent with P2's 14 plants (phase.diff 24).
- **Negative.**
  - N1 to N7 messages, "exit 2, no stdout, the error line exactly the ... message: yes", "no output file: yes": negative.log 3922-3935.
  - "unreachable through the binary: none": 3529.
  - B0 accepted: 3921.
- **Size.**
  - 1014636, 970208, 44428, 45056, 40320, 9428: size.log 8792.
  - 67832 now and in the record: 8793.
  - Core growth 0: 8783.
  - 2324: 8787, 8821.
  - "28 keep every figure": 8692.
  - The `charts 1` control refused: 10841.
- **API.**
  - 718, 4 gone, 62 new, 656 equal: test.log 4019.
  - 58 = 62 - 4, with 4 widened rows: test.log 4078-4093.
  - 13 widened, 4 pub, 9 crate-private, DRAWINGML among them: test.log 4111-4129.
  - 156 documented items: test.log 4142.
- **Run.**
  - Charts 27 of 27 on host and i686 against P2's run: run.log 5503-5509.
  - Kinds 27 of 27 against part 2's run: run.log 5712-5716.
  - CHANGELOG ASCII, no U+2014, no control byte, at most 80 columns: run.log, the CHANGELOG form section.
  - Keep against records/m5p3-open-2026-10-13-27569d2.txt: 1103 pairs, 0 new, 0 changed, 0 gone (run.log 6111-6125).
  - "no verdict or class moved": run.log 7462-7466.
- **Record sha256s.** All eight match my recomputation of records/: size c5e81bd9..., api 45261724..., charts .md 593ffa21... and .txt fe73d15b..., kinds .md 0a2edfb5... and .txt 406f5298..., open .md d5ee1a49... and .txt edc5fda6...
- **"The open .txt has the same sha256 as P2's."** P2's record is edc5fda6... (run.log 6113), the same as the new one (run.log 7499).

Placeholders still to fill (RECORD_COMMIT, SCAN_SHA, SCAN_RESULT, INDEPENDENT_CHECK) are left unfilled as intended.

Unrelated observation, not a defect of this phase: run.log prints smooth 0 over the corpus, while the spec's facts (spec 72) count 8 `smooth` elements in 4 workbooks. No P3 figure depends on it, and planning verified the P1 and P2 smooth field. I note it only for planning.
