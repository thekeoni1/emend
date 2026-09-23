# Independent check 2, M1 part 7 P1

Checked commit: 43731f229d8a202f9de2d63e558306997a3c506c (the phase's last code commit; HEAD when the check ran).
Checker: a fresh general-purpose subagent given only the files below in <scratch>/p1/check2 (written <check> in the answer), not this session's conversation. This is the phase's second and last check.
Inputs, sha256 as hashed before the check (<scratch>/p1/check2-inputs.sha256):
  84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f  0001-bootstrap.md
  9970dddbde46173de658ecc625deb8320515c9e788b80376cea4b273d296268e  0014-xml-comparison.md
  974281a3e5b72c21f7bb7a236153e8166e6404cf119d2478d93ff84bba55a9f1  diff-code-43731f2.patch
  0224762463c9c2ced19ed614d143af3c42cbddf6f62ff0763bdababc4bfe5523  diff-code-b315caf.patch
  4755c6dd7ecdb70ecc7eb34442a97fc093de0793fbad53605b604c098ebc3b3d  diff-commands.txt
  5d99f413380af0d62a4620ddc2af519c5fd931b4cb6d12290477b89236c536e4  diff-phase.patch
  9c9c5551bcd96354fd1437ba016ae46ff41ad2ecd70e0061cba84ee5b901520a  gate.log
  ebced1cdff475968e27a39b0c60aea32fce759c1a14042911fda92e1048f1d2c  independent-check-1.md
  552f8f02198df293b03014f02cfdb0c150cbdaad7d634a10c9562e1837618aeb  negative.log
  14699bc6de05352dc49a2141fbe4883910ea613811947dc8200b0d7c43922601  report-draft-2.md
  16d8f910f43ae1fa625cba7763326f54198abb98c8d5b763b5612f692aed845c  scan-for-checkers-2.log
  0d5ed8fd9c7a2660a6768004a8a83a4d20cd6c81331b26bd2c019f3e960f04b0  test.log
The logs it read are the logs committed with this answer, unchanged since (compare the sha256 above with scan.log).
The answer below is the checker's final message, extracted from its transcript, byte count 15852, sha256 4e873dbe5553114f25a3fe1564769c0a91bc85b02b8075c029a7b901aa10d856; it is all ASCII and holds no machine path, so it is unaltered.

---

The harness put a project instructions file (CLAUDE.md), auto-memory notes and git-status context into my context. I did not use any of it as evidence; everything below comes from the files in <check>.

Before starting I hashed every input. The hashes the logs cite match the files:
- <check>/diff-phase.patch is 5d99f413..., the hash gate.log line 1257 prints for the combined diff.
- <check>/diff-code-b315caf.patch is 02247624... and <check>/diff-code-43731f2.patch is 974281a3..., matching gate.log lines 1251-1254.
- <check>/0014-xml-comparison.md is 9970dddb..., matching gate.log line 1264.
- gate.log, test.log, negative.log and independent-check-1.md hash to the values scan-for-checkers-2.log prints (lines 1339, 1349, 1359, 1375).
- The spec text inside diff-phase.patch (lines 118-500) is byte-equal to <check>/0014-xml-comparison.md.

## 1. Does the diff do what the spec says, and nothing else?

Yes. I found no real defect in the code. One sentence of the spec is not implemented (G1), and a few paths are untested (G3).

**"Nothing else", confirmed in each diff:**
- **<check>/diff-phase.patch**: 10 files.
  - ROADMAP.md, specs/0013 and specs/0014 are the blobs planning's messages gave (gate.log lines 1197-1244).
  - The rest is tools/xlsx-diff plus one lock-file line (diff-phase lines 1-11; gate.log lines 1312-1330). No package joined the lock file.
  - xlsx-diff's tree has exactly zip and quick-xml as direct dependencies, and no emend (gate.log lines 1352-1382).
  - All 12 licences are permissive (gate.log lines 1384-1405).
- **<check>/diff-code-b315caf.patch**:
  - Cargo.toml adds only `quick-xml =0.42.0` with default features off (gate.log lines 1331-1343).
  - lib.rs adds `mod xml`, XmlCounts, read_xml, write_xml, compare_xml, the `compared` list, one loop, and four archive rows after files_identical (diff-phase lines 516-682). The entry rows and the exit status are untouched.
  - The only test changes are the helpers, p3.rs (four archive rows) and the new p7.rs (test.log lines 2815-2911).
- **<check>/diff-code-43731f2.patch**: only xml.rs (the Limits text, Text::markup and its three call sites) and p7.rs (new fixtures and tests), lines 1-170. gate.log lines 1304-1311 confirm the path list.

**The code against the spec (line numbers in diff-phase.patch):**
- **Which entries**: `is_xml_name` lower-cases the raw name and checks for .xml, .rels or .vml (lines 764-769). It runs only in the `changed` branch (lines 656-660).
- **What is compared**:
  - Names are written `{ns}local`, and `{ns}local@{ans}attr` for attributes. xmlns declarations are skipped (line 957). An Unknown prefix gives `false`, which is a parse error (lines 944-946, 961-963).
  - Attribute values go through `normalized_value`, or are hashed raw and counted (lines 972-978).
  - Text is assembled across Text, CDATA and GeneralRef events. Comments and PIs are transparent. Whitespace-only text is skipped (lines 916-936).
  - Decl and DocType events are ignored (line 1055).
- **Output**:
  - The parse row comes first (lines 592-603). A parse error ends the entry there (lines 604-607).
  - element rows come before attribute rows, taken from BTreeMaps via `differing` in byte order and escaped with escape_name (lines 608-619). The values row comes last.
  - All counts are u64.
- **Rulings**:
  - Ruling 1: row layout matches (line 563).
  - Ruling 2: `a.error.or(b.error)` (lines 587-591).
  - Ruling 3: the early return skips names_equal and values_equal.
  - Ruling 4: `error_position()` for reader errors (line 1016); `buffer_position()` for an unknown prefix, an attribute error or end of input with an element open (lines 1024, 1031, 1059).
  - Ruling 5: the open-element stack is checked at Eof (lines 1058-1060).
  - Ruling 6: see F1 below.
  - Ruling 7: a tag byte, then write_u64 lengths (lines 794-802), and a pinned unit test (lines 1123-1129).
  - Ruling 8: helper widths and 25 keys (lines 1149-1178).
- **Privacy**: no value, hash or quick-xml message reaches a row.
  - The no-leak test plants a sentinel in text, an attribute and CDATA on each side, with a control (lines 1737-1803).
  - M10 shows the test bites (test.log lines 3359-3406).
- **32-bit**: every count and offset is u64. Both value vectors live only inside compare_xml and are dropped on return (lines 574-640).

**Check 1's F1 is fixed, and the fix is right.**
- `Text::markup()` clears `after_cr` (43731f2 patch lines 35-37). It is called for Comment and PI (line 61) and on both sides of a CDATA section (lines 49-53).
- The other ways a CR can stop being half of a pair were already covered: a reference clears the state (push_reference, push_unresolved), and every tag flushes the node.
- So a CR and an LF count as one line end only when they are adjacent characters, which is what XML 1.0 section 2.11 says.
- Evidence:
  - Fixtures: phase diff lines 1407-1416 and 1455-1469.
  - Through the binary: N10 (negative.log lines 1544-1616).
  - M11, which undoes the fix, bites two tests (test.log lines 3407-3483 and 3566).

### Findings

**G1 (defect, minor: undeclared departure from the spec text).** Spec "32-bit and streaming" (0014 lines 179-181) says an entry whose value vectors cannot fit is "an allocation failure the process reports as an error on stderr with exit 2".
- Nothing in either diff does that. The vectors grow with plain `push` (diff-phase lines 929, 979).
- The module's Limits say only "an allocation failure of the process" (diff-phase lines 739-740), and the report does not name the difference.
- With std's default handler, an allocation failure aborts the process rather than exiting 2. That behaviour comes from my knowledge of std, not from a primary.
- Planning should either rule the Limits wording sufficient or ask for exit 2.

**G2 (note).** For F1, the "differs" direction is tested only with a comment (diff-phase lines 1455-1469). The processing-instruction case has no fixture of its own. It goes through the same match arm as a comment (43731f2 patch line 61), so the risk is low.

**G3 (evidence gap).** Some of what the corpus relies on is untested:
- No fixture has a CR LF (or a lone CR) inside an attribute value. Whether `normalized_value` applies line-end normalisation before its section 3.3.3 whitespace mapping is taken on trust from quick-xml.
- The byte order mark fixture is UTF-8 only (diff-phase line 1399). quick-xml is built with default features off. The spec's Facts count 158 entries with a byte order mark and 44 with no declaration (0014 lines 53-55) but do not say they are all UTF-8.
- P2 would show either problem as false value deltas or parse errors.

**G4 (note, carried from check 1's F6).** The case map maps "one element added, one removed, one attribute added, one attribute's value changed (names equal, values 1 and 1)" to the fixture "all four at once" (test.log case-map lines after 2575). That fixture's expected rows are not "names equal, values 1 and 1". The fixture that matches the parenthetical is "one attribute's value changed" (diff-phase lines 1326-1330). Both fixtures exist and pass.

**G5 (note, ruling 7).** Ruling 7 says the pinned unit test is "the only place a hash is ever written, and it is a test's, not a record's". But test.log lines 3350-3351 print two hash values, one of them the pinned constant: they are M9's failing assertion on i686. They hash a fictional fixture, so there is no privacy content, but test.log will be committed as a record. Planning may accept this or ask the mutation block to filter the lines.

## 2. Do the logs contain every line the spec and the rules require?

Yes, with the gaps below.

**Present:**
- **Header lines**: in all four logs, line 1 is the command, line 2 the invocation and line 3 the about line, naming the phase commit and HEAD.
- **Phase-commit guard, each with controls**: gate.log lines 34-39; test.log lines 47-51; negative.log lines 51-54; scan-for-checkers-2.log lines 19-23 and 57.
- **SERIALIZED lines with process counts**: gate.log line 44; test.log line 40; negative.log line 42; scan-for-checkers-2.log line 42.
- **Superseded attempts named by sha256 and hashed by a printed command**: gate.log lines 41-47; test.log lines 35-39; negative.log lines 36-41.
- **Shape greps with controls**: gate.log lines 5-15; test.log lines 5-15; negative.log lines 5-21; the scan over every printed script at scan-for-checkers-2.log lines 1419-1442.
- **isnot control**: negative.log line 55; gate.log in the structure block after line 1150.
- **Gate-file listing and no self-test**: gate.log lines 1273-1290.
- **Tests on both targets**: test.log lines 1845 and 2401.
- **The p7 tests and the pinned test on both targets**: test.log lines 2564-2569.
- **Bite claims measured**: test.log lines 2915-3499.
- **Ruling 8 measured, with controls**: test.log lines 2720-2911.
- **Negative log on both binaries**: malformed entry, unknown prefix and non-zip, with rows and exit status printed (negative.log lines 931-1641).
- **Commit messages**: both say their counts are the log's, not the message's (test.log lines 3500-3566).
- **Diff commands stated**: <check>/diff-commands.txt.
- **Control bytes and non-ASCII**: none in any log (my own count: 0 in each).
- **Absolute machine paths**: none. The only path-shaped strings are concatenated control fixtures (gate.log line 813; scan-for-checkers-2.log lines 443, 495-496, 527).

**G6 (evidence gap, F9).** negative.log line 50 carries a departure line: the cases ran in a mkdir scratch directory, not a git worktree or clone.
- 0001 lines 92-93 say "A departure from this needs planning's word before the run".
- The line cites specs/0013 P2 verification ruling 5, which I cannot read. As quoted there, that ruling concerns a stated root, not this departure.
- No primary shows planning's word before the run. F9 is declared but not settled.

**G7 (evidence gap, F10; the report's answer is inaccurate).** The logs use `<scratch>/p1` without a root line of its own, and the report describes the roots wrongly.
- gate.log declares a root line for `<scratch>/w` (line 45) and says "the two roots" (line 46).
- test.log declares root lines for `<scratch>/w` and `<scratch>/mut` (lines 41-42) and says "the three roots" (line 43).
- Both logs use `<scratch>/p1` for the script and the target directory (line 2 and the SERIALIZED line) without a root line for it.
- negative.log also uses `<scratch>/p1` (line 2, and `<scratch>/p1/out/gate.log` in the binaries block) but declares only `<scratch>/p1/target` (line 45).
- 0014 line 234 asks for "one header line per root".
- scan-for-checkers-2.log line 45 reads "<scratch> stands for the directory above this script's own, which holds the script". The script is at `<scratch>/p1/...` (line 2), so the wording contradicts itself.

**G8 (note, F8).** The probe covers p1 to p6 and passthrough's copies tests (test.log lines 2761-2811).
- The faults tests (16) and the null_rewrite tests (37) are cleared only by `git grep 'xlsx_diff::run'` (test.log lines 2780-2790).
- A test that spawns the xlsx-diff binary would not be found by that grep. None is shown to exist.

**Check 1's findings F1 to F14:**
- **F1**: fixed in 43731f2, and the fix is right (see question 1; M11 and N10).
- **F2**: fixed. An added entry now stands before the first xml row (diff-phase lines 1696 and 1715-1717; test.log lines 2638-2639).
- **F3**: fixed with a byte order mark fixture and xml:space fixtures (diff-phase lines 1397-1406 and 1471-1491; test.log lines 2632-2637). The remaining encoding gap is G3.
- **F4**: answered in the Limits (43731f2 patch lines 21-23) and in report finding 3. Still open for a planning ruling.
- **F5**: answered in report finding 6, but the value appears in no primary (see G11). Open as an evidence gap.
- **F6**: not addressed in the report. It stays a note (G4).
- **F7**: fixed. `bad` now counts failures (scan-for-checkers-2.log lines 284-286), with a two-failure control (lines 549-555); the output line is 1446. Report finding 4 covers it.
- **F8**: fixed (test.log lines 2780-2811), with the residual noted in G8.
- **F9**: declared with a departure line; planning's prior word is not shown (G6). Open.
- **F10**: answered in report finding 5, but the answer is inaccurate (G7). Open.
- **F11**: fixed. The report withdraws the claim (report line 41). The negative script's own comment still asserts "The tool's rows were right" (negative.log line 147), but only as prose.
- **F12**: fixed. The hashes are printed directly under the rerun line (negative.log lines 36-41).
- **F13**: fixed. The derivation comment is at diff-phase lines 1534-1537, and my byte counts agree: 3 + 12 = 15 and 3 + 11 + 4 = 18.
- **F14**: still a paraphrase (see G14).

## 3. Does every number and quoted string in the report appear in a primary?

No.

**Checked and found in a primary:**
- **Commits and their roles**: 3c4e7de, b45c524, b315caf and 43731f2 (gate.log lines 1150-1157). The doc blobs are at gate.log lines 1197-1244.
- **`=0.42.0`**: gate.log line 1339. No emend in the tree: gate.log line 1379.
- **"gate PASS"**: gate.log lines 1539-1550.
- **508 passed, 0 failed, 2 ignored, on both targets**: test.log lines 1845 and 2401.
- **15 p7 tests on each target, and the pinned test on both**: test.log lines 2564 and 2568.
- **Attempt 1 at 43731f2 failed one check on two script faults, named by sha256**: test.log lines 35 and 37.
- **The superseded logs**: 22c8e204 (gate.log lines 41 and 47), f06e3a1c (test.log lines 35 and 38), c70ce532 (negative.log lines 36-37).
- **Ruling 8**: probe 0 with control 12 (test.log lines 2761 and 2764); copies 0 with control 13 (lines 2797-2798); p3.rs at the base as the only failure (line 2911).
- **M9**: does not bite on the host (test.log line 3329) and bites on i686 (line 3356).
- **N0 to N10 and their cases**: negative.log lines 76-100 and 931-1663. The departure line is at line 50.
- **Offsets 15 and 18**: diff-phase lines 1534-1537, 1555 and 1579.
- **The 25 archive keys**: test.log line 3523; diff-phase line 1166.

**G9 (evidence gap).** Report line 16 says "173 checks as expected". No log prints 173.
- My own count of negative.log lines ending ": as-expected" is 173, so the figure is correct.
- But it is the report's own count. Under 0001 lines 219-221, a figure called measured must be printed by the log.

**G10 (defect in the report).** Report line 39 (finding 5) says "gate.log and test.log use one root, <scratch>/p1, and need no root line". That is wrong: gate.log and test.log declare other roots (`<scratch>/w`, and in test.log also `<scratch>/mut`) and speak of two and three roots (see G7). The cited "specs/0013 P2 verification ruling 5" is not in <check>, so I could not check it.

**G11 (evidence gap).** Report line 40 says "CHUNK is 64 * 1024 at tools/xlsx-diff/src/lib.rs". Neither the constant nor its value is in any primary.
- The only related text is the authored doc comment "through a 64 KiB buffer" (diff-phase line 543).
- The report does say no log prints it, but the value itself cannot be verified from the inputs.

**G12 (note, wording).** Report line 15 says "Mutations M1 to M11 with each planted change, run and failing assertion printed". M9 on the host ran in measure mode and failed nothing, so no failing assertion is printed for it (test.log lines 3296-3329).

**G13 (note).** Report line 16 lists "a sentinel case". negative.log has two sentinel cases, N7a and N7b, plus the N7 scan block (lines 1645-1650). This is not wrong, just compressed.

**G14 (note, F14).** Report line 35 attributes "a usize length equals write_u64 on 64-bit" to test.log as "test.log's own explanation". No line of test.log holds those words. The nearest text is a script comment, "writes the same eight bytes as write_u64" (test.log lines 912-915). That is prose, not a measurement; the measurement is lines 3329 and 3356.