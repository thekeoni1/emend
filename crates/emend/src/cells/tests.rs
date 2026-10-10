//! Byte-built packages for the shared strings table and the cell reader.
//!
//! `specs/0010-reading-cells.md`, "P1" and "P2": every fixture is a real zip
//! built byte by byte through [`crate::testing`], and every case goes through
//! [`Package::shared_strings`] or [`Package::cells`] rather than
//! [`SharedStrings::read`] or [`Cells`] alone, so the relationship, the
//! `Override`, the entry and the parse are all covered by the test that names
//! the behaviour.
//!
//! The package scaffolding here is this module's own rather than shared with
//! `package::tests`: those helpers are private to that module, and moving
//! them would mean re-running part 2's committed tests to prove the move
//! changed nothing. What is shared is the zip builder, which part 2 already
//! put in [`crate::testing`] for exactly this.

use std::io::Cursor;

use super::*;
use crate::package::Package;
use crate::testing::{Build, Spec};

const TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const MAIN_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
const ATTR_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const DECLARATION: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>";
const WORKBOOK_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
const WORKSHEET_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
const STRINGS_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml";

/// A package with the five parts every fixture needs, plus a shared strings
/// part and relationship as asked for.
///
/// `strings` of `Some` adds `xl/sharedStrings.xml` as an entry with the
/// `Override` a producer writes for it; `target` of `Some` adds the
/// relationship pointing at that target. They are separate arguments because
/// the two absences are different cases: no relationship at all, and a
/// relationship naming a target the package has no entry for. The bytes are
/// `&[u8]` and not `&str` so the fixture that is deliberately not UTF-8 goes
/// through the same builder as the rest.
fn build(strings: Option<&[u8]>, target: Option<&str>) -> Vec<u8> {
    build_with_sheet(None, strings, target)
}

/// The same package, with the worksheet part's bytes given.
///
/// `sheet` of `None` is the empty `<sheetData/>` every P1 fixture carried, so
/// P1's cases go through this function unchanged; the cell reader's fixtures
/// pass their own worksheet. The bytes are `&[u8]` for the same reason the
/// shared strings part's are: the fixture that is deliberately not UTF-8 goes
/// through the same builder as the rest.
fn build_with_sheet(sheet: Option<&[u8]>, strings: Option<&[u8]>, target: Option<&str>) -> Vec<u8> {
    let strings_override = match strings {
        Some(_) => {
            format!("<Override PartName=\"/xl/sharedStrings.xml\" ContentType=\"{STRINGS_TYPE}\"/>")
        }
        None => String::new(),
    };
    let content_types = format!(
        "{DECLARATION}\n<Types xmlns=\"{TYPES_NS}\">\
         <Default Extension=\"rels\" \
         ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
         <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
         <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
         <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>\
         {strings_override}</Types>"
    );
    let package_rels = format!(
        "{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\">\
         <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/officeDocument\" \
         Target=\"xl/workbook.xml\"/></Relationships>"
    );
    let workbook = format!(
        "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
         <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
    );
    let strings_rel = match target {
        Some(target) => format!(
            "<Relationship Id=\"rId2\" Type=\"{ATTR_NS}/sharedStrings\" Target=\"{target}\"/>"
        ),
        None => String::new(),
    };
    let workbook_rels = format!(
        "{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\">\
         <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/worksheet\" \
         Target=\"worksheets/sheet1.xml\"/>{strings_rel}</Relationships>"
    );
    let default_sheet =
        format!("{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"><sheetData/></worksheet>");
    let sheet = sheet.unwrap_or(default_sheet.as_bytes());

    let mut specs = vec![
        Spec::new(b"[Content_Types].xml", content_types.as_bytes()).deflated(),
        Spec::new(b"_rels/.rels", package_rels.as_bytes()).deflated(),
        Spec::new(b"xl/workbook.xml", workbook.as_bytes()).deflated(),
        Spec::new(b"xl/_rels/workbook.xml.rels", workbook_rels.as_bytes()).deflated(),
        Spec::new(b"xl/worksheets/sheet1.xml", sheet).deflated(),
    ];
    if let Some(body) = strings {
        specs.push(Spec::new(b"xl/sharedStrings.xml", body).deflated());
    }
    Build::new(specs).bytes()
}

/// The package a fixture makes, opened.
fn opened(strings: Option<&[u8]>, target: Option<&str>) -> Package<Cursor<Vec<u8>>> {
    Package::open(Cursor::new(build(strings, target))).expect("the fixture opens")
}

/// The table of a package whose shared strings part is `body`.
fn table(body: &str) -> SharedStrings {
    opened(Some(body.as_bytes()), Some("sharedStrings.xml"))
        .shared_strings()
        .expect("the table parses")
}

/// What a package whose shared strings part is `body` is refused with.
fn refused(body: &[u8]) -> Error {
    opened(Some(body), Some("sharedStrings.xml"))
        .shared_strings()
        .expect_err("the fixture was accepted")
}

/// An `sst` part in the main namespace holding `body` between its root tags.
fn sst(body: &str) -> String {
    format!("{DECLARATION}\n<sst xmlns=\"{MAIN_NS}\">{body}</sst>")
}

/// Asserts that `error` is [`Error::Package`] carrying exactly `message`.
///
/// [`Error`] has no `PartialEq`, deliberately: an error that names a zip
/// entry is not equal to the same error naming another. `package::tests`
/// spells the same two helpers for the same reason.
#[track_caller]
fn assert_package(error: Error, message: &str) {
    match error {
        Error::Package(what) => assert_eq!(what, message, "the message"),
        other => panic!("expected a package error, got {other:?}"),
    }
}

/// Asserts that `error` is [`Error::Xml`], whatever entry it names.
#[track_caller]
fn assert_xml(error: Error) {
    match error {
        Error::Xml(_) => {}
        other => panic!("expected an xml error, got {other:?}"),
    }
}

/// Every item of a table, read back through the public accessor.
fn items(table: &SharedStrings) -> Vec<&str> {
    (0..table.len())
        .map(|index| table.get(index).expect("within the length"))
        .collect()
}

#[test]
fn a_plain_item_is_its_text() {
    let table = table(&sst("<si><t>hello</t></si>"));
    assert_eq!(items(&table), ["hello"]);
    assert_eq!(table.len(), 1);
    assert!(!table.is_empty());
}

#[test]
fn the_runs_of_an_item_concatenate() {
    // A producer splits one string into runs to give parts of it different
    // formatting. The string is the concatenation; the formatting is not
    // modelled here, and the `<rPr>` is in the fixture to prove a run's
    // properties are not mistaken for its text.
    let table = table(&sst(
        "<si><r><rPr><b/></rPr><t>bold</t></r><r><t> and plain</t></r></si>",
    ));
    assert_eq!(items(&table), ["bold and plain"]);
}

#[test]
fn a_phonetic_subtree_is_skipped() {
    // `<rPh>` is a reading shown beside the text, not part of it, and it
    // holds a `<t>` of its own: a reader that took every `<t>` under the item
    // would concatenate the reading into the string.
    let table = table(&sst(
        "<si><t>base</t><rPh sb=\"0\" eb=\"1\"><t>reading</t></rPh>\
         <phoneticPr fontId=\"1\"/></si>",
    ));
    assert_eq!(items(&table), ["base"]);
}

#[test]
fn the_phonetic_skip_ends_at_its_own_end_tag() {
    // The skip is a depth and not a flag, so text after `<rPh>` closes
    // belongs to the item again and a `<t>` nested deeper inside it does not
    // end the skip early.
    //
    // The `<t>y</t>` AFTER the nested `<r>` is what makes this a measurement
    // rather than a position, and two flags are measured, not one. A flag
    // that suppresses TEXT while the nesting is still counted is clear again
    // by the time `y` arrives and reads "ayb"; a flag cleared at the first
    // end tag inside the subtree ends the skip there, so the `</r>` closes
    // the item instead and it reads "a". Both are planted as M8 and M9 in
    // this phase's `test.log` and both fail this test, which is what the
    // sentence above rests on. An earlier version of this test put the
    // nested `<t>` last inside `<rPh>`, where the premature clear came after
    // the only inner text had already been skipped and a flag passed.
    let table = table(&sst(
        "<si><t>a</t><rPh sb=\"0\" eb=\"1\"><r><t>x</t></r><t>y</t></rPh>\
         <t>b</t></si>",
    ));
    assert_eq!(items(&table), ["ab"]);
}

#[test]
fn references_are_expanded() {
    // `quick-xml` 0.42 splits a text run at every reference, so this also
    // pins that the text either side of one is kept, and in order.
    let table = table(&sst(
        "<si><t>a&amp;b&lt;c&gt;d&quot;e&apos;f</t></si><si><t>&#65;&#x42;</t></si>",
    ));
    assert_eq!(items(&table), ["a&b<c>d\"e'f", "AB"]);
}

// P1 ruling 1: XML 1.0 section 2.11 line ends. Four cases, each comparing
// what the reader gives with the string the test wrote. `quick-xml` 0.42 does
// not normalise, so without the normalisation in `push_text` the first two
// read back the bytes the fixture wrote and these tests fail.

#[test]
fn a_carriage_return_line_feed_pair_becomes_one_line_feed() {
    let table = table(&sst("<si><t>a\r\nb</t></si>"));
    assert_eq!(items(&table), ["a\nb"]);
}

#[test]
fn a_lone_carriage_return_becomes_a_line_feed() {
    let table = table(&sst("<si><t>a\rb</t></si>"));
    assert_eq!(items(&table), ["a\nb"]);
}

#[test]
fn a_line_feed_alone_is_unchanged() {
    // The case that must NOT change, so the normalisation is not just
    // "replace every line end with something".
    let table = table(&sst("<si><t>a\nb</t></si>"));
    assert_eq!(items(&table), ["a\nb"]);
}

#[test]
fn a_carriage_return_through_a_reference_is_kept() {
    // Section 2.11 normalises the literal input before parsing, so a CR a
    // reference stands for is not a line end and survives as CR. This is the
    // one way a caller can still get a CR out of this reader, and the corpus
    // writes no `&#13;` anywhere.
    let table = table(&sst("<si><t>a&#13;b</t></si>"));
    assert_eq!(items(&table), ["a\rb"]);
}

#[test]
fn line_ends_normalise_inside_cdata_too() {
    // 2.11 is about the literal input, and a CDATA section is literal input:
    // the same pair in a CDATA section is the same one LF.
    let table = table(&sst("<si><t><![CDATA[a\r\nb]]></t></si>"));
    assert_eq!(items(&table), ["a\nb"]);
}

#[test]
fn a_reference_between_two_line_ends_separates_them() {
    // The CR LF state is carried across chunks, so this pins that a reference
    // between a lone CR and a lone LF leaves TWO line ends and does not
    // swallow the second: "x\r" then `&#65;` then "\nz".
    let table = table(&sst("<si><t>x\r&#65;\nz</t></si>"));
    assert_eq!(items(&table), ["x\nA\nz"]);
}

// P1 ruling 2: CDATA is text. The corpus carries none, so these two tests and
// `line_ends_normalise_inside_cdata_too` are the only thing proving it.

#[test]
fn an_item_wholly_in_cdata_is_its_text() {
    // Before ruling 2 this read as the empty string: `quick-xml` delivers
    // CDATA as its own event and the reader ignored it, so the item lost its
    // text with no error at all.
    let table = table(&sst("<si><t><![CDATA[hello]]></t></si>"));
    assert_eq!(items(&table), ["hello"]);
}

#[test]
fn text_on_both_sides_of_a_cdata_section_concatenates() {
    // And in order, which an implementation that appended CDATA after the
    // text rather than in place would fail.
    let table = table(&sst("<si><t>a<![CDATA[b]]>c</t></si>"));
    assert_eq!(items(&table), ["abc"]);
}

// P1 ruling 3: an element inside simple content is refused.

#[test]
fn an_element_inside_a_text_element_is_refused() {
    // Both spellings, because `quick-xml` reports them as different events
    // and a reader that refused only `Start` would let `<br/>` through.
    assert_package(
        refused(sst("<si><t>a<b>x</b>c</t></si>").as_bytes()),
        "a text element holds an element",
    );
    assert_package(
        refused(sst("<si><t>a<b/>c</t></si>").as_bytes()),
        "a text element holds an element",
    );
}

#[test]
fn spaces_are_kept_with_and_without_xml_space() {
    // The reader never trims, so the attribute makes no difference to it.
    // Both forms are here because the corpus writes both.
    let table = table(&sst(
        "<si><t xml:space=\"preserve\">  padded  </t></si><si><t>  also padded  </t></si>",
    ));
    assert_eq!(items(&table), ["  padded  ", "  also padded  "]);
}

#[test]
fn an_empty_text_and_an_empty_item_are_both_the_empty_string() {
    // An item that is there and empty is not an item that is missing: the
    // indexes after it would otherwise shift, which is what the third item
    // is here to catch.
    let table = table(&sst(
        "<si><t/></si><si/><si><t>after</t></si><si><t></t></si>",
    ));
    assert_eq!(items(&table), ["", "", "after", ""]);
    assert_eq!(table.len(), 4);
}

#[test]
fn count_and_unique_count_are_ignored() {
    // The corpus has parts whose attributes disagree with their contents, so
    // the items are what is there and the attributes are not read at all.
    let body = format!(
        "{DECLARATION}\n<sst xmlns=\"{MAIN_NS}\" count=\"99\" uniqueCount=\"7\">\
         <si><t>one</t></si><si><t>two</t></si></sst>"
    );
    let table = table(&body);
    assert_eq!(items(&table), ["one", "two"]);
    assert_eq!(table.len(), 2, "an attribute became the length");
}

#[test]
fn text_in_another_namespace_is_not_the_item_s_text() {
    // An `extLst` carries an extension's own vocabulary, so the `<t>` a
    // producer puts in there is some other schema's `t`. Only the main
    // namespace's is this item's text, which is the property, and it holds
    // however deeply the foreign element nests.
    //
    // A `<t>` in the MAIN namespace inside an `extLst` would be read as this
    // item's text, because the rule is every `<t>` under the item rather
    // than every `<t>` in a place the schema allows one. Planning ruled on
    // that in this part's doc commit: `specs/0010-reading-cells.md`, ruling
    // 7, records it as a known reading with no corpus witness and no change.
    let body = format!(
        "{DECLARATION}\n<sst xmlns=\"{MAIN_NS}\" xmlns:x=\"http://example.invalid/ext\">\
         <si><t>kept</t><extLst><x:ext uri=\"{{0}}\"><x:t>not text</x:t></x:ext>\
         </extLst></si></sst>"
    );
    assert_eq!(items(&table(&body)), ["kept"]);
}

#[test]
fn the_strict_namespace_is_accepted() {
    // Both spellings of the main namespace, as everywhere else in the crate.
    let body = format!(
        "{DECLARATION}\n<sst xmlns=\"http://purl.oclc.org/ooxml/spreadsheetml/main\">\
         <si><t>strict</t></si></sst>"
    );
    assert_eq!(items(&table(&body)), ["strict"]);
}

#[test]
fn get_past_the_end_is_none() {
    let table = table(&sst("<si><t>only</t></si>"));
    assert_eq!(table.get(0), Some("only"));
    assert_eq!(table.get(table.len()), None, "get at the length");
    assert_eq!(table.get(u32::MAX), None);
}

#[test]
fn a_workbook_with_no_shared_strings_gives_an_empty_table() {
    // No relationship, no part, no `Override`: an empty table rather than an
    // error or an `Option`, so a cell naming an index in such a workbook is
    // out of range and not a case of its own.
    let table = opened(None, None)
        .shared_strings()
        .expect("an empty table, not an error");
    assert_eq!(table.len(), 0);
    assert!(table.is_empty());
    assert_eq!(table.get(0), None);
}

#[test]
fn a_relationship_naming_no_entry_is_refused() {
    // A relationship that names a part the archive does not hold is a claim
    // the package makes about itself and cannot keep, so following it and
    // finding nothing is an error and not an empty table (P1 ruling 5). This
    // is the case the test above is NOT: there, the workbook says it has no
    // table, and an empty one is the right answer.
    //
    // The two cases share a method and differed only in this, so the pair is
    // what pins the distinction: the same call, one refusal, one empty table.
    let error = opened(None, Some("absent.xml"))
        .shared_strings()
        .expect_err("a refusal, not an empty table");
    assert_package(error, "no such part");
}

#[test]
fn a_root_of_the_wrong_name_is_refused() {
    let body = format!("{DECLARATION}\n<strings xmlns=\"{MAIN_NS}\"><si><t>x</t></si></strings>");
    assert_package(
        refused(body.as_bytes()),
        "the shared strings part's root is not sst",
    );
}

#[test]
fn a_root_in_another_namespace_is_refused() {
    // The local name alone is not enough: `sst` in some other vocabulary is
    // not this part.
    let body = format!("{DECLARATION}\n<sst xmlns=\"http://example.invalid/other\"/>");
    assert_package(
        refused(body.as_bytes()),
        "the shared strings part's root is not sst",
    );
}

#[test]
fn a_part_with_no_root_at_all_is_refused() {
    // The same refusal from the other side of the reader: a part that ends
    // without ever naming a root is not an empty table.
    let body = format!("{DECLARATION}\n<!-- nothing here -->");
    assert_package(
        refused(body.as_bytes()),
        "the shared strings part's root is not sst",
    );
}

#[test]
fn an_unknown_entity_is_refused() {
    // A value with a hole in it is not the value. The reference is
    // well-formed XML, so nothing else in the stack refuses it.
    assert_package(
        refused(sst("<si><t>a&nbsp;b</t></si>").as_bytes()),
        "a cell holds a reference this layer cannot expand",
    );
}

#[test]
fn text_that_is_not_utf8_is_an_xml_error() {
    // Not a package refusal: the part is not XML this reader can decode, and
    // the index in the error is the zip entry's, as everywhere else.
    let mut body = sst("<si><t>ok</t></si>").into_bytes();
    let at = body
        .windows(2)
        .position(|window| window == b"ok")
        .expect("the fixture holds the text it replaces");
    body[at] = 0xff;
    assert_xml(refused(&body));
}

#[test]
fn the_fixture_builder_makes_a_package_the_reader_accepts() {
    // The control for every refusal above: the same builder, with nothing
    // wrong with it, is read without error. Without this a refusal could be
    // the fixture's own fault and the test would still pass.
    let table = table(&sst("<si><t>control</t></si>"));
    assert_eq!(items(&table), ["control"]);
}

#[test]
fn a_second_call_gives_an_equal_table() {
    // What this pins is the PROPERTY a caller depends on: a second call gives
    // a table equal to the first, and both are owned, so neither borrows the
    // package and either can be held while cells stream through it.
    //
    // It was called `the_table_is_read_afresh_on_every_call`, which named a
    // MECHANISM these assertions cannot tell apart from caching the table and
    // handing back a clone: that implementation would satisfy both and the old
    // name would be false. Independent check 1 of P1 found it and P1 ruling 9
    // renamed it. The reading afresh is visible in the signature, not here.
    let mut package = opened(
        Some(sst("<si><t>x</t></si>").as_bytes()),
        Some("sharedStrings.xml"),
    );
    let first = package.shared_strings().expect("the table");
    let second = package.shared_strings().expect("the table again");
    assert_eq!(first, second);
    assert_eq!(items(&first), ["x"]);
}

// ---------------------------------------------------------------------------
// P2: the cell reader.
//
// Every fixture is a worksheet built byte by byte and read through
// `Package::cells`, and every assertion compares a cell's fields with what
// the fixture wrote. The shared strings table is not involved except in the
// `resolve` cases, which build one too.
// ---------------------------------------------------------------------------

const SHEET_PART: &[u8] = b"xl/worksheets/sheet1.xml";

/// A worksheet in the main namespace whose `<sheetData>` holds `rows`.
fn worksheet(rows: &str) -> String {
    format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"><sheetData>{rows}</sheetData></worksheet>"
    )
}

/// A worksheet whose `<sheetData>` holds `rows`, followed by `siblings`.
///
/// The siblings go AFTER `<sheetData>`, where a producer writes conditional
/// formatting and the rest, so a fixture can put an unknown element beside
/// the cells as well as inside one.
fn worksheet_with_siblings(rows: &str, siblings: &str) -> String {
    format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"><sheetData>{rows}</sheetData>\
         {siblings}</worksheet>"
    )
}

/// A worksheet holding one row of `cells`, the row numbered 1.
fn one_row(cells: &str) -> String {
    worksheet(&format!("<row r=\"1\">{cells}</row>"))
}

/// A worksheet holding one row of `cells`, the row numbered as given.
///
/// Separate from [`one_row`] because the row number is the subject of two
/// tests rather than scaffolding for them: the spelling is passed through
/// verbatim, so a test can give one `u32::from_str` accepts and the A1 form
/// does not.
fn one_row_in_row(number: &str, cells: &str) -> String {
    worksheet(&format!("<row r=\"{number}\">{cells}</row>"))
}

/// The package a worksheet fixture makes, opened.
fn opened_sheet(sheet: &[u8], strings: Option<&[u8]>) -> Package<Cursor<Vec<u8>>> {
    let target = strings.map(|_| "sharedStrings.xml");
    Package::open(Cursor::new(build_with_sheet(Some(sheet), strings, target)))
        .expect("the fixture opens")
}

/// Every cell of a package whose worksheet part is `sheet`.
fn read_cells(sheet: &str) -> Vec<Cell> {
    let mut package = opened_sheet(sheet.as_bytes(), None);
    package
        .cells(SHEET_PART)
        .expect("the part opens")
        .collect::<Result<Vec<_>, Error>>()
        .expect("the cells parse")
}

/// The one cell of a package whose worksheet part is `sheet`.
#[track_caller]
fn one_cell(sheet: &str) -> Cell {
    let mut cells = read_cells(sheet);
    assert_eq!(cells.len(), 1, "one cell");
    cells.remove(0)
}

/// What a package whose worksheet part is `sheet` is refused with.
///
/// The refusal may come from opening the part or from any cell after it, so
/// this drives the whole iterator and returns the first error, and panics if
/// every cell was accepted.
fn refused_sheet(sheet: &[u8]) -> Error {
    let mut package = opened_sheet(sheet, None);
    let cells = match package.cells(SHEET_PART) {
        Ok(cells) => cells,
        Err(error) => return error,
    };
    for cell in cells {
        match cell {
            Ok(_) => {}
            Err(error) => return error,
        }
    }
    panic!("the fixture was accepted");
}

/// The A1 reference of every cell, in the order they were read.
fn references(cells: &[Cell]) -> Vec<String> {
    cells.iter().map(|cell| cell.reference.a1()).collect()
}

#[test]
fn every_cell_type_reads_as_its_kind() {
    // One cell per `t` the layer knows, in the order the spec lists them, so
    // a kind mapped to the wrong name shows up as a mismatch here rather
    // than as a wrong value in a caller.
    let cells = read_cells(&one_row(
        "<c r=\"A1\"><v>1</v></c>\
         <c r=\"B1\" t=\"n\"><v>2</v></c>\
         <c r=\"C1\" t=\"s\"><v>0</v></c>\
         <c r=\"D1\" t=\"inlineStr\"><is><t>in</t></is></c>\
         <c r=\"E1\" t=\"str\"><f>A1</f><v>cached</v></c>\
         <c r=\"F1\" t=\"b\"><v>1</v></c>\
         <c r=\"G1\" t=\"e\"><v>#REF!</v></c>\
         <c r=\"H1\" t=\"d\"><v>2026-09-18</v></c>",
    ));
    let kinds: Vec<CellKind> = cells.iter().map(|cell| cell.kind).collect();
    assert_eq!(
        kinds,
        [
            CellKind::Number,
            CellKind::Number,
            CellKind::SharedString,
            CellKind::InlineString,
            CellKind::FormulaString,
            CellKind::Boolean,
            CellKind::Error,
            CellKind::Date,
        ]
    );
    assert_eq!(
        references(&cells),
        ["A1", "B1", "C1", "D1", "E1", "F1", "G1", "H1"]
    );
    // The values travel with the kinds: a cell whose kind was right and
    // whose value came from the wrong element would pass the list above.
    let values: Vec<Option<&str>> = cells.iter().map(|cell| cell.value.as_deref()).collect();
    assert_eq!(
        values,
        [
            Some("1"),
            Some("2"),
            Some("0"),
            Some("in"),
            Some("cached"),
            Some("1"),
            Some("#REF!"),
            Some("2026-09-18"),
        ]
    );
}

#[test]
fn a_number_is_a_number_with_or_without_its_type() {
    // The corpus writes both spellings: 1302755 cells with `t="n"` and 8233
    // without any `t` at all (`specs/0010-reading-cells.md`, "Harness
    // runs"). They are the same kind and the fixture proves the absence is
    // not read as some other kind.
    let cells = read_cells(&one_row(
        "<c r=\"A1\"><v>1.10</v></c><c r=\"B1\" t=\"n\"><v>1.10</v></c>",
    ));
    assert_eq!(cells[0].kind, CellKind::Number);
    assert_eq!(cells[1].kind, CellKind::Number);
    // Nothing is normalised: the trailing zero is the producer's text and
    // stays. `number()` parses it on request, which is P3's.
    assert_eq!(cells[0].value.as_deref(), Some("1.10"));
    assert_eq!(cells[1].value.as_deref(), Some("1.10"));
}

#[test]
fn a_value_may_be_text_empty_or_absent() {
    // Three different answers, and the difference matters to a caller: `<v/>`
    // is a cell with an empty value, and a cell with no `<v>` has none. A
    // reader that folded them would report the second as empty text.
    let cells = read_cells(&one_row(
        "<c r=\"A1\"><v>7</v></c><c r=\"B1\"><v/></c><c r=\"C1\"/>\
         <c r=\"D1\"></c>",
    ));
    let values: Vec<Option<&str>> = cells.iter().map(|cell| cell.value.as_deref()).collect();
    assert_eq!(values, [Some("7"), Some(""), None, None]);
    assert_eq!(references(&cells), ["A1", "B1", "C1", "D1"]);
}

#[test]
fn an_inline_string_is_the_text_under_its_is() {
    let cell = one_cell(&one_row(
        "<c r=\"A1\" t=\"inlineStr\"><is><t>inline</t></is></c>",
    ));
    assert_eq!(cell.value.as_deref(), Some("inline"));
    assert_eq!(cell.kind, CellKind::InlineString);
}

#[test]
fn the_runs_of_an_inline_string_concatenate() {
    // The same rule as an `<si>`: the string is the concatenation of the
    // runs' `<t>` texts, and a run's properties are not its text.
    let cell = one_cell(&one_row(
        "<c r=\"A1\" t=\"inlineStr\"><is>\
         <r><rPr><b/></rPr><t>bold</t></r><r><t> and plain</t></r></is></c>",
    ));
    assert_eq!(cell.value.as_deref(), Some("bold and plain"));
}

#[test]
fn a_phonetic_subtree_inside_an_inline_string_is_skipped() {
    // `<is>` and `<si>` are read by one function, so this is the same rule
    // proved on the other container. The `<t>y</t>` AFTER the nested `<r>` is
    // what makes it a measurement rather than a position: a flag that
    // suppressed text while still counting the nesting reads "ayb" here, and
    // a flag cleared at the first end tag inside the subtree reads "a".
    // M8 and M9 of this phase's `test.log` plant both and print both.
    let cell = one_cell(&one_row(
        "<c r=\"A1\" t=\"inlineStr\"><is><t>a</t>\
         <rPh sb=\"0\" eb=\"1\"><r><t>x</t></r><t>y</t></rPh><t>b</t></is></c>",
    ));
    assert_eq!(cell.value.as_deref(), Some("ab"));
}

#[test]
fn a_normal_formula_is_its_text() {
    // Both spellings of a normal formula: `t` absent, and `t="normal"`.
    let cells = read_cells(&one_row(
        "<c r=\"A1\"><f>SUM(B1:B9)</f><v>45</v></c>\
         <c r=\"B1\"><f t=\"normal\">1+1</f><v>2</v></c>",
    ));
    let first = cells[0].formula.as_ref().expect("a formula");
    assert_eq!(first.text, "SUM(B1:B9)");
    assert_eq!(first.kind, FormulaKind::Normal);
    assert_eq!(first.reference, None);
    // The cached value is the cell's value: emend stores formulas and reads
    // cached values, and evaluates nothing (`CLAUDE.md`).
    assert_eq!(cells[0].value.as_deref(), Some("45"));
    let second = cells[1].formula.as_ref().expect("a formula");
    assert_eq!(second.text, "1+1");
    assert_eq!(second.kind, FormulaKind::Normal);
}

#[test]
fn a_shared_master_carries_its_range_and_its_follower_carries_no_text() {
    // The corpus has 44 masters and 292 followers
    // (`specs/0010-reading-cells.md`, "Harness runs"). `master` is whether
    // `ref` was there, which is how the format marks the one cell holding
    // the expression the group follows.
    let cells = read_cells(&one_row(
        "<c r=\"A1\"><f t=\"shared\" ref=\"A1:A3\" si=\"0\">B1*2</f><v>2</v></c>\
         <c r=\"A2\"><f t=\"shared\" si=\"0\"/><v>4</v></c>",
    ));
    let master = cells[0].formula.as_ref().expect("a formula");
    assert_eq!(
        master.kind,
        FormulaKind::Shared {
            index: 0,
            master: true
        }
    );
    assert_eq!(master.text, "B1*2");
    assert_eq!(master.reference.as_deref(), Some("A1:A3"));
    let follower = cells[1].formula.as_ref().expect("a formula");
    assert_eq!(
        follower.kind,
        FormulaKind::Shared {
            index: 0,
            master: false
        }
    );
    // Empty, because a follower carries no expression of its own: the
    // master's is the group's, and this layer does not copy it over.
    assert_eq!(follower.text, "");
    assert_eq!(follower.reference, None);
}

#[test]
fn an_array_formula_carries_its_range() {
    let cell = one_cell(&one_row(
        "<c r=\"A1\"><f t=\"array\" ref=\"A1:A2\">TRANSPOSE(B1:C1)</f><v>1</v></c>",
    ));
    let formula = cell.formula.as_ref().expect("a formula");
    assert_eq!(formula.kind, FormulaKind::Array);
    assert_eq!(formula.reference.as_deref(), Some("A1:A2"));
    assert_eq!(formula.text, "TRANSPOSE(B1:C1)");
}

#[test]
fn a_data_table_formula_is_accepted_by_name() {
    // The corpus holds no `dataTable` witness, so it is accepted by name and
    // its own attributes are not modelled: `dt2D`, `dtr` and the rest are
    // not read, and part 4's rewrite copies them raw.
    let cell = one_cell(&one_row(
        "<c r=\"A1\"><f t=\"dataTable\" ref=\"B2:C3\" dt2D=\"1\" dtr=\"0\" \
         r1=\"A1\"/><v>1</v></c>",
    ));
    let formula = cell.formula.as_ref().expect("a formula");
    assert_eq!(formula.kind, FormulaKind::DataTable);
    assert_eq!(formula.reference.as_deref(), Some("B2:C3"));
    assert_eq!(formula.text, "");
}

#[test]
fn a_formulas_other_attributes_are_not_modelled() {
    // `ca` and `aca` are in the fixture to prove they are neither read nor
    // refused: the formula that comes back is the same as one without them,
    // and a caller that needs them waits for part 4's raw copy.
    let with = one_cell(&one_row(
        "<c r=\"A1\"><f ca=\"1\" aca=\"1\">RAND()</f><v>0.5</v></c>",
    ));
    let without = one_cell(&one_row("<c r=\"A1\"><f>RAND()</f><v>0.5</v></c>"));
    assert_eq!(with.formula, without.formula);
    assert_eq!(
        with.formula.as_ref().expect("a formula").kind,
        FormulaKind::Normal
    );
}

#[test]
fn a_character_reference_is_expanded_in_a_value_a_formula_and_a_text() {
    // One function assembles all three, so this proves the one place rather
    // than three. `&#65;` is `A` and `&amp;` is `&`.
    let cells = read_cells(&one_row(
        "<c r=\"A1\" t=\"str\"><f>CONCAT(&quot;x&quot;,&#65;1)</f><v>&#65;&amp;b</v></c>\
         <c r=\"B1\" t=\"inlineStr\"><is><t>&#65;&amp;b</t></is></c>",
    ));
    assert_eq!(cells[0].value.as_deref(), Some("A&b"));
    assert_eq!(
        cells[0].formula.as_ref().expect("a formula").text,
        "CONCAT(\"x\",A1)"
    );
    assert_eq!(cells[1].value.as_deref(), Some("A&b"));
}

#[test]
fn a_cell_reference_parses_only_the_a1_form() {
    assert_eq!(CellRef::parse(b"A1"), Some(CellRef { row: 1, col: 1 }));
    assert_eq!(
        CellRef::parse(b"XFD1048576"),
        Some(CellRef {
            row: 1048576,
            col: 16384
        })
    );
    // Lower case, a leading zero, letters alone, digits alone, and letters
    // after the digits are each refused rather than guessed at.
    assert_eq!(CellRef::parse(b"a1"), None);
    assert_eq!(CellRef::parse(b"A01"), None);
    assert_eq!(CellRef::parse(b"A"), None);
    assert_eq!(CellRef::parse(b"1"), None);
    assert_eq!(CellRef::parse(b"A1B"), None);
    assert_eq!(CellRef::parse(b""), None);
    // A column past `u32::MAX`. `MWLQKWU` is 4294967295 exactly, the largest
    // that fits, and `MWLQKWV` is one more, so the pair shows the boundary is
    // where it is claimed and not one column either side of it. Both
    // spellings were computed rather than recalled: the first draft of this
    // test named `FXSHRXW`, which is `i32::MAX` and not `u32::MAX`, and the
    // assertion caught it.
    assert_eq!(
        CellRef::parse(b"MWLQKWU1"),
        Some(CellRef {
            row: 1,
            col: u32::MAX
        })
    );
    assert_eq!(CellRef::parse(b"MWLQKWV1"), None);
    // A row past `u32::MAX`, the same boundary on the other axis.
    assert_eq!(
        CellRef::parse(b"A4294967295"),
        Some(CellRef {
            row: u32::MAX,
            col: 1
        })
    );
    assert_eq!(CellRef::parse(b"A4294967296"), None);
}

#[test]
fn a_reference_writes_itself_back() {
    // Round trip through both directions, on the two boundaries of the
    // base-26 spelling that a naive one gets wrong: `Z` to `AA`, and the
    // largest reference Excel itself writes.
    for spelling in ["A1", "Z1", "AA1", "AZ9", "BA10", "XFD1048576", "MWLQKWU1"] {
        let parsed = CellRef::parse(spelling.as_bytes()).expect("parses");
        assert_eq!(parsed.a1(), spelling, "{spelling}");
    }
}

#[test]
fn a_row_without_a_number_follows_the_last() {
    // The first row with no `r` is row 1, and a later one is the last plus
    // one: the numbering is a running count and not the row's position among
    // the rows, so the third row here is 6 and not 3.
    let cells = read_cells(&worksheet(
        "<row><c><v>1</v></c></row>\
         <row><c><v>2</v></c></row>\
         <row r=\"5\"><c><v>3</v></c></row>\
         <row><c><v>4</v></c></row>",
    ));
    assert_eq!(references(&cells), ["A1", "A2", "A5", "A6"]);
}

#[test]
fn a_cell_without_a_reference_follows_the_previous() {
    // Within a row the column is a running count too, and it restarts at
    // each row: the second row's first cell is A and not D. A cell with an
    // explicit `r` moves the count, so the cell after C1 is D1.
    let cells = read_cells(&worksheet(
        "<row r=\"1\"><c><v>1</v></c><c><v>2</v></c><c r=\"C1\"><v>3</v></c>\
         <c><v>4</v></c></row>\
         <row r=\"2\"><c><v>5</v></c></row>",
    ));
    assert_eq!(references(&cells), ["A1", "B1", "C1", "D1", "A2"]);
}

#[test]
fn a_cells_own_reference_wins_over_its_row() {
    // Not refused: the cell's own reference is what every other reader
    // takes, and the row's attributes are not modelled here
    // (`specs/0010-reading-cells.md`, "Module `cells`").
    //
    // The cell AFTER it is C1, not C7: a cell with no `r` has no reference of
    // its own to win with, so its column counts from the previous cell's, as
    // the spec says, and its row is the enclosing `<row>`'s. This expectation
    // read C7 in the first draft, and the assertion caught it; the two halves
    // of a derived reference come from two different places, and this is the
    // fixture that shows which.
    let cells = read_cells(&worksheet(
        "<row r=\"1\"><c r=\"B7\"><v>1</v></c><c><v>2</v></c></row>",
    ));
    assert_eq!(references(&cells), ["B7", "C1"]);
    assert_eq!(cells[0].reference.row, 7);
    assert_eq!(cells[1].reference.row, 1);
}

#[test]
fn a_style_index_is_carried() {
    // 2094287 of the corpus's 2094601 cells carry an `s`
    // (`specs/0010-reading-cells.md`, "Harness runs"). The styles part is
    // not read here: the index is carried and nothing is looked up.
    let cells = read_cells(&one_row(
        "<c r=\"A1\" s=\"0\"><v>1</v></c><c r=\"B1\" s=\"164\"><v>2</v></c>\
         <c r=\"C1\"><v>3</v></c>",
    ));
    let styles: Vec<Option<u32>> = cells.iter().map(|cell| cell.style).collect();
    assert_eq!(styles, [Some(0), Some(164), None]);
}

#[test]
fn an_unknown_element_is_skipped_with_its_subtree() {
    // Two places at once: an `extLst` INSIDE a `<c>`, whose subtree holds a
    // `<v>` that must not become the cell's value, and a
    // `conditionalFormatting` sibling of `<sheetData>`, whose subtree holds a
    // `<c>` that must not become a cell. A reader that walked into either
    // would report three cells or the wrong value.
    let cells = read_cells(&worksheet_with_siblings(
        "<row r=\"1\"><c r=\"A1\"><v>1</v>\
         <extLst><ext uri=\"{X}\"><v>999</v><c r=\"Z9\"><v>8</v></c></ext></extLst></c></row>",
        "<conditionalFormatting sqref=\"A1\"><cfRule type=\"expression\">\
         <row r=\"9\"><c r=\"Z9\"><v>7</v></c></row></cfRule></conditionalFormatting>",
    ));
    assert_eq!(references(&cells), ["A1"]);
    assert_eq!(cells[0].value.as_deref(), Some("1"));
}

#[test]
fn a_cell_in_another_namespace_is_not_a_cell() {
    // The strict main namespace: the elements are matched on namespace and
    // local name, so a `<c>` a producer put in a namespace of its own is an
    // unknown element and is skipped with its subtree, `<v>` and all.
    let cells = read_cells(&worksheet(
        "<row r=\"1\"><c r=\"A1\"><v>1</v></c>\
         <other:c xmlns:other=\"urn:other\" r=\"B1\"><other:v>2</other:v></other:c>\
         <c><v>3</v></c></row>",
    ));
    // The third cell carries NO `r`, which is what makes the count visible:
    // its column comes from the previous cell's, so a reader that had
    // counted the foreign element would report C1 here and the answer is
    // B1. An earlier version of this test gave that cell `r="C1"`, which
    // overwrote the running count either way and made the claim in this
    // comment unmeasurable; independent check 1 of this phase found it.
    assert_eq!(references(&cells), ["A1", "B1"]);
    assert_eq!(cells[1].value.as_deref(), Some("3"));
}

#[test]
fn the_other_main_namespace_spelling_is_read_too() {
    // The STRICT spelling, written out. An earlier version reached it as
    // `MAIN[1]`, a position in the constant: reorder `MAIN` and this test
    // would silently exercise the transitional spelling and still pass,
    // leaving the spec's strict-namespace clause untested. That is check 1's
    // C12 and P2 ruling 8, which asks for the spelling by its bytes. The
    // literal is also asserted to be one of the crate's own two spellings,
    // so the test cannot drift from the constant either.
    const STRICT: &str = "http://purl.oclc.org/ooxml/spreadsheetml/main";
    assert!(
        MAIN.iter().any(|spelling| *spelling == STRICT.as_bytes()),
        "the literal is one of the crate's main-namespace spellings"
    );
    let sheet = format!(
        "{DECLARATION}\n<worksheet xmlns=\"{STRICT}\"><sheetData>\
         <row r=\"1\"><c r=\"A1\"><v>1</v></c></row></sheetData></worksheet>"
    );
    let cells = read_cells(&sheet);
    assert_eq!(references(&cells), ["A1"]);
}

#[test]
fn an_empty_sheet_data_gives_no_cells() {
    let cells = read_cells(&format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"><sheetData/></worksheet>"
    ));
    assert!(cells.is_empty(), "no cells");
    // And a worksheet with no `<sheetData>` at all gives none either, rather
    // than being refused: a sheet may hold nothing but its properties.
    let none = read_cells(&format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\">\
         <sheetPr/><dimension ref=\"A1\"/></worksheet>"
    ));
    assert!(none.is_empty(), "no cells");
}

#[test]
fn a_worksheet_root_of_the_wrong_name_is_refused() {
    let sheet = format!("{DECLARATION}\n<sheet xmlns=\"{MAIN_NS}\"><sheetData/></sheet>");
    assert_package(
        refused_sheet(sheet.as_bytes()),
        "the worksheet part's root is not worksheet",
    );
}

#[test]
fn a_worksheet_root_in_another_namespace_is_refused() {
    // The right local name in the wrong namespace is not the right root: the
    // namespace is half of an element's name.
    let sheet = format!("{DECLARATION}\n<worksheet xmlns=\"urn:other\"><sheetData/></worksheet>");
    assert_package(
        refused_sheet(sheet.as_bytes()),
        "the worksheet part's root is not worksheet",
    );
}

#[test]
fn a_cell_type_this_layer_does_not_know_is_refused() {
    // Refused rather than guessed at: a reader that fell back to `Number`
    // would report a wrong value as a right one.
    assert_package(
        refused_sheet(one_row("<c r=\"A1\" t=\"z\"><v>1</v></c>").as_bytes()),
        "a cell has a type this layer does not know",
    );
}

#[test]
fn a_formula_type_this_layer_does_not_know_is_refused() {
    assert_package(
        refused_sheet(one_row("<c r=\"A1\"><f t=\"cse\">1</f></c>").as_bytes()),
        "a formula has a type this layer does not know",
    );
}

#[test]
fn a_shared_formula_without_an_index_is_refused() {
    assert_package(
        refused_sheet(one_row("<c r=\"A1\"><f t=\"shared\" ref=\"A1:A2\">1</f></c>").as_bytes()),
        "a shared formula has no index",
    );
}

#[test]
fn a_shared_index_that_is_not_a_number_is_refused() {
    // The same message as an absent `si`: either way this layer has no index
    // to carry, and the catalogue holds one message for the case.
    assert_package(
        refused_sheet(one_row("<c r=\"A1\"><f t=\"shared\" si=\"x\">1</f></c>").as_bytes()),
        "a shared formula has no index",
    );
}

#[test]
fn a_cell_reference_that_is_not_a1_is_refused() {
    assert_package(
        refused_sheet(one_row("<c r=\"A1:B2\"><v>1</v></c>").as_bytes()),
        "a cell reference is not of the form A1",
    );
}

#[test]
fn a_row_number_that_is_not_a_number_is_refused() {
    assert_package(
        refused_sheet(worksheet("<row r=\"one\"><c><v>1</v></c></row>").as_bytes()),
        "a row number is not a number",
    );
}

#[test]
fn a_style_index_that_is_not_a_number_is_refused() {
    assert_package(
        refused_sheet(one_row("<c r=\"A1\" s=\"x\"><v>1</v></c>").as_bytes()),
        "a cell's style index is not a number",
    );
}

#[test]
fn an_unknown_entity_in_a_cell_is_refused() {
    // In a value, in a formula and in an inline string alike: all three go
    // through one function, and a value with a hole in it is not the value.
    for body in [
        "<c r=\"A1\"><v>&nbsp;</v></c>",
        "<c r=\"A1\"><f>&nbsp;</f></c>",
        "<c r=\"A1\" t=\"inlineStr\"><is><t>&nbsp;</t></is></c>",
    ] {
        assert_package(
            refused_sheet(one_row(body).as_bytes()),
            "a cell holds a reference this layer cannot expand",
        );
    }
}

#[test]
fn an_element_inside_a_cells_value_is_refused() {
    // `<v>` and `<f>` are simple content in the schema, as `<t>` is, and the
    // same refusal covers all three.
    for body in [
        "<c r=\"A1\"><v>a<b/>c</v></c>",
        "<c r=\"A1\"><f>a<b/>c</f></c>",
    ] {
        assert_package(
            refused_sheet(one_row(body).as_bytes()),
            "a text element holds an element",
        );
    }
}

#[test]
fn a_worksheet_that_is_not_utf8_is_refused() {
    // The same class of refusal as the shared strings part's: not UTF-8 is
    // `Error::Xml` carrying the entry's index, in every part.
    let mut sheet = one_row("<c r=\"A1\"><v>").into_bytes();
    sheet.push(0xff);
    sheet.extend_from_slice(b"</v></c></row></sheetData></worksheet>");
    assert_xml(refused_sheet(&sheet));
}

#[test]
fn a_worksheet_cut_off_mid_cell_is_refused() {
    // The part ends before the cell's end tag, which is an XML error and not
    // a cell with a missing field.
    let sheet = format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"><sheetData>\
         <row r=\"1\"><c r=\"A1\"><v>1</v>"
    );
    assert_xml(refused_sheet(sheet.as_bytes()));
}

#[test]
fn the_iterator_is_spent_after_a_refusal() {
    // A reader that carried on from an unknown position would be guessing,
    // so the error is the last item: one `Some(Err)`, then `None`.
    let sheet = one_row("<c r=\"A1\" t=\"z\"><v>1</v></c><c r=\"B1\"><v>2</v></c>");
    let mut package = opened_sheet(sheet.as_bytes(), None);
    let mut cells = package.cells(SHEET_PART).expect("the part opens");
    assert!(cells.next().expect("an item").is_err(), "the refusal");
    assert!(cells.next().is_none(), "spent");
    assert!(cells.next().is_none(), "still spent");
}

#[test]
fn cells_on_a_part_that_is_not_there_is_refused() {
    let mut package = opened(None, None);
    // Matched rather than `expect_err`: a reader that is streaming a part
    // has nothing to print, so `Cells` is not `Debug`.
    let error = match package.cells(b"xl/worksheets/sheet9.xml") {
        Ok(_) => panic!("a refusal, not an empty read"),
        Err(error) => error,
    };
    assert_package(error, "no such part");
}

#[test]
fn resolving_a_shared_string_gives_the_item_it_names() {
    let sheet = one_row("<c r=\"A1\" t=\"s\"><v>1</v></c>");
    let strings = sst("<si><t>zero</t></si><si><t>one</t></si>");
    let mut package = opened_sheet(sheet.as_bytes(), Some(strings.as_bytes()));
    let table = package.shared_strings().expect("the table");
    let cells: Vec<Cell> = package
        .cells(SHEET_PART)
        .expect("the part opens")
        .collect::<Result<_, Error>>()
        .expect("the cells parse");
    // The table is owned, so it is held here WHILE the cells stream through
    // a mutable borrow of the same package: that is the reason
    // `shared_strings` hands its table over rather than lending it.
    assert_eq!(table.resolve(&cells[0]).expect("resolves"), Some("one"));
}

#[test]
fn resolving_a_cell_of_another_kind_gives_its_own_value() {
    let cells = read_cells(&one_row(
        "<c r=\"A1\"><v>1.5</v></c><c r=\"B1\" t=\"inlineStr\"><is><t>in</t></is></c>\
         <c r=\"C1\"/>",
    ));
    let table = SharedStrings::default();
    assert_eq!(table.resolve(&cells[0]).expect("resolves"), Some("1.5"));
    assert_eq!(table.resolve(&cells[1]).expect("resolves"), Some("in"));
    // No value at all resolves to nothing, and is not an error.
    assert_eq!(table.resolve(&cells[2]).expect("resolves"), None);
}

#[test]
fn resolving_an_index_that_is_not_a_number_is_refused() {
    let cells = read_cells(&one_row("<c r=\"A1\" t=\"s\"><v>x</v></c>"));
    let table = SharedStrings::default();
    assert_package(
        table
            .resolve(&cells[0])
            .expect_err("a refusal, not a missing item"),
        "a shared string index is not a number",
    );
}

#[test]
fn resolving_an_index_out_of_range_is_refused() {
    let cells = read_cells(&one_row("<c r=\"A1\" t=\"s\"><v>2</v></c>"));
    let table = table(&sst("<si><t>zero</t></si><si><t>one</t></si>"));
    assert_package(
        table
            .resolve(&cells[0])
            .expect_err("a refusal, not a missing item"),
        "a shared string index is out of range",
    );
    // A workbook with no table at all is the same answer and not a case of
    // its own: the index is out of range of an empty table.
    assert_package(
        SharedStrings::default()
            .resolve(&cells[0])
            .expect_err("a refusal"),
        "a shared string index is out of range",
    );
}

#[test]
fn reading_cells_then_writing_copies_the_package_byte_for_byte() {
    // Reading records no edit, so the output is the input's bytes: the
    // fidelity harness's whole claim, proved here on a package the test
    // built rather than on a corpus file.
    let sheet = one_row("<c r=\"A1\" t=\"s\"><v>0</v></c><c r=\"B1\"><f>A1</f><v>1</v></c>");
    let input = build_with_sheet(
        Some(sheet.as_bytes()),
        Some(sst("<si><t>x</t></si>").as_bytes()),
        Some("sharedStrings.xml"),
    );
    let mut package = Package::open(Cursor::new(input.clone())).expect("the fixture opens");
    let read: Vec<Cell> = package
        .cells(SHEET_PART)
        .expect("the part opens")
        .collect::<Result<_, Error>>()
        .expect("the cells parse");
    assert_eq!(references(&read), ["A1", "B1"]);
    let mut out = Vec::new();
    package.write_to(&mut out).expect("the copy writes");
    // Kept for the open check, as `written` keeps the package tests' writes
    // (`specs/0015-open-checks.md`, "The keep").
    crate::testing::keep(&input, &out);
    assert_eq!(out, input, "byte for byte");
}

#[test]
fn a_shared_string_cell_with_no_value_resolves_to_nothing() {
    // `t="s"` with no `<v>` at all: there is no index to look up, so
    // `resolve` answers `Ok(None)` rather than refusing. The distinction
    // matters to a counter: `specs/0010-reading-cells.md`, "Harness runs",
    // counts `resolved` and `unresolved` as the cells whose `resolve`
    // SUCCEEDS and FAILS, so such a cell counts as resolved although no item
    // was read. Independent check 2 of this phase built the package and
    // found `--cells` and the run log's own second reader disagreeing on it;
    // the corpus holds none, and this test is what pins the answer.
    let cells = read_cells(&one_row("<c r=\"A1\" t=\"s\"/>"));
    let table = table(&sst("<si><t>zero</t></si>"));
    assert_eq!(cells[0].kind, CellKind::SharedString);
    assert_eq!(
        table.resolve(&cells[0]).expect("a success, not a refusal"),
        None
    );
}

#[test]
fn the_crate_root_re_exports_every_cell_type() {
    // `specs/0010-reading-cells.md`, "Crate": `lib.rs` re-exports
    // `cells::{Cell, CellRef, Cells, Formula, FormulaKind, Kind,
    // SharedStrings}` (`Kind` renamed `CellKind` by `specs/0030`). The clause
    // is about paths, so the test names each type through the crate root and
    // the compiler is the assertion: it fails to build if a re-export is
    // missing. Both independent checks of this phase
    // found the six P2 types reachable only as `emend::cells::Cell`.
    use crate::{Cell as R1, CellRef as R2, Formula as R4, FormulaKind as R5};
    use crate::{CellKind as R6, SharedStrings as R7};
    let _: fn(&[u8]) -> Option<R2> = R2::parse;
    let _: R6 = R6::Number;
    let _: R5 = R5::Normal;
    let table = R7::default();
    assert_eq!(table.len(), 0);
    let cells = read_cells(&one_row("<c r=\"A1\"><f>A1</f><v>1</v></c>"));
    let cell: &R1 = &cells[0];
    let formula: &R4 = cell.formula.as_ref().expect("the formula");
    assert_eq!(formula.kind, R5::Normal);
    // `Cells` is the seventh name, and it is reached as a type through the
    // crate root in the same way: `Package::cells` returns it.
    let sheet = one_row("<c r=\"A1\"><v>1</v></c>");
    let mut package = opened_sheet(sheet.as_bytes(), None);
    let stream: crate::Cells<'_, Cursor<Vec<u8>>> = package.cells(SHEET_PART).expect("the part");
    assert_eq!(stream.count(), 1);
}

#[test]
fn two_cells_past_the_last_column_share_the_last_reference() {
    // What the reader does at the top of the column range, pinned rather than
    // left to be discovered. A cell at the last column `u32` can hold, then
    // two cells with no `r` of their own: the running count saturates, so
    // both take the same reference as the cell before them. Three cells are
    // read and two carry an identical `CellRef`.
    //
    // Saturation is the only alternative to wrapping or refusing, and the
    // format's own last column is XFD (16384), so no producer can reach
    // this. A caller keying cells by reference would nevertheless collapse
    // the two, which is why it is written down here rather than left as a
    // silent property. Raised as finding P2-F3 and kept by P2 ruling 3.
    let cells = read_cells(&one_row(
        "<c r=\"MWLQKWU1\"><v>1</v></c><c><v>2</v></c><c><v>3</v></c>",
    ));
    assert_eq!(cells.len(), 3);
    assert_eq!(cells[0].reference.col, u32::MAX);
    assert_eq!(cells[1].reference, cells[0].reference);
    assert_eq!(cells[2].reference, cells[0].reference);
    assert_eq!(cells[2].value.as_deref(), Some("3"));
}

#[test]
fn a_row_number_of_zero_is_refused() {
    // Finding P2-F2, ruled: a row is numbered from 1, so `r="0"` is refused
    // rather than read. It used to be accepted, which gave a cell whose
    // `a1()` was `A0`, a spelling `CellRef::parse` refuses for its leading
    // zero, so the reader could emit a reference its own parser would not
    // take back. The message is the one an unreadable row number already
    // had (P2 ruling 2).
    assert_package(
        refused_sheet(one_row_in_row("0", "<c><v>1</v></c>").as_bytes()),
        "a row number is not a number",
    );
}

#[test]
fn every_reference_the_reader_emits_parses_back() {
    // The round trip P2-F2 broke, now asserted as a property over every
    // reference a sheet yields rather than over a list of spellings: each
    // cell's `a1()` goes back through `CellRef::parse` and must give the
    // same `CellRef`.
    //
    // The sheet holds the cases that could break it: an explicit `r`, a cell
    // with no `r` after one, a row with no `r`, the largest row and column
    // the format itself writes, and the saturated column two cells share.
    // The saturated pair round trips too: ruling 3 keeps saturation, and
    // what it costs is that two cells carry one reference, not that the
    // reference is unwritable.
    let cells = read_cells(&worksheet(
        "<row r=\"1\"><c r=\"A1\"><v>1</v></c><c><v>2</v></c></row>\
         <row><c><v>3</v></c></row>\
         <row r=\"1048576\"><c r=\"XFD1048576\"><v>4</v></c></row>\
         <row r=\"4294967295\"><c r=\"MWLQKWU4294967295\"><v>5</v></c>\
         <c><v>6</v></c></row>",
    ));
    assert_eq!(cells.len(), 6);
    for cell in &cells {
        let written = cell.reference.a1();
        assert_eq!(
            CellRef::parse(written.as_bytes()),
            Some(cell.reference),
            "{written}"
        );
    }
    assert_eq!(cells[4].reference.col, u32::MAX);
    assert_eq!(cells[5].reference, cells[4].reference);
}

#[test]
fn a_number_this_layer_reads_is_digits_alone() {
    // Ruling 2: one helper reads every number this layer takes out of an
    // attribute, so one spelling is a number everywhere or nowhere. The four
    // attributes are a row's `r`, a cell's `s`, a formula's `si` and the
    // shared string index, and each has its own message.
    //
    // TWO of the eight spellings are ones `u32::from_str` would have taken
    // and the format does not write: `+1` and `007`, which is what ruling 2
    // changed. `from_str` refuses the other six itself, measured in
    // `what_the_standard_librarys_parse_takes_and_this_refuses`, so for
    // those six this test pins behaviour this phase did not change. They are
    // here as coverage of the four sites, not as evidence about the old
    // reader. An earlier draft of this comment said all eight were spellings
    // `from_str` would have taken; the independent checks of P3 measured
    // `from_str` and found that false.
    //
    // `si` is the one that is not refused by message: an unreadable index is
    // the same case as an absent one (ruling 4), so the refusal is "a shared
    // formula has no index".
    for spelling in ["+1", "007", "0x1", " 1", "1 ", "1.0", "-1", ""] {
        assert_package(
            refused_sheet(one_row_in_row(spelling, "<c/>").as_bytes()),
            "a row number is not a number",
        );
        assert_package(
            refused_sheet(
                one_row(&format!("<c r=\"A1\" s=\"{spelling}\"><v>1</v></c>")).as_bytes(),
            ),
            "a cell's style index is not a number",
        );
        assert_package(
            refused_sheet(
                one_row(&format!(
                    "<c r=\"A1\"><f t=\"shared\" si=\"{spelling}\">1</f></c>"
                ))
                .as_bytes(),
            ),
            "a shared formula has no index",
        );
        let cells = read_cells(&one_row(&format!(
            "<c r=\"A1\" t=\"s\"><v>{spelling}</v></c>"
        )));
        assert_package(
            SharedStrings::default()
                .resolve(&cells[0])
                .expect_err("a refusal, not an item"),
            "a shared string index is not a number",
        );
    }
}

/// What `u32::from_str` takes that `decimal` does not, measured rather than
/// described, because `decimal`'s doc comment says what is delegated and what
/// is refused first and both halves are claims about the standard library.
///
/// This is the only test in the crate that asserts the standard library's own
/// behaviour. It is here because a sentence in a doc comment of this crate
/// depends on it, and because a toolchain that widened or narrowed the parse
/// would change what `decimal` delegates: the two checks before the
/// delegation would then be the wrong two.
#[test]
fn what_the_standard_librarys_parse_takes_and_this_refuses() {
    // The two `decimal` refuses and `from_str` takes.
    assert_eq!("+1".parse::<u32>(), Ok(1));
    assert_eq!("+0".parse::<u32>(), Ok(0));
    assert_eq!("007".parse::<u32>(), Ok(7));
    assert_eq!(decimal("+1"), None);
    assert_eq!(decimal("+0"), None);
    assert_eq!(decimal("007"), None);
    // A leading minus is NOT one of them: an unsigned parse refuses it, so
    // the refusal a caller sees for a signed spelling is the standard
    // library's and not this helper's added rule.
    assert!("-1".parse::<u32>().is_err());
    assert!("-0".parse::<u32>().is_err());
    // And the six the format does not write that `from_str` refuses too, so
    // `a_number_this_layer_reads_is_digits_alone`'s list is not eight
    // spellings the old reader accepted.
    for spelling in ["0x1", " 1", "1 ", "1.0", "-1", ""] {
        assert!(spelling.parse::<u32>().is_err(), "{spelling}");
        assert_eq!(decimal(spelling), None, "{spelling}");
    }
    // The range is delegated, which is the other half of the doc comment.
    assert_eq!(decimal("4294967295"), Some(u32::MAX));
    assert_eq!(decimal("4294967296"), None);
    assert!("4294967296".parse::<u32>().is_err());
}

#[test]
fn zero_is_a_number_where_zero_is_a_legal_index() {
    // The other half of ruling 2: the helper refuses a LEADING zero, not the
    // number 0, because a style index and a shared string index both count
    // from 0. Only `row_number` refuses zero, and it does so itself.
    let cells = read_cells(&one_row("<c r=\"A1\" s=\"0\" t=\"s\"><v>0</v></c>"));
    assert_eq!(cells[0].style, Some(0));
    let table = table(&sst("<si><t>zero</t></si>"));
    assert_eq!(table.resolve(&cells[0]).expect("resolves"), Some("zero"));
    let shared = read_cells(&one_row("<c r=\"A1\"><f t=\"shared\" si=\"0\">1</f></c>"));
    assert_eq!(
        shared[0].formula.as_ref().map(|formula| formula.kind),
        Some(FormulaKind::Shared {
            index: 0,
            master: false
        })
    );
}

#[test]
fn an_index_past_u32_is_refused_like_one_that_is_not_a_number() {
    // Ruling 4: an `si` past `u32::MAX` keeps the message it has, which says
    // the formula has no index. It has one; this layer cannot carry it, and
    // the catalogue holds one message for every `si` it cannot read. Pinned
    // so that a later ruling that splits the two changes a test.
    assert_package(
        refused_sheet(
            one_row("<c r=\"A1\"><f t=\"shared\" si=\"4294967296\">1</f></c>").as_bytes(),
        ),
        "a shared formula has no index",
    );
    assert_package(
        refused_sheet(one_row_in_row("4294967296", "<c/>").as_bytes()),
        "a row number is not a number",
    );
    assert_package(
        refused_sheet(one_row("<c r=\"A1\" s=\"4294967296\"><v>1</v></c>").as_bytes()),
        "a cell's style index is not a number",
    );
}

// The typed views: `Cell::number`, `Cell::boolean` and `Cell::error`
// (`specs/0010-reading-cells.md`, "P3"). P2 implemented the three because
// `--cells` counts `numbers_parsed` and `booleans_parsed` through the first
// two; the cases below are the ones P3's section lists, and `error` had no
// caller and no test at all until here.

#[test]
fn a_numbers_text_is_read_as_an_f64() {
    // The three spellings the spec names, each a thing a naive reader gets
    // wrong: a trailing zero in the fraction, an exponent with a sign, and a
    // negative zero.
    let cells = read_cells(&one_row(
        "<c r=\"A1\"><v>1.10</v></c><c r=\"B1\"><v>1E+3</v></c>\
         <c r=\"C1\"><v>-0</v></c>",
    ));
    assert_eq!(cells[0].number(), Some(1.1));
    assert_eq!(cells[1].number(), Some(1000.0));
    // `-0.0 == 0.0` is true, so the equality above would hold for either
    // sign: the sign bit is the assertion here.
    let zero = cells[2].number().expect("parses");
    assert_eq!(zero, 0.0);
    assert!(zero.is_sign_negative(), "-0 keeps its sign");
}

#[test]
fn the_parse_is_the_standard_librarys_and_wider_than_the_format() {
    // What `str::parse::<f64>` accepts, measured here rather than described
    // in the doc comment alone, because `number`'s doc names these cases and
    // `specs/0010` asks for `inf` and `nan` "as the standard library parses
    // them and stated as such".
    //
    // A spreadsheet writes none of them. They are here so that a caller
    // knows a cell reading `Some(inf)` may be a producer's literal text and
    // not an overflow this layer computed, and so that a later change to
    // narrow the parse changes a test.
    for spelling in ["inf", "INF", "infinity", "1e309"] {
        let cells = read_cells(&one_row(&format!("<c r=\"A1\"><v>{spelling}</v></c>")));
        let number = cells[0].number().expect("parses");
        assert!(number.is_infinite(), "{spelling}");
        assert!(number.is_sign_positive(), "{spelling}");
    }
    for spelling in ["-inf", "-1e309"] {
        let cells = read_cells(&one_row(&format!("<c r=\"A1\"><v>{spelling}</v></c>")));
        let number = cells[0].number().expect("parses");
        assert!(number.is_infinite(), "{spelling}");
        assert!(number.is_sign_negative(), "{spelling}");
    }
    for spelling in ["nan", "NaN"] {
        let cells = read_cells(&one_row(&format!("<c r=\"A1\"><v>{spelling}</v></c>")));
        // `NaN != NaN`, so `assert_eq!` against `f64::NAN` would fail on a
        // correct answer: the predicate is the assertion.
        assert!(cells[0].number().expect("parses").is_nan(), "{spelling}");
    }
    // A leading plus is accepted here and refused by `decimal`: a producer's
    // number is not one of the format's index tokens (P2 ruling 2).
    let plus = read_cells(&one_row("<c r=\"A1\"><v>+1</v></c>"));
    assert_eq!(plus[0].number(), Some(1.0));
}

#[test]
fn a_number_cell_whose_text_is_not_a_number_has_none() {
    // The spec's "a non-number `Number` cell `None`", and the cases either
    // side of it: surrounding space, which `f64` refuses, and a cell with no
    // value at all, which is the 5410 the corpus holds.
    for spelling in ["x", " 1", "1 ", "1,5", "0x1", "TRUE", ""] {
        let cells = read_cells(&one_row(&format!("<c r=\"A1\"><v>{spelling}</v></c>")));
        assert_eq!(cells[0].number(), None, "{spelling}");
    }
    let empty = read_cells(&one_row("<c r=\"A1\"/>"));
    assert_eq!(empty[0].kind, CellKind::Number);
    assert_eq!(empty[0].value, None);
    assert_eq!(empty[0].number(), None);
}

#[test]
fn a_booleans_text_is_read_as_a_bool() {
    // `1` and `0` are what the format writes, and `TRUE` is the spelling a
    // caller expects to work and does not: the answer is `None`, not an
    // error, because a producer's text is not this layer's to correct.
    let cells = read_cells(&one_row(
        "<c r=\"A1\" t=\"b\"><v>1</v></c><c r=\"B1\" t=\"b\"><v>0</v></c>\
         <c r=\"C1\" t=\"b\"><v>TRUE</v></c>",
    ));
    assert_eq!(cells[0].boolean(), Some(true));
    assert_eq!(cells[1].boolean(), Some(false));
    assert_eq!(cells[2].boolean(), None);
    for spelling in ["true", "-1", "2", "01", ""] {
        let cells = read_cells(&one_row(&format!(
            "<c r=\"A1\" t=\"b\"><v>{spelling}</v></c>"
        )));
        assert_eq!(cells[0].boolean(), None, "{spelling}");
    }
    let valueless = read_cells(&one_row("<c r=\"A1\" t=\"b\"/>"));
    assert_eq!(valueless[0].kind, CellKind::Boolean);
    assert_eq!(valueless[0].boolean(), None);
}

#[test]
fn an_error_cells_text_is_its_error() {
    // The error is carried as written: this layer does not name the errors,
    // so a spelling it has never seen comes back whole rather than as `None`.
    let cells = read_cells(&one_row(
        "<c r=\"A1\" t=\"e\"><v>#REF!</v></c><c r=\"B1\" t=\"e\"><v>#DIV/0!</v></c>\
         <c r=\"C1\" t=\"e\"><v>#SOMETHING_NEW!</v></c><c r=\"D1\" t=\"e\"/>",
    ));
    assert_eq!(cells[0].error(), Some("#REF!"));
    assert_eq!(cells[1].error(), Some("#DIV/0!"));
    assert_eq!(cells[2].error(), Some("#SOMETHING_NEW!"));
    assert_eq!(cells[3].kind, CellKind::Error);
    assert_eq!(cells[3].error(), None);
}

#[test]
fn each_typed_view_answers_only_for_its_own_kind() {
    // One cell of every kind the format writes, each asked all three
    // questions. The point is the whole table at once: a view that forgot
    // its kind check would answer for a neighbour, and the shared-string
    // cell is the one that would be read as the number 0 if `number` went by
    // the text alone.
    let table = table(&sst("<si><t>zero</t></si>"));
    let cells = read_cells(&one_row(
        "<c r=\"A1\"><v>1</v></c>\
         <c r=\"B1\" t=\"s\"><v>0</v></c>\
         <c r=\"C1\" t=\"str\"><f>A1</f><v>1</v></c>\
         <c r=\"D1\" t=\"b\"><v>1</v></c>\
         <c r=\"E1\" t=\"e\"><v>#REF!</v></c>\
         <c r=\"F1\" t=\"d\"><v>2026-09-18</v></c>\
         <c r=\"G1\" t=\"inlineStr\"><is><t>1</t></is></c>",
    ));
    let answers: Vec<(Option<f64>, Option<bool>, Option<&str>)> = cells
        .iter()
        .map(|cell| (cell.number(), cell.boolean(), cell.error()))
        .collect();
    assert_eq!(
        answers,
        [
            (Some(1.0), None, None),
            (None, None, None),
            (None, None, None),
            (None, Some(true), None),
            (None, None, Some("#REF!")),
            (None, None, None),
            (None, None, None),
        ]
    );
    // The shared-string cell's own value is an index, and `resolve` is the
    // only thing that reads it: the typed views leave it alone.
    assert_eq!(table.resolve(&cells[1]).expect("resolves"), Some("zero"));
    // A date is carried as its text and not converted, which is the one
    // place a reader is most tempted to help (`specs/0010`, "Module
    // `cells`": no date conversion).
    assert_eq!(cells[5].value.as_deref(), Some("2026-09-18"));
}
