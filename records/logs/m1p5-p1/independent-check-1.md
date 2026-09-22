# Independent check 1: M1 part 5 P1

Written by the implementing session. The header is mine; everything after
the rule is the checker's answer, byte for byte as it was returned.

The checker was a fresh subagent with no access to this conversation. It was
given the twelve files below and nothing else, and asked the three questions
CLAUDE.md names.

Commit checked: abf2618530548627b659c0b6fce93d4370c4e6f7
("M1 part 5 P1: style additions, style edits and the styles rewrite"),
which was HEAD when the check ran and when this file was written.
Range read: 7cc4f13..abf2618.

The inputs, each as the checker received it (sha256, then the name it had in
the check directory and what it is):

- 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
  0001-bootstrap.md (specs/0001-bootstrap.md at abf2618)
- 7d38541fce40680dfa1abd076a5565283b7e11233ddae896a298ae49555fdaab
  0012-styles.md (specs/0012-styles.md at abf2618, the P1 rulings)
- 40cb83c8409158ee095c92e1fd8069aea21485b4daca63de34b7055a093b0671
  diff-doc-1.patch (7cc4f13..489d4e2)
- 0431de36fb57f17291dfa139928f444d8b9d39e4573f8fc07babb64780fc5c90
  diff-doc-2.patch (489d4e2..cbc691b)
- 8307ec7f4956566d12d169b97822589d15a0494cee9edd2e56e6d9e7684a3f29
  diff-code-1.patch (cbc691b..35cd499)
- 2b16b0000799a2e272493ea301f0ef2a46b7066f681407f4462fe911fc89f829
  diff-code-2.patch (35cd499..abf2618)
- f3bff6d45762b5a001369707a17f3d02706053e1cef61ecb331302da60712a53
  diff-combined.patch (7cc4f13..abf2618)
- 7a7444e2a82c4dfc4c4e3bb9dee3a45e8f5bee9d9382847e5e4972ed6782b02c
  gate.log (records/logs/m1p5-p1/gate.log)
- 5ebb012ba07380d3bf50232f8d96ba90360d1b9cb63b741a34cd2143cf8d3c5d
  test.log (records/logs/m1p5-p1/test.log)
- 43877889ddf1d6f3f5a55437205cefd4ecde05626a00ac189ca18ed888371af3
  negative.log (records/logs/m1p5-p1/negative.log)
- 456001288f3c1e582ca3094ee29fa7db90d56102fe68d568e0f7a3292d8dfb36
  run.log (records/logs/m1p5-p1/run.log)
- f7ea84b8d4a923192a6d527f5d396c8e98195d4550e53c6b50a61ef0cc64501a
  scan.log (the pre-check scan run, superseded by the committed scan.log;
  the checker says so itself in its finding F8)
- f698ec32d2e88a6bc71c1dc9cebe7c72b702cef0990b4e46a1cd4c1a075cd029
  report-draft.md (the draft report as it stood before this answer; the
  report sent to planning is later and answers the findings)

The answer as returned: 22289 bytes, sha256
60d1fb303bd912666ad5059f6210276e2175b6c0d386f3977ae5e56b0bce405f. Every byte
of it is ASCII and none is a control byte.

ONE substitution was made, and it is the only difference between the text
below and the answer as returned. The checker's disclosure sentence, the
first line of its answer, quoted this repository's own working-copy path to
CLAUDE.md, which is under this machine's home directory. specs/0001 forbids
that path in a committed file and the phase's scan counts it, so the 25
bytes of the absolute path were replaced by the 16 bytes <repo>/CLAUDE.md,
once, at offset 93 of the answer as returned. Nothing else was touched; the
sentence's meaning is unchanged. The answer text below is therefore 22280
bytes, and the answer as returned is recoverable from it by putting the path
back.

Its "&lt;" and "&gt;" are the checker's own spelling of the angle brackets.

---

## Independent check: M1 part 5 P1 (7cc4f13..abf2618)

Disclosure first: the harness placed `<repo>/CLAUDE.md` (the project instruction file) and several memory-file summaries into my context as system reminders. I have not used them as evidence for anything below; every citation is to one of the files in the check directory.

### 1. Does the diff do what the spec says for P1, and nothing else?

Yes for both code commits, with three observations at the end of this section.

**Which diff I read for each "nothing else".**

- For code commit 1 (35cd499) I read `diff-code-1.patch` in full. It touches seven files: `crates/emend/src/lib.rs`, `crates/emend/src/package.rs`, `crates/emend/src/package/tests.rs`, new `crates/emend/src/styles.rs`, new `crates/emend/src/styles/tests.rs`, `tools/emend-copy/src/main.rs`, `tools/emend-copy/tests/null_rewrite.rs` (diff-code-1.patch:1, 28, 214, 260, 650, 1057, 1225). Every hunk is one of the items 0012-styles.md:513-526 lists. I confirm "nothing else" for commit 1 from that diff.
- For code commit 2 (abf2618) I read `diff-code-2.patch` in full. It touches four files: `cells.rs` (one visibility line), `package.rs`, `package/tests.rs` (a pure append, `@@ -3243,3 +3243,651 @@`), `styles.rs` (additions only). I confirm "nothing else" for commit 2 from that diff, subject to observations (a) to (c) below.
- `diff-doc-1.patch` carries only ROADMAP.md, specs/0011 and the new specs/0012 (diff-doc-1.patch:1, 52, 97); `diff-doc-2.patch` carries only specs/0012 (diff-doc-2.patch:1). `diff-combined.patch` is the union of the four and adds nothing the per-commit diffs do not have.

**P1 first code commit, item by item (0012-styles.md:513-526).**

- `Workbook::date1904`: field diff-code-1.patch:93-98; read in `parse_workbook_sheets` at diff-code-1.patch:199-201, `1` or `true` only, first `&lt;workbookPr&gt;` at root depth, `false` when absent (diff-code-1.patch:210). Test `date1904_reads_one_and_true_and_nothing_else` covers `1`, `true`, `0`, `false`, the attribute absent and `workbookPr` absent (diff-code-1.patch:949-964). The spec asks for five of those six; the extra `false` case is harmless.
- `Styles`, `NumFmt`, `Xf`, `NumberFormat`, `number_format`: diff-code-1.patch:342-414. `Package::styles` with the once-and-kept cache: diff-code-1.patch:150-166; test `the_part_is_read_once_and_kept` diff-code-1.patch:910-915.
- The five reading messages with catalogue entries (ruling 2): catalogue diff-code-1.patch:56-73, constants diff-code-1.patch:530-534 and 156. The set-equality test is extended to `styles.rs` in the same commit (diff-code-1.patch:222-256), as ruling 2 requires.
- Tests the spec names: five ids present and absent (783), escaped quote and escaped `&lt;` (805-825), `&amp;#9;` read as a tab and a literal tab read as a space (828-844), second `numFmts` and second `cellXfs` refused (1036, 1049), part without `numFmts` (847), part without `cellXfs` (854), an `xf` with children and one empty (861), prefixed root and strict namespace (892), each refusal (969-1055), `number_format` for a custom id, a producer-spelled built-in, a built-in with no `numFmt`, an `xf` with no `numFmtId`, out of range (920-944), and the tool's line on a fictional workbook (1291-1318). All present.
- `--styles`: usage, operation, no-edit path and the line in the spec's key order at diff-code-1.patch:1085, 1102-1104, 1112, 1123-1127, 1142-1167. Counts only, no code or font name printed (1140-1141, 1164-1166).

**P1 second code commit, item by item (0012-styles.md:528-568).**

- `add_cell_style` diff-code-2.patch:241-304; `set_cell_style` diff-code-2.patch:316-320; the styles rewrite `rewrite_styles` diff-code-2.patch:456-571 called once per write at diff-code-2.patch:346-366; the worksheet start-tag rewrite `restyled_tag` diff-code-2.patch:647-663 used at diff-code-2.patch:742-758.
- The other five messages with catalogue entries: diff-code-2.patch:25-42 and constants diff-code-2.patch:416-423.
- The validation the spec asks for at write: index below count plus additions (diff-code-2.patch:381-391), styles part not removed and not replaced (392-405), `cellXfs` absent refused once the stream ends leaving no output (567-569), `count` not a digit string refused (diff-code-2.patch:1612-1627).
- The id rule: `BuiltIn(id)` adds nothing; `Code` matches an existing `numFmt`, then an earlier recorded code, else the largest of 164, part-max+1 and recorded-max+1 (diff-code-2.patch:258-293). Codes written in id order (diff-code-2.patch:355-356), `xf`s in record order (diff-code-2.patch:481).
- Tests the spec names, all present in `package/tests.rs`: base with children, tag rebuilt, `numFmtId` replaced, `applyNumberFormat` replaced when present and appended when not, and a base without `numFmtId` gaining one first (diff-code-2.patch:866-894); single-quoted base keeping each quote (897-911); empty base and `None` (914-935); tab, line feed and carriage return written as references and read back equal (1001-1020); new code 164 on a part with no custom id (946-962); code matched to an existing custom code, to a spelled-out built-in below 164, largest+1 otherwise, and two additions with equal codes giving one `numFmt` and two `xf`s (965-998); a `BuiltIn` adding nothing (914-935); counts rewritten, a container without `count`, empty `&lt;numFmts/&gt;` and `&lt;cellXfs/&gt;` opened up (1023-1048); a part without `numFmts` gaining one before `fonts` (871, 886) and before `cellXfs` on a part with `cellXfs` alone (1051-1063); a part without `cellXfs` refused at write with no output and no temporary (1175-1194); a prefixed part's new elements prefixed (1066-1103); every byte outside the two containers kept on a part with `fonts`, `dxfs` and `extLst` (1106-1136), the same test also pinning the existing children byte for byte and, at 872-873, a base whose spacing a rebuild would change; value edit with a style, with `None`, and a styled cell with no style edit (1204-1224); a clear with `Some` and with `None` (1227-1244); style edit alone on a formula, an inline string, an empty styled cell and inside an array `ref` (1247-1272), on a formula leaving the chain (1275-1291), on absent cells in an existing row and a new row and `None` writing nothing (1294-1311); last style edit wins and a later value edit joins it (1314-1328); `Some` refused with no styles part and `None` accepted (1331-1350); index at count-plus-additions refused and below it accepted (1353-1372); style edit to a replaced part, addition on a replaced and on a removed styles part (1375-1400); the sample's read-back through `Package::styles` and `Package::cells` (1403-1441).

**P1 rulings 1 to 5.**

1. Date-like test: implemented in the tool, over the whole code, quoted runs, bracketed runs and backslash escapes removed, documented as the survey's test and no library's (diff-code-1.patch:1169-1220). Mutations R1a and R1b pin it (test.log:2468-2501).
2. Message split: five reading messages catalogued in commit 1 (diff-code-1.patch:56-73), the other five in commit 2 (diff-code-2.patch:25-42); the set-equality test reads `styles.rs` from commit 1 (diff-code-1.patch:224, 230, 256). test.log:2711-2713 measures this at the two commits: "the reading's catalogue items: at 35cd499... 5, at its parent 0" and "the additions' and the write's catalogue items: at abf2618... 5, at its parent 0".
3. `normalized_value(XmlVersion::Implicit1_0)` for `formatCode` (diff-code-1.patch:555-561); writing tab, line feed and carriage return as references after `escape` (diff-code-2.patch:1590-1601); round trip pinned at diff-code-2.patch:1001-1020; mutation R3 (test.log:2523).
4. Second `numFmts` or `cellXfs` refused with the tenth message, two tests (diff-code-1.patch:452-467, 533-534, 1036, 1049); mutation R4 (test.log:2543).
5. Each attribute keeps its own quote character (diff-code-2.patch:1546-1551), one test (diff-code-2.patch:897-911); mutation R5 (test.log:2565).

There is no "R2" mutation; the labels track the ruling numbers and ruling 2 is a commit-content rule, checked instead by test.log:2701-2718. That is coherent, not a gap.

**The "Errors" split of the ten messages.** Confirmed as above: five in commit 1, five in commit 2, each with a named pinning test, and the catalogue in `package`'s header holds all ten (negative.log:3342-3399 lists every one of the ten by text). Commit 2's header also records that two of the reading's five are now raised from the write as well (diff-code-2.patch:25-30), with the tests that pin those paths; that is a true statement about the new code, not an extra message.

**Observations (things I would put to planning rather than call correct or incorrect).**

(a) Commit 2 adds the "Consequences a caller should know" bullets to `package.rs`'s module header (diff-code-2.patch:65-82). 0012-styles.md:582-593 schedules "the catalogue and Limits in `package`'s, `styles`'s and `dates`'s module headers" for P3. The text matches the spec's Design section word for word in substance, so nothing new is claimed, but it is P3 work landed in P1. Commit 1 likewise carries a `# Limits` block in `styles.rs` (diff-code-1.patch:315-325).

(b) `cells::forbidden_in_xml` becomes `pub(crate)` (diff-code-2.patch:9-13). The spec asks for the check ("a character XML 1.0 forbids (the set `specs/0011` names)", 0012-styles.md:180-182) but does not name the visibility change. The report discloses it as decision 3, and test.log:2716 measures it at the commit and at its parent.

(c) `Styles` gains a private `has_num_fmts` field (diff-code-2.patch:1450-1452). The spec's shape is `Styles { num_fmts, cell_xfs }` (0012-styles.md:123). The field is not public and is what the "a part with no `numFmts` that gains a code" rule needs; I read it as an implementation detail of a rule the spec does state, not a departure.

(d) Smaller point: `rewrite_styles` creates a missing `&lt;numFmts&gt;` before the first root child whose name is in `AFTER_NUM_FMTS` in *document* order (diff-code-2.patch:465-466, 505). The spec says "the first root child among `fonts`, `fills`, `borders`, `cellStyleXfs` and `cellXfs` in that order, the schema's order" (0012-styles.md:256-259). The two agree for any part whose children are in schema order, which 0012-styles.md:62-66 says all 27 corpus parts are, and for every test fixture. They could differ on a part that is not in schema order.

(e) One thing I could not confirm from the diffs alone: the new `_` arm of `Pending::write_row` (diff-code-2.patch:635) now calls `replaced_cell` for a pending clear that has no style edit, where part 4's code wrote nothing. `replaced_cell`'s body is not in either code diff (the hunk header at diff-code-2.patch:639 shows it unchanged), so I cannot read from the diff that `replaced_cell(reference, None, None, prefix)` emits nothing. Indirect evidence that it does: no part 4 test was removed or changed (test.log:1997 "test names removed since the previous phase: 0") and both targets pass 441 tests (test.log:1423, 1910).

### 2. Do the logs contain every line the spec requires?

Yes, for the five logs I was given, with one stated exception about scan.log.

- **Phase commit given and compared with HEAD, with controls, both values printed** (specs/0001, 0001-bootstrap.md:208-212). gate.log:1-8; test.log:1-2, 10-13; negative.log:1-2, 9-12; run.log:1-9; scan.log:1-6. Each prints the given commit and `git rev-parse HEAD` and at least two controls (the null commit and a short sha of the real commit).
- **Superseded attempts hashed from files that exist** (0001-bootstrap.md:185-191). test.log:4-6 (`61b46327... 184536 bytes test-attempt-1.log`), negative.log:4-6 (`bbb176c5... 215458 bytes negative-attempt-1.log`), run.log:11-13 (`f2953e62... 69296 bytes run-attempt-1.log`), each under a printed `command: sha256sum and wc -c` line. gate.log and scan.log carry no rerun header, so nothing is due there.
- **Counting controls.** gate.log:730-755 ending "counting controls: all as expected"; test.log:939; negative.log:2208; run.log:889; scan.log:890-891.
- **No control bytes, with a positive control** (0001-bootstrap.md:104-109). scan.log:906-907 gives the positive control (`f_ctrl on ESC, ESC and SI count=3`) and `f_ctrl count=0` for each of gate.log, test.log, negative.log, run.log (scan.log:1135, 1145, 1155, 1165) and both record files (scan.log:1180, 1191). test.log also prints "control bytes in the cargo output: 0 (expect 0)" for each target (test.log:1426, 1913).
- **Gate files.** gate.log:807-813 lists the ten gate files checked against the diff, finds none in it, states that no self-test is due and that none is committed, with a control that the same search does find a name the diff holds. That is what 0001-bootstrap.md:113-122 asks of a phase that changes no gate file. scan.log:1195-1199 repeats the measurement.
- **The negative log's families.** Part 5's own family is there: N49 to N55 for `--styles` parsing, including the usage line and the accepted form (negative.log:2752-2795); Y1 and Y2 the accepted `--styles` with the fixture's hand-counted line and the output byte-identical (2935-2949); Y3 to Y7 the five reading messages each through the built binary, each exit 2, one stderr line, no output, no temporary, no styles line (2950-2991); Y8 the five addition and write messages named from the source beside the package tests that raise them (2992-3006). The catalogue accounting at negative.log:3342-3399 covers all ten new messages and states that 16 messages are test-only, which is where the five of Y8 sit. All earlier families (N, E, F, S, D, T, U, C, X, R, G) are re-run.
- **The corpus run's acceptance for P1** (0012-styles.md:456-461). Both targets: 27 summary rows, outcome, `lo` and `lo_in` ok in 27, leftover no in 27, `order_preserved` and `comment_equal` yes in 27, `files_identical` yes in 27 (run.log:1086-1087 host, 1201-1202 i686); entry totals identical 870 with recompressed, changed, dropped, added and flagged all 0 (run.log:1088, 1203); every entry identical and unflagged and every output the input's bytes (run.log:1160-1162, 1275-1277); the tool's totals `xfs 992; num_fmts 109; custom 100; dated 106; date1904 2` with 18 workbooks having dated above 0 (run.log:1157, 1272); the log's own zipfile and ElementTree count agreeing on every key in all 27 on both targets, with its own totals equal to the tool's (run.log:1158, 1163-1167, 1273, 1278-1280), plus a planted-difference control on each target (run.log:1166-1167, 1281-1282). The run also records the operation in both run rows (run.log:1291-1297) and the record pair with its byte counts and hashes (run.log:1302-1305).
- **The scan's self-scan and the three shape greps.** Self-scan of the script's own source before it is printed, count 0, with a positive control on a copy carrying one concatenated shape (scan.log:16-27). The three forbidden shapes, each pattern assembled at run time, with six positive and six negative controls, run over all four sibling logs, the scan's own source and both record files: 7 files, 0 instances, 0 errors (scan.log:1229-1259). The pre-P6d bare-shape control runs over all 11 diff files and the logs and records: 17 files, 0 hits (scan.log:1204-1227). Commit messages: four scanned, four commits in range, each with the Co-Authored-By line last and zero non-ASCII, zero session-link hits (scan.log:1078-1123).
- **Exception, as my brief says.** scan.log ends `RESULT: FAIL` with "structural checks failed: 1" (scan.log:1258-1262), the failing line being "the check answers went through all three greps as prose: no" with "check answers scanned: 0" (scan.log:1200, 1257). That is the pre-check run and is expected; I cannot verify the committed final scan.log, which does not exist yet.
- One small thing I can only infer: gate.log's status as a first attempt rests on the absence of a `rerun:` line (gate.log has none), not on a positive statement.

### 3. Does every number and quoted string in the report appear in a primary?

Almost all of them do. Verified:

- Commit list and roles: gate.log:768-776 prints the four commits and their classification; the file list and per-file hashes are gate.log:795-806. specs/0012 sha256 `7d38541fce40680dfa1abd076a5565283b7e11233ddae896a298ae49555fdaab` (report:13) is gate.log:801.
- Log sizes and hashes (report:25-32): gate.log 61097 / `7a7444e2...` at scan.log:1133; test.log 187704 / `5ebb012b...` at scan.log:1143; negative.log 217466 / `43877889...` at scan.log:1153; run.log 75629 / `45600128...` at scan.log:1163.
- Record pair (report:35-38): `.md` 5650 bytes / `93cb2240...` and `.txt` 198280 bytes / `08e33db5...` at run.log:1304-1305 and scan.log:1173, 1184.
- gate.log claims (report:40-41): "nine PASS lines: as-expected" gate.log:1027, the nine step names gate.log:1015-1023; commits classified gate.log:772-776; "files in the diff: 11; files scored as new or modified: 11" gate.log:1009; "gate log checks all as expected: yes" gate.log:1052.
- test.log claims (report:42-48): 441 passed, 2 ignored on each target at test.log:1423 and 1910, and both targets executed 441 at test.log:2697; 394 at part 4 P3 and 47 names added, 23 package, 22 styles, 2 tool, at test.log:1941 and 1945; cells module at 86 at test.log:2266; the eight mutations R1a, R1b, R3, R4, R5, S1, S2, S3 each compiling and each caught by the test named for it at test.log:2468-2681 (the "fails: yes" line for each is at 2479, 2501, 2523, 2543, 2565, 2585, 2661, 2681); the attempt-1 reason and its hash at test.log:4-6.
- negative.log claims (report:49-54): N49 to N55 at negative.log:2752-2795; Y1 and Y2 at 2935-2949; Y3 to Y7 at 2950-2991; Y8 at 2992-3006; the attempt-1 reason and hash at negative.log:4-6.
- Corpus run figures (report:66-81): all confirmed at the run.log lines cited in section 2, including the host window 17:24:17Z to 17:27:38Z (run.log:1055) and the i686 window 17:27:46Z to 17:31:13Z (run.log:1197).
- The three decisions (report:91-99): decision 1 matches diff-code-2.patch:287 and 294 (both overflow paths return `STYLE_OUT_OF_RANGE`), and the spec names no message for overflow (0012-styles.md:175-182); decision 2 matches the `_` arm at diff-code-2.patch:635 and is pinned by the C1 case of `a_clear_takes_the_style_edit` at diff-code-2.patch:1237-1243; decision 3 matches diff-code-2.patch:9-13 and test.log:2716.

Not found in a primary I hold:

1. **The whole "Attempt 1's host pass, for the record" paragraph (report:83-87)**: "27 summary rows, outcome, lo and lo_in ok in all 27, files_identical yes in all 27, 870 entries all identical and none recompressed, changed, dropped, added or flagged; totals xfs 992, num_fmts 109, custom 100, dated 106, date1904 2, 18 workbooks with dated above 0; the log's own ElementTree count equal on every total." `run-attempt-1.log` is not among my inputs. run.log prints only its sha256 and byte count (run.log:13) and the prose claim "its host pass was complete and its totals were the spec's on every key" (run.log:11). None of those specific figures is printed by any log I was given.
2. **Report:59, "its printed lines showed the tool and the log's own count equal"**: this appears only in run.log's printed script comment (run.log:72-74), which under 0001-bootstrap.md:219-221 is a figure in a script's prose and not a measurement.
3. **Report:74-75, "the two targets' outputs are byte-identical to each other"**: no log line states this. It follows from run.log:1161 and run.log:1276 (each target's outputs are the input's bytes), but the line run.log:1285 that compares the targets says something narrower, that both printed the same styles line for every workbook, and run.log:1284 that both classified every entry the same way.
4. Report:3, "The record commit is the last commit of the phase", and report:33-34 and 62 and 103 (the FILL lines) are forward-looking; there is nothing to verify yet.

### Findings

- F1. Report:83-87 states attempt 1's host figures (27 rows, 870 entries, the five totals, 18 workbooks, the ElementTree agreement) that no committed log in my inputs prints; `run-attempt-1.log` was not given to me. Either drop the paragraph, or point it at the kept attempt-1 file by its hash and say where that file is. Report:59 rests on a script comment, which 0001-bootstrap.md:219-221 says is not a measurement.
- F2. Report:74-75 claims the two targets' outputs are byte-identical to each other. That is a derivation from two printed lines, not a printed comparison. Either say so or word it as the log does.
- F3. Commit 2 carries the Limits and consequences bullets in `package.rs`'s module header (diff-code-2.patch:65-82) that 0012-styles.md:582-593 schedules for P3. Substance matches the spec's Design section; it is early, not wrong. `styles.rs` carries a Limits block from commit 1 (diff-code-1.patch:315-325) for the same reason.
- F4. `rewrite_styles` places a created `&lt;numFmts&gt;` before the first root child in document order among the five names, where 0012-styles.md:256-259 words the rule as the schema's order (diff-code-2.patch:505). Identical on every corpus part and every fixture; worth a ruling or a Limits sentence.
- F5. `Styles` gains a private `has_num_fmts` field (diff-code-2.patch:1450-1452) that the spec's struct shape does not mention, and `cells::forbidden_in_xml` becomes `pub(crate)` (diff-code-2.patch:9-13). Both are disclosed or implied; neither is named by the spec.
- F6. I could not confirm from either code diff that a clear alone on an absent cell still writes nothing under the new `_` arm of `Pending::write_row` (diff-code-2.patch:635), because `replaced_cell`'s body is unchanged and therefore not in the diff. The evidence that it does is indirect: no test was removed (test.log:1997) and both targets pass 441 (test.log:1423, 1910). The new tests cover the style-`None` absent case (diff-code-2.patch:1301-1310) but not clear-alone-absent.
- F7. ROADMAP.md's part 5 P1 row still reads "the nine messages" (diff-doc-1.patch:45); ruling 4 made it ten and doc commit 2 changed only specs/0012. A planning-file inconsistency, not the implementer's diff.
- F8. scan.log as given is the pre-check run and ends `RESULT: FAIL` on the one check about the check answers (scan.log:1200, 1257, 1262). Expected per my brief; the committed scan.log is outside what I could verify.