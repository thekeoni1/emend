//! Pivot tables on byte-built packages (`specs/0033-read-breadth-survey.md`,
//! "P2: pivot tables"): every modelled attribute and child, each cache
//! source, the field join by position, every axis and subtotal, the values
//! field in each list, a table with no cache and one the workbook relates,
//! unmodelled children walked past, the refusals, and the strict namespace.
//! The Cargo feature `read`.

use super::*;
use crate::cells::{Area, CellRef};
use crate::package::{
    Axis, DataField, FieldRef, PivotCache, PivotField, PivotSource, PivotTable, Subtotal,
};

/// The two content types the fixtures add.
const PIVOT_TABLE_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml";
const PIVOT_CACHE_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml";

/// The message every refusal below carries.
const UNREADABLE: &str = "a pivot table cannot be read";

/// One pivot table of a fixture: its part's body, its cache's body (`None`
/// for no cache relationship), and whether the sheet relates it (else the
/// workbook part does).
struct Table {
    body: String,
    cache: Option<String>,
    from_sheet: bool,
}

/// A table the sheet relates, with a cache.
fn table(body: String, cache: String) -> Table {
    Table {
        body,
        cache: Some(cache),
        from_sheet: true,
    }
}

/// The part name of table `n` (from 1) and of its cache.
fn table_part(n: usize) -> String {
    format!("xl/pivotTables/pivotTable{n}.xml")
}
fn cache_part(n: usize) -> String {
    format!("xl/pivotCache/pivotCacheDefinition{n}.xml")
}

/// A `pivotTableDefinition` in `ns` with the root attributes `root` and the
/// children `body`.
fn definition_in(ns: &str, root: &str, body: &str) -> String {
    format!(
        "{DECLARATION}\n<pivotTableDefinition xmlns=\"{ns}\" {root}>{body}</pivotTableDefinition>"
    )
}
fn definition(root: &str, body: &str) -> String {
    definition_in(MAIN_NS, root, body)
}

/// A `pivotCacheDefinition` in `ns` with the root attributes `root` and the
/// children `body`.
fn cache_in(ns: &str, root: &str, body: &str) -> String {
    format!(
        "{DECLARATION}\n<pivotCacheDefinition xmlns=\"{ns}\" xmlns:r=\"{ATTR_NS}\" {root}>{body}</pivotCacheDefinition>"
    )
}
fn cache(root: &str, body: &str) -> String {
    cache_in(MAIN_NS, root, body)
}

/// A cache with a worksheet source naming `name` and the cache fields
/// `fields`.
fn cache_of(fields: &[&str]) -> String {
    let fields: String = fields
        .iter()
        .map(|name| format!("<cacheField name=\"{name}\" numFmtId=\"0\"/>"))
        .collect();
    cache(
        "",
        &format!(
            "<cacheSource type=\"worksheet\"><worksheetSource name=\"Source\"/></cacheSource>\
             <cacheFields count=\"0\">{fields}</cacheFields>"
        ),
    )
}

/// The smallest table: a location and `fields` pivot fields with no axis.
fn plain(fields: usize, extra: &str) -> String {
    let fields = "<pivotField showAll=\"0\"/>".repeat(fields);
    definition(
        "name=\"T\" cacheId=\"1\"",
        &format!(
            "<location ref=\"A1:B2\" firstHeaderRow=\"1\" firstDataRow=\"1\" firstDataCol=\"0\"/>\
             <pivotFields count=\"0\">{fields}</pivotFields>{extra}"
        ),
    )
}

/// The smallest package with `tables` added, in order, each with its rels
/// part, its cache and the overrides; the sheet's or the workbook's rels
/// relate each.
fn pivot_parts(tables: &[Table]) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut parts = smallest();
    let mut overrides = String::new();
    let mut sheet_rels = String::new();
    let mut workbook_rels = String::new();
    for (at, table) in tables.iter().enumerate() {
        let n = at + 1;
        overrides.push_str(&format!(
            "<Override PartName=\"/{}\" ContentType=\"{PIVOT_TABLE_TYPE}\"/>",
            table_part(n)
        ));
        let rel = relationship(
            &format!("rIdP{n}"),
            &format!("{ATTR_NS}/pivotTable"),
            &format!(
                "{}{}",
                if table.from_sheet { "../" } else { "" },
                &table_part(n)[3..]
            ),
        );
        if table.from_sheet {
            sheet_rels.push_str(&rel);
        } else {
            workbook_rels.push_str(&rel);
        }
        parts.push((table_part(n).into_bytes(), table.body.clone().into_bytes()));
        if let Some(body) = &table.cache {
            overrides.push_str(&format!(
                "<Override PartName=\"/{}\" ContentType=\"{PIVOT_CACHE_TYPE}\"/>",
                cache_part(n)
            ));
            parts.push((
                format!("xl/pivotTables/_rels/pivotTable{n}.xml.rels").into_bytes(),
                rels(&relationship(
                    "rId1",
                    &format!("{ATTR_NS}/pivotCacheDefinition"),
                    &format!("../pivotCache/pivotCacheDefinition{n}.xml"),
                ))
                .into_bytes(),
            ));
            parts.push((cache_part(n).into_bytes(), body.clone().into_bytes()));
        }
    }
    set(
        &mut parts,
        b"[Content_Types].xml",
        types(&format!(
            "<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
             <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
             <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>{overrides}"
        )),
    );
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}{workbook_rels}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            relationship("rId2", &format!("{ATTR_NS}/styles"), "styles.xml"),
        )),
    );
    if !sheet_rels.is_empty() {
        parts.push((
            b"xl/worksheets/_rels/sheet1.xml.rels".to_vec(),
            rels(&sheet_rels).into_bytes(),
        ));
    }
    parts
}

/// The pivot tables of a package built from `tables`.
#[track_caller]
fn read(tables: &[Table]) -> Vec<PivotTable> {
    let mut package = package(pivot_parts(tables));
    package
        .pivot_tables()
        .expect("the pivot tables read")
        .to_vec()
}

/// The refusal of a package built from `tables`.
#[track_caller]
fn refused(tables: &[Table]) -> Error {
    let mut package = package(pivot_parts(tables));
    package
        .pivot_tables()
        .expect_err("the pivot tables are refused")
}

fn area(text: &str) -> Area {
    Area::parse(text.as_bytes()).expect("an area")
}

#[test]
fn a_package_with_no_pivot_table_part_has_none() {
    let mut package = smallest_package();
    assert!(package.pivot_tables().expect("read").is_empty(), "none");
}

#[test]
fn a_pivot_table_with_every_modelled_attribute_and_child_is_read() {
    let body = definition(
        "name=\"Sales &amp; Costs\" cacheId=\"7\" dataCaption=\"Values\" updatedVersion=\"8\"",
        "<location ref=\"B3:E20\" firstHeaderRow=\"1\" firstDataRow=\"2\" firstDataCol=\"1\" rowPageCount=\"1\" colPageCount=\"1\"/>\
         <pivotFields count=\"4\">\
           <pivotField axis=\"axisRow\" name=\"ignored\" showAll=\"0\"><items count=\"1\"><item t=\"default\"/></items></pivotField>\
           <pivotField axis=\"axisCol\"/>\
           <pivotField axis=\"axisPage\"/>\
           <pivotField dataField=\"1\"/>\
         </pivotFields>\
         <rowFields count=\"1\"><field x=\"0\"/></rowFields>\
         <colFields count=\"1\"><field x=\"1\"/></colFields>\
         <pageFields count=\"1\"><pageField fld=\"2\" hier=\"-1\"/></pageFields>\
         <dataFields count=\"1\"><dataField name=\"Sum of Amount\" fld=\"3\" baseField=\"0\" baseItem=\"0\"/></dataFields>",
    );
    let source = cache(
        "r:id=\"rId9\" refreshedBy=\"Nobody\" recordCount=\"42\"",
        "<cacheSource type=\"worksheet\"><worksheetSource ref=\"A1:D43\" sheet=\"Data\"/></cacheSource>\
         <cacheFields count=\"4\">\
           <cacheField name=\"Region\" numFmtId=\"0\"><sharedItems/></cacheField>\
           <cacheField name=\"Year\" numFmtId=\"0\"/>\
           <cacheField name=\"Product\" numFmtId=\"0\"/>\
           <cacheField name=\"Amount\" numFmtId=\"0\"/>\
         </cacheFields>",
    );
    let tables = read(&[table(body, source)]);
    assert_eq!(
        tables,
        vec![PivotTable {
            part: table_part(1).into_bytes(),
            sheet: Some(b"xl/worksheets/sheet1.xml".to_vec()),
            name: "Sales & Costs".to_owned(),
            cache_id: 7,
            location: area("B3:E20"),
            first_header_row: 1,
            first_data_row: 2,
            first_data_col: 1,
            cache: Some(PivotCache {
                part: cache_part(1).into_bytes(),
                source: PivotSource::Worksheet {
                    sheet: Some("Data".to_owned()),
                    reference: Some(area("A1:D43")),
                    name: None,
                },
                fields: vec![
                    "Region".to_owned(),
                    "Year".to_owned(),
                    "Product".to_owned(),
                    "Amount".to_owned(),
                ],
                record_count: Some(42),
            }),
            fields: vec![
                PivotField {
                    name: Some("Region".to_owned()),
                    axis: Some(Axis::Row),
                    data_field: false,
                },
                PivotField {
                    name: Some("Year".to_owned()),
                    axis: Some(Axis::Column),
                    data_field: false,
                },
                PivotField {
                    name: Some("Product".to_owned()),
                    axis: Some(Axis::Page),
                    data_field: false,
                },
                PivotField {
                    name: Some("Amount".to_owned()),
                    axis: None,
                    data_field: true,
                },
            ],
            row_fields: vec![FieldRef::Field(0)],
            column_fields: vec![FieldRef::Field(1)],
            page_fields: vec![FieldRef::Field(2)],
            data_fields: vec![DataField {
                name: Some("Sum of Amount".to_owned()),
                field: 3,
                subtotal: Subtotal::Sum,
            }],
        }]
    );
}

#[test]
fn each_cache_source_is_read_as_written() {
    let fields = "<cacheFields count=\"1\"><cacheField name=\"F\" numFmtId=\"0\"/></cacheFields>";
    let sources = [
        (
            "<cacheSource type=\"worksheet\"><worksheetSource name=\"Table1\"/></cacheSource>",
            PivotSource::Worksheet {
                sheet: None,
                reference: None,
                name: Some("Table1".to_owned()),
            },
        ),
        (
            "<cacheSource type=\"worksheet\"><worksheetSource r:id=\"rId1\"/></cacheSource>",
            PivotSource::Worksheet {
                sheet: None,
                reference: None,
                name: None,
            },
        ),
        (
            "<cacheSource type=\"worksheet\"/>",
            PivotSource::Worksheet {
                sheet: None,
                reference: None,
                name: None,
            },
        ),
        (
            "<cacheSource type=\"external\" connectionId=\"3\"/>",
            PivotSource::External {
                connection_id: Some(3),
            },
        ),
        (
            "<cacheSource type=\"external\"/>",
            PivotSource::External {
                connection_id: None,
            },
        ),
        (
            "<cacheSource type=\"consolidation\"><consolidation><rangeSets count=\"0\"/></consolidation></cacheSource>",
            PivotSource::Consolidation,
        ),
        ("<cacheSource type=\"scenario\"/>", PivotSource::Scenario),
    ];
    for (source, expected) in sources {
        let tables = read(&[table(plain(1, ""), cache("", &format!("{source}{fields}")))]);
        let cache = tables[0].cache.as_ref().expect("a cache");
        assert_eq!(cache.source, expected, "{source}");
        assert_eq!(cache.record_count, None, "no recordCount");
    }
}

#[test]
fn fields_join_the_cache_by_position_and_past_its_fields_have_no_name() {
    let tables = read(&[table(plain(3, ""), cache_of(&["A", "B"]))]);
    let names: Vec<Option<String>> = tables[0]
        .fields
        .iter()
        .map(|field| field.name.clone())
        .collect();
    assert_eq!(
        names,
        vec![Some("A".to_owned()), Some("B".to_owned()), None],
        "the third field is past the cache's two"
    );
}

#[test]
fn every_axis_and_every_subtotal_and_the_default_are_read() {
    let axes = [
        ("axisRow", Axis::Row),
        ("axisCol", Axis::Column),
        ("axisPage", Axis::Page),
        ("axisValues", Axis::Values),
    ];
    let subtotals = [
        ("sum", Subtotal::Sum),
        ("count", Subtotal::Count),
        ("average", Subtotal::Average),
        ("max", Subtotal::Max),
        ("min", Subtotal::Min),
        ("product", Subtotal::Product),
        ("countNums", Subtotal::CountNums),
        ("stdDev", Subtotal::StdDev),
        ("stdDevp", Subtotal::StdDevP),
        ("var", Subtotal::Var),
        ("varp", Subtotal::VarP),
    ];
    let fields: String = axes
        .iter()
        .map(|(axis, _)| format!("<pivotField axis=\"{axis}\"/>"))
        .chain(std::iter::once(
            "<pivotField dataField=\"true\"/>".to_owned(),
        ))
        .collect();
    let data: String = subtotals
        .iter()
        .map(|(subtotal, _)| format!("<dataField fld=\"4\" subtotal=\"{subtotal}\"/>"))
        .chain(std::iter::once("<dataField fld=\"4\"/>".to_owned()))
        .collect();
    let body = definition(
        "name=\"T\" cacheId=\"1\"",
        &format!(
            "<location ref=\"A1\" firstHeaderRow=\"0\" firstDataRow=\"1\" firstDataCol=\"1\"/>\
             <pivotFields count=\"5\">{fields}</pivotFields><dataFields>{data}</dataFields>"
        ),
    );
    let tables = read(&[table(body, cache_of(&["A", "B", "C", "D", "E"]))]);
    let got: Vec<Option<Axis>> = tables[0].fields.iter().map(|field| field.axis).collect();
    let mut want: Vec<Option<Axis>> = axes.iter().map(|(_, axis)| Some(*axis)).collect();
    want.push(None);
    assert_eq!(got, want, "the axes");
    assert!(tables[0].fields[4].data_field, "dataField=\"true\"");
    assert!(!tables[0].fields[0].data_field, "dataField absent");
    let got: Vec<Subtotal> = tables[0]
        .data_fields
        .iter()
        .map(|data| data.subtotal)
        .collect();
    let mut want: Vec<Subtotal> = subtotals.iter().map(|(_, subtotal)| *subtotal).collect();
    want.push(Subtotal::Sum);
    assert_eq!(got, want, "the subtotals, the last by default");
    assert_eq!(tables[0].location, area("A1"), "a one-cell location");
}

#[test]
fn minus_two_is_the_values_field_in_each_axis_list() {
    let body = plain(
        2,
        "<rowFields><field x=\"-2\"/><field x=\"1\"/></rowFields>\
         <colFields><field x=\"-2\"/></colFields>\
         <pageFields><pageField fld=\"-2\"/></pageFields>",
    );
    let tables = read(&[table(body, cache_of(&["A", "B"]))]);
    assert_eq!(
        tables[0].row_fields,
        vec![FieldRef::Values, FieldRef::Field(1)]
    );
    assert_eq!(tables[0].column_fields, vec![FieldRef::Values]);
    assert_eq!(tables[0].page_fields, vec![FieldRef::Values]);
}

#[test]
fn a_table_with_no_cache_relationship_has_no_cache_and_no_field_names() {
    let tables = read(&[Table {
        body: plain(2, ""),
        cache: None,
        from_sheet: true,
    }]);
    assert_eq!(tables[0].cache, None, "no cache");
    assert!(
        tables[0].fields.iter().all(|field| field.name.is_none()),
        "no names without a cache"
    );
}

#[test]
fn a_table_the_workbook_relates_has_no_sheet_beside_one_the_sheet_relates() {
    let tables = read(&[
        Table {
            body: plain(1, ""),
            cache: Some(cache_of(&["A"])),
            from_sheet: false,
        },
        table(plain(1, ""), cache_of(&["B"])),
    ]);
    assert_eq!(tables.len(), 2, "both, by content type");
    assert_eq!(tables[0].part, table_part(1).into_bytes(), "entry order");
    assert_eq!(tables[0].sheet, None, "the workbook relates the first");
    assert_eq!(
        tables[1].sheet,
        Some(b"xl/worksheets/sheet1.xml".to_vec()),
        "the sheet relates the second"
    );
}

#[test]
fn unmodelled_children_with_content_are_walked_past() {
    let body = plain(
        1,
        "<rowItems count=\"2\"><i><x/></i><i t=\"grand\"><x v=\"3\"/></i></rowItems>\
         <colItems><i/></colItems>\
         <formats count=\"1\"><format><pivotArea type=\"all\"><references><reference field=\"0\"/></references></pivotArea></format></formats>\
         <pivotTableStyleInfo name=\"PivotStyleLight16\" showRowHeaders=\"1\"/>\
         <extLst><ext uri=\"{962EF5D1-5CA2-4c93-8EF4-DBF5C05439D2}\" xmlns:x14=\"http://schemas.microsoft.com/office/spreadsheetml/2009/9/main\">\
           <x14:pivotTableDefinition><location ref=\"Z9\"/><pivotFields/></x14:pivotTableDefinition></ext></extLst>\
         <rowFields><field x=\"0\"/></rowFields>",
    );
    let source = cache(
        "",
        "<cacheSource type=\"external\" connectionId=\"1\"/>\
         <cacheFields><cacheField name=\"A\" numFmtId=\"0\"><sharedItems count=\"2\"><s v=\"x\"/><s v=\"y\"/></sharedItems></cacheField></cacheFields>\
         <cacheHierarchies count=\"1\"><cacheHierarchy uniqueName=\"[A]\"><fieldsUsage count=\"1\"><fieldUsage x=\"0\"/></fieldsUsage></cacheHierarchy></cacheHierarchies>\
         <kpis count=\"0\"/><dimensions count=\"1\"><dimension name=\"D\"/></dimensions>\
         <measureGroups count=\"0\"/><maps count=\"0\"/>",
    );
    let tables = read(&[table(body, source)]);
    assert_eq!(
        tables[0].location,
        area("A1:B2"),
        "the x14 location is not read"
    );
    assert_eq!(
        tables[0].fields.len(),
        1,
        "the x14 pivotFields are not read"
    );
    assert_eq!(
        tables[0].row_fields,
        vec![FieldRef::Field(0)],
        "read after them"
    );
    let cache = tables[0].cache.as_ref().expect("a cache");
    assert_eq!(
        cache.fields,
        vec!["A".to_owned()],
        "the shared items walked past"
    );
}

#[test]
fn the_strict_namespace_is_read() {
    let body = definition_in(
        MAIN_STRICT,
        "name=\"S\" cacheId=\"2\"",
        "<location ref=\"C3\" firstHeaderRow=\"1\" firstDataRow=\"1\" firstDataCol=\"0\"/>\
         <pivotFields><pivotField axis=\"axisRow\"/></pivotFields><rowFields><field x=\"0\"/></rowFields>",
    );
    let source = cache_in(
        MAIN_STRICT,
        "recordCount=\"5\"",
        "<cacheSource type=\"worksheet\"><worksheetSource ref=\"A1:A6\" sheet=\"S\"/></cacheSource>\
         <cacheFields><cacheField name=\"K\"/></cacheFields>",
    );
    let tables = read(&[table(body, source)]);
    assert_eq!(tables[0].name, "S");
    assert_eq!(tables[0].fields[0].name.as_deref(), Some("K"));
    assert_eq!(tables[0].fields[0].axis, Some(Axis::Row));
    assert_eq!(
        tables[0].cache.as_ref().map(|cache| cache.record_count),
        Some(Some(5))
    );
}

#[test]
fn a_root_outside_the_main_namespace_or_of_another_name_is_refused() {
    for body in [
        definition_in("urn:other", "name=\"T\" cacheId=\"1\"", ""),
        format!("{DECLARATION}\n<pivotTable xmlns=\"{MAIN_NS}\" name=\"T\" cacheId=\"1\"/>"),
    ] {
        assert_package(refused(&[table(body, cache_of(&["A"]))]), UNREADABLE);
    }
    let wrong_cache = cache("", "").replace("pivotCacheDefinition", "pivotCacheRecords");
    assert_package(refused(&[table(plain(1, ""), wrong_cache)]), UNREADABLE);
}

#[test]
fn the_spec_s_refusals_are_refused_with_the_message() {
    let location =
        "<location ref=\"A1:B2\" firstHeaderRow=\"1\" firstDataRow=\"1\" firstDataCol=\"0\"/>";
    let fields = "<pivotFields><pivotField/><pivotField/></pivotFields>";
    let tables = [
        // No name.
        definition("cacheId=\"1\"", &format!("{location}{fields}")),
        // No cacheId.
        definition("name=\"T\"", &format!("{location}{fields}")),
        // A cacheId that is not a u32.
        definition("name=\"T\" cacheId=\"-1\"", &format!("{location}{fields}")),
        // No location.
        definition("name=\"T\" cacheId=\"1\"", fields),
        // A location lacking ref, and lacking firstDataCol.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!(
                "<location firstHeaderRow=\"1\" firstDataRow=\"1\" firstDataCol=\"0\"/>{fields}"
            ),
        ),
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!("<location ref=\"A1\" firstHeaderRow=\"1\" firstDataRow=\"1\"/>{fields}"),
        ),
        // A ref of A1:.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!(
                "<location ref=\"A1:\" firstHeaderRow=\"1\" firstDataRow=\"1\" firstDataCol=\"0\"/>{fields}"
            ),
        ),
        // Two location.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!("{location}{location}{fields}"),
        ),
        // Two pivotFields.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!("{location}{fields}{fields}"),
        ),
        // An axis of axisX.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!("{location}<pivotFields><pivotField axis=\"axisX\"/></pivotFields>"),
        ),
        // A dataField flag that is not a boolean.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!("{location}<pivotFields><pivotField dataField=\"yes\"/></pivotFields>"),
        ),
        // A subtotal of mean.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!(
                "{location}{fields}<dataFields><dataField fld=\"0\" subtotal=\"mean\"/></dataFields>"
            ),
        ),
        // An x of -3.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!("{location}{fields}<rowFields><field x=\"-3\"/></rowFields>"),
        ),
        // A field x past the fields.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!("{location}{fields}<colFields><field x=\"2\"/></colFields>"),
        ),
        // A pageField fld past the fields.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!("{location}{fields}<pageFields><pageField fld=\"2\"/></pageFields>"),
        ),
        // A dataField fld past the fields, and one with no fld.
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!("{location}{fields}<dataFields><dataField fld=\"2\"/></dataFields>"),
        ),
        definition(
            "name=\"T\" cacheId=\"1\"",
            &format!("{location}{fields}<dataFields><dataField/></dataFields>"),
        ),
    ];
    for body in tables {
        assert_package(
            refused(&[table(body.clone(), cache_of(&["A", "B"]))]),
            UNREADABLE,
        );
    }
    let source = "<cacheSource type=\"worksheet\"/>";
    let names = "<cacheFields><cacheField name=\"A\"/></cacheFields>";
    let caches = [
        // A type of database.
        cache("", &format!("<cacheSource type=\"database\"/>{names}")),
        // A cacheSource with no type.
        cache("", &format!("<cacheSource/>{names}")),
        // No cacheSource.
        cache("", names),
        // A connectionId that is not a u32.
        cache(
            "",
            &format!("<cacheSource type=\"external\" connectionId=\"x\"/>{names}"),
        ),
        // A ref that is not an area.
        cache(
            "",
            &format!(
                "<cacheSource type=\"worksheet\"><worksheetSource ref=\"A1:\"/></cacheSource>{names}"
            ),
        ),
        // A recordCount that is not a u64.
        cache("recordCount=\"many\"", &format!("{source}{names}")),
        // A cacheField with no name.
        cache(
            "",
            &format!("{source}<cacheFields><cacheField numFmtId=\"0\"/></cacheFields>"),
        ),
        // Two cacheSource, and two cacheFields.
        cache("", &format!("{source}{source}{names}")),
        cache("", &format!("{source}{names}{names}")),
    ];
    for body in caches {
        assert_package(refused(&[table(plain(1, ""), body.clone())]), UNREADABLE);
    }
}

#[test]
fn a_cache_relationship_naming_no_entry_is_no_such_part() {
    let mut parts = pivot_parts(&[table(plain(1, ""), cache_of(&["A"]))]);
    drop_part(&mut parts, cache_part(1).as_bytes());
    let mut package = package(parts);
    assert_package(package.pivot_tables().expect_err("refused"), "no such part");
}

#[test]
fn the_list_is_read_once_and_lent() {
    let mut package = package(pivot_parts(&[table(plain(1, ""), cache_of(&["A"]))]));
    let first = package.pivot_tables().expect("read").to_vec();
    let again = package.pivot_tables().expect("read again").to_vec();
    assert_eq!(first, again, "the same list");
    assert_eq!(first[0].location.first, CellRef::parse(b"A1").expect("A1"));
}
