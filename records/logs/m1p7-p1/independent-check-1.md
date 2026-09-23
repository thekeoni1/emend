# Independent check 1, M1 part 7 P1

Checked commit: b315caf18e1f8ccd25c14088c9f1414fcdf66b15 (the phase's first code commit; HEAD when the check ran, read by git rev-parse HEAD at the time).
Checker: a fresh general-purpose subagent given only the files below in <scratch>/p1/check (written <check> in the answer), not this session's conversation.
Inputs, sha256 as hashed before the check (<scratch>/p1/check1-inputs.sha256):
  84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f  0001-bootstrap.md
  9970dddbde46173de658ecc625deb8320515c9e788b80376cea4b273d296268e  0014-xml-comparison.md
  0224762463c9c2ced19ed614d143af3c42cbddf6f62ff0763bdababc4bfe5523  diff-code-b315caf.patch
  22d69b4765045c1dce9e06d1ab233d768d0b9022828388fd7ce91a144b95f3e3  diff-commands.txt
  bef21fc78359839654128467e1ec514106f5b5965c6a6070bb16cc3b23163c69  diff-phase.patch
  22c8e204c58b08a43867009466f38ceea96e8c70481b1be9d4eaeef8ee0174c5  gate.log
  c70ce532d7226464eebc2919b9a4ab1c9b02fddf09f977dd0c54e4e13271865b  negative.log
  c0869846e54788f919210fcf788b3f7ea0a938f610eae5b7a80dd6a459c9518d  report-draft.md
  61fd3227aae30e533b01fd4436c76796d9d58f0c3e463757585bac82e7d759f1  scan-for-checkers.log
  f06e3a1c4edcb45a3d3bae957d9a21127a3ede77a8877f224a87d24c971c2624  test.log
The logs it read are superseded: the phase gained a second code commit, 43731f2, for this answer's F1, and every log was rerun at it. The b315caf logs were committed nowhere.
The answer below is the checker's final message, extracted from its transcript, byte count 9904, sha256 e7dee1d0bb265e6e21e1efe1479ed35ab0ea0a387c9bef91544f8a021293d522; it is all ASCII and holds no machine path, so it is unaltered.

---

The harness put CLAUDE.md, auto-memory notes and git-status context into my context. I did not use any of it as evidence. Everything below comes only from the files in <check>. Line numbers are given in the named file's own numbering.

Before answering I hashed every input. The hashes match the primaries:
- <check>/0014-xml-comparison.md is 9970dddb..., the blob gate.log names for both doc commits (gate.log lines 1178-1181).
- <check>/diff-code-b315caf.patch is 02247624... and <check>/diff-phase.patch is bef21fc7..., the hashes gate.log prints (gate.log lines 1216-1219).
- gate.log, test.log and negative.log hash to the values scan-for-checkers.log lists (scan-for-checkers.log lines 1301, 1311, 1321).

**1. Does the diff do what the spec says and nothing else?**

Yes, with one small defect (F1) and some untested paths (F2, F3).

"Nothing else", in the phase diff (<check>/diff-phase.patch): 10 files.
- ROADMAP.md, specs/0013 and specs/0014 are planning's files at the hashes planning's messages gave (gate.log lines 1163-1210).
- The rest is tools/xlsx-diff plus one lock-file line (gate.log lines 1256-1300).
- There is no change to the core, emend-copy or any probe, and emend is not in xlsx-diff's dependency tree (gate.log lines 1307-1337).

"Nothing else", in the code diff (<check>/diff-code-b315caf.patch):
- Cargo.toml adds only `quick-xml =0.42.0` with default features off (lines 21-23).
- lib.rs adds `mod xml`, `XmlCounts`, `read_xml`, `write_xml`, `compare_xml`, the list of compared entries, and the four archive rows after `files_identical` (lines 28-194). The entry rows and the exit status are unchanged.
- The test helpers, p3.rs and the new p7.rs are the only test changes (lines 622-1229).

The code against the spec:
- **Which entries** (line 169, lines 272-277): only `changed` entries, with a case-insensitive `.xml`, `.rels` or `.vml` suffix.
- **Output** (lines 98-150):
  - The `parse` row comes first.
  - `element` and `attribute` rows come from BTreeMaps in byte order, via `escape_name` (ruling 1), and only where counts differ.
  - A parse error on either side ends the entry at its `parse` row.
  - The offset is the input's, then the output's (ruling 2).
  - names_equal and values_equal count only entries that parsed on both sides (ruling 3).
- **Ruling 4** (lines 512-533, 547-552): `error_position()` for a reader error; `buffer_position()` for an unknown prefix, an attribute error, or end of input with an element open.
- **Ruling 5**: end of input with an element open is an error.
- **Ruling 6**:
  - Comments and processing instructions do not end a text node (line 546).
  - CR LF and a lone CR become LF (lines 351-370).
  - A value that does not unescape is hashed raw and counted.
- **Ruling 7**: the hash is a tag byte plus `write_u64` lengths (lines 302-310), and a unit test pins it.
- **Privacy**: rows carry names and counts only. The no-leak test plants a value in text, an attribute and a CDATA section on each side (lines 1188-1228).
- **32-bit**: every count is u64, and both value vectors are dropped when `compare_xml` returns.
- **Tests**: every case listed in "### P1" and the fixtures added by rulings 5 and 6 are in p7.rs (lines 765-1229).

**F1 (defect, minor).** The line-end state carries across markup, which departs from XML 1.0 section 2.11 as the module's own Limits cite it (code diff lines 230-231).
- `after_cr` is set in `push_literal` (code diff lines 351-369) and is not reset when a text node continues past a comment, a processing instruction or a CDATA boundary (lines 540-546).
- So `x\r<!--c-->\ny` and `<![CDATA[x\r]]>\ny` give one LF. XML 1.0 gives two, because the CR and the LF are not next to each other in the document.
- The effect is that two texts which differ can compare equal. No test covers this.

**F2 (evidence gap).** Neither diff shows that `added` entry rows come before the `xml` rows.
- The `xml` loop is inserted after a closing brace whose loop is not shown (code diff lines 175-183).
- The only ordering test (code diff lines 1135-1144) has no `added` entry, and the dropped/added test (lines 1068-1086) has no compared entry.
- The spec asks for xml rows "after every `entry` row" (0014 line 126).

**F3 (evidence gap).** No fixture has a byte order mark or an `xml:`-prefixed attribute (for example `xml:space`).
- The spec's Facts count 158 entries that start with a byte order mark (0014 lines 53-54).
- The code relies on quick-xml to skip the byte order mark and to pre-bind the `xml` prefix (code diff lines 496-556, 331-336). If either fails, P2 would show false value deltas or parse errors.

**F4 (note).** On a side where the reader returns an error, an unfinished text node is never flushed (code diff lines 512-518), so a raw reference in it is not counted. Ruling 2 ("count up to the failure") does not settle this case.

**F5 (note).** The 64 KiB buffer is `CHUNK` (code diff line 63). Its value is not in either diff, so I cannot confirm 64 KiB from the inputs.

**F6 (note).** The "### P1" phrase "one element added, one removed, one attribute added, one attribute's value changed (names equal, values 1 and 1)" is ambiguous. The tests cover each change alone and all four together (code diff lines 785-843), so either reading is covered.

**2. Do the logs contain every line the spec and the rules require?**

Yes, with the gaps below.

Present and correct:
- **Headers**: all four logs have command, invocation, then the about line third, naming the phase commit and HEAD.
- **Phase-commit guard with controls**: gate.log lines 31-36, test.log lines 43-46, negative.log lines 47-50, scan-for-checkers.log lines 19-23.
- **SERIALIZED lines with process counts**: gate.log line 39, test.log line 35, negative.log line 39, scan-for-checkers.log line 46.
- **Superseded attempts named by sha256**: test.log lines 32-34; negative.log lines 35-38, covering the old log and the old script.
- **Shape greps with controls**: gate.log lines 5-15, test.log lines 5-15, negative.log lines 5-22; the scan runs them over every printed script (lines 1371-1395).
- **isnot control**: gate.log line 1144, negative.log line 51.
- **Gate-file listing and no self-test**: gate.log lines 1237-1254.
- **Tests on both targets**: test.log lines 1710 and 2264.
- **Bite claims measured**: test.log lines 2690-3203.
- **Ruling 8 probe with control**: test.log lines 2484-2688.
- **Negative log**: a malformed entry, unknown prefixes and a non-zip, through both built binaries, with rows and exit status printed (negative.log lines 893-1523).
- **Diff commands stated**: <check>/diff-commands.txt.
- **Commit message on unprinted counts**: the code commit says so (test.log lines 3205-3236).
- **No machine paths**: I found no absolute machine path in any log; the only hits are scan-control strings built by concatenation.

**F7 (defect in a log, minor).** "structural checks failed" is a 0/1 flag, not a count.
- `bad=1` sets it (scan-for-checkers.log line 273, printed at line 864).
- The pre-check shows two structural "no" lines (lines 1339, 1390) but prints "structural checks failed: 1" (line 1397).
- The final scan uses the same script, so its tally would understate any real failures.

**F8 (evidence gap).** Ruling 8's probe covers only xlsx-diff's p1 to p6 (test.log line 2566).
- The workspace also has the test binaries copies, faults and null_rewrite (test.log lines 2272-2274). If any of them runs xlsx-diff on a changed XML entry, it was not probed.
- The report limits its claim to p1 to p6, so the report is not wrong.

**F9 (note).** The negative cases ran in a fresh scratch directory with the gate's binaries copied in (negative.log lines 41, 771-782), not in a `git worktree` or clone as 0001 line 89 says. No departure line is printed. The main tree was untouched (negative.log lines 1544-1549).

**F10 (note).** gate.log and test.log use `<scratch>/p1` (the script and the build directory) without a `root:` header line of their own; negative.log has one only for `<scratch>/p1/target` (line 42). The 0013 ruling text ("one header line per root") is not in <check>, so I cannot judge this.

**3. Does every number and quoted string in the report appear in a primary?**

No, one claim does not (F11). Everything else checks out:
- The commits 3c4e7de, b45c524 and b315caf: gate.log lines 1121-1127.
- `=0.42.0`: gate.log line 1291.
- 25 archive keys: test.log line 3226.
- 506 / 0 / 2 on both targets: test.log lines 1710, 2264.
- 13 p7 tests on each target: test.log line 2415.
- The attempt-1 sha256 715ed6b7... and the quoted 'error: test failed': test.log lines 32, 34.
- N0 to N9 and their cases: negative.log lines 797-1538.
- Probe 0 and control 10: test.log lines 2575-2590.
- The single failing test with p3.rs at the base: test.log line 2688.
- M1 to M10, with M9 biting on i686 only: test.log lines 3189-3203.
- Offsets 3, 4, 7, 9, 15, 18 and 124: code diff lines 972-1008, 1132; negative.log line 1386.

**F11 (evidence gap).** "the tool's rows were the same" (report line 15) is in no primary. negative.log line 35 says only that the script's reader was fixed and "nothing else changed". No log compares attempt 1's rows with attempt 2's.

**F12 (note, wording).** Report line 15 says attempt 1 is "named by sha256 in the rerun line". The hashes are on negative.log lines 37-38, under the sha256sum command, not on the rerun line (line 35).

**F13 (note).** Report line 34 says the offsets are "derived from the fixtures by hand in the tests". p7.rs gives a derivation for 9, 7, 3 and 4 (code diff lines 963-966, 1131) but none for 15 or 18. Counting bytes, both values are correct.

**F14 (note).** Report line 31's "write_usize writes the same eight bytes as write_u64" is the report's own paraphrase. test.log line 32 says "a usize length that equals write_u64 on 64-bit". What is measured is that the change does not bite on the host (test.log line 3095).