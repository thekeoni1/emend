# Independent check 1, M2 part 2 P1

- Checked commit (the phase commit): cc818102b3bdd85b58af859249751b0d98349945
- HEAD when the check ran: cc818102b3bdd85b58af859249751b0d98349945
- Checker: a fresh general-purpose subagent, given only the directory <scratch>/p1/check1 and the three questions of CLAUDE.md
- Answer: 17713 bytes, sha256 f613b6560f67ec82fb5886cfb01cbfae6edf60c956a1aa370694ce3d70e5d4b3, committed below verbatim; it is plain ASCII and names no machine path, so nothing was replaced
- Inputs, each with its sha256 (sha256sum over <scratch>/p1/check1):

    54c3b8c94525489d26ab9a1c6c05c8102d5a60d0baf557477405fff0253949d0  diff-05cc43a.patch
    fc3711613640e30bfb9c143153103911523c3195d136a71e5fdf3c9759d242ba  diff-cc81810.patch
    191dc425f2217fd23ddc4371489094131321ee088a7d206f29d54c7089094652  diff-commands.txt
    9db5c2674b77699d9069679f30222a118d9c61796f60268b1a89ad1ec59ef98c  diff-doc-b3152ca.patch
    5ba73476d6852a347265f610a65081b61f7e057f618f9613bfc8d20f02a48794  diff-f14885f.patch
    abc5d613b9702aa569f89ab7e6a5d9af0a69d5be966761228c831d28a1452441  diff-phase.patch
    e7f548b70c7fbd8f22d1232509d148af2883735fddafe71d5efcd78262f9bf62  log-gate.log
    3cc8d00093d818139fc49779734a8f238cc92c126a87d5ad389b39ca3e7bed86  log-negative.log
    509af36fd180e473e2f029e135351889586a9281c15c56803ee7fe039a4b1c2b  log-run.log
    d26465c844245d2bbd4eca145107e5a5d3fd82d3ec6ea36b2ee70822522b36c3  log-scan-pre-check.log
    332177a9c45f17685aac9b989a1fc909a278e1c9d8f5093dd21b5de7b933f5c2  log-test.log
    d7d885c44f93acd7c8688be3ac28ec7986d025b7ef157d013b659356af111287  record-m2p2-open-2026-09-25-cc81810.md
    4c5fc231cc46cf90831b4ffbe75b3f07685d4f14706d8ee552829fbd81b402ab  record-m2p2-open-2026-09-25-cc81810.txt
    245c7eb258aae6ced83769487d54520e2c4ffd94ff23abf23572c7778e0c9dda  record-m2p2-p1-2026-09-25-cc81810.md
    056f41ebc9dcfb00b1d24add41bedc545c0bdc1719ee6f278e2e84d015a35763  record-m2p2-p1-2026-09-25-cc81810.txt
    cb2b3b7db7fc44e01552837e3c0b1431cc473968ea81d33723a22ea7887f4c2e  report-draft.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    8384b41b66cb44488275086c89e42f8c41d4d68a4ae885cce9c53607e6500643  specs-0017-fuller-styles.md
    0f5b9068072ae7cf158d8fa8bc378836730956f4cc5a3978cbcedcbbaf44e22c  superseded-gate-attempt-1.err
    16930ee644c0473285a4eccf8212988320b66631fde342114cdc1b9eb1f4ca97  superseded-gate-attempt-1.log
    a44425282a39e47b0d772cb23a36895e3d19d5d13e7ca3cd2819826fcafe4190  superseded-scan-pre-check-attempt-1.log

---

# Independent check 1: M2 part 2 P1 (phase commit cc81810)

The harness put the project's CLAUDE.md and an auto-memory index into my
context; I did not use either as evidence. Everything below comes from the
files in <scratch>/p1/check1 only. I changed none of them, and I ran no
cargo, no git and no build.

Method, in brief:
- I read the spec's P1 section, rulings 1 to 5, "Design", "Harness runs"
  and "Phases", plus specs-0001's "How this task runs" and "Rules for
  every phase".
- I read all four per-commit diffs in full, and the combined diff's
  file list and index lines.
- I hashed every file with sha256sum. Stripping each per-commit patch's
  message header and hashing the rest reproduces gate.log's four per-commit
  diff hashes (04b572d7..., 354ac4a5..., 2c798811..., 14008d2c...).
  diff-phase.patch hashes to abc5d613..., which is gate.log's combined-diff
  hash. The per-commit index lines match the combined diff's index lines
  file for file, and no file is touched by two commits. So the combined
  diff is exactly the union of the four commits.
- specs-0017-fuller-styles.md hashes to 8384b41b..., the hash gate.log and
  run.log give for planning's uncommitted spec. The three superseded files
  hash to the values the two rerun headers print (16930ee6...,
  0f5b9068..., a4442528...).

## Question 1: does the diff do what P1 (with rulings 1 to 5) says, and nothing else?

In substance, yes. Here is what I confirmed, and which diff I read for it:

- Doc commit (diff-doc-b3152ca.patch). It touches only ROADMAP.md,
  specs/0016 and specs/0017. gate.log checks all three against planning's
  hashes, at the doc commit and at HEAD. Nothing else: confirmed from
  diff-doc-b3152ca.patch.
- First code commit (diff-05cc43a.patch). It is the archive.rs header
  sentence alone, and it comes before any other code change. Nothing else:
  confirmed from diff-05cc43a.patch.
- Second code commit (diff-f14885f.patch), the core of the phase:
  - The three counts and their accessors, with a second fonts, fills or
    borders refused under the widened message.
  - Style (no alignment or protection field, per ruling 3), Font, Color,
    Fill, Pattern (19 variants), Border, Side, BorderStyle (13),
    Alignment, Horizontal (8), Vertical (5), Protection, Underline,
    VerticalAlign and Scheme, all with composers.
  - add_style, with add_cell_style defined over it.
  - Equality lookup for fonts, fills and borders. Border is wired end to
    end (ruling 3).
  - The five-container rewrite, and the rewrite's refusal for a
    collection gone at write time using the new string (ruling 1).
  - The three new messages, catalogued.
  - The two part 5 tests change only their expected string (ruling 2),
    and the catalogue set test reads elements.rs as a fifth source
    (ruling 5).
  - The P1 test list is covered test for test. The 31 tests added in this
    commit map onto every item of that list and onto ruling 3's two
    border cases.
  - Extras that serve the above: number_text and STYLE_OUT_OF_RANGE become
    pub(crate), a one-character borrow change (`*have == code`), and the
    module-header corrections the report names in its item 8.
  - I found nothing beyond P1 and those items (read in diff-f14885f.patch).
- Third code commit (diff-cc81810.patch). It adds --font-sample, whose
  constants, line fields, order, asserts and read_back rule match "Harness
  runs". It also adds 3 tests and 3 usage cases. Nothing else: confirmed
  from diff-cc81810.patch.

Findings:

F1 (defect, needs planning's ruling).
- Where: diff-f14885f.patch line 2222 (styles/elements.rs, Font::compose).
- What the code does: it escapes the font name with
  quick_xml::escape::escape.
- What the spec says: "Design", "The elements", font bullet (spec line
  352), says the name is "escaped by the crate's `escape`". "Facts
  planning checked" (spec lines 179-180) names the same function as the
  one a text goes through.
- Why the two differ: the report's item 6 (report-draft.md line 63) says
  the crate's path writes a tab or line feed in a numFmt code as a
  character reference, but the font name is not treated that way. So the
  function used is not the crate's `escape`.
- Consequence: a font name that holds a tab is accepted by the test
  (diff-f14885f.patch line 2630, `a_font_name_empty_or_holding_a_forbidden_character_is_refused`)
  and written raw. The report's own item 6 says it would then read back
  as a space.
- The report says this was done "as the spec says". The spec text does
  not support that. Either the code changes, or planning rules that
  quick_xml's escape is acceptable.

F2 (remark).
- Where: diff-f14885f.patch line 122 (package.rs "Limits" bullet).
- What the doc says: "A numFmts, fonts, fills, borders or cellXfs the
  rewrite grows and whose count disagreed ... comes out corrected".
- What the code does: rewrite_styles passes all five containers through
  `counted` on every styles rewrite, grown or not. So a number-format-only
  edit (add_cell_style) now also corrects a wrong `count` on fonts, fills
  or borders, which part 5 copied raw.
- This is what "Module `package`" asks for ("a count that disagreed with
  the children is corrected, as today"), and no corpus part has a wrong
  count. But the doc wording reads as narrower than the code.
- No test pins the no-addition, wrong-count case for fonts, fills or
  borders. The test `an_empty_container_opens_up_...` corrects fills,
  which does gain an addition.

F3 (remark).
- Where: diff-f14885f.patch line 83.
- The new catalogue paragraph types a count ("The fuller cell styles add
  three").
- Spec "Errors" says: "The catalogue count in the module header is not
  typed". The existing paragraphs use the same "add five" form, so this
  may be the accepted style. Planning should say whether the sentence
  covers per-section counts.

F4 (remark).
- Where: diff-f14885f.patch lines 1771-1772.
- Report item 3 (report-draft.md line 56) says Eq is derived "on the
  enums". Color is an enum and derives no Eq (it holds an f64). The code
  is fine; the report sentence is inaccurate.

F5 (gap in evidence).
- Report item 5 (report-draft.md line 61) says "A container with no count
  attribute stays without one at the rewrite". `counted` is not in any
  diff here, and no test added by this phase covers a grown fonts, fills
  or borders without a count attribute.
- The item also says "the count-corrected test uses count=\"0\"". In that
  test (`an_empty_container_opens_up_a_wrong_count_is_corrected_and_order_is_not_assumed`)
  the count that is corrected is fills' count="7". The count="0"
  containers are the empty fonts, which is opened up, and borders, which
  is kept.
- I could not confirm the claim from any file here.

F6 (gap in evidence).
- The reader's numFmt and xf branches (styles.rs, Styles::read) are
  unchanged context outside the diff hunks.
- With `open` now able to be Fonts, Fills or Borders, I cannot see from
  diff-f14885f.patch whether those branches check which container is
  open. If they only check that some container is open, a stray `numFmt`
  or `xf` inside `<fonts>` would be read.
- The test `the_counts_are_the_children_not_the_count_attribute` does not
  cover that case. Low risk, but I cannot confirm it.

## Question 2: do the logs contain every line the spec and specs-0001 require?

Present and verified:

- First lines. Every log opens with "command: git rev-parse HEAD", then
  an "about:" line that names the phase commit and HEAD at run.
- Phase commit given and compared.
  - Each log receives the phase commit as an argument (the invocation
    line).
  - Each compares it with HEAD and prints both values, and each has a
    control that refuses a fabricated commit.
  - Where: gate lines 42-47, test "the phase commit given..." block,
    negative lines 48-50, run lines 22-26, scan lines 19-23.
- Shape greps with controls. The three shape greps run over each log's own
  script, with a fixture control per shape: gate, test, negative, run and
  scan all do this.
  - My own count of the three forbidden shapes over all five logs, both
    superseded logs and the four record files is 0 in every file.
- Non-ASCII and control bytes. My count is 0 in every file of the
  directory.
- Absolute paths. No home or temporary path appears except inside built
  control fixtures (run.log lines 1504 and 1509, scan line 531).
- Gate files. gate.log lists all ten gate files against the diff and
  finds 0 in it, so no self-test is required.
- Commit messages. gate.log checks that every message is ASCII and ends
  with the Co-Authored-By line only.
- Tests. test.log shows host and i686 each at passed 619, failed 0,
  ignored 2. The per-binary breakdown sums to 619. It lists 34 added test
  names and 0 removed, each passing on both targets, and the source-side
  count (34) agrees.
- Keyword table. It is printed at test.log lines 2302-2355: 54 lines
  against 54 pinned rows, 0 misspelled, with a control.
- Negative log.
  - F1 (the template without fonts) exits 2, prints the exact collection
    message on stderr, writes nothing to stdout and creates no output file
    (ruling 4). F2 and F3 are extra cases, and F0 is the control.
  - The binary is the gate's, hash-checked against gate.log (whose hash
    e7f548b7... equals the file here), and run from a detached worktree.
- Run log, against the checks "Harness runs" lists for P1:
  - The four byte-span checks: head_same, fonts_fills_gap, fills_xfs_gap,
    tail_same.
  - fonts_prefix, font_added and fonts_count_grew, and the same three for
    fills.
  - xfs_prefix, xfs_added, first_xf_ok, first_xf_content, second_xf_ok
    and xf_count_grew.
  - The worksheet as specs/0012 P2 compares it: ws_prefix_same,
    ws_suffix_same, changed_as_styled, added_as_edited and
    nothing_removed.
  - read_back equal to styled + 1 (line_as_spec, with controls).
  - Totals xfs_after 1046, fonts_added 27 and fills_added 27, on both
    targets.
  - lo and lo_in ok in all 27.
  - Styles part and first worksheet changed. Every other entry is
    identical, and the only flag is descriptor, on changed entries
    (entries_p1).
  - order_preserved yes and files_identical no in all 27.
  - 22 planted-flag controls, each turning its own flag.
  - Outputs byte-identical between host and i686.
  - The keep and the edit open check: 210 pairs, 201 unchanged, 9 new,
    no verdict moved.
- Rerun headers.
  - gate.log line 49 names the superseded attempt and hashes its log,
    stderr and script.
  - The scan pre-check's line 42 does the same for its log and script.
  - The hashes of the log and stderr files match the superseded files
    here.

Findings:

F7 (defect, needs planning's reading).
- The rule: "Phases" carries "every placeholder a log prints has its own
  root line".
- Where it is broken: negative.log line 39 declares <tree> inside the
  <build> root line, not on a root line of its own.
- What happened: scan pre-check attempt 1 flagged exactly this as a
  structural failure. Attempt 2 did not fix the log. It relaxed the check
  so that a declaration anywhere in a root line counts (scan header line
  42, "Changed: a declaration is '<word> stands for' anywhere in a root
  line").
- The change is named, so this is not a silent re-run until pass. But it
  is a loosened check, not a repaired log, and the rule's wording ("its
  own root line") is not what attempt 2 tests.
- Report item 12 calls this the implementer's "operational reading". It
  needs planning's ruling.

F8 (gap in evidence).
- Attempt 1 of gate.log failed in a way the log could not show: a
  traceback on the script's stderr, which the log does not hold.
- Attempt 2 fixed that one helper, but still records no stderr for its
  own script. The only "stderr bytes: 0" is for check.sh (gate line 1605).
  test, negative, run and scan do not record their scripts' stderr
  either.
- So nothing in any log shows that attempt 2 (or any other log) ran with
  an empty stderr. The failure mode that forced the rerun is still
  unobservable.

F9 (defect, minor).
- Where: gate.log line 795, the new removed_in.
- The rerun header (line 49) says removed_in "reads a path absent at the
  base as no lines". The code maps every non-zero exit from `git show`,
  for any reason, to an empty file.
- With removed lines present, an empty old file then makes `owner` index
  past the list. That crashes to stderr again, which F8 says is not
  logged.
- The ruled checks would still fail for the three named test files. For
  any other test file, a crash would print no removed line, and the
  "every removed line lies in a ruled test" check would pass vacuously.
- No control distinguishes "absent at base" from "git show failed".

F10 (remark).
- "Harness runs" P1 says the run log "builds the expected bytes itself
  from the constants and prints them once".
- The F1 and L1 bytes appear only as literals inside the printed scripts,
  twice: run.log line 398 in the runner's script and line 1823 in the
  printed fontcmp.py.
- There is no output line that prints them. The intent (the expected
  element is visible in the log and independent of the crate) is met; the
  letter "once" is not.

F11 (remark).
- superseded-gate-attempt-1.err line 3 holds an absolute system path
  (a system library directory, in Python's traceback).
- The scan pre-check scans the four sibling logs and the records, but not
  the superseded attempts.
- If the superseded files are committed with the logs, the final scan
  should cover them. Line 3 would then be an absolute machine path in a
  committed file. No log says where the superseded attempts are kept or
  whether they will be committed.

F12 (gap in evidence).
- The two superseded scripts (m2p2-p1-gate-attempt-1.sh, hashed 734ff9c8...,
  and m2p2-p1-scan-pre-check-attempt-1.sh, hashed b38c5dac...) are not in
  this directory, so I cannot verify those two hashes.
- The superseded logs do print their scripts. gate.log attempt 1's
  printed removed_in shows `check=True` (lines 711-712 of its script
  block), which is consistent with the header's account.

F13 (remark).
- negative.log's check lines say "stderr exactly the collection message".
  The captured stderr is "error: " followed by the message.
- The check evidently compares the prefixed line. The wording is loose,
  not wrong in effect.

F14 (remark, not a defect).
- There is no final scan.log yet, only the pre-check stage (attempt 2,
  RESULT: PASS, 0 hits, 0 structural failures). This is expected, since
  specs-0001 puts the final scan after the check answers.
- The final stage must list and hash this answer, and should include
  the superseded files (see F11).

## Question 3: does every number and quoted string in report-draft.md appear in a primary?

These appear in a primary:
- all four full commit ids and e81bf5b;
- "nine PASS";
- the quoted phrase "checks all as expected: yes" (gate.log's last line
  is "gate log checks all as expected: yes");
- attempt 1 ending yes (superseded-gate-attempt-1.log, last line);
- the traceback (superseded-gate-attempt-1.err);
- 619, 0 and 2 on both targets; 34 added and none removed; 54 and 54;
- F0 to F3 with exit 2 and the quoted message; two usage shapes;
- 27 of 27; xfs_after 1046, fonts_added 27, fills_added 27;
- 22 of 22 plants; byte-identical host and i686 outputs;
- 210, 201 and 9; no verdict moved; no output refused on an accepted
  input;
- pre-check attempt 2 with 0 hits, and attempt 1's two structural failures
  (superseded scan log lines 1610 and 1612);
- the record names;
- `Composed`, `#[cfg_attr(not(test), expect(dead_code))]`, `count="0"`
  and '<word> stands for'.

These do not appear in any primary, or are contradicted by one:

1. Line 6, "nothing pushed; e81bf5b ... is also local". No log prints
   remote or push state.
2. Line 55, "(expect fails once they are live)". This is a counterfactual
   about the compiler or clippy. No log measures it.
3. Line 56, "Eq on the enums". Contradicted: Color derives no Eq
   (diff-f14885f.patch line 1771). See F4.
4. Line 61, "A container with no count attribute stays without one at the
   rewrite". No primary shows it; `counted` is not in any diff. See F5.
5. Line 62, "the count-corrected test uses `count=\"0\"`". The corrected
   count in that test is count="7" (see F5).
6. Line 63, "as the spec says". Contradicted: the spec says "the crate's
   `escape`" (spec line 352). See F1.
7. Lines 64-66, "the numFmt code is" (written as a character reference)
   and "it would read back as a space". Neither appears in any primary.
   The crate's escape function is not in the diffs, and no log measures
   the read-back.
8. Line 72, "(clippy type_complexity)". This string appears in no primary.
   gate.log shows only "PASS clippy".
9. Line 76, "specs/0016 P5 verification ruling 3". This matches the scan
   log (line 1603). However, specs/0017 "Phases" cites the placeholder
   rule as specs/0016's close-out ruling 4. I cannot see specs/0016, so I
   cannot say which citation is right. Planning should reconcile them.

Summary:
- The diff implements P1 and rulings 1 to 5. The one substantive
  departure is F1: the font name is escaped with quick_xml rather than the
  crate's `escape`.
- The logs carry the required lines, with three open points:
  - F7: <tree> has no root line of its own; the scan check was relaxed
    instead of the log being fixed.
  - F8 and F9: no log records its own script's stderr, and the
    removed_in fix is broader than its header says.
  - F11: the superseded err file holds an absolute path, and its commit
    status is unstated.
- The report has nine sentences a primary does not support or
  contradicts, listed above.
