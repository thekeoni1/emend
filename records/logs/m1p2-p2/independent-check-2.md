Milestone 1 part 2 P2 (specs/0009-package-layer.md) independent check 2
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
extracted answer: 28714 bytes, sha256 2fac1bb9fc0403d0aa08495c9f89c90addf419e796ea1c24d927414fbde793ee
---
Independent check, Milestone 1 part 2 P2 (specs/0009-package-layer.md, "P2:
content types, relationships, the workbook").

The harness put CLAUDE.md into my context automatically. I say so as asked and
I have not used it as evidence for anything: my sources are specs/0009-package-
layer.md, specs/0001-bootstrap.md, the diff d6fa91d..1334830, the five logs
under records/logs/m1p2-p2/, the record pair records/m1p2-p2-2026-09-18-1334830
.md and .txt, and the draft report. Where I say "recomputed" I ran the sum or
the comparison myself over the primary.

I read the whole of crates/emend/src/package.rs, the xml.rs and archive.rs
hunks, the tool hunks, the move commit's two sides, the five logs in full or by
section, and both part-family tables of the record.

================================================================
1. Does the phase diff do what the spec says, and nothing else?
================================================================

Substantially yes. The diff is twelve files and every one is P2's.

Delivered, against the P2 phase bullet:

- Code commit 1 (82d4e77) is the move. I checked it mechanically rather than by
  eye: I extracted the 476 lines the commit removes from archive/tests.rs, the
  522 lines it adds as testing.rs, stripped "pub(crate) " and blank lines from
  both, and diffed. The only difference is the new module header and the ten
  zip format constants, each with an APPNOTE citation, exactly as P2 ruling 1
  directs; no moved line changed otherwise. Nothing in archive.rs became
  pub(crate); archive/tests.rs names the ten one by one from crate::testing
  rather than through a glob.
- Code commit 2 (5fc1b3d) and 3 (1334830) give package::{Package, ContentTypes,
  Default, Override, Relationships, Relationship, Workbook, Sheet, SheetKind},
  xml::Scope and emend-copy --survey. Every spec clause I checked in the code
  holds: ContentTypes::of tries the Override on "/" plus the entry name byte
  for byte and then the Default with eq_ignore_ascii_case; target_part strips a
  leading slash or joins to the source's directory and resolves "." and ".."
  either way (ruling 5), with the two refusals; the relationships namespace and
  the eight relationship types are public (transitional, strict) pairs, looked
  up transitional first (Relationships::first_of); the workbook counts both
  officeDocument forms together (ruling 6) and refuses a resolved name no entry
  has (ruling 3); an incomplete Default, Override or Relationship is refused
  with the one message of ruling 4; a sheet name is read through
  normalized_value(XmlVersion::Implicit1_0) (ruling 2) and the rest are bytes as
  written; Package::open parses nothing.
- Scope records each xmlns and xmlns:<p> with its depth and drops them at the
  matching end tag; I traced the depth arithmetic (start increments then
  declares, end retains depth < self.depth) and it is right for the root and for
  an empty element, and resolve_element/resolve_attribute differ exactly where
  the spec says they must (an unprefixed attribute has no namespace).
- --survey prints "survey" then the seventeen keys in the spec's order, counts
  only, with every field u64; it walks content types, every entry named as a
  rels part, and the workbook, and exits 2 with the message on any parse error
  before any output file exists.
- The specs/0001 "isnot" rule and the ROADMAP P1 row are in the doc commits, as
  the P1 verification ruling directs.

Not present, correctly: I grepped the diff's added lines for write_to,
remove_relationship, add_relationship, remove_override, add_override,
remove_part, replace_part, add_part, --drop-calc-chain, TempFile,
probe-zip-xml-write and probe-emend-package-xml. The only hit is a doc sentence
in package.rs saying those are P3's. tools/size-probe is untouched and there is
no size.log. No P3 work leaked in.

Additions the spec does not name, all of which I judge inside P2's scope
because --survey or the private fields require them: rels_part_name and
rels_part_source (the survey key "rels_parts" is defined as entries named as a
rels part "each parsed for the source its name derives", which is precisely
rels_part_source); the accessors defaults(), overrides(), all(), source(),
first_of(); the constants CONTENT_TYPES_PART, CONTENT_TYPES_NS,
RELATIONSHIPS_NS; the tool's Error::Report for a failed stdout write; Scope's
re-export from lib.rs.

Two things in the diff I would call wrong, neither functional:

F7. archive.rs's new catalogue entry says Error::Package is "Produced by
[`crate::package`] and by [`crate::xml::Scope`], each of which catalogues its
own messages in its module header with the test that pins it". xml.rs's own
"# Errors" header was not updated: it still opens "This module produces one
variant, catalogued here as `archive`'s are", lists only Error::Xml and
Error::Io, and closes "Both are asserted by a test in this module's `tests`".
The Scope message is in fact catalogued with its test, but in package.rs's
header, as "an element or attribute uses a prefix its document does not bind"
... (`rel_id_prefix_unbound`). So the spec's "Errors" clause is met in
substance and archive.rs's sentence about xml.rs's header is not accurate.

F8. crates/emend/src/package/tests.rs's header says "The smallest package is
five parts" and lists five; smallest() returns six, adding xl/styles.xml (which
workbook_options_take_the_first and the styles relationship need). A comment,
not a claim in a log.

I checked the P2 test list case by case against the 37 names test.log prints,
and every case in the spec is there, including the chartsheet (asserted inside
strict_namespaces and several_sheets_in_order, which also covers a
dialogsheet), the three ruling-4 cases, ruling 3's office_document_target_absent
and open_reads_no_xml. I read six of the tests in full (target_parent,
extension_matched_case_insensitively, rel_prefix_bound_three_ways,
sheet_name_escapes, two_office_documents_one_of_each_form,
default_lacks_an_attribute); each asserts a specific value or a specific
message, none is a smoke test. Nothing P2 names is missing.

================================================================
2. Do the logs contain every line the spec requires?
================================================================

Almost. Five logs are present (gate, test, negative, run, scan; no size.log is
due in P2), each names its commit and the HEAD at run on its first lines, each
prints its own script and scripts/counting.sh, each runs counting_controls, and
each ends with a single result line. Every P2 clause of "Harness runs" is
asserted in run.log and every one printed as-expected. Three gaps and one
mislabel:

F1. specs/0009 P2 ruling 1 ends "The test count stays 178 on each target and
`test.log` shows it." No log under records/logs/m1p2-p2/ contains the string
178 at all: I grepped all five. test.log was run at the final commit and shows
"host: passed 217; failed 0; ignored 2" and the same for i686, which is the
count after code commits 2 and 3 added 39 tests. The move's own test count is
claimed only in commit 82d4e77's message, and a commit message is not a log.
The property the ruling asked to be shown (the move changed no test) is
therefore not shown by a log. What would settle it: a line in test.log, or in a
rerun, printing the count at 82d4e77, or planning accepting the commit message
plus the byte-level equality of the moved code (which I did verify above, and
which is the stronger evidence).

F2. specs/0001, as this phase amended it, reads: "A negated condition goes
through a helper that negates (`isnot() {...}`), with a control showing `isnot
true` reports UNEXPECTED." Only run.log carries that control ("control: isnot
reports a passing command as UNEXPECTED, so it can fail: as-expected").
gate.log defines isnot and uses it four times (the structure_sound rejections)
and negative.log defines it and uses it once ("control: a key no message holds
selects nothing"), and neither log has the control. In gate.log the exposure is
small, because the positive "is structure_sound" on the phase's real class list
printed as-expected in the same block, so isnot cannot be stuck on
"as-expected" without that line disagreeing. In negative.log the single isnot
use is the only one, so a mis-defined helper would make that control vacuous
and nothing else in the log would notice. This is the phase's own new rule,
unmet in two of the three logs that use the helper.

F3. records/logs/m1p2-p2/scan.log line 2 reads "about: M1 part 2 P1 phase diff
d6fa91d3acac30e2ad229e76fe6bc19569d9b457..1334830933799ccfd119423c44605f650447
fda0; HEAD at run: ...". The phase label is P1; this is P2. The stale string is
in the printed script (`echo "about: M1 part 2 P1 phase diff $BASE..$C; ..."`),
inherited from the P1 scan script. The commit ids are correct, resolved from
git, and line 6 names P2 correctly, so nothing is misidentified in substance;
but this is exactly the inherited-text class the log headers say they audited
for.

F10. columns_resolve, the guard that entry_counts' column names are in the real
header, cannot fail on an empty file: its only rule is NR == 1, so on a file
with no lines END runs with `miss` unset and it exits 0, that is "as-expected".
It is not vacuous as used here (the entries file's existence is checked before
the block, entry_counts printed 781 and 89 from it, and the "781 XML-named
entries" assertion would have failed on an empty file), but the form itself
reads a missing file as a pass.

The two aggregating awk forms in run.log, which the brief asks about
specifically. I answer for each: which branch produced the printed line, could
it have printed anything else, and does the fixture control exercise the same
code path as the real use.

entry_counts (run.log, the function at "entry_counts() {"). Real printed line,
both labels: "xml_total=781 xml_ok=781 xml_identical=781 xml_recompressed=0
xml_bad_status=0 xml_bad_flags=0 xml_descriptor=0 xml_crc_differs=0
xml_size_differs=0 other_total=89 other_bad_status=0 other_bad_flags=0". Every
one of the twelve fields is incremented inside a data-dependent branch and
every one is printed with "+ 0", so a zero means "no row took that branch", not
"the branch does not exist". The fixture control prints "xml_total=4 xml_ok=3
xml_identical=1 xml_recompressed=2 xml_bad_status=1 xml_bad_flags=1
xml_descriptor=1 xml_crc_differs=1 xml_size_differs=1 other_total=2
other_bad_status=1 other_bad_flags=1": all twelve fields non-zero, so every
branch is shown to fire, including the case-folded name (docProps/app.XML
counted as XML) and each of the seven failure classes. The form resolves
columns by header NAME into idx[], so the fixture's ten-column header and the
runner's nineteen-column header take the same code path; the positions differ
between the two, which is what makes the fixture a real control rather than a
coincidence. The one failure mode left is idx["name"] being unset, which in awk
makes $idx["name"] read $0; that is guarded by columns_resolve with a positive
control and a negative one (crc_out renamed), and the real header is printed in
the log so a reader can check it. Conclusion: the line could have differed had
the data been wrong, and the fixture exercises the same path. Subject to F10,
this check can fail.

survey_counts (run.log, the function at "survey_counts() {"). Real printed
line, both labels: "lines=27 not_survey=0 untyped=0 unresolved=0 case_only=0
sheets_unresolved=0 without_shared_strings=0 without_styles=0 with_calc_chain=9
entries=870 typed=843 defaults=110 overrides=614 rels_parts=216
relationships=878 external=234 resolved=644 sheets=95". All eighteen fields are
data-dependent; the fixture prints every one of the eighteen non-zero
("lines=2 not_survey=1 untyped=1 unresolved=1 case_only=1 sheets_unresolved=1
without_shared_strings=1 without_styles=1 with_calc_chain=1 entries=12
typed=11 defaults=3 overrides=4 rels_parts=3 relationships=7 external=3
resolved=3 sheets=5"), and I recomputed each of those eighteen from the
fixture's two survey lines by hand: all eighteen are right. The pairs are read
by name, not by position, and "for (k in v) delete v[k]" clears the array per
line, so a key missing from line N cannot inherit line N-1's value, which is
the obvious way a missing key would have hidden. The fixture's third line,
built with real tabs from printf rather than a backslash-t (the defect the log's
own comment and the report's departure 3 record), is space separated and scores
not_survey=1, so the "no line was anything but a survey line" check is live. The
same function is called on the concatenated per-workbook stdout in the real use,
one file in both cases. Two residual notes: a key absent from every line would
leave the pair comparison "sheets_resolved != sheets" comparing two
uninitialised values, which awk calls equal, so that one class could go quiet;
but the same absence would drive "sheets=95" to 0 and fail, and the
"one survey line per workbook" check ties lines=27 to the 27 summary rows and
to the separately counted 27 stdout files. Conclusion: this line too could have
differed, and the control is on the same path.

The two shape greps in scan.log. SHAPE and BANG are assembled at run time and
printed; both include isnot in the helper alternation, as the P1 verification
ruling requires. Six controls, all through f_shape/f_bang: each pattern scores 1
on a fixture holding its own forbidden form, 0 on a fixture doing the same work
with a named function and with isnot, and 0 on the other pattern's fixture. So
neither grep is a pattern that matches nothing. I ran both patterns myself,
with /usr/bin/grep under LC_ALL=C, over all five logs: 0 hits in every one,
including scan.log itself, which the phase's own scan cannot cover and which
specs/0001 leaves to planning's verification scan.

Other things I checked in the logs and found sound: gate.log's eight PASS lines
and exit 0 on a cold cache; the ten gate files listed with 0 in the diff, which
is what specs/0001 asks of a phase that changes none; the core's tree with
exactly two direct dependencies, quick-xml 0.42.0 and zip 8.6.0, memchr the only
crate quick-xml brings, and all twelve packages' licences permissive; every one
of the twelve diff files scored, added and modified apart; test.log's ignored
block, which names the two 4 GiB zip64 cases and asserts none of the xml or
package tests is ignored; negative.log's fifteen extracted messages, each case's
key asserted to name exactly one of them.

Corpus privacy. I ran my own scan of the five logs. No hostname, no non-ASCII
byte, no control byte outside tab/LF/CR, no email address outside the allowed
three shapes, and no absolute machine path: the only hits for the path forms are
control fixtures assembled from pieces ("/mnt/", "c/", "Users/") and the pattern
text of a grep, plus run.log's unexpanded default CORPUS=${EMEND_CORPUS_DIR:-
$HOME/emend-testing} and its fictional planted-path control, both of which P1
ruling 7 allows explicitly. No workbook cell content, no original file name and
no corpus directory path appears. The per-workbook survey lines are an id plus
counts; the entry-name-quoting branch in run.log never fired ("no row violated
an expectation"). Nothing to flag.

I confirmed this is the earlier scan run, as the brief said: scan.log prints
"check answers scanned: 0". Its four sibling hashes I checked against the files
on disk and all four match byte for byte, as do the two record-pair hashes.

================================================================
3. Does the draft report's every number and quoted string appear in a primary?
================================================================

I took them one at a time. Unless noted, the figure is in a primary and, where
it is a total or a difference, I recomputed it myself.

Commits and structure
- The eight hashes and subjects, and "d6fa91d, the parent of bbde39e":
  gate.log's commit listing and scan.log's, both resolved from git. "Five doc
  commits and three code commits, interleaved": gate.log's class list reads doc,
  doc, code, doc, doc, code, doc, code; I counted 5 and 3.
- "gate.log classifies all eight ... none mixes planning documents with code,
  ... the range opens with a doc commit, ... at least one code commit": those
  are the four conditions of structure_sound, printed as one as-expected line.
- "The replacement, structure_sound, carries five controls": gate.log prints
  exactly five (accepts this phase's shape; rejects code first, mixed, empty, no
  code), plus two commit_class controls.

The code
- "package.rs, 878 lines": wc -l gives 878, and the diff adds 878.
- "Scope ... in 133 lines of a 423-line module": xml.rs is 423 lines and the
  Scope block runs from line 291 to 423, which is 133. Recomputed.
- "archive.rs gains Error::Package(&'static str), 14 lines": gate.log prints
  "added lines: 14" for archive.rs and the diff's numstat says 14.
- "tools/emend-copy/src/main.rs, 456 lines": wc -l gives 456.

The rulings
- Rulings 3 to 6 and the Version10/Implicit1_0 correction: all in specs/0009 as
  committed, and the reason is in Scope's doc comment as the report says.

F5. "`--survey` over the corpus prints the same 27 lines it printed at 5fc1b3d,
byte for byte, so no corpus part holds such an element": I could not find this
in a primary. No committed log records a corpus run at 5fc1b3d, no run
directory for that commit exists beside the two that do, and run.log's own
"about" line is the final commit. The conclusion the sentence draws is
nonetheless measured, differently and more directly: at 1334830, with the
refusing code in, all 27 workbooks surveyed with outcome ok and exit 0 on both
targets, which could not have happened had any corpus part carried an
incomplete Default, Override or Relationship. I would either drop the
comparison or replace it with that argument.

The tests
F4. "217 tests on each target, up from 178 at P1: 37 in the package module, 23
in the xml module, 9 in the tool's integration binary, the rest the zip layer's."
The first four figures are in test.log (217 and 217; 37 and 37; 23 and 23; 9 and
9) and 178 is in specs/0009's P1 verification rulings. "The rest the zip
layer's" disagrees with test.log, which prints "zip layer tests on the host:
78". The rest is 217 - 37 - 23 - 9 = 148, of which 78 are archive::tests and the
other 70 are in the harness tools' own test binaries. The delta does recompute:
217 - 178 = 39 = the 37 package tests plus the two survey tool tests, and the
xml module is unchanged at 23, which is consistent with Scope being exercised
from the package tests. The sentence needs "the rest the zip layer's 78 and the
harness tools' own binaries", or similar.
- "test.log names the package module's 37 and the tool's 9 ... both counts are
  asserted equal on the two targets, and no test of either new module is
  ignored": all three are printed as-expected in test.log.
- The case-by-case list and "open_reads_no_xml ... a package whose content types
  part is truncated mid-tag opens, its entries index, and the first
  content_types() call fails": I read the test; it sets the content types part to
  a root tag with no closing bracket, asserts the archive is indexed and a part
  findable, then asserts Error::Xml from the first content_types() call.
- "built byte by byte through crate::testing": package/tests.rs imports Build and
  Spec from crate::testing and nothing of either layer builds a fixture.

The harness runs
- The quoted aggregate block (lines=27 ... sheets=95) is run.log's
  "survey_counts over all of them" line, word for word, on both labels.
- I recomputed every total in it from the 27 per-id survey lines printed in
  run.log, with my own awk: entries 870, typed 843, defaults 110, overrides 614,
  rels_parts 216, relationships 878, external 234, resolved 644, sheets 95,
  sheets_resolved 95, untyped 0, unresolved 0, case_only 0, shared_strings 27 of
  27, styles 27 of 27, calc_chain 9. All agree.
- "Both targets printed byte-identical survey lines": run.log says so; I also
  sorted the host and i686 blocks and diffed them, and they are identical.
- "870 rows per target, 781 XML-named and 89 other": run.log's entry_counts
  line and "entry rows in all: 870"; 781 + 89 = 870. The per-workbook identical
  counts also sum to 870 on each label, which I recomputed.
- "typed is 843, which is 870 less the 27 content types parts": 870 - 843 = 27.
- "every diff exit 0, lo and lo_in ok on every file on both targets, no leftover
  temporary, order preserved and the archive comment unchanged" and "the two
  targets agree on every entry's id, name, status and flags": each is a printed
  as-expected line in run.log.
- The record pair: 5646 bytes / sha256 58f4d626...2777f and 198280 bytes /
  sha256 0d83de1c...cc625, and "Its line 11 is `- Tool arguments: --survey`". I
  checked all five against the files themselves, not only against run.log: byte
  counts, both hashes and line 11 match exactly.

The refusals
- "fourteen of the fifteen messages in package.rs's catalogue" and the reason
  the fifteenth is unreachable: negative.log extracts fifteen messages from
  package.rs and xml.rs and prints them; S1 to S14 cover fourteen; I counted
  fifteen bullets in package.rs's header catalogue and the one left is "an
  external relationship has no part", catalogued against
  package::tests::external_target, which exists in test.log's list.
- "116 as-expected lines" for negative.log and "99" for run.log: I counted them,
  116 and 99, with zero UNEXPECTED in either.
- "N1 to N15" and "X1, X2, R1 and R2": the labels and their contents are in
  negative.log. One wording note: the paragraph opens "negative.log runs every
  one of the layer's messages through the BUILT binary" and then says fourteen of
  fifteen; the qualification follows immediately, so it reads as a slip rather
  than a claim, but "every one but one" would be exact.

Departures
- Departure 3's three defects and departure 4's shape-grep hit: both are in the
  rerun headers of negative.log and gate.log. gate.log's rerun note names the
  superseded round by byte count and sha256 (54710 bytes, sha256 627b8048...),
  as the report says; negative.log's names the three defects and that no check
  of the layer changed.

The logs section
F6. "scan.log is written last and hashes every committed sibling and both check
answers". The first half is true of the committed file: it hashes the four
siblings and the record pair, and I verified all six hashes against the files.
The second half is not true of it: it prints "check answers scanned: 0". The
claim holds only of the final scan run, which the log's own header says is still
to come. As written, it describes a log that does not yet exist.
- "Each ... ends with a single yes-or-no line": four do; scan.log ends "RESULT:
  PASS". Trivial, but the sentence covers five logs.
- "Each grep has a positive and a negative control, and a cross control showing
  it does not match the other's form": six control lines in scan.log, all
  as-expected; verified.

The amendment (the "Entries by part family" paragraph)

Every figure in it is in a primary and every pairing holds. I read both tables
in records/m1p2-p2-2026-09-18-1334830.md and took the survey figures from
run.log:

- rels 216 and rels_parts 216: both present.
- worksheet 95 and sheets 95: both present.
- calcChain 9 and with_calc_chain 9: both present.
- total 870 and entries 870: both present. I also summed the table's twenty-three
  family rows myself and got 870, so the printed total is not a separate figure.
- content-types 27, sharedStrings 27, styles 27: all three present; 870 - 843 =
  27, recomputed; without_shared_strings 0 and without_styles 0 over 27 files:
  both present.
- "Both tables in the record read the same on host and on i686": I extracted the
  two tables and diffed them; they are identical row for row.
- The description of the instrument is accurate: scripts/passthrough-record.py
  builds the table from the entries table's rows, classifying each name by the
  first matching regex of its RULES tuple into the FAMILIES list.

Two qualifications, since you asked for them.

The revised opening, "Same 27 workbooks, different code over different readings
of them", is right for four of the six pairings and generous for two.

- rels 216 against rels_parts 216 is the weakest. Both instruments classify the
  same zip entry NAMES: the record's regex "(^|/)_rels/[^/]*\.rels$", the
  survey's rels_part_source over the archive's entry names. The agreement shows
  two name rules agreeing, which is worth having, but it does not witness the
  layer's parsing of anything. I would not count it among the six.
- total 870 against entries 870 is the same fact twice rather than a second
  witness: both count zip entries, and run.log already asserts the equality
  directly ("the survey counted the same entries the diff did: as-expected").
- worksheet 95 against sheets 95, calcChain 9 against with_calc_chain 9, and
  content-types 27 against 870 minus 843 are genuine: names under
  xl/worksheets and its siblings against <sheet> elements inside the workbook
  part; an entry named xl/calcChain.xml against a calcChain relationship that
  resolves; and an entry named [Content_Types].xml against the survey's rule of
  skipping exactly one entry per workbook.
- sharedStrings 27 and styles 27 are genuine too, with one nuance the sentence
  already handles: 27 entries across 27 files does not by itself mean one each,
  and the per-file part comes from the survey's own without_shared_strings 0 and
  without_styles 0, which the sentence cites. As written it holds.

So I would say "witnessed a second way by a different instrument" of four of
the six, and describe the other two as consistency checks. "Independent" was
indeed the wrong word, and "different readings" is still slightly strong for the
rels row.

One last observation, not a defect in the report, which claims nothing about it:
specs/0009's facts include "Every one of the 614 Override part names resolves
exactly", and the survey does not measure that. case_only counts an override
whose part name matches an entry only after ASCII folding; an override naming no
entry in either form is counted nowhere. The converse direction, that every
entry has a type, is measured by untyped 0. P2's "Harness runs" clause does not
ask for more, so this is a note for P3 rather than a gap in the phase.

================================================================
Summary of findings
================================================================

F1  Ruling 1's "test.log shows it" for the count 178 is unmet: no log holds the
    figure. The property itself I verified from the diff.               (Q2)
F2  gate.log and negative.log use isnot without the "isnot true" control
    specs/0001 requires; only run.log has it.                           (Q2)
F3  scan.log's "about" line names the phase P1, from an inherited string in its
    printed script. Commit ids correct.                                 (Q2)
F4  "the rest the zip layer's" disagrees with test.log's 78; the rest is 148.
                                                                        (Q3)
F5  "the same 27 lines it printed at 5fc1b3d, byte for byte" is in no primary;
    its conclusion is measured another way by run.log.                  (Q3)
F6  "scan.log ... hashes ... both check answers" is true only of the final scan
    run; the committed one prints "check answers scanned: 0".           (Q3)
F7  archive.rs says xml.rs's module header catalogues the Scope message with its
    test; xml.rs's header still says "one variant" and names neither.    (Q1)
F8  package/tests.rs's header says five parts; smallest() returns six.  (Q1)
F9  Minor wording: "runs every one of the layer's messages" then "fourteen of
    the fifteen"; and scan.log's last line is "RESULT: PASS", not a yes or no.
                                                                        (Q3)
F10 columns_resolve reads an empty or unreadable file as a pass. Not vacuous as
    used, but the form cannot fail on a missing file.                    (Q2)

Nothing I found bears on whether P2's acceptance was met. The two aggregating
awk forms and the two shape greps all could have printed something else had the
data been wrong, each is controlled on the same code path as its real use, and
every class each of them counts is shown non-zero on a fixture. The corpus
figures reproduce when recomputed from the primaries, the record pair's hashes
and byte counts are what the report says, and the diff delivers P2 and no part
of P3.
