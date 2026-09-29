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
use emend::cells::{Area, CellRef, FormulaKind, Kind, Value};
use emend::dates::{DateTime, Epoch};
use emend::package::{
    CONTENT_TYPES_PART, CORE_PROPERTIES, Cfvo, CfvoKind, ErrorStyle, HYPERLINK, IconSetKind,
    LinkTarget, MAIN, Operator, PIVOT_TABLE, Package, Properties, Property,
    RELATIONSHIPS_ATTRIBUTE, Rule, SheetKind, SheetProtection, TABLE, Validation, ValidationKind,
    legacy_password_hash, rels_part_source,
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
| --link-sample | --props-sample | --protect-sample | --validate-sample | --cf-sample | --scale-sample]";

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

    // Both reading operations run first, so a package the layer cannot read
    // stops the run before any output file exists. Each yields the one line
    // it prints, so the reporting below is the same code for either.
    let reported = match operation {
        Operation::Survey => Some(survey(input)?.line()),
        Operation::SharedStrings => Some(shared_strings_line(input)?),
        Operation::Cells => Some(cells_line(input)?),
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
        Operation::None
        | Operation::Survey
        | Operation::SharedStrings
        | Operation::Cells
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
        let candidate = matches!(cell.kind, Kind::Number | Kind::SharedString)
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
        if at == text && cell.kind == Kind::InlineString && cell.value.as_deref() == Some("emend") {
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
        let number = cell.kind == Kind::Number && cell.value.is_some() && cell.formula.is_none();
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
                        cell.kind == Kind::InlineString && cell.value.as_deref() == Some(text)
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
                Kind::Number => counted.number += 1,
                Kind::SharedString => counted.shared += 1,
                Kind::InlineString => counted.inline += 1,
                Kind::FormulaString => counted.formula_string += 1,
                Kind::Boolean => counted.boolean += 1,
                Kind::Error => counted.error += 1,
                Kind::Date => counted.date += 1,
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
            if cell.kind == Kind::SharedString {
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
