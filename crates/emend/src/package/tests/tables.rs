//! Tables on byte-built worksheets (`specs/0021-tables.md`, "P1", and the
//! P1 rulings): each test builds the smallest package with its worksheet
//! holding a given body and, where it says so, a rels part and table parts,
//! records table edits, and compares the written parts with the bytes the
//! design gives, every other part asserted untouched.

use super::*;

/// An area from its `A1` form.
fn area(a1: &str) -> Area {
    Area::parse(a1.as_bytes()).expect("an A1 area")
}

/// The content type of a table part.
const TABLE_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml";

/// A `table` relationship.
fn table_rel(id: &str, target: &str) -> String {
    relationship(id, &format!("{ATTR_NS}/table"), target)
}

/// A worksheet part whose root binds `r` to the relationships namespace.
fn sheet_r(body: &str) -> String {
    format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">{body}</worksheet>"
    )
}

/// The smallest package with the worksheet `sheet`, its rels part holding
/// `rels_body` when one is given, and each named table part with its
/// `Override`.
fn with_tables(sheet: String, rels_body: Option<&str>, tables: &[(&str, String)]) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet);
    if let Some(body) = rels_body {
        with_part(
            &mut parts,
            "xl/worksheets/_rels/sheet1.xml.rels",
            &rels(body),
            None,
        );
    }
    for (name, bytes) in tables {
        with_part(&mut parts, name, bytes, Some(TABLE_TYPE));
    }
    parts
}

/// A table part of the fixtures: the root's attributes and children as
/// given, in the default namespace.
fn table_xml(attributes: &str, children: &str) -> String {
    format!("{DECLARATION}\n<table xmlns=\"{MAIN_NS}\" {attributes}>{children}</table>")
}

/// A worksheet of two tables as a producer writes them: `rId1` to the first
/// by an absolute target, `rId2` to the second by a relative one.
fn two_tables() -> Parts {
    let first = format!(
        "{DECLARATION}\n<x:table xmlns:x=\"{MAIN_NS}\" id=\"1\" name=\"Sales\" \
         displayName=\"Sales\" ref=\"A1:C4\" totalsRowCount=\"1\"><x:autoFilter ref=\"A1:C3\"/>\
         <x:tableColumns count=\"3\"><x:tableColumn id=\"1\" name=\"Item\"/>\
         <x:tableColumn id=\"2\" name=\"Qty\" totalsRowFunction=\"sum\"/>\
         <x:tableColumn id=\"3\" name=\"Total\"><x:calculatedColumnFormula>B2*2\
         </x:calculatedColumnFormula></x:tableColumn></x:tableColumns>\
         <x:tableStyleInfo name=\"Custom Style 1\" showFirstColumn=\"0\" showLastColumn=\"0\" \
         showRowStripes=\"1\" showColumnStripes=\"0\"/></x:table>"
    );
    let second = table_xml(
        "id=\"3\" name=\"Other\" displayName=\"Other\" ref=\"E1:F3\"",
        "<tableColumns count=\"2\"><tableColumn id=\"1\" name=\"P\"/>\
         <tableColumn id=\"2\" name=\"Q\"/></tableColumns>",
    );
    with_tables(
        sheet_r(TWO_BODY),
        Some(&format!(
            "{}{}",
            table_rel("rId1", "/xl/tables/table1.xml"),
            table_rel("rId2", "../tables/table3.xml")
        )),
        &[
            ("xl/tables/table1.xml", first),
            ("xl/tables/table3.xml", second),
        ],
    )
}

/// The worksheet body of [`two_tables`].
const TWO_BODY: &str = "<sheetData/><tableParts count=\"2\"><tablePart r:id=\"rId1\"/>\
                        <tablePart r:id=\"rId2\"/></tableParts>";

/// Two columns `x` and `y` in the style `TableStyleMedium2` with row stripes
/// only.
fn plain<'a>(name: &'a str) -> Table<'a> {
    Table {
        name,
        columns: &["x", "y"],
        style: Some(TableStyleInfo {
            style: TableStyle::Medium(2),
            first_column: false,
            last_column: false,
            row_stripes: true,
            column_stripes: false,
        }),
    }
}

/// The part [`plain`] adds on `reference` with `id`, as literal bytes.
fn plain_part(id: u32, name: &str, reference: &str) -> String {
    format!(
        "{DECLARATION}\n<table xmlns=\"{MAIN_NS}\" id=\"{id}\" name=\"{name}\" \
         displayName=\"{name}\" ref=\"{reference}\" totalsRowShown=\"0\">\
         <autoFilter ref=\"{reference}\"/><tableColumns count=\"2\">\
         <tableColumn id=\"1\" name=\"x\"/><tableColumn id=\"2\" name=\"y\"/></tableColumns>\
         <tableStyleInfo name=\"TableStyleMedium2\" showFirstColumn=\"0\" showLastColumn=\"0\" \
         showRowStripes=\"1\" showColumnStripes=\"0\"/></table>"
    )
}

/// The header row [`plain`] writes on row `row`, columns A and B.
fn header_row(row: u32, prefix: &str) -> String {
    format!(
        "<{prefix}row r=\"{row}\"><{prefix}c r=\"A{row}\" t=\"inlineStr\"><{prefix}is>\
         <{prefix}t xml:space=\"preserve\">x</{prefix}t></{prefix}is></{prefix}c>\
         <{prefix}c r=\"B{row}\" t=\"inlineStr\"><{prefix}is><{prefix}t xml:space=\"preserve\">y\
         </{prefix}t></{prefix}is></{prefix}c></{prefix}row>"
    )
}

/// A stored part of the output, as text.
fn text_of(output: &[u8], name: &[u8]) -> String {
    String::from_utf8(stored_data(output, name)).expect("UTF-8")
}

/// The refusal a write of `edit` on `parts` gives, with no output.
#[track_caller]
fn refused_with(parts: Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let (_, mut package) = opened(parts);
    edit(&mut package);
    refused(package)
}

/// Records `table` on `reference` of the worksheet.
#[track_caller]
fn add(package: &mut Package<Cursor<Vec<u8>>>, reference: &str, table: Table<'_>) {
    package
        .add_table(SHEET, area(reference), table)
        .expect("recorded");
}

#[test]
fn the_tables_of_a_part_with_two_and_of_one_with_none_are_read() {
    let (_, mut package) = opened(two_tables());
    let tables = package.tables(SHEET).expect("read");
    assert_eq!(
        tables,
        [
            TableEntry {
                name: "Sales".to_string(),
                display_name: "Sales".to_string(),
                area: area("A1:C4"),
                columns: vec!["Item".to_string(), "Qty".to_string(), "Total".to_string()],
                style: Some(("Custom Style 1".to_string(), false, false, true, false)),
                header_row: true,
                totals_row: true,
                calculated: true,
                part: b"xl/tables/table1.xml".to_vec(),
            },
            TableEntry {
                name: "Other".to_string(),
                display_name: "Other".to_string(),
                area: area("E1:F3"),
                columns: vec!["P".to_string(), "Q".to_string()],
                style: None,
                header_row: true,
                totals_row: false,
                calculated: false,
                part: b"xl/tables/table3.xml".to_vec(),
            },
        ]
    );
    let (_, mut package) = opened(smallest());
    assert_eq!(package.tables(SHEET).expect("read"), []);
}

#[test]
fn a_read_refuses_a_part_or_relationship_it_cannot_read() {
    let message = "a table part or its relationship cannot be read";
    let root = |attributes: &str| {
        with_tables(
            sheet_r("<sheetData/><tableParts count=\"1\"><tablePart r:id=\"rId1\"/></tableParts>"),
            Some(&table_rel("rId1", "../tables/table1.xml")),
            &[("xl/tables/table1.xml", table_xml(attributes, ""))],
        )
    };
    let cases = [
        // A `tablePart` whose `r:id` names no relationship, and one with
        // no `r:id`.
        with_tables(
            sheet_r("<sheetData/><tableParts count=\"1\"><tablePart r:id=\"rId9\"/></tableParts>"),
            Some(&table_rel("rId1", "../tables/table1.xml")),
            &[],
        ),
        with_tables(
            sheet_r("<sheetData/><tableParts count=\"1\"><tablePart/></tableParts>"),
            None,
            &[],
        ),
        // A relationship whose part is missing.
        with_tables(
            sheet_r("<sheetData/><tableParts count=\"1\"><tablePart r:id=\"rId1\"/></tableParts>"),
            Some(&table_rel("rId1", "../tables/table9.xml")),
            &[],
        ),
        // A root without `ref`, with one that does not parse, and without
        // `id`.
        root("id=\"1\" name=\"T\" displayName=\"T\""),
        root("id=\"1\" name=\"T\" displayName=\"T\" ref=\"A1:\""),
        root("name=\"T\" displayName=\"T\" ref=\"A1:B2\""),
    ];
    for parts in cases {
        let (_, mut package) = opened(parts);
        assert_package(package.tables(SHEET).expect_err("refused"), message);
    }
}

#[test]
fn the_added_parts_bytes_with_and_without_a_style_and_two_additions_take_successive_names() {
    let parts = with_tables(worksheet("<sheetData/>"), None, &[]);
    let output = edited_output(parts.clone(), |p| {
        add(p, "A1:B3", plain("First"));
        add(
            p,
            "D1:E2",
            Table {
                style: None,
                ..plain("Second")
            },
        );
    });
    assert_untouched(&output, &parts, &[SHEET, b"[Content_Types].xml".as_slice()]);
    assert_added_part(
        &output,
        b"xl/tables/table1.xml",
        plain_part(1, "First", "A1:B3").as_bytes(),
    );
    let unstyled = format!(
        "{DECLARATION}\n<table xmlns=\"{MAIN_NS}\" id=\"2\" name=\"Second\" \
         displayName=\"Second\" ref=\"D1:E2\" totalsRowShown=\"0\"><autoFilter ref=\"D1:E2\"/>\
         <tableColumns count=\"2\"><tableColumn id=\"1\" name=\"x\"/>\
         <tableColumn id=\"2\" name=\"y\"/></tableColumns></table>"
    );
    assert_added_part(&output, b"xl/tables/table2.xml", unstyled.as_bytes());
    let row = "<row r=\"1\"><c r=\"A1\" t=\"inlineStr\"><is><t xml:space=\"preserve\">x</t></is></c>\
               <c r=\"B1\" t=\"inlineStr\"><is><t xml:space=\"preserve\">y</t></is></c>\
               <c r=\"D1\" t=\"inlineStr\"><is><t xml:space=\"preserve\">x</t></is></c>\
               <c r=\"E1\" t=\"inlineStr\"><is><t xml:space=\"preserve\">y</t></is></c></row>";
    assert_eq!(
        text_of(&output, SHEET),
        format!(
            "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><sheetData>{row}\
             </sheetData><tableParts count=\"2\"><tablePart r:id=\"rId1\"/>\
             <tablePart r:id=\"rId2\"/></tableParts></worksheet>"
        )
    );
    assert_added_part(
        &output,
        b"xl/worksheets/_rels/sheet1.xml.rels",
        format!(
            "{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\">{}{}</Relationships>",
            table_rel("rId1", "../tables/table1.xml"),
            table_rel("rId2", "../tables/table2.xml")
        )
        .as_bytes(),
    );
    let types = part_text(&parts, b"[Content_Types].xml").replace(
        "</Types>",
        &format!(
            "<Override PartName=\"/xl/tables/table1.xml\" ContentType=\"{TABLE_TYPE}\"/>\
             <Override PartName=\"/xl/tables/table2.xml\" ContentType=\"{TABLE_TYPE}\"/></Types>"
        ),
    );
    assert_eq!(text_of(&output, b"[Content_Types].xml"), types);
}

#[test]
fn an_addition_takes_the_smallest_free_name_and_id_and_is_appended_to_the_container() {
    // `table1` and `table3` are taken, and the ids 1 and 3: the addition is
    // `table2` with id 2, its `tablePart` appended after the kept two, the
    // kept ones byte for byte, `count` set.
    let parts = two_tables();
    let output = edited_output(parts.clone(), |p| add(p, "A10:B12", plain("Added")));
    assert_untouched(
        &output,
        &parts,
        &[
            SHEET,
            b"xl/worksheets/_rels/sheet1.xml.rels".as_slice(),
            b"[Content_Types].xml".as_slice(),
        ],
    );
    assert_added_part(
        &output,
        b"xl/tables/table2.xml",
        plain_part(2, "Added", "A10:B12").as_bytes(),
    );
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(&format!(
            "<sheetData>{}</sheetData><tableParts count=\"3\"><tablePart r:id=\"rId1\"/>\
             <tablePart r:id=\"rId2\"/><tablePart r:id=\"rId3\"/></tableParts>",
            header_row(10, "")
        ))
    );
    assert_eq!(
        text_of(&output, b"xl/worksheets/_rels/sheet1.xml.rels"),
        rels(&format!(
            "{}{}{}",
            table_rel("rId1", "/xl/tables/table1.xml"),
            table_rel("rId2", "../tables/table3.xml"),
            table_rel("rId3", "../tables/table2.xml")
        ))
    );
}

#[test]
fn the_template_part_is_retagged_and_its_container_and_rels_part_created_at_the_end() {
    let mut package = new_package();
    package
        .add_table(SHEET, area("A1:B2"), plain("Table1"))
        .expect("recorded");
    let out = {
        let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = Vec::new();
        package.write_to(&mut out).expect("write_to");
        out
    };
    let mut reopened = Package::open(Cursor::new(out)).expect("open the output");
    let read = |package: &mut Package<Cursor<Vec<u8>>>, name: &[u8]| {
        let index = package.part(name).expect("the part");
        let mut text = String::new();
        package
            .read_part(index)
            .expect("read_part")
            .read_to_string(&mut text)
            .expect("UTF-8");
        text
    };
    let sheet = read(&mut reopened, SHEET);
    println!("template worksheet after the table: {sheet}");
    let template = String::from_utf8(TEMPLATE[4].1.to_vec()).expect("UTF-8");
    let want = template
        .replace(
            &format!("<worksheet xmlns=\"{MAIN_NS}\">"),
            &format!("<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">"),
        )
        .replace(
            "<sheetData/>",
            &format!("<sheetData>{}</sheetData>", header_row(1, "")),
        )
        .replace(
            "</worksheet>",
            "<tableParts count=\"1\"><tablePart r:id=\"rId1\"/></tableParts></worksheet>",
        );
    assert_ne!(want, template, "the template's root was found");
    assert_eq!(sheet, want);
    let rels_text = read(&mut reopened, SHEET_RELS);
    println!("template worksheet's created rels part: {rels_text}");
    assert!(
        rels_text.ends_with(&format!(
            "<Relationships xmlns=\"{RELS_NS}\">{}</Relationships>",
            table_rel("rId1", "../tables/table1.xml")
        )),
        "{rels_text}"
    );
    assert_eq!(
        read(&mut reopened, b"xl/tables/table1.xml"),
        plain_part(1, "Table1", "A1:B2")
    );
    assert_eq!(reopened.tables(SHEET).expect("read back").len(), 1);
}

#[test]
fn a_container_is_created_before_ext_lst_and_on_a_prefixed_root() {
    let parts = with_tables(
        sheet_r("<sheetData/><extLst><ext uri=\"x\"/></extLst>"),
        None,
        &[],
    );
    let output = edited_output(parts, |p| add(p, "A1:B2", plain("T")));
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(&format!(
            "<sheetData>{}</sheetData><tableParts count=\"1\"><tablePart r:id=\"rId1\"/>\
             </tableParts><extLst><ext uri=\"x\"/></extLst>",
            header_row(1, "")
        ))
    );
    let prefixed = format!(
        "{DECLARATION}\n<x:worksheet xmlns:x=\"{MAIN_NS}\" xmlns:rel=\"{ATTR_NS}\">\
         <x:sheetData/></x:worksheet>"
    );
    let parts = with_tables(prefixed, None, &[]);
    let output = edited_output(parts, |p| add(p, "A1:B2", plain("T")));
    assert_eq!(
        text_of(&output, SHEET),
        format!(
            "{DECLARATION}\n<x:worksheet xmlns:x=\"{MAIN_NS}\" xmlns:rel=\"{ATTR_NS}\">\
             <x:sheetData>{}</x:sheetData><x:tableParts count=\"1\">\
             <x:tablePart rel:id=\"rId1\"/></x:tableParts></x:worksheet>",
            header_row(1, "x:")
        )
    );
    // A container that comes after `extLst` is out of schema order: the one
    // created before `extLst` meets it, the two-container message.
    let parts = with_tables(
        sheet_r("<sheetData/><extLst><ext uri=\"x\"/></extLst><tableParts count=\"0\"/>"),
        None,
        &[],
    );
    assert_package(
        refused_with(parts, |p| add(p, "A1:B2", plain("T"))),
        "a worksheet has two tableParts elements",
    );
    // And a second container in its place.
    let parts = with_tables(
        sheet_r("<sheetData/><tableParts count=\"0\"/><tableParts count=\"0\"/>"),
        None,
        &[],
    );
    assert_package(
        refused_with(parts, |p| add(p, "A1:B2", plain("T"))),
        "a worksheet has two tableParts elements",
    );
}

#[test]
fn the_header_cells_are_written_as_text_over_a_value_and_the_row_appended() {
    let body = "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row>\
                <row r=\"3\"><c r=\"C3\"><v>2</v></c></row></sheetData>";
    let parts = with_tables(sheet_r(body), None, &[]);
    let output = edited_output(parts, |p| {
        add(p, "A3:B5", plain("Low"));
        add(p, "A1:B2", plain("High"));
    });
    let row = |r: u32, rest: &str| {
        format!(
            "<row r=\"{r}\"><c r=\"A{r}\" t=\"inlineStr\"><is><t xml:space=\"preserve\">x</t></is></c>\
             <c r=\"B{r}\" t=\"inlineStr\"><is><t xml:space=\"preserve\">y</t></is></c>{rest}</row>"
        )
    };
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(&format!(
            "<sheetData>{}{}</sheetData><tableParts count=\"2\"><tablePart r:id=\"rId1\"/>\
             <tablePart r:id=\"rId2\"/></tableParts>",
            row(1, ""),
            row(3, "<c r=\"C3\"><v>2</v></c>")
        ))
    );
}

#[test]
fn an_overlap_with_a_table_a_pivot_a_merge_the_autofilter_or_an_earlier_addition_is_refused() {
    let message = "a table overlaps a table, a pivot table, a merge or the autofilter";
    // A kept table.
    assert_package(
        refused_with(two_tables(), |p| add(p, "C4:D6", plain("T"))),
        message,
    );
    // A pivot table's location.
    let pivot = with_tables(
        sheet_r("<sheetData/>"),
        Some(&relationship(
            "rId1",
            &format!("{ATTR_NS}/pivotTable"),
            "../pivotTables/pivotTable1.xml",
        )),
        &[],
    );
    let mut pivot = pivot;
    with_part(
        &mut pivot,
        "xl/pivotTables/pivotTable1.xml",
        &format!(
            "{DECLARATION}\n<pivotTableDefinition xmlns=\"{MAIN_NS}\" name=\"P\">\
             <location ref=\"H1:I5\" firstHeaderRow=\"1\"/></pivotTableDefinition>"
        ),
        None,
    );
    assert_package(
        refused_with(pivot, |p| add(p, "I5:J7", plain("T"))),
        message,
    );
    // A merge the part keeps, and one this write adds.
    let merged =
        sheet_r("<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"A20:B20\"/></mergeCells>");
    assert_package(
        refused_with(with_tables(merged.clone(), None, &[]), |p| {
            add(p, "A19:B21", plain("T"))
        }),
        message,
    );
    assert_package(
        refused_with(with_tables(sheet_r("<sheetData/>"), None, &[]), |p| {
            p.merge_cells(SHEET, area("A12:B12")).expect("recorded");
            add(p, "A10:B12", plain("T"));
        }),
        message,
    );
    // A kept merge this write unmerges does not refuse the table.
    let output = edited_output(with_tables(merged, None, &[]), |p| {
        p.unmerge_cells(SHEET, area("A20:B20")).expect("recorded");
        add(p, "A19:B21", plain("T"));
    });
    assert!(text_of(&output, SHEET).contains("<tablePart r:id=\"rId1\"/>"));
    // The sheet's autofilter.
    assert_package(
        refused_with(
            with_tables(
                sheet_r("<sheetData/><autoFilter ref=\"K1:L9\"/>"),
                None,
                &[],
            ),
            |p| add(p, "L5:M7", plain("T")),
        ),
        message,
    );
    // An earlier addition.
    assert_package(
        refused_with(with_tables(sheet_r("<sheetData/>"), None, &[]), |p| {
            add(p, "A1:B3", plain("One"));
            add(p, "B3:C5", plain("Two"));
        }),
        message,
    );
    // A kept table part the check cannot read.
    let unreadable = with_tables(
        sheet_r("<sheetData/><tableParts count=\"1\"><tablePart r:id=\"rId1\"/></tableParts>"),
        Some(&table_rel("rId1", "../tables/table1.xml")),
        &[(
            "xl/tables/table1.xml",
            table_xml("id=\"1\" name=\"T\" displayName=\"T\"", ""),
        )],
    );
    assert_package(
        refused_with(unreadable, |p| add(p, "A10:B12", plain("U"))),
        "a table part or its relationship cannot be read",
    );
}

#[test]
fn a_header_cell_holding_a_formula_is_refused() {
    let body = "<sheetData><row r=\"1\"><c r=\"B1\"><f>1+1</f><v>2</v></c></row></sheetData>";
    assert_package(
        refused_with(with_tables(sheet_r(body), None, &[]), |p| {
            add(p, "A1:B2", plain("T"))
        }),
        "a table header cell holds a formula",
    );
    // Below the header row a formula is the caller's, kept.
    let body = "<sheetData><row r=\"2\"><c r=\"B2\"><f>1+1</f><v>2</v></c></row></sheetData>";
    let output = edited_output(with_tables(sheet_r(body), None, &[]), |p| {
        add(p, "A1:B2", plain("T"))
    });
    assert!(text_of(&output, SHEET).contains("<c r=\"B2\"><f>1+1</f><v>2</v></c>"));
}

#[test]
fn a_name_in_use_is_refused_and_one_of_a_removed_sheets_table_is_taken() {
    let message = "a table name is already in use";
    // A kept table's, ignoring case.
    assert_package(
        refused_with(two_tables(), |p| add(p, "A10:B12", plain("sALES"))),
        message,
    );
    // A defined name's.
    let mut named = sheeted(
        2,
        "",
        "<definedNames><definedName name=\"Rate\">0.5</definedName></definedNames>",
    );
    with_part(
        &mut named,
        "xl/worksheets/_rels/sheet1.xml.rels",
        &rels(""),
        None,
    );
    assert_package(
        refused_with(named, |p| {
            p.add_table(SHEET, area("A1:B2"), plain("RATE"))
                .expect("recorded")
        }),
        message,
    );
    // An earlier addition's.
    assert_package(
        refused_with(with_tables(sheet_r("<sheetData/>"), None, &[]), |p| {
            add(p, "A1:B2", plain("Same"));
            add(p, "D1:E2", plain("same"));
        }),
        message,
    );
    // A table of a sheet this write removes is not counted.
    let mut two = sheeted(2, "", "");
    with_sheet_rels(&mut two, 2, &table_rel("rId1", "../tables/table1.xml"));
    with_part(
        &mut two,
        "xl/tables/table1.xml",
        &table_xml(
            "id=\"1\" name=\"Old\" displayName=\"Old\" ref=\"A1:B2\"",
            "",
        ),
        Some(TABLE_TYPE),
    );
    let output = edited_output(two, |p| {
        p.remove_sheet("S2").expect("remove_sheet");
        p.add_table(SHEET, area("A1:B2"), plain("Old"))
            .expect("recorded");
    });
    let mut reopened = Package::open(Cursor::new(output)).expect("open the output");
    let tables = reopened.tables(SHEET).expect("read back");
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].name, "Old");
    // The removed sheet's id 1 is free; its entry name `table1` is not, as
    // an addition may not reuse a name this write removes (P1 ruling 6).
    assert_eq!(tables[0].part, b"xl/tables/table2.xml");
    let index = reopened.part(b"xl/tables/table2.xml").expect("the part");
    let mut text = String::new();
    reopened
        .read_part(index)
        .expect("read_part")
        .read_to_string(&mut text)
        .expect("UTF-8");
    assert_eq!(text, plain_part(1, "Old", "A1:B2"));
}

#[test]
fn the_call_refuses_each_rule_and_records_nothing() {
    let parts = with_tables(sheet_r("<sheetData/>"), None, &[]);
    let (_, mut package) = opened(parts.clone());
    let long = "t".repeat(256);
    let refusals: [(&str, Table<'_>, &str); 12] = [
        ("A1:B2", plain(""), "a table name is not valid"),
        ("A1:B2", plain("a b"), "a table name is not valid"),
        ("A1:B2", plain("A1"), "a table name is not valid"),
        ("A1:B2", plain("abc1"), "a table name is not valid"),
        ("A1:B2", plain("R"), "a table name is not valid"),
        ("A1:B2", plain(&long), "a table name is not valid"),
        (
            "A1:B2",
            Table {
                columns: &["x", ""],
                ..plain("T")
            },
            "a table column name is not valid",
        ),
        (
            "A1:B2",
            Table {
                columns: &["Col", "cOL"],
                ..plain("T")
            },
            "a table column name is not valid",
        ),
        (
            "A1:B2",
            Table {
                columns: &["x", "y", "z"],
                ..plain("T")
            },
            "a table column name is not valid",
        ),
        (
            "A1:B2",
            Table {
                style: Some(TableStyleInfo {
                    style: TableStyle::Light(22),
                    first_column: false,
                    last_column: false,
                    row_stripes: false,
                    column_stripes: false,
                }),
                ..plain("T")
            },
            "a table style number is out of range",
        ),
        ("A1:B1", plain("T"), "a table area is too small"),
        (
            "A1:B2",
            Table {
                columns: &["x", "y\u{1}"],
                ..plain("T")
            },
            "a text holds a character XML cannot carry",
        ),
    ];
    for (reference, table, message) in refusals {
        assert_package(
            package
                .add_table(SHEET, area(reference), table)
                .expect_err("refused"),
            message,
        );
    }
    assert_package(
        package
            .add_table(
                SHEET,
                Area {
                    first: CellRef { row: 1, col: 1 },
                    last: CellRef {
                        row: 1_048_577,
                        col: 2,
                    },
                },
                plain("T"),
            )
            .expect_err("refused"),
        "a row or column index or count is out of range",
    );
    // Nothing recorded, not even a header cell: the write is the input.
    assert_untouched(&written(package), &parts, &[]);
    // The limits themselves are taken: a name of 255 characters, the first
    // an underscore, and every style family at its highest number; and a
    // name that reads as a reference past the grid, Excel's own `Table1`
    // among them (P1 ruling 5).
    let (_, mut package) = opened(with_tables(sheet_r("<sheetData/>"), None, &[]));
    for (name, reference) in [("Table1", "D1:E2"), ("xyz1", "G1:H2")] {
        package
            .add_table(SHEET, area(reference), plain(name))
            .expect("a reference past the grid is a name");
    }
    let longest = format!("_{}", "t".repeat(254));
    for (at, style) in [
        TableStyle::Light(21),
        TableStyle::Medium(28),
        TableStyle::Dark(11),
    ]
    .into_iter()
    .enumerate()
    {
        let row = at as u32 * 3 + 1;
        package
            .add_table(
                SHEET,
                Area {
                    first: CellRef { row, col: 1 },
                    last: CellRef {
                        row: row + 1,
                        col: 2,
                    },
                },
                Table {
                    name: &format!("{}{at}", &longest[..254]),
                    columns: &["x", "y"],
                    style: Some(TableStyleInfo {
                        style,
                        first_column: true,
                        last_column: true,
                        row_stripes: true,
                        column_stripes: true,
                    }),
                },
            )
            .expect("the limits are taken");
    }
}

#[test]
fn a_strict_worksheet_relationship_gives_the_strict_type_and_declaration() {
    // P1 ruling 3: the worksheet's own relationship in the workbook's rels
    // part is strict, so the table's relationship type and the retag's
    // namespace are the strict forms, as a hyperlink's are.
    const STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
    let mut parts = with_tables(worksheet("<sheetData/>"), None, &[]);
    let workbook_rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        &format!("{ATTR_NS}/worksheet"),
        &format!("{STRICT}/worksheet"),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", workbook_rels);
    let output = edited_output(parts, |p| add(p, "A1:B2", plain("T")));
    assert_eq!(
        text_of(&output, SHEET),
        format!(
            "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{STRICT}\"><sheetData>{}\
             </sheetData><tableParts count=\"1\"><tablePart r:id=\"rId1\"/></tableParts>\
             </worksheet>",
            header_row(1, "")
        )
    );
    assert_added_part(
        &output,
        SHEET_RELS,
        format!(
            "{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\"><Relationship Id=\"rId1\" \
             Type=\"{STRICT}/table\" Target=\"../tables/table1.xml\"/></Relationships>"
        )
        .as_bytes(),
    );
}

#[test]
fn a_sheet_added_in_the_same_write_refuses_a_table() {
    // P1 ruling 2: the relationship's source is not yet a part the package
    // holds; the caller writes twice, as for an external link.
    let (_, mut package) = opened(smallest());
    let part = package.add_sheet("Two").expect("add_sheet");
    package
        .add_table(&part, area("A1:B2"), plain("T"))
        .expect("recorded");
    assert_package(refused(package), "no such part");
}

#[cfg(feature = "rowcol")]
#[test]
fn a_table_edit_with_a_row_insertion_is_refused() {
    assert_package(
        refused_with(with_tables(sheet_r("<sheetData/>"), None, &[]), |p| {
            p.insert_rows(SHEET, 20, 1).expect("recorded");
            add(p, "A1:B2", plain("T"));
        }),
        "a worksheet with a row or column insertion or deletion takes no other edit in the same write",
    );
}

#[test]
fn a_table_edit_on_a_removed_or_replaced_part_is_refused() {
    let (_, mut package) = opened(smallest());
    add(&mut package, "A1:B2", plain("T"));
    package.remove_part(SHEET);
    assert_package(
        refusal(package).expect("refused"),
        "an edit to a removed part",
    );
    let (_, mut package) = opened(smallest());
    add(&mut package, "A1:B2", plain("T"));
    package.replace_part(SHEET, replacement_from_reader(Cursor::new(b"x".to_vec())));
    assert_package(
        refusal(package).expect("refused"),
        "a cell edit to a part this write replaces",
    );
}
