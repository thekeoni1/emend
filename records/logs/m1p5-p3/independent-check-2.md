# Independent check 2: M1 part 5 P3

Written by the implementing session. The header is mine; everything after
the rule is the checker's answer, byte for byte as it was returned.

The checker was a fresh subagent (not a fork) with no access to this
conversation. It was given the twenty-one files below and nothing else, and
asked the three questions CLAUDE.md names, and for each of check 1's
findings whether the reruns and the revised report answer it. It was told
to write <repo>, <scratch>, <home> and <corpus> in place of absolute paths
and to answer in ASCII. Its inputs are check 1's with run.log attempt 2 and
its record pair, the third scan run for the checkers, check 1's answer and
the revised report. This is the last check of the phase (specs/0001: at
most two).

Commit checked: f565868c0836d7e3e6538051e4bd5b99c106e3bc
("M1 part 5 P3: probe-emend-styles-write-xml and probe-emend-dates-write-xml"),
which was HEAD when the check ran and when this file was written. Range
read: 747a96e..f565868.

The inputs, each as the checker received it (sha256, then the name it had in
the check directory and what it is):

- 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
  0001-bootstrap.md (specs/0001-bootstrap.md at f565868)
- dafccc6709a74a1e59da71b86dad9f04144f10b1c84dd9e03c558c8c4a5db554
  0012-styles.md (specs/0012-styles.md at f565868, with "Planning rulings during P3")
- c6312fa953731e5b69e562224a5f5f962734b459e73f3eed282348d8e23c8aed
  check-1-answer.md (check 1's answer as returned; the body of independent-check-1.md)
- ef5e163ece79508528b25339f5110f30083796acee6da1d3f90ac039f4292b4a
  diff-code-1.patch (git show 805526d, C1)
- cb151b12c29070a386ca254b08f9da60c53334a8d85b1ca8bd8d2a46797efa84
  diff-code-2.patch (git show c08bf76, C2)
- 4c2b887ff0385bf364046b2cdb8d57a65c77284c783ab99d3aac9b003542f6e8
  diff-code-3.patch (git show b2bf6e3, C3)
- 8b78ad4052e367c7437db6b39adafcc840c59f48f258247151ea1f1c0f527a4f
  diff-code-4.patch (git show 478da97, C4)
- 951922a8bb2a61cbcad42e540430001b32e9ddbde5f40fb239a08d626ca842ea
  diff-code-5.patch (git show f565868, C5)
- bc7dd3c571653fecb04bc9b6173307f74dab29535054c58bb3b1a28f959a7dc5
  diff-doc-1.patch (git show 6256267, doc 1)
- ac69677e76bde049ae8e855a4a397fe8f9b7f522384b3d6cf2c195e9fa1d6d55
  diff-doc-2.patch (git show 130f7ff, doc 2)
- fcc9a755bb8611cef0758f0902af8904dc6d180c928d0f866f2725b389985fba
  diff-phase.patch (git diff 747a96e f565868, the whole phase)
- c3b1fb2c5a17eff20c0062909e8b3babb9b87f5dfe631660df31f1d5a2dc52f4
  gate.log (records/logs/m1p5-p3/gate.log, attempt 2, the committed one)
- 15fd3c360b1b712d28482fc4acad618507ea9bc6052f5a6ea4f476a376c968bc
  m1p5-p3-2026-09-23-f565868.md (records/m1p5-p3-2026-09-23-f565868.md, the committed one)
- 663224cf6743aa3d4c661a35165e4a63bc1fe09908cd1b813671d963bb0f37ac
  m1p5-p3-2026-09-23-f565868.txt (records/m1p5-p3-2026-09-23-f565868.txt, the committed one)
- ddbd418538fb4e50afca98e991e098f323c3a57fae7aeef4f03e0a4631db0778
  negative.log (records/logs/m1p5-p3/negative.log, the committed one)
- 783ce6b21dc065614a159853aa035f9df9d53bab1a79d270d53969689e3c8195
  report-revised.md (the report as revised after check 1; the report sent to planning is later and answers this check's findings)
- c6178b56c9fe9843da5b341f325e8a6d7e13034a031d985c2198ddb7b15d2404
  run.log (records/logs/m1p5-p3/run.log, attempt 2, the committed one)
- c440f66dbffb69141bb9dbf9929f7c0334892b67236b07b68177d505ae8e1f28
  scan-for-checkers.log (the third scan run for the checkers; superseded, hashed in the committed scan.log's rerun header)
- 3a270d092928321d4e7e0dc4cfa6ec91698e6d51305a351c6d05a5eeeef09159
  size-m1p5-2026-09-22-f565868.md (records/size-m1p5-2026-09-22-f565868.md, the committed one)
- 270faa28d564b6ff0c4b25f81e7673777a1fa47de105846f56c849fc310abf3f
  size.log (records/logs/m1p5-p3/size.log, the committed one)
- aaea9f937a7437d6818d2ffaeae5908bea7b10bd43ab0096c6c4fa2b5121b3f4
  test.log (records/logs/m1p5-p3/test.log, attempt 3, the committed one)

The answer as returned: 20266 bytes, sha256
ea6ee57ca5c47a96d1a4bda761ae2de5c7034363ad0d4abe81f482d80a305f30.

---

# Independent check 2, M1 part 5 P3

Inputs read: every file in the check2 directory (0001-bootstrap.md,
0012-styles.md, check-1-answer.md, diff-code-1..5, diff-doc-1..2,
diff-phase.patch, gate.log, test.log, size.log, negative.log, run.log,
scan-for-checkers.log, the size record, the run record pair and
report-revised.md). The harness also put the repository's CLAUDE.md and a
memory index into my context; I have not used either as evidence.

I hashed every input with sha256sum. Every hash report-revised.md gives
matches the file I was given: gate.log c3b1fb2c..., test.log aaea9f93...,
size.log 270faa28..., negative.log ddbd4185..., run.log c6178b56...,
scan-for-checkers.log c440f66d..., the size record 3a270d09..., the run
record .md 15fd3c36... and .txt 663224cf.... 0012-styles.md hashes to
dafccc67..., which is the value gate.log line 862 prints for
specs/0012-styles.md at the phase commit and the value doc commit 130f7ff's
message gives.

Summary: 1 yes; 2 yes (with one log still to come and two items I cannot
check); 3 no (one number and one claim in the report have no source among
my inputs; the rest check out).

---

## 1. Does the diff do what the spec says for P3, and nothing else?

yes.

How I read the diffs. For each commit's "nothing else" I read its own
diff (diff-code-1..5, diff-doc-1..2). For the phase as a whole I read
diff-phase.patch. I extracted every +/- line from the phase diff and from
the seven per-commit diffs together, sorted both sets and compared them:
they are identical (+1963/-12 by my count, which includes the two doc
commits; check 1 gave +1844/-12, which I could not reproduce, but the
sets match either way). The phase diff covers 14 files, and gate.log line
869 says "files in the diff: 14".

- Order and split (ruling 1). gate.log lines 820-835 list the commits in
  this order: doc 6256267, doc 130f7ff, code 805526d, c08bf76, b2bf6e3,
  478da97, f565868. Its classifier gives "2 planning-doc, 5 code". So the
  second doc commit comes before the first code commit, as ruling 1 and
  the rulings' preamble ask.
- C1 (diff-code-1). A single comment line in null_rewrite.rs changes from
  "the first addition refuses" to "the styles read refuses". That is P2
  verification ruling 6 and nothing more. The other P2 verification
  rulings bind the logs, not the code. Ruling 3 (per-flag controls) is
  met: run.log prints "flags turned by some pair: 12 of 12". Ruling 4
  (`#!/usr/bin/env bash` and a printed invocation) is met: every log has
  one of each and none has `#!/bin/sh`. Ruling 1 (the tokenizer statement)
  is covered under Q2.
- C2 (diff-code-2).
  - Cargo.toml adds `[features] dates = []` with no dependency.
  - dates.rs adds Epoch {Y1900, Y1904} with Epoch::of, DateTime with the
    seven fields, serial and from_serial.
  - serial refuses month 0 or 13, day 0, a day past the month (which
    covers 1900-02-29), hour > 23, minute > 59, second > 59 and
    millisecond > 999 (ruling 3). It also refuses a day before the
    epoch's first day or at or past the day after 9999-12-31.
  - from_serial checks finiteness first, then computes
    `(serial * DAY_MS as f64).round()` and applies the first-day, end and
    day-60 refusals to the rounded value (ruling 2).
  - The 14 tests cover every item the "P3" paragraph lists, plus ruling
    3's four time refusals and ruling 2's three boundaries from both
    sides.
  - lib.rs gates the module and the `DateTime`/`Epoch` re-export on the
    feature.
  - Extras beyond the spec's derive lists: `Eq` on both types and `Debug`
    on Epoch. Both are harmless.
  - I checked the day arithmetic by hand against the pinned values:
    - 1900 system: count = civil - days_from_civil(1899,12,31), plus 1
      when count >= 60.
    - 1904 system: civil - days_from_civil(1904,1,1).
    - END_1900 = 2958466 and END_1904 = END_1900 - 1462 = 2957004.
    - This matches "Module `dates`".
- C3 (diff-code-3).
  - emend-copy enables `emend/dates`.
  - --date-sample reads the first Worksheet once for M and calls
    add_cell_style(None, BuiltIn(22)). It sets (M + 1, 1) to
    Number(SAMPLE_DATE.serial(Epoch::of(workbook))) with that style, then
    writes and reopens.
  - It asserts xfs_after = X + 1 and cells_after = cells_before + 1 and
    removes the output on failure.
  - read_back requires number() == serial, style == Some(dated), and
    from_serial == SAMPLE_DATE, as "Harness runs" P3 says.
  - It prints the eight keys in ruling 5's order: date1904, serial,
    added, xfs_before, xfs_after, cells_before, cells_after, read_back.
    The serial is the tool's constant.
  - It adds four integration tests and three flag-combination cases.
    The styled_run, styled_run_refused and print_line refactors serve the
    new flag, and the commit message names them.
  - No reference, code or date is taken from the command line.
- C4 (diff-code-4). 57 added lines, all `//!`, in dates.rs (Errors and
  Limits), package.rs (Limits) and styles.rs (Limits). They cover number
  formats only, the refusals, the Some/None write rule and the feature
  being outside the core. There is no code change: test.log line 2987
  reads "changed lines in C4: 57; of them //! lines: 57" and has a
  refusing control at line 2989.
- C5 (diff-code-5).
  - size-probe gains `dates = ["emend", "xml", "emend/dates"]` (ruling 7)
    and two [[bin]] entries.
  - I diffed the two new probe sources against each other. They differ
    only in these places: the usage line and doc comments; the seven
    date arguments parsed with str::parse; Epoch::of plus serial
    replacing the constant Number(2.5); from_serial of the number read
    back; and the extra "date" output line.
  - The styles probe calls styles() and keeps its counts, then
    add_cell_style(Some(0), Code(arg)) and (None, BuiltIn(14)), then
    set_cell_style on the number and text cells. After the write it
    reads styles() counts and both cells' style through Package::cells.
    That matches "Size".
- Doc commits.
  - Doc 1 carries ROADMAP.md and 0012 with the P2 sections.
  - Doc 2 adds only "Planning rulings during P3" (+61 lines).
  - I cannot check that either one is unedited against planning's
    handoff. Their messages say the hash comparison was made and that no
    log prints it. The ROADMAP.md hash in doc 1's message (9d287364...)
    does equal gate.log line 849's value at the phase commit.

## 2. Do the logs contain every line the spec requires of them?

yes. Every line P3's text, "Size", "Harness runs" P3, rulings 1-7 and the
P2 verification rulings that apply "from P3" require is present in the
five logs, with two exceptions. First, scan.log does not exist yet and I
cannot check it. Second, two requirements depend on documents I was not
given (listed at the end).

- gate.log:
  - Line 11 is the rerun header for attempt 2, with attempt 1's log and
    script hashed at lines 13-14.
  - Nine PASS lines at 1139-1147, and "nine PASS lines: as-expected".
  - The dependency tree with the feature: "the tree with the dates
    feature is the tree without it, byte for byte: as-expected" (938).
  - "gate log checks all as expected: yes" (1175).
  - My counts: 118 ": as-expected", 0 ": UNEXPECTED". The one ": no"
    line is line 16, "ANTHROPIC_API_KEY set in this shell: no".
- test.log (ruling 4):
  - Line 5 is the attempt-3 rerun header, with attempts 1 and 2 hashed
    at lines 7-10.
  - Workspace: "host: passed 463; failed 0; ignored 2" (1615) and the
    same for i686 (2124).
  - `cargo test -p emend`, plain and with the feature, on both targets:
    347 with 0 dates tests and 361 with 14 (2517, 2525, 2536, 2544).
  - Clippy on the core with the feature off, on the host: "host: clippy
    on the core with the dates feature off passes: yes" (2553).
  - Mutations D1-D4 and T1-T3 each compile and each fails the named test
    (2699-2911), with a clean-tree control (2918, 2923).
  - "test log checks all as expected: yes" (3006). My counts: 157
    ": yes", 0 ": no".
- size.log:
  - The core build off and on for both targets, with "all four core
    builds exit 0: yes" (1856-1872).
  - Each of the eleven probes has its own printed build command (1877-1897,
    ruling 7). The dates probe is the only one built with
    `--features dates`.
  - Figures 1-4 at 2220-2233. The part-4 reconciliation, the 5 commits
    and the growth 26348 = 4224 + 22124 are at 2240-2257. Planning's
    expectations are printed as readings at 2261-2264.
  - The release binaries are built and hashed (2448-2452).
  - The record is printed at 2463-2593. I diffed that text, after
    removing the 4-space indent, against size-m1p5-2026-09-22-f565868.md:
    they are identical.
  - "size log checks all as expected: yes" (2605). My counts: 191 ": yes"
    and 7 ": no". The seven are 2151, 2152, 2172, 2173, 2191, 2192 and
    2264, and all are readings.
- negative.log (ruling 6, "P3"):
  - Ruling 6's source lines are printed by grep (3561-3576):
    package.rs:1252 STYLE_OUT_OF_RANGE, 2226 NO_CELL_XFS, 1237
    add_cell_style, and styles.rs:426/554/587 NOT_A_NUMBER. The probe's
    lines 184, 187, 192 and 309 are printed too.
  - W1 (--style-sample, no-cellXfs) is `ec_case W1 2 "$NOXFS_MSG"`
    (script line 1683) and exits 2 with the exact line.
  - W2 (the release probe) is `pr_refused W2 "$RANGE_MSG"` (1687). It
    exits 2 with empty stdout; output 1 is present and 2349 bytes, and
    output 2 is absent.
  - W3 and W4 check NOTANUM_MSG by both routes. W5 is the positive
    control with 13 lines. W6 is the empty code.
  - A1 and A2 check the accepted line in each epoch, and A3 and A4 are
    refusals.
  - V: "reached here: 3; reached by no case here: 2". Each unreached
    message is named beside its package test (4073, 4077).
  - Messages: 51 previous, 0 added, 0 removed.
  - "negative log checks all as expected: yes". My counts: 476
    ": as-expected", 0 ": UNEXPECTED".
- run.log (Harness runs P3):
  - The four release binaries are checked against size.log's hashes
    (2064-2070).
  - On each target: 27 ok, lo and lo_in ok 27, files_identical no 27,
    order preserved 27, "54 entries changed ... and none moved otherwise".
  - Also on each target: "every workbook's entry rows are the two changed
    parts and the rest identical", "xfs_after totals 1019", "date1904
    totals 2", serial 43889.5 in 2 and 45351.5 in 25, the pairing per
    workbook, read_back 1 in all 27, and "changed 0, added 1, removed 0
    in all 27".
  - The per-flag controls (P2 ruling 3) end in "flags turned by some
    pair: 12 of 12".
  - The tokenizer statement P2 verification ruling 1 requires is item 9,
    lines 103-111.
  - "run log checks all as expected: yes". The line "verdict rows other
    than as-expected: 0; of them marked UNEXPECTED: 0" is present. My
    count: 211 ": as-expected".
- I ran my own greps for the three forbidden shapes (chained
  `$((yn|is|isnot) ...&&|||`, leading bang, assignment prefix) over all
  five logs. Every count was 0, which agrees with scan-for-checkers.log
  lines 1385-1389. The two rewritten checks in test.log (script lines
  823-828, 877-880) are named functions, each with a refusing control.

Not checked or not applicable:
- scan.log (final) does not exist yet. The pre-check scan
  (scan-for-checkers.log, third run) reads "RESULT: FAIL" only on "the
  check answers went through all three greps as prose: no" (1392, 1402,
  1404). That result is expected before any answer exists.
- P2 verification ruling 2's label ("the hash planning's handoff gave").
  No P3 log compares a file with a hash planning gave: grep finds the
  0012 hash only in gate.log's file table and the scan's file line, and
  the doc commit messages say no log prints that comparison. The label
  requirement therefore seems to have nothing to attach to. I record
  this as an observation, not a gap.
- Whether W2 and W4's treatment of the probe's outputs matches specs/0011
  P3 ruling 8: 0011 is not among my inputs. negative.log's header (lines
  43-46) states its reading of that ruling.

## 3. Does every number and every quoted string in the revised report appear in one of the primaries?

no. Almost all of them do. The exceptions:

1. Run section: "(the committed records/logs/m1p5-p2/run.log carries the
   same totals line at its line 2335)". P2's run.log is not among my
   inputs, and neither "2335" nor that claim appears in any of them. The
   report states it as fact without marking it as the implementer's own
   statement. It may be true, but I cannot verify it from the primaries.
2. Finding 7: "Attempt 2: ... 211 as-expected, every figure as attempt
   1's". run.log line 12 supports "Attempt 1 passed every check (211
   as-expected, 0 other)". Attempt 1's log is hashed but not given, so
   "every figure as attempt 1's" is unverifiable from my inputs. One
   circumstantial fact: record-attempt-1.txt is 199156 bytes, the same
   size as the new .txt.
3. Minor wording (the content does exist in a primary):
   - The test.log paragraph says "test.log line 5 names the absolute
     path". Line 5 says only "printed an absolute path" and does not name
     the path, which is correct for a committed log.
   - The same paragraph says "whose replacement did not compile". That
     is in a script comment at test.log lines 633-635, not in line 5.
   - It also says "(see Findings 5)", which should read "Finding 5".
4. Finding 6 lists the superseded attempts but leaves out the SECOND run
   of the pre-check scan (scan-for-checkers-2.log, bb8dbb84...). The
   report's own scan.log paragraph names that run, and
   scan-for-checkers.log line 15 hashes it. The list is incomplete, but
   it does not contradict the rest of the report.
5. Not verifiable, and not errors:
   - Doc 1 "as planning handed them over": no primary holds planning's
     hashes.
   - Finding 2's reference to planning's handoff message: the report
     itself says it is not a primary.
   - Finding 3's reason: the report now states plainly that it is an
     unlogged observation. size.log's script comment (lines 56-57)
     states the same thing, which is prose and not a measurement.
6. Placeholders: RECORD_COMMIT, SCAN_LOG_SHA and SCAN_LOG_SUMMARY, as
   expected. The report also still ends with "CHECKS" under
   "## Independent checks". That is a fourth placeholder, for the check
   answers themselves, and the brief does not list it.

Confirmed in the primaries (each number or quote checked):
- Commit ids and subjects, and base 747a96e (gate.log 816-835).
- Every log and record sha256 (my own sha256sum). scan-for-checkers.log
  is c440f66d...; 37b3800e... and bb8dbb84... are at its lines 14-15;
  run-attempt-1.log b4ebedcc... is at run.log line 14.
- gate.log: the line 11 quote; nine PASS names; 118 / 0; the ": no"
  line.
- test.log:
  - The two 463 lines; 347 / 361; dates 0 / 14 on both targets.
  - D1-D4 and T1-T3 each "fails: yes".
  - The two control quotes (2958, 2989).
  - "lines of that file at C2's parent: 0" (2956) and "changed lines in
    C4: 57; of them //! lines: 57" (2987).
  - The two check names quoted in Finding 5 (2957, 2988); "test log
    checks all as expected: yes"; 157 / 0.
  - 14 dates tests and "flag-combination cases the diff adds for the new
    flag: 3" (2982).
- size.log: 191 yes and the seven no lines as described; 304412, 303104,
  235024, 393216, 88804; 4224, 4096, 3536; 26348 = 304412 - 278064;
  22124; drift 0; the five commits; 4980, 4096, 4112; the three
  expectation lines verbatim (2262-2264); "size log checks all as
  expected: yes"; the one-xf fixture with its date1904 variant
  ("generated: ... a styles part of one cell format ... and the same
  with date1904 set").
- negative.log:
  - 476 / 0, and 51 / 0 / 0 with the exact quoted wording.
  - The A1 and A2 lines verbatim (730, 736).
  - W1-W6 as described; W5's 13 lines (791).
  - "add_cell_style begins at line 1237; the first out-of-range raise is
    at line 1252".
  - "reached here: 3; reached by no case here: 2", the two unreached
    messages and both test names (4073, 4077).
  - "five and five" (sections Y3-Y7 and Y8/V).
- run.log:
  - 211; the verdict-rows line; "the binary size.log hashed:
    as-expected".
  - The summary, order, files_identical and entry-totals lines on both
    targets, and the workbooks/cell-count lines.
  - "xfs_after 1019; date1904 2" and the two serial lines.
  - The two "both targets" lines; "flags turned by some pair: 12 of 12".
  - The five pairs (sample, empty, nofmts, d1904 with "1", dtrue with
    "true").
  - The empty pair's two plants that "do not apply ... and says so".
  - The reorder and value plants "alone" on every pair and on the first
    corpus workbook per target.
  - DATED_XF's bytes (646-647).
  - The run directory <corpus>/runs/2026-09-23-f565868-b (2331; record
    .md line 1).
  - The tokenizer quote from P2 verification ruling 1 (0012 line 834).
- The record dates: the size record is 2026-09-22 (size.log run at
  23:50:57Z) and the run record is 2026-09-23 (run.log at 00:17:36Z).

---

## Check 1's items, one by one

- Q2, the missing tokenizer statement: RESOLVED. run.log header item 9
  (lines 103-111) states that the byte spans come from regular expressions
  and cmp.py's top_level and cell_spans, "which end a tag at the first
  `>`", and that planning found no raw `>` in an attribute value. This is
  a new attempt, rerun with its predecessor hashed (line 12-17), not an
  edit.
- Q2, "eleven flags" before twelve: RESOLVED. Item 3 now reads "It raises
  twelve flags" (line 74), and the list that follows has twelve names.
- Q3.1, "chained=2" attributed to the wrong file: RESOLVED. The report no
  longer quotes chained=2 (grep finds none). Finding 5 now attributes the
  finding to the first scan run, scan-for-checkers-1.log, whose hash
  37b3800e... is printed at scan-for-checkers.log line 14. It also cites
  test.log line 5, which states the finding.
- Q3.2, "package.rs:1238-1242": RESOLVED. The numbers are gone (grep finds
  neither). Finding 2 now says ruling 6 cites no line numbers and that the
  handoff message is not a primary, and it cites negative.log's 1237/1252,
  which are present.
- Q3.3, Finding 6 incomplete: MOSTLY RESOLVED. It now lists gate attempt
  1, test attempts 1 and 2, run attempt 1 with its record pair, and the
  first scan run. It still leaves out the second scan run (see Q3 item 4
  above).
- Q3.4, the clippy citation: RESOLVED in substance. The report now cites
  line 5 for the absolute path and line 454 for the clippy block. Line 454
  is the comment "(attempt 1 of this log printed it bare, an absolute
  machine path)" inside the clippy block (449-455). The wording "names the
  absolute path" for line 5 is still loose.
- Q3.5, P2's run.log totals: NOT RESOLVED. The report now adds "at its
  line 2335", but the file is still not among the primaries, and the
  claim is still stated as fact rather than marked as the implementer's
  own.
- Q3.6, Finding 3 unmeasured: RESOLVED by disclosure. Finding 3 now says
  the reason "is my own unlogged observation from a scratch run ... stated
  here as a reason and not a measurement".
- Q3.7, doc 1 "as planning handed them over": UNCHANGED. It is still
  unverifiable from the primaries, and it is not an error.
- Q3.8, placeholders: as expected (RECORD_COMMIT, SCAN_LOG_SHA,
  SCAN_LOG_SUMMARY), plus CHECKS (see Q3 item 6).
- Check 1's minor notes on the C3 and C4 messages: RECORDED, not changed.
  The report's Finding 8 states both and explains why the commits are not
  amended. I confirm both:
  - C3's message says "no log prints a count at this commit", yet test.log
    prints C3's 3 flag-combination cases (2982) and the 4 integration
    tests (2159). Its counts are therefore printed, and only the
    disclaimer is wrong.
  - C4's "The ten styles messages" is printed by none of my inputs.
    negative.log gives them as five (Y3 to Y7) and five (Y8/V). Under
    specs/0001 ("a count it states is one a committed log prints, or the
    message says that no log prints it"), this stands unless an earlier
    committed log (P1's) prints ten, which I cannot check.
- Check 1's observation on test.log's short run time (00:09:00Z to
  00:09:47Z): unchanged. I have no evidence either way.
