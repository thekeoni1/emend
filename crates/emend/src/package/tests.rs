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
    Build, METHOD_DEFLATE, Spec, TEMPORARY_LOCK, crc32, keep, keep_dir, names, override_keep,
    parse_independently, record_fixture, replacement_temporaries, stored_data, take_fixture,
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
    (parts, open_fixture(bytes))
}

/// Opens `bytes` as a package and records them as this thread's fixture, so
/// [`written`] can keep them beside the output (`specs/0015-open-checks.md`,
/// P1 ruling 2).
#[track_caller]
fn open_fixture(bytes: Vec<u8>) -> Package<Cursor<Vec<u8>>> {
    record_fixture(&bytes);
    Package::open(Cursor::new(bytes)).expect("the fixture opens")
}

/// Writes the package back and returns the output bytes.
///
/// Holds [`TEMPORARY_LOCK`] for the write: a rewritten part and a
/// replacement each make a temporary of the one form, in this one process, so
/// a write here that ran beside the zip layer's snapshot tests would fail
/// them on a race rather than on either layer.
///
/// With a keep directory named, the fixture [`open_fixture`] recorded and the
/// output are kept there (`specs/0015-open-checks.md`, "The keep"). The
/// fixture is taken, so each write needs its own open, and it must be the
/// one this package was opened from: its entry names and CRC-32s, read
/// independently, are compared with the package's (P1 ruling 2).
#[track_caller]
fn written(package: Package<Cursor<Vec<u8>>>) -> Vec<u8> {
    let fixture = take_fixture();
    let keeping = keep_dir().is_some();
    if keeping {
        let fixture = fixture
            .as_deref()
            .expect("the keep has no fixture: open it with open_fixture");
        let recorded: Vec<(Vec<u8>, u32)> = parse_independently(fixture)
            .entries
            .iter()
            .map(|entry| (entry.name.clone(), entry.crc32))
            .collect();
        let opened: Vec<(Vec<u8>, u32)> = package
            .archive()
            .entries()
            .iter()
            .map(|entry| (entry.name().to_vec(), entry.crc32()))
            .collect();
        assert_eq!(
            recorded, opened,
            "the keep's fixture is not the one this package was opened from"
        );
    }
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut out = Vec::new();
    package.write_to(&mut out).expect("write_to");
    if let Some(fixture) = fixture.filter(|_| keeping) {
        keep(&fixture, &out);
    }
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
    let mut package = open_fixture(input.clone());
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
    // Part 5 P1 adds the fourth (`specs/0012-styles.md`, "Errors", and P1
    // ruling 2): the styles part's reading raises from its own module.
    const STYLES: &str = include_str!("../styles.rs");
    // Milestone 2 part 2 P1 adds the fifth (`specs/0017-fuller-styles.md`,
    // "Errors"): a style field's refusals raise from `styles`'s submodule.
    const ELEMENTS: &str = include_str!("../styles/elements.rs");
    // Milestone 3 part 5 P1 adds the sixth (`specs/0027-read-back.md`,
    // "Errors"): the typed read of the styles part raises from its own
    // submodule, compiled with the Cargo feature `read` and read here
    // whatever the features.
    const READ: &str = include_str!("../styles/read.rs");
    // Part 5 P2 adds the seventh: the worksheet's typed read raises from
    // `package`'s own submodule, compiled with the Cargo feature `read`.
    const SHEET_READ: &str = include_str!("sheet_read.rs");
    // Milestone 5 part 2 P3 adds the eighth and ninth
    // (`specs/0033-read-breadth-survey.md`, "Errors"): the pivot table and
    // external link readers raise from `package`'s submodules, compiled with
    // the Cargo feature `read`.
    const PIVOT: &str = include_str!("pivot.rs");
    const EXTLINK: &str = include_str!("extlink.rs");
    // Milestone 5 part 3 P2 adds the tenth (`specs/0034-charts-read.md`,
    // "Errors"): the chart reader raises from `package`'s submodule,
    // compiled with the Cargo feature `charts`.
    const CHART: &str = include_str!("chart.rs");
    // Milestone 5 part 4 P2 adds the eleventh
    // (`specs/0035-charts-write-first-set.md`, "Errors", P1 verification
    // ruling G): the chart writer raises from `package`'s submodule,
    // compiled with the Cargo feature `charts`.
    const CHART_WRITE: &str = include_str!("chart_write.rs");
    // Milestone 6 part 4 P2 adds the twelfth (`specs/0041-chart-text-and-
    // edits.md`, "Errors"): the in-place chart edits, which raise through
    // the chart writer's constants.
    const CHART_EDIT: &str = include_str!("chart_edit.rs");

    let (package_raised, package_calls, package_literals) = raised(PACKAGE);
    let (xml_raised, xml_calls, xml_literals) = raised(XML);
    let (cells_raised, cells_calls, cells_literals) = raised(CELLS);
    let (styles_raised, styles_calls, styles_literals) = raised(STYLES);
    let (elements_raised, elements_calls, elements_literals) = raised(ELEMENTS);
    let (read_raised, read_calls, read_literals) = raised(READ);
    let (sheet_raised, sheet_calls, sheet_literals) = raised(SHEET_READ);
    let (pivot_raised, pivot_calls, pivot_literals) = raised(PIVOT);
    let (extlink_raised, extlink_calls, extlink_literals) = raised(EXTLINK);
    let (chart_raised, chart_calls, chart_literals) = raised(CHART);
    let (write_raised, write_calls, write_literals) = raised(CHART_WRITE);
    let (edit_raised, edit_calls, edit_literals) = raised(CHART_EDIT);
    // Every `Error::Package(` in any of the twelve takes a literal. If one
    // ever takes a variable the extractor cannot read it, and this is the
    // line that says so instead of the comparison quietly passing on a
    // subset.
    assert_eq!(
        (
            package_calls,
            xml_calls,
            cells_calls,
            styles_calls,
            elements_calls,
            read_calls,
            sheet_calls,
            pivot_calls,
            extlink_calls,
            chart_calls,
            write_calls,
            edit_calls
        ),
        (
            package_literals,
            xml_literals,
            cells_literals,
            styles_literals,
            elements_literals,
            read_literals,
            sheet_literals,
            pivot_literals,
            extlink_literals,
            chart_literals,
            write_literals,
            edit_literals
        ),
        "an Error::Package is raised with something other than a literal"
    );

    // The catalogue lives in `package.rs`; `xml.rs`, `cells.rs`, `styles.rs`,
    // `styles/elements.rs`, `styles/read.rs`, `package/sheet_read.rs`,
    // `package/pivot.rs`, `package/extlink.rs`, `package/chart.rs`,
    // `package/chart_write.rs` and `package/chart_edit.rs` say so rather than
    // keeping a list of their own.
    let catalogue = catalogued(PACKAGE);
    let mut raised_all = package_raised;
    raised_all.extend(xml_raised);
    raised_all.extend(cells_raised);
    raised_all.extend(styles_raised);
    raised_all.extend(elements_raised);
    raised_all.extend(read_raised);
    raised_all.extend(sheet_raised);
    raised_all.extend(pivot_raised);
    raised_all.extend(extlink_raised);
    raised_all.extend(chart_raised);
    raised_all.extend(write_raised);
    raised_all.extend(edit_raised);

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

const WORKBOOK: &[u8] = b"xl/workbook.xml";

/// The smallest package's workbook part as a write that asks for a full
/// calculation on load leaves it: `<calcPr fullCalcOnLoad="1"/>` right after
/// `</sheets>`, its last child (`specs/0013-formulas.md`, P1 ruling 1).
fn flagged_workbook() -> String {
    format!(
        "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
         <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets>\
         <calcPr fullCalcOnLoad=\"1\"/></workbook>"
    )
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
    let parts = with_rows(rows);
    let output = edited_output(parts.clone(), |p| {
        p.clear_cell(SHEET, at("A1"));
        p.clear_cell(SHEET, at("B1"));
        p.clear_cell(SHEET, at("C1"));
    });
    // A row left with no cells but its style-kept one stays, attributes and
    // the whitespace around the dropped cells included.
    assert_part(
        &output,
        SHEET,
        &sheet_with("<row r=\"1\">\n  <c r=\"A1\" s=\"4\"/>\n  \n  \n</row>"),
    );
    // A1's normal formula went, so the formula-change rule asks for a full
    // calculation on load (`specs/0013-formulas.md`, P1 ruling 4); nothing
    // else changes.
    assert_part(&output, WORKBOOK, &flagged_workbook());
    assert_untouched(&output, &parts, &[SHEET, WORKBOOK]);
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
        // The removed formula also asks for a full calculation on load
        // (`specs/0013-formulas.md`, P1 ruling 4).
        assert_part(&output, WORKBOOK, &flagged_workbook());
        assert_untouched(
            &output,
            &parts,
            &[
                b"[Content_Types].xml",
                b"xl/_rels/workbook.xml.rels",
                b"xl/calcChain.xml",
                SHEET,
                WORKBOOK,
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
fn replacing_a_formula_without_a_chain_changes_the_sheet_and_the_flag_alone() {
    // Part 4's `replacing_a_formula_without_a_chain_changes_the_sheet_alone`,
    // renamed: the formula-change rule now asks for a full calculation on
    // load whether or not there is a chain (`specs/0013-formulas.md`, P1
    // ruling 4), so the workbook part changes too, and nothing else.
    let mut parts = with_rows("<row r=\"1\"><c r=\"A1\"><f>1+1</f><v>2</v></c></row>");
    let output = edited_output(parts.clone(), |p| {
        p.set_cell(SHEET, at("A1"), Value::Number(3.0))
    });
    assert_untouched(&output, &parts, &[SHEET, WORKBOOK]);
    assert_part(&output, WORKBOOK, &flagged_workbook());
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
    package
        .parts()
        .drop_calc_chain_once()
        .expect("the chain rule");
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
    package
        .parts()
        .drop_calc_chain_once()
        .expect("the chain rule");
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
fn a_cell_edit_off_the_grid_is_refused_at_the_write() {
    // specs/0023-scale.md, P1 verification ruling 5: a value, a clear or a
    // style recorded for a cell past row 1048576 or column 16384, or at row
    // or column 0, is refused by the write, and the last cell of the grid is
    // written.
    let past_row = CellRef {
        row: 1_048_577,
        col: 1,
    };
    let past_col = CellRef {
        row: 1,
        col: 16_385,
    };
    let (_, mut package) = opened(smallest());
    package.set_cell(SHEET, past_row, Value::Number(1.0));
    assert_package(
        refused(package),
        "a row or column index or count is out of range",
    );

    let (_, mut package) = opened(smallest());
    package.clear_cell(SHEET, past_col);
    assert_package(
        refused(package),
        "a row or column index or count is out of range",
    );

    let (_, mut package) = opened(smallest());
    package.set_cell_style(SHEET, past_row, None);
    assert_package(
        refused(package),
        "a row or column index or count is out of range",
    );

    let (_, mut package) = opened(smallest());
    package.set_cell(SHEET, CellRef { row: 0, col: 1 }, Value::Number(1.0));
    assert_package(
        refused(package),
        "a row or column index or count is out of range",
    );

    let (_, mut package) = opened(smallest());
    package.set_cell_style(SHEET, CellRef { row: 1, col: 0 }, None);
    assert_package(
        refused(package),
        "a row or column index or count is out of range",
    );

    let last = CellRef {
        row: 1_048_576,
        col: 16_384,
    };
    let sheet = edited_sheet("", |p| p.set_cell(SHEET, last, Value::Number(1.0)));
    assert_eq!(
        sheet,
        sheet_with("<row r=\"1048576\"><c r=\"XFD1048576\"><v>1</v></c></row>")
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
    assert_eq!(cells[1].kind, crate::cells::CellKind::InlineString);
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
    // Not `refused`: it takes the lock this test already holds.
    let error = match package.write_to(&mut out) {
        Ok(()) => panic!("the write was not refused: it wrote {} byte(s)", out.len()),
        Err(error) => error,
    };
    assert_package(
        error,
        "a worksheet whose rows or cells are out of order cannot be edited",
    );
    assert!(out.is_empty(), "a refused write wrote to the output");
    assert_eq!(replacement_temporaries(), before, "a temporary was left");
}

// ------------------------------------------------------------------ styles
//
// `specs/0012-styles.md`, "P1", second code commit: cell formats added to
// the styles part and style edits applied to worksheets, the output's parts
// read back through the independent zip reader and compared with bytes each
// test writes out by hand.

const STYLES_PART: &[u8] = b"xl/styles.xml";

/// A styles part in the main namespace with `body` between its root tags.
fn style_sheet(body: &str) -> String {
    format!("{DECLARATION}\n<styleSheet xmlns=\"{MAIN_NS}\">{body}</styleSheet>")
}

/// The smallest package with its styles part holding `body` and its
/// worksheet's `<sheetData>` holding `rows`.
fn styled(body: &str, rows: &str) -> Parts {
    let mut parts = with_rows(rows);
    set(&mut parts, STYLES_PART, style_sheet(body));
    parts
}

/// Four cell formats and no number formats: what the style-edit tests need
/// an index to name.
const FOUR_XFS: &str = "<cellXfs count=\"4\"><xf/><xf/><xf/><xf/></cellXfs>";

/// The styles part after `edit` on a package whose styles part holds
/// `body`, as text; every other part is asserted untouched.
#[track_caller]
fn added_styles(body: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let parts = styled(body, "");
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[STYLES_PART]);
    String::from_utf8(stored_data(&output, STYLES_PART)).expect("the part is UTF-8")
}

/// Records one addition and asserts the index it returns.
#[track_caller]
fn add(
    package: &mut Package<Cursor<Vec<u8>>>,
    base: Option<u32>,
    format: NumberFormat,
    index: u32,
) {
    let got = package
        .add_cell_style(base, format)
        .expect("the addition is recorded");
    assert_eq!(got, index, "the index the addition returns");
}

/// A `NumberFormat::Code`.
fn code(text: &str) -> NumberFormat {
    NumberFormat::Code(text.to_string())
}

/// The worksheet part after `edit` on a package whose styles part holds
/// `body` and whose `<sheetData>` holds `rows`, as text; every part but the
/// worksheet and the styles part is asserted untouched.
#[track_caller]
fn restyled_sheet(
    body: &str,
    rows: &str,
    edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>),
) -> String {
    let parts = styled(body, rows);
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[SHEET, STYLES_PART]);
    String::from_utf8(stored_data(&output, SHEET)).expect("the part is UTF-8")
}

#[test]
fn an_xf_added_from_a_base_with_children_keeps_them_and_rebuilds_its_tag() {
    // The first base has `numFmtId` and `applyNumberFormat` (both replaced)
    // and spacing a rebuild changes; the second has neither (`numFmtId`
    // gained first, `applyNumberFormat` appended last). Both keep their
    // children raw, and the existing children stay byte for byte. The part
    // has no `numFmts`, so the code's gains one before `fonts`.
    let first = "<xf numFmtId=\"0\"  fontId=\"0\"\n applyNumberFormat=\"0\" applyAlignment=\"1\">\
                 <alignment horizontal=\"center\"/></xf>";
    let second = "<xf fontId=\"1\" applyAlignment=\"1\">\n<alignment wrapText=\"1\"/></xf>";
    let fonts = "<fonts count=\"2\"><font><sz val=\"11\"/></font><font><b/></font></fonts>";
    let got = added_styles(
        &format!("{fonts}<cellXfs count=\"2\">{first}{second}</cellXfs>"),
        |p| {
            add(p, Some(0), code("0.000"), 2);
            add(p, Some(1), NumberFormat::BuiltIn(14), 3);
        },
    );
    assert_eq!(
        got,
        style_sheet(&format!(
            "<numFmts count=\"1\"><numFmt numFmtId=\"164\" formatCode=\"0.000\"/></numFmts>\
             {fonts}<cellXfs count=\"4\">{first}{second}\
             <xf numFmtId=\"164\" fontId=\"0\" applyNumberFormat=\"1\" applyAlignment=\"1\">\
             <alignment horizontal=\"center\"/></xf>\
             <xf numFmtId=\"14\" fontId=\"1\" applyAlignment=\"1\" applyNumberFormat=\"1\">\n\
             <alignment wrapText=\"1\"/></xf></cellXfs>"
        ))
    );
}

#[test]
fn an_xf_added_from_a_single_quoted_base_keeps_each_quote() {
    // P1 ruling 5: a single-quoted value holding a double quote stays
    // well-formed because each attribute keeps its own quote character.
    let base = "<xf numFmtId='0' fontId='0' other='a\"b'/>";
    let got = added_styles(&format!("<cellXfs count=\"1\">{base}</cellXfs>"), |p| {
        add(p, Some(0), NumberFormat::BuiltIn(2), 1);
    });
    assert_eq!(
        got,
        style_sheet(&format!(
            "<cellXfs count=\"2\">{base}\
             <xf numFmtId='2' fontId='0' other='a\"b' applyNumberFormat=\"1\"/></cellXfs>"
        ))
    );
}

#[test]
fn an_xf_added_from_an_empty_base_and_from_none() {
    // A built-in adds nothing to `numFmts`, which keeps its bytes.
    let num_fmts = "<numFmts count=\"1\"><numFmt numFmtId=\"164\" formatCode=\"0.0\"/></numFmts>";
    let base = "<xf numFmtId=\"164\" fontId=\"2\" fillId=\"0\" borderId=\"0\" xfId=\"0\"/>";
    let got = added_styles(
        &format!("{num_fmts}<cellXfs count=\"1\">{base}</cellXfs>"),
        |p| {
            add(p, Some(0), NumberFormat::BuiltIn(22), 1);
            add(p, None, NumberFormat::BuiltIn(14), 2);
        },
    );
    assert_eq!(
        got,
        style_sheet(&format!(
            "{num_fmts}<cellXfs count=\"3\">{base}\
             <xf numFmtId=\"22\" fontId=\"2\" fillId=\"0\" borderId=\"0\" xfId=\"0\" \
             applyNumberFormat=\"1\"/>\
             <xf numFmtId=\"14\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\" \
             applyNumberFormat=\"1\"/></cellXfs>"
        ))
    );
}

/// The form a `None` base takes, with `id` as its number format.
fn plain_xf(id: u32) -> String {
    format!(
        "<xf numFmtId=\"{id}\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\" \
         applyNumberFormat=\"1\"/>"
    )
}

#[test]
fn a_new_code_takes_164_on_a_part_with_no_custom_id() {
    // A producer's spelling of built-in 14 is the only numFmt: the new id is
    // 164, which is larger than 14 plus one.
    let spelled = "<numFmt numFmtId=\"14\" formatCode=\"d/m/yyyy\"/>";
    let got = added_styles(
        &format!("<numFmts count=\"1\">{spelled}</numFmts><cellXfs count=\"1\"><xf/></cellXfs>"),
        |p| add(p, None, code("0.00%"), 1),
    );
    assert_eq!(
        got,
        style_sheet(&format!(
            "<numFmts count=\"2\">{spelled}<numFmt numFmtId=\"164\" formatCode=\"0.00%\"/>\
             </numFmts><cellXfs count=\"2\"><xf/>{}</cellXfs>",
            plain_xf(164)
        ))
    );
}

#[test]
fn a_code_is_matched_or_numbered_after_the_largest_id() {
    // Equal to an existing custom code: its id, nothing added. Equal to a
    // spelled-out built-in: that id below 164, nothing added. New codes:
    // one more than the largest id, the part's or one already recorded,
    // written in id order. Two additions with an equal new code share one
    // `numFmt` and are two `xf`s.
    let formats = "<numFmt numFmtId=\"170\" formatCode=\"0.0\"/>\
                   <numFmt numFmtId=\"14\" formatCode=\"d/m/yyyy\"/>\
                   <numFmt numFmtId=\"164\" formatCode=\"#,##0\"/>";
    let got = added_styles(
        &format!("<numFmts count=\"3\">{formats}</numFmts><cellXfs count=\"1\"><xf/></cellXfs>"),
        |p| {
            add(p, None, code("0.0"), 1);
            add(p, None, code("d/m/yyyy"), 2);
            add(p, None, code("new one"), 3);
            add(p, None, code("new two"), 4);
            add(p, None, code("new one"), 5);
        },
    );
    assert_eq!(
        got,
        style_sheet(&format!(
            "<numFmts count=\"5\">{formats}\
             <numFmt numFmtId=\"171\" formatCode=\"new one\"/>\
             <numFmt numFmtId=\"172\" formatCode=\"new two\"/></numFmts>\
             <cellXfs count=\"6\"><xf/>{}{}{}{}{}</cellXfs>",
            plain_xf(170),
            plain_xf(14),
            plain_xf(171),
            plain_xf(172),
            plain_xf(171)
        ))
    );
}

#[test]
fn a_code_is_escaped_and_reads_back_equal_to_itself() {
    // P1 ruling 3: `escape`, then a tab, a line feed and a carriage return
    // as references, so the reader's normalisation gives the code back.
    let text = "0.0\"x\"&<\t\n\r'";
    let parts = styled("<cellXfs count=\"1\"><xf/></cellXfs>", "");
    let output = edited_output(parts, |p| add(p, None, code(text), 1));
    let got = String::from_utf8(stored_data(&output, STYLES_PART)).expect("UTF-8");
    assert_eq!(
        got,
        style_sheet(&format!(
            "<numFmts count=\"1\"><numFmt numFmtId=\"164\" \
             formatCode=\"0.0&quot;x&quot;&amp;&lt;&#9;&#10;&#13;&apos;\"/></numFmts>\
             <cellXfs count=\"2\"><xf/>{}</cellXfs>",
            plain_xf(164)
        ))
    );
    let mut reopened = Package::open(Cursor::new(output)).expect("the output opens");
    let styles = reopened.styles().expect("the output's styles read");
    assert_eq!(styles.number_format(1), Some(code(text)));
}

#[test]
fn counts_are_rewritten_and_empty_containers_opened_up() {
    // Both counts rewritten; a container without `count` gets none; an
    // empty container with `count` and one without are opened up.
    let got = added_styles("<numFmts count=\"0\"/><cellXfs><xf/></cellXfs>", |p| {
        add(p, None, code("0.0"), 1)
    });
    assert_eq!(
        got,
        style_sheet(&format!(
            "<numFmts count=\"1\"><numFmt numFmtId=\"164\" formatCode=\"0.0\"/></numFmts>\
             <cellXfs><xf/>{}</cellXfs>",
            plain_xf(164)
        ))
    );
    let got = added_styles("<numFmts/><cellXfs count=\"0\" />", |p| {
        add(p, None, code("0.0"), 0)
    });
    assert_eq!(
        got,
        style_sheet(&format!(
            "<numFmts><numFmt numFmtId=\"164\" formatCode=\"0.0\"/></numFmts>\
             <cellXfs count=\"1\" >{}</cellXfs>",
            plain_xf(164)
        ))
    );
}

#[test]
fn a_part_with_cell_xfs_alone_gains_num_fmts_before_it() {
    let got = added_styles("<cellXfs count=\"1\"><xf/></cellXfs><cellStyles/>", |p| {
        add(p, None, code("0.0"), 1)
    });
    assert_eq!(
        got,
        style_sheet(&format!(
            "<numFmts count=\"1\"><numFmt numFmtId=\"164\" formatCode=\"0.0\"/></numFmts>\
             <cellXfs count=\"2\"><xf/>{}</cellXfs><cellStyles/>",
            plain_xf(164)
        ))
    );
}

#[test]
fn a_prefixed_part_gets_prefixed_elements() {
    // The new `xf` and `numFmt` take their container's prefix, and a
    // created `numFmts` the root's.
    let prefixed = |body: &str| {
        format!("{DECLARATION}\n<x:styleSheet xmlns:x=\"{MAIN_NS}\">{body}</x:styleSheet>")
    };
    for with_num_fmts in [true, false] {
        let existing = if with_num_fmts {
            "<x:numFmts count=\"1\"><x:numFmt numFmtId=\"164\" formatCode=\"0\"/></x:numFmts>"
        } else {
            ""
        };
        let mut parts = styled("", "");
        set(
            &mut parts,
            STYLES_PART,
            prefixed(&format!(
                "{existing}<x:cellXfs count=\"1\"><x:xf numFmtId=\"0\"/></x:cellXfs>"
            )),
        );
        let output = edited_output(parts, |p| add(p, Some(0), code("0.0"), 1));
        let id = if with_num_fmts { 165 } else { 164 };
        let num_fmts = if with_num_fmts {
            "<x:numFmts count=\"2\"><x:numFmt numFmtId=\"164\" formatCode=\"0\"/>\
             <x:numFmt numFmtId=\"165\" formatCode=\"0.0\"/></x:numFmts>"
        } else {
            "<x:numFmts count=\"1\"><x:numFmt numFmtId=\"164\" formatCode=\"0.0\"/></x:numFmts>"
        };
        assert_part(
            &output,
            STYLES_PART,
            &prefixed(&format!(
                "{num_fmts}<x:cellXfs count=\"2\"><x:xf numFmtId=\"0\"/>\
                 <x:xf numFmtId=\"{id}\" applyNumberFormat=\"1\"/></x:cellXfs>"
            )),
        );
    }
}

#[test]
fn every_byte_outside_the_two_containers_is_kept() {
    // Every root child of a full part, in the schema's order, with the
    // corpus's one line feed after the declaration and none between
    // elements; only the two containers change.
    let before = "<fonts count=\"1\"><font><sz val=\"11\"/><name val=\"A\"/></font></fonts>\
                  <fills count=\"1\"><fill><patternFill patternType=\"none\"/></fill></fills>\
                  <borders count=\"1\"><border><left/><right/></border></borders>\
                  <cellStyleXfs count=\"1\"><xf numFmtId=\"0\" fontId=\"0\"/></cellStyleXfs>";
    let after = "<cellStyles count=\"1\"><cellStyle name=\"N\" xfId=\"0\" builtinId=\"0\"/>\
                 </cellStyles><dxfs count=\"1\"><dxf><numFmt numFmtId=\"164\" \
                 formatCode=\"0\"/></dxf></dxfs><tableStyles count=\"0\"/>\
                 <extLst><ext uri=\"urn:x\"><cellXfs><xf/></cellXfs></ext></extLst>";
    let formats = "<numFmt numFmtId=\"164\" formatCode=\"0.0\"/>";
    let xfs = "<xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\"/>";
    let got = added_styles(
        &format!(
            "<numFmts count=\"1\">{formats}</numFmts>{before}\
             <cellXfs count=\"1\">{xfs}</cellXfs>{after}"
        ),
        |p| add(p, Some(0), code("0.00"), 1),
    );
    assert_eq!(
        got,
        style_sheet(&format!(
            "<numFmts count=\"2\">{formats}<numFmt numFmtId=\"165\" formatCode=\"0.00\"/>\
             </numFmts>{before}<cellXfs count=\"2\">{xfs}\
             <xf numFmtId=\"165\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\" \
             applyNumberFormat=\"1\"/></cellXfs>{after}"
        ))
    );
}

#[test]
fn an_addition_is_refused_on_a_bad_base_or_format() {
    let (_, mut package) = opened(styled("<cellXfs count=\"2\"><xf/><xf/></cellXfs>", ""));
    let refused_with = |package: &mut Package<Cursor<Vec<u8>>>, base, format| match package
        .add_cell_style(base, format)
    {
        Ok(index) => panic!("not refused: recorded as {index}"),
        Err(error) => error,
    };
    assert_package(
        refused_with(&mut package, Some(2), NumberFormat::BuiltIn(14)),
        "a style index is out of range",
    );
    assert_package(
        refused_with(&mut package, Some(u32::MAX), NumberFormat::BuiltIn(14)),
        "a style index is out of range",
    );
    assert_package(
        refused_with(&mut package, None, NumberFormat::BuiltIn(164)),
        "a built-in number format id is 164 or more",
    );
    for text in ["", "0\u{1}0", "0\u{FFFE}"] {
        assert_package(
            refused_with(&mut package, None, code(text)),
            "a number format code is empty or holds a character XML cannot carry",
        );
    }
    // Nothing was recorded by the refusals: the next index is the part's
    // count, and a base names the part's own `xf`s only.
    add(&mut package, Some(1), NumberFormat::BuiltIn(163), 2);
    assert_package(
        refused_with(&mut package, Some(2), NumberFormat::BuiltIn(14)),
        "a style index is out of range",
    );
}

#[test]
fn a_part_without_cell_xfs_is_refused_at_write_with_no_output() {
    // The refusal is found once the part has streamed into its temporary:
    // the output is empty and no temporary is left.
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let before = replacement_temporaries();
    let (_, mut package) = opened(styled("<numFmts count=\"0\"/>", ""));
    let index = package
        .add_cell_style(None, NumberFormat::BuiltIn(14))
        .expect("recorded: the part is read without cellXfs");
    assert_eq!(index, 0);
    let mut out = Vec::new();
    // Not `refused`: it takes the lock this test already holds.
    let error = match package.write_to(&mut out) {
        Ok(()) => panic!("the write was not refused: it wrote {} byte(s)", out.len()),
        Err(error) => error,
    };
    assert_package(error, "a styles part without cellXfs cannot take a style");
    assert!(out.is_empty(), "a refused write wrote to the output");
    assert_eq!(replacement_temporaries(), before, "a temporary was left");
}

#[test]
fn a_count_that_is_not_a_number_is_refused_at_write() {
    let (_, mut package) = opened(styled("<cellXfs count=\"two\"><xf/><xf/></cellXfs>", ""));
    add(&mut package, None, NumberFormat::BuiltIn(14), 2);
    assert_package(refused(package), "a style attribute is not a number");
}

#[test]
fn a_value_edit_takes_the_style_edit_or_keeps_the_cells_own() {
    let rows = "<row r=\"1\"><c r=\"A1\" s=\"3\"><v>0</v></c><c r=\"B1\" s=\"3\"><v>0</v></c>\
                <c r=\"C1\" s=\"3\"><v>0</v></c></row>";
    let got = restyled_sheet(FOUR_XFS, rows, |p| {
        p.set_cell(SHEET, at("A1"), Value::Number(1.0));
        p.set_cell_style(SHEET, at("A1"), Some(1));
        p.set_cell(SHEET, at("B1"), Value::Number(2.0));
        p.set_cell_style(SHEET, at("B1"), None);
        p.set_cell(SHEET, at("C1"), Value::Number(3.0));
        // And a new cell with a style.
        p.set_cell(SHEET, at("D1"), Value::Boolean(true));
        p.set_cell_style(SHEET, at("D1"), Some(2));
    });
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\" s=\"1\"><v>1</v></c><c r=\"B1\"><v>2</v></c>\
             <c r=\"C1\" s=\"3\"><v>3</v></c><c r=\"D1\" s=\"2\" t=\"b\"><v>1</v></c></row>"
        )
    );
}

#[test]
fn a_clear_takes_the_style_edit() {
    // `Some(i)` leaves `<c r s/>`, `None` removes the cell, and a clear of
    // an absent cell with `Some(i)` writes the style alone.
    let rows =
        "<row r=\"1\"><c r=\"A1\" s=\"3\"><v>0</v></c><c r=\"B1\" s=\"3\"><v>0</v></c></row>";
    let got = restyled_sheet(FOUR_XFS, rows, |p| {
        p.clear_cell(SHEET, at("A1"));
        p.set_cell_style(SHEET, at("A1"), Some(2));
        p.clear_cell(SHEET, at("B1"));
        p.set_cell_style(SHEET, at("B1"), None);
        p.clear_cell(SHEET, at("C1"));
        p.set_cell_style(SHEET, at("C1"), Some(1));
    });
    assert_eq!(
        got,
        sheet_with("<row r=\"1\"><c r=\"A1\" s=\"2\"/><c r=\"C1\" s=\"1\"/></row>")
    );
}

#[test]
fn a_style_edit_alone_rewrites_the_start_tag_and_nothing_else() {
    // A formula (its `<f>` and `<v>` raw), an inline string, an empty
    // styled cell, an array master and a cell inside its `ref` (allowed:
    // no formula refusal applies to a style edit), and `None` removing an
    // `s` from a cell with a type.
    let rows = "<row r=\"1\"><c r=\"A1\" s=\"1\" t=\"str\"><f>B1&amp;\"x\"</f><v>ab</v></c>\
                <c r=\"B1\" t=\"inlineStr\"><is><t>x</t></is></c><c r=\"C1\" s=\"1\"/></row>\
                <row r=\"2\"><c r=\"A2\" s=\"1\" t=\"e\"><f t=\"array\" ref=\"A2:B2\">1/0</f>\
                <v>#DIV/0!</v></c><c r=\"B2\" t=\"e\"><v>#DIV/0!</v></c></row>";
    let got = restyled_sheet(FOUR_XFS, rows, |p| {
        p.set_cell_style(SHEET, at("A1"), Some(2));
        p.set_cell_style(SHEET, at("B1"), Some(2));
        p.set_cell_style(SHEET, at("C1"), Some(3));
        p.set_cell_style(SHEET, at("A2"), None);
        p.set_cell_style(SHEET, at("B2"), Some(0));
    });
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\" s=\"2\" t=\"str\"><f>B1&amp;\"x\"</f><v>ab</v></c>\
             <c r=\"B1\" s=\"2\" t=\"inlineStr\"><is><t>x</t></is></c><c r=\"C1\" s=\"3\"/></row>\
             <row r=\"2\"><c r=\"A2\" t=\"e\"><f t=\"array\" ref=\"A2:B2\">1/0</f>\
             <v>#DIV/0!</v></c><c r=\"B2\" s=\"0\" t=\"e\"><v>#DIV/0!</v></c></row>"
        )
    );
}

#[test]
fn a_style_edit_on_a_formula_leaves_the_chain() {
    // No formula is removed, so the chain's three parts are untouched.
    let mut parts = chain_with_formula();
    set(&mut parts, STYLES_PART, style_sheet(FOUR_XFS));
    let output = edited_output(parts.clone(), |p| {
        p.set_cell_style(SHEET, at("A1"), Some(3));
    });
    assert_untouched(&output, &parts, &[SHEET]);
    assert_names(&output, &part_names(&parts));
    assert_part(
        &output,
        SHEET,
        &sheet_with(
            "<row r=\"1\"><c r=\"A1\" s=\"3\"><f>B1*2</f><v>4</v></c><c r=\"B1\"><v>2</v></c></row>",
        ),
    );
}

#[test]
fn a_style_edit_alone_on_an_absent_cell_writes_it_or_nothing() {
    // In an existing row, between two cells; in a new row; `None` on absent
    // cells in both places writes nothing; and a clear with no style edit on
    // an absent cell writes nothing either, in an existing row (E1) and in a
    // row that is not there (A7), which is the arm `write_row` takes for a
    // pending edit with no value and no style (P1 verification ruling 8).
    let rows = "<row r=\"1\"><c r=\"A1\"><v>1</v></c><c r=\"C1\"><v>3</v></c></row>";
    let got = restyled_sheet(FOUR_XFS, rows, |p| {
        p.set_cell_style(SHEET, at("B1"), Some(2));
        p.set_cell_style(SHEET, at("A5"), Some(1));
        p.set_cell_style(SHEET, at("D1"), None);
        p.set_cell_style(SHEET, at("A3"), None);
        p.clear_cell(SHEET, at("E1"));
        p.clear_cell(SHEET, at("A7"));
    });
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\"><v>1</v></c><c r=\"B1\" s=\"2\"/><c r=\"C1\"><v>3</v></c>\
             </row><row r=\"5\"><c r=\"A5\" s=\"1\"/></row>"
        )
    );
}

#[test]
fn the_last_style_edit_wins_and_a_later_value_edit_joins_it() {
    let rows = "<row r=\"1\"><c r=\"A1\" s=\"1\"><v>1</v></c><c r=\"B1\"><v>2</v></c></row>";
    let got = restyled_sheet(FOUR_XFS, rows, |p| {
        p.set_cell_style(SHEET, at("A1"), Some(2));
        p.set_cell_style(SHEET, at("A1"), Some(3));
        p.set_cell_style(SHEET, at("B1"), Some(2));
        p.set_cell(SHEET, at("B1"), Value::Number(5.0));
    });
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\" s=\"3\"><v>1</v></c><c r=\"B1\" s=\"2\"><v>5</v></c></row>"
        )
    );
}

#[test]
fn a_some_edit_needs_a_styles_part_and_a_none_edit_does_not() {
    let mut parts = with_rows("<row r=\"1\"><c r=\"A1\" s=\"1\"><v>1</v></c></row>");
    drop_part(&mut parts, STYLES_PART);
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&worksheet_rel()),
    );
    let (_, mut package) = opened(parts.clone());
    package.set_cell_style(SHEET, at("A1"), Some(0));
    assert_package(refused(package), "the workbook has no styles part");

    let output = edited_output(parts.clone(), |p| p.set_cell_style(SHEET, at("A1"), None));
    assert_untouched(&output, &parts, &[SHEET]);
    assert_part(
        &output,
        SHEET,
        &sheet_with("<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>"),
    );
}

#[test]
fn an_index_at_the_count_plus_additions_is_refused_and_below_it_accepted() {
    let rows = "<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>";
    let (_, mut package) = opened(styled(FOUR_XFS, rows));
    add(&mut package, None, NumberFormat::BuiltIn(14), 4);
    package.set_cell_style(SHEET, at("A1"), Some(5));
    assert_package(refused(package), "a style index is out of range");

    let got = restyled_sheet(FOUR_XFS, rows, |p| {
        add(p, None, NumberFormat::BuiltIn(14), 4);
        p.set_cell_style(SHEET, at("A1"), Some(4));
    });
    assert_eq!(
        got,
        sheet_with("<row r=\"1\"><c r=\"A1\" s=\"4\"><v>1</v></c></row>")
    );
    // Without the addition, 4 is out of range.
    let (_, mut package) = opened(styled(FOUR_XFS, rows));
    package.set_cell_style(SHEET, at("A1"), Some(4));
    assert_package(refused(package), "a style index is out of range");
}

#[test]
fn a_style_edit_or_addition_on_a_replaced_or_removed_part_is_refused() {
    let rows = "<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>";
    let (_, mut package) = opened(styled(FOUR_XFS, rows));
    package.set_cell_style(SHEET, at("A1"), Some(1));
    package.replace_part(SHEET, replacement_from_reader(Cursor::new(Vec::new())));
    assert_package(
        refused(package),
        "a cell edit to a part this write replaces",
    );

    let (_, mut package) = opened(styled(FOUR_XFS, rows));
    add(&mut package, None, NumberFormat::BuiltIn(14), 4);
    package.replace_part(
        STYLES_PART,
        replacement_from_reader(Cursor::new(Vec::new())),
    );
    assert_package(
        refused(package),
        "a style addition to a part this write replaces",
    );

    let (_, mut package) = opened(styled(FOUR_XFS, rows));
    add(&mut package, None, NumberFormat::BuiltIn(14), 4);
    package.remove_part(STYLES_PART);
    assert_package(refused(package), "an edit to a removed part");
}

#[test]
fn the_sample_reads_back_through_styles_and_cells() {
    // `--style-sample`'s edit on a byte-built package: a derived format on
    // an existing number cell, a built-in date on a new one, and every
    // read-back the tool will make.
    let body = "<numFmts count=\"1\"><numFmt numFmtId=\"164\" formatCode=\"0.0\"/></numFmts>\
                <cellXfs count=\"2\"><xf numFmtId=\"0\"/><xf numFmtId=\"164\" fontId=\"0\">\
                <alignment horizontal=\"left\"/></xf></cellXfs>";
    let rows = "<row r=\"1\"><c r=\"A1\" s=\"1\"><v>7</v></c></row>";
    let output = edited_output(styled(body, rows), |p| {
        add(p, Some(1), code("0.000"), 2);
        add(p, None, NumberFormat::BuiltIn(14), 3);
        p.set_cell_style(SHEET, at("A1"), Some(2));
        p.set_cell(SHEET, at("A2"), Value::Number(45000.5));
        p.set_cell_style(SHEET, at("A2"), Some(3));
    });
    let mut reopened = Package::open(Cursor::new(output)).expect("the output opens");
    let styles = reopened.styles().expect("the output's styles read").clone();
    assert_eq!((styles.cell_xfs.len(), styles.num_fmts.len()), (4, 2));
    assert_eq!(styles.number_format(2), Some(code("0.000")));
    assert_eq!(styles.number_format(3), Some(NumberFormat::BuiltIn(14)));
    let cells: Vec<_> = reopened
        .cells(SHEET)
        .expect("the sheet opens")
        .collect::<Result<_, _>>()
        .expect("the sheet reads");
    assert_eq!(cells.len(), 2);
    assert_eq!(
        (
            cells[0].reference,
            cells[0].style,
            cells[0].value.as_deref()
        ),
        (at("A1"), Some(2), Some("7"))
    );
    assert_eq!(
        (cells[1].reference, cells[1].style, cells[1].number()),
        (at("A2"), Some(3), Some(45000.5))
    );
}

// ---------------------------------------------------------------- formulas
//
// `specs/0013-formulas.md`, "P1": a written formula, the formula-change rule
// and the workbook's `calcPr`, on byte-built packages, the output's parts
// compared with bytes each test writes out by hand.

/// The smallest package's `<sheets>`, which most workbook fixtures below keep.
const SHEETS: &str = "<sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets>";

/// A workbook part in the main namespace with `body` between its root tags.
fn workbook_part(body: &str) -> String {
    format!("{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">{body}</workbook>")
}

/// The worksheet after `edit` on a package whose sheet holds `rows`, with the
/// workbook part asserted flagged and every other part untouched: the
/// smallest package has no chain, so a formula write changes those two alone.
#[track_caller]
fn formula_sheet(rows: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let parts = with_rows(rows);
    let output = edited_output(parts.clone(), edit);
    assert_part(&output, WORKBOOK, &flagged_workbook());
    assert_untouched(&output, &parts, &[SHEET, WORKBOOK]);
    String::from_utf8(stored_data(&output, SHEET)).expect("the part is UTF-8")
}

/// The workbook part after [`Package::recalculate_on_open`] alone on a
/// package whose workbook part is `workbook`; every other part is asserted
/// untouched.
#[track_caller]
fn flagged_part(workbook: &str) -> String {
    let mut parts = smallest();
    set(&mut parts, WORKBOOK, workbook.to_string());
    let output = edited_output(parts.clone(), |p| p.recalculate_on_open());
    assert_untouched(&output, &parts, &[WORKBOOK]);
    String::from_utf8(stored_data(&output, WORKBOOK)).expect("the part is UTF-8")
}

/// Every cell of the output's worksheet, read back through `Package::cells`.
#[track_caller]
fn cells_of(output: Vec<u8>) -> Vec<crate::cells::Cell> {
    let mut package = Package::open(Cursor::new(output)).expect("the output opens");
    package
        .cells(SHEET)
        .expect("the sheet")
        .collect::<Result<Vec<_>, _>>()
        .expect("the cells read")
}

#[test]
fn a_formula_is_written_to_an_absent_cell_in_an_existing_row_and_in_a_new_row() {
    let got = formula_sheet("<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>", |p| {
        p.set_cell(SHEET, at("B1"), Value::Formula("A1*2".to_string()));
        p.set_cell(SHEET, at("A3"), Value::Formula("1+1".to_string()));
    });
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\"><v>1</v></c><c r=\"B1\"><f>A1*2</f></c></row>\
             <row r=\"3\"><c r=\"A3\"><f>1+1</f></c></row>"
        )
    );
}

#[test]
fn a_formula_on_an_empty_styled_cell_keeps_the_style() {
    let got = formula_sheet("<row r=\"1\"><c r=\"A1\" s=\"3\"/></row>", |p| {
        p.set_cell(SHEET, at("A1"), Value::Formula("1+1".to_string()))
    });
    assert_eq!(
        got,
        sheet_with("<row r=\"1\"><c r=\"A1\" s=\"3\"><f>1+1</f></c></row>")
    );
}

#[test]
fn a_formula_over_a_value_loses_the_type_and_the_value() {
    // A number, an inline string, a shared string and a boolean.
    let rows = "<row r=\"1\"><c r=\"A1\" s=\"1\"><v>1</v></c>\
                <c r=\"B1\" t=\"inlineStr\"><is><t>x</t></is></c>\
                <c r=\"C1\" t=\"s\"><v>0</v></c><c r=\"D1\" t=\"b\"><v>1</v></c></row>";
    let got = formula_sheet(rows, |p| {
        for cell in ["A1", "B1", "C1", "D1"] {
            p.set_cell(SHEET, at(cell), Value::Formula("1+1".to_string()));
        }
    });
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\" s=\"1\"><f>1+1</f></c><c r=\"B1\"><f>1+1</f></c>\
             <c r=\"C1\"><f>1+1</f></c><c r=\"D1\"><f>1+1</f></c></row>"
        )
    );
}

#[test]
fn a_formula_over_a_normal_formula_loses_its_value_and_ca() {
    let rows = "<row r=\"1\"><c r=\"A1\"><f>1+2</f><v>3</v></c>\
                <c r=\"B1\" t=\"str\"><f ca=\"1\">NOW()</f><v>x</v></c></row>";
    let got = formula_sheet(rows, |p| {
        p.set_cell(SHEET, at("A1"), Value::Formula("2+3".to_string()));
        p.set_cell(SHEET, at("B1"), Value::Formula("4+5".to_string()));
    });
    assert_eq!(
        got,
        sheet_with("<row r=\"1\"><c r=\"A1\"><f>2+3</f></c><c r=\"B1\"><f>4+5</f></c></row>")
    );
}

#[test]
fn a_formula_in_a_shared_group_an_array_or_a_data_table_is_refused() {
    let formula = || Value::Formula("1+1".to_string());
    for cell in ["A1", "B1", "A2", "B2", "B3"] {
        let error = refused_edit(FORMULAS, |p| p.set_cell(SHEET, at(cell), formula()));
        assert_package(
            error,
            "a cell in a shared, array or data table formula cannot be edited",
        );
    }
    let table = "<row r=\"1\"><c r=\"A1\"><f t=\"dataTable\" ref=\"B1:C2\" dt2D=\"0\" dtr=\"0\" \
                 r1=\"Z1\"/><v>1</v></c><c r=\"B1\"><v>2</v></c></row>";
    for cell in ["A1", "B1", "C2"] {
        let error = refused_edit(table, |p| p.set_cell(SHEET, at(cell), formula()));
        assert_package(
            error,
            "a cell in a shared, array or data table formula cannot be edited",
        );
    }
}

#[test]
fn a_formula_text_is_escaped_and_reads_back_equal() {
    // `&`, `<`, `>`, quoted strings with both quote characters, a carriage
    // return and a tab (P1 ruling 2: quotes stay literal).
    let text = "IF(A1<1,\"a&b\",\">\")&\"'q'\"&\"c\r\td\"";
    let parts = with_rows("<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>");
    let output = edited_output(parts.clone(), |p| {
        p.set_cell(SHEET, at("B1"), Value::Formula(text.to_string()))
    });
    assert_part(
        &output,
        SHEET,
        &sheet_with(
            "<row r=\"1\"><c r=\"A1\"><v>1</v></c><c r=\"B1\"><f>IF(A1&lt;1,\"a&amp;b\",\"&gt;\")\
             &amp;\"'q'\"&amp;\"c&#13;\td\"</f></c></row>",
        ),
    );
    assert_untouched(&output, &parts, &[SHEET, WORKBOOK]);
    let cells = cells_of(output);
    let written = &cells[1];
    assert_eq!(written.reference, at("B1"));
    assert_eq!(written.kind, crate::cells::CellKind::Number);
    assert_eq!(written.value, None, "no cached value");
    assert_eq!(
        written.formula,
        Some(crate::cells::Formula {
            text: text.to_string(),
            kind: crate::cells::FormulaKind::Normal,
            reference: None,
        })
    );
}

#[test]
fn an_empty_formula_text_is_refused() {
    let error = refused_edit("<row r=\"1\"></row>", |p| {
        p.set_cell(SHEET, at("A1"), Value::Formula(String::new()))
    });
    assert_package(error, "a formula text is empty");
}

#[test]
fn a_formula_text_starting_with_an_equals_sign_is_refused() {
    for text in ["=1+1", "==1"] {
        let error = refused_edit("<row r=\"1\"></row>", |p| {
            p.set_cell(SHEET, at("A1"), Value::Formula(text.to_string()))
        });
        assert_package(error, "a formula text starts with an equals sign");
    }
    // An equals sign after the first character is the formula's own.
    let got = formula_sheet("<row r=\"1\"></row>", |p| {
        p.set_cell(SHEET, at("A1"), Value::Formula("1=1".to_string()))
    });
    assert_eq!(
        got,
        sheet_with("<row r=\"1\"><c r=\"A1\"><f>1=1</f></c></row>")
    );
}

#[test]
fn a_formula_text_with_a_character_xml_forbids_is_refused() {
    for bad in [
        "\u{1}", "1+\u{8}", "\u{B}", "\u{1F}", "\u{FFFE}", "\u{FFFF}",
    ] {
        let error = refused_edit("<row r=\"1\"></row>", |p| {
            p.set_cell(SHEET, at("A1"), Value::Formula(bad.to_string()))
        });
        assert_package(error, "a formula text holds a character XML cannot carry");
    }
}

#[test]
fn a_formula_overridden_by_a_number_leaves_the_chain_and_calc_pr() {
    // The last value edit for C1 is a number, so the write changes no
    // formula: the chain, its override, its relationship and the workbook
    // part all stay.
    let parts = chain_with_formula();
    let output = edited_output(parts.clone(), |p| {
        p.set_cell(SHEET, at("C1"), Value::Formula("1+1".to_string()));
        p.set_cell(SHEET, at("C1"), Value::Number(3.0));
    });
    assert_part(
        &output,
        SHEET,
        &sheet_with(
            "<row r=\"1\"><c r=\"A1\" s=\"1\"><f>B1*2</f><v>4</v></c><c r=\"B1\"><v>2</v></c>\
             <c r=\"C1\"><v>3</v></c></row>",
        ),
    );
    assert_untouched(&output, &parts, &[SHEET]);
    assert_names(&output, &part_names(&parts));
}

#[test]
fn a_formula_and_a_style_edit_on_one_cell_both_apply() {
    let parts = styled(
        "<cellXfs count=\"2\"><xf/><xf/></cellXfs>",
        "<row r=\"1\"><c r=\"A1\" s=\"0\"><v>1</v></c></row>",
    );
    let output = edited_output(parts.clone(), |p| {
        p.set_cell(SHEET, at("A1"), Value::Formula("1+1".to_string()));
        p.set_cell_style(SHEET, at("A1"), Some(1));
    });
    assert_part(
        &output,
        SHEET,
        &sheet_with("<row r=\"1\"><c r=\"A1\" s=\"1\"><f>1+1</f></c></row>"),
    );
    assert_part(&output, WORKBOOK, &flagged_workbook());
    assert_untouched(&output, &parts, &[SHEET, WORKBOOK]);
}

#[test]
fn a_written_formula_drops_the_chain_and_sets_the_flag() {
    let parts = chain_with_formula();
    let output = edited_output(parts.clone(), |p| {
        p.set_cell(SHEET, at("C1"), Value::Formula("B1+1".to_string()))
    });
    assert_chain_gone(&output);
    assert_part(&output, WORKBOOK, &flagged_workbook());
    assert_part(
        &output,
        SHEET,
        &sheet_with(
            "<row r=\"1\"><c r=\"A1\" s=\"1\"><f>B1*2</f><v>4</v></c><c r=\"B1\"><v>2</v></c>\
             <c r=\"C1\"><f>B1+1</f></c></row>",
        ),
    );
    assert_untouched(
        &output,
        &parts,
        &[
            b"[Content_Types].xml",
            b"xl/_rels/workbook.xml.rels",
            b"xl/calcChain.xml",
            SHEET,
            WORKBOOK,
        ],
    );
    let mut package = Package::open(Cursor::new(output)).expect("the output opens");
    let workbook = package.workbook().expect("the workbook");
    assert!(workbook.calc_chain.is_none());
    assert!(workbook.full_calc_on_load);
}

#[test]
fn the_callers_drop_then_a_written_formula_records_the_recipe_once() {
    let parts = chain_with_formula();
    let (_, mut package) = opened(parts.clone());
    assert!(package.drop_calc_chain().expect("the recipe"));
    package.set_cell(SHEET, at("C1"), Value::Formula("1+1".to_string()));
    package.parts().formula_change_rule().expect("the rule");
    assert_eq!(removals_recorded(&package), (1, 1, 1), "after the rule");
    let output = written(package);
    assert_chain_gone(&output);
    assert_part(&output, WORKBOOK, &flagged_workbook());
    // After `write_to`'s own pass of the rule (P1 verification ruling 3):
    // the input names the chain once in each of the three places and the
    // output in none, so each removal is applied once; a second recording
    // of the recipe fails the write itself.
    let count =
        |data: Vec<u8>, needle: &[u8]| data.windows(needle.len()).filter(|w| *w == needle).count();
    let input = |name: &[u8]| {
        let found = parts.iter().find(|(n, _)| n.as_slice() == name);
        found.expect("the input part").1.clone()
    };
    // The override by its part name, the relationship by its target.
    for (part, needle) in [
        (
            b"[Content_Types].xml".as_slice(),
            b"PartName=\"/xl/calcChain.xml\"".as_slice(),
        ),
        (b"xl/_rels/workbook.xml.rels", b"Target=\"calcChain.xml\""),
    ] {
        let what = String::from_utf8_lossy(part);
        assert_eq!(count(input(part), needle), 1, "the input's {what}");
        assert_eq!(
            count(stored_data(&output, part), needle),
            0,
            "the output's {what}"
        );
    }
    let entries = names(&parse_independently(&output));
    assert_eq!(entries.len(), parts.len() - 1, "one entry removed");
}

#[test]
fn recalculate_on_open_then_a_written_formula_writes_the_attribute_once() {
    let got = formula_sheet("<row r=\"1\"></row>", |p| {
        p.recalculate_on_open();
        p.recalculate_on_open();
        p.set_cell(SHEET, at("A1"), Value::Formula("1+1".to_string()));
    });
    assert_eq!(
        got,
        sheet_with("<row r=\"1\"><c r=\"A1\"><f>1+1</f></c></row>")
    );
    // On a workbook with a chain: the chain dropped, the attribute once.
    let (_, mut package) = opened(chain_with_formula());
    package.recalculate_on_open();
    package.set_cell(SHEET, at("C1"), Value::Formula("1+1".to_string()));
    let output = written(package);
    assert_chain_gone(&output);
    assert_part(&output, WORKBOOK, &flagged_workbook());
}

#[test]
fn recalculate_on_open_without_a_chain_changes_the_workbook_part_alone() {
    let parts = smallest();
    let output = edited_output(parts.clone(), |p| p.recalculate_on_open());
    assert_names(&output, &part_names(&parts));
    assert_part(&output, WORKBOOK, &flagged_workbook());
    assert_untouched(&output, &parts, &[WORKBOOK]);
}

#[test]
fn a_present_calc_pr_gains_or_takes_the_attribute_and_keeps_every_other_byte() {
    // (input tag, output tag): calcId alone, the four LibreOffice
    // attributes, the attribute at 0, at 1 in single quotes (kept), at true,
    // a start tag with an end tag (kept a pair), whitespace before `/>`, and
    // no attributes at all (P1 ruling 5).
    let cases = [
        (
            "<calcPr calcId=\"191029\"/>",
            "<calcPr calcId=\"191029\" fullCalcOnLoad=\"1\"/>",
        ),
        (
            "<calcPr iterateCount=\"100\" refMode=\"A1\" iterate=\"false\" iterateDelta=\"0.001\"/>",
            "<calcPr iterateCount=\"100\" refMode=\"A1\" iterate=\"false\" iterateDelta=\"0.001\" \
             fullCalcOnLoad=\"1\"/>",
        ),
        (
            "<calcPr calcId=\"1\" fullCalcOnLoad=\"0\"/>",
            "<calcPr calcId=\"1\" fullCalcOnLoad=\"1\"/>",
        ),
        (
            "<calcPr fullCalcOnLoad='1' calcId=\"1\"/>",
            "<calcPr fullCalcOnLoad='1' calcId=\"1\"/>",
        ),
        (
            "<calcPr fullCalcOnLoad='true'/>",
            "<calcPr fullCalcOnLoad='1'/>",
        ),
        (
            "<calcPr calcId=\"1\"></calcPr>",
            "<calcPr calcId=\"1\" fullCalcOnLoad=\"1\"></calcPr>",
        ),
        (
            "<calcPr calcId=\"1\" />",
            "<calcPr calcId=\"1\" fullCalcOnLoad=\"1\" />",
        ),
        ("<calcPr/>", "<calcPr fullCalcOnLoad=\"1\"/>"),
    ];
    for (tag, want) in cases {
        let got = flagged_part(&workbook_part(&format!("{SHEETS}{tag}<extLst/>")));
        assert_eq!(
            got,
            workbook_part(&format!("{SHEETS}{want}<extLst/>")),
            "{tag}"
        );
    }
}

#[test]
fn an_absent_calc_pr_goes_after_the_last_of_the_nine() {
    // P1 ruling 1: after `definedNames`, after `sheets` alone, after
    // `workbookPr` alone with no `sheets`, before whitespace and comments
    // that follow, and past another namespace's children between the nine.
    let new = "<calcPr fullCalcOnLoad=\"1\"/>";
    let cases = [
        (
            format!(
                "{SHEETS}\n<definedNames><definedName name=\"a\">1</definedName></definedNames>\n<extLst/>"
            ),
            format!(
                "{SHEETS}\n<definedNames><definedName name=\"a\">1</definedName></definedNames>{new}\n<extLst/>"
            ),
        ),
        (
            format!("<bookViews/>{SHEETS}\n  <oleSize ref=\"A1\"/>"),
            format!("<bookViews/>{SHEETS}{new}\n  <oleSize ref=\"A1\"/>"),
        ),
        (
            "<workbookPr date1904=\"0\"/>\n".to_string(),
            format!("<workbookPr date1904=\"0\"/>{new}\n"),
        ),
        (
            format!("{SHEETS}<!-- c --><definedNames/><!-- d -->"),
            format!("{SHEETS}<!-- c --><definedNames/>{new}<!-- d -->"),
        ),
        (
            format!(
                "<workbookPr/>\n<mc:AlternateContent xmlns:mc=\"urn:mc\"><mc:Choice/></mc:AlternateContent>\n\
                 <bookViews/>{SHEETS}<q:x xmlns:q=\"urn:q\"/>\n<fileRecoveryPr/>"
            ),
            format!(
                "<workbookPr/>\n<mc:AlternateContent xmlns:mc=\"urn:mc\"><mc:Choice/></mc:AlternateContent>\n\
                 <bookViews/>{SHEETS}{new}<q:x xmlns:q=\"urn:q\"/>\n<fileRecoveryPr/>"
            ),
        ),
    ];
    for (body, want) in cases {
        assert_eq!(
            flagged_part(&workbook_part(&body)),
            workbook_part(&want),
            "{body}"
        );
    }
}

#[test]
fn an_absent_calc_pr_in_a_root_with_no_children_goes_inside_it() {
    let new = "<calcPr fullCalcOnLoad=\"1\"/>";
    assert_eq!(flagged_part(&workbook_part("")), workbook_part(new));
    let empty = format!("{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"/>");
    assert_eq!(flagged_part(&empty), workbook_part(new));
}

#[test]
fn a_calc_pr_in_a_prefixed_workbook_takes_the_prefix() {
    let prefixed = |body: &str| {
        format!(
            "{DECLARATION}\n<x:workbook xmlns:x=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
             <x:sheets><x:sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></x:sheets>{body}\
             </x:workbook>"
        )
    };
    assert_eq!(
        flagged_part(&prefixed("")),
        prefixed("<x:calcPr fullCalcOnLoad=\"1\"/>")
    );
    assert_eq!(
        flagged_part(&prefixed("<x:calcPr calcId=\"1\"/>")),
        prefixed("<x:calcPr calcId=\"1\" fullCalcOnLoad=\"1\"/>")
    );
}

#[test]
fn a_workbook_with_two_calc_prs_is_refused_at_write_with_no_output() {
    let two = workbook_part(&format!("{SHEETS}<calcPr calcId=\"1\"/><calcPr/>"));
    let with_two = |rows: &str| {
        let mut parts = with_rows(rows);
        set(&mut parts, WORKBOOK, two.clone());
        parts
    };
    // Reading does not refuse, and the first is the one read.
    let (_, mut package) = opened(with_two(""));
    assert!(!package.workbook().expect("the workbook").full_calc_on_load);
    // The flag recorded, a written formula, and a removed formula (the
    // rule's second validation), each refused before any output.
    let (_, mut package) = opened(with_two(""));
    package.recalculate_on_open();
    assert_package(refused(package), "a workbook has two calcPr elements");
    let (_, mut package) = opened(with_two("<row r=\"1\"></row>"));
    package.set_cell(SHEET, at("A1"), Value::Formula("1+1".to_string()));
    assert_package(refused(package), "a workbook has two calcPr elements");
    let (_, mut package) = opened(with_two(
        "<row r=\"1\"><c r=\"A1\"><f>1</f><v>1</v></c></row>",
    ));
    package.clear_cell(SHEET, at("A1"));
    assert_package(refused(package), "a workbook has two calcPr elements");
    // Validation refuses before any part is rewritten: a formula in a shared
    // group, which the worksheet rewrite would refuse, gets the count's
    // message, not the worksheet's (the backstop alone would give the other).
    let (_, mut package) = opened(with_two(FORMULAS));
    package.set_cell(SHEET, at("B1"), Value::Formula("1+1".to_string()));
    assert_package(refused(package), "a workbook has two calcPr elements");
    // A write that changes no formula does not read the count.
    let (_, mut package) = opened(with_two("<row r=\"1\"></row>"));
    package.set_cell(SHEET, at("A1"), Value::Number(1.0));
    written(package);
}

#[test]
fn the_calc_pr_rewrite_refuses_a_second_calc_pr_itself() {
    // The backstop of P1 ruling 3, called directly: validation refuses first
    // on every path through `write_to`.
    let mut parts = smallest();
    set(
        &mut parts,
        WORKBOOK,
        workbook_part(&format!("{SHEETS}<calcPr/><calcPr/>")),
    );
    let (_, mut package) = opened(parts);
    let index = package.part(WORKBOOK).expect("the workbook part");
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    match rewrite_workbook(&mut package.archive.core, index, Some(true), None) {
        Ok(_) => panic!("the rewrite took two calcPr elements"),
        Err(error) => assert_package(error, "a workbook has two calcPr elements"),
    }
}

#[test]
fn every_byte_outside_the_calc_pr_tag_is_kept() {
    let body = |calc_pr: &str| {
        format!(
            "<fileVersion appName=\"xl\"/>\r\n<workbookPr date1904=\"1\"/>\n\
             <bookViews><workbookView xWindow=\"0\"/></bookViews>{SHEETS}\
             <definedNames><definedName name=\"a\">Sheet1!$A$1</definedName></definedNames>\t\
             {calc_pr}  <pivotCaches><pivotCache cacheId=\"1\" r:id=\"rId9\"/></pivotCaches>\
             <extLst><ext uri=\"{{x}}\"><y/></ext></extLst>"
        )
    };
    let with_mark = |calc_pr: &str| {
        let mut bytes = String::from("\u{FEFF}");
        bytes.push_str(&workbook_part(&body(calc_pr)));
        bytes
    };
    assert_eq!(
        flagged_part(&with_mark("<calcPr calcId=\"191029\"/>")),
        with_mark("<calcPr calcId=\"191029\" fullCalcOnLoad=\"1\"/>")
    );
}

#[test]
fn full_calc_on_load_is_read_from_the_first_calc_pr() {
    let read = |tags: &str| {
        let mut parts = smallest();
        set(
            &mut parts,
            WORKBOOK,
            workbook_part(&format!("{SHEETS}{tags}")),
        );
        let (_, mut package) = opened(parts);
        package.workbook().expect("the workbook").full_calc_on_load
    };
    assert!(read("<calcPr fullCalcOnLoad=\"1\"/>"));
    assert!(read("<calcPr fullCalcOnLoad=\"true\"/>"));
    assert!(!read("<calcPr fullCalcOnLoad=\"0\"/>"));
    assert!(!read("<calcPr fullCalcOnLoad=\"false\"/>"));
    assert!(!read("<calcPr calcId=\"1\"/>"));
    assert!(!read(""));
    assert!(!read("<calcPr/><calcPr fullCalcOnLoad=\"1\"/>"));
    assert!(read("<calcPr fullCalcOnLoad=\"1\"/><calcPr/>"));
}

#[test]
fn a_workbook_part_replaced_or_removed_refuses_the_flag() {
    let message = "the workbook part cannot be edited in a package that replaces or removes it";
    let replacement = || replacement_from_reader(Cursor::new(workbook_part(SHEETS).into_bytes()));
    let (_, mut package) = opened(smallest());
    package.replace_part(WORKBOOK, replacement());
    package.recalculate_on_open();
    assert_package(refused(package), message);
    let (_, mut package) = opened(smallest());
    package.remove_part(WORKBOOK);
    package.recalculate_on_open();
    assert_package(refused(package), message);
    // The rule's own case: a written formula.
    let (_, mut package) = opened(with_rows("<row r=\"1\"></row>"));
    package.replace_part(WORKBOOK, replacement());
    package.set_cell(SHEET, at("A1"), Value::Formula("1+1".to_string()));
    assert_package(refused(package), message);
    // The control: the replacement alone is written.
    let (_, mut package) = opened(smallest());
    package.replace_part(WORKBOOK, replacement());
    written(package);
}

// ------------------------------------------------------------------ the keep

/// A fresh directory for a keep test, removed first if a failed run left one.
fn keep_test_dir(label: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "emend-keep-test-{}-package-{label}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the directory");
    dir
}

#[test]
fn written_keeps_the_fixture_and_the_output() {
    let dir = keep_test_dir("kept");
    override_keep(Some(dir.clone()));
    let (_, mut package) = opened(smallest());
    package.set_cell(SHEET, at("A1"), Value::Number(1.0));
    let fixture = Build::new(
        smallest()
            .iter()
            .map(|(name, data)| Spec::new(name, data))
            .collect(),
    )
    .bytes();
    let output = written(package);
    let test = "package.tests.written_keeps_the_fixture_and_the_output";
    let read = |suffix: &str| std::fs::read(dir.join(format!("{test}.1.{suffix}"))).expect("read");
    assert_eq!(read("in.xlsx"), fixture);
    assert_eq!(read("out.xlsx"), output);
    std::fs::remove_dir_all(&dir).expect("remove the directory");
}

#[test]
fn written_with_a_keep_refuses_a_package_whose_fixture_was_not_recorded() {
    let dir = keep_test_dir("unrecorded");
    override_keep(Some(dir.clone()));
    let bytes = Build::new(
        smallest()
            .iter()
            .map(|(name, data)| Spec::new(name, data))
            .collect(),
    )
    .bytes();
    let package = Package::open(Cursor::new(bytes)).expect("the fixture opens");
    let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| written(package)));
    let message = *refused
        .expect_err("the keep refused")
        .downcast::<String>()
        .expect("a formatted message");
    assert!(
        message.contains("the keep has no fixture: open it with open_fixture"),
        "the message: {message}"
    );
    assert_eq!(std::fs::read_dir(&dir).expect("list").count(), 0);
    std::fs::remove_dir_all(&dir).expect("remove the directory");
}

#[test]
fn written_with_a_keep_refuses_a_fixture_the_package_was_not_opened_from() {
    let dir = keep_test_dir("stale");
    override_keep(Some(dir.clone()));
    // The recorded fixture is the smallest package; the one written was
    // opened from other bytes, with a worksheet that differs.
    let (_, _) = opened(smallest());
    let other = Build::new(
        with_rows("<row r=\"1\"></row>")
            .iter()
            .map(|(name, data)| Spec::new(name, data))
            .collect(),
    )
    .bytes();
    let package = Package::open(Cursor::new(other)).expect("the other opens");
    let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| written(package)));
    let message = *refused
        .expect_err("the keep refused")
        .downcast::<String>()
        .expect("a formatted message");
    assert!(
        message.contains("the keep's fixture is not the one this package was opened from"),
        "the message: {message}"
    );
    assert_eq!(std::fs::read_dir(&dir).expect("list").count(), 0);
    std::fs::remove_dir_all(&dir).expect("remove the directory");
}

#[test]
fn written_with_no_keep_needs_no_fixture() {
    // The gate's case: nothing recorded, nothing named, the write as before.
    override_keep(None);
    let bytes = Build::new(
        smallest()
            .iter()
            .map(|(name, data)| Spec::new(name, data))
            .collect(),
    )
    .bytes();
    let package = Package::open(Cursor::new(bytes.clone())).expect("the fixture opens");
    assert_eq!(written(package), bytes);
}

// `Package::new` (`specs/0016-new-workbook-and-sheets.md`, "P2: the
// template and `Package::new`"): the template written and opened back. Each
// call writes the template's additions through the archive layer, which makes
// temporaries of the one form, so every call here holds `TEMPORARY_LOCK`.

/// A new package, made under [`TEMPORARY_LOCK`].
#[track_caller]
fn new_package() -> Package<Cursor<Vec<u8>>> {
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    Package::new().expect("Package::new")
}

/// The bytes a new package holds, read as a write with no edits, which is a
/// verbatim round trip, under [`TEMPORARY_LOCK`] and never kept.
#[track_caller]
fn new_bytes() -> Vec<u8> {
    let package = new_package();
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut out = Vec::new();
    package.write_to(&mut out).expect("write_to");
    out
}

/// A new package with its bytes recorded as this thread's fixture, so
/// [`written`] keeps the pair (`specs/0016`, "Harness runs": each phase's
/// run log opens its keep).
#[track_caller]
fn new_recorded() -> Package<Cursor<Vec<u8>>> {
    record_fixture(&new_bytes());
    new_package()
}

/// The template's file for an entry, by name.
fn template_file(name: &[u8]) -> &'static [u8] {
    TEMPLATE
        .iter()
        .find(|(entry, _)| *entry == name)
        .map(|(_, bytes)| *bytes)
        .expect("a template entry")
}

#[test]
fn new_twice_gives_the_same_bytes() {
    let first = new_bytes();
    let second = new_bytes();
    assert_eq!(first, second);
    assert!(!first.is_empty());
}

#[test]
fn new_holds_the_six_template_files_in_order_byte_for_byte() {
    let bytes = new_bytes();
    let parsed = parse_independently(&bytes);
    let want: Vec<&[u8]> = vec![
        b"[Content_Types].xml",
        b"_rels/.rels",
        b"xl/workbook.xml",
        b"xl/_rels/workbook.xml.rels",
        b"xl/worksheets/sheet1.xml",
        b"xl/styles.xml",
    ];
    assert_names(&bytes, &want);
    // The independent reader's CRC-32 and size of each entry are the
    // template file's, computed here without the layer.
    for entry in &parsed.entries {
        let file = template_file(&entry.name);
        assert_eq!(entry.crc32, crc32(file), "the CRC-32 of {:?}", entry.name);
        assert_eq!(entry.uncompressed_size, file.len() as u64);
        assert_eq!(entry.method, METHOD_DEFLATE);
        assert_eq!(entry.gp_flags, 0);
    }
    // And the layer's own read gives the file's bytes back.
    let mut package = new_package();
    for (name, file) in TEMPLATE {
        let index = package.part(name).expect("the entry is there");
        let mut read = Vec::new();
        package
            .read_part(index)
            .expect("read_part")
            .read_to_end(&mut read)
            .expect("read");
        assert_eq!(read, file, "the bytes of {name:?}");
    }
    // The template files are under the 4 KB the spec's "32-bit" section
    // states, and each is its declaration, one line feed and the root, with
    // no line feed at the end (P2 ruling 1).
    let total: usize = TEMPLATE.iter().map(|(_, bytes)| bytes.len()).sum();
    assert!(total < 4096, "the template is {total} bytes");
    for (name, file) in TEMPLATE {
        assert!(
            file.starts_with(b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<"),
            "the declaration of {name:?}"
        );
        assert_eq!(file.last(), Some(&b'>'), "the end of {name:?}");
        assert_eq!(file.iter().filter(|b| **b == b'\n').count(), 1);
        assert!(!file.contains(&b'\r'));
    }
}

#[test]
fn new_is_the_empty_archive_with_the_template_added() {
    // The bytes new() opens are the ones its recipe writes: the constant
    // opened, the six additions, one write. Built here from the same private
    // constant and table, so the check is that new() adds nothing else.
    assert_eq!(EMPTY_ARCHIVE.len(), 22);
    let recipe = {
        let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut template =
            Package::open(Cursor::new(EMPTY_ARCHIVE.to_vec())).expect("the empty archive opens");
        assert_eq!(template.archive().entries().len(), 0);
        for (name, bytes) in TEMPLATE {
            template.add_part(name, replacement_from_reader(bytes));
        }
        let mut out = Vec::new();
        template.write_to(&mut out).expect("write_to");
        out
    };
    // A write of new() with no edits is byte for byte those bytes.
    assert_eq!(new_bytes(), recipe);
    // The end record: no comment, six entries.
    let parsed = parse_independently(&recipe);
    assert_eq!(parsed.entries.len(), 6);
    assert_eq!(&recipe[recipe.len() - 2..], &[0, 0]);
}

#[test]
fn new_opens_back_as_one_empty_sheet_with_one_cell_format() {
    let mut package = new_package();
    let workbook = package.workbook().expect("workbook").clone();
    assert_eq!(workbook.part, b"xl/workbook.xml");
    assert_eq!(workbook.sheets.len(), 1);
    let sheet = &workbook.sheets[0];
    assert_eq!(sheet.name, "Sheet1");
    assert_eq!(sheet.sheet_id, 1);
    assert_eq!(sheet.rel_id, b"rId1");
    assert_eq!(sheet.state, None);
    assert_eq!(sheet.part, b"xl/worksheets/sheet1.xml");
    assert_eq!(sheet.kind, SheetKind::Worksheet);
    assert_eq!(workbook.shared_strings, None);
    assert_eq!(workbook.styles.as_deref(), Some(&b"xl/styles.xml"[..]));
    assert_eq!(workbook.calc_chain, None);
    assert_eq!(workbook.theme, None);
    assert!(!workbook.date1904);
    assert!(!workbook.full_calc_on_load);
    let styles = package.styles().expect("styles");
    assert_eq!(styles.cell_xfs.len(), 1);
    assert_eq!(styles.num_fmts.len(), 0);
    assert_eq!(styles.number_format(0), Some(NumberFormat::BuiltIn(0)));
    let cells: Vec<_> = package
        .cells(b"xl/worksheets/sheet1.xml")
        .expect("cells")
        .collect();
    assert!(cells.is_empty());
    assert_eq!(
        package
            .content_types()
            .expect("content types")
            .overrides()
            .len(),
        3
    );
}

#[test]
fn new_takes_a_number_a_text_a_boolean_and_a_formula() {
    let part: &[u8] = b"xl/worksheets/sheet1.xml";
    let mut package = new_recorded();
    let at = |row, col| CellRef { row, col };
    package.set_cell(part, at(1, 1), Value::Number(7.0));
    package.set_cell(part, at(1, 2), Value::Text("emend".to_string()));
    package.set_cell(part, at(1, 3), Value::Boolean(true));
    package.set_cell(part, at(2, 1), Value::Formula("A1*2".to_string()));
    let out = written(package);
    assert_names(&out, &part_names(&template_parts()));
    let mut back = Package::open(Cursor::new(out)).expect("the output opens");
    let cells: Vec<_> = back
        .cells(part)
        .expect("cells")
        .collect::<Result<_, _>>()
        .expect("every cell reads");
    assert_eq!(cells.len(), 4);
    assert_eq!(cells[0].reference, at(1, 1));
    assert_eq!(cells[0].number(), Some(7.0));
    assert_eq!(cells[1].reference, at(1, 2));
    assert_eq!(cells[1].kind, crate::cells::CellKind::InlineString);
    assert_eq!(cells[1].value.as_deref(), Some("emend"));
    assert_eq!(cells[2].reference, at(1, 3));
    assert_eq!(cells[2].boolean(), Some(true));
    assert_eq!(cells[3].reference, at(2, 1));
    assert_eq!(
        cells[3].formula.as_ref().map(|f| f.text.as_str()),
        Some("A1*2")
    );
    assert_eq!(cells[3].value, None);
    // The formula set the flag: the workbook part gains `calcPr` right after
    // `sheets`, the one change to its bytes, and no chain was there to drop.
    let workbook = back.workbook().expect("workbook").clone();
    assert!(workbook.full_calc_on_load);
    assert_eq!(workbook.calc_chain, None);
    let index = back.part(b"xl/workbook.xml").expect("the workbook part");
    let mut bytes = Vec::new();
    back.read_part(index)
        .expect("read_part")
        .read_to_end(&mut bytes)
        .expect("read");
    let want = String::from_utf8(template_file(b"xl/workbook.xml").to_vec())
        .expect("UTF-8")
        .replace("</sheets>", "</sheets><calcPr fullCalcOnLoad=\"1\"/>");
    assert_eq!(String::from_utf8(bytes).expect("UTF-8"), want);
}

#[test]
fn new_takes_a_cell_style_over_its_one_format() {
    let part: &[u8] = b"xl/worksheets/sheet1.xml";
    let mut package = new_recorded();
    let at = CellRef { row: 1, col: 1 };
    let dated = package
        .add_cell_style(Some(0), NumberFormat::BuiltIn(14))
        .expect("add_cell_style");
    assert_eq!(dated, 1);
    package.set_cell(part, at, Value::Number(45000.5));
    package.set_cell_style(part, at, Some(dated));
    let out = written(package);
    let mut back = Package::open(Cursor::new(out)).expect("the output opens");
    let styles = back.styles().expect("styles");
    assert_eq!(styles.cell_xfs.len(), 2);
    assert_eq!(styles.number_format(1), Some(NumberFormat::BuiltIn(14)));
    let xf = &styles.cell_xfs[1];
    assert_eq!(
        (xf.font_id, xf.fill_id, xf.border_id, xf.xf_id),
        (Some(0), Some(0), Some(0), Some(0))
    );
    let cells: Vec<_> = back
        .cells(part)
        .expect("cells")
        .collect::<Result<_, _>>()
        .expect("every cell reads");
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].number(), Some(45000.5));
    assert_eq!(cells[0].style, Some(1));
}

#[test]
fn new_written_with_no_edits_is_its_own_bytes_and_is_kept() {
    let bytes = new_bytes();
    let package = new_recorded();
    assert_eq!(written(package), bytes);
}

/// The template's parts as `(name, bytes)`, for [`part_names`].
fn template_parts() -> Vec<(Vec<u8>, Vec<u8>)> {
    TEMPLATE
        .iter()
        .map(|(name, bytes)| (name.to_vec(), bytes.to_vec()))
        .collect()
}

// The sheet list (`specs/0016-new-workbook-and-sheets.md`, "P3: `sheets`,
// `add_sheet`, `rename_sheet`, `move_sheet`", with the planning rulings
// during P3): on byte-built packages, each output part compared with bytes
// the test builds by hand (`specs/0011-editing-cells.md`, P1 ruling 5).

/// A package of `n` worksheets named `S1` to `Sn`, `sheetId` 1 to n and
/// relationships `rId1` to `rIdn`, the styles relationship after them; the
/// workbook's root children between `<workbook ...>` and `<sheets>` are
/// `before`, and those after `</sheets>` are `after`.
fn sheeted(n: usize, before: &str, after: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut overrides =
        format!("<Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>");
    let mut sheets = String::new();
    let mut rels_body = String::new();
    for i in 1..=n {
        overrides.push_str(&format!(
            "<Override PartName=\"/xl/worksheets/sheet{i}.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>"
        ));
        sheets.push_str(&format!(
            "<sheet name=\"S{i}\" sheetId=\"{i}\" r:id=\"rId{i}\"/>"
        ));
        rels_body.push_str(&relationship(
            &format!("rId{i}"),
            &format!("{ATTR_NS}/worksheet"),
            &format!("worksheets/sheet{i}.xml"),
        ));
    }
    rels_body.push_str(&relationship(
        &format!("rId{}", n + 1),
        &format!("{ATTR_NS}/styles"),
        "styles.xml",
    ));
    let mut parts = vec![
        (
            b"[Content_Types].xml".to_vec(),
            types(&format!(
                "<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
                 <Default Extension=\"xml\" ContentType=\"application/xml\"/>{overrides}"
            ))
            .into_bytes(),
        ),
        (
            b"_rels/.rels".to_vec(),
            rels(&relationship(
                "rId1",
                &format!("{ATTR_NS}/officeDocument"),
                "xl/workbook.xml",
            ))
            .into_bytes(),
        ),
        (
            b"xl/workbook.xml".to_vec(),
            format!(
                "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">{before}\
                 <sheets>{sheets}</sheets>{after}</workbook>"
            )
            .into_bytes(),
        ),
        (b"xl/_rels/workbook.xml.rels".to_vec(), rels(&rels_body).into_bytes()),
    ];
    for i in 1..=n {
        parts.push((
            format!("xl/worksheets/sheet{i}.xml").into_bytes(),
            format!("{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"><sheetData/></worksheet>")
                .into_bytes(),
        ));
    }
    parts.push((b"xl/styles.xml".to_vec(), b"<styleSheet/>".to_vec()));
    parts
}

/// The bytes of a part in `parts`, as text.
fn part_text(parts: &[(Vec<u8>, Vec<u8>)], name: &[u8]) -> String {
    let (_, bytes) = parts
        .iter()
        .find(|(part, _)| part == name)
        .expect("the fixture has that part");
    String::from_utf8(bytes.clone()).expect("UTF-8")
}

/// The names of the projected list.
fn sheet_names(package: &mut Package<Cursor<Vec<u8>>>) -> Vec<String> {
    package
        .sheets()
        .expect("sheets")
        .into_iter()
        .map(|sheet| sheet.name)
        .collect()
}

#[test]
fn adding_a_sheet_records_its_entry_override_relationship_and_sheet() {
    let (parts, mut package) = opened(smallest());
    let part = package.add_sheet("Two").expect("add_sheet");
    assert_eq!(part, b"xl/worksheets/sheet2.xml");
    let added = package.sheets().expect("sheets");
    assert_eq!(added.len(), 2);
    assert_eq!(
        added[1],
        Sheet {
            name: "Two".to_string(),
            sheet_id: 2,
            rel_id: b"rId3".to_vec(),
            state: None,
            part: b"xl/worksheets/sheet2.xml".to_vec(),
            kind: SheetKind::Worksheet,
        }
    );
    // The file's list is still the file's.
    assert_eq!(package.workbook().expect("workbook").sheets.len(), 1);
    let out = written(package);
    let mut names = part_names(&parts);
    names.push(b"xl/worksheets/sheet2.xml");
    assert_names(&out, &names);
    assert_untouched(
        &out,
        &parts,
        &[
            b"[Content_Types].xml",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
        ],
    );
    assert_added_part(&out, b"xl/worksheets/sheet2.xml", BLANK_WORKSHEET);
    assert_part(
        &out,
        b"[Content_Types].xml",
        &part_text(&parts, b"[Content_Types].xml").replace(
            "</Types>",
            &format!(
                "<Override PartName=\"/xl/worksheets/sheet2.xml\" ContentType=\"{WORKSHEET_TYPE}\"/></Types>"
            ),
        ),
    );
    assert_part(
        &out,
        b"xl/_rels/workbook.xml.rels",
        &rels(&format!(
            "{}{}{}",
            worksheet_rel(),
            styles_rel(),
            relationship(
                "rId3",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet2.xml"
            )
        )),
    );
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace(
            "</sheets>",
            "<sheet name=\"Two\" sheetId=\"2\" r:id=\"rId3\"/></sheets>",
        ),
    );
}

#[test]
fn an_added_sheet_skips_an_entry_name_that_is_taken() {
    // `sheet2.xml` is an entry but not a sheet: the new part is sheet3.
    let mut parts = smallest();
    parts.push((b"xl/worksheets/sheet2.xml".to_vec(), b"<x/>".to_vec()));
    let (_, mut package) = opened(parts);
    assert_eq!(
        package.add_sheet("Two").expect("add_sheet"),
        b"xl/worksheets/sheet3.xml"
    );
    // And a second add skips the first one's pending name.
    assert_eq!(
        package.add_sheet("Three").expect("add_sheet"),
        b"xl/worksheets/sheet4.xml"
    );
}

#[test]
fn a_default_for_xml_of_the_worksheet_type_needs_no_override() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"[Content_Types].xml",
        types(&format!(
            "<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
             <Default Extension=\"xml\" ContentType=\"{WORKSHEET_TYPE}\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>"
        )),
    );
    let (parts, mut package) = opened(parts);
    package.add_sheet("Two").expect("add_sheet");
    let out = written(package);
    assert_part(
        &out,
        b"[Content_Types].xml",
        &part_text(&parts, b"[Content_Types].xml"),
    );
}

#[test]
fn a_strict_workbook_gets_a_strict_worksheet_relationship() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_STRICT}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            styles_rel()
        )),
    );
    let (_, mut package) = opened(parts);
    package.add_sheet("Two").expect("add_sheet");
    let out = written(package);
    assert_part(
        &out,
        b"xl/_rels/workbook.xml.rels",
        &rels(&format!(
            "{}{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_STRICT}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            styles_rel(),
            relationship(
                "rId3",
                &format!("{ATTR_STRICT}/worksheet"),
                "worksheets/sheet2.xml"
            )
        )),
    );
}

#[test]
fn a_new_sheet_id_is_one_more_than_the_largest_and_two_adds_differ() {
    let mut parts = sheeted(2, "", "");
    let workbook = part_text(&parts, b"xl/workbook.xml").replace("sheetId=\"2\"", "sheetId=\"9\"");
    set(&mut parts, b"xl/workbook.xml", workbook);
    let (parts, mut package) = opened(parts);
    let first = package.add_sheet("A").expect("add_sheet");
    let second = package.add_sheet("B").expect("add_sheet");
    let list = package.sheets().expect("sheets");
    assert_eq!(
        list.iter()
            .map(|sheet| (sheet.sheet_id, sheet.rel_id.clone(), sheet.part.clone()))
            .collect::<Vec<_>>()[2..],
        [
            (10, b"rId4".to_vec(), first),
            (11, b"rId5".to_vec(), second),
        ]
    );
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace(
            "</sheets>",
            "<sheet name=\"A\" sheetId=\"10\" r:id=\"rId4\"/>\
             <sheet name=\"B\" sheetId=\"11\" r:id=\"rId5\"/></sheets>",
        ),
    );
}

#[test]
fn a_sheet_name_breaking_a_rule_is_refused_and_records_nothing() {
    let (_, mut package) = opened(sheeted(2, "", ""));
    let long = "x".repeat(32);
    let cases: Vec<(&str, &str)> = vec![
        ("", "a sheet name is empty"),
        (long.as_str(), "a sheet name is longer than 31 characters"),
        ("a:b", "a sheet name holds a character Excel refuses"),
        ("a\\b", "a sheet name holds a character Excel refuses"),
        ("a/b", "a sheet name holds a character Excel refuses"),
        ("a?b", "a sheet name holds a character Excel refuses"),
        ("a*b", "a sheet name holds a character Excel refuses"),
        ("a[b", "a sheet name holds a character Excel refuses"),
        ("a]b", "a sheet name holds a character Excel refuses"),
        ("'ab", "a sheet name starts or ends with an apostrophe"),
        ("ab'", "a sheet name starts or ends with an apostrophe"),
        ("a\u{1}b", "a text holds a character XML cannot carry"),
        ("History", "a sheet name is the reserved name History"),
        ("hISTORY", "a sheet name is the reserved name History"),
        ("S2", "a sheet name is already in use"),
        ("s2", "a sheet name is already in use"),
    ];
    for (name, message) in &cases {
        assert_package(package.add_sheet(name).expect_err(name), message);
        assert_package(package.rename_sheet("S1", name).expect_err(name), message);
    }
    assert_eq!(sheet_names(&mut package), ["S1", "S2"]);
    // Nothing recorded: the write is a verbatim round trip.
    let bytes = Build::new(
        sheeted(2, "", "")
            .iter()
            .map(|(name, data)| Spec::new(name, data))
            .collect(),
    )
    .bytes();
    assert_eq!(written(package), bytes);
}

#[test]
fn a_name_of_31_multi_byte_characters_is_accepted() {
    let (_, mut package) = opened(smallest());
    let name = "\u{e9}".repeat(31);
    assert_eq!(name.len(), 62);
    package.add_sheet(&name).expect("31 characters, 62 bytes");
    let name32 = "\u{e9}".repeat(32);
    assert_package(
        package.add_sheet(&name32).expect_err("32 characters"),
        "a sheet name is longer than 31 characters",
    );
}

#[test]
fn a_rename_keeps_the_element_outside_its_name_and_escapes_it() {
    let mut parts = smallest();
    let workbook = part_text(&parts, b"xl/workbook.xml").replace(
        "<sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/>",
        "<sheet  name = 'Sheet1' sheetId=\"1\"\n state=\"visible\" r:id=\"rId1\" />",
    );
    set(&mut parts, b"xl/workbook.xml", workbook);
    let (parts, mut package) = opened(parts);
    package
        .rename_sheet("Sheet1", "A&B<\"C\"\t'd>\n\re")
        .expect("rename_sheet");
    assert_eq!(sheet_names(&mut package), ["A&B<\"C\"\t'd>\n\re"]);
    let out = written(package);
    let want = part_text(&parts, b"xl/workbook.xml").replace(
        "name = 'Sheet1'",
        "name = 'A&amp;B&lt;&quot;C&quot;&#9;&apos;d&gt;&#10;&#13;e'",
    );
    assert_part(&out, b"xl/workbook.xml", &want);
    // Read back, the name is the one given.
    let mut back = Package::open(Cursor::new(out)).expect("opens");
    assert_eq!(
        back.workbook().expect("workbook").sheets[0].name,
        "A&B<\"C\"\t'd>\n\re"
    );
}

#[test]
fn a_rename_to_its_own_name_in_another_case_is_allowed() {
    let (parts, mut package) = opened(sheeted(2, "", ""));
    package
        .rename_sheet("S2", "s2")
        .expect("own name, other case");
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace("name=\"S2\"", "name=\"s2\""),
    );
}

#[test]
fn a_sheet_that_is_not_there_is_refused() {
    let (_, mut package) = opened(sheeted(2, "", ""));
    assert_package(
        package.rename_sheet("S9", "X").expect_err("rename"),
        "no sheet of that name",
    );
    assert_package(
        package
            .move_sheet("s1", 0)
            .expect_err("names are matched exactly"),
        "no sheet of that name",
    );
    assert_package(
        package.remove_sheet("S9").expect_err("remove"),
        "no sheet of that name",
    );
}

#[test]
fn moves_to_the_front_to_the_end_to_its_own_place_and_past_the_end() {
    let (parts, mut package) = opened(sheeted(3, "", ""));
    package.move_sheet("S3", 0).expect("to the front");
    assert_eq!(sheet_names(&mut package), ["S3", "S1", "S2"]);
    package.move_sheet("S3", 2).expect("to the end");
    assert_eq!(sheet_names(&mut package), ["S1", "S2", "S3"]);
    package.move_sheet("S2", 1).expect("its own place");
    assert_package(
        package.move_sheet("S1", 3).expect_err("past the end"),
        "a sheet position is past the end",
    );
    // Back where it started: the sheets are rewritten as they were.
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml"),
    );
}

#[test]
fn a_move_to_its_own_place_records_nothing() {
    let (parts, mut package) = opened(sheeted(2, "", ""));
    package.move_sheet("S1", 0).expect("its own place");
    let out = written(package);
    assert_untouched(&out, &parts, &[]);
}

#[test]
fn a_move_remaps_local_sheet_ids_and_the_active_tab_and_keeps_first_sheet() {
    let views = "<bookViews><workbookView activeTab=\"2\" firstSheet=\"1\"/>\
                 <workbookView/></bookViews>";
    let names = "<definedNames>\n<definedName name=\"a\" localSheetId=\"0\">S1!A1</definedName>\n\
                 <definedName name=\"b\">S2!A1</definedName>\n\
                 <definedName localSheetId='2' name=\"c\">S3!A1</definedName></definedNames>";
    let (parts, mut package) = opened(sheeted(3, views, names));
    package.move_sheet("S3", 0).expect("move");
    let out = written(package);
    let want = part_text(&parts, b"xl/workbook.xml")
        .replace(
            "<sheet name=\"S1\" sheetId=\"1\" r:id=\"rId1\"/><sheet name=\"S2\" sheetId=\"2\" r:id=\"rId2\"/>\
             <sheet name=\"S3\" sheetId=\"3\" r:id=\"rId3\"/>",
            "<sheet name=\"S3\" sheetId=\"3\" r:id=\"rId3\"/><sheet name=\"S1\" sheetId=\"1\" r:id=\"rId1\"/>\
             <sheet name=\"S2\" sheetId=\"2\" r:id=\"rId2\"/>",
        )
        // S3 was the active tab, index 2, and is now 0; firstSheet stays (P3
        // ruling 8); the second view's absent activeTab read 0, which was S1,
        // now 1, so it is appended.
        .replace(
            "<workbookView activeTab=\"2\" firstSheet=\"1\"/><workbookView/>",
            "<workbookView activeTab=\"0\" firstSheet=\"1\"/><workbookView activeTab=\"1\"/>",
        )
        .replace("localSheetId=\"0\"", "localSheetId=\"1\"")
        .replace("localSheetId='2'", "localSheetId='0'");
    assert_part(&out, b"xl/workbook.xml", &want);
}

#[test]
fn a_local_sheet_id_past_the_count_is_copied_unchanged() {
    // P3 rulings 5 and 10: an index past the file's sheets names no sheet,
    // so a move copies it as it is; LibreOffice refuses such a file, input
    // and output alike.
    let names = "<definedNames><definedName name=\"a\" localSheetId=\"0\">S1!A1</definedName>\
                 <definedName name=\"d\" localSheetId=\"7\"/></definedNames>";
    let (parts, mut package) = opened(sheeted(3, "", names));
    package.move_sheet("S3", 0).expect("move");
    let out = written(package);
    let want = part_text(&parts, b"xl/workbook.xml")
        .replace(
            "<sheet name=\"S1\" sheetId=\"1\" r:id=\"rId1\"/><sheet name=\"S2\" sheetId=\"2\" r:id=\"rId2\"/>\
             <sheet name=\"S3\" sheetId=\"3\" r:id=\"rId3\"/>",
            "<sheet name=\"S3\" sheetId=\"3\" r:id=\"rId3\"/><sheet name=\"S1\" sheetId=\"1\" r:id=\"rId1\"/>\
             <sheet name=\"S2\" sheetId=\"2\" r:id=\"rId2\"/>",
        )
        .replace("localSheetId=\"0\"", "localSheetId=\"1\"");
    assert_part(&out, b"xl/workbook.xml", &want);
}

#[test]
fn a_move_leaves_a_first_sheet_past_the_end_unchanged() {
    // P3 ruling 10: firstSheet is clamped only when a sheet was removed.
    let views = "<bookViews><workbookView firstSheet=\"9\"/></bookViews>";
    let (parts, mut package) = opened(sheeted(3, views, ""));
    package.move_sheet("S3", 0).expect("move");
    let out = written(package);
    let want = part_text(&parts, b"xl/workbook.xml")
        .replace(
            "<sheet name=\"S1\" sheetId=\"1\" r:id=\"rId1\"/><sheet name=\"S2\" sheetId=\"2\" r:id=\"rId2\"/>\
             <sheet name=\"S3\" sheetId=\"3\" r:id=\"rId3\"/>",
            "<sheet name=\"S3\" sheetId=\"3\" r:id=\"rId3\"/><sheet name=\"S1\" sheetId=\"1\" r:id=\"rId1\"/>\
             <sheet name=\"S2\" sheetId=\"2\" r:id=\"rId2\"/>",
        )
        // The absent activeTab read 0, which was S1, now 1.
        .replace(
            "<workbookView firstSheet=\"9\"/>",
            "<workbookView firstSheet=\"9\" activeTab=\"1\"/>",
        );
    assert_part(&out, b"xl/workbook.xml", &want);
}

#[test]
fn a_book_view_index_that_is_not_a_number_refuses_a_move_at_the_write() {
    // P3 ruling 10: the remap cannot say what such a value means.
    for view in [
        "<workbookView activeTab=\"x\"/>",
        "<workbookView activeTab=\"\"/>",
        "<workbookView firstSheet=\"-1\"/>",
        "<workbookView activeTab=\"0\" firstSheet=\"1.0\"/>",
    ] {
        let views = format!("<bookViews>{view}</bookViews>");
        let (_, mut package) = opened(sheeted(2, &views, ""));
        package.move_sheet("S2", 0).expect("recorded");
        let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let before = replacement_temporaries();
        let error = package.write_to(Vec::new()).expect_err(view);
        assert_package(error, "a sheet edit no longer applies at the write");
        assert_eq!(replacement_temporaries(), before, "no temporary is left");
    }
}

#[test]
fn an_add_alone_leaves_defined_names_and_book_views_as_they_are() {
    let views = "<bookViews><workbookView firstSheet=\"1\"/></bookViews>";
    let names = "<definedNames><definedName name=\"a\" localSheetId=\"1\">S2!A1</definedName></definedNames>";
    let (parts, mut package) = opened(sheeted(2, views, names));
    package.add_sheet("New").expect("add");
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace(
            "</sheets>",
            "<sheet name=\"New\" sheetId=\"3\" r:id=\"rId4\"/></sheets>",
        ),
    );
}

// With the feature `metadata` the names are read with the sheets, so the
// same workbook is refused at the read instead
// (`specs/0019-names-links-properties-protection.md`, "Defined names";
// `names::a_local_sheet_id_that_is_not_a_number_is_refused_at_the_read`).
#[cfg(not(feature = "metadata"))]
#[test]
fn a_local_sheet_id_that_is_not_a_number_refuses_a_move_at_the_write() {
    let names =
        "<definedNames><definedName name=\"a\" localSheetId=\"x\">A1</definedName></definedNames>";
    let (_, mut package) = opened(sheeted(2, "", names));
    package.move_sheet("S2", 0).expect("recorded");
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let before = replacement_temporaries();
    let error = package.write_to(Vec::new()).expect_err("refused");
    assert_package(error, "a sheet edit no longer applies at the write");
    assert_eq!(replacement_temporaries(), before, "no temporary is left");
}

#[test]
fn a_sheet_added_to_a_workbook_with_no_sheets_element_is_refused_at_the_write() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/workbook.xml",
        format!("{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"/>"),
    );
    let (_, mut package) = opened(parts);
    package.add_sheet("Two").expect("recorded");
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    assert_package(
        package.write_to(Vec::new()).expect_err("refused"),
        "a sheet edit no longer applies at the write",
    );
}

#[test]
fn a_sheet_id_past_u32_is_refused_when_recorded() {
    let mut parts = smallest();
    let workbook = part_text(&parts, b"xl/workbook.xml")
        .replace("sheetId=\"1\"", &format!("sheetId=\"{}\"", u32::MAX));
    set(&mut parts, b"xl/workbook.xml", workbook);
    let (_, mut package) = opened(parts);
    assert_package(
        package.add_sheet("Two").expect_err("refused"),
        "a sheet edit no longer applies at the write",
    );
    assert_eq!(sheet_names(&mut package), ["Sheet1"]);
}

#[test]
fn a_sheets_element_holding_anything_but_sheets_is_refused() {
    for inner in ["<!-- note -->", "text", "<other/>"] {
        let mut parts = sheeted(2, "", "");
        let workbook = part_text(&parts, b"xl/workbook.xml")
            .replace("</sheets>", &format!("{inner}</sheets>"));
        set(&mut parts, b"xl/workbook.xml", workbook);
        let (_, mut package) = opened(parts);
        package.rename_sheet("S1", "One").expect("recorded");
        let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        assert_package(
            package.write_to(Vec::new()).expect_err(inner),
            "a sheets element holds something other than sheet elements",
        );
    }
}

#[test]
fn whitespace_between_sheets_is_dropped_and_a_mark_kept() {
    let mut parts = sheeted(2, "", "");
    let workbook = part_text(&parts, b"xl/workbook.xml")
        .replace("<sheets><sheet", "<sheets>\n  <sheet")
        .replace("/><sheet", "/>\n  <sheet")
        .replace("</sheets>", "\n</sheets>");
    let mut marked = "\u{feff}".to_string();
    marked.push_str(&workbook);
    set(&mut parts, b"xl/workbook.xml", marked.clone());
    let (_, mut package) = opened(parts);
    package.rename_sheet("S1", "One").expect("rename");
    let out = written(package);
    let want = marked
        .replace("<sheets>\n  <sheet", "<sheets><sheet")
        .replace("/>\n  <sheet", "/><sheet")
        .replace("\n</sheets>", "</sheets>")
        .replace("name=\"S1\"", "name=\"One\"");
    assert_part(&out, b"xl/workbook.xml", &want);
}

#[test]
fn a_prefixed_workbook_takes_a_rename_and_a_move_and_refuses_an_add() {
    let prefixed = |parts: &mut Vec<(Vec<u8>, Vec<u8>)>| {
        let workbook = format!(
            "{DECLARATION}\n<x:workbook xmlns:x=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><x:sheets>\
             <x:sheet name=\"S1\" sheetId=\"1\" r:id=\"rId1\"/><x:sheet name=\"S2\" sheetId=\"2\" r:id=\"rId2\"/>\
             </x:sheets></x:workbook>"
        );
        set(parts, b"xl/workbook.xml", workbook);
    };
    let mut parts = sheeted(2, "", "");
    prefixed(&mut parts);
    let (parts, mut package) = opened(parts);
    package.rename_sheet("S1", "One").expect("rename");
    package.move_sheet("S2", 0).expect("move");
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace(
            "<x:sheet name=\"S1\" sheetId=\"1\" r:id=\"rId1\"/><x:sheet name=\"S2\" sheetId=\"2\" r:id=\"rId2\"/>",
            "<x:sheet name=\"S2\" sheetId=\"2\" r:id=\"rId2\"/><x:sheet name=\"One\" sheetId=\"1\" r:id=\"rId1\"/>",
        ),
    );
    let mut parts = sheeted(2, "", "");
    prefixed(&mut parts);
    let (_, mut package) = opened(parts);
    package.add_sheet("Three").expect("recorded");
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let before = replacement_temporaries();
    assert_package(
        package.write_to(Vec::new()).expect_err("refused"),
        "a sheet cannot be added to a prefixed workbook",
    );
    assert_eq!(replacement_temporaries(), before, "no temporary is left");
}

#[test]
fn a_workbook_root_binding_no_relationships_prefix_refuses_an_add() {
    let mut parts = sheeted(1, "", "");
    let workbook = format!(
        "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\"><sheets>\
         <sheet name=\"S1\" sheetId=\"1\" xmlns:q=\"{ATTR_NS}\" q:id=\"rId1\"/></sheets></workbook>"
    );
    set(&mut parts, b"xl/workbook.xml", workbook);
    let (_, mut package) = opened(parts);
    package.add_sheet("Two").expect("recorded");
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    assert_package(
        package.write_to(Vec::new()).expect_err("refused"),
        "the workbook root binds no prefix for the relationships namespace",
    );
}

#[test]
fn a_sheet_edit_on_a_replaced_workbook_part_is_refused_at_the_write() {
    let (_, mut package) = opened(sheeted(2, "", ""));
    package.rename_sheet("S1", "One").expect("recorded");
    package.replace_part(b"xl/workbook.xml", replacement_from_reader(&b"<x/>"[..]));
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    assert_package(
        package.write_to(Vec::new()).expect_err("refused"),
        "the workbook part cannot be edited in a package that replaces or removes it",
    );
}

#[test]
fn sheets_projects_every_edit_in_order() {
    let (_, mut package) = opened(sheeted(2, "", ""));
    assert_eq!(sheet_names(&mut package), ["S1", "S2"]);
    package.add_sheet("N").expect("add");
    assert_eq!(sheet_names(&mut package), ["S1", "S2", "N"]);
    package.move_sheet("N", 0).expect("move");
    assert_eq!(sheet_names(&mut package), ["N", "S1", "S2"]);
    package.rename_sheet("S1", "T").expect("rename");
    assert_eq!(sheet_names(&mut package), ["N", "T", "S2"]);
    // The renamed sheet is found by its new name, and its old one is free.
    package.move_sheet("T", 2).expect("move by the new name");
    package.add_sheet("S1").expect("the old name is free");
    assert_eq!(sheet_names(&mut package), ["N", "S2", "T", "S1"]);
    assert_eq!(
        package.workbook().expect("workbook").sheets.len(),
        2,
        "the file's list is unchanged"
    );
}

#[test]
fn a_cell_edit_on_an_added_sheet_writes_into_its_blank_part() {
    let (_, mut package) = opened(smallest());
    let part = package.add_sheet("Two").expect("add");
    package.set_cell(&part, CellRef { row: 1, col: 1 }, Value::Number(7.0));
    package.move_sheet("Two", 0).expect("move");
    let out = written(package);
    let want = String::from_utf8(BLANK_WORKSHEET.to_vec())
        .expect("UTF-8")
        .replace(
            "<sheetData/>",
            "<sheetData><row r=\"1\"><c r=\"A1\"><v>7</v></c></row></sheetData>",
        );
    assert_added_part(&out, &part, want.as_bytes());
    let mut back = Package::open(Cursor::new(out)).expect("opens");
    let first = back.workbook().expect("workbook").sheets[0].clone();
    assert_eq!(
        (first.name.as_str(), first.part.as_slice()),
        ("Two", part.as_slice())
    );
    let cells: Vec<_> = back
        .cells(&part)
        .expect("cells")
        .collect::<Result<_, _>>()
        .expect("reads");
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].number(), Some(7.0));
}

#[test]
fn cells_on_a_sheet_added_and_not_yet_written_is_refused() {
    // P3 ruling 2: `cells` reads the file, which does not hold the part yet.
    let (_, mut package) = opened(smallest());
    let part = package.add_sheet("Two").expect("add");
    package.set_cell(&part, CellRef { row: 1, col: 1 }, Value::Number(7.0));
    assert_package(
        package.cells(&part).map(|_| ()).expect_err("pending"),
        "no such part",
    );
}

#[test]
fn a_new_workbook_takes_a_sheet_moved_first() {
    // The template's `activeTab="0"` names Sheet1, which moves to 1.
    let mut package = new_recorded();
    package.add_sheet("Two").expect("add");
    package.move_sheet("Two", 0).expect("move");
    let out = written(package);
    let want = String::from_utf8(template_file(b"xl/workbook.xml").to_vec())
        .expect("UTF-8")
        .replace("activeTab=\"0\"", "activeTab=\"1\"")
        .replace(
            "<sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets>",
            "<sheets><sheet name=\"Two\" sheetId=\"2\" r:id=\"rId3\"/>\
             <sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets>",
        );
    let mut back = Package::open(Cursor::new(out)).expect("opens");
    let index = back.part(b"xl/workbook.xml").expect("the workbook part");
    let mut bytes = Vec::new();
    back.read_part(index)
        .expect("read_part")
        .read_to_end(&mut bytes)
        .expect("read");
    assert_eq!(String::from_utf8(bytes).expect("UTF-8"), want);
}

// Removing a sheet (`specs/0016-new-workbook-and-sheets.md`, "Removing a
// sheet", P4 and P4 rulings 1 to 5).

const DRAWING_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawing+xml";
const CHART_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";
const COMMENTS_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml";
const CHARTSHEET_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml";
const CALC_CHAIN_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml";

/// Adds a part at the end of `parts`, with an `Override` of `content_type`
/// when one is given.
fn with_part(parts: &mut Parts, name: &str, bytes: &str, content_type: Option<&str>) {
    parts.push((name.as_bytes().to_vec(), bytes.as_bytes().to_vec()));
    if let Some(content_type) = content_type {
        let types = part_text(parts, b"[Content_Types].xml").replace(
            "</Types>",
            &format!("<Override PartName=\"/{name}\" ContentType=\"{content_type}\"/></Types>"),
        );
        set(parts, b"[Content_Types].xml", types);
    }
}

/// Adds a `Default` for `extension` to the content types part.
fn with_default(parts: &mut Parts, extension: &str, content_type: &str) {
    let types = part_text(parts, b"[Content_Types].xml").replace(
        "</Types>",
        &format!("<Default Extension=\"{extension}\" ContentType=\"{content_type}\"/></Types>"),
    );
    set(parts, b"[Content_Types].xml", types);
}

/// The content types part without the `Override` for `name`.
fn types_without(parts: &Parts, names: &[&str]) -> String {
    let mut types = part_text(parts, b"[Content_Types].xml");
    for name in names {
        let element = types
            .split("<Override ")
            .find(|element| element.starts_with(&format!("PartName=\"/{name}\"")))
            .map(|element| {
                format!(
                    "<Override {}",
                    &element[..=element.find("/>").expect("closed") + 1]
                )
            })
            .expect("the override is there");
        types = types.replace(&element, "");
    }
    types
}

/// The entry names of `parts` without those named.
fn names_without<'p>(parts: &'p Parts, gone: &[&str]) -> Vec<&'p [u8]> {
    part_names(parts)
        .into_iter()
        .filter(|name| !gone.iter().any(|gone| gone.as_bytes() == *name))
        .collect()
}

/// The workbook relationships part without the relationship `id`.
fn workbook_rels_without(parts: &Parts, id: &str) -> String {
    let rels = part_text(parts, b"xl/_rels/workbook.xml.rels");
    let start = rels
        .find(&format!("<Relationship Id=\"{id}\""))
        .expect("the relationship is there");
    let end = start + rels[start..].find("/>").expect("closed") + 2;
    format!("{}{}", &rels[..start], &rels[end..])
}

/// The `<sheet>` element `sheeted` writes for sheet `i`.
fn sheet_element(i: usize) -> String {
    format!("<sheet name=\"S{i}\" sheetId=\"{i}\" r:id=\"rId{i}\"/>")
}

/// A worksheet whose rels part holds `body`, for sheet `i` of `sheeted`.
fn with_sheet_rels(parts: &mut Parts, i: usize, body: &str) {
    with_part(
        parts,
        &format!("xl/worksheets/_rels/sheet{i}.xml.rels"),
        &rels(body),
        None,
    );
}

#[test]
fn removing_the_last_of_two_drops_its_sheet_relationship_override_entry_and_rels_part() {
    let mut parts = sheeted(2, "", "");
    with_sheet_rels(
        &mut parts,
        2,
        "<Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink\" Target=\"https://example.com/\" TargetMode=\"External\"/>",
    );
    let (parts, mut package) = opened(parts);
    package.remove_sheet("S2").expect("remove_sheet");
    assert_eq!(sheet_names(&mut package), ["S1"]);
    let out = written(package);
    assert_names(
        &out,
        &names_without(
            &parts,
            &[
                "xl/worksheets/sheet2.xml",
                "xl/worksheets/_rels/sheet2.xml.rels",
            ],
        ),
    );
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace(&sheet_element(2), ""),
    );
    assert_part(
        &out,
        b"xl/_rels/workbook.xml.rels",
        &workbook_rels_without(&parts, "rId2"),
    );
    assert_part(
        &out,
        b"[Content_Types].xml",
        &types_without(&parts, &["xl/worksheets/sheet2.xml"]),
    );
    assert_untouched(
        &out,
        &parts,
        &[
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
            b"[Content_Types].xml",
            b"xl/worksheets/sheet2.xml",
            b"xl/worksheets/_rels/sheet2.xml.rels",
        ],
    );
}

/// `sheeted(n)` with a drawing on sheet 2 whose rels part names a chart and
/// an image; the drawing and chart have overrides, the image a `Default`.
fn with_drawing(n: usize) -> Parts {
    let mut parts = sheeted(n, "", "");
    with_default(&mut parts, "png", "image/png");
    with_sheet_rels(
        &mut parts,
        2,
        &relationship(
            "rId1",
            &format!("{ATTR_NS}/drawing"),
            "../drawings/drawing1.xml",
        ),
    );
    with_part(
        &mut parts,
        "xl/drawings/drawing1.xml",
        "<wsDr/>",
        Some(DRAWING_TYPE),
    );
    with_part(
        &mut parts,
        "xl/drawings/_rels/drawing1.xml.rels",
        &rels(&format!(
            "{}{}",
            relationship("rId1", &format!("{ATTR_NS}/chart"), "../charts/chart1.xml"),
            relationship("rId2", &format!("{ATTR_NS}/image"), "../media/image1.png"),
        )),
        None,
    );
    with_part(
        &mut parts,
        "xl/charts/chart1.xml",
        "<chartSpace/>",
        Some(CHART_TYPE),
    );
    with_part(&mut parts, "xl/media/image1.png", "png", None);
    parts
}

#[test]
fn a_drawing_and_its_chart_reached_only_from_the_removed_sheet_go_with_it() {
    let (parts, mut package) = opened(with_drawing(2));
    package.remove_sheet("S2").expect("remove_sheet");
    let out = written(package);
    let gone = [
        "xl/worksheets/sheet2.xml",
        "xl/worksheets/_rels/sheet2.xml.rels",
        "xl/drawings/drawing1.xml",
        "xl/drawings/_rels/drawing1.xml.rels",
        "xl/charts/chart1.xml",
        "xl/media/image1.png",
    ];
    assert_names(&out, &names_without(&parts, &gone));
    assert_part(
        &out,
        b"[Content_Types].xml",
        &types_without(
            &parts,
            &[
                "xl/worksheets/sheet2.xml",
                "xl/drawings/drawing1.xml",
                "xl/charts/chart1.xml",
            ],
        ),
    );
}

#[test]
fn a_drawing_also_reached_from_a_kept_sheet_is_kept_whole() {
    let mut parts = with_drawing(2);
    with_sheet_rels(
        &mut parts,
        1,
        &relationship(
            "rId1",
            &format!("{ATTR_NS}/drawing"),
            "../drawings/drawing1.xml",
        ),
    );
    let (parts, mut package) = opened(parts);
    package.remove_sheet("S2").expect("remove_sheet");
    let out = written(package);
    assert_names(
        &out,
        &names_without(
            &parts,
            &[
                "xl/worksheets/sheet2.xml",
                "xl/worksheets/_rels/sheet2.xml.rels",
            ],
        ),
    );
    assert_part(
        &out,
        b"[Content_Types].xml",
        &types_without(&parts, &["xl/worksheets/sheet2.xml"]),
    );
}

#[test]
fn two_sheets_sharing_a_drawing_both_removed_take_it_with_the_second() {
    // P4 ruling 3: the first removal keeps the drawing, which the other
    // sheet still reaches; the second takes it, and nothing is recorded
    // twice.
    let mut parts = with_drawing(3);
    with_sheet_rels(
        &mut parts,
        3,
        &relationship(
            "rId1",
            &format!("{ATTR_NS}/drawing"),
            "../drawings/drawing1.xml",
        ),
    );
    let (parts, mut package) = opened(parts);
    package.remove_sheet("S2").expect("first");
    package.remove_sheet("S3").expect("second");
    assert_eq!(sheet_names(&mut package), ["S1"]);
    let out = written(package);
    assert_names(
        &out,
        &names_without(
            &parts,
            &[
                "xl/worksheets/sheet2.xml",
                "xl/worksheets/sheet3.xml",
                "xl/worksheets/_rels/sheet2.xml.rels",
                "xl/worksheets/_rels/sheet3.xml.rels",
                "xl/drawings/drawing1.xml",
                "xl/drawings/_rels/drawing1.xml.rels",
                "xl/charts/chart1.xml",
                "xl/media/image1.png",
            ],
        ),
    );
}

#[test]
fn a_comments_part_and_its_vml_reached_only_from_the_sheet_go() {
    let mut parts = sheeted(2, "", "");
    with_default(
        &mut parts,
        "vml",
        "application/vnd.openxmlformats-officedocument.vmlDrawing",
    );
    with_sheet_rels(
        &mut parts,
        2,
        &format!(
            "{}{}",
            relationship("rId1", &format!("{ATTR_NS}/comments"), "../comments1.xml"),
            relationship(
                "rId2",
                &format!("{ATTR_NS}/vmlDrawing"),
                "../drawings/vmlDrawing1.vml"
            ),
        ),
    );
    with_part(
        &mut parts,
        "xl/comments1.xml",
        "<comments/>",
        Some(COMMENTS_TYPE),
    );
    with_part(&mut parts, "xl/drawings/vmlDrawing1.vml", "<xml/>", None);
    let (parts, mut package) = opened(parts);
    package.remove_sheet("S2").expect("remove_sheet");
    let out = written(package);
    assert_names(
        &out,
        &names_without(
            &parts,
            &[
                "xl/worksheets/sheet2.xml",
                "xl/worksheets/_rels/sheet2.xml.rels",
                "xl/comments1.xml",
                "xl/drawings/vmlDrawing1.vml",
            ],
        ),
    );
    assert_part(
        &out,
        b"[Content_Types].xml",
        &types_without(&parts, &["xl/worksheets/sheet2.xml", "xl/comments1.xml"]),
    );
}

#[test]
fn a_relationship_to_no_entry_or_outside_the_package_contributes_nothing() {
    let mut parts = sheeted(2, "", "");
    with_sheet_rels(
        &mut parts,
        2,
        &format!(
            "{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/drawing"),
                "../drawings/missing.xml"
            ),
            relationship(
                "rId2",
                &format!("{ATTR_NS}/drawing"),
                "../../../../outside.xml"
            ),
        ),
    );
    let (parts, mut package) = opened(parts);
    package.remove_sheet("S2").expect("remove_sheet");
    let out = written(package);
    assert_names(
        &out,
        &names_without(
            &parts,
            &[
                "xl/worksheets/sheet2.xml",
                "xl/worksheets/_rels/sheet2.xml.rels",
            ],
        ),
    );
}

#[test]
fn an_external_relationship_is_never_followed() {
    // The external target spells an entry no kept part reaches; followed, it
    // would go.
    let mut parts = sheeted(2, "", "");
    with_sheet_rels(
        &mut parts,
        2,
        &format!(
            "<Relationship Id=\"rId1\" Type=\"{ATTR_NS}/hyperlink\" Target=\"../orphan.xml\" TargetMode=\"External\"/>"
        ),
    );
    with_part(&mut parts, "xl/orphan.xml", "<orphan/>", None);
    let (parts, mut package) = opened(parts);
    package.remove_sheet("S2").expect("remove_sheet");
    let out = written(package);
    assert_names(
        &out,
        &names_without(
            &parts,
            &[
                "xl/worksheets/sheet2.xml",
                "xl/worksheets/_rels/sheet2.xml.rels",
            ],
        ),
    );
    assert_untouched(
        &out,
        &parts,
        &[
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
            b"[Content_Types].xml",
            b"xl/worksheets/sheet2.xml",
            b"xl/worksheets/_rels/sheet2.xml.rels",
        ],
    );
}

/// `sheeted(n)` with a `calcPr` and a calculation chain, sheet 2 holding a
/// formula.
fn with_chain(n: usize) -> Parts {
    let mut parts = sheeted(n, "", "<calcPr calcId=\"191029\"/>");
    let rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        "</Relationships>",
        &format!(
            "{}</Relationships>",
            relationship("rId9", &format!("{ATTR_NS}/calcChain"), "calcChain.xml")
        ),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", rels);
    set(
        &mut parts,
        b"xl/worksheets/sheet2.xml",
        format!(
            "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"><sheetData><row r=\"1\"><c r=\"A1\"><f>1+1</f><v>2</v></c></row></sheetData></worksheet>"
        ),
    );
    with_part(
        &mut parts,
        "xl/calcChain.xml",
        &format!("{DECLARATION}\n<calcChain xmlns=\"{MAIN_NS}\"><c r=\"A1\" i=\"2\"/></calcChain>"),
        Some(CALC_CHAIN_TYPE),
    );
    parts
}

#[test]
fn removing_a_sheet_drops_the_chain_and_sets_full_calc_on_load() {
    let (parts, mut package) = opened(with_chain(2));
    package.remove_sheet("S2").expect("remove_sheet");
    let out = written(package);
    assert_names(
        &out,
        &names_without(&parts, &["xl/worksheets/sheet2.xml", "xl/calcChain.xml"]),
    );
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml")
            .replace(&sheet_element(2), "")
            .replace(
                "<calcPr calcId=\"191029\"/>",
                "<calcPr calcId=\"191029\" fullCalcOnLoad=\"1\"/>",
            ),
    );
    let rels = workbook_rels_without(&parts, "rId2");
    let start = rels.find("<Relationship Id=\"rId9\"").expect("the chain");
    let end = start + rels[start..].find("/>").expect("closed") + 2;
    assert_part(
        &out,
        b"xl/_rels/workbook.xml.rels",
        &format!("{}{}", &rels[..start], &rels[end..]),
    );
    assert_part(
        &out,
        b"[Content_Types].xml",
        &types_without(&parts, &["xl/worksheets/sheet2.xml", "xl/calcChain.xml"]),
    );
}

#[test]
fn two_removals_drop_the_chain_once() {
    let (parts, mut package) = opened(with_chain(3));
    package.remove_sheet("S2").expect("first");
    package.remove_sheet("S3").expect("second");
    let out = written(package);
    assert_names(
        &out,
        &names_without(
            &parts,
            &[
                "xl/worksheets/sheet2.xml",
                "xl/worksheets/sheet3.xml",
                "xl/calcChain.xml",
            ],
        ),
    );
}

#[test]
fn a_workbook_with_no_chain_keeps_its_calc_pr() {
    let (parts, mut package) = opened(sheeted(2, "", "<calcPr calcId=\"191029\"/>"));
    package.remove_sheet("S2").expect("remove_sheet");
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace(&sheet_element(2), ""),
    );
}

#[test]
fn removing_the_first_sheet_remaps_local_sheet_ids_active_tabs_and_first_sheet() {
    let views = "<bookViews><workbookView activeTab=\"0\" firstSheet=\"1\"/>\
                 <workbookView activeTab=\"1\"/><workbookView/></bookViews>";
    let names = "<definedNames><definedName name=\"a\" localSheetId=\"0\">S1!A1</definedName>\
                 <definedName name=\"b\" localSheetId=\"1\">S2!A1</definedName>\
                 <definedName name=\"c\">S1!A1</definedName></definedNames>";
    let (parts, mut package) = opened(sheeted(2, views, names));
    package.remove_sheet("S1").expect("remove_sheet");
    let out = written(package);
    let want = part_text(&parts, b"xl/workbook.xml")
        .replace(&sheet_element(1), "")
        // activeTab 0 named S1, gone: the first visible sheet, 0. activeTab
        // 1 named S2, now 0. firstSheet 1 loses the one removed sheet before
        // it. The absent activeTab read 0 and maps to 0, so stays absent.
        .replace("firstSheet=\"1\"", "firstSheet=\"0\"")
        .replace(
            "<workbookView activeTab=\"1\"/>",
            "<workbookView activeTab=\"0\"/>",
        )
        // localSheetId 0 goes with its element; 1 becomes 0.
        .replace(
            "<definedName name=\"a\" localSheetId=\"0\">S1!A1</definedName>",
            "",
        )
        .replace(
            "name=\"b\" localSheetId=\"1\"",
            "name=\"b\" localSheetId=\"0\"",
        );
    assert_part(&out, b"xl/workbook.xml", &want);
}

#[test]
fn an_active_tab_on_the_removed_sheet_goes_to_the_first_visible_sheet() {
    let views = "<bookViews><workbookView activeTab=\"0\"/></bookViews>";
    let mut parts = sheeted(3, views, "");
    let workbook = part_text(&parts, b"xl/workbook.xml").replace(
        "<sheet name=\"S2\" sheetId=\"2\"",
        "<sheet name=\"S2\" state=\"hidden\" sheetId=\"2\"",
    );
    set(&mut parts, b"xl/workbook.xml", workbook);
    let (parts, mut package) = opened(parts);
    package.remove_sheet("S1").expect("remove_sheet");
    let out = written(package);
    let want = part_text(&parts, b"xl/workbook.xml")
        .replace(&sheet_element(1), "")
        .replace("activeTab=\"0\"", "activeTab=\"1\"");
    assert_part(&out, b"xl/workbook.xml", &want);
}

#[test]
fn a_sheet_whose_state_says_visible_counts_as_visible() {
    // P4 ruling 1: the first sheet removed when the second says visible is
    // allowed, and the active tab goes to it, at 0.
    let views = "<bookViews><workbookView activeTab=\"0\"/></bookViews>";
    let mut parts = sheeted(2, views, "");
    let workbook = part_text(&parts, b"xl/workbook.xml").replace(
        "<sheet name=\"S2\" sheetId=\"2\"",
        "<sheet name=\"S2\" state=\"visible\" sheetId=\"2\"",
    );
    set(&mut parts, b"xl/workbook.xml", workbook);
    let (parts, mut package) = opened(parts);
    package.remove_sheet("S1").expect("allowed");
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace(&sheet_element(1), ""),
    );
}

#[test]
fn the_last_sheet_cannot_be_removed() {
    let (parts, mut package) = opened(smallest());
    assert_package(
        package.remove_sheet("Sheet1").expect_err("refused"),
        "the last sheet cannot be removed",
    );
    // A pending sheet counts: with it, Sheet1 may go, and then it may not.
    package.add_sheet("Two").expect("add");
    package.remove_sheet("Sheet1").expect("allowed");
    assert_package(
        package.remove_sheet("Two").expect_err("refused"),
        "the last sheet cannot be removed",
    );
    assert_eq!(sheet_names(&mut package), ["Two"]);
    drop(parts);
}

#[test]
fn the_last_visible_sheet_cannot_be_removed() {
    let mut parts = sheeted(3, "", "");
    let workbook = part_text(&parts, b"xl/workbook.xml")
        .replace(
            "<sheet name=\"S2\" sheetId=\"2\"",
            "<sheet name=\"S2\" state=\"hidden\" sheetId=\"2\"",
        )
        .replace(
            "<sheet name=\"S3\" sheetId=\"3\"",
            "<sheet name=\"S3\" state=\"other\" sheetId=\"3\"",
        );
    set(&mut parts, b"xl/workbook.xml", workbook);
    let (parts, mut package) = opened(parts);
    assert_package(
        package.remove_sheet("S1").expect_err("refused"),
        "the last visible sheet cannot be removed",
    );
    // Nothing was recorded, and a hidden sheet may go.
    package.remove_sheet("S2").expect("a hidden sheet");
    let out = written(package);
    assert_names(&out, &names_without(&parts, &["xl/worksheets/sheet2.xml"]));
}

#[test]
fn a_chartsheet_is_removed_with_what_only_it_reaches() {
    let mut parts = sheeted(1, "", "");
    let workbook = part_text(&parts, b"xl/workbook.xml").replace(
        "</sheets>",
        "<sheet name=\"C1\" sheetId=\"2\" r:id=\"rId7\"/></sheets>",
    );
    set(&mut parts, b"xl/workbook.xml", workbook);
    let workbook_rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        "</Relationships>",
        &format!(
            "{}</Relationships>",
            relationship(
                "rId7",
                &format!("{ATTR_NS}/chartsheet"),
                "chartsheets/sheet1.xml"
            )
        ),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", workbook_rels);
    with_part(
        &mut parts,
        "xl/chartsheets/sheet1.xml",
        "<chartsheet/>",
        Some(CHARTSHEET_TYPE),
    );
    with_part(
        &mut parts,
        "xl/chartsheets/_rels/sheet1.xml.rels",
        &rels_text_drawing(),
        None,
    );
    with_part(
        &mut parts,
        "xl/drawings/drawing1.xml",
        "<wsDr/>",
        Some(DRAWING_TYPE),
    );
    with_part(
        &mut parts,
        "xl/drawings/_rels/drawing1.xml.rels",
        &rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/chart"),
            "../charts/chart1.xml",
        )),
        None,
    );
    with_part(
        &mut parts,
        "xl/charts/chart1.xml",
        "<chartSpace/>",
        Some(CHART_TYPE),
    );
    let (parts, mut package) = opened(parts);
    package.remove_sheet("C1").expect("remove_sheet");
    let out = written(package);
    let gone = [
        "xl/chartsheets/sheet1.xml",
        "xl/chartsheets/_rels/sheet1.xml.rels",
        "xl/drawings/drawing1.xml",
        "xl/drawings/_rels/drawing1.xml.rels",
        "xl/charts/chart1.xml",
    ];
    assert_names(&out, &names_without(&parts, &gone));
    assert_part(
        &out,
        b"[Content_Types].xml",
        &types_without(
            &parts,
            &[
                "xl/chartsheets/sheet1.xml",
                "xl/drawings/drawing1.xml",
                "xl/charts/chart1.xml",
            ],
        ),
    );
    assert_part(
        &out,
        b"xl/_rels/workbook.xml.rels",
        &workbook_rels_without(&parts, "rId7"),
    );
}

/// A rels part naming one drawing, for a chartsheet.
fn rels_text_drawing() -> String {
    rels(&relationship(
        "rId1",
        &format!("{ATTR_NS}/drawing"),
        "../drawings/drawing1.xml",
    ))
}

#[test]
fn remove_then_add_of_the_same_name_takes_new_ids() {
    // P4 ruling 2: the removed entry still counts as taken; the sheetId is the
    // projected maximum plus one, here the removed sheet's; the relationship
    // id is never reused.
    let (parts, mut package) = opened(sheeted(2, "", ""));
    package.remove_sheet("S2").expect("remove");
    let part = package.add_sheet("S2").expect("add");
    assert_eq!(part, b"xl/worksheets/sheet3.xml");
    let added = package.sheets().expect("sheets")[1].clone();
    assert_eq!(
        (added.sheet_id, added.rel_id.as_slice()),
        (2, b"rId4".as_slice())
    );
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace(
            &sheet_element(2),
            "<sheet name=\"S2\" sheetId=\"2\" r:id=\"rId4\"/>",
        ),
    );
    let mut back = Package::open(Cursor::new(out)).expect("opens");
    let sheets = back.workbook().expect("workbook").sheets.clone();
    assert_eq!(sheets[1].part, b"xl/worksheets/sheet3.xml");
}

#[test]
fn every_byte_outside_the_regions_is_kept_with_ext_lst_and_pivot_caches() {
    let before = "\n  <fileVersion appName=\"xl\"/>\n  <bookViews><workbookView activeTab=\"1\"/></bookViews>\n  ";
    let after = "\n  <calcPr calcId=\"1\"/>\n  <pivotCaches><pivotCache cacheId=\"1\" r:id=\"rId8\"/></pivotCaches>\n  \
                 <extLst><ext uri=\"{X}\"><x:y xmlns:x=\"urn:x\" a=\"1\"/></ext></extLst>\n";
    let mut parts = sheeted(2, before, after);
    let rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        "</Relationships>",
        &format!(
            "{}</Relationships>",
            relationship(
                "rId8",
                &format!("{ATTR_NS}/pivotCacheDefinition"),
                "pivotCache/pivotCacheDefinition1.xml"
            )
        ),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", rels);
    with_part(
        &mut parts,
        "xl/pivotCache/pivotCacheDefinition1.xml",
        "<pivotCacheDefinition/>",
        None,
    );
    let (parts, mut package) = opened(parts);
    package.remove_sheet("S1").expect("remove_sheet");
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml")
            .replace(&sheet_element(1), "")
            .replace("activeTab=\"1\"", "activeTab=\"0\""),
    );
    assert!(
        part_names(&parts)
            .iter()
            .any(|name| *name == b"xl/pivotCache/pivotCacheDefinition1.xml")
    );
    assert_names(&out, &names_without(&parts, &["xl/worksheets/sheet1.xml"]));
}

#[test]
fn adding_then_removing_a_pending_sheet_writes_the_input_back() {
    // Check 1's F4 and P4 ruling 4: the add's recorded additions and the cell
    // edit on its part are dropped, the workbook part is not rewritten, and
    // the output is the input.
    let parts = smallest();
    let bytes = Build::new(
        parts
            .iter()
            .map(|(name, data)| Spec::new(name, data))
            .collect(),
    )
    .bytes();
    let mut package = open_fixture(bytes.clone());
    let part = package.add_sheet("Two").expect("add");
    package.set_cell(&part, CellRef { row: 1, col: 1 }, Value::Number(7.0));
    package.move_sheet("Two", 0).expect("move");
    package.remove_sheet("Two").expect("remove");
    assert_eq!(sheet_names(&mut package), ["Sheet1"]);
    let out = written(package);
    assert_eq!(out, bytes, "the output is the input");
}

#[test]
fn a_pending_sheet_removed_frees_its_name_and_later_adds_follow_on() {
    let (parts, mut package) = opened(smallest());
    let first = package.add_sheet("Two").expect("add");
    package.remove_sheet("Two").expect("remove");
    let again = package.add_sheet("Three").expect("add again");
    assert_eq!(first, again, "the pending name is free again");
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace(
            "</sheets>",
            "<sheet name=\"Three\" sheetId=\"2\" r:id=\"rId3\"/></sheets>",
        ),
    );
}

#[test]
fn a_cell_edit_on_a_sheet_then_removed_refuses_the_write() {
    let (_, mut package) = opened(sheeted(2, "", ""));
    package.set_cell(
        b"xl/worksheets/sheet2.xml",
        CellRef { row: 1, col: 1 },
        Value::Number(1.0),
    );
    package.remove_sheet("S2").expect("recorded");
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    assert_package(
        package.write_to(Vec::new()).expect_err("refused"),
        "an edit to a removed part",
    );
}

#[test]
fn a_rels_part_that_cannot_be_read_refuses_the_removal_and_records_nothing() {
    let mut parts = sheeted(2, "", "");
    with_sheet_rels(&mut parts, 2, "");
    set(
        &mut parts,
        b"xl/worksheets/_rels/sheet2.xml.rels",
        "<notRelationships/>".to_string(),
    );
    let (parts, mut package) = opened(parts);
    assert!(package.remove_sheet("S2").is_err());
    assert_eq!(sheet_names(&mut package), ["S1", "S2"]);
    let out = written(package);
    assert_untouched(&out, &parts, &[]);
}

#[test]
fn a_removed_rels_part_loses_its_override_too() {
    // P4 ruling 9: an Override naming the removed sheet's rels part goes with
    // it, so no Override names a removed entry.
    let mut parts = sheeted(2, "", "");
    with_sheet_rels(
        &mut parts,
        2,
        &format!(
            "<Relationship Id=\"rId1\" Type=\"{ATTR_NS}/hyperlink\" Target=\"https://example.com/\" TargetMode=\"External\"/>"
        ),
    );
    let types = part_text(&parts, b"[Content_Types].xml").replace(
        "</Types>",
        "<Override PartName=\"/xl/worksheets/_rels/sheet2.xml.rels\" \
         ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/></Types>",
    );
    set(&mut parts, b"[Content_Types].xml", types);
    let (parts, mut package) = opened(parts);
    package.remove_sheet("S2").expect("remove_sheet");
    let out = written(package);
    assert_names(
        &out,
        &names_without(
            &parts,
            &[
                "xl/worksheets/sheet2.xml",
                "xl/worksheets/_rels/sheet2.xml.rels",
            ],
        ),
    );
    assert_part(
        &out,
        b"[Content_Types].xml",
        &types_without(
            &parts,
            &[
                "xl/worksheets/sheet2.xml",
                "xl/worksheets/_rels/sheet2.xml.rels",
            ],
        ),
    );
}

// ------------------------------- fuller cell styles (specs/0017, P1)

use crate::styles::{BorderStyle, Color, Pattern, Scheme, Side, Underline, VerticalAlign};

/// The three collections the template has, with one font, the two required
/// fills and one border, before `cell_xfs`.
fn with_collections(cell_xfs: &str) -> String {
    format!(
        "<fonts count=\"1\"><font><sz val=\"11\"/></font></fonts>\
         <fills count=\"2\"><fill><patternFill patternType=\"none\"/></fill>\
         <fill><patternFill patternType=\"gray125\"/></fill></fills>\
         <borders count=\"1\"><border><left/><right/><top/><bottom/><diagonal/></border></borders>\
         {cell_xfs}"
    )
}

/// Records one `add_style` and asserts the index it returns.
#[track_caller]
fn add_styled(
    package: &mut Package<Cursor<Vec<u8>>>,
    base: Option<u32>,
    style: &Style,
    index: u32,
) {
    let got = package
        .add_style(base, style)
        .expect("the addition is recorded");
    assert_eq!(got, index, "the index the addition returns");
}

/// A font with a name and nothing else.
fn named(name: &str) -> Font {
    Font {
        name: Some(name.to_owned()),
        ..Font::default()
    }
}

/// A solid fill of an indexed colour.
fn solid(index: u32) -> Fill {
    Fill {
        pattern: Pattern::Solid,
        foreground: Some(Color::Indexed(index)),
        background: None,
        gradient: None,
    }
}

/// A border with a thin bottom and nothing else.
fn underlined() -> Border {
    Border {
        bottom: Some(Side {
            style: BorderStyle::Thin,
            color: None,
        }),
        ..Border::default()
    }
}

/// Nothing is recorded by a refused addition.
#[track_caller]
fn assert_nothing_recorded(package: &Package<Cursor<Vec<u8>>>) {
    let edits = &package.edits;
    assert!(edits.xf_additions.is_empty(), "an xf was recorded");
    assert!(edits.num_fmt_additions.is_empty(), "a numFmt was recorded");
    assert!(edits.font_additions.is_empty(), "a font was recorded");
    assert!(edits.fill_additions.is_empty(), "a fill was recorded");
    assert!(edits.border_additions.is_empty(), "a border was recorded");
}

#[test]
fn a_font_with_every_field_reaches_the_part_as_its_element() {
    let font = Font {
        name: Some("A & B".to_owned()),
        size: Some(10.5),
        bold: true,
        italic: true,
        strike: true,
        underline: Some(Underline::Double),
        vertical_align: Some(VerticalAlign::Subscript),
        color: Some(Color::Rgb(0xFF00_8000)),
        family: Some(2),
        scheme: Some(Scheme::Major),
    };
    let xfs = "<cellXfs count=\"1\"><xf numFmtId=\"0\" fontId=\"0\"/></cellXfs>";
    let got = added_styles(&with_collections(xfs), |p| {
        let style = Style {
            font: Some(font.clone()),
            ..Style::default()
        };
        add_styled(p, None, &style, 1);
    });
    let expected = with_collections(
        "<cellXfs count=\"2\"><xf numFmtId=\"0\" fontId=\"0\"/>\
         <xf numFmtId=\"0\" fontId=\"1\" fillId=\"0\" borderId=\"0\" xfId=\"0\" applyFont=\"1\"/>\
         </cellXfs>",
    )
    .replace(
        "<fonts count=\"1\"><font><sz val=\"11\"/></font></fonts>",
        "<fonts count=\"2\"><font><sz val=\"11\"/></font>\
         <font><b/><i/><strike/><u val=\"double\"/><vertAlign val=\"subscript\"/>\
         <sz val=\"10.5\"/><color rgb=\"FF008000\"/><name val=\"A &amp; B\"/>\
         <family val=\"2\"/><scheme val=\"major\"/></font></fonts>",
    );
    assert_eq!(got, style_sheet(&expected));
}

#[test]
fn the_rewrite_grows_fonts_fills_and_cell_xfs_and_keeps_num_fmts_and_borders() {
    // A font and a fill on a base, the second cell format naming the same
    // font: `fonts` grows by one, `fills` by one, `cellXfs` by two, and
    // `numFmts` and `borders` come out byte for byte, count and all.
    let num_fmts = "<numFmts count=\"1\"><numFmt numFmtId=\"164\" formatCode=\"0.0\"/></numFmts>";
    let base = "<xf numFmtId=\"164\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\" \
                applyNumberFormat=\"1\"/>";
    let body = format!(
        "{num_fmts}{}",
        with_collections(&format!("<cellXfs count=\"1\">{base}</cellXfs>"))
    );
    let got = added_styles(&body, |p| {
        let both = Style {
            font: Some(named("emend")),
            fill: Some(solid(64)),
            ..Style::default()
        };
        add_styled(p, Some(0), &both, 1);
        let font = Style {
            font: Some(named("emend")),
            ..Style::default()
        };
        add_styled(p, None, &font, 2);
    });
    let expected = body
        .replace(
            "<fonts count=\"1\"><font><sz val=\"11\"/></font></fonts>",
            "<fonts count=\"2\"><font><sz val=\"11\"/></font>\
             <font><name val=\"emend\"/></font></fonts>",
        )
        .replace(
            "<fill><patternFill patternType=\"gray125\"/></fill></fills>",
            "<fill><patternFill patternType=\"gray125\"/></fill>\
             <fill><patternFill patternType=\"solid\"><fgColor indexed=\"64\"/></patternFill>\
             </fill></fills>",
        )
        .replace("<fills count=\"2\">", "<fills count=\"3\">")
        .replace(
            &format!("<cellXfs count=\"1\">{base}</cellXfs>"),
            &format!(
                "<cellXfs count=\"3\">{base}\
                 <xf numFmtId=\"164\" fontId=\"1\" fillId=\"2\" borderId=\"0\" xfId=\"0\" \
                 applyNumberFormat=\"1\" applyFont=\"1\" applyFill=\"1\"/>\
                 <xf numFmtId=\"0\" fontId=\"1\" fillId=\"0\" borderId=\"0\" xfId=\"0\" \
                 applyFont=\"1\"/></cellXfs>"
            ),
        );
    assert_eq!(got, style_sheet(&expected));
    assert!(got.contains(num_fmts), "numFmts is byte-identical");
    assert!(
        got.contains("<borders count=\"1\"><border><left/><right/><top/><bottom/><diagonal/></border></borders>"),
        "borders is byte-identical"
    );
}

#[test]
fn the_rewrite_grows_borders() {
    let xfs = "<cellXfs count=\"1\"><xf/></cellXfs>";
    let got = added_styles(&with_collections(xfs), |p| {
        let style = Style {
            border: Some(underlined()),
            ..Style::default()
        };
        add_styled(p, Some(0), &style, 1);
    });
    let expected = with_collections(
        "<cellXfs count=\"2\"><xf/><xf borderId=\"1\" applyBorder=\"1\"/></cellXfs>",
    )
    .replace(
        "<borders count=\"1\"><border><left/><right/><top/><bottom/><diagonal/></border></borders>",
        "<borders count=\"2\"><border><left/><right/><top/><bottom/><diagonal/></border>\
         <border><left/><right/><top/><bottom style=\"thin\"/><diagonal/></border></borders>",
    );
    assert_eq!(got, style_sheet(&expected));
}

#[test]
fn an_empty_container_opens_up_a_wrong_count_is_corrected_and_order_is_not_assumed() {
    // `cellXfs` before the collections, a non-schema order: each container
    // is still found by its name. `fonts` is empty, `fills` says 7 of its 2
    // children, `borders` gains nothing and is kept byte for byte.
    let body = "<cellXfs count=\"1\"><xf/></cellXfs><fonts count=\"0\"/>\
                <fills count=\"7\"><fill/><fill/></fills><borders count=\"0\"/>";
    let got = added_styles(body, |p| {
        let style = Style {
            font: Some(named("a")),
            fill: Some(solid(9)),
            ..Style::default()
        };
        add_styled(p, None, &style, 1);
    });
    assert_eq!(
        got,
        style_sheet(
            "<cellXfs count=\"2\"><xf/><xf numFmtId=\"0\" fontId=\"0\" fillId=\"2\" \
             borderId=\"0\" xfId=\"0\" applyFont=\"1\" applyFill=\"1\"/></cellXfs>\
             <fonts count=\"1\"><font><name val=\"a\"/></font></fonts>\
             <fills count=\"3\"><fill/><fill/><fill><patternFill patternType=\"solid\">\
             <fgColor indexed=\"9\"/></patternFill></fill></fills><borders count=\"0\"/>"
        )
    );
}

#[test]
fn the_new_elements_take_their_containers_prefix() {
    let body = format!(
        "{DECLARATION}\n<x:styleSheet xmlns:x=\"{MAIN_NS}\"><x:fonts count=\"0\"/>\
         <x:fills count=\"0\"></x:fills><x:borders count=\"0\"/>\
         <x:cellXfs count=\"0\"/></x:styleSheet>"
    );
    let mut parts = with_rows("");
    set(&mut parts, STYLES_PART, body.clone());
    let output = edited_output(parts.clone(), |p| {
        let style = Style {
            fill: Some(solid(9)),
            border: Some(Border::default()),
            ..Style::default()
        };
        add_styled(p, None, &style, 0);
    });
    assert_untouched(&output, &parts, &[STYLES_PART]);
    let got = String::from_utf8(stored_data(&output, STYLES_PART)).expect("UTF-8");
    assert_eq!(
        got,
        format!(
            "{DECLARATION}\n<x:styleSheet xmlns:x=\"{MAIN_NS}\"><x:fonts count=\"0\"/>\
             <x:fills count=\"1\"><x:fill><x:patternFill patternType=\"solid\">\
             <x:fgColor indexed=\"9\"/></x:patternFill></x:fill></x:fills>\
             <x:borders count=\"1\"><x:border><x:left/><x:right/><x:top/><x:bottom/>\
             <x:diagonal/></x:border></x:borders><x:cellXfs count=\"1\">\
             <x:xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\" \
             applyFill=\"1\" applyBorder=\"1\"/></x:cellXfs></x:styleSheet>"
        )
    );
}

#[test]
fn equal_additions_share_one_element_and_one_id_and_unequal_ones_do_not() {
    let (_, mut package) = opened(styled(&with_collections("<cellXfs><xf/></cellXfs>"), ""));
    let style = |font: Font, fill: Fill, border: Border| Style {
        font: Some(font),
        fill: Some(fill),
        border: Some(border),
        ..Style::default()
    };
    add_styled(
        &mut package,
        None,
        &style(named("a"), solid(1), underlined()),
        1,
    );
    add_styled(
        &mut package,
        None,
        &style(named("a"), solid(1), underlined()),
        2,
    );
    let mut bold = named("a");
    bold.bold = true;
    let mut doubled = underlined();
    doubled.diagonal_up = true;
    add_styled(&mut package, None, &style(bold, solid(2), doubled), 3);
    let edits = &package.edits;
    assert_eq!(
        (
            edits.font_additions.len(),
            edits.fill_additions.len(),
            edits.border_additions.len(),
            edits.xf_additions.len()
        ),
        (2, 2, 2, 3)
    );
    let ids = |index: usize| String::from_utf8(edits.xf_additions[index].element(b"")).unwrap();
    for (index, (font, fill, border)) in [(1, 2, 1), (1, 2, 1), (2, 3, 2)].into_iter().enumerate() {
        assert_eq!(
            ids(index),
            format!(
                "<xf numFmtId=\"0\" fontId=\"{font}\" fillId=\"{fill}\" borderId=\"{border}\" \
                 xfId=\"0\" applyFont=\"1\" applyFill=\"1\" applyBorder=\"1\"/>"
            )
        );
    }
}

#[test]
fn a_derived_xf_from_a_base_with_every_id_replaces_them_in_place() {
    // Values replaced, flags set to 1, the base's order and quotes kept,
    // nothing appended; `xfId` and every other attribute as the base wrote
    // them.
    let base = "<xf numFmtId=\"3\" fontId='4' fillId=\"5\" borderId=\"6\" xfId=\"1\" \
                applyBorder=\"0\" applyFill=\"false\" applyFont=\"0\" applyNumberFormat=\"0\" \
                quotePrefix=\"1\"/>";
    let got = added_styles(
        &with_collections(&format!("<cellXfs count=\"1\">{base}</cellXfs>")),
        |p| {
            let style = Style {
                number_format: Some(NumberFormat::BuiltIn(14)),
                font: Some(named("a")),
                fill: Some(solid(1)),
                border: Some(underlined()),
                ..Style::default()
            };
            add_styled(p, Some(0), &style, 1);
        },
    );
    assert!(
        got.contains(
            "<xf numFmtId=\"14\" fontId='1' fillId=\"2\" borderId=\"1\" xfId=\"1\" \
             applyBorder=\"1\" applyFill=\"1\" applyFont=\"1\" applyNumberFormat=\"1\" \
             quotePrefix=\"1\"/></cellXfs>"
        ),
        "{got}"
    );
}

#[test]
fn a_derived_xf_gains_what_its_base_lacks_in_the_stated_order() {
    // The base lacks `numFmtId`, `fontId`, `fillId` and every apply flag but
    // `applyBorder`: `numFmtId` goes first, then the base's attributes, then
    // `fontId fillId`, then `applyNumberFormat applyFont applyFill`. Its
    // content is kept raw, since nothing replaces it.
    let base = "<xf borderId=\"0\" applyBorder=\"1\" applyAlignment=\"1\">\
                <alignment horizontal=\"left\"/><extLst/></xf>";
    let got = added_styles(
        &with_collections(&format!("<cellXfs count=\"1\">{base}</cellXfs>")),
        |p| {
            let style = Style {
                number_format: Some(NumberFormat::BuiltIn(2)),
                font: Some(named("a")),
                fill: Some(solid(1)),
                ..Style::default()
            };
            add_styled(p, Some(0), &style, 1);
        },
    );
    assert!(
        got.contains(&format!(
            "{base}<xf numFmtId=\"2\" borderId=\"0\" applyBorder=\"1\" applyAlignment=\"1\" \
             fontId=\"1\" fillId=\"2\" applyNumberFormat=\"1\" applyFont=\"1\" applyFill=\"1\">\
             <alignment horizontal=\"left\"/><extLst/></xf></cellXfs>"
        )),
        "{got}"
    );
}

#[test]
fn a_derived_xf_with_no_base_names_every_id_and_flags_what_is_given() {
    let (_, mut package) = opened(styled(&with_collections("<cellXfs><xf/></cellXfs>"), ""));
    let every = Style {
        number_format: Some(NumberFormat::BuiltIn(9)),
        font: Some(named("a")),
        fill: Some(solid(1)),
        border: Some(underlined()),
        ..Style::default()
    };
    add_styled(&mut package, None, &every, 1);
    add_styled(&mut package, None, &Style::default(), 2);
    let element =
        |index: usize| String::from_utf8(package.edits.xf_additions[index].element(b"")).unwrap();
    assert_eq!(
        element(0),
        "<xf numFmtId=\"9\" fontId=\"1\" fillId=\"2\" borderId=\"1\" xfId=\"0\" \
         applyNumberFormat=\"1\" applyFont=\"1\" applyFill=\"1\" applyBorder=\"1\"/>"
    );
    assert_eq!(
        element(1),
        "<xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\"/>"
    );
}

#[test]
fn add_cell_style_is_add_style_with_a_number_format_alone() {
    let body = with_collections("<cellXfs><xf fontId=\"0\"><alignment/></xf></cellXfs>");
    let (_, mut one) = opened(styled(&body, ""));
    let (_, mut other) = opened(styled(&body, ""));
    for base in [None, Some(0)] {
        let format = code("0.00%");
        let style = Style {
            number_format: Some(format.clone()),
            ..Style::default()
        };
        let a = one.add_cell_style(base, format).expect("recorded");
        let b = other.add_style(base, &style).expect("recorded");
        assert_eq!(a, b);
    }
    assert_eq!(one.edits.xf_additions, other.edits.xf_additions);
    assert_eq!(one.edits.num_fmt_additions, other.edits.num_fmt_additions);
}

#[test]
fn a_font_fill_or_border_on_a_part_without_its_collection_is_refused_and_records_nothing() {
    let refusals = [
        (
            "<fills/><borders/>",
            Style {
                font: Some(named("a")),
                ..Style::default()
            },
        ),
        (
            "<fonts/><borders/>",
            Style {
                fill: Some(solid(1)),
                ..Style::default()
            },
        ),
        (
            "<fonts/><fills/>",
            Style {
                border: Some(underlined()),
                ..Style::default()
            },
        ),
    ];
    for (collections, style) in refusals {
        let (_, mut package) = opened(styled(
            &format!("{collections}<cellXfs><xf/></cellXfs>"),
            "",
        ));
        let with_code = Style {
            number_format: Some(code("0.0")),
            ..style
        };
        assert_package(
            package.add_style(Some(0), &with_code).expect_err("refused"),
            "a style addition needs a collection the styles part lacks",
        );
        assert_nothing_recorded(&package);
        // What the part has is still taken, at the index nothing took.
        add_styled(&mut package, None, &Style::default(), 1);
    }
}

#[test]
fn a_refused_font_name_or_field_records_nothing() {
    let (_, mut package) = opened(styled(&with_collections("<cellXfs><xf/></cellXfs>"), ""));
    let refusals = [
        (
            Style {
                font: Some(named("")),
                ..Style::default()
            },
            "a font name is empty or holds a character XML cannot carry",
        ),
        (
            Style {
                font: Some(Font {
                    size: Some(0.0),
                    ..Font::default()
                }),
                ..Style::default()
            },
            "a style field is out of range",
        ),
        (
            Style {
                font: Some(named("a")),
                fill: Some(Fill {
                    pattern: Pattern::Solid,
                    foreground: Some(Color::Theme {
                        index: 0,
                        tint: Some(1.5),
                    }),
                    background: None,
                    gradient: None,
                }),
                ..Style::default()
            },
            "a style field is out of range",
        ),
    ];
    for (style, message) in refusals {
        let with_code = Style {
            number_format: Some(code("0.0")),
            ..style
        };
        assert_package(
            package.add_style(None, &with_code).expect_err("refused"),
            message,
        );
        assert_nothing_recorded(&package);
    }
}

#[test]
fn a_rewrite_adding_only_a_number_format_corrects_a_disagreeing_count() {
    // specs/0017-fuller-styles.md, P1 verification ruling 5: every one of the
    // five containers has its count rewritten whenever the part is, grown or
    // not. fonts says 9 of its 1 child and is corrected to 1; fills agrees
    // and borders has no count, and both come out byte for byte.
    let fills = "<fills count=\"2\"><fill/><fill/></fills>";
    let borders = "<borders><border/></borders>";
    let body = format!(
        "<fonts count=\"9\"><font/></fonts>{fills}{borders}<cellXfs count=\"1\"><xf/></cellXfs>"
    );
    let got = added_styles(&body, |p| add(p, None, NumberFormat::BuiltIn(14), 1));
    assert_eq!(
        got,
        style_sheet(&format!(
            "<fonts count=\"1\"><font/></fonts>{fills}{borders}<cellXfs count=\"2\"><xf/>\
             <xf numFmtId=\"14\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\" \
             applyNumberFormat=\"1\"/></cellXfs>"
        ))
    );
}

#[test]
fn a_collection_id_past_u32_is_refused() {
    assert_eq!(collection_id(Some(u32::MAX), None, 0).ok(), Some(u32::MAX));
    assert_eq!(collection_id(Some(7), Some(2), 5).ok(), Some(9));
    for (count, position, recorded) in [
        (Some(u32::MAX), None, 1),
        (Some(1), Some(u32::MAX as usize), 0),
    ] {
        assert_package(
            collection_id(count, position, recorded).expect_err("refused"),
            "a style index is out of range",
        );
    }
    assert_package(
        collection_id(None, None, 0).expect_err("refused"),
        "a style addition needs a collection the styles part lacks",
    );
}

#[test]
fn a_collection_gone_at_the_write_is_refused_with_no_output() {
    // No path of the crate reaches this: the cached read comes from another
    // part, one with the collections, so `add_style` records what the part
    // being rewritten cannot take (`specs/0017-fuller-styles.md`, P1 ruling
    // 1). The refusal is found once the part has streamed.
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let before = replacement_temporaries();
    for (lacking, style) in [
        (
            "<fills/><borders/>",
            Style {
                font: Some(named("a")),
                ..Style::default()
            },
        ),
        (
            "<fonts/><borders/>",
            Style {
                fill: Some(solid(1)),
                ..Style::default()
            },
        ),
        (
            "<fonts/><fills/>",
            Style {
                border: Some(underlined()),
                ..Style::default()
            },
        ),
    ] {
        let (_, mut package) = opened(styled(&format!("{lacking}<cellXfs><xf/></cellXfs>"), ""));
        let (_, mut other) = opened(styled(&with_collections("<cellXfs><xf/></cellXfs>"), ""));
        other.styles().expect("the other part reads");
        package.styles = other.styles.take();
        add_styled(&mut package, None, &style, 1);
        let mut out = Vec::new();
        // Not `refused`: it takes the lock this test already holds.
        let error = match package.write_to(&mut out) {
            Ok(()) => panic!("the write was not refused: it wrote {} byte(s)", out.len()),
            Err(error) => error,
        };
        assert_package(
            error,
            "a style addition needs a collection the styles part lacks",
        );
        assert!(out.is_empty(), "a refused write wrote to the output");
    }
    assert_eq!(replacement_temporaries(), before, "a temporary was left");
}

// ------------------ alignment and protection (specs/0017, P2)

use crate::styles::{Alignment, Horizontal, Protection, Vertical};

/// An alignment centred both ways.
fn centred() -> Alignment {
    Alignment {
        horizontal: Some(Horizontal::Center),
        vertical: Some(Vertical::Center),
        ..Alignment::default()
    }
}

/// A protection that unlocks the cell.
fn unlocked() -> Protection {
    Protection {
        locked: Some(false),
        hidden: None,
    }
}

/// The xf the addition over `base` writes, alone, from a part whose only
/// cell format is `base`.
#[track_caller]
fn derived_from(base: &str, style: &Style) -> String {
    let got = added_styles(
        &with_collections(&format!("<cellXfs count=\"1\">{base}</cellXfs>")),
        |p| add_styled(p, Some(0), style, 1),
    );
    let at = got.find(base).expect("the base is kept") + base.len();
    let end = got.rfind("</cellXfs>").expect("cellXfs closes");
    got[at..end].to_owned()
}

/// A style giving `alignment` and `protection` alone.
fn children(alignment: Option<Alignment>, protection: Option<Protection>) -> Style {
    Style {
        alignment,
        protection,
        ..Style::default()
    }
}

#[test]
fn an_alignment_replaces_the_bases_own() {
    let base = "<xf applyAlignment=\"0\"><alignment wrapText=\"1\"/></xf>";
    assert_eq!(
        derived_from(base, &children(Some(centred()), None)),
        "<xf applyAlignment=\"1\"><alignment horizontal=\"center\" vertical=\"center\"/></xf>"
    );
}

#[test]
fn an_alignment_and_a_protection_replace_both_and_keep_the_bytes_between() {
    // A start-to-end alignment and whitespace between the two children: the
    // spans are replaced whole and the whitespace is the base's.
    let base = "<xf><alignment horizontal=\"left\"></alignment>\n <protection locked=\"1\"/></xf>";
    assert_eq!(
        derived_from(base, &children(Some(centred()), Some(unlocked()))),
        "<xf applyAlignment=\"1\" applyProtection=\"1\">\
         <alignment horizontal=\"center\" vertical=\"center\"/>\n <protection locked=\"0\"/></xf>"
    );
}

#[test]
fn a_new_alignment_goes_before_the_bases_protection() {
    let base = "<xf applyProtection=\"1\"><protection hidden=\"1\"/></xf>";
    assert_eq!(
        derived_from(base, &children(Some(Alignment::default()), None)),
        "<xf applyProtection=\"1\" applyAlignment=\"1\"><alignment/><protection hidden=\"1\"/></xf>"
    );
}

#[test]
fn a_new_protection_goes_after_the_bases_alignment_and_before_other_children() {
    let base = "<xf><alignment indent=\"2\"/><extLst><ext uri=\"x\"/></extLst></xf>";
    assert_eq!(
        derived_from(base, &children(None, Some(unlocked()))),
        "<xf applyProtection=\"1\"><alignment indent=\"2\"/><protection locked=\"0\"/>\
         <extLst><ext uri=\"x\"/></extLst></xf>"
    );
}

#[test]
fn both_new_go_first_and_an_ext_lst_is_kept_raw_after_them() {
    // The extLst holds an alignment and a protection of its own, which are
    // not the xf's children and are neither read nor replaced.
    let base = "<xf><extLst><ext uri=\"x\"><alignment/><protection/></ext></extLst></xf>";
    assert_eq!(
        derived_from(base, &children(Some(centred()), Some(unlocked()))),
        "<xf applyAlignment=\"1\" applyProtection=\"1\">\
         <alignment horizontal=\"center\" vertical=\"center\"/><protection locked=\"0\"/>\
         <extLst><ext uri=\"x\"><alignment/><protection/></ext></extLst></xf>"
    );
}

#[test]
fn a_base_with_no_children_gains_both_with_an_end_tag() {
    let base = "<xf numFmtId=\"0\"/>";
    assert_eq!(
        derived_from(base, &children(Some(centred()), Some(unlocked()))),
        "<xf numFmtId=\"0\" applyAlignment=\"1\" applyProtection=\"1\">\
         <alignment horizontal=\"center\" vertical=\"center\"/><protection locked=\"0\"/></xf>"
    );
}

#[test]
fn a_base_with_children_given_neither_keeps_its_content_byte_for_byte() {
    let base = "<xf><alignment wrapText=\"1\"/><protection/></xf>";
    assert_eq!(
        derived_from(base, &children(None, None)),
        "<xf><alignment wrapText=\"1\"/><protection/></xf>"
    );
}

#[test]
fn with_no_base_the_children_are_written_and_flagged() {
    let (_, mut package) = opened(styled(&with_collections("<cellXfs><xf/></cellXfs>"), ""));
    let style = Style {
        border: Some(underlined()),
        alignment: Some(Alignment {
            horizontal: Some(Horizontal::Right),
            ..Alignment::default()
        }),
        ..Style::default()
    };
    add_styled(&mut package, None, &style, 1);
    add_styled(&mut package, None, &children(None, Some(unlocked())), 2);
    let element =
        |index: usize| String::from_utf8(package.edits.xf_additions[index].element(b"")).unwrap();
    assert_eq!(
        element(0),
        "<xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"1\" xfId=\"0\" \
         applyBorder=\"1\" applyAlignment=\"1\"><alignment horizontal=\"right\"/></xf>"
    );
    assert_eq!(
        element(1),
        "<xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\" \
         applyProtection=\"1\"><protection locked=\"0\"/></xf>"
    );
}

#[test]
fn a_new_child_takes_the_cell_xfs_prefix_and_the_bases_bytes_keep_theirs() {
    let body = format!(
        "{DECLARATION}\n<x:styleSheet xmlns:x=\"{MAIN_NS}\"><x:cellXfs count=\"1\">\
         <x:xf><x:protection/></x:xf></x:cellXfs></x:styleSheet>"
    );
    let mut parts = with_rows("");
    set(&mut parts, STYLES_PART, body);
    let output = edited_output(parts.clone(), |p| {
        add_styled(p, Some(0), &children(Some(Alignment::default()), None), 1);
    });
    let got = String::from_utf8(stored_data(&output, STYLES_PART)).expect("UTF-8");
    assert!(
        got.contains(
            "<x:xf><x:protection/></x:xf><x:xf applyAlignment=\"1\"><x:alignment/>\
             <x:protection/></x:xf></x:cellXfs>"
        ),
        "{got}"
    );
}

#[test]
fn a_rotation_out_of_range_refuses_the_addition_and_records_nothing() {
    let (_, mut package) = opened(styled(&with_collections("<cellXfs><xf/></cellXfs>"), ""));
    let style = Style {
        font: Some(named("a")),
        alignment: Some(Alignment {
            text_rotation: Some(181),
            ..Alignment::default()
        }),
        ..Style::default()
    };
    assert_package(
        package.add_style(Some(0), &style).expect_err("refused"),
        "a style field is out of range",
    );
    assert_nothing_recorded(&package);
}

#[cfg(feature = "charts")]
mod chart;
#[cfg(feature = "charts")]
mod chart_write;
#[cfg(feature = "charts")]
mod chartsheet;
#[cfg(feature = "read")]
mod colors;
#[cfg(feature = "comments")]
mod comments;
#[cfg(feature = "rules")]
mod conditional;
#[cfg(feature = "copy")]
mod copy;
#[cfg(feature = "rules")]
mod dxf;
#[cfg(feature = "read")]
mod extlink;
mod gradient;
#[cfg(feature = "rowcol")]
mod grouping;
#[cfg(feature = "images")]
mod images;
#[cfg(feature = "rowcol")]
mod linestyle;
#[cfg(feature = "metadata")]
mod links;
#[cfg(feature = "read")]
mod namedstyles;
#[cfg(feature = "metadata")]
mod names;
#[cfg(feature = "read")]
mod pivot;
#[cfg(feature = "metadata")]
mod properties;
#[cfg(feature = "metadata")]
mod protection;
#[cfg(feature = "read")]
mod readback;
#[cfg(feature = "richtext")]
mod richtext;
mod rowcol;
#[cfg(feature = "sheetprops")]
mod sheetprops;
#[cfg(feature = "read")]
mod sheetread;
#[cfg(feature = "rowcol")]
mod shift;
#[cfg(feature = "read")]
mod survey;
#[cfg(feature = "tables")]
mod tables;
#[cfg(feature = "rules")]
mod validation;
#[cfg(feature = "rules")]
mod visual;
