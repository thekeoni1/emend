# Independent check 1: M1 part 5 P3

Written by the implementing session. The header is mine; everything after
the rule is the checker's answer, byte for byte as it was returned.

The checker was a fresh subagent (not a fork) with no access to this
conversation. It was given the twenty files below and nothing else, and
asked the three questions CLAUDE.md names, with specs/0001's input list in
P1's shape: the two specs, the phase diff and one diff per commit, the draft
report, every log the phase had written and the scan for the checkers. It
was told to write <repo>, <scratch>, <home> and <corpus> in place of
absolute paths and to answer in ASCII.

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
- 757407b66d68ca336dc6327e866f9422dac1775b558b9efe8e3f91d290832327
  m1p5-p3-2026-09-23-f565868.md (the run record .md of run.log attempt 1; superseded, hashed in the committed run.log's rerun header)
- 512f7837942585d3d6487d0b81e739c9ab257995db56e7ec77dee094ca2ae341
  m1p5-p3-2026-09-23-f565868.txt (the run record .txt of run.log attempt 1; superseded, hashed likewise)
- ddbd418538fb4e50afca98e991e098f323c3a57fae7aeef4f03e0a4631db0778
  negative.log (records/logs/m1p5-p3/negative.log, the committed one)
- c090fec28f6984b5bc2c640d5a2174f01987800747e4edc2eb0b826c7e052193
  report-draft.md (the draft report as check 1 read it; the report sent to planning is later)
- b4ebedccfb00a564f9dfdfff1fc260d6d92807c00ccc963b47747f0f736128e0
  run.log (run.log attempt 1; superseded, hashed in the committed run.log's rerun header)
- bb8dbb84f60258061468a19ab82c41e603b5646406a4659691562ccc44b31b51
  scan-for-checkers.log (the second scan run for the checkers; superseded, hashed in the committed scan.log's rerun header)
- 3a270d092928321d4e7e0dc4cfa6ec91698e6d51305a351c6d05a5eeeef09159
  size-m1p5-2026-09-22-f565868.md (records/size-m1p5-2026-09-22-f565868.md, the committed one)
- 270faa28d564b6ff0c4b25f81e7673777a1fa47de105846f56c849fc310abf3f
  size.log (records/logs/m1p5-p3/size.log, the committed one)
- aaea9f937a7437d6818d2ffaeae5908bea7b10bd43ab0096c6c4fa2b5121b3f4
  test.log (records/logs/m1p5-p3/test.log, attempt 3, the committed one)

The answer as returned: 14669 bytes, sha256
c6312fa953731e5b69e562224a5f5f962734b459e73f3eed282348d8e23c8aed.

---

# Independent check 1, M1 part 5 P3

Inputs read: every file in the check directory (the two specs, the five
per-commit code diffs, the two doc diffs, the phase diff, gate.log,
test.log, size.log, negative.log, run.log, scan-for-checkers.log, the
size record, the run record pair and report-draft.md). The harness also
placed the repository's CLAUDE.md and a memory index in my context; I
have not used either as evidence. I recomputed sha256 over the input
files: every hash the draft report gives for the logs and records
matches the file I was given, and 0012-styles.md hashes to dafccc67...,
the value gate.log prints for specs/0012-styles.md at the phase commit.

Summary: 1 yes; 2 no (one required header statement is missing from
run.log); 3 no (one quoted string attributed to a log I was given is not
in it, one citation has no source among the primaries, and one finding
contradicts the rest of the report).

---

## 1. Does the diff do what the spec says for P3, and nothing else?

yes.

Diffs read: diff-code-1..5 for each commit's "nothing else"; diff-doc-1
and diff-doc-2 for the doc commits; diff-phase.patch for the whole. Added
and removed line counts across the seven per-commit diffs (+1844/-12)
equal the phase diff's (+1844/-12), and the phase diff names only the 14
files gate.log lists under "files in the diff: 14".

- Commit order and split (ruling 1): gate.log "the phase's commits" shows
  doc 6256267, doc 130f7ff, then C1 805526d, C2 c08bf76, C3 b2bf6e3,
  C4 478da97, C5 f565868, each code commit's parent the previous one
  (test.log: "each code commit is the next one's parent, C1 to C5: yes").
- C1 (diff-code-1): one comment line in tools/emend-copy/tests/
  null_rewrite.rs, "the first addition refuses" -> "the styles read
  refuses" (P2 verification ruling 6). Nothing else.
- C2 (diff-code-2): `[features] dates = []` with no dependency;
  dates.rs with Epoch {Y1900, Y1904} (Copy, PartialEq), Epoch::of,
  DateTime with the seven fields (Copy, PartialEq, Debug), serial and
  from_serial; the module and re-export under cfg(feature = "dates") in
  lib.rs. serial refuses month outside 1-12, day 0 or past the month
  (so 1900-02-29), hour > 23, minute/second > 59, millisecond > 999
  (ruling 3), dates before the epoch's first day and after 9999-12-31.
  from_serial checks finiteness, rounds to the millisecond first, then
  applies first-day, end and day-60 refusals to the rounded value
  (ruling 2). Tests (14, dates/tests.rs) cover every item "P3" lists:
  the pinned values in both epochs, noon and a millisecond, 1900-02-29
  and day 60, the day before each epoch and after 9999-12-31,
  2023-02-29 / month 13 / day 0 (plus month 0 and 2023-04-31), NaN and
  both infinities, the carry into the next day, the round trip, ruling
  3's hour 24 / minute 60 / second 60 / millisecond 1000, and ruling 2's
  three boundaries from both sides. The derives Eq (and Debug on Epoch)
  go past the spec's list; they are harmless. No new dependency: gate.log
  "the tree with the dates feature is the tree without it, byte for byte:
  as-expected".
- C3 (diff-code-3): emend-copy enables emend's `dates`; --date-sample adds
  one xf over no base with BuiltIn(22), sets (M + 1, 1) to the serial of
  2024-02-29 12:00:00.000 in Epoch::of the workbook, writes, reopens,
  asserts xfs_after = X + 1 and cells_after = cells_before + 1 (removing
  the output on failure), and prints the eight keys in ruling 5's order
  (date1904, serial, added, xfs_before, xfs_after, cells_before,
  cells_after, read_back). read_back requires number() == serial, style
  == the new index, and from_serial equal to the date written, as
  "Harness runs" P3 says. The edit_line -> print_line split and the
  styled_run / styled_run_refused wrappers are refactors the new flag
  shares; C3's message names both. Four integration tests and three
  flag-combination cases, as its message says.
- C4 (diff-code-4): 57 added lines, all `//!` (test.log: "changed lines
  in C4: 57; of them //! lines: 57", with a refusing control). Limits in
  package.rs, styles.rs and an Errors/Limits section in dates.rs: number
  formats only, the refusals, the feature outside the core. No code.
- C5 (diff-code-5): size-probe's `dates = ["emend", "xml", "emend/dates"]`
  (ruling 7), two [[bin]] entries, probe-emend-styles-write-xml
  (styles() counts, add_cell_style(Some(0), Code(arg)) and (None,
  BuiltIn(14)), set_cell_style on the number and text cells, post-write
  styles() counts and the two styles read back), and
  probe-emend-dates-write-xml (the same, the number cell holding the
  serial of a DateTime parsed from seven integers with str::parse, and
  from_serial of the number read back printed). This matches "Size".
- Doc commits: doc 1 carries ROADMAP.md and the spec with the P2 rulings
  (P2 verification ruling 9); doc 2 adds "Planning rulings during P3"
  before the first code commit. Whether they are "unedited" relative to
  what planning handed over I cannot check: no primary holds planning's
  hashes for those blobs.

Minor notes, not departures: C3's message says "no log prints a count at
this commit", but test.log's C3 block does read counts at C3 ("flag-
combination cases the diff adds for the new flag: 3"); the message was
written before the log. C4's message states "The ten styles messages";
no single log line prints ten (negative.log prints them as five Y3-Y7
reading messages and five V messages).

## 2. Do the logs contain every line the spec requires of them?

no. Every line P3's own text requires is present, but one statement a
P2 verification ruling requires of a later comparison is missing, and
one run.log header line is wrong.

Missing:
- P2 verification ruling 1 ends: "The hazard did not arise, and a later
  comparison that keeps the tokenizer says so in its header." run.log's
  cmp.py keeps that tokenizer (top_level: `close = data.find(b">", i)`,
  ending a tag at the first `>`), and run.log header item 3 says only
  "It keeps P2's readers and byte spans". No line in run.log's script
  header or in cmp.py's docstring states that the tokenizer ends a tag
  at the first `>` or that no attribute value holds a raw `>`
  (`grep -i token` over run.log finds nothing). I read the ruling as a
  requirement; if planning meant it as a permission, this item falls.

Wrong:
- run.log header item 3: "It raises eleven flags:" followed by twelve
  names (head_same, no_fmt_added, tail_same, xfs_prefix, xfs_added,
  dated_xf_ok, xf_count_grew, ws_prefix_same, ws_suffix_same,
  nothing_changed, added_as_edited, nothing_removed). The output says
  "flags turned by some pair: 12 of 12". A stale count in the header; the
  checks themselves cover all twelve.

Present (with the evidence):
- test.log (ruling 4): "cargo test -p emend" plain and "--features dates"
  on both targets, 347/361, "the plain run holds no dates test (0): yes"
  and "the feature run holds exactly the workspace run's dates names:
  yes" per target; "cargo clippy -p emend --all-targets -- -D warnings"
  once on the host, "host: clippy on the core with the dates feature off
  passes: yes". Workspace "passed 463; failed 0; ignored 2" on both
  targets. Mutations D1-D4, T1-T3 each with the failing test named.
  "test log checks all as expected: yes". Rerun header hashes attempts
  1 and 2 (log and script).
- size.log: all eleven probes built with their own --features (each
  command printed, ruling 7); "all four core builds exit 0: yes" for
  cargo build -p emend off/on on both targets; figures 1-4 under "the
  figures specs/0012 asks for"; the whole growth 26348 = 4224 + 22124;
  part 4's nine classified (six identical, three "code" with the five
  commits listed); planning's expectations printed as readings
  ("the feature's cost 4980 is under 4000: no"); release binaries and
  the probe hashed; the record printed. I diffed the record text size.log
  prints against size-m1p5-2026-09-22-f565868.md: identical. "size log
  checks all as expected: yes".
- negative.log (ruling 6): W1 --style-sample on the no-cellXfs fixture,
  exit 2, "a styles part without cellXfs cannot take a style"; W2 the
  release probe (hash checked against size.log), exit 2, "a style index
  is out of range", output 1 present, output 2 absent; W3/W4 "a style
  attribute is not a number" by both routes; source lines printed
  (package.rs:1237 add_cell_style, 1252 the raise, 2226 NO_CELL_XFS;
  probe lines 184, 187, 192, 309); V names the two unreached messages
  beside their package tests. "negative log checks all as expected: yes".
  Whether W2's treatment of output 1 matches specs/0011 P3 ruling 8 I
  cannot check: 0011 is not among my inputs.
- run.log: release binaries checked against size.log's hashes; on both
  targets 27 ok, lo and lo_in ok 27, two changed entries per workbook,
  "xfs_after 1019; date1904 2", "serial 43889.5 in 2 and 45351.5 in 25",
  pairing asserted per workbook, read_back 1 in all 27, changed 0 /
  added 1 / removed 0, dated xf byte-equal to the numFmtId="22" xf, no
  numFmt added; per-flag controls (P2 ruling 3); `#!/usr/bin/env bash`
  and the invocation printed (P2 ruling 4). "run log checks all as
  expected: yes".
- gate.log: nine PASS lines, the dependency tree with and without the
  feature, no gate file changed. "gate log checks all as expected: yes".
- scan.log: only the pre-check scan is present (scan-for-checkers.log,
  second run, "RESULT: FAIL" on "the check answers went through all
  three greps as prose: no"). That is expected before any answer exists;
  the final scan.log is still to be written and I cannot check it.

## 3. Does every number and every quoted string in the draft report appear in one of the primaries?

no. Most do; the exceptions:

1. Finding 5 quotes scan-for-checkers.log as saying "the forbidden
   shapes in records/logs/m1p5-p3/test.log: chained=2". The
   scan-for-checkers.log I was given (sha256 bb8dbb84..., the hash the
   report gives) says at line 1385 "the forbidden shapes in
   records/logs/m1p5-p3/test.log: chained=0 leading-bang=0
   assignment=0". The chained=2 line would be in the FIRST scan run
   (scan-for-checkers-1.log, hashed in the rerun header, not among my
   inputs). The quote is attributed to the wrong file. `grep chained=2`
   over all inputs finds only report-draft.md.
2. Finding 2 says "The ruling cites package.rs:1238-1242 for the raise
   inside add_cell_style". Ruling 6 in 0012-styles.md (lines 935-943)
   cites no line numbers; "1238" and "1242" appear in no input except
   the report. The numbers 1237 and 1252 are in negative.log.
3. Finding 6 says the superseded attempts are "gate.log attempt 1 and
   test.log attempt 1". test.log attempt 2 was also superseded (test.log
   line 5 and its rerun header hash test-attempt-2.log, 1312943d...), as
   the report itself says at line 64 and in Finding 5; so was the first
   scan run (scan-for-checkers-1.log, 37b3800e...). Finding 6 is
   incomplete and contradicts the rest of the report.
4. The report's test.log paragraph says attempt 1 "printed the clippy
   block's crate path as an absolute path (test.log line 5)". Line 5
   says only "printed an absolute path"; the clippy detail is in the
   script comment at test.log line 453-454 ("attempt 1 of this log
   printed it bare"). The content is in a primary; the line citation is
   imprecise.
5. "(P2's run.log carries the same totals line)": P2's run.log is not
   among my inputs, so I cannot confirm this.
6. Finding 3 ("Part 4's generated fixture has an empty styles part,
   which the new styles probe refuses (base 0 is out of range)") rests
   only on a size.log script comment (line 56). No log line shows that
   refusal being run. Under specs/0001's rule that a figure in a script's
   prose or comments is not a measurement, this is an unmeasured claim.
   negative.log W2 shows the probe refusing a no-cellXfs fixture, which
   is a different fixture.
7. Doc 1 "as planning handed them over": I cannot check this, as said
   under question 1.
8. Placeholders are still unfilled: RECORD_COMMIT, SCAN_LOG_SHA,
   SCAN_LOG_SUMMARY, CHECKS.

Numbers and quotes I confirmed in the primaries:
- Commit ids and subjects (gate.log, test.log); base 747a96e (gate.log).
- Log and record sha256s (my own hashes of the files, and gate/negative/
  run logs where they print them); scan-for-checkers.log bb8dbb84....
- gate.log: 118 as-expected, 0 UNEXPECTED, the one ": no" line
  "ANTHROPIC_API_KEY set in this shell: no", nine PASS names, the line 11
  quote.
- test.log: 157 yes, 0 no; "host: passed 463; failed 0; ignored 2" (and
  i686); 347 / 361, dates 0 / 14; D1-D4, T1-T3; the two control lines
  quoted; "lines of that file at C2's parent: 0" and "changed lines in
  C4: 57; of them //! lines: 57" (present in attempt 3 as well).
- size.log: 191 yes; the seven ": no" lines as described; 304412,
  303104, 235024, 393216, 88804; 4224, 4096, 3536; 26348 = 304412 -
  278064; 22124; drift 0 for probe-zip-xml; the five commits
  35cd499, abf2618, 07b84be, c08bf76, 478da97; 4980, 4096, 4112; the
  three expectation lines. I recomputed each subtraction from the raw
  probe figures (e.g. 837628 - 533216 = 304412; 842608 - 837628 = 4980).
- negative.log: 476 as-expected, 0 UNEXPECTED; 51 / 0 / 0; the A1 and A2
  edit lines; W1-W6 messages, W5 13 lines; "add_cell_style begins at
  line 1237; the first out-of-range raise is at line 1252"; "reached
  here: 3; reached by no case here: 2"; both test names.
- run.log: 211 as-expected; "verdict rows other than as-expected: 0; of
  them marked UNEXPECTED: 0"; "the binary size.log hashed: as-expected"
  (as a substring of each binary's line); the summary, order,
  files_identical and entry-totals lines on both targets; "workbooks:
  27; the tool and the comparison agree: 27; every flag holds: 27";
  "cell counts as P3: 27; read_back is 1: 27; serial paired with
  date1904: 27"; "xfs_after 1019; date1904 2"; the two serial lines; the
  two "both targets" lines; "flags turned by some pair: 12 of 12"; the
  five control pairs; the empty pair's "plants that do not apply: 2";
  the reorder and value plants on every pair and on the first corpus
  workbook per target; the DATED_XF bytes; the run directory
  <corpus>/runs/2026-09-23-f565868.
- The record dates (size record 2026-09-22, run record 2026-09-23) match
  size.log's 23:50Z start and run.log's 00:00Z start.

Not an error, only an observation: test.log ran from 00:09:00Z to
00:09:47Z, covering two workspace test runs, four crate runs, several
clippy/build runs and seven mutation builds. That is fast, but a warm
cache from attempts 1 and 2 could explain it; I have no evidence either
way.
