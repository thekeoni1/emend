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
