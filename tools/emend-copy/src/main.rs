//! emend-copy: opens a zip archive with the emend layer and writes it back.
//!
//! `specs/0008-zip-layer.md`, "Harness runs", and `specs/0009-package-
//! layer.md`, "Harness runs". At most ONE operation per invocation, given
//! after the two paths:
//!
//! - none: no edits, so the output must be byte-identical to the input and the
//!   harness must report every entry `identical` with an empty flags column.
//! - `--replace <name>`: the named entry is replaced by its OWN decompressed
//!   bytes, so the harness must report that entry `recompressed` (or
//!   `identical`) and every other entry `identical` and unflagged.
//! - `--xml-null-rewrite`: every entry whose name ends in `.xml` or `.rels`,
//!   ASCII case-insensitively, is replaced by its own bytes passed through
//!   `emend::xml::Stream`, the mark when present followed by every token's raw
//!   bytes. Each such entry must come back `recompressed` or `identical` and
//!   never `changed`, and every other entry `identical` and unflagged: the
//!   rewrite is the identity, so this is what proves the token stream loses
//!   nothing across the corpus.
//! - `--drop-calc-chain`: the recipe of `specs/0009-package-layer.md`,
//!   "Module `package`", recorded by `Package::drop_calc_chain`: the
//!   workbook's `calcChain` relationship, the `Override` for its target when
//!   the package has one, and the entry when the package has one, each its own
//!   removal and nothing implicit. A VALID package with no chain records no
//!   edit, so its output must be byte-identical to its input; a package with
//!   one must come back with those three gone, the two rewritten parts
//!   `changed` and every other entry `identical` and unflagged. The content
//!   types part is read before the chain is looked for, so a package without
//!   one is refused here as `--survey` refuses it, whether or not its edit
//!   would be empty (part 2 P3 verification ruling 6).
//! - `--survey`: no edit either, so the output must be byte-identical again,
//!   and one line goes to stdout: the word `survey` then tab-separated
//!   `key=value` pairs, COUNTS ONLY. The package layer's whole map is walked
//!   to produce them, so a corpus file whose content types, relationships,
//!   targets or sheets the layer cannot read is an error (exit 2) and the
//!   runner records the run as one. The line names no entry and no value from
//!   the workbook: a count is not content.
//! - `--shared-strings`: no edit either, so the output must be
//!   byte-identical again, and one line goes to stdout: the word `strings`
//!   then tab-separated `key=value`, `items` (the table's length) and
//!   `bytes` (the sum of the items' UTF-8 lengths). Two counts and nothing
//!   else: a shared string IS content, so no item, no length of one and no
//!   fragment of one is printed. A workbook the layer cannot read the table
//!   of is an error (exit 2), as `--survey` is.
//! - `--cells`: no edit either, so the output must be byte-identical again,
//!   and one line goes to stdout: the word `cells` then tab-separated
//!   `key=value`, the keys `specs/0010-reading-cells.md`, "Harness runs",
//!   lists. Every sheet of the workbook is streamed in `<sheets>` order.
//!   COUNTS ONLY, for the same reason `--shared-strings` prints two: a
//!   cell's value IS content, so no value, no formula and no shared string
//!   is printed, nor the length of one. A failed `resolve` is counted as
//!   `unresolved` and the run carries on, so that count is the whole
//!   workbook's; an error from any other parse is exit 2 with the message.
//! - `--read-sample`: no edit either, so the output must be byte-identical
//!   again (`specs/0027-read-back.md`, "Harness runs"), and one line goes to
//!   stdout: the word `read` then tab-separated `key=value`, COUNTS ONLY.
//!   Every cell of every sheet of kind `Worksheet` is read and each cell's
//!   `s` resolved with `cell_style`; the columns, rows and settings of W,
//!   the first such sheet, are read, and the shared strings table walked
//!   item by item. The keys: `cells`, `styled` (cells with an `s`),
//!   `resolved` (styles resolved, equal to `styled`, asserted), then over
//!   the distinct cell formats the cells use: `fonts`, `fills` and `borders`
//!   (the distinct fonts, `Some` fills and borders among them), `gradients`
//!   (those whose fill reads as `None`) and `codes` (those whose number
//!   format reads as a code); then `columns` (W's `<col>`), `rows` (W's
//!   rows), `merges`, `links`, `frozen` and `protected` (1 when W has a
//!   frozen pane, a `<sheetProtection>`), `items` (the table's items walked,
//!   equal to the held table's length, asserted) and `sst_items_equal` (the
//!   items whose text equals the held table's at the same index, equal to
//!   `items`, asserted). A refusal by the layer, or an assertion that does
//!   not hold, is exit 2 with no output.
//! - `--items`: no edit either, so the output must be byte-identical again,
//!   and one line goes to stdout: the word `items` then `items` (the
//!   shared strings table's items, walked one at a time without the table
//!   held, `specs/0027-read-back.md`, decision 6). A count and nothing else.
//! - `--edit-sample`: the sample cell edit of `specs/0011-editing-cells.md`,
//!   "Harness runs", on the first sheet of kind `Worksheet`: its first
//!   `Number` or `SharedString` cell with a value and no formula set to
//!   42.5, the second such cell cleared, and a text and a boolean added in a
//!   new row after the last. The output is reopened and the sheet read again,
//!   and one line goes to stdout: the word `edit` then tab-separated
//!   `key=value`, COUNTS ONLY, for the reason `--cells` gives. A cell count
//!   after the write other than the one the edits imply is exit 2.
//! - `--edit-formula-sample`: the formula sample edit of the same section:
//!   the sheets of kind `Worksheet` are read in `<sheets>` order until a cell
//!   with a formula of kind `Normal` is found, that cell is set to
//!   `Number(1)` and the package written, so the write's rule drops the
//!   calculation chain when there is one. One line goes to stdout: the word
//!   `edit` then tab-separated `formula_replaced`, `chain_dropped`,
//!   `cells_after` and `read_back`, COUNTS ONLY. A cell count after the
//!   write other than the one before is exit 2. A workbook with no normal
//!   formula gets no edit, so its output is byte-identical, and every key of
//!   its line is 0. From part 6 the same write also asks for a full
//!   calculation on load, so the workbook part's `calcPr` changes too
//!   (`specs/0013-formulas.md`, P1 ruling 4); the line is unchanged.
//! - `--styles`: no edit, so the output must be byte-identical again, and one
//!   line goes to stdout: the word `styles` then tab-separated `key=value`,
//!   `xfs`, `num_fmts`, `custom`, `dated` and `date1904`, the keys
//!   `specs/0012-styles.md`, "Harness runs", lists. COUNTS ONLY: a format
//!   code is content, so no code, no font name and no length of either is
//!   printed. A workbook whose styles part the layer cannot read is exit 2
//!   with the message.
//! - `--calc`: no edit, so the output must be byte-identical again, and one
//!   line goes to stdout: the word `calc` then tab-separated `key=value`,
//!   `chain` (1 when the workbook has a `calcChain` relationship, else 0) and
//!   `full_calc` (1 when its `calcPr` asks for a full calculation on load,
//!   else 0), the keys `specs/0013-formulas.md`, "Harness runs", lists. Two
//!   flags and nothing else. A workbook the layer cannot read is exit 2 with
//!   the message.
//! - `--formula-sample`: the formula sample of `specs/0013-formulas.md`,
//!   "Harness runs", on the first sheet of kind `Worksheet`: A, its first
//!   `Number` cell with a value and no formula, is set to `Number(21)`; a new
//!   row after the last takes the formula of A's reference followed by `*2`
//!   in column 1 and `1+1` in column 2; and F, its first cell with a normal
//!   formula, when there is one, takes `2+3`. The write's rule drops the
//!   calculation chain and asks for a full calculation on load. One line goes
//!   to stdout: the word `formula` then tab-separated `added`, `replaced`,
//!   `chain_before`, `chain_after`, `full_calc_before`, `full_calc_after`,
//!   `cells_before`, `cells_after` and `read_back`. COUNTS ONLY: the formula
//!   texts are this tool's own constants and A's reference, and none reaches
//!   the line or a message. `cells_after` and `read_back` are asserted
//!   against what the edits imply; either failing is exit 2 with the message
//!   and no output. With no A nothing is recorded, so the output is
//!   byte-identical and `added` and `replaced` are 0.
//! - `--style-sample`: the sample style edit of `specs/0012-styles.md`,
//!   "Harness runs": two cell formats are added, one over the first sampled
//!   cell's own format with the code `0.000` and one over no base with the
//!   built-in id 14; the sampled cell takes the first and a new cell one row
//!   past the last takes `Number(45000.5)` with the second. One line goes to
//!   stdout: the word `edit` then `styled`, `added`, `xfs_before`,
//!   `xfs_after`, `formats_added`, `cells_before`, `cells_after` and
//!   `read_back`. COUNTS ONLY, as `--styles` is: the format code is this
//!   tool's own constant and no cell's text, reference or format code is
//!   printed. `xfs_after` and `cells_after` are asserted against what the
//!   edits imply; either failing is exit 2 with the message and no output,
//!   as `--edit-sample`'s count assertion is.
//! - `--date-sample`: the sample dated cell of `specs/0012-styles.md`,
//!   "Harness runs", `--style-sample`'s new cell alone: one cell format is
//!   added over no base with the built-in id 22, and a new cell one row past
//!   the last takes the serial of 2024-02-29 12:00:00.000 in the workbook's
//!   own epoch with that format. One line goes to stdout: the word `edit`
//!   then `date1904`, `serial`, `added`, `xfs_before`, `xfs_after`,
//!   `cells_before`, `cells_after` and `read_back` ("Planning rulings during
//!   P3", ruling 5). The serial is this tool's own constant written, 45351.5
//!   or 43889.5, never a number read from the workbook. `xfs_after` and
//!   `cells_after` are asserted as `--style-sample`'s are. The tool enables
//!   the crate's `dates` feature for this operation.
//! - `--font-sample`: the sample font and fill of
//!   `specs/0017-fuller-styles.md`, "Harness runs": two cell formats are
//!   added through `add_style`, one over the first sampled cell's own format
//!   with the tool's font F1 and fill L1, and one over no base with F1 alone,
//!   so the part gains one font and one fill; the sampled cell takes the
//!   first and a new cell one row past the last takes `Number(7)` with the
//!   second. One line goes to stdout: the word `edit` then `styled`,
//!   `added`, `xfs_before`, `xfs_after`, `fonts_added`, `fills_added`,
//!   `cells_before`, `cells_after` and `read_back`. COUNTS ONLY: the font's
//!   name and the colours are this tool's own constants, and no cell's text
//!   or reference is printed. `xfs_after` and `cells_after` are asserted as
//!   `--style-sample`'s are. A styles part without `fonts` or `fills` is
//!   refused by the layer with no output.
//! - `--border-sample`: the sample border, alignment and protection of
//!   `specs/0017-fuller-styles.md`, "Harness runs": two cell formats are
//!   added through `add_style`, one over the first sampled cell's own format
//!   with the tool's border B1, alignment G1 and an unlocked, unhidden
//!   protection, and one over no base with B1 and a right alignment, so the
//!   part gains one border; the cells are `--font-sample`'s. One line goes to
//!   stdout: the word `edit` then `styled`, `added`, `xfs_before`,
//!   `xfs_after`, `borders_added`, `cells_before`, `cells_after` and
//!   `read_back`, counts only. `xfs_after` and `cells_after` are asserted as
//!   `--style-sample`'s are. A styles part without `borders` is refused by
//!   the layer with no output.
//! - `--format-sample`: the sample formats, pane and merge of
//!   `specs/0018-rows-and-columns.md`, "Harness runs", with P1 ruling 1: on
//!   the first sheet of kind `Worksheet`, column 2 takes width 20.5, column
//!   3 is hidden, row 2 takes height 30, row 3 is hidden, the panes are
//!   frozen at B2, and the two cells of the row past the last row with a
//!   cell, columns 1 and 2, are merged, so no cell is cleared. One line goes
//!   to stdout: the word `format` then `cells_before`, `cells_after`,
//!   `rows_before`, `rows_after`, `merges_before` and `merges_after`. COUNTS
//!   ONLY, and every index is this tool's own constant or M. The cells come
//!   from the layer's reader; the `<row>` elements of the first main-
//!   namespace `sheetData` and the `<mergeCell>` elements of `mergeCells`
//!   are counted by this tool's own pass over the part's tokens, as
//!   `--sheet-sample` counts the workbook's. Asserted: `cells_after` equal
//!   to `cells_before`, `rows_after` equal to `rows_before` plus rows 2 and 3
//!   where they were absent, `merges_after` one more; a failure is exit 2
//!   with the message and no output, and so is a refusal by the layer.
//! - `--insert-rows-sample` and `--delete-rows-sample`: the row samples of
//!   `specs/0018-rows-and-columns.md`, "Harness runs", with P2 ruling 6. On
//!   the first sheet of kind `Worksheet`, k is `max(2, T + 1)`, T the
//!   largest last row of the table and pivot table ranges its relationships
//!   name, read by this tool's own pass (0 when there are none); two rows
//!   are inserted at k, or row k is deleted. One line goes to stdout: the
//!   word `insert_rows` then `at`, `count`, `cells_before`, `cells_after`,
//!   `from_k_before`, `from_k2_after`, `formulas_moved`, `chain_before`,
//!   `chain_after` and `full_calc`; or `delete_rows` then `at`, `count`,
//!   `cells_before`, `cells_after`, `from_k1_before`, `from_k_after`,
//!   `formulas_moved`, `chain_before`, `chain_after` and `full_calc`. COUNTS
//!   ONLY. Asserted: an insertion keeps every cell and moves those from k
//!   on to k + 2 on; a deletion loses row k's cells and moves those after it
//!   up by one. A failure is exit 2 with the message and no output, and so is
//!   the layer's refusal of a workbook the rule refuses.
//! - `--insert-columns-sample` and `--delete-columns-sample`: the column
//!   samples of `specs/0018-rows-and-columns.md`, "Harness runs", with P3
//!   ruling 3, the row samples over columns. c is `max(2, U + 1)`, U the
//!   largest last column of the same ranges; one column is inserted at c, or
//!   column c is deleted. One line goes to stdout: the word `insert_columns`
//!   then `at`, `count`, `cells_before`, `cells_after`, `from_c_before`,
//!   `from_c1_after`, `formulas_moved`, `chain_before`, `chain_after` and
//!   `full_calc`; or `delete_columns` then `at`, `count`, `cells_before`,
//!   `cells_after`, `from_c1_before`, `from_c_after`, `formulas_moved`,
//!   `chain_before`, `chain_after` and `full_calc`. COUNTS ONLY. Asserted as
//!   the row samples assert, over columns. The tool turns on the layer's
//!   `rowcol` feature for the four shift samples.
//!
//! - `--new-sample`: the new workbook sample of
//!   `specs/0016-new-workbook-and-sheets.md`, "Harness runs". The first path
//!   is WRITTEN, not read: `Package::new()` with no edits goes to it. It is
//!   then opened, its first sheet takes `Number(7)`, `Text("emend")` and
//!   `Boolean(true)` in row 1, columns 1 to 3, and the second path is
//!   written. The second is reopened and one line goes to stdout: the word
//!   `new` then `entries`, `sheets`, `cells` and `read_back` (the cells that
//!   read back with the value written). COUNTS ONLY, and the values are this
//!   tool's own constants. `cells` and `read_back` are asserted equal to 3;
//!   either failing is exit 2 with the message, the second file removed and
//!   the first kept (P2 ruling 4). Each file goes through a temporary and is
//!   moved into place, replacing what was there, as every output is.
//!
//! - `--sheet-sample`: the sheet sample of
//!   `specs/0016-new-workbook-and-sheets.md`, "Harness runs", with the
//!   planning rulings during P3, 1 and 7. A worksheet is added under the
//!   first free name of `emend`, `emend2`, `emend3` and so on, compared as the
//!   layer's name check compares; (1, 1) on it takes `Number(7)`; it moves to
//!   position 0; the package is written. The output is reopened and one line
//!   goes to stdout: the word `sheet` then `sheets_before`, `sheets_after`,
//!   `first` (1 when the output's first sheet is the added part), `cells` (the
//!   added part's cell count), `names` (the `definedName` elements, counted
//!   before and after and asserted equal), `active_before` and `active_after`
//!   (the first `workbookView`'s `activeTab`, absent read as 0). COUNTS ONLY:
//!   the name is this tool's own and no sheet name of the workbook is
//!   printed. `sheets_after` one more, `first` 1, `cells` 1, the names equal
//!   and `active_after` one more are asserted; a failure is exit 2 with the
//!   message and no output. The two counts are read by this tool's own pass
//!   over the workbook part's tokens (`emend::xml::Stream`, quick-xml
//!   underneath), not by a crate call (P3 ruling 7).
//!
//! - `--remove-sample`: the removal sample of
//!   `specs/0016-new-workbook-and-sheets.md`, "Harness runs". The last sheet
//!   of the list is removed and the package written. When the layer refuses
//!   the removal as the last sheet or the last visible sheet, nothing is
//!   recorded and the output is the input's bytes. The output is reopened
//!   and one line goes to stdout: the word `remove` then `sheets_before`,
//!   `sheets_after`, `removed` (0 or 1), `parts` (the entries the output has
//!   fewer than the input, the chain counted), `chain_before` and
//!   `chain_after` (1 when the workbook names a calculation chain),
//!   `names_before`, `names_after`, `local_removed` (the input's
//!   `definedName` elements whose `localSheetId` is the removed sheet's
//!   index) and `active_before` and `active_after` (as `--sheet-sample`
//!   reads them). COUNTS ONLY. Asserted: `sheets_after` is `sheets_before`
//!   less `removed`; `parts` is 0 and the output the input's bytes when
//!   nothing was removed; `chain_after` is 0 when a sheet was removed;
//!   `names_after` is `names_before` less `local_removed`; and
//!   `active_after` is the first visible sheet's index when the active tab
//!   was the removed sheet, and `active_before` otherwise. A failure is exit
//!   2 with the message and no output. The counts come from the same pass
//!   as `--sheet-sample`'s.
//!
//! - `--names-sample`: the names sample of
//!   `specs/0019-names-links-properties-protection.md`, "Harness runs". W is
//!   the first sheet of kind `Worksheet` and S its name; a reference to S is
//!   built here as `'S'!` with every apostrophe in S doubled and is never
//!   printed. `emend_global` with the text `'S'!$A$1` for the workbook and
//!   `emend_local` with `'S'!$B$2` for S are added and the package written.
//!   The output is reopened and one line goes to stdout: the word `names`
//!   then `names_before`, `names_after`, `local_after` (the names whose scope
//!   is W's index), `chain_before`, `chain_after`, `full_calc` (the output's
//!   flag) and `names_removed`: on the reopened output, in memory,
//!   `emend_local` for S is removed, the package written into a buffer and
//!   that reopened, and its names counted. COUNTS ONLY, and the names and
//!   texts are this tool's own constants. Asserted: `names_after` two more
//!   than `names_before`, `local_after` one more than the input's, and
//!   `names_removed` one more than `names_before`; a failure is exit 2 with
//!   the message and no output, and so is a refusal by the layer. The tool
//!   turns on the layer's `metadata` feature for it.
//!
//! - `--link-sample`: the link sample of
//!   `specs/0019-names-links-properties-protection.md`, "Harness runs". W is
//!   the first sheet of kind `Worksheet`, S its name and M its largest row
//!   number as the layer's cell read gives it. `(M + 1, 1)` is set to
//!   `Text("emend link")` and takes an external link to
//!   `https://example.com/` with the tooltip `emend sample`, and `(M + 1, 2)`
//!   a link to the location `'S'!A1`, S's apostrophes doubled and never
//!   printed; the package is written. The output is reopened and one line
//!   goes to stdout: the word `link` then `cells_before`, `cells_after`,
//!   `links_before`, `links_after` (the `hyperlink` elements of W's main-
//!   namespace `hyperlinks`, counted by this tool's own pass over the part's
//!   tokens), `external_rels_before`, `external_rels_after` (the external
//!   relationships of the hyperlink type in W's rels part) and
//!   `r_bound_before` (1 when W's root binds a prefix to the relationships
//!   namespace). COUNTS ONLY. Asserted: one more cell, two more links and
//!   one more external relationship; a failure is exit 2 with the message
//!   and no output, and so is a refusal by the layer, a cell past the grid
//!   included.
//!
//! - `--props-sample`: the properties sample of
//!   `specs/0019-names-links-properties-protection.md`, "Harness runs": the
//!   title `emend sample`, the keywords `emend` and the modified date
//!   `2026-01-01T00:00:00Z` set, the package written and reopened. One line
//!   goes to stdout: the word `props` then `core_before` (1 when the input
//!   has a core-properties relationship), `title_after`, `keywords_after`
//!   and `modified_after` (1 when the reopened value is the one set), and
//!   `props_before` and `props_after` (how many of the thirteen properties
//!   the part holds). COUNTS ONLY, the values being this tool's own.
//!   Asserted: the three read back; a failure is exit 2 with the message and
//!   no output, and so is a refusal by the layer.
//!
//! - `--protect-sample`: the protection sample of the same section: W, the
//!   first sheet of kind `Worksheet`, protected with the password `emend`
//!   and the default options but `sort` and `autoFilter` unlocked, and the
//!   workbook's structure locked with the same password. One line: the
//!   word `protect` then `sheet_prot_before`, `sheet_prot_after` (the
//!   `sheetProtection` children of W's root in the main namespace, counted
//!   by this tool's own pass), `book_prot_before`, `book_prot_after` (the
//!   workbook root's `workbookProtection` children, likewise) and `hash`
//!   (the password's hash as four hex digits, a constant). Asserted: one of
//!   each after; a failure is exit 2 with the message and no output.
//! - `--validate-sample`: the validation sample of
//!   `specs/0020-validation-and-conditional-formatting.md`, "Harness runs",
//!   with P1 ruling 2: on W, the first sheet of kind `Worksheet`, with M its
//!   largest row number as the layer's cell read gives it, a list
//!   validation of the tool's quoted list on the cell of row M + 1, column
//!   1, blanks allowed, with an input message and a `Stop` error message of
//!   the tool's own texts; and a whole-number validation between 1 and 10,
//!   the operator given, on rows M + 1 to M + 2 of column 2. The output is
//!   reopened, and the list validation removed from it again in memory, the
//!   write going into a `Cursor` that is reopened. One line: the word
//!   `validate` then `dv_before` and `dv_after` (the entries
//!   `Package::validations` reads from W), `container_before` (1 when W's
//!   root has a main-namespace `dataValidations` child), `count_after` (that
//!   child's `count` in the output, both read by this tool's own pass over
//!   the part's tokens) and `dv_removed` (the entries read after the
//!   removal). COUNTS ONLY: every formula and message is this tool's
//!   constant and no reference is printed. Asserted: `dv_after` two more
//!   than `dv_before`, `count_after` equal to `dv_after`, `dv_removed` one
//!   more than `dv_before`; a failure is exit 2 with the message and no
//!   output, and so is a refusal by the layer (a validation of the file
//!   overlapping the sample's cells among them, or M + 2 past the grid).
//! - `--cf-sample`: the conditional format sample of
//!   `specs/0020-validation-and-conditional-formatting.md`, "Harness runs":
//!   one differential style (a bold font in `Rgb(0x9C0006)` and a solid fill
//!   with `Rgb(0xFFC7CE)` as its background, openpyxl's light red fill with
//!   dark red text, the colours as the spec gives them), then on W, with M as
//!   `--validate-sample` has it, a `cellIs` greater-than `5` rule on row
//!   M + 1, columns 1 to 3, and an `expression` rule of the tool's own text
//!   `TRUE` with stop-if-true on row M + 2, column 1, both with that style
//!   and no priority. The output is reopened, and the second rule's
//!   conditional format removed from it again in memory, the write going
//!   into a `Cursor` that is reopened. One line: the word `cf` then
//!   `cf_before` and `cf_after` (the rules `Package::conditional_formats`
//!   reads from W), `dxfs_before` and `dxfs_after` (the styles part's `dxf`
//!   children of its main-namespace `dxfs`, counted by this tool's own pass
//!   over the part's tokens), `dxfs_present_before` (1 when the styles part
//!   has a `dxfs`), `priority_max_before` (the highest priority W's rules
//!   read, 0 when there is none), `priority_first_after` (the priority the
//!   first added rule reads back with) and `cf_removed` (the rules read after
//!   the removal). COUNTS ONLY. Asserted: `cf_after` two more than
//!   `cf_before`, `dxfs_after` one more than `dxfs_before`,
//!   `priority_first_after` one more than `priority_max_before`, and
//!   `cf_removed` one more than `cf_before`; a failure is exit 2 with the
//!   message and no output, and so is a refusal by the layer.
//! - `--scale-sample`: the visual rule sample of
//!   `specs/0020-validation-and-conditional-formatting.md`, "Harness runs":
//!   on W, with M as `--validate-sample` has it, three conditional formats
//!   with no style and no priority: a colour scale from `Min` in
//!   `Rgb(0xF8696B)` to `Max` in `Rgb(0x63BE7B)` on rows M + 1 to M + 3 of
//!   column 1, a data bar from `Min` to `Max` in `Rgb(0x638EC6)` with the
//!   value shown on the same rows of column 2, and a three-arrow icon set
//!   with the thresholds 0, 33 and 67 percent on column 3, the colours as the
//!   spec gives them. The output is reopened. One line: the word `scale`
//!   then `cf_before` and `cf_after` (the rules `Package::conditional_formats`
//!   reads from W), `visual_after` (the output's rules of the kinds
//!   `colorScale`, `dataBar` and `iconSet`) and `extended_before` (the
//!   input's rules holding an extension). COUNTS ONLY. Asserted: `cf_after`
//!   three more than `cf_before`, and `visual_after` three more than the
//!   input's visual rules; a failure is exit 2 with the message and no
//!   output, and so is a refusal by the layer.
//! - `--table-sample`: the table sample of `specs/0021-tables.md`, "Harness
//!   runs": on W, with M as `--validate-sample` has it, a table named
//!   `emend_table` over rows M + 1 to M + 3 of columns 1 and 2, its columns
//!   `emend_a` and `emend_b`, in `TableStyleMedium2` with row stripes only.
//!   The output is reopened. One line: the word `table` then
//!   `tables_before` and `tables_after` (the entries `Package::tables` reads
//!   from W), `parts_before` and `parts_after` (the archive's entries under
//!   `xl/tables/`), `rels_before` and `rels_after` (W's relationships of type
//!   `TABLE`, either form), `container_before` (1 when W's root has a
//!   main-namespace `tableParts` child), `r_bound_before` (1 when W's root
//!   binds a prefix to the relationships namespace), both read by this
//!   tool's own pass over the part's tokens, `id_after` (the added table's
//!   root `id`, read by the same pass over its part) and `cells_before` and
//!   `cells_after` (W's cells as the layer's cell read gives them); then the
//!   table removed again from the reopened output in memory, the write going
//!   into a `Cursor` that is reopened, for `tables_removed` and
//!   `parts_removed` (specs/0021 "P2"). COUNTS ONLY: the name and the column
//!   names are this tool's constants and no reference is printed. Asserted:
//!   `tables_after`, `parts_after` and `rels_after` one more than before,
//!   `cells_after` two more, and `tables_removed` and `parts_removed` equal
//!   to before; a
//!   failure is exit 2 with the message and no output, and so is a refusal
//!   by the layer (a table, merge or autofilter of the file on the sample's
//!   cells, a table of that name, or M + 3 past the grid).
//! - `--comment-sample`: the comment sample of
//!   `specs/0022-comments-and-images.md`, "Harness runs": on W, with M as
//!   `--table-sample` has it, a comment by `emend` with the text `emend
//!   comment` on (M + 1, 1). The output is reopened. One line: the word
//!   `comment` then `comments_before` and `comments_after` (the entries
//!   `Package::comments` reads from W), `parts_before` and `parts_after`
//!   (the archive's entries `xl/comments*.xml` and
//!   `xl/drawings/vmlDrawing*.vml`), `legacy_before` (1 when W's root has a
//!   main-namespace `legacyDrawing` child), `comments_part_before` (1 when
//!   W has a relationship of type `COMMENTS`, either form), `r_bound_before`
//!   (1 when W's root binds a prefix to the relationships namespace),
//!   `shapes_before` and `shapes_after` (the shapes of W's VML part holding
//!   a `ClientData` of `ObjectType` `Note`) and `authors_before` and
//!   `authors_after` (the `author` elements of W's comments part), the last
//!   three read by this tool's own pass over the part's tokens. COUNTS ONLY:
//!   the author and the text are this tool's constants and no reference is
//!   printed. Asserted: `comments_after` and `shapes_after` one more than
//!   before, `parts_after` two, one or no more as W had neither part, the
//!   VML part alone or both, and `authors_after` one more unless a kept
//!   author's text is `emend`; then the comment removed again from the
//!   reopened output in memory, the write going into a `Cursor` that is
//!   reopened, for `comments_removed` and `parts_removed`
//!   (specs/0022 "P2"), asserted equal to `comments_before` and
//!   `parts_before`; a failure is exit 2 with the message and no output, and
//!   so is a refusal by the layer (threaded comments on W, a part it cannot
//!   read, or M + 1 past the grid).
//! - `--image-sample`: the image sample of
//!   `specs/0022-comments-and-images.md`, "Harness runs": on W, with M as
//!   `--table-sample` has it, the tool's constant PNG at its own size on
//!   (M + 1, 1). The output is reopened. One line: the word `image` then
//!   `images_before` and `images_after` (the entries `Package::images`
//!   reads from W), `drawing_before` (1 when W's root has a main-namespace
//!   `drawing` child), `anchors_before` and `anchors_after` (the root
//!   children of W's drawing part, by this tool's own pass over its
//!   tokens), `media_before` and `media_after` (the archive's entries under
//!   `xl/media/`), `r_bound_before` (1 when W's root binds a prefix to the
//!   relationships namespace), and `from_after`, the added entry's `from`
//!   as `column,row`. COUNTS ONLY: the image is this tool's constant and no
//!   reference is printed. Asserted: `images_after`, `anchors_after` and
//!   `media_after` one more than before, and `from_after` `0,M`; a failure
//!   is exit 2 with the message and no output, and so is a refusal by the
//!   layer (a drawing part it cannot read, two drawing elements on W, or
//!   M + 1 past the grid).
//! - `--richtext-sample`: the rich text sample of
//!   `specs/0024-rich-text.md`, "Harness runs": on W, with M as
//!   `--table-sample` has it, a rich value of two runs, `emend` bold and
//!   ` rich` with no font, on (M + 1, 1). The output is reopened. One line:
//!   the word `richtext` then `sst_rich_before` and `sst_rich_after` (the
//!   shared strings items `SharedStrings::runs_of` gives runs for),
//!   `inline_rich_before` and `inline_rich_after` (W's cells whose `runs`
//!   is `Some`), `runs_after` (the written cell's run count), `text_after`
//!   (1 when its value is `emend rich`), `bold_first` (1 when its first run's
//!   font is bold and its second run has none) and `r_bound_before` (1 when
//!   W's root binds a prefix to the relationships namespace). COUNTS ONLY:
//!   the runs are this tool's constants and no reference is printed.
//!   Asserted: `sst_rich_after` equal to before, `inline_rich_after` one
//!   more, `runs_after` 2, `text_after` and `bold_first` 1; a failure is
//!   exit 2 with the message and no output, and so is a refusal by the
//!   layer (run properties it cannot read, or M + 1 past the grid).
//! - `--richcomment-sample`: the rich comment sample of
//!   `specs/0024-rich-text.md`, "Harness runs": on W, with M as
//!   `--table-sample` has it, a comment by `emend` of two runs, `emend:`
//!   bold and ` rich comment` with no font, on (M + 1, 1). The output is
//!   reopened. One line: the word `richcomment` then `--comment-sample`'s
//!   counts before its removal, `comments_before` to `authors_after`, each
//!   asserted as there, then `rich_comments_before` and
//!   `rich_comments_after` (the entries `Package::comments` reads from W
//!   whose `runs` is `Some`), `runs_after` (the written comment's run count)
//!   and `text_after` (1 when its text is `emend: rich comment`). COUNTS
//!   ONLY: the author and the runs are this tool's constants and no
//!   reference is printed. Asserted besides: `rich_comments_after` one more
//!   than before, `runs_after` 2 and `text_after` 1; a failure is exit 2
//!   with the message and no output, and so is a refusal by the layer
//!   (threaded comments on W, a part it cannot read, run properties it
//!   cannot read, or M + 1 past the grid).
//! - `--sheetview-sample`: the sheet view sample of
//!   `specs/0025-sheet-and-print-settings.md`, "Harness runs": on W, the
//!   tab colour `FF00AA55` and a sheet view of zoom 85, gridlines off and
//!   headings on. The output is reopened and W read by this tool's own pass
//!   over its tokens. One line: the word `sheetview` then `sheetpr_before`
//!   and `sheetpr_after` (W's `<sheetPr>` elements), `tabcolor_before` and
//!   `tabcolor_after` (the `<tabColor>` children of W's `<sheetPr>`),
//!   `views_before` and `views_after` (W's `<sheetView>` elements),
//!   `zoom_ok`, `grid_ok` and `headings_ok` (1 when the first `<sheetView>`
//!   carries `zoomScale="85"`, `showGridLines="0"` and
//!   `showRowColHeaders="1"`), `kept_attrs` (the first `<sheetView>`'s
//!   attributes other than those three, after the write) and
//!   `r_bound_before` (1 when W's root binds a prefix to the relationships
//!   namespace). COUNTS ONLY: the settings are this tool's constants and no
//!   reference is printed. Asserted: `sheetpr_after` 1, `tabcolor_after` 1
//!   with `rgb="FF00AA55"`, `views_after` equal to before or 1 when W had
//!   none, the three flags 1, and the kept attributes the same names and
//!   bytes in the same order as before (for a created view, exactly
//!   `workbookViewId="0"`); a failure is exit 2 with the message and no
//!   output, and so is a refusal by the layer (two `<sheetPr>` on W, one out
//!   of schema order, or two `<sheetViews>`).
//! - `--pagesetup-sample`: the page setup sample of
//!   `specs/0025-sheet-and-print-settings.md`, "Harness runs": on W, print
//!   options centred horizontally, a left margin of 0.5, a page setup of
//!   landscape fitted one page wide and automatic tall with fit to page
//!   on, an odd header `HeaderFooter::section("emend", "", "&P of &N")` and
//!   an odd footer `section("", "emend", "")`. The output is reopened and W
//!   read by this tool's own pass over its tokens. One line: the word
//!   `pagesetup` then, for each of `printOptions`, `pageMargins`,
//!   `pageSetup` and `headerFooter`, `<name>_before` and `<name>_after` (W's
//!   root children of that name), then `rid_kept` (1 when W's `<pageSetup>`
//!   had a relationship id and keeps that attribute byte for byte, or had
//!   none), `fit_ok` (1 when `<sheetPr>`'s `<pageSetUpPr>` carries
//!   `fitToPage="1"` and `<pageSetup>` `fitToWidth="1"` and
//!   `fitToHeight="0"`), `margin_ok` (`left="0.5"`), `orient_ok`
//!   (`orientation="landscape"`), `header_ok` and `footer_ok` (1 when the
//!   odd header and footer hold the two texts), `hf_kept` (the
//!   `<headerFooter>` children other than those two, after the write) and
//!   `r_bound_before`. COUNTS ONLY: the settings are this tool's constants
//!   and no reference or text from the input is printed. Asserted: each
//!   `_after` 1, `rid_kept`, the five flags 1, and the other
//!   `<headerFooter>` children the same bytes in the same order as before;
//!   a failure is exit 2 with the message and no output, and so is a
//!   refusal by the layer (two of an element the sample sets on W, or one
//!   out of schema order).
//! - `--printarea-sample`: the print area sample of
//!   `specs/0025-sheet-and-print-settings.md`, "Harness runs": on W, with M
//!   its largest row number as the cell read gives it, the print area A1 to
//!   (M + 1, 8), the print titles of row 1, and an autofilter over the same
//!   area. The output is reopened. One line: the word `printarea` then
//!   `names_before` and `names_after` (the workbook's defined names),
//!   `area_ok`, `titles_ok` and `filterdb_ok` (1 when W's sheet-scoped
//!   `_xlnm.Print_Area`, `_xlnm.Print_Titles` and `_xlnm._FilterDatabase`
//!   exist, each once, with the expected text behind W's name bare or
//!   quoted), `autofilter_before` and `autofilter_after` (W's `<autoFilter>`
//!   root children), `hidden_ok` (1 when the filter database is hidden) and
//!   `r_bound_before`. COUNTS ONLY: no name, reference or text is printed.
//!   Asserted: `names_after` three more than before less those of the three
//!   W already had, the three flags and `hidden_ok` 1, `autofilter_after`
//!   1; a failure is exit 2 with the message and no output, and so is a
//!   refusal by the layer (a table of W overlapping the area, two
//!   `<autoFilter>` on W, or M + 1 past the grid).
//! - `--group-sample`: the group sample of
//!   `specs/0026-grouping-and-line-styles.md`, "Harness runs": on W, with M
//!   its largest row number as the cell read gives it, the rows M + 1 to
//!   M + 2 grouped at level 1 hidden, row M + 3 collapsed, the columns 2 to
//!   3 grouped at level 1 shown, and the summary rows above their groups.
//!   The output is reopened and W read by this tool's own pass over its
//!   tokens. One line: the word `group` then `rows_before` (W's `<row>`
//!   elements), `rows_present` (the input's `<row>` elements numbered M + 1
//!   to M + 3, rows with no cell, which the write edits in place;
//!   "Planning rulings during P1", ruling 6), `rows_after`, `level_rows`
//!   (rows carrying `outlineLevel="1"` after), `hidden_rows` (of those, the ones carrying `hidden="1"`),
//!   `collapsed_rows` (rows carrying `collapsed="1"`), `cols_before` and
//!   `cols_after` (W's `<col>` elements), `level_cols` (`<col>` elements
//!   carrying `outlineLevel="1"` after), `max_row_ok` and `max_col_ok` (1
//!   when `<sheetFormatPr>` carries `outlineLevelRow` and `outlineLevelCol`
//!   at least 1), `summary_ok` (1 when `<sheetPr>`'s `<outlinePr>` carries
//!   `summaryBelow="0"`), `kept_cols` (summed over columns 2 and 3, the
//!   attributes of the `<col>` covering the column other than `min`, `max`,
//!   `outlineLevel`, `hidden` and `collapsed`, each the same name and bytes
//!   before and after) and `r_bound_before`. COUNTS ONLY: the levels and the
//!   flags are this tool's constants and no reference is printed. Asserted:
//!   `rows_after` equal to `rows_before + 3 - rows_present` with the rows
//!   M + 1 to M + 3 all present after the write, `level_rows` 2, `hidden_rows` 2,
//!   `collapsed_rows` 1, the `<col>` elements carrying the level covering
//!   exactly columns 2 to 3 between them and numbering 1 when the input's
//!   attributes for the two columns, as the write leaves them, are the same
//!   name and bytes in the same order, else 2 ("Planning rulings during
//!   P1", ruling 2), the kept attributes and the three flags; a failure is
//!   exit 2 with the message and no output, and so is a refusal by the
//!   layer (M + 3 past the grid, two `<sheetPr>` or `<sheetFormatPr>` on W,
//!   or one out of schema order).
//! - `--linestyle-sample`: the line style sample of
//!   `specs/0026-grouping-and-line-styles.md`, "Harness runs": a bold font
//!   added on cell format 0 as the format i, row M + 1 of W styled i and
//!   column 9 of W styled i. The output is reopened and W read by this
//!   tool's own pass over its tokens, the styles part through the layer.
//!   One line: the word `linestyle` then `rows_before` (W's `<row>`
//!   elements), `rows_present` (the input's `<row>` numbered M + 1, 0 or 1,
//!   which the write edits in place; "Planning rulings during P2", ruling
//!   1), `rows_after`, `row_style_ok` (1 when row M + 1 carries `s="i"` and
//!   `customFormat="1"`), `cols_before` and `cols_after` (W's `<col>`
//!   elements), `col_style_ok` (1 when the `<col>` covering column 9 covers
//!   it alone and carries `style="i"`), `col_split` (the number of `<col>`
//!   elements after the write inside the span of the input's `<col>`
//!   covering column 9, 1 to 3; 1 when none covered it), `kept_cols` (the
//!   attributes of the `<col>` covering column 9 other than `min`, `max`
//!   and `style`, each the same name and bytes before and after),
//!   `xfs_before` and `xfs_after` (the styles part's cell formats) and
//!   `r_bound_before`. COUNTS ONLY: the font and the indices are this
//!   tool's constants and no reference is printed. Asserted: `rows_after`
//!   equal to `rows_before + 1 - rows_present` with row M + 1 present
//!   after the write, the two flags 1, `col_split` from 1 to 3, the kept
//!   attributes, `xfs_after` one more than before and i equal to
//!   `xfs_before`; a failure is exit 2 with the message and no output, and
//!   so is a refusal by the layer (M + 1 past the grid, a styles part the
//!   layer cannot take a format into).
//! - `--roundtrip-sample`: the round-trip sample of
//!   `specs/0027-read-back.md`, "Harness runs": for each distinct `s` of
//!   W's cells, in increasing order, `cell_style(s)`, then `add_style` with
//!   no base and every field of the value read (a fill of `None` given as no
//!   fill). The output is reopened and each added cell format resolved with
//!   `cell_style` again. One line: the word `roundtrip` then `styles_in`
//!   (distinct `s` on W), `styles_added`, `xfs_before`, `xfs_after`,
//!   `fonts_before`, `fonts_after`, `fills_before`, `fills_after`,
//!   `borders_before`, `borders_after` and `equal` (added cell formats that
//!   read back equal to the value read from the input, every field but a
//!   `None` fill, which `add_style` writes as `fillId="0"` and which is
//!   compared with fill 0's read value: the value `cell_style` gives the
//!   output's first cell format whose fill is fill 0; `specs/0027-read-back.md`,
//!   "Planning verification and rulings on P1", ruling 1). COUNTS ONLY: no
//!   reference, index, name or code is printed. Asserted: `styles_added`
//!   equal to `styles_in`, `xfs_after` equal to `xfs_before + styles_in`,
//!   each collection grown by the distinct values added (equal values share
//!   one element, as `add_style` records them), and `equal` equal to
//!   `styles_in`; a failure is exit 2 with the message and no output, and so
//!   is a refusal by the layer (a style it cannot read, a value it cannot
//!   write).
//! - `--namedstyles-sample`: the named styles sample of
//!   `specs/0032-read-breadth-styles.md`, "Harness runs", and "Planning
//!   rulings during P1": every named style read (`named_style` of every
//!   `cellStyleXfs` index), the default font and every differential style
//!   read; then, for each differential style whose fields are a font, a
//!   fill and a border alone (at least one given),
//!   `add_differential_style` of the value read, the package written. The
//!   output is reopened and its differential styles read again. One line:
//!   the word `namedstyles` then `named` (`cellStyle` children),
//!   `named_xfs` (`cellStyleXfs` children), `builtin` (named styles with a
//!   `builtinId`), `custom` (with `customBuiltin`), `resolved` (named
//!   styles resolved), `default_font_size` (the default font's size, 0 for
//!   none), `dxfs`, `dxf_fonts`, `dxf_fills`, `dxf_borders`, `dxf_numfmts`,
//!   `dxf_alignments`, `dxf_protections` (the `dxf`s read with each field
//!   given), `dxf_gradients` (a `fill` child read as no fill),
//!   `dxf_implied_solid` (a `patternFill` with no `patternType`, read as
//!   solid), `dxf_inside_sides` (`vertical` and `horizontal` sides read
//!   past), `dxf_written` (written back) and `dxf_equal` (appended ones that
//!   read back equal to the value read from the input). `dxf_gradients`,
//!   `dxf_implied_solid` and `dxf_inside_sides` are taken from the part's
//!   own bytes beside the typed read. COUNTS ONLY: no reference, index, name,
//!   code or text is printed, and a style name is counted, never printed.
//!   Asserted: `resolved` equal to `named_xfs`, each implied solid fill read
//!   as `Solid`, the output's differential styles the input's followed by
//!   the ones written, and `dxf_equal` equal to `dxf_written`; a failure is
//!   exit 2 with the message and no output, and so is a refusal by the
//!   layer (a style it cannot read).
//! - `--colors-sample`: the colours sample of
//!   `specs/0032-read-breadth-styles.md`, "Harness runs", and "Planning
//!   rulings during P2": the input written back unedited, and every colour
//!   of every typed font, fill (`fgColor` and `bgColor`) and border side
//!   (`left`, `right`, `top`, `bottom`, `diagonal`), then of every
//!   differential style, resolved through `Package::resolve_color`. One
//!   line: the word `colors` then `colors` (colour values met), `rgb`,
//!   `theme`, `indexed`, `auto` (by form), `resolved` (`Some`),
//!   `unresolved` (`None`), `tinted` (resolved with a tint), `theme_part`
//!   (1 with a theme part), `scheme_dark_text` (font colours of theme 1
//!   whose resolved `rgb` has every channel below 128), `scheme_light_fill`
//!   (fill colours of theme 0 whose resolved `rgb` has every channel at or
//!   above 128), `palette_override` (1 with an `indexedColors`), and
//!   `digest_fonts`, `digest_fills`, `digest_borders` and `digest_dxfs`:
//!   64-bit FNV-1a over each group's entries in that order, an entry
//!   `rgb:tinted` as eight plus eight lower-case hex digits or `none`, each
//!   followed by a newline (ruling 10). COUNTS, FLAGS AND DIGESTS ONLY: no
//!   reference, name, text or colour value is printed. Asserted: the four
//!   forms summing to `colors`, and `resolved` and `unresolved` too; a
//!   failure is exit 2 with the message and no output, and so is a refusal
//!   by the layer (a style or theme it cannot read, a colour it cannot
//!   resolve).
//! - `--kinds-sample`: the survey sample of
//!   `specs/0033-read-breadth-survey.md`, "Harness runs": the input written
//!   back unedited, `Package::part_kinds`, then `Package::sheet_parts` of
//!   every sheet of the workbook's sheet list. One line: the word `kinds`
//!   then every `PartKinds` field by its name, `entries` (the archive's
//!   entries), `kinds_sum` (every field summed), `sheets` (sheets walked),
//!   then every `SheetParts` field summed over the sheets, under the prefix
//!   `sheet_`. COUNTS ONLY: no name, reference, path or text is printed.
//!   Asserted: `kinds_sum` equal to `entries` less one; a failure is exit 2
//!   with the message and no output, and so is a refusal by the layer.
//! - `--pivot-sample`: the pivot sample of
//!   `specs/0033-read-breadth-survey.md`, "Harness runs": the input written
//!   back unedited and `Package::pivot_tables`. One line: the word `pivots`
//!   then `pivot_tables`, `with_sheet` (`sheet` `Some`), `with_cache`,
//!   `source_worksheet`, `source_external`, `source_other`, `source_named`
//!   (a worksheet source with a `name`), `source_ref` (with a `reference`),
//!   `cache_fields`, `records_known` (`record_count` `Some`), `fields`,
//!   `named_fields` (`name` `Some`), `axis_row`, `axis_column`,
//!   `axis_page`, `axis_values`, `data_field_flags`, `row_fields`,
//!   `column_fields`, `page_fields`, `values_refs` (`FieldRef::Values`
//!   across the three lists), `data_fields`, `named_data_fields`,
//!   `subtotal_sum`, `subtotal_other`, `location_cells` (the cells of every
//!   `location` area summed), and `digest_names` and `digest_fields`:
//!   64-bit FNV-1a over the pivot table names and over the cache field
//!   names, tables in order and each cache's fields in order, each followed
//!   by a newline. COUNTS AND DIGESTS ONLY: no name, sheet, reference or
//!   field text is printed. Asserted: the three source counts summing to
//!   `with_cache`; a failure is exit 2 with the message and no output, and
//!   so is a refusal by the layer.
//! - `--extlink-sample`: the external link sample of
//!   `specs/0033-read-breadth-survey.md`, "Harness runs": the input written
//!   back unedited and `Package::external_links`. One line: the word
//!   `extlinks` then `links`, `books`, `dde`, `ole`, `sheet_names` (over
//!   the books), `defined_names` (over the books), and `digest_targets`
//!   and `digest_sheets`: 64-bit FNV-1a over the book and OLE targets and
//!   over the books' sheet names, links in order, each followed by a
//!   newline. COUNTS AND DIGESTS ONLY: no target, sheet name, service,
//!   topic or program id is printed. Asserted: `books`, `dde` and `ole`
//!   summing to `links`; a failure is exit 2 with the message and no
//!   output, and so is a refusal by the layer.
//! - `--charts-sample`: the charts sample of `specs/0034-charts-read.md`,
//!   "Harness runs", with every field (P1's, then P2's axes, legend, pivot
//!   source and style): the input written back
//!   unedited and `Package::charts` of every worksheet and chartsheet of
//!   the workbook's sheet list (`specs/0038-chartsheets.md`, "Harness
//!   runs"). One line: the word `charts` then `sheets` (worksheets and
//!   chartsheets read), `charts`, `plots`, `kind_bar` (bar and 3D bar plots),
//!   `kind_line` (line and 3D line), `kind_pie` (pie and 3D pie),
//!   `kind_doughnut`, `kind_scatter`, `kind_other` (every other kind),
//!   `combined` (charts with two or more plots), `bar_dir_bar`,
//!   `bar_dir_col`, `grouping_clustered`, `grouping_stacked`,
//!   `grouping_standard`, `grouping_percent`, `series`, `name_ref`,
//!   `name_text`, `unnamed`, `cat_ref`, `cat_lit`, `val_ref`, `val_lit`,
//!   `sizes`, `cached` (references with a cache, over categories, values
//!   and sizes), `fills`, `smooth`, `markers`, `labels`, `titles`,
//!   `auto_title_deleted`, `axes`, `axis_cat`, `axis_val`, `axis_other`
//!   (date and series axes), `axis_titles`, `axis_numfmt`, `axis_bounds`
//!   (a `min` or `max`), `axis_deleted`, `gridlines` (major), `legends`,
//!   `pivot_charts` (charts with a pivot source), `styles`, `anchor_two`,
//!   `anchor_one`, `anchor_abs`, `edit_as_onecell`, `edit_as_twocell`,
//!   `edit_as_abs`, and `digest_titles`, `digest_names`,
//!   `digest_formulas`, `digest_numfmts` and `digest_sources`: 64-bit
//!   FNV-1a over the chart titles, over the series names (a reference's
//!   formula or the text), over every reference's formula (categories,
//!   values, sizes, series in order), over the axes' number formats and
//!   over the pivot sources, each followed by a newline. COUNTS, FLAGS AND
//!   DIGESTS ONLY: no title, name, formula, number format, pivot source or
//!   part name is printed. Asserted: the six kind counts summing to
//!   `plots`, `name_ref`, `name_text` and `unnamed` to `series`, and
//!   `axis_cat`, `axis_val` and `axis_other` to `axes`; a failure is exit
//!   2 with the message and no output, and so is a refusal by the layer.
//! - `--add-chart-sample`: the add-chart sample of
//!   `specs/0035-charts-write-first-set.md`, "Harness runs": on W, with M
//!   its largest row number as the cell read gives it and its name quoted
//!   by doubling each apostrophe and wrapping in apostrophes, one clustered
//!   column chart on a two-cell anchor from (M + 2, 1) to (M + 18, 9),
//!   offsets zero, `editAs` `oneCell`: the title `emend chart`, a series
//!   named `emend series` over the categories A1:A(M + 1) and the values
//!   B1:B(M + 1) of W, a series named by C1 of W over the values C1:C(M +
//!   1), the axis titles `emend categories` and `emend values`, the legend
//!   at the bottom, no size. The output is reopened. One line: the word
//!   `addchart` then `charts_before` and `charts_after` (`Package::charts`
//!   of W), `read_back_equal` (1 when the last entry of the output's
//!   charts of W reads as the call gave it, every field compared),
//!   `anchors_before` and `anchors_after` (the root children of W's drawing
//!   part), `chart_parts_before` and `chart_parts_after` (entries under
//!   `xl/charts/`), `drawing_created` (1 when the input's W had no drawing)
//!   and `two_after` (charts of W on a two-cell anchor). COUNTS AND FLAGS
//!   ONLY: the texts are this tool's constants, and no reference, sheet
//!   name or part name is printed. Asserted: `charts_after`,
//!   `anchors_after`, `chart_parts_after` and `two_after` one more than
//!   before, `read_back_equal` 1; a failure is exit 2 with the message and
//!   no output, and so is a refusal by the layer (M + 18 past the grid, a
//!   drawing it cannot read).
//! - `--chart-kinds-sample`: the chart kinds sample of
//!   `specs/0035-charts-write-first-set.md`, "Harness runs": on W, with M
//!   and its quoted name as `--add-chart-sample` has them and M' the larger
//!   of M and 1 (P2 ruling J), six charts, the k-th (k from 0) on a
//!   one-cell anchor at (M + 2 + 16 k, 1), offset zero, 480 by 288 pixels:
//!   a clustered bar chart with direction bar, a stacked column chart, a
//!   standard line chart, a pie, a scatter chart with markers and lines and
//!   a standard area chart, each titled `emend kind k` with one series named
//!   by B1 of W over the categories A2:A(M' + 1) and the values B2:B(M' + 1),
//!   the legend at the right, and the axis titles `emend x` and `emend y`
//!   but on the pie, which takes none. The output is reopened. One line:
//!   the word `chartkinds` then `charts_before` and `charts_after`
//!   (`Package::charts` of W), `kinds_read_back` (how many of the last six
//!   entries of the output's charts of W read as their calls gave them,
//!   every field compared), `one_after` (charts of W on a one-cell anchor)
//!   and `chart_parts_after` (entries under `xl/charts/`). COUNTS ONLY, as
//!   the add-chart sample's. Asserted: `charts_after`, `one_after` and
//!   `chart_parts_after` six more than before, `kinds_read_back` 6; a
//!   failure is exit 2 with the message and no output, and so is a refusal
//!   by the layer.
//! - `--chart-more-sample`: the more kinds sample of
//!   `specs/0036-charts-write-second-set.md`, "Harness runs": on W, with M,
//!   M' and its quoted name as `--chart-kinds-sample` has them, four
//!   charts, the k-th (k from 0) on a one-cell anchor at (M + 2 + 16 k, 1),
//!   offset zero, 480 by 288 pixels: a doughnut with hole size 50, a radar
//!   chart with markers, a bubble chart and a stock chart, each titled
//!   `emend more k` with the legend at the right; the doughnut and the
//!   radar one series named by B1 of W over the categories A2:A(M' + 1)
//!   and the values B2:B(M' + 1), the bubble the same with the sizes
//!   C2:C(M' + 1), the stock three series named `high`, `low` and `close`
//!   over the values of B, C and D, rows 2 to M' + 1, and the categories
//!   of A; the axis titles `emend x` and `emend y` but on the doughnut,
//!   which takes none. The output is reopened. One line: the word
//!   `chartmore` then `charts_before` and `charts_after` (`Package::charts`
//!   of W), `kinds_read_back` (how many of the last four entries of the
//!   output's charts of W read back with the call's kind, its series
//!   count, sizes on every series of the bubble and on none of the others,
//!   and its axes count) and `chart_parts_after` (entries under
//!   `xl/charts/`). COUNTS ONLY, as the add-chart sample's. Asserted:
//!   `charts_after` and `chart_parts_after` four more than before,
//!   `kinds_read_back` 4; a failure is exit 2 with the message and no
//!   output, and so is a refusal by the layer.
//! - `--chart-3d-sample`: the 3D sample of
//!   `specs/0036-charts-write-second-set.md`, "Harness runs": on W, with M,
//!   M' and its quoted name as `--chart-kinds-sample` has them, five
//!   charts placed as the more kinds sample's, the k-th (k from 0) titled
//!   `emend 3d k` with the legend at the bottom and no axis titles: a
//!   clustered 3D column chart, a standard 3D line chart (the series
//!   axis), a 3D pie, a stacked 3D area chart, and a clustered column
//!   chart with a standard line combined on the secondary axes, their
//!   title `emend y2` and format `0.0`. Each first plot takes the kinds
//!   sample's one series; the combined line one series named by C1 of W
//!   over the categories A2:A(M' + 1) and the values C2:C(M' + 1). The
//!   output is reopened. One line: the word `chart3d` then `charts_before`
//!   and `charts_after`, `kinds_read_back` (how many of the last five
//!   entries of the output's charts of W read back with the call's kinds,
//!   one plot of one series each, or the combined bar and line, and the
//!   axes: three for the 3D line, none for the 3D pie, two for the other
//!   3D plots, and for the combined chart four with ids 10, 20, 30 and 40,
//!   the last's number format `0.0`) and `chart_parts_after`. COUNTS ONLY,
//!   as the add-chart sample's. Asserted: `charts_after` and
//!   `chart_parts_after` five more than before, `kinds_read_back` 5; a
//!   failure is exit 2 with the message and no output, and so is a refusal
//!   by the layer.
//! - `--chart-surface-sample`: the surface sample of
//!   `specs/0039-chart-kinds-and-formatting.md`, "Harness runs": on W, with
//!   M, M' and its quoted name as `--chart-kinds-sample` has them, four
//!   charts placed as the more kinds sample's, the k-th (k from 0) titled
//!   `emend surface k`: a contour surface (`wireframe` false) and a 3D
//!   surface (`wireframe` true), each of three series named by B1, C1 and
//!   D1 of W over the categories A2:A(L) and the values of B, C and D, rows
//!   2 to L, L being the smaller of M' + 1 and 1001 (ruling A), with the axis titles `emend x` and `emend y` and no
//!   legend; a pie of pie with no split and a bar of pie split at position
//!   2, each of the kinds sample's one series with the legend at the right.
//!   The output is reopened. One line: the word `chartsurface` then
//!   `charts_before` and `charts_after` (`Package::charts` of W),
//!   `surface`, `surface_3d` and `of_pie` (the plots of each kind among the
//!   last four entries of the output's charts of W), `series` (their
//!   series), `rows` (L - 1, the data rows referenced), `read_back` (how many of them read as their calls gave them,
//!   every field compared) and `chart_parts_after` (entries under
//!   `xl/charts/`). COUNTS ONLY, as the add-chart sample's. Asserted:
//!   `charts_after` and `chart_parts_after` four more than before,
//!   `surface` 1, `surface_3d` 1, `of_pie` 2, `series` 8 and `read_back` 4;
//!   a failure is exit 2 with the message and no output, and so is a
//!   refusal by the layer.
//! - `--chart-labels-sample`: the labels sample of
//!   `specs/0036-charts-write-second-set.md`, "Harness runs": on W, with M,
//!   M' and its quoted name as `--chart-kinds-sample` has them, two charts
//!   placed as the more kinds sample's, the k-th (k from 0) titled `emend
//!   labels k` with the legend at the right and no axis titles: a clustered
//!   column chart of two series, the kinds sample's over B coloured
//!   `0x1F77B4` and the same over C named by C1 coloured `0xFF7F0E`, each
//!   labelled with its value outside its end in the format `#,##0`, its
//!   category axis formatted `@` and its value axis `#,##0.00`; and a pie
//!   of the kinds sample's series coloured `0x2CA02C`, labelled with its
//!   categories and percentages at the best fit. The output is reopened.
//!   One line: the word `chartlabels` then `charts_before` and
//!   `charts_after`, `read_back` (how many of the last two entries of the
//!   output's charts of W read back with the call's kind, every series'
//!   fill its colour and its labels present, and the bar's two axes with
//!   the call's formats) and `chart_parts_after`. COUNTS ONLY, as the
//!   add-chart sample's. Asserted: `charts_after` and `chart_parts_after`
//!   two more than before, `read_back` 2; a failure is exit 2 with the
//!   message and no output, and so is a refusal by the layer.
//! - `--remove-chart-sample`: the chart removal sample of
//!   `specs/0036-charts-write-second-set.md`, "Harness runs": the
//!   add-chart sample's chart added on W, the write going into memory and
//!   that reopened; then `remove_chart` of the last index (the addition)
//!   and, when the input's `charts(W)` is not empty, of index 0 too; the
//!   output written and reopened. One line: the word `removechart` then
//!   `charts_before` and `charts_after`, `kept_removed` (1 when the input
//!   had a chart on W), `chart_parts_before` and `chart_parts_after` (the
//!   entries `xl/charts/chart*`, not their rels, style or colours parts),
//!   `parts_dropped` (the input's entries the output lacks),
//!   `drawing_dropped` (1 when W's drawing went), `anchors_before` and
//!   `anchors_after` (the root children of W's drawing part, by this tool's
//!   own pass), `charts_other_sheets` (the charts of every other
//!   worksheet). COUNTS ONLY. Asserted: `charts_after` is `charts_before -
//!   kept_removed` and `charts_other_sheets` the input's; when
//!   `kept_removed` is 0, every entry of the input in the output with the
//!   same uncompressed bytes, but for a rels part the addition created
//!   (W's or its drawing's), left with no relationship, and W's drawing
//!   when it held no anchor, which goes with its rels part, W, its rels
//!   part and the content types changing (P3 ruling M); a failure is exit
//!   2 with the message and no output, and so is a refusal by the layer (a
//!   chart the input's count cannot read: P3 ruling N).
//! - `--chartsheet-sample`: the chartsheet sample of
//!   `specs/0038-chartsheets.md`, "Harness runs": on each workbook, with W,
//!   M and its quoted name as `--add-chart-sample` has them,
//!   `Package::add_chartsheet` of the name `emend chartsheet` holding the
//!   add-chart sample's chart at the size 960 by 640 pixels (9144000 by
//!   6096000 EMU); the output written and reopened. One line: the word
//!   `chartsheet` then `part_length` (the new part's name's length),
//!   `charts` and `series` (`Package::charts` of the new part),
//!   `sheets_after` (the sheet list's length) and `kept_charts` (the charts
//!   of the input's chartsheets, read before the call). COUNTS ONLY.
//!   Asserted: one sheet more, the last a chartsheet of that name at the
//!   returned part, its one chart in an absolute anchor at 0,0 reading as
//!   the call gave it, every field compared, and the charts of every
//!   chartsheet `kept_charts` plus one; a failure is exit 2 with the
//!   message and no output, and so is a refusal by the layer (an input
//!   chartsheet whose charts cannot be read, "no such part" for one the
//!   workbook's relationship names and the archive lacks).
//! - `--commentbox-sample`: the comment box sample of
//!   `specs/0028-images-and-comments-second-half.md`, "Harness runs": on W,
//!   with M as `--table-sample` has it, a comment by `emend` with the text
//!   `emend box` on (M + 1, 1), then its box set to 200 by 100 pixels,
//!   shown. The output is reopened. One line: the word `commentbox` then
//!   `comments_before` and `comments_after` (the entries `Package::comments`
//!   reads from W), `shapes_before` and `shapes_after` (the note shapes of
//!   W's VML part, by this tool's own pass), `sized_before` (the input's
//!   entries whose box reads `Some`), `box_equal` (1 when the added entry's
//!   box reads back equal to the box set) and `visible_after` (1 when it
//!   reads back shown). COUNTS ONLY: the author, the text and the box are
//!   this tool's constants and no reference is printed. Asserted:
//!   `comments_after` and `shapes_after` one more than before, `box_equal`
//!   and `visible_after` 1; a failure is exit 2 with the message and no
//!   output, and so is a refusal by the layer (M + 1 past the grid, a VML
//!   part it cannot read).
//! - `--anchor-sample`: the anchor sample of
//!   `specs/0028-images-and-comments-second-half.md`, "Harness runs": on W,
//!   with M as `--table-sample` has it, the tool's PNG on a two-cell anchor
//!   from (M + 1, 1) to (M + 3, 3), zero offsets, `editAs` `oneCell`, at its
//!   own size, and on an absolute anchor at (96, 96) pixels sized 4 by 4.
//!   The output is reopened. One line: the word `anchor` then
//!   `images_before` and `images_after` (the entries `Package::images`
//!   reads from W), `two_after` and `absolute_after` (those of each anchor
//!   kind), `anchors_before` and `anchors_after` (the root children of W's
//!   drawing part, by this tool's own pass), `media_before` and
//!   `media_after` (the archive's entries under `xl/media/`). COUNTS ONLY.
//!   Asserted: `images_after`, `anchors_after` and `media_after` two more
//!   than before, `two_after` and `absolute_after` each one more than the
//!   input's; a failure is exit 2 with the message and no output, and so is
//!   a refusal by the layer (M + 3 past the grid, a drawing it cannot read).
//! - `--remove-image-sample`: the image removal sample of
//!   `specs/0028-images-and-comments-second-half.md`, "Harness runs": on W,
//!   with M as `--table-sample` has it, the tool's PNG on (M + 1, 1), the
//!   write going into memory and that reopened; then `remove_image` of the
//!   last index (the addition) and, when the input's `images(W)` is not
//!   empty, of index 0 too; the output written and reopened. One line: the
//!   word `removeimage` then `images_before` and `images_after`,
//!   `kept_removed` (1 when the input had an image), `media_before`,
//!   `media_after`, `media_dropped` (`media_before - media_after`),
//!   `drawing_dropped` (1 when neither the input nor the output has a
//!   drawing for W), `anchors_before` and `anchors_after`. COUNTS ONLY.
//!   Asserted: `images_after` is `images_before - kept_removed`;
//!   `media_dropped` is 0 when `kept_removed` is 0, else the count of media
//!   entries the removed picture's root child names through `r:embed` or
//!   `r:link`, by relationships no other root child of the drawing names,
//!   that no other relationship of any rels part of the input targets, by
//!   this tool's own pass; `drawing_dropped` is 1 when the input had no
//!   drawing; a failure is exit 2 with the message and no output, and so is
//!   a refusal by the layer (M + 1 past the grid, a drawing or a media entry
//!   it cannot read).
//! - `--copy-sample`: the sheet copy sample of
//!   `specs/0028-images-and-comments-second-half.md`, "Harness runs": S, the
//!   first worksheet in `<sheets>` order with a `comments` relationship, or
//!   W when none has one, copied as `emend copy`. The output is reopened.
//!   One line: the word `copy` then `sheets_before` and `sheets_after` (the
//!   workbook's sheets), `cells_source` and `cells_copy` (the cells
//!   `Package::cells` reads from S in the input and from the copy in the
//!   output), `comments_source` and `comments_copy` (the entries
//!   `Package::comments` reads), `merges_source`, `merges_copy`,
//!   `links_source` and `links_copy` (the merges and hyperlinks
//!   `Package::sheet_settings` reads), `formulas` (1 when a cell of S has a
//!   formula), `chain_before` and `chain_after` (1 when the workbook has a
//!   calculation chain). COUNTS ONLY: the copy's name is this tool's
//!   constant and no name or reference is printed. Asserted: `sheets_after`
//!   one more than before, each copy count equal to its source's, and
//!   `chain_after` 0 when `formulas` is 1; a failure is exit 2 with the
//!   message and no output, and so is a refusal by the layer (the name in
//!   use, a source the copy cannot carry).
//!
//! The entry's bytes go to a temporary file and are handed to the layer as a
//! reader, through the `io::copy` wrapper, rather than being collected into
//! memory: a corpus entry can be tens of megabytes and `usize` is 32 bits on
//! the target that matters.
//!
//! Exit 0 on success, 2 on any error, with the message on stderr as
//! `error: ...`, matching `xlsx-diff` and `passthrough`. Messages name at
//! most an entry index: never a path, never an entry's content.

use std::env;
use std::ffi::OsString;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::{self, ExitCode};

use emend::archive::{Archive, Edits, Error as LayerError, replacement_from_reader};
use emend::cells::{Area, CellKind, CellRef, FormulaKind, RichText, Run, Value};
use emend::dates::{DateTime, Epoch};
use emend::package::{
    Anchor, COMMENTS, CONTENT_TYPES_PART, CORE_PROPERTIES, Cfvo, CfvoKind, Comment, CommentBox,
    CommentEntry, DRAWING, EditAs, ErrorStyle, HYPERLINK, HeaderFooter, IconSetKind, Image,
    ImageAnchor, LinkTarget, MAIN, Operator, Orientation, PIVOT_TABLE, Package, PageMargins,
    PageSetup, PrintOptions, Properties, Property, RELATIONSHIPS_ATTRIBUTE, Rule, SheetKind,
    SheetParts, SheetProtection, SheetView, TABLE, Table, TableStyle, TableStyleInfo, VML_DRAWING,
    Validation, ValidationKind, legacy_password_hash, rels_part_name, rels_part_source,
};
use emend::styles::{
    Alignment, Border, BorderStyle, Color, Fill, Font, Horizontal, NumberFormat, Pattern,
    Protection, Side, Style, Vertical,
};
use emend::xml::{BYTE_ORDER_MARK, Stream};

const USAGE: &str = "usage: emend-copy <input.xlsx> <output.xlsx> \
[--replace <name> | --xml-null-rewrite | --survey | --drop-calc-chain \
| --shared-strings | --cells | --edit-sample | --edit-formula-sample | --styles \
| --style-sample | --date-sample | --calc | --formula-sample | --new-sample | --sheet-sample \
| --remove-sample | --font-sample | --border-sample | --format-sample | --insert-rows-sample \
| --delete-rows-sample | --insert-columns-sample | --delete-columns-sample | --names-sample \
| --link-sample | --props-sample | --protect-sample | --validate-sample | --cf-sample | --scale-sample \
| --table-sample | --comment-sample | --image-sample | --richtext-sample \
| --richcomment-sample | --sheetview-sample | --pagesetup-sample | --printarea-sample \
| --group-sample | --linestyle-sample | --roundtrip-sample | --read-sample | --items \
| --namedstyles-sample | --colors-sample | --commentbox-sample | --anchor-sample \
| --remove-image-sample | --copy-sample | --kinds-sample | --pivot-sample | --extlink-sample \
| --charts-sample | --add-chart-sample | --chart-kinds-sample | --chart-more-sample \
| --chart-3d-sample | --chart-labels-sample | --remove-chart-sample | --chartsheet-sample \
| --chart-surface-sample]";

/// The one operation an invocation may carry, chosen by the arguments after
/// the two paths. At most one: the parser refuses two at once, an unknown
/// flag, and a flag before the paths, each with the usage line.
enum Operation {
    /// No edits: a verbatim round trip.
    None,
    /// Replace one named entry with its own bytes.
    Replace(Vec<u8>),
    /// Replace every XML-named entry with its own bytes through the stream.
    XmlNullRewrite,
    /// No edits, and count the package layer's map on stdout.
    Survey,
    /// Drop the calculation chain, the phase's first real edit.
    DropCalcChain,
    /// No edits, and count the shared strings table on stdout.
    SharedStrings,
    /// No edits, and count every cell of every sheet on stdout.
    Cells,
    /// No edits, and count what the typed reads give on stdout.
    ReadSample,
    /// No edits, and count the shared strings table walked item by item.
    Items,
    /// The sample cell edit, and its counts on stdout.
    EditSample,
    /// The formula sample edit, and its counts on stdout.
    EditFormulaSample,
    /// No edits, and count the styles part on stdout.
    Styles,
    /// The sample style edit, and its counts on stdout.
    StyleSample,
    /// The sample font and fill, and their counts on stdout.
    FontSample,
    /// The sample border, alignment and protection, and their counts.
    BorderSample,
    /// The sample dated cell, and its counts on stdout.
    DateSample,
    /// No edits, and the workbook's two calculation flags on stdout.
    Calc,
    /// The formula sample of part 6, and its counts on stdout.
    FormulaSample,
    /// A new workbook written to the first path, the sample cells to the
    /// second, and its counts on stdout.
    NewSample,
    /// The sheet sample, and its counts on stdout.
    SheetSample,
    /// The removal sample, and its counts on stdout.
    RemoveSample,
    /// The sample column and row formats, pane and merge, and their counts.
    FormatSample,
    /// Two rows inserted at the sample point, and the counts.
    InsertRowsSample,
    /// One row deleted at the sample point, and the counts.
    DeleteRowsSample,
    /// One column inserted at the sample point, and the counts.
    InsertColumnsSample,
    /// One column deleted at the sample point, and the counts.
    DeleteColumnsSample,
    /// Two defined names added and one removed again, and the counts.
    NamesSample,
    /// A cell and two hyperlinks past the last row, and the counts.
    LinkSample,
    /// Three core properties set, and the counts.
    PropsSample,
    /// The first worksheet and the workbook protected, and the counts.
    ProtectSample,
    /// Two data validations past the last row, one removed again, and the
    /// counts.
    ValidateSample,
    /// A differential style and two conditional formats past the last row,
    /// one removed again, and the counts.
    CfSample,
    /// A colour scale, a data bar and an icon set past the last row, and
    /// the counts.
    ScaleSample,
    /// A table past the last row, and the counts.
    TableSample,
    /// A comment past the last row, and the counts.
    CommentSample,
    /// An image past the last row, and the counts.
    ImageSample,
    /// A rich value past the last row, and the counts.
    RichtextSample,
    /// A comment of runs past the last row, and the counts.
    RichcommentSample,
    /// A tab colour and a sheet view, and the counts.
    SheetviewSample,
    /// Print options, margins, a page setup and a header and footer, and
    /// the counts.
    PagesetupSample,
    /// A print area, print titles and an autofilter, and the counts.
    PrintareaSample,
    /// Rows and columns grouped, a row collapsed and the outline summary,
    /// and the counts.
    GroupSample,
    /// A style on a row and a column, and the counts.
    LinestyleSample,
    /// Every cell format W uses read back typed and added again, and the
    /// counts.
    RoundtripSample,
    /// Every named and differential style read, the differential styles
    /// written back, and the counts.
    NamedstylesSample,
    /// Every colour of every font, fill, border and differential style
    /// resolved, the input written back unedited, and the counts.
    ColorsSample,
    /// Every part kind counted and every sheet's parts summed, the input
    /// written back unedited, and the counts.
    KindsSample,
    /// Every pivot table read with its cache, the input written back
    /// unedited, and the counts and digests.
    PivotSample,
    /// Every external link read, the input written back unedited, and the
    /// counts and digests.
    ExtlinkSample,
    /// Every worksheet's charts read, the input written back unedited, and
    /// the counts and digests.
    ChartsSample,
    /// A column chart past the last row of W, and the counts.
    AddChartSample,
    /// One chart of each kind past the last row of W, and the counts.
    ChartKindsSample,
    /// A doughnut, a radar, a bubble and a stock chart past the last row of
    /// W, and the counts.
    ChartMoreSample,
    /// Four 3D charts and a combined chart past the last row of W, and the
    /// counts.
    Chart3dSample,
    /// Two surface charts and two of-pie charts past the last row of W, and
    /// the counts.
    ChartSurfaceSample,
    /// A column chart and a pie with colours, labels and axis formats past
    /// the last row of W, and the counts.
    ChartLabelsSample,
    /// A chart added and removed again, the first kept one removed, and
    /// the counts.
    RemoveChartSample,
    /// A chartsheet holding a column chart at the end of the list, and the
    /// counts.
    ChartsheetSample,
    /// A comment with a shown box past the last row, and the counts.
    CommentboxSample,
    /// The first worksheet with comments, or W, copied, and the counts.
    CopySample,
    /// An image on a two-cell and one on an absolute anchor, and the counts.
    AnchorSample,
    /// An image added and removed again, the first kept one removed, and
    /// the counts.
    RemoveImageSample,
}

/// Exit status for every error, as in `xlsx-diff` and `passthrough`.
const EXIT_ERROR: u8 = 2;

/// Everything that stops a copy.
enum Error {
    Usage,
    ReadInput(io::ErrorKind),
    ReadOutput(io::ErrorKind),
    Layer(LayerError),
    NoFileName,
    Create(io::ErrorKind),
    Finish(io::ErrorKind),
    Rename(io::ErrorKind),
    NoSuchName,
    Extract(io::ErrorKind),
    Rewrite(io::ErrorKind),
    Report(io::ErrorKind),
    NoWorksheet,
    NoNewRow,
    CellsAfter { expected: u64, found: u64 },
    XfsAfter { expected: u64, found: u64 },
    ReadBack { expected: u64, found: u64 },
    NewReadBack { expected: u64, found: u64 },
    SheetSample(&'static str),
    NoFreeName,
    RemoveSample(&'static str),
    NamesSample(&'static str),
    LinkSample(&'static str),
    PropsSample(&'static str),
    ProtectSample(&'static str),
    ValidateSample(&'static str),
    CfSample(&'static str),
    ScaleSample(&'static str),
    TableSample(&'static str),
    CommentSample(&'static str),
    ImageSample(&'static str),
    RichtextSample(&'static str),
    RichcommentSample(&'static str),
    SheetviewSample(&'static str),
    PagesetupSample(&'static str),
    PrintareaSample(&'static str),
    GroupSample(&'static str),
    LinestyleSample(&'static str),
    RoundtripSample(&'static str),
    NamedstylesSample(&'static str),
    ColorsSample(&'static str),
    KindsSample(&'static str),
    PivotSample(&'static str),
    ExtlinkSample(&'static str),
    ChartsSample(&'static str),
    AddChartSample(&'static str),
    ChartKindsSample(&'static str),
    ChartMoreSample(&'static str),
    Chart3dSample(&'static str),
    ChartSurfaceSample(&'static str),
    ChartLabelsSample(&'static str),
    RemoveChartSample(&'static str),
    ChartsheetSample(&'static str),
    ReadSample(&'static str),
    CommentboxSample(&'static str),
    CopySample(&'static str),
    AnchorSample(&'static str),
    RemoveImageSample(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Usage => f.write_str(USAGE),
            Error::ReadInput(kind) => write!(f, "cannot read the input file: {kind}"),
            Error::ReadOutput(kind) => write!(f, "cannot read the output file: {kind}"),
            Error::Layer(e) => write!(f, "{e}"),
            Error::NoFileName => f.write_str("the output path has no file name"),
            Error::Create(kind) => write!(f, "cannot create the output file: {kind}"),
            Error::Finish(kind) => write!(f, "cannot finish the output file: {kind}"),
            Error::Rename(kind) => write!(f, "cannot move the output into place: {kind}"),
            // The name is NOT echoed: it comes from the command line, but the
            // rule for this tool is that a message names at most an index.
            Error::NoSuchName => f.write_str("the archive has no entry with that name"),
            Error::Extract(kind) => write!(f, "cannot extract the entry to replace: {kind}"),
            Error::Rewrite(kind) => write!(f, "cannot write the rewritten part: {kind}"),
            Error::Report(kind) => write!(f, "cannot write the report line: {kind}"),
            Error::NoWorksheet => f.write_str("the workbook has no sheet of kind Worksheet"),
            Error::NoNewRow => f.write_str("the worksheet's last row is the last row there is"),
            Error::CellsAfter { expected, found } => write!(
                f,
                "the output's sheet holds {found} cell(s) and the edits imply {expected}"
            ),
            Error::XfsAfter { expected, found } => write!(
                f,
                "the output's styles part holds {found} cell format(s) and the edits imply \
                 {expected}"
            ),
            Error::ReadBack { expected, found } => write!(
                f,
                "the output's sheet reads back {found} written formula(s) and the edits imply \
                 {expected}"
            ),
            Error::SheetSample(what) => {
                write!(f, "the output's {what} is not what the edits imply")
            }
            Error::NoFreeName => f.write_str("no free sheet name of the sample's form"),
            Error::RemoveSample(what) => {
                write!(f, "the output's {what} is not what the removal implies")
            }
            Error::NamesSample(what) => {
                write!(f, "the output's {what} is not what the names imply")
            }
            Error::LinkSample(what) => {
                write!(f, "the output's {what} is not what the links imply")
            }
            Error::PropsSample(what) => {
                write!(f, "the output's {what} is not what the properties imply")
            }
            Error::ProtectSample(what) => {
                write!(f, "the output's {what} is not what the protection implies")
            }
            Error::ValidateSample(what) => {
                write!(f, "the output's {what} is not what the validations imply")
            }
            Error::CfSample(what) => {
                write!(
                    f,
                    "the output's {what} is not what the conditional formats imply"
                )
            }
            Error::ScaleSample(what) => {
                write!(f, "the output's {what} is not what the visual rules imply")
            }
            Error::TableSample(what) => {
                write!(f, "the output's {what} is not what the table implies")
            }
            Error::CommentSample(what) => {
                write!(f, "the output's {what} is not what the comment implies")
            }
            Error::ImageSample(what) => {
                write!(f, "the output's {what} is not what the image implies")
            }
            Error::RichtextSample(what) => {
                write!(f, "the output's {what} is not what the rich text implies")
            }
            Error::RichcommentSample(what) => {
                write!(
                    f,
                    "the output's {what} is not what the rich comment implies"
                )
            }
            Error::SheetviewSample(what) => {
                write!(f, "the output's {what} is not what the sheet view implies")
            }
            Error::PagesetupSample(what) => {
                write!(f, "the output's {what} is not what the page setup implies")
            }
            Error::PrintareaSample(what) => {
                write!(f, "the output's {what} is not what the print area implies")
            }
            Error::GroupSample(what) => {
                write!(f, "the output's {what} is not what the groups imply")
            }
            Error::LinestyleSample(what) => {
                write!(f, "the output's {what} is not what the line styles imply")
            }
            Error::RoundtripSample(what) => {
                write!(f, "the output's {what} is not what the round trip implies")
            }
            Error::NamedstylesSample(what) => {
                write!(f, "the {what} is not what the named styles imply")
            }
            Error::ColorsSample(what) => {
                write!(f, "the input's {what} is not what the colours imply")
            }
            Error::KindsSample(what) => {
                write!(f, "the input's {what} is not what the survey implies")
            }
            Error::PivotSample(what) => {
                write!(f, "the input's {what} is not what the pivot tables imply")
            }
            Error::ExtlinkSample(what) => {
                write!(f, "the input's {what} is not what the external links imply")
            }
            Error::ChartsSample(what) => {
                write!(f, "the input's {what} is not what the charts imply")
            }
            Error::AddChartSample(what) | Error::ChartsheetSample(what) => {
                write!(f, "the output's {what} is not what the chart implies")
            }
            Error::ChartKindsSample(what)
            | Error::ChartMoreSample(what)
            | Error::Chart3dSample(what)
            | Error::ChartSurfaceSample(what)
            | Error::ChartLabelsSample(what) => {
                write!(f, "the output's {what} is not what the charts imply")
            }
            Error::ReadSample(what) => {
                write!(f, "the input's {what} is not what the reads imply")
            }
            Error::AnchorSample(what) => {
                write!(f, "the output's {what} is not what the anchors imply")
            }
            Error::RemoveImageSample(what) | Error::RemoveChartSample(what) => {
                write!(f, "the output's {what} is not what the removal implies")
            }
            Error::CommentboxSample(what) => {
                write!(f, "the output's {what} is not what the comment box implies")
            }
            Error::CopySample(what) => {
                write!(f, "the output's {what} is not what the sheet copy implies")
            }
            Error::NewReadBack { expected, found } => write!(
                f,
                "the output's sheet reads back {found} written value(s) and the edits imply \
                 {expected}"
            ),
        }
    }
}

impl From<LayerError> for Error {
    fn from(e: LayerError) -> Self {
        Error::Layer(e)
    }
}

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();
    // The operation comes after the two paths, and there is at most one. Any
    // other shape is the usage error: two operations at once and an operation
    // before the paths both leave a flag where a path or nothing belongs.
    let result = match args.as_slice() {
        [input, output] => run(Path::new(input), Path::new(output), Operation::None),
        [input, output, flag] if flag == "--xml-null-rewrite" => run(
            Path::new(input),
            Path::new(output),
            Operation::XmlNullRewrite,
        ),
        [input, output, flag] if flag == "--survey" => {
            run(Path::new(input), Path::new(output), Operation::Survey)
        }
        [input, output, flag] if flag == "--shared-strings" => run(
            Path::new(input),
            Path::new(output),
            Operation::SharedStrings,
        ),
        [input, output, flag] if flag == "--cells" => {
            run(Path::new(input), Path::new(output), Operation::Cells)
        }
        [input, output, flag] if flag == "--read-sample" => {
            run(Path::new(input), Path::new(output), Operation::ReadSample)
        }
        [input, output, flag] if flag == "--items" => {
            run(Path::new(input), Path::new(output), Operation::Items)
        }
        [input, output, flag] if flag == "--edit-sample" => {
            run(Path::new(input), Path::new(output), Operation::EditSample)
        }
        [input, output, flag] if flag == "--edit-formula-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::EditFormulaSample,
        ),
        [input, output, flag] if flag == "--styles" => {
            run(Path::new(input), Path::new(output), Operation::Styles)
        }
        [input, output, flag] if flag == "--style-sample" => {
            run(Path::new(input), Path::new(output), Operation::StyleSample)
        }
        [input, output, flag] if flag == "--font-sample" => {
            run(Path::new(input), Path::new(output), Operation::FontSample)
        }
        [input, output, flag] if flag == "--border-sample" => {
            run(Path::new(input), Path::new(output), Operation::BorderSample)
        }
        [input, output, flag] if flag == "--format-sample" => {
            run(Path::new(input), Path::new(output), Operation::FormatSample)
        }
        [input, output, flag] if flag == "--insert-rows-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::InsertRowsSample,
        ),
        [input, output, flag] if flag == "--delete-rows-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::DeleteRowsSample,
        ),
        [input, output, flag] if flag == "--insert-columns-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::InsertColumnsSample,
        ),
        [input, output, flag] if flag == "--delete-columns-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::DeleteColumnsSample,
        ),
        [input, output, flag] if flag == "--names-sample" => {
            run(Path::new(input), Path::new(output), Operation::NamesSample)
        }
        [input, output, flag] if flag == "--link-sample" => {
            run(Path::new(input), Path::new(output), Operation::LinkSample)
        }
        [input, output, flag] if flag == "--props-sample" => {
            run(Path::new(input), Path::new(output), Operation::PropsSample)
        }
        [input, output, flag] if flag == "--protect-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::ProtectSample,
        ),
        [input, output, flag] if flag == "--validate-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::ValidateSample,
        ),
        [input, output, flag] if flag == "--scale-sample" => {
            run(Path::new(input), Path::new(output), Operation::ScaleSample)
        }
        [input, output, flag] if flag == "--table-sample" => {
            run(Path::new(input), Path::new(output), Operation::TableSample)
        }
        [input, output, flag] if flag == "--comment-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::CommentSample,
        ),
        [input, output, flag] if flag == "--image-sample" => {
            run(Path::new(input), Path::new(output), Operation::ImageSample)
        }
        [input, output, flag] if flag == "--richtext-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::RichtextSample,
        ),
        [input, output, flag] if flag == "--richcomment-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::RichcommentSample,
        ),
        [input, output, flag] if flag == "--sheetview-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::SheetviewSample,
        ),
        [input, output, flag] if flag == "--pagesetup-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::PagesetupSample,
        ),
        [input, output, flag] if flag == "--printarea-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::PrintareaSample,
        ),
        [input, output, flag] if flag == "--group-sample" => {
            run(Path::new(input), Path::new(output), Operation::GroupSample)
        }
        [input, output, flag] if flag == "--linestyle-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::LinestyleSample,
        ),
        [input, output, flag] if flag == "--anchor-sample" => {
            run(Path::new(input), Path::new(output), Operation::AnchorSample)
        }
        [input, output, flag] if flag == "--remove-image-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::RemoveImageSample,
        ),
        [input, output, flag] if flag == "--commentbox-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::CommentboxSample,
        ),
        [input, output, flag] if flag == "--copy-sample" => {
            run(Path::new(input), Path::new(output), Operation::CopySample)
        }
        [input, output, flag] if flag == "--namedstyles-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::NamedstylesSample,
        ),
        [input, output, flag] if flag == "--colors-sample" => {
            run(Path::new(input), Path::new(output), Operation::ColorsSample)
        }
        [input, output, flag] if flag == "--kinds-sample" => {
            run(Path::new(input), Path::new(output), Operation::KindsSample)
        }
        [input, output, flag] if flag == "--pivot-sample" => {
            run(Path::new(input), Path::new(output), Operation::PivotSample)
        }
        [input, output, flag] if flag == "--extlink-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::ExtlinkSample,
        ),
        [input, output, flag] if flag == "--charts-sample" => {
            run(Path::new(input), Path::new(output), Operation::ChartsSample)
        }
        [input, output, flag] if flag == "--add-chart-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::AddChartSample,
        ),
        [input, output, flag] if flag == "--chart-kinds-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::ChartKindsSample,
        ),
        [input, output, flag] if flag == "--chart-more-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::ChartMoreSample,
        ),
        [input, output, flag] if flag == "--chart-3d-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::Chart3dSample,
        ),
        [input, output, flag] if flag == "--chart-surface-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::ChartSurfaceSample,
        ),
        [input, output, flag] if flag == "--chart-labels-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::ChartLabelsSample,
        ),
        [input, output, flag] if flag == "--remove-chart-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::RemoveChartSample,
        ),
        [input, output, flag] if flag == "--chartsheet-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::ChartsheetSample,
        ),
        [input, output, flag] if flag == "--roundtrip-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::RoundtripSample,
        ),
        [input, output, flag] if flag == "--cf-sample" => {
            run(Path::new(input), Path::new(output), Operation::CfSample)
        }
        [input, output, flag] if flag == "--date-sample" => {
            run(Path::new(input), Path::new(output), Operation::DateSample)
        }
        [input, output, flag] if flag == "--calc" => {
            run(Path::new(input), Path::new(output), Operation::Calc)
        }
        [input, output, flag] if flag == "--formula-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::FormulaSample,
        ),
        [input, output, flag] if flag == "--sheet-sample" => {
            run(Path::new(input), Path::new(output), Operation::SheetSample)
        }
        [input, output, flag] if flag == "--remove-sample" => {
            run(Path::new(input), Path::new(output), Operation::RemoveSample)
        }
        [input, output, flag] if flag == "--new-sample" => {
            run(Path::new(input), Path::new(output), Operation::NewSample)
        }
        [input, output, flag] if flag == "--drop-calc-chain" => run(
            Path::new(input),
            Path::new(output),
            Operation::DropCalcChain,
        ),
        [input, output, flag, name] if flag == "--replace" => run(
            Path::new(input),
            Path::new(output),
            Operation::Replace(name.as_bytes().to_vec()),
        ),
        _ => Err(Error::Usage),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let _ = writeln!(io::stderr().lock(), "error: {err}");
            ExitCode::from(EXIT_ERROR)
        }
    }
}

fn run(input: &Path, output: &Path, operation: Operation) -> Result<(), Error> {
    // The recipe records its edits on a `Package` and writes through it, so it
    // does not share the `Edits` path the other operations build.
    if matches!(operation, Operation::DropCalcChain) {
        return drop_calc_chain(input, output);
    }
    if matches!(operation, Operation::EditSample) {
        return edit_sample(input, output);
    }
    if matches!(operation, Operation::EditFormulaSample) {
        return edit_formula_sample(input, output);
    }
    if matches!(operation, Operation::StyleSample) {
        return style_sample(input, output);
    }
    if matches!(operation, Operation::FontSample) {
        return font_sample(input, output);
    }
    if matches!(operation, Operation::BorderSample) {
        return border_sample(input, output);
    }
    if matches!(operation, Operation::DateSample) {
        return date_sample(input, output);
    }
    if matches!(operation, Operation::FormulaSample) {
        return formula_sample(input, output);
    }
    if matches!(operation, Operation::NewSample) {
        return new_sample(input, output);
    }
    if matches!(operation, Operation::SheetSample) {
        return sheet_sample(input, output);
    }
    if matches!(operation, Operation::RemoveSample) {
        return remove_sample(input, output);
    }
    if matches!(operation, Operation::FormatSample) {
        return format_sample(input, output);
    }
    if matches!(operation, Operation::InsertRowsSample) {
        return shift_sample(input, output, true, false);
    }
    if matches!(operation, Operation::DeleteRowsSample) {
        return shift_sample(input, output, false, false);
    }
    if matches!(operation, Operation::InsertColumnsSample) {
        return shift_sample(input, output, true, true);
    }
    if matches!(operation, Operation::DeleteColumnsSample) {
        return shift_sample(input, output, false, true);
    }
    if matches!(operation, Operation::NamesSample) {
        return names_sample(input, output);
    }
    if matches!(operation, Operation::LinkSample) {
        return link_sample(input, output);
    }
    if matches!(operation, Operation::PropsSample) {
        return props_sample(input, output);
    }
    if matches!(operation, Operation::ProtectSample) {
        return protect_sample(input, output);
    }
    if matches!(operation, Operation::ValidateSample) {
        return validate_sample(input, output);
    }
    if matches!(operation, Operation::CfSample) {
        return cf_sample(input, output);
    }
    if matches!(operation, Operation::ScaleSample) {
        return scale_sample(input, output);
    }
    if matches!(operation, Operation::TableSample) {
        return table_sample(input, output);
    }
    if matches!(operation, Operation::CommentSample) {
        return comment_sample(input, output);
    }
    if matches!(operation, Operation::ImageSample) {
        return image_sample(input, output);
    }
    if matches!(operation, Operation::RichtextSample) {
        return richtext_sample(input, output);
    }
    if matches!(operation, Operation::RichcommentSample) {
        return richcomment_sample(input, output);
    }
    if matches!(operation, Operation::SheetviewSample) {
        return sheetview_sample(input, output);
    }
    if matches!(operation, Operation::PagesetupSample) {
        return pagesetup_sample(input, output);
    }
    if matches!(operation, Operation::PrintareaSample) {
        return printarea_sample(input, output);
    }
    if matches!(operation, Operation::GroupSample) {
        return group_sample(input, output);
    }
    if matches!(operation, Operation::LinestyleSample) {
        return linestyle_sample(input, output);
    }
    if matches!(operation, Operation::RoundtripSample) {
        return roundtrip_sample(input, output);
    }
    if matches!(operation, Operation::NamedstylesSample) {
        return namedstyles_sample(input, output);
    }
    if matches!(operation, Operation::ColorsSample) {
        return colors_sample(input, output);
    }
    if matches!(operation, Operation::KindsSample) {
        return kinds_sample(input, output);
    }
    if matches!(operation, Operation::PivotSample) {
        return pivot_sample(input, output);
    }
    if matches!(operation, Operation::ExtlinkSample) {
        return extlink_sample(input, output);
    }
    if matches!(operation, Operation::ChartsSample) {
        return charts_sample(input, output);
    }
    if matches!(operation, Operation::AddChartSample) {
        return add_chart_sample(input, output);
    }
    if matches!(operation, Operation::ChartKindsSample) {
        return chart_kinds_sample(input, output);
    }
    if matches!(operation, Operation::ChartMoreSample) {
        return chart_more_sample(input, output);
    }
    if matches!(operation, Operation::Chart3dSample) {
        return chart_3d_sample(input, output);
    }
    if matches!(operation, Operation::ChartSurfaceSample) {
        return chart_surface_sample(input, output);
    }
    if matches!(operation, Operation::ChartLabelsSample) {
        return chart_labels_sample(input, output);
    }
    if matches!(operation, Operation::RemoveChartSample) {
        return remove_chart_sample(input, output);
    }
    if matches!(operation, Operation::ChartsheetSample) {
        return chartsheet_sample(input, output);
    }
    if matches!(operation, Operation::CommentboxSample) {
        return commentbox_sample(input, output);
    }
    if matches!(operation, Operation::CopySample) {
        return copy_sample(input, output);
    }
    if matches!(operation, Operation::AnchorSample) {
        return anchor_sample(input, output);
    }
    if matches!(operation, Operation::RemoveImageSample) {
        return remove_image_sample(input, output);
    }

    // Both reading operations run first, so a package the layer cannot read
    // stops the run before any output file exists. Each yields the one line
    // it prints, so the reporting below is the same code for either.
    let reported = match operation {
        Operation::Survey => Some(survey(input)?.line()),
        Operation::SharedStrings => Some(shared_strings_line(input)?),
        Operation::Cells => Some(cells_line(input)?),
        Operation::ReadSample => Some(read_line(input)?),
        Operation::Items => Some(items_line(input)?),
        Operation::Styles => Some(styles_line(input)?),
        Operation::Calc => Some(calc_line(input)?),
        _ => None,
    };

    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut archive = Archive::open(BufReader::new(file))?;

    // Every replacement's bytes go to a temporary file first: the layer takes
    // a reader for the whole write, and holding one that borrows the archive
    // would conflict with it.
    let mut edits = Edits::new();
    let extracted = match operation {
        // Handled above, before any output file exists.
        Operation::DropCalcChain => unreachable!("the recipe has its own path"),
        Operation::EditSample => unreachable!("the sample edit has its own path"),
        Operation::EditFormulaSample => unreachable!("the formula sample has its own path"),
        Operation::StyleSample => unreachable!("the style sample has its own path"),
        Operation::FontSample => unreachable!("the font sample has its own path"),
        Operation::BorderSample => unreachable!("the border sample has its own path"),
        Operation::DateSample => unreachable!("the date sample has its own path"),
        Operation::FormulaSample => unreachable!("the part 6 formula sample has its own path"),
        Operation::NewSample => unreachable!("the new workbook sample has its own path"),
        Operation::SheetSample => unreachable!("the sheet sample has its own path"),
        Operation::RemoveSample => unreachable!("the removal sample has its own path"),
        Operation::FormatSample => unreachable!("the format sample has its own path"),
        Operation::InsertRowsSample
        | Operation::DeleteRowsSample
        | Operation::InsertColumnsSample
        | Operation::DeleteColumnsSample => {
            unreachable!("the row and column samples have their own path")
        }
        Operation::NamesSample => unreachable!("the names sample has its own path"),
        Operation::LinkSample => unreachable!("the link sample has its own path"),
        Operation::PropsSample => unreachable!("the properties sample has its own path"),
        Operation::ProtectSample => unreachable!("the protection sample has its own path"),
        Operation::ValidateSample => unreachable!("the validation sample has its own path"),
        Operation::CfSample => unreachable!("the conditional format sample has its own path"),
        Operation::ScaleSample => unreachable!("the visual rule sample has its own path"),
        Operation::TableSample => unreachable!("the table sample has its own path"),
        Operation::CommentSample => unreachable!("the comment sample has its own path"),
        Operation::ImageSample => unreachable!("the image sample has its own path"),
        Operation::RichtextSample => unreachable!("the rich text sample has its own path"),
        Operation::RichcommentSample => unreachable!("the rich comment sample has its own path"),
        Operation::SheetviewSample => unreachable!("the sheet view sample has its own path"),
        Operation::PagesetupSample => unreachable!("the page setup sample has its own path"),
        Operation::PrintareaSample => unreachable!("the print area sample has its own path"),
        Operation::GroupSample => unreachable!("the group sample has its own path"),
        Operation::LinestyleSample => unreachable!("the line style sample has its own path"),
        Operation::RoundtripSample => unreachable!("the round-trip sample has its own path"),
        Operation::NamedstylesSample => {
            unreachable!("the named styles sample has its own path")
        }
        Operation::ColorsSample => unreachable!("the colours sample has its own path"),
        Operation::KindsSample => unreachable!("the survey sample has its own path"),
        Operation::PivotSample => unreachable!("the pivot sample has its own path"),
        Operation::ExtlinkSample => unreachable!("the external link sample has its own path"),
        Operation::ChartsSample => unreachable!("the charts sample has its own path"),
        Operation::AddChartSample => unreachable!("the add-chart sample has its own path"),
        Operation::ChartKindsSample => unreachable!("the chart kinds sample has its own path"),
        Operation::ChartMoreSample => unreachable!("the more kinds sample has its own path"),
        Operation::Chart3dSample => unreachable!("the 3D sample has its own path"),
        Operation::ChartSurfaceSample => unreachable!("the surface sample has its own path"),
        Operation::ChartLabelsSample => unreachable!("the labels sample has its own path"),
        Operation::RemoveChartSample => unreachable!("the chart removal sample has its own path"),
        Operation::ChartsheetSample => unreachable!("the chartsheet sample has its own path"),
        Operation::CommentboxSample => unreachable!("the comment box sample has its own path"),
        Operation::CopySample => unreachable!("the sheet copy sample has its own path"),
        Operation::AnchorSample => unreachable!("the anchor sample has its own path"),
        Operation::RemoveImageSample => unreachable!("the image removal sample has its own path"),
        Operation::None
        | Operation::Survey
        | Operation::SharedStrings
        | Operation::Cells
        | Operation::ReadSample
        | Operation::Items
        | Operation::Styles
        | Operation::Calc => Vec::new(),
        Operation::Replace(name) => {
            let index = archive.index_of(&name).ok_or(Error::NoSuchName)?;
            let (mut file, guard) = temporary(output, "-entry")?;
            io::copy(&mut archive.read(index)?, &mut file).map_err(|e| Error::Extract(e.kind()))?;
            file.seek(SeekFrom::Start(0))
                .map_err(|e| Error::Extract(e.kind()))?;
            edits.replace(index, replacement_from_reader(file));
            vec![guard]
        }
        Operation::XmlNullRewrite => {
            let indices: Vec<usize> = archive
                .entries()
                .iter()
                .enumerate()
                .filter(|(_, entry)| is_xml_name(entry.name()))
                .map(|(index, _)| index)
                .collect();
            let mut guards = Vec::with_capacity(indices.len());
            for index in indices {
                // A separate temporary per entry: they are all handed to the
                // layer and read during the one write.
                let (mut file, guard) = temporary(output, &format!("-xml{index}"))?;
                guards.push(guard);
                rewrite_part(&mut archive, index, &mut file)?;
                file.seek(SeekFrom::Start(0))
                    .map_err(|e| Error::Rewrite(e.kind()))?;
                edits.replace(index, replacement_from_reader(file));
            }
            guards
        }
    };

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;

    // From here the temporary file is this run's own, so a failure removes it
    // and leaves no partial output where the harness would read one.
    let result = write(&mut archive, file, edits)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    drop(extracted);
    result?;

    if let Some(line) = reported {
        let mut out = io::stdout().lock();
        writeln!(out, "{line}").map_err(|e| Error::Report(e.kind()))?;
        out.flush().map_err(|e| Error::Report(e.kind()))?;
    }
    Ok(())
}

/// The survey's counts, in the order `specs/0009-package-layer.md`, "Harness
/// runs", lists its keys.
///
/// Every field is `u64`: a count is a count of entries, relationships or
/// sheets, and `usize` is 32 bits on the target that matters.
#[derive(Default)]
struct Survey {
    entries: u64,
    typed: u64,
    untyped: u64,
    defaults: u64,
    overrides: u64,
    overrides_unresolved: u64,
    rels_parts: u64,
    relationships: u64,
    external: u64,
    resolved: u64,
    unresolved: u64,
    case_only: u64,
    sheets: u64,
    sheets_resolved: u64,
    shared_strings: u64,
    styles: u64,
    calc_chain: u64,
    theme: u64,
}

impl Survey {
    /// The one line the tool prints: `survey` then tab-separated pairs.
    fn line(&self) -> String {
        let pairs = [
            ("entries", self.entries),
            ("typed", self.typed),
            ("untyped", self.untyped),
            ("defaults", self.defaults),
            ("overrides", self.overrides),
            ("overrides_unresolved", self.overrides_unresolved),
            ("rels_parts", self.rels_parts),
            ("relationships", self.relationships),
            ("external", self.external),
            ("resolved", self.resolved),
            ("unresolved", self.unresolved),
            ("case_only", self.case_only),
            ("sheets", self.sheets),
            ("sheets_resolved", self.sheets_resolved),
            ("shared_strings", self.shared_strings),
            ("styles", self.styles),
            ("calc_chain", self.calc_chain),
            ("theme", self.theme),
        ];
        let mut line = String::from("survey");
        for (key, value) in pairs {
            line.push('\t');
            line.push_str(key);
            line.push('=');
            line.push_str(&value.to_string());
        }
        line
    }
}

/// Drops the calculation chain, through the package layer.
///
/// [`Package::drop_calc_chain`] records the recipe of
/// `specs/0009-package-layer.md`, "Module `package`": the workbook's
/// `calcChain` relationship, the `Override` for its target when there is one,
/// the entry when there is one, each only when its subject is there. The
/// recipe lived here until `specs/0011-editing-cells.md` moved it into the
/// crate; the harness sees exactly the three changes it asks for, and a valid
/// package with no chain records nothing and comes back byte for byte, which
/// is the control for this operation.
///
/// The workbook and then the content types part are read before the chain is
/// looked for, so a package without a content types part is refused whether
/// or not it has a chain. Part 2 P3 measured the other order: with no chain
/// the recipe recorded no edit, `write_to` returned from `validate` at once,
/// and a package with no `[Content_Types].xml` was copied byte for byte while
/// `--survey` refused it. Planning ruled (part 2 P3 verification ruling 6)
/// that an operation which would refuse a package should refuse it whether or
/// not its edit turns out to be empty, so the two operations answer the same
/// question the same way. The method reads the content types part only when
/// there is a chain, so that read stays here.
fn drop_calc_chain(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    // Read for the refusals, in the order the recipe met them before it moved:
    // the answers are cached, so the method's own reads cost nothing extra.
    package.workbook()?;
    package.content_types()?;
    package.drop_calc_chain()?;

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`: a failure
    // removes it rather than leaving a partial output for the harness.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

/// What the first read of the sample sheet finds.
struct Sampled {
    /// A: the first `Number` or `SharedString` cell with a value and no
    /// formula.
    first: Option<CellRef>,
    /// A's `s` and its value text, which the style sample needs: the `s` as
    /// the base of its addition, the text to compare the read-back against.
    /// Neither reaches a line or a message.
    first_style: Option<u32>,
    first_value: Option<String>,
    /// B: the second such cell, and whether it carried `s`.
    second: Option<(CellRef, bool)>,
    /// M: the largest row number, 0 for a sheet with no cells.
    max_row: u32,
    cells: u64,
}

/// Reads one sheet through `Package::cells` for what the sample edit needs.
fn sample<R: io::Read + io::Seek>(package: &mut Package<R>, part: &[u8]) -> Result<Sampled, Error> {
    let mut sampled = Sampled {
        first: None,
        first_style: None,
        first_value: None,
        second: None,
        max_row: 0,
        cells: 0,
    };
    for cell in package.cells(part)? {
        let cell = cell?;
        sampled.cells += 1;
        sampled.max_row = sampled.max_row.max(cell.reference.row);
        let candidate = matches!(cell.kind, CellKind::Number | CellKind::SharedString)
            && cell.value.is_some()
            && cell.formula.is_none();
        if candidate && sampled.first.is_none() {
            sampled.first = Some(cell.reference);
            sampled.first_style = cell.style;
            sampled.first_value = cell.value.clone();
        } else if candidate && sampled.second.is_none() {
            sampled.second = Some((cell.reference, cell.style.is_some()));
        }
    }
    Ok(sampled)
}

/// The sample cell edit of `specs/0011-editing-cells.md`, "Harness runs".
///
/// The first sheet of kind `Worksheet` in workbook order is read once for A,
/// B and M ([`Sampled`]); A is set to `Number(42.5)`, B cleared, `(M + 1, 1)`
/// set to `Text("emend")` and `(M + 1, 2)` to `Boolean(true)`, and the
/// package is written. The output is then opened and the same part read, for
/// `cells_after` and `read_back`. No reference, value or text reaches the
/// line or a message: a clear of a styled cell keeps it as `<c r s/>`, so
/// only an unstyled B is `dropped` and the count falls by it. A failure
/// after the output is in place, the count's assertion included, removes the
/// output, so a refused run leaves none.
fn edit_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let before = sample(&mut package, &part)?;
    let new_row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
    let text = CellRef {
        row: new_row,
        col: 1,
    };
    let boolean = CellRef {
        row: new_row,
        col: 2,
    };

    if let Some(first) = before.first {
        package.set_cell(&part, first, Value::Number(42.5));
    }
    if let Some((second, _)) = before.second {
        package.clear_cell(&part, second);
    }
    package.set_cell(&part, text, Value::Text("emend".to_string()));
    package.set_cell(&part, boolean, Value::Boolean(true));

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    let set = 2 + u64::from(before.first.is_some());
    let cleared = u64::from(before.second.is_some());
    let dropped = u64::from(matches!(before.second, Some((_, false))));
    let expected = before.cells + 2 - dropped;

    // The output is in place: a failure from here on removes it, as every
    // other refusal of the tool leaves no output (P2 verification ruling 3).
    let checked = read_back(output, &part, before.first, text, boolean, expected);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (cells_after, read_back) = checked?;
    edit_line(&[
        ("set", set),
        ("cleared", cleared),
        ("dropped", dropped),
        ("rows_added", 1),
        ("cells_before", before.cells),
        ("cells_after", cells_after),
        ("read_back", read_back),
    ])
}

/// Reopens the output and reads the edited sheet back: its cell count,
/// asserted equal to `expected`, the count the edits imply, and how many of
/// the three set values read back as written.
fn read_back(
    output: &Path,
    part: &[u8],
    first: Option<CellRef>,
    text: CellRef,
    boolean: CellRef,
    expected: u64,
) -> Result<(u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let mut cells_after = 0u64;
    let mut read_back = 0u64;
    for cell in written.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        let at = cell.reference;
        if Some(at) == first && cell.number() == Some(42.5) {
            read_back += 1;
        }
        if at == text
            && cell.kind == CellKind::InlineString
            && cell.value.as_deref() == Some("emend")
        {
            read_back += 1;
        }
        if at == boolean && cell.boolean() == Some(true) {
            read_back += 1;
        }
    }
    if cells_after != expected {
        return Err(Error::CellsAfter {
            expected,
            found: cells_after,
        });
    }
    Ok((cells_after, read_back))
}

/// The formula sample edit of `specs/0011-editing-cells.md`, "Harness runs".
///
/// The sheets of kind `Worksheet` are read in `<sheets>` order until one holds
/// a cell with a formula of kind `Normal`; that sheet is read to its end for
/// its count, the first such cell is set to `Number(1)` and the package
/// written, and the write's rule drops the calculation chain when the
/// workbook has one. The output is reopened and that sheet read back for the
/// line: `formula_replaced` (0 or 1), `chain_dropped` (1 when the input's
/// workbook names a chain and the output's does not, measured from both
/// reads), `cells_after` (the read-back count, asserted equal to the count
/// before: exit 2 when it differs) and `read_back` (1 when the edited cell
/// reads back as `number()` 1 with no formula). With no normal formula
/// nothing is recorded, so the output is byte-identical, and every key is 0:
/// a `cells_after` of 0 then means no sheet was read (P3 rulings 4 to 6). A
/// failure after the output is in place removes it, as `edit_sample`'s does.
fn edit_formula_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let chain_before = workbook.calc_chain.is_some();
    let parts: Vec<Vec<u8>> = workbook
        .sheets
        .iter()
        .filter(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .collect();
    let mut found = None;
    for part in &parts {
        let mut first = None;
        let mut cells = 0u64;
        for cell in package.cells(part)? {
            let cell = cell?;
            cells += 1;
            let normal = cell
                .formula
                .as_ref()
                .is_some_and(|formula| formula.kind == FormulaKind::Normal);
            if normal && first.is_none() {
                first = Some(cell.reference);
            }
        }
        if let Some(reference) = first {
            found = Some((part.as_slice(), reference, cells));
            break;
        }
    }
    if let Some((part, reference, _)) = found {
        package.set_cell(part, reference, Value::Number(1.0));
    }

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    // The output is in place: a failure from here on removes it, as in
    // `edit_sample`.
    let checked = formula_read_back(output, found);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (chain_after, cells_after, read_back) = checked?;
    edit_line(&[
        ("formula_replaced", u64::from(found.is_some())),
        ("chain_dropped", u64::from(chain_before && !chain_after)),
        ("cells_after", cells_after),
        ("read_back", read_back),
    ])
}

/// Reopens the output for the formula sample's line: whether its workbook
/// still names a calculation chain; the edited sheet's cell count, asserted
/// equal to the count before; and whether the edited cell reads back as
/// `number()` 1 with no formula. Both counts are 0 when no sheet was edited.
fn formula_read_back(
    output: &Path,
    edited: Option<(&[u8], CellRef, u64)>,
) -> Result<(bool, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let chain_after = written.workbook()?.calc_chain.is_some();
    let Some((part, reference, before)) = edited else {
        return Ok((chain_after, 0, 0));
    };
    let mut cells_after = 0u64;
    let mut read_back = 0u64;
    for cell in written.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        if cell.reference == reference && cell.number() == Some(1.0) && cell.formula.is_none() {
            read_back += 1;
        }
    }
    if cells_after != before {
        return Err(Error::CellsAfter {
            expected: before,
            found: cells_after,
        });
    }
    Ok((chain_after, cells_after, read_back))
}

/// What the first read of the part 6 formula sample's sheet finds.
struct FormulaSampled {
    /// A: the first `Number` cell with a value and no formula.
    number: Option<CellRef>,
    /// F: the first cell with a formula of kind `Normal`.
    normal: Option<CellRef>,
    /// M: the largest row number, 0 for a sheet with no cells.
    max_row: u32,
    cells: u64,
}

/// The formula sample of `specs/0013-formulas.md`, "Harness runs".
///
/// The first sheet of kind `Worksheet` in workbook order is read once for A,
/// F and M ([`FormulaSampled`]). With A, A is set to `Number(21)`, `(M + 1,
/// 1)` to the formula of A's reference followed by `*2`, `(M + 1, 2)` to
/// `1+1`, and F, when it exists, to `2+3`; the package is written, so the
/// write's rule drops the chain and records the flag. Without A nothing is
/// recorded and the write is the verbatim copy. The output is reopened and
/// its workbook and the same sheet read for the line; `cells_after` is
/// asserted equal to `cells_before` plus the cells added, and `read_back`,
/// the written formulas reading back as `Normal` with the text written and
/// no value, to `added + replaced`. Either failing removes the output, as
/// `edit_sample`'s assertion does. No reference or formula text reaches the
/// line or a message.
fn formula_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let chain_before = workbook.calc_chain.is_some();
    let full_calc_before = workbook.full_calc_on_load;
    let part = workbook
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let mut before = FormulaSampled {
        number: None,
        normal: None,
        max_row: 0,
        cells: 0,
    };
    for cell in package.cells(&part)? {
        let cell = cell?;
        before.cells += 1;
        before.max_row = before.max_row.max(cell.reference.row);
        let number =
            cell.kind == CellKind::Number && cell.value.is_some() && cell.formula.is_none();
        if number && before.number.is_none() {
            before.number = Some(cell.reference);
        }
        let normal = cell
            .formula
            .as_ref()
            .is_some_and(|formula| formula.kind == FormulaKind::Normal);
        if normal && before.normal.is_none() {
            before.normal = Some(cell.reference);
        }
    }

    // The formulas written, each with its reference and text, for the
    // read-back; empty without A.
    let mut written: Vec<(CellRef, String)> = Vec::new();
    if let Some(number) = before.number {
        let new_row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
        package.set_cell(&part, number, Value::Number(21.0));
        written.push((
            CellRef {
                row: new_row,
                col: 1,
            },
            format!("{}*2", number.a1()),
        ));
        written.push((
            CellRef {
                row: new_row,
                col: 2,
            },
            "1+1".to_string(),
        ));
        if let Some(normal) = before.normal {
            written.push((normal, "2+3".to_string()));
        }
        for (reference, text) in &written {
            package.set_cell(&part, *reference, Value::Formula(text.clone()));
        }
    }
    let added = if before.number.is_some() { 2 } else { 0 };
    let replaced = u64::from(before.number.is_some() && before.normal.is_some());

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    // The output is in place: a failure from here on removes it, as in
    // `edit_sample`.
    let expected = (before.cells + added, added + replaced);
    let checked = formula_sample_read_back(output, &part, &written, expected);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (chain_after, full_calc_after, cells_after, read_back) = checked?;
    print_line(&format!(
        "formula\tadded={added}\treplaced={replaced}\tchain_before={}\tchain_after={}\t\
         full_calc_before={}\tfull_calc_after={}\tcells_before={}\tcells_after={cells_after}\t\
         read_back={read_back}",
        u64::from(chain_before),
        u64::from(chain_after),
        u64::from(full_calc_before),
        u64::from(full_calc_after),
        before.cells,
    ))
}

/// Reopens the output for the part 6 formula sample's line: the workbook's
/// two flags, the sheet's cell count, asserted equal to `expected.0`, and how
/// many of the written formulas read back as `Normal` with the text written
/// and no value, asserted equal to `expected.1`.
fn formula_sample_read_back(
    output: &Path,
    part: &[u8],
    written: &[(CellRef, String)],
    expected: (u64, u64),
) -> Result<(bool, bool, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let chain_after = workbook.calc_chain.is_some();
    let full_calc_after = workbook.full_calc_on_load;
    let mut cells_after = 0u64;
    let mut read_back = 0u64;
    for cell in package.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        let as_written = written.iter().any(|(reference, text)| {
            cell.reference == *reference
                && cell.value.is_none()
                && cell.formula.as_ref().is_some_and(|formula| {
                    formula.kind == FormulaKind::Normal && formula.text == *text
                })
        });
        read_back += u64::from(as_written);
    }
    if cells_after != expected.0 {
        return Err(Error::CellsAfter {
            expected: expected.0,
            found: cells_after,
        });
    }
    if read_back != expected.1 {
        return Err(Error::ReadBack {
            expected: expected.1,
            found: read_back,
        });
    }
    Ok((chain_after, full_calc_after, cells_after, read_back))
}

/// The sample style edit of `specs/0012-styles.md`, "Harness runs".
///
/// The styles part is read for X (`cell_xfs.len()`) and F (`num_fmts.len()`),
/// and the first sheet of kind `Worksheet` in workbook order is read once
/// through `Package::cells` for A, the first `Number` or `SharedString` cell
/// with a value and no formula, and M, the largest row number ([`Sampled`]).
/// Two cell formats are then recorded: S1 over A's own `s` (`None` when A has
/// none or does not exist) with the code `0.000`, and S2 over no base with
/// the built-in id 14. A takes `Some(S1)` when it exists, and `(M + 1, 1)` is
/// set to `Number(45000.5)` with `Some(S2)`.
///
/// The output is written, reopened, and its styles part and the same sheet
/// read back. `xfs_after` is asserted equal to X + 2 and `cells_after` to
/// `cells_before + 1`; either failing removes the output, as `edit_sample`'s
/// assertion does, so a refused run leaves none. No reference, value, text or
/// format code reaches the line or a message: the line is counts only.
fn style_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let styles = package.styles()?;
    let xfs_before = styles.cell_xfs.len() as u64;
    let formats_before = styles.num_fmts.len() as u64;
    let before = sample(&mut package, &part)?;
    let new_row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
    let new_cell = CellRef {
        row: new_row,
        col: 1,
    };

    // S1 takes A's own format as its base, so the addition keeps whatever
    // font, fill and border A had; S2 takes none, so it is the derived
    // format's plain shape. The code is one no corpus part defines, which is
    // what makes `formats_added` 1 in every workbook.
    let coded =
        package.add_cell_style(before.first_style, NumberFormat::Code("0.000".to_string()))?;
    let dated = package.add_cell_style(None, NumberFormat::BuiltIn(14))?;
    if let Some(first) = before.first {
        package.set_cell_style(&part, first, Some(coded));
    }
    package.set_cell(&part, new_cell, Value::Number(45000.5));
    package.set_cell_style(&part, new_cell, Some(dated));

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    // The output is in place: a failure from here on removes it, as
    // `edit_sample`'s assertion does.
    let checked = style_read_back(
        output,
        &part,
        &before,
        new_cell,
        (coded, dated),
        (xfs_before + 2, before.cells + 1),
    );
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (xfs_after, formats_after, cells_after, read_back) = checked?;
    edit_line(&[
        ("styled", u64::from(before.first.is_some())),
        ("added", 1),
        ("xfs_before", xfs_before),
        ("xfs_after", xfs_after),
        (
            "formats_added",
            formats_after.saturating_sub(formats_before),
        ),
        ("cells_before", before.cells),
        ("cells_after", cells_after),
        ("read_back", read_back),
    ])
}

/// Reopens the output for the style sample's line: the styles part's cell
/// format count, asserted equal to `expected.0`; its `numFmt` count; the
/// edited sheet's cell count, asserted equal to `expected.1`; and how many of
/// the three read-backs hold: A's value text unchanged with `s` the new
/// coded format, the new cell's number with `s` the new dated format, and the
/// output's own number formats for the two new indices.
fn style_read_back(
    output: &Path,
    part: &[u8],
    before: &Sampled,
    new_cell: CellRef,
    added: (u32, u32),
    expected: (u64, u64),
) -> Result<(u64, u64, u64, u64), Error> {
    let (coded, dated) = added;
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let styles = written.styles()?;
    let xfs_after = styles.cell_xfs.len() as u64;
    let formats_after = styles.num_fmts.len() as u64;
    let formats_read_back = styles.number_format(coded)
        == Some(NumberFormat::Code("0.000".to_string()))
        && styles.number_format(dated) == Some(NumberFormat::BuiltIn(14));
    if xfs_after != expected.0 {
        return Err(Error::XfsAfter {
            expected: expected.0,
            found: xfs_after,
        });
    }
    let mut cells_after = 0u64;
    let mut read_back = u64::from(formats_read_back);
    for cell in written.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        let at = cell.reference;
        if Some(at) == before.first && cell.value == before.first_value && cell.style == Some(coded)
        {
            read_back += 1;
        }
        if at == new_cell && cell.number() == Some(45000.5) && cell.style == Some(dated) {
            read_back += 1;
        }
    }
    if cells_after != expected.1 {
        return Err(Error::CellsAfter {
            expected: expected.1,
            found: cells_after,
        });
    }
    Ok((xfs_after, formats_after, cells_after, read_back))
}

/// F1 of `specs/0017-fuller-styles.md`, "Harness runs": the font the font
/// sample adds.
fn sample_font() -> Font {
    Font {
        name: Some("emend".to_string()),
        size: Some(12.0),
        bold: true,
        color: Some(Color::Rgb(0xFF33_6699)),
        ..Font::default()
    }
}

/// L1 of the same section: a solid fill with an `rgb` foreground and an
/// `indexed` 64 background, the corpus's commonest solid fill shape.
fn sample_fill() -> Fill {
    Fill {
        pattern: Pattern::Solid,
        foreground: Some(Color::Rgb(0xFFDD_EEFF)),
        background: Some(Color::Indexed(64)),
    }
}

/// The sample font and fill of `specs/0017-fuller-styles.md`, "Harness
/// runs", `--font-sample`.
///
/// As [`style_sample`], with X the `cell_xfs` count, F and I the `fonts` and
/// `fills` counts, A and M from [`sample`]. S1 is `add_style` over A's own
/// `s` with F1 and L1, S2 over no base with F1 alone, so F1 is one font
/// element; A takes `Some(S1)` when it exists, and `(M + 1, 1)` is set to
/// `Number(7)` with `Some(S2)`. `xfs_after` is asserted equal to X + 2 and
/// `cells_after` to `cells_before + 1`; either failing removes the output.
fn font_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let styles = package.styles()?;
    let xfs_before = styles.cell_xfs.len() as u64;
    let counts_before = (styles.fonts(), styles.fills());
    let before = sample(&mut package, &part)?;
    let new_row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
    let new_cell = CellRef {
        row: new_row,
        col: 1,
    };

    let both = Style {
        font: Some(sample_font()),
        fill: Some(sample_fill()),
        ..Style::default()
    };
    let font_alone = Style {
        font: Some(sample_font()),
        ..Style::default()
    };
    let first = package.add_style(before.first_style, &both)?;
    let second = package.add_style(None, &font_alone)?;
    if let Some(at) = before.first {
        package.set_cell_style(&part, at, Some(first));
    }
    package.set_cell(&part, new_cell, Value::Number(7.0));
    package.set_cell_style(&part, new_cell, Some(second));

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    // The output is in place: a failure from here on removes it.
    let checked = font_read_back(
        output,
        &part,
        &before,
        new_cell,
        counts_before,
        (first, second),
        (xfs_before + 2, before.cells + 1),
    );
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let after = checked?;
    let added = |after: Option<u32>, before: Option<u32>| {
        u64::from(after.unwrap_or(0).saturating_sub(before.unwrap_or(0)))
    };
    edit_line(&[
        ("styled", u64::from(before.first.is_some())),
        ("added", 1),
        ("xfs_before", xfs_before),
        ("xfs_after", after.xfs),
        ("fonts_added", added(after.fonts, counts_before.0)),
        ("fills_added", added(after.fills, counts_before.1)),
        ("cells_before", before.cells),
        ("cells_after", after.cells),
        ("read_back", after.read_back),
    ])
}

/// What [`font_read_back`] reads off the output.
struct FontReadBack {
    xfs: u64,
    fonts: Option<u32>,
    fills: Option<u32>,
    cells: u64,
    read_back: u64,
}

/// Reopens the output for the font sample's line: the cell format count,
/// asserted equal to `expected.0`; the `fonts` and `fills` counts; the
/// edited sheet's cell count, asserted equal to `expected.1`; and how many
/// of the two read-backs hold: A's value text unchanged with `s` the first
/// new format, whose font and fill are the part's first new ones (F and I),
/// and the new cell's number 7 with `s` the second, whose font is F and
/// whose fill is 0.
fn font_read_back(
    output: &Path,
    part: &[u8],
    before: &Sampled,
    new_cell: CellRef,
    counts_before: (Option<u32>, Option<u32>),
    added: (u32, u32),
    expected: (u64, u64),
) -> Result<FontReadBack, Error> {
    let (first, second) = added;
    let (fonts, fills) = counts_before;
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let styles = written.styles()?;
    let xfs_after = styles.cell_xfs.len() as u64;
    let counts_after = (styles.fonts(), styles.fills());
    let ids = |index: u32| {
        styles
            .cell_xfs
            .get(index as usize)
            .map(|xf| (xf.font_id, xf.fill_id))
    };
    let first_ids = ids(first) == Some((fonts, fills));
    let second_ids = ids(second) == Some((fonts, Some(0)));
    if xfs_after != expected.0 {
        return Err(Error::XfsAfter {
            expected: expected.0,
            found: xfs_after,
        });
    }
    let mut cells_after = 0u64;
    let mut read_back = 0u64;
    for cell in written.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        let at = cell.reference;
        if Some(at) == before.first
            && cell.value == before.first_value
            && cell.style == Some(first)
            && first_ids
        {
            read_back += 1;
        }
        if at == new_cell && cell.number() == Some(7.0) && cell.style == Some(second) && second_ids
        {
            read_back += 1;
        }
    }
    if cells_after != expected.1 {
        return Err(Error::CellsAfter {
            expected: expected.1,
            found: cells_after,
        });
    }
    Ok(FontReadBack {
        xfs: xfs_after,
        fonts: counts_after.0,
        fills: counts_after.1,
        cells: cells_after,
        read_back,
    })
}

/// B1 of `specs/0017-fuller-styles.md`, "Harness runs": four thin sides in
/// theme colour 1 and no diagonal.
fn sample_border() -> Border {
    let side = Some(Side {
        style: BorderStyle::Thin,
        color: Some(Color::Theme {
            index: 1,
            tint: None,
        }),
    });
    Border {
        left: side,
        right: side,
        top: side,
        bottom: side,
        ..Border::default()
    }
}

/// The sample border, alignment and protection of
/// `specs/0017-fuller-styles.md`, "Harness runs", `--border-sample`.
///
/// As [`font_sample`], with B the `borders` count. S1 is `add_style` over A's
/// own `s` with B1, G1 (centred both ways, wrapped, indent 1) and a
/// protection neither locked nor hidden; S2 over no base with B1 and a right
/// alignment, so B1 is one border element. A takes `Some(S1)` when it
/// exists, and `(M + 1, 1)` is set to `Number(7)` with `Some(S2)`.
/// `xfs_after` is asserted equal to X + 2 and `cells_after` to
/// `cells_before + 1`; either failing removes the output.
fn border_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let styles = package.styles()?;
    let xfs_before = styles.cell_xfs.len() as u64;
    let borders_before = styles.borders();
    let before = sample(&mut package, &part)?;
    let new_row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
    let new_cell = CellRef {
        row: new_row,
        col: 1,
    };

    let first_style = Style {
        border: Some(sample_border()),
        alignment: Some(Alignment {
            horizontal: Some(Horizontal::Center),
            vertical: Some(Vertical::Center),
            wrap_text: true,
            indent: Some(1),
            ..Alignment::default()
        }),
        protection: Some(Protection {
            locked: Some(false),
            hidden: Some(false),
        }),
        ..Style::default()
    };
    let second_style = Style {
        border: Some(sample_border()),
        alignment: Some(Alignment {
            horizontal: Some(Horizontal::Right),
            ..Alignment::default()
        }),
        ..Style::default()
    };
    let first = package.add_style(before.first_style, &first_style)?;
    let second = package.add_style(None, &second_style)?;
    if let Some(at) = before.first {
        package.set_cell_style(&part, at, Some(first));
    }
    package.set_cell(&part, new_cell, Value::Number(7.0));
    package.set_cell_style(&part, new_cell, Some(second));

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    // The output is in place: a failure from here on removes it.
    let checked = border_read_back(
        output,
        &part,
        &before,
        new_cell,
        borders_before,
        (first, second),
        (xfs_before + 2, before.cells + 1),
    );
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let after = checked?;
    let added = u64::from(
        after
            .borders
            .unwrap_or(0)
            .saturating_sub(borders_before.unwrap_or(0)),
    );
    edit_line(&[
        ("styled", u64::from(before.first.is_some())),
        ("added", 1),
        ("xfs_before", xfs_before),
        ("xfs_after", after.xfs),
        ("borders_added", added),
        ("cells_before", before.cells),
        ("cells_after", after.cells),
        ("read_back", after.read_back),
    ])
}

/// What [`border_read_back`] reads off the output.
struct BorderReadBack {
    xfs: u64,
    borders: Option<u32>,
    cells: u64,
    read_back: u64,
}

/// Reopens the output for the border sample's line: the cell format count,
/// asserted equal to `expected.0`; the `borders` count; the edited sheet's
/// cell count, asserted equal to `expected.1`; and how many of the two
/// read-backs hold: A's value text unchanged with `s` the first new format,
/// whose border is the part's first new one (B), and the new cell's number 7
/// with `s` the second, whose border is B too.
fn border_read_back(
    output: &Path,
    part: &[u8],
    before: &Sampled,
    new_cell: CellRef,
    borders_before: Option<u32>,
    added: (u32, u32),
    expected: (u64, u64),
) -> Result<BorderReadBack, Error> {
    let (first, second) = added;
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let styles = written.styles()?;
    let xfs_after = styles.cell_xfs.len() as u64;
    let borders_after = styles.borders();
    let border_of = |index: u32| styles.cell_xfs.get(index as usize).map(|xf| xf.border_id);
    let first_ids = border_of(first) == Some(borders_before);
    let second_ids = border_of(second) == Some(borders_before);
    if xfs_after != expected.0 {
        return Err(Error::XfsAfter {
            expected: expected.0,
            found: xfs_after,
        });
    }
    let mut cells_after = 0u64;
    let mut read_back = 0u64;
    for cell in written.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        let at = cell.reference;
        if Some(at) == before.first
            && cell.value == before.first_value
            && cell.style == Some(first)
            && first_ids
        {
            read_back += 1;
        }
        if at == new_cell && cell.number() == Some(7.0) && cell.style == Some(second) && second_ids
        {
            read_back += 1;
        }
    }
    if cells_after != expected.1 {
        return Err(Error::CellsAfter {
            expected: expected.1,
            found: cells_after,
        });
    }
    Ok(BorderReadBack {
        xfs: xfs_after,
        borders: borders_after,
        cells: cells_after,
        read_back,
    })
}

/// The new workbook sample (`specs/0016-new-workbook-and-sheets.md`,
/// "Harness runs", `--new-sample`): `first` is written with
/// `Package::new()`'s bytes, opened, and the three sample cells set on its
/// first sheet are written to `second`, which is then read back.
fn new_sample(first: &Path, second: &Path) -> Result<(), Error> {
    write_into(Package::new()?, first)?;
    let file = File::open(first).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    for (col, value) in new_sample_values() {
        package.set_cell(&part, CellRef { row: 1, col }, value);
    }
    write_into(package, second)?;
    // The second file is in place: a failure from here on removes it, and
    // the first, which is the constructor's and not the edit's, stays.
    let checked = new_read_back(second, &part);
    if checked.is_err() {
        let _ = fs::remove_file(second);
    }
    let (entries, sheets, cells, read_back) = checked?;
    let line =
        format!("new\tentries={entries}\tsheets={sheets}\tcells={cells}\tread_back={read_back}");
    print_line(&line)
}

/// The three values the new workbook sample writes in row 1, by column.
fn new_sample_values() -> [(u32, Value); 3] {
    [
        (1, Value::Number(7.0)),
        (2, Value::Text("emend".to_string())),
        (3, Value::Boolean(true)),
    ]
}

/// Reopens the new workbook sample's second file: its entry and sheet
/// counts, its first sheet's cell count, asserted equal to 3, and how many
/// of those cells read back with the value written, asserted equal to 3.
fn new_read_back(output: &Path, part: &[u8]) -> Result<(u64, u64, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let entries = written.archive().entries().len() as u64;
    let sheets = written.workbook()?.sheets.len() as u64;
    let values = new_sample_values();
    let mut cells = 0u64;
    let mut read_back = 0u64;
    for cell in written.cells(part)? {
        let cell = cell?;
        cells += 1;
        let matched = values.iter().any(|(col, value)| {
            cell.reference == CellRef { row: 1, col: *col }
                && match value {
                    Value::Number(n) => cell.number() == Some(*n),
                    Value::Text(text) => {
                        cell.kind == CellKind::InlineString && cell.value.as_deref() == Some(text)
                    }
                    Value::Boolean(b) => cell.boolean() == Some(*b),
                    _ => false,
                }
        });
        read_back += u64::from(matched);
    }
    let expected = values.len() as u64;
    if cells != expected {
        return Err(Error::CellsAfter {
            expected,
            found: cells,
        });
    }
    if read_back != expected {
        return Err(Error::NewReadBack {
            expected,
            found: read_back,
        });
    }
    Ok((entries, sheets, cells, read_back))
}

/// The sheet sample (`specs/0016-new-workbook-and-sheets.md`, "Harness
/// runs", `--sheet-sample`).
fn sheet_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let before = package.sheets()?;
    let (names_before, active_before, _) = workbook_counts(&mut package, None)?;
    let folded = |name: &str| {
        name.chars()
            .flat_map(char::to_lowercase)
            .collect::<String>()
    };
    let taken: Vec<String> = before.iter().map(|sheet| folded(&sheet.name)).collect();
    let name = (1u32..=before.len() as u32 + 1)
        .map(|n| {
            if n == 1 {
                "emend".to_string()
            } else {
                format!("emend{n}")
            }
        })
        .find(|candidate| !taken.contains(&folded(candidate)))
        .ok_or(Error::NoFreeName)?;
    let part = package.add_sheet(&name)?;
    package.set_cell(&part, CellRef { row: 1, col: 1 }, Value::Number(7.0));
    package.move_sheet(&name, 0)?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = sheet_read_back(
        output,
        &part,
        before.len() as u64,
        names_before,
        active_before,
    );
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (sheets_after, cells, names_after, active_after) = checked?;
    edit_line_named(
        "sheet",
        &[
            ("sheets_before", before.len() as u64),
            ("sheets_after", sheets_after),
            ("first", 1),
            ("cells", cells),
            ("names", names_after),
            ("active_before", active_before),
            ("active_after", active_after),
        ],
    )
}

/// Reopens the sheet sample's output and asserts what the edits imply: one
/// sheet more, the added part first, one cell in it, the same number of
/// defined names, and the first view's active tab one more.
fn sheet_read_back(
    output: &Path,
    part: &[u8],
    sheets_before: u64,
    names_before: u64,
    active_before: u64,
) -> Result<(u64, u64, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let sheets = written.workbook()?.sheets.clone();
    let sheets_after = sheets.len() as u64;
    if sheets_after != sheets_before + 1 {
        return Err(Error::SheetSample("sheet count"));
    }
    if sheets.first().map(|sheet| sheet.part.as_slice()) != Some(part) {
        return Err(Error::SheetSample("first sheet"));
    }
    let mut cells = 0u64;
    for cell in written.cells(part)? {
        cell?;
        cells += 1;
    }
    if cells != 1 {
        return Err(Error::SheetSample("added sheet's cell count"));
    }
    let (names_after, active_after, _) = workbook_counts(&mut written, None)?;
    if names_after != names_before {
        return Err(Error::SheetSample("defined name count"));
    }
    if active_after != active_before + 1 {
        return Err(Error::SheetSample("active tab"));
    }
    Ok((sheets_after, cells, names_after, active_after))
}

/// The removal sample (`specs/0016-new-workbook-and-sheets.md`, "Harness
/// runs", `--remove-sample`).
fn remove_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let before = package.sheets()?;
    let sheets_before = before.len() as u64;
    let last = sheets_before.checked_sub(1).ok_or(Error::NoWorksheet)?;
    let chain_before = u64::from(package.workbook()?.calc_chain.is_some());
    let entries_before = package.archive().entries().len() as u64;
    let (names_before, active_before, local_removed) = workbook_counts(&mut package, Some(last))?;
    let removed = match package.remove_sheet(&before[last as usize].name) {
        Ok(()) => 1u64,
        Err(LayerError::Package(
            "the last sheet cannot be removed" | "the last visible sheet cannot be removed",
        )) => 0,
        Err(e) => return Err(e.into()),
    };
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = remove_read_back(input, output, removed, last, active_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (sheets_after, entries_after, chain_after, names_after, active_after) = checked?;
    let parts = entries_before.saturating_sub(entries_after);
    let local_removed = local_removed * removed;
    let expected_names = names_before.checked_sub(local_removed);
    let failure = if sheets_after != sheets_before - removed {
        Some("sheet count")
    } else if removed == 0 && parts != 0 {
        Some("entry count")
    } else if removed == 1 && chain_after != 0 {
        Some("calculation chain")
    } else if expected_names != Some(names_after) {
        Some("defined name count")
    } else {
        None
    };
    if let Some(what) = failure {
        let _ = fs::remove_file(output);
        return Err(Error::RemoveSample(what));
    }
    edit_line_named(
        "remove",
        &[
            ("sheets_before", sheets_before),
            ("sheets_after", sheets_after),
            ("removed", removed),
            ("parts", parts),
            ("chain_before", chain_before),
            ("chain_after", chain_after),
            ("names_before", names_before),
            ("names_after", names_after),
            ("local_removed", local_removed),
            ("active_before", active_before),
            ("active_after", active_after),
        ],
    )
}

/// Reopens the removal sample's output: its sheet and entry counts, whether
/// it names a chain, its defined names and first view's active tab. With
/// nothing removed the output must be the input's bytes; with a sheet
/// removed the active tab must be the first visible sheet's index when it
/// was the removed sheet's, and unchanged otherwise.
fn remove_read_back(
    input: &Path,
    output: &Path,
    removed: u64,
    last: u64,
    active_before: u64,
) -> Result<(u64, u64, u64, u64, u64), Error> {
    if removed == 0 {
        let a = fs::read(input).map_err(|e| Error::ReadInput(e.kind()))?;
        let b = fs::read(output).map_err(|e| Error::ReadOutput(e.kind()))?;
        if a != b {
            return Err(Error::RemoveSample("bytes"));
        }
    }
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let entries_after = written.archive().entries().len() as u64;
    let workbook = written.workbook()?;
    let sheets = workbook.sheets.clone();
    let chain_after = u64::from(workbook.calc_chain.is_some());
    let (names_after, active_after, _) = workbook_counts(&mut written, None)?;
    let first_visible = sheets
        .iter()
        .position(|sheet| {
            sheet
                .state
                .as_deref()
                .is_none_or(|state| state == b"visible")
        })
        .unwrap_or(0) as u64;
    let expected = if removed == 1 && active_before == last {
        first_visible
    } else {
        active_before
    };
    if active_after != expected {
        return Err(Error::RemoveSample("active tab"));
    }
    Ok((
        sheets.len() as u64,
        entries_after,
        chain_after,
        names_after,
        active_after,
    ))
}

/// This tool's own pass over the workbook part's tokens: the `definedName`
/// elements that are children of the root's `definedNames`, the first
/// `workbookView`'s `activeTab` (absent read as 0), and the `definedName`
/// elements whose `localSheetId` is `scope`. Tags are read from each token's
/// bytes by local name, so the pass shares no reader with the layer's
/// rewrite (P3 ruling 7).
fn workbook_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    scope: Option<u64>,
) -> Result<(u64, u64, u64), Error> {
    let part = package.workbook()?.part.clone();
    let index = package.part(&part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut region: Vec<u8> = Vec::new();
    let mut names = 0u64;
    let mut scoped = 0u64;
    let mut active: Option<u64> = None;
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            depth = depth.saturating_sub(1);
            if depth < 2 {
                region.clear();
            }
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        let local = tag_local_name(raw);
        if depth == 2 {
            region = local.to_vec();
        } else if depth == 3 && region == b"definedNames" && local == b"definedName" {
            names += 1;
            if scope.is_some() && tag_number(raw, b"localSheetId") == scope {
                scoped += 1;
            }
        } else if depth == 3
            && region == b"bookViews"
            && local == b"workbookView"
            && active.is_none()
        {
            active = Some(tag_number(raw, b"activeTab").unwrap_or(0));
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
            if depth < 2 {
                region.clear();
            }
        }
    }
    Ok((names, active.unwrap_or(0), scoped))
}

/// A start tag's element name after any prefix.
fn tag_local_name(raw: &[u8]) -> &[u8] {
    let end = raw[1..]
        .iter()
        .position(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'/' | b'>'))
        .map_or(raw.len(), |at| at + 1);
    let name = &raw[1..end];
    match name.iter().position(|byte| *byte == b':') {
        Some(colon) => &name[colon + 1..],
        None => name,
    }
}

/// An unprefixed attribute's value in a start tag, read as a decimal number.
fn tag_number(raw: &[u8], attribute: &[u8]) -> Option<u64> {
    let mut at = 0;
    while let Some(found) = raw[at..]
        .windows(attribute.len())
        .position(|window| window == attribute)
    {
        let start = at + found;
        at = start + attribute.len();
        let before = raw.get(start.wrapping_sub(1)).copied().unwrap_or(b'<');
        if !matches!(before, b' ' | b'\t' | b'\n' | b'\r') {
            continue;
        }
        let rest = &raw[at..];
        let rest = rest.strip_prefix(b"=").or_else(|| {
            let trimmed = rest.trim_ascii_start();
            trimmed.strip_prefix(b"=")
        })?;
        let rest = rest.trim_ascii_start();
        let quote = *rest.first()?;
        let value = &rest[1..];
        let end = value.iter().position(|byte| *byte == quote)?;
        return std::str::from_utf8(&value[..end]).ok()?.parse().ok();
    }
    None
}

/// The format sample (`specs/0018-rows-and-columns.md`, "Harness runs",
/// `--format-sample`, and P1 ruling 1).
///
/// W, the first sheet of kind `Worksheet`, is read once for its cells and
/// its largest row number M, and counted by [`sheet_counts`]; column 2 gets
/// width 20.5, column 3 is hidden, row 2 gets height 30, row 3 is hidden,
/// the panes are frozen at B2 and `(M + 1, 1)..(M + 1, 2)` is merged, below
/// the data so no cell is cleared. The output is reopened for the same
/// counts, and `cells_after` equal to `cells_before`, `rows_after` equal to
/// `rows_before` plus rows 2 and 3 where they were absent, and
/// `merges_after` equal to `merges_before` plus one are asserted; a failure
/// after the output is in place removes it.
fn format_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let before = sample(&mut package, &part)?;
    let counts = sheet_counts(&mut package, &part)?;
    let merge_row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
    package.set_column_width(&part, 2, Some(20.5))?;
    package.set_column_hidden(&part, 3, true)?;
    package.set_row_height(&part, 2, Some(30.0))?;
    package.set_row_hidden(&part, 3, true)?;
    package.freeze_panes(&part, Some(CellRef { row: 2, col: 2 }))?;
    package.merge_cells(
        &part,
        Area {
            first: CellRef {
                row: merge_row,
                col: 1,
            },
            last: CellRef {
                row: merge_row,
                col: 2,
            },
        },
    )?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = format_read_back(output, &part, before.cells, &counts);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (cells_after, after) = checked?;
    edit_line_named(
        "format",
        &[
            ("cells_before", before.cells),
            ("cells_after", cells_after),
            ("rows_before", counts.rows),
            ("rows_after", after.rows),
            ("merges_before", counts.merges),
            ("merges_after", after.merges),
        ],
    )
}

/// The row and column samples (`specs/0018-rows-and-columns.md`, "Harness
/// runs", `--insert-rows-sample` and `--delete-rows-sample`, with P2 ruling
/// 6, and `--insert-columns-sample` and `--delete-columns-sample`, with P3
/// ruling 3).
///
/// W is the first sheet of kind `Worksheet`; T (U on the column axis) the
/// largest last row (column) of the table and pivot table ranges W's
/// relationships name ([`table_last`]), 0 when there are none; k (c) is
/// `max(2, T + 1)`. A row insertion puts two rows at k, a column insertion
/// one column at c, and a deletion removes row k or column c. The output is
/// reopened and read again: an insertion keeps every cell and the cells
/// from the point on are the output's from the point plus the count on; a
/// deletion loses the cells at the point and the cells after it are the
/// output's from the point on. A failure after the output is in place
/// removes it; a refusal by the layer is exit 2 with no output.
fn shift_sample(input: &Path, output: &Path, insert: bool, columns: bool) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let chain_before = u64::from(package.workbook()?.calc_chain.is_some());
    let at = table_last(&mut package, &part, columns)?
        .saturating_add(1)
        .max(2);
    let count: u32 = if insert && !columns { 2 } else { 1 };
    let before = axis_counts(&mut package, &part, at, count, columns)?;
    match (insert, columns) {
        (true, false) => package.insert_rows(&part, at, count)?,
        (false, false) => package.delete_rows(&part, at, count)?,
        (true, true) => package.insert_columns(&part, at, count)?,
        (false, true) => package.delete_columns(&part, at, count)?,
    }
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = shift_read_back(output, &part, at, count, insert, columns, &before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, chain_after, full_calc) = checked?;
    if columns {
        let (name, moved) = if insert {
            (
                "insert_columns",
                [
                    ("from_c_before", before.from_k),
                    ("from_c1_after", after.from_k_count),
                ],
            )
        } else {
            (
                "delete_columns",
                [
                    ("from_c1_before", before.from_k1),
                    ("from_c_after", after.from_k),
                ],
            )
        };
        return edit_line_named(
            name,
            &[
                ("at", u64::from(at)),
                ("count", u64::from(count)),
                ("cells_before", before.cells),
                ("cells_after", after.cells),
                moved[0],
                moved[1],
                ("formulas_moved", before.formulas_from_k),
                ("chain_before", chain_before),
                ("chain_after", chain_after),
                ("full_calc", full_calc),
            ],
        );
    }
    if insert {
        edit_line_named(
            "insert_rows",
            &[
                ("at", u64::from(at)),
                ("count", u64::from(count)),
                ("cells_before", before.cells),
                ("cells_after", after.cells),
                ("from_k_before", before.from_k),
                ("from_k2_after", after.from_k_count),
                ("formulas_moved", before.formulas_from_k),
                ("chain_before", chain_before),
                ("chain_after", chain_after),
                ("full_calc", full_calc),
            ],
        )
    } else {
        edit_line_named(
            "delete_rows",
            &[
                ("at", u64::from(at)),
                ("count", u64::from(count)),
                ("cells_before", before.cells),
                ("cells_after", after.cells),
                ("from_k1_before", before.from_k1),
                ("from_k_after", after.from_k),
                ("formulas_moved", before.formulas_from_k),
                ("chain_before", chain_before),
                ("chain_after", chain_after),
                ("full_calc", full_calc),
            ],
        )
    }
}

/// A sheet's cells counted about an index k on one axis, the row or the
/// column.
struct AxisCounts {
    cells: u64,
    /// Cells at index k.
    at_k: u64,
    /// Cells at k or after, k + 1 or after, and k + count or after.
    from_k: u64,
    from_k1: u64,
    from_k_count: u64,
    /// Cells with a formula at k or after.
    formulas_from_k: u64,
}

/// Counts the cells of `part` about index `at` on the row or column axis
/// through the layer's reader.
fn axis_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
    at: u32,
    count: u32,
    columns: bool,
) -> Result<AxisCounts, Error> {
    let mut counts = AxisCounts {
        cells: 0,
        at_k: 0,
        from_k: 0,
        from_k1: 0,
        from_k_count: 0,
        formulas_from_k: 0,
    };
    let at = u64::from(at);
    for cell in package.cells(part)? {
        let cell = cell?;
        let index = u64::from(if columns {
            cell.reference.col
        } else {
            cell.reference.row
        });
        counts.cells += 1;
        counts.at_k += u64::from(index == at);
        counts.from_k += u64::from(index >= at);
        counts.from_k1 += u64::from(index > at);
        counts.from_k_count += u64::from(index >= at + u64::from(count));
        counts.formulas_from_k += u64::from(index >= at && cell.formula.is_some());
    }
    Ok(counts)
}

/// Reopens a row or column sample's output, asserts what the shift implies
/// and reads the chain and the full calculation flag.
fn shift_read_back(
    output: &Path,
    part: &[u8],
    at: u32,
    count: u32,
    insert: bool,
    columns: bool,
    before: &AxisCounts,
) -> Result<(AxisCounts, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let after = axis_counts(&mut written, part, at, count, columns)?;
    let (expected, moved_before, moved_after) = if insert {
        (before.cells, before.from_k, after.from_k_count)
    } else {
        (before.cells - before.at_k, before.from_k1, after.from_k)
    };
    if after.cells != expected {
        return Err(Error::CellsAfter {
            expected,
            found: after.cells,
        });
    }
    if moved_after != moved_before {
        return Err(Error::SheetSample("count of moved cells"));
    }
    let chain_after = u64::from(written.workbook()?.calc_chain.is_some());
    let full_calc = u64::from(written.workbook()?.full_calc_on_load);
    Ok((after, chain_after, full_calc))
}

/// T of the row samples, or U of the column samples: the largest last row,
/// or column, of the `ref` of every table part and the `location ref` of
/// every pivot table part that `part`'s relationships name, read by this
/// tool's own pass over each part's tokens (`emend::xml::Stream`, quick-xml
/// underneath); 0 when there are none. A range that does not read is 0
/// here: the layer refuses it at the write.
fn table_last<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
    columns: bool,
) -> Result<u32, Error> {
    let rels = package.relationships(part)?;
    let mut targets: Vec<(Vec<u8>, bool)> = Vec::new();
    for (kind, location) in [(TABLE, false), (PIVOT_TABLE, true)] {
        for form in kind {
            for rel in rels.of_kind(form) {
                if let Ok(target) = rels.target_part(rel) {
                    targets.push((target, location));
                }
            }
        }
    }
    let mut top = 0u32;
    for (target, location) in targets {
        let Some(index) = package.part(&target) else {
            continue;
        };
        let mut stream = Stream::new(package.read_part(index)?, index)?;
        let mut depth = 0usize;
        while let Some(token) = stream.next()? {
            let raw = token.raw;
            if raw.starts_with(b"</") {
                depth = depth.saturating_sub(1);
                continue;
            }
            if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
                continue;
            }
            depth += 1;
            let wanted = if location {
                tag_local_name(raw) == b"location"
            } else {
                depth == 1
            };
            if wanted {
                let last = tag_attribute(raw, b"ref")
                    .and_then(|value| Area::parse(&value))
                    .map_or(0, |area| {
                        if columns {
                            area.last.col
                        } else {
                            area.last.row
                        }
                    });
                top = top.max(last);
                break;
            }
            if raw.ends_with(b"/>") {
                depth -= 1;
            }
        }
    }
    Ok(top)
}

/// Reopens the format sample's output and asserts its three counts.
fn format_read_back(
    output: &Path,
    part: &[u8],
    cells_before: u64,
    before: &SheetCounts,
) -> Result<(u64, SheetCounts), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let mut cells = 0u64;
    for cell in written.cells(part)? {
        cell?;
        cells += 1;
    }
    if cells != cells_before {
        return Err(Error::CellsAfter {
            expected: cells_before,
            found: cells,
        });
    }
    let after = sheet_counts(&mut written, part)?;
    let created = u64::from(!before.row_two) + u64::from(!before.row_three);
    if after.rows != before.rows + created || !after.row_two || !after.row_three {
        return Err(Error::SheetSample("row count"));
    }
    if after.merges != before.merges + 1 {
        return Err(Error::SheetSample("merge count"));
    }
    Ok((cells, after))
}

/// What [`sheet_counts`] reads off a worksheet part.
struct SheetCounts {
    /// `<row>` elements in the first `sheetData` of the main namespace.
    rows: u64,
    /// Whether rows 2 and 3 are among them.
    row_two: bool,
    row_three: bool,
    /// `<mergeCell>` elements in every `mergeCells` of the main namespace.
    merges: u64,
}

/// Counts a worksheet's rows and merges by this tool's own pass over the
/// part's tokens (`emend::xml::Stream`, quick-xml underneath), as
/// [`workbook_counts`] counts the workbook's (`specs/0018-rows-and-
/// columns.md`, P1 ruling 1). A root child is in the main namespace when
/// the namespace its prefix, or its lack of one, is bound to, on the child
/// itself or else on the root, is one of the two main forms; the rows and
/// merges inside are counted by local name.
fn sheet_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<SheetCounts, Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut counts = SheetCounts {
        rows: 0,
        row_two: false,
        row_three: false,
        merges: 0,
    };
    let mut depth = 0usize;
    let mut root: Vec<u8> = Vec::new();
    let mut region: Vec<u8> = Vec::new();
    let mut seen_data = false;
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            depth = depth.saturating_sub(1);
            if depth < 2 {
                region.clear();
            }
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        let local = tag_local_name(raw);
        if depth == 1 {
            root = raw.to_vec();
        } else if depth == 2 {
            let main = MAIN.contains(&element_namespace(raw, &root).as_slice());
            region = match local {
                b"sheetData" if main && !seen_data => {
                    seen_data = true;
                    local.to_vec()
                }
                b"mergeCells" if main => local.to_vec(),
                _ => Vec::new(),
            };
        } else if depth == 3 && region == b"sheetData" && local == b"row" {
            counts.rows += 1;
            match tag_number(raw, b"r") {
                Some(2) => counts.row_two = true,
                Some(3) => counts.row_three = true,
                _ => {}
            }
        } else if depth == 3 && region == b"mergeCells" && local == b"mergeCell" {
            counts.merges += 1;
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
            if depth < 2 {
                region.clear();
            }
        }
    }
    Ok(counts)
}

/// The namespace an element's name is in: the declaration for its prefix
/// (`xmlns` for none) on its own start tag, else on `root`'s, else empty.
fn element_namespace(raw: &[u8], root: &[u8]) -> Vec<u8> {
    let prefix = raw[1..]
        .iter()
        .take_while(|byte| !matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'/' | b'>'))
        .position(|byte| *byte == b':')
        .map(|colon| &raw[1..1 + colon]);
    let attribute = match prefix {
        Some(prefix) => [b"xmlns:".as_slice(), prefix].concat(),
        None => b"xmlns".to_vec(),
    };
    tag_attribute(raw, &attribute)
        .or_else(|| tag_attribute(root, &attribute))
        .unwrap_or_default()
}

/// An attribute's value in a start tag, as written, when the tag holds an
/// attribute of exactly that name.
fn tag_attribute(raw: &[u8], attribute: &[u8]) -> Option<Vec<u8>> {
    let mut at = 0;
    while let Some(found) = raw[at..]
        .windows(attribute.len())
        .position(|window| window == attribute)
    {
        let start = at + found;
        at = start + attribute.len();
        let before = raw.get(start.wrapping_sub(1)).copied().unwrap_or(b'<');
        if !matches!(before, b' ' | b'\t' | b'\n' | b'\r') {
            continue;
        }
        let Some(rest) = raw[at..].trim_ascii_start().strip_prefix(b"=") else {
            continue;
        };
        let rest = rest.trim_ascii_start();
        let quote = *rest.first()?;
        let value = &rest[1..];
        let end = value.iter().position(|byte| *byte == quote)?;
        return Some(value[..end].to_vec());
    }
    None
}

/// Writes `package` to `output` through a temporary in its directory and
/// moves it into place, removing the temporary when either step fails, as
/// every output of this tool is written.
fn write_into<R: io::Read + io::Seek>(package: Package<R>, output: &Path) -> Result<(), Error> {
    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

/// The names sample (`specs/0019-names-links-properties-protection.md`,
/// "Harness runs", `--names-sample`): the two names added on W's sheet name,
/// the output written and read back, then the local name removed in memory
/// and that read back. A failure after the output is in place removes it; a
/// refusal by the layer is exit 2 with no output.
fn names_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let (at, sheet) = workbook
        .sheets
        .iter()
        .enumerate()
        .find(|(_, sheet)| sheet.kind == SheetKind::Worksheet)
        .map(|(at, sheet)| (at, sheet.name.clone()))
        .ok_or(Error::NoWorksheet)?;
    let at = u32::try_from(at).map_err(|_| Error::NamesSample("sheet index"))?;
    let names_before = workbook.defined_names().len() as u64;
    let local_before = local_names(workbook.defined_names(), at);
    let chain_before = workbook.calc_chain.is_some();
    let quoted = format!("'{}'!", sheet.replace('\'', "''"));
    package.add_defined_name("emend_global", &format!("{quoted}$A$1"), None)?;
    package.add_defined_name("emend_local", &format!("{quoted}$B$2"), Some(&sheet))?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = names_read_back(output, at, &sheet, names_before, local_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (names_after, local_after, chain_after, full_calc, names_removed) = checked?;
    edit_line_named(
        "names",
        &[
            ("names_before", names_before),
            ("names_after", names_after),
            ("local_after", local_after),
            ("chain_before", u64::from(chain_before)),
            ("chain_after", u64::from(chain_after)),
            ("full_calc", u64::from(full_calc)),
            ("names_removed", names_removed),
        ],
    )
}

/// The link sample (`specs/0019-names-links-properties-protection.md`,
/// "Harness runs", `--link-sample`): a text cell and two links on the row
/// past W's last, the output written and read back. A failure after the
/// output is in place removes it; a refusal by the layer is exit 2 with no
/// output.
fn link_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let (part, sheet) = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| (sheet.part.clone(), sheet.name.clone()))
        .ok_or(Error::NoWorksheet)?;
    let before = sample(&mut package, &part)?;
    let (links_before, r_bound_before) = link_counts(&mut package, &part)?;
    let external_before = external_links(&mut package, &part)?;
    let row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
    let first = CellRef { row, col: 1 };
    let second = CellRef { row, col: 2 };
    let location = format!("'{}'!A1", sheet.replace('\'', "''"));
    package.set_cell(&part, first, Value::Text("emend link".to_string()));
    package.set_hyperlink(
        &part,
        first,
        LinkTarget::External("https://example.com/"),
        Some("emend sample"),
    )?;
    package.set_hyperlink(&part, second, LinkTarget::Location(&location), None)?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = link_read_back(output, &part, before.cells, links_before, external_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (cells_after, links_after, external_after) = checked?;
    edit_line_named(
        "link",
        &[
            ("cells_before", before.cells),
            ("cells_after", cells_after),
            ("links_before", links_before),
            ("links_after", links_after),
            ("external_rels_before", external_before),
            ("external_rels_after", external_after),
            ("r_bound_before", u64::from(r_bound_before)),
        ],
    )
}

/// Reopens the link sample's output and asserts one more cell in W, two more
/// `hyperlink` elements and one more external hyperlink relationship.
fn link_read_back(
    output: &Path,
    part: &[u8],
    cells_before: u64,
    links_before: u64,
    external_before: u64,
) -> Result<(u64, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let cells_after = sample(&mut package, part)?.cells;
    let (links_after, _) = link_counts(&mut package, part)?;
    let external_after = external_links(&mut package, part)?;
    if cells_after != cells_before + 1 {
        return Err(Error::LinkSample("cell count"));
    }
    if links_after != links_before + 2 {
        return Err(Error::LinkSample("hyperlink count"));
    }
    if external_after != external_before + 1 {
        return Err(Error::LinkSample("external hyperlink relationship count"));
    }
    Ok((cells_after, links_after, external_after))
}

/// The `hyperlink` elements of the part's main-namespace `hyperlinks`
/// children of the root, and whether the root binds a prefix to the
/// relationships namespace, by this tool's own pass over the part's tokens.
fn link_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<(u64, bool), Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut root: Vec<u8> = Vec::new();
    let mut in_links = false;
    let mut links = 0u64;
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            depth = depth.saturating_sub(1);
            if depth < 2 {
                in_links = false;
            }
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        let local = tag_local_name(raw);
        if depth == 1 {
            root = raw.to_vec();
        } else if depth == 2 {
            in_links =
                local == b"hyperlinks" && MAIN.contains(&element_namespace(raw, &root).as_slice());
        } else if depth == 3 && in_links && local == b"hyperlink" {
            links += 1;
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
            if depth < 2 {
                in_links = false;
            }
        }
    }
    Ok((links, binds_relationships(&root)))
}

/// Whether a start tag declares a prefix bound to the relationships
/// namespace, in either form.
fn binds_relationships(tag: &[u8]) -> bool {
    let mut at = 0;
    while let Some(found) = tag[at..]
        .windows(b"xmlns:".len())
        .position(|window| window == b"xmlns:")
    {
        let start = at + found;
        at = start + b"xmlns:".len();
        let name_len = tag[at..]
            .iter()
            .position(|byte| matches!(byte, b'=' | b' ' | b'\t' | b'\n' | b'\r'))
            .unwrap_or(0);
        let attribute = [b"xmlns:".as_slice(), &tag[at..at + name_len]].concat();
        if tag_attribute(tag, &attribute)
            .is_some_and(|value| RELATIONSHIPS_ATTRIBUTE.contains(&value.as_slice()))
        {
            return true;
        }
    }
    false
}

/// The part's external relationships of the hyperlink type, in either form.
fn external_links<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<u64, Error> {
    let rels = package.relationships(part)?;
    Ok(HYPERLINK
        .into_iter()
        .map(|form| rels.of_kind(form).filter(|rel| rel.external).count() as u64)
        .sum())
}

/// The validation sample (`specs/0020-validation-and-conditional-
/// formatting.md`, "Harness runs", `--validate-sample`): two validations on
/// the rows past W's last, the output written and read back, then the first
/// removed in memory and that read back. A failure after the output is in
/// place removes it; a refusal by the layer is exit 2 with no output.
fn validate_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let max_row = sample(&mut package, &part)?.max_row;
    let dv_before = package.validations(&part)?.len() as u64;
    let (container_before, _) = validation_container(&mut package, &part)?;
    // A row past the grid is the layer's to refuse, with its index message.
    let row = max_row.saturating_add(1);
    let listed = [cell_area(row, 1, row, 1)];
    package.add_validation(
        &part,
        &listed,
        Validation {
            kind: ValidationKind::List,
            operator: None,
            formula1: "\"emend,sample\"",
            formula2: None,
            allow_blank: true,
            hide_drop_down: false,
            input: Some(("emend", "pick one")),
            error: Some((ErrorStyle::Stop, "emend", "not in the list")),
        },
    )?;
    package.add_validation(
        &part,
        &[cell_area(row, 2, row.saturating_add(1), 2)],
        Validation {
            kind: ValidationKind::Whole,
            operator: Some(Operator::Between),
            formula1: "1",
            formula2: Some("10"),
            allow_blank: false,
            hide_drop_down: false,
            input: None,
            error: None,
        },
    )?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = validate_read_back(output, &part, &listed, dv_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (dv_after, count_after, dv_removed) = checked?;
    edit_line_named(
        "validate",
        &[
            ("dv_before", dv_before),
            ("dv_after", dv_after),
            ("container_before", u64::from(container_before)),
            ("count_after", count_after),
            ("dv_removed", dv_removed),
        ],
    )
}

/// The area from row `first_row`, column `first_col` to row `last_row`,
/// column `last_col`.
fn cell_area(first_row: u32, first_col: u32, last_row: u32, last_col: u32) -> Area {
    Area {
        first: CellRef {
            row: first_row,
            col: first_col,
        },
        last: CellRef {
            row: last_row,
            col: last_col,
        },
    }
}

/// Reopens the validation sample's output and asserts two more
/// validations and a `count` equal to them; then removes the list
/// validation in memory, writes into a `Cursor`, reopens it and asserts one
/// more than the input's.
fn validate_read_back(
    output: &Path,
    part: &[u8],
    listed: &[Area],
    dv_before: u64,
) -> Result<(u64, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let dv_after = package.validations(part)?.len() as u64;
    if dv_after != dv_before + 2 {
        return Err(Error::ValidateSample("validation count"));
    }
    let count_after = validation_container(&mut package, part)?
        .1
        .ok_or(Error::ValidateSample("dataValidations count"))?;
    if count_after != dv_after {
        return Err(Error::ValidateSample("dataValidations count"));
    }
    package.remove_validation(part, listed)?;
    let mut buffer = io::Cursor::new(Vec::new());
    package.write_to(&mut buffer)?;
    buffer.set_position(0);
    let mut removed = Package::open(buffer)?;
    let dv_removed = removed.validations(part)?.len() as u64;
    if dv_removed != dv_before + 1 {
        return Err(Error::ValidateSample("validation count after the removal"));
    }
    Ok((dv_after, count_after, dv_removed))
}

/// Whether the part's root has a main-namespace `dataValidations` child,
/// and that child's `count` as a decimal number when it has one, by this
/// tool's own pass over the part's tokens.
fn validation_container<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<(bool, Option<u64>), Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut root: Vec<u8> = Vec::new();
    let mut found = (false, None);
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            depth = depth.saturating_sub(1);
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        if depth == 1 {
            root = raw.to_vec();
        } else if depth == 2
            && !found.0
            && tag_local_name(raw) == b"dataValidations"
            && MAIN.contains(&element_namespace(raw, &root).as_slice())
        {
            found = (true, tag_number(raw, b"count"));
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
        }
    }
    Ok(found)
}

/// The conditional format sample (`specs/0020-validation-and-conditional-
/// formatting.md`, "Harness runs", `--cf-sample`): a differential style and
/// two rules on the rows past W's last, the output written and read back,
/// then the second rule's conditional format removed in memory and that read
/// back. A failure after the output is in place removes it; a refusal by the
/// layer is exit 2 with no output.
fn cf_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let styles = workbook.styles.clone();
    let part = workbook
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let max_row = sample(&mut package, &part)?.max_row;
    let rules_before = package.conditional_formats(&part)?;
    let cf_before = rules_before.len() as u64;
    let priority_max_before = rules_before
        .iter()
        .map(|rule| rule.priority)
        .max()
        .unwrap_or(0);
    let (dxfs_present_before, dxfs_before) = match &styles {
        Some(styles) => dxf_counts(&mut package, styles)?,
        None => (false, 0),
    };
    let dxf = package.add_differential_style(&Style {
        font: Some(Font {
            bold: true,
            color: Some(Color::Rgb(0x9C0006)),
            ..Font::default()
        }),
        fill: Some(Fill {
            pattern: Pattern::Solid,
            foreground: None,
            background: Some(Color::Rgb(0xFFC7CE)),
        }),
        ..Style::default()
    })?;
    // A row past the grid is the layer's to refuse, with its index message.
    let row = max_row.saturating_add(1);
    let greater = [cell_area(row, 1, row, 3)];
    let expression = [cell_area(
        row.saturating_add(1),
        1,
        row.saturating_add(1),
        1,
    )];
    package.add_conditional_format(
        &part,
        &greater,
        Rule::CellIs {
            operator: Operator::GreaterThan,
            formula1: "5",
            formula2: None,
        },
        Some(dxf),
        None,
        false,
    )?;
    package.add_conditional_format(
        &part,
        &expression,
        Rule::Expression("TRUE"),
        Some(dxf),
        None,
        true,
    )?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = cf_read_back(
        output,
        &part,
        styles.as_deref(),
        (&greater, &expression),
        (cf_before, dxfs_before, priority_max_before),
    );
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (cf_after, dxfs_after, priority_first_after, cf_removed) = checked?;
    edit_line_named(
        "cf",
        &[
            ("cf_before", cf_before),
            ("cf_after", cf_after),
            ("dxfs_before", dxfs_before),
            ("dxfs_after", dxfs_after),
            ("dxfs_present_before", u64::from(dxfs_present_before)),
            ("priority_max_before", u64::from(priority_max_before)),
            ("priority_first_after", u64::from(priority_first_after)),
            ("cf_removed", cf_removed),
        ],
    )
}

/// Reopens the conditional format sample's output and asserts two more
/// rules, one more `dxf`, and the first added rule's priority one past the
/// input's highest; then removes the second rule's conditional format in
/// memory, writes into a `Cursor`, reopens it and asserts one more rule
/// than the input's.
fn cf_read_back(
    output: &Path,
    part: &[u8],
    styles: Option<&[u8]>,
    (greater, expression): (&[Area], &[Area]),
    (cf_before, dxfs_before, priority_max_before): (u64, u64, u32),
) -> Result<(u64, u64, u32, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let rules = package.conditional_formats(part)?;
    let cf_after = rules.len() as u64;
    if cf_after != cf_before + 2 {
        return Err(Error::CfSample("rule count"));
    }
    let styles = styles.ok_or(Error::CfSample("styles part"))?;
    let (_, dxfs_after) = dxf_counts(&mut package, styles)?;
    if dxfs_after != dxfs_before + 1 {
        return Err(Error::CfSample("dxf count"));
    }
    let priority_first_after = rules
        .iter()
        .find(|rule| rule.areas == greater)
        .map(|rule| rule.priority)
        .ok_or(Error::CfSample("first added rule"))?;
    if u64::from(priority_first_after) != u64::from(priority_max_before) + 1 {
        return Err(Error::CfSample("first added rule's priority"));
    }
    package.remove_conditional_format(part, expression)?;
    let mut buffer = io::Cursor::new(Vec::new());
    package.write_to(&mut buffer)?;
    buffer.set_position(0);
    let mut removed = Package::open(buffer)?;
    let cf_removed = removed.conditional_formats(part)?.len() as u64;
    if cf_removed != cf_before + 1 {
        return Err(Error::CfSample("rule count after the removal"));
    }
    Ok((cf_after, dxfs_after, priority_first_after, cf_removed))
}

/// Whether the styles part's root has a main-namespace `dxfs` child, and
/// how many `dxf` children the first one holds, by this tool's own pass
/// over the part's tokens.
fn dxf_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<(bool, u64), Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut root: Vec<u8> = Vec::new();
    let (mut present, mut inside, mut children) = (false, false, 0u64);
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            depth = depth.saturating_sub(1);
            if depth < 2 {
                inside = false;
            }
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        let local = tag_local_name(raw);
        if depth == 1 {
            root = raw.to_vec();
        } else if depth == 2 {
            inside = !present
                && local == b"dxfs"
                && MAIN.contains(&element_namespace(raw, &root).as_slice());
            present |= inside;
        } else if depth == 3 && inside && local == b"dxf" {
            children += 1;
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
            if depth < 2 {
                inside = false;
            }
        }
    }
    Ok((present, children))
}

/// The rule kinds `--scale-sample` counts as visual.
const VISUAL_KINDS: [&str; 3] = ["colorScale", "dataBar", "iconSet"];

/// The visual rule sample (`specs/0020-validation-and-conditional-
/// formatting.md`, "Harness runs", `--scale-sample`): a colour scale, a data
/// bar and an icon set on the rows past W's last, the output written and
/// read back. A failure after the output is in place removes it; a refusal
/// by the layer is exit 2 with no output.
fn scale_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let max_row = sample(&mut package, &part)?.max_row;
    let rules = package.conditional_formats(&part)?;
    let cf_before = rules.len() as u64;
    let visual_before = rules
        .iter()
        .filter(|rule| VISUAL_KINDS.contains(&rule.kind.as_str()))
        .count() as u64;
    let extended_before = rules.iter().filter(|rule| rule.extended).count() as u64;
    // A row past the grid is the layer's to refuse, with its index message.
    let first = max_row.saturating_add(1);
    let last = max_row.saturating_add(3);
    let thresholds = [
        CfvoKind::Percent(0.0),
        CfvoKind::Percent(33.0),
        CfvoKind::Percent(67.0),
    ];
    let visual = [
        Rule::ColorScale {
            min: Cfvo {
                kind: CfvoKind::Min,
                color: Color::Rgb(0xF8696B),
            },
            mid: None,
            max: Cfvo {
                kind: CfvoKind::Max,
                color: Color::Rgb(0x63BE7B),
            },
        },
        Rule::DataBar {
            min: CfvoKind::Min,
            max: CfvoKind::Max,
            color: Color::Rgb(0x638EC6),
            show_value: true,
        },
        Rule::IconSet {
            set: IconSetKind::ThreeArrows,
            thresholds: &thresholds,
            reverse: false,
            show_value: true,
        },
    ];
    for (column, rule) in (1u32..).zip(visual) {
        package.add_conditional_format(
            &part,
            &[cell_area(first, column, last, column)],
            rule,
            None,
            None,
            false,
        )?;
    }
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = scale_read_back(output, &part, cf_before, visual_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (cf_after, visual_after) = checked?;
    edit_line_named(
        "scale",
        &[
            ("cf_before", cf_before),
            ("cf_after", cf_after),
            ("visual_after", visual_after),
            ("extended_before", extended_before),
        ],
    )
}

/// Reopens the visual rule sample's output and asserts three more rules,
/// three more of them visual.
fn scale_read_back(
    output: &Path,
    part: &[u8],
    cf_before: u64,
    visual_before: u64,
) -> Result<(u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let rules = package.conditional_formats(part)?;
    let cf_after = rules.len() as u64;
    if cf_after != cf_before + 3 {
        return Err(Error::ScaleSample("rule count"));
    }
    let visual_after = rules
        .iter()
        .filter(|rule| VISUAL_KINDS.contains(&rule.kind.as_str()))
        .count() as u64;
    if visual_after != visual_before + 3 {
        return Err(Error::ScaleSample("visual rule count"));
    }
    Ok((cf_after, visual_after))
}

/// The table sample (`specs/0021-tables.md`, "Harness runs",
/// `--table-sample`): a table on the rows past W's last, the output written
/// and read back, then the table removed in memory and that read back. A
/// failure after the output is in place removes it; a
/// refusal by the layer is exit 2 with no output.
fn table_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let before = TableCounts {
        tables: package.tables(&part)?.len() as u64,
        parts: table_entries(&package),
        rels: table_relationships(&mut package, &part)?,
        cells: sampled.cells,
    };
    let (container_before, r_bound_before) =
        container_and_binding(&mut package, &part, b"tableParts")?;
    // A row past the grid is the layer's to refuse, with its index message.
    let row = sampled.max_row.saturating_add(1);
    package.add_table(
        &part,
        cell_area(row, 1, row.saturating_add(2), 2),
        Table {
            name: SAMPLE_TABLE,
            columns: &["emend_a", "emend_b"],
            style: Some(TableStyleInfo {
                style: TableStyle::Medium(2),
                first_column: false,
                last_column: false,
                row_stripes: true,
                column_stripes: false,
            }),
        },
    )?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = table_read_back(output, &part, &before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, id_after, removed) = checked?;
    edit_line_named(
        "table",
        &[
            ("tables_before", before.tables),
            ("tables_after", after.tables),
            ("parts_before", before.parts),
            ("parts_after", after.parts),
            ("rels_before", before.rels),
            ("rels_after", after.rels),
            ("container_before", u64::from(container_before)),
            ("r_bound_before", u64::from(r_bound_before)),
            ("id_after", id_after),
            ("cells_before", before.cells),
            ("cells_after", after.cells),
            ("tables_removed", removed.tables),
            ("parts_removed", removed.parts),
        ],
    )
}

/// The table sample's name, the tool's constant.
const SAMPLE_TABLE: &str = "emend_table";

/// What the table sample counts on each side of the write.
struct TableCounts {
    tables: u64,
    parts: u64,
    rels: u64,
    cells: u64,
}

/// Reopens the table sample's output and asserts one more table, table
/// part and table relationship, and two more cells; then removes the table
/// in memory, writes into a `Cursor`, reopens it and asserts the tables and
/// the table parts the input had; gives the counts, the added table's `id`
/// and the counts after the removal.
fn table_read_back(
    output: &Path,
    part: &[u8],
    before: &TableCounts,
) -> Result<(TableCounts, u64, TableCounts), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let tables = package.tables(part)?;
    let after = TableCounts {
        tables: tables.len() as u64,
        parts: table_entries(&package),
        rels: table_relationships(&mut package, part)?,
        cells: sample(&mut package, part)?.cells,
    };
    if after.tables != before.tables + 1 {
        return Err(Error::TableSample("table count"));
    }
    if after.parts != before.parts + 1 {
        return Err(Error::TableSample("table part count"));
    }
    if after.rels != before.rels + 1 {
        return Err(Error::TableSample("table relationship count"));
    }
    if after.cells != before.cells + 2 {
        return Err(Error::TableSample("cell count"));
    }
    let added = tables
        .iter()
        .find(|table| table.name == SAMPLE_TABLE)
        .ok_or(Error::TableSample("table"))?;
    let index = package.part(&added.part).ok_or(Error::NoSuchName)?;
    let id = {
        let mut stream = Stream::new(package.read_part(index)?, index)?;
        let mut id = None;
        while let Some(token) = stream.next()? {
            let raw = token.raw;
            if raw.starts_with(b"<") && !raw.starts_with(b"<?") && !raw.starts_with(b"<!") {
                id = tag_number(raw, b"id");
                break;
            }
        }
        id.ok_or(Error::TableSample("table id"))?
    };
    package.remove_table(part, SAMPLE_TABLE)?;
    let mut buffer = io::Cursor::new(Vec::new());
    package.write_to(&mut buffer)?;
    buffer.set_position(0);
    let mut reopened = Package::open(buffer)?;
    let removed = TableCounts {
        tables: reopened.tables(part)?.len() as u64,
        parts: table_entries(&reopened),
        rels: table_relationships(&mut reopened, part)?,
        cells: 0,
    };
    if removed.tables != before.tables {
        return Err(Error::TableSample("table count after the removal"));
    }
    if removed.parts != before.parts {
        return Err(Error::TableSample("table part count after the removal"));
    }
    Ok((after, id, removed))
}

/// The comment sample (`specs/0022-comments-and-images.md`, "Harness
/// runs", `--comment-sample`): a comment on the cell below W's last row, the
/// output written and read back, then the comment removed in memory and
/// that read back (P2). A failure after the output is in place removes it;
/// a refusal by the layer is exit 2 with no output.
fn comment_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let before = comment_counts(&mut package, &part)?;
    let (legacy_before, r_bound_before) =
        container_and_binding(&mut package, &part, b"legacyDrawing")?;
    let comments_part_before = related(&mut package, &part, COMMENTS)?.is_some();
    // A row past the grid is the layer's to refuse, with its index message.
    let row = sampled.max_row.saturating_add(1);
    let cell = CellRef { row, col: 1 };
    package.add_comment(
        &part,
        cell,
        Comment {
            author: SAMPLE_AUTHOR,
            text: "emend comment",
        },
    )?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let parts_more = match (legacy_before, comments_part_before) {
        (false, _) => 2,
        (true, false) => 1,
        (true, true) => 0,
    };
    let checked = comment_read_back(output, &part, cell, &before, parts_more);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, removed) = checked?;
    edit_line_named(
        "comment",
        &[
            ("comments_before", before.comments),
            ("comments_after", after.comments),
            ("parts_before", before.parts),
            ("parts_after", after.parts),
            ("legacy_before", u64::from(legacy_before)),
            ("comments_part_before", u64::from(comments_part_before)),
            ("r_bound_before", u64::from(r_bound_before)),
            ("shapes_before", before.shapes),
            ("shapes_after", after.shapes),
            ("authors_before", before.authors),
            ("authors_after", after.authors),
            ("comments_removed", removed.0),
            ("parts_removed", removed.1),
        ],
    )
}

/// The comment sample's author, the tool's constant.
const SAMPLE_AUTHOR: &str = "emend";

/// The comment box sample's box, the tool's constant
/// (`specs/0028-images-and-comments-second-half.md`, "Harness runs").
const SAMPLE_BOX: CommentBox = CommentBox {
    width: 200,
    height: 100,
    visible: true,
};

/// The comment box sample (`specs/0028-images-and-comments-second-half.md`,
/// "Harness runs", `--commentbox-sample`): a comment on the cell below W's
/// last row with the tool's box, the output written and read back. A failure
/// after the output is in place removes it; a refusal by the layer is exit 2
/// with no output.
fn commentbox_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let kept = package.comments(&part)?;
    let comments_before = kept.len() as u64;
    let sized_before = kept.iter().filter(|entry| entry.shape.is_some()).count() as u64;
    let shapes_before = match related(&mut package, &part, VML_DRAWING)? {
        Some(target) => note_shapes(&mut package, &target)?,
        None => 0,
    };
    // A row past the grid is the layer's to refuse, with its index message.
    let row = sampled.max_row.saturating_add(1);
    let cell = CellRef { row, col: 1 };
    package.add_comment(
        &part,
        cell,
        Comment {
            author: SAMPLE_AUTHOR,
            text: "emend box",
        },
    )?;
    package.set_comment_box(&part, cell, SAMPLE_BOX)?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = commentbox_read_back(output, &part, cell, comments_before, shapes_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (comments_after, shapes_after, box_equal, visible_after) = checked?;
    edit_line_named(
        "commentbox",
        &[
            ("comments_before", comments_before),
            ("comments_after", comments_after),
            ("shapes_before", shapes_before),
            ("shapes_after", shapes_after),
            ("sized_before", sized_before),
            ("box_equal", box_equal),
            ("visible_after", visible_after),
        ],
    )
}

/// Reopens the comment box sample's output and asserts one more comment and
/// note shape on W and the added comment's box read back equal and shown;
/// gives the comments, the note shapes and the two flags.
fn commentbox_read_back(
    output: &Path,
    part: &[u8],
    cell: CellRef,
    comments_before: u64,
    shapes_before: u64,
) -> Result<(u64, u64, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let read = package.comments(part)?;
    let comments_after = read.len() as u64;
    if comments_after != comments_before + 1 {
        return Err(Error::CommentboxSample("comment count"));
    }
    let shapes_after = match related(&mut package, part, VML_DRAWING)? {
        Some(target) => note_shapes(&mut package, &target)?,
        None => 0,
    };
    if shapes_after != shapes_before + 1 {
        return Err(Error::CommentboxSample("note shape count"));
    }
    let shape = read
        .iter()
        .find(|entry| entry.cell == cell)
        .ok_or(Error::CommentboxSample("written comment"))?
        .shape;
    let box_equal = u64::from(shape == Some(SAMPLE_BOX));
    if box_equal != 1 {
        return Err(Error::CommentboxSample("written comment's box"));
    }
    let visible_after = u64::from(shape.is_some_and(|shape| shape.visible));
    if visible_after != 1 {
        return Err(Error::CommentboxSample("written comment's visibility"));
    }
    Ok((comments_after, shapes_after, box_equal, visible_after))
}

/// The name the sheet copy sample gives its copy.
const COPY_NAME: &str = "emend copy";

/// What the sheet copy sample counts on a sheet: its cells, comments, merges
/// and hyperlinks, and whether a cell has a formula.
struct CopyCounts {
    cells: u64,
    comments: u64,
    merges: u64,
    links: u64,
    formulas: bool,
}

/// The sheet copy sample's counts for the worksheet `part`.
fn copy_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<CopyCounts, Error> {
    let mut cells = 0u64;
    let mut formulas = false;
    for cell in package.cells(part)? {
        let cell = cell?;
        cells += 1;
        formulas |= cell.formula.is_some();
    }
    let comments = package.comments(part)?.len() as u64;
    let settings = package.sheet_settings(part)?;
    Ok(CopyCounts {
        cells,
        comments,
        merges: settings.merges.len() as u64,
        links: settings.hyperlinks.len() as u64,
        formulas,
    })
}

/// The sheet copy sample (`specs/0028-images-and-comments-second-half.md`,
/// "Harness runs", `--copy-sample`): S, the first worksheet with a comments
/// relationship or W, copied under the tool's name, the output written and
/// read back. A failure after the output is in place removes it; a refusal by
/// the layer is exit 2 with no output.
fn copy_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let sheets: Vec<_> = workbook
        .sheets
        .iter()
        .filter(|sheet| sheet.kind == SheetKind::Worksheet)
        .cloned()
        .collect();
    let sheets_before = workbook.sheets.len() as u64;
    let chain_before = workbook.calc_chain.is_some();
    let mut source = None;
    for sheet in &sheets {
        if related(&mut package, &sheet.part, COMMENTS)?.is_some() {
            source = Some(sheet.clone());
            break;
        }
    }
    let source = match source {
        Some(sheet) => sheet,
        None => sheets.first().cloned().ok_or(Error::NoWorksheet)?,
    };
    let before = copy_counts(&mut package, &source.part)?;
    package.copy_sheet(&source.name, COPY_NAME)?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = copy_read_back(output, &before, sheets_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (sheets_after, after, chain_after) = checked?;
    edit_line_named(
        "copy",
        &[
            ("sheets_before", sheets_before),
            ("sheets_after", sheets_after),
            ("cells_source", before.cells),
            ("cells_copy", after.cells),
            ("comments_source", before.comments),
            ("comments_copy", after.comments),
            ("merges_source", before.merges),
            ("merges_copy", after.merges),
            ("links_source", before.links),
            ("links_copy", after.links),
            ("formulas", u64::from(before.formulas)),
            ("chain_before", u64::from(chain_before)),
            ("chain_after", u64::from(chain_after)),
        ],
    )
}

/// Reopens the sheet copy sample's output and asserts one more sheet, the
/// copy's counts equal to the source's and no chain left when the source
/// has a formula; gives the sheets, the copy's counts and whether a chain is
/// left.
fn copy_read_back(
    output: &Path,
    before: &CopyCounts,
    sheets_before: u64,
) -> Result<(u64, CopyCounts, bool), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let sheets_after = workbook.sheets.len() as u64;
    let chain_after = workbook.calc_chain.is_some();
    let part = workbook
        .sheet(COPY_NAME)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::CopySample("copied sheet"))?;
    if sheets_after != sheets_before + 1 {
        return Err(Error::CopySample("sheet count"));
    }
    let after = copy_counts(&mut package, &part)?;
    if after.cells != before.cells {
        return Err(Error::CopySample("cell count"));
    }
    if after.comments != before.comments {
        return Err(Error::CopySample("comment count"));
    }
    if after.merges != before.merges {
        return Err(Error::CopySample("merge count"));
    }
    if after.links != before.links {
        return Err(Error::CopySample("hyperlink count"));
    }
    if before.formulas && chain_after {
        return Err(Error::CopySample("calculation chain"));
    }
    Ok((sheets_after, after, chain_after))
}

/// The image sample's PNG, the tool's constant (`specs/0022-comments-and-
/// images.md`, "Harness runs", and P3 ruling 2): 2 by 2 pixels, RGB, red,
/// green, blue and white from the top left, 82 bytes, one stored deflate
/// block. A committed fixture, produced by Python 3.14.4's `zlib` and
/// `struct` with this program, its standard output being the bytes:
///
/// ```text
/// import struct, sys, zlib
/// def chunk(kind, data): return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data))
/// raw = bytes([0, 255, 0, 0, 0, 255, 0, 0, 0, 0, 255, 255, 255, 255])
/// ihdr = struct.pack('>IIBBBBB', 2, 2, 8, 2, 0, 0, 0)
/// png = b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', ihdr) + chunk(b'IDAT', zlib.compress(raw, 0)) + chunk(b'IEND', b'')
/// sys.stdout.buffer.write(png)
/// ```
const SAMPLE_PNG: [u8; 82] = [
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x08, 0x02, 0x00, 0x00, 0x00, 0xfd, 0xd4, 0x9a,
    0x73, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, 0x54, 0x78, 0x01, 0x01, 0x0e, 0x00, 0xf1, 0xff,
    0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x1f, 0xee,
    0x05, 0xfb, 0xde, 0xdd, 0xec, 0x2b, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42,
    0x60, 0x82,
];

/// The image sample (`specs/0022-comments-and-images.md`, "Harness runs",
/// `--image-sample`): the tool's PNG on the cell below W's last row, the
/// output written and read back. A failure after the output is in place
/// removes it; a refusal by the layer is exit 2 with no output.
fn image_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let before = image_counts(&mut package, &part)?;
    let (drawing_before, r_bound_before) = container_and_binding(&mut package, &part, b"drawing")?;
    // A row past the grid is the layer's to refuse, with its index message.
    let row = sampled.max_row.saturating_add(1);
    package.add_image(
        &part,
        CellRef { row, col: 1 },
        Image {
            bytes: &SAMPLE_PNG,
            size: None,
        },
    )?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = image_read_back(output, &part, &before, sampled.max_row);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, from) = checked?;
    let mut line = String::from("image");
    for (key, value) in [
        ("images_before", before.images),
        ("images_after", after.images),
        ("drawing_before", u64::from(drawing_before)),
        ("anchors_before", before.anchors),
        ("anchors_after", after.anchors),
        ("media_before", before.media.len() as u64),
        ("media_after", after.media.len() as u64),
        ("r_bound_before", u64::from(r_bound_before)),
    ] {
        line.push_str(&format!("\t{key}={value}"));
    }
    line.push_str(&format!("\tfrom_after={},{}", from.0, from.1));
    print_line(&line)
}

/// The rich text sample (`specs/0024-rich-text.md`, "Harness runs",
/// `--richtext-sample`): a rich value of two runs on the cell below W's last
/// row, the output written and read back. A failure after the output is in
/// place removes it; a refusal by the layer is exit 2 with no output.
fn richtext_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let sst_before = sst_rich(&mut package)?;
    let inline_before = inline_rich(&mut package, &part, None)?.0;
    let (_, r_bound_before) = container_and_binding(&mut package, &part, b"sheetData")?;
    // A row past the grid is the layer's to refuse, with its index message.
    let row = sampled.max_row.saturating_add(1);
    let cell = CellRef { row, col: 1 };
    package.set_cell(
        &part,
        cell,
        Value::Rich(RichText {
            runs: vec![
                Run {
                    text: "emend".to_string(),
                    font: Some(Font {
                        bold: true,
                        ..Font::default()
                    }),
                },
                Run {
                    text: " rich".to_string(),
                    font: None,
                },
            ],
        }),
    );
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = richtext_read_back(output, &part, cell, sst_before, inline_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (sst_after, inline_after, runs_after, text_after, bold_first) = checked?;
    edit_line_named(
        "richtext",
        &[
            ("sst_rich_before", sst_before),
            ("sst_rich_after", sst_after),
            ("inline_rich_before", inline_before),
            ("inline_rich_after", inline_after),
            ("runs_after", runs_after),
            ("text_after", u64::from(text_after)),
            ("bold_first", u64::from(bold_first)),
            ("r_bound_before", u64::from(r_bound_before)),
        ],
    )
}

/// The shared strings items `SharedStrings::runs_of` gives runs for.
fn sst_rich<R: io::Read + io::Seek>(package: &mut Package<R>) -> Result<u64, Error> {
    let table = package.shared_strings()?;
    Ok((0..table.len())
        .filter(|index| table.runs_of(*index).is_some())
        .count() as u64)
}

/// W's cells whose `runs` is `Some`, and the runs and value of the cell at
/// `at` when one is named and read.
fn inline_rich<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
    at: Option<CellRef>,
) -> Result<(u64, Option<RichText>, Option<String>), Error> {
    let mut count = 0;
    let mut runs = None;
    let mut value = None;
    for cell in package.cells(part)? {
        let cell = cell?;
        if cell.runs.is_some() {
            count += 1;
        }
        if Some(cell.reference) == at {
            runs = cell.runs;
            value = cell.value;
        }
    }
    Ok((count, runs, value))
}

/// Reopens the rich text sample's output and asserts the table's rich items
/// unchanged, one more rich cell on W, and the written cell's two runs, its
/// text and its fonts; gives the counts and the two flags.
fn richtext_read_back(
    output: &Path,
    part: &[u8],
    cell: CellRef,
    sst_before: u64,
    inline_before: u64,
) -> Result<(u64, u64, u64, bool, bool), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let sst_after = sst_rich(&mut package)?;
    if sst_after != sst_before {
        return Err(Error::RichtextSample("shared strings rich item count"));
    }
    let (inline_after, runs, value) = inline_rich(&mut package, part, Some(cell))?;
    if inline_after != inline_before + 1 {
        return Err(Error::RichtextSample("rich cell count"));
    }
    let runs = runs.ok_or(Error::RichtextSample("written cell's runs"))?;
    let runs_after = runs.runs.len() as u64;
    if runs_after != 2 {
        return Err(Error::RichtextSample("written cell's run count"));
    }
    let text_after = value.as_deref() == Some("emend rich");
    if !text_after {
        return Err(Error::RichtextSample("written cell's text"));
    }
    let bold_first =
        runs.runs[0].font.as_ref().is_some_and(|font| font.bold) && runs.runs[1].font.is_none();
    if !bold_first {
        return Err(Error::RichtextSample("written cell's fonts"));
    }
    Ok((sst_after, inline_after, runs_after, text_after, bold_first))
}

/// The rich comment sample (`specs/0024-rich-text.md`, "Harness runs",
/// `--richcomment-sample`): a comment of two runs on the cell below W's last
/// row, the output written and read back. A failure after the output is in
/// place removes it; a refusal by the layer is exit 2 with no output.
fn richcomment_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let before = comment_counts(&mut package, &part)?;
    let rich_before = rich_comments(&mut package, &part, None)?.0;
    let (legacy_before, r_bound_before) =
        container_and_binding(&mut package, &part, b"legacyDrawing")?;
    let comments_part_before = related(&mut package, &part, COMMENTS)?.is_some();
    // A row past the grid is the layer's to refuse, with its index message.
    let row = sampled.max_row.saturating_add(1);
    let cell = CellRef { row, col: 1 };
    package.add_rich_comment(
        &part,
        cell,
        SAMPLE_AUTHOR,
        &RichText {
            runs: vec![
                Run {
                    text: "emend:".to_string(),
                    font: Some(Font {
                        bold: true,
                        ..Font::default()
                    }),
                },
                Run {
                    text: " rich comment".to_string(),
                    font: None,
                },
            ],
        },
    )?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let parts_more = match (legacy_before, comments_part_before) {
        (false, _) => 2,
        (true, false) => 1,
        (true, true) => 0,
    };
    let checked = richcomment_read_back(output, &part, cell, &before, parts_more, rich_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, rich_after, runs_after, text_after) = checked?;
    edit_line_named(
        "richcomment",
        &[
            ("comments_before", before.comments),
            ("comments_after", after.comments),
            ("parts_before", before.parts),
            ("parts_after", after.parts),
            ("legacy_before", u64::from(legacy_before)),
            ("comments_part_before", u64::from(comments_part_before)),
            ("r_bound_before", u64::from(r_bound_before)),
            ("shapes_before", before.shapes),
            ("shapes_after", after.shapes),
            ("authors_before", before.authors),
            ("authors_after", after.authors),
            ("rich_comments_before", rich_before),
            ("rich_comments_after", rich_after),
            ("runs_after", runs_after),
            ("text_after", u64::from(text_after)),
        ],
    )
}

/// The entries `Package::comments` reads from W whose `runs` is `Some`, and
/// the entry on `at` when one is named and read.
fn rich_comments<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
    at: Option<CellRef>,
) -> Result<(u64, Option<CommentEntry>), Error> {
    let mut count = 0;
    let mut found = None;
    for entry in package.comments(part)? {
        if entry.runs.is_some() {
            count += 1;
        }
        if Some(entry.cell) == at {
            found = Some(entry);
        }
    }
    Ok((count, found))
}

/// Reopens the rich comment sample's output and asserts the comment
/// sample's counts, one more rich comment on W, and the written comment's
/// two runs and its text; gives the counts and the flag.
fn richcomment_read_back(
    output: &Path,
    part: &[u8],
    cell: CellRef,
    before: &CommentCounts,
    parts_more: u64,
    rich_before: u64,
) -> Result<(CommentCounts, u64, u64, bool), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = comment_counts_after(&mut package, part, before, parts_more)?;
    let (rich_after, entry) = rich_comments(&mut package, part, Some(cell))?;
    if rich_after != rich_before + 1 {
        return Err(Error::RichcommentSample("rich comment count"));
    }
    let entry = entry.ok_or(Error::RichcommentSample("written comment"))?;
    let runs = entry
        .runs
        .ok_or(Error::RichcommentSample("written comment's runs"))?;
    let runs_after = runs.runs.len() as u64;
    if runs_after != 2 {
        return Err(Error::RichcommentSample("written comment's run count"));
    }
    let text_after = entry.text == "emend: rich comment";
    if !text_after {
        return Err(Error::RichcommentSample("written comment's text"));
    }
    Ok((after, rich_after, runs_after, text_after))
}

/// What the image sample counts on each side of the write: W's images, the
/// root children of W's drawing part, and the archive's media entries by
/// name.
struct ImageCounts {
    images: u64,
    anchors: u64,
    media: Vec<Vec<u8>>,
}

/// The image sample's counts of W.
fn image_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<ImageCounts, Error> {
    let images = package.images(part)?.len() as u64;
    let anchors = match related(package, part, DRAWING)? {
        Some(target) => drawing_root_children(package, &target)?,
        None => 0,
    };
    let media = package
        .archive()
        .entries()
        .iter()
        .map(|entry| entry.name().to_vec())
        .filter(|name| name.starts_with(b"xl/media/"))
        .collect();
    Ok(ImageCounts {
        images,
        anchors,
        media,
    })
}

/// Reopens the image sample's output and asserts one more image, anchor and
/// media entry, the added entry's `from` being (0, `max_row`); gives the
/// counts and that `from`.
fn image_read_back(
    output: &Path,
    part: &[u8],
    before: &ImageCounts,
    max_row: u32,
) -> Result<(ImageCounts, (u32, u32)), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = image_counts(&mut package, part)?;
    if after.images != before.images + 1 {
        return Err(Error::ImageSample("image count"));
    }
    if after.anchors != before.anchors + 1 {
        return Err(Error::ImageSample("drawing anchor count"));
    }
    if after.media.len() != before.media.len() + 1 {
        return Err(Error::ImageSample("media entry count"));
    }
    // The added entry: the image whose media entry the input lacks.
    let added: Vec<&Vec<u8>> = after
        .media
        .iter()
        .filter(|name| !before.media.contains(name))
        .collect();
    let [added] = added.as_slice() else {
        return Err(Error::ImageSample("media entry count"));
    };
    let from = package
        .images(part)?
        .into_iter()
        .find(|entry| entry.part == **added)
        .map(|entry| entry.from)
        .ok_or(Error::ImageSample("added image"))?;
    if from != (0, max_row) {
        return Err(Error::ImageSample("added image's anchor"));
    }
    Ok((after, from))
}

/// The root children of a drawing part, by this tool's own pass over its
/// tokens.
fn drawing_root_children<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<u64, Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut count = 0u64;
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            depth = depth.saturating_sub(1);
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        if depth == 2 {
            count += 1;
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
        }
    }
    Ok(count)
}

/// The anchor sample (`specs/0028-images-and-comments-second-half.md`,
/// "Harness runs", `--anchor-sample`): the tool's PNG on a two-cell and on
/// an absolute anchor, the output written and read back. A failure after the
/// output is in place removes it; a refusal by the layer is exit 2 with no
/// output.
fn anchor_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = first_worksheet(&mut package)?;
    let sampled = sample(&mut package, &part)?;
    let before = image_counts(&mut package, &part)?;
    let kinds_before = anchor_kinds(&mut package, &part)?;
    // Rows past the grid are the layer's to refuse, with its index message.
    let row = |more: u32| sampled.max_row.saturating_add(more);
    package.add_image_at(
        &part,
        ImageAnchor::TwoCell {
            from: CellRef {
                row: row(1),
                col: 1,
            },
            to: CellRef {
                row: row(3),
                col: 3,
            },
            from_offset: (0, 0),
            to_offset: (0, 0),
            edit_as: EditAs::OneCell,
        },
        Image {
            bytes: &SAMPLE_PNG,
            size: None,
        },
    )?;
    package.add_image_at(
        &part,
        ImageAnchor::Absolute { position: (96, 96) },
        Image {
            bytes: &SAMPLE_PNG,
            size: Some((4, 4)),
        },
    )?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = anchor_read_back(output, &part, &before, kinds_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, (two_after, absolute_after)) = checked?;
    edit_line_named(
        "anchor",
        &[
            ("images_before", before.images),
            ("images_after", after.images),
            ("two_after", two_after),
            ("absolute_after", absolute_after),
            ("anchors_before", before.anchors),
            ("anchors_after", after.anchors),
            ("media_before", before.media.len() as u64),
            ("media_after", after.media.len() as u64),
        ],
    )
}

/// The first worksheet in `<sheets>` order, W.
fn first_worksheet<R: io::Read + io::Seek>(package: &mut Package<R>) -> Result<Vec<u8>, Error> {
    package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)
}

/// The images of W on a two-cell and on an absolute anchor.
fn anchor_kinds<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<(u64, u64), Error> {
    let images = package.images(part)?;
    let of = |kind: Anchor| images.iter().filter(|entry| entry.anchor == kind).count() as u64;
    Ok((of(Anchor::TwoCell), of(Anchor::Absolute)))
}

/// Reopens the anchor sample's output and asserts two more images, anchors
/// and media entries, one more of each anchor kind; gives the counts and the
/// kinds after.
fn anchor_read_back(
    output: &Path,
    part: &[u8],
    before: &ImageCounts,
    kinds_before: (u64, u64),
) -> Result<(ImageCounts, (u64, u64)), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = image_counts(&mut package, part)?;
    if after.images != before.images + 2 {
        return Err(Error::AnchorSample("image count"));
    }
    if after.anchors != before.anchors + 2 {
        return Err(Error::AnchorSample("drawing anchor count"));
    }
    if after.media.len() != before.media.len() + 2 {
        return Err(Error::AnchorSample("media entry count"));
    }
    let kinds = anchor_kinds(&mut package, part)?;
    if kinds.0 != kinds_before.0 + 1 {
        return Err(Error::AnchorSample("two-cell image count"));
    }
    if kinds.1 != kinds_before.1 + 1 {
        return Err(Error::AnchorSample("absolute image count"));
    }
    Ok((after, kinds))
}

/// The image removal sample (`specs/0028-images-and-comments-second-
/// half.md`, "Harness runs", `--remove-image-sample`): the tool's PNG added
/// below W's last row and written into memory, then that image removed and,
/// when the input had one, the first kept image too; the output written and
/// read back. A failure after the output is in place removes it; a refusal
/// by the layer is exit 2 with no output.
fn remove_image_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = first_worksheet(&mut package)?;
    let sampled = sample(&mut package, &part)?;
    let before = image_counts(&mut package, &part)?;
    let (drawing_before, _) = container_and_binding(&mut package, &part, b"drawing")?;
    let kept_removed = u64::from(before.images > 0);
    // What the removal of the first kept image drops, by this tool's own
    // count, read before anything is written.
    let expected_dropped = if kept_removed == 1 {
        dropped_media(&mut package, &part)?
    } else {
        0
    };
    // A row past the grid is the layer's to refuse, with its index message.
    let row = sampled.max_row.saturating_add(1);
    package.add_image(
        &part,
        CellRef { row, col: 1 },
        Image {
            bytes: &SAMPLE_PNG,
            size: None,
        },
    )?;
    let mut buffer = io::Cursor::new(Vec::new());
    package.write_to(&mut buffer)?;
    buffer.set_position(0);
    let mut added = Package::open(buffer)?;
    let last = added
        .images(&part)?
        .len()
        .checked_sub(1)
        .ok_or(Error::RemoveImageSample("image count after the addition"))?;
    added.remove_image(&part, last)?;
    if kept_removed == 1 {
        added.remove_image(&part, 0)?;
    }
    write_into(added, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = removal_read_back(
        output,
        &part,
        &before,
        kept_removed,
        expected_dropped,
        drawing_before,
    );
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, media_dropped, drawing_dropped) = checked?;
    edit_line_named(
        "removeimage",
        &[
            ("images_before", before.images),
            ("images_after", after.images),
            ("kept_removed", kept_removed),
            ("media_before", before.media.len() as u64),
            ("media_after", after.media.len() as u64),
            ("media_dropped", media_dropped),
            ("drawing_dropped", drawing_dropped),
            ("anchors_before", before.anchors),
            ("anchors_after", after.anchors),
        ],
    )
}

/// Reopens the image removal sample's output and asserts the images the
/// input had less the one removed, the media entries the tool's own count
/// says go, and no drawing where the input had none; gives the counts, the
/// media entries dropped and whether the drawing is gone.
fn removal_read_back(
    output: &Path,
    part: &[u8],
    before: &ImageCounts,
    kept_removed: u64,
    expected_dropped: u64,
    drawing_before: bool,
) -> Result<(ImageCounts, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = image_counts(&mut package, part)?;
    let (drawing_after, _) = container_and_binding(&mut package, part, b"drawing")?;
    if after.images != before.images - kept_removed {
        return Err(Error::RemoveImageSample("image count"));
    }
    let media_dropped = (before.media.len() as u64)
        .checked_sub(after.media.len() as u64)
        .ok_or(Error::RemoveImageSample("media entry count"))?;
    if media_dropped != expected_dropped {
        return Err(Error::RemoveImageSample("media entries dropped"));
    }
    let drawing_dropped = u64::from(!drawing_before && !drawing_after);
    if !drawing_before && drawing_dropped != 1 {
        return Err(Error::RemoveImageSample("drawing"));
    }
    Ok((after, media_dropped, drawing_dropped))
}

/// A drawing root child as [`dropped_media`] reads it: whether it holds a
/// `pic`, its `embed` and `link` values, and every attribute value.
type RootRefs = (bool, Vec<Vec<u8>>, Vec<Vec<u8>>);

/// The media entries the removal of W's first image drops, by this tool's
/// own pass: the first root child of W's drawing part that holds a `pic`;
/// the values of its prefixed attributes named `embed` or `link` that no
/// attribute of another root child carries; the internal targets those name
/// through the drawing's rels part; and of those, the ones no other
/// relationship of any rels part of the archive targets.
fn dropped_media<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<u64, Error> {
    let drawing =
        related(package, part, DRAWING)?.ok_or(Error::RemoveImageSample("input's drawing"))?;
    let index = package.part(&drawing).ok_or(Error::NoSuchName)?;
    // Per root child: whether it holds a `pic`, its `embed` and `link`
    // values, and every attribute value it carries.
    let roots: Vec<RootRefs> = {
        let mut stream = Stream::new(package.read_part(index)?, index)?;
        let mut depth = 0usize;
        let mut roots: Vec<RootRefs> = Vec::new();
        while let Some(token) = stream.next()? {
            let raw = token.raw;
            if raw.starts_with(b"</") {
                depth = depth.saturating_sub(1);
                continue;
            }
            if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
                continue;
            }
            depth += 1;
            if depth == 2 {
                roots.push((false, Vec::new(), Vec::new()));
            }
            if depth >= 2
                && let Some((pic, named, values)) = roots.last_mut()
            {
                *pic |= tag_local_name(raw) == b"pic";
                for (name, bytes) in tag_attributes(raw) {
                    // The value, between the quotes that end the span.
                    let value = bytes
                        .iter()
                        .position(|byte| matches!(byte, b'"' | b'\''))
                        .and_then(|open| bytes.get(open + 1..bytes.len() - 1))
                        .unwrap_or_default()
                        .to_vec();
                    let local = name.rsplit(|byte| *byte == b':').next().unwrap_or(&name);
                    if name.contains(&b':') && (local == b"embed" || local == b"link") {
                        named.push(value.clone());
                    }
                    values.push(value);
                }
            }
            if raw.ends_with(b"/>") {
                depth -= 1;
            }
        }
        roots
    };
    let first = roots
        .iter()
        .position(|(pic, _, _)| *pic)
        .ok_or(Error::RemoveImageSample("input's picture"))?;
    let mut removed: Vec<Vec<u8>> = Vec::new();
    for id in &roots[first].1 {
        let elsewhere = roots
            .iter()
            .enumerate()
            .any(|(at, (_, _, values))| at != first && values.contains(id));
        if !elsewhere && !removed.contains(id) {
            removed.push(id.clone());
        }
    }
    let mut targets: Vec<Vec<u8>> = Vec::new();
    {
        let rels = package.relationships(&drawing)?;
        for id in &removed {
            if let Some(rel) = rels.by_id(id).filter(|rel| !rel.external)
                && let Ok(target) = rels.target_part(rel)
                && !targets.contains(&target)
            {
                targets.push(target);
            }
        }
    }
    let sources: Vec<Vec<u8>> = package
        .archive()
        .entries()
        .iter()
        .filter_map(|entry| rels_part_source(entry.name()))
        .collect();
    let mut dropped = 0u64;
    for target in targets {
        let mut others = 0u64;
        for source in &sources {
            let rels = package.relationships(source)?;
            for rel in rels.all() {
                let skipped = *source == drawing && removed.contains(&rel.id);
                if !rel.external && !skipped && rels.target_part(rel).is_ok_and(|at| at == target) {
                    others += 1;
                }
            }
        }
        dropped += u64::from(others == 0);
    }
    Ok(dropped)
}

/// What the comment sample counts on each side of the write, and whether a
/// kept author's text is the sample's.
struct CommentCounts {
    comments: u64,
    parts: u64,
    shapes: u64,
    authors: u64,
    author_kept: bool,
}

/// The comment sample's counts of W.
fn comment_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<CommentCounts, Error> {
    let comments = package.comments(part)?.len() as u64;
    let (authors, author_kept) = match related(package, part, COMMENTS)? {
        Some(target) => comment_authors(package, &target)?,
        None => (0, false),
    };
    let shapes = match related(package, part, VML_DRAWING)? {
        Some(target) => note_shapes(package, &target)?,
        None => 0,
    };
    Ok(CommentCounts {
        comments,
        parts: comment_entries(package),
        shapes,
        authors,
        author_kept,
    })
}

/// Reopens the comment sample's output and asserts one more comment and
/// note shape, `parts_more` more comments and VML entries, and one more
/// author unless the sample's was kept; then removes the comment on `cell`
/// in memory, writes into a `Cursor`, reopens it and asserts the comments
/// and the comments and VML entries the input had; gives the counts and,
/// after the removal, the comments and the entries.
fn comment_read_back(
    output: &Path,
    part: &[u8],
    cell: CellRef,
    before: &CommentCounts,
    parts_more: u64,
) -> Result<(CommentCounts, (u64, u64)), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = comment_counts_after(&mut package, part, before, parts_more)?;
    // The comment removed again in memory (specs/0022 "P2"): the write goes
    // into a Cursor that is reopened, for the comments and the parts.
    package.remove_comment(part, cell)?;
    let mut buffer = io::Cursor::new(Vec::new());
    package.write_to(&mut buffer)?;
    buffer.set_position(0);
    let mut reopened = Package::open(buffer)?;
    let removed = (
        reopened.comments(part)?.len() as u64,
        comment_entries(&reopened),
    );
    if removed.0 != before.comments {
        return Err(Error::CommentSample("comment count after the removal"));
    }
    if removed.1 != before.parts {
        return Err(Error::CommentSample(
            "comments and VML part count after the removal",
        ));
    }
    Ok((after, removed))
}

/// The comment sample's counts of W in its reopened output, asserted: one
/// more comment and note shape, `parts_more` more comments and VML entries,
/// and one more author unless the sample's was kept. The rich comment sample
/// asserts the same.
fn comment_counts_after<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
    before: &CommentCounts,
    parts_more: u64,
) -> Result<CommentCounts, Error> {
    let after = comment_counts(package, part)?;
    if after.comments != before.comments + 1 {
        return Err(Error::CommentSample("comment count"));
    }
    if after.parts != before.parts + parts_more {
        return Err(Error::CommentSample("comments and VML part count"));
    }
    if after.shapes != before.shapes + 1 {
        return Err(Error::CommentSample("note shape count"));
    }
    if after.authors != before.authors + u64::from(!before.author_kept) {
        return Err(Error::CommentSample("author count"));
    }
    Ok(after)
}

/// The archive's entries `xl/comments*.xml` and
/// `xl/drawings/vmlDrawing*.vml`.
fn comment_entries<R: io::Read + io::Seek>(package: &Package<R>) -> u64 {
    package
        .archive()
        .entries()
        .iter()
        .filter(|entry| {
            let name = entry.name();
            (name.starts_with(b"xl/comments") && name.ends_with(b".xml"))
                || (name.starts_with(b"xl/drawings/vmlDrawing") && name.ends_with(b".vml"))
        })
        .count() as u64
}

/// The part the first internal relationship of `part` of type `kind`, either
/// form, names.
fn related<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
    kind: [&[u8]; 2],
) -> Result<Option<Vec<u8>>, Error> {
    let rels = package.relationships(part)?;
    for form in kind {
        if let Some(rel) = rels.of_kind(form).find(|rel| !rel.external) {
            return Ok(Some(rels.target_part(rel)?));
        }
    }
    Ok(None)
}

/// The `author` elements of a comments part, and whether one's text is the
/// sample's author, by this tool's own pass over the part's tokens.
fn comment_authors<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<(u64, bool), Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut count = 0u64;
    let mut kept = false;
    let mut open = false;
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        let tag = raw.starts_with(b"<") && !raw.starts_with(b"<?") && !raw.starts_with(b"<!");
        if open && !tag && raw == SAMPLE_AUTHOR.as_bytes() {
            kept = true;
        }
        open = false;
        if tag && !raw.starts_with(b"</") && tag_local_name(raw) == b"author" {
            count += 1;
            open = !raw.ends_with(b"/>");
        }
    }
    Ok((count, kept))
}

/// The shapes of a VML part that hold a `ClientData` of `ObjectType`
/// `Note`, by this tool's own pass over the part's tokens.
fn note_shapes<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<u64, Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut shape: Option<usize> = None;
    let mut note = false;
    let mut count = 0u64;
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            if shape == Some(depth) {
                count += u64::from(note);
                shape = None;
            }
            depth = depth.saturating_sub(1);
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        let local = tag_local_name(raw);
        if local == b"shape" && shape.is_none() {
            shape = Some(depth);
            note = false;
        } else if local == b"ClientData" && shape.is_some() {
            note = tag_attribute(raw, b"ObjectType").as_deref() == Some(b"Note".as_slice());
        }
        if raw.ends_with(b"/>") {
            if shape == Some(depth) {
                shape = None;
            }
            depth -= 1;
        }
    }
    Ok(count)
}

/// The archive's entries under `xl/tables/`.
fn table_entries<R: io::Read + io::Seek>(package: &Package<R>) -> u64 {
    package
        .archive()
        .entries()
        .iter()
        .filter(|entry| entry.name().starts_with(b"xl/tables/"))
        .count() as u64
}

/// The part's relationships of the table type, in either form.
fn table_relationships<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<u64, Error> {
    let rels = package.relationships(part)?;
    Ok(TABLE
        .into_iter()
        .map(|form| rels.of_kind(form).count() as u64)
        .sum())
}

/// What the sheet view sample reads of W by this tool's own pass over its
/// tokens: the `<sheetPr>` root children, the `<tabColor>` children of the
/// first and the `rgb` of the last of those, the `<sheetView>` children of
/// the `<sheetViews>` root children, and the first `<sheetView>`'s start
/// tag.
struct ViewFacts {
    sheet_pr: u64,
    tab_color: u64,
    tab_rgb: Option<Vec<u8>>,
    views: u64,
    first_view: Option<Vec<u8>>,
}

/// The sheet view sample's reading of `part`.
fn view_facts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<ViewFacts, Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut root: Vec<u8> = Vec::new();
    // The root child open at depth 2, when it is of the main namespace.
    let mut open: Vec<u8> = Vec::new();
    let mut facts = ViewFacts {
        sheet_pr: 0,
        tab_color: 0,
        tab_rgb: None,
        views: 0,
        first_view: None,
    };
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            depth = depth.saturating_sub(1);
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        let main = MAIN.contains(&element_namespace(raw, &root).as_slice());
        if depth == 1 {
            root = raw.to_vec();
        } else if depth == 2 {
            open = if main {
                tag_local_name(raw).to_vec()
            } else {
                Vec::new()
            };
            if open == b"sheetPr" {
                facts.sheet_pr += 1;
            }
        } else if depth == 3 && main {
            let local = tag_local_name(raw);
            if open == b"sheetPr" && local == b"tabColor" && facts.sheet_pr == 1 {
                facts.tab_color += 1;
                facts.tab_rgb = tag_attribute(raw, b"rgb");
            } else if open == b"sheetViews" && local == b"sheetView" {
                facts.views += 1;
                if facts.first_view.is_none() {
                    facts.first_view = Some(raw.to_vec());
                }
            }
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
        }
    }
    Ok(facts)
}

/// A start tag's attributes in order, each as its name and its bytes from
/// the name to the closing quote; namespace declarations among them.
fn tag_attributes(raw: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
    let space = |byte: u8| matches!(byte, b' ' | b'\t' | b'\n' | b'\r');
    let mut out = Vec::new();
    let mut at = raw
        .iter()
        .position(|byte| space(*byte))
        .unwrap_or(raw.len());
    loop {
        while at < raw.len() && space(raw[at]) {
            at += 1;
        }
        if at >= raw.len() || matches!(raw[at], b'/' | b'>') {
            return out;
        }
        let start = at;
        while at < raw.len() && raw[at] != b'=' && !space(raw[at]) {
            at += 1;
        }
        let name = raw[start..at].to_vec();
        while at < raw.len() && raw[at] != b'"' && raw[at] != b'\'' {
            at += 1;
        }
        let Some(&quote) = raw.get(at) else {
            return out;
        };
        let Some(length) = raw[at + 1..].iter().position(|byte| *byte == quote) else {
            return out;
        };
        at += length + 2;
        out.push((name, raw[start..at].to_vec()));
    }
}

/// The attributes the sheet view sample sets.
const VIEW_SET: [&[u8]; 3] = [b"zoomScale", b"showGridLines", b"showRowColHeaders"];

/// The first `<sheetView>`'s attributes other than [`VIEW_SET`]'s.
fn kept_view_attributes(view: Option<&[u8]>) -> Vec<(Vec<u8>, Vec<u8>)> {
    view.map(tag_attributes)
        .unwrap_or_default()
        .into_iter()
        .filter(|(name, _)| !VIEW_SET.contains(&name.as_slice()))
        .collect()
}

/// The sheet view sample (`specs/0025-sheet-and-print-settings.md`,
/// "Harness runs", `--sheetview-sample`): a tab colour and a sheet view on
/// W, the output written and read back. A failure after the output is in
/// place removes it; a refusal by the layer is exit 2 with no output.
fn sheetview_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let before = view_facts(&mut package, &part)?;
    let (_, r_bound_before) = container_and_binding(&mut package, &part, b"sheetData")?;
    package.set_tab_color(&part, Some(Color::Rgb(0xFF00_AA55)))?;
    package.set_sheet_view(
        &part,
        &SheetView {
            zoom: Some(85),
            show_grid_lines: Some(false),
            show_headings: Some(true),
            ..SheetView::default()
        },
    )?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = sheetview_read_back(output, &part, &before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, kept) = checked?;
    edit_line_named(
        "sheetview",
        &[
            ("sheetpr_before", before.sheet_pr),
            ("sheetpr_after", after.sheet_pr),
            ("tabcolor_before", before.tab_color),
            ("tabcolor_after", after.tab_color),
            ("views_before", before.views),
            ("views_after", after.views),
            ("zoom_ok", 1),
            ("grid_ok", 1),
            ("headings_ok", 1),
            ("kept_attrs", kept),
            ("r_bound_before", u64::from(r_bound_before)),
        ],
    )
}

/// Reopens the sheet view sample's output and asserts what
/// `--sheetview-sample` says it asserts; gives W's reading and the count of
/// kept attributes.
fn sheetview_read_back(
    output: &Path,
    part: &[u8],
    before: &ViewFacts,
) -> Result<(ViewFacts, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = view_facts(&mut package, part)?;
    if after.sheet_pr != 1 {
        return Err(Error::SheetviewSample("sheetPr count"));
    }
    if after.tab_color != 1 || after.tab_rgb.as_deref() != Some(b"FF00AA55".as_slice()) {
        return Err(Error::SheetviewSample("tabColor"));
    }
    let views_ok = if before.views == 0 {
        after.views == 1
    } else {
        after.views == before.views
    };
    if !views_ok {
        return Err(Error::SheetviewSample("sheetView count"));
    }
    let view = after.first_view.as_deref().unwrap_or_default();
    for (name, value, what) in [
        (b"zoomScale".as_slice(), b"85".as_slice(), "zoomScale"),
        (b"showGridLines", b"0", "showGridLines"),
        (b"showRowColHeaders", b"1", "showRowColHeaders"),
    ] {
        if tag_attribute(view, name).as_deref() != Some(value) {
            return Err(Error::SheetviewSample(what));
        }
    }
    let kept_before = kept_view_attributes(before.first_view.as_deref());
    let kept_after = kept_view_attributes(after.first_view.as_deref());
    let kept_ok = if before.views == 0 {
        kept_after == [(b"workbookViewId".to_vec(), b"workbookViewId=\"0\"".to_vec())]
    } else {
        kept_after == kept_before
    };
    if !kept_ok {
        return Err(Error::SheetviewSample("kept sheetView attributes"));
    }
    Ok((after, kept_after.len() as u64))
}

/// The three built-in names the print area sample writes, in its line's
/// order.
const PRINT_NAMES: [&str; 3] = [
    "_xlnm.Print_Area",
    "_xlnm.Print_Titles",
    "_xlnm._FilterDatabase",
];

/// Whether `text` is `reference` behind the sheet named `sheet`, the name
/// bare or quoted with an apostrophe doubled; the crate decides which
/// (`specs/0025-sheet-and-print-settings.md`, "Planning rulings during P3",
/// ruling 6) and the run log's own check pins the choice.
fn behind_sheet(text: &str, sheet: &str, reference: &str) -> bool {
    let quoted = format!("'{}'", sheet.replace('\'', "''"));
    text == format!("{sheet}!{reference}") || text == format!("{quoted}!{reference}")
}

/// The print area sample (`specs/0025-sheet-and-print-settings.md`,
/// "Harness runs", `--printarea-sample`): a print area, print titles and an
/// autofilter on W, the output written and read back. A failure after the
/// output is in place removes it; a refusal by the layer is exit 2 with no
/// output.
fn printarea_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let (at, sheet) = workbook
        .sheets
        .iter()
        .enumerate()
        .find(|(_, sheet)| sheet.kind == SheetKind::Worksheet)
        .map(|(at, sheet)| (at as u32, sheet.clone()))
        .ok_or(Error::NoWorksheet)?;
    let part = sheet.part.clone();
    let names_before = workbook.defined_names().len() as u64;
    let had = PRINT_NAMES
        .iter()
        .filter(|name| {
            workbook
                .defined_names()
                .iter()
                .any(|have| have.sheet == Some(at) && have.name.eq_ignore_ascii_case(name))
        })
        .count() as u64;
    let sampled = sample(&mut package, &part)?;
    let autofilter_before = root_children(&mut package, &part, b"autoFilter")?;
    let (_, r_bound_before) = container_and_binding(&mut package, &part, b"sheetData")?;
    // A row past the grid is the layer's to refuse, with its index message.
    let last = CellRef {
        row: sampled.max_row.saturating_add(1),
        col: 8,
    };
    let area = Area {
        first: CellRef { row: 1, col: 1 },
        last,
    };
    package.set_print_area(&part, &[area])?;
    package.set_print_titles(&part, Some((1, 1)), None)?;
    package.set_auto_filter(&part, Some(area))?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = printarea_read_back(output, &part, at, &sheet.name, last.row, names_before, had);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (names_after, autofilter_after) = checked?;
    edit_line_named(
        "printarea",
        &[
            ("names_before", names_before),
            ("names_after", names_after),
            ("area_ok", 1),
            ("titles_ok", 1),
            ("filterdb_ok", 1),
            ("autofilter_before", autofilter_before),
            ("autofilter_after", autofilter_after),
            ("hidden_ok", 1),
            ("r_bound_before", u64::from(r_bound_before)),
        ],
    )
}

/// Reopens the print area sample's output and asserts what
/// `--printarea-sample` says it asserts; gives the name count and W's
/// `<autoFilter>` count.
fn printarea_read_back(
    output: &Path,
    part: &[u8],
    at: u32,
    sheet: &str,
    last_row: u32,
    names_before: u64,
    had: u64,
) -> Result<(u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let names = workbook.defined_names();
    let names_after = names.len() as u64;
    if names_after + had != names_before + 3 {
        return Err(Error::PrintareaSample("defined name count"));
    }
    let area = format!("$A$1:$H${last_row}");
    for (name, reference, what) in [
        (PRINT_NAMES[0], area.as_str(), "print area"),
        (PRINT_NAMES[1], "$1:$1", "print titles"),
        (PRINT_NAMES[2], area.as_str(), "filter database"),
    ] {
        let found: Vec<_> = names
            .iter()
            .filter(|have| have.sheet == Some(at) && have.name == name)
            .collect();
        if found.len() != 1 || !behind_sheet(&found[0].text, sheet, reference) {
            return Err(Error::PrintareaSample(what));
        }
        if name == PRINT_NAMES[2] && !found[0].hidden {
            return Err(Error::PrintareaSample("filter database's hidden flag"));
        }
    }
    let autofilter_after = root_children(&mut package, part, b"autoFilter")?;
    if autofilter_after != 1 {
        return Err(Error::PrintareaSample("autoFilter count"));
    }
    Ok((names_after, autofilter_after))
}

/// The four root children the page setup sample sets, by local name.
const PAGE_ELEMENTS: [&[u8]; 4] = [
    b"printOptions",
    b"pageMargins",
    b"pageSetup",
    b"headerFooter",
];

/// What the page setup sample reads of W by this tool's own pass over its
/// tokens: the count of each of [`PAGE_ELEMENTS`] among the root's
/// main-namespace children, the start tags of the first of each (the
/// `<headerFooter>`'s children as their local names and bytes), and the
/// start tag of the first `<sheetPr>`'s `<pageSetUpPr>`.
#[derive(Default)]
struct PageFacts {
    counts: [u64; 4],
    tags: [Option<Vec<u8>>; 4],
    hf_children: Vec<(Vec<u8>, Vec<u8>)>,
    set_up_pr: Option<Vec<u8>>,
}

/// The page setup sample's reading of `part`.
fn page_facts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<PageFacts, Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut root: Vec<u8> = Vec::new();
    // The main-namespace root child open at depth 2, and whether it is the
    // first of its name.
    let mut open: Vec<u8> = Vec::new();
    let mut first = false;
    let mut sheet_prs = 0u64;
    let mut facts = PageFacts::default();
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            if depth == 3
                && open == b"headerFooter"
                && first
                && let Some((_, bytes)) = facts.hf_children.last_mut()
            {
                bytes.extend_from_slice(raw);
            }
            depth = depth.saturating_sub(1);
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            if depth == 3
                && open == b"headerFooter"
                && first
                && let Some((_, bytes)) = facts.hf_children.last_mut()
            {
                bytes.extend_from_slice(raw);
            }
            continue;
        }
        depth += 1;
        let main = MAIN.contains(&element_namespace(raw, &root).as_slice());
        if depth == 1 {
            root = raw.to_vec();
        } else if depth == 2 {
            open = if main {
                tag_local_name(raw).to_vec()
            } else {
                Vec::new()
            };
            first = false;
            if open == b"sheetPr" {
                sheet_prs += 1;
            }
            if let Some(at) = PAGE_ELEMENTS
                .iter()
                .position(|name| *name == open.as_slice())
            {
                facts.counts[at] += 1;
                if facts.tags[at].is_none() {
                    facts.tags[at] = Some(raw.to_vec());
                    first = true;
                }
            }
        } else if depth == 3 && main && open == b"headerFooter" && first {
            facts
                .hf_children
                .push((tag_local_name(raw).to_vec(), raw.to_vec()));
        } else if depth == 3
            && main
            && open == b"sheetPr"
            && sheet_prs == 1
            && tag_local_name(raw) == b"pageSetUpPr"
            && facts.set_up_pr.is_none()
        {
            facts.set_up_pr = Some(raw.to_vec());
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
        }
    }
    Ok(facts)
}

/// The relationship id attribute of a `<pageSetup>` start tag, as its
/// bytes, when it has one: the attribute whose name ends `:id`.
fn relationship_id(tag: Option<&[u8]>) -> Option<Vec<u8>> {
    tag_attributes(tag?)
        .into_iter()
        .find(|(name, _)| name.ends_with(b":id"))
        .map(|(_, bytes)| bytes)
}

/// The odd header and footer the page setup sample writes, as their
/// elements' escaped text.
const SAMPLE_HEADER: &[u8] = b"&amp;Lemend&amp;R&amp;P of &amp;N";
const SAMPLE_FOOTER: &[u8] = b"&amp;Cemend";

/// The `<headerFooter>` children other than the odd header and footer.
fn other_hf_children(children: &[(Vec<u8>, Vec<u8>)]) -> Vec<(Vec<u8>, Vec<u8>)> {
    children
        .iter()
        .filter(|(local, _)| local != b"oddHeader" && local != b"oddFooter")
        .cloned()
        .collect()
}

/// Whether a `<headerFooter>` child named `local` holds exactly `text`.
fn hf_text(children: &[(Vec<u8>, Vec<u8>)], local: &[u8], text: &[u8]) -> bool {
    children.iter().any(|(name, bytes)| {
        name == local
            && bytes.ends_with(b">")
            && bytes
                .iter()
                .position(|byte| *byte == b'>')
                .and_then(|open| {
                    let close = bytes.iter().rposition(|byte| *byte == b'<')?;
                    Some(&bytes[open + 1..close] == text)
                })
                .unwrap_or(false)
    })
}

/// The page setup sample (`specs/0025-sheet-and-print-settings.md`,
/// "Harness runs", `--pagesetup-sample`): print options, a margin, a page
/// setup and a header and footer on W, the output written and read back.
/// A failure after the output is in place removes it; a refusal by the
/// layer is exit 2 with no output.
fn pagesetup_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let before = page_facts(&mut package, &part)?;
    let (_, r_bound_before) = container_and_binding(&mut package, &part, b"sheetData")?;
    package.set_print_options(
        &part,
        &PrintOptions {
            horizontal_centered: Some(true),
            ..PrintOptions::default()
        },
    )?;
    package.set_page_margins(
        &part,
        &PageMargins {
            left: Some(0.5),
            ..PageMargins::default()
        },
    )?;
    package.set_page_setup(
        &part,
        &PageSetup {
            orientation: Some(Orientation::Landscape),
            fit_to_width: Some(1),
            fit_to_height: Some(0),
            fit_to_page: Some(true),
            ..PageSetup::default()
        },
    )?;
    package.set_header_footer(
        &part,
        &HeaderFooter {
            odd_header: Some(HeaderFooter::section("emend", "", "&P of &N")),
            odd_footer: Some(HeaderFooter::section("", "emend", "")),
            ..HeaderFooter::default()
        },
    )?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = pagesetup_read_back(output, &part, &before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, kept) = checked?;
    let mut pairs: Vec<(String, u64)> = Vec::new();
    for (at, name) in PAGE_ELEMENTS.iter().enumerate() {
        let name = String::from_utf8_lossy(name);
        pairs.push((format!("{name}_before"), before.counts[at]));
        pairs.push((format!("{name}_after"), after.counts[at]));
    }
    for (key, value) in [
        ("rid_kept", 1),
        ("fit_ok", 1),
        ("margin_ok", 1),
        ("orient_ok", 1),
        ("header_ok", 1),
        ("footer_ok", 1),
        ("hf_kept", kept),
        ("r_bound_before", u64::from(r_bound_before)),
    ] {
        pairs.push((key.to_string(), value));
    }
    let borrowed: Vec<(&str, u64)> = pairs
        .iter()
        .map(|(key, value)| (key.as_str(), *value))
        .collect();
    edit_line_named("pagesetup", &borrowed)
}

/// Reopens the page setup sample's output and asserts what
/// `--pagesetup-sample` says it asserts; gives W's reading and the count of
/// the other `<headerFooter>` children.
fn pagesetup_read_back(
    output: &Path,
    part: &[u8],
    before: &PageFacts,
) -> Result<(PageFacts, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = page_facts(&mut package, part)?;
    if after.counts != [1; 4] {
        return Err(Error::PagesetupSample(
            "printOptions, pageMargins, pageSetup or headerFooter count",
        ));
    }
    let tag = |at: usize| after.tags[at].as_deref().unwrap_or_default();
    if relationship_id(before.tags[2].as_deref()) != relationship_id(after.tags[2].as_deref()) {
        return Err(Error::PagesetupSample("pageSetup relationship id"));
    }
    let fit = after
        .set_up_pr
        .as_deref()
        .is_some_and(|pr| tag_attribute(pr, b"fitToPage").as_deref() == Some(b"1".as_slice()))
        && tag_attribute(tag(2), b"fitToWidth").as_deref() == Some(b"1".as_slice())
        && tag_attribute(tag(2), b"fitToHeight").as_deref() == Some(b"0".as_slice());
    if !fit {
        return Err(Error::PagesetupSample("fit to page"));
    }
    if tag_attribute(tag(1), b"left").as_deref() != Some(b"0.5".as_slice()) {
        return Err(Error::PagesetupSample("left margin"));
    }
    if tag_attribute(tag(2), b"orientation").as_deref() != Some(b"landscape".as_slice()) {
        return Err(Error::PagesetupSample("orientation"));
    }
    if tag_attribute(tag(0), b"horizontalCentered").as_deref() != Some(b"1".as_slice()) {
        return Err(Error::PagesetupSample("horizontalCentered"));
    }
    if !hf_text(&after.hf_children, b"oddHeader", SAMPLE_HEADER) {
        return Err(Error::PagesetupSample("odd header"));
    }
    if !hf_text(&after.hf_children, b"oddFooter", SAMPLE_FOOTER) {
        return Err(Error::PagesetupSample("odd footer"));
    }
    let kept = other_hf_children(&after.hf_children);
    if kept != other_hf_children(&before.hf_children) {
        return Err(Error::PagesetupSample("other headerFooter children"));
    }
    Ok((after, kept.len() as u64))
}

/// What the group sample reads of W by this tool's own pass over its
/// tokens: each `<row>`'s level, hidden and collapsed flags, the `<row>`
/// elements numbered inside a window, each `<col>`'s start tag, the first
/// `<sheetFormatPr>`'s start tag and the first `<sheetPr>`'s `<outlinePr>`
/// start tag.
struct GroupFacts {
    rows: u64,
    in_window: u64,
    /// The start tags of the rows inside the window, in order.
    window_rows: Vec<Vec<u8>>,
    level_rows: u64,
    hidden_rows: u64,
    collapsed_rows: u64,
    cols: Vec<Vec<u8>>,
    format: Option<Vec<u8>>,
    outline: Option<Vec<u8>>,
}

/// The group sample's reading of `part`, `window` the row numbers counted
/// in [`GroupFacts::in_window`].
fn group_facts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
    window: (u32, u32),
) -> Result<GroupFacts, Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut root: Vec<u8> = Vec::new();
    // The root child open at depth 2, when it is of the main namespace.
    let mut open: Vec<u8> = Vec::new();
    let mut sheet_prs = 0u64;
    let mut facts = GroupFacts {
        rows: 0,
        in_window: 0,
        window_rows: Vec::new(),
        level_rows: 0,
        hidden_rows: 0,
        collapsed_rows: 0,
        cols: Vec::new(),
        format: None,
        outline: None,
    };
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            depth = depth.saturating_sub(1);
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        let main = MAIN.contains(&element_namespace(raw, &root).as_slice());
        if depth == 1 {
            root = raw.to_vec();
        } else if depth == 2 {
            open = if main {
                tag_local_name(raw).to_vec()
            } else {
                Vec::new()
            };
            if open == b"sheetPr" {
                sheet_prs += 1;
            }
            if open == b"sheetFormatPr" && facts.format.is_none() {
                facts.format = Some(raw.to_vec());
            }
        } else if depth == 3 && main {
            let local = tag_local_name(raw);
            if open == b"sheetData" && local == b"row" {
                facts.rows += 1;
                let number = tag_attribute(raw, b"r")
                    .and_then(|r| std::str::from_utf8(&r).ok()?.parse::<u32>().ok());
                if number.is_some_and(|r| (window.0..=window.1).contains(&r)) {
                    facts.in_window += 1;
                    facts.window_rows.push(raw.to_vec());
                }
                let flag =
                    |name: &[u8]| tag_attribute(raw, name).as_deref() == Some(b"1".as_slice());
                if flag(b"outlineLevel") {
                    facts.level_rows += 1;
                    if flag(b"hidden") {
                        facts.hidden_rows += 1;
                    }
                }
                if flag(b"collapsed") {
                    facts.collapsed_rows += 1;
                }
            } else if open == b"cols" && local == b"col" {
                facts.cols.push(raw.to_vec());
            } else if open == b"sheetPr"
                && local == b"outlinePr"
                && sheet_prs == 1
                && facts.outline.is_none()
            {
                facts.outline = Some(raw.to_vec());
            }
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
        }
    }
    Ok(facts)
}

/// A `<col>`'s `min` and `max`, when both are whole numbers.
fn col_span(tag: &[u8]) -> Option<(u32, u32)> {
    let number = |name: &[u8]| -> Option<u32> {
        std::str::from_utf8(&tag_attribute(tag, name)?)
            .ok()?
            .parse()
            .ok()
    };
    Some((number(b"min")?, number(b"max")?))
}

/// The start tag of the `<col>` in `cols` that covers `column`.
fn covering(cols: &[Vec<u8>], column: u32) -> Option<&[u8]> {
    cols.iter()
        .find(|tag| col_span(tag).is_some_and(|(min, max)| (min..=max).contains(&column)))
        .map(Vec::as_slice)
}

/// The attributes the group sample's split and edit set or move.
const GROUP_SET: [&[u8]; 5] = [b"min", b"max", b"outlineLevel", b"hidden", b"collapsed"];

/// A `<col>`'s attributes other than [`GROUP_SET`]'s.
fn kept_col_attributes(tag: Option<&[u8]>) -> Vec<(Vec<u8>, Vec<u8>)> {
    tag.map(tag_attributes)
        .unwrap_or_default()
        .into_iter()
        .filter(|(name, _)| !GROUP_SET.contains(&name.as_slice()))
        .collect()
}

/// The attributes, other than `min` and `max`, that the write leaves on the
/// `<col>` covering a column the input's `tag` covered (none for `None`):
/// `hidden` dropped and `outlineLevel="1"` set in place or appended, as the
/// crate's row and column format does.
fn grouped_col_attributes(tag: Option<&[u8]>) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut attributes: Vec<(Vec<u8>, Vec<u8>)> = tag
        .map(tag_attributes)
        .unwrap_or_default()
        .into_iter()
        .filter(|(name, _)| !matches!(name.as_slice(), b"min" | b"max" | b"hidden"))
        .collect();
    let level = (b"outlineLevel".to_vec(), b"outlineLevel=\"1\"".to_vec());
    match attributes
        .iter_mut()
        .find(|(name, _)| name.as_slice() == b"outlineLevel")
    {
        Some(present) => *present = level,
        None => attributes.push(level),
    }
    attributes
}

/// Whether a start tag carries the attribute `name` as a whole number at
/// least 1.
fn at_least_one(tag: Option<&[u8]>, name: &[u8]) -> bool {
    tag.and_then(|tag| tag_attribute(tag, name))
        .and_then(|value| std::str::from_utf8(&value).ok()?.parse::<u8>().ok())
        .is_some_and(|value| value >= 1)
}

/// The group sample (`specs/0026-grouping-and-line-styles.md`, "Harness
/// runs", `--group-sample`): two rows grouped hidden, the row after them
/// collapsed, two columns grouped and the summary rows above, on W, the
/// output written and read back. A failure after the output is in place
/// removes it; a refusal by the layer is exit 2 with no output.
fn group_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    // Past the grid, the layer's own refusal names the index.
    let first = sampled.max_row.saturating_add(1);
    let window = (first, first.saturating_add(2));
    let before = group_facts(&mut package, &part, window)?;
    let (_, r_bound_before) = container_and_binding(&mut package, &part, b"sheetData")?;
    package.group_rows(&part, first, first.saturating_add(1), 1, true)?;
    package.set_row_collapsed(&part, first.saturating_add(2), true)?;
    package.group_columns(&part, 2, 3, 1, false)?;
    package.set_outline_summary(&part, Some(false), None);
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = group_read_back(output, &part, &before, window);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, level_cols, kept) = checked?;
    edit_line_named(
        "group",
        &[
            ("rows_before", before.rows),
            ("rows_present", before.in_window),
            ("rows_after", after.rows),
            ("level_rows", after.level_rows),
            ("hidden_rows", after.hidden_rows),
            ("collapsed_rows", after.collapsed_rows),
            ("cols_before", before.cols.len() as u64),
            ("cols_after", after.cols.len() as u64),
            ("level_cols", level_cols),
            ("max_row_ok", 1),
            ("max_col_ok", 1),
            ("summary_ok", 1),
            ("kept_cols", kept),
            ("r_bound_before", u64::from(r_bound_before)),
        ],
    )
}

/// Reopens the group sample's output and asserts what `--group-sample`
/// says it asserts; gives W's reading, the `<col>` elements carrying the
/// level and the count of kept attributes.
fn group_read_back(
    output: &Path,
    part: &[u8],
    before: &GroupFacts,
    window: (u32, u32),
) -> Result<(GroupFacts, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = group_facts(&mut package, part, window)?;
    // The rows of the window the input lacks are created, the rest edited
    // in place ("Planning rulings during P1", ruling 6).
    if after.rows != before.rows + 3 - before.in_window || after.in_window != 3 {
        return Err(Error::GroupSample("row count"));
    }
    if after.level_rows != 2 || after.hidden_rows != 2 {
        return Err(Error::GroupSample("grouped rows"));
    }
    if after.collapsed_rows != 1 {
        return Err(Error::GroupSample("collapsed rows"));
    }
    let leveled: Vec<(u32, u32)> = after
        .cols
        .iter()
        .filter(|tag| tag_attribute(tag, b"outlineLevel").as_deref() == Some(b"1".as_slice()))
        .filter_map(|tag| col_span(tag))
        .collect();
    let input_two = covering(&before.cols, 2);
    let input_three = covering(&before.cols, 3);
    let expected: &[(u32, u32)] =
        if grouped_col_attributes(input_two) == grouped_col_attributes(input_three) {
            &[(2, 3)]
        } else {
            &[(2, 2), (3, 3)]
        };
    if leveled != expected {
        return Err(Error::GroupSample("grouped columns"));
    }
    let mut kept = 0u64;
    for (column, input) in [(2, input_two), (3, input_three)] {
        let kept_before = kept_col_attributes(input);
        let kept_after = kept_col_attributes(covering(&after.cols, column));
        if kept_after != kept_before {
            return Err(Error::GroupSample("kept col attributes"));
        }
        kept += kept_after.len() as u64;
    }
    if !at_least_one(after.format.as_deref(), b"outlineLevelRow")
        || !at_least_one(after.format.as_deref(), b"outlineLevelCol")
    {
        return Err(Error::GroupSample("outline maximums"));
    }
    if after
        .outline
        .as_deref()
        .and_then(|tag| tag_attribute(tag, b"summaryBelow"))
        .as_deref()
        != Some(b"0".as_slice())
    {
        return Err(Error::GroupSample("outline summary"));
    }
    Ok((after, expected.len() as u64, kept))
}

/// The line style sample (`specs/0026-grouping-and-line-styles.md`,
/// "Harness runs", `--linestyle-sample`): a bold font added on cell format
/// 0, the new format set on row M + 1 and on column 9 of W, the output
/// written and read back. A failure after the output is in place removes it;
/// a refusal by the layer is exit 2 with no output.
fn linestyle_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let xfs_before = package.styles()?.cell_xfs.len() as u64;
    let sampled = sample(&mut package, &part)?;
    // Past the grid, the layer's own refusal names the index.
    let row = sampled.max_row.saturating_add(1);
    let before = group_facts(&mut package, &part, (row, row))?;
    let (_, r_bound_before) = container_and_binding(&mut package, &part, b"sheetData")?;
    let bold = Style {
        font: Some(Font {
            bold: true,
            ..Font::default()
        }),
        ..Style::default()
    };
    let style = package.add_style(Some(0), &bold)?;
    package.set_row_style(&part, row, Some(style))?;
    package.set_column_style(&part, LINESTYLE_COLUMN, Some(style))?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = linestyle_read_back(output, &part, &before, row, style, xfs_before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, split, kept, xfs_after) = checked?;
    edit_line_named(
        "linestyle",
        &[
            ("rows_before", before.rows),
            ("rows_present", before.in_window),
            ("rows_after", after.rows),
            ("row_style_ok", 1),
            ("cols_before", before.cols.len() as u64),
            ("cols_after", after.cols.len() as u64),
            ("col_style_ok", 1),
            ("col_split", split),
            ("kept_cols", kept),
            ("xfs_before", xfs_before),
            ("xfs_after", xfs_after),
            ("r_bound_before", u64::from(r_bound_before)),
        ],
    )
}

/// The column the line style sample styles.
const LINESTYLE_COLUMN: u32 = 9;

/// The round-trip sample (`specs/0027-read-back.md`, "Harness runs",
/// `--roundtrip-sample`): each cell format W's cells name read with
/// `cell_style`, added again with no base and every field of the value
/// read, the output written and each addition read back. A failure after
/// the output is in place removes it; a refusal by the layer is exit 2 with
/// no output.
fn roundtrip_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let mut used = std::collections::BTreeSet::new();
    for cell in package.cells(&part)? {
        if let Some(style) = cell?.style {
            used.insert(style);
        }
    }
    let before = collection_counts(&mut package)?;
    let mut read = Vec::new();
    for &index in &used {
        read.push(package.cell_style(index)?);
    }
    let mut added = Vec::new();
    for style in &read {
        added.push(package.add_style(
            None,
            &Style {
                number_format: Some(style.number_format.clone()),
                font: Some(style.font.clone()),
                fill: style.fill,
                border: Some(style.border),
                alignment: style.alignment,
                protection: style.protection,
            },
        )?);
    }
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = roundtrip_read_back(output, &read, &added, before);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, equal) = checked?;
    edit_line_named(
        "roundtrip",
        &[
            ("styles_in", used.len() as u64),
            ("styles_added", added.len() as u64),
            ("xfs_before", before[0]),
            ("xfs_after", after[0]),
            ("fonts_before", before[1]),
            ("fonts_after", after[1]),
            ("fills_before", before[2]),
            ("fills_after", after[2]),
            ("borders_before", before[3]),
            ("borders_after", after[3]),
            ("equal", equal),
        ],
    )
}

/// The styles part's cell formats, fonts, fills and borders, in that order,
/// a missing collection as 0.
fn collection_counts<R: io::Read + io::Seek>(package: &mut Package<R>) -> Result<[u64; 4], Error> {
    let styles = package.styles()?;
    Ok([
        styles.cell_xfs.len() as u64,
        u64::from(styles.fonts().unwrap_or(0)),
        u64::from(styles.fills().unwrap_or(0)),
        u64::from(styles.borders().unwrap_or(0)),
    ])
}

/// The number of distinct values among `values`, equal ones counted once,
/// as `add_style` records one element for equal additions.
fn distinct<T: PartialEq>(values: impl Iterator<Item = T>) -> u64 {
    let mut seen: Vec<T> = Vec::new();
    for value in values {
        if !seen.contains(&value) {
            seen.push(value);
        }
    }
    seen.len() as u64
}

/// Reopens the round-trip sample's output and asserts what
/// `--roundtrip-sample` says it asserts; gives the counts after and the
/// number of additions that read back equal.
fn roundtrip_read_back(
    output: &Path,
    read: &[emend::styles::CellStyle],
    added: &[u32],
    before: [u64; 4],
) -> Result<([u64; 4], u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = collection_counts(&mut package)?;
    let count = read.len() as u64;
    if added.len() != read.len() || after[0] != before[0] + count {
        return Err(Error::RoundtripSample("cell format count"));
    }
    let grown = [
        distinct(read.iter().map(|style| &style.font)),
        distinct(read.iter().filter_map(|style| style.fill.as_ref())),
        distinct(read.iter().map(|style| &style.border)),
    ];
    for (at, what) in [(1, "font count"), (2, "fill count"), (3, "border count")] {
        if after[at] != before[at] + grown[at - 1] {
            return Err(Error::RoundtripSample(what));
        }
    }
    // A `None` fill (a gradient, or no pattern) is written as fill 0, so it
    // reads back as fill 0's value ("Planning verification and rulings on
    // P1", ruling 1): read from the output's first cell format naming fill 0.
    let fill_zero = match read.iter().any(|style| style.fill.is_none()) {
        false => None,
        true => {
            let first = package
                .styles()?
                .cell_xfs
                .iter()
                .position(|xf| xf.fill_id.unwrap_or(0) == 0)
                .ok_or(Error::RoundtripSample("fill 0"))?;
            let first = u32::try_from(first).map_err(|_| Error::RoundtripSample("fill 0"))?;
            package.cell_style(first)?.fill
        }
    };
    let mut equal = 0u64;
    for (style, &index) in read.iter().zip(added) {
        let mut expected = style.clone();
        if expected.fill.is_none() {
            expected.fill = fill_zero;
        }
        if package.cell_style(index)? == expected {
            equal += 1;
        }
    }
    if equal != count {
        return Err(Error::RoundtripSample("read-back style"));
    }
    Ok((after, equal))
}

/// What the named styles sample reads of one `dxf` from the styles part's
/// own bytes, beside the typed read.
#[derive(Default)]
struct DxfBytes {
    /// Whether it has a `fill` child.
    fill: bool,
    /// Whether its fill's `patternFill` has no `patternType`.
    implied_solid: bool,
    /// Its border's `vertical` and `horizontal` children.
    inside_sides: u64,
}

/// The named styles sample (`specs/0032-read-breadth-styles.md`, "Harness
/// runs", `--namedstyles-sample`, and "Planning rulings during P1"). A
/// failure after the output is in place removes it; a refusal by the layer
/// is exit 2 with no output.
fn namedstyles_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let (named, named_xfs, builtin, custom) = {
        let styles = package.styles()?;
        let named = &styles.cell_styles;
        (
            named.len() as u64,
            styles.cell_style_xfs.len() as u64,
            named
                .iter()
                .filter(|style| style.builtin_id.is_some())
                .count() as u64,
            named.iter().filter(|style| style.custom_builtin).count() as u64,
        )
    };
    let mut resolved = 0u64;
    for index in 0..named_xfs {
        let index = u32::try_from(index).map_err(|_| Error::NamedstylesSample("input's index"))?;
        package.named_style(index)?;
        resolved += 1;
    }
    if resolved != named_xfs {
        return Err(Error::NamedstylesSample("input's named style count"));
    }
    let default_font_size = package.default_font()?.size.unwrap_or(0.0);
    let read = package.differential_styles()?.to_vec();
    let bytes = dxf_bytes(&mut package)?;
    if bytes.len() != read.len() {
        return Err(Error::NamedstylesSample("input's dxf count"));
    }
    let count = |given: fn(&Style) -> bool| read.iter().filter(|style| given(style)).count() as u64;
    let pairs = read.iter().zip(&bytes);
    let gradients = pairs
        .clone()
        .filter(|(style, bytes)| bytes.fill && style.fill.is_none())
        .count() as u64;
    let mut implied_solid = 0u64;
    for (style, bytes) in pairs.clone() {
        if bytes.implied_solid {
            if style.fill.map(|fill| fill.pattern) != Some(Pattern::Solid) {
                return Err(Error::NamedstylesSample("input's implied solid fill"));
            }
            implied_solid += 1;
        }
    }
    let inside_sides: u64 = bytes.iter().map(|bytes| bytes.inside_sides).sum();
    let writable: Vec<&Style> = read
        .iter()
        .filter(|style| {
            style.number_format.is_none()
                && style.alignment.is_none()
                && style.protection.is_none()
                && (style.font.is_some() || style.fill.is_some() || style.border.is_some())
        })
        .collect();
    for (at, style) in writable.iter().enumerate() {
        let index = package.add_differential_style(style)?;
        if u64::from(index) != (read.len() + at) as u64 {
            return Err(Error::NamedstylesSample("input's dxf index"));
        }
    }
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = namedstyles_read_back(output, &read, &writable);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let equal = checked?;
    let mut line = String::from("namedstyles");
    let values = [
        ("named", named.to_string()),
        ("named_xfs", named_xfs.to_string()),
        ("builtin", builtin.to_string()),
        ("custom", custom.to_string()),
        ("resolved", resolved.to_string()),
        ("default_font_size", default_font_size.to_string()),
        ("dxfs", read.len().to_string()),
        ("dxf_fonts", count(|style| style.font.is_some()).to_string()),
        ("dxf_fills", count(|style| style.fill.is_some()).to_string()),
        (
            "dxf_borders",
            count(|style| style.border.is_some()).to_string(),
        ),
        (
            "dxf_numfmts",
            count(|style| style.number_format.is_some()).to_string(),
        ),
        (
            "dxf_alignments",
            count(|style| style.alignment.is_some()).to_string(),
        ),
        (
            "dxf_protections",
            count(|style| style.protection.is_some()).to_string(),
        ),
        ("dxf_gradients", gradients.to_string()),
        ("dxf_implied_solid", implied_solid.to_string()),
        ("dxf_inside_sides", inside_sides.to_string()),
        ("dxf_written", writable.len().to_string()),
        ("dxf_equal", equal.to_string()),
    ];
    for (key, value) in values {
        line.push('\t');
        line.push_str(key);
        line.push('=');
        line.push_str(&value);
    }
    print_line(&line)
}

/// Reopens the named styles sample's output and asserts what
/// `--namedstyles-sample` says it asserts; gives the number of appended
/// differential styles that read back equal to the value read.
fn namedstyles_read_back(output: &Path, read: &[Style], written: &[&Style]) -> Result<u64, Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let back = package.differential_styles()?;
    if back.len() != read.len() + written.len() {
        return Err(Error::NamedstylesSample("output's dxf count"));
    }
    if back[..read.len()] != *read {
        return Err(Error::NamedstylesSample("output's kept dxf"));
    }
    let equal = back[read.len()..]
        .iter()
        .zip(written)
        .filter(|(back, written)| back == *written)
        .count() as u64;
    if equal != written.len() as u64 {
        return Err(Error::NamedstylesSample("output's appended dxf"));
    }
    Ok(equal)
}

/// The `dxf` children of the styles part's main-namespace `<dxfs>`, as
/// [`DxfBytes`] reads them from the part's bytes, in part order.
fn dxf_bytes<R: io::Read + io::Seek>(package: &mut Package<R>) -> Result<Vec<DxfBytes>, Error> {
    let part = package
        .workbook()?
        .styles
        .clone()
        .ok_or(Error::NamedstylesSample("input's styles part"))?;
    let index = package.part(&part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut root: Vec<u8> = Vec::new();
    // Each open element's local name and whether it is of the main
    // namespace, the root first.
    let mut path: Vec<(Vec<u8>, bool)> = Vec::new();
    let mut out: Vec<DxfBytes> = Vec::new();
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            path.pop();
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        if path.is_empty() {
            root = raw.to_vec();
        }
        let main = MAIN.contains(&element_namespace(raw, &root).as_slice());
        let local = tag_local_name(raw).to_vec();
        let is = |at: usize, name: &[u8]| path.get(at).is_some_and(|(n, m)| *m && n == name);
        if main && is(1, b"dxfs") {
            match path.len() {
                2 if local == b"dxf" => out.push(DxfBytes::default()),
                3 if is(2, b"dxf") && local == b"fill" => {
                    if let Some(last) = out.last_mut() {
                        last.fill = true;
                    }
                }
                4 if is(2, b"dxf")
                    && is(3, b"fill")
                    && local == b"patternFill"
                    && tag_attribute(raw, b"patternType").is_none() =>
                {
                    if let Some(last) = out.last_mut() {
                        last.implied_solid = true;
                    }
                }
                4 if is(2, b"dxf")
                    && is(3, b"border")
                    && (local == b"vertical" || local == b"horizontal") =>
                {
                    if let Some(last) = out.last_mut() {
                        last.inside_sides += 1;
                    }
                }
                _ => {}
            }
        }
        if !raw.ends_with(b"/>") {
            path.push((local, main));
        }
    }
    Ok(out)
}

/// Where a colour the colours sample meets sits: a font's, a fill's (either
/// colour), or a border side's.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ColorRole {
    Font,
    Fill,
    Border,
}

/// The colours of a font, a fill and a border, in the order
/// `--colors-sample` lists them: the font's colour; the fill's `fgColor` then
/// `bgColor`; the border's `left`, `right`, `top`, `bottom`, `diagonal`.
/// Absent colours give nothing.
fn colors_of(
    font: Option<&Font>,
    fill: Option<&Fill>,
    border: Option<&Border>,
) -> Vec<(ColorRole, Color)> {
    let mut colors = Vec::new();
    if let Some(color) = font.and_then(|font| font.color) {
        colors.push((ColorRole::Font, color));
    }
    if let Some(fill) = fill {
        for color in [fill.foreground, fill.background].into_iter().flatten() {
            colors.push((ColorRole::Fill, color));
        }
    }
    if let Some(border) = border {
        for side in [
            border.left,
            border.right,
            border.top,
            border.bottom,
            border.diagonal,
        ] {
            if let Some(color) = side.and_then(|side| side.color) {
                colors.push((ColorRole::Border, color));
            }
        }
    }
    colors
}

/// 64-bit FNV-1a over `entries`, each entry's bytes followed by a newline
/// (`specs/0032-read-breadth-styles.md`, "Planning rulings during P2",
/// ruling 10), as 16 lower-case hex digits.
fn fnv_digest(entries: &[String]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for entry in entries {
        for byte in entry.bytes().chain(*b"\n") {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    format!("{hash:016x}")
}

/// The colours sample (`specs/0032-read-breadth-styles.md`, "Harness runs",
/// `--colors-sample`, and "Planning rulings during P2"). The input is
/// written back unedited; a refusal by the layer is exit 2 with no output.
fn colors_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    // The four groups, in the order ruling 10 gives.
    let mut groups: [Vec<(ColorRole, Color)>; 4] = Default::default();
    let palette_override = {
        let styles = package.styles()?;
        for font in styles.typed_fonts() {
            groups[0].extend(colors_of(Some(font), None, None));
        }
        for fill in styles.typed_fills() {
            groups[1].extend(colors_of(None, fill.as_ref(), None));
        }
        for border in styles.typed_borders() {
            groups[2].extend(colors_of(None, None, Some(border)));
        }
        styles.indexed_colors.is_some()
    };
    for style in package.differential_styles()?.to_vec() {
        groups[3].extend(colors_of(
            style.font.as_ref(),
            style.fill.as_ref(),
            style.border.as_ref(),
        ));
    }
    let theme_part = package.theme_colors()?.is_some();
    let (mut rgb, mut theme, mut indexed, mut auto) = (0u64, 0u64, 0u64, 0u64);
    let (mut resolved, mut unresolved, mut tinted) = (0u64, 0u64, 0u64);
    let (mut dark_text, mut light_fill) = (0u64, 0u64);
    let mut digests = Vec::new();
    for group in &groups {
        let mut entries = Vec::new();
        for (role, color) in group {
            match color {
                Color::Rgb(_) => rgb += 1,
                Color::Theme { .. } => theme += 1,
                Color::Indexed(_) => indexed += 1,
                Color::Auto => auto += 1,
            }
            let Some(found) = package.resolve_color(color)? else {
                unresolved += 1;
                entries.push("none".to_string());
                continue;
            };
            resolved += 1;
            if found.tint.is_some() {
                tinted += 1;
            }
            entries.push(format!("{:08x}:{:08x}", found.rgb, found.tinted()));
            let channels = [found.rgb >> 16, found.rgb >> 8, found.rgb].map(|c| c & 0xFF);
            match (role, color) {
                (ColorRole::Font, Color::Theme { index: 1, .. })
                    if channels.iter().all(|c| *c < 128) =>
                {
                    dark_text += 1;
                }
                (ColorRole::Fill, Color::Theme { index: 0, .. })
                    if channels.iter().all(|c| *c >= 128) =>
                {
                    light_fill += 1;
                }
                _ => {}
            }
        }
        digests.push(fnv_digest(&entries));
    }
    let colors: u64 = groups.iter().map(|group| group.len() as u64).sum();
    if rgb + theme + indexed + auto != colors {
        return Err(Error::ColorsSample("colour count by form"));
    }
    if resolved + unresolved != colors {
        return Err(Error::ColorsSample("resolved count"));
    }
    write_into(package, output)?;
    let mut line = String::from("colors");
    let values = [
        ("colors", colors.to_string()),
        ("rgb", rgb.to_string()),
        ("theme", theme.to_string()),
        ("indexed", indexed.to_string()),
        ("auto", auto.to_string()),
        ("resolved", resolved.to_string()),
        ("unresolved", unresolved.to_string()),
        ("tinted", tinted.to_string()),
        ("theme_part", u64::from(theme_part).to_string()),
        ("scheme_dark_text", dark_text.to_string()),
        ("scheme_light_fill", light_fill.to_string()),
        ("palette_override", u64::from(palette_override).to_string()),
        ("digest_fonts", digests[0].clone()),
        ("digest_fills", digests[1].clone()),
        ("digest_borders", digests[2].clone()),
        ("digest_dxfs", digests[3].clone()),
    ];
    for (key, value) in values {
        line.push('\t');
        line.push_str(key);
        line.push('=');
        line.push_str(&value);
    }
    print_line(&line)
}

/// The survey sample (`specs/0033-read-breadth-survey.md`, "Harness runs",
/// `--kinds-sample`). The input is written back unedited; a refusal by the
/// layer is exit 2 with no output.
fn kinds_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let kinds = package.part_kinds()?;
    let entries = package.archive().entries().len() as u64;
    let parts: Vec<Vec<u8>> = package
        .workbook()?
        .sheets
        .iter()
        .map(|sheet| sheet.part.clone())
        .collect();
    let mut sheets = SheetParts::default();
    for part in &parts {
        let one = package.sheet_parts(part)?;
        for (sum, count) in [
            (&mut sheets.drawings, one.drawings),
            (&mut sheets.charts, one.charts),
            (&mut sheets.images, one.images),
            (&mut sheets.comments, one.comments),
            (&mut sheets.vml_drawings, one.vml_drawings),
            (&mut sheets.tables, one.tables),
            (&mut sheets.pivot_tables, one.pivot_tables),
            (&mut sheets.slicers, one.slicers),
            (&mut sheets.timelines, one.timelines),
            (&mut sheets.printer_settings, one.printer_settings),
            (&mut sheets.external_hyperlinks, one.external_hyperlinks),
            (&mut sheets.sparkline_groups, one.sparkline_groups),
        ] {
            *sum = sum.saturating_add(count);
        }
    }
    let fields = [
        ("worksheets", kinds.worksheets),
        ("chartsheets", kinds.chartsheets),
        ("dialogsheets", kinds.dialogsheets),
        ("macrosheets", kinds.macrosheets),
        ("charts", kinds.charts),
        ("chartex", kinds.chartex),
        ("drawings", kinds.drawings),
        ("images", kinds.images),
        ("tables", kinds.tables),
        ("pivot_tables", kinds.pivot_tables),
        ("pivot_caches", kinds.pivot_caches),
        ("pivot_cache_records", kinds.pivot_cache_records),
        ("external_links", kinds.external_links),
        ("vba_projects", kinds.vba_projects),
        ("slicers", kinds.slicers),
        ("slicer_caches", kinds.slicer_caches),
        ("timelines", kinds.timelines),
        ("timeline_caches", kinds.timeline_caches),
        ("comments", kinds.comments),
        ("threaded_comments", kinds.threaded_comments),
        ("persons", kinds.persons),
        ("custom_xml", kinds.custom_xml),
        ("connections", kinds.connections),
        ("query_tables", kinds.query_tables),
        ("custom_properties", kinds.custom_properties),
        ("printer_settings", kinds.printer_settings),
        ("calc_chains", kinds.calc_chains),
        ("other_typed", kinds.other_typed),
        ("untyped", kinds.untyped),
    ];
    let kinds_sum: u64 = fields.iter().map(|(_, count)| u64::from(*count)).sum();
    if kinds_sum + 1 != entries {
        return Err(Error::KindsSample("kinds sum"));
    }
    write_into(package, output)?;
    let mut line = String::from("kinds");
    let mut values: Vec<(&str, u64)> = fields
        .iter()
        .map(|(key, count)| (*key, u64::from(*count)))
        .collect();
    values.push(("entries", entries));
    values.push(("kinds_sum", kinds_sum));
    values.push(("sheets", parts.len() as u64));
    for (key, count) in [
        ("sheet_drawings", sheets.drawings),
        ("sheet_charts", sheets.charts),
        ("sheet_images", sheets.images),
        ("sheet_comments", sheets.comments),
        ("sheet_vml_drawings", sheets.vml_drawings),
        ("sheet_tables", sheets.tables),
        ("sheet_pivot_tables", sheets.pivot_tables),
        ("sheet_slicers", sheets.slicers),
        ("sheet_timelines", sheets.timelines),
        ("sheet_printer_settings", sheets.printer_settings),
        ("sheet_external_hyperlinks", sheets.external_hyperlinks),
        ("sheet_sparkline_groups", sheets.sparkline_groups),
    ] {
        values.push((key, u64::from(count)));
    }
    for (key, value) in values {
        line.push('\t');
        line.push_str(key);
        line.push('=');
        line.push_str(&value.to_string());
    }
    print_line(&line)
}

/// The pivot sample (`specs/0033-read-breadth-survey.md`, "Harness runs",
/// `--pivot-sample`). The input is written back unedited; a refusal by the
/// layer is exit 2 with no output.
fn pivot_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::{Axis, FieldRef, PivotSource, Subtotal};
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let tables = package.pivot_tables()?.to_vec();
    let mut counts: [(&str, u64); 26] = [
        ("pivot_tables", 0),
        ("with_sheet", 0),
        ("with_cache", 0),
        ("source_worksheet", 0),
        ("source_external", 0),
        ("source_other", 0),
        ("source_named", 0),
        ("source_ref", 0),
        ("cache_fields", 0),
        ("records_known", 0),
        ("fields", 0),
        ("named_fields", 0),
        ("axis_row", 0),
        ("axis_column", 0),
        ("axis_page", 0),
        ("axis_values", 0),
        ("data_field_flags", 0),
        ("row_fields", 0),
        ("column_fields", 0),
        ("page_fields", 0),
        ("values_refs", 0),
        ("data_fields", 0),
        ("named_data_fields", 0),
        ("subtotal_sum", 0),
        ("subtotal_other", 0),
        ("location_cells", 0),
    ];
    let mut add = |key: &str, by: u64| {
        // Every key below is one of the array's.
        let slot = counts.iter_mut().find(|(name, _)| *name == key).unwrap();
        slot.1 = slot.1.saturating_add(by);
    };
    let flag = |yes: bool| u64::from(yes);
    let mut names = Vec::new();
    let mut cache_names = Vec::new();
    for table in &tables {
        add("pivot_tables", 1);
        add("with_sheet", flag(table.sheet.is_some()));
        names.push(table.name.clone());
        if let Some(cache) = &table.cache {
            add("with_cache", 1);
            match &cache.source {
                PivotSource::Worksheet {
                    reference, name, ..
                } => {
                    add("source_worksheet", 1);
                    add("source_named", flag(name.is_some()));
                    add("source_ref", flag(reference.is_some()));
                }
                PivotSource::External { .. } => add("source_external", 1),
                PivotSource::Consolidation | PivotSource::Scenario => add("source_other", 1),
            }
            add("cache_fields", cache.fields.len() as u64);
            add("records_known", flag(cache.record_count.is_some()));
            cache_names.extend(cache.fields.iter().cloned());
        }
        add("fields", table.fields.len() as u64);
        for field in &table.fields {
            add("named_fields", flag(field.name.is_some()));
            match field.axis {
                Some(Axis::Row) => add("axis_row", 1),
                Some(Axis::Column) => add("axis_column", 1),
                Some(Axis::Page) => add("axis_page", 1),
                Some(Axis::Values) => add("axis_values", 1),
                None => {}
            }
            add("data_field_flags", flag(field.data_field));
        }
        add("row_fields", table.row_fields.len() as u64);
        add("column_fields", table.column_fields.len() as u64);
        add("page_fields", table.page_fields.len() as u64);
        let values = table
            .row_fields
            .iter()
            .chain(&table.column_fields)
            .chain(&table.page_fields)
            .filter(|field| matches!(field, FieldRef::Values))
            .count();
        add("values_refs", values as u64);
        add("data_fields", table.data_fields.len() as u64);
        for data in &table.data_fields {
            add("named_data_fields", flag(data.name.is_some()));
            if data.subtotal == Subtotal::Sum {
                add("subtotal_sum", 1);
            } else {
                add("subtotal_other", 1);
            }
        }
        let rows = u64::from(table.location.last.row - table.location.first.row) + 1;
        let cols = u64::from(table.location.last.col - table.location.first.col) + 1;
        add("location_cells", rows.saturating_mul(cols));
    }
    let count = |key: &str| {
        counts
            .iter()
            .find(|(name, _)| *name == key)
            .map_or(0, |(_, value)| *value)
    };
    let sources = count("source_worksheet") + count("source_external") + count("source_other");
    if sources != count("with_cache") {
        return Err(Error::PivotSample("source count"));
    }
    write_into(package, output)?;
    let mut line = String::from("pivots");
    for (key, value) in counts {
        line.push('\t');
        line.push_str(key);
        line.push('=');
        line.push_str(&value.to_string());
    }
    for (key, value) in [
        ("digest_names", fnv_digest(&names)),
        ("digest_fields", fnv_digest(&cache_names)),
    ] {
        line.push('\t');
        line.push_str(key);
        line.push('=');
        line.push_str(&value);
    }
    print_line(&line)
}

/// The external link sample (`specs/0033-read-breadth-survey.md`, "Harness
/// runs", `--extlink-sample`). The input is written back unedited; a refusal
/// by the layer is exit 2 with no output.
fn extlink_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::ExternalLinkKind;
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let links = package.external_links()?.to_vec();
    let (mut books, mut dde, mut ole, mut sheet_names, mut defined_names) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut targets = Vec::new();
    let mut sheets = Vec::new();
    for link in &links {
        match &link.kind {
            ExternalLinkKind::Book {
                target,
                sheet_names: names,
                defined_names: defined,
            } => {
                books += 1;
                sheet_names += names.len() as u64;
                defined_names += u64::from(*defined);
                targets.push(target.clone());
                sheets.extend(names.iter().cloned());
            }
            ExternalLinkKind::Dde { .. } => dde += 1,
            ExternalLinkKind::Ole { target, .. } => {
                ole += 1;
                targets.push(target.clone());
            }
        }
    }
    let count = links.len() as u64;
    if books + dde + ole != count {
        return Err(Error::ExtlinkSample("link count"));
    }
    write_into(package, output)?;
    let mut line = String::from("extlinks");
    for (key, value) in [
        ("links", count.to_string()),
        ("books", books.to_string()),
        ("dde", dde.to_string()),
        ("ole", ole.to_string()),
        ("sheet_names", sheet_names.to_string()),
        ("defined_names", defined_names.to_string()),
        ("digest_targets", fnv_digest(&targets)),
        ("digest_sheets", fnv_digest(&sheets)),
    ] {
        line.push('\t');
        line.push_str(key);
        line.push('=');
        line.push_str(&value);
    }
    print_line(&line)
}

/// The add-chart sample's counts of W: its charts, those on a two-cell
/// anchor, its drawing's root children, the entries under `xl/charts/` and
/// whether W has a drawing.
struct AddChartCounts {
    charts: u64,
    two: u64,
    anchors: u64,
    chart_parts: u64,
    drawing: bool,
}

/// The add-chart sample's counts of W in `package`.
fn add_chart_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<(AddChartCounts, Vec<emend::package::ChartEntry>), Error> {
    let charts = package.charts(part)?;
    let target = related(package, part, DRAWING)?;
    let anchors = match &target {
        Some(target) => drawing_root_children(package, target)?,
        None => 0,
    };
    let chart_parts = package
        .archive()
        .entries()
        .iter()
        .filter(|entry| entry.name().starts_with(b"xl/charts/"))
        .count() as u64;
    let counts = AddChartCounts {
        charts: charts.len() as u64,
        two: charts
            .iter()
            .filter(|entry| entry.anchor.kind == Anchor::TwoCell)
            .count() as u64,
        anchors,
        chart_parts,
        drawing: target.is_some(),
    };
    Ok((counts, charts))
}

/// The add-chart sample (`specs/0035-charts-write-first-set.md`, "Harness
/// runs", `--add-chart-sample`): one clustered column chart on W past its
/// last row, the output written and read back. A failure after the output
/// is in place removes it; a refusal by the layer is exit 2 with no output.
fn add_chart_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::{
        BarDirection, Grouping, LegendPosition, NewChart, NewPlot, NewSeries, NewSeriesName,
    };
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = first_worksheet(&mut package)?;
    let name = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.part == part)
        .map(|sheet| sheet.name.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let (before, _) = add_chart_counts(&mut package, &part)?;
    // Rows past the grid are the layer's to refuse, with its index message.
    let row = |more: u32| sampled.max_row.saturating_add(more);
    let sheet = format!("'{}'", name.replace('\'', "''"));
    let last = row(1);
    let categories = format!("{sheet}!$A$1:$A${last}");
    let first_values = format!("{sheet}!$B$1:$B${last}");
    let second_name = format!("{sheet}!$C$1");
    let second_values = format!("{sheet}!$C$1:$C${last}");
    let series = [
        NewSeries {
            name: Some(NewSeriesName::Text("emend series")),
            categories: Some(&categories),
            values: &first_values,
            sizes: None,
            color: None,
            labels: None,
        },
        NewSeries {
            name: Some(NewSeriesName::Reference(&second_name)),
            categories: None,
            values: &second_values,
            sizes: None,
            color: None,
            labels: None,
        },
    ];
    let chart = NewChart {
        plot: NewPlot::Bar {
            direction: BarDirection::Column,
            grouping: Grouping::Clustered,
        },
        title: Some("emend chart"),
        series: &series,
        category_axis_title: Some("emend categories"),
        value_axis_title: Some("emend values"),
        category_axis_format: None,
        value_axis_format: None,
        legend: Some(LegendPosition::Bottom),
        size: None,
        combined: None,
    };
    let anchor = ImageAnchor::TwoCell {
        from: CellRef {
            row: row(2),
            col: 1,
        },
        to: CellRef {
            row: row(18),
            col: 9,
        },
        from_offset: (0, 0),
        to_offset: (0, 0),
        edit_as: EditAs::OneCell,
    };
    package.add_chart(&part, anchor, &chart)?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = add_chart_read_back(output, &part, &before, &chart, anchor);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, equal) = checked?;
    edit_line_named(
        "addchart",
        &[
            ("charts_before", before.charts),
            ("charts_after", after.charts),
            ("read_back_equal", u64::from(equal)),
            ("anchors_before", before.anchors),
            ("anchors_after", after.anchors),
            ("chart_parts_before", before.chart_parts),
            ("chart_parts_after", after.chart_parts),
            ("drawing_created", u64::from(!before.drawing)),
            ("two_after", after.two),
        ],
    )
}

/// Reopens the add-chart sample's output and asserts one more chart, anchor,
/// chart part and two-cell chart, and the last chart of W read back as
/// `chart` on `anchor` gave it; gives the counts after and that equality.
fn add_chart_read_back(
    output: &Path,
    part: &[u8],
    before: &AddChartCounts,
    chart: &emend::package::NewChart<'_>,
    anchor: ImageAnchor,
) -> Result<(AddChartCounts, bool), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let (after, charts) = add_chart_counts(&mut package, part)?;
    if after.charts != before.charts + 1 {
        return Err(Error::AddChartSample("chart count"));
    }
    if after.anchors != before.anchors + 1 {
        return Err(Error::AddChartSample("drawing anchor count"));
    }
    if after.chart_parts != before.chart_parts + 1 {
        return Err(Error::AddChartSample("chart part count"));
    }
    if after.two != before.two + 1 {
        return Err(Error::AddChartSample("two-cell chart count"));
    }
    let equal = charts
        .last()
        .is_some_and(|entry| added_chart_reads_back(entry, chart, anchor));
    if !equal {
        return Err(Error::AddChartSample("read-back chart"));
    }
    Ok((after, equal))
}

/// Whether `entry` reads as `specs/0035-charts-write-first-set.md`, "The
/// call", says a bar chart added as `chart` on the two-cell `anchor` reads:
/// every field of the anchor and the chart compared.
fn added_chart_reads_back(
    entry: &emend::package::ChartEntry,
    chart: &emend::package::NewChart<'_>,
    anchor: ImageAnchor,
) -> bool {
    use emend::package::ChartAnchor;
    let ImageAnchor::TwoCell {
        from, to, edit_as, ..
    } = anchor
    else {
        return false;
    };
    let anchored = ChartAnchor {
        kind: Anchor::TwoCell,
        from: (from.col - 1, from.row - 1),
        to: Some((to.col - 1, to.row - 1)),
        edit_as: Some(edit_as),
    };
    entry.anchor == anchored && expected_bar_chart(chart).is_some_and(|want| entry.chart == want)
}

/// The chart `charts` reads back for the bar chart `chart` as
/// `specs/0035-charts-write-first-set.md`, "The call", gives it, every
/// field; `None` for another plot. The add-chart and chartsheet samples
/// share it.
fn expected_bar_chart(chart: &emend::package::NewChart<'_>) -> Option<emend::package::Chart> {
    use emend::package::{
        AxisKind, AxisPosition, BarDirection, Chart, ChartAxis, ChartKind, Data, NewPlot,
        NewSeriesName, Plot, Series, SeriesName,
    };
    let NewPlot::Bar {
        direction,
        grouping,
    } = chart.plot
    else {
        return None;
    };
    let data = |formula: &str| Data::Reference {
        formula: formula.to_owned(),
        cached: None,
    };
    let series = chart
        .series
        .iter()
        .zip(0u32..)
        .map(|(one, k)| Series {
            index: k,
            order: k,
            name: one.name.map(|name| match name {
                NewSeriesName::Reference(formula) => SeriesName::Reference(formula.to_owned()),
                NewSeriesName::Text(text) => SeriesName::Text(text.to_owned()),
            }),
            categories: one.categories.map(data),
            values: Some(data(one.values)),
            sizes: None,
            fill: None,
            smooth: false,
            marker: false,
            labels: false,
        })
        .collect();
    let (category, value) = if direction == BarDirection::Bar {
        (AxisPosition::Left, AxisPosition::Bottom)
    } else {
        (AxisPosition::Bottom, AxisPosition::Left)
    };
    let axis = |id, kind, position, crosses, title: Option<&str>, gridlines| ChartAxis {
        id,
        kind,
        position,
        crosses,
        title: title.map(str::to_owned),
        number_format: Some("General".to_owned()),
        min: None,
        max: None,
        deleted: false,
        major_gridlines: gridlines,
    };
    let expected = Chart {
        title: chart.title.map(str::to_owned),
        auto_title_deleted: chart.title.is_none(),
        plots: vec![Plot {
            kind: ChartKind::Bar,
            bar_direction: Some(direction),
            grouping: Some(grouping),
            scatter_style: None,
            vary_colors: false,
            series,
            axis_ids: vec![10, 20],
        }],
        axes: vec![
            axis(
                10,
                AxisKind::Category,
                category,
                20,
                chart.category_axis_title,
                false,
            ),
            axis(20, AxisKind::Value, value, 10, chart.value_axis_title, true),
        ],
        legend: chart.legend,
        pivot_source: None,
        style: None,
    };
    Some(expected)
}

/// The chartsheet sample (`specs/0038-chartsheets.md`, "Harness runs",
/// `--chartsheet-sample`): the add-chart sample's column chart on a new
/// chartsheet at 960 by 640 pixels, 9144000 by 6096000 EMU, the output
/// written and read back. A failure after the output is in place removes
/// it; a refusal by the layer is exit 2 with no output.
fn chartsheet_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::{
        BarDirection, Grouping, LegendPosition, NewChart, NewPlot, NewSeries, NewSeriesName,
    };
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = first_worksheet(&mut package)?;
    let name = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.part == part)
        .map(|sheet| sheet.name.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let sheets_before = package.workbook()?.sheets.len() as u64;
    let kept_before = chartsheet_charts(&mut package)?;
    let sheet = format!("'{}'", name.replace('\'', "''"));
    let last = sampled.max_row.saturating_add(1);
    let categories = format!("{sheet}!$A$1:$A${last}");
    let first_values = format!("{sheet}!$B$1:$B${last}");
    let second_name = format!("{sheet}!$C$1");
    let second_values = format!("{sheet}!$C$1:$C${last}");
    let series = [
        NewSeries {
            name: Some(NewSeriesName::Text("emend series")),
            categories: Some(&categories),
            values: &first_values,
            sizes: None,
            color: None,
            labels: None,
        },
        NewSeries {
            name: Some(NewSeriesName::Reference(&second_name)),
            categories: None,
            values: &second_values,
            sizes: None,
            color: None,
            labels: None,
        },
    ];
    let chart = NewChart {
        plot: NewPlot::Bar {
            direction: BarDirection::Column,
            grouping: Grouping::Clustered,
        },
        title: Some("emend chart"),
        series: &series,
        category_axis_title: Some("emend categories"),
        value_axis_title: Some("emend values"),
        category_axis_format: None,
        value_axis_format: None,
        legend: Some(LegendPosition::Bottom),
        size: Some((960, 640)),
        combined: None,
    };
    let added = package.add_chartsheet("emend chartsheet", &chart)?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = chartsheet_read_back(output, &added, sheets_before, kept_before, &chart);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (charts, series, sheets_after) = checked?;
    edit_line_named(
        "chartsheet",
        &[
            ("part_length", added.len() as u64),
            ("charts", charts),
            ("series", series),
            ("sheets_after", sheets_after),
            ("kept_charts", kept_before),
        ],
    )
}

/// The charts of every chartsheet of the workbook's sheet list.
fn chartsheet_charts<R: io::Read + io::Seek>(package: &mut Package<R>) -> Result<u64, Error> {
    let parts: Vec<Vec<u8>> = package
        .workbook()?
        .sheets
        .iter()
        .filter(|sheet| sheet.kind == SheetKind::Chartsheet)
        .map(|sheet| sheet.part.clone())
        .collect();
    let mut count = 0u64;
    for part in &parts {
        count += package.charts(part)?.len() as u64;
    }
    Ok(count)
}

/// Reopens the chartsheet sample's output and asserts one more sheet, the
/// last a chartsheet named `emend chartsheet` at `part`, its one chart read
/// back as `chart` gave it in an absolute anchor at 0,0, and the input's
/// chartsheets' charts kept; gives the chart and series counts of the new
/// part and the sheet count after.
fn chartsheet_read_back(
    output: &Path,
    part: &[u8],
    sheets_before: u64,
    kept_before: u64,
    chart: &emend::package::NewChart<'_>,
) -> Result<(u64, u64, u64), Error> {
    use emend::package::ChartAnchor;
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let sheets = package.workbook()?.sheets.clone();
    if sheets.len() as u64 != sheets_before + 1 {
        return Err(Error::ChartsheetSample("sheet count"));
    }
    let last = sheets
        .last()
        .ok_or(Error::ChartsheetSample("sheet count"))?;
    if last.kind != SheetKind::Chartsheet || last.name != "emend chartsheet" || last.part != part {
        return Err(Error::ChartsheetSample("added sheet"));
    }
    let charts = package.charts(part)?;
    let absolute = ChartAnchor {
        kind: Anchor::Absolute,
        from: (0, 0),
        to: None,
        edit_as: None,
    };
    let equal = charts.len() == 1
        && charts[0].anchor == absolute
        && expected_bar_chart(chart).is_some_and(|want| charts[0].chart == want);
    if !equal {
        return Err(Error::ChartsheetSample("read-back chart"));
    }
    if chartsheet_charts(&mut package)? != kept_before + 1 {
        return Err(Error::ChartsheetSample("kept chartsheets' chart count"));
    }
    let series = charts[0]
        .chart
        .plots
        .iter()
        .map(|plot| plot.series.len() as u64)
        .sum();
    Ok((charts.len() as u64, series, sheets.len() as u64))
}

/// The chart kinds sample (`specs/0035-charts-write-first-set.md`,
/// "Harness runs", `--chart-kinds-sample`, P2 ruling J): one chart of each
/// kind on W past its last row, the output written and read back. A failure
/// after the output is in place removes it; a refusal by the layer is exit
/// 2 with no output.
fn chart_kinds_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::{
        BarDirection, Grouping, LegendPosition, NewChart, NewPlot, NewSeries, NewSeriesName,
        ScatterStyle,
    };
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = first_worksheet(&mut package)?;
    let name = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.part == part)
        .map(|sheet| sheet.name.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let before = chart_kinds_counts(&mut package, &part)?.0;
    let sheet = format!("'{}'", name.replace('\'', "''"));
    // The references take row 2 at least, an empty sheet's included.
    let last = sampled.max_row.max(1).saturating_add(1);
    let series_name = format!("{sheet}!$B$1");
    let categories = format!("{sheet}!$A$2:$A${last}");
    let values = format!("{sheet}!$B$2:$B${last}");
    let series = [NewSeries {
        name: Some(NewSeriesName::Reference(&series_name)),
        categories: Some(&categories),
        values: &values,
        sizes: None,
        color: None,
        labels: None,
    }];
    let plots = [
        NewPlot::Bar {
            direction: BarDirection::Bar,
            grouping: Grouping::Clustered,
        },
        NewPlot::Bar {
            direction: BarDirection::Column,
            grouping: Grouping::Stacked,
        },
        NewPlot::Line {
            grouping: Grouping::Standard,
        },
        NewPlot::Pie,
        NewPlot::Scatter {
            style: ScatterStyle::LineMarker,
        },
        NewPlot::Area {
            grouping: Grouping::Standard,
        },
    ];
    let titles: Vec<String> = (0..plots.len())
        .map(|k| format!("emend kind {k}"))
        .collect();
    let mut calls = Vec::with_capacity(plots.len());
    for ((plot, title), k) in plots.into_iter().zip(&titles).zip(0u32..) {
        let pie = plot == NewPlot::Pie;
        let chart = NewChart {
            plot,
            title: Some(title),
            series: &series,
            category_axis_title: (!pie).then_some("emend x"),
            value_axis_title: (!pie).then_some("emend y"),
            category_axis_format: None,
            value_axis_format: None,
            legend: Some(LegendPosition::Right),
            size: Some((480, 288)),
            combined: None,
        };
        // Rows past the grid are the layer's to refuse, with its index message.
        let row = sampled
            .max_row
            .saturating_add(2)
            .saturating_add(k.saturating_mul(16));
        let anchor = ImageAnchor::OneCell {
            from: CellRef { row, col: 1 },
            offset: (0, 0),
        };
        package.add_chart(&part, anchor, &chart)?;
        calls.push((chart, anchor));
    }
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = chart_kinds_read_back(output, &part, &before, &calls);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, read_back) = checked?;
    edit_line_named(
        "chartkinds",
        &[
            ("charts_before", before.charts),
            ("charts_after", after.charts),
            ("kinds_read_back", read_back),
            ("one_after", after.one),
            ("chart_parts_after", after.chart_parts),
        ],
    )
}

/// The more kinds sample (`specs/0036-charts-write-second-set.md`,
/// "Harness runs", `--chart-more-sample`): a doughnut, a radar, a bubble
/// and a stock chart on W past its last row, the output written and read
/// back. A failure after the output is in place removes it; a refusal by
/// the layer is exit 2 with no output.
fn chart_more_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::{LegendPosition, NewChart, NewPlot, NewSeries, NewSeriesName, RadarStyle};
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = first_worksheet(&mut package)?;
    let name = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.part == part)
        .map(|sheet| sheet.name.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let before = chart_kinds_counts(&mut package, &part)?.0;
    let sheet = format!("'{}'", name.replace('\'', "''"));
    // The references take row 2 at least, an empty sheet's included.
    let last = sampled.max_row.max(1).saturating_add(1);
    let column = |letter: char| format!("{sheet}!${letter}$2:${letter}${last}");
    let series_name = format!("{sheet}!$B$1");
    let (a, b, c, d) = (column('A'), column('B'), column('C'), column('D'));
    let one = [NewSeries {
        name: Some(NewSeriesName::Reference(&series_name)),
        categories: Some(&a),
        values: &b,
        sizes: None,
        color: None,
        labels: None,
    }];
    let bubble = [NewSeries {
        sizes: Some(&c),
        ..one[0]
    }];
    let stock = [("high", &b), ("low", &c), ("close", &d)].map(|(name, values)| NewSeries {
        name: Some(NewSeriesName::Text(name)),
        categories: Some(&a),
        values,
        sizes: None,
        color: None,
        labels: None,
    });
    let plots: [(NewPlot, &[NewSeries<'_>]); 4] = [
        (NewPlot::Doughnut { hole_size: 50 }, &one),
        (
            NewPlot::Radar {
                style: RadarStyle::Marker,
            },
            &one,
        ),
        (NewPlot::Bubble, &bubble),
        (NewPlot::Stock, &stock),
    ];
    let titles: Vec<String> = (0..plots.len())
        .map(|k| format!("emend more {k}"))
        .collect();
    let mut calls = Vec::with_capacity(plots.len());
    for (((plot, series), title), k) in plots.into_iter().zip(&titles).zip(0u32..) {
        let axes = !matches!(plot, NewPlot::Doughnut { .. });
        let chart = NewChart {
            plot,
            title: Some(title),
            series,
            category_axis_title: axes.then_some("emend x"),
            value_axis_title: axes.then_some("emend y"),
            category_axis_format: None,
            value_axis_format: None,
            legend: Some(LegendPosition::Right),
            size: Some((480, 288)),
            combined: None,
        };
        // Rows past the grid are the layer's to refuse, with its index message.
        let row = sampled
            .max_row
            .saturating_add(2)
            .saturating_add(k.saturating_mul(16));
        let anchor = ImageAnchor::OneCell {
            from: CellRef { row, col: 1 },
            offset: (0, 0),
        };
        package.add_chart(&part, anchor, &chart)?;
        calls.push(chart);
    }
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = chart_more_read_back(output, &part, &before, &calls);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, read_back) = checked?;
    edit_line_named(
        "chartmore",
        &[
            ("charts_before", before.charts),
            ("charts_after", after.charts),
            ("kinds_read_back", read_back),
            ("chart_parts_after", after.chart_parts),
        ],
    )
}

/// Reopens the more kinds sample's output and asserts four more charts and
/// chart parts, and each of the last four charts of W read back with its
/// call's kind, series count, sizes on a bubble's series alone and axes
/// count; gives the counts after and how many read back.
fn chart_more_read_back(
    output: &Path,
    part: &[u8],
    before: &ChartKindsCounts,
    calls: &[emend::package::NewChart<'_>],
) -> Result<(ChartKindsCounts, u64), Error> {
    use emend::package::{ChartKind, NewPlot};
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let (after, charts) = chart_kinds_counts(&mut package, part)?;
    let added = calls.len() as u64;
    if after.charts != before.charts + added {
        return Err(Error::ChartMoreSample("chart count"));
    }
    if after.chart_parts != before.chart_parts + added {
        return Err(Error::ChartMoreSample("chart part count"));
    }
    let last = &charts[charts.len() - calls.len()..];
    let read_back = last
        .iter()
        .zip(calls)
        .filter(|(entry, chart)| {
            let (kind, axes) = match chart.plot {
                NewPlot::Doughnut { .. } => (ChartKind::Doughnut, 0),
                NewPlot::Radar { .. } => (ChartKind::Radar, 2),
                NewPlot::Bubble => (ChartKind::Bubble, 2),
                NewPlot::Stock => (ChartKind::Stock, 2),
                _ => return false,
            };
            let [plot] = entry.chart.plots.as_slice() else {
                return false;
            };
            plot.kind == kind
                && plot.series.len() == chart.series.len()
                && plot
                    .series
                    .iter()
                    .all(|one| one.sizes.is_some() == (kind == ChartKind::Bubble))
                && entry.chart.axes.len() == axes
        })
        .count() as u64;
    if read_back != added {
        return Err(Error::ChartMoreSample("read-back chart"));
    }
    Ok((after, read_back))
}

/// The 3D sample (`specs/0036-charts-write-second-set.md`, "Harness runs",
/// `--chart-3d-sample`): four 3D charts and a combined chart on W past its
/// last row, the output written and read back. A failure after the output
/// is in place removes it; a refusal by the layer is exit 2 with no output.
fn chart_3d_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::{
        BarDirection, Grouping, LegendPosition, NewChart, NewCombined, NewPlot, NewSeries,
        NewSeriesName,
    };
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = first_worksheet(&mut package)?;
    let name = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.part == part)
        .map(|sheet| sheet.name.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let before = chart_kinds_counts(&mut package, &part)?.0;
    let sheet = format!("'{}'", name.replace('\'', "''"));
    // The references take row 2 at least, an empty sheet's included.
    let last = sampled.max_row.max(1).saturating_add(1);
    let column = |letter: char| format!("{sheet}!${letter}$2:${letter}${last}");
    let (b_name, c_name) = (format!("{sheet}!$B$1"), format!("{sheet}!$C$1"));
    let (a, b, c) = (column('A'), column('B'), column('C'));
    let one = [NewSeries {
        name: Some(NewSeriesName::Reference(&b_name)),
        categories: Some(&a),
        values: &b,
        sizes: None,
        color: None,
        labels: None,
    }];
    let over_c = [NewSeries {
        name: Some(NewSeriesName::Reference(&c_name)),
        values: &c,
        ..one[0]
    }];
    let column_plot = NewPlot::Bar {
        direction: BarDirection::Column,
        grouping: Grouping::Clustered,
    };
    let line = NewCombined {
        plot: NewPlot::Line {
            grouping: Grouping::Standard,
        },
        series: &over_c,
        secondary_axis: true,
        secondary_axis_title: Some("emend y2"),
        secondary_axis_format: Some("0.0"),
    };
    let plots = [
        (
            NewPlot::Bar3D {
                direction: BarDirection::Column,
                grouping: Grouping::Clustered,
            },
            None,
        ),
        (
            NewPlot::Line3D {
                grouping: Grouping::Standard,
            },
            None,
        ),
        (NewPlot::Pie3D, None),
        (
            NewPlot::Area3D {
                grouping: Grouping::Stacked,
            },
            None,
        ),
        (column_plot, Some(line)),
    ];
    let titles: Vec<String> = (0..plots.len()).map(|k| format!("emend 3d {k}")).collect();
    let mut calls = Vec::with_capacity(plots.len());
    for (((plot, combined), title), k) in plots.into_iter().zip(&titles).zip(0u32..) {
        let chart = NewChart {
            plot,
            title: Some(title),
            series: &one,
            category_axis_title: None,
            value_axis_title: None,
            category_axis_format: None,
            value_axis_format: None,
            legend: Some(LegendPosition::Bottom),
            size: Some((480, 288)),
            combined,
        };
        // Rows past the grid are the layer's to refuse, with its index message.
        let row = sampled
            .max_row
            .saturating_add(2)
            .saturating_add(k.saturating_mul(16));
        let anchor = ImageAnchor::OneCell {
            from: CellRef { row, col: 1 },
            offset: (0, 0),
        };
        package.add_chart(&part, anchor, &chart)?;
        calls.push(chart);
    }
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = chart_3d_read_back(output, &part, &before, &calls);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, read_back) = checked?;
    edit_line_named(
        "chart3d",
        &[
            ("charts_before", before.charts),
            ("charts_after", after.charts),
            ("kinds_read_back", read_back),
            ("chart_parts_after", after.chart_parts),
        ],
    )
}

/// Reopens the 3D sample's output and asserts five more charts and chart
/// parts, and each of the last five charts of W read back with its call's
/// kinds, one series a plot and its axes: three for the 3D line, none for
/// the 3D pie, two for the other 3D plots, and the combined chart's four
/// with ids 10, 20, 30 and 40, the last's number format the call's; gives
/// the counts after and how many read back.
fn chart_3d_read_back(
    output: &Path,
    part: &[u8],
    before: &ChartKindsCounts,
    calls: &[emend::package::NewChart<'_>],
) -> Result<(ChartKindsCounts, u64), Error> {
    use emend::package::{ChartKind, NewPlot};
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let (after, charts) = chart_kinds_counts(&mut package, part)?;
    let added = calls.len() as u64;
    if after.charts != before.charts + added {
        return Err(Error::Chart3dSample("chart count"));
    }
    if after.chart_parts != before.chart_parts + added {
        return Err(Error::Chart3dSample("chart part count"));
    }
    let last = &charts[charts.len() - calls.len()..];
    let read_back = last
        .iter()
        .zip(calls)
        .filter(|(entry, chart)| {
            let chart_read = &entry.chart;
            let one_each = chart_read.plots.iter().all(|plot| plot.series.len() == 1);
            let kinds: Vec<ChartKind> = chart_read.plots.iter().map(|plot| plot.kind).collect();
            match (chart.plot, chart.combined) {
                (_, Some(combined)) => {
                    let ids: Vec<i64> = chart_read.axes.iter().map(|axis| axis.id).collect();
                    let format = combined.secondary_axis_format.map(str::to_owned);
                    one_each
                        && kinds == [ChartKind::Bar, ChartKind::Line]
                        && ids == [10, 20, 30, 40]
                        && chart_read.axes[3].number_format == format
                }
                (plot, None) => {
                    let (kind, axes) = match plot {
                        NewPlot::Bar3D { .. } => (ChartKind::Bar3D, 2),
                        NewPlot::Line3D { .. } => (ChartKind::Line3D, 3),
                        NewPlot::Pie3D => (ChartKind::Pie3D, 0),
                        NewPlot::Area3D { .. } => (ChartKind::Area3D, 2),
                        _ => return false,
                    };
                    one_each && kinds == [kind] && chart_read.axes.len() == axes
                }
            }
        })
        .count() as u64;
    if read_back != added {
        return Err(Error::Chart3dSample("read-back chart"));
    }
    Ok((after, read_back))
}

/// The last row the surface sample references (`specs/0039`, "Planning
/// rulings during P1", ruling A): at most 1000 data rows, rows 2 to 1001,
/// so that LibreOffice opens the output inside the harness's budget.
const SAMPLE_LAST_ROW: u32 = 1001;

/// The surface sample (`specs/0039-chart-kinds-and-formatting.md`, "Harness
/// runs", `--chart-surface-sample`): a contour and a 3D surface of three
/// series and a pie of pie and a bar of pie of one on W past its last row,
/// the output written and read back. A failure after the output is in place
/// removes it; a refusal by the layer is exit 2 with no output.
fn chart_surface_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::{
        LegendPosition, NewChart, NewPlot, NewSeries, NewSeriesName, OfPieKind, OfPieSplit,
    };
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = first_worksheet(&mut package)?;
    let name = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.part == part)
        .map(|sheet| sheet.name.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let before = chart_kinds_counts(&mut package, &part)?.0;
    let sheet = format!("'{}'", name.replace('\'', "''"));
    // The references take row 2 at least, an empty sheet's included, and
    // row SAMPLE_LAST_ROW at most (specs/0039, ruling A).
    let last = sampled
        .max_row
        .max(1)
        .saturating_add(1)
        .min(SAMPLE_LAST_ROW);
    let rows = u64::from(last - 1);
    let column = |letter: char| format!("{sheet}!${letter}$2:${letter}${last}");
    let named = |letter: char| format!("{sheet}!${letter}$1");
    let (b_name, c_name, d_name) = (named('B'), named('C'), named('D'));
    let (a, b, c, d) = (column('A'), column('B'), column('C'), column('D'));
    let over_b = NewSeries {
        name: Some(NewSeriesName::Reference(&b_name)),
        categories: Some(&a),
        values: &b,
        sizes: None,
        color: None,
        labels: None,
    };
    let three = [
        over_b,
        NewSeries {
            name: Some(NewSeriesName::Reference(&c_name)),
            values: &c,
            ..over_b
        },
        NewSeries {
            name: Some(NewSeriesName::Reference(&d_name)),
            values: &d,
            ..over_b
        },
    ];
    let one = [over_b];
    let plots: [(NewPlot, &[NewSeries<'_>]); 4] = [
        (NewPlot::Surface { wireframe: false }, &three),
        (NewPlot::Surface3D { wireframe: true }, &three),
        (
            NewPlot::OfPie {
                kind: OfPieKind::Pie,
                split: None,
            },
            &one,
        ),
        (
            NewPlot::OfPie {
                kind: OfPieKind::Bar,
                split: Some(OfPieSplit::Position(2)),
            },
            &one,
        ),
    ];
    let titles: Vec<String> = (0..plots.len())
        .map(|k| format!("emend surface {k}"))
        .collect();
    let mut calls = Vec::with_capacity(plots.len());
    for (((plot, series), title), k) in plots.into_iter().zip(&titles).zip(0u32..) {
        let surface = matches!(plot, NewPlot::Surface { .. } | NewPlot::Surface3D { .. });
        let chart = NewChart {
            plot,
            title: Some(title),
            series,
            category_axis_title: surface.then_some("emend x"),
            value_axis_title: surface.then_some("emend y"),
            category_axis_format: None,
            value_axis_format: None,
            legend: (!surface).then_some(LegendPosition::Right),
            size: Some((480, 288)),
            combined: None,
        };
        // Rows past the grid are the layer's to refuse, with its index message.
        let row = sampled
            .max_row
            .saturating_add(2)
            .saturating_add(k.saturating_mul(16));
        let anchor = ImageAnchor::OneCell {
            from: CellRef { row, col: 1 },
            offset: (0, 0),
        };
        package.add_chart(&part, anchor, &chart)?;
        calls.push(chart);
    }
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = chart_surface_read_back(output, &part, &before, &calls);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, kinds, series, read_back) = checked?;
    edit_line_named(
        "chartsurface",
        &[
            ("charts_before", before.charts),
            ("charts_after", after.charts),
            ("surface", kinds[0]),
            ("surface_3d", kinds[1]),
            ("of_pie", kinds[2]),
            ("series", series),
            ("rows", rows),
            ("read_back", read_back),
            ("chart_parts_after", after.chart_parts),
        ],
    )
}

/// Reopens the surface sample's output and asserts four more charts and
/// chart parts, the last four charts of W one contour, one 3D surface and
/// two of-pie plots of eight series in all, each read back as its call gave
/// it; gives the counts after, the plots of each kind, the series and how
/// many read back.
fn chart_surface_read_back(
    output: &Path,
    part: &[u8],
    before: &ChartKindsCounts,
    calls: &[emend::package::NewChart<'_>],
) -> Result<(ChartKindsCounts, [u64; 3], u64, u64), Error> {
    use emend::package::ChartKind;
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let (after, charts) = chart_kinds_counts(&mut package, part)?;
    let added = calls.len() as u64;
    if after.charts != before.charts + added {
        return Err(Error::ChartSurfaceSample("chart count"));
    }
    if after.chart_parts != before.chart_parts + added {
        return Err(Error::ChartSurfaceSample("chart part count"));
    }
    let last = &charts[charts.len() - calls.len()..];
    let plots = || last.iter().flat_map(|entry| &entry.chart.plots);
    let kinds = [ChartKind::Surface, ChartKind::Surface3D, ChartKind::OfPie]
        .map(|kind| plots().filter(|plot| plot.kind == kind).count() as u64);
    let series = plots().map(|plot| plot.series.len() as u64).sum::<u64>();
    if kinds != [1, 1, 2] {
        return Err(Error::ChartSurfaceSample("plot kinds"));
    }
    if series != 8 {
        return Err(Error::ChartSurfaceSample("series count"));
    }
    let read_back = last
        .iter()
        .zip(calls)
        .filter(|(entry, chart)| entry.chart == chart_as_read(chart))
        .count() as u64;
    if read_back != added {
        return Err(Error::ChartSurfaceSample("read-back chart"));
    }
    Ok((after, kinds, series, read_back))
}

/// The labels sample (`specs/0036-charts-write-second-set.md`, "Harness
/// runs", `--chart-labels-sample`): a clustered column chart of two
/// coloured, labelled series with both axis formats and a pie of one
/// coloured, labelled series on W past its last row, the output written and
/// read back. A failure after the output is in place removes it; a refusal
/// by the layer is exit 2 with no output.
fn chart_labels_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::{
        BarDirection, Grouping, LabelPosition, LegendPosition, NewChart, NewLabels, NewPlot,
        NewSeries, NewSeriesName,
    };
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = first_worksheet(&mut package)?;
    let name = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.part == part)
        .map(|sheet| sheet.name.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let before = chart_kinds_counts(&mut package, &part)?.0;
    let sheet = format!("'{}'", name.replace('\'', "''"));
    // The references take row 2 at least, an empty sheet's included.
    let last = sampled.max_row.max(1).saturating_add(1);
    let column = |letter: char| format!("{sheet}!${letter}$2:${letter}${last}");
    let (b_name, c_name) = (format!("{sheet}!$B$1"), format!("{sheet}!$C$1"));
    let (a, b, c) = (column('A'), column('B'), column('C'));
    let bar_labels = NewLabels {
        value: true,
        category: false,
        series_name: false,
        percent: false,
        position: Some(LabelPosition::OutsideEnd),
        number_format: Some("#,##0"),
    };
    let over_b = NewSeries {
        name: Some(NewSeriesName::Reference(&b_name)),
        categories: Some(&a),
        values: &b,
        sizes: None,
        color: None,
        labels: None,
    };
    let bar_series = [
        NewSeries {
            color: Some(0x1F_77B4),
            labels: Some(bar_labels),
            ..over_b
        },
        NewSeries {
            name: Some(NewSeriesName::Reference(&c_name)),
            values: &c,
            color: Some(0xFF_7F0E),
            labels: Some(bar_labels),
            ..over_b
        },
    ];
    let pie_series = [NewSeries {
        color: Some(0x2C_A02C),
        labels: Some(NewLabels {
            value: false,
            category: true,
            series_name: false,
            percent: true,
            position: Some(LabelPosition::BestFit),
            number_format: None,
        }),
        ..over_b
    }];
    let titles = ["emend labels 0", "emend labels 1"];
    let bar = NewChart {
        plot: NewPlot::Bar {
            direction: BarDirection::Column,
            grouping: Grouping::Clustered,
        },
        title: Some(titles[0]),
        series: &bar_series,
        category_axis_title: None,
        value_axis_title: None,
        category_axis_format: Some("@"),
        value_axis_format: Some("#,##0.00"),
        legend: Some(LegendPosition::Right),
        size: Some((480, 288)),
        combined: None,
    };
    let pie = NewChart {
        plot: NewPlot::Pie,
        title: Some(titles[1]),
        series: &pie_series,
        category_axis_format: None,
        value_axis_format: None,
        ..bar
    };
    let calls = [bar, pie];
    for (chart, k) in calls.iter().zip(0u32..) {
        // Rows past the grid are the layer's to refuse, with its index message.
        let row = sampled
            .max_row
            .saturating_add(2)
            .saturating_add(k.saturating_mul(16));
        let anchor = ImageAnchor::OneCell {
            from: CellRef { row, col: 1 },
            offset: (0, 0),
        };
        package.add_chart(&part, anchor, chart)?;
    }
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = chart_labels_read_back(output, &part, &before, &calls);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, read_back) = checked?;
    edit_line_named(
        "chartlabels",
        &[
            ("charts_before", before.charts),
            ("charts_after", after.charts),
            ("read_back", read_back),
            ("chart_parts_after", after.chart_parts),
        ],
    )
}

/// Reopens the labels sample's output and asserts two more charts and chart
/// parts, and each of the last two charts of W read back with its call's
/// kind, every series with its call's colour as its fill and with labels,
/// and the bar's two axes with the call's number formats; gives the counts
/// after and how many read back.
fn chart_labels_read_back(
    output: &Path,
    part: &[u8],
    before: &ChartKindsCounts,
    calls: &[emend::package::NewChart<'_>],
) -> Result<(ChartKindsCounts, u64), Error> {
    use emend::package::{ChartKind, NewPlot};
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let (after, charts) = chart_kinds_counts(&mut package, part)?;
    let added = calls.len() as u64;
    if after.charts != before.charts + added {
        return Err(Error::ChartLabelsSample("chart count"));
    }
    if after.chart_parts != before.chart_parts + added {
        return Err(Error::ChartLabelsSample("chart part count"));
    }
    let last = &charts[charts.len() - calls.len()..];
    let read_back = last
        .iter()
        .zip(calls)
        .filter(|(entry, chart)| {
            let read = &entry.chart;
            let [plot] = read.plots.as_slice() else {
                return false;
            };
            let series = plot.series.len() == chart.series.len()
                && plot
                    .series
                    .iter()
                    .zip(chart.series)
                    .all(|(read, call)| read.fill == call.color && read.labels);
            let formats: Vec<Option<&str>> = read
                .axes
                .iter()
                .map(|axis| axis.number_format.as_deref())
                .collect();
            let (kind, axes) = match chart.plot {
                NewPlot::Bar { .. } => (
                    ChartKind::Bar,
                    vec![chart.category_axis_format, chart.value_axis_format],
                ),
                NewPlot::Pie => (ChartKind::Pie, Vec::new()),
                _ => return false,
            };
            series && plot.kind == kind && formats == axes
        })
        .count() as u64;
    if read_back != added {
        return Err(Error::ChartLabelsSample("read-back chart"));
    }
    Ok((after, read_back))
}

/// The remove-chart sample's counts of a package: W's charts, the chart
/// parts under `xl/charts/` (not their rels, style or colours parts), the
/// root children of W's drawing, W's drawing part, the charts of every
/// other worksheet, and every entry's name.
struct RemoveChartCounts {
    charts: u64,
    chart_parts: u64,
    anchors: u64,
    drawing: Option<Vec<u8>>,
    others: u64,
    names: Vec<Vec<u8>>,
}

/// The remove-chart sample's counts of W in `package`.
fn remove_chart_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<RemoveChartCounts, Error> {
    let charts = package.charts(part)?.len() as u64;
    let target = related(package, part, DRAWING)?;
    let anchors = match &target {
        Some(target) => drawing_root_children(package, target)?,
        None => 0,
    };
    let names: Vec<Vec<u8>> = package
        .archive()
        .entries()
        .iter()
        .map(|entry| entry.name().to_vec())
        .collect();
    let chart_parts = names
        .iter()
        .filter(|name| {
            name.strip_prefix(b"xl/charts/".as_slice())
                .is_some_and(|rest| rest.starts_with(b"chart") && !rest.contains(&b'/'))
        })
        .count() as u64;
    let worksheets: Vec<Vec<u8>> = package
        .workbook()?
        .sheets
        .iter()
        .filter(|sheet| sheet.kind == SheetKind::Worksheet && sheet.part != part)
        .map(|sheet| sheet.part.clone())
        .collect();
    let mut others = 0u64;
    for sheet in worksheets {
        others += package.charts(&sheet)?.len() as u64;
    }
    Ok(RemoveChartCounts {
        charts,
        chart_parts,
        anchors,
        drawing: target,
        others,
        names,
    })
}

/// The remove-chart sample (`specs/0036-charts-write-second-set.md`,
/// "Harness runs", `--remove-chart-sample`): the add-chart sample's chart
/// added on W and written into memory, then that chart removed and, when
/// the input's W had one, its first chart too; the output written and read
/// back. A failure after the output is in place removes it; a refusal by
/// the layer is exit 2 with no output.
fn remove_chart_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::{
        BarDirection, Grouping, LegendPosition, NewChart, NewPlot, NewSeries, NewSeriesName,
    };
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = first_worksheet(&mut package)?;
    let name = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.part == part)
        .map(|sheet| sheet.name.clone())
        .ok_or(Error::NoWorksheet)?;
    let sampled = sample(&mut package, &part)?;
    let before = remove_chart_counts(&mut package, &part)?;
    let kept_removed = u64::from(before.charts > 0);
    // The add-chart sample's chart and anchor.
    let row = |more: u32| sampled.max_row.saturating_add(more);
    let sheet = format!("'{}'", name.replace('\'', "''"));
    let last = row(1);
    let categories = format!("{sheet}!$A$1:$A${last}");
    let first_values = format!("{sheet}!$B$1:$B${last}");
    let second_name = format!("{sheet}!$C$1");
    let second_values = format!("{sheet}!$C$1:$C${last}");
    let series = [
        NewSeries {
            name: Some(NewSeriesName::Text("emend series")),
            categories: Some(&categories),
            values: &first_values,
            sizes: None,
            color: None,
            labels: None,
        },
        NewSeries {
            name: Some(NewSeriesName::Reference(&second_name)),
            categories: None,
            values: &second_values,
            sizes: None,
            color: None,
            labels: None,
        },
    ];
    let chart = NewChart {
        plot: NewPlot::Bar {
            direction: BarDirection::Column,
            grouping: Grouping::Clustered,
        },
        title: Some("emend chart"),
        series: &series,
        category_axis_title: Some("emend categories"),
        value_axis_title: Some("emend values"),
        category_axis_format: None,
        value_axis_format: None,
        legend: Some(LegendPosition::Bottom),
        size: None,
        combined: None,
    };
    // Rows past the grid are the layer's to refuse, with its index message.
    let anchor = ImageAnchor::TwoCell {
        from: CellRef {
            row: row(2),
            col: 1,
        },
        to: CellRef {
            row: row(18),
            col: 9,
        },
        from_offset: (0, 0),
        to_offset: (0, 0),
        edit_as: EditAs::OneCell,
    };
    package.add_chart(&part, anchor, &chart)?;
    let mut buffer = io::Cursor::new(Vec::new());
    package.write_to(&mut buffer)?;
    buffer.set_position(0);
    let mut added = Package::open(buffer)?;
    let last = added
        .charts(&part)?
        .len()
        .checked_sub(1)
        .ok_or(Error::RemoveChartSample("chart count after the addition"))?;
    added.remove_chart(&part, last)?;
    if kept_removed == 1 {
        added.remove_chart(&part, 0)?;
    }
    write_into(added, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = remove_chart_read_back(input, output, &part, &before, kept_removed);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (after, parts_dropped, drawing_dropped) = checked?;
    edit_line_named(
        "removechart",
        &[
            ("charts_before", before.charts),
            ("charts_after", after.charts),
            ("kept_removed", kept_removed),
            ("chart_parts_before", before.chart_parts),
            ("chart_parts_after", after.chart_parts),
            ("parts_dropped", parts_dropped),
            ("drawing_dropped", drawing_dropped),
            ("anchors_before", before.anchors),
            ("anchors_after", after.anchors),
            ("charts_other_sheets", after.others),
        ],
    )
}

/// Reopens the remove-chart sample's output and asserts W's charts less the
/// one removed and every other worksheet's charts unchanged, and with
/// nothing kept removed the input's entries as P3 ruling M gives them;
/// gives the counts after, how many of the input's entries the output lacks
/// and whether W's drawing went.
fn remove_chart_read_back(
    input: &Path,
    output: &Path,
    part: &[u8],
    before: &RemoveChartCounts,
    kept_removed: u64,
) -> Result<(RemoveChartCounts, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = remove_chart_counts(&mut package, part)?;
    if after.charts != before.charts - kept_removed {
        return Err(Error::RemoveChartSample("chart count"));
    }
    if after.others != before.others {
        return Err(Error::RemoveChartSample("other sheets' chart count"));
    }
    let parts_dropped = before
        .names
        .iter()
        .filter(|name| !after.names.contains(name))
        .count() as u64;
    let drawing_dropped = u64::from(before.drawing.is_some() && after.drawing.is_none());
    if kept_removed == 0 {
        // The addition and its removal cancel (P3 ruling M): every entry of
        // the input is in the output with the same bytes, but for a rels
        // part the addition created, left with no relationship, and W's
        // drawing when it held no anchor, which goes with its rels part, W,
        // W's rels part and the content types changing.
        let emptied = before.drawing.as_ref().filter(|_| before.anchors == 0);
        if drawing_dropped != u64::from(emptied.is_some()) {
            return Err(Error::RemoveChartSample("drawing"));
        }
        let sheet_rels = rels_part_name(part);
        let changed: Vec<&[u8]> = match emptied {
            Some(_) => vec![part, &sheet_rels, CONTENT_TYPES_PART],
            None => Vec::new(),
        };
        let dropped: Vec<Vec<u8>> = emptied
            .map(|drawing| vec![drawing.clone(), rels_part_name(drawing)])
            .unwrap_or_default();
        let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
        let mut source = Package::open(BufReader::new(file))?;
        for name in &before.names {
            let Some(to) = package.part(name) else {
                if dropped.contains(name) {
                    continue;
                }
                return Err(Error::RemoveChartSample("entry names"));
            };
            if changed.contains(&name.as_slice()) {
                continue;
            }
            let from = source.part(name).ok_or(Error::NoSuchName)?;
            if !same_bytes(source.read_part(from)?, package.read_part(to)?)
                .map_err(|e| Error::ReadOutput(e.kind()))?
            {
                return Err(Error::RemoveChartSample("entry bytes"));
            }
        }
        for name in after
            .names
            .iter()
            .filter(|name| !before.names.contains(name))
        {
            let created = rels_part_source(name).filter(|source| {
                source.as_slice() == part || before.drawing.as_ref() == Some(source)
            });
            let Some(owner) = created else {
                return Err(Error::RemoveChartSample("entry names"));
            };
            if !package.relationships(&owner)?.all().is_empty() {
                return Err(Error::RemoveChartSample("created rels part"));
            }
        }
    }
    Ok((after, parts_dropped, drawing_dropped))
}

/// Whether two readers give the same bytes, read a block at a time.
fn same_bytes(mut left: impl io::Read, mut right: impl io::Read) -> io::Result<bool> {
    let mut a = [0u8; 8192];
    let mut b = [0u8; 8192];
    loop {
        let n = read_full(&mut left, &mut a)?;
        let m = read_full(&mut right, &mut b)?;
        if a[..n] != b[..m] {
            return Ok(false);
        }
        if n == 0 {
            return Ok(true);
        }
    }
}

/// Fills `buffer` from `reader` until it is full or the reader ends; gives
/// how many bytes it holds.
fn read_full(reader: &mut impl io::Read, buffer: &mut [u8]) -> io::Result<usize> {
    let mut filled = 0;
    while filled < buffer.len() {
        match reader.read(&mut buffer[filled..])? {
            0 => break,
            n => filled += n,
        }
    }
    Ok(filled)
}

/// The chart kinds sample's counts of W: its charts, those on a one-cell
/// anchor and the entries under `xl/charts/`.
struct ChartKindsCounts {
    charts: u64,
    one: u64,
    chart_parts: u64,
}

/// The chart kinds sample's counts of W in `package`, with its charts.
fn chart_kinds_counts<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
) -> Result<(ChartKindsCounts, Vec<emend::package::ChartEntry>), Error> {
    let charts = package.charts(part)?;
    let chart_parts = package
        .archive()
        .entries()
        .iter()
        .filter(|entry| entry.name().starts_with(b"xl/charts/"))
        .count() as u64;
    let counts = ChartKindsCounts {
        charts: charts.len() as u64,
        one: charts
            .iter()
            .filter(|entry| entry.anchor.kind == Anchor::OneCell)
            .count() as u64,
        chart_parts,
    };
    Ok((counts, charts))
}

/// Reopens the chart kinds sample's output and asserts six more charts,
/// one-cell charts and chart parts, and each of the last six charts of W
/// read back as its call gave it; gives the counts after and how many read
/// back.
fn chart_kinds_read_back(
    output: &Path,
    part: &[u8],
    before: &ChartKindsCounts,
    calls: &[(emend::package::NewChart<'_>, ImageAnchor)],
) -> Result<(ChartKindsCounts, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let (after, charts) = chart_kinds_counts(&mut package, part)?;
    let added = calls.len() as u64;
    if after.charts != before.charts + added {
        return Err(Error::ChartKindsSample("chart count"));
    }
    if after.one != before.one + added {
        return Err(Error::ChartKindsSample("one-cell chart count"));
    }
    if after.chart_parts != before.chart_parts + added {
        return Err(Error::ChartKindsSample("chart part count"));
    }
    let last = &charts[charts.len() - calls.len()..];
    let read_back = last
        .iter()
        .zip(calls)
        .filter(|(entry, (chart, anchor))| {
            let ImageAnchor::OneCell { from, .. } = *anchor else {
                return false;
            };
            let anchored = emend::package::ChartAnchor {
                kind: Anchor::OneCell,
                from: (from.col - 1, from.row - 1),
                to: None,
                edit_as: None,
            };
            entry.anchor == anchored && entry.chart == chart_as_read(chart)
        })
        .count() as u64;
    if read_back != added {
        return Err(Error::ChartKindsSample("read-back chart"));
    }
    Ok((after, read_back))
}

/// The chart `specs/0035-charts-write-first-set.md`, "The call", says
/// `Package::charts` reads back for `chart`, of any kind, the second set's
/// included (`specs/0036-charts-write-second-set.md`) and the surface and
/// of-pie plots (`specs/0039-chart-kinds-and-formatting.md`), its combined
/// plot apart.
fn chart_as_read(chart: &emend::package::NewChart<'_>) -> emend::package::Chart {
    use emend::package::{
        AxisKind, AxisPosition, BarDirection, Chart, ChartAxis, ChartKind, Data, NewPlot,
        NewSeriesName, Plot, Series, SeriesName,
    };
    let data = |formula: &str| Data::Reference {
        formula: formula.to_owned(),
        cached: None,
    };
    let series = chart
        .series
        .iter()
        .zip(0u32..)
        .map(|(one, k)| Series {
            index: k,
            order: k,
            name: one.name.map(|name| match name {
                NewSeriesName::Reference(formula) => SeriesName::Reference(formula.to_owned()),
                NewSeriesName::Text(text) => SeriesName::Text(text.to_owned()),
            }),
            categories: one.categories.map(data),
            values: Some(data(one.values)),
            sizes: one.sizes.map(data),
            fill: None,
            smooth: false,
            marker: chart.plot == NewPlot::Stock,
            labels: false,
        })
        .collect();
    let (kind, bar_direction, grouping, scatter_style) = match chart.plot {
        NewPlot::Bar {
            direction,
            grouping,
        } => (ChartKind::Bar, Some(direction), Some(grouping), None),
        NewPlot::Line { grouping } => (ChartKind::Line, None, Some(grouping), None),
        NewPlot::Pie => (ChartKind::Pie, None, None, None),
        NewPlot::Scatter { style } => (ChartKind::Scatter, None, None, Some(style)),
        NewPlot::Area { grouping } => (ChartKind::Area, None, Some(grouping), None),
        NewPlot::Doughnut { .. } => (ChartKind::Doughnut, None, None, None),
        NewPlot::Radar { .. } => (ChartKind::Radar, None, None, None),
        NewPlot::Bubble => (ChartKind::Bubble, None, None, None),
        NewPlot::Stock => (ChartKind::Stock, None, None, None),
        NewPlot::Bar3D {
            direction,
            grouping,
        } => (ChartKind::Bar3D, Some(direction), Some(grouping), None),
        NewPlot::Line3D { grouping } => (ChartKind::Line3D, None, Some(grouping), None),
        NewPlot::Pie3D => (ChartKind::Pie3D, None, None, None),
        NewPlot::Area3D { grouping } => (ChartKind::Area3D, None, Some(grouping), None),
        NewPlot::Surface { .. } => (ChartKind::Surface, None, None, None),
        NewPlot::Surface3D { .. } => (ChartKind::Surface3D, None, None, None),
        NewPlot::OfPie { .. } => (ChartKind::OfPie, None, None, None),
    };
    let round = matches!(
        kind,
        ChartKind::Pie | ChartKind::Pie3D | ChartKind::Doughnut | ChartKind::OfPie
    );
    let axis = |id, kind, position, crosses, title: Option<&str>, gridlines| ChartAxis {
        id,
        kind,
        position,
        crosses,
        title: title.map(str::to_owned),
        number_format: Some("General".to_owned()),
        min: None,
        max: None,
        deleted: false,
        major_gridlines: gridlines,
    };
    let (category, value) = if bar_direction == Some(BarDirection::Bar) {
        (AxisPosition::Left, AxisPosition::Bottom)
    } else {
        (AxisPosition::Bottom, AxisPosition::Left)
    };
    let first = if matches!(kind, ChartKind::Scatter | ChartKind::Bubble) {
        AxisKind::Value
    } else {
        AxisKind::Category
    };
    let (mut axis_ids, mut axes) = if round {
        (Vec::new(), Vec::new())
    } else {
        (
            vec![10, 20],
            vec![
                axis(10, first, category, 20, chart.category_axis_title, false),
                axis(20, AxisKind::Value, value, 10, chart.value_axis_title, true),
            ],
        )
    };
    // A line 3D plot whatever its grouping, a standard bar or area 3D plot
    // (`specs/0036-charts-write-second-set.md`, P2 ruling F) and both
    // surfaces (`specs/0039-chart-kinds-and-formatting.md`) write the series
    // axis.
    let depth = matches!(
        kind,
        ChartKind::Line3D | ChartKind::Surface | ChartKind::Surface3D
    ) || matches!(kind, ChartKind::Bar3D | ChartKind::Area3D)
        && grouping == Some(emend::package::Grouping::Standard);
    if depth {
        axis_ids.push(30);
        axes.push(ChartAxis {
            number_format: None,
            major_gridlines: false,
            ..axis(30, AxisKind::Series, AxisPosition::Bottom, 20, None, false)
        });
    }
    Chart {
        title: chart.title.map(str::to_owned),
        auto_title_deleted: chart.title.is_none(),
        plots: vec![Plot {
            kind,
            bar_direction,
            grouping,
            scatter_style,
            vary_colors: round,
            series,
            axis_ids,
        }],
        axes,
        legend: chart.legend,
        pivot_source: None,
        style: None,
    }
}

/// The charts sample (`specs/0034-charts-read.md`, "Harness runs",
/// `--charts-sample`, every field: P1's and the axes, legend, pivot source
/// and style P2 adds). The input is written back unedited; a refusal by the
/// layer is exit 2 with no output.
fn charts_sample(input: &Path, output: &Path) -> Result<(), Error> {
    use emend::package::{
        Anchor, AxisKind, BarDirection, ChartKind, Data, EditAs, Grouping, SeriesName,
    };
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    // A chartsheet's chart is read as a worksheet's
    // (`specs/0038-chartsheets.md`, "Reading").
    let parts: Vec<Vec<u8>> = package
        .workbook()?
        .sheets
        .iter()
        .filter(|sheet| matches!(sheet.kind, SheetKind::Worksheet | SheetKind::Chartsheet))
        .map(|sheet| sheet.part.clone())
        .collect();
    let mut counts: [(&str, u64); 50] = [
        ("sheets", 0),
        ("charts", 0),
        ("plots", 0),
        ("kind_bar", 0),
        ("kind_line", 0),
        ("kind_pie", 0),
        ("kind_doughnut", 0),
        ("kind_scatter", 0),
        ("kind_other", 0),
        ("combined", 0),
        ("bar_dir_bar", 0),
        ("bar_dir_col", 0),
        ("grouping_clustered", 0),
        ("grouping_stacked", 0),
        ("grouping_standard", 0),
        ("grouping_percent", 0),
        ("series", 0),
        ("name_ref", 0),
        ("name_text", 0),
        ("unnamed", 0),
        ("cat_ref", 0),
        ("cat_lit", 0),
        ("val_ref", 0),
        ("val_lit", 0),
        ("sizes", 0),
        ("cached", 0),
        ("fills", 0),
        ("smooth", 0),
        ("markers", 0),
        ("labels", 0),
        ("titles", 0),
        ("auto_title_deleted", 0),
        ("axes", 0),
        ("axis_cat", 0),
        ("axis_val", 0),
        ("axis_other", 0),
        ("axis_titles", 0),
        ("axis_numfmt", 0),
        ("axis_bounds", 0),
        ("axis_deleted", 0),
        ("gridlines", 0),
        ("legends", 0),
        ("pivot_charts", 0),
        ("styles", 0),
        ("anchor_two", 0),
        ("anchor_one", 0),
        ("anchor_abs", 0),
        ("edit_as_onecell", 0),
        ("edit_as_twocell", 0),
        ("edit_as_abs", 0),
    ];
    let mut add = |key: &str, by: u64| {
        // Every key below is one of the array's.
        let slot = counts.iter_mut().find(|(name, _)| *name == key).unwrap();
        slot.1 = slot.1.saturating_add(by);
    };
    let flag = |yes: bool| u64::from(yes);
    let (mut titles, mut names, mut formulas) = (Vec::new(), Vec::new(), Vec::new());
    let (mut numfmts, mut sources) = (Vec::new(), Vec::new());
    for part in &parts {
        add("sheets", 1);
        for entry in package.charts(part)? {
            add("charts", 1);
            match entry.anchor.kind {
                Anchor::TwoCell => add("anchor_two", 1),
                Anchor::OneCell => add("anchor_one", 1),
                Anchor::Absolute => add("anchor_abs", 1),
            }
            match entry.anchor.edit_as {
                Some(EditAs::OneCell) => add("edit_as_onecell", 1),
                Some(EditAs::TwoCell) => add("edit_as_twocell", 1),
                Some(EditAs::Absolute) => add("edit_as_abs", 1),
                None => {}
            }
            let chart = entry.chart;
            if let Some(title) = chart.title {
                add("titles", 1);
                titles.push(title);
            }
            add("auto_title_deleted", flag(chart.auto_title_deleted));
            for axis in chart.axes {
                add("axes", 1);
                add(
                    match axis.kind {
                        AxisKind::Category => "axis_cat",
                        AxisKind::Value => "axis_val",
                        AxisKind::Date | AxisKind::Series => "axis_other",
                    },
                    1,
                );
                add("axis_titles", flag(axis.title.is_some()));
                if let Some(format) = axis.number_format {
                    add("axis_numfmt", 1);
                    numfmts.push(format);
                }
                add(
                    "axis_bounds",
                    flag(axis.min.is_some() || axis.max.is_some()),
                );
                add("axis_deleted", flag(axis.deleted));
                add("gridlines", flag(axis.major_gridlines));
            }
            add("legends", flag(chart.legend.is_some()));
            if let Some(source) = chart.pivot_source {
                add("pivot_charts", 1);
                sources.push(source);
            }
            add("styles", flag(chart.style.is_some()));
            add("plots", chart.plots.len() as u64);
            add("combined", flag(chart.plots.len() >= 2));
            for plot in chart.plots {
                add(
                    match plot.kind {
                        ChartKind::Bar | ChartKind::Bar3D => "kind_bar",
                        ChartKind::Line | ChartKind::Line3D => "kind_line",
                        ChartKind::Pie | ChartKind::Pie3D => "kind_pie",
                        ChartKind::Doughnut => "kind_doughnut",
                        ChartKind::Scatter => "kind_scatter",
                        _ => "kind_other",
                    },
                    1,
                );
                match plot.bar_direction {
                    Some(BarDirection::Bar) => add("bar_dir_bar", 1),
                    Some(BarDirection::Column) => add("bar_dir_col", 1),
                    None => {}
                }
                match plot.grouping {
                    Some(Grouping::Clustered) => add("grouping_clustered", 1),
                    Some(Grouping::Stacked) => add("grouping_stacked", 1),
                    Some(Grouping::Standard) => add("grouping_standard", 1),
                    Some(Grouping::PercentStacked) => add("grouping_percent", 1),
                    None => {}
                }
                for series in plot.series {
                    add("series", 1);
                    match series.name {
                        Some(SeriesName::Reference(formula)) => {
                            add("name_ref", 1);
                            names.push(formula);
                        }
                        Some(SeriesName::Text(text)) => {
                            add("name_text", 1);
                            names.push(text);
                        }
                        None => add("unnamed", 1),
                    }
                    for (data, reference, literal) in [
                        (series.categories, "cat_ref", "cat_lit"),
                        (series.values, "val_ref", "val_lit"),
                        (series.sizes, "sizes", "sizes"),
                    ] {
                        match data {
                            Some(Data::Reference { formula, cached }) => {
                                add(reference, 1);
                                add("cached", flag(cached.is_some()));
                                formulas.push(formula);
                            }
                            Some(Data::Literal { .. }) => add(literal, 1),
                            None => {}
                        }
                    }
                    add("fills", flag(series.fill.is_some()));
                    add("smooth", flag(series.smooth));
                    add("markers", flag(series.marker));
                    add("labels", flag(series.labels));
                }
            }
        }
    }
    let count = |key: &str| {
        counts
            .iter()
            .find(|(name, _)| *name == key)
            .map_or(0, |(_, value)| *value)
    };
    let kinds: u64 = [
        "kind_bar",
        "kind_line",
        "kind_pie",
        "kind_doughnut",
        "kind_scatter",
        "kind_other",
    ]
    .iter()
    .map(|key| count(key))
    .sum();
    if kinds != count("plots") {
        return Err(Error::ChartsSample("plot count"));
    }
    if count("name_ref") + count("name_text") + count("unnamed") != count("series") {
        return Err(Error::ChartsSample("series count"));
    }
    if count("axis_cat") + count("axis_val") + count("axis_other") != count("axes") {
        return Err(Error::ChartsSample("axis count"));
    }
    write_into(package, output)?;
    let mut line = String::from("charts");
    for (key, value) in counts {
        line.push('\t');
        line.push_str(key);
        line.push('=');
        line.push_str(&value.to_string());
    }
    for (key, value) in [
        ("digest_titles", fnv_digest(&titles)),
        ("digest_names", fnv_digest(&names)),
        ("digest_formulas", fnv_digest(&formulas)),
        ("digest_numfmts", fnv_digest(&numfmts)),
        ("digest_sources", fnv_digest(&sources)),
    ] {
        line.push('\t');
        line.push_str(key);
        line.push('=');
        line.push_str(&value);
    }
    print_line(&line)
}

/// The attributes the line style sample's split and edit set on a `<col>`.
const LINESTYLE_SET: [&[u8]; 3] = [b"min", b"max", b"style"];

/// Reopens the line style sample's output and asserts what
/// `--linestyle-sample` says it asserts; gives W's reading, `col_split`,
/// the count of kept attributes and the cell formats after.
fn linestyle_read_back(
    output: &Path,
    part: &[u8],
    before: &GroupFacts,
    row: u32,
    style: u32,
    xfs_before: u64,
) -> Result<(GroupFacts, u64, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let after = group_facts(&mut package, part, (row, row))?;
    let xfs_after = package.styles()?.cell_xfs.len() as u64;
    if xfs_after != xfs_before + 1 || u64::from(style) != xfs_before {
        return Err(Error::LinestyleSample("cell format count"));
    }
    // A present row is edited in place, an absent one created ("Planning
    // rulings during P2", ruling 1).
    if after.rows != before.rows + 1 - before.in_window || after.in_window != 1 {
        return Err(Error::LinestyleSample("row count"));
    }
    let index = style.to_string().into_bytes();
    let styled_row = after.window_rows.first().is_some_and(|tag| {
        tag_attribute(tag, b"s").as_deref() == Some(index.as_slice())
            && tag_attribute(tag, b"customFormat").as_deref() == Some(b"1".as_slice())
    });
    if !styled_row {
        return Err(Error::LinestyleSample("row style"));
    }
    let column = covering(&after.cols, LINESTYLE_COLUMN);
    let styled_column = column.is_some_and(|tag| {
        col_span(tag) == Some((LINESTYLE_COLUMN, LINESTYLE_COLUMN))
            && tag_attribute(tag, b"style").as_deref() == Some(index.as_slice())
    });
    if !styled_column {
        return Err(Error::LinestyleSample("column style"));
    }
    let input = covering(&before.cols, LINESTYLE_COLUMN);
    let keep = |tag: Option<&[u8]>| -> Vec<(Vec<u8>, Vec<u8>)> {
        tag.map(tag_attributes)
            .unwrap_or_default()
            .into_iter()
            .filter(|(name, _)| !LINESTYLE_SET.contains(&name.as_slice()))
            .collect()
    };
    let kept = keep(column);
    if kept != keep(input) {
        return Err(Error::LinestyleSample("kept col attributes"));
    }
    let split = match input.and_then(col_span) {
        Some((min, max)) => after
            .cols
            .iter()
            .filter_map(|tag| col_span(tag))
            .filter(|(from, to)| *from >= min && *to <= max)
            .count() as u64,
        None => 1,
    };
    if !(1..=3).contains(&split) {
        return Err(Error::LinestyleSample("col split"));
    }
    Ok((after, split, kept.len() as u64, xfs_after))
}

/// Whether the part's root has a main-namespace child named `name`
/// (`tableParts` for the table sample, `legacyDrawing` for the comment
/// sample), and whether the root binds a prefix to the relationships
/// namespace, by this tool's own pass over the part's tokens.
fn container_and_binding<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
    name: &[u8],
) -> Result<(bool, bool), Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut root: Vec<u8> = Vec::new();
    let mut found = false;
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            depth = depth.saturating_sub(1);
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        if depth == 1 {
            root = raw.to_vec();
        } else if depth == 2
            && tag_local_name(raw) == name
            && MAIN.contains(&element_namespace(raw, &root).as_slice())
        {
            found = true;
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
        }
    }
    Ok((found, binds_relationships(&root)))
}

/// The thirteen core properties, for the counts.
const PROPERTIES: [Property; 13] = [
    Property::Title,
    Property::Subject,
    Property::Creator,
    Property::Description,
    Property::Language,
    Property::Keywords,
    Property::LastModifiedBy,
    Property::Revision,
    Property::Category,
    Property::ContentStatus,
    Property::Version,
    Property::Created,
    Property::Modified,
];

/// The values the properties sample sets.
const SAMPLE_PROPERTIES: [(Property, &str); 3] = [
    (Property::Title, "emend sample"),
    (Property::Keywords, "emend"),
    (Property::Modified, "2026-01-01T00:00:00Z"),
];

/// How many of the thirteen properties `properties` holds.
fn property_count(properties: &Properties) -> u64 {
    PROPERTIES
        .iter()
        .filter(|property| properties.get(**property).is_some())
        .count() as u64
}

/// The properties sample (`specs/0019-names-links-properties-protection.md`,
/// "Harness runs", `--props-sample`): three properties set, the output
/// written and read back. A failure after the output is in place removes
/// it; a refusal by the layer is exit 2 with no output.
fn props_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let core_before = package
        .relationships(b"")?
        .first_of(CORE_PROPERTIES)
        .is_some();
    let props_before = property_count(&package.properties()?);
    for (property, value) in SAMPLE_PROPERTIES {
        package.set_property(property, Some(value))?;
    }
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = props_read_back(output);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (read_back, props_after) = checked?;
    edit_line_named(
        "props",
        &[
            ("core_before", u64::from(core_before)),
            ("title_after", read_back[0]),
            ("keywords_after", read_back[1]),
            ("modified_after", read_back[2]),
            ("props_before", props_before),
            ("props_after", props_after),
        ],
    )
}

/// Reopens the properties sample's output: 1 for each of the three values
/// read back as set, each asserted, and the count of properties held.
fn props_read_back(output: &Path) -> Result<([u64; 3], u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let properties = package.properties()?;
    let mut read_back = [0u64; 3];
    for (at, (property, value)) in SAMPLE_PROPERTIES.iter().enumerate() {
        read_back[at] = u64::from(properties.get(*property) == Some(*value));
        if read_back[at] != 1 {
            return Err(Error::PropsSample("property read back"));
        }
    }
    Ok((read_back, property_count(&properties)))
}

/// The protection sample (`specs/0019-names-links-properties-protection.md`,
/// "Harness runs", `--protect-sample`): W and the workbook protected with
/// the password `emend`, the output written and read back.
fn protect_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let book = workbook.part.clone();
    let part = workbook
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let sheet_before = root_children(&mut package, &part, b"sheetProtection")?;
    let book_before = root_children(&mut package, &book, b"workbookProtection")?;
    let options = SheetProtection {
        sort: false,
        auto_filter: false,
        ..SheetProtection::default()
    };
    package.protect_sheet(&part, Some("emend"), options)?;
    package.protect_workbook(Some("emend"), true, false)?;
    write_into(package, output)?;
    // The output is in place: a failure from here on removes it.
    let checked = protect_read_back(output, &part, &book);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (sheet_after, book_after) = checked?;
    let mut line = String::from("protect");
    for (key, value) in [
        ("sheet_prot_before", sheet_before),
        ("sheet_prot_after", sheet_after),
        ("book_prot_before", book_before),
        ("book_prot_after", book_after),
    ] {
        line.push_str(&format!("\t{key}={value}"));
    }
    line.push_str(&format!("\thash={:04X}", legacy_password_hash("emend")));
    print_line(&line)
}

/// Reopens the protection sample's output: one `sheetProtection` in W and
/// one `workbookProtection` in the workbook, each asserted.
fn protect_read_back(output: &Path, part: &[u8], book: &[u8]) -> Result<(u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let sheet_after = root_children(&mut package, part, b"sheetProtection")?;
    let book_after = root_children(&mut package, book, b"workbookProtection")?;
    if sheet_after != 1 {
        return Err(Error::ProtectSample("sheetProtection count"));
    }
    if book_after != 1 {
        return Err(Error::ProtectSample("workbookProtection count"));
    }
    Ok((sheet_after, book_after))
}

/// The main-namespace children of a part's root named `local`, by this
/// tool's own pass over the part's tokens.
fn root_children<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    part: &[u8],
    local: &[u8],
) -> Result<u64, Error> {
    let index = package.part(part).ok_or(Error::NoSuchName)?;
    let mut stream = Stream::new(package.read_part(index)?, index)?;
    let mut depth = 0usize;
    let mut root: Vec<u8> = Vec::new();
    let mut count = 0u64;
    while let Some(token) = stream.next()? {
        let raw = token.raw;
        if raw.starts_with(b"</") {
            depth = depth.saturating_sub(1);
            continue;
        }
        if !raw.starts_with(b"<") || raw.starts_with(b"<?") || raw.starts_with(b"<!") {
            continue;
        }
        depth += 1;
        if depth == 1 {
            root = raw.to_vec();
        } else if depth == 2
            && tag_local_name(raw) == local
            && MAIN.contains(&element_namespace(raw, &root).as_slice())
        {
            count += 1;
        }
        if raw.ends_with(b"/>") {
            depth -= 1;
        }
    }
    Ok(count)
}

/// The defined names whose scope is the sheet at index `at`.
fn local_names(names: &[emend::package::DefinedName], at: u32) -> u64 {
    names.iter().filter(|name| name.sheet == Some(at)).count() as u64
}

/// Reopens the names sample's output: its name count, asserted two more
/// than `names_before`; the names scoped to W, asserted one more than
/// `local_before`; the chain and the flag; and, after `emend_local` for
/// `sheet` is removed and the package written into memory and reopened, the
/// name count there, asserted one more than `names_before`.
fn names_read_back(
    output: &Path,
    at: u32,
    sheet: &str,
    names_before: u64,
    local_before: u64,
) -> Result<(u64, u64, bool, bool, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let names_after = workbook.defined_names().len() as u64;
    let local_after = local_names(workbook.defined_names(), at);
    let chain_after = workbook.calc_chain.is_some();
    let full_calc = workbook.full_calc_on_load;
    if names_after != names_before + 2 {
        return Err(Error::NamesSample("defined name count"));
    }
    if local_after != local_before + 1 {
        return Err(Error::NamesSample("count of names scoped to the sheet"));
    }
    package.remove_defined_name("emend_local", Some(sheet))?;
    let mut buffer = io::Cursor::new(Vec::new());
    package.write_to(&mut buffer)?;
    buffer.set_position(0);
    let mut removed = Package::open(buffer)?;
    let names_removed = removed.workbook()?.defined_names().len() as u64;
    if names_removed != names_before + 1 {
        return Err(Error::NamesSample("defined name count after the removal"));
    }
    Ok((
        names_after,
        local_after,
        chain_after,
        full_calc,
        names_removed,
    ))
}

/// The date and time the date sample writes: its serial is 45351.5 in the
/// 1900 system and 43889.5 in the 1904 system (`specs/0012-styles.md`,
/// "Module `dates`", the pinned values plus noon).
const SAMPLE_DATE: DateTime = DateTime {
    year: 2024,
    month: 2,
    day: 29,
    hour: 12,
    minute: 0,
    second: 0,
    millisecond: 0,
};

/// The sample dated cell of `specs/0012-styles.md`, "Harness runs":
/// `--style-sample`'s new cell alone.
///
/// The first sheet of kind `Worksheet` in workbook order is read once for M
/// ([`Sampled`]); one cell format is added over no base with the built-in
/// id 22, and `(M + 1, 1)` is set to `Number` of [`SAMPLE_DATE`]'s serial in
/// the workbook's epoch with that format. The output is written, reopened,
/// and its styles part and the same sheet read back. `xfs_after` is asserted
/// equal to X + 1 and `cells_after` to `cells_before + 1`; either failing
/// removes the output, as the style sample's assertion does. The line
/// carries the serial, which is this tool's constant and no workbook's
/// content, and counts.
fn date_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let epoch = Epoch::of(workbook);
    let part = workbook
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let xfs_before = package.styles()?.cell_xfs.len() as u64;
    let before = sample(&mut package, &part)?;
    let new_row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
    let new_cell = CellRef {
        row: new_row,
        col: 1,
    };
    let serial = SAMPLE_DATE
        .serial(epoch)
        .expect("2024-02-29 12:00 is a date in both epochs");

    let dated = package.add_cell_style(None, NumberFormat::BuiltIn(22))?;
    package.set_cell(&part, new_cell, Value::Number(serial));
    package.set_cell_style(&part, new_cell, Some(dated));

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    // The output is in place: a failure from here on removes it, as
    // `style_sample`'s assertion does.
    let checked = date_read_back(
        output,
        &part,
        new_cell,
        (epoch, serial, dated),
        (xfs_before + 1, before.cells + 1),
    );
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (xfs_after, cells_after, read_back) = checked?;
    let mut line = format!(
        "edit\tdate1904={}\tserial={serial}",
        u8::from(epoch == Epoch::Y1904)
    );
    for (key, value) in [
        ("added", 1),
        ("xfs_before", xfs_before),
        ("xfs_after", xfs_after),
        ("cells_before", before.cells),
        ("cells_after", cells_after),
        ("read_back", read_back),
    ] {
        line.push_str(&format!("\t{key}={value}"));
    }
    print_line(&line)
}

/// Reopens the output for the date sample's line: the styles part's cell
/// format count, asserted equal to `expected.0`; the edited sheet's cell
/// count, asserted equal to `expected.1`; and the read-back, 1 when the new
/// cell holds the serial written with `s` the new format and the serial
/// reads back through [`DateTime::from_serial`] in the same epoch as
/// [`SAMPLE_DATE`], else 0.
fn date_read_back(
    output: &Path,
    part: &[u8],
    new_cell: CellRef,
    written: (Epoch, f64, u32),
    expected: (u64, u64),
) -> Result<(u64, u64, u64), Error> {
    let (epoch, serial, dated) = written;
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let xfs_after = package.styles()?.cell_xfs.len() as u64;
    if xfs_after != expected.0 {
        return Err(Error::XfsAfter {
            expected: expected.0,
            found: xfs_after,
        });
    }
    let mut cells_after = 0u64;
    let mut read_back = 0u64;
    for cell in package.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        if cell.reference == new_cell
            && cell.number() == Some(serial)
            && cell.style == Some(dated)
            && cell.number().and_then(|n| DateTime::from_serial(epoch, n)) == Some(SAMPLE_DATE)
        {
            read_back = 1;
        }
    }
    if cells_after != expected.1 {
        return Err(Error::CellsAfter {
            expected: expected.1,
            found: cells_after,
        });
    }
    Ok((xfs_after, cells_after, read_back))
}

/// Prints one `edit` line: the word, then tab-separated `key=value`.
fn edit_line(pairs: &[(&str, u64)]) -> Result<(), Error> {
    edit_line_named("edit", pairs)
}

/// [`edit_line`] with another first word.
fn edit_line_named(word: &str, pairs: &[(&str, u64)]) -> Result<(), Error> {
    let mut line = String::from(word);
    for (key, value) in pairs {
        line.push('\t');
        line.push_str(key);
        line.push('=');
        line.push_str(&value.to_string());
    }
    print_line(&line)
}

/// Prints `line` and a line feed on stdout, reporting a failure to write or
/// to flush.
fn print_line(line: &str) -> Result<(), Error> {
    let mut out = io::stdout().lock();
    writeln!(out, "{line}").map_err(|e| Error::Report(e.kind()))?;
    out.flush().map_err(|e| Error::Report(e.kind()))
}

/// Writes the package back, reporting a buffering failure rather than losing
/// it in a drop, as [`write()`] does for the archive.
fn write_package<R: io::Read + io::Seek>(package: Package<R>, file: File) -> Result<(), Error> {
    let mut out = BufWriter::new(file);
    package.write_to(&mut out)?;
    out.into_inner()
        .map_err(|e| Error::Finish(e.error().kind()))?
        .sync_all()
        .map_err(|e| Error::Finish(e.kind()))?;
    Ok(())
}

/// Whether an entry has exactly that name.
fn named(names: &[Vec<u8>], name: &[u8]) -> bool {
    names.iter().any(|entry| entry.as_slice() == name)
}

/// Whether an entry has that name after ASCII case folding, and not before.
///
/// This is what `case_only` counts: it measures the exact-bytes rule instead
/// of assuming it, so a corpus that needed case folding would say so.
fn named_case_only(names: &[Vec<u8>], name: &[u8]) -> bool {
    !named(names, name) && names.iter().any(|entry| entry.eq_ignore_ascii_case(name))
}

/// Reads the shared strings table and counts it.
///
/// Two numbers, both derived from the items and neither of them an item:
/// `items` is the table's length and `bytes` the sum of the items' UTF-8
/// lengths. The sum is taken over the public accessor rather than a method on
/// the table, because nothing in the library needs it; the tool wants it to
/// show the corpus's tables have been read whole and not truncated.
///
/// A workbook with no shared strings relationship gives an empty table, so
/// this prints `items=0 bytes=0` for one rather than refusing it: the absence
/// is not an error, and the run script knows which corpus files have a table.
fn shared_strings_line(input: &Path) -> Result<String, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let table = package.shared_strings()?;
    let items = u64::from(table.len());
    // `u64` and not `usize`: the sum of a corpus table's items overflows
    // neither, but `usize` is 32 bits on the target that matters and every
    // count in this tool is already `u64`.
    let mut bytes = 0u64;
    for index in 0..table.len() {
        let item = table.get(index).unwrap_or_default();
        bytes += item.len() as u64;
    }
    Ok(format!("strings\titems={items}\tbytes={bytes}"))
}

/// The line `--read-sample` prints (`specs/0027-read-back.md`, "Harness
/// runs"): what the typed reads give over the workbook, counts only.
fn read_line(input: &Path) -> Result<String, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let worksheets: Vec<Vec<u8>> = package
        .workbook()?
        .sheets
        .iter()
        .filter(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .collect();
    let part = worksheets.first().cloned().ok_or(Error::NoWorksheet)?;
    let (mut cells, mut styled, mut resolved) = (0u64, 0u64, 0u64);
    let mut used = std::collections::BTreeSet::new();
    for sheet in &worksheets {
        let mut styles = Vec::new();
        for cell in package.cells(sheet)? {
            let cell = cell?;
            cells += 1;
            if let Some(style) = cell.style {
                styled += 1;
                styles.push(style);
            }
        }
        // Each cell's style resolved, as a caller resolves `Cell::style`.
        for style in styles {
            package.cell_style(style)?;
            resolved += 1;
            used.insert(style);
        }
    }
    if resolved != styled {
        return Err(Error::ReadSample("resolved styles"));
    }
    let mut formats = Vec::new();
    for &index in &used {
        formats.push(package.cell_style(index)?);
    }
    let fonts = distinct(formats.iter().map(|format| &format.font));
    let fills = distinct(formats.iter().filter_map(|format| format.fill.as_ref()));
    let borders = distinct(formats.iter().map(|format| &format.border));
    let gradients = formats
        .iter()
        .filter(|format| format.fill.is_none())
        .count() as u64;
    let codes = formats
        .iter()
        .filter(|format| matches!(format.number_format, NumberFormat::Code(_)))
        .count() as u64;
    let columns = package.columns(&part)?.len() as u64;
    let mut rows = 0u64;
    for row in package.rows(&part)? {
        row?;
        rows += 1;
    }
    let settings = package.sheet_settings(&part)?;
    let table = package.shared_strings()?;
    let (mut items, mut equal) = (0u64, 0u64);
    for (at, item) in (0u32..).zip(package.shared_string_items()?) {
        let item = item?;
        items += 1;
        if table.get(at) == Some(item.text.as_str()) {
            equal += 1;
        }
    }
    if items != u64::from(table.len()) {
        return Err(Error::ReadSample("item count"));
    }
    if equal != items {
        return Err(Error::ReadSample("item text"));
    }
    let fields = [
        ("cells", cells),
        ("styled", styled),
        ("resolved", resolved),
        ("fonts", fonts),
        ("fills", fills),
        ("borders", borders),
        ("gradients", gradients),
        ("codes", codes),
        ("columns", columns),
        ("rows", rows),
        ("merges", settings.merges.len() as u64),
        ("links", settings.hyperlinks.len() as u64),
        ("frozen", u64::from(settings.freeze.is_some())),
        ("protected", u64::from(settings.protection.is_some())),
        ("items", items),
        ("sst_items_equal", equal),
    ];
    let mut line = String::from("read");
    for (key, value) in fields {
        line.push_str(&format!("\t{key}={value}"));
    }
    Ok(line)
}

/// The line `--items` prints: the shared strings table's items, walked one
/// at a time (`specs/0027-read-back.md`, decision 6).
fn items_line(input: &Path) -> Result<String, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let mut items = 0u64;
    for item in package.shared_string_items()? {
        item?;
        items += 1;
    }
    Ok(format!("items\titems={items}"))
}

/// The counts `--cells` prints, one field per key.
///
/// Every field is `u64` and not `usize`: the corpus's 2094601 cells fit
/// either, but `usize` is 32 bits on the target that matters and every count
/// in this tool is already `u64`.
#[derive(Default)]
struct CellCounts {
    sheets: u64,
    cells: u64,
    number: u64,
    shared: u64,
    inline: u64,
    formula_string: u64,
    boolean: u64,
    error: u64,
    date: u64,
    values: u64,
    empty: u64,
    formulas: u64,
    shared_master: u64,
    shared_follower: u64,
    array: u64,
    data_table: u64,
    numbers_parsed: u64,
    booleans_parsed: u64,
    resolved: u64,
    unresolved: u64,
    max_row: u64,
    max_col: u64,
    styled: u64,
}

impl CellCounts {
    /// The one line the tool prints: `cells` then tab-separated pairs, in the
    /// order `specs/0010-reading-cells.md`, "Harness runs", lists the keys.
    fn line(&self) -> String {
        let pairs = [
            ("sheets", self.sheets),
            ("cells", self.cells),
            ("number", self.number),
            ("shared", self.shared),
            ("inline", self.inline),
            ("formula_string", self.formula_string),
            ("boolean", self.boolean),
            ("error", self.error),
            ("date", self.date),
            ("values", self.values),
            ("empty", self.empty),
            ("formulas", self.formulas),
            ("shared_master", self.shared_master),
            ("shared_follower", self.shared_follower),
            ("array", self.array),
            ("data_table", self.data_table),
            ("numbers_parsed", self.numbers_parsed),
            ("booleans_parsed", self.booleans_parsed),
            ("resolved", self.resolved),
            ("unresolved", self.unresolved),
            ("max_row", self.max_row),
            ("max_col", self.max_col),
            ("styled", self.styled),
        ];
        let mut line = String::from("cells");
        for (key, value) in pairs {
            line.push('\t');
            line.push_str(key);
            line.push('=');
            line.push_str(&value.to_string());
        }
        line
    }
}

/// Streams every sheet of the workbook in `<sheets>` order and counts it.
///
/// COUNTS ONLY. A cell's value IS content, so no value, no formula, no item
/// of the shared strings table and no length of one is printed: the line
/// names the keys the spec lists and nothing else.
///
/// The shared strings table is read once, before any sheet, and held while
/// the cells stream through a mutable borrow of the same package: that is
/// what `Package::shared_strings` handing its table over OWNED is for.
///
/// A failed `resolve` is COUNTED and does not stop the run, so `unresolved`
/// is the whole count over the workbook rather than the first one found. An
/// error from any other parse is this tool's exit 2 with the message, as
/// `--survey` and `--shared-strings` are.
fn cells_line(input: &Path) -> Result<String, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let table = package.shared_strings()?;
    // The part names are collected first: `cells` borrows the package for as
    // long as its reader streams, and the workbook is lent by the same
    // package.
    let parts: Vec<Vec<u8>> = package
        .workbook()?
        .sheets
        .iter()
        .map(|sheet| sheet.part.clone())
        .collect();
    let mut counted = CellCounts {
        sheets: parts.len() as u64,
        ..CellCounts::default()
    };
    for part in &parts {
        for cell in package.cells(part)? {
            let cell = cell?;
            counted.cells += 1;
            match cell.kind {
                CellKind::Number => counted.number += 1,
                CellKind::SharedString => counted.shared += 1,
                CellKind::InlineString => counted.inline += 1,
                CellKind::FormulaString => counted.formula_string += 1,
                CellKind::Boolean => counted.boolean += 1,
                CellKind::Error => counted.error += 1,
                CellKind::Date => counted.date += 1,
            }
            if cell.value.is_some() {
                counted.values += 1;
            }
            // `empty` is neither value nor formula, which is both the `<c/>`
            // a producer writes for a styled blank and a `<c>` holding only
            // something this layer does not read.
            if cell.value.is_none() && cell.formula.is_none() {
                counted.empty += 1;
            }
            if let Some(formula) = &cell.formula {
                counted.formulas += 1;
                match formula.kind {
                    FormulaKind::Normal => {}
                    FormulaKind::Shared { master: true, .. } => counted.shared_master += 1,
                    FormulaKind::Shared { master: false, .. } => counted.shared_follower += 1,
                    FormulaKind::Array => counted.array += 1,
                    FormulaKind::DataTable => counted.data_table += 1,
                }
            }
            if cell.number().is_some() {
                counted.numbers_parsed += 1;
            }
            if cell.boolean().is_some() {
                counted.booleans_parsed += 1;
            }
            if cell.kind == CellKind::SharedString {
                match table.resolve(&cell) {
                    Ok(_) => counted.resolved += 1,
                    Err(_) => counted.unresolved += 1,
                }
            }
            counted.max_row = counted.max_row.max(u64::from(cell.reference.row));
            counted.max_col = counted.max_col.max(u64::from(cell.reference.col));
            if cell.style.is_some() {
                counted.styled += 1;
            }
        }
    }
    Ok(counted.line())
}

/// Reads the workbook and its styles part and counts them.
///
/// `specs/0012-styles.md`, "Harness runs": `xfs` is the number of cell
/// formats, `num_fmts` of number formats, `custom` of those with an id of 164
/// or more, `dated` of the cell formats whose number format is a date by the
/// test below, and `date1904` is 1 when the workbook counts from 1904. COUNTS
/// ONLY: a format code is content, so none is printed.
fn styles_line(input: &Path) -> Result<String, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let date1904 = u64::from(package.workbook()?.date1904);
    let styles = package.styles()?;
    let xfs = styles.cell_xfs.len() as u64;
    let num_fmts = styles.num_fmts.len() as u64;
    let custom = styles
        .num_fmts
        .iter()
        .filter(|format| format.id >= 164)
        .count() as u64;
    let mut dated = 0u64;
    for xf in 0..styles.cell_xfs.len() {
        // In range by construction: `xf` counts the cell formats read.
        let format = u32::try_from(xf)
            .ok()
            .and_then(|xf| styles.number_format(xf));
        if format.as_ref().is_some_and(is_date_format) {
            dated += 1;
        }
    }
    Ok(format!(
        "styles\txfs={xfs}\tnum_fmts={num_fmts}\tcustom={custom}\tdated={dated}\tdate1904={date1904}"
    ))
}

/// The `--calc` line: whether the workbook names a calculation chain, and
/// whether its `calcPr` asks for a full calculation on load.
fn calc_line(input: &Path) -> Result<String, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let chain = u64::from(workbook.calc_chain.is_some());
    let full_calc = u64::from(workbook.full_calc_on_load);
    Ok(format!("calc\tchain={chain}\tfull_calc={full_calc}"))
}

/// Whether a number format shows a date or a time, by the survey's test.
///
/// A built-in id among 14 to 22, 27 to 36, 45 to 47 and 50 to 58, the date,
/// time and locale-defined date ids of ECMA-376 Part 1, 18.8.30; or a code
/// that holds one of the letters `d m h y s`, in either case, once every
/// quoted run, every bracketed run and every backslash escape is removed,
/// over the whole code. This is the survey's test, as `specs/0012-styles.md`,
/// "Facts planning checked", states it (P1 ruling 1), and no library's: it
/// lives in this tool because the layer does not decide which numbers are
/// dates.
fn is_date_format(format: &NumberFormat) -> bool {
    match format {
        NumberFormat::BuiltIn(id) => matches!(id, 14..=22 | 27..=36 | 45..=47 | 50..=58),
        NumberFormat::Code(code) => is_date_code(code),
    }
}

/// The code half of [`is_date_format`].
///
/// A quoted run is a `"` and the next `"`, and a bracketed run a `[` and the
/// next `]`, with what lies between; a `"` or a `[` with no closing partner
/// starts no run and is an ordinary character, as the regular expression
/// `"[^"]*"|\[[^\]]*\]|\\.` the runs are modelled on reads it. A backslash
/// escape is the backslash and the character after it. What is left is
/// searched for the letters.
fn is_date_code(code: &str) -> bool {
    let chars: Vec<char> = code.chars().collect();
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        let closing = match c {
            '"' => Some('"'),
            '[' => Some(']'),
            _ => None,
        };
        if let Some(closing) = closing
            && let Some(offset) = chars[at + 1..].iter().position(|inner| *inner == closing)
        {
            at += offset + 2;
            continue;
        }
        if c == '\\' && at + 1 < chars.len() {
            at += 2;
            continue;
        }
        if matches!(c, 'd' | 'm' | 'h' | 'y' | 's' | 'D' | 'M' | 'H' | 'Y' | 'S') {
            return true;
        }
        at += 1;
    }
    false
}

/// Walks the package layer's whole map and counts it.
fn survey(input: &Path) -> Result<Survey, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let names: Vec<Vec<u8>> = package
        .archive()
        .entries()
        .iter()
        .map(|entry| entry.name().to_vec())
        .collect();
    let mut counted = Survey {
        entries: names.len() as u64,
        ..Survey::default()
    };

    {
        let types = package.content_types()?;
        counted.defaults = types.defaults().len() as u64;
        counted.overrides = types.overrides().len() as u64;
        // Every entry but the content types part itself has a type or does
        // not: that is what the part is for.
        for name in &names {
            if name.as_slice() == CONTENT_TYPES_PART {
                continue;
            }
            if types.of(name).is_some() {
                counted.typed += 1;
            } else {
                counted.untyped += 1;
            }
        }
        // An Override names a part by an absolute name. specs/0009 fixes
        // that every one of the corpus's resolves exactly, and P2's survey
        // measured only the case-folded near miss, so an Override naming no
        // entry at all was counted nowhere (P2 verification, check 2's
        // observation; ruling 11).
        for over in types.overrides() {
            let part = over.part_name.strip_prefix(b"/").unwrap_or(&over.part_name);
            if !named(&names, part) {
                counted.overrides_unresolved += 1;
                if named_case_only(&names, part) {
                    counted.case_only += 1;
                }
            }
        }
    }

    // Every entry named as a rels part, read for the source its name derives.
    let sources: Vec<Vec<u8>> = names
        .iter()
        .filter_map(|name| rels_part_source(name))
        .collect();
    counted.rels_parts = sources.len() as u64;
    for source in &sources {
        let rels = package.relationships(source)?;
        for rel in rels.all() {
            counted.relationships += 1;
            if rel.external {
                counted.external += 1;
                continue;
            }
            let target = rels.target_part(rel)?;
            if named(&names, &target) {
                counted.resolved += 1;
            } else {
                counted.unresolved += 1;
                if named_case_only(&names, &target) {
                    counted.case_only += 1;
                }
            }
        }
    }

    let workbook = package.workbook()?;
    counted.sheets = workbook.sheets.len() as u64;
    counted.sheets_resolved = workbook
        .sheets
        .iter()
        .filter(|sheet| named(&names, &sheet.part))
        .count() as u64;
    counted.shared_strings = u64::from(workbook.shared_strings.is_some());
    counted.styles = u64::from(workbook.styles.is_some());
    counted.calc_chain = u64::from(workbook.calc_chain.is_some());
    counted.theme = u64::from(workbook.theme.is_some());
    Ok(counted)
}

/// Whether an entry name is one the null rewrite touches: `.xml` or `.rels`,
/// ASCII case-insensitively (`specs/0009-package-layer.md`, "Harness runs").
///
/// A directory entry's name ends in `/`, so none matches.
fn is_xml_name(name: &[u8]) -> bool {
    ends_with_ascii_ci(name, b".xml") || ends_with_ascii_ci(name, b".rels")
}

fn ends_with_ascii_ci(name: &[u8], suffix: &[u8]) -> bool {
    name.len() >= suffix.len() && name[name.len() - suffix.len()..].eq_ignore_ascii_case(suffix)
}

/// Passes one entry's bytes through the token stream and writes them out.
///
/// This is the null rewrite: the mark when present, then every token's raw
/// bytes in order, which is the input byte for byte. It streams, so a 63.7 MB
/// part never lands in memory.
fn rewrite_part<R: io::Read + io::Seek>(
    archive: &mut Archive<R>,
    index: usize,
    file: &mut File,
) -> Result<(), Error> {
    let mut out = BufWriter::new(file);
    let mut stream = Stream::new(archive.read(index)?, index)?;
    if stream.bom() {
        out.write_all(&BYTE_ORDER_MARK)
            .map_err(|e| Error::Rewrite(e.kind()))?;
    }
    while let Some(token) = stream.next()? {
        out.write_all(token.raw)
            .map_err(|e| Error::Rewrite(e.kind()))?;
    }
    out.flush().map_err(|e| Error::Rewrite(e.kind()))?;
    Ok(())
}

/// Creates one of this run's temporary files, and the guard that removes it.
fn temporary(output: &Path, kind: &str) -> Result<(File, Removed), Error> {
    let path = temporary_path(output, kind)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| Error::Create(e.kind()))?;
    Ok((file, Removed(path)))
}

/// Writes the archive back, and reports a buffering failure rather than losing
/// it in a drop.
fn write<R: io::Read + io::Seek>(
    archive: &mut Archive<R>,
    file: File,
    edits: Edits,
) -> Result<(), Error> {
    let mut out = BufWriter::new(file);
    archive.write_to(&mut out, edits)?;
    out.into_inner()
        .map_err(|e| Error::Finish(e.error().kind()))?
        .sync_all()
        .map_err(|e| Error::Finish(e.kind()))?;
    Ok(())
}

/// A temporary file removed when it goes out of scope, whatever stopped the
/// run.
struct Removed(PathBuf);

impl Drop for Removed {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

/// A name beside the target, hidden and unique to this process, so a
/// concurrent run cannot collide and a crash leaves nothing the harness
/// would mistake for an output.
///
/// With an empty `kind` this is the output's own temporary,
/// `.<name>.emend-copy-<pid>.tmp`, exactly the form P1 made and
/// `specs/0008-zip-layer.md` names; `-entry` marks the extracted entry's file,
/// a different file that the P2 rulings keep distinct from it. Both carry the
/// leading dot and the `.tmp` suffix the runner's leftover glob matches
/// (`specs/0006-passthrough.md`, the P1 ruling of 2026-09-16).
fn temporary_path(output: &Path, kind: &str) -> Result<PathBuf, Error> {
    let name = output.file_name().ok_or(Error::NoFileName)?;
    let mut temporary = OsString::from(".");
    temporary.push(name);
    temporary.push(format!(".emend-copy{kind}-{}.tmp", process::id()));
    Ok(output.with_file_name(temporary))
}
