# Independent check 1, M2 part 4c P1

Written by the implementing session: the checked phase commit is
40ae039ae33239953bf194caed753315fe9ef7fc, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk (its prompt named it
<phase>/check1 and asked for <home> and <repo> in place of machine paths),
holding these files, by sha256. The logs there are the four this phase
committed, gate, test and negative at their first attempt and run at its
second, and the scan's pre-check stage, which the final scan.log follows:

    80c4407a62df17f1bc5edc9a87b20ba6fe97e04af237085a3fa00881b8a85142  commit-messages.txt
    74301ed5a64ff694591d8e91fcf93d4f05bb3726f6e7da2a206d7926fbed26e7  diff-0985ac4.diff
    c0b5611f2299df66e03e791f523f2ab31e4171f115ef97d0e835c49a4933a1c9  diff-2e84516.diff
    81e991e735d0f62be04f2eea9ccecc214aded6d6e7a98c016769cbf947874dc3  diff-40ae039.diff
    85d43ef41c77de8a489b7e5c4f1b1ae0002b99b9e306cf5c5008c7dcb41ff2da  diff-phase-c143473..40ae039.diff
    7e66c0df74b564b50cf9053616df4393bb1ff0587aec75a91a29957b21caba40  doc-commit-1fefcbf.stat
    ee1b4004bda6973be7fb07b8c23a192512af39eaf859c7503e5ed03a3e97383b  gate.log
    516a5e2cfea0f3b182603f6fa94020615b68b617956b4c10becc2fb3db28ea94  m2p4c-open-2026-09-30-40ae039.md
    d5b746db8cb301cc50fd6d312296f45dd95f4eabdb899bb46ca049d3175bb04f  m2p4c-open-2026-09-30-40ae039.txt
    1b2cafcd80ff7be0b53635ebafb38a30baae3e8727d7ffbf36a3b5daeaf392b3  m2p4c-p1-2026-09-30-40ae039.md
    7ec25106302d250ed1c6e1f7464ff56eb92dc8afccb1a19fa5890b5e38130891  m2p4c-p1-2026-09-30-40ae039.txt
    4e90647aa763a6b92196f03dc5946f4666d23f589e157a4a746dfa3933bbe774  negative.log
    d5267f185ddddc05aaedd13f7f1b595a993c7cfe3e5af7c58467e40d26b82880  report-draft.md
    fc1817cb1e4adba69a7b16c774dbb03ca39aae62fbbd2ae76186c8a1624a1362  run.log
    3cd9ecca3f36a2c5aef8109348103b8662fe6348a5de168381328ebd9dc75672  scan-pre-check.log
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  spec-0001-bootstrap.md
    b96f97ac6d36ff1d12075032625f5d1e650cca4f919d1aa442a0fce8da3b23e2  spec-0021-committed.md
    1498597f3b029641d0cf05deab45495df56fc058b5eb9efcbb9ccaf532247140  spec-0021-working-tree-with-P1-rulings.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr/test.stderr
    4b6c70f47786f87ae0b8f6f91d53e83c0b2686add2df2b59eeeb0e897c27b8b6  test.log

One non-ASCII character in the answer is replaced by an ASCII equivalent
(specs/0001, "Rules for every phase": the answer is committed verbatim but
for this): answer line 86, column 217, U+2212 MINUS SIGN written as "-"
(U+002D). The extracted answer before the replacement is 8801 bytes, sha256
6889719f1d3651d6c07f0fb5da7b52e398befdf5f346bb2c65ae6e222a9e3715, kept in
this phase's folder on the disk as check1-answer.raw.

---

Disclosure: the harness put project instructions (CLAUDE.md), auto-memory, git status and a user email into my context. I used none of it as evidence. Everything below comes from `<phase>/check1` only.

## 1. Does the diff do what the spec (with the P1 rulings) says for P1, and nothing else?

Largely yes. There is one contrary doc comment and a few small extras or overstatements.

**First code commit, 0985ac4** (`diff-0985ac4.diff`)
- It holds the three `//!` edits of the 0020 ruling-6 carry: the compose claim now cites the validation test and the new conditional test; a value point's `val` is described as the caller's number as the crate prints numbers; and a number format, alignment or protection is "refused at the call", as is a style with none of a font, a fill and a border.
- It adds P1 ruling 1's test, `a_conditional_format_and_a_cell_edit_on_one_part_compose`, in `conditional.rs`. The test compares the whole worksheet against bytes it builds, so the kept bytes are checked.
- Nothing else is in the commit. `gate.log` confirms that all 29 `package.rs` lines are `//!` lines and that `conditional.rs` loses no line.
- One caveat: 0020's own ruling-6 text is not in my evidence. I could check the phrases only against the 0021 "Phases" summary (committed spec, lines 520-527).

**Crate commit, 2e84516** (`diff-2e84516.diff`), against "Design"
- Present as the spec asks:
  - the feature `tables = []` with no dependency;
  - the four types, with the derives the spec lists;
  - `add_table` and `tables`, with signatures as specified;
  - the nine messages, "no table of that name" left out as P1 says;
  - the catalogue entries.
- The rulings are followed:
  - Name rule bounded by the grid (ruling 5), lines 859-877.
  - Entry names taken over the input archive plus this write's additions, with a removed entry still counted as taken, while the id stays projected (ruling 6), lines 541-613.
  - Relationship type and retag namespace follow the worksheet's own form (ruling 3), lines 616-624 and 1278-1313.
  - A sheet added in the same write gets "no such part" (ruling 2).
  - The ruling-4 readings are implemented: `CellRef::parse` in upper case, the feature's own defined-name reader, a root with no `id` skipped, and the relative target.
- **Contrary:** the public doc comment on `add_table` (lines 341-343) says N is "the smallest positive integer no entry of the projected archive has". That is the pre-ruling-6 rule. The code follows ruling 6, but the doc says otherwise.
- **Extra, not in the spec:** a kept `mergeCell` or `autoFilter` whose `ref` does not parse is refused with the overlap message (lines 1260-1262). The spec says nothing about this case.
- **Extra, early:** the `//!` catalogue already states "no such part" for an added sheet (lines 41-44). Ruling 2 puts that in P3. It is harmless.

**Crate commit, against the P1 test list** (lines 1615-2307)
- Every listed item has a test, including:
  - the read of two tables (custom style, totals row, calculated column, absolute and relative targets) and of none;
  - the three read refusals;
  - literal part bytes with and without a style;
  - `table2` with id 2;
  - successive names;
  - container, template, `extLst` and prefixed-root cases;
  - the overlap cases (kept table, pivot, kept and added merge, autofilter, earlier addition);
  - the header-cell formula refusal;
  - names in use, and a removed sheet's table taking `table2` with id 1;
  - the call-time refusals, with `Table1` and `xyz1` accepted;
  - the shift refusal and the replaced-part refusal;
  - the strict-form test (ruling 3) and the added-sheet test (ruling 2).
- **Overstated test name:** `the_header_cells_are_written_as_text_over_a_value_and_the_row_appended` has no appended row in its body: rows 1 and 3 both already exist. In the crate tests the header row is only ever created inside an empty `sheetData`. An append after existing rows is pinned only by the tool test (`A_CELL`, header on row 2).
- **Overclaim in a comment:** the module's `//!` says "every other part asserted untouched", but only 3 of the 15 tests call `assert_untouched`.

**Tool commit, 40ae039** (`diff-40ae039.diff`), against "Harness runs"
- W is the first sheet of kind Worksheet, and M comes from the tool's cell read.
- The name, columns and style are the spec's constants, and the area is (M+1,1):(M+3,2). A row past the grid is left for the layer to refuse.
- The line's keys come in the spec's order. The +1, +1, +1 and +2 relations are asserted, and a failure removes the output.
- There is no removal step, as P1 requires. Nothing extra.

## 2. Do the logs contain every line the spec requires?

**Spec pin:** every log pins b96f97ac.
- `gate.log`: lines 23-27
- `test.log`: lines 24-25
- `negative.log`: line 63
- `run.log`: lines 4 and 1487
- `scan-pre-check.log`: lines 8 and 51

**Gate** (gate.log output)
- The crate's tests with the feature off and on, on both targets: 512 and 526.
- The pub first lines: 141 off, 143 on, adding exactly `add_table` and `tables`.
- The consumer crate names the six gated items, with a control.

**Test log**
- Off and on on both targets, and the workspace run of 826 with the feature on.
- 18 tests added, none removed.
- Plants T1 to T7.
- The template test's printed output (the "the tables template test's printed parts" section).

**Negative log**
- N1 refused with the overlap message and N3 with the two-container message, each with controls: N2 and N0 for N1, N4 and N0 for N3. N5 and N6 are extra.

**Run log**
- 27 of 27 ok on both targets; every flag holds in every workbook; the tool and the comparison agree; outputs identical across the two targets; 12 plants, each turning its own flag.
- The keep regenerated on both targets and compared; the edit open check with 0 outputs refused on an accepted input.
- Part 4b P4's record named by its record commit, 2f7933bd (lines 34-37 and 2704-2712).

**Weak points**
- **The header-row shape.** The spec says `sheetData` "differs by exactly one appended `<row>`". The run log (change CF, lines 146-149) reinterprets that, and treats the 2 "filled" workbooks as passing instead of failing the expectation and leaving the ruling to planning. The report does raise it for planning. But the "filled" branch has no planted control: the `data_rows` plant ran on d8245b84086f, which is an "appended" case.
- **The strict form never runs on the corpus.** `strict=0` in all 27 workbooks, so ruling 3 rests on the one crate test and plant T6.
- **The scan.** Only `scan-pre-check.log` exists. It ran last (14:42:43, after the run log ended at 14:42:23) and passed. The final `scan.log` that "Phases" requires is not in evidence yet.
- **A control that cannot fail.** In `test.log`, "the carry test's ok line is absent from the crate's run with the features off" is guaranteed by the test's `rules` gate.
- **One test not in the crate-only counts.** The rowcol-gated test, `a_table_edit_with_a_row_insertion_is_refused`, is left out of the crate-only `--features tables` runs: 14 of the 15 tables tests run there, so 526 - 512 = 14. It does pass in the workspace run (`test.log`, "each added test").

## 3. Does every number, hash and quoted string in `report-draft.md` appear in a primary?

These match:
- All log and record hashes. I recomputed them with `sha256sum` over the copies: ee1b4004, 4b6c70f4, 4e90647a, fc1817cb, 3cd9ecca, 1b2cafcd, 7ec25106, 516a5e2c, d5b746db, and 1498597f for the working-copy spec.
- The doc-commit blobs 5fc132f2, 85ba5204 and b96f97ac (`gate.log` lines 1594-1615).
- The commit ids.
- The test counts: 512, 526, 826, and 18 made of 1 + 15 + 2.
- N0 to N6, the four constants reached and the five stated unreachable.
- 27 of 27, twelve flags, 25 appended and 2 filled.
- The attempt-1 files: e3b0c442 at 0 bytes and bbda3d9f.
- The ASCII and trailer checks (`gate.log` lines 1555-1564).
- The quoted "exactly one appended <row>" matches the spec's "Harness runs" text.

These do not appear in a primary, or go beyond one:
1. **"origin/main 116def3".** No log prints the current origin/main. 116def3 appears only in the ROADMAP diff, as the end of part 4b's close-out push (`diff-phase-...diff` line 20). That supports it historically, not as a present fact.
2. **"2 hold an empty row numbered M + 1 (a row with no cell)".** The run log prints only `row=filled` (lines 2258 and 2268). "Numbered M + 1" follows from `tblcmp.py`'s `num == n` (line 1840). "A row with no cell" is an inference from how M is defined; no log line says it, and the comparison's code also handles a row that holds cells.
3. **"attempt 1" for the gate, test and negative logs.** No log says this literally. It is consistent with RERUN_NOTE and RERUN_FILES being unset in each log.

I found no number in the report that disagrees with a primary.
