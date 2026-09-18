//! Byte-built packages for the package layer.
//!
//! `specs/0009-package-layer.md`, "P2: content types, relationships, the
//! workbook": every fixture is a real zip built byte by byte through
//! [`crate::testing`], so nothing that builds a test package shares code with
//! the layer under test and the crate keeps its two dependencies.
//!
//! The smallest package is five parts: `[Content_Types].xml`, `_rels/.rels`,
//! `xl/workbook.xml`, `xl/_rels/workbook.xml.rels` and one worksheet. Each
//! test starts from [`smallest`] and replaces or removes exactly the part it
//! is about, so what a test changes is what it asserts.

use std::io::Cursor;

use super::*;
use crate::testing::{Build, Spec};

/// The namespaces the fixtures write, as text.
const TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const MAIN_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
const ATTR_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
/// The strict forms of the two above, for `strict_namespaces`.
const MAIN_STRICT: &str = "http://purl.oclc.org/ooxml/spreadsheetml/main";
const ATTR_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";

const DECLARATION: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>";

/// The content type of a worksheet part, used by several fixtures.
const WORKSHEET_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
/// The content type of the workbook part.
const WORKBOOK_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";

/// A `[Content_Types].xml` with `body` between its root tags.
fn types(body: &str) -> String {
    format!("{DECLARATION}\n<Types xmlns=\"{TYPES_NS}\">{body}</Types>")
}

/// A `.rels` part with `body` between its root tags.
fn rels(body: &str) -> String {
    format!("{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\">{body}</Relationships>")
}

/// One `<Relationship>` element.
fn relationship(id: &str, kind: &str, target: &str) -> String {
    format!("<Relationship Id=\"{id}\" Type=\"{kind}\" Target=\"{target}\"/>")
}

/// The five parts of the smallest package, in the order a producer writes
/// them.
fn smallest() -> Vec<(Vec<u8>, Vec<u8>)> {
    let content_types = types(&format!(
        "<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
         <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
         <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
         <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>"
    ));
    let package_rels = rels(&relationship(
        "rId1",
        &format!("{ATTR_NS}/officeDocument"),
        "xl/workbook.xml",
    ));
    let workbook = format!(
        "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
         <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
    );
    let workbook_rels = rels(&format!(
        "{}{}",
        relationship(
            "rId1",
            &format!("{ATTR_NS}/worksheet"),
            "worksheets/sheet1.xml"
        ),
        relationship("rId2", &format!("{ATTR_NS}/styles"), "styles.xml"),
    ));
    let worksheet =
        format!("{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"><sheetData/></worksheet>");
    vec![
        (b"[Content_Types].xml".to_vec(), content_types.into_bytes()),
        (b"_rels/.rels".to_vec(), package_rels.into_bytes()),
        (b"xl/workbook.xml".to_vec(), workbook.into_bytes()),
        (
            b"xl/_rels/workbook.xml.rels".to_vec(),
            workbook_rels.into_bytes(),
        ),
        (b"xl/worksheets/sheet1.xml".to_vec(), worksheet.into_bytes()),
        (b"xl/styles.xml".to_vec(), b"<styleSheet/>".to_vec()),
    ]
}

/// Replaces the bytes of the part named `name`, which must be there.
#[track_caller]
fn set(parts: &mut [(Vec<u8>, Vec<u8>)], name: &[u8], bytes: String) {
    let part = parts
        .iter_mut()
        .find(|(part, _)| part == name)
        .expect("the fixture has that part");
    part.1 = bytes.into_bytes();
}

/// Removes the part named `name`, which must be there.
#[track_caller]
fn drop_part(parts: &mut Vec<(Vec<u8>, Vec<u8>)>, name: &[u8]) {
    let at = parts
        .iter()
        .position(|(part, _)| part == name)
        .expect("the fixture has that part");
    parts.remove(at);
}

/// Builds the parts into a real archive and opens it as a package.
#[track_caller]
fn package(parts: Vec<(Vec<u8>, Vec<u8>)>) -> Package<Cursor<Vec<u8>>> {
    let specs = parts
        .iter()
        .map(|(name, data)| Spec::new(name, data))
        .collect();
    let bytes = Build::new(specs).bytes();
    Package::open(Cursor::new(bytes)).expect("the fixture opens")
}

/// The smallest package, opened.
fn smallest_package() -> Package<Cursor<Vec<u8>>> {
    package(smallest())
}

/// Asserts that `error` is [`Error::Package`] carrying exactly `message`.
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

// ------------------------------------------------------- the map resolves

#[test]
fn map_resolves() {
    let mut package = smallest_package();
    // Content types: both Defaults, both Overrides, in document order.
    let types = package.content_types().expect("content types");
    assert_eq!(types.defaults().len(), 2, "defaults");
    assert_eq!(types.defaults()[0].extension, b"rels", "the first default");
    assert_eq!(types.overrides().len(), 2, "overrides");
    assert_eq!(
        types.of(b"xl/worksheets/sheet1.xml"),
        Some(WORKSHEET_TYPE.as_bytes()),
        "the worksheet's type"
    );
    assert_eq!(
        types.of(b"_rels/.rels"),
        Some(b"application/vnd.openxmlformats-package.relationships+xml".as_slice()),
        "a type from a Default"
    );

    // The package's own relationships, and the workbook part they name.
    let package_rels = package.relationships(b"").expect("package rels");
    assert_eq!(package_rels.all().len(), 1, "the package's relationships");
    assert_eq!(package_rels.source(), b"", "the source");
    let office = package_rels
        .first_of(OFFICE_DOCUMENT)
        .expect("an officeDocument relationship");
    assert_eq!(office.id, b"rId1", "its id");
    assert!(!office.external, "it is internal");
    assert_eq!(
        package_rels.target_part(office).expect("its part"),
        b"xl/workbook.xml",
        "the workbook part"
    );

    // The workbook, its one sheet and its styles part.
    let workbook = package.workbook().expect("workbook");
    assert_eq!(workbook.part, b"xl/workbook.xml", "the workbook part");
    assert_eq!(workbook.sheets.len(), 1, "sheets");
    let sheet = &workbook.sheets[0];
    assert_eq!(sheet.name, "Sheet1", "the sheet name");
    assert_eq!(sheet.sheet_id, 1, "the sheet id");
    assert_eq!(sheet.rel_id, b"rId1", "the relationship id");
    assert_eq!(sheet.state, None, "no state");
    assert_eq!(sheet.part, b"xl/worksheets/sheet1.xml", "the sheet's part");
    assert_eq!(sheet.kind, SheetKind::Worksheet, "the sheet's kind");
    assert_eq!(
        workbook.styles.as_deref(),
        Some(b"xl/styles.xml".as_slice()),
        "the styles part"
    );
    assert_eq!(workbook.shared_strings, None, "no shared strings");
    assert_eq!(workbook.calc_chain, None, "no calculation chain");
    assert_eq!(workbook.theme, None, "no theme");
    assert!(workbook.sheet("Sheet1").is_some(), "found by name");
    assert!(workbook.sheet("sheet1").is_none(), "names match exactly");
}

// ---------------------------------------------------------------- targets

#[test]
fn target_absolute() {
    let mut parts = smallest();
    // Ten of the corpus's twenty-seven files write the workbook this way.
    set(
        &mut parts,
        b"_rels/.rels",
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/officeDocument"),
            "/xl/workbook.xml",
        )),
    );
    let mut package = package(parts);
    assert_eq!(
        package.workbook().expect("workbook").part,
        b"xl/workbook.xml",
        "an absolute target loses its slash"
    );
}

#[test]
fn target_relative() {
    let mut package = smallest_package();
    // `worksheets/sheet1.xml` joined to the workbook's directory.
    assert_eq!(
        package.workbook().expect("workbook").sheets[0].part,
        b"xl/worksheets/sheet1.xml",
        "a relative target joins the source's directory"
    );
}

#[test]
fn target_parent() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/worksheet"),
            "../other/./sheet1.xml",
        )),
    );
    let mut package = package(parts);
    assert_eq!(
        package.workbook().expect("workbook").sheets[0].part,
        b"other/sheet1.xml",
        "`..` climbs out of xl and `.` stays put"
    );
}

#[test]
fn target_above_root() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"_rels/.rels",
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/officeDocument"),
            "../workbook.xml",
        )),
    );
    let mut package = package(parts);
    // The package's own rels have no directory, so one `..` is already out.
    assert_package(
        package.workbook().expect_err("refused"),
        "a relationship target outside the package",
    );
}

#[test]
fn external_target() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}<Relationship Id=\"rId9\" Type=\"{ATTR_NS}/hyperlink\" \
             Target=\"https://example.invalid/x\" TargetMode=\"External\"/>",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet1.xml"
            ),
        )),
    );
    let mut package = package(parts);
    let workbook_rels = package
        .relationships(b"xl/workbook.xml")
        .expect("workbook rels");
    let hyperlink = workbook_rels.by_id(b"rId9").expect("the hyperlink");
    assert!(hyperlink.external, "TargetMode External");
    assert_eq!(
        hyperlink.target, b"https://example.invalid/x",
        "the target as written"
    );
    assert_package(
        workbook_rels
            .target_part(hyperlink)
            .expect_err("no part for it"),
        "an external relationship has no part",
    );
}

#[test]
fn rels_part_absent() {
    let mut package = smallest_package();
    let sheet_rels = package
        .relationships(b"xl/worksheets/sheet1.xml")
        .expect("an absent rels part is an empty set");
    assert!(sheet_rels.all().is_empty(), "nothing in it");
    assert_eq!(
        sheet_rels.source(),
        b"xl/worksheets/sheet1.xml",
        "it still knows its source"
    );
}

#[test]
fn rels_names_derive() {
    // The name the layer looks for, and the source the survey derives back.
    assert_eq!(rels_part_name(b""), b"_rels/.rels", "the package's own");
    assert_eq!(
        rels_part_name(b"xl/workbook.xml"),
        b"xl/_rels/workbook.xml.rels",
        "a part in a directory"
    );
    assert_eq!(
        rels_part_name(b"thing.xml"),
        b"_rels/thing.xml.rels",
        "a part at the root"
    );
    assert_eq!(
        rels_part_source(b"_rels/.rels").as_deref(),
        Some(b"".as_slice()),
        "back to the package"
    );
    assert_eq!(
        rels_part_source(b"xl/_rels/workbook.xml.rels").as_deref(),
        Some(b"xl/workbook.xml".as_slice()),
        "back to the part"
    );
    assert_eq!(
        rels_part_source(b"xl/workbook.xml"),
        None,
        "not a rels part at all"
    );
    assert_eq!(
        rels_part_source(b"xl/other/workbook.xml.rels"),
        None,
        "a .rels name outside a _rels directory"
    );
}

// ---------------------------------------------------------- content types

#[test]
fn override_beats_default() {
    let mut package = smallest_package();
    let types = package.content_types().expect("content types");
    assert_eq!(
        types.of(b"xl/workbook.xml"),
        Some(WORKBOOK_TYPE.as_bytes()),
        "the Override wins over the xml Default"
    );
}

#[test]
fn extension_matched_case_insensitively() {
    let mut parts = smallest();
    parts.push((b"xl/oddly.XML".to_vec(), b"<x/>".to_vec()));
    let mut package = package(parts);
    let types = package.content_types().expect("content types");
    assert_eq!(
        types.of(b"xl/oddly.XML"),
        Some(b"application/xml".as_slice()),
        "Extension=\"xml\" covers .XML"
    );
}

#[test]
fn part_with_no_content_type() {
    let mut parts = smallest();
    parts.push((b"docProps/thumbnail.jpeg".to_vec(), b"not a jpeg".to_vec()));
    let mut package = package(parts);
    let types = package.content_types().expect("content types");
    assert_eq!(
        types.of(b"docProps/thumbnail.jpeg"),
        None,
        "no Default and no Override covers it"
    );
    assert_eq!(
        types.of(b"noDotAtAll"),
        None,
        "a name with no extension has no Default either"
    );
}

// ------------------------------------------------------------- namespaces

#[test]
fn strict_namespaces() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"_rels/.rels",
        rels(&relationship(
            "rId1",
            &format!("{ATTR_STRICT}/officeDocument"),
            "xl/workbook.xml",
        )),
    );
    set(
        &mut parts,
        b"xl/workbook.xml",
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{MAIN_STRICT}\" xmlns:r=\"{ATTR_STRICT}\">\
             <sheets><sheet name=\"Strict\" sheetId=\"7\" r:id=\"rId1\"/></sheets></workbook>"
        ),
    );
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&relationship(
            "rId1",
            &format!("{ATTR_STRICT}/chartsheet"),
            "worksheets/sheet1.xml",
        )),
    );
    let mut package = package(parts);
    let workbook = package.workbook().expect("workbook");
    assert_eq!(workbook.sheets.len(), 1, "sheets");
    assert_eq!(workbook.sheets[0].name, "Strict", "the sheet name");
    assert_eq!(
        workbook.sheets[0].kind,
        SheetKind::Chartsheet,
        "the strict chartsheet type"
    );
}

/// The `r:id` prefix under three bindings: `r`, another name, and declared on
/// `<sheets>` rather than on the root.
#[test]
fn rel_prefix_bound_three_ways() {
    for workbook in [
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
             <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
        ),
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:zz=\"{ATTR_NS}\">\
             <sheets><sheet name=\"Sheet1\" sheetId=\"1\" zz:id=\"rId1\"/></sheets></workbook>"
        ),
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\">\
             <sheets xmlns:r=\"{ATTR_NS}\">\
             <sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
        ),
    ] {
        let mut parts = smallest();
        set(&mut parts, b"xl/workbook.xml", workbook);
        let mut package = package(parts);
        assert_eq!(
            package.workbook().expect("workbook").sheets[0].part,
            b"xl/worksheets/sheet1.xml",
            "the id resolves however its prefix is bound"
        );
    }
}

#[test]
fn rel_id_prefix_unbound() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/workbook.xml",
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\">\
             <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
        ),
    );
    let mut package = package(parts);
    assert_package(
        package.workbook().expect_err("refused"),
        "an element or attribute uses a prefix its document does not bind",
    );
}

// ----------------------------------------------------------------- sheets

#[test]
fn sheet_name_escapes() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/workbook.xml",
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
             <sheets><sheet name=\"A &amp; B&#32;C\" sheetId=\"1\" r:id=\"rId1\"/></sheets>\
             </workbook>"
        ),
    );
    let mut package = package(parts);
    let workbook = package.workbook().expect("workbook");
    assert_eq!(
        workbook.sheets[0].name, "A & B C",
        "a reference and a character reference are both read"
    );
    assert!(
        workbook.sheet("A & B C").is_some(),
        "the unescaped name is what `sheet` matches"
    );
}

#[test]
fn sheet_states() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/workbook.xml",
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><sheets>\
             <sheet name=\"Visible\" sheetId=\"1\" r:id=\"rId1\"/>\
             <sheet name=\"Hidden\" sheetId=\"2\" state=\"hidden\" r:id=\"rId1\"/>\
             <sheet name=\"Very\" sheetId=\"3\" state=\"veryHidden\" r:id=\"rId1\"/>\
             </sheets></workbook>"
        ),
    );
    let mut package = package(parts);
    let workbook = package.workbook().expect("workbook");
    assert_eq!(workbook.sheets[0].state, None, "no state attribute");
    assert_eq!(
        workbook.sheets[1].state.as_deref(),
        Some(b"hidden".as_slice()),
        "the state as written"
    );
    assert_eq!(
        workbook.sheets[2].state.as_deref(),
        Some(b"veryHidden".as_slice()),
        "and the other one"
    );
}

#[test]
fn several_sheets_in_order() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/workbook.xml",
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><sheets>\
             <sheet name=\"Third\" sheetId=\"30\" r:id=\"rId3\"/>\
             <sheet name=\"First\" sheetId=\"10\" r:id=\"rId1\"/>\
             <sheet name=\"Second\" sheetId=\"20\" r:id=\"rId2\"/>\
             </sheets></workbook>"
        ),
    );
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            relationship(
                "rId2",
                &format!("{ATTR_NS}/chartsheet"),
                "chartsheets/sheet1.xml"
            ),
            relationship(
                "rId3",
                &format!("{ATTR_NS}/dialogsheet"),
                "dialogsheets/sheet1.xml"
            ),
        )),
    );
    let mut package = package(parts);
    let workbook = package.workbook().expect("workbook");
    let order: Vec<(&str, u32, SheetKind)> = workbook
        .sheets
        .iter()
        .map(|sheet| (sheet.name.as_str(), sheet.sheet_id, sheet.kind))
        .collect();
    assert_eq!(
        order,
        vec![
            ("Third", 30, SheetKind::Dialogsheet),
            ("First", 10, SheetKind::Worksheet),
            ("Second", 20, SheetKind::Chartsheet),
        ],
        "document order, not id order, and each kind from its relationship"
    );
}

#[test]
fn sheet_of_an_unknown_relationship_kind() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&relationship(
            "rId1",
            "http://example.invalid/whatever",
            "worksheets/sheet1.xml",
        )),
    );
    let mut package = package(parts);
    assert_eq!(
        package.workbook().expect("workbook").sheets[0].kind,
        SheetKind::Other,
        "an unknown type is recorded, not refused"
    );
}

#[test]
fn workbook_options_take_the_first() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}{}{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            relationship(
                "rId2",
                &format!("{ATTR_NS}/sharedStrings"),
                "sharedStrings.xml"
            ),
            relationship("rId3", &format!("{ATTR_NS}/styles"), "first.xml"),
            relationship("rId4", &format!("{ATTR_NS}/styles"), "second.xml"),
            relationship("rId5", &format!("{ATTR_NS}/calcChain"), "calcChain.xml"),
        )),
    );
    let mut package = package(parts);
    let workbook = package.workbook().expect("workbook");
    assert_eq!(
        workbook.shared_strings.as_deref(),
        Some(b"xl/sharedStrings.xml".as_slice()),
        "the shared strings part"
    );
    assert_eq!(
        workbook.styles.as_deref(),
        Some(b"xl/first.xml".as_slice()),
        "the first styles relationship in document order"
    );
    assert_eq!(
        workbook.calc_chain.as_deref(),
        Some(b"xl/calcChain.xml".as_slice()),
        "the calculation chain"
    );
}

// ----------------------------------------------------------------- errors

#[test]
fn open_reads_no_xml() {
    let mut parts = smallest();
    // A root tag with no closing bracket: not well-formed, and refused before
    // the root's own name can be read. Opening must still succeed, because
    // the layer parses on demand.
    set(
        &mut parts,
        b"[Content_Types].xml",
        format!("{DECLARATION}\n<Types xmlns=\"{TYPES_NS}\""),
    );
    let mut package = package(parts);
    assert_eq!(
        package.archive().entries().len(),
        6,
        "the archive is indexed"
    );
    assert_eq!(
        package.part(b"xl/workbook.xml"),
        Some(2),
        "and its parts are findable"
    );
    assert_xml(package.content_types().expect_err("refused on first call"));
}

#[test]
fn content_types_part_absent() {
    let mut parts = smallest();
    drop_part(&mut parts, b"[Content_Types].xml");
    let mut package = package(parts);
    assert_package(
        package.content_types().expect_err("refused"),
        "no [Content_Types].xml part",
    );
}

#[test]
fn root_wrong_in_types() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"[Content_Types].xml",
        format!("{DECLARATION}\n<Kinds xmlns=\"{TYPES_NS}\"/>"),
    );
    let mut package = package(parts);
    assert_package(
        package.content_types().expect_err("refused"),
        "a content types part whose root is not Types",
    );
}

#[test]
fn root_wrong_namespace_in_types() {
    let mut parts = smallest();
    // The right name in the wrong namespace is the wrong root.
    set(
        &mut parts,
        b"[Content_Types].xml",
        format!("{DECLARATION}\n<Types xmlns=\"http://example.invalid/types\"/>"),
    );
    let mut package = package(parts);
    assert_package(
        package.content_types().expect_err("refused"),
        "a content types part whose root is not Types",
    );
}

#[test]
fn root_wrong_in_rels() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"_rels/.rels",
        format!("{DECLARATION}\n<Relations xmlns=\"{RELS_NS}\"/>"),
    );
    let mut package = package(parts);
    assert_package(
        package.relationships(b"").expect_err("refused"),
        "a relationships part whose root is not Relationships",
    );
}

#[test]
fn root_wrong_in_workbook() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/workbook.xml",
        format!("{DECLARATION}\n<book xmlns=\"{MAIN_NS}\"/>"),
    );
    let mut package = package(parts);
    assert_package(
        package.workbook().expect_err("refused"),
        "a workbook part whose root is not workbook",
    );
}

#[test]
fn no_office_document() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"_rels/.rels",
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/thumbnail"),
            "docProps/thumbnail.jpeg",
        )),
    );
    let mut package = package(parts);
    assert_package(
        package.workbook().expect_err("refused"),
        "no officeDocument relationship",
    );
}

#[test]
fn two_office_documents() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"_rels/.rels",
        rels(&format!(
            "{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/officeDocument"),
                "xl/workbook.xml"
            ),
            relationship("rId2", &format!("{ATTR_NS}/officeDocument"), "xl/other.xml"),
        )),
    );
    let mut package = package(parts);
    assert_package(
        package.workbook().expect_err("refused"),
        "more than one officeDocument relationship",
    );
}

#[test]
fn two_office_documents_one_of_each_form() {
    let mut parts = smallest();
    // The transitional and the strict form together: the package does not
    // say which document is the workbook.
    set(
        &mut parts,
        b"_rels/.rels",
        rels(&format!(
            "{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/officeDocument"),
                "xl/workbook.xml"
            ),
            relationship(
                "rId2",
                &format!("{ATTR_STRICT}/officeDocument"),
                "xl/workbook.xml",
            ),
        )),
    );
    let mut package = package(parts);
    assert_package(
        package.workbook().expect_err("refused"),
        "more than one officeDocument relationship",
    );
}

#[test]
fn office_document_target_absent() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"_rels/.rels",
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/officeDocument"),
            "xl/missing.xml",
        )),
    );
    let mut package = package(parts);
    assert_package(
        package.workbook().expect_err("refused"),
        "the officeDocument relationship names no entry",
    );
}

#[test]
fn sheet_lacks_rel_id() {
    for sheet in [
        "<sheet sheetId=\"1\" r:id=\"rId1\"/>",
        "<sheet name=\"Sheet1\" r:id=\"rId1\"/>",
        "<sheet name=\"Sheet1\" sheetId=\"1\"/>",
    ] {
        let mut parts = smallest();
        set(
            &mut parts,
            b"xl/workbook.xml",
            format!(
                "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
                 <sheets>{sheet}</sheets></workbook>"
            ),
        );
        let mut package = package(parts);
        assert_package(
            package.workbook().expect_err("refused"),
            "a sheet element lacks name, sheetId or r:id",
        );
    }
}

#[test]
fn sheet_id_not_a_number() {
    for id in ["", "1x", "+1", "4294967296"] {
        let mut parts = smallest();
        set(
            &mut parts,
            b"xl/workbook.xml",
            format!(
                "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><sheets>\
                 <sheet name=\"Sheet1\" sheetId=\"{id}\" r:id=\"rId1\"/></sheets></workbook>"
            ),
        );
        let mut package = package(parts);
        assert_package(
            package.workbook().expect_err("refused"),
            "a sheetId that is not a number",
        );
    }
}

#[test]
fn sheet_relationship_missing() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/workbook.xml",
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><sheets>\
             <sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId404\"/></sheets></workbook>"
        ),
    );
    let mut package = package(parts);
    assert_package(
        package.workbook().expect_err("refused"),
        "a sheet's relationship is missing",
    );
}

#[test]
fn two_relationships_one_id() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            relationship("rId1", &format!("{ATTR_NS}/styles"), "styles.xml"),
        )),
    );
    let mut package = package(parts);
    assert_package(
        package
            .relationships(b"xl/workbook.xml")
            .expect_err("refused"),
        "two relationships with the same id",
    );
}

#[test]
fn non_utf8_sheet_name() {
    let mut parts = smallest();
    let mut workbook = format!(
        "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><sheets>\
         <sheet name=\"X\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
    )
    .into_bytes();
    // A byte that cannot start a UTF-8 sequence, in place of the name's `X`.
    let at = workbook
        .windows(9)
        .position(|window| window == b"name=\"X\" ".as_slice())
        .expect("the name is there")
        + 6;
    workbook[at] = 0xFF;
    let part = parts
        .iter_mut()
        .find(|(name, _)| name == b"xl/workbook.xml")
        .expect("the workbook part");
    part.1 = workbook;
    let mut package = package(parts);
    assert_xml(package.workbook().expect_err("refused"));
}

/// P2 ruling 4: one message for a structurally incomplete element, and one
/// test per element kind. A `Default` needs `Extension` and `ContentType`.
#[test]
fn default_lacks_an_attribute() {
    for element in [
        "<Default ContentType=\"application/xml\"/>",
        "<Default Extension=\"xml\"/>",
        "<Default/>",
    ] {
        let mut parts = smallest();
        set(&mut parts, b"[Content_Types].xml", types(element));
        let mut package = package(parts);
        assert_package(
            package.content_types().expect_err("refused"),
            "a Default, Override or Relationship element lacks a required attribute",
        );
    }
}

/// An `Override` needs `PartName` and `ContentType`.
#[test]
fn override_lacks_an_attribute() {
    for element in [
        "<Override ContentType=\"application/xml\"/>",
        "<Override PartName=\"/xl/workbook.xml\"/>",
        "<Override/>",
    ] {
        let mut parts = smallest();
        set(&mut parts, b"[Content_Types].xml", types(element));
        let mut package = package(parts);
        assert_package(
            package.content_types().expect_err("refused"),
            "a Default, Override or Relationship element lacks a required attribute",
        );
    }
}

/// A `Relationship` needs `Id`, `Type` and `Target`.
#[test]
fn relationship_lacks_an_attribute() {
    let kind = format!("{ATTR_NS}/officeDocument");
    for element in [
        format!("<Relationship Type=\"{kind}\" Target=\"xl/workbook.xml\"/>"),
        "<Relationship Id=\"rId1\" Target=\"xl/workbook.xml\"/>".to_string(),
        format!("<Relationship Id=\"rId1\" Type=\"{kind}\"/>"),
        "<Relationship/>".to_string(),
    ] {
        let mut parts = smallest();
        set(&mut parts, b"_rels/.rels", rels(&element));
        let mut package = package(parts);
        assert_package(
            package.relationships(b"").expect_err("refused"),
            "a Default, Override or Relationship element lacks a required attribute",
        );
    }
}
