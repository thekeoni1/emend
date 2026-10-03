//! The Open Packaging Conventions layer: content types, relationships, sheets.
//!
//! `specs/0009-package-layer.md`, "Module `package`". A [`Package`] wraps an
//! [`Archive`] and parses nothing until asked: [`Package::content_types`]
//! reads `[Content_Types].xml`, [`Package::relationships`] reads one part's
//! `.rels`, and [`Package::workbook`] follows the package's `officeDocument`
//! relationship to the workbook part and maps its sheets to theirs. Each
//! answer is parsed once and kept. The part edits and [`Package::write_to`]
//! came with part 2's P3; [`Package::cells`] reads a worksheet
//! (`specs/0010-reading-cells.md`), and [`Package::set_cell`] and
//! [`Package::clear_cell`] record cell edits that `write_to` applies
//! (`specs/0011-editing-cells.md`). [`Package::styles`] reads the workbook's
//! styles part (`specs/0012-styles.md`).
//!
//! Every name is bytes, compared byte for byte, because a zip entry's name is
//! bytes and the archive layer indexes it that way. No percent-decoding and
//! no case folding, except that a content type `Default`'s extension is
//! compared ASCII case-insensitively, as the convention says. The corpus
//! needs neither of the two the layer refuses: of its 644 internal
//! relationship targets every one resolves to an entry by exact bytes, and
//! none carries `%` or `&` (Design, "Limits"; the survey measures both
//! rules rather than assuming them, through its `unresolved` and `case_only`
//! keys).
//!
//! # The namespaces
//!
//! The two package namespaces have one form each. Everything from the
//! spreadsheet vocabulary has two, the transitional and the strict, and a
//! lookup tries them in that order: [`MAIN`], [`RELATIONSHIPS_ATTRIBUTE`] and
//! the eight relationship types below are pairs for that reason.
//!
//! # Errors
//!
//! [`Error::Xml`] carries the entry's index and comes from [`Stream`] or from
//! an attribute the layer interprets (a namespace URI, a sheet name), never
//! from a byte of the workbook's content.
//!
//! [`Error::Package`] carries one of these fixed strings and nothing else:
//!
//! - "no \[Content_Types\].xml part" (`open_reads_no_xml`,
//!   `content_types_part_absent`).
//! - "a content types part whose root is not Types" (`root_wrong_in_types`).
//! - "a Default, Override or Relationship element lacks a required attribute"
//!   (`default_lacks_an_attribute`, `override_lacks_an_attribute`,
//!   `relationship_lacks_an_attribute`).
//! - "a relationships part whose root is not Relationships"
//!   (`root_wrong_in_rels`).
//! - "two relationships with the same id" (`two_relationships_one_id`).
//! - "a relationship target outside the package" (`target_above_root`).
//! - "an external relationship has no part" (`external_target`).
//! - "no officeDocument relationship" (`no_office_document`).
//! - "more than one officeDocument relationship" (`two_office_documents`).
//! - "the officeDocument relationship names no entry"
//!   (`office_document_target_absent`).
//! - "a workbook part whose root is not workbook" (`root_wrong_in_workbook`).
//! - "a sheet element lacks name, sheetId or r:id" (`sheet_lacks_rel_id`).
//! - "a sheetId that is not a number" (`sheet_id_not_a_number`).
//! - "a sheet's relationship is missing" (`sheet_relationship_missing`).
//! - "an element or attribute uses a prefix its document does not bind",
//!   raised by [`Scope`] and reached from here through an `r:id` whose prefix
//!   nothing binds (`rel_id_prefix_unbound`), from the styles reader
//!   through a cell format's child whose prefix nothing binds
//!   (`styles::tests::an_xf_child_with_an_unbound_prefix_is_refused`), and,
//!   with the Cargo feature `metadata`, from the workbook read through a
//!   `definedName` attribute whose prefix nothing binds
//!   (`names::a_defined_name_attribute_with_an_unbound_prefix_is_refused_at_the_read`).
//! - "no such relationship" (`removing_an_absent_relationship`).
//! - "no such override" (`removing_an_absent_override`).
//! - "an override for that part already exists"
//!   (`adding_an_override_that_exists`).
//! - "an edit to a removed part" (`editing_a_removed_part`,
//!   `editing_a_removed_rels_part`, `editing_removed_content_types`, and
//!   with the Cargo feature `metadata`
//!   `properties::a_core_part_removed_or_replaced_refuses_a_property_edit`).
//! - "no such part" (`removing_an_absent_part`, `replacing_an_absent_part`,
//!   `a_relationship_on_a_part_that_is_not_there`,
//!   `a_relationship_removal_on_a_part_that_is_not_there`,
//!   `cells::tests::a_relationship_naming_no_entry_is_refused`,
//!   `cells::tests::cells_on_a_part_that_is_not_there_is_refused`,
//!   `styles::tests::a_styles_relationship_naming_no_entry_is_refused`,
//!   `cells_on_a_sheet_added_and_not_yet_written_is_refused`, and with the
//!   Cargo feature `metadata`
//!   `properties::a_property_edit_on_a_missing_core_part_is_refused_at_the_write`).
//! - "no Default covers rels" (`no_default_for_rels`).
//!
//! The cell edits add eight (`specs/0011-editing-cells.md`, "Errors"); the
//! first two are raised by [`Value`] in `cells` and reported by
//! [`Package::write_to`]:
//!
//! - "a number that is not finite cannot be written"
//!   (`a_number_that_is_not_finite_is_refused`).
//! - "a text holds a character XML cannot carry"
//!   (`a_text_with_a_character_xml_forbids_is_refused`, and with the Cargo
//!   feature `rules` for a validation's or a rule's formula or message and a
//!   value point's formula,
//!   `validation::the_call_refuses_each_rule_and_records_nothing`,
//!   `conditional::the_call_refuses_each_rule_and_records_nothing`,
//!   `visual::the_value_rule_and_a_visual_rule_with_a_style_are_refused_at_the_call`).
//! - "a cell carries an attribute this layer does not model"
//!   (`a_cell_with_an_attribute_this_layer_does_not_model_is_refused`).
//! - "a cell edit to a part this write replaces"
//!   (`a_cell_edit_to_a_missing_removed_or_replaced_part_is_refused`).
//! - "a worksheet without explicit references cannot be edited"
//!   (`a_worksheet_without_explicit_references_is_refused`).
//! - "a worksheet whose rows or cells are out of order cannot be edited"
//!   (`a_worksheet_out_of_order_is_refused`,
//!   `a_refused_cell_edit_leaves_no_temporary_and_no_output`).
//! - "a cell in a shared, array or data table formula cannot be edited"
//!   (`a_cell_in_a_shared_or_array_formula_is_refused`,
//!   `an_array_whose_extent_is_unknown_is_refused`,
//!   `a_data_table_is_refused_as_an_array_is`).
//! - "a worksheet without sheetData cannot be edited"
//!   (`a_set_on_a_worksheet_without_sheet_data_is_refused`).
//!
//! The write also refuses a cell value, clear or style edit whose reference
//! lies off the grid with "a row or column index or count is out of range",
//! the message the formats below raise at their calls (`specs/0023-scale.md`,
//! P1 verification ruling 5; `a_cell_edit_off_the_grid_is_refused_at_the_write`).
//!
//! The styles part's reading adds five (`specs/0012-styles.md`, "Errors"),
//! the first raised here by [`Package::styles`] and the rest by `styles`:
//!
//! - "the workbook has no styles part"
//!   (`styles::tests::a_workbook_with_no_styles_relationship_is_refused`).
//! - "the styles part's root is not styleSheet"
//!   (`styles::tests::a_root_of_the_wrong_name_is_refused`,
//!   `a_root_in_another_namespace_is_refused`,
//!   `a_part_with_no_root_at_all_is_refused`).
//! - "a numFmt lacks numFmtId or formatCode"
//!   (`styles::tests::a_num_fmt_without_an_id_is_refused`,
//!   `a_num_fmt_without_a_code_is_refused`).
//! - "a style attribute is not a number"
//!   (`styles::tests::a_num_fmt_id_that_is_not_a_number_is_refused`,
//!   `an_xf_id_that_is_not_a_number_is_refused`).
//! - "the styles part has more than one numFmts, fonts, fills, borders or cellXfs"
//!   (`styles::tests::a_second_num_fmts_is_refused`,
//!   `a_second_cell_xfs_is_refused`,
//!   `a_second_fonts_fills_or_borders_is_refused`), widened from its two
//!   containers by `specs/0017-fuller-styles.md`, "Module `styles`: the
//!   counts", and P1 ruling 2.
//!
//! The additions and the style edits add five more (the same "Errors"),
//! raised here by [`Package::add_cell_style`] and [`Package::write_to`]; the
//! last two of the reading's also come from the write, "the workbook has no
//! styles part" (`a_some_edit_needs_a_styles_part_and_a_none_edit_does_not`)
//! and "a style attribute is not a number"
//! (`a_count_that_is_not_a_number_is_refused_at_write`):
//!
//! - "a style index is out of range"
//!   (`an_addition_is_refused_on_a_bad_base_or_format`,
//!   `an_index_at_the_count_plus_additions_is_refused_and_below_it_accepted`).
//! - "a built-in number format id is 164 or more"
//!   (`an_addition_is_refused_on_a_bad_base_or_format`).
//! - "a number format code is empty or holds a character XML cannot carry"
//!   (`an_addition_is_refused_on_a_bad_base_or_format`).
//! - "a style addition to a part this write replaces"
//!   (`a_style_edit_or_addition_on_a_replaced_or_removed_part_is_refused`).
//! - "a styles part without cellXfs cannot take a style"
//!   (`a_part_without_cell_xfs_is_refused_at_write_with_no_output`).
//!
//! The fuller cell styles add three (`specs/0017-fuller-styles.md`,
//! "Errors"), raised by [`Package::add_style`], the first also by
//! [`Package::write_to`] and the other two by `styles`:
//!
//! - "a style addition needs a collection the styles part lacks"
//!   (`a_font_fill_or_border_on_a_part_without_its_collection_is_refused_and_records_nothing`,
//!   `a_collection_id_past_u32_is_refused`, and at the write, for a part that
//!   lost the collection after it was read, which no path of this crate can
//!   cause, `a_collection_gone_at_the_write_is_refused_with_no_output`; P1
//!   ruling 1).
//! - "a font name is empty or holds a character XML cannot carry"
//!   (`styles::elements::tests::a_font_name_empty_or_holding_a_forbidden_character_is_refused`,
//!   `a_refused_font_name_or_field_records_nothing`).
//! - "a style field is out of range"
//!   (`styles::elements::tests::a_field_out_of_range_is_refused`,
//!   `a_refused_font_name_or_field_records_nothing`,
//!   `a_rotation_out_of_range_refuses_the_addition_and_records_nothing`).
//!
//! Formulas and the workbook's `calcPr` add five (`specs/0013-formulas.md`,
//! "Errors"); the first three are raised by [`Value`] in `cells` and reported
//! by [`Package::write_to`], the last two by `write_to` itself:
//!
//! - "a formula text is empty" (`an_empty_formula_text_is_refused`).
//! - "a formula text starts with an equals sign"
//!   (`a_formula_text_starting_with_an_equals_sign_is_refused`, and with the
//!   Cargo feature `rules` for a validation's, a rule's and a value point's
//!   formula, `validation::the_call_refuses_each_rule_and_records_nothing`,
//!   `conditional::the_call_refuses_each_rule_and_records_nothing`,
//!   `visual::the_value_rule_and_a_visual_rule_with_a_style_are_refused_at_the_call`).
//! - "a formula text holds a character XML cannot carry"
//!   (`a_formula_text_with_a_character_xml_forbids_is_refused`).
//! - "a workbook has two calcPr elements"
//!   (`a_workbook_with_two_calc_prs_is_refused_at_write_with_no_output`,
//!   `the_calc_pr_rewrite_refuses_a_second_calc_pr_itself`).
//! - "the workbook part cannot be edited in a package that replaces or removes it"
//!   (`a_workbook_part_replaced_or_removed_refuses_the_flag`).
//!
//! The sheet list adds these (`specs/0016-new-workbook-and-sheets.md`,
//! "Errors"), raised by [`Package::add_sheet`], [`Package::rename_sheet`],
//! [`Package::move_sheet`], [`Package::remove_sheet`] and
//! [`Package::write_to`]; a name holding a
//! character XML cannot carry reuses "a text holds a character XML cannot
//! carry" above (P3 ruling 3,
//! `a_sheet_name_breaking_a_rule_is_refused_and_records_nothing`):
//!
//! - "a sheet name is empty"
//!   (`a_sheet_name_breaking_a_rule_is_refused_and_records_nothing`).
//! - "a sheet name is longer than 31 characters"
//!   (`a_sheet_name_breaking_a_rule_is_refused_and_records_nothing`,
//!   `a_name_of_31_multi_byte_characters_is_accepted`).
//! - "a sheet name holds a character Excel refuses"
//!   (`a_sheet_name_breaking_a_rule_is_refused_and_records_nothing`).
//! - "a sheet name starts or ends with an apostrophe"
//!   (`a_sheet_name_breaking_a_rule_is_refused_and_records_nothing`).
//! - "a sheet name is the reserved name History"
//!   (`a_sheet_name_breaking_a_rule_is_refused_and_records_nothing`).
//! - "a sheet name is already in use"
//!   (`a_sheet_name_breaking_a_rule_is_refused_and_records_nothing`).
//! - "no sheet of that name" (`a_sheet_that_is_not_there_is_refused`).
//! - "the last sheet cannot be removed" (`the_last_sheet_cannot_be_removed`).
//! - "the last visible sheet cannot be removed"
//!   (`the_last_visible_sheet_cannot_be_removed`).
//! - "a sheet position is past the end"
//!   (`moves_to_the_front_to_the_end_to_its_own_place_and_past_the_end`).
//! - "the workbook root binds no prefix for the relationships namespace"
//!   (`a_workbook_root_binding_no_relationships_prefix_refuses_an_add`).
//! - "a sheets element holds something other than sheet elements"
//!   (`a_sheets_element_holding_anything_but_sheets_is_refused`).
//! - "a sheet cannot be added to a prefixed workbook"
//!   (`a_prefixed_workbook_takes_a_rename_and_a_move_and_refuses_an_add`).
//! - "a sheet edit no longer applies at the write", the write's defence
//!   against an edit that no longer holds, which no public call reaches, and
//!   the refusal of what cannot be written: a `localSheetId` that is not a
//!   decimal `u32` (P3 ruling 5), an `activeTab` or `firstSheet` that is not
//!   one (P3 ruling 10), a `sheetId` past `u32::MAX` and a sheet added to a
//!   workbook part with no `<sheets>` (P3 ruling 6)
//!   (`a_local_sheet_id_that_is_not_a_number_refuses_a_move_at_the_write`,
//!   `a_book_view_index_that_is_not_a_number_refuses_a_move_at_the_write`,
//!   `a_sheet_id_past_u32_is_refused_when_recorded`,
//!   `a_sheet_added_to_a_workbook_with_no_sheets_element_is_refused_at_the_write`).
//!
//! The sheet edits also meet the formulas' "the workbook part cannot be
//! edited in a package that replaces or removes it"
//! (`a_sheet_edit_on_a_replaced_workbook_part_is_refused_at_the_write`).
//!
//! [`Package::new`] adds none (`specs/0016-new-workbook-and-sheets.md`,
//! "`Package::new`"): it can fail only with what [`Package::write_to`] and
//! [`Package::open`] refuse. The template's fixed bytes themselves are never
//! refused; the temporary a written part goes through can fail on I/O, as
//! in any write. No test reaches an error from it
//! (`new_is_the_empty_archive_with_the_template_added` shows the bytes it
//! opens are the ones its recipe writes).
//!
//! Rows and columns add these (`specs/0018-rows-and-columns.md`, "Errors",
//! and P1 rulings 3, 4 and 6); the first four are raised by the calls that
//! record an edit, the rest by [`Package::write_to`]. The formats, panes and
//! merges also meet "a worksheet without sheetData cannot be edited", "a cell
//! in a shared, array or data table formula cannot be edited", "a cell
//! carries an attribute this layer does not model", "an edit to a removed
//! part" and "a cell edit to a part this write replaces" (P1 rulings 7 and 8;
//! `rowcol::a_row_format_on_a_part_without_sheet_data_is_refused`,
//! `a_covered_cell_in_an_array_or_with_an_unmodelled_attribute_is_refused`,
//! `an_edit_on_a_removed_or_replaced_part_is_refused`):
//!
//! - "a row or column index or count is out of range"
//!   (`rowcol::indices_sizes_and_a_freeze_at_a1_are_refused_at_the_call`,
//!   `a_merge_of_one_cell_or_off_the_grid_is_refused_at_the_call`).
//! - "a column width or a row height is out of range"
//!   (`rowcol::indices_sizes_and_a_freeze_at_a1_are_refused_at_the_call`).
//! - "a freeze at A1 freezes nothing"
//!   (`rowcol::indices_sizes_and_a_freeze_at_a1_are_refused_at_the_call`).
//! - "a merge needs more than one cell"
//!   (`rowcol::a_merge_of_one_cell_or_off_the_grid_is_refused_at_the_call`).
//! - "a merge overlaps a merge the worksheet has, or a mergeCell ref does not parse"
//!   (`rowcol::a_merge_overlapping_a_merge_is_refused`), for a merge the part
//!   has, one recorded earlier in the same write, and a `<mergeCell>` whose
//!   `ref` is not an area (P1 ruling 4).
//! - "no merge of that range"
//!   (`rowcol::an_unmerge_removes_its_merge_refuses_an_absent_one_and_drops_the_last`),
//!   the file's merges alone being matched (P1 ruling 5).
//! - "a cell edit inside a merge this write records"
//!   (`rowcol::a_cell_edit_inside_a_pending_merge_is_refused_and_the_top_left_is_not`).
//! - "a worksheet whose cols are out of order or overlap cannot be edited"
//!   (`rowcol::cols_out_of_order_overlapping_malformed_or_doubled_are_refused`),
//!   also for a `<col>` lacking `min` or `max`, with either not a decimal
//!   `u32`, a `min` of 0 or a `min` above `max` (P1 ruling 3), and for
//!   those malformed forms under a column insertion or deletion, which checks
//!   no order or overlap (P3 ruling 1,
//!   `shift::cols_moved_grown_cut_clipped_dropped_and_the_container_dropped`).
//! - "a worksheet has two cols, sheetViews or mergeCells elements, or a sheetViews without a sheetView"
//!   (`rowcol::cols_out_of_order_overlapping_malformed_or_doubled_are_refused`,
//!   `sheet_views_without_a_sheet_view_or_doubled_is_refused`,
//!   `an_unmerge_removes_its_merge_refuses_an_absent_one_and_drops_the_last`,
//!   and for an element the write created and then met in an input out of
//!   schema order, `an_element_created_and_then_met_in_the_input_is_refused`,
//!   P1 ruling 2).
//!
//! Inserting and deleting rows and columns adds these
//! (`specs/0018-rows-and-columns.md`, "Errors", P2 rulings 1, 2 and 5, and
//! P3 rulings 1 and 2), the Cargo feature `rowcol`'s
//! messages (decision 7 A: without the feature no call records a shift and
//! none of them is raised), raised by [`Package::write_to`];
//! the calls refuse a count of 0 or rows or columns past the grid with "a row
//! or column index or count is out of range", and the write meets the order
//! and explicit-reference refusals of the cell edits
//! (`shift::a_count_of_zero_or_rows_past_the_grid_are_refused_at_the_call`,
//! `a_count_of_zero_or_columns_past_the_grid_are_refused_at_the_call`,
//! `the_order_rule_holds_on_the_input`):
//!
//! - "a worksheet with a row or column insertion or deletion takes no other edit in the same write"
//!   (`shift::a_second_shift_or_any_other_edit_for_the_part_is_refused`,
//!   `a_second_insertion_for_the_part_is_refused`,
//!   `a_row_and_a_column_shift_on_one_part_are_refused`).
//! - "a row or column would be moved off the worksheet"
//!   (`shift::a_row_moved_off_the_grid_is_refused`,
//!   `a_ref_pushed_past_the_grid_is_a_grow_and_one_moved_past_it_off_the_grid`,
//!   `a_col_to_the_last_column_is_clipped_and_a_cell_there_is_refused`).
//! - "an insertion or deletion cuts a shared, array or data table formula"
//!   (`shift::a_shared_formula_before_after_straddling_inside_and_deleted_whole`,
//!   `an_array_formula_likewise_and_one_whose_ref_does_not_parse`,
//!   `an_array_formula_before_the_point_stays_and_a_deletion_inside_it_is_refused`,
//!   `a_ref_pushed_past_the_grid_is_a_grow_and_one_moved_past_it_off_the_grid`,
//!   `formulas_on_the_column_axis_move_are_refused_when_cut_and_go_with_their_cells`).
//! - "an insertion or deletion would move a table or a pivot table"
//!   (`shift::a_table_or_pivot_before_the_point_stays_and_one_at_or_after_it_is_refused`,
//!   `a_table_or_pivot_on_the_column_axis_before_the_point_stays_and_one_at_or_after_it_is_refused`),
//!   also for a table or pivot `ref` that is absent or does not parse and a
//!   part that is not there (P2 ruling 2,
//!   `a_table_ref_absent_a_table_part_missing_or_a_pivot_ref_unparsed_is_refused`).
//! - "a range an insertion or deletion would move does not parse"
//!   (`shift::an_array_formula_likewise_and_one_whose_ref_does_not_parse`,
//!   `a_dimension_moved_grown_cut_and_emptied`,
//!   `merges_moved_grown_cut_dropped_and_the_container_dropped`,
//!   `the_four_lists_move_are_cut_lose_ranges_and_drop_their_elements`,
//!   `list_ranges_cut_an_emptied_filter_dropped_and_each_list_unparsed_refused`),
//!   and for a row's `spans` item under a column insertion or deletion (P3
//!   ruling 2,
//!   `spans_moved_grown_cut_and_dropped_and_one_that_does_not_parse_refused`).
//!
//! Defined names add these (`specs/0019-names-links-properties-protection.md`,
//! "Errors"), the Cargo feature `metadata`'s messages (decision 1 A: without
//! the feature no call records a name edit and none of them is raised); the
//! first is raised by `Package::add_defined_name`, the rest by
//! [`Package::write_to`]. The calls also meet "no sheet of that name" and "a
//! text holds a character XML cannot carry", and the write "the workbook part
//! cannot be edited in a package that replaces or removes it" and "a workbook
//! has two calcPr elements", through the formula-change rule a name edit
//! fires:
//!
//! - "a defined name is not valid"
//!   (`names::the_name_rule_refuses_each_clause_and_records_nothing`).
//! - "a defined name of that name and scope already exists"
//!   (`names::a_duplicate_against_the_file_or_a_pending_edit_is_refused`).
//! - "no defined name of that name and scope"
//!   (`names::a_removal_drops_a_name_by_scope_and_refuses_an_absent_one`).
//! - "a workbook has two definedNames or workbookProtection elements"
//!   (`names::a_second_defined_names_is_refused`).
//!
//! Hyperlinks add these, the feature `metadata`'s too; the first is raised
//! by `Package::set_hyperlink`, the rest by [`Package::write_to`]. The calls
//! also meet "a text holds a character XML cannot carry" and "a row or
//! column index or count is out of range", and the write part 3's one-shift
//! message and what a cell edit to the part meets:
//!
//! - "a hyperlink target or location is empty"
//!   (`links::the_calls_refuse_an_empty_target_a_forbidden_character_or_a_cell_off_the_grid`).
//! - "no hyperlink on that cell"
//!   (`links::a_removal_of_an_absent_link_or_of_one_whose_ref_is_a_range_is_refused`).
//! - "a worksheet has two hyperlinks or sheetProtection elements"
//!   (`links::a_second_hyperlinks_is_refused`,
//!   `protection::a_second_sheet_protection_is_refused`).
//!
//! Document properties and protection add these, the feature `metadata`'s
//! too; the first two are raised by the calls that record an edit, the
//! rest by [`Package::write_to`] and the properties read. The workbook's
//! second `<workbookProtection>` meets "a workbook has two definedNames or
//! workbookProtection elements"
//! (`protection::a_second_workbook_protection_is_refused`), a property value
//! holding a character XML forbids "a text holds a character XML cannot
//! carry", and a core part the write removes or replaces "an edit to a
//! removed part":
//!
//! - "a property value is not valid"
//!   (`properties::a_date_value_must_be_the_one_form_and_every_value_xml_can_carry`).
//! - "a password is not valid"
//!   (`protection::a_password_outside_printable_ascii_or_past_255_characters_is_refused`).
//! - "a core properties part binds no prefix for a namespace the edit needs"
//!   (`properties::a_root_binding_no_dcterms_prefix_refuses_an_append_and_allows_a_replace`,
//!   `properties::xsi_bound_only_as_the_default_namespace_refuses_an_appended_date`).
//! - "a core properties part has two elements of one property"
//!   (`properties::a_second_element_of_one_property_is_refused_at_the_read_and_the_write`).
//!
//! Data validations add these, the Cargo feature `rules`'s messages
//! (`specs/0020-validation-and-conditional-formatting.md`, "Errors", and the
//! P1 rulings; without the feature no call records a validation edit and
//! none of them is raised); the first four are raised by
//! `Package::add_validation` and `Package::remove_validation`, the rest by
//! [`Package::write_to`] and by `Package::validations`, which refuses with
//! the fifth. The calls also meet "a row or column index or count is out of
//! range", "a text holds a character XML cannot carry" and "a formula text
//! starts with an equals sign", and the write part 3's one-shift message and
//! what a cell edit to the part meets
//! (`validation::the_call_refuses_each_rule_and_records_nothing`,
//! `a_validation_edit_on_a_removed_or_replaced_part_is_refused`):
//!
//! - "a validation or a conditional format has no area"
//!   (`validation::the_call_refuses_each_rule_and_records_nothing`).
//! - "a validation or a rule formula is empty or missing"
//!   (`validation::the_call_refuses_each_rule_and_records_nothing`).
//! - "a validation message is too long"
//!   (`validation::the_call_refuses_each_rule_and_records_nothing`).
//! - "a validation field does not apply to its kind"
//!   (`validation::the_call_refuses_each_rule_and_records_nothing`).
//! - "a validation overlaps one the worksheet has, or a dataValidation sqref does not parse"
//!   (`validation::an_overlap_with_a_kept_validation_or_an_earlier_addition_is_refused`,
//!   `two_containers_or_a_kept_sqref_that_does_not_parse_are_refused`,
//!   `a_read_refuses_an_unparsed_sqref_or_an_enumeration_value_it_does_not_name`).
//! - "no validation on those areas"
//!   (`validation::a_removal_drops_its_list_in_order_and_the_last_drops_the_container`).
//! - "a worksheet has two dataValidations elements"
//!   (`validation::two_containers_or_a_kept_sqref_that_does_not_parse_are_refused`).
//!
//! Differential styles add these, the feature `rules`'s too, raised by
//! `Package::add_differential_style` and, for a second `<dxfs>`, by
//! [`Package::write_to`] as well; the call also meets the element writers'
//! refusals and "the workbook has no styles part", and the write "a style
//! addition to a part this write replaces"
//! (`dxf::a_dxf_on_a_replaced_styles_part_is_refused`):
//!
//! - "a differential style holds a field this part does not write"
//!   (`dxf::a_field_this_part_does_not_write_or_no_field_is_refused_and_records_nothing`).
//! - "a styles part has two dxfs elements"
//!   (`dxf::two_dxfs_are_refused_at_the_call_and_a_created_then_met_one_at_the_write`).
//! - "a differential style index is out of range", at the write for a
//!   conditional format's `dxf` at or past the `dxf` children plus the
//!   write's differential styles
//!   (`conditional::a_style_index_at_the_count_plus_additions_is_refused_and_below_it_taken`),
//!   and at the call for an index `u32` cannot hold, which no test reaches
//!   (a part would need more than `u32::MAX` children).
//!
//! Conditional formats add these, the feature `rules`'s too; the first is
//! raised by `Package::add_conditional_format`, the rest by
//! [`Package::write_to`] and by `Package::conditional_formats`, which
//! refuses with the fourth. The calls also meet "a validation or a
//! conditional format has no area", "a validation or a rule formula is
//! empty or missing", "a row or column index or count is out of range", "a
//! formula text starts with an equals sign" and "a text holds a character
//! XML cannot carry"
//! (`conditional::the_call_refuses_each_rule_and_records_nothing`), and the
//! write part 3's one-shift message and what a cell edit to the part meets
//! (`conditional::a_conditional_format_edit_on_a_removed_or_replaced_part_is_refused`):
//!
//! - "a conditional format rule and its differential style do not agree"
//!   (`conditional::the_call_refuses_each_rule_and_records_nothing`, and for
//!   a visual rule given a style
//!   `visual::the_value_rule_and_a_visual_rule_with_a_style_are_refused_at_the_call`).
//! - "a colour scale, data bar or icon set value is not valid", at the call
//!   (`visual::the_value_rule_and_a_visual_rule_with_a_style_are_refused_at_the_call`);
//!   a value point's formula with a leading `=` or a character XML forbids
//!   meets part 4's messages instead, as every formula does (P3 ruling 2, the
//!   same test).
//! - "no conditional format on those areas"
//!   (`conditional::a_removal_drops_its_list_and_an_absent_or_extended_one_is_refused`).
//! - "a conditional format has an extension this part does not remove"
//!   (`conditional::a_removal_drops_its_list_and_an_absent_or_extended_one_is_refused`).
//! - "a conditionalFormatting sqref or a cfRule attribute does not parse"
//!   (`conditional::a_read_refuses_an_attribute_that_does_not_parse`,
//!   `priorities_follow_the_highest_kept_one_and_a_given_one_is_written_as_given`,
//!   `a_removal_drops_its_list_and_an_absent_or_extended_one_is_refused`).
//!
//! Tables add these, the Cargo feature `tables`'s messages
//! (`specs/0021-tables.md`, "Errors", and the P1 rulings; without the
//! feature no call records a table edit and none of them is raised); the
//! first four are raised by `Package::add_table`, the rest by
//! [`Package::write_to`], after `Package::add_table` or
//! `Package::remove_table`, and, the last, by `Package::tables` too. The call
//! also meets "a row or column index or count is out of range" and "a text
//! holds a character XML cannot carry"
//! (`tables::the_call_refuses_each_rule_and_records_nothing`), and the write
//! part 3's one-shift message, "no such part" for a sheet added in the same
//! write, and what a cell edit to the part meets
//! (`tables::a_table_edit_with_a_row_insertion_is_refused`,
//! `a_sheet_added_in_the_same_write_refuses_a_table`,
//! `a_table_edit_on_a_removed_or_replaced_part_is_refused`,
//! `a_removal_on_a_removed_or_replaced_part_is_refused`):
//!
//! - "a table name is not valid"
//!   (`tables::the_call_refuses_each_rule_and_records_nothing`).
//! - "a table column name is not valid"
//!   (`tables::the_call_refuses_each_rule_and_records_nothing`).
//! - "a table style number is out of range"
//!   (`tables::the_call_refuses_each_rule_and_records_nothing`).
//! - "a table area is too small"
//!   (`tables::the_call_refuses_each_rule_and_records_nothing`).
//! - "a table name is already in use"
//!   (`tables::a_name_in_use_is_refused_and_one_of_a_removed_sheets_table_is_taken`).
//! - "a table overlaps a table, a pivot table, a merge or the autofilter"
//!   (`tables::an_overlap_with_a_table_a_pivot_a_merge_the_autofilter_or_an_earlier_addition_is_refused`).
//! - "a table header cell holds a formula"
//!   (`tables::a_header_cell_holding_a_formula_is_refused`).
//! - "no table of that name"
//!   (`tables::a_removal_of_an_absent_name_or_of_one_this_write_adds_is_refused`,
//!   `a_removal_on_a_sheet_added_in_the_same_write_finds_no_table`).
//! - "a worksheet has two tableParts elements"
//!   (`tables::a_container_is_created_before_ext_lst_and_on_a_prefixed_root`).
//! - "a table part or its relationship cannot be read"
//!   (`tables::a_read_refuses_a_part_or_relationship_it_cannot_read`,
//!   `an_overlap_with_a_table_a_pivot_a_merge_the_autofilter_or_an_earlier_addition_is_refused`).
//!
//! Comments add these, the Cargo feature `comments`'s messages
//! (`specs/0022-comments-and-images.md`, "Errors", and the P1 rulings;
//! without the feature no call records a comment edit and none of them is
//! raised); the first two are raised by `Package::add_comment`, the rest by
//! [`Package::write_to`], after `Package::add_comment` or
//! `Package::remove_comment`, and, the last, by `Package::comments` too. The call
//! also meets "a row or column index or count is out of range" and "a text
//! holds a character XML cannot carry"
//! (`comments::the_call_refuses_each_rule_and_records_nothing`), and the
//! write part 3's one-shift message, "no such part" for a sheet added in the
//! same write, and what a cell edit to the part meets, also when the write
//! removes, replaces or adds the sheet's comments part or VML part through
//! the part calls (P1 verification ruling 1 (2))
//! (`comments::a_comment_with_a_row_insertion_is_refused`,
//! `a_comment_on_a_removed_replaced_or_added_part_is_refused`,
//! `a_comment_on_a_sheet_whose_comments_part_this_write_removes_is_refused`
//! and the five tests after it):
//!
//! - "a comment author is not valid"
//!   (`comments::the_call_refuses_each_rule_and_records_nothing`).
//! - "a comment text is not valid"
//!   (`comments::the_call_refuses_each_rule_and_records_nothing`).
//! - "a worksheet has threaded comments"
//!   (`comments::a_sheet_with_threaded_comments_is_refused`).
//! - "a worksheet has two legacyDrawing elements"
//!   (`comments::two_legacy_drawings_or_one_out_of_schema_order_are_refused`).
//! - "no comment on that cell"
//!   (`comments::a_removal_of_an_absent_cell_a_cell_this_write_adds_or_one_named_twice_is_refused`,
//!   `a_removal_then_an_addition_of_one_cell_replaces_and_needs_a_kept_comment`).
//! - "a comments part or its drawing cannot be read"
//!   (`comments::a_read_refuses_what_it_cannot_read`,
//!   `the_write_refuses_parts_it_cannot_read`,
//!   `a_removal_on_a_sheet_whose_comments_part_is_missing_is_refused_at_the_write`).
//!
//! Images add these, the Cargo feature `images`'s messages
//! (`specs/0022-comments-and-images.md`, "Errors", and the P3 rulings;
//! without the feature no call records an image and none of them is
//! raised); the first two are raised by `Package::add_image`, the rest by
//! [`Package::write_to`] after it, and, the last, by `Package::images` too.
//! The call also meets "a row or column index or count is out of range"
//! (`images::the_call_refuses_each_rule_and_records_nothing`), and the write
//! part 3's one-shift message, "no such part" for a sheet added in the same
//! write, and what a cell edit to the part meets, also when the write
//! removes or replaces the sheet's drawing part through the part calls
//! (`images::an_image_with_a_row_insertion_is_refused`,
//! `an_image_on_a_removed_replaced_or_added_part_is_refused`):
//!
//! - "an image is not a PNG or JPEG"
//!   (`images::the_call_refuses_each_rule_and_records_nothing`).
//! - "an image size is out of range"
//!   (`images::the_call_refuses_each_rule_and_records_nothing`).
//! - "a worksheet has two drawing elements"
//!   (`images::two_drawing_elements_or_one_out_of_schema_order_are_refused`).
//! - "a drawing part or its relationship cannot be read"
//!   (`images::a_read_refuses_what_it_cannot_read`,
//!   `the_write_refuses_a_drawing_it_cannot_read`,
//!   `a_drawing_whose_root_is_user_shapes_is_refused`).
//!
//! Rich text adds these, the Cargo feature `richtext`'s messages
//! (`specs/0024-rich-text.md`, "Errors"; without the feature no value
//! carries runs and neither is raised); both are raised in `cells`, the
//! first by a rich value's check and reported by [`Package::write_to`], the
//! second by the read of a cell, through [`Package::cells`], of the shared
//! strings table, through [`Package::shared_strings`], and, with the Cargo
//! feature `comments` too, of a comment, through `Package::comments`. A
//! rich value also meets, at the write, "a text holds a character XML
//! cannot carry", the font refusals of [`Package::add_style`] and the
//! off-grid refusal (`richtext::the_write_refuses_each_rule`,
//! `richtext::a_rich_value_off_the_grid_is_refused`):
//!
//! - "a rich text is not valid" (`richtext::the_write_refuses_each_rule`).
//! - "a run's properties cannot be read"
//!   (`richtext::a_run_property_that_cannot_be_read_is_refused`,
//!   `richtext::a_value_or_colour_the_write_would_refuse_is_refused_at_the_read`,
//!   `comments::rich::a_comment_s_run_properties_that_cannot_be_read_are_refused`).
//!
//! Sheet and print settings add these, the Cargo feature `sheetprops`'s
//! messages (`specs/0025-sheet-and-print-settings.md`, "Errors"; without the
//! feature neither is raised), the first at the call and the second at the
//! write. A setting also meets, at the write, "no such part", "an edit to a
//! removed part", "a cell edit to a part this write replaces" and part 3's
//! one-shift message, as any edit of the part does
//! (`sheetprops::a_setting_on_a_removed_replaced_or_absent_part_is_refused`,
//! `sheetprops::a_setting_on_a_part_with_a_pending_shift_is_refused`), and a
//! sheet view part 3's message for two `<sheetViews>` or one without a
//! `<sheetView>`
//! (`sheetprops::a_view_on_two_sheet_views_or_one_without_a_view_is_refused`):
//!
//! - "a sheet setting is out of range"
//!   (`sheetprops::a_setting_out_of_range_is_refused_at_the_call_and_records_nothing`).
//! - "a worksheet has two sheetPr, autoFilter, printOptions, pageMargins, pageSetup or headerFooter elements"
//!   (`sheetprops::two_sheet_pr_or_one_out_of_schema_order_are_refused`).
//!
//! The `cells` module raises its messages through the same error and they are
//! catalogued here with the rest, so there is one list for the modules
//! (`specs/0010-reading-cells.md`, "Errors"):
//!
//! - "the shared strings part's root is not sst"
//!   (`cells::tests::a_root_of_the_wrong_name_is_refused`,
//!   `a_root_in_another_namespace_is_refused`,
//!   `a_part_with_no_root_at_all_is_refused`).
//! - "a cell holds a reference this layer cannot expand"
//!   (`cells::tests::an_unknown_entity_is_refused`,
//!   `an_unknown_entity_in_a_cell_is_refused`), the second being the same
//!   message raised from a cell rather than from an item (P1 ruling 6).
//! - "a text element holds an element"
//!   (`cells::tests::an_element_inside_a_text_element_is_refused`,
//!   `an_element_inside_a_cells_value_is_refused`).
//! - "the worksheet part's root is not worksheet"
//!   (`cells::tests::a_worksheet_root_of_the_wrong_name_is_refused`,
//!   `a_worksheet_root_in_another_namespace_is_refused`).
//! - "a cell reference is not of the form A1"
//!   (`cells::tests::a_cell_reference_that_is_not_a1_is_refused`, and on
//!   the write path
//!   `package::tests::a_cell_reference_not_a1_refuses_the_edit`).
//! - "a row number is not a number"
//!   (`cells::tests::a_row_number_that_is_not_a_number_is_refused`,
//!   `a_row_number_of_zero_is_refused`,
//!   `a_number_this_layer_reads_is_digits_alone`,
//!   `an_index_past_u32_is_refused_like_one_that_is_not_a_number`, and on
//!   the write path
//!   `package::tests::a_row_number_not_a_number_refuses_the_edit`): a row
//!   is numbered from 1, so 0 is one of the spellings this layer has no row
//!   number for (P2 ruling 2).
//! - "a cell's style index is not a number"
//!   (`cells::tests::a_style_index_that_is_not_a_number_is_refused`,
//!   `a_number_this_layer_reads_is_digits_alone`,
//!   `an_index_past_u32_is_refused_like_one_that_is_not_a_number`).
//! - "a cell has a type this layer does not know"
//!   (`cells::tests::a_cell_type_this_layer_does_not_know_is_refused`).
//! - "a formula has a type this layer does not know"
//!   (`cells::tests::a_formula_type_this_layer_does_not_know_is_refused`,
//!   and on the write path, for an edited cell only,
//!   `package::tests::an_unknown_formula_type_refuses_an_edit_to_its_cell`).
//! - "a shared formula has no index"
//!   (`cells::tests::a_shared_formula_without_an_index_is_refused`,
//!   `a_shared_index_that_is_not_a_number_is_refused`,
//!   `a_number_this_layer_reads_is_digits_alone`,
//!   `an_index_past_u32_is_refused_like_one_that_is_not_a_number`), the last
//!   two being an `si` this layer cannot read rather than one that is
//!   absent: one message covers both (P2 ruling 4).
//! - "a shared string index is not a number"
//!   (`cells::tests::resolving_an_index_that_is_not_a_number_is_refused`,
//!   `a_number_this_layer_reads_is_digits_alone`).
//! - "a shared string index is out of range"
//!   (`cells::tests::resolving_an_index_out_of_range_is_refused`).
//!
//! # Limits
//!
//! What this layer does NOT do, each a decision rather than an omission
//! (`specs/0009-package-layer.md`, "Design" and the P3 rulings):
//!
//! - Part names are compared by exact bytes. A relationship target that
//!   differs from an entry's name in case alone resolves to nothing, and the
//!   survey counts such a target rather than the layer guessing
//!   (`case_only`).
//! - No percent-decoding of a target, and no reference is expanded in a part
//!   name: a target carrying `%` or `&` is taken as the bytes it is. The
//!   corpus's 644 internal targets carry neither.
//! - Where several relationships share a kind, the FIRST in document order
//!   is the one [`Relationships::first_of`] answers with, and the workbook's
//!   four optional parts are resolved that way. Nothing merges them.
//! - A removed element loses its own bytes and nothing else: the whitespace
//!   that surrounded it stays, because it belongs to other tokens.
//! - An addition goes immediately before the root's end tag with no
//!   whitespace added, which is how every rels part in the corpus writes
//!   consecutive elements (all 662 gaps between them are empty).
//! - A generated relationship id is `rId<n>`, one more than the largest of
//!   that form already in the part, counting the ids pending removal and
//!   those already generated for it in this write. An id of any other form
//!   is never generated and never collides with one.
//! - A rels part created for a part that had none takes its prefix from the
//!   package's own `_rels/.rels`, up to that part's root start tag, so the
//!   new part is written the way the package writes its others. With no
//!   `_rels/.rels` to read, the prefix is the form 148 of the corpus's 216
//!   rels parts use. No producer's name describes the rule, and none is
//!   claimed for it (P3 ruling 10).
//!
//! The two cell-reading entry points add their own limits
//! (`specs/0010-reading-cells.md`, "Module `package`"):
//!
//! - [`Package::shared_strings`] parses the part on EVERY call and returns
//!   the table owned, where [`Package::content_types`] and
//!   [`Package::workbook`] parse once and lend. A caller that resolves cells
//!   against the table reads it once and keeps it; calling this in a loop
//!   re-parses the part each time. Owned rather than lent because a cell
//!   reader borrows the package mutably, so a lent table could not be used
//!   while reading, and cloning one would copy every string.
//! - [`Package::cells`] borrows the package mutably for the life of the
//!   iteration, so one sheet is read at a time and nothing else may be asked
//!   of the package until the reader is dropped. Two sheets side by side, or
//!   an edit made while reading, are outside what this layer offers.
//! - Neither records an edit. A package read this way and then written comes
//!   back byte for byte, which is what the fidelity harness measures.
//!
//! The cell edits add theirs (`specs/0011-editing-cells.md`, "Module
//! `package`"; the refusals are the eight messages catalogued above):
//!
//! - A cell is written as the edit's value and nothing else. A new cell has
//!   no `s` unless a style edit gives it one; a replaced cell keeps its `s`
//!   byte for byte, unless a style edit changes it, and loses its `t`, its
//!   formula and its value; a cell written into a row that has
//!   `customFormat` takes no style from the row.
//! - Text is written as an inline string. Nothing is appended to the shared
//!   strings table, and no rich text run is written but a rich value's, the
//!   Cargo feature `richtext`'s, also inline (below).
//! - Nothing is recomputed. The cached values of formulas that depend on an
//!   edited cell stay as the producer wrote them, because emend does not
//!   evaluate. A write that changes a formula asks for a recalculation on
//!   open (below); one that changes only values does not, and a caller who
//!   wants it calls [`Package::recalculate_on_open`].
//! - When an edit removes a normal formula and the workbook has a
//!   calculation chain, the chain is dropped in the same write, once, by
//!   [`Package::drop_calc_chain`]'s recipe: the chain would otherwise name a
//!   cell with no formula. Nothing else about calculation changes.
//! - `<dimension>` and a row's `spans` are not updated when a new cell lies
//!   outside them. A cell edit copies `<cols>`, merged cells, tables, data
//!   validation and conditional formatting raw and never adjusts them; only
//!   the column formats and the merge calls of
//!   `specs/0018-rows-and-columns.md` rewrite `<cols>` and `<mergeCells>`.
//!   A cell edit inserts and deletes no row or column; `Package::insert_rows`,
//!   `Package::delete_rows`, `Package::insert_columns` and
//!   `Package::delete_columns`, the Cargo feature `rowcol`'s, move the grid
//!   and the ranges "The shift rule" names, and no formula text is fixed up.
//!   On the column axis they also move a row's `spans` (each item, the
//!   attribute dropped when none is left) and the `<col>` intervals (an
//!   emptied `<cols>` dropped, streamed and never held whole); they check a
//!   `<col>`'s `min` and `max` but not the order, overlap or number of
//!   `<cols>`, which move element by element as they stand (P3 rulings 1 and
//!   2). A
//!   hyperlink the rule drops leaves its relationship in the worksheet's
//!   rels part, unused (P2 ruling 3). A recorded shift rewrites its part
//!   whether or not it moves anything: a shift past every row, cell, merge,
//!   `dimension` and range of the sheet writes the input's bytes again, and
//!   the entry is rewritten and recompressed all the same (P2 ruling 13).
//! - A worksheet the rewrite cannot place an edit in safely is refused, not
//!   repaired: a row or cell without `r`, rows or cells out of order, an
//!   array or data table `ref` whose extent does not parse (edited or not),
//!   an edited cell in a shared, array or data table formula or carrying an
//!   attribute this layer does not model, and a set on a worksheet without
//!   `<sheetData>`. An unedited cell is copied raw, whatever it carries.
//! - A worksheet is rewritten at most once per write, in one streaming pass
//!   that holds one token and the sorted edits. A root child the write
//!   counts or reorders is held whole, bounded by that element's own size
//!   and never by the sheet's rows: `<cols>` and `<mergeCells>` when a write
//!   edits them, and `<mergeCells>` and `<dataValidations>` when it inserts
//!   or deletes rows or columns (a column shift streams `<cols>`); no row is
//!   held whole (`specs/0018-rows-and-columns.md`, P1 verification ruling 4,
//!   and P3 verification ruling 2). The edits themselves are
//!   held in memory, in a `Vec` in call order, so a caller that records more
//!   edits than its address space holds cannot.
//!
//! The style edits and additions add theirs (`specs/0012-styles.md`, "Module
//! `package`: recording and applying style edits"):
//!
//! - A new `xf` copies its base's font, fill, border, alignment and
//!   protection and changes only what the addition gives: its number format
//!   through [`Package::add_cell_style`], and a new font, fill or border
//!   through [`Package::add_style`] (`specs/0017-fuller-styles.md`).
//! - A style set on a cell whose row has `customFormat` or whose column has
//!   a `<col style>` overrides those, as any `s` does.
//! - A cell shown as a date is a number with a date format; what a consumer
//!   shows for a format code is the consumer's.
//! - `cellStyles` is never touched, and `dxfs` only by the Cargo feature
//!   `rules`'s differential styles (below). `fonts`, `fills` and
//!   `borders` grow at their ends when an addition gives a font, a fill or a
//!   border; nothing already in them is read, changed or compared.
//! - Whenever the styles part is rewritten, each of `numFmts`, `fonts`,
//!   `fills`, `borders` and `cellXfs`, grown or not, has its `count`
//!   rewritten to its children plus its additions, so one whose `count`
//!   disagreed with its children comes out corrected, even in a write that
//!   adds only a number format; one whose `count` agreed, or that has none,
//!   comes out byte for byte (`specs/0017-fuller-styles.md`, P1
//!   verification ruling 5).
//! - A style edit alone rewrites the cell's start tag and nothing else: it
//!   removes no formula, never drops the chain, and the formula refusals do
//!   not apply to it.
//! - The styles part is rewritten at most once per write, in one streaming
//!   pass that holds one token, the recorded additions and five counts.
//! - An addition is a cell format derived from an existing one, or the plain
//!   form over no base, with its number format, font, fill, border,
//!   alignment and protection as given (`specs/0017-fuller-styles.md`); the
//!   base's other children are kept as its bytes.
//! - A write with a `Some` style edit reads the styles part to check the
//!   index against the `cellXfs` count plus the additions; a `None` edit
//!   needs no styles part at all (the same section, 3).
//! - Dates are not the core's. The core writes and reads a cell's number;
//!   the serial a date stands for comes from the crate's `dates` feature,
//!   which is off by default and outside the core's size, and nothing here
//!   calls it (the same section, 2). A `t="d"` cell's text is not converted.
//!
//! The fuller cell styles add theirs (`specs/0017-fuller-styles.md`, "Module
//! `package`" and the P1 and P2 rulings; the refusals are the three messages
//! catalogued above, with "a style index is out of range" for an id `u32`
//! cannot hold):
//!
//! - The caller's picture: [`Package::add_style`] records one cell format
//!   carrying a [`Style`]'s number format, font, fill, border, alignment and
//!   protection, each `Some` field replacing what the base has; the font,
//!   the fill and the border are each appended once to `<fonts>`, `<fills>`
//!   or `<borders>`, equal ones recorded by one package sharing an element
//!   and an id; [`Package::set_cell_style`] then names the index it returns.
//! - What is not read: the part's own fonts, fills and borders are counted
//!   and nothing inside them is read, so an added font equal to one the part
//!   already has is a second copy (producers write such copies too);
//!   nothing is deduplicated at the `xf` level. A cell format's alignment,
//!   protection and apply flags are not read back as values: its ids are.
//! - What is kept: `x14ac:knownFonts` on `fonts` as it was; `dxfs` (but
//!   for the feature `rules`'s additions), `cellStyles`, `cellStyleXfs` and
//!   the theme, untouched, so a
//!   [`crate::styles::Color::Theme`] names whatever the workbook's theme defines at that
//!   index and a [`crate::styles::Color::Indexed`] whatever its `colors` or the default
//!   palette defines; a derived cell format's children other than its first
//!   main-namespace `alignment` and `protection`, as the base's bytes (one
//!   inside an `extLst` is not a child and is kept raw).
//! - What cannot be written: a font's `charset`, `outline`, `shadow`,
//!   `condense` or `extend`; a gradient fill; an inside-of-range border side
//!   (`vertical`, `horizontal`); an alignment's `readingOrder`,
//!   `justifyLastLine` or `relativeIndent`.
//! - A part without `<fonts>`, `<fills>` or `<borders>` refuses an addition
//!   that needs the collection; the template has all three. Whether a
//!   consumer shows a font name it does not have is the consumer's.
//! - Fuller styles have not been through an Excel check yet: the harness
//!   opens their outputs in LibreOffice, and Excel check 2 covers
//!   Milestone 1's edits alone.
//!
//! Formulas add theirs (`specs/0013-formulas.md`, "Module `package`"; the
//! refusals are the five messages catalogued above, and the shared, array
//! and data table refusals of the cell edits apply to a formula as to any
//! value):
//!
//! - Formulas are written, never evaluated. A written formula has no cached
//!   value, so a reader that shows cached values (emend's own, openpyxl's
//!   `data_only`) shows nothing for that cell until an application has
//!   recalculated the file; the cached values of the other formulas are
//!   unchanged and may be stale.
//! - That is why a write whose last value edit for some cell is a formula,
//!   or that removes a normal formula, drops the calculation chain (once, by
//!   [`Package::drop_calc_chain`]'s recipe, whether or not the caller has
//!   already recorded it) and sets `fullCalcOnLoad="1"` on the workbook's
//!   `calcPr`, whether or not there was a chain. The file then computes on
//!   its next open in an application that honours the flag or that
//!   recalculates on load anyway. A row or column insertion or deletion
//!   (the Cargo feature `rowcol`'s) that moves or removes at least one
//!   `<f>` is a formula change too, with the same effect; one that meets no
//!   `<f>` at or after its point leaves the chain and the flag alone
//!   (`specs/0018-rows-and-columns.md`, "The shift rule"). A caller whose
//!   edit changed an input of existing formulas without touching a formula
//!   calls [`Package::recalculate_on_open`] itself: emend cannot know the
//!   dependency graph without the chain it drops. Whether Excel honours the
//!   flag is not measured by this part; that check is the milestone's.
//! - The workbook part is rewritten once, copying every byte except the
//!   `calcPr` start tag: a present `fullCalcOnLoad` takes `1` in its own
//!   quote character, and an absent one is appended after the last
//!   attribute. An absent `calcPr` is inserted, prefixed as the root is,
//!   immediately after the last present of the nine children ECMA-376
//!   18.2.27 orders before it, or inside the root when none is present.
//!   Whitespace, comments and any root child outside the main namespace
//!   that follow the last of the nine are held, so the new element sits
//!   directly after it (P1 verification finding F1). No other `calcPr`
//!   attribute is read or written.
//! - A second `calcPr` in the workbook part is refused, and so is the flag
//!   on a workbook part the same write replaces or removes. Validation
//!   refuses both before any part is rewritten when the flag is recorded or
//!   a formula is written; when the rule fires only because the worksheet
//!   rewrite found a removed formula, the refusal comes at the rule's
//!   second validation, after the worksheet temporaries exist, which are
//!   then removed, and no output is written (P1 verification ruling 4).
//! - A worksheet with two `<sheetData>` elements is edited in the first and
//!   the second is copied raw; a reader counts cells in both (P2 ruling
//!   3(a)).
//!
//! A new workbook adds these (`specs/0016-new-workbook-and-sheets.md`, "The
//! template" and "`Package::new`"):
//!
//! - [`Package::new`] is a fixed template of six parts, one sheet named
//!   `Sheet1` with no cell, written through the ordinary write and opened
//!   back; there is no writer of its own and no template of the caller's. A
//!   caller who wants another starting workbook opens it and edits it.
//! - The template has no `docProps` parts, no shared strings part, no theme,
//!   no `calcPr` and no calculation chain; a formula written into it gets
//!   `calcPr` from the formula-change rule, as in any workbook.
//! - Its styles part carries one font, the two fills Excel requires, one
//!   empty border, one cell format and the `Normal` cell style, which is the
//!   precondition `crate::styles` states for a style edit.
//! - Writing the template compresses each part through a temporary file in
//!   the system's temporary directory, as the archive layer does for every
//!   added entry, and the temporary is removed however the write ends.
//!
//! The sheet list adds these (`specs/0016-new-workbook-and-sheets.md`,
//! "Sheet names", "Renaming and moving", "The workbook rewrite", and P3
//! rulings 4 to 8):
//!
//! - [`Package::workbook`] answers with the file's sheets and
//!   [`Package::sheets`] with the projected list; the edits are checked
//!   against the projected list when they are recorded, and the write
//!   replays them.
//! - Names are compared ignoring case through `char::to_lowercase` on both
//!   sides, an approximation of Excel's rule that needs no table of its own.
//!   Nothing else about a name is checked beyond the rules the catalogue
//!   lists.
//! - A rename or a move changes the sheet list and nothing that names a
//!   sheet in text: formulas, defined names' text, pivot sources, chart
//!   series and the `docProps` title list are untouched, as openpyxl leaves
//!   them.
//! - When the list is rewritten, whitespace between the `<sheet>` elements is
//!   not reproduced; the children are written back to back.
//! - A `localSheetId` past the file's sheet count names no sheet and is
//!   copied unchanged. `activeTab` follows its sheet; `firstSheet`, the tab
//!   strip's scroll position, is not moved by an add or a move, even past
//!   the end. When the list is rewritten, a `localSheetId`, `activeTab` or
//!   `firstSheet` that is not a decimal `u32` refuses the write, since the
//!   remap cannot say what it means (P3 ruling 10).
//! - An added sheet's part is the template's blank worksheet, with no
//!   `tabSelected` view; a cell edit on it is written into that part.
//! - A `sheetId` past `u32::MAX` and a sheet added to a workbook part with
//!   no `<sheets>` are refused with "a sheet edit no longer applies at the
//!   write" (P3 ruling 6); neither occurs in the corpus (P3 verification
//!   ruling 1).
//!
//! Removing a sheet adds these (`specs/0016-new-workbook-and-sheets.md`,
//! "Removing a sheet", and P4 rulings 1 to 5 and 9):
//!
//! - The caller's picture: [`Package::remove_sheet`] takes the sheet out of
//!   the list, removes its relationship, and removes every part only it
//!   reaches, with those parts' rels parts and the overrides of each, so no
//!   `Override` names a removed entry (P4 ruling 9); `definedName`
//!   elements scoped to it go, and the other sheet indexes in
//!   `<definedNames>` and `<bookViews>` follow their sheets.
//! - A sheet is visible when its `state` is absent or exactly `visible`; any
//!   other value counts as hidden. The last sheet and the last visible
//!   sheet are not removed.
//! - What the walk does not do: it reads no worksheet text, so a formula in
//!   a kept sheet naming the removed sheet becomes `#REF!` when Excel
//!   recalculates, as with openpyxl; it does not touch pivot caches,
//!   external links, `definedNames` text, the `docProps` title list or
//!   `tabSelected` in kept sheets; and a part reachable from the removed
//!   sheet and from a kept part is kept whole.
//! - A workbook with a calculation chain loses it, and gets
//!   `fullCalcOnLoad="1"`, when a sheet is removed: the chain names cells
//!   the sheet held.
//! - A file entry recorded for removal still counts as taken, so a sheet
//!   added after the removal takes the next free name; its `sheetId` may
//!   equal the removed sheet's, and a relationship id is never reused (P4
//!   ruling 2). A cell or style edit recorded on a file sheet that is then
//!   removed refuses the write with "an edit to a removed part".
//! - Sheet edits have not been through an Excel check yet: the harness
//!   opens their outputs in LibreOffice, and Excel check 2 covers
//!   Milestone 1's edits alone.
//!
//! Rows and columns add these (`specs/0018-rows-and-columns.md`, "Design",
//! decisions 2 to 7 and the P1 to P3 rulings), the caller's picture:
//!
//! - Formats: [`Package::set_column_width`] and
//!   [`Package::set_column_hidden`] change the `<col>` covering the column,
//!   split so that the edited column alone carries the change and every
//!   other column keeps every attribute it had; a column no `<col>` covers
//!   gets an element of its own, and a width of `None` there writes nothing.
//!   [`Package::set_row_height`] and [`Package::set_row_hidden`] retag the
//!   `<row>`, every other attribute and child kept, or create it empty in
//!   row order. A number is written by the cell edits' form. Styles on rows
//!   and columns (`<row s>`, `<col style>`), outline levels and
//!   `sheetFormatPr` defaults are kept when present and never set.
//! - Panes: [`Package::freeze_panes`] replaces the first `<sheetView>`'s
//!   `<pane>` and every `<selection>` with the frozen pane and one
//!   selection, or removes both for `None`; the view's other children and
//!   every other view are copied raw. Split (unfrozen) panes and selections
//!   are not modelled.
//! - Merges: [`Package::merge_cells`] appends a `<mergeCell>` and sets
//!   `count`; the covered cells other than the top-left lose their value
//!   and formula and keep their `s`, as a cleared cell does, and a formula
//!   cleared so is a formula change. [`Package::unmerge_cells`] removes a
//!   merge of the file whose range is exactly the area and writes nothing
//!   into the cells, so values the merge cleared do not come back.
//! - An absent `<cols>`, `<sheetViews>` or `<mergeCells>` the write needs is
//!   created in schema position, before the first root child of the main
//!   namespace that the worksheet's order puts later.
//! - Insertions and deletions, the Cargo feature `rowcol`: what moves with
//!   the rows or columns is the grid (every `<row r>` and `<c r>`), a
//!   deleted row's or column's cells going whole and every row kept on a
//!   column deletion; `dimension`; the merges; on the column axis the
//!   `<col>` intervals and a row's `spans`; the shared, array and data table
//!   formulas' `ref`; and the ranges of conditional formats, data
//!   validations, hyperlinks and the filter. Each is moved, grown, cut,
//!   clipped to the grid or dropped as "The shift rule" gives, and the
//!   counts follow.
//! - What stays where it was: the text of every formula, in cells, defined
//!   names, conditional formats, data validations and tables; defined names,
//!   print areas and filter databases; the views; outline levels and
//!   `collapsed`, which travel with their row or column; comments and their
//!   anchors; drawings and charts; `extLst` content with its own ranges;
//!   ignored errors, protected ranges, breaks, sort state and pivot caches;
//!   a row's `spans` on a row shift. A caller inserting above a formula's
//!   input, a comment or a chart gets openpyxl's result: the cells move and
//!   the rest points where it did. A table or pivot table not wholly before
//!   the point on the edit's axis is refused rather than moved, since moving
//!   their parts is later work.
//! - One insertion or deletion per worksheet per write, and no other edit
//!   of that worksheet in the same write (decision 5): every recorded edit
//!   is in the input file's coordinates. A caller who wants to insert and
//!   then fill writes twice: `write_to` into a `Cursor<Vec<u8>>`,
//!   [`Package::open`] on its bytes, the edits, the second write.
//! - Nothing of this is read back as a typed value: widths, heights, panes
//!   and merges stay unmodelled on the read side, and a merged range reads
//!   as its cells.
//! - Rows and columns have not been through an Excel check yet: the harness
//!   opens every output in LibreOffice, and the Excel check of Milestone 2
//!   follows its close-out push.
//!
//! Names, links, properties and protection add these, all the Cargo feature
//! `metadata`'s (`specs/0019-names-links-properties-protection.md`,
//! "Design", decisions 1 to 5 and the P1 to P3 rulings), the caller's
//! picture:
//!
//! - Defined names: `Package::add_defined_name` appends a
//!   `<definedName>` to `<definedNames>`, created in schema position when
//!   absent, its text stored as given and never parsed; a scope names a
//!   sheet of the projected list and follows it to its index in the output.
//!   `Package::remove_defined_name` drops the file's names of that name
//!   (ignoring case) and scope, and a container left empty. Either is a
//!   formula change: the chain dropped once, `fullCalcOnLoad` set. Nothing
//!   that uses a name (a formula, a validation, a print setting) is touched
//!   or checked, and the built-in `_xlnm.` names are out of scope.
//! - Hyperlinks: `Package::set_hyperlink` replaces the link whose `ref` is
//!   exactly the cell, or appends one, as `r:id` for an external target
//!   (its relationship on the worksheet's rels part, `TargetMode` external)
//!   or `location` for a place in the workbook; `Package::remove_hyperlink`
//!   drops it and, when no kept link names the same `r:id`, its
//!   relationship. The cell's value and style are not touched and no
//!   `display` is written. A root binding no prefix to the relationships
//!   namespace gains `xmlns:r` (or `r1`, `r2`, ...), in the form of the
//!   worksheet's own relationship. A sheet added in the same write takes a
//!   location link and refuses an external one ("no such part"); the
//!   caller writes twice. A link whose `ref` is a range is never matched.
//! - Core properties: `Package::set_property` replaces an element's
//!   content in place (its start tag kept; an empty-element tag given
//!   content is opened up), removes it, or appends it with the prefix the
//!   root binds to its namespace, refusing an append the root binds no
//!   prefix for (`xsi` bound only as the default namespace counts as none).
//!   With no core-properties relationship the write adds
//!   `docProps/core.xml`, its package relationship and its `Override`; a
//!   relationship naming a part the archive lacks refuses the edit ("no such
//!   part"), as the read does. `app.xml` and `custom.xml` are never touched:
//!   `TitlesOfParts`, `Company` and the custom properties stay as they are.
//! - Protection: `Package::protect_sheet` writes `<sheetProtection>` with
//!   each option that differs from the schema's default, so the default
//!   options write `<sheetProtection sheet="1"/>` and `sheet` false with
//!   the rest at their defaults writes `<sheetProtection/>`;
//!   `Package::protect_workbook` writes `<workbookProtection>` with its
//!   locks. Each replaces the part's element or creates one in schema
//!   position; an unprotect drops it. An unprotect of a sheet with no
//!   element still rewrites the worksheet part, to its own bytes (the
//!   harness lists it recompressed); the workbook's does not rewrite the
//!   workbook part. The password is stored as the legacy sixteen-bit hash
//!   (`legacy_password_hash`), which is Excel's advisory lock and not
//!   encryption: the hash is trivially reversed and Excel removes the lock
//!   on request. `protectedRanges` and the SHA-512 form are out of scope.
//! - Read back as typed values: the defined names (`Workbook::defined_names`,
//!   read with the sheets, so with the feature on the workbook read refuses
//!   a `definedName` whose `localSheetId` is not a decimal `u32`, whose
//!   attribute carries a prefix nothing binds, or whose text the cell
//!   reader refuses, for every caller) and the core properties
//!   (`Package::properties`). Hyperlinks and protection are not read back.
//! - With the feature on, [`Workbook`] holds `pub(crate)` fields for the
//!   names it reads, so it is not constructible outside the crate.
//! - None of this has been through an Excel check yet: the harness opens
//!   every output in LibreOffice, and the Excel check of Milestone 2 is
//!   pending, run or deferred again before the close-out push.
//!
//! Validations and conditional formats add these, all the Cargo feature
//! `rules`'s (`specs/0020-validation-and-conditional-formatting.md`,
//! "Design", decisions 1 to 5 and the P1 to P3 rulings), the caller's
//! picture:
//!
//! - What they touch: `Package::add_validation` and
//!   `Package::remove_validation` edit the worksheet's `<dataValidations>`
//!   alone; `Package::add_conditional_format` and
//!   `Package::remove_conditional_format` its `<conditionalFormatting>`
//!   elements alone; `Package::add_differential_style` the styles part's
//!   `<dxfs>` alone (and, as every styles rewrite, the other containers'
//!   counts). No cell's value or style changes; a cell edit in the same
//!   write composes with them
//!   (`validation::a_validation_and_a_cell_edit_on_one_part_compose`,
//!   `conditional::a_conditional_format_and_a_cell_edit_on_one_part_compose`);
//!   a row or column insertion or deletion does not (the one-shift rule).
//!   An absent container is created in schema position; a part out of
//!   schema order, whose one container comes after a child that follows it,
//!   is refused with the two-container message rather than written into.
//! - A formula is text: a validation's `formula1` and `formula2`, a rule's
//!   `<formula>` children and a formula value point's `val` hold what the
//!   caller gave, escaped, without a leading `=`, never parsed and never
//!   evaluated; a number, percent or percentile value point's `val` holds
//!   the caller's number as the crate prints numbers. None is a formula
//!   change (no chain dropped, no recalculation asked for). A list's
//!   `formula1` is a range or a quoted list of constants; the crate does not
//!   tell them apart. The blank rules carry Excel's formula for the first
//!   area's top-left cell.
//! - One validation per cell: an addition overlapping a validation the
//!   part keeps, or one added before it in the write, is refused (decision
//!   4 A); a removal matches the file's elements by their parsed `sqref`,
//!   area for area in order, never an addition of the same write, and drops
//!   every such element. Titles are limited to 32 characters and texts to
//!   255, as Excel limits them.
//! - The priority rule: a rule's priority not given is the part's highest
//!   kept priority (the main namespace's rules only; an `x14` rule's is not
//!   counted) plus one, one more for each earlier addition without one, as
//!   openpyxl assigns it; one given is written as given, and two rules of
//!   one priority are Excel's to order. A dxf rule needs a differential
//!   style and a visual rule takes none.
//! - Differential styles: `Package::add_differential_style` returns the
//!   styles part's `dxf` children plus the earlier additions, the position
//!   a `dxfId` names; the `<dxf>` holds the style's font, fill and border as
//!   a cell format's are written, without `apply` flags; nothing is
//!   deduplicated and no `dxf` is removed. Excel reads a differential solid
//!   fill's colour from `bgColor`, so a caller sets the fill's `background`
//!   (decision 5 A), as openpyxl's `PatternFill(bgColor=..)` does. A number
//!   format, an alignment or a protection is refused at the call, "a
//!   differential style holds a field this part does not write", and so is a
//!   style with none of a font, a fill and a border.
//! - The visual rules are the 2007 form (decision 2 A): a data bar is a
//!   solid bar of one colour with no negative bar or axis, and no `x14`
//!   extension is written. A rule the part keeps whose `cfRule` holds an
//!   `extLst` (twinned with an `x14` element in the sheet's extension list,
//!   which this crate never touches) is not removed: its removal is refused,
//!   so no twin is left without its rule. `x14` validations are kept and
//!   never read.
//! - What is read back (decision 3 A): `Package::validations` gives each
//!   validation's areas, kind, operator, formulas, flags and messages;
//!   `Package::conditional_formats` each rule's areas, `type` text (a kind
//!   this crate does not write included), operator (none for a text
//!   operator), formulas, style, priority, stop and whether it holds an
//!   extension, but not a visual rule's children. The differential styles
//!   are not read back. Both reads are snapshots of the file.
//! - None of this has been through an Excel check yet: the harness opens
//!   every output in LibreOffice, and the Excel check (`specs/0019`
//!   decision 6, deferred again) is pending, run or deferred again before
//!   the close-out push; what Excel shows for a rule is Excel's.
//!
//! Tables add these, all the Cargo feature `tables`'s (`specs/0021-tables.md`,
//! "Design", decisions 1 to 5 and the P1 and P2 rulings), the caller's
//! picture:
//!
//! - What they touch: `Package::add_table` writes one new part,
//!   `xl/tables/tableN.xml`, with its `Override` and a `table` relationship
//!   on the worksheet's rels part (created when the sheet has none), a
//!   `<tablePart>` in the worksheet's `<tableParts>` (created in schema
//!   position when absent, its `count` set to its children) and the header
//!   cells, the area's top row, as text through the cell edits, so a later
//!   `set_cell` on a header cell wins, and a header cell the file keeps as
//!   a `<c>` holding an `<f>` is refused. `Package::remove_table` drops the `<tablePart>`, its
//!   relationship, the part's `Override` and the entry, and a
//!   `<tableParts>` left empty; the cells keep their values and styles, and
//!   a `dxf` a removed table named stays in `<dxfs>`. A table part's own
//!   rels part, which no corpus workbook and no Excel writes, is not
//!   dropped by the removal: out of scope (P2 verification ruling 2). A
//!   root binding no prefix to the relationships namespace gains one, in
//!   the form of the worksheet's own relationship, as for a hyperlink. A
//!   row or column insertion or deletion does not compose with a table
//!   edit (the one-shift rule), and a sheet added in the same write takes
//!   neither: the addition meets "no such part" and the removal "no table
//!   of that name", the removal finding no file table first; the caller
//!   writes twice.
//! - Names: a table's name is written as both `name` and `displayName`,
//!   and shares the defined names' space: at the write it is compared,
//!   ASCII case folded, with every table and every `definedName` of the
//!   projected workbook (a table or sheet the write removes not counted, a
//!   table it adds counted). A removal matches `name` or `displayName` with
//!   the case folded, never a table the same write adds, and one table
//!   named twice for removal in one write is refused. The entry name's N is
//!   the smallest free in the input's archive and the write's additions,
//!   the names compared ASCII case folded (a removed entry's name still
//!   taken); the `id` is the smallest no table
//!   part of the projected workbook carries.
//! - The shape written, openpyxl's and Excel's for a new table with no
//!   totals row: the declaration on its own line, the root in the default
//!   namespace with `id`, `name`, `displayName`, `ref` and
//!   `totalsRowShown="0"`, an `<autoFilter>` of the table's `ref`, one
//!   `<tableColumn>` per name with ids 1 upward, and a `<tableStyleInfo>`
//!   naming a built-in style (`TableStyleLight1` to `21`, `Medium1` to
//!   `28`, `Dark1` to `11`) with its four flags written as `0` or `1`, or
//!   none when the caller gives no style; the relationship's target is
//!   relative (`../tables/tableN.xml`).
//! - What is never written: a totals row (`totalsRowCount` and the
//!   `totalsRow` attributes and children), a calculated column, a custom
//!   style (`<tableStyles>` in the styles part is never touched), a
//!   `headerRowCount` of `0`, a sort state, a filter on a column, the
//!   `dxfId` and cell-style attributes, and the `xr` namespace and its
//!   `xr:uid`. A table the part reads keeps all of these as they are, and
//!   no table is resized, renamed or moved in place.
//! - What is read back: `Package::tables` gives each table's name, display
//!   name, area, column names, style name (built-in or custom, as text)
//!   with its four flags, whether it has a header row, a totals row and a
//!   calculated column, and its entry name, from a target in either the
//!   absolute or the relative form; a snapshot of the file, not of the
//!   pending edits.
//! - None of this has been through an Excel check yet: the harness opens
//!   every output in LibreOffice, and the Excel check (`specs/0019`
//!   decision 6) is pending, run or deferred again before the close-out
//!   push; what Excel shows for a table's banding is Excel's.
//!
//! Comments add these, all the Cargo feature `comments`'s
//! (`specs/0022-comments-and-images.md`, "Comments: the calls" and
//! "Comments: the parts", decisions 2, 3 and 6, the P1, P2 and P3 rulings),
//! the caller's picture:
//!
//! - What they touch: a sheet's comments part and its VML part, each
//!   appended to or created (`xl/comments<N>.xml`,
//!   `xl/drawings/vmlDrawing<N>.vml`, N the smallest free in the archive
//!   and the write's additions, ASCII case folded, a removed entry's name
//!   still taken) with its `Override` and a `comments` or `vmlDrawing`
//!   relationship in the form of the worksheet's own; the worksheet only
//!   when its `<legacyDrawing>` is created in schema position (the root
//!   retagged as for a table) or dropped with the last comment.
//! - The shape written, openpyxl's: a comment's text as one run with no
//!   properties, its author appended to `<authors>` unless a kept author's
//!   text equals it; one hidden note shape per comment in the fixed box
//!   the design gives, numbered `1024 * D + K`. `K` is one above the larger
//!   of the part's kept `v:shape` root children and the largest
//!   `_x0000_s<n>` number in `D`'s range a kept shape carries on `id` or
//!   `o:spid` (P1 ruling 1). `D` is the part's own `idmap`, or the smallest
//!   positive integer no VML entry the write keeps uses as its `idmap` (an
//!   entry this write removes not counted); the uniqueness of shape ids
//!   across a workbook rests on distinct `idmap` numbers per part.
//! - Removal: a comment's element and its note shape go; a `commentList`
//!   left empty drops the comments part, its relationship, its `Override`
//!   and the entry, and the VML part goes with it, its `<legacyDrawing>`
//!   too, unless it still holds a shape of another `ObjectType` or with
//!   none (a form control, a plain drawing shape), when it is kept with
//!   that shape. Authors are never removed.
//! - What cannot be read, refused at the write: a comments part whose root
//!   is not `comments`, whose `commentList` comes before any `authors` or
//!   is missing or doubled, or a kept `comment` without a `ref` that parses;
//!   a VML part whose root is not an unprefixed `xml` or does not bind a
//!   prefix to each of the three VML namespaces.
//! - What is never written: formatting runs (but through
//!   `Package::add_rich_comment`, the Cargo feature `richtext`'s, below), a
//!   comment's size or position beyond the fixed box, a threaded comment. A
//!   sheet with a threaded comments relationship refuses every comment edit
//!   (decision 3 A); `Package::comments` reads such a sheet's legacy fallback,
//!   the comments part Excel writes beside the threaded one.
//! - What is read back: `Package::comments` gives each comment's cell,
//!   author and text, runs flattened, and with `richtext` its runs (below);
//!   a snapshot of the file.
//!
//! Images add these, all the Cargo feature `images`'s
//! (`specs/0022-comments-and-images.md`, "Images: the calls" and "Images:
//! the parts", decisions 4, 5 and 6, the P3 rulings), the caller's picture:
//!
//! - What they touch: one media entry per image, `xl/media/image<N>.png` or
//!   `.jpeg` with N free over both extensions, with its `Override`; the
//!   sheet's drawing part, appended to (the kept bytes a prefix, one
//!   `oneCellAnchor` more, its `cNvPr` `id` one above the largest anywhere
//!   in the part) or created (`xl/drawings/drawing<N>.xml`, the
//!   transitional namespaces, the relationship types in the sheet's form)
//!   with its `Override`; the drawing's rels part, created when it has
//!   none; the worksheet only when its `<drawing>` is created in schema
//!   position. The drawing is the one the sheet's `<drawing>` element's
//!   `r:id` names, for the read and the write alike.
//! - What cannot be read, under "a drawing part or its relationship cannot
//!   be read", beside the spec's list: a `pic` with no `blip`, an empty
//!   `wsDr` root on an append, a `cNvPr` `id` that is not a decimal number
//!   within `u32`, and an external drawing or `r:embed` relationship.
//! - What is never written: a two-cell or absolute anchor, an SVG, EMF,
//!   GIF, BMP or TIFF image, an image's `descr`, an image removal; images
//!   are never checked for overlap.
//! - What is read back: `Package::images` gives each picture's anchor kind,
//!   `from` and media entry with its size, from a root anchor or the first
//!   anchor in a root `AlternateContent`'s first `Choice` (P3 ruling 1); a
//!   snapshot of the file.
//! - None of this has been through an Excel check yet: the harness opens
//!   every output in LibreOffice; what Excel shows for a note or a picture
//!   is Excel's.
//!
//! Rich text adds these, all the Cargo feature `richtext`'s, a comment's
//! runs with `comments` too (`specs/0024-rich-text.md`, "Design", decisions
//! 2 to 5, the P1 and P2 rulings), the caller's picture:
//!
//! - What it touches: a `Value::Rich` is one cell edit in the worksheet's
//!   one rewrite, as `Value::Text` is, written as an inline string
//!   (`t="inlineStr"`) whose `<is>` holds one `<r>` per run: the run's
//!   `<rPr>` first when it has a font, then `<t xml:space="preserve">`, a
//!   carriage return as `&#13;`. The `<rPr>` is the font's `<font>` element
//!   renamed, the typeface child `rFont` for `name`, the children in
//!   `Font::compose`'s order (`b`, `i`, `strike`, `u`, `vertAlign`, `sz`,
//!   `color`, `rFont`, `family`, `scheme`), of which each of the corpus's 17
//!   measured sequences is a subsequence (M3 part 2 P2's run log,
//!   `records/logs/m3p2-p2/run.log`); a default font is `<rPr/>`.
//!   `Package::add_rich_comment` is `add_comment` with its `<text>` holding
//!   the runs so, each `<t>` with `xml:space="preserve"` only when its text
//!   begins or ends with whitespace, as a plain comment's.
//! - What is never written: a run into the shared strings table (a rich
//!   value is always an inline string, decision 2); the five run
//!   properties [`crate::styles::Font`] has no field for, `charset`,
//!   `outline`, `shadow`, `condense` and `extend`; a phonetic run
//!   (`<rPh>`, `<phoneticPr>`); a run with no text, which the write and the
//!   comment call refuse.
//! - What is read back: `Cell::runs` for an inline string,
//!   `SharedStrings::runs_of` for an item of the table and
//!   `CommentEntry::runs` for a comment, each `Some` when an `<r>` was met
//!   and a run with text is left, a leading plain `<t>` counted, beside the
//!   flattened text the value, the item and the comment's `text` keep. A
//!   plain `<t>` before the runs reads as a first run with no font, so a
//!   container whose only text is such a `<t>` reads as that one run; a
//!   `<t>` after the runs, which the schema does not allow, reads as a run
//!   of its own in its place.
//! - What the read drops from the typed view: the five children above; a
//!   `tint` beside `rgb` or `indexed` (P1 verification ruling 5); an
//!   `<r/>`, a run whose `<t>` texts total empty and an empty `<t>` after
//!   the runs, so a container whose every run is empty reads `None` (P1
//!   verification ruling 4, P2 verification ruling 4); an `<rPh>`, skipped
//!   as the flattened text of a cell or an item skips it. A comment's
//!   `text` still holds a phonetic run's text, as `specs/0022` defines it,
//!   so it and the runs differ only for a comment with an `<rPh>`. An
//!   unedited cell, item or comment is copied raw, so the file keeps
//!   everything the typed view drops.
//! - What cannot be read, "a run's properties cannot be read" at the read of a
//!   cell, the table or a comment: an `<rPr>` child outside the main namespace,
//!   one this layer does not know or holding an element, a `val` that does not
//!   parse, a colour with no form [`crate::styles::Color`] has, a size or tint
//!   that is not finite, a tint outside `-1.0..=1.0`, and any font
//!   [`Package::add_style`] would refuse, so every rich text the read gives is
//!   one the write accepts. With `comments` and `richtext` both on,
//!   `Package::comments` refuses a kept comment whose `<rPr>` it cannot read,
//!   so a reader of comments refuses that workbook.
//! - The table's runs: the rich items' runs sit in one list sorted by item
//!   index, found by binary search, with nothing for an item without runs;
//!   the reader's listener allocates nothing until an `<r>` is met (P1
//!   ruling 3). The contract (decision 4) is that the table's memory for an
//!   item without runs does not grow; its measure is M3 part 2 P3's run
//!   log (`records/logs/m3p2-p3/run.log`), which repeats part 1's three read
//!   runs and checks each peak RSS against part 1's decision record: on tier
//!   d (4800000 items, none rich) each of `--shared-strings` and `--cells`
//!   peaked within 104 kB of part 1's on both labels.
//! - None of this has been through an Excel check yet; what Excel shows for
//!   a run is Excel's.
//!
//! # What streams and what is held
//!
//! Measured on the generated scale workbooks of `specs/0023-scale.md`
//! (Milestone 3 part 1: P2's run log, `records/logs/m3p1-p2/run.log`, and
//! P3's, `records/logs/m3p1-p3/run.log`, which holds the read runs), each
//! tool run under a 2048 MiB address-space cap on the host and on
//! `i686-unknown-linux-musl`. Tier c is one worksheet of 1048576 rows by
//! 120 columns whose entry is 5025024826 bytes uncompressed, a zip64 entry
//! larger than a 32-bit address space; the figures are the peak resident
//! set of the whole tool run, the larger of the two targets'.
//!
//! In memory independent of a part's size:
//!
//! - Reading cells: [`Package::cells`] is an iterator over the entry's
//!   reader, one token at a time. A whole pass over tier c's sheet runs
//!   within the runs' peaks below.
//! - Rewriting a worksheet: a cell, style, protection, row or column edit
//!   streams the input entry token by token into a temporary file, which is
//!   then compressed into a second one and copied out in fixed blocks. Tier
//!   c's sheet entry was rewritten this way by a formula edit, a row
//!   deletion, a column insertion, a column deletion and a sheet
//!   protection, at most 7860 kB on either target, its sizes over `u32`
//!   carried whole. The temporaries are files in the directory
//!   `std::env::temp_dir` names (`TMPDIR`), on disk or wherever that is,
//!   and their size follows the part, not the process's memory.
//! - Every entry no edit touches, the large sheet included, is copied raw
//!   in fixed blocks without being decompressed: a properties edit, a sheet
//!   added and a sheet removed on tier c each ran in under 4 s at most
//!   7864 kB.
//!
//! Held whole, so the memory follows the part:
//!
//! - The shared strings table: [`Package::shared_strings`] returns it as
//!   an owned list of every item; its cost at scale is the decision record
//!   of `specs/0023-scale.md`'s P3. With the Cargo feature `richtext` the
//!   rich items' runs are held beside it, and nothing for any other item
//!   (Limits, rich text).
//! - The workbook, content types, relationships and styles parts, each
//!   parsed into its model once; and an existing comments VML part or
//!   drawing part, read whole when a comment or an image is added beside
//!   it. Each is small next to a worksheet.
//! - The recorded edits, as the cell edits above say, and a root child the
//!   write counts or reorders, bounded by that element and never by the
//!   sheet's rows.
//!
//! An edit that needs a row or column off the grid is refused, so on a
//! sheet whose rows reach 1048576 a write that adds a row after the last is
//! refused with "a row or column index or count is out of range" (P1
//! verification ruling 5). A caller that writes the whole package into
//! memory, as some of the harness tool's read-backs do, holds the whole
//! output archive itself; that is the caller's memory, not this layer's.

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Cursor, Read, Seek, SeekFrom, Write};
use std::str;

use quick_xml::XmlVersion;
use quick_xml::escape::escape;
use quick_xml::events::{BytesStart, Event};

use crate::archive::{Archive, Core, Edits, Error, Replacement, TempFile, replacement_from_reader};
use crate::cells::{
    Area, CellRef, Cells, SharedStrings, Value, decimal, forbidden_in_xml, number_text,
};
#[cfg(all(feature = "comments", feature = "richtext"))]
use crate::cells::{Collect, RichText, Runs};
use crate::styles::{
    Alignment, Border, Changes, Composed, Fill, Font, NewXf, NumberFormat, Protection, Style,
    Styles, counted, num_fmt_element, scan_start_tag,
};
#[cfg(any(feature = "rules", feature = "sheetprops"))]
use crate::styles::{Color, color_element};
use crate::xml::{BYTE_ORDER_MARK, Scope, Stream};

/// The part every package must have, by exact bytes.
pub const CONTENT_TYPES_PART: &[u8] = b"[Content_Types].xml";

/// The namespace of `[Content_Types].xml` (OPC, part 2).
pub const CONTENT_TYPES_NS: &[u8] = b"http://schemas.openxmlformats.org/package/2006/content-types";

/// The namespace of a `.rels` part (OPC, part 2).
pub const RELATIONSHIPS_NS: &[u8] = b"http://schemas.openxmlformats.org/package/2006/relationships";

/// The main spreadsheet namespace, transitional then strict.
pub const MAIN: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/spreadsheetml/2006/main",
    b"http://purl.oclc.org/ooxml/spreadsheetml/main",
];

/// The namespace of the `r:id` attribute, transitional then strict.
pub const RELATIONSHIPS_ATTRIBUTE: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships",
];

/// The `officeDocument` relationship type, transitional then strict.
pub const OFFICE_DOCUMENT: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument",
];

/// The `worksheet` relationship type, transitional then strict.
pub const WORKSHEET: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet",
];

/// The `chartsheet` relationship type, transitional then strict.
pub const CHARTSHEET: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/chartsheet",
];

/// The `dialogsheet` relationship type, transitional then strict.
pub const DIALOGSHEET: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/dialogsheet",
];

/// The `sharedStrings` relationship type, transitional then strict.
pub const SHARED_STRINGS: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/sharedStrings",
];

/// The `styles` relationship type, transitional then strict.
pub const STYLES: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/styles",
];

/// The `calcChain` relationship type, transitional then strict.
pub const CALC_CHAIN: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/calcChain",
];

/// The `theme` relationship type, transitional then strict.
pub const THEME: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/theme",
];

/// The `table` relationship type, transitional then strict
/// (`specs/0018-rows-and-columns.md`, "The shift rule"; feature `rowcol`,
/// and feature `tables` for `specs/0021-tables.md`).
#[cfg(any(feature = "rowcol", feature = "tables"))]
pub const TABLE: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/table",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/table",
];

/// The `pivotTable` relationship type, transitional then strict (feature
/// `rowcol`, and feature `tables`).
#[cfg(any(feature = "rowcol", feature = "tables"))]
pub const PIVOT_TABLE: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/pivotTable",
];

/// A table part's content type, which the strict form shares (feature
/// `tables`).
#[cfg(feature = "tables")]
const TABLE_CONTENT_TYPE: &[u8] =
    b"application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml";

/// The `comments` relationship type, transitional then strict (the Cargo
/// feature `comments`; `specs/0022-comments-and-images.md`, "Comments: the
/// parts").
#[cfg(feature = "comments")]
pub const COMMENTS: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/comments",
];

/// The `vmlDrawing` relationship type, transitional then strict, which a
/// worksheet's `<legacyDrawing>` names (the Cargo feature `comments`).
#[cfg(feature = "comments")]
pub const VML_DRAWING: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/vmlDrawing",
];

/// The `threadedComment` relationship type. Microsoft's extension has one
/// form, written twice so it is looked up as the pairs are (the Cargo
/// feature `comments`; decision 3 A).
#[cfg(feature = "comments")]
pub const THREADED_COMMENTS: [&[u8]; 2] = [
    b"http://schemas.microsoft.com/office/2017/10/relationships/threadedComment",
    b"http://schemas.microsoft.com/office/2017/10/relationships/threadedComment",
];

/// A comments part's and a VML part's content types, which the strict form
/// shares (the Cargo feature `comments`).
#[cfg(feature = "comments")]
const COMMENTS_CONTENT_TYPE: &[u8] =
    b"application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml";
#[cfg(feature = "comments")]
const VML_CONTENT_TYPE: &[u8] = b"application/vnd.openxmlformats-officedocument.vmlDrawing";

/// The three VML namespaces, in the order `v`, `o`, `x` (ECMA-376 Part 4;
/// the Cargo feature `comments`).
#[cfg(feature = "comments")]
const VML_NAMESPACES: [&[u8]; 3] = [
    b"urn:schemas-microsoft-com:vml",
    b"urn:schemas-microsoft-com:office:office",
    b"urn:schemas-microsoft-com:office:excel",
];

/// The `drawing` relationship type, transitional then strict, which a
/// worksheet's `<drawing>` names (the Cargo feature `images`;
/// `specs/0022-comments-and-images.md`, "Images: the parts").
#[cfg(feature = "images")]
pub const DRAWING: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/drawing",
];

/// The `image` relationship type, transitional then strict, which a
/// drawing's `blip` names (the Cargo feature `images`).
#[cfg(feature = "images")]
pub const IMAGE: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/image",
];

/// A drawing part's content type, which the strict form shares (the Cargo
/// feature `images`).
#[cfg(feature = "images")]
const DRAWING_CONTENT_TYPE: &[u8] = b"application/vnd.openxmlformats-officedocument.drawing+xml";

/// The spreadsheet-drawing namespace and the DrawingML one, transitional
/// then strict (ECMA-376 Part 1, 20.5 and 20.1; the Cargo feature `images`).
#[cfg(feature = "images")]
const SPREADSHEET_DRAWING: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing",
    b"http://purl.oclc.org/ooxml/drawingml/spreadsheetDrawing",
];
#[cfg(feature = "images")]
const DRAWINGML: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/drawingml/2006/main",
    b"http://purl.oclc.org/ooxml/drawingml/main",
];

/// The markup-compatibility namespace, whose `AlternateContent` a drawing
/// may wrap an anchor or a picture in (the Cargo feature `images`).
#[cfg(feature = "images")]
const MARKUP_COMPATIBILITY: &[u8] = b"http://schemas.openxmlformats.org/markup-compatibility/2006";

/// EMU per pixel at Excel's 96 dpi (the Cargo feature `images`).
#[cfg(feature = "images")]
const EMU_PER_PIXEL: u64 = 9525;

/// The `hyperlink` relationship type, transitional then strict, the type a
/// link to a target outside the package is written with (the Cargo feature
/// `metadata`; `specs/0019-names-links-properties-protection.md`,
/// "Hyperlinks").
#[cfg(feature = "metadata")]
pub const HYPERLINK: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink",
    b"http://purl.oclc.org/ooxml/officeDocument/relationships/hyperlink",
];

/// The core-properties relationship type of the package: the package
/// form, which every corpus package writes, then the older
/// `officeDocument` form some producers write (the Cargo feature
/// `metadata`; `specs/0019-names-links-properties-protection.md`,
/// "Document properties").
#[cfg(feature = "metadata")]
pub const CORE_PROPERTIES: [&[u8]; 2] = [
    b"http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
    b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/metadata/core-properties",
];

/// The core-properties part's content type.
#[cfg(feature = "metadata")]
const CORE_PROPERTIES_TYPE: &[u8] = b"application/vnd.openxmlformats-package.core-properties+xml";

/// The namespaces of the core-properties part (ECMA-376 Part 2, 11).
#[cfg(feature = "metadata")]
const CP_NS: &[u8] = b"http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
#[cfg(feature = "metadata")]
const DC_NS: &[u8] = b"http://purl.org/dc/elements/1.1/";
#[cfg(feature = "metadata")]
const DCTERMS_NS: &[u8] = b"http://purl.org/dc/terms/";
#[cfg(feature = "metadata")]
const DCMITYPE_NS: &[u8] = b"http://purl.org/dc/dcmitype/";
#[cfg(feature = "metadata")]
const XSI_NS: &[u8] = b"http://www.w3.org/2001/XMLSchema-instance";

/// One of the thirteen core properties (ECMA-376 Part 2, 11), each mapped
/// to its element (`specs/0019-names-links-properties-protection.md`,
/// "Document properties"). The Cargo feature `metadata`.
#[cfg(feature = "metadata")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Property {
    /// `dc:title`.
    Title,
    /// `dc:subject`.
    Subject,
    /// `dc:creator`.
    Creator,
    /// `dc:description`.
    Description,
    /// `dc:language`.
    Language,
    /// `cp:keywords`.
    Keywords,
    /// `cp:lastModifiedBy`.
    LastModifiedBy,
    /// `cp:revision`.
    Revision,
    /// `cp:category`.
    Category,
    /// `cp:contentStatus`.
    ContentStatus,
    /// `cp:version`.
    Version,
    /// `dcterms:created`, written with `xsi:type="dcterms:W3CDTF"`.
    Created,
    /// `dcterms:modified`, written with `xsi:type="dcterms:W3CDTF"`.
    Modified,
}

#[cfg(feature = "metadata")]
impl Property {
    /// Every property, in the order above.
    const ALL: [Property; 13] = [
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

    /// The property's element: its namespace and local name.
    fn element(self) -> (&'static [u8], &'static [u8]) {
        match self {
            Property::Title => (DC_NS, b"title"),
            Property::Subject => (DC_NS, b"subject"),
            Property::Creator => (DC_NS, b"creator"),
            Property::Description => (DC_NS, b"description"),
            Property::Language => (DC_NS, b"language"),
            Property::Keywords => (CP_NS, b"keywords"),
            Property::LastModifiedBy => (CP_NS, b"lastModifiedBy"),
            Property::Revision => (CP_NS, b"revision"),
            Property::Category => (CP_NS, b"category"),
            Property::ContentStatus => (CP_NS, b"contentStatus"),
            Property::Version => (CP_NS, b"version"),
            Property::Created => (DCTERMS_NS, b"created"),
            Property::Modified => (DCTERMS_NS, b"modified"),
        }
    }

    /// Whether the property is a date written with `xsi:type`.
    fn dated(self) -> bool {
        matches!(self, Property::Created | Property::Modified)
    }

    /// The property whose element has this namespace and local name.
    fn of(namespace: Option<&[u8]>, local: &[u8]) -> Option<Property> {
        Property::ALL.into_iter().find(|property| {
            Some(property.element().0) == namespace && property.element().1 == local
        })
    }
}

/// The core properties a part holds, as [`Package::properties`] reads them:
/// a snapshot of the file, not of the pending edits. The Cargo feature
/// `metadata`.
#[cfg(feature = "metadata")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Properties {
    values: Vec<(Property, String)>,
}

#[cfg(feature = "metadata")]
impl Properties {
    /// The property's text, unescaped, or `None` when the part has no
    /// element for it.
    pub fn get(&self, property: Property) -> Option<&str> {
        self.values
            .iter()
            .find(|(have, _)| *have == property)
            .map(|(_, text)| text.as_str())
    }
}

/// A worksheet's protection options, the attributes of `<sheetProtection>`
/// (ECMA-376 Part 1, 18.3.1.85), each `true` when that action is locked
/// (`specs/0019-names-links-properties-protection.md`, "Protection"). The
/// default is the protected sheet with every option at the schema's own
/// default, which writes `<sheetProtection sheet="1"/>`: `objects`,
/// `scenarios`, `select_locked_cells` and `select_unlocked_cells` unlocked,
/// every other action locked. The Cargo feature `metadata`.
#[cfg(feature = "metadata")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct SheetProtection {
    pub sheet: bool,
    pub objects: bool,
    pub scenarios: bool,
    pub format_cells: bool,
    pub format_columns: bool,
    pub format_rows: bool,
    pub insert_columns: bool,
    pub insert_rows: bool,
    pub insert_hyperlinks: bool,
    pub delete_columns: bool,
    pub delete_rows: bool,
    pub select_locked_cells: bool,
    pub sort: bool,
    pub auto_filter: bool,
    pub pivot_tables: bool,
    pub select_unlocked_cells: bool,
}

#[cfg(feature = "metadata")]
impl std::default::Default for SheetProtection {
    fn default() -> Self {
        SheetProtection {
            sheet: true,
            objects: false,
            scenarios: false,
            format_cells: true,
            format_columns: true,
            format_rows: true,
            insert_columns: true,
            insert_rows: true,
            insert_hyperlinks: true,
            delete_columns: true,
            delete_rows: true,
            select_locked_cells: false,
            sort: true,
            auto_filter: true,
            pivot_tables: true,
            select_unlocked_cells: false,
        }
    }
}

#[cfg(feature = "metadata")]
impl SheetProtection {
    /// Each option's attribute name, its value here and its schema
    /// default, in the schema's order.
    fn options(&self) -> [(&'static [u8], bool, bool); 16] {
        [
            (b"sheet", self.sheet, false),
            (b"objects", self.objects, false),
            (b"scenarios", self.scenarios, false),
            (b"formatCells", self.format_cells, true),
            (b"formatColumns", self.format_columns, true),
            (b"formatRows", self.format_rows, true),
            (b"insertColumns", self.insert_columns, true),
            (b"insertRows", self.insert_rows, true),
            (b"insertHyperlinks", self.insert_hyperlinks, true),
            (b"deleteColumns", self.delete_columns, true),
            (b"deleteRows", self.delete_rows, true),
            (b"selectLockedCells", self.select_locked_cells, false),
            (b"sort", self.sort, true),
            (b"autoFilter", self.auto_filter, true),
            (b"pivotTables", self.pivot_tables, true),
            (b"selectUnlockedCells", self.select_unlocked_cells, false),
        ]
    }
}

/// A worksheet protection as recorded: the password's hash, when given,
/// and the options.
#[cfg(feature = "metadata")]
type SheetLock = (Option<u16>, SheetProtection);

/// A workbook protection as recorded: the password's hash, when given, and
/// the structure and windows locks.
#[cfg(feature = "metadata")]
type BookLock = (Option<u16>, bool, bool);

/// The legacy password hash Excel stores in `password` and
/// `workbookPassword` (ECMA-376 first edition Part 4; the algorithm
/// openpyxl and LibreOffice write): from zero, each character at 1-based
/// position `i` is rotated left by `i` within fifteen bits and exclusive-ored
/// in; then the length, then `0xCE4B`
/// (`specs/0019-names-links-properties-protection.md`, "Protection"). The
/// callers accept printable ASCII only, where each character is one byte.
/// The Cargo feature `metadata`.
#[cfg(feature = "metadata")]
pub fn legacy_password_hash(password: &str) -> u16 {
    let mut hash: u16 = 0;
    let mut length: u16 = 0;
    for (position, byte) in password.bytes().enumerate() {
        let shift = (position + 1) % 15;
        let value = u32::from(byte) & 0x7FFF;
        let rotated = ((value << shift) | (value >> (15 - shift))) & 0x7FFF;
        hash ^= rotated as u16;
        length = length.wrapping_add(1);
    }
    hash ^ length ^ 0xCE4B
}

/// Whether a password is 1 to 255 characters, every one printable ASCII.
#[cfg(feature = "metadata")]
fn password_ok(password: &str) -> bool {
    (1..=255).contains(&password.len())
        && password.bytes().all(|byte| (0x20..=0x7E).contains(&byte))
}

/// One `Default` element of `[Content_Types].xml`, bytes as written.
///
/// The name shadows the prelude's `Default` trait inside this module, which
/// is why nothing here names that trait. `specs/0009-package-layer.md` fixes
/// the type's name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Default {
    /// The `Extension` attribute, without a dot.
    pub extension: Vec<u8>,
    /// The `ContentType` attribute.
    pub content_type: Vec<u8>,
}

/// One `Override` element of `[Content_Types].xml`, bytes as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Override {
    /// The `PartName` attribute, its leading slash kept.
    pub part_name: Vec<u8>,
    /// The `ContentType` attribute.
    pub content_type: Vec<u8>,
}

/// A package's content types, in document order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentTypes {
    defaults: Vec<Default>,
    overrides: Vec<Override>,
}

impl ContentTypes {
    /// Every `Default`, in document order.
    pub fn defaults(&self) -> &[Default] {
        &self.defaults
    }

    /// Every `Override`, in document order.
    pub fn overrides(&self) -> &[Override] {
        &self.overrides
    }

    /// The content type of the entry `name`, or `None`.
    ///
    /// The `Override` whose part name is `/` followed by `name` byte for
    /// byte, else the `Default` whose extension equals the bytes after the
    /// name's last `.` compared ASCII case-insensitively (the convention
    /// makes an extension case-insensitive; the corpus writes them in lower
    /// case), else nothing.
    pub fn of(&self, name: &[u8]) -> Option<&[u8]> {
        let mut part_name = Vec::with_capacity(name.len() + 1);
        part_name.push(b'/');
        part_name.extend_from_slice(name);
        if let Some(over) = self
            .overrides
            .iter()
            .find(|over| over.part_name == part_name)
        {
            return Some(&over.content_type);
        }
        let dot = name.iter().rposition(|byte| *byte == b'.')?;
        let extension = &name[dot + 1..];
        self.defaults
            .iter()
            .find(|default| default.extension.eq_ignore_ascii_case(extension))
            .map(|default| default.content_type.as_slice())
    }
}

/// One `Relationship` element of a `.rels` part, bytes as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relationship {
    /// The `Id` attribute.
    pub id: Vec<u8>,
    /// The `Type` attribute, the full URI.
    pub kind: Vec<u8>,
    /// The `Target` attribute, as written: absolute, relative or external.
    pub target: Vec<u8>,
    /// Whether `TargetMode` is `External`, by exact bytes.
    pub external: bool,
}

/// The relationships of one part, or of the package itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relationships {
    source: Vec<u8>,
    list: Vec<Relationship>,
}

impl Relationships {
    /// The part these belong to, empty for the package's own.
    pub fn source(&self) -> &[u8] {
        &self.source
    }

    /// Every relationship, in document order.
    pub fn all(&self) -> &[Relationship] {
        &self.list
    }

    /// The relationship with that id, or `None`.
    pub fn by_id(&self, id: &[u8]) -> Option<&Relationship> {
        self.list.iter().find(|rel| rel.id == id)
    }

    /// Every relationship whose `Type` is exactly `kind`, in document order.
    ///
    /// The full URI, not a name: a caller that means one of the eight types
    /// this module knows passes one form of that constant, and tries the
    /// other form after it, as [`Relationships::first_of`] does.
    pub fn of_kind<'s>(&'s self, kind: &[u8]) -> impl Iterator<Item = &'s Relationship> {
        self.list.iter().filter(move |rel| rel.kind == kind)
    }

    /// The first relationship of either form of `kind`, transitional first.
    ///
    /// "The first in document order when there are several"
    /// (`specs/0009-package-layer.md`), and the transitional form before the
    /// strict one, as the namespace rule says.
    pub fn first_of(&self, kind: [&[u8]; 2]) -> Option<&Relationship> {
        kind.into_iter().find_map(|form| self.of_kind(form).next())
    }

    /// The entry name an internal relationship's target resolves to.
    ///
    /// The target without its leading slash when it has one, else the target
    /// joined to the source part's directory; `.` and `..` segments are
    /// resolved either way. The spec's letter resolves them for the relative
    /// case alone, which on this corpus is the same function: no absolute
    /// target carries a dot segment. Resolving both is the weaker assumption,
    /// since a target with `..` in it is not an entry name whichever form it
    /// takes.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "an external relationship has no part", or "a
    /// relationship target outside the package" when `..` climbs above the
    /// root or nothing is left.
    pub fn target_part(&self, rel: &Relationship) -> Result<Vec<u8>, Error> {
        if rel.external {
            return Err(Error::Package("an external relationship has no part"));
        }
        let outside = Error::Package("a relationship target outside the package");
        let mut segments: Vec<&[u8]> = Vec::new();
        let target = match rel.target.strip_prefix(b"/") {
            Some(absolute) => absolute,
            None => {
                // The source's directory, which is nothing when the source is
                // the package itself or an entry at the root.
                if let Some(slash) = self.source.iter().rposition(|byte| *byte == b'/') {
                    for segment in self.source[..slash].split(|byte| *byte == b'/') {
                        if !segment.is_empty() {
                            segments.push(segment);
                        }
                    }
                }
                &rel.target[..]
            }
        };
        for segment in target.split(|byte| *byte == b'/') {
            match segment {
                // An empty segment is a doubled or trailing slash, and `.` is
                // the directory itself: neither moves.
                b"" | b"." => {}
                b".." => {
                    if segments.pop().is_none() {
                        return Err(outside);
                    }
                }
                _ => segments.push(segment),
            }
        }
        if segments.is_empty() {
            return Err(outside);
        }
        let mut name = Vec::new();
        for (at, segment) in segments.iter().enumerate() {
            if at > 0 {
                name.push(b'/');
            }
            name.extend_from_slice(segment);
        }
        Ok(name)
    }
}

/// What a sheet's relationship says it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SheetKind {
    /// A `worksheet` relationship.
    Worksheet,
    /// A `chartsheet` relationship.
    Chartsheet,
    /// A `dialogsheet` relationship.
    Dialogsheet,
    /// Anything else, recorded rather than refused.
    Other,
}

/// One `<sheet>` of the workbook's `<sheets>`, in document order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sheet {
    /// The `name` attribute, unescaped and normalized.
    pub name: String,
    /// The `sheetId` attribute, a decimal `u32`.
    pub sheet_id: u32,
    /// The `r:id` attribute, bytes as written.
    pub rel_id: Vec<u8>,
    /// The `state` attribute, bytes as written, absent when not written.
    pub state: Option<Vec<u8>>,
    /// The entry name the sheet's relationship resolves to.
    pub part: Vec<u8>,
    /// The kind that relationship's `Type` says.
    pub kind: SheetKind,
}

/// The workbook part and what it points at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workbook {
    /// The workbook part's entry name.
    pub part: Vec<u8>,
    /// Its sheets, in `<sheets>` order.
    pub sheets: Vec<Sheet>,
    /// The resolved part of its first `sharedStrings` relationship.
    pub shared_strings: Option<Vec<u8>>,
    /// The resolved part of its first `styles` relationship.
    pub styles: Option<Vec<u8>>,
    /// The resolved part of its first `calcChain` relationship.
    pub calc_chain: Option<Vec<u8>>,
    /// The resolved part of its first `theme` relationship.
    pub theme: Option<Vec<u8>>,
    /// Whether the workbook counts dates from 1904: `<workbookPr>`'s
    /// `date1904` written as `1` or `true`, and `false` for any other value,
    /// when the attribute is absent and when `<workbookPr>` is
    /// (`specs/0012-styles.md`, "Module `dates`"). The first `<workbookPr>`
    /// among the root's children is the one read.
    pub date1904: bool,
    /// Whether the workbook asks for a full calculation on load: `<calcPr>`'s
    /// `fullCalcOnLoad` written as `1` or `true`, and `false` for any other
    /// value, when the attribute is absent and when `<calcPr>` is
    /// (`specs/0013-formulas.md`, "Module `package`"). The first `<calcPr>`
    /// among the root's children is the one read, as for `date1904`.
    pub full_calc_on_load: bool,
    /// The `<definedName>` elements of the root's `<definedNames>`, in
    /// document order (`specs/0019-names-links-properties-protection.md`,
    /// "Defined names"); [`Workbook::defined_names`] answers with them.
    #[cfg(feature = "metadata")]
    pub(crate) defined_names: Vec<DefinedName>,
    /// How many `<definedNames>` elements the root has; a write that edits
    /// a name refuses two.
    #[cfg(feature = "metadata")]
    pub(crate) defined_names_containers: usize,
    /// How many `<workbookProtection>` elements the root has; a write that
    /// protects or unprotects the workbook refuses two.
    #[cfg(feature = "metadata")]
    pub(crate) workbook_protections: usize,
}

impl Workbook {
    /// The sheet with that unescaped name, matched exactly.
    pub fn sheet(&self, name: &str) -> Option<&Sheet> {
        self.sheets.iter().find(|sheet| sheet.name == name)
    }

    /// The file's defined names, in document order: every `<definedName>`
    /// of the root's `<definedNames>`, read with the sheets. A snapshot of
    /// the file, not of the pending edits. The Cargo feature `metadata`
    /// (`specs/0019-names-links-properties-protection.md`, "Defined names").
    #[cfg(feature = "metadata")]
    pub fn defined_names(&self) -> &[DefinedName] {
        &self.defined_names
    }
}

/// One `<definedName>` of the workbook part, as [`Workbook::defined_names`]
/// reads it (`specs/0019-names-links-properties-protection.md`, "Defined
/// names"). The Cargo feature `metadata`.
#[cfg(feature = "metadata")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinedName {
    /// The `name` attribute, unescaped and normalized as a sheet name is.
    pub name: String,
    /// The element's content, unescaped, with XML's line-end normalisation:
    /// stored, never parsed or evaluated.
    pub text: String,
    /// The `localSheetId`, an index into the file's sheet list
    /// ([`Workbook::sheets`]), or `None` for a workbook-level name.
    pub sheet: Option<u32>,
    /// Whether `hidden` is written as `1` or `true`.
    pub hidden: bool,
}

/// A spreadsheet package: the archive, and the parts it names.
///
/// See the module header. Opening parses nothing; each accessor parses on its
/// first call and keeps the answer, so a package whose content types part is
/// malformed still opens.
pub struct Package<R: Read + Seek> {
    archive: Archive<R>,
    content_types: Option<ContentTypes>,
    /// Parsed relationships by source, the package's own under an empty name.
    relationships: Vec<Relationships>,
    workbook: Option<Workbook>,
    /// How many `<calcPr>` elements the workbook part has among its root's
    /// children, counted by the read that fills `workbook`; a write that
    /// rewrites the part refuses two (`specs/0013-formulas.md`, P1 ruling 3).
    calc_prs: usize,
    styles: Option<Styles>,
    /// Recorded edits, applied by [`Package::write_to`] and nowhere else.
    ///
    /// Nothing is removed implicitly: a removed relationship's target, a
    /// removed part's own rels part and its override are each their own call,
    /// so the caller says exactly what changes and the harness checks exactly
    /// that (`specs/0009-package-layer.md`, "Module `package`").
    edits: Recorded,
}

/// The edits a [`Package`] has been told to make, in the order they arrived.
#[derive(Default)]
struct Recorded {
    /// `(source, id)`: the relationship to drop from `source`'s rels part.
    rel_removals: Vec<(Vec<u8>, Vec<u8>)>,
    /// `(source, relationship)`: added at the end of `source`'s rels part.
    rel_additions: Vec<(Vec<u8>, Relationship)>,
    /// Part names, as the `PartName` attribute writes them.
    override_removals: Vec<Vec<u8>>,
    /// `(part_name, content_type)`, both as the attributes write them.
    override_additions: Vec<(Vec<u8>, Vec<u8>)>,
    part_removals: Vec<Vec<u8>>,
    part_replacements: Vec<(Vec<u8>, Replacement)>,
    part_additions: Vec<(Vec<u8>, Replacement)>,
    /// `(part, reference, value)`: a cell set, or cleared when the value is
    /// `None`, in the order recorded, so the last for a reference wins.
    cell_edits: Vec<(Vec<u8>, CellRef, Option<Value>)>,
    /// `(part, reference, style)`: a cell's `s` set, or removed when the
    /// style is `None`, in the order recorded, so the last for a reference
    /// wins, independently of its value edits (`specs/0012-styles.md`).
    style_edits: Vec<(Vec<u8>, CellRef, Option<u32>)>,
    /// The cell formats [`Package::add_cell_style`] recorded, in record
    /// order, each composed at that call.
    xf_additions: Vec<NewXf>,
    /// `(id, code)`: the number formats those additions need that the part
    /// does not define, in record order.
    num_fmt_additions: Vec<(u32, String)>,
    /// The distinct fonts those additions name, in record order, each beside
    /// its element; the id of the one at `i` is the part's font count plus
    /// `i` (`specs/0017-fuller-styles.md`, "Module `package`").
    font_additions: Vec<(Font, Composed)>,
    /// The distinct fills, as `font_additions`.
    fill_additions: Vec<(Fill, Composed)>,
    /// The distinct borders, as `font_additions`.
    border_additions: Vec<(Border, Composed)>,
    /// Whether the workbook part gets `fullCalcOnLoad="1"`: recorded by
    /// [`Package::recalculate_on_open`], or by the formula-change rule of
    /// [`Package::write_to`] (`specs/0013-formulas.md`).
    recalculate: bool,
    /// The sheet edits, in record order, each checked at its call against
    /// the list the ones before it project
    /// (`specs/0016-new-workbook-and-sheets.md`, "The projected sheet list").
    sheet_edits: Vec<SheetEdit>,
    /// `(part, column, field)`: a column's width or hidden flag, in record
    /// order, so the last for a column wins per field
    /// (`specs/0018-rows-and-columns.md`, "Module `package`: the calls").
    column_formats: Vec<(Vec<u8>, u32, LineField)>,
    /// `(part, row, field)`: a row's height or hidden flag, as
    /// `column_formats`.
    row_formats: Vec<(Vec<u8>, u32, LineField)>,
    /// `(part, at)`: a freeze at `Some(cell)` or an unfreeze, the last for
    /// a part winning.
    panes: Vec<(Vec<u8>, Option<CellRef>)>,
    /// `(part, area)`: a merge, normalised, in record order.
    merges: Vec<(Vec<u8>, Area)>,
    /// `(part, area)`: an unmerge, normalised, in record order.
    unmerges: Vec<(Vec<u8>, Area)>,
    /// `(part, shift)`: an insertion or deletion, in record order; a write
    /// takes one per part and nothing else for it (decision 5).
    #[cfg(feature = "rowcol")]
    shifts: Vec<(Vec<u8>, AxisShift)>,
    /// The defined names added and removed, in record order
    /// (`specs/0019-names-links-properties-protection.md`, "Defined names").
    #[cfg(feature = "metadata")]
    name_edits: Vec<NameEdit>,
    /// `(part, cell, link)`: a hyperlink set, or removed when `None`, in
    /// record order, so the last for a cell wins
    /// (`specs/0019-names-links-properties-protection.md`, "Hyperlinks").
    #[cfg(feature = "metadata")]
    links: Vec<(Vec<u8>, CellRef, Option<LinkSet>)>,
    /// `(part, cell, id)`: the relationship the write recorded for each
    /// external link, filled by [`Package::write_to`] before it validates.
    #[cfg(feature = "metadata")]
    link_ids: Vec<(Vec<u8>, CellRef, Vec<u8>)>,
    /// `(part, strict)`: for each part with a link edit, whether the
    /// worksheet's own relationship in the workbook's rels part is of the
    /// strict form, which the link's relationship type and a retag's
    /// declaration both follow (P2 verification ruling 2).
    #[cfg(feature = "metadata")]
    link_strict: Vec<(Vec<u8>, bool)>,
    /// `(property, value)`: a core property set, or removed when `None`, in
    /// record order, so the last for a property wins
    /// (`specs/0019-names-links-properties-protection.md`, "Document
    /// properties").
    #[cfg(feature = "metadata")]
    properties: Vec<(Property, Option<String>)>,
    /// `(part, protection)`: a worksheet protected with the hash and
    /// options, or unprotected when `None`, the last for a part winning
    /// ("Protection").
    #[cfg(feature = "metadata")]
    sheet_protections: Vec<(Vec<u8>, Option<SheetLock>)>,
    /// The workbook protected with the hash and the structure and windows
    /// locks, or unprotected when `None`, the last winning.
    #[cfg(feature = "metadata")]
    book_protections: Vec<Option<BookLock>>,
    /// `(part, edit)`: a data validation added or removed, in record order
    /// (`specs/0020-validation-and-conditional-formatting.md`, "Data
    /// validation").
    #[cfg(feature = "rules")]
    validations: Vec<(Vec<u8>, ValidationEdit)>,
    /// The differential styles [`Package::add_differential_style`] recorded,
    /// in record order, each composed at its call
    /// (`specs/0020-validation-and-conditional-formatting.md`,
    /// "Differential styles").
    #[cfg(feature = "rules")]
    dxf_additions: Vec<Composed>,
    /// The styles part's `dxf` children, read once per package by the first
    /// call or write that needs them (P2 ruling 3: the index a `dxfId`
    /// names is a position).
    #[cfg(feature = "rules")]
    dxf_children: Option<u64>,
    /// `(part, edit)`: a conditional format added or removed, in record
    /// order ("Conditional formatting").
    #[cfg(feature = "rules")]
    rules: Vec<(Vec<u8>, RuleEdit)>,
    /// `(part, table)`: a table added, in record order
    /// (`specs/0021-tables.md`, "The calls").
    #[cfg(feature = "tables")]
    table_additions: Vec<(Vec<u8>, NewTable)>,
    /// `(part, table, strict)`: the area and relationship id the write
    /// recorded for each table addition, and whether the part takes the
    /// strict forms, filled by [`Package::write_to`] before it validates.
    #[cfg(feature = "tables")]
    table_works: Vec<(Vec<u8>, TableWork, bool)>,
    /// `(part, name)`: a table removed, in record order (`specs/0021-tables.md`,
    /// "The calls", `remove_table`).
    #[cfg(feature = "tables")]
    table_removals: Vec<(Vec<u8>, String)>,
    /// `(part, id)`: the relationship of each table the write removes, whose
    /// `<tablePart>` the worksheet pass skips, filled by
    /// [`Package::write_to`] before it validates.
    #[cfg(feature = "tables")]
    table_removed_ids: Vec<(Vec<u8>, Vec<u8>)>,
    /// `(part, cell, comment)`: a comment added, or removed when `None`, in
    /// record order (`specs/0022-comments-and-images.md`, "Comments: the
    /// calls").
    #[cfg(feature = "comments")]
    comment_edits: Vec<(Vec<u8>, CellRef, Option<NewComment>)>,
    /// `(part, work)`: for each part whose `<legacyDrawing>` the write
    /// creates or drops, what the worksheet pass does, filled by
    /// [`Package::write_to`] before it validates.
    #[cfg(feature = "comments")]
    comment_works: Vec<(Vec<u8>, LegacyWork)>,
    /// `(part, cell, image)`: an image added, in record order
    /// (`specs/0022-comments-and-images.md`, "Images: the calls").
    #[cfg(feature = "images")]
    image_edits: Vec<(Vec<u8>, CellRef, NewImage)>,
    /// `(part, work)`: for each part whose `<drawing>` the write creates,
    /// what the worksheet pass does, filled by [`Package::write_to`] before
    /// it validates.
    #[cfg(feature = "images")]
    image_works: Vec<(Vec<u8>, DrawingWork)>,
    /// `(part, colour)`: a tab colour set, or dropped when `None`, the last
    /// for a part winning (`specs/0025-sheet-and-print-settings.md`, "The
    /// calls").
    #[cfg(feature = "sheetprops")]
    tab_colors: Vec<(Vec<u8>, Option<Color>)>,
    /// `(part, view)`: a sheet view's settings, in record order, so the
    /// last for a field wins ("The calls").
    #[cfg(feature = "sheetprops")]
    sheet_views: Vec<(Vec<u8>, SheetView)>,
}

/// One recorded change to the workbook's defined names. A scope is the
/// entry name of the sheet the call named, so it follows that sheet through
/// a rename, a move or a removal in the same write; `None` is the workbook.
#[cfg(feature = "metadata")]
#[derive(Clone)]
enum NameEdit {
    /// A name, its text and its scope, written at the end of `<definedNames>`.
    Add {
        name: String,
        text: String,
        sheet: Option<Vec<u8>>,
    },
    /// The file's names of that name, ignoring case, and that scope, dropped.
    Remove {
        name: String,
        sheet: Option<Vec<u8>>,
    },
}

/// One field of a column's or a row's format as a call records it.
#[derive(Clone, Copy)]
enum LineField {
    /// A width or a height, `None` to remove it.
    Size(Option<f64>),
    Hidden(bool),
}

/// A column's or a row's format edits in one write, each field the last
/// recorded: `Some(Some(size))` a size, `Some(None)` its removal, `None` no
/// size edit; `Some(flag)` the hidden flag set or removed.
#[derive(Clone, Copy, Default)]
struct LineFormat {
    size: Option<Option<f64>>,
    hidden: Option<bool>,
}

impl LineFormat {
    /// Whether the format writes an attribute where there is no element:
    /// a size, or the hidden flag set.
    fn writes_when_absent(&self) -> bool {
        matches!(self.size, Some(Some(_))) || self.hidden == Some(true)
    }
}

/// What one worksheet's rewrite applies: its cell edits, sorted, and the
/// formats, pane and merges recorded for it.
#[derive(Default)]
struct SheetWork {
    cells: Vec<(CellRef, CellEdit)>,
    /// Row formats by row, ascending.
    rows: Vec<(u32, LineFormat)>,
    /// Column formats by column, ascending.
    columns: Vec<(u32, LineFormat)>,
    /// `Some(at)` when a freeze or an unfreeze is recorded.
    pane: Option<Option<CellRef>>,
    merges: Vec<Area>,
    unmerges: Vec<Area>,
    /// The hyperlinks recorded for the part, each cell's last record, in
    /// record order (`specs/0019-names-links-properties-protection.md`,
    /// "Hyperlinks").
    #[cfg(feature = "metadata")]
    links: Vec<(CellRef, LinkEdit)>,
    /// The `r:id`s of the links the rewrite drops, filled by the pass, so the
    /// write removes their relationships.
    #[cfg(feature = "metadata")]
    dropped_ids: std::cell::RefCell<Vec<Vec<u8>>>,
    /// Whether the part's links take the strict forms (the relationship
    /// type and a retag's namespace).
    #[cfg(feature = "metadata")]
    links_strict: bool,
    /// The part's protection edit, the last recorded: `Some(Some(..))` a
    /// protect, `Some(None)` an unprotect.
    #[cfg(feature = "metadata")]
    protection: Option<Option<SheetLock>>,
    /// The data validation edits recorded for the part, in record order.
    #[cfg(feature = "rules")]
    validations: Vec<ValidationEdit>,
    /// The conditional format edits recorded for the part, in record order.
    #[cfg(feature = "rules")]
    rules: Vec<RuleEdit>,
    /// The tables the write adds to the part, in record order
    /// (`specs/0021-tables.md`, "The worksheet").
    #[cfg(feature = "tables")]
    tables: Vec<TableWork>,
    /// Whether the part's table relationships take the strict forms (the
    /// relationship type and a retag's namespace; P1 ruling 3).
    #[cfg(feature = "tables")]
    tables_strict: bool,
    /// The relationship ids of the tables the write removes from the part.
    #[cfg(feature = "tables")]
    tables_removed: Vec<Vec<u8>>,
    /// The id of the VML part's relationship when the write creates the
    /// part's `<legacyDrawing>` (`specs/0022-comments-and-images.md`,
    /// "Comments: the parts").
    #[cfg(feature = "comments")]
    legacy: Option<Vec<u8>>,
    /// Whether that relationship takes the strict forms (a retag's
    /// namespace).
    #[cfg(feature = "comments")]
    legacy_strict: bool,
    /// Whether the write drops the part's `<legacyDrawing>`, its VML part
    /// gone with the last comment.
    #[cfg(feature = "comments")]
    legacy_drop: bool,
    /// The id of the drawing part's relationship when the write creates the
    /// part's `<drawing>` (`specs/0022-comments-and-images.md`, "Images: the
    /// parts").
    #[cfg(feature = "images")]
    drawing: Option<Vec<u8>>,
    /// Whether that relationship takes the strict forms (a retag's
    /// namespace).
    #[cfg(feature = "images")]
    drawing_strict: bool,
    /// The part's sheet settings (`specs/0025-sheet-and-print-settings.md`,
    /// "The pass").
    #[cfg(feature = "sheetprops")]
    settings: Settings,
}

/// A hyperlink's target as a caller gives it
/// (`specs/0019-names-links-properties-protection.md`, "Hyperlinks"). The
/// Cargo feature `metadata`.
#[cfg(feature = "metadata")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkTarget<'a> {
    /// A URL, a file path or a `mailto:` address outside the package,
    /// written as a relationship with `TargetMode="External"`.
    External(&'a str),
    /// A place in the workbook, `'Sheet 1'!A1` or a defined name, written as
    /// the `location` attribute.
    Location(&'a str),
}

/// One recorded hyperlink edit: a link to set, or `None` in
/// [`Recorded`]'s list for a removal.
#[cfg(feature = "metadata")]
#[derive(Clone)]
struct LinkSet {
    /// `true` for an external target, whose text is the relationship's.
    external: bool,
    /// The target or the location.
    text: String,
    tooltip: Option<String>,
}

/// What the worksheet pass does for one cell's hyperlink.
#[cfg(feature = "metadata")]
#[derive(Clone)]
enum LinkEdit {
    /// Write this link, `rel_id` the relationship an external target was
    /// given at the write.
    Set {
        set: LinkSet,
        rel_id: Option<Vec<u8>>,
    },
    /// Drop the link the part has on the cell.
    Remove,
}

/// The kind of a data validation, the values of ECMA-376 Part 1, 18.18.20
/// other than `none`, which is an input message alone and is read back as
/// no kind, never written (`specs/0020-validation-and-conditional-
/// formatting.md`, "Data validation"). The Cargo feature `rules`.
#[cfg(feature = "rules")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationKind {
    /// A whole number, `whole`.
    Whole,
    /// A decimal number, `decimal`.
    Decimal,
    /// A value from a list, `list`.
    List,
    /// A date, `date`.
    Date,
    /// A time, `time`.
    Time,
    /// A text length, `textLength`.
    TextLength,
    /// A formula that must be true, `custom`.
    Custom,
}

#[cfg(feature = "rules")]
impl ValidationKind {
    const ALL: [(ValidationKind, &'static [u8]); 7] = [
        (ValidationKind::Whole, b"whole"),
        (ValidationKind::Decimal, b"decimal"),
        (ValidationKind::List, b"list"),
        (ValidationKind::Date, b"date"),
        (ValidationKind::Time, b"time"),
        (ValidationKind::TextLength, b"textLength"),
        (ValidationKind::Custom, b"custom"),
    ];

    /// The `type` attribute's value.
    fn name(self) -> &'static [u8] {
        Self::ALL
            .iter()
            .find(|(kind, _)| *kind == self)
            .map_or(b"", |(_, name)| name)
    }

    /// Whether the kind compares a value with an operator: every kind but
    /// `List` and `Custom`.
    fn takes_operator(self) -> bool {
        !matches!(self, ValidationKind::List | ValidationKind::Custom)
    }
}

/// A comparison, the values of ECMA-376 Part 1, 18.18.21, which name
/// 18.18.15's too (`specs/0020-validation-and-conditional-formatting.md`,
/// "Data validation"). The Cargo feature `rules`.
#[cfg(feature = "rules")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// Between two values, both included, `between`.
    Between,
    /// Outside two values, `notBetween`.
    NotBetween,
    /// `equal`.
    Equal,
    /// `notEqual`.
    NotEqual,
    /// `greaterThan`.
    GreaterThan,
    /// `lessThan`.
    LessThan,
    /// `greaterThanOrEqual`.
    GreaterThanOrEqual,
    /// `lessThanOrEqual`.
    LessThanOrEqual,
}

#[cfg(feature = "rules")]
impl Operator {
    const ALL: [(Operator, &'static [u8]); 8] = [
        (Operator::Between, b"between"),
        (Operator::NotBetween, b"notBetween"),
        (Operator::Equal, b"equal"),
        (Operator::NotEqual, b"notEqual"),
        (Operator::GreaterThan, b"greaterThan"),
        (Operator::LessThan, b"lessThan"),
        (Operator::GreaterThanOrEqual, b"greaterThanOrEqual"),
        (Operator::LessThanOrEqual, b"lessThanOrEqual"),
    ];

    /// The `operator` attribute's value.
    fn name(self) -> &'static [u8] {
        Self::ALL
            .iter()
            .find(|(operator, _)| *operator == self)
            .map_or(b"", |(_, name)| name)
    }

    /// Whether the operator compares with two values.
    fn two_values(self) -> bool {
        matches!(self, Operator::Between | Operator::NotBetween)
    }
}

/// What a data validation's error message does, ECMA-376 Part 1, 18.18.18.
/// The Cargo feature `rules`.
#[cfg(feature = "rules")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorStyle {
    /// The entry is refused, `stop`.
    Stop,
    /// The user is warned and may keep the entry, `warning`.
    Warning,
    /// The user is told and the entry is kept, `information`.
    Information,
}

#[cfg(feature = "rules")]
impl ErrorStyle {
    const ALL: [(ErrorStyle, &'static [u8]); 3] = [
        (ErrorStyle::Stop, b"stop"),
        (ErrorStyle::Warning, b"warning"),
        (ErrorStyle::Information, b"information"),
    ];

    /// The `errorStyle` attribute's value.
    fn name(self) -> &'static [u8] {
        Self::ALL
            .iter()
            .find(|(style, _)| *style == self)
            .map_or(b"", |(_, name)| name)
    }
}

/// The value of an enumeration's attribute, `None` for a value the
/// enumeration does not name.
#[cfg(feature = "rules")]
fn named<T: Copy>(all: &[(T, &'static [u8])], value: &[u8]) -> Option<T> {
    all.iter()
        .find(|(_, name)| *name == value)
        .map(|(item, _)| *item)
}

/// A data validation as a caller gives it to [`Package::add_validation`]
/// (`specs/0020-validation-and-conditional-formatting.md`, "Data
/// validation", and the P1 rulings). The Cargo feature `rules`.
///
/// A formula is text, given without a leading `=`, never parsed and never
/// evaluated; a `List`'s `formula1` is a range or a quoted list of
/// constants (`"a,b,c"`), which the crate does not tell apart.
#[cfg(feature = "rules")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Validation<'a> {
    /// What the cell's value must be.
    pub kind: ValidationKind,
    /// The comparison, for a kind other than `List` and `Custom`; `None`
    /// writes no `operator`, which reads as `Between`.
    pub operator: Option<Operator>,
    /// The first formula, or the only one.
    pub formula1: &'a str,
    /// The second formula, given with `Between` and `NotBetween` (or no
    /// operator) and with no other.
    pub formula2: Option<&'a str>,
    /// Whether an empty cell passes, `allowBlank`.
    pub allow_blank: bool,
    /// Whether a `List`'s drop-down arrow is HIDDEN: written as
    /// `showDropDown="1"`, the attribute's name saying the opposite of what
    /// it does.
    pub hide_drop_down: bool,
    /// The input message's title and text, shown when the cell is
    /// selected.
    pub input: Option<(&'a str, &'a str)>,
    /// The error message's style, title and text, shown when an entry
    /// fails.
    pub error: Option<(ErrorStyle, &'a str, &'a str)>,
}

/// A data validation as [`Package::validations`] reads it from a worksheet
/// part (`specs/0020-validation-and-conditional-formatting.md`, "Data
/// validation"). The Cargo feature `rules`.
#[cfg(feature = "rules")]
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationEntry {
    /// The `sqref` list, in its order, each area normalised.
    pub areas: Vec<Area>,
    /// `None` for a `type` of `none` or none at all: an input message
    /// alone.
    pub kind: Option<ValidationKind>,
    /// `None` when the element writes no `operator`.
    pub operator: Option<Operator>,
    /// The `formula1` child's text.
    pub formula1: Option<String>,
    /// The `formula2` child's text.
    pub formula2: Option<String>,
    /// `allowBlank`.
    pub allow_blank: bool,
    /// `showDropDown`, which hides the arrow.
    pub hide_drop_down: bool,
    /// Present when `showInputMessage` is `1` or `true`: `promptTitle` and
    /// `prompt`, each empty when absent.
    pub input: Option<(String, String)>,
    /// Present when `showErrorMessage` is `1` or `true`: `errorStyle`
    /// (`Stop` when absent), `errorTitle` and `error`, each empty when
    /// absent.
    pub error: Option<(ErrorStyle, String, String)>,
}

/// One recorded data validation edit: an addition, composed at the call,
/// or the removal of the file's validations on a list of areas.
#[cfg(feature = "rules")]
#[derive(Clone)]
enum ValidationEdit {
    Add(NewValidation),
    Remove(Vec<Area>),
}

/// A data validation to append: its areas, its start tag's attributes
/// before `sqref` (escaped, each with its leading space) and its children's
/// names and escaped texts.
#[cfg(feature = "rules")]
#[derive(Clone)]
struct NewValidation {
    areas: Vec<Area>,
    attributes: Vec<u8>,
    formulas: Vec<(&'static [u8], String)>,
}

/// A conditional format's rule as a caller gives it to
/// [`Package::add_conditional_format`], one variant per `type` of ECMA-376
/// Part 1, 18.18.12 this part writes
/// (`specs/0020-validation-and-conditional-formatting.md`, "Conditional
/// formatting"). The first eight take a differential style, the three
/// visual ones none. A formula is text, given without a leading `=`, never
/// parsed and never evaluated. The Cargo feature `rules`.
#[cfg(feature = "rules")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rule<'a> {
    /// `cellIs`: the cell's value compared with one formula, or two for
    /// `Between` and `NotBetween`.
    CellIs {
        /// The comparison.
        operator: Operator,
        /// The first formula, or the only one.
        formula1: &'a str,
        /// The second formula, with `Between` and `NotBetween` alone.
        formula2: Option<&'a str>,
    },
    /// `expression`: a formula that is true for the cells it styles.
    Expression(&'a str),
    /// `containsBlanks`, written with Excel's formula for the first area's
    /// top-left cell.
    ContainsBlanks,
    /// `notContainsBlanks`, likewise.
    NotContainsBlanks,
    /// `duplicateValues`.
    DuplicateValues,
    /// `uniqueValues`.
    UniqueValues,
    /// `top10`: the top or bottom `rank` values, or percent.
    Top {
        /// 1 to 1000.
        rank: u32,
        /// A percentage of the cells rather than a count.
        percent: bool,
        /// The bottom values rather than the top.
        bottom: bool,
    },
    /// `aboveAverage`: above (or below) the average, the average itself
    /// included when `equal`.
    AboveAverage {
        /// Above rather than below.
        above: bool,
        /// The average itself included.
        equal: bool,
    },
    /// `colorScale`: two or three points, each with its colour; the first
    /// may be `Min` and the last `Max`. A visual rule: no style.
    ColorScale {
        /// The first point.
        min: Cfvo<'a>,
        /// A middle point, for a three-colour scale.
        mid: Option<Cfvo<'a>>,
        /// The last point.
        max: Cfvo<'a>,
    },
    /// `dataBar` in the 2007 form, with no `x14` extension: a solid bar of
    /// one colour. A visual rule: no style.
    DataBar {
        /// The bar's lowest point.
        min: CfvoKind<'a>,
        /// The bar's highest point.
        max: CfvoKind<'a>,
        /// The bar's colour.
        color: Color,
        /// Whether the cell's value is shown beside the bar.
        show_value: bool,
    },
    /// `iconSet`: its thresholds number as the set's icons, the first
    /// `Percent(0)` by Excel's convention. A visual rule: no style.
    IconSet {
        /// The set.
        set: IconSetKind,
        /// The thresholds, one per icon.
        thresholds: &'a [CfvoKind<'a>],
        /// Whether the icons are in reverse order.
        reverse: bool,
        /// Whether the cell's value is shown beside the icon.
        show_value: bool,
    },
}

/// One `<cfRule>` as [`Package::conditional_formats`] reads it, with the
/// areas of the `<conditionalFormatting>` holding it
/// (`specs/0020-validation-and-conditional-formatting.md`, "Conditional
/// formatting"). The Cargo feature `rules`.
#[cfg(feature = "rules")]
#[derive(Debug, Clone, PartialEq)]
pub struct RuleEntry {
    /// The holding element's `sqref` list, in its order, each area
    /// normalised.
    pub areas: Vec<Area>,
    /// The `type` attribute's text, a kind this part does not write
    /// included.
    pub kind: String,
    /// `operator`, when the rule writes one that [`Operator`] names; `None`
    /// for a text rule's `containsText`, `notContains`, `beginsWith` or
    /// `endsWith` (P2 ruling 6).
    pub operator: Option<Operator>,
    /// The `formula` children's texts, in order.
    pub formulas: Vec<String>,
    /// `dxfId`.
    pub dxf: Option<u32>,
    /// `priority`.
    pub priority: u32,
    /// `stopIfTrue`.
    pub stop_if_true: bool,
    /// Whether the rule holds an `extLst`, an extension this part keeps
    /// and never removes.
    pub extended: bool,
}

/// A value point of a colour scale, a data bar or an icon set, ECMA-376
/// Part 1, 18.3.1.11 and 18.18.13
/// (`specs/0020-validation-and-conditional-formatting.md`, "Conditional
/// formatting"). The Cargo feature `rules`.
#[cfg(feature = "rules")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CfvoKind<'a> {
    /// The lowest value in the area, `min`: a scale's or a bar's first
    /// point only.
    Min,
    /// The highest value, `max`: a scale's or a bar's last point only.
    Max,
    /// A number, `num`.
    Num(f64),
    /// A percentage of the range, 0 to 100, `percent`.
    Percent(f64),
    /// A percentile, 0 to 100, `percentile`.
    Percentile(f64),
    /// A formula's value, `formula`, given without a leading `=`.
    Formula(&'a str),
}

/// A colour scale's point: its value and its colour. The Cargo feature
/// `rules`.
#[cfg(feature = "rules")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cfvo<'a> {
    /// Where the point is.
    pub kind: CfvoKind<'a>,
    /// Its colour, written as a font's colour is.
    pub color: Color,
}

/// An icon set, the 17 values of ECMA-376 Part 1, 18.18.42. The Cargo
/// feature `rules`.
#[cfg(feature = "rules")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSetKind {
    /// `3Arrows`.
    ThreeArrows,
    /// `3ArrowsGray`.
    ThreeArrowsGray,
    /// `3Flags`.
    ThreeFlags,
    /// `3TrafficLights1`.
    ThreeTrafficLights1,
    /// `3TrafficLights2`.
    ThreeTrafficLights2,
    /// `3Signs`.
    ThreeSigns,
    /// `3Symbols`.
    ThreeSymbols,
    /// `3Symbols2`.
    ThreeSymbols2,
    /// `4Arrows`.
    FourArrows,
    /// `4ArrowsGray`.
    FourArrowsGray,
    /// `4RedToBlack`.
    FourRedToBlack,
    /// `4Rating`.
    FourRating,
    /// `4TrafficLights`.
    FourTrafficLights,
    /// `5Arrows`.
    FiveArrows,
    /// `5ArrowsGray`.
    FiveArrowsGray,
    /// `5Rating`.
    FiveRating,
    /// `5Quarters`.
    FiveQuarters,
}

#[cfg(feature = "rules")]
impl IconSetKind {
    /// The `iconSet` attribute's value.
    fn name(self) -> &'static [u8] {
        match self {
            IconSetKind::ThreeArrows => b"3Arrows",
            IconSetKind::ThreeArrowsGray => b"3ArrowsGray",
            IconSetKind::ThreeFlags => b"3Flags",
            IconSetKind::ThreeTrafficLights1 => b"3TrafficLights1",
            IconSetKind::ThreeTrafficLights2 => b"3TrafficLights2",
            IconSetKind::ThreeSigns => b"3Signs",
            IconSetKind::ThreeSymbols => b"3Symbols",
            IconSetKind::ThreeSymbols2 => b"3Symbols2",
            IconSetKind::FourArrows => b"4Arrows",
            IconSetKind::FourArrowsGray => b"4ArrowsGray",
            IconSetKind::FourRedToBlack => b"4RedToBlack",
            IconSetKind::FourRating => b"4Rating",
            IconSetKind::FourTrafficLights => b"4TrafficLights",
            IconSetKind::FiveArrows => b"5Arrows",
            IconSetKind::FiveArrowsGray => b"5ArrowsGray",
            IconSetKind::FiveRating => b"5Rating",
            IconSetKind::FiveQuarters => b"5Quarters",
        }
    }

    /// How many icons the set has, which its thresholds must number: the
    /// digit its name starts with.
    fn icons(self) -> usize {
        usize::from(self.name()[0] - b'0')
    }
}

/// A visual rule's child element, composed at the call but for the prefix:
/// its name, its attributes (escaped, each with its leading space), its
/// `cfvo` types and values, and its colours.
#[cfg(feature = "rules")]
#[derive(Clone)]
struct VisualChild {
    name: &'static [u8],
    attributes: Vec<u8>,
    cfvos: Vec<(&'static [u8], Option<Vec<u8>>)>,
    colors: Vec<Color>,
}

#[cfg(feature = "rules")]
impl VisualChild {
    /// The element with `prefix` on every name.
    fn element(&self, prefix: &[u8]) -> Vec<u8> {
        let mut out = [b"<".as_slice(), prefix, self.name].concat();
        out.extend_from_slice(&self.attributes);
        out.push(b'>');
        for (kind, value) in &self.cfvos {
            out.extend_from_slice(
                &[b"<".as_slice(), prefix, b"cfvo type=\"", kind, b"\""].concat(),
            );
            if let Some(value) = value {
                out.extend_from_slice(&[b" val=\"".as_slice(), value, b"\""].concat());
            }
            out.extend_from_slice(b"/>");
        }
        // The styles writer composes the unprefixed element; the prefix is
        // spliced on as bytes (P3 verification ruling 2).
        for color in &self.colors {
            out.extend_from_slice(&color_element("color", color).element(prefix));
        }
        out.extend_from_slice(&[b"</".as_slice(), prefix, self.name, b">"].concat());
        out
    }
}

/// A value point's `type` and `val`, refused when it is not valid where it
/// stands (`specs/0020-validation-and-conditional-formatting.md`, the value
/// rule): `Min` only first and `Max` only last, and neither in an icon set;
/// a percent or percentile 0 to 100; a finite number; a formula not empty,
/// each with the value message. A formula with a leading `=` or a character
/// XML forbids meets part 4's messages instead, as every formula does (P3
/// ruling 2).
#[cfg(feature = "rules")]
fn cfvo_parts(
    kind: CfvoKind<'_>,
    first: bool,
    last: bool,
) -> Result<(&'static [u8], Option<Vec<u8>>), Error> {
    let number = |value: f64| Some(crate::cells::number_text(value).into_bytes());
    Ok(match kind {
        CfvoKind::Min if first => (b"min", None),
        CfvoKind::Max if last => (b"max", None),
        CfvoKind::Min | CfvoKind::Max => return Err(VISUAL_VALUE),
        CfvoKind::Num(value) if value.is_finite() => (b"num", number(value)),
        CfvoKind::Percent(value) if (0.0..=100.0).contains(&value) => (b"percent", number(value)),
        CfvoKind::Percentile(value) if (0.0..=100.0).contains(&value) => {
            (b"percentile", number(value))
        }
        CfvoKind::Formula(text) if !text.is_empty() => {
            rule_formula_ok(text)?;
            (b"formula", Some(attribute_escaped(text)))
        }
        _ => return Err(VISUAL_VALUE),
    })
}

/// Refuses a colour a visual rule cannot write: a theme tint that is not
/// finite or lies outside `-1.0..=1.0`.
#[cfg(feature = "rules")]
fn visual_color(color: Color) -> Result<Color, Error> {
    match color {
        Color::Theme {
            tint: Some(tint), ..
        } if !(-1.0..=1.0).contains(&tint) => Err(VISUAL_VALUE),
        color => Ok(color),
    }
}

/// A visual rule's `type` and child element, or `None` for a dxf rule.
#[cfg(feature = "rules")]
fn visual_child(rule: &Rule<'_>) -> Result<Option<(&'static [u8], VisualChild)>, Error> {
    let show = |show_value: bool| -> Vec<u8> {
        if show_value {
            Vec::new()
        } else {
            b" showValue=\"0\"".to_vec()
        }
    };
    Ok(Some(match *rule {
        Rule::ColorScale { min, mid, max } => {
            let points: Vec<Cfvo<'_>> = [Some(min), mid, Some(max)].into_iter().flatten().collect();
            let last = points.len() - 1;
            let mut cfvos = Vec::new();
            let mut colors = Vec::new();
            for (at, point) in points.iter().enumerate() {
                cfvos.push(cfvo_parts(point.kind, at == 0, at == last)?);
                colors.push(visual_color(point.color)?);
            }
            (
                b"colorScale",
                VisualChild {
                    name: b"colorScale",
                    attributes: Vec::new(),
                    cfvos,
                    colors,
                },
            )
        }
        Rule::DataBar {
            min,
            max,
            color,
            show_value,
        } => (
            b"dataBar",
            VisualChild {
                name: b"dataBar",
                attributes: show(show_value),
                cfvos: vec![cfvo_parts(min, true, false)?, cfvo_parts(max, false, true)?],
                colors: vec![visual_color(color)?],
            },
        ),
        Rule::IconSet {
            set,
            thresholds,
            reverse,
            show_value,
        } => {
            if thresholds.len() != set.icons() {
                return Err(VISUAL_VALUE);
            }
            let mut attributes = [b" iconSet=\"".as_slice(), set.name(), b"\""].concat();
            if reverse {
                attributes.extend_from_slice(b" reverse=\"1\"");
            }
            attributes.extend_from_slice(&show(show_value));
            let cfvos = thresholds
                .iter()
                .map(|kind| cfvo_parts(*kind, false, false))
                .collect::<Result<_, _>>()?;
            (
                b"iconSet",
                VisualChild {
                    name: b"iconSet",
                    attributes,
                    cfvos,
                    colors: Vec::new(),
                },
            )
        }
        _ => return Ok(None),
    }))
}

/// One recorded conditional format edit: an addition, composed at the call
/// but for its priority, or the removal of the file's conditional formats
/// on a list of areas.
#[cfg(feature = "rules")]
#[derive(Clone)]
enum RuleEdit {
    Add(NewRule),
    Remove(Vec<Area>),
}

/// A conditional format to append: its areas, its style, the priority
/// given, its rule's attributes before `priority` and after it (escaped,
/// each with its leading space), and the `formula` children's escaped
/// texts.
#[cfg(feature = "rules")]
#[derive(Clone)]
struct NewRule {
    areas: Vec<Area>,
    dxf: Option<u32>,
    priority: Option<u32>,
    before: Vec<u8>,
    after: Vec<u8>,
    formulas: Vec<String>,
    /// A visual rule's child element.
    visual: Option<VisualChild>,
}

/// One of Excel's built-in banded table styles, written as
/// `TableStyleLight7`, `TableStyleMedium2` or `TableStyleDark1`
/// (`specs/0021-tables.md`, "Types"): `Light` 1 to 21, `Medium` 1 to 28,
/// `Dark` 1 to 11. The Cargo feature `tables`.
#[cfg(feature = "tables")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableStyle {
    /// `TableStyleLight1` to `TableStyleLight21`.
    Light(u8),
    /// `TableStyleMedium1` to `TableStyleMedium28`.
    Medium(u8),
    /// `TableStyleDark1` to `TableStyleDark11`.
    Dark(u8),
}

#[cfg(feature = "tables")]
impl TableStyle {
    /// The style's name, `None` for a number outside its range.
    fn name(self) -> Option<String> {
        let (family, number, highest) = match self {
            TableStyle::Light(number) => ("Light", number, 21),
            TableStyle::Medium(number) => ("Medium", number, 28),
            TableStyle::Dark(number) => ("Dark", number, 11),
        };
        (1..=highest)
            .contains(&number)
            .then(|| format!("TableStyle{family}{number}"))
    }
}

/// A table's `<tableStyleInfo>`: its built-in style and the four flags,
/// each written as `0` or `1`. The Cargo feature `tables`.
#[cfg(feature = "tables")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableStyleInfo {
    /// The banded style.
    pub style: TableStyle,
    /// `showFirstColumn`.
    pub first_column: bool,
    /// `showLastColumn`.
    pub last_column: bool,
    /// `showRowStripes`.
    pub row_stripes: bool,
    /// `showColumnStripes`.
    pub column_stripes: bool,
}

/// A table as a caller gives it to [`Package::add_table`]
/// (`specs/0021-tables.md`, "Types"): its name, written as both `name` and
/// `displayName`, one column name per column of its area, and a style or
/// none. The Cargo feature `tables`.
#[cfg(feature = "tables")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Table<'a> {
    /// The table's name.
    pub name: &'a str,
    /// The column names, left to right.
    pub columns: &'a [&'a str],
    /// The `<tableStyleInfo>`, or `None` to write none.
    pub style: Option<TableStyleInfo>,
}

/// A table as [`Package::tables`] reads it from a worksheet's table part
/// (`specs/0021-tables.md`, "Types"). The Cargo feature `tables`.
#[cfg(feature = "tables")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableEntry {
    /// The root's `name`.
    pub name: String,
    /// The root's `displayName`.
    pub display_name: String,
    /// The root's `ref`, normalised.
    pub area: Area,
    /// Each `tableColumn`'s `name`, in order.
    pub columns: Vec<String>,
    /// The `tableStyleInfo`'s `name` as text (built-in or custom, empty when
    /// absent) with `showFirstColumn`, `showLastColumn`, `showRowStripes`
    /// and `showColumnStripes`, each `false` when absent; `None` when the
    /// part has no `tableStyleInfo`.
    pub style: Option<(String, bool, bool, bool, bool)>,
    /// Whether `headerRowCount` is other than `0` (absent is one row).
    pub header_row: bool,
    /// Whether `totalsRowCount` is other than `0` (absent is none).
    pub totals_row: bool,
    /// Whether any column carries a `calculatedColumnFormula`.
    pub calculated: bool,
    /// The table part's entry name.
    pub part: Vec<u8>,
}

/// A cell comment as a caller gives it to [`Package::add_comment`]
/// (`specs/0022-comments-and-images.md`, "Comments: types"): its author and
/// its text, written as one run with no properties. The Cargo feature
/// `comments`.
#[cfg(feature = "comments")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Comment<'a> {
    /// The author, 1 to 255 characters.
    pub author: &'a str,
    /// The text, 1 to 32767 characters.
    pub text: &'a str,
}

/// A cell comment as [`Package::comments`] reads it from a worksheet's
/// comments part. The Cargo feature `comments`.
#[cfg(feature = "comments")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentEntry {
    /// The comment's `ref`.
    pub cell: CellRef,
    /// The `authors` child at the comment's `authorId`, empty when the list
    /// is shorter.
    pub author: String,
    /// Every `t` descendant of the comment's `text`, in document order,
    /// runs flattened.
    pub text: String,
    /// The runs of the comment's `text` when it holds at least one `<r>`,
    /// else `None`, read as a cell's inline string's runs are: an `<rPr>`
    /// through the one reader, a run with no text dropped and a `text` whose
    /// every run is empty giving `None`, an `<rPh>` skipped
    /// (`specs/0024-rich-text.md`, "Comments"). [`CommentEntry::text`] stays
    /// every `t` descendant, so the two differ only by a phonetic run's text.
    /// The Cargo features `comments` and `richtext`.
    #[cfg(feature = "richtext")]
    pub runs: Option<RichText>,
}

/// One worksheet part's comment edits, as the write resolves them in record
/// order: the additions, a cell's last winning in its place; the cells
/// whose comment is removed; and the cells an addition replaces after a
/// removal of the same cell, which must hold a kept comment.
#[cfg(feature = "comments")]
#[derive(Default)]
struct PartComments {
    additions: Vec<(CellRef, NewComment)>,
    removals: Vec<CellRef>,
    replaced: Vec<CellRef>,
}

/// What a worksheet rewrite does with `<legacyDrawing>` for the comments:
/// creates it naming the VML relationship `create`, or drops it, in the
/// worksheet's form `strict`.
#[cfg(feature = "comments")]
#[derive(Clone)]
struct LegacyWork {
    create: Option<Vec<u8>>,
    drop: bool,
    strict: bool,
}

/// A comment to add, as [`Package::add_comment`] or, with the Cargo feature
/// `richtext`, [`Package::add_rich_comment`] checked it: `text` the
/// flattened text, and `runs` the runs written in its place when the comment
/// is rich.
#[cfg(feature = "comments")]
#[derive(Clone)]
struct NewComment {
    author: String,
    text: String,
    #[cfg(feature = "richtext")]
    runs: Option<RichText>,
}

/// An image as a caller gives it to [`Package::add_image`]
/// (`specs/0022-comments-and-images.md`, "Images: types"): the whole PNG or
/// JPEG file, and the pixel width and height to place it at, or `None` for
/// the file's own. The Cargo feature `images`.
#[cfg(feature = "images")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Image<'a> {
    /// The whole file.
    pub bytes: &'a [u8],
    /// The width and height in pixels, or `None` for the header's.
    pub size: Option<(u32, u32)>,
}

/// The anchor a drawing's picture sits in. The Cargo feature `images`.
#[cfg(feature = "images")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    /// `oneCellAnchor`: a cell and an extent.
    OneCell,
    /// `twoCellAnchor`: two cells.
    TwoCell,
    /// `absoluteAnchor`: a position and an extent, no cell.
    Absolute,
}

/// An image as [`Package::images`] reads it from a worksheet's drawing
/// part. The Cargo feature `images`.
#[cfg(feature = "images")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageEntry {
    /// The anchor's `from` column and row, zero-based; both 0 for an
    /// absolute anchor.
    pub from: (u32, u32),
    /// The anchor's kind.
    pub anchor: Anchor,
    /// The media entry's name, the picture's `blip` `r:embed` resolved
    /// through the drawing's rels part.
    pub part: Vec<u8>,
    /// The entry's uncompressed size from the archive's index.
    pub bytes: u64,
}

/// An image to add, as [`Package::add_image`] checked it: its bytes,
/// whether it is a PNG (else a JPEG), and its extent in EMU.
#[cfg(feature = "images")]
#[derive(Clone)]
struct NewImage {
    bytes: Vec<u8>,
    png: bool,
    cx: u64,
    cy: u64,
}

/// One worksheet part's image additions, in record order, each at its
/// cell.
#[cfg(feature = "images")]
type PartImages = Vec<(CellRef, NewImage)>;

/// What a worksheet rewrite does with `<drawing>` for the images: creates it
/// naming the drawing relationship `create`, in the worksheet's form
/// `strict`.
#[cfg(feature = "images")]
#[derive(Clone)]
struct DrawingWork {
    create: Vec<u8>,
    strict: bool,
}

/// A table to add, as [`Package::add_table`] checked it: its area, its
/// name, its column names and its style's name with the four flags.
#[cfg(feature = "tables")]
#[derive(Clone)]
struct NewTable {
    area: Area,
    name: String,
    columns: Vec<String>,
    style: Option<(String, TableStyleInfo)>,
}

/// A relationship's id and the part it names.
#[cfg(any(feature = "tables", feature = "comments"))]
type RelTarget = (Vec<u8>, Vec<u8>);

/// A table the write adds to a worksheet: its area and the id of the
/// relationship its `<tablePart>` names.
#[cfg(feature = "tables")]
#[derive(Clone)]
struct TableWork {
    area: Area,
    rel_id: Vec<u8>,
}

/// Whether the part's rewrite carries no table edit (the Cargo feature
/// `tables`).
#[cfg(all(feature = "rowcol", feature = "tables"))]
fn no_tables(work: &SheetWork) -> bool {
    work.tables.is_empty() && work.tables_removed.is_empty()
}

/// Whether the part's rewrite carries no table edit; always, without the
/// feature `tables`.
#[cfg(all(feature = "rowcol", not(feature = "tables")))]
fn no_tables(_: &SheetWork) -> bool {
    true
}

/// Whether the part's rewrite carries no data validation or conditional
/// format edit (the Cargo feature `rules`).
#[cfg(all(feature = "rowcol", feature = "rules"))]
fn no_rules(work: &SheetWork) -> bool {
    work.validations.is_empty() && work.rules.is_empty()
}

/// Whether the part's rewrite carries no data validation or conditional
/// format edit; always, without the feature `rules`.
#[cfg(all(feature = "rowcol", not(feature = "rules")))]
fn no_rules(_: &SheetWork) -> bool {
    true
}

/// Whether the part's rewrite carries no hyperlink or protection edit (the
/// Cargo feature `metadata`).
#[cfg(all(feature = "rowcol", feature = "metadata"))]
fn no_links(work: &SheetWork) -> bool {
    work.links.is_empty() && work.protection.is_none()
}

/// Whether the part's rewrite carries no hyperlink edit; always, without the
/// feature `metadata`.
#[cfg(all(feature = "rowcol", not(feature = "metadata")))]
fn no_links(_: &SheetWork) -> bool {
    true
}

/// Whether the part's rewrite carries no sheet setting (the Cargo feature
/// `sheetprops`).
#[cfg(all(feature = "rowcol", feature = "sheetprops"))]
fn no_settings(work: &SheetWork) -> bool {
    work.settings.is_empty()
}

/// Whether the part's rewrite carries no sheet setting; always, without the
/// feature `sheetprops`.
#[cfg(all(feature = "rowcol", not(feature = "sheetprops")))]
fn no_settings(_: &SheetWork) -> bool {
    true
}

impl SheetWork {
    /// The merge other than at its top-left that covers `cell`, if any.
    fn covers(&self, cell: CellRef) -> bool {
        self.merges
            .iter()
            .any(|area| area.holds(cell) && area.first != cell)
    }

    /// Whether `cell` is a header cell of a table the write adds to the
    /// part: the top row of its area (`specs/0021-tables.md`, decision 3 A).
    #[cfg(feature = "tables")]
    fn table_header(&self, cell: CellRef) -> bool {
        self.tables.iter().any(|table| {
            table.area.first.row == cell.row
                && (table.area.first.col..=table.area.last.col).contains(&cell.col)
        })
    }
}

/// One recorded change to the workbook's list of sheets.
#[derive(Clone)]
enum SheetEdit {
    /// A sheet added at the end: its name, entry, `sheetId` and
    /// relationship id, each the value it will have.
    Add {
        name: String,
        part: Vec<u8>,
        sheet_id: u32,
        rel_id: Vec<u8>,
    },
    /// The sheet named `from` takes the name `to`.
    Rename { from: String, to: String },
    /// The sheet named `name` goes to position `to`, 0 first.
    Move { name: String, to: usize },
    /// The sheet named `name` leaves the list (P4 ruling 4 for a pending
    /// one).
    Remove { name: String },
}

/// One sheet of the projected list and where it came from.
#[derive(Clone)]
struct Projected {
    sheet: Sheet,
    /// Its index in the file's `<sheets>`, or `None` for an added sheet.
    origin: Option<usize>,
    /// Whether an edit gave it a name other than the file's.
    renamed: bool,
}

/// The file's sheets with `edits` applied in order. An edit that no longer
/// holds, a name that is not there or a position past the end, is
/// `SHEET_EDIT_STALE`: [`Package`]'s calls check each edit when it is
/// recorded, so this is the write's defence and no public call reaches it.
fn project(file: &[Sheet], edits: &[SheetEdit]) -> Result<Vec<Projected>, Error> {
    let mut list: Vec<Projected> = file
        .iter()
        .enumerate()
        .map(|(at, sheet)| Projected {
            sheet: sheet.clone(),
            origin: Some(at),
            renamed: false,
        })
        .collect();
    for edit in edits {
        match edit {
            SheetEdit::Add {
                name,
                part,
                sheet_id,
                rel_id,
            } => list.push(Projected {
                sheet: Sheet {
                    name: name.clone(),
                    sheet_id: *sheet_id,
                    rel_id: rel_id.clone(),
                    state: None,
                    part: part.clone(),
                    kind: SheetKind::Worksheet,
                },
                origin: None,
                renamed: false,
            }),
            SheetEdit::Rename { from, to } => {
                let at = position_of(&list, from).ok_or(SHEET_EDIT_STALE)?;
                list[at].sheet.name = to.clone();
                list[at].renamed = true;
            }
            SheetEdit::Move { name, to } => {
                let at = position_of(&list, name).ok_or(SHEET_EDIT_STALE)?;
                if *to >= list.len() {
                    return Err(SHEET_EDIT_STALE);
                }
                let moved = list.remove(at);
                list.insert(*to, moved);
            }
            SheetEdit::Remove { name } => {
                let at = position_of(&list, name).ok_or(SHEET_EDIT_STALE)?;
                list.remove(at);
            }
        }
    }
    Ok(list)
}

/// Whether a sheet is visible: its `state` absent or exactly `visible`; any
/// other value counts as hidden (P4 ruling 1).
fn is_visible(sheet: &Sheet) -> bool {
    sheet
        .state
        .as_deref()
        .is_none_or(|state| state == b"visible")
}

/// The position of the sheet named exactly `name` in `list`.
fn position_of(list: &[Projected], name: &str) -> Option<usize> {
    list.iter().position(|entry| entry.sheet.name == name)
}

/// A name compared as Excel compares sheet names, ignoring case: every
/// `char` through `char::to_lowercase`, the no-dependency approximation the
/// limits name (`specs/0016-new-workbook-and-sheets.md`, "Sheet names").
fn folded(name: &str) -> String {
    name.chars().flat_map(char::to_lowercase).collect()
}

/// Checks a defined name against the rule of
/// `specs/0019-names-links-properties-protection.md`, "Defined names": 1 to
/// 255 characters; the first a letter, `_` or `\`; the rest letters, digits,
/// `.`, `_`, `\` or `?` ("letter" and "digit" by `char::is_alphabetic` and
/// `char::is_numeric`); not `_xlnm.`-prefixed; not a cell reference on the
/// grid (`specs/0021-tables.md`, P1 ruling 5), not of the `R1C1` form, and
/// not `R` or `C` alone, which Excel reserves (P1 verification ruling 4),
/// each ignoring case.
#[cfg(feature = "metadata")]
fn check_defined_name(name: &str) -> Result<(), Error> {
    let mut chars = name.chars();
    let first_ok = chars
        .next()
        .is_some_and(|first| first.is_alphabetic() || first == '_' || first == '\\');
    let rest_ok =
        chars.all(|c| c.is_alphabetic() || c.is_numeric() || matches!(c, '.' | '_' | '\\' | '?'));
    let built_in = name
        .get(..6)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("_xlnm."));
    if !first_ok
        || !rest_ok
        || name.chars().count() > 255
        || built_in
        || names_a_cell(name)
        || name.eq_ignore_ascii_case("r")
        || name.eq_ignore_ascii_case("c")
    {
        return Err(DEFINED_NAME_INVALID);
    }
    Ok(())
}

/// Whether a defined name or a table name reads as a cell Excel takes: an
/// `A1` reference on the grid, ignoring case, so `TABLE1` past the last
/// column is a name, or the `R1C1` form (`specs/0021-tables.md`, P1 ruling 5
/// and P1 verification ruling 4: one test shared by both rules).
#[cfg(any(feature = "metadata", feature = "tables"))]
fn names_a_cell(name: &str) -> bool {
    CellRef::parse(name.to_ascii_uppercase().as_bytes()).is_some_and(in_grid) || is_r1c1_form(name)
}

/// Whether `name` is `R` or `r`, optional digits, `C` or `c`, optional
/// digits, and nothing else.
#[cfg(any(feature = "metadata", feature = "tables"))]
fn is_r1c1_form(name: &str) -> bool {
    let bytes = name.as_bytes();
    let Some(rest) = bytes
        .strip_prefix(b"R")
        .or_else(|| bytes.strip_prefix(b"r"))
    else {
        return false;
    };
    let digits = rest.iter().take_while(|byte| byte.is_ascii_digit()).count();
    let rest = &rest[digits..];
    let Some(rest) = rest.strip_prefix(b"C").or_else(|| rest.strip_prefix(b"c")) else {
        return false;
    };
    rest.iter().all(u8::is_ascii_digit)
}

/// Checks a name a caller gives against the rules Excel enforces on entry
/// and against every other sheet of `list`, `except` being the renamed
/// sheet's own position (`specs/0016-new-workbook-and-sheets.md`, "Sheet
/// names", and P3 ruling 3).
fn check_sheet_name(name: &str, list: &[Projected], except: Option<usize>) -> Result<(), Error> {
    if name.is_empty() {
        return Err(SHEET_NAME_EMPTY);
    }
    if name.chars().count() > 31 {
        return Err(SHEET_NAME_LONG);
    }
    if name
        .chars()
        .any(|c| matches!(c, ':' | '\\' | '/' | '?' | '*' | '[' | ']'))
    {
        return Err(SHEET_NAME_CHARACTER);
    }
    if name.starts_with('\'') || name.ends_with('\'') {
        return Err(SHEET_NAME_APOSTROPHE);
    }
    if name.chars().any(forbidden_in_xml) {
        return Err(TEXT_NOT_XML);
    }
    let own = folded(name);
    if own == "history" {
        return Err(SHEET_NAME_HISTORY);
    }
    if list
        .iter()
        .enumerate()
        .any(|(at, entry)| Some(at) != except && folded(&entry.sheet.name) == own)
    {
        return Err(SHEET_NAME_IN_USE);
    }
    Ok(())
}

/// What one reference's edits in one write ask for: a value edit, a style
/// edit, or both, each the last recorded of its kind.
#[derive(Clone, Default)]
struct CellEdit {
    /// `Some(Some(value))` a set, `Some(None)` a clear, `None` no value edit.
    value: Option<Option<Value>>,
    /// `Some(Some(i))` the style `i`, `Some(None)` no `s`, `None` no style
    /// edit.
    style: Option<Option<u32>>,
}

impl CellEdit {
    /// Whether the edit writes a cell where there is none: a set, or a style
    /// `Some(i)` with or without a clear.
    fn writes_when_absent(&self) -> bool {
        matches!(self.value, Some(Some(_))) || matches!(self.style, Some(Some(_)))
    }

    /// The `s` bytes a cell written for this edit carries: the edit's style
    /// when there is a style edit, else `existing`, the cell's own.
    fn style_bytes(&self, existing: Option<Vec<u8>>) -> Option<Vec<u8>> {
        match self.style {
            Some(Some(index)) => Some(index.to_string().into_bytes()),
            Some(None) => None,
            None => existing,
        }
    }
}

/// One entry the walk of [`Package::remove_sheet`] removes, with its own rels
/// part and each one's `Override` when there is one to remove (P4 ruling 9).
struct WalkRemoval {
    entry: Vec<u8>,
    rels: Option<Vec<u8>>,
    over: Option<Vec<u8>>,
    rels_over: Option<Vec<u8>>,
}

/// What the chain recipe removes: the `calcChain` relationship on the
/// workbook part, the `Override` for its target when the content types part
/// holds one, and the entry when the package holds one.
struct ChainRemovals {
    workbook: Vec<u8>,
    id: Vec<u8>,
    over: Option<Vec<u8>>,
    entry: Option<Vec<u8>>,
}

impl Recorded {
    fn is_empty(&self) -> bool {
        self.rel_removals.is_empty()
            && self.rel_additions.is_empty()
            && self.override_removals.is_empty()
            && self.override_additions.is_empty()
            && self.part_removals.is_empty()
            && self.part_replacements.is_empty()
            && self.part_additions.is_empty()
            && self.cell_edits.is_empty()
            && self.style_edits.is_empty()
            && self.xf_additions.is_empty()
            && !self.recalculate
            && self.sheet_edits.is_empty()
            && self.column_formats.is_empty()
            && self.row_formats.is_empty()
            && self.panes.is_empty()
            && self.merges.is_empty()
            && self.unmerges.is_empty()
            && self.no_shifts()
            && self.no_name_edits()
            && self.no_link_edits()
            && self.no_rule_edits()
            && self.no_table_edits()
            && self.no_comment_edits()
            && self.no_image_edits()
            && self.no_setting_edits()
    }

    /// Whether no sheet setting is recorded; always, without the feature
    /// `sheetprops`.
    #[cfg(feature = "sheetprops")]
    fn no_setting_edits(&self) -> bool {
        self.tab_colors.is_empty() && self.sheet_views.is_empty()
    }

    /// Whether no sheet setting is recorded; always, without the feature
    /// `sheetprops`.
    #[cfg(not(feature = "sheetprops"))]
    fn no_setting_edits(&self) -> bool {
        true
    }

    /// Whether no comment is added; always, without the feature `comments`.
    #[cfg(feature = "comments")]
    fn no_comment_edits(&self) -> bool {
        self.comment_edits.is_empty()
    }

    /// Whether no comment is added; always, without the feature `comments`.
    #[cfg(not(feature = "comments"))]
    fn no_comment_edits(&self) -> bool {
        true
    }

    /// Whether no image is added; always, without the feature `images`.
    #[cfg(feature = "images")]
    fn no_image_edits(&self) -> bool {
        self.image_edits.is_empty()
    }

    /// Whether no image is added; always, without the feature `images`.
    #[cfg(not(feature = "images"))]
    fn no_image_edits(&self) -> bool {
        true
    }

    /// Whether no table is added or removed; always, without the feature
    /// `tables`.
    #[cfg(feature = "tables")]
    fn no_table_edits(&self) -> bool {
        self.table_additions.is_empty() && self.table_removals.is_empty()
    }

    /// Whether no table is added or removed; always, without the feature
    /// `tables`.
    #[cfg(not(feature = "tables"))]
    fn no_table_edits(&self) -> bool {
        true
    }

    /// Whether no data validation, differential style or conditional format
    /// is added or removed; always, without the feature `rules`.
    #[cfg(feature = "rules")]
    fn no_rule_edits(&self) -> bool {
        self.validations.is_empty() && self.dxf_additions.is_empty() && self.rules.is_empty()
    }

    /// Whether no data validation is added or removed; always, without the
    /// feature `rules`.
    #[cfg(not(feature = "rules"))]
    fn no_rule_edits(&self) -> bool {
        true
    }

    /// Whether no defined name is added or removed; always, without the
    /// feature `metadata`.
    #[cfg(feature = "metadata")]
    fn no_name_edits(&self) -> bool {
        self.name_edits.is_empty()
    }

    /// Whether no hyperlink, core property or protection is edited;
    /// always, without the feature `metadata`.
    #[cfg(feature = "metadata")]
    fn no_link_edits(&self) -> bool {
        self.links.is_empty()
            && self.properties.is_empty()
            && self.sheet_protections.is_empty()
            && self.book_protections.is_empty()
    }

    /// Whether no hyperlink, core property or protection is edited;
    /// always, without the feature `metadata`.
    #[cfg(not(feature = "metadata"))]
    fn no_link_edits(&self) -> bool {
        true
    }

    /// Whether no defined name is added or removed; always, without the
    /// feature `metadata`.
    #[cfg(not(feature = "metadata"))]
    fn no_name_edits(&self) -> bool {
        true
    }

    /// Whether no insertion or deletion is recorded; always, without the
    /// feature `rowcol`.
    #[cfg(feature = "rowcol")]
    fn no_shifts(&self) -> bool {
        self.shifts.is_empty()
    }

    /// Whether no insertion or deletion is recorded; always, without the
    /// feature `rowcol`.
    #[cfg(not(feature = "rowcol"))]
    fn no_shifts(&self) -> bool {
        true
    }

    /// The entries [`Package::add_sheet`] recorded, in record order, whose
    /// bytes are the template's blank worksheet until a cell edit writes
    /// into them.
    fn added_sheet_parts(&self) -> Vec<Vec<u8>> {
        self.sheet_edits
            .iter()
            .filter_map(|edit| match edit {
                SheetEdit::Add { part, .. } => Some(part.clone()),
                _ => None,
            })
            // A pending sheet removed again has no addition left (P4 ruling
            // 4).
            .filter(|part| self.part_additions.iter().any(|(added, _)| added == part))
            .collect()
    }

    /// Whether the last value edit recorded for some reference writes a
    /// [`Value::Formula`], which is one half of the formula-change rule; the
    /// other half, a removed normal formula, is known only once a worksheet
    /// has been streamed.
    fn writes_formula(&self) -> bool {
        self.cell_parts().iter().any(|part| {
            self.cell_edits_of(part)
                .iter()
                .any(|(_, edit)| matches!(edit.value, Some(Some(Value::Formula(_)))))
        })
    }

    /// Every worksheet part with an edit, once each: the value edits' parts
    /// in the order of each part's first, then the style edits' parts, then
    /// those of the formats, panes, merges and unmerges
    /// (`specs/0018-rows-and-columns.md`, P1 ruling 7), each not already
    /// listed.
    fn cell_parts(&self) -> Vec<Vec<u8>> {
        let mut parts: Vec<Vec<u8>> = Vec::new();
        let named = self
            .cell_edits
            .iter()
            .map(|(part, _, _)| part)
            .chain(self.style_edits.iter().map(|(part, _, _)| part))
            .chain(self.column_formats.iter().map(|(part, _, _)| part))
            .chain(self.row_formats.iter().map(|(part, _, _)| part))
            .chain(self.panes.iter().map(|(part, _)| part))
            .chain(self.merges.iter().map(|(part, _)| part))
            .chain(self.unmerges.iter().map(|(part, _)| part));
        #[cfg(feature = "rowcol")]
        let named = named.chain(self.shifts.iter().map(|(part, _)| part));
        #[cfg(feature = "metadata")]
        let named = named
            .chain(self.links.iter().map(|(part, _, _)| part))
            .chain(self.sheet_protections.iter().map(|(part, _)| part));
        #[cfg(feature = "rules")]
        let named = named
            .chain(self.validations.iter().map(|(part, _)| part))
            .chain(self.rules.iter().map(|(part, _)| part));
        #[cfg(feature = "tables")]
        let named = named
            .chain(self.table_additions.iter().map(|(part, _)| part))
            .chain(self.table_removals.iter().map(|(part, _)| part));
        // A sheet setting (`specs/0025-sheet-and-print-settings.md`,
        // "Planning rulings during P1", ruling 1).
        #[cfg(feature = "sheetprops")]
        let named = named
            .chain(self.tab_colors.iter().map(|(part, _)| part))
            .chain(self.sheet_views.iter().map(|(part, _)| part));
        // A comment edit rewrites the worksheet only when its
        // `<legacyDrawing>` is created or dropped
        // (`specs/0022-comments-and-images.md`, "Harness runs" and "P2").
        #[cfg(feature = "comments")]
        let named = named.chain(self.comment_works.iter().map(|(part, _)| part));
        // An image only when its `<drawing>` is created ("Harness runs").
        #[cfg(feature = "images")]
        let named = named.chain(self.image_works.iter().map(|(part, _)| part));
        for part in named {
            if !parts.iter().any(|seen| seen == part) {
                parts.push(part.clone());
            }
        }
        parts
    }

    /// One part's edits sorted by row then column, the last value edit and
    /// the last style edit recorded for a reference winning, each of its
    /// own kind.
    fn cell_edits_of(&self, part: &[u8]) -> Vec<(CellRef, CellEdit)> {
        let mut sorted: BTreeMap<CellRef, CellEdit> = BTreeMap::new();
        for (at, reference, value) in &self.cell_edits {
            if at.as_slice() == part {
                sorted.entry(*reference).or_default().value = Some(value.clone());
            }
        }
        for (at, reference, style) in &self.style_edits {
            if at.as_slice() == part {
                sorted.entry(*reference).or_default().style = Some(*style);
            }
        }
        sorted.into_iter().collect()
    }

    /// Everything one worksheet's rewrite applies.
    fn sheet_work(&self, part: &[u8]) -> SheetWork {
        let lines = |recorded: &[(Vec<u8>, u32, LineField)]| {
            let mut sorted: BTreeMap<u32, LineFormat> = BTreeMap::new();
            for (at, index, field) in recorded {
                if at.as_slice() == part {
                    let format = sorted.entry(*index).or_default();
                    match field {
                        LineField::Size(size) => format.size = Some(*size),
                        LineField::Hidden(hidden) => format.hidden = Some(*hidden),
                    }
                }
            }
            sorted.into_iter().collect()
        };
        let areas = |recorded: &[(Vec<u8>, Area)]| -> Vec<Area> {
            recorded
                .iter()
                .filter(|(at, _)| at.as_slice() == part)
                .map(|(_, area)| *area)
                .collect()
        };
        SheetWork {
            cells: self.cell_edits_of(part),
            rows: lines(&self.row_formats),
            columns: lines(&self.column_formats),
            pane: self
                .panes
                .iter()
                .rev()
                .find(|(at, _)| at.as_slice() == part)
                .map(|(_, pane)| *pane),
            merges: areas(&self.merges),
            unmerges: areas(&self.unmerges),
            #[cfg(feature = "metadata")]
            links: self.links_of(part),
            #[cfg(feature = "metadata")]
            dropped_ids: std::cell::RefCell::new(Vec::new()),
            #[cfg(feature = "metadata")]
            links_strict: self
                .link_strict
                .iter()
                .any(|(at, strict)| at.as_slice() == part && *strict),
            #[cfg(feature = "metadata")]
            protection: self
                .sheet_protections
                .iter()
                .rev()
                .find(|(at, _)| at.as_slice() == part)
                .map(|(_, protection)| *protection),
            #[cfg(feature = "rules")]
            validations: self
                .validations
                .iter()
                .filter(|(at, _)| at.as_slice() == part)
                .map(|(_, edit)| edit.clone())
                .collect(),
            #[cfg(feature = "rules")]
            rules: self
                .rules
                .iter()
                .filter(|(at, _)| at.as_slice() == part)
                .map(|(_, edit)| edit.clone())
                .collect(),
            #[cfg(feature = "tables")]
            tables: self
                .table_works
                .iter()
                .filter(|(at, _, _)| at.as_slice() == part)
                .map(|(_, table, _)| table.clone())
                .collect(),
            #[cfg(feature = "tables")]
            tables_strict: self
                .table_works
                .iter()
                .any(|(at, _, strict)| at.as_slice() == part && *strict),
            #[cfg(feature = "tables")]
            tables_removed: self
                .table_removed_ids
                .iter()
                .filter(|(at, _)| at.as_slice() == part)
                .map(|(_, id)| id.clone())
                .collect(),
            #[cfg(feature = "comments")]
            legacy: self
                .comment_works
                .iter()
                .find(|(at, _)| at.as_slice() == part)
                .and_then(|(_, work)| work.create.clone()),
            #[cfg(feature = "comments")]
            legacy_strict: self
                .comment_works
                .iter()
                .any(|(at, work)| at.as_slice() == part && work.strict),
            #[cfg(feature = "comments")]
            legacy_drop: self
                .comment_works
                .iter()
                .any(|(at, work)| at.as_slice() == part && work.drop),
            #[cfg(feature = "images")]
            drawing: self
                .image_works
                .iter()
                .find(|(at, _)| at.as_slice() == part)
                .map(|(_, work)| work.create.clone()),
            #[cfg(feature = "images")]
            drawing_strict: self
                .image_works
                .iter()
                .any(|(at, work)| at.as_slice() == part && work.strict),
            #[cfg(feature = "sheetprops")]
            settings: self.settings_of(part),
        }
    }

    /// One part's sheet settings: the last tab colour recorded, and the
    /// sheet view's fields each from the last call that gives it
    /// (`specs/0025-sheet-and-print-settings.md`, "The calls").
    #[cfg(feature = "sheetprops")]
    fn settings_of(&self, part: &[u8]) -> Settings {
        let mut view: Option<SheetView> = None;
        for (at, later) in &self.sheet_views {
            if at.as_slice() == part {
                view = Some(view.unwrap_or_default().then(later));
            }
        }
        Settings {
            tab: self
                .tab_colors
                .iter()
                .rev()
                .find(|(at, _)| at.as_slice() == part)
                .map(|(_, color)| *color),
            view,
        }
    }

    /// One part's hyperlink edits: each cell's last record, in the order of
    /// those records, an external set carrying the relationship id the write
    /// gave it.
    #[cfg(feature = "metadata")]
    fn links_of(&self, part: &[u8]) -> Vec<(CellRef, LinkEdit)> {
        let mut out: Vec<(CellRef, LinkEdit)> = Vec::new();
        for (at, (source, reference, set)) in self.links.iter().enumerate() {
            let last = !self.links[at + 1..]
                .iter()
                .any(|(other, cell, _)| other == source && cell == reference);
            if source.as_slice() != part || !last {
                continue;
            }
            let edit = match set {
                None => LinkEdit::Remove,
                Some(set) => LinkEdit::Set {
                    set: set.clone(),
                    rel_id: self
                        .link_ids
                        .iter()
                        .find(|(p, cell, _)| p.as_slice() == part && cell == reference)
                        .map(|(_, _, id)| id.clone()),
                },
            };
            out.push((*reference, edit));
        }
        out
    }

    /// Every source whose rels part this set changes, once each, in order.
    fn rel_sources(&self) -> Vec<Vec<u8>> {
        let mut sources: Vec<Vec<u8>> = Vec::new();
        for source in self
            .rel_removals
            .iter()
            .map(|(source, _)| source)
            .chain(self.rel_additions.iter().map(|(source, _)| source))
        {
            if !sources.iter().any(|seen| seen == source) {
                sources.push(source.clone());
            }
        }
        sources
    }
}

/// An archive with no entries: the 22-byte end of central directory record,
/// zero entries on this disk and in all, a directory of size 0 at offset 0,
/// and an empty comment. [`Package::new`] opens it and adds the template.
const EMPTY_ARCHIVE: [u8; 22] = [
    b'P', b'K', 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// The new workbook's six parts, in the order they are written
/// (`specs/0016-new-workbook-and-sheets.md`, "The template", and its P2
/// rulings 1 to 6): entry name, then the bytes of the file of that name under
/// `crates/emend/template/`, which are the entry's bytes exactly.
///
/// The files were written by hand for that spec. They are fictional and hold
/// no cell: a content types part with the `rels` and `xml` defaults and three
/// overrides; the package relationship to `xl/workbook.xml`; a workbook of
/// one sheet named `Sheet1`, one book view and no `calcPr`; its relationships
/// to the worksheet and the styles part; a worksheet whose `sheetData` is
/// empty; and a styles part holding one font, the two fills Excel requires,
/// one empty border, one cell style format, one cell format and the `Normal`
/// cell style, the collections `crate::styles`'s precondition names. Each is
/// the declaration, one line feed, and the root with no whitespace between
/// elements, with no line feed at the end.
const TEMPLATE: [(&[u8], &[u8]); 6] = [
    (
        CONTENT_TYPES_PART,
        include_bytes!("../template/[Content_Types].xml"),
    ),
    (b"_rels/.rels", include_bytes!("../template/_rels/.rels")),
    (
        b"xl/workbook.xml",
        include_bytes!("../template/xl/workbook.xml"),
    ),
    (
        b"xl/_rels/workbook.xml.rels",
        include_bytes!("../template/xl/_rels/workbook.xml.rels"),
    ),
    (
        b"xl/worksheets/sheet1.xml",
        include_bytes!("../template/xl/worksheets/sheet1.xml"),
    ),
    (
        b"xl/styles.xml",
        include_bytes!("../template/xl/styles.xml"),
    ),
];

/// The template's blank worksheet: the part every added sheet starts as.
const BLANK_WORKSHEET: &[u8] = TEMPLATE[4].1;

/// The transitional worksheet content type, which the strict form shares.
const WORKSHEET_CONTENT_TYPE: &[u8] =
    b"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";

impl Package<Cursor<Vec<u8>>> {
    /// A new workbook: the template's six parts, written and opened back
    /// (`specs/0016-new-workbook-and-sheets.md`, "`Package::new`").
    ///
    /// The empty archive is opened, the six parts are recorded as additions
    /// in the template's order, [`Package::write_to`] writes them into memory,
    /// and the result is opened, so every later edit takes the ordinary path
    /// and nothing here is a writer of its own. The archive layer writes an
    /// added entry with fixed flags, method and time, so every call gives the
    /// same bytes, on every target. The workbook holds one sheet, `Sheet1`, of
    /// kind [`SheetKind::Worksheet`] and no cell; the caller who wants it on
    /// disk writes it with no edits, which is a verbatim round trip.
    ///
    /// # Errors
    ///
    /// Whatever [`Package::write_to`] and [`Package::open`] refuse. The bytes
    /// are fixed and are never refused themselves; the temporary a written
    /// part goes through can fail on I/O, as in any write. No test can reach
    /// an error here; the module header's catalogue says so.
    pub fn new() -> Result<Self, Error> {
        let mut template = Package::open(Cursor::new(EMPTY_ARCHIVE.to_vec()))?;
        for (name, bytes) in TEMPLATE {
            template.add_part(name, replacement_from_reader(bytes));
        }
        let mut written = Vec::new();
        template.write_to(&mut written)?;
        Package::open(Cursor::new(written))
    }
}

impl<R: Read + Seek> Package<R> {
    /// Opens the archive and parses no XML.
    ///
    /// The reader must be `'static`, as [`Archive::open`] says: a borrowed
    /// reader such as `&mut File` is not accepted (`specs/0016`, "Planning
    /// rulings during P5", ruling 2).
    ///
    /// # Errors
    ///
    /// Whatever [`Archive::open`] refuses.
    pub fn open(reader: R) -> Result<Self, Error>
    where
        R: 'static,
    {
        Ok(Package {
            archive: Archive::open(reader)?,
            content_types: None,
            relationships: Vec::new(),
            workbook: None,
            calc_prs: 0,
            styles: None,
            edits: Recorded::default(),
        })
    }

    /// The archive underneath.
    pub fn archive(&self) -> &Archive<R> {
        &self.archive
    }

    /// The index of the entry named `name`, by exact bytes.
    pub fn part(&self, name: &[u8]) -> Option<usize> {
        self.archive.index_of(name)
    }

    /// One entry's decompressed bytes.
    ///
    /// # Errors
    ///
    /// Whatever [`Archive::read`] refuses.
    pub fn read_part(&mut self, index: usize) -> Result<impl Read + '_, Error> {
        self.archive.read(index)
    }

    /// The workbook's shared strings, parsed on every call and returned OWNED.
    ///
    /// `specs/0010-reading-cells.md`, "Module `package`": unlike
    /// [`Package::content_types`] and [`Package::workbook`], which cache and
    /// lend, this parses each time and hands the table over. A caller streams
    /// cells through a mutable borrow of the package, so a lent table could
    /// not be held across that borrow, and cloning a cached one would cost a
    /// copy of every string.
    ///
    /// A workbook with no `sharedStrings` relationship gives an empty table,
    /// so a cell naming an index in such a workbook is out of range rather
    /// than a case of its own.
    ///
    /// A relationship that DOES name a part, where the archive holds no such
    /// entry, is refused with "no such part" (`specs/0010-reading-cells.md`,
    /// P1 ruling 5). The two cases are not the same answer: no relationship
    /// is a workbook saying it has no table, while an unresolved target is a
    /// claim the package makes about itself and cannot keep, and a reader
    /// asked to follow it and finding nothing says so rather than reporting
    /// an empty table that no part witnesses. Part 2's survey counts
    /// unresolved targets across the package; the readers do not paper over
    /// them.
    ///
    /// Records no edit: a package read this way and written writes a copy.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "the shared strings part's root is not sst", "a
    /// cell holds a reference this layer cannot expand", "a text element
    /// holds an element", or "no such part" for a relationship whose target
    /// the archive does not hold; [`Error::Xml`] for bytes that are not
    /// well-formed XML or not UTF-8; whatever [`Package::workbook`] refuses,
    /// since the relationship is followed through it.
    pub fn shared_strings(&mut self) -> Result<SharedStrings, Error> {
        self.parts().shared_strings()
    }

    /// A streaming read over the worksheet part named `part`.
    ///
    /// `specs/0010-reading-cells.md`, "Module `package`". The entry name is
    /// the one a sheet's relationship resolved to, which is
    /// [`Sheet::part`]; nothing here looks a sheet up by name, because a
    /// caller that has the workbook already has the part.
    ///
    /// The reader borrows the package for as long as it streams, which is
    /// why [`Package::shared_strings`] hands its table over OWNED: a caller
    /// resolving shared strings while streaming cells holds the table and
    /// the borrow at the same time.
    ///
    /// Records no edit: a package read this way and written writes a copy.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part" when the archive holds no entry of
    /// that name, a part [`Package::add_sheet`] added and not yet written
    /// among them (P3 ruling 2), whatever [`Archive::read`] refuses for the
    /// entry, and whatever [`Stream::new`] refuses for its first bytes. The
    /// cells themselves are read as the iterator runs, so a malformed
    /// worksheet is an error from [`Cells::next`] and not from this call.
    pub fn cells(&mut self, part: &[u8]) -> Result<Cells<'_, R>, Error> {
        let index = self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        let reader = self.archive.read(index)?;
        Cells::new(Box::new(reader), index)
    }

    /// The package's content types, parsed on the first call and kept.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no \[Content_Types\].xml part" or "a content types
    /// part whose root is not Types"; [`Error::Xml`] for bytes that are not
    /// well-formed XML.
    pub fn content_types(&mut self) -> Result<&ContentTypes, Error> {
        self.parts().into_content_types()
    }

    /// The relationships of the part `source`, or of the package when
    /// `source` is empty, parsed on the first call for that source and kept.
    ///
    /// An absent rels part gives an empty set: most parts have none, so that
    /// is an answer, not an error.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a relationships part whose root is not
    /// Relationships" or "two relationships with the same id";
    /// [`Error::Xml`] for bytes that are not well-formed XML.
    pub fn relationships(&mut self, source: &[u8]) -> Result<&Relationships, Error> {
        self.parts().into_relationships(source)
    }

    /// The workbook part and its sheets, parsed on the first call and kept.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] for any of the officeDocument, workbook root and
    /// sheet messages the module header lists; [`Error::Xml`] for bytes that
    /// are not well-formed XML.
    pub fn workbook(&mut self) -> Result<&Workbook, Error> {
        self.parts().into_workbook()
    }

    /// The workbook's styles part, parsed on the first call and kept.
    ///
    /// `specs/0012-styles.md`, "Module `styles`: what is read": the part the
    /// workbook's first `styles` relationship names, read through [`Stream`]
    /// and [`Scope`] with the element resolution [`Cells`] uses, for its
    /// number formats and its cell formats.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "the workbook has no styles part" when the workbook
    /// has no `styles` relationship, "no such part" when the relationship
    /// names an entry the archive does not hold, and the styles messages the
    /// module header lists; [`Error::Xml`] for bytes that are not well-formed
    /// XML or not UTF-8; whatever [`Package::workbook`] refuses, since the
    /// relationship is followed through it.
    pub fn styles(&mut self) -> Result<&Styles, Error> {
        self.parts().into_styles()
    }

    /// Records the removal of the relationship `id` from `source`'s rels part.
    ///
    /// Nothing is read or written here, and nothing else goes with it: the
    /// target part, its override and its own rels part are the caller's
    /// separate calls. [`Package::write_to`] refuses an id the part does not
    /// hold.
    pub fn remove_relationship(&mut self, source: &[u8], id: &[u8]) {
        self.parts().remove_relationship(source, id)
    }

    /// Records a new relationship on `source` and returns the id it will have.
    ///
    /// The id is `rId<n>`, `n` one more than the largest among the part's ids
    /// of the form `rId<digits>` and the ids this call has already handed out
    /// for the same source, or `rId1` when there is none. An id of another
    /// form is never generated, so a generated id never collides with one.
    ///
    /// # Errors
    ///
    /// Whatever [`Package::relationships`] refuses for `source`'s rels part,
    /// since the id rule has to read the ids already there. A source with no
    /// rels part is an empty set, not an error: [`Package::write_to`] creates
    /// the part.
    pub fn add_relationship(
        &mut self,
        source: &[u8],
        kind: &[u8],
        target: &[u8],
        external: bool,
    ) -> Result<Vec<u8>, Error> {
        self.parts()
            .add_relationship(source, kind, target, external)
    }

    /// Records the removal of the `Override` for `part_name`.
    ///
    /// `part_name` is matched as the `PartName` attribute writes it, by exact
    /// bytes, leading slash included. [`Package::write_to`] refuses an
    /// override the part does not hold.
    pub fn remove_override(&mut self, part_name: &[u8]) {
        self.parts().remove_override(part_name)
    }

    /// Records a new `Override` for `part_name`.
    ///
    /// [`Package::write_to`] refuses a part name that already has one.
    pub fn add_override(&mut self, part_name: &[u8], content_type: &[u8]) {
        self.parts().add_override(part_name, content_type)
    }

    /// Records the removal of the entry `name`.
    ///
    /// The entry alone: its override and any relationship naming it are the
    /// caller's separate calls.
    pub fn remove_part(&mut self, name: &[u8]) {
        self.parts().remove_part(name)
    }

    /// Records new bytes for the entry `name`.
    pub fn replace_part(&mut self, name: &[u8], replacement: Replacement) {
        self.parts().replace_part(name, replacement)
    }

    /// Records a new entry `name`.
    ///
    /// The entry alone: the caller adds an override or relies on a `Default`
    /// covering its extension.
    pub fn add_part(&mut self, name: &[u8], replacement: Replacement) {
        self.parts().add_part(name, replacement)
    }

    /// Records the removal of the workbook's calculation chain, and says
    /// whether there was one.
    ///
    /// The recipe of `specs/0009-package-layer.md`, "Module `package`", moved
    /// here from `tools/emend-copy` (`specs/0011-editing-cells.md`, "Module
    /// `package`"): when the workbook has a `calcChain` relationship, the
    /// three removals in the order the recipe gives them, each only when its
    /// subject is there: the relationship, the `Override` for its target
    /// when the content types part has one, and the entry when the package
    /// has one. The `Override` is matched as the package writes it, leading
    /// slash and all, and removed by those same bytes. With no `calcChain`
    /// relationship nothing is recorded and the answer is `false`.
    ///
    /// # Errors
    ///
    /// Whatever [`Package::workbook`] refuses, and, when there is a chain,
    /// whatever [`Package::content_types`] and [`Relationships::target_part`]
    /// refuse. Nothing is recorded when this fails.
    pub fn drop_calc_chain(&mut self) -> Result<bool, Error> {
        self.parts().drop_calc_chain()
    }

    /// Records `reference` in the worksheet part `part` set to `value`.
    ///
    /// `part` is the worksheet's entry name, as [`Package::cells`] takes it;
    /// the caller finds it through [`Workbook::sheet`]. Nothing is read and
    /// nothing can fail here, and the last edit recorded for a reference
    /// wins. [`Package::write_to`] checks the value and the reference, which
    /// must lie inside the grid ("a row or column index or count is out of
    /// range"), and rewrites the part (`specs/0011-editing-cells.md`, "Module
    /// `package`"; `specs/0023-scale.md`, P1 verification ruling 5).
    pub fn set_cell(&mut self, part: &[u8], reference: CellRef, value: Value) {
        self.parts().set_cell(part, reference, value)
    }

    /// Records `reference` in the worksheet part `part` cleared.
    ///
    /// A cell with a style keeps it and loses everything else; a cell without
    /// one is removed; a cell that does not exist stays absent. As
    /// [`Package::set_cell`], nothing is read here, the last edit recorded
    /// for a reference wins, and the write refuses a reference off the grid.
    pub fn clear_cell(&mut self, part: &[u8], reference: CellRef) {
        self.parts().clear_cell(part, reference)
    }

    /// Records a new cell format and returns the index it will have.
    ///
    /// `specs/0012-styles.md`, "Module `styles`: what can be added". The new
    /// `xf` is a copy of the existing cell format `base` with its number
    /// format changed to `format`, or with `None` the form `<xf numFmtId="N"
    /// fontId="0" fillId="0" borderId="0" xfId="0" applyNumberFormat="1"/>`;
    /// it is composed here and appended to `<cellXfs>` by
    /// [`Package::write_to`]. The index is the number of `xf` elements
    /// `<cellXfs>` holds plus the number already recorded by this package.
    ///
    /// The number format's id: `BuiltIn(id)` is `id`, and nothing is added to
    /// `<numFmts>`, whether or not the part spells that id out. `Code(code)`
    /// is the id of the first `numFmt` whose code equals it, else of the
    /// first equal code recorded earlier by this package, else a new id: the
    /// largest of 164, one more than the largest id the part defines and one
    /// more than the largest recorded, and the new `(id, code)` is recorded.
    /// Two additions with equal codes share one id; two additions with an
    /// equal base and format are two cell formats, since nothing is
    /// deduplicated at the `xf` level.
    ///
    /// The styles part is read first when it is not cached, so this fails as
    /// [`Package::styles`] fails. Nothing is recorded when this fails.
    ///
    /// # Errors
    ///
    /// Whatever [`Package::styles`] refuses; [`Error::Package`] "a style
    /// index is out of range" for a `base` at or beyond the part's cell
    /// formats (a base names an existing `xf` only, never one recorded here),
    /// and for an index or a new id that `u32` cannot hold; "a built-in number
    /// format id is 164 or more"; "a number format code is empty or holds a
    /// character XML cannot carry" (the set [`Value::Text`] refuses).
    pub fn add_cell_style(
        &mut self,
        base: Option<u32>,
        format: NumberFormat,
    ) -> Result<u32, Error> {
        let style = Style {
            number_format: Some(format),
            ..Style::default()
        };
        self.parts().add_style(base, &style)
    }

    /// Records a new cell format carrying `style` and returns the index it
    /// will have.
    ///
    /// `specs/0017-fuller-styles.md`, "Module `styles`: what can be added".
    /// [`Package::add_cell_style`] is this with a number format alone, and
    /// what it says of the base, the index and the number format holds here.
    /// A font, a fill and a border are each appended to the part's `<fonts>`,
    /// `<fills>` or `<borders>` by [`Package::write_to`], and the new cell
    /// format names it: its id is that of the first equal one recorded
    /// earlier by this package, else the collection's count plus the number
    /// of distinct ones recorded earlier. Nothing is compared with the part's
    /// own fonts, fills and borders, which are not read, so an equal one is a
    /// second copy.
    ///
    /// With a base, the new cell format is a copy of it with each id `style`
    /// gives replaced and the apply flag of each thing it gives set to `1`,
    /// an id or a flag the base lacks added; its content is the base's with
    /// the `alignment` and `protection` given in place of the base's (a new
    /// `alignment` first, a new `protection` after any `alignment`), every
    /// other child the base's bytes. With `None`, `<xf numFmtId="N"
    /// fontId="F" fillId="I" borderId="B" xfId="0"/>` with the given ids or
    /// 0, the apply flag of each thing given, and the given `alignment` and
    /// `protection` as its children.
    ///
    /// The styles part is read first when it is not cached, so this fails as
    /// [`Package::styles`] fails. Nothing is recorded when this fails.
    ///
    /// # Errors
    ///
    /// Whatever [`Package::add_cell_style`] refuses; [`Error::Package`] "a
    /// style addition needs a collection the styles part lacks" for a font,
    /// a fill or a border given to a part without `<fonts>`, `<fills>` or
    /// `<borders>`; "a font name is empty or holds a character XML cannot
    /// carry"; "a style field is out of range" for a font size that is not
    /// finite or not above zero, a tint that is not finite or lies outside
    /// `-1.0..=1.0`, and a text rotation above 180 that is not 255; "a style index is out of range" for an id `u32`
    /// cannot hold.
    pub fn add_style(&mut self, base: Option<u32>, style: &Style) -> Result<u32, Error> {
        self.parts().add_style(base, style)
    }

    /// Records `reference` in the worksheet part `part` given the style
    /// `style`: `Some(i)` names a cell format by index, one the part has or
    /// one [`Package::add_cell_style`] returned, and `None` removes the
    /// cell's `s`.
    ///
    /// As [`Package::set_cell`], nothing is read and nothing can fail here,
    /// and the last style edit recorded for a reference wins, independently
    /// of the last value edit for it (`specs/0012-styles.md`, "Module
    /// `package`"). [`Package::write_to`] checks the index and the
    /// reference, which must lie inside the grid, and rewrites the part.
    pub fn set_cell_style(&mut self, part: &[u8], reference: CellRef, style: Option<u32>) {
        self.parts().set_cell_style(part, reference, style)
    }

    /// Records the width of `column` in the worksheet part `part`:
    /// `Some(w)` writes `width="w" customWidth="1"` on the `<col>` that
    /// covers the column, `None` removes both.
    ///
    /// `specs/0018-rows-and-columns.md`, "Module `package`: the calls". The
    /// column is 1-based. Nothing is read here; the last width recorded for
    /// a column wins. [`Package::write_to`] splits a `<col>` that covers
    /// more than the column so that the column alone carries the change,
    /// creates one where none covers it, and drops one left with `min` and
    /// `max` alone.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a row or column index or count is out of range"
    /// for a column outside `1..=16384`; "a column width or a row height is
    /// out of range" for a width that is not finite or lies outside
    /// `0..=255`. Nothing is recorded when this fails.
    pub fn set_column_width(
        &mut self,
        part: &[u8],
        column: u32,
        width: Option<f64>,
    ) -> Result<(), Error> {
        self.parts()
            .line_format(part, column, LineField::Size(width), false)
    }

    /// Records `column` in the worksheet part `part` hidden or shown:
    /// `hidden="1"` set on, or removed from, the `<col>` that covers it, as
    /// [`Package::set_column_width`] places a width.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a row or column index or count is out of range"
    /// for a column outside `1..=16384`. Nothing is recorded when this fails.
    pub fn set_column_hidden(
        &mut self,
        part: &[u8],
        column: u32,
        hidden: bool,
    ) -> Result<(), Error> {
        self.parts()
            .line_format(part, column, LineField::Hidden(hidden), false)
    }

    /// Records the height of `row` in the worksheet part `part`: `Some(h)`
    /// writes `ht="h" customHeight="1"` on the `<row>`, `None` removes both.
    ///
    /// A row the part does not have is created, empty unless a cell edit
    /// falls in it; a row present is retagged, its other attributes and its
    /// children kept. Nothing is read here; the last height recorded for a
    /// row wins.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a row or column index or count is out of range"
    /// for a row outside `1..=1048576`; "a column width or a row height is
    /// out of range" for a height that is not finite or lies outside
    /// `0..=409`. Nothing is recorded when this fails.
    pub fn set_row_height(
        &mut self,
        part: &[u8],
        row: u32,
        height: Option<f64>,
    ) -> Result<(), Error> {
        self.parts()
            .line_format(part, row, LineField::Size(height), true)
    }

    /// Records `row` in the worksheet part `part` hidden or shown:
    /// `hidden="1"` set on, or removed from, the `<row>`, placed as
    /// [`Package::set_row_height`] places a height.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a row or column index or count is out of range"
    /// for a row outside `1..=1048576`. Nothing is recorded when this fails.
    pub fn set_row_hidden(&mut self, part: &[u8], row: u32, hidden: bool) -> Result<(), Error> {
        self.parts()
            .line_format(part, row, LineField::Hidden(hidden), true)
    }

    /// Records the worksheet part `part` frozen at `at`: `Some(c)` freezes
    /// the rows above `c` and the columns left of it, `None` unfreezes.
    ///
    /// `specs/0018-rows-and-columns.md`, "Module `package`: the calls". The
    /// write replaces the first `<sheetView>`'s `<pane>` and every
    /// `<selection>` in it with a frozen pane and one selection naming the
    /// pane, or for `None` removes them and writes nothing; a `<sheetViews>`
    /// that is absent is created for `Some`. The last recorded for a part
    /// wins.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a freeze at A1 freezes nothing"; "a row or column
    /// index or count is out of range" for a cell outside the grid. Nothing
    /// is recorded when this fails.
    pub fn freeze_panes(&mut self, part: &[u8], at: Option<CellRef>) -> Result<(), Error> {
        self.parts().freeze_panes(part, at)
    }

    /// Records the tab colour of the worksheet part `part`: `Some` writes
    /// `<tabColor>` under `<sheetPr>`, its attributes as a font's colour
    /// writes them, `None` drops it; the last record for a part wins. The
    /// write replaces the part's `<tabColor>`, or creates one first among
    /// `<sheetPr>`'s children, creating `<sheetPr>` in schema position when
    /// the part lacks it, every other attribute and child of `<sheetPr>`
    /// kept; a `<sheetPr>` the drop leaves with no attribute and no child
    /// goes with it. The Cargo feature `sheetprops`
    /// (`specs/0025-sheet-and-print-settings.md`, "The calls", "The pass").
    ///
    /// The part is not looked up here: a sheet [`Package::add_sheet`] adds
    /// in the same write takes the colour on its blank worksheet ("Planning
    /// rulings during P1", ruling 1).
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a sheet setting is out of range" for a theme
    /// colour whose tint is not finite or lies outside `-1.0..=1.0`; nothing
    /// is recorded when this fails. The write refuses a part the archive
    /// lacks, "no such part"; a part with two `<sheetPr>` or one after
    /// another root child, "a worksheet has two sheetPr, autoFilter,
    /// printOptions, pageMargins, pageSetup or headerFooter elements"; a
    /// part with a row or column shift in the same write; and what a cell
    /// edit to the part would meet.
    #[cfg(feature = "sheetprops")]
    pub fn set_tab_color(&mut self, part: &[u8], color: Option<Color>) -> Result<(), Error> {
        self.parts().set_tab_color(part, color)
    }

    /// Records the settings of the worksheet part `part`'s first
    /// `<sheetView>`: each `Some` field written as its attribute, the
    /// fields of two calls on one part each standing unless the later call
    /// gives them. The write sets the attributes the call gives first, in
    /// the schema's order, and keeps every other attribute after them byte
    /// for byte, `tabSelected` and `workbookViewId` among them, and every
    /// child; a part with no `<sheetViews>` gains one holding
    /// `<sheetView workbookViewId="0">`, as [`Package::freeze_panes`]
    /// creates one. A freeze in the same write is applied to the same
    /// `<sheetView>`, the pane after the attributes. The Cargo feature
    /// `sheetprops` (`specs/0025-sheet-and-print-settings.md`, "The calls",
    /// "The pass").
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a sheet setting is out of range" for a zoom
    /// outside `10..=400`; nothing is recorded when this fails. The write
    /// refuses a part the archive lacks, "no such part"; a part with two
    /// `<sheetViews>`, or one without a `<sheetView>`, with part 3's
    /// message; a part with a row or column shift in the same write; and
    /// what a cell edit to the part would meet.
    #[cfg(feature = "sheetprops")]
    pub fn set_sheet_view(&mut self, part: &[u8], view: &SheetView) -> Result<(), Error> {
        self.parts().set_sheet_view(part, view)
    }

    /// Records `area` in the worksheet part `part` merged.
    ///
    /// `specs/0018-rows-and-columns.md`, "Module `package`: the calls". The
    /// write appends a `<mergeCell>` to `<mergeCells>`, created when absent,
    /// and sets its `count`. The covered cells other than the top-left lose
    /// their value and formula and keep their `s`, as
    /// [`Package::clear_cell`] leaves a cell. The corners are normalised
    /// here.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a merge needs more than one cell"; "a row or
    /// column index or count is out of range" for an area outside the grid.
    /// Nothing is recorded when this fails. The write refuses an area that
    /// overlaps a merge the part has or one recorded before it, and a cell
    /// edit recorded for a covered cell other than the top-left.
    pub fn merge_cells(&mut self, part: &[u8], area: Area) -> Result<(), Error> {
        self.parts().merge_cells(part, area)
    }

    /// Records the merge of exactly `area` in the worksheet part `part`
    /// removed; nothing is written into the cells. A `<mergeCells>` left
    /// empty is dropped. The corners are normalised here.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a row or column index or count is out of range"
    /// for an area outside the grid. Nothing is recorded when this fails.
    /// The write refuses an area the file's merges do not hold, "no merge of
    /// that range".
    pub fn unmerge_cells(&mut self, part: &[u8], area: Area) -> Result<(), Error> {
        self.parts().unmerge_cells(part, area)
    }

    /// Records `count` rows inserted in the worksheet part `part` at row
    /// `at`: the rows from `at` on move down by `count`.
    ///
    /// `specs/0018-rows-and-columns.md`, "Insert and delete" and "The shift
    /// rule". The write moves every row and cell, the merges, `dimension`,
    /// the shared, array and data table `ref`s and the conditional format,
    /// data validation, hyperlink and filter ranges with them; formula
    /// text, defined names, comments, drawings and views stay where they
    /// were. A worksheet takes one insertion or deletion per write and no
    /// other edit in that write: a caller that wants to insert and then fill
    /// writes twice. The Cargo feature `rowcol` (decision 7 A).
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a row or column index or count is out of range"
    /// for a `count` of 0 or rows past 1048576. Nothing is recorded when
    /// this fails. The write refuses what "The shift rule" refuses.
    #[cfg(feature = "rowcol")]
    pub fn insert_rows(&mut self, part: &[u8], at: u32, count: u32) -> Result<(), Error> {
        self.parts().shift(part, at, count, true, false)
    }

    /// Records `count` rows deleted from the worksheet part `part` from row
    /// `at`: those rows and their cells go, and the rows after them move up
    /// by `count`, as [`Package::insert_rows`] moves them down.
    ///
    /// # Errors
    ///
    /// As [`Package::insert_rows`].
    #[cfg(feature = "rowcol")]
    pub fn delete_rows(&mut self, part: &[u8], at: u32, count: u32) -> Result<(), Error> {
        self.parts().shift(part, at, count, false, false)
    }

    /// Records `count` columns inserted in the worksheet part `part` at
    /// column `at`: the columns from `at` on move right by `count`.
    ///
    /// `specs/0018-rows-and-columns.md`, "The shift rule", on the column
    /// axis: every cell, `spans`, the `<col>` intervals, `dimension`, the
    /// merges, the shared, array and data table `ref`s and the four range
    /// lists move as [`Package::insert_rows`] moves them on the row axis,
    /// with the same limits and the same one-shift rule. The Cargo feature
    /// `rowcol`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a row or column index or count is out of range"
    /// for a `count` of 0 or columns past 16384. Nothing is recorded when
    /// this fails. The write refuses what "The shift rule" refuses.
    #[cfg(feature = "rowcol")]
    pub fn insert_columns(&mut self, part: &[u8], at: u32, count: u32) -> Result<(), Error> {
        self.parts().shift(part, at, count, true, true)
    }

    /// Records `count` columns deleted from the worksheet part `part` from
    /// column `at`: their cells go, every row stays, and the columns after
    /// them move left by `count`.
    ///
    /// # Errors
    ///
    /// As [`Package::insert_columns`].
    #[cfg(feature = "rowcol")]
    pub fn delete_columns(&mut self, part: &[u8], at: u32, count: u32) -> Result<(), Error> {
        self.parts().shift(part, at, count, false, true)
    }

    /// Records a defined name: `name` with the content `text`, for the whole
    /// workbook (`sheet` `None`) or for the sheet named `sheet` in
    /// [`Package::sheets`]'s list. The Cargo feature `metadata`
    /// (`specs/0019-names-links-properties-protection.md`, "Defined names").
    ///
    /// The text is stored, never parsed or checked as a formula, and is
    /// written as a cell's text is, escaped; Excel stores it without a
    /// leading `=`, which is the caller's to leave out. The scope follows its
    /// sheet through a rename, a move or a removal recorded in the same
    /// write and becomes the sheet's index in the output's list. The write
    /// appends the name at the end of `<definedNames>`, creating that
    /// element in schema position when the workbook has none, and applies
    /// the formula-change rule: the calculation chain dropped once when the
    /// workbook has one, `fullCalcOnLoad` set (decision 2 A). Nothing that
    /// uses a name is touched or checked.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a defined name is not valid" for a `name` outside
    /// the rule (1 to 255 characters, the first a letter, `_` or `\`, the
    /// rest letters, digits, `.`, `_`, `\` or `?`, not `_xlnm.`-prefixed,
    /// not a cell reference on the grid and not of the `R1C1` form, each
    /// ignoring case)
    /// or an empty `text`; "a text holds a character XML cannot carry" for a
    /// `text` holding one; "no sheet of that name" for a `sheet` the list
    /// does not hold; and whatever [`Package::workbook`] refuses. Nothing is
    /// recorded when this fails. The write refuses a name and scope the file
    /// has, or that this write adds or removes already, ignoring case, "a
    /// defined name of that name and scope already exists"; a scope whose
    /// sheet a later edit removed, "no sheet of that name"; and a workbook
    /// part with two `<definedNames>`, "a workbook has two definedNames or
    /// workbookProtection elements".
    #[cfg(feature = "metadata")]
    pub fn add_defined_name(
        &mut self,
        name: &str,
        text: &str,
        sheet: Option<&str>,
    ) -> Result<(), Error> {
        self.parts().add_defined_name(name, text, sheet)
    }

    /// Records the removal of the file's defined name `name`, compared
    /// ignoring case, with the scope `sheet` (`None` for a workbook-level
    /// name, else a sheet named in [`Package::sheets`]'s list, matched
    /// against the file's `localSheetId`). The write drops the element and
    /// applies the formula-change rule, as [`Package::add_defined_name`]
    /// says; a `<definedNames>` left empty is dropped. The Cargo feature
    /// `metadata`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no sheet of that name" for a `sheet` the list does
    /// not hold, and whatever [`Package::workbook`] refuses; nothing is
    /// recorded when this fails. The write refuses a name and scope the file
    /// does not have, "no defined name of that name and scope"; a name and
    /// scope this write also adds, "a defined name of that name and scope
    /// already exists" (the caller writes twice); and a workbook part with
    /// two `<definedNames>`.
    #[cfg(feature = "metadata")]
    pub fn remove_defined_name(&mut self, name: &str, sheet: Option<&str>) -> Result<(), Error> {
        self.parts().remove_defined_name(name, sheet)
    }

    /// Records a hyperlink on the cell `reference` of the worksheet part
    /// `part`: to `target`, with `tooltip` when given. The last record for a
    /// cell in one write wins. The Cargo feature `metadata`
    /// (`specs/0019-names-links-properties-protection.md`, "Hyperlinks").
    ///
    /// The write drops a link the part has whose `ref` is exactly that cell
    /// (and the relationship its `r:id` names), and appends
    /// `<hyperlink ref=".." r:id=".." tooltip=".."/>` for an external target,
    /// whose relationship of type [`HYPERLINK`] it records on the part's rels
    /// part with `TargetMode="External"`, or
    /// `<hyperlink ref=".." location=".." tooltip=".."/>` for a location, at
    /// the end of `<hyperlinks>`, creating that element in schema position
    /// when the part has none. A root binding no prefix to the relationships
    /// namespace is retagged with one (decision 3 A). The cell itself, its
    /// value and style, is not touched; no `display` is written. The text is
    /// not parsed: a URL is written as given, escaped.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a hyperlink target or location is empty" for an
    /// empty target, "a text holds a character XML cannot carry" for a
    /// target or tooltip holding one, and "a row or column index or count is
    /// out of range" for a cell off the grid. Nothing is recorded when this
    /// fails. The write refuses a part with two `<hyperlinks>`, "a worksheet
    /// has two hyperlinks or sheetProtection elements"; a part with a row or
    /// column insertion or deletion in the same write, with part 3's
    /// one-shift message; and what a cell edit to the part would meet.
    #[cfg(feature = "metadata")]
    pub fn set_hyperlink(
        &mut self,
        part: &[u8],
        reference: CellRef,
        target: LinkTarget<'_>,
        tooltip: Option<&str>,
    ) -> Result<(), Error> {
        self.parts().set_hyperlink(part, reference, target, tooltip)
    }

    /// Records the removal of the hyperlink whose `ref` is exactly the cell
    /// `reference` in the worksheet part `part`, and of the relationship its
    /// `r:id` names; a `<hyperlinks>` left empty is dropped. A link whose
    /// `ref` is a range is never matched. The Cargo feature `metadata`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a row or column index or count is out of range"
    /// for a cell off the grid; nothing is recorded when this fails. The
    /// write refuses a cell the part has no link on, "no hyperlink on that
    /// cell", and what [`Package::set_hyperlink`] says it refuses.
    #[cfg(feature = "metadata")]
    pub fn remove_hyperlink(&mut self, part: &[u8], reference: CellRef) -> Result<(), Error> {
        self.parts().remove_hyperlink(part, reference)
    }

    /// The package's core properties: `docProps/core.xml` as the package's
    /// core-properties relationship names it, each of the thirteen
    /// [`Property`] elements read by namespace and local name, its text
    /// unescaped, every other child ignored; empty when the package has no
    /// such relationship. A snapshot of the file, not of the pending edits.
    /// The Cargo feature `metadata`
    /// (`specs/0019-names-links-properties-protection.md`, "Document
    /// properties").
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part" for a relationship whose part is
    /// missing, "a core properties part has two elements of one property",
    /// and whatever [`Stream`] refuses.
    #[cfg(feature = "metadata")]
    pub fn properties(&mut self) -> Result<Properties, Error> {
        self.parts().properties()
    }

    /// Records a core property's value, or its removal when `value` is
    /// `None`; the last record for a property wins. The write replaces the
    /// element's content in place (its start tag kept byte for byte), drops
    /// it, or appends it before the root's end tag with the prefix the root
    /// binds to its namespace; with no core-properties part it adds
    /// `docProps/core.xml`, its package relationship and its `Override`
    /// (decision 5 A). `app.xml` and `custom.xml` are never touched. The
    /// Cargo feature `metadata`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a property value is not valid" for a `Created`
    /// or `Modified` value not of the form `YYYY-MM-DDThh:mm:ssZ`, and "a
    /// text holds a character XML cannot carry" for a value holding one;
    /// nothing is recorded when this fails. The write refuses an append
    /// the root binds no prefix for, "a core properties part binds no prefix
    /// for a namespace the edit needs", a part with two elements of one
    /// property, and a core part this write removes or replaces, "an edit
    /// to a removed part".
    #[cfg(feature = "metadata")]
    pub fn set_property(&mut self, property: Property, value: Option<&str>) -> Result<(), Error> {
        self.parts().set_property(property, value)
    }

    /// Records the protection of the worksheet part `part`: `options` as
    /// its `<sheetProtection>` attributes and, when given, `password`'s
    /// [`legacy_password_hash`] as four upper-case hex digits; the last
    /// record for a part wins. The write replaces the part's element whole
    /// or creates one in schema position. The lock is Excel's advisory one,
    /// not encryption. The Cargo feature `metadata`
    /// (`specs/0019-names-links-properties-protection.md`, "Protection").
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a password is not valid" for a password that is
    /// not 1 to 255 printable ASCII characters; nothing is recorded when
    /// this fails. The write refuses a part with two `<sheetProtection>`, "a
    /// worksheet has two hyperlinks or sheetProtection elements", a part
    /// with a row or column shift in the same write, and what a cell edit
    /// to the part would meet.
    #[cfg(feature = "metadata")]
    pub fn protect_sheet(
        &mut self,
        part: &[u8],
        password: Option<&str>,
        options: SheetProtection,
    ) -> Result<(), Error> {
        self.parts().protect_sheet(part, password, options)
    }

    /// Records the removal of the worksheet part's `<sheetProtection>`;
    /// a part with none is left as it is. The Cargo feature `metadata`.
    ///
    /// # Errors
    ///
    /// None at the call; the write refuses what [`Package::protect_sheet`]
    /// says it refuses.
    #[cfg(feature = "metadata")]
    pub fn unprotect_sheet(&mut self, part: &[u8]) -> Result<(), Error> {
        self.edits.sheet_protections.push((part.to_vec(), None));
        Ok(())
    }

    /// Records the protection of the workbook: `lockStructure` when
    /// `structure`, `lockWindows` when `windows`, and `workbookPassword`
    /// when a password is given; an element with neither lock and no
    /// password is still written. The last record wins. The Cargo feature
    /// `metadata`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a password is not valid" as for
    /// [`Package::protect_sheet`]. The write refuses a workbook part with
    /// two `<workbookProtection>`, "a workbook has two definedNames or
    /// workbookProtection elements", and a workbook part this write
    /// replaces or removes.
    #[cfg(feature = "metadata")]
    pub fn protect_workbook(
        &mut self,
        password: Option<&str>,
        structure: bool,
        windows: bool,
    ) -> Result<(), Error> {
        let hash = match password {
            Some(password) if !password_ok(password) => return Err(PASSWORD_INVALID),
            Some(password) => Some(legacy_password_hash(password)),
            None => None,
        };
        self.edits
            .book_protections
            .push(Some((hash, structure, windows)));
        Ok(())
    }

    /// Records the removal of the workbook's `<workbookProtection>`; a
    /// workbook with none is left as it is. The Cargo feature `metadata`.
    #[cfg(feature = "metadata")]
    pub fn unprotect_workbook(&mut self) {
        self.edits.book_protections.push(None);
    }

    /// Records a data validation on `areas` of the worksheet part `part`
    /// (`part` as [`Package::set_cell`] names it). The Cargo feature `rules`
    /// (`specs/0020-validation-and-conditional-formatting.md`, "Data
    /// validation", and the P1 rulings).
    ///
    /// The write appends `<dataValidation type=".." operator=".."
    /// allowBlank=".." showDropDown="1" showInputMessage=".."
    /// showErrorMessage=".." errorStyle=".." errorTitle=".." error=".."
    /// promptTitle=".." prompt=".." sqref="..">` with its `<formula1>` and
    /// `<formula2>` to `<dataValidations>`, `operator` only when given,
    /// `showDropDown` only when the arrow is hidden, the messages' attributes
    /// only when a message is given, the element and its children taking the
    /// container's prefix, and sets the container's `count` to its children;
    /// an absent container is created in schema position. The cells are not
    /// touched. The areas are written as the `sqref` list, space-separated
    /// in the order given, each normalised.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: "a validation or a
    /// conditional format has no area" for no area; "a row or column index
    /// or count is out of range" for a corner off the grid; "a validation
    /// field does not apply to its kind" for an operator on `List` or
    /// `Custom`, or `hide_drop_down` on another kind than `List`; "a
    /// validation or a rule formula is empty or missing" for an empty
    /// formula, a `formula2` missing for `Between` or `NotBetween` (or no
    /// operator on a kind that takes one) or given with any other; "a
    /// formula text starts with an equals sign"; "a text holds a character
    /// XML cannot carry" for a formula or a message holding one; and "a
    /// validation message is too long" for a title past 32 characters or a
    /// text past 255. The write refuses an area overlapping a validation
    /// the part keeps or one this write adds before it, and a kept `sqref`
    /// that does not parse, "a validation overlaps one the worksheet has,
    /// or a dataValidation sqref does not parse"; a part with two
    /// `<dataValidations>`, "a worksheet has two dataValidations elements";
    /// a part with a row or column insertion or deletion in the same write,
    /// with part 3's one-shift message; and what a cell edit to the part
    /// would meet.
    #[cfg(feature = "rules")]
    pub fn add_validation(
        &mut self,
        part: &[u8],
        areas: &[Area],
        validation: Validation<'_>,
    ) -> Result<(), Error> {
        self.parts().add_validation(part, areas, validation)
    }

    /// Records the removal of every `<dataValidation>` of the worksheet
    /// part `part` whose `sqref`, read as a list, is `areas`, area for area
    /// in order; a `<dataValidations>` left empty is dropped. Only the
    /// file's validations are matched, never one this write adds. A removal
    /// and an addition of one list in one write replace the validation: the
    /// addition is checked against the part without the removed one. The
    /// Cargo feature `rules`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: "a validation or a
    /// conditional format has no area" and "a row or column index or count
    /// is out of range", as for [`Package::add_validation`]. The write
    /// refuses a list the part has no validation on, "no validation on those
    /// areas", and what [`Package::add_validation`] says it refuses.
    #[cfg(feature = "rules")]
    pub fn remove_validation(&mut self, part: &[u8], areas: &[Area]) -> Result<(), Error> {
        let areas = checked_areas(areas)?;
        self.edits
            .validations
            .push((part.to_vec(), ValidationEdit::Remove(areas)));
        Ok(())
    }

    /// The data validations of the worksheet part `part`, in part order,
    /// read from the file: a snapshot, not of the pending edits. Only the
    /// main namespace's `<dataValidations>` among the root's children is
    /// read; an `x14` extension's are not. The Cargo feature `rules`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part" for a part the archive lacks (a
    /// sheet added and not yet written among them); "a validation overlaps
    /// one the worksheet has, or a dataValidation sqref does not parse" for
    /// a `sqref` that does not parse or a `type`, `operator` or
    /// `errorStyle` the enumerations do not name; part 2's unbound-prefix
    /// refusal; and whatever [`Stream`] refuses.
    #[cfg(feature = "rules")]
    pub fn validations(&mut self, part: &[u8]) -> Result<Vec<ValidationEntry>, Error> {
        let index = self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        read_validations(self.archive.read(index)?, index)
    }

    /// Records a differential style, a `<dxf>` appended to the styles part's
    /// `<dxfs>`, and returns its index, which a conditional format's rule
    /// names: the part's `dxf` children plus the differential styles recorded
    /// before it (P2 ruling 3). The Cargo feature `rules`
    /// (`specs/0020-validation-and-conditional-formatting.md`,
    /// "Differential styles").
    ///
    /// The `<dxf>` holds the style's font, fill and border, in that order, as
    /// [`Package::add_style`] writes them, with no `apply` flags; nothing is
    /// deduplicated and a `dxf` is never removed. The write grows `<dxfs>` at
    /// its end, `count` set to its children, or creates it with the root's
    /// prefix before the first of `tableStyles`, `colors` and `extLst`, or
    /// before the root's end tag. Excel reads a differential solid fill's
    /// colour from `bgColor`, so a caller sets the fill's `background`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a differential style holds a field this part does
    /// not write" for a number format, an alignment or a protection, or for a
    /// style with none of a font, a fill and a border; the refusals of the
    /// three element writers; "the workbook has no styles part"; "a styles
    /// part has two dxfs elements"; and "a differential style index is out of
    /// range" for an index `u32` cannot hold. Nothing is recorded when this
    /// fails. The write refuses what a styles rewrite refuses, a second
    /// `<dxfs>`, and a styles part this write replaces, "a style addition to
    /// a part this write replaces".
    #[cfg(feature = "rules")]
    pub fn add_differential_style(&mut self, style: &Style) -> Result<u32, Error> {
        self.parts().add_differential_style(style)
    }

    /// Records a conditional format on `areas` of the worksheet part `part`:
    /// one `<conditionalFormatting sqref="..">` holding one `<cfRule>` of
    /// `rule`, styled by the differential style `dxf`, with `priority` when
    /// given and `stopIfTrue` when `stop_if_true`. The Cargo feature `rules`
    /// (`specs/0020-validation-and-conditional-formatting.md`, "Conditional
    /// formatting", and the P2 rulings).
    ///
    /// The write puts the additions, in record order and with the root's
    /// prefix, immediately before the first root child of the main
    /// namespace whose schema position follows `conditionalFormatting`'s,
    /// or before the root's end tag: after the last conditional format on a
    /// part in schema order. A rule with no formula is written as an empty
    /// element. A priority not given is the part's highest kept priority
    /// plus one, and one more for each earlier addition without one, as
    /// openpyxl assigns it; one given is written as given. The blank rules'
    /// formula is Excel's, built for the first area's top-left cell. The
    /// cells are not touched.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: "a validation or a
    /// conditional format has no area"; "a row or column index or count is
    /// out of range" for a corner off the grid, a `priority` of 0 or a
    /// `rank` outside 1 to 1000; "a conditional format rule and its
    /// differential style do not agree" for a rule with no `dxf`; "a
    /// validation or a rule formula is empty or missing" for an empty
    /// formula, or a `formula2` missing for `Between` or `NotBetween` or
    /// given with another operator; "a formula text starts with an equals
    /// sign"; and "a text holds a character XML cannot carry". The write
    /// refuses a `dxf` at or past the styles part's `dxf` children plus this
    /// write's differential styles, "a differential style index is out of
    /// range"; a kept `sqref` or `priority` that does not parse, "a
    /// conditionalFormatting sqref or a cfRule attribute does not parse"; a
    /// part with a row or column insertion or deletion in the same write,
    /// with part 3's one-shift message; and what a cell edit to the part
    /// would meet.
    #[cfg(feature = "rules")]
    pub fn add_conditional_format(
        &mut self,
        part: &[u8],
        areas: &[Area],
        rule: Rule<'_>,
        dxf: Option<u32>,
        priority: Option<u32>,
        stop_if_true: bool,
    ) -> Result<(), Error> {
        self.parts()
            .add_conditional_format(part, areas, rule, dxf, priority, stop_if_true)
    }

    /// Records the removal of every `<conditionalFormatting>` of the
    /// worksheet part `part` whose `sqref`, read as a list, is `areas`,
    /// area for area in order, with every rule it holds. Only the file's
    /// conditional formats are matched. The Cargo feature `rules`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: "a validation or a
    /// conditional format has no area" and "a row or column index or count
    /// is out of range". The write refuses a list the part has no
    /// conditional format on, "no conditional format on those areas"; one
    /// whose rule holds an `extLst`, "a conditional format has an extension
    /// this part does not remove"; and what
    /// [`Package::add_conditional_format`] says it refuses.
    #[cfg(feature = "rules")]
    pub fn remove_conditional_format(&mut self, part: &[u8], areas: &[Area]) -> Result<(), Error> {
        let areas = checked_areas(areas)?;
        self.edits
            .rules
            .push((part.to_vec(), RuleEdit::Remove(areas)));
        Ok(())
    }

    /// The conditional formats' rules of the worksheet part `part`, every
    /// `<cfRule>` of every main-namespace `<conditionalFormatting>` among the
    /// root's children, in part order, read from the file: a snapshot, not
    /// of the pending edits. A visual rule's children are not read back. The
    /// Cargo feature `rules`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part" for a part the archive lacks; "a
    /// conditionalFormatting sqref or a cfRule attribute does not parse" for
    /// a `sqref` or a `priority` that does not parse, an `operator` neither
    /// [`Operator`] nor ECMA-376 Part 1, 18.18.15 names, or a `dxfId` that is
    /// not a decimal `u32`;
    /// part 2's unbound-prefix refusal; and whatever [`Stream`] refuses.
    #[cfg(feature = "rules")]
    pub fn conditional_formats(&mut self, part: &[u8]) -> Result<Vec<RuleEntry>, Error> {
        let index = self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        read_rules(self.archive.read(index)?, index)
    }

    /// Records a table on `area` of the worksheet part `part` (`part` as
    /// [`Package::set_cell`] names it). The Cargo feature `tables`
    /// (`specs/0021-tables.md`, "The calls", and the P1 rulings).
    ///
    /// The call also records [`Package::set_cell`] of each column's name as
    /// text on the area's top row, left to right, as a caller's own edit
    /// would; the last record for a cell in one write wins. The write adds
    /// the table part `xl/tables/tableN.xml`, N the smallest positive
    /// integer no entry of the archive or of this write's additions has,
    /// ASCII case folded, an entry this write removes still counting as
    /// taken (P1 ruling 6), with the smallest `id` no table of the projected
    /// workbook carries, its `Override`, and a relationship of type
    /// [`TABLE`] on the part's rels part with the relative target, both in
    /// the form of the worksheet's own relationship in the workbook's rels
    /// part; appends
    /// `<tablePart r:id=".."/>` to the part's `<tableParts>`, `count` set to
    /// its children, creating that element in schema position when the part
    /// has none; and retags a root binding no prefix to the relationships
    /// namespace with one. The part holds the declaration on its own line
    /// and the root on one: `id`, `name` and `displayName` (both the name),
    /// `ref`, `totalsRowShown="0"`, an `<autoFilter>` of the same `ref`,
    /// one `<tableColumn>` per name with ids from 1, and the
    /// `<tableStyleInfo>` when a style is given.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: "a row or column
    /// index or count is out of range" for a corner off the grid; "a text
    /// holds a character XML cannot carry" for a name or a column name
    /// holding one; "a table name is not valid" for a name outside the rule
    /// (1 to 255 characters, the first a letter, `_` or `\`, the rest
    /// letters, digits, `.` or `_`, not a cell reference on the grid nor of
    /// the `R1C1` form, ignoring case, and not `C`, `c`, `R` or `r`); "a
    /// table column name is not
    /// valid" for a column name empty or past 255 characters, two equal
    /// ignoring ASCII case, or a list whose length is not the area's width;
    /// "a table style number is out of range"; and "a table area is too
    /// small" for an area of one row. The write refuses a name a table of the
    /// projected
    /// workbook, a `definedName` of the workbook part or an earlier
    /// addition has, ignoring ASCII case, "a table name is already in use";
    /// an area overlapping a table or a pivot table the part has, an
    /// earlier addition for the part, a merge the part keeps or this write
    /// adds, or the part's `<autoFilter>`, "a table overlaps a table, a
    /// pivot table, a merge or the autofilter"; a header cell whose `<c>`
    /// holds an `<f>`, "a table header cell holds a formula"; a table part
    /// or pivot table part it cannot read, "a table part or its
    /// relationship cannot be read"; a part with two `<tableParts>`, "a
    /// worksheet has two tableParts elements"; a sheet added in the same
    /// write, "no such part" (P1 ruling 2); a part with a row or column
    /// insertion or deletion in the same write, with part 3's one-shift
    /// message; and what a cell edit to the part would meet.
    #[cfg(feature = "tables")]
    pub fn add_table(&mut self, part: &[u8], area: Area, table: Table<'_>) -> Result<(), Error> {
        self.parts().add_table(part, area, table)
    }

    /// Records the removal of the table named `name`, compared ignoring ASCII
    /// case against each table's `name` and `displayName`, from the
    /// worksheet part `part`. The Cargo feature `tables`
    /// (`specs/0021-tables.md`, "The calls").
    ///
    /// The write drops the part's `<tablePart>` whose `r:id` names the
    /// table's relationship, the relationship, the table part's `Override`
    /// when the content types part has one, and the table part itself; a
    /// `<tableParts>` left empty is dropped. The cells keep their values and
    /// styles, as Excel's "Convert to Range" keeps them, and a differential
    /// style the table named stays. A table added and a table removed in one
    /// write are checked against the workbook with the removal applied, so
    /// a removal and an addition of one name replace the table; the removed
    /// part's entry name is not reused.
    ///
    /// # Errors
    ///
    /// None at the call. The write refuses a name no table of the part's
    /// file carries, a name this write adds among them (the caller writes
    /// twice), "no table of that name"; a table part or relationship it
    /// cannot read, "a table part or its relationship cannot be read"; and
    /// what [`Package::add_table`] says it refuses of the part.
    #[cfg(feature = "tables")]
    pub fn remove_table(&mut self, part: &[u8], name: &str) -> Result<(), Error> {
        self.edits
            .table_removals
            .push((part.to_vec(), name.to_string()));
        Ok(())
    }

    /// The tables of the worksheet part `part`: each `<tablePart>` of its
    /// `<tableParts>`, in part order, its `r:id` resolved through the part's
    /// rels part and its table part read, a snapshot of the file, not of the
    /// pending edits; empty for a part with no `<tableParts>`. The Cargo
    /// feature `tables`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part" for a part the archive lacks; "a
    /// table part or its relationship cannot be read" for a `tablePart`
    /// with no `r:id`, an `r:id` with no relationship, a relationship whose
    /// part is missing, a root with no `id`, `name`, `displayName` or
    /// `ref`, or a `ref` that does not parse; part 2's unbound-prefix
    /// refusal; and whatever [`Stream`] refuses.
    #[cfg(feature = "tables")]
    pub fn tables(&mut self, part: &[u8]) -> Result<Vec<TableEntry>, Error> {
        self.parts().tables(part)
    }

    /// Records a comment with `comment`'s author and text on `cell` of the
    /// worksheet part `part`. The Cargo feature `comments`
    /// (`specs/0022-comments-and-images.md`, "Comments: the calls").
    ///
    /// The write appends a `<comment>` to the sheet's comments part, its
    /// author to `<authors>` unless a kept author's text equals it, and a
    /// hidden note shape at the cell to the sheet's VML part; a part the
    /// sheet lacks is created, `xl/comments<N>.xml` or
    /// `xl/drawings/vmlDrawing<N>.vml` with N the smallest positive integer no
    /// entry of the archive or of this write's additions has, ASCII case
    /// folded, an entry this write removes still counting as taken, with its
    /// `Override` and a relationship of type [`COMMENTS`] or [`VML_DRAWING`]
    /// on the part's rels part with the relative target, in the form of the
    /// worksheet's own relationship; and creates `<legacyDrawing r:id=".."/>`
    /// in schema position when the sheet has none, retagging a root binding
    /// no prefix to the relationships namespace with one. A kept comment on
    /// the cell is replaced, its element and its shape skipped; of two
    /// additions on one cell in one write the last wins.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: "a row or column
    /// index or count is out of range" for a cell off the grid; "a text
    /// holds a character XML cannot carry" for an author or a text holding
    /// one; "a comment author is not valid" for an author empty or past 255
    /// characters; "a comment text is not valid" for a text empty or past
    /// 32767 characters. The write refuses a sheet with a relationship of
    /// type [`THREADED_COMMENTS`], "a worksheet has threaded comments"; a
    /// part with two `<legacyDrawing>` elements, or one out of schema order,
    /// "a worksheet has two legacyDrawing elements"; a comments
    /// relationship whose part is missing, a comments part whose root has
    /// no `commentList` or two, a kept `comment` whose `ref` is absent or
    /// does not parse, a `legacyDrawing` whose relationship or part is
    /// missing, a comments part on a sheet with no `legacyDrawing`, a VML
    /// part holding no shape where the comments part holds a comment, one
    /// whose root's end tag cannot be found, or whose root binds no prefix
    /// to one of the three VML namespaces, "a comments part or its drawing
    /// cannot be read"; a sheet added in the same write, "no such part"; a
    /// part this write removes or replaces with part 1's messages for a
    /// cell edit; and a part with a row or column insertion or deletion in
    /// the same write, with part 3's one-shift message.
    #[cfg(feature = "comments")]
    pub fn add_comment(
        &mut self,
        part: &[u8],
        cell: CellRef,
        comment: Comment<'_>,
    ) -> Result<(), Error> {
        self.parts().add_comment(part, cell, comment)
    }

    /// Records a comment by `author` whose text is `text`'s runs on `cell` of
    /// the worksheet part `part`. The Cargo features `comments` and
    /// `richtext` (`specs/0024-rich-text.md`, "Comments").
    ///
    /// [`Package::add_comment`] with the comment's `<text>` holding one `<r>`
    /// per run, in order: the run's `<rPr>` first when it has a font, written
    /// as a rich cell value's is, then its `<t>`, with `xml:space="preserve"`
    /// when the run's text begins or ends with whitespace. The author rule,
    /// the replacement of a kept comment, the parts, the note shape and
    /// [`Package::remove_comment`] are `add_comment`'s.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: what
    /// [`Package::add_comment`] refuses at the call, the text rules applied
    /// to the runs: "a text holds a character XML cannot carry" for a run's
    /// text holding one; "a comment text is not valid" for no run, a run
    /// with no text, or runs whose texts together pass 32767 characters; and
    /// a run's font [`Package::add_style`] would refuse, with its messages.
    /// The write refuses what [`Package::add_comment`] says it refuses.
    #[cfg(all(feature = "comments", feature = "richtext"))]
    pub fn add_rich_comment(
        &mut self,
        part: &[u8],
        cell: CellRef,
        author: &str,
        text: &RichText,
    ) -> Result<(), Error> {
        self.parts().add_rich_comment(part, cell, author, text)
    }

    /// Records the removal of the comment on `cell` of the worksheet part
    /// `part`. The Cargo feature `comments` (`specs/0022-comments-and-
    /// images.md`, "Comments: the calls").
    ///
    /// The write skips the `<comment>` whose `ref` is the cell and, in the
    /// VML part, the note shape (`ClientData` of `ObjectType` `Note`) whose
    /// `Row` and `Column` are the cell's, zero-based. A `<commentList>` left
    /// empty drops the comments part, its relationship, its `Override` and
    /// the entry; the VML part goes with it, with its relationship, its
    /// `Override` and the worksheet's `<legacyDrawing>`, unless it still
    /// holds a shape of another `ObjectType`, a form control, when it is kept
    /// with that shape. Authors are never removed. A comment added on the
    /// cell after its removal in the same write replaces it.
    ///
    /// # Errors
    ///
    /// None at the call. The write refuses a cell with no comment in the
    /// part's file, a cell this write adds a comment on before the removal,
    /// and a cell named twice for removal, "no comment on that cell" (the
    /// caller writes twice); and what [`Package::add_comment`] says it
    /// refuses of the part.
    #[cfg(feature = "comments")]
    pub fn remove_comment(&mut self, part: &[u8], cell: CellRef) -> Result<(), Error> {
        self.parts().remove_comment(part, cell)
    }

    /// The comments of the worksheet part `part`: each `<comment>` of the
    /// comments part its `comments` relationship names, in document order,
    /// with its cell, its author and its text; a snapshot of the file, not of
    /// the pending edits; empty for a part with no such relationship. A
    /// threaded comment reads as the legacy fallback Excel writes for it.
    /// The Cargo feature `comments`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part" for a part the archive lacks; "a
    /// comments part or its drawing cannot be read" for a relationship whose
    /// part is missing, a root other than `comments` or without
    /// `commentList`, and a `comment` without `ref` or `authorId` or whose
    /// `ref` or `authorId` does not parse; part 2's unbound-prefix refusal;
    /// and whatever [`Stream`] refuses.
    #[cfg(feature = "comments")]
    pub fn comments(&mut self, part: &[u8]) -> Result<Vec<CommentEntry>, Error> {
        self.parts().comments(part)
    }

    /// Records `image` on the worksheet part `part`, anchored at `cell`'s
    /// top-left corner. The Cargo feature `images`
    /// (`specs/0022-comments-and-images.md`, "Images: the calls").
    ///
    /// The write adds the bytes, unchanged, as the media entry
    /// `xl/media/image<N>.png` or `.jpeg`, N the smallest positive integer
    /// with neither name taken by an entry of the archive or of this write's
    /// additions, ASCII case folded, an entry this write removes still
    /// counting, with its `Override`; and appends a one-cell anchor holding
    /// the picture, its `cNvPr` `id` one above the part's largest, before the
    /// end tag of the sheet's drawing part, with a relationship of type
    /// [`IMAGE`] on the drawing's rels part, created when it has none. A
    /// sheet with no drawing part gets `xl/drawings/drawing<N>.xml` by the
    /// same allocation, with its `Override` and its rels part, a relationship
    /// of type [`DRAWING`] on the sheet's rels part with the relative target,
    /// in the form of the worksheet's own relationship, and
    /// `<drawing r:id=".."/>` created in schema position, retagging a root
    /// binding no prefix to the relationships namespace with one. The extent
    /// is the size times 9525 EMU per pixel. Images are never checked for
    /// overlap.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] at the call, nothing recorded: "a row or column
    /// index or count is out of range" for a cell off the grid; "an image is
    /// not a PNG or JPEG" for bytes that are neither a PNG of 24 bytes or
    /// more whose first chunk is `IHDR` nor a JPEG with a baseline, extended
    /// or progressive `SOF` before its scan; "an image size is out of range"
    /// for a width or height of 0 or past `i32::MAX` EMU. The write refuses
    /// a part with two `<drawing>` elements, or one out of schema order, "a
    /// worksheet has two drawing elements"; a `<drawing>` without `r:id` or
    /// naming no drawing relationship, a relationship whose part is missing,
    /// and a drawing part whose root is not `wsDr`, or is empty, or that
    /// holds a `cNvPr` `id` that is not a number or leaves none below
    /// `u32::MAX`, "a drawing part or its relationship cannot be read"; a
    /// sheet added in the same write, "no such part"; a part this write
    /// removes or replaces, or whose drawing part it removes, replaces or
    /// adds through the part calls, with part 1's messages for a cell edit;
    /// and a part with a row or column insertion or deletion in the same
    /// write, with part 3's one-shift message.
    #[cfg(feature = "images")]
    pub fn add_image(&mut self, part: &[u8], cell: CellRef, image: Image<'_>) -> Result<(), Error> {
        self.parts().add_image(part, cell, image)
    }

    /// The images of the worksheet part `part`: each root child of the
    /// drawing part its `<drawing>` element's `r:id` names, as the write
    /// finds it (P3 verification ruling 1 (2)), in document order,
    /// that is a `oneCellAnchor`, `twoCellAnchor` or `absoluteAnchor`, or
    /// the first such anchor in a root `AlternateContent`'s first `Choice`
    /// (P3 ruling 1), holding a `pic` directly, in a `grpSp` or in an
    /// `AlternateContent`'s first `Choice`, the first found; with the
    /// anchor's `from`, its kind, and the media entry the `blip`'s `r:embed`
    /// names with its size. A snapshot of the file, not of the pending
    /// edits; empty for a part with no `<drawing>`; an anchor without
    /// a picture (a chart, a shape) gives none. The Cargo feature `images`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "no such part" for a part the archive lacks; "a
    /// worksheet has two drawing elements" for two, or one out of schema
    /// order; "a drawing part or its relationship cannot be read" for a
    /// `<drawing>` without `r:id` or naming no drawing relationship, a
    /// relationship whose part is missing, a root other than `wsDr`, a
    /// picture with no
    /// `blip` or a `blip` without `r:embed`, an `r:embed` naming no internal
    /// relationship or one whose entry is missing, and a one- or two-cell
    /// anchor whose `from` lacks a `col` or `row` number; part 2's
    /// unbound-prefix refusal; and whatever [`Stream`] refuses.
    #[cfg(feature = "images")]
    pub fn images(&mut self, part: &[u8]) -> Result<Vec<ImageEntry>, Error> {
        self.parts().images(part)
    }

    /// Records that the workbook part gets `fullCalcOnLoad="1"` on its
    /// `<calcPr>`, asking an application to recalculate every formula when
    /// it opens the file.
    ///
    /// `specs/0013-formulas.md`, "Module `package`". Nothing is read and
    /// nothing can fail here, and recording it twice records it once.
    /// [`Package::write_to`] records it itself for a write that changes a
    /// formula; a caller whose edits changed an input of existing formulas
    /// and no formula calls this, since emend cannot know which formulas
    /// depend on which cells. Whether an application honours the request is
    /// the application's: nothing is evaluated here.
    pub fn recalculate_on_open(&mut self) {
        self.parts().recalculate_on_open()
    }

    /// The workbook's sheets with the recorded sheet edits applied, in the
    /// order they were recorded (`specs/0016-new-workbook-and-sheets.md`,
    /// "The projected sheet list").
    ///
    /// [`Package::workbook`] keeps answering with the file's list, as
    /// [`Package::cells`] reads the file and not the pending cell edits. A
    /// sheet added and not yet written has `part`, `sheet_id` and `rel_id` set
    /// to the values it will get, `state` `None` and `kind`
    /// [`SheetKind::Worksheet`].
    ///
    /// # Errors
    ///
    /// Whatever [`Package::workbook`] refuses.
    pub fn sheets(&mut self) -> Result<Vec<Sheet>, Error> {
        self.parts().sheets()
    }

    /// Records a new worksheet named `name` at the end of the list, and
    /// returns its entry name (`specs/0016-new-workbook-and-sheets.md`,
    /// "Adding a sheet").
    ///
    /// Recorded in the order the write applies them: the entry, the
    /// template's blank worksheet, at `<dir>/worksheets/sheet<n>.xml`, `<dir>`
    /// the workbook part's directory and `n` the smallest positive integer
    /// whose name is neither an entry nor a pending addition; an `Override`
    /// for it with the worksheet content type, unless the content types
    /// part has a `Default` for `xml` of that type; a worksheet relationship
    /// on the workbook part, in the form of the workbook's first worksheet
    /// relationship or transitional when there is none, its target relative
    /// to the workbook part; and a `<sheet>` at the end of the projected list
    /// with `sheetId` one more than the largest there. A caller places it with
    /// [`Package::move_sheet`], and writes into it with [`Package::set_cell`]
    /// on the returned name (P3 ruling 2).
    ///
    /// # Errors
    ///
    /// The name rules' messages ("a sheet name is empty", "a sheet name is
    /// longer than 31 characters", "a sheet name holds a character Excel
    /// refuses", "a sheet name starts or ends with an apostrophe", "a text
    /// holds a character XML cannot carry", "a sheet name is the reserved
    /// name History", "a sheet name is already in use"); "a sheet edit no
    /// longer applies at the write" for a `sheetId` past `u32::MAX`; and
    /// whatever reading the workbook, its relationships and the content
    /// types refuses. Nothing is recorded when this fails.
    pub fn add_sheet(&mut self, name: &str) -> Result<Vec<u8>, Error> {
        self.parts().add_sheet(name)
    }

    /// Records that the sheet named `name` takes the name `to`
    /// (`specs/0016-new-workbook-and-sheets.md`, "Renaming and moving").
    ///
    /// `to` is checked as [`Package::add_sheet`] checks a name, against every
    /// other sheet of the projected list, so a sheet may be renamed to its
    /// own name or to its own name in another case. Formulas, defined names
    /// and anything else that names the sheet in text are not touched.
    ///
    /// # Errors
    ///
    /// "no sheet of that name" when the projected list has no sheet named
    /// exactly `name`; the name rules' messages for `to`; whatever
    /// [`Package::workbook`] refuses. Nothing is recorded when this fails.
    pub fn rename_sheet(&mut self, name: &str, to: &str) -> Result<(), Error> {
        self.parts().rename_sheet(name, to)
    }

    /// Records that the sheet named `name` moves to position `to`, 0 first
    /// (`specs/0016-new-workbook-and-sheets.md`, "Renaming and moving"). A
    /// move to the sheet's own position records nothing.
    ///
    /// # Errors
    ///
    /// "no sheet of that name"; "a sheet position is past the end" for `to`
    /// at or past the projected list's length; whatever
    /// [`Package::workbook`] refuses. Nothing is recorded when this fails.
    pub fn move_sheet(&mut self, name: &str, to: usize) -> Result<(), Error> {
        self.parts().move_sheet(name, to)
    }

    /// Records the removal of the sheet named `name`
    /// (`specs/0016-new-workbook-and-sheets.md`, "Removing a sheet", and P4
    /// rulings 1 to 5 and 9).
    ///
    /// Recorded in this order: the `<sheet>` gone from the projected list;
    /// the workbook relationship its `r:id` names removed; the walk's
    /// removals; and, when the workbook has a `calcChain` relationship, the
    /// recipe of [`Package::drop_calc_chain`], recorded once however many
    /// edits ask for it, and a full calculation on load as
    /// [`Package::recalculate_on_open`] records it.
    ///
    /// The walk: R is every entry reachable from the sheet's part through
    /// internal relationships, transitively, the part included; K is every
    /// entry reachable the same way from the package's relationships, with
    /// every relationship recorded for removal and this sheet's left out,
    /// and no edge followed out of a removed sheet's part. Each entry of R
    /// not in K and not already recorded for removal is removed, with its
    /// own rels part when the package holds one, and each one's `Override`
    /// when the content types part holds one, so no `Override` names a
    /// removed entry (P4 ruling 9). A target outside the package or naming
    /// no entry contributes nothing, and an external relationship is never
    /// followed.
    ///
    /// A sheet added and not yet written leaves the list, and its part,
    /// `Override` and relationship additions and the cell and style edits
    /// recorded on its part are dropped: nothing is written for it (P4
    /// ruling 4).
    ///
    /// # Errors
    ///
    /// "no sheet of that name"; "the last sheet cannot be removed" when the
    /// projected list has one sheet; "the last visible sheet cannot be
    /// removed" when every other sheet is hidden (P4 ruling 1); whatever
    /// reading the workbook, a reached rels part or the content types part
    /// refuses, and for the chain whatever [`Relationships::target_part`]
    /// refuses. Nothing is recorded when this fails.
    pub fn remove_sheet(&mut self, name: &str) -> Result<(), Error> {
        self.parts().remove_sheet(name)
    }

    /// Writes the package back with every recorded edit applied.
    ///
    /// Every edit is validated first, so a refused set writes nothing. Then
    /// each worksheet with cell edits is streamed into a temporary file once,
    /// with all of its edits, as `specs/0011-editing-cells.md`, "Module
    /// `package`", describes; a refusal found while a part streams also
    /// writes nothing. When the write changes formulas (the last value edit
    /// for some reference is a [`Value::Formula`], or an edit replaced or
    /// cleared a cell whose formula was a normal one), the calculation chain
    /// is dropped in the same write through [`Package::drop_calc_chain`],
    /// unless its relationship is already recorded for removal, a full
    /// calculation on load is recorded as [`Package::recalculate_on_open`]
    /// records it, and the set is validated again (`specs/0013-formulas.md`).
    /// The styles part is rewritten when a cell format was added, and the
    /// workbook part when the full calculation is recorded, its `calcPr`
    /// start tag alone changing. Then
    /// `[Content_Types].xml` and each rels part with edits are read through
    /// [`Stream`] and rewritten into a temporary file: every token's `raw` is
    /// copied except a removed element's own bytes, so the whitespace around
    /// one stays, and additions go immediately before the root's end tag with
    /// no whitespace added. The temporaries become the entries'
    /// replacements, [`Archive::write_to`] runs with the assembled
    /// [`Edits`], and the temporaries are removed however the write ends.
    ///
    /// Takes the package by value (P3 ruling 12): a package is written at
    /// most once, because a recorded [`Replacement`] is a reader that cannot
    /// be read twice or cloned, and a refused or failed write spends it too.
    /// A caller that wants to write again opens the input again, which the
    /// type enforces rather than a refusal at run time.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] for an edit that cannot apply: "no such
    /// relationship", "no such override", "an override for that part already
    /// exists", "an edit to a removed part", "no such part", "no Default
    /// covers rels", and for a cell edit "a number that is not finite cannot
    /// be written", "a text holds a character XML cannot carry", "a cell
    /// edit to a part this write replaces", "the worksheet part's root is not
    /// worksheet", "a worksheet without explicit references cannot be
    /// edited", "a worksheet whose rows or cells are out of order cannot be
    /// edited", "a cell in a shared, array or data table formula cannot be
    /// edited", "a cell carries an attribute this layer does not model",
    /// "a worksheet without sheetData cannot be edited", and three the
    /// `cells` module raises too: "a cell reference is not of the form A1",
    /// "a row number is not a number" and, for an edited cell, "a formula
    /// has a type this layer does not know". For a style edit or addition
    /// (`specs/0012-styles.md`): "a style index is out of range", "the
    /// workbook has no styles part" and whatever else reading the styles
    /// part refuses, "a style addition to a part this write replaces", "a
    /// styles part without cellXfs cannot take a style" and, for a `count`
    /// it rewrites, "a style attribute is not a number". For a formula
    /// (`specs/0013-formulas.md`): "a formula text is empty", "a formula
    /// text starts with an equals sign", "a formula text holds a character
    /// XML cannot carry", and for the workbook rewrite "a workbook has two
    /// calcPr elements" and "the workbook part cannot be edited in a package
    /// that replaces or removes it".
    /// [`Error::Xml`] for a part name or value that is not UTF-8, carrying
    /// the index of the part the value would have been written into.
    /// Whatever [`Stream`] refuses for a part it rewrites, and whatever
    /// [`Archive::write_to`] refuses for the assembled edits.
    pub fn write_to(mut self, mut out: impl Write) -> Result<(), Error> {
        self.parts().write_to(&mut out)
    }

    /// The package's fields as one view, over the archive's erased core.
    fn parts(&mut self) -> Parts<'_> {
        Parts {
            archive: &mut self.archive.core,
            content_types: &mut self.content_types,
            relationships: &mut self.relationships,
            workbook: &mut self.workbook,
            calc_prs: &mut self.calc_prs,
            styles: &mut self.styles,
            edits: &mut self.edits,
        }
    }
}

/// A [`Package`]'s fields, borrowed, with the archive as its erased core.
///
/// Every method body of [`Package`] lives here, so it is compiled once
/// however many reader types a consumer uses; the methods of [`Package`]
/// build this view and forward (`specs/0016-new-workbook-and-sheets.md`,
/// "P5" and "Planning rulings during P5", ruling 1).
struct Parts<'a> {
    archive: &'a mut Core,
    content_types: &'a mut Option<ContentTypes>,
    relationships: &'a mut Vec<Relationships>,
    workbook: &'a mut Option<Workbook>,
    calc_prs: &'a mut usize,
    styles: &'a mut Option<Styles>,
    edits: &'a mut Recorded,
}

impl<'a> Parts<'a> {
    /// [`Package::part`], over the view.
    fn part(&self, name: &[u8]) -> Option<usize> {
        self.archive.index_of(name)
    }

    /// The body of [`Package::shared_strings`].
    fn shared_strings(&mut self) -> Result<SharedStrings, Error> {
        let part = match self.workbook()?.shared_strings.clone() {
            Some(part) => part,
            None => return Ok(SharedStrings::default()),
        };
        let Some(index) = self.archive.index_of(&part) else {
            return Err(NO_SUCH_PART);
        };
        let mut stream = Stream::new(self.archive.read(index)?, index)?;
        SharedStrings::read(&mut stream, index)
    }

    /// The body of [`Package::content_types`].
    fn content_types(&mut self) -> Result<&ContentTypes, Error> {
        if self.content_types.is_none() {
            let index = self
                .archive
                .index_of(CONTENT_TYPES_PART)
                .ok_or(NO_CONTENT_TYPES)?;
            let parsed = parse_content_types(self.archive, index)?;
            return Ok(self.content_types.insert(parsed));
        }
        // The branch above returns whenever the field was empty.
        Ok(self.content_types.as_ref().unwrap())
    }

    /// The body of [`Package::relationships`].
    fn relationships(&mut self, source: &[u8]) -> Result<&Relationships, Error> {
        if let Some(at) = self
            .relationships
            .iter()
            .position(|rels| rels.source == source)
        {
            return Ok(&self.relationships[at]);
        }
        let part = rels_part_name(source);
        let parsed = match self.archive.index_of(&part) {
            Some(index) => parse_relationships(self.archive, index, source.to_vec())?,
            None => Relationships {
                source: source.to_vec(),
                list: Vec::new(),
            },
        };
        self.relationships.push(parsed);
        // The push above leaves at least one.
        Ok(self.relationships.last().unwrap())
    }

    /// The body of [`Package::workbook`].
    fn workbook(&mut self) -> Result<&Workbook, Error> {
        if self.workbook.is_none() {
            let parsed = self.parse_workbook()?;
            return Ok(self.workbook.insert(parsed));
        }
        // The branch above returns whenever the field was empty.
        Ok(self.workbook.as_ref().unwrap())
    }

    /// [`Package::workbook`]'s work, once, with nothing cached yet.
    fn parse_workbook(&mut self) -> Result<Workbook, Error> {
        let part = {
            let rels = self.relationships(b"")?;
            let found: Vec<&Relationship> = OFFICE_DOCUMENT
                .into_iter()
                .flat_map(|form| rels.of_kind(form))
                .collect();
            match found.as_slice() {
                [] => return Err(Error::Package("no officeDocument relationship")),
                [rel] => rels.target_part(rel)?,
                // Both forms are counted together: a package that carries a
                // transitional and a strict officeDocument relationship does
                // not say which document is the workbook.
                _ => return Err(Error::Package("more than one officeDocument relationship")),
            }
        };
        let index = self.archive.index_of(&part).ok_or(Error::Package(
            "the officeDocument relationship names no entry",
        ))?;
        let read = parse_workbook_sheets(self.archive, index)?;
        *self.calc_prs = read.calc_prs;
        let rels = self.relationships(&part)?;
        let mut sheets = Vec::with_capacity(read.sheets.len());
        for sheet in read.sheets {
            let rel = rels
                .by_id(&sheet.rel_id)
                .ok_or(Error::Package("a sheet's relationship is missing"))?;
            sheets.push(Sheet {
                name: sheet.name,
                sheet_id: sheet.sheet_id,
                rel_id: sheet.rel_id,
                state: sheet.state,
                part: rels.target_part(rel)?,
                kind: sheet_kind(&rel.kind),
            });
        }
        Ok(Workbook {
            part,
            sheets,
            shared_strings: resolved_first(rels, SHARED_STRINGS)?,
            styles: resolved_first(rels, STYLES)?,
            calc_chain: resolved_first(rels, CALC_CHAIN)?,
            theme: resolved_first(rels, THEME)?,
            date1904: read.date1904,
            full_calc_on_load: read.full_calc_on_load,
            #[cfg(feature = "metadata")]
            defined_names: read.defined_names,
            #[cfg(feature = "metadata")]
            defined_names_containers: read.defined_names_containers,
            #[cfg(feature = "metadata")]
            workbook_protections: read.workbook_protections,
        })
    }

    /// The body of [`Package::styles`].
    fn styles(&mut self) -> Result<&Styles, Error> {
        if self.styles.is_none() {
            let part = self.workbook()?.styles.clone().ok_or(NO_STYLES_PART)?;
            let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
            let parsed = {
                let mut stream = Stream::new(self.archive.read(index)?, index)?;
                Styles::read(&mut stream, index)?
            };
            return Ok(self.styles.insert(parsed));
        }
        // The branch above returns whenever the field was empty.
        Ok(self.styles.as_ref().unwrap())
    }

    /// The body of [`Package::remove_relationship`].
    fn remove_relationship(&mut self, source: &[u8], id: &[u8]) {
        self.edits.rel_removals.push((source.to_vec(), id.to_vec()));
    }

    /// The body of [`Package::add_relationship`].
    fn add_relationship(
        &mut self,
        source: &[u8],
        kind: &[u8],
        target: &[u8],
        external: bool,
    ) -> Result<Vec<u8>, Error> {
        let mut highest = 0u32;
        for id in self
            .relationships(source)?
            .all()
            .iter()
            .map(|rel| rel.id.clone())
            .collect::<Vec<Vec<u8>>>()
        {
            highest = highest.max(numbered_id(&id).unwrap_or(0));
        }
        for (at, rel) in &self.edits.rel_additions {
            if at == source {
                highest = highest.max(numbered_id(&rel.id).unwrap_or(0));
            }
        }
        let mut id = b"rId".to_vec();
        id.extend_from_slice((highest + 1).to_string().as_bytes());
        self.edits.rel_additions.push((
            source.to_vec(),
            Relationship {
                id: id.clone(),
                kind: kind.to_vec(),
                target: target.to_vec(),
                external,
            },
        ));
        Ok(id)
    }

    /// The body of [`Package::remove_override`].
    fn remove_override(&mut self, part_name: &[u8]) {
        self.edits.override_removals.push(part_name.to_vec());
    }

    /// The body of [`Package::add_override`].
    fn add_override(&mut self, part_name: &[u8], content_type: &[u8]) {
        self.edits
            .override_additions
            .push((part_name.to_vec(), content_type.to_vec()));
    }

    /// The body of [`Package::remove_part`].
    fn remove_part(&mut self, name: &[u8]) {
        self.edits.part_removals.push(name.to_vec());
    }

    /// The body of [`Package::replace_part`].
    fn replace_part(&mut self, name: &[u8], replacement: Replacement) {
        self.edits
            .part_replacements
            .push((name.to_vec(), replacement));
    }

    /// The body of [`Package::add_part`].
    fn add_part(&mut self, name: &[u8], replacement: Replacement) {
        self.edits.part_additions.push((name.to_vec(), replacement));
    }

    /// The body of [`Package::drop_calc_chain`].
    fn drop_calc_chain(&mut self) -> Result<bool, Error> {
        match self.chain_removals()? {
            Some(chain) => {
                self.record_chain_removals(chain);
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// What the chain recipe removes, read before any of it is recorded:
    /// each read borrows the package, and recording takes it mutably. `None`
    /// when the workbook has no `calcChain` relationship.
    fn chain_removals(&mut self) -> Result<Option<ChainRemovals>, Error> {
        let workbook = self.workbook()?.part.clone();
        let (id, part) = {
            let rels = self.relationships(&workbook)?;
            match rels.first_of(CALC_CHAIN) {
                Some(rel) => (rel.id.clone(), rels.target_part(rel)?),
                None => return Ok(None),
            }
        };
        let over = self.override_of(&part)?;
        let entry = self.part(&part).is_some().then_some(part);
        Ok(Some(ChainRemovals {
            workbook,
            id,
            over,
            entry,
        }))
    }

    /// Records the chain recipe's removals, in the recipe's order, skipping
    /// an override or entry already recorded for removal.
    fn record_chain_removals(&mut self, chain: ChainRemovals) {
        self.remove_relationship(&chain.workbook, &chain.id);
        if let Some(over) = chain.over
            && !self.edits.override_removals.contains(&over)
        {
            self.remove_override(&over);
        }
        if let Some(entry) = chain.entry
            && !self.edits.part_removals.contains(&entry)
        {
            self.remove_part(&entry);
        }
    }

    /// The `PartName` of the `Override` for the entry `part`, as the content
    /// types part writes it, leading slash and all, when it holds one.
    fn override_of(&mut self, part: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        Ok(self
            .content_types()?
            .overrides()
            .iter()
            .find(|over| {
                over.part_name
                    .strip_prefix(b"/".as_slice())
                    .unwrap_or(&over.part_name)
                    == part
            })
            .map(|over| over.part_name.clone()))
    }

    /// The body of [`Package::set_cell`].
    fn set_cell(&mut self, part: &[u8], reference: CellRef, value: Value) {
        self.edits
            .cell_edits
            .push((part.to_vec(), reference, Some(value)));
    }

    /// The body of [`Package::clear_cell`].
    fn clear_cell(&mut self, part: &[u8], reference: CellRef) {
        self.edits.cell_edits.push((part.to_vec(), reference, None));
    }

    /// The body of [`Package::add_style`].
    fn add_style(&mut self, base: Option<u32>, style: &Style) -> Result<u32, Error> {
        let recorded = self.edits.xf_additions.len();
        self.styles()?;
        // A borrow of the field alone, so the recorded edits stay readable.
        let Some(styles) = self.styles.as_ref() else {
            return Err(NO_STYLES_PART);
        };
        let existing = styles.cell_xfs.len();
        if let Some(base) = base
            && usize::try_from(base).map_or(true, |base| base >= existing)
        {
            return Err(STYLE_OUT_OF_RANGE);
        }
        let (num_fmt, new_code) = match &style.number_format {
            None => (None, None),
            Some(NumberFormat::BuiltIn(id)) if *id >= 164 => return Err(BUILT_IN_164),
            Some(NumberFormat::BuiltIn(id)) => (Some(*id), None),
            Some(NumberFormat::Code(code)) => {
                if code.is_empty() || code.chars().any(forbidden_in_xml) {
                    return Err(CODE_REFUSED);
                }
                let defined = styles
                    .num_fmts
                    .iter()
                    .map(|format| (format.id, &format.code))
                    .chain(
                        self.edits
                            .num_fmt_additions
                            .iter()
                            .map(|(id, code)| (*id, code)),
                    )
                    .find(|(_, have)| *have == code)
                    .map(|(id, _)| id);
                match defined {
                    Some(id) => (Some(id), None),
                    None => {
                        let mut id = 164u32;
                        for have in styles
                            .num_fmts
                            .iter()
                            .map(|format| format.id)
                            .chain(self.edits.num_fmt_additions.iter().map(|(id, _)| *id))
                        {
                            id = id.max(have.checked_add(1).ok_or(STYLE_OUT_OF_RANGE)?);
                        }
                        (Some(id), Some(code.clone()))
                    }
                }
            }
        };
        // Each of the three: its id, and its element when it is new.
        let font = match &style.font {
            None => None,
            Some(font) => {
                let recorded = &self.edits.font_additions;
                let position = recorded.iter().position(|(have, _)| have == font);
                let id = collection_id(styles.fonts(), position, recorded.len())?;
                Some((id, position.is_none().then(|| font.compose()).transpose()?))
            }
        };
        let fill = match &style.fill {
            None => None,
            Some(fill) => {
                let recorded = &self.edits.fill_additions;
                let position = recorded.iter().position(|(have, _)| have == fill);
                let id = collection_id(styles.fills(), position, recorded.len())?;
                Some((id, position.is_none().then(|| fill.compose()).transpose()?))
            }
        };
        let border = match &style.border {
            None => None,
            Some(border) => {
                let recorded = &self.edits.border_additions;
                let position = recorded.iter().position(|(have, _)| have == border);
                let id = collection_id(styles.borders(), position, recorded.len())?;
                Some((
                    id,
                    position.is_none().then(|| border.compose()).transpose()?,
                ))
            }
        };
        let index = u32::try_from(existing + recorded).map_err(|_| STYLE_OUT_OF_RANGE)?;
        // The cell format's own children, composed here as the collections'
        // elements are; only an alignment can be refused (its rotation).
        let alignment = style
            .alignment
            .as_ref()
            .map(Alignment::compose)
            .transpose()?;
        let protection = style.protection.as_ref().map(Protection::compose);
        let changes = Changes {
            num_fmt,
            font: font.as_ref().map(|(id, _)| *id),
            fill: fill.as_ref().map(|(id, _)| *id),
            border: border.as_ref().map(|(id, _)| *id),
            alignment,
            protection,
        };
        // The base is in range by the check above, and its bytes are ones the
        // reader accepted, so this is `None` only for a part that could not
        // have been read.
        let xf = styles
            .derived_xf(base, &changes)
            .ok_or(STYLE_OUT_OF_RANGE)?;
        if let (Some(id), Some(code)) = (num_fmt, new_code) {
            self.edits.num_fmt_additions.push((id, code));
        }
        if let (Some(font), Some((_, Some(element)))) = (&style.font, font) {
            self.edits.font_additions.push((font.clone(), element));
        }
        if let (Some(fill), Some((_, Some(element)))) = (&style.fill, fill) {
            self.edits.fill_additions.push((*fill, element));
        }
        if let (Some(border), Some((_, Some(element)))) = (&style.border, border) {
            self.edits.border_additions.push((*border, element));
        }
        self.edits.xf_additions.push(xf);
        Ok(index)
    }

    /// The body of [`Package::set_cell_style`].
    fn set_cell_style(&mut self, part: &[u8], reference: CellRef, style: Option<u32>) {
        self.edits
            .style_edits
            .push((part.to_vec(), reference, style));
    }

    /// The body of the four format calls: a column's (`row` false) or a
    /// row's width, height or hidden flag.
    fn line_format(
        &mut self,
        part: &[u8],
        index: u32,
        field: LineField,
        row: bool,
    ) -> Result<(), Error> {
        let limit = if row { GRID_ROWS } else { GRID_COLUMNS };
        if !(1..=limit).contains(&index) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        if let LineField::Size(Some(size)) = field {
            let most = if row { 409.0 } else { 255.0 };
            if !size.is_finite() || !(0.0..=most).contains(&size) {
                return Err(SIZE_OUT_OF_RANGE);
            }
        }
        let recorded = if row {
            &mut self.edits.row_formats
        } else {
            &mut self.edits.column_formats
        };
        recorded.push((part.to_vec(), index, field));
        Ok(())
    }

    /// The body of [`Package::freeze_panes`].
    fn freeze_panes(&mut self, part: &[u8], at: Option<CellRef>) -> Result<(), Error> {
        if let Some(cell) = at {
            if !in_grid(cell) {
                return Err(INDEX_OUT_OF_RANGE);
            }
            if cell.row == 1 && cell.col == 1 {
                return Err(FREEZE_AT_A1);
            }
        }
        self.edits.panes.push((part.to_vec(), at));
        Ok(())
    }

    /// The body of [`Package::set_tab_color`].
    #[cfg(feature = "sheetprops")]
    fn set_tab_color(&mut self, part: &[u8], color: Option<Color>) -> Result<(), Error> {
        if let Some(Color::Theme {
            tint: Some(tint), ..
        }) = color
            && !(-1.0..=1.0).contains(&tint)
        {
            return Err(SETTING_OUT_OF_RANGE);
        }
        self.edits.tab_colors.push((part.to_vec(), color));
        Ok(())
    }

    /// The body of [`Package::set_sheet_view`].
    #[cfg(feature = "sheetprops")]
    fn set_sheet_view(&mut self, part: &[u8], view: &SheetView) -> Result<(), Error> {
        if view.zoom.is_some_and(|zoom| !(10..=400).contains(&zoom)) {
            return Err(SETTING_OUT_OF_RANGE);
        }
        self.edits.sheet_views.push((part.to_vec(), *view));
        Ok(())
    }

    /// The body of [`Package::merge_cells`].
    fn merge_cells(&mut self, part: &[u8], area: Area) -> Result<(), Error> {
        let area = area.normalised();
        if !in_grid(area.first) || !in_grid(area.last) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        if area.cells() < 2 {
            return Err(MERGE_ONE_CELL);
        }
        self.edits.merges.push((part.to_vec(), area));
        Ok(())
    }

    /// The body of [`Package::insert_rows`], [`Package::delete_rows`],
    /// [`Package::insert_columns`] and [`Package::delete_columns`].
    #[cfg(feature = "rowcol")]
    fn shift(
        &mut self,
        part: &[u8],
        at: u32,
        count: u32,
        insert: bool,
        columns: bool,
    ) -> Result<(), Error> {
        let limit = if columns { GRID_COLUMNS } else { GRID_ROWS };
        let last = u64::from(at) + u64::from(count);
        if at == 0 || count == 0 || last - 1 > u64::from(limit) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        self.edits.shifts.push((
            part.to_vec(),
            AxisShift {
                shift: Shift {
                    insert,
                    at,
                    count,
                    limit,
                },
                columns,
            },
        ));
        Ok(())
    }

    /// The body of [`Package::unmerge_cells`].
    fn unmerge_cells(&mut self, part: &[u8], area: Area) -> Result<(), Error> {
        let area = area.normalised();
        if !in_grid(area.first) || !in_grid(area.last) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        self.edits.unmerges.push((part.to_vec(), area));
        Ok(())
    }

    /// The body of [`Package::recalculate_on_open`].
    fn recalculate_on_open(&mut self) {
        self.edits.recalculate = true;
    }

    /// The body of [`Package::add_defined_name`].
    #[cfg(feature = "metadata")]
    fn add_defined_name(
        &mut self,
        name: &str,
        text: &str,
        sheet: Option<&str>,
    ) -> Result<(), Error> {
        check_defined_name(name)?;
        if text.is_empty() {
            return Err(DEFINED_NAME_INVALID);
        }
        if text.chars().any(crate::cells::forbidden_in_xml) {
            return Err(TEXT_NOT_XML);
        }
        let sheet = self.name_scope(sheet)?;
        self.edits.name_edits.push(NameEdit::Add {
            name: name.to_string(),
            text: text.to_string(),
            sheet,
        });
        Ok(())
    }

    /// The body of [`Package::remove_defined_name`].
    #[cfg(feature = "metadata")]
    fn remove_defined_name(&mut self, name: &str, sheet: Option<&str>) -> Result<(), Error> {
        let sheet = self.name_scope(sheet)?;
        self.edits.name_edits.push(NameEdit::Remove {
            name: name.to_string(),
            sheet,
        });
        Ok(())
    }

    /// The body of [`Package::set_hyperlink`].
    #[cfg(feature = "metadata")]
    fn set_hyperlink(
        &mut self,
        part: &[u8],
        reference: CellRef,
        target: LinkTarget<'_>,
        tooltip: Option<&str>,
    ) -> Result<(), Error> {
        if !in_grid(reference) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        let (external, text) = match target {
            LinkTarget::External(text) => (true, text),
            LinkTarget::Location(text) => (false, text),
        };
        if text.is_empty() {
            return Err(LINK_EMPTY);
        }
        let forbidden = |text: &str| text.chars().any(crate::cells::forbidden_in_xml);
        if forbidden(text) || tooltip.is_some_and(forbidden) {
            return Err(TEXT_NOT_XML);
        }
        self.edits.links.push((
            part.to_vec(),
            reference,
            Some(LinkSet {
                external,
                text: text.to_string(),
                tooltip: tooltip.map(str::to_string),
            }),
        ));
        Ok(())
    }

    /// The body of [`Package::remove_hyperlink`].
    #[cfg(feature = "metadata")]
    fn remove_hyperlink(&mut self, part: &[u8], reference: CellRef) -> Result<(), Error> {
        if !in_grid(reference) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        self.edits.links.push((part.to_vec(), reference, None));
        Ok(())
    }

    /// When a property is set and the package has no core-properties
    /// relationship, records `docProps/core.xml` as an addition holding the
    /// set properties, its package relationship with the relative target
    /// and its `Override` (decision 5 A), before validation so each is
    /// checked as any other edit is.
    #[cfg(feature = "metadata")]
    fn record_core_creation(&mut self) -> Result<(), Error> {
        if self.edits.properties.is_empty() || self.core_part()?.is_some() {
            return Ok(());
        }
        let props = property_edits(&self.edits.properties);
        if props.iter().all(|(_, value)| value.is_none()) {
            return Ok(());
        }
        let bytes = new_core_part(&props);
        self.add_part(
            b"docProps/core.xml",
            replacement_from_reader(Cursor::new(bytes)),
        );
        self.add_relationship(b"", CORE_PROPERTIES[0], b"docProps/core.xml", false)?;
        self.add_override(b"/docProps/core.xml", CORE_PROPERTIES_TYPE);
        Ok(())
    }

    /// The body of [`Package::properties`].
    #[cfg(feature = "metadata")]
    fn properties(&mut self) -> Result<Properties, Error> {
        let Some(part) = self.core_part()? else {
            return Ok(Properties::default());
        };
        let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
        read_core(self.archive, index)
    }

    /// The entry the package's core-properties relationship names, either
    /// form, `None` when it has none.
    #[cfg(feature = "metadata")]
    fn core_part(&mut self) -> Result<Option<Vec<u8>>, Error> {
        let rels = self.relationships(b"")?;
        match rels.first_of(CORE_PROPERTIES) {
            Some(rel) => Ok(Some(rels.target_part(rel)?)),
            None => Ok(None),
        }
    }

    /// The body of [`Package::set_property`].
    #[cfg(feature = "metadata")]
    fn set_property(&mut self, property: Property, value: Option<&str>) -> Result<(), Error> {
        if let Some(value) = value {
            if property.dated() && !w3cdtf_ok(value) {
                return Err(PROPERTY_INVALID);
            }
            if value.chars().any(crate::cells::forbidden_in_xml) {
                return Err(TEXT_NOT_XML);
            }
        }
        self.edits
            .properties
            .push((property, value.map(str::to_string)));
        Ok(())
    }

    /// The body of [`Package::protect_sheet`].
    #[cfg(feature = "metadata")]
    fn protect_sheet(
        &mut self,
        part: &[u8],
        password: Option<&str>,
        options: SheetProtection,
    ) -> Result<(), Error> {
        let hash = match password {
            Some(password) if !password_ok(password) => return Err(PASSWORD_INVALID),
            Some(password) => Some(legacy_password_hash(password)),
            None => None,
        };
        self.edits
            .sheet_protections
            .push((part.to_vec(), Some((hash, options))));
        Ok(())
    }

    /// The body of [`Package::add_conditional_format`]: the call's checks,
    /// then the rule's attributes and formulas composed and recorded.
    #[cfg(feature = "rules")]
    fn add_conditional_format(
        &mut self,
        part: &[u8],
        areas: &[Area],
        rule: Rule<'_>,
        dxf: Option<u32>,
        priority: Option<u32>,
        stop_if_true: bool,
    ) -> Result<(), Error> {
        let areas = checked_areas(areas)?;
        let visual = visual_child(&rule)?;
        // A dxf rule needs a style and a visual rule takes none.
        if dxf.is_none() != visual.is_some() {
            return Err(RULE_STYLE);
        }
        if priority == Some(0) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        let top_left = areas[0].first.a1();
        let (kind, operator, formulas): (&[u8], Option<Operator>, Vec<String>) = match rule {
            Rule::CellIs {
                operator,
                formula1,
                formula2,
            } => {
                if formula1.is_empty()
                    || formula2.is_some_and(str::is_empty)
                    || formula2.is_some() != operator.two_values()
                {
                    return Err(FORMULA_MISSING);
                }
                let formulas = [Some(formula1), formula2].into_iter().flatten();
                (
                    b"cellIs",
                    Some(operator),
                    formulas.map(str::to_string).collect(),
                )
            }
            Rule::Expression(formula) => {
                if formula.is_empty() {
                    return Err(FORMULA_MISSING);
                }
                (b"expression", None, vec![formula.to_string()])
            }
            Rule::ContainsBlanks => (
                b"containsBlanks",
                None,
                vec![format!("LEN(TRIM({top_left}))=0")],
            ),
            Rule::NotContainsBlanks => (
                b"notContainsBlanks",
                None,
                vec![format!("LEN(TRIM({top_left}))>0")],
            ),
            Rule::DuplicateValues => (b"duplicateValues", None, Vec::new()),
            Rule::UniqueValues => (b"uniqueValues", None, Vec::new()),
            Rule::Top { rank, .. } if !(1..=1000).contains(&rank) => {
                return Err(INDEX_OUT_OF_RANGE);
            }
            Rule::Top { .. } => (b"top10", None, Vec::new()),
            Rule::AboveAverage { .. } => (b"aboveAverage", None, Vec::new()),
            Rule::ColorScale { .. } | Rule::DataBar { .. } | Rule::IconSet { .. } => (
                visual.as_ref().map_or(b"".as_slice(), |(kind, _)| kind),
                None,
                Vec::new(),
            ),
        };
        for formula in &formulas {
            rule_formula_ok(formula)?;
        }
        let attribute =
            |name: &[u8], value: &[u8]| [b" ".as_slice(), name, b"=\"", value, b"\""].concat();
        let mut before = attribute(b"type", kind);
        if let Some(dxf) = dxf {
            before.extend_from_slice(&attribute(b"dxfId", dxf.to_string().as_bytes()));
        }
        let mut after = Vec::new();
        if stop_if_true {
            after.extend_from_slice(&attribute(b"stopIfTrue", b"1"));
        }
        if let Some(operator) = operator {
            after.extend_from_slice(&attribute(b"operator", operator.name()));
        }
        match rule {
            Rule::Top {
                rank,
                percent,
                bottom,
            } => {
                after.extend_from_slice(&attribute(b"rank", rank.to_string().as_bytes()));
                if percent {
                    after.extend_from_slice(&attribute(b"percent", b"1"));
                }
                if bottom {
                    after.extend_from_slice(&attribute(b"bottom", b"1"));
                }
            }
            Rule::AboveAverage { above, equal } => {
                if !above {
                    after.extend_from_slice(&attribute(b"aboveAverage", b"0"));
                }
                if equal {
                    after.extend_from_slice(&attribute(b"equalAverage", b"1"));
                }
            }
            _ => {}
        }
        let formulas = formulas
            .iter()
            .map(|text| quick_xml::escape::partial_escape(text).into_owned())
            .collect();
        self.edits.rules.push((
            part.to_vec(),
            RuleEdit::Add(NewRule {
                areas,
                dxf,
                priority,
                before,
                after,
                formulas,
                visual: visual.map(|(_, child)| child),
            }),
        ));
        Ok(())
    }

    /// The body of [`Package::add_differential_style`].
    #[cfg(feature = "rules")]
    fn add_differential_style(&mut self, style: &Style) -> Result<u32, Error> {
        let written = [
            style.font.is_some(),
            style.fill.is_some(),
            style.border.is_some(),
        ];
        if style.number_format.is_some()
            || style.alignment.is_some()
            || style.protection.is_some()
            || !written.contains(&true)
        {
            return Err(DXF_FIELD);
        }
        let mut dxf = Composed::new();
        if let Some(font) = &style.font {
            dxf.append(&font.compose()?);
        }
        if let Some(fill) = &style.fill {
            dxf.append(&fill.compose()?);
        }
        if let Some(border) = &style.border {
            dxf.append(&border.compose()?);
        }
        let id = self.dxf_children()? + self.edits.dxf_additions.len() as u64;
        let id = u32::try_from(id).map_err(|_| DXF_OUT_OF_RANGE)?;
        self.edits.dxf_additions.push(dxf);
        Ok(id)
    }

    /// The styles part's `dxf` children, read once per package; a part
    /// with two `<dxfs>` is refused.
    #[cfg(feature = "rules")]
    fn dxf_children(&mut self) -> Result<u64, Error> {
        if let Some(children) = self.edits.dxf_children {
            return Ok(children);
        }
        let part = self.workbook()?.styles.clone().ok_or(NO_STYLES_PART)?;
        let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
        let (children, containers) = read_dxfs(self.archive.read(index)?, index)?;
        if containers > 1 {
            return Err(TWO_DXFS);
        }
        self.edits.dxf_children = Some(children);
        Ok(children)
    }

    /// The body of [`Package::add_validation`]: the call's checks, then the
    /// element's attributes and children composed and recorded.
    #[cfg(feature = "rules")]
    fn add_validation(
        &mut self,
        part: &[u8],
        areas: &[Area],
        validation: Validation<'_>,
    ) -> Result<(), Error> {
        let areas = checked_areas(areas)?;
        let kind = validation.kind;
        if (validation.operator.is_some() && !kind.takes_operator())
            || (validation.hide_drop_down && kind != ValidationKind::List)
        {
            return Err(FIELD_NOT_OF_KIND);
        }
        // `None` on a kind that takes an operator reads as `Between`.
        let two = kind.takes_operator() && validation.operator.is_none_or(Operator::two_values);
        if validation.formula1.is_empty()
            || validation.formula2.is_some_and(str::is_empty)
            || validation.formula2.is_some() != two
        {
            return Err(FORMULA_MISSING);
        }
        let formulas: Vec<&str> = [Some(validation.formula1), validation.formula2]
            .into_iter()
            .flatten()
            .collect();
        for formula in &formulas {
            rule_formula_ok(formula)?;
        }
        let texts = [validation.input, validation.error.map(|(_, t, x)| (t, x))];
        for (title, text) in texts.into_iter().flatten() {
            let forbidden = |text: &str| text.chars().any(crate::cells::forbidden_in_xml);
            if forbidden(title) || forbidden(text) {
                return Err(TEXT_NOT_XML);
            }
            if title.chars().count() > 32 || text.chars().count() > 255 {
                return Err(MESSAGE_LONG);
            }
        }
        // Each attribute Excel's order gives, its value already escaped
        // (P1 ruling 5 for `showDropDown`'s place).
        let flag = |on: bool| -> Vec<u8> { if on { b"1".to_vec() } else { b"0".to_vec() } };
        let mut written: Vec<(&[u8], Vec<u8>)> = vec![(b"type", kind.name().to_vec())];
        if let Some(operator) = validation.operator {
            written.push((b"operator", operator.name().to_vec()));
        }
        written.push((b"allowBlank", flag(validation.allow_blank)));
        if validation.hide_drop_down {
            written.push((b"showDropDown", flag(true)));
        }
        written.push((b"showInputMessage", flag(validation.input.is_some())));
        written.push((b"showErrorMessage", flag(validation.error.is_some())));
        if let Some((style, title, text)) = validation.error {
            written.push((b"errorStyle", style.name().to_vec()));
            written.push((b"errorTitle", attribute_escaped(title)));
            written.push((b"error", attribute_escaped(text)));
        }
        if let Some((title, text)) = validation.input {
            written.push((b"promptTitle", attribute_escaped(title)));
            written.push((b"prompt", attribute_escaped(text)));
        }
        let mut attributes = Vec::new();
        for (name, value) in written {
            attributes.extend_from_slice(&[b" ".as_slice(), name, b"=\"", &value, b"\""].concat());
        }
        let names: [&'static [u8]; 2] = [b"formula1", b"formula2"];
        let formulas = names
            .into_iter()
            .zip(formulas)
            .map(|(name, text)| (name, quick_xml::escape::partial_escape(text).into_owned()))
            .collect();
        self.edits.validations.push((
            part.to_vec(),
            ValidationEdit::Add(NewValidation {
                areas,
                attributes,
                formulas,
            }),
        ));
        Ok(())
    }

    /// Records the relationship of every external link the write will
    /// append, each cell's last set, on its part's rels part, and keeps the
    /// id for the worksheet pass; the type's form, and a retag's namespace
    /// form, follow the worksheet's own relationship in the workbook's rels
    /// part (P2 verification ruling 2).
    #[cfg(feature = "metadata")]
    fn record_link_relationships(&mut self) -> Result<(), Error> {
        let mut parts: Vec<Vec<u8>> = Vec::new();
        for (part, _, _) in &self.edits.links {
            if !parts.contains(part) {
                parts.push(part.clone());
            }
        }
        for part in parts {
            let strict = self.sheet_is_strict(&part)?;
            self.edits.link_strict.push((part.clone(), strict));
            let wanted: Vec<(CellRef, String)> = self
                .edits
                .links_of(&part)
                .into_iter()
                .filter_map(|(cell, edit)| match edit {
                    LinkEdit::Set { set, .. } if set.external => Some((cell, set.text)),
                    _ => None,
                })
                .collect();
            if wanted.is_empty() {
                continue;
            }
            let kind = HYPERLINK[usize::from(strict)];
            for (cell, target) in wanted {
                let id = self.add_relationship(&part, kind, target.as_bytes(), true)?;
                self.edits.link_ids.push((part.clone(), cell, id));
            }
        }
        Ok(())
    }

    /// The body of [`Package::add_table`]: the call's checks, then the
    /// table and its header cells recorded (`specs/0021-tables.md`, "Names"
    /// and "The calls").
    #[cfg(feature = "tables")]
    fn add_table(&mut self, part: &[u8], area: Area, table: Table<'_>) -> Result<(), Error> {
        let area = area.normalised();
        if !in_grid(area.first) || !in_grid(area.last) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        let forbidden = |text: &str| text.chars().any(crate::cells::forbidden_in_xml);
        if forbidden(table.name) || table.columns.iter().any(|column| forbidden(column)) {
            return Err(TEXT_NOT_XML);
        }
        check_table_name(table.name)?;
        let width = u64::from(area.last.col - area.first.col) + 1;
        let lengths_ok = table
            .columns
            .iter()
            .all(|column| (1..=255).contains(&column.chars().count()));
        let distinct = table.columns.iter().enumerate().all(|(at, column)| {
            !table.columns[..at]
                .iter()
                .any(|before| before.eq_ignore_ascii_case(column))
        });
        if !lengths_ok || !distinct || table.columns.len() as u64 != width {
            return Err(TABLE_COLUMN_INVALID);
        }
        let style = match table.style {
            Some(info) => Some((info.style.name().ok_or(TABLE_STYLE_RANGE)?, info)),
            None => None,
        };
        if area.first.row == area.last.row {
            return Err(TABLE_TOO_SMALL);
        }
        for (column, name) in (area.first.col..).zip(table.columns) {
            let cell = CellRef {
                row: area.first.row,
                col: column,
            };
            self.set_cell(part, cell, Value::Text((*name).to_string()));
        }
        self.edits.table_additions.push((
            part.to_vec(),
            NewTable {
                area,
                name: table.name.to_string(),
                columns: table.columns.iter().map(|c| (*c).to_string()).collect(),
                style,
            },
        ));
        Ok(())
    }

    /// The body of [`Package::tables`].
    #[cfg(feature = "tables")]
    fn tables(&mut self, part: &[u8]) -> Result<Vec<TableEntry>, Error> {
        let index = self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        let ids = table_part_ids(self.archive.read(index)?, index)?;
        let mut targets = Vec::new();
        {
            let rels = self.relationships(part)?;
            for id in ids {
                let rel = rels.by_id(&id).ok_or(TABLE_UNREADABLE)?;
                targets.push(rels.target_part(rel).map_err(|_| TABLE_UNREADABLE)?);
            }
        }
        let mut entries = Vec::new();
        for target in targets {
            let at = self.archive.index_of(&target).ok_or(TABLE_UNREADABLE)?;
            entries.push(read_table(self.archive.read(at)?, at, target)?);
        }
        Ok(entries)
    }

    /// The targets of `source`'s internal relationships of `kind`, either
    /// form, in part order; a target outside the package refuses with the
    /// table read message.
    #[cfg(feature = "tables")]
    fn related(&mut self, source: &[u8], kind: [&[u8]; 2]) -> Result<Vec<Vec<u8>>, Error> {
        Ok(self
            .related_ids(source, kind)?
            .into_iter()
            .map(|(_, target)| target)
            .collect())
    }

    /// [`Parts::related`] with each relationship's id beside its target.
    #[cfg(feature = "tables")]
    fn related_ids(&mut self, source: &[u8], kind: [&[u8]; 2]) -> Result<Vec<RelTarget>, Error> {
        let rels = self.relationships(source)?;
        rels.all()
            .iter()
            .filter(|rel| !rel.external && kind.contains(&rel.kind.as_slice()))
            .map(|rel| {
                let target = rels.target_part(rel).map_err(|_| TABLE_UNREADABLE)?;
                Ok((rel.id.clone(), target))
            })
            .collect()
    }

    /// Records each table removal's four removals (`specs/0021-tables.md`,
    /// "The calls", `remove_table`): the part's `table` relationship whose
    /// part's root names the table, ASCII case folded, against `name` or
    /// `displayName`, the `Override` for that part when the content types
    /// part has one, and the entry; the `<tablePart>` naming the
    /// relationship is the worksheet pass's. A name no table of the part's
    /// file carries, one this write also adds included, is refused; a table
    /// part the search cannot read is refused with the read message.
    #[cfg(feature = "tables")]
    fn record_table_removals(&mut self) -> Result<(), Error> {
        let removals = self.edits.table_removals.clone();
        let mut found: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)> = Vec::new();
        for (part, name) in removals {
            let folded = name.to_ascii_lowercase();
            let mut hit = None;
            if self.archive.index_of(&part).is_some() {
                for (id, target) in self.related_ids(&part, TABLE)? {
                    let index = self.archive.index_of(&target).ok_or(TABLE_UNREADABLE)?;
                    let root = table_root(self.archive, index)?;
                    if hit.is_none()
                        && root
                            .names
                            .iter()
                            .any(|have| have.to_ascii_lowercase() == folded)
                    {
                        hit = Some((id, target));
                    }
                }
            }
            let (id, target) = hit.ok_or(NO_SUCH_TABLE)?;
            if found.iter().any(|(_, _, have)| *have == target) {
                return Err(NO_SUCH_TABLE);
            }
            found.push((part, id, target));
        }
        for (part, id, target) in found {
            let over = self
                .override_of(&target)?
                .filter(|over| !self.edits.override_removals.contains(over));
            self.remove_relationship(&part, &id);
            if let Some(over) = over {
                self.remove_override(&over);
            }
            if !self.edits.part_removals.contains(&target) {
                self.remove_part(&target);
            }
            self.edits.table_removed_ids.push((part, id));
        }
        Ok(())
    }

    /// Records each table addition's part, `Override` and relationship,
    /// and keeps its relationship id for the worksheet pass, after the
    /// write-time checks of `specs/0021-tables.md`, "Names" and "The calls":
    /// the name against the projected workbook's tables, the workbook
    /// part's defined names and the earlier additions; the area against the
    /// part's tables and pivot tables, the earlier additions for the part
    /// and the merges this write adds. The part's own merges, autofilter and
    /// header cells are checked by the rewrite.
    #[cfg(feature = "tables")]
    fn record_tables(&mut self) -> Result<(), Error> {
        self.record_table_removals()?;
        if self.edits.table_additions.is_empty() {
            return Ok(());
        }
        let removed = self.edits.part_removals.clone();
        let workbook = self.workbook()?.part.clone();
        let sheets: Vec<Vec<u8>> = self
            .workbook()?
            .sheets
            .iter()
            .map(|sheet| sheet.part.clone())
            .collect();
        // The projected workbook's tables: every sheet's, less a sheet or
        // a table part this write removes.
        let mut kept: Vec<(Vec<u8>, TableRoot)> = Vec::new();
        for sheet in sheets {
            if removed.contains(&sheet) || self.archive.index_of(&sheet).is_none() {
                continue;
            }
            for target in self.related(&sheet, TABLE)? {
                if removed.contains(&target) {
                    continue;
                }
                let index = self.archive.index_of(&target).ok_or(TABLE_UNREADABLE)?;
                kept.push((sheet.clone(), table_root(self.archive, index)?));
            }
        }
        let defined = match self.archive.index_of(&workbook) {
            Some(index) => defined_name_values(self.archive.read(index)?, index)?,
            None => Vec::new(),
        };
        let mut entries = self.taken_entries();
        let additions = self.edits.table_additions.clone();
        let mut names: Vec<String> = Vec::new();
        let mut ids: Vec<u64> = kept.iter().filter_map(|(_, root)| root.id).collect();
        let mut works: Vec<(Vec<u8>, TableWork, bool)> = Vec::new();
        for (part, table) in additions {
            let folded = table.name.to_ascii_lowercase();
            let in_use = kept
                .iter()
                .flat_map(|(_, root)| root.names.iter())
                .chain(defined.iter())
                .chain(names.iter())
                .any(|name| name.to_ascii_lowercase() == folded);
            if in_use {
                return Err(TABLE_NAME_IN_USE);
            }
            names.push(table.name.clone());
            // The part's tables, pivot tables, earlier additions and the
            // merges this write adds (decision 4 A).
            let mut taken: Vec<Area> = Vec::new();
            for (sheet, root) in &kept {
                if *sheet == part {
                    taken.push(root.area.ok_or(TABLE_UNREADABLE)?);
                }
            }
            if self.archive.index_of(&part).is_some() && !removed.contains(&part) {
                for target in self.related(&part, PIVOT_TABLE)? {
                    let index = self.archive.index_of(&target).ok_or(TABLE_UNREADABLE)?;
                    let value = part_ref(self.archive, index, true)?.ok_or(TABLE_UNREADABLE)?;
                    taken.push(Area::parse(&value).ok_or(TABLE_UNREADABLE)?);
                }
            }
            taken.extend(
                works
                    .iter()
                    .filter(|(at, _, _)| *at == part)
                    .map(|(_, work, _)| work.area),
            );
            taken.extend(
                self.edits
                    .merges
                    .iter()
                    .filter(|(at, _)| *at == part)
                    .map(|(_, merge)| *merge),
            );
            if taken.iter().any(|area| area.overlaps(&table.area)) {
                return Err(TABLE_OVERLAP);
            }
            // The smallest unused of each; a list of k values leaves one of
            // 1 to k + 1 unused, so neither count runs past it.
            let entry = free_entry(&entries, |n| format!("xl/tables/table{n}.xml"));
            let mut id = 1u64;
            while ids.contains(&id) {
                id += 1;
            }
            ids.push(id);
            entries.push(entry.to_ascii_lowercase());
            let strict = self.sheet_is_strict(&part)?;
            self.add_part(
                entry.as_bytes(),
                replacement_from_reader(Cursor::new(table_part_bytes(id, &table))),
            );
            self.add_override(format!("/{entry}").as_bytes(), TABLE_CONTENT_TYPE);
            let target = relative_target(&part, entry.as_bytes());
            let rel_id =
                self.add_relationship(&part, TABLE[usize::from(strict)], &target, false)?;
            works.push((
                part,
                TableWork {
                    area: table.area,
                    rel_id,
                },
                strict,
            ));
        }
        self.edits.table_works = works;
        Ok(())
    }

    /// The body of [`Package::add_comment`]: the call's checks, then the
    /// comment recorded (`specs/0022-comments-and-images.md`, "Comments:
    /// types" and "Comments: the calls").
    #[cfg(feature = "comments")]
    fn add_comment(
        &mut self,
        part: &[u8],
        cell: CellRef,
        comment: Comment<'_>,
    ) -> Result<(), Error> {
        if !in_grid(cell) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        let forbidden = |text: &str| text.chars().any(crate::cells::forbidden_in_xml);
        if forbidden(comment.author) || forbidden(comment.text) {
            return Err(TEXT_NOT_XML);
        }
        if !(1..=255).contains(&comment.author.chars().count()) {
            return Err(COMMENT_AUTHOR_INVALID);
        }
        if !(1..=32767).contains(&comment.text.chars().count()) {
            return Err(COMMENT_TEXT_INVALID);
        }
        self.edits.comment_edits.push((
            part.to_vec(),
            cell,
            Some(NewComment {
                author: comment.author.to_string(),
                text: comment.text.to_string(),
                #[cfg(feature = "richtext")]
                runs: None,
            }),
        ));
        Ok(())
    }

    /// The body of [`Package::add_rich_comment`]: [`Parts::add_comment`]'s
    /// checks on the author and on the flattened text, each run's text and
    /// font checked as the cell write checks them, then the comment recorded
    /// with its runs (`specs/0024-rich-text.md`, "Comments").
    #[cfg(all(feature = "comments", feature = "richtext"))]
    fn add_rich_comment(
        &mut self,
        part: &[u8],
        cell: CellRef,
        author: &str,
        text: &RichText,
    ) -> Result<(), Error> {
        if !in_grid(cell) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        let forbidden = |text: &str| text.chars().any(crate::cells::forbidden_in_xml);
        if forbidden(author) || text.runs.iter().any(|run| forbidden(&run.text)) {
            return Err(TEXT_NOT_XML);
        }
        if !(1..=255).contains(&author.chars().count()) {
            return Err(COMMENT_AUTHOR_INVALID);
        }
        let flat = text.text();
        if text.runs.is_empty()
            || text.runs.iter().any(|run| run.text.is_empty())
            || flat.chars().count() > 32767
        {
            return Err(COMMENT_TEXT_INVALID);
        }
        for font in text.runs.iter().filter_map(|run| run.font.as_ref()) {
            font.check()?;
        }
        self.edits.comment_edits.push((
            part.to_vec(),
            cell,
            Some(NewComment {
                author: author.to_string(),
                text: flat,
                runs: Some(text.clone()),
            }),
        ));
        Ok(())
    }

    /// The body of [`Package::remove_comment`].
    #[cfg(feature = "comments")]
    fn remove_comment(&mut self, part: &[u8], cell: CellRef) -> Result<(), Error> {
        self.edits.comment_edits.push((part.to_vec(), cell, None));
        Ok(())
    }

    /// The body of [`Package::comments`].
    #[cfg(feature = "comments")]
    fn comments(&mut self, part: &[u8]) -> Result<Vec<CommentEntry>, Error> {
        self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        let Some(target) = self.comments_target(part)? else {
            return Ok(Vec::new());
        };
        let index = self.archive.index_of(&target).ok_or(COMMENTS_UNREADABLE)?;
        read_comments(self.archive.read(index)?, index)
    }

    /// The part the first internal `comments` relationship of `part` names,
    /// either form; a target outside the package refuses with the comments
    /// read message.
    #[cfg(feature = "comments")]
    fn comments_target(&mut self, part: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.comments_rel(part)?.map(|(_, target)| target))
    }

    /// [`Parts::comments_target`] with the relationship's id beside it.
    #[cfg(feature = "comments")]
    fn comments_rel(&mut self, part: &[u8]) -> Result<Option<RelTarget>, Error> {
        let rels = self.relationships(part)?;
        match rels
            .all()
            .iter()
            .find(|rel| !rel.external && COMMENTS.contains(&rel.kind.as_slice()))
        {
            Some(rel) => Ok(Some((
                rel.id.clone(),
                rels.target_part(rel).map_err(|_| COMMENTS_UNREADABLE)?,
            ))),
            None => Ok(None),
        }
    }

    /// Records each worksheet's comment edits
    /// (`specs/0022-comments-and-images.md`, "Comments: the calls" and
    /// "Comments: the parts"): the comments part appended to, created or,
    /// when its last comment goes, dropped; the VML part likewise, a dropped
    /// comments part taking it along unless it holds a shape of another
    /// `ObjectType`; their overrides and relationships; and what the
    /// worksheet pass does with `<legacyDrawing>`. A cell's last addition
    /// wins, in that addition's place; a removal followed by an addition of
    /// one cell is the replacement. A removal of a cell this write adds a
    /// comment on or names twice is refused first, while the edits are
    /// grouped by part; then, part by part, the refusals of a removed,
    /// replaced, absent or shifted part, then the sheet's threaded
    /// comments, then what the parts cannot be read for, then a removal or
    /// a replacement of a cell with no comment.
    #[cfg(feature = "comments")]
    fn record_comments(&mut self) -> Result<(), Error> {
        if self.edits.comment_edits.is_empty() {
            return Ok(());
        }
        let mut parts: Vec<(Vec<u8>, PartComments)> = Vec::new();
        for (part, cell, edit) in self.edits.comment_edits.clone() {
            let at = match parts.iter().position(|(have, _)| *have == part) {
                Some(at) => at,
                None => {
                    parts.push((part, PartComments::default()));
                    parts.len() - 1
                }
            };
            let edits = &mut parts[at].1;
            match edit {
                Some(comment) => {
                    // An addition after a removal of the cell replaces its
                    // comment, which must then be there.
                    if let Some(gone) = edits.removals.iter().position(|have| *have == cell) {
                        edits.removals.remove(gone);
                        edits.replaced.push(cell);
                    }
                    edits.additions.retain(|(have, _)| *have != cell);
                    edits.additions.push((cell, comment));
                }
                None => {
                    // A removal of a cell this write adds a comment on, or one
                    // named twice: the caller writes twice.
                    if edits.additions.iter().any(|(have, _)| *have == cell)
                        || edits.removals.contains(&cell)
                        || edits.replaced.contains(&cell)
                    {
                        return Err(NO_SUCH_COMMENT);
                    }
                    edits.removals.push(cell);
                }
            }
        }
        let mut entries = self.taken_entries();
        // The `idmap` numbers of the projected workbook's VML parts, read
        // when the first part needs one allocated.
        let mut idmaps: Option<Vec<u64>> = None;
        let mut works: Vec<(Vec<u8>, LegacyWork)> = Vec::new();
        for (part, edits) in parts {
            if self.edits.part_removals.contains(&part) {
                return Err(REMOVED_PART_EDITED);
            }
            if self
                .edits
                .part_replacements
                .iter()
                .any(|(name, _)| *name == part)
            {
                return Err(CELL_EDIT_REPLACED);
            }
            // A sheet this write adds has no relationships part to read yet
            // (P1 ruling 2 of `specs/0021-tables.md`, as for a table).
            let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
            #[cfg(feature = "rowcol")]
            if self.edits.shifts.iter().any(|(at, _)| *at == part) {
                return Err(SHIFT_ALONE);
            }
            if self
                .relationships(&part)?
                .all()
                .iter()
                .any(|rel| THREADED_COMMENTS.contains(&rel.kind.as_slice()))
            {
                return Err(THREADED_COMMENTS_HELD);
            }
            let comments = self.comments_rel(&part)?;
            let legacy = element_rel_id(
                self.archive.read(index)?,
                index,
                LEGACY_DRAWING,
                TWO_LEGACY_DRAWINGS,
                COMMENTS_UNREADABLE,
            )?;
            let vml = match &legacy {
                Some(id) => {
                    let rels = self.relationships(&part)?;
                    let rel = rels
                        .by_id(id)
                        .filter(|rel| !rel.external && VML_DRAWING.contains(&rel.kind.as_slice()))
                        .ok_or(COMMENTS_UNREADABLE)?;
                    Some(rels.target_part(rel).map_err(|_| COMMENTS_UNREADABLE)?)
                }
                None => None,
            };
            // A part this write removes, replaces or adds through the part
            // calls is not read for the edit (P1 verification ruling 1 (2)).
            self.edited_part_untouched(comments.as_ref().map(|(_, target)| target.as_slice()))?;
            self.edited_part_untouched(vml.as_deref())?;
            let at_of = |archive: &Core, target: Option<&Vec<u8>>| match target {
                Some(target) => archive
                    .index_of(target)
                    .map(Some)
                    .ok_or(COMMENTS_UNREADABLE),
                None => Ok(None),
            };
            let comments_at = at_of(self.archive, comments.as_ref().map(|(_, target)| target))?;
            let vml_at = at_of(self.archive, vml.as_ref())?;
            if comments.is_some() && vml.is_none() {
                return Err(COMMENTS_UNREADABLE);
            }
            let strict = self.sheet_is_strict(&part)?;
            let added: Vec<CellRef> = edits.additions.iter().map(|(cell, _)| *cell).collect();
            let skipped: Vec<CellRef> = added.iter().chain(&edits.removals).copied().collect();
            let wanted: Vec<CellRef> = edits
                .removals
                .iter()
                .chain(&edits.replaced)
                .copied()
                .collect();
            // The comments part: how many comments it held, and whether the
            // write drops it.
            let mut held = 0u64;
            let mut dropped = false;
            match (&comments, comments_at) {
                (Some((id, target)), Some(at)) => {
                    let appended =
                        comments_appended(self.archive.read(at)?, at, &edits.additions, &skipped)?;
                    if wanted.iter().any(|cell| !appended.found.contains(cell)) {
                        return Err(NO_SUCH_COMMENT);
                    }
                    held = appended.held;
                    if appended.kept == 0 && edits.additions.is_empty() {
                        dropped = true;
                        self.drop_with_relationship(&part, id, target)?;
                    } else {
                        self.replace_part(
                            target,
                            replacement_from_reader(Cursor::new(appended.bytes)),
                        );
                    }
                }
                _ => {
                    if !wanted.is_empty() {
                        return Err(NO_SUCH_COMMENT);
                    }
                    let entry = free_entry(&entries, |n| format!("xl/comments{n}.xml"));
                    entries.push(entry.to_ascii_lowercase());
                    self.add_part(
                        entry.as_bytes(),
                        replacement_from_reader(Cursor::new(comments_part_bytes(&edits.additions))),
                    );
                    self.add_override(format!("/{entry}").as_bytes(), COMMENTS_CONTENT_TYPE);
                    let target = relative_target(&part, entry.as_bytes());
                    self.add_relationship(&part, COMMENTS[usize::from(strict)], &target, false)?;
                }
            }
            // The VML part (P1 ruling 1 for its shape ids).
            match (&vml, vml_at, &legacy) {
                (Some(target), Some(at), Some(id)) => {
                    let mut bytes = Vec::new();
                    self.archive.read(at)?.read_to_end(&mut bytes)?;
                    let read = read_vml(&bytes, at)?;
                    if held > 0 && read.shapes.is_empty() {
                        return Err(COMMENTS_UNREADABLE);
                    }
                    let block = match (read.idmap, added.is_empty()) {
                        (Some(block), _) => block,
                        // Nothing is numbered, so no block is allocated.
                        (None, true) => 0,
                        (None, false) => {
                            if idmaps.is_none() {
                                idmaps = Some(self.vml_idmaps()?);
                            }
                            smallest_unused(idmaps.get_or_insert_with(Vec::new))
                        }
                    };
                    let other = read
                        .shapes
                        .iter()
                        .any(|shape| !shape.is_note && !skips(shape, &skipped));
                    if dropped && !other {
                        // The last comment gone and no control left: the VML
                        // part goes too, with its `<legacyDrawing>`.
                        self.drop_with_relationship(&part, id, target)?;
                        works.push((
                            part,
                            LegacyWork {
                                create: None,
                                drop: true,
                                strict,
                            },
                        ));
                    } else {
                        let bytes = vml_appended(&bytes, &read, block, &skipped, &added);
                        self.replace_part(target, replacement_from_reader(Cursor::new(bytes)));
                    }
                }
                _ => {
                    if idmaps.is_none() {
                        idmaps = Some(self.vml_idmaps()?);
                    }
                    let block = smallest_unused(idmaps.get_or_insert_with(Vec::new));
                    let entry = free_entry(&entries, |n| format!("xl/drawings/vmlDrawing{n}.vml"));
                    entries.push(entry.to_ascii_lowercase());
                    self.add_part(
                        entry.as_bytes(),
                        replacement_from_reader(Cursor::new(vml_part_bytes(block, &added))),
                    );
                    self.add_override(format!("/{entry}").as_bytes(), VML_CONTENT_TYPE);
                    let target = relative_target(&part, entry.as_bytes());
                    let id = self.add_relationship(
                        &part,
                        VML_DRAWING[usize::from(strict)],
                        &target,
                        false,
                    )?;
                    works.push((
                        part,
                        LegacyWork {
                            create: Some(id),
                            drop: false,
                            strict,
                        },
                    ));
                }
            }
        }
        self.edits.comment_works = works;
        Ok(())
    }

    /// Drops the part `target`, the relationship `id` of `source` that names
    /// it and its `Override` when the content types part has one, as
    /// `remove_table` drops a table part.
    #[cfg(feature = "comments")]
    fn drop_with_relationship(
        &mut self,
        source: &[u8],
        id: &[u8],
        target: &[u8],
    ) -> Result<(), Error> {
        let over = self
            .override_of(target)?
            .filter(|over| !self.edits.override_removals.contains(over));
        self.remove_relationship(source, id);
        if let Some(over) = over {
            self.remove_override(&over);
        }
        if !self
            .edits
            .part_removals
            .iter()
            .any(|gone| gone.as_slice() == target)
        {
            self.remove_part(target);
        }
        Ok(())
    }

    /// Refuses a comment edit on a sheet whose comments part or VML part,
    /// `target`, this write removes, replaces or adds through the part calls,
    /// with part 1's messages for a cell edit ("an edit to a removed part",
    /// "a cell edit to a part this write replaces"), as a cell edit on a
    /// replaced worksheet is refused (`specs/0022-comments-and-images.md`, P1
    /// verification ruling 1 (2)).
    #[cfg(any(feature = "comments", feature = "images"))]
    fn edited_part_untouched(&self, target: Option<&[u8]>) -> Result<(), Error> {
        let Some(target) = target else {
            return Ok(());
        };
        if self
            .edits
            .part_removals
            .iter()
            .any(|gone| gone.as_slice() == target)
        {
            return Err(REMOVED_PART_EDITED);
        }
        let named = |list: &[(Vec<u8>, Replacement)]| {
            list.iter().any(|(name, _)| name.as_slice() == target)
        };
        if named(&self.edits.part_replacements) || named(&self.edits.part_additions) {
            return Err(CELL_EDIT_REPLACED);
        }
        Ok(())
    }

    /// The `idmap` number of every VML part of the projected workbook: each
    /// entry whose name ends in `.vml`, ASCII case folded, that this write
    /// does not remove; a part with no `idmap` uses none.
    #[cfg(feature = "comments")]
    fn vml_idmaps(&mut self) -> Result<Vec<u64>, Error> {
        let names: Vec<Vec<u8>> = self
            .archive
            .entries()
            .iter()
            .map(|entry| entry.name().to_vec())
            .filter(|name| {
                name.to_ascii_lowercase().ends_with(b".vml")
                    && !self.edits.part_removals.contains(name)
            })
            .collect();
        let mut used = Vec::new();
        for name in names {
            let index = self.archive.index_of(&name).ok_or(NO_SUCH_PART)?;
            if let Some(block) = vml_idmap(self.archive.read(index)?, index)? {
                used.push(block);
            }
        }
        Ok(used)
    }

    /// The body of [`Package::add_image`]: the call's checks, then the image
    /// recorded (`specs/0022-comments-and-images.md`, "Images: types" and
    /// "Images: the calls").
    #[cfg(feature = "images")]
    fn add_image(&mut self, part: &[u8], cell: CellRef, image: Image<'_>) -> Result<(), Error> {
        if !in_grid(cell) {
            return Err(INDEX_OUT_OF_RANGE);
        }
        let (png, width, height) = image_header(image.bytes)?;
        let (width, height) = image.size.unwrap_or((width, height));
        let most = u64::from(i32::MAX.unsigned_abs());
        let extent = |pixels: u32| {
            let emu = u64::from(pixels) * EMU_PER_PIXEL;
            (pixels > 0 && emu <= most).then_some(emu)
        };
        let (Some(cx), Some(cy)) = (extent(width), extent(height)) else {
            return Err(IMAGE_SIZE_INVALID);
        };
        self.edits.image_edits.push((
            part.to_vec(),
            cell,
            NewImage {
                bytes: image.bytes.to_vec(),
                png,
                cx,
                cy,
            },
        ));
        Ok(())
    }

    /// The body of [`Package::images`].
    #[cfg(feature = "images")]
    fn images(&mut self, part: &[u8]) -> Result<Vec<ImageEntry>, Error> {
        let sheet = self.archive.index_of(part).ok_or(NO_SUCH_PART)?;
        let Some(target) = self.drawing_of(part, sheet)? else {
            return Ok(Vec::new());
        };
        let index = self.archive.index_of(&target).ok_or(DRAWING_UNREADABLE)?;
        let pictures = read_images(self.archive.read(index)?, index)?;
        let mut out = Vec::new();
        for picture in pictures {
            let media = {
                let rels = self.relationships(&target)?;
                let rel = rels
                    .by_id(&picture.embed)
                    .filter(|rel| !rel.external)
                    .ok_or(DRAWING_UNREADABLE)?;
                rels.target_part(rel).map_err(|_| DRAWING_UNREADABLE)?
            };
            let at = self.archive.index_of(&media).ok_or(DRAWING_UNREADABLE)?;
            out.push(ImageEntry {
                from: picture.from,
                anchor: picture.anchor,
                bytes: self.archive.entries()[at].uncompressed_size(),
                part: media,
            });
        }
        Ok(out)
    }

    /// The drawing part of the worksheet part `part` (entry `index`): the
    /// part its `<drawing>` element's `r:id` names through an internal
    /// relationship of type [`DRAWING`], either form, or `None` for a sheet
    /// with no such element, the read and the write sharing it (P3
    /// verification ruling 1 (2)). Two elements, or one out of schema
    /// order, are refused with the two-elements message; an element without
    /// `r:id`, or one naming no such relationship or a target outside the
    /// package, with the drawing read message.
    #[cfg(feature = "images")]
    fn drawing_of(&mut self, part: &[u8], index: usize) -> Result<Option<Vec<u8>>, Error> {
        let element = element_rel_id(
            self.archive.read(index)?,
            index,
            DRAWING_ELEMENT,
            TWO_DRAWINGS,
            DRAWING_UNREADABLE,
        )?;
        let Some(id) = element else {
            return Ok(None);
        };
        let rels = self.relationships(part)?;
        let rel = rels
            .by_id(&id)
            .filter(|rel| !rel.external && DRAWING.contains(&rel.kind.as_slice()))
            .ok_or(DRAWING_UNREADABLE)?;
        Ok(Some(rels.target_part(rel).map_err(|_| DRAWING_UNREADABLE)?))
    }

    /// Records each worksheet's image additions
    /// (`specs/0022-comments-and-images.md`, "Images: the parts"): a media
    /// entry per image with its `Override`; the anchors appended to the
    /// sheet's drawing part, each with an `image` relationship on its rels
    /// part, or a drawing part created with its `Override` and its rels
    /// part, the sheet's `drawing` relationship and what the worksheet pass
    /// does with `<drawing>`. Part by part, in record order: the refusals of
    /// a removed, replaced, absent or shifted part, then what the drawing
    /// cannot be read for.
    #[cfg(feature = "images")]
    fn record_images(&mut self) -> Result<(), Error> {
        if self.edits.image_edits.is_empty() {
            return Ok(());
        }
        let mut parts: Vec<(Vec<u8>, PartImages)> = Vec::new();
        for (part, cell, image) in self.edits.image_edits.clone() {
            match parts.iter_mut().find(|(have, _)| *have == part) {
                Some((_, images)) => images.push((cell, image)),
                None => parts.push((part, vec![(cell, image)])),
            }
        }
        let mut entries = self.taken_entries();
        let mut works: Vec<(Vec<u8>, DrawingWork)> = Vec::new();
        for (part, images) in parts {
            if self.edits.part_removals.contains(&part) {
                return Err(REMOVED_PART_EDITED);
            }
            if self
                .edits
                .part_replacements
                .iter()
                .any(|(name, _)| *name == part)
            {
                return Err(CELL_EDIT_REPLACED);
            }
            // A sheet this write adds has no relationships part to read yet,
            // as for a comment.
            let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
            #[cfg(feature = "rowcol")]
            if self.edits.shifts.iter().any(|(at, _)| *at == part) {
                return Err(SHIFT_ALONE);
            }
            let drawing = self.drawing_of(&part, index)?;
            // A drawing part this write removes, replaces or adds through the
            // part calls is not read for the edit, as a comment's parts are.
            self.edited_part_untouched(drawing.as_deref())?;
            let strict = self.sheet_is_strict(&part)?;
            // A media entry per image, N free over both extensions.
            let mut media: Vec<Vec<u8>> = Vec::new();
            for (_, image) in &images {
                let n = free_media(&entries);
                let (extension, content_type): (&str, &[u8]) = if image.png {
                    ("png", b"image/png")
                } else {
                    ("jpeg", b"image/jpeg")
                };
                let entry = format!("xl/media/image{n}.{extension}");
                entries.push(entry.to_ascii_lowercase());
                self.add_part(
                    entry.as_bytes(),
                    replacement_from_reader(Cursor::new(image.bytes.clone())),
                );
                self.add_override(format!("/{entry}").as_bytes(), content_type);
                media.push(entry.into_bytes());
            }
            match drawing {
                Some(target) => {
                    let at = self.archive.index_of(&target).ok_or(DRAWING_UNREADABLE)?;
                    let mut bytes = Vec::new();
                    self.archive.read(at)?.read_to_end(&mut bytes)?;
                    let read = read_drawing(&bytes, at)?;
                    let mut out = bytes[..read.end].to_vec();
                    let mut id = read.highest;
                    for ((cell, image), entry) in images.iter().zip(&media) {
                        let rel = self.add_relationship(
                            &target,
                            IMAGE[usize::from(strict)],
                            &relative_target(&target, entry),
                            false,
                        )?;
                        id = id
                            .checked_add(1)
                            .filter(|id| *id <= u64::from(u32::MAX))
                            .ok_or(DRAWING_UNREADABLE)?;
                        out.extend_from_slice(&anchor_bytes(&read.form, *cell, image, id, &rel));
                    }
                    out.extend_from_slice(&bytes[read.end..]);
                    self.replace_part(&target, replacement_from_reader(Cursor::new(out)));
                }
                None => {
                    let entry = free_entry(&entries, |n| format!("xl/drawings/drawing{n}.xml"));
                    entries.push(entry.to_ascii_lowercase());
                    let form = DrawingForm {
                        x: b"xdr:".to_vec(),
                        a: Some(b"a".to_vec()),
                        r: None,
                        strict: false,
                    };
                    let mut anchors = Vec::new();
                    let mut rels = Vec::new();
                    for (k, ((cell, image), media)) in (1u64..).zip(images.iter().zip(&media)) {
                        let rel = format!("rId{k}").into_bytes();
                        anchors.extend_from_slice(&anchor_bytes(&form, *cell, image, k, &rel));
                        rels.extend_from_slice(
                            &[
                                b"<Relationship Id=\"".as_slice(),
                                &rel,
                                b"\" Type=\"",
                                IMAGE[usize::from(strict)],
                                b"\" Target=\"",
                                &relative_target(entry.as_bytes(), media),
                                b"\"/>",
                            ]
                            .concat(),
                        );
                    }
                    self.add_part(
                        entry.as_bytes(),
                        replacement_from_reader(Cursor::new(drawing_part_bytes(&anchors))),
                    );
                    self.add_override(format!("/{entry}").as_bytes(), DRAWING_CONTENT_TYPE);
                    // The drawing's rels part, in the package's form as a
                    // created rels part is: the drawing is not in the archive
                    // yet, so it cannot be a relationship call's source.
                    self.rels_default()?;
                    let mut rels_part = rels_prefix(self.archive)?;
                    rels_part.extend_from_slice(
                        &[
                            b"<Relationships xmlns=\"".as_slice(),
                            RELATIONSHIPS_NS,
                            b"\">",
                        ]
                        .concat(),
                    );
                    rels_part.extend_from_slice(&rels);
                    rels_part.extend_from_slice(b"</Relationships>");
                    self.add_part(
                        &rels_part_name(entry.as_bytes()),
                        replacement_from_reader(Cursor::new(rels_part)),
                    );
                    let target = relative_target(&part, entry.as_bytes());
                    let id =
                        self.add_relationship(&part, DRAWING[usize::from(strict)], &target, false)?;
                    works.push((part, DrawingWork { create: id, strict }));
                }
            }
        }
        self.edits.image_works = works;
        Ok(())
    }

    /// Every entry name of the archive and of this write's additions, ASCII
    /// case folded: the names a created part may not take. An entry this
    /// write removes still counts as taken, so an addition never reuses a
    /// removed entry's name (`specs/0021-tables.md`, P1 ruling 6).
    #[cfg(any(feature = "tables", feature = "comments", feature = "images"))]
    fn taken_entries(&self) -> Vec<String> {
        self.archive
            .entries()
            .iter()
            .map(|entry| entry.name())
            .chain(
                self.edits
                    .part_additions
                    .iter()
                    .map(|(name, _)| name.as_slice()),
            )
            .map(|name| String::from_utf8_lossy(name).to_ascii_lowercase())
            .collect()
    }

    /// Whether the workbook's relationship to the worksheet part `part` is
    /// of the strict form: the file's relationship whose target is the part,
    /// or the one this write adds for a sheet it adds; transitional when
    /// there is none.
    #[cfg(any(
        feature = "metadata",
        feature = "tables",
        feature = "comments",
        feature = "images"
    ))]
    fn sheet_is_strict(&mut self, part: &[u8]) -> Result<bool, Error> {
        let workbook = self.workbook()?.part.clone();
        let added: Vec<Relationship> = self
            .edits
            .rel_additions
            .iter()
            .filter(|(source, _)| *source == workbook)
            .map(|(_, rel)| rel.clone())
            .collect();
        let rels = self.relationships(&workbook)?;
        for rel in rels.all().iter().chain(added.iter()) {
            if !rel.external
                && WORKSHEET.contains(&rel.kind.as_slice())
                && rels.target_part(rel).is_ok_and(|target| target == part)
            {
                return Ok(rel.kind.as_slice() == WORKSHEET[1]);
            }
        }
        Ok(false)
    }

    /// A defined name's scope as recorded: the entry name of the sheet
    /// named `sheet` in the projected list, or `None` for the workbook.
    #[cfg(feature = "metadata")]
    fn name_scope(&mut self, sheet: Option<&str>) -> Result<Option<Vec<u8>>, Error> {
        let list = self.projected()?;
        match sheet {
            None => Ok(None),
            Some(name) => {
                let at = position_of(&list, name).ok_or(NO_SUCH_SHEET)?;
                Ok(Some(list[at].sheet.part.clone()))
            }
        }
    }

    /// The body of [`Package::sheets`].
    fn sheets(&mut self) -> Result<Vec<Sheet>, Error> {
        Ok(self
            .projected()?
            .into_iter()
            .map(|entry| entry.sheet)
            .collect())
    }

    /// The projected list with each sheet's origin, for the sheet edits.
    fn projected(&mut self) -> Result<Vec<Projected>, Error> {
        let file = self.workbook()?.sheets.clone();
        project(&file, &self.edits.sheet_edits)
    }

    /// The body of [`Package::add_sheet`].
    fn add_sheet(&mut self, name: &str) -> Result<Vec<u8>, Error> {
        let list = self.projected()?;
        check_sheet_name(name, &list, None)?;
        let sheet_id = list
            .iter()
            .map(|entry| entry.sheet.sheet_id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or(SHEET_EDIT_STALE)?;
        let workbook = self.workbook()?.part.clone();
        let dir = match workbook.iter().rposition(|byte| *byte == b'/') {
            Some(slash) => workbook[..=slash].to_vec(),
            None => Vec::new(),
        };
        let mut n = 1u32;
        let part = loop {
            let mut candidate = dir.clone();
            candidate.extend_from_slice(format!("worksheets/sheet{n}.xml").as_bytes());
            let taken = self.archive.index_of(&candidate).is_some()
                || self
                    .edits
                    .part_additions
                    .iter()
                    .any(|(added, _)| *added == candidate);
            if !taken {
                break candidate;
            }
            n = n.checked_add(1).ok_or(SHEET_EDIT_STALE)?;
        };
        // Every read comes before the first record, so a refusal records
        // nothing; add_relationship below reads the rels part cached here.
        let covered = self.content_types()?.defaults().iter().any(|default| {
            default.extension.eq_ignore_ascii_case(b"xml")
                && default.content_type == WORKSHEET_CONTENT_TYPE
        });
        let kind = self
            .relationships(&workbook)?
            .all()
            .iter()
            .find(|rel| WORKSHEET.contains(&rel.kind.as_slice()))
            .map_or_else(|| WORKSHEET[0].to_vec(), |rel| rel.kind.clone());
        self.add_part(&part, replacement_from_reader(BLANK_WORKSHEET));
        if !covered {
            let mut part_name = b"/".to_vec();
            part_name.extend_from_slice(&part);
            self.add_override(&part_name, WORKSHEET_CONTENT_TYPE);
        }
        let rel_id = self.add_relationship(&workbook, &kind, &part[dir.len()..], false)?;
        self.edits.sheet_edits.push(SheetEdit::Add {
            name: name.to_string(),
            part: part.clone(),
            sheet_id,
            rel_id,
        });
        Ok(part)
    }

    /// The body of [`Package::rename_sheet`].
    fn rename_sheet(&mut self, name: &str, to: &str) -> Result<(), Error> {
        let list = self.projected()?;
        let at = position_of(&list, name).ok_or(NO_SUCH_SHEET)?;
        check_sheet_name(to, &list, Some(at))?;
        self.edits.sheet_edits.push(SheetEdit::Rename {
            from: name.to_string(),
            to: to.to_string(),
        });
        Ok(())
    }

    /// The body of [`Package::move_sheet`].
    fn move_sheet(&mut self, name: &str, to: usize) -> Result<(), Error> {
        let list = self.projected()?;
        let at = position_of(&list, name).ok_or(NO_SUCH_SHEET)?;
        if to >= list.len() {
            return Err(SHEET_PAST_END);
        }
        if to != at {
            self.edits.sheet_edits.push(SheetEdit::Move {
                name: name.to_string(),
                to,
            });
        }
        Ok(())
    }

    /// The body of [`Package::remove_sheet`].
    fn remove_sheet(&mut self, name: &str) -> Result<(), Error> {
        let list = self.projected()?;
        let at = position_of(&list, name).ok_or(NO_SUCH_SHEET)?;
        if list.len() == 1 {
            return Err(LAST_SHEET);
        }
        if !list
            .iter()
            .enumerate()
            .any(|(other, entry)| other != at && is_visible(&entry.sheet))
        {
            return Err(LAST_VISIBLE_SHEET);
        }
        let workbook = self.workbook()?.part.clone();
        let sheet = list[at].sheet.clone();
        let edit = SheetEdit::Remove {
            name: name.to_string(),
        };
        if list[at].origin.is_none() {
            self.drop_pending_sheet(&workbook, &sheet);
            self.edits.sheet_edits.push(edit);
            return Ok(());
        }

        // Every read comes before the first record, so a refusal records
        // nothing (P4 ruling 5).
        let file = self.workbook()?.sheets.clone();
        let mut stops: Vec<Vec<u8>> = file
            .iter()
            .enumerate()
            .filter(|(origin, _)| !list.iter().any(|entry| entry.origin == Some(*origin)))
            .map(|(_, removed)| removed.part.clone())
            .collect();
        stops.push(sheet.part.clone());
        let mut skipped = self.edits.rel_removals.clone();
        skipped.push((workbook.clone(), sheet.rel_id.clone()));
        let reached = self.reachable(&sheet.part, &[], &[])?;
        let kept: BTreeSet<Vec<u8>> = self.reachable(b"", &skipped, &stops)?.into_iter().collect();
        let mut removals: Vec<WalkRemoval> = Vec::new();
        for entry in reached {
            if kept.contains(&entry) || self.edits.part_removals.contains(&entry) {
                continue;
            }
            let rels = rels_part_name(&entry);
            let rels = (self.archive.index_of(&rels).is_some()
                && !self.edits.part_removals.contains(&rels))
            .then_some(rels);
            let over = self
                .override_of(&entry)?
                .filter(|over| !self.edits.override_removals.contains(over));
            let rels_over = match &rels {
                Some(rels) => self
                    .override_of(rels)?
                    .filter(|over| !self.edits.override_removals.contains(over)),
                None => None,
            };
            removals.push(WalkRemoval {
                entry,
                rels,
                over,
                rels_over,
            });
        }
        let has_chain = self
            .relationships(&workbook)?
            .first_of(CALC_CHAIN)
            .is_some();
        let chain = if has_chain && !self.calc_chain_pending()? {
            self.chain_removals()?
        } else {
            None
        };

        self.edits.sheet_edits.push(edit);
        self.remove_relationship(&workbook, &sheet.rel_id);
        for WalkRemoval {
            entry,
            rels,
            over,
            rels_over,
        } in removals
        {
            self.remove_part(&entry);
            if let Some(rels) = rels {
                self.remove_part(&rels);
            }
            if let Some(over) = over {
                self.remove_override(&over);
            }
            if let Some(over) = rels_over {
                self.remove_override(&over);
            }
        }
        if let Some(chain) = chain {
            self.record_chain_removals(chain);
        }
        if has_chain {
            self.edits.recalculate = true;
        }
        Ok(())
    }

    /// Every entry reachable from `root`, an entry or the package itself as
    /// the empty name, through internal relationships, transitively, in the
    /// order first reached; `root` itself first when it is an entry. A
    /// relationship in `skipped`, as `(source, id)`, is not followed, and no
    /// edge is followed out of a part in `stops`. Each rels part is read
    /// through [`Package::relationships`], which parses it once.
    fn reachable(
        &mut self,
        root: &[u8],
        skipped: &[(Vec<u8>, Vec<u8>)],
        stops: &[Vec<u8>],
    ) -> Result<Vec<Vec<u8>>, Error> {
        let mut found: Vec<Vec<u8>> = Vec::new();
        let mut seen: BTreeSet<Vec<u8>> = BTreeSet::new();
        if !root.is_empty() {
            if self.archive.index_of(root).is_none() {
                return Ok(found);
            }
            found.push(root.to_vec());
            seen.insert(root.to_vec());
        }
        let mut queue: Vec<Vec<u8>> = vec![root.to_vec()];
        let mut next = 0;
        while let Some(source) = queue.get(next).cloned() {
            next += 1;
            if stops.contains(&source) {
                continue;
            }
            let targets: Vec<Vec<u8>> = {
                let rels = self.relationships(&source)?;
                rels.all()
                    .iter()
                    .filter(|rel| {
                        !rel.external
                            && !skipped
                                .iter()
                                .any(|(from, id)| *from == source && *id == rel.id)
                    })
                    .filter_map(|rel| rels.target_part(rel).ok())
                    .collect()
            };
            for target in targets {
                if self.archive.index_of(&target).is_some() && seen.insert(target.clone()) {
                    found.push(target.clone());
                    queue.push(target);
                }
            }
        }
        Ok(found)
    }

    /// Drops what [`Package::add_sheet`] recorded for a sheet not yet
    /// written, and the cell and style edits on its part (P4 ruling 4).
    fn drop_pending_sheet(&mut self, workbook: &[u8], sheet: &Sheet) {
        let part = sheet.part.as_slice();
        let mut part_name = b"/".to_vec();
        part_name.extend_from_slice(part);
        self.edits.part_additions.retain(|(added, _)| added != part);
        self.edits
            .override_additions
            .retain(|(name, _)| *name != part_name);
        self.edits
            .rel_additions
            .retain(|(source, rel)| !(source == workbook && rel.id == sheet.rel_id));
        self.edits.cell_edits.retain(|(at, _, _)| at != part);
        self.edits.style_edits.retain(|(at, _, _)| at != part);
        self.edits.column_formats.retain(|(at, _, _)| at != part);
        self.edits.row_formats.retain(|(at, _, _)| at != part);
        self.edits.panes.retain(|(at, _)| at != part);
        self.edits.merges.retain(|(at, _)| at != part);
        self.edits.unmerges.retain(|(at, _)| at != part);
        #[cfg(feature = "rowcol")]
        self.edits.shifts.retain(|(at, _)| at != part);
        #[cfg(feature = "metadata")]
        self.edits.links.retain(|(at, _, _)| at != part);
        #[cfg(feature = "metadata")]
        self.edits.sheet_protections.retain(|(at, _)| at != part);
        #[cfg(feature = "rules")]
        self.edits.validations.retain(|(at, _)| at != part);
        #[cfg(feature = "rules")]
        self.edits.rules.retain(|(at, _)| at != part);
        #[cfg(feature = "tables")]
        self.edits.table_additions.retain(|(at, _)| at != part);
        #[cfg(feature = "tables")]
        self.edits.table_removals.retain(|(at, _)| at != part);
        #[cfg(feature = "sheetprops")]
        self.edits.tab_colors.retain(|(at, _)| at != part);
        #[cfg(feature = "sheetprops")]
        self.edits.sheet_views.retain(|(at, _)| at != part);
        #[cfg(feature = "comments")]
        self.edits.comment_edits.retain(|(at, _, _)| at != part);
        #[cfg(feature = "images")]
        self.edits.image_edits.retain(|(at, _, _)| at != part);
    }

    /// The body of [`Package::write_to`], over the writer as a trait object.
    fn write_to(mut self, out: &mut dyn Write) -> Result<(), Error> {
        // An external link's relationship is recorded before validation, so
        // the rels part it goes into is checked as any other edit's is.
        #[cfg(feature = "metadata")]
        self.record_link_relationships()?;
        #[cfg(feature = "metadata")]
        self.record_core_creation()?;
        // A table's part, override and relationship are recorded before
        // validation too (`specs/0021-tables.md`, "The part").
        #[cfg(feature = "tables")]
        self.record_tables()?;
        // A comment's parts, overrides and relationships likewise
        // (`specs/0022-comments-and-images.md`, "Comments: the parts").
        #[cfg(feature = "comments")]
        self.record_comments()?;
        // And an image's (`specs/0022-comments-and-images.md`, "Images: the
        // parts").
        #[cfg(feature = "images")]
        self.record_images()?;
        self.validate()?;
        let mut edits = Edits::new();
        // The guards outlive the write: each replacement reads its file back,
        // and Drop removes it however this call ends.
        let mut temporaries: Vec<TempFile> = Vec::new();

        // The cell edits go first, each worksheet rewritten once with all of
        // its edits, because a removed formula adds the chain's three
        // removals, which the content types and rels rewrites below apply
        // (`specs/0011-editing-cells.md`, "Module `package`").
        let mut changed_formula = self.edits.writes_formula();
        let added_sheets = self.edits.added_sheet_parts();
        for part in self.edits.cell_parts() {
            let work = self.edits.sheet_work(&part);
            #[cfg(feature = "rowcol")]
            let shift = self
                .edits
                .shifts
                .iter()
                .find(|(at, _)| *at == part)
                .map(|(_, shift)| *shift);
            match self.archive.index_of(&part) {
                Some(index) => {
                    #[cfg(feature = "rowcol")]
                    let rewritten = match shift {
                        Some(shift) => shift_worksheet(self.archive, index, &shift),
                        None => rewrite_worksheet(self.archive, index, &work),
                    };
                    #[cfg(not(feature = "rowcol"))]
                    let rewritten = rewrite_worksheet(self.archive, index, &work);
                    let (temp, replacement, removed) = rewritten?;
                    changed_formula |= removed;
                    temporaries.push(temp);
                    edits.replace(index, replacement);
                    #[cfg(feature = "metadata")]
                    self.drop_link_relationships(&part, &work)?;
                }
                None if added_sheets.contains(&part) => {
                    // A sheet this write adds: its edits go onto the template's
                    // blank worksheet, and the result is the added entry's
                    // bytes (`specs/0016-new-workbook-and-sheets.md`, P3
                    // ruling 2). Its index in an error is where it is added.
                    let slot = self
                        .edits
                        .part_additions
                        .iter()
                        .position(|(added, _)| *added == part)
                        .ok_or(NO_SUCH_PART)?;
                    let index = self.archive.entries().len() + slot;
                    #[cfg(feature = "rowcol")]
                    let rewritten = match shift {
                        Some(shift) => shift_worksheet_from(BLANK_WORKSHEET, index, &shift),
                        None => rewrite_worksheet_from(BLANK_WORKSHEET, index, &work),
                    };
                    #[cfg(not(feature = "rowcol"))]
                    let rewritten = rewrite_worksheet_from(BLANK_WORKSHEET, index, &work);
                    let (temp, replacement, removed) = rewritten?;
                    changed_formula |= removed;
                    temporaries.push(temp);
                    self.edits.part_additions[slot].1 = replacement;
                }
                None => return Err(NO_SUCH_PART),
            }
        }
        // A defined name added or removed is a formula change too
        // (`specs/0019-names-links-properties-protection.md`, decision 2 A).
        if changed_formula || !self.edits.no_name_edits() {
            self.formula_change_rule()?;
        }

        // The styles part, rewritten once when a cell format was added
        // (`specs/0012-styles.md`, "Module `package`"). `validate` has read
        // it, so the workbook names it and the counts are cached.
        #[cfg(feature = "rules")]
        let dxfs_added = !self.edits.dxf_additions.is_empty();
        #[cfg(not(feature = "rules"))]
        let dxfs_added = false;
        if !self.edits.xf_additions.is_empty() || dxfs_added {
            #[cfg(feature = "rules")]
            let dxf_children = if dxfs_added { self.dxf_children()? } else { 0 };
            let part = self.workbook()?.styles.clone().ok_or(NO_STYLES_PART)?;
            let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
            let styles = self.styles()?;
            let counts = Counts {
                num_fmts: styles.num_fmts.len() as u64,
                fonts: styles.fonts().map_or(0, u64::from),
                fills: styles.fills().map_or(0, u64::from),
                borders: styles.borders().map_or(0, u64::from),
                cell_xfs: styles.cell_xfs.len() as u64,
                has_num_fmts: styles.has_num_fmts(),
            };
            let mut codes = self.edits.num_fmt_additions.clone();
            codes.sort_by_key(|(id, _)| *id);
            let additions = StyleAdditions {
                codes: &codes,
                fonts: &self.edits.font_additions,
                fills: &self.edits.fill_additions,
                borders: &self.edits.border_additions,
                xfs: &self.edits.xf_additions,
                #[cfg(feature = "rules")]
                dxfs: dxfs_added.then_some((dxf_children, self.edits.dxf_additions.as_slice())),
            };
            let (temp, replacement) = rewrite_styles(self.archive, index, &counts, &additions)?;
            temporaries.push(temp);
            edits.replace(index, replacement);
        }

        // The core-properties part, rewritten once when a property is edited
        // and the package has one (`specs/0019-names-links-properties-
        // protection.md`, "Document properties").
        #[cfg(feature = "metadata")]
        if !self.edits.properties.is_empty()
            && let Some(part) = self.core_part()?
            && let Some(index) = self.archive.index_of(&part)
        {
            let props = property_edits(&self.edits.properties);
            let (temp, replacement) = rewrite_core(self.archive, index, &props)?;
            temporaries.push(temp);
            edits.replace(index, replacement);
        }

        // The workbook part, rewritten once when a full calculation on load
        // was asked for (`specs/0013-formulas.md`, "Module `package`") or a
        // sheet edit was recorded (`specs/0016-new-workbook-and-sheets.md`,
        // "The workbook rewrite"). `validate` has read the workbook, so its
        // part and its `calcPr` count are cached, and has replayed the sheet
        // edits, so the plan below is the one it checked.
        let plan = if self.edits.sheet_edits.is_empty() {
            None
        } else {
            let file_count = self.workbook()?.sheets.len();
            Some(SheetPlan::new(self.projected()?, file_count)).filter(SheetPlan::changes)
        };
        #[cfg(feature = "metadata")]
        let names = if self.edits.name_edits.is_empty() {
            None
        } else {
            Some(self.name_plan()?)
        };
        #[cfg(feature = "metadata")]
        let protection = match self.edits.book_protections.last() {
            Some(protection) => Some((*protection, self.workbook()?.workbook_protections == 0)),
            None => None,
        };
        // An unprotect of a workbook with no element changes nothing, so it
        // alone does not rewrite the part.
        #[cfg(feature = "metadata")]
        let protecting = protection.is_some_and(|edit| edit != (None, true));
        #[cfg(not(feature = "metadata"))]
        let protecting = false;
        if self.edits.recalculate || plan.is_some() || protecting {
            let part = self.workbook()?.part.clone();
            let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
            let book = BookEdits {
                calc: self.edits.recalculate.then_some(*self.calc_prs > 0),
                plan: plan.as_ref(),
                #[cfg(feature = "metadata")]
                names: names.as_ref(),
                #[cfg(feature = "metadata")]
                protection,
            };
            let (temp, replacement) = rewrite_workbook_part(self.archive, index, &book)?;
            temporaries.push(temp);
            edits.replace(index, replacement);
        }

        if !self.edits.override_removals.is_empty() || !self.edits.override_additions.is_empty() {
            let index = self
                .archive
                .index_of(CONTENT_TYPES_PART)
                .ok_or(NO_CONTENT_TYPES)?;
            let additions = self.override_elements(index)?;
            let removals = self.edits.override_removals.clone();
            let (temp, replacement) = rewrite_part(
                self.archive,
                index,
                &|local, start| {
                    Ok(local == b"Override" && attribute_in(start, b"PartName", &removals, index)?)
                },
                CONTENT_TYPES_NS,
                &additions,
            )?;
            temporaries.push(temp);
            edits.replace(index, replacement);
        }

        for source in self.edits.rel_sources() {
            let name = rels_part_name(&source);
            let existing = self.archive.index_of(&name);
            if existing.is_none() {
                // A part with no rels part gets one, and its content type is
                // not written anywhere: the package's Default for the rels
                // extension covers it, so this call is the check that there
                // is one and its value is deliberately unused. It runs BEFORE
                // the elements are built, which is what makes the index in an
                // Error::Xml for one of them well defined: the call reads
                // [Content_Types].xml, so that part exists by the time
                // relationship_elements falls back to its index
                // (P3 ruling 13).
                self.rels_default()?;
            }
            let additions = self.relationship_elements(&source)?;
            let removals: Vec<Vec<u8>> = self
                .edits
                .rel_removals
                .iter()
                .filter(|(at, _)| *at == source)
                .map(|(_, id)| id.clone())
                .collect();
            match existing {
                Some(index) => {
                    let (temp, replacement) = rewrite_part(
                        self.archive,
                        index,
                        &|local, start| {
                            Ok(local == b"Relationship"
                                && attribute_in(start, b"Id", &removals, index)?)
                        },
                        RELATIONSHIPS_NS,
                        &additions,
                    )?;
                    temporaries.push(temp);
                    edits.replace(index, replacement);
                }
                None => {
                    // The Default was checked above, before the elements.
                    let (temp, replacement) = new_rels_part(self.archive, &additions)?;
                    temporaries.push(temp);
                    edits.add(name, replacement);
                }
            }
        }

        for name in &self.edits.part_removals {
            let index = self.archive.index_of(name).ok_or(NO_SUCH_PART)?;
            edits.remove(index);
        }
        for (name, replacement) in self.edits.part_replacements.drain(..) {
            let index = self.archive.index_of(&name).ok_or(NO_SUCH_PART)?;
            edits.replace(index, replacement);
        }
        for (name, replacement) in self.edits.part_additions.drain(..) {
            edits.add(name, replacement);
        }

        let result = self.archive.write_to(out, edits);
        drop(temporaries);
        result
    }

    /// Records the removal of each relationship a dropped link's `r:id`
    /// named, when the part's rels part has it and nothing removes it
    /// already (`specs/0019-names-links-properties-protection.md`,
    /// "Hyperlinks").
    #[cfg(feature = "metadata")]
    fn drop_link_relationships(&mut self, part: &[u8], work: &SheetWork) -> Result<(), Error> {
        for id in work.dropped_ids.take() {
            let present = self.relationships(part)?.by_id(&id).is_some();
            let pending = self
                .edits
                .rel_removals
                .iter()
                .any(|(source, gone)| source == part && *gone == id);
            if present && !pending {
                self.remove_relationship(part, &id);
            }
        }
        Ok(())
    }

    /// The formula-change rule of [`Package::write_to`], for a write whose
    /// cell edits wrote a formula or removed a normal one
    /// (`specs/0013-formulas.md`, "Module `package`", and P1 ruling 4).
    ///
    /// The chain lists cells by their formulas, and Excel repairs a workbook
    /// whose chain names a cell with no formula, so unless the caller has
    /// already recorded the chain's relationship for removal the recipe is
    /// applied here; and the full calculation on load is recorded, whether
    /// or not there was a chain, since no cached value was written. The
    /// whole set is then validated again with both in it, which is where a
    /// workbook part that cannot be rewritten is refused for this case.
    fn formula_change_rule(&mut self) -> Result<(), Error> {
        self.drop_calc_chain_once()?;
        self.edits.recalculate = true;
        self.validate()
    }

    /// The chain half of the formula-change rule: the recipe of
    /// [`Package::drop_calc_chain`], unless the caller has already recorded
    /// the chain's relationship for removal, so it is recorded once.
    fn drop_calc_chain_once(&mut self) -> Result<(), Error> {
        if !self.calc_chain_pending()? {
            self.drop_calc_chain()?;
        }
        Ok(())
    }

    /// Whether the workbook's `calcChain` relationship is already recorded
    /// for removal, so the chain rule of [`Package::write_to`] does not
    /// record it twice.
    fn calc_chain_pending(&mut self) -> Result<bool, Error> {
        let workbook = self.workbook()?.part.clone();
        let id = match self.relationships(&workbook)?.first_of(CALC_CHAIN) {
            Some(rel) => rel.id.clone(),
            None => return Ok(false),
        };
        Ok(self
            .edits
            .rel_removals
            .iter()
            .any(|(source, gone)| *source == workbook && *gone == id))
    }

    /// Refuses every edit that cannot apply, before anything is written.
    fn validate(&mut self) -> Result<(), Error> {
        if self.edits.is_empty() {
            return Ok(());
        }
        // A part this set removes may not also be edited, and the parts whose
        // bytes a rels or override edit rewrites count as edited.
        let removed = self.edits.part_removals.clone();
        let is_removed = |name: &[u8]| removed.iter().any(|gone| gone.as_slice() == name);
        for (name, _) in &self.edits.part_replacements {
            if is_removed(name) {
                return Err(REMOVED_PART_EDITED);
            }
        }
        for (name, _) in &self.edits.part_additions {
            if is_removed(name) {
                return Err(REMOVED_PART_EDITED);
            }
        }
        for source in self.edits.rel_sources() {
            if is_removed(&rels_part_name(&source)) {
                return Err(REMOVED_PART_EDITED);
            }
        }
        if (!self.edits.override_removals.is_empty() || !self.edits.override_additions.is_empty())
            && is_removed(CONTENT_TYPES_PART)
        {
            return Err(REMOVED_PART_EDITED);
        }

        // A relationship edit's source is the package itself, written as the
        // empty name, or a part the package holds: a rels part is never
        // created for a part that does not exist (P3 ruling 13a).
        for source in self.edits.rel_sources() {
            if source.is_empty() {
                continue;
            }
            if self.archive.index_of(&source).is_none() {
                return Err(NO_SUCH_PART);
            }
        }

        // Every removal names something that is there, and no addition
        // duplicates an override the part already holds.
        for source in self.edits.rel_sources() {
            let ids: Vec<Vec<u8>> = self
                .relationships(&source)?
                .all()
                .iter()
                .map(|rel| rel.id.clone())
                .collect();
            for (at, id) in &self.edits.rel_removals {
                if *at == source && !ids.iter().any(|have| have == id) {
                    return Err(NO_SUCH_RELATIONSHIP);
                }
            }
        }
        if !self.edits.override_removals.is_empty() || !self.edits.override_additions.is_empty() {
            let present: Vec<Vec<u8>> = self
                .content_types()?
                .overrides()
                .iter()
                .map(|over| over.part_name.clone())
                .collect();
            for name in &self.edits.override_removals {
                if !present.iter().any(|have| have == name) {
                    return Err(NO_SUCH_OVERRIDE);
                }
            }
            for (name, _) in &self.edits.override_additions {
                if present.iter().any(|have| have == name)
                    && !self.edits.override_removals.iter().any(|gone| gone == name)
                {
                    return Err(OVERRIDE_EXISTS);
                }
            }
        }

        // A part removed or replaced is named by the entry it is, so a name no
        // entry carries is refused here rather than resolved to an index that
        // does not exist (P3 ruling 9). An addition is not checked: a name the
        // archive already holds is its own refusal, `Error::DuplicateName`.
        for name in &self.edits.part_removals {
            if self.archive.index_of(name).is_none() {
                return Err(NO_SUCH_PART);
            }
        }
        for (name, _) in &self.edits.part_replacements {
            if self.archive.index_of(name).is_none() {
                return Err(NO_SUCH_PART);
            }
        }

        // A cell edit names a part that is there, that this write neither
        // removes nor replaces, and carries a value the format can hold
        // (`specs/0011-editing-cells.md`, "Module `package`").
        let added_sheets = self.edits.added_sheet_parts();
        for part in self.edits.cell_parts() {
            if self.archive.index_of(&part).is_none() && !added_sheets.contains(&part) {
                return Err(NO_SUCH_PART);
            }
            if is_removed(&part) {
                return Err(REMOVED_PART_EDITED);
            }
            if self
                .edits
                .part_replacements
                .iter()
                .any(|(name, _)| *name == part)
            {
                return Err(CELL_EDIT_REPLACED);
            }
        }
        for (_, _, value) in &self.edits.cell_edits {
            if let Some(value) = value {
                value.check()?;
            }
        }
        // A cell value, clear or style edit names a cell inside the grid
        // (`specs/0023-scale.md`, P1 verification ruling 5).
        if self.edits.cell_edits.iter().any(|(_, at, _)| !in_grid(*at))
            || self
                .edits
                .style_edits
                .iter()
                .any(|(_, at, _)| !in_grid(*at))
        {
            return Err(INDEX_OUT_OF_RANGE);
        }

        // A merge overlaps no merge recorded before it for its part, and no
        // cell edit is recorded for a cell a merge covers other than its
        // top-left (`specs/0018-rows-and-columns.md`, "Merges", and P1 ruling
        // 5). The file's own merges are checked by the rewrite.
        for part in self.edits.cell_parts() {
            let work = self.edits.sheet_work(&part);
            for (at, merge) in work.merges.iter().enumerate() {
                if work.merges[..at]
                    .iter()
                    .any(|before| before.overlaps(merge))
                {
                    return Err(MERGE_OVERLAP);
                }
            }
            if work
                .cells
                .iter()
                .any(|(reference, _)| work.covers(*reference))
            {
                return Err(EDIT_IN_MERGE);
            }
            // One insertion or deletion per part and nothing else for it
            // (`specs/0018-rows-and-columns.md`, decision 5), and no table or
            // pivot table it would move or cut ("The shift rule").
            #[cfg(feature = "rowcol")]
            self.shift_alone(&part, &work)?;
            // A validation added overlaps none added before it for its part
            // (`specs/0020-validation-and-conditional-formatting.md`, "Data
            // validation"); the part's own are checked by the rewrite.
            #[cfg(feature = "rules")]
            {
                let added: Vec<&[Area]> = work
                    .validations
                    .iter()
                    .filter_map(|edit| match edit {
                        ValidationEdit::Add(added) => Some(added.areas.as_slice()),
                        ValidationEdit::Remove(_) => None,
                    })
                    .collect();
                for (at, areas) in added.iter().enumerate() {
                    if added[..at]
                        .iter()
                        .any(|before| lists_overlap(before, areas))
                    {
                        return Err(VALIDATION_OVERLAP);
                    }
                }
            }
        }
        // A rule's differential style is one the styles part has or this
        // write adds ("Conditional formatting").
        #[cfg(feature = "rules")]
        {
            let highest = self
                .edits
                .rules
                .iter()
                .filter_map(|(_, edit)| match edit {
                    RuleEdit::Add(added) => added.dxf,
                    RuleEdit::Remove(_) => None,
                })
                .max();
            if let Some(highest) = highest
                && u64::from(highest)
                    >= self.dxf_children()? + self.edits.dxf_additions.len() as u64
            {
                return Err(DXF_OUT_OF_RANGE);
            }
        }

        // A style edit names a cell format the part has or this package
        // added, for which the styles part is read; a `None` edit needs no
        // styles part at all. An addition goes into a styles part this write
        // neither removes nor replaces (`specs/0012-styles.md`, "Module
        // `package`").
        let additions = self.edits.xf_additions.len();
        if additions > 0 || self.edits.style_edits.iter().any(|(_, _, s)| s.is_some()) {
            let count = self.styles()?.cell_xfs.len() + additions;
            for (_, _, style) in &self.edits.style_edits {
                if let Some(style) = style
                    && usize::try_from(*style).map_or(true, |style| style >= count)
                {
                    return Err(STYLE_OUT_OF_RANGE);
                }
            }
        }
        #[cfg(feature = "rules")]
        let additions = additions + self.edits.dxf_additions.len();
        if additions > 0 {
            let part = self.workbook()?.styles.clone().ok_or(NO_STYLES_PART)?;
            if is_removed(&part) {
                return Err(REMOVED_PART_EDITED);
            }
            if self
                .edits
                .part_replacements
                .iter()
                .any(|(name, _)| *name == part)
            {
                return Err(STYLE_ADDITION_REPLACED);
            }
        }

        // A write that will rewrite the workbook part, for the flag recorded
        // or for the formula-change rule a written formula fires, rewrites a
        // part this write neither removes nor replaces and that has at most
        // one `calcPr` (`specs/0013-formulas.md`, P1 ruling 3). A rule fired
        // by a removed formula is checked when the rule validates again.
        let calc =
            self.edits.recalculate || self.edits.writes_formula() || !self.edits.no_name_edits();
        let sheets = !self.edits.sheet_edits.is_empty();
        #[cfg(feature = "metadata")]
        let protecting = !self.edits.book_protections.is_empty();
        #[cfg(not(feature = "metadata"))]
        let protecting = false;
        #[cfg(feature = "metadata")]
        if protecting && self.workbook()?.workbook_protections > 1 {
            return Err(TWO_BOOK_ELEMENTS);
        }
        if calc || sheets || protecting {
            let part = self.workbook()?.part.clone();
            if is_removed(&part)
                || self
                    .edits
                    .part_replacements
                    .iter()
                    .any(|(name, _)| *name == part)
            {
                return Err(WORKBOOK_REPLACED);
            }
            if calc && *self.calc_prs > 1 {
                return Err(TWO_CALC_PRS);
            }
        }
        // The sheet edits are replayed on the file's list: each was checked
        // when recorded, so this refuses only a set that no longer holds, the
        // write's defence (`specs/0016-new-workbook-and-sheets.md`, "The
        // projected sheet list").
        if sheets {
            self.projected()?;
        }
        // A property edit, a set or a removal, on a package whose
        // core-properties relationship names a part the archive lacks is
        // refused, as the read refuses it, rather than lost at the write
        // (`specs/0019-names-links-properties-protection.md`, P3 ruling 1).
        #[cfg(feature = "metadata")]
        if !self.edits.properties.is_empty()
            && let Some(part) = self.core_part()?
            && self.archive.index_of(&part).is_none()
        {
            return Err(NO_SUCH_PART);
        }
        // A core-properties part the edits rewrite is one this write
        // neither removes nor replaces (`specs/0019-names-links-properties-
        // protection.md`, "Document properties").
        #[cfg(feature = "metadata")]
        if !self.edits.properties.is_empty()
            && let Some(part) = self.core_part()?
            && self.archive.index_of(&part).is_some()
            && (is_removed(&part)
                || self
                    .edits
                    .part_replacements
                    .iter()
                    .any(|(name, _)| *name == part))
        {
            return Err(REMOVED_PART_EDITED);
        }
        // The defined names resolve and are neither doubled nor missing
        // (`specs/0019-names-links-properties-protection.md`, "Defined
        // names").
        #[cfg(feature = "metadata")]
        if !self.edits.name_edits.is_empty() {
            self.name_plan()?;
        }
        Ok(())
    }

    /// The defined-name edits checked against the file and one another and
    /// resolved for the workbook rewrite: a second `<definedNames>`, an
    /// addition whose name and scope the file has or another edit of this
    /// write names (the additions first, so an addition and a removal of one
    /// name and scope are a duplicate whatever their order), an addition's
    /// scope no longer in the projected list, and a removal matching none of
    /// the file's names, are each refused.
    #[cfg(feature = "metadata")]
    fn name_plan(&mut self) -> Result<NamePlan, Error> {
        let workbook = self.workbook()?;
        if workbook.defined_names_containers > 1 {
            return Err(TWO_BOOK_ELEMENTS);
        }
        let containers = workbook.defined_names_containers;
        let file = workbook.defined_names.clone();
        let sheets: Vec<Vec<u8>> = workbook.sheets.iter().map(|s| s.part.clone()).collect();
        let plan = SheetPlan::new(self.projected()?, sheets.len());
        let edits = self.edits.name_edits.clone();
        // Whether a file name has that scope: its `localSheetId` names the
        // sheet whose entry is `scope`, or it has none and `scope` is `None`.
        let file_scope = |sheet: Option<u32>, scope: &Option<Vec<u8>>| match (sheet, scope) {
            (None, None) => true,
            (Some(at), Some(part)) => {
                usize::try_from(at).is_ok_and(|at| sheets.get(at) == Some(part))
            }
            _ => false,
        };
        let key = |edit: &NameEdit| match edit {
            NameEdit::Add { name, sheet, .. } | NameEdit::Remove { name, sheet } => {
                (folded(name), sheet.clone())
            }
        };
        let mut added = Vec::new();
        for (at, edit) in edits.iter().enumerate() {
            let NameEdit::Add { name, text, sheet } = edit else {
                continue;
            };
            let (folded_name, _) = key(edit);
            if file
                .iter()
                .any(|have| folded(&have.name) == folded_name && file_scope(have.sheet, sheet))
            {
                return Err(NAME_EXISTS);
            }
            let doubled = edits.iter().enumerate().any(|(other, earlier)| {
                other != at
                    && key(earlier) == key(edit)
                    && (other < at || matches!(earlier, NameEdit::Remove { .. }))
            });
            if doubled {
                return Err(NAME_EXISTS);
            }
            let index = match sheet {
                None => None,
                Some(part) => {
                    let at = plan
                        .list
                        .iter()
                        .position(|entry| entry.sheet.part == *part)
                        .ok_or(NO_SUCH_SHEET)?;
                    Some(u32::try_from(at).map_err(|_| SHEET_EDIT_STALE)?)
                }
            };
            added.push((name.clone(), text.clone(), index));
        }
        let mut removed: Vec<usize> = Vec::new();
        for edit in &edits {
            let NameEdit::Remove { name, sheet } = edit else {
                continue;
            };
            let folded_name = folded(name);
            let before = removed.len();
            for (ordinal, have) in file.iter().enumerate() {
                if folded(&have.name) == folded_name && file_scope(have.sheet, sheet) {
                    removed.push(ordinal);
                }
            }
            if removed.len() == before {
                return Err(NO_SUCH_NAME);
            }
        }
        // A name whose sheet the write removes goes with it, as the rewrite
        // drops it (`specs/0016-new-workbook-and-sheets.md`, P3 ruling 5).
        let kept = file
            .iter()
            .enumerate()
            .filter(|(ordinal, have)| {
                let with_sheet = plan.reindexed
                    && have.sheet.is_some_and(|at| {
                        usize::try_from(at)
                            .is_ok_and(|at| at < plan.file_count && plan.positions[at].is_none())
                    });
                !removed.contains(ordinal) && !with_sheet
            })
            .count();
        Ok(NamePlan {
            drop: containers == 1 && kept == 0 && added.is_empty(),
            create: containers == 0 && !added.is_empty(),
            removed,
            added,
        })
    }

    /// Refuses a second insertion or deletion for `part`, or one beside any
    /// other edit of it (`specs/0018-rows-and-columns.md`, decision 5), and
    /// one that would move or cut a table or a pivot table.
    #[cfg(feature = "rowcol")]
    fn shift_alone(&mut self, part: &[u8], work: &SheetWork) -> Result<(), Error> {
        let shifts: Vec<AxisShift> = self
            .edits
            .shifts
            .iter()
            .filter(|(at, _)| *at == part)
            .map(|(_, shift)| *shift)
            .collect();
        if let Some(shift) = shifts.first() {
            let alone = shifts.len() == 1
                && work.cells.is_empty()
                && work.rows.is_empty()
                && work.columns.is_empty()
                && work.pane.is_none()
                && work.merges.is_empty()
                && work.unmerges.is_empty()
                && no_links(work)
                && no_rules(work)
                && no_tables(work)
                && no_settings(work);
            if !alone {
                return Err(SHIFT_ALONE);
            }
            self.tables_before(part, shift)?;
        }
        Ok(())
    }

    /// Refuses a shift that would move or cut a table or a pivot table the
    /// worksheet `part` names (`specs/0018-rows-and-columns.md`, "The shift
    /// rule", and P2 ruling 2): each `table` relationship's part's root `ref`
    /// and each `pivotTable` relationship's part's `<location ref>` must lie
    /// wholly before the point. A part that is not there, or a `ref` that is
    /// absent or does not parse, is refused the same way.
    #[cfg(feature = "rowcol")]
    fn tables_before(&mut self, part: &[u8], shift: &AxisShift) -> Result<(), Error> {
        if self.archive.index_of(part).is_none() {
            // A sheet this write adds has no relationships part yet.
            return Ok(());
        }
        let rels = self.relationships(part)?;
        let mut targets: Vec<(Vec<u8>, bool)> = Vec::new();
        for (kind, location) in [(TABLE, false), (PIVOT_TABLE, true)] {
            for form in kind {
                for rel in rels.of_kind(form) {
                    targets.push((rels.target_part(rel).map_err(|_| TABLE_MOVED)?, location));
                }
            }
        }
        for (target, location) in targets {
            let index = self.archive.index_of(&target).ok_or(TABLE_MOVED)?;
            let value = part_ref(self.archive, index, location)?.ok_or(TABLE_MOVED)?;
            let area = Area::parse(&value).ok_or(TABLE_MOVED)?;
            if shift.of(area.last) >= shift.shift.at {
                return Err(TABLE_MOVED);
            }
        }
        Ok(())
    }

    /// The `Default` covering the `rels` extension, which a created rels part
    /// takes its content type from.
    fn rels_default(&mut self) -> Result<Vec<u8>, Error> {
        let types = self.content_types()?;
        for def in types.defaults() {
            if def.extension.eq_ignore_ascii_case(b"rels") {
                return Ok(def.content_type.clone());
            }
        }
        Err(NO_RELS_DEFAULT)
    }

    /// The `<Override/>` elements to add, serialised.
    fn override_elements(&self, index: usize) -> Result<Vec<Vec<u8>>, Error> {
        self.edits
            .override_additions
            .iter()
            .map(|(name, content_type)| {
                let mut out = b"<Override PartName=\"".to_vec();
                out.extend_from_slice(escaped(name, index)?.as_bytes());
                out.extend_from_slice(b"\" ContentType=\"");
                out.extend_from_slice(escaped(content_type, index)?.as_bytes());
                out.extend_from_slice(b"\"/>");
                Ok(out)
            })
            .collect()
    }

    /// The `<Relationship/>` elements to add to `source`'s rels part.
    ///
    /// The index an unwritable value is reported against is the rels part's
    /// when the package has one. When it does not, the part is about to be
    /// created and has no index, so the error names the SOURCE part instead
    /// of entry 0, which is some unrelated entry (P3 ruling 13b); for the
    /// package's own relationships, whose source is the empty name and has no
    /// entry either, it names `[Content_Types].xml`, the one part every
    /// package has. [`Package::validate`] has already refused a source that
    /// is neither, so no case is left over.
    fn relationship_elements(&self, source: &[u8]) -> Result<Vec<Vec<u8>>, Error> {
        let index = self
            .archive
            .index_of(&rels_part_name(source))
            .or_else(|| self.archive.index_of(source))
            .or_else(|| self.archive.index_of(CONTENT_TYPES_PART))
            // Unreachable: a source is the package or an entry, and a package
            // whose content types part is missing is refused before this by
            // the Default check that a created rels part needs. Entry 0
            // rather than a panic, because a wrong index in a message is not
            // worth a crash in a library.
            .unwrap_or(0);
        self.edits
            .rel_additions
            .iter()
            .filter(|(at, _)| at.as_slice() == source)
            .map(|(_, rel)| {
                let mut out = b"<Relationship Id=\"".to_vec();
                out.extend_from_slice(escaped(&rel.id, index)?.as_bytes());
                out.extend_from_slice(b"\" Type=\"");
                out.extend_from_slice(escaped(&rel.kind, index)?.as_bytes());
                out.extend_from_slice(b"\" Target=\"");
                out.extend_from_slice(escaped(&rel.target, index)?.as_bytes());
                if rel.external {
                    out.extend_from_slice(b"\" TargetMode=\"External\"/>");
                } else {
                    out.extend_from_slice(b"\"/>");
                }
                Ok(out)
            })
            .collect()
    }

    /// [`Parts::content_types`], consuming the view so that the result borrows
    /// the package itself.
    fn into_content_types(mut self) -> Result<&'a ContentTypes, Error> {
        self.content_types()?;
        let slot = self.content_types;
        // Filled by the call above.
        Ok(slot.as_ref().unwrap())
    }

    /// [`Parts::relationships`], consuming the view so that the result borrows
    /// the package itself.
    fn into_relationships(mut self, source: &[u8]) -> Result<&'a Relationships, Error> {
        self.relationships(source)?;
        let all = self.relationships;
        // The call above found or added the set for `source`.
        Ok(all.iter().find(|rels| rels.source == source).unwrap())
    }

    /// [`Parts::workbook`], consuming the view so that the result borrows
    /// the package itself.
    fn into_workbook(mut self) -> Result<&'a Workbook, Error> {
        self.workbook()?;
        let slot = self.workbook;
        // Filled by the call above.
        Ok(slot.as_ref().unwrap())
    }

    /// [`Parts::styles`], consuming the view so that the result borrows
    /// the package itself.
    fn into_styles(mut self) -> Result<&'a Styles, Error> {
        self.styles()?;
        let slot = self.styles;
        // Filled by the call above.
        Ok(slot.as_ref().unwrap())
    }
}

/// The rels part of `source`, or the package's own when `source` is empty.
///
/// `<dir>/<base>` keeps its `<dir>/_rels/<base>.rels`; the package's is
/// `_rels/.rels`, which is the same rule with an empty base.
pub fn rels_part_name(source: &[u8]) -> Vec<u8> {
    let mut name = Vec::with_capacity(source.len() + 13);
    match source.iter().rposition(|byte| *byte == b'/') {
        Some(slash) => {
            name.extend_from_slice(&source[..=slash]);
            name.extend_from_slice(b"_rels/");
            name.extend_from_slice(&source[slash + 1..]);
        }
        None => {
            name.extend_from_slice(b"_rels/");
            name.extend_from_slice(source);
        }
    }
    name.extend_from_slice(b".rels");
    name
}

/// The part name a rels part's own name says its relationships belong to.
///
/// The inverse of [`rels_part_name`], or `None` for an entry that is not
/// named as a rels part. The survey uses it to find the source of every
/// `.rels` entry it sees.
pub fn rels_part_source(name: &[u8]) -> Option<Vec<u8>> {
    let base = name.strip_suffix(b".rels")?;
    let (directory, file) = match base.iter().rposition(|byte| *byte == b'/') {
        Some(slash) => (&base[..=slash], &base[slash + 1..]),
        None => (&base[..0], base),
    };
    let directory = directory.strip_suffix(b"_rels/")?;
    let mut source = Vec::with_capacity(directory.len() + file.len());
    source.extend_from_slice(directory);
    source.extend_from_slice(file);
    Some(source)
}

/// Whether `namespace` is either form of the main spreadsheet namespace.
fn is_main(namespace: Option<&[u8]>) -> bool {
    matches!(namespace, Some(ns) if MAIN.contains(&ns))
}

/// Whether `namespace` is either form of the `r:id` namespace.
fn is_relationships_attribute(namespace: Option<&[u8]>) -> bool {
    matches!(namespace, Some(ns) if RELATIONSHIPS_ATTRIBUTE.contains(&ns))
}

/// The sheet kind a relationship type names.
fn sheet_kind(kind: &[u8]) -> SheetKind {
    if WORKSHEET.contains(&kind) {
        SheetKind::Worksheet
    } else if CHARTSHEET.contains(&kind) {
        SheetKind::Chartsheet
    } else if DIALOGSHEET.contains(&kind) {
        SheetKind::Dialogsheet
    } else {
        SheetKind::Other
    }
}

/// The resolved part of the first relationship of either form of `kind`.
fn resolved_first(rels: &Relationships, kind: [&[u8]; 2]) -> Result<Option<Vec<u8>>, Error> {
    match rels.first_of(kind) {
        Some(rel) => Ok(Some(rels.target_part(rel)?)),
        None => Ok(None),
    }
}

/// What a `Default`, `Override` or `Relationship` missing one of its
/// required attributes is refused with (P2 ruling 4).
///
/// One message for all three, because the rule is one rule: an element that
/// does not carry the attributes its kind is made of declares nothing, and a
/// part holding one is corrupt as a package. The layer already refuses a
/// `<sheet>` that lacks `name`, `sheetId` or `r:id` the same way.
const INCOMPLETE: Error =
    Error::Package("a Default, Override or Relationship element lacks a required attribute");

/// What [`Package::write_to`] refuses an edit with, one message each.
///
/// Six of the seven are P3's own (`specs/0009-package-layer.md`, "Module
/// `package`", and P3 ruling 9 for "no such part"); the first is P2's
/// message, named here so that the two places that raise it cannot drift
/// apart. They are constants rather than literals at the point of refusal
/// because several are raised from more than one place, and a message that
/// reads two ways in one crate is the defect the catalogue exists to catch.
const NO_CONTENT_TYPES: Error = Error::Package("no [Content_Types].xml part");
const NO_SUCH_RELATIONSHIP: Error = Error::Package("no such relationship");
const NO_SUCH_OVERRIDE: Error = Error::Package("no such override");
const OVERRIDE_EXISTS: Error = Error::Package("an override for that part already exists");
const REMOVED_PART_EDITED: Error = Error::Package("an edit to a removed part");
const NO_RELS_DEFAULT: Error = Error::Package("no Default covers rels");
const NO_SUCH_PART: Error = Error::Package("no such part");

/// What [`Package::write_to`] refuses a cell edit with
/// (`specs/0011-editing-cells.md`, "Errors"). The two about a value are
/// raised by [`Value`] itself, in `cells`.
const CELL_EDIT_REPLACED: Error = Error::Package("a cell edit to a part this write replaces");
const UNMODELLED_ATTRIBUTE: Error =
    Error::Package("a cell carries an attribute this layer does not model");
const NO_EXPLICIT_REFERENCES: Error =
    Error::Package("a worksheet without explicit references cannot be edited");
const OUT_OF_ORDER: Error =
    Error::Package("a worksheet whose rows or cells are out of order cannot be edited");
const IN_SHARED_OR_ARRAY: Error =
    Error::Package("a cell in a shared, array or data table formula cannot be edited");
const NO_SHEET_DATA: Error = Error::Package("a worksheet without sheetData cannot be edited");

/// What [`Package::styles`], [`Package::add_cell_style`] and
/// [`Package::write_to`] refuse a style with (`specs/0012-styles.md`,
/// "Errors"); the reading's other four are raised by `styles`.
const NO_STYLES_PART: Error = Error::Package("the workbook has no styles part");
pub(crate) const STYLE_OUT_OF_RANGE: Error = Error::Package("a style index is out of range");
const BUILT_IN_164: Error = Error::Package("a built-in number format id is 164 or more");
const CODE_REFUSED: Error =
    Error::Package("a number format code is empty or holds a character XML cannot carry");
const STYLE_ADDITION_REPLACED: Error =
    Error::Package("a style addition to a part this write replaces");
const NO_CELL_XFS: Error = Error::Package("a styles part without cellXfs cannot take a style");
/// What [`Package::add_style`] and [`Package::write_to`] refuse a font, a
/// fill or a border with when the part lacks its collection
/// (`specs/0017-fuller-styles.md`, "Errors" and P1 ruling 1); the two
/// messages about a field are raised by `styles`.
const NEEDS_COLLECTION: Error =
    Error::Package("a style addition needs a collection the styles part lacks");

/// The id a font, a fill or a border gets: the collection's `count` plus its
/// `position` among the distinct ones recorded, or plus `recorded` when it is
/// new.
///
/// # Errors
///
/// "a style addition needs a collection the styles part lacks" for a part
/// without the collection, and "a style index is out of range" for an id
/// `u32` cannot hold.
fn collection_id(
    count: Option<u32>,
    position: Option<usize>,
    recorded: usize,
) -> Result<u32, Error> {
    let count = count.ok_or(NEEDS_COLLECTION)?;
    u32::try_from(position.unwrap_or(recorded))
        .ok()
        .and_then(|offset| count.checked_add(offset))
        .ok_or(STYLE_OUT_OF_RANGE)
}

/// What [`Package::write_to`] refuses a workbook rewrite with
/// (`specs/0013-formulas.md`, "Errors"); the three about a formula's text are
/// raised by [`Value`] itself, in `cells`.
const TWO_CALC_PRS: Error = Error::Package("a workbook has two calcPr elements");
const SHEET_NAME_EMPTY: Error = Error::Package("a sheet name is empty");
const SHEET_NAME_LONG: Error = Error::Package("a sheet name is longer than 31 characters");
const SHEET_NAME_CHARACTER: Error = Error::Package("a sheet name holds a character Excel refuses");
const SHEET_NAME_APOSTROPHE: Error =
    Error::Package("a sheet name starts or ends with an apostrophe");
const SHEET_NAME_HISTORY: Error = Error::Package("a sheet name is the reserved name History");
const SHEET_NAME_IN_USE: Error = Error::Package("a sheet name is already in use");
/// Raised by `Value::Text` too, and with the Cargo feature `richtext` by a
/// rich value's run; a sheet name uses the same words
/// (`specs/0016-new-workbook-and-sheets.md`, P3 ruling 3). One definition,
/// which `cells` takes (`specs/0024-rich-text.md`, "Errors").
pub(crate) const TEXT_NOT_XML: Error = Error::Package("a text holds a character XML cannot carry");
const NO_SUCH_SHEET: Error = Error::Package("no sheet of that name");
const SHEET_PAST_END: Error = Error::Package("a sheet position is past the end");
const LAST_SHEET: Error = Error::Package("the last sheet cannot be removed");
const LAST_VISIBLE_SHEET: Error = Error::Package("the last visible sheet cannot be removed");
const NO_RELATIONSHIPS_PREFIX: Error =
    Error::Package("the workbook root binds no prefix for the relationships namespace");
const SHEETS_NOT_SHEETS: Error =
    Error::Package("a sheets element holds something other than sheet elements");
const SHEET_ADDED_TO_PREFIXED: Error =
    Error::Package("a sheet cannot be added to a prefixed workbook");
const SHEET_EDIT_STALE: Error = Error::Package("a sheet edit no longer applies at the write");
const INDEX_OUT_OF_RANGE: Error = Error::Package("a row or column index or count is out of range");
const SIZE_OUT_OF_RANGE: Error = Error::Package("a column width or a row height is out of range");
const FREEZE_AT_A1: Error = Error::Package("a freeze at A1 freezes nothing");
const MERGE_ONE_CELL: Error = Error::Package("a merge needs more than one cell");
const MERGE_OVERLAP: Error =
    Error::Package("a merge overlaps a merge the worksheet has, or a mergeCell ref does not parse");
const NO_SUCH_MERGE: Error = Error::Package("no merge of that range");
const EDIT_IN_MERGE: Error = Error::Package("a cell edit inside a merge this write records");
const COLS_DISORDER: Error =
    Error::Package("a worksheet whose cols are out of order or overlap cannot be edited");
const TWO_ELEMENTS: Error = Error::Package(
    "a worksheet has two cols, sheetViews or mergeCells elements, or a sheetViews without a sheetView",
);

#[cfg(feature = "rowcol")]
const SHIFT_ALONE: Error = Error::Package(
    "a worksheet with a row or column insertion or deletion takes no other edit in the same write",
);
#[cfg(feature = "rowcol")]
const OFF_GRID: Error = Error::Package("a row or column would be moved off the worksheet");
#[cfg(feature = "rowcol")]
const CUTS_FORMULA: Error =
    Error::Package("an insertion or deletion cuts a shared, array or data table formula");
#[cfg(feature = "rowcol")]
const TABLE_MOVED: Error =
    Error::Package("an insertion or deletion would move a table or a pivot table");
#[cfg(feature = "rowcol")]
const RANGE_UNPARSED: Error =
    Error::Package("a range an insertion or deletion would move does not parse");
#[cfg(feature = "metadata")]
const DEFINED_NAME_INVALID: Error = Error::Package("a defined name is not valid");
#[cfg(feature = "metadata")]
const NAME_EXISTS: Error = Error::Package("a defined name of that name and scope already exists");
#[cfg(feature = "metadata")]
const NO_SUCH_NAME: Error = Error::Package("no defined name of that name and scope");
#[cfg(feature = "metadata")]
const TWO_BOOK_ELEMENTS: Error =
    Error::Package("a workbook has two definedNames or workbookProtection elements");
#[cfg(feature = "metadata")]
const LINK_EMPTY: Error = Error::Package("a hyperlink target or location is empty");
#[cfg(feature = "metadata")]
const NO_SUCH_LINK: Error = Error::Package("no hyperlink on that cell");
#[cfg(feature = "metadata")]
const TWO_SHEET_ELEMENTS: Error =
    Error::Package("a worksheet has two hyperlinks or sheetProtection elements");
#[cfg(feature = "metadata")]
const PROPERTY_INVALID: Error = Error::Package("a property value is not valid");
#[cfg(feature = "metadata")]
const PASSWORD_INVALID: Error = Error::Package("a password is not valid");
#[cfg(feature = "metadata")]
const CORE_NO_PREFIX: Error =
    Error::Package("a core properties part binds no prefix for a namespace the edit needs");
#[cfg(feature = "metadata")]
const CORE_TWO: Error = Error::Package("a core properties part has two elements of one property");
#[cfg(feature = "rules")]
const NO_AREA: Error = Error::Package("a validation or a conditional format has no area");
#[cfg(feature = "rules")]
const FORMULA_MISSING: Error = Error::Package("a validation or a rule formula is empty or missing");
#[cfg(feature = "rules")]
const MESSAGE_LONG: Error = Error::Package("a validation message is too long");
#[cfg(feature = "rules")]
const FIELD_NOT_OF_KIND: Error = Error::Package("a validation field does not apply to its kind");
#[cfg(feature = "rules")]
const VALIDATION_OVERLAP: Error = Error::Package(
    "a validation overlaps one the worksheet has, or a dataValidation sqref does not parse",
);
#[cfg(feature = "rules")]
const NO_SUCH_VALIDATION: Error = Error::Package("no validation on those areas");
#[cfg(feature = "rules")]
const TWO_VALIDATIONS: Error = Error::Package("a worksheet has two dataValidations elements");
#[cfg(feature = "rules")]
const DXF_FIELD: Error =
    Error::Package("a differential style holds a field this part does not write");
#[cfg(feature = "rules")]
const TWO_DXFS: Error = Error::Package("a styles part has two dxfs elements");
#[cfg(feature = "rules")]
const DXF_OUT_OF_RANGE: Error = Error::Package("a differential style index is out of range");
#[cfg(feature = "rules")]
const RULE_STYLE: Error =
    Error::Package("a conditional format rule and its differential style do not agree");
#[cfg(feature = "rules")]
const NO_SUCH_RULE: Error = Error::Package("no conditional format on those areas");
#[cfg(feature = "rules")]
const VISUAL_VALUE: Error =
    Error::Package("a colour scale, data bar or icon set value is not valid");
#[cfg(feature = "rules")]
const RULE_EXTENDED: Error =
    Error::Package("a conditional format has an extension this part does not remove");
#[cfg(feature = "rules")]
const RULE_UNPARSED: Error =
    Error::Package("a conditionalFormatting sqref or a cfRule attribute does not parse");
#[cfg(feature = "tables")]
const TABLE_NAME_INVALID: Error = Error::Package("a table name is not valid");
#[cfg(feature = "tables")]
const TABLE_COLUMN_INVALID: Error = Error::Package("a table column name is not valid");
#[cfg(feature = "tables")]
const TABLE_STYLE_RANGE: Error = Error::Package("a table style number is out of range");
#[cfg(feature = "tables")]
const TABLE_TOO_SMALL: Error = Error::Package("a table area is too small");
#[cfg(feature = "tables")]
const TABLE_NAME_IN_USE: Error = Error::Package("a table name is already in use");
#[cfg(feature = "tables")]
const TABLE_OVERLAP: Error =
    Error::Package("a table overlaps a table, a pivot table, a merge or the autofilter");
#[cfg(feature = "tables")]
const TABLE_HEADER_FORMULA: Error = Error::Package("a table header cell holds a formula");
#[cfg(feature = "tables")]
const NO_SUCH_TABLE: Error = Error::Package("no table of that name");
#[cfg(feature = "tables")]
const TWO_TABLE_PARTS: Error = Error::Package("a worksheet has two tableParts elements");
#[cfg(feature = "tables")]
const TABLE_UNREADABLE: Error = Error::Package("a table part or its relationship cannot be read");
#[cfg(feature = "comments")]
const COMMENT_AUTHOR_INVALID: Error = Error::Package("a comment author is not valid");
#[cfg(feature = "comments")]
const COMMENT_TEXT_INVALID: Error = Error::Package("a comment text is not valid");
#[cfg(feature = "comments")]
const THREADED_COMMENTS_HELD: Error = Error::Package("a worksheet has threaded comments");
#[cfg(feature = "comments")]
const TWO_LEGACY_DRAWINGS: Error = Error::Package("a worksheet has two legacyDrawing elements");
#[cfg(feature = "comments")]
const NO_SUCH_COMMENT: Error = Error::Package("no comment on that cell");
#[cfg(feature = "comments")]
const COMMENTS_UNREADABLE: Error = Error::Package("a comments part or its drawing cannot be read");

/// The messages of the Cargo feature `images`
/// (`specs/0022-comments-and-images.md`, "Errors").
#[cfg(feature = "images")]
const IMAGE_NOT_PNG_OR_JPEG: Error = Error::Package("an image is not a PNG or JPEG");
#[cfg(feature = "images")]
const IMAGE_SIZE_INVALID: Error = Error::Package("an image size is out of range");
#[cfg(feature = "images")]
const TWO_DRAWINGS: Error = Error::Package("a worksheet has two drawing elements");

/// A sheet setting a call cannot record: a zoom outside `10..=400`, a tab
/// colour's tint not finite or outside `-1.0..=1.0`
/// (`specs/0025-sheet-and-print-settings.md`, "Errors"). The Cargo feature
/// `sheetprops`.
#[cfg(feature = "sheetprops")]
const SETTING_OUT_OF_RANGE: Error = Error::Package("a sheet setting is out of range");

/// A worksheet whose settings element the write cannot place: two of one,
/// or one after a root child the schema puts after it ("The pass"). The
/// Cargo feature `sheetprops`.
#[cfg(feature = "sheetprops")]
const TWO_SETTINGS: Error = Error::Package(
    "a worksheet has two sheetPr, autoFilter, printOptions, pageMargins, pageSetup or headerFooter elements",
);
#[cfg(feature = "images")]
const DRAWING_UNREADABLE: Error =
    Error::Package("a drawing part or its relationship cannot be read");

/// The grid a worksheet's indices lie in, as the `xlsx` format's
/// applications bound it (`specs/0018-rows-and-columns.md`, "Module
/// `package`: the calls").
const GRID_ROWS: u32 = 1_048_576;
const GRID_COLUMNS: u32 = 16_384;

/// Whether a cell lies inside the grid.
fn in_grid(cell: CellRef) -> bool {
    (1..=GRID_ROWS).contains(&cell.row) && (1..=GRID_COLUMNS).contains(&cell.col)
}

/// The areas a validation or a conditional format names, each normalised:
/// refused when there is none or a corner lies off the grid
/// (`specs/0020-validation-and-conditional-formatting.md`, "Areas and
/// formulas", and P1 ruling 5).
#[cfg(feature = "rules")]
fn checked_areas(areas: &[Area]) -> Result<Vec<Area>, Error> {
    if areas.is_empty() {
        return Err(NO_AREA);
    }
    let areas: Vec<Area> = areas.iter().map(|area| area.normalised()).collect();
    if areas
        .iter()
        .any(|area| !in_grid(area.first) || !in_grid(area.last))
    {
        return Err(INDEX_OUT_OF_RANGE);
    }
    Ok(areas)
}

/// Refuses a validation's or a rule's formula that starts with `=` (P1
/// ruling 4) or holds a character XML cannot carry; an empty one is the
/// caller's to refuse.
#[cfg(feature = "rules")]
fn rule_formula_ok(formula: &str) -> Result<(), Error> {
    if formula.starts_with('=') {
        return Err(Error::Package("a formula text starts with an equals sign"));
    }
    if formula.chars().any(crate::cells::forbidden_in_xml) {
        return Err(TEXT_NOT_XML);
    }
    Ok(())
}

const WORKBOOK_REPLACED: Error =
    Error::Package("the workbook part cannot be edited in a package that replaces or removes it");

/// The value of the unprefixed attribute `name`, bytes as written.
///
/// Namespace declarations are skipped: `xmlns` and `xmlns:<p>` are not
/// attributes of the element in the sense any caller here means.
fn written_attribute(
    start: &BytesStart<'_>,
    name: &[u8],
    index: usize,
) -> Result<Option<Vec<u8>>, Error> {
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (local, prefix) = attribute.key.decompose();
        if prefix.is_none() && local.into_inner().as_bytes() == name {
            return Ok(Some(attribute.value.as_bytes().to_vec()));
        }
    }
    Ok(None)
}

/// Whether the element's unprefixed attribute `name` holds one of `values`.
///
/// Compared by the bytes the attribute writes, which is what the parsed
/// `Override` and `Relationship` records hold, so a caller's value taken from
/// one of those matches here (exact bytes; Limits).
fn attribute_in(
    start: &BytesStart<'_>,
    name: &[u8],
    values: &[Vec<u8>],
    index: usize,
) -> Result<bool, Error> {
    match written_attribute(start, name, index)? {
        Some(value) => Ok(values.contains(&value)),
        None => Ok(false),
    }
}

/// `value` escaped for an attribute, or [`Error::Xml`] when it is not UTF-8.
///
/// The crate's reader decodes text and markup as `str` (P1 ruling 2), so a
/// value the layer writes has to be `str` too; `index` is the entry the value
/// would have been written into, which is all the error carries.
fn escaped(value: &[u8], index: usize) -> Result<String, Error> {
    let text = str::from_utf8(value).map_err(|_| Error::Xml(index))?;
    Ok(escape(text).into_owned())
}

/// The `<n>` of an id written exactly `rId<digits>`, `None` for any other
/// form.
///
/// A value too large for `u32` is another form: it is not an id this layer
/// would have generated, and treating it as one would make the next id
/// collide.
fn numbered_id(id: &[u8]) -> Option<u32> {
    let digits = id.strip_prefix(b"rId".as_slice())?;
    if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) {
        return None;
    }
    str::from_utf8(digits).ok()?.parse().ok()
}

/// A file to rewrite a part into: one handle, readable, mode 0600.
///
/// The same shape as the zip layer's own temporary (`specs/0008-zip-layer.md`,
/// the P2 rulings of 2026-09-17, item 4): the bytes are the caller's and the
/// temporary directory is shared.
fn temp_for_part() -> Result<(TempFile, File), Error> {
    let temp = TempFile::new()?;
    let mut options = OpenOptions::new();
    options.read(true).write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(&temp.0)?;
    Ok((temp, file))
}

/// Rewinds the file the part was written into and makes it a replacement.
///
/// The caller hands over the handle its [`BufWriter`] gave back, so the
/// buffer is already flushed.
fn written_part(temp: TempFile, mut file: File) -> Result<(TempFile, Replacement), Error> {
    // The handle is at the end of what was just written, and the replacement
    // reads from wherever it is: without this it would read nothing at all.
    file.seek(SeekFrom::Start(0))?;
    Ok((temp, replacement_from_reader(file)))
}

/// What [`rewrite_part`] asks about each element it copies: the element's
/// resolved local name and its start tag, `true` when that element's own
/// bytes are to be dropped.
///
/// A named type because the borrow and the two arguments together are more
/// than clippy's `type_complexity` allows written out at the call.
type Removes<'a> = &'a dyn Fn(&[u8], &BytesStart<'_>) -> Result<bool, Error>;

/// Rewrites one part into a temporary file: the elements `remove` names lose
/// their own bytes, `additions` go in immediately before the root's end tag,
/// and every other byte of the input is copied as it was read.
///
/// `namespace` is the namespace the removed and added elements live in, so an
/// element of the same local name in another namespace is left alone;
/// `remove` sees that resolved local name and the start tag. A removed
/// element's surrounding whitespace stays because it is another token's
/// [`Token::raw`], not part of the element's own. An element written as a
/// start tag rather than an empty one takes its children and its end tag with
/// it, since the element is what the caller named.
///
/// A root written as an empty element has no end tag, so its own bytes go out
/// with the `/` of its `/>` dropped, then the additions, then `</` and the
/// root's name as the input writes it (P3 ruling 8): the prolog and the
/// root's attributes are kept byte for byte, and only those two bytes change.
fn rewrite_part(
    archive: &mut Core,
    index: usize,
    remove: Removes<'_>,
    namespace: &[u8],
    additions: &[Vec<u8>],
) -> Result<(TempFile, Replacement), Error> {
    /// The root element's depth, so an `<Override>` or `<Relationship>` is 2.
    const ROOT_DEPTH: usize = 1;

    let (temp, file) = temp_for_part()?;
    let mut out = BufWriter::new(file);
    let mut stream = Stream::new(archive.read(index)?, index)?;
    // The mark is in no token's raw (`xml`'s module header), so a rewrite
    // that keeps the part's bytes has to write it back itself.
    if stream.bom() {
        out.write_all(&BYTE_ORDER_MARK)?;
    }
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut added = false;
    // The depth of a removed element written as a start tag, whose children
    // and end tag go with it.
    let mut dropping: Option<usize> = None;
    while let Some(token) = stream.next()? {
        let empty = matches!(token.event, Event::Empty(_));
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                depth += 1;
                // A start tag's own declarations bind for that tag, so the
                // scope takes them before the name is resolved.
                scope.start(start)?;
                let (element_ns, local) = scope.resolve_element(start.name())?;
                if depth == ROOT_DEPTH && empty {
                    let raw = token.raw;
                    let slash = raw
                        .iter()
                        .rposition(|byte| *byte == b'/')
                        .ok_or(Error::Xml(index))?;
                    out.write_all(&raw[..slash])?;
                    out.write_all(&raw[slash + 1..])?;
                    for addition in additions {
                        out.write_all(addition)?;
                    }
                    out.write_all(b"</")?;
                    out.write_all(start.name().into_inner().as_bytes())?;
                    out.write_all(b">")?;
                    added = true;
                } else if depth > ROOT_DEPTH
                    && dropping.is_none()
                    && element_ns == Some(namespace)
                    && remove(local, start)?
                {
                    if !empty {
                        dropping = Some(depth);
                    }
                } else if dropping.is_none() {
                    out.write_all(token.raw)?;
                }
                if empty {
                    scope.end();
                    depth = depth.saturating_sub(1);
                }
            }
            Event::End(_) => {
                if depth == ROOT_DEPTH && !added {
                    for addition in additions {
                        out.write_all(addition)?;
                    }
                    added = true;
                }
                let ends_removed = dropping == Some(depth);
                if dropping.is_none() {
                    out.write_all(token.raw)?;
                }
                if ends_removed {
                    dropping = None;
                }
                scope.end();
                depth = depth.saturating_sub(1);
            }
            _ => {
                if dropping.is_none() {
                    out.write_all(token.raw)?;
                }
            }
        }
    }
    // A part whose root never ends has nowhere to put an addition. The parser
    // refuses an unclosed root before this, so this is the belt on top of the
    // braces: an addition is never silently dropped.
    if !added && !additions.is_empty() {
        return Err(Error::Xml(index));
    }
    written_part(temp, out.into_inner().map_err(|e| Error::Io(e.into()))?)
}

/// The number of children each container the styles rewrite grows holds in
/// the part as read, and whether it has a `<numFmts>` at all.
struct Counts {
    num_fmts: u64,
    fonts: u64,
    fills: u64,
    borders: u64,
    cell_xfs: u64,
    has_num_fmts: bool,
}

/// What the styles rewrite appends, container by container.
struct StyleAdditions<'a> {
    /// The new number formats, in id order.
    codes: &'a [(u32, String)],
    fonts: &'a [(Font, Composed)],
    fills: &'a [(Fill, Composed)],
    borders: &'a [(Border, Composed)],
    xfs: &'a [NewXf],
    /// The styles part's `dxf` children and the differential styles to
    /// append, when there are any (the Cargo feature `rules`).
    #[cfg(feature = "rules")]
    dxfs: Option<(u64, &'a [Composed])>,
}

/// Rewrites the styles part with the recorded additions into a temporary
/// file.
///
/// `specs/0012-styles.md`, "Module `package`", over the five containers of
/// `specs/0017-fuller-styles.md`: every token's `raw` is copied except that
/// the `<numFmts>`, `<fonts>`, `<fills>`, `<borders>` and `<cellXfs>` start
/// tags among the root's children, found by name in whatever order the part
/// gives them, have their `count` values replaced by their children plus
/// their additions, every other byte of the tag kept; each container's new
/// elements (the `numFmt`s in id order, the rest in record order) go
/// immediately before its end tag, each taking the container's prefix; an
/// empty container that gains children is opened up as part 2 opens an empty
/// root; and a part with no `<numFmts>` that gains a code gets one, with the
/// root's prefix, immediately before the first of `fonts`, `fills`,
/// `borders`, `cellStyleXfs` and `cellXfs` among the root's children. Once
/// the stream ends, leaving no output, a part with no `<cellXfs>` is
/// refused, and then one without the `<fonts>`, `<fills>` or `<borders>` an
/// addition goes into, which [`Package::add_style`] has already refused
/// unless the part changed after it was read.
fn rewrite_styles(
    archive: &mut Core,
    index: usize,
    counts: &Counts,
    additions: &StyleAdditions<'_>,
) -> Result<(TempFile, Replacement), Error> {
    /// The root's children, before the first of which a new `<numFmts>`
    /// goes, in the schema's order.
    const AFTER_NUM_FMTS: [&[u8]; 5] =
        [b"fonts", b"fills", b"borders", b"cellStyleXfs", b"cellXfs"];
    /// The containers the rewrite grows, by the index `grown` and `seen` use.
    const GROWN: [&[u8]; 5] = [b"numFmts", b"fonts", b"fills", b"borders", b"cellXfs"];
    const NUM_FMTS: usize = 0;
    const CELL_XFS: usize = 4;

    let (temp, file) = temp_for_part()?;
    let mut out = BufWriter::new(file);
    let mut stream = Stream::new(archive.read(index)?, index)?;
    if stream.bom() {
        out.write_all(&BYTE_ORDER_MARK)?;
    }
    // Each container's new elements, with its prefix, and how many there are.
    let elements = |container: usize, prefix: &[u8]| -> Vec<u8> {
        match container {
            NUM_FMTS => additions
                .codes
                .iter()
                .flat_map(|(id, code)| num_fmt_element(prefix, *id, code))
                .collect(),
            1 => additions
                .fonts
                .iter()
                .flat_map(|(_, e)| e.element(prefix))
                .collect(),
            2 => additions
                .fills
                .iter()
                .flat_map(|(_, e)| e.element(prefix))
                .collect(),
            3 => additions
                .borders
                .iter()
                .flat_map(|(_, e)| e.element(prefix))
                .collect(),
            _ => additions
                .xfs
                .iter()
                .flat_map(|xf| xf.element(prefix))
                .collect(),
        }
    };
    let added = [
        additions.codes.len(),
        additions.fonts.len(),
        additions.fills.len(),
        additions.borders.len(),
        additions.xfs.len(),
    ];
    let had = [
        counts.num_fmts,
        counts.fonts,
        counts.fills,
        counts.borders,
        counts.cell_xfs,
    ];

    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut root_prefix: Vec<u8> = Vec::new();
    // Whether a `<numFmts>` still has to be created before the next of
    // `AFTER_NUM_FMTS`.
    let mut create_num_fmts = !counts.has_num_fmts && added[NUM_FMTS] > 0;
    let mut seen = [false; 5];
    // The open container among the root's children, with its prefix.
    let mut open: Option<(usize, Vec<u8>)> = None;
    #[cfg(feature = "rules")]
    let mut dxfs = DxfsPass {
        added: additions.dxfs,
        met: false,
        open: None,
    };
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                if depth == 1 {
                    root_prefix = prefix_of(start);
                }
                // The differential styles' `<dxfs>`: created before the first
                // of the children that follow it, or grown in place
                // (`specs/0020-validation-and-conditional-formatting.md`,
                // "Differential styles").
                #[cfg(feature = "rules")]
                if depth == 2 && ours {
                    if let Some(bytes) = dxfs.create_before(local, &root_prefix) {
                        out.write_all(&bytes)?;
                    }
                    if local == b"dxfs" && dxfs.added.is_some() {
                        out.write_all(&dxfs.open(token.raw, start, empty, index)?)?;
                        if empty {
                            scope.end();
                            depth -= 1;
                        }
                        continue;
                    }
                }
                if depth == 2 && ours && create_num_fmts && AFTER_NUM_FMTS.contains(&local) {
                    let p = &root_prefix;
                    out.write_all(&[b"<".as_slice(), p, b"numFmts count=\""].concat())?;
                    out.write_all(
                        (had[NUM_FMTS] + added[NUM_FMTS] as u64)
                            .to_string()
                            .as_bytes(),
                    )?;
                    out.write_all(b"\">")?;
                    out.write_all(&elements(NUM_FMTS, p))?;
                    out.write_all(&[b"</".as_slice(), p, b"numFmts>"].concat())?;
                    create_num_fmts = false;
                }
                let container = if depth == 2 && ours {
                    GROWN.iter().position(|name| *name == local)
                } else {
                    None
                };
                match container {
                    Some(container) => {
                        seen[container] = true;
                        let count = had[container] + added[container] as u64;
                        let tag = counted(token.raw, count, index)?;
                        let prefix = prefix_of(start);
                        if empty && added[container] > 0 {
                            out.write_all(&opened_up(&tag, index)?)?;
                            out.write_all(&elements(container, &prefix))?;
                            out.write_all(b"</")?;
                            out.write_all(start.name().into_inner().as_bytes())?;
                            out.write_all(b">")?;
                        } else {
                            out.write_all(&tag)?;
                            if !empty {
                                open = Some((container, prefix));
                            }
                        }
                    }
                    None => out.write_all(token.raw)?,
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                if depth == 2
                    && let Some((container, prefix)) = open.take()
                {
                    out.write_all(&elements(container, &prefix))?;
                }
                #[cfg(feature = "rules")]
                if depth == 2 {
                    out.write_all(&dxfs.close())?;
                }
                #[cfg(feature = "rules")]
                if depth == 1
                    && let Some(bytes) = dxfs.create_at_end(&root_prefix)
                {
                    out.write_all(&bytes)?;
                }
                out.write_all(token.raw)?;
                scope.end();
                depth -= 1;
            }
            _ => out.write_all(token.raw)?,
        }
    }
    if !seen[CELL_XFS] {
        return Err(NO_CELL_XFS);
    }
    if (1..CELL_XFS).any(|container| added[container] > 0 && !seen[container]) {
        return Err(NEEDS_COLLECTION);
    }
    written_part(temp, out.into_inner().map_err(|e| Error::Io(e.into()))?)
}

/// The `<dxfs>` part of a styles rewrite
/// (`specs/0020-validation-and-conditional-formatting.md`, "Differential
/// styles", and P2 ruling 3): the part's `dxf` children and the additions,
/// whether the part's `<dxfs>` has been met or created, and the open one's
/// prefix. The Cargo feature `rules`.
#[cfg(feature = "rules")]
struct DxfsPass<'a> {
    added: Option<(u64, &'a [Composed])>,
    met: bool,
    open: Option<Vec<u8>>,
}

#[cfg(feature = "rules")]
impl DxfsPass<'_> {
    /// The root's children before the first of which a new `<dxfs>` goes,
    /// in 18.8.39's order.
    const BEFORE: [&'static [u8]; 3] = [b"tableStyles", b"colors", b"extLst"];

    /// The new `<dxf>` elements with `prefix`.
    fn elements(&self, prefix: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        for dxf in self.added.map_or(&[][..], |(_, added)| added) {
            out.extend_from_slice(&[b"<".as_slice(), prefix, b"dxf>"].concat());
            out.extend_from_slice(&dxf.element(prefix));
            out.extend_from_slice(&[b"</".as_slice(), prefix, b"dxf>"].concat());
        }
        out
    }

    /// The `count` the container is written with.
    fn count(&self) -> u64 {
        self.added
            .map_or(0, |(children, added)| children + added.len() as u64)
    }

    /// A new `<dxfs>` before the root child `local`, when the part has none
    /// yet and `local` is one that follows it.
    fn create_before(&mut self, local: &[u8], prefix: &[u8]) -> Option<Vec<u8>> {
        if Self::BEFORE.contains(&local) {
            return self.create_at_end(prefix);
        }
        None
    }

    /// A new `<dxfs>` with the root's `prefix`, when the part still has none.
    fn create_at_end(&mut self, prefix: &[u8]) -> Option<Vec<u8>> {
        if self.added.is_none() || self.met {
            return None;
        }
        self.met = true;
        let mut out = [b"<".as_slice(), prefix, b"dxfs count=\""].concat();
        out.extend_from_slice(self.count().to_string().as_bytes());
        out.extend_from_slice(b"\">");
        out.extend_from_slice(&self.elements(prefix));
        out.extend_from_slice(&[b"</".as_slice(), prefix, b"dxfs>"].concat());
        Some(out)
    }

    /// The part's `<dxfs>` start tag: a second (or one after a created one)
    /// refused; `count` set; an empty one opened up and written whole.
    fn open(
        &mut self,
        raw: &[u8],
        start: &BytesStart<'_>,
        empty: bool,
        index: usize,
    ) -> Result<Vec<u8>, Error> {
        if self.met {
            return Err(TWO_DXFS);
        }
        self.met = true;
        let tag = counted(raw, self.count(), index)?;
        let prefix = prefix_of(start);
        if !empty {
            self.open = Some(prefix);
            return Ok(tag);
        }
        let mut out = opened_up(&tag, index)?;
        out.extend_from_slice(&self.elements(&prefix));
        out.extend_from_slice(&[b"</", start.name().into_inner().as_bytes(), b">"].concat());
        Ok(out)
    }

    /// At a root child's end tag: the additions, when it is the open
    /// `<dxfs>`.
    fn close(&mut self) -> Vec<u8> {
        match self.open.take() {
            Some(prefix) => self.elements(&prefix),
            None => Vec::new(),
        }
    }
}

/// The `dxf` children of a styles part's main-namespace `<dxfs>`, and how
/// many `<dxfs>` its root has, by one streaming pass.
#[cfg(feature = "rules")]
fn read_dxfs(input: impl Read, index: usize) -> Result<(u64, usize), Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut inside = false;
    let (mut children, mut containers) = (0u64, 0usize);
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                if depth == 2 {
                    inside = ours && local == b"dxfs";
                    containers += usize::from(inside);
                    if !inside && !empty {
                        skip_element(&mut stream, &mut scope)?;
                        scope.end();
                        depth -= 1;
                        continue;
                    }
                } else if depth == 3 && inside && ours && local == b"dxf" {
                    children += 1;
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
    }
    Ok((children, containers))
}

/// What the workbook rewrite does to the sheet list
/// (`specs/0016-new-workbook-and-sheets.md`, "The workbook rewrite").
struct SheetPlan {
    /// The projected list, in order.
    list: Vec<Projected>,
    /// How many `<sheet>` elements the file's `<sheets>` holds.
    file_count: usize,
    /// For each of the file's sheets, by its index there, its position in
    /// the projected list, `None` when it is removed.
    positions: Vec<Option<usize>>,
    /// Whether a file sheet's index changes, a move or a removal, which is
    /// when `<definedNames>` and `<bookViews>` are rewritten.
    reindexed: bool,
    /// The position of the first visible projected sheet (P4 ruling 1),
    /// where an `activeTab` naming a removed sheet goes.
    first_visible: usize,
}

impl SheetPlan {
    fn new(list: Vec<Projected>, file_count: usize) -> Self {
        let mut positions = vec![None; file_count];
        for (at, entry) in list.iter().enumerate() {
            if let Some(origin) = entry.origin
                && origin < file_count
            {
                positions[origin] = Some(at);
            }
        }
        let reindexed = positions
            .iter()
            .enumerate()
            .any(|(origin, at)| *at != Some(origin));
        let first_visible = list
            .iter()
            .position(|entry| is_visible(&entry.sheet))
            .unwrap_or(0);
        SheetPlan {
            list,
            file_count,
            positions,
            reindexed,
            first_visible,
        }
    }

    /// Whether the projected list holds a sheet this write adds.
    fn adds(&self) -> bool {
        self.list.iter().any(|entry| entry.origin.is_none())
    }

    /// Whether the projected list differs from the file's: a sheet added,
    /// moved, removed or renamed. A pending sheet added and removed again
    /// leaves none of these, and the workbook part is then not rewritten
    /// (P4 ruling 4).
    fn changes(&self) -> bool {
        self.reindexed || self.adds() || self.list.iter().any(|entry| entry.renamed)
    }

    /// A file index's projected index: its own position, the first visible
    /// sheet's when it is removed, and the value unchanged when it names no
    /// sheet of the file (P3 ruling 5).
    fn remapped(&self, index: usize) -> usize {
        match self.positions.get(index) {
            Some(Some(at)) => *at,
            Some(None) => self.first_visible,
            None => index,
        }
    }

    /// A present `firstSheet`: when a sheet was removed, decremented by the
    /// removed sheets before it and clamped to the projected list's last
    /// position; an add or a move leaves it as it is, even past the end (P3
    /// rulings 8 and 10).
    fn first_sheet(&self, value: usize) -> usize {
        if self.positions.iter().all(Option::is_some) {
            return value;
        }
        let removed = self
            .positions
            .iter()
            .take(value.min(self.file_count))
            .filter(|at| at.is_none())
            .count();
        (value - removed).min(self.list.len().saturating_sub(1))
    }
}

/// What one rewrite of the workbook part applies: `calcPr` when `calc` is
/// `Some(has_calc_pr)`, the sheet list when `plan` is, and the defined names
/// when `names` is.
struct BookEdits<'a> {
    calc: Option<bool>,
    plan: Option<&'a SheetPlan>,
    #[cfg(feature = "metadata")]
    names: Option<&'a NamePlan>,
    /// The workbook's protection edit, the last recorded: `Some(Some(..))`
    /// a protect with the hash and the two locks, `Some(None)` an
    /// unprotect; and whether the part lacks the element.
    #[cfg(feature = "metadata")]
    protection: Option<(Option<BookLock>, bool)>,
}

/// [`rewrite_workbook_part`] with no defined-name edit, the form the part 6
/// backstop test calls directly.
#[cfg(test)]
fn rewrite_workbook(
    archive: &mut Core,
    index: usize,
    calc: Option<bool>,
    plan: Option<&SheetPlan>,
) -> Result<(TempFile, Replacement), Error> {
    let edits = BookEdits {
        calc,
        plan,
        #[cfg(feature = "metadata")]
        names: None,
        #[cfg(feature = "metadata")]
        protection: None,
    };
    rewrite_workbook_part(archive, index, &edits)
}

/// Rewrites the workbook part into a temporary file: `calcPr` when `calc`
/// is `Some`, the `<sheets>`, `<definedNames>` and `<bookViews>` regions
/// when `plan` is (`specs/0013-formulas.md`, "Module `package`";
/// `specs/0016-new-workbook-and-sheets.md`, "The workbook rewrite", and P3
/// rulings 4, 5, 6 and 8), and the defined names when `names` is
/// (`specs/0019-names-links-properties-protection.md`, "Defined names"). One
/// pass; every token's `raw` is copied, the mark included, except:
///
/// - `calcPr` (`calc` is `Some(has_calc_pr)`): the start tag of the one
///   `<calcPr>` among the root's children keeps every byte but the value of
///   an unprefixed `fullCalcOnLoad` (now `1`, in its own quotes) or gains
///   ` fullCalcOnLoad="1"` right after its last attribute. With no
///   `<calcPr>`, `<calcPr fullCalcOnLoad="1"/>` is created as "Creating an
///   element in the workbook" says, below. A second `<calcPr>` is refused,
///   leaving no output.
/// - `<sheets>`: its children become the projected list, back to back, an
///   existing `<sheet>` element copied byte for byte with a renamed sheet's
///   `name` value replaced between its own quotes, a new one written as
///   `<sheet name=".." sheetId=".." r:id=".."/>` with the prefix the root
///   binds to the relationships namespace. Anything in `<sheets>` other than
///   `<sheet>` elements and whitespace is refused.
/// - `<definedNames>`, when a sheet moved or was removed: a `<definedName>`'s
///   `localSheetId` is remapped between its own quotes, or the element
///   dropped when its sheet was removed. With `names`, a `<definedName>`
///   recorded for removal is skipped whole, the additions go back to back
///   before the container's end tag, a container left empty is dropped
///   whole, one that is absent is created, and a second is refused.
/// - `<bookViews>`, when a sheet moved or was removed: a `<workbookView>`'s
///   `activeTab` is remapped, an absent one appended when it maps away from
///   0, and a present `firstSheet` follows P3 ruling 8.
///
/// Creating an element in the workbook: an absent element the write needs
/// goes, with the root's prefix, immediately after the end of the last root
/// child of the main namespace that [`WORKBOOK_CHILDREN`] puts before it, or
/// after the root start tag when there is none. The tokens that follow such
/// an end at the root's level are held back until the next child of the
/// root in the main namespace shows, by its place in the table, whether the
/// element goes before it; a main-namespace name outside the table counts as
/// after every name in it. Two created elements go in the table's order.
fn rewrite_workbook_part(
    archive: &mut Core,
    index: usize,
    edits: &BookEdits<'_>,
) -> Result<(TempFile, Replacement), Error> {
    let (calc, plan) = (edits.calc, edits.plan);
    #[cfg(feature = "metadata")]
    let names = edits.names;
    let (temp, file) = temp_for_part()?;
    let mut out = BufWriter::new(file);
    let mut stream = Stream::new(archive.read(index)?, index)?;
    if stream.bom() {
        out.write_all(&BYTE_ORDER_MARK)?;
    }
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut root_prefix: Vec<u8> = Vec::new();
    let mut rels_prefix: Option<Vec<u8>> = None;
    let mut calc_prs = 0usize;
    // The elements still to be created, in the table's order, and the bytes
    // held back since the last point they could go.
    let mut pending: Vec<&'static [u8]> = Vec::new();
    #[cfg(feature = "metadata")]
    if let Some((Some(_), true)) = edits.protection {
        pending.push(b"workbookProtection");
    }
    #[cfg(feature = "metadata")]
    let mut protections = 0usize;
    #[cfg(feature = "metadata")]
    if names.is_some_and(|names| names.create) {
        pending.push(b"definedNames");
    }
    if calc == Some(false) {
        pending.push(b"calcPr");
    }
    let mut held: Vec<u8> = Vec::new();
    // The root child being walked when it is one of the three regions.
    let mut region: Option<&'static [u8]> = None;
    // Inside `<sheets>`: the file's `<sheet>` elements' bytes, the one being
    // read, and the prefix and end tag of `<sheets>` itself.
    let mut file_sheets: Vec<Vec<u8>> = Vec::new();
    let mut current: Option<Vec<u8>> = None;
    let mut sheets_prefix: Vec<u8> = Vec::new();
    let mut sheets_seen = false;
    // The `<definedName>` elements met so far, the ordinal a removal names,
    // and the `<definedNames>` containers.
    #[cfg(feature = "metadata")]
    let mut name_ordinal = 0usize;
    #[cfg(feature = "metadata")]
    let mut name_containers = 0usize;
    while let Some(token) = stream.next()? {
        // A sheet element being read is kept whole, whatever it holds.
        if let Some(bytes) = current.as_mut() {
            bytes.extend_from_slice(token.raw);
            match token.event {
                Event::Start(ref start) => {
                    depth += 1;
                    scope.start(start)?;
                }
                Event::Empty(ref start) => {
                    scope.start(start)?;
                    scope.end();
                }
                Event::End(_) => {
                    scope.end();
                    depth -= 1;
                    if depth == 2 {
                        file_sheets.push(current.take().unwrap_or_default());
                    }
                }
                _ => {}
            }
            continue;
        }
        let inserting = !pending.is_empty();
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                if depth == 1 {
                    root_prefix = prefix_of(start);
                    rels_prefix = relationships_prefix(start, index)?;
                    if let Some(plan) = plan
                        && plan.adds()
                    {
                        if !root_prefix.is_empty() {
                            return Err(SHEET_ADDED_TO_PREFIXED);
                        }
                        if rels_prefix.is_none() {
                            return Err(NO_RELATIONSHIPS_PREFIX);
                        }
                    }
                }
                if depth == 3 && region == Some(b"sheets") {
                    if !(ours && local == b"sheet") {
                        return Err(SHEETS_NOT_SHEETS);
                    }
                    if empty {
                        file_sheets.push(token.raw.to_vec());
                        scope.end();
                        depth -= 1;
                    } else {
                        current = Some(token.raw.to_vec());
                    }
                    continue;
                }
                // A defined name recorded for removal goes, its content with
                // it; the ordinal counts every one the file has.
                #[cfg(feature = "metadata")]
                if depth == 3 && region == Some(b"definedNames") && ours && local == b"definedName"
                {
                    let ordinal = name_ordinal;
                    name_ordinal += 1;
                    if names.is_some_and(|names| names.removed.contains(&ordinal)) {
                        if !empty {
                            skip_element(&mut stream, &mut scope)?;
                        }
                        scope.end();
                        depth -= 1;
                        continue;
                    }
                }
                // The bytes this token writes: its own, or its start tag
                // with an attribute remapped.
                let raw: std::borrow::Cow<'_, [u8]> = match (plan, region, depth) {
                    (Some(plan), Some(b"definedNames"), 3)
                        if plan.reindexed && ours && local == b"definedName" =>
                    {
                        match defined_name_tag(token.raw, plan, index)? {
                            Some(tag) => tag.into(),
                            None => {
                                // Its sheet was removed: the element goes, its
                                // content with it.
                                if !empty {
                                    skip_element(&mut stream, &mut scope)?;
                                }
                                scope.end();
                                depth -= 1;
                                continue;
                            }
                        }
                    }
                    (Some(plan), Some(b"bookViews"), 3)
                        if plan.reindexed && ours && local == b"workbookView" =>
                    {
                        workbook_view_tag(token.raw, plan, index)?.into()
                    }
                    _ => token.raw.into(),
                };
                if depth == 2 && ours {
                    region = regions(plan.is_some(), names_edited(edits))
                        .iter()
                        .copied()
                        .find(|name| *name == local);
                }
                if depth == 2 && ours && inserting {
                    // The elements that go before this child are written,
                    // then what was held back since the last child.
                    let at = book_position(local);
                    for which in pending.iter().filter(|which| book_position(which) < at) {
                        out.write_all(&created_book_element(which, &root_prefix, edits))?;
                    }
                    pending.retain(|which| book_position(which) >= at);
                    out.write_all(&held)?;
                    held.clear();
                }
                // The workbook's `<workbookProtection>`: replaced whole or
                // dropped, a second refused
                // (`specs/0019-names-links-properties-protection.md`,
                // "Protection").
                #[cfg(feature = "metadata")]
                if depth == 2
                    && ours
                    && local == b"workbookProtection"
                    && let Some((protection, _)) = edits.protection
                {
                    protections += 1;
                    if protections > 1 {
                        return Err(TWO_BOOK_ELEMENTS);
                    }
                    let bytes = match protection {
                        Some(protection) => book_protection_element(protection, &prefix_of(start)),
                        None => Vec::new(),
                    };
                    if !empty {
                        skip_element(&mut stream, &mut scope)?;
                    }
                    out.write_all(&bytes)?;
                    scope.end();
                    depth -= 1;
                    continue;
                }
                #[cfg(feature = "metadata")]
                if depth == 2 && region == Some(b"definedNames") && names.is_some() {
                    name_containers += 1;
                    if name_containers > 1 {
                        return Err(TWO_BOOK_ELEMENTS);
                    }
                    if names.is_some_and(|names| names.drop) {
                        // Left empty: the container goes whole.
                        if !empty {
                            skip_element(&mut stream, &mut scope)?;
                        }
                        region = None;
                        scope.end();
                        depth -= 1;
                        continue;
                    }
                    if let Some(names) = names
                        && empty
                        && !names.added.is_empty()
                    {
                        out.write_all(&opened_up(&raw, index)?)?;
                        out.write_all(&name_elements(&names.added, &root_prefix))?;
                        out.write_all(b"</")?;
                        out.write_all(start.name().into_inner().as_bytes())?;
                        out.write_all(b">")?;
                        region = None;
                        scope.end();
                        depth -= 1;
                        continue;
                    }
                }
                let inserting = !pending.is_empty();
                if depth == 1 {
                    if inserting && empty {
                        // A root with no children at all: opened up as part 2
                        // opens an empty root, with the new elements inside.
                        out.write_all(&opened_up(&raw, index)?)?;
                        for which in &pending {
                            out.write_all(&created_book_element(which, &root_prefix, edits))?;
                        }
                        pending.clear();
                        out.write_all(b"</")?;
                        out.write_all(start.name().into_inner().as_bytes())?;
                        out.write_all(b">")?;
                    } else {
                        out.write_all(&raw)?;
                    }
                } else if depth == 2 && ours && local == b"calcPr" && calc.is_some() {
                    calc_prs += 1;
                    if calc_prs > 1 {
                        return Err(TWO_CALC_PRS);
                    }
                    out.write_all(&held)?;
                    held.clear();
                    out.write_all(&full_calc_tag(&raw, index)?)?;
                } else if depth >= 2 && inserting && !held.is_empty() {
                    held.extend_from_slice(&raw);
                } else if depth == 2 && inserting && !ours {
                    // Another namespace's child of the root is held with what
                    // follows it, since a child the table puts before a
                    // pending element may come after.
                    held.extend_from_slice(&raw);
                } else if depth == 2 {
                    write_region_start(&mut out, &raw, region, empty, index)?;
                } else {
                    out.write_all(&raw)?;
                }
                if depth == 2 && region == Some(b"sheets") {
                    sheets_seen = true;
                    sheets_prefix = prefix_of(start);
                    if empty {
                        if let Some(plan) = plan {
                            write_sheets(
                                &mut out,
                                plan,
                                &file_sheets,
                                &sheets_prefix,
                                rels_prefix.as_deref(),
                                index,
                            )?;
                        }
                        out.write_all(b"</")?;
                        out.write_all(start.name().into_inner().as_bytes())?;
                        out.write_all(b">")?;
                    }
                }
                if empty {
                    if depth == 2 {
                        region = None;
                    }
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                #[cfg(feature = "metadata")]
                if depth == 2
                    && region == Some(b"definedNames")
                    && let Some(names) = names
                {
                    out.write_all(&name_elements(&names.added, &root_prefix))?;
                }
                if depth == 2 && region == Some(b"sheets") {
                    if let Some(plan) = plan {
                        write_sheets(
                            &mut out,
                            plan,
                            &file_sheets,
                            &sheets_prefix,
                            rels_prefix.as_deref(),
                            index,
                        )?;
                    }
                    out.write_all(token.raw)?;
                } else if depth == 1 && inserting {
                    for which in &pending {
                        out.write_all(&created_book_element(which, &root_prefix, edits))?;
                    }
                    pending.clear();
                    out.write_all(&held)?;
                    held.clear();
                    out.write_all(token.raw)?;
                } else if inserting && !held.is_empty() {
                    held.extend_from_slice(token.raw);
                } else {
                    out.write_all(token.raw)?;
                }
                if depth == 2 {
                    region = None;
                }
                scope.end();
                depth -= 1;
            }
            _ => {
                if depth == 2 && region == Some(b"sheets") {
                    // Between `<sheet>` elements only whitespace may stand,
                    // and it is not reproduced.
                    let text = matches!(token.event, Event::Text(_));
                    if !text || !token.raw.iter().all(u8::is_ascii_whitespace) {
                        return Err(SHEETS_NOT_SHEETS);
                    }
                } else if inserting && (depth == 1 || !held.is_empty()) {
                    // Between the root's children, or inside a held child.
                    held.extend_from_slice(token.raw);
                } else {
                    out.write_all(token.raw)?;
                }
            }
        }
    }
    if let Some(plan) = plan
        && plan.adds()
        && !sheets_seen
    {
        return Err(SHEET_EDIT_STALE);
    }
    written_part(temp, out.into_inner().map_err(|e| Error::Io(e.into()))?)
}

/// The workbook root children whose content the rewrite walks: the three
/// regions of the sheet plan, `<definedNames>` alone for a name edit.
fn regions(plan: bool, names: bool) -> &'static [&'static [u8]] {
    const REGIONS: [&[u8]; 3] = [b"sheets", b"definedNames", b"bookViews"];
    if plan {
        &REGIONS
    } else if names {
        &REGIONS[1..2]
    } else {
        &[]
    }
}

/// Whether the rewrite carries a defined-name edit; never without the
/// feature `metadata`.
fn names_edited(edits: &BookEdits<'_>) -> bool {
    #[cfg(feature = "metadata")]
    return edits.names.is_some();
    #[cfg(not(feature = "metadata"))]
    {
        let _ = edits;
        false
    }
}

/// The bytes of a workbook root child the rewrite creates, with the root's
/// prefix: `<calcPr fullCalcOnLoad="1"/>`, or `<definedNames>` holding the
/// added names.
fn created_book_element(which: &[u8], prefix: &[u8], edits: &BookEdits<'_>) -> Vec<u8> {
    #[cfg(feature = "metadata")]
    if which == b"workbookProtection"
        && let Some((Some(protection), _)) = edits.protection
    {
        return book_protection_element(protection, prefix);
    }
    #[cfg(feature = "metadata")]
    if which == b"definedNames"
        && let Some(names) = edits.names
    {
        return [
            b"<".as_slice(),
            prefix,
            b"definedNames>",
            &name_elements(&names.added, prefix),
            b"</",
            prefix,
            b"definedNames>",
        ]
        .concat();
    }
    let _ = (which, edits);
    [b"<".as_slice(), prefix, b"calcPr fullCalcOnLoad=\"1\"/>"].concat()
}

/// A `<workbookProtection>` element with `prefix`: `workbookPassword` when a
/// hash is given, `lockStructure="1"` and `lockWindows="1"` when locked, and
/// written with no attribute when none applies, as Excel writes it
/// (`specs/0019-names-links-properties-protection.md`, "Protection").
#[cfg(feature = "metadata")]
fn book_protection_element((hash, structure, windows): BookLock, prefix: &[u8]) -> Vec<u8> {
    let mut out = [b"<".as_slice(), prefix, b"workbookProtection"].concat();
    if let Some(hash) = hash {
        out.extend_from_slice(format!(" workbookPassword=\"{hash:04X}\"").as_bytes());
    }
    if structure {
        out.extend_from_slice(b" lockStructure=\"1\"");
    }
    if windows {
        out.extend_from_slice(b" lockWindows=\"1\"");
    }
    out.extend_from_slice(b"/>");
    out
}

/// The added `<definedName>` elements, back to back, with `prefix`: `name`
/// escaped as an attribute, `localSheetId` when the name has a scope, and
/// the text escaped as a cell's text is (`specs/0011-editing-cells.md`).
#[cfg(feature = "metadata")]
fn name_elements(added: &[(String, String, Option<u32>)], prefix: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for (name, text, sheet) in added {
        out.extend_from_slice(b"<");
        out.extend_from_slice(prefix);
        out.extend_from_slice(b"definedName name=\"");
        out.extend_from_slice(&attribute_escaped(name));
        out.push(b'"');
        if let Some(sheet) = sheet {
            out.extend_from_slice(b" localSheetId=\"");
            out.extend_from_slice(sheet.to_string().as_bytes());
            out.push(b'"');
        }
        out.push(b'>');
        out.extend_from_slice(escape(text.as_str()).replace('\r', "&#13;").as_bytes());
        out.extend_from_slice(b"</");
        out.extend_from_slice(prefix);
        out.extend_from_slice(b"definedName>");
    }
    out
}

/// What the workbook rewrite does to the defined names
/// (`specs/0019-names-links-properties-protection.md`, "Defined names").
#[cfg(feature = "metadata")]
struct NamePlan {
    /// The ordinals, in document order among the root's `<definedNames>`
    /// children, of the file's names recorded for removal.
    removed: Vec<usize>,
    /// The additions in record order: name, text and the scope's index in
    /// the output's sheet list.
    added: Vec<(String, String, Option<u32>)>,
    /// Whether the file's `<definedNames>` is left empty and dropped.
    drop: bool,
    /// Whether the workbook has no `<definedNames>` and one is created.
    create: bool,
}

/// Writes a root child's start tag. `<sheets/>` with sheets to write is
/// opened up, and the caller writes its children and end tag.
fn write_region_start(
    out: &mut impl Write,
    raw: &[u8],
    region: Option<&'static [u8]>,
    empty: bool,
    index: usize,
) -> Result<(), Error> {
    if empty && region == Some(b"sheets") {
        out.write_all(&opened_up(raw, index)?)?;
    } else {
        out.write_all(raw)?;
    }
    Ok(())
}

/// Writes the projected list as `<sheets>`'s children, back to back.
fn write_sheets(
    out: &mut impl Write,
    plan: &SheetPlan,
    file_sheets: &[Vec<u8>],
    prefix: &[u8],
    rels_prefix: Option<&[u8]>,
    index: usize,
) -> Result<(), Error> {
    if file_sheets.len() != plan.file_count {
        return Err(SHEET_EDIT_STALE);
    }
    for entry in &plan.list {
        match entry.origin {
            Some(origin) => {
                let bytes = file_sheets.get(origin).ok_or(SHEET_EDIT_STALE)?;
                if entry.renamed {
                    out.write_all(&with_attribute(
                        bytes,
                        b"name",
                        &attribute_escaped(&entry.sheet.name),
                        index,
                    )?)?;
                } else {
                    out.write_all(bytes)?;
                }
            }
            None => {
                let rels = rels_prefix.ok_or(NO_RELATIONSHIPS_PREFIX)?;
                out.write_all(b"<")?;
                out.write_all(prefix)?;
                out.write_all(b"sheet name=\"")?;
                out.write_all(&attribute_escaped(&entry.sheet.name))?;
                out.write_all(b"\" sheetId=\"")?;
                out.write_all(entry.sheet.sheet_id.to_string().as_bytes())?;
                out.write_all(b"\" ")?;
                out.write_all(rels)?;
                out.write_all(b":id=\"")?;
                out.write_all(&entry.sheet.rel_id)?;
                out.write_all(b"\"/>")?;
            }
        }
    }
    Ok(())
}

/// The prefix the root start tag binds to the relationships namespace, in
/// either form, the first such binding when there are two.
fn relationships_prefix(start: &BytesStart<'_>, index: usize) -> Result<Option<Vec<u8>>, Error> {
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        let key = attribute.key.as_ref().as_bytes();
        if let Some(prefix) = key.strip_prefix(b"xmlns:")
            && RELATIONSHIPS_ATTRIBUTE.contains(&attribute.value.as_bytes())
        {
            return Ok(Some(prefix.to_vec()));
        }
    }
    Ok(None)
}

/// A sheet name escaped for an attribute: `&`, `<`, `>`, the apostrophe and
/// the quote as entities, and tab, line feed and carriage return as
/// character references, so a reader's normalisation gives the name back
/// (P3 ruling 4).
fn attribute_escaped(name: &str) -> Vec<u8> {
    escape(name)
        .replace('\t', "&#9;")
        .replace('\n', "&#10;")
        .replace('\r', "&#13;")
        .into_bytes()
}

/// A start tag with the unprefixed attribute `name` set to `value` (already
/// escaped): replaced between its own quotes when present, else appended
/// after the last attribute, so every other byte of the tag is kept.
fn with_attribute(raw: &[u8], name: &[u8], value: &[u8], index: usize) -> Result<Vec<u8>, Error> {
    let tag = scan_start_tag(raw).ok_or(Error::Xml(index))?;
    if let Some(present) = tag
        .attributes
        .iter()
        .find(|attribute| &raw[attribute.name.clone()] == name)
    {
        let mut out = raw[..present.value.start].to_vec();
        out.extend_from_slice(value);
        out.extend_from_slice(&raw[present.value.end..]);
        return Ok(out);
    }
    let at = match tag.attributes.last() {
        // The closing quote follows the value.
        Some(last) => last.value.end + 1,
        None => tag.name_end,
    };
    let mut out = raw[..at].to_vec();
    out.push(b' ');
    out.extend_from_slice(name);
    out.extend_from_slice(b"=\"");
    out.extend_from_slice(value);
    out.push(b'"');
    out.extend_from_slice(&raw[at..]);
    Ok(out)
}

/// The value of the unprefixed attribute `name` in a start tag, read as a
/// decimal `u32`: `Ok(None)` when absent, and "a sheet edit no longer applies
/// at the write" when it is not a decimal `u32` (P3 rulings 5 and 10).
fn index_attribute(raw: &[u8], name: &[u8], index: usize) -> Result<Option<usize>, Error> {
    let tag = scan_start_tag(raw).ok_or(Error::Xml(index))?;
    match tag
        .attributes
        .iter()
        .find(|attribute| &raw[attribute.name.clone()] == name)
    {
        None => Ok(None),
        Some(present) => {
            let value = &raw[present.value.clone()];
            if value.is_empty() || !value.iter().all(u8::is_ascii_digit) {
                return Err(SHEET_EDIT_STALE);
            }
            let number = str::from_utf8(value)
                .ok()
                .and_then(|text| text.parse::<u32>().ok())
                .ok_or(SHEET_EDIT_STALE)?;
            Ok(Some(number as usize))
        }
    }
}

/// A `<definedName>` start tag with its `localSheetId` remapped, the tag as
/// it is without one, or `None` when the sheet it names was removed.
fn defined_name_tag(raw: &[u8], plan: &SheetPlan, index: usize) -> Result<Option<Vec<u8>>, Error> {
    match index_attribute(raw, b"localSheetId", index)? {
        None => Ok(Some(raw.to_vec())),
        Some(local) if local < plan.file_count && plan.positions[local].is_none() => Ok(None),
        Some(local) => {
            let to = plan.remapped(local);
            if to == local {
                Ok(Some(raw.to_vec()))
            } else {
                Ok(Some(with_attribute(
                    raw,
                    b"localSheetId",
                    to.to_string().as_bytes(),
                    index,
                )?))
            }
        }
    }
}

/// A `<workbookView>` start tag with `activeTab` remapped (absent reads 0,
/// and stays absent when it maps to 0) and a present `firstSheet` as P3
/// ruling 8 gives it.
fn workbook_view_tag(raw: &[u8], plan: &SheetPlan, index: usize) -> Result<Vec<u8>, Error> {
    let mut tag = raw.to_vec();
    let active = index_attribute(&tag, b"activeTab", index)?;
    let to = plan.remapped(active.unwrap_or(0));
    if Some(to) != active && !(active.is_none() && to == 0) {
        tag = with_attribute(&tag, b"activeTab", to.to_string().as_bytes(), index)?;
    }
    if let Some(first) = index_attribute(&tag, b"firstSheet", index)? {
        let moved = plan.first_sheet(first);
        if moved != first {
            tag = with_attribute(&tag, b"firstSheet", moved.to_string().as_bytes(), index)?;
        }
    }
    Ok(tag)
}

/// Reads past the rest of an element whose start tag was just read, its end
/// tag included, keeping the scope in step.
fn skip_element<R: Read>(stream: &mut Stream<R>, scope: &mut Scope) -> Result<(), Error> {
    let mut depth = 1usize;
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) => {
                depth += 1;
                scope.start(start)?;
            }
            Event::End(_) => {
                depth -= 1;
                if depth == 0 {
                    return Ok(());
                }
                scope.end();
            }
            _ => {}
        }
    }
    Ok(())
}

/// The root children of a workbook in the order ECMA-376 Part 1, 18.2.27
/// gives them: an element the workbook rewrite creates goes immediately
/// after the last root child of the main namespace that comes before it
/// here (`specs/0019-names-links-properties-protection.md`, "Creating an
/// element in the workbook"). `calcPr`'s place is after the first nine,
/// which is where part 6 put it (`specs/0013-formulas.md`, P1 ruling 1).
const WORKBOOK_CHILDREN: [&[u8]; 19] = [
    b"fileVersion",
    b"fileSharing",
    b"workbookPr",
    b"workbookProtection",
    b"bookViews",
    b"sheets",
    b"functionGroups",
    b"externalReferences",
    b"definedNames",
    b"calcPr",
    b"oleSize",
    b"customWorkbookViews",
    b"pivotCaches",
    b"smartTagPr",
    b"smartTagTypes",
    b"webPublishing",
    b"fileRecoveryPr",
    b"webPublishObjects",
    b"extLst",
];

/// A workbook root child's place in [`WORKBOOK_CHILDREN`]. A name outside
/// the table counts as after every name in it, so a created element goes
/// before it, as part 6's `calcPr` went before any main-namespace child that
/// was not one of the nine before it.
fn book_position(local: &[u8]) -> usize {
    WORKBOOK_CHILDREN
        .iter()
        .position(|name| *name == local)
        .unwrap_or(WORKBOOK_CHILDREN.len())
}

/// A `<calcPr>` start tag's bytes with `fullCalcOnLoad` set to `1`: an
/// unprefixed one present has its value replaced between its own quotes,
/// else ` fullCalcOnLoad="1"` goes right after the last attribute's closing
/// quote, or after the name when there is none, so every other byte of the
/// tag is kept (`specs/0013-formulas.md`, P1 ruling 5).
fn full_calc_tag(raw: &[u8], index: usize) -> Result<Vec<u8>, Error> {
    let tag = scan_start_tag(raw).ok_or(Error::Xml(index))?;
    if let Some(present) = tag
        .attributes
        .iter()
        .find(|attribute| &raw[attribute.name.clone()] == b"fullCalcOnLoad")
    {
        let mut out = raw[..present.value.start].to_vec();
        out.push(b'1');
        out.extend_from_slice(&raw[present.value.end..]);
        return Ok(out);
    }
    let at = match tag.attributes.last() {
        // The closing quote follows the value.
        Some(last) => last.value.end + 1,
        None => tag.name_end,
    };
    let mut out = raw[..at].to_vec();
    out.extend_from_slice(b" fullCalcOnLoad=\"1\"");
    out.extend_from_slice(&raw[at..]);
    Ok(out)
}

/// One worksheet's cell edits and row formats, sorted, and how far the
/// rewrite has come through each.
struct Pending<'e> {
    edits: &'e [(CellRef, CellEdit)],
    next: usize,
    /// The row formats by row (`specs/0018-rows-and-columns.md`).
    rows: &'e [(u32, LineFormat)],
    next_row: usize,
    /// The entry's index, for an error.
    index: usize,
}

impl Pending<'_> {
    fn peek(&self) -> Option<CellRef> {
        self.edits.get(self.next).map(|(reference, _)| *reference)
    }

    /// Whether any edit still pending writes a cell where there is none, in
    /// `row` or, with `None`, anywhere: a set, or a style `Some(i)`.
    fn sets(&self, row: Option<u32>) -> bool {
        self.edits[self.next..].iter().any(|(reference, edit)| {
            edit.writes_when_absent() && row.is_none_or(|r| reference.row == r)
        })
    }

    /// Whether anything still pending writes a row where there is none: a
    /// cell a set or a style writes, or a row format that writes an
    /// attribute.
    fn opens_rows(&self) -> bool {
        self.sets(None)
            || self.rows[self.next_row..]
                .iter()
                .any(|(_, format)| format.writes_when_absent())
    }

    /// The format recorded for `row`, taken, when the next pending one is
    /// for it.
    fn take_row_format(&mut self, row: u32) -> Option<LineFormat> {
        match self.rows.get(self.next_row) {
            Some((at, format)) if *at == row => {
                self.next_row += 1;
                Some(*format)
            }
            _ => None,
        }
    }

    /// Writes the cells pending in `row` before column `before` (all of the
    /// row's with `None`), none of which exists: a set as its value's element
    /// with the edit's style when there is one, a style `Some(i)` alone or
    /// with a clear as `<c r="A1" s="i"/>`; a clear alone and a style `None`
    /// alone write nothing. Only a set is refused inside an array's range: a
    /// style edit never touches a formula (`specs/0012-styles.md`).
    fn write_row(
        &mut self,
        out: &mut impl Write,
        row: u32,
        before: Option<u32>,
        arrays: &[Area],
        prefix: &[u8],
    ) -> Result<(), Error> {
        while let Some((reference, edit)) = self.edits.get(self.next) {
            if reference.row != row || before.is_some_and(|col| reference.col >= col) {
                break;
            }
            let style = edit.style_bytes(None);
            match &edit.value {
                Some(Some(value)) => {
                    if arrays.iter().any(|area| area.holds(*reference)) {
                        return Err(IN_SHARED_OR_ARRAY);
                    }
                    out.write_all(&value.element(*reference, style.as_deref(), prefix))?;
                }
                _ => out.write_all(&replaced_cell(*reference, style.as_deref(), None, prefix))?,
            }
            self.next += 1;
        }
        Ok(())
    }

    /// Writes a new `<row>` for every pending row before `row` (every row
    /// left with `None`) that sets a cell or whose format writes an
    /// attribute; one with clears alone, or a format that only removes,
    /// writes nothing. A row with a format and no cell is written empty.
    /// `prefix` is the `<sheetData>`'s, which the rows take.
    fn write_rows(
        &mut self,
        out: &mut impl Write,
        row: Option<u32>,
        arrays: &[Area],
        prefix: &[u8],
    ) -> Result<(), Error> {
        loop {
            let cell_row = self.peek().map(|reference| reference.row);
            let format_row = self.rows.get(self.next_row).map(|(at, _)| *at);
            let next = match (cell_row, format_row) {
                (Some(a), Some(b)) => a.min(b),
                (Some(a), None) => a,
                (None, Some(b)) => b,
                (None, None) => break,
            };
            if row.is_some_and(|r| next >= r) {
                break;
            }
            let format = self.take_row_format(next);
            let sets = self.sets(Some(next));
            if sets || format.is_some_and(|format| format.writes_when_absent()) {
                let close: &[u8] = if sets { b">" } else { b"/>" };
                let mut tag = [b"<".as_slice(), prefix, b"row r=\""].concat();
                tag.extend_from_slice(next.to_string().as_bytes());
                tag.push(b'"');
                tag.extend_from_slice(close);
                if let Some(format) = format {
                    tag = line_formatted(&tag, &format, ROW_SIZE, self.index)?;
                }
                out.write_all(&tag)?;
                if sets {
                    self.write_row(out, next, None, arrays, prefix)?;
                    out.write_all(&[b"</".as_slice(), prefix, b"row>"].concat())?;
                }
            } else {
                self.write_row(out, next, None, arrays, prefix)?;
            }
        }
        Ok(())
    }
}

/// The size attribute and its custom flag of a row, and of a column.
const ROW_SIZE: [&[u8]; 2] = [b"ht", b"customHeight"];
const COLUMN_SIZE: [&[u8]; 2] = [b"width", b"customWidth"];

/// A `<row>` or `<col>` start tag with a format applied, every other byte
/// kept: a size set writes `size` and `custom="1"` (replaced in place or
/// appended), a size removed drops both, and the hidden flag is set as
/// `hidden="1"` or dropped (`specs/0018-rows-and-columns.md`, "Module
/// `package`: the calls").
fn line_formatted(
    raw: &[u8],
    format: &LineFormat,
    names: [&[u8]; 2],
    index: usize,
) -> Result<Vec<u8>, Error> {
    let mut tag = raw.to_vec();
    match format.size {
        Some(Some(size)) => {
            tag = with_attribute(&tag, names[0], number_text(size).as_bytes(), index)?;
            tag = with_attribute(&tag, names[1], b"1", index)?;
        }
        Some(None) => {
            tag = without_attribute(&tag, names[0], index)?;
            tag = without_attribute(&tag, names[1], index)?;
        }
        None => {}
    }
    match format.hidden {
        Some(true) => tag = with_attribute(&tag, b"hidden", b"1", index)?,
        Some(false) => tag = without_attribute(&tag, b"hidden", index)?,
        None => {}
    }
    Ok(tag)
}

/// A start tag without the unprefixed attribute `name` and the whitespace
/// before it, every other byte kept; the tag as it is when `name` is absent.
fn without_attribute(raw: &[u8], name: &[u8], index: usize) -> Result<Vec<u8>, Error> {
    let tag = scan_start_tag(raw).ok_or(Error::Xml(index))?;
    let Some(present) = tag
        .attributes
        .iter()
        .find(|attribute| &raw[attribute.name.clone()] == name)
    else {
        return Ok(raw.to_vec());
    };
    let mut from = present.name.start;
    while from > 0 && matches!(raw[from - 1], b' ' | b'\t' | b'\n' | b'\r') {
        from -= 1;
    }
    let mut out = raw[..from].to_vec();
    // The closing quote follows the value.
    out.extend_from_slice(&raw[present.value.end + 1..]);
    Ok(out)
}

/// The value of the unprefixed attribute `name` in a start tag, as written.
fn attribute_value<'r>(
    raw: &'r [u8],
    name: &[u8],
    index: usize,
) -> Result<Option<&'r [u8]>, Error> {
    let tag = scan_start_tag(raw).ok_or(Error::Xml(index))?;
    Ok(tag
        .attributes
        .iter()
        .find(|attribute| &raw[attribute.name.clone()] == name)
        .map(|attribute| &raw[attribute.value.clone()]))
}

/// What the rewrite does with the `<c>` it is inside.
enum InCell {
    /// Not edited: every token is copied.
    Copy,
    /// Edited: every token is dropped and the edit written at the end tag.
    Replace {
        reference: CellRef,
        style: Option<Vec<u8>>,
        /// The cell's own prefix, which its replacement keeps.
        prefix: Vec<u8>,
        value: Option<Value>,
        /// Whether the cell's `<f>` was a formula this edit removes.
        formula: bool,
    },
}

/// The bytes that replace an edited cell: the value's element, or for a
/// clear the cell with its style alone, or nothing.
fn replaced_cell(
    reference: CellRef,
    style: Option<&[u8]>,
    value: Option<&Value>,
    prefix: &[u8],
) -> Vec<u8> {
    match (value, style) {
        (Some(value), _) => value.element(reference, style, prefix),
        (None, Some(style)) => {
            let mut out = [b"<".as_slice(), prefix, b"c r=\""].concat();
            out.extend_from_slice(reference.a1().as_bytes());
            out.extend_from_slice(b"\" s=\"");
            out.extend_from_slice(style);
            out.extend_from_slice(b"\"/>");
            out
        }
        (None, None) => Vec::new(),
    }
}

/// A cell's start tag written anew for a style edit alone
/// (`specs/0012-styles.md`, "Module `package`"): `prefix` on the name, the
/// cell's own `r` and `t` bytes, and `style` as `s` between them when there
/// is one, closed with `/>` for an empty cell and `>` otherwise.
fn restyled_tag(tag: &CellTag, style: Option<&[u8]>, prefix: &[u8], empty: bool) -> Vec<u8> {
    let mut out = [b"<".as_slice(), prefix, b"c r=\""].concat();
    out.extend_from_slice(&tag.written_reference);
    out.push(b'"');
    if let Some(style) = style {
        out.extend_from_slice(b" s=\"");
        out.extend_from_slice(style);
        out.push(b'"');
    }
    if let Some(kind) = &tag.kind {
        out.extend_from_slice(b" t=\"");
        out.extend_from_slice(kind);
        out.push(b'"');
    }
    out.extend_from_slice(if empty { b"/>" } else { b">" });
    out
}

/// An empty element's raw bytes with the `/` of its `/>` dropped, so it can
/// be given content and an end tag (part 2's rule for an empty root).
fn opened_up(raw: &[u8], index: usize) -> Result<Vec<u8>, Error> {
    let slash = raw
        .iter()
        .rposition(|byte| *byte == b'/')
        .ok_or(Error::Xml(index))?;
    let mut out = raw[..slash].to_vec();
    out.extend_from_slice(&raw[slash + 1..]);
    Ok(out)
}

/// An element's qualified-name prefix with its colon, or empty.
fn prefix_of(start: &BytesStart<'_>) -> Vec<u8> {
    match start.name().prefix() {
        Some(prefix) => [prefix.into_inner().as_bytes(), b":"].concat(),
        None => Vec::new(),
    }
}

/// What a `<c>` start tag says to the rewrite.
struct CellTag {
    reference: Option<CellRef>,
    /// `r`'s bytes as written, which a style edit alone writes back.
    written_reference: Vec<u8>,
    style: Option<Vec<u8>>,
    /// `t`'s bytes as written, which a style edit alone writes back.
    kind: Option<Vec<u8>>,
    /// Whether it carries an attribute other than `r`, `s` and `t`.
    unmodelled: bool,
}

fn cell_tag(start: &BytesStart<'_>, scope: &Scope, index: usize) -> Result<CellTag, Error> {
    let mut tag = CellTag {
        reference: None,
        written_reference: Vec::new(),
        style: None,
        kind: None,
        unmodelled: false,
    };
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        match (namespace, local) {
            (None, b"r") => {
                tag.reference = Some(
                    CellRef::parse(attribute.value.as_bytes())
                        .ok_or(Error::Package("a cell reference is not of the form A1"))?,
                );
                tag.written_reference = attribute.value.as_bytes().to_vec();
            }
            (None, b"s") => tag.style = Some(attribute.value.as_bytes().to_vec()),
            (None, b"t") => tag.kind = Some(attribute.value.as_bytes().to_vec()),
            _ => tag.unmodelled = true,
        }
    }
    Ok(tag)
}

/// A `<row>`'s `r`: absent is refused, since an implied number would move
/// when a row is inserted before it.
fn explicit_row(start: &BytesStart<'_>, scope: &Scope, index: usize) -> Result<u32, Error> {
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        if namespace.is_none() && local == b"r" {
            return str::from_utf8(attribute.value.as_bytes())
                .ok()
                .and_then(decimal)
                .filter(|number| *number > 0)
                .ok_or(Error::Package("a row number is not a number"));
        }
    }
    Err(NO_EXPLICIT_REFERENCES)
}

/// An `<f>` start tag's `t`, and its `ref` for an array or data table
/// formula.
enum FormulaTag {
    /// `t` absent or `normal`: a formula an edit may remove.
    Removable,
    Shared,
    /// `array` or `dataTable` (P1 ruling 3): a master whose `ref` names
    /// other cells holding its results, `None` when the `ref` does not parse.
    Array(Option<Area>),
}

fn formula_tag(start: &BytesStart<'_>, scope: &Scope, index: usize) -> Result<FormulaTag, Error> {
    let mut kind: Option<Vec<u8>> = None;
    let mut reference: Option<Vec<u8>> = None;
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        match (namespace, local) {
            (None, b"t") => kind = Some(attribute.value.as_bytes().to_vec()),
            (None, b"ref") => reference = Some(attribute.value.as_bytes().to_vec()),
            _ => {}
        }
    }
    match kind.as_deref() {
        None | Some(b"normal") => Ok(FormulaTag::Removable),
        Some(b"shared") => Ok(FormulaTag::Shared),
        Some(b"array") | Some(b"dataTable") => Ok(FormulaTag::Array(
            reference.as_deref().and_then(Area::parse),
        )),
        Some(_) => Err(Error::Package(
            "a formula has a type this layer does not know",
        )),
    }
}

/// The root children of a worksheet in the order ECMA-376 Part 1,
/// 18.3.1.99 gives them: an element the rewrite creates goes immediately
/// before the first root child of the main namespace that comes later here,
/// or before the root's end tag when none does
/// (`specs/0018-rows-and-columns.md`, "Creating an element").
const WORKSHEET_CHILDREN: [&[u8]; 39] = [
    b"sheetPr",
    b"dimension",
    b"sheetViews",
    b"sheetFormatPr",
    b"cols",
    b"sheetData",
    b"sheetCalcPr",
    b"sheetProtection",
    b"protectedRanges",
    b"scenarios",
    b"autoFilter",
    b"sortState",
    b"dataConsolidate",
    b"customSheetViews",
    b"mergeCells",
    b"phoneticPr",
    b"conditionalFormatting",
    b"dataValidations",
    b"hyperlinks",
    b"printOptions",
    b"pageMargins",
    b"pageSetup",
    b"headerFooter",
    b"rowBreaks",
    b"colBreaks",
    b"customProperties",
    b"cellWatches",
    b"ignoredErrors",
    b"smartTags",
    b"drawing",
    b"legacyDrawing",
    b"legacyDrawingHF",
    b"drawingHF",
    b"picture",
    b"oleObjects",
    b"controls",
    b"webPublishItems",
    b"tableParts",
    b"extLst",
];

/// The three root children a worksheet rewrite may create, in schema order.
const CREATED: [&[u8]; 3] = [b"sheetViews", b"cols", b"mergeCells"];
const VIEWS: usize = 0;
const COLS: usize = 1;
const MERGES: usize = 2;

/// A root child's place in [`WORKSHEET_CHILDREN`].
fn schema_position(local: &[u8]) -> Option<usize> {
    WORKSHEET_CHILDREN.iter().position(|name| *name == local)
}

/// The frozen pane and its one selection for a freeze at `at`, with
/// `prefix` on both names (`specs/0018-rows-and-columns.md`, "Panes").
fn pane_elements(at: CellRef, prefix: &[u8]) -> Vec<u8> {
    let active: &[u8] = match (at.col > 1, at.row > 1) {
        (true, true) => b"bottomRight",
        (true, false) => b"topRight",
        (false, _) => b"bottomLeft",
    };
    let mut out = [b"<".as_slice(), prefix, b"pane"].concat();
    if at.col > 1 {
        out.extend_from_slice(format!(" xSplit=\"{}\"", at.col - 1).as_bytes());
    }
    if at.row > 1 {
        out.extend_from_slice(format!(" ySplit=\"{}\"", at.row - 1).as_bytes());
    }
    out.extend_from_slice(b" topLeftCell=\"");
    out.extend_from_slice(at.a1().as_bytes());
    out.extend_from_slice(b"\" activePane=\"");
    out.extend_from_slice(active);
    out.extend_from_slice(b"\" state=\"frozen\"/><");
    out.extend_from_slice(prefix);
    out.extend_from_slice(b"selection pane=\"");
    out.extend_from_slice(active);
    out.extend_from_slice(b"\"/>");
    out
}

/// A `<mergeCell>` for `area`, with `prefix` on its name.
fn merge_element(area: &Area, prefix: &[u8]) -> Vec<u8> {
    let mut out = [b"<".as_slice(), prefix, b"mergeCell ref=\""].concat();
    out.extend_from_slice(area.a1().as_bytes());
    out.extend_from_slice(b"\"/>");
    out
}

/// One piece of a `<cols>` held while it is rewritten.
enum ColItem {
    /// A `<col>`: its start tag, whatever follows up to and including its
    /// end tag (nothing for an empty element), and its interval.
    Col {
        start: Vec<u8>,
        rest: Vec<u8>,
        min: u32,
        max: u32,
    },
    /// Anything else among the children, copied.
    Other(Vec<u8>),
}

/// A `<cols>` being read: its start tag, its prefix, its children so far,
/// and whether the last `<col>` is still open.
struct ColsBuffer {
    depth: usize,
    start: Vec<u8>,
    empty: bool,
    name: Vec<u8>,
    prefix: Vec<u8>,
    items: Vec<ColItem>,
    open: bool,
}

impl ColsBuffer {
    /// Where a token's bytes go: the open `<col>`'s tail, or a piece of its
    /// own.
    fn push(&mut self, raw: &[u8]) {
        match self.items.last_mut() {
            Some(ColItem::Col { rest, .. }) if self.open => rest.extend_from_slice(raw),
            _ => self.items.push(ColItem::Other(raw.to_vec())),
        }
    }
}

/// A `<col>`'s `min` and `max`, refused as P1 ruling 3 gives: absent, not a
/// decimal `u32`, a `min` of 0 or a `min` above `max`.
fn col_interval(raw: &[u8], index: usize) -> Result<(u32, u32), Error> {
    let number = |name: &[u8]| -> Result<u32, Error> {
        attribute_value(raw, name, index)?
            .and_then(|value| str::from_utf8(value).ok())
            .and_then(decimal)
            .ok_or(COLS_DISORDER)
    };
    let (min, max) = (number(b"min")?, number(b"max")?);
    if min == 0 || min > max {
        return Err(COLS_DISORDER);
    }
    Ok((min, max))
}

/// A `<col>` start tag with `min` and `max` set.
fn col_retagged(raw: &[u8], min: u32, max: u32, index: usize) -> Result<Vec<u8>, Error> {
    let tag = with_attribute(raw, b"min", min.to_string().as_bytes(), index)?;
    with_attribute(&tag, b"max", max.to_string().as_bytes(), index)
}

/// Whether a start tag carries no attribute but `min` and `max`.
fn bare_col(raw: &[u8], index: usize) -> Result<bool, Error> {
    let tag = scan_start_tag(raw).ok_or(Error::Xml(index))?;
    Ok(tag
        .attributes
        .iter()
        .all(|attribute| matches!(&raw[attribute.name.clone()], b"min" | b"max")))
}

/// The children of a `<cols>` with the column formats applied, and whether
/// any `<col>` is left (`specs/0018-rows-and-columns.md`, "Module
/// `package`: the calls"): a covering `<col>` split so the edited column
/// alone carries the change, the other pieces keeping every attribute with
/// `min` and `max` set; a column no `<col>` covers given one of its own,
/// placed by `min`, when the format writes something; an edited piece left
/// with `min` and `max` alone dropped. The elements are refused when out of
/// order or overlapping.
fn cols_content(
    items: &[ColItem],
    formats: &[(u32, LineFormat)],
    prefix: &[u8],
    index: usize,
) -> Result<(Vec<u8>, bool), Error> {
    let mut out = Vec::new();
    let mut any = false;
    let mut next = 0usize;
    let mut last_max = 0u32;
    let created = |column: u32, format: &LineFormat| -> Result<Vec<u8>, Error> {
        let base = [b"<".as_slice(), prefix, b"col/>"].concat();
        let tag = col_retagged(&base, column, column, index)?;
        line_formatted(&tag, format, COLUMN_SIZE, index)
    };
    for item in items {
        match item {
            ColItem::Other(raw) => out.extend_from_slice(raw),
            ColItem::Col {
                start,
                rest,
                min,
                max,
            } => {
                if *min <= last_max {
                    return Err(COLS_DISORDER);
                }
                last_max = *max;
                while let Some((column, format)) = formats.get(next)
                    && *column < *min
                {
                    if format.writes_when_absent() {
                        out.extend_from_slice(&created(*column, format)?);
                        any = true;
                    }
                    next += 1;
                }
                let mut from = *min;
                while let Some((column, format)) = formats.get(next)
                    && *column <= *max
                {
                    if *column > from {
                        out.extend_from_slice(&col_retagged(start, from, column - 1, index)?);
                        out.extend_from_slice(rest);
                        any = true;
                    }
                    let edited = line_formatted(
                        &col_retagged(start, *column, *column, index)?,
                        format,
                        COLUMN_SIZE,
                        index,
                    )?;
                    if !bare_col(&edited, index)? {
                        out.extend_from_slice(&edited);
                        out.extend_from_slice(rest);
                        any = true;
                    }
                    from = column + 1;
                    next += 1;
                }
                if from <= *max {
                    if from == *min {
                        out.extend_from_slice(start);
                    } else {
                        out.extend_from_slice(&col_retagged(start, from, *max, index)?);
                    }
                    out.extend_from_slice(rest);
                    any = true;
                }
            }
        }
    }
    for (column, format) in &formats[next..] {
        if format.writes_when_absent() {
            out.extend_from_slice(&created(*column, format)?);
            any = true;
        }
    }
    Ok((out, any))
}

/// A `<mergeCells>` being read: its start tag, its prefix, the bytes of the
/// children it keeps, and how many `<mergeCell>` elements those are.
struct MergesBuffer {
    depth: usize,
    start: Vec<u8>,
    empty: bool,
    name: Vec<u8>,
    prefix: Vec<u8>,
    content: Vec<u8>,
    kept: u64,
}

/// The `<mergeCells>` element for `kept` bytes of kept children plus the
/// merges this write adds, with `count` set to all of them; nothing when
/// none is left.
fn merges_element(buffer: &MergesBuffer, merges: &[Area], index: usize) -> Result<Vec<u8>, Error> {
    let count = buffer.kept + merges.len() as u64;
    if count == 0 {
        return Ok(Vec::new());
    }
    let start = with_attribute(&buffer.start, b"count", count.to_string().as_bytes(), index)?;
    let mut out = if buffer.empty {
        opened_up(&start, index)?
    } else {
        start
    };
    out.extend_from_slice(&buffer.content);
    for area in merges {
        out.extend_from_slice(&merge_element(area, &buffer.prefix));
    }
    out.extend_from_slice(&[b"</".as_slice(), &buffer.name, b">"].concat());
    Ok(out)
}

/// What a worksheet rewrite creates for one of [`CREATED`], with the root's
/// `prefix`, or nothing when the write needs no such element.
fn created_element(
    which: usize,
    work: &SheetWork,
    prefix: &[u8],
    index: usize,
) -> Result<Vec<u8>, Error> {
    let p = prefix;
    match which {
        VIEWS => {
            // The created view takes the view settings by the settings pass's
            // merge (`specs/0025-sheet-and-print-settings.md`, "The calls").
            let base = [b"<".as_slice(), p, b"sheetView workbookViewId=\"0\"/>"].concat();
            let tag = view_tag(work, &base, index)?;
            let mut out = [b"<".as_slice(), p, b"sheetViews>"].concat();
            match work.pane {
                Some(Some(at)) => {
                    out.extend_from_slice(&opened_up(&tag, index)?);
                    out.extend_from_slice(&pane_elements(at, p));
                    out.extend_from_slice(&[b"</".as_slice(), p, b"sheetView>"].concat());
                }
                _ if view_edit(work).1 => out.extend_from_slice(&tag),
                _ => return Ok(Vec::new()),
            }
            out.extend_from_slice(&[b"</".as_slice(), p, b"sheetViews>"].concat());
            Ok(out)
        }
        COLS => {
            let (content, any) = cols_content(&[], &work.columns, p, index)?;
            if !any {
                return Ok(Vec::new());
            }
            let mut out = [b"<".as_slice(), p, b"cols>"].concat();
            out.extend_from_slice(&content);
            out.extend_from_slice(&[b"</".as_slice(), p, b"cols>"].concat());
            Ok(out)
        }
        _ => {
            let buffer = MergesBuffer {
                depth: 0,
                start: [b"<".as_slice(), p, b"mergeCells>"].concat(),
                empty: false,
                name: [p, b"mergeCells".as_slice()].concat(),
                prefix: p.to_vec(),
                content: Vec::new(),
                kept: 0,
            };
            merges_element(&buffer, &work.merges, index)
        }
    }
}

/// Rewrites one worksheet part with its edits into a temporary file, and
/// says whether any edit removed a formula.
///
/// `specs/0011-editing-cells.md`, "Module `package`": every token's `raw` is
/// copied except that an edited `<c>` (start tag, children and end tag, or
/// its one empty tag) is replaced by the edit's form; a new cell goes before
/// the first cell of its row with a greater column, or before `</row>`; a new
/// row goes before the first `<row>` with a greater number, or before
/// `</sheetData>`; an empty `<sheetData/>` or `<row/>` that gains content is
/// opened up as part 2 opens an empty root.
///
/// `specs/0018-rows-and-columns.md` adds the formats, the pane and the
/// merges: a `<row>` with a format is retagged and one that is absent
/// created; `<cols>` is rewritten whole when a column has a format; the first
/// `<sheetView>` loses its `<pane>` and `<selection>` children and, for a
/// freeze, gains the frozen pane first; `<mergeCells>` keeps its children
/// less the unmerged ones and gains the merges, `count` set; a covered cell
/// is cleared; and `<sheetViews>`, `<cols>` and `<mergeCells>` are created in
/// schema position when the write needs one the part lacks. Everything else
/// is the input's bytes.
fn rewrite_worksheet(
    archive: &mut Core,
    index: usize,
    work: &SheetWork,
) -> Result<(TempFile, Replacement, bool), Error> {
    rewrite_worksheet_from(archive.read(index)?, index, work)
}

/// [`rewrite_worksheet`]'s work on any reader of a worksheet's bytes: the
/// archive's entry, or the template's blank worksheet for a sheet this write
/// adds (`specs/0016-new-workbook-and-sheets.md`, P3 ruling 2), `index`
/// being the entry's index for an error.
fn rewrite_worksheet_from(
    input: impl Read,
    index: usize,
    work: &SheetWork,
) -> Result<(TempFile, Replacement, bool), Error> {
    let (temp, file) = temp_for_part()?;
    let mut out = BufWriter::new(file);
    let mut stream = Stream::new(input, index)?;
    if stream.bom() {
        out.write_all(&BYTE_ORDER_MARK)?;
    }
    let mut pending = Pending {
        edits: &work.cells,
        next: 0,
        rows: &work.rows,
        next_row: 0,
        index,
    };
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut root = false;
    let mut root_prefix: Vec<u8> = Vec::new();
    // The depth of the open `<sheetData>`; its rows are one deeper.
    let mut data: Option<usize> = None;
    let mut seen_data = false;
    // The open `<row>`'s number, the last row number seen and the open row's
    // last column, which order is checked against.
    let mut row: Option<u32> = None;
    let mut last_row = 0u32;
    let mut last_col = 0u32;
    // The `<c>` being read, at its depth.
    let mut cell: Option<(usize, InCell)> = None;
    // The prefixes new elements take: the `<sheetData>`'s for a new row, the
    // open `<row>`'s for a new cell in it.
    let mut data_prefix: Vec<u8> = Vec::new();
    let mut row_prefix: Vec<u8> = Vec::new();
    let mut arrays: Vec<Area> = Vec::new();
    let mut removed_formula = false;

    // Which of `CREATED` the write touches, which it must create when the
    // part lacks it, and which the pass has met or created.
    let touched = [
        work.pane.is_some() || view_edit(work).0,
        !work.columns.is_empty(),
        !work.merges.is_empty() || !work.unmerges.is_empty(),
    ];
    let needed = [
        matches!(work.pane, Some(Some(_))) || view_edit(work).1,
        work.columns
            .iter()
            .any(|(_, format)| format.writes_when_absent()),
        !work.merges.is_empty(),
    ];
    let mut met = [false; 3];
    // The open `<sheetViews>`'s depth and whether it has shown a
    // `<sheetView>`; the first `<sheetView>`'s depth while it is open.
    let mut views: Option<usize> = None;
    let mut has_view = false;
    let mut first_view: Option<usize> = None;
    let mut cols: Option<ColsBuffer> = None;
    let mut merges: Option<MergesBuffer> = None;
    let mut unmerged = vec![false; work.unmerges.len()];
    #[cfg(feature = "metadata")]
    let mut links = LinksPass::new(&work.links);
    #[cfg(feature = "metadata")]
    let mut protect = ProtectPass {
        edit: work.protection,
        met: false,
    };
    #[cfg(feature = "rules")]
    let mut rules = ValidationsPass::new(&work.validations);
    #[cfg(feature = "rules")]
    let mut formats = FormatsPass::new(&work.rules);
    #[cfg(feature = "tables")]
    let mut tables = TablesPass::new(&work.tables, &work.tables_removed);
    #[cfg(feature = "images")]
    let mut drawing = ElementPass::new(DRAWING_ELEMENT, work.drawing.as_deref());
    #[cfg(feature = "comments")]
    let mut legacy = ElementPass::new(LEGACY_DRAWING, work.legacy.as_deref());
    #[cfg(feature = "sheetprops")]
    let mut settings = SettingsPass::new(&work.settings);

    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                // The part's merges and autofilter against the tables the
                // write adds (`specs/0021-tables.md`, decision 4 A).
                #[cfg(feature = "tables")]
                tables.observe(token.raw, (local, ours, depth), &work.unmerges, index)?;
                // Inside a held `<sheetPr>`: its children are the settings
                // pass's (`specs/0025-sheet-and-print-settings.md`, "The
                // pass").
                #[cfg(feature = "sheetprops")]
                if settings.held.is_some() {
                    settings.child(token.raw, local, ours, depth);
                    if empty {
                        scope.end();
                        depth -= 1;
                    }
                    continue;
                }
                // Inside a held `<hyperlinks>`: its children are the links
                // pass's (`specs/0019-names-links-properties-protection.md`).
                #[cfg(feature = "metadata")]
                if links.buffer.is_some() {
                    let skipped =
                        links.child(token.raw, start, (local, ours, depth), &scope, index)?;
                    if skipped && !empty {
                        skip_element(&mut stream, &mut scope)?;
                    }
                    if skipped || empty {
                        scope.end();
                        depth -= 1;
                    }
                    continue;
                }
                // Inside a held `<dataValidations>`: its children are the
                // validations pass's (`specs/0020-validation-and-conditional-
                // formatting.md`, "Data validation").
                // Inside a held `<conditionalFormatting>`: its tokens are the
                // formats pass's ("Conditional formatting").
                #[cfg(feature = "rules")]
                if formats.held.is_some() {
                    formats.child(token.raw, (local, ours, depth), index)?;
                    if empty {
                        scope.end();
                        depth -= 1;
                    }
                    continue;
                }
                #[cfg(feature = "rules")]
                if rules.buffer.is_some() {
                    let skipped = rules.child(token.raw, (local, ours, depth), index)?;
                    if skipped && !empty {
                        skip_element(&mut stream, &mut scope)?;
                    }
                    if skipped || empty {
                        scope.end();
                        depth -= 1;
                    }
                    continue;
                }
                // Inside a held `<tableParts>`: its children are the tables
                // pass's (`specs/0021-tables.md`, "The worksheet").
                #[cfg(feature = "tables")]
                if tables.buffer.is_some() {
                    let skipped =
                        tables.child(token.raw, start, (local, ours, depth), &scope, index)?;
                    if skipped && !empty {
                        skip_element(&mut stream, &mut scope)?;
                    }
                    if skipped || empty {
                        scope.end();
                        depth -= 1;
                    }
                    continue;
                }
                if root && depth == 2 && ours {
                    // Creation in schema position, and the refusal of an
                    // element met twice or after it was created (P1 ruling
                    // 2).
                    if let Some(position) = schema_position(local) {
                        // The settings elements: the refusal of one met twice
                        // or out of order, and the creation of one before
                        // every element above (`specs/0025-sheet-and-print-
                        // settings.md`, "The pass").
                        #[cfg(feature = "sheetprops")]
                        {
                            settings.observe(local, position)?;
                            out.write_all(&settings.create_before(
                                position,
                                &root_prefix,
                                index,
                            )?)?;
                        }
                        for which in 0..CREATED.len() {
                            let at = schema_position(CREATED[which]).unwrap_or(0);
                            // A created `<sheetProtection>` goes in its
                            // place among the created elements.
                            #[cfg(feature = "metadata")]
                            if at > ProtectPass::position()
                                && let Some(bytes) = protect.create_before(position, &root_prefix)
                            {
                                out.write_all(&bytes)?;
                            }
                            if needed[which] && !met[which] && at < position {
                                out.write_all(&created_element(which, work, &root_prefix, index)?)?;
                                met[which] = true;
                            }
                        }
                        #[cfg(feature = "metadata")]
                        if let Some(bytes) = protect.create_before(position, &root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "rules")]
                        if let Some(bytes) = formats.create_before(position, &root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "rules")]
                        if let Some(bytes) = rules.create_before(position, &root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "metadata")]
                        if let Some(bytes) = links.create_before(position, &root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "images")]
                        if let Some(bytes) = drawing.create_before(position, &root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "comments")]
                        if let Some(bytes) = legacy.create_before(position, &root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "tables")]
                        if let Some(bytes) = tables.create_before(position, &root_prefix) {
                            out.write_all(&bytes)?;
                        }
                    }
                    if let Some(which) = CREATED.iter().position(|name| *name == local)
                        && touched[which]
                    {
                        if met[which] {
                            return Err(TWO_ELEMENTS);
                        }
                        met[which] = true;
                    }
                }
                // A `<legacyDrawing>` the write drops with its VML part, the
                // last comment gone (`specs/0022-comments-and-images.md`,
                // `remove_comment`).
                #[cfg(feature = "comments")]
                if root && depth == 2 && ours && local == LEGACY_DRAWING && work.legacy_drop {
                    if !empty {
                        skip_element(&mut stream, &mut scope)?;
                    }
                    scope.end();
                    depth -= 1;
                    continue;
                }
                // The part's `<sheetPr>`, held to its end tag and rewritten
                // (`specs/0025-sheet-and-print-settings.md`, "The pass").
                #[cfg(feature = "sheetprops")]
                if root && depth == 2 && ours && local == SHEET_PR && settings.touches_sheet_pr() {
                    let held = Held {
                        depth,
                        start: token.raw.to_vec(),
                        empty,
                        name: start.name().into_inner().as_bytes().to_vec(),
                        prefix: prefix_of(start),
                        children: Vec::new(),
                    };
                    if empty {
                        out.write_all(&settings.sheet_pr_written(held, index)?)?;
                        scope.end();
                        depth -= 1;
                    } else {
                        settings.held = Some(held);
                    }
                    continue;
                }
                // The part's `<sheetProtection>`: replaced whole or dropped,
                // a second refused.
                #[cfg(feature = "metadata")]
                if root
                    && depth == 2
                    && ours
                    && local == b"sheetProtection"
                    && protect.edit.is_some()
                {
                    if protect.met {
                        return Err(TWO_SHEET_ELEMENTS);
                    }
                    protect.met = true;
                    let bytes = protect.element(&prefix_of(start));
                    if !empty {
                        skip_element(&mut stream, &mut scope)?;
                    }
                    out.write_all(&bytes)?;
                    scope.end();
                    depth -= 1;
                    continue;
                }
                #[cfg(feature = "rules")]
                if root && depth == 2 && ours && local == CONDITIONAL_FORMATTING && formats.touched
                {
                    out.write_all(&formats.open(token.raw, depth, empty, index)?)?;
                    if empty {
                        scope.end();
                        depth -= 1;
                    }
                    continue;
                }
                #[cfg(feature = "rules")]
                if root && depth == 2 && ours && local == DATA_VALIDATIONS && rules.touched {
                    out.write_all(&rules.open(token.raw, start, depth, empty, index)?)?;
                    if empty {
                        scope.end();
                        depth -= 1;
                    }
                    continue;
                }
                #[cfg(feature = "metadata")]
                if root && depth == 2 && ours && local == HYPERLINKS && links.touched {
                    out.write_all(&links.open(token.raw, start, depth, empty, index)?)?;
                    if empty {
                        scope.end();
                        depth -= 1;
                    }
                    continue;
                }
                #[cfg(feature = "tables")]
                if root && depth == 2 && ours && local == TABLE_PARTS && tables.touched {
                    out.write_all(&tables.open(token.raw, start, depth, empty, index)?)?;
                    if empty {
                        scope.end();
                        depth -= 1;
                    }
                    continue;
                }
                if !root {
                    if !ours || local != b"worksheet" {
                        return Err(Error::Package("the worksheet part's root is not worksheet"));
                    }
                    root = true;
                    root_prefix = prefix_of(start);
                    // The root's start tag, retagged with a relationships
                    // prefix when an external link needs one (decision 3 A).
                    #[cfg(feature = "metadata")]
                    let root_raw = links.root_tag(token.raw, start, work.links_strict, index)?;
                    #[cfg(not(feature = "metadata"))]
                    let root_raw = token.raw.to_vec();
                    // And for a table's `r:id` (`specs/0021-tables.md`, "The
                    // worksheet").
                    #[cfg(feature = "tables")]
                    let root_raw = tables.root_tag(&root_raw, work.tables_strict, index)?;
                    // And for a created `<legacyDrawing>`'s
                    // (`specs/0022-comments-and-images.md`, "Comments: the
                    // parts").
                    #[cfg(feature = "comments")]
                    let root_raw = legacy.root_tag(&root_raw, work.legacy_strict, index)?;
                    // And for a created `<drawing>`'s ("Images: the parts").
                    #[cfg(feature = "images")]
                    let root_raw = drawing.root_tag(&root_raw, work.drawing_strict, index)?;
                    #[cfg(feature = "metadata")]
                    let links_needed = links.needed || matches!(protect.edit, Some(Some(_)));
                    #[cfg(not(feature = "metadata"))]
                    let links_needed = false;
                    #[cfg(feature = "rules")]
                    let links_needed = links_needed || rules.needed || formats.needed;
                    #[cfg(feature = "tables")]
                    let links_needed = links_needed || !tables.tables.is_empty();
                    #[cfg(feature = "comments")]
                    let links_needed = links_needed || work.legacy.is_some();
                    #[cfg(feature = "images")]
                    let links_needed = links_needed || work.drawing.is_some();
                    #[cfg(feature = "sheetprops")]
                    let links_needed = links_needed || settings.needs_sheet_pr();
                    if empty && (needed.iter().any(|need| *need) || links_needed) {
                        out.write_all(&opened_up(&root_raw, index)?)?;
                        #[cfg(feature = "sheetprops")]
                        out.write_all(&settings.create_before(usize::MAX, &root_prefix, index)?)?;
                        for which in 0..CREATED.len() {
                            #[cfg(feature = "metadata")]
                            if schema_position(CREATED[which]).unwrap_or(0)
                                > ProtectPass::position()
                                && let Some(bytes) = protect.create_before(usize::MAX, &root_prefix)
                            {
                                out.write_all(&bytes)?;
                            }
                            if needed[which] {
                                out.write_all(&created_element(which, work, &root_prefix, index)?)?;
                                met[which] = true;
                            }
                        }
                        #[cfg(feature = "metadata")]
                        if let Some(bytes) = protect.create_before(usize::MAX, &root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "rules")]
                        if let Some(bytes) = formats.create_at_end(&root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "rules")]
                        if let Some(bytes) = rules.create_at_end(&root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "metadata")]
                        if let Some(bytes) = links.create_at_end(&root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "images")]
                        if let Some(bytes) = drawing.create_at_end(&root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "comments")]
                        if let Some(bytes) = legacy.create_at_end(&root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        #[cfg(feature = "tables")]
                        if let Some(bytes) = tables.create_at_end(&root_prefix) {
                            out.write_all(&bytes)?;
                        }
                        out.write_all(b"</")?;
                        out.write_all(start.name().into_inner().as_bytes())?;
                        out.write_all(b">")?;
                    } else {
                        out.write_all(&root_raw)?;
                    }
                } else if let Some(buffer) = cols.as_mut() {
                    if depth == buffer.depth + 1 && ours && local == b"col" {
                        let (min, max) = col_interval(token.raw, index)?;
                        buffer.items.push(ColItem::Col {
                            start: token.raw.to_vec(),
                            rest: Vec::new(),
                            min,
                            max,
                        });
                        buffer.open = !empty;
                    } else {
                        buffer.push(token.raw);
                    }
                } else if let Some(buffer) = merges.as_mut() {
                    if depth == buffer.depth + 1 && ours && local == b"mergeCell" {
                        let area = attribute_value(token.raw, b"ref", index)?
                            .and_then(Area::parse)
                            .ok_or(MERGE_OVERLAP)?;
                        let mut gone = false;
                        for (at, unmerge) in work.unmerges.iter().enumerate() {
                            if *unmerge == area {
                                unmerged[at] = true;
                                gone = true;
                            }
                        }
                        if gone {
                            // The unmerged element goes, its content with it.
                            if !empty {
                                skip_element(&mut stream, &mut scope)?;
                            }
                            scope.end();
                            depth -= 1;
                            continue;
                        }
                        if work.merges.iter().any(|merge| merge.overlaps(&area)) {
                            return Err(MERGE_OVERLAP);
                        }
                        buffer.kept += 1;
                    }
                    buffer.content.extend_from_slice(token.raw);
                } else if first_view.is_some_and(|at| depth == at + 1)
                    && ours
                    && (local == b"pane" || local == b"selection")
                {
                    // The view's pane and selections go (`specs/0018-rows-
                    // and-columns.md`, "Panes"); the freeze wrote its own.
                    if !empty {
                        skip_element(&mut stream, &mut scope)?;
                    }
                    scope.end();
                    depth -= 1;
                    continue;
                } else if views.is_some_and(|at| depth == at + 1)
                    && ours
                    && local == b"sheetView"
                    && !has_view
                {
                    has_view = true;
                    let prefix = prefix_of(start);
                    // Its attributes with the view settings merged
                    // (`specs/0025-sheet-and-print-settings.md`, "The pass").
                    let raw = view_tag(work, token.raw, index)?;
                    match (work.pane, empty) {
                        (Some(Some(at)), true) => {
                            out.write_all(&opened_up(&raw, index)?)?;
                            out.write_all(&pane_elements(at, &prefix))?;
                            out.write_all(b"</")?;
                            out.write_all(start.name().into_inner().as_bytes())?;
                            out.write_all(b">")?;
                        }
                        (pane, _) => {
                            out.write_all(&raw)?;
                            if let Some(Some(at)) = pane {
                                out.write_all(&pane_elements(at, &prefix))?;
                            }
                            // Its pane and selections go only for a freeze or
                            // an unfreeze.
                            if !empty && pane.is_some() {
                                first_view = Some(depth);
                            }
                        }
                    }
                } else if depth == 2 && ours && local == b"sheetViews" && touched[VIEWS] {
                    if empty {
                        return Err(TWO_ELEMENTS);
                    }
                    out.write_all(token.raw)?;
                    views = Some(depth);
                } else if depth == 2 && ours && local == b"cols" && touched[COLS] {
                    let buffer = ColsBuffer {
                        depth,
                        start: token.raw.to_vec(),
                        empty,
                        name: start.name().into_inner().as_bytes().to_vec(),
                        prefix: prefix_of(start),
                        items: Vec::new(),
                        open: false,
                    };
                    if empty {
                        out.write_all(&cols_written(&buffer, &work.columns, index)?)?;
                    } else {
                        cols = Some(buffer);
                    }
                } else if depth == 2 && ours && local == b"mergeCells" && touched[MERGES] {
                    let buffer = MergesBuffer {
                        depth,
                        start: token.raw.to_vec(),
                        empty,
                        name: start.name().into_inner().as_bytes().to_vec(),
                        prefix: prefix_of(start),
                        content: Vec::new(),
                        kept: 0,
                    };
                    if empty {
                        out.write_all(&merges_element(&buffer, &work.merges, index)?)?;
                    } else {
                        merges = Some(buffer);
                    }
                } else if let Some((at, ref mut inside)) = cell {
                    // Inside a cell: only its own `<f>` matters.
                    if depth == at + 1 && ours && local == b"f" {
                        // A table's header cell holding a formula
                        // (`specs/0021-tables.md`, decision 3 A).
                        #[cfg(feature = "tables")]
                        if let InCell::Replace { reference, .. } = inside
                            && work.table_header(*reference)
                        {
                            return Err(TABLE_HEADER_FORMULA);
                        }
                        let tag = formula_tag(start, &scope, index);
                        match inside {
                            InCell::Copy => {
                                if let Ok(FormulaTag::Array(area)) = tag {
                                    arrays.push(area.ok_or(IN_SHARED_OR_ARRAY)?);
                                }
                            }
                            InCell::Replace { formula, .. } => match tag? {
                                FormulaTag::Removable => *formula = true,
                                FormulaTag::Shared | FormulaTag::Array(_) => {
                                    return Err(IN_SHARED_OR_ARRAY);
                                }
                            },
                        }
                    }
                    if matches!(inside, InCell::Copy) {
                        out.write_all(token.raw)?;
                    }
                } else if data.is_some_and(|at| depth == at + 1) && ours && local == b"row" {
                    let number = explicit_row(start, &scope, index)?;
                    if number <= last_row {
                        return Err(OUT_OF_ORDER);
                    }
                    last_row = number;
                    pending.write_rows(&mut out, Some(number), &arrays, &data_prefix)?;
                    row_prefix = prefix_of(start);
                    // A row with a format is retagged, every other byte kept
                    // (`specs/0018-rows-and-columns.md`).
                    let raw = match pending.take_row_format(number) {
                        Some(format) => line_formatted(token.raw, &format, ROW_SIZE, index)?,
                        None => token.raw.to_vec(),
                    };
                    if empty && pending.sets(Some(number)) {
                        out.write_all(&opened_up(&raw, index)?)?;
                        pending.write_row(&mut out, number, None, &arrays, &row_prefix)?;
                        out.write_all(b"</")?;
                        out.write_all(start.name().into_inner().as_bytes())?;
                        out.write_all(b">")?;
                    } else {
                        out.write_all(&raw)?;
                        if empty {
                            // Clears of cells a `<row/>` does not hold.
                            pending.write_row(&mut out, number, None, &arrays, &row_prefix)?;
                        } else {
                            row = Some(number);
                            last_col = 0;
                        }
                    }
                } else if let Some(open) = row
                    && data.is_some_and(|at| depth == at + 2)
                    && ours
                    && local == b"c"
                {
                    let tag = cell_tag(start, &scope, index)?;
                    let reference = tag.reference.ok_or(NO_EXPLICIT_REFERENCES)?;
                    if reference.row != open || reference.col <= last_col {
                        return Err(OUT_OF_ORDER);
                    }
                    last_col = reference.col;
                    pending.write_row(&mut out, open, Some(reference.col), &arrays, &row_prefix)?;
                    // An edit recorded for the cell, or the clear a merge
                    // covering it asks for (`specs/0018-rows-and-columns.md`,
                    // decision 6 and P1 ruling 8).
                    let edit = if pending.peek() == Some(reference) {
                        let edit = pending.edits[pending.next].1.clone();
                        pending.next += 1;
                        Some(edit)
                    } else if work.covers(reference) {
                        Some(CellEdit {
                            value: Some(None),
                            style: None,
                        })
                    } else {
                        None
                    };
                    if let Some(edit) = edit {
                        if tag.unmodelled {
                            return Err(UNMODELLED_ATTRIBUTE);
                        }
                        let style = edit.style_bytes(tag.style.clone());
                        match edit.value {
                            // A style edit alone: the start tag anew, and the
                            // children and end tag copied as an unedited
                            // cell's are, so no formula goes and no formula
                            // refusal applies (`specs/0012-styles.md`).
                            None => {
                                out.write_all(&restyled_tag(
                                    &tag,
                                    style.as_deref(),
                                    &prefix_of(start),
                                    empty,
                                ))?;
                                if !empty {
                                    cell = Some((depth, InCell::Copy));
                                }
                            }
                            Some(value) => {
                                if arrays.iter().any(|area| area.holds(reference)) {
                                    return Err(IN_SHARED_OR_ARRAY);
                                }
                                if empty {
                                    out.write_all(&replaced_cell(
                                        reference,
                                        style.as_deref(),
                                        value.as_ref(),
                                        &prefix_of(start),
                                    ))?;
                                } else {
                                    cell = Some((
                                        depth,
                                        InCell::Replace {
                                            reference,
                                            style,
                                            prefix: prefix_of(start),
                                            value,
                                            formula: false,
                                        },
                                    ));
                                }
                            }
                        }
                    } else {
                        out.write_all(token.raw)?;
                        if !empty {
                            cell = Some((depth, InCell::Copy));
                        }
                    }
                } else if depth == 2 && ours && local == b"sheetData" && !seen_data {
                    seen_data = true;
                    data_prefix = prefix_of(start);
                    if empty && pending.opens_rows() {
                        out.write_all(&opened_up(token.raw, index)?)?;
                        pending.write_rows(&mut out, None, &arrays, &data_prefix)?;
                        out.write_all(b"</")?;
                        out.write_all(start.name().into_inner().as_bytes())?;
                        out.write_all(b">")?;
                    } else {
                        out.write_all(token.raw)?;
                        if !empty {
                            data = Some(depth);
                        }
                    }
                } else {
                    out.write_all(token.raw)?;
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                #[cfg(feature = "sheetprops")]
                if let Some(bytes) = settings.end(token.raw, depth, index)? {
                    out.write_all(&bytes)?;
                    scope.end();
                    depth -= 1;
                    continue;
                }
                #[cfg(feature = "metadata")]
                if let Some(bytes) = links.held(token.raw, Some(depth), index)? {
                    out.write_all(&bytes)?;
                    scope.end();
                    depth -= 1;
                    continue;
                }
                #[cfg(feature = "rules")]
                if let Some(bytes) = formats.held(token.raw, Some(depth))? {
                    out.write_all(&bytes)?;
                    scope.end();
                    depth -= 1;
                    continue;
                }
                #[cfg(feature = "rules")]
                if let Some(bytes) = rules.held(token.raw, Some(depth), index)? {
                    out.write_all(&bytes)?;
                    scope.end();
                    depth -= 1;
                    continue;
                }
                #[cfg(feature = "tables")]
                if let Some(bytes) = tables.held(token.raw, Some(depth), index)? {
                    out.write_all(&bytes)?;
                    scope.end();
                    depth -= 1;
                    continue;
                }
                if let Some(buffer) = cols.as_mut() {
                    if depth == buffer.depth {
                        if let Some(buffer) = cols.take() {
                            out.write_all(&cols_written(&buffer, &work.columns, index)?)?;
                        }
                    } else {
                        buffer.push(token.raw);
                        if depth == buffer.depth + 1 {
                            buffer.open = false;
                        }
                    }
                } else if let Some(buffer) = merges.as_mut() {
                    if depth == buffer.depth {
                        if let Some(buffer) = merges.take() {
                            out.write_all(&merges_element(&buffer, &work.merges, index)?)?;
                        }
                    } else {
                        buffer.content.extend_from_slice(token.raw);
                    }
                } else {
                    match cell {
                        Some((at, InCell::Copy)) => {
                            out.write_all(token.raw)?;
                            if depth == at {
                                cell = None;
                            }
                        }
                        Some((at, InCell::Replace { .. })) => {
                            if depth == at
                                && let Some((
                                    _,
                                    InCell::Replace {
                                        reference,
                                        style,
                                        prefix,
                                        value,
                                        formula,
                                    },
                                )) = cell.take()
                            {
                                removed_formula |= formula;
                                out.write_all(&replaced_cell(
                                    reference,
                                    style.as_deref(),
                                    value.as_ref(),
                                    &prefix,
                                ))?;
                            }
                        }
                        None => {
                            if let Some(open) = row
                                && data.is_some_and(|at| depth == at + 1)
                            {
                                pending.write_row(&mut out, open, None, &arrays, &row_prefix)?;
                                row = None;
                            } else if data == Some(depth) {
                                pending.write_rows(&mut out, None, &arrays, &data_prefix)?;
                                data = None;
                            } else if first_view == Some(depth) {
                                first_view = None;
                            } else if views == Some(depth) {
                                if !has_view {
                                    return Err(TWO_ELEMENTS);
                                }
                                views = None;
                            } else if depth == 1 {
                                // The root's end: whatever the write needs and
                                // the part still lacks goes here.
                                #[cfg(feature = "sheetprops")]
                                out.write_all(&settings.create_before(
                                    usize::MAX,
                                    &root_prefix,
                                    index,
                                )?)?;
                                for which in 0..CREATED.len() {
                                    #[cfg(feature = "metadata")]
                                    if schema_position(CREATED[which]).unwrap_or(0)
                                        > ProtectPass::position()
                                        && let Some(bytes) =
                                            protect.create_before(usize::MAX, &root_prefix)
                                    {
                                        out.write_all(&bytes)?;
                                    }
                                    if needed[which] && !met[which] {
                                        out.write_all(&created_element(
                                            which,
                                            work,
                                            &root_prefix,
                                            index,
                                        )?)?;
                                        met[which] = true;
                                    }
                                }
                                #[cfg(feature = "metadata")]
                                if let Some(bytes) = protect.create_before(usize::MAX, &root_prefix)
                                {
                                    out.write_all(&bytes)?;
                                }
                                #[cfg(feature = "rules")]
                                if let Some(bytes) = formats.create_at_end(&root_prefix) {
                                    out.write_all(&bytes)?;
                                }
                                #[cfg(feature = "rules")]
                                if let Some(bytes) = rules.create_at_end(&root_prefix) {
                                    out.write_all(&bytes)?;
                                }
                                #[cfg(feature = "metadata")]
                                if let Some(bytes) = links.create_at_end(&root_prefix) {
                                    out.write_all(&bytes)?;
                                }
                                #[cfg(feature = "images")]
                                if let Some(bytes) = drawing.create_at_end(&root_prefix) {
                                    out.write_all(&bytes)?;
                                }
                                #[cfg(feature = "comments")]
                                if let Some(bytes) = legacy.create_at_end(&root_prefix) {
                                    out.write_all(&bytes)?;
                                }
                                #[cfg(feature = "tables")]
                                if let Some(bytes) = tables.create_at_end(&root_prefix) {
                                    out.write_all(&bytes)?;
                                }
                            }
                            out.write_all(token.raw)?;
                        }
                    }
                }
                scope.end();
                depth -= 1;
            }
            _ => {
                #[cfg(feature = "sheetprops")]
                if settings.other(token.raw, depth) {
                    continue;
                }
                #[cfg(feature = "metadata")]
                if links.held(token.raw, None, index)?.is_some() {
                    continue;
                }
                #[cfg(feature = "rules")]
                if formats.held(token.raw, None)?.is_some() {
                    continue;
                }
                #[cfg(feature = "rules")]
                if rules.held(token.raw, None, index)?.is_some() {
                    continue;
                }
                #[cfg(feature = "tables")]
                if tables.held(token.raw, None, index)?.is_some() {
                    continue;
                }
                if let Some(buffer) = cols.as_mut() {
                    buffer.push(token.raw);
                } else if let Some(buffer) = merges.as_mut() {
                    buffer.content.extend_from_slice(token.raw);
                } else if !matches!(cell, Some((_, InCell::Replace { .. }))) {
                    out.write_all(token.raw)?;
                }
            }
        }
    }
    // A set, or a row format that writes, that found no `<sheetData>` to go
    // into (P1 ruling 2 of `specs/0011-editing-cells.md`); a clear alone has
    // nothing to remove there.
    if pending.opens_rows() {
        return Err(NO_SHEET_DATA);
    }
    // An unmerge the file's merges did not hold (`specs/0018-rows-and-
    // columns.md`, P1 ruling 5).
    if unmerged.iter().any(|found| !found) {
        return Err(NO_SUCH_MERGE);
    }
    // A removal of a link the part does not have (`specs/0019-names-links-
    // properties-protection.md`, "Hyperlinks").
    #[cfg(feature = "metadata")]
    links.finish(work)?;
    // A removal of a validation the part does not have (`specs/0020-
    // validation-and-conditional-formatting.md`, "Data validation").
    #[cfg(feature = "rules")]
    rules.finish()?;
    // A removal of a conditional format the part does not have ("Conditional
    // formatting").
    #[cfg(feature = "rules")]
    formats.finish()?;
    let (temp, replacement) =
        written_part(temp, out.into_inner().map_err(|e| Error::Io(e.into()))?)?;
    Ok((temp, replacement, removed_formula))
}

/// A `<cols>` with the column formats applied: its start tag (opened up
/// when it was empty), the children [`cols_content`] gives and its end tag,
/// or nothing when no `<col>` is left.
fn cols_written(
    buffer: &ColsBuffer,
    formats: &[(u32, LineFormat)],
    index: usize,
) -> Result<Vec<u8>, Error> {
    let (content, any) = cols_content(&buffer.items, formats, &buffer.prefix, index)?;
    if !any {
        return Ok(Vec::new());
    }
    let mut out = if buffer.empty {
        opened_up(&buffer.start, index)?
    } else {
        buffer.start.clone()
    };
    out.extend_from_slice(&content);
    out.extend_from_slice(&[b"</".as_slice(), &buffer.name, b">"].concat());
    Ok(out)
}

/// Whether a date property's value is exactly `YYYY-MM-DDThh:mm:ssZ`: a
/// digit in every digit position and the other characters as written
/// (`specs/0019-names-links-properties-protection.md`, "Document
/// properties").
#[cfg(feature = "metadata")]
fn w3cdtf_ok(value: &str) -> bool {
    const FORM: &[u8; 20] = b"0000-00-00T00:00:00Z";
    let bytes = value.as_bytes();
    bytes.len() == FORM.len()
        && bytes
            .iter()
            .zip(FORM.iter())
            .all(|(byte, form)| match form {
                b'0' => byte.is_ascii_digit(),
                _ => byte == form,
            })
}

/// Reads a core-properties part: each child of the root that is one of the
/// thirteen properties, its text; a second element of one property is
/// refused.
#[cfg(feature = "metadata")]
fn read_core(archive: &mut Core, index: usize) -> Result<Properties, Error> {
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut values: Vec<(Property, String)> = Vec::new();
    while let Some(token) = stream.next()? {
        let (start, empty) = match token.event {
            Event::Start(ref start) => (start, false),
            Event::Empty(ref start) => (start, true),
            Event::End(_) => {
                scope.end();
                depth = depth.saturating_sub(1);
                continue;
            }
            _ => continue,
        };
        depth += 1;
        scope.start(start)?;
        let (namespace, local) = scope.resolve_element(start.name())?;
        if depth == 2
            && let Some(property) = Property::of(namespace, local)
        {
            if values.iter().any(|(have, _)| *have == property) {
                return Err(CORE_TWO);
            }
            let text = if empty {
                String::new()
            } else {
                let text = crate::cells::text_of(&mut stream, &mut scope, index)?;
                depth -= 1;
                values.push((property, text));
                continue;
            };
            values.push((property, text));
        }
        if empty {
            scope.end();
            depth -= 1;
        }
    }
    Ok(Properties { values })
}

/// The prefix a start tag declares for `namespace`: `Some(b"")` for the
/// default namespace, `Some(p)` for `xmlns:p`, `None` when it declares none.
#[cfg(feature = "metadata")]
fn declared_prefix(
    start: &BytesStart<'_>,
    namespace: &[u8],
    index: usize,
) -> Result<Option<Vec<u8>>, Error> {
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.value.as_bytes() != namespace {
            continue;
        }
        let key = attribute.key.as_ref().as_bytes();
        if key == b"xmlns" {
            return Ok(Some(Vec::new()));
        }
        if let Some(prefix) = key.strip_prefix(b"xmlns:") {
            return Ok(Some(prefix.to_vec()));
        }
    }
    Ok(None)
}

/// One core property element: `<p:name>text</p:name>`, a date with
/// `xsi:type="dcterms:W3CDTF"`, `prefix` and the others with their colons,
/// or bare when empty.
#[cfg(feature = "metadata")]
fn core_element(
    property: Property,
    text: &str,
    prefix: &[u8],
    xsi: &[u8],
    dcterms: &[u8],
) -> Vec<u8> {
    let colon = |p: &[u8]| -> Vec<u8> {
        if p.is_empty() {
            Vec::new()
        } else {
            [p, b":"].concat()
        }
    };
    let name = [colon(prefix).as_slice(), property.element().1].concat();
    let mut out = [b"<".as_slice(), &name].concat();
    if property.dated() {
        out.extend_from_slice(b" ");
        out.extend_from_slice(&colon(xsi));
        out.extend_from_slice(b"type=\"");
        out.extend_from_slice(&colon(dcterms));
        out.extend_from_slice(b"W3CDTF\"");
    }
    out.push(b'>');
    out.extend_from_slice(escape(text).replace('\r', "&#13;").as_bytes());
    out.extend_from_slice(&[b"</".as_slice(), &name, b">"].concat());
    out
}

/// The core properties this write sets or removes, each property's last
/// record, in the order of those records.
#[cfg(feature = "metadata")]
fn property_edits(recorded: &[(Property, Option<String>)]) -> Vec<(Property, Option<String>)> {
    recorded
        .iter()
        .enumerate()
        .filter(|(at, (property, _))| {
            !recorded[at + 1..]
                .iter()
                .any(|(later, _)| later == property)
        })
        .map(|(_, edit)| edit.clone())
        .collect()
}

/// A new `docProps/core.xml` holding the set properties, with the fixed
/// root binding `cp`, `dc`, `dcterms`, `dcmitype` and `xsi`, the prefixes
/// Excel writes (decision 5 A): the declaration, one line feed and the root,
/// as the template's parts are.
#[cfg(feature = "metadata")]
fn new_core_part(edits: &[(Property, Option<String>)]) -> Vec<u8> {
    let mut out = b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<cp:coreProperties xmlns:cp=\"".to_vec();
    out.extend_from_slice(CP_NS);
    out.extend_from_slice(b"\" xmlns:dc=\"");
    out.extend_from_slice(DC_NS);
    out.extend_from_slice(b"\" xmlns:dcterms=\"");
    out.extend_from_slice(DCTERMS_NS);
    out.extend_from_slice(b"\" xmlns:dcmitype=\"");
    out.extend_from_slice(DCMITYPE_NS);
    out.extend_from_slice(b"\" xmlns:xsi=\"");
    out.extend_from_slice(XSI_NS);
    out.extend_from_slice(b"\">");
    for (property, value) in edits {
        if let Some(text) = value {
            let prefix: &[u8] = match property.element().0 {
                ns if ns == DC_NS => b"dc",
                ns if ns == DCTERMS_NS => b"dcterms",
                _ => b"cp",
            };
            out.extend_from_slice(&core_element(*property, text, prefix, b"xsi", b"dcterms"));
        }
    }
    out.extend_from_slice(b"</cp:coreProperties>");
    out
}

/// Rewrites a core-properties part with the property edits: the root's
/// start tag copied raw; a child that is an edited property replaced (its
/// start tag kept, opened up when empty, its content the new text) or
/// skipped whole; every other token copied; each set property the part
/// lacks appended before the root's end tag with the prefix the root binds
/// to its namespace, refused when it binds none
/// (`specs/0019-names-links-properties-protection.md`, "Document
/// properties").
#[cfg(feature = "metadata")]
fn rewrite_core(
    archive: &mut Core,
    index: usize,
    edits: &[(Property, Option<String>)],
) -> Result<(TempFile, Replacement), Error> {
    let (temp, file) = temp_for_part()?;
    let mut out = BufWriter::new(file);
    let mut stream = Stream::new(archive.read(index)?, index)?;
    if stream.bom() {
        out.write_all(&BYTE_ORDER_MARK)?;
    }
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut met: Vec<Property> = Vec::new();
    // The prefixes the root binds, for the appended elements.
    let mut prefixes: [Option<Vec<u8>>; 4] = [None, None, None, None];
    let appended = |prefixes: &[Option<Vec<u8>>; 4], met: &[Property]| -> Result<Vec<u8>, Error> {
        let mut out = Vec::new();
        for (property, value) in edits {
            let Some(text) = value else {
                continue;
            };
            if met.contains(property) {
                continue;
            }
            let ns = property.element().0;
            let which = if ns == CP_NS {
                0
            } else if ns == DC_NS {
                1
            } else {
                2
            };
            let prefix = prefixes[which].as_ref().ok_or(CORE_NO_PREFIX)?;
            // `xsi` bound only as the default namespace gives no prefix an
            // attribute can take, so an appended date is refused as for no
            // binding (P3 ruling 6).
            let (xsi, dcterms): (&[u8], &[u8]) = if property.dated() {
                (
                    prefixes[3]
                        .as_deref()
                        .filter(|prefix| !prefix.is_empty())
                        .ok_or(CORE_NO_PREFIX)?,
                    prefixes[2].as_deref().ok_or(CORE_NO_PREFIX)?,
                )
            } else {
                (b"", b"")
            };
            out.extend_from_slice(&core_element(*property, text, prefix, xsi, dcterms));
        }
        Ok(out)
    };
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                if depth == 1 {
                    for (at, ns) in [CP_NS, DC_NS, DCTERMS_NS, XSI_NS].into_iter().enumerate() {
                        prefixes[at] = declared_prefix(start, ns, index)?;
                    }
                    if empty {
                        out.write_all(&opened_up(token.raw, index)?)?;
                        out.write_all(&appended(&prefixes, &met)?)?;
                        out.write_all(b"</")?;
                        out.write_all(start.name().into_inner().as_bytes())?;
                        out.write_all(b">")?;
                    } else {
                        out.write_all(token.raw)?;
                    }
                } else if depth == 2
                    && let Some(property) = Property::of(namespace, local)
                {
                    if met.contains(&property) {
                        return Err(CORE_TWO);
                    }
                    met.push(property);
                    let name = start.name().into_inner().as_bytes().to_vec();
                    match edits.iter().find(|(edited, _)| *edited == property) {
                        None => out.write_all(token.raw)?,
                        Some((_, value)) => {
                            let raw = token.raw.to_vec();
                            if !empty {
                                skip_element(&mut stream, &mut scope)?;
                            }
                            if let Some(text) = value {
                                if empty {
                                    out.write_all(&opened_up(&raw, index)?)?;
                                } else {
                                    out.write_all(&raw)?;
                                }
                                out.write_all(
                                    escape(text.as_str()).replace('\r', "&#13;").as_bytes(),
                                )?;
                                out.write_all(b"</")?;
                                out.write_all(&name)?;
                                out.write_all(b">")?;
                            }
                            scope.end();
                            depth -= 1;
                            continue;
                        }
                    }
                } else {
                    out.write_all(token.raw)?;
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                if depth == 1 {
                    out.write_all(&appended(&prefixes, &met)?)?;
                }
                out.write_all(token.raw)?;
                scope.end();
                depth -= 1;
            }
            _ => out.write_all(token.raw)?,
        }
    }
    written_part(temp, out.into_inner().map_err(|e| Error::Io(e.into()))?)
}

/// The `<sheetProtection>` part of a worksheet rewrite
/// (`specs/0019-names-links-properties-protection.md`, "Protection"): the
/// part's element replaced whole or dropped, or one created in schema
/// position. The Cargo feature `metadata`.
#[cfg(feature = "metadata")]
struct ProtectPass {
    edit: Option<Option<SheetLock>>,
    met: bool,
}

#[cfg(feature = "metadata")]
impl ProtectPass {
    /// The element's place in [`WORKSHEET_CHILDREN`].
    fn position() -> usize {
        schema_position(b"sheetProtection").unwrap_or(0)
    }

    /// The element the edit writes, with `prefix`, or nothing for an
    /// unprotect.
    fn element(&self, prefix: &[u8]) -> Vec<u8> {
        match self.edit {
            Some(Some((hash, options))) => sheet_protection_element(hash, &options, prefix),
            _ => Vec::new(),
        }
    }

    /// The element the write creates, when it protects a part that lacks
    /// one and a root child at `position` comes after it.
    fn create_before(&mut self, position: usize, prefix: &[u8]) -> Option<Vec<u8>> {
        if matches!(self.edit, Some(Some(_))) && !self.met && Self::position() < position {
            self.met = true;
            return Some(self.element(prefix));
        }
        None
    }
}

/// A `<sheetProtection>` element: `password` first when a hash is given,
/// then, in the schema's order, `name="1"` for each option locked that the
/// schema leaves unlocked by default and `name="0"` for each unlocked that
/// it locks by default, so the default options write
/// `<sheetProtection sheet="1"/>`.
#[cfg(feature = "metadata")]
fn sheet_protection_element(
    hash: Option<u16>,
    options: &SheetProtection,
    prefix: &[u8],
) -> Vec<u8> {
    let mut out = [b"<".as_slice(), prefix, b"sheetProtection"].concat();
    if let Some(hash) = hash {
        out.extend_from_slice(format!(" password=\"{hash:04X}\"").as_bytes());
    }
    for (name, locked, default) in options.options() {
        if locked != default {
            out.push(b' ');
            out.extend_from_slice(name);
            out.extend_from_slice(if locked { b"=\"1\"" } else { b"=\"0\"" });
        }
    }
    out.extend_from_slice(b"/>");
    out
}

/// The settings of a worksheet's first `<sheetView>` that
/// [`Package::set_sheet_view`] writes, each `Some` field as its attribute
/// and each `None` field leaving the file's as it is
/// (`specs/0025-sheet-and-print-settings.md`, "Types"). Which sheet is
/// selected (`tabSelected`) and which window the view belongs to
/// (`workbookViewId`) are the workbook's and not modelled. The Cargo
/// feature `sheetprops`.
#[cfg(feature = "sheetprops")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SheetView {
    /// `zoomScale`, a percentage in `10..=400`.
    pub zoom: Option<u32>,
    /// `showGridLines`.
    pub show_grid_lines: Option<bool>,
    /// `showRowColHeaders`: the row and column headings.
    pub show_headings: Option<bool>,
    /// `showZeros`.
    pub show_zeros: Option<bool>,
    /// `showFormulas`: formulas shown in place of their values.
    pub show_formulas: Option<bool>,
    /// `rightToLeft`.
    pub right_to_left: Option<bool>,
    /// `view`.
    pub view: Option<ViewKind>,
}

/// A sheet view's kind, the `view` attribute's three values. The Cargo
/// feature `sheetprops`.
#[cfg(feature = "sheetprops")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ViewKind {
    /// `normal`.
    #[default]
    Normal,
    /// `pageBreakPreview`.
    PageBreakPreview,
    /// `pageLayout`.
    PageLayout,
}

#[cfg(feature = "sheetprops")]
impl ViewKind {
    fn keyword(self) -> &'static [u8] {
        match self {
            Self::Normal => b"normal",
            Self::PageBreakPreview => b"pageBreakPreview",
            Self::PageLayout => b"pageLayout",
        }
    }
}

#[cfg(feature = "sheetprops")]
impl SheetView {
    /// These fields with each `Some` field of `later` in its place.
    fn then(self, later: &SheetView) -> SheetView {
        SheetView {
            zoom: later.zoom.or(self.zoom),
            show_grid_lines: later.show_grid_lines.or(self.show_grid_lines),
            show_headings: later.show_headings.or(self.show_headings),
            show_zeros: later.show_zeros.or(self.show_zeros),
            show_formulas: later.show_formulas.or(self.show_formulas),
            right_to_left: later.right_to_left.or(self.right_to_left),
            view: later.view.or(self.view),
        }
    }

    /// The attributes the `Some` fields write, in the order ECMA-376 Part
    /// 1, 18.3.1.87 gives them, a flag as `1` or `0`.
    fn attributes(&self) -> Vec<(&'static [u8], Vec<u8>)> {
        let flag = |on: bool| if on { b"1".to_vec() } else { b"0".to_vec() };
        let mut out: Vec<(&'static [u8], Vec<u8>)> = Vec::new();
        for (name, value) in [
            (b"showFormulas".as_slice(), self.show_formulas),
            (b"showGridLines", self.show_grid_lines),
            (b"showRowColHeaders", self.show_headings),
            (b"showZeros", self.show_zeros),
            (b"rightToLeft", self.right_to_left),
        ] {
            if let Some(on) = value {
                out.push((name, flag(on)));
            }
        }
        if let Some(view) = self.view {
            out.push((b"view", view.keyword().to_vec()));
        }
        if let Some(zoom) = self.zoom {
            out.push((b"zoomScale", zoom.to_string().into_bytes()));
        }
        out
    }
}

/// One worksheet's sheet settings in one write
/// (`specs/0025-sheet-and-print-settings.md`, "The pass"). The Cargo feature
/// `sheetprops`.
#[cfg(feature = "sheetprops")]
#[derive(Clone, Copy, Default)]
struct Settings {
    /// `Some(Some(colour))` a tab colour, `Some(None)` its removal, `None`
    /// no tab colour edit.
    tab: Option<Option<Color>>,
    /// The first `<sheetView>`'s fields, `None` when no call names them.
    view: Option<SheetView>,
}

#[cfg(all(feature = "sheetprops", feature = "rowcol"))]
impl Settings {
    fn is_empty(&self) -> bool {
        self.tab.is_none() && self.view.is_none()
    }
}

/// `<sheetPr>`'s name, and its children in the order ECMA-376 Part 1,
/// 18.3.1.82 gives them.
#[cfg(feature = "sheetprops")]
const SHEET_PR: &[u8] = b"sheetPr";
#[cfg(feature = "sheetprops")]
const SHEET_PR_CHILDREN: [&[u8]; 3] = [b"tabColor", b"outlinePr", b"pageSetUpPr"];

/// The one attribute-merge routine of the settings pass
/// (`specs/0025-sheet-and-print-settings.md`, "The pass", decision 2 A): a
/// start tag with each of `set` written first, in the order given, which is
/// the schema's, and every other attribute after them, each attribute's
/// bytes kept and the attributes in their order, one space before each and
/// none before the tag's end; an attribute is one of `set`'s when its
/// qualified name as written is one of `set`'s names, which carry no
/// prefix, so a prefixed attribute of the same local name is another
/// attribute and is kept (`specs/0025-sheet-and-print-settings.md`, P1
/// verification ruling 2). The element's name and whether its tag is empty
/// are kept, and with nothing to set the tag is returned as it is.
#[cfg(feature = "sheetprops")]
fn merged_tag(raw: &[u8], set: &[(&[u8], Vec<u8>)], index: usize) -> Result<Vec<u8>, Error> {
    if set.is_empty() {
        return Ok(raw.to_vec());
    }
    let tag = scan_start_tag(raw).ok_or(Error::Xml(index))?;
    let mut out = raw[..tag.name_end].to_vec();
    for (name, value) in set {
        out.push(b' ');
        out.extend_from_slice(name);
        out.extend_from_slice(b"=\"");
        out.extend_from_slice(value);
        out.push(b'"');
    }
    for attribute in &tag.attributes {
        let name = &raw[attribute.name.clone()];
        if set.iter().any(|(given, _)| *given == name) {
            continue;
        }
        out.push(b' ');
        // The closing quote follows the value.
        out.extend_from_slice(&raw[attribute.name.start..=attribute.value.end]);
    }
    out.extend_from_slice(if raw.ends_with(b"/>") {
        b"/>".as_slice()
    } else {
        b">"
    });
    Ok(out)
}

/// A first `<sheetView>`'s start tag with the part's view settings merged.
#[cfg(feature = "sheetprops")]
fn view_tag(work: &SheetWork, raw: &[u8], index: usize) -> Result<Vec<u8>, Error> {
    match &work.settings.view {
        Some(view) => merged_tag(raw, &view.attributes(), index),
        None => Ok(raw.to_vec()),
    }
}

/// A first `<sheetView>`'s start tag as it is, without the feature
/// `sheetprops`.
#[cfg(not(feature = "sheetprops"))]
fn view_tag(_: &SheetWork, raw: &[u8], _: usize) -> Result<Vec<u8>, Error> {
    Ok(raw.to_vec())
}

/// Whether the write edits the part's first `<sheetView>`'s attributes, and
/// whether it sets any, which creates a `<sheetViews>` the part lacks.
#[cfg(feature = "sheetprops")]
fn view_edit(work: &SheetWork) -> (bool, bool) {
    match &work.settings.view {
        Some(view) => (true, !view.attributes().is_empty()),
        None => (false, false),
    }
}

/// No view edit, without the feature `sheetprops`.
#[cfg(not(feature = "sheetprops"))]
fn view_edit(_: &SheetWork) -> (bool, bool) {
    (false, false)
}

/// An element the settings pass holds to its end tag: its start tag,
/// whether that tag was empty, its qualified name, its prefix and its
/// children so far. The Cargo feature `sheetprops`.
#[cfg(feature = "sheetprops")]
struct Held {
    depth: usize,
    start: Vec<u8>,
    empty: bool,
    name: Vec<u8>,
    prefix: Vec<u8>,
    children: Vec<HeldChild>,
}

/// One piece of a held element's content.
#[cfg(feature = "sheetprops")]
enum HeldChild {
    /// A child element of the main namespace: its local name and its bytes,
    /// start tag to end tag.
    Element { local: Vec<u8>, bytes: Vec<u8> },
    /// Anything else: text between the children, a comment, an element of
    /// another namespace with its content.
    Other(Vec<u8>),
}

#[cfg(feature = "sheetprops")]
impl HeldChild {
    fn bytes(&mut self) -> &mut Vec<u8> {
        match self {
            Self::Element { bytes, .. } | Self::Other(bytes) => bytes,
        }
    }

    fn is(&self, name: &[u8]) -> bool {
        matches!(self, Self::Element { local, .. } if local.as_slice() == name)
    }
}

/// The settings pass of a worksheet rewrite
/// (`specs/0025-sheet-and-print-settings.md`, "The pass"): the part's
/// `<sheetPr>` held to its end tag and rewritten, or created in schema
/// position with the root's prefix, and the refusal of one met twice or out
/// of schema order. The first `<sheetView>`'s attributes go through the same
/// merge routine where the rewrite meets that element ([`view_tag`]). The
/// Cargo feature `sheetprops`.
#[cfg(feature = "sheetprops")]
struct SettingsPass<'w> {
    work: &'w Settings,
    /// Whether the part's `<sheetPr>` has been met or created.
    sheet_pr_met: bool,
    /// The greatest place in [`WORKSHEET_CHILDREN`] of a root child met.
    last: Option<usize>,
    /// The `<sheetPr>` being read.
    held: Option<Held>,
}

#[cfg(feature = "sheetprops")]
impl<'w> SettingsPass<'w> {
    fn new(work: &'w Settings) -> Self {
        Self {
            work,
            sheet_pr_met: false,
            last: None,
            held: None,
        }
    }

    /// Whether the write edits the part's `<sheetPr>`.
    fn touches_sheet_pr(&self) -> bool {
        self.work.tab.is_some()
    }

    /// Whether the write creates a `<sheetPr>` the part lacks.
    fn needs_sheet_pr(&self) -> bool {
        matches!(self.work.tab, Some(Some(_)))
    }

    /// A root child of the main namespace met at `position` in
    /// [`WORKSHEET_CHILDREN`]: a `<sheetPr>` the write edits is refused when
    /// it is the second, or comes after a root child the schema puts after
    /// it, which is also the case of one met after the write created one.
    fn observe(&mut self, local: &[u8], position: usize) -> Result<(), Error> {
        if local == SHEET_PR && self.touches_sheet_pr() {
            if self.sheet_pr_met || self.last.is_some_and(|last| last > position) {
                return Err(TWO_SETTINGS);
            }
            self.sheet_pr_met = true;
        }
        self.last = Some(self.last.map_or(position, |last| last.max(position)));
        Ok(())
    }

    /// The elements the write creates before a root child at `position`,
    /// or at the root's end for `usize::MAX`, with the root's `prefix`.
    fn create_before(
        &mut self,
        position: usize,
        prefix: &[u8],
        index: usize,
    ) -> Result<Vec<u8>, Error> {
        let at = schema_position(SHEET_PR).unwrap_or(0);
        if self.needs_sheet_pr() && !self.sheet_pr_met && at < position {
            self.sheet_pr_met = true;
            let name = [prefix, SHEET_PR].concat();
            let held = Held {
                depth: 0,
                start: [b"<".as_slice(), &name, b"/>"].concat(),
                empty: true,
                name,
                prefix: prefix.to_vec(),
                children: Vec::new(),
            };
            return self.sheet_pr_written(held, index);
        }
        Ok(Vec::new())
    }

    /// A token inside the held `<sheetPr>` at `depth`, a start or empty tag
    /// of local name `local`: a child of its own one deeper, or more of the
    /// child it is inside.
    fn child(&mut self, raw: &[u8], local: &[u8], ours: bool, depth: usize) {
        if let Some(held) = self.held.as_mut() {
            if depth == held.depth + 1 {
                held.children.push(if ours {
                    HeldChild::Element {
                        local: local.to_vec(),
                        bytes: raw.to_vec(),
                    }
                } else {
                    HeldChild::Other(raw.to_vec())
                });
            } else if let Some(last) = held.children.last_mut() {
                last.bytes().extend_from_slice(raw);
            }
        }
    }

    /// Any token other than a tag while a `<sheetPr>` is held: text between
    /// its children is a piece of its own, anything deeper belongs to the
    /// child it is in. Whether the pass took it.
    fn other(&mut self, raw: &[u8], depth: usize) -> bool {
        let Some(held) = self.held.as_mut() else {
            return false;
        };
        match held.children.last_mut() {
            Some(last) if depth > held.depth => last.bytes().extend_from_slice(raw),
            _ => held.children.push(HeldChild::Other(raw.to_vec())),
        }
        true
    }

    /// An end tag at `depth` while a `<sheetPr>` is held: the rewritten
    /// element at its own end, nothing at a child's, which joins the child;
    /// `None` when nothing is held.
    fn end(&mut self, raw: &[u8], depth: usize, index: usize) -> Result<Option<Vec<u8>>, Error> {
        let Some(held) = self.held.as_mut() else {
            return Ok(None);
        };
        if depth == held.depth {
            return match self.held.take() {
                Some(held) => self.sheet_pr_written(held, index).map(Some),
                None => Ok(None),
            };
        }
        match held.children.last_mut() {
            Some(last) => last.bytes().extend_from_slice(raw),
            None => held.children.push(HeldChild::Other(raw.to_vec())),
        }
        Ok(Some(Vec::new()))
    }

    /// A held `<sheetPr>` with the tab colour edit applied: the part's
    /// `<tabColor>` dropped, the edit's placed before the first child the
    /// schema puts after it, every other attribute and child kept; nothing
    /// when the element is left with no attribute and no child.
    fn sheet_pr_written(&self, held: Held, index: usize) -> Result<Vec<u8>, Error> {
        let mut children = held.children;
        if self.work.tab.is_some() {
            children.retain(|child| !child.is(SHEET_PR_CHILDREN[0]));
        }
        if let Some(Some(color)) = &self.work.tab {
            let at = children
                .iter()
                .position(|child| SHEET_PR_CHILDREN[1..].iter().any(|name| child.is(name)))
                .unwrap_or(children.len());
            children.insert(
                at,
                HeldChild::Element {
                    local: SHEET_PR_CHILDREN[0].to_vec(),
                    bytes: color_element("tabColor", color).element(&held.prefix),
                },
            );
        }
        let tag = scan_start_tag(&held.start).ok_or(Error::Xml(index))?;
        let content = children.iter().any(|child| match child {
            HeldChild::Other(bytes) => !bytes.iter().all(u8::is_ascii_whitespace),
            HeldChild::Element { .. } => true,
        });
        if tag.attributes.is_empty() && !content {
            return Ok(Vec::new());
        }
        if children.is_empty() {
            let mut out = held.start;
            if !held.empty {
                out.extend_from_slice(&[b"</".as_slice(), &held.name, b">"].concat());
            }
            return Ok(out);
        }
        let mut out = if held.empty {
            opened_up(&held.start, index)?
        } else {
            held.start
        };
        for mut child in children {
            out.append(child.bytes());
        }
        out.extend_from_slice(&[b"</".as_slice(), &held.name, b">"].concat());
        Ok(out)
    }
}

/// The hyperlinks' part of a worksheet rewrite
/// (`specs/0019-names-links-properties-protection.md`, "Hyperlinks"): which
/// links the part has are skipped, which are appended, the `<hyperlinks>`
/// held while it is read, and the prefix `r:id` takes. The Cargo feature
/// `metadata`.
#[cfg(feature = "metadata")]
struct LinksPass<'w> {
    links: &'w [(CellRef, LinkEdit)],
    /// Whether the write edits the part's links, sets any, and has met or
    /// created a `<hyperlinks>`.
    touched: bool,
    needed: bool,
    met: bool,
    /// The prefix the root binds to the relationships namespace, or the one
    /// its retag binds; empty until the root is read.
    rels_prefix: Vec<u8>,
    buffer: Option<HyperlinksBuffer>,
    /// For each edit, whether the part had a link on its cell.
    found: Vec<bool>,
    /// The `r:id`s of the links skipped.
    dropped: Vec<Vec<u8>>,
    /// The `r:id`s of the links kept, whose relationships stay (P2
    /// verification ruling 6).
    kept_ids: Vec<Vec<u8>>,
}

/// A `<hyperlinks>` being read: its start tag, its prefix and the bytes of
/// the children it keeps, with how many `<hyperlink>` elements those are.
#[cfg(feature = "metadata")]
struct HyperlinksBuffer {
    depth: usize,
    start: Vec<u8>,
    empty: bool,
    name: Vec<u8>,
    prefix: Vec<u8>,
    content: Vec<u8>,
    kept: u64,
}

/// A root child's place in [`WORKSHEET_CHILDREN`] for `<hyperlinks>`.
#[cfg(feature = "metadata")]
const HYPERLINKS: &[u8] = b"hyperlinks";

#[cfg(feature = "metadata")]
impl<'w> LinksPass<'w> {
    fn new(links: &'w [(CellRef, LinkEdit)]) -> Self {
        LinksPass {
            links,
            touched: !links.is_empty(),
            needed: links
                .iter()
                .any(|(_, edit)| matches!(edit, LinkEdit::Set { .. })),
            met: false,
            rels_prefix: Vec::new(),
            buffer: None,
            found: vec![false; links.len()],
            dropped: Vec::new(),
            kept_ids: Vec::new(),
        }
    }

    /// The root's start tag as the rewrite writes it: as it is, or, when an
    /// external link is appended and the root binds no prefix to the
    /// relationships namespace, with one bound (`r`, or the first of `r1`,
    /// `r2` and so on that the tag does not declare), in the namespace form
    /// of the worksheet's own relationship (decision 3 A; P2 verification
    /// ruling 2).
    fn root_tag(
        &mut self,
        raw: &[u8],
        start: &BytesStart<'_>,
        strict: bool,
        index: usize,
    ) -> Result<Vec<u8>, Error> {
        if let Some(prefix) = relationships_prefix(start, index)? {
            self.rels_prefix = prefix;
            return Ok(raw.to_vec());
        }
        let external = self.links.iter().any(|(_, edit)| {
            matches!(
                edit,
                LinkEdit::Set {
                    rel_id: Some(_),
                    ..
                }
            )
        });
        if !external {
            return Ok(raw.to_vec());
        }
        let mut n = 0u32;
        let prefix = loop {
            let candidate = if n == 0 {
                b"r".to_vec()
            } else {
                format!("r{n}").into_bytes()
            };
            let declaration = [b"xmlns:".as_slice(), &candidate].concat();
            if attribute_value(raw, &declaration, index)?.is_none() {
                break candidate;
            }
            n = n.checked_add(1).ok_or(Error::Xml(index))?;
        };
        let namespace = RELATIONSHIPS_ATTRIBUTE[usize::from(strict)];
        let tag = with_attribute(
            raw,
            &[b"xmlns:".as_slice(), &prefix].concat(),
            namespace,
            index,
        )?;
        self.rels_prefix = prefix;
        Ok(tag)
    }

    /// The `<hyperlinks>` the write creates, when it needs one the part
    /// lacks and a root child at `position` comes after it in schema order.
    fn create_before(&mut self, position: usize, prefix: &[u8]) -> Option<Vec<u8>> {
        let at = schema_position(HYPERLINKS).unwrap_or(0);
        if self.needed && !self.met && at < position {
            self.met = true;
            return Some(self.container(prefix));
        }
        None
    }

    /// The `<hyperlinks>` the write creates at the root's end, when it needs
    /// one the part still lacks.
    fn create_at_end(&mut self, prefix: &[u8]) -> Option<Vec<u8>> {
        if self.needed && !self.met {
            self.met = true;
            return Some(self.container(prefix));
        }
        None
    }

    /// A new `<hyperlinks>` holding the sets, with `prefix`.
    fn container(&self, prefix: &[u8]) -> Vec<u8> {
        let mut out = [b"<".as_slice(), prefix, b"hyperlinks>"].concat();
        out.extend_from_slice(&self.set_elements(prefix));
        out.extend_from_slice(&[b"</".as_slice(), prefix, b"hyperlinks>"].concat());
        out
    }

    /// The sets as `<hyperlink>` elements, back to back, with `prefix`.
    fn set_elements(&self, prefix: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        for (cell, edit) in self.links {
            let LinkEdit::Set { set, rel_id } = edit else {
                continue;
            };
            out.extend_from_slice(&[b"<".as_slice(), prefix, b"hyperlink ref=\""].concat());
            out.extend_from_slice(cell.a1().as_bytes());
            out.push(b'"');
            match rel_id {
                Some(id) => {
                    out.push(b' ');
                    out.extend_from_slice(&self.rels_prefix);
                    out.extend_from_slice(b":id=\"");
                    out.extend_from_slice(id);
                    out.push(b'"');
                }
                None => {
                    out.extend_from_slice(b" location=\"");
                    out.extend_from_slice(&attribute_escaped(&set.text));
                    out.push(b'"');
                }
            }
            if let Some(tooltip) = &set.tooltip {
                out.extend_from_slice(b" tooltip=\"");
                out.extend_from_slice(&attribute_escaped(tooltip));
                out.push(b'"');
            }
            out.extend_from_slice(b"/>");
        }
        out
    }

    /// A `<hyperlinks>` start tag the part has: the second is refused; an
    /// empty one is written at once with the sets, or dropped when none is
    /// set; any other is held. Returns the bytes to write now.
    fn open(
        &mut self,
        raw: &[u8],
        start: &BytesStart<'_>,
        depth: usize,
        empty: bool,
        index: usize,
    ) -> Result<Vec<u8>, Error> {
        if self.met {
            return Err(TWO_SHEET_ELEMENTS);
        }
        self.met = true;
        let buffer = HyperlinksBuffer {
            depth,
            start: raw.to_vec(),
            empty,
            name: start.name().into_inner().as_bytes().to_vec(),
            prefix: prefix_of(start),
            content: Vec::new(),
            kept: 0,
        };
        if empty {
            return self.close(buffer, index);
        }
        self.buffer = Some(buffer);
        Ok(Vec::new())
    }

    /// A start or empty tag inside the held `<hyperlinks>`: a `<hyperlink>`
    /// whose `ref` is exactly an edited cell is to be skipped whole, its
    /// `r:id` kept for the removal of its relationship; anything else is
    /// held. Returns whether the caller skips the element.
    fn child(
        &mut self,
        token_raw: &[u8],
        start: &BytesStart<'_>,
        (local, ours, depth): (&[u8], bool, usize),
        scope: &Scope,
        index: usize,
    ) -> Result<bool, Error> {
        let Some(buffer) = self.buffer.as_mut() else {
            return Ok(false);
        };
        if depth == buffer.depth + 1 && ours && local == b"hyperlink" {
            let cell = attribute_value(token_raw, b"ref", index)?.and_then(CellRef::parse);
            let edited: Vec<usize> = self
                .links
                .iter()
                .enumerate()
                .filter(|(_, (at, _))| Some(*at) == cell)
                .map(|(which, _)| which)
                .collect();
            let mut ids: Vec<Vec<u8>> = Vec::new();
            for attribute in start.attributes() {
                let attribute = attribute.map_err(|_| Error::Xml(index))?;
                if attribute.key.as_namespace_binding().is_some() {
                    continue;
                }
                let (namespace, name) = scope.resolve_attribute(attribute.key)?;
                if name == b"id"
                    && namespace.is_some_and(|ns| RELATIONSHIPS_ATTRIBUTE.contains(&ns))
                {
                    ids.push(attribute.value.as_bytes().to_vec());
                }
            }
            if !edited.is_empty() {
                for which in edited {
                    self.found[which] = true;
                }
                self.dropped.extend(ids);
                return Ok(true);
            }
            self.kept_ids.extend(ids);
            buffer.kept += 1;
        }
        buffer.content.extend_from_slice(token_raw);
        Ok(false)
    }

    /// An end tag or any other token inside the held `<hyperlinks>`; at the
    /// container's own end tag, the bytes it is written as.
    fn held(
        &mut self,
        raw: &[u8],
        end_depth: Option<usize>,
        index: usize,
    ) -> Result<Option<Vec<u8>>, Error> {
        let Some(depth) = self.buffer.as_ref().map(|buffer| buffer.depth) else {
            return Ok(None);
        };
        if end_depth == Some(depth) {
            return match self.buffer.take() {
                Some(buffer) => Ok(Some(self.close(buffer, index)?)),
                None => Ok(None),
            };
        }
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.content.extend_from_slice(raw);
        }
        Ok(Some(Vec::new()))
    }

    /// The held container written: its start tag (opened up when it was
    /// empty), the kept children, the sets and its end tag; nothing when no
    /// link is left.
    fn close(&self, buffer: HyperlinksBuffer, index: usize) -> Result<Vec<u8>, Error> {
        let sets = self.set_elements(&buffer.prefix);
        if buffer.kept == 0 && sets.is_empty() {
            return Ok(Vec::new());
        }
        let mut out = if buffer.empty {
            opened_up(&buffer.start, index)?
        } else {
            buffer.start
        };
        out.extend_from_slice(&buffer.content);
        out.extend_from_slice(&sets);
        out.extend_from_slice(&[b"</".as_slice(), &buffer.name, b">"].concat());
        Ok(out)
    }

    /// After the pass: a removal the part had no link for is refused, and
    /// the dropped `r:id`s that no kept link also names are handed back
    /// (P2 verification ruling 6).
    fn finish(self, work: &SheetWork) -> Result<(), Error> {
        for (which, (_, edit)) in self.links.iter().enumerate() {
            if matches!(edit, LinkEdit::Remove) && !self.found[which] {
                return Err(NO_SUCH_LINK);
            }
        }
        let kept = self.kept_ids;
        work.dropped_ids
            .borrow_mut()
            .extend(self.dropped.into_iter().filter(|id| !kept.contains(id)));
        Ok(())
    }
}

/// A `sqref` list read into its areas, in order, each normalised: `None`
/// when it is empty or an item does not parse (`specs/0020-validation-and-
/// conditional-formatting.md`, "Data validation").
#[cfg(feature = "rules")]
fn parsed_list(value: &[u8]) -> Option<Vec<Area>> {
    let mut areas = Vec::new();
    for item in value
        .split(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
        .filter(|item| !item.is_empty())
    {
        areas.push(Area::parse(item)?);
    }
    if areas.is_empty() {
        return None;
    }
    Some(areas)
}

/// Whether an area of one list shares a cell with an area of the other.
#[cfg(feature = "rules")]
fn lists_overlap(one: &[Area], other: &[Area]) -> bool {
    one.iter().any(|a| other.iter().any(|b| a.overlaps(b)))
}

/// The `sqref` value for `areas`: each in `A1` form, space-separated.
#[cfg(feature = "rules")]
fn sqref_value(areas: &[Area]) -> Vec<u8> {
    let items: Vec<String> = areas.iter().map(Area::a1).collect();
    items.join(" ").into_bytes()
}

/// The `<dataValidations>` of a worksheet name, its place in
/// [`WORKSHEET_CHILDREN`].
#[cfg(feature = "rules")]
const DATA_VALIDATIONS: &[u8] = b"dataValidations";

/// The data validations' part of a worksheet rewrite
/// (`specs/0020-validation-and-conditional-formatting.md`, "Data
/// validation"): which validations the part has are skipped, which are
/// appended, and the `<dataValidations>` held while it is read, bounded by
/// that one element. The Cargo feature `rules`.
#[cfg(feature = "rules")]
struct ValidationsPass<'w> {
    edits: &'w [ValidationEdit],
    /// Whether the write edits the part's validations, adds any, and has
    /// met or created a `<dataValidations>`.
    touched: bool,
    needed: bool,
    met: bool,
    buffer: Option<ValidationsBuffer>,
    /// For each edit, whether a removal found the part's validation.
    found: Vec<bool>,
}

/// A `<dataValidations>` being read: its start tag, its prefix and the
/// bytes of the children it keeps, with how many `<dataValidation>`
/// elements those are.
#[cfg(feature = "rules")]
struct ValidationsBuffer {
    depth: usize,
    start: Vec<u8>,
    empty: bool,
    name: Vec<u8>,
    prefix: Vec<u8>,
    content: Vec<u8>,
    kept: u64,
}

#[cfg(feature = "rules")]
impl<'w> ValidationsPass<'w> {
    fn new(edits: &'w [ValidationEdit]) -> Self {
        ValidationsPass {
            edits,
            touched: !edits.is_empty(),
            needed: edits
                .iter()
                .any(|edit| matches!(edit, ValidationEdit::Add(_))),
            met: false,
            buffer: None,
            found: vec![false; edits.len()],
        }
    }

    /// The additions, in record order.
    fn additions(&self) -> impl Iterator<Item = &NewValidation> {
        self.edits.iter().filter_map(|edit| match edit {
            ValidationEdit::Add(added) => Some(added),
            ValidationEdit::Remove(_) => None,
        })
    }

    /// The additions as `<dataValidation>` elements, back to back, with
    /// `prefix`.
    fn elements(&self, prefix: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        for added in self.additions() {
            out.extend_from_slice(&[b"<".as_slice(), prefix, b"dataValidation"].concat());
            out.extend_from_slice(&added.attributes);
            out.extend_from_slice(b" sqref=\"");
            out.extend_from_slice(&sqref_value(&added.areas));
            out.extend_from_slice(b"\">");
            for (name, text) in &added.formulas {
                out.extend_from_slice(&[b"<".as_slice(), prefix, name, b">"].concat());
                out.extend_from_slice(text.as_bytes());
                out.extend_from_slice(&[b"</".as_slice(), prefix, name, b">"].concat());
            }
            out.extend_from_slice(&[b"</".as_slice(), prefix, b"dataValidation>"].concat());
        }
        out
    }

    /// The `<dataValidations>` the write creates, when it needs one the part
    /// lacks and a root child at `position` comes after it in schema order.
    fn create_before(&mut self, position: usize, prefix: &[u8]) -> Option<Vec<u8>> {
        let at = schema_position(DATA_VALIDATIONS).unwrap_or(0);
        if at < position {
            return self.create_at_end(prefix);
        }
        None
    }

    /// The `<dataValidations>` the write creates at the root's end, when it
    /// needs one the part still lacks.
    fn create_at_end(&mut self, prefix: &[u8]) -> Option<Vec<u8>> {
        if !self.needed || self.met {
            return None;
        }
        self.met = true;
        let count = self.additions().count();
        let mut out = [b"<".as_slice(), prefix, b"dataValidations count=\""].concat();
        out.extend_from_slice(count.to_string().as_bytes());
        out.extend_from_slice(b"\">");
        out.extend_from_slice(&self.elements(prefix));
        out.extend_from_slice(&[b"</".as_slice(), prefix, b"dataValidations>"].concat());
        Some(out)
    }

    /// A `<dataValidations>` start tag the part has: the second is refused;
    /// an empty one is written at once, or dropped when nothing is added;
    /// any other is held. Returns the bytes to write now.
    fn open(
        &mut self,
        raw: &[u8],
        start: &BytesStart<'_>,
        depth: usize,
        empty: bool,
        index: usize,
    ) -> Result<Vec<u8>, Error> {
        if self.met {
            return Err(TWO_VALIDATIONS);
        }
        self.met = true;
        let buffer = ValidationsBuffer {
            depth,
            start: raw.to_vec(),
            empty,
            name: start.name().into_inner().as_bytes().to_vec(),
            prefix: prefix_of(start),
            content: Vec::new(),
            kept: 0,
        };
        if empty {
            return self.close(buffer, index);
        }
        self.buffer = Some(buffer);
        Ok(Vec::new())
    }

    /// A start or empty tag inside the held `<dataValidations>`: a
    /// `<dataValidation>` whose parsed `sqref` a removal names is to be
    /// skipped whole; one kept is refused when an addition overlaps it, or
    /// when its `sqref` does not parse; anything else is held. Returns
    /// whether the caller skips the element.
    fn child(
        &mut self,
        raw: &[u8],
        (local, ours, depth): (&[u8], bool, usize),
        index: usize,
    ) -> Result<bool, Error> {
        let Some(at) = self.buffer.as_ref().map(|buffer| buffer.depth) else {
            return Ok(false);
        };
        let element = depth == at + 1 && ours && local == b"dataValidation";
        if element {
            let areas = attribute_value(raw, b"sqref", index)?
                .and_then(parsed_list)
                .ok_or(VALIDATION_OVERLAP)?;
            let mut gone = false;
            for (which, edit) in self.edits.iter().enumerate() {
                if matches!(edit, ValidationEdit::Remove(list) if *list == areas) {
                    self.found[which] = true;
                    gone = true;
                }
            }
            if gone {
                return Ok(true);
            }
            if self
                .additions()
                .any(|added| lists_overlap(&added.areas, &areas))
            {
                return Err(VALIDATION_OVERLAP);
            }
        }
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.kept += u64::from(element);
            buffer.content.extend_from_slice(raw);
        }
        Ok(false)
    }

    /// An end tag or any other token inside the held `<dataValidations>`;
    /// at the container's own end tag, the bytes it is written as.
    fn held(
        &mut self,
        raw: &[u8],
        end_depth: Option<usize>,
        index: usize,
    ) -> Result<Option<Vec<u8>>, Error> {
        let Some(depth) = self.buffer.as_ref().map(|buffer| buffer.depth) else {
            return Ok(None);
        };
        if end_depth == Some(depth) {
            return match self.buffer.take() {
                Some(buffer) => Ok(Some(self.close(buffer, index)?)),
                None => Ok(None),
            };
        }
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.content.extend_from_slice(raw);
        }
        Ok(Some(Vec::new()))
    }

    /// The held container written: its start tag with `count` set to the
    /// kept children and the additions (opened up when it was empty), the
    /// kept children, the additions and its end tag; nothing when no
    /// validation is left.
    fn close(&self, buffer: ValidationsBuffer, index: usize) -> Result<Vec<u8>, Error> {
        let count = buffer.kept + self.additions().count() as u64;
        if count == 0 {
            return Ok(Vec::new());
        }
        let start = with_attribute(&buffer.start, b"count", count.to_string().as_bytes(), index)?;
        let mut out = if buffer.empty {
            opened_up(&start, index)?
        } else {
            start
        };
        out.extend_from_slice(&buffer.content);
        out.extend_from_slice(&self.elements(&buffer.prefix));
        out.extend_from_slice(&[b"</".as_slice(), &buffer.name, b">"].concat());
        Ok(out)
    }

    /// After the pass: a removal the part had no validation for is refused.
    fn finish(&self) -> Result<(), Error> {
        for (which, edit) in self.edits.iter().enumerate() {
            if matches!(edit, ValidationEdit::Remove(_)) && !self.found[which] {
                return Err(NO_SUCH_VALIDATION);
            }
        }
        Ok(())
    }
}

/// Checks a table name against the rule of `specs/0021-tables.md`,
/// "Names": 1 to 255 characters; the first a letter, `_` or `\`; the rest
/// letters, digits, `.` or `_` ("letter" and "digit" by `char::is_alphabetic`
/// and `char::is_numeric`); not a cell reference on the grid nor of the
/// `R1C1` form, ignoring case (P1 rulings 4 and 5, P1 verification ruling
/// 4); and not `C`, `c`, `R` or `r` alone.
#[cfg(feature = "tables")]
fn check_table_name(name: &str) -> Result<(), Error> {
    let mut chars = name.chars();
    let first_ok = chars
        .next()
        .is_some_and(|first| first.is_alphabetic() || first == '_' || first == '\\');
    let rest_ok = chars.all(|c| c.is_alphabetic() || c.is_numeric() || matches!(c, '.' | '_'));
    // A reference past the grid, `TABLE1` among them, is a name Excel
    // takes, and its own default.
    if !first_ok
        || !rest_ok
        || name.chars().count() > 255
        || names_a_cell(name)
        || matches!(name, "C" | "c" | "R" | "r")
    {
        return Err(TABLE_NAME_INVALID);
    }
    Ok(())
}

/// The entry name `form` gives for the smallest positive N whose name,
/// ASCII case folded, `taken` does not hold; `taken` is folded already. A
/// list of k names leaves one of 1 to k + 1 free, so N runs no further.
#[cfg(any(feature = "tables", feature = "comments", feature = "images"))]
fn free_entry(taken: &[String], form: impl Fn(u64) -> String) -> String {
    let mut n = 1u64;
    while taken.contains(&form(n).to_ascii_lowercase()) {
        n += 1;
    }
    form(n)
}

/// The target of a relationship from the part `from` to the entry `to`,
/// relative to `from`'s directory: `../tables/table1.xml` from
/// `xl/worksheets/sheet1.xml` (P1 ruling 4).
#[cfg(any(feature = "tables", feature = "comments", feature = "images"))]
fn relative_target(from: &[u8], to: &[u8]) -> Vec<u8> {
    let from: Vec<&[u8]> = from.split(|byte| *byte == b'/').collect();
    let to: Vec<&[u8]> = to.split(|byte| *byte == b'/').collect();
    let directory = &from[..from.len() - 1];
    let common = directory
        .iter()
        .zip(&to[..to.len() - 1])
        .take_while(|(a, b)| a == b)
        .count();
    let mut out = b"../".repeat(directory.len() - common);
    out.extend_from_slice(&to[common..].join(b"/".as_slice()));
    out
}

/// A table part's bytes (`specs/0021-tables.md`, "The part"): the
/// declaration on its own line and the root on one, in the default
/// namespace, with no `xr` attribute.
#[cfg(feature = "tables")]
fn table_part_bytes(id: u64, table: &NewTable) -> Vec<u8> {
    let name = attribute_escaped(&table.name);
    let reference = table.area.a1();
    let mut out =
        b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<table xmlns=\"".to_vec();
    out.extend_from_slice(MAIN[0]);
    out.extend_from_slice(format!("\" id=\"{id}\" name=\"").as_bytes());
    out.extend_from_slice(&name);
    out.extend_from_slice(b"\" displayName=\"");
    out.extend_from_slice(&name);
    out.extend_from_slice(format!("\" ref=\"{reference}\" totalsRowShown=\"0\">").as_bytes());
    out.extend_from_slice(format!("<autoFilter ref=\"{reference}\"/>").as_bytes());
    out.extend_from_slice(format!("<tableColumns count=\"{}\">", table.columns.len()).as_bytes());
    for (id, column) in (1u64..).zip(&table.columns) {
        out.extend_from_slice(format!("<tableColumn id=\"{id}\" name=\"").as_bytes());
        out.extend_from_slice(&attribute_escaped(column));
        out.extend_from_slice(b"\"/>");
    }
    out.extend_from_slice(b"</tableColumns>");
    if let Some((style, info)) = &table.style {
        let flag = |on: bool| if on { "1" } else { "0" };
        out.extend_from_slice(
            format!(
                "<tableStyleInfo name=\"{style}\" showFirstColumn=\"{}\" \
                 showLastColumn=\"{}\" showRowStripes=\"{}\" showColumnStripes=\"{}\"/>",
                flag(info.first_column),
                flag(info.last_column),
                flag(info.row_stripes),
                flag(info.column_stripes),
            )
            .as_bytes(),
        );
    }
    out.extend_from_slice(b"</table>");
    out
}

/// What the write's checks read of a table part the workbook keeps: its
/// root's `id` when it is a decimal number, its `name` and `displayName`,
/// and its `ref` when it parses.
#[cfg(feature = "tables")]
struct TableRoot {
    id: Option<u64>,
    names: Vec<String>,
    area: Option<Area>,
}

/// An attribute's value, normalised as XML reads it.
#[cfg(feature = "tables")]
fn normalized_text(
    attribute: &quick_xml::events::attributes::Attribute<'_>,
    index: usize,
) -> Result<String, Error> {
    Ok(attribute
        .normalized_value(XmlVersion::Implicit1_0)
        .map_err(|_| Error::Xml(index))?
        .into_owned())
}

/// Reads a table part's root start tag, the part streamed no further
/// (`specs/0021-tables.md`, "32-bit and streaming").
#[cfg(feature = "tables")]
fn table_root(archive: &mut Core, index: usize) -> Result<TableRoot, Error> {
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut scope = Scope::new(index);
    let mut root = TableRoot {
        id: None,
        names: Vec::new(),
        area: None,
    };
    while let Some(token) = stream.next()? {
        let (Event::Start(ref start) | Event::Empty(ref start)) = token.event else {
            continue;
        };
        scope.start(start)?;
        for attribute in start.attributes() {
            let attribute = attribute.map_err(|_| Error::Xml(index))?;
            if attribute.key.as_namespace_binding().is_some() {
                continue;
            }
            let (namespace, local) = scope.resolve_attribute(attribute.key)?;
            if namespace.is_some() {
                continue;
            }
            match local {
                b"id" => {
                    root.id = str::from_utf8(attribute.value.as_bytes())
                        .ok()
                        .and_then(|text| text.parse().ok());
                }
                b"name" | b"displayName" => root.names.push(normalized_text(&attribute, index)?),
                b"ref" => root.area = Area::parse(attribute.value.as_bytes()),
                _ => {}
            }
        }
        break;
    }
    Ok(root)
}

/// The `name` of every `<definedName>` of the workbook part's
/// `<definedNames>`, normalised (P1 ruling 4: this feature's own reader).
#[cfg(feature = "tables")]
fn defined_name_values(input: impl Read, index: usize) -> Result<Vec<String>, Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut inside = false;
    let mut names = Vec::new();
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                if depth == 2 {
                    inside = ours && local == b"definedNames";
                } else if depth == 3 && inside && ours && local == b"definedName" {
                    for attribute in start.attributes() {
                        let attribute = attribute.map_err(|_| Error::Xml(index))?;
                        if attribute.key.as_ref().as_bytes() == b"name" {
                            names.push(normalized_text(&attribute, index)?);
                        }
                    }
                }
                if matches!(token.event, Event::Empty(_)) {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
    }
    Ok(names)
}

/// The `r:id` of each `<tablePart>` of the worksheet's `<tableParts>`, in
/// part order; a `tablePart` with none is refused with the read message.
#[cfg(feature = "tables")]
fn table_part_ids(input: impl Read, index: usize) -> Result<Vec<Vec<u8>>, Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut inside = false;
    let mut ids = Vec::new();
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                if depth == 2 {
                    inside = ours && local == TABLE_PARTS;
                } else if depth == 3 && inside && ours && local == b"tablePart" {
                    let mut id = None;
                    for attribute in start.attributes() {
                        let attribute = attribute.map_err(|_| Error::Xml(index))?;
                        if attribute.key.as_namespace_binding().is_some() {
                            continue;
                        }
                        let (namespace, name) = scope.resolve_attribute(attribute.key)?;
                        if name == b"id" && is_relationships_attribute(namespace) {
                            id = Some(attribute.value.as_bytes().to_vec());
                        }
                    }
                    ids.push(id.ok_or(TABLE_UNREADABLE)?);
                }
                if matches!(token.event, Event::Empty(_)) {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
    }
    Ok(ids)
}

/// Whether a table attribute's value is on: `1` or `true`.
#[cfg(feature = "tables")]
fn table_flag(value: &[u8]) -> bool {
    value == b"1" || value == b"true"
}

/// Reads one table part into its entry (`specs/0021-tables.md`, "The
/// calls"), streamed to the end of its `<tableStyleInfo>` or of its root.
#[cfg(feature = "tables")]
fn read_table(input: impl Read, index: usize, part: Vec<u8>) -> Result<TableEntry, Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let (mut id, mut name, mut display_name, mut area) = (false, None, None, None);
    let (mut header_row, mut totals_row, mut calculated) = (true, false, false);
    let mut columns = Vec::new();
    let mut style = None;
    let mut in_columns = false;
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                let mut attributes: Vec<(Vec<u8>, Vec<u8>, String)> = Vec::new();
                for attribute in start.attributes() {
                    let attribute = attribute.map_err(|_| Error::Xml(index))?;
                    if attribute.key.as_namespace_binding().is_some() {
                        continue;
                    }
                    let (namespace, local) = scope.resolve_attribute(attribute.key)?;
                    if namespace.is_none() && matches!(depth, 1..=3) {
                        let text = normalized_text(&attribute, index)?;
                        attributes.push((
                            local.to_vec(),
                            attribute.value.as_bytes().to_vec(),
                            text,
                        ));
                    }
                }
                let value = |name: &[u8]| attributes.iter().find(|(key, _, _)| key == name);
                if depth == 1 {
                    id = value(b"id").is_some();
                    name = value(b"name").map(|(_, _, text)| text.clone());
                    display_name = value(b"displayName").map(|(_, _, text)| text.clone());
                    area = value(b"ref").and_then(|(_, raw, _)| Area::parse(raw));
                    header_row = value(b"headerRowCount").is_none_or(|(_, raw, _)| raw != b"0");
                    totals_row = value(b"totalsRowCount").is_some_and(|(_, raw, _)| raw != b"0");
                } else if depth == 2 && ours {
                    in_columns = local == b"tableColumns";
                    if local == b"tableStyleInfo" {
                        let on = |key: &[u8]| value(key).is_some_and(|(_, raw, _)| table_flag(raw));
                        style = Some((
                            value(b"name")
                                .map(|(_, _, text)| text.clone())
                                .unwrap_or_default(),
                            on(b"showFirstColumn"),
                            on(b"showLastColumn"),
                            on(b"showRowStripes"),
                            on(b"showColumnStripes"),
                        ));
                        break;
                    }
                } else if depth == 3 && in_columns && ours && local == b"tableColumn" {
                    columns.push(
                        value(b"name")
                            .map(|(_, _, text)| text.clone())
                            .unwrap_or_default(),
                    );
                } else if depth == 4 && in_columns && ours && local == b"calculatedColumnFormula" {
                    calculated = true;
                }
                if matches!(token.event, Event::Empty(_)) {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
    }
    match (id, name, display_name, area) {
        (true, Some(name), Some(display_name), Some(area)) => Ok(TableEntry {
            name,
            display_name,
            area,
            columns,
            style,
            header_row,
            totals_row,
            calculated,
            part,
        }),
        _ => Err(TABLE_UNREADABLE),
    }
}

/// A worksheet root's start tag with a prefix bound to the relationships
/// namespace, and that prefix: the tag as it is with the prefix it binds,
/// or, when it binds none, with one bound (`r`, or the first of `r1`, `r2`
/// and so on that the tag does not declare) in the transitional or the
/// strict form, as part 4a's links retag it (`specs/0021-tables.md`, P1
/// ruling 3; `specs/0022-comments-and-images.md`, "Comments: the parts").
#[cfg(any(feature = "tables", feature = "comments", feature = "images"))]
fn rels_prefixed(raw: &[u8], strict: bool, index: usize) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let tag = scan_start_tag(raw).ok_or(Error::Xml(index))?;
    for attribute in &tag.attributes {
        if let Some(prefix) = raw[attribute.name.clone()].strip_prefix(b"xmlns:")
            && RELATIONSHIPS_ATTRIBUTE.contains(&&raw[attribute.value.clone()])
        {
            return Ok((raw.to_vec(), prefix.to_vec()));
        }
    }
    let mut n = 0u32;
    let prefix = loop {
        let candidate = if n == 0 {
            b"r".to_vec()
        } else {
            format!("r{n}").into_bytes()
        };
        let declaration = [b"xmlns:".as_slice(), &candidate].concat();
        if attribute_value(raw, &declaration, index)?.is_none() {
            break candidate;
        }
        n = n.checked_add(1).ok_or(Error::Xml(index))?;
    };
    let namespace = RELATIONSHIPS_ATTRIBUTE[usize::from(strict)];
    let tag = with_attribute(
        raw,
        &[b"xmlns:".as_slice(), &prefix].concat(),
        namespace,
        index,
    )?;
    Ok((tag, prefix))
}

/// The `<tableParts>` of a worksheet name, its place in
/// [`WORKSHEET_CHILDREN`].
#[cfg(feature = "tables")]
const TABLE_PARTS: &[u8] = b"tableParts";

/// The tables' part of a worksheet rewrite (`specs/0021-tables.md`, "The
/// worksheet"): the part's merges and autofilter checked against the
/// additions, the `<tableParts>` held while it is read, bounded by that one
/// element, the additions appended with `count` set, and the prefix `r:id`
/// takes. The Cargo feature `tables`.
#[cfg(feature = "tables")]
struct TablesPass<'w> {
    tables: &'w [TableWork],
    /// The relationship ids of the tables the write removes from the part.
    removed: &'w [Vec<u8>],
    /// Whether the write adds or removes a table on the part, and has met
    /// or created a `<tableParts>`.
    touched: bool,
    met: bool,
    /// The prefix the root binds to the relationships namespace, or the one
    /// its retag binds; empty until the root is read.
    rels_prefix: Vec<u8>,
    buffer: Option<TablePartsBuffer>,
    /// Whether the root child last opened is a `<mergeCells>`.
    in_merges: bool,
}

/// A `<tableParts>` being read: its start tag, its prefix and the bytes of
/// the children it keeps, with how many `<tablePart>` elements those are.
#[cfg(feature = "tables")]
struct TablePartsBuffer {
    depth: usize,
    start: Vec<u8>,
    empty: bool,
    name: Vec<u8>,
    prefix: Vec<u8>,
    content: Vec<u8>,
    kept: u64,
}

#[cfg(feature = "tables")]
impl<'w> TablesPass<'w> {
    fn new(tables: &'w [TableWork], removed: &'w [Vec<u8>]) -> Self {
        TablesPass {
            tables,
            removed,
            touched: !tables.is_empty() || !removed.is_empty(),
            met: false,
            rels_prefix: Vec::new(),
            buffer: None,
            in_merges: false,
        }
    }

    /// Checks a root child or a merge against the additions: a
    /// `<mergeCell>` the part keeps (one this write does not unmerge) or the
    /// root's `<autoFilter>` overlapping one, or whose `ref` does not parse,
    /// is refused (decision 4 A).
    fn observe(
        &mut self,
        raw: &[u8],
        (local, ours, depth): (&[u8], bool, usize),
        unmerges: &[Area],
        index: usize,
    ) -> Result<(), Error> {
        if self.tables.is_empty() {
            return Ok(());
        }
        let merge = depth == 3 && self.in_merges && ours && local == b"mergeCell";
        if depth == 2 {
            self.in_merges = ours && local == b"mergeCells";
        }
        if merge || (depth == 2 && ours && local == b"autoFilter") {
            let area = attribute_value(raw, b"ref", index)?
                .and_then(Area::parse)
                .ok_or(TABLE_OVERLAP)?;
            if merge && unmerges.contains(&area) {
                return Ok(());
            }
            if self.tables.iter().any(|table| table.area.overlaps(&area)) {
                return Err(TABLE_OVERLAP);
            }
        }
        Ok(())
    }

    /// The root's start tag as the rewrite writes it: as it is, or, when a
    /// table is added and the root binds no prefix to the relationships
    /// namespace, with one bound (`r`, or the first of `r1`, `r2` and so on
    /// that the tag does not declare), in the worksheet's own form (P1
    /// ruling 3), as part 4a's links do.
    fn root_tag(&mut self, raw: &[u8], strict: bool, index: usize) -> Result<Vec<u8>, Error> {
        if self.tables.is_empty() {
            return Ok(raw.to_vec());
        }
        let (tag, prefix) = rels_prefixed(raw, strict, index)?;
        self.rels_prefix = prefix;
        Ok(tag)
    }

    /// The additions as `<tablePart>` elements, back to back, with
    /// `prefix`.
    fn elements(&self, prefix: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        for table in self.tables {
            out.extend_from_slice(&[b"<".as_slice(), prefix, b"tablePart "].concat());
            out.extend_from_slice(&self.rels_prefix);
            out.extend_from_slice(b":id=\"");
            out.extend_from_slice(&table.rel_id);
            out.extend_from_slice(b"\"/>");
        }
        out
    }

    /// The `<tableParts>` the write creates, when it needs one the part lacks
    /// and a root child at `position` comes after it in schema order.
    fn create_before(&mut self, position: usize, prefix: &[u8]) -> Option<Vec<u8>> {
        let at = schema_position(TABLE_PARTS).unwrap_or(0);
        if at < position {
            return self.create_at_end(prefix);
        }
        None
    }

    /// The `<tableParts>` the write creates at the root's end, when it needs
    /// one the part still lacks.
    fn create_at_end(&mut self, prefix: &[u8]) -> Option<Vec<u8>> {
        if self.tables.is_empty() || self.met {
            return None;
        }
        self.met = true;
        let mut out = [b"<".as_slice(), prefix, b"tableParts count=\""].concat();
        out.extend_from_slice(self.tables.len().to_string().as_bytes());
        out.extend_from_slice(b"\">");
        out.extend_from_slice(&self.elements(prefix));
        out.extend_from_slice(&[b"</".as_slice(), prefix, b"tableParts>"].concat());
        Some(out)
    }

    /// A `<tableParts>` start tag the part has: the second is refused; an
    /// empty one is written at once; any other is held. Returns the bytes to
    /// write now.
    fn open(
        &mut self,
        raw: &[u8],
        start: &BytesStart<'_>,
        depth: usize,
        empty: bool,
        index: usize,
    ) -> Result<Vec<u8>, Error> {
        if self.met {
            return Err(TWO_TABLE_PARTS);
        }
        self.met = true;
        let buffer = TablePartsBuffer {
            depth,
            start: raw.to_vec(),
            empty,
            name: start.name().into_inner().as_bytes().to_vec(),
            prefix: prefix_of(start),
            content: Vec::new(),
            kept: 0,
        };
        if empty {
            return self.close(buffer, index);
        }
        self.buffer = Some(buffer);
        Ok(Vec::new())
    }

    /// A start or empty tag inside the held `<tableParts>`: a
    /// `<tablePart>` whose `r:id` names a removed table's relationship is to
    /// be skipped whole; one kept is counted; anything else is held.
    /// Returns whether the caller skips the element.
    fn child(
        &mut self,
        raw: &[u8],
        start: &BytesStart<'_>,
        (local, ours, depth): (&[u8], bool, usize),
        scope: &Scope,
        index: usize,
    ) -> Result<bool, Error> {
        let Some(at) = self.buffer.as_ref().map(|buffer| buffer.depth) else {
            return Ok(false);
        };
        let element = depth == at + 1 && ours && local == b"tablePart";
        if element && !self.removed.is_empty() {
            for attribute in start.attributes() {
                let attribute = attribute.map_err(|_| Error::Xml(index))?;
                if attribute.key.as_namespace_binding().is_some() {
                    continue;
                }
                let (namespace, name) = scope.resolve_attribute(attribute.key)?;
                if name == b"id"
                    && is_relationships_attribute(namespace)
                    && self
                        .removed
                        .iter()
                        .any(|id| id.as_slice() == attribute.value.as_bytes())
                {
                    return Ok(true);
                }
            }
        }
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.kept += u64::from(element);
            buffer.content.extend_from_slice(raw);
        }
        Ok(false)
    }

    /// An end tag or any other token inside the held `<tableParts>`; at the
    /// container's own end tag, the bytes it is written as.
    fn held(
        &mut self,
        raw: &[u8],
        end_depth: Option<usize>,
        index: usize,
    ) -> Result<Option<Vec<u8>>, Error> {
        let Some(depth) = self.buffer.as_ref().map(|buffer| buffer.depth) else {
            return Ok(None);
        };
        if end_depth == Some(depth) {
            return match self.buffer.take() {
                Some(buffer) => Ok(Some(self.close(buffer, index)?)),
                None => Ok(None),
            };
        }
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.content.extend_from_slice(raw);
        }
        Ok(Some(Vec::new()))
    }

    /// The held container written: its start tag with `count` set to the
    /// kept children and the additions (opened up when it was empty), the
    /// kept children, the additions and its end tag; nothing when no table
    /// is left.
    fn close(&self, buffer: TablePartsBuffer, index: usize) -> Result<Vec<u8>, Error> {
        let count = buffer.kept + self.tables.len() as u64;
        if count == 0 {
            return Ok(Vec::new());
        }
        let start = with_attribute(&buffer.start, b"count", count.to_string().as_bytes(), index)?;
        let mut out = if buffer.empty {
            opened_up(&start, index)?
        } else {
            start
        };
        out.extend_from_slice(&buffer.content);
        out.extend_from_slice(&self.elements(&buffer.prefix));
        out.extend_from_slice(&[b"</".as_slice(), &buffer.name, b">"].concat());
        Ok(out)
    }
}

/// The `<legacyDrawing>` of a worksheet name, its place in
/// [`WORKSHEET_CHILDREN`].
#[cfg(feature = "comments")]
const LEGACY_DRAWING: &[u8] = b"legacyDrawing";

/// The `<drawing>` of a worksheet name, its place in [`WORKSHEET_CHILDREN`].
#[cfg(feature = "images")]
const DRAWING_ELEMENT: &[u8] = b"drawing";

/// The `r:id` of the worksheet's root child `name`, `<legacyDrawing>` or
/// `<drawing>`, the part streamed once (`specs/0022-comments-and-images.md`,
/// "Comments: the parts" and "Images: the calls"): a second such element,
/// or one after a root child that follows it in schema order, is refused
/// with `two`, and one without an `r:id` with `unreadable`.
#[cfg(any(feature = "comments", feature = "images"))]
fn element_rel_id(
    input: impl Read,
    index: usize,
    name: &[u8],
    two: Error,
    unreadable: Error,
) -> Result<Option<Vec<u8>>, Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let place = schema_position(name).unwrap_or(0);
    // The furthest schema place of a root child met so far.
    let mut furthest = 0usize;
    let mut found: Option<Vec<u8>> = None;
    let mut met = false;
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                depth += 1;
                scope.start(start)?;
                if depth <= 2 {
                    let (namespace, local) = scope.resolve_element(start.name())?;
                    let ours = is_main(namespace);
                    if depth == 1 && (!ours || local != b"worksheet") {
                        return Err(Error::Package("the worksheet part's root is not worksheet"));
                    }
                    if depth == 2 && ours {
                        if local == name {
                            if met || furthest > place {
                                return Err(two);
                            }
                            met = true;
                            for attribute in start.attributes() {
                                let attribute = attribute.map_err(|_| Error::Xml(index))?;
                                if attribute.key.as_namespace_binding().is_some() {
                                    continue;
                                }
                                let (namespace, name) = scope.resolve_attribute(attribute.key)?;
                                if name == b"id" && is_relationships_attribute(namespace) {
                                    found = Some(attribute.value.as_bytes().to_vec());
                                }
                            }
                            if found.is_none() {
                                return Err(unreadable);
                            }
                        }
                        if let Some(at) = schema_position(local) {
                            furthest = furthest.max(at);
                        }
                    }
                }
                if matches!(token.event, Event::Empty(_)) {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
    }
    Ok(found)
}

/// A text as the comments part writes it in element content: escaped as a
/// cell's inline string is, a carriage return as a character reference.
#[cfg(feature = "comments")]
fn content_escaped(text: &str) -> Vec<u8> {
    escape(text).replace('\r', "&#13;").into_bytes()
}

/// For each addition, the `authorId` it takes, and the authors to append:
/// a kept author whose text equals the addition's gives its index, an
/// author not kept is appended once and takes the kept count plus the
/// authors appended before it.
#[cfg(feature = "comments")]
fn author_ids(kept: &[String], additions: &[(CellRef, NewComment)]) -> (Vec<u64>, Vec<String>) {
    let mut appended: Vec<String> = Vec::new();
    let mut ids = Vec::new();
    for (_, comment) in additions {
        let id = match kept.iter().position(|author| *author == comment.author) {
            Some(at) => at,
            None => match appended.iter().position(|author| *author == comment.author) {
                Some(at) => kept.len() + at,
                None => {
                    appended.push(comment.author.clone());
                    kept.len() + appended.len() - 1
                }
            },
        };
        ids.push(id as u64);
    }
    (ids, appended)
}

/// The `<author>` elements for `authors`, with `prefix`.
#[cfg(feature = "comments")]
fn author_elements(authors: &[String], prefix: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for author in authors {
        out.extend_from_slice(&[b"<".as_slice(), prefix, b"author>"].concat());
        out.extend_from_slice(&content_escaped(author));
        out.extend_from_slice(&[b"</".as_slice(), prefix, b"author>"].concat());
    }
    out
}

/// The `<comment>` elements for the additions, with `prefix`: one run with
/// no properties, `xml:space="preserve"` on its `<t>` when the text begins
/// or ends with whitespace; with the Cargo feature `richtext`, a rich
/// comment's runs, one `<r>` each.
#[cfg(feature = "comments")]
fn comment_elements(additions: &[(CellRef, NewComment)], ids: &[u64], prefix: &[u8]) -> Vec<u8> {
    let p = prefix;
    let mut out = Vec::new();
    for ((cell, comment), id) in additions.iter().zip(ids) {
        out.extend_from_slice(&[b"<".as_slice(), p, b"comment ref=\""].concat());
        out.extend_from_slice(cell.a1().as_bytes());
        out.extend_from_slice(format!("\" authorId=\"{id}\"><").as_bytes());
        out.extend_from_slice(&[p, b"text>"].concat());
        #[cfg(feature = "richtext")]
        if let Some(rich) = &comment.runs {
            for run in &rich.runs {
                out.extend_from_slice(&[b"<".as_slice(), p, b"r>"].concat());
                if let Some(font) = &run.font {
                    out.extend_from_slice(&font.composed("rPr", "rFont").element(p));
                }
                out.extend_from_slice(&comment_t(&run.text, p));
                out.extend_from_slice(&[b"</".as_slice(), p, b"r>"].concat());
            }
            out.extend_from_slice(&[b"</".as_slice(), p, b"text></", p, b"comment>"].concat());
            continue;
        }
        out.extend_from_slice(&comment_t(&comment.text, p));
        out.extend_from_slice(&[b"</".as_slice(), p, b"text></", p, b"comment>"].concat());
    }
    out
}

/// One `<t>` of a written comment holding `text`, with `prefix`:
/// `xml:space="preserve"` when the text begins or ends with whitespace.
#[cfg(feature = "comments")]
fn comment_t(text: &str, prefix: &[u8]) -> Vec<u8> {
    let p = prefix;
    let mut out = [b"<".as_slice(), p, b"t"].concat();
    let space = |c: Option<char>| c.is_some_and(|c| matches!(c, ' ' | '\t' | '\n' | '\r'));
    if space(text.chars().next()) || space(text.chars().last()) {
        out.extend_from_slice(b" xml:space=\"preserve\"");
    }
    out.push(b'>');
    out.extend_from_slice(&content_escaped(text));
    out.extend_from_slice(&[b"</".as_slice(), p, b"t>"].concat());
    out
}

/// A created comments part's bytes (`specs/0022-comments-and-images.md`,
/// "Comments: the parts"): the declaration on its own line and the root on
/// one, in the default namespace, with no `shapeId`, `xr` namespace or
/// `uid`.
#[cfg(feature = "comments")]
fn comments_part_bytes(additions: &[(CellRef, NewComment)]) -> Vec<u8> {
    let (ids, authors) = author_ids(&[], additions);
    let mut out =
        b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<comments xmlns=\""
            .to_vec();
    out.extend_from_slice(MAIN[0]);
    out.extend_from_slice(b"\"><authors>");
    out.extend_from_slice(&author_elements(&authors, b""));
    out.extend_from_slice(b"</authors><commentList>");
    out.extend_from_slice(&comment_elements(additions, &ids, b""));
    out.extend_from_slice(b"</commentList></comments>");
    out
}

/// What [`comments_appended`] gives: the part's new bytes, how many
/// `<comment>` elements it held and kept, and the cells it held them on.
#[cfg(feature = "comments")]
struct Appended {
    bytes: Vec<u8>,
    held: u64,
    kept: u64,
    found: Vec<CellRef>,
}

/// A kept comments part with the edits (`specs/0022-comments-and-
/// images.md`, "Comments: the parts" and `remove_comment`), streamed: every
/// byte copied but the `<comment>` of a cell in `skipped` (a replaced or a
/// removed one), the new authors appended before `<authors>`'s end tag and
/// the new comments before `<commentList>`'s, each with its container's
/// prefix. A root other than `comments`, a `commentList` before any
/// `authors`, none or two, or a `comment` whose `ref` is absent or does not
/// parse, cannot be read.
#[cfg(feature = "comments")]
fn comments_appended(
    input: impl Read,
    index: usize,
    additions: &[(CellRef, NewComment)],
    skipped: &[CellRef],
) -> Result<Appended, Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let mut out = Vec::new();
    if stream.bom() {
        out.extend_from_slice(&BYTE_ORDER_MARK);
    }
    let mut found: Vec<CellRef> = Vec::new();
    let mut gone = 0u64;
    let mut depth = 0usize;
    let mut kept: Vec<String> = Vec::new();
    // The author whose text is being read, and whether a CR came last.
    let mut author: Option<(String, bool)> = None;
    let mut ids: Option<Vec<u64>> = None;
    // The open container's prefix and its end tag's bytes come from the
    // token; whether it is `<authors>` or `<commentList>`.
    let mut in_authors: Option<Vec<u8>> = None;
    let mut in_list: Option<Vec<u8>> = None;
    let mut lists = 0u32;
    let mut held = 0u64;
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                if depth == 1 && (!ours || local != b"comments") {
                    return Err(COMMENTS_UNREADABLE);
                }
                if depth == 2 && ours && local == b"authors" && ids.is_none() {
                    let prefix = prefix_of(start);
                    if empty {
                        let (got, appended) = author_ids(&kept, additions);
                        ids = Some(got);
                        out.extend_from_slice(&opened_up(token.raw, index)?);
                        out.extend_from_slice(&author_elements(&appended, &prefix));
                        out.extend_from_slice(
                            &[b"</".as_slice(), start.name().into_inner().as_bytes(), b">"]
                                .concat(),
                        );
                    } else {
                        out.extend_from_slice(token.raw);
                        in_authors = Some(prefix);
                    }
                } else if depth == 3 && in_authors.is_some() && ours && local == b"author" {
                    out.extend_from_slice(token.raw);
                    if empty {
                        kept.push(String::new());
                    } else {
                        author = Some((String::new(), false));
                    }
                } else if depth == 2 && ours && local == b"commentList" {
                    lists += 1;
                    let Some(ids) = ids.as_ref().filter(|_| lists == 1) else {
                        return Err(COMMENTS_UNREADABLE);
                    };
                    let prefix = prefix_of(start);
                    if empty {
                        out.extend_from_slice(&opened_up(token.raw, index)?);
                        out.extend_from_slice(&comment_elements(additions, ids, &prefix));
                        out.extend_from_slice(
                            &[b"</".as_slice(), start.name().into_inner().as_bytes(), b">"]
                                .concat(),
                        );
                    } else {
                        out.extend_from_slice(token.raw);
                        in_list = Some(prefix);
                    }
                } else if depth == 3 && in_list.is_some() && ours && local == b"comment" {
                    held += 1;
                    let cell = attribute_value(token.raw, b"ref", index)?
                        .and_then(CellRef::parse)
                        .ok_or(COMMENTS_UNREADABLE)?;
                    found.push(cell);
                    if skipped.contains(&cell) {
                        // A replaced or removed cell's comment goes whole
                        // (decision 6 A; `remove_comment`).
                        gone += 1;
                        if !empty {
                            skip_element(&mut stream, &mut scope)?;
                        }
                        scope.end();
                        depth -= 1;
                        continue;
                    }
                    out.extend_from_slice(token.raw);
                } else {
                    out.extend_from_slice(token.raw);
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                if depth == 3
                    && let Some((text, _)) = author.take()
                {
                    kept.push(text);
                } else if depth == 2
                    && let Some(prefix) = in_authors.take()
                {
                    let (got, appended) = author_ids(&kept, additions);
                    ids = Some(got);
                    out.extend_from_slice(&author_elements(&appended, &prefix));
                } else if depth == 2
                    && let Some(prefix) = in_list.take()
                    && let Some(ids) = &ids
                {
                    out.extend_from_slice(&comment_elements(additions, ids, &prefix));
                }
                out.extend_from_slice(token.raw);
                scope.end();
                depth -= 1;
            }
            Event::Text(ref chunk) => {
                if let Some((text, after_cr)) = author.as_mut() {
                    crate::cells::push_text(text, chunk, after_cr);
                }
                out.extend_from_slice(token.raw);
            }
            Event::CData(ref chunk) => {
                if let Some((text, after_cr)) = author.as_mut() {
                    crate::cells::push_text(text, chunk, after_cr);
                }
                out.extend_from_slice(token.raw);
            }
            Event::GeneralRef(ref reference) => {
                if let Some((text, after_cr)) = author.as_mut() {
                    text.push(crate::cells::expand(reference, index)?);
                    *after_cr = false;
                }
                out.extend_from_slice(token.raw);
            }
            _ => out.extend_from_slice(token.raw),
        }
    }
    if lists == 0 {
        return Err(COMMENTS_UNREADABLE);
    }
    Ok(Appended {
        bytes: out,
        held,
        kept: held - gone,
        found,
    })
}

/// Reads a comments part into its entries (`specs/0022-comments-and-
/// images.md`, "Comments: the calls"), streamed once.
///
/// With the Cargo feature `richtext` the walk also tells a comment's runs to
/// the listener a cell's inline string has (`specs/0024-rich-text.md`,
/// "Comments"): each `<r>` child of `<text>` opened and closed, its `<rPr>`
/// read by the one reader, and each `<t>` with whether it is a run's; a
/// `<t>` under an `<rPh>` child goes to the text, as it always has, and not
/// to the runs, as a cell's phonetic run does not.
#[cfg(feature = "comments")]
fn read_comments(input: impl Read, index: usize) -> Result<Vec<CommentEntry>, Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut authors: Vec<String> = Vec::new();
    let mut in_authors = false;
    let mut lists = 0u32;
    let mut in_list = false;
    // The comment being read: its cell, its `authorId`, its text, and the
    // depth of its `<text>` while that is open.
    let mut comment: Option<(CellRef, u64, String, Option<usize>)> = None;
    let mut entries: Vec<CommentEntry> = Vec::new();
    // The open comment's runs, the texts given to them (the flattened text
    // their first-run rule reads, which a phonetic run's text is not part
    // of), whether a run is open, and the depth of an `<rPh>` being passed.
    #[cfg(feature = "richtext")]
    let mut runs = Collect::default();
    #[cfg(feature = "richtext")]
    let mut runs_text = String::new();
    #[cfg(feature = "richtext")]
    let mut in_run = false;
    #[cfg(feature = "richtext")]
    let mut phonetic: Option<usize> = None;
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                if depth == 1 && (!ours || local != b"comments") {
                    return Err(COMMENTS_UNREADABLE);
                }
                if depth == 2 {
                    in_authors = ours && local == b"authors" && !empty;
                    if ours && local == b"commentList" {
                        lists += 1;
                        if lists > 1 {
                            return Err(COMMENTS_UNREADABLE);
                        }
                        in_list = !empty;
                    }
                } else if depth == 3 && in_authors && ours && local == b"author" {
                    authors.push(if empty {
                        String::new()
                    } else {
                        let text = crate::cells::text_of(&mut stream, &mut scope, index)?;
                        depth -= 1;
                        text
                    });
                    if empty {
                        scope.end();
                        depth -= 1;
                    }
                    continue;
                } else if depth == 3 && in_list && ours && local == b"comment" {
                    let cell = attribute_value(token.raw, b"ref", index)?
                        .and_then(CellRef::parse)
                        .ok_or(COMMENTS_UNREADABLE)?;
                    let id = attribute_value(token.raw, b"authorId", index)?
                        .and_then(digits_value)
                        .ok_or(COMMENTS_UNREADABLE)?;
                    if empty {
                        entries.push(comment_entry(cell, id, String::new(), &authors));
                    } else {
                        comment = Some((cell, id, String::new(), None));
                        #[cfg(feature = "richtext")]
                        {
                            runs = Collect::default();
                            runs_text.clear();
                            in_run = false;
                            phonetic = None;
                        }
                    }
                } else if let Some((_, _, text, inside)) = comment.as_mut() {
                    if depth == 4 && ours && local == b"text" {
                        *inside = (!empty).then_some(depth);
                    } else if inside.is_some() && ours && local == b"t" && !empty {
                        let read = crate::cells::text_of(&mut stream, &mut scope, index)?;
                        #[cfg(feature = "richtext")]
                        if phonetic.is_none() {
                            runs.text(&read, in_run);
                            runs_text.push_str(&read);
                        }
                        text.push_str(&read);
                        depth -= 1;
                        continue;
                    }
                    // A child of `<text>` the runs hear: an `<rPh>` passed,
                    // an `<r>` opened, and the open run's `<rPr>` read.
                    #[cfg(feature = "richtext")]
                    if inside.is_some() && ours && phonetic.is_none() {
                        if depth == 5 && local == b"rPh" && !empty {
                            phonetic = Some(depth);
                        } else if depth == 5 && local == b"r" {
                            runs.open_run();
                            if empty {
                                runs.close_run();
                            } else {
                                in_run = true;
                            }
                        } else if depth == 6 && in_run && local == b"rPr" {
                            runs.properties(&mut stream, &mut scope, index, empty)?;
                            if !empty {
                                // The reader consumed the `<rPr>` through its
                                // end tag and gave it back to `scope`.
                                depth -= 1;
                                continue;
                            }
                        }
                    }
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                #[cfg(feature = "richtext")]
                if phonetic == Some(depth) {
                    phonetic = None;
                } else if in_run && depth == 5 {
                    in_run = false;
                    runs.close_run();
                }
                if let Some((_, _, _, inside)) = comment.as_mut()
                    && *inside == Some(depth)
                {
                    *inside = None;
                } else if depth == 3
                    && let Some((cell, id, text, _)) = comment.take()
                {
                    let entry = comment_entry(cell, id, text, &authors);
                    #[cfg(feature = "richtext")]
                    let entry = CommentEntry {
                        runs: std::mem::take(&mut runs).rich(&runs_text),
                        ..entry
                    };
                    entries.push(entry);
                } else if depth == 2 {
                    in_authors = false;
                    in_list = false;
                }
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
    }
    if lists == 0 {
        return Err(COMMENTS_UNREADABLE);
    }
    Ok(entries)
}

/// One comment's entry: its author the `authors` child at `id`, empty past
/// the list.
#[cfg(feature = "comments")]
fn comment_entry(cell: CellRef, id: u64, text: String, authors: &[String]) -> CommentEntry {
    let author = usize::try_from(id)
        .ok()
        .and_then(|at| authors.get(at))
        .cloned()
        .unwrap_or_default();
    CommentEntry {
        cell,
        author,
        text,
        #[cfg(feature = "richtext")]
        runs: None,
    }
}

/// What the write reads of a kept VML part (`specs/0022-comments-and-
/// images.md`, "Comments: the parts", and P1 ruling 1): the prefixes its root
/// binds to the three VML namespaces, its `idmap` number, whether it has
/// the shapetype `_x0000_t202`, its shapes, and where its root's end tag
/// starts.
#[cfg(feature = "comments")]
struct VmlRead {
    prefixes: [Vec<u8>; 3],
    idmap: Option<u64>,
    shapetype: bool,
    shapes: Vec<VmlShape>,
    end: usize,
}

/// One `<v:shape>` of a VML part: its bytes' span, the numbers its `id` and
/// `o:spid` carry in the form `_x0000_s<n>`, whether it is a note shape
/// (`ClientData` of `ObjectType` `Note`), and a note's zero-based row and
/// column.
#[cfg(feature = "comments")]
struct VmlShape {
    start: usize,
    end: usize,
    numbers: Vec<u64>,
    is_note: bool,
    note: Option<(u64, u64)>,
}

/// Whether `shape` is the note of a cell in `cells`, to be left out.
#[cfg(feature = "comments")]
fn skips(shape: &VmlShape, cells: &[CellRef]) -> bool {
    shape.note.is_some_and(|(row, column)| {
        cells
            .iter()
            .any(|cell| u64::from(cell.row) - 1 == row && u64::from(cell.col) - 1 == column)
    })
}

/// The number of a shape id of the form `_x0000_s<n>`.
#[cfg(feature = "comments")]
fn shape_number(value: &[u8]) -> Option<u64> {
    value.strip_prefix(b"_x0000_s").and_then(digits_value)
}

/// The value of one or more ASCII digits, `None` for anything else or a
/// value past `u64`.
#[cfg(any(feature = "comments", feature = "images"))]
fn digits_value(value: &[u8]) -> Option<u64> {
    if value.is_empty() || !value.iter().all(u8::is_ascii_digit) {
        return None;
    }
    str::from_utf8(value).ok()?.parse().ok()
}

/// The first number of an `idmap`'s `data` list, when it is a decimal
/// number.
#[cfg(feature = "comments")]
fn idmap_number(value: &[u8]) -> Option<u64> {
    let first = value.split(|byte| *byte == b',').next().unwrap_or(value);
    digits_value(first.trim_ascii())
}

/// Reads a kept VML part held whole. A part the parser refuses, a root other
/// than an unprefixed `xml` or with no end tag, or a root that binds no
/// prefix to one of the three namespaces, cannot be read; an element whose
/// prefix nothing binds meets part 2's refusal.
#[cfg(feature = "comments")]
fn read_vml(bytes: &[u8], index: usize) -> Result<VmlRead, Error> {
    let unreadable = |error: Error| match error {
        Error::Xml(_) => COMMENTS_UNREADABLE,
        other => other,
    };
    let mut stream = Stream::new(Cursor::new(bytes), index).map_err(unreadable)?;
    let mut scope = Scope::new(index);
    let mut at: usize = if stream.bom() {
        BYTE_ORDER_MARK.len()
    } else {
        0
    };
    let mut depth = 0usize;
    let mut prefixes: [Option<Vec<u8>>; 3] = [None, None, None];
    let (mut idmap, mut shapetype, mut layout) = (None, false, false);
    let mut shapes: Vec<VmlShape> = Vec::new();
    // The open shape, its depth, its `ClientData`'s depth and whether that is
    // a note, and the row or column whose text is being read.
    let mut open: Option<(VmlShape, usize, Option<usize>, bool)> = None;
    let mut cell: (Option<u64>, Option<u64>) = (None, None);
    let mut field: Option<(bool, String)> = None;
    let mut end = None;
    while let Some(token) = stream.next().map_err(unreadable)? {
        let from = at;
        at = at.checked_add(token.raw.len()).ok_or(COMMENTS_UNREADABLE)?;
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let is = |which: usize| namespace == Some(VML_NAMESPACES[which]);
                if depth == 1 {
                    if namespace.is_some() || local != b"xml" || empty {
                        return Err(COMMENTS_UNREADABLE);
                    }
                    for attribute in start.attributes() {
                        let attribute = attribute.map_err(|_| COMMENTS_UNREADABLE)?;
                        if let Some(quick_xml::name::PrefixDeclaration::Named(prefix)) =
                            attribute.key.as_namespace_binding()
                        {
                            for (which, namespace) in VML_NAMESPACES.iter().enumerate() {
                                if attribute.value.as_bytes() == *namespace
                                    && prefixes[which].is_none()
                                {
                                    prefixes[which] = Some(prefix.as_bytes().to_vec());
                                }
                            }
                        }
                    }
                } else if depth == 2 && is(1) && local == b"shapelayout" {
                    layout = !empty;
                } else if depth == 3 && layout && is(1) && local == b"idmap" && idmap.is_none() {
                    idmap = attribute_value(token.raw, b"data", index)?.and_then(idmap_number);
                } else if depth == 2 && is(0) && local == b"shapetype" {
                    shapetype |= attribute_value(token.raw, b"id", index)?
                        == Some(b"_x0000_t202".as_slice());
                } else if depth == 2 && is(0) && local == b"shape" {
                    let mut numbers = Vec::new();
                    for attribute in start.attributes() {
                        let attribute = attribute.map_err(|_| Error::Xml(index))?;
                        if attribute.key.as_namespace_binding().is_some() {
                            continue;
                        }
                        let (namespace, name) = scope.resolve_attribute(attribute.key)?;
                        let spid = namespace == Some(VML_NAMESPACES[1]) && name == b"spid";
                        if (namespace.is_none() && name == b"id") || spid {
                            numbers.extend(shape_number(attribute.value.as_bytes()));
                        }
                    }
                    let shape = VmlShape {
                        start: from,
                        end: at,
                        numbers,
                        is_note: false,
                        note: None,
                    };
                    if empty {
                        shapes.push(shape);
                    } else {
                        open = Some((shape, depth, None, false));
                        cell = (None, None);
                    }
                } else if let Some((_, _, client, note)) = open.as_mut() {
                    if is(2) && local == b"ClientData" && client.is_none() {
                        *client = (!empty).then_some(depth);
                        *note = attribute_value(token.raw, b"ObjectType", index)?
                            == Some(b"Note".as_slice());
                    } else if client.is_some_and(|at| depth == at + 1)
                        && is(2)
                        && (local == b"Row" || local == b"Column")
                        && !empty
                    {
                        field = Some((local == b"Row", String::new()));
                    }
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::Text(ref chunk) => {
                if let Some((_, text)) = field.as_mut() {
                    text.push_str(chunk);
                }
            }
            Event::End(_) => {
                if let Some((row, text)) = field.take() {
                    let number = digits_value(text.trim().as_bytes());
                    if row {
                        cell.0 = number;
                    } else {
                        cell.1 = number;
                    }
                }
                if depth == 2 {
                    layout = false;
                }
                if let Some((shape, at_depth, client, note)) = open.as_mut() {
                    if *client == Some(depth) {
                        *client = Some(usize::MAX);
                    }
                    if *at_depth == depth {
                        shape.end = at;
                        shape.is_note = *note;
                        if *note && let (Some(row), Some(column)) = cell {
                            shape.note = Some((row, column));
                        }
                        if let Some((shape, ..)) = open.take() {
                            shapes.push(shape);
                        }
                    }
                }
                if depth == 1 {
                    end = Some(from);
                }
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
    }
    let [Some(v), Some(o), Some(x)] = prefixes else {
        return Err(COMMENTS_UNREADABLE);
    };
    Ok(VmlRead {
        prefixes: [v, o, x],
        idmap,
        shapetype,
        shapes,
        end: end.ok_or(COMMENTS_UNREADABLE)?,
    })
}

/// The `idmap` number of a VML part the write does not edit, streamed to
/// its first shape at most; a part the parser refuses cannot be read.
#[cfg(feature = "comments")]
fn vml_idmap(input: impl Read, index: usize) -> Result<Option<u64>, Error> {
    let unreadable = |error: Error| match error {
        Error::Xml(_) => COMMENTS_UNREADABLE,
        other => other,
    };
    let mut stream = Stream::new(input, index).map_err(unreadable)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut layout = false;
    while let Some(token) = stream.next().map_err(unreadable)? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let office = namespace == Some(VML_NAMESPACES[1]);
                if depth == 2 && office && local == b"shapelayout" {
                    layout = !empty;
                } else if depth == 3 && layout && office && local == b"idmap" {
                    return Ok(attribute_value(token.raw, b"data", index)?.and_then(idmap_number));
                } else if depth == 2 && namespace == Some(VML_NAMESPACES[0]) && local == b"shape" {
                    return Ok(None);
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                if depth == 2 {
                    layout = false;
                }
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
    }
    Ok(None)
}

/// The smallest positive number `used` does not hold, now held.
#[cfg(feature = "comments")]
fn smallest_unused(used: &mut Vec<u64>) -> u64 {
    let mut n = 1u64;
    while used.contains(&n) {
        n += 1;
    }
    used.push(n);
    n
}

/// The shapetype `_x0000_t202` a note shape names, with the `v` and `o`
/// prefixes given (each with its colon).
#[cfg(feature = "comments")]
fn note_shapetype(v: &[u8], o: &[u8]) -> Vec<u8> {
    [
        b"<".as_slice(),
        v,
        b"shapetype id=\"_x0000_t202\" coordsize=\"21600,21600\" ",
        o,
        b"spt=\"202\" path=\"m,l,21600r21600,l21600,xe\"><",
        v,
        b"stroke joinstyle=\"miter\"/><",
        v,
        b"path gradientshapeok=\"t\" ",
        o,
        b"connecttype=\"rect\"/></",
        v,
        b"shapetype>",
    ]
    .concat()
}

/// A hidden note shape for `cell` with the id number `number` and the
/// z-index `k`, with the `v`, `o` and `x` prefixes given (each with its
/// colon): openpyxl's defaults, the box Excel places beside the cell.
#[cfg(feature = "comments")]
fn note_shape(prefixes: [&[u8]; 3], number: u64, k: u64, cell: CellRef) -> Vec<u8> {
    let [v, o, x] = prefixes;
    let row = u64::from(cell.row) - 1;
    let column = u64::from(cell.col) - 1;
    let top = row.saturating_sub(1);
    let anchor = format!(
        "{}, 15, {top}, 2, {}, 15, {}, 2",
        column + 1,
        column + 3,
        top + 4
    );
    let mut out = [b"<".as_slice(), v, b"shape id=\"_x0000_s"].concat();
    out.extend_from_slice(number.to_string().as_bytes());
    out.extend_from_slice(
        b"\" type=\"#_x0000_t202\" style=\"position:absolute;margin-left:59.25pt;\
          margin-top:1.5pt;width:108pt;height:59.25pt;z-index:",
    );
    out.extend_from_slice(k.to_string().as_bytes());
    out.extend_from_slice(b";visibility:hidden\" fillcolor=\"#ffffe1\" ");
    out.extend_from_slice(o);
    out.extend_from_slice(b"insetmode=\"auto\">");
    out.extend_from_slice(
        &[
            b"<".as_slice(),
            v,
            b"fill color2=\"#ffffe1\"/><",
            v,
            b"shadow on=\"t\" color=\"black\" obscured=\"t\"/><",
            v,
            b"path ",
            o,
            b"connecttype=\"none\"/><",
            v,
            b"textbox style=\"mso-direction-alt:auto\"><div style=\"text-align:left\"/></",
            v,
            b"textbox><",
            x,
            b"ClientData ObjectType=\"Note\"><",
            x,
            b"MoveWithCells/><",
            x,
            b"SizeWithCells/><",
            x,
            b"Anchor>",
        ]
        .concat(),
    );
    out.extend_from_slice(anchor.as_bytes());
    out.extend_from_slice(
        &[
            b"</".as_slice(),
            x,
            b"Anchor><",
            x,
            b"AutoFill>False</",
            x,
            b"AutoFill><",
            x,
            b"Row>",
        ]
        .concat(),
    );
    out.extend_from_slice(row.to_string().as_bytes());
    out.extend_from_slice(&[b"</".as_slice(), x, b"Row><", x, b"Column>"].concat());
    out.extend_from_slice(column.to_string().as_bytes());
    out.extend_from_slice(
        &[
            b"</".as_slice(),
            x,
            b"Column></",
            x,
            b"ClientData></",
            v,
            b"shape>",
        ]
        .concat(),
    );
    out
}

/// A created VML part's bytes (`specs/0022-comments-and-images.md`,
/// "Comments: the parts"): one line, no declaration, `idmap` `block` and
/// the note shapes numbered `1024 * block + K`, K from 1.
#[cfg(feature = "comments")]
fn vml_part_bytes(block: u64, cells: &[CellRef]) -> Vec<u8> {
    let mut out = b"<xml".to_vec();
    for (name, namespace) in [b"v".as_slice(), b"o", b"x"]
        .into_iter()
        .zip(VML_NAMESPACES)
    {
        out.extend_from_slice(&[b" xmlns:".as_slice(), name, b"=\"", namespace, b"\""].concat());
    }
    out.extend_from_slice(b"><o:shapelayout v:ext=\"edit\"><o:idmap v:ext=\"edit\" data=\"");
    out.extend_from_slice(block.to_string().as_bytes());
    out.extend_from_slice(b"\"/></o:shapelayout>");
    out.extend_from_slice(&note_shapetype(b"v:", b"o:"));
    for (k, cell) in (1u64..).zip(cells) {
        out.extend_from_slice(&note_shape(
            [b"v:", b"o:", b"x:"],
            1024 * block + k,
            k,
            *cell,
        ));
    }
    out.extend_from_slice(b"</xml>");
    out
}

/// A kept VML part with the edits (P1 ruling 1): its bytes up to the
/// root's end tag, the note shape of a cell in `skipped` (a replaced or a
/// removed one) left out, and, when `cells` adds any, the shapetype when the
/// part has none and a note shape per cell numbered `1024 * block + K`, K
/// the larger of one above the kept shapes and one above the largest
/// number in `block`'s range a kept shape carries; then the end tag and
/// what follows it.
#[cfg(feature = "comments")]
fn vml_appended(
    bytes: &[u8],
    read: &VmlRead,
    block: u64,
    skipped: &[CellRef],
    cells: &[CellRef],
) -> Vec<u8> {
    let low = 1024 * block;
    let mut kept = 0u64;
    let mut highest = 0u64;
    let mut out = Vec::new();
    let mut from = 0usize;
    for shape in &read.shapes {
        if skips(shape, skipped) {
            out.extend_from_slice(&bytes[from..shape.start]);
            from = shape.end;
            continue;
        }
        kept += 1;
        for number in &shape.numbers {
            if *number > low && *number <= low + 1024 {
                highest = highest.max(number - low);
            }
        }
    }
    out.extend_from_slice(&bytes[from..read.end]);
    let colon = |prefix: &[u8]| [prefix, b":"].concat();
    let [v, o, x] = [
        colon(&read.prefixes[0]),
        colon(&read.prefixes[1]),
        colon(&read.prefixes[2]),
    ];
    if !read.shapetype && !cells.is_empty() {
        out.extend_from_slice(&note_shapetype(&v, &o));
    }
    let first = kept.max(highest) + 1;
    for (k, cell) in (first..).zip(cells) {
        out.extend_from_slice(&note_shape([&v, &o, &x], low + k, k, *cell));
    }
    out.extend_from_slice(&bytes[read.end..]);
    out
}

/// A worksheet root child that names one relationship, `<legacyDrawing>`
/// or `<drawing>`, in a worksheet rewrite
/// (`specs/0022-comments-and-images.md`, "Comments: the parts" and "Images:
/// the parts"): the element created in schema position when the write adds
/// the part it names to a sheet that has none, and the prefix its `r:id`
/// takes. The Cargo features `comments` and `images`.
#[cfg(any(feature = "comments", feature = "images"))]
struct ElementPass<'w> {
    name: &'static [u8],
    rel_id: Option<&'w [u8]>,
    written: bool,
    /// The prefix the root binds to the relationships namespace, or the one
    /// its retag binds; empty until the root is read.
    rels_prefix: Vec<u8>,
}

#[cfg(any(feature = "comments", feature = "images"))]
impl<'w> ElementPass<'w> {
    fn new(name: &'static [u8], rel_id: Option<&'w [u8]>) -> Self {
        ElementPass {
            name,
            rel_id,
            written: false,
            rels_prefix: Vec::new(),
        }
    }

    /// The root's start tag as the rewrite writes it: as it is, or, when the
    /// element is created and the root binds no prefix to the relationships
    /// namespace, with one bound, as a table's retag binds it.
    fn root_tag(&mut self, raw: &[u8], strict: bool, index: usize) -> Result<Vec<u8>, Error> {
        if self.rel_id.is_none() {
            return Ok(raw.to_vec());
        }
        let (tag, prefix) = rels_prefixed(raw, strict, index)?;
        self.rels_prefix = prefix;
        Ok(tag)
    }

    /// The element the write creates, when a root child at `position` comes
    /// after it in schema order.
    fn create_before(&mut self, position: usize, prefix: &[u8]) -> Option<Vec<u8>> {
        if schema_position(self.name).unwrap_or(0) < position {
            return self.create_at_end(prefix);
        }
        None
    }

    /// The element the write creates at the root's end, when it is still to
    /// be written.
    fn create_at_end(&mut self, prefix: &[u8]) -> Option<Vec<u8>> {
        let id = self.rel_id?;
        if self.written {
            return None;
        }
        self.written = true;
        let mut out = [b"<".as_slice(), prefix, self.name, b" "].concat();
        out.extend_from_slice(&self.rels_prefix);
        out.extend_from_slice(b":id=\"");
        out.extend_from_slice(id);
        out.extend_from_slice(b"\"/>");
        Some(out)
    }
}

/// The smallest positive N for which neither `xl/media/image<N>.png` nor
/// `.jpeg` is in `taken`, which is ASCII case folded already
/// (`specs/0022-comments-and-images.md`, "Images: the parts").
#[cfg(feature = "images")]
fn free_media(taken: &[String]) -> u64 {
    let mut n = 1u64;
    while ["png", "jpeg"]
        .iter()
        .any(|extension| taken.contains(&format!("xl/media/image{n}.{extension}")))
    {
        n += 1;
    }
    n
}

/// An image's format and pixel size from its first bytes
/// (`specs/0022-comments-and-images.md`, "Images: types"): `(true, width,
/// height)` for a PNG whose first chunk is `IHDR`, read at offsets 16 and
/// 20; `(false, width, height)` for a JPEG whose segments, walked from the
/// start, reach a baseline, extended or progressive `SOF` before the scan,
/// its height and width at offsets 5 and 7 of the segment. Anything else is
/// refused.
#[cfg(feature = "images")]
fn image_header(bytes: &[u8]) -> Result<(bool, u32, u32), Error> {
    const PNG: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    let u16_at = |at: usize| -> Result<usize, Error> {
        match bytes.get(at..at.checked_add(2).ok_or(IMAGE_NOT_PNG_OR_JPEG)?) {
            Some(&[high, low]) => Ok(usize::from(u16::from_be_bytes([high, low]))),
            _ => Err(IMAGE_NOT_PNG_OR_JPEG),
        }
    };
    if bytes.starts_with(&PNG) {
        let (Some(b"IHDR"), Some(width), Some(height)) =
            (bytes.get(12..16), bytes.get(16..20), bytes.get(20..24))
        else {
            return Err(IMAGE_NOT_PNG_OR_JPEG);
        };
        let number = |field: &[u8]| {
            let mut four = [0u8; 4];
            four.copy_from_slice(field);
            u32::from_be_bytes(four)
        };
        return Ok((true, number(width), number(height)));
    }
    if !bytes.starts_with(&[0xFF, 0xD8]) {
        return Err(IMAGE_NOT_PNG_OR_JPEG);
    }
    // Every step past `at` is checked (`CLAUDE.md`; P3 verification ruling
    // 1 (1)).
    let after = |at: usize, step: usize| at.checked_add(step).ok_or(IMAGE_NOT_PNG_OR_JPEG);
    let mut at = 2usize;
    loop {
        if bytes.get(at) != Some(&0xFF) {
            return Err(IMAGE_NOT_PNG_OR_JPEG);
        }
        // Fill bytes before a marker.
        while bytes.get(after(at, 1)?) == Some(&0xFF) {
            at = after(at, 1)?;
        }
        let marker = *bytes.get(after(at, 1)?).ok_or(IMAGE_NOT_PNG_OR_JPEG)?;
        match marker {
            0xC0..=0xC2 => {
                let height = u16_at(after(at, 5)?)?;
                let width = u16_at(after(at, 7)?)?;
                let to_u32 = |value: usize| u32::try_from(value).map_err(|_| IMAGE_NOT_PNG_OR_JPEG);
                return Ok((false, to_u32(width)?, to_u32(height)?));
            }
            // The scan, or the image's end, with no frame before it; a
            // stuffed byte outside a scan.
            0xDA | 0xD9 | 0x00 => return Err(IMAGE_NOT_PNG_OR_JPEG),
            // The markers that stand alone.
            0x01 | 0xD0..=0xD8 => at = after(at, 2)?,
            _ => {
                let length = u16_at(after(at, 2)?)?;
                if length < 2 {
                    return Err(IMAGE_NOT_PNG_OR_JPEG);
                }
                at = after(after(at, 2)?, length)?;
            }
        }
    }
}

/// How an added anchor is written into a drawing part: the prefix, colon
/// and all, the root gives the spreadsheet-drawing namespace; the prefixes
/// the root binds to the DrawingML and relationships namespaces, when it
/// binds them; and whether the root's namespace is the strict form, which
/// a namespace bound on the added elements follows.
#[cfg(feature = "images")]
struct DrawingForm {
    x: Vec<u8>,
    a: Option<Vec<u8>>,
    r: Option<Vec<u8>>,
    strict: bool,
}

/// What the write reads of a kept drawing part: its form, the largest
/// `cNvPr` `id` anywhere in it, and where its root's end tag starts.
#[cfg(feature = "images")]
struct DrawingRead {
    form: DrawingForm,
    highest: u64,
    end: usize,
}

/// Reads a kept drawing part held whole (`specs/0022-comments-and-
/// images.md`, "Images: the parts", and P3 ruling 1: the largest `id` is
/// taken over the whole part, an `AlternateContent`'s `Choice` and
/// `Fallback` included). A part the parser refuses, a root other than
/// `wsDr` in the spreadsheet-drawing namespace or an empty one, and a
/// `cNvPr` `id` that is not a decimal number cannot be read; an element
/// whose prefix nothing binds meets part 2's refusal.
#[cfg(feature = "images")]
fn read_drawing(bytes: &[u8], index: usize) -> Result<DrawingRead, Error> {
    let unreadable = |error: Error| match error {
        Error::Xml(_) => DRAWING_UNREADABLE,
        other => other,
    };
    let mut stream = Stream::new(Cursor::new(bytes), index).map_err(unreadable)?;
    let mut scope = Scope::new(index);
    let mut at: usize = if stream.bom() {
        BYTE_ORDER_MARK.len()
    } else {
        0
    };
    let mut depth = 0usize;
    let mut form = None;
    let mut highest = 0u64;
    let mut end = None;
    while let Some(token) = stream.next().map_err(unreadable)? {
        let from = at;
        at = at.checked_add(token.raw.len()).ok_or(DRAWING_UNREADABLE)?;
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                if depth == 1 {
                    let strict = match namespace {
                        Some(namespace) if namespace == SPREADSHEET_DRAWING[0] => false,
                        Some(namespace) if namespace == SPREADSHEET_DRAWING[1] => true,
                        _ => return Err(DRAWING_UNREADABLE),
                    };
                    if local != b"wsDr" || empty {
                        return Err(DRAWING_UNREADABLE);
                    }
                    let x = match start.name().prefix() {
                        Some(prefix) => [prefix.into_inner().as_bytes(), b":"].concat(),
                        None => Vec::new(),
                    };
                    let (mut a, mut r) = (None, None);
                    for attribute in start.attributes() {
                        let attribute = attribute.map_err(|_| DRAWING_UNREADABLE)?;
                        let value = attribute.value.as_bytes();
                        match attribute.key.as_namespace_binding() {
                            Some(quick_xml::name::PrefixDeclaration::Named(prefix)) => {
                                if DRAWINGML.contains(&value) && a.is_none() {
                                    a = Some(prefix.as_bytes().to_vec());
                                }
                                if RELATIONSHIPS_ATTRIBUTE.contains(&value) && r.is_none() {
                                    r = Some(prefix.as_bytes().to_vec());
                                }
                            }
                            Some(quick_xml::name::PrefixDeclaration::Default)
                                if DRAWINGML.contains(&value) && a.is_none() =>
                            {
                                a = Some(Vec::new());
                            }
                            _ => {}
                        }
                    }
                    form = Some(DrawingForm { x, a, r, strict });
                } else if local == b"cNvPr"
                    && let Some(value) = attribute_value(token.raw, b"id", index)?
                {
                    highest = highest.max(digits_value(value).ok_or(DRAWING_UNREADABLE)?);
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                if depth == 1 {
                    end = Some(from);
                }
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
    }
    Ok(DrawingRead {
        form: form.ok_or(DRAWING_UNREADABLE)?,
        highest,
        end: end.ok_or(DRAWING_UNREADABLE)?,
    })
}

/// `prefix` with its colon, or nothing for the default namespace.
#[cfg(feature = "images")]
fn with_colon(prefix: &[u8]) -> Vec<u8> {
    if prefix.is_empty() {
        Vec::new()
    } else {
        [prefix, b":"].concat()
    }
}

/// An added image's `<oneCellAnchor>` (`specs/0022-comments-and-images.md`,
/// "Images: the parts"): at `cell`'s top-left corner, the extent the image
/// carries, the picture's `cNvPr` `id` and name from `id`, its `blip`
/// naming the relationship `rel`; the DrawingML and relationships
/// namespaces bound on the added elements when `form` says the root binds
/// neither.
#[cfg(feature = "images")]
fn anchor_bytes(
    form: &DrawingForm,
    cell: CellRef,
    image: &NewImage,
    id: u64,
    rel: &[u8],
) -> Vec<u8> {
    let strict = usize::from(form.strict);
    let x = form.x.as_slice();
    let a = match &form.a {
        Some(prefix) => with_colon(prefix),
        None => b"a:".to_vec(),
    };
    let a = a.as_slice();
    let r = form.r.clone().unwrap_or_else(|| b"r".to_vec());
    let bind_a = match form.a {
        Some(_) => Vec::new(),
        None => [b" xmlns:a=\"".as_slice(), DRAWINGML[strict], b"\""].concat(),
    };
    let bind_r = match form.r {
        Some(_) => Vec::new(),
        None => [
            b" xmlns:r=\"".as_slice(),
            RELATIONSHIPS_ATTRIBUTE[strict],
            b"\"",
        ]
        .concat(),
    };
    let column = (cell.col - 1).to_string();
    let row = (cell.row - 1).to_string();
    let (cx, cy) = (image.cx.to_string(), image.cy.to_string());
    let id = id.to_string();
    [
        b"<".as_slice(),
        x,
        b"oneCellAnchor",
        &bind_a,
        b"><",
        x,
        b"from><",
        x,
        b"col>",
        column.as_bytes(),
        b"</",
        x,
        b"col><",
        x,
        b"colOff>0</",
        x,
        b"colOff><",
        x,
        b"row>",
        row.as_bytes(),
        b"</",
        x,
        b"row><",
        x,
        b"rowOff>0</",
        x,
        b"rowOff></",
        x,
        b"from><",
        x,
        b"ext cx=\"",
        cx.as_bytes(),
        b"\" cy=\"",
        cy.as_bytes(),
        b"\"/><",
        x,
        b"pic><",
        x,
        b"nvPicPr><",
        x,
        b"cNvPr id=\"",
        id.as_bytes(),
        b"\" name=\"Image ",
        id.as_bytes(),
        b"\"/><",
        x,
        b"cNvPicPr/></",
        x,
        b"nvPicPr><",
        x,
        b"blipFill><",
        a,
        b"blip",
        &bind_r,
        b" ",
        &r,
        b":embed=\"",
        rel,
        b"\"/><",
        a,
        b"stretch><",
        a,
        b"fillRect/></",
        a,
        b"stretch></",
        x,
        b"blipFill><",
        x,
        b"spPr><",
        a,
        b"prstGeom prst=\"rect\"><",
        a,
        b"avLst/></",
        a,
        b"prstGeom></",
        x,
        b"spPr></",
        x,
        b"pic><",
        x,
        b"clientData/></",
        x,
        b"oneCellAnchor>",
    ]
    .concat()
}

/// A created drawing part's bytes (`specs/0022-comments-and-images.md`,
/// "Images: the parts"): the declaration on its own line, then the root
/// binding `xdr` and `a` with the anchors, on one line.
#[cfg(feature = "images")]
fn drawing_part_bytes(anchors: &[u8]) -> Vec<u8> {
    [
        b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<xdr:wsDr xmlns:xdr=\""
            .as_slice(),
        SPREADSHEET_DRAWING[0],
        b"\" xmlns:a=\"",
        DRAWINGML[0],
        b"\">",
        anchors,
        b"</xdr:wsDr>",
    ]
    .concat()
}

/// A picture an anchor of a drawing part holds, as [`read_images`] finds
/// it: the anchor's kind and `from`, and the `blip`'s `r:embed`.
#[cfg(feature = "images")]
struct DrawingPicture {
    anchor: Anchor,
    from: (u32, u32),
    embed: Vec<u8>,
}

/// The namespaces [`read_images`] tells apart.
#[cfg(feature = "images")]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
    Drawing,
    Main,
    Compat,
    Other,
}

/// An element open while [`read_images`] reads: its namespace and local
/// name, how many `Choice` children an `AlternateContent` has met, and
/// whether a `Choice` is its `AlternateContent`'s first.
#[cfg(feature = "images")]
struct Step {
    space: Space,
    local: Vec<u8>,
    choices: u32,
    first: bool,
}

#[cfg(feature = "images")]
impl Step {
    fn is(&self, space: Space, local: &[u8]) -> bool {
        self.space == space && self.local == local
    }
}

/// An anchor being read: its kind and depth; the depth of its `from` while
/// open, the `col` and `row` numbers and the field whose text is open; the
/// depth of the picture chosen while open and whether one was; and what the
/// picture's first `blip` carried (`Some(None)` for none).
#[cfg(feature = "images")]
struct AnchorRead {
    kind: Anchor,
    depth: usize,
    from: Option<usize>,
    column: Option<u32>,
    row: Option<u32>,
    field: Option<(bool, String)>,
    pic: Option<usize>,
    chose: bool,
    embed: Option<Option<Vec<u8>>>,
}

/// The pictures of a drawing part, streamed once
/// (`specs/0022-comments-and-images.md`, "Images: the calls", and P3 ruling
/// 1): each root anchor, or the first anchor in a root `AlternateContent`'s
/// first `Choice`, that holds a `pic` directly, in a `grpSp` or in an
/// `AlternateContent`'s first `Choice`, the first found, gives one. A root
/// other than `wsDr`, a picture without a `blip` or whose `blip` has no
/// `r:embed`, and a one- or two-cell anchor whose `from` lacks a `col` or
/// `row` number cannot be read.
#[cfg(feature = "images")]
fn read_images(input: impl Read, index: usize) -> Result<Vec<DrawingPicture>, Error> {
    let unreadable = |error: Error| match error {
        Error::Xml(_) => DRAWING_UNREADABLE,
        other => other,
    };
    let mut stream = Stream::new(input, index).map_err(unreadable)?;
    let mut scope = Scope::new(index);
    let mut path: Vec<Step> = Vec::new();
    let mut pictures = Vec::new();
    let mut anchor: Option<AnchorRead> = None;
    // Whether the open root `Choice` has given its anchor.
    let mut choice_taken = false;
    while let Some(token) = stream.next().map_err(unreadable)? {
        let ended = match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let space = match namespace {
                    Some(namespace) if SPREADSHEET_DRAWING.contains(&namespace) => Space::Drawing,
                    Some(namespace) if DRAWINGML.contains(&namespace) => Space::Main,
                    Some(namespace) if namespace == MARKUP_COMPATIBILITY => Space::Compat,
                    _ => Space::Other,
                };
                let mut step = Step {
                    space,
                    local: local.to_vec(),
                    choices: 0,
                    first: false,
                };
                if step.is(Space::Compat, b"Choice")
                    && let Some(parent) = path.last_mut()
                    && parent.is(Space::Compat, b"AlternateContent")
                {
                    parent.choices += 1;
                    step.first = parent.choices == 1;
                }
                let depth = path.len();
                let kind = match (space, local) {
                    (Space::Drawing, b"oneCellAnchor") => Some(Anchor::OneCell),
                    (Space::Drawing, b"twoCellAnchor") => Some(Anchor::TwoCell),
                    (Space::Drawing, b"absoluteAnchor") => Some(Anchor::Absolute),
                    _ => None,
                };
                if depth == 0 {
                    if !step.is(Space::Drawing, b"wsDr") {
                        return Err(DRAWING_UNREADABLE);
                    }
                } else if let Some(read) = anchor.as_mut() {
                    let below = &path[read.depth + 1..];
                    if depth == read.depth + 1 && step.is(Space::Drawing, b"from") {
                        read.from = Some(depth);
                    } else if read.from.is_some_and(|from| depth == from + 1)
                        && space == Space::Drawing
                        && (local == b"col" || local == b"row")
                    {
                        read.field = Some((local == b"row", String::new()));
                    }
                    let in_container = below
                        .first()
                        .is_some_and(|step| step.is(Space::Drawing, b"grpSp"))
                        || (below.len() >= 2
                            && below[0].is(Space::Compat, b"AlternateContent")
                            && below[1].is(Space::Compat, b"Choice")
                            && below[1].first);
                    if !read.chose
                        && step.is(Space::Drawing, b"pic")
                        && (below.is_empty() || in_container)
                    {
                        read.chose = true;
                        read.pic = Some(depth);
                    } else if read.pic.is_some_and(|pic| depth > pic)
                        && read.embed.is_none()
                        && step.is(Space::Main, b"blip")
                    {
                        let mut embed = None;
                        for attribute in start.attributes() {
                            let attribute = attribute.map_err(|_| Error::Xml(index))?;
                            if attribute.key.as_namespace_binding().is_some() {
                                continue;
                            }
                            let (namespace, name) = scope.resolve_attribute(attribute.key)?;
                            if name == b"embed" && is_relationships_attribute(namespace) {
                                embed = Some(attribute.value.as_bytes().to_vec());
                            }
                        }
                        read.embed = Some(embed);
                    }
                } else if let Some(kind) = kind {
                    let in_root_choice = depth == 3
                        && path[1].is(Space::Compat, b"AlternateContent")
                        && path[2].is(Space::Compat, b"Choice")
                        && path[2].first
                        && !choice_taken;
                    if depth == 1 || in_root_choice {
                        choice_taken |= in_root_choice;
                        anchor = Some(AnchorRead {
                            kind,
                            depth,
                            from: None,
                            column: None,
                            row: None,
                            field: None,
                            pic: None,
                            chose: false,
                            embed: None,
                        });
                    }
                } else if depth == 1 && step.is(Space::Compat, b"AlternateContent") {
                    choice_taken = false;
                }
                path.push(step);
                matches!(token.event, Event::Empty(_))
            }
            Event::Text(ref chunk) => {
                if let Some((_, text)) = anchor.as_mut().and_then(|read| read.field.as_mut()) {
                    text.push_str(chunk);
                }
                false
            }
            Event::End(_) => true,
            _ => false,
        };
        if !ended {
            continue;
        }
        path.pop();
        scope.end();
        let depth = path.len();
        let Some(read) = anchor.as_mut() else {
            continue;
        };
        if let Some((row, text)) = read.field.take() {
            let number =
                digits_value(text.trim().as_bytes()).and_then(|value| u32::try_from(value).ok());
            if row {
                read.row = number;
            } else {
                read.column = number;
            }
        }
        if read.from == Some(depth) {
            read.from = None;
        }
        if read.pic == Some(depth) {
            read.pic = None;
        }
        if read.depth == depth {
            if read.chose {
                let Some(Some(embed)) = read.embed.take() else {
                    return Err(DRAWING_UNREADABLE);
                };
                let from = match (read.kind, read.column, read.row) {
                    (Anchor::Absolute, _, _) => (0, 0),
                    (_, Some(column), Some(row)) => (column, row),
                    _ => return Err(DRAWING_UNREADABLE),
                };
                pictures.push(DrawingPicture {
                    anchor: read.kind,
                    from,
                    embed,
                });
            }
            anchor = None;
        }
    }
    Ok(pictures)
}

/// The `<conditionalFormatting>` of a worksheet name, its place in
/// [`WORKSHEET_CHILDREN`].
#[cfg(feature = "rules")]
const CONDITIONAL_FORMATTING: &[u8] = b"conditionalFormatting";

/// The conditional formats' part of a worksheet rewrite
/// (`specs/0020-validation-and-conditional-formatting.md`, "Conditional
/// formatting", and P2 rulings 4 and 5): each `<conditionalFormatting>` held
/// while it is read, bounded by that one element, and skipped whole when a
/// removal names its list; the highest priority the kept ones carry; and the
/// additions, written once before the first root child that follows them in
/// schema order. The Cargo feature `rules`.
#[cfg(feature = "rules")]
struct FormatsPass<'w> {
    edits: &'w [RuleEdit],
    touched: bool,
    needed: bool,
    written: bool,
    held: Option<FormatHold>,
    /// The highest `priority` of the kept elements' rules.
    highest: u32,
    /// For each edit, whether a removal found the part's element.
    found: Vec<bool>,
}

/// A `<conditionalFormatting>` being read: its depth, its bytes so far, its
/// parsed `sqref`, whether a rule holds an `extLst`, its rules' highest
/// priority, and whether a priority did not parse.
#[cfg(feature = "rules")]
struct FormatHold {
    depth: usize,
    bytes: Vec<u8>,
    areas: Vec<Area>,
    extended: bool,
    highest: u32,
    unparsed: bool,
}

#[cfg(feature = "rules")]
impl<'w> FormatsPass<'w> {
    fn new(edits: &'w [RuleEdit]) -> Self {
        FormatsPass {
            edits,
            touched: !edits.is_empty(),
            needed: edits.iter().any(|edit| matches!(edit, RuleEdit::Add(_))),
            written: false,
            held: None,
            highest: 0,
            found: vec![false; edits.len()],
        }
    }

    /// The additions, each a `<conditionalFormatting>` holding its one
    /// `<cfRule>`, with `prefix`; a priority not given is the highest kept
    /// plus one, and one more for each earlier addition without one.
    fn elements(&self, prefix: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut next = u64::from(self.highest);
        for edit in self.edits {
            let RuleEdit::Add(added) = edit else {
                continue;
            };
            let priority = match added.priority {
                Some(priority) => u64::from(priority),
                None => {
                    next += 1;
                    next
                }
            };
            out.extend_from_slice(
                &[b"<".as_slice(), prefix, b"conditionalFormatting sqref=\""].concat(),
            );
            out.extend_from_slice(&sqref_value(&added.areas));
            out.extend_from_slice(&[b"\"><".as_slice(), prefix, b"cfRule"].concat());
            out.extend_from_slice(&added.before);
            out.extend_from_slice(format!(" priority=\"{priority}\"").as_bytes());
            out.extend_from_slice(&added.after);
            if added.formulas.is_empty() && added.visual.is_none() {
                out.extend_from_slice(b"/>");
            } else {
                out.push(b'>');
                for formula in &added.formulas {
                    out.extend_from_slice(&[b"<".as_slice(), prefix, b"formula>"].concat());
                    out.extend_from_slice(formula.as_bytes());
                    out.extend_from_slice(&[b"</".as_slice(), prefix, b"formula>"].concat());
                }
                if let Some(visual) = &added.visual {
                    out.extend_from_slice(&visual.element(prefix));
                }
                out.extend_from_slice(&[b"</".as_slice(), prefix, b"cfRule>"].concat());
            }
            out.extend_from_slice(&[b"</".as_slice(), prefix, b"conditionalFormatting>"].concat());
        }
        out
    }

    /// The additions, when they are not written yet and a root child at
    /// `position` follows `conditionalFormatting` in schema order.
    fn create_before(&mut self, position: usize, prefix: &[u8]) -> Option<Vec<u8>> {
        let at = schema_position(CONDITIONAL_FORMATTING).unwrap_or(0);
        if at < position {
            return self.create_at_end(prefix);
        }
        None
    }

    /// The additions, when they are not written yet.
    fn create_at_end(&mut self, prefix: &[u8]) -> Option<Vec<u8>> {
        if !self.needed || self.written {
            return None;
        }
        self.written = true;
        Some(self.elements(prefix))
    }

    /// A `<conditionalFormatting>` start tag the part has: its `sqref` read
    /// and the element held, or, for an empty one, settled at once.
    fn open(
        &mut self,
        raw: &[u8],
        depth: usize,
        empty: bool,
        index: usize,
    ) -> Result<Vec<u8>, Error> {
        let areas = attribute_value(raw, b"sqref", index)?
            .and_then(parsed_list)
            .ok_or(RULE_UNPARSED)?;
        let hold = FormatHold {
            depth,
            bytes: raw.to_vec(),
            areas,
            extended: false,
            highest: 0,
            unparsed: false,
        };
        if empty {
            return self.settle(hold);
        }
        self.held = Some(hold);
        Ok(Vec::new())
    }

    /// A start or empty tag inside the held element: a `<cfRule>`'s
    /// priority read, an `extLst` under a rule noted, the bytes kept.
    fn child(
        &mut self,
        raw: &[u8],
        (local, ours, depth): (&[u8], bool, usize),
        index: usize,
    ) -> Result<(), Error> {
        let Some(hold) = self.held.as_mut() else {
            return Ok(());
        };
        if depth == hold.depth + 1 && ours && local == b"cfRule" {
            let priority = attribute_value(raw, b"priority", index)?
                .and_then(|value| str::from_utf8(value).ok())
                .and_then(crate::cells::decimal);
            match priority {
                Some(priority) => hold.highest = hold.highest.max(priority),
                None => hold.unparsed = true,
            }
        } else if depth == hold.depth + 2 && ours && local == b"extLst" {
            hold.extended = true;
        }
        hold.bytes.extend_from_slice(raw);
        Ok(())
    }

    /// An end tag or any other token inside the held element; at the
    /// element's own end tag, the bytes it is written as.
    fn held(&mut self, raw: &[u8], end_depth: Option<usize>) -> Result<Option<Vec<u8>>, Error> {
        let Some(hold) = self.held.as_mut() else {
            return Ok(None);
        };
        hold.bytes.extend_from_slice(raw);
        if end_depth != Some(hold.depth) {
            return Ok(Some(Vec::new()));
        }
        match self.held.take() {
            Some(hold) => Ok(Some(self.settle(hold)?)),
            None => Ok(None),
        }
    }

    /// A whole element read: skipped when a removal names its list (refused
    /// when a rule holds an extension), else kept, a priority that does not
    /// parse refused and its highest counted.
    fn settle(&mut self, hold: FormatHold) -> Result<Vec<u8>, Error> {
        let mut gone = false;
        for (which, edit) in self.edits.iter().enumerate() {
            if matches!(edit, RuleEdit::Remove(list) if *list == hold.areas) {
                self.found[which] = true;
                gone = true;
            }
        }
        if gone {
            if hold.extended {
                return Err(RULE_EXTENDED);
            }
            return Ok(Vec::new());
        }
        if hold.unparsed {
            return Err(RULE_UNPARSED);
        }
        self.highest = self.highest.max(hold.highest);
        Ok(hold.bytes)
    }

    /// After the pass: a removal the part had no element for is refused.
    fn finish(&self) -> Result<(), Error> {
        for (which, edit) in self.edits.iter().enumerate() {
            if matches!(edit, RuleEdit::Remove(_)) && !self.found[which] {
                return Err(NO_SUCH_RULE);
            }
        }
        Ok(())
    }
}

/// Reads a worksheet part's conditional formats for
/// [`Package::conditional_formats`]: every `<cfRule>` of every main-namespace
/// `<conditionalFormatting>` among the root's children, in part order, with
/// its `formula` children's texts; every other root child is skipped whole.
#[cfg(feature = "rules")]
fn read_rules(input: impl Read, index: usize) -> Result<Vec<RuleEntry>, Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut areas: Option<Vec<Area>> = None;
    let mut current: Option<RuleEntry> = None;
    let mut out = Vec::new();
    loop {
        // A formula whose text is read once the token is let go.
        let mut formula = false;
        let Some(token) = stream.next()? else {
            break;
        };
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                let skip = match depth {
                    1 => {
                        if !ours || local != b"worksheet" {
                            return Err(Error::Package(
                                "the worksheet part's root is not worksheet",
                            ));
                        }
                        false
                    }
                    2 if ours && local == CONDITIONAL_FORMATTING => {
                        let list = attribute_value(token.raw, b"sqref", index)?
                            .and_then(parsed_list)
                            .ok_or(RULE_UNPARSED)?;
                        areas = (!empty).then_some(list);
                        false
                    }
                    3 if ours && local == b"cfRule" => match areas.as_ref() {
                        Some(list) => {
                            let entry = read_rule(start, &scope, list.clone(), index)?;
                            if empty {
                                out.push(entry);
                            } else {
                                current = Some(entry);
                            }
                            false
                        }
                        None => true,
                    },
                    4 if ours && current.is_some() && local == b"formula" => {
                        if empty {
                            if let Some(entry) = current.as_mut() {
                                entry.formulas.push(String::new());
                            }
                        } else {
                            formula = true;
                        }
                        false
                    }
                    4 if ours && local == b"extLst" => {
                        if let Some(entry) = current.as_mut() {
                            entry.extended = true;
                        }
                        true
                    }
                    _ => true,
                };
                if skip && !empty {
                    skip_element(&mut stream, &mut scope)?;
                    scope.end();
                    depth -= 1;
                    continue;
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                if depth == 3
                    && let Some(entry) = current.take()
                {
                    out.push(entry);
                }
                if depth == 2 {
                    areas = None;
                }
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
        if formula {
            // `text_of` reads up to and including the end tag.
            let text = crate::cells::text_of(&mut stream, &mut scope, index)?;
            depth -= 1;
            if let Some(entry) = current.as_mut() {
                entry.formulas.push(text);
            }
        }
    }
    Ok(out)
}

/// The operators of ECMA-376 Part 1, 18.18.15 that [`Operator`] does not
/// name, which a text rule carries (P2 ruling 6).
#[cfg(feature = "rules")]
const TEXT_OPERATORS: [&[u8]; 4] = [b"containsText", b"notContains", b"beginsWith", b"endsWith"];

/// One `<cfRule>` start tag read: its unprefixed attributes, a prefix
/// nothing binds refused, and an `operator`, `dxfId` or `priority` that does
/// not parse refused (P2 rulings 4 and 6).
#[cfg(feature = "rules")]
fn read_rule(
    start: &BytesStart<'_>,
    scope: &Scope,
    areas: Vec<Area>,
    index: usize,
) -> Result<RuleEntry, Error> {
    let mut entry = RuleEntry {
        areas,
        kind: String::new(),
        operator: None,
        formulas: Vec::new(),
        dxf: None,
        priority: 0,
        stop_if_true: false,
        extended: false,
    };
    let mut priority = None;
    let number = |value: &[u8]| {
        str::from_utf8(value)
            .ok()
            .and_then(crate::cells::decimal)
            .ok_or(RULE_UNPARSED)
    };
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        if namespace.is_some() {
            continue;
        }
        let value = attribute.value.as_bytes();
        match local {
            b"type" => {
                entry.kind = attribute
                    .normalized_value(XmlVersion::Implicit1_0)
                    .map_err(|_| Error::Xml(index))?
                    .into_owned();
            }
            // An operator of 18.18.15 that `Operator` does not name reads as
            // none, the kind saying what the rule is (P2 ruling 6).
            b"operator" if TEXT_OPERATORS.contains(&value) => entry.operator = None,
            b"operator" => {
                entry.operator = Some(named(&Operator::ALL, value).ok_or(RULE_UNPARSED)?);
            }
            b"dxfId" => entry.dxf = Some(number(value)?),
            b"priority" => priority = Some(number(value)?),
            b"stopIfTrue" => entry.stop_if_true = flag_on(value),
            _ => {}
        }
    }
    entry.priority = priority.ok_or(RULE_UNPARSED)?;
    Ok(entry)
}

/// Whether a flag attribute's value is on: `1` or `true`.
#[cfg(feature = "rules")]
fn flag_on(value: &[u8]) -> bool {
    matches!(value, b"1" | b"true")
}

/// Reads a worksheet part's data validations for [`Package::validations`]:
/// every `<dataValidation>` of the root's main-namespace
/// `<dataValidations>`, in part order, with its `formula1` and `formula2`;
/// every other root child is skipped whole.
#[cfg(feature = "rules")]
fn read_validations(input: impl Read, index: usize) -> Result<Vec<ValidationEntry>, Error> {
    let mut stream = Stream::new(input, index)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut inside = false;
    let mut current: Option<ValidationEntry> = None;
    let mut out = Vec::new();
    loop {
        // A formula child whose text is read once the token is let go:
        // `Some(true)` for `formula2`.
        let mut formula: Option<bool> = None;
        let Some(token) = stream.next()? else {
            break;
        };
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                let skip = match depth {
                    1 => {
                        if !ours || local != b"worksheet" {
                            return Err(Error::Package(
                                "the worksheet part's root is not worksheet",
                            ));
                        }
                        false
                    }
                    2 => {
                        inside = ours && local == DATA_VALIDATIONS && !empty;
                        !inside
                    }
                    3 if inside && ours && local == b"dataValidation" => {
                        let entry = read_validation(start, &scope, index)?;
                        if empty {
                            out.push(entry);
                        } else {
                            current = Some(entry);
                        }
                        false
                    }
                    4 if ours
                        && (local == b"formula1" || local == b"formula2")
                        && current.is_some() =>
                    {
                        let second = local == b"formula2";
                        if empty {
                            if let Some(entry) = current.as_mut() {
                                *(if second {
                                    &mut entry.formula2
                                } else {
                                    &mut entry.formula1
                                }) = Some(String::new());
                            }
                        } else {
                            formula = Some(second);
                        }
                        false
                    }
                    _ => true,
                };
                if skip && !empty {
                    skip_element(&mut stream, &mut scope)?;
                    scope.end();
                    depth -= 1;
                    continue;
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                if depth == 3
                    && let Some(entry) = current.take()
                {
                    out.push(entry);
                }
                if depth == 2 {
                    inside = false;
                }
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
        if let Some(second) = formula {
            // `text_of` reads up to and including the end tag.
            let text = crate::cells::text_of(&mut stream, &mut scope, index)?;
            depth -= 1;
            if let Some(entry) = current.as_mut() {
                if second {
                    entry.formula2 = Some(text);
                } else {
                    entry.formula1 = Some(text);
                }
            }
        }
    }
    Ok(out)
}

/// One `<dataValidation>` start tag read: its unprefixed attributes, a
/// prefix nothing binds refused, an enumeration value it does not name
/// refused with the overlap message (P1 ruling 5).
#[cfg(feature = "rules")]
fn read_validation(
    start: &BytesStart<'_>,
    scope: &Scope,
    index: usize,
) -> Result<ValidationEntry, Error> {
    let mut areas: Option<Vec<Area>> = None;
    let mut entry = ValidationEntry {
        areas: Vec::new(),
        kind: None,
        operator: None,
        formula1: None,
        formula2: None,
        allow_blank: false,
        hide_drop_down: false,
        input: None,
        error: None,
    };
    let (mut show_input, mut show_error) = (false, false);
    let mut style = ErrorStyle::Stop;
    let mut texts: [String; 4] = [String::new(), String::new(), String::new(), String::new()];
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        if namespace.is_some() {
            continue;
        }
        let value = attribute.value.as_bytes();
        let text = |at: usize, texts: &mut [String; 4]| -> Result<(), Error> {
            texts[at] = attribute
                .normalized_value(XmlVersion::Implicit1_0)
                .map_err(|_| Error::Xml(index))?
                .into_owned();
            Ok(())
        };
        match local {
            b"sqref" => areas = parsed_list(value),
            b"type" if value == b"none" => entry.kind = None,
            b"type" => {
                entry.kind = Some(named(&ValidationKind::ALL, value).ok_or(VALIDATION_OVERLAP)?);
            }
            b"operator" => {
                entry.operator = Some(named(&Operator::ALL, value).ok_or(VALIDATION_OVERLAP)?);
            }
            b"errorStyle" => style = named(&ErrorStyle::ALL, value).ok_or(VALIDATION_OVERLAP)?,
            b"allowBlank" => entry.allow_blank = flag_on(value),
            b"showDropDown" => entry.hide_drop_down = flag_on(value),
            b"showInputMessage" => show_input = flag_on(value),
            b"showErrorMessage" => show_error = flag_on(value),
            b"promptTitle" => text(0, &mut texts)?,
            b"prompt" => text(1, &mut texts)?,
            b"errorTitle" => text(2, &mut texts)?,
            b"error" => text(3, &mut texts)?,
            _ => {}
        }
    }
    entry.areas = areas.ok_or(VALIDATION_OVERLAP)?;
    let [prompt_title, prompt, error_title, error] = texts;
    if show_input {
        entry.input = Some((prompt_title, prompt));
    }
    if show_error {
        entry.error = Some((style, error_title, error));
    }
    Ok(entry)
}

/// One insertion or deletion of rows (`specs/0018-rows-and-columns.md`,
/// "The shift rule"): `count` indices at `at`, on an axis whose last index
/// is `limit`.
#[cfg(feature = "rowcol")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Shift {
    insert: bool,
    at: u32,
    count: u32,
    limit: u32,
}

/// What the shift rule does to an interval `[a, b]` on its axis.
#[cfg(feature = "rowcol")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Moved {
    /// Unchanged or moved whole, to these bounds (which may pass the
    /// grid).
    Whole(u64, u64),
    /// Grown by an insertion inside it, or cut by a deletion that
    /// overlaps it, to these bounds.
    Changed(u64, u64),
    /// Wholly inside a deletion.
    Gone,
}

#[cfg(feature = "rowcol")]
impl Shift {
    /// Where the index `i` goes, `None` when a deletion removes it. The
    /// answer may pass the grid; the caller refuses or clips.
    fn index(&self, i: u32) -> Option<u64> {
        let (i, k, n) = (u64::from(i), u64::from(self.at), u64::from(self.count));
        if i < k {
            Some(i)
        } else if self.insert {
            Some(i + n)
        } else if i < k + n {
            None
        } else {
            Some(i - n)
        }
    }

    /// What happens to the interval `[a, b]`, `a <= b`.
    fn interval(&self, a: u32, b: u32) -> Moved {
        let (a, b) = (u64::from(a), u64::from(b));
        let (k, n) = (u64::from(self.at), u64::from(self.count));
        if b < k {
            return Moved::Whole(a, b);
        }
        if self.insert {
            return if a >= k {
                Moved::Whole(a + n, b + n)
            } else {
                Moved::Changed(a, b + n)
            };
        }
        let last = k + n - 1;
        if a > last {
            Moved::Whole(a - n, b - n)
        } else if a >= k && b <= last {
            Moved::Gone
        } else {
            let start = if a < k { a } else { k };
            let end = if b > last { b - n } else { k - 1 };
            Moved::Changed(start, end)
        }
    }

    /// An area's rows through the rule, clipped to the grid: `None` when the
    /// rule removes it or it lies wholly past the grid.
    fn area_clipped(&self, area: Area) -> Option<Area> {
        let (a, b) = match self.interval(area.first.row, area.last.row) {
            Moved::Whole(a, b) | Moved::Changed(a, b) => (a, b),
            Moved::Gone => return None,
        };
        let limit = u64::from(self.limit);
        if a > limit {
            return None;
        }
        Some(Area {
            first: CellRef {
                row: a as u32,
                col: area.first.col,
            },
            last: CellRef {
                row: b.min(limit) as u32,
                col: area.last.col,
            },
        })
    }

    /// Whether an index is at or after the point in the input, so a formula
    /// there is moved or removed (`specs/0018-rows-and-columns.md`, P2
    /// ruling 7).
    fn reaches(&self, i: u32) -> bool {
        i >= self.at
    }
}

/// A [`Shift`] and the axis it runs on (`specs/0018-rows-and-columns.md`, P3
/// ruling 5): the arithmetic is the row form's, and an area on the column
/// axis goes through it transposed.
#[cfg(feature = "rowcol")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AxisShift {
    shift: Shift,
    columns: bool,
}

#[cfg(feature = "rowcol")]
impl AxisShift {
    /// A cell's index on the axis.
    fn of(&self, cell: CellRef) -> u32 {
        if self.columns { cell.col } else { cell.row }
    }

    /// A cell with its index on the axis replaced.
    fn moved(&self, cell: CellRef, to: u32) -> CellRef {
        if self.columns {
            CellRef {
                row: cell.row,
                col: to,
            }
        } else {
            CellRef {
                row: to,
                col: cell.col,
            }
        }
    }

    /// An area in the row form the arithmetic reads, transposed on the
    /// column axis; applied twice it gives the area back.
    fn rowwise(&self, area: Area) -> Area {
        if !self.columns {
            return area;
        }
        let flip = |cell: CellRef| CellRef {
            row: cell.col,
            col: cell.row,
        };
        Area {
            first: flip(area.first),
            last: flip(area.last),
        }
    }

    /// [`Shift::area_clipped`] on the axis.
    fn area_clipped(&self, area: Area) -> Option<Area> {
        self.shift
            .area_clipped(self.rowwise(area))
            .map(|moved| self.rowwise(moved))
    }

    /// [`Shift::index`] on the axis.
    fn index(&self, i: u32) -> Option<u64> {
        self.shift.index(i)
    }

    /// [`Shift::reaches`] on the axis.
    fn reaches(&self, i: u32) -> bool {
        self.shift.reaches(i)
    }

    /// The axis's last index.
    fn limit(&self) -> u64 {
        u64::from(self.shift.limit)
    }

    /// An interval `[a, b]` through the rule and the clip: `None` when the
    /// rule removes it or it lies wholly past the grid.
    fn interval_clipped(&self, a: u32, b: u32) -> Option<(u32, u32)> {
        let (a, b) = match self.shift.interval(a, b) {
            Moved::Whole(a, b) | Moved::Changed(a, b) => (a, b),
            Moved::Gone => return None,
        };
        if a > self.limit() {
            return None;
        }
        Some((a as u32, b.min(self.limit()) as u32))
    }
}

/// A `<row>` start tag with its `spans` through the column rule (P3 ruling
/// 2): each `a:b` item moved, grown, cut or clipped, an item the rule
/// removes dropped, the attribute dropped when none is left, the tag's own
/// bytes when nothing changes; an item that does not parse is refused.
#[cfg(feature = "rowcol")]
fn spans_tag(raw: &[u8], shift: &AxisShift, index: usize) -> Result<Vec<u8>, Error> {
    let Some(value) = attribute_value(raw, b"spans", index)? else {
        return Ok(raw.to_vec());
    };
    let mut kept: Vec<Vec<u8>> = Vec::new();
    let mut changed = false;
    for item in value
        .split(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
        .filter(|item| !item.is_empty())
    {
        let bounds = str::from_utf8(item).ok().and_then(|text| {
            let (a, b) = text.split_once(':')?;
            let (a, b) = (decimal(a)?, decimal(b)?);
            (a >= 1 && a <= b).then_some((a, b))
        });
        let (a, b) = bounds.ok_or(RANGE_UNPARSED)?;
        match shift.interval_clipped(a, b) {
            Some(moved) if moved == (a, b) => kept.push(item.to_vec()),
            Some((a, b)) => {
                changed = true;
                kept.push(format!("{a}:{b}").into_bytes());
            }
            None => changed = true,
        }
    }
    if !changed {
        Ok(raw.to_vec())
    } else if kept.is_empty() {
        without_attribute(raw, b"spans", index)
    } else {
        with_attribute(raw, b"spans", &kept.join(&b' '), index)
    }
}

/// A `<col>` start tag through the column rule: `None` when the rule
/// removes its interval or moves it wholly past the grid, else the tag
/// with `min` and `max` moved, grown, cut or clipped, its own bytes when
/// they do not change. A malformed `min` or `max` is refused with the cols
/// message (P3 ruling 1).
#[cfg(feature = "rowcol")]
fn col_tag(raw: &[u8], shift: &AxisShift, index: usize) -> Result<Option<Vec<u8>>, Error> {
    let (min, max) = col_interval(raw, index)?;
    Ok(match shift.interval_clipped(min, max) {
        None => None,
        Some(moved) if moved == (min, max) => Some(raw.to_vec()),
        Some((a, b)) => Some(col_retagged(raw, a, b, index)?),
    })
}

/// A range the rule reads, parsed, or the P2 ruling 2 refusal.
#[cfg(feature = "rowcol")]
fn shift_area(value: &[u8]) -> Result<Area, Error> {
    Area::parse(value).ok_or(RANGE_UNPARSED)
}

/// A space-separated list of ranges through the rule: `None` when every
/// range is removed, else the list as P2 ruling 4 writes it, the input's
/// own bytes when nothing changed.
#[cfg(feature = "rowcol")]
fn shifted_list(value: &[u8], shift: &AxisShift) -> Result<Option<Vec<u8>>, Error> {
    let mut kept: Vec<Vec<u8>> = Vec::new();
    let mut changed = false;
    for range in value
        .split(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
        .filter(|range| !range.is_empty())
    {
        let area = shift_area(range)?;
        match shift.area_clipped(area) {
            Some(moved) if moved == area => kept.push(range.to_vec()),
            Some(moved) => {
                changed = true;
                kept.push(moved.a1().into_bytes());
            }
            None => changed = true,
        }
    }
    if kept.is_empty() {
        return Ok(None);
    }
    Ok(Some(if changed {
        kept.join(&b' ')
    } else {
        value.to_vec()
    }))
}

/// A start tag with the unprefixed attribute `name`, when it has one, run
/// through the list rule: `Ok(None)` when the list empties, the tag
/// unchanged when it has no such attribute.
#[cfg(feature = "rowcol")]
fn list_tag(
    raw: &[u8],
    name: &[u8],
    shift: &AxisShift,
    index: usize,
) -> Result<Option<Vec<u8>>, Error> {
    let Some(value) = attribute_value(raw, name, index)? else {
        return Ok(Some(raw.to_vec()));
    };
    match shifted_list(value, shift)? {
        None => Ok(None),
        Some(list) if list == value => Ok(Some(raw.to_vec())),
        Some(list) => Ok(Some(with_attribute(raw, name, &list, index)?)),
    }
}

/// A counted container held whole while its children go through the rule:
/// `mergeCells` or `dataValidations` (P1 verification ruling 4).
#[cfg(feature = "rowcol")]
struct CountedBuffer {
    depth: usize,
    start: Vec<u8>,
    name: Vec<u8>,
    content: Vec<u8>,
    kept: u64,
}

#[cfg(feature = "rowcol")]
impl CountedBuffer {
    /// The container with `count` set to the children it keeps, or nothing
    /// when it keeps none.
    fn written(&self, index: usize) -> Result<Vec<u8>, Error> {
        if self.kept == 0 {
            return Ok(Vec::new());
        }
        let mut out = with_attribute(
            &self.start,
            b"count",
            self.kept.to_string().as_bytes(),
            index,
        )?;
        out.extend_from_slice(&self.content);
        out.extend_from_slice(&[b"</".as_slice(), &self.name, b">"].concat());
        Ok(out)
    }
}

/// An open `<hyperlinks>` under the rule (P2 ruling 8), or an open `<cols>`
/// under the column rule: its start tag, written when the first kept child
/// arrives, and the bytes held since the last kept child, so an emptied
/// container is dropped without being held whole.
#[cfg(feature = "rowcol")]
struct LinksBuffer {
    depth: usize,
    start: Vec<u8>,
    started: bool,
    held: Vec<u8>,
    /// The child the rule reads: `hyperlink` or `col`.
    child: &'static [u8],
}

/// Rewrites one worksheet part through one insertion or deletion of rows,
/// and says whether a formula was moved or removed.
///
/// `specs/0018-rows-and-columns.md`, "The shift rule", with the P2 rulings:
/// every `<row r>` and `<c r>` is retagged, a deleted row skipped whole; a
/// shared, array or data table `ref` moved whole, or refused when grown or
/// cut; `<dimension>`, every `<mergeCell>` and the four list attributes
/// (`conditionalFormatting sqref`, `dataValidation sqref`, `hyperlink ref`,
/// `autoFilter ref`) through the rule, dropped when it removes them, the
/// counts following and an emptied container dropped. Every other byte is
/// the input's.
#[cfg(feature = "rowcol")]
fn shift_worksheet(
    archive: &mut Core,
    index: usize,
    shift: &AxisShift,
) -> Result<(TempFile, Replacement, bool), Error> {
    shift_worksheet_from(archive.read(index)?, index, shift)
}

/// [`shift_worksheet`]'s work on any reader of a worksheet's bytes.
#[cfg(feature = "rowcol")]
fn shift_worksheet_from(
    input: impl Read,
    index: usize,
    shift: &AxisShift,
) -> Result<(TempFile, Replacement, bool), Error> {
    let (temp, file) = temp_for_part()?;
    let mut out = BufWriter::new(file);
    let mut stream = Stream::new(input, index)?;
    if stream.bom() {
        out.write_all(&BYTE_ORDER_MARK)?;
    }
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut root = false;
    let mut data: Option<usize> = None;
    let mut seen_data = false;
    // The open row's input number and its last column, for the order rule.
    let mut row: Option<u32> = None;
    let mut last_row = 0u32;
    let mut last_col = 0u32;
    // The depth of a deleted row being read: its tokens are not written,
    // but its formulas are still read.
    let mut dropping: Option<usize> = None;
    // The open cell's depth, and whether it is at or after the point.
    let mut cell: Option<(usize, bool)> = None;
    let mut moved_formula = false;
    let mut counted: Option<CountedBuffer> = None;
    // The open `<hyperlinks>`: its depth, its start tag and name, whether
    // the start tag has been written, and the bytes held since the last
    // kept child (P2 ruling 8).
    let mut links: Option<LinksBuffer> = None;

    // Where a token's bytes go: nowhere inside a deleted row, the counted
    // container's content, the held hyperlinks bytes, or the output.
    macro_rules! emit {
        ($bytes:expr) => {{
            let bytes: &[u8] = $bytes;
            if dropping.is_some() {
            } else if let Some(buffer) = counted.as_mut() {
                buffer.content.extend_from_slice(bytes);
            } else if let Some(links) = links.as_mut() {
                links.held.extend_from_slice(bytes);
            } else {
                out.write_all(bytes)?;
            }
        }};
    }

    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let ours = is_main(namespace);
                if !root {
                    if !ours || local != b"worksheet" {
                        return Err(Error::Package("the worksheet part's root is not worksheet"));
                    }
                    root = true;
                    out.write_all(token.raw)?;
                } else if let Some((at, reaches)) = cell {
                    // Inside a cell: its own `<f>` is read, whether the row
                    // is kept or deleted.
                    let mut raw = token.raw.to_vec();
                    if depth == at + 1 && ours && local == b"f" {
                        moved_formula |= reaches;
                        raw = shifted_formula(token.raw, shift, dropping.is_some(), index)?;
                    }
                    emit!(&raw);
                } else if data.is_some_and(|at| depth == at + 1) && ours && local == b"row" {
                    let number = explicit_row(start, &scope, index)?;
                    if number <= last_row {
                        return Err(OUT_OF_ORDER);
                    }
                    last_row = number;
                    if shift.columns {
                        out.write_all(&spans_tag(token.raw, shift, index)?)?;
                        if !empty {
                            row = Some(number);
                            last_col = 0;
                        }
                    } else {
                        match shift.index(number) {
                            None => {
                                if !empty {
                                    dropping = Some(depth);
                                    row = Some(number);
                                    last_col = 0;
                                }
                            }
                            Some(to) => {
                                if to > shift.limit() {
                                    return Err(OFF_GRID);
                                }
                                let raw = if to == u64::from(number) {
                                    token.raw.to_vec()
                                } else {
                                    with_attribute(
                                        token.raw,
                                        b"r",
                                        to.to_string().as_bytes(),
                                        index,
                                    )?
                                };
                                out.write_all(&raw)?;
                                if !empty {
                                    row = Some(number);
                                    last_col = 0;
                                }
                            }
                        }
                    }
                } else if let Some(open) = row
                    && data.is_some_and(|at| depth == at + 2)
                    && ours
                    && local == b"c"
                {
                    let tag = cell_tag(start, &scope, index)?;
                    let reference = tag.reference.ok_or(NO_EXPLICIT_REFERENCES)?;
                    if reference.row != open || reference.col <= last_col {
                        return Err(OUT_OF_ORDER);
                    }
                    last_col = reference.col;
                    let on_axis = shift.of(reference);
                    match shift.index(on_axis) {
                        Some(to) if to > shift.limit() => return Err(OFF_GRID),
                        Some(to) if to != u64::from(on_axis) => {
                            let moved = shift.moved(reference, to as u32);
                            emit!(&with_attribute(
                                token.raw,
                                b"r",
                                moved.a1().as_bytes(),
                                index
                            )?);
                        }
                        Some(_) => emit!(token.raw),
                        // A deleted row's cell goes with the row; a deleted
                        // column's cell goes alone, its formula still read.
                        None => {
                            if dropping.is_none() && !empty {
                                dropping = Some(depth);
                            }
                        }
                    }
                    if !empty {
                        cell = Some((depth, shift.reaches(on_axis)));
                    }
                } else if dropping.is_some() {
                    // Anything else inside a deleted row goes with it.
                } else if depth == 2 && ours && local == b"sheetData" && !seen_data {
                    seen_data = true;
                    out.write_all(token.raw)?;
                    if !empty {
                        data = Some(depth);
                    }
                } else if depth == 2 && ours && local == b"dimension" {
                    let raw = match attribute_value(token.raw, b"ref", index)? {
                        Some(value) => {
                            let area = shift_area(value)?;
                            match shift.area_clipped(area) {
                                Some(moved) if moved == area => token.raw.to_vec(),
                                Some(moved) => {
                                    with_attribute(token.raw, b"ref", moved.a1().as_bytes(), index)?
                                }
                                None => with_attribute(token.raw, b"ref", b"A1", index)?,
                            }
                        }
                        None => token.raw.to_vec(),
                    };
                    out.write_all(&raw)?;
                } else if depth == 2
                    && ours
                    && (local == b"mergeCells" || local == b"dataValidations")
                    && counted.is_none()
                {
                    let buffer = CountedBuffer {
                        depth,
                        start: token.raw.to_vec(),
                        name: start.name().into_inner().as_bytes().to_vec(),
                        content: Vec::new(),
                        kept: 0,
                    };
                    if empty {
                        out.write_all(token.raw)?;
                    } else {
                        counted = Some(buffer);
                    }
                } else if let Some(buffer) = counted.as_mut()
                    && depth == buffer.depth + 1
                    && ours
                    && (local == b"mergeCell" || local == b"dataValidation")
                {
                    let kept = if local == b"mergeCell" {
                        match attribute_value(token.raw, b"ref", index)? {
                            Some(value) => {
                                let area = shift_area(value)?;
                                match shift.area_clipped(area) {
                                    Some(moved) if moved == area => Some(token.raw.to_vec()),
                                    Some(moved) if moved.cells() > 1 => Some(with_attribute(
                                        token.raw,
                                        b"ref",
                                        moved.a1().as_bytes(),
                                        index,
                                    )?),
                                    _ => None,
                                }
                            }
                            None => Some(token.raw.to_vec()),
                        }
                    } else {
                        list_tag(token.raw, b"sqref", shift, index)?
                    };
                    match kept {
                        Some(raw) => {
                            buffer.kept += 1;
                            buffer.content.extend_from_slice(&raw);
                        }
                        None => {
                            if !empty {
                                skip_element(&mut stream, &mut scope)?;
                            }
                            scope.end();
                            depth -= 1;
                            continue;
                        }
                    }
                } else if depth == 2
                    && ours
                    && (local == b"hyperlinks" || local == b"cols" && shift.columns)
                    && links.is_none()
                {
                    if empty {
                        out.write_all(token.raw)?;
                    } else {
                        links = Some(LinksBuffer {
                            depth,
                            start: token.raw.to_vec(),
                            started: false,
                            held: Vec::new(),
                            child: if local == b"cols" {
                                b"col"
                            } else {
                                b"hyperlink"
                            },
                        });
                    }
                } else if let Some(open) = links.as_mut()
                    && depth == open.depth + 1
                    && ours
                    && local == open.child
                {
                    let kept = if local == b"col" {
                        col_tag(token.raw, shift, index)?
                    } else {
                        list_tag(token.raw, b"ref", shift, index)?
                    };
                    match kept {
                        Some(raw) => {
                            if !open.started {
                                out.write_all(&open.start)?;
                                open.started = true;
                            }
                            out.write_all(&open.held)?;
                            open.held.clear();
                            out.write_all(&raw)?;
                        }
                        None => {
                            if !empty {
                                skip_element(&mut stream, &mut scope)?;
                            }
                            scope.end();
                            depth -= 1;
                            continue;
                        }
                    }
                } else if depth == 2
                    && ours
                    && (local == b"conditionalFormatting" || local == b"autoFilter")
                {
                    let name: &[u8] = if local == b"autoFilter" {
                        b"ref"
                    } else {
                        b"sqref"
                    };
                    match list_tag(token.raw, name, shift, index)? {
                        Some(raw) => out.write_all(&raw)?,
                        None => {
                            if !empty {
                                skip_element(&mut stream, &mut scope)?;
                            }
                            scope.end();
                            depth -= 1;
                            continue;
                        }
                    }
                } else {
                    emit!(token.raw);
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                if cell.is_some_and(|(at, _)| at == depth) {
                    emit!(token.raw);
                    cell = None;
                    if dropping == Some(depth) {
                        dropping = None;
                    }
                } else if dropping == Some(depth) {
                    dropping = None;
                    row = None;
                } else if row.is_some() && data.is_some_and(|at| depth == at + 1) {
                    emit!(token.raw);
                    row = None;
                } else if data == Some(depth) {
                    out.write_all(token.raw)?;
                    data = None;
                } else if counted.as_ref().is_some_and(|buffer| buffer.depth == depth) {
                    if let Some(buffer) = counted.take() {
                        out.write_all(&buffer.written(index)?)?;
                    }
                } else if links.as_ref().is_some_and(|open| open.depth == depth) {
                    if let Some(open) = links.take()
                        && open.started
                    {
                        out.write_all(&open.held)?;
                        out.write_all(token.raw)?;
                    }
                } else {
                    emit!(token.raw);
                }
                scope.end();
                depth -= 1;
            }
            _ => emit!(token.raw),
        }
    }
    let (temp, replacement) =
        written_part(temp, out.into_inner().map_err(|e| Error::Io(e.into()))?)?;
    Ok((temp, replacement, moved_formula))
}

/// An `<f>` start tag through the rule: a shared, array or data table
/// `ref` moved whole, refused when the rule grows or cuts it, and
/// unchanged when it lies wholly inside a deletion (the row is dropped
/// with it, P2 ruling 1); a moved end past the grid is a grow (P2 ruling
/// 5). Every other formula tag is its bytes.
#[cfg(feature = "rowcol")]
fn shifted_formula(
    raw: &[u8],
    shift: &AxisShift,
    dropping: bool,
    index: usize,
) -> Result<Vec<u8>, Error> {
    let kind = attribute_value(raw, b"t", index)?;
    if !matches!(kind, Some(b"shared") | Some(b"array") | Some(b"dataTable")) {
        return Ok(raw.to_vec());
    }
    let Some(value) = attribute_value(raw, b"ref", index)? else {
        return Ok(raw.to_vec());
    };
    let area = shift.rowwise(shift_area(value)?);
    match shift.shift.interval(area.first.row, area.last.row) {
        Moved::Gone => Ok(raw.to_vec()),
        Moved::Changed(..) => Err(CUTS_FORMULA),
        Moved::Whole(a, b) => {
            if dropping {
                // A master in a deleted row whose ref the rule keeps whole
                // cannot be: the ref starts at the master.
                return Err(CUTS_FORMULA);
            }
            if b > shift.limit() {
                return Err(if a > shift.limit() {
                    OFF_GRID
                } else {
                    CUTS_FORMULA
                });
            }
            if a == u64::from(area.first.row) {
                return Ok(raw.to_vec());
            }
            let moved = Area {
                first: CellRef {
                    row: a as u32,
                    col: area.first.col,
                },
                last: CellRef {
                    row: b as u32,
                    col: area.last.col,
                },
            };
            with_attribute(raw, b"ref", shift.rowwise(moved).a1().as_bytes(), index)
        }
    }
}

/// The `ref` a table part's root carries, or a pivot table part's first
/// `<location>`, read by a pass over the part's tokens.
#[cfg(any(feature = "rowcol", feature = "tables"))]
fn part_ref(archive: &mut Core, index: usize, location: bool) -> Result<Option<Vec<u8>>, Error> {
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                depth += 1;
                scope.start(start)?;
                let (namespace, local) = scope.resolve_element(start.name())?;
                let wanted = if location {
                    is_main(namespace) && local == b"location"
                } else {
                    depth == 1
                };
                if wanted {
                    return Ok(attribute_value(token.raw, b"ref", index)?.map(<[u8]>::to_vec));
                }
                if matches!(token.event, Event::Empty(_)) {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                scope.end();
                depth -= 1;
            }
            _ => {}
        }
    }
    Ok(None)
}

/// What a created rels part begins with when the package has no
/// `_rels/.rels` of its own to take the form from.
///
/// The form 148 of the corpus's 216 rels parts write, which is the majority
/// (P3 ruling 10). It is a fallback and not a default: a package that has its
/// own rels parts supplies the form itself.
fn fallback_rels_prefix() -> Vec<u8> {
    let mut prefix = BYTE_ORDER_MARK.to_vec();
    prefix.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"utf-8\"?>");
    prefix
}

/// The bytes a created rels part begins with: the package's own `_rels/.rels`
/// up to its root start tag.
///
/// Every token's `raw` before the first `Start` or `Empty` event, the mark
/// first when the part has one, so the declaration and whatever follows it
/// come out exactly as that package writes them. No corpus package mixes two
/// forms across its own rels parts, which is what makes one of them the
/// package's form (P3 ruling 10).
fn rels_prefix(archive: &mut Core) -> Result<Vec<u8>, Error> {
    let Some(index) = archive.index_of(&rels_part_name(b"")) else {
        return Ok(fallback_rels_prefix());
    };
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut prefix = Vec::new();
    if stream.bom() {
        prefix.extend_from_slice(&BYTE_ORDER_MARK);
    }
    while let Some(token) = stream.next()? {
        if matches!(token.event, Event::Start(_) | Event::Empty(_)) {
            break;
        }
        prefix.extend_from_slice(token.raw);
    }
    Ok(prefix)
}

/// A rels part for a source that had none, in the form the package's own rels
/// parts use.
///
/// The prefix from [`rels_prefix`], the root on one line, the new elements
/// with no whitespace between them, the end tag and nothing after
/// (P3 ruling 10).
fn new_rels_part(
    archive: &mut Core,
    additions: &[Vec<u8>],
) -> Result<(TempFile, Replacement), Error> {
    let prefix = rels_prefix(archive)?;
    let (temp, file) = temp_for_part()?;
    let mut out = BufWriter::new(file);
    out.write_all(&prefix)?;
    out.write_all(b"<Relationships xmlns=\"")?;
    out.write_all(RELATIONSHIPS_NS)?;
    out.write_all(b"\">")?;
    for addition in additions {
        out.write_all(addition)?;
    }
    out.write_all(b"</Relationships>")?;
    written_part(temp, out.into_inner().map_err(|e| Error::Io(e.into()))?)
}

/// Reads `[Content_Types].xml`.
fn parse_content_types(archive: &mut Core, index: usize) -> Result<ContentTypes, Error> {
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut scope = Scope::new(index);
    let mut root = false;
    let mut defaults = Vec::new();
    let mut overrides = Vec::new();
    while let Some(token) = stream.next()? {
        let (start, empty) = match token.event {
            Event::Start(ref start) => (start, false),
            Event::Empty(ref start) => (start, true),
            Event::End(_) => {
                scope.end();
                continue;
            }
            _ => continue,
        };
        // A start tag's own declarations bind for that tag, so the scope
        // takes them before its name is resolved; an empty element's are
        // dropped again below.
        scope.start(start)?;
        let (namespace, local) = scope.resolve_element(start.name())?;
        let ours = namespace == Some(CONTENT_TYPES_NS);
        if !root {
            if !ours || local != b"Types" {
                return Err(Error::Package(
                    "a content types part whose root is not Types",
                ));
            }
            root = true;
        } else if ours && local == b"Default" {
            let (Some(extension), Some(content_type)) = (
                written_attribute(start, b"Extension", index)?,
                written_attribute(start, b"ContentType", index)?,
            ) else {
                return Err(INCOMPLETE);
            };
            defaults.push(Default {
                extension,
                content_type,
            });
        } else if ours && local == b"Override" {
            let (Some(part_name), Some(content_type)) = (
                written_attribute(start, b"PartName", index)?,
                written_attribute(start, b"ContentType", index)?,
            ) else {
                return Err(INCOMPLETE);
            };
            overrides.push(Override {
                part_name,
                content_type,
            });
        }
        if empty {
            scope.end();
        }
    }
    Ok(ContentTypes {
        defaults,
        overrides,
    })
}

/// Reads one `.rels` part, whose relationships belong to `source`.
fn parse_relationships(
    archive: &mut Core,
    index: usize,
    source: Vec<u8>,
) -> Result<Relationships, Error> {
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut scope = Scope::new(index);
    let mut root = false;
    let mut list: Vec<Relationship> = Vec::new();
    while let Some(token) = stream.next()? {
        let (start, empty) = match token.event {
            Event::Start(ref start) => (start, false),
            Event::Empty(ref start) => (start, true),
            Event::End(_) => {
                scope.end();
                continue;
            }
            _ => continue,
        };
        scope.start(start)?;
        let (namespace, local) = scope.resolve_element(start.name())?;
        let ours = namespace == Some(RELATIONSHIPS_NS);
        if !root {
            if !ours || local != b"Relationships" {
                return Err(Error::Package(
                    "a relationships part whose root is not Relationships",
                ));
            }
            root = true;
        } else if ours && local == b"Relationship" {
            let (Some(id), Some(kind), Some(target)) = (
                written_attribute(start, b"Id", index)?,
                written_attribute(start, b"Type", index)?,
                written_attribute(start, b"Target", index)?,
            ) else {
                return Err(INCOMPLETE);
            };
            if list.iter().any(|rel| rel.id == id) {
                return Err(Error::Package("two relationships with the same id"));
            }
            let mode = written_attribute(start, b"TargetMode", index)?;
            list.push(Relationship {
                id,
                kind,
                target,
                external: mode.as_deref() == Some(b"External"),
            });
        }
        if empty {
            scope.end();
        }
    }
    Ok(Relationships { source, list })
}

/// One `<sheet>` as the workbook part writes it, before resolution.
struct RawSheet {
    name: String,
    sheet_id: u32,
    rel_id: Vec<u8>,
    state: Option<Vec<u8>>,
}

/// What [`parse_workbook_sheets`] reads from the workbook part.
struct RawWorkbook {
    sheets: Vec<RawSheet>,
    date1904: bool,
    full_calc_on_load: bool,
    /// The `<calcPr>` elements among the root's children, counted.
    calc_prs: usize,
    /// The `<definedName>` elements of the root's `<definedNames>`.
    #[cfg(feature = "metadata")]
    defined_names: Vec<DefinedName>,
    /// The `<definedNames>` elements among the root's children, counted.
    #[cfg(feature = "metadata")]
    defined_names_containers: usize,
    /// The `<workbookProtection>` elements among the root's children.
    #[cfg(feature = "metadata")]
    workbook_protections: usize,
}

/// Reads the workbook part's `<sheets>`, its first `<workbookPr>`'s
/// `date1904` and its first `<calcPr>`'s `fullCalcOnLoad`, and counts its
/// `<calcPr>` elements, skipping everything else; with the feature
/// `metadata`, also its defined names and `<definedNames>` elements.
fn parse_workbook_sheets(archive: &mut Core, index: usize) -> Result<RawWorkbook, Error> {
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut scope = Scope::new(index);
    let mut root = false;
    /// The root element's depth, so `<sheets>` is 2 and a `<sheet>` 3.
    const ROOT_DEPTH: usize = 1;
    let mut depth = 0usize;
    let mut sheets_at: Option<usize> = None;
    let mut out = Vec::new();
    // `None` until the first `<workbookPr>` is read.
    let mut date1904: Option<bool> = None;
    // `None` until the first `<calcPr>` is read.
    let mut full_calc_on_load: Option<bool> = None;
    let mut calc_prs = 0usize;
    #[cfg(feature = "metadata")]
    let mut names: Vec<DefinedName> = Vec::new();
    #[cfg(feature = "metadata")]
    let mut names_containers = 0usize;
    #[cfg(feature = "metadata")]
    let mut protections = 0usize;
    // The depth of the `<definedNames>` being read, as `sheets_at`.
    #[cfg(feature = "metadata")]
    let mut names_at: Option<usize> = None;
    while let Some(token) = stream.next()? {
        let (start, empty) = match token.event {
            Event::Start(ref start) => (start, false),
            Event::Empty(ref start) => (start, true),
            Event::End(_) => {
                scope.end();
                if sheets_at == Some(depth) {
                    sheets_at = None;
                }
                #[cfg(feature = "metadata")]
                if names_at == Some(depth) {
                    names_at = None;
                }
                depth = depth.saturating_sub(1);
                continue;
            }
            _ => continue,
        };
        depth += 1;
        scope.start(start)?;
        let (namespace, local) = scope.resolve_element(start.name())?;
        let ours = is_main(namespace);
        if !root {
            if !ours || local != b"workbook" {
                return Err(Error::Package("a workbook part whose root is not workbook"));
            }
            root = true;
        } else if ours && local == b"sheets" && depth == ROOT_DEPTH + 1 {
            sheets_at = Some(depth);
        } else if ours && local == b"sheet" && sheets_at == Some(depth - 1) {
            out.push(read_sheet(start, &scope, index)?);
        } else if ours && local == b"workbookPr" && depth == ROOT_DEPTH + 1 && date1904.is_none() {
            let value = written_attribute(start, b"date1904", index)?;
            date1904 = Some(matches!(value.as_deref(), Some(b"1") | Some(b"true")));
        } else if ours && local == b"calcPr" && depth == ROOT_DEPTH + 1 {
            calc_prs += 1;
            if full_calc_on_load.is_none() {
                let value = written_attribute(start, b"fullCalcOnLoad", index)?;
                full_calc_on_load = Some(matches!(value.as_deref(), Some(b"1") | Some(b"true")));
            }
        }
        #[cfg(feature = "metadata")]
        if root && ours && local == b"workbookProtection" && depth == ROOT_DEPTH + 1 {
            protections += 1;
        }
        #[cfg(feature = "metadata")]
        if root && ours && local == b"definedNames" && depth == ROOT_DEPTH + 1 {
            names_containers += 1;
            names_at = Some(depth);
        } else if root && ours && local == b"definedName" && names_at == Some(depth - 1) {
            let (name, sheet, hidden) = read_defined_name(start, &scope, index)?;
            let text = if empty {
                String::new()
            } else {
                // The text read consumes the end tag and closes the scope.
                let text = crate::cells::text_of(&mut stream, &mut scope, index)?;
                depth -= 1;
                text
            };
            names.push(DefinedName {
                name,
                text,
                sheet,
                hidden,
            });
            if !empty {
                continue;
            }
        }
        if empty {
            scope.end();
            if sheets_at == Some(depth) {
                sheets_at = None;
            }
            #[cfg(feature = "metadata")]
            if names_at == Some(depth) {
                names_at = None;
            }
            depth = depth.saturating_sub(1);
        }
    }
    Ok(RawWorkbook {
        sheets: out,
        date1904: date1904.unwrap_or(false),
        full_calc_on_load: full_calc_on_load.unwrap_or(false),
        calc_prs,
        #[cfg(feature = "metadata")]
        defined_names: names,
        #[cfg(feature = "metadata")]
        defined_names_containers: names_containers,
        #[cfg(feature = "metadata")]
        workbook_protections: protections,
    })
}

/// Reads one `<definedName>` start tag: its `name`, unescaped and normalized
/// as a sheet name is; its `localSheetId`, refused with "a sheet edit no
/// longer applies at the write" when it is not a decimal `u32`
/// (`specs/0016-new-workbook-and-sheets.md`, P3 ruling 10); and whether
/// `hidden` is `1` or `true`. An absent `name` reads as empty.
#[cfg(feature = "metadata")]
fn read_defined_name(
    start: &BytesStart<'_>,
    scope: &Scope,
    index: usize,
) -> Result<(String, Option<u32>, bool), Error> {
    let mut name = String::new();
    let mut sheet: Option<u32> = None;
    let mut hidden = false;
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        if namespace.is_some() {
            continue;
        }
        match local {
            b"name" => {
                name = attribute
                    .normalized_value(XmlVersion::Implicit1_0)
                    .map_err(|_| Error::Xml(index))?
                    .into_owned();
            }
            b"localSheetId" => {
                let value = attribute.value.as_bytes();
                if value.is_empty() || !value.iter().all(u8::is_ascii_digit) {
                    return Err(SHEET_EDIT_STALE);
                }
                sheet = Some(
                    str::from_utf8(value)
                        .ok()
                        .and_then(|text| text.parse::<u32>().ok())
                        .ok_or(SHEET_EDIT_STALE)?,
                );
            }
            b"hidden" => hidden = matches!(attribute.value.as_bytes(), b"1" | b"true"),
            _ => {}
        }
    }
    Ok((name, sheet, hidden))
}

/// Reads one `<sheet>` element's attributes.
fn read_sheet(start: &BytesStart<'_>, scope: &Scope, index: usize) -> Result<RawSheet, Error> {
    let mut name: Option<String> = None;
    let mut sheet_id: Option<Vec<u8>> = None;
    let mut rel_id: Option<Vec<u8>> = None;
    let mut state: Option<Vec<u8>> = None;
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        match local {
            // A sheet name is a value the layer interprets, so it is
            // unescaped and normalized (P2 ruling 2); the rest are bytes as
            // written, and `sheetId` is checked below.
            b"name" if namespace.is_none() => {
                name = Some(
                    attribute
                        .normalized_value(XmlVersion::Implicit1_0)
                        .map_err(|_| Error::Xml(index))?
                        .into_owned(),
                );
            }
            b"sheetId" if namespace.is_none() => {
                sheet_id = Some(attribute.value.as_bytes().to_vec());
            }
            b"state" if namespace.is_none() => {
                state = Some(attribute.value.as_bytes().to_vec());
            }
            b"id" if is_relationships_attribute(namespace) => {
                rel_id = Some(attribute.value.as_bytes().to_vec());
            }
            _ => {}
        }
    }
    let (Some(name), Some(sheet_id), Some(rel_id)) = (name, sheet_id, rel_id) else {
        return Err(Error::Package(
            "a sheet element lacks name, sheetId or r:id",
        ));
    };
    let not_a_number = Error::Package("a sheetId that is not a number");
    if sheet_id.is_empty() || !sheet_id.iter().all(|byte| byte.is_ascii_digit()) {
        return Err(not_a_number);
    }
    let sheet_id = std::str::from_utf8(&sheet_id)
        .ok()
        .and_then(|digits| digits.parse::<u32>().ok())
        .ok_or(not_a_number)?;
    Ok(RawSheet {
        name,
        sheet_id,
        rel_id,
        state,
    })
}
