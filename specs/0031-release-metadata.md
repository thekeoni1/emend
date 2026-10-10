# 0031: Milestone 4 part 2, the release metadata

Status: APPROVED 2026-10-10, decisions 2 to 4 taken by the maintainer
(A in each, typed in the planning session); decision 1 is asked at
the close-out. Written by the planning
session for the implementing session.

Milestone 4 ("Release readiness", `ROADMAP.md`) ends with the crate
in a state a registry could take: a `CHANGELOG.md` whose every
figure cites a record, the package metadata a registry page shows,
the package's contents listed and its build verified by cargo's own
packaging, and the maintainer's decision, publish 0.1.0 or hold,
asked at the close-out and recorded. Nothing is published in this
part: the act of publishing, if decided, is a later step with its
own spec (`ROADMAP.md`, Milestone 4: "Nothing is published to any
registry in this milestone without a separate maintainer decision
at part 2's close-out").

The part changes no code the probes link: the crate-level doc
comment (decision 3), two files copied within this repository
(decision 4), the manifest's metadata and two documentation files.
The size record expects 0 growth.

## Goal

At the close-out, HEAD holds:

- `CHANGELOG.md` at the repository root, one release section
  "0.1.0 (unreleased)", written from the specs and the records (the
  form below), passing the same citation check as the README;
- `crates/emend/Cargo.toml` with `description`, `readme`,
  `keywords`, `categories`, `repository` per decision 2, the
  docs.rs metadata table, and `publish = false` kept;
- the crate-level doc comment per decision 3; the licence texts in
  the package per decision 4;
- the run logs recording `cargo package --list`, `cargo package`
  (the verify build) with the .crate's byte count and sha256, and
  `cargo publish --dry-run` on a scratch copy of the tree with the
  `publish = false` line removed, the copy's diff printed;
- `records/size-m4p2-<date>-<sha>.md` with 0 of 28 probes moved;
- the maintainer's decision 1 recorded in `ROADMAP.md`.

## Size

The budget figure stays 436992 section bytes (the core probe over
`probe-zip-xml`, `records/size-m4p1-2026-10-10-669fa91.md`), ceiling
458752. Growth expected 0 and accepted up to 0 on every probe: no
line of code the probes link changes in this part. A figure that
moves is a finding, attributed by part 7's per-file method.

## The CHANGELOG (P1)

`CHANGELOG.md`, UTF-8 with no control byte and no U+2014, lines of
at most 80 columns, one top heading "# Changelog" and one release
heading "## 0.1.0 (unreleased)". Under it, four subsections in this
order, each a bulleted list:

- "Added": one bullet per capability, in spec order, naming the
  spec that added it by its repository path (`specs/0008` to
  `specs/0028`): the zip layer; the package layer; reading cells;
  editing cells; styles and the `dates` feature; formulas; a new
  workbook from the template and sheets; the fuller styles; rows
  and columns (`rowcol`); names, links, properties and protection
  (`metadata`); validations and conditional formats (`rules`);
  tables; comments and images (`comments`, `images`); rich text
  (`richtext`); sheet and print settings (`sheetprops`); grouping
  and line styles; read-back (`read`); the worksheet copy (`copy`).
  A feature-gated capability names its feature in backticks, as the
  README does.
- "Changed": the six API changes of `specs/0030` "The API changes",
  one bullet each, "`SharedStringItem`, formerly `Item`" and so on
  for the five renames, and `prefix_len` made crate-private, every
  bullet citing `specs/0030`.
- "Measured": the README's measured sentences, each citing its
  record by path (fidelity, the open count, size, speed): the same
  sentences, so the two files cannot disagree; the run log checks
  each is byte-equal to the README's.
- "Not supported by design": the four openpyxl exclusions in
  `CLAUDE.md`'s words, one bullet.

The citation check (P1's run log), the README's form from
`records/logs/m3p7-p4/run.log` section 1 applied to both files:
every sentence holding a digit cites a path under `records/` that
exists at the phase commit, with these additions for the CHANGELOG:
a path under `specs/` that exists at the phase commit counts as a
citation; a release heading (`^## [0-9]`) is exempt; the exemptions
and the `specs/` extension apply to the CHANGELOG alone. The log
prints the command, the per-file summary line (sentences,
with-digit, cited, uncited, missing) and a control on a planted
uncited sentence and on a planted path that does not exist, each
refused (the negative log).

## The crate-level doc comment (decision 3)

`crates/emend/src/lib.rs`'s `//!` block reads as a milestone
narrative ("Milestone 1 part 1 is the zip layer alone ..."), which
docs.rs would show as the crate's front page. Option A
(recommended): P1 rewrites it as the README's first two paragraphs
(what emend is; the openpyxl comparison with its four exclusions),
the dependency sentence, and one line per feature naming what it
adds, with no figure (a figure belongs in a record, and the doc
comment is not under the citation check); the sentences on the
harness and on `u64` stay. The change touches doc lines only, so
the size log expects 0. Option B: the block stays as it is.

## The licence texts and the README in the package (decision 4)

`cargo package --list` at HEAD lists no README and no licence text:
both live at the repository root, outside the package directory,
and cargo includes neither on its own. Option A (recommended):
`readme = "../../README.md"` in the manifest (cargo copies it into
the package as `README.md`), and `LICENSE-MIT` and `LICENSE-APACHE`
copied byte for byte into `crates/emend/` (this repository's own
files, a copy within the repository, not from another project; the
run log prints `cmp` of each pair). Option B: `readme` set as in A,
no licence text in the package, the SPDX expression alone. Either
way `license = "MIT OR Apache-2.0"` stays from the workspace table
and `license-file` is not set.

## The package metadata (P2, decision 2)

`crates/emend/Cargo.toml` `[package]` gains, in this order after
`publish = false`:

- `description`: "Read-modify-write of Office Open XML spreadsheets:
  open an existing .xlsx, change what you were asked to change, and
  hand the file back with everything else intact." (one line in the
  manifest; the README's first sentence).
- `readme` per decision 4.
- `repository` per decision 2: A (recommended), the GitHub URL of
  this repository, which a registry page links and which the
  maintainer makes public at publication or leaves as it is; B, no
  `repository` until the maintainer publishes. No `homepage` and no
  `documentation`: docs.rs is the default.
- `keywords = ["xlsx", "spreadsheet", "excel", "ooxml", "openpyxl"]`
  (five, each under 20 characters, crates.io's limits).
- `categories = ["parser-implementations", "encoding"]` (slugs from
  crates.io's category list; the run log prints the two slugs and
  the dry-run's acceptance of them, which is the check).
- `[package.metadata.docs.rs]` with `all-features = true`, so docs.rs
  documents every feature. No `doc_cfg` attribute: the gate's doc
  step runs on stable, and every gated item's doc comment already
  names its Cargo feature in words (kept by `specs/0030`'s review).

`publish = false` stays (`ROADMAP.md`, Milestone 4). `version`
stays 0.1.0. `edition`, `license` and `rust-version` stay inherited.

## The packaging runs (P2)

Three commands, each printed with its exit status and its output in
the run log, run on the host with the gate's target root:

1. `cargo package -p emend --list` (offline): the file list, which
   the log compares with the expected list (the manifest, the lock
   file, `Cargo.toml.orig`, `.cargo_vcs_info.json`, every `src/`
   file, every `template/` file, `README.md`, and per decision 4
   the two licence texts): no file more, none fewer. A test source
   under `src/` is in the package as cargo packages it; nothing is
   excluded by `exclude`, since the gate tests the package as it
   ships.
2. `cargo package -p emend` (the verify build, host): exit 0; the
   .crate's path under the phase's target root, its byte count and
   sha256 printed; the .crate's tar listing equal to the list of 1.
   `cargo package` does not read `publish`, so the tree's manifest
   serves as it is.
3. `cargo publish -p emend --dry-run` on a scratch copy of the tree
   at the phase commit (`git worktree` or `git archive` into
   `<scratch>`, the log says which) with the `publish = false` line
   removed, the copy's `git diff` printed (one line removed, nothing
   else): exit 0, the output's warning lines printed and counted
   (0 expected; a warning is a finding). The dry-run needs the
   registry index, so the log prints that the command reached the
   network and nothing else of this phase did. The copy is removed
   by the script that made it (`"${var:?}"`, `specs/0001`).

The negative log: the dry-run on the tree's own manifest refused by
cargo for `publish = false` (the refusal's text printed: the proof
the line holds); `cargo package --list` with a planted `readme` path
that does not exist refused.

## Harness runs

None over the corpus. No line of code the samples run changes in
this part (decision 3's change is doc lines; decision 4's files are
not compiled), so the four samples are not rerun; the P2 run log
prints the diff of `crates/emend/src` between the part 1 close-out
doc commit e4ab25b and the phase commit restricted to non-comment
lines, expected empty (a filter whose control is a planted code
line). The API listing is rerun at P2 with part 1's script
(`<home>/.cache/emend-build/m3p7-p3/gen/api-listing.py` 59f4043c)
and must equal part 1's record row for row: 544 items, 0 changes
(`records/api-review-m4p1-2026-10-10-669fa91.md`).

## Phases

As `specs/0001`: a doc commit first, code commits, one record
commit, the scan last, at most two independent checks per phase, a
report, STOP. One target root per phase,
`<home>/.cache/emend-build/m4p2-<phase>/target`. P1's gate prunes
part 1's P3 set (`dry-gate gate test target` under
`<home>/.cache/emend-build/m4p1-p3`) and the close-out's target
root (`<home>/.cache/emend-build/m4p1-close/target`), each listed
with `du -sb` and removed with `rm -r`, as `specs/0028` P2 ruling 8
and `specs/0030` close-out ruling 10 say. The close-out's
`gen.py` and logs stay. The three logged scripts run in
`<scratch>` copies with `scripts/counting.sh`; a dry run of each
precedes its logged run.

### P1: the CHANGELOG, the doc comment, the licence texts

The doc commit (this spec, `specs/0030`'s close-out rulings and
`ROADMAP.md`'s part 1 rows, both already in the working tree); then
`CHANGELOG.md`, the doc comment (decision 3), the copies (decision
4), each its own commit. Logs: gate (with the prune block), test
(the gate's test counts unchanged from part 1's P3: 514 off,
526 on, 558 cm, 573 pair and 918 all on each target, read from
`records/logs/m4p1-p3/gate.log`), run (the citation check on
both files, the "Measured" sentences byte-equal to the README's,
the `cmp` of the copies), negative, scan. One independent check.

### P2: the metadata, the packaging, the size record

The doc commit (the P1 verification rulings); the manifest commit
(decision 2 and the metadata above); then the logs: gate, test (the
same counts), size (`records/size-m4p2-<date>-<sha>.md`, 0 of 28
moved, part 1's script with its names changed and the change list
printed), run (the three packaging commands, the `src` diff, the
listing rerun), negative, scan. One independent check. The report
asks nothing of the maintainer: decision 1 is planning's to ask at
the close-out.

## Close-out push (after P2)

As part 1's (`specs/0030` "Close-out push (after P3)" and its
close-out rulings), with these differences: the doc commit holds
this spec and `ROADMAP.md`; the base is part 1's close-out doc
commit e4ab25b, named by its subject ("M4 part 1 close-out: commit
planning docs (the P3 verification and rulings)");
`PREV_RECORD_PATTERN` is `^M4 part 1 close-out record` (20285f1),
in the range exactly once; the close-out doc commit's subject is
"M4 part 2 close-out: commit planning docs (the P2 verification and
rulings, decision 1 <publish 0.1.0|hold>)", the bracket replaced by
the maintainer's decision; `CARGO_TARGET_DIR` printed and checked as
`<home>/.cache/emend-build/m4p2-close/target` (part 1's change 22,
kept); no Excel-record check, the stray-record count 0 kept; the
record commit "M4 part 2 close-out record: the push, the CI run and
the scan" holding the three logs only. The generator transforms
part 1's scripts; its numbered changes continue from 23.

Decision 1 is asked by planning after P2's verification and before
the close-out doc commit, and written into `ROADMAP.md`'s part 2
row with its date. "publish 0.1.0" scopes a later step with its own
spec (`publish = false` lifted, the version, the tag, the command,
the registry's page read back); "hold" leaves `publish = false` and
the milestone ends with the push.

## Decisions for the maintainer

Decided by the maintainer 2026-10-10: 2, 3 and 4 as recommended (A
in each), typed in the planning session.

1. At the close-out: publish 0.1.0 or hold. Not asked before P2's
   verification.
2. `repository` in the manifest: A (recommended), this repository's
   GitHub URL; B, none until publication.
3. The crate-level doc comment: A (recommended), rewritten from the
   README with no figure; B, kept.
4. The licence texts and README in the package: A (recommended),
   `readme` set and the two licence texts copied into the crate
   directory; B, `readme` set, no licence text in the package.

## Out of scope

- Publishing to any registry; a tag; a version other than 0.1.0;
  a licence change; the MSRV policy beyond the pin (`ROADMAP.md`
  "Unscoped").
- Doctests or examples in the README or the doc comment (`specs/
  0029` decision 4, option B's cost).
- The Milestone 2 Excel check (skipped by the maintainer 2026-10-09;
  its stage kept; the push script's Excel-record check restored by
  the part whose push the maintainer makes it gate).
- Charts; a second crate; the xlsx-diff, passthrough and
  privacy-gate backlogs under "Unscoped".
- Any code change in `crates/emend/src` beyond doc lines.
