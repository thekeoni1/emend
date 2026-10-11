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
the phase directory with `scripts/counting.sh` copied into
`<scratch>` by `git show`, as part 1's P3 did (P1 verification
ruling 5); a dry run of each precedes its logged run.

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
rulings, decision 1 hold)", the maintainer's decision in its
subject; `CARGO_TARGET_DIR` printed and checked as
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

## Planning rulings during P1 (2026-10-10)

Asked by the implementing session after the doc commit 28729bb,
before any code commit. Written into the working spec, cited by
the logs by this file's hash, and committed with P2's doc commit;
no second P1 doc commit.

1. "Lines of at most 80 columns" and "byte-equal to the README's
   sentences" cannot both hold literally: the README's five
   measured paragraphs are single lines of 466, 166, 194, 203 and
   412 bytes. Ruling: each "Measured" bullet is wrapped at spaces
   within 80 columns (the section holds no double space), and the
   run log compares the bullet with its "- " dropped and its
   continuation lines joined by one space (the join the README
   check's units() uses) byte for byte with the README paragraph,
   with a control planting a one-byte change, refused. "Byte-equal"
   in "The CHANGELOG (P1)" means this.
2. "Measured" holds all five paragraphs of the README's "What has
   been measured", in the README's order; the copy-sample paragraph
   counts under fidelity. The spec's "(fidelity, the open count,
   size, speed)" named four and reads as five.
3. The `readme` line is added by P1's decision-4 commit, directly
   after `publish = false`, with the two licence copies; P2 inserts
   `description` above it, giving the order "The package metadata
   (P2, decision 2)" lists. Confirmed.
4. "Added": (a) each bullet cites the spec's full file path in
   backticks (`specs/0008-zip-layer.md` and so on), since the
   check's `specs/` extension needs a path that exists at the phase
   commit. (b) The eighteen-bullet mapping as proposed, 0008 to
   0028 without 0014, 0015 and 0023 (harness, open checks and
   scale: measurements, not capabilities); the grouping and line
   styles bullet names `rowcol`, the feature `specs/0026` grows.
   (c) The comments-and-images bullet cites `specs/0022` and adds
   "grown by `specs/0028-images-and-comments-second-half.md`
   (removal, anchors, the comment box)", so both paths are cited;
   the copy bullet cites `specs/0028` for `copy` as listed.
5. Wording as read: "Not supported by design" quotes the README's
   sentence (lower-case "not"); "Changed" bullet 6 reads
   "`Archive::prefix_len` made crate-private, compiled for the
   crate's own tests only" (`specs/0030` P2 ruling 3); bullet 3
   names the crate root's re-export following the rename.

6. The phase scan's denylist form, carried from part 1's P3
   (`f_deny`, a case-sensitive substring `-F`), over-matches one
   line of the Apache licence text, so the copy decision 4 A
   requires would fail the scan while the hook's whole-word form
   (`scripts/privacy.sh`, `-i -w -F`) gives 0 on it, as the
   2026-09-16 count-only ruling found on the same text. Ruling:
   option (a). The scan's `f_deny` takes the hook's form `-i -w -F`
   for every file it reads, as a numbered change printed in the
   scan log's change list, with two controls: a planted denylist
   entry still hits under the whole-word form; the substring form
   run beside it on `crates/emend/LICENSE-APACHE` counts its one
   hit (never printed) as the reason for the change. One form for
   the hook and the scan from here on; the copy is this
   repository's own file, cmp-equal to the root's (the run log).
   No exemption by path (option (b)).

## Planning verification and rulings on P1 (2026-10-10)

Verified by planning from the commits, the logs and its own build:
doc commit 28729bb (three files at the hashes the message names);
139139a adds `CHANGELOG.md` alone; 10685ce changes `//!` lines of
`lib.rs` alone (0 non-doc lines in its diff, counted by planning);
phase commit d41c390 adds the `readme` line after `publish = false`
and the two licence copies, each `cmp`-equal to the root file;
record 5640bfa holds the six files of `records/logs/m4p2-p1/` at
the report's hashes (gate 226de5b0, test 6550e4b9, run 9ca5bf7f,
negative 37229999, check 74e36007, scan eeb2e7fb). Planning's own
build at d41c390 (`<home>/.cache/emend-planning/m4p2-p1-build.txt`):
514, 526, 558, 573 and 918 crate tests on each target by feature
set, 1112 workspace tests on each, clippy 0, doc 0 under
`-D warnings`, `cargo package --list` naming `README.md` and the
two licence texts. Planning's `scripts/privacy.sh --range
20285f1..HEAD`: 5 commits, 1552 blobs, 0 findings. The CHANGELOG's
"Added" descriptions read by planning against the Goal sections of
`specs/0011`, `0013`, `0016`, `0021`, `0022`, `0026`, `0027` and
`0028`: each claim is the spec's.

1. P1 is DONE. The lines ending ": no" in the logs are the
   API-key form ("ANTHROPIC_API_KEY set in this shell: no") and
   the doc commit's descriptive line ("the diff-alone sentence:
   no"), neither a failed check, as in part 1's logs.
2. Check 1's point 2: "Everything heavier is a Cargo feature, off
   by default, with no dependency of its own" is `README.md` line
   18 word for word (read by planning); the checker had no README
   among its inputs. No change.
3. Check 1's point 3: the run log's labels "planning's two files"
   and "planning's uncommitted spec and ROADMAP.md" (lines 57,
   2395, 2396) overstate the pattern (line 1280), which admits
   this phase's logs and `specs/0031` alone; the check is stricter
   than its label and the result stands. Recorded, not rerun. P2's
   scripts label the pattern by what it admits.
4. Check 1's point 5: the size bullet cites part 7's size record
   because ruling 1 makes the bullet the README's sentence; the
   figure is the same and `records/size-m4p1-2026-10-10-669fa91.md`
   confirms it with 0 of 28 moved. No change in this part.
5. Check 1's point 7 and the report's D7: "run in `<scratch>`
   copies with `scripts/counting.sh`" meant what part 1's P3 did
   (the scripts run from the phase directory, `counting.sh` copied
   into `<scratch>` by `git show`); the sentence in "Phases" reads
   that way from here on. Accepted.
6. The disclosures are accepted: ruling 6 asked mid-phase; the
   three run-script defects and the placeholder token fixed before
   any logged run, the superseded scripts kept with their hashes in
   the report (check 1's point 1 answered there); the fork's
   temporary file. No logged run repeated.
7. The gate's prune removed part 1's P3 set (6053758313 bytes) and
   the close-out target (1779553675 bytes), each listed with
   `du -sb`, the kept names present. Planning's own build pruned
   sixteen of its earlier output directories under its cache
   (part 1's P3 outputs and Milestone 2 part 4's), 0.7 GB.
8. P2 starts. Its doc commit holds this spec (these rulings) and
   `ROADMAP.md` (P1's row DONE, P2's IN PROGRESS).

## Planning rulings during P2 (2026-10-10)

Asked by the implementing session after the doc commit 66fd35f and
the phase commit 2c31aad, before any logged run.

1. F1, the packaging finding: `cargo publish --dry-run` on a
   `git archive` copy fails its verify build (exit 101, "couldn't
   read `src/../template/_rels/.rels`"): outside a git checkout
   cargo's file walk skips dotfiles, and `template/_rels/.rels`,
   which `include_bytes!` needs, is left out of the 54 files; in
   the repository the list has 56 and the package builds. Ruling:
   option (a). The dry run runs in a detached `git worktree` of
   the phase commit under `<scratch>` with the `publish = false`
   line removed and `--allow-dirty`, the copy's diff printed (that
   one line), the worktree removed by `git worktree remove --force`
   after the script's own directory check. The run log records the
   failing `git archive` form as a finding with its counts (54
   against 56, the one missing path named): a package built from
   any tree that is not a git checkout is incomplete. Option (b),
   an `include` list in the manifest so packaging does not depend
   on git, is a manifest change beyond "The package metadata" and
   is carried to the publish step's spec (decision 1), which tests
   it by the same `git archive` dry run; `cargo publish` from this
   checkout packages the file, so the finding does not block the
   decision.
2. Q1: cargo's own end-of-dry-run line "warning: aborting upload
   due to dry run" is counted on its own (exactly once); every
   other `^warning:` line is counted with 0 expected. "0 expected"
   in "The packaging runs (P2)" reads as 0 besides that line.
3. Q2: the listing rerun is compared row for row with the record's
   544 table rows (which carry no line column), and with part 1's
   `listing.tsv` by key on every column but `line`; the `lib.rs`
   rows' line shift is printed and checked equal to `lib.rs`'s
   growth since 669fa91 (P1's doc comment), every other file's
   line values unchanged.
4. N1: a dry run validates no keyword or category against the
   registry; the log states the dry run's exit, that no output
   line names a keyword or category, and that this is not the
   registry's validation. "The package metadata (P2, decision 2)"
   reads that way; the registry's acceptance is the publish step's
   to record.
5. N2: `records/size-m4p1-2026-10-10-669fa91.md` line 72 keeps a
   heading from part 7's record ("part 6's record") where part 7's
   is meant: a stale heading in a committed record, recorded here,
   not rewritten (a record stands as history). P2's record names
   part 1's record. The size log keeping the host release
   `emend-copy` build and dropping the builds no P2 log uses is
   accepted and printed in its change list.

Asked after the logged runs and check 1, before the record commit
(logs installed uncommitted; the size record written).

6. Check 1's A: the run and negative logs' end-of-log labels
   ("planning's two files", "planning's uncommitted spec and
   ROADMAP.md", "planning's spec") repeat the P1 template's text
   against P1 verification ruling 3, while the patterns admit this
   phase's logs, the size record and `specs/0031` alone and the
   entry blocks label them so. The checks are stricter than the
   labels and the results stand. Ruling: the labels are corrected
   in the generators and the two logs rerun with C and D below;
   the first attempts are kept outside the repository as
   superseded with their hashes (run cd3feb2f, negative bfeb6e51)
   and disclosed; gate, test and size stand.
7. Check 1's C: the run log's FINDING claims a verify-build
   failure it did not measure (the log counts the list alone; the
   failure was seen in unlogged dry runs). Ruling: the rerun adds
   `cargo package -p emend --offline` on the `git archive` copy,
   its exit status and its error line printed and checked as the
   measured failure (no network: the dependencies from the cache;
   a control is the same command in the worktree copy, exit 0),
   and the FINDING sentence names what the log measures. Ruling 1
   is amended: the missing names are two, `template/_rels/.rels`
   and `.cargo_vcs_info.json`, the second git-only and expected;
   the first is the finding.
8. Check 1's D: the network sentence reads "no other logged
   command of this phase reached the network"; the dry runs that
   did are disclosed in the report.
9. Check 1's B: `<home>/.cache/emend-build/m4p2-p1/target` was due
   at P2's gate under the one-target-root rule, and planning's P2
   brief said "no prune block": planning's error, recorded. P2's
   gate does not rerun for it; the P1 target and P2's own join
   the next gate that builds (the publish step's first gate if
   decision 1 is "publish", otherwise the next milestone's first),
   named in the close-out rulings, each with its `du -sb` line.
10. Check 1's E: the description's text is the spec's quoted
    sentence ("and hand the file back"), a sentence standing on
    its own; "the README's first sentence" in "The package
    metadata (P2, decision 2)" reads "after the README's first
    sentence". No change to the manifest.
11. Check 1's F: the negative log's `git archive` copies (change
    LD, inherited), the HEAD-guard controls' forms (inherited) and
    the local-offset timestamps in a printed diff (the form every
    committed commit date carries) are recorded; no rerun for
    them. Check 2 reads the two reruns; then the final scan, the
    record commit and the report.

## Planning verification and rulings on P2 (2026-10-10)

Verified by planning from the commits, the logs and its own build:
doc commit 66fd35f (two files at the hashes the message names);
phase commit 2c31aad changes `crates/emend/Cargo.toml` alone (the
six metadata lines and one blank, 0 removed; the description above
the `readme` line, the repository URL, the five keywords, the two
categories, the docs.rs table); record 24b927c holds the eight
files of `records/logs/m4p2-p2/` and the size record at the
report's hashes (gate 9e11d942, test bc8c052b, size a445c58d, run
3af3524c, negative f37cdd61, check 1 7f5a2ec7, check 2 0f89d23c,
scan cf3debb1; the size record ce2340fe, 28 of 28 identical,
budget 436992, 21760 under the ceiling). Planning's own build at
2c31aad (`<home>/.cache/emend-planning/m4p2-p2-build.txt`): the
package list 56 entries with one dotfile; `cargo package` exit 0,
the .crate 522253 bytes at sha256 0f1b2ffb, 56 tar entries, equal
to the log's; a `git archive` copy lists 54 and its verify build
exits 101 naming `_rels/.rels`, the finding reproduced; 0 non-
comment lines in `crates/emend/src` since e4ab25b; 918 and 514
tests on each target; clippy 0, doc 0. Planning's
`scripts/privacy.sh --range 5640bfa..HEAD`: 3 commits, 1560 blobs,
0 findings; no session link; no API key.

1. P2 is DONE. The lines ending ": no" in the logs are the
   API-key form, the doc commit's descriptive line and the size
   log's own descriptive lines (overrides absent, growths not above
   the limit), none a failed check.
2. Check 2's D1: the negative rerun's script changed one code
   line beyond ruling 6's labels (`for f in "$B"/*.out` gained
   `"$B"/*.err`, so the control-byte count covers the captured
   standard error, 8 files against 4) while its rerun note and
   change LE say "its cases are unchanged". The change is a
   correction attempt 1 lacked, and it is undisclosed in the log:
   recorded here as the disclosure, with the line (negative.log
   line 1823). No third round: the log's cases and results stand,
   and a rerun would differ by the note alone. The rule stands:
   every line a rerun's script changes is named in its note.
3. Check 2's D5: ruling 7's control is the same command in the
   worktree after the publish line's removal, with
   `--allow-dirty`; "the same command" means in that tree's state.
   Accepted.
4. The reruns' disclosures (three unlogged dry runs and one trial
   reaching the network; the superseded size and run scripts
   before any logged run, by hash; the forks' temporary file and
   killed command) are accepted. Attempt 1 of the run and negative
   logs stays under `<home>/.cache/emend-build/m4p2-p2/superseded/
   attempt-1/` with its SHA256SUMS and NOTE.txt.
5. The packaging finding (ruling 1 as amended by ruling 7) is the
   one item the publish step's spec must carry: an `include` list
   in the manifest, tested by the `git archive` dry run. The
   registry's validation of keywords and categories (ruling 4) is
   the second. Both are written into `ROADMAP.md` under part 2.
6. Pending prunes (ruling 9): `<home>/.cache/emend-build/
   m4p2-p1/target` and `m4p2-p2/target`, each with a `du -sb` line
   at the next gate that builds; the close-out's scripts have no
   prune block.
7. Decision 1 is asked of the maintainer now, before the close-out
   doc commit, and recorded in `ROADMAP.md`'s part 2 row and in
   the close-out doc commit's subject.

## Planning verification and rulings on the close-out (2026-10-11)

Rulings 1 to 7 were written in planning's cache
(`<home>/.cache/emend-planning/m4p2-close-rulings.md`, sha256
12f4e10b) on 2026-10-10 while the tree had to stay clean for the
push script, and are copied here unchanged in substance with the
first doc commit after the record commit, as part 1's were.
Rulings 8 to 10 follow the push.

Verified by planning after the push, from git, gh and the
committed logs: `git ls-remote` gives ac5dc0d for the remote's
main; CI run 38161194914 on headSha ac5dc0d, event push, status
completed, conclusion success, read live with `gh run view` and
again from `ci-run.log` (every step success, 0 other
conclusions); the record commit b0f0b1f holds the three logs only
(`records/logs/m4p2-close/push.log` c552aabb, 1210 lines, its
first 1135 lines byte-equal to pre attempt 2's log 0c692bd1;
`ci-run.log` 0c3a449e, 669 lines; `scan.log` 7621200e, 1547
lines; 0 lines ending ": no" in each; "pre checks all as
expected: yes" and "push checks all as expected: yes"; the hook
scanned 10 commits and 1568 blobs with 0 findings over
e4ab25b..ac5dc0d; git reported e4ab25b..ac5dc0d for main; the
repository still PRIVATE; `scan.log` "total hits: 0", "structural
checks failed: 0", the placeholder checks 0 undeclared on both
logs and on the script's own source, change 27's controls as
expected, "RESULT: PASS"); the maintainer's typed authorisation
recorded verbatim with its date (10/11/2026); the record commit's
message names gen.py f60d8d69, push d24a0aed, ci 126f945a, scan
b7606aaa, changes 23 to 27 and pre attempt 1 (30fa3935, 7df93c86)
as superseded, as ruling 7 requires; planning's own
`scripts/privacy.sh --range e4ab25b..HEAD` over 11 commits and
1571 blobs, 0 findings; no line of the three logs names a home,
mount, temp or corpus path; every `ANTHROPIC_API_KEY=` text in
the range is a script's planted-value control or its print of
"unset"; no commit message in the range carries a session link;
no API key in planning's environment.

Verified by planning: doc commit ac5dc0d (two files at the hashes the
message names, equal to the working copies planning sent, f0c16b71
and 41ea43dc); tree clean; the carried scripts cmp-equal to part 1's
(push 2acfc4f2, ci 9bfc4432, scan 5b491376); the generated scripts
read by diff against them (gen.py 01e6f2d6, push d24a0aed, ci
126f945a, scan 60564fef): bash -n clean, 0 non-ASCII bytes; the base
pattern resolves to e4ab25b alone, equal to origin/main and the
remote's main; the previous record pattern to 20285f1 alone in the
range; the range e4ab25b..ac5dc0d holds 10 commits; the lines that
differ from part 1's scripts are the headers' changes 23 to 26, the
names, patterns, the target root and its other-path control, the
part-1 wording, and item 22's "ruling 2".

1. The close-out scan as generated (change 23 alone) keeps part 1's
   denylist form `f_deny` (`count_from_file -F`, a case-sensitive
   substring), and the range e4ab25b..ac5dc0d adds
   crates/emend/LICENSE-APACHE, on which that form counts 1 line
   (planning's own count, never printed) while the whole-word form
   `-i -w -F` counts 0: the scan would end FAIL at both stages on
   the file decision 4 A requires, exactly as the P1 phase scan
   would have before specs/0031 P1 ruling 6 (change EQ). Ruling:
   the scan script gains change 27, the same as change EQ: `f_deny`
   takes the hook's whole-word form `-i -w -F` for every file it
   reads, with two controls: a planted denylist entry still hits
   under the whole-word form; the substring form run beside it on
   crates/emend/LICENSE-APACHE counts its one hit (never printed)
   as the reason. The generator is amended (its diff kept as
   gen-change27.diff beside it), the scan script regenerated, its
   hash and the generator's reported for planning's word. The push
   and ci scripts as generated are accepted (planning's word on
   d24a0aed and 126f945a); pre mode waits for the scan's
   regeneration so that one set of hashes is printed by change 21's
   hash lines.
2. Noted and accepted: the temp file name "$T/p7rec" (internal,
   never a label); pre mode's printed line 22 naming part 1's root
   as part 1's change-list text, with line 24 below it naming part
   2's; no prune block (P2 verification ruling 6); no Excel-record
   check.
3. Change 27 as generated (gen.py ccb4c01c, its diff gen-change27.diff
   fd2c08b7 equal to planning's own diff of gen.py.before-change27
   against gen.py; scan 5b26c317; push d24a0aed and ci 126f945a
   byte-equal to the accepted ones; read by planning: f_deny with the
   mode -iwF, which count_from_file passes to grep as one argument;
   f_deny_substring kept outside FORMS for one control; the planted-
   entry control under the new form; the licence copy read from the
   phase commit with its byte count and hash printed; its substring
   count 1 and whole-word count 0 as two controls): accepted.
   Planning's word on the scripts; pre mode runs from fresh copies
   under <scratch>/close with CARGO_TARGET_DIR set as change 24 says,
   detached, on the clean tree.

Verified by planning from the files: pre attempt 1's push.log
30fa3935 (1134 lines, 0 lines ending ": no", "pre checks all as
expected: yes"; the hash lines naming ccb4c01c, d24a0aed, 126f945a,
5b26c317; 10 commits; the Excel-record count 0; privacy 10 commits,
1568 blobs, 0 findings); scan-pre.log 7df93c86 ends RESULT: FAIL on
one structural check, line 1519: the scan script's own source prints
<home> (its header item 27, line 14, citing planning's cache file by
path) while its root lines declare <repo> and <scratch> alone; total
hits 0; change 27's controls as expected (planted entry 1; the licence
copy 11358 bytes, substring 1, whole-word 0).

4. Attempt 1 is superseded by the scan's own structural check, not by
   a finding in the range: a placeholder in header text that the
   change-27 wording introduced (the implementing session's error in
   the header; planning read the change-27 lines and did not run the
   placeholder check on them, planning's miss). Ruling: the proposed
   fix. gen.py's header item 27 cites "planning's close-out ruling 1
   (m4p2-close-rulings.md in planning's cache)" with no <home>/ token
   and no other change (the diff kept as gen-change27b.diff beside
   gen.py); the scan script is regenerated (push and ci byte-equal to
   the accepted ones, checked); because change 21's hash lines in the
   pre log name the scan script, pre mode AND the pre-stage scan run
   again from fresh copies as attempt 2, RERUN_NOTE naming attempt 1
   by its hashes. No root line is added to the scan (a code change
   for a comment's sake). Attempt 1 stays under superseded/
   pre-attempt-1/ with SHA256SUMS and a NOTE.txt naming this ruling.
   The hashes of gen.py and the scan script are reported before the
   rerun for planning's word; planning runs the placeholder count on
   the regenerated script itself this time. The independent check
   runs on attempt 2's logs.
5. Ruling 4 applied (gen.py f60d8d69; gen-change27b.diff 00d79c42,
   equal to planning's own diff, the two header lines alone; scan
   b7606aaa; push d24a0aed and ci 126f945a byte-equal; NOTE.txt
   34fda290 in superseded/pre-attempt-1/). Planning's own placeholder
   count on the scan script: path tokens of the form <word>/ are
   <repo>/ and <scratch>/ alone, both declared by root lines; no
   <home>; bash -n clean; 0 non-ASCII bytes. Accepted; planning's word
   for pre attempt 2 and the pre-stage scan from fresh copies, with
   RERUN_NOTE naming attempt 1 by its hashes, then one independent
   check on both logs.

Verified by planning from the check directory's copies: pre attempt
2's push.log 0c692bd1 (1135 lines, 0 lines ending ": no", "pre checks
all as expected: yes"; the rerun line naming attempt 1 by both
hashes; CARGO_TARGET_DIR printed and checked; the hash lines
f60d8d69, d24a0aed, 126f945a, b7606aaa; the base by subject equal to
origin/main; 20285f1 in the range once; the Excel-record count 0 and
the clone's 1; identity 10 of 10, 0 mismatches; privacy 10 commits,
1568 blobs, 0 findings); scan-pre.log 430c897b (1535 lines, 0 ": no",
hits 0, structural 0, both placeholder checks 0 undeclared, RESULT:
PASS); check 1's answer 627a1b59, 0 non-ASCII bytes; origin/main
still e4ab25b, HEAD ac5dc0d, tree clean, no API key.

6. Check 1's Q3 points: (a) the carried scripts' equality with the
   printed text is gen.py's own assert (lines 28 to 45, a primary not
   among the check's inputs) and planning's cmp of the carried copies
   against part 1's files; (b) ROADMAP.md's hash 41ea43dc is
   `git show ac5dc0d:ROADMAP.md | sha256sum`; the superseded path is
   printed in full by no log. All three recorded; the report cites
   the primaries. No rerun.
7. Planning's word on pre attempt 2's log 0c692bd1 is given. The push
   needs the maintainer's MAINTAINER_AUTH and MAINTAINER_AUTH_DATE
   typed in the implementing session; then push mode appended to the
   same log, ci-run.log, the post scan, the one record commit "M4
   part 2 close-out record: the push, the CI run and the scan"
   holding the three logs only, its message naming gen.py f60d8d69,
   push d24a0aed, ci 126f945a, scan b7606aaa, changes 23 to 27, and
   pre attempt 1 (30fa3935, 7df93c86) as superseded.

8. Part 2 and Milestone 4 are DONE and PUSHED (2026-10-11). The
   record commit b0f0b1f is local and goes with the next push,
   whose script's previous-record pattern is `^M4 part 2
   close-out record` (resolving to b0f0b1f alone in its range)
   and whose base is ac5dc0d, resolved by the subject "M4 part 2
   close-out: commit planning docs (the P2 verification and
   rulings, decision 1 hold)". Decision 1 stands: HOLD; nothing
   is published; `publish = false` stays; the publish step and
   its two carried items stay under `ROADMAP.md` "Unscoped".
9. The close-out's independent check (627a1b59) stays in the
   phase cache, as part 1's did. Pending prunes at the next gate
   that builds, each with a `du -sb` line: `<home>/.cache/
   emend-build/m4p2-p1/target`, `m4p2-p2/target` and
   `m4p2-close/target` (P2 verification ruling 6 and ruling 9
   there, extended by the close-out's own root).
10. The record commit's message states that push mode, the CI
    log and the post scan ran from a byte-identical copy of the
    pre-mode scratch directory in the implementing session's next
    scratch after a context clear, the scripts, library and log
    compared with `cmp`. No committed primary holds that
    comparison; it is recorded here as the implementing session's
    statement. What the committed log does show is accepted as
    sufficient: push mode's lines are appended to the very bytes
    of pre attempt 2's log (the 1135-line prefix is byte-equal to
    0c692bd1), push mode re-checks the environment, the key, the
    target root, the hook and the base before pushing, and the
    post scan's structural checks read the scripts' own source.
    No rerun. The implementing session stops.

## Decisions for the maintainer

Decided by the maintainer 2026-10-10: 2, 3 and 4 as recommended (A
in each), typed in the planning session; 1 decided "hold" on
2026-10-10 after P2's verification, typed in the planning session.

1. At the close-out: publish 0.1.0 or hold. Not asked before P2's
   verification. Decided: hold (recommended). `publish = false`
   stays; the milestone ends with part 2's push; the publish step
   (P2 verification ruling 5's two items) stays available under
   "Unscoped" for a later decision.
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
