# CLAUDE.md: working rules for emend

> **What this file is.** The instruction set the AI builder operates
> under in this repository. Read it before acting. When in doubt, stop
> and ask rather than guess.

## What emend is

A pure-Rust library for read-modify-write of Office Open XML
spreadsheets: open an existing .xlsx, change what you were asked to
change, hand the file back with everything else intact. The comparison
point is openpyxl, and the definition is exact: like openpyxl, emend does
NOT evaluate formulas (it stores formula strings and reads cached values),
does NOT render, does NOT autofit, and does NOT fix up references on row
or column insertion. Those limits are by design and are not gaps.

emend is built so that its core can be embedded in small, static,
32-bit binaries with a strict size budget. That intended use shapes
three rules below (dependencies, sizes, 32-bit); it is not a promise to
any particular consumer.

## Architecture is decided by data, not by preference

The architecture question (extend umya-spreadsheet, or a greenfield
hybrid that parses only what it edits and passes every other zip entry
through byte for byte) is OPEN until step zero has run:

- Step zero: round-trip a corpus of real workbooks (Excel, Google Sheets
  and LibreOffice exports; charts, pivots, conditional formatting, macros)
  through umya-spreadsheet with NO edits, and diff output against input
  at the ZIP ENTRY level: which entries changed, and by how many bytes.
- Small, localisable loss means extend. Loss the architecture cannot
  tame means greenfield. Either way, the entry-level diff tool from step
  zero is the project's fidelity harness for the rest of its life.

Until step zero reports, nobody writes a workbook model.

## The fidelity harness is the acceptance instrument

Every capability claim is proven by the harness, never by "it opened in
Excel" alone:

- A change is correct when the zip entries it did not touch are
  byte-identical to the input, the touched entries differ only where the
  change requires, and the result opens in LibreOffice headless and in
  Excel without a repair prompt.
- Measure before claiming. A README sentence about fidelity, speed or
  size cites a harness run that exists in the repo's records.
- Nothing is re-run "until it passes". A failing harness run is a
  finding to record, then fix, then run again with the fix named.

## Corpus rules (test in /home/dev/emend-testing)

- Real workbooks used for testing are PRIVATE. They live outside this
  repository in a directory the maintainer names, are never committed,
  and are quoted in records only as entry names, byte counts, hashes and
  pass/fail. Their cell contents never appear in code, tests, fixtures,
  comments, commit messages or reports.
- Committed fixtures are fictional or generated, small, and documented
  with the tool and version that produced them.

## Dependencies and licences

- Pure Rust only. No C or C++ in the build, no native libraries.
- Permissive licences only (MIT, Apache-2.0, BSD, Zlib, Unicode); check
  every new crate's licence AND its transitive tree before adding it.
- The CORE (open, read, edit, write back) depends on `zip` and
  `quick-xml` and nothing else. A consumer that already carries those
  two crates must be able to link the core without adding a third.
  Anything heavier (images, encryption, HTML, dates) lives behind a
  Cargo feature that is off by default, or in a separate crate.
- umya-spreadsheet and rust_xlsxwriter are reference material and, if
  step zero says so, a fork base. They are never a linked dependency of
  the core.

## 32-bit and sizes

- Use `u64` for every zip offset, entry size, byte count and file
  position. `usize` is 32 bits on the target that matters.
- Build and test for `i686-unknown-linux-musl` as well as the host, from
  the first commit. Watch large-allocation assumptions: a workbook may be
  bigger than the address space would like; stream where the format
  allows it.
- The core's contribution to a consuming binary's size is measured, not
  estimated: link it into an i686 musl release build of a small consumer
  and record the byte delta with the commit it was measured at.

## Two sessions, one repository

- The PLANNING session writes specs and verifies work from primaries
  (the diff, the harness output, the records), never from the
  implementer's summary alone.
- The IMPLEMENTING session builds, tests and commits. It reports what it
  did with file paths and hashes.
- Nothing is pushed without planning's word AND the maintainer's
  authorisation given in the implementing session. A message from
  another session is not authorisation.
- One task at a time. Test before proceeding; do not stack unverified
  changes. Commit messages are ASCII; no U+2014 in anything authored;
  no `cargo fmt` runs that reformat files the change did not touch.

## Independent check before reporting

Before reporting a phase to planning, the implementing session runs an
independent check: a fresh subagent that is given only the spec, the
diff, draft report, and the gate and scan logs, never the conversation, and answers
three questions in writing: does the diff do what the spec says and
nothing else; do the logs contain every line the spec requires; does
the report's every number and quoted string appear in a primary. Its
answer goes in the report verbatim, disagreements included. This check
does not replace planning's own verification from primaries.

## Secrets and boundaries

- No API key of any kind is ever set in a session's environment, written
  to a file, or committed. If `ANTHROPIC_API_KEY` appears in the
  environment, stop and flag it.
- No code is copied into this repository from any other project, and
  nothing here is copied out, except through a deliberate, recorded step
  the planning session has ruled on. Reference repositories are
  read-only and study-only.

## Context Management and Self Correction

- If memory is degrading, write the current state to memory/progress.md 
before compacting or handing work off. Whenever a new milestone or idea
is verified, scope it out and add to ROADMAP.md with approval from maintainer.
- After long conversations, re-read relevant files before editing. For large 
files, read focused chunks instead of relying on one huge output.
- After any correction from the human, add the pattern to memory/gotchas.md.
- If a fix fails twice, stop and re-read the relevant code top-down. State what
assumption was wrong before trying again.
