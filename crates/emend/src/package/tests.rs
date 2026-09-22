//! Byte-built packages for the package layer.
//!
//! `specs/0009-package-layer.md`, "P2: content types, relationships, the
//! workbook": every fixture is a real zip built byte by byte through
//! [`crate::testing`], so nothing that builds a test package shares code with
//! the layer under test and the crate keeps its two dependencies.
//!
//! The smallest package is six parts: `[Content_Types].xml`, `_rels/.rels`,
//! `xl/workbook.xml`, `xl/_rels/workbook.xml.rels`, one worksheet and
//! `xl/styles.xml`, which the workbook's relationships name and
//! `workbook_options_take_the_first` needs, and which is why
//! `open_reads_no_xml` asserts six entries. Each
//! test starts from [`smallest`] and replaces or removes exactly the part it
//! is about, so what a test changes is what it asserts.

use std::collections::BTreeSet;
use std::io::Cursor;

use super::*;
use crate::testing::{
    Build, METHOD_DEFLATE, Spec, TEMPORARY_LOCK, crc32, names, parse_independently,
    replacement_temporaries, stored_data,
};

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

/// The six parts of the smallest package, in the order a producer writes
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
    // Ten of the corpus's twenty-seven files write the workbook this way. The
    // second relationship is the same part reached by an absolute target that
    // also carries a `.` and a `..`, which `target_part` resolves in either
    // form (`specs/0009-package-layer.md`, P2 ruling 5, pinned by a fixture
    // rather than by argument in the P2 verification ruling 2). No corpus
    // target combines the two, so only this fixture covers it.
    set(
        &mut parts,
        b"_rels/.rels",
        rels(&format!(
            "{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/officeDocument"),
                "/xl/workbook.xml"
            ),
            relationship(
                "rId2",
                &format!("{ATTR_NS}/theme"),
                "/xl/./nowhere/../workbook.xml"
            ),
        )),
    );
    let mut package = package(parts);
    assert_eq!(
        package.workbook().expect("workbook").part,
        b"xl/workbook.xml",
        "an absolute target loses its slash"
    );
    let package_rels = package.relationships(b"").expect("package rels");
    let plain = package_rels.by_id(b"rId1").expect("the plain absolute one");
    let dotted = package_rels
        .by_id(b"rId2")
        .expect("the dotted absolute one");
    assert_eq!(
        package_rels.target_part(dotted).expect("it resolves"),
        b"xl/workbook.xml",
        "`.` and `..` resolve in an absolute target too"
    );
    assert_eq!(
        package_rels.target_part(dotted).expect("it resolves"),
        package_rels.target_part(plain).expect("it resolves"),
        "the two forms name the same entry"
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

// ------------------------------------------------------------- the edits
//
// `specs/0009-package-layer.md`, "P3: the edits, the first real edit, the
// size record": each edit alone on the smallest package, the output read by
// the independent parser and the edited part's bytes compared with bytes the
// test builds by hand. Every fixture part is stored, so
// [`crate::testing::stored_data`] can read a part back out of the output
// without asking the layer that wrote it.

/// A fixture's parts: each entry's name and its bytes, in the order a
/// producer writes them. Named because clippy counts the pair of them in a
/// return type as one type too many.
type Parts = Vec<(Vec<u8>, Vec<u8>)>;

/// The parts as bytes and as an open package, so a test can compare the
/// output against the input it was built from.
#[track_caller]
fn opened(parts: Parts) -> (Parts, Package<Cursor<Vec<u8>>>) {
    let specs = parts
        .iter()
        .map(|(name, data)| Spec::new(name, data))
        .collect();
    let bytes = Build::new(specs).bytes();
    let package = Package::open(Cursor::new(bytes)).expect("the fixture opens");
    (parts, package)
}

/// Writes the package back and returns the output bytes.
///
/// Holds [`TEMPORARY_LOCK`] for the write: a rewritten part and a
/// replacement each make a temporary of the one form, in this one process, so
/// a write here that ran beside the zip layer's snapshot tests would fail
/// them on a race rather than on either layer.
#[track_caller]
fn written(package: Package<Cursor<Vec<u8>>>) -> Vec<u8> {
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut out = Vec::new();
    package.write_to(&mut out).expect("write_to");
    out
}

/// Asserts that `output` holds exactly these entry names, in this order.
#[track_caller]
fn assert_names(output: &[u8], expected: &[&[u8]]) {
    let parsed = parse_independently(output);
    let got: Vec<Vec<u8>> = names(&parsed);
    let want: Vec<Vec<u8>> = expected.iter().map(|name| name.to_vec()).collect();
    assert_eq!(got, want, "the entry names");
}

/// The names of `parts`, for [`assert_names`] when an edit changed none.
fn part_names(parts: &[(Vec<u8>, Vec<u8>)]) -> Vec<&[u8]> {
    parts.iter().map(|(name, _)| name.as_slice()).collect()
}

/// Asserts that every part of `parts` except those named comes back byte for
/// byte.
#[track_caller]
fn assert_untouched(output: &[u8], parts: &[(Vec<u8>, Vec<u8>)], except: &[&[u8]]) {
    for (name, data) in parts {
        if except.contains(&name.as_slice()) {
            continue;
        }
        assert_eq!(
            &stored_data(output, name),
            data,
            "the part {:?} changed",
            String::from_utf8_lossy(name)
        );
    }
}

/// Asserts that the part named holds exactly `expected`.
#[track_caller]
fn assert_part(output: &[u8], name: &[u8], expected: &str) {
    assert_eq!(
        String::from_utf8(stored_data(output, name)).expect("the part is UTF-8"),
        expected,
        "the part {:?}",
        String::from_utf8_lossy(name)
    );
}

/// Asserts that an ADDED part holds exactly `expected`.
///
/// An added entry has no original method to keep, so the zip layer deflates
/// it and its bytes cannot be sliced out the way a stored part's can. The
/// independent reader's CRC-32 and uncompressed size, against the expected
/// bytes' own CRC-32 and length, say the same thing without inflating
/// anything: `crc32` here is `crate::testing`'s, computed from the expected
/// bytes, not a number the layer wrote.
#[track_caller]
fn assert_added_part(output: &[u8], name: &[u8], expected: &[u8]) {
    let parsed = parse_independently(output);
    let entry = parsed
        .entries
        .iter()
        .find(|entry| entry.name == name)
        .unwrap_or_else(|| panic!("no entry named {:?}", String::from_utf8_lossy(name)));
    assert_eq!(entry.method, METHOD_DEFLATE, "an added entry is deflated");
    assert_eq!(
        entry.uncompressed_size,
        expected.len() as u64,
        "the size of {:?}",
        String::from_utf8_lossy(name)
    );
    assert_eq!(
        entry.crc32,
        crc32(expected),
        "the CRC-32 of {:?}",
        String::from_utf8_lossy(name)
    );
}

/// The worksheet and styles relationships of the smallest package's workbook,
/// which several tests remove from or add to.
fn worksheet_rel() -> String {
    relationship(
        "rId1",
        &format!("{ATTR_NS}/worksheet"),
        "worksheets/sheet1.xml",
    )
}

fn styles_rel() -> String {
    relationship("rId2", &format!("{ATTR_NS}/styles"), "styles.xml")
}

#[test]
fn removing_a_relationship_drops_its_element_alone() {
    let (parts, mut package) = opened(smallest());
    package.remove_relationship(b"xl/workbook.xml", b"rId2");
    let output = written(package);

    assert_names(&output, &part_names(&parts));
    assert_untouched(&output, &parts, &[b"xl/_rels/workbook.xml.rels"]);
    assert_part(
        &output,
        b"xl/_rels/workbook.xml.rels",
        &rels(&worksheet_rel()),
    );
}

#[test]
fn removing_an_element_keeps_the_whitespace_around_it() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        format!(
            "{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\">\n  {}\n  {}\n</Relationships>",
            worksheet_rel(),
            styles_rel()
        ),
    );
    let (parts, mut package) = opened(parts);
    package.remove_relationship(b"xl/workbook.xml", b"rId2");
    let output = written(package);

    // The element's own bytes go; the newline and two spaces that preceded it
    // and the newline that followed it are other tokens' raw and stay.
    assert_part(
        &output,
        b"xl/_rels/workbook.xml.rels",
        &format!(
            "{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\">\n  {}\n  \n</Relationships>",
            worksheet_rel()
        ),
    );
    assert_untouched(&output, &parts, &[b"xl/_rels/workbook.xml.rels"]);
}

#[test]
fn removing_an_element_written_as_a_pair() {
    // A raw copy keeps whichever form the input has, so a removal has to
    // handle an element written as a start tag with an end tag as well as an
    // empty one: the children and the end tag go with it.
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        format!(
            "{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\">{}<Relationship Id=\"rId2\" \
             Type=\"{ATTR_NS}/styles\" Target=\"styles.xml\"></Relationship></Relationships>",
            worksheet_rel()
        ),
    );
    let (parts, mut package) = opened(parts);
    package.remove_relationship(b"xl/workbook.xml", b"rId2");
    let output = written(package);

    assert_part(
        &output,
        b"xl/_rels/workbook.xml.rels",
        &rels(&worksheet_rel()),
    );
    assert_untouched(&output, &parts, &[b"xl/_rels/workbook.xml.rels"]);
}

#[test]
fn adding_a_relationship_goes_before_the_end_tag() {
    let (parts, mut package) = opened(smallest());
    let id = package
        .add_relationship(
            b"xl/workbook.xml",
            format!("{ATTR_NS}/calcChain").as_bytes(),
            b"calcChain.xml",
            false,
        )
        .expect("the id");
    assert_eq!(id, b"rId3", "one more than the largest");
    let output = written(package);

    assert_names(&output, &part_names(&parts));
    assert_part(
        &output,
        b"xl/_rels/workbook.xml.rels",
        &rels(&format!(
            "{}{}{}",
            worksheet_rel(),
            styles_rel(),
            relationship("rId3", &format!("{ATTR_NS}/calcChain"), "calcChain.xml")
        )),
    );
    assert_untouched(&output, &parts, &[b"xl/_rels/workbook.xml.rels"]);
}

#[test]
fn an_external_relationship_writes_its_target_mode() {
    let (_, mut package) = opened(smallest());
    package
        .add_relationship(
            b"xl/workbook.xml",
            format!("{ATTR_NS}/hyperlink").as_bytes(),
            b"https://example.invalid/page",
            true,
        )
        .expect("the id");
    let output = written(package);

    assert_part(
        &output,
        b"xl/_rels/workbook.xml.rels",
        &rels(&format!(
            "{}{}<Relationship Id=\"rId3\" Type=\"{ATTR_NS}/hyperlink\" \
             Target=\"https://example.invalid/page\" TargetMode=\"External\"/>",
            worksheet_rel(),
            styles_rel()
        )),
    );
}

#[test]
fn removing_an_override_drops_its_element_alone() {
    let (parts, mut package) = opened(smallest());
    package.remove_override(b"/xl/worksheets/sheet1.xml");
    let output = written(package);

    assert_names(&output, &part_names(&parts));
    assert_part(
        &output,
        b"[Content_Types].xml",
        &types(&format!(
            "<Default Extension=\"rels\" \
             ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
             <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>"
        )),
    );
    assert_untouched(&output, &parts, &[b"[Content_Types].xml"]);
}

#[test]
fn adding_an_override_goes_before_the_end_tag() {
    let (parts, mut package) = opened(smallest());
    package.add_override(b"/xl/calcChain.xml", b"application/xml");
    let output = written(package);

    let (name, data) = parts
        .iter()
        .find(|(name, _)| name == b"[Content_Types].xml")
        .expect("the fixture has it");
    let before = String::from_utf8(data.clone()).expect("UTF-8");
    let expected = before.replace(
        "</Types>",
        "<Override PartName=\"/xl/calcChain.xml\" ContentType=\"application/xml\"/></Types>",
    );
    assert_part(&output, name, &expected);
    assert_untouched(&output, &parts, &[b"[Content_Types].xml"]);
}

#[test]
fn removing_a_part_drops_the_entry_alone() {
    let (parts, mut package) = opened(smallest());
    package.remove_part(b"xl/styles.xml");
    let output = written(package);

    assert_names(
        &output,
        &[
            b"[Content_Types].xml",
            b"_rels/.rels",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
            b"xl/worksheets/sheet1.xml",
        ],
    );
    // Nothing else: the relationship naming it and any override for it are
    // the caller's separate calls, and this test makes neither.
    assert_untouched(&output, &parts, &[b"xl/styles.xml"]);
}

#[test]
fn replacing_a_part_changes_only_its_bytes() {
    let (parts, mut package) = opened(smallest());
    package.replace_part(
        b"xl/styles.xml",
        replacement_from_reader(Cursor::new(b"<styleSheet><fonts/></styleSheet>".to_vec())),
    );
    let output = written(package);

    assert_names(&output, &part_names(&parts));
    assert_part(
        &output,
        b"xl/styles.xml",
        "<styleSheet><fonts/></styleSheet>",
    );
    assert_untouched(&output, &parts, &[b"xl/styles.xml"]);
}

#[test]
fn adding_a_part_adds_the_entry_alone() {
    let (parts, mut package) = opened(smallest());
    package.add_part(
        b"xl/calcChain.xml",
        replacement_from_reader(Cursor::new(b"<calcChain/>".to_vec())),
    );
    let output = written(package);

    let mut expected = part_names(&parts);
    expected.push(b"xl/calcChain.xml");
    assert_names(&output, &expected);
    assert_added_part(&output, b"xl/calcChain.xml", b"<calcChain/>");
    assert_untouched(&output, &parts, &[]);
}

#[test]
fn an_addition_escapes_its_values() {
    let (_, mut package) = opened(smallest());
    // The five characters `quick-xml`'s escape covers, in a target and in a
    // content type, so a value can never close its own attribute.
    package
        .add_relationship(
            b"xl/workbook.xml",
            format!("{ATTR_NS}/hyperlink").as_bytes(),
            b"page?a=1&b=2<>\"'",
            true,
        )
        .expect("the id");
    package.add_override(b"/xl/a&b.xml", b"text/plain;x=\"1\"");
    let output = written(package);

    assert_part(
        &output,
        b"xl/_rels/workbook.xml.rels",
        &rels(&format!(
            "{}{}<Relationship Id=\"rId3\" Type=\"{ATTR_NS}/hyperlink\" \
             Target=\"page?a=1&amp;b=2&lt;&gt;&quot;&apos;\" TargetMode=\"External\"/>",
            worksheet_rel(),
            styles_rel()
        )),
    );
    assert_part(
        &output,
        b"[Content_Types].xml",
        &types(&format!(
            "<Default Extension=\"rels\" \
             ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
             <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
             <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>\
             <Override PartName=\"/xl/a&amp;b.xml\" ContentType=\"text/plain;x=&quot;1&quot;\"/>"
        )),
    );
}

#[test]
fn an_empty_root_in_a_rels_part_gains_an_end_tag() {
    // P3 ruling 8: `<Relationships/>` has no end tag to add before, so its
    // own bytes go out with the `/` of its `/>` dropped and an end tag is
    // written after the additions. The prolog and the attributes are kept.
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        format!("{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\"/>"),
    );
    let (parts, mut package) = opened(parts);
    let id = package
        .add_relationship(
            b"xl/workbook.xml",
            format!("{ATTR_NS}/styles").as_bytes(),
            b"styles.xml",
            false,
        )
        .expect("the id");
    assert_eq!(id, b"rId1", "an empty set starts at rId1");
    let output = written(package);

    assert_names(&output, &part_names(&parts));
    assert_part(
        &output,
        b"xl/_rels/workbook.xml.rels",
        &format!(
            "{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\">{}</Relationships>",
            relationship("rId1", &format!("{ATTR_NS}/styles"), "styles.xml")
        ),
    );
}

#[test]
fn an_empty_root_in_content_types_gains_an_end_tag() {
    // The same rule for `[Content_Types].xml`: `<Types/>` parses to empty
    // lists, so an added override reaches the root (P3 ruling 8).
    let mut parts = smallest();
    set(
        &mut parts,
        b"[Content_Types].xml",
        format!("{DECLARATION}\n<Types xmlns=\"{TYPES_NS}\" />"),
    );
    let (parts, mut package) = opened(parts);
    package.add_override(b"/xl/workbook.xml", WORKBOOK_TYPE.as_bytes());
    let output = written(package);

    assert_names(&output, &part_names(&parts));
    // The space before the slash is the input's own byte and stays: only the
    // slash goes.
    assert_part(
        &output,
        b"[Content_Types].xml",
        &format!(
            "{DECLARATION}\n<Types xmlns=\"{TYPES_NS}\" ><Override \
             PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/></Types>"
        ),
    );
}

/// The rels part a created one would be, for the smallest package's
/// worksheet, which has none.
const SHEET_RELS: &[u8] = b"xl/worksheets/_rels/sheet1.xml.rels";

/// The relationship the created-part tests add.
fn created_body() -> String {
    format!(
        "<Relationships xmlns=\"{RELS_NS}\">{}</Relationships>",
        relationship("rId1", &format!("{ATTR_NS}/hyperlink"), "../other.xml")
    )
}

/// Adds that relationship to the worksheet, which has no rels part.
#[track_caller]
fn add_to_the_worksheet(package: &mut Package<Cursor<Vec<u8>>>) -> Vec<u8> {
    let id = package
        .add_relationship(
            b"xl/worksheets/sheet1.xml",
            format!("{ATTR_NS}/hyperlink").as_bytes(),
            b"../other.xml",
            false,
        )
        .expect("the id");
    assert_eq!(id, b"rId1", "a part with no rels part is an empty set");
    id
}

#[test]
fn a_created_rels_part_takes_the_packages_own_form() {
    // P3 ruling 10: the prefix is the package's own `_rels/.rels` up to its
    // root start tag. The fixture's form is the declaration and one LF, which
    // is the form 68 of the corpus's 216 rels parts write.
    let (parts, mut package) = opened(smallest());
    add_to_the_worksheet(&mut package);
    let output = written(package);

    let mut expected = part_names(&parts);
    expected.push(SHEET_RELS);
    assert_names(&output, &expected);
    assert_added_part(
        &output,
        SHEET_RELS,
        format!("{DECLARATION}\n{}", created_body()).as_bytes(),
    );
    assert_untouched(&output, &parts, &[]);
}

#[test]
fn a_created_rels_part_takes_a_mark_and_no_separator_when_the_package_does() {
    // The other corpus form: a byte order mark, a lower-case encoding name
    // and nothing between the declaration and the root.
    let mut parts = smallest();
    let their_form = format!(
        "\u{feff}<?xml version=\"1.0\" encoding=\"utf-8\"?><Relationships xmlns=\"{RELS_NS}\">{}</Relationships>",
        relationship(
            "rId1",
            &format!("{ATTR_NS}/officeDocument"),
            "xl/workbook.xml"
        )
    );
    set(&mut parts, b"_rels/.rels", their_form);
    let (parts, mut package) = opened(parts);
    add_to_the_worksheet(&mut package);
    let output = written(package);

    assert_added_part(
        &output,
        SHEET_RELS,
        format!(
            "\u{feff}<?xml version=\"1.0\" encoding=\"utf-8\"?>{}",
            created_body()
        )
        .as_bytes(),
    );
    assert_untouched(&output, &parts, &[]);
}

#[test]
fn a_created_rels_part_falls_back_when_the_package_has_no_rels_part() {
    // No `_rels/.rels` to copy: the prefix is the corpus's majority form.
    let mut parts = smallest();
    drop_part(&mut parts, b"_rels/.rels");
    let (parts, mut package) = opened(parts);
    add_to_the_worksheet(&mut package);
    let output = written(package);

    assert_added_part(
        &output,
        SHEET_RELS,
        format!(
            "\u{feff}<?xml version=\"1.0\" encoding=\"utf-8\"?>{}",
            created_body()
        )
        .as_bytes(),
    );
    assert_untouched(&output, &parts, &[]);
}

#[test]
fn a_generated_id_is_one_more_than_the_largest() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}",
            relationship(
                "rId7",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            relationship("rId3", &format!("{ATTR_NS}/styles"), "styles.xml")
        )),
    );
    let (_, mut package) = opened(parts);
    let first = package
        .add_relationship(b"xl/workbook.xml", b"kind", b"a.xml", false)
        .expect("the id");
    let second = package
        .add_relationship(b"xl/workbook.xml", b"kind", b"b.xml", false)
        .expect("the id");
    // Not the count, and not the document order: one more than the largest,
    // then one more again, so two calls never collide.
    assert_eq!(first, b"rId8", "one more than the largest");
    assert_eq!(second, b"rId9", "one more than the one just handed out");
}

#[test]
fn an_id_of_another_form_is_ignored_by_the_rule() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}{}",
            relationship(
                "worksheet",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            relationship("rId2x", &format!("{ATTR_NS}/styles"), "styles.xml"),
            relationship("rId", &format!("{ATTR_NS}/theme"), "theme/theme1.xml")
        )),
    );
    let (_, mut package) = opened(parts);
    let id = package
        .add_relationship(b"xl/workbook.xml", b"kind", b"a.xml", false)
        .expect("the id");
    // None of the three is `rId<digits>`, so the largest is nothing and the
    // first generated id is rId1; it cannot collide with any of them.
    assert_eq!(id, b"rId1", "no id of the generated form is there");
}

#[test]
fn a_removed_id_is_not_reused_within_a_write() {
    // P3 ruling 7: the ids counted include those pending removal, so a write
    // that removes rId2 and adds one does not hand out rId2 again.
    let (_, mut package) = opened(smallest());
    package.remove_relationship(b"xl/workbook.xml", b"rId2");
    let id = package
        .add_relationship(b"xl/workbook.xml", b"kind", b"a.xml", false)
        .expect("the id");
    assert_eq!(id, b"rId3", "the removed id is still counted");
}

#[test]
fn several_edits_to_one_part_apply_together() {
    let (parts, mut package) = opened(smallest());
    package.remove_relationship(b"xl/workbook.xml", b"rId1");
    package.remove_relationship(b"xl/workbook.xml", b"rId2");
    package
        .add_relationship(
            b"xl/workbook.xml",
            format!("{ATTR_NS}/worksheet").as_bytes(),
            b"worksheets/sheet2.xml",
            false,
        )
        .expect("the id");
    package
        .add_relationship(
            b"xl/workbook.xml",
            format!("{ATTR_NS}/styles").as_bytes(),
            b"styles2.xml",
            false,
        )
        .expect("the id");
    let output = written(package);

    // Both removals and both additions, in one rewrite of the one part: the
    // additions in the order they were recorded, before the end tag.
    assert_part(
        &output,
        b"xl/_rels/workbook.xml.rels",
        &rels(&format!(
            "{}{}",
            relationship(
                "rId3",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet2.xml"
            ),
            relationship("rId4", &format!("{ATTR_NS}/styles"), "styles2.xml")
        )),
    );
    assert_untouched(&output, &parts, &[b"xl/_rels/workbook.xml.rels"]);
}

// --------------------------------------------------- the refusals, and none
// of them writes anything

/// Writes the package back expecting a refusal, and asserts that nothing at
/// all reached the output: "every edit is validated first, so a refused set
/// writes nothing".
///
/// A write that is not refused panics with what it wrote, so a failure says
/// what the refusal was expected on rather than only that the write
/// succeeded (P1 ruling 8).
#[track_caller]
fn refused(package: Package<Cursor<Vec<u8>>>) -> Error {
    match refusal(package) {
        Ok(error) => error,
        Err(out) => panic!(
            "the write was not refused: it wrote {} byte(s), entries {:?}",
            out.len(),
            names(&parse_independently(&out))
                .iter()
                .map(|name| String::from_utf8_lossy(name).into_owned())
                .collect::<Vec<_>>()
        ),
    }
}

/// The error a write refused on, or its output when it was not refused.
#[track_caller]
fn refusal(package: Package<Cursor<Vec<u8>>>) -> Result<Error, Vec<u8>> {
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut out = Vec::new();
    match package.write_to(&mut out) {
        Ok(()) => Err(out),
        Err(error) => {
            assert!(
                out.is_empty(),
                "a refused set wrote {} byte(s) to the output",
                out.len()
            );
            Ok(error)
        }
    }
}

#[test]
fn removing_an_absent_relationship() {
    let (_, mut package) = opened(smallest());
    package.remove_relationship(b"xl/workbook.xml", b"rId9");
    assert_package(refused(package), "no such relationship");
}

#[test]
fn removing_an_absent_override() {
    let (_, mut package) = opened(smallest());
    package.remove_override(b"/xl/styles.xml");
    assert_package(refused(package), "no such override");
}

#[test]
fn adding_an_override_that_exists() {
    let (_, mut package) = opened(smallest());
    package.add_override(b"/xl/workbook.xml", b"application/xml");
    assert_package(refused(package), "an override for that part already exists");
}

#[test]
fn an_override_removed_and_added_again_is_not_a_duplicate() {
    // Changing a part's content type is a removal and an addition of the same
    // PartName in one write. The refusal above is for a name that KEEPS its
    // override, so this is allowed, and the rewrite drops the old element
    // where it stood and writes the new one before the end tag.
    let (parts, mut package) = opened(smallest());
    package.remove_override(b"/xl/worksheets/sheet1.xml");
    package.add_override(b"/xl/worksheets/sheet1.xml", b"application/xml");
    let output = written(package);

    assert_part(
        &output,
        b"[Content_Types].xml",
        &types(&format!(
            "<Default Extension=\"rels\" \
             ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
             <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
             <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"application/xml\"/>"
        )),
    );
    assert_untouched(&output, &parts, &[b"[Content_Types].xml"]);
}

#[test]
fn editing_a_removed_part() {
    let (_, mut package) = opened(smallest());
    package.remove_part(b"xl/styles.xml");
    package.replace_part(
        b"xl/styles.xml",
        replacement_from_reader(Cursor::new(b"<styleSheet/>".to_vec())),
    );
    assert_package(refused(package), "an edit to a removed part");
}

#[test]
fn editing_a_removed_rels_part() {
    // The rels part a relationship edit would rewrite counts as edited, so
    // removing that part and editing its relationships in one write is
    // refused rather than resolved in some order.
    let (_, mut package) = opened(smallest());
    package.remove_part(b"xl/_rels/workbook.xml.rels");
    package.remove_relationship(b"xl/workbook.xml", b"rId2");
    assert_package(refused(package), "an edit to a removed part");
}

#[test]
fn editing_removed_content_types() {
    let (_, mut package) = opened(smallest());
    package.remove_part(b"[Content_Types].xml");
    package.remove_override(b"/xl/workbook.xml");
    assert_package(refused(package), "an edit to a removed part");
}

#[test]
fn removing_an_absent_part() {
    let (_, mut package) = opened(smallest());
    package.remove_part(b"xl/calcChain.xml");
    assert_package(refused(package), "no such part");
}

#[test]
fn replacing_an_absent_part() {
    let (_, mut package) = opened(smallest());
    package.replace_part(
        b"xl/calcChain.xml",
        replacement_from_reader(Cursor::new(b"<calcChain/>".to_vec())),
    );
    assert_package(refused(package), "no such part");
}

#[test]
fn no_default_for_rels() {
    // A created rels part's content type comes from the package's Default for
    // the rels extension; with no such Default the created part would have no
    // content type at all, so the write is refused.
    let mut parts = smallest();
    set(
        &mut parts,
        b"[Content_Types].xml",
        types(&format!(
            "<Default Extension=\"xml\" ContentType=\"application/xml\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
             <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>"
        )),
    );
    let (_, mut package) = opened(parts);
    add_to_the_worksheet(&mut package);
    assert_package(refused(package), "no Default covers rels");
}

// ---------------------------------------------- the temporaries, and the
// recipe P3's tool applies

#[test]
fn a_failed_replacement_leaves_no_temporary() {
    // The rewrite of the rels part makes a temporary of this layer's own, and
    // the failing replacement makes the write fail after it exists: both
    // temporaries have to be gone when the call returns, whichever way it
    // ended.
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let before = replacement_temporaries();
    let (_, mut package) = opened(smallest());
    package.remove_relationship(b"xl/workbook.xml", b"rId2");
    package.replace_part(
        b"xl/styles.xml",
        Box::new(|_: &mut dyn Write| Err(std::io::Error::other("the replacement failed"))),
    );
    let mut out = Vec::new();
    let error = package.write_to(&mut out).expect_err("the failure");
    assert!(
        matches!(error, Error::Io(_)),
        "the replacement's own error, got {error:?}"
    );
    assert_eq!(
        replacement_temporaries(),
        before,
        "a temporary was left in the temporary directory"
    );
}

/// The smallest package with a calculation chain: the entry, its override and
/// the workbook relationship that names it.
fn with_a_calc_chain() -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut parts = smallest();
    set(
        &mut parts,
        b"[Content_Types].xml",
        types(&format!(
            "<Default Extension=\"rels\" \
             ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
             <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
             <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>\
             <Override PartName=\"/xl/calcChain.xml\" \
             ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.\
             calcChain+xml\"/>"
        )),
    );
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}{}",
            worksheet_rel(),
            styles_rel(),
            relationship("rId3", &format!("{ATTR_NS}/calcChain"), "calcChain.xml")
        )),
    );
    parts.push((
        b"xl/calcChain.xml".to_vec(),
        format!("{DECLARATION}\n<calcChain xmlns=\"{MAIN_NS}\"><c r=\"A1\" i=\"1\"/></calcChain>")
            .into_bytes(),
    ));
    parts
}

#[test]
fn the_recipe_drops_the_chain_and_nothing_else() {
    // The three calls the tool makes, in the order the Design gives them:
    // the relationship, the override, the entry. Nothing is implicit.
    let (parts, mut package) = opened(with_a_calc_chain());
    package.remove_relationship(b"xl/workbook.xml", b"rId3");
    package.remove_override(b"/xl/calcChain.xml");
    package.remove_part(b"xl/calcChain.xml");
    let output = written(package);

    assert_names(
        &output,
        &[
            b"[Content_Types].xml",
            b"_rels/.rels",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
            b"xl/worksheets/sheet1.xml",
            b"xl/styles.xml",
        ],
    );
    assert_part(
        &output,
        b"xl/_rels/workbook.xml.rels",
        &rels(&format!("{}{}", worksheet_rel(), styles_rel())),
    );
    assert_part(
        &output,
        b"[Content_Types].xml",
        &types(&format!(
            "<Default Extension=\"rels\" \
             ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
             <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
             <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>"
        )),
    );
    assert_untouched(
        &output,
        &parts,
        &[
            b"[Content_Types].xml",
            b"xl/_rels/workbook.xml.rels",
            b"xl/calcChain.xml",
        ],
    );
}

#[test]
fn a_package_with_no_chain_comes_back_byte_for_byte() {
    // The recipe finds no calcChain relationship, so it records no edit, and
    // an empty set of edits is the verbatim round trip P1 proved at the zip
    // layer: the same property through `Package::write_to`.
    let parts = smallest();
    let specs = parts
        .iter()
        .map(|(name, data)| Spec::new(name, data))
        .collect();
    let input = Build::new(specs).bytes();
    let mut package = Package::open(Cursor::new(input.clone())).expect("the fixture opens");
    assert!(
        package
            .relationships(b"xl/workbook.xml")
            .expect("rels")
            .first_of(CALC_CHAIN)
            .is_none(),
        "the fixture has no calcChain relationship"
    );
    let output = written(package);
    assert_eq!(output, input, "an empty set of edits changed the bytes");
}

/// The recipe as `tools/emend-copy` recorded it before
/// `specs/0011-editing-cells.md` moved it into the crate: each removal its own
/// call, each only when its subject is there, read in full before any is
/// recorded. Kept here so the method is compared with the code it replaced.
fn the_tools_recipe(package: &mut Package<Cursor<Vec<u8>>>) {
    let workbook = package.workbook().expect("the workbook").part.clone();
    package.content_types().expect("the content types");
    let chain = {
        let rels = package.relationships(&workbook).expect("the rels");
        rels.first_of(CALC_CHAIN)
            .map(|rel| (rel.id.clone(), rels.target_part(rel).expect("a target")))
    };
    if let Some((id, part)) = chain {
        let over = package
            .content_types()
            .expect("the content types")
            .overrides()
            .iter()
            .find(|over| {
                over.part_name
                    .strip_prefix(b"/".as_slice())
                    .unwrap_or(&over.part_name)
                    == part.as_slice()
            })
            .map(|over| over.part_name.clone());
        let entry = package.part(&part).is_some();
        package.remove_relationship(&workbook, &id);
        if let Some(over) = over {
            package.remove_override(&over);
        }
        if entry {
            package.remove_part(&part);
        }
    }
}

/// The three kinds of removal a package has recorded, for comparison.
type Removals = (Vec<(Vec<u8>, Vec<u8>)>, Vec<Vec<u8>>, Vec<Vec<u8>>);

fn removals(package: &Package<Cursor<Vec<u8>>>) -> Removals {
    let edits = &package.edits;
    // The recipe records removals only, so every other kind must be empty.
    assert!(edits.rel_additions.is_empty(), "a relationship was added");
    assert!(edits.override_additions.is_empty(), "an override was added");
    assert!(edits.part_replacements.is_empty(), "a part was replaced");
    assert!(edits.part_additions.is_empty(), "a part was added");
    (
        edits.rel_removals.clone(),
        edits.override_removals.clone(),
        edits.part_removals.clone(),
    )
}

#[test]
fn the_method_records_what_the_tools_recipe_recorded() {
    // specs/0011, P1's first code commit: the tool's recipe and the method
    // record the same edits on a package with a chain and none on one without.
    // Two more shapes the recipe handles: a chain with no Override, and a
    // chain whose entry is missing. Each is checked against a typed
    // expectation too, so the two cannot agree on an empty answer by accident.
    let full = with_a_calc_chain();
    let mut no_override = with_a_calc_chain();
    set(
        &mut no_override,
        b"[Content_Types].xml",
        types(&format!(
            "<Default Extension=\"rels\" \
             ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
             <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
             <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>"
        )),
    );
    let mut no_entry = with_a_calc_chain();
    drop_part(&mut no_entry, b"xl/calcChain.xml");
    let rel = (b"xl/workbook.xml".to_vec(), b"rId3".to_vec());
    let over = b"/xl/calcChain.xml".to_vec();
    let entry = b"xl/calcChain.xml".to_vec();
    let cases: [(&str, Parts, bool, Removals); 4] = [
        (
            "a chain",
            full,
            true,
            (vec![rel.clone()], vec![over.clone()], vec![entry.clone()]),
        ),
        (
            "a chain with no Override",
            no_override,
            true,
            (vec![rel.clone()], vec![], vec![entry]),
        ),
        (
            "a chain with no entry",
            no_entry,
            true,
            (vec![rel], vec![over], vec![]),
        ),
        ("no chain", smallest(), false, (vec![], vec![], vec![])),
    ];
    for (label, parts, had, expected) in cases {
        let (_, mut by_tool) = opened(parts.clone());
        the_tools_recipe(&mut by_tool);
        let (_, mut by_method) = opened(parts);
        let answer = by_method.drop_calc_chain().expect("the recipe applies");
        assert_eq!(answer, had, "{label}: the method's answer");
        assert_eq!(
            removals(&by_method),
            removals(&by_tool),
            "{label}: method against tool"
        );
        assert_eq!(removals(&by_method), expected, "{label}: the removals");
        if !had {
            assert!(by_method.edits.is_empty(), "{label}: nothing recorded");
        }
    }
}

#[test]
fn the_method_records_nothing_when_the_workbook_is_refused() {
    // It reads the workbook first, so it fails as workbook() does, and a
    // failure records nothing.
    let mut parts = with_a_calc_chain();
    drop_part(&mut parts, b"_rels/.rels");
    let (_, mut package) = opened(parts);
    assert_package(
        package
            .drop_calc_chain()
            .expect_err("no officeDocument relationship"),
        "no officeDocument relationship",
    );
    assert!(
        package.edits.is_empty(),
        "a refused recipe recorded an edit"
    );
}

/// Asserts that `error` is [`Error::Xml`] naming exactly that entry.
#[track_caller]
fn assert_xml_index(error: Error, index: usize) {
    match error {
        Error::Xml(at) => assert_eq!(at, index, "the entry the error names"),
        other => panic!("expected an xml error, got {other:?}"),
    }
}

#[test]
fn a_relationship_on_a_part_that_is_not_there() {
    // P3 ruling 13a: a rels part is never created for a part the package does
    // not hold, so the edit is refused rather than inventing xl/_rels/
    // nowhere.xml.rels for a part that does not exist.
    let (_, mut package) = opened(smallest());
    package
        .add_relationship(b"xl/nowhere.xml", b"kind", b"a.xml", false)
        .expect("the id");
    assert_package(refused(package), "no such part");
}

#[test]
fn a_relationship_removal_on_a_part_that_is_not_there() {
    let (_, mut package) = opened(smallest());
    package.remove_relationship(b"xl/nowhere.xml", b"rId1");
    assert_package(refused(package), "no such part");
}

#[test]
fn an_unwritable_value_names_the_source_when_the_rels_part_is_new() {
    // P3 ruling 13b: the rels part does not exist yet, so it has no index to
    // name; the error names the SOURCE part rather than entry 0, which is
    // some unrelated entry. xl/styles.xml is the fixture's sixth entry and
    // has no rels part.
    let (_, mut package) = opened(smallest());
    let index = package.part(b"xl/styles.xml").expect("the fixture has it");
    assert_eq!(index, 5, "the fixture's own order");
    package
        .add_relationship(b"xl/styles.xml", b"kind", b"\xff", false)
        .expect("the id");
    assert_xml_index(refused(package), index);
}

#[test]
fn an_unwritable_value_names_content_types_for_the_packages_own_rels() {
    // The one case left: the source is the package itself, the empty name,
    // which has no entry of its own, and its rels part is absent too. The
    // error names [Content_Types].xml, the one part every package has. The
    // fixture puts that part SECOND, so an index of 0 would be the fallback
    // rather than the answer, and the two cannot be confused.
    let mut parts = smallest();
    drop_part(&mut parts, b"_rels/.rels");
    let types = parts.remove(0);
    parts.insert(1, types);
    let (_, mut package) = opened(parts);
    let index = package
        .part(b"[Content_Types].xml")
        .expect("the fixture has it");
    assert_eq!(index, 1, "the content types part is not entry 0 here");
    package
        .add_relationship(b"", b"kind", b"\xff", false)
        .expect("the id");
    assert_xml_index(refused(package), index);
}

/// The message text of a Rust string literal starting at `rest`, and the
/// bytes consumed, or `None` if `rest` does not start a literal.
///
/// Used by both extractors below so the catalogue and the source are read by
/// the same rule. `\[` and `\]` are unescaped, which is how the catalogue
/// writes `[Content_Types].xml` inside a doc comment; no message holds any
/// other escape, and one that did would come back with its backslash and
/// fail the comparison rather than be silently mangled.
fn string_literal(rest: &str) -> Option<String> {
    let body = rest.strip_prefix('"')?;
    let mut text = String::new();
    let mut chars = body.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(text),
            '\\' => match chars.next()? {
                '[' => text.push('['),
                ']' => text.push(']'),
                other => {
                    text.push('\\');
                    text.push(other);
                }
            },
            other => text.push(other),
        }
    }
    None
}

/// Every message the module header catalogues, from a source file's own text.
///
/// A catalogue entry is a doc-comment list item whose first character is a
/// quote: `//! - "the message" (the tests that pin it)`.
fn catalogued(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("//!"))
        .map(str::trim)
        .filter_map(|line| string_literal(line.strip_prefix("- ")?))
        .collect()
}

/// Every message raised as `Error::Package("...")`, with the count of
/// `Error::Package(` occurrences beside it so a non-literal argument shows up
/// as a difference rather than as a silent omission.
///
/// This reads the file's raw text, so a commented-out or otherwise inactive
/// `Error::Package("...")` counts too. That is deliberate: the failure is
/// loud and the fix is to delete the dead line or catalogue it, where a
/// parser clever enough to skip comments could be wrong about which code is
/// reachable and say nothing.
fn raised(source: &str) -> (BTreeSet<String>, usize, usize) {
    const CALL: &str = "Error::Package(";
    let mut set = BTreeSet::new();
    let mut calls = 0;
    let mut literals = 0;
    let mut at = 0;
    while let Some(found) = source[at..].find(CALL) {
        let after = at + found + CALL.len();
        calls += 1;
        at = after;
        if let Some(message) = string_literal(source[after..].trim_start()) {
            literals += 1;
            set.insert(message);
        }
    }
    (set, calls, literals)
}

#[test]
fn the_catalogue_and_the_source_hold_the_same_messages() {
    // `specs/0010-reading-cells.md`, "Errors", carrying part 2 P3
    // verification ruling 2: the catalogue is checked as a SET against the
    // messages the source raises, never as a count in prose. The count this
    // replaces ("catalogued with the fourteen other package messages", in
    // `xml.rs`) was true when it was written and was falsified by a later
    // commit of the same phase, which no check could see. A set comparison
    // cannot go stale: adding a message without cataloguing it fails here.
    const PACKAGE: &str = include_str!("../package.rs");
    const XML: &str = include_str!("../xml.rs");
    // Part 3 P1 adds the third module, so the catalogue cannot fall behind
    // it either (`specs/0010-reading-cells.md`, "Errors").
    const CELLS: &str = include_str!("../cells.rs");

    let (package_raised, package_calls, package_literals) = raised(PACKAGE);
    let (xml_raised, xml_calls, xml_literals) = raised(XML);
    let (cells_raised, cells_calls, cells_literals) = raised(CELLS);
    // Every `Error::Package(` in any of the three takes a literal. If one
    // ever takes a variable the extractor cannot read it, and this is the
    // line that says so instead of the comparison quietly passing on a
    // subset.
    assert_eq!(
        (package_calls, xml_calls, cells_calls),
        (package_literals, xml_literals, cells_literals),
        "an Error::Package is raised with something other than a literal"
    );

    // The catalogue lives in `package.rs`; `xml.rs` and `cells.rs` say so
    // rather than keeping a list of their own.
    let catalogue = catalogued(PACKAGE);
    let mut raised_all = package_raised;
    raised_all.extend(xml_raised);
    raised_all.extend(cells_raised);

    assert!(
        !catalogue.is_empty() && !raised_all.is_empty(),
        "an extractor found nothing, so the comparison below would be vacuous"
    );
    let missing = raised_all.difference(&catalogue).collect::<Vec<_>>();
    let stale = catalogue.difference(&raised_all).collect::<Vec<_>>();
    assert!(missing.is_empty(), "raised but not catalogued: {missing:?}");
    assert!(stale.is_empty(), "catalogued but never raised: {stale:?}");
}

#[test]
fn the_catalogue_extractors_can_fail() {
    // The control for the test above. Each extractor is shown finding a
    // planted message and the comparison is shown failing on it, so a zero
    // difference there means the sets were read and matched rather than that
    // nothing was read. The fixtures are text, not this crate's source.
    let catalogue_text = "//! - \"a planted message\" (`some_test`).\n//! not a list item\n";
    assert_eq!(
        catalogued(catalogue_text),
        BTreeSet::from(["a planted message".to_string()]),
        "the catalogue extractor missed a list item"
    );
    // A doc line that is not a list item contributes nothing.
    assert!(catalogued("//! \"not a list item\"\n").is_empty());

    let source_text = "return Err(Error::Package(\"a planted message\"));\n";
    let (set, calls, literals) = raised(source_text);
    assert_eq!(
        (set.len(), calls, literals),
        (1, 1, 1),
        "the source extractor missed a raised message"
    );
    // A non-literal argument counts as a call and not as a literal, which is
    // what the assertion in the test above rests on.
    let variable = "Err(Error::Package(message))";
    let (empty, one_call, no_literal) = raised(variable);
    assert_eq!((empty.len(), one_call, no_literal), (0, 1, 0));

    // The escaped brackets the catalogue writes come back unescaped, so the
    // one message that carries them compares equal to the source's form.
    assert_eq!(
        string_literal("\"no \\[Content_Types\\].xml part\"").as_deref(),
        Some("no [Content_Types].xml part")
    );
}

#[test]
fn removing_an_override_keeps_both_default_spellings() {
    // `specs/0010-reading-cells.md`, "P1", carrying part 2 P3 verification
    // ruling 4: the two `Default` forms the raw copy has to keep are the two
    // SPELLINGS of an empty element, with and without a space before the
    // slash. A producer writes one or the other and the copy keeps whichever
    // the input has, so a rewrite that normalised the spelling would change
    // bytes no edit asked it to change. The removal here is of an `Override`,
    // and every `Default` must come back exactly as written.
    // The `rels` Default carries the space, the `xml` one does not. Both
    // orders are run, so neither spelling is only ever seen in one position.
    let spaced = "<Default Extension=\"rels\" \
                  ContentType=\"application/vnd.openxmlformats-package.relationships+xml\" />";
    let tight = "<Default Extension=\"xml\" ContentType=\"application/xml\"/>";
    let keep = format!("<Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>");
    let drop = format!(
        "<Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>"
    );

    // The point of the fixture is the space, so the fixture is checked for
    // it: without this, editing the constant would leave a test that passes
    // while covering only one spelling.
    assert!(
        spaced.contains(" />") && !tight.contains(" />"),
        "the fixture no longer carries both Default spellings"
    );

    for defaults in [format!("{spaced}{tight}"), format!("{tight}{spaced}")] {
        let mut parts = smallest();
        set(
            &mut parts,
            b"[Content_Types].xml",
            types(&format!("{defaults}{keep}{drop}")),
        );
        let (parts, mut package) = opened(parts);
        package.remove_override(b"/xl/worksheets/sheet1.xml");
        let output = written(package);

        // The one `Override` is gone and both `Default` spellings are byte
        // for byte what went in, the space before the slash included.
        assert_part(
            &output,
            b"[Content_Types].xml",
            &types(&format!("{defaults}{keep}")),
        );
        assert_untouched(&output, &parts, &[b"[Content_Types].xml"]);
    }
}

// ------------------------------------------------------------ cell edits
//
// `specs/0011-editing-cells.md`, "P1": byte-built worksheets in byte-built
// packages, the edited part read back through the independent zip reader and
// compared with bytes each test writes out by hand.

const SHEET: &[u8] = b"xl/worksheets/sheet1.xml";

/// A worksheet part with `body` between its root tags.
fn worksheet(body: &str) -> String {
    format!("{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\">{body}</worksheet>")
}

/// The smallest package with its worksheet's `<sheetData>` holding `rows`.
fn with_rows(rows: &str) -> Parts {
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        worksheet(&format!("<sheetData>{rows}</sheetData>")),
    );
    parts
}

/// A reference, from its A1 spelling.
#[track_caller]
fn at(a1: &str) -> CellRef {
    CellRef::parse(a1.as_bytes()).expect("an A1 reference")
}

/// Opens `parts`, lets `edit` record, writes, and returns the output.
#[track_caller]
fn edited_output(parts: Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Vec<u8> {
    let (_, mut package) = opened(parts);
    edit(&mut package);
    written(package)
}

/// The worksheet part after `edit`, as text; every other part is asserted
/// untouched.
#[track_caller]
fn edited_sheet(rows: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let parts = with_rows(rows);
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[SHEET]);
    String::from_utf8(stored_data(&output, SHEET)).expect("the part is UTF-8")
}

/// The worksheet part `rows` would be, for comparison with an output.
fn sheet_with(rows: &str) -> String {
    worksheet(&format!("<sheetData>{rows}</sheetData>"))
}

/// The error a write refuses `rows` with after `edit`.
#[track_caller]
fn refused_edit(rows: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let (_, mut package) = opened(with_rows(rows));
    edit(&mut package);
    match refusal(package) {
        Ok(error) => error,
        Err(out) => panic!(
            "the edit to the rows {rows} was not refused: it wrote the worksheet {}",
            String::from_utf8_lossy(&stored_data(&out, SHEET))
        ),
    }
}

#[test]
fn a_number_cell_replaced_keeps_its_style_and_loses_its_type() {
    let got = edited_sheet(
        "<row r=\"1\"><c r=\"A1\" s=\"3\" t=\"n\"><v>1</v></c></row>",
        |p| p.set_cell(SHEET, at("A1"), Value::Number(2.5)),
    );
    assert_eq!(
        got,
        sheet_with("<row r=\"1\"><c r=\"A1\" s=\"3\"><v>2.5</v></c></row>")
    );
}

#[test]
fn a_text_is_written_inline_and_escaped() {
    let text = " a&b<c>d\"e'f\tg\nh\ri ";
    let got = edited_sheet("<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>", |p| {
        p.set_cell(SHEET, at("A1"), Value::Text(text.to_string()))
    });
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\" t=\"inlineStr\"><is><t xml:space=\"preserve\">\
             \x20a&amp;b&lt;c&gt;d&quot;e&apos;f\tg\nh&#13;i </t></is></c></row>"
        )
    );
}

#[test]
fn a_boolean_is_written_as_one_or_zero() {
    for (value, digit) in [(true, "1"), (false, "0")] {
        let got = edited_sheet("<row r=\"1\"><c r=\"A1\" t=\"s\"><v>0</v></c></row>", |p| {
            p.set_cell(SHEET, at("A1"), Value::Boolean(value))
        });
        assert_eq!(
            got,
            sheet_with(&format!(
                "<row r=\"1\"><c r=\"A1\" t=\"b\"><v>{digit}</v></c></row>"
            ))
        );
    }
}

#[test]
fn a_numbers_text_is_display_between_the_bounds_and_exponent_outside() {
    let cases: [(f64, &str); 8] = [
        (2.5, "2.5"),
        (1.0, "1"),
        (-0.0, "-0"),
        (0.1 + 0.2, "0.30000000000000004"),
        (1e21, "1E21"),
        (1.5e-8, "1.5E-8"),
        (1e-7, "0.0000001"),
        (f64::MAX, "1.7976931348623157E308"),
    ];
    for (number, text) in cases {
        let got = edited_sheet("<row r=\"1\"></row>", |p| {
            p.set_cell(SHEET, at("A1"), Value::Number(number))
        });
        assert_eq!(
            got,
            sheet_with(&format!("<row r=\"1\"><c r=\"A1\"><v>{text}</v></c></row>")),
            "{number:?}"
        );
    }
}

#[test]
fn a_number_that_is_not_finite_is_refused() {
    for number in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let error = refused_edit("<row r=\"1\"></row>", |p| {
            p.set_cell(SHEET, at("A1"), Value::Number(number))
        });
        assert_package(error, "a number that is not finite cannot be written");
    }
}

#[test]
fn a_text_with_a_character_xml_forbids_is_refused() {
    for bad in [
        "\u{0}", "a\u{8}", "\u{B}", "\u{C}", "\u{1F}", "\u{FFFE}", "\u{FFFF}",
    ] {
        let error = refused_edit("<row r=\"1\"></row>", |p| {
            p.set_cell(SHEET, at("A1"), Value::Text(bad.to_string()))
        });
        assert_package(error, "a text holds a character XML cannot carry");
    }
}

#[test]
fn a_new_cell_goes_where_its_column_puts_it() {
    let row = "<row r=\"2\"><c r=\"B2\"><v>1</v></c><c r=\"D2\"><v>2</v></c></row>";
    let one = |col: &str| format!("<c r=\"{col}\"><v>9</v></c>");
    for (col, want) in [
        (
            "A2",
            format!(
                "<row r=\"2\">{}<c r=\"B2\"><v>1</v></c><c r=\"D2\"><v>2</v></c></row>",
                one("A2")
            ),
        ),
        (
            "C2",
            format!(
                "<row r=\"2\"><c r=\"B2\"><v>1</v></c>{}<c r=\"D2\"><v>2</v></c></row>",
                one("C2")
            ),
        ),
        (
            "E2",
            format!(
                "<row r=\"2\"><c r=\"B2\"><v>1</v></c><c r=\"D2\"><v>2</v></c>{}</row>",
                one("E2")
            ),
        ),
    ] {
        let got = edited_sheet(row, |p| p.set_cell(SHEET, at(col), Value::Number(9.0)));
        assert_eq!(got, sheet_with(&want), "{col}");
    }
}

#[test]
fn a_new_row_goes_where_its_number_puts_it() {
    let rows = "<row r=\"2\" spans=\"1:1\"><c r=\"A2\"><v>1</v></c></row>\
                <row r=\"4\"><c r=\"A4\"><v>2</v></c></row>";
    let new = |n: u32| format!("<row r=\"{n}\"><c r=\"B{n}\"><v>9</v></c></row>");
    let two = "<row r=\"2\" spans=\"1:1\"><c r=\"A2\"><v>1</v></c></row>";
    let four = "<row r=\"4\"><c r=\"A4\"><v>2</v></c></row>";
    for (n, want) in [
        (1, format!("{}{two}{four}", new(1))),
        (3, format!("{two}{}{four}", new(3))),
        (5, format!("{two}{four}{}", new(5))),
    ] {
        let got = edited_sheet(rows, |p| {
            p.set_cell(SHEET, CellRef { row: n, col: 2 }, Value::Number(9.0))
        });
        assert_eq!(got, sheet_with(&want), "row {n}");
    }
}

#[test]
fn an_empty_sheet_data_is_opened_up_for_new_rows() {
    // smallest() writes `<sheetData/>`.
    let parts = smallest();
    let output = edited_output(parts.clone(), |p| {
        p.set_cell(SHEET, at("B3"), Value::Boolean(true));
        p.set_cell(SHEET, at("A1"), Value::Number(1.0));
    });
    assert_untouched(&output, &parts, &[SHEET]);
    assert_part(
        &output,
        SHEET,
        &sheet_with(
            "<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>\
             <row r=\"3\"><c r=\"B3\" t=\"b\"><v>1</v></c></row>",
        ),
    );
}

#[test]
fn an_empty_row_is_opened_up_for_a_new_cell() {
    let got = edited_sheet("<row r=\"1\" ht=\"20\"/>", |p| {
        p.set_cell(SHEET, at("C1"), Value::Number(3.0))
    });
    assert_eq!(
        got,
        sheet_with("<row r=\"1\" ht=\"20\"><c r=\"C1\"><v>3</v></c></row>")
    );
}

#[test]
fn several_edits_in_one_row_and_in_several_rows_go_in_one_write() {
    let rows = "\n  <row r=\"1\"><c r=\"B1\"><v>1</v></c></row>\n  <row r=\"3\"><c r=\"A3\"><v>2</v></c></row>\n";
    let got = edited_sheet(rows, |p| {
        p.set_cell(SHEET, at("C3"), Value::Number(5.0));
        p.set_cell(SHEET, at("A1"), Value::Number(3.0));
        p.set_cell(SHEET, at("C1"), Value::Number(4.0));
        p.set_cell(SHEET, at("A2"), Value::Boolean(false));
        p.clear_cell(SHEET, at("A3"));
        p.set_cell(SHEET, at("A4"), Value::Number(6.0));
    });
    // A new row goes immediately before the start tag of the row after it,
    // or before `</sheetData>`: after whatever whitespace the input wrote
    // there, which is another token's and stays where it was.
    assert_eq!(
        got,
        sheet_with(
            "\n  <row r=\"1\"><c r=\"A1\"><v>3</v></c><c r=\"B1\"><v>1</v></c>\
             <c r=\"C1\"><v>4</v></c></row>\n  <row r=\"2\"><c r=\"A2\" t=\"b\"><v>0</v></c></row>\
             <row r=\"3\"><c r=\"C3\"><v>5</v></c></row>\n<row r=\"4\"><c r=\"A4\"><v>6</v></c></row>"
        )
    );
}

#[test]
fn the_last_edit_recorded_for_a_cell_wins() {
    let row = "<row r=\"1\"><c r=\"A1\" s=\"2\"><v>1</v></c></row>";
    let got = edited_sheet(row, |p| {
        p.set_cell(SHEET, at("A1"), Value::Number(2.0));
        p.set_cell(SHEET, at("A1"), Value::Number(3.0));
    });
    assert_eq!(
        got,
        sheet_with("<row r=\"1\"><c r=\"A1\" s=\"2\"><v>3</v></c></row>")
    );
    let got = edited_sheet(row, |p| {
        p.set_cell(SHEET, at("A1"), Value::Number(2.0));
        p.clear_cell(SHEET, at("A1"));
    });
    assert_eq!(got, sheet_with("<row r=\"1\"><c r=\"A1\" s=\"2\"/></row>"));
}

#[test]
fn clearing_a_cell_that_is_not_there_changes_nothing() {
    let rows = "<row r=\"1\"><c r=\"A1\"><v>1</v></c></row><row r=\"3\"/>";
    let got = edited_sheet(rows, |p| {
        p.clear_cell(SHEET, at("B1"));
        p.clear_cell(SHEET, at("A2"));
        p.clear_cell(SHEET, at("A3"));
        p.clear_cell(SHEET, at("A9"));
    });
    assert_eq!(got, sheet_with(rows));
}

#[test]
fn a_cleared_cell_keeps_its_style_or_goes_with_its_own_bytes() {
    let rows = "<row r=\"1\">\n  <c r=\"A1\" s=\"4\" t=\"s\"><f>1+1</f><v>2</v></c>\n  \
                <c r=\"B1\"><v>1</v></c>\n  <c r=\"C1\"/>\n</row>";
    let got = edited_sheet(rows, |p| {
        p.clear_cell(SHEET, at("A1"));
        p.clear_cell(SHEET, at("B1"));
        p.clear_cell(SHEET, at("C1"));
    });
    // A row left with no cells but its style-kept one stays, attributes and
    // the whitespace around the dropped cells included.
    assert_eq!(
        got,
        sheet_with("<row r=\"1\">\n  <c r=\"A1\" s=\"4\"/>\n  \n  \n</row>")
    );
}

#[test]
fn a_cell_with_an_attribute_this_layer_does_not_model_is_refused() {
    for tag in [
        "<c r=\"A1\" cm=\"1\"><v>1</v></c>",
        "<c r=\"A1\" vm=\"1\"/>",
    ] {
        let error = refused_edit(&format!("<row r=\"1\">{tag}</row>"), |p| {
            p.set_cell(SHEET, at("A1"), Value::Number(1.0))
        });
        assert_package(
            error,
            "a cell carries an attribute this layer does not model",
        );
    }
    // A cell carrying one that is NOT edited is copied as it is.
    let rows = "<row r=\"1\"><c r=\"A1\" cm=\"1\"><v>1</v></c></row>";
    let got = edited_sheet(rows, |p| p.set_cell(SHEET, at("B1"), Value::Number(2.0)));
    assert_eq!(
        got,
        sheet_with("<row r=\"1\"><c r=\"A1\" cm=\"1\"><v>1</v></c><c r=\"B1\"><v>2</v></c></row>")
    );
}

/// A row holding a shared master at A1, its follower at B1, and an array
/// master at A2 whose `ref` is A2:B3.
const FORMULAS: &str = "<row r=\"1\"><c r=\"A1\"><f t=\"shared\" ref=\"A1:B1\" si=\"0\">C1</f>\
                        <v>1</v></c><c r=\"B1\"><f t=\"shared\" si=\"0\"/><v>1</v></c></row>\
                        <row r=\"2\"><c r=\"A2\"><f t=\"array\" ref=\"A2:B3\">X1:X2</f><v>1</v></c>\
                        <c r=\"B2\"><v>1</v></c></row>";

#[test]
fn a_cell_in_a_shared_or_array_formula_is_refused() {
    // A1 a shared master, B1 a shared follower, A2 an array master, B2 a
    // cell inside the array's ref, B3 a NEW cell inside it.
    for cell in ["A1", "B1", "A2", "B2", "B3"] {
        let error = refused_edit(FORMULAS, |p| {
            p.set_cell(SHEET, at(cell), Value::Number(1.0))
        });
        assert_package(
            error,
            "a cell in a shared, array or data table formula cannot be edited",
        );
    }
    // Outside the array's ref, the same sheet is edited.
    let got = edited_sheet(FORMULAS, |p| {
        p.set_cell(SHEET, at("C2"), Value::Number(7.0))
    });
    assert_eq!(
        got,
        sheet_with(&FORMULAS.replace(
            "<c r=\"B2\"><v>1</v></c></row>",
            "<c r=\"B2\"><v>1</v></c><c r=\"C2\"><v>7</v></c></row>"
        ))
    );
}

#[test]
fn an_array_whose_extent_is_unknown_is_refused() {
    let rows = "<row r=\"1\"><c r=\"A1\"><f t=\"array\" ref=\"A1:B\">1</f><v>1</v></c></row>";
    let error = refused_edit(rows, |p| p.set_cell(SHEET, at("D9"), Value::Number(1.0)));
    assert_package(
        error,
        "a cell in a shared, array or data table formula cannot be edited",
    );
}

#[test]
fn a_data_table_is_refused_as_an_array_is() {
    // A1 a data table master whose ref is B1:C2; D1 outside it.
    let rows = "<row r=\"1\"><c r=\"A1\"><f t=\"dataTable\" ref=\"B1:C2\" dt2D=\"0\" dtr=\"0\" r1=\"Z1\"/>\
                <v>1</v></c><c r=\"B1\"><v>2</v></c></row>";
    for cell in ["A1", "B1", "C2"] {
        let error = refused_edit(rows, |p| p.set_cell(SHEET, at(cell), Value::Number(1.0)));
        assert_package(
            error,
            "a cell in a shared, array or data table formula cannot be edited",
        );
    }
    let error = refused_edit(rows, |p| p.clear_cell(SHEET, at("A1")));
    assert_package(
        error,
        "a cell in a shared, array or data table formula cannot be edited",
    );
    let got = edited_sheet(rows, |p| p.set_cell(SHEET, at("D1"), Value::Number(7.0)));
    assert_eq!(
        got,
        sheet_with(&rows.replace(
            "<c r=\"B1\"><v>2</v></c></row>",
            "<c r=\"B1\"><v>2</v></c><c r=\"D1\"><v>7</v></c></row>"
        ))
    );
    // A data table whose ref does not parse has an unknown extent.
    let rows = "<row r=\"1\"><c r=\"A1\"><f t=\"dataTable\" ref=\"B1:\"/><v>1</v></c></row>";
    let error = refused_edit(rows, |p| p.set_cell(SHEET, at("D9"), Value::Number(1.0)));
    assert_package(
        error,
        "a cell in a shared, array or data table formula cannot be edited",
    );
}

#[test]
fn a_set_on_a_worksheet_without_sheet_data_is_refused() {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet("<dimension ref=\"A1\"/>"));
    let (_, mut package) = opened(parts.clone());
    package.set_cell(SHEET, at("A1"), Value::Number(1.0));
    let error = match refusal(package) {
        Ok(error) => error,
        Err(out) => panic!(
            "a set on a worksheet without sheetData was not refused: it wrote the worksheet {}",
            String::from_utf8_lossy(&stored_data(&out, SHEET))
        ),
    };
    assert_package(error, "a worksheet without sheetData cannot be edited");
    // A clear alone has nothing to remove there, and the part is unchanged.
    let output = edited_output(parts.clone(), |p| p.clear_cell(SHEET, at("A1")));
    assert_untouched(&output, &parts, &[]);
}

/// The chain package with sheet1 holding a normal formula at A1 and a value
/// at B1.
fn chain_with_formula() -> Parts {
    let mut parts = with_a_calc_chain();
    set(
        &mut parts,
        SHEET,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\" s=\"1\"><f>B1*2</f><v>4</v></c><c r=\"B1\"><v>2</v></c></row>",
        ),
    );
    parts
}

/// The content types and workbook rels the chain package has with its chain
/// gone, as `the_recipe_drops_the_chain_and_nothing_else` has them.
#[track_caller]
fn assert_chain_gone(output: &[u8]) {
    assert_names(
        output,
        &[
            b"[Content_Types].xml",
            b"_rels/.rels",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
            b"xl/worksheets/sheet1.xml",
            b"xl/styles.xml",
        ],
    );
    assert_part(
        output,
        b"xl/_rels/workbook.xml.rels",
        &rels(&format!("{}{}", worksheet_rel(), styles_rel())),
    );
    assert_part(
        output,
        b"[Content_Types].xml",
        &types(&format!(
            "<Default Extension=\"rels\" \
             ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
             <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
             <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>"
        )),
    );
}

#[test]
fn replacing_a_normal_formula_drops_the_chain_in_the_same_write() {
    let parts = chain_with_formula();
    for clear in [false, true] {
        let output = edited_output(parts.clone(), |p| {
            if clear {
                p.clear_cell(SHEET, at("A1"));
            } else {
                p.set_cell(SHEET, at("A1"), Value::Number(5.0));
            }
        });
        assert_chain_gone(&output);
        let cell = if clear {
            "<c r=\"A1\" s=\"1\"/>"
        } else {
            "<c r=\"A1\" s=\"1\"><v>5</v></c>"
        };
        assert_part(
            &output,
            SHEET,
            &sheet_with(&format!(
                "<row r=\"1\">{cell}<c r=\"B1\"><v>2</v></c></row>"
            )),
        );
        assert_untouched(
            &output,
            &parts,
            &[
                b"[Content_Types].xml",
                b"xl/_rels/workbook.xml.rels",
                b"xl/calcChain.xml",
                SHEET,
            ],
        );
        // The output reads as a package: the survey's own questions.
        let mut package = Package::open(Cursor::new(output)).expect("the output opens");
        assert!(
            package
                .workbook()
                .expect("the workbook")
                .calc_chain
                .is_none()
        );
        assert_eq!(package.content_types().expect("types").overrides().len(), 2);
    }
}

#[test]
fn replacing_a_formula_without_a_chain_changes_the_sheet_alone() {
    let mut parts = with_rows("<row r=\"1\"><c r=\"A1\"><f>1+1</f><v>2</v></c></row>");
    let output = edited_output(parts.clone(), |p| {
        p.set_cell(SHEET, at("A1"), Value::Number(3.0))
    });
    assert_untouched(&output, &parts, &[SHEET]);
    set(
        &mut parts,
        SHEET,
        sheet_with("<row r=\"1\"><c r=\"A1\"><v>3</v></c></row>"),
    );
    assert_part(
        &output,
        SHEET,
        &String::from_utf8(parts[4].1.clone()).unwrap(),
    );
}

/// The removals `package` has recorded: relationships, overrides, parts.
fn removals_recorded(package: &Package<Cursor<Vec<u8>>>) -> (usize, usize, usize) {
    (
        package.edits.rel_removals.len(),
        package.edits.override_removals.len(),
        package.edits.part_removals.len(),
    )
}

#[test]
fn a_chain_the_caller_already_dropped_is_dropped_once() {
    // P1 ruling 4: the three removals are counted as recorded, after the
    // chain rule `write_to` applies for a removed formula has run, and the
    // output is then asserted as before.
    let (_, mut package) = opened(chain_with_formula());
    assert!(package.drop_calc_chain().expect("the recipe"));
    package.set_cell(SHEET, at("A1"), Value::Number(5.0));
    assert_eq!(
        removals_recorded(&package),
        (1, 1, 1),
        "after the caller's drop"
    );
    package.drop_calc_chain_once().expect("the chain rule");
    assert_eq!(
        removals_recorded(&package),
        (1, 1, 1),
        "after the chain rule"
    );
    assert_chain_gone(&written(package));
}

#[test]
fn a_chain_the_caller_did_not_drop_is_dropped_once_by_the_rule() {
    // The control for the count above: with no drop by the caller the rule
    // records the three removals itself, once each.
    let (_, mut package) = opened(chain_with_formula());
    package.set_cell(SHEET, at("A1"), Value::Number(5.0));
    assert_eq!(
        removals_recorded(&package),
        (0, 0, 0),
        "before the chain rule"
    );
    package.drop_calc_chain_once().expect("the chain rule");
    assert_eq!(
        removals_recorded(&package),
        (1, 1, 1),
        "after the chain rule"
    );
    assert_chain_gone(&written(package));
}

#[test]
fn a_cell_reference_not_a1_refuses_the_edit() {
    // P1 ruling 1: the `cells` message, raised on the write path by a cell
    // the edit does not touch, since every cell's `r` places the edit.
    let error = refused_edit("<row r=\"1\"><c r=\"1A\"><v>1</v></c></row>", |p| {
        p.set_cell(SHEET, at("C9"), Value::Number(1.0))
    });
    assert_package(error, "a cell reference is not of the form A1");
}

#[test]
fn a_row_number_not_a_number_refuses_the_edit() {
    for rows in ["<row r=\"x\"/>", "<row r=\"0\"/>"] {
        let error = refused_edit(rows, |p| p.set_cell(SHEET, at("C9"), Value::Number(1.0)));
        assert_package(error, "a row number is not a number");
    }
}

#[test]
fn an_unknown_formula_type_refuses_an_edit_to_its_cell() {
    let rows = "<row r=\"1\"><c r=\"A1\"><f t=\"bogus\">1</f><v>1</v></c></row>";
    for edit in [
        (|p: &mut Package<Cursor<Vec<u8>>>| p.set_cell(SHEET, at("A1"), Value::Number(1.0)))
            as fn(&mut Package<Cursor<Vec<u8>>>),
        |p| p.clear_cell(SHEET, at("A1")),
    ] {
        let error = refused_edit(rows, edit);
        assert_package(error, "a formula has a type this layer does not know");
    }
    // Only the edited cell's formula is read: an edit beside it is made.
    let got = edited_sheet(rows, |p| p.set_cell(SHEET, at("B1"), Value::Number(7.0)));
    assert_eq!(
        got,
        sheet_with(&rows.replace("</c></row>", "</c><c r=\"B1\"><v>7</v></c></row>"))
    );
}

#[test]
fn a_value_edit_that_removes_no_formula_keeps_the_chain() {
    let parts = chain_with_formula();
    let output = edited_output(parts.clone(), |p| {
        p.set_cell(SHEET, at("B1"), Value::Number(3.0))
    });
    assert_untouched(&output, &parts, &[SHEET]);
}

#[test]
fn a_worksheet_without_explicit_references_is_refused() {
    for rows in [
        "<row><c r=\"A1\"><v>1</v></c></row>",
        "<row r=\"1\"><c><v>1</v></c></row>",
    ] {
        let error = refused_edit(rows, |p| p.set_cell(SHEET, at("B2"), Value::Number(1.0)));
        assert_package(
            error,
            "a worksheet without explicit references cannot be edited",
        );
    }
}

#[test]
fn a_worksheet_out_of_order_is_refused() {
    for rows in [
        "<row r=\"2\"/><row r=\"1\"/>",
        "<row r=\"1\"><c r=\"B1\"/><c r=\"A1\"/></row>",
        "<row r=\"1\"><c r=\"A1\"/><c r=\"A1\"/></row>",
        "<row r=\"1\"/><row r=\"1\"/>",
        "<row r=\"1\"><c r=\"A2\"/></row>",
    ] {
        let error = refused_edit(rows, |p| p.set_cell(SHEET, at("C9"), Value::Number(1.0)));
        assert_package(
            error,
            "a worksheet whose rows or cells are out of order cannot be edited",
        );
    }
}

#[test]
fn edits_to_two_sheets_go_in_one_write() {
    let mut parts = with_rows("<row r=\"1\"/>");
    let second = b"xl/worksheets/sheet2.xml";
    parts.push((second.to_vec(), sheet_with("").into_bytes()));
    let output = edited_output(parts.clone(), |p| {
        p.set_cell(SHEET, at("A1"), Value::Number(1.0));
        p.set_cell(second, at("A1"), Value::Number(2.0));
    });
    assert_untouched(&output, &parts, &[SHEET, second]);
    assert_part(
        &output,
        SHEET,
        &sheet_with("<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>"),
    );
    assert_part(
        &output,
        second,
        &sheet_with("<row r=\"1\"><c r=\"A1\"><v>2</v></c></row>"),
    );
}

#[test]
fn a_cell_edit_to_a_part_that_is_not_a_worksheet_is_refused_by_root() {
    let (_, mut package) = opened(smallest());
    package.set_cell(b"xl/styles.xml", at("A1"), Value::Number(1.0));
    assert_package(
        refused(package),
        "the worksheet part's root is not worksheet",
    );
}

#[test]
fn a_cell_edit_to_a_missing_removed_or_replaced_part_is_refused() {
    let (_, mut package) = opened(smallest());
    package.set_cell(b"xl/worksheets/nowhere.xml", at("A1"), Value::Number(1.0));
    assert_package(refused(package), "no such part");

    let (_, mut package) = opened(smallest());
    package.remove_part(SHEET);
    package.set_cell(SHEET, at("A1"), Value::Number(1.0));
    assert_package(refused(package), "an edit to a removed part");

    let (_, mut package) = opened(smallest());
    package.replace_part(SHEET, replacement_from_reader(Cursor::new(Vec::new())));
    package.clear_cell(SHEET, at("A1"));
    assert_package(
        refused(package),
        "a cell edit to a part this write replaces",
    );
}

#[test]
fn every_byte_outside_sheet_data_is_kept() {
    let before = "<dimension ref=\"A1:B2\"/><sheetViews><sheetView workbookViewId=\"0\"/>\
                  </sheetViews><cols><col min=\"1\" max=\"2\" width=\"9\"/></cols>\n";
    let after = "\n<mergeCells count=\"1\"><mergeCell ref=\"A1:B1\"/></mergeCells>\
                 <pageMargins left=\"0.7\" right=\"0.7\" top=\"0.75\" bottom=\"0.75\" \
                 header=\"0.3\" footer=\"0.3\"/>";
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        format!(
            "\u{FEFF}{DECLARATION}\r\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
             {before}<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row></sheetData>{after}</worksheet>"
        ),
    );
    let output = edited_output(parts, |p| p.set_cell(SHEET, at("B2"), Value::Number(2.0)));
    assert_part(
        &output,
        SHEET,
        &format!(
            "\u{FEFF}{DECLARATION}\r\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
             {before}<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row>\
             <row r=\"2\"><c r=\"B2\"><v>2</v></c></row></sheetData>{after}</worksheet>"
        ),
    );
}

#[test]
fn the_strict_namespace_is_edited_as_the_transitional_one() {
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        format!(
            "{DECLARATION}\n<x:worksheet xmlns:x=\"{MAIN_STRICT}\"><x:sheetData>\
             <x:row r=\"1\"><x:c r=\"A1\"><x:v>1</x:v></x:c></x:row></x:sheetData></x:worksheet>"
        ),
    );
    let output = edited_output(parts, |p| {
        p.set_cell(SHEET, at("A1"), Value::Number(2.0));
        p.set_cell(SHEET, at("B1"), Value::Text("t".to_string()));
        p.set_cell(SHEET, at("A2"), Value::Boolean(true));
    });
    // The part binds the strict main namespace to a prefix and no default,
    // so every new element takes the prefix of the element it goes into:
    // unprefixed, it would be in no namespace and no reader would see it.
    assert_part(
        &output,
        SHEET,
        &format!(
            "{DECLARATION}\n<x:worksheet xmlns:x=\"{MAIN_STRICT}\"><x:sheetData>\
             <x:row r=\"1\"><x:c r=\"A1\"><x:v>2</x:v></x:c><x:c r=\"B1\" t=\"inlineStr\">\
             <x:is><x:t xml:space=\"preserve\">t</x:t></x:is></x:c></x:row>\
             <x:row r=\"2\"><x:c r=\"A2\" t=\"b\"><x:v>1</x:v></x:c></x:row>\
             </x:sheetData></x:worksheet>"
        ),
    );
    let mut package = Package::open(Cursor::new(output)).expect("the output opens");
    let cells: Vec<crate::cells::Cell> = package
        .cells(SHEET)
        .expect("the reader")
        .collect::<Result<_, _>>()
        .expect("the cells");
    assert_eq!(cells.len(), 3, "every new cell is in the main namespace");
}

#[test]
fn the_values_written_read_back_through_the_cell_reader() {
    let output = edited_output(smallest(), |p| {
        p.set_cell(SHEET, at("A1"), Value::Number(42.5));
        p.set_cell(SHEET, at("B1"), Value::Text("emend\r\n".to_string()));
        p.set_cell(SHEET, at("C1"), Value::Boolean(true));
    });
    let mut package = Package::open(Cursor::new(output)).expect("the output opens");
    let cells: Vec<crate::cells::Cell> = package
        .cells(SHEET)
        .expect("the reader")
        .collect::<Result<_, _>>()
        .expect("the cells");
    assert_eq!(cells.len(), 3);
    assert_eq!(cells[0].number(), Some(42.5));
    assert_eq!(cells[1].kind, crate::cells::Kind::InlineString);
    assert_eq!(cells[1].value.as_deref(), Some("emend\r\n"));
    assert_eq!(cells[2].boolean(), Some(true));
}

#[test]
fn a_refused_cell_edit_leaves_no_temporary_and_no_output() {
    // The refusal is found while the part streams, after its temporary
    // exists: `refused` asserts the output is empty, and the temporary
    // directory holds what it held before.
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let before = replacement_temporaries();
    let (_, mut package) = opened(with_rows("<row r=\"2\"/><row r=\"1\"/>"));
    package.set_cell(SHEET, at("A1"), Value::Number(1.0));
    let mut out = Vec::new();
    let error = package.write_to(&mut out).expect_err("refused");
    assert_package(
        error,
        "a worksheet whose rows or cells are out of order cannot be edited",
    );
    assert!(out.is_empty(), "a refused write wrote to the output");
    assert_eq!(replacement_temporaries(), before, "a temporary was left");
}
