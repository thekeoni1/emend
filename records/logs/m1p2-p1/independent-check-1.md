Milestone 1 part 2 P1 (specs/0009-package-layer.md) independent check 1
checked commit: 7a856adac9e00b3b6050c4132e016dd409f02bd1
HEAD when the check ran: 7a856adac9e00b3b6050c4132e016dd409f02bd1

Inputs, each as given to the checker, with the sha256 it had then. Every one
of the five logs named here is SUPERSEDED: this check is what caused the
phase's code commits to be redone, and each rerun log's header names the log
it replaces by sha256 and says why. The commit above is therefore no longer in
the phase's history; the phase now ends at 47257953f752345c4333cae27a03369d0be62048.
  specs/0009-package-layer.md              ef229aedf7f9181100783ff3f676b7401345dcc68e405f130a14ec2b80249584
  phase diff c414445..7a856ad              ec6072f5e8c957dc4bc85deca6ed4d0efcea0bf57f0e643db186186edc67fde8
  the draft report                         d82f5d4ce5f433fc0fec4216ef47d961a39fe7ee81301dc051a1154d43b9ba90
  records/logs/m1p2-p1/gate.log            dd6ea7245654519bf3d7ec7f283909ca2a58cf2797546b5a19a67ae784213ccd
  records/logs/m1p2-p1/test.log            3f4780a3e3dce5009cbf58193f9da3865a26732520add283e913ad1a2b7a7f69
  records/logs/m1p2-p1/negative.log        fd1a309d02252ad24bbc06fd442fa4f653afa0b66b0830629c79402beae326e2
  records/logs/m1p2-p1/run.log             7f5e7abbde02db2110f08f652f7254f604cb374f792ac0c56ebc0e0aec4c77e1
  records/logs/m1p2-p1/scan.log            2cd4f2a7b27be75a5a8d96ee43593b046661d756b048315ab601c5b3331dd515

The answer below this line is verbatim. Non-ASCII characters replaced: none;
the checker's file was already plain ASCII, which this session verified before
writing it here.
extracted answer: 17005 bytes, sha256 eb28a2af7daf2bc593a04931bd7171e1cbf953ab8d88e1a25a8bd80ec6860cee
---
The harness put the repository's CLAUDE.md into my context automatically. I say
so as asked, and I have not used it as evidence for anything below: my sources
are specs/0009-package-layer.md, specs/0001-bootstrap.md, specs/0008-zip-layer.md,
the phase diff c414445..7a856ad, the five logs under records/logs/m1p2-p1/, the
record pair records/m1p2-p1-2026-09-17-7a856ad.{md,txt}, and the draft report.

## 1. Does the phase diff do what the spec says, and nothing else?

Substantially yes. I found no work in the diff that belongs to P2 or P3, and
nothing P1 names that is missing from it.

Commit split, read from git:

    0e6c60d  ROADMAP.md, specs/0008-zip-layer.md, specs/0009-package-layer.md
    9b87536  specs/0009-package-layer.md
    b97d01c  specs/0009-package-layer.md
    e6559d8  Cargo.lock, crates/emend/Cargo.toml, crates/emend/src/archive.rs,
             crates/emend/src/lib.rs, crates/emend/src/xml.rs,
             crates/emend/src/xml/tests.rs,
             tools/size-probe/src/bin/probe-emend-copy.rs
    7a856ad  tools/emend-copy/src/main.rs, tools/emend-copy/tests/null_rewrite.rs

Three doc commits precede both code commits, which is what P1 ruling 3 asks
("This ruling is its own doc commit, before any code commit"). Docs and code are
not mixed in any commit.

What the spec asks of P1 and the diff delivers:

- The dependency, pinned and featured as the spec writes it:
  `quick-xml = { version = "=0.42.0", default-features = false }` in
  crates/emend/Cargo.toml, and only there in the core.
- `xml::{Stream, Token}` with `new(reader, index)`, `bom()`, `next()` and
  `Token { event, raw }`; the whole input including the mark is fed to the
  parser and positions are mapped through an origin of 3 or 0
  (`let end = self.origin + self.reader.buffer_position();`), which is
  ruling 1 exactly, not the draft's own strip.
- `Error::Xml(usize)` added to archive.rs with the catalogue entry and the
  Display arm "not well-formed XML or not UTF-8 (entry {index})".
- The three accepted not-well-formed classes (ruling 2) and the fourth,
  character-range class (ruling 4) are in the xml module's Limits, each with a
  test (`the_three_accepted_classes_are_accepted_and_reconstruct`,
  `a_character_xml_forbids_is_accepted_when_it_is_valid_utf8`).
- Every case in P1's test list is present in crates/emend/src/xml/tests.rs: the
  every-form part in all eight framings, the two-mark part asserting
  `kinds == vec!["Text", "Empty"]`, the empty part, position tiling inside
  `read_part`, the four refused malformed shapes, the non-UTF-8 byte in text and
  in an attribute value and in a name, the binary part with its four named
  shapes, and NUL plus ESC accepted and reconstructed.
- `emend-copy --xml-null-rewrite` with `is_xml_name` matching `.xml` and
  `.rels` ASCII case-insensitively, at most one operation after the two paths,
  one temporary per rewritten entry, and the mark written from `bom()`.
- Residue 1: the stale sentence in probe-emend-copy.rs is rewritten in the
  FIRST code commit, as the spec requires.

Two items worth planning's eye, neither an out-of-scope addition of function:

- Residue 2, "the duplicate-name candidates comment", is not in the diff. I
  checked: that comment lives in a log script, records/logs/m1p1-p3/size.log
  ("## the duplicate-name candidates (P3 ruling 2)"), not in any committed
  source file, so P1's code commit had nothing to edit. The spec's own gloss
  supports the report here ("which part 2's size script does not carry since it
  measures no candidates"), and part 2's size script is P3's work. I read this
  as discharged, but it is planning's call.
- Four public or semi-public items beyond the spec's named surface:
  `pub const BYTE_ORDER_MARK` (re-exported from lib.rs), `Stream::position`
  (`#[cfg(test)] pub(crate)`), and in the tool `Error::Rewrite` and the
  `temporary` helper. The report discloses the first two as items 4 and 5. Only
  BYTE_ORDER_MARK widens the crate's public API beyond what "Module `xml`"
  lists.

Two smaller discrepancies inside the diff:

- The spec's tool test asks for "the XML parts' decompressed bytes identical".
  tools/emend-copy/tests/null_rewrite.rs compares CRC-32 and uncompressed size
  for the deflated XML entries (raw bytes only for the stored `_rels/.rels`),
  and says why in its header: the test builds deflate without a compressor and
  cannot inflate the layer's output. That is a strong pin, not the literal
  assertion the spec words. What would settle it: planning either accepting
  CRC-32 plus length as the decompressed-bytes check, or asking for an inflater
  in the test.
- crates/emend/src/xml.rs's header says "[`Error::Io`] is returned unchanged
  when the underlying reader fails", while `parse_error` reconstructs it:
  `quick_xml::Error::Io(e) => Error::Io(io::Error::from(e.kind()))`, and that
  function's own comment says "the kind is carried across rather than the error
  itself". The header sentence is wrong about "unchanged"; the behaviour is
  right and tested.

I found no `package` module, no `Scope`, no move to `testing.rs`, no
`--survey`, no `--drop-calc-chain`, no probe-zip-xml-write,
no probe-emend-package-xml and no size record anywhere in the diff.

## 2. Do the logs contain every line the spec requires?

Yes, for all five logs, and the important checks are structurally capable of
failing. Details, because this is where the repository's history says to look:

- All five logs open with `command: git rev-parse HEAD` and an `about:` line
  naming the phase commit (or, for scan.log, the phase diff
  c414445..7a856ad) and the HEAD at run time, followed by the phase line and
  SERIALIZED. The five required logs are present and no sixth was needed
  (size.log is P3's).
- gate.log: "eight PASS lines: as-expected", "exit: 0; stderr bytes: 0", and
  "the licences step passed with quick-xml in the core's tree: as-expected".
  Because no gate file is in the diff, it lists them:
  "gate files checked: 10; of them in the phase diff: 0", which is what
  specs/0001 demands of a phase that changes none.
- test.log: both targets, "host executed 178 tests; i686-unknown-linux-musl
  executed 178 tests" and "the i686 run executed the same test names as the host
  run: yes".
- negative.log ran in a scratch clone with the gate's four binaries copied in,
  and prints the same four sha256 values gate.log prints; every fixture is
  generated ("the generator exits 0"). All three refusal classes the spec names
  are there (N1 to N3 two operations at once, N4 to N6 unknown operation, N7 and
  N8 an operation before the paths), plus the runner's tab refusal (R1) with its
  no-tab control (R2), and three accepted forms (N11 to N13) so the refusals are
  not vacuous.
- run.log asserts the acceptance from the entry rows, not the summary, and the
  assertions are read out of the one printed measurement line
  (`eval "$(echo "$line" | tr ' ' '\n' | sed 's/^/e_/')"`), so the printed
  numbers and the verdicts cannot disagree.
- scan.log is last, hashes all four siblings at bytes that match the files on
  disk (I recomputed: gate dd6ea72..., test 3f4780a..., negative fd1a309...,
  run 7f5e7ab...), scans the record pair, runs the pre-P6d bare-shape control
  over 18 files, and greps the four printed scripts for the forbidden compound
  shape: "instances of the forbidden compound shape: 0 (expect 0); errors: 0:
  yes", with both a positive and a negative control on the pattern. It reports
  "check answers scanned: 0", so this is the earlier run, as I was told.

Could-it-have-printed-anything-else, per log:

- The verdict helpers are `is() { if "$@"; then echo as-expected; else echo
  UNEXPECTED; fi; }` and `note() { echo "  $1: $2"; [ "$2" = as-expected ] ||
  fail=1; }` (gate, negative, run), and `yn` printing yes or no (test, scan).
  The string UNEXPECTED appears in these logs only inside those definitions and
  in the `ctl` helper's own source line, never as an output; test.log and
  scan.log have zero lines ending ": no". So the green lines are the taken
  branch of a two-branch form, not a constant.
- Every multi-condition check is a named function evaluated once:
  `clean_source`, `unchanged_by_this_phase` (gate.log), `denylist_usable`,
  `files_complete`, `messages_complete`, `pre_clean`, `pre_covers_diff`
  (scan.log). I searched all five printed scripts for the T0b P2 shape, on one
  line and across a continuation, and found none outside the deliberate control
  fixture.
- run.log's load-bearing form, `entry_counts`, is run first on a six-row
  fixture built in the log with a written expectation that makes every counter
  non-zero, including `xml_bad_status=1 xml_bad_flags=1 xml_crc_differs=1
  xml_size_differs=1 other_bad_status=1 other_bad_flags=1`, and the printed
  output equals the printed expectation. The zeros over the corpus are
  therefore zeros the form can report otherwise. Three of the real columns are
  additionally proven live by non-zero results: name (xml_total 781), status
  (xml_identical 10, xml_recompressed 771) and flags (xml_descriptor 242).
- One structural remark, the closest thing I found to a green line that could
  not go red. `entry_counts` resolves columns by header name
  (`$idx["crc_in"] != $idx["crc_out"]`), and run.log neither prints the real
  `entries.<label>.tsv` header nor asserts that those four columns exist. Had
  the diff tool's header named them anything else, awk would compare `$0` with
  `$0` and `xml_crc_differs` and `xml_size_differs` would be pinned at 0, while
  the control, which supplies its own header, would still pass. It is not a
  false green here: those exact names are in the committed, unchanged
  scripts/passthrough-run.sh ENTRIES_HEADER, so the columns do exist. A printed
  header line, or a "these four columns resolve" check, would close the class.
- A second, smaller one: scan.log line 159 hangs a structural check off a
  successful `git show`, `git show "$C:scripts/privacy-patterns.txt" > ... &&
  structural ...`, so a failed read would omit the check rather than fail it.
  It ran here ("this script's patterns equal scripts/privacy-patterns.txt at
  7a856ad...: yes").

Corpus privacy. I found no cell content, no original workbook file name and no
absolute machine path in any log or in the record pair; workbooks appear as ids
only, and `/home/dev` and `/root/` do not occur. Two lines touch the corpus
directory's NAME, both in run.log's printed script, not in output, and run.log's
header discloses them: the unexpanded default
`CORPUS=${EMEND_CORPUS_DIR:-$HOME/emend-testing}`, and a planted fictional
`/home/someone/emend-testing/corpus/x.xlsx` used as the positive control for the
record-privacy grep. I note them because my instructions name "a path naming the
corpus directory" as a defect; the report says part 1's run logs carry the first
form. Planning should say whether the disclosed shell default stays acceptable.

## 3. Does the report's every number and quoted string appear in a primary?

Almost all of them do. I recomputed the derived counts rather than trusting
them. Verified against a primary:

- The five commit hashes and subjects, and the base c414445: git, and scan.log's
  "the doc commit resolves to 0e6c60d...; its parent is the base, c4144452...".
- Check-line counts: gate 84, test 30, negative 52, run 54. I counted
  `as-expected$` in gate/negative/run (84, 52, 54) and `: yes$` in test (30);
  all four match exactly, with no contrary line anywhere.
- Gate: "eight PASS lines", exit 0, stderr empty, "the ten it checked" ("gate
  files checked: 10"), "All 12 files in the diff were scored, the 4 added ones
  absolutely and the 8 modified ones by change" ("files added in this phase: 4",
  "files modified in this phase: 8", "files in the diff: 12; files scored as new
  or modified: 12").
- "the core's tree is 12 packages, every one permissively licensed": "distinct
  packages in the core's tree: 12", "packages listed: 12 of the 12 in the tree",
  "lines whose licence names an allowed expression: 12 of 12". "brings `memchr`
  alone": "memchr is in the core's tree, as the only crate quick-xml brings:
  as-expected".
- Tests: "178 tests on each target", "0 failed", "23 of its own", "7" in the
  null_rewrite binary, "Two tests are ignored, both the documented 4 GiB zip64
  cases" ("ignored tests on the host: 2; of them in the xml module: 0; of them
  the documented 4 GiB zip64 cases: 2"). I also counted 23 `#[test]` in
  crates/emend/src/xml/tests.rs and 7 in the tool's test file.
- "xml.rs, 284 lines": the file is 284 lines; the diff stat says 284 insertions.
  "compacts once per 64 KiB released": `const COMPACT_AT: usize = 64 * 1024;`.
- Run: 27 of 27 ok on each label, lo and lo_in ok, no leftovers, order
  preserved, comment equal, files_identical no for all 27, diff exit 1 on all
  27, --timeout 300, lo-timeout 600, 2048 MiB, and the four timestamps
  23:28:15Z/23:31:39Z and 23:31:39Z/23:35:08Z: run.log's summary block and the
  two run rows.
- "870 entry rows per label, 781 XML-named and 89 other. Of the 781: 771
  recompressed, 10 identical, 0 any other status, 242 carrying descriptor ... 0
  CRC-32 differences and 0 uncompressed-size differences ... Of the 89 non-XML:
  every one identical and not one flagged": both labels print
  `xml_total=781 xml_ok=781 xml_identical=10 xml_recompressed=771
  xml_bad_status=0 xml_bad_flags=0 xml_descriptor=242 xml_crc_differs=0
  xml_size_differs=0 other_total=89 other_bad_status=0 other_bad_flags=0` and
  "entry rows in all: 870; XML-named 781; other 89". The record's family table
  totals agree independently: 870, 99 identical, 771 recompressed, 242
  descriptor, and 10 + 89 = 99 recomputes.
- Record pair: 15368 bytes / 6c4a6e8f..., 207736 bytes / f5fce548...  I ran
  sha256sum and stat on both files; all four values match run.log and scan.log.
- "four lines match a home-or-corpus path form, and all four are lines of the
  printed script": I ran the log's own pattern over run.log and got exactly 4
  hits, at lines 88, 445, 450 and 451, all inside the printed script, and the
  three categories the report names account for all four.
- Negative: "8 case families" is not the log's own word (its script says "Three
  families of case", N, X and R) but it does have exactly 8 case sections (N1-N3,
  N4-N6, N7-N8, N9-N10, N11-N13, X1, R1, R2), so the number is recomputable; the
  wording is loose, not wrong. "naming the FAILING entry's index (1, not 0)":
  "X1 stderr naming entry 1: 1; naming entry 0: 0". "Every message asserted in
  this log is read from the committed source at run time": the three source-read
  blocks are printed with their commands.
- The two rerun hashes, edccb3c8... (test) and c878cbd1... (negative), and the
  quoted first-round lines 'ignored tests on the host: 2; of them in the core: 1'
  and 'no test of the core is ignored: no', appear verbatim in the rerun headers
  of test.log and negative.log.
- Ruling quotations in report section 2 ("Both were raised before the run...")
  and the measured tuple 4/4/0, 7/4/3, 7/4/3, 10/7/3 appear in
  specs/0009-package-layer.md, "Planning rulings during P1", which is in the
  diff.

Figures and claims I could NOT find in a primary:

1. Report item 7: "26 files and 765 XML-named entries", and "One corpus file is
   an .xltx". Neither 26, nor 765, nor "xltx" occurs in any of the five logs or
   the record pair. This is the implementer's own aside, not a measurement the
   phase printed.
2. Report item 7's re-measurement: "Counted over all 27, every fact in
   specs/0009 reproduces exactly: 781 entries, 565 .xml and 216 .rels, 158
   starting with a mark." 781 is measured in run.log and 216 appears as the
   `rels` family row in the record; 565, 216 and 158 appear in
   specs/0009-package-layer.md's "Facts planning checked", which is in the diff,
   so the numbers are in a primary as PLANNING's facts. The report's own
   recount of 565 and 158 is nowhere printed, so "reproduces exactly" is prose,
   not a logged measurement. Grep of the five logs for 565 and 158 returns
   nothing.
3. Report item 6: "Both targets produced byte-identical output on that file."
   run.log compares the two labels' id, name, status and flags rows
   ("both targets classified every entry the same way"); it does not compare
   either target's output file bytes with the other's. The claim is not
   measured. The 28-second and 30-second timings themselves DO appear, in
   run.log's header, explicitly as a measurement taken outside the log.
4. "records/logs/m1p2-p1/scan.log  TBD_SCAN", "TBD_CHECKS", "TBD_CHECK": open
   placeholders. Nothing to verify yet, but they must be filled before the
   report goes to planning.

No number in the report disagrees with the log it came from, and every
arithmetic relation I could recompute did recompute (781 + 89 = 870;
10 + 771 = 781; 10 + 89 = 99; 4 + 8 = 12; 12 diff files + 4 sibling logs + 2
record files = the 18 files the bare-shape control covered).
