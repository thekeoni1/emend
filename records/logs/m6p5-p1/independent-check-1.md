1. Does the diff do what the spec says (P1: gradient fills, as amended by rulings A to L) and nothing else?

Verdict: yes, with two minor doubts.

Evidence (`git diff 7359783 029400e`, whose sha256 282f7f84... equals check1/diff.patch, read file by file):
- The doc commit 893e12c holds only specs/0042, specs/0041 and ROADMAP.md. Their blobs hash to 68f67c4a..., 82d246d8... and c927a44e..., as its message states.
- The types match "Types":
  - `Gradient`, `GradientKind::{Linear{degree}, Path{left,right,top,bottom}}` and `Stop`, with the derives the spec names, exported from styles.rs and lib.rs.
  - `Fill` gains `gradient: Option<Gradient>` and loses `Copy`.
- The composer matches "The styles part": `gradientFill`, no `degree` for 0, `type="path"` with all four sides always written, each stop's `position` and `<color>`.
- `check_gradient` checks in ruling E's order: the one-child rule, then the kind's numbers (finite, 0..=360 or 0..=1), fewer than two stops, each position's range and order, and each stop's colour, all with FIELD_OUT_OF_RANGE. In `add_style` it runs after alignment and protection; the pattern colours are checked in the fill step.
- The typed read (`read_gradient`) matches ruling D:
  - `type` absent or `linear` reads as Linear, `path` as Path, and absent numbers as 0.
  - Any finite number is taken; no stop gives an empty list.
  - An unreadable number, a missing position, a missing colour, a second colour or an unknown child gives STYLE_UNREADABLE.
  - The dxf reader goes through the same `read_fill`.
- Ruling B: the three tests are renamed (colors.rs, namedstyles.rs, readback.rs).
- Ruling C, in the core commit's main.rs and null_rewrite.rs:
  - `dxf_gradients` now counts `gradientFill` under a dxf fill and asserts it reads Some.
  - `--read-sample`'s `gradients` now counts fills read with a gradient.
  - The roundtrip None-fill case is kept on an empty `<fill/>`.
  - The null-rewrite figures are dxf_gradients 1, dxf_fills 3, dxf_written 4, dxf_equal 4.
- Ruling A: 16 probe sources gain 17 `gradient: None,` lines (two in the rules probe), and `.clone()` / `.as_ref()` replace the implicit copies.
- `--gradient-sample` (6285028) matches "Harness runs": G1 linear at 90, G2 path at 0.5 x4, two Rgb stops each, two cells at (M+1,1) and (M+1,2) holding Number(7) and Number(8), the read-back asserted, and the one line `edit added ... gradients_before`. Two tool tests come with it.
- The P1 tests in gradient.rs are 12 and cover what the spec lists:
  - both forms, the degree-0 form, the one-child rule and each range;
  - the order test that ruling E names (a border on a part without `<borders>` beside degree 400 gives the collection message);
  - the read of each form, including an empty stop list, and its refusals;
  - the written-then-read round trip and the dxf round trip.
- Rulings I and J: the fix 029400e is one file, +4 -1, `#[cfg(any(test, feature = "rules"))]` on `Fill::compose` plus its doc line. Its only non-test caller is `add_differential_style` (rules).
- Ruling H, checked in git:
  - The trees of 0f8d809 and 6a7644e are both dfbde7d2...; those of 4bb297d and 6285028 are both be8917bb....
  - Each rewritten message is the old one plus the sentence and a blank line.
  - 4bb297d's parent is 0f8d809.
- Nothing for P2 (named styles) is in the diff. There is no U+2014 in the diff, and the commit messages are ASCII.

Doubts:
- (a) package.rs's catalogue entry for "a style field is out of range" already gains the gradient limits in P1. "Errors" and "P2" put "the catalogue entry and limits" in P2. This adds to an existing message, not a new entry, but it is early.
- (b) In `add_differential_style`, the fill's gradient is checked inside `fill.compose()`, after the fill's own colours but before the border's. That fits ruling E's "in a dxf after the colours" only if "the colours" means the fill's colours.

2. Do the logs contain every line the spec requires (the "Harness runs" and "Phases" requirements for P1, and rulings F, K and L)?

Verdict: yes, with one exception. scan.log is not yet written: it is "written last, by the scan" (README), and only scan-pre.log exists. The record commit is not made either: the four records and the logs are untracked in git status. Both are things the "Phases" order says come later, but the report does not say they are still to come.

Evidence:
- gate.log:
  - five roles by full subject and the chain via ~0..~4;
  - ruling H's trees, "FINDING M2 (specs/0042 ruling I), named and not rerun here";
  - "commits in the range: 4", "1 planning-doc, 3 code", distinct messages 155/155;
  - test names 1417/1432 with exactly the three B renames lost;
  - main.rs "lines added: 36; lines removed: 17" and the fix's four lines and one;
  - nine PASS lines, clippy and doc clean on the gate's sets;
  - feature tests on both targets at 523/696/560/738/741/802/1182;
  - "probe sources changed: 16; added lines other than 'gradient: None,': 0; lines removed: 0";
  - charts probe 1140868 (+2516 over 1138352), format probe 974636 (+2460 over 972176), budget 441420 against 438960;
  - the prune with du -sb and `rm -r "${PREV_DIR:?}/${n:?}"`, m6p4-p1 not there, the others 10806750084, 8167491881 and 2118373673 bytes.
- test.log: ws-host and ws-i686 1429; the five feature sets on both targets at 523/629/559/668/1182; the gradient module's 12 tests (8, 3, 1) passed by name per set; the three B names only under their new names; clippy with 0 diagnostic lines on the five sets, the workspace and the probe's emend,xml, charts and images; rustdoc on the five.
- negative.log (ruling K): N16 to N19 each "exit 2, no stdout", with the exact message (STYLE_UNREADABLE, NEEDS_COLLECTION, FIELD_OUT_OF_RANGE twice), plus "no output file"; B11 to B13 as controls; "the 19 cases were refused, N1 to N19"; "output files: 14"; "fixtures written: 18".
- run.log:
  - release builds at the phase commit on both targets;
  - --gradient-sample on 27/27 per label, lo and lo_in ok 27, entry totals identical 816, recompressed 0, changed 54, dropped 0, added 0;
  - 54 of 54 three ways; "gradients_before: 6 in 2 workbooks; 18d1e8ba72b5 4, f9efcac9808f 2"; 27 of 27 outputs byte-identical across targets.
  - Ruling F: three samples times two targets, each "lines equal 27 of 27; fields moved 0", and "total extLst 6 fills 0 dxfs 0 other 0; workbooks 2 of 27".
  - Ruling L: compared against 2026-10-05-6b6125b-roundtrip, with the control line "the directory ruling F named for the round trip, 2026-10-16-5e52758-roundtrip: tool_args host []".
  - The Excel-form fixture through --namedstyles-sample and --gradient-sample on both targets, byte-identical, lo and lo_in ok; the ten checks hold, and the planted stop gives 2 "no".
  - The keep's open record: pairs 2227, lo_in ok 2215, lo ok 2216.
- Each of the four logs prints PLANNING_SPEC 2041984d... and ends "checks all as expected: yes".

3. Does every number and quoted string in the draft report appear in a primary?

Verdict: yes, with exceptions.

Exceptions:
- **A wrong figure.** "The touched files by directory" says "crates/emend/src/package/tests 6". git's numstat shows 5 there: colors.rs, dxf.rs, gradient.rs, namedstyles.rs and readback.rs.
- **An inconsistent grouping.** "crates/emend/src 5" folds in crates/emend/src/styles (elements.rs and read.rs), while package/ and styles/elements/ are listed apart. Read strictly, it is crates/emend/src 3 and crates/emend/src/styles 2. As printed, the counts add up to 34, not the 33 files the report states two paragraphs earlier.
- **Quotes found only in the spec.** "method `compose` is never used" and "`git commit-tree`" appear only in the working spec (rulings I and H). gate.log words M2 as "Fill::compose is never used there", and no log reran clippy at 6a7644e or 6285028, as ruling I intends.
- **A session's statement that no primary carries.** "Nothing pushed (session's statement)" is not in the README. git's local tracking ref agrees (main is "ahead 5" of origin/main, which is f630f67), but only as of the last fetch.
- **A small wording difference on M3.** The report says "the facts file and ruling F, on the session's word". The README says "the facts file and the session's proposal". Ruling F's text does name 5e52758 for the roundtrip, so the substance holds.

Every other number, hash and quoted string I checked matches its primary:
- the four commit hashes and subjects;
- the numstat table, and 33 files, 1721 added, 92 deleted;
- the 16 files and 17 lines;
- the tree hashes;
- the hashes of the eight log and record files and of scan-pre.log, which ends "total hits: 0", "structural checks failed: 0", "RESULT: PASS";
- the four "run ended at" times;
- the gate, test, negative and run figures listed under question 2;
- the fixture's id, size and sha256, and its two sample lines;
- "exactly the 12 this phase's tests write" and "exactly the 2 ruling C names";
- the 2213/2215 record rows;
- the dry runs d1, d1b and d2, gen1 71f4b045..., and the "session's statement" items on first logged runs, outer stderr 0 bytes, M1 to M3 and ruling K's correction, all of which README.md carries.
