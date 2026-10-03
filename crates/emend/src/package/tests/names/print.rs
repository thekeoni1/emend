//! The print area, the print titles and the autofilter on byte-built
//! workbooks and worksheets (`specs/0025-sheet-and-print-settings.md`, "P3:
//! the print area, the print titles, the autofilter", "The calls", and
//! "Planning rulings during P3"): each test records the call and compares
//! the written workbook or worksheet part with the bytes the design gives.
//! The Cargo features `sheetprops` and `metadata`.

use super::*;

/// The table part content type.
const TABLE_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml";

fn area(a1: &str) -> Area {
    Area::parse(a1.as_bytes()).expect("an area")
}

/// The part of `book(n, ..)`'s sheet `i`.
fn sheet_part(i: usize) -> Vec<u8> {
    format!("xl/worksheets/sheet{i}.xml").into_bytes()
}

/// A sheet-scoped `<definedName>` as the write appends one, its text
/// escaped as `specs/0019`'s writer escapes a name's text (an apostrophe as
/// `&apos;`, which reads back as `'`).
fn local(name: &str, sheet: u32, text: &str, hidden: bool) -> String {
    let hidden = if hidden { " hidden=\"1\"" } else { "" };
    let text = text.replace('\'', "&apos;");
    format!("<definedName name=\"{name}\" localSheetId=\"{sheet}\"{hidden}>{text}</definedName>")
}

#[test]
fn a_print_area_of_one_and_of_two_areas_creates_the_container() {
    // `S1` names a cell, so the sheet's name is quoted (ruling 6).
    assert_eq!(
        edited_book(1, "", |p| p
            .set_print_area(&sheet_part(1), &[area("B2:A1")])
            .expect("recorded")),
        book_part(
            1,
            &format!(
                "<definedNames>{}</definedNames>",
                local("_xlnm.Print_Area", 0, "'S1'!$A$1:$B$2", false)
            )
        )
    );
    assert_eq!(
        edited_book(2, "", |p| p
            .set_print_area(&sheet_part(2), &[area("A1:C3"), area("E5")])
            .expect("recorded")),
        book_part(
            2,
            &format!(
                "<definedNames>{}</definedNames>",
                local(
                    "_xlnm.Print_Area",
                    1,
                    "'S2'!$A$1:$C$3,'S2'!$E$5:$E$5",
                    false
                )
            )
        )
    );
}

#[test]
fn a_print_area_replaces_this_sheet_s_and_keeps_another_sheet_s() {
    let file = "<definedNames><definedName name=\"_xlnm.Print_Area\" localSheetId=\"0\">'S1'!$A$1</definedName>\
                <definedName name=\"_xlnm.Print_Area\" localSheetId=\"1\">'S2'!$A$1</definedName>\
                <definedName name=\"Other\">1</definedName></definedNames>";
    assert_eq!(
        edited_book(2, file, |p| p
            .set_print_area(&sheet_part(2), &[area("B2:C3")])
            .expect("recorded")),
        book_part(
            2,
            &format!(
                "<definedNames><definedName name=\"_xlnm.Print_Area\" localSheetId=\"0\">'S1'!$A$1</definedName>\
                 <definedName name=\"Other\">1</definedName>{}</definedNames>",
                local("_xlnm.Print_Area", 1, "'S2'!$B$2:$C$3", false)
            )
        )
    );
    // An empty slice drops it; a container left empty goes.
    let one = "<definedNames><definedName name=\"_xlnm.Print_Area\" localSheetId=\"0\">'S1'!$A$1</definedName></definedNames>";
    assert_eq!(
        edited_book(1, one, |p| p
            .set_print_area(&sheet_part(1), &[])
            .expect("recorded")),
        book_part(1, "")
    );
    // The last call for a part wins.
    assert_eq!(
        edited_book(1, "", |p| {
            p.set_print_area(&sheet_part(1), &[area("A1:B2")])
                .expect("recorded");
            p.set_print_area(&sheet_part(1), &[area("C3:D4")])
                .expect("recorded");
        }),
        book_part(
            1,
            &format!(
                "<definedNames>{}</definedNames>",
                local("_xlnm.Print_Area", 0, "'S1'!$C$3:$D$4", false)
            )
        )
    );
}

#[test]
fn print_titles_of_rows_of_columns_and_of_both_columns_first() {
    let titles = |rows: Option<(u32, u32)>, cols: Option<(u32, u32)>| {
        edited_book(1, "", move |p| {
            p.set_print_titles(&sheet_part(1), rows, cols)
                .expect("recorded")
        })
    };
    for (rows, cols, text) in [
        (Some((1, 3)), None, "'S1'!$1:$3"),
        (None, Some((1, 2)), "'S1'!$A:$B"),
        (Some((2, 2)), Some((27, 28)), "'S1'!$AA:$AB,'S1'!$2:$2"),
    ] {
        assert_eq!(
            titles(rows, cols),
            book_part(
                1,
                &format!(
                    "<definedNames>{}</definedNames>",
                    local("_xlnm.Print_Titles", 0, text, false)
                )
            ),
            "{text}"
        );
    }
    // Both `None` drops the file's.
    let file = "<definedNames><definedName name=\"_xlnm.Print_Titles\" localSheetId=\"0\">'S1'!$1:$1</definedName></definedNames>";
    assert_eq!(
        edited_book(1, file, |p| p
            .set_print_titles(&sheet_part(1), None, None)
            .expect("recorded")),
        book_part(1, "")
    );
}

#[test]
fn the_sheet_s_name_is_bare_when_plain_and_quoted_otherwise() {
    // Ruling 6: bare for a plain identifier that names no cell, else quoted
    // with an apostrophe doubled; the projected name, a rename followed.
    for (name, text) in [
        ("Data", "Data!$A$1:$A$1"),
        ("_d.2", "_d.2!$A$1:$A$1"),
        ("My Sheet", "'My Sheet'!$A$1:$A$1"),
        ("O'Brien", "'O''Brien'!$A$1:$A$1"),
        ("2024", "'2024'!$A$1:$A$1"),
        ("AB12", "'AB12'!$A$1:$A$1"),
        ("R1C1", "'R1C1'!$A$1:$A$1"),
        ("a-b", "'a-b'!$A$1:$A$1"),
    ] {
        let (_, mut package) = opened(book(1, ""));
        package
            .set_print_area(&sheet_part(1), &[area("A1")])
            .expect("recorded");
        package.rename_sheet("S1", name).expect("rename");
        let out = written(package);
        let mut reopened = Package::open(Cursor::new(out)).expect("open the output");
        assert_eq!(
            names_of(&mut reopened),
            vec![self::name("_xlnm.Print_Area", text, Some(0), false)],
            "{name}"
        );
    }
}

#[test]
fn the_three_names_follow_their_sheet_after_a_removal_in_the_same_write() {
    let (_, mut package) = opened(book(3, ""));
    let part = sheet_part(3);
    package
        .set_print_area(&part, &[area("A1:B2")])
        .expect("recorded");
    package
        .set_print_titles(&part, Some((1, 1)), None)
        .expect("recorded");
    package
        .set_auto_filter(&part, Some(area("A1:B5")))
        .expect("recorded");
    package.remove_sheet("S1").expect("remove_sheet");
    let out = written(package);
    let mut reopened = Package::open(Cursor::new(out)).expect("open the output");
    assert_eq!(
        names_of(&mut reopened),
        vec![
            name("_xlnm.Print_Area", "'S3'!$A$1:$B$2", Some(1), false),
            name("_xlnm.Print_Titles", "'S3'!$1:$1", Some(1), false),
            name("_xlnm._FilterDatabase", "'S3'!$A$1:$B$5", Some(1), true),
        ]
    );
}

#[test]
fn an_autofilter_is_created_after_sheet_data_and_before_merge_cells_with_its_hidden_name() {
    let mut parts = book(1, "");
    set(
        &mut parts,
        &sheet_part(1),
        worksheet("<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"D1:E1\"/></mergeCells>"),
    );
    let output = edited_output(parts.clone(), |p| {
        p.set_auto_filter(&sheet_part(1), Some(area("C5:A1")))
            .expect("recorded")
    });
    assert_untouched(&output, &parts, &[WORKBOOK, sheet_part(1).as_slice()]);
    assert_eq!(
        String::from_utf8(stored_data(&output, &sheet_part(1))).expect("UTF-8"),
        worksheet(
            "<sheetData/><autoFilter ref=\"A1:C5\"/><mergeCells count=\"1\"><mergeCell ref=\"D1:E1\"/></mergeCells>"
        )
    );
    assert_eq!(
        String::from_utf8(stored_data(&output, WORKBOOK)).expect("UTF-8"),
        book_part(
            1,
            &format!(
                "<definedNames>{}</definedNames>",
                local("_xlnm._FilterDatabase", 0, "'S1'!$A$1:$C$5", true)
            )
        )
    );
}

#[test]
fn an_autofilter_replaces_the_part_s_its_children_dropped_and_none_drops_both() {
    let file_names = "<definedNames><definedName name=\"_xlnm._FilterDatabase\" localSheetId=\"0\" hidden=\"1\">'S1'!$A$1:$B$9</definedName></definedNames>";
    let body = "<sheetData/><autoFilter ref=\"A1:B9\" x=\"1\"><filterColumn colId=\"0\"><filters><filter val=\"a\"/></filters></filterColumn><sortState ref=\"A2:B9\"/></autoFilter>";
    let mut parts = book(1, file_names);
    set(&mut parts, &sheet_part(1), worksheet(body));
    let output = edited_output(parts.clone(), |p| {
        p.set_auto_filter(&sheet_part(1), Some(area("A1:D4")))
            .expect("recorded")
    });
    assert_eq!(
        String::from_utf8(stored_data(&output, &sheet_part(1))).expect("UTF-8"),
        worksheet("<sheetData/><autoFilter ref=\"A1:D4\" x=\"1\"></autoFilter>")
    );
    assert_eq!(
        String::from_utf8(stored_data(&output, WORKBOOK)).expect("UTF-8"),
        book_part(
            1,
            &format!(
                "<definedNames>{}</definedNames>",
                local("_xlnm._FilterDatabase", 0, "'S1'!$A$1:$D$4", true)
            )
        )
    );
    // `None` drops the element and the name, and the container with them.
    let output = edited_output(parts, |p| {
        p.set_auto_filter(&sheet_part(1), None).expect("recorded")
    });
    assert_eq!(
        String::from_utf8(stored_data(&output, &sheet_part(1))).expect("UTF-8"),
        worksheet("<sheetData/>")
    );
    assert_eq!(
        String::from_utf8(stored_data(&output, WORKBOOK)).expect("UTF-8"),
        book_part(1, "")
    );
}

#[test]
fn an_autofilter_overlapping_a_table_of_its_part_is_refused_at_the_write() {
    let overlap = "a table overlaps a table, a pivot table, a merge or the autofilter";
    let with_table = |table: &str| {
        let mut parts = book(1, "");
        set(
            &mut parts,
            &sheet_part(1),
            format!(
                "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><sheetData/><tableParts count=\"1\"><tablePart r:id=\"rId1\"/></tableParts></worksheet>"
            ),
        );
        with_part(
            &mut parts,
            "xl/worksheets/_rels/sheet1.xml.rels",
            &rels(&relationship(
                "rId1",
                &format!("{ATTR_NS}/table"),
                "../tables/table1.xml",
            )),
            None,
        );
        with_part(&mut parts, "xl/tables/table1.xml", table, Some(TABLE_TYPE));
        parts
    };
    let table = |r: &str| {
        format!(
            "{DECLARATION}\n<table xmlns=\"{MAIN_NS}\" id=\"1\" name=\"T\" displayName=\"T\" ref=\"{r}\"/>"
        )
    };
    let filter = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.set_auto_filter(&sheet_part(1), Some(area("A1:H2")))
            .expect("recorded")
    };
    for bad in [table("A1:H2"), table("H2:J9"), table("x"), table("")] {
        let (_, mut package) = opened(with_table(&bad));
        filter(&mut package);
        assert_package(refused(package), overlap);
    }
    // The control: a table clear of the area is written.
    let (_, mut package) = opened(with_table(&table("A3:H9")));
    filter(&mut package);
    written(package);
}

#[test]
fn the_built_in_names_keep_the_calculation_chain() {
    // Ruling 2: no formula-change rule for the three names.
    let parts = with_chain(2);
    let output = edited_output(parts.clone(), |p| {
        p.set_print_area(&sheet_part(1), &[area("A1:B2")])
            .expect("recorded");
        p.set_print_titles(&sheet_part(1), Some((1, 1)), None)
            .expect("recorded");
    });
    assert_untouched(&output, &parts, &[WORKBOOK]);
    let workbook = String::from_utf8(stored_data(&output, WORKBOOK)).expect("UTF-8");
    assert!(!workbook.contains("fullCalcOnLoad"), "{workbook}");
    // The control: another name edit drops the chain.
    let (_, mut package) = opened(with_chain(2));
    package.add_defined_name("n", "1", None).expect("add");
    let output = written(package);
    assert!(
        !names(&parse_independently(&output)).contains(&b"xl/calcChain.xml".to_vec()),
        "the chain was kept"
    );
}

#[test]
fn the_p3_refusals() {
    let (parts, mut package) = opened(book(1, ""));
    let part = sheet_part(1);
    for (rows, cols) in [
        (Some((0, 0)), None),
        (Some((3, 2)), None),
        (Some((1, 1_048_577)), None),
        (None, Some((0, 1))),
        (None, Some((2, 1))),
        (None, Some((1, 16_385))),
    ] {
        assert_package(
            package
                .set_print_titles(&part, rows, cols)
                .expect_err("refused"),
            "a print area or title is not valid",
        );
    }
    let off = Area {
        first: CellRef { row: 1, col: 1 },
        last: CellRef {
            row: 1_048_577,
            col: 1,
        },
    };
    let index = "a row or column index or count is out of range";
    assert_package(
        package.set_print_area(&part, &[off]).expect_err("refused"),
        index,
    );
    assert_package(
        package
            .set_auto_filter(&part, Some(off))
            .expect_err("refused"),
        index,
    );
    assert_untouched(&written(package), &parts, &[]);
    // A part the archive lacks, and one removed in the same write.
    let (_, mut package) = opened(book(1, ""));
    package
        .set_print_area(b"xl/worksheets/sheet9.xml", &[area("A1")])
        .expect("recorded");
    assert_package(refused(package), "no such part");
    let (_, mut package) = opened(book(2, ""));
    package
        .set_print_titles(&sheet_part(2), Some((1, 1)), None)
        .expect("recorded");
    package.remove_sheet("S2").expect("remove_sheet");
    assert_package(refused(package), "an edit to a removed part");
    // Every other built-in name is still refused through add_defined_name.
    let (_, mut package) = opened(book(1, ""));
    for built_in in [
        "_xlnm.Print_Area",
        "_xlnm.Criteria",
        "_xlnm._FilterDatabase",
    ] {
        assert_package(
            package
                .add_defined_name(built_in, "1", Some("S1"))
                .expect_err("refused"),
            "a defined name is not valid",
        );
    }
}
