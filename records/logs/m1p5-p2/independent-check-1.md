# Independent check 1: M1 part 5 P2

Written by the implementing session. The header is mine; everything after
the rule is the checker's answer, byte for byte as it was returned.

The checker was a fresh subagent (not a fork) with no access to this
conversation. It was given the twelve files below and nothing else, and
asked the three questions CLAUDE.md names, with the instruction to write
<repo>, <scratch>, <home> and <corpus> in place of absolute paths and to
answer in ASCII. The input list is planning's ruling of 2026-09-22 (P1's
shape: specs/0001, the phase spec, the phase diff and one diff per commit,
the draft report, every log the phase had written, and the scan run for the
checkers). An earlier check 1, launched by the session that began P2, died
with that session before it answered; nothing of it is recorded.

Commit checked: 7c965755863bbcc45b32cc36d92b29baf1067cbf
("M1 part 5 P2: emend-copy --style-sample"), which was HEAD when the check
ran and when this file was written. Range read: c3d1adb..7c96575.

The inputs, each as the checker received it (sha256, then the name it had in
the check directory and what it is):

- 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
  0001-bootstrap.md (specs/0001-bootstrap.md at 7c96575)
- 88299b3dee58164ec567efbf5a64a0fc201b1c5694da4f6bc2368fc1a787b075
  0012-styles.md (specs/0012-styles.md at 7c96575)
- cd357aacea4b1fe8c68ee82b4e1dcd52a1eea180fa2a8acae99d87ef4c4a2591
  diff-phase.patch (c3d1adb..7c96575)
- f4668caebe609d3b45f113d4de91ac4c71fd2cbd112c9183841d04b482e26b1e
  diff-doc.patch (c3d1adb..6e6062c)
- da8f0c9cabae4f7203af20df229eab329fcb37a8e4574c3fad29202265a0423c
  diff-code-1.patch (6e6062c..07b84be)
- 5f0b83025e1d4e5d61eab07301a8abfe16dabcd86ca2811e8b1b1ffc8a2ce0f2
  diff-code-2.patch (07b84be..7c96575)
- da996d7dbd806daa06a89ff066c6809a1b4bbb98c159c116b22620e9bc7a41de
  report-draft.md (the draft report before any check; the report sent to
  planning is later and answers the findings)
- bd54f9e7133fb4470b4a27bfd4c4b32ef6d690bdaec4eb07060fe3586e9e1a41
  gate.log (records/logs/m1p5-p2/gate.log, attempt 2, the committed one)
- 955e2a8e1cfce3a365dd866b2b9390b7f629335dfd7711ab5285fa1daa160254
  test.log (records/logs/m1p5-p2/test.log, attempt 3, the committed one)
- 33ff400a70e3b7aaa2b4969a3501df6d4b46a0b50457a869cb28916228a5d243
  negative.log (attempt 2, superseded after this check by attempt 3 under
  planning's P2 ruling 3; hashed in the committed negative.log's rerun
  header)
- a815f652f16c107e3bff5d897f8fe75425494c9fee7db3fd50df73779b888e98
  run.log (attempt 2, superseded after this check by attempt 3 under
  planning's P2 rulings 1, 2 and 6; hashed in the committed run.log's
  rerun header)
- 7f575cf3c55b78de7523e50bb6d5cc1c794e08e24267ff11ef5bcab79dfb946b
  scan-for-checkers.log (the first scan run for the checkers; superseded,
  hashed in the committed scan.log's rerun header)

The answer as returned: 16582 bytes, sha256
1822672adf83fe18699daa41563e8aa12a441edd97ab9663f818ed320064daa6. Every byte
of it is ASCII and none is a control byte other than the line feed. No
substitution was made: the text below is the answer as returned. Its
opening says "the eleven input files"; it was given the twelve above, and
its own list of files read names all twelve.

---
# Independent check 1, M1 part 5 P2

Disclosure: the project instruction file (CLAUDE.md) was placed in my
context by the harness as a system reminder; I did not use it as evidence.
Everything below rests only on the eleven input files in <scratch>.

Files read in full or in chunks: 0001-bootstrap.md, 0012-styles.md,
diff-phase.patch, diff-doc.patch, diff-code-1.patch, diff-code-2.patch,
report-draft.md, gate.log, test.log, negative.log (script header, the
message-list block, and every output section), run.log, scan-for-checkers.log.
I also confirmed mechanically that diff-phase.patch is exactly the union of
the three per-commit diffs (same five file sections, identical hunks once the
`index` lines are set aside).

## 1. Does the diff do what "P2" and "Harness runs" say, and nothing else?

Mostly yes, with one departure that sits outside the diff and one small
inaccuracy in a test comment.

What the diff does, against the spec:

- Doc commit (read in diff-doc.patch): ROADMAP.md (P1 row to DONE with "ten
  messages", P2 row to READY) and specs/0012-styles.md (the P1 verification
  rulings section plus the edits in "Crate", "what can be added", the
  numFmts creation point, the clear-with-Some sentence and the P2 paragraph).
  0012 "P1 verification ruling 10" says P2's first doc commit carries this
  spec and ROADMAP.md unedited. The diff shows only those two files. I cannot
  confirm "unedited" from the diff: that needs planning's own copy. The
  0012-styles.md I was given (44039 bytes) matches the byte count gate.log
  prints for specs/0012-styles.md at the phase commit ("bytes 44039").
- Code commit 1 (read in diff-code-1.patch): only
  crates/emend/src/package/tests.rs. It adds `p.clear_cell(SHEET, at("E1"))`
  and `p.clear_cell(SHEET, at("A7"))` to
  `a_style_edit_alone_on_an_absent_cell_writes_it_or_nothing`, plus a comment.
  The expected string is not touched, so the test asserts that both clears
  write nothing. This is what 0012 "P2" asks for ("the clear-alone-on-an-absent-cell
  case added to the test ... (ruling 8)"). Nothing else in this commit.
- Code commit 2 (read in diff-code-2.patch): tools/emend-copy/src/main.rs and
  tools/emend-copy/tests/null_rewrite.rs. Checked against "Harness runs",
  `--style-sample`:
  - X and F come from `styles.cell_xfs.len()` and `styles.num_fmts.len()`
    (diff-code-2 lines 151-153).
  - The first `SheetKind::Worksheet` is read once through `sample()` for A,
    M and the cell count (lines 144-154). `sample()` gains `first_style` and
    `first_value` (lines 95-117).
  - S1 is `add_cell_style(before.first_style, NumberFormat::Code("0.000"))`,
    with a `None` base when A is absent or has no `s`. S2 is
    `add_cell_style(None, NumberFormat::BuiltIn(14))` (lines 165-167).
  - A gets `Some(coded)` when it exists, and `(M + 1, 1)` gets
    `Value::Number(45000.5)` and `Some(dated)` (lines 168-172).
  - The tool writes, reopens, reads the output's styles and the same sheet
    (lines 174-201, 223-267).
  - It asserts `xfs_after == X + 2` and `cells_after == cells_before + 1`.
    A failure raises `Error::XfsAfter` or `Error::CellsAfter` and removes the
    output (lines 196-201, 240-265).
  - `read_back` has three terms. (a) A's value is unchanged and its `s` is
    `Some(coded)`. (b) The new cell holds 45000.5 with `Some(dated)`. (c) Both
    number formats read back (lines 237-259).
  - The line's keys come in the spec's order (lines 202-214).
  - The tool takes no reference, code or date from the command line: the
    only new argument form is `[input, output, flag]` (lines 67-69).
  The integration tests use the tool's existing `edit_specs` fixture, extended
  with a fictional styles part. That matches "an integration test on a
  fictional workbook (the tool's existing fixtures)". There are four new
  tests and three new flag-combination cases.
- The runner and the record script are unchanged. Neither
  scripts/passthrough-run.sh nor scripts/passthrough-record.py appears in
  diff-phase.patch, which matches "The runner and the record script are
  unchanged".

"Nothing else": confirmed from diff-phase.patch, cross-checked against
diff-doc.patch, diff-code-1.patch and diff-code-2.patch:
- The phase diff holds exactly five files, and gate.log agrees: "files in
  the diff: 5".
- Every hunk in main.rs is part of the new operation: usage text, the
  `Operation` variant, the `Error` variant and its Display arm, the dispatch,
  the `Sampled` fields and the two new functions.
- The shared `sample()` now also clones A's value for every caller,
  `--edit-sample` included (diff-code-2 line 117). That is harmless, but it
  does touch a path other than the new operation.

Findings on question 1:
- DEPARTURE, outside the diff but part of what "P2" asks for. 0012 "P2" asks
  for "the run log's `ElementTree` and byte-span check", and "Harness runs"
  says "the run log's own count ... with `ElementTree`". run.log's
  comparison uses `xml.parsers.expat` directly: run.log line 36 reads "## the
  comparison this log runs (python3, zipfile and expat)", and line 85 is
  `import xml.parsers.expat`. It is still an independent parser, since it
  shares no code with the library. But it is not the parser the spec names,
  and neither the report nor the log declares the departure.
- A small inaccuracy in a test comment (diff-code-2 lines 532-533). It says
  "the first addition refuses before any output file exists". In the tool,
  `package.styles()?` (line 151) runs before the first `add_cell_style` and
  is what refuses. The message is the same, so the test stays correct.

## 2. Do the logs contain every line the spec requires?

No. Most required lines are present, but run.log does not print the script
that ran it.

Present, with evidence:
- gate.log: phase commit given and compared with HEAD, with a null-sha and a
  short-sha control (lines 2-8). The rerun note and the hashed attempt 1
  (lines 10-12). Nine PASS lines on a cold cache after `cargo clean`. The ten
  gate files listed against the diff: "gate files checked: 10; of them in the
  phase diff: 0". The four debug binary hashes. The printed script. Status
  accounted for.
- test.log: both targets, "host executed 445 tests;
  i686-unknown-linux-musl executed 445 tests". The four added names. The
  mutations C1 to C5, each with its planted change, the run and the failing
  assertion's output, plus an unmutated control. The code commits' claims
  read at each commit. Attempts 1 and 2 hashed.
- negative.log: the four binaries are copied in and matched to the hashes
  gate.log printed. Sections N56 to N62 and Z1 to Z5 all pass. The catalogue
  section C is present. It ends "negative log checks all as expected: yes".
  Attempt 1 is hashed.
- run.log:
  - The comparison and the generator for its controls are printed.
  - Three control pairs are covered. Every one of the 17 flags is turned by
    a plant on some pair ("planted faults that turned their own flag, over
    the three pairs: 46"), and the plants that cannot apply say so.
  - On both targets it prints "summary rows: 27; outcome ok: 27; lo ok: 27;
    lo_in ok: 27; leftover no: 27", "order_preserved yes: 27", "files_identical
    no: 27", "xfs_after totals 1046", "formats_added totals 27", "read_back is
    styled plus two in all 27", and "every workbook's entry rows are the two
    changed parts and the rest identical".
  - The two labels agree. The record pair is hashed. I recomputed the
    per-workbook lines myself. No flag reads `no` in any of the 54 `cmp`
    lines. `fmt_count_in_matched` and `xf_count_in_matched` are yes in all 27
    on both targets. The per-target sums are xfs_before 992, xfs_after 1046,
    formats_added 27 and styled 27. The host and i686 edit lines are
    identical.
- scan-for-checkers.log: the self-scan runs before the script is printed.
  f_home is in its narrowed form. It has the three shape greps with their
  controls, and it scans the four logs and the record pair. It ends
  "structural checks failed: 1" and "RESULT: FAIL". The one failure is "the
  check answers went through all three greps as prose: no" with "check
  answers scanned: 0". Header item E says this is the pre-answer run and a
  final scan follows. That matches specs/0001, but it means no passing scan
  log exists yet.

Missing or not shown:
- run.log does not print the script that ran it. The rule in specs/0001
  ("Rules for every phase") is "Print the exact script into the log."
  run.log's sections (lines 36, 694, 851, 863, 882, 1001, 1120, 1125, 1135,
  1145, 1152) print the Python comparison and generator, but no shell
  driver. The log's own header says the corpus directory's name "appears in
  the script printed below, in the line that takes the default CLAUDE.md
  gives", and no such script is below. Functions whose results the log
  reports, such as `entries_p2`, `columns_resolve` and `all_yes`, never
  appear as code. This has two consequences:
  - scan-for-checkers.log's line "every printed script went through all
    three greps: 7 files: yes" covers only run.log's Python for this log. The
    shell script that holds the checks was never scanned for the forbidden
    shapes.
  - The P2 acceptance "every other entry `identical` unflagged, `descriptor`
    alone allowed on a changed entry" rests on a check I cannot read. The
    summary prints "flagged 34" on both targets (run.log lines 917 and 1036),
    and no per-entry flag is printed. From the primaries I can confirm only
    the check's verdict ("entry rows as P2 expects: 27") and its controls'
    labels, not what it tests.
- The spec says the new xf must be exactly `<xf numFmtId="14" fontId="0"
  fillId="0" borderId="0" xfId="0" applyNumberFormat="1"/>`. run.log's
  `dated_xf_ok` tests that each of the six attribute strings is present,
  that the element is empty and that it is a single element. It does not
  test byte equality, although its docstring says "by bytes". The flag
  would accept reordered or extra attributes. The integration test
  (`ADDED_DATED_XF`) pins the exact bytes on the fixture, but the corpus
  check does not.
- Harness runs asks that "`count` grows by one" and "by two". run.log's
  `fmt_count_grew` and `xf_count_grew` hold the output's count to the
  output's own children, and those children to the input's plus one or two.
  They do not compare against the input's `count` attribute. The report
  discloses this as its decision 1. Together with `*_count_in_matched=yes`
  in all 27, the spec's reading also holds on this corpus. Planning has to
  rule on it.
- negative.log contradicts itself about the message list:
  - Its header item 2 says: "The message list is compared BY NAME with P1's
    committed negative.log: none may be removed, and P2 adds none".
  - The code compares with a different log:
    `PREV_NEG=$MAIN/records/logs/m1p4-p3/negative.log`.
  - The output reads "the previous phase's count, read from
    records/logs/m1p4-p3/negative.log: 41" and "P1 adds ten messages to the
    library, the number specs/0012 names: as-expected".
  So no log shows that P2 adds no library message. The P1 comparison is
  carried over unchanged.
- The logs carry several stale script comments. None affects a result:
  - gate.log's header says it was written "from P1's gate script
    (records/logs/m1p5-p2/gate.log)", and its item 5 still says the
    code-commit control names crates/emend/src/package.rs. The body names
    tools/emend-copy/src/main.rs.
  - test.log's item 1 says previous figures are read from
    records/logs/m1p5-p2/test.log. The code reads m1p5-p1.
  - negative.log's header says "from P1's negative script
    (records/logs/m1p5-p2/negative.log)".
- I cannot check the commit messages themselves: they are not among my
  inputs. test.log checks the tests and counts it says the messages name,
  and scan-for-checkers.log shows that each message ends with the
  Co-Authored-By line.

## 3. Does every number and quoted string in the draft report appear in a primary?

No. Most do, but some do not, and one claim is contradicted.

Supported:
- The commit ids and base c3d1adb (gate.log).
- The hash prefixes ea9c27add247cb7d and 88299b3dee58164e (gate.log,
  "ROADMAP.md: sha256 ea9c27add247cb7d..." and "specs/0012-styles.md:
  sha256 88299b3dee58164e...").
- E1 and A7 (diff-code-1).
- "four integration tests and three flag-combination cases" (diff-code-2;
  test.log "flag-combination cases the diff adds for the new flag: 3").
- "Nine PASS on a cold cache" and gate attempt 1's cause (gate.log lines
  10-12).
- C1 to C5, each caught by the test its line names (test.log).
- test attempt 2's cause (test.log rerun line).
- N56-N62 and Z1-Z5 (negative.log).
- negative attempt 1's cause (negative.log rerun line).
- run.log attempt 1's cause: "six parts", no final newline, "missing
  command twice" (run.log line 11).
- 27 workbooks, 25.2.3-2-deb13u6, and "the debug binaries gate.log hashed".
- The corpus counts: outcome ok 27, lo ok 27, lo_in ok 27, leftover no 27,
  order_preserved yes 27, comment_equal yes 27, files_identical yes 0.
- "seventeen flags" (17 PLANTS; "plants that turned their flag: 17").
- The totals xfs_after 1046, formats_added 27 and styled 27.
- The three control pairs, including the no-numFmts pair.
- Record-pair lines 1139-1140, 14978 and 199072 bytes, both sha256 values,
  and "Neither half names a home, mount or corpus path" (run.log
  1139-1140, 1145-1150).
- `fmt_count_in_matched` and `xf_count_in_matched` yes in all 27 on both
  targets (my recount of run.log).

Not in any primary, or contradicted by one:
- CONTRADICTED. Report item 2 says: "The run script is ... printed into the
  log in full, as P1's was ... the log carries the assembled text, which is
  what any reader checks." run.log prints no shell script, only its Python
  helpers (see question 2). The statement is false as it stands.
- NOT IN A PRIMARY. The line reference "package.rs:1389" in decision 1.
  crates/emend/src/package.rs is not in any diff, and no log prints that
  line or its content.
- NOT IN A PRIMARY. "Both hashes matched the prefixes planning gave before
  the commit was written". The prefixes are in gate.log. That planning gave
  them, and that they matched before the commit, appears in no primary.
- NOT IN A PRIMARY. test.log attempt 1's details: "the block reading each
  code commit's claims still named P1's commit subjects", and the "three
  flaws" (a bare `--`, a check that could not fail, an unused helper). The
  rerun line says only "Attempt 1 failed a different check of this
  script's and is kept as well", with its hash. The same holds for "Attempts
  1 and 2 failed one check each": the log says attempt 2 failed one check,
  and it does not give a count for attempt 1.
- NOT IN A PRIMARY. For run.log, "its abandoned run directory was removed".
  The rerun line says only that this attempt's run directory "is a fresh
  one". "failed no check" is also slightly stronger than the log's "stopped
  by hand ... not by a check".
- NOT LITERALLY PRINTED. "0 UNEXPECTED". run.log never prints that phrase.
  It prints "run log checks all as expected: yes", and the word UNEXPECTED
  does not appear in the output, so the claim is consistent with the log.
- MISLEADING BY OMISSION. The report says C1 "is caught by ruling 8's case,
  which is the measurement P1's check 2 said a phase diff could not
  supply". That the case catches C1 is in test.log. But test.log also shows
  that C1 breaks four tests that already existed:
  - a_cleared_cell_keeps_its_style_or_goes_with_its_own_bytes
  - a_clear_takes_the_style_edit
  - clearing_a_cell_that_is_not_there_changes_nothing
  - several_edits_in_one_row_and_in_several_rows_go_in_one_write
  clearing_a_cell_that_is_not_there_changes_nothing already shows the arm
  failing in an existing row and outside any row. So the mutation does not
  show that ruling 8's case adds detection the suite lacked. What P1's check
  2 said is not in any primary.

Summary:
- Q1: yes for the diff itself. The run log's use of expat instead of
  ElementTree is an undeclared departure from "P2".
- Q2: no. run.log does not print its script. `descriptor`-only flags on
  changed entries cannot be confirmed from the logs. `dated_xf_ok` is weaker
  than the spec's byte form. negative.log's header claim about P2 adding no
  message is not what its code measures. The only scan log is a RESULT:
  FAIL pre-answer run, as designed.
- Q3: no. One report claim is contradicted by run.log, and "package.rs:1389"
  and several attempt details appear in no primary.
