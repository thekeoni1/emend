# 0050: the newcomer's intro

Status: APPROVED 2026-11-13 (the maintainer, typed in the planning
session, to planning's four-point proposal and its recommendation
of a standalone docs task: "i agree, and also keep it short and
concise. lets do it now"). Milestone 8 is DONE and PUBLISHED
(`specs/0049`); its three local commits 5df377c, 031732b and
e9dcaac go with this task's push.

The maintainer's complaint, 2026-11-13: half of the README's intro
compares emend with openpyxl, and a reader who does not know emend
cannot tell what it is or why to pick it over other libraries.

## Goal

1. The README opens with what emend is, when to use it, how it
   differs from the Rust crates a reader would weigh it against,
   and one short example. openpyxl is one sentence in the intro.
2. The crate's own doc comment (`crates/emend/src/lib.rs`) opens
   with the same two paragraphs and holds the same example as a
   doc test, so the example is compiled and run by the gate.
3. Nothing else changes: no code, no manifest, no CHANGELOG, no
   other README section. The crates.io page shows the new README
   with the next release; nothing is published by this task.

## The README text (P1)

The README's lines from `# emend` to the line before
`## Dependencies and features` are replaced by the text below,
verbatim, with `EXAMPLE` replaced as the next section says. Lines
are wrapped as here.

````markdown
# emend

emend edits existing Excel workbooks (.xlsx files) from Rust. Open
a workbook, change the cells, styles or sheets you need, and write
it back. Every zip entry the edit did not touch comes back byte
for byte, so the charts, pivot tables, macros and formatting you
never looked at are still there.

## When to use it

Use emend when the workbook already exists and has to survive the
edit: a template someone built in Excel, a report with charts and
pivot tables, a file whose formatting you do not own. emend parses
only the parts an edit touches and copies the rest unchanged. On a
round trip of 27 real workbooks it left 870 zip entries
byte-identical and dropped none
(`records/compare-m6p6-2026-10-24-d88f735.md`).

The other Rust crates do different jobs:

- `calamine` reads workbooks.
- `rust_xlsxwriter` writes new workbooks.
- `umya-spreadsheet` reads a workbook into its own model and
  writes a new file from it
  (`records/decision-architecture.md`, a reading of its 3.1.0
  source; emend has not measured it).

The core depends on `zip` and `quick-xml` and nothing else, and
measures 456248 section bytes over a static 32-bit binary that
already links those two (`records/logs/m8p3-p3/gate.log`). It
streams: a round trip of a 1159829654-byte workbook peaked at
7872 kB of memory
(`records/compare-m6p6-2026-10-24-d88f735.md`).

emend has the scope of Python's openpyxl and the same limits: it
does not evaluate formulas, render, or autofit. See "Out of scope
by design" below, and "What has been measured" for the comparison.

## Example

```rust
EXAMPLE
```
````

Rules for the text:

1. Each figure above is already cited by the README at e9dcaac
   from the same record. The phase's driver proves it: each of
   870, 456248, 1159829654 and 7872 is found in the record the
   sentence cites, the line printed by `grep -n -F`.
2. The `calamine` and `rust_xlsxwriter` lines say only what each
   crate says of itself. The driver downloads each crate's file
   for its newest version in the sparse index, prints the version
   and the file's sha256, and prints by `grep -n` the `description`
   line of its `Cargo.toml`. A description that does not support
   the line (a reader; a writer) is a FINDING and the phase stops.
   The downloads live in the phase's scratch directory, are
   read-only reference material, and nothing from them is copied
   into the repository.
3. The `umya-spreadsheet` line rests on the record it cites and
   says so. No sentence claims a measured result for any of the
   three.

## The example (P1)

`EXAMPLE` is written by the implementing session from the public
API at e9dcaac, under these limits:

- the core only, no feature; at most 15 lines; no `unwrap`;
- it opens a workbook from a reader, sets one cell on the first
  sheet, and writes the result to a writer;
- file names in it are plain placeholders with no office
  extension literal the scan would flag; if the scan's rule makes
  that impossible, STOP and report before choosing;
- the same lines stand in the crate's doc comment as a doc test
  that RUNS (not `no_run`), on an in-memory workbook from
  `Workbook::new()` if that keeps it within the limits, hidden
  set-up lines allowed in the doc test only;
- the README's block equals the doc test's visible lines byte for
  byte; the driver proves it and prints the count of lines.

The crate doc comment's first two paragraphs (the "emend is a
pure-Rust library" paragraph and the openpyxl paragraph) are
replaced by the README's first paragraph and its openpyxl
paragraph, wrapped to the file's width. No other line of
`lib.rs` changes.

## Acceptance

1. `git diff --stat` over the code commit names `README.md` and
   `crates/emend/src/lib.rs` and nothing else.
2. The gate `scripts/check.sh` passes on both targets with the
   doc test run on each; the core's budget figure is 456248, a
   difference a FINDING.
3. The README's "Out of scope by design" paragraph and every
   section from `## Dependencies and features` on are byte for
   byte as at e9dcaac (the driver compares the tails' sha256).
4. No U+2014; the privacy scan passes; rules 1 to 3 above hold.

## Phases

One phase and a close-out, by the standing per-phase workflow.

| Phase | Content | Status |
| --- | --- | --- |
| P1 | doc commit (this spec, `specs/0049`, `ROADMAP.md`); one code commit (README, `lib.rs`); the driver's checks above; the gate; logs under `records/logs/docs-intro-p1/`; README of the record; one independent check; one record commit | logged run VERIFIED 2026-11-13 (record 25ceb42); change 2 (ruling F) pending |
| Close | close-out push carrying 5df377c, 031732b, e9dcaac and P1's commits; no tag; CI; the post scan; the record | PLANNED |

The stop report before the logged run carries the full diff of
the two files, so planning reads the example before it is logged.

## Planning rulings at P1's stop report (2026-11-13)

Given by message at the stop report, since the driver pinned this
file's sha256 (a7d109d9); written here after the record commit.
Planning verified first: doc commit 40ee6b1 on e9dcaac, code commit
4dfd4f4, the diff, dry run d3's last line and its 0 FINDING lines.

A. Spec fact: the constructor is `Package::new()`, not
   `Workbook::new()`. The hidden lines stand.
B. The example stays at the module doc's end under `# Example`.
   Its two over-width lines are accepted; the 15-line limit wins.
C. Change 1, a second code commit (2f7891a, no amend): the crate
   doc comment's second paragraph is the openpyxl paragraph's
   first sentence alone. The sentence after it names README
   sections the crate docs do not have. The README keeps it.
D. No environment line and no prune in this phase.
E. The amend c58ebec to 4dfd4f4 (message wrap, same tree) is
   accepted as disclosed and named in the record's README.

## Planning verification of P1's logged run (2026-11-13)

From primaries: 25ceb42 on 2f7891a on 4dfd4f4 on 40ee6b1; the
record's `verify.log` sha256 fa264d94, last line "verify log
checks all as expected: yes", no FINDING line, no line ending
": no"; `driver.out` exit 0 and stderr 0 bytes; the doc test ok on
both targets; the budget 456248; the remote unchanged at 635aaba.
ACCEPTED, with the independent check's points disposed of so:

- The calamine description test matched "read" inside
  "Spreadsheet" and could not fail. The verdict stands on the
  printed description. Change 2's run tests for "reader".
- The controls the check found missing (the budget's FINDING
  branch, the privacy scan, five small checks) are accepted as
  they are for a docs phase.

F. Change 2 (the check's note, planning's judgement): the README
   said umya-spreadsheet reads "a model of the whole file". The
   cited record does not support "whole": it lists parts the
   source never mentions and a mode that writes unparsed sheets
   from raw bytes. The text above now reads "into its own model
   and writes a new file from it". Steps: a doc commit of this
   spec; one code commit changing that README bullet alone; the
   driver rerun once with the fix named, its log
   `verify-change2.log` beside the first; a short addendum to the
   record's README; one record commit; the scan; report; STOP. No
   second independent check: the change is two lines of spec text.
