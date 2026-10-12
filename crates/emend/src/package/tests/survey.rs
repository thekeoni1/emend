//! The survey on byte-built packages (`specs/0033-read-breadth-survey.md`,
//! "P1: the survey"): every part kind counted by its content type with the
//! sum rule, an untyped entry and one of a type the table does not name, a
//! strict package, and one sheet's relationships, its drawing's
//! relationships and its sparkline groups. The Cargo feature `read`.

use super::*;
use crate::package::{PartKinds, SheetParts};

/// The `application/vnd.` prefix every typed fixture entry below shares.
const VND: &str = "application/vnd.";

/// The `x14` namespace a sparkline group is written in.
const X14_NS: &str = "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main";

/// Each content type of the table with the entry name the fixture gives it,
/// one entry per type (two macrosheet types, so `macrosheets` counts 2).
const TYPED: [(&str, &str); 28] = [
    (
        "openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
        "xl/chartsheets/sheet1.xml",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
        "xl/dialogsheets/sheet1.xml",
    ),
    ("ms-excel.macrosheet+xml", "xl/macrosheets/sheet1.xml"),
    ("ms-excel.intlmacrosheet+xml", "xl/macrosheets/intl1.xml"),
    (
        "openxmlformats-officedocument.drawingml.chart+xml",
        "xl/charts/chart1.xml",
    ),
    ("ms-office.chartex+xml", "xl/charts/chartEx1.xml"),
    (
        "openxmlformats-officedocument.drawing+xml",
        "xl/drawings/drawing1.xml",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.table+xml",
        "xl/tables/table1.xml",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.pivotTable+xml",
        "xl/pivotTables/pivotTable1.xml",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml",
        "xl/pivotCache/pivotCacheDefinition1.xml",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml",
        "xl/pivotCache/pivotCacheRecords1.xml",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.externalLink+xml",
        "xl/externalLinks/externalLink1.xml",
    ),
    ("ms-office.vbaProject", "xl/vbaProject.bin"),
    ("ms-excel.slicer+xml", "xl/slicers/slicer1.xml"),
    (
        "ms-excel.slicerCache+xml",
        "xl/slicerCaches/slicerCache1.xml",
    ),
    ("ms-excel.timeline+xml", "xl/timelines/timeline1.xml"),
    (
        "ms-excel.timelineCache+xml",
        "xl/timelineCaches/timelineCache1.xml",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.comments+xml",
        "xl/comments1.xml",
    ),
    (
        "ms-excel.threadedcomments+xml",
        "xl/threadedComments/threadedComment1.xml",
    ),
    ("ms-excel.person+xml", "xl/persons/person.xml"),
    (
        "openxmlformats-officedocument.customXmlProperties+xml",
        "customXml/itemProps1.xml",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.connections+xml",
        "xl/connections.xml",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.queryTable+xml",
        "xl/queryTables/queryTable1.xml",
    ),
    (
        "openxmlformats-officedocument.custom-properties+xml",
        "docProps/custom.xml",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.printerSettings",
        "xl/printerSettings/printerSettings1.bin",
    ),
    (
        "openxmlformats-officedocument.spreadsheetml.calcChain+xml",
        "xl/calcChain.xml",
    ),
    // Two more the table does not name, which count as `other_typed`.
    (
        "openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
        "xl/sharedStrings.xml",
    ),
    ("ms-office.chartstyle+xml", "xl/charts/style1.xml"),
];

/// The smallest package's content types with `extra` appended inside the
/// root.
fn types_with(extra: &str) -> String {
    types(&format!(
        "<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
         <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
         <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
         <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>{extra}"
    ))
}

/// The sum of every field, as the sum rule takes it.
fn sum(kinds: &PartKinds) -> u64 {
    [
        kinds.worksheets,
        kinds.chartsheets,
        kinds.dialogsheets,
        kinds.macrosheets,
        kinds.charts,
        kinds.chartex,
        kinds.drawings,
        kinds.images,
        kinds.tables,
        kinds.pivot_tables,
        kinds.pivot_caches,
        kinds.pivot_cache_records,
        kinds.external_links,
        kinds.vba_projects,
        kinds.slicers,
        kinds.slicer_caches,
        kinds.timelines,
        kinds.timeline_caches,
        kinds.comments,
        kinds.threaded_comments,
        kinds.persons,
        kinds.custom_xml,
        kinds.connections,
        kinds.query_tables,
        kinds.custom_properties,
        kinds.printer_settings,
        kinds.calc_chains,
        kinds.other_typed,
        kinds.untyped,
    ]
    .into_iter()
    .map(u64::from)
    .sum()
}

/// Asserts the sum rule: every entry but the content types part, once.
#[track_caller]
fn assert_sum(package: &mut Package<Cursor<Vec<u8>>>, kinds: &PartKinds) {
    let entries = package.archive().entries().len() as u64;
    assert_eq!(
        sum(kinds),
        entries - 1,
        "the fields sum to the entries less one"
    );
}

#[test]
fn every_kind_in_the_table_is_counted_by_its_content_type_and_the_fields_sum_to_the_entries() {
    let mut parts = smallest();
    let mut overrides = String::new();
    for (kind, name) in TYPED {
        overrides.push_str(&format!(
            "<Override PartName=\"/{name}\" ContentType=\"{VND}{kind}\"/>"
        ));
        parts.push((name.as_bytes().to_vec(), b"<x/>".to_vec()));
    }
    // An image by its extension's default, and one more by an override.
    overrides.push_str("<Default Extension=\"png\" ContentType=\"image/png\"/>");
    overrides
        .push_str("<Override PartName=\"/xl/media/image2.svg\" ContentType=\"image/svg+xml\"/>");
    parts.push((b"xl/media/image1.png".to_vec(), b"png".to_vec()));
    parts.push((b"xl/media/image2.svg".to_vec(), b"<svg/>".to_vec()));
    set(&mut parts, b"[Content_Types].xml", types_with(&overrides));
    let mut package = package(parts);
    let kinds = package.part_kinds().expect("the survey");
    let expected = PartKinds {
        worksheets: 1,
        chartsheets: 1,
        dialogsheets: 1,
        macrosheets: 2,
        charts: 1,
        chartex: 1,
        drawings: 1,
        images: 2,
        tables: 1,
        pivot_tables: 1,
        pivot_caches: 1,
        pivot_cache_records: 1,
        external_links: 1,
        vba_projects: 1,
        slicers: 1,
        slicer_caches: 1,
        timelines: 1,
        timeline_caches: 1,
        comments: 1,
        threaded_comments: 1,
        persons: 1,
        custom_xml: 1,
        connections: 1,
        query_tables: 1,
        custom_properties: 1,
        printer_settings: 1,
        calc_chains: 1,
        // The package rels and the workbook rels (the `rels` default), the
        // workbook part, the styles part (the `xml` default), and the two
        // unnamed types above.
        other_typed: 6,
        untyped: 0,
    };
    assert_eq!(kinds, expected);
    assert_sum(&mut package, &kinds);
}

#[test]
fn an_untyped_entry_and_one_of_an_unnamed_type_are_counted_apart() {
    let mut parts = smallest();
    parts.push((b"xl/blob.dat".to_vec(), b"data".to_vec()));
    parts.push((b"xl/noextension".to_vec(), b"data".to_vec()));
    parts.push((b"xl/thing.xml".to_vec(), b"<x/>".to_vec()));
    // A type that only begins like a named one is not that one.
    set(
        &mut parts,
        b"[Content_Types].xml",
        types_with(&format!(
            "<Override PartName=\"/xl/thing.xml\" ContentType=\"{VND}openxmlformats-officedocument.spreadsheetml.worksheet+xmlx\"/>"
        )),
    );
    let mut package = package(parts);
    let kinds = package.part_kinds().expect("the survey");
    assert_eq!(kinds.untyped, 2, "an unknown extension and none at all");
    assert_eq!(kinds.worksheets, 1, "the near-worksheet type is not one");
    assert_eq!(
        kinds.other_typed, 5,
        "rels twice, workbook, styles, near type"
    );
    assert_sum(&mut package, &kinds);
}

#[test]
fn a_strict_package_counts_its_worksheet_with_the_transitional() {
    // Strict packages write the transitional content types (only the
    // namespaces and relationship types move to purl.oclc.org), so the one
    // spelling counts both.
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
             <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
        ),
    );
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&relationship(
            "rId1",
            &format!("{ATTR_STRICT}/worksheet"),
            "worksheets/sheet1.xml",
        )),
    );
    set(
        &mut parts,
        b"xl/worksheets/sheet1.xml",
        format!("{DECLARATION}\n<worksheet xmlns=\"{MAIN_STRICT}\"><sheetData/></worksheet>"),
    );
    let mut package = package(parts);
    let kinds = package.part_kinds().expect("the survey");
    assert_eq!(kinds.worksheets, 1);
    let sheet = package.workbook().expect("the workbook").sheets[0]
        .part
        .clone();
    assert_eq!(sheet, b"xl/worksheets/sheet1.xml");
    assert_eq!(
        package.sheet_parts(&sheet).expect("the sheet"),
        SheetParts::default()
    );
    assert_sum(&mut package, &kinds);
}

/// The sheet rels part of the smallest package's sheet holding `body`.
const SHEET_RELS: &[u8] = b"xl/worksheets/_rels/sheet1.xml.rels";

#[test]
fn a_sheet_counts_each_relationship_kind_and_its_drawings_charts_and_images() {
    let mut parts = smallest();
    let sheet_rels = rels(
        &[
            relationship(
                "rId1",
                &format!("{ATTR_NS}/drawing"),
                "../drawings/drawing1.xml",
            ),
            relationship("rId2", &format!("{ATTR_NS}/comments"), "../comments1.xml"),
            relationship(
                "rId3",
                &format!("{ATTR_NS}/vmlDrawing"),
                "../drawings/vmlDrawing1.vml",
            ),
            relationship("rId4", &format!("{ATTR_NS}/table"), "../tables/table1.xml"),
            // A strict form counts with the transitional.
            relationship(
                "rId5",
                &format!("{ATTR_STRICT}/table"),
                "../tables/table2.xml",
            ),
            relationship(
                "rId6",
                &format!("{ATTR_NS}/pivotTable"),
                "../pivotTables/pivotTable1.xml",
            ),
            relationship(
                "rId7",
                "http://schemas.microsoft.com/office/2007/relationships/slicer",
                "../slicers/slicer1.xml",
            ),
            relationship(
                "rId8",
                "http://schemas.microsoft.com/office/2011/relationships/timeline",
                "../timelines/timeline1.xml",
            ),
            relationship(
                "rId9",
                &format!("{ATTR_NS}/printerSettings"),
                "../printerSettings/printerSettings1.bin",
            ),
            // A kind the survey does not count.
            relationship(
                "rId10",
                &format!("{ATTR_NS}/ctrlProp"),
                "../ctrlProps/ctrlProp1.xml",
            ),
            // A second drawing naming no entry: counted, not followed.
            relationship(
                "rId11",
                &format!("{ATTR_NS}/drawing"),
                "../drawings/drawing9.xml",
            ),
        ]
        .concat(),
    );
    parts.push((SHEET_RELS.to_vec(), sheet_rels.into_bytes()));
    parts.push((b"xl/drawings/drawing1.xml".to_vec(), b"<x/>".to_vec()));
    let drawing_rels = rels(
        &[
            relationship("rId1", &format!("{ATTR_NS}/chart"), "../charts/chart1.xml"),
            relationship("rId2", &format!("{ATTR_NS}/image"), "../media/image1.png"),
            relationship(
                "rId3",
                &format!("{ATTR_STRICT}/image"),
                "../media/image2.png",
            ),
            // A target naming no entry is counted by its relationship.
            relationship("rId4", &format!("{ATTR_NS}/chart"), "../charts/chart9.xml"),
        ]
        .concat(),
    );
    parts.push((
        b"xl/drawings/_rels/drawing1.xml.rels".to_vec(),
        drawing_rels.into_bytes(),
    ));
    let mut package = package(parts);
    let counts = package
        .sheet_parts(b"xl/worksheets/sheet1.xml")
        .expect("the sheet");
    assert_eq!(
        counts,
        SheetParts {
            drawings: 2,
            charts: 2,
            images: 2,
            comments: 1,
            vml_drawings: 1,
            tables: 2,
            pivot_tables: 1,
            slicers: 1,
            timelines: 1,
            printer_settings: 1,
            external_hyperlinks: 0,
            sparkline_groups: 0,
        }
    );
}

#[test]
fn an_external_hyperlink_is_counted_and_an_internal_one_is_not() {
    let mut parts = smallest();
    let sheet_rels = rels(&format!(
        "<Relationship Id=\"rId1\" Type=\"{ATTR_NS}/hyperlink\" Target=\"https://example.invalid/a\" TargetMode=\"External\"/>\
         <Relationship Id=\"rId2\" Type=\"{ATTR_STRICT}/hyperlink\" Target=\"https://example.invalid/b\" TargetMode=\"External\"/>\
         {}",
        relationship("rId3", &format!("{ATTR_NS}/hyperlink"), "../other.xml"),
    ));
    parts.push((SHEET_RELS.to_vec(), sheet_rels.into_bytes()));
    let mut package = package(parts);
    let counts = package
        .sheet_parts(b"xl/worksheets/sheet1.xml")
        .expect("the sheet");
    assert_eq!(counts.external_hyperlinks, 2);
    assert_eq!(
        counts,
        SheetParts {
            external_hyperlinks: 2,
            ..SheetParts::default()
        }
    );
}

#[test]
fn sparkline_groups_are_counted_in_the_x14_namespace_alone() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/worksheets/sheet1.xml",
        format!(
            "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:x14=\"{X14_NS}\"><sheetData/>\
             <extLst><ext uri=\"{{05C60535-1F16-4fd2-B633-F4F36F0B64E0}}\">\
             <x14:sparklineGroups><x14:sparklineGroup><x14:sparklines/></x14:sparklineGroup>\
             <y:sparklineGroup xmlns:y=\"{X14_NS}\"/></x14:sparklineGroups></ext>\
             <ext uri=\"{{00000000-0000-0000-0000-000000000000}}\"><sparklineGroup/></ext>\
             </extLst></worksheet>"
        ),
    );
    let mut package = package(parts);
    let counts = package
        .sheet_parts(b"xl/worksheets/sheet1.xml")
        .expect("the sheet");
    // Two in x14, whatever the prefix; the main-namespace one is not.
    assert_eq!(counts.sparkline_groups, 2);
    // The same package's sheet with no extLst counts none.
    let mut parts = smallest();
    parts.push((
        b"xl/worksheets/sheet2.xml".to_vec(),
        b"<worksheet/>".to_vec(),
    ));
    let mut package = super::package(parts);
    assert_eq!(
        package
            .sheet_parts(b"xl/worksheets/sheet2.xml")
            .expect("the sheet")
            .sparkline_groups,
        0
    );
}

#[test]
fn a_chartsheet_counts_its_drawing_and_no_sparklines() {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/workbook.xml",
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
             <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/>\
             <sheet name=\"Chart1\" sheetId=\"2\" r:id=\"rId3\"/></sheets></workbook>"
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
            relationship("rId2", &format!("{ATTR_NS}/styles"), "styles.xml"),
            relationship(
                "rId3",
                &format!("{ATTR_NS}/chartsheet"),
                "chartsheets/sheet1.xml"
            ),
        )),
    );
    parts.push((
        b"xl/chartsheets/sheet1.xml".to_vec(),
        format!(
            "{DECLARATION}\n<chartsheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
             <sheetViews><sheetView workbookViewId=\"0\"/></sheetViews><drawing r:id=\"rId1\"/></chartsheet>"
        )
        .into_bytes(),
    ));
    parts.push((
        b"xl/chartsheets/_rels/sheet1.xml.rels".to_vec(),
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/drawing"),
            "../drawings/drawing1.xml",
        ))
        .into_bytes(),
    ));
    parts.push((b"xl/drawings/drawing1.xml".to_vec(), b"<x/>".to_vec()));
    parts.push((
        b"xl/drawings/_rels/drawing1.xml.rels".to_vec(),
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/chart"),
            "../charts/chart1.xml",
        ))
        .into_bytes(),
    ));
    let mut package = package(parts);
    let sheet = package.workbook().expect("the workbook").sheets[1].clone();
    assert_eq!(sheet.kind, SheetKind::Chartsheet);
    assert_eq!(
        package.sheet_parts(&sheet.part).expect("the chartsheet"),
        SheetParts {
            drawings: 1,
            charts: 1,
            ..SheetParts::default()
        }
    );
}

#[test]
fn a_sheet_with_no_rels_part_counts_nothing() {
    let mut package = smallest_package();
    assert_eq!(
        package
            .sheet_parts(b"xl/worksheets/sheet1.xml")
            .expect("the sheet"),
        SheetParts::default()
    );
}

#[test]
fn sheet_parts_of_a_name_the_archive_lacks_is_refused() {
    let mut package = smallest_package();
    let error = package
        .sheet_parts(b"xl/worksheets/sheet2.xml")
        .expect_err("no such sheet");
    assert_package(error, "no such part");
    // Control: the sheet that is there answers.
    assert!(package.sheet_parts(b"xl/worksheets/sheet1.xml").is_ok());
}
