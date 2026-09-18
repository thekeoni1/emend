Milestone 1 part 2 P2 (specs/0009-package-layer.md) independent check 1
checked commit: 1334830933799ccfd119423c44605f650447fda0
HEAD when the check ran: 1334830933799ccfd119423c44605f650447fda0

Inputs, each as given to the checker, with the sha256 it had then. CLAUDE.md
gives the checker the spec, the diff, the draft report and the gate and scan
logs; both checks went further and read the other three logs and the record
pair as well, so all of them are listed.
  specs/0009-package-layer.md at 1334830       22784ad21159d97a2e88ec27e452fa125ac09fd687542239ce53258dab90bca6
  specs/0001-bootstrap.md at 1334830           98ae150c8a63bfc7d8e7073cf82f0ce98bfb7bbd3feee323c7fb5f68219fa16b
  phase diff d6fa91d..1334830                  db775e99a4f2e3cd3f6443713310c990157a3b1e4a89e0c53fdc501619f2fc51
  records/logs/m1p2-p2/gate.log                82ac193b57f38b2fe87a9beb3cdeb8ebf4935f6cc650a1ba7da9ab897870210a  SUPERSEDED
  records/logs/m1p2-p2/test.log                dab229889f1a7ba5b270b60ce7b38b60d6daf1967091045b24eab6e64250e943
  records/logs/m1p2-p2/negative.log            6777b74b4b0e94687cb25c826244dbd30ce51653c5dab6e7121be45944ab9df6  SUPERSEDED
  records/logs/m1p2-p2/run.log                 1ef103f093ab1300ed2da65684e5938d6277832106f64855fcccc53b91b51146
  records/logs/m1p2-p2/scan.log                1ca1672a009241d8fdd42870e0e431cec689981c5de4b84b3ef124063a7fee31  SUPERSEDED
  records/m1p2-p2-2026-09-18-1334830.md        58f4d6263ed8cf60fb1692c5dc73208787e46bec865f18ce825022c8e692777f
  records/m1p2-p2-2026-09-18-1334830.txt       0d83de1c92c9fe1bc68ba766927d7afa720f1ed433c6d16b5c86b6ecc3dcc625

Three of those logs are superseded by the repairs these two answers forced,
and the phase now ends at the doc commit that carries planning's rulings on
their findings, c8b8fec, rather than at 1334830:
  gate.log     -> 0bee0d1bef65ca2ac834af817d33e769548c65cb9329efc0f084fd47458a4fb7
  negative.log -> 42be6da5e0f979550b6e7914b4237329a1f74940b463ea078099ce49bc2cc3c2
  scan.log     -> the final pass, written after this file, which scans it
Each rerun log's own note names the round it replaces by byte count and
sha256 and says what changed and why. specs/0009 P2 rulings 7 to 11 record
planning's decisions on every finding below.

The draft report reached the checker as text inside its brief rather than as
a file, so the exact bytes it read are not recoverable. It differed from the
report finally sent in the ways that report's own section on these two checks
lists, one of which (the zip layer sentence) had already been corrected in the
working copy before the answers arrived, which is why both answers quote it.
Two amendments were sent to both checkers by message while they worked, and
both answered on them.

The answer below this line is verbatim. Non-ASCII characters replaced: none;
the checker's file was already plain ASCII, which this session verified before
writing it here.
extracted answer: 19775 bytes, sha256 b0e2122485127fc132798cdcabdd88ce0ea3839c34bb88383baa92367f86f12e
---
Independent check, Milestone 1 part 2 P2 ("content types, relationships, the
workbook").

The harness put CLAUDE.md into my context automatically. I say so as asked and
I have not used it as evidence for anything: my sources are
specs/0009-package-layer.md, specs/0001-bootstrap.md, the phase diff
d6fa91d..1334830, the five logs under records/logs/m1p2-p2/, the committed
record pair records/m1p2-p2-2026-09-18-1334830.md and .txt, and the draft
report. I read the scan log that says "check answers scanned: 0", so I am
reading the earlier of its two runs, as the brief describes.

I re-ran two things for myself rather than trusting the logs' own numbers: the
two forbidden-shape greps over all five committed logs (0 in every file,
scan.log included, which scan.log itself cannot cover), and the survey
aggregation, recomputed from the 54 survey lines run.log prints (27 per label).

=====================================================================
1. Does the phase diff do what the spec says, and nothing else?

Yes, with four small documentation inaccuracies inside the diff and no
substantive departure I could find.

What the spec asks of P2 and what the diff delivers:

- Code commit 1, the move (82d4e77). I compared the 476 lines removed from
  crates/emend/src/archive/tests.rs against all 522 lines of
  crates/emend/src/testing.rs. Every difference is one of: the new module
  header, the ten zip-format constants declared from APPNOTE with a citation
  each, or a `pub(crate)` on an item that was private. No moved line of code
  changed. The move set is exactly ruling 1's list (crc32, deflate_stored,
  METHOD_STORE, METHOD_DEFLATE, Spec, Build, OutEntry, Parsed,
  parse_independently, names), and every item ruling 1 says stays is still in
  archive/tests.rs (round_trip, assert_identical, check_round_trip, open_err,
  bytes_replacement, apply, REPLACEMENT_LOCK, FailingWriter,
  replacement_temporaries, at_u16, at_u32, data_start_of) and absent from
  testing.rs. Ruling 1's "nothing in archive.rs becomes pub(crate) for the
  tests' sake" holds: the only change to crates/emend/src/archive.rs in the
  whole phase is the Error::Package variant, its Display arm and its
  catalogue lines (14 inserted lines, 0 deleted).

- Code commit 2 (5fc1b3d): package::{Package, ContentTypes, Default,
  Override, Relationships, Relationship, Workbook, Sheet, SheetKind},
  xml::Scope and --survey, all present. I read crates/emend/src/package.rs in
  full against "Module `package`" as amended: exact-byte part lookup;
  content_types/relationships/workbook parsed on first call and kept;
  Override before Default with the extension compared
  ASCII-case-insensitively; targets resolved with the leading slash stripped
  or joined to the source directory and dot segments resolved either way
  (ruling 5); duplicate ids, wrong roots, external targets and targets above
  the root each refused with the spec's message; the ten namespace and
  relationship-type constants as (transitional, strict) pairs; officeDocument
  counted across both forms (ruling 6) and "names no entry" when the resolved
  name is not an entry (ruling 3); the four workbook options taking the first
  of a kind; sheets read only from <sheets> at the root's child depth, with
  name normalized and sheetId a decimal u32.

- Code commit 3 (1334830) is ruling 4 and nothing else: the shared INCOMPLETE
  message, the three parse sites changed from "record nothing" to "refuse",
  one catalogue entry, and three tests (57 added test lines). Nothing else is
  touched.

- P3's work is absent, as required. No edit method (remove_relationship,
  add_relationship, remove_override, add_override, remove_part, replace_part,
  add_part), no Package::write_to, no TempFile change, no --drop-calc-chain,
  no probe-zip-xml-write or probe-emend-package-xml (tools/size-probe is not
  in the diff at all), no size log or size record.

- The two spec files and the ROADMAP row are the doc commits' work the P1
  verification ruling directed: the P1 DONE status, the new negated-condition
  rule in specs/0001-bootstrap.md, and the six P2 rulings with the Design
  amendments they name. The ROADMAP P2 row still reads READY, which is right
  at a phase's last code commit.

- The P2 test list is covered case by case in crates/emend/src/package/tests.rs
  (37 tests, plus 10 helpers): map_resolves; target_absolute, target_relative,
  target_parent; external_target; rels_part_absent; override_beats_default;
  extension_matched_case_insensitively; part_with_no_content_type;
  strict_namespaces; rel_prefix_bound_three_ways; sheet_name_escapes;
  sheet_states; several_sheets_in_order; and the errors, including
  root_wrong_in_types, root_wrong_in_rels, root_wrong_in_workbook,
  non_utf8_sheet_name, rel_id_prefix_unbound, sheet_relationship_missing,
  target_above_root, two_relationships_one_id, no_office_document,
  two_office_documents and open_reads_no_xml. The "chartsheet" case has no
  test of its own: it is covered inside strict_namespaces and
  several_sheets_in_order, which is coverage, not a gap, but the report's
  "every case in the spec's P2 list has a test" reads as one test per case.

Things in the diff the spec does not name, which I judge in scope rather than
creep, since P2's own survey keys require them: rels_part_source (the inverse
of rels_part_name, needed for the rels_parts key), ContentTypes::defaults and
overrides and Relationships::all and source (needed for the defaults,
overrides and relationships keys), Relationships::first_of (it implements the
spec's "first in document order, transitional before strict"), the
CONTENT_TYPES_PART/CONTENT_TYPES_NS/RELATIONSHIPS_NS constants, and the
re-export of Scope from lib.rs beside P1's Stream and Token.

Four documentation inaccuracies inside the diff:

(a) crates/emend/src/package/tests.rs's header says "The smallest package is
    five parts: `[Content_Types].xml`, `_rels/.rels`, `xl/workbook.xml`,
    `xl/_rels/workbook.xml.rels` and one worksheet", but smallest() returns
    six parts: it also writes `xl/styles.xml`, which is why
    open_reads_no_xml asserts `entries().len() == 6`. The sixth part is
    reasonable (the workbook's rels names styles.xml and the spec wants
    `styles` resolved); the sentence is stale.

(b) records/logs/m1p2-p2/gate.log's own script says, at the end of the
    comment that explains why no commit count is asserted: "What is asserted
    instead is the structure specs/0001 actually requires: every commit in the
    range is either a doc commit touching only planning docs or a code commit
    touching none, and no doc commit comes after a code commit." The final
    clause is not asserted and could not be: this phase's range is doc, doc,
    code, doc, doc, code, doc, code, and structure_sound only requires the
    FIRST commit to be a doc commit. The comment forty lines below says so
    correctly. The check is right; this one sentence contradicts it.

(c) records/logs/m1p2-p2/scan.log's second line reads "about: M1 part 2 P1
    phase diff d6fa91d3acac...1334830933799...". The commits are the right
    ones and the "phase:" line below says P2, but the phase label in the
    "about" line is wrong.

(d) The module header of crates/emend/src/package.rs writes the first
    catalogue entry as "no `[Content_Types].xml` part" while the message the
    code carries has no backticks. Cosmetic; the negative log compares the
    extracted string, not the header.

None of (a) to (d) changes behaviour.

=====================================================================
2. Do the logs contain every line the spec requires?

All five logs the spec names are present (gate.log, test.log, negative.log,
run.log, scan.log), each opens with the command that read it, the phase
commit and the HEAD at run, each prints its own script and the shared counting
library, each runs the counting controls, and each ends with one yes-or-no
line (scan.log ends "RESULT: PASS" rather than a yes/no line). specs/0001's
gate-file clause is satisfied: ten gate files listed, 0 in the diff. The
acceptance clauses of "Harness runs" for P2 are each asserted by name in
run.log and each is as-expected; I found no ": UNEXPECTED" line in any log.

One line the spec asks for that I could not find:

- P2 ruling 1 ends "The test count stays 178 on each target and `test.log`
  shows it." No log prints 178 anywhere (I grepped all five). test.log ran
  only at 1334830 and shows 217 on each target. The ruling's substance is
  established another way, by the move commit's diff being visibility-only,
  which I checked myself; but the line the ruling names is absent. What would
  settle it: a printed count at 82d4e77, or planning's word that the diff
  evidence discharges the clause.

On "a check that cannot fail", which the brief puts first. I looked at every
important check in each log for the branch that produced its line:

- run.log entry_counts. It indexes columns by header name, so a renamed
  column would make awk compare $0 with $0 and park xml_crc_differs and
  xml_size_differs at 0 whatever the data said. That exact hole is closed:
  COLUMNS_USED is printed, the real runner header is printed, and
  columns_resolve is asserted on the actual entries file for both labels, with
  a positive control and a negative control (the same header with crc_out
  renamed) that goes through isnot, plus a control showing `isnot true` reports
  UNEXPECTED. entry_counts itself is exercised on a six-row fixture whose one
  bad XML row and one flagged binary row make every class non-zero, and the
  printed fixture line equals the printed expectation. This form can fail and
  is shown to fail.

- run.log survey_counts. One awk pass, keys read by name, and a three-line
  fixture whose expected output has untyped=1, unresolved=1, case_only=1,
  sheets_unresolved=1, without_shared_strings=1, without_styles=1,
  with_calc_chain=1 and not_survey=1 for a space-separated line. So the form
  demonstrably reads each of the classes the acceptance rests on, and a
  space-separated or empty line cannot pass as a survey line. One residual,
  which I judge bounded rather than fatal: if the tool ever stopped printing
  the `untyped`, `unresolved` or `case_only` key, `v["key"] + 0` would be 0 and
  those three assertions would stay green. What bounds it is that the totals
  (relationships 878, external 234, resolved 644, sheets 95, rels_parts 216,
  overrides 614, entries equal to the diff's row count) would collapse to 0 if
  the key names moved, and that run.log prints all 27 survey lines verbatim
  per label, so the key set is visible in the primary. The only place the
  exact 17-key line is pinned as a string is the tool's unit test.

- scan.log's two shape greps. SHAPE='[$][(](yn|is|isnot) .*(&&|[|][|])' and
  BANG='[$][(](yn|is|isnot) [!]', both assembled at run time so the script
  cannot trip itself. Each has a positive control that scores 1, a negative
  control (a named function doing the same work with the negating helper) that
  scores 0, and a cross control on the other's fixture that scores 0. Both can
  fail. The tab lesson from the negative log's first round was applied: the
  survey shape in negative.log and the fixtures in run.log build a real tab
  with printf instead of writing backslash-t.

- Other forms I checked for the same failure mode: the counting library's
  forms all print "error" rather than 0 on an unreadable file and each has a
  control; negative.log's messages are extracted from package.rs and xml.rs at
  run time and each case's key is asserted to select exactly one message, with
  a control showing a key no message holds selects none, so an empty expected
  string cannot read as a pass; negative.log's usage text is read from the
  source through a Python reader that undoes the Rust string continuation, and
  is asserted non-empty; test.log's tool-test extraction has a control showing
  the same extraction over a non-existent binary yields 0 names; gate.log's
  five privacy forms each have a hit control and a clean control, and its
  structure_sound has five controls, one per way it can fail.

Corpus privacy. I found no cell content, no original file name and no corpus
path in any log. Workbooks appear as 12-hex ids; the record pair carries ids,
producer, tags and counts. The only three occurrences of "emend-testing" or a
"/home/" shape are inside run.log's printed script: the unexpanded default
`CORPUS=${EMEND_CORPUS_DIR:-$HOME/emend-testing}`, the pattern of the
record-privacy grep, and the fictional positive control
`/home/someone/emend-testing/corpus/x.xlsx`. P1 ruling 7 permits exactly
those; no expanded path appears anywhere, and run.log's own check reports 0
such lines in both halves of the record pair with its control at 1.

=====================================================================
3. Does the draft report's every number and quoted string appear in a
   primary?

Almost all of them do. I checked every figure and quotation, including the
amended paragraph. What I verified against a primary, in brief:

- The eight commits, their subjects and order, the base d6fa91d and the split
  into five doc and three code commits: gate.log and scan.log.
- 878 lines of package.rs, 1024 of package/tests.rs, 522 of testing.rs, 14
  added lines in archive.rs, 456 lines of main.rs, and "133 lines of a
  423-line module" for Scope: all exact against the diff (Scope begins at line
  291 of a 423-line xml.rs, so 133).
- 217 tests on each target, 37 package, 23 xml, 9 tool, 2 ignored and neither
  in the new modules: test.log. "178 at P1" is in the committed spec (the P1
  verification ruling).
- The survey aggregate block quoted in the report is byte-for-byte the line
  run.log prints for both labels, in the same key order. I recomputed every
  total from the 54 printed survey lines: entries 870, typed 843, defaults 110,
  overrides 614, rels_parts 216, relationships 878, external 234, resolved 644,
  sheets 95, and 9 files with a chain. 870 - 843 = 27 checks out, and 870
  equals the diff's entry-row count, as run.log asserts.
- 870 rows, 781 XML-named, 89 other, all identical and unflagged,
  files_identical yes on 27, diff exit 0, lo and lo_in ok, no leftover, order
  preserved, comment unchanged, and both labels agreeing on classes and on
  survey lines: run.log.
- The record pair's 5646 bytes / sha256 58f4d626... and 198280 bytes / sha256
  0d83de1c..., and "Its line 11 is `- Tool arguments: --survey`": run.log
  prints both hashes and the grep hit "11:- Tool arguments: --survey"; I
  recomputed the two hashes and both byte counts from the committed files.
- S0 to S14, the 15 extracted messages, the one unreachable message and the
  test that reaches it, N1 to N15 in the shapes the report lists, X1, X2, R1,
  R2: negative.log.
- 116 as-expected lines in negative.log and 99 in run.log, and the closing
  lines of test.log, negative.log and run.log: I counted them.
- gate.log's rerun note names its superseded round by 54710 bytes and sha256
  627b8048..., as the report says; negative.log's rerun note names the three
  defects the report's departure 3 lists.

Three items I could not find in a primary, and one count that does not add up:

(i) "the rest the zip layer's" (the tests paragraph). test.log prints "zip
    layer tests on the host: 78", while 217 - 37 - 23 - 9 = 148. The remaining
    70 are the other test binaries (xlsx-diff's and the passthrough tools'
    integration tests), not the zip layer's. The four cited counts are right;
    the fifth clause is wrong as written.

(ii) "`--survey` over the corpus prints the same 27 lines it printed at
     5fc1b3d, byte for byte". No log or committed record holds a survey run at
     5fc1b3d; the only corpus survey in the repository is the one at 1334830.
     The conclusion the sentence draws is independently supported: at 1334830
     an incomplete Default, Override or Relationship is exit 2 and an error
     outcome, and run.log shows 27 of 27 outcomes ok on both labels, so no
     corpus part holds such an element and the counts cannot have moved. It is
     the byte-for-byte comparison with the earlier commit that rests on
     nothing committed. What would settle it: label it an unlogged aside, as
     P1's departures 1 and 7 were, or cite the earlier output by hash.

(iii) "twelve cases compared stderr against 'error: ' alone" (departure 3).
      The committed negative.log's rerun note says three of its own checks
      failed, which the report matches, but gives no count of affected cases,
      and the superseded round was never committed. From the committed script
      the number of cases whose expectation is the usage text is eleven (N1 to
      N11). What would settle it: drop the number, or say it came from the
      uncommitted round.

(iv) The amended paragraph opens "Six of those figures are witnessed a second
     way" and then names seven table rows: rels, worksheet, calcChain, total,
     content-types, sharedStrings and styles. Five distinct values are
     involved (216, 95, 9, 870, 27). Neither reading gives six.

On the amendment itself. Every pairing holds in the primaries. The record's
"Entries by part family" table, host and i686 alike, gives content-types 27,
rels 216, workbook 27, worksheet 95, sharedStrings 27, styles 27, theme 27,
calcChain 9 and total 870; run.log's survey aggregate gives rels_parts 216,
sheets 95, with_calc_chain 9, entries 870, typed 843 (so 870 - 843 = 27),
without_shared_strings 0 and without_styles 0 over 27 files. The two tables in
the record are identical row for row between the labels.

The reworded claim is fair. The instruments are genuinely different code:
scripts/passthrough-record.py classifies the diff's entry rows by name with a
fixed table of regular expressions, and xlsx-diff is a separate crate whose
only dependency is the zip crate, so nothing in that path runs the package
layer. Two qualifications I would want a reader to have:

- The strength of the pairing varies. `calcChain` 9 against with_calc_chain 9,
  and `worksheet` 95 against sheets 95, compare a name-based family count with
  something the layer derived by walking relationships or the workbook's
  <sheets>, which is a real cross-check. `total` 870 against entries 870
  compares two independent zip readers. But `rels` 216 against rels_parts 216
  is two NAME-based classifiers: the survey's rels_parts comes from
  rels_part_source applied to entry names, not from the relationship graph. It
  confirms that the two spellings of the rels naming rule agree, which is
  worth having, but it is not the layer's relationship walk confirming the
  record.
- The parenthetical "(870 less 843 typed)" explains the content-types row of
  27 only; the sharedStrings and styles rows of 27 are witnessed instead by
  without_shared_strings 0 and without_styles 0 over 27 files, which the
  sentence does go on to say.

=====================================================================
Two notes that belong to whoever files this answer

- records/logs/m1p2-p2/run.log, scan.log and negative.log each cite
  "independent check 1 of this phase" as an input already acted on (the
  column-resolution class, the git show that skipped rather than failed, and
  the reader of the messages). The draft report also says "two independent
  checks". If those citations are right, this answer is the phase's SECOND
  check and should be filed as independent-check-2.md, and specs/0001 allows
  no third round.
- The scan log I read reports "check answers scanned: 0", so the committed
  scan log must be the later run, and the report's sentence about it hashing
  "both check answers" describes that run, not this one.
