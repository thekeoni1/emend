# Independent check 1, M2 part 2 P2

- Checked commit (the phase commit): 09c941f04e921a51c34f05ad6371e3d2b30354e7
- HEAD when the check ran: 09c941f04e921a51c34f05ad6371e3d2b30354e7
- Checker: a fresh general-purpose subagent, given only the directory <scratch>/p2/check1 and the three questions of CLAUDE.md
- Answer: 14499 bytes, sha256 189cb19f28bf7901c25d57e6fed91409b8d2864f49e2728d069a96c680e316fe, committed below verbatim; it is plain ASCII and names no machine path, so nothing was replaced
- Inputs, each with its sha256 (sha256sum over <scratch>/p2/check1):

    62069c6edeac58ff9b48c4df7835c8f2dfdecf2b808bc09f0d482775108d7905  diff-09c941f.patch
    71947e269c5f797a967795c1bf6d9aceba3e4de3ad6845a7ecfe22e95297fb01  diff-24ce03b.patch
    9cfab4a4b261e8a30e1236746db8467ca5e40817fb9e917d39cb40884e898086  diff-4bd46a1.patch
    60dc5218c083f6bc70bd1b2aeb049834189d33e4481e1e64ea2e1c6f589820fd  diff-commands.txt
    2760024fe6427370ecceb6496fb98691947d1879f3199827614d30013f13b250  diff-doc-06d485b.patch
    427b78485e0dd9aee9f8de4ae78d4ad9338c84b590b96109093fafac37d8c6c6  diff-phase.patch
    777aafef9b0e7d0f563a4a23dbe7ee3b34d8c106318bdb690d73d646eaf8d746  log-gate.log
    bce2848f5e38eb4162075cb78e308033c4ecbd64f8601bf414ebdf4973c06fe7  log-negative.log
    1956afec62d0bab2ec8a61bfd11609b28ca5cd43cc42938df9292d18d819e2f6  log-run.log
    f3dd9e23a999e650e5d1f9dc4383847ab94f5f335e8f6d485334734abbdb3900  log-scan-pre-check.log
    1991ad376b71ac3231c221fe27a021ba89035fb3508b7302c846630460e12f06  log-test.log
    a75ef8b56f5618df4eb9b5d88ce159705831df378fb25dd7695f8473525395ec  record-m2p2-open-2026-09-25-09c941f.md
    ae0580212410d70ece63ff8f4a1767606bdc8d033603494d6a57a59adc71a9b1  record-m2p2-open-2026-09-25-09c941f.txt
    fa6632370ab0a84093b86933980aa69f4727844cd1cea537d617fdc6610dda09  record-m2p2-p2-2026-09-25-09c941f.md
    a64e224eacc622f1efd4318b3d8efda1036d4cbfe8e7a2d4bac2bb5688475f7c  record-m2p2-p2-2026-09-25-09c941f.txt
    2bcbfcbd4265ff6ea31af1b895dc25b9a2238aaff0fcb80553c39fddcff086d0  report-draft.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    51672b8eab6c104a517a609dab77a1d65d186335a6f52165f96eb6d9e183b97c  specs-0017-fuller-styles.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-gate.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-negative.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-run.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-pre-check.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-test.txt

---

# Independent check 1, M2 part 2 P2 (phase commit 09c941f)

The harness put a CLAUDE.md and a user memory index into my context; I did
not use either as evidence. Everything below is from the files in
<scratch>/p2/check1. I changed none of them. To compare the doc commit's
added text with the spec, I wrote two temporary files in <scratch>/p2 and
deleted them afterwards. I ran no cargo, git or build.

## Summary

- Q1: Yes, with remarks. The four commits do what P2 and the named rulings
  ask, and I found nothing outside that scope. Rulings 4, 5 and 10 are in
  the code. The one departure from the spec's words (two P1 tests edited)
  is disclosed and needs planning's ruling (F1).
- Q2: Yes, with small defects in script comments (F7, F8) and two gaps in
  the evidence the controls give (F9, F10). No required line is missing.
- Q3: Two report items are not in a primary as written (F12, F13), one
  count's wording is inaccurate (F14), and one claim rests on a primary
  that is not in this directory (F15). Every other number and quoted string
  appears in a primary.

## Q1: the diff against the spec

What I read: diff-doc-06d485b.patch, diff-24ce03b.patch,
diff-4bd46a1.patch, diff-09c941f.patch and diff-phase.patch, all in full.

- Doc commit (diff-doc-06d485b.patch). It touches ROADMAP.md (the P1 row
  set to DONE and the P2 row to READY) and appends to
  specs/0017-fuller-styles.md. The appended text is byte for byte lines
  701 to 865 of the spec in this directory (checked with diff). gate.log
  lines 1178-1206 show both blobs at the sha256 planning's message gave,
  at the doc commit and at HEAD. Nothing else is in it.
- Code commit 1, 24ce03b (diff-24ce03b.patch). It holds rulings 4 and 5
  and nothing else:
  - Ruling 4: styles::attribute_text (quick_xml escape, then the
    references &#9; &#10; &#13;) is split out of num_fmt_element and used
    for Font::name. The test a_font_name_takes_the_number_format_codes_path
    expects `&#9;`.
  - Ruling 5: the package.rs limits bullet is reworded ("grown or not"),
    and the test
    a_rewrite_adding_only_a_number_format_corrects_a_disagreeing_count
    covers fonts count 9 corrected to 1, a fills count that agrees kept,
    and a borders with no count kept.
  - It is the first code commit, so these land "before any P2 change" as
    ruling 11 asks. Rulings 7 and 8 are script rules and appear in the
    printed log scripts, not in this commit. I read ruling 11's "(the
    scripts)" as allowing that.
- Code commit 2, 4bd46a1 (diff-4bd46a1.patch). This is the P2 content
  replacement:
  - Style gains alignment and protection.
  - Xf gets two Range fields, filled by consume_subtree, which now returns
    the spans of the first main-namespace alignment and protection direct
    children.
  - derived_xf splits ids from flags and adds applyAlignment and
    applyProtection in the spec's order. The content rule is: a replaced
    child keeps its place; a new alignment goes at the start; a new
    protection goes after the base's alignment, else at the start after
    any new alignment; every other byte is the base's.
  - NewXf content becomes a Composed, so new children take the cellXfs
    prefix.
  - add_style composes the alignment (the only refusal is the rotation)
    and the protection.
  - Ruling 10: the four `expect(dead_code)` attributes (Horizontal::keyword,
    Vertical::keyword, Alignment::compose, Protection::compose) are
    removed, which covers every one ruling 10 names. gate.log's clippy PASS
    (line 1520 area) also rules out an expectation that is no longer met.
  - The supporting changes are all needed by the above: Composed::new made
    pub(crate), Composed::raw/append/is_empty added, Changes losing `Copy`
    because Composed is not Copy, and the header sentences that said "no
    alignment or protection" corrected.
  - Tests: the seven content cases the P2 list names, the no-base form,
    the prefix, and the rotation refusal recording nothing.
- Code commit 3, 09c941f (diff-09c941f.patch). This adds --border-sample
  as "Harness runs" describes it:
  - B1 is four thin sides in Theme 1 with no diagonal. G1 is center/center,
    wrap, indent 1. The protection is locked false, hidden false. S2 is B1
    plus a right alignment.
  - The same cells as --font-sample.
  - The line keys are the spec's. xfs_after and cells_after are asserted,
    and the output is removed on a difference.
  - read_back compares border_id with Some(B).
  - Tests cover the line and the parts, the refusal without borders, and
    three usage vectors. Nothing else.
- Phase diff (diff-phase.patch). I compared the multiset of +/- lines in
  the phase diff with the union of the four per-commit diffs. They are
  identical except for one commit-message line that begins with "-" in
  diff-09c941f.patch. The sha256 of diff-phase.patch is the combined-diff
  hash gate.log prints (427b7848...). The phase touches 9 files: no
  manifest, no lock and no gate file.

Findings for Q1:

- F1 (remark; a departure for planning's ruling). The spec's P2 test list
  says "the part 5 and P1 tests unedited". Two P1 tests
  (a_derived_xf_from_a_base_with_every_id_replaces_them_in_place and
  a_derived_xf_with_no_base_names_every_id_and_flags_what_is_given) gain
  `..Style::default()` (diff-4bd46a1.patch, hunks at package/tests.rs 6873
  and 6925). This is needed for them to compile once Style has two more
  fields. It is disclosed (report decision 1) and pinned by gate.log lines
  1502-1507. Planning's ruling 3 foresaw literals written with
  `..Default::default()`; these two P1 literals were not written that way.
- F2 (remark). The refusal order is now base, format, font, fill, border,
  alignment (the rotation). This extends the order ruling 10 accepted.
  a_rotation_out_of_range_refuses_the_addition_and_records_nothing pins
  that a font given before a bad rotation records nothing. The report does
  not state the extended order.
- F3 (remark, about code I cannot see). consume_subtree now calls
  scope.resolve_element(...)? on every direct child of a consumed element
  when bytes are being kept (diff-4bd46a1.patch line 658). Before this
  phase, only scope.start was called there. If resolve_element can fail
  where scope.start does not (for example on an unbound prefix), the reader
  now has a new refusal path. The diffs do not show resolve_element's
  behaviour, and no test names this case. The corpus run (27 of 27 reads)
  shows that no corpus part hits it.
- F4 (remark, style). diff-4bd46a1.patch line 93 is a 117-character doc
  line in Package::add_style's error list (a text rotation above 180 ...).
  rustfmt does not wrap comments, so the gate passes, but the line is not
  wrapped like its neighbours.
- F5 (gap in evidence). The P2 test list also names these composer tests: a
  border with every side and both diagonals, with one side, and with none;
  an alignment with every field and with none; a protection with each
  field; the rewrite growing borders. They are P1 tests, and test.log shows
  them by name and passing
  (a_border_is_written_with_its_five_sides_always,
  an_alignment_is_written_with_every_field_or_none,
  a_protection_is_written_with_each_field, the_rewrite_grows_borders).
  Their bodies are outside the diffs, so I cannot confirm they cover every
  sub-case the list names.
- F6 (remark). The tool's read_back compares a cell's style with the index
  add_style returned, not with X and X + 1 as the spec states them. The run
  log's comparison checks s = X and s = X + 1 independently (bordercmp
  changed_as_styled and added_as_edited), so the end-to-end claim still
  holds.

## Q2: the logs against the required lines

These are present, each with a control where the rules ask for one:

- The three shape greps (CHAIN, BANG, ASSIGN) over each script, each with
  a fixture control (all five logs).
- The phase commit given by argument, printed beside HEAD in the "about:"
  line, and compared, with a null-commit control and a short-sha control
  (gate, test, scan) and a not_head control (run).
- Root lines for every placeholder each log prints. I checked each log's
  "<word>/" tokens against its root lines: gate, test, negative, run and
  scan are all covered. Every <scratch>/ use is <scratch>/p2 or
  <scratch>/w, and each of those has its own root line.
- Ruling 7: `exec 2>"$ERR"` at the start of every script (gate line 132,
  test 94, negative 93, run 95, scan 110). Each log's last block counts 0
  bytes and has a one-byte copy control. The five stderr-*.txt files are
  0 bytes.
- Ruling 8: gate.log's test_bodies uses `git cat-file -e` for the commit
  and the path. A missing commit exits non-zero, and a failed `git show`
  raises. There is a control for a non-existent commit (gate.log lines
  847-848).
- The gate files are listed against the diff, and there is no self-test.
- The commit-message form is checked.
- Public fns: 89 at both ends.
- Base tests edited only on the disclosed six lines.
- Nine PASS lines.
- test.log: both targets 633/0/2; 14 names added and 0 removed; the added
  tests passed on both targets; the keyword table has 54 lines against 54
  rows.
- negative.log: B1 is the spec's case (exit 2, the collection message, no
  output). B0 and B2 are controls. There are two usage shapes. It runs
  through the gate's binary, checked by hash, in a detached worktree.
- run.log: every check "Harness runs" lists for P2:
  - bytes before <borders, between </borders> and <cellXfs, and after
    </cellXfs>;
  - the borders prefix, one B1 element, count +1;
  - the xfs prefix, two xfs;
  - the first xf's attributes, and its content by ElementTree and by
    bytes;
  - the second xf byte for byte;
  - count +2;
  - the worksheet as P1;
  - read_back = styled + 1;
  - totals 1046 and 27;
  - entry rows: the picked sheet and styles changed, descriptor flag only,
    the rest identical and unflagged;
  - lo and lo_in ok, order_preserved, files_identical no;
  - 18 plants of 18;
  - host and i686 byte for byte the same;
  - the record checked against the run directory;
  - the keep compared with P1's record;
  - the edit open check over the host keep.
- scan (pre-check): control-byte, non-ASCII and U+2014 forms with
  controls; the pre-P6d bare-shape control; the three shape greps over
  every printed script; the placeholder check; RESULT: PASS.

I also ran the three shape patterns myself over all five logs. For all
three forms and all five logs the count was 0.

- F7 (defect, minor, authored text in a log). log-gate.log line 92 says
  P1's gate script is "printed in records/logs/m2p2-p2/gate.log". P1's
  script is in records/logs/m2p2-p1/gate.log; the P2 log cannot hold P1's
  change list. log-scan-pre-check.log line 91 has the same wrong directory
  for P1's scan script. The test, negative and run scripts name
  m2p2-p1 correctly (log-test.log 72, log-negative.log 73, log-run.log 73).
  This looks like the p1-to-p2 rename reaching a path it should not have.
- F8 (remark). log-gate.log lines 125-126 repeat `set -uo pipefail`. It
  does no harm.
- F9 (gap in evidence, a control). The control "a path absent at the base
  gives no test and no failure" (log-gate.log lines 845-846) tests only
  that stdout is empty. It ignores test_bodies' exit status, so it would
  also pass if the path-absent branch failed. The "no failure" half is not
  measured by this control. The main loop's tb_fail does record status.
- F10 (gap in evidence, a control). first_xf_content is the AND of an
  ElementTree child comparison and a byte containment test (log-run.log
  lines 2295-2297). Its only plant, changing locked="0" to "1", breaks
  both parts at once. No plant shows that the ElementTree part (base
  children kept, order) turns the flag on its own. The spec asks for a
  control per check, and this one is covered only as a whole.
- F11 (remark). For P1, "Harness runs" asks the log to build the expected
  bytes from the constants and print them once. In run.log the B1, G1 and
  P1 constants appear only inside the printed scripts (line 378, and
  bordercmp.py at 1826). No output line prints the expected elements. I
  count the printed source as the printing, but planning may want it
  said.

## Q3: the report's numbers and quoted strings

These appear in a primary:

- the full doc sha and the short shas;
- 633, 0 failed, 2 ignored, 14, 54 and 54 (test.log);
- nine PASS and "checks all as expected: yes" (gate.log line 1566, as part
  of "gate log checks all as expected: yes");
- the six added lines (gate.log lines 1502-1507, six lines printed; the
  word "six" is derived);
- 27 of 27, 1046, 27, 18 of 18, 220, 210 and "no output refused on an
  accepted input" (run.log);
- 0 hits and 0 structural failures ("total hits: 0" and "structural checks
  failed: 0", scan);
- "entry rows as P1 expects" (run.log);
- `..Style::default()`;
- "Facts planning checked" (the spec heading);
- the record file names;
- ruling numbers 3, 4, 5, 7, 8 and 10.

Findings for Q3:

- F12 (not in a primary as written). Report line 20 says "all attempt 1".
  No log prints "attempt 1". Each log prints no "rerun:" line, and the
  rerun block's RERUN_FILES branch prints nothing, which supports the
  claim, but the words are the report's.
- F13 (not in a primary). Report line 62 says the run-log summary text is
  "carried from P1's script". P1's run log is not in this directory, so
  this cannot be verified here.
- F14 (inaccurate wording). Report line 28 calls four of the six added
  lines "the four P2 usage-case lines". gate.log lines 1504-1507 show one
  comment line ("// And with M2 part 2 P2's.") and three usage vectors.
  The count of six is right; "four usage-case lines" is not.
- F15 (claim resting partly on a primary not here). Report lines 64-66
  say "every corpus part agrees", citing specs/0017 "Facts planning
  checked". That section states count = children for fonts, fills and
  borders (27 of 27 each). It says nothing about numFmts or cellXfs, which
  it leaves to specs/0012, not in this directory. run.log does measure
  cellXfs and borders (counts_in_matched=yes on every workbook, both
  labels), and head_same=yes on 27 of 27 on both labels shows that nothing
  before <borders changed, which covers numFmts, fonts and fills
  indirectly. The claim is supported by run.log, but not by the source the
  report cites.
- The report's scan item says "final stage after the check answer". That
  is a plan, and no final-stage log exists yet.
