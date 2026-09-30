# Independent check 1, M2 part 4c P2

Written by the implementing session: the checked phase commit is
6cfd5041c9bff1fb3c92273854ae8494090d11e3, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk (its prompt named it
<phase>/check1, asked for <home> and <repo> in place of machine paths and
for plain ASCII), holding these files, by sha256. The logs there are the
four this phase committed, each at its first attempt, and the scan's
pre-check stage, which the final scan.log follows:

    535d652918660b3dfd1cd70769bb5dc4be36eec3f36f6f8aca6f8e18ad96dcff  commit-messages.txt
    a0961b11fe1a2d43541005045b1c6204fdc21a542ec16629db5625d213dd9baf  diff-2b1cd7d.diff
    d599b065e5ca8960848295d3b113dea1df5515902d44b1f7c4418f7da89d1f01  diff-4a68144.diff
    034ea9d63fd1a94f57fdc717e2caf2a82da96da32e5dd5b438631e18c41fccc0  diff-6cfd504.diff
    845736d9e8dce59b04199842dba9aa996a1b2033fd8a2a43ae803161fd90c9b9  diff-phase-7176a34..6cfd504.diff
    611cee112bf2f66dc3f7d53678e6f2d66b1b218ae49a50fed3b83076ed814de0  doc-commit-30241a1.stat
    c4bbb0577b3fbca9a872ecda14d0466a1f47b35f622040477d25489d59b84812  gate.log
    1d52cbd73569e4306851457abf1b1fcae19524552a344c5af7df485390e5b3fa  m2p4c-open-2026-09-30-6cfd504.md
    4737226ac20d18aa4906d5e214b97d439ad184c9d8fda3396a7aaa4bcf3d91e9  m2p4c-open-2026-09-30-6cfd504.txt
    9948fde8adb2016205dd3611605f428c169aafdd9d14f271c2d6539d022d3351  m2p4c-p2-2026-09-30-6cfd504.md
    e9d5e3945b9356c9ace2bba9ea62df8df6f71f30e7c532a6c33c6bbbf6fe072c  m2p4c-p2-2026-09-30-6cfd504.txt
    f829c234a90260293899324828f0fd829dbfdb82399ee4e2ec3d43af38da147a  negative.log
    5993f6ae1f6930972a91e73a252fd78a013b15499b18a5cb0a88c7f663f2ec0e  report-draft.md
    1fd88fc706371e1ba31721878c5f571dd5be288c5a7900867c8cb868b2c3c6e9  run.log
    6ad3ec27a1d49c98934c8d8467cd7829a9886d8d172a14b0b7ceffc1f4c8ad8c  scan-pre-check.log
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  spec-0001-bootstrap.md
    3bea86d571105fa3383a268f8938dd34a74ca77a5b57b5d92a09723550eeb8ea  spec-0021-committed.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/test.stderr
    532971903184d5cf05e2dcf3271fef45a4545794f59051ad9d92754fe6fbf101  test.log

---

Disclosure: the harness put project instructions, auto-memory, git status and user details into my context. I did not use any of them as evidence. Everything below comes from files in <phase>/check1. spec-0021-committed.md hashes to 3bea86d5...eb8ea, which is the pin every log states.

**1. Does the diff do what the spec says for P2, and nothing else?**

**Carry commit 2b1cd7d, against "Planning verification and rulings on P1", ruling 5**
- **P1 ruling 5.** `check_defined_name` now calls the shared `names_a_cell`, which is `CellRef::parse(...).is_some_and(in_grid) || is_r1c1_form(name)`. Before, it used an unbounded `is_some()` (diff-2b1cd7d.diff, hunks at 2898 and 2926). names.rs adds "Table1" and "xyz1" to the accepted list. That list really is the accepted one: plant R6 fails with `"Table1" refused` (test.log, section "seven planted changes").
- **P1 verification ruling 4.** `names_a_cell` is gated `any(metadata, tables)` and `check_table_name` calls it. tables.rs adds "R1C1" and "rc" as refusals. Two gaps:
  - For a defined name, the commit message says R1C1 and rc "stay refused". No primary I hold shows those existing names.rs lines, and no plant targets the defined-name side of the R1C1 form (R5 plants against the tables test only).
  - The ruling names `RC`; the test pins the lower-case `rc`. Case is folded, so this is equivalent, but it is not literally the ruling's string.
- **P1 ruling 7 (A).** `add_table`'s doc now gives ruling 6's rule ("an entry this write removes still counting as taken"). gate.log, section "each code commit touches its own area only", shows the old sentence at count 0 and the new one at 1.
- **P1 ruling 7 (B).** The tables `//!` is narrowed to the tests that call `assert_untouched`, but it cites "P1 verification ruling 5" rather than P1 ruling 7 (B), a small miscitation. The header-cell test adds A7:B8, appended after rows 1 and 3. That matches the ruling.
- **Extras:** the doc lines of `add_defined_name` and `check_table_name` change to match the new rule. These are consequent, not extra behaviour. Nothing contrary.

**Removal commit 4a68144, against "The calls" (`remove_table`) and the P2 test list**
- **Matches the spec:**
  - The signature matches.
  - Names are matched case folded against `name`/`displayName`.
  - The four removals are made: the relationship, the Override when present, the entry, and the `tablePart` skipped in `TablesPass::child`.
  - An empty container is dropped (`close` returns nothing at count 0).
  - A name this write only adds finds no file table, so it gets "no table of that name".
  - A missing table part gives the read message.
  - Removals are recorded before additions (`record_tables` calls `record_table_removals` first).
- **All nine P2 test-list items are present** across the five new tests (diff-4a68144.diff, from line 850).
- **Extra behaviour:** a second removal of the same table is refused with "no table of that name". The spec does not state this. It is disclosed as stated reading 1 and pinned by a test.
- **Weak:** "the cells untouched" is asserted over `two_tables()`, whose sheetData is empty. Only the last-table test carries a cell.
- **Unspecified or untested:**
  - `remove_table` on a part absent from the archive returns "no table of that name" (`index_of(part).is_none()`). "The worksheet" says a table edit on a sheet added in the same write meets "no such part", and the P2 diff does not show which message wins.
  - `table_removals.retain(...)` (hunk at 6409) drops removals for a part in some path. No test pins `remove_table` on a removed or replaced part.
  - The order of "a removal followed by an addition" is not enforced; the code is order-independent.

**Tool commit 6cfd504, against "Harness runs"**
- It removes the table on the reopened output, writes into a `Cursor`, reopens, and asserts `tables_removed == tables_before` and `parts_removed == parts_before`. The output is removed on failure, and both keys are printed. This matches.
- `removed.rels` is computed but neither printed nor asserted. That is harmless, not required.
- Nothing contrary found.

**2. Do the logs contain every line the spec requires?**

- **Spec pin:** gate, test, negative, run and scan-pre-check all pin 3bea86d5 and print the working copy separately.
- **Stderr:** every stderr file is 0 bytes.
- **Test counts:**
  - Feature off and on, both targets: 512 / 531 (gate.log and test.log).
  - The workspace: 831 on both targets, 5 added, 0 removed.
- **Negative log:** N6 (a relationship naming a missing part) is refused with `TABLE_UNREADABLE`, with N2 as its control (negative.log, "the checks"). NO_SUCH_TABLE is stated unreachable with its reason.
  - **Weak:** N6's refusal comes from `add_table`'s write-time read (P1 code) before the removal is ever reached. It would have come out the same without P2. It meets the spec's wording, but it does not exercise the removal.
- **Run log:**
  - 27 of 27 ok on host and i686, and the outputs are byte-identical across the targets.
  - `line_as_spec` checks `tables_removed`/`parts_removed` against the before values, with a control (lin_ctl2).
  - **Weak:** the tool exits 2 without printing unless its own assertion held, so on an ok row the log's check of these keys cannot come out otherwise. The removal's output is never extracted, so no byte-level check shows W, its rels part or the content types restored on the corpus.
- **P1 ruling 7 (C):** "the data_rows plant on a filled header row: applied on 75387404b8af; row=filled; data_rows=no", with its control. It is host only. The plant is the same `emend_b` to `emend_c` substitution the appended case uses. That suffices, because data_rows is a whole-sheetData equality.
- **Flag plants:** 12 of 12 turned their flag.
- **Keep and open check:**
  - The keep is regenerated on both targets (1158 files, identical).
  - It is compared against P1's open-check record, named by commit 7176a340 (a typed `PREV_OPEN_REC`, cross-checked against the subject).
  - The edit open check ran. The 4 new pairs are ok/ok. No P1 verdict moved, and no output was refused on an accepted input.
- **Test plants:** R1 to R7 each pass unplanted and fail planted. No plant covers:
  - relationship or entry removal (the test asserts both, though);
  - the projected removal-plus-addition test;
  - the missing-part refusal;
  - the double-removal refusal.
- **Attempt numbers:** no log prints one. "Attempt 1" rests only on RERUN_NOTE and SUPERSEDED being unset.
- **Scan:** scan-pre-check.log says PASS with 0 hits. It is not the final scan; the final `scan.log` must still be written last, after the check answers.

**3. Does every number, hash and quoted string in report-draft.md appear in a primary?**

These match primaries:
- Log hashes: gate.log c4bbb057, test.log 53297190, negative.log f829c234, run.log 1fd88fc7, scan-pre-check.log 6ad3ec27.
- Record hashes: 9948fde8, e9d5e394, 1d52cbd7, 4737226a (checked by sha256sum and in run.log's install lines).
- Doc-commit blobs: specs/0021 at 3bea86d5 and ROADMAP.md at dc334d27.
- Commit ids: 7176a34, 30241a1, 2b1cd7d, 4a68144, 6cfd504.
- Counts: 512/531, 831, five added and none removed, R1 to R7, N0 to N6, 27 of 27, 12 of 12, and 0 hits.
- The public-list lines (gate.log 2039 to 2046).
- The 23 typed test edits.
- Stated reading 2 ("no corpus workbook has one"): the corpus record's .txt lists every entry, 903 rows per label, and none is under `xl/tables/_rels`.

Not found in, or not supported by, a P2 primary:
- **"origin/main 116def3":** appears only in the spec (P1 verification ruling 3, as P1's report fact). No P2 log prints it. The ruling allows a report to state such a git fact.
- **"1c1c3fc", and the amend story around it:** appears in no primary.
- **"attempt 1" for each log:** no log states an attempt number. It is inferred from RERUN_NOTE unset.
- **Stated reading 1:** the diff supports the refusal itself. The parenthetical "the removal is checked against the projected workbook" is the report's gloss: the code refuses a duplicate target within the recorded removals.

No number in the draft disagrees with a primary.
