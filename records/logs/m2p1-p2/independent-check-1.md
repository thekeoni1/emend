# Independent check 1, M2 part 1 P2

Checked commit: e7c7061ba07c0616377bb1abbecf6c7b062da123
HEAD at the check: e7c7061ba07c0616377bb1abbecf6c7b062da123
Checker: a fresh general-purpose subagent (not a fork), given only the files below in one scratch directory.
Inputs, each with its sha256 (as given to the checker):
- 0016-new-workbook-and-sheets.md: 3d8b8aa8872d5a25e664ceca73fa9dcd614b38cbd03f135e5678947ac0f69e6f
- 0001-bootstrap.md: 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
- commit-fc82ca7.diff: 9c048fc95809bf4795faf61de1bd81b692534102281198ee5fdacc3bb8b2d408
- commit-5f6505a.diff: 3bd6d95a7099582b5e5a2a7f3567e463423925d9f8f30a5bcd554e9a10adb998
- commit-e7c7061.diff: b70f44e7b94da260adc390b661059f19439df002bae7108f01fa1b26aa0d490f
- combined.diff: 689d63d5813431a5c64ccbee6a074922ec23c5419eaf2a57ae27350620979696
- draft-report.md: 210d2a85ad796a624c58adbb34dd52892e5728e0c17a32143c3aa5648df6dd20
- gate.log: 0891a0f395fb9d7ed36175b47cfaf3b231ebf427f7cada22112845f63770fb2a
- test.log: e37491d431e27a3dd7de7d0e4466ed4e044387ef1a4bc1f0f0f8bc59c29d7feb
- negative.log: f459b1e604db79ef4841f281983d8a0a969cf69102d47eb1f1adace8f4d15a7b
- run.log: ad90c73a9eb38c45206ff818778187bfde008595804038174ed1ea4ccf9676e2
- scan-pre-check.log: bacd1b4e4ef1dbab72c186c246c73aaa3568184962b960be84d303c8d5826957
- m2p1-open-2026-09-25-e7c7061.md: 668c716f7b01843399151a323488e02ac1bb20c94793b9b4378f257dea426827
- m2p1-open-2026-09-25-e7c7061.txt: c62398c89614ddaaa950d30ab3f46dc3c57ce44a258d9e829c6bb504acb6428c
The negative.log given was attempt 1, sha256 f459b1e604db79ef4841f281983d8a0a969cf69102d47eb1f1adace8f4d15a7b; the committed negative.log is attempt 2, rerun for this check's F1 and F9.
The answer is ASCII and holds no absolute machine path; it is reproduced verbatim below (sha256 of the answer text 0d25a1828e54c5c3b633712b6e28b2d13343739127ab1afaeea3dc3a47743bd0, 12957 bytes).

---

Note: the harness put a CLAUDE.md into my context. I have not used it as evidence.

Inputs I read: everything in `<scratch>/p2/check1/` and nothing else. The hashes I computed match the draft report's prefixes: gate.log 0891a0f3, test.log e37491d4, negative.log f459b1e6, run.log ad90c73a, the record .md 668c716f and the .txt c62398c8. I checked mechanically that combined.diff is exactly the three per-commit diffs added together: the same 14 file blocks, byte-identical.

## Q1. Does the diff do what the spec says for P2, and nothing else?

Yes, apart from the findings below.

**"Nothing else", by diff**
- **commit-fc82ca7.diff (doc):** only ROADMAP.md (lines 13-27) and specs/0016 (lines 28-190). The spec change is the P1 rulings 1-11 and the P1 verification 1-8. The P2 rulings are not committed, as the spec says they should not be yet (spec 836-838). gate.log 1021-1045 matches the two blobs to planning's hashes. Nothing else.
- **commit-5f6505a.diff (code 1):** two files only. styles.rs gets a six-line doc sentence (lines 26-31). excel-check-open.ps1 gets a comment (lines 44-47) and the try around Close (lines 56-60). gate.log 1137-1142 confirms these are the only two paths. Nothing else.
- **commit-e7c7061.diff (code 2):** these files only:
  - package.rs: catalogue note 37-42, limits 52-67, the `Cursor` import 75, `EMPTY_ARCHIVE` 83-88, `TEMPLATE` 90-127, `new` 129-156
  - package/tests.rs: 170-425
  - the six template files: 426-478
  - emend-copy main.rs: 480-677
  - null_rewrite.rs: 679-744

  gate.log 1143-1163 confirms: 10 paths, none outside the area, and no Cargo.lock or manifest change. Nothing else. The one addition the spec does not name is the `write_into` helper in main.rs (672-686), which is the file handling ruling 4 asks for.
- **combined.diff:** the same as the union above.

**Template bytes against rulings 1 to 3**
- **Ruling 1:** each file is the declaration, one LF, then the root. There is no whitespace between elements and no newline at the end. The diff marks "No newline at end of file" at 434, 443, 452, 461, 470 and 479. The test asserts exactly one LF and no CR (tests 258-266).
- **Ruling 2 (styles.xml, line 460):**
  - `fonts` count 1: sz 11, Calibri.
  - `fills` count 2: none and gray125.
  - `borders` count 1: left, right, top, bottom, diagonal.
  - `cellStyleXfs` count 1.
  - `cellXfs` count 1, with font 0, fill 0, border 0 and xfId 0.
  - `cellStyles` count 1: Normal, builtinId 0.
  - No numFmts, dxfs or tableStyles.
- **Ruling 3:**
  - Content types (433): Default for rels and for xml, and three transitional Overrides.
  - Rels (442, 451): attributes in the order Id, Type, Target. The workbook rels have rId1 for the worksheet and rId2 for styles.
- **The spec's other parts:**
  - workbook.xml (469) has the default namespace, `r` bound, workbookPr, one workbookView with activeTab 0, one sheet Sheet1/1/rId1, and no calcPr.
  - sheet1.xml (478) is an empty sheetData only.
  - The six files total 2600 bytes (run.log 977), under 4 KB.

**Package::new against the spec and ruling 6**
- The signature is `impl Package<Cursor<Vec<u8>>> { pub fn new() -> Result<Self, Error> }` (e7c7061 129, 147).
- The constant and the table are private and non-pub (86, 105), and the table is `include_bytes!` in the spec's order (105-127).
- It opens the empty archive, adds the six parts, writes them to a Vec and reopens the result (148-154), as the spec says.

**Catalogue and limits against ruling 5**
- The catalogue says `new` adds no message (37-42).
- The template is documented in package's doc comments: the six files, the constant, "written by hand", "fictional and hold no cell" (83-104).
- There is no README in the template directory (only six files are added: gate.log 1146-1157).
- New limits are at 52-67.

**--new-sample against the spec and ruling 4**
- It writes `new()` to `<a>`, reopens it, sets (1,1) Number 7, (1,2) Text "emend" and (1,3) Boolean true, and writes `<b>` (579-604, 606-613).
- It asserts cells and read_back equal 3 and exits with the error on a mismatch (642-655).
- Each file goes through a temporary in its own directory and is renamed into place (658-674).
- On a failed check `<b>` is removed and `<a>` stays (594-600).
- The usage string and parser are updated (509, 547-549). The refusal vectors are added to the tool test (687-690).

**The tests "P2" lists (spec 525-543)**
All present:
1. `new()` twice: `new_twice_gives_the_same_bytes` (211)
2. The six entries by name and order, byte for byte: `new_holds_the_six_template_files_in_order_byte_for_byte` (219)
3. The opened workbook: `new_opens_back_as_one_empty_sheet_with_one_cell_format` (295). It covers Sheet1, Worksheet, one cellXf, no shared strings, no chain, date1904 false and full_calc false.
4. The four values: `new_takes_a_number_a_text_a_boolean_and_a_formula` (333). It checks calcPr goes right after sheets (375-378).
5. The cell style: `new_takes_a_cell_style_over_its_one_format` (381)
6. The write with no edits: `new_written_with_no_edits_is_its_own_bytes_and_is_kept` (412), together with `new_is_the_empty_archive_with_the_template_added` (269)
7. The tool's line: `the_new_sample_writes_a_new_workbook_then_three_cells` (null_rewrite, 699)

test.log 1905-1913 and 1954-1970 show all 8 added and passing on both targets.

**The Close fix against P1 verification ruling 3 (spec 774-780)**
Close sits in a try. The catch writes "close-error: ..." into `$message` and does not touch `$result`. The row is appended and the loop goes on (5f6505a 56-63). See F4.

## Q2. Do the logs contain every line the spec requires?

Yes. Every required line is present:
- **Template sha256s printed:** run.log 968-979, which also prints byte counts and the total. Also gate.log 1068-1073.
- **--new-sample:**
  - Once on each target: run.log 988-1004.
  - Pair diffed: run.log 1030-1070.
  - Targets compared byte for byte: run.log 1006-1008.
  - Independent zipfile check of `<a>`: run.log 1010-1028.
  - Opened through the pinned image: all four files, both `<b>` included (run.log 966-967, 1072-1080). There is a truncated-file control.
- **Edit open check over the keep:**
  - The keep is regenerated on both targets and compared (run.log 1081-1195).
  - It is checked against P1's record (1196-1220).
  - edit-open-check.sh is run (1222-1381): pairs 155, findings 0.
  - Totals and verdicts are checked (1488-1535).
  - The record pair is installed (1537-1556).
- **Stage rebuild:**
  - Same list, hash unchanged: run.log 1559-1561.
  - 332 files staged, 330 hash-checked: 1562-1569.
  - Zip sha256, bytes and mode: 1570-1576.
- **Negative log:**
  - Worktree and build: negative.log 591-596.
  - N0 to N5: 598-649, with every printed command naming `<build>/target/debug/emend-copy`, the worktree build it runs (ruling 4).
  - The statement that nothing new is reachable: 650-654.
  - Worktree removed: 656.
  - Family P uses the git-show form with its hashes (P1 verification ruling 1): 574-580 and 657-679.
- **Tree-status checks printing the lines:**
  - gate.log 1385-1393
  - test.log 50-51 and 2006-2010
  - negative.log 42-43 and 684-688
  - run.log 45-46 and 1583-1590
  - scan-pre-check.log 1428-1432
- **Forbidden-shape rules:**
  - The self-greps at the top of each log: gate.log, test.log, negative.log and run.log 5-15; scan-pre-check.log 5-16.
  - The scan over every printed script: scan-pre-check.log 1473-1500, zero instances.
  - My own grep of the three patterns over all logs, both records and the draft report found 0.
  - All inputs are ASCII.

Every log ends with its "all as expected: yes" line. The scan ends with RESULT: PASS and 0 hits (1501-1505).

## Q3. Does every number, hash and quoted string in draft-report.md appear in a primary?

Almost all do.

**Confirmed in a primary**
- **Report lines 4-8** (commit ids, parent 56e7c34, subjects): gate.log 1039-1040 and 1041-1045 (in the file).
- **75479a6b and 9e10adfe:** gate.log 1025-1041.
- **3d8b8aa8:** gate.log 1389-1391.
- **Nine PASS lines:** gate.log 1354-1365.
- **531, 8 added, 0 removed:** test.log 1291, 1870, 1905.
- **Negative log:**
  - N0-N5: negative.log 598-649.
  - The "cannot create the output file" message: negative.log 637 and 644.
  - Family P: negative.log 657-679.
- **2600 bytes:** run.log 977.
- **The new line:** run.log 991 and 1000.
- **ed286608 and 2020, 45963b48 and 2095:** run.log 994-995 and 1003-1004.
- **Five entries identical:** run.log 1034-1069.
- **lo_check results:** run.log 1072-1080.
- **Keep figures:**
  - 155 pairs: run.log 1200.
  - 149 P1 rows: run.log 1200.
  - 6 new pairs: run.log 1201.
  - 310 files, identical on both targets: run.log 1188.
- **Open check (144/11, 145/10, findings 0):** run.log 1381. The record .md totals agree.
- **Stage and zip:**
  - bcda16d9: run.log 1561.
  - 332 files: run.log 1564.
  - b25fdf16, 33012799 bytes, 332 entries, testzip ok, mode 600: run.log 1573-1574.
  - The zip path: run.log 35 with 1573.
- **99b3376d:** commit-fc82ca7.diff 136 and 165.
- **Record hashes 668c716f and c62398c8:** run.log 1550-1554.
- **"RESULT: PASS with 0 hits":** scan-pre-check.log 1501 and 1505.
- **"Seven crate tests and one tool test":** test.log 1932-1949.

**Not in a primary, or different**
- **Report line 43, "the four files this phase changed":** no primary gives four. test.log 1923 prints "the .rs files the phase changed: 5". See F6.
- **Report line 13, "(P2 rulings 1 to 6)":** attached to the --new-sample bullet. The commit message cites only ruling 4 for that part (e7c7061 21-24). See F7.
- **Report line 16 ("each a first attempt") and line 42 (unlogged development runs):** these are disclosures, and no primary can confirm them. No log carries a rerun header, which is consistent with them.

## Findings

**F1 (gap): the negative log rebuilds the binary instead of copying the gate's in.**
- specs/0001 asks for the gate's built binaries to be copied in rather than rebuilt (0001-bootstrap.md 88-92). P1 verification ruling 1 says "uses a worktree or clone as before" (spec 761-770).
- negative.log 593-596 rebuilds emend-copy in the worktree instead. The binary it runs (a09faf16) is therefore not the gate's (gate.log 1379, bc793bcc). No negative line ties it to the gate build.
- The draft report does not disclose this.
- The whole negative run took 7 seconds (negative.log 31 and 689), which suggests a warm `<build>/target`. The log does not say so.

**F2 (gap): no P2 record pins the template's sha256s.**
- The spec says each template file is "pinned by its sha256 in P2's run log and record" (spec 99-100).
- The run log pins them (run.log 968-979). The only P2 record, m2p1-open-2026-09-25-e7c7061.md and .txt, does not carry them: grep finds 0 of the hashes.
- P2 also says "No corpus record" (spec 543). Planning should rule on which record was meant.

**F3 (nit): the first code commit does not cite the P2 rulings.**
- Spec 838 says "P2's first code commit cites them by number". 5f6505a cites only P1 verification ruling 3 (5f6505a 8-9).
- The rulings are cited in e7c7061 (lines 6, 10, 15-16, 23), the commit that implements them.

**F4 (nit): close-error overwrites an earlier read-error message.**
- When a close throws after a read error, `$message` is overwritten (5f6505a 59). The read-error text is lost, although the seen columns still hold read-error.
- The ruling does not address this case.

**F5 (nit): package.rs over-states that new() cannot fail.**
- package.rs says that on the fixed bytes neither write_to nor open refuses (e7c7061 37-42 and 142-146).
- The new limits text (e7c7061 65-67) says the write goes through a temporary in the system's temporary directory. That write can fail with an I/O error whatever the bytes are.
- The spec's own wording ("which no test can reach") is kept.

**F6 (nit): "four files" in the report.**
Report line 43 says four files. No primary prints that count, and test.log 1923 prints 5 .rs files changed.

**F7 (nit): ruling attribution in the report.**
Report line 13 attributes "P2 rulings 1 to 6" to the --new-sample bullet. The commit message ties --new-sample to ruling 4 only.

**F8 (nit): a count in the e7c7061 message that no log prints.**
- The message states "the 22-byte end record" (e7c7061 11-12). 0001-bootstrap.md 221-224 requires a count in a commit message to be printed by a committed log, or the message to say no log prints it.
- No log prints 22. The figure is the spec's, and a test asserts it (tests 274).

**F9 (nit): a root line in the negative log is imprecise.**
The root line says the binary "is built and run in `<tree>`" (negative.log 36). It is built into `<build>/target`, outside `<tree>` (negative.log 39, 596).

Verdict: the P2 diff does what the spec and rulings 1-6 ask and nothing else, every required log line is present, and the report's figures match the primaries except the "four files" count and the attribution nit, with one process gap (F1) and one spec-wording gap (F2) for planning to rule on.
