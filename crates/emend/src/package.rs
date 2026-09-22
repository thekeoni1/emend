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
//!   nothing binds (`rel_id_prefix_unbound`).
//! - "no such relationship" (`removing_an_absent_relationship`).
//! - "no such override" (`removing_an_absent_override`).
//! - "an override for that part already exists"
//!   (`adding_an_override_that_exists`).
//! - "an edit to a removed part" (`editing_a_removed_part`,
//!   `editing_a_removed_rels_part`, `editing_removed_content_types`).
//! - "no such part" (`removing_an_absent_part`, `replacing_an_absent_part`,
//!   `a_relationship_on_a_part_that_is_not_there`,
//!   `a_relationship_removal_on_a_part_that_is_not_there`,
//!   `cells::tests::a_relationship_naming_no_entry_is_refused`,
//!   `cells::tests::cells_on_a_part_that_is_not_there_is_refused`,
//!   `styles::tests::a_styles_relationship_naming_no_entry_is_refused`).
//! - "no Default covers rels" (`no_default_for_rels`).
//!
//! The cell edits add eight (`specs/0011-editing-cells.md`, "Errors"); the
//! first two are raised by [`Value`] in `cells` and reported by
//! [`Package::write_to`]:
//!
//! - "a number that is not finite cannot be written"
//!   (`a_number_that_is_not_finite_is_refused`).
//! - "a text holds a character XML cannot carry"
//!   (`a_text_with_a_character_xml_forbids_is_refused`).
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
//! - "the styles part has more than one numFmts or cellXfs"
//!   (`styles::tests::a_second_num_fmts_is_refused`,
//!   `a_second_cell_xfs_is_refused`).
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
//!   strings table, and no rich text run is written.
//! - Nothing is recomputed. The cached values of formulas that depend on an
//!   edited cell stay as the producer wrote them, because emend does not
//!   evaluate; whether the workbook asks for a recalculation on open is part
//!   6's question.
//! - When an edit removes a normal formula and the workbook has a
//!   calculation chain, the chain is dropped in the same write, once, by
//!   [`Package::drop_calc_chain`]'s recipe: the chain would otherwise name a
//!   cell with no formula. Nothing else about calculation changes.
//! - `<dimension>` and a row's `spans` are not updated when a new cell lies
//!   outside them. `<cols>`, merged cells, tables, data validation and
//!   conditional formatting are copied raw and never adjusted. No row or
//!   column is inserted or deleted, so no reference is fixed up.
//! - A worksheet the rewrite cannot place an edit in safely is refused, not
//!   repaired: a row or cell without `r`, rows or cells out of order, an
//!   array or data table `ref` whose extent does not parse (edited or not),
//!   an edited cell in a shared, array or data table formula or carrying an
//!   attribute this layer does not model, and a set on a worksheet without
//!   `<sheetData>`. An unedited cell is copied raw, whatever it carries.
//! - A worksheet is rewritten at most once per write, in one streaming pass
//!   that holds one token and the sorted edits. The edits themselves are
//!   held in memory, in a `Vec` in call order, so a caller that records more
//!   edits than its address space holds cannot.
//!
//! The style edits and additions add theirs (`specs/0012-styles.md`, "Module
//! `package`: recording and applying style edits"):
//!
//! - A new `xf` copies its base's font, fill, border, alignment and
//!   protection and changes its number format only.
//! - A style set on a cell whose row has `customFormat` or whose column has
//!   a `<col style>` overrides those, as any `s` does.
//! - A cell shown as a date is a number with a date format; what a consumer
//!   shows for a format code is the consumer's.
//! - `dxfs`, `cellStyles`, `fonts`, `fills` and `borders` are never touched.
//! - A `numFmts` or `cellXfs` whose `count` disagreed with its children
//!   comes out corrected, since the rewritten `count` is the children's.
//! - A style edit alone rewrites the cell's start tag and nothing else: it
//!   removes no formula, never drops the chain, and the formula refusals do
//!   not apply to it.
//! - The styles part is rewritten at most once per write, in one streaming
//!   pass that holds one token, the recorded additions and two counts.
//! - Number formats only. An addition is a cell format derived from an
//!   existing one, or the plain form over no base, with its number format
//!   changed; no font, fill, border, alignment or protection is ever added
//!   (`specs/0012-styles.md`, "Decisions for the maintainer", 1).
//! - A write with a `Some` style edit reads the styles part to check the
//!   index against the `cellXfs` count plus the additions; a `None` edit
//!   needs no styles part at all (the same section, 3).
//! - Dates are not the core's. The core writes and reads a cell's number;
//!   the serial a date stands for comes from the crate's `dates` feature,
//!   which is off by default and outside the core's size, and nothing here
//!   calls it (the same section, 2). A `t="d"` cell's text is not converted.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::str;

use quick_xml::XmlVersion;
use quick_xml::escape::escape;
use quick_xml::events::{BytesStart, Event};

use crate::archive::{Archive, Edits, Error, Replacement, TempFile, replacement_from_reader};
use crate::cells::{CellRef, Cells, SharedStrings, Value, decimal, forbidden_in_xml};
use crate::styles::{NewXf, NumberFormat, Styles, counted, num_fmt_element};
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
}

impl Workbook {
    /// The sheet with that unescaped name, matched exactly.
    pub fn sheet(&self, name: &str) -> Option<&Sheet> {
        self.sheets.iter().find(|sheet| sheet.name == name)
    }
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
    }

    /// Every worksheet part with a cell edit, a value edit or a style edit,
    /// once each: the value edits' parts in the order of each part's first,
    /// then the style edits' parts not already listed.
    fn cell_parts(&self) -> Vec<Vec<u8>> {
        let mut parts: Vec<Vec<u8>> = Vec::new();
        let named = self
            .cell_edits
            .iter()
            .map(|(part, _, _)| part)
            .chain(self.style_edits.iter().map(|(part, _, _)| part));
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

impl<R: Read + Seek> Package<R> {
    /// Opens the archive and parses no XML.
    ///
    /// # Errors
    ///
    /// Whatever [`Archive::open`] refuses.
    pub fn open(reader: R) -> Result<Self, Error> {
        Ok(Package {
            archive: Archive::open(reader)?,
            content_types: None,
            relationships: Vec::new(),
            workbook: None,
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
    /// that name, whatever [`Archive::read`] refuses for the entry, and
    /// whatever [`Stream::new`] refuses for its first bytes. The cells
    /// themselves are read as the iterator runs, so a malformed worksheet is
    /// an error from [`Cells::next`] and not from this call.
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
        if self.content_types.is_none() {
            let index = self
                .archive
                .index_of(CONTENT_TYPES_PART)
                .ok_or(NO_CONTENT_TYPES)?;
            let parsed = parse_content_types(&mut self.archive, index)?;
            return Ok(self.content_types.insert(parsed));
        }
        // The branch above returns whenever the field was empty.
        Ok(self.content_types.as_ref().unwrap())
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
        if let Some(at) = self
            .relationships
            .iter()
            .position(|rels| rels.source == source)
        {
            return Ok(&self.relationships[at]);
        }
        let part = rels_part_name(source);
        let parsed = match self.archive.index_of(&part) {
            Some(index) => parse_relationships(&mut self.archive, index, source.to_vec())?,
            None => Relationships {
                source: source.to_vec(),
                list: Vec::new(),
            },
        };
        self.relationships.push(parsed);
        // The push above leaves at least one.
        Ok(self.relationships.last().unwrap())
    }

    /// The workbook part and its sheets, parsed on the first call and kept.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] for any of the officeDocument, workbook root and
    /// sheet messages the module header lists; [`Error::Xml`] for bytes that
    /// are not well-formed XML.
    pub fn workbook(&mut self) -> Result<&Workbook, Error> {
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
        let (raw, date1904) = parse_workbook_sheets(&mut self.archive, index)?;
        let rels = self.relationships(&part)?;
        let mut sheets = Vec::with_capacity(raw.len());
        for sheet in raw {
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
            date1904,
        })
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

    /// Records the removal of the relationship `id` from `source`'s rels part.
    ///
    /// Nothing is read or written here, and nothing else goes with it: the
    /// target part, its override and its own rels part are the caller's
    /// separate calls. [`Package::write_to`] refuses an id the part does not
    /// hold.
    pub fn remove_relationship(&mut self, source: &[u8], id: &[u8]) {
        self.edits.rel_removals.push((source.to_vec(), id.to_vec()));
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

    /// Records the removal of the `Override` for `part_name`.
    ///
    /// `part_name` is matched as the `PartName` attribute writes it, by exact
    /// bytes, leading slash included. [`Package::write_to`] refuses an
    /// override the part does not hold.
    pub fn remove_override(&mut self, part_name: &[u8]) {
        self.edits.override_removals.push(part_name.to_vec());
    }

    /// Records a new `Override` for `part_name`.
    ///
    /// [`Package::write_to`] refuses a part name that already has one.
    pub fn add_override(&mut self, part_name: &[u8], content_type: &[u8]) {
        self.edits
            .override_additions
            .push((part_name.to_vec(), content_type.to_vec()));
    }

    /// Records the removal of the entry `name`.
    ///
    /// The entry alone: its override and any relationship naming it are the
    /// caller's separate calls.
    pub fn remove_part(&mut self, name: &[u8]) {
        self.edits.part_removals.push(name.to_vec());
    }

    /// Records new bytes for the entry `name`.
    pub fn replace_part(&mut self, name: &[u8], replacement: Replacement) {
        self.edits
            .part_replacements
            .push((name.to_vec(), replacement));
    }

    /// Records a new entry `name`.
    ///
    /// The entry alone: the caller adds an override or relies on a `Default`
    /// covering its extension.
    pub fn add_part(&mut self, name: &[u8], replacement: Replacement) {
        self.edits.part_additions.push((name.to_vec(), replacement));
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
        let workbook = self.workbook()?.part.clone();
        // Read everything the three removals need before recording any of
        // them: each read borrows the package, and recording takes it
        // mutably.
        let (id, part) = {
            let rels = self.relationships(&workbook)?;
            match rels.first_of(CALC_CHAIN) {
                Some(rel) => (rel.id.clone(), rels.target_part(rel)?),
                None => return Ok(false),
            }
        };
        let over = self
            .content_types()?
            .overrides()
            .iter()
            .find(|over| {
                over.part_name
                    .strip_prefix(b"/".as_slice())
                    .unwrap_or(&over.part_name)
                    == part.as_slice()
            })
            .map(|over| over.part_name.clone());
        let entry = self.part(&part).is_some();

        self.remove_relationship(&workbook, &id);
        if let Some(over) = over {
            self.remove_override(&over);
        }
        if entry {
            self.remove_part(&part);
        }
        Ok(true)
    }

    /// Records `reference` in the worksheet part `part` set to `value`.
    ///
    /// `part` is the worksheet's entry name, as [`Package::cells`] takes it;
    /// the caller finds it through [`Workbook::sheet`]. Nothing is read and
    /// nothing can fail here, and the last edit recorded for a reference
    /// wins. [`Package::write_to`] checks the value and rewrites the part
    /// (`specs/0011-editing-cells.md`, "Module `package`").
    pub fn set_cell(&mut self, part: &[u8], reference: CellRef, value: Value) {
        self.edits
            .cell_edits
            .push((part.to_vec(), reference, Some(value)));
    }

    /// Records `reference` in the worksheet part `part` cleared.
    ///
    /// A cell with a style keeps it and loses everything else; a cell without
    /// one is removed; a cell that does not exist stays absent. As
    /// [`Package::set_cell`], nothing is read here and the last edit recorded
    /// for a reference wins.
    pub fn clear_cell(&mut self, part: &[u8], reference: CellRef) {
        self.edits.cell_edits.push((part.to_vec(), reference, None));
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
        let (id, new_code) = match format {
            NumberFormat::BuiltIn(id) if id >= 164 => return Err(BUILT_IN_164),
            NumberFormat::BuiltIn(id) => (id, None),
            NumberFormat::Code(code) => {
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
                    .find(|(_, have)| **have == code)
                    .map(|(id, _)| id);
                match defined {
                    Some(id) => (id, None),
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
                        (id, Some(code))
                    }
                }
            }
        };
        let index = u32::try_from(existing + recorded).map_err(|_| STYLE_OUT_OF_RANGE)?;
        // The base is in range by the check above, and its bytes are ones the
        // reader accepted, so this is `None` only for a part that could not
        // have been read.
        let xf = styles.derived_xf(base, id).ok_or(STYLE_OUT_OF_RANGE)?;
        if let Some(code) = new_code {
            self.edits.num_fmt_additions.push((id, code));
        }
        self.edits.xf_additions.push(xf);
        Ok(index)
    }

    /// Records `reference` in the worksheet part `part` given the style
    /// `style`: `Some(i)` names a cell format by index, one the part has or
    /// one [`Package::add_cell_style`] returned, and `None` removes the
    /// cell's `s`.
    ///
    /// As [`Package::set_cell`], nothing is read and nothing can fail here,
    /// and the last style edit recorded for a reference wins, independently
    /// of the last value edit for it (`specs/0012-styles.md`, "Module
    /// `package`"). [`Package::write_to`] checks the index and rewrites the
    /// part.
    pub fn set_cell_style(&mut self, part: &[u8], reference: CellRef, style: Option<u32>) {
        self.edits
            .style_edits
            .push((part.to_vec(), reference, style));
    }

    /// Writes the package back with every recorded edit applied.
    ///
    /// Every edit is validated first, so a refused set writes nothing. Then
    /// each worksheet with cell edits is streamed into a temporary file once,
    /// with all of its edits, as `specs/0011-editing-cells.md`, "Module
    /// `package`", describes; a refusal found while a part streams also
    /// writes nothing. When an edit replaced or cleared a cell whose formula
    /// was a normal one, the calculation chain is dropped in the same write
    /// through [`Package::drop_calc_chain`], unless its relationship is
    /// already recorded for removal, and the set is validated again. Then
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
    /// it rewrites, "a style attribute is not a number".
    /// [`Error::Xml`] for a part name or value that is not UTF-8, carrying
    /// the index of the part the value would have been written into.
    /// Whatever [`Stream`] refuses for a part it rewrites, and whatever
    /// [`Archive::write_to`] refuses for the assembled edits.
    pub fn write_to(mut self, out: impl Write) -> Result<(), Error> {
        self.validate()?;
        let mut edits = Edits::new();
        // The guards outlive the write: each replacement reads its file back,
        // and Drop removes it however this call ends.
        let mut temporaries: Vec<TempFile> = Vec::new();

        // The cell edits go first, each worksheet rewritten once with all of
        // its edits, because a removed formula adds the chain's three
        // removals, which the content types and rels rewrites below apply
        // (`specs/0011-editing-cells.md`, "Module `package`").
        let mut removed_formula = false;
        for part in self.edits.cell_parts() {
            let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
            let sorted = self.edits.cell_edits_of(&part);
            let (temp, replacement, removed) =
                rewrite_worksheet(&mut self.archive, index, &sorted)?;
            removed_formula |= removed;
            temporaries.push(temp);
            edits.replace(index, replacement);
        }
        if removed_formula {
            self.drop_calc_chain_once()?;
        }

        // The styles part, rewritten once when a cell format was added
        // (`specs/0012-styles.md`, "Module `package`"). `validate` has read
        // it, so the workbook names it and the counts are cached.
        if !self.edits.xf_additions.is_empty() {
            let part = self.workbook()?.styles.clone().ok_or(NO_STYLES_PART)?;
            let index = self.archive.index_of(&part).ok_or(NO_SUCH_PART)?;
            let styles = self.styles()?;
            let counts = Counts {
                num_fmts: styles.num_fmts.len() as u64,
                cell_xfs: styles.cell_xfs.len() as u64,
                has_num_fmts: styles.has_num_fmts(),
            };
            let mut codes = self.edits.num_fmt_additions.clone();
            codes.sort_by_key(|(id, _)| *id);
            let (temp, replacement) = rewrite_styles(
                &mut self.archive,
                index,
                &counts,
                &codes,
                &self.edits.xf_additions,
            )?;
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
                &mut self.archive,
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
                        &mut self.archive,
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
                    let (temp, replacement) = new_rels_part(&mut self.archive, &additions)?;
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

    /// The chain rule of [`Package::write_to`], for a write whose cell edits
    /// removed a formula.
    ///
    /// The chain lists the cell whose formula went, and Excel repairs a
    /// workbook whose chain names a cell with no formula. Unless the caller
    /// has already recorded the chain's relationship for removal, the recipe
    /// is applied here, and the whole set is validated again with its three
    /// removals in it.
    fn drop_calc_chain_once(&mut self) -> Result<(), Error> {
        if !self.calc_chain_pending()? && self.drop_calc_chain()? {
            self.validate()?;
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
        for part in self.edits.cell_parts() {
            if self.archive.index_of(&part).is_none() {
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
const STYLE_OUT_OF_RANGE: Error = Error::Package("a style index is out of range");
const BUILT_IN_164: Error = Error::Package("a built-in number format id is 164 or more");
const CODE_REFUSED: Error =
    Error::Package("a number format code is empty or holds a character XML cannot carry");
const STYLE_ADDITION_REPLACED: Error =
    Error::Package("a style addition to a part this write replaces");
const NO_CELL_XFS: Error = Error::Package("a styles part without cellXfs cannot take a style");

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
fn rewrite_part<R: Read + Seek>(
    archive: &mut Archive<R>,
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

/// What the styles rewrite knows of the part before it streams: its two
/// containers' child counts, read by [`Package::styles`] from the same bytes,
/// and whether it has a `<numFmts>`.
struct Counts {
    num_fmts: u64,
    cell_xfs: u64,
    has_num_fmts: bool,
}

/// Rewrites the styles part with the recorded additions into a temporary
/// file.
///
/// `specs/0012-styles.md`, "Module `package`": every token's `raw` is copied
/// except that the `<numFmts>` and `<cellXfs>` start tags among the root's
/// children have their `count` values replaced by their children plus the
/// additions, every other byte of the tag kept; the new `numFmt` elements
/// (`codes`, in id order) go immediately before `</numFmts>` and the new `xf`
/// elements immediately before `</cellXfs>`, each taking its container's
/// prefix; an empty container that gains children is opened up as part 2
/// opens an empty root; and a part with no `<numFmts>` that gains a code gets
/// one, with the root's prefix, immediately before the first of `fonts`,
/// `fills`, `borders`, `cellStyleXfs` and `cellXfs` among the root's
/// children. A part with no `<cellXfs>` is refused once the stream ends,
/// leaving no output.
fn rewrite_styles<R: Read + Seek>(
    archive: &mut Archive<R>,
    index: usize,
    counts: &Counts,
    codes: &[(u32, String)],
    xfs: &[NewXf],
) -> Result<(TempFile, Replacement), Error> {
    /// The root's children, before the first of which a new `<numFmts>`
    /// goes, in the schema's order.
    const AFTER_NUM_FMTS: [&[u8]; 5] =
        [b"fonts", b"fills", b"borders", b"cellStyleXfs", b"cellXfs"];

    let (temp, file) = temp_for_part()?;
    let mut out = BufWriter::new(file);
    let mut stream = Stream::new(archive.read(index)?, index)?;
    if stream.bom() {
        out.write_all(&BYTE_ORDER_MARK)?;
    }
    let num_fmt_elements = |prefix: &[u8]| -> Vec<u8> {
        codes
            .iter()
            .flat_map(|(id, code)| num_fmt_element(prefix, *id, code))
            .collect()
    };
    let xf_elements =
        |prefix: &[u8]| -> Vec<u8> { xfs.iter().flat_map(|xf| xf.element(prefix)).collect() };
    let num_fmts_count = counts.num_fmts + codes.len() as u64;
    let cell_xfs_count = counts.cell_xfs + xfs.len() as u64;

    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut root_prefix: Vec<u8> = Vec::new();
    // Whether a `<numFmts>` still has to be created before the next of
    // `AFTER_NUM_FMTS`.
    let mut create_num_fmts = !counts.has_num_fmts && !codes.is_empty();
    let mut seen_cell_xfs = false;
    // The open container among the root's children, with its prefix.
    let mut open: Option<(bool, Vec<u8>)> = None;
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
                if depth == 2 && ours && create_num_fmts && AFTER_NUM_FMTS.contains(&local) {
                    let p = &root_prefix;
                    out.write_all(&[b"<".as_slice(), p, b"numFmts count=\""].concat())?;
                    out.write_all(num_fmts_count.to_string().as_bytes())?;
                    out.write_all(b"\">")?;
                    out.write_all(&num_fmt_elements(p))?;
                    out.write_all(&[b"</".as_slice(), p, b"numFmts>"].concat())?;
                    create_num_fmts = false;
                }
                let container = if depth == 2 && ours && local == b"numFmts" {
                    Some((true, num_fmts_count, !codes.is_empty()))
                } else if depth == 2 && ours && local == b"cellXfs" {
                    seen_cell_xfs = true;
                    Some((false, cell_xfs_count, true))
                } else {
                    None
                };
                match container {
                    Some((is_num_fmts, count, gains)) => {
                        let tag = counted(token.raw, count, index)?;
                        let prefix = prefix_of(start);
                        if empty && gains {
                            out.write_all(&opened_up(&tag, index)?)?;
                            out.write_all(&if is_num_fmts {
                                num_fmt_elements(&prefix)
                            } else {
                                xf_elements(&prefix)
                            })?;
                            out.write_all(b"</")?;
                            out.write_all(start.name().into_inner().as_bytes())?;
                            out.write_all(b">")?;
                        } else {
                            out.write_all(&tag)?;
                            if !empty {
                                open = Some((is_num_fmts, prefix));
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
                    && let Some((is_num_fmts, prefix)) = open.take()
                {
                    out.write_all(&if is_num_fmts {
                        num_fmt_elements(&prefix)
                    } else {
                        xf_elements(&prefix)
                    })?;
                }
                out.write_all(token.raw)?;
                scope.end();
                depth -= 1;
            }
            _ => out.write_all(token.raw)?,
        }
    }
    if !seen_cell_xfs {
        return Err(NO_CELL_XFS);
    }
    written_part(temp, out.into_inner().map_err(|e| Error::Io(e.into()))?)
}

/// A rectangle of cells, both corners included: an array formula's `ref`.
#[derive(Clone, Copy)]
struct Area {
    first: CellRef,
    last: CellRef,
}

impl Area {
    /// Reads `A1:B2` or `A1`; `None` for anything else, so the caller refuses
    /// a formula whose extent it cannot know.
    fn parse(value: &[u8]) -> Option<Self> {
        let mut halves = value.splitn(2, |byte| *byte == b':');
        let a = CellRef::parse(halves.next()?)?;
        let b = match halves.next() {
            Some(half) => CellRef::parse(half)?,
            None => a,
        };
        Some(Area {
            first: CellRef {
                row: a.row.min(b.row),
                col: a.col.min(b.col),
            },
            last: CellRef {
                row: a.row.max(b.row),
                col: a.col.max(b.col),
            },
        })
    }

    fn holds(&self, cell: CellRef) -> bool {
        (self.first.row..=self.last.row).contains(&cell.row)
            && (self.first.col..=self.last.col).contains(&cell.col)
    }
}

/// One worksheet's cell edits, sorted, and how far the rewrite has come.
struct Pending<'e> {
    edits: &'e [(CellRef, CellEdit)],
    next: usize,
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
    /// left with `None`) that sets a cell; one with clears alone writes
    /// nothing. `prefix` is the `<sheetData>`'s, which the rows take.
    fn write_rows(
        &mut self,
        out: &mut impl Write,
        row: Option<u32>,
        arrays: &[Area],
        prefix: &[u8],
    ) -> Result<(), Error> {
        while let Some(next) = self.peek() {
            if row.is_some_and(|r| next.row >= r) {
                break;
            }
            let opens = self.sets(Some(next.row));
            if opens {
                out.write_all(b"<")?;
                out.write_all(prefix)?;
                out.write_all(format!("row r=\"{}\">", next.row).as_bytes())?;
            }
            self.write_row(out, next.row, None, arrays, prefix)?;
            if opens {
                out.write_all(b"</")?;
                out.write_all(prefix)?;
                out.write_all(b"row>")?;
            }
        }
        Ok(())
    }
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

/// Rewrites one worksheet part with its cell edits into a temporary file,
/// and says whether any edit removed a formula.
///
/// `specs/0011-editing-cells.md`, "Module `package`": every token's `raw` is
/// copied except that an edited `<c>` (start tag, children and end tag, or
/// its one empty tag) is replaced by the edit's form; a new cell goes before
/// the first cell of its row with a greater column, or before `</row>`; a new
/// row goes before the first `<row>` with a greater number, or before
/// `</sheetData>`; an empty `<sheetData/>` or `<row/>` that gains content is
/// opened up as part 2 opens an empty root. Everything else is the input's
/// bytes, so nothing outside `<sheetData>` changes.
///
/// `edits` is sorted by row then column, as [`Recorded::cell_edits_of`]
/// gives it.
fn rewrite_worksheet<R: Read + Seek>(
    archive: &mut Archive<R>,
    index: usize,
    edits: &[(CellRef, CellEdit)],
) -> Result<(TempFile, Replacement, bool), Error> {
    let (temp, file) = temp_for_part()?;
    let mut out = BufWriter::new(file);
    let mut stream = Stream::new(archive.read(index)?, index)?;
    if stream.bom() {
        out.write_all(&BYTE_ORDER_MARK)?;
    }
    let mut pending = Pending { edits, next: 0 };
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut root = false;
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
                } else if let Some((at, ref mut inside)) = cell {
                    // Inside a cell: only its own `<f>` matters.
                    if depth == at + 1 && ours && local == b"f" {
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
                    if empty && pending.sets(Some(number)) {
                        out.write_all(&opened_up(token.raw, index)?)?;
                        pending.write_row(&mut out, number, None, &arrays, &row_prefix)?;
                        out.write_all(b"</")?;
                        out.write_all(start.name().into_inner().as_bytes())?;
                        out.write_all(b">")?;
                    } else {
                        out.write_all(token.raw)?;
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
                    if pending.peek() == Some(reference) {
                        let edit = pending.edits[pending.next].1.clone();
                        pending.next += 1;
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
                    if empty && pending.sets(None) {
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
                        }
                        out.write_all(token.raw)?;
                    }
                }
                scope.end();
                depth -= 1;
            }
            _ => {
                if !matches!(cell, Some((_, InCell::Replace { .. }))) {
                    out.write_all(token.raw)?;
                }
            }
        }
    }
    // A set that found no `<sheetData>` to go into (P1 ruling 2); a clear
    // alone has nothing to remove there.
    if pending.sets(None) {
        return Err(NO_SHEET_DATA);
    }
    let (temp, replacement) =
        written_part(temp, out.into_inner().map_err(|e| Error::Io(e.into()))?)?;
    Ok((temp, replacement, removed_formula))
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
fn rels_prefix<R: Read + Seek>(archive: &mut Archive<R>) -> Result<Vec<u8>, Error> {
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
fn new_rels_part<R: Read + Seek>(
    archive: &mut Archive<R>,
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
fn parse_content_types<R: Read + Seek>(
    archive: &mut Archive<R>,
    index: usize,
) -> Result<ContentTypes, Error> {
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
fn parse_relationships<R: Read + Seek>(
    archive: &mut Archive<R>,
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

/// Reads the workbook part's `<sheets>` and its first `<workbookPr>`'s
/// `date1904`, skipping everything else.
fn parse_workbook_sheets<R: Read + Seek>(
    archive: &mut Archive<R>,
    index: usize,
) -> Result<(Vec<RawSheet>, bool), Error> {
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
    while let Some(token) = stream.next()? {
        let (start, empty) = match token.event {
            Event::Start(ref start) => (start, false),
            Event::Empty(ref start) => (start, true),
            Event::End(_) => {
                scope.end();
                if sheets_at == Some(depth) {
                    sheets_at = None;
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
        }
        if empty {
            scope.end();
            if sheets_at == Some(depth) {
                sheets_at = None;
            }
            depth = depth.saturating_sub(1);
        }
    }
    Ok((out, date1904.unwrap_or(false)))
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
