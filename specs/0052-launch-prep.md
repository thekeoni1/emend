# 0052: Milestone 9 part 2, launch preparation

Status: APPROVED 2026-11-23 (the maintainer, in the planning
session, chose the recommended option A for each of the three
decisions below). The part was approved in outline on 2026-11-14
("i agree with all your recs").

## Goal

A newcomer who lands on the repository can run a program that
uses emend, knows how to report a bug, and knows what the
`specs/` and `records/` directories are. The maintainer has a
draft announcement to edit and post after 0.1.2 is published.

1. Two example programs that the gate compiles and the minimum
   Rust version builds.
2. A short `CONTRIBUTING.md` at the repository's root.
3. Two sentences in the README that point to both.
4. A draft announcement.
5. No file under `crates/emend/src` changes. The budget figure
   stays 456248, a difference a FINDING.

## Facts at drafting (HEAD a031c83)

- `crates/emend` has no `examples` directory and no
  dev-dependency.
- The gate's host and static steps run `cargo build --workspace
  --all-targets`, which compiles a package's examples.
  `scripts/msrv.sh` runs `cargo test -p emend`, which compiles
  them too, so each example must build with Rust 1.89.
- The crate's manifest packages files by an `include` list that
  names no example.
- One fixture is committed: the generated workbook under
  `tests/fixtures/`, 29191 bytes, two sheets.
- The hygiene step refuses a tracked office file outside
  `tests/fixtures/`. It reads tracked paths; an example's source
  holds no workbook path, because each path is an argument.

## Decisions (all A, the maintainer, 2026-11-23)

1. Are the examples part of the published package?
   - A (recommended): no. The `include` list and the manifest
     stay as they are, so 0.1.2's package differs from 0.1.1's in
     the README, the version and the changelog alone. The README
     names the directory in the repository.
   - B: yes, `examples/**` joins the `include` list.
2. What does `CONTRIBUTING.md` say about pull requests?
   - A (recommended): bug reports are welcome; a pull request
     follows an issue that agreed the change, passes
     `scripts/check.sh`, and adds no dependency to the core.
   - B: bug reports only; no pull requests for now.
3. Where does the draft announcement live?
   - A (recommended): outside the repository, a file planning
     hands to the maintainer. Nothing is committed.
   - B: committed under `records/`.

## The examples

Under `crates/emend/examples/`, each one file, each using the
core alone (no feature, no dependency beyond `std`), each with a
`main` that returns a `Result` and prints one usage line when its
arguments are missing:

- `edit_cell.rs`: arguments `<input> <output>`. Opens the input,
  sets B2 of the first sheet to a text, writes the output. It is
  the README's example as a program.
- `new_workbook.rs`: argument `<output>`. Starts from
  `Package::new`, writes a few cells of text and numbers into the
  first sheet, writes the output.

Each file opens with a comment of a few lines: what it does and
the command that runs it. No path with an office extension is
written in either source. Prose in comments follows the
docs-writing rules.

## CONTRIBUTING.md

One screen, plain prose, these sections:

- Reporting a bug: the emend version, the Rust version, the
  target, what was called, what came back. Never attach a
  private workbook. Reduce to a workbook that can be public, or
  give entry names, byte counts and the error text.
- What the directories are: `specs/` holds the decision and the
  acceptance for every change; `records/` holds the measurements
  the README cites, with the logs that produced them.
- Building and testing: the pinned toolchain, `scripts/check.sh`,
  the two targets. It points to the README's section and does
  not repeat it.
- Pull requests, by decision 2.
- Licence: a contribution is offered under the crate's two
  licences, MIT or Apache-2.0.

It makes no claim about fidelity, speed or size.

## The README

Two sentences, no other change: one at the end of "Example"
naming `crates/emend/examples/` and the command that runs an
example; one line before "Licence" naming `CONTRIBUTING.md`.

## The announcement draft

Planning writes it. Every figure in it is one the README already
cites, with the same record. It names the limits ("Out of scope
by design") as plainly as the claims. The maintainer edits and
posts it after part 3's read-back. It is no part of a phase.

## Acceptance

1. The gate passes on both targets with the examples compiled.
   A control shows the host step fails when an example does not
   compile (a copy of the tree, one line broken, never the
   repository).
2. `scripts/msrv.sh` exits 0.
3. Each example runs once on the host and on the static target.
   `new_workbook`'s output and `edit_cell`'s output over the
   committed fixture are both read back by the diff tool. For
   `edit_cell` the log holds the count of entries byte-identical
   to the fixture and names each entry that differs; a differing
   entry that is not the first sheet or the shared strings is a
   FINDING.
4. `git diff --stat` over the code commit names the two
   examples, `CONTRIBUTING.md` and `README.md`, and no other
   file: no manifest, no file under `crates/emend/src`.
5. The budget figure is 456248.
6. No U+2014; the privacy scan passes; every `$` line in a log
   is followed by that command's own output. Every script names
   its tools and checks them before it starts, and no control
   passes on an empty or missing input (`specs/0051` ruling L).

## Phases

By the standing per-phase workflow.

| Phase | Content | Status |
| --- | --- | --- |
| P1 | doc commit (this spec, `specs/0051`, `ROADMAP.md`); one code commit; the gate; `scripts/msrv.sh`; the example runs; logs under `records/logs/m9p2-p1/`; one independent check; one record commit | PLANNED |
| Close | close-out push carrying a031c83 and this part's commits; no tag; both workflows; the post scan; the record | PLANNED |
