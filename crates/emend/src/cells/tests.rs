//! Byte-built packages for the shared strings table.
//!
//! `specs/0010-reading-cells.md`, "P1": every fixture is a real zip built
//! byte by byte through [`crate::testing`], and every case goes through
//! [`Package::shared_strings`] rather than [`SharedStrings::read`] alone, so
//! the relationship, the `Override`, the entry and the parse are all covered
//! by the test that names the behaviour.
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
    let sheet = format!("{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"><sheetData/></worksheet>");

    let mut specs = vec![
        Spec::new(b"[Content_Types].xml", content_types.as_bytes()).deflated(),
        Spec::new(b"_rels/.rels", package_rels.as_bytes()).deflated(),
        Spec::new(b"xl/workbook.xml", workbook.as_bytes()).deflated(),
        Spec::new(b"xl/_rels/workbook.xml.rels", workbook_rels.as_bytes()).deflated(),
        Spec::new(b"xl/worksheets/sheet1.xml", sheet.as_bytes()).deflated(),
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
    // rather than a position. A flag cleared at the first `</t>` inside the
    // subtree is live again by the time `y` arrives, so it reads "ayb" where
    // the depth reads "ab". An earlier version of this test put the nested
    // `<t>` last inside `<rPh>`, so the premature clear happened after the
    // only inner text had already been skipped and the flag passed the test
    // too; independent check 2 of this phase built the flag and proved it.
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
    // than every `<t>` in a place the schema allows one. No producer writes
    // that and `specs/0010` asks for the rule as written, so it is raised to
    // planning as a question and not decided here.
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
