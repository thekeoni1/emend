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
| P1 | doc commit (this spec, `specs/0051`, `ROADMAP.md`); one code commit; the gate; `scripts/msrv.sh`; the example runs; logs under `records/logs/m9p2-p1/`; one independent check; one record commit | DONE 2026-11-23 (doc e37b276, code 3adfeb1, record b4f38ea, pushed 2026-11-23) |
| Close | close-out push carrying a031c83 and this part's commits; no tag; both workflows; the post scan; the record | DONE and PUSHED 2026-11-23 (7cf1b10; record ce847c5, local) |

## Planning ruling A at P1's stop report (2026-11-23)

Planning read both commits and the first dry run's log. The code
commit was accepted as it stood.

- (a) Planning's brief said the gate needs the root mount's
  propagation shared. It does not: the first dry run passed the
  nine steps with it private. The driver's check became a note
  that prints the reading.
- (b) The broken-example control also requires the failing output
  to name the planted line, `edit_cell.rs:29`, by an exact match.
- (c) The sentence "Every other part of the input is copied
  unchanged" in `edit_cell.rs` stays. It states the design the
  README states, and this phase's record backs it for the
  fixture.
- (d) `set_cell` with a text writes an inline string, so the
  shared strings entry does not change. The acceptance allows
  either entry.

## Planning verification of P1's logged run (2026-11-23)

Read from primaries. The log is 4451 lines, sha256
ca87c4a20337cc28b2771f54cbc86dc6bb7049cc5e9cf2c44eadc975a24d9b3c,
run once on 3adfeb1 with the driver whose sha256 it prints: 71
lines as expected, 0 not as expected, 0 FINDING lines, exit
status 0. Nine gate PASS lines; the broken clone fails the host
step at `edit_cell.rs:29:12`; "PASS msrv 1.89"; `edit_cell` over
the fixture leaves 8 of 9 entries byte-identical on both targets,
the first sheet alone differing; the budget figure is 456248.

The code commit names four files, 110 lines added, none removed,
no manifest and no file under `crates/emend/src`. The record is
commit b4f38ea (parent 3adfeb1), local: the log and a README,
whose sha256 values planning recomputed. Both messages are ASCII
and end with the Co-Authored-By line only. No U+2014 and no home
path. The remote is unchanged at 87b4745. ACCEPTED. P1 is DONE
2026-11-23.

The independent check found no departure. Its notes, disposed of:

- The record's README says each control reads a planted fixture;
  the two tool-guard controls and one empty-file control do not.
  Accepted as a loose sentence; the log is the primary.
- Some `$` lines print a command without its quoting. Accepted:
  the output under each is that command's own.
- Two checks would pass on an empty input ("no PASS host line",
  "no office extension in the examples"). Accepted: each stands
  beside a check that fails in that case. The rule of `specs/0051`
  ruling L covers controls.
- `CONTRIBUTING.md` carries a title and "at the user's option".
  Accepted.

## Close-out

A doc commit of this spec and `ROADMAP.md`; a close script with a
pre mode over 87b4745..HEAD and a post mode, built from part 1's
final script, with these changes: the range and its commit list,
the tool guard requiring a count of tools above zero, and every
control of the pre mode failing on an empty or missing input
(the three kinds part 1's record lists). The pre mode runs the
gate's hygiene step and the privacy scan over the range. Then the
pre report; planning's word; the maintainer's authorisation typed
in the implementing session; one push of main, no tag; the `ci`
and `msrv` runs of the pushed head read by `gh run view` with
`--json`; the post scan; the record, local.

## Planning verification of the close-out (2026-11-23)

Planning read the close script's 206-line diff from part 1's and
checked with its own `command -v` that every tool of both modes
exists, before giving its word on the pre log (549 lines, sha256
a905e05f82e65041ab168e85f9af14568a999f740cff6d7af7713dcf8fb345d3,
57 lines as expected, 0 not).

The push went out once on 2026-11-23, main 87b4745..7cf1b10, five
commits, no tag, on the maintainer's authorisation typed in the
implementing session. Read from primaries after it: the remote's
main is 7cf1b10 and its tags are v0.1.0 and v0.1.1 alone.
Planning's own `gh run view` of runs 45303751433 (`ci`) and
45303751470 (`msrv`) reads event push, the pushed head, completed,
success, one job each completed with success. The post log is 486
lines, sha256
8910c0124e01af09997ca6e435e7ef746ea8759ae79c96df9304fdd903934791:
51 lines as expected, 0 not, 0 FINDING lines, exit status 0,
privacy 5 commits, 3146 blobs, 0 findings.

The record is commit ce847c5 (parent 7cf1b10), local: four files
under `records/logs/m9p2-close/`, sha256 values recomputed by
planning. The message is ASCII and ends with the Co-Authored-By
line only. No U+2014 and no home path. The tree is clean.
ACCEPTED. Part 2 is DONE and PUSHED 2026-11-23 at 7cf1b10; the
record goes with the next push.

The independent check found no departure. Its notes, disposed of:

- The status control shows the planted line reads 14 bytes and a
  missing file reads "missing", without showing the check refuse
  it. Accepted: the check compares with 0.
- Header comments of the script still cite part 1's ruling, and
  three trailing spaces went. Accepted.
- The maintainer's typed words, the push's own output and the
  wait loop are in no log. Accepted as in part 1: the push's
  result is in the post log.

The announcement draft is written and with the maintainer,
outside the repository.
