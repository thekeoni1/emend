# 0049: Milestone 8 part 3, the Excel check and the release of 0.1.1

Status: APPROVED 2026-11-10 (decisions 1 to 3 A, typed in the
planning session: "great, i agree with your recs" to planning's
recommendation of A in each); the milestone APPROVED 2026-11-07
(Milestone 8, decisions 1 to 4 A, `ROADMAP.md` "Milestone 8"); part
2 DONE and PUSHED 2026-11-10 at 9f862ca (`specs/0048`), its
close-out record f44debd local.

Milestone 8 part 3 closes the milestone: Microsoft Excel is asked
to open the outputs of the samples this milestone changed, the
release texts and the manifest name 0.1.1, the tag is pushed, the
maintainer publishes from a clean clone of the tag on a machine of
their own, and the publication is read back against the tag. No
code of the crate changes but the version.

## Goal

1. The seven samples Milestone 8 changed (sheet, part 1; delete-rows,
   delete-columns, comment, richcomment, commentbox and remove,
   part 2) rerun on the host over the 39 corpus workbooks at P1's
   phase commit, with the keep of the crate's tests; a stage of
   their outputs carried by the maintainer to the machine with
   Excel; the record `records/excel-check-m8-<utc-date>.md`.
2. Milestone 8 decision 3 proven or refuted: a note written beside
   a threaded comments part opens in Excel without a repair prompt
   (the comment, richcomment and commentbox rows of 15205770ae5c).
3. The release texts: the CHANGELOG's "Unreleased" entries under
   "## 0.1.1 (<utc-date>)"; the README's size sentence at part 2's
   record (456264, 2488 under 458752) and one sentence for this
   check; the manifest's version 0.1.1.
4. The tag `v0.1.1`, the publication by the maintainer, and its
   verification against the tag.

## Size

No code the core links changes. No size record is taken: the
version string is not in a probe's section bytes by any record so
far, and P3's gate builds the probes; if the format probe's figure
at P3 differs from 989480 the gate log says so and it is a FINDING
for planning before the texts cite 456264.

## Facts planning checked (2026-11-10)

- `crates/emend/Cargo.toml` reads `version = "0.1.0"`, holds the
  `include` list of `specs/0045` P3 and no `publish` key.
- `CHANGELOG.md` holds "## Unreleased" with "### Added" (two
  entries) and "### Changed" (two entries) above "## 0.1.0
  (2026-10-29)".
- `README.md` line 70 cites 451512 and 7240 under the ceiling from
  `records/size-m7p2-2026-10-28-43471ed.md`; lines 76 and 78 hold
  one sentence per Excel record (Milestone 7 parts 1 and 2).
- The latest open record is `records/m7p2-open-2026-10-27-43471ed`.
- `scripts/excel-check-list.py` requires `--run`, `--keep`,
  `--record` and `--out`; `--run-plain` repeats.
- The Excel records so far cover the 27 workbooks of their day. The
  12 workbooks Milestone 7 part 3 added have never been opened in
  Excel from an emend output (decision 1).
- Part 2 P4's records (`records/m8p2-<name>-2026-11-10-f625b99`)
  are the latest committed record of each of the 59 runs.
- The tag `v0.1.0` is lightweight (`specs/0045` decision 7 A); the
  maintainer published from the laptop (decision 4 A there).

## The reruns, the keep, the list and the stage (P1)

As `specs/0045` "The second Excel check (P2)" and `specs/0044`
"The reruns (P1)", "The keep and its open record (P1)" and "The
list and the stage (P1)", with these differences:

- The samples are the seven of the goal and the formula sample
  (below), over the 39 workbooks,
  host label only, into `<runs>/<utc-date>-<shortsha>-<name>`,
  recorded as `records/m8p3-<name>-<utc-date>-<shortsha>.md` and
  `.txt`. Each is compared per id on outcome, lo and lo_in with
  part 2 P4's record of the same name: the same 39, changed 0,
  expected; anything else a FINDING, recorded, not fixed.
- The keep is `<phase>/keep-host`; the open record
  `records/m8p3-open-<utc-date>-<shortsha>`, its totals compared
  with `records/m7p2-open-2026-10-27-43471ed`'s. The pair count
  grows by the tests Milestone 8 added; the run log prints both
  totals lines and the difference, which is not a FINDING; a pair
  of the earlier record missing from this one, or an earlier `ok`
  now otherwise, is.
- The list script requires `--run`, the formula sample's
  directory, whose rows carry the references Excel reads back. The
  formula sample is therefore rerun as an eighth, unchanged by
  this milestone, recorded and compared as the seven; its rows
  give the `seen` and `expect` check on all 39. The list:
  `--run` the formula directory, `--run-plain` for each of the
  seven in the usage text's order, `--keep` and `--record` as
  above. No script is changed.
- The stage is `<corpus>/work/excel-check-m8-<utc-date>/`, its zip
  beside it, modes and assertions as `specs/0044` says. Earlier
  stages and zips are not read, removed or moved.
- Decision 1 B adds, after the eight, the other 51 runs over the
  12 workbooks Milestone 7 part 3 added, taken from part 2 P4's
  run directories if they stand on the disk with their
  `run.host.tsv` naming f625b99, else rerun over those 12 alone;
  the run log says which. Their rows enter the list by
  `--run-plain`.

The log prints row counts by source and by label, never a row, a
sheet name, a table name or an original file name.

## The maintainer's check and the record (P2)

As `specs/0044` "The maintainer's check (P2)" and "The record
(P2)" with rulings T, U and W there: the zip carried, the opener
run with Excel closed, `open-results.txt` and the machine section
brought back under the stage; a corpus row the opener refuses is
opened by hand and its outcome noted. Planning assembles
`records/excel-check-m8-<utc-date>.md` by part 1 of Milestone 7's
script and sends its sha256; the implementing session writes it
byte for byte and commits it with the logs.

Acceptance (decision 2 A): the six rows this milestone changed
(delete-rows 90a5b01f9541, delete-columns 91d2bdffa46a, comment,
richcomment and commentbox 15205770ae5c, remove 495c7b79b183) read
`pass`; every other corpus row reads as its row of the latest
Excel record that holds it, or `pass` where none does; a fixture
output refused or repaired on an input Excel accepted, outside the
pairs `specs/0045` ruling C allows, is a FINDING. A FINDING on one
of the six rows stops the part before P3 for the maintainer's
choice, per finding: fixed in a phase inserted here with its
samples rerun, or the change documented as a limit in the README
and the CHANGELOG. Any other FINDING is recorded, named in the
README's sentence, and gates nothing (Milestone 8, part 3's
outline).

The close-out's Excel-record checks gain, by a numbered change,
exactly one `records/excel-check-m8-*.md` with a result cell, with
its control, beside the two that stand.

## The release texts and the manifest (P3)

- `crates/emend/Cargo.toml`: `version = "0.1.1"`; `Cargo.lock`
  follows by cargo's own write, no other line of it changing.
  The packaging dry run as `specs/0045` P3: the `git archive`
  extracted outside any checkout, `cargo package --list` and
  `cargo publish --dry-run --allow-dirty` there, the file count
  equal to the checkout's, `template/_rels/.rels` among them, exit
  0, no token variable set and the log printing them absent.
- `CHANGELOG.md`: "## Unreleased" becomes "## 0.1.1 (<utc-date>)",
  the date written at the close-out doc commit (P3's log checks
  the heading's form; `specs/0045` ruling AB); its "Changed" entry
  on the size sentence names 456264 and
  `records/size-m8p2-2026-11-10-8065795.md`; a "Measured" line
  holds the Excel sentence. The 0.1.0 entry is not touched.
- `README.md`: the size sentence reads 456264 section bytes, 2488
  under the ceiling of 458752, citing part 2's size record; "What
  has been measured" gains one sentence for this check, every
  figure the record's and printed by a command in the run log;
  a limit is added only if decision 2's choice documents one.
- The crate-level doc comment is read against the README's first
  paragraph and left unless a sentence contradicts the release.

## Close-out push (after P3)

As part 2's (`specs/0048`, "Close-out push (after P4)") with these
differences: the base is part 2's close-out doc commit 9f862ca by
its subject ("M8 part 2 close-out: commit planning docs (the P4
verification and rulings)"); `PREV_RECORD_PATTERN` is `^M8 part 2
close-out record` (f44debd), in the range exactly once; the doc
commit's subject is "M8 part 3 close-out: commit planning docs
(the P3 verification and rulings)" and it carries the CHANGELOG's
date; `CARGO_TARGET_DIR`
`<home>/.cache/emend-build/m8p3-close/target`; the CHANGELOG check
reads "## 0.1.1 (<date>)" above an unchanged 0.1.0 entry and no
"Unreleased" heading; the three Excel-record checks; the size
record check drops (none in the range); the size block's label
reads "the adding commit" (part 2's independent check, point 1);
the lightweight tag `v0.1.1` on the tip, created after the pre
checks pass and pushed with the branch in one `git push origin
main v0.1.1` after the maintainer's typed authorisation naming the
branch and the tag (`specs/0045` decision 7 A); the whole-history
private-content scan at the tag, its counts beside the last run's
(`specs/0045` decision 5 A); the post scan over the range. No
prune (`specs/0048` ruling O). The record commit "M8 part 3
close-out record: the push, the CI run and the scan" holds the
three logs only. One independent check.

## The publication and its verification (P4)

As `specs/0045` "The publication and its verification (P4)" with
0.1.1 for 0.1.0: the maintainer publishes from a clean clone of
`v0.1.1` on the laptop with their own token, which no session
sees; tells the implementing session when the registry lists
0.1.1; the run log reads back the crate file and the index entry
(checksums equal), compares every packaged file with the tag's
tree, lists the files cargo adds, reads the keywords and
categories, and reads 0.1.0 still listed and not yanked. A
difference is a FINDING and a stop. The record commit "M8 part 3
P4 record: the publication verified" holds the log and the
README; it and the close-out record stay local until the next
milestone's first push. `ROADMAP.md` then reads Milestone 8 DONE,
PUSHED and PUBLISHED.

## Harness runs

P1: the seven samples and the formula sample over the 39 on the
host; the keep and its open record; under decision 1 B the 51
other runs over 12
workbooks where their directories do not stand. P2: none. P3: the
packaging dry run. P4: none. The scale tiers and the openpyxl
comparison are not run.

## Phases

Each phase as `specs/0001` "Rules for every phase": a doc commit
first, the code commits if any, a dry run and a stop report for
planning's word, the logged gate and run, the logs README, one
independent check, one record commit, the scan last, the report
under `<phase>`, then stop. Target root
`<home>/.cache/emend-build/m8p3-pN/target`. The gate generator is
part 2 P4's with the part's names as a numbered change; P1's gate
prunes part 2's P4 and close-out build targets by the `rm -r`
form with its control; each later gate prunes the phase before
it. Doc and clippy sets as `specs/0048` ruling I.

### P1: the reruns, the keep, the list, the stage

The doc commit holds this spec, `ROADMAP.md` (part 2's close row
and heading, part 3's rows) and the working `specs/0048` (ruling
O and the close-out verification), on f44debd. No code commit.
The logs as `specs/0044` P1 names them. The report prints the
stage's and the zip's paths by placeholder, the row counts and
the zip's bytes and sha256. Then planning's verification; then
the maintainer's check.

### P2: the record

After the maintainer's check. The doc commit; the record written
byte for byte; the logs as `specs/0044` P2 names them; the
acceptance printed per the six rows and in totals.

### P3: the release texts and the manifest

One code commit for the manifest and the lock, one docs commit
for the texts; the logs as `specs/0045` P3 names them.

### P4: the publication verified

After the close-out push and the maintainer's publication. The
doc commit (the close-out's verification); the record commit; the
scan.

## Decisions for the maintainer

To be typed in the planning session; the recommended option is A
in each.

Decided by the maintainer 2026-11-10, typed in the planning session
("great, i agree with your recs" to planning's recommendation): 1
A, 2 A, 3 A. The A branches above apply; decision 1 B's runs are
not made.

1. What Excel is asked to open: A (recommended), the seven changed
   samples over the 39 workbooks and the tests' kept pairs, the
   milestone's outline; B, A and also every other sample over the
   12 workbooks added in Milestone 7 part 3, which no Excel check
   has covered (612 more corpus rows before error rows are left
   out, more opener time on the laptop); C, the kept pairs and the
   six changed rows alone.
2. A refusal or repair on one of the six changed rows: A
   (recommended), it stops the part before the release texts and
   the maintainer chooses per finding, fix or documented limit;
   every other finding is recorded and gates nothing; B, nothing
   gates, as the outline's words, and such a finding is documented
   in the 0.1.1 texts as found.
3. The tag and the publish machine: A (recommended), as 0.1.0: a
   lightweight tag `v0.1.1` pushed with the branch, the publish
   typed by the maintainer on the laptop from a clean clone of the
   tag; B, another form the maintainer names.

## Out of scope

- Any change to the crate's code but the version; the namespace
  residue of `specs/0048` ruling N (under "Unscoped").
- Any new corpus file; any change to the opener or the list
  script without a ruling.
- Removing earlier stages, zips or build roots but by the gates'
  standing prune; the maintainer's deletions are by a method the
  maintainer names.
- Any registry token in any session, file or log.
- Any byte, sheet name, table name, original name or note of a
  maintainer's workbook in a record, log or message.

## Planning rulings during P1

Given 2026-11-11 (UTC) on the stop report after dry run d1
(report sha256 b01ae429c781...; doc commit 4f8fdfc; scripts gate
e6739e5c, negative 5a036ef8, run da45f8af, scan b4fdee87).

A. The negative's NA48 control. The one "no" of d1's negative log
   (line 142) is a driver input, not a script fault: the control
   presumes the working spec differs from the committed blob, and
   the driver named the blob's own hash. The second dry negative
   with the variable unset (f80b1a26, "yes") is accepted as the
   evidence, its cause named before it ran. No script change. The
   logged driver gives PLANNING_SPEC only when the working spec
   differs from the phase commit's blob, and PLANNING_ROADMAP
   likewise, on every step. With this section written the working
   spec does differ, so PLANNING_SPEC is given as the hash
   planning sends with the word.
B. The three FINDING lines of the run (the three pairs of the
   test named below, missing from the keep) are expected again
   in the logged run. The old name:
   `a_shared_formula_before_after_straddling_inside_and_deleted_whole`.
   Cause, checked by planning in the diff of
   2740ce7: that commit renamed the test function to
   `a_shared_formula_before_after_and_deleted_whole`; the old
   name has 3 keys in the m7p2 open record and none in the tree.
   They are recorded with that cause in the logs README, not
   fixed, and they gate nothing. Any other FINDING line in the
   logged run is reported as found.
C. The word. The logged gate (PRUNE_PREVIOUS=yes, removing
   m8p2-p4's and m8p2-close's targets and nothing else), the
   negative, the run, the logs README, one independent check, the
   one record commit, the final scan, the report, then STOP.
   Nothing is pushed. The scan pre-check's four dry-layout lines
   must read as expected in the final scan.
D. Given 2026-11-11 on the final report: the scan log's carried
   sentence at line 6716 ("The two sibling logs ...") is stale
   prose from part 2 P4; the check itself lists and scans three
   logs. It stands in this record as printed and is corrected in
   the next scan script carried from this one.

## Planning verification of P1

Done 2026-11-11 from primaries; P1 is DONE at record commit
c15561b on doc commit 4f8fdfc. Nothing is pushed.

- The record commit holds 24 files, every one under `records/`:
  18 named `m8p3-<name>-2026-11-11-4f8fdfc` and six under
  `records/logs/m8p3-p1` (gate.log 7fc19b19, negative.log
  daf02a4d, run.log d48ede4c, scan.log 762604aa, README
  b4536f7b, independent-check-1 aa9b226c). Trailer as ruled.
- Gate, negative and run each end "checks all as expected: yes";
  the scan ends RESULT: PASS. The gate log's UNEXPECTED strings
  are all in printed script text or the passing control line.
- Both pruned targets are gone from the build root.
- Planning's own per-id comparison of the eight records with
  `m8p2-<name>-2026-11-10-f625b99`: 40 rows each side, changed
  0, missing 0, new 0, in all eight.
- Planning's own read of the open record against
  `m7p2-open-2026-10-27-43471ed`: pairs 2280 against 2251,
  lo_in ok 2268 against 2239, lo ok 2269 against 2240 (+29
  each); gone 3, new 32, none leaving ok.
- FINDING lines: exactly 3 (run.log 4208 to 4210), the renamed
  test's pairs of ruling B; no other.
- The stage directory (mode 700) holds 4850 files: the list
  (sha256 9307b03b..., 4848 rows: corpus 312, fixture 2268,
  input 2268), the opener and 4848 under `files/`. The zip is
  161525935 bytes, sha256 269a8485..., mode 600; the run log
  prints 4850 entries stored, testzip ok.
- The independent check answers yes to all three questions.

Next: the maintainer's Excel check on the laptop (P2).

## Planning rulings before P2

E. The assembly script (2026-11-11, prepared while the maintainer
   runs the check). It is Milestone 7 part 2's
   (`assemble-excel-record-p2.py`, sha256 35f29a5f, itself part
   1's with `specs/0045` rulings X and Y) with these changes, as
   `assemble-excel-record-m8.py` (sha256 0ff31ed9): the title and
   the opening name this spec, the stage, the phase commit
   4f8fdfc, the eight samples and P1's open record; the two
   earlier committed Excel records are read (`--earlier`, oldest
   first) and each corpus row but the six is compared by workbook
   id and sample with its row of the latest one that holds it;
   the six changed rows have their own table; the allowed
   pass/refused fixture pairs are those the Milestone 7 part 2
   record lists, any other such pair a FINDING; the findings and
   the acceptance paragraph are computed from the counts, the
   acceptance reading "Not accepted" when one of the six rows is
   not `pass`; a refused corpus row with no by-hand outcome is a
   new stop (exit 2); the by-hand date is an argument. The
   tables, the machine section, the earlier stops and the
   counting are unchanged. A trial on scratch results (every row
   `pass` but one refused corpus row given a by-hand outcome)
   printed 2655 lines and 2580 table rows, 216 corpus rows held
   by an earlier record; the trial and its inputs were removed.
   Expected from the earlier records: every corpus row `pass`
   but the remove sample's `18d1e8ba72b5`, the documented limit
   (the removed sheet's references), refused by the opener and
   opened by hand.

F. The namespace residue (`specs/0048` ruling N; the maintainer's
   decision, 2026-11-11, on planning's scoping: A). The sample
   saves and reopens between the add and the remove, so at the
   removal the declaration cannot be told from one the input
   carried. It is a documented limit and no code changes: P3
   adds one sentence to the README's limits and one line to the
   0.1.1 CHANGELOG entry, each saying that a removal does not
   undo a namespace declaration, or an emptied relationships
   part, that an earlier addition wrote, and that the result is
   valid. The size cost is 0. "Out of scope" above is read with
   this: the code stays out, the two sentences are in. Dropping
   the emptied part (B) and stripping the declaration (C) were
   not chosen.

G. The check's results and the record (2026-11-12). The
   maintainer ran the opener on 2026-11-12 (Excel LTSC, Version
   2408 Build 16.0.17932.21056, 64-bit; Windows 11 Home
   10.0.26300) and brought `open-results.txt` back under the
   stage (542571 bytes, sha256 b5337dde...): 4848 rows, each
   matching one list row, none repeated, none missing. Planning
   checked the stage's 4848 files against the list's bytes and
   sha256: 0 differing. Results: corpus 311 pass and 1 fail;
   fixture 1905 pass and 363 fail; input 1907 pass and 361
   fail; 725 fail in all; no read or close error; 76 seen
   values compared, 0 differing. The one corpus row refused is
   the remove sample's `18d1e8ba72b5`, as ruling E expected;
   the maintainer opened it by hand the same day and Excel
   offered a repair, so its cell reads `repair (by hand)`.
   The six changed rows read `pass`, 6 of 6. Fixture pairs:
   pass/pass 1883, refused/refused 339, refused/pass 22,
   pass/refused 24, each of the 24 among the allowed pairs.
   Other corpus rows: 216 held by an earlier record, 216
   reading the same; 90 no earlier record holds, 90 `pass`.
   The record is `records/excel-check-m8-2026-11-12.md`, 2655
   lines, 667640 bytes, sha256 e58daaf8..., assembled by ruling
   E's script with the by-hand file (one row, sha256
   b6304939...); its findings read "None" and its acceptance
   "Accepted". The implementing session writes it byte for byte
   from `<home>/.cache/emend-build/m8p3-p2/record-from-planning/`
   and checks the sha256 in the run log. Decision 2 A is met
   on the record; P3 follows P2's verification.
