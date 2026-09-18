Milestone 1 part 2 P1 (specs/0009-package-layer.md) independent check 2
checked commit: 47257953f752345c4333cae27a03369d0be62048
HEAD when the check ran: 47257953f752345c4333cae27a03369d0be62048

Inputs, each as given to the checker, with the sha256 it had then. The scan log
named here is the pass that reports "check answers scanned: 1"; the committed
scan log is the final pass run after this answer was saved, and its header
names this one.
  specs/0009-package-layer.md                  d398011dcff8159f937b3ea4683a6231a8c771936b508d15838673c7d9d5c9c7
  phase diff c414445..4725795                  d39ef5e94895aa0fad4f88daa2e5bd0f6020f5aaeece7ae438d115e5009466b2
  the draft report                             ed8cdfa8256d0fe30a10fc7ddbb81fc3f07213f27dc6a82fea3747dc8059d4e4
  records/logs/m1p2-p1/gate.log                7b3a33369073e8e76d5981061b7f71cc873fbd6b805347c2e2a06472664d2838
  records/logs/m1p2-p1/test.log                a7af4fd0f8735bf9f6d89040f41766f9144850601504070ce38f0c95792cd82e
  records/logs/m1p2-p1/negative.log            a4345e0f9cc6fd21d920a2c51285efa3fd75ef010d026edba62ea18ead54bc47
  records/logs/m1p2-p1/run.log                 a6e03ca97b80eb8397460a42404304d76ccb381c2b69b5c27fd4beb71ec93504
  records/logs/m1p2-p1/scan.log (pre-final)    5a7655269590c818d10f2bf993965d341dd835761eef00a89d9de13a66b40013
  records/logs/m1p2-p1/independent-check-1.md  6167031817cf6428af5ca479e7dbee71002919bff9ee19b912a9daae38f0ed34

The answer below this line is verbatim. Non-ASCII characters replaced: none;
the checker's file was already plain ASCII, which this session verified before
writing it here.
extracted answer: 20897 bytes, sha256 664f01ea8e42dffeb69c8d8fbd17d848e2ec52067e332f56b4c22b7613d33894
---
The harness put the repository's CLAUDE.md into my context automatically. I say
so as asked, and I have not used it as evidence for anything below. My sources
are specs/0009-package-layer.md, specs/0001-bootstrap.md, specs/0008-zip-layer.md,
the phase diff c414445..4725795, the five logs under records/logs/m1p2-p1/,
records/logs/m1p2-p1/independent-check-1.md, the record pair
records/m1p2-p1-2026-09-18-4725795.md and .txt, and the draft report. I read the
code and the logs myself rather than trusting either summary, and I re-ran the
shell forms I was unsure about in a scratch directory.

## 1. Does the phase diff do what the spec says, and nothing else?

Yes. Twelve files, six commits, four doc commits then two code commits, docs and
code never mixed. I found nothing P1 names missing and nothing that belongs to
P2 or P3.

The four repairs check 1 forced have all landed, and three of the four are sound
as written.

(a) The xml.rs module header. `git diff 7a856ad 4725795` is exactly two files,
and the xml.rs half is exactly the sentence:

    -//! - [`Error::Io`] is returned unchanged when the underlying reader fails, so
    -//!   a failing disk is never reported as a malformed part.
    +//! - [`Error::Io`] when the underlying reader fails, so a failing disk is
    +//!   never reported as a malformed part. The kind is carried across, not the
    +//!   error itself: `quick-xml` holds its I/O error in an `Arc` to stay
    +//!   `Clone`, so it cannot be moved out (see `parse_error`).

That now matches `parse_error` exactly (`quick_xml::Error::Io(e) =>
Error::Io(io::Error::from(e.kind()))`) and it no longer contradicts that
function's own doc comment. The repair is right, and the diff carries no other
code or test change between the two heads, as the draft report says.

(b) run.log's entries header and column check. Both halves landed. The log now
prints the runner's real header ("the entries header, as the runner wrote it:"
followed by the tab-separated line beginning `id label row name status ...`) and
asserts, per label, "every column the classifying form indexes is in that
header: as-expected", through the named function `columns_resolve` against
`COLUMNS_USED='id name status flags crc_in crc_out uncompressed_in
uncompressed_out'`. That list is a superset of what `entry_counts` indexes, so
the check is slightly stronger than the gap it closes. It carries two controls
in the same command form (a header with all the columns, and the same header
with `crc_out` renamed, the latter through `isnot`) and both print as-expected.
One residual, narrow hole, which I tested rather than guessed: the awk exits
from its `NR == 1` block, so on a ZERO-LINE file it reaches `END` with `miss`
unset and exits 0, i.e. reports as-expected. A missing file exits 2 and reports
UNEXPECTED, and a header with a wrong name exits 1. In this run the header is
printed non-empty immediately above and `entry rows in all: 870` is asserted, so
the green line here is real; the hole is only that an empty entries file would be
waved through by this one check.

(c) scan.log's patterns comparison. Landed and sound. The `git show` and the
comparison are now one named function:

    patterns_match() {
      git show "$C:scripts/privacy-patterns.txt" >"$T/patterns-at-c" 2>/dev/null || return 1
      cmp -s "$T/p6-patterns" "$T/patterns-at-c"
    }

A failed read now returns 1, so `yn` prints no and sets `bad=1`, which is what
check 1 asked for. It printed yes.

(d) The draft report's departures 6 and 7. Both repaired. Departure 6 now
withdraws the byte-identical-output claim in as many words and says what run.log
actually compares (id, name, status and flags rows). Departure 7 now labels its
own figures as an unlogged aside and says explicitly that only the 781 is
measured in run.log. I agree with both readings, and I can add something in the
report's favour: the 28-second and 30-second timings are not prose alone. The
run's own per-workbook stdout rows print 28.04 for id 9132ce5303d8 under host and
29.64 for the same id under i686, so the header's "about 28" and "about 30" are
corroborated by printed output.

Departures 9, 10 and 11, the defects the session says it found in its own
scripts, all check out:

- 9 (gate.log's typed commit counts). The replacement is structural and can fail
  in both directions. `commit_class` classifies each commit by the paths it
  touches (everything under `specs/` or `ROADMAP.md` is docs), `order_sound`
  rejects mixed, empty, a doc commit after a code commit, and a range with no
  code commit, and there are six controls: two on the classifier (the first doc
  commit is `doc`, HEAD is `code`) and four on the ordering (accepts docs then
  code; rejects late doc, mixed, and no-code, the last three through `isnot`).
  The printed class list shows four `doc` then two `code`. I could not construct
  a way for these lines to be green on a bad range.
- 10 (the `$(is ! f x)` control). The fix is real: `isnot() { if "$@"; then echo
  UNEXPECTED; else echo as-expected; fi; }` in gate.log and run.log, and run.log
  additionally controls the negation itself ("control: isnot reports a passing
  command as UNEXPECTED, so it can fail: as-expected"). `!` appears in no
  `is`/`isnot` argument list in any of the five printed scripts.
- 11 (the column check) is (b) above.

Scope. No `package` module, no `Scope`, no `testing.rs`, no `--survey`, no
`--drop-calc-chain`, no `probe-zip-xml-write`, no `probe-emend-package-xml` and
no size record anywhere in the diff. `Error::Package` is not added; only
`Error::Xml(usize)`, which P1's Errors section asks for. Residue 1 (the stale
`probe-emend-copy.rs` sentence) is in the FIRST code commit a9f2cad, as "Crate"
requires; residue 2 is discharged by ruling 8. Every case in P1's amended test
list is present: the every-form body in all eight framings, the two-mark part
asserting `kinds == vec!["Text", "Empty"]`, the empty part, the position tiling
inside `read_part`, the four refused malformed shapes, the non-UTF-8 byte in
text and in an attribute value and in a name, the binary part with its four
named shapes, NUL and ESC accepted and reconstructed, the three accepted
classes, and the tool test on a byte-built archive of three XML-named parts and
one binary part with CRC-32 and uncompressed size for the deflated ones and raw
bytes for the stored one (ruling 5). `BYTE_ORDER_MARK` public and re-exported is
ruling 6, not an overreach.

Three small observations for planning, none a defect I would hold the phase for:

- The draft report's departure 4 still offers to make `BYTE_ORDER_MARK`
  `pub(crate)` and asks planning to say. Ruling 6 already settled that; the
  offer reads as an open question that is closed.
- The tool test carries its own copy of the byte-level builder and
  `parse_independently`. That is a third copy (after `archive/tests.rs` and
  `tools/passthrough/tests`), and the file's header says why a cross-crate
  `#[cfg(test)]` module cannot be shared. P2's move of the builder into
  `testing.rs` will not consolidate this copy; planning may want to say whether
  it should.
- `Tee::front` and `COMPACT_AT` are `usize`. Every input position, origin and
  count is `u64` (`seen`, `origin`, `dropped`, `release_to`); the `usize` values
  are in-memory buffer indices, which cannot be otherwise, and the module's
  Limits says the bound is the largest single token. I read this as consistent
  with the 32-bit rule rather than a departure from it.

## 2. Do the logs contain every line the spec requires?

Yes, for all five. Each opens with `command: git rev-parse HEAD`, an `about:`
line naming the phase commit (scan.log names the phase diff) and the HEAD at run
time, the phase line and SERIALIZED, and each carries a `rerun:` header naming
the log it replaces by byte count and sha256, which is what specs/0001 asks of a
rerun of an uncommitted log. The set is gate, test, negative, run and scan; no
`self-test.log` was needed, and gate.log establishes that ("gate files checked:
10; of them in the phase diff: 0"). size.log is P3's.

What the spec requires and I checked line by line:

- gate.log: the full gate on a cold cache after `cargo clean`, eight PASS lines
  including `PASS licences`, exit 0, stderr 0 bytes, and the licences step with
  quick-xml in the core's tree (the tree and all 12 package licences are printed;
  "lines whose licence names an allowed expression: 12 of 12"). 90 as-expected
  lines, no UNEXPECTED output line, final "gate log checks all as expected: yes".
- test.log: both targets, "host: passed 178; failed 0; ignored 2" and the same
  for i686-unknown-linux-musl, "the i686 run executed the same test names as the
  host run: yes", the xml module's 23 tests printed BY NAME so the list can be
  read against the spec, the tool's 7 taken from the null_rewrite binary's own
  section of the cargo output, and the ignored pair pinned to the two documented
  4 GiB zip64 reasons with a control that an ignored test with another reason is
  not counted. 30 yes lines, no no line.
- negative.log: the tool's three refusal classes, each in more than one shape
  (N1 to N3 two operations at once in both orders and the same one twice, N4 to
  N6 an unknown operation including one a letter away, N7 and N8 an operation
  before the paths), plus N9 and N10 on the operation's own arguments, three
  accepted controls N11 to N13, X1 for the layer's refusal, and R1/R2 for the
  runner's tab refusal and its control. 52 as-expected lines, final "negative log
  checks all as expected: yes". It ran in a scratch clone with the gate's four
  binaries copied in, and all four hashes equal gate.log's and run.log's, which I
  verified by comparison across the three logs.
- run.log: the whole corpus on both targets with the LibreOffice check,
  `--tool-arg --xml-null-rewrite` recorded in both run rows, the acceptance
  asserted from the ENTRY rows per class, and the record pair written and hashed.
  59 as-expected lines.
- scan.log: last, hashes all four siblings and independent-check-1.md, and every
  one of those hashes matches the file on disk (I recomputed all six). It scans
  the record pair, runs the pre-P6d bare-shape control over 19 files including
  every one of the 12 in the diff, and greps every printed script for the
  forbidden compound shape with a positive and a negative control:
  "instances of the forbidden compound shape: 0 (expect 0); errors: 0: yes". It
  reports "check answers scanned: 1", so this is the earlier run, as I was told.

Could-it-have-printed-anything-else. The verdict helpers are the two-branch
`is`/`isnot`/`yn`/`note`/`check`/`check_ctl`/`structural` forms; the string
UNEXPECTED occurs in the five logs only inside those definitions and in
comments, never as output, and only gate.log and negative.log contain a line
ending ": no" (the ANTHROPIC_API_KEY line, a generator comment, and N11's
expected "byte-identical to the input: no"). Every multi-condition check is a
named function: `order_sound`, `commit_class`, `clean_source`,
`unchanged_by_this_phase`, `status_accounted` (gate); `passed_matches`,
`entry_clean` (test); `ec_ok`, `no_leftovers`, `all_four_executable`,
`n11_statuses`, `main_untouched` (negative); `columns_resolve`, `both_halves`,
`status_accounted` (run); `denylist_usable`, `patterns_match`, `files_complete`,
`messages_complete`, `pre_clean`, `pre_covers_diff`, `shape_clean`,
`scan_passed` (scan). I tested two forms I was not willing to assume:

- gate.log's `bash -n "$s"; note "$s parses" "$(is [ $? -eq 0 ])"` does read the
  preceding command's status inside the substitution. I reproduced it: a failing
  `bash -n` prints UNEXPECTED. The three "parses" lines can fail.
- `columns_resolve` on an empty file returns 0, as described under (b).

run.log's load-bearing `entry_counts` is exercised first on a six-row fixture
whose written expectation makes every one of the twelve counters non-zero,
including `xml_crc_differs=1` and `xml_size_differs=1`, and the printed output
equals the printed expectation, so the corpus zeros are zeros the form can report
otherwise; three of the live columns are additionally proven non-vacuous by
non-zero results (781, 771/10, 242). The assertions are read back out of the one
printed line by `eval "$(echo "$line" | tr ' ' '\n' | sed 's/^/e_/')"`, so a
printed figure and its verdict cannot disagree.

Two things I would put in front of planning, neither a false green in this run:

- run.log's `nrows`, `count_where`, `count_in` and `sum_col` are the only
  counting forms in the five logs with no positive control of their own
  (specs/0001 asks for one for every scan form). Their failure mode is safe: a
  column name that did not resolve leaves `k` unset and the count 0, which makes
  the comparison against `rows` FAIL rather than pass. They are also corroborated
  in fact, since `sum_col`'s summary totals (identical 99, recompressed 771)
  agree with `entry_counts`' independent 10 + 89 = 99 and 771 from the entries
  file. A control would still close the class.
- negative.log's `left_none` counts with `find ... | wc -l`, outside
  scripts/counting.sh; the planted-temporary control at the end of the log covers
  the un-`maxdepth`ed form, not this one.

Two textual defects inside committed logs, both small:

- test.log's script says the first round reported two lines "(see this log's
  rerun header)". Its rerun header does not name that round at all: it names the
  round at 7a856ad, sha256 3f4780a3e3dce5009cbf58193f9da3865a26732520add283e913ad1a2b7a7f69,
  which "printed 178 tests on each target and 30 yes lines with no no line". The
  cross-reference points at something the header no longer carries. The spec asks
  each rerun to name the log it REPLACES, which it does, so this is a stale
  pointer rather than a missing required line.
- negative.log's header calls the operation-parsing family "N1 to N12" while the
  cases run to N13.

Corpus privacy. I found no cell content, no original workbook file name and no
absolute machine path in any log, in the check answer or in the record pair.
Workbooks appear as twelve-hex ids only. Every hit of the home-or-corpus path
form is accounted for: four in run.log (the unexpanded `CORPUS` default, the
record-privacy check's own pattern twice, and the fictional planted control),
one in gate.log and two in scan.log, all of them assembled-at-run-time control
fixtures in printed scripts and not output. Ruling 7 covers exactly these. The
record pair itself scores 0 on that form with a planted-path control that fires.
scan.log's total hits, read errors and structural failures are all 0 and it
prints RESULT: PASS.

## 3. Does the draft report's every number and quoted string appear in a primary?

Almost all do, and I recomputed the derived ones rather than trusting them:
781 + 89 = 870, 10 + 771 = 781, 10 + 89 = 99, 4 + 8 = 12, and the record pair's
15368 and 207736 bytes with both sha256 values, which I recomputed on disk. The
five commit ids and subjects, the base c414445, 178 tests, 0 failed, 23 and 7
tests, 2 ignored, 90/30/52/59 check lines and scan.log's 30 and 17, the 12
packages and their licences, the eight PASS lines, 10 gate files, 4 added and 8
modified files, 27 of 27 workbooks, the four timestamps, `--timeout 300`,
lo-timeout 600, 2048 MiB, 870/781/89/771/10/242/0/0, the four binary hashes, the
"Tool arguments: --xml-null-rewrite" line and the four path-form lines all appear
in the logs I named, and `COMPACT_AT` is `64 * 1024` in the diff. I found one
number that DISAGREES with its primary and several claims with no primary.

1. WRONG, and it should be fixed before the report goes out. "`crates/emend/src/
   xml.rs`, 284 lines, is the whole layer". The file is 286 lines at 4725795 and
   the diff records `+286` insertions; `git show 4725795:crates/emend/src/xml.rs
   | wc -l` gives 286. 284 was true at 7a856ad, which check 1 verified; the two
   lines the departure-8 repair added made it stale. This is the third typed
   count in this phase to go stale exactly the way departures 1 and 9 describe.
2. NOT IN ANY PRIMARY, and misattributed. Departure 1: "The rerun header names
   the superseded log by sha256 edccb3c8f7a219e53b720bdbccbefda58d68fc9f668b66c47c528874d656dbe0
   and by what it printed." test.log's rerun header names 3f4780a3e3dce...
   instead, and the string edccb3c8 occurs in no committed log; it survives only
   as an eight-character prefix inside check 1's answer. Departure 2 has the same
   problem: "The rerun header names the superseded log by sha256
   c878cbd1b313e241c560680f723f9ed6e59837fb8c02857fd2281cc2a0e89dfb", while
   negative.log's header names fd1a309d0225... and gives the redone code commits,
   not the two comments quoting the compound form, as the reason. I grepped all
   of records/ for both hashes: the only hit is check 1's answer. The two
   departures describe real history, which check 1 corroborates, but the header
   they point at is the wrong one, and the two 64-character hashes are in no
   primary at all.
3. NOT IN A PRIMARY. Departure 9's parenthetical about the intermediate gate
   round: "(the gate itself passed in that round: 8 PASS lines, licences
   included, stderr empty)". gate.log's header says only "the gate itself passed
   in that round too" for that round, with no figures, no byte count and no
   sha256; the 8 PASS lines, licences and empty stderr that ARE printed belong to
   the committed round. The report's own blanket sentence, "each header names the
   log it supersedes by sha256", is therefore not true of that intermediate gate
   round.
4. OFF BY ONE. "Of the superseded rounds, the four written at 7a856ad all
   passed." Five logs were written at 7a856ad: each of the five rerun headers
   names its own predecessor at that commit, scan.log's included (45629 bytes,
   sha256 2cd4f2a7..., RESULT: PASS). All five passed, so the claim understates
   rather than overstates.
5. NOT IN A PRIMARY, and the report says so. Departure 7's "26 files and 765
   XML-named entries" and "One corpus file is an .xltx": none of 26, 765 or xltx
   occurs in any log or in the record pair. Departure 7 now labels the whole item
   an unlogged aside, which is the right disposition; I note the figures only
   because the question asks. Within the same item, 781 is measured in run.log
   and 565, 216 and 158 are in specs/0009's "Facts planning checked", which is in
   the diff, so those four are in a primary as planning's facts; the report's own
   recount of them is printed nowhere and the report says as much.
6. NOT IN A PRIMARY, and the report says so. Departure 10's account of the killed
   round ("The killed round's log, which printed the UNEXPECTED line, is in the
   scratch directory ... nothing from it was committed") and departure 6's
   statement that the two timings were taken outside the logs. Both are disclosed
   as unlogged. As noted in answer 1, the timings are in fact corroborated by
   run.log's own per-workbook rows.
7. OPEN PLACEHOLDERS: "TBD_CHECKS" in the status line and "TBD_CHECK" in the
   independent-checks section. They must be filled before the report goes to
   planning.
8. WILL GO STALE. The report's scan.log figures ("30 as expected and 17 yes;
   RESULT: PASS") describe the run that says "check answers scanned: 1". The
   final scan pass, which specs/0001 requires after this answer is saved, will
   scan two answers and will not print the same lines. The scan.log figures, and
   the sentence "Every one was rerun at 4725795", need re-stating against the
   final pass rather than this one.

One wording point rather than a number. "Every message asserted in this log is
read from the committed source at run time, the layer's XML message included."
The usage text and the runner's tab refusal are read with wildcards, but the XML
message is read with a sed pattern that spells the whole message out
(`'s/^ *write!(f, "\(not well-formed XML or not UTF-8 (entry {index})\)")$/\1/p'`),
so the string also lives in the log's own text. The check can still fail, and
would, if the source message changed: the capture would come back empty and "the
XML message format was read from the layer's source and is not empty" would print
UNEXPECTED. So the claim is sound about the assertion and a little generous about
"read, never typed".

Where I disagree with check 1. Nowhere on substance. Its two flagged
discrepancies are both now closed by planning rather than by me: ruling 5 accepts
CRC-32 plus uncompressed size as the fidelity pin for a deflated entry, and the
xml.rs header sentence it caught is repaired. Its structural finding on run.log's
columns and its finding on scan.log's chained `git show` are both fixed, and I
checked the fixes rather than the report's description of them. Its item 4
(placeholders) is still partly open, as item 7 above says.
