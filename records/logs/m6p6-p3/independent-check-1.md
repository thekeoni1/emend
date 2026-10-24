Check 1, M6 part 6 P3 (specs/0043 "P3: the close", rulings M to Q)

1. Does the diff do what the spec says, and nothing else?

Verdict: yes, with one doubt about test coverage.

Evidence:
- The range has two commits. 7f3b43c's parent is 3102354, and 7ac8b74's parent is 7f3b43c. `git diff --numstat 3102354 7ac8b74` gives the six files and the counts the draft states.
- Doc commit 7f3b43c touches only `ROADMAP.md` (1/1) and `specs/0043` (138/0).
  - The ROADMAP change is the P2 row turned from PLANNED to DONE.
  - The spec it commits has sha256 cc1b5d90..., and ROADMAP has d585c8e8..., both as its commit message states.
  - The committed spec holds rulings H to K, "Planning verification of P2" and ruling L.
  - The working spec (9c3809ca) differs from it only by the added section "Planning rulings during P3" (M to Q).
- Code commit 7ac8b74 touches only `CHANGELOG.md`, `package.rs`, `package/tests.rs` and `package/tests/chartsheet.rs`.
  - Ruling M's form: `rewrite_workbook_part` records `strict = namespace == Some(MAIN[1])`. When `rels_prefix` is None, `write_sheets` writes `xmlns:r="RELATIONSHIPS_ATTRIBUTE[strict]" r:id=...` between sheetId and r:id. MAIN[1] and RELATIONSHIPS_ATTRIBUTE[1] are both the strict namespaces, so the index pairs them correctly.
  - When the root binds a prefix, the old path (`out.write_all(rels)`) is unchanged.
  - A prefixed root's add still returns `SHEET_ADDED_TO_PREFIXED`, and the existing test `a_prefixed_workbook_takes_a_rename_and_a_move_and_refuses_an_add` is untouched.
  - `NO_RELATIONSHIPS_PREFIX` is gone, with its two sites, its catalogue entry and the chartsheet limit bullet. `git grep` finds neither the name nor the message under `crates/`, `tools/` or `scripts/` at 7ac8b74. The message survives only in `CHANGELOG.md`'s quotation and in older records.
  - The refusing test is renamed `..._takes_an_add`. It asserts the exact workbook bytes, reopens the output, and checks the names, the part through its relationship, and the kind.
  - The new test `an_openpyxl_form_workbook_takes_a_sheet_and_a_chartsheet` builds the openpyxl form in the test (each `<sheet>` declaring `xmlns:r`, `state="visible"`). It adds a sheet and a chartsheet, checks that the kept `<sheet>` is byte for byte, reads back the kinds and the chart, and commits no fixture (reading 1 (a)).
  - `CHANGELOG.md` gains one 5-line bullet under "### Fixed", inside the code commit (G-R1).
- Nothing outside this scope moves.
- Doubt: no test exercises the strict branch (`strict == true`) of the new declaration. Both tests use the transitional `MAIN_NS`/`ATTR_NS`. Ruling M accepted the strict pairing as a reading but did not ask for a test, so this is a coverage gap, not a departure from the spec.

2. Do the logs contain every line the spec requires?

Verdict: yes.

Evidence (each log ends "... checks all as expected: yes"):
- **Gate:** the prune of P2's target reads "1 of 1; their bytes in all: 6250546308", then `rm -r` with no `-f`, then "every name of the prune set is gone". The CHANGELOG bullet is checked under "### Fixed", once, citing specs/0043. The distinct messages read 155/154 (one-line reader) and 169/168 (several-line reader), "ruling M's one gone and none added". The sets read off 538, ch 712, im 575, ci 754, cc 757, cr 819, all 1200 on host and i686.
- **Test:** ws 1449 on host and i686, and off 538, read 645, rules 575, read,rules 685, all 1200 on both. The API listing is 844 rows against part 5 P3's 844, 0 gone and 0 new. The record is installed with sha256 c1f704b2....
- **Size:**
  - The format probe goes from 984664 to 984744, a budget growth of 80 within 7304, leaving 7224 under 458752.
  - The charts cost is 166680, moved 16. The other cost movements are "not judged" (S-R1 (a)).
  - The charts cost is stated whole with the six-part chain (decision 2 A).
  - The record is installed with sha256 dfcc7f32.... It cites `specs/0038` ruling O and names specs/0043 ruling N, and the log checks the working spec for ruling N with a control.
- **Negative:**
  - "the 22 cases were refused, N1 to N7 and N9 to N23".
  - The N8 section uses the size log's release binaries, with their hashes checked against the size log's lines, and the kept 834a7f6d input is hash-checked.
  - It runs 6 accepted runs, each with `--survey` read-back, 4 active-tab refusals (N8 and B4 under the sheet sample), and the 4 N8P controls with SHEET_ADDED_TO_PREFIXED. The summary line matches the one the draft quotes.
  - It confirms that the old message is gone from source and catalogue, and that ND42's test has no message literal.
- **Run:**
  - Sheet host and i686, chartsheet and copy host and i686 are each "27 of 27" byte-identical by sha256 to their reference runs, and "6 of 6" is stated.
  - Each of the four openpyxl-form runs prints the exit status, the output sha256, the new `<sheet>` element and "lo_in ok, lo ok", as R-R3 asks.
  - The open check reads "the regenerated keep 2240, the record 2237", 2237 found, and "the new pairs are exactly the 3 this phase's tests write".
- **The 10 records:** the run log installs eight, the size log one and the test log one. All ten are present untracked, and their hashes appear in the logs.
- **README:** it states the order (gate, test, size, negative, run) and why: the negative reads the size log's release binaries. It also records F5, its cause and the move.
- **Scan pre-check:** "total hits: 0", "structural checks failed: 0", "RESULT: PASS".

3. Does every number and quoted string in the draft appear in a primary?

Verdict: yes, with three exceptions.

What I verified:
- The hash of every log, of the README, of `scan-pre.log` (590284ed, also at <phase>/scan-pre/scan.log) and of both drivers (e4764992 and d77621d2) matches the draft. Both record hashes match the files.
- Both full commit hashes and subjects match git.
- The numstat matches git.
- origin/main resolves to ee78ffa. f15a11b is an ancestor of HEAD and is on no remote branch.
- The two drivers differ in exactly the two lines the draft names.
- <phase>/superseded/logged-refused-1 holds `logged.exit` "2", `logged.out` "negative exists", and a copy of the refused driver that is byte-identical (cmp) to the copy given to me.
- <phase>/dry/d1/negative-build exists.
- Every figure in the gate, test, size, negative and run bullets appears in the logs as quoted.

Exceptions:
- **(a) Ruling L is already committed.** The draft (and the README) says "Planning's rulings L to Q are in the working specs/0043, uncommitted". Ruling L is already in the spec committed at 7f3b43c, under "Planning verification of P2". Only M to Q are uncommitted. The README also calls the committed spec "rulings H to K", which leaves out the L it contains.
- **(b) The size finding is not reported.**
  - The size log prints "the findings ...: 1" and "FINDING: probe-emend-tables-write-xml: its growth over M6 part 5's record, -32 section bytes, shrank 32, a FINDING (expected 0 to 80, accepted up to 2080)".
  - The size record carries the same finding.
  - The draft's "Finding during P3" lists only F5, and neither the draft nor the README mentions this shrink.
  - The size log still ends "yes", so the finding is recorded and not fixed, as the script intends. But the draft's statement of findings is incomplete, and planning has no ruling on this finding in L to Q.
- **(c) The F5 rerun departs from ruling Q's text, and only the session's word covers it.**
  - Ruling Q says the logged script "reruns unchanged from the start at the same `PLANNING_SPEC`".
  - The rerun's driver was changed instead: its PLANNING_SPEC is 9c3809ca, not 72f241bc, and its header says "A to Q".
  - The draft states this openly as the session's statement, along with "planning accepted it by message". No primary I was given carries planning's acceptance. The rerun is consistent in itself, since the spec did move to 9c3809ca when Q was added, but the acceptance needs planning's own confirmation.

Minor points:
- The draft's run bullet lists "sheet host" but leaves out the sheet i686 rerun. The log has that rerun at 27 of 27, held to the host reference rows (R-R1).
- The draft's ".." in the quoted `<sheet>` element stands for the tool's constant sheet names. That matches the log apart from the elision.
- "Nothing pushed (session's statement)" is consistent with the local origin/main ref. I made no network check.
