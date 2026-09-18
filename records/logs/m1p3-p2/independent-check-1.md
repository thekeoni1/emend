# Independent check 1: emend M1 part 3 P2 (the cell reader)

Checker: a fresh session with no access to the implementing conversation. Judged
only from `specs/0010-reading-cells.md`, `specs/0001-bootstrap.md`, `CLAUDE.md`,
`ROADMAP.md`, the phase diff `1f4481d^..6c27167`, the five logs in
`records/logs/m1p3-p2/`, the record pair, and the draft report. I read no file
under the corpus directory, ran no corpus runner, and changed nothing in the
repository. My own runs: `git`, `grep`, `cargo test --workspace`,
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -D warnings`.

`records/logs/m1p3-p2/scan.log` was given to me as its FIRST run and is judged as
such; where its content depends on being the last log written I say so.

**Verdicts in one line each.**

1. **Does the diff do what the spec says, and nothing else? Mostly yes, with
   three clauses not carried out.** Every one of the spec's P2 test cases is
   covered by a named test, the reader matches the design, and the diff contains
   no feature the spec did not ask for. But `lib.rs` does not carry the
   re-export list the spec's "Crate" section requires (C1), P1 ruling 6's
   catalogue requirement is not done (C2), and ruling 1/ruling 12's `bytes`
   1497091 corpus assertion is made nowhere (C3). One test comment claims a
   property its assertions cannot see (C11).
2. **Do the logs contain every line the spec requires? No.** `gate.log` does not
   carry the phase-commit rule that this phase's own first commit added to
   `specs/0001`, and `run.log` carries it without the control (C4). Everything
   else the spec and `specs/0001` ask for is present, including the four-log
   shape, the scan log's self-scan, the three shape greps, and the sha256 rule.
   `run.log`'s account of finding P2-F1 is a measurement, not a story.
3. **Does the report's every number and quoted string appear in a primary?
   Almost.** Every figure I could check traces to a primary and is correct. Five
   report sentences are not supported by the primaries as written (C4, C5, C7,
   C8, C10), and one number (107 deletions) appears in no log, though `git`
   confirms it.

---

## 1. The diff against the spec's P2 section

### 1.1 The named items

`specs/0010-reading-cells.md:330-331` names
`cells::{Cell, CellRef, Cells, Formula, FormulaKind, Kind}`, `Package::cells`,
`SharedStrings::resolve`, and `--cells`.

| clause | where | verdict |
| --- | --- | --- |
| `CellRef { row: u32, col: u32 }`, 1-based (spec:94) | `crates/emend/src/cells.rs:250-256` | carried |
| `parse` accepts capitals then digits, no leading zero, `None` otherwise or past `u32` (spec:94-98) | `cells.rs:265-301` | carried |
| `a1()` writes it back (spec:98) | `cells.rs:304-321` | carried |
| No sheet-size limit enforced (spec:98-100) | `cells.rs:246-249`, no bound in `parse` | carried |
| `Kind` seven variants, `t` absent or `n` is `Number` (spec:101-103) | `cells.rs:329-344`, `346-360`, `read_opening` default `cells.rs:798` | carried |
| any other `t` refused "a cell has a type this layer does not know" (spec:103-106) | `cells.rs:357` | carried |
| `Cell { reference, style, kind, value, formula }` (spec:108-110) | `cells.rs:413-428` | carried |
| `value` is `<v>` unescaped, `Some("")` for `<v/>`, `<is>` text for inline, `None` for neither (spec:110-113) | `cells.rs:665-688`, `713-730` | carried |
| nothing trimmed (spec:113-116) | `push_text` `cells.rs:1107-1126` does no trimming | carried |
| `Formula { text, kind, reference }`, text empty for a follower (spec:117-119) | `cells.rs:394-404`, `675-679` | carried |
| `FormulaKind` four variants; `si` absent refused "a shared formula has no index"; other `t` refused (spec:119-127) | `cells.rs:363-387`, `893-911` | carried |
| `ca`, `aca` and every other `<f>` attribute not modelled (spec:126-128) | `cells.rs:884-891` reads only `t`, `si`, `ref` | carried |
| `SharedStrings::resolve` semantics and the two distinct refusals (spec:140-143) | `cells.rs:227-241` | carried |
| `Package::cells(&mut self, part: &[u8])`, "no such part", records no edit (spec:185-189) | `package.rs:622-626` | carried, see C20 |
| `--cells`: one line, the word `cells`, 23 keys in the spec's order (spec:244-257) | `tools/emend-copy/src/main.rs:502-541` (field order identical to spec:246-257), `553-621` | carried |
| a `resolve` failure counted, run continues; any other parse error is exit 2 (spec:253-257) | `main.rs:609-612`, `main.rs:202` and the tool's existing exit-2 path | carried |

Two deviations from the spec's literal words, neither behavioural:

- **C20 (minor).** `specs/0010:185-188` says `Package::cells` opens the entry
  "through `read_part` and `Stream`". `package.rs:623-625` calls
  `self.archive.read(index)` directly; `read_part` (`package.rs:552-554`) is a
  one-line wrapper around exactly that call, so the behaviour is identical.
- **C19 (latent, worth planning's eye).** The spec does not say what
  `resolve` does for a `SharedString` cell with NO value.
  `cells.rs:231-233` answers `Ok(None)`, documented at `cells.rs:219-222`. The
  tool then counts that cell as **`resolved`** (`main.rs:609-612`: `Ok(_) =>
  counted.resolved += 1`), while `run.log`'s independent reader counts the same cell as
  **`unresolved`** (`run.log`, the reader's `if index is None or index >= items
  ... unresolved += 1`). The corpus has no such cell (`shared` 783275 =
  `resolved` 783275, `run.log:2131-2132`) and no fixture builds one, so the
  whole-line agreement between the two routes does not cover this case, and no
  test covers it either. If part 4 or a future corpus meets one, the two routes
  will disagree and the run will read as a defect in one of them.

### 1.2 The spec's P2 test list, item by item

`specs/0010:331-350`. Every case is covered; I found no case with no test. Test
names are in `crates/emend/src/cells/tests.rs` unless marked.

| spec case | test | line |
| --- | --- | --- |
| every kind | `every_cell_type_reads_as_its_kind` | 583 |
| `t="n"` and `t` absent both `Number` | `a_number_is_a_number_with_or_without_its_type` | 634 |
| `<v>` text | `a_value_may_be_text_empty_or_absent` (`Some("7")`) | 651 |
| `<v/>` | same test (`Some("")`) | 651 |
| no `<v>` | same test (`None`, both `<c/>` and `<c></c>`) | 651 |
| inline string of one `<t>` | `an_inline_string_is_the_text_under_its_is` | 665 |
| inline string of runs | `the_runs_of_an_inline_string_concatenate` | 674 |
| inline string with `<rPh>` | `a_phonetic_subtree_inside_an_inline_string_is_skipped` | 685 |
| a normal formula | `a_normal_formula_is_its_text` (both `t` absent and `t="normal"`) | 698 |
| a shared master with `ref` | `a_shared_master_carries_its_range_and_its_follower_carries_no_text` | 717 |
| its follower with empty text | same test (`follower.text == ""`) | 717 |
| an array formula | `an_array_formula_carries_its_range` | 751 |
| `dataTable` accepted by name | `a_data_table_formula_is_accepted_by_name` | 762 |
| `ca` and `aca` present, not modelled | `a_formulas_other_attributes_are_not_modelled` | 777 |
| a character reference in `<v>` | `a_character_reference_is_expanded_in_a_value_a_formula_and_a_text` | 793 |
| ... in `<f>` | same test (`CONCAT("x",A1)`) | 793 |
| ... in `<t>` | same test (cell B1's `<is><t>`) | 793 |
| `parse` on `A1` | `a_cell_reference_parses_only_the_a1_form` | 810 |
| ... `XFD1048576` | same | 811 |
| ... `a1` | same | 820 |
| ... `A01` | same | 821 |
| ... `A` | same | 822 |
| ... `1` | same | 823 |
| ... `A1B` | same | 824 |
| ... a column beyond `u32` | same (`MWLQKWU1` Some, `MWLQKWV1` None) | 832-839 |
| ... `a1()` back | `a_reference_writes_itself_back` (7 spellings incl. `Z1`/`AA1`) | 852 |
| a row without `r` | `a_row_without_a_number_follows_the_last` | 863 |
| a cell without `r` | `a_cell_without_a_reference_follows_the_previous` | 877 |
| a cell whose reference names another row | `a_cells_own_reference_wins_over_its_row` | 890 |
| a style index | `a_style_index_is_carried` (0, 164, absent) | 910 |
| unknown elements skipped with subtrees (`extLst` in `<c>`, `conditionalFormatting` beside `sheetData`) | `an_unknown_element_is_skipped_with_its_subtree` (both places in one fixture) | 923 |
| elements in another namespace skipped | `a_cell_in_another_namespace_is_not_a_cell` | 940 |
| the strict main namespace | `the_other_main_namespace_spelling_is_read_too` | 956, see C12 |
| empty `<sheetData/>` gives no cells | `an_empty_sheet_data_gives_no_cells` (and no `<sheetData>` at all) | 970 |
| error: unknown cell type | `a_cell_type_this_layer_does_not_know_is_refused` | 1005 |
| error: unknown formula type | `a_formula_type_this_layer_does_not_know_is_refused` | 1015 |
| error: shared formula without `si` | `a_shared_formula_without_an_index_is_refused` | 1023 |
| error: a bad reference | `a_cell_reference_that_is_not_a1_is_refused` (`r="A1:B2"`) | 1041 |
| error: a bad row number | `a_row_number_that_is_not_a_number_is_refused` | 1049 |
| error: a bad style index | `a_style_index_that_is_not_a_number_is_refused` | 1057 |
| error: the wrong root | `a_worksheet_root_of_the_wrong_name_is_refused` + `a_worksheet_root_in_another_namespace_is_refused` | 985, 994 |
| error: an unknown entity | `an_unknown_entity_in_a_cell_is_refused` (`<v>`, `<f>`, `<t>`) | 1065 |
| error: non-UTF-8 | `a_worksheet_that_is_not_utf8_is_refused` | 1096 |
| `resolve` on an index that is not a number | `resolving_an_index_that_is_not_a_number_is_refused` | 1171 |
| `resolve` on one out of range | `resolving_an_index_out_of_range_is_refused` (and the empty-table case) | 1183 |
| `resolve` on one in range | `resolving_a_shared_string_gives_the_item_it_names` | 1141 |
| `resolve` on a non-shared cell | `resolving_a_cell_of_another_kind_gives_its_own_value` | 1158 |
| `Package::cells` then `write_to` byte-identical | `reading_cells_then_writing_copies_the_package_byte_for_byte` | 1203 |

That is 48 distinct spec cases, all covered (the two root-refusal tests share one
row). `cells/tests.rs` holds 71 `#[test]`
functions (29 from P1, 42 new), which `test.log:1799` reports as 71 on the host
and `test.log:1803` as 71 on i686; my own `cargo test --workspace` reproduced
333 passing and 2 ignored, and `cargo fmt --check` and
`cargo clippy --all-targets -D warnings` are clean at HEAD.

Tests in the diff that the spec's list does not ask for, all defensible:
`a_shared_index_that_is_not_a_number_is_refused` (1031),
`an_element_inside_a_cells_value_is_refused` (1081, ruling 3 on `<v>`/`<f>`),
`a_worksheet_cut_off_mid_cell_is_refused` (1106),
`the_iterator_is_spent_after_a_refusal` (1117),
`cells_on_a_part_that_is_not_there_is_refused` (1129), plus the two integration
tests in `tools/emend-copy/tests/null_rewrite.rs` and three new argument-refusal
rows there (`null_rewrite.rs:530-540`). None of these is scope creep.

### 1.3 What the spec asked for and I cannot find

**C1 (material).** `specs/0010:85-88`, "Crate": "`lib.rs` re-exports
`cells::{Cell, CellRef, Cells, Formula, FormulaKind, Kind, SharedStrings}`."
`crates/emend/src/lib.rs:35` re-exports `cells::SharedStrings` alone, and
`lib.rs` is not in the phase diff at all (`gate.log:796-805` lists the eight
files; `lib.rs` is not among them). The six P2 types are reachable only as
`emend::cells::Cell` and so on, which is how `tools/emend-copy/src/main.rs:73`
imports them. Nothing in the diff, in any log, or in the report mentions this
clause. It is a one-line change; the report should have either made it or named
it as deferred with a reason.

**C2 (material).** `specs/0010:459-461`, P1 ruling 6: the message "a cell holds
a reference this layer cannot expand" "stands; P2 raises it from a cell as well,
and **the catalogue then lists both tests beside it**." `package.rs:79-80` still
lists only `cells::tests::an_unknown_entity_is_refused`, though P2 added
`an_unknown_entity_in_a_cell_is_refused` (`cells/tests.rs:1065`), which raises
that same message three times. The same omission, not covered by a ruling but
the same pattern, applies to "a text element holds an element"
(`package.rs:81-82` lists only `an_element_inside_a_text_element_is_refused`,
not P2's `an_element_inside_a_cells_value_is_refused` at `cells/tests.rs:1081`).
Nothing catches this: `the_catalogue_and_the_source_hold_the_same_messages`
(`package/tests.rs:2119-2162`) compares message SETS, not the test names beside
them.

**C3 (material).** `specs/0010:434`, P1 ruling 1: "P2's `--shared-strings` run
expects `items` 110582 and `bytes` 1497091, **both asserted by name**", carried
forward by ruling 12 (`specs/0010:486`: "P2's run expects the `bytes` figure of
ruling 1") and by planning's own ROADMAP row for P2, which this phase's doc
commit installed: "the corpus run with every count asserted by name, **`bytes`
1497091**" (`git diff 1f4481d^..1f4481d -- ROADMAP.md`, the P2 row). No log of
this phase asserts it: `grep -rn '1497091\|1497107' records/logs/m1p3-p2/`
returns nothing, and `run.log` mentions `--shared-strings` only in prose
(`run.log:24`, `run.log:741`). The only committed corpus measurement of `bytes`
is P1's **1497107** (`records/m1p3-p1-2026-09-18-71ab06e.txt:5`), taken before
the line-end normalisation landed in `da20c6b`, so the committed corpus figure
for `bytes` is now stale and no new one replaces it. Meanwhile
`crates/emend/src/cells.rs:42-45` states as fact that "`--shared-strings`
reports 1497091 bytes and not the 1497107 a reader without this normalisation
reports". The figure itself is not invented - planning's own Python reader
measured 1497091 (`specs/0010:409-410`) - but this phase's reader has never
been run over the corpus to produce it, and `specs/0001:214-216` says nothing is
called measured unless a log prints the measurement. The report's "What is not
done" does not list it.

### 1.4 Position or mechanism rather than property

I went through all 42 new tests. The assertions are properties: kinds and values
compared with what the fixture wrote, references compared as A1 strings,
`with.formula == without.formula` for the unmodelled attributes (777), a
round trip for `a1()` (852), `next()` returning `None` twice after a refusal
(1117), and output bytes equal to input bytes for the copy (1203). No assertion
depends on where an element sits or on how the reader is built. Two items are
worth naming:

**C11 (a comment claiming what the test cannot see; the recorded defect class,
in a comment rather than an assertion).** `cells/tests.rs:950-952`:

    // C1 keeps its own reference; the skipped element moved no count, which
    // is visible because the third cell names itself.
    assert_eq!(cells[1].value.as_deref(), Some("3"));

The third cell carries `r="C1"` (line 947), so `read_opening`
(`cells.rs:809-814`) takes the reference from the attribute and the running
column count is overwritten either way. Whether the skipped `other:c` moved the
count is exactly what this fixture CANNOT show, and the assertion under the
comment checks the value, not the count. The sentence is self-contradictory as
written ("visible because the third cell names itself" is the reason it is not
visible). Dropping the `r` from the third cell would make the claim a
measurement: a reader that counted the foreign element would then report `D1`.

**C12 (a position dependency, benign).** `cells/tests.rs:963` builds the strict
worksheet namespace as `String::from_utf8_lossy(MAIN[1])`. `MAIN[1]` is the
strict spelling today (`package.rs:160-163`), but the test pins an INDEX: reorder
`MAIN` and this test silently exercises the transitional spelling and still
passes, so the spec's "the strict main namespace" clause would no longer be
covered. P1's `the_strict_namespace_is_accepted` (367) spells the literal for
the `sst` root; this one should do the same.

### 1.5 The two corrected expectations, M6 and M7

Both claims are printed measurements, as `specs/0001:190-196` now requires.

- **M6.** Planted at `test.log:1928-1929` (`MWLQKWU1` -> `FXSHRXW1`,
  `MWLQKWV1` -> `FXSHRXX1`), applied and run at `test.log:2068-2074`, and the
  failing assertion printed at `test.log:2076-2079`: `left: Some(CellRef { row: 1, col:
  2147483647 })` against `right: Some(CellRef { row: 1, col: 4294967295 })`.
  I verified the arithmetic myself: `FXSHRXW` = 2147483647 = `i32::MAX`, and
  `MWLQKWU` = 4294967295 = `u32::MAX`, `MWLQKWV` = 4294967296. The corrected
  expectation is the one the spec implies (`specs/0010:97-98`: `None` "for a
  value that does not fit `u32`"), and the pair is the right form: the largest
  that fits and one more.
- **M7.** Planted at `test.log:1932`, applied and run at `test.log:2085-2091`, failing
  assertion at `test.log:2092-2095`: `left: ["B7", "C1"]` against `right: ["B7", "C7"]`.
  I believe `C1` is the expectation the spec implies, and I say so
  independently of the report: `specs/0010:159-161` says a `<c>` without `r` is
  "the previous cell's column plus one **within its row**", and
  `specs/0010:164-167` says only that a cell's OWN reference wins over the
  enclosing `<row>`. A cell with no `r` has no own reference to win with, so the
  row it sits in supplies the row number and the previous cell supplies the
  column. The implementation does exactly that (`cells.rs:583-594` passes
  `self.row` and `self.col` to `read_opening`, which builds
  `CellRef { row, col: previous + 1 }` at `cells.rs:828-831`). The alternative
  reading - that the malformed neighbour's row propagates - would require the
  reader to treat a cell's `r` as moving the row cursor, which no clause asks
  for. The report is right to put this in front of planning; my own reading
  agrees with the reader.

Both mutations edit a TEST expectation rather than the code, which is the
correct shape for a claim about an expectation: what is being measured is that
the assertion discriminates. The other five mutations (M1-M5) edit the code and
are printed the same way (`test.log:1948-2067`), with the unmutated worktree
passing as the control (`test.log:2101-2108`: `exit: 0`, then 249/0/14/0 and "the unmutated worktree
passes, so each failure above is the mutation's: yes").

### 1.6 The design clauses

- **`u64` where a count could exceed 32 bits.** `CellRef::parse` accumulates in
  `u64` with a bound check per step (`cells.rs:283-296`), so a column or row past
  `u32::MAX` returns `None` rather than wrapping, on i686 too. `row`, `col`,
  `style`, `si` and the table index are `u32`, which is what
  `specs/0010:88-90` requires ("`u32` where the format says so"). `len()`
  saturates rather than wraps (`cells.rs:117-119`). Every one of the tool's 23
  counters is `u64` (`main.rs:473-497`), and the two casts present
  (`main.rs:567` `parts.len() as u64`, `cells.rs:311` a value below 26 to `u8`)
  are widening or provably in range. The only `usize` in the module's API is the
  zip entry index, which is the crate's existing convention from part 1. No byte
  offset appears in this module at all. Clause carried.
- **Streaming rather than buffering a sheet.** `Cells::advance`
  (`cells.rs:545-645`) reads one token at a time and returns on the first `<c>`;
  nothing collects a sheet. `read_cell` holds one `Cell`. `skip_subtree`
  (`cells.rs:1017-1044`) walks past an unknown subtree with a depth counter and
  keeps no tokens. `text_of` holds one element's text. The one whole-part cost is
  the shared strings table, which the spec permits and the module's Limits
  section states (`cells.rs:71-85`). Clause carried; the largest corpus sheet
  (1040008 cells in one workbook, `run.log:1863` `max_cells=1040008`) was read on
  both targets.
- **`Cells` borrows the package for the life of the iteration.**
  `Package::cells(&mut self) -> Result<Cells<'_, R>, Error>`
  (`package.rs:622`) ties `'p` to the mutable borrow, and the borrow is real
  rather than decorative: `Cells.stream` holds `Stream<Box<dyn Read + 'p>>`
  (`cells.rs:505`) over the reader `self.archive.read(index)` returned. The
  `PhantomData<&'p mut Package<R>>` (`cells.rs:524`) names whose borrow it is.
  `resolving_a_shared_string_gives_the_item_it_names`
  (`cells/tests.rs:1141-1155`) compiles only because the table is owned and can
  be held across that borrow, which is the property `specs/0010:180-185` gives as
  the reason `shared_strings` returns owned. Clause carried. The report's further
  sentence, "so a second reader cannot be opened on the same package while cells
  are being read", is true of the borrow checker but is asserted by no test and
  cannot be without a compile-fail harness; the report should not present it
  beside the tested claims without saying so.

### 1.7 Things in the diff the spec did not ask for

- `Cell::{number, boolean, error}` (`cells.rs:437-466`) are P3's in both
  `specs/0010:356-360` and the ROADMAP row the doc commit installed. See C10: the
  necessity argument holds for `number` and `boolean` and not for `error`.
- `SharedStrings::is_empty` exists (`cells.rs:122-124`) but is P1's, not this
  phase's.
- Nothing else. The eight files in the diff are the two spec files and ROADMAP
  (planning's own doc commit, `git diff 1f4481d^..1f4481d --stat`), plus
  `cells.rs`, `cells/tests.rs`, `package.rs`, `emend-copy/src/main.rs` and
  `emend-copy/tests/null_rewrite.rs`. `da20c6b` carries the rulings and nothing
  of the feature, which I verified directly:
  `git show da20c6b:crates/emend/src/cells.rs` declares only `SharedStrings`,
  `text_of`, `push_text` and `expand` - no `Cell`, `CellRef`, `Cells`,
  `Formula`, `FormulaKind`, `Kind`, `Package::cells` or `resolve`. Rulings 8 and
  9 are in that commit too (`shared_strings_counts_the_items_and_edits_nothing`
  and `a_second_call_gives_an_equal_table` both present at `da20c6b`), as ruling
  12 requires.
- **C21 (minor, stale).** Ruling 7 (`specs/0010:463-465`) decided that a
  main-namespace `<t>` inside an `extLst` under `<si>` is the item's text.
  `cells/tests.rs:355-357` still says that reading "is raised to planning as a
  question and not decided here". The text is unchanged since `d1ebf1c`, so the
  comment now contradicts a ruling the same phase's doc commit recorded.

---

## 2. The logs against the rules

### 2.1 First lines, and the phase-commit rule

`specs/0001:202-207` (added by THIS phase's first commit, and so binding on this
phase's own logs): "A log's phase commit is given to the script, by argument or
environment, and compared with `git rev-parse HEAD` before the run, with a
control that refuses a fabricated commit; the header prints both values and the
comparison. A header that reads the commit from HEAD and prints it twice asserts
nothing."

| log | given to the script | compared with HEAD | both printed | control | verdict |
| --- | --- | --- | --- | --- | --- |
| `gate.log` | **no** (`gate.log:92` `C=$(git rev-parse HEAD)`; usage line `gate.log:20` takes no argument) | **no** | prints `$C` twice (`gate.log:94`, output line 2) | **none** | **fails the rule** |
| `test.log` | yes (`test.log:54` usage, `$C` from `$1`) | yes (`test.log:120-121`) | yes (lines 2, 10-12) | yes, the null commit (`test.log:125`, output line 12) | passes |
| `negative.log` | yes (`negative.log:115` `C=${1:?phase commit required}`) | yes | yes (lines 2, 6-8) | yes (output line 8) | passes |
| `run.log` | yes (`run.log:193`) | yes, a hard guard (`run.log:200`) | yes (`run.log:696` prints `$C` and a fresh `git rev-parse HEAD`) | **none printed** | partial |
| `scan.log` | yes (`scan.log:36` usage) | yes (output lines 3-5) | yes | **two** (not-HEAD and a short sha, lines 6-7) | passes, best form |

**C4.** `gate.log` does exactly what the new rule names as asserting nothing:
its about line reads
`about: M1 part 3 P2 phase commit... last is <C>; HEAD at run: <C>` with both
halves from the same `$C`. `grep -n "PHASE_COMMIT\|phase commit given"
records/logs/m1p3-p2/gate.log` finds no such check, and `grep -nE '\$\{?1'`
finds no positional argument in the script. So the draft report's sentence - "a
log's phase commit is given to the script and compared with HEAD, both printed,
with a control that refuses a fabricated commit: **every log in this phase does
this**" - is false twice over: `gate.log` does none of it, and `run.log` has no
control. This is the rule this phase's own doc commit added, in the phase whose
job was to carry it out, so I rate it the most serious log finding of the phase.

Everything else about the first lines is in order: line 1 `command: git rev-parse
HEAD`, line 2 the about line with the commit, then the phase line with the UTC
timestamp, in all five logs, the form every previous phase used and the one
`specs/0001` P1R question 3 accepted.

### 2.2 The script printed into each log

Present in all five: `gate.log:14` "## script", `test.log:14` "## script that
wrote this log", `negative.log:10`, `run.log:76`, `scan.log:29`. Each also prints
`scripts/counting.sh` verbatim ("## the shared counting library this log
sources") and runs `counting_controls` with one printed line per form
("## counting controls" at `gate.log:736`, `test.log:800`, `negative.log:1567`,
`run.log:1622`, `scan.log:839`; 24 control lines each, every one as expected).

### 2.3 Compound checks in named functions; counting forms with controls

- `scan.log:1156-1186` runs the three forbidden-shape greps over all four
  sibling logs, the two records and its own source: `chained=0 leading-bang=0
  assignment=0` for each of the 7 files, with six positive controls and six
  cross controls. I re-ran the three patterns myself over the five logs and found
  no hit, and my own search for a two-condition `$([ ... && ... && ... ])` form
  turned up only single-condition `X && echo A || echo B` reporting idioms
  (`gate.log:87,118,320,570`, `scan.log:226,395`, and the counting library's own
  summary lines), which are not compound checks.
- Every compound condition I found is a named function: `status_accounted`
  (`gate.log:563-567`), `structure_sound` (`gate.log:199-202`), `own_un_ok`
  (`run.log:951-953`), `names_no_office_file` (`run.log:1430`), `fresh_dir`,
  `full_sha`/`head_is`/`not_head` (`test.log:120-125`).
- Every ad-hoc counting form has its own printed control section:
  `run.log:1649` (the column guard), `1654` (the entry classifier), `1667` (the
  cells-line reader, `comp_counts` included), `1690` (the independent reader);
  `test.log:1606-1609` (the per-binary counter: a control row and a
  no-such-binary control); `gate.log` `ctl` lines for `f_nonascii`, `f_ctrl`,
  `f_live`, `f_pre`, `f_office` with a hit and a clean fixture each;
  `negative.log` 37 control lines. This clause is met.

### 2.4 The scan log

Judged as the first run I was given:

- **Written last.** `test.log` 19:14:20, `negative.log` 19:18:16, `run.log`
  19:39:53-19:47:31, `gate.log` 19:48:12-19:49:40, `scan.log`
  19:50:03-19:50:05. Yes.
- **Hashes every committed sibling.** `scan.log:1063-1132` prints
  `wc -c` and `sha256sum` for `gate.log`, `test.log`, `negative.log`, `run.log`
  and both records, nine forms each, all zero. I recomputed all six hashes at my
  own run time and every one matches byte for byte
  (`de43c52...`, `10cd737...`, `f376dad...`, `62e2230...`, `d5cb7f1...`,
  `1ca6d1f...`). `check answers scanned: 0` (`scan.log:1110`), correct for a first run; the final
  run must add this answer and the second checker's, which is the one thing the
  version I hold cannot show.
- **Scans its own source with the pre-P6d forms before printing itself, with a
  control.** `scan.log:16-27`: `pre-P6d forms over this script's source:
  count=0`, then `the same form over a copy with one concatenated shape appended:
  count=1`. Yes, and the control is the right shape.
- **The three greps over every printed script including its own source.**
  `scan.log:1156-1186`, 7 files, as above. Yes.
- The pre-P6d bare-shape control also covers all 8 diff files plus the 4 sibling
  logs and 2 records (`scan.log:1134-1155`, 14 files, 0 hits, `scan.log:1153-1155`).

### 2.5 sha256 values

Every hash any of the five logs cites is printed by a command over a file that
existed at run time, and the commands are visible in the printed scripts:

- superseded attempts: `gate.log:5-7` (two attempts, 58764 and 59231 bytes),
  `test.log:5-6` (one attempt, 129734 bytes), each under
  `command: sha256sum and wc -c over each superseded attempt named above`, with
  the script's fallback that prints `NO HASH EXISTS for ...` when a file is gone
  (`gate.log:108-111`) - not triggered, so both files existed;
- the eight diff files at the phase commit (`gate.log:797-804`);
- the four gate binaries (`gate.log:1021-1024`), re-hashed independently by
  `negative.log:1598-1601` after copying them into a scratch clone and by
  `run.log:1710-1715`. All three logs print the same four values
  (`23dcaa32...`, `a718bb29...`, `1a753f78...`, `3830dc06...`), which is what
  supports the report's claim that one set of hashes stands for all three logs;
- the record pair (`run.log:2167-2172`), matching my own `sha256sum`;
- two test files read for message coverage (`negative.log:2265,2267`); the
  `cells/tests.rs` hash there equals `gate.log:799`'s.

No typed hash anywhere. Clause met.

### 2.6 Bite claims

`specs/0001:190-196`. `test.log:1865-2108` plants seven mutations in a detached
worktree at the phase commit, one per copy, prints the applier verbatim
(`test.log:1870-1947`), prints each run's exit status, the failing test names and
the real `left`/`right` values, and runs the unmutated worktree as the control.
Both mutation runs and the control use `--no-fail-fast`, which the header's
rerun note explains was the fault in attempt 1 (`test.log:4`). Every bite claim I
could find in the diff's comments maps to a mutation: the line-end comments
(M1), the CDATA comments (M2, M4), "a reader that refused only `Start` would let
`<br/>` through" (M3), the empty-table-versus-refusal pair (M5), and the two
expectation comments (M6, M7). The commit message of `da20c6b` asserts the same
five plantings and is now backed by `test.log`. I found no bite claim left
unmeasured.

### 2.7 Four logs, no self-test.log, no size.log

- **No `self-test.log`: consistent, and I checked the diff myself.** The phase
  diff is ROADMAP.md, `crates/emend/src/cells.rs`, `crates/emend/src/cells/
  tests.rs`, `crates/emend/src/package.rs`, `specs/0001-bootstrap.md`,
  `specs/0010-reading-cells.md`, `tools/emend-copy/src/main.rs`,
  `tools/emend-copy/tests/null_rewrite.rs`. None of these is one of the ten gate
  files `specs/0001:113-118` names (`scripts/check.sh`, `scripts/privacy.sh`,
  `scripts/privacy-patterns.txt`, `scripts/privacy-identity.txt`,
  `scripts/sys-allowlist.txt`, `scripts/install-hooks.sh`,
  `scripts/counting.sh`, `scripts/gate-self-test.sh`, `.gitignore`, the CI
  workflow). So `specs/0001:119-121`'s other duty applies, and `gate.log:806-814`
  discharges it exactly: it lists all ten names, scores them against the diff
  (`gate files checked: 10; of them in the phase diff: 0`), asserts zero with a
  control that the same `grep -Fxq` form DOES find a name the diff holds
  (`ROADMAP.md`), and asserts that no `self-test.log` is present in the log
  directory. `scan.log:1104-1109` repeats the measurement independently. The
  gate log's assertion is correct.
- **No `size.log`: consistent.** `specs/0010:280-292`, "Phases", lists
  `size.log` "in P3", and `specs/0010:355-372` puts `probe-emend-cells-xml`, the
  float-parser variant and `records/size-m1p3-...md` in P3. So P2 owes none.
  See C15 on the report's wording.
- **C16 (minor, a miscount as written).** The report says "Four, in
  `records/logs/m1p3-p2/`: `gate.log`, `test.log`, `negative.log`, `run.log`,
  and `scan.log` written last" - a sentence that says four and names five. The
  directory holds five logs; "four" is right only for the SIBLINGS scan.log
  hashes, which is how `scan.log:1104` counts them ("sibling logs scanned: 4
  (expected 4; scan.log cannot scan itself)").

### 2.8 Finding P2-F1: measurement or story

**Measurement.** The figures printed support the claim, the arithmetic holds, and
I can find nothing in the reader that would produce this departure.

What `run.log` prints:

- The two departures are named BEFORE the run, in the header that lists the
  figures the run is judged against (`run.log:66-72`), rather than discovered
  and then explained.
- Both targets: `values=2089191`, `empty=5410`, `cells=2094601`
  (`run.log:2132-2133`), the two target lines identical file to file
  (`run.log:2135`).
- `values + empty = 2094601; cells = 2094601`, asserted
  (`run.log:1921-1922` host, `run.log:2110-2111` i686).
- The composition from the independent expat reader:
  `t_n=1302755 t_absent=8233 values_text=2089124 values_empty=67
  neither_with_children=0` (`run.log:2145`), with `comp_counts` first shown
  moving every counter on a built fixture (`run.log:1686-1688`).
- `t_n` and `t_absent` are the spec's exactly and sum to `number`
  (`run.log:2148-2158`). `values_empty` is the spec's 67 exactly
  (`run.log:2154-2155`).
  `values_text` is 2089124 against the spec's 2089071, "the difference is 53"
  (`run.log:2152`), and `values_text + values_empty = 2089191 = values`
  (`run.log:2159-2160`).
- `cells holding neither a <v> nor an <f> while holding another child: 0`
  (`run.log:2162-2164`).

The arithmetic: 2089191 - 2089138 = 53 and 5463 - 5410 = 53, in opposite
directions; both partitions sum to 2094601, which all three routes agree on. The
spec's own figures are internally consistent only if 53 cells hold children but
neither `<v>` nor `<f>` (`specs/0010:272-273`), and the independent reader
measures zero such cells while measuring 53 MORE cells with `<v>` text than the
spec claims. Since `cells` is fixed and agreed, and `empty` is measured with
`neither_with_children=0`, the 53 must sit in `values`, and `values_text` is
where they are. That inference is tight, and the report states it correctly.

Nothing in the reader looks wrong in a way that would produce it. I checked the
three constructs that could move a cell between the two buckets:
`<v/>` and `<v></v>` both give `Some("")` (`cells.rs:665-671`, `text_of`
returning on the first `End`), and the independent reader gives `""` for both
too (`value = child.text or ""`), so 53 such cells would have shown as
`values_empty=120`, not 67. A `<v>` inside a skipped subtree sets nothing
(`cells.rs:689-691`, `skip_subtree`), and the independent reader iterates the
cell's direct children only, so neither route could invent a value. A cell with a
formula and no value would fall into neither bucket, and the exact sum to `cells`
shows there are none. The one explanation I can offer for planning's original
count coming out 53 short - offered as a hypothesis for planning to test, not as
a finding - is that a regex over `<v>` text would miss a `<v>` whose text begins
with a character reference (`<v>&#...`), which both readers expand to text;
`specs/0010:57-58` counts character references in formula texts but says nothing
about `<v>` texts. `run.log` does not attempt that identification, and is right
not to: it measures what the corpus holds and leaves which side is wrong to
planning, as `specs/0010:277-278` and `specs/0001` require. The reader was not
changed and the run was not repeated (`run.log:70-72`).

**C9 (an inference presented as a measurement, in `run.log` and repeated in the
report).** `run.log:1926-1927` prints `number - numbers_parsed = 5410; empty =
5410` and concludes "host every Number cell holding a value parsed as a number:
as-expected". The identity does not establish that. `number - numbers_parsed`
equals (Number cells with no value) + (Number cells whose value `str::parse::
<f64>` refuses), and `empty` is (cells of ANY kind with neither value nor
formula). The equation holds if, say, 19 of the empty cells were `str` cells and
19 Number cells failed to parse. Nothing printed gives `empty` broken down by
kind, and the spec's own type table (`specs/0010:56-59`) allows up to 19 `str`
cells with no `<f>` whose `<v>` status is unstated. The report repeats the
conclusion as fact: "1310988 minus 1305578 is exactly the 5410 `Number` cells
that hold no value at all, and every one of the remaining 1305578 parses." What
is measured is the identity; the decomposition is an inference. A per-kind
`empty` count would close it in one line.

---

## 3. The report's numbers and quoted strings

Every figure below I traced to a primary and checked. Unless a row says
otherwise, it is correct.

| report claim | primary | verdict |
| --- | --- | --- |
| commits `1f4481d`, `da20c6b`, `6c27167` and their subjects | `git log --reverse 1f4481d^..HEAD`; `gate.log:773-780` | correct |
| "eight files, 2383 insertions and 107 deletions" | `git diff --stat 1f4481d^..HEAD`: 8 files, 2383 insertions, 107 deletions. 8 and 2383 also in a log: `gate.log:805` "files in the diff: 8" and the per-file added-line counts (2 + 987 + 860 + 71 + 24 + 110 + 177 + 152 = 2383). **107 appears in no log**; I verified it with git | correct, one number log-less |
| the eight file names | `gate.log:797-804` | correct |
| "No gate file is in the diff, which `gate.log` asserts against the diff by name" | `gate.log:806-814` | correct (I checked the diff myself) |
| `PhantomData<&'p mut Package<R>>` | `cells.rs:524` | correct |
| "a cell has a type this layer does not know" | `cells.rs:357`; raised through the binary at `negative.log:2203` | correct quote |
| "Nine messages were added to `package.rs`'s catalogue" | `git diff da20c6b..6c27167 -- package.rs` adds exactly 9 catalogue bullets | correct of the feature commit; **ten across the phase** (`da20c6b` added "a text element holds an element"). See C17 |
| `the_catalogue_and_the_source_hold_the_same_messages` still passes | `test.log:1063` (host), `test.log:1428` (i686), `test.log:1715` | correct |
| M6 `left Some(CellRef { row: 1, col: 2147483647 })` / `right ... 4294967295` | `test.log:2078-2079` | correct (punctuation aside) |
| M7 `left ["B7", "C1"]` / `right ["B7", "C7"]` | `test.log:2094-2095` | correct |
| `FXSHRXW` is `i32::MAX`; `MWLQKWU` = 4294967295 | `test.log:2078-2079` prints both values; I recomputed both by hand | correct |
| "333 tests ... the same 333 names on i686 ... against 280 in P1's log" | `test.log:1603`, `1611`, `2111-2113` | correct; my own run summed 333 |
| "The library's own tests are 249" | `test.log:1597` (per-binary list) and `test.log:1610` | correct |
| "the `cells` module runs 71 tests where P1's log printed 20" | `test.log:1799-1801`; `records/logs/m1p3-p1/test.log:1413` | correct |
| "The tool's integration binary holds 14 where P1's log printed 12", both new names | `test.log:1821-1824`; `m1p3-p1/test.log:1432`; both names in the list at `test.log:1807-1820` | correct |
| "Every previous phase's figure ... was read from that phase's committed log" | `test.log:1611-1612`, `1800-1801`, `1822-1823`, each with a check line under it | correct |
| "23 tab-separated `key=value` counts", spec order | `main.rs:502-541`; `negative.log:2171` "fields after the word cells: 23; of them key=number pairs: 23" | correct |
| "458 seconds wall" | `run.log:2179` | correct |
| "All 870 entries `identical` and unflagged on each target" | `run.log:1796` and `1985` | correct |
| `files_identical`/`order_preserved`/`lo`/`lo_in` 27 of 27 on both targets | `run.log:1787-1794`, `1976-1983` | correct |
| the 18 agreeing key totals | `run.log:1869-1911` (host) and `2058-2117` (i686), and the record table `records/m1p3-p2-2026-09-18-6c27167.md:24-46` | every figure correct |
| `max_row` 130001, `max_col` 28, `numbers_parsed` 1305578 | `run.log:2132` | correct |
| "Twenty-one of the twenty-three agree exactly" | the logs assert 18 keys against a spec figure per target (`grep -c "is the spec's"` = 18), `numbers_parsed` against a derived expectation, and nothing for `max_row`/`max_col` | **imprecise; see C8** |
| P2-F1: 2089191/2089138, 5410/5463, 53, sum 2094601 | `run.log:66-72`, `2132`, `2159-2164` | correct |
| `values_text` 2089124 vs 2089071, `values_empty` 67 | `run.log:2145`, `2152`, `2154` | correct |
| "ZERO cells that hold neither while holding any other child element" | `run.log:2162-2164` | correct |
| the quoted spec parenthetical "the 5410 empty elements and 53 cells with neither `<v>` nor `<f>`" | `specs/0010:272-273` | exact quote |
| `t_n` 1302755, `t_absent` 8233 | `run.log:2148-2151` | correct |
| "1310988 minus 1305578 is exactly the 5410 Number cells that hold no value" | `run.log:1926-1928` prints the identity | arithmetic correct, decomposition inferred; see C9 |
| "the record pair is `records/m1p3-p2-2026-09-18-6c27167.md` and `.txt`" | both exist; hashes at `run.log:2168-2169` match mine | correct |
| "Three logs were rerun" | `test.log:4` names 1 superseded attempt; `gate.log:4` names 2; `negative.log`, `run.log`, `scan.log` headers name none | **wrong; see C5** |
| gate.log's two faults, described | `gate.log:4` | correct paraphrase |
| "four fixtures ... whose answers were written into the script before either route saw them" | `run.log:1721-1722` says the expected line was pre-written for the FIRST TWO | **overstated; see C7** |
| "One holds a `<c>` inside an `extLst` and a `<row>` inside a `conditionalFormatting`: four `<c>` elements, two cells. Both routes said two." | `run.log:925-935` (the fixture, 4 `<c>`), `1701`, `1727-1729`, `1742` | correct |
| "Each workbook's WHOLE line, all 23 keys at once, was compared with ... python3's expat parser" | `run.log:1862` (host), `run.log:2051` (i686); the reader uses `xml.etree.ElementTree` (`run.log:479`), which is expat | correct |
| the four binaries' hashes stand for all three logs | `gate.log:1021-1024`, `negative.log:1598-1603`, `run.log:1710-1716`, identical values | correct |
| "No `self-test.log`, because no gate file changed" | `gate.log:806-814`, and the diff | correct |
| "No `size.log`: `specs/0010`, 'Size', puts part 3's size record after P3" | `specs/0010:284-286` puts `size.log` **in** P3 | **wrong preposition; see C15** |
| "`Cell::{number, boolean, error}` ... They had to be" / "exercised ... through the cell tests that reach them" | `grep -rn '\.error()\|\.number()\|\.boolean()'`: only `main.rs:602` and `main.rs:605`. No test in `cells/tests.rs` calls any of the three; `error()` is called by nothing at all | **wrong; see C10** |
| "Nothing has been pushed" | `git log --oneline origin/main..HEAD` shows 12 local commits over `f4f48e8`, including `9bd9102` and `7702f6e` | correct |

Commit-message hygiene, which the report does not claim but `CLAUDE.md` requires:
all three messages are ASCII (`grep -cP '[^\x00-\x7F]'` = 0) and end with the
Co-Authored-By line alone, with no session line.

---

## 4. Findings, in the order I would fix them

| # | severity | finding |
| --- | --- | --- |
| C4 | high | `gate.log` does not carry the phase-commit rule this phase's own doc commit added to `specs/0001:202-207`: the commit is not given to the script, not compared with HEAD, and has no control; the header prints one `$C` twice, the exact form the rule says asserts nothing. `run.log` gives and compares but prints no control. The report's "every log in this phase does this" is false. |
| C3 | high | Ruling 1 (`specs/0010:434`), ruling 12 (`:486`) and the ROADMAP P2 row require P2's corpus run to assert `bytes` 1497091 by name. No log asserts it; no `--shared-strings` corpus pass was made this phase; the only committed corpus `bytes` figure is P1's 1497107, now stale; `cells.rs:43-44` states 1497091 as fact. Not listed in the report's "What is not done". |
| C1 | material | `specs/0010:85-88` requires `lib.rs` to re-export the six P2 types; `lib.rs:35` re-exports `SharedStrings` alone and is not in the diff. Unmentioned anywhere. |
| C2 | material | P1 ruling 6 (`specs/0010:459-461`) requires the catalogue to list both tests beside "a cell holds a reference this layer cannot expand"; `package.rs:79-80` lists one. Same omission for "a text element holds an element" (`package.rs:81-82`). The set-equality test cannot catch it. |
| C11 | material | `cells/tests.rs:950-952` claims "the skipped element moved no count, which is visible because the third cell names itself" - the third cell's explicit `r="C1"` is precisely what makes that invisible, and the assertion under the comment checks a value. The recorded defect class, in a comment. Drop the `r`. |
| C9 | material | `run.log:1926-1928` and the report present "every Number cell holding a value parsed" as measured; the printed identity `number - numbers_parsed = empty` does not separate empty cells of another kind from Number cells that fail to parse. No per-kind `empty` figure exists. |
| C10 | material | `Cell::error` is called by no test and no tool path, so "They had to be" is true of `number` and `boolean` only; and no test in `cells/tests.rs` reaches any of the three, so "exercised ... through the cell tests that reach them" is false. |
| C13 | material | `records/m1p3-p2-2026-09-18-6c27167.txt` is malformed: every per-workbook row is split across two lines after the `sheets` column (3 fields then 23; `awk -F'\t' '{print NF}'` gives 3, 23, 3, 23 ...). P1's record is 3 fields per row and well formed, so this is new. The values themselves are consistent (on the rows I checked, `number + shared = cells` and `values + empty = cells`), but the table cannot be read a row per line. |
| C5 | minor | "Three logs were rerun" - two logs were, over three superseded attempts (`test.log` 1, `gate.log` 2). |
| C6 | minor | `run.log`'s printed script makes first-person historical claims that belong to P1's run.log, not this one: "The first attempt of this log wrote `isnot LC_ALL=C ...`" (`run.log:1423-1429`), "The second attempt of this log did exactly that" (`run.log:209`), "In every run of this log HEAD did equal the argument" (`run.log:196-199`). P2's run.log has no rerun header and no superseded attempt; the eight attempts those sentences describe are listed in `records/logs/m1p3-p1/run.log:6`. This is the same class as `gate.log` attempt 1's own recorded fault (a header carrying the previous phase's label). |
| C7 | minor | "four fixtures ... whose answers were written into the script before either route saw them" overstates `run.log:1721-1722`, which pre-writes the expected line for the first two; `own-unresolved` is asserted by two keys and `own-noffice` by `sheets=0` plus the refusal. |
| C8 | minor | "Twenty-one of the twenty-three agree exactly" (report and `records/...md:18-20`). The honest decomposition is 18 asserted against a spec figure, 1 (`numbers_parsed`) against a derived expectation, 2 (`max_row`, `max_col`) unconstrained, 2 departing. |
| C12 | minor | `cells/tests.rs:963` reaches the strict namespace as `MAIN[1]`, a position; reorder `MAIN` and the clause is silently untested. |
| C14 | minor | `records/...md:17-18` attributes the judged totals to "Facts planning checked"; the per-key totals it tabulates are `specs/0010:262-278`, "Harness runs". |
| C15 | minor | "puts part 3's size record after P3" (report, and `scan.log`'s sibling note) - `specs/0010:284-286` puts `size.log` IN P3. |
| C16 | minor | "Four, in `records/logs/m1p3-p2/`" then five files named. The directory holds five logs; four is the sibling count `scan.log:1104` asserts. |
| C17 | minor | "Nine messages were added to `package.rs`'s catalogue" is true of `6c27167` and off by one for the phase (ten). |
| C18 | minor | `specs/0010:249-250`, "streams every sheet of the workbook in `<sheets>` order", is proven nowhere: every fixture has one sheet and no corpus assertion is order-sensitive. |
| C19 | minor, latent | A `SharedString` cell with no value: the library answers `Ok(None)` and the tool counts it `resolved`, while `run.log`'s independent reader counts it `unresolved`. No fixture and no corpus witness, so the two-route agreement does not cover it; no test either. |
| C20 | trivial | `Package::cells` calls `archive.read` where `specs/0010:185-188` words it "through `read_part`". Identical behaviour. |
| C21 | trivial | `cells/tests.rs:355-357` still says the `extLst`/`<t>` question is "raised to planning ... and not decided here"; ruling 7 decided it in this phase's own doc commit. |

## 5. What I checked and found nothing wrong with

Said explicitly, because silence is not a verdict:

- The 48 spec test cases are all covered, by name, and I found no case with no
  test.
- No assertion in the 42 new tests pins a position or a mechanism. The one
  problem in this class is a comment (C11) and one is a fixture-construction
  nit (C12).
- The seven mutations, the control, the `--no-fail-fast` fix and the kept and
  hashed attempt 1 satisfy the new bite rule in full.
- Every `u64`/`u32` choice matches `specs/0010:88-90` and "32-bit and
  streaming"; nothing in the module could wrap on i686; nothing buffers a
  sheet; the package borrow is real and is exercised by a test that would not
  compile without it.
- The gate ran on a cold cache from `/` by absolute path with nine PASS lines
  and empty stderr (`gate.log:994-1005`), and I reproduced `cargo test
  --workspace` (333 passed, 2 ignored), `cargo fmt --check` and `cargo clippy
  --all-targets -D warnings` clean at HEAD.
- The corpus acceptance is as claimed on both targets: 870 entries identical,
  27/27 on `files_identical`, `order_preserved`, `lo` and `lo_in`, the two
  targets' lines identical file to file, and the per-workbook whole-line
  agreement with an independent expat reader on all 27, on both targets.
- Every sha256 in every log is printed by a command over a file that existed at
  run time; the six I could recompute match exactly.
- `da20c6b` carries the rulings and nothing of the feature, as ruling 12
  requires; `6c27167` carries the feature.
- The three commit messages are ASCII and end with the Co-Authored-By line
  alone; nothing is pushed.
- The absence of `self-test.log` and `size.log` is correct, and the gate log's
  assertion that no gate file is in the diff is correct against the diff I
  checked myself.
- `run.log`'s account of finding P2-F1 is a measurement. The reader was not
  changed to match and the run was not repeated until it agreed.
