# Independent check 1: M1 part 6 P3

Written by the implementing session. The header is mine; everything after
the rule is the checker's answer. The answer was returned as a message
whose display escaped the characters < and > as HTML entities; this file
writes them back as < and >, and is otherwise the answer as returned,
transcribed by me.

The checker was a fresh subagent (not a fork) with no access to this
conversation. It was given the 18 files below and nothing else, told not
to build or run tests, and asked the three questions CLAUDE.md names. It
was told to write <scratch> in place of the scratch directory.

Commit checked: e0a7b951e493d3bac74fa01e1fb4c96199158ec6
("M1 part 6 P3: probe-emend-formulas-write-xml"),
which was HEAD when the check ran and when this file was written. Range
read: a29c3f9..e0a7b95. Each per-commit diff's producing command is named
beside it below (P1 verification ruling 2).

After the check, gate.log was rerun (attempt 2) for Q2 F13 and
negative.log (attempts 3 to 5) for Q2 F3 and F4; the report names each
change.

The inputs, each as the checker received it (sha256, then the name it had in
the check directory and what it is):

- 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
  0001-bootstrap.md (specs/0001-bootstrap.md at e0a7b95)
- c10c98d18eeb76f621e221577925745b971ea91f44bdb115669c9670afa8c59a
  0013-formulas.md (specs/0013-formulas.md at e0a7b95 (committed in 92dd877))
- 3ca14d6b21cb2e92f09bebfb36ddadbcb591d23f123ef6512003b81782062db9
  diff-61a7d53.patch (made by: git show --format=fuller 61a7d53)
- a6b0efbc1584904bd48dd51735cb5af038312e108540f99703a23d76d1199fa6
  diff-92dd877.patch (made by: git show --format=fuller 92dd877)
- e6f9dbfc1a999ccb5ece7e9ad1146e06dc6e736ca4d624066f06be28f264485b
  diff-code-combined.patch (made by: git diff 92dd877 e0a7b95)
- 4e49d5d07216677a8976b52f89a05d04455a0dd164958fb360ca50e654d88360
  diff-commands.txt (the command that made each diff)
- d27b26c9c868cf0f8b064cac77e8e30cc6263b13cf1099533f97c6ac2785e3a1
  diff-e0a7b95.patch (made by: git show --format=fuller e0a7b95)
- c57981450f486cbee0e7a09f73ad35d659a2a0a4d9f32b2cf2f576868cd2208a
  diff-phase.patch (made by: git diff a29c3f9 e0a7b95)
- aa299f388edebff85756fbb99bcf040c0c5e5e54160f4620ddb904fc3204efc7
  gate.log (records/logs/m1p6-p3/gate.log as installed then; superseded after this check by attempt 2 (Q2 F13))
- 8de6b3012d6d48455898f34b85b0700cd9ac87f95a46eec4cea89966ba2c7699
  m1p6-p3-2026-09-23-e0a7b95.md (records/m1p6-p3-2026-09-23-e0a7b95.md)
- ba3839be1f3cb17b7c8864a235ee6cdb622dd7b574997ec19239fb3dc9f64746
  m1p6-p3-2026-09-23-e0a7b95.txt (records/m1p6-p3-2026-09-23-e0a7b95.txt)
- 64d28694b578c72ed4d6f7314dd7b66962b5dab916633ecd12236602d04afa1b
  negative.log (records/logs/m1p6-p3/negative.log as installed then (attempt 2); superseded after this check (Q2 F3, F4))
- bfe74630bea116e0d58613401557430f2fea159deeab61c34bd86948ce4f71f3
  report-draft.md (the draft report)
- 732391888f5ae6dccea7b3c047627da2be5bcb61c35ddca8689fcf8bee5b51bc
  run.log (records/logs/m1p6-p3/run.log)
- 6bfa43ab8270704a642f1acca3d2d824740835223fe2ba47edfd76291a71c30c
  scan-for-checkers.log (the scan run before this check, FAIL only for want of answers)
- 7e37457a4501a2c973f3cd0870a4256e3a297446bbd307c99943a90dfec5fe2c
  size-m1p6-2026-09-23-e0a7b95.md (records/size-m1p6-2026-09-23-e0a7b95.md)
- d1dee5143d58afa74123a7b6fe3b308fb4f197c92b4cd5a591c7c9bc7aa2c9ec
  size.log (records/logs/m1p6-p3/size.log)
- 79542e1c63694276bbd4ed901e80b6c9ec2782d697f597c2d6929dfd9897075f
  test.log (records/logs/m1p6-p3/test.log)

The answer as transcribed: 10557 bytes, sha256
f6100ac58a3326606e10484ddbe02602f738b60a1921bbd3979734c3632dd952.

---

# Independent check: M1 part 6 P3 (specs/0013-formulas.md "### P3", "Size", "Harness runs", P2 rulings)

Inputs: the 20 files in `<scratch>/p3/check/`, and nothing outside it. I ran no builds or tests. One method note: I compared `0013-formulas.md:584-702` with the `+` lines of `diff-92dd877.patch:33-157` and they match exactly. I also diffed the size record printed at `size.log:3013-3155` against `size-m1p6-2026-09-23-e0a7b95.md` with its 4-space indent removed, and they are identical. Both comparisons used temporary files outside the check directory.

## 1. Does the diff do what the spec's P3 says, and nothing else?

**Yes**, with the low findings below.

- F1 (scope): `diff-phase.patch` touches six files (lines 1, 14, 40, 105, 235, 270): ROADMAP.md, the spec, cells.rs, package.rs, the size-probe Cargo.toml and the new probe. That matches "### P3" (`0013-formulas.md:365-381`) plus ruling 7 of the P2 verification (`:692-697`). gate.log:1159-1166 says the same, and test.log:3436-3445 derives that every changed line in C1 is a comment or blank (47 lines in package.rs, 15 in cells.rs).
- F2 (the doc commit): "### P3" says "P3's first **code** commit carries the P2 verification rulings" (`0013-formulas.md:367`). Ruling 7 (`:694`) instead says the "first **doc** commit carries this spec". The doc commit 92dd877 does it, as ruling 7 says (`diff-92dd877.patch:7-11`, gate.log:1118-1145). The spec contradicts itself here and the work follows the later ruling. Planning should say which one binds.
- F3 (the Limits cover everything required): C1 (`diff-61a7d53.patch:71-111`) covers:
  - the caller's picture;
  - never evaluated;
  - the Excel check pending (:90-91);
  - the refusals;
  - P1 verification F1, the foreign child held (:98-100);
  - P1 verification ruling 4, the two refusal points (:102-108);
  - P2 ruling 3(a) (:109-111).

  The cells Limits (:35-46) state four kinds, and the escaping follows P1 ruling 2.
- F4 (wording, low): at `diff-61a7d53.patch:81-84` the comment says the chain is dropped "(once, ...) whether or not the caller has already recorded it". The spec (`0013-formulas.md:132-133`) says "once, unless the caller has already recorded it". The meaning is probably the same (the recipe is applied once either way), but it reads as though a second drop could happen.
- F5 (claim beyond the ruling, low): `diff-61a7d53.patch:110` adds "a reader counts cells in both". P2 ruling 3(a) (`0013-formulas.md:612-614`) says only "edited in the first and the second copied raw". Nothing in this check set tests or prints the reader's behaviour on two `<sheetData>`.
- F6 (the probe matches "Size"): `diff-e0a7b95.patch:267-303` does the styles-probe work, then `set_cell` of `Formula` from argv on a third cell, then the write. After the write it reads `full_calc_on_load`, `calc_chain` and the cell's formula through `Package::cells`, and prints all three (:162-183). That matches `0013-formulas.md:208-219`. The `[[bin]]` entry is the only functional manifest change; test.log:3509-3512 and gate.log:1187-1197 confirm dependencies and features are unchanged.
- F7 (the probe path, informational): the probe calls `drop_calc_chain()` before the formula (`diff-e0a7b95.patch:254`). So the rule's own chain-drop runs only on its "already recorded" branch. The header comment (:75-77) says only that the chain drop depends on the input. That is not a spec violation, but the probe does not exercise the rule dropping the chain by itself.
- F8 (the probe leaves an empty file, low): the probe creates `second` before `write_to` (:273). So a refused write leaves a 0-byte second output: negative.log:5155 and :5179 ("present, 0 bytes") for L8 and L12, and likewise L9-L11. The library's "no output" rule (`0013-formulas.md:195-196`) is about the library, not the probe, but the log prints this without comment.

## 2. Do the logs contain every line the spec requires?

**Yes**, with the defects below.

- F1 (lines 1-3): in all five logs, line 1 is `command:`, line 2 is `invocation:` and line 3 is `about:` (gate/test/size/negative/run.log:1-3).
- F2 (SERIALIZED lines): each carries the cargo and rustc process counts: gate.log:38, test.log:35, size.log:39, negative.log:38, run.log:44.
- F3 (second roots): gate.log:39, test.log:36-37 (two extra roots, one line each), size.log:40 and run.log:46 each state the spelled path and that it is distinct, with an `apart` control. **Defect (low):** negative.log:40 spells the mktemp root `<scratch>/w` and says it is "distinct from `<scratch>`". It then asserts "neither inside the other" (:42), yet as spelled `<scratch>/w` is inside `<scratch>`. The other logs compare against `<scratch>/p3`. negative.log:41 also gives the primary root a line of its own; that is harmless.
- F4 (build directory spelled once): gate, test, size and run use `<scratch>/p3/target` throughout (for example run.log:43, size.log:2245ff). **Defect (low):** negative.log:3421 spells it `<scratch>/p3/target`, but :3422-3425 and :3430-3433 print `copied target/debug/emend-copy`, a bare relative form. This is the same class of defect as P1 verification ruling 6 (`0013-formulas.md:562-564`).
- F5 (no typed commit id): the phase commit comes in by argument and is checked against HEAD, which specs/0001:199-203 permits. Roles, base, the part 5 commit and the P2 commits are all read from git or from records: gate.log:1065-1074, run.log:71-86, size.log:2310-2313. **Borderline:** size.log:158 types `PREV_REC=records/size-m1p5-2026-09-22-f565868.md`, a file name that contains a short commit id. The commit itself is then read out of the record (:2310).
- F6 (P2's superseded run directories): run.log:71-103 prints each `rm -rf -- <corpus>/runs/...` with a before and after line, and prints `-71e643b-c` as still standing. This meets P2 verification ruling 4. **Weakness:** the superseded set is built from typed suffixes, `STALE="$p2_date-$p2_last $p2_date-$p2_last-b"` (run.log:404). It is never derived by listing `<corpus>/runs`, so any other stale P2 directory would go unnoticed.
- F7 (per-commit diff hashes): gate.log:1147-1156 prints each commit's hash beside the command that produced it. Only the combined-diff hash can be reproduced here: `diff-phase.patch` sha256 c579814...208a matches gate.log:1155. The check directory's per-commit patches were made with `--format=fuller` (`diff-commands.txt`), not gate's `--format=`, so they cannot be matched against gate.log. Stating the command is what P1 verification ruling 2 requires; the report itself does not state it.
- F8 (commit messages that state measurements): C2 says "Measured in P3's size.log, not here." (`diff-e0a7b95.patch:15`). test.log:3525-3529 shows C2 states no count, and :3477-3495 accounts for C1's count words. No message states a measurement that no log prints.
- F9 ("Size"): size.log:2244-2305 gives the twelve probes' figures. :2307-2604 shows all eleven earlier probes rebuilt and reproducing part 5's hashes (11 "yes" lines), classified 6 identical and 5 code, with the causing commits listed. :2609-2658 gives the figures: budget 312812 against the 393216 ceiling, headroom 80404, own cost 968, growth 8400 = 968 + 7432, dates 4980. The record is printed in the log and identical to the committed file.
- F10 (negative log, P3 cases): the two-calcPr case exits 2 with the message through the release `--formula-sample` on host and i686 (L2 and L4, negative.log:5086-5107) and through the release probe (L8). The release binaries are the ones size.log hashed, read out of it by command (:5057-5070). None of the five messages is unreached (:5436-5440), so no "unreached" line is owed.
- F11 (corpus run): run.log uses the release binaries size.log hashed (:2083-2098) and builds nothing. It covers both targets, derives the ruling 1 expectations with a control for each (:2238-2273, :2380-2432), and the record pair's sha256 values match the committed files (:2610-2611).
- F12 (a check-shaped "no" line, low): size.log:2657 prints "... is under 6000: no", but the log's verdict is "all as expected: yes" (:3162). It is a measurement, not an assertion, but it looks like a failed check.
- F13 (template leftover, low): gate.log:1091-1093 labels 61a7d53, which modified package.rs, as "the commit that modified the tool's source". C1 did not touch the tool.

## 3. Does every number and quoted string in report-draft.md appear in a primary?

**No.** Two statements have no primary.

- F1: "size.log and run.log passed at their first attempt" (report-draft.md:71). No log prints this. The only support is that neither log has a `rerun:` line, which is an absence, not a statement.
- F2: item 5 (report-draft.md:72-75), a test copy of run.log's script that ran earlier, "touched no run directory", "stopping before the corpus run". No primary holds this; the report admits as much. That execution is also not named by file and hash in run.log's header, as specs/0001:186-190 and P1 verification ruling 9 require for superseded attempts. Planning should decide whether it counts as an attempt.
- F3 (numbers that match their primaries):

  | Report statement (report-draft.md) | Primary |
  | --- | --- |
  | The four commit ids | gate.log:1078-1080, 1071 |
  | Nine gate steps PASS | gate.log:1370-1381 |
  | 492 / 0 / 2 on both targets; P2 492; no test added | test.log:1935, 2473, 2515-2516, 2544-2545 |
  | full_calc 1 / chain 0 / formula_read_back 1 | test.log:3125-3169 |
  | PM1-PM6 results and the PM3 explanation | test.log:3418-3428 |
  | Twelve probes; 6 identical / 5 code | size.log:2600-2604 |
  | 312812, 80404, 393216, 315392 | size.log:2610-2614 |
  | 968, file 0, .text 800 | size.log:2617-2618 |
  | 304412, 8400, 7432, drift 0, 5 commits | size.log:2626-2651 |
  | 4980 | size.log:2620 |
  | 6000 yes/no | size.log:2655-2657 |
  | The two-calcPr message; all five reached | negative.log:5089, 5436-5446 |
  | 38 / 15 / 9 / 6 / 0 / 19; run directory `2026-09-23-e0a7b95` | run.log:2382, 2230, 2272 |
  | The three P2 directory names | run.log:86-103 |
  | Superseded attempts' hashes | test.log:34; negative.log:37 |
- F4 (paraphrase, not a quotation): PM3's reason, "the value reads back as none either way" (report-draft.md:30-31), is a paraphrase of test.log:987-990. It is not in quotation marks.
- F5 (omission, not a false number): the report gives no sha256 or byte counts for the logs and records, and does not give the command behind each per-commit diff in the check directory. The P2 verification relied on "the sha256 and byte counts the report prints" (`0013-formulas.md:631-633`).
