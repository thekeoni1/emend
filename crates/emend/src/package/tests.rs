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
fn written(package: &mut Package<Cursor<Vec<u8>>>) -> Vec<u8> {
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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
    let output = written(&mut package);

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
#[track_caller]
fn refused(package: &mut Package<Cursor<Vec<u8>>>) -> Error {
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut out = Vec::new();
    let error = package.write_to(&mut out).expect_err("refused");
    assert!(
        out.is_empty(),
        "a refused set wrote {} byte(s) to the output",
        out.len()
    );
    error
}

#[test]
fn removing_an_absent_relationship() {
    let (_, mut package) = opened(smallest());
    package.remove_relationship(b"xl/workbook.xml", b"rId9");
    assert_package(refused(&mut package), "no such relationship");
}

#[test]
fn removing_an_absent_override() {
    let (_, mut package) = opened(smallest());
    package.remove_override(b"/xl/styles.xml");
    assert_package(refused(&mut package), "no such override");
}

#[test]
fn adding_an_override_that_exists() {
    let (_, mut package) = opened(smallest());
    package.add_override(b"/xl/workbook.xml", b"application/xml");
    assert_package(
        refused(&mut package),
        "an override for that part already exists",
    );
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
    let output = written(&mut package);

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
    assert_package(refused(&mut package), "an edit to a removed part");
}

#[test]
fn editing_a_removed_rels_part() {
    // The rels part a relationship edit would rewrite counts as edited, so
    // removing that part and editing its relationships in one write is
    // refused rather than resolved in some order.
    let (_, mut package) = opened(smallest());
    package.remove_part(b"xl/_rels/workbook.xml.rels");
    package.remove_relationship(b"xl/workbook.xml", b"rId2");
    assert_package(refused(&mut package), "an edit to a removed part");
}

#[test]
fn editing_removed_content_types() {
    let (_, mut package) = opened(smallest());
    package.remove_part(b"[Content_Types].xml");
    package.remove_override(b"/xl/workbook.xml");
    assert_package(refused(&mut package), "an edit to a removed part");
}

#[test]
fn removing_an_absent_part() {
    let (_, mut package) = opened(smallest());
    package.remove_part(b"xl/calcChain.xml");
    assert_package(refused(&mut package), "no such part");
}

#[test]
fn replacing_an_absent_part() {
    let (_, mut package) = opened(smallest());
    package.replace_part(
        b"xl/calcChain.xml",
        replacement_from_reader(Cursor::new(b"<calcChain/>".to_vec())),
    );
    assert_package(refused(&mut package), "no such part");
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
    assert_package(refused(&mut package), "no Default covers rels");
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
    let output = written(&mut package);

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
    let output = written(&mut package);
    assert_eq!(output, input, "an empty set of edits changed the bytes");
}
