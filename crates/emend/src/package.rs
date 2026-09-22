//! The Open Packaging Conventions layer: content types, relationships, sheets.
//!
//! `specs/0009-package-layer.md`, "Module `package`". A [`Package`] wraps an
//! [`Archive`] and parses nothing until asked: [`Package::content_types`]
//! reads `[Content_Types].xml`, [`Package::relationships`] reads one part's
//! `.rels`, and [`Package::workbook`] follows the package's `officeDocument`
//! relationship to the workbook part and maps its sheets to theirs. Each
//! answer is parsed once and kept. Nothing here reads a cell, and nothing
//! here writes: the edits and `write_to` are P3.
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
//!   `cells::tests::cells_on_a_part_that_is_not_there_is_refused`).
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
//! The `cells` module raises its messages through the same error and they are
//! catalogued here with the rest, so there is one list for the three modules
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
//!   (`cells::tests::a_cell_reference_that_is_not_a1_is_refused`).
//! - "a row number is not a number"
//!   (`cells::tests::a_row_number_that_is_not_a_number_is_refused`,
//!   `a_row_number_of_zero_is_refused`,
//!   `a_number_this_layer_reads_is_digits_alone`,
//!   `an_index_past_u32_is_refused_like_one_that_is_not_a_number`): a row is
//!   numbered from 1, so 0 is one of the spellings this layer has no row
//!   number for (P2 ruling 2).
//! - "a cell's style index is not a number"
//!   (`cells::tests::a_style_index_that_is_not_a_number_is_refused`,
//!   `a_number_this_layer_reads_is_digits_alone`,
//!   `an_index_past_u32_is_refused_like_one_that_is_not_a_number`).
//! - "a cell has a type this layer does not know"
//!   (`cells::tests::a_cell_type_this_layer_does_not_know_is_refused`).
//! - "a formula has a type this layer does not know"
//!   (`cells::tests::a_formula_type_this_layer_does_not_know_is_refused`).
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
use crate::cells::{CellRef, Cells, SharedStrings, Value, decimal};
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
    }

    /// Every worksheet part with a cell edit, once each, in the order of each
    /// part's first edit.
    fn cell_parts(&self) -> Vec<Vec<u8>> {
        let mut parts: Vec<Vec<u8>> = Vec::new();
        for (part, _, _) in &self.cell_edits {
            if !parts.iter().any(|seen| seen == part) {
                parts.push(part.clone());
            }
        }
        parts
    }

    /// One part's cell edits sorted by row then column, the last recorded
    /// for a reference winning.
    fn cell_edits_of(&self, part: &[u8]) -> Vec<(CellRef, Option<Value>)> {
        let mut sorted: BTreeMap<CellRef, Option<Value>> = BTreeMap::new();
        for (at, reference, value) in &self.cell_edits {
            if at.as_slice() == part {
                sorted.insert(*reference, value.clone());
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
        let raw = parse_workbook_sheets(&mut self.archive, index)?;
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
        })
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
    /// edited", "a cell carries an attribute this layer does not model" and
    /// "a worksheet without sheetData cannot be edited".
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
        // The chain lists the cell whose formula went, and Excel repairs a
        // workbook whose chain names a cell with no formula. Unless the
        // caller has already recorded the chain's relationship for removal,
        // the recipe is applied here, and the whole set is validated again
        // with its three removals in it.
        if removed_formula && !self.calc_chain_pending()? && self.drop_calc_chain()? {
            self.validate()?;
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
    edits: &'e [(CellRef, Option<Value>)],
    next: usize,
}

impl Pending<'_> {
    fn peek(&self) -> Option<CellRef> {
        self.edits.get(self.next).map(|(reference, _)| *reference)
    }

    /// Whether any edit still pending sets a cell, in `row` or, with `None`,
    /// anywhere.
    fn sets(&self, row: Option<u32>) -> bool {
        self.edits[self.next..]
            .iter()
            .any(|(reference, value)| value.is_some() && row.is_none_or(|r| reference.row == r))
    }

    /// Writes the cells set in `row` before column `before` (all of the row's
    /// with `None`); a pending clear of a cell that is not there writes
    /// nothing.
    fn write_row(
        &mut self,
        out: &mut impl Write,
        row: u32,
        before: Option<u32>,
        arrays: &[Area],
        prefix: &[u8],
    ) -> Result<(), Error> {
        while let Some((reference, value)) = self.edits.get(self.next) {
            if reference.row != row || before.is_some_and(|col| reference.col >= col) {
                break;
            }
            if let Some(value) = value {
                if arrays.iter().any(|area| area.holds(*reference)) {
                    return Err(IN_SHARED_OR_ARRAY);
                }
                out.write_all(&value.element(*reference, None, prefix))?;
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
    style: Option<Vec<u8>>,
    /// Whether it carries an attribute other than `r`, `s` and `t`.
    unmodelled: bool,
}

fn cell_tag(start: &BytesStart<'_>, scope: &Scope, index: usize) -> Result<CellTag, Error> {
    let mut tag = CellTag {
        reference: None,
        style: None,
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
            }
            (None, b"s") => tag.style = Some(attribute.value.as_bytes().to_vec()),
            (None, b"t") => {}
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
    edits: &[(CellRef, Option<Value>)],
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
                        let value = pending.edits[pending.next].1.clone();
                        pending.next += 1;
                        if tag.unmodelled {
                            return Err(UNMODELLED_ATTRIBUTE);
                        }
                        if arrays.iter().any(|area| area.holds(reference)) {
                            return Err(IN_SHARED_OR_ARRAY);
                        }
                        if empty {
                            out.write_all(&replaced_cell(
                                reference,
                                tag.style.as_deref(),
                                value.as_ref(),
                                &prefix_of(start),
                            ))?;
                        } else {
                            cell = Some((
                                depth,
                                InCell::Replace {
                                    reference,
                                    style: tag.style,
                                    prefix: prefix_of(start),
                                    value,
                                    formula: false,
                                },
                            ));
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

/// Reads the workbook part's `<sheets>`, skipping everything else.
fn parse_workbook_sheets<R: Read + Seek>(
    archive: &mut Archive<R>,
    index: usize,
) -> Result<Vec<RawSheet>, Error> {
    let mut stream = Stream::new(archive.read(index)?, index)?;
    let mut scope = Scope::new(index);
    let mut root = false;
    /// The root element's depth, so `<sheets>` is 2 and a `<sheet>` 3.
    const ROOT_DEPTH: usize = 1;
    let mut depth = 0usize;
    let mut sheets_at: Option<usize> = None;
    let mut out = Vec::new();
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
        }
        if empty {
            scope.end();
            if sheets_at == Some(depth) {
                sheets_at = None;
            }
            depth = depth.saturating_sub(1);
        }
    }
    Ok(out)
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
